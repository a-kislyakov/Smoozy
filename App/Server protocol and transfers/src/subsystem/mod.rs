//! The file-transfer subsystem channel — §1.1's **second mechanism**.
//!
//! **§1.1: the surface is made of exactly two mechanisms, and a builder needs
//! both named, because they have different framing, different failure text and
//! different capability questions.** The first is the command channel, next
//! door in [`crate::transport`]. This is the second: *"The standard SSH
//! file-transfer subsystem, which is what carries directory listing, file
//! attributes, file operations, free-space readings, and every byte of every
//! transfer. Its operations are byte-offset reads and writes against an open
//! handle, which is what lets Smoozy own the byte loop rather than hand a whole
//! file to a tool."*
//!
//! **No `sftp` program is run for it** — `Servers` K1a — and there is no
//! second connection: the subsystem is a session on the connection
//! [`crate::transport::ssh::Connection`] holds, admitted and counted under T5
//! like every other session.
//!
//! **Three things are negotiated and none is inferred from a version number**
//! (§1.2). The announced extension set and the declared operation limits are
//! both read from the server's own opening reply, once per channel, and
//! recorded against the connected period — never against the server row,
//! because the machine may have been replaced under the address. **A capability
//! that is not announced is never attempted and never guessed at.**
//!
//! **The variance is real and it is what makes this load-bearing rather than a
//! courtesy.** Observed on three machines across three sellers: the two servers
//! on OpenSSH 9.x announced **eleven** extensions and the server on **8.9p1**
//! announced the **first eight**, missing `copy-data`, `home-directory` and
//! `users-groups-by-id@openssh.com`. Every extension this document leans on was
//! present on all three, and the declared limits were byte-identical across all
//! three despite the version gap.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Servers` K1a.

pub mod client;
pub mod wire;

pub use wire::{extension, kind, open_flag, Attributes, StatusCode};

/// The one handle an open returns. **Four bytes on the servers observed — an
/// index into that server's own table — and it is opaque here.**
///
/// **A handle does not survive its channel**, which is the observation T5's
/// lease change turns on: a server read its own handle at offset 0 and returned
/// `SSH_FXP_DATA`; a second server handed that same handle returned
/// `SSH_FXP_STATUS` code **4 `Failure`**; and after the first channel was
/// closed, a third server handed it returned the same. Posed again on a rental
/// with a control — *"the second channel then served `stat` on the same path
/// and returned size 21. So the channel was alive and it was the handle that
/// was refused."*
///
/// **So an operation holding an open handle when the lease changes resumes by
/// re-opening, and never by re-using**, and the type carries no way to move one
/// between channels: it is returned by a [`client::Subsystem`] and taken only
/// by that same one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Handle(Vec<u8>);

/// What the server said in its opening reply — §1.2 items 1 and 2 together.
///
/// **Held against the connected period.** Nothing carries one of these over a
/// connection `Servers` replaced (C11 step 2: *"Negotiate capabilities afresh.
/// Nothing negotiated on the previous connection is carried over."*).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Announcement {
    /// The version the server named. **Read rather than assumed**, and
    /// **compared in [`client::Subsystem::negotiate`]**, which is the one place
    /// a channel is made: a reply naming anything but
    /// [`Announcement::REVISION_THIS_DOCUMENT_USES`] never becomes an
    /// `Announcement` at all. So every value this field can hold is the
    /// revision this document's operations and attribute structure are written
    /// against.
    ///
    /// **It used to say *this is where it is checked* and nothing compared
    /// it.** A server announcing anything else was parsed as revision 3 anyway,
    /// so its attributes came back as wrong sizes and wrong times rather than
    /// as a refusal.
    pub version: u32,
    /// The extension names, exactly as announced. **Nothing is added and
    /// nothing is inferred from the version.**
    pub extensions: Vec<String>,
    /// §1.2 item 2's four figures.
    pub limits: Limits,
}

