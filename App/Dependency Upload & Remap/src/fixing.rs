//! The two copies of a linked `.blend` — `Dependency Upload & Remap` §6.5,
//! Row 175: M13–M18, and the three replacement triggers. **Producing a fixed
//! copy is the acting half's** (M14 drives a Blender); **whether one is owed
//! is a comparison of records and is here.**
//!
//! **M13: every linked `.blend` the check resolves gets a fixed copy —
//! whichever route put it on the machine — and the render is made to read the
//! fixed copy; the copy the user sent up survives untouched where it lies.**
//! The top file's library pair points at the fixed copy's address; a fixed
//! copy's own library references point at other fixed copies; every other
//! reference in a fixed copy points at mirrors.
//!
//! **M15: uniformity is deliberate — every linked `.blend` gets a fixed copy
//! even where its references all resolved to *nothing to change*.** One rule,
//! no conditional targets.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` S9. `File
//! Browser` W1a.

use smoozy_state::identities::Identity;

use crate::bytes::{MachineLocation, RecordedLocation};
use crate::contents::ContentIdentity;
use crate::landing::CouldNotBeFixed;
use crate::record::{FixingRecord, MachineKey, TheRecord};

/// **M13 and M15 as values.**
pub const EVERY_LINKED_BLEND_GETS_A_FIXED_COPY_WHICHEVER_ROUTE_PUT_IT_THERE: bool = true;

/// **M14: a fixed copy is produced on the machine, by a worker driving the
/// job's Blender, from the machine copy of the linked `.blend`, and published
/// by one rename** — **open the machine file headless**, rewrite each
/// resolved reference by the per-setting route, **write the copy straight to
/// the staging path with the save-time relative-remapping turned off** (Fact
/// 77's first recorded trap), publish by rename into A3's address; the fixing
/// record (K8) written at publication. The act is the acting half's; the trap
/// is named here.
///
/// **There is no copy step.** M14's own repair note of 2026-09-04 struck it,
/// because a copy from one place on a rented machine to another is an act
/// this stage establishes no route to; Blender's own save takes the file to
/// write as an argument, which needs no copy and no program. This citation
/// read *"copy to a staging path, open headless"* until 2026-09-05 — the step
/// the plan struck on the day the code was written, and the step that could
/// not be built.
pub const THE_SAVE_TIME_RELATIVE_REMAPPING_IS_TURNED_OFF: bool = true;

/// **M16: the three replacement triggers.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplacementTrigger {
    /// **The first**: a changed linked `.blend` arrives through this module,
    /// is checked as any file is, its fixed copy is produced staged, and one
    /// rename replaces the old — the old copy readable to the last moment.
    AChangedLinkedBlendArrivedThroughThisModule,
    /// **The second: the file standing still while its own references move.**
    /// The located answers for its holding file differ from the (reference
    /// bytes → machine location) pairs the fixing record holds — a reference
    /// located by hand in a later sitting, one a shift resolved, a rejected
    /// guess reinstated. Without this the render would find Smoozy's versions
    /// for the references resolved early and the artist's computer for the
    /// ones resolved late, at success, silently.
    TheLocatedAnswersMoved,
    /// **M16b, the third: the machine copy a fixed copy was derived from is
    /// published over** — by hand through `File Browser`, or there with no
    /// known local origin — the file changed and the located answers did not.
    TheMachineCopyWasPublishedOver,
}

/// **M16's second trigger: the record's list is the comparison**, made
/// whenever a check runs over a file whose tree holds that `.blend`. The
/// pairs are compared as sets — order carries nothing.
pub fn the_located_answers_moved(
    fixing: &FixingRecord,
    current_pairs: &[(RecordedLocation, MachineLocation)],
) -> bool {
    let mut held: Vec<&(RecordedLocation, MachineLocation)> = fixing.points_at.iter().collect();
    let mut now: Vec<&(RecordedLocation, MachineLocation)> = current_pairs.iter().collect();
    held.sort();
    now.sort();
    held != now
}

/// **M16b's trigger: a fixed copy is owed a re-production whenever the
/// content identity last published at the machine location K8 records it as
/// derived from differs from the content identity K8 records for that copy.**
/// Both values are already in the local record — K8's own, and the publishing
/// act's, which is a placement record's (K7) or a published transfer
/// record's (`Server protocol and transfers` S9) — so the comparison is a
/// record against a record. **Nothing reads the machine and nothing is
/// inferred from a file's presence, name or size (V15, Q10).**
pub fn the_machine_copy_was_published_over(
    fixing: &FixingRecord,
    last_published_at_derived_from: Option<&ContentIdentity>,
) -> bool {
    match last_published_at_derived_from {
        Some(published) => !published.is_the_same_file_as(&fixing.derived_from_content),
        None => false,
    }
}

