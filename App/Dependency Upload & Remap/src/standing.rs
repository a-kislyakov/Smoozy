//! The check standing — `Dependency Upload & Remap` §7.3, Row 179: V10–V15.
//! What makes `Rendering`'s warning stop, and what makes it come back.
//!
//! **V15: presence, size and the machine's clock decide nothing in this
//! section.** A killed transfer leaves a plausible file at the final name
//! with no marker, so the replacement trigger is the publication act, never a
//! comparison of what sits on the machine; and no trigger compares two
//! clocks. **Kept by the types**: no function here takes a reading of the
//! machine, and every mark is written by an act.
//!
//! **This is the store `File Browser` X7a writes into** — the
//! replaced-since-check mark on a standing, at its publication (V13) — and
//! the other of X7a's two writes drops a `Render Settings` recorded reading,
//! which is that document's store and not this one's.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `File Browser` X7a. `Rendering` W2. `State,
//! identity and recovery` D5.

use std::collections::HashSet;
use std::time::SystemTime;

use smoozy_render_settings::file_half::BlenderIdentity;
use smoozy_server_protocol::transfers::SourceReading;
use smoozy_state::observed::{Clock, Reading};

use crate::bytes::MachineLocation;
use crate::record::{MachineKey, TheRecord};
use crate::rows::RowState;

/// **V10: whether the sitting completed** — a stopped or connection-cut
/// sitting is recorded incomplete and stops nothing. These are also the three
/// declared endings the diary's *checked a file's dependencies* line carries
/// (G15, G16).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Completion {
    Completed,
    StoppedByTheUser,
    TheConnectionDropped,
}

/// **V12: the standing's counts, its own vocabulary rather than the spec's
/// row states** — *found but not sent* and *confirmed* are the internal names
/// for *found on your computer* and *already on the machine* as they stand at
/// completion, and **there is no count for *sent before, not confirmed***,
/// which a sitting that ends without an upload can leave rows in. Nothing
/// breaks on that gap: such a row is not confirmed, so V11 leaves the warning
/// standing over it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Counts {
    pub not_found: usize,
    pub could_not_be_sent: usize,
    pub could_not_be_fixed: usize,
    pub found_but_not_sent: usize,
    pub set_aside: usize,
    pub confirmed: usize,
}

/// **The fixing half of V11 for a row inside a linked `.blend`**: the holding
/// file's fixing record must match the current located answers (M16's second
/// trigger), and no M16 trigger has left its re-production owed (M16b's
/// third).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixingStatus {
    pub the_fixing_matches_the_current_located_answers: bool,
    pub a_re_production_is_owed: bool,
}

impl FixingStatus {
    pub fn is_current(self) -> bool {
        self.the_fixing_matches_the_current_located_answers && !self.a_re_production_is_owed
    }
}

/// One row as it stands at the sitting's completion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RowAtCompletion {
    pub state: RowState,
    /// Present for a row inside a linked `.blend`.
    pub inside_a_linked_blend: Option<FixingStatus>,
}

impl RowAtCompletion {
    /// **V11: *confirmed on the machine* is Q5's establishment — and for a row
    /// inside a linked `.blend`, Q5's establishment plus a current fixing.**
    pub fn is_confirmed(&self) -> bool {
        self.state == RowState::AlreadyOnTheMachine
            && self
                .inside_a_linked_blend
                .map(FixingStatus::is_current)
                .unwrap_or(true)
    }
}

/// **V11: what stops the warning — a complete check whose every row is either
/// confirmed on the machine or set aside.** Rows the user set aside do not
/// count as unfound. **A row found on the user's computer and never sent
/// leaves the warning standing**: a found-but-unsent piece is a piece the
/// render will not find. Round 1 ruled that the faithful reading of *leaves
/// nothing unfound* — *nothing left out of place for the render* — rather
/// than a tightening beyond it.
pub fn every_row_is_confirmed_or_set_aside(rows: &[RowAtCompletion]) -> bool {
    rows.iter()
        .all(|row| row.is_confirmed() || row.state == RowState::SetAside)
}

