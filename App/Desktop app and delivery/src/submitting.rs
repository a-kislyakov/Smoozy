//! **The submitting driver: the Submit button's whole act, from the page's
//! composition to the machine** — step 10c piece 6.
//!
//! **Everything this file calls is built elsewhere, and it writes none of it
//! again.** The gate, the identities, the manifest, the reporting field, the
//! engine gate, the ordered unit and the walk over an act's units are
//! `Rendering`'s (`App/Rendering/src/submission.rs` and
//! `acting/submitting.rs`). The provisional record and its resolution are
//! `Queue & History`'s (`acting/submission.rs`). The remap material and its
//! placement are `Dependency Upload & Remap`'s M10 and M11; the material's
//! publication is `Job files and outputs` M5; the version list is `Server
//! Setup`'s search driven by `Render Settings`' own ask. **What is here is the
//! joining**: those seams given real ends over `Server protocol and
//! transfers` T5's per-server point (`src/servers/sessions.rs`), in the order
//! the clauses give.
//!
//! # The one sender — the runner's R3 for step 10c
//!
//! **Two traits describe the one act of handing a job to the machine**:
//! `Rendering`'s `AsksTheSupervisorForTheJobRecord`, the third step of
//! `Rendering` S4's ordered unit, and `Queue & History`'s `SendsTheSubmission`,
//! which hands the submission over and asks, after a lost acknowledgement,
//! whether the machine holds the job (`State, identity and recovery` §12 rule
//! 4). **[`TheOneSender`] implements both, over one route**, with step 10c
//! piece 1's request names and never a second spelling of one.
//!
//! **`Queue & History`'s `submit` is what orders the two halves**: the
//! provisional local record written first, durable, and only then the
//! submission sent — and the submission it sends is `Rendering`'s whole
//! ordered unit, because *"the submission's own atomicity is `Rendering`'s"*.
//! **So the provisional record is down before any byte reaches the machine**,
//! which is `Rendering` S3 and `State, identity and recovery` §7 step 2.
//!
//! # What the machine cannot answer yet, said plainly
//!
//! **The machine's program does not take a job.** Step 10c piece 5 builds the
//! supervisor's answer to *the job record is written* and to *does the machine
//! hold this job*; until then the desk build refuses both by name. **So the
//! last step of the unit is proved against a stand-in** (this file's tests and
//! `tests/a_job_is_submitted_through_the_servers_own_point.rs`), and the
//! running application meets the refusal and says so. **Two things follow
//! from there being no answer shape yet**, each written at its own site:
//!
//! - **An acknowledgement is any answer that does not refuse the request by
//!   name.** Piece 5 names the answer's fields and may narrow it.
//! - ~~**The hold ask cannot be read.**~~ **It is read, since step 10c's
//!   eighth-pass fix piece 4 (2026-09-27)**, by the one reader
//!   `src/servers/the_machines_work.rs` already had for it: *present* is in
//!   the line, *absent* is not submitted with `Rendering` S4b's debt, and only
//!   the machine's *cannot say* — or no answer at all — keeps the provisional
//!   record, the direction `State, identity and recovery` V1 step 2's third
//!   branch rules safe. *(Piece 5 did build the machine's answer; this
//!   paragraph's older sentences describe the file before it did.)*
//!
//! **`Rendering` S13's second job is sent, held** — the whole range, its first
//! frame included, since decision C (2026-10-02). The hold instruction rides
//! the job record request in the two fields step 10c piece 5 named, and the
//! machine places the hold in the same act. ~~The second job was refused
//! before anything of it was sent~~, on the ground that no field existed for
//! the hold — true when step 10c piece 6 wrote it, false from the day piece 5
//! named the fields, and never noticed until the eleventh pass on Hyperstack
//! (fault 4), where *Render one frame first* rendered one frame and ended.
//! Fixed by step 10c's eleventh-pass fix piece 8.
//!
//! # How a clause marker is written in this crate
//!
//! **This file keeps no header list** and writes every other document's name
//! at its own site, because `Rendering`, `Queue & History` and this crate's
//! own plan share nearly every letter. **A bare marker below means a clause of
//! this crate's own plan, `Preparation/Architecture/Desktop app and
//! delivery.md`.**

use std::collections::BTreeMap;
use std::future::Future;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant, SystemTime};

use smoozy_dependencies::acting::placing::{
    arrange_the_materials_placement, the_ordered_unit, TheOrderedUnit,
};
use smoozy_dependencies::bytes::{MachineLocation, RecordedLocation};
use smoozy_dependencies::record::{MachineKey, ResolvedSet};
use smoozy_dependencies::remap::RemapMaterial;
use smoozy_download::arrangement::TheNarrowing;
use smoozy_job_files::acting::{TheMachinesDirectories, WhatTheMachineSaid as WhatTheFolderSaid};
use smoozy_job_files::layout::TheLayout;
use smoozy_job_files::manifest::{
    CarriedAndNotInterpretedHere, SubmissionManifest, TheBlenderToRenderBy,
    TheCardMakeAtSubmission, TheRecordedReading,
};
use smoozy_job_files::publication::AJobsPaths;
use smoozy_queue_history::acting::submission::{
    resolve, submit, AProvisionalRecord, HowItResolved, SendsTheSubmission, WhatTheSubmissionGave,
    WhatTheSubmissionLeft, WritesTheProvisionalRecord,
};
use smoozy_queue_history::record::{
    AnswerOrigin, DependencyCheckAtSubmission, LocalJobRecord, Standing, TheBlenderTheEntryKeeps,
    TheDeviceAsk, TheServerAsItStood, WhatTheNoteCarried,
};
use smoozy_render_settings::file_half::{BlenderIdentity, RecordedReading};
use smoozy_render_settings::frames::FrameForm;
use smoozy_render_settings::payload::Payload;
use smoozy_rendering::acting::submitting::{
    place_one_scene_jobs_unit, walk_the_acts_units,
    AsksTheSupervisorForTheJobRecord, HowTheUnitEnded, PlacesTheRemapMaterial, TheActsUnits,
};
use smoozy_rendering::submission::{
    compose_the_manifest, issue_the_identities, the_engine_gate,
    the_group_identity, ManifestInputs, TheGate, TheIdentities, WhatTheDesktopHoldsAboutTheBuild,
    THE_ORDERED_UNIT,
};
use smoozy_security_and_licence::plaintext::Plaintext;
use smoozy_server_protocol::asking_the_supervisor::WhatCameBack;
use smoozy_server_protocol::framing::{Fields, Request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS};
use smoozy_queue_history::record::WhatTheMachineSaidAboutTheJob;
use smoozy_server_protocol::requests::{
    THE_HELD_BEHIND_FIELD, THE_HOLD_REASON_FIELD, THE_JOB_FIELD, THE_JOB_RECORD_IS_WRITTEN_REQUEST,
    THE_LABEL_FIELD,
};
use smoozy_server_protocol::subsystem::client::Subsystem;
use smoozy_server_protocol::transfers::moving::{carry_out, NobodyIsWatching, Transfer};
use smoozy_server_protocol::transfers::publication::Intent;
use smoozy_server_protocol::transfers::{Direction, Ending, SourceReading, TransferRecord};
use smoozy_server_protocol::transport::ssh::CommandFailure;
use smoozy_server_protocol::transport::{
    CarriesAValueOnACommandsStandardInput, CommandResult, CommandString,
};
use smoozy_server_setup::readings::{AListedThing, TheMachinesFileSurface, WhatTheMachineSaid};
use smoozy_server_setup::wiring::TheMachinesFiles;
use smoozy_servers::readings::CommandChannel;
use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_state::observed::{Clock, Reading};
use smoozy_state::outcomes::{Keeps, Recorded, UnitOutcome};

use crate::downloading::{
    the_arrangement_is_made_at_submission, TheRunsTransfers, WhatTheSubmissionChose,
    THE_INACTIVITY_DEADLINE,
};
use crate::record::dependency_records::TheDependencyRecord;
use crate::record::download_records::DownloadArrangements;
use crate::record::job_records::TheSubmissionsWriter;
use crate::record::outcome_records::OutcomeRecords;
use crate::scheduling::TheChannelsDraw;
use crate::servers::sessions::{AHeldConnection, ALentChannel, NotLent, ThisServersSessions};
use crate::servers::the_machines_work::{what_the_machine_says_about, AsksTheSupervisor};
use crate::transfers::ThePlatformFileAdapter;
use crate::uploading::TheRecordAnUploadWrites;

/// **How long the driver waits before asking the desktop point again**, where
/// the point had no room for one of the submission's two transfers.
///
/// **R13: a refusal is a reason to wait and never a reason to fail**, and the
/// point has no way to call a caller back. **The figure is this build's**, the
/// one `src/uploading.rs` asks again at, and no promise rests on it.
const THE_PAUSE_BEFORE_ASKING_AGAIN: Duration = Duration::from_millis(100);

/// **The action a scene-job's outcome record is written under** — `State,
/// identity and recovery` §9.4, and `Rendering` §7's first row: the act's
/// scene-jobs, each with `Rendering` S4's three units.
pub const ONE_SCENE_JOBS_SUBMISSION: &str = "submitting one scene-job";

/// **The action `Rendering` S4b's debt is written under**: the removal a dead
/// unit's leftovers are owed, taken by a new action at a later connection.
pub const THE_CLEANUP_DEBT_OF_A_SUBMISSION: &str =
    "removing what a submission left that never became a job";

// ════════════════════════════════════════════════════════════════════════════
// Waiting on an asynchronous seam from a synchronous one
// ════════════════════════════════════════════════════════════════════════════

/// **`Queue & History`'s seam is synchronous and every machine act is not**, so
/// the one sender waits on its asks here: a thread of its own, a runtime made
/// for the one wait and dropped with it. **The point lends from the period's
/// own thread** (`Desktop app and delivery` P5), so nothing here holds the
/// connection; it only waits for an answer.
///
/// **The future is made on the waiting thread**, so what it borrows need not
/// cross threads, and a caller already inside a runtime is never asked to
/// block it.
fn waited_for<M, F>(make: M) -> F::Output
where
    M: FnOnce() -> F + Send,
    F: Future,
    F::Output: Send,
{
    std::thread::scope(|scope| {
        scope
            .spawn(move || {
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("a runtime for one wait");
                runtime.block_on(make())
            })
            .join()
            .expect("the wait did not panic")
    })
}

// ════════════════════════════════════════════════════════════════════════════
// The machine's own seams, over the per-server point
// ════════════════════════════════════════════════════════════════════════════

/// **`Servers`' command seam and `Server Setup`'s file seam, over `Server
/// protocol and transfers` T5's per-server point.** Each command is one
/// command at the point, and each file operation one reading on the reading
/// session — **so a command may close the reading session between two of
/// them**, which is the point's own rule and why no operation here holds a
/// handle across two asks.
///
/// **It is what `Server Setup`'s search, `Render Settings`' version ask and
/// its four-value read are run over.** Those three were built with a real end
/// in the tests only; this is the first one in the running application.
pub struct OnThisServersPoint<'s, C: AHeldConnection> {
    pub sessions: &'s ThisServersSessions<C>,
}

fn as_a_command_failure(not_lent: NotLent) -> CommandFailure {
    match not_lent {
        // **`Server protocol and transfers` T2: a machine that would not open a
        // session** — the one failure the transport itself says that way.
        NotLent::TheMachineAllowsNoSession => CommandFailure::NoChannelCouldBeOpened,
        _ => CommandFailure::TheRequestDidNotReachTheMachine,
    }
}

impl<C: AHeldConnection> CommandChannel for OnThisServersPoint<'_, C> {
    async fn run(&mut self, command: CommandString) -> Result<CommandResult, CommandFailure> {
        self.sessions
            .a_command(command, None)
            .await
            .map_err(as_a_command_failure)
    }
}

impl<C: AHeldConnection> CarriesAValueOnACommandsStandardInput for OnThisServersPoint<'_, C> {
    /// **No route for a secret here, and none is reached.** The one value this
    /// seam carries is a machine password for `Server Setup`'s elevated retry
    /// (E2), and nothing this file drives asks elevated — `Render Settings`'
    /// version ask says *"It never asks elevated"*. **A caller that did would
    /// be refused rather than have its secret written to a command's input**
    /// by a route nobody wrote down.
    async fn run_carrying_a_value_on_its_standard_input(
        &mut self,
        _command: CommandString,
        _value: Plaintext,
    ) -> Result<CommandResult, CommandFailure> {
        Err(CommandFailure::TheRequestDidNotReachTheMachine)
    }
}

impl<C: AHeldConnection> OnThisServersPoint<'_, C> {
    /// One of `Server Setup`'s file operations, as one reading.
    async fn one_file_act<T, A>(&self, act: A) -> Result<T, WhatTheMachineSaid>
    where
        T: Send + 'static,
        A: for<'a> FnOnce(
                TheMachinesFiles<'a, C::Channel>,
            ) -> crate::servers::sessions::ItsOwnFuture<
                'a,
                Result<T, WhatTheMachineSaid>,
            > + Send
            + 'static,
    {
        match self
            .sessions
            .a_reading(true, move |subsystem| {
                Box::pin(async move { Ok(act(TheMachinesFiles::on(subsystem)).await) })
            })
            .await
        {
            Ok(Ok(said)) => said,
            Ok(Err(_)) | Err(_) => Err(WhatTheMachineSaid::TheChannelIsGone),
        }
    }
}

impl<C: AHeldConnection> TheMachinesFileSurface for OnThisServersPoint<'_, C> {
    async fn read_whole(&mut self, path: &str) -> Result<Vec<u8>, WhatTheMachineSaid> {
        let path = path.to_string();
        self.one_file_act(move |mut files| Box::pin(async move { files.read_whole(&path).await }))
            .await
    }

    async fn write_whole(&mut self, path: &str, bytes: &[u8]) -> Result<(), WhatTheMachineSaid> {
        let (path, bytes) = (path.to_string(), bytes.to_vec());
        self.one_file_act(move |mut files| {
            Box::pin(async move { files.write_whole(&path, &bytes).await })
        })
        .await
    }

    async fn replace_by_rename(&mut self, from: &str, to: &str) -> Result<(), WhatTheMachineSaid> {
        let (from, to) = (from.to_string(), to.to_string());
        self.one_file_act(move |mut files| {
            Box::pin(async move { files.replace_by_rename(&from, &to).await })
        })
        .await
    }

    async fn read_link(&mut self, path: &str) -> Result<Vec<u8>, WhatTheMachineSaid> {
        let path = path.to_string();
        self.one_file_act(move |mut files| Box::pin(async move { files.read_link(&path).await }))
            .await
    }

    async fn make_one_directory(&mut self, path: &str) -> Result<(), WhatTheMachineSaid> {
        let path = path.to_string();
        self.one_file_act(move |mut files| {
            Box::pin(async move { files.make_one_directory(&path).await })
        })
        .await
    }

    async fn remove_one_file(&mut self, path: &str) -> Result<(), WhatTheMachineSaid> {
        let path = path.to_string();
        self.one_file_act(move |mut files| {
            Box::pin(async move { files.remove_one_file(&path).await })
        })
        .await
    }

    async fn remove_one_empty_directory(&mut self, path: &str) -> Result<(), WhatTheMachineSaid> {
        let path = path.to_string();
        self.one_file_act(move |mut files| {
            Box::pin(async move { files.remove_one_empty_directory(&path).await })
        })
        .await
    }

    async fn list_one_directory(
        &mut self,
        path: &str,
    ) -> Result<Vec<AListedThing>, WhatTheMachineSaid> {
        let path = path.to_string();
        self.one_file_act(move |mut files| {
            Box::pin(async move { files.list_one_directory(&path).await })
        })
        .await
    }

    async fn plant_a_link(&mut self, target: &str, at: &str) -> Result<(), WhatTheMachineSaid> {
        let (target, at) = (target.to_string(), at.to_string());
        self.one_file_act(move |mut files| {
            Box::pin(async move { files.plant_a_link(&target, &at).await })
        })
        .await
    }
}

/// **`Render Settings` B1's list, asked for at `Server Setup` B12's moment**,
/// over the per-server point. **The ask is `Render Settings`' own and the
/// search is `Server Setup`'s** (`Server Setup` B10 and `Server Setup` B11); this supplies the two seams and
/// nothing else.
///
/// **`smoozys_own_blender_area` is `Server Setup` B9's exact path**, from the
/// layout the server row records, or nothing where no placement is recorded.
///
/// **`the_server` is the local server identifier `sessions` lends for**, and
/// it is what each build's asking-run takes its turn at the one link site by
/// (step 10c's eleventh-pass fix piece 4c): **both searches in the
/// application — the Render Settings page's and the Set up tab's — come
/// through here**, and no two asks on one server are at the site at once.
pub async fn the_blenders_on_the_machine<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    the_server: &str,
    smoozys_own_blender_area: Option<&str>,
) -> smoozy_render_settings::acting::versions::TheVersionListAsk {
    let mut channel = OnThisServersPoint { sessions };
    let mut files = OnThisServersPoint { sessions };
    smoozy_render_settings::acting::versions::ask_for_the_version_list(
        &mut channel,
        &mut files,
        the_server,
        smoozys_own_blender_area,
    )
    .await
}

/// **The `.blend`'s four values as they stand now** — `Render Settings` Q14
/// item 2's reading, which is also `Job files and outputs` M1's *four-value
/// file identity at submission*. The reading is `Render Settings`' own.
///
/// **`the_server` is the local server identifier `sessions` lends for**, and
/// it is what the reading takes its turn at the one link site by (step 10c's
/// eleventh-pass fix piece 4b): **every reader in the application comes
/// through here**, and no two on one server are at the site at once.
pub async fn the_four_values_of<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    the_server: &str,
    the_blend: &str,
) -> smoozy_render_settings::readings::FourValuesNow {
    let mut machine = OnThisServersPoint { sessions };
    smoozy_render_settings::acting::four_values::read_the_four_values(
        &mut machine,
        the_server,
        the_blend,
    )
    .await
}

// ════════════════════════════════════════════════════════════════════════════
// The one sender
// ════════════════════════════════════════════════════════════════════════════

/// **What the supervisor's answer to the job record request came to**, kept
/// by the sender for the driver, because neither seam's return carries it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheSupervisorSaid {
    /// An answer that does not refuse the request — the acknowledgement.
    ItTookIt,
    /// **The package said it does not answer that request**, naming it —
    /// `Server protocol and transfers`' reserved field. **A definite answer**:
    /// the machine did not write the job record.
    ItDoesNotTakeJobs,
    /// **Nothing correlated came back, or the request was never asked** —
    /// `Server protocol and transfers` C17a's silence, which is not yet the
    /// supervisor's. **Whether the machine holds the job is not known.**
    NothingThatCanBeRead,
}

/// **The one sender, over one route** — see this file's head.
///
/// **Made per scene-job**, holding that scene-job's unit, because
/// `Queue & History`'s `send` hands over *the* submission and `Rendering`'s
/// unit is what the submission is.
pub struct TheOneSender<'a, C: AHeldConnection> {
    sessions: &'a ThisServersSessions<C>,
    package_area: &'a str,
    /// **`Rendering` S3's submission command identity, the idempotency key** —
    /// the job record request's own identity (`Server protocol and transfers`
    /// C1).
    the_submission_command: Identity,
    the_unit: TheUnitInHand<'a, C>,
    /// **The label, read from `Queue & History`'s provisional record** when it
    /// is handed over — step 10c piece 1's owed item 5: `Rendering`'s seam
    /// passes the job alone and the request carries the label too.
    the_label: Mutex<Option<String>>,
    /// **`Rendering` S13's hold instruction, on the first-frame check's second
    /// job and on no other** — the two fields ride the job record request
    /// together (`Server protocol and transfers`' `THE_HELD_BEHIND_FIELD` and
    /// `THE_HOLD_REASON_FIELD`, *"both or neither"*).
    the_hold: Option<smoozy_rendering::submission::TheHoldInstruction>,
    what_the_supervisor_said: Mutex<Option<WhatTheSupervisorSaid>>,
    how_the_unit_ended: Mutex<Option<HowTheUnitEnded>>,
}

