//! The record — `Dependency Upload & Remap` §1, Row 178: K1–K11.
//!
//! **Eight promises across five sections of the spec read this record, and it
//! is the only place the artist's file's remap lives.** Losing it costs the
//! user every decision they made by hand, and nothing anywhere holds a copy.
//!
//! **K1: every durable fact this module keeps on the user's side is a record
//! kind inside the local record** (`State, identity and recovery` §1.1).
//! Five kinds, the list closed: the located-answer record (K3), the placement
//! record (K7), the fixing record (K8), the check standing (V10) and the
//! shift record (K6). The per-job resolved set (K9) and M22f's placed-copy
//! list are fields of the job's own record, not a sixth kind.
//!
//! **K2: nothing about this record is ever written on a rented machine**, with
//! exactly two machine-side exceptions, both material rather than state: the
//! remap material of 6.4 and the fixed copies of 6.5 — derived from the
//! record, re-derivable from it, and never read back as state.
//!
//! **K11: the record answers *where did this live* and *what did the user
//! decide*; it never answers *what does this file need now*.** Only the check
//! answers that, and the check reads the file every time it runs. Kept
//! structurally: **no function in this crate reads a located answer as a
//! statement about the file's current contents** — the record carries no
//! field for one, and [`TheRecord`] offers no question of that shape.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` S9, S13. `Job
//! files and outputs` R10, R15. `State, identity and recovery` §1.1, §2.1,
//! §2.2, §2.3, §11.1, D5, J5. `File Browser` W1, W1a.

use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use smoozy_server_protocol::transfers::SourceReading;
use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_state::lifetimes::Lifetime;
use smoozy_state::observed::{Clock, Reading};

use crate::bytes::{MachineLocation, RecordedLocation, StoredLocation};
use crate::contents::ContentIdentity;
use crate::standing::CheckStanding;

/// **K1's five kinds, closed.**
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RecordKind {
    /// K3–K5: keyed with no machine in it.
    LocatedAnswer,
    /// K7: one per (machine, mirrored location).
    Placement,
    /// K8: one per (machine, fixed copy).
    Fixing,
    /// 7.3 V10: one per (root file's machine path, machine).
    CheckStanding,
    /// K6: written when a hand-locate teaches a shift.
    Shift,
}

/// **K1: five and no more.** A builder that needs a sixth has found a fault
/// in §1 rather than a case it does not cover.
pub const THE_FIVE_KINDS: [RecordKind; 5] = [
    RecordKind::LocatedAnswer,
    RecordKind::Placement,
    RecordKind::Fixing,
    RecordKind::CheckStanding,
    RecordKind::Shift,
];

/// **K9: the per-job resolved set is a field of the job's own local record
/// rather than a sixth kind.** And **M22f: so is the placed-copy list**, in the
/// same shape.
pub const THE_RESOLVED_SET_AND_THE_PLACED_COPY_LIST_ARE_FIELDS_OF_THE_JOBS_RECORD: bool = true;

/// **K10 and U3: what each kind survives, as `State, identity and recovery`
/// §11.1's class.** Located answers and shift records carry no machine key and
/// survive the machine outright — **user-held**. Placement records, fixing
/// records and check standings describe a machine and are disqualified with
/// its installation identity — **subject-held** on their server row. Nothing
/// here has an age, a cap, or an elapsed-time lifetime.
pub fn lifetime_of(kind: RecordKind) -> Lifetime {
    match kind {
        RecordKind::LocatedAnswer | RecordKind::Shift => Lifetime::UserHeld,
        RecordKind::Placement | RecordKind::Fixing | RecordKind::CheckStanding => {
            Lifetime::SubjectHeld
        }
    }
}

/// **K2's rule as a value**: the two machine-side things derived from the
/// record are material, and neither is ever read back as state.
pub const THE_MACHINE_SIDE_MATERIAL_IS_NEVER_READ_BACK_AS_STATE: bool = true;

/// **K11 as a value**: no path in this crate reads a located answer as a
/// statement about the file's current contents.
pub const THE_RECORD_NEVER_ANSWERS_WHAT_THE_FILE_NEEDS_NOW: bool = true;

/// **K7's keying: under the server row and the installation identity it was
/// placed under.** A record carrying a superseded installation identity is
/// **disqualified rather than ended** (`State, identity and recovery`
/// §11.1) — a recycled machine answers on the same address with everything
/// gone, and the installation identity is the only thing that tells (its
/// §2.3).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MachineKey {
    pub server_row: Identity,
    pub installation: Identity,
}

