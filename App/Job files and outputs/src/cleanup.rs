//! §11 — cleanup, and the abandoned-material sweep M7 defines.
//!
//! **K1: two rulings are applied and no third rule is invented.** `State,
//! identity and recovery` §11.3 has already settled both halves and they are
//! taken as they stand.
//!
//! **K4: the abandoned-material sweep is bounded to the two directories this
//! document names, and it is record-driven, never enumeration-driven.**
//! *"Neither reaches anything the user put there, nor anything a render wrote …
//! and neither is triggered by a machine being reachable again."*
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` S28, S29.
//! `Queue & History` X2.

use crate::acting::{TheMachinesDirectories, WhatTheMachineSaid};
use crate::publication::AJobsPaths;

/// The things §11 K1 gives a lifetime to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Thing {
    /// **A fetched derivative, on the user's computer.**
    ///
    /// **K2: it is subject-held on its job record, and on its own age, and it
    /// is not throwaway on the user's computer.** The shape calls the
    /// derivatives throwaway and that is true of the machine-side set alone:
    /// `Queue & History` promises that a finished job keeps its preview, the
    /// ordinary end of a job is that the machine is destroyed, and once it is
    /// gone the copy cannot be made again. **K1's row below is K2 kept** — this
    /// one clears when the job record is deleted or the age ending is reached,
    /// and never with an action.
    AFetchedDerivative,
    /// **A machine-side still copy** — the set of B22, and not the media
    /// worker's moving picture or contact sheet.
    AMachineSideStillCopy,
    /// **A job record directory.**
    AJobRecordDirectory,
    /// **A job output folder whose record was published** — standing, or since
    /// erased by `Queue & History` X2's owed erase, which removes the record
    /// and never the folder (M7).
    AJobOutputFolderWhoseRecordWasPublished,
    /// **A job output folder whose record was never published** — the abandoned
    /// submission, and only that (M7).
    AJobOutputFolderWhoseRecordWasNeverPublished,
    /// **A staged directory or file this document published from.**
    AStagedDirectoryThisDocumentPublishedFrom,
}

/// K1's *cleared when* column.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClearedWhen {
    /// *"Its job record is deleted, **or its age ending is reached, whichever
    /// comes first**."* **The period is thirty days**, `State, identity and
    /// recovery` §11.1 R16, ruled 2026-09-02 — cited here and re-decided
    /// nowhere.
    TheJobRecordIsDeletedOrTheAgeEndingIsReached,
    /// *"Once the fetched copy exists, **or** the job's record is gone."*
    /// **Nothing earlier — the machine is the only place a derivative can be
    /// made.**
    TheFetchedCopyExistsOrTheJobsRecordIsGone,
    /// *"The job's record is gone, or the machine is destroyed."*
    TheJobsRecordIsGoneOrTheMachineIsDestroyed,
    /// *"It is the user's outputs and this document does not remove it while
    /// the machine lives"* — the removals are the machine-reaching server
    /// removal the specs already define, and the machine's own destruction.
    /// **Reached by no sweep, ever.**
    NotWhileTheMachineLives,
    /// **The abandoned-material sweep reaches it** (M7, K4).
    TheAbandonedMaterialSweepReachesIt,
    /// Its publication, under `Server protocol and transfers` §7.8.
    ItsPublication,
}

/// **K1's table, performed.**
pub fn cleared_when(thing: Thing) -> ClearedWhen {
    match thing {
        Thing::AFetchedDerivative => ClearedWhen::TheJobRecordIsDeletedOrTheAgeEndingIsReached,
        Thing::AMachineSideStillCopy => ClearedWhen::TheFetchedCopyExistsOrTheJobsRecordIsGone,
        Thing::AJobRecordDirectory => ClearedWhen::TheJobsRecordIsGoneOrTheMachineIsDestroyed,
        Thing::AJobOutputFolderWhoseRecordWasPublished => ClearedWhen::NotWhileTheMachineLives,
        Thing::AJobOutputFolderWhoseRecordWasNeverPublished => {
            ClearedWhen::TheAbandonedMaterialSweepReachesIt
        }
        Thing::AStagedDirectoryThisDocumentPublishedFrom => ClearedWhen::ItsPublication,
    }
}

