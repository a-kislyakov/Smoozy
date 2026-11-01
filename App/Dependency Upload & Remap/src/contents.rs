//! Telling two files apart by what is inside them — `Dependency Upload &
//! Remap` §2, Row 168: Q1–Q11.
//!
//! **Four different questions are put to one mechanism, and the mechanism is
//! one content identity compared at whichever end holds the bytes.** *Already
//! on the machine* as a row state; the overwrite warning; the second reference
//! to a file already up; and confirming a *sent before, not confirmed* row.
//!
//! **Q10: nothing in this module ever marks a row *already on the machine*,
//! *sent before, not confirmed*, or anything else from a file's presence, name
//! or size.** A part-arrived file carries all three convincingly (Fact 18) and
//! carries nothing else; every state below comes from a record Smoozy wrote or
//! a contents comparison Smoozy ran. **Kept by the types**: no function here
//! takes a presence, a name or a size as an argument on its own, and the
//! [`Establishment`] a row gets has no constructor from any of them.
//!
//! **Q6: the four-value read and the read-through are deliberate comparisons
//! the spec orders, not readings of state.** A file found at a location no
//! record names, and not read through, is **never** *already on the machine*.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` S9, S13, S22,
//! S23. `Job files and outputs` N15.

use smoozy_server_protocol::transfers::integrity::Digest;
use smoozy_state::identities::Identity;

use crate::bytes::MachineLocation;
use crate::bytes::StoredLocation;
use crate::check::set_member_matches;
use crate::readings::{FourValuesNow, MachineContents, MirroredFolderListing, Readings};
use crate::record::{MachineKey, PlacementRecord, PublishedTransfer, TheRecord};

/// **Q1: the content identity is the pair (size in bytes, digest), and the
/// digest function is the one `Server protocol and transfers` S23 fixes per
/// protocol version** — the transfer's pass-through digest (S22) **is** the
/// content identity of what was published, and no second read of a file that
/// just crossed the wire is ever taken. **Equality of the pair is this
/// module's definition of *the same file***; the size is compared beside the
/// digest, and a partial can never satisfy it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentIdentity {
    pub size: u64,
    pub digest: Digest,
}

impl ContentIdentity {
    /// **Q1's whole test.** A hybrid or a truncation fails it, which is the
    /// sentence that closes Row 170's central hazard.
    pub fn is_the_same_file_as(&self, other: &ContentIdentity) -> bool {
        self == other
    }

    /// For the desk and for tests: the identity of a whole slice.
    pub fn of(bytes: &[u8]) -> ContentIdentity {
        ContentIdentity {
            size: bytes.len() as u64,
            digest: Digest::of(bytes),
        }
    }
}

/// **Q4: the cost is processor rather than disk, and no rate is promised
/// anywhere.** Every promise is written as *reading both files through, at
/// both ends, on each machine rather than between them*, and none names a
/// duration (§12 item 6).
pub const NO_RATE_IS_PROMISED_ANYWHERE: bool = true;

/// Which of Q5's two routes established the row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ByWhichRoute {
    /// Route 1: the record fast path — the four values matched what was
    /// recorded at publication, so the recorded digest stood for the
    /// contents with no read.
    TheRecord,
    /// Route 2: the machine copy read through by the digest worker. **A
    /// confirmation is recorded** (Q7) where this route establishes it.
    TheReadThrough,
}

/// **Q5's outcome for one file** — established by record or by reading,
/// never by presence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Establishment {
    /// The contents at the resolved machine location are established equal
    /// to the local file's.
    AlreadyOnTheMachine(ByWhichRoute),
    /// Not already there. **Where the location is occupied by a different
    /// file, Q8's overwrite warning fires before anything is sent.**
    NotAlreadyThere { occupied_by_a_different_file: bool },
    /// The read-through did not complete — *"a digest that did not complete
    /// is nothing"* (Section 9) — so nothing is established either way and
    /// the row is not confirmed.
    NotEstablished,
    /// The local candidate could not be read through, so there is nothing to
    /// compare against.
    TheLocalFileCouldNotBeRead,
}