/// **V12: the counts at completion.** A *sent before, not confirmed* row has
/// no count and is simply not confirmed.
pub fn counts_at_completion(rows: &[RowAtCompletion]) -> Counts {
    let mut counts = Counts::default();
    for row in rows {
        if row.is_confirmed() {
            counts.confirmed += 1;
            continue;
        }
        match row.state {
            RowState::NotFound => counts.not_found += 1,
            RowState::CouldNotBeSent => counts.could_not_be_sent += 1,
            RowState::CouldNotBeFixed => counts.could_not_be_fixed += 1,
            RowState::FoundOnYourComputer => counts.found_but_not_sent += 1,
            RowState::SetAside => counts.set_aside += 1,
            // An unconfirmed fixing over an *already there* row, or a
            // sent-before-not-confirmed row: not confirmed, no count.
            RowState::AlreadyOnTheMachine | RowState::SentBeforeNotConfirmed => {}
        }
    }
    counts
}

/// **V10: one check standing per (root file's machine path, machine under its
/// installation identity).** A standing under a superseded installation
/// identity is disqualified with everything else machine-keyed (K7) — a newly
/// rented or recycled machine has no standing, so for that machine nobody has
/// looked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckStanding {
    pub root: MachineLocation,
    pub machine: MachineKey,
    /// The Blender version the check read with (G2).
    pub read_with: BlenderIdentity,
    /// The moment the sitting completed, on the desktop's clock (`State,
    /// identity and recovery` D5), and whether it completed.
    pub at: Reading<()>,
    pub completion: Completion,
    /// The row counts per state at completion.
    pub counts: Counts,
    /// Whether every row was confirmed or set aside at completion (V11).
    pub every_row_confirmed_or_set_aside: bool,
    /// The four-value identity of the root's machine copy as the reading took
    /// it.
    pub root_four_values: SourceReading,
    /// **V13's first mark**: the root file's machine copy replaced.
    pub replaced_since_check: bool,
    /// **V13's third mark, V13a's widening**: a Smoozy act replaced, refixed or
    /// deleted anything the root's tree had resolved.
    pub disturbed: bool,
}

impl CheckStanding {
    /// **V10: written at the sitting's end, from the rows as they stand.**
    #[allow(clippy::too_many_arguments)]
    pub fn at_the_sittings_end(
        root: MachineLocation,
        machine: MachineKey,
        read_with: BlenderIdentity,
        at: SystemTime,
        completion: Completion,
        rows: &[RowAtCompletion],
        root_four_values: SourceReading,
    ) -> CheckStanding {
        CheckStanding {
            root,
            machine,
            read_with,
            at: Reading::taken((), at, Clock::ThisComputer),
            completion,
            counts: counts_at_completion(rows),
            every_row_confirmed_or_set_aside: every_row_is_confirmed_or_set_aside(rows),
            root_four_values,
            replaced_since_check: false,
            disturbed: false,
        }
    }

    /// **V13: any Smoozy publication onto the root's path marks the standing
    /// replaced-since-check**, because a check describes the file it read and
    /// what has just been published is a different file. **The mark is written
    /// by the module that published, at its publication: `File Browser` X7a for
    /// its own publications — an upload's, a copy's and a same-filesystem
    /// move's — and this module's own publication handling for a row send.**
    ///
    /// **The actor was widened from *any Smoozy upload* on 2026-09-05**: a copy
    /// and a same-filesystem move publish over the same path with the same
    /// **Replace** answer, and this trigger's own name and its own reason cover
    /// them. **V13a does not reach the case** — it marks the standings whose
    /// *tree* resolved the location, and a root is not a member of its own
    /// tree.
    pub fn replaced_since_check(mut self) -> CheckStanding {
        self.replaced_since_check = true;
        self
    }

    /// **V13a: the disturbance mark is written by the act, not by the warning
    /// in front of it** — any Smoozy act that replaced, refixed or deleted
    /// anything a root's tree resolved marks that root's standing disturbed,
    /// whether or not a warning stood before it.
    pub fn disturbed(mut self) -> CheckStanding {
        self.disturbed = true;
        self
    }

    /// **V13's version clause: a submission naming a Blender other than the
    /// standing's is not covered by it** — a check run with a different
    /// version would describe a file the render never reads. Round 1 ruled it
    /// correct and unopposed.
    pub fn covers(&self, the_submissions_version: &BlenderIdentity) -> bool {
        &self.read_with == the_submissions_version
    }
}

