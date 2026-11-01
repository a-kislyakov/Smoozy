//! The real end of [`crate::acting::TheMachinesDirectories`], over the
//! file-transfer subsystem.
//!
//! **This is the only file in the crate that knows a protocol exists.**
//! Everything above it takes the machine's answers as values, which is what
//! makes the layout, the names, the budget, the manifest and the sweep's
//! derivation testable at the desk.
//!
//! **Every path crosses as a protocol field** — `Server protocol and transfers`
//! §1.1's first carriage, *"length-delimited data on a route no shell parses"* —
//! so **no path is ever interpolated into a command string** and there is no
//! command string in this file at all.

use smoozy_server_protocol::subsystem::client::Subsystem;
use smoozy_server_protocol::subsystem::{StatusCode, SubsystemFailure};
use smoozy_server_protocol::transfers::moving::THE_MOST_WRITES_IN_FLIGHT;
use tokio::io::{AsyncRead, AsyncWrite};

use crate::acting::{TheMachinesDirectories, WhatTheMachineSaid};

/// How big a bite one read of a whole file takes.
///
/// **The draft's own recommended pair is what a server that announces no limits
/// gets**, and the subsystem already carries it; the manifest and an attempt
/// record are a few kilobytes, so one bite of this size reads either whole and
/// the loop below exists for the case where a server answers short.
const A_BITE: usize = 32_768;

impl<S: AsyncRead + AsyncWrite + Unpin + Send> TheMachinesDirectories for Subsystem<S> {
    async fn make_one_directory(&mut self, path: &str) -> Result<(), WhatTheMachineSaid> {
        // **N9's plain form.** The subsystem has no make-parents operation, so
        // the silent collision path does not exist to be taken.
        self.make_directory(path.as_bytes())
            .await
            .map_err(what_it_said)
    }

    async fn write_a_whole_file_durably(
        &mut self,
        path: &str,
        bytes: &[u8],
    ) -> Result<(), WhatTheMachineSaid> {
        // **The exclusive create**, so two attempts can never land on one
        // object, which is what M8's *written once and never rewritten* wants
        // of the record's own file.
        let handle = self
            .create_exclusively(path.as_bytes())
            .await
            .map_err(what_it_said)?;
        // **`Server protocol and transfers` R6's byte half: the write chunk
        // is the server's own largest write, never a figure of Smoozy's own
        // choosing where the server has stated one.**
        let limit = self.announcement().limits.write_chunk().max(1);
        let mut at = 0u64;
        for chunk in bytes.chunks(limit) {
            self.send_a_write(&handle, at, chunk)
                .await
                .map_err(what_it_said)?;
            at += chunk.len() as u64;
            // **The answers are collected every so many pieces, as the byte
            // loop collects them** (`Server protocol and transfers`' bound on
            // writes in flight). Added 2026-10-19, the dead-line desk
            // coverage: until then every piece was handed over before one
            // answer was read, and above about six megabytes the answers
            // piled up unread on the channel until the connection stopped
            // moving and its keepalive called a healthy line broken.
            if self.writes_in_flight() >= THE_MOST_WRITES_IN_FLIGHT {
                self.settle_the_writes().await.map_err(what_it_said)?;
            }
        }
        self.settle_the_writes().await.map_err(what_it_said)?;
        // **Durable before the call returns, as M8 defines it — three of the
        // four parts `State, identity and recovery` §1.3 rule 6 names.** The
        // bytes are written and **the file itself is flushed** here. **The
        // fourth part is not performed**: the directory that afterwards holds a
        // published name is never flushed, so a rename published from this file
        // is **ordered but not durable**. No route to that flush is invented —
        // this operation takes an open file handle, the subsystem opens no
        // directory to hand one back, and whether a server would answer a flush
        // against such a handle is read nowhere here. M8 carries what it costs.
        //
        // **Where the connection never announced this operation the write is
        // not claimed durable**, and what reaches the caller says so: M8's
        // requirement governs on this route, so the submission stops and the
        // cause is *this connection never offered the operation* rather than a
        // refusal the machine never made.
        self.make_durable(&handle).await.map_err(what_it_said)?;
        self.close(&handle).await.map_err(what_it_said)
    }

    async fn read_a_whole_file(&mut self, path: &str) -> Result<Vec<u8>, WhatTheMachineSaid> {
        let handle = self
            .open_to_read(path.as_bytes())
            .await
            .map_err(what_it_said)?;
        let mut whole = Vec::new();
        loop {
            let bite = self
                .read_at(&handle, whole.len() as u64, A_BITE)
                .await
                .map_err(what_it_said)?;
            if bite.is_empty() {
                break;
            }
            whole.extend_from_slice(&bite);
        }
        self.close(&handle).await.map_err(what_it_said)?;
        Ok(whole)
    }