/// The record's answer for one machine location, from a placement record
/// or a published transfer record (`Server protocol and transfers` S9,
/// which `File Browser`'s hand-uploads carry).
fn the_records_entry_for(
    record: &dyn TheRecord,
    machine: &MachineKey,
    at: &MachineLocation,
    current_installation: &Identity,
) -> Option<(
    ContentIdentity,
    smoozy_server_protocol::transfers::SourceReading,
)> {
    if let Some(placement) = record.placements_on(machine).into_iter().find(|placement| {
        &placement.mirrored_location == at && placement.stands_under(current_installation)
    }) {
        return Some((placement.content, placement.four_values_at_publication));
    }
    record
        .published_transfers_on(machine)
        .into_iter()
        .find(|published| {
            &published.at == at && published.machine.is_current_under(current_installation)
        })
        .map(|published| (published.content, published.four_values_at_publication))
}

/// **Q5: a row is *already on the machine* when the contents of the file at
/// the resolved machine location are established equal to the contents of the
/// local file the row resolved to — established by record or by reading,
/// never by presence.** Two routes, tried in order.
///
/// **Route 1, the record fast path.** A placement record (K7) or a published
/// transfer record names that machine location with a content identity under
/// the current installation identity. The machine copy's four values are
/// read and compared against the four recorded at publication. Where all four
/// match, the copy is bit-for-bit what was published (Fact 21: a
/// save-by-rename changes the inode; a hand-restored modification time still
/// moves the change time, which cannot be set back), and the recorded digest
/// stands for its contents with no read.
///
/// **Route 2, the read-through.** Where no record names the location, or the
/// four values moved, or the channel cannot answer, the machine copy is read
/// through by the digest worker and compared against the local candidate's
/// digest.
pub fn already_on_the_machine(
    local: &StoredLocation,
    machine: &MachineKey,
    at: &MachineLocation,
    current_installation: &Identity,
    record: &dyn TheRecord,
    readings: &dyn Readings,
) -> Establishment {
    let Some(local_contents) = readings.the_local_files_content_identity(local) else {
        return Establishment::TheLocalFileCouldNotBeRead;
    };

    // Route 1.
    if let Some((recorded_contents, recorded_four)) =
        the_records_entry_for(record, machine, at, current_installation)
    {
        match readings.the_four_values_now(machine, at) {
            FourValuesNow::Answered(now) if now.is_the_same_file_as(&recorded_four) => {
                return if recorded_contents.is_the_same_file_as(&local_contents) {
                    Establishment::AlreadyOnTheMachine(ByWhichRoute::TheRecord)
                } else {
                    Establishment::NotAlreadyThere {
                        occupied_by_a_different_file: true,
                    }
                };
            }
            // Values moved, nothing there, or the channel cannot answer: the
            // fast path does not fire and the ordinary route runs.
            FourValuesNow::Answered(_)
            | FourValuesNow::NothingThere
            | FourValuesNow::TheChannelCannotAnswer => {}
        }
    }

    // Route 2.
    match readings.the_machine_copys_contents(machine, at) {
        MachineContents::Read(there) => {
            if there.is_the_same_file_as(&local_contents) {
                Establishment::AlreadyOnTheMachine(ByWhichRoute::TheReadThrough)
            } else {
                Establishment::NotAlreadyThere {
                    occupied_by_a_different_file: true,
                }
            }
        }
        MachineContents::NothingThere => Establishment::NotAlreadyThere {
            occupied_by_a_different_file: false,
        },
        MachineContents::DidNotComplete => Establishment::NotEstablished,
    }
}

/// **Q5's set-row membership test, which can only withhold confirmation and
/// never grant it.** A set row is *already on the machine* only where the
/// mirrored folder's membership — enumerated on the machine (a listing on the
/// transfer surface read with `Job files and outputs` N15's discipline) and
/// matched by the same pattern rule G22 applies to the user's own folder —
/// equals the local set member for member, *and* each member's contents are
/// established by the two routes above.
///
/// **Nothing ever removes a member from the machine, so a set's machine-side
/// membership only grows** — and a surplus member drawn into a render is real
/// content the user's project does not hold, which is worse than a member
/// missing. Where the membership differs the row is simply not confirmed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Membership {
    Equal,
    Differs {
        surplus_on_the_machine: Vec<Vec<u8>>,
        missing_from_the_machine: Vec<Vec<u8>>,
    },
}

