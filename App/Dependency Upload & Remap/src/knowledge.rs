//! What Smoozy knows about a machine, answerable before a write, and the
//! live-job dependency set — `Dependency Upload & Remap` §7.1 and §7.2, Rows
//! 172 and 173: V1–V9a.
//!
//! **V1: *what Smoozy placed and fixed on this machine, and what points at
//! what* is answered from the local record alone — the placement records, the
//! fixing records and the per-job resolved sets — at the moment a destination
//! is named, with no read of the machine.** All three warnings that read it
//! are shown before a write. **Every function in this file takes the record
//! and never a reading**, which is how the line is kept by the types.
//!
//! **These are the readings `File Browser`'s warning layer takes whole at its
//! W1**, and the record kinds its W1a reads are K7's placement record and
//! K8's fixing record ([`crate::record`]).
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `File Browser` W1, W1a, W1b, N7a. `Rendering`
//! W4a. `Job files and outputs` R15.
//!
//! **`File Browser` M3 and `Desktop app and delivery` A7 are not listed above
//! and are written out at the site**, because this plan defines an M3 and an
//! A7 of its own.
//!
//! **`Desktop app and delivery` G1, G2 and G3 are not listed above and are
//! written out at the site**, because this plan defines a G1, a G2 and a G3
//! of its own.

use smoozy_state::identities::Identity;

use crate::bytes::{MachineLocation, StoredLocation};
use crate::landing::mirror_of;
use crate::record::{HoldingFile, LiveJob, MachineKey, PlacementRecord, TheRecord};
use smoozy_job_files::layout::TheLayout;

/// **V2 and V2a: the points-at reading for a named machine location.**
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PointsAt {
    /// Which fixed copies' fixings point at it (K8) — a `.blend` that was
    /// fixed to point at assets and would be left pointing at nothing.
    pub fixed_copies_whose_fixing_points_at_it: Vec<MachineLocation>,
    /// Which placement records name it (K7), by the source location each was
    /// placed from — an asset shared by two projects because a later reference
    /// was pointed at the copy already there.
    pub placed_from: Vec<StoredLocation>,
    /// Which live or waiting jobs' resolved sets name it (K9), the remap
    /// material included.
    pub jobs_whose_set_names_it: Vec<Identity>,
    /// **V2a: which fixed copies were made from it** — one more field of a
    /// record V2 already reads rather than a second reading; what is done
    /// with it is M16b's trigger, which the user never meets.
    pub fixed_copies_made_from_it: Vec<MachineLocation>,
}

pub fn what_points_at(
    location: &MachineLocation,
    machine: &MachineKey,
    current_installation: &Identity,
    record: &dyn TheRecord,
) -> PointsAt {
    let mut out = PointsAt::default();
    for fixing in record.fixings_on(machine) {
        if !fixing.machine.is_current_under(current_installation) {
            continue;
        }
        if fixing.points_at.iter().any(|(_, at)| at == location) {
            out.fixed_copies_whose_fixing_points_at_it
                .push(fixing.fixed_copy.clone());
        }
        if &fixing.derived_from == location {
            out.fixed_copies_made_from_it.push(fixing.fixed_copy);
        }
    }
    for placement in record.placements_on(machine) {
        if placement.stands_under(current_installation) && &placement.mirrored_location == location
        {
            out.placed_from.push(placement.source_location);
        }
    }
    for job in record.live_or_waiting_jobs_on(machine) {
        if job.machine.is_current_under(current_installation) && job.depends_on(location) {
            out.jobs_whose_set_names_it.push(job.job);
        }
    }
    out
}

impl PointsAt {
    /// Whether anything the record shows depended on the location — the
    /// delete warning's whole test (V3).
    pub fn anything_depends_on_it(&self) -> bool {
        !self.fixed_copies_whose_fixing_points_at_it.is_empty()
            || !self.placed_from.is_empty()
            || !self.jobs_whose_set_names_it.is_empty()
    }
}

/// **V3: the three warnings that read it, and their wording is `File
/// Browser`'s.** All three warn and none blocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Warning {
    /// Anything the record shows depended on, whether or not a job is live.
    Delete,
    /// Q8's, before a replace — the same knowledge, its trigger Q8's contents
    /// test inside this module's own send pre-flight and never a warning-time
    /// read.
    Overwrite,
    /// A plain upload about to land on a file this module placed or fixed is
    /// named, with the offer to send it through this module instead.
    PlainUpload,
}