/// Everything one scene-job's ordered unit needs to reach the machine.
struct TheUnitInHand<'a, C: AHeldConnection> {
    job: Identity,
    paths: AJobsPaths,
    manifest: SubmissionManifest,
    remap: TheRemapPlacement<'a, C>,
    the_run: &'a TheRunsTransfers,
    the_submission_class: &'static str,
}

impl<'a, C: AHeldConnection> TheOneSender<'a, C> {
    /// What the supervisor said, once the unit has asked it.
    pub fn what_the_supervisor_said(&self) -> Option<WhatTheSupervisorSaid> {
        self.what_the_supervisor_said
            .lock()
            .ok()
            .and_then(|held| held.clone())
    }

    fn the_unit_ended(&self) -> Option<HowTheUnitEnded> {
        self.how_the_unit_ended
            .lock()
            .ok()
            .and_then(|held| held.clone())
    }

    /// **The job record request, asked** — `Rendering` S4's third step and
    /// `Queue & History`'s hand-over, one route.
    async fn the_job_record_is_asked_for(&self, job: &Identity) -> WhatTheSupervisorSaid {
        let mut fields = Fields::none().and(THE_JOB_FIELD, job.as_text());
        // **The label rides where one was given** — step 10c piece 1's shape.
        if let Some(label) = self.the_label.lock().ok().and_then(|held| held.clone()) {
            if !label.is_empty() {
                fields = fields.and(THE_LABEL_FIELD, label);
            }
        }
        // **`Rendering` S13: the hold rides the same request that writes the
        // job record**, so the machine places it in the same supervisor act
        // and the job never enters the line (`Rented-server runtime` U2 and `Rented-server runtime` U5).
        // The subject is the request's own job and the origin the submission
        // itself, so neither travels.
        //
        // **A hold with no dependency carries its reason alone** — `Rendering`
        // S18a's hold for the driver (held-job piece A, 2026-10-12).
        if let Some(hold) = &self.the_hold {
            if let Some(first) = &hold.dependency {
                fields = fields.and(THE_HELD_BEHIND_FIELD, first.as_text());
            }
            fields = fields.and(THE_HOLD_REASON_FIELD, hold.reason);
        }
        let request = Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            self.the_submission_command.clone(),
            THE_JOB_RECORD_IS_WRITTEN_REQUEST,
        )
        .carrying(fields);
        let said = match self
            .sessions
            .ask_the_supervisor(self.package_area, &request)
            .await
        {
            WhatCameBack::ItAnswered(answered) => {
                if answered
                    .the_request_name_it_says_it_does_not_answer()
                    .is_some()
                {
                    WhatTheSupervisorSaid::ItDoesNotTakeJobs
                } else {
                    // **Any answer that does not refuse the request by name is
                    // the acknowledgement** — no answer field is named for it
                    // yet (step 10c piece 1's owed item 2); piece 5 names them
                    // and may narrow this.
                    WhatTheSupervisorSaid::ItTookIt
                }
            }
            WhatCameBack::NothingCorrelatedCameBack | WhatCameBack::ItWasNeverAsked => {
                WhatTheSupervisorSaid::NothingThatCanBeRead
            }
        };
        if let Ok(mut held) = self.what_the_supervisor_said.lock() {
            *held = Some(said.clone());
        }
        said
    }
}

/// **`Rendering`'s seam, implemented by the one sender** (R3).
impl<C: AHeldConnection> AsksTheSupervisorForTheJobRecord for &TheOneSender<'_, C> {
    async fn write_the_job_record(&mut self, job: &Identity) -> Result<(), String> {
        match self.the_job_record_is_asked_for(job).await {
            WhatTheSupervisorSaid::ItTookIt => Ok(()),
            WhatTheSupervisorSaid::ItDoesNotTakeJobs => {
                Err("the machine's program does not answer the job record request".to_string())
            }
            WhatTheSupervisorSaid::NothingThatCanBeRead => {
                Err("nothing that answers the job record request came back".to_string())
            }
        }
    }
}

/// **`Queue & History`'s seam, implemented by the same sender** (R3).
impl<C: AHeldConnection> SendsTheSubmission for TheOneSender<'_, C> {
    /// **The submission handed over is `Rendering`'s whole ordered unit** — the
    /// remap material placed, the job's material published by one rename, the
    /// job record asked of the supervisor — because *"the submission's own
    /// atomicity is `Rendering`'s"*. `Queue & History`'s `submit` has already
    /// written the provisional record by the time this runs.
    fn send(&self, record: &AProvisionalRecord) -> WhatTheSubmissionGave {
        if let Ok(mut held) = self.the_label.lock() {
            *held = Some(record.label.clone());
        }
        let unit = &self.the_unit;
        let ended = waited_for(|| async move {
            let mut remap = &unit.remap;
            let mut machine = TheMaterialsSeat {
                sessions: self.sessions,
                the_run: unit.the_run,
                class: unit.the_submission_class,
                seated: None,
            };
            let mut supervisor = self;
            place_one_scene_jobs_unit(
                &mut remap,
                &mut machine,
                &mut supervisor,
                &unit.job,
                &unit.paths,
                &unit.manifest,
            )
            .await
        });
        let acknowledged = ended == HowTheUnitEnded::ItIsInTheLine;
        if let Ok(mut held) = self.how_the_unit_ended.lock() {
            *held = Some(ended);
        }
        if acknowledged {
            WhatTheSubmissionGave::ItWasAcknowledged
        } else {
            // **The seam has two answers and a unit that stopped is neither.**
            // The record stands and the driver reads how the unit ended; it
            // is never read as an acknowledgement.
            WhatTheSubmissionGave::TheAcknowledgementWasLost
        }
    }

    /// **`State, identity and recovery` §12 rule 4: ask the machine for the
    /// identity**, in the same act — `Rendering` S9: *"Smoozy resolves them by
    /// asking the machine, never by guessing"*.
    ///
    /// **One answer has one reader.** The machine answers this request with
    /// `State, identity and recovery` V1 step 2's three words, and
    /// [`crate::servers::the_machines_work::what_the_machine_says_about`] is
    /// where the desktop reads them, for a later connection's step 7 as for
    /// this act. ~~Until 2026-09-27 this read no field at all and answered
    /// *could not be asked* to every answer~~, so a lost acknowledgement always
    /// ended *could not tell* even where the machine would have said — step
    /// 10c's eighth-pass fix piece 4.
    ///
    /// - ***present*** → it holds the job, and the unit is in the line;
    /// - ***absent*** → it does not, and the unit is not submitted, with
    ///   `Rendering` S4b's debt taken by the driver;
    /// - **everything else** — the machine's own *cannot say*, a request it
    ///   refused, an answer with no word in it, a silence, an ask never made —
    ///   is *could not be asked*, which keeps the provisional record: the one
    ///   direction that cannot lose a job the machine has.
    fn does_the_machine_hold(&self, job: &Identity) -> Result<bool, String> {
        let mut machine = TheSupervisorOnThisPoint {
            sessions: self.sessions,
            package_area: self.package_area,
        };
        match what_the_machine_says_about(&mut machine, job) {
            WhatTheMachineSaidAboutTheJob::ItHoldsTheJob => Ok(true),
            WhatTheMachineSaidAboutTheJob::ItsOperationRecordShowsTheSubmissionWasRefusedOrNeverTaken => {
                Ok(false)
            }
            WhatTheMachineSaidAboutTheJob::TheMachineCannotSay => {
                Err("the machine could not say whether it holds the job".to_string())
            }
        }
    }
}

/// **The machine's supervisor, asked over this sender's point**, for the one
/// reader of *does the machine hold this job* — `the_machines_work.rs`'s seam,
/// given this file's point and this file's wait.
struct TheSupervisorOnThisPoint<'s, C: AHeldConnection> {
    sessions: &'s ThisServersSessions<C>,
    package_area: &'s str,
}

impl<C: AHeldConnection> AsksTheSupervisor for TheSupervisorOnThisPoint<'_, C> {
    fn ask(&mut self, request: &Request) -> WhatCameBack {
        let (sessions, package_area) = (self.sessions, self.package_area);
        waited_for(|| async move { sessions.ask_the_supervisor(package_area, request).await })
    }
}

// ════════════════════════════════════════════════════════════════════════════
// The unit's two transfers, under `Rendering` §7's submission class
// ════════════════════════════════════════════════════════════════════════════

/// **One transfer's admission: the desktop point's slot, then a seat at the
/// server's point, then the file-transfer session on it.** `Rendering` §7:
/// *"every transfer a submission act arranges carries the submission class"*,
/// and the two it covers are the remap material's placement and the material's
/// publication. **R17 and R13**: no room is a reason to wait while the
/// connection is live, and never a reason to fail.
async fn a_seat_for_one_transfer<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    the_run: &TheRunsTransfers,
    class: &'static str,
) -> Option<(TheChannelsDraw, Subsystem<ALentChannel<C>>)> {
    let draw = loop {
        match the_run.admit_to_carry(Direction::Up, &[class], class) {
            Ok(draw) => break draw,
            Err(_) if sessions.a_connection_is_live() => {
                tokio::time::sleep(THE_PAUSE_BEFORE_ASKING_AGAIN).await;
            }
            Err(_) => return None,
        }
    };
    let channel = sessions.a_transfer_channel().await.ok()?;
    let subsystem = Subsystem::negotiate(channel).await.ok()?;
    Some((draw, subsystem))
}

/// **`Job files and outputs` M5's publication, on a seat of its own.**
///
/// The seat is taken at the first operation and **given back at the rename**,
/// which is M5's last act — so the slot is not held while the supervisor is
/// asked.
struct TheMaterialsSeat<'a, C: AHeldConnection> {
    sessions: &'a ThisServersSessions<C>,
    the_run: &'a TheRunsTransfers,
    class: &'static str,
    seated: Option<(TheChannelsDraw, Subsystem<ALentChannel<C>>)>,
}

impl<C: AHeldConnection> TheMaterialsSeat<'_, C> {
    async fn the_subsystem(
        &mut self,
    ) -> Result<&mut Subsystem<ALentChannel<C>>, WhatTheFolderSaid> {
        if self.seated.is_none() {
            self.seated = a_seat_for_one_transfer(self.sessions, self.the_run, self.class).await;
        }
        self.seated
            .as_mut()
            .map(|(_, subsystem)| subsystem)
            .ok_or(WhatTheFolderSaid::TheChannelIsGone)
    }

    fn the_seat_is_given_back(&mut self) {
        if let Some((mut draw, subsystem)) = self.seated.take() {
            let _ = draw.the_channel_ended(Instant::now());
            drop(subsystem);
        }
    }
}

impl<C: AHeldConnection> TheMachinesDirectories for TheMaterialsSeat<'_, C> {
    async fn make_one_directory(&mut self, path: &str) -> Result<(), WhatTheFolderSaid> {
        self.the_subsystem().await?.make_one_directory(path).await
    }

    async fn write_a_whole_file_durably(
        &mut self,
        path: &str,
        bytes: &[u8],
    ) -> Result<(), WhatTheFolderSaid> {
        self.the_subsystem()
            .await?
            .write_a_whole_file_durably(path, bytes)
            .await
    }

    async fn read_a_whole_file(&mut self, path: &str) -> Result<Vec<u8>, WhatTheFolderSaid> {
        self.the_subsystem().await?.read_a_whole_file(path).await
    }

    async fn rename_without_replacing(
        &mut self,
        from: &str,
        to: &str,
    ) -> Result<(), WhatTheFolderSaid> {
        let renamed = self
            .the_subsystem()
            .await?
            .rename_without_replacing(from, to)
            .await;
        self.the_seat_is_given_back();
        renamed
    }

    async fn remove_one_file(&mut self, path: &str) -> Result<(), WhatTheFolderSaid> {
        self.the_subsystem().await?.remove_one_file(path).await
    }

    async fn remove_one_empty_directory(&mut self, path: &str) -> Result<(), WhatTheFolderSaid> {
        self.the_subsystem()
            .await?
            .remove_one_empty_directory(path)
            .await
    }

    async fn list_one_directory(&mut self, path: &str) -> Result<Vec<Vec<u8>>, WhatTheFolderSaid> {
        self.the_subsystem().await?.list_one_directory(path).await
    }
}

/// **`Dependency Upload & Remap` M10's placement: the material staged,
/// integrity-checked and published by one rename, through the transfer
/// contract** (`Server protocol and transfers` §7), before the job's own
/// material.
///
/// **The material is written to Smoozy's own temporary area on this computer
/// first** (L3a), because §7's loop sends a file; it is removed once the
/// transfer ends.
struct TheRemapPlacement<'a, C: AHeldConnection> {
    sessions: &'a ThisServersSessions<C>,
    the_run: &'a TheRunsTransfers,
    class: &'static str,
    layout: TheLayout,
    material: RemapMaterial,
    server_row: Identity,
    installation: Option<Identity>,
    run: Identity,
    record: &'a dyn TheRecordAnUploadWrites,
    the_temporary_area: Option<std::path::PathBuf>,
}

impl<C: AHeldConnection> PlacesTheRemapMaterial for &TheRemapPlacement<'_, C> {
    async fn place(&mut self, job: &Identity) -> Result<(), String> {
        let placed = self.the_material_is_placed(job).await;
        // **M11: only a publication lets the submission go ahead**, and the
        // unit that stops it is named in M11's own words.
        match the_ordered_unit(placed.is_ok()) {
            TheOrderedUnit::TheMaterialIsPlacedAndTheSubmissionMayGoAhead => Ok(()),
            TheOrderedUnit::TheSubmissionIsNotAttempted {
                the_unit_that_stopped_it,
            } => Err(format!(
                "{the_unit_that_stopped_it}: {}",
                placed.err().unwrap_or_default()
            )),
        }
    }
}

impl<C: AHeldConnection> TheRemapPlacement<'_, C> {
    async fn the_material_is_placed(&self, job: &Identity) -> Result<(), String> {
        let Some(installation) = self.installation.clone() else {
            return Err("no installation of Smoozy's is confirmed on this server".to_string());
        };
        let final_path = RemapMaterial::location_in(&self.layout, job);
        let final_path = String::from_utf8_lossy(final_path.as_bytes()).into_owned();
        let Some((the_remaps_folder, _)) = final_path.rsplit_once('/') else {
            return Err("the remap material's place has no folder".to_string());
        };
        let staging = format!(
            "{the_remaps_folder}/{}",
            smoozy_file_browser::partial::THE_RESERVED_STAGING_FOLDER
        );

        // **The source, on this computer, in Smoozy's own temporary area.**
        let Some(area) = self.the_temporary_area.clone() else {
            return Err("this computer named no temporary area for Smoozy".to_string());
        };
        let source = area.join(format!("remap-material-{}", job.as_text()));
        if std::fs::create_dir_all(&area).is_err()
            || std::fs::write(&source, self.material.as_written()).is_err()
        {
            return Err("the remap material could not be written on this computer".to_string());
        }
        let the_source = source.to_string_lossy().into_owned();

        let transfer = Identity::issue(IdentityKind::Transfer)
            .map_err(|_| "this computer offered no randomness for an identity".to_string())?;
        let mut arranged: TransferRecord = arrange_the_materials_placement(
            &self.layout,
            &self.material,
            job,
            transfer,
            self.server_row.clone(),
            installation,
            self.run.clone(),
            &the_source,
            &staging,
        )
        .map_err(|_| "the remap material's address will not fit".to_string())?;

        let placed = self.carried(&mut arranged, the_remaps_folder).await;
        let _ = std::fs::remove_file(&source);
        placed
    }

    async fn carried(
        &self,
        arranged: &mut TransferRecord,
        the_remaps_folder: &str,
    ) -> Result<(), String> {
        // **`Server protocol and transfers` S8: the record is durable before
        // the first byte is written.**
        let mut keeper = TheTransferRecordsHere {
            record: self.record,
        };
        use smoozy_server_protocol::transfers::moving::KeepsTheRecord;
        keeper
            .write_it_down(arranged)
            .map_err(|failure| format!("the transfer could not be written down: {}", failure.0))?;

        let Some((mut draw, mut subsystem)) =
            a_seat_for_one_transfer(self.sessions, self.the_run, self.class).await
        else {
            return Err("no session for the transfer could be had on this connection".to_string());
        };
        // **`Dependency Upload & Remap` A2's `remaps` segment, made plainly
        // where it is not there** — `Job files and outputs` N9's plain form,
        // whose refusal over a folder already there is the answer rather than
        // a fault. A folder that truly could not be made fails the transfer
        // below, by name.
        let _ = subsystem.make_one_directory(the_remaps_folder).await;
        let mut local = ThePlatformFileAdapter;
        let mut watcher = NobodyIsWatching;
        let happened = carry_out(
            Transfer {
                subsystem: &mut subsystem,
                local: &mut local,
                budget: &mut draw,
                keeper: &mut keeper,
                watcher: &mut watcher,
                inactivity_deadline: THE_INACTIVITY_DEADLINE,
                // **The material is immutable and its place is a function of
                // the job identity alone**, so nothing stands there to replace.
                intent: Intent::CreateIntended,
            },
            arranged,
        )
        .await;
        let _ = draw.the_channel_ended(Instant::now());
        drop(subsystem);
        match happened.ending {
            Ending::Published => Ok(()),
            other => Err(format!("the transfer ended {other:?}")),
        }
    }
}

/// **`Server protocol and transfers` S8's writer, through the application's
/// own record** — one short act per write.
struct TheTransferRecordsHere<'a> {
    record: &'a dyn TheRecordAnUploadWrites,
}

impl smoozy_server_protocol::transfers::moving::KeepsTheRecord for TheTransferRecordsHere<'_> {
    fn write_it_down(
        &mut self,
        record: &TransferRecord,
    ) -> Result<(), smoozy_server_protocol::transfers::moving::LocalFailure> {
        use smoozy_server_protocol::transfers::moving::LocalFailure;
        let mut written = Err(LocalFailure("the local record would not open".into()));
        self.record.with_the_record(&mut |connection| {
            written = crate::record::transfer_records::put(connection, record)
                .map_err(|failure| LocalFailure(format!("{failure:?}")));
        });
        written
    }
}

// ════════════════════════════════════════════════════════════════════════════
// The act
// ════════════════════════════════════════════════════════════════════════════