/// **V11 and V12 together: what `Rendering`'s warning does over a standing.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheWarning {
    /// V11: a complete check whose every row is confirmed or set aside, with
    /// no mark brought it back and the submission's version covered.
    Stopped,
    /// V12: a complete check that left things unfound replaces the warning
    /// with the standing's counts; wording is `Rendering`'s and Design's.
    Replaced(Counts),
    /// Standing — incomplete, marked, or a version the standing does not
    /// cover.
    Standing,
}

pub fn the_warning_over(
    standing: &CheckStanding,
    the_submissions_version: &BlenderIdentity,
) -> TheWarning {
    if standing.completion != Completion::Completed
        || standing.replaced_since_check
        || standing.disturbed
        || !standing.covers(the_submissions_version)
    {
        return TheWarning::Standing;
    }
    if standing.every_row_confirmed_or_set_aside {
        TheWarning::Stopped
    } else {
        TheWarning::Replaced(standing.counts)
    }
}

/// **V14: *nobody has ever checked this file on this machine* and *a check
/// was run and the file has been replaced since* are two different record
/// states, not two absences.** Supplied to `Rendering`'s Row 5 as data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheRecordSays {
    NobodyHasEverChecked,
    AStanding(Box<CheckStanding>),
}

pub fn what_the_record_says(
    root: &MachineLocation,
    machine: &MachineKey,
    record: &dyn TheRecord,
) -> WhatTheRecordSays {
    match record.check_standing(root, machine) {
        Some(standing) => WhatTheRecordSays::AStanding(Box::new(standing)),
        None => WhatTheRecordSays::NobodyHasEverChecked,
    }
}

/// **`File Browser` N6's answer for a file on this machine**: the source path
/// an upload through Files published it from, where that document's upload
/// records answer one. **Every caller that writes a mark hands it in**, since
/// those records are `File Browser`'s and this crate does not reach them.
pub type WhereFilesSaysItCameFrom<'a> =
    &'a dyn Fn(&MachineLocation) -> Option<crate::bytes::StoredLocation>;

/// **`File Browser` N6's answer where no file came up through Files** — for a caller that
/// holds no upload records, and for tests.
pub fn nothing_came_up_through_files(_: &MachineLocation) -> Option<crate::bytes::StoredLocation> {
    None
}

/// **Which holding file V13a's walk takes for a file on the machine** —
/// `File Browser` N6's source path first, then the source location of the
/// placement record that put it at that path (K7), and K5's machine-keyed
/// holding file where neither names it. **One rule, in one place**, so
/// [`roots_whose_tree_resolved`] and [`the_walk_reaches_the_roots_own_answers`]
/// cannot disagree about it.
///
/// **`File Browser` N6 first is the ruling of 2026-09-29, its first way, built at step
/// 10c's dependency wave piece 7b.** The check keys a root's answers under
/// `File Browser` N6's source path (K3), and a file uploaded through Files has no placement
/// record, so without `File Browser` N6's answer the walk looked a Files root's answers up
/// under a key they were never written under.
pub fn the_holding_file_the_walk_takes(
    file: &MachineLocation,
    machine: &MachineKey,
    placements: &[crate::record::PlacementRecord],
    came_up_through_files: WhereFilesSaysItCameFrom<'_>,
) -> crate::record::HoldingFile {
    use crate::record::HoldingFile;
    if let Some(source) = came_up_through_files(file) {
        return HoldingFile::OnTheUsersComputer(source);
    }
    placements
        .iter()
        .find(|placement| &placement.mirrored_location == file)
        .map(|placement| HoldingFile::OnTheUsersComputer(placement.source_location.clone()))
        .unwrap_or_else(|| HoldingFile::OnTheMachineWithNoKnownLocalOrigin {
            machine: machine.clone(),
            location: file.clone(),
        })
}