impl MachineKey {
    pub fn of(server_row: Identity, installation: Identity) -> MachineKey {
        debug_assert_eq!(server_row.kind(), IdentityKind::LocalServer);
        debug_assert_eq!(installation.kind(), IdentityKind::Installation);
        MachineKey {
            server_row,
            installation,
        }
    }

    /// Whether a record under this key stands under the server row's
    /// **current** installation identity. A disqualified record answers
    /// nothing anywhere below.
    pub fn is_current_under(&self, the_rows_current_installation: &Identity) -> bool {
        &self.installation == the_rows_current_installation
    }
}

/// **K3's holding file — identified by its source location on the user's own
/// computer, as stored on disk (F12) — and K5's one degenerate case.**
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HoldingFile {
    /// For the file the user points at, the source path `File Browser` hands
    /// over; for a linked file below it, the source location its own
    /// resolution recorded. **No machine and no installation identity is any
    /// part of this key**, which is what makes the answer travel.
    OnTheUsersComputer(StoredLocation),
    /// **K5: a file that was already on the machine with no known local
    /// origin.** Its references still get located answers, keyed by the
    /// holding file's machine location under the machine's installation
    /// identity — and those answers do not travel. A relative location inside
    /// such a file resolves against its machine folder and nothing else.
    OnTheMachineWithNoKnownLocalOrigin {
        machine: MachineKey,
        location: MachineLocation,
    },
}

impl HoldingFile {
    /// **K3's whole point: the answer travels with the file.** K5's case does
    /// not, because there is no user-side file for it to travel with.
    pub fn its_answers_travel(&self) -> bool {
        matches!(self, HoldingFile::OnTheUsersComputer(_))
    }
}

/// **K3: one located-answer record per (holding file, reference as the file
/// records it), and the reference key is the recorded location's exact bytes,
/// capitals included.** Two references reading `METallic.png` and
/// `metallic.png` are two records with two answers.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LocatedAnswerKey {
    pub holding_file: HoldingFile,
    pub reference: RecordedLocation,
}

/// **K6's shift record key — a record key inside the local record, and not an
/// identity in `State, identity and recovery` §2.2's register.** No identity
/// anywhere is derived from one, and none of §2.1's rules is touched.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ShiftKey(pub u64);

/// **F5's arbitrary pick, recorded so the row can show it**: the pick taken,
/// the candidates it was taken among, and that it was a coin toss rather than
/// a judgment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArbitraryPick {
    pub taken: StoredLocation,
    pub among: Vec<StoredLocation>,
}

/// **K4: how a reference was resolved** — one of four.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HowResolved {
    /// The recorded location itself resolved.
    TheRecordedLocationItself,
    /// The search — from which searched folder, and whether the pick was
    /// arbitrary and among which candidates (F4, F5).
    TheSearch {
        from_folder: Vec<u8>,
        arbitrary: Option<ArbitraryPick>,
    },
    /// A shift, naming the shift record (F19).
    AShift(ShiftKey),
    /// The user's own hand — a Locate…, or a proposal accepted (F10, F24).
    TheUsersOwnHand,
}

/// **K4: what the reference resolved to** — a whole location on the user's
/// computer as stored on disk, or a folder for a set row (3.6).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedTo {
    /// **K4's *"a whole location on the user's computer, as stored on
    /// disk"*** — a file, or the folder a folder-shaped reference names
    /// (F28's G6 kinds). **This was `AFile` until 2026-09-05**, which was the
    /// code's own narrowing of K4 and left a folder-shaped reference's answer
    /// with nowhere to go.
    AWholeLocation(StoredLocation),
    /// **K4's *"or a folder for a set row"*** (3.6).
    AFolderForASetRow(StoredLocation),
}

impl ResolvedTo {
    pub fn location(&self) -> &StoredLocation {
        match self {
            ResolvedTo::AWholeLocation(location) | ResolvedTo::AFolderForASetRow(location) => {
                location
            }
        }
    }
}

/// A resolution as the record holds it: what, and how.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resolution {
    pub to: ResolvedTo,
    pub how: HowResolved,
}