/// **The content identity last published at a machine location, from the
/// record alone** — a placement under the current installation, else a
/// published transfer.
pub fn last_published_at(
    record: &dyn TheRecord,
    machine: &MachineKey,
    at: &MachineLocation,
    current_installation: &Identity,
) -> Option<ContentIdentity> {
    record
        .placements_on(machine)
        .into_iter()
        .find(|placement| {
            &placement.mirrored_location == at
                && placement.machine.is_current_under(current_installation)
        })
        .map(|placement| placement.content)
        .or_else(|| {
            record
                .published_transfers_on(machine)
                .into_iter()
                .find(|published| {
                    &published.at == at && published.machine.is_current_under(current_installation)
                })
                .map(|published| published.content)
        })
}

/// **Whether a fixed copy's re-production is owed, and by which trigger**
/// — the second and third are record comparisons and are asked here; the
/// first is an arrival the acting half performs, which replaces the copy as
/// part of the arrival itself.
pub fn re_production_is_owed(
    fixing: &FixingRecord,
    current_pairs: &[(RecordedLocation, MachineLocation)],
    last_published_at_derived_from: Option<&ContentIdentity>,
) -> Option<ReplacementTrigger> {
    if the_machine_copy_was_published_over(fixing, last_published_at_derived_from) {
        return Some(ReplacementTrigger::TheMachineCopyWasPublishedOver);
    }
    if the_located_answers_moved(fixing, current_pairs) {
        return Some(ReplacementTrigger::TheLocatedAnswersMoved);
    }
    None
}

/// **M16a: a *could not be fixed* row is tried again on its own, and the
/// retry is M14's production run once more for that one file — no check
/// re-run, nothing re-sent.** The retry needs nothing the sitting holds; it
/// is safe because the production is atomic and the old copy stands; it is
/// one operation and holds up nothing. **A retry that fails again leaves the
/// row as it was, and the reason it names does not have to change** — two
/// reasons are stable by construction and a retry reproduces them exactly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyItCouldNotBeFixed {
    /// A3's address will not fit A5's two byte checks — the same arithmetic
    /// over the same bytes.
    TheAddressWillNotFit(CouldNotBeFixed),
    /// The machine copy is no longer at the location the fixed copy is
    /// derived from, where the production has nothing to open. **It is the
    /// next check, not the retry, that re-establishes what is on the machine.**
    TheMachineCopyIsNoLongerThere,
    /// The production did not reach publication — nothing at the address, no
    /// fixing record.
    TheProductionDidNotPublish,
    /// **The fixing's own report says the copy is not fixed, so it is not
    /// published.**
    ///
    /// The script reports every pair, and this is that report read back and
    /// compared against the pairs the fixing was given. A pair that was not
    /// applied, a line that would not read back, or a pair the report never
    /// mentioned: any of them and the copy is not what the fixing was for.
    ///
    /// **Why *not applied* is fatal here, when M8a records two benign causes
    /// for it.** The code cannot tell a benign cause from a live one — both
    /// look like zero data-blocks matched. **Refusing to publish costs** a row
    /// reading *could not be fixed* on a library that may have been fine,
    /// which the user can see and the next check clears. **Publishing costs** a
    /// library that silently asks the rented machine for a file on the
    /// artist's own computer: a finished render at full price with the object
    /// in the substitute colour, and a check that told the user the file was
    /// ready. This module takes the loud side everywhere else — Fact 18's
    /// trap, the digest seam answering *did not complete*, G6b saying nothing
    /// rather than inferring — and it takes it here.
    ///
    /// **Stable under a retry**: the same pairs against the same machine copy
    /// reproduce it exactly, and it is the next check, not the retry, that
    /// changes the pairs — which is what
    /// [`WhyItCouldNotBeFixed::TheMachineCopyIsNoLongerThere`] already says in
    /// its own words.
    TheReportSaysThePairsDidNotAllApply {
        /// Pairs the report said were not applied.
        not_applied: usize,
        /// Pairs the report never mentioned at all.
        never_mentioned: usize,
        /// Lines carrying the marker's head whose body would not read back.
        would_not_read_back: usize,
    },
}

impl WhyItCouldNotBeFixed {
    /// **M16a: stable by construction — a retry reproduces it exactly.**
    pub fn is_stable_under_a_retry(self) -> bool {
        matches!(
            self,
            WhyItCouldNotBeFixed::TheAddressWillNotFit(_)
                | WhyItCouldNotBeFixed::TheMachineCopyIsNoLongerThere
                | WhyItCouldNotBeFixed::TheReportSaysThePairsDidNotAllApply { .. }
        )
    }
}

/// **M16a: the row's state comes from the fixing record and the publication,
/// never from an attempt**, so it can never be quietly turned into a success.
pub const THE_ROWS_STATE_COMES_FROM_THE_FIXING_RECORD_AND_THE_PUBLICATION: bool = true;

/// **M16b's refusal, recorded**: warning at the upload instead is refused —
/// it would put a warning in front of the user about a file this module did
/// not place, which no spec grants, and `File Browser`'s W1a has already
/// declined the class. What holds the gap shut until the next check is V13a's
/// disturbance mark.
pub const NO_WARNING_IS_ADDED_AT_THE_UPLOAD: bool = true;