/// **Whether V13a's walk reaches a root's own answers** — whether the walk
/// keys the root the way the check that wrote them did.
///
/// **The case the walk was first written for**: a root whose holding file it
/// finds by the placement at the root's path, or by K5's machine key. **The
/// case it meets since piece 7b**: a root uploaded through Files, keyed by
/// `File Browser` N6's source path, which the walk now takes first. **Where
/// the record and the world parted company**: from the day the check keyed a
/// root by `File Browser` N6's answer (step 10c's dependency wave piece 5) until piece 7b,
/// a *Replace* over a file two projects shared marked the other project's
/// standing nothing.
///
/// **The safe side still binds.** Where this answers `false` — the check
/// keyed the root one way and the walk finds it another, as for a root that
/// came up as a dependency with no Files record — `Rendering`'s warning does
/// not stop; the reader of the standing asks this before it lets a standing
/// stop the warning.
pub fn the_walk_reaches_the_roots_own_answers(
    root: &MachineLocation,
    machine: &MachineKey,
    record: &dyn TheRecord,
    the_check_wrote_under: &crate::record::HoldingFile,
    came_up_through_files: WhereFilesSaysItCameFrom<'_>,
) -> bool {
    &the_holding_file_the_walk_takes(
        root,
        machine,
        &record.placements_on(machine),
        came_up_through_files,
    ) == the_check_wrote_under
}

/// **V13a: which roots' trees resolved a machine location** — the roots whose
/// standing the disturbing act marks. Walked through the record alone: the
/// holding file of each file on the machine ([`the_holding_file_the_walk_takes`]),
/// its located answers, and the mirrors those answers land at, down through
/// every linked `.blend`. Bounded by a visited set (G13), and reaching no
/// machine (V1).
pub fn roots_whose_tree_resolved(
    location: &MachineLocation,
    machine: &MachineKey,
    layout: &smoozy_job_files::layout::TheLayout,
    record: &dyn TheRecord,
    roots: &[MachineLocation],
    came_up_through_files: WhereFilesSaysItCameFrom<'_>,
) -> Vec<MachineLocation> {
    use crate::landing::mirror_of;

    let placements = record.placements_on(machine);
    let holding_file_of = |file: &MachineLocation| {
        the_holding_file_the_walk_takes(file, machine, &placements, came_up_through_files)
    };

    let mut out = Vec::new();
    for root in roots {
        let mut visited: HashSet<MachineLocation> = HashSet::new();
        let mut pending = vec![root.clone()];
        let mut reached = false;
        while let Some(file) = pending.pop() {
            if !visited.insert(file.clone()) {
                continue;
            }
            for answer in record.located_answers_of(&holding_file_of(&file)) {
                let Some(resolution) = answer.resolution else {
                    continue;
                };
                let Ok(at) = mirror_of(layout, resolution.to.location()) else {
                    continue;
                };
                if &at == location {
                    reached = true;
                }
                // **The walk descends on the file's name, and §12 item 26 is
                // why that is written here rather than argued for.** Nothing in
                // this project says how a resolved answer is recognised as a
                // linked file — no Fact, no source line, no research question,
                // no spec clause and no clause of this document — and the
                // question is **permanently unresolved**, the Creator's ruling
                // of 2026-08-31 having closed the route that would answer it.
                // It is **§12 item 26**, filed on 2026-09-05 out of round 2 of
                // the code audit of `File Browser`, and this note carried it
                // before the item existed to be named.
                // **No guess is written here and no behaviour is changed by
                // this note.**
                //
                // **What breaks if a library reference can name a file that is
                // not called `.blend`**: such a file never enters a root's
                // tree, so its own dependencies are invisible to this walk, a
                // later upload, copy, move or delete over one of them writes no
                // *disturbed* mark, `Rendering` W2 fires no submission warning,
                // and the render is paid for against a tree the check no longer
                // describes.
                //
                // **What a repair would need**: descending on the reference's
                // recorded **kind** rather than on the file's name. K4 holds
                // the kind since 2026-09-29 (step 10c's dependency wave piece
                // 7b, `LocatedAnswer::read_as`), so the record no longer
                // stands in the way; the question itself is still §12 item
                // 26's, and this walk is left on the name.
                if at.as_bytes().ends_with(b".blend") {
                    pending.push(at);
                }
            }
        }
        if reached {
            out.push(root.clone());
        }
    }
    out
}

