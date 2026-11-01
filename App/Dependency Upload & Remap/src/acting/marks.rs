//! **§7.3 performed — V13 and V13a.** The two marks a publication of this
//! module's own writes into the check standings, at the publication and by the
//! module that published.
//!
//! **V13's actor was stated on 2026-08-22 and this is it.** *"The mark is
//! written by the module that published, at its publication: `File Browser`
//! X7a for its own publications — an upload's, a copy's and a same-filesystem
//! move's — and **this module's own publication handling for a row send**
//! landing on such a path."* The trigger stood with no actor until 2026-08-22,
//! no document performed the write, and the submission warning it feeds could
//! never fire.
//!
//! **The actor was widened on 2026-09-05 from *any Smoozy upload* to any
//! Smoozy publication**, and this file was already keeping the wider clause:
//! [`what_this_publication_marks`] asks only where a publication landed and
//! never what kind of act it was, so a copy's publication and a
//! same-filesystem move's mark exactly as an upload's do. **What was narrower
//! than the clause was the wording here and nowhere else**, and the quotations
//! were brought into line the same day.
//!
//! **V13a: the disturbance mark is written by the act, not by the warning in
//! front of it.** Any Smoozy act that replaced, refixed or deleted anything a
//! root's tree resolved marks that root's standing disturbed, **whether or not
//! a warning stood before it** — because M16b's publication carries none.
//!
//! **V15 binds every line below: presence, size and the machine's clock decide
//! nothing.** A killed transfer leaves a plausible file at the final name with
//! no marker, so **the trigger is the publication act and never a comparison of
//! what sits on the machine**, and no trigger here compares two clocks.
//!
//! **V1: this reads the local record alone and reaches no machine.** Every
//! answer below is composed at the moment a publication lands, from records
//! Smoozy wrote.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `File Browser` X7a.

use crate::bytes::MachineLocation;
use crate::record::{MachineKey, TheRecord};
use crate::standing::{roots_whose_tree_resolved, WhereFilesSaysItCameFrom};

/// **What one publication of this module's own owes the standings.**
///
/// **Two marks and no third**, and they are different questions about one act:
/// *did this land on a root's own machine copy* and *did this touch anything a
/// root's tree resolved*. A publication can owe both, either or neither.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WhatThePublicationMarks {
    /// **V13's first trigger.** The roots whose own machine copy this
    /// publication landed on — *"a check describes the file it read and what
    /// has just been published is a different file"*. **Any Smoozy
    /// publication onto that path**, whatever the act was.
    pub replaced_since_check: Vec<MachineLocation>,
    /// **V13's third trigger, as V13a widens it.** The roots whose tree
    /// resolved the location this publication landed on.
    pub disturbed: Vec<MachineLocation>,
}

impl WhatThePublicationMarks {
    pub fn nothing() -> WhatThePublicationMarks {
        WhatThePublicationMarks::default()
    }

    pub fn is_nothing(&self) -> bool {
        self.replaced_since_check.is_empty() && self.disturbed.is_empty()
    }
}

/// **V13 and V13a, at a publication.**
///
/// **Two arguments are not this module's record**: the location the
/// publication landed at, which is the act's own and not a reading, and
/// `File Browser` N6's answer for a file on the machine, which is that
/// document's records and which the walk takes first
/// ([`crate::standing::the_holding_file_the_walk_takes`]). **The caller hands
/// `File Browser` N6's answer in**; without it the walk cannot find a root uploaded through
/// Files, and a *Replace* over something that root's tree resolved would mark
/// it nothing. Everything else is walked through the record.
///
/// **A root is never marked twice for one act**, and a root that is both
/// replaced and disturbed carries both marks — they are two states of one
/// standing and neither stands in for the other (V14).
pub fn what_this_publication_marks(
    landed_at: &MachineLocation,
    machine: &MachineKey,
    layout: &smoozy_job_files::layout::TheLayout,
    record: &dyn TheRecord,
    the_roots_with_a_standing: &[MachineLocation],
    came_up_through_files: WhereFilesSaysItCameFrom<'_>,
) -> WhatThePublicationMarks {
    // **V13's first trigger is an equality between two machine locations and
    // nothing else** — the root's own machine path against where the
    // publication landed. No listing, no size, no clock, **and no question
    // about which kind of act published**: an upload's publication, a copy's
    // and a same-filesystem move's all reach this the same way, which is the
    // clause as it was widened on 2026-09-05.
    let replaced_since_check: Vec<MachineLocation> = the_roots_with_a_standing
        .iter()
        .filter(|root| *root == landed_at)
        .cloned()
        .collect();

    // **V13a's widened third trigger**, walked through the record by
    // [`roots_whose_tree_resolved`], which reaches no machine.
    let disturbed = roots_whose_tree_resolved(
        landed_at,
        machine,
        layout,
        record,
        the_roots_with_a_standing,
        came_up_through_files,
    );

    WhatThePublicationMarks {
        replaced_since_check,
        disturbed,
    }
}

/// **V13a's own sentence as a value**: the qualifier is gone, and any act that
/// replaced, refixed or deleted anything a root's tree resolved marks that
/// root's standing disturbed **whether or not a warning stood before it**.
///
/// M16b's publication is exactly such an act and carries no warning, and V3's
/// plain-upload trigger — *placed or fixed* — does not reach it.
pub const THE_MARK_IS_WRITTEN_BY_THE_ACT_AND_NOT_BY_A_WARNING: bool = true;