impl Warning {
    /// **V3a: every one of them is owned elsewhere.** The three of V3 are
    /// `File Browser`'s, which settles dismissibility at `File Browser` M3 — the delete
    /// warning dismissible under `file-browser.delete`, the other two not
    /// dismissible with no row written. The missing-assets warning whose
    /// standing V10–V15 build is `Rendering`'s W4a, `rendering.missing-assets`.
    pub fn owned_by(self) -> &'static str {
        "File Browser"
    }

    pub fn blocks(self) -> bool {
        false
    }
}

/// **V3a: this document owns no dismissible warning and contributes no row to
/// the one suppression table, and the set is enumerated rather than asserted.**
/// `Desktop app and delivery` G3 binds every module to record its dismissible
/// warnings' suppressions in the one table and `Desktop app and delivery` A7
/// makes a module that has
/// not declared its identities unfinished; **a declaration is satisfied by a
/// complete enumeration ending in none.** The check's own outcomes are
/// statements of what happened, each carried by a row or a report the user
/// acts on rather than dismisses; no spec sentence gives any of them a
/// switch-off. **Structural as well as counted**: `Desktop app and delivery`
/// G1 leaves no store outside the one table, so a per-check suppression here
/// would have neither a place nor an identity — its G2's invisible failure
/// closed by construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Statement {
    /// *Could not be read*, with the version named (G3).
    CouldNotBeRead,
    /// A sitting said plainly to be incomplete (G16).
    TheSittingIsIncomplete,
    /// The search's per-folder report (F7, F1a).
    TheSearchsPerFolderReport,
    /// The linked-item statement of G6a.
    ALinkedItemWasNotFoundInsideItsFile,
    /// The sent-up-as-a-dependency statement of V9a.
    SentUpAsADependencyBefore,
    /// A row's own reason under *could not be sent* or *could not be fixed*
    /// (A5, A3, M14).
    ARowsOwnReason,
}

impl Statement {
    pub const ALL: [Statement; 6] = [
        Statement::CouldNotBeRead,
        Statement::TheSittingIsIncomplete,
        Statement::TheSearchsPerFolderReport,
        Statement::ALinkedItemWasNotFoundInsideItsFile,
        Statement::SentUpAsADependencyBefore,
        Statement::ARowsOwnReason,
    ];

    /// Not a row, not a warning, no action, no suppression, no switch-off.
    pub fn is_dismissible(self) -> bool {
        false
    }
}

/// **V3a: the enumeration ending in none.**
pub const THIS_DOCUMENT_CONTRIBUTES_NO_ROW_TO_THE_ONE_SUPPRESSION_TABLE: bool = true;

/// **V4: the knowledge is one computer's.** A second computer's Smoozy cannot
/// warn about what the first placed — the spec's own limit, restated only
/// because the warnings' completeness inherits it.
pub const THE_KNOWLEDGE_IS_ONE_COMPUTERS: bool = true;

/// **V5: a job's dependency set is fixed at submission and never re-derived:
/// the manifest's `.blend` and the resolved set the remap material names
/// (K9).** `Job files and outputs` R15 supplies the first with its four-value
/// staleness rule; this is the second. One durable answer for the job's whole
/// life.
pub fn the_dependency_set_of(job: &LiveJob) -> (&MachineLocation, &crate::record::ResolvedSet) {
    (&job.blend, &job.resolved_set)
}

/// **V6: the one rule — what Smoozy knows changes only by Smoozy's own
/// recorded acts, and a warning fires before any act that touches what a live
/// or waiting job's set names.** The same asset can sit in several live jobs'
/// sets at once, and the warning names every job whose set holds it.
pub fn the_jobs_whose_set_names(location: &MachineLocation, jobs: &[LiveJob]) -> Vec<Identity> {
    jobs.iter()
        .filter(|job| job.depends_on(location))
        .map(|job| job.job.clone())
        .collect()
}

/// **V6 as a value**: the record is updated at the moment Smoozy publishes an
/// upload, a fixing, or performs a delete — never by watching the machine.
pub const THE_RECORD_CHANGES_ONLY_BY_SMOOZYS_OWN_RECORDED_ACTS: bool = true;