/// **K4: what a located answer holds.** Each write of one is one durable write
/// made at the moment the user acts or the search resolves, never batched to
/// the end of a sitting (G16).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocatedAnswer {
    pub key: LocatedAnswerKey,
    /// Absent where the row is *not found* — F7: a *not found* row is the
    /// **absence** of a resolution rather than a statement about the disk.
    pub resolution: Option<Resolution>,
    /// The **set-aside** mark, remembered beyond the sitting.
    pub set_aside: bool,
    /// A **rejected guess** — a proposal the user turned down, kept so it is
    /// not put forward again by itself and can be reinstated.
    pub rejected_guess: Option<StoredLocation>,
    /// **K4's last limb, added 2026-09-29 (step 10c's dependency wave piece
    /// 7b): what the check read the reference as** — every kind and setting
    /// under which the latest reading of the holding file carried these bytes,
    /// once each, in the reading's order. **Each reading writes it afresh and
    /// a decision carries it unchanged.** It is what M10 composes a pair's
    /// kind and setting from, so the pairs need nothing a run holds.
    ///
    /// **Empty where no reading has written it**, and such an answer gives no
    /// pair until the file is checked again. **It says what the check read,
    /// never what the file holds now** (K11).
    pub read_as: Vec<ReadAs>,
}

/// **K4: one kind and one setting a reference was read under** — M4's
/// *"the data-block kind and the setting that holds the location"*, as the
/// check's reading carried them (`crate::check::ReadReference`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReadAs {
    pub kind: crate::check::Kind,
    /// A property path rather than an instance (M8).
    pub setting: String,
}

impl ReadAs {
    /// **K4: every kind and setting the reading carried `reference` under,
    /// once each, in the reading's order.**
    pub fn in_the_reading(
        references: &[crate::check::ReadReference],
        reference: &RecordedLocation,
    ) -> Vec<ReadAs> {
        let mut read_as: Vec<ReadAs> = Vec::new();
        for one in references.iter().filter(|one| &one.recorded == reference) {
            let this = ReadAs {
                kind: one.kind,
                setting: one.setting.clone(),
            };
            if !read_as.contains(&this) {
                read_as.push(this);
            }
        }
        read_as
    }
}

impl LocatedAnswer {
    /// **F20: whether this answer was produced by the named shift**, which is
    /// what the one-action undo enumerates by.
    pub fn was_produced_by(&self, shift: ShiftKey) -> bool {
        matches!(
            &self.resolution,
            Some(Resolution {
                how: HowResolved::AShift(key),
                ..
            }) if *key == shift
        )
    }
}

/// **K6: a shift record — the old root, the new root, and the moment.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShiftRecord {
    pub key: ShiftKey,
    pub old_root: Vec<u8>,
    pub new_root: Vec<u8>,
    /// On the desktop's clock, stated as that clock (`State, identity and
    /// recovery` D5).
    pub at: Reading<()>,
}

impl ShiftRecord {
    pub fn written(
        key: ShiftKey,
        old_root: Vec<u8>,
        new_root: Vec<u8>,
        at: SystemTime,
    ) -> ShiftRecord {
        ShiftRecord {
            key,
            old_root,
            new_root,
            at: Reading::taken((), at, Clock::ThisComputer),
        }
    }
}

/// **Q7's confirmation outcome, written into the placement record with the
/// moment it was taken.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmationOutcome {
    /// *Already on the machine* re-established.
    Confirmed,
    /// The file is absent: back to *not found*, and the record is marked
    /// gone.
    Gone,
    /// Present, values moved, and the read-through differed or did not
    /// complete — not confirmed.
    NotConfirmed,
}

/// A confirmation as the record holds it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Confirmation {
    pub outcome: ConfirmationOutcome,
    pub at: Reading<()>,
}

/// **Q11: a published file is whole by the contract, and publication is the
/// only event that creates a placement record.** This is what a publication
/// hands over, and [`PlacementRecord::at_publication`] is the record's one
/// constructor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Published {
    pub at: MachineLocation,
    pub source: StoredLocation,
    /// The transfer's own pass-through digest beside the size (Q1, `Server
    /// protocol and transfers` S22).
    pub content: ContentIdentity,
    /// The four-value identity read at publication — `Server protocol and
    /// transfers` S13's and `Job files and outputs` R10's one shape.
    pub four_values: SourceReading,
}

/// **K7: one placement record per (machine, mirrored location).**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlacementRecord {
    pub machine: MachineKey,
    pub mirrored_location: MachineLocation,
    /// The source location the upload came from — without which a relative
    /// location cannot be resolved after the file has moved, and the shift
    /// has nothing to shift from (F17).
    pub source_location: StoredLocation,
    pub content: ContentIdentity,
    pub four_values_at_publication: SourceReading,
    /// Whether a later confirmation has re-established it (Q7).
    pub confirmation: Option<Confirmation>,
    /// Q7: marked gone where a confirmation found the file absent.
    pub marked_gone: bool,
}