/// **K3: nothing on the machine is cleared on the strength of a free-space
/// reading alone.**
///
/// `State, identity and recovery` §11.4, and the reason is Fact 28's: **the
/// reading is a choice of mount, and a machine whose disk is full is precisely
/// the machine whose records are most needed.** Nothing in this module takes a
/// free-space reading at all, which is that rule kept by there being no route.
pub const NOTHING_IS_CLEARED_ON_THE_STRENGTH_OF_A_FREE_SPACE_READING: bool = true;

/// **K5: a downloaded output on the user's computer is in no class at all.** It
/// is the user's file; **nothing counts it, caps it, ages it or removes it.**
pub const A_DOWNLOADED_OUTPUT_IS_IN_NO_CLASS_AT_ALL: bool = true;

/// What the desktop's own record says about one submission.
///
/// **M7 keys the sweep on this and never on what is found by enumerating the
/// job area** (S28, S29): *"the sweep begins from the desktop's own record of
/// the submission that stopped before M5's publication … **because that record
/// says the publication never happened, and never because of anything found by
/// enumerating the job area**."*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheDesktopRecordSays {
    /// **A submission whose outcome is not yet settled is not in that state and
    /// its folder is left alone.**
    TheOutcomeIsNotSettledYet,
    /// **A folder whose job's record was published is outside this sweep for
    /// good, whether or not that record still stands.** `Queue & History` X2's
    /// erase enumerates what it removes and the job output folder is not among
    /// it.
    TheRecordWasPublished,
    /// The submission stopped before M5's publication.
    ItStoppedBeforeThePublication,
}

/// One submission the sweep is asked about.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ASubmission {
    /// **The paths the desktop itself composed at submission** (M1), which is
    /// what the sweep removes at — not a path read off the machine.
    pub paths: AJobsPaths,
    pub record: WhatTheDesktopRecordSays,
}

/// What the sweep is owed against one submission, derived before anything is
/// asked of a machine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatIsOwed {
    pub the_job_output_folder: Option<String>,
    pub the_staging_directory: Option<String>,
}

impl WhatIsOwed {
    pub fn nothing() -> WhatIsOwed {
        WhatIsOwed {
            the_job_output_folder: None,
            the_staging_directory: None,
        }
    }

    pub fn is_nothing(&self) -> bool {
        self.the_job_output_folder.is_none() && self.the_staging_directory.is_none()
    }
}

/// **K4's derivation, and it is the whole of what decides.**
///
/// **Bounded to the two directories this document names**: a job output folder
/// whose record was never published, and a staged directory left by an
/// interrupted publication.
pub fn what_the_sweep_is_owed(submission: &ASubmission) -> WhatIsOwed {
    match submission.record {
        WhatTheDesktopRecordSays::TheOutcomeIsNotSettledYet
        | WhatTheDesktopRecordSays::TheRecordWasPublished => WhatIsOwed::nothing(),
        WhatTheDesktopRecordSays::ItStoppedBeforeThePublication => WhatIsOwed {
            the_job_output_folder: Some(submission.paths.job_output_folder.clone()),
            the_staging_directory: Some(submission.paths.staging_directory.clone()),
        },
    }
}

/// What the sweep did, per path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheSweepDid {
    ItWasRemoved,
    /// **It was not there**, which is the ordinary case for a submission that
    /// stopped before the folder was made.
    ItWasNotThere,
    /// **Something was inside it and the sweep did not reach in.** *"A folder
    /// whose record was never published has held no render, since without the
    /// published record no job ever joined the line"* — so this cannot happen
    /// from Smoozy's own acts, and where it happens anyway **the sweep leaves
    /// it exactly where it is** rather than removing what somebody else put
    /// there.
    SomethingWasInsideItSoItWasLeftAlone,
    /// **A name inside it is not text, so the sweep could not compose the path
    /// to remove at and left the directory standing.**
    ///
    /// **It is a separate outcome because the one above means somebody else's
    /// file was found**, and reporting that here would name a cause that is not
    /// the one that happened. **Smoozy cannot produce this case in a staging
    /// directory**: that directory holds only `manifest.json` and names this
    /// document itself composed, so a name the sweep cannot use is not one
    /// Smoozy put there.
    ANameInsideItIsNotTextSoItWasLeftStanding,
    TheMachineRefused(WhatTheMachineSaid),
}

