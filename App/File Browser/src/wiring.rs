//! The real ends of this document's seams.
//!
//! **This is the only file in the crate that knows a protocol or an outside
//! library exists.** Everything above it takes the machine's answers as values,
//! which is what keeps the listing, the permission walk, the already-exists
//! question, the request, the pool and the continuation testable at the desk.
//!
//! **`Server protocol and transfers` §1.1's prohibition is kept by there being
//! no command string here at all.** Every path below crosses as a protocol
//! field on the file-transfer subsystem, which is the first carriage that
//! document names — *"length-delimited data on a route no shell parses"*.
//!
//! **What is not here, and why.** The local reads §12 hands to `Desktop app and
//! delivery`'s platform file adapter — the enumeration walk, the stored-name
//! reading of source names, [`crate::continuation::PathIdentity`] and K9's
//! read-through — have their real end in that crate, because that is the
//! direction this workspace's dependencies run: the desktop crate depends on
//! the feature modules and supplies their adapters, and `Keys` and `Server
//! protocol and transfers` are both already served that way. **Nothing is
//! stubbed for them here**; the seams are the traits and the adapter that fills
//! them is that document's.
//!
//! **P5's reading is not wired here either, and it is not this crate's**: the
//! machine kind and the non-interactive probe are `Server Setup`'s, and that
//! crate implements them over its own kind table. Nothing is written for it
//! here.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Server protocol and transfers` §1.1, §1.2, §1.3, S16.

use std::time::SystemTime;

use smoozy_server_protocol::subsystem::client::Subsystem;
use smoozy_server_protocol::subsystem::{StatusCode, SubsystemFailure};
use tokio::io::{AsyncRead, AsyncWrite};

use crate::already_exists::{ReadsAReceivingFolder, ReceivingFolder};
use crate::blend_version::{Decoded, DecodesZstandard, PrefixUnavailable, ReadsAPrefix};
use crate::listing::{CompleteListing, DirectoryAnswer, Lease, ListedEntry, ReadsADirectory};
use crate::managing::{TheMachinesFiles, WhatTheMachineSaid};
use crate::permission::{CanListAComponent, WhatSmoozyMet};
use crate::picture::{ReadsAFileInPieces, WhyItWasNotRead};
use crate::uploading::{MakesTheLandingFolders, WhatTheMachineSaidAboutAFolder};

/// **`Server protocol and transfers` §1.3's separation, read rather than
/// derived.** The status code is carried as itself by that document and this
/// only names the two it separates; **nothing parses a message.**
fn what_it_said(failure: SubsystemFailure) -> WhatTheMachineSaid {
    match failure {
        SubsystemFailure::TheChannelIsGone => WhatTheMachineSaid::TheChannelIsGone,
        SubsystemFailure::TheServerRefused(StatusCode::NoSuchFile) => {
            WhatTheMachineSaid::ThePathIsNotThere
        }
        SubsystemFailure::TheServerRefused(StatusCode::PermissionDenied) => {
            WhatTheMachineSaid::TheLoginMayNot
        }
        // **The base rename's own refusal over an existing target is status 4,
        // `Failure`** — observed on four servers across three sellers plus the
        // caller's own, and stated by S16 as the base operation sufficing only
        // where the final name does not exist. **It is read as *already there*
        // only where the operation asked was one that refuses for that reason**,
        // which is why this mapping is applied per call below rather than here.
        _ => WhatTheMachineSaid::ItRefused,
    }
}

/// The same failure read for a folder-making call, where the plain form's
/// refusal is the answer N3 chose it for.
fn what_it_said_about_a_folder(failure: SubsystemFailure) -> WhatTheMachineSaidAboutAFolder {
    match failure {
        SubsystemFailure::TheChannelIsGone => WhatTheMachineSaidAboutAFolder::TheChannelIsGone,
        SubsystemFailure::TheServerRefused(StatusCode::PermissionDenied) => {
            WhatTheMachineSaidAboutAFolder::TheLoginMayNot
        }
        // **N3's whole reason for the plain form**: it refuses against an
        // existing folder rather than returning success, which is the silent
        // collision path the make-parents form takes.
        SubsystemFailure::TheServerRefused(StatusCode::Failure) => {
            WhatTheMachineSaidAboutAFolder::ItIsAlreadyThere
        }
        _ => WhatTheMachineSaidAboutAFolder::ItRefused,
    }
}

impl<S: AsyncRead + AsyncWrite + Unpin + Send> MakesTheLandingFolders for Subsystem<S> {
    /// **N3's plain form.** The subsystem has no make-parents operation, so the
    /// silent collision path does not exist to be taken.
    async fn make_one_folder(&mut self, path: &[u8]) -> Result<(), WhatTheMachineSaidAboutAFolder> {
        self.make_directory(path)
            .await
            .map_err(what_it_said_about_a_folder)
    }
}

impl<S: AsyncRead + AsyncWrite + Unpin + Send> TheMachinesFiles for Subsystem<S> {
    async fn remove_one_file(&mut self, path: &[u8]) -> Result<(), WhatTheMachineSaid> {
        self.remove(path).await.map_err(what_it_said)
    }

    /// **M2's bottom-up order is what makes this call's *empty* true**, and the
    /// order is [`crate::managing::the_delete_order`]'s rather than this call's.
    async fn remove_one_empty_folder(&mut self, path: &[u8]) -> Result<(), WhatTheMachineSaid> {
        self.remove_directory(path).await.map_err(what_it_said)
    }

    /// **The base rename**, which S16 observed refusing over an existing target
    /// at status 4, `Failure`. **That refusal is the judgement M4 rests on**, so
    /// it is read as *already there* here and nowhere else.
    async fn base_rename(&mut self, from: &[u8], to: &[u8]) -> Result<(), WhatTheMachineSaid> {
        self.rename(from, to).await.map_err(|failure| {
            if matches!(
                failure,
                SubsystemFailure::TheServerRefused(StatusCode::Failure)
            ) {
                WhatTheMachineSaid::ItIsAlreadyThere
            } else {
                what_it_said(failure)
            }
        })
    }

