//! **Read the file, driven** — `Render Settings` Q4's Pre-load over `Server
//! protocol and transfers` T5's per-server point. Built at step 10c's
//! dependency wave piece 3, 2026-09-29.
//!
//! **What this file is: the desktop's real end of `Render Settings`' one seam
//! for a reading, `DrivesOneFileInspection`**, which that crate's `pre_load`
//! drives and which nothing implemented outside its own tests. Everything
//! `pre_load` holds — the withholding, the join, the discard and the
//! judgement of `Render Settings` B5a, `Render Settings` Q5, `Render Settings` Q17, `Render Settings` Q7 and `Render Settings` Q8 — is that crate's and is used here unchanged. **What is here is
//! the route, and the two things the page supplies at the landing.**
//!
//! # The route
//!
//! **One request, *a reading of a `.blend`*, asking for the file's reading**
//! — the request `Server protocol and transfers` names for both readings,
//! answered by the file-inspection worker `Render Settings` W1 declares, which
//! starts the build by its own path with `Job files and outputs` R7's line and
//! `Job files and outputs` R1's script composed with no job frames and no job scene. The request is
//! asked, and asked again under its own identity until the machine says the
//! reading has a recorded outcome (`Server protocol and transfers` C4, `Server protocol and transfers` C8).
//!
//! **The document comes back through the worker's captured stream and never
//! inside the answer — the runner's ruling of 2026-09-28.** One message's
//! ceiling is one MiB and a file's reading can be longer: the 5.2 splash
//! scene's is 277 KB for 250 frames, so a scene of roughly 950 frames or more
//! would not fit. The answer names the two streams and their lengths, the exit
//! status, whether the start mark was seen and whether Smoozy ended the run;
//! the streams are read back over the reading session to their stated
//! lengths, and the document is found between `Job files and outputs` R1's
//! two sentinels by that crate's own reader.
//!
//! **What comes back is handed to `Render Settings`' judgement as its own
//! frame** — the channel's outcome, the two sentinels, the error map's
//! emptiness, the exit status and whether Smoozy ended the run — and this file
//! never names an ending itself. Where the machine's answer has no field that
//! frame carries, the frame is composed so that the judgement lands on the arm
//! that claims nothing about the file, and each such place says why.
//!
//! # What the page supplies at the landing, for `Render Settings` Q17
//!
//! **The four values the read started under, and the four values standing
//! when it lands**, both read fresh by `Render Settings` Q14 item 2's own
//! command-channel reading over the same point. A file replaced, moved or
//! published over while the read ran reads back different, and the reading is
//! discarded rather than stored. **The version pick now** is the page's own
//! resolution, taken from the run's memory at the landing.
//!
//! # The outputs half, and only for a file of one scene
//!
//! **The outputs half the same document carries describes one scene, and does
//! not say which** — the script reads `bpy.context.scene` where no job scene
//! is named — so on a file with two scenes, *What it will write* for the
//! second would be the first's. Established at the desk on a two-scene file
//! (`Audits/Build/Step 10c/Dependency wave piece 3 — what landed.md`). **The
//! runner ruled on 2026-09-29 that it is kept only where the file has one
//! scene** (`Design/Render Settings § What it will write`), where that scene
//! is the only one it can be. It is composed by `Job files and outputs`' own
//! [`the_reading_and_everything_derived_from`], the one composition both
//! sides call, and handed to the page beside the settings half at the
//! storing.
//!
//! # The one case beside `Render Settings` Q8's four
//!
//! **Where the machine answers and runs no Blender** — a program that does not
//! know the request, or a reading it did not admit — the frame lands on
//! `Render Settings`' explicit-absence arm (see [`the_machine_did_not_take_it`]),
//! which a real Blender that printed no start mark also lands on. **So which
//! of the two it was is kept here, per file, by the read that ran**, and the
//! landing says [`WhatTheReadCameTo::TheServerRanNoBlender`] for it — for a
//! press that joined the running read too, which is why it is kept per file
//! rather than per press.
//!
//! **Every marker in this file that belongs to another document is written out
//! with that document at its own site, and this file keeps no header list** —
//! the documents it cites share most of their letters with this crate's own
//! plan.

use std::future::Future;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, SystemTime};

use smoozy_dependencies::acting::checking::{
    read_a_stream_back_over_the_file_session, TheMachineAReadingIsAskedOf,
    BETWEEN_TWO_ASKS_OF_ONE_READING,
};
use smoozy_job_files::composing::{
    the_reading_and_everything_derived_from, WhatTheReadingIsComposedAgainst,
};
use smoozy_job_files::main_output::WhereTheRenderEnters;
use smoozy_job_files::reading::{
    the_reading_started_in, ArtistsText, TheOutputsHalf, WhyTheDocumentWasNotRead,
};
use smoozy_job_files::reading_script::{
    the_printed_reading_in, AnotherScenesHalf, TheOtherScenes, ThePrintedReading,
};
use smoozy_render_settings::acting::pre_load::{
    pre_load, ABlendToRead, DrivesOneFileInspection, HoldsTheFilesHalf, HowThePreLoadWent,
    ReadsInFlight, TheFourValuesNowStanding, ThePageTheReadingArrivesAt,
};
use smoozy_render_settings::endings::{ChannelOutcome, Ending, ExitStatus, WhatTheInvocationLeft};
use smoozy_render_settings::file_half::{
    version_text, BlenderIdentity, FileKey, Landed, StartedUnder,
};
use smoozy_render_settings::reading::SettingsHalf;
use smoozy_render_settings::readings::FourValuesNow;
use smoozy_server_protocol::asking_the_supervisor::WhatCameBack;
use smoozy_server_protocol::framing::{
    Answered, Fields, Request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
};
use smoozy_server_protocol::requests as asked;
use smoozy_server_protocol::transfers::SourceReading;
use smoozy_state::identities::{Identity, IdentityKind};

use crate::servers::sessions::{AHeldConnection, ThisServersSessions};
use crate::surface::render_settings::WhatTheRunHolds;

/// **The name `Render Settings`' diary declaration is registered under** —
/// `Desktop app and delivery` A7's hosting rule reads a module's declaration by
/// this name, and the one act line that module declares (*a `.blend` read*) is
/// written under it.
pub const THE_MODULE: &str = "Render Settings";

/// **The bound on one whole reading, as the desk waits on it** — `Server
/// Setup` B1's one number for every Blender run the desktop waits on, the same
/// the check's reading carries. **What ends at it is the wait and not the
/// run**; the waiter's expired deadline is `Server protocol and transfers`
/// C16 and `Server protocol and transfers` C17's *the waiter gave up*, which `Render Settings`' frame puts
/// with the channel's own ending.
pub const THE_BOUND_ON_ONE_READING: Duration =
    smoozy_dependencies::acting::blender::THE_TIME_LIMIT_A_BLENDER_RUN_CARRIES;

// ════════════════════════════════════════════════════════════════════════════
// The machine, over the per-server point
// ════════════════════════════════════════════════════════════════════════════

/// **What a file's reading needs of a machine**: the supervisor asked, a
/// captured stream read back, and `Render Settings` Q14 item 2's four values.
pub trait TheMachineAFileIsReadOn: TheMachineAReadingIsAskedOf {
    /// **`Job files and outputs` R10's four values of the `.blend` as they
    /// stand now**, by `Render Settings`' own command-channel reading.
    fn the_four_values_of(&mut self, the_blend: &str)
        -> impl Future<Output = FourValuesNow> + Send;

    /// **The names standing in one folder on the machine**, read once on the
    /// file session — which `Dependency Upload & Remap` Q5's read-through asks
    /// first, so a file not there costs no worker. `Some` of nothing where the
    /// folder is not there; `None` where it could not be read. Added
    /// 2026-10-04, the 38's piece 7; **a machine that answers nothing of the
    /// file session answers `None`**, which asks the read-through of nothing.
    fn the_names_in(&mut self, _folder: &[u8]) -> impl Future<Output = Option<Vec<Vec<u8>>>> + Send {
        async { None }
    }

    /// **`Dependency Upload & Remap` M14's folders, made with A7's plain form
    /// segment by segment**: every folder above `inside`, which is the
    /// staging path beside the fixed copy's address. `false` where one could
    /// not be made. The 38's piece 7.
    fn the_folders_above_are_made(&mut self, _inside: &[u8]) -> impl Future<Output = bool> + Send {
        async { false }
    }

    /// **M14's publication**: the fixing's staged copy renamed over its
    /// address, one rename, and the staged copy removed where the rename is
    /// refused (`smoozy_dependencies::acting::fixing_worker::the_fixed_copy_is_published`).
    /// The 38's piece 7.
    fn the_fixed_copy_is_published(
        &mut self,
        _fixing: &smoozy_dependencies::acting::fixing_worker::OneFixing,
    ) -> impl Future<Output = smoozy_dependencies::acting::fixing_worker::HowTheFixingWent> + Send {
        async {
            smoozy_dependencies::acting::fixing_worker::HowTheFixingWent::ItCouldNotBeFixed(
                smoozy_dependencies::fixing::WhyItCouldNotBeFixed::TheProductionDidNotPublish,
            )
        }
    }

    /// **A staged copy that will not be published, removed** — `Dependency
    /// Upload & Remap` §10's *staged, saved, published by one rename, or
    /// nothing*. A removal that fails says nothing: the staging sweep's
    /// (`Dependency Upload & Remap` O1). The 38's piece 7.
    fn the_staged_copy_is_removed(&mut self, _staging: &[u8]) -> impl Future<Output = ()> + Send {
        async {}
    }
}

/// **The machine a reading is asked of, as `Server protocol and transfers` T5's
/// per-server point lends it**: the supervisor asked through the point, with
/// the package area the server row records, and every stream read back on the
/// point's reading session.
pub struct ThisServersMachine<'s, C: AHeldConnection> {
    pub sessions: &'s ThisServersSessions<C>,
    pub package_area: &'s str,
    /// **The local server identifier `sessions` lends for** — what the four
    /// values' reading takes its turn at the one link site by (step 10c's
    /// eleventh-pass fix piece 4b).
    pub server: &'s str,
}

impl<C: AHeldConnection> TheMachineAReadingIsAskedOf for ThisServersMachine<'_, C> {
    async fn ask(&mut self, request: &Request) -> WhatCameBack {
        self.sessions
            .ask_the_supervisor(self.package_area, request)
            .await
    }

    /// **One captured stream, on the reading session, to its stated length**
    /// — `Dependency Upload & Remap`'s own read-back, which already holds the
    /// stated length as the whole answer and a short one as an error in
    /// words.
    async fn read_the_stream_back(&mut self, path: &[u8], length: u64) -> Result<Vec<u8>, String> {
        let path = path.to_vec();
        let read = self
            .sessions
            .a_reading(true, move |subsystem| {
                Box::pin(async move {
                    Ok(read_a_stream_back_over_the_file_session(subsystem, &path, length).await)
                })
            })
            .await;
        match read {
            Ok(Ok(read)) => read,
            Ok(Err(failure)) => Err(format!("{failure:?}")),
            Err(not_lent) => Err(format!("{not_lent:?}")),
        }
    }
}

impl<C: AHeldConnection> TheMachineAFileIsReadOn for ThisServersMachine<'_, C> {
    async fn the_four_values_of(&mut self, the_blend: &str) -> FourValuesNow {
        crate::submitting::the_four_values_of(self.sessions, self.server, the_blend).await
    }

    async fn the_names_in(&mut self, folder: &[u8]) -> Option<Vec<Vec<u8>>> {
        use smoozy_server_protocol::subsystem::{StatusCode, SubsystemFailure};
        let path = folder.to_vec();
        match self
            .sessions
            .a_reading(true, move |subsystem| {
                Box::pin(async move { Ok(subsystem.list(&path).await) })
            })
            .await
        {
            Ok(Ok(Ok(entries))) => Some(entries.into_iter().map(|entry| entry.name).collect()),
            Ok(Ok(Err(SubsystemFailure::TheServerRefused(StatusCode::NoSuchFile)))) => Some(Vec::new()),
            _ => None,
        }
    }

    async fn the_folders_above_are_made(&mut self, inside: &[u8]) -> bool {
        use smoozy_dependencies::acting::sending::{
            make_the_mirror_directories, open_an_asset_upload, HowTheDirectoryWent,
            TheMirrorDirectoriesSeen,
        };
        let Ok(witness) = open_an_asset_upload();
        let at = smoozy_dependencies::bytes::MachineLocation::composed(inside.to_vec());
        matches!(
            self.sessions
                .a_reading(true, move |subsystem| {
                    Box::pin(async move {
                        let seen = TheMirrorDirectoriesSeen::default();
                        Ok(make_the_mirror_directories(&witness, subsystem, &at, &seen).await)
                    })
                })
                .await,
            Ok(Ok(HowTheDirectoryWent::ItStands))
        )
    }

    async fn the_fixed_copy_is_published(
        &mut self,
        fixing: &smoozy_dependencies::acting::fixing_worker::OneFixing,
    ) -> smoozy_dependencies::acting::fixing_worker::HowTheFixingWent {
        use smoozy_dependencies::acting::fixing_worker::HowTheFixingWent;
        let fixing = fixing.clone();
        match self
            .sessions
            .a_reading(true, move |subsystem| {
                Box::pin(async move {
                    Ok(smoozy_dependencies::acting::fixing_worker::the_fixed_copy_is_published(
                        subsystem, &fixing,
                    )
                    .await)
                })
            })
            .await
        {
            Ok(Ok(went)) => went,
            _ => HowTheFixingWent::ItCouldNotBeFixed(
                smoozy_dependencies::fixing::WhyItCouldNotBeFixed::TheProductionDidNotPublish,
            ),
        }
    }

    async fn the_staged_copy_is_removed(&mut self, staging: &[u8]) {
        let path = staging.to_vec();
        let _ = self
            .sessions
            .a_reading(false, move |subsystem| {
                Box::pin(async move { Ok(subsystem.remove(&path).await) })
            })
            .await;
    }
}