/// **K4's sweep, performed over one submission.**
///
/// The staged directory goes first, because the material inside it is this
/// document's own and the directory cannot go while it holds any. **The job
/// output folder is only removed where it is empty**, and the sweep never
/// enumerates its way into one.
pub async fn sweep(
    machine: &mut impl TheMachinesDirectories,
    submission: &ASubmission,
) -> Vec<(String, WhatTheSweepDid)> {
    let owed = what_the_sweep_is_owed(submission);
    let mut done = Vec::new();
    if let Some(staging) = owed.the_staging_directory {
        done.push((staging.clone(), remove_a_directory(machine, &staging, true).await));
    }
    if let Some(folder) = owed.the_job_output_folder {
        done.push((folder.clone(), remove_a_directory(machine, &folder, false).await));
    }
    done
}

/// One directory, removed, with whatever this document itself staged inside it
/// removed first where that is allowed.
async fn remove_a_directory(
    machine: &mut impl TheMachinesDirectories,
    path: &str,
    what_is_inside_is_this_documents_own: bool,
) -> WhatTheSweepDid {
    let inside = match machine.list_one_directory(path).await {
        Ok(names) => names,
        Err(WhatTheMachineSaid::ThePathIsNotThere) => return WhatTheSweepDid::ItWasNotThere,
        Err(said) => return WhatTheSweepDid::TheMachineRefused(said),
    };
    if !inside.is_empty() {
        if !what_is_inside_is_this_documents_own {
            return WhatTheSweepDid::SomethingWasInsideItSoItWasLeftAlone;
        }
        for name in inside {
            // **N15 is kept where it matters and this line is not it.** The
            // listing came back as bytes and nothing was parsed out of a
            // rendered line, which is the whole of what N15 asks. **What
            // happens here is that the sweep needs the name as text to compose
            // the path it removes at**, and a name that is not text gets no
            // path composed — so the directory is left standing and the outcome
            // says which of the two it was. **The seam is not widened to carry
            // byte paths for it**: a staging directory holds only
            // `manifest.json` and names this document composed, so this is not
            // a case Smoozy can produce there.
            let Ok(name) = String::from_utf8(name) else {
                return WhatTheSweepDid::ANameInsideItIsNotTextSoItWasLeftStanding;
            };
            if let Err(said) = machine
                .remove_one_file(&format!("{path}/{name}"))
                .await
            {
                return WhatTheSweepDid::TheMachineRefused(said);
            }
        }
    }
    match machine.remove_one_empty_directory(path).await {
        Ok(()) => WhatTheSweepDid::ItWasRemoved,
        Err(WhatTheMachineSaid::ThePathIsNotThere) => WhatTheSweepDid::ItWasNotThere,
        Err(said) => WhatTheSweepDid::TheMachineRefused(said),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::{MachineRoot, TheLayout, WhereThePackageAreaSits};
    use crate::publication::testing::AMachine;
    use crate::publication::{place_a_jobs_material, AJobsPaths};
    use smoozy_state::identities::{Identity, IdentityKind};

    fn a_layout() -> TheLayout {
        TheLayout::of(
            MachineRoot::chosen("/ephemeral/smoozy").unwrap(),
            WhereThePackageAreaSits::UnderTheRoot,
        )
    }

    fn a_submission(record: WhatTheDesktopRecordSays) -> (TheLayout, Identity, ASubmission) {
        let layout = a_layout();
        let job = Identity::issue(IdentityKind::Job).unwrap();
        let paths = AJobsPaths::composed(&layout, &job, None).unwrap();
        (layout, job, ASubmission { paths, record })
    }

    #[test]
    fn a_published_record_is_outside_the_sweep_for_good() {
        let (_, _, submission) = a_submission(WhatTheDesktopRecordSays::TheRecordWasPublished);
        assert!(what_the_sweep_is_owed(&submission).is_nothing());
    }

    #[test]
    fn a_submission_whose_outcome_is_not_settled_is_left_alone() {
        let (_, _, submission) = a_submission(WhatTheDesktopRecordSays::TheOutcomeIsNotSettledYet);
        assert!(what_the_sweep_is_owed(&submission).is_nothing());
    }

    #[tokio::test]
    async fn the_sweep_removes_the_two_directories_m7_names_and_nothing_else() {
        let (layout, job, submission) =
            a_submission(WhatTheDesktopRecordSays::ItStoppedBeforeThePublication);
        let mut machine = AMachine::with_the_areas_of(&layout);
        // A submission that made both directories and stopped at the rename.
        machine.refuse_at = Some(submission.paths.job_record_directory.clone());
        place_a_jobs_material(
            &mut machine,
            &submission.paths,
            &crate::publication::testing::a_manifest(&job, &submission.paths),
        )
        .await;
        // Something belonging to somebody else, in the same areas.
        machine
            .directories
            .insert(format!("{}/somebody-else", layout.area(crate::layout::Area::Job)));

        let done = sweep(&mut machine, &submission).await;
        assert_eq!(
            done,
            vec![
                (
                    submission.paths.staging_directory.clone(),
                    WhatTheSweepDid::ItWasRemoved
                ),
                (
                    submission.paths.job_output_folder.clone(),
                    WhatTheSweepDid::ItWasRemoved
                ),
            ]
        );
        assert!(machine.directories.contains(&format!(
            "{}/somebody-else",
            layout.area(crate::layout::Area::Job)
        )));
        assert!(machine.files.is_empty(), "the staged manifest went with it");
    }

    #[tokio::test]
    async fn the_sweep_never_reaches_inside_a_job_output_folder() {
        // *"Neither reaches anything the user put there, nor anything a render
        // wrote."*
        let (layout, _, submission) =
            a_submission(WhatTheDesktopRecordSays::ItStoppedBeforeThePublication);
        let mut machine = AMachine::with_the_areas_of(&layout);
        machine
            .directories
            .insert(submission.paths.job_output_folder.clone());
        machine.files.insert(
            format!("{}/frame_0001.png", submission.paths.job_output_folder),
            vec![1, 2, 3],
        );

        let done = sweep(&mut machine, &submission).await;
        assert_eq!(
            done.last().unwrap().1,
            WhatTheSweepDid::SomethingWasInsideItSoItWasLeftAlone
        );
        assert!(machine
            .directories
            .contains(&submission.paths.job_output_folder));
        assert_eq!(machine.files.len(), 1);
    }

    #[tokio::test]
    async fn a_sweep_over_a_submission_that_never_reached_the_machine_removes_nothing_and_says_so() {
        let (layout, _, submission) =
            a_submission(WhatTheDesktopRecordSays::ItStoppedBeforeThePublication);
        let mut machine = AMachine::with_the_areas_of(&layout);
        let done = sweep(&mut machine, &submission).await;
        for (_, what) in done {
            assert_eq!(what, WhatTheSweepDid::ItWasNotThere);
        }
    }

    #[tokio::test]
    async fn a_name_the_sweep_cannot_use_is_not_reported_as_somebody_elses_material() {
        // The two outcomes are two different things: one says a file somebody
        // else put there was found, the other says the sweep could not compose
        // the path to remove at. Reporting the first for the second names a
        // cause that is not the one that happened.
        let (layout, _, submission) =
            a_submission(WhatTheDesktopRecordSays::ItStoppedBeforeThePublication);
        let mut machine = AMachine::with_the_areas_of(&layout);
        machine
            .directories
            .insert(submission.paths.staging_directory.clone());
        machine.a_name_that_is_not_text_inside = Some(submission.paths.staging_directory.clone());

        let done = sweep(&mut machine, &submission).await;
        assert_eq!(
            done.first().unwrap().1,
            WhatTheSweepDid::ANameInsideItIsNotTextSoItWasLeftStanding
        );
        // And it is left exactly where it is.
        assert!(machine
            .directories
            .contains(&submission.paths.staging_directory));
    }

    #[test]
    fn k1s_table_gives_the_user_s_outputs_no_removal_at_all() {
        assert_eq!(
            cleared_when(Thing::AJobOutputFolderWhoseRecordWasPublished),
            ClearedWhen::NotWhileTheMachineLives
        );
        // And the abandoned one is the only job output folder any sweep reaches.
        assert_eq!(
            cleared_when(Thing::AJobOutputFolderWhoseRecordWasNeverPublished),
            ClearedWhen::TheAbandonedMaterialSweepReachesIt
        );
    }
}