    /// **The POSIX-rename extension route**, and `Server protocol and
    /// transfers` §1.2's rule is that document's: a capability that is not
    /// announced is never attempted, and its own call refuses before the wire.
    async fn replacing_rename(&mut self, from: &[u8], to: &[u8]) -> Result<(), WhatTheMachineSaid> {
        self.posix_rename(from, to).await.map_err(what_it_said)
    }

    /// **M9 and N3's plain make-directory**, the same call the landing folders
    /// take.
    async fn make_one_directory(&mut self, path: &[u8]) -> Result<(), WhatTheMachineSaid> {
        self.make_directory(path).await.map_err(|failure| {
            if matches!(
                failure,
                SubsystemFailure::TheServerRefused(StatusCode::Failure)
            ) {
                WhatTheMachineSaid::ItIsAlreadyThere
            } else {
                what_it_said(failure)
            }
        })
    }

    /// **The stable file identity, and the honest absence.**
    ///
    /// Revision 3's attribute structure carries no inode, so **there is no route
    /// to it on this surface** and this answers `None` rather than composing one
    /// out of the fields that are there. That is §13 item 9's ground and
    /// `Server protocol and transfers` S15's identity-unavailable branch, and it
    /// is what [`crate::partial::resolve`] and
    /// [`crate::managing::may_the_copy_staging_be_removed`] both meet as *not
    /// removed automatically*.
    async fn stable_identity_of(
        &mut self,
        _path: &[u8],
    ) -> Option<smoozy_server_protocol::transfers::StableFileIdentity> {
        None
    }
}

/// **The two entries every directory reading carries for the directory itself
/// and for its parent, which are not names of anything inside it.**
///
/// **Found 2026-09-07 by driving L1's reading against OpenSSH's own
/// `sftp-server`, which is the first time this crate's real end had met a real
/// server.** A real folder holding four things came back holding six: `.` and
/// `..` ahead of them, each carrying the size and the modification time of the
/// folder itself and of its parent. Every reading here is the folder's
/// contents, so a listing carrying those two is wrong about what the folder
/// holds — the browser would show two folders that are not in it, X1's
/// receiving-folder listing would hold two names the destination does not, and
/// anything composed from a listing's entries would carry them.
///
/// **Nothing above this line could have caught it**, because every reading
/// above [`TheReadingChannel`] takes entries as values, and every value written
/// by hand held only the folder's own names.
///
/// **It is plain correctness rather than a clause**, and it is the same
/// judgement `Job files and outputs`, `Download` and `Server Setup` each make
/// at their own real end — this crate's was the one that did not.
///
/// **The comparison is on the bytes the machine sent** (L1a), never on a
/// decoded form.
fn is_the_folder_itself_or_its_parent(entry: &smoozy_server_protocol::subsystem::Entry) -> bool {
    entry.name == b"." || entry.name == b".."
}

/// **L1's one long-lived reading channel, and the four readings that ride it.**
///
/// *"A folder is listed by the file-transfer subsystem's directory reading on
/// the connection's long-lived reading channel"* (L1), *"the prefix reads ride
/// the reading channel and are serialised behind the listing"* (L7), the
/// already-exists walk's machine side is *"one directory reading per receiving
/// folder"* over the same route (X1), and P3's component walk runs *"on the
/// subsystem route"* too. **All four are one channel here rather than four
/// seams that happen to agree**, and that is what makes the serialisation a
/// property of the code: one of these holds one `&mut` to one subsystem, so two
/// readings cannot be in flight on it at once.
///
/// **The lease is carried rather than read, because the protocol has no such
/// field.** `Server protocol and transfers` §1.1 item 2 names the transport
/// fact the number stands for — reopening the channel loses the server's place
/// — and one [`Subsystem`] **is** one channel, so within one of these the lease
/// cannot change. A reopened channel is a new subsystem and a new number,
/// handed in by whoever reopened it. **Nothing here invents one and there is no
/// default**: the constructor takes it.
///
/// **What is owed above this, and it is not this crate's.** Nobody yet opens,
/// holds or reopens the reading channel for a connected period — that is
/// `Servers`' period and `Server protocol and transfers`' channel admission,
/// whose own `arriving_later` page records the admission point as the caller's.
/// So the lease values a running application would pass through this come from
/// a holder that does not exist, and what is proved at this desk is the reading
/// rather than the holding.
pub struct TheReadingChannel<'a, S> {
    subsystem: &'a mut Subsystem<S>,
    lease: Lease,
}

impl<'a, S> TheReadingChannel<'a, S> {
    /// The channel, on the lease its holder opened it at.
    pub fn on(subsystem: &'a mut Subsystem<S>, lease: Lease) -> TheReadingChannel<'a, S> {
        TheReadingChannel { subsystem, lease }
    }
}

impl<S: AsyncRead + AsyncWrite + Unpin + Send> ReadsADirectory for TheReadingChannel<'_, S> {
    /// **L1's one batched reading, and L3's *output arrived is never read as
    /// listing complete*.**
    ///
    /// The subsystem's own directory reading opens the directory, reads it out
    /// and closes it, and it answers `Ok` **only after the server has reported
    /// the directory's end** — a reading cut by the connection comes back as a
    /// failure instead. So `the_subsystem_reported_the_end` is true on every
    /// answer this route can give and false on none, which is L3's rule holding
    /// by construction rather than a value composed here. **The
    /// partly-read shape [`crate::listing::DirectoryRead`] carries is wider
    /// than this one route needs**, and that is stated rather than glossed: it
    /// is the state machine L3a's discard is written on.
    ///
    /// **No stat per entry is issued** (L1): the four fields come back with the
    /// names in the one reading, and [`ListedEntry::from_a_protocol_entry`] is
    /// the only way into the type (L2).
    ///
    /// **The names cross as the bytes the subsystem returned** (L1a).
    async fn read_a_directory(&mut self, path: &str, on: Lease) -> DirectoryAnswer {
        // **The reading is taken on this channel, whatever lease the caller's
        // pass is on**, and the answer belongs to `the_channels_lease`. A
        // caller on a stale lease discards this at
        // `DirectoryRead::a_piece_arrived`, which is where L3a lives. Refusing
        // it here would have to answer *the machine did not answer*, which is a
        // statement about the connection and would be untrue of a machine that
        // is answering perfectly well.
        let _ = on;
        match self.subsystem.list(path.as_bytes()).await {
            Ok(entries) => DirectoryAnswer::Entries {
                entries: entries
                    .iter()
                    .filter(|entry| !is_the_folder_itself_or_its_parent(entry))
                    .map(ListedEntry::from_a_protocol_entry)
                    .collect(),
                the_subsystem_reported_the_end: true,
            },
            // **`Server protocol and transfers` §1.3's separation is carried as
            // the status itself and told apart at [`crate::permission`]**;
            // nothing is derived from it here.
            Err(failure) => DirectoryAnswer::Refused(failure),
        }
    }