    async fn rename_without_replacing(
        &mut self,
        from: &str,
        to: &str,
    ) -> Result<(), WhatTheMachineSaid> {
        // **M6: the base operation, which refuses to replace an existing
        // target.** The POSIX-rename extension is not asked for and would be
        // the wrong operation here.
        self.rename(from.as_bytes(), to.as_bytes())
            .await
            .map_err(what_it_said)
    }

    async fn remove_one_file(&mut self, path: &str) -> Result<(), WhatTheMachineSaid> {
        self.remove(path.as_bytes()).await.map_err(what_it_said)
    }

    async fn remove_one_empty_directory(&mut self, path: &str) -> Result<(), WhatTheMachineSaid> {
        self.remove_directory(path.as_bytes())
            .await
            .map_err(what_it_said)
    }

    async fn list_one_directory(&mut self, path: &str) -> Result<Vec<Vec<u8>>, WhatTheMachineSaid> {
        let entries = self.list(path.as_bytes()).await.map_err(what_it_said)?;
        // **N15: names as bytes, and nothing is decoded.** The two entries
        // every listing carries for the directory itself and its parent are not
        // names of anything inside it.
        Ok(entries
            .into_iter()
            .map(|entry| entry.name)
            .filter(|name| name.as_slice() != b"." && name.as_slice() != b"..")
            .collect())
    }
}

/// **`Server protocol and transfers` §1.3's two separations of the machine's
/// own words, and two readings of Smoozy's own beside them.**
///
/// **N10 is why the machine's own refusals collapse into one value**: the
/// generic failure the second creation of a directory answers with *"gives no
/// way to tell *already exists* from any other failure"*, so every refusal
/// beyond the two §1.3 separates becomes [`WhatTheMachineSaid::ItRefused`] and
/// nothing is concluded from it.
///
/// **The other two failures are not the machine refusing anything and are not
/// collapsed with what it did say.** A capability the connection never
/// announced is Smoozy's own check, made before the machine is asked — N6's
/// shape — and an answer that could not be read is Smoozy declining to guess.
/// **N10's reasoning does not reach either**, because it is about what may be
/// concluded from a refusal the machine made, and neither of these is one.
fn what_it_said(failure: SubsystemFailure) -> WhatTheMachineSaid {
    match failure {
        SubsystemFailure::TheChannelIsGone => WhatTheMachineSaid::TheChannelIsGone,
        SubsystemFailure::TheServerRefused(StatusCode::NoSuchFile) => {
            WhatTheMachineSaid::ThePathIsNotThere
        }
        SubsystemFailure::TheServerRefused(StatusCode::PermissionDenied) => {
            WhatTheMachineSaid::TheLoginMayNot
        }
        SubsystemFailure::TheServerRefused(_) => WhatTheMachineSaid::ItRefused,
        SubsystemFailure::ThatCapabilityWasNotAnnounced => {
            WhatTheMachineSaid::ThisConnectionNeverOfferedTheOperation
        }
        // **A revision this code's operations are not written against is an
        // answer Smoozy cannot read**, for the same reason and with the same
        // consequence: nothing is guessed from it, and no operation on the
        // channel is attempted on the strength of it. **It is not the machine
        // refusing anything**, so it does not go to `ItRefused`.
        SubsystemFailure::TheAnswerCouldNotBeRead
        | SubsystemFailure::TheServerOfferedAnotherRevision(_) => {
            WhatTheMachineSaid::TheAnswerCouldNotBeRead
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_capability_that_was_never_announced_is_not_reported_as_a_refusal() {
        // **The round's own fault**: a cause Smoozy established for itself
        // reached the caller as *the machine refused*, so a machine whose
        // file-transfer server never offered the flush took no job and Smoozy
        // could not say why. The three are three values now.
        assert_eq!(
            what_it_said(SubsystemFailure::ThatCapabilityWasNotAnnounced),
            WhatTheMachineSaid::ThisConnectionNeverOfferedTheOperation
        );
        assert_eq!(
            what_it_said(SubsystemFailure::TheAnswerCouldNotBeRead),
            WhatTheMachineSaid::TheAnswerCouldNotBeRead
        );
        assert_eq!(
            what_it_said(SubsystemFailure::TheServerRefused(StatusCode::Failure)),
            WhatTheMachineSaid::ItRefused
        );
    }

    #[test]
    fn the_two_separations_the_protocol_establishes_are_still_two_answers() {
        assert_eq!(
            what_it_said(SubsystemFailure::TheServerRefused(StatusCode::NoSuchFile)),
            WhatTheMachineSaid::ThePathIsNotThere
        );
        assert_eq!(
            what_it_said(SubsystemFailure::TheServerRefused(
                StatusCode::PermissionDenied
            )),
            WhatTheMachineSaid::TheLoginMayNot
        );
        assert_eq!(
            what_it_said(SubsystemFailure::TheChannelIsGone),
            WhatTheMachineSaid::TheChannelIsGone
        );
    }
}
