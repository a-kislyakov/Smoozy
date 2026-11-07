//! §3.3 and §2.4 — the job's material published as one object, and the two
//! directories a job needs.
//!
//! **M5: a job's whole submission material is staged in one directory and
//! published by a single rename of that directory onto the job record
//! directory's path.** `Rendering`'s Row 2 promises that a job *"either reaches
//! the machine and joins the line, or it does not exist at all"*, which `State,
//! identity and recovery` §9.2 test 1 makes atomic entire — and **test 2 forbids
//! reaching that by ordering or by care**. **Fact 78's two-file half is answered
//! and negative** — *"no single operation updates two independent files
//! atomically"* — so several files cannot be published one at a time and called
//! atomic. **One directory rename is one operation.**
//!
//! **M6: that publication needs no extension, because a job identity is never
//! reused.** The base protocol's rename *"refuses to replace an existing
//! target"* and only the POSIX-rename extension overwrites (S16). **The target
//! here never exists**, so the base operation is enough and S17's declared
//! degradation does not reach this unit.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` S1, S16, S17.

use crate::acting::{TheMachinesDirectories, WhatTheMachineSaid};
use crate::attempts::AttemptRecord;
use crate::manifest::SubmissionManifest;
use crate::names::{self, WhyThePathIsRefused};

/// **The paths a job's publication uses, at the path they have always been
/// reached at**, with the staging prefix beside them. They live in
/// `smoozy-job-files-machine`, the half of this plan the rented machine's own
/// program may take, because the machine reads the manifest those paths are
/// composed into. **What stays here is every act**: the placement, the
/// attempt record's write, and their test machine. Split at step 10c piece 3.
pub use smoozy_job_files_machine::publication::{the_setup_runs_staging_folder, AJobsPaths};
pub(crate) use smoozy_job_files_machine::publication::THE_STAGING_PREFIX;

/// Which step of the placement was being taken.
///
/// **§12: *where the folder cannot be made, the record is **not attempted**,
/// naming the folder as the unit that stopped it*.** So the ending names a
/// step.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhichStep {
    /// **M7 and §12's ordered pair**: the job output folder is created **before**
    /// the record is published. *"The order is that way round so `Queue &
    /// History`'s *show outputs* has a folder to open on a job that has written
    /// nothing yet."*
    CreatingTheJobOutputFolder,
    CreatingTheStagingDirectory,
    WritingTheMaterialIntoTheStagingDirectory,
    /// **M5's one rename.**
    ThePublication,
}

/// How a placement ended. **There is no third ending**, which is what Row 2's
/// *either reaches the machine and joins the line, or it does not exist at all*
/// needs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HowThePlacementEnded {
    /// The record is published. The job exists.
    ItLanded,
    /// **The record was not published**, and the step that stopped it is named.
    ///
    /// **T1: no unit here is retried inside its action.** A failed submission is
    /// a new submission with a new job identity and a new folder, which is
    /// `Rendering`'s.
    ItStoppedAt(WhichStep, WhatTheMachineSaid),
}