/// **What the page hands over at Submit** — every value the page composed,
/// and nothing the driver has to go back to the page for.
///
/// **Cloned for one reason**: `Rendering` S11's *Submit the remaining
/// scenes* composes its fresh act from the act it follows, the scene-jobs
/// resolved as not submitted and nothing else (the 38's piece 2).
#[derive(Clone)]
pub struct WhatThePageHandsOver {
    /// The server row the act is made on, as it stands (`Queue & History`
    /// N10's server half).
    pub server: TheServerAsItStood,
    /// The installation the row last confirmed, where one is.
    pub installation: Option<Identity>,
    /// `Job files and outputs` L1's layout, as the row records it.
    pub layout: TheLayout,
    /// The package area, from piece 4's `where_the_package_area_is`.
    pub package_area: String,
    /// The `.blend`'s own path on the machine (`File Browser` L1a's bytes).
    pub the_blend: String,
    /// The job's name as typed, or nothing (`Queue & History` Y8).
    pub label: Option<String>,
    /// **`Rendering` S5a's field**, composed from the page's answer and the
    /// stored default by `Rendering`'s own composer.
    pub reporting: CarriedAndNotInterpretedHere,
    /// **`Queue & History` N10a**: whether the version was the user's override.
    pub the_version_origin: AnswerOrigin,
    /// `Queue & History` N15's first mark, as `Rendering` W2 found the record.
    pub dependency_check: Option<DependencyCheckAtSubmission>,
    /// `Rendering` S3a's input: what the desktop holds about the target build.
    pub what_the_desktop_holds_about_the_build: WhatTheDesktopHoldsAboutTheBuild,
    /// **`Rendering` W4's *Render one frame first***, set on the page.
    pub the_first_frame_check: bool,
    /// **The submit strip's *Download as it renders*, as the box stands at the
    /// moment of Submit.** `Rendering` puts auto-download's moment at
    /// submitting and `Download` R1 makes the arrangement there; **where it is
    /// off nothing is written and the job has never had an arrangement**,
    /// which is not the same thing as one switched off.
    pub the_download_as_it_renders_choice: bool,
    /// **`Download` R1's destination, as the strip names it** — the folder
    /// chosen for this job, else the standing folder, else §8's proposal, which
    /// is the order the box offers them in. **Fixed for the life of the job**,
    /// because nothing can be asked of a user who is not there.
    ///
    /// **Nothing where nobody named one** (`Download` H3), and then no
    /// arrangement is made: an arrangement with no destination would have to
    /// ask.
    pub where_the_downloads_go: Option<Vec<u8>>,
    /// **`Download` N3's narrowing, as the strip's format box composed it** —
    /// the format names exactly as the reading recorded them (`Download` N1).
    /// **Nothing is *everything***, which is no filter rather than a frozen
    /// list.
    pub the_formats_chosen: Option<Vec<Vec<u8>>>,
    /// **One per scene-job, in the order the act enumerates them** — the
    /// payload `Render Settings` composed for it (Y1), which is also its kept
    /// setup (record version 25).
    pub scene_jobs: Vec<Payload>,
    /// **The reading standing for the file on the page, where one does** —
    /// what *Read the file* left. **It never reaches the machine**: the
    /// manifest says no reading (`Job files and outputs` R7's second bullet),
    /// and the render worker takes its own at the render's start. It is read
    /// here for the local record alone — `Queue & History` N14's file's values
    /// beside the overrides (`Render Settings` Y1a) and `Queue & History`
    /// E12a's ask as the reading gave it.
    pub the_reading_in_force: Option<RecordedReading>,
    /// **`Job files and outputs` M1b's card make field**, composed from the
    /// hardware reading `Servers` holds for this row by `Rendering`'s own
    /// composer. **`Rendering` L23 looks the render technology up from it on
    /// the machine, which cannot take the reading itself.**
    pub the_card_make: TheCardMakeAtSubmission,
    /// This run's identity (`Server protocol and transfers` C11).
    pub run: Identity,
    /// **`Queue & History` N15's second mark: the earlier job this one carries
    /// on from**, where the page was opened by *Carry on* (`Rendering` A4) —
    /// written onto every scene-job's record of the act. Nothing on every
    /// other page. The 38's piece 2.
    pub carries_on_from: Option<smoozy_queue_history::record::CarriesOnFrom>,
    /// **Smoozy's own temporary area on this computer** (L3a), where the remap
    /// material is written before §7's loop sends it. Nothing where the system
    /// names none, and then the material cannot be placed and says so.
    pub the_temporary_area: Option<std::path::PathBuf>,
    /// **`Job files and outputs` P23: the server row's chosen fallback folder
    /// as it stands at the moment of Submit**, or nothing where none is set —
    /// carried on the manifest beside the statement that the job's folder is
    /// chosen at render start (`Job files and outputs` M1, `Rendering` S5), so
    /// a later change to the row never moves this job.
    pub the_servers_fallback_folder: Option<String>,
    /// **`Server Setup` R1c's readings beside the job's own**, taken at the
    /// press for the server the act is made on
    /// ([`crate::servers::the_driver_swap::the_driver_at_submission`]) and
    /// judged per job as it is composed. **The default holds nothing.** Added
    /// 2026-10-12, held-job piece A.
    pub the_driver_at_submission: crate::servers::the_driver_swap::TheDriverAtSubmission,
    /// **`Queue & History` P5b: the frame text the user typed, as Render
    /// Settings accepted it** (`Render Settings` F1), for the local record
    /// alone — written beside each job's kept setup at submission and **never
    /// into a payload, a manifest or anything sent to a machine**. One value
    /// for the act: a single scene's own typed range, or the group's where
    /// several scenes are ticked, which every scene-job of an all-scenes act
    /// carries (`Render Settings` J3). **Nothing where the range was the file's
    /// own** or the page held no typed text. The eighteenth notes, piece 7.
    pub the_typed_frames: Option<String>,
}

// ════════════════════════════════════════════════════════════════════════════
// The remap, composed at submission — step 10c's dependency wave piece 7
// ════════════════════════════════════════════════════════════════════════════

/// **`Dependency Upload & Remap` M10's composition at submission, and
/// `Dependency Upload & Remap` M22c's copies beside it.**
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TheRemapComposed {
    pub pairs: Vec<smoozy_dependencies::remap::Pair>,
    pub copies_owed: Vec<smoozy_dependencies::acting::placing::TheCopyTheMachineOwes>,
    /// **`Dependency Upload & Remap` M10's *"the reason any answer produced
    /// none"***, from the
    /// composition itself.
    pub without_a_pair: Vec<(RecordedLocation, smoozy_dependencies::remap::NoPairFor)>,
    /// **A located answer no reading has written its kind and setting into**
    /// (`Dependency Upload & Remap` K4's last limb) — so it gives no pair until
    /// the file is checked again.
    pub without_a_kind: Vec<RecordedLocation>,
}

impl TheRemapComposed {
    /// **`Dependency Upload & Remap` K9's machine locations the material
    /// names** — every pair's new location and every copy's mirror, once
    /// each, in the order composed.
    pub fn the_machine_locations_it_names(&self) -> Vec<MachineLocation> {
        let mut named: Vec<MachineLocation> = Vec::new();
        let every = self
            .pairs
            .iter()
            .map(|pair| pair.new_location.clone())
            .chain(self.copies_owed.iter().map(|copy| copy.from.clone()));
        for at in every {
            if !named.contains(&at) {
                named.push(at);
            }
        }
        named
    }
}

/// **The record would not answer the composition**, so the material cannot be
/// composed from it — and a material composed with nothing where the record
/// holds answers would render the artist's own paths at success.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheRecordWouldNotAnswer;

/// **`Dependency Upload & Remap` M10: *"Composition happens on the desktop,
/// from the record, at submission — the located answers and the landing rule
/// give the pairs."*** From the record alone: the answers are read under the
/// holding file a check of this file keys them by (`File Browser` N6's source
/// path, else `Dependency Upload & Remap` K5's machine key —
/// [`crate::record::dependency_records::WhatFilesSays`]), each answer's kind
/// and setting are what the check read it as (`Dependency Upload & Remap`
/// K4's last limb), and the pairs are `Dependency Upload & Remap`'s own
/// `compose_the_pairs` and the copies its `compose_the_copies_owed`, handed the
/// same answers, so each kind is taken by one of the two (`Dependency Upload &
/// Remap` M22a).
///
/// **The case this was first written for, and the cases it meets now.** It
/// was written, at step 10c piece 6, for a file never checked, so every
/// material carried no pairs. Dependency piece 7 composed the pairs from the
/// record and the run's own check, since the record kept no kind; **so from
/// the first restart after a check, a checked and sent file was submitted with
/// no pairs** — the pictures on the machine, the render looking for them at
/// the Mac's paths. Piece 7b gave the record the kinds, and the run holds
/// nothing this needs.
///
/// **Nothing is composed** where the page names no installation: no machine
/// key can be made, and nothing was placed under one.
pub fn the_remap_is_composed(
    given: &WhatThePageHandsOver,
    record: &dyn TheRecordAnUploadWrites,
) -> Result<TheRemapComposed, TheRecordWouldNotAnswer> {
    let Some(installation) = given.installation.clone() else {
        return Ok(TheRemapComposed::default());
    };
    let mut read = None;
    record.with_the_record(&mut |connection| {
        read = Some(the_remap_for_file(connection, &given.server.local_server,
            installation.clone(), &given.layout, &given.the_blend));
    });
    read.unwrap_or(Err(TheRecordWouldNotAnswer))
}

/// `Render Settings` Q4 and `Dependency Upload & Remap` M4: one saved-answer
/// lookup and pair composition for submission and Read the file.
pub fn the_remap_for_file(
    connection: &rusqlite::Connection,
    server: &Identity,
    installation: Identity,
    layout: &TheLayout,
    blend: &str,
) -> Result<TheRemapComposed, TheRecordWouldNotAnswer> {
    let machine = MachineKey::of(server.clone(), installation);
    let root = MachineLocation::composed(blend.as_bytes().to_vec());
    let holding = crate::record::dependency_records::WhatFilesSays::read(connection, &machine)
        .the_holding_file_a_check_keys(&root);
    let keeper = TheDependencyRecord::over(connection);
    let answers = keeper.the_answers_of(&holding).map_err(|_| TheRecordWouldNotAnswer)?;
    let fixings = keeper.the_fixings_on(&machine).map_err(|_| TheRecordWouldNotAnswer)?;

    let (to_compose, without_a_kind) = smoozy_dependencies::remap::the_answers_to_compose(answers);
    let pairs = smoozy_dependencies::remap::compose_the_pairs(
        layout,
        &holding,
        &to_compose,
        &fixings,
    );
    let copies = smoozy_dependencies::acting::placing::compose_the_copies_owed(
        layout,
        &holding,
        &to_compose,
    );
    Ok(TheRemapComposed {
        pairs: pairs.pairs,
        copies_owed: copies.copies,
        without_a_pair: pairs.without_a_pair,
        without_a_kind,
    })
}

/// **What became of one scene-job** — the whole of what `Rendering` S10 has
/// the user told, per scene.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatBecameOfOneJob {
    /// **In the machine's line** (`Queue & History` Q3).
    ItIsInTheLine,
    /// **`Rendering` S3a: refused before anything of it was sent**, naming the
    /// engine and the build.
    TheBuildLacksTheEngine { engine: String, build: String },
    /// **`Queue & History` Y3: the record was refused, so nothing was sent.**
    TheRecordWasRefused,
    /// **`Rendering` S4: an earlier step stopped it and the later were not
    /// attempted.** Nothing of the job is in the line.
    ItStoppedBeforeTheJobRecord { the_step: &'static str },
    /// **The machine's program answered that it does not take jobs** — the
    /// step 10c piece 5 builds. The job is not on the machine.
    TheServerDoesNotTakeJobs,
    /// **`Rendering` S9 and S10: Smoozy could not account for it.** The
    /// provisional record stands and the machine is asked again at a later
    /// connection (`State, identity and recovery` V1 step 2).
    NotKnown,
    /// **`Rendering` S13's second job, not sent because its first job is not
    /// in the machine's line** — a hold behind a job the machine never took
    /// would stand for ever. **It says nothing of its own**: the first job's
    /// own reason is what the user is told. *(This was
    /// `TheHeldRestCannotBeSentYet`, the second job refused on every act for
    /// want of a field, until step 10c's eleventh-pass fix piece 8.)*
    NotSentBehindAFirstFrameThatWasNot,
    /// **`Rendering` S4a: a unit before it was left unaccounted, which ends
    /// the act's run.** Submitting the rest is the user's next act (S11).
    NotAttemptedBehindAnUnaccountedOne,
    /// **The file on the machine could not be read for its four values**, so
    /// `Job files and outputs` M1's identity at submission has no value.
    TheFileOnTheServerCouldNotBeRead,
    /// Something on this computer failed before anything was sent — no
    /// identity could be issued, a path will not fit, the record would not
    /// open.
    ThisComputerCouldNotComposeIt,
}

/// **What the whole act came to.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheActCameTo {
    /// **The first-frame check needs a frame range and none is known**, so
    /// `Rendering` S13's first frame cannot be named and nothing was sent: no
    /// range was typed, and either the file was not read or the job names no
    /// scene of the reading's several. **`a_scene_can_be_ticked` is the
    /// second**, where ticking a scene would name one
    /// (`Design/Render Settings § Submitting`, 2026-10-01).
    TheFirstFrameIsNotKnown { a_scene_can_be_ticked: bool },
    /// Every scene-job, in the order the act enumerated them.
    TheseJobs(Vec<WhatBecameOfOneJob>),
}

/// **`Rendering` S4a across acts: units against one machine are serialised at
/// the desktop, in the order the user made them.** One act per server at a
/// time; a second waits for the first to end.
fn the_line_for(server: &Identity) -> Arc<Mutex<()>> {
    static LINES: OnceLock<Mutex<BTreeMap<String, Arc<Mutex<()>>>>> = OnceLock::new();
    let lines = LINES.get_or_init(|| Mutex::new(BTreeMap::new()));
    let mut lines = lines
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    Arc::clone(lines.entry(server.as_text()).or_default())
}

// ════════════════════════════════════════════════════════════════════════════
// The acts in flight — the Creator's seventh notes, piece Z7
// ════════════════════════════════════════════════════════════════════════════

/// **One act of submitting while it runs, as the queue draws it** —
/// `Queue & History` C10's *Submitting…* row: in its server's line from the
/// press of *Submit* (the runner's ruling on C10) until the machine's own row
/// replaces it. **Held for the run and nowhere else**: nothing durable is
/// written for it, and the provisional records stay the durable half.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnActInFlight {
    /// The local server identifier, as text — the line the row is drawn in.
    pub server: String,
    /// The job's name, as the queue names a job (`Queue & History` Y8).
    pub name: String,
    /// **The act's job identities, once `Rendering` S3 has issued them** —
    /// what the machine's own row is known by.
    pub jobs: Vec<Identity>,
    /// **The act ended with a job in the machine's line**, so the row stays
    /// until the machine's own row can take its place.
    pub ended_in_the_line: bool,
    /// **The moment of the reading in force on its server when the ended act
    /// was first drawn over one.** Compared for equality alone (`State,
    /// identity and recovery` D5).
    pub the_reading_at_its_end: Option<SystemTime>,
    /// **What the act is known by** — what the row's cross hands back
    /// ([`a_not_submitted_row_is_cleared`]).
    pub which: u64,
    /// **The `.blend` the act is of, on the machine** — what tells that file
    /// sent to that server again, which takes a not-submitted row away. Empty
    /// for an act begun by [`an_act_is_begun_on`].
    pub file: String,
    /// **`Queue & History` C10 as changed 2026-10-19: the act ended with a
    /// job not in the machine's line**, and this is what it came to — the
    /// row stays in its server's line saying what the panel says, *Not
    /// submitted.* and why, with a cross that clears it. **It goes when the
    /// user clears it, when that file is submitted to that server again, or
    /// when the run ends**; nothing durable holds it. *(Out of the live pass on
    /// Vast for the thirteenth and fourteenth notes: the line went while a job
    /// was sent, the act ended *files not placed*, and the row went with
    /// nothing said anywhere the user was looking.)*
    pub not_submitted: Option<WhatTheActCameTo>,
}

fn the_acts_in_flight() -> &'static Mutex<BTreeMap<u64, AnActInFlight>> {
    static HELD: OnceLock<Mutex<BTreeMap<u64, AnActInFlight>>> = OnceLock::new();
    HELD.get_or_init(|| Mutex::new(BTreeMap::new()))
}

/// **The act's hold on its row.** Begun at the press; the act tells it its
/// identities and how it ended. **Dropped, it takes the row away** unless the
/// act ended with a job in the machine's line and none unaccounted.
pub struct TheActInFlight {
    which: u64,
    kept: bool,
}

/// **An act is begun: its *Submitting…* row is in its server's line from
/// here.** The press of *Submit* calls it before the act's thread starts.
pub fn an_act_is_begun(given: &WhatThePageHandsOver) -> TheActInFlight {
    an_act_is_begun_on_the_file(
        &given.server.local_server.as_text(),
        &given.the_blend,
        smoozy_queue_history::naming::what_a_job_is_called(given.label.as_deref(), Some(&given.the_blend)),
    )
}

/// [`an_act_is_begun`], over the two things the row reads.
pub fn an_act_is_begun_on(server: &str, name: String) -> TheActInFlight {
    an_act_is_begun_on_the_file(server, "", name)
}

/// [`an_act_is_begun`], over the server, the file and the name. **A row an
/// earlier act on that file and that server left saying *Not submitted.*
/// goes now** — `Queue & History` C10: it goes when that file is submitted to
/// that server again, since the new act's own row says what is happening.
pub fn an_act_is_begun_on_the_file(server: &str, file: &str, name: String) -> TheActInFlight {
    static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let which = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if let Ok(mut held) = the_acts_in_flight().lock() {
        if !file.is_empty() {
            held.retain(|_, act| !(act.not_submitted.is_some() && act.server == server && act.file == file));
        }
        held.insert(
            which,
            AnActInFlight {
                server: server.to_string(),
                name,
                jobs: Vec::new(),
                ended_in_the_line: false,
                the_reading_at_its_end: None,
                which,
                file: file.to_string(),
                not_submitted: None,
            },
        );
    }
    TheActInFlight { which, kept: false }
}

/// **The cross on a not-submitted row, pressed** — `Queue & History` C10: the
/// row goes. **True where a not-submitted row was there to clear**; an act
/// still running, or one whose row waits for the machine's own, is never
/// taken by it.
pub fn a_not_submitted_row_is_cleared(which: u64) -> bool {
    let Ok(mut held) = the_acts_in_flight().lock() else {
        return false;
    };
    if held.get(&which).is_some_and(|act| act.not_submitted.is_some()) {
        held.remove(&which);
        return true;
    }
    false
}

impl TheActInFlight {
    /// What this act is known by while it runs — for a test to look it up.
    pub fn which(&self) -> u64 {
        self.which
    }

    /// **`Rendering` S3's identities, issued.**
    pub fn its_jobs_are(&self, jobs: &[Identity]) {
        if let Ok(mut held) = the_acts_in_flight().lock() {
            if let Some(act) = held.get_mut(&self.which) {
                act.jobs = jobs.to_vec();
            }
        }
    }

    /// **How the act ended.** Every job in the machine's line keeps the row
    /// for the machine's own to replace. **An act Smoozy could not account
    /// for** — a job *not known*, or one left unattempted behind it — takes
    /// it away, as before: what the queue already shows for that case takes
    /// over (`Queue & History` C10's sentence on it, and the part banner of
    /// `Rendering` S10). **Every other ending leaves a job not in the line,
    /// and the row stays saying so** — a refusal before the provisional
    /// record or after it, a frame not known, or an act that failed part-way
    /// such as *files not placed* — until it is cleared, that file is sent to
    /// that server again, or the run ends (C10 as changed 2026-10-19).
    pub fn it_ended(&mut self, came_to: &WhatTheActCameTo) {
        let in_the_line = match came_to {
            WhatTheActCameTo::TheFirstFrameIsNotKnown { .. } => false,
            WhatTheActCameTo::TheseJobs(jobs) => {
                if jobs.is_empty()
                    || jobs.iter().any(|job| {
                        matches!(
                            job,
                            WhatBecameOfOneJob::NotKnown | WhatBecameOfOneJob::NotAttemptedBehindAnUnaccountedOne
                        )
                    })
                {
                    return;
                }
                jobs.iter().all(|job| *job == WhatBecameOfOneJob::ItIsInTheLine)
            }
        };
        if let Ok(mut held) = the_acts_in_flight().lock() {
            if let Some(act) = held.get_mut(&self.which) {
                if in_the_line {
                    act.ended_in_the_line = true;
                } else {
                    act.not_submitted = Some(came_to.clone());
                }
                self.kept = true;
            }
        }
    }
}

impl Drop for TheActInFlight {
    fn drop(&mut self) {
        if self.kept {
            return;
        }
        if let Ok(mut held) = the_acts_in_flight().lock() {
            held.remove(&self.which);
        }
    }
}

/// **One act in flight, by what it is known by** — `None` once its row is
/// gone.
pub fn the_act_in_flight(which: u64) -> Option<AnActInFlight> {
    the_acts_in_flight().lock().ok()?.get(&which).cloned()
}