impl PlacementRecord {
    /// **Q11: the one constructor, and it takes a publication.** A transfer
    /// that ended any other way leaves staged material and **no placement
    /// record**, so an interrupted upload cannot leave the record believing a
    /// file is up. Nothing here takes a listing, a presence or a size.
    pub fn at_publication(machine: MachineKey, published: Published) -> PlacementRecord {
        PlacementRecord {
            machine,
            mirrored_location: published.at,
            source_location: published.source,
            content: published.content,
            four_values_at_publication: published.four_values,
            confirmation: None,
            marked_gone: false,
        }
    }

    /// **Q7: the confirmation's outcome, written in with the moment.** A
    /// *gone* outcome marks the record gone.
    pub fn confirmed(mut self, outcome: ConfirmationOutcome, at: SystemTime) -> PlacementRecord {
        self.confirmation = Some(Confirmation {
            outcome,
            at: Reading::taken((), at, Clock::ThisComputer),
        });
        if outcome == ConfirmationOutcome::Gone {
            self.marked_gone = true;
        }
        self
    }

    /// Whether the record answers *a file was placed here* under the
    /// current installation identity and has not been found gone.
    pub fn stands_under(&self, current_installation: &Identity) -> bool {
        self.machine.is_current_under(current_installation) && !self.marked_gone
    }
}

/// **A published transfer record's half that this module reads** — `Server
/// protocol and transfers` S9, which `File Browser`'s hand-uploads carry —
/// so Q5's fast path and Q9's lookup can name a file this module did not
/// place.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublishedTransfer {
    pub machine: MachineKey,
    pub at: MachineLocation,
    pub content: ContentIdentity,
    pub four_values_at_publication: SourceReading,
}

/// **K8: one fixing record per (machine, fixed copy).**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixingRecord {
    pub machine: MachineKey,
    /// The **machine location** of the copy the fixed copy was derived from —
    /// the mirror where this module placed the file, or the file's own
    /// machine location where it is there by another route — and that copy's
    /// content identity.
    pub derived_from: MachineLocation,
    pub derived_from_content: ContentIdentity,
    /// Where the fixed copy sits (5.1 A3).
    pub fixed_copy: MachineLocation,
    /// **What the fixing points at** — the (reference bytes → machine
    /// location) pairs written into it, which is also the list M16's second
    /// replacement trigger compares against the current located answers.
    pub points_at: Vec<(RecordedLocation, MachineLocation)>,
}

/// **K9: a submitted job's resolved set as composed at submission** — the
/// machine locations its remap material names, mirrors and fixed copies,
/// beside the manifest's `.blend`, **and the remap material's own machine
/// location** (M10). Immutable for the life of the job's record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedSet {
    pub locations: Vec<MachineLocation>,
    pub remap_material: MachineLocation,
}

impl ResolvedSet {
    /// Whether the set names a machine location — the material included, so
    /// V2's points-at reading, V6's live-job rule and V3's delete warning
    /// reach it with no new mechanism.
    pub fn names(&self, location: &MachineLocation) -> bool {
        &self.remap_material == location || self.locations.contains(location)
    }
}

/// **A live or waiting job as the record knows it, for V5 and V6**: the
/// manifest's `.blend` (`Job files and outputs` R15 supplies it with its
/// identity) and K9's resolved set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiveJob {
    pub job: Identity,
    pub machine: MachineKey,
    pub blend: MachineLocation,
    pub resolved_set: ResolvedSet,
}

impl LiveJob {
    /// **V5's whole answer to *what does this job depend on***: the `.blend`
    /// and the resolved set, one durable answer for the job's whole life.
    pub fn depends_on(&self, location: &MachineLocation) -> bool {
        &self.blend == location || self.resolved_set.names(location)
    }
}

