//! The listing — `File Browser` §1.1 and §1.2, L1 to L6 with L1a, L3a.
//!
//! **What each listed value is worth is settled here; how the list is laid
//! out, sorted and shown is Design's.**
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` 1.1 item 2, 1.2,
//! §12 item 12, G4. `Job files and outputs` N15. `State, identity and recovery`
//! D5, §1.3 rule 5.

use std::time::SystemTime;

use smoozy_server_protocol::subsystem::{Entry, SubsystemFailure};
use smoozy_state::observed::{Clock, MachineClock};

/// **L1a: a name that comes back from the machine is carried as the bytes the
/// subsystem returned, and a listing that decodes eagerly is forbidden.**
///
/// Observed: a directory listing that decoded entry names as UTF-8 **raised**
/// on a non-UTF-8 name — `UnicodeDecodeError('utf-8', b'bad\xff\xfe.txt', 3,
/// 4, 'invalid start byte')` — *while the write, the read-back over the
/// subsystem and the machine's own `ls` all handled the same name*. **So the
/// failure is the reader's, not the machine's, and it fails a name Smoozy is
/// otherwise able to move.**
///
/// The rule this type keeps: **every comparison and every path built from a
/// name uses those bytes, and a decode happens only at the point a name is
/// shown**, where an undecodable byte is replaced for display and never for
/// identity. `Server protocol and transfers` §12 item 12 records this as the
/// second in-Smoozy instance of the same hazard.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntryName(Vec<u8>);

impl EntryName {
    /// The one route in from the machine side.
    pub fn from_the_machine(bytes: Vec<u8>) -> EntryName {
        EntryName(bytes)
    }

    /// The route in from the user's own computer, where a name is read **as
    /// stored on disk** through the platform file adapter (X1, N2). Same rule,
    /// same type: the two sides now hold one rule rather than only the machine
    /// side.
    pub fn as_stored_on_disk(bytes: Vec<u8>) -> EntryName {
        EntryName(bytes)
    }

    /// **Identity.** Every comparison in this crate goes through the bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// **Display, and only display.** An undecodable byte is replaced here and
    /// nowhere else, so a substitution can never reach a comparison or a path.
    pub fn for_display(&self) -> String {
        String::from_utf8_lossy(&self.0).into_owned()
    }

    /// Whether this name decodes as UTF-8 without substitution. Used by
    /// [`crate::keep_both`] to decide whether a trim can avoid splitting a
    /// multi-byte sequence — **a courtesy, never a condition**.
    pub fn decodes_cleanly(&self) -> bool {
        std::str::from_utf8(&self.0).is_ok()
    }
}

/// What a listed thing is — the fourth of L1's four fields.
///
/// **Derived from the file-type field of the mode the subsystem returns, and
/// from nothing else.** The protocol's own definition of that field:
/// `draft-ietf-secsh-filexfer-02` §5, `permissions` is *"a bit mask of file
/// permissions as defined by posix."* — **documented**, and already cited at
/// `Preparation/Architecture/Sources/Server protocol and transfers.sources.md`
/// for the attribute-setting operation. The mask and the two values read from
/// a header shipped on this disk,
/// `<SDK>/usr/include/sys/_types/_s_ifmt.h`, read 2026-09-04:
/// `#define S_IFMT 0170000 /* [XSI] type of file mask */`,
/// `#define S_IFDIR 0040000 /* [XSI] directory */`,
/// `#define S_IFREG 0100000 /* [XSI] regular */`,
/// `#define S_IFLNK 0120000 /* [XSI] symbolic link */`.
///
/// **A mode that is absent, or whose type field is none of the three, is
/// [`Kind::Unknown`] and never a guess.** L8's discipline for the version tag
/// applied to the kind field: an unrecognised shape is shown as unrecognised.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    File,
    Folder,
    /// A symbolic link, told apart because W4 already turns on the difference
    /// between a path and what it reaches: *"a symbolic link or a second mount
    /// makes one location into two"*. **Nothing here follows one** — this
    /// document reads paths, not inodes (§13 item 5).
    Link,
    /// The mode was absent, or its type field is none of the above. **Not a
    /// file and not a folder.**
    Unknown,
}

/// The POSIX type-of-file mask, from the header quoted at [`Kind`].
const S_IFMT: u32 = 0o170000;
const S_IFDIR: u32 = 0o040000;
const S_IFREG: u32 = 0o100000;
const S_IFLNK: u32 = 0o120000;