/// **The *Submitting…* rows to draw now**, in the order the acts began.
/// `the_reading_of` gives a server's reading in force: its moment and the
/// jobs it holds.
///
/// **No two rows for one job**: an act whose server's reading holds one of
/// its jobs draws no row, and one that has ended leaves the list then. **No
/// gap either**: an ended act's row stays until that reading, or, for a job
/// that ended before any reading saw it, until its server's next reading.
/// **A server with no reading here leaves its acts as they are**: the next
/// reading is counted from the first one drawn after the act ended.
pub fn the_acts_in_flight_to_draw(
    the_reading_of: &dyn Fn(&str) -> Option<(SystemTime, Vec<Identity>)>,
) -> Vec<AnActInFlight> {
    let Ok(mut held) = the_acts_in_flight().lock() else {
        return Vec::new();
    };
    let mut drawn = Vec::new();
    held.retain(|_, act| {
        // **A row saying *Not submitted.* stays whatever the reading holds**
        // — it goes only when cleared, sent again or the run ends (`Queue &
        // History` C10). A first-frame check's first job may be in the line
        // and drawn by the machine's own row beside it.
        if act.not_submitted.is_some() {
            drawn.push(act.clone());
            return true;
        }
        let reading = the_reading_of(&act.server);
        let its_row_is_there = reading
            .as_ref()
            .is_some_and(|(_, jobs)| jobs.iter().any(|job| act.jobs.contains(job)));
        if its_row_is_there {
            return !act.ended_in_the_line;
        }
        if let (true, Some((now, _))) = (act.ended_in_the_line, reading.as_ref()) {
            match act.the_reading_at_its_end {
                None => act.the_reading_at_its_end = Some(*now),
                Some(then) if then != *now => return false,
                Some(_) => {}
            }
        }
        drawn.push(act.clone());
        true
    });
    drawn
}

// ════════════════════════════════════════════════════════════════════════════
// The first-frame check's second job, as this run sent it — step 10c's
// eleventh-pass fix piece 8
// ════════════════════════════════════════════════════════════════════════════

/// **What this run knows of a first-frame check's second job** — the job it is
/// held behind, that job's one frame, and how many frames the held job itself
/// renders. **It is what lets the queue draw `Design/Queue & History § What the
/// job writes`' pair on the held entry** — *Frame 1 is rendered…*, *Render
/// all — 240 frames*, *Cancel* — and `Queue & History` H6's statement of
/// what the job waits behind.
///
/// This is the legacy run fallback. New pair submissions also keep their
/// purpose, exact frame and submitted range on each provisional desktop job
/// record (`Queue & History` P5a). The machine's hold still supplies approval
/// authority; the retained values only describe the pair after reopening.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheFirstFrameCheck {
    /// **`Rendering` S13's dependency: the first job's identity**, which the
    /// second job's request names as the job it is held behind.
    pub the_first_job: Identity,
    /// The first job's one frame — the range's own first frame, never *1*
    /// (`Specs/Render Settings`: *"Nothing in Smoozy assumes the first frame is
    /// frame 1"*).
    pub the_first_frame: i64,
    /// **How many frames the second job renders** — the held job's own count,
    /// its first frame included (`Rendering` S13 as amended 2026-10-02) — the
    /// figure *Render all* names.
    pub frames_held: u32,
}

fn the_checks_this_run() -> &'static Mutex<BTreeMap<String, TheFirstFrameCheck>> {
    static HELD: OnceLock<Mutex<BTreeMap<String, TheFirstFrameCheck>>> = OnceLock::new();
    HELD.get_or_init(|| Mutex::new(BTreeMap::new()))
}

/// **The second job of a first-frame check is about to be sent**, held
/// behind `check`'s first job. Written before the request goes, so a reading
/// that lands while the act is still ending already knows it.
pub fn a_first_frame_check_is_sent(the_held_job: &Identity, check: TheFirstFrameCheck) {
    if let Ok(mut held) = the_checks_this_run().lock() {
        held.insert(the_held_job.as_text(), check);
    }
}

/// **The first-frame check `job` is the second job of**, where this run sent
/// it — and nothing otherwise, including every job held at the start of its
/// render (`Rented-server runtime` U6), which is a different question with
/// different words (`Design/Queue & History § What the job writes`).
pub fn the_first_frame_check_behind(job: &Identity) -> Option<TheFirstFrameCheck> {
    the_checks_this_run().lock().ok()?.get(&job.as_text()).cloned()
}

/// **The range `Rendering` S13's check splits, where its first frame can be
/// named**: the payload's own where the user typed one; else, where the job
/// names a scene the reading in force holds, **that scene's frame start, end
/// and step** — `Render Settings` F4's *the file's own range, resolved from the
/// reading in force*, floored at a step of one as the machine floors it
/// (`Render Settings` Y9). **Nothing where neither stands**: no range typed,
/// and nothing read or a job naming none of the reading's several scenes.
///
/// *Until step 10c's Sesterce-pass fix piece 1 only the payload's own range
/// was read here, so after* Read the file *an untouched range — no entry at
/// all (`Render Settings` F4) — was always refused as a file not read.*
fn the_range_the_check_splits(payload: &Payload, the_reading_in_force: Option<&RecordedReading>) -> Option<FrameForm> {
    if let Some(typed) = &payload.frame_range {
        return Some(typed.clone());
    }
    let named = payload.scene.as_ref()?;
    let scene = the_reading_in_force?
        .settings
        .scenes
        .iter()
        .find(|scene| scene.name.as_bytes() == named.name.as_bytes())?;
    Some(FrameForm::Triple {
        first: scene.frame_start,
        last: scene.frame_end,
        step: scene.frame_step.max(1),
    })
}

/// **The act, performed.** It blocks until the act ends; the page calls it off
/// its own thread (P5).
pub fn the_act_is_performed<C: AHeldConnection>(
    given: WhatThePageHandsOver,
    sessions: &ThisServersSessions<C>,
    the_run: &TheRunsTransfers,
    record: &dyn TheRecordAnUploadWrites,
) -> WhatTheActCameTo {
    the_act_is_performed_naming_its_jobs(given, sessions, the_run, record).0
}

/// **One job the act sent, by its identity and the scene-job it is of** —
/// the index into [`WhatThePageHandsOver::scene_jobs`]. A first-frame check
/// sends two jobs of one scene-job (`Rendering` S13). In the act's own
/// enumerated order, one per entry of [`WhatTheActCameTo::TheseJobs`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AJobAsSent {
    pub job: Identity,
    pub of_the_scene_job: usize,
}

/// **[`the_act_is_performed`], with each job the act enumerated named** —
/// so `Rendering` S10's telling can say which scenes were submitted, which
/// were not and which Smoozy could not account for, and S11's *Submit the
/// remaining scenes* can compose its fresh act from the rest (the 38's piece
/// 2). **Nothing where the act enumerated no job.**
pub fn the_act_is_performed_naming_its_jobs<C: AHeldConnection>(
    given: WhatThePageHandsOver,
    sessions: &ThisServersSessions<C>,
    the_run: &TheRunsTransfers,
    record: &dyn TheRecordAnUploadWrites,
) -> (WhatTheActCameTo, Vec<AJobAsSent>) {
    let in_flight = an_act_is_begun(&given);
    the_act_is_performed_in_flight(given, sessions, the_run, record, in_flight)
}

/// [`the_act_is_performed_naming_its_jobs`], **for an act whose
/// *Submitting…* row was begun at the press** ([`an_act_is_begun`]) — the
/// Creator's seventh notes, piece Z7. The row learns the act's identities as
/// `Rendering` S3 issues them and how the act ended.
pub fn the_act_is_performed_in_flight<C: AHeldConnection>(
    given: WhatThePageHandsOver,
    sessions: &ThisServersSessions<C>,
    the_run: &TheRunsTransfers,
    record: &dyn TheRecordAnUploadWrites,
    mut in_flight: TheActInFlight,
) -> (WhatTheActCameTo, Vec<AJobAsSent>) {
    let (came_to, named) = the_act_runs(given, sessions, the_run, record, &in_flight);
    in_flight.it_ended(&came_to);
    (came_to, named)
}

fn the_act_runs<C: AHeldConnection>(
    given: WhatThePageHandsOver,
    sessions: &ThisServersSessions<C>,
    the_run: &TheRunsTransfers,
    record: &dyn TheRecordAnUploadWrites,
    in_flight: &TheActInFlight,
) -> (WhatTheActCameTo, Vec<AJobAsSent>) {
    let line = the_line_for(&given.server.local_server);
    let _serialised = line.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    // **`Rendering` S13: two jobs only where the range holds more than one
    // frame** — the first frame as its own job, and **the whole range, its
    // first frame included,** as a second job held behind it (decision C,
    // Creator-stated 2026-10-02). The range is
    // [`the_range_the_check_splits`]'s. **The second job carries the index of
    // its first**, whose identity its request names as the job it is held
    // behind. **`Rendering` W4's third way on — submit with the first-frame
    // check — reaches this composition unchanged**: the band's box and the
    // strip's switch hand over the same flag.
    let mut the_jobs: Vec<(Payload, Option<usize>)> = Vec::new();
    // **Which scene-job each job is of**, beside `the_jobs` and in its order.
    let mut of_the_scene_job: Vec<usize> = Vec::new();
    for (scene_job, payload) in given.scene_jobs.iter().enumerate() {
        if !given.the_first_frame_check {
            the_jobs.push((payload.clone(), None));
            of_the_scene_job.push(scene_job);
            continue;
        }
        let not_known = WhatTheActCameTo::TheFirstFrameIsNotKnown {
            a_scene_can_be_ticked: given.the_reading_in_force.is_some() && payload.scene.is_none(),
        };
        let Some(asked) = the_range_the_check_splits(payload, given.the_reading_in_force.as_ref()) else {
            return (not_known, Vec::new());
        };
        match smoozy_rendering::submission::the_first_frame_split(&asked.explicit_list()) {
            None => return (not_known, Vec::new()),
            // **One frame is one job, as it was asked** — a range of one typed,
            // or the file's own of one, which the payload already says.
            Some(smoozy_rendering::submission::TheFirstFrameSplit::OneJobWithNoHold { .. }) => {
                the_jobs.push((payload.clone(), None));
                of_the_scene_job.push(scene_job);
            }
            Some(smoozy_rendering::submission::TheFirstFrameSplit::TwoJobs {
                the_first_frame, ..
            }) => {
                let mut first = payload.clone();
                first.frame_range = Some(FrameForm::List(the_first_frame));
                // **`Render Settings` Y1b, `Rendering` S13 and `Rendering` L10c: the check
                // alone is marked**, so where the job's only output is a video
                // the server writes this one frame as a still picture and the
                // check has a preview. The second job is left unmarked and
                // writes the video as the file asks. Added 2026-10-27, the
                // nineteenth notes, notes 3 and 9.
                first.the_check_is_written_as_a_picture = true;
                // **The second job is the range exactly as the user asked it**,
                // in the form they asked it in, so the entry's *Frames* reads
                // *1–240* and a simulation never baked renders one unbroken run
                // from the range's start (`Rendering` S13, decision C,
                // 2026-10-02). *From step 10c's eleventh-pass fix piece 8 until
                // then it was the rest of the range, one frame on.* The range
                // is still written down where it came from the reading, so the
                // two jobs split the same frames.
                let mut the_whole = payload.clone();
                the_whole.frame_range = Some(asked);
                the_jobs.push((first, None));
                the_jobs.push((the_whole, Some(the_jobs.len() - 1)));
                of_the_scene_job.push(scene_job);
                of_the_scene_job.push(scene_job);
            }
        }
    }

    // **`Rendering` S2: one act, one group** — issued where the act names
    // more than one scene.
    let Ok(group) = the_group_identity(given.scene_jobs.len()) else {
        return (
            WhatTheActCameTo::TheseJobs(vec![
                WhatBecameOfOneJob::ThisComputerCouldNotComposeIt;
                the_jobs.len()
            ]),
            Vec::new(),
        );
    };

    // **`Rendering` S3: every identity is issued before anything is sent.**
    let mut identities: Vec<TheIdentities> = Vec::new();
    for _ in &the_jobs {
        match issue_the_identities() {
            Ok(issued) => identities.push(issued),
            Err(_) => {
                return (
                    WhatTheActCameTo::TheseJobs(vec![
                        WhatBecameOfOneJob::ThisComputerCouldNotComposeIt;
                        the_jobs.len()
                    ]),
                    Vec::new(),
                )
            }
        }
    }
    let order: Vec<Identity> = identities.iter().map(|issued| issued.job.clone()).collect();
    in_flight.its_jobs_are(&order);
    // **`Rendering` S13's *writes into*, once both identities are issued**: the
    // check's payload names its range job, so the check writes into the range
    // job's output folder and marks every file it writes (`Job files and
    // outputs` P18a). The twenty-first notes, note 6.
    for at in 0..the_jobs.len() {
        if let Some(first_at) = the_jobs[at].1 {
            the_jobs[first_at].0.the_check_writes_into = Some(identities[at].job.as_text());
        }
    }
    let named: Vec<AJobAsSent> = order
        .iter()
        .zip(&of_the_scene_job)
        .map(|(job, scene_job)| AJobAsSent {
            job: job.clone(),
            of_the_scene_job: *scene_job,
        })
        .collect();

    // **What this run knows of each first-frame check it sends**, written
    // before anything is sent — see [`TheFirstFrameCheck`].
    let mut purposes = BTreeMap::new();
    for (at, (the_whole, first_at)) in the_jobs.iter().enumerate() {
        let Some(first_at) = *first_at else {
            continue;
        };
        let (Some(the_first_frame), frames_held) = (
            the_jobs[first_at]
                .0
                .frame_range
                .as_ref()
                .and_then(|first| first.explicit_list().first().copied()),
            the_whole
                .frame_range
                .as_ref()
                .map(|range| range.explicit_list().len())
                .unwrap_or_default(),
        ) else {
            continue;
        };
        // `Queue & History` P5a: each job's provisional write keeps its own
        // copy, before any bytes reach the machine. No second hold is made.
        let purpose = crate::record::job_records::FirstFramePurpose {
            check_job: identities[first_at].job.as_text(),
            range_job: identities[at].job.as_text(),
            first_frame: the_first_frame,
            submitted_range: the_whole.frame_range.clone().expect("the split has a range"),
        };
        purposes.insert(purpose.check_job.clone(), purpose.clone());
        purposes.insert(purpose.range_job.clone(), purpose);
        a_first_frame_check_is_sent(
            &identities[at].job,
            TheFirstFrameCheck {
                the_first_job: identities[first_at].job.clone(),
                the_first_frame,
                frames_held: u32::try_from(frames_held).unwrap_or(u32::MAX),
            },
        );
    }

    // **`Job files and outputs` M1's four-value identity at submission**, read
    // once for the file the act is of — **in its turn at the one link site on
    // this server** (step 10c's eleventh-pass fix piece 4b), so a send or the
    // page reading beside it can never hand it another file's values.
    let the_server = given.server.local_server.as_text();
    let four_values =
        waited_for(|| async { the_four_values_of(sessions, &the_server, &given.the_blend).await });

    // **`Dependency Upload & Remap` M10: the pairs, composed once for the file
    // the act is of** — every scene-job of one act renders the same file, so
    // each carries the same pairs in a material of its own. **A record that
    // would not answer sends nothing**: a material composed empty over answers
    // the record holds is the silent wrong render `Dependency Upload & Remap`
    // M10 exists to prevent.
    let Ok(remap) = the_remap_is_composed(&given, record) else {
        return (
            WhatTheActCameTo::TheseJobs(vec![
                WhatBecameOfOneJob::ThisComputerCouldNotComposeIt;
                the_jobs.len()
            ]),
            named,
        );
    };

    let the_submission_class = the_submission_class();
    let what_became: std::cell::RefCell<BTreeMap<String, WhatBecameOfOneJob>> = Default::default();
    let the_outcome_records: std::cell::RefCell<BTreeMap<String, Identity>> = Default::default();
    let mut at = 0usize;

    // **`Rendering` S4a within one act: `Rendering`'s own walk**, which begins
    // no unit until the previous one's outcome record reads ended. Every unit
    // here is performed before its future is handed back, so the walk is
    // driven to its end at once.
    at_once(walk_the_acts_units(
        TheActsUnits {
            in_the_enumerated_order: &order,
        },
        |job| {
            let (payload, first_at) = the_jobs[at].clone();
            let issued = identities[at].clone();
            let typed = the_typed_frames_of_this_job(&given, &payload, of_the_scene_job[at]);
            at += 1;
            // **`Rendering` S13: the second job is held behind the first**, and
            // only where the first is in the machine's line. **A hold whose
            // dependency the machine never took would never end**: nothing
            // removes a job record that was never written, so `Queue &
            // History` H9's drop could never fire, and `Rented-server runtime`
            // U21 leaves such a hold standing for ever. **Whether the rest is
            // dropped once the first job has run is the machine's alone**
            // (`Queue & History` H7's predicate, evaluated at `Rented-server
            // runtime` U19's moment); nothing here decides it.
            let held_behind = first_at.map(|first_at| {
                let first = &identities[first_at].job;
                let in_the_line = what_became.borrow().get(&first.as_text())
                    == Some(&WhatBecameOfOneJob::ItIsInTheLine);
                (first.clone(), in_the_line)
            });
            // **`Server Setup` R1c, judged for this job as it is composed**:
            // its resolved device, the engine it will use, the card make the
            // manifest carries, and the version its pick answered, against the
            // readings taken at the press. Held-job piece A, 2026-10-12; the
            // engine since `Server Setup` R1c's bullet of 2026-11-02.
            let held_for_the_driver = given.the_driver_at_submission.holds_this_job(
                the_device_ask(&given, &payload).the_choice(),
                the_engine_at_submission(&given, &payload),
                &given.the_card_make,
                smoozy_server_setup::blender::Version::parse(&payload.the_manifests_blender().version),
            );
            let (became, ended, outcome_record) = one_scene_job(
                &given,
                sessions,
                the_run,
                record,
                the_submission_class,
                group.as_ref(),
                &issued,
                payload,
                held_behind,
                held_for_the_driver,
                &four_values,
                &remap,
                purposes.get(&issued.job.as_text()).cloned(),
                typed,
            );
            what_became.borrow_mut().insert(job.as_text(), became);
            if let Some(outcome_record) = outcome_record {
                the_outcome_records
                    .borrow_mut()
                    .insert(job.as_text(), outcome_record);
            }
            std::future::ready(ended)
        },
        |previous| {
            let outcome_record = the_outcome_records
                .borrow()
                .get(&previous.as_text())?
                .clone();
            the_outcome_record(record, &outcome_record)
        },
    ));

    let mut what_became = what_became.into_inner();
    (
        WhatTheActCameTo::TheseJobs(
            order
                .iter()
                .map(|job| {
                    what_became
                        .remove(&job.as_text())
                        .unwrap_or(WhatBecameOfOneJob::NotAttemptedBehindAnUnaccountedOne)
                })
                .collect(),
        ),
        named,
    )
}

/// **`Queue & History` P5b: which job of the act keeps the frame text the user
/// typed** — the one whose range is its scene-job's range as the page composed
/// it. **A first-frame check does not**: its one frame is Smoozy's split
/// (`Rendering` S13), not anything typed, so its entry draws the numbers form.
/// The held whole range keeps it, being the range exactly as asked. A text
/// handed over with no range under it is kept by no job. The eighteenth notes,
/// piece 7.
fn the_typed_frames_of_this_job(
    given: &WhatThePageHandsOver,
    payload: &Payload,
    of_the_scene_job: usize,
) -> Option<String> {
    let asked = given.scene_jobs.get(of_the_scene_job)?.frame_range.as_ref()?;
    (payload.frame_range.as_ref() == Some(asked))
        .then(|| given.the_typed_frames.clone())
        .flatten()
}

/// **A future whose every step is already done, driven to its end here.**
fn at_once<F: Future>(future: F) -> F::Output {
    let mut future = std::pin::pin!(future);
    let mut context = std::task::Context::from_waker(std::task::Waker::noop());
    loop {
        if let std::task::Poll::Ready(value) = future.as_mut().poll(&mut context) {
            return value;
        }
        std::thread::yield_now();
    }
}

/// **`Rendering` §7's submission class, by the name its declaration carries.**
/// Read from the one declared order the run's point is made over, so the name
/// is spelled nowhere here.
fn the_submission_class() -> &'static str {
    crate::downloading::the_declared_order()
        .into_iter()
        .find(|class| class.declared_by == "Rendering")
        .map(|class| class.name)
        .expect("the one declared order carries `Rendering` §7's class, by construction")
}

