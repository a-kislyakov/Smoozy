//! The real ends of this document's machine-side seams.
//!
//! **This is the only file in this crate that knows a protocol exists.**
//! Everything above it takes the machine's answers as values, which is what
//! keeps the walk, the vouching, the narrowing, the fetch pipeline and the
//! request testable at the desk.
//!
//! **`Server protocol and transfers` §1.1's prohibition is kept by there being
//! no command string here at all.** Every path below crosses as a protocol
//! field on the file-transfer subsystem, which is the first carriage that
//! document names — *"length-delimited data on a route no shell parses"* — and
//! **every one of them crosses as bytes**, which is `Job files and outputs`
//! N15's and `File Browser` L1a's rule and the reason nothing here decodes a
//! name.
//!
//! # What is not here, and why
//!
//! **The local acts.** The name check, the exclusive create of the final name,
//! the write-failure classes, the local enumeration and deletion, and the
//! folder-reveal are `Desktop app and delivery`'s platform file adapter's —
//! C3, C10, C13, F1 to F9, F10, F10a and K20 to K24 — and that crate depends
//! on the feature modules and supplies their adapters, so a feature module
//! cannot reach into it. **Nothing is stubbed for them here**; the seams are
//! the traits and the adapter that fills them is that document's.
//!
//! **`Server protocol and transfers` S13's four-value reading.** It is a
//! command-channel reading, because revision 3's attribute structure carries
//! neither the inode nor the change time — the subsystem's own
//! [`Attributes`](smoozy_server_protocol::subsystem::Attributes) says so.
//! **No command is composed here for it.** [`super::fetching::TheFourValues`]
//! is the seam; the desktop reads it over the server's own point (the 38's
//! piece 9), and that header says what stands where it cannot be taken.
//!
//! **The connected period.** Nobody in this workspace opens, holds or reopens
//! a reading channel for a connected period, and `Server protocol and
//! transfers` T5's own code says admission is the caller's. So what is wired
//! here is the reading, not the holding: one of these borrows one subsystem,
//! and whoever opened it hands it in.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` §1.1, §1.2, §1.3,
//! T5, S13. `Job files and outputs` N15. `File Browser` L1, L1a.
//! `Desktop app and delivery` C3, C10, C13, F1, F9, F10, F10a, K20, K24.

use smoozy_server_protocol::subsystem::client::Subsystem;
use smoozy_server_protocol::subsystem::SubsystemFailure;
use tokio::io::{AsyncRead, AsyncWrite};

use super::record_directory::ReadsOneFileOnTheMachine;
use super::walking::{ADirectoryEntry, ReadsOneDirectoryOnTheMachine, TheEntrysKind};

/// **The reading route both of this document's machine-side readings ride.**
///
/// `Server protocol and transfers` T5 gives a connection **one** long-lived
/// reading channel, and W2's aggregate bound is arithmetic about that one
/// channel. **One of these holds one `&mut` to one subsystem, so two readings
/// cannot be in flight on it at once** — the serialisation is a property of
/// the code rather than a rule somebody has to keep, which is exactly how
/// `File Browser`'s own reading channel keeps it.
///
/// **It is a borrow and not a holder.** The channel is opened, held and
/// reopened by whoever owns the connected period, and this type is what a walk
/// or a record-directory read is handed for the duration of one act.
pub struct TheReadingRoute<'a, S> {
    subsystem: &'a mut Subsystem<S>,
}