impl Kind {
    pub fn from_mode(permissions: Option<u32>) -> Kind {
        match permissions {
            None => Kind::Unknown,
            Some(mode) => match mode & S_IFMT {
                S_IFDIR => Kind::Folder,
                S_IFREG => Kind::File,
                S_IFLNK => Kind::Link,
                _ => Kind::Unknown,
            },
        }
    }

    /// Whether the already-exists walk may read into this thing as a receiving
    /// folder (X2, X3). **A link is not a folder here**, because following one
    /// is the alias case W4 declines to model.
    pub fn is_a_folder(self) -> bool {
        matches!(self, Kind::Folder)
    }
}

/// **L5: the size shown is the apparent size, and it is declared as that
/// wherever anything consumes it.**
///
/// The subsystem route carries no allocated-block count, so the sparse file's
/// lie — 500 GB reported on a 60 GB allocation, 0 blocks allocated — **is not
/// catchable on this route** (Fact 7). What that costs: a sparse file shows its
/// claimed size in the browser, which is true of its apparent size and says
/// nothing about disk taken.
///
/// **L6: a zero-length file reads 0 on this route at no extra cost, and
/// nothing more is inferred from it** — presence, name and size are never
/// state (`State, identity and recovery` §1.3 rule 5).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ApparentSize(u64);

impl ApparentSize {
    pub fn of(bytes: u64) -> ApparentSize {
        ApparentSize(bytes)
    }

    pub fn bytes(self) -> u64 {
        self.0
    }
}

/// **L5's other half, stated as a constant so it can be cited.**
///
/// *"No will-it-fit decision is made from a listed size anywhere in this
/// document"* — the free-space reading of [`crate::free_space`] is the only
/// figure about room.
pub const NO_WILL_IT_FIT_DECISION_IS_MADE_FROM_A_LISTED_SIZE: bool = true;

/// **L4: the last-changed time is taken in the epoch form and carries its
/// moment and its clock** (`State, identity and recovery` D5).
///
/// The machine has no idea what time zone anyone using it is in, and no
/// timezone was configured at all on the observed container — **the `+0000` is
/// a fallback and the epoch form is the unambiguous one** (Fact 7).
///
/// **The listing shows that instant rendered as a calendar time in the user's
/// own zone, computed from the epoch value alone.** That rendering is
/// presentation and is not this crate's.
///
/// **No age or elapsed-time presentation is ever derived from it against the
/// desktop's clock** — that would subtract readings taken on two clocks, which
/// D5 forbids. **This type carries no subtraction and no comparison of any
/// kind**, so the arithmetic D5 forbids has nowhere to happen: the only thing
/// anything does with one is render it, and [`LastChanged::on_the_clock_of`]
/// is what names the clock it must be rendered against.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LastChanged {
    seconds_since_the_epoch: i64,
}

impl LastChanged {
    pub fn at_epoch_seconds(seconds: i64) -> LastChanged {
        LastChanged {
            seconds_since_the_epoch: seconds,
        }
    }

    pub fn epoch_seconds(self) -> i64 {
        self.seconds_since_the_epoch
    }

    /// The clock this moment is on — **the machine's own, named by the server
    /// row it was read from**, which is D5's own form.
    pub fn on_the_clock_of(self, machine: MachineClock) -> Clock {
        Clock::AMachine(machine)
    }
}

/// **L4's other half, stated as a constant so it can be cited.**
///
/// *"No age or elapsed-time presentation is ever derived from it against the
/// desktop's clock."*
pub const NO_ELAPSED_TIME_IS_DERIVED_FROM_A_LISTED_TIME: bool = true;

/// One entry of a listing, as the browser holds it — **L1's four fields, from
/// L1's one reading**.
///
/// **L2: no reading of a listing is line-oriented, and no rendered line is
/// ever parsed.** A newline in a file name splits a listing across lines on
/// both routes, which is `Job files and outputs` N15's rule and it binds this
/// document identically: every reading takes the protocol's own fields.
/// **That is kept structurally here** — [`ListedEntry::from_a_protocol_entry`]
/// is the only constructor, and it takes a
/// [`smoozy_server_protocol::subsystem::Entry`]. There is no route into this
/// type from text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListedEntry {
    pub name: EntryName,
    pub size: ApparentSize,
    pub last_changed: Option<LastChanged>,
    pub kind: Kind,
}

