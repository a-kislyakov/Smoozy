//! The real ends of this module's seams.
//!
//! **This is the only file in the crate that knows a protocol exists.**
//! Everything above it takes the machine's answers as values, which is what
//! keeps the kind reading, the family judgement, the hand-over's order, the
//! components edit, the search's honesty and every ending testable at the desk.
//!
//! **`Server protocol and transfers` 1.1's prohibition is kept here by there
//! being no command string in this file at all.** Every path below crosses as a
//! protocol field on the file-transfer subsystem — that document's first
//! carriage — and the commands are [`crate::readings`]' and
//! [`crate::elevated`]'s constants, sent through `Servers`' own command channel.
//!
//! **What is not here, and why.** The command channel's real end is `Servers`'
//! — that crate implements its own `CommandChannel` over the transport, and
//! this one calls that performance rather than writing a second. **The machine's
//! own setup worker is not here either**: the fetch, the unpack, the `ldd`
//! reads, the publication and the read-backs are `Rented-server runtime`'s to
//! host and are named clause by clause at [`crate::arriving_later`].
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` §1.1.
//!
//! **`Server protocol and transfers` T1 and T5 are not listed above**, because
//! this plan defines a T1 and a T5 of its own and a bare one could mean either;
//! where this file cites one, the document is written out at the site.
//! `File Browser` L1.

use smoozy_file_browser::listing::Kind;
use smoozy_server_protocol::subsystem::client::Subsystem;
use smoozy_server_protocol::subsystem::{open_flag, Attributes};
use tokio::io::{AsyncRead, AsyncWrite};

use crate::readings::{AListedThing, TheMachinesFileSurface, WhatTheMachineSaid};

/// **The one reading and writing channel this module's acts ride.**
///
/// `File Browser` L1 already gives a connection **one long-lived reading
/// channel**, and this module's own reads and writes are of the same standing:
/// one holder of one `&mut Subsystem`, so two of this module's file operations
/// cannot be in flight on it at once. **That serialisation is a property of the
/// code rather than a rule anybody keeps.**
///
/// **`Server protocol and transfers` T5 counts every session on a connection
/// against one admission point per server, and this call does not admit
/// itself** — admission is the caller's, exactly as it is for every other
/// session that document names.
pub struct TheMachinesFiles<'a, S> {
    subsystem: &'a mut Subsystem<S>,
}

impl<'a, S> TheMachinesFiles<'a, S> {
    pub fn on(subsystem: &'a mut Subsystem<S>) -> TheMachinesFiles<'a, S> {
        TheMachinesFiles { subsystem }
    }
}

impl<S: AsyncRead + AsyncWrite + Unpin + Send> TheMachinesFileSurface for TheMachinesFiles<'_, S> {
    /// **A whole small file, read in the subsystem's own announced chunk.**
    ///
    /// Every file this module reads whole is small and fixed by a clause —
    /// `/etc/os-release`, `/usr/lib/os-release`, the first process's name, the
    /// Debian sources file, its own landing check. **Nothing here reads a file
    /// whose size it does not already expect to be small**, which is why there
    /// is no cap and no streaming: a caller wanting bytes at scale wants
    /// `Server protocol and transfers` §7's contract and not this.
    async fn read_whole(&mut self, path: &str) -> Result<Vec<u8>, WhatTheMachineSaid> {
        let handle = self
            .subsystem
            .open_to_read(path.as_bytes())
            .await
            .map_err(WhatTheMachineSaid::from)?;
        let chunk = self.subsystem.announcement().limits.read_chunk();
        let mut out = Vec::new();
        loop {
            let piece = match self.subsystem.read_at(&handle, out.len() as u64, chunk).await {
                Ok(piece) => piece,
                Err(failure) => {
                    let _ = self.subsystem.close(&handle).await;
                    return Err(WhatTheMachineSaid::from(failure));
                }
            };
            // **The subsystem's own end of file is an empty piece**, which is
            // that crate's own reading and not a length this one guesses at.
            if piece.is_empty() {
                break;
            }
            out.extend_from_slice(&piece);
        }
        let _ = self.subsystem.close(&handle).await;
        Ok(out)
    }

    /// **A whole file written and settled before it is called written.**
    ///
    /// The writes are sent in the subsystem's own announced chunk and
    /// **settled** before the handle is closed, because a write that was never
    /// acknowledged is a write nobody may report as having landed — which is the
    /// same reason R7's landing check reads its own byte back rather than
    /// trusting an exit.
    async fn write_whole(&mut self, path: &str, bytes: &[u8]) -> Result<(), WhatTheMachineSaid> {
        let handle = self
            .subsystem
            .open(
                path.as_bytes(),
                open_flag::WRITE | open_flag::CREATE | open_flag::TRUNCATE,
                Attributes::permissions(0o600),
            )
            .await
            .map_err(WhatTheMachineSaid::from)?;
        let chunk = self.subsystem.announcement().limits.write_chunk();
        let mut at = 0u64;
        for piece in bytes.chunks(chunk.max(1)) {
            if let Err(failure) = self.subsystem.send_a_write(&handle, at, piece).await {
                let _ = self.subsystem.close(&handle).await;
                return Err(WhatTheMachineSaid::from(failure));
            }
            at += piece.len() as u64;
        }
        if let Err(failure) = self.subsystem.settle_the_writes().await {
            let _ = self.subsystem.close(&handle).await;
            return Err(WhatTheMachineSaid::from(failure));
        }
        if self.subsystem.announcement().announces(smoozy_server_protocol::subsystem::extension::FSYNC) {
            if let Err(failure) = self.subsystem.make_durable(&handle).await {
                let _ = self.subsystem.close(&handle).await;
                return Err(WhatTheMachineSaid::from(failure));
            }
        }
        self.subsystem
            .close(&handle)
            .await
            .map_err(WhatTheMachineSaid::from)
    }

    async fn replace_by_rename(&mut self, from: &str, to: &str) -> Result<(), WhatTheMachineSaid> {
        self.subsystem.posix_rename(from.as_bytes(), to.as_bytes()).await
            .map_err(WhatTheMachineSaid::from)
    }

    async fn read_link(&mut self, path: &str) -> Result<Vec<u8>, WhatTheMachineSaid> {
        self.subsystem.read_link(path.as_bytes()).await.map_err(WhatTheMachineSaid::from)
    }

    /// **The plain make-directory, whose refusal over an existing folder is the
    /// answer rather than a fault.**
    ///
    /// The subsystem has no make-parents operation, so the silent collision path
    /// that form takes does not exist to be taken — which is why R7's five areas
    /// are created directly under the root and **no deep creation is ever
    /// needed**.
    async fn make_one_directory(&mut self, path: &str) -> Result<(), WhatTheMachineSaid> {
        self.subsystem
            .make_directory(path.as_bytes())
            .await
            .map_err(|failure| {
                if matches!(
                    failure,
                    smoozy_server_protocol::subsystem::SubsystemFailure::TheServerRefused(
                        smoozy_server_protocol::subsystem::StatusCode::Failure
                    )
                ) {
                    WhatTheMachineSaid::ItIsAlreadyThere
                } else {
                    WhatTheMachineSaid::from(failure)
                }
            })
    }

    async fn remove_one_file(&mut self, path: &str) -> Result<(), WhatTheMachineSaid> {
        self.subsystem
            .remove(path.as_bytes())
            .await
            .map_err(WhatTheMachineSaid::from)
    }

    async fn remove_one_empty_directory(&mut self, path: &str) -> Result<(), WhatTheMachineSaid> {
        self.subsystem
            .remove_directory(path.as_bytes())
            .await
            .map_err(WhatTheMachineSaid::from)
    }

    /// **The directory reading, with each entry's kind read from the mode the
    /// listing already carried.**
    ///
    /// **No stat is issued per entry**, which is `File Browser`'s own rule for
    /// the same reading and is kept here for the same reason: a listing that
    /// stats every entry is a listing whose cost is the folder's size.
    async fn list_one_directory(
        &mut self,
        path: &str,
    ) -> Result<Vec<AListedThing>, WhatTheMachineSaid> {
        let held = self
            .subsystem
            .list(path.as_bytes())
            .await
            .map_err(WhatTheMachineSaid::from)?;
        Ok(held
            .into_iter()
            .map(|entry| AListedThing {
                it_is_a_directory: Kind::from_mode(entry.attributes.permissions).is_a_folder(),
                name: entry.name,
            })
            .collect())
    }

    /// **The planted-link carriage's own operation, in the argument order
    /// OpenSSH's server takes** — which is `Server protocol and transfers`'
    /// reading of its own `PROTOCOL` file and is that crate's to keep.
    async fn plant_a_link(&mut self, target: &str, at: &str) -> Result<(), WhatTheMachineSaid> {
        // A site carries one token, so whatever an earlier run left at it is
        // met and replaced rather than being an obstruction — the same shape
        // that crate's own carriage uses. Its absence is not a fault.
        let _ = self.subsystem.remove(at.as_bytes()).await;
        self.subsystem
            .plant_a_link(target.as_bytes(), at.as_bytes())
            .await
            .map_err(WhatTheMachineSaid::from)
    }
}

// **The carriage directory is made by [`crate::acts::make_the_carriage_directory`]
// and not here.** It sat in this file, taking the real end rather than the seam,
// and no caller anywhere reached it — so every carriage token of this module was
// written under a directory nothing made. It moved above the seam on 2026-09-04
// so that the desk runs the act rather than a stand-in's version of it.

#[cfg(test)]
mod tests {


    #[test]
    fn the_carriage_directory_is_the_one_that_document_names() {
        assert_eq!(
            smoozy_server_protocol::carriage::THE_DIRECTORY_ON_THE_SUBSYSTEM,
            ".smoozy-carriage"
        );
        // **Every site this module names sits inside it**, which is what keeps
        // the link in the login's own writable area and never on the mount it
        // points at.
        for site in [
            crate::placement::THE_ROOTS_CARRIAGE_SITE,
            crate::placement::THE_PACKAGE_AREAS_CARRIAGE_SITE,
            crate::readings::THE_VERSION_ASKS_CARRIAGE_SITE,
            crate::acts::THE_COMPONENTS_STAGING_SITE,
        ] {
            assert!(site
                .link_path_on_the_subsystem
                .starts_with(smoozy_server_protocol::carriage::THE_DIRECTORY_ON_THE_SUBSYSTEM));
            assert!(site.is_drawn_from_class_twos_alphabet());
        }
    }
}