pub fn set_row_membership(
    pattern_name: &[u8],
    local_members: &[Vec<u8>],
    mirrored: &MirroredFolderListing,
) -> Membership {
    let local: Vec<&Vec<u8>> = local_members
        .iter()
        .filter(|member| set_member_matches(pattern_name, member))
        .collect();
    let remote: Vec<&Vec<u8>> = mirrored
        .members
        .iter()
        .filter(|member| set_member_matches(pattern_name, member))
        .collect();
    let surplus: Vec<Vec<u8>> = remote
        .iter()
        .filter(|member| !local.contains(member))
        .map(|member| (*member).clone())
        .collect();
    let missing: Vec<Vec<u8>> = local
        .iter()
        .filter(|member| !remote.contains(member))
        .map(|member| (*member).clone())
        .collect();
    if surplus.is_empty() && missing.is_empty() {
        Membership::Equal
    } else {
        Membership::Differs {
            surplus_on_the_machine: surplus,
            missing_from_the_machine: missing,
        }
    }
}

/// **Q7: confirmation of a *sent before, not confirmed* row is Q5 run against
/// the record's own entry.** The state exists where the record says a file was
/// sent for this reference to this machine and the local original is no
/// longer there to compare. Confirmation: the four-value fast path against
/// the placement record; where it holds, *already on the machine*; where the
/// file is absent, back to *not found* and the placement record is marked
/// gone; where the file is present but the values moved, the read-through —
/// against the **recorded** content identity, since there is no local
/// original.
pub fn confirm(
    placement: &PlacementRecord,
    readings: &dyn Readings,
) -> crate::record::ConfirmationOutcome {
    use crate::record::ConfirmationOutcome::*;
    match readings.the_four_values_now(&placement.machine, &placement.mirrored_location) {
        FourValuesNow::Answered(now)
            if now.is_the_same_file_as(&placement.four_values_at_publication) =>
        {
            return Confirmed;
        }
        FourValuesNow::NothingThere => return Gone,
        FourValuesNow::Answered(_) | FourValuesNow::TheChannelCannotAnswer => {}
    }
    match readings.the_machine_copys_contents(&placement.machine, &placement.mirrored_location) {
        MachineContents::Read(there) if there.is_the_same_file_as(&placement.content) => Confirmed,
        MachineContents::NothingThere => Gone,
        MachineContents::Read(_) | MachineContents::DidNotComplete => NotConfirmed,
    }
}

/// **Q8: the choice offered is replace or stop and nothing wider** — the spec
/// closes *keep both* and *skip existing* by name, because the landing rule
/// of 5.2 leaves no second place for the file to go and a skip would leave
/// the reference pointing at a file that is not the one it means.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverwriteChoice {
    Replace,
    Stop,
}

/// **Q8: the overwrite warning is Q5's route 2 finding a different file at
/// the landing place — and its scope is this module's own row sends, reaching
/// no plain `File Browser` upload.** It fires inside the send pre-flight,
/// where Q5's establishment is already being made, so its contents test is
/// the send decision's own reading and never a read added at warning time.
/// What it names is read off K7, K8 and 7.1's points-at reading; its wording
/// and shape are `File Browser`'s.
pub fn the_overwrite_warning_fires(establishment: &Establishment) -> bool {
    matches!(
        establishment,
        Establishment::NotAlreadyThere {
            occupied_by_a_different_file: true
        }
    )
}

/// **Q8's scope as a value**: this module's own row sends, and no plain
/// upload.
pub const THE_OVERWRITE_WARNING_REACHES_NO_PLAIN_UPLOAD: bool = true;

/// **Q9: the second-reference question — *have I seen these contents anywhere
/// on this machine* — is a lookup by content identity across the machine's
/// placement and published-transfer records.** A hit means the reference is
/// pointed at the copy already there rather than uploaded again. A file the
/// user placed twice by routes Smoozy did not perform has no record and the
/// lookup misses it; the cost is one redundant mirror copy, never wrongness.
pub fn seen_anywhere_on_this_machine(
    contents: &ContentIdentity,
    machine: &MachineKey,
    current_installation: &Identity,
    record: &dyn TheRecord,
) -> Option<MachineLocation> {
    if let Some(placement) = record.placements_on(machine).into_iter().find(|placement| {
        placement.stands_under(current_installation)
            && placement.content.is_the_same_file_as(contents)
    }) {
        return Some(placement.mirrored_location);
    }
    record
        .published_transfers_on(machine)
        .into_iter()
        .find(|published: &PublishedTransfer| {
            published.machine.is_current_under(current_installation)
                && published.content.is_the_same_file_as(contents)
        })
        .map(|published| published.at)
}