impl ListedEntry {
    /// The one constructor. **L2 kept by there being no other.**
    pub fn from_a_protocol_entry(entry: &Entry) -> ListedEntry {
        ListedEntry {
            name: EntryName::from_the_machine(entry.name.clone()),
            size: ApparentSize::of(entry.attributes.size.unwrap_or(0)),
            last_changed: entry
                .attributes
                .modification_time
                .map(|seconds| LastChanged::at_epoch_seconds(i64::from(seconds))),
            kind: Kind::from_mode(entry.attributes.permissions),
        }
    }
}

/// **L1: a folder is listed by the file-transfer subsystem's directory reading
/// on the connection's long-lived reading channel** (`Server protocol and
/// transfers` 1.1 item 2), **and the four fields come back in that one
/// reading, batched.**
///
/// Observed: a 15-entry folder cost zero per-file stats, and 5010 entries cost
/// **52 round trips rather than 5010**, 18.22 s on that route on that box
/// (Fact 7).
///
/// **No reading here ever issues a stat per entry**, and no ordinary listing is
/// taken on the command channel — which program would answer it on an
/// arbitrary rental, and whether its output survives Fact 37's caution, are
/// unestablished (§13 item 9). **The one exception is the elevated retry of
/// §2.2**, which is [`crate::permission`]'s and runs on the command channel
/// because no *configured* subsystem carries elevation (P6).
/// **The reading is asynchronous because the machine is at the other end of a
/// connection**, and its real end — [`crate::wiring::TheReadingChannel`] — is
/// the file-transfer subsystem's own directory reading, which is itself
/// asynchronous. Nothing above this trait blocks a thread on a machine.
pub trait ReadsADirectory {
    /// One reading of one directory, batched. **The lease is what a
    /// channel-lease change moves** — see [`DirectoryRead`].
    fn read_a_directory(
        &mut self,
        path: &str,
        lease: Lease,
    ) -> impl std::future::Future<Output = DirectoryAnswer> + Send;

    /// The lease the reading channel is on now. A change in this value is
    /// L3a's moment.
    fn the_channels_lease(&self) -> Lease;
}

/// **L1's prohibition, stated as a constant so it can be cited from the
/// declaration table and from a test.**
pub const NO_READING_HERE_EVER_ISSUES_A_STAT_PER_ENTRY: bool = true;

/// Which lease of the reading channel a pass was taken on.
///
/// **A lease is not a connection.** `Server protocol and transfers` 1.1 item 2
/// names the transport fact this stands for: reopening loses the server's
/// place. The number is opaque and only ever compared for equality.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Lease(pub u64);

/// What one directory reading returned.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DirectoryAnswer {
    /// Entries, and whether the subsystem reported the directory's end.
    ///
    /// **L3: output arrived is never read as listing complete.** A truncated
    /// listing and a genuinely short one were both four lines, separable only
    /// by the ending status (Fact 7).
    Entries {
        entries: Vec<ListedEntry>,
        the_subsystem_reported_the_end: bool,
    },
    /// The server, or the channel, said no. **The separation is `Server
    /// protocol and transfers` 1.3's and is read rather than derived** —
    /// [`crate::permission`] is where it is told apart.
    Refused(SubsystemFailure),
}

/// One directory's reading, from open to end — **L3 and L3a kept together,
/// because the two rules are one state machine.**
///
/// **L3a: a channel-lease change never turns a partly read directory into an
/// accepted File Browser enumeration.** `Server protocol and transfers` §1.1
/// item 2's continuation — retaining the entries already returned and
/// discarding them from the second pass — *"is sufficient for a best-effort
/// walk and insufficient here"*, because [X2](crate::already_exists) forbids a
/// collision count wrong in the quiet direction.
///
/// **For an interactive listing and for every directory of X1's already-exists
/// walk alike, the whole in-progress result is discarded after a lease change
/// and the directory is read again from its beginning on the reopened channel.
/// Only one open-to-end pass is accepted.**
///
/// **This does not claim a snapshot**: after an accepted walk, X4's race with
/// later machine changes still stands and publication remains the guard
/// against a silent overwrite.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectoryRead {
    path: String,
    lease: Lease,
    so_far: Vec<ListedEntry>,
    ended: bool,
    /// How many times this directory has been read again after a lease change.
    /// **The cost of one lease change is one current directory read again**,
    /// and repeated lease changes repeat that cost while the reading continues
    /// to report underway.
    restarts: u32,
}