impl<'a, S> TheReadingRoute<'a, S> {
    /// The route, over a channel its holder opened.
    pub fn on(subsystem: &'a mut Subsystem<S>) -> TheReadingRoute<'a, S> {
        TheReadingRoute { subsystem }
    }
}

/// **The file-type bits of a Unix mode, and the directory value among them.**
///
/// This is the one bit of a mode this file reads and it reads nothing else. It
/// is not a permission check: whether the walk may descend is answered by the
/// reading itself refusing, which is `Server protocol and transfers` §1.3's
/// separation and not a mode this crate interprets.
///
/// # Whether every server this app renders on says which entries are folders
/// is an open question, and it is not answered here
///
/// **The field this reads is optional.** `Attributes::permissions` is
/// `Option<u32>` in this workspace's own protocol crate, filled only where the
/// server set the permissions flag in its reply, and the revision-3 attributes
/// structure the crate models carries **no separate kind field at all**. So on
/// this route the kind is one bit of a value a server may or may not send.
///
/// **The question is parked with the protocol crate's owner**: does the
/// listing carry an entry's kind on the servers Smoozy connects to, and if not,
/// is there any other field on this route that carries it? **No answer is
/// guessed at here, and no publication is cited, because none has answered
/// it.** The one measurement the plan cites for this route is one box.
///
/// **What stands while it is open** is
/// [`TheEntrysKind::ItWasNotCarried`](super::walking::TheEntrysKind::ItWasNotCarried):
/// the absence is carried as an absence, the walk that met it is discarded and
/// retried at the cadence, and nothing reads it as *this is not a folder*.
/// That costs one walk cadence per unanswered entry and fails towards fetching
/// late; reading it as a file fails towards a job reported as having produced
/// nothing.
const THE_TYPE_BITS: u32 = 0o170_000;
const A_DIRECTORY: u32 = 0o040_000;
/// **A plain file's type bits.** Added 2026-09-30, step 10c's eleventh-pass
/// fix piece 2. **Documented**: the header shipped on this disk,
/// `<SDK>/usr/include/sys/_types/_s_ifmt.h`, read 2026-09-30:
/// `#define S_IFREG 0100000 /* [XSI] regular */` — the same header, and the
/// same line, `File Browser`'s `listing.rs` quotes for its own `Kind::File`,
/// under `draft-ietf-secsh-filexfer-02` §5's *"a bit mask of file permissions
/// as defined by posix."* **Only a plain file's listed size is the file's
/// own**; a symbolic link's is the link's.
const A_PLAIN_FILE: u32 = 0o100_000;

impl<S: AsyncRead + AsyncWrite + Unpin + Send> ReadsOneDirectoryOnTheMachine
    for TheReadingRoute<'_, S>
{
    /// **W1's route and W4's four fields, in one reading per folder.**
    ///
    /// *"The subsystem route takes the protocol's own fields — kind, size,
    /// modification time, name, batched at about 100 entries per reply"* — so
    /// **no text is mined and a name holding a newline cannot split a
    /// reading** (`Job files and outputs` N15). No stat per entry is issued:
    /// the fields come back with the names in the one reading.
    ///
    /// **The route answers only after the server has reported the directory's
    /// end**, so a reading cut mid-listing comes back as a failure rather than
    /// as a short complete listing — which is W1's fourth reason holding by
    /// construction rather than by a value composed here.
    ///
    /// **A missing size or modification time reads as zero and nothing else.**
    /// Revision 3 makes both optional fields; a size that is absent is not a
    /// size of zero as a *fact*, and nothing downstream reads it as one — J1's
    /// zero-byte rule is about a file the machine said is zero bytes long, and
    /// an entry with no size never becomes a judged-finished frame's evidence
    /// because [`crate::vouching::a_judged_finished_frame_is_fetchable`] is
    /// driven off the inventory's own entries. **What an absent field costs is
    /// a fetch that waits**, which is the direction this document fails in
    /// everywhere else.
    async fn read_one_directory(
        &mut self,
        path: &[u8],
    ) -> Result<Vec<ADirectoryEntry>, SubsystemFailure> {
        let entries = self.subsystem.list(path).await?;
        Ok(entries
            .iter()
            .map(|entry| ADirectoryEntry {
                name: entry.name.clone(),
                // **W1 treats *kind* as a field the route supplies, and on
                // this route it is one bit of a value a server may or may not
                // send.** Revision 3's attributes structure carries no
                // separate kind field at all and its permissions field is
                // optional, filled only where the server set the permissions
                // flag in its reply. **An absent value is carried as an
                // absence and never read as *this is not a folder***: that
                // would be a silent wrong answer, and it is the caller's to
                // decide what to do with, not this file's to guess at.
                kind: match entry.attributes.permissions {
                    Some(mode) if mode & THE_TYPE_BITS == A_DIRECTORY => TheEntrysKind::AFolder,
                    Some(mode) if mode & THE_TYPE_BITS == A_PLAIN_FILE => TheEntrysKind::APlainFile,
                    Some(_) => TheEntrysKind::NotAFolder,
                    None => TheEntrysKind::ItWasNotCarried,
                },
                size: entry.attributes.size.unwrap_or(0),
                modification_time: entry.attributes.modification_time.unwrap_or(0) as i64,
            })
            .collect())
    }
}

impl<S: AsyncRead + AsyncWrite + Unpin + Send> ReadsOneFileOnTheMachine for TheReadingRoute<'_, S> {
    /// **One whole small file back, on the same reading route.**
    ///
    /// The two files J2 and N2 name — the attempt record and the manifest —
    /// are records Smoozy itself wrote, and `Job files and outputs` M8 and M1
    /// fix both shapes, so this reads to the end and hands back the bytes.
    /// **Nothing here parses one**; the reading and the reading back are two
    /// acts and only the first is the machine's.
    ///
    /// **The read is batched at the connection's own negotiated read size**
    /// and ends where the server hands back nothing, which is the subsystem's
    /// own end-of-file — `Server protocol and transfers`' client answers an
    /// empty run of bytes there rather than a failure.
    ///
    /// **The handle is closed on every path out**, including the failing one:
    /// a handle left open on a long-lived reading channel is a handle the
    /// server counts against its own open-handle limit for the life of the
    /// connection.
    async fn read_one_whole_file(&mut self, path: &[u8]) -> Result<Vec<u8>, SubsystemFailure> {
        let handle = self.subsystem.open_to_read(path).await?;
        let chunk = self.subsystem.announcement().limits.read_chunk();
        let mut bytes = Vec::new();
        let outcome = loop {
            match self
                .subsystem
                .read_at(&handle, bytes.len() as u64, chunk)
                .await
            {
                Ok(read) if read.is_empty() => break Ok(()),
                Ok(read) => bytes.extend_from_slice(&read),
                Err(failure) => break Err(failure),
            }
        };
        // The close's own refusal is not the read's answer: the bytes either
        // came back or they did not, and a server that will not close a handle
        // says nothing about them.
        let _ = self.subsystem.close(&handle).await;
        outcome.map(|()| bytes)
    }
}