    fn the_channels_lease(&self) -> Lease {
        self.lease
    }
}

impl<S: AsyncRead + AsyncWrite + Unpin + Send> ReadsAReceivingFolder for TheReadingChannel<'_, S> {
    /// **X1's machine side: one directory reading per receiving folder**, the
    /// same operation the listing takes and never a stat per landing place.
    ///
    /// **X2's *not there* is the protocol's own no-such-file status** and is
    /// read off it rather than inferred from an empty answer; every other
    /// refusal stops the question (X2), with which of the three Smoozy met told
    /// apart by `Server protocol and transfers` §1.3's statuses at
    /// [`WhatSmoozyMet::from`].
    ///
    /// **The pass is read elevated nowhere.** P6: the configured subsystem
    /// channel authenticates as the login and no elevation reaches inside it,
    /// so the listing this makes is what the login could see — which is the
    /// fact P8 asks the value to carry, not a default.
    ///
    /// **The moment is the desktop's own clock**, which is what
    /// [`CompleteListing::of`] records it as: it says when Smoozy looked, and it
    /// is never subtracted from a machine-clock reading.
    async fn read(&mut self, path: &[u8]) -> ReceivingFolder {
        match self.subsystem.list(path).await {
            Ok(entries) => ReceivingFolder::Listing(CompleteListing::of(
                entries
                    .iter()
                    .filter(|entry| !is_the_folder_itself_or_its_parent(entry))
                    .map(ListedEntry::from_a_protocol_entry)
                    .collect(),
                SystemTime::now(),
                false,
            )),
            Err(failure) => match WhatSmoozyMet::from(&failure) {
                WhatSmoozyMet::Gone => ReceivingFolder::NotThere,
                met => ReceivingFolder::CouldNotBeRead(met),
            },
        }
    }
}

impl<S: AsyncRead + AsyncWrite + Unpin + Send> CanListAComponent for TheReadingChannel<'_, S> {
    /// **P3's one directory reading per component.** The walk asks only whether
    /// the component listed, so the entries are dropped here; what it needs is
    /// the status, which crosses whole and is told apart by the walk itself.
    async fn list(&mut self, component_path: &str) -> Result<(), SubsystemFailure> {
        self.subsystem
            .list(component_path.as_bytes())
            .await
            .map(|_| ())
    }
}

impl<S: AsyncRead + AsyncWrite + Unpin + Send> ReadsAPrefix for TheReadingChannel<'_, S> {
    /// **L7's bounded prefix, over the subsystem's byte-offset reads against an
    /// open handle**, on this same channel — which is the whole reason L7 names
    /// the channel: *"a reading that opened a channel of its own would offer
    /// sixty channel admissions for sixty `.blend` files in one folder"*.
    ///
    /// **Nothing opens the file for Blender and no machine-side program is
    /// asked**: this is an open, some reads and a close.
    ///
    /// **The read is taken in pieces of the negotiated size and never one of
    /// Smoozy's own choosing** — `Server protocol and transfers` §1.2 item 2's
    /// largest read, taken from the extension that reports it where the server
    /// announces one and otherwise from the specification's own recommended
    /// figure, and read off the announcement rather than assumed.
    ///
    /// **Fewer bytes than `want` is a short file and not a failure** (the
    /// trait's own words): the end of the file arrives as an empty answer from
    /// that document's own read, and the loop stops on it. **A refusal is
    /// [`PrefixUnavailable`] and the tag goes blank**, which is the same ending
    /// L8 gives an unrecognised shape — nothing is guessed from it.
    ///
    /// **L9 grows the prefix by asking again from the front**, so each call
    /// opens, reads and closes. That costs the earlier bytes twice and no
    /// promise rests on it: L9 says outright that the tag is a cost rather than
    /// a correctness question, and on the pinned decoder the opening text
    /// arrives inside the first fetch.
    async fn read_the_front_of(
        &mut self,
        path: &str,
        want: usize,
    ) -> Result<Vec<u8>, PrefixUnavailable> {
        let handle = self
            .subsystem
            .open_to_read(path.as_bytes())
            .await
            .map_err(|_| PrefixUnavailable)?;
        let piece = self.subsystem.announcement().limits.read_chunk().max(1);
        let mut got: Vec<u8> = Vec::new();
        while got.len() < want {
            let at = u64::try_from(got.len()).unwrap_or(u64::MAX);
            let asking = piece.min(want - got.len());
            match self.subsystem.read_at(&handle, at, asking).await {
                // The end of the file, which that document's own read returns
                // as an empty answer rather than as a failure.
                Ok(bytes) if bytes.is_empty() => break,
                Ok(bytes) => got.extend_from_slice(&bytes),
                Err(_) => {
                    let _ = self.subsystem.close(&handle).await;
                    return Err(PrefixUnavailable);
                }
            }
        }
        let _ = self.subsystem.close(&handle).await;
        Ok(got)
    }
}

/// **L29's reading, over a transfer channel's own subsystem** — the
/// byte-offset reads against an open handle, **on the channel the pool lent
/// for this one picture and never on L1's reading channel**, which a whole file
/// would hold behind every listing and every L7 prefix for as long as it took.
///
/// **The status is told apart the way [`what_it_said`] tells it**: the
/// protocol's own permission-denied and no-such-file statuses, the channel
/// gone, and every other refusal as itself. Nothing parses a message.
fn why_the_picture_was_not_read(failure: SubsystemFailure) -> WhyItWasNotRead {
    match failure {
        SubsystemFailure::TheChannelIsGone => WhyItWasNotRead::TheLineWent,
        SubsystemFailure::TheServerRefused(StatusCode::NoSuchFile) => WhyItWasNotRead::ItIsNotThere,
        SubsystemFailure::TheServerRefused(StatusCode::PermissionDenied) => WhyItWasNotRead::TheLoginMayNot,
        _ => WhyItWasNotRead::ItRefused,
    }
}