/// **M5, M7 and N9 performed as one act.**
///
/// The order, and every step of it is one of §12's:
///
/// 1. **The job output folder**, with N9's plain create-directory operation.
///    **N10: because a job identity is never reused, a refusal at a fresh job
///    folder is a fault and not a case** — any refusal fails the submission,
///    and no attempt is made to tell *already exists* from anything else.
/// 2. **The staging directory**, beside the job record directory's own path.
/// 3. **The material**, written into it and made durable.
/// 4. **One rename**, of the staging directory onto the job record directory's
///    path.
///
/// **Nothing is created deep** (N11): both directories are one segment under an
/// area that already exists, and the five areas exist before the first job.
///
/// **Step 1 is taken only where the manifest names the folder** (M7 as
/// narrowed 2026-10-04): **where it says the folder is chosen at render start
/// (§6.6 P21's second case) no folder is made here**, the render worker makes
/// it, and a submission that stops before publication leaves no folder for
/// M7's sweep. The Creator's second notes, piece 4.
pub async fn place_a_jobs_material(
    machine: &mut impl TheMachinesDirectories,
    paths: &AJobsPaths,
    manifest: &SubmissionManifest,
) -> HowThePlacementEnded {
    if let Some(named) = manifest.job_output_folder.the_path_it_names() {
        if let Err(said) = machine.make_one_directory(named).await {
            return HowThePlacementEnded::ItStoppedAt(WhichStep::CreatingTheJobOutputFolder, said);
        }
    }
    if let Err(said) = machine.make_one_directory(&paths.staging_directory).await {
        return HowThePlacementEnded::ItStoppedAt(WhichStep::CreatingTheStagingDirectory, said);
    }
    let manifest_at = paths.the_manifest_where_it_is_written();
    if let Err(said) = machine
        .write_a_whole_file_durably(&manifest_at, &manifest.as_written())
        .await
    {
        return HowThePlacementEnded::ItStoppedAt(
            WhichStep::WritingTheMaterialIntoTheStagingDirectory,
            said,
        );
    }
    match machine
        .rename_without_replacing(&paths.staging_directory, &paths.job_record_directory)
        .await
    {
        Ok(()) => HowThePlacementEnded::ItLanded,
        Err(said) => HowThePlacementEnded::ItStoppedAt(WhichStep::ThePublication, said),
    }
}

/// **M8, performed.**
///
/// One attempt record, **written durably before the render starts**, into the
/// job record directory. **It is written once and never rewritten**: the name
/// carries the attempt's number, so a later attempt writes a new file rather
/// than opening this one.
///
/// **§12 makes it atomic by staging and one publication**, for the same reason
/// M5 is: *"a half-written frame list read by a later re-run would name a last
/// frame that was never asked for"*.
///
/// **N5's budget is checked here on both of this record's paths, before either
/// is used.** [`AJobsPaths::composed`] cannot check them: the name carries the
/// attempt's number and is only known once there is a record.
pub async fn write_an_attempt_record(
    machine: &mut impl TheMachinesDirectories,
    paths: &AJobsPaths,
    record: &AttemptRecord,
) -> Result<(), WhyTheAttemptRecordWasNotWritten> {
    let staged = format!(
        "{}/{THE_STAGING_PREFIX}{}",
        paths.job_record_directory,
        record.file_name()
    );
    let published = format!("{}/{}", paths.job_record_directory, record.file_name());
    // **N8, and N6's cause established before the machine is asked.** Both
    // paths are checked, not only the folder they sit in.
    names::a_path_fits(&staged).map_err(WhyTheAttemptRecordWasNotWritten::ThePathIsRefused)?;
    names::a_path_fits(&published).map_err(WhyTheAttemptRecordWasNotWritten::ThePathIsRefused)?;
    machine
        .write_a_whole_file_durably(&staged, &record.as_written())
        .await
        .map_err(WhyTheAttemptRecordWasNotWritten::TheMachineSaid)?;
    machine
        .rename_without_replacing(&staged, &published)
        .await
        .map_err(WhyTheAttemptRecordWasNotWritten::TheMachineSaid)
}

/// Why an attempt record did not go on the machine.
///
/// **Two causes and they are not the same kind of thing.** One is Smoozy's own
/// arithmetic against N5's budget, established before the machine is asked
/// (N6); the other is what the machine said.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhyTheAttemptRecordWasNotWritten {
    ThePathIsRefused(WhyThePathIsRefused),
    TheMachineSaid(WhatTheMachineSaid),
}

#[cfg(test)]
pub(crate) mod testing {
    use super::*;
    use crate::layout::TheLayout;
    use std::collections::{BTreeMap, BTreeSet};
    use crate::manifest::{
        AnchorState, CarriedAndNotInterpretedHere, FrameRange, SceneField, TheBlenderToRenderBy,
        TheRecordedReading,
    };
    use smoozy_server_protocol::transfers::SourceReading;
    use smoozy_state::identities::Identity;