// ════════════════════════════════════════════════════════════════════════════
// The request, and what its answer left
// ════════════════════════════════════════════════════════════════════════════

/// **The one request, *a reading of a `.blend`*, asking for the file's
/// reading** — the fields `Server protocol and transfers` names for it: which
/// reading, the build, the `.blend` and the version its script is composed
/// for. **Every path is a protocol field and never a word of a command.**
pub fn the_files_reading_asked(operation: &Identity, of: &ABlendToRead<'_>) -> Request {
    Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        operation.clone(),
        asked::THE_READING_OF_A_BLEND_REQUEST,
    )
    .carrying(
        Fields::none()
            .and(asked::THE_WHICH_READING_FIELD, asked::THE_FILES_READING)
            .and(asked::THE_BUILD_FIELD, of.with.path.as_bytes())
            .and(asked::THE_BLEND_FIELD, of.the_blends_path.as_bytes())
            .and(
                asked::THE_BLENDER_VERSION_FIELD,
                version_text(of.with.version),
            ),
    )
}

/// **The channel's own ending** — `Render Settings` Q8's first, which says
/// nothing about the file. Every other field is the one that claims nothing.
fn the_machine_could_not_be_reached() -> WhatTheInvocationLeft {
    WhatTheInvocationLeft {
        channel: ChannelOutcome::TheMachineCouldNotBeReached,
        smoozy_ended_it: false,
        exit: ExitStatus::Failed,
        start_sentinel_seen: false,
        end_sentinel_seen: false,
        error_map_is_empty: None,
        document: None,
    }
}

/// **The machine answered and ran no Blender**: its program does not know the
/// request, or it did not admit the reading, in its own words.
///
/// **No Blender ran, so there is no exit status to report, and the frame is
/// composed to land on the one arm of `Render Settings`' judgement that claims
/// nothing about the file** — a completed invocation at a succeeding status
/// with no start sentinel, which that crate keeps as *an explicit absence
/// rather than a label guessed at*. **A failing status would land on `Render
/// Settings` Q8's second ending**, and the page would tell the user that this Blender cannot
/// open their file, which nothing here found.
fn the_machine_did_not_take_it() -> WhatTheInvocationLeft {
    WhatTheInvocationLeft {
        channel: ChannelOutcome::TheInvocationAnswered,
        smoozy_ended_it: false,
        exit: ExitStatus::Succeeded,
        start_sentinel_seen: false,
        end_sentinel_seen: false,
        error_map_is_empty: None,
        document: None,
    }
}

/// **What one file's reading left**: `Render Settings`' own frame, and the
/// two things that frame has no field for.
#[derive(Debug)]
pub struct WhatTheReadingLeft {
    pub frame: WhatTheInvocationLeft,
    /// **The machine answered and ran no Blender** ([`the_machine_did_not_take_it`]).
    pub no_blender_ran: bool,
    /// **The whole document, where one was read**, for the outputs half.
    pub printed: Option<ThePrintedReading>,
    /// **`Job files and outputs` R4b: every other scene's outputs half**, out
    /// of the same document. `None` from a machine program older than that clause,
    /// whose reading holds one scene.
    pub the_other_scenes: Option<Vec<AnotherScenesHalf>>,
}

impl WhatTheReadingLeft {
    fn only(frame: WhatTheInvocationLeft) -> WhatTheReadingLeft {
        WhatTheReadingLeft {
            frame,
            no_blender_ran: false,
            printed: None,
            the_other_scenes: None,
        }
    }
}

/// **One file's reading, asked until it ends, and what it left** —
/// `Render Settings` Q4 over the one request.
///
/// **The bound and the pace are handed in** so a test can watch the bound
/// fire without waiting a minute; the one caller hands in
/// [`THE_BOUND_ON_ONE_READING`] and the check's own pace.
pub async fn what_the_files_reading_left<M>(
    machine: &mut M,
    request: &Request,
    bound: Duration,
    pace: Duration,
) -> WhatTheReadingLeft
where
    M: TheMachineAReadingIsAskedOf,
{
    let no_blender_ran = || WhatTheReadingLeft {
        frame: the_machine_did_not_take_it(),
        no_blender_ran: true,
        printed: None,
        the_other_scenes: None,
    };
    let could_not_be_reached = || WhatTheReadingLeft::only(the_machine_could_not_be_reached());
    let deadline = tokio::time::Instant::now() + bound;
    let ended = loop {
        let answered = match tokio::time::timeout_at(deadline, machine.ask(request)).await {
            // **`Server protocol and transfers` C16 and `Server protocol and transfers` C17: a waiter's
            // expired deadline says the waiter gave up**, and `Render
            // Settings`' frame puts it with the channel's own ending.
            Err(_) => return could_not_be_reached(),
            Ok(WhatCameBack::ItAnswered(answered)) => answered,
            Ok(WhatCameBack::NothingCorrelatedCameBack) | Ok(WhatCameBack::ItWasNeverAsked) => {
                return could_not_be_reached()
            }
        };
        if answered
            .the_request_name_it_says_it_does_not_answer()
            .is_some()
        {
            return no_blender_ran();
        }
        match answered.fields.text(asked::WHAT_BECAME_OF_IT_FIELD) {
            Some(asked::IT_HAS_A_RECORDED_OUTCOME) => break answered,
            Some(asked::IT_IS_STILL_IN_HAND) => {}
            _ => return no_blender_ran(),
        }
        if tokio::time::Instant::now() + pace >= deadline {
            return could_not_be_reached();
        }
        tokio::time::sleep(pace).await;
    };
    what_the_ended_reading_left(machine, &ended).await
}

/// **What a reading with a recorded outcome left**, in `Render Settings`' own
/// frame, from the answer's fields and the two streams it names.
async fn what_the_ended_reading_left<M>(machine: &mut M, ended: &Answered) -> WhatTheReadingLeft
where
    M: TheMachineAReadingIsAskedOf,
{
    // **`Render Settings` Q8: a read Smoozy ended itself is none of the three**, and this is the
    // machine's own record of whose ending it was — never an inference.
    let smoozy_ended_it = ended.fields.text(asked::THE_RUN_WAS_ENDED_BY_SMOOZY_FIELD)
        == Some(asked::THE_ANSWER_IS_YES);
    let exit = match ended.fields.text(asked::THE_EXIT_STATUS_FIELD) {
        Some("0") => ExitStatus::Succeeded,
        // A status other than nought, or none at all — a run ended by a
        // signal, or a build that could not be started.
        _ => ExitStatus::Failed,
    };
    let the_machine_saw_the_start_mark =
        ended.fields.text(asked::THE_START_MARK_SEEN_FIELD) == Some(asked::THE_ANSWER_IS_YES);
    let stream = |at: &str, length: &str| -> Option<(Vec<u8>, u64)> {
        let path = ended.fields.value(at)?.to_vec();
        let length = ended.fields.text(length)?.parse().ok()?;
        Some((path, length))
    };
    let named = (
        stream(
            asked::THE_ORDINARY_STREAM_FIELD,
            asked::THE_ORDINARY_STREAM_LENGTH_FIELD,
        ),
        stream(
            asked::THE_COMPLAINTS_STREAM_FIELD,
            asked::THE_COMPLAINTS_STREAM_LENGTH_FIELD,
        ),
    );
    // **A reading whose worker started no Blender names no stream**: a build
    // that could not be run. It is a failing status with nothing read back —
    // `Render Settings` Q8's second ending, which names the version Smoozy ran — and the streams
    // are never invented as empty.
    let (Some(ordinary), Some(complaints)) = named else {
        return WhatTheReadingLeft::only(WhatTheInvocationLeft {
            channel: ChannelOutcome::TheInvocationAnswered,
            smoozy_ended_it,
            exit,
            start_sentinel_seen: the_machine_saw_the_start_mark,
            end_sentinel_seen: false,
            error_map_is_empty: None,
            document: None,
        });
    };
    // **`Render Settings` Q10: the connection dropping while the streams are read back is the
    // channel's own ending**, and nothing half-read is judged.
    let Ok(ordinary) = machine.read_the_stream_back(&ordinary.0, ordinary.1).await else {
        return WhatTheReadingLeft::only(the_machine_could_not_be_reached());
    };
    let Ok(complaints) = machine
        .read_the_stream_back(&complaints.0, complaints.1)
        .await
    else {
        return WhatTheReadingLeft::only(the_machine_could_not_be_reached());
    };
    what_the_streams_left(
        &ordinary,
        &complaints,
        smoozy_ended_it,
        exit,
        the_machine_saw_the_start_mark,
    )
}

/// **`Render Settings` Q7's material, from the two streams**: the start sentinel, the whole
/// document between both, its error map and its settings half.
///
/// **The document is looked for on the ordinary stream first and then on the
/// complaints**, the same as the check's reading, and it is read by `Job files
/// and outputs`' own reader — one spelling of each sentinel, not two.
pub fn what_the_streams_left(
    ordinary: &[u8],
    complaints: &[u8],
    smoozy_ended_it: bool,
    exit: ExitStatus,
    the_machine_saw_the_start_mark: bool,
) -> WhatTheReadingLeft {
    // **`Render Settings` Q8's start sentinel is the mark the script writes as
    // its very first act** (added 2026-10-02, the Local View crash), so a
    // Blender that crashed partway through the script is the third ending —
    // *reading the file did not finish*, with *Blender output* — and never
    // *this Blender could not open this file*. A machine whose program predates
    // that mark composed a script that never writes it, and there the opening
    // mark still shows the start, which is what `the_reading_started_in` reads.
    let start_sentinel_seen = the_machine_saw_the_start_mark
        || the_reading_started_in(ordinary)
        || the_reading_started_in(complaints);
    let (printed, the_other_scenes) = match the_printed_reading_in(ordinary) {
        Err(WhyTheDocumentWasNotRead::TheSentinelsWereNotBothThere) => (
            the_printed_reading_in(complaints),
            TheOtherScenes::in_the_stream(complaints).the_other_scenes,
        ),
        read => (read, TheOtherScenes::in_the_stream(ordinary).the_other_scenes),
    };
    let (end_sentinel_seen, error_map_is_empty, document, printed) = match printed {
        Ok(printed) => {
            let error_map_is_empty = printed.errors.is_empty();
            // **The settings half is `Render Settings`' own shape**, carried
            // by the reading and not interpreted by it; a half that will not
            // read into that shape is no document, and `Render Settings` Q7
            // then makes it no reading.
            let settings =
                <SettingsHalf as serde::Deserialize>::deserialize(&printed.settings.0).ok();
            (true, Some(error_map_is_empty), settings, Some(printed))
        }
        // Both marks were there and what sat between them is not a document.
        Err(WhyTheDocumentWasNotRead::WhatSatBetweenTheMarksIsNotADocument) => {
            (true, None, None, None)
        }
        Err(_) => (false, None, None, None),
    };
    WhatTheReadingLeft {
        frame: WhatTheInvocationLeft {
            channel: ChannelOutcome::TheInvocationAnswered,
            smoozy_ended_it,
            exit,
            start_sentinel_seen,
            end_sentinel_seen,
            error_map_is_empty,
            document,
        },
        no_blender_ran: false,
        the_other_scenes: printed.as_ref().and(the_other_scenes),
        printed,
    }
}