impl<S: AsyncRead + AsyncWrite + Unpin + Send> ReadsAFileInPieces for Subsystem<S> {
    type Handle = smoozy_server_protocol::subsystem::Handle;

    async fn open_the_file(&mut self, path: &[u8]) -> Result<Self::Handle, WhyItWasNotRead> {
        self.open_to_read(path)
            .await
            .map_err(why_the_picture_was_not_read)
    }

    async fn send_a_piece(&mut self, handle: &Self::Handle, at: u64, want: usize) -> Result<u32, WhyItWasNotRead> {
        self.send_a_read(handle, at, want)
            .await
            .map_err(why_the_picture_was_not_read)
    }

    async fn the_piece(&mut self, ticket: u32) -> Result<Vec<u8>, WhyItWasNotRead> {
        self.the_read_answered(ticket)
            .await
            .map_err(why_the_picture_was_not_read)
    }

    async fn close_the_file(&mut self, handle: &Self::Handle) {
        let _ = self.close(handle).await;
    }

    /// **The negotiated read size and never one of Smoozy's own choosing** —
    /// `Server protocol and transfers` §1.2 item 2's largest read.
    fn the_piece_size(&self) -> usize {
        self.announcement().limits.read_chunk().max(1)
    }
}

/// **L8's streaming Zstandard decode, over the decoder this crate pins.**
///
/// **The pin is `zstd` at an exact version, and it is the answer §13 item 10
/// said the reading would settle.** **That item is CLOSED**, on this reading,
/// written into the plan on 2026-09-04 with its citations at that document's
/// companion `File Browser.sources.md` J4, which is an entry of that file and
/// not a clause of any plan. *(It said the item had no permanence line and was a
/// question a publication answers; the question was answered.)*
///
/// **What the format's own specification carries** (RFC 8878, *Zstandard
/// Compression and the 'application/zstd' Media Type*, read 2026-09-04 at
/// `https://www.rfc-editor.org/rfc/rfc8878.html`):
///
/// - §2: *"each block can be decompressed without waiting for its successor"* —
///   so the format does not require the whole frame before the first bytes.
/// - §3.1.1.1: the frame header is *"a variable size, with a minimum of 2 bytes
///   up to a maximum of 14 bytes depending on optional parameters"*, and the
///   magic number is *"4 bytes, little-endian format. Value: 0xFD2FB528."*
/// - §3.1.1.2.4: *"The size of Block_Content is limited by Block_Maximum_Size,
///   which is the smallest of: Window_Size [or] 128 KB"*.
///
/// **What the library's own publication carries** (`ZSTD_decompressStream`,
/// Zstandard's own manual, read 2026-09-04 at
/// `https://facebook.github.io/zstd/zstd_manual.html`): it *"tries to flush all
/// data decoded immediately, respecting output buffer size"*, and
/// `ZSTD_DStreamOutSize()` is the *"recommended size for output buffer.
/// Guarantee to successfully flush at least one complete block in all
/// circumstances."*
///
/// **And it was run, because the reading settles the format and not an
/// implementation.** Two decoders were driven against frames whose first bytes
/// were the observed opening text, at three compression levels, on 2026-09-04:
/// the C library reached the head from **16 to 43 bytes** of prefix in every
/// case, while a pure-Rust decoder needed **up to the whole file** because it
/// hands out only what has left the declared window. **That difference is why
/// the pin is this one**, and it is what makes L9's cap a cost rather than a
/// correctness question.
#[derive(Debug, Clone, Copy, Default)]
pub struct TheZstandardDecoder;

/// The magic number a Zstandard frame opens with — RFC 8878 §3.1.1.1, quoted
/// above. **Four bytes, little-endian.**
const THE_FRAME_MAGIC_NUMBER: [u8; 4] = 0xFD2F_B528u32.to_le_bytes();