fn the_outcome_record(
    record: &dyn TheRecordAnUploadWrites,
    identity: &Identity,
) -> Option<Recorded> {
    let mut read = None;
    record.with_the_record(&mut |connection| {
        read = OutcomeRecords::over(connection)
            .read(identity)
            .ok()
            .flatten();
    });
    read
}

/// **A unit's outcomes, written as they became true** (`State, identity and
/// recovery` §9.4 P5 item 2).
fn the_units_ended(
    record: &dyn TheRecordAnUploadWrites,
    identity: &Identity,
    ended: &[(&str, UnitOutcome)],
) {
    record.with_the_record(&mut |connection| {
        let mut keeper = OutcomeRecords::over(connection);
        for (unit, outcome) in ended {
            let _ = keeper.a_unit_ended(identity, unit, outcome.clone());
        }
    });
}

/// **One scene-job, from its outcome record to what the user is told.**
#[allow(clippy::too_many_arguments)]
fn one_scene_job<C: AHeldConnection>(
    given: &WhatThePageHandsOver,
    sessions: &ThisServersSessions<C>,
    the_run: &TheRunsTransfers,
    record: &dyn TheRecordAnUploadWrites,
    the_submission_class: &'static str,
    group: Option<&Identity>,
    issued: &TheIdentities,
    payload: Payload,
    held_behind: Option<(Identity, bool)>,
    held_for_the_driver: bool,
    four_values: &smoozy_render_settings::readings::FourValuesNow,
    remap: &TheRemapComposed,
    first_frame_purpose: Option<crate::record::job_records::FirstFramePurpose>,
    typed_frames: Option<String>,
) -> (WhatBecameOfOneJob, HowTheUnitEnded, Option<Identity>) {
    let nothing_ran = |why: &str| HowTheUnitEnded::ItStoppedAt {
        step: THE_ORDERED_UNIT[0],
        the_later_were_not_attempted: THE_ORDERED_UNIT[1..].to_vec(),
        why: why.to_string(),
    };
    let job = &issued.job;

    // **The outcome record first**: the unit's three units enumerated durably
    // before the first is attempted — `Rendering` §7's first row.
    let mut outcome_record = None;
    record.with_the_record(&mut |connection| {
        outcome_record = OutcomeRecords::over(connection)
            .enumerate(ONE_SCENE_JOBS_SUBMISSION, &THE_ORDERED_UNIT)
            .ok();
    });
    let Some(outcome_record) = outcome_record else {
        return (
            WhatBecameOfOneJob::ThisComputerCouldNotComposeIt,
            nothing_ran("the outcome record could not be written"),
            None,
        );
    };
    let refused = |became: WhatBecameOfOneJob, why: &str| {
        the_units_ended(
            record,
            &outcome_record,
            &THE_ORDERED_UNIT
                .iter()
                .map(|unit| {
                    (
                        *unit,
                        UnitOutcome::NotAttempted {
                            stopped_by: why.to_string(),
                        },
                    )
                })
                .collect::<Vec<_>>(),
        );
        (became, nothing_ran(why), Some(outcome_record.clone()))
    };

    // **`Rendering` S13's hold instruction** — `Rendering`'s own composition,
    // with its reason in the placing document's words (`Rented-server runtime`
    // U2). ~~Refused here as *"the held second job has no field for its
    // hold"*~~ from step 10c piece 6 until the eleventh-pass fix piece 8,
    // though step 10c piece 5 had named both fields.
    //
    // **`Rendering` S18a: a job `Server Setup` R1c holds for the driver
    // carries that hold, and a job the check already holds carries the
    // check's and no second one.** Held-job piece A, 2026-10-12.
    let the_hold = match held_behind {
        None => smoozy_rendering::submission::the_hold_this_job_carries(job, None, held_for_the_driver),
        Some((first, true)) => {
            smoozy_rendering::submission::the_hold_this_job_carries(job, Some(&first), held_for_the_driver)
        }
        Some((_, false)) => {
            return refused(
                WhatBecameOfOneJob::NotSentBehindAFirstFrameThatWasNot,
                "the first frame's job is not in the machine's line",
            );
        }
    };
    // **A file read on the page is submitted like any other** — the runner's
    // ruling of 2026-09-29, step 10c's four small things after *Read the
    // file*. Until then a standing reading was refused here, on the ground
    // that nothing in the running application could take one; dependency
    // piece 3 made that untrue. The manifest below says no reading either way.
    let smoozy_render_settings::readings::FourValuesNow::Answered(four_values) = four_values else {
        return refused(
            WhatBecameOfOneJob::TheFileOnTheServerCouldNotBeRead,
            "the file's four values could not be read",
        );
    };

    // **`Rendering` S3a: the one gate between the act and the unit.**
    let the_target_build = payload.the_manifests_blender();
    if let TheGate::Refused { engine, build } = the_engine_gate(
        &payload,
        &given.what_the_desktop_holds_about_the_build,
        &the_target_build,
    ) {
        return refused(
            WhatBecameOfOneJob::TheBuildLacksTheEngine { engine, build },
            "the build does not carry the engine picked",
        );
    }

    let Some((paths, manifest)) = the_manifest_of(given, group, job, &payload, four_values.clone())
    else {
        return refused(
            WhatBecameOfOneJob::ThisComputerCouldNotComposeIt,
            "the job's paths will not fit",
        );
    };

    // **`Queue & History` N10 to `Queue & History` N22's record, and the kept setup beside it.**
    let whole = the_record_at_submission(given, group, job, &payload, &the_target_build);
    let provisional = AProvisionalRecord {
        job: job.clone(),
        server: given.server.local_server.clone(),
        group: group.cloned(),
        label: given.label.clone().unwrap_or_default(),
    };
    let mut writer = TheWriterHere {
        record,
        the_whole_record: whole,
        the_kept_setup: payload.clone(),
        first_frame_purpose,
        typed_frames,
        the_reporting_choice: given.reporting.0.to_string(),
        the_choice_is_on: given.the_download_as_it_renders_choice,
        the_arrangement: what_the_submission_chose(given, job),
        // **`Dependency Upload & Remap` K9: the job's resolved set, written
        // with the provisional record** — the machine locations the material
        // names, and the material's own. **Only where the row names an
        // installation**: `Dependency Upload & Remap` K9's set is read by
        // machine, and a machine is its
        // server row under its installation.
        the_resolved_set: given.installation.clone().map(|installation| {
            (
                MachineKey::of(given.server.local_server.clone(), installation),
                MachineLocation::composed(given.the_blend.as_bytes().to_vec()),
                ResolvedSet {
                    locations: remap.the_machine_locations_it_names(),
                    remap_material: RemapMaterial::location_in(&given.layout, job),
                },
            )
        }),
    };

    let sender = TheOneSender {
        sessions,
        package_area: &given.package_area,
        the_submission_command: issued.submission_command.clone(),
        the_unit: TheUnitInHand {
            job: job.clone(),
            paths,
            manifest,
            remap: TheRemapPlacement {
                sessions,
                the_run,
                class: the_submission_class,
                layout: given.layout.clone(),
                // **`Dependency Upload & Remap` M10: composed on the desktop,
                // from the record, at submission** — [`the_remap_is_composed`],
                // once for the act. ~~The record's located answers have no
                // writer in the running application, so the material carries
                // no pairs~~: true until 2026-09-29, when dependency pieces 4 to
                // 6 gave the record its writer, and false from then until piece
                // 7 composed from it. **`Dependency Upload & Remap` M6: composed
                // for the version the job renders with, and naming it.**
                // `Dependency Upload & Remap` M22c's copies ride it.
                material: RemapMaterial::composed(
                    job,
                    &the_pick_as_an_identity(&payload),
                    remap.pairs.clone(),
                )
                .with_the_copies_owed(remap.copies_owed.clone()),
                server_row: given.server.local_server.clone(),
                installation: given.installation.clone(),
                run: given.run.clone(),
                record,
                the_temporary_area: given.the_temporary_area.clone(),
            },
            the_run,
            the_submission_class,
        },
        the_label: Mutex::new(None),
        the_hold,
        what_the_supervisor_said: Mutex::new(None),
        how_the_unit_ended: Mutex::new(None),
    };

    // Keep the exact paths and bytes before any of them can reach the server.
    // This is a candidate only: an unknown or accepted job owes no cleanup.
    if let Some(installation) = &given.installation {
        let material = crate::record::submission_material::Material {
            job: job.clone(),
            server: given.server.local_server.clone(),
            installation: installation.clone(),
            material_folder: sender.the_unit.paths.job_record_directory.clone(),
            material: sender.the_unit.manifest.as_written(),
            remap_path: String::from_utf8(
                RemapMaterial::location_in(&given.layout, job)
                    .as_bytes()
                    .to_vec(),
            )
            .expect("a path composed from the recorded layout"),
            remap: sender.the_unit.remap.material.as_written(),
            output_folder: sender
                .the_unit
                .manifest
                .job_output_folder
                .the_path_it_names()
                .map(str::to_string),
            done: [false; 3],
        };
        let mut kept = false;
        record.with_the_record(&mut |connection| {
            kept = material.keep(connection).is_ok();
        });
        if !kept {
            return refused(
                WhatBecameOfOneJob::TheRecordWasRefused,
                "the submission's material record was refused",
            );
        }
    }

    // **`Queue & History` Y3: the record first, then the send.**
    let left = submit(&mut writer, &sender, &provisional);
    let ended = sender.the_unit_ended();
    let (became, units): (WhatBecameOfOneJob, Vec<(&str, UnitOutcome)>) = match (&left, &ended) {
        (WhatTheSubmissionLeft::TheRecordWasRefusedSoNothingWasSent { .. }, _) => {
            return refused(
                WhatBecameOfOneJob::TheRecordWasRefused,
                "the local record refused it",
            );
        }
        (WhatTheSubmissionLeft::ItWasAcknowledged, _) => (
            WhatBecameOfOneJob::ItIsInTheLine,
            THE_ORDERED_UNIT
                .iter()
                .map(|unit| (*unit, UnitOutcome::Done))
                .collect(),
        ),
        (_, Some(HowTheUnitEnded::ItStoppedAt { step, .. })) if *step != THE_ORDERED_UNIT[2] => {
            // **Nothing of the job record was sent**, so the machine will never
            // act on this identity: the provisional record goes, and a job
            // that never reached the machine leaves no trace.
            let _ = writer.remove_it(job);
            let at = THE_ORDERED_UNIT
                .iter()
                .position(|unit| unit == step)
                .unwrap_or(0);
            let mut units: Vec<(&str, UnitOutcome)> = Vec::new();
            for (position, unit) in THE_ORDERED_UNIT.iter().enumerate() {
                let outcome = match position.cmp(&at) {
                    std::cmp::Ordering::Less => UnitOutcome::Done,
                    std::cmp::Ordering::Equal => UnitOutcome::Failed,
                    std::cmp::Ordering::Greater => UnitOutcome::NotAttempted {
                        stopped_by: step.to_string(),
                    },
                };
                units.push((*unit, outcome));
            }
            (
                WhatBecameOfOneJob::ItStoppedBeforeTheJobRecord { the_step: *step },
                units,
            )
        }
        _ => match sender.what_the_supervisor_said() {
            // **A definite answer: the machine did not write the record.**
            // The material it published is never a job, so `Rendering` S4b's
            // debt is taken now rather than after an ask that cannot be read.
            Some(WhatTheSupervisorSaid::ItDoesNotTakeJobs) => {
                let _ = writer.remove_it(job);
                (
                    WhatBecameOfOneJob::TheServerDoesNotTakeJobs,
                    vec![
                        (THE_ORDERED_UNIT[0], UnitOutcome::Done),
                        (THE_ORDERED_UNIT[1], UnitOutcome::Done),
                        (THE_ORDERED_UNIT[2], UnitOutcome::Failed),
                    ],
                )
            }
            // **`Rendering` S9: the machine is asked, never guessed about.**
            _ => match resolve(&mut writer, &sender, job) {
                HowItResolved::TheMachineHoldsIt => (
                    WhatBecameOfOneJob::ItIsInTheLine,
                    THE_ORDERED_UNIT
                        .iter()
                        .map(|unit| (*unit, UnitOutcome::Done))
                        .collect(),
                ),
                HowItResolved::TheMachineDoesNotHoldIt => (
                    WhatBecameOfOneJob::ItStoppedBeforeTheJobRecord {
                        the_step: THE_ORDERED_UNIT[2],
                    },
                    vec![
                        (THE_ORDERED_UNIT[0], UnitOutcome::Done),
                        (THE_ORDERED_UNIT[1], UnitOutcome::Done),
                        (THE_ORDERED_UNIT[2], UnitOutcome::Failed),
                    ],
                ),
                // **Unaccounted: the third unit keeps no outcome**, which is
                // what ends the act's run (`Rendering` S4a) and what the later
                // connection's resolution reads.
                HowItResolved::TheMachineCouldNotBeAsked { .. }
                | HowItResolved::TheLocalWriteWasRefused { .. } => (
                    WhatBecameOfOneJob::NotKnown,
                    vec![
                        (THE_ORDERED_UNIT[0], UnitOutcome::Done),
                        (THE_ORDERED_UNIT[1], UnitOutcome::Done),
                    ],
                ),
            },
        },
    };
    the_units_ended(record, &outcome_record, &units);

    // The same record removal that proves non-acceptance creates the debt.
    // Reconciliation takes that path too (`JobRecords::the_resolution`).

    // **`Download` R1's arrangement is not made here.** It was, until
    // 2026-09-27, and a quit while the page still said *Submitting.* then lost
    // it: the machine held the job and rendered it, and its frames never came
    // home (the seventh pass on Vast, fault 1). **It is written with the
    // provisional record, before anything is sent** — [`TheWriterHere`] — and
    // every branch above that removes the record takes it with the record.

    let ended = ended.unwrap_or_else(|| nothing_ran("the unit did not run"));
    let ended = match became {
        // **A unit that was left unaccounted is not *ended* for `Rendering`
        // S4a's walk** — its outcome record holds no third outcome — so the
        // walk stops behind it whatever the unit's own value says.
        WhatBecameOfOneJob::ItIsInTheLine => HowTheUnitEnded::ItIsInTheLine,
        _ => ended,
    };
    (became, ended, Some(outcome_record))
}

/// **What `Download` R1's arrangement would hold for this job**, composed from
/// what the page handed over. Whether one is made is the strip's choice, and
/// the making is [`TheWriterHere`]'s, with the provisional record.
///
/// **Nothing is said to the user about a refusal here, and the telling is the
/// page's — before this act rather than after it.** `Download` R1's *there is
/// nowhere to put it* is the one refusal the user can act on, and it is
/// knowable before a byte moves: it fires exactly where the strip could name
/// no folder. **So `Design/Render Settings § Submitting` puts the line on the
/// submit strip** — `the_frames_have_nowhere_to_go` in
/// [`crate::surface::render_settings`], the drawing's `res-nofolder` — where
/// the user is standing and where the control that fixes it already is.
/// ~~Until 2026-09-20 nothing anywhere said it~~, and a job rendered with the
/// switch on whose frames were never coming home.
///
/// **The other two refusals are not the user's to act on** — the record would
/// not take it, and the choice was off — and neither stops the submission.
fn what_the_submission_chose(
    given: &WhatThePageHandsOver,
    job: &Identity,
) -> WhatTheSubmissionChose {
    WhatTheSubmissionChose {
        job: job.clone(),
        server_row: given.server.local_server.clone(),
        // **No folder named is no destination**, and `Download` R1's own maker
        // refuses it rather than this file inventing one here.
        destination: given.where_the_downloads_go.clone().unwrap_or_default(),
        narrowing: match &given.the_formats_chosen {
            Some(chosen) => TheNarrowing::TheseFormats(chosen.clone()),
            None => TheNarrowing::Everything,
        },
    }
}

/// The version pick as `Render Settings`' identity pair.
fn the_pick_as_an_identity(payload: &Payload) -> BlenderIdentity {
    let blender: TheBlenderToRenderBy = payload.the_manifests_blender();
    BlenderIdentity {
        path: blender.build_path,
        version: the_version_in(&blender.version),
    }
}

fn the_version_in(text: &str) -> smoozy_server_setup::blender::Version {
    let mut parts = text.split('.').map(|part| part.parse::<u32>().unwrap_or(0));
    smoozy_server_setup::blender::Version {
        major: parts.next().unwrap_or(0),
        minor: parts.next().unwrap_or(0),
        patch: parts.next().unwrap_or(0),
    }
}

/// **`Rendering` S5 and `Job files and outputs` M1: the manifest, composed on
/// the desktop**, with the paths it names.
fn the_manifest_of(
    given: &WhatThePageHandsOver,
    group: Option<&Identity>,
    job: &Identity,
    payload: &Payload,
    four_values: SourceReading,
) -> Option<(AJobsPaths, SubmissionManifest)> {
    let paths = AJobsPaths::composed(&given.layout, job, given.label.as_deref()).ok()?;
    let manifest = compose_the_manifest(ManifestInputs {
        job,
        group,
        payload,
        layout: &given.layout,
        // **`Job files and outputs` P21's second case, which is every job this
        // desktop submits**: the manifest carries no reading (`Job files and
        // outputs` R7's second bullet), so the render worker composes the
        // job's first map and chooses the job's folder with it, at render
        // start — where the file points (`Job files and outputs` P18), or the
        // fallback (`Job files and outputs` P19) in the server's chosen folder
        // as it stands now (`Job files and outputs` P23). **`Job files and
        // outputs` N3's name is composed here**, where the label is, and
        // carried. The Creator's second notes, piece 4.
        job_output_folder: smoozy_job_files::manifest::WhereTheJobOutputFolderIs::ChosenAtRenderStart {
            // **`Job files and outputs` P18a: a first-frame check carries its
            // range job's name**, so the two write into one folder; every other
            // job carries its own.
            the_folders_name: smoozy_job_files::names::job_output_folder_name(
                &the_job_whose_folder_it_writes_into(job, payload),
                given.label.as_deref(),
            ),
            the_servers_fallback_folder_at_submission: given
                .the_servers_fallback_folder
                .as_deref()
                .map(str::trim)
                .filter(|folder| !folder.is_empty())
                .map(str::to_string),
        },
        blend_path_on_the_machine: given.the_blend.clone(),
        file_identity_at_submission: four_values,
        // **`Job files and outputs` R7's second bullet: the manifest says so
        // and carries no map**, and the render worker takes the reading after
        // the load — **also where a reading stands on the page** (the runner's
        // ruling of 2026-09-29). The recorded-reading composition R7's first
        // bullet names is not built; a name the page anchored to its reading
        // is still checked against the reading in force at the attempt
        // (`Rendering` P1a), whatever its anchor says.
        recorded_reading: TheRecordedReading::ThereIsNone,
        reporting: given.reporting.clone(),
        card_make: given.the_card_make.clone(),
    });
    Some((paths, manifest))
}

/// **The job whose output folder a job writes into**: its range job, for a
/// first-frame check whose payload names one (`Rendering` S13, `Job files and
/// outputs` P18a); itself, for every other job. Added 2026-10-31, the
/// twenty-first notes, note 6.
pub fn the_job_whose_folder_it_writes_into(job: &Identity, payload: &Payload) -> Identity {
    payload
        .the_check_writes_into
        .as_deref()
        .and_then(|range| Identity::read_back(smoozy_state::identities::IdentityKind::Job, range))
        .unwrap_or_else(|| job.clone())
}