/// **Q9's bound: a hit on a linked `.blend` saves the upload and nothing
/// else.** A `.blend` is remapped in place and what a render reads is
/// Smoozy's fixed copy of it (6.5), so the reference is never pointed at the
/// copy already there — that copy is what the fixed copy is produced from.
/// *Already on the machine* means the file does not need sending, never that
/// it does not need fixing.
pub const A_HIT_ON_A_LINKED_BLEND_SAVES_THE_UPLOAD_AND_NOTHING_ELSE: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::readings::{EnumerationCasing, FolderListing, StoredNameAnswer};
    use crate::record::tests::{a_machine, a_publication, contents, four_values};
    use crate::record::{ConfirmationOutcome, InMemoryRecord, PlacementRecord};
    use std::collections::HashMap;

    /// A bench whose every reading is a value written by hand.
    #[derive(Default)]
    pub struct Bench {
        pub local: HashMap<Vec<u8>, ContentIdentity>,
        pub four_now: HashMap<Vec<u8>, FourValuesNow>,
        pub machine_contents: HashMap<Vec<u8>, MachineContents>,
    }

    impl Readings for Bench {
        fn the_local_files_content_identity(
            &self,
            file: &StoredLocation,
        ) -> Option<ContentIdentity> {
            self.local.get(file.as_bytes()).cloned()
        }
        fn the_four_values_now(
            &self,
            _machine: &MachineKey,
            at: &MachineLocation,
        ) -> FourValuesNow {
            self.four_now
                .get(at.as_bytes())
                .cloned()
                .unwrap_or(FourValuesNow::TheChannelCannotAnswer)
        }
        fn the_machine_copys_contents(
            &self,
            _machine: &MachineKey,
            at: &MachineLocation,
        ) -> MachineContents {
            self.machine_contents
                .get(at.as_bytes())
                .cloned()
                .unwrap_or(MachineContents::NothingThere)
        }
        fn enumerate_folder(&self, _folder: &[u8]) -> FolderListing {
            FolderListing::CouldNotBeRead
        }
        fn enumeration_casing_of(&self, _folder: &[u8]) -> EnumerationCasing {
            EnumerationCasing::IsTheStoredName
        }
        fn the_stored_name_reading(&self, _path: &[u8]) -> StoredNameAnswer {
            StoredNameAnswer::NotThere
        }
        fn the_sources_prefix_digest(
            &self,
            _source: &StoredLocation,
            _up_to: u64,
        ) -> Option<Digest> {
            None
        }
    }

    fn local() -> StoredLocation {
        StoredLocation::as_the_system_returned_it(b"/Users/a/x.png")
    }
    fn mirror() -> MachineLocation {
        MachineLocation::composed(b"/root/assets/files/Users/a/x.png".to_vec())
    }

    #[test]
    fn a_partial_never_satisfies_the_content_identity() {
        let whole = ContentIdentity::of(b"the whole file");
        let partial = ContentIdentity::of(b"the whole");
        assert!(!whole.is_the_same_file_as(&partial));
        // And a same-size hybrid with different bytes fails on the digest.
        let hybrid = ContentIdentity::of(b"the whole fil3");
        assert_eq!(hybrid.size, whole.size);
        assert!(!whole.is_the_same_file_as(&hybrid));
    }

    #[test]
    fn route_one_fires_where_all_four_values_match_and_the_recorded_digest_equals_the_local() {
        let machine = a_machine();
        let mut record = InMemoryRecord::default();
        record.placements.push(PlacementRecord::at_publication(
            machine.clone(),
            a_publication(mirror().as_bytes(), local().as_bytes(), b"red"),
        ));
        let mut bench = Bench::default();
        bench
            .local
            .insert(local().as_bytes().to_vec(), contents(b"red"));
        bench.four_now.insert(
            mirror().as_bytes().to_vec(),
            FourValuesNow::Answered(four_values(3, "77", 100, 100)),
        );
        // No read-through is available at all, which proves route 1 answered.
        assert_eq!(
            already_on_the_machine(
                &local(),
                &machine,
                &mirror(),
                &machine.installation,
                &record,
                &bench
            ),
            Establishment::AlreadyOnTheMachine(ByWhichRoute::TheRecord)
        );
    }

    #[test]
    fn a_moved_change_time_alone_takes_the_fast_path_down_and_the_read_through_decides() {
        let machine = a_machine();
        let mut record = InMemoryRecord::default();
        record.placements.push(PlacementRecord::at_publication(
            machine.clone(),
            a_publication(mirror().as_bytes(), local().as_bytes(), b"red"),
        ));
        let mut bench = Bench::default();
        bench
            .local
            .insert(local().as_bytes().to_vec(), contents(b"red"));
        // A hand-restored modification time still moves the change time.
        bench.four_now.insert(
            mirror().as_bytes().to_vec(),
            FourValuesNow::Answered(four_values(3, "77", 100, 101)),
        );
        bench.machine_contents.insert(
            mirror().as_bytes().to_vec(),
            MachineContents::Read(contents(b"red")),
        );
        assert_eq!(
            already_on_the_machine(
                &local(),
                &machine,
                &mirror(),
                &machine.installation,
                &record,
                &bench
            ),
            Establishment::AlreadyOnTheMachine(ByWhichRoute::TheReadThrough)
        );
        // And with different contents there, the row is not already there and
        // the location is occupied — Q8's case.
        bench.machine_contents.insert(
            mirror().as_bytes().to_vec(),
            MachineContents::Read(contents(b"blue")),
        );
        let establishment = already_on_the_machine(
            &local(),
            &machine,
            &mirror(),
            &machine.installation,
            &record,
            &bench,
        );
        assert_eq!(
            establishment,
            Establishment::NotAlreadyThere {
                occupied_by_a_different_file: true
            }
        );
        assert!(the_overwrite_warning_fires(&establishment));
    }

    #[test]
    fn a_file_at_a_location_no_record_names_and_not_read_through_is_never_already_there() {
        let machine = a_machine();
        let record = InMemoryRecord::default();
        let mut bench = Bench::default();
        bench
            .local
            .insert(local().as_bytes().to_vec(), contents(b"red"));
        // The channel answers four values for a file that is there — and
        // that presence is worth nothing without a record or a read.
        bench.four_now.insert(
            mirror().as_bytes().to_vec(),
            FourValuesNow::Answered(four_values(3, "77", 100, 100)),
        );
        bench.machine_contents.insert(
            mirror().as_bytes().to_vec(),
            MachineContents::DidNotComplete,
        );
        assert_eq!(
            already_on_the_machine(
                &local(),
                &machine,
                &mirror(),
                &machine.installation,
                &record,
                &bench
            ),
            Establishment::NotEstablished
        );
    }

    #[test]
    fn a_record_under_a_superseded_installation_answers_nothing() {
        let machine = a_machine();
        let mut record = InMemoryRecord::default();
        record.placements.push(PlacementRecord::at_publication(
            machine.clone(),
            a_publication(mirror().as_bytes(), local().as_bytes(), b"red"),
        ));
        let mut bench = Bench::default();
        bench
            .local
            .insert(local().as_bytes().to_vec(), contents(b"red"));
        bench.four_now.insert(
            mirror().as_bytes().to_vec(),
            FourValuesNow::Answered(four_values(3, "77", 100, 100)),
        );
        let recycled = Identity::issue(IdentityKind::Installation).expect("randomness");
        assert_eq!(
            already_on_the_machine(&local(), &machine, &mirror(), &recycled, &record, &bench),
            Establishment::NotAlreadyThere {
                occupied_by_a_different_file: false
            }
        );
    }

    use smoozy_state::identities::IdentityKind;

    #[test]
    fn nothing_there_is_not_already_there_and_fires_no_warning() {
        let machine = a_machine();
        let record = InMemoryRecord::default();
        let mut bench = Bench::default();
        bench
            .local
            .insert(local().as_bytes().to_vec(), contents(b"red"));
        let establishment = already_on_the_machine(
            &local(),
            &machine,
            &mirror(),
            &machine.installation,
            &record,
            &bench,
        );
        assert_eq!(
            establishment,
            Establishment::NotAlreadyThere {
                occupied_by_a_different_file: false
            }
        );
        assert!(!the_overwrite_warning_fires(&establishment));
    }

    #[test]
    fn a_set_rows_membership_can_only_withhold() {
        let local = vec![b"tile.1001.png".to_vec(), b"tile.1002.png".to_vec()];
        let equal = MirroredFolderListing {
            members: vec![b"tile.1002.png".to_vec(), b"tile.1001.png".to_vec()],
        };
        assert_eq!(
            set_row_membership(b"tile.<UDIM>.png", &local, &equal),
            Membership::Equal
        );
        let surplus = MirroredFolderListing {
            members: vec![
                b"tile.1001.png".to_vec(),
                b"tile.1002.png".to_vec(),
                b"tile.1003.png".to_vec(),
            ],
        };
        assert_eq!(
            set_row_membership(b"tile.<UDIM>.png", &local, &surplus),
            Membership::Differs {
                surplus_on_the_machine: vec![b"tile.1003.png".to_vec()],
                missing_from_the_machine: vec![],
            }
        );
        // A file in the folder that the pattern does not match is not a
        // member on either side.
        let with_a_stranger = MirroredFolderListing {
            members: vec![
                b"tile.1001.png".to_vec(),
                b"tile.1002.png".to_vec(),
                b"notes.txt".to_vec(),
            ],
        };
        assert_eq!(
            set_row_membership(b"tile.<UDIM>.png", &local, &with_a_stranger),
            Membership::Equal
        );
    }

    #[test]
    fn confirmation_runs_against_the_records_own_entry() {
        let machine = a_machine();
        let placement = PlacementRecord::at_publication(
            machine.clone(),
            a_publication(mirror().as_bytes(), local().as_bytes(), b"red"),
        );
        let mut bench = Bench::default();
        bench.four_now.insert(
            mirror().as_bytes().to_vec(),
            FourValuesNow::Answered(four_values(3, "77", 100, 100)),
        );
        assert_eq!(confirm(&placement, &bench), ConfirmationOutcome::Confirmed);

        bench
            .four_now
            .insert(mirror().as_bytes().to_vec(), FourValuesNow::NothingThere);
        assert_eq!(confirm(&placement, &bench), ConfirmationOutcome::Gone);

        // Values moved, and the read-through against the *recorded* identity
        // decides — there is no local original.
        bench.four_now.insert(
            mirror().as_bytes().to_vec(),
            FourValuesNow::Answered(four_values(3, "78", 100, 100)),
        );
        bench.machine_contents.insert(
            mirror().as_bytes().to_vec(),
            MachineContents::Read(contents(b"red")),
        );
        assert_eq!(confirm(&placement, &bench), ConfirmationOutcome::Confirmed);
        bench.machine_contents.insert(
            mirror().as_bytes().to_vec(),
            MachineContents::Read(contents(b"blue")),
        );
        assert_eq!(
            confirm(&placement, &bench),
            ConfirmationOutcome::NotConfirmed
        );
    }

    #[test]
    fn the_second_reference_lookup_is_by_content_identity_and_misses_what_smoozy_never_placed() {
        let machine = a_machine();
        let mut record = InMemoryRecord::default();
        record.placements.push(PlacementRecord::at_publication(
            machine.clone(),
            a_publication(mirror().as_bytes(), local().as_bytes(), b"red"),
        ));
        assert_eq!(
            seen_anywhere_on_this_machine(
                &contents(b"red"),
                &machine,
                &machine.installation,
                &record
            ),
            Some(mirror())
        );
        assert_eq!(
            seen_anywhere_on_this_machine(
                &contents(b"blue"),
                &machine,
                &machine.installation,
                &record
            ),
            None
        );
        // A hand-upload's published transfer record counts too.
        record.published_transfers.push(PublishedTransfer {
            machine: machine.clone(),
            at: MachineLocation::composed(b"/root/jobs/hand/blue.png".to_vec()),
            content: contents(b"blue"),
            four_values_at_publication: four_values(4, "9", 1, 1),
        });
        assert_eq!(
            seen_anywhere_on_this_machine(
                &contents(b"blue"),
                &machine,
                &machine.installation,
                &record
            )
            .map(|at| at.as_bytes().to_vec()),
            Some(b"/root/jobs/hand/blue.png".to_vec())
        );
    }
}