impl DecodesZstandard for TheZstandardDecoder {
    fn first_decompressed_bytes(&mut self, compressed_prefix: &[u8], want: usize) -> Decoded {
        use std::io::Read;

        // Fewer than the magic number's own four bytes says nothing at all
        // about the format, so it is more input wanted rather than a verdict.
        if compressed_prefix.len() < THE_FRAME_MAGIC_NUMBER.len() {
            return Decoded::MoreInputWanted;
        }
        if compressed_prefix[..4] != THE_FRAME_MAGIC_NUMBER {
            // **Not this format, and nothing is inferred from that beyond a
            // blank tag** — L8's own discipline.
            return Decoded::NotThisFormat;
        }

        let Ok(mut decoder) = zstd::stream::read::Decoder::new(compressed_prefix) else {
            return Decoded::MoreInputWanted;
        };
        let mut out = vec![0u8; want];
        let mut got = 0;
        while got < want {
            match decoder.read(&mut out[got..]) {
                Ok(0) => break,
                Ok(n) => got += n,
                // **A prefix that ran out mid-frame is more input wanted, not a
                // failure**: that is exactly the state L9's growth answers.
                Err(_) => break,
            }
        }
        out.truncate(got);
        if got == 0 {
            Decoded::MoreInputWanted
        } else {
            Decoded::Bytes(out)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blend_version::{read_the_tag, PrefixGrowth, PrefixUnavailable, ReadsAPrefix, Tag};

    /// A `.blend`-shaped file: the observed opening text, then a body.
    fn a_blend(head: &[u8], body_bytes: usize, compressible: bool) -> Vec<u8> {
        let mut body = head.to_vec();
        let mut x: u32 = 0x1234_5678;
        while body.len() < body_bytes {
            if compressible {
                body.extend_from_slice(b"mesh mesh mesh mesh ");
            } else {
                x = x.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                body.extend_from_slice(&x.to_le_bytes());
            }
        }
        body
    }

    struct AFileOnAMachine(Vec<u8>);

    impl ReadsAPrefix for AFileOnAMachine {
        async fn read_the_front_of(
            &mut self,
            _path: &str,
            want: usize,
        ) -> Result<Vec<u8>, PrefixUnavailable> {
            Ok(self.0[..want.min(self.0.len())].to_vec())
        }
    }

    #[tokio::test]
    async fn the_five_shape_comes_out_of_a_compressed_file_at_the_first_fetch() {
        for level in [1i32, 3, 19] {
            let whole = zstd::stream::encode_all(
                &a_blend(b"BLENDER17-01v0500REND", 8 * 1024 * 1024, false)[..],
                level,
            )
            .expect("the pinned decoder's own compressor");
            let mut reader = AFileOnAMachine(whole);
            let tag = read_the_tag(
                "/jobs/scene.blend",
                &mut reader,
                &mut TheZstandardDecoder,
                PrefixGrowth::DECLARED,
            )
            .await;
            match tag {
                Tag::SavedWith(version) => assert_eq!(version.as_a_tag(), "5.0"),
                other => panic!("level {level} gave {other:?}"),
            }
        }
    }

    #[tokio::test]
    async fn the_four_five_shape_comes_out_too_and_at_the_first_fetch() {
        let whole =
            zstd::stream::encode_all(&a_blend(b"BLENDER-v405REND", 4 * 1024 * 1024, true)[..], 3)
                .expect("compressed");
        let mut reader = AFileOnAMachine(whole);
        assert!(matches!(
            read_the_tag(
                "/jobs/scene.blend",
                &mut reader,
                &mut TheZstandardDecoder,
                PrefixGrowth::DECLARED
            )
            .await,
            Tag::SavedWith(_)
        ));
    }

    #[tokio::test]
    async fn a_plain_uncompressed_blend_never_reaches_the_decoder_at_all() {
        // L8's attempt order: the plain bytes first. A 4.5 save may be plain,
        // and so may a 5.x one where the preference was changed.
        let mut reader = AFileOnAMachine(a_blend(b"BLENDER-v405REND", 4096, true));
        let tag = read_the_tag(
            "/jobs/scene.blend",
            &mut reader,
            &mut TheZstandardDecoder,
            PrefixGrowth::DECLARED,
        )
        .await;
        assert!(matches!(tag, Tag::SavedWith(_)));
    }

    #[test]
    fn something_that_is_not_this_format_is_said_to_be_and_never_guessed_at() {
        let mut decoder = TheZstandardDecoder;
        assert_eq!(
            decoder.first_decompressed_bytes(b"not a frame at all, just bytes", 64),
            Decoded::NotThisFormat
        );
        // Fewer bytes than the magic number is more input wanted, not a verdict.
        assert_eq!(
            decoder.first_decompressed_bytes(b"\xFD\x2F", 64),
            Decoded::MoreInputWanted
        );
    }

    #[tokio::test]
    async fn a_compressed_file_that_is_not_a_blend_leaves_the_tag_blank() {
        let whole =
            zstd::stream::encode_all(&a_blend(b"NOT A BLENDER FILE", 1024, true)[..], 3).unwrap();
        let mut reader = AFileOnAMachine(whole);
        assert!(matches!(
            read_the_tag(
                "/jobs/thing.dat",
                &mut reader,
                &mut TheZstandardDecoder,
                PrefixGrowth::DECLARED
            )
            .await,
            Tag::Blank(_)
        ));
    }

    #[test]
    fn the_head_comes_out_of_a_prefix_far_smaller_than_the_declared_cap() {
        // The measurement the pin was chosen on, run here so it is a test
        // rather than a note: the head reaches the reader from a prefix of a
        // few dozen bytes, at the worst compression level and on
        // incompressible content.
        let whole = zstd::stream::encode_all(
            &a_blend(b"BLENDER17-01v0500REND", 8 * 1024 * 1024, false)[..],
            19,
        )
        .unwrap();
        let mut decoder = TheZstandardDecoder;
        let mut smallest = None;
        for cap in [16usize, 32, 64, 256, 4096] {
            if let Decoded::Bytes(bytes) =
                decoder.first_decompressed_bytes(&whole[..cap.min(whole.len())], 64)
            {
                if bytes.starts_with(b"BLENDER17-01v0500REND") {
                    smallest = Some(cap);
                    break;
                }
            }
        }
        let smallest = smallest.expect("the head comes out of a bounded prefix");
        assert!(
            smallest <= 4096,
            "and well inside the first fetch, which is {} bytes",
            PrefixGrowth::DECLARED.first_fetch
        );
        assert!(smallest < PrefixGrowth::DECLARED.cap);
    }

    // ---- The four real ends, driven against a far end that speaks the
    // protocol. **What is faked is the wire; the readings under test are the
    // real ones.**

    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    use smoozy_server_protocol::subsystem::kind;
    use smoozy_server_protocol::subsystem::wire::{Attributes, Composed, Fields};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    use crate::already_exists::{walk, Arriving, Comparison, TheQuestionStopped};
    use crate::listing::{DirectoryRead, EntryName, Kind};
    use crate::permission::{walk as walk_the_components, WalkOutcome};

    const A_DIRECTORY: u32 = 0o040_755;
    const A_FILE: u32 = 0o100_644;

    /// One entry as the far end holds it: the name, the size and the mode.
    type AnEntryOnTheMachine = (Vec<u8>, u64, u32);

    /// What the far end holds: folders it will list, files it will serve, and
    /// paths it refuses with a named status.
    #[derive(Default, Clone)]
    struct AMachine {
        folders: Vec<(Vec<u8>, Vec<AnEntryOnTheMachine>)>,
        files: Vec<(Vec<u8>, Vec<u8>)>,
        refuses: Vec<(Vec<u8>, u32)>,
    }

    /// **A far end that lists folders and serves file bytes, over a pipe, at the
    /// desk.**
    ///
    /// It answers exactly the packets these four readings send — the opening
    /// `INIT`, `OPENDIR`, `READDIR` to its end, `OPEN`, byte-offset `READ`s and
    /// `CLOSE` — and refuses everything else rather than inventing an answer.
    /// **It announces no extension**, so the negotiated limits are the draft's
    /// own recommended pair. **Every packet kind it is sent is recorded**, which
    /// is how a test can say that no stat per entry was issued.
    fn a_far_end_holding(machine: AMachine, seen: Arc<Mutex<Vec<u8>>>) -> tokio::io::DuplexStream {
        let (ours, mut theirs) = tokio::io::duplex(1024 * 1024);
        tokio::spawn(async move {
            // Handles this far end has given out, and how far each has been
            // read — a directory handle is drained once, which is what makes the
            // second READDIR the end-of-file status.
            let mut open_directories: HashMap<Vec<u8>, (usize, bool)> = HashMap::new();
            let mut open_files: HashMap<Vec<u8>, usize> = HashMap::new();
            let mut next_handle = 0u32;
            loop {
                let mut length = [0u8; 4];
                if theirs.read_exact(&mut length).await.is_err() {
                    return;
                }
                let mut whole = vec![0u8; u32::from_be_bytes(length) as usize];
                if theirs.read_exact(&mut whole).await.is_err() {
                    return;
                }
                let (packet_kind, payload) = (whole[0], &whole[1..]);
                seen.lock()
                    .expect("the record of what was asked")
                    .push(packet_kind);
                let mut fields = Fields::over(payload);
                let reply = match packet_kind {
                    kind::INIT => {
                        let mut composed = Composed::new();
                        composed.u32(3);
                        composed.framed(kind::VERSION)
                    }
                    kind::OPENDIR => {
                        let id = fields.u32().unwrap_or(0);
                        let path = fields.string().unwrap_or_default().to_vec();
                        if let Some((_, status)) =
                            machine.refuses.iter().find(|(held, _)| held == &path)
                        {
                            a_status(id, *status)
                        } else if let Some(at) =
                            machine.folders.iter().position(|(held, _)| held == &path)
                        {
                            next_handle += 1;
                            let handle = format!("d{next_handle}").into_bytes();
                            open_directories.insert(handle.clone(), (at, false));
                            a_handle(id, &handle)
                        } else {
                            a_status(id, 2)
                        }
                    }
                    kind::READDIR => {
                        let id = fields.u32().unwrap_or(0);
                        let handle = fields.string().unwrap_or_default().to_vec();
                        match open_directories.get_mut(&handle) {
                            Some((at, drained)) if !*drained => {
                                *drained = true;
                                a_name_reply(id, &machine.folders[*at].1)
                            }
                            // **The directory's end, reported as the status
                            // L3 turns on.**
                            Some(_) => a_status(id, 1),
                            None => a_status(id, 4),
                        }
                    }
                    kind::OPEN => {
                        let id = fields.u32().unwrap_or(0);
                        let path = fields.string().unwrap_or_default().to_vec();
                        if let Some((_, status)) =
                            machine.refuses.iter().find(|(held, _)| held == &path)
                        {
                            a_status(id, *status)
                        } else if let Some(at) =
                            machine.files.iter().position(|(held, _)| held == &path)
                        {
                            next_handle += 1;
                            let handle = format!("f{next_handle}").into_bytes();
                            open_files.insert(handle.clone(), at);
                            a_handle(id, &handle)
                        } else {
                            a_status(id, 2)
                        }
                    }
                    kind::READ => {
                        let id = fields.u32().unwrap_or(0);
                        let handle = fields.string().unwrap_or_default().to_vec();
                        let offset = fields.u64().unwrap_or(0) as usize;
                        let length = fields.u32().unwrap_or(0) as usize;
                        match open_files.get(&handle) {
                            Some(at) => {
                                let bytes = &machine.files[*at].1;
                                if offset >= bytes.len() {
                                    // **The end of the file, as that document's
                                    // own read reports it.**
                                    a_status(id, 1)
                                } else {
                                    let to = (offset + length).min(bytes.len());
                                    let mut composed = Composed::new();
                                    composed.u32(id).string(&bytes[offset..to]);
                                    composed.framed(kind::DATA)
                                }
                            }
                            None => a_status(id, 4),
                        }
                    }
                    kind::CLOSE => a_status(fields.u32().unwrap_or(0), 0),
                    _ => a_status(fields.u32().unwrap_or(0), 4),
                };
                if theirs.write_all(&reply).await.is_err() {
                    return;
                }
                if theirs.flush().await.is_err() {
                    return;
                }
            }
        });
        ours
    }

    fn a_status(id: u32, code: u32) -> Vec<u8> {
        let mut composed = Composed::new();
        composed.u32(id).u32(code).string(b"").string(b"");
        composed.framed(kind::STATUS)
    }

    fn a_handle(id: u32, handle: &[u8]) -> Vec<u8> {
        let mut composed = Composed::new();
        composed.u32(id).string(handle);
        composed.framed(kind::HANDLE)
    }

    /// Revision 3's own name reply: a count, then per entry the name, the
    /// display form and the attributes.
    fn a_name_reply(id: u32, entries: &[AnEntryOnTheMachine]) -> Vec<u8> {
        let mut composed = Composed::new();
        composed
            .u32(id)
            .u32(u32::try_from(entries.len()).unwrap_or(0));
        for (name, size, mode) in entries {
            composed
                .string(name)
                .string(b"a rendered line nobody reads");
            composed.attributes(&Attributes {
                size: Some(*size),
                permissions: Some(*mode),
                access_time: None,
                modification_time: Some(1_700_000_000),
            });
        }
        composed.framed(kind::NAME)
    }

    async fn a_channel_over(
        machine: AMachine,
        seen: Arc<Mutex<Vec<u8>>>,
    ) -> Subsystem<tokio::io::DuplexStream> {
        Subsystem::negotiate(a_far_end_holding(machine, seen))
            .await
            .expect("the far end announces revision 3")
    }

    /// **L1's one batched reading, taken over the real route.**
    ///
    /// The four fields come back in that one reading; **no stat is issued at
    /// all**, which is asserted off the packets the far end was actually sent
    /// rather than off a comment; the end is the subsystem's own report (L3);
    /// and a name that does not decode crosses as the bytes the machine sent
    /// (L1a).
    #[tokio::test]
    async fn one_batched_reading_brings_back_the_four_fields_and_no_stat_is_issued() {
        let seen = Arc::new(Mutex::new(Vec::new()));
        let mut subsystem = a_channel_over(
            AMachine {
                folders: vec![(
                    b"/jobs".to_vec(),
                    vec![
                        (b"scene.blend".to_vec(), 4_096, A_FILE),
                        (b"renders".to_vec(), 0, A_DIRECTORY),
                        (b"bad\xff\xfe.txt".to_vec(), 7, A_FILE),
                    ],
                )],
                ..Default::default()
            },
            seen.clone(),
        )
        .await;
        let mut channel = TheReadingChannel::on(&mut subsystem, Lease(1));

        let answer = channel.read_a_directory("/jobs", Lease(1)).await;
        let DirectoryAnswer::Entries {
            entries,
            the_subsystem_reported_the_end,
        } = answer
        else {
            panic!("the folder listed");
        };
        assert!(the_subsystem_reported_the_end);
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0].name.as_bytes(), b"scene.blend");
        assert_eq!(entries[0].size.bytes(), 4_096);
        assert_eq!(entries[0].kind, Kind::File);
        assert_eq!(
            entries[0]
                .last_changed
                .expect("the epoch form")
                .epoch_seconds(),
            1_700_000_000
        );
        assert_eq!(entries[1].kind, Kind::Folder);
        // **L1a: the undecodable name is the machine's bytes and nothing else.**
        assert_eq!(entries[2].name.as_bytes(), b"bad\xff\xfe.txt");

        let asked = seen.lock().expect("the record of what was asked").clone();
        assert!(
            !asked.contains(&kind::STAT) && !asked.contains(&kind::LSTAT),
            "L1: no reading here ever issues a stat per entry, and this asked {asked:?}"
        );
        assert_eq!(asked.iter().filter(|k| **k == kind::OPENDIR).count(), 1);
    }