/// **V15 as a value.**
pub const PRESENCE_SIZE_AND_THE_MACHINES_CLOCK_DECIDE_NOTHING: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bytes::{RecordedLocation, StoredLocation};
    use crate::check::tests::a_blender;
    use crate::landing::tests::a_layout;
    use crate::record::tests::{a_machine, a_publication, four_values};
    use crate::record::{
        HoldingFile, HowResolved, InMemoryRecord, LocatedAnswer, LocatedAnswerKey, PlacementRecord,
        Resolution, ResolvedTo,
    };
    use smoozy_server_setup::blender::Version;

    fn plain(state: RowState) -> RowAtCompletion {
        RowAtCompletion {
            state,
            inside_a_linked_blend: None,
        }
    }

    fn a_standing(rows: &[RowAtCompletion], completion: Completion) -> CheckStanding {
        CheckStanding::at_the_sittings_end(
            MachineLocation::composed(b"/root/assets/files/Users/a/scene.blend".to_vec()),
            a_machine(),
            a_blender(),
            SystemTime::UNIX_EPOCH,
            completion,
            rows,
            four_values(1, "1", 1, 1),
        )
    }

    #[test]
    fn a_complete_check_with_every_row_confirmed_or_set_aside_stops_the_warning() {
        let rows = [
            plain(RowState::AlreadyOnTheMachine),
            plain(RowState::SetAside),
        ];
        let standing = a_standing(&rows, Completion::Completed);
        assert_eq!(
            the_warning_over(&standing, &a_blender()),
            TheWarning::Stopped
        );
        assert_eq!(standing.counts.confirmed, 1);
        assert_eq!(standing.counts.set_aside, 1);
    }

    #[test]
    fn a_found_but_unsent_row_and_a_could_not_be_sent_row_replace_the_warning_with_counts() {
        let rows = [
            plain(RowState::AlreadyOnTheMachine),
            plain(RowState::FoundOnYourComputer),
            plain(RowState::CouldNotBeSent),
            plain(RowState::NotFound),
        ];
        let standing = a_standing(&rows, Completion::Completed);
        assert_eq!(
            the_warning_over(&standing, &a_blender()),
            TheWarning::Replaced(Counts {
                not_found: 1,
                could_not_be_sent: 1,
                could_not_be_fixed: 0,
                found_but_not_sent: 1,
                set_aside: 0,
                confirmed: 1,
            })
        );
    }

    #[test]
    fn a_sent_before_not_confirmed_row_has_no_count_and_leaves_the_warning_standing() {
        let rows = [plain(RowState::SentBeforeNotConfirmed)];
        let standing = a_standing(&rows, Completion::Completed);
        assert_eq!(standing.counts, Counts::default());
        assert_eq!(
            the_warning_over(&standing, &a_blender()),
            TheWarning::Replaced(Counts::default())
        );
    }

    #[test]
    fn a_row_inside_a_linked_blend_is_confirmed_only_with_a_current_fixing() {
        let current = RowAtCompletion {
            state: RowState::AlreadyOnTheMachine,
            inside_a_linked_blend: Some(FixingStatus {
                the_fixing_matches_the_current_located_answers: true,
                a_re_production_is_owed: false,
            }),
        };
        let stale = RowAtCompletion {
            state: RowState::AlreadyOnTheMachine,
            inside_a_linked_blend: Some(FixingStatus {
                the_fixing_matches_the_current_located_answers: false,
                a_re_production_is_owed: false,
            }),
        };
        let owed = RowAtCompletion {
            state: RowState::AlreadyOnTheMachine,
            inside_a_linked_blend: Some(FixingStatus {
                the_fixing_matches_the_current_located_answers: true,
                a_re_production_is_owed: true,
            }),
        };
        assert!(current.is_confirmed());
        assert!(!stale.is_confirmed());
        assert!(!owed.is_confirmed());
        assert!(every_row_is_confirmed_or_set_aside(&[current]));
        assert!(!every_row_is_confirmed_or_set_aside(&[stale]));
    }

    #[test]
    fn an_incomplete_sitting_stops_nothing_and_each_mark_brings_the_warning_back() {
        let rows = [plain(RowState::AlreadyOnTheMachine)];
        assert_eq!(
            the_warning_over(
                &a_standing(&rows, Completion::StoppedByTheUser),
                &a_blender()
            ),
            TheWarning::Standing
        );
        assert_eq!(
            the_warning_over(
                &a_standing(&rows, Completion::TheConnectionDropped),
                &a_blender()
            ),
            TheWarning::Standing
        );
        let standing = a_standing(&rows, Completion::Completed);
        assert_eq!(
            the_warning_over(&standing.clone().replaced_since_check(), &a_blender()),
            TheWarning::Standing
        );
        assert_eq!(
            the_warning_over(&standing.clone().disturbed(), &a_blender()),
            TheWarning::Standing
        );
        let other_version = BlenderIdentity {
            path: "/root/blender/blender-5.2.0/blender".to_string(),
            version: Version {
                major: 5,
                minor: 2,
                patch: 0,
            },
        };
        assert!(!standing.covers(&other_version));
        assert_eq!(
            the_warning_over(&standing, &other_version),
            TheWarning::Standing
        );
    }

    #[test]
    fn never_checked_and_replaced_since_are_two_record_states() {
        let machine = a_machine();
        let root = MachineLocation::composed(b"/root/assets/files/Users/a/scene.blend".to_vec());
        let mut record = InMemoryRecord::default();
        assert_eq!(
            what_the_record_says(&root, &machine, &record),
            WhatTheRecordSays::NobodyHasEverChecked
        );
        let mut standing = a_standing(
            &[plain(RowState::AlreadyOnTheMachine)],
            Completion::Completed,
        )
        .replaced_since_check();
        standing.machine = machine.clone();
        record.standings.push(standing.clone());
        assert_eq!(
            what_the_record_says(&root, &machine, &record),
            WhatTheRecordSays::AStanding(Box::new(standing))
        );
    }

    #[test]
    fn the_disturbed_roots_are_those_whose_tree_resolved_the_location_walked_through_the_record() {
        let machine = a_machine();
        let layout = a_layout();
        let mut record = InMemoryRecord::default();
        let scene = StoredLocation::as_the_system_returned_it(b"/Users/a/scene.blend");
        let lib = StoredLocation::as_the_system_returned_it(b"/Users/a/lib/c.blend");
        let tex = StoredLocation::as_the_system_returned_it(b"/Users/a/tex/x.png");
        let other = StoredLocation::as_the_system_returned_it(b"/Users/a/other.blend");
        let scene_at = crate::landing::mirror_of(&layout, &scene).expect("fits");
        let lib_at = crate::landing::mirror_of(&layout, &lib).expect("fits");
        let tex_at = crate::landing::mirror_of(&layout, &tex).expect("fits");
        let other_at = crate::landing::mirror_of(&layout, &other).expect("fits");
        for (at, source) in [(&scene_at, &scene), (&lib_at, &lib), (&other_at, &other)] {
            record.placements.push(PlacementRecord::at_publication(
                machine.clone(),
                a_publication(at.as_bytes(), source.as_bytes(), b"bytes"),
            ));
        }
        let answer =
            |holder: &StoredLocation, reference: &[u8], to: &StoredLocation| LocatedAnswer {
                key: LocatedAnswerKey {
                    holding_file: HoldingFile::OnTheUsersComputer(holder.clone()),
                    reference: RecordedLocation::as_the_reading_carried_it(reference),
                },
                resolution: Some(Resolution {
                    to: ResolvedTo::AWholeLocation(to.clone()),
                    how: HowResolved::TheRecordedLocationItself,
                }),
                set_aside: false,
                rejected_guess: None,
                read_as: Vec::new(),
            };
        // scene → lib → tex; other → nothing.
        record
            .located_answers
            .push(answer(&scene, b"//lib/c.blend", &lib));
        record
            .located_answers
            .push(answer(&lib, b"//tex/x.png", &tex));
        let roots = vec![scene_at.clone(), other_at.clone()];
        assert_eq!(
            roots_whose_tree_resolved(
                &tex_at,
                &machine,
                &layout,
                &record,
                &roots,
                &nothing_came_up_through_files
            ),
            vec![scene_at.clone()]
        );
        assert_eq!(
            roots_whose_tree_resolved(
                &lib_at,
                &machine,
                &layout,
                &record,
                &roots,
                &nothing_came_up_through_files
            ),
            vec![scene_at]
        );
        assert!(roots_whose_tree_resolved(
            &MachineLocation::composed(b"/root/assets/files/nothing".to_vec()),
            &machine,
            &layout,
            &record,
            &roots,
            &nothing_came_up_through_files
        )
        .is_empty());
    }

    /// **The ruling of 2026-09-29, its first way, built at step 10c's
    /// dependency wave piece 7b: the walk takes `File Browser` N6's answer
    /// first, so it reaches the answers of a root uploaded through Files.**
    /// Without that answer it keys the root by K5's machine key, the check's
    /// answers are under `File Browser` N6's source path, and the walk reaches nothing —
    /// which is what every Files root met until 7b.
    #[test]
    fn the_walk_takes_file_browsers_answer_first_and_reaches_a_root_from_files() {
        let machine = a_machine();
        let layout = a_layout();
        let source = StoredLocation::as_the_system_returned_it(b"/Users/a/scene.blend");
        // **Uploaded through Files**: at a place of the user's choosing on the
        // machine, with no placement record.
        let by_files = MachineLocation::composed(b"/root/projects/scene.blend".to_vec());
        let record = InMemoryRecord::default();
        let n6 = HoldingFile::OnTheUsersComputer(source.clone());
        let k5 = HoldingFile::OnTheMachineWithNoKnownLocalOrigin {
            machine: machine.clone(),
            location: by_files.clone(),
        };
        let files_says = |file: &MachineLocation| (file == &by_files).then(|| source.clone());
        assert!(the_walk_reaches_the_roots_own_answers(
            &by_files,
            &machine,
            &record,
            &n6,
            &files_says
        ));
        assert!(!the_walk_reaches_the_roots_own_answers(
            &by_files,
            &machine,
            &record,
            &k5,
            &files_says
        ));
        // **No Files record for it**: K5, as before.
        assert!(!the_walk_reaches_the_roots_own_answers(
            &by_files,
            &machine,
            &record,
            &n6,
            &nothing_came_up_through_files
        ));
        assert!(the_walk_reaches_the_roots_own_answers(
            &by_files,
            &machine,
            &record,
            &k5,
            &nothing_came_up_through_files
        ));

        // **Sent up as a dependency** — a placement at the root's own path —
        // and with no Files record the walk takes the source the placement
        // names.
        let mirrored = crate::landing::mirror_of(&layout, &source).expect("fits");
        let mut placed = InMemoryRecord::default();
        placed.placements.push(PlacementRecord::at_publication(
            machine.clone(),
            a_publication(mirrored.as_bytes(), source.as_bytes(), b"bytes"),
        ));
        assert_eq!(
            the_holding_file_the_walk_takes(
                &mirrored,
                &machine,
                &placed.placements,
                &nothing_came_up_through_files
            ),
            n6
        );
    }

    /// **V13a over a root uploaded through Files**: a *Replace* over a picture
    /// the root's tree resolved reaches the root, once `File Browser` N6's answer is handed
    /// in — and reaches nothing without it, which was the fault.
    #[test]
    fn a_publication_over_what_a_files_roots_tree_resolved_reaches_that_root() {
        let machine = a_machine();
        let layout = a_layout();
        let scene = StoredLocation::as_the_system_returned_it(b"/Users/a/scene.blend");
        let tex = StoredLocation::as_the_system_returned_it(b"/Users/a/tex/x.png");
        let by_files = MachineLocation::composed(b"/root/projects/scene.blend".to_vec());
        let tex_at = crate::landing::mirror_of(&layout, &tex).expect("fits");
        let mut record = InMemoryRecord::default();
        record.located_answers.push(LocatedAnswer {
            key: LocatedAnswerKey {
                holding_file: HoldingFile::OnTheUsersComputer(scene.clone()),
                reference: RecordedLocation::as_the_reading_carried_it(b"//tex/x.png"),
            },
            resolution: Some(Resolution {
                to: ResolvedTo::AWholeLocation(tex.clone()),
                how: HowResolved::TheRecordedLocationItself,
            }),
            set_aside: false,
            rejected_guess: None,
            read_as: Vec::new(),
        });
        let roots = vec![by_files.clone()];
        let files_says = |file: &MachineLocation| (file == &by_files).then(|| scene.clone());
        assert_eq!(
            roots_whose_tree_resolved(&tex_at, &machine, &layout, &record, &roots, &files_says),
            vec![by_files.clone()]
        );
        assert!(roots_whose_tree_resolved(
            &tex_at,
            &machine,
            &layout,
            &record,
            &roots,
            &nothing_came_up_through_files
        )
        .is_empty());
    }
}