/// **The local record as this module reads it.** Every question below is
/// answered from the record alone (V1) and none reaches a machine. The store
/// itself is `Desktop app and delivery` §5's; this trait is the shape of what
/// this module asks of it, and **it is the trait `File Browser`'s warning
/// layer reads for its W1 and W1a.**
pub trait TheRecord {
    fn placements_on(&self, machine: &MachineKey) -> Vec<PlacementRecord>;
    fn published_transfers_on(&self, machine: &MachineKey) -> Vec<PublishedTransfer>;
    fn fixings_on(&self, machine: &MachineKey) -> Vec<FixingRecord>;
    fn located_answer(&self, key: &LocatedAnswerKey) -> Option<LocatedAnswer>;
    fn located_answers_of(&self, holding_file: &HoldingFile) -> Vec<LocatedAnswer>;
    fn shift(&self, key: ShiftKey) -> Option<ShiftRecord>;
    fn live_or_waiting_jobs_on(&self, machine: &MachineKey) -> Vec<LiveJob>;
    fn check_standing(&self, root: &MachineLocation, machine: &MachineKey)
        -> Option<CheckStanding>;
}

/// **A record held in process memory, for tests and for nothing durable.**
/// `State, identity and recovery` §1.1 J5 makes the local record the only
/// durable store on this side; this is a test double of its shape and never a
/// second store.
#[derive(Debug, Default)]
pub struct InMemoryRecord {
    pub placements: Vec<PlacementRecord>,
    pub published_transfers: Vec<PublishedTransfer>,
    pub fixings: Vec<FixingRecord>,
    pub located_answers: Vec<LocatedAnswer>,
    pub shifts: Vec<ShiftRecord>,
    pub jobs: Vec<LiveJob>,
    pub standings: Vec<CheckStanding>,
}