    /// **A copy of `manifest.rs`'s own test helper, made on 2026-09-19** when
    /// that file left for `smoozy-job-files-machine`: a test-only helper in
    /// another crate cannot be reached from this crate's tests, and the
    /// placement's tests here still need a manifest to place. **Keep the two
    /// the same**; the note on the original says what it cost once when two
    /// helpers disagreed.
    ///
    /// **One label, and the manifest's folder is the paths' folder.** This
    /// helper used to compose a folder of its own carrying the label
    /// `Kitchen`, while the placement's own test composed its paths with no
    /// label at all — and the two disagreed with nothing anywhere noticing.
    pub(crate) fn a_manifest(job: &Identity, paths: &AJobsPaths) -> SubmissionManifest {
        SubmissionManifest::composed(
            job,
            None,
            TheBlenderToRenderBy {
                version: "4.5.9".to_string(),
                build_path: "/ephemeral/smoozy/blender/blender-4.5.9/blender".to_string(),
            },
            SceneField {
                name: "Scene".to_string(),
                anchor: AnchorState::Anchored {
                    the_reading_it_came_from: "reading-1".to_string(),
                },
            },
            FrameRange::TheUsersParsedIntegers(vec![1, 2, 3]),
            CarriedAndNotInterpretedHere(serde_json::json!({"samples": 128})),
            CarriedAndNotInterpretedHere(serde_json::json!({"remap": "asset-set-1"})),
            CarriedAndNotInterpretedHere(serde_json::json!({"reporting": "everything"})),
            "/ephemeral/smoozy/assets/kitchen.blend".to_string(),
            SourceReading {
                size: 4096,
                inode: "912".to_string(),
                modification_time: 1_756_800_000,
                change_time: 1_756_800_001,
            },
            TheRecordedReading::ThereIsNone,
            // **M1b's card make**, as the desktop composes it from the
            // hardware reading `Servers` holds.
            smoozy_job_files_machine::manifest::TheCardMakeAtSubmission::TheMakes(vec![
                "NVIDIA".to_string()
            ]),
            crate::manifest::WhereTheJobOutputFolderIs::named_by(paths),
        )
    }


    /// A machine that keeps what it was asked to keep, so every ordering rule
    /// above is exercised without a socket.
    #[derive(Default)]
    pub(crate) struct AMachine {
        pub directories: BTreeSet<String>,
        pub files: BTreeMap<String, Vec<u8>>,
        pub made_durable: BTreeSet<String>,
        pub refuse_at: Option<String>,
        /// **A path this machine refuses to read**, so a test can put the
        /// failure on the read alone with the write having landed.
        pub refuse_reads_at: Option<String>,
        /// **A path this machine refuses to remove.**
        pub refuse_removals_at: Option<String>,
        /// Every path this machine was ever asked to create, in order, so a
        /// test can assert the order rather than the outcome.
        pub asked_to_create: Vec<String>,
        /// **A name this machine returns that is not text**, inside the
        /// directory named, so a test can reach the sweep's own path for one.
        pub a_name_that_is_not_text_inside: Option<String>,
    }

    impl AMachine {
        pub(crate) fn with_the_areas_of(layout: &TheLayout) -> AMachine {
            let mut machine = AMachine::default();
            for area in crate::layout::EVERY_AREA {
                machine.directories.insert(layout.area(area));
            }
            machine
        }

        fn parent_of(path: &str) -> Option<String> {
            let cut = path.rfind('/')?;
            Some(path[..cut].to_string())
        }
    }