    /// **L3a, end to end over the real reading.**
    ///
    /// A reopened channel is a new subsystem and a new lease, and the pass taken
    /// on the old one is discarded rather than joined to it. **The lease is
    /// carried by the channel value**, so a caller learns of the change by
    /// asking rather than by being told.
    #[tokio::test]
    async fn a_reopened_channel_is_a_new_lease_and_the_part_read_pass_is_discarded() {
        let machine = AMachine {
            folders: vec![(b"/jobs".to_vec(), vec![(b"a.txt".to_vec(), 1, A_FILE)])],
            ..Default::default()
        };
        let seen = Arc::new(Mutex::new(Vec::new()));
        let mut first = a_channel_over(machine.clone(), seen.clone()).await;
        let mut channel = TheReadingChannel::on(&mut first, Lease(7));
        let mut pass = DirectoryRead::opened("/jobs", channel.the_channels_lease());
        let DirectoryAnswer::Entries { entries, .. } =
            channel.read_a_directory("/jobs", Lease(7)).await
        else {
            panic!("the folder listed");
        };
        // The pass is deliberately left unfinished, which is what a lease change
        // arrives in the middle of.
        pass.a_piece_arrived(channel.the_channels_lease(), entries, false);
        assert!(pass.still_underway());

        // **The channel is reopened**: a second subsystem, and the holder's own
        // next number.
        let mut second = a_channel_over(machine, seen).await;
        let mut reopened = TheReadingChannel::on(&mut second, Lease(8));
        assert_ne!(reopened.the_channels_lease(), Lease(7));
        let DirectoryAnswer::Entries { entries, .. } =
            reopened.read_a_directory("/jobs", Lease(8)).await
        else {
            panic!("the folder listed");
        };
        pass.a_piece_arrived(reopened.the_channels_lease(), entries, true);
        // **Only one open-to-end pass is accepted**, and the discard happened.
        assert_eq!(pass.restarts(), 1);
        assert_eq!(pass.accepted().map(<[_]>::len), None);
    }