impl DirectoryRead {
    /// Open a pass on one lease.
    pub fn opened(path: impl Into<String>, lease: Lease) -> DirectoryRead {
        DirectoryRead {
            path: path.into(),
            lease,
            so_far: Vec::new(),
            ended: false,
            restarts: 0,
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn lease(&self) -> Lease {
        self.lease
    }

    pub fn restarts(&self) -> u32 {
        self.restarts
    }

    /// A piece of the directory came back on the lease the pass is on.
    ///
    /// **A piece that arrives on another lease is not taken.** It is the same
    /// discard L3a orders, arriving from the other side.
    pub fn a_piece_arrived(
        &mut self,
        on: Lease,
        entries: Vec<ListedEntry>,
        the_subsystem_reported_the_end: bool,
    ) {
        if on != self.lease {
            self.the_lease_changed(on);
            return;
        }
        self.so_far.extend(entries);
        self.ended = self.ended || the_subsystem_reported_the_end;
    }

    /// **L3a's whole rule.** The in-progress result is discarded and the
    /// directory is read again from its beginning on the reopened channel.
    pub fn the_lease_changed(&mut self, to: Lease) {
        self.lease = to;
        self.so_far.clear();
        self.ended = false;
        self.restarts += 1;
    }

    /// **L3: a listing is complete when the subsystem reports its end; a
    /// listing cut by the connection is no listing.**
    ///
    /// `None` while the pass is still underway — **and a caller may not read
    /// the entries out of an unaccepted pass**, because there is no route to
    /// them but this one.
    pub fn accepted(&self) -> Option<&[ListedEntry]> {
        if self.ended {
            Some(&self.so_far)
        } else {
            None
        }
    }

    /// Whether the reading still reports underway (G4-shaped). A lease change
    /// does not end it; it restarts it.
    pub fn still_underway(&self) -> bool {
        !self.ended
    }
}

/// **L3's second half: what the cache keeps when a pass is cut.**
///
/// *"The cache of §1.5 keeps the previous complete one rather than the
/// fragment."* Stated here beside L3 and kept at [`crate::cache`], where the
/// last complete entry remains on screen until the replacement pass is
/// accepted.
pub const A_CUT_PASS_NEVER_REPLACES_THE_LAST_COMPLETE_ONE: bool = true;

/// What a completed pass becomes: the listing, with the moment it was taken.
///
/// The moment is the **desktop's own** clock, because it records when Smoozy
/// looked rather than anything the machine said — the listing's own moment
/// Row 346 asks for. It is never subtracted from [`LastChanged`], which is on
/// the machine's clock (D5).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompleteListing {
    pub entries: Vec<ListedEntry>,
    pub taken_at: SystemTime,
    pub clock: Clock,
    /// **P8: what an elevated act shows is what elevation could see, and it is
    /// not remembered as the machine's ordinary truth.**
    pub read_elevated: bool,
}

impl CompleteListing {
    pub fn of(
        entries: Vec<ListedEntry>,
        taken_at: SystemTime,
        read_elevated: bool,
    ) -> CompleteListing {
        CompleteListing {
            entries,
            taken_at,
            clock: Clock::ThisComputer,
            read_elevated,
        }
    }