    impl TheMachinesDirectories for AMachine {
        async fn make_one_directory(&mut self, path: &str) -> Result<(), WhatTheMachineSaid> {
            self.asked_to_create.push(path.to_string());
            if self.refuse_at.as_deref() == Some(path) {
                return Err(WhatTheMachineSaid::ItRefused);
            }
            // **The plain form refuses an existing target and makes no
            // parents**, which is the whole of what N9 is about.
            match AMachine::parent_of(path) {
                Some(parent) if !self.directories.contains(&parent) => {
                    return Err(WhatTheMachineSaid::ThePathIsNotThere)
                }
                _ => {}
            }
            if !self.directories.insert(path.to_string()) {
                return Err(WhatTheMachineSaid::ItRefused);
            }
            Ok(())
        }

        async fn write_a_whole_file_durably(
            &mut self,
            path: &str,
            bytes: &[u8],
        ) -> Result<(), WhatTheMachineSaid> {
            if self.refuse_at.as_deref() == Some(path) {
                return Err(WhatTheMachineSaid::ItRefused);
            }
            match AMachine::parent_of(path) {
                Some(parent) if !self.directories.contains(&parent) => {
                    return Err(WhatTheMachineSaid::ThePathIsNotThere)
                }
                _ => {}
            }
            self.files.insert(path.to_string(), bytes.to_vec());
            self.made_durable.insert(path.to_string());
            Ok(())
        }

        async fn read_a_whole_file(&mut self, path: &str) -> Result<Vec<u8>, WhatTheMachineSaid> {
            if self.refuse_reads_at.as_deref() == Some(path) {
                return Err(WhatTheMachineSaid::TheChannelIsGone);
            }
            self.files
                .get(path)
                .cloned()
                .ok_or(WhatTheMachineSaid::ThePathIsNotThere)
        }

        async fn rename_without_replacing(
            &mut self,
            from: &str,
            to: &str,
        ) -> Result<(), WhatTheMachineSaid> {
            if self.refuse_at.as_deref() == Some(to) {
                return Err(WhatTheMachineSaid::ItRefused);
            }
            if self.directories.contains(to) || self.files.contains_key(to) {
                // **The base rename refuses to replace an existing target.**
                return Err(WhatTheMachineSaid::ItRefused);
            }
            if self.directories.remove(from) {
                self.directories.insert(to.to_string());
                let moving: Vec<String> = self
                    .files
                    .keys()
                    .filter(|path| path.starts_with(&format!("{from}/")))
                    .cloned()
                    .collect();
                for path in moving {
                    let bytes = self.files.remove(&path).unwrap();
                    self.files
                        .insert(path.replacen(from, to, 1), bytes);
                }
                return Ok(());
            }
            match self.files.remove(from) {
                Some(bytes) => {
                    self.files.insert(to.to_string(), bytes);
                    Ok(())
                }
                None => Err(WhatTheMachineSaid::ThePathIsNotThere),
            }
        }

        async fn remove_one_file(&mut self, path: &str) -> Result<(), WhatTheMachineSaid> {
            if self.refuse_removals_at.as_deref() == Some(path) {
                return Err(WhatTheMachineSaid::TheLoginMayNot);
            }
            match self.files.remove(path) {
                Some(_) => Ok(()),
                None => Err(WhatTheMachineSaid::ThePathIsNotThere),
            }
        }

        async fn remove_one_empty_directory(
            &mut self,
            path: &str,
        ) -> Result<(), WhatTheMachineSaid> {
            if self
                .files
                .keys()
                .any(|held| held.starts_with(&format!("{path}/")))
            {
                return Err(WhatTheMachineSaid::ItRefused);
            }
            if self.directories.remove(path) {
                Ok(())
            } else {
                Err(WhatTheMachineSaid::ThePathIsNotThere)
            }
        }