/// **M17: a file that went up as a dependency and is later rendered as a root
/// renders the user's own copy, not the fixed copy** — its own remap lives in
/// the record like any root's, composed into that job's material after its
/// own check (V9).
pub const A_ROOT_RENDERS_THE_USERS_OWN_COPY: bool = true;

/// **M18: bringing a fixed copy down gives the user a file pointing at
/// machine locations, and that is the spec's own accepted cost** — their own
/// copy is still up there untouched. Nothing here prevents the download.
pub const NOTHING_PREVENTS_BRINGING_A_FIXED_COPY_DOWN: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::record::tests::{a_machine, a_publication, contents};
    use crate::record::{InMemoryRecord, PlacementRecord};

    fn recorded(bytes: &[u8]) -> RecordedLocation {
        RecordedLocation::as_the_reading_carried_it(bytes)
    }
    fn at(bytes: &[u8]) -> MachineLocation {
        MachineLocation::composed(bytes.to_vec())
    }

    fn a_fixing(machine: &MachineKey) -> FixingRecord {
        FixingRecord {
            machine: machine.clone(),
            derived_from: at(b"/root/assets/files/Users/a/lib/c.blend"),
            derived_from_content: contents(b"chairs v1"),
            fixed_copy: at(b"/root/assets/fixed/assets/files/Users/a/lib/c.blend"),
            points_at: vec![
                (
                    recorded(b"//tex/x.png"),
                    at(b"/root/assets/files/Users/a/tex/x.png"),
                ),
                (
                    recorded(b"//tex/y.png"),
                    at(b"/root/assets/files/Users/a/tex/y.png"),
                ),
            ],
        }
    }

    #[test]
    fn the_second_trigger_fires_where_the_answers_moved_and_order_carries_nothing() {
        let machine = a_machine();
        let fixing = a_fixing(&machine);
        let same_in_another_order = vec![fixing.points_at[1].clone(), fixing.points_at[0].clone()];
        assert!(!the_located_answers_moved(&fixing, &same_in_another_order));
        let moved = vec![
            fixing.points_at[0].clone(),
            (
                recorded(b"//tex/y.png"),
                at(b"/root/assets/files/Archive/tex/y.png"),
            ),
        ];
        assert!(the_located_answers_moved(&fixing, &moved));
        let one_added = vec![
            fixing.points_at[0].clone(),
            fixing.points_at[1].clone(),
            (
                recorded(b"//tex/z.png"),
                at(b"/root/assets/files/Users/a/tex/z.png"),
            ),
        ];
        assert!(the_located_answers_moved(&fixing, &one_added));
        assert_eq!(
            re_production_is_owed(&fixing, &moved, Some(&contents(b"chairs v1"))),
            Some(ReplacementTrigger::TheLocatedAnswersMoved)
        );
    }

    #[test]
    fn the_third_trigger_is_a_record_against_a_record() {
        let machine = a_machine();
        let fixing = a_fixing(&machine);
        let mut record = InMemoryRecord::default();
        record.placements.push(PlacementRecord::at_publication(
            machine.clone(),
            a_publication(
                fixing.derived_from.as_bytes(),
                b"/Users/a/lib/c.blend",
                b"chairs v1",
            ),
        ));
        let last = last_published_at(
            &record,
            &machine,
            &fixing.derived_from,
            &machine.installation,
        );
        assert_eq!(last, Some(contents(b"chairs v1")));
        assert!(!the_machine_copy_was_published_over(&fixing, last.as_ref()));

        // Published over by hand: the placement is replaced by a new
        // publication of different bytes.
        record.placements.clear();
        record.placements.push(PlacementRecord::at_publication(
            machine.clone(),
            a_publication(
                fixing.derived_from.as_bytes(),
                b"/Users/a/lib/c.blend",
                b"chairs v2",
            ),
        ));
        let last = last_published_at(
            &record,
            &machine,
            &fixing.derived_from,
            &machine.installation,
        );
        assert!(the_machine_copy_was_published_over(&fixing, last.as_ref()));
        assert_eq!(
            re_production_is_owed(&fixing, &fixing.points_at.clone(), last.as_ref()),
            Some(ReplacementTrigger::TheMachineCopyWasPublishedOver)
        );
        // With no publication on record there is nothing to compare, and
        // nothing is inferred.
        assert!(!the_machine_copy_was_published_over(&fixing, None));
        assert_eq!(
            re_production_is_owed(&fixing, &fixing.points_at.clone(), None),
            None
        );
    }

    #[test]
    fn two_of_the_three_reasons_are_stable_under_a_retry() {
        assert!(WhyItCouldNotBeFixed::TheMachineCopyIsNoLongerThere.is_stable_under_a_retry());
        assert!(WhyItCouldNotBeFixed::TheAddressWillNotFit(
            CouldNotBeFixed::TheMachineCopyIsNotUnderTheRoot
        )
        .is_stable_under_a_retry());
        assert!(!WhyItCouldNotBeFixed::TheProductionDidNotPublish.is_stable_under_a_retry());
    }
}