/// **`§ What it will write`'s material: the outputs half, with the scene it
/// was read from** — `Design/Render Settings § What it will write`, corrected
/// 2026-10-03 (the 38's piece 10, item 25): *"the read fills this group where
/// the scene it read is the one ticked."* **The reading names the scene it
/// read** (`Job files and outputs`' printed reading); a reading from a script
/// older than that names none, and then only a file of one scene says which —
/// it can only be that one. **Nothing where neither says**, which is the
/// group as it is before a read.
///
/// **Composed by `Job files and outputs`' own composition**, the one both
/// sides call (that crate's `composing`): the destinations filtered to those
/// that will be written, and the beauty walk entered at the scene's first
/// view layer, which is where `Rendering`'s render worker enters it for a job
/// with no pin. **The map it also composes is not kept**: the page composes
/// its own for what it draws, and the render composes the job's at its start.
pub fn the_outputs_half_and_the_scene_it_was_read_from(
    printed: ThePrintedReading,
    settings: Option<&SettingsHalf>,
    the_blends_path: &str,
) -> Option<(Vec<u8>, TheOutputsHalf)> {
    let scenes = &settings?.scenes;
    let read_from: Vec<u8> = match (&printed.the_scene_it_read, scenes.as_slice()) {
        (Some(named), _) => named.as_bytes().to_vec(),
        (None, [scene]) => scene.name.as_bytes().to_vec(),
        (None, _) => return None,
    };
    let scene = scenes.iter().find(|scene| scene.name.as_bytes() == read_from.as_slice())?;
    let enters_at = WhereTheRenderEnters {
        scene: ArtistsText::of(scene.name.as_bytes().to_vec()),
        view_layer: ArtistsText::of(
            scene
                .view_layers
                .first()
                .map(|layer| layer.name.as_bytes().to_vec())
                .unwrap_or_default(),
        ),
    };
    let folder = the_blends_path
        .rsplit_once('/')
        .map(|(folder, _)| folder)
        .unwrap_or_default();
    the_reading_and_everything_derived_from(
        printed,
        WhatTheReadingIsComposedAgainst {
            the_folder_the_blend_sits_in: folder.as_bytes(),
            job_output_folder: THE_JOB_OUTPUT_FOLDER_NO_JOB_HAS_YET,
            enters_at: &enters_at,
            // **No job yet, so no pins** (`Job files and outputs` R4 item 5):
            // what this read keeps is the outputs half, which is the file's own.
            the_jobs_pins: &[],
        },
    )
    .ok()
    .map(|composed| (read_from, composed.reading.outputs))
}

/// **`Job files and outputs` R4b: every other scene's outputs half, each with
/// its scene** — each half made into the reading naming that scene, which the
/// desk proved it the same as (§14 item 30), and composed by
/// [`the_outputs_half_and_the_scene_it_was_read_from`] exactly as the read
/// scene's is. **A scene whose half holds errors has none**, as a reading
/// naming it would be no reading (`Render Settings` Q7). Nothing from a
/// reading that holds one scene.
pub fn the_other_scenes_outputs_halves(
    printed: &ThePrintedReading,
    the_other_scenes: Option<&[AnotherScenesHalf]>,
    settings: Option<&SettingsHalf>,
    the_blends_path: &str,
) -> Vec<(Vec<u8>, TheOutputsHalf)> {
    the_other_scenes
        .unwrap_or_default()
        .iter()
        .filter(|half| half.errors.is_empty())
        .filter_map(|half| printed.as_a_reading_naming(half))
        .filter_map(|named| the_outputs_half_and_the_scene_it_was_read_from(named, settings, the_blends_path))
        .collect()
}

/// **A job output folder for the composition's map, which is not kept** —
/// there is no job yet, and the outputs half does not depend on it.
const THE_JOB_OUTPUT_FOLDER_NO_JOB_HAS_YET: &str = "/ephemeral/smoozy/jobs/job";

/// **Which files' latest read the machine answered without running a
/// Blender**, keyed by (server, `.blend`). **Written by the read that ran,
/// before its outcome goes out**, so a press that joined it lands the same
/// case; cleared by the next read of that file. One for the run, like
/// [`the_reads_in_flight`].
fn the_reads_that_ran_no_blender() -> &'static Mutex<std::collections::BTreeSet<(String, String)>> {
    static RAN_NO_BLENDER: OnceLock<Mutex<std::collections::BTreeSet<(String, String)>>> =
        OnceLock::new();
    RAN_NO_BLENDER.get_or_init(Mutex::default)
}

// ════════════════════════════════════════════════════════════════════════════
// The seam, implemented
// ════════════════════════════════════════════════════════════════════════════

/// **What the landing needs that the read learnt**: the four values standing
/// when the read ended (`Render Settings` Q17), the operation identity the read went out
/// under — the one the diary's act line carries (`Render Settings` §12) — and
/// the outputs half, where the file has one scene.
#[derive(Debug, Default)]
pub struct WhatTheReadLearnt {
    pub the_four_values_now: Option<SourceReading>,
    pub operation: Option<Identity>,
    /// With the scene it was read from (the 38's piece 10, item 25).
    pub the_outputs_half: Option<(Vec<u8>, TheOutputsHalf)>,
    /// **`Job files and outputs` R4b: every other scene's**, each with its
    /// scene; empty from a reading that holds one scene.
    pub the_other_scenes_outputs_halves: Vec<(Vec<u8>, TheOutputsHalf)>,
    /// **The scene the reading names as the one it read**, by its own bytes,
    /// or nothing where a script older than 2026-10-03 named none — what
    /// `Render Settings` J7's third naming act ticks (the Creator's note 5 on
    /// the test copy). Kept apart from the outputs half, which a composition
    /// that fails leaves out, so the tick never falls to the first scene for
    /// that reason.
    pub the_scene_it_read: Option<Vec<u8>>,
}

/// **`Render Settings`' `DrivesOneFileInspection`, over a machine.**
pub struct TheFilesReadingOn<M> {
    pub machine: M,
    pub learnt: Arc<Mutex<WhatTheReadLearnt>>,
    pub bound: Duration,
    pub pace: Duration,
}

impl<M> TheFilesReadingOn<M> {
    pub fn over(machine: M) -> TheFilesReadingOn<M> {
        TheFilesReadingOn {
            machine,
            learnt: Arc::default(),
            bound: THE_BOUND_ON_ONE_READING,
            pace: BETWEEN_TWO_ASKS_OF_ONE_READING,
        }
    }
}

impl<M: TheMachineAFileIsReadOn + Send> DrivesOneFileInspection for TheFilesReadingOn<M> {
    async fn read_one_blend(&mut self, of: ABlendToRead<'_>) -> WhatTheInvocationLeft {
        // **`Server protocol and transfers` C1: the identity is issued before
        // the request leaves.** A computer that cannot issue one sends
        // nothing.
        let Ok(operation) = Identity::issue(IdentityKind::Operation) else {
            return the_machine_could_not_be_reached();
        };
        let request = the_files_reading_asked(&operation, &of);
        let the_file = (of.server.as_text(), of.the_blends_path.to_string());
        if let Ok(mut ran_no_blender) = the_reads_that_ran_no_blender().lock() {
            ran_no_blender.remove(&the_file);
        }
        let WhatTheReadingLeft {
            frame: left,
            no_blender_ran,
            printed,
            the_other_scenes,
        } = what_the_files_reading_left(&mut self.machine, &request, self.bound, self.pace).await;
        if no_blender_ran {
            if let Ok(mut ran_no_blender) = the_reads_that_ran_no_blender().lock() {
                ran_no_blender.insert(the_file);
            }
        }
        let the_scene_it_read = printed
            .as_ref()
            .and_then(|printed| printed.the_scene_it_read.as_ref())
            .map(|named| named.as_bytes().to_vec());
        let the_other_scenes_outputs_halves = printed
            .as_ref()
            .map(|printed| {
                the_other_scenes_outputs_halves(
                    printed,
                    the_other_scenes.as_deref(),
                    left.document.as_ref(),
                    of.the_blends_path,
                )
            })
            .unwrap_or_default();
        let the_outputs_half = printed.and_then(|printed| {
            the_outputs_half_and_the_scene_it_was_read_from(
                printed,
                left.document.as_ref(),
                of.the_blends_path,
            )
        });
        // **`Render Settings` Q17's second test's material: the four values
        // standing now**, read
        // after the invocation ended. A machine that cannot answer leaves
        // nothing standing, and the reading is then discarded rather than
        // stored — the safe direction that crate names.
        let now = match left.channel {
            ChannelOutcome::TheMachineCouldNotBeReached => None,
            ChannelOutcome::TheInvocationAnswered => {
                match self.machine.the_four_values_of(of.the_blends_path).await {
                    FourValuesNow::Answered(now) => Some(now),
                    FourValuesNow::TheChannelCannotAnswer => None,
                }
            }
        };
        if let Ok(mut learnt) = self.learnt.lock() {
            learnt.the_four_values_now = now;
            learnt.operation = Some(operation);
            learnt.the_outputs_half = the_outputs_half;
            learnt.the_other_scenes_outputs_halves = the_other_scenes_outputs_halves;
            learnt.the_scene_it_read = the_scene_it_read;
        }
        left
    }
}

/// **The page as it stands when the reading lands** — `Render Settings`
/// Q17's two tests, asked
/// of the run's memory and of what the read learnt, never remembered from the
/// press.
pub struct ThePageAtTheLanding<'h> {
    pub held: &'h Mutex<WhatTheRunHolds>,
    pub server: &'h str,
    pub file: &'h str,
    pub learnt: Arc<Mutex<WhatTheReadLearnt>>,
}

impl ThePageTheReadingArrivesAt for ThePageAtTheLanding<'_> {
    fn the_version_pick_now(&self, _key: &FileKey) -> Option<BlenderIdentity> {
        let held = self.held.lock().ok()?;
        crate::surface::render_settings::the_version_resolved(&held, self.server, self.file)
            .the_payloads_pick()
    }

    fn the_four_values_now_standing(&self, _key: &FileKey) -> TheFourValuesNowStanding {
        match self
            .learnt
            .lock()
            .ok()
            .and_then(|learnt| learnt.the_four_values_now.clone())
        {
            Some(now) => TheFourValuesNowStanding::These(now),
            None => TheFourValuesNowStanding::NoneStands,
        }
    }
}

/// **The file's half this run holds, reached for the one act of storing.**
///
/// **`Render Settings` Q18: the moment is stamped at the act of storing the
/// reading.** `pre_load` is handed a moment when it is called, which is before
/// the read runs, so this stamps the desktop's clock here, at the storing, and
/// the handed moment is not used.
///
/// **The outputs half goes down in the same act**, from what the read learnt:
/// kept where the file has one scene and dropped where it has more, so a
/// stored reading never stands beside another reading's outputs.
///
/// **And so does `Render Settings` J7's third naming act** — the read's own
/// tick of the scene it names as the file's saved one — so no page is ever
/// drawn from a stored reading with no scene ticked (the Creator's note 5 on
/// the test copy).
pub struct TheFilesHalfOfThisRun<'h> {
    pub held: &'h Mutex<WhatTheRunHolds>,
    pub server: &'h str,
    pub file: &'h str,
    pub learnt: Arc<Mutex<WhatTheReadLearnt>>,
}

impl HoldsTheFilesHalf for TheFilesHalfOfThisRun<'_> {
    fn a_reading_landed(
        &self,
        key: FileKey,
        started_under: &StartedUnder,
        settings: SettingsHalf,
        the_pages_current_pick: Option<&BlenderIdentity>,
        the_four_values_now: &SourceReading,
        _handed_at_the_press: SystemTime,
    ) -> Landed {
        match self.held.lock() {
            Ok(mut held) => {
                let landed = held.readings.a_reading_landed(
                    key,
                    started_under,
                    settings,
                    the_pages_current_pick,
                    the_four_values_now,
                    SystemTime::now(),
                );
                if landed == Landed::Stored {
                    let (outputs, others, the_scene_it_read) = self
                        .learnt
                        .lock()
                        .ok()
                        .map(|mut learnt| {
                            (
                                learnt.the_outputs_half.take(),
                                std::mem::take(&mut learnt.the_other_scenes_outputs_halves),
                                learnt.the_scene_it_read.take(),
                            )
                        })
                        .unwrap_or_default();
                    crate::surface::render_settings::the_outputs_half_landed(
                        &mut held,
                        self.server,
                        self.file,
                        outputs,
                        others,
                    );
                    crate::surface::render_settings::the_read_ticks_its_scene(
                        &mut held,
                        self.server,
                        self.file,
                        the_scene_it_read.as_deref(),
                    );
                }
                landed
            }
            // A memory this run cannot take stores nothing, which is `Render
            // Settings` Q17's own act for a page that moved.
            Err(_) => Landed::DiscardedTheFileMoved,
        }
    }
}

