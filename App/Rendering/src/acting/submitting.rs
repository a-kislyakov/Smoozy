//! **§4.1 to §4.4 performed on the desk — the gate, one scene-job's ordered
//! unit placed, the act's units serialised against one machine, and the
//! cleanup debt taken at a later connection.**
//!
//! # S4's ordered unit, and why the order is the whole of it
//!
//! **The remap material is placed, then the job's material is published, then
//! the job record is written — and where an earlier step fails, the later is
//! not attempted, naming what stopped it.** Each unit is a single
//! publication; the chain's ordering is `State, identity and recovery` §9.3's
//! and nothing in it is atomic by care.
//!
//! **A job is in the line exactly when its job record exists** (`Queue &
//! History` Q3). So a published material set with no job record is never a
//! job — and its removal is S4b's recorded cleanup debt, because no sweep
//! carries the case: `Job files and outputs` M7's sweep reaches only a
//! submission that stopped **before** that document's M5 publication, and here
//! the material **was** published.
//!
//! # S4a, and the fault it exists against
//!
//! **Submission units against one machine are serialised at the desktop, and
//! the serialisation is what fixes the line's order.** Without it three jobs
//! submitted in succession join in the order their remap material and
//! manifests finished publishing — **a function of each job's size and never
//! of the order the user acted** — against the spec's own *a new job joins the
//! end of its machine's line*.
//!
//! **A unit ending refused holds nothing back** (S8): the failure is per
//! scene-job and the next unit proceeds.
//!
//! # What is reused rather than written again
//!
//! The material's publication is `Job files and outputs`' own
//! `place_a_jobs_material`, over that document's `TheMachinesDirectories`
//! seam; the remap material's placement is `Dependency Upload & Remap` M10's;
//! the identities are [`crate::submission::issue_the_identities`]'s; the
//! manifest is [`crate::submission::compose_the_manifest`]'s. **This module
//! sequences them and writes none of them again.**
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Job files and outputs` M5, M7, N10.
//! `Dependency Upload & Remap` M10, M11. `Queue & History` Q3, Q7. `Server
//! protocol and transfers` C4, C8, S28. `State, identity and recovery` D5.

use smoozy_job_files::acting::TheMachinesDirectories;
use smoozy_job_files::manifest::SubmissionManifest;
use smoozy_job_files::publication::{place_a_jobs_material, AJobsPaths, HowThePlacementEnded};
use smoozy_state::identities::Identity;

use crate::submission::{may_the_next_unit_begin, THE_ORDERED_UNIT};

/// Placing the remap material, behind a trait — **`Dependency Upload & Remap`
/// M10 and M11's, and it goes first.**
///
/// **A submission composed without it renders the artist's own recorded
/// locations in force and finishes at success**, which is why L30 refuses to
/// render without it on the far side and why it is the first unit here.
pub trait PlacesTheRemapMaterial {
    fn place(
        &mut self,
        job: &Identity,
    ) -> impl std::future::Future<Output = Result<(), String>> + Send;
}

/// Writing the job record, behind a trait — **the supervisor's own act**
/// (`Rented-server runtime` S1), asked for over the connection.
///
/// **`Queue & History` Q3: a job is in the line exactly when its job record
/// exists**, and the supervisor writes the record and the per-job note in one
/// act and journals it. **Nothing on the desk writes it**, which is why this
/// is an ask and not a write.
pub trait AsksTheSupervisorForTheJobRecord {
    fn write_the_job_record(
        &mut self,
        job: &Identity,
    ) -> impl std::future::Future<Output = Result<(), String>> + Send;
}

/// How one scene-job's ordered unit ended.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HowTheUnitEnded {
    /// All three landed and the job is in the line.
    ItIsInTheLine,
    /// **A step failed and the later ones were not attempted**, naming what
    /// stopped it and which of [`THE_ORDERED_UNIT`]'s three it was.
    ItStoppedAt {
        step: &'static str,
        the_later_were_not_attempted: Vec<&'static str>,
        why: String,
    },
}