        async fn list_one_directory(
            &mut self,
            path: &str,
        ) -> Result<Vec<Vec<u8>>, WhatTheMachineSaid> {
            if !self.directories.contains(path) {
                return Err(WhatTheMachineSaid::ThePathIsNotThere);
            }
            let inside = format!("{path}/");
            let mut names: BTreeSet<Vec<u8>> = BTreeSet::new();
            if self.a_name_that_is_not_text_inside.as_deref() == Some(path) {
                names.insert(vec![0xff, 0xfe]);
            }
            for held in self.directories.iter().chain(self.files.keys()) {
                if let Some(rest) = held.strip_prefix(&inside) {
                    let first = rest.split('/').next().unwrap_or(rest);
                    if !first.is_empty() {
                        names.insert(first.as_bytes().to_vec());
                    }
                }
            }
            Ok(names.into_iter().collect())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::testing::AMachine;
    use super::*;
    use crate::layout::{MachineRoot, TheLayout, WhereThePackageAreaSits};
    use crate::manifest::MANIFEST_FILE_NAME;
    use smoozy_state::identities::{Identity, IdentityKind};

    fn a_layout() -> TheLayout {
        TheLayout::of(
            MachineRoot::chosen("/ephemeral/smoozy").unwrap(),
            WhereThePackageAreaSits::UnderTheRoot,
        )
    }

    fn a_job() -> Identity {
        Identity::issue(IdentityKind::Job).unwrap()
    }

    /// An absolute path of exactly this many bytes, in segments no name budget
    /// could object to.
    fn a_root_of_exactly(bytes: usize) -> String {
        assert!(bytes >= 4);
        if bytes.is_multiple_of(2) {
            "/a".repeat(bytes / 2)
        } else {
            format!("{}/aa", "/a".repeat((bytes - 3) / 2))
        }
    }

    #[tokio::test]
    async fn the_material_lands_and_the_staging_name_is_gone() {
        let layout = a_layout();
        let job = a_job();
        let paths = AJobsPaths::composed(&layout, &job, Some("Kitchen")).unwrap();
        let manifest = crate::publication::testing::a_manifest(&job, &paths);
        let mut machine = AMachine::with_the_areas_of(&layout);

        assert_eq!(
            place_a_jobs_material(&mut machine, &paths, &manifest).await,
            HowThePlacementEnded::ItLanded
        );
        assert!(machine.directories.contains(&paths.job_record_directory));
        assert!(
            !machine.directories.contains(&paths.staging_directory),
            "the rename is the cleanup and the staging name must be gone"
        );
        // The manifest came with it, under the published path.
        let read = machine
            .files
            .get(&format!(
                "{}/{MANIFEST_FILE_NAME}",
                paths.job_record_directory
            ))
            .cloned()
            .unwrap();
        assert_eq!(SubmissionManifest::read_back(&read).unwrap(), manifest);
    }

    #[tokio::test]
    async fn the_job_output_folder_is_made_before_the_record_is_published() {
        // M7's order, and §12's ordered pair, asserted on what the machine was
        // asked rather than on what came back.
        let layout = a_layout();
        let job = a_job();
        let paths = AJobsPaths::composed(&layout, &job, None).unwrap();
        let mut machine = AMachine::with_the_areas_of(&layout);
        place_a_jobs_material(&mut machine, &paths, &crate::publication::testing::a_manifest(&job, &paths)).await;
        assert_eq!(
            machine.asked_to_create,
            vec![
                paths.job_output_folder.clone(),
                paths.staging_directory.clone()
            ]
        );
    }

    /// **M7 as narrowed 2026-10-04 (§6.6 P21's second case): a manifest that
    /// says the folder is chosen at render start makes no folder at
    /// submission** — the render worker makes it where the file points, and a
    /// folder made here in Smoozy's job area would be one nothing ever writes
    /// into. The Creator's second notes, piece 4.
    #[tokio::test]
    async fn a_folder_chosen_at_render_start_is_not_made_at_submission() {
        let layout = a_layout();
        let job = a_job();
        let paths = AJobsPaths::composed(&layout, &job, None).unwrap();
        let mut manifest = crate::publication::testing::a_manifest(&job, &paths);
        manifest.job_output_folder = crate::manifest::WhereTheJobOutputFolderIs::ChosenAtRenderStart {
            the_folders_name: crate::names::job_output_folder_name(&job, None),
            the_servers_fallback_folder_at_submission: None,
        };
        let mut machine = AMachine::with_the_areas_of(&layout);
        assert_eq!(place_a_jobs_material(&mut machine, &paths, &manifest).await, HowThePlacementEnded::ItLanded);
        assert_eq!(machine.asked_to_create, vec![paths.staging_directory.clone()]);
        assert!(!machine.directories.contains(&paths.job_output_folder));
    }

    #[tokio::test]
    async fn where_the_folder_cannot_be_made_the_record_is_not_attempted() {
        // §12: *"the record is **not attempted**, naming the folder as the unit
        // that stopped it."*
        let layout = a_layout();
        let job = a_job();
        let paths = AJobsPaths::composed(&layout, &job, None).unwrap();
        let mut machine = AMachine::with_the_areas_of(&layout);
        machine.refuse_at = Some(paths.job_output_folder.clone());

        let ended =
            place_a_jobs_material(&mut machine, &paths, &crate::publication::testing::a_manifest(&job, &paths))
                .await;
        assert_eq!(
            ended,
            HowThePlacementEnded::ItStoppedAt(
                WhichStep::CreatingTheJobOutputFolder,
                WhatTheMachineSaid::ItRefused
            )
        );
        assert!(!machine.directories.contains(&paths.staging_directory));
        assert!(!machine.directories.contains(&paths.job_record_directory));
        assert!(machine.files.is_empty(), "nothing was written at all");
    }

    #[tokio::test]
    async fn a_publication_that_is_refused_leaves_no_record_and_the_staging_still_there() {
        // Row 2: *"either reaches the machine and joins the line, or it does not
        // exist at all."* A refused rename leaves the staging directory, which
        // K4's sweep is what reaches.
        let layout = a_layout();
        let job = a_job();
        let paths = AJobsPaths::composed(&layout, &job, None).unwrap();
        let mut machine = AMachine::with_the_areas_of(&layout);
        machine.refuse_at = Some(paths.job_record_directory.clone());

        let ended =
            place_a_jobs_material(&mut machine, &paths, &crate::publication::testing::a_manifest(&job, &paths))
                .await;
        assert!(matches!(
            ended,
            HowThePlacementEnded::ItStoppedAt(WhichStep::ThePublication, _)
        ));
        assert!(!machine.directories.contains(&paths.job_record_directory));
        assert!(machine.directories.contains(&paths.staging_directory));
    }

    #[tokio::test]
    async fn m6_needs_no_extension_because_the_target_never_exists() {
        // And the base operation's refusal is the backstop: publishing onto a
        // path that already holds something does not replace it.
        let layout = a_layout();
        let job = a_job();
        let paths = AJobsPaths::composed(&layout, &job, None).unwrap();
        let mut machine = AMachine::with_the_areas_of(&layout);
        machine
            .directories
            .insert(paths.job_record_directory.clone());

        let ended =
            place_a_jobs_material(&mut machine, &paths, &crate::publication::testing::a_manifest(&job, &paths))
                .await;
        assert!(matches!(
            ended,
            HowThePlacementEnded::ItStoppedAt(WhichStep::ThePublication, _)
        ));
    }

    #[tokio::test]
    async fn nothing_is_ever_created_deep() {
        // N11: *"the parent directories are made once, at hand-over, so no area
        // is ever created deep."* A machine whose areas do not exist refuses
        // rather than making them.
        let layout = a_layout();
        let job = a_job();
        let paths = AJobsPaths::composed(&layout, &job, None).unwrap();
        let mut machine = AMachine::default();
        let ended =
            place_a_jobs_material(&mut machine, &paths, &crate::publication::testing::a_manifest(&job, &paths))
                .await;
        assert_eq!(
            ended,
            HowThePlacementEnded::ItStoppedAt(
                WhichStep::CreatingTheJobOutputFolder,
                WhatTheMachineSaid::ThePathIsNotThere
            )
        );
    }

    #[tokio::test]
    async fn an_attempt_record_lands_durably_and_a_second_one_does_not_touch_the_first() {
        let layout = a_layout();
        let job = a_job();
        let paths = AJobsPaths::composed(&layout, &job, None).unwrap();
        let mut machine = AMachine::with_the_areas_of(&layout);
        place_a_jobs_material(&mut machine, &paths, &crate::publication::testing::a_manifest(&job, &paths)).await;

        let first = AttemptRecord {
            number: 1,
            frame_list_in_the_order_it_was_written: vec![1, 2, 3],
            map_in_force: crate::attempts::TheMapInForce::NoneWasComposedForThisAttempt,
            file_identity_at_this_attempt: crate::publication::testing::a_manifest(&job, &paths)
                .file_identity_at_submission,
            the_file_changed_before_this_attempt: false,
            a_frame_an_earlier_attempt_wrote_was_kept: false,
        };
        let mut second = first.clone();
        second.number = 2;
        second.frame_list_in_the_order_it_was_written = vec![3];

        write_an_attempt_record(&mut machine, &paths, &first)
            .await
            .unwrap();
        write_an_attempt_record(&mut machine, &paths, &second)
            .await
            .unwrap();

        let at = |record: &AttemptRecord| {
            format!("{}/{}", paths.job_record_directory, record.file_name())
        };
        assert!(machine.made_durable.iter().any(|path| path.contains("attempt-1")));
        assert_eq!(
            AttemptRecord::read_back(machine.files.get(&at(&first)).unwrap()).unwrap(),
            first,
            "a second attempt rewrote the first"
        );
        assert_eq!(
            AttemptRecord::read_back(machine.files.get(&at(&second)).unwrap()).unwrap(),
            second
        );
    }

    #[test]
    fn l7_holds_for_every_path_this_document_composes_for_a_job() {
        // *"Nothing but what a render writes ever sits inside a job output
        // folder"* — not the manifest, not the attempt records, not the staging.
        let layout = a_layout();
        let job = a_job();
        let paths = AJobsPaths::composed(&layout, &job, Some("Kitchen")).unwrap();
        assert!(layout.is_inside_a_job_output_folder(&format!("{}/x", paths.job_output_folder)));
        for path in [&paths.job_record_directory, &paths.staging_directory] {
            assert!(
                !layout.is_inside_a_job_output_folder(path),
                "{path} sits inside a job output folder"
            );
        }
    }

    #[test]
    fn every_path_a_job_gets_is_under_something_the_layout_owns() {
        // N2, asked of all three.
        let layout = a_layout();
        let job = a_job();
        let paths = AJobsPaths::composed(&layout, &job, Some("Kitchen")).unwrap();
        for path in [
            &paths.job_output_folder,
            &paths.job_record_directory,
            &paths.staging_directory,
        ] {
            assert!(layout.is_under_something_this_layout_owns(path));
        }
    }

    #[tokio::test]
    async fn the_folder_the_manifest_names_is_the_folder_that_was_made() {
        // **The round's sharpest fault.** The placement is handed both the
        // paths it creates and the manifest it publishes, and the two used to
        // be composed separately with nothing comparing them. The render worker
        // reads the manifest (M2) and points the redirect at the folder it
        // names; Blender makes a folder that is not there without a word; the
        // folder Smoozy made stays empty, the completeness statement reports
        // every expected file absent, and the carry-on re-renders the whole
        // range. **The render worked and the user is told twice that it did
        // not.**
        let layout = a_layout();
        let job = a_job();
        for label in [None, Some("Kitchen")] {
            let paths = AJobsPaths::composed(&layout, &job, label).unwrap();
            let manifest = crate::publication::testing::a_manifest(&job, &paths);
            let mut machine = AMachine::with_the_areas_of(&layout);
            assert_eq!(
                place_a_jobs_material(&mut machine, &paths, &manifest).await,
                HowThePlacementEnded::ItLanded
            );
            let named = manifest.job_output_folder.the_path_it_names().expect("named at submission");
            assert!(
                machine.directories.contains(named),
                "the manifest names a folder the placement never made: {named}"
            );
        }
    }

    #[test]
    fn the_budget_is_checked_before_any_path_is_used() {
        // **N8: the check is made before any path is used**, and N6 says the
        // cause is Smoozy's own arithmetic rather than the machine's words. A
        // root that fits on its own, under which a job folder's whole path does
        // not: the composition refuses and the machine is never asked.
        let deep = format!("/{}", vec!["segment"; 508].join("/"));
        assert!(deep.len() < names::PATH_BUDGET_BYTES);
        let layout = TheLayout::of(
            MachineRoot::chosen(&deep).unwrap(),
            WhereThePackageAreaSits::UnderTheRoot,
        );
        let refused = AJobsPaths::composed(&layout, &a_job(), None);
        assert!(matches!(
            refused,
            Err(WhyThePathIsRefused::TheWholePathIsOverTheBudget { .. })
        ));
    }

    #[test]
    fn n8s_own_words_hold_the_manifests_path_and_not_only_the_folder_it_sits_in() {
        // **N8: *"checking the folder alone would pass a destination whose
        // files then fail at open."*** Three folders used to be checked and the
        // file inside them was not, so a destination where every folder fits
        // and `manifest.json` does not was composed, handed out, and refused by
        // the machine at open — with the cause arriving as the machine's own
        // words rather than as Smoozy's arithmetic (N6).
        let job = a_job();
        let record_name = names::job_record_directory_name(&job);
        // The manifest's own path one byte over the budget, with the staging
        // directory it sits in comfortably inside it.
        let root = a_root_of_exactly(names::PATH_BUDGET_BYTES - record_name.len() - 34);
        let layout = TheLayout::of(
            MachineRoot::chosen(&root).unwrap(),
            WhereThePackageAreaSits::UnderTheRoot,
        );
        let refused = AJobsPaths::composed(&layout, &job, None);
        assert!(
            matches!(
                refused,
                Err(WhyThePathIsRefused::TheWholePathIsOverTheBudget {
                    it_is_this_many_bytes
                }) if it_is_this_many_bytes == names::PATH_BUDGET_BYTES + 1
            ),
            "the folders fit and the file inside them does not: {refused:?}"
        );
    }

    #[tokio::test]
    async fn an_attempt_records_own_two_paths_are_checked_before_the_machine_is_asked() {
        // The same rule, at the one place the name is known: it carries the
        // attempt's number, so no earlier composition can measure it.
        let job = a_job();
        let record_name = names::job_record_directory_name(&job);
        // Every path `composed` checks fits exactly; the attempt record's
        // staged path is one byte longer than the manifest's.
        let root = a_root_of_exactly(names::PATH_BUDGET_BYTES - record_name.len() - 35);
        let layout = TheLayout::of(
            MachineRoot::chosen(&root).unwrap(),
            WhereThePackageAreaSits::UnderTheRoot,
        );
        let paths = AJobsPaths::composed(&layout, &job, None).expect("every folder and the manifest fit");
        let mut machine = AMachine::with_the_areas_of(&layout);
        let record = AttemptRecord {
            number: 1,
            frame_list_in_the_order_it_was_written: vec![1],
            map_in_force: crate::attempts::TheMapInForce::NoneWasComposedForThisAttempt,
            file_identity_at_this_attempt: crate::publication::testing::a_manifest(&job, &paths)
                .file_identity_at_submission,
            the_file_changed_before_this_attempt: false,
            a_frame_an_earlier_attempt_wrote_was_kept: false,
        };
        assert!(matches!(
            write_an_attempt_record(&mut machine, &paths, &record).await,
            Err(WhyTheAttemptRecordWasNotWritten::ThePathIsRefused(
                WhyThePathIsRefused::TheWholePathIsOverTheBudget { .. }
            ))
        ));
        // **And the machine was never asked**, which is N6's whole point.
        assert!(machine.files.is_empty());
    }
}
