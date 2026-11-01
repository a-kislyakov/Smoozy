//! The readings the judgements stand on, behind one trait the acting half
//! implements — the split `App/Keys/` draws at its `usable::Readings` and
//! `App/Render Settings/` at its `readings::Readings`.
//!
//! **Every reading here is a reading and none is a judgement.** What the
//! judgement does with each is the rest of this crate's: Q5's establishment
//! ([`crate::contents`]), F3's walk and F14a's exactness ([`crate::search`]),
//! F18's look at a shifted location ([`crate::shift`]), A10's contents test
//! ([`crate::landing`]).
//!
//! **What is deliberately not on this trait, because V15 forbids it as
//! evidence**: a file's presence on its own, a size on its own, and any moment
//! on the machine's clock. A reading that answers *absent* answers it as one
//! outcome of a comparison the spec orders (Q6), never as a state.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` S12, S13, S22.
//! `File Browser` §13. `Desktop app and delivery` C15, F10a. `Job files and
//! outputs` N15.

use smoozy_server_protocol::transfers::integrity::Digest;
use smoozy_server_protocol::transfers::SourceReading;

use crate::bytes::MachineLocation;
use crate::bytes::StoredLocation;
use crate::contents::ContentIdentity;
use crate::record::MachineKey;

/// **Q5 route 1's four-value read, and which channel answers it.** The
/// file-transfer surface cannot: revision 3's attribute structure carries no
/// inode and no change time, so the read is a **command-channel reading** on
/// `File Browser` §13 item 9's ground. **Where the channel cannot answer, the
/// fast path simply does not fire and the ordinary contents route runs.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FourValuesNow {
    /// The four values as `Server protocol and transfers` S13's one shape.
    Answered(SourceReading),
    /// Nothing stands at the location. **A comparison's outcome, not a state**
    /// (Q6): Q7 turns it into *not found* with the placement marked gone.
    NothingThere,
    /// Which programs answer such a reading on an arbitrary rental is not
    /// established (`File Browser` §13 item 9), so the reading may be
    /// unavailable — and then nothing is claimed.
    TheChannelCannotAnswer,
}

/// **Q3: the machine copy read through by the digest worker** — Smoozy's own
/// package code, never a tool found on the machine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MachineContents {
    Read(ContentIdentity),
    NothingThere,
    /// Section 9's contents-reading row: **a digest that did not complete is
    /// nothing.** The row is then not established either way.
    DidNotComplete,
}

/// **The stored-name reading — F12 on macOS, F14a on the other two systems —
/// which `Desktop app and delivery`'s platform file adapter owns (F13, its
/// F10a) and this module consumes.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoredNameAnswer {
    /// **The name as stored on disk, byte for byte.** On macOS from F12's
    /// call set (`getattrlist(ATTR_CMN_NAME)`, `fcntl(F_GETPATH)`,
    /// `realpath(3)`); on Windows NTFS and ReFS the last component of
    /// `FileNormalizedNameInformation`'s normalized name (F14a); in a folding
    /// ext4 directory the enumerated name itself (F14a).
    TheStoredName(StoredLocation),
    /// **F14a's bound, the specification's own product-behaviour note**: on
    /// a Windows volume that is neither NTFS nor ReFS the class is refused
    /// with `STATUS_INVALID_DEVICE_REQUEST`. The volume says it cannot answer
    /// rather than handing back a wrong spelling (§12 item 3).
    TheClassIsRefused,
    /// Nothing is at the path — it moved between the enumeration and the
    /// query. F23: the row this was for fails alone.
    NotThere,
}

/// One entry of a folder as enumerated — F8's stored name as the
/// enumeration returned it, and whether it is a folder the walk descends
/// into (F2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumeratedEntry {
    pub name: Vec<u8>,
    pub is_a_folder: bool,
}