impl HowTheUnitEnded {
    fn stopped(at: usize, why: String) -> HowTheUnitEnded {
        HowTheUnitEnded::ItStoppedAt {
            step: THE_ORDERED_UNIT[at],
            the_later_were_not_attempted: THE_ORDERED_UNIT[at + 1..].to_vec(),
            why,
        }
    }

    /// **S4b's case: the material was published and the job record was
    /// not.** A published material set with no job record is never a job, and
    /// its removal is the act's recorded cleanup debt.
    pub fn left_material_that_is_never_a_job(&self) -> bool {
        matches!(
            self,
            HowTheUnitEnded::ItStoppedAt {
                step: "the job record written",
                ..
            }
        )
    }
}

/// **S4, performed: one scene-job's ordered unit, in order.**
pub async fn place_one_scene_jobs_unit(
    remap: &mut impl PlacesTheRemapMaterial,
    machine: &mut impl TheMachinesDirectories,
    supervisor: &mut impl AsksTheSupervisorForTheJobRecord,
    job: &Identity,
    paths: &AJobsPaths,
    manifest: &SubmissionManifest,
) -> HowTheUnitEnded {
    // 1. **The remap material, placed through the transfer contract before
    //    the submission material** (`Dependency Upload & Remap` M10, M11).
    if let Err(why) = remap.place(job).await {
        return HowTheUnitEnded::stopped(0, why);
    }
    // 2. **The job's whole submission material, staged in one directory and
    //    published by a single rename** (`Job files and outputs` M5), with
    //    the job output folder created before it.
    if let HowThePlacementEnded::ItStoppedAt(step, said) =
        place_a_jobs_material(machine, paths, manifest).await
    {
        return HowTheUnitEnded::stopped(1, format!("{step:?}: {said:?}"));
    }
    // 3. **The supervisor writes the job record and the per-job note in one
    //    act and journals it**, and the job is in the line exactly then.
    if let Err(why) = supervisor.write_the_job_record(job).await {
        return HowTheUnitEnded::stopped(2, why);
    }
    HowTheUnitEnded::ItIsInTheLine
}

/// **S4a's serialisation, as the shape of the act rather than as a rule
/// somebody keeps.**
///
/// The units of one act against one machine are attempted **one at a time, in
/// the outcome record's enumerated order**, and the next begins only when the
/// previous unit's submission command has a recorded outcome the desktop has
/// read. **A unit ending refused holds nothing back** — S8's failure is per
/// scene-job — so the walk carries on to the next either way.
///
/// **`Queue & History` Q7 keys the line by the order the supervisor writes
/// job records**, and this is what makes that order the user's own.
pub struct TheActsUnits<'a> {
    /// The scene-jobs of one act, in the outcome record's enumerated order.
    pub in_the_enumerated_order: &'a [Identity],
}

/// What one act's walk produced, per scene-job in the order it was attempted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheActDid {
    pub per_scene_job: Vec<(Identity, HowTheUnitEnded)>,
}

impl WhatTheActDid {
    /// **S4b: what the act owes as a cleanup debt** — the scene-jobs whose
    /// material published and whose job record never did.
    pub fn the_cleanup_debt_is_owed_for(&self) -> Vec<&Identity> {
        self.per_scene_job
            .iter()
            .filter(|(_, ended)| ended.left_material_that_is_never_a_job())
            .map(|(job, _)| job)
            .collect()
    }
}