/// **V7: the three parked cases fall under that rule without extensions** — a
/// checked file replaced after the check (the snapshot undisturbed, the
/// live-job warning fires, the standing marked disturbed); one shared asset
/// replaced while several jobs need it (every holding job named); a
/// dependency selected later to render as a new root (no invalidation, a fresh
/// root check — V9).
pub const THE_THREE_PARKED_CASES_NEED_NO_EXTENSION: bool = true;

/// **V8: what a touched file does to the running render is deliberately not
/// modelled, and the warning is uniform because the behaviour is not.** A rule
/// per kind would be a model of behaviour the research shows is not uniform,
/// so the warning covers every touch the same way and promises nothing about
/// what the touch does.
pub const THE_WARNING_IS_UNIFORM_AND_PROMISES_NOTHING_ABOUT_WHAT_A_TOUCH_DOES: bool = true;

/// **V9: a file checked as a dependency has no check standing as a root.**
/// Rendering it needs its own check; submitting it unchecked gets the same
/// warning any unchecked file gets. Kept structurally: a standing is written
/// only for a root the user asked to check, and nothing in [`crate::standing`]
/// writes one from a tree walk.
pub const A_DEPENDENCY_HAS_NO_STANDING_AS_A_ROOT: bool = true;

/// **V9a: the panel's sent-up-as-a-dependency statement** — the file was sent
/// up as a dependency before, its own recorded locations still point at the
/// user's computer, and it needs a check like any other file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SentUpAsADependency {
    pub placed_at: MachineLocation,
}