#[cfg(test)]
mod tests {
    /// **§1.1's prohibition, kept by there being no command string in this
    /// crate at all.**
    ///
    /// *"No user path, package path, option, identifier or other value of
    /// user, provider or machine origin is ever interpolated into a command
    /// string."* This crate composes none, which is the strongest form of it,
    /// and the test reads the crate's own sources rather than trusting the
    /// claim.
    #[test]
    fn no_command_string_is_composed_anywhere_in_this_crate() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut looked_at = 0usize;
        let mut still_to_read = vec![root];
        while let Some(at) = still_to_read.pop() {
            for entry in std::fs::read_dir(&at).expect("the crate's own sources are readable") {
                let path = entry.expect("a directory entry").path();
                if path.is_dir() {
                    still_to_read.push(path);
                    continue;
                }
                if path.extension().and_then(|it| it.to_str()) != Some("rs") {
                    continue;
                }
                let source = std::fs::read_to_string(&path).expect("a source file is readable");
                looked_at += 1;
                // **The needles are joined here rather than written out**,
                // because a source-reading test that spells what it forbids
                // finds itself in the file it is written in.
                for composing in [
                    ["CommandString", "::constant"].concat(),
                    ["run_one", "_command"].concat(),
                    ["a_command", "_naming"].concat(),
                ] {
                    assert!(
                        !source.contains(&composing),
                        "{} composes or runs a command: {composing}",
                        path.display()
                    );
                }
            }
        }
        assert!(
            looked_at > 10,
            "the sweep read the crate rather than nothing"
        );
    }
}