/// **`Render Settings` Q5's one table of reads in flight, for this run.**
pub fn the_reads_in_flight() -> &'static ReadsInFlight {
    static IN_FLIGHT: OnceLock<ReadsInFlight> = OnceLock::new();
    IN_FLIGHT.get_or_init(ReadsInFlight::new)
}

/// **What pressing *Read the file* came to.**
#[derive(Debug, Clone, PartialEq)]
pub enum WhatTheReadCameTo {
    /// `Render Settings`' own outcome, with the operation the read went out
    /// under where one did.
    ThePreLoad {
        went: HowThePreLoadWent,
        operation: Option<Identity>,
    },
    /// **`Render Settings` Q17's first test has nothing to start under**: the four values could
    /// not be read, so no reading could carry the file identity `Render
    /// Settings` Q11 requires of every recorded reading, and no Blender was
    /// started for one.
    TheFourValuesCouldNotBeRead,
    /// **The server answered and ran no Blender** — the one case beside
    /// `Render Settings` Q8's four, which that crate's judgement puts on its
    /// explicit-absence arm (`Design/Render Settings § When the read gives
    /// nothing`, 2026-09-29).
    TheServerRanNoBlender,
}

/// **One press of *Read the file*, performed** — `Render Settings` Q4, `Render Settings` Q5,
/// `Render Settings` Q17 over one machine, landing in `held`.
///
/// 1. **`Render Settings` B5a first**: no version resolves, nothing is asked
///    of the machine.
/// 2. **`Render Settings` Q17's first material**: the four values the read
///    starts under.
/// 3. **`pre_load`**, whole: the join, the read over the seam above, the
///    judgement and the landing — `Render Settings` Q5, `Render Settings` Q7, `Render Settings` Q8, `Render Settings` Q6 and `Render Settings` Q17.
pub async fn the_file_is_read<M>(
    machine: M,
    held: &Mutex<WhatTheRunHolds>,
    server: &str,
    file: &str,
) -> WhatTheReadCameTo
where
    M: TheMachineAFileIsReadOn + Send,
{
    let key = FileKey {
        server: crate::surface::render_settings::the_servers_identity(server),
        path: file.to_string(),
    };
    let resolution = match held.lock() {
        Ok(held) => crate::surface::render_settings::the_version_resolved(&held, server, file),
        Err(_) => {
            return WhatTheReadCameTo::ThePreLoad {
                went: HowThePreLoadWent::TheReadIsNotOffered,
                operation: None,
            }
        }
    };
    // **`Render Settings` B5a: no version resolves, so nothing is asked of the
    // machine at all**
    // — not even the four values. `pre_load` holds the same test and answers
    // the same.
    if resolution.the_payloads_pick().is_none() {
        return WhatTheReadCameTo::ThePreLoad {
            went: HowThePreLoadWent::TheReadIsNotOffered,
            operation: None,
        };
    }
    let mut reads = TheFilesReadingOn::over(machine);
    let started_under = match reads.machine.the_four_values_of(file).await {
        FourValuesNow::Answered(values) => values,
        FourValuesNow::TheChannelCannotAnswer => {
            return WhatTheReadCameTo::TheFourValuesCouldNotBeRead
        }
    };
    the_file_is_read_with(
        &mut reads,
        held,
        server,
        file,
        key,
        &resolution,
        started_under,
    )
    .await
}