    /// Byte-for-byte, per X5's rule. **Never a decoded comparison.**
    pub fn holds(&self, name: &EntryName) -> Option<&ListedEntry> {
        self.entries.iter().find(|entry| &entry.name == name)
    }
}

// ════════════════════════════════════════════════════════════════════════════
// A folder's size, counted only when asked — §1.7, L24 to L28, 2026-10-30
// ════════════════════════════════════════════════════════════════════════════
//
// L24 holds here by absence: no reading in this crate gives a folder entry a
// size. L25 to L28 are built where the count is taken and shown, not here: the
// machine's own program walks the folder (L25, L26), and the desktop holds the
// figure beside the listing, drops it on the next reading (L27), and offers the
// act only where the server's program carries the request (L28).

// ════════════════════════════════════════════════════════════════════════════
// Hidden names and the *Show hidden files* choice — §1.6, L19 to L23, Row 520
// ════════════════════════════════════════════════════════════════════════════

/// **The byte a hidden name starts with** — a period, `0x2E` (L19).
pub const THE_BYTE_A_HIDDEN_NAME_STARTS_WITH: u8 = 0x2E;

impl EntryName {
    /// **L19: a hidden name is an entry whose name's first byte is `0x2E`,
    /// read off L1a's bytes and never off a decoded display string.** Files,
    /// folders and every other kind alike. **`.` and `..` are not entries here
    /// in either state**: the reading already drops them.
    ///
    /// A name whose bytes are not valid text is still hidden when its first
    /// byte is a period, because nothing here decodes it.
    pub fn is_hidden(&self) -> bool {
        self.0.first() == Some(&THE_BYTE_A_HIDDEN_NAME_STARTS_WITH)
    }
}

/// **L21: the choice is a filter on composition, and nothing else takes it as
/// an input.** Whether the list shows this entry, given the user's choice.
///
/// **This is the only function in this crate that takes the choice**, and a
/// test says so. X1's walk, X12's pre-flight, M4's refusal and its cached hint,
/// W1 to W5's readings, L11's free space and U8's leftover telling all read the
/// machine or the record whole: a folder deleted, moved or copied takes its
/// hidden contents, an already-exists count includes a hidden collision, and an
/// upload sends a hidden local file like any other (N2).
///
/// **The choice it takes is L20's**: one setting in the local record, app-wide
/// and never keyed by server row, absent meaning off, written only by the
/// user's own act on the checkbox. The record is `Desktop app and delivery`'s,
/// so the keeper is that crate's settings store and this crate holds no copy.
///
/// **L23: it is never a gate on where the browser may stand.** A hidden folder
/// reached directly opens and lists, and its own listing is filtered here like
/// any other; nothing in this crate asks the choice before opening a folder.
///
/// **L21 as amended 2026-10-02: a row that carries a refusal is never left
/// out**, whatever the choice. P2 tells a refusal met on a folder reached
/// directly on that folder's own row in its parent's list, so with the choice
/// off a hidden folder the login may not read would otherwise vanish and the
/// user would see the parent folder with nothing said (Rule 8).
pub fn the_list_shows(name: &EntryName, it_carries_a_refusal: bool, hidden_files_are_shown: bool) -> bool {
    hidden_files_are_shown || it_carries_a_refusal || !name.is_hidden()
}

/// **L22: turning the choice off removes every hidden entry from the selection
/// at the same moment**, so no act is ever aimed at a name the list does not
/// show. **Turning it on adds nothing to the selection.** The pasteboard (7.7)
/// is not a selection and is never handed here. **A row that carries a refusal
/// is never chosen** — it is not a button — so none is handed here, and the
/// filter is asked as for a row without one.
pub fn the_selection_after_the_choice<'a>(
    selected: impl IntoIterator<Item = &'a EntryName>,
    hidden_files_are_shown: bool,
) -> Vec<&'a EntryName> {
    selected
        .into_iter()
        .filter(|name| the_list_shows(name, false, hidden_files_are_shown))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_server_protocol::subsystem::wire::Attributes;

    fn an_entry(name: &[u8], size: u64, mode: u32, modified: u32) -> Entry {
        Entry {
            name: name.to_vec(),
            attributes: Attributes {
                size: Some(size),
                permissions: Some(mode),
                access_time: None,
                modification_time: Some(modified),
            },
        }
    }

    #[test]
    fn a_name_that_does_not_decode_is_still_a_name() {
        // The exact bytes the observed reader raised on.
        let name = EntryName::from_the_machine(b"bad\xff\xfe.txt".to_vec());
        assert_eq!(name.as_bytes(), b"bad\xff\xfe.txt");
        assert!(!name.decodes_cleanly());
        // It shows, with the undecodable bytes replaced — and the replacement
        // never reaches the identity.
        assert!(name.for_display().contains('\u{fffd}'));
        assert_ne!(name.for_display().as_bytes(), name.as_bytes());
    }

    #[test]
    fn two_spellings_of_one_accented_name_are_two_names() {
        // X5's rule met at the type: no normalisation anywhere.
        let composed = EntryName::from_the_machine("\u{e9}.png".as_bytes().to_vec());
        let decomposed = EntryName::from_the_machine("e\u{301}.png".as_bytes().to_vec());
        assert_ne!(composed, decomposed);
    }

    #[test]
    fn the_four_fields_come_from_one_protocol_entry() {
        let listed = ListedEntry::from_a_protocol_entry(&an_entry(
            b"scene.blend",
            4096,
            0o100644,
            1_700_000_000,
        ));
        assert_eq!(listed.name.as_bytes(), b"scene.blend");
        assert_eq!(listed.size.bytes(), 4096);
        assert_eq!(listed.kind, Kind::File);
        assert_eq!(listed.last_changed.unwrap().epoch_seconds(), 1_700_000_000);
        assert_eq!(
            listed
                .last_changed
                .unwrap()
                .on_the_clock_of(MachineClock("server-7".to_string())),
            Clock::AMachine(MachineClock("server-7".to_string()))
        );
    }

    #[test]
    fn the_kind_comes_from_the_modes_type_field_and_an_unknown_one_is_not_guessed() {
        assert_eq!(Kind::from_mode(Some(0o040755)), Kind::Folder);
        assert_eq!(Kind::from_mode(Some(0o100644)), Kind::File);
        assert_eq!(Kind::from_mode(Some(0o120777)), Kind::Link);
        // A socket is none of the three and is not folded into one.
        assert_eq!(Kind::from_mode(Some(0o140755)), Kind::Unknown);
        // No mode at all.
        assert_eq!(Kind::from_mode(None), Kind::Unknown);
        // And a link is not a folder for the walk.
        assert!(!Kind::Link.is_a_folder());
        assert!(!Kind::Unknown.is_a_folder());
        assert!(Kind::Folder.is_a_folder());
    }

    #[test]
    fn a_zero_length_file_reads_zero_and_nothing_more_is_inferred() {
        let listed = ListedEntry::from_a_protocol_entry(&an_entry(b"empty", 0, 0o100644, 1));
        assert_eq!(listed.size.bytes(), 0);
        assert_eq!(listed.kind, Kind::File);
    }

    #[test]
    fn a_truncated_listing_and_a_genuinely_short_one_are_separated_by_the_end_status() {
        let four = vec![
            ListedEntry::from_a_protocol_entry(&an_entry(b"a", 1, 0o100644, 1)),
            ListedEntry::from_a_protocol_entry(&an_entry(b"b", 1, 0o100644, 1)),
        ];

        let mut genuinely_short = DirectoryRead::opened("/jobs", Lease(1));
        genuinely_short.a_piece_arrived(Lease(1), four.clone(), true);
        assert_eq!(genuinely_short.accepted().map(<[_]>::len), Some(2));

        let mut truncated = DirectoryRead::opened("/jobs", Lease(1));
        truncated.a_piece_arrived(Lease(1), four, false);
        assert_eq!(truncated.accepted(), None);
        assert!(truncated.still_underway());
    }

    #[test]
    fn a_lease_change_discards_the_whole_pass_and_reads_again_from_the_beginning() {
        let mut read = DirectoryRead::opened("/jobs", Lease(1));
        read.a_piece_arrived(
            Lease(1),
            vec![ListedEntry::from_a_protocol_entry(&an_entry(
                b"one", 1, 0o100644, 1,
            ))],
            false,
        );
        read.the_lease_changed(Lease(2));
        assert_eq!(read.accepted(), None);
        assert_eq!(read.restarts(), 1);
        assert_eq!(read.lease(), Lease(2));

        // The whole directory again, on the reopened channel, and only then
        // accepted.
        read.a_piece_arrived(
            Lease(2),
            vec![
                ListedEntry::from_a_protocol_entry(&an_entry(b"one", 1, 0o100644, 1)),
                ListedEntry::from_a_protocol_entry(&an_entry(b"two", 1, 0o100644, 1)),
            ],
            true,
        );
        assert_eq!(read.accepted().map(<[_]>::len), Some(2));
    }

    #[test]
    fn a_piece_arriving_on_another_lease_is_never_folded_into_the_pass() {
        // The fault L3a exists against: entries retained across a reopen,
        // which can miss a name added or moved behind the new cursor.
        let mut read = DirectoryRead::opened("/jobs", Lease(1));
        read.a_piece_arrived(
            Lease(1),
            vec![ListedEntry::from_a_protocol_entry(&an_entry(
                b"one", 1, 0o100644, 1,
            ))],
            false,
        );
        read.a_piece_arrived(
            Lease(7),
            vec![ListedEntry::from_a_protocol_entry(&an_entry(
                b"two", 1, 0o100644, 1,
            ))],
            true,
        );
        // Neither piece survives, and the pass is not accepted on the strength
        // of the second one's end marker.
        assert_eq!(read.accepted(), None);
        assert_eq!(read.restarts(), 1);
    }

    #[test]
    fn repeated_lease_changes_repeat_the_cost_and_never_accept_a_partial_pass() {
        let mut read = DirectoryRead::opened("/jobs", Lease(1));
        for lease in 2..=5 {
            read.a_piece_arrived(
                Lease(lease - 1),
                vec![ListedEntry::from_a_protocol_entry(&an_entry(
                    b"x", 1, 0o100644, 1,
                ))],
                false,
            );
            read.the_lease_changed(Lease(lease));
        }
        assert_eq!(read.restarts(), 4);
        assert_eq!(read.accepted(), None);
        assert!(read.still_underway());
    }

    #[test]
    fn a_listing_holds_a_name_by_bytes_and_not_by_display() {
        let listing = CompleteListing::of(
            vec![ListedEntry::from_a_protocol_entry(&an_entry(
                b"Case.txt",
                1,
                0o100644,
                1,
            ))],
            SystemTime::UNIX_EPOCH,
            false,
        );
        assert!(listing
            .holds(&EntryName::from_the_machine(b"Case.txt".to_vec()))
            .is_some());
        assert!(listing
            .holds(&EntryName::from_the_machine(b"case.txt".to_vec()))
            .is_none());
        assert_eq!(listing.clock, Clock::ThisComputer);
    }

    /// **L19: hidden is the first byte, files and folders alike, and a name
    /// that does not decode is still hidden** — test 9 of the plan text.
    #[test]
    fn a_hidden_name_is_one_whose_first_byte_is_a_period() {
        let name = |bytes: &[u8]| EntryName::from_the_machine(bytes.to_vec());
        assert!(name(b".smoozy-staging").is_hidden());
        assert!(name(b".bashrc").is_hidden());
        assert!(name(b".\xff\xfe").is_hidden(), "a name whose bytes are not text is still hidden");
        assert!(name(b".\xff\xfe").for_display().starts_with('.'));
        assert!(!name(b"kitchen.blend").is_hidden(), "a period later in the name is not hidden");
        assert!(!name(b" .hidden").is_hidden());
        assert!(!name(b"").is_hidden());
    }

    /// **L21 and L22: the filter leaves hidden names out only while the choice
    /// is off, and turning it off drops them from the selection.**
    #[test]
    fn the_choice_filters_the_list_and_the_selection_and_nothing_more() {
        let hidden = EntryName::from_the_machine(b".cache".to_vec());
        let plain = EntryName::from_the_machine(b"renders".to_vec());
        assert!(!the_list_shows(&hidden, false, false));
        assert!(the_list_shows(&hidden, false, true));
        assert!(the_list_shows(&plain, false, false));
        assert!(the_list_shows(&plain, false, true));
        assert_eq!(the_selection_after_the_choice([&hidden, &plain], false), vec![&plain]);
        assert_eq!(the_selection_after_the_choice([&hidden, &plain], true), vec![&hidden, &plain]);
        assert!(the_selection_after_the_choice([&hidden], false).is_empty());
    }

    /// **L21 as amended 2026-10-02: a row that carries a refusal is never left
    /// out**, so P2's refusal on a hidden folder reached directly is told with
    /// the choice off.
    #[test]
    fn a_row_that_carries_a_refusal_is_never_left_out() {
        let hidden = EntryName::from_the_machine(b".cache".to_vec());
        assert!(the_list_shows(&hidden, true, false), "a refused hidden folder vanished with the box off");
        assert!(the_list_shows(&hidden, true, true));
    }

    /// **L21: no reading, walk or act in this crate filters by the choice** —
    /// the filter is the one function that takes it, and nothing else in the
    /// crate's working code calls it or tests a name for being hidden.
    #[test]
    fn nothing_but_the_list_takes_the_choice() {
        let folder = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut found = Vec::new();
        for entry in std::fs::read_dir(&folder).expect("the source folder") {
            let path = entry.expect("an entry").path();
            if path.extension().and_then(|e| e.to_str()) != Some("rs") {
                continue;
            }
            let text = std::fs::read_to_string(&path).expect("a source file");
            let working = text.split("#[cfg(test)]").next().unwrap_or_default();
            for asked in ["is_hidden()", "the_list_shows(", "hidden_files_are_shown"] {
                if working.contains(asked) {
                    found.push(path.file_name().unwrap().to_str().unwrap().to_string());
                    break;
                }
            }
        }
        assert_eq!(found, vec!["listing.rs".to_string()], "something but the list reads the choice");
    }
}