/// **V13's second trigger is not this module's to write and is not here.** A
/// version change — *"a submission naming a Blender other than the standing's
/// is not covered by it"* — is read at submission from the standing's own
/// recorded version against the version being submitted, which is
/// [`crate::standing::CheckStanding`]'s field and a comparison rather than an
/// act. **Nothing publishes for it, so nothing marks for it.**
pub const THE_VERSION_TRIGGER_IS_A_COMPARISON_AND_NOT_AN_ACT: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bytes::{RecordedLocation, StoredLocation};
    use crate::record::tests::{a_machine, a_publication};
    use crate::record::{
        HoldingFile, InMemoryRecord, LocatedAnswer, LocatedAnswerKey, PlacementRecord, Resolution,
        ResolvedTo,
    };
    use crate::standing::nothing_came_up_through_files;
    use smoozy_job_files::layout::{MachineRoot, TheLayout, WhereThePackageAreaSits};

    fn a_layout() -> TheLayout {
        TheLayout::of(
            MachineRoot::chosen("/ephemeral/smoozy").expect("a root"),
            WhereThePackageAreaSits::UnderTheRoot,
        )
    }

    fn at(path: &[u8]) -> MachineLocation {
        MachineLocation::composed(path.to_vec())
    }

    /// **V13's first trigger: a publication onto a root's own machine copy
    /// marks that standing replaced-since-check, and touches no other root.**
    #[test]
    fn a_publication_onto_a_roots_own_copy_marks_that_standing_and_no_other() {
        let machine = a_machine();
        let layout = a_layout();
        let record = InMemoryRecord::default();
        let root = at(b"/ephemeral/smoozy/assets/files/Users/a/scene.blend");
        let another = at(b"/ephemeral/smoozy/assets/files/Users/a/other.blend");

        let marks = what_this_publication_marks(
            &root,
            &machine,
            &layout,
            &record,
            &[root.clone(), another.clone()],
            &nothing_came_up_through_files,
        );
        assert_eq!(marks.replaced_since_check, vec![root]);
        assert!(marks.disturbed.is_empty());
    }

    /// **V13a: a publication onto something a root's tree resolved marks that
    /// root disturbed, with no warning in front of it.**
    ///
    /// This is M16b's own shape: a `.blend` on the machine by another route is
    /// published over, V3's *placed or fixed* trigger does not reach it, and
    /// the standing must still stop saying the file is ready to render.
    #[test]
    fn a_publication_onto_something_the_tree_resolved_marks_the_root_disturbed() {
        let machine = a_machine();
        let layout = a_layout();
        let source = b"/Users/a/scene.blend";
        let root = at(b"/ephemeral/smoozy/assets/files/Users/a/scene.blend");
        let texture_source = b"/Users/a/tex/METallic.png";
        let texture = at(b"/ephemeral/smoozy/assets/files/Users/a/tex/METallic.png");

        let mut record = InMemoryRecord::default();
        // The placement that says which user file the root's machine copy is.
        record.placements.push(PlacementRecord::at_publication(
            machine.clone(),
            a_publication(root.as_bytes(), source, b"a scene"),
        ));
        // One located answer of that holding file, resolving to the texture.
        record.located_answers.push(LocatedAnswer {
            key: LocatedAnswerKey {
                holding_file: HoldingFile::OnTheUsersComputer(
                    StoredLocation::as_the_system_returned_it(source),
                ),
                reference: RecordedLocation::as_the_reading_carried_it(b"//tex/METallic.png"),
            },
            resolution: Some(Resolution {
                to: ResolvedTo::AWholeLocation(StoredLocation::as_the_system_returned_it(
                    texture_source,
                )),
                how: crate::record::HowResolved::TheRecordedLocationItself,
            }),
            set_aside: false,
            rejected_guess: None,
            read_as: Vec::new(),
        });

        let marks = what_this_publication_marks(
            &texture,
            &machine,
            &layout,
            &record,
            &[root.clone()],
            &nothing_came_up_through_files,
        );
        assert!(
            marks.replaced_since_check.is_empty(),
            "the publication did not land on the root's own copy"
        );
        assert_eq!(marks.disturbed, vec![root]);
        const { assert!(THE_MARK_IS_WRITTEN_BY_THE_ACT_AND_NOT_BY_A_WARNING) };
    }

    /// **A publication that touched nothing marks nothing**, and that is an
    /// answer rather than an omission — no standing is disturbed on the
    /// strength of an act reaching somewhere else.
    #[test]
    fn a_publication_that_touched_nothing_marks_nothing() {
        let machine = a_machine();
        let layout = a_layout();
        let record = InMemoryRecord::default();
        let root = at(b"/ephemeral/smoozy/assets/files/Users/a/scene.blend");
        let elsewhere = at(b"/ephemeral/smoozy/assets/files/Users/b/unrelated.png");
        let marks = what_this_publication_marks(
            &elsewhere,
            &machine,
            &layout,
            &record,
            &[root],
            &nothing_came_up_through_files,
        );
        assert!(marks.is_nothing());
    }
}