/// **S4a, performed.**
///
/// `place` is [`place_one_scene_jobs_unit`] bound to the machine's own seams
/// by the caller; `has_a_recorded_outcome` is `Server protocol and transfers`
/// C8's answer about the previous unit's submission command, which is what
/// [`may_the_next_unit_begin`] tests.
pub async fn walk_the_acts_units<P, F, R>(
    units: TheActsUnits<'_>,
    mut place: P,
    mut the_previous_units_record: R,
) -> WhatTheActDid
where
    P: FnMut(Identity) -> F,
    F: std::future::Future<Output = HowTheUnitEnded>,
    R: FnMut(&Identity) -> Option<smoozy_state::outcomes::Recorded>,
{
    let mut done: Vec<(Identity, HowTheUnitEnded)> = Vec::new();
    for job in units.in_the_enumerated_order {
        if let Some((previous, _)) = done.last() {
            // **The next unit begins only when the previous unit's
            // submission command has a recorded outcome the desktop has
            // read.** Without this the line's order is a function of each
            // job's size.
            let record = the_previous_units_record(previous);
            if !may_the_next_unit_begin(record.as_ref()) {
                break;
            }
        }
        let ended = place(job.clone()).await;
        done.push((job.clone(), ended));
    }
    WhatTheActDid {
        per_scene_job: done,
    }
}