    /// **X1's machine side over the real reading**, and X2's two endings.
    ///
    /// An absent receiving folder is the protocol's own no-such-file status and
    /// ends its branch; a refused one stops the question rather than being
    /// skipped, which is the count wrong in the quiet direction X2 exists
    /// against.
    #[tokio::test]
    async fn the_already_exists_walk_reads_one_folder_at_a_time_over_the_real_route() {
        let seen = Arc::new(Mutex::new(Vec::new()));
        let mut subsystem = a_channel_over(
            AMachine {
                folders: vec![(b"/dest".to_vec(), vec![(b"there.txt".to_vec(), 3, A_FILE)])],
                ..Default::default()
            },
            seen.clone(),
        )
        .await;
        let mut channel = TheReadingChannel::on(&mut subsystem, Lease(1));

        let arriving = vec![
            an_arriving_file(&[b"there.txt"]),
            an_arriving_file(&[b"fresh.txt"]),
            // Beneath a folder the machine does not hold: X2 ends the branch.
            an_arriving_file(&[b"tree", b"deep.txt"]),
        ];
        let answer = walk(b"/dest", &arriving, &mut channel, SystemTime::UNIX_EPOCH)
            .await
            .expect("the question was answered");
        assert_eq!(
            answer
                .comparisons
                .iter()
                .map(|comparison| comparison.what)
                .collect::<Vec<_>>(),
            vec![
                Comparison::CollidesWithAFile,
                Comparison::LandsFresh,
                Comparison::LandsFresh
            ]
        );
        assert_eq!(answer.colliding, 1);
        // The destination and the absent folder, and never one per landing
        // place.
        assert_eq!(answer.readings_taken, 2);

        // **And a refusal stops the question.**
        let mut refusing = a_channel_over(
            AMachine {
                refuses: vec![(b"/dest".to_vec(), 3)],
                ..Default::default()
            },
            Arc::new(Mutex::new(Vec::new())),
        )
        .await;
        let mut channel = TheReadingChannel::on(&mut refusing, Lease(1));
        let stopped = walk(
            b"/dest",
            &[an_arriving_file(&[b"a.txt"])],
            &mut channel,
            SystemTime::UNIX_EPOCH,
        )
        .await
        .expect_err("a refused receiving folder stops it");
        assert_eq!(
            stopped,
            TheQuestionStopped::AFolderCouldNotBeRead {
                folder: b"/dest".to_vec(),
                met: WhatSmoozyMet::Refused
            }
        );
    }