/// **The local job record at submission** — `Queue & History` N10 to `Queue & History` N22,
/// every fact of it composed from what the page handed over.
fn the_record_at_submission(
    given: &WhatThePageHandsOver,
    group: Option<&Identity>,
    job: &Identity,
    payload: &Payload,
    the_target_build: &TheBlenderToRenderBy,
) -> LocalJobRecord {
    LocalJobRecord {
        job: job.clone(),
        group: group.cloned(),
        server: given.server.clone(),
        // **`Queue & History` N16: the submitted moment is this computer's.**
        submitted_at: Some(Reading::taken((), SystemTime::now(), Clock::ThisComputer)),
        label: given.label.clone().filter(|label| !label.is_empty()),
        blend_path_on_the_machine: Some(given.the_blend.clone()),
        scene_name: payload
            .scene
            .as_ref()
            .map(|scene| scene.name.as_text().to_string()),
        // **`Queue & History` N22's mark: the submission named no scene.**
        submitted_with_no_scene_named: payload.scene.is_none(),
        resolution_percentage_typed: payload.resolution_percentage,
        picture_size_from_the_read: None,
        main_output_from_the_read: None,
        blender: TheBlenderTheEntryKeeps {
            pick: smoozy_render_settings::payload::VersionPick {
                build_path: the_target_build.build_path.clone(),
                version: the_target_build.version.clone(),
            },
            origin: given.the_version_origin,
        },
        // **`Queue & History` E12a: the user's own answer, else the file's own
        // as the reading on the page gave it for the job's scene, else the
        // file's own unread.** A job naming no scene renders the scene the
        // file was saved showing, and the reading does not say which that is.
        device_asked_for: the_device_ask(given, payload),
        // **`Queue & History` N14, composed by `Render Settings` Y1a**: the
        // file's value beside each override, and the file's own frames,
        // samples, percentage and engine where they were left alone (as
        // corrected 2026-10-07), from the reading standing at submission, and
        // absent where none stood.
        the_files_values_beside_the_overrides: smoozy_render_settings::payload::the_companion(
            payload,
            given.the_reading_in_force.as_ref(),
            payload.scene.as_ref().map(|scene| &scene.name),
        ),
        dependency_check: given.dependency_check,
        carries_on_from: given.carries_on_from.clone(),
        disturbances: Vec::new(),
        attempts_where_the_file_changed: Vec::new(),
        the_frames: smoozy_queue_history::record::TheFramesTheReadFound::default(),
        standing: Standing::Provisional,
        the_render_start_has_been_learned: false,
        what_the_note_carried: WhatTheNoteCarried::default(),
        // **`Job files and outputs` P21's second case, every job this desktop
        // submits**: the job has no folder until the render worker chooses it,
        // and `Queue & History` N9 brings it here (`Queue & History` Y8).
        the_job_output_folder: smoozy_queue_history::record::TheJobOutputFolderTheRecordHolds::ChosenAtRenderStart,
        // **`Queue & History` N23's mark is written only before a release**
        // (`Server Setup` R1c), never at submission.
        the_driver_was_below_the_bar: None,
    }
}

/// **`Queue & History` E12a: the user's own answer, else the file's own as the
/// reading on the page gave it for the job's scene, else the file's own
/// unread.** A job naming no scene renders the scene the file was saved
/// showing, and the reading does not say which that is. **Read for the local
/// record and for `Server Setup` R1c alike**, so the device a job is held for
/// is the one its entry says it asked for.
fn the_device_ask(given: &WhatThePageHandsOver, payload: &Payload) -> TheDeviceAsk {
    match (payload.device, the_jobs_scene_as_read(given, payload)) {
        (Some(choice), _) => TheDeviceAsk::TheUsersOwn(choice),
        (None, Some(scene)) => TheDeviceAsk::TheFilesOwnAsTheReadingInForceGaveIt(
            smoozy_render_settings::device::the_resolved_choice(None, scene),
        ),
        (None, None) => TheDeviceAsk::TheFilesOwnUnread,
    }
}

/// **The job's scene as the reading standing on the page holds it**, found by
/// the name's own bytes.
fn the_jobs_scene_as_read<'a>(
    given: &'a WhatThePageHandsOver,
    payload: &Payload,
) -> Option<&'a smoozy_render_settings::reading::SceneReading> {
    let named = payload.scene.as_ref()?;
    given
        .the_reading_in_force
        .as_ref()?
        .settings
        .scenes
        .iter()
        .find(|scene| scene.name.as_bytes() == named.name.as_bytes())
}

/// **`Server Setup` R1c's bullet of 2026-11-02: the engine this job will use**,
/// from the same reading [`the_device_ask`] reads for its device.
fn the_engine_at_submission(
    given: &WhatThePageHandsOver,
    payload: &Payload,
) -> crate::servers::the_driver_swap::TheEngineAtSubmission {
    crate::servers::the_driver_swap::TheEngineAtSubmission::of(
        payload.engine.as_ref().map(|entry| entry.pick),
        given
            .the_reading_in_force
            .as_ref()
            .and_then(|reading| Some((the_jobs_scene_as_read(given, payload)?, reading))),
    )
}

/// **`Queue & History`'s record seam, through the application's own record**
/// — the lock taken for the write alone, never across the submission.
///
/// **`Download` R1's arrangement is written here, in the same act as the
/// provisional record and before anything is sent** (`Queue & History` Y3's
/// *the record first, then the send*). `Download` §12 says why it is written
/// durably at all: *a crash a moment later leaves an arrangement that
/// resumes*. Written after the machine's answer, as it was until 2026-09-27,
/// it was defeated by exactly that: a quit on *Submitting.* left a job the
/// machine rendered and no arrangement to bring its frames home. **Now a quit
/// at any moment after this write leaves both**, and the unaccounted record is
/// asked about at the next connection (`State, identity and recovery` V1 step
/// 2), where the arrangement resumes under `Download` R12.
///
/// **The record first and the arrangement second, in that order**, because
/// the arrangement rides the job's entry (`Download` R3): a record with no
/// arrangement is a job whose frames are not fetched by themselves, and an
/// arrangement with no record would be walked for a job that leaves no trace.
///
/// **Every removal of the record takes the arrangement with it** — the three
/// branches of [`one_scene_job`] that remove the provisional record, and
/// `Queue & History`'s `resolve` — **because the removal is
/// `JobRecords::remove`'s, and that removes the arrangement in the same
/// transaction** (`Download` R3: *a cancelled job's provisional record removal
/// takes the arrangement with it*).
struct TheWriterHere<'a> {
    record: &'a dyn TheRecordAnUploadWrites,
    the_whole_record: LocalJobRecord,
    the_kept_setup: Payload,
    first_frame_purpose: Option<crate::record::job_records::FirstFramePurpose>,
    /// **`Queue & History` P5b's typed frame text**, for this job's record
    /// alone ([`the_typed_frames_of_this_job`]).
    typed_frames: Option<String>,
    /// **`Telegram` V1c: the manifest's reporting field, as JSON text**, kept
    /// on the record (version 55) so the queue entry reads the choice.
    the_reporting_choice: String,
    /// The submit strip's *Download frames as they render*.
    the_choice_is_on: bool,
    the_arrangement: WhatTheSubmissionChose,
    /// **`Dependency Upload & Remap` K9's set**, with the machine and the
    /// manifest's `.blend` it is read beside.
    the_resolved_set: Option<(MachineKey, MachineLocation, ResolvedSet)>,
}