/// **S4b: what the debt names, and it is composed from the desktop's own
/// record and never from enumerating the machine.**
///
/// `Server protocol and transfers` S28 forbids removing a path on the
/// strength of no record naming it, so every path in the debt is one the
/// desktop's own record composed at submission — the material set at the job
/// record directory's path, the remap material in the asset area, and the job
/// output folder where it is empty.
///
/// **The removal is safe because the record cannot appear later**: C8's *no
/// record of that identity* is answered from command records written durably
/// before acting (`Server protocol and transfers` C4), so a submission
/// command the machine never recorded is one it will never act on.
pub fn the_debts_paths(paths: &AJobsPaths, the_remap_materials_path: &str) -> Vec<String> {
    vec![
        paths.job_record_directory.clone(),
        the_remap_materials_path.to_string(),
        paths.job_output_folder.clone(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::testing::a_manifest;
    use crate::material::testing::a_job;
    use smoozy_job_files::acting::WhatTheMachineSaid;
    use smoozy_job_files::layout::{MachineRoot, TheLayout, WhereThePackageAreaSits};
    use std::collections::BTreeMap;

    /// A machine that answers, so the ordered unit is walked for real.
    #[derive(Default)]
    struct AMachineThat {
        refuse_directory: bool,
        refuse_write: bool,
        refuse_rename: bool,
        made: Vec<String>,
        wrote: BTreeMap<String, Vec<u8>>,
        renamed: Vec<(String, String)>,
    }

    impl TheMachinesDirectories for AMachineThat {
        async fn make_one_directory(&mut self, path: &str) -> Result<(), WhatTheMachineSaid> {
            if self.refuse_directory {
                return Err(WhatTheMachineSaid::ItRefused);
            }
            self.made.push(path.to_string());
            Ok(())
        }
        async fn write_a_whole_file_durably(
            &mut self,
            path: &str,
            bytes: &[u8],
        ) -> Result<(), WhatTheMachineSaid> {
            if self.refuse_write {
                return Err(WhatTheMachineSaid::ItRefused);
            }
            self.wrote.insert(path.to_string(), bytes.to_vec());
            Ok(())
        }
        async fn read_a_whole_file(&mut self, path: &str) -> Result<Vec<u8>, WhatTheMachineSaid> {
            self.wrote
                .get(path)
                .cloned()
                .ok_or(WhatTheMachineSaid::ThePathIsNotThere)
        }
        async fn rename_without_replacing(
            &mut self,
            from: &str,
            to: &str,
        ) -> Result<(), WhatTheMachineSaid> {
            if self.refuse_rename {
                return Err(WhatTheMachineSaid::ItRefused);
            }
            self.renamed.push((from.to_string(), to.to_string()));
            Ok(())
        }
        async fn remove_one_file(&mut self, _path: &str) -> Result<(), WhatTheMachineSaid> {
            Ok(())
        }
        async fn remove_one_empty_directory(
            &mut self,
            _path: &str,
        ) -> Result<(), WhatTheMachineSaid> {
            Ok(())
        }
        async fn list_one_directory(
            &mut self,
            _path: &str,
        ) -> Result<Vec<Vec<u8>>, WhatTheMachineSaid> {
            Ok(Vec::new())
        }
    }

    #[derive(Default)]
    struct ARemapThat {
        refuses: bool,
        placed: usize,
    }
    impl PlacesTheRemapMaterial for ARemapThat {
        async fn place(&mut self, _job: &Identity) -> Result<(), String> {
            if self.refuses {
                return Err("the transfer was refused".to_string());
            }
            self.placed += 1;
            Ok(())
        }
    }

    #[derive(Default)]
    struct ASupervisorThat {
        refuses: bool,
        wrote: usize,
    }
    impl AsksTheSupervisorForTheJobRecord for ASupervisorThat {
        async fn write_the_job_record(&mut self, _job: &Identity) -> Result<(), String> {
            if self.refuses {
                return Err("the supervisor did not answer".to_string());
            }
            self.wrote += 1;
            Ok(())
        }
    }

    fn paths_for(job: &Identity) -> AJobsPaths {
        let layout = TheLayout::of(
            MachineRoot::chosen("/ephemeral/smoozy").unwrap(),
            WhereThePackageAreaSits::UnderTheRoot,
        );
        AJobsPaths::composed(&layout, job, Some("Kitchen")).unwrap()
    }

    fn run<F: std::future::Future>(future: F) -> F::Output {
        // The units are asynchronous because the transfers are; nothing here
        // needs a runtime beyond driving one future to its end.
        futures_lite_block_on(future)
    }

    /// A one-line executor, so these tests need no runtime of their own and
    /// the workspace gains no dependency for them.
    fn futures_lite_block_on<F: std::future::Future>(mut future: F) -> F::Output {
        use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
        fn no_op(_: *const ()) {}
        fn clone(data: *const ()) -> RawWaker {
            RawWaker::new(data, &VTABLE)
        }
        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, no_op, no_op, no_op);
        let waker = unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) };
        let mut context = Context::from_waker(&waker);
        let mut future = unsafe { std::pin::Pin::new_unchecked(&mut future) };
        loop {
            match future.as_mut().poll(&mut context) {
                Poll::Ready(value) => return value,
                Poll::Pending => std::thread::yield_now(),
            }
        }
    }

    /// **The three units land in order and the job is in the line.**
    #[test]
    fn the_ordered_unit_places_the_remap_then_the_material_then_the_record() {
        let job = a_job();
        let manifest = a_manifest(&job, "5.2.0", "Kitchen");
        let paths = paths_for(&job);
        let mut remap = ARemapThat::default();
        let mut machine = AMachineThat::default();
        let mut supervisor = ASupervisorThat::default();
        let ended = run(place_one_scene_jobs_unit(
            &mut remap,
            &mut machine,
            &mut supervisor,
            &job,
            &paths,
            &manifest,
        ));
        assert_eq!(ended, HowTheUnitEnded::ItIsInTheLine);
        assert_eq!(remap.placed, 1);
        assert_eq!(supervisor.wrote, 1);
        assert_eq!(machine.renamed.len(), 1, "one publication");
    }

    /// **Where an earlier step fails, the later is not attempted, naming what
    /// stopped it.** The fault is introduced one unit at a time.
    #[test]
    fn a_failed_earlier_unit_stops_the_later_ones_and_names_what_stopped_it() {
        let job = a_job();
        let manifest = a_manifest(&job, "5.2.0", "Kitchen");
        let paths = paths_for(&job);

        // The remap material refused: nothing of the material is published
        // and no job record is asked for.
        let mut remap = ARemapThat {
            refuses: true,
            placed: 0,
        };
        let mut machine = AMachineThat::default();
        let mut supervisor = ASupervisorThat::default();
        let ended = run(place_one_scene_jobs_unit(
            &mut remap,
            &mut machine,
            &mut supervisor,
            &job,
            &paths,
            &manifest,
        ));
        let HowTheUnitEnded::ItStoppedAt {
            step,
            the_later_were_not_attempted,
            ..
        } = &ended
        else {
            panic!("{ended:?}");
        };
        assert_eq!(*step, THE_ORDERED_UNIT[0]);
        assert_eq!(the_later_were_not_attempted.len(), 2);
        assert!(machine.made.is_empty());
        assert_eq!(supervisor.wrote, 0);
        assert!(!ended.left_material_that_is_never_a_job());

        // The publication refused: no job record is asked for, and nothing
        // was published, so there is no cleanup debt either.
        let mut remap = ARemapThat::default();
        let mut machine = AMachineThat {
            refuse_rename: true,
            ..AMachineThat::default()
        };
        let mut supervisor = ASupervisorThat::default();
        let ended = run(place_one_scene_jobs_unit(
            &mut remap,
            &mut machine,
            &mut supervisor,
            &job,
            &paths,
            &manifest,
        ));
        assert!(matches!(ended, HowTheUnitEnded::ItStoppedAt { .. }));
        assert_eq!(supervisor.wrote, 0);
        assert!(!ended.left_material_that_is_never_a_job());
    }

    /// **S4b: material published and no job record written is never a job**,
    /// and the debt is taken for exactly that case and no other.
    #[test]
    fn a_unit_that_published_and_never_got_its_record_owes_the_cleanup_debt() {
        let job = a_job();
        let manifest = a_manifest(&job, "5.2.0", "Kitchen");
        let paths = paths_for(&job);
        let mut remap = ARemapThat::default();
        let mut machine = AMachineThat::default();
        let mut supervisor = ASupervisorThat {
            refuses: true,
            wrote: 0,
        };
        let ended = run(place_one_scene_jobs_unit(
            &mut remap,
            &mut machine,
            &mut supervisor,
            &job,
            &paths,
            &manifest,
        ));
        assert!(ended.left_material_that_is_never_a_job());
        let did = WhatTheActDid {
            per_scene_job: vec![(job.clone(), ended)],
        };
        assert_eq!(did.the_cleanup_debt_is_owed_for(), vec![&job]);
        // **The debt's paths are the desktop's own and never an enumeration
        // of the machine.**
        let owed = the_debts_paths(&paths, "/ephemeral/smoozy/assets/remaps/j1");
        assert_eq!(owed.len(), 3);
        assert!(owed.contains(&paths.job_output_folder));
    }

    fn a_record_whose_units(landed: bool) -> smoozy_state::outcomes::Recorded {
        use smoozy_state::outcomes::{Recorded, UnitOutcome};
        Recorded {
            identity: a_job(),
            action: "submitting one scene-job".to_string(),
            units: THE_ORDERED_UNIT
                .iter()
                .map(|unit| (unit.to_string(), landed.then_some(UnitOutcome::Done)))
                .collect(),
        }
    }

    /// **S4a: the next unit waits on the previous unit's recorded
    /// outcome.** The fault is introduced by the previous unit's outcome not
    /// having landed, and the walk must stop rather than racing ahead —
    /// which is what fixes the line's order to the user's own rather than to
    /// each job's size.
    #[test]
    fn the_walk_waits_on_the_previous_units_recorded_outcome() {
        let jobs = vec![a_job(), a_job(), a_job()];
        let did = run(walk_the_acts_units(
            TheActsUnits {
                in_the_enumerated_order: &jobs,
            },
            |_| async move { HowTheUnitEnded::ItIsInTheLine },
            |_| Some(a_record_whose_units(false)),
        ));
        assert_eq!(
            did.per_scene_job.len(),
            1,
            "nothing after the first was attempted"
        );
    }

    /// **A unit ending refused holds nothing back** — S8's failure is per
    /// scene-job and the next unit proceeds once the previous unit's outcome
    /// has landed.
    #[test]
    fn a_refused_unit_does_not_hold_the_next_one_back() {
        let jobs = vec![a_job(), a_job()];
        let did = run(walk_the_acts_units(
            TheActsUnits {
                in_the_enumerated_order: &jobs,
            },
            |_| async move { HowTheUnitEnded::stopped(0, "the transfer was refused".to_string()) },
            |_| Some(a_record_whose_units(true)),
        ));
        assert_eq!(did.per_scene_job.len(), 2, "the second was attempted");
        assert!(did.the_cleanup_debt_is_owed_for().is_empty());
    }

}