impl Announcement {
    /// **§1.2's rule, as one question.** A capability that is not in this list
    /// is never attempted.
    pub fn announces(&self, name: &str) -> bool {
        self.extensions.iter().any(|announced| announced == name)
    }

    /// **The revision this document's operations and attribute structure are
    /// written against.** A server naming anything else is met by the caller
    /// rather than worked around here.
    pub const REVISION_THIS_DOCUMENT_USES: u32 = 3;
}

/// The subsystem's declared operation limits — §1.2 item 2.
///
/// **The fallback pair is the protocol's own published recommendation and not
/// Smoozy's invention.** `draft-ietf-secsh-filexfer-02` Section 3, read
/// 2026-08-20: *"All servers SHOULD support packets of at least 34000 bytes
/// (where the packet size refers to the full length, including the header
/// above). This should allow for reads and writes of at most 32768 bytes."*
///
/// **What stays true is the caution rather than the invention**: the draft says
/// **SHOULD** and not **MUST**, so what an unannouncing server will actually
/// accept is recommended by the specification and guaranteed by nobody. §12
/// item 3 labels that and it is **permanently unresolved as an observation** —
/// reaching this branch wants a rental that announces no limits and the Creator
/// ruled on 2026-08-31 that there will be no further rented machine for
/// answering a question. **The branch has never been reached on four boxes
/// across three sellers**, and it is untested rather than disproved.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limits {
    pub largest_packet: u64,
    pub largest_read: u64,
    pub largest_write: u64,
    /// **A per-machine reading rather than a constant** — the three servers
    /// read agreed at 1,019 and the caller's own end reported 1,048,571.
    pub open_handles: u64,
}

impl Limits {
    /// The pair the draft recommends, for a server that announces none.
    pub const RECOMMENDED_WHERE_NONE_ARE_ANNOUNCED: Limits = Limits {
        largest_packet: 34_000,
        largest_read: 32_768,
        largest_write: 32_768,
        // The draft states no number, so nothing is claimed. One at a time is
        // what an unannouncing server is asked for.
        open_handles: 1,
    };

    /// **R6's byte half.** The write chunk is the largest write, chosen inside
    /// the negotiated limit — never above it, and never a figure of Smoozy's
    /// own choosing where the server has stated one.
    pub fn write_chunk(&self) -> usize {
        usize::try_from(self.largest_write).unwrap_or(usize::MAX)
    }

    /// The read chunk, on the same rule.
    pub fn read_chunk(&self) -> usize {
        usize::try_from(self.largest_read).unwrap_or(usize::MAX)
    }
}

/// One entry of a directory listing.
///
/// **The name is bytes.** §12 item 12: a listing that decodes entry names as
/// UTF-8 raises on a name the write, the read-back and the machine's own `ls`
/// all coped with, so *"the machine-side enumeration must carry names as bytes
/// and decode only for display"*.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub name: Vec<u8>,
    pub attributes: Attributes,
}

/// What `statvfs@openssh.com` answered for one path — F2's first route.
///
/// **F3: the available figure is the one read, never the free figure**, because
/// of the root-reserved blocks. Both are carried so that nothing has to go back
/// to the wire, and [`FilesystemStatistics::available_bytes`] is the only
/// composition anything uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FilesystemStatistics {
    pub fragment_size: u64,
    pub blocks: u64,
    pub free_blocks: u64,
    pub available_blocks: u64,
}

impl FilesystemStatistics {
    /// **F3.** The available figure, in bytes.
    pub fn available_bytes(&self) -> u64 {
        self.available_blocks.saturating_mul(self.fragment_size)
    }
}

/// **Why an operation did not answer as asked, at the path it has always been
/// reached at.**
///
/// The type sits in the other half of this module, because
/// [`crate::free_space::WhyThereIsNoReading`] carries one and that shape is
/// composed on both sides of the connection. **What each of its five answers
/// means is written at the type itself**, including the one raised by
/// [`client::Subsystem::negotiate`] and by nothing else — a server offering
/// another revision — since the negotiation is the only moment a revision is
/// read.
pub use smoozy_framing::subsystem_answers::SubsystemFailure;