/// **V9a's reading: K7 looked up at the mirror A4 derives from the root's own
/// source path.** The mirror is a function of the source location, so the
/// lookup is K7's own key reached through A4, not a scan and not a new index.
/// **It answers from the record of the act, never from the machine's present
/// contents**: a record a later confirmation marked gone still answers; a
/// disqualified record answers nothing, because a recycled machine was given
/// nothing. A K5 root, named by a machine location, takes the same lookup at
/// that location directly. Machine-scoped and one computer's (V4).
pub fn sent_up_as_a_dependency_before(
    root: &HoldingFile,
    layout: &TheLayout,
    machine: &MachineKey,
    current_installation: &Identity,
    record: &dyn TheRecord,
) -> Option<SentUpAsADependency> {
    let at = match root {
        HoldingFile::OnTheUsersComputer(source) => mirror_of(layout, source).ok()?,
        HoldingFile::OnTheMachineWithNoKnownLocalOrigin { location, .. } => location.clone(),
    };
    record
        .placements_on(machine)
        .into_iter()
        .find(|placement: &PlacementRecord| {
            placement.machine.is_current_under(current_installation)
                && placement.mirrored_location == at
        })
        .map(|placement| SentUpAsADependency {
            placed_at: placement.mirrored_location,
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::landing::tests::a_layout;
    use crate::record::tests::{a_machine, a_publication, contents};
    use crate::record::{
        ConfirmationOutcome, FixingRecord, InMemoryRecord, PlacementRecord, ResolvedSet,
    };
    use smoozy_state::identities::IdentityKind;

    fn at(bytes: &[u8]) -> MachineLocation {
        MachineLocation::composed(bytes.to_vec())
    }

    #[test]
    fn the_points_at_reading_answers_from_the_record_alone_and_names_every_holder() {
        let machine = a_machine();
        let mirror = at(b"/ephemeral/smoozy/assets/files/Users/a/tex/x.png");
        let mut record = InMemoryRecord::default();
        record.placements.push(PlacementRecord::at_publication(
            machine.clone(),
            a_publication(mirror.as_bytes(), b"/Users/a/tex/x.png", b"red"),
        ));
        let fixed = at(b"/ephemeral/smoozy/assets/fixed/assets/files/Users/a/lib/c.blend");
        record.fixings.push(FixingRecord {
            machine: machine.clone(),
            derived_from: at(b"/ephemeral/smoozy/assets/files/Users/a/lib/c.blend"),
            derived_from_content: contents(b"c"),
            fixed_copy: fixed.clone(),
            points_at: vec![(
                crate::bytes::RecordedLocation::as_the_reading_carried_it(b"//tex/x.png"),
                mirror.clone(),
            )],
        });
        let job = Identity::issue(IdentityKind::Job).expect("randomness");
        record.jobs.push(LiveJob {
            job: job.clone(),
            machine: machine.clone(),
            blend: at(b"/ephemeral/smoozy/assets/files/Users/a/scene.blend"),
            resolved_set: ResolvedSet {
                locations: vec![mirror.clone()],
                remap_material: at(b"/ephemeral/smoozy/assets/remaps/j"),
            },
        });

        let points = what_points_at(&mirror, &machine, &machine.installation, &record);
        assert_eq!(
            points.fixed_copies_whose_fixing_points_at_it,
            vec![fixed.clone()]
        );
        assert_eq!(
            points.placed_from,
            vec![StoredLocation::as_the_system_returned_it(
                b"/Users/a/tex/x.png"
            )]
        );
        assert_eq!(points.jobs_whose_set_names_it, vec![job.clone()]);
        assert!(points.anything_depends_on_it());

        // V2a: the derived-from location answers which fixed copies were
        // made from it.
        let derived = what_points_at(
            &at(b"/ephemeral/smoozy/assets/files/Users/a/lib/c.blend"),
            &machine,
            &machine.installation,
            &record,
        );
        assert_eq!(derived.fixed_copies_made_from_it, vec![fixed]);

        // The remap material is in the set, so the delete warning reaches it.
        let material = what_points_at(
            &at(b"/ephemeral/smoozy/assets/remaps/j"),
            &machine,
            &machine.installation,
            &record,
        );
        assert_eq!(material.jobs_whose_set_names_it, vec![job]);

        // A recycled machine's records answer nothing.
        let recycled = Identity::issue(IdentityKind::Installation).expect("randomness");
        assert!(!what_points_at(&mirror, &machine, &recycled, &record).anything_depends_on_it());
    }

    #[test]
    fn the_warnings_are_file_browsers_none_blocks_and_no_statement_is_dismissible() {
        for warning in [Warning::Delete, Warning::Overwrite, Warning::PlainUpload] {
            assert_eq!(warning.owned_by(), "File Browser");
            assert!(!warning.blocks());
        }
        assert_eq!(Statement::ALL.len(), 6);
        assert!(Statement::ALL
            .iter()
            .all(|statement| !statement.is_dismissible()));
    }

    #[test]
    fn every_job_whose_set_names_the_location_is_named() {
        let machine = a_machine();
        let shared = at(b"/ephemeral/smoozy/assets/files/Users/a/tex/x.png");
        let jobs: Vec<LiveJob> = (0..3)
            .map(|index| LiveJob {
                job: Identity::issue(IdentityKind::Job).expect("randomness"),
                machine: machine.clone(),
                blend: at(b"/b"),
                resolved_set: ResolvedSet {
                    locations: if index < 2 {
                        vec![shared.clone()]
                    } else {
                        vec![]
                    },
                    remap_material: at(format!("/r/{index}").as_bytes()),
                },
            })
            .collect();
        let named = the_jobs_whose_set_names(&shared, &jobs);
        assert_eq!(named, vec![jobs[0].job.clone(), jobs[1].job.clone()]);
        assert_eq!(the_dependency_set_of(&jobs[0]).0, &at(b"/b"));
    }

    #[test]
    fn the_sent_up_statement_is_k7_at_the_mirror_and_a_gone_record_still_answers() {
        let machine = a_machine();
        let layout = a_layout();
        let source = StoredLocation::as_the_system_returned_it(b"/Users/a/lib/c.blend");
        let root = HoldingFile::OnTheUsersComputer(source.clone());
        let mut record = InMemoryRecord::default();
        assert_eq!(
            sent_up_as_a_dependency_before(
                &root,
                &layout,
                &machine,
                &machine.installation,
                &record
            ),
            None
        );
        let mirror = mirror_of(&layout, &source).expect("fits");
        record.placements.push(
            PlacementRecord::at_publication(
                machine.clone(),
                a_publication(mirror.as_bytes(), source.as_bytes(), b"c"),
            )
            .confirmed(ConfirmationOutcome::Gone, std::time::SystemTime::UNIX_EPOCH),
        );
        assert_eq!(
            sent_up_as_a_dependency_before(
                &root,
                &layout,
                &machine,
                &machine.installation,
                &record
            ),
            Some(SentUpAsADependency { placed_at: mirror })
        );
        let recycled = Identity::issue(IdentityKind::Installation).expect("randomness");
        assert_eq!(
            sent_up_as_a_dependency_before(&root, &layout, &machine, &recycled, &record),
            None
        );
    }
}