impl WritesTheProvisionalRecord for TheWriterHere<'_> {
    fn write_it(&mut self, provisional: &AProvisionalRecord) -> Result<(), String> {
        let mut written = Err("the local record would not open".to_string());
        let (whole, kept) = (self.the_whole_record.clone(), self.the_kept_setup.clone());
        let (on, chose) = (self.the_choice_is_on, self.the_arrangement.clone());
        let the_set = self.the_resolved_set.clone();
        let purpose = self.first_frame_purpose.clone();
        let typed = self.typed_frames.clone();
        let reporting = self.the_reporting_choice.clone();
        self.record.with_the_record(&mut |connection| {
            written = TheSubmissionsWriter::of(connection, whole.clone(), kept.clone())
                .with_first_frame_purpose(purpose.clone())
                .with_the_typed_frames(typed.clone())
                .write_it(provisional);
            // **`Telegram` V1c: the choice the job was submitted with**, kept
            // for the queue entry's control. A refusal costs only the control's
            // reading, so it never takes the submission with it.
            if written.is_ok() {
                let _ = crate::record::job_records::JobRecords::over(connection)
                    .the_reporting_choice_is_written(&whole.job, &reporting);
            }
            // **`Dependency Upload & Remap` K9: written with the provisional
            // record, before anything is sent**, and immutable from here. **A
            // set the record refuses takes the record with it**: a job whose
            // set is not kept is one `File Browser`'s live-job warning cannot
            // name, and `Dependency Upload & Remap` V6's warning before a touch
            // is the one thing the set is
            // for. So it is `Queue & History` Y3's refusal — nothing is sent.
            if written.is_ok() {
                if let Some((machine, blend, set)) = &the_set {
                    if TheDependencyRecord::over(connection)
                        .a_jobs_resolved_set_is_written(&provisional.job, machine, blend, set)
                        .is_err()
                    {
                        let _ = TheSubmissionsWriter::of(connection, whole.clone(), kept.clone())
                            .remove_it(&provisional.job);
                        written = Err("the job's resolved set was refused".to_string());
                    }
                }
            }
            // **Only over a record that was written.** A refused record sends
            // nothing, so there is no job for an arrangement to ride. **Its
            // own answer is not read**: none of `Download` R1's refusals stops
            // the submission.
            if written.is_ok() {
                let _ = the_arrangement_is_made_at_submission(
                    &mut DownloadArrangements::over(connection),
                    on,
                    chose.clone(),
                );
            }
        });
        written
    }

    fn remove_it(&mut self, job: &Identity) -> Result<(), String> {
        let mut removed = Err("the local record would not open".to_string());
        let (whole, kept) = (self.the_whole_record.clone(), self.the_kept_setup.clone());
        self.record.with_the_record(&mut |connection| {
            removed =
                TheSubmissionsWriter::of(connection, whole.clone(), kept.clone()).remove_it(job);
        });
        removed
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// **The Creator's seventh notes, piece Z7, and `Queue & History` C10 as
    /// changed 2026-10-19: how the act ended decides its row.** Every job in
    /// the line keeps it for the machine's own row. **A job not in the line
    /// keeps it, saying *Not submitted.***: a refusal before the provisional
    /// record or after it, a frame not known, files not placed, and a scene
    /// refused beside one in the line. *Not known*, one left behind it, and a
    /// lock take it away. *(Until 2026-10-19 every refusal took it away, and
    /// the live pass on Vast saw the row go with nothing said.)*
    #[test]
    fn z7_the_row_is_kept_in_the_line_or_saying_not_submitted() {
        use WhatBecameOfOneJob as Became;
        let server = Identity::issue(IdentityKind::LocalServer).expect("randomness").as_text();
        let ended = |came_to: WhatTheActCameTo| {
            let mut in_flight = an_act_is_begun_on(&server, "Kitchen".to_string());
            let which = in_flight.which();
            assert!(the_act_in_flight(which).is_some(), "the row stands from the beginning");
            in_flight.it_ended(&came_to);
            drop(in_flight);
            let left = the_act_in_flight(which).map(|act| (act.ended_in_the_line, act.not_submitted.is_some()));
            a_not_submitted_row_is_cleared(which);
            left
        };
        assert_eq!(ended(WhatTheActCameTo::TheseJobs(vec![Became::ItIsInTheLine])), Some((true, false)));
        for not_submitted in [
            WhatTheActCameTo::TheFirstFrameIsNotKnown { a_scene_can_be_ticked: false },
            WhatTheActCameTo::TheseJobs(vec![Became::TheBuildLacksTheEngine {
                engine: "CYCLES".to_string(),
                build: "4.5".to_string(),
            }]),
            WhatTheActCameTo::TheseJobs(vec![Became::TheRecordWasRefused]),
            WhatTheActCameTo::TheseJobs(vec![Became::TheServerDoesNotTakeJobs]),
            WhatTheActCameTo::TheseJobs(vec![Became::ItStoppedBeforeTheJobRecord { the_step: "upload" }]),
            WhatTheActCameTo::TheseJobs(vec![Became::ItIsInTheLine, Became::TheRecordWasRefused]),
        ] {
            assert_eq!(ended(not_submitted.clone()), Some((false, true)), "{not_submitted:?}");
        }
        for gone in [
            WhatTheActCameTo::TheseJobs(vec![Became::NotKnown]),
            WhatTheActCameTo::TheseJobs(vec![Became::ItIsInTheLine, Became::NotKnown]),
            WhatTheActCameTo::TheseJobs(vec![Became::ItIsInTheLine, Became::NotAttemptedBehindAnUnaccountedOne]),
        ] {
            assert_eq!(ended(gone.clone()), None, "{gone:?}");
        }
    }

    /// **`Queue & History` C10 as changed 2026-10-19: a not-submitted row stays
    /// whatever the readings hold, and goes when it is cleared or when that
    /// file is sent to that server again** — and never for another file, or
    /// that file sent to another server. Its cross takes no row that is
    /// still under way or waiting for the machine's own.
    #[test]
    fn a_not_submitted_row_stays_until_cleared_or_sent_again() {
        let server = Identity::issue(IdentityKind::LocalServer).expect("randomness").as_text();
        let other = Identity::issue(IdentityKind::LocalServer).expect("randomness").as_text();
        let at = |seconds: u64| SystemTime::UNIX_EPOCH + Duration::from_secs(seconds);
        let reading: Mutex<(SystemTime, Vec<Identity>)> = Mutex::new((at(1), Vec::new()));
        let drawn = |which: u64| {
            let the_reading_of = |held: &str| (held == server).then(|| reading.lock().unwrap().clone());
            the_acts_in_flight_to_draw(&the_reading_of)
                .into_iter()
                .any(|act| act.which == which)
        };
        let failed = || WhatTheActCameTo::TheseJobs(vec![WhatBecameOfOneJob::ItStoppedBeforeTheJobRecord { the_step: "upload" }]);

        let mut in_flight = an_act_is_begun_on_the_file(&server, "/root/kitchen.blend", "Kitchen".to_string());
        let which = in_flight.which();
        assert!(!a_not_submitted_row_is_cleared(which), "an act under way is not cleared by the cross");
        in_flight.it_ended(&failed());
        drop(in_flight);
        for moment in 2..5 {
            *reading.lock().unwrap() = (at(moment), Vec::new());
            assert!(drawn(which), "a later reading took the not-submitted row away");
        }
        assert!(a_not_submitted_row_is_cleared(which));
        assert!(!drawn(which) && the_act_in_flight(which).is_none(), "cleared, and still drawn");
        assert!(!a_not_submitted_row_is_cleared(which), "cleared twice");

        // **Sent again**: that file to that server takes it away; another file,
        // or that file to another server, does not.
        let mut in_flight = an_act_is_begun_on_the_file(&server, "/root/kitchen.blend", "Kitchen".to_string());
        let which = in_flight.which();
        in_flight.it_ended(&failed());
        drop(in_flight);
        let elsewhere = an_act_is_begun_on_the_file(&other, "/root/kitchen.blend", "Kitchen".to_string());
        let another = an_act_is_begun_on_the_file(&server, "/root/hallway.blend", "Hallway".to_string());
        assert!(the_act_in_flight(which).is_some(), "another server or another file took it away");
        let again = an_act_is_begun_on_the_file(&server, "/root/kitchen.blend", "Kitchen".to_string());
        assert!(the_act_in_flight(which).is_none(), "sending that file again left the old row standing");
        assert!(the_act_in_flight(again.which()).is_some(), "the new act's own row is drawn");

        // **An act whose row waits for the machine's own is not the cross's.**
        let mut kept = an_act_is_begun_on(&server, "Garden".to_string());
        let kept_is = kept.which();
        kept.it_ended(&WhatTheActCameTo::TheseJobs(vec![WhatBecameOfOneJob::ItIsInTheLine]));
        drop(kept);
        assert!(!a_not_submitted_row_is_cleared(kept_is));
        drop((elsewhere, another, again));
        if let Ok(mut held) = the_acts_in_flight().lock() {
            held.remove(&kept_is);
        }
    }

    /// **The Creator's seventh notes, piece Z7: the row gives way to the
    /// machine's own, and never leaves a gap.** While its server's reading
    /// holds none of its jobs it is drawn; the reading that holds one draws
    /// the machine's row instead, and the ended act leaves. **A job that
    /// ended before any reading saw it** leaves at its server's next reading.
    #[test]
    fn z7_the_row_gives_way_to_the_machines_own_or_to_the_next_reading() {
        let server = Identity::issue(IdentityKind::LocalServer).expect("randomness").as_text();
        let job = Identity::issue(IdentityKind::Job).expect("randomness");
        let at = |seconds: u64| SystemTime::UNIX_EPOCH + Duration::from_secs(seconds);
        let reading: Mutex<Option<(SystemTime, Vec<Identity>)>> = Mutex::new(Some((at(1), Vec::new())));
        let mine = |which: u64| {
            let the_reading_of = |held: &str| (held == server).then(|| reading.lock().unwrap().clone()).flatten();
            the_acts_in_flight_to_draw(&the_reading_of)
                .into_iter()
                .filter(|act| act.server == server)
                .count()
                + usize::from(the_act_in_flight(which).is_some()) * 10
        };

        // **Replaced by the machine's row.**
        let mut in_flight = an_act_is_begun_on(&server, "Kitchen".to_string());
        let which = in_flight.which();
        assert_eq!(mine(which), 11, "drawn, and held");
        in_flight.its_jobs_are(&[job.clone()]);
        *reading.lock().unwrap() = Some((at(2), vec![job.clone()]));
        assert_eq!(mine(which), 10, "the reading holds it while the act runs: not drawn twice");
        *reading.lock().unwrap() = Some((at(3), Vec::new()));
        assert_eq!(mine(which), 11);
        in_flight.it_ended(&WhatTheActCameTo::TheseJobs(vec![WhatBecameOfOneJob::ItIsInTheLine]));
        drop(in_flight);
        assert_eq!(mine(which), 11, "ended in the line, and drawn until the machine's row comes");
        assert_eq!(mine(which), 11, "the same reading again keeps it");
        *reading.lock().unwrap() = Some((at(4), vec![job.clone()]));
        assert_eq!(mine(which), 0, "the machine's row has taken its place");

        // **A job that ended before any reading saw it.**
        *reading.lock().unwrap() = None;
        let mut in_flight = an_act_is_begun_on(&server, "Hallway".to_string());
        let which = in_flight.which();
        in_flight.its_jobs_are(&[Identity::issue(IdentityKind::Job).expect("randomness")]);
        in_flight.it_ended(&WhatTheActCameTo::TheseJobs(vec![WhatBecameOfOneJob::ItIsInTheLine]));
        drop(in_flight);
        assert_eq!(mine(which), 11, "no reading yet");
        *reading.lock().unwrap() = Some((at(5), Vec::new()));
        assert_eq!(mine(which), 11, "the first reading drawn after its end");
        *reading.lock().unwrap() = Some((at(6), Vec::new()));
        assert_eq!(mine(which), 0, "the next holds none of it: gone");
    }

    /// **`Rendering` S4a across acts: one act per server at a time**, and acts
    /// on two servers are independent, the promise being per line.
    #[test]
    fn acts_on_one_server_share_one_line_and_two_servers_have_two() {
        let one = Identity::issue(IdentityKind::LocalServer).expect("randomness");
        let other = Identity::issue(IdentityKind::LocalServer).expect("randomness");
        assert!(Arc::ptr_eq(&the_line_for(&one), &the_line_for(&one)));
        assert!(!Arc::ptr_eq(&the_line_for(&one), &the_line_for(&other)));
        let held = the_line_for(&one);
        let _first = held.lock().expect("the first act holds the line");
        assert!(
            the_line_for(&one).try_lock().is_err(),
            "a second act went ahead of the first"
        );
        assert!(the_line_for(&other).try_lock().is_ok());
    }

    /// **`Rendering` S13 as amended 2026-10-02 (decision C): the second job
    /// is the whole range, its first frame included** — a run stays the run,
    /// a stepped run keeps its step, and a list stays the list. *Until then it
    /// was the rest of the range, one frame on.* Driven through the act itself
    /// in `tests/a_job_is_submitted_through_the_servers_own_point.rs`; this is
    /// the split's own half, for every form.
    #[test]
    fn the_second_job_is_the_whole_range() {
        for asked in [
            FrameForm::Triple { first: 1001, last: 1240, step: 1 },
            FrameForm::Triple { first: 1, last: 120, step: 2 },
            FrameForm::List(vec![1, 3, 7]),
        ] {
            let frames = asked.explicit_list();
            let Some(smoozy_rendering::submission::TheFirstFrameSplit::TwoJobs {
                the_first_frame,
                the_whole_range_held,
            }) = smoozy_rendering::submission::the_first_frame_split(&frames)
            else {
                panic!("two jobs for {asked:?}");
            };
            assert_eq!(the_first_frame, vec![frames[0]], "{asked:?}");
            assert_eq!(the_whole_range_held, frames, "{asked:?}");
        }
        assert!(matches!(
            smoozy_rendering::submission::the_first_frame_split(&[1]),
            Some(smoozy_rendering::submission::TheFirstFrameSplit::OneJobWithNoHold { .. })
        ));
    }

    /// **The submission's two transfers ride `Rendering` §7's class**, named by
    /// its declaration in the run's one order — the class ranked below manual
    /// download and above dependency-upload.
    #[test]
    fn the_submissions_class_is_the_one_rendering_declares() {
        let declared = crate::downloading::the_declared_order();
        let rank_of = |name: &str| {
            declared
                .iter()
                .find(|class| class.name == name)
                .map(|class| class.rank)
                .expect("declared")
        };
        let ours = rank_of(the_submission_class());
        let manual = rank_of(smoozy_file_browser::pool::THE_DECLARED_ORDER[1].name);
        let dependency = rank_of(smoozy_file_browser::pool::THE_DECLARED_ORDER[2].name);
        assert!(manual < ours && ours < dependency);
    }

    /// **A future whose steps are all done is driven to its end at once.**
    #[test]
    fn a_ready_walk_is_driven_to_its_end_at_once() {
        assert_eq!(at_once(std::future::ready(7)), 7);
        assert_eq!(waited_for(|| async { 8 }), 8);
    }

    /// A local record in memory, at the version this build knows.
    struct ARecordInHand(Mutex<rusqlite::Connection>);

    impl ARecordInHand {
        fn made() -> ARecordInHand {
            let connection = rusqlite::Connection::open_in_memory().expect("a record");
            for migration in crate::record::migrations::ALL {
                if let crate::record::migrations::WhatItRuns::TheseStatements(statements) =
                    migration.what_it_runs
                {
                    for statement in statements {
                        connection
                            .execute(statement, [])
                            .expect("the migration runs");
                    }
                }
            }
            ARecordInHand(Mutex::new(connection))
        }
    }

    impl TheRecordAnUploadWrites for ARecordInHand {
        fn with_the_record(&self, act: &mut dyn FnMut(&rusqlite::Connection)) -> bool {
            let held = self
                .0
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            act(&held);
            true
        }
    }

    pub(crate) fn a_page_that_hands_over(
        the_choice_is_on: bool,
        where_they_go: Option<Vec<u8>>,
    ) -> WhatThePageHandsOver {
        let issue = |kind| Identity::issue(kind).expect("randomness");
        let layout = TheLayout::of(
            smoozy_job_files::layout::MachineRoot::chosen("/srv/smoozy").expect("a root"),
            smoozy_job_files::layout::WhereThePackageAreaSits::UnderTheRoot,
        );
        WhatThePageHandsOver {
            server: TheServerAsItStood {
                local_server: issue(IdentityKind::LocalServer),
                label: None,
                hue: None,
            },
            installation: None,
            package_area: layout.area(smoozy_job_files::layout::Area::Package),
            layout,
            the_blend: "/srv/smoozy/assets/kitchen.blend".to_string(),
            label: None,
            reporting: smoozy_rendering::submission::the_reporting_field(
                None,
                smoozy_render_settings::reporting::ReportingChoice::Everything,
            ),
            the_version_origin: AnswerOrigin::ResolvedForThem,
            dependency_check: None,
            what_the_desktop_holds_about_the_build:
                WhatTheDesktopHoldsAboutTheBuild::NothingButTheEntry,
            the_first_frame_check: false,
            the_download_as_it_renders_choice: the_choice_is_on,
            where_the_downloads_go: where_they_go,
            the_formats_chosen: None,
            scene_jobs: Vec::new(),
            the_reading_in_force: None,
            the_card_make: TheCardMakeAtSubmission::TheMakes(Vec::new()),
            run: issue(IdentityKind::Run),
            carries_on_from: None,
            the_temporary_area: None,
            the_servers_fallback_folder: None,
            the_driver_at_submission: Default::default(),
            the_typed_frames: None,
        }
    }

    /// **`Queue & History` N15's second mark goes on the record the act
    /// writes** — the job a carry-on's page was opened from, with its label
    /// as it stood, and nothing on any other act (the 38's piece 2).
    #[test]
    fn a_carry_ons_record_carries_the_mark_naming_the_earlier_job() {
        use smoozy_render_settings::payload::{compose, Composition};
        let payload = compose(Composition {
            version_pick: &BlenderIdentity {
                path: "/srv/smoozy/blender/blender-5.2.2/blender".to_string(),
                version: the_version_in("5.2.2"),
            },
            scene: None,
            answers: &smoozy_render_settings::store::SceneEntries::default(),
            standing: None,
            stale: false,
        });
        let job = Identity::issue(IdentityKind::Job).unwrap();
        let mut given = a_page_that_hands_over(true, None);
        let plain = the_record_at_submission(&given, None, &job, &payload, &payload.the_manifests_blender());
        assert_eq!(plain.carries_on_from, None);
        let mark = smoozy_queue_history::record::CarriesOnFrom {
            earlier_job: Identity::issue(IdentityKind::Job).unwrap(),
            the_label_as_it_stood: "Kitchen, take one".to_string(),
        };
        given.carries_on_from = Some(mark.clone());
        let carried = the_record_at_submission(&given, None, &job, &payload, &payload.the_manifests_blender());
        assert_eq!(carried.carries_on_from, Some(mark));
    }

    /// **`Job files and outputs` P21's second case, at Submit**: the manifest
    /// says the job's folder is chosen at render start and carries `Job files and outputs` N3's name
    /// and the server row's chosen fallback folder as it stands, trimmed
    /// (`Job files and outputs` M1, `Job files and outputs` P23; `Rendering` S5); the local record
    /// says the same, so the entry has no folder until the machine's choice
    /// arrives (`Queue & History` Y8). The Creator's second notes, piece 4.
    #[test]
    fn a_submission_has_its_folder_chosen_at_render_start_with_the_rows_fallback_folder() {
        use smoozy_render_settings::payload::{compose, Composition};
        let payload = compose(Composition {
            version_pick: &BlenderIdentity {
                path: "/srv/smoozy/blender/blender-5.2.2/blender".to_string(),
                version: the_version_in("5.2.2"),
            },
            scene: None,
            answers: &smoozy_render_settings::store::SceneEntries::default(),
            standing: None,
            stale: false,
        });
        let job = Identity::issue(IdentityKind::Job).unwrap();
        let mut given = a_page_that_hands_over(true, None);
        given.label = Some("Kitchen".to_string());
        given.the_servers_fallback_folder = Some("  /mnt/renders ".to_string());
        let (_, manifest) = the_manifest_of(
            &given,
            None,
            &job,
            &payload,
            smoozy_server_protocol::transfers::SourceReading {
                size: 1,
                inode: "1".into(),
                modification_time: 1,
                change_time: 1,
            },
        )
        .expect("a manifest");
        assert_eq!(
            manifest.job_output_folder,
            smoozy_job_files::manifest::WhereTheJobOutputFolderIs::ChosenAtRenderStart {
                the_folders_name: smoozy_job_files::names::job_output_folder_name(&job, Some("Kitchen")),
                the_servers_fallback_folder_at_submission: Some("/mnt/renders".to_string()),
            }
        );
        given.the_servers_fallback_folder = Some("   ".to_string());
        let (_, none_set) = the_manifest_of(
            &given,
            None,
            &job,
            &payload,
            smoozy_server_protocol::transfers::SourceReading { size: 1, inode: "1".into(), modification_time: 1, change_time: 1 },
        )
        .expect("a manifest");
        assert!(matches!(
            none_set.job_output_folder,
            smoozy_job_files::manifest::WhereTheJobOutputFolderIs::ChosenAtRenderStart {
                the_servers_fallback_folder_at_submission: None,
                ..
            }
        ));
        let record = the_record_at_submission(&given, None, &job, &payload, &payload.the_manifests_blender());
        assert_eq!(
            record.the_job_output_folder,
            smoozy_queue_history::record::TheJobOutputFolderTheRecordHolds::ChosenAtRenderStart
        );
    }

    /// A payload to submit with, and the whole record composed from it.
    fn a_writer_for<'a>(
        record: &'a ARecordInHand,
        given: &WhatThePageHandsOver,
        job: &Identity,
    ) -> (TheWriterHere<'a>, AProvisionalRecord) {
        use smoozy_render_settings::payload::{compose, Composition};
        let payload = compose(Composition {
            version_pick: &BlenderIdentity {
                path: "/srv/smoozy/blender/blender-5.2.2/blender".to_string(),
                version: the_version_in("5.2.2"),
            },
            scene: None,
            answers: &smoozy_render_settings::store::SceneEntries::default(),
            standing: None,
            stale: false,
        });
        let whole =
            the_record_at_submission(given, None, job, &payload, &payload.the_manifests_blender());
        let provisional = AProvisionalRecord {
            job: job.clone(),
            server: given.server.local_server.clone(),
            group: None,
            label: String::new(),
        };
        let writer = TheWriterHere {
            record,
            the_whole_record: whole,
            the_kept_setup: payload,
            first_frame_purpose: None,
            typed_frames: None,
            the_reporting_choice: given.reporting.0.to_string(),
            the_choice_is_on: given.the_download_as_it_renders_choice,
            the_arrangement: what_the_submission_chose(given, job),
            the_resolved_set: None,
        };
        (writer, provisional)
    }

    fn the_arrangement_held_for(
        record: &ARecordInHand,
        job: &Identity,
    ) -> Option<smoozy_download::arrangement::AnArrangement> {
        use smoozy_download::arrangement::KeepsTheArrangement;
        let held = record.0.lock().expect("the record");
        DownloadArrangements::over(&held)
            .the_arrangement_for(job)
            .expect("it reads")
    }

    fn the_record_stands(record: &ARecordInHand, job: &Identity) -> bool {
        let held = record.0.lock().expect("the record");
        crate::record::job_records::JobRecords::over(&held)
            .read(job)
            .expect("it reads")
            .is_some()
    }

    /// **A machine on the far side of `Queue & History`'s seam**, which
    /// answers the send and the hold ask as it is told, **and looks at the
    /// local record at the moment the send arrives.**
    struct AMachineThatAnswers<'a> {
        gave: WhatTheSubmissionGave,
        holds: Result<bool, String>,
        record: &'a ARecordInHand,
        what_stood_at_the_send: Mutex<Option<(bool, bool)>>,
    }

    impl SendsTheSubmission for AMachineThatAnswers<'_> {
        fn send(&self, sent: &AProvisionalRecord) -> WhatTheSubmissionGave {
            *self.what_stood_at_the_send.lock().expect("held") = Some((
                the_record_stands(self.record, &sent.job),
                the_arrangement_held_for(self.record, &sent.job).is_some(),
            ));
            self.gave.clone()
        }

        fn does_the_machine_hold(&self, _job: &Identity) -> Result<bool, String> {
            self.holds.clone()
        }
    }

    #[test]
    fn seventeenth_piece_2_again_the_desktop_writer_keeps_purpose_before_the_send() {
        let record = ARecordInHand::made();
        let given = a_page_that_hands_over(false, None);
        let job = Identity::issue(IdentityKind::Job).unwrap();
        let first = Identity::issue(IdentityKind::Job).unwrap();
        let purpose = crate::record::job_records::FirstFramePurpose {check_job:first.as_text(), range_job:job.as_text(),
            first_frame:1001, submitted_range:FrameForm::Triple {first:1001,last:1011,step:2}};
        let (mut writer, provisional) = a_writer_for(&record, &given, &job);
        writer.first_frame_purpose = Some(purpose.clone());
        writer.write_it(&provisional).unwrap();
        record.with_the_record(&mut |connection| {
            assert_eq!(crate::record::job_records::JobRecords::over(connection).first_frame_purpose(&job).unwrap(), Some(purpose.clone()));
        });
    }

    /// **`Queue & History` P5b: the frame text the user typed goes down with
    /// the job's record before the send, and only on the job whose range it
    /// is** — the eighteenth notes, piece 7. The held whole range of a
    /// first-frame pair keeps it; the check's one frame, Smoozy's own split,
    /// does not; a scene-job with no range keeps none.
    #[test]
    fn the_typed_frames_go_down_with_the_record_of_the_job_whose_range_they_are() {
        use smoozy_render_settings::payload::{compose, Composition};
        let a_payload = |range: Option<FrameForm>| {
            compose(Composition {
                version_pick: &BlenderIdentity {
                    path: "/srv/smoozy/blender/blender-5.2.2/blender".to_string(),
                    version: the_version_in("5.2.2"),
                },
                scene: None,
                answers: &smoozy_render_settings::store::SceneEntries {
                    frame_range: range,
                    ..Default::default()
                },
                standing: None,
                stale: false,
            })
        };
        let typed = FrameForm::List(vec![1, 2, 3, 4]);
        let mut given = a_page_that_hands_over(false, None);
        given.the_typed_frames = Some("1,2-4".to_string());
        given.scene_jobs = vec![a_payload(Some(typed.clone())), a_payload(None)];
        // The held whole range, exactly as asked, keeps the text.
        assert_eq!(
            the_typed_frames_of_this_job(&given, &a_payload(Some(typed.clone())), 0).as_deref(),
            Some("1,2-4")
        );
        // The check's one frame does not.
        assert_eq!(the_typed_frames_of_this_job(&given, &a_payload(Some(FrameForm::List(vec![1]))), 0), None);
        // A scene-job with no range under it keeps none, and nor does a job of
        // a scene-job the act does not hold.
        assert_eq!(the_typed_frames_of_this_job(&given, &a_payload(None), 1), None);
        assert_eq!(the_typed_frames_of_this_job(&given, &a_payload(Some(typed.clone())), 7), None);
        given.the_typed_frames = None;
        assert_eq!(the_typed_frames_of_this_job(&given, &a_payload(Some(typed)), 0), None);

        // **Written with the record, before the send.**
        let record = ARecordInHand::made();
        let given = a_page_that_hands_over(false, None);
        let job = Identity::issue(IdentityKind::Job).unwrap();
        let (mut writer, provisional) = a_writer_for(&record, &given, &job);
        writer.typed_frames = Some("1,2-4".to_string());
        writer.write_it(&provisional).unwrap();
        record.with_the_record(&mut |connection| {
            let records = crate::record::job_records::JobRecords::over(connection);
            assert_eq!(records.the_typed_frames_of(&job).unwrap().as_deref(), Some("1,2-4"));
        });
    }

    /// **`Download` R1's arrangement is down with the provisional record,
    /// before anything is sent** — so a quit while the page says
    /// *Submitting.* leaves both, and the frames of a job the machine took come
    /// home at the next connection (`Download` R12). *(The seventh pass on
    /// Vast, fault 1: the arrangement was written after the machine's answer,
    /// and a quit two seconds after the machine held the job lost it.)*
    #[test]
    fn the_arrangement_is_down_with_the_record_before_the_send() {
        use smoozy_download::arrangement::DesiredState;
        let record = ARecordInHand::made();
        let given = a_page_that_hands_over(true, Some(b"/Users/someone/Renders".to_vec()));
        let job = Identity::issue(IdentityKind::Job).expect("randomness");
        let (mut writer, provisional) = a_writer_for(&record, &given, &job);
        let machine = AMachineThatAnswers {
            gave: WhatTheSubmissionGave::ItWasAcknowledged,
            holds: Ok(true),
            record: &record,
            what_stood_at_the_send: Mutex::new(None),
        };
        assert_eq!(
            submit(&mut writer, &machine, &provisional),
            WhatTheSubmissionLeft::ItWasAcknowledged
        );
        assert_eq!(
            *machine.what_stood_at_the_send.lock().expect("held"),
            Some((true, true)),
            "the record and the arrangement were not both down when the send left"
        );
        let made = the_arrangement_held_for(&record, &job).expect("it stands after the answer");
        assert_eq!(made.destination, b"/Users/someone/Renders".to_vec());
        assert!(made.narrowing.is_no_filter());
        assert_eq!(made.desired_state, DesiredState::On);
        assert_eq!(made.server_row, given.server.local_server);
    }

    /// **A lost acknowledgement leaves the record and the arrangement
    /// standing** wherever the machine holds the job or could not be asked —
    /// `State, identity and recovery` V1 step 2 asks again at the next
    /// connection, and the arrangement resumes under `Download` R12.
    #[test]
    fn an_unaccounted_submission_keeps_its_arrangement() {
        for holds in [Ok(true), Err("the connection dropped".to_string())] {
            let record = ARecordInHand::made();
            let given = a_page_that_hands_over(true, Some(b"/Users/someone/Renders".to_vec()));
            let job = Identity::issue(IdentityKind::Job).expect("randomness");
            let (mut writer, provisional) = a_writer_for(&record, &given, &job);
            let machine = AMachineThatAnswers {
                gave: WhatTheSubmissionGave::TheAcknowledgementWasLost,
                holds: holds.clone(),
                record: &record,
                what_stood_at_the_send: Mutex::new(None),
            };
            submit(&mut writer, &machine, &provisional);
            resolve(&mut writer, &machine, &job);
            assert!(the_record_stands(&record, &job), "{holds:?}");
            assert!(
                the_arrangement_held_for(&record, &job).is_some(),
                "{holds:?}"
            );
        }
    }

    /// **Every branch that removes the provisional record takes the
    /// arrangement with it** (`Download` R3: *a cancelled job's provisional
    /// record removal takes the arrangement with it*). Two of
    /// [`one_scene_job`]'s three call the writer's removal directly — the unit
    /// stopped before the job record, and the machine refusing jobs by name —
    /// and the third is `resolve` hearing *does not hold*. **All three reach
    /// the one removal, so both routes are driven here.** Left behind, the
    /// arrangement would be resumed on every later connection and walk a
    /// folder for a job the machine never had.
    #[test]
    fn every_removal_of_the_record_takes_the_arrangement() {
        // The machine said it does not hold the job.
        let record = ARecordInHand::made();
        let given = a_page_that_hands_over(true, Some(b"/Users/someone/Renders".to_vec()));
        let job = Identity::issue(IdentityKind::Job).expect("randomness");
        let (mut writer, provisional) = a_writer_for(&record, &given, &job);
        let machine = AMachineThatAnswers {
            gave: WhatTheSubmissionGave::TheAcknowledgementWasLost,
            holds: Ok(false),
            record: &record,
            what_stood_at_the_send: Mutex::new(None),
        };
        submit(&mut writer, &machine, &provisional);
        assert_eq!(
            resolve(&mut writer, &machine, &job),
            HowItResolved::TheMachineDoesNotHoldIt
        );
        assert!(!the_record_stands(&record, &job));
        assert!(
            the_arrangement_held_for(&record, &job).is_none(),
            "an arrangement outlived the record `resolve` removed"
        );

        // The two direct removals.
        let record = ARecordInHand::made();
        let job = Identity::issue(IdentityKind::Job).expect("randomness");
        let (mut writer, provisional) = a_writer_for(&record, &given, &job);
        writer.write_it(&provisional).expect("written");
        assert!(the_arrangement_held_for(&record, &job).is_some());
        writer.remove_it(&job).expect("removed");
        assert!(!the_record_stands(&record, &job));
        assert!(
            the_arrangement_held_for(&record, &job).is_none(),
            "an arrangement outlived the record the unit removed"
        );
    }

    /// **`State, identity and recovery` V1 step 2 at a later connection**:
    /// a provisional record the machine says it never took is removed leaving
    /// no trace, and that includes its arrangement.
    #[test]
    fn a_later_connections_removal_takes_the_arrangement() {
        let record = ARecordInHand::made();
        let given = a_page_that_hands_over(true, Some(b"/Users/someone/Renders".to_vec()));
        let job = Identity::issue(IdentityKind::Job).expect("randomness");
        let (mut writer, provisional) = a_writer_for(&record, &given, &job);
        writer.write_it(&provisional).expect("written");
        {
            let held = record.0.lock().expect("the record");
            crate::record::job_records::JobRecords::over(&held)
                .the_resolution(
                    &job,
                    smoozy_queue_history::record::WhatTheMachineSaidAboutTheJob::ItsOperationRecordShowsTheSubmissionWasRefusedOrNeverTaken,
                )
                .expect("resolved");
        }
        assert!(!the_record_stands(&record, &job));
        assert!(the_arrangement_held_for(&record, &job).is_none());
    }

    /// **A refused record sends nothing and makes no arrangement**; there is
    /// no job for one to ride.
    #[test]
    fn a_refused_record_makes_no_arrangement() {
        let record = ARecordInHand::made();
        let given = a_page_that_hands_over(true, Some(b"/Users/someone/Renders".to_vec()));
        let job = Identity::issue(IdentityKind::Job).expect("randomness");
        let (mut writer, mut provisional) = a_writer_for(&record, &given, &job);
        // A record the writer was not given is refused by the store.
        provisional.label = "a stranger".to_string();
        assert!(writer.write_it(&provisional).is_err());
        assert!(the_arrangement_held_for(&record, &job).is_none());
    }

    /// **The choice off is not an arrangement switched off**: nothing is
    /// written and the job has never had one. **And no folder named is no
    /// arrangement**, because `Download` R1 fixes the destination at the moment
    /// it is made and nothing can be asked of a user who is not there. **The
    /// record itself is written either way.**
    #[test]
    fn the_choice_off_and_no_folder_each_write_nothing() {
        for given in [
            a_page_that_hands_over(false, Some(b"/Users/someone/Renders".to_vec())),
            a_page_that_hands_over(true, None),
        ] {
            let record = ARecordInHand::made();
            let job = Identity::issue(IdentityKind::Job).expect("randomness");
            let (mut writer, provisional) = a_writer_for(&record, &given, &job);
            writer.write_it(&provisional).expect("written");
            assert!(the_record_stands(&record, &job));
            let held = record.0.lock().expect("the record");
            assert_eq!(
                held.query_row("SELECT count(*) FROM download_arrangement", [], |row| row
                    .get::<_, i64>(
                    0
                ))
                .expect("counted"),
                0,
                "something was written for a job that has no arrangement"
            );
        }
    }

    // ── Step 10c's dependency wave piece 7 ─────────────────────────────────

    /// **`Dependency Upload & Remap` K9: the job's resolved set is written with
    /// the provisional record, and a set the record refuses takes the record
    /// with it** — nothing is sent for a job whose set is not kept.
    #[test]
    fn the_resolved_set_is_written_with_the_provisional_record() {
        let record = ARecordInHand::made();
        let mut given = a_page_that_hands_over(false, None);
        let installation = Identity::issue(IdentityKind::Installation).expect("randomness");
        given.installation = Some(installation.clone());
        let machine = MachineKey::of(given.server.local_server.clone(), installation);
        let job = Identity::issue(IdentityKind::Job).expect("randomness");
        let mirror = MachineLocation::composed(b"/srv/smoozy/assets/files/Users/a/tex/wood.png".to_vec());
        let set = ResolvedSet {
            locations: vec![mirror.clone()],
            remap_material: RemapMaterial::location_in(&given.layout, &job),
        };
        let blend = MachineLocation::composed(given.the_blend.as_bytes().to_vec());
        let (mut writer, provisional) = a_writer_for(&record, &given, &job);
        writer.the_resolved_set = Some((machine.clone(), blend.clone(), set.clone()));
        writer.write_it(&provisional).expect("written");
        assert!(the_record_stands(&record, &job));
        let kept = {
            let held = record.0.lock().expect("the record");
            TheDependencyRecord::over(&held)
                .what_it_holds_for(&machine)
                .expect("it reads")
        };
        let live = smoozy_dependencies::record::TheRecord::live_or_waiting_jobs_on(&kept, &machine);
        assert_eq!(live.len(), 1, "{live:?}");
        assert_eq!(live[0].resolved_set, set);
        assert!(live[0].resolved_set.names(&mirror));

        // **A set the store refuses**: one already written under this job.
        let refused = Identity::issue(IdentityKind::Job).expect("randomness");
        {
            let held = record.0.lock().expect("the record");
            TheDependencyRecord::over(&held)
                .a_jobs_resolved_set_is_written(&refused, &machine, &blend, &set)
                .expect("the first set");
        }
        let (mut writer, provisional) = a_writer_for(&record, &given, &refused);
        writer.the_resolved_set = Some((machine, blend, set));
        assert!(writer.write_it(&provisional).is_err());
        assert!(
            !the_record_stands(&record, &refused),
            "a job whose set was not kept stands to be sent"
        );
    }

    /// **`Dependency Upload & Remap` M10 at submission, from the record alone**
    /// — no check of the file in this run, as after Smoozy is closed and
    /// opened. Each answer gives a pair for every kind and setting the check
    /// read it as (`Dependency Upload & Remap` K4's last limb); an answer no
    /// reading wrote them into gives none and is named; the bake is copied,
    /// not pointed at (`Dependency Upload & Remap` M22c).
    #[test]
    fn the_pairs_are_composed_from_the_record_alone() {
        use smoozy_dependencies::bytes::StoredLocation;
        use smoozy_dependencies::check::Kind;
        use smoozy_dependencies::record::HoldingFile;
        use smoozy_dependencies::record::{
            HowResolved, LocatedAnswer, LocatedAnswerKey, ReadAs, Resolution, ResolvedTo,
        };
        let record = ARecordInHand::made();
        let mut given = a_page_that_hands_over(false, None);
        let installation = Identity::issue(IdentityKind::Installation).expect("randomness");
        given.installation = Some(installation.clone());
        let machine = MachineKey::of(given.server.local_server.clone(), installation);
        // **No upload through Files names it**, so a check keys it by its
        // place on the server (`Dependency Upload & Remap` K5).
        let holding = HoldingFile::OnTheMachineWithNoKnownLocalOrigin {
            machine: machine.clone(),
            location: MachineLocation::composed(given.the_blend.as_bytes().to_vec()),
        };
        let read_as = |kinds: &[(Kind, &str)]| {
            kinds
                .iter()
                .map(|(kind, setting)| ReadAs {
                    kind: *kind,
                    setting: setting.to_string(),
                })
                .collect::<Vec<_>>()
        };
        let answer = |recorded: &[u8], to: Option<&[u8]>, set_aside: bool, kinds: &[(Kind, &str)]| {
            LocatedAnswer {
                key: LocatedAnswerKey {
                    holding_file: holding.clone(),
                    reference: RecordedLocation::as_the_reading_carried_it(recorded),
                },
                resolution: to.map(|to| Resolution {
                    to: ResolvedTo::AWholeLocation(StoredLocation::as_the_system_returned_it(to)),
                    how: HowResolved::TheRecordedLocationItself,
                }),
                set_aside,
                rejected_guess: None,
                read_as: read_as(kinds),
            }
        };
        let picture = [(Kind::Image, "filepath")];
        {
            let held = record.0.lock().expect("the record");
            TheDependencyRecord::over(&held)
                .the_answers_are_written(&[
                    answer(b"//tex/wood.png", Some(b"/Users/a/proj/tex/wood.png"), false, &picture),
                    answer(b"//tex/gone.png", None, false, &picture),
                    answer(b"//tex/aside.png", Some(b"/Users/a/proj/tex/aside.png"), true, &picture),
                    answer(
                        b"//cache/",
                        Some(b"/Users/a/proj/cache"),
                        false,
                        &[(Kind::PhysicsPointCache, "point_cache.filepath")],
                    ),
                    // **The same bytes read as a clip and as a picture**: one
                    // answer, a pair for each.
                    answer(
                        b"//clip.mov",
                        Some(b"/Users/a/proj/clip.mov"),
                        false,
                        &[(Kind::MovieClip, "filepath"), (Kind::Image, "filepath")],
                    ),
                    // **Written before any reading carried its kind.**
                    answer(b"//tex/old.png", Some(b"/Users/a/proj/tex/old.png"), false, &[]),
                ])
                .expect("written");
        }

        let composed = the_remap_is_composed(&given, &record).expect("the record answers");
        let mirror = |local: &[u8]| {
            smoozy_dependencies::landing::mirror_of(
                &given.layout,
                &StoredLocation::as_the_system_returned_it(local),
            )
            .expect("it fits")
        };
        let pairs: Vec<(Kind, &str, &[u8], MachineLocation)> = composed
            .pairs
            .iter()
            .map(|pair| {
                (
                    pair.kind,
                    pair.setting.as_str(),
                    pair.expected.as_bytes(),
                    pair.new_location.clone(),
                )
            })
            .collect();
        let (wood, clip) = (mirror(b"/Users/a/proj/tex/wood.png"), mirror(b"/Users/a/proj/clip.mov"));
        assert_eq!(
            pairs,
            vec![
                (Kind::MovieClip, "filepath", b"//clip.mov".as_slice(), clip.clone()),
                (Kind::Image, "filepath", b"//clip.mov".as_slice(), clip.clone()),
                (Kind::Image, "filepath", b"//tex/wood.png".as_slice(), wood.clone()),
            ],
            "{composed:#?}"
        );
        assert_eq!(composed.copies_owed.len(), 1, "{composed:#?}");
        assert_eq!(composed.copies_owed[0].recorded.as_bytes(), b"//cache/");
        assert_eq!(
            composed.without_a_kind,
            vec![RecordedLocation::as_the_reading_carried_it(b"//tex/old.png")]
        );
        assert_eq!(composed.without_a_pair.len(), 3, "{composed:#?}");
        let names = composed.the_machine_locations_it_names();
        assert!(names.contains(&wood) && names.contains(&clip), "{names:?}");
        assert_eq!(names.len(), 3, "{names:?}");

        // **The same answers on another machine are not this one's**: the
        // holding file is keyed to this machine (K5), so nothing is read.
        given.installation = Some(Identity::issue(IdentityKind::Installation).expect("randomness"));
        let elsewhere = the_remap_is_composed(&given, &record).expect("the record answers");
        assert!(elsewhere.pairs.is_empty(), "{elsewhere:#?}");
    }
}