impl Handle {
    pub(crate) fn of(bytes: Vec<u8>) -> Handle {
        Handle(bytes)
    }

    pub(crate) fn bytes(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn an_announcement(extensions: &[&str]) -> Announcement {
        Announcement {
            version: 3,
            extensions: extensions.iter().map(|name| name.to_string()).collect(),
            limits: Limits::RECOMMENDED_WHERE_NONE_ARE_ANNOUNCED,
        }
    }

    #[test]
    fn the_eight_nine_server_is_a_real_case_and_the_three_that_vary_are_the_tail() {
        // Observed: the 8.9p1 server announced the first eight and the two 9.x
        // servers eleven. Every extension this document leans on was present on
        // all three, which is what this test states.
        let older = an_announcement(&[
            extension::POSIX_RENAME,
            extension::STATVFS,
            "fstatvfs@openssh.com",
            "hardlink@openssh.com",
            extension::FSYNC,
            "lsetstat@openssh.com",
            extension::LIMITS,
            "expand-path@openssh.com",
        ]);
        assert!(older.announces(extension::POSIX_RENAME));
        assert!(older.announces(extension::STATVFS));
        assert!(older.announces(extension::FSYNC));
        assert!(!older.announces("copy-data"));
    }

    #[test]
    fn a_capability_not_announced_is_never_attempted() {
        // §1.2's rule at the type. A server announcing nothing answers no to
        // every one of the three, and the caller degrades in a declared way
        // rather than trying it to see.
        let bare = an_announcement(&[]);
        assert!(!bare.announces(extension::POSIX_RENAME));
        assert!(!bare.announces(extension::STATVFS));
        assert!(!bare.announces(extension::FSYNC));
    }

    #[test]
    fn the_fallback_pair_is_the_drafts_own_recommendation() {
        // 32,768 for each of read and write, and 34,000 for the packet — the
        // draft's own numbers, quoted at `Limits`.
        let limits = Limits::RECOMMENDED_WHERE_NONE_ARE_ANNOUNCED;
        assert_eq!(limits.largest_read, 32_768);
        assert_eq!(limits.largest_write, 32_768);
        assert_eq!(limits.largest_packet, 34_000);
    }

    #[test]
    fn the_write_chunk_is_the_negotiated_limit_and_never_a_figure_of_our_own() {
        // The three rentals read reported max-write 261,120, and R6's byte half
        // is that number rather than one chosen here.
        let observed = Limits {
            largest_packet: 262_144,
            largest_read: 261_120,
            largest_write: 261_120,
            open_handles: 1_019,
        };
        assert_eq!(observed.write_chunk(), 261_120);
        assert_eq!(observed.read_chunk(), 261_120);
    }

    #[test]
    fn the_available_figure_is_the_one_read() {
        // F3, and the container reading that answered to the byte: `f_frsize`
        // 4096, `f_blocks` 15,728,640, `f_bavail` 15,728,451 — 64,424,509,440
        // bytes, exactly the 60 GiB rented.
        let statistics = FilesystemStatistics {
            fragment_size: 4096,
            blocks: 15_728_640,
            free_blocks: 15_728_451,
            available_blocks: 15_728_451,
        };
        assert_eq!(statistics.available_bytes(), 64_423_735_296);
    }

    #[test]
    fn refusal_and_absence_are_two_answers() {
        // §1.3's whole supply to `File Browser`: a separation rather than a
        // limit, and no message anywhere may say the two cannot be told apart.
        let gone = SubsystemFailure::TheServerRefused(StatusCode::NoSuchFile);
        let refused = SubsystemFailure::TheServerRefused(StatusCode::PermissionDenied);
        assert!(gone.is_the_path_not_being_there());
        assert!(!gone.is_the_login_not_being_allowed());
        assert!(refused.is_the_login_not_being_allowed());
        assert!(!refused.is_the_path_not_being_there());
    }
}