/// **F8: the walk enumerates; it never asks the disk whether two names are
/// the same.** This is the enumeration, and the trait offers no lookup by
/// name beside it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FolderListing {
    /// The folder's entries, with the count of entries **the listing itself
    /// could not read**.
    ///
    /// **F7: a failure fails alone.** One entry that will not read is skipped
    /// and named, and the walk carries on over the rest — it does not throw
    /// away every entry gathered so far. **What that cost before 2026-09-05**:
    /// where such a folder was the first in the user's order, its whole
    /// subtree was never walked, the folders below it supplied the answers
    /// instead, and F4's *the first folder in the user's order wins* was
    /// quietly reversed.
    Entries {
        entries: Vec<EnumeratedEntry>,
        /// **The entry has no name to give** — a listing that failed on an
        /// entry failed before it had one — so the count is what is reported,
        /// against the folder it was in.
        that_would_not_read: usize,
    },
    /// F7: **the folder itself** would not open — missing, moved or refused,
    /// one case and not separated (F1a).
    CouldNotBeRead,
}

impl FolderListing {
    /// A listing where every entry read.
    pub fn of(entries: Vec<EnumeratedEntry>) -> FolderListing {
        FolderListing::Entries {
            entries,
            that_would_not_read: 0,
        }
    }
}

/// **What the casing an enumeration returns is worth on this folder's
/// filesystem**, supplied by the acting half from what it knows of the
/// system and the volume, and never inferred here from the names themselves.
///
/// F14a's arrangement *"never rests on what the enumeration's casing does"*
/// where that is the half nobody has written down; where it is established,
/// the enumerated bytes are the stored name and F9's byte equality is the
/// whole test.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnumerationCasing {
    /// **The enumerated name is the stored name.** macOS — Fact 16, *"APFS
    /// stores names byte for byte and normalises only its lookup hash"*; a
    /// case-folding ext4 directory — F14a's kernel citation, *"a
    /// byte-per-byte match to what is actually written in the disk"*; and
    /// every ordinary case-sensitive directory, where there is only one
    /// spelling to return.
    IsTheStoredName,
    /// **Not established.** Windows, where *"what nobody at Microsoft has
    /// written is the words *and its capitalisation*"* (F14), and a folding
    /// Linux filesystem that is not ext4 (F14a, §12 item 11). Exactness is
    /// then decided by the stored-name reading alone, and where that is
    /// refused the search reports the limit.
    NotEstablished,
}

/// The readings behind this document's judgements. **The acting half
/// implements this; nothing in this crate does outside tests.**
pub trait Readings {
    /// **Q2: the local file's content identity, computed by the application
    /// reading the file through.** `None` where the file cannot be read.
    fn the_local_files_content_identity(&self, file: &StoredLocation) -> Option<ContentIdentity>;

    /// **Q5 route 1's command-channel read of the four values now standing
    /// at a machine location.**
    fn the_four_values_now(&self, machine: &MachineKey, at: &MachineLocation) -> FourValuesNow;

    /// **Q3's read-through of the machine copy by the digest worker.**
    fn the_machine_copys_contents(
        &self,
        machine: &MachineKey,
        at: &MachineLocation,
    ) -> MachineContents;

    /// **F8's enumeration of one folder on the user's own computer**, with
    /// the walk's per-folder outcome where it cannot be read (F7).
    fn enumerate_folder(&self, folder: &[u8]) -> FolderListing;

    /// **What the enumeration's casing is worth on the filesystem holding
    /// this folder** — see [`EnumerationCasing`].
    fn enumeration_casing_of(&self, folder: &[u8]) -> EnumerationCasing;

    /// **The stored-name reading of one path** — F12, F13 and F14a — asked
    /// of `Desktop app and delivery`'s file adapter (its C15's sending-end
    /// twin). Asked once per fold-equal candidate where the enumeration's
    /// casing is not established, and once per resolution to record F12's
    /// whole stored location.
    fn the_stored_name_reading(&self, path: &[u8]) -> StoredNameAnswer;

    /// **A10: the digest of the source file's own prefix up to a checkpoint's
    /// offset** — the source read from the user's computer as it now is,
    /// compared against the checkpoint's recorded running digest (`Server
    /// protocol and transfers` S12, S22). `None` where the source cannot be
    /// read that far, which is a difference.
    fn the_sources_prefix_digest(&self, source: &StoredLocation, up_to: u64) -> Option<Digest>;
}

/// **`Job files and outputs` N15 and `Dependency Upload & Remap` Q5's
/// set-row membership: the mirrored
/// folder's members as listed on the transfer surface**, never line-oriented.
/// Kept off the trait above because it is a listing of the machine taken for
/// one comparison, and the comparison is [`crate::contents::set_row_membership`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MirroredFolderListing {
    pub members: Vec<Vec<u8>>,
}