/// **The same, once the four values are in hand** — split out so a test can
/// hand in a driver whose bound and pace it chose.
pub async fn the_file_is_read_with<M>(
    reads: &mut TheFilesReadingOn<M>,
    held: &Mutex<WhatTheRunHolds>,
    server: &str,
    file: &str,
    key: FileKey,
    resolution: &smoozy_render_settings::version::Resolution,
    started_under: SourceReading,
) -> WhatTheReadCameTo
where
    M: TheMachineAFileIsReadOn + Send,
{
    let page = ThePageAtTheLanding {
        held,
        server,
        file,
        learnt: Arc::clone(&reads.learnt),
    };
    let the_file = (key.server.as_text(), key.path.clone());
    let went = pre_load(
        the_reads_in_flight(),
        reads,
        &page,
        &TheFilesHalfOfThisRun {
            held,
            server,
            file,
            learnt: Arc::clone(&reads.learnt),
        },
        key,
        resolution,
        started_under,
        SystemTime::now(),
    )
    .await;
    if went == HowThePreLoadWent::NoReading(Ending::NotAReadingAndNoneOfTheThreeEndingsFits)
        && the_reads_that_ran_no_blender()
            .lock()
            .is_ok_and(|ran_no_blender| ran_no_blender.contains(&the_file))
    {
        return WhatTheReadCameTo::TheServerRanNoBlender;
    }
    let operation = reads
        .learnt
        .lock()
        .ok()
        .and_then(|learnt| learnt.operation.clone());
    WhatTheReadCameTo::ThePreLoad { went, operation }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::surface::render_settings::tests::{
        a_run_with_a_reading, a_scene, a_server, a_version, four_values, the_words_of_a_line,
    };
    use crate::surface::render_settings::{the_panel, WhichSetting};
    use smoozy_job_files::manifest::CarriedAndNotInterpretedHere;
    use smoozy_job_files::reading::{THE_CLOSING_SENTINEL, THE_OPENING_SENTINEL};
    use smoozy_job_files::reading::{
        ArtistsText, AtOneFrame, Destination, DestinationKind, Format, GatingConditions,
        HowManyViewsTheSceneWrites, RecordedLocation, TheVersionInHand, WhereItComesFrom,
        WhichScene,
    };
    use smoozy_job_files::reading_script::{TheObservedOutputs, ThePrintedReading};
    use smoozy_render_settings::endings::Ending;
    use smoozy_render_settings::names::ExactName;
    use smoozy_render_settings::reading::OfferedEngineIdentifiers;
    use std::collections::VecDeque;
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// **A file of its own for each test.** `Render Settings` Q5's table of
    /// reads in flight is one for the run, as it is in the application, so two
    /// tests reading one (server, file) at once would join each other's read.
    fn a_file_of_its_own(test: &str) -> String {
        format!("/ephemeral/smoozy/assets/{test}.blend")
    }
    const THE_BUILD: &str = "/opt/blender-5.1.0/blender";

    /// **What a reading of this file prints**, in the machine's own shape:
    /// the scene's output path, `frames` frames, and the settings half given.
    fn a_printed_document(settings: &SettingsHalf, frames: usize) -> Vec<u8> {
        let layer = || ArtistsText::of(b"ViewLayer".to_vec());
        let gating = GatingConditions {
            compositing_switch: true,
            a_compositing_tree_is_present: false,
            a_live_final_output_node: false,
            the_final_output_node_is_muted: false,
            four_fives_use_nodes_switch: None,
            save_output_switch: None,
            the_render_goes_out_through_a_sequencer_holding_a_scene_strip: false,
        };
        let printed = ThePrintedReading {
            outputs: TheObservedOutputs {
                taken_by: TheVersionInHand::of(5, 1),
                gating: gating.clone(),
                destinations: vec![Destination {
                    kind: DestinationKind::TheRegularOutputPath,
                    recorded_location: Some(RecordedLocation::of(b"//out/f_".to_vec())),
                    format: Some(Format {
                        as_the_file_records_it: ArtistsText::of(b"PNG".to_vec()),
                        the_ending_it_writes: Some(ArtistsText::of(b".png".to_vec())),
                        is_video_shaped: false,
                    }),
                    origin: WhereItComesFrom::TheScenesOutputPath,
                    scene: WhichScene::TheJobsOwnScene,
                    the_view_layers_that_feed_it: vec![layer()],
                    the_location_with_its_folder_replaced: None,
                }],
                scenes_reached_through_a_sequencer_strip: Vec::new(),
                the_view_transform: None,
                the_tree_is_shared_with: None,
                per_frame: (1..=frames as i64)
                    .map(|frame| AtOneFrame {
                        frame,
                        view_layers_that_render: vec![layer()],
                        gating: gating.clone(),
                        the_regular_output_paths_answered_path: Some(RecordedLocation::of(
                            format!("/ephemeral/smoozy/assets/out/f_{frame:04}.png").into_bytes(),
                        )),
                    })
                    .collect(),
                recorded_locations: Vec::new(),
                views: HowManyViewsTheSceneWrites::One,
                the_compositing_tree: None,
            },
            settings: CarriedAndNotInterpretedHere(
                serde_json::to_value(settings).expect("it renders"),
            ),
            dependencies: CarriedAndNotInterpretedHere(serde_json::json!({})),
            errors: std::collections::BTreeMap::new(),
            the_baked_caches_on_the_machine: Vec::new(),
            the_scene_holds_a_simulation_not_baked: false,
            the_scene_it_read: None,
            the_open_files_path: None,
            the_check_is_written_as_a_picture: false,
        };
        serde_json::to_vec(&printed).expect("it renders")
    }

    fn framed(document: &[u8]) -> Vec<u8> {
        let mut stream = b"Blender 5.1.0\nRead blend: kitchen.blend\n".to_vec();
        stream.extend_from_slice(THE_OPENING_SENTINEL);
        stream.extend_from_slice(document);
        stream.extend_from_slice(THE_CLOSING_SENTINEL);
        stream.extend_from_slice(b"\nBlender quit\n");
        stream
    }

    fn a_settings_half() -> SettingsHalf {
        SettingsHalf {
            scenes: vec![a_scene("Scene")],
            offered_engine_identifiers: OfferedEngineIdentifiers {
                cycles: Some(ExactName::as_the_reading_produced_it("CYCLES")),
                eevee: Some(ExactName::as_the_reading_produced_it("BLENDER_EEVEE")),
            },
            announced_add_on_case: None,
        }
    }

    /// **A stand-in machine**: it answers each ask from a script, serves each
    /// stream from what it holds, and answers the four values in turn.
    struct AStandIn {
        answers: VecDeque<Fields>,
        streams: Vec<(Vec<u8>, Vec<u8>)>,
        four_values: VecDeque<FourValuesNow>,
        asked: Arc<Mutex<Vec<Request>>>,
        /// Where one is given, every ask waits until it is told to go on.
        gate: Option<Arc<tokio::sync::Notify>>,
        /// A program placed before this release, which knows no such request.
        refused_by_name: bool,
    }

    impl AStandIn {
        fn answering(answers: Vec<Fields>, streams: Vec<(Vec<u8>, Vec<u8>)>) -> AStandIn {
            AStandIn {
                answers: answers.into(),
                streams,
                four_values: vec![
                    FourValuesNow::Answered(four_values()),
                    FourValuesNow::Answered(four_values()),
                ]
                .into(),
                asked: Arc::default(),
                gate: None,
                refused_by_name: false,
            }
        }
    }

    impl TheMachineAReadingIsAskedOf for AStandIn {
        async fn ask(&mut self, request: &Request) -> WhatCameBack {
            if let Some(gate) = &self.gate {
                gate.notified().await;
            }
            self.asked.lock().expect("the list").push(request.clone());
            let empty = Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "0.1.7");
            if self.refused_by_name {
                return WhatCameBack::ItAnswered(
                    empty.saying_it_does_not_answer_that_request(&request.name),
                );
            }
            match self.answers.pop_front() {
                Some(fields) => WhatCameBack::ItAnswered(empty.carrying(fields)),
                None => WhatCameBack::NothingCorrelatedCameBack,
            }
        }

        async fn read_the_stream_back(
            &mut self,
            path: &[u8],
            length: u64,
        ) -> Result<Vec<u8>, String> {
            let (_, held) = self
                .streams
                .iter()
                .find(|(at, _)| at == path)
                .ok_or("no such stream")?;
            if (held.len() as u64) < length {
                return Err("the stream ended before its stated length".to_string());
            }
            Ok(held[..length as usize].to_vec())
        }
    }

    impl TheMachineAFileIsReadOn for AStandIn {
        async fn the_four_values_of(&mut self, _the_blend: &str) -> FourValuesNow {
            self.four_values
                .pop_front()
                .unwrap_or(FourValuesNow::TheChannelCannotAnswer)
        }
    }

    fn still_in_hand() -> Fields {
        Fields::none().and(asked::WHAT_BECAME_OF_IT_FIELD, asked::IT_IS_STILL_IN_HAND)
    }

    /// What the ordinary stream holds when Blender crashed after the reading
    /// script's first act: the started mark, then Blender's own crash line.
    fn crashed_after_its_first_act() -> Vec<u8> {
        let mut stream = smoozy_job_files::reading::THE_STARTED_MARK.to_vec();
        stream.extend_from_slice(b"Writing: /tmp/the-file.crash.txt\n");
        stream
    }

    fn ended(ordinary: &[u8], complaints: &[u8], status: Option<&str>) -> Fields {
        let mut fields = Fields::none()
            .and(
                asked::WHAT_BECAME_OF_IT_FIELD,
                asked::IT_HAS_A_RECORDED_OUTCOME,
            )
            .and(
                asked::THE_ORDINARY_STREAM_FIELD,
                b"/pkg/captured-streams/op/ordinary".to_vec(),
            )
            .and(
                asked::THE_ORDINARY_STREAM_LENGTH_FIELD,
                ordinary.len().to_string(),
            )
            .and(
                asked::THE_COMPLAINTS_STREAM_FIELD,
                b"/pkg/captured-streams/op/complaints".to_vec(),
            )
            .and(
                asked::THE_COMPLAINTS_STREAM_LENGTH_FIELD,
                complaints.len().to_string(),
            )
            .and(
                asked::THE_RUN_WAS_ENDED_BY_SMOOZY_FIELD,
                asked::THE_ANSWER_IS_NO,
            );
        if let Some(status) = status {
            fields = fields.and(asked::THE_EXIT_STATUS_FIELD, status);
        }
        fields
    }

    fn streams(ordinary: Vec<u8>, complaints: Vec<u8>) -> Vec<(Vec<u8>, Vec<u8>)> {
        vec![
            (b"/pkg/captured-streams/op/ordinary".to_vec(), ordinary),
            (b"/pkg/captured-streams/op/complaints".to_vec(), complaints),
        ]
    }

    /// **The run's memory with a pick for the file**: `a_run_with_a_reading`'s
    /// list, and 5.1.0 picked for `file`.
    fn a_run_with_a_pick(file: &str) -> Mutex<WhatTheRunHolds> {
        let mut held = a_run_with_a_reading();
        let _ = crate::surface::render_settings::the_answer_is_taken(
            &mut held,
            &a_server(),
            file,
            WhichSetting::VersionPick,
            THE_BUILD,
        );
        Mutex::new(held)
    }

    fn the_key(file: &str) -> FileKey {
        FileKey {
            server: crate::surface::render_settings::the_servers_identity(&a_server()),
            path: file.to_string(),
        }
    }

    fn block_on<F: Future>(future: F) -> F::Output {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("a runtime")
            .block_on(future)
    }

    fn quick<M>(machine: M) -> TheFilesReadingOn<M> {
        TheFilesReadingOn {
            bound: Duration::from_secs(5),
            pace: Duration::from_millis(1),
            ..TheFilesReadingOn::over(machine)
        }
    }

    fn the_read_of(
        held: &Mutex<WhatTheRunHolds>,
        file: &str,
        machine: AStandIn,
    ) -> WhatTheReadCameTo {
        block_on(async {
            let resolution = crate::surface::render_settings::the_version_resolved(
                &held.lock().expect("held"),
                &a_server(),
                file,
            );
            let mut reads = quick(machine);
            let started_under = match reads.machine.the_four_values_of(file).await {
                FourValuesNow::Answered(values) => values,
                FourValuesNow::TheChannelCannotAnswer => {
                    return WhatTheReadCameTo::TheFourValuesCouldNotBeRead
                }
            };
            the_file_is_read_with(
                &mut reads,
                held,
                &a_server(),
                file,
                the_key(file),
                &resolution,
                started_under,
            )
            .await
        })
    }

    /// **Read the file, as a person meets it: the press asks the machine, and
    /// the page then holds the file's values.** Until this piece the press
    /// answered *"Smoozy cannot do this yet. Sending instructions to the server
    /// is not built."* and nothing was read.
    ///
    /// **The runner's ruling of 2026-09-28, with a document over one MiB**: the per-frame table is long
    /// enough that the document could never ride inside an answer, and it
    /// comes back whole through the stream.
    #[test]
    fn the_file_is_read_through_the_stream_and_the_page_holds_its_values() {
        let file =
            &a_file_of_its_own("the_file_is_read_through_the_stream_and_the_page_holds_its_values");
        let document = a_printed_document(&a_settings_half(), 6000);
        assert!(document.len() > smoozy_server_protocol::framing::THE_LARGEST_FRAME);
        let ordinary = framed(&document);
        let machine = AStandIn::answering(
            vec![
                still_in_hand(),
                still_in_hand(),
                ended(&ordinary, b"", Some("0")),
            ],
            streams(ordinary.clone(), Vec::new()),
        );
        let asked_list = Arc::clone(&machine.asked);
        let held = a_run_with_a_pick(file);
        let came_to = the_read_of(&held, file, machine);
        let WhatTheReadCameTo::ThePreLoad { went, operation } = came_to else {
            panic!("{came_to:?}");
        };
        assert!(went.the_files_half_was_replaced(), "{went:?}");
        let HowThePreLoadWent::AReading { the_diary_act, .. } = &went else {
            panic!("{went:?}");
        };
        assert_eq!(
            the_diary_act.as_ref().map(|act| act.taken_by.path.as_str()),
            Some(THE_BUILD)
        );

        // **One request, asked under one identity until it ended.**
        let asked_list = asked_list.lock().expect("the list");
        assert_eq!(asked_list.len(), 3);
        for request in asked_list.iter() {
            assert_eq!(request.name, asked::THE_READING_OF_A_BLEND_REQUEST);
            assert_eq!(Some(&request.identity), operation.as_ref());
            assert_eq!(
                request.fields.text(asked::THE_WHICH_READING_FIELD),
                Some(asked::THE_FILES_READING)
            );
            assert_eq!(
                request.fields.value(asked::THE_BUILD_FIELD),
                Some(THE_BUILD.as_bytes())
            );
            assert_eq!(
                request.fields.value(asked::THE_BLEND_FIELD),
                Some(file.as_bytes())
            );
            assert_eq!(
                request.fields.text(asked::THE_BLENDER_VERSION_FIELD),
                Some("5.1.0")
            );
        }

        // **The page holds the file's values**: the scene's range and count.
        let held = held.lock().expect("held");
        let panel = the_panel(&held, &a_server(), file);
        assert!(panel.a_reading_stands);
        assert_eq!(
            panel.the_range.value.as_ref().map(|value| value.as_str()),
            Some("1-240")
        );
        assert_eq!(
            panel.samples.value.as_ref().map(|value| value.as_str()),
            Some("2048")
        );
        assert!(
            !panel.read_at.is_empty(),
            "the header does not say when the file was read"
        );
    }

    /// **`Render Settings` Q8's endings, each from the machine's own answer, and never one that
    /// claims something about the file that nothing found.**
    #[test]
    fn each_way_a_read_gives_nothing_is_told_apart_and_nothing_is_stored() {
        let ordinary = framed(&a_printed_document(&a_settings_half(), 3));
        let mut short = ordinary.clone();
        short.truncate(ordinary.len() / 2);
        // **`None` is the one case beside the four**: the machine answered and
        // ran no Blender, which the landing names on its own.
        let cases: Vec<(&str, Vec<Fields>, Vec<u8>, Option<Ending>)> = vec![
            (
                "nothing came back",
                Vec::new(),
                Vec::new(),
                Some(Ending::TheMachineCouldNotBeReached),
            ),
            (
                "a program that does not know the request",
                vec![still_in_hand()],
                b"refused by name".to_vec(),
                None,
            ),
            (
                "not admitted, in the machine's own words",
                vec![Fields::none().and(
                    asked::THE_MACHINES_OWN_WORDS_FIELD,
                    "the file-inspection worker was not admitted: a setup holds the machine",
                )],
                Vec::new(),
                None,
            ),
            (
                "a build that could not be run names no stream",
                vec![Fields::none()
                    .and(
                        asked::WHAT_BECAME_OF_IT_FIELD,
                        asked::IT_HAS_A_RECORDED_OUTCOME,
                    )
                    .and(
                        asked::THE_RUN_WAS_ENDED_BY_SMOOZY_FIELD,
                        asked::THE_ANSWER_IS_NO,
                    )],
                Vec::new(),
                Some(Ending::TheFileCouldNotBeOpenedByThisVersion),
            ),
            (
                "a Blender that ended at a failing status before the script",
                vec![ended(b"Error: cannot read file\n", b"", Some("1"))],
                b"Error: cannot read file\n".to_vec(),
                Some(Ending::TheFileCouldNotBeOpenedByThisVersion),
            ),
            // **The Local View crash, 2026-10-02**: the script wrote its started
            // mark, then Blender died by a signal partway through the walk —
            // no exit status, no document. **The read ran**, so it is never
            // *this Blender could not open this file*.
            (
                "a Blender that crashed after the script's first act",
                vec![ended(&crashed_after_its_first_act(), b"", None)],
                crashed_after_its_first_act(),
                Some(Ending::TheReadRanAndSomethingInItFailed),
            ),
            (
                "a document cut short",
                vec![ended(&short, b"", Some("1"))],
                short.clone(),
                Some(Ending::TheReadRanAndSomethingInItFailed),
            ),
            (
                "a read Smoozy ended",
                vec![Fields::none()
                    .and(
                        asked::WHAT_BECAME_OF_IT_FIELD,
                        asked::IT_HAS_A_RECORDED_OUTCOME,
                    )
                    .and(
                        asked::THE_RUN_WAS_ENDED_BY_SMOOZY_FIELD,
                        asked::THE_ANSWER_IS_YES,
                    )],
                Vec::new(),
                Some(Ending::SmoozyStoppedIt),
            ),
        ];
        for (at, (what, answers, ordinary, ending)) in cases.into_iter().enumerate() {
            let file = &a_file_of_its_own(&format!("each-way-{at}"));
            let held = a_run_with_a_pick(file);
            let mut machine = AStandIn::answering(answers, streams(ordinary.clone(), Vec::new()));
            machine.refused_by_name = ordinary == b"refused by name";
            let came_to = the_read_of(&held, file, machine);
            let expected = match ending {
                Some(ending) => WhatTheReadCameTo::ThePreLoad {
                    went: HowThePreLoadWent::NoReading(ending),
                    operation: match &came_to {
                        WhatTheReadCameTo::ThePreLoad { operation, .. } => operation.clone(),
                        _ => None,
                    },
                },
                None => WhatTheReadCameTo::TheServerRanNoBlender,
            };
            assert_eq!(came_to, expected, "{what}");
            assert!(
                held.lock()
                    .expect("held")
                    .readings
                    .standing(&the_key(file))
                    .is_none(),
                "{what}"
            );
        }
    }

    /// **A read's ending, landed on the page the way the press lands it**, and
    /// the page drawn after it.
    fn landed_and_drawn(
        held: &Mutex<WhatTheRunHolds>,
        file: &str,
        came_to: WhatTheReadCameTo,
    ) -> crate::surface::render_settings::TheRenderSettings {
        let mut held = held.lock().expect("held");
        let _ = crate::surface::render_settings::the_read_lands_in(
            &mut held,
            None,
            &a_server(),
            file,
            came_to,
        );
        the_panel(&held, &a_server(), file)
    }

    /// **Where the machine answers and runs no Blender, the page says it
    /// could not read the file on the server** — step 10c's four small things
    /// after *Read the file*, item 3, `Preparation/Design/Render Settings.md`
    /// § *When the read gives nothing*. **Until then it said *"The read ran and
    /// did not finish."***, which is false there: no read ran.
    ///
    /// Both of the machine's ways: a program that does not know the request,
    /// and a reading it did not admit. **A Blender that did run and printed no
    /// start mark still says reading the file did not finish**, because there
    /// it did.
    #[test]
    fn a_machine_that_runs_no_blender_says_the_file_could_not_be_read_on_the_server() {
        let not_admitted = || {
            vec![Fields::none().and(
                asked::THE_MACHINES_OWN_WORDS_FIELD,
                "the file-inspection worker was not admitted: a setup holds the machine",
            )]
        };
        for (at, refused_by_name) in [true, false].into_iter().enumerate() {
            let file = &a_file_of_its_own(&format!("no-blender-ran-{at}"));
            let mut machine = AStandIn::answering(
                if refused_by_name { vec![still_in_hand()] } else { not_admitted() },
                Vec::new(),
            );
            machine.refused_by_name = refused_by_name;
            let held = a_run_with_a_pick(file);
            let came_to = the_read_of(&held, file, machine);
            let panel = landed_and_drawn(&held, file, came_to);
            let failure = panel.the_read_failure.expect("the ending is drawn");
            assert_eq!(
                failure.what_did_not_happen.as_str().replace('\u{a0}', " "),
                "Could not read the file on the server.",
                "refused by name: {refused_by_name}"
            );
            assert!(
                failure.the_raw_text.is_none(),
                "no Blender ran, so there is no Blender output to open"
            );
            assert!(failure.what_to_do_about_it.is_none());
        }

        // **A Blender that ran, ended at nought and printed no start mark** —
        // on the same file as the last read above, so what that read left is
        // cleared by this one rather than said again.
        let file = &a_file_of_its_own("no-blender-ran-1");
        let ordinary = b"Blender 5.1.0\nBlender quit\n".to_vec();
        let machine = AStandIn::answering(
            vec![ended(&ordinary, b"", Some("0"))],
            streams(ordinary.clone(), Vec::new()),
        );
        let held = a_run_with_a_pick(file);
        let came_to = the_read_of(&held, file, machine);
        let panel = landed_and_drawn(&held, file, came_to);
        assert_eq!(
            panel
                .the_read_failure
                .expect("the ending is drawn")
                .what_did_not_happen
                .as_str()
                .replace('\u{a0}', " "),
            "Reading the file did not finish."
        );
    }

    /// **`§ What it will write` is drawn from the read only where the file has
    /// one scene** — step 10c's four small things after *Read the file*, item
    /// 2, the runner's ruling of 2026-09-29. The reading asks for no scene, so
    /// its outputs half is the scene the file was saved showing, and nothing in
    /// it says which scene that is: **on a file with two, ticking the other
    /// would show the first one's outputs.** So there it stays as it is before
    /// a read.
    #[test]
    fn what_it_will_write_is_drawn_from_the_read_only_where_the_file_has_one_scene() {
        for (scenes, drawn) in [
            (vec![a_scene("Scene")], true),
            (vec![a_scene("Exterior"), a_scene("Interior")], false),
        ] {
            let count = scenes.len();
            let file = &a_file_of_its_own(&format!("what-it-will-write-{count}"));
            let settings = SettingsHalf {
                scenes,
                ..a_settings_half()
            };
            let ordinary = framed(&a_printed_document(&settings, 3));
            let machine = AStandIn::answering(
                vec![ended(&ordinary, b"", Some("0"))],
                streams(ordinary.clone(), Vec::new()),
            );
            let held = a_run_with_a_pick(file);
            let came_to = the_read_of(&held, file, machine);
            assert!(
                matches!(&came_to, WhatTheReadCameTo::ThePreLoad { went, .. } if went.the_files_half_was_replaced()),
                "{came_to:?}"
            );
            let held = held.lock().expect("held");
            let panel = the_panel(&held, &a_server(), file);
            assert_eq!(
                panel.what_it_will_write.is_some(),
                drawn,
                "a file of {count} scenes: {:?}",
                panel.what_it_will_write
            );
            if drawn {
                let writes = panel.what_it_will_write.expect("drawn");
                assert_eq!(writes.rows.len(), 1, "{writes:?}");
            }
            // **It goes with the reading it came with**: `Render Settings`
            // J10's end of the connected period drops the reading, and the
            // group goes back to how it is before a read.
            let mut held = held;
            crate::surface::render_settings::the_period_ended_for(&mut held, &a_server());
            let panel = the_panel(&held, &a_server(), file);
            assert!(panel.what_it_will_write.is_none(), "a file of {count} scenes");
        }
    }

    /// **The 38's piece 10, item 25: a file of three scenes saved showing
    /// *Interior*.** The reading names the scene it read; ticking *Interior*
    /// draws *What it will write* from it, ticking *Exterior* does not
    /// (`Design/Render Settings § What it will write`, corrected 2026-10-03).
    #[test]
    fn a_file_of_several_scenes_draws_what_it_will_write_where_the_scene_the_read_names_is_ticked() {
        use smoozy_render_settings::file_half::FileKey;
        use smoozy_render_settings::names::{NamedAnswer, Provenance};
        let file = &a_file_of_its_own("what-it-will-write-named-scene");
        let settings = SettingsHalf {
            scenes: vec![a_scene("Exterior"), a_scene("Interior"), a_scene("Kitchen")],
            ..a_settings_half()
        };
        let mut document: serde_json::Value =
            serde_json::from_slice(&a_printed_document(&settings, 3)).expect("a document");
        document
            .as_object_mut()
            .expect("an object")
            .insert("the_scene_it_read".to_string(), serde_json::json!(b"Interior".to_vec()));
        let ordinary = framed(&serde_json::to_vec(&document).expect("it renders"));
        let machine = AStandIn::answering(
            vec![ended(&ordinary, b"", Some("0"))],
            streams(ordinary.clone(), Vec::new()),
        );
        let held = a_run_with_a_pick(file);
        let came_to = the_read_of(&held, file, machine);
        assert!(
            matches!(&came_to, WhatTheReadCameTo::ThePreLoad { went, .. } if went.the_files_half_was_replaced()),
            "{came_to:?}"
        );
        let mut held = held.lock().expect("held");
        let key = FileKey {
            server: crate::surface::render_settings::the_servers_identity(&a_server()),
            path: file.to_string(),
        };
        let tick = |held: &mut WhatTheRunHolds, scene: &str, on: bool| {
            let name = ExactName::as_the_reading_produced_it(scene);
            if on {
                let provenance =
                    Provenance::ARecordedReading(held.readings.standing(&key).expect("a reading").identity());
                held.store.tick_a_scene(key.clone(), NamedAnswer { name, provenance });
            } else {
                held.store.untick_a_scene(key.clone(), &name);
            }
        };
        tick(&mut held, "Interior", true);
        let writes = the_panel(&held, &a_server(), file).what_it_will_write;
        assert_eq!(writes.map(|writes| writes.rows.len()), Some(1), "Interior is the scene the read named");
        tick(&mut held, "Interior", false);
        tick(&mut held, "Exterior", true);
        assert!(
            the_panel(&held, &a_server(), file).what_it_will_write.is_none(),
            "another scene's outputs are never drawn under the one ticked"
        );
    }

    /// **The Creator's seventh notes, note 1 (`Job files and outputs` R4b): a
    /// reading that holds every scene's half.** The file of three scenes is
    /// read through the run's own seam, naming *Interior*, with *Exterior*'s
    /// half beside it writing JPEG and *Kitchen*'s holding an error. Ticking
    /// *Exterior* draws the group and the download box from *Exterior*'s own
    /// half — its JPEG, not *Interior*'s PNG — where before that clause both stood as
    /// before a read. *Kitchen*, whose half holds an error, has none, as a
    /// reading naming it would be none. With *Interior* and *Exterior* ticked
    /// the group stays as before a read and the box offers both formats.
    #[test]
    fn a_reading_of_every_scene_draws_the_ticked_scenes_own_outputs() {
        use crate::surface::render_settings::{
            a_format_in_smoozys_words, the_download_panel_from, WhatTheRecordHolds,
        };
        use crate::surface::transfers::{TheFormatAnswer, TheFormatReading};
        use smoozy_render_settings::file_half::FileKey;
        use smoozy_render_settings::names::{NamedAnswer, Provenance};
        let file = &a_file_of_its_own("what-it-will-write-every-scene");
        let settings = SettingsHalf {
            scenes: vec![a_scene("Exterior"), a_scene("Interior"), a_scene("Kitchen")],
            ..a_settings_half()
        };
        let mut document: serde_json::Value =
            serde_json::from_slice(&a_printed_document(&settings, 3)).expect("a document");
        let mut exterior = document["outputs"].clone();
        exterior["recorded_locations"] = serde_json::json!([]);
        let destination = &mut exterior["destinations"][0];
        destination["recorded_location"] = serde_json::json!(b"//ext/e_".to_vec());
        destination["format"]["as_the_file_records_it"] = serde_json::json!(b"JPEG".to_vec());
        destination["format"]["the_ending_it_writes"] = serde_json::json!(b".jpg".to_vec());
        let whole = document.as_object_mut().expect("an object");
        whole.insert("the_scene_it_read".to_string(), serde_json::json!(b"Interior".to_vec()));
        whole.insert(
            "the_other_scenes".to_string(),
            serde_json::json!([
                {
                    "scene": b"Exterior".to_vec(),
                    "outputs": exterior.clone(),
                    "the_scene_holds_a_simulation_not_baked": false,
                    "errors": {},
                },
                {
                    "scene": b"Kitchen".to_vec(),
                    "outputs": exterior,
                    "the_scene_holds_a_simulation_not_baked": false,
                    "errors": {"the regular output path at frame 1": "broken"},
                },
            ]),
        );
        let ordinary = framed(&serde_json::to_vec(&document).expect("it renders"));
        let machine = AStandIn::answering(
            vec![ended(&ordinary, b"", Some("0"))],
            streams(ordinary.clone(), Vec::new()),
        );
        let held = a_run_with_a_pick(file);
        let came_to = the_read_of(&held, file, machine);
        assert!(
            matches!(&came_to, WhatTheReadCameTo::ThePreLoad { went, .. } if went.the_files_half_was_replaced()),
            "{came_to:?}"
        );
        let mut held = held.lock().expect("held");
        let key = FileKey {
            server: crate::surface::render_settings::the_servers_identity(&a_server()),
            path: file.to_string(),
        };
        let tick = |held: &mut WhatTheRunHolds, scene: &str, on: bool| {
            let name = ExactName::as_the_reading_produced_it(scene);
            if on {
                let provenance =
                    Provenance::ARecordedReading(held.readings.standing(&key).expect("a reading").identity());
                held.store.tick_a_scene(key.clone(), NamedAnswer { name, provenance });
            } else {
                held.store.untick_a_scene(key.clone(), &name);
            }
        };
        let record = WhatTheRecordHolds::default();
        let the_box = |held: &WhatTheRunHolds| {
            let TheFormatAnswer::TheBox(the_box) = the_download_panel_from(held, &a_server(), file, &record) else {
                panic!("the box");
            };
            the_box.reading
        };
        let drawn = |reading: &TheFormatReading| serde_json::to_string(reading).expect("drawn");
        let png = a_format_in_smoozys_words(b"PNG");
        let jpeg = a_format_in_smoozys_words(b"JPEG");

        tick(&mut held, "Interior", true);
        let interior = the_panel(&held, &a_server(), file).what_it_will_write.expect("Interior's group");
        let reading = the_box(&held);
        assert!(drawn(&reading).contains(&png) && !drawn(&reading).contains(&jpeg), "{reading:?}");

        tick(&mut held, "Interior", false);
        tick(&mut held, "Exterior", true);
        let exterior = the_panel(&held, &a_server(), file)
            .what_it_will_write
            .expect("Exterior's own half fills the group");
        assert_ne!(
            serde_json::to_string(&exterior).expect("drawn"),
            serde_json::to_string(&interior).expect("drawn"),
            "Exterior's own outputs, not Interior's"
        );
        let reading = the_box(&held);
        assert!(drawn(&reading).contains(&jpeg) && !drawn(&reading).contains(&png), "{reading:?}");
        assert!(!matches!(reading, TheFormatReading::ReadForAnotherScene { .. } | TheFormatReading::NotRead { .. }));

        // **Several ticked: the group as before a read; the box offers both.**
        tick(&mut held, "Interior", true);
        assert!(the_panel(&held, &a_server(), file).what_it_will_write.is_none(), "several ticked");
        let reading = the_box(&held);
        assert!(drawn(&reading).contains(&jpeg) && drawn(&reading).contains(&png), "{reading:?}");

        // **A scene whose half holds an error has none**, as a reading naming
        // it would be no reading.
        tick(&mut held, "Interior", false);
        tick(&mut held, "Exterior", false);
        tick(&mut held, "Kitchen", true);
        assert!(the_panel(&held, &a_server(), file).what_it_will_write.is_none(), "Kitchen's half holds an error");
    }

    // ── The Creator's note 5 on the test copy: the read ticks the saved scene ──

    /// **A reading of a file of these scenes, naming the one it read** (or
    /// none, which is a script older than 2026-10-03), as the machine prints
    /// it, framed in the ordinary stream.
    fn a_reading_naming(scenes: &[&str], the_scene_it_read: Option<&str>) -> Vec<u8> {
        let settings = SettingsHalf {
            scenes: scenes.iter().map(|name| a_scene(name)).collect(),
            ..a_settings_half()
        };
        let mut document: serde_json::Value =
            serde_json::from_slice(&a_printed_document(&settings, 3)).expect("a document");
        if let Some(named) = the_scene_it_read {
            document
                .as_object_mut()
                .expect("an object")
                .insert("the_scene_it_read".to_string(), serde_json::json!(named.as_bytes().to_vec()));
        }
        framed(&serde_json::to_vec(&document).expect("it renders"))
    }

    /// **One press of *Read the file*, answered by a machine holding that
    /// reading.** The reading must be stored, or the test says so.
    fn the_file_is_read_holding(held: &Mutex<WhatTheRunHolds>, file: &str, ordinary: Vec<u8>) {
        let machine = AStandIn::answering(
            vec![ended(&ordinary, b"", Some("0"))],
            streams(ordinary.clone(), Vec::new()),
        );
        let came_to = the_read_of(held, file, machine);
        assert!(
            matches!(&came_to, WhatTheReadCameTo::ThePreLoad { went, .. } if went.the_files_half_was_replaced()),
            "{came_to:?}"
        );
    }

    /// The scenes ticked for `file`, by name, in the order they were ticked.
    fn the_ticks(held: &WhatTheRunHolds, file: &str) -> Vec<String> {
        held.store
            .per_file(&the_key(file))
            .map(|per_file| {
                per_file
                    .scenes_ticked
                    .iter()
                    .map(|ticked| String::from_utf8_lossy(ticked.name.as_bytes()).into_owned())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// **The scene the composed job carries**, by name, or nothing.
    fn the_scene_the_job_carries(held: &WhatTheRunHolds, file: &str) -> Vec<Option<String>> {
        let row = crate::surface::render_settings::tests::a_row_on(&a_server(), true);
        let given = crate::surface::render_settings::the_act_as_the_page_composed_it(
            held,
            &a_server(),
            file,
            &crate::surface::render_settings::WhatTheRecordHolds::default(),
            &row,
            crate::surface::render_settings::tests::a_run(),
        )
        .unwrap_or_else(|said| panic!("{said:?}"));
        given
            .scene_jobs
            .iter()
            .map(|job| {
                job.scene
                    .as_ref()
                    .map(|scene| String::from_utf8_lossy(scene.name.as_bytes()).into_owned())
            })
            .collect()
    }

    /// **The Creator's note 5 on the test copy, the gap it found**: a file of
    /// three scenes saved showing *Interior*, which is not the first. After the
    /// read, *Interior* is ticked — `Render Settings` J7's third naming act
    /// (2026-10-03), `Design/Render Settings § Scenes` — and the job carries
    /// its name by `Render Settings` F5's ticked branch, so the page and the render name the
    /// same scene. **Before this the read ticked nothing, the page stood in
    /// *Exterior* (the reading's first) and the job carried no name**, which
    /// renders the saved scene: one scene described, another rendered.
    #[test]
    fn a_read_ticks_the_scene_the_file_was_saved_showing_and_the_job_carries_it() {
        let file = &a_file_of_its_own("note-5-the-saved-scene-is-ticked");
        let held = a_run_with_a_pick(file);
        the_file_is_read_holding(
            &held,
            file,
            a_reading_naming(&["Exterior", "Interior", "Kitchen"], Some("Interior")),
        );
        let held = held.lock().expect("held");
        assert_eq!(the_ticks(&held, file), vec!["Interior".to_string()], "the scene ticked after the read");
        assert_eq!(
            the_scene_the_job_carries(&held, file),
            vec![Some("Interior".to_string())],
            "the scene the job carries"
        );
        // **`Render Settings` J7's third act carries that reading's provenance**, exactly as a
        // user's tick drawn from that reading would.
        let reading = held.readings.standing(&the_key(file)).expect("a reading").identity();
        let ticked = &held.store.per_file(&the_key(file)).expect("ticked").scenes_ticked[0];
        assert_eq!(
            ticked.provenance,
            smoozy_render_settings::names::Provenance::ARecordedReading(reading)
        );
    }

    /// **A read that names no scene** — a script older than 2026-10-03 —
    /// **ticks the reading's first** (`Render Settings` J7's third act, its last limb).
    #[test]
    fn a_read_naming_no_scene_ticks_the_readings_first() {
        let file = &a_file_of_its_own("note-5-no-scene-named");
        let held = a_run_with_a_pick(file);
        the_file_is_read_holding(&held, file, a_reading_naming(&["Exterior", "Interior"], None));
        let held = held.lock().expect("held");
        assert_eq!(the_ticks(&held, file), vec!["Exterior".to_string()]);
        assert_eq!(the_scene_the_job_carries(&held, file), vec![Some("Exterior".to_string())]);
    }

    /// **Reading the file again keeps the user's ticks that still name a
    /// scene in the file, and ticks the saved one where none is left** —
    /// `Design/Render Settings § Scenes`, `Render Settings` J7's paragraph.
    #[test]
    fn a_read_again_keeps_the_users_ticks_and_ticks_the_saved_one_where_none_is_left() {
        use smoozy_render_settings::names::{NamedAnswer, Provenance};
        let file = &a_file_of_its_own("note-5-a-read-again");
        let held = a_run_with_a_pick(file);
        let three = || a_reading_naming(&["Exterior", "Interior", "Kitchen"], Some("Interior"));
        the_file_is_read_holding(&held, file, three());
        {
            let mut held = held.lock().expect("held");
            let provenance =
                Provenance::ARecordedReading(held.readings.standing(&the_key(file)).expect("a reading").identity());
            held.store
                .untick_a_scene(the_key(file), &ExactName::as_the_reading_produced_it("Interior"));
            for scene in ["Exterior", "Kitchen"] {
                held.store.tick_a_scene(
                    the_key(file),
                    NamedAnswer {
                        name: ExactName::as_the_reading_produced_it(scene),
                        provenance: provenance.clone(),
                    },
                );
            }
        }
        the_file_is_read_holding(&held, file, three());
        assert_eq!(
            the_ticks(&held.lock().expect("held"), file),
            vec!["Exterior".to_string(), "Kitchen".to_string()],
            "the user's two ticks stand and nothing is added"
        );
        // **The file changed, and neither tick names a scene in it**: the
        // read's tick takes their place.
        the_file_is_read_holding(&held, file, a_reading_naming(&["Day", "Night"], Some("Night")));
        assert_eq!(the_ticks(&held.lock().expect("held"), file), vec!["Night".to_string()]);
    }

    /// **A page opened from a history entry keeps the scene it arrived with**,
    /// even where the file no longer holds it — `Render Settings` J7's paragraph: *"never
    /// replaced by the read"*; a name the file does not carry fails at the
    /// attempt by `Render Settings` J9.
    #[test]
    fn a_page_opened_from_a_history_entry_keeps_its_scene_through_a_read() {
        use smoozy_render_settings::names::{NamedAnswer, Provenance};
        use smoozy_render_settings::store::{KeptSetup, KeptVersion, SceneEntries};
        let file = &a_file_of_its_own("note-5-a-history-page");
        let held = a_run_with_a_pick(file);
        for kept in ["Kitchen", "Attic"] {
            held.lock().expect("held").store.load_a_kept_setup(
                the_key(file),
                KeptSetup {
                    scene: NamedAnswer {
                        name: ExactName::as_the_reading_produced_it(kept),
                        provenance: Provenance::AKeptSetup {
                            the_old_job: "an old job".to_string(),
                        },
                    },
                    entries: SceneEntries::default(),
                    // The old job's own pick, so the read is offered.
                    version: KeptVersion::TheUsersOverride(BlenderIdentity {
                        path: THE_BUILD.to_string(),
                        version: a_version(5, 1, 0),
                    }),
                },
                false,
            );
            the_file_is_read_holding(
                &held,
                file,
                a_reading_naming(&["Exterior", "Interior", "Kitchen"], Some("Interior")),
            );
            assert_eq!(the_ticks(&held.lock().expect("held"), file), vec![kept.to_string()]);
        }
    }

    /// **`Render Settings` J3's last sentence: an answer given before the
    /// first read goes on standing in the box it was typed into** — the
    /// pre-read set becomes the entries of the scene the page then stands in,
    /// which after `Render Settings` J7's third act is the ticked saved scene. `Store`'s
    /// `a_reading_first_landed` had been built and only its own test called
    /// it, so a range typed before a read went from the page at the read and
    /// never reached the job.
    #[test]
    fn a_range_typed_before_the_read_stands_in_the_ticked_scene_after_it() {
        let file = &a_file_of_its_own("note-5-a-range-typed-first");
        let held = a_run_with_a_pick(file);
        let refused = crate::surface::render_settings::the_answer_is_taken(
            &mut held.lock().expect("held"),
            &a_server(),
            file,
            WhichSetting::FrameRange,
            "10-20",
        );
        assert_eq!(refused, None);
        the_file_is_read_holding(
            &held,
            file,
            a_reading_naming(&["Exterior", "Interior"], Some("Interior")),
        );
        let held = held.lock().expect("held");
        let panel = the_panel(&held, &a_server(), file);
        assert_eq!(panel.the_range.value.as_ref().map(|value| value.as_str()), Some("10-20"));
        assert_eq!(panel.the_range.whose.as_str(), "the-users");
        let row = crate::surface::render_settings::tests::a_row_on(&a_server(), true);
        let given = crate::surface::render_settings::the_act_as_the_page_composed_it(
            &held,
            &a_server(),
            file,
            &crate::surface::render_settings::WhatTheRecordHolds::default(),
            &row,
            crate::surface::render_settings::tests::a_run(),
        )
        .unwrap_or_else(|said| panic!("{said:?}"));
        assert!(given.scene_jobs[0].frame_range.is_some(), "the typed range reaches the job");
    }

    /// **The same, on real readings taken by Blender 5.2.2**: the 5.2 splash
    /// scene, one scene, draws *What it will write* from the read; a file of
    /// two scenes (*Exterior*, the one it was saved showing, writing PNG, and
    /// *Interior* writing EXR) draws nothing from it.
    ///
    /// **Ignored, because the readings are not in the tree.** Run by hand with
    /// `SMOOZY_DESK_SPLASH_RESULTS` naming the splash tester's `results/`
    /// folder and `SMOOZY_DESK_TWO_SCENES` naming a reading of the two-scene
    /// file, each a JSON object whose `printed` is the document between the
    /// marks.
    #[test]
    #[ignore = "reads real readings from outside the tree"]
    fn what_it_will_write_on_a_real_splash_scene_and_a_real_file_of_two_scenes() {
        let splash = std::path::PathBuf::from(
            std::env::var("SMOOZY_DESK_SPLASH_RESULTS").expect("SMOOZY_DESK_SPLASH_RESULTS"),
        )
        .join("reading-5.2-as.json");
        let two = std::path::PathBuf::from(
            std::env::var("SMOOZY_DESK_TWO_SCENES").expect("SMOOZY_DESK_TWO_SCENES"),
        );
        for (at, (path, drawn)) in [(splash, true), (two, false)].into_iter().enumerate() {
            let file = &a_file_of_its_own(&format!("real-what-it-will-write-{at}"));
            let whole: serde_json::Value =
                serde_json::from_slice(&std::fs::read(&path).expect("the reading"))
                    .expect("it reads");
            let document = serde_json::to_vec(&whole["printed"]).expect("it renders");
            let ordinary = framed(&document);
            let machine = AStandIn::answering(
                vec![ended(&ordinary, b"", Some("0"))],
                streams(ordinary.clone(), Vec::new()),
            );
            let mut held = a_run_with_a_reading();
            held.version_lists
                .get_mut(&a_server())
                .expect("the list")
                .entries
                .push(smoozy_server_setup::search::ACandidate {
                    path: "/opt/blender-5.2.2/blender".to_string(),
                    answered: Some(a_version(5, 2, 2)),
                });
            let _ = crate::surface::render_settings::the_answer_is_taken(
                &mut held,
                &a_server(),
                file,
                WhichSetting::VersionPick,
                "/opt/blender-5.2.2/blender",
            );
            let held = Mutex::new(held);
            let came_to = the_read_of(&held, file, machine);
            assert!(
                matches!(&came_to, WhatTheReadCameTo::ThePreLoad { went, .. } if went.the_files_half_was_replaced()),
                "{}: {came_to:?}",
                path.display()
            );
            let held = held.lock().expect("held");
            let panel = the_panel(&held, &a_server(), file);
            println!(
                "{}: {} scene(s); what it will write {:?}",
                path.display(),
                held.readings
                    .standing(&the_key(file))
                    .map(|reading| reading.settings.scenes.len())
                    .unwrap_or_default(),
                panel.what_it_will_write.as_ref().map(|writes| {
                    writes
                        .rows
                        .iter()
                        .map(|row| {
                            format!(
                                "{} × {} {:?}",
                                row.place.as_str(),
                                row.how_many.as_str(),
                                row.format.as_ref().map(|format| format.as_str().to_string())
                            )
                        })
                        .collect::<Vec<_>>()
                })
            );
            assert_eq!(panel.what_it_will_write.is_some(), drawn, "{}", path.display());
        }
    }

    /// **The bound over the whole reading**: a machine that says *still in
    /// hand* for ever is given up on, and the wait's ending is the channel's.
    #[test]
    fn a_reading_still_in_hand_for_ever_is_given_up_at_the_bound() {
        let file = &a_file_of_its_own("a_reading_still_in_hand_for_ever_is_given_up_at_the_bound");
        let machine = AStandIn::answering(vec![still_in_hand(); 10_000], Vec::new());
        let started = std::time::Instant::now();
        let left = block_on(what_the_files_reading_left(
            &mut { machine },
            &the_files_reading_asked(
                &Identity::issue(IdentityKind::Operation).expect("an identity"),
                &ABlendToRead {
                    server: &the_key(file).server,
                    the_blends_path: file,
                    with: &BlenderIdentity {
                        path: THE_BUILD.to_string(),
                        version: a_version(5, 1, 0),
                    },
                },
            ),
            Duration::from_millis(50),
            Duration::from_millis(5),
        ));
        assert_eq!(left.frame.channel, ChannelOutcome::TheMachineCouldNotBeReached);
        // **Given up at the bound, not when the machine stopped answering.**
        assert!(started.elapsed() < Duration::from_secs(5), "{:?}", started.elapsed());
    }

    /// **`Render Settings` Q17, from the file's side: a file replaced while the read ran reads
    /// back different, and the reading is discarded rather than stored.** And
    /// a machine that cannot say what stands now stores nothing either.
    #[test]
    fn a_reading_of_a_file_that_moved_under_it_is_discarded() {
        let file = &a_file_of_its_own("a_reading_of_a_file_that_moved_under_it_is_discarded");
        let ordinary = framed(&a_printed_document(&a_settings_half(), 3));
        for now in [
            FourValuesNow::Answered(smoozy_server_protocol::transfers::SourceReading {
                size: 8192,
                ..four_values()
            }),
            FourValuesNow::TheChannelCannotAnswer,
        ] {
            let file = &format!(
                "{file}-{}",
                matches!(now, FourValuesNow::TheChannelCannotAnswer)
            );
            let mut machine = AStandIn::answering(
                vec![ended(&ordinary, b"", Some("0"))],
                streams(ordinary.clone(), Vec::new()),
            );
            machine.four_values = vec![FourValuesNow::Answered(four_values()), now].into();
            let held = a_run_with_a_pick(file);
            let came_to = the_read_of(&held, file, machine);
            assert!(
                matches!(
                    came_to,
                    WhatTheReadCameTo::ThePreLoad {
                        went: HowThePreLoadWent::AReading {
                            landed: Landed::DiscardedTheFileMoved,
                            the_diary_act: None
                        },
                        ..
                    }
                ),
                "{came_to:?}"
            );
            assert!(held
                .lock()
                .expect("held")
                .readings
                .standing(&the_key(file))
                .is_none());
        }
    }

    /// **Where the four values cannot be read at the press, no Blender is
    /// started**: no reading could carry the file identity every recorded
    /// reading carries (`Render Settings` Q11).
    #[test]
    fn with_no_four_values_to_start_under_nothing_is_asked_of_the_supervisor() {
        let file = &a_file_of_its_own(
            "with_no_four_values_to_start_under_nothing_is_asked_of_the_supervisor",
        );
        let mut machine = AStandIn::answering(Vec::new(), Vec::new());
        machine.four_values = vec![FourValuesNow::TheChannelCannotAnswer].into();
        let asked_list = Arc::clone(&machine.asked);
        let held = a_run_with_a_pick(file);
        let came_to = block_on(the_file_is_read(machine, &held, &a_server(), file));
        assert_eq!(came_to, WhatTheReadCameTo::TheFourValuesCouldNotBeRead);
        assert!(asked_list.lock().expect("the list").is_empty());
    }

    /// **The section's third and fourth desk checks: a second press while one
    /// runs joins it, and a version change while it runs discards the
    /// reading.** `Render Settings` Q5 and `Render Settings` Q17, through this file's seam and
    /// the run's real memory.
    #[test]
    fn a_second_press_joins_the_first_and_a_version_change_while_it_runs_discards_it() {
        let file = &a_file_of_its_own(
            "a_second_press_joins_the_first_and_a_version_change_while_it_runs_discards_it",
        );
        let ordinary = framed(&a_printed_document(&a_settings_half(), 3));
        let asks = Arc::new(AtomicUsize::new(0));

        // `Render Settings` Q5: two presses of one file, while the first is in
        // flight.
        let held = a_run_with_a_pick(file);
        let gate = Arc::new(tokio::sync::Notify::new());
        let mut first = AStandIn::answering(
            vec![ended(&ordinary, b"", Some("0"))],
            streams(ordinary.clone(), Vec::new()),
        );
        first.gate = Some(Arc::clone(&gate));
        let first_asked = Arc::clone(&first.asked);
        let second = AStandIn::answering(
            vec![ended(&ordinary, b"", Some("0"))],
            streams(ordinary.clone(), Vec::new()),
        );
        let second_asked = Arc::clone(&second.asked);
        let server = a_server();
        let (one, two) = block_on(async {
            let resolution = crate::surface::render_settings::the_version_resolved(
                &held.lock().expect("held"),
                &a_server(),
                file,
            );
            let mut first = quick(first);
            let mut second = quick(second);
            let releasing = async {
                // `join!` polls the two presses first, so both are in: the
                // first is waiting on its machine, the second has joined it.
                for _ in 0..16 {
                    tokio::task::yield_now().await;
                }
                asks.fetch_add(1, Ordering::Relaxed);
                gate.notify_one();
            };
            let (one, two, ()) = tokio::join!(
                the_file_is_read_with(
                    &mut first,
                    &held,
                    &server,
                    file,
                    the_key(file),
                    &resolution,
                    four_values()
                ),
                the_file_is_read_with(
                    &mut second,
                    &held,
                    &server,
                    file,
                    the_key(file),
                    &resolution,
                    four_values()
                ),
                releasing,
            );
            (one, two)
        });
        assert_eq!(first_asked.lock().expect("the list").len(), 1);
        assert_eq!(
            second_asked.lock().expect("the list").len(),
            0,
            "the second press started a read"
        );
        for came_to in [&one, &two] {
            assert!(
                matches!(came_to, WhatTheReadCameTo::ThePreLoad { went, .. } if went.the_files_half_was_replaced()),
                "{came_to:?}"
            );
        }

        // `Render Settings` Q17: the version picked on the page moves while the
        // read runs.
        let held = a_run_with_a_pick(file);
        held.lock()
            .expect("held")
            .version_lists
            .get_mut(&a_server())
            .expect("the list")
            .entries
            .push(smoozy_server_setup::search::ACandidate {
                path: "/opt/blender-4.5.12/blender".to_string(),
                answered: Some(a_version(4, 5, 12)),
            });
        let gate = Arc::new(tokio::sync::Notify::new());
        let mut machine = AStandIn::answering(
            vec![ended(&ordinary, b"", Some("0"))],
            streams(ordinary.clone(), Vec::new()),
        );
        machine.gate = Some(Arc::clone(&gate));
        let server = a_server();
        let came_to = block_on(async {
            let resolution = crate::surface::render_settings::the_version_resolved(
                &held.lock().expect("held"),
                &a_server(),
                file,
            );
            let mut reads = quick(machine);
            let moving = async {
                tokio::task::yield_now().await;
                let _ = crate::surface::render_settings::the_answer_is_taken(
                    &mut held.lock().expect("held"),
                    &a_server(),
                    file,
                    WhichSetting::VersionPick,
                    "/opt/blender-4.5.12/blender",
                );
                gate.notify_one();
            };
            let (came_to, ()) = tokio::join!(
                the_file_is_read_with(
                    &mut reads,
                    &held,
                    &server,
                    file,
                    the_key(file),
                    &resolution,
                    four_values()
                ),
                moving,
            );
            came_to
        });
        assert!(
            matches!(
                came_to,
                WhatTheReadCameTo::ThePreLoad {
                    went: HowThePreLoadWent::AReading {
                        landed: Landed::DiscardedTheVersionMoved,
                        the_diary_act: None
                    },
                    ..
                }
            ),
            "{came_to:?}"
        );
        assert!(held
            .lock()
            .expect("held")
            .readings
            .standing(&the_key(file))
            .is_none());
    }

    /// **`Render Settings` Q18: the moment a reading states is stamped when it
    /// is stored**, not when *Read the file* was pressed — `pre_load` is
    /// handed a moment at the press, and a read takes seconds.
    #[test]
    fn a_reading_is_stamped_when_it_is_stored_and_not_at_the_press() {
        let file = &a_file_of_its_own("stamped");
        let held = a_run_with_a_pick(file);
        let pick = crate::surface::render_settings::the_version_resolved(
            &held.lock().expect("held"),
            &a_server(),
            file,
        )
        .the_payloads_pick();
        let landed = TheFilesHalfOfThisRun {
            held: &held,
            server: &a_server(),
            file,
            learnt: Arc::default(),
        }
        .a_reading_landed(
            the_key(file),
            &StartedUnder {
                version_pick: pick.clone().expect("a pick"),
                file_identity: four_values(),
            },
            a_settings_half(),
            pick.as_ref(),
            &four_values(),
            SystemTime::UNIX_EPOCH,
        );
        assert_eq!(landed, Landed::Stored);
        let held = held.lock().expect("held");
        let stored_at = held
            .readings
            .standing(&the_key(file))
            .expect("stored")
            .stored_at
            .taken_at();
        assert!(
            stored_at > SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000),
            "{stored_at:?}"
        );
    }

    /// **The section's first and second desk checks, through this file's seam,
    /// with the three real splash readings as the machine's streams**: 5.2
    /// shows frames 1–250, 514 samples and *0.01*; 4.5, an EEVEE file, shows no
    /// early-stop line.
    ///
    /// **Ignored, because the readings are not in the tree**: they are the
    /// splash tester's. Run by hand with `SMOOZY_DESK_SPLASH_RESULTS` naming
    /// that tester's `results/` folder.
    #[test]
    #[ignore = "reads the splash tester's readings from outside the tree"]
    fn the_three_splash_readings_come_back_through_the_stream_and_draw_what_blender_shows() {
        let file = &a_file_of_its_own("splash");
        let folder = std::path::PathBuf::from(
            std::env::var("SMOOZY_DESK_SPLASH_RESULTS").expect("SMOOZY_DESK_SPLASH_RESULTS"),
        );
        for (name, version, build) in [
            (
                "reading-5.2-as.json",
                a_version(5, 2, 2),
                "/opt/blender-5.2.2/blender",
            ),
            (
                "reading-5.1-as.json",
                a_version(5, 1, 2),
                "/opt/blender-5.1.2/blender",
            ),
            (
                "reading-4.5-as.json",
                a_version(4, 5, 12),
                "/opt/blender-4.5.12/blender",
            ),
        ] {
            let whole: serde_json::Value =
                serde_json::from_slice(&std::fs::read(folder.join(name)).expect("the reading"))
                    .expect("it reads");
            let document = serde_json::to_vec(&whole["printed"]).expect("it renders");
            let ordinary = framed(&document);
            let machine = AStandIn::answering(
                vec![still_in_hand(), ended(&ordinary, b"", Some("0"))],
                streams(ordinary.clone(), Vec::new()),
            );
            let mut held = a_run_with_a_reading();
            held.version_lists
                .get_mut(&a_server())
                .expect("the list")
                .entries
                .push(smoozy_server_setup::search::ACandidate {
                    path: build.to_string(),
                    answered: Some(version),
                });
            let _ = crate::surface::render_settings::the_answer_is_taken(
                &mut held,
                &a_server(),
                file,
                WhichSetting::VersionPick,
                build,
            );
            let held = Mutex::new(held);
            let came_to = the_read_of(&held, file, machine);
            assert!(
                matches!(&came_to, WhatTheReadCameTo::ThePreLoad { went, .. } if went.the_files_half_was_replaced()),
                "{name}: {came_to:?}"
            );
            let held = held.lock().expect("held");
            let panel = the_panel(&held, &a_server(), file);
            let lines: Vec<String> = panel
                .the_samples_lines
                .iter()
                .map(the_words_of_a_line)
                .collect();
            println!(
                "{name}: {} bytes on the stream; range {:?}, frames {}, samples {:?}, lines {lines:?}",
                ordinary.len(),
                panel.the_range.value.as_ref().map(|value| value.as_str().to_string()),
                the_words_of_a_line(&panel.the_frame_count),
                panel.samples.value.as_ref().map(|value| value.as_str().to_string()),
            );
            match name {
                "reading-5.2-as.json" => {
                    assert_eq!(
                        panel.the_range.value.as_ref().map(|value| value.as_str()),
                        Some("1-250")
                    );
                    assert!(the_words_of_a_line(&panel.the_frame_count).contains("<250>"));
                    assert_eq!(
                        panel.samples.value.as_ref().map(|value| value.as_str()),
                        Some("514")
                    );
                    assert!(lines[0].contains("<0.01>"), "{lines:?}");
                }
                "reading-4.5-as.json" => assert!(lines.is_empty(), "{lines:?}"),
                _ => assert!(lines[0].contains("<0.01>"), "{lines:?}"),
            }
        }
    }
}