impl TheRecord for InMemoryRecord {
    fn placements_on(&self, machine: &MachineKey) -> Vec<PlacementRecord> {
        self.placements
            .iter()
            .filter(|record| &record.machine == machine)
            .cloned()
            .collect()
    }
    fn published_transfers_on(&self, machine: &MachineKey) -> Vec<PublishedTransfer> {
        self.published_transfers
            .iter()
            .filter(|record| &record.machine == machine)
            .cloned()
            .collect()
    }
    fn fixings_on(&self, machine: &MachineKey) -> Vec<FixingRecord> {
        self.fixings
            .iter()
            .filter(|record| &record.machine == machine)
            .cloned()
            .collect()
    }
    fn located_answer(&self, key: &LocatedAnswerKey) -> Option<LocatedAnswer> {
        self.located_answers
            .iter()
            .find(|answer| &answer.key == key)
            .cloned()
    }
    fn located_answers_of(&self, holding_file: &HoldingFile) -> Vec<LocatedAnswer> {
        self.located_answers
            .iter()
            .filter(|answer| &answer.key.holding_file == holding_file)
            .cloned()
            .collect()
    }
    fn shift(&self, key: ShiftKey) -> Option<ShiftRecord> {
        self.shifts.iter().find(|shift| shift.key == key).cloned()
    }
    fn live_or_waiting_jobs_on(&self, machine: &MachineKey) -> Vec<LiveJob> {
        self.jobs
            .iter()
            .filter(|job| &job.machine == machine)
            .cloned()
            .collect()
    }
    fn check_standing(
        &self,
        root: &MachineLocation,
        machine: &MachineKey,
    ) -> Option<CheckStanding> {
        self.standings
            .iter()
            .find(|standing| &standing.root == root && &standing.machine == machine)
            .cloned()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use smoozy_server_protocol::transfers::integrity::Digest;

    pub fn a_machine() -> MachineKey {
        MachineKey::of(
            Identity::issue(IdentityKind::LocalServer).expect("randomness"),
            Identity::issue(IdentityKind::Installation).expect("randomness"),
        )
    }

    pub fn contents(bytes: &[u8]) -> ContentIdentity {
        ContentIdentity {
            size: bytes.len() as u64,
            digest: Digest::of(bytes),
        }
    }

    pub fn four_values(size: u64, inode: &str, modified: i64, changed: i64) -> SourceReading {
        SourceReading {
            size,
            inode: inode.to_string(),
            modification_time: modified,
            change_time: changed,
        }
    }

    pub fn a_publication(at: &[u8], source: &[u8], bytes: &[u8]) -> Published {
        Published {
            at: MachineLocation::composed(at.to_vec()),
            source: StoredLocation::as_the_system_returned_it(source),
            content: contents(bytes),
            four_values: four_values(bytes.len() as u64, "77", 100, 100),
        }
    }

    #[test]
    fn the_five_kinds_take_the_lifetimes_k10_gives_them() {
        assert_eq!(lifetime_of(RecordKind::LocatedAnswer), Lifetime::UserHeld);
        assert_eq!(lifetime_of(RecordKind::Shift), Lifetime::UserHeld);
        assert_eq!(lifetime_of(RecordKind::Placement), Lifetime::SubjectHeld);
        assert_eq!(lifetime_of(RecordKind::Fixing), Lifetime::SubjectHeld);
        assert_eq!(
            lifetime_of(RecordKind::CheckStanding),
            Lifetime::SubjectHeld
        );
        assert_eq!(THE_FIVE_KINDS.len(), 5);
    }

    #[test]
    fn two_references_differing_only_in_capitals_are_two_keys() {
        let holder = HoldingFile::OnTheUsersComputer(StoredLocation::as_the_system_returned_it(
            b"/Users/alice/proj/scene.blend",
        ));
        let one = LocatedAnswerKey {
            holding_file: holder.clone(),
            reference: RecordedLocation::as_the_reading_carried_it(b"//tex/METallic.png"),
        };
        let other = LocatedAnswerKey {
            holding_file: holder,
            reference: RecordedLocation::as_the_reading_carried_it(b"//tex/metallic.png"),
        };
        assert_ne!(one, other);
    }

    #[test]
    fn a_k5_holding_files_answers_do_not_travel_and_a_users_do() {
        let travels = HoldingFile::OnTheUsersComputer(StoredLocation::as_the_system_returned_it(
            b"/Users/alice/a.blend",
        ));
        let stays = HoldingFile::OnTheMachineWithNoKnownLocalOrigin {
            machine: a_machine(),
            location: MachineLocation::composed(
                b"/ephemeral/smoozy/assets/files/x/a.blend".to_vec(),
            ),
        };
        assert!(travels.its_answers_travel());
        assert!(!stays.its_answers_travel());
    }

    #[test]
    fn a_placement_record_is_made_from_a_publication_and_a_superseded_installation_disqualifies_it()
    {
        let machine = a_machine();
        let record = PlacementRecord::at_publication(
            machine.clone(),
            a_publication(
                b"/root/assets/files/Users/a/x.png",
                b"/Users/a/x.png",
                b"red",
            ),
        );
        assert!(record.stands_under(&machine.installation));
        let recycled = Identity::issue(IdentityKind::Installation).expect("randomness");
        assert!(!record.stands_under(&recycled));
    }

    #[test]
    fn a_gone_confirmation_marks_the_record_gone_and_a_confirmed_one_does_not() {
        let machine = a_machine();
        let record = PlacementRecord::at_publication(
            machine.clone(),
            a_publication(
                b"/root/assets/files/Users/a/x.png",
                b"/Users/a/x.png",
                b"red",
            ),
        );
        let confirmed = record
            .clone()
            .confirmed(ConfirmationOutcome::Confirmed, SystemTime::UNIX_EPOCH);
        assert!(!confirmed.marked_gone);
        assert_eq!(
            confirmed.confirmation.as_ref().map(|c| c.outcome),
            Some(ConfirmationOutcome::Confirmed)
        );
        let gone = record.confirmed(ConfirmationOutcome::Gone, SystemTime::UNIX_EPOCH);
        assert!(gone.marked_gone);
        assert!(!gone.stands_under(&machine.installation));
    }

    #[test]
    fn a_resolved_set_names_the_remap_material_beside_the_assets() {
        let material = MachineLocation::composed(b"/root/assets/remaps/abc".to_vec());
        let mirror = MachineLocation::composed(b"/root/assets/files/Users/a/x.png".to_vec());
        let set = ResolvedSet {
            locations: vec![mirror.clone()],
            remap_material: material.clone(),
        };
        assert!(set.names(&material));
        assert!(set.names(&mirror));
        assert!(!set.names(&MachineLocation::composed(
            b"/root/assets/files/other".to_vec()
        )));
    }

    #[test]
    fn a_located_answer_knows_which_shift_produced_it() {
        let key = LocatedAnswerKey {
            holding_file: HoldingFile::OnTheUsersComputer(
                StoredLocation::as_the_system_returned_it(b"/a.blend"),
            ),
            reference: RecordedLocation::as_the_reading_carried_it(b"/work/x.png"),
        };
        let answer = LocatedAnswer {
            key,
            resolution: Some(Resolution {
                to: ResolvedTo::AWholeLocation(StoredLocation::as_the_system_returned_it(
                    b"/Archive/x.png",
                )),
                how: HowResolved::AShift(ShiftKey(4)),
            }),
            set_aside: false,
            rejected_guess: None,
            read_as: Vec::new(),
        };
        assert!(answer.was_produced_by(ShiftKey(4)));
        assert!(!answer.was_produced_by(ShiftKey(5)));
    }
}