    fn an_arriving_file(relative: &[&[u8]]) -> Arriving {
        Arriving {
            relative: relative
                .iter()
                .map(|bytes| EntryName::from_the_machine(bytes.to_vec()))
                .collect(),
            is_a_folder: false,
        }
    }

    /// **P3's walk over the real reading**: one directory reading per
    /// component, and the component that answered *permission denied* named as
    /// the component it is.
    #[tokio::test]
    async fn the_permission_walk_names_the_component_the_machine_actually_refused() {
        let seen = Arc::new(Mutex::new(Vec::new()));
        let mut subsystem = a_channel_over(
            AMachine {
                folders: vec![(b"/jobs".to_vec(), Vec::new())],
                refuses: vec![(b"/jobs/render-3".to_vec(), 3)],
                ..Default::default()
            },
            seen.clone(),
        )
        .await;
        let mut channel = TheReadingChannel::on(&mut subsystem, Lease(1));
        let outcome = walk_the_components(
            "/jobs/render-3/frame.exr",
            &WhatSmoozyMet::Refused,
            &mut channel,
        )
        .await;
        assert_eq!(
            outcome,
            WalkOutcome::RefusedAt("/jobs/render-3".to_string())
        );
        assert!(outcome.the_elevation_question_arises());
        // One reading per component, and it stopped at the refusal rather than
        // walking on.
        assert_eq!(
            seen.lock()
                .expect("the record of what was asked")
                .iter()
                .filter(|k| **k == kind::OPENDIR)
                .count(),
            2
        );
    }

    /// **L7 and L8 over the real reading**: the version comes out of a bounded
    /// prefix fetched by byte-offset reads against an open handle, on this same
    /// channel.
    ///
    /// **Nothing runs Blender and nothing opens the file for it** — the packets
    /// the far end was sent are an open, some reads and a close, and that is
    /// asserted rather than described. **The pieces are the negotiated read
    /// size** and never a figure of Smoozy's own choosing (`Server protocol and
    /// transfers` §1.2 item 2).
    #[tokio::test]
    async fn the_version_tag_comes_off_the_machine_through_the_reading_channel() {
        let whole = zstd::stream::encode_all(
            &a_blend(b"BLENDER17-01v0500REND", 8 * 1024 * 1024, false)[..],
            3,
        )
        .expect("the pinned decoder's own compressor");
        let seen = Arc::new(Mutex::new(Vec::new()));
        let mut subsystem = a_channel_over(
            AMachine {
                files: vec![(b"/jobs/scene.blend".to_vec(), whole)],
                ..Default::default()
            },
            seen.clone(),
        )
        .await;
        let mut channel = TheReadingChannel::on(&mut subsystem, Lease(1));
        let tag = read_the_tag(
            "/jobs/scene.blend",
            &mut channel,
            &mut TheZstandardDecoder,
            PrefixGrowth::DECLARED,
        )
        .await;
        match tag {
            Tag::SavedWith(version) => assert_eq!(version.as_a_tag(), "5.0"),
            other => panic!("the tag came back {other:?}"),
        }
        let asked = seen.lock().expect("the record of what was asked").clone();
        assert_eq!(asked.iter().filter(|k| **k == kind::OPEN).count(), 1);
        assert_eq!(asked.iter().filter(|k| **k == kind::CLOSE).count(), 1);
        assert!(asked.contains(&kind::READ));
        // The first fetch is 4 KiB and the negotiated read is 32 KiB, so the
        // whole of it fits one read — and the tag arrived on that one fetch.
        assert_eq!(asked.iter().filter(|k| **k == kind::READ).count(), 1);
    }

    /// **The negotiated read size is what a prefix is asked for in**, and a
    /// prefix larger than one read comes back whole rather than short.
    #[tokio::test]
    async fn a_prefix_larger_than_one_read_is_taken_in_pieces_of_the_negotiated_size() {
        let bytes: Vec<u8> = (0..100_000u32).map(|n| (n % 251) as u8).collect();
        let seen = Arc::new(Mutex::new(Vec::new()));
        let mut subsystem = a_channel_over(
            AMachine {
                files: vec![(b"/jobs/big.blend".to_vec(), bytes.clone())],
                ..Default::default()
            },
            seen.clone(),
        )
        .await;
        let mut channel = TheReadingChannel::on(&mut subsystem, Lease(1));
        let got = channel
            .read_the_front_of("/jobs/big.blend", 100_000)
            .await
            .expect("the file is there");
        assert_eq!(got, bytes);
        // 32,768 at a time, which is the draft's own recommended read where the
        // server announces no limits: four reads, the last a short one.
        assert_eq!(
            seen.lock()
                .expect("the record of what was asked")
                .iter()
                .filter(|k| **k == kind::READ)
                .count(),
            4
        );
    }

    /// **A short file is a short answer and not a failure**, and a refused open
    /// leaves the tag blank with nothing guessed from it.
    #[tokio::test]
    async fn a_short_file_answers_short_and_a_refused_open_leaves_the_tag_blank() {
        let seen = Arc::new(Mutex::new(Vec::new()));
        let mut subsystem = a_channel_over(
            AMachine {
                files: vec![(b"/jobs/tiny.blend".to_vec(), b"BLENDER-v405REND".to_vec())],
                refuses: vec![(b"/jobs/shut.blend".to_vec(), 3)],
                ..Default::default()
            },
            seen.clone(),
        )
        .await;
        let mut channel = TheReadingChannel::on(&mut subsystem, Lease(1));
        let got = channel
            .read_the_front_of("/jobs/tiny.blend", 4 * 1024)
            .await
            .expect("fewer bytes is a short file and not a failure");
        assert_eq!(got, b"BLENDER-v405REND".to_vec());

        assert_eq!(
            channel
                .read_the_front_of("/jobs/shut.blend", 4 * 1024)
                .await,
            Err(PrefixUnavailable)
        );
        assert_eq!(
            read_the_tag(
                "/jobs/shut.blend",
                &mut channel,
                &mut TheZstandardDecoder,
                PrefixGrowth::DECLARED
            )
            .await,
            Tag::Blank(crate::blend_version::WhyItIsBlank::ThePrefixCouldNotBeRead)
        );
    }
}