// ════════════════════════════════════════════════════════════════════════════
// Two version searches on one server — step 10c's eleventh-pass fix piece 4c
// ════════════════════════════════════════════════════════════════════════════

/// **The Render Settings page's version search and the Set up tab's Blender
/// search, on one server at once**, both through
/// [`the_blenders_on_the_machine`] — the one call both make.
///
/// The stand-in lends the real point over OpenSSH's own `sftp-server`, so the
/// `blender-candidate` link is a real link in a real home, and each command is
/// answered **a round trip after it is asked**, reading the link as it stands
/// then — the moment another search's link can land first.
///
/// **macOS and Linux only**, like piece 4b's `two_readings_on_one_server`
/// beside it: the stand-in's connection is a Unix socket pair handed to
/// OpenSSH's `sftp-server`, and Windows has neither. What it proves — two
/// searches on one server taking turns — is the same code on every system.
#[cfg(all(test, unix))]
mod two_version_searches_on_one_server {
    use std::os::fd::OwnedFd;
    use std::path::PathBuf;
    use std::process::{Command, Stdio};
    use std::sync::Arc;
    use std::time::Duration;

    use smoozy_render_settings::acting::versions::TheVersionListAsk;
    use smoozy_server_protocol::transport::{CommandResult, CommandString, Ending};
    use smoozy_server_setup::blender::Version;

    use crate::servers::sessions::{
        lend_until_it_ends, ACommandRunning, AHeldConnection, NoSession, TheAdditionalConnection,
        ThisServersSessions,
    };

    const WHERE_SFTP_SERVER_LIVES: &[&str] = &[
        "/usr/libexec/sftp-server",
        "/usr/lib/openssh/sftp-server",
        "/usr/libexec/openssh/sftp-server",
        "/usr/lib/ssh/sftp-server",
    ];

    const A_ROUND_TRIP: Duration = Duration::from_millis(150);

    /// **A local server identifier as the record writes it**, the key both
    /// searches name the server by — this test's own, so no other test of this
    /// binary shares its turn.
    const THIS_SERVER: &str = "4c4c4c4c4c4c4c4c4c4c4c4c4c4c4c20";

    /// **One rented machine's home, with two builds on it** — each a file that
    /// answers `--version` with its own line.
    struct AMachine {
        sftp_server: PathBuf,
        home: PathBuf,
    }

    impl AMachine {
        fn standing(named: &str) -> Option<Arc<AMachine>> {
            let sftp_server = WHERE_SFTP_SERVER_LIVES
                .iter()
                .map(PathBuf::from)
                .find(|path| path.exists())?;
            let home = std::env::temp_dir().join(format!("smzy-4c-{named}-{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&home);
            std::fs::create_dir_all(&home).expect("the login's home");
            let home = std::fs::canonicalize(&home).expect("the home is there");
            for (folder, line) in THE_TWO_BUILDS {
                std::fs::create_dir_all(home.join(folder)).expect("the build's folder");
                std::fs::write(home.join(folder).join("blender"), line).expect("the build");
            }
            Some(Arc::new(AMachine { sftp_server, home }))
        }

        fn the_build_in(&self, folder: &str) -> String {
            self.home
                .join(folder)
                .join("blender")
                .to_str()
                .expect("a path in words")
                .to_string()
        }

        /// **What the machine would answer, now**: the whole-disk search finds
        /// both builds, a version ask runs whatever the link points at, and
        /// the clock is a clock.
        fn the_command_runs(&self, command: &str) -> CommandResult {
            let answered = |output: String| CommandResult {
                output: output.into_bytes(),
                complaint: Vec::new(),
                status: Some(0),
            };
            if command == smoozy_server_setup::readings::THE_SEARCH.as_str() {
                return answered(
                    THE_TWO_BUILDS
                        .iter()
                        .map(|(folder, _)| format!("{}\n", self.the_build_in(folder)))
                        .collect(),
                );
            }
            if command == "date +%s" {
                return answered("1759200000\n".to_string());
            }
            let asked = smoozy_server_setup::readings::ASK_THE_BUILD_ITS_VERSION.as_str();
            if command == asked {
                let link = smoozy_server_setup::readings::THE_VERSION_ASKS_CARRIAGE_SITE
                    .link_path_on_the_subsystem;
                return match std::fs::read_to_string(self.home.join(link)) {
                    Ok(line) => answered(line),
                    Err(_) => CommandResult {
                        output: Vec::new(),
                        complaint:
                            b"bash: ~/.smoozy-carriage/blender-candidate: No such file or directory"
                                .to_vec(),
                        status: Some(127),
                    },
                };
            }
            CommandResult {
                output: Vec::new(),
                complaint: b"no such command here".to_vec(),
                status: Some(127),
            }
        }
    }

    /// **The two builds**, a long-term-support one and an unstable one.
    const THE_TWO_BUILDS: [(&str, &str); 2] = [
        ("opt/blender-4.2/", "Blender 4.2.3 LTS\n"),
        ("opt/blender-4.5/", "Blender 4.5.1\n"),
    ];

    impl Drop for AMachine {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.home);
        }
    }

    struct AConnectionToIt(Arc<AMachine>);

    impl AHeldConnection for AConnectionToIt {
        type Channel = tokio::net::UnixStream;

        async fn open_a_session(&mut self) -> Result<tokio::net::UnixStream, NoSession> {
            let (ours, theirs) = std::os::unix::net::UnixStream::pair().expect("a socket pair");
            ours.set_nonblocking(true).expect("the end we drive");
            let reading: OwnedFd = theirs.try_clone().expect("its reading end").into();
            let writing: OwnedFd = theirs.into();
            let mut child = Command::new(&self.0.sftp_server)
                .current_dir(&self.0.home)
                .stdin(Stdio::from(reading))
                .stdout(Stdio::from(writing))
                .stderr(Stdio::null())
                .spawn()
                .expect("the machine's own file-transfer server starts");
            // **It ends when our end closes**, and is waited for so it leaves
            // nothing behind.
            std::thread::spawn(move || {
                let _ = child.wait();
            });
            Ok(tokio::net::UnixStream::from_std(ours).expect("the end we drive"))
        }

        async fn a_command(
            &mut self,
            command: CommandString,
            _frame: Option<Vec<u8>>,
        ) -> Result<CommandResult, NoSession> {
            tokio::time::sleep(A_ROUND_TRIP).await;
            Ok(self.0.the_command_runs(command.as_str()))
        }

        /// **Handed back running**, the way the real transport hands back a
        /// command whose request has gone out — so the point serves the other
        /// search's link while this command is on its way.
        async fn a_command_begun(
            &mut self,
            command: CommandString,
            _frame: Option<Vec<u8>>,
        ) -> Result<ACommandRunning, NoSession> {
            let machine = Arc::clone(&self.0);
            Ok(ACommandRunning::to_be_run(async move {
                tokio::time::sleep(A_ROUND_TRIP).await;
                Ok(machine.the_command_runs(command.as_str()))
            }))
        }

        async fn until_it_ends(&mut self) -> Ending {
            std::future::pending().await
        }

        async fn torn_down(self) -> Ending {
            Ending::TornDownBySmoozy
        }
    }

    struct APointLending {
        sessions: ThisServersSessions<AConnectionToIt>,
        lending: tokio::task::JoinHandle<Ending>,
    }

    impl APointLending {
        async fn over(machine: &Arc<AMachine>) -> APointLending {
            let sessions = ThisServersSessions::<AConnectionToIt>::for_one_server();
            let more = Arc::clone(machine);
            let additional: TheAdditionalConnection<AConnectionToIt> = Arc::new(move || {
                let more = Arc::clone(&more);
                Box::pin(async move { Some(AConnectionToIt(more)) })
            });
            let mut primary = AConnectionToIt(Arc::clone(machine));
            let lending = tokio::spawn({
                let sessions = sessions.clone();
                async move { lend_until_it_ends(&sessions, &mut primary, &additional).await }
            });
            while !sessions.a_connection_is_live() {
                tokio::task::yield_now().await;
            }
            APointLending { sessions, lending }
        }
    }

    impl Drop for APointLending {
        fn drop(&mut self) {
            self.lending.abort();
        }
    }

    /// **What a search came back with, as the list shows it**: each build's
    /// path beside the version it was recorded as answering.
    fn pairs(asked: TheVersionListAsk) -> Vec<(String, Option<Version>)> {
        match asked {
            TheVersionListAsk::Answered(list) => list
                .entries
                .into_iter()
                .map(|entry| (entry.path, entry.answered))
                .collect(),
            TheVersionListAsk::TheMachineCouldNotBeAsked => Vec::new(),
        }
    }

    #[tokio::test]
    async fn the_pages_search_beside_the_set_up_tabs_pairs_each_build_with_its_own_version() {
        let Some(machine) = AMachine::standing("versions") else {
            println!("SKIPPED: no sftp-server on this system");
            return;
        };
        let point = APointLending::over(&machine).await;

        // **The page asks first, and the Set up tab is drawn half a round trip
        // later** — so the two searches reach the link one candidate apart.
        let (the_pages, the_tabs) = tokio::join!(
            super::the_blenders_on_the_machine(&point.sessions, THIS_SERVER, None),
            async {
                tokio::time::sleep(A_ROUND_TRIP / 2).await;
                super::the_blenders_on_the_machine(&point.sessions, THIS_SERVER, None).await
            },
        );

        let what_is_there = vec![
            (
                machine.the_build_in(THE_TWO_BUILDS[0].0),
                Version::parse("4.2.3"),
            ),
            (
                machine.the_build_in(THE_TWO_BUILDS[1].0),
                Version::parse("4.5.1"),
            ),
        ];
        // **Both together, so a failure shows what each list would have
        // offered the user.**
        assert_eq!(
            (pairs(the_pages), pairs(the_tabs)),
            (what_is_there.clone(), what_is_there),
            "(the page's list, the Set up tab's list)"
        );
        assert!(
            std::fs::symlink_metadata(
                machine.home.join(
                    smoozy_server_setup::readings::THE_VERSION_ASKS_CARRIAGE_SITE
                        .link_path_on_the_subsystem
                )
            )
            .is_err(),
            "the link is taken away"
        );
    }
}
