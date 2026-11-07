//! **`Server Setup` R1b on the desktop: the driver follows every Blender on the
//! server, and the swap waits until nothing there is live.** Added 2026-10-12,
//! the OptiX and driver work's piece that builds the Creator's ruling of
//! 2026-10-11 (`Audits/Build/Step 10c/The Creator's rulings on OptiX and the
//! driver, 2026-10-11.md`, its last two sections).
//!
//! **What is not here, because it exists.** The comparison is `Server Setup`'s
//! own (`smoozy_server_setup::ready_action::a_swap_is_owed`, over
//! `the_bar_for_the_newest_blender_on_the_server`); the swap is `Server Setup`
//! R1 item 5's, unchanged; the swap's own run is driven by
//! [`crate::setting_up::the_swap_is_made`]. **This file holds what the
//! comparison concluded for the connected period, reads the live set, keeps the
//! transfers with each server, and starts an owed swap at the two moments
//! `Server Setup` R1b names.**
//!
//! # The four things held here, and how long each lives
//!
//! - **What the comparison concluded, per server, for the connected period and
//!   never stored** (`Server Setup` R1b, `State, identity and recovery` D4). A
//!   later comparison replaces it; the period's end forgets it
//!   ([`the_period_ended`]). Read by `Server Setup` R1c, the job held for the
//!   driver, through [`a_swap_is_owed_on`].
//! - **Whether `Server Setup` E2's ask was declined for the swap** — then the
//!   swap is not asked for again in that connected period.
//! - **The transfers under way between this desktop and each machine** — an
//!   upload or a manual download while its run holds it, and the automatic
//!   downloads while their arrangements still owe a file. `Server Setup` R1b
//!   counts them as live work (the runner's ruling, 2026-10-11). **The moment
//!   the last of them ends is one of the two moments an owed swap starts at.**
//! - **Nothing else.** Nothing here polls and nothing is timed.
//!
//! # The job held for the driver — `Server Setup` R1c
//!
//! Built here by held-job piece A, 2026-10-12, and nothing of it draws.
//!
//! - **The live set leaves it out**: [`anything_is_live_in`] counts every
//!   hold but one of the driver's reason, for `Server Setup` R1b and for
//!   nothing else.
//! - **What the submission reads**: [`TheDriverAtSubmission`], taken at the
//!   press, and `smoozy_server_setup::ready_action::a_job_is_held_for_the_driver`
//!   judges it per job.
//! - **What item 5 came to in a run**: the driving tells this file as it goes
//!   ([`item_5_compared_and_acted`], [`item_5s_last_check_read`]), because a
//!   run's ending does not carry it.
//! - **The release at a run's ending**: [`what_the_ending_releases`], and the
//!   sequence itself, [`the_driver_holds_are_released`] — one at a time, in
//!   submission order, each after the one before has an outcome read, and
//!   `Queue & History` N23's mark written first where the driver did not reach
//!   the bar.
//! - **A standing hold found by a queue reading**:
//!   [`a_standing_driver_hold_is_compared`].
//!
//! **This file keeps no header list**: every other document's marker is
//! written with that document at its own site, because this crate's own plan
//! defines clauses of the same short names.

use std::collections::{HashMap, HashSet};
use std::sync::{Mutex, OnceLock};

use smoozy_queue_history::acting::adoption::TheMachinesWorkingState;
use smoozy_queue_history::line::QueueState;
use smoozy_queue_history::readings::AJobAsTheMachineReadsIt;
use smoozy_state::identities::Identity;
use smoozy_server_setup::blender::Version;
use smoozy_server_setup::kind::MachineKind;
use smoozy_server_setup::ready_action;
use smoozy_servers::hardware::{TheDriversOwnAnswer, WhereK1PlacesIt};

use crate::setting_up::{TheRunsEnding, WhatWasAskedFor, WhereTheSetUpGot};

// ════════════════════════════════════════════════════════════════════════════
// What the comparison concluded, held for the connected period
// ════════════════════════════════════════════════════════════════════════════

/// **One server's period-held answer.**
#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct WhatThePeriodHolds {
    /// **A swap is owed**, with the Blenders the comparison read, so the swap's
    /// own run takes the same bar. Nothing where none is owed.
    owed: Option<Vec<Version>>,
    /// **`Server Setup` E2's ask was declined for the swap in this period.**
    declined: bool,
}

fn held() -> &'static Mutex<HashMap<String, WhatThePeriodHolds>> {
    static HELD: OnceLock<Mutex<HashMap<String, WhatThePeriodHolds>>> = OnceLock::new();
    HELD.get_or_init(|| Mutex::new(HashMap::new()))
}

fn with_the_server<T>(server: &str, act: impl FnOnce(&mut WhatThePeriodHolds) -> T) -> Option<T> {
    let mut held = held().lock().ok()?;
    Some(act(held.entry(server.to_string()).or_default()))
}

/// **Whether `Server Setup` R1b's comparison concludes *a swap is owed* on this
/// server for this connected period** — what `Server Setup` R1c reads at
/// submission.
pub fn a_swap_is_owed_on(server: &str) -> bool {
    held()
        .lock()
        .map(|held| held.get(server).is_some_and(|it| it.owed.is_some()))
        .unwrap_or(false)
}

/// **The Blenders the owed swap was concluded for**, where one is owed.
pub fn the_blenders_the_swap_is_owed_for(server: &str) -> Option<Vec<Version>> {
    held().lock().ok()?.get(server)?.owed.clone()
}

/// **A comparison concluded**, and it replaces what stood (`Server Setup` R1b:
/// *"A later comparison replaces it."*).
pub fn the_comparison_concluded(server: &str, owed: Option<Vec<Version>>) {
    with_the_server(server, |it| it.owed = owed);
}

/// **Whether `Server Setup` E2's ask was declined for the swap in this
/// period**, so it is not asked for again.
pub fn the_ask_was_declined_on(server: &str) -> bool {
    held()
        .lock()
        .map(|held| held.get(server).is_some_and(|it| it.declined))
        .unwrap_or(false)
}

/// **`Servers` S11: every period-held thing ends the moment the period ends**
/// — what the comparison concluded, a declined ask, and the transfers counted
/// with that server. A swap owed is lost with it, and the next comparison
/// finds it again (`Server Setup` R1b).
pub fn the_period_ended(server: &str) {
    if let Ok(mut held) = held().lock() {
        held.remove(server);
    }
    if let Ok(mut transfers) = the_transfers().lock() {
        transfers.a_period_ended(server);
    }
}

/// **What a run whose `Server Setup` R1 item 5 compared leaves for the
/// period** — the ready action and the swap's own run, and nothing else.
///
/// - ***Done*, with the swap left out for live work**: a swap is owed, for the
///   Blenders the run's bar was taken over.
/// - ***Done* any other way**: the swap was made, missed (kept on the row
///   instead, `Servers` H11b), or not owed. Nothing is owed.
/// - **`Server Setup` E2's ask declined at the driver step**: no swap, and it
///   is not asked for again this period.
/// - **Anything else** — a run cut off part-way, stopped or failed — **leaves
///   what stood**, so the swap waits again.
///
/// **`Server Setup` R1c's release follows it**, at
/// [`the_holds_are_released_after_the_run`], which both runs call next.
pub fn the_ending_is_kept(server: &str, came_to: &WhereTheSetUpGot, asked_for: &WhatWasAskedFor) {
    match came_to {
        WhereTheSetUpGot::TheRunEnded {
            ending: TheRunsEnding::ItReachedDone { the_swap_waits, .. },
            ..
        } => {
            let owed = the_swap_waits.then(|| {
                let mut read = asked_for.the_blenders_on_the_server.clone();
                if let Some(staged) = asked_for
                    .install_this_blender
                    .as_deref()
                    .and_then(Version::parse)
                {
                    read.push(staged);
                }
                read
            });
            the_comparison_concluded(server, owed);
        }
        WhereTheSetUpGot::TheRunEnded {
            ending:
                TheRunsEnding::AStepNeedsAdminAccess {
                    step: smoozy_server_setup::ready_action::Step::Driver,
                },
            ..
        } => {
            with_the_server(server, |it| it.declined = true);
        }
        _ => {}
    }
}

// ════════════════════════════════════════════════════════════════════════════
// The comparison, at `Server Setup` B12's moments and after an install
// ════════════════════════════════════════════════════════════════════════════

/// **`Rented-server runtime` K1's placing, read as `Server Setup` M1's kind** —
/// its own kernel is the systemd kind; the refused row is a machine the table
/// does not place.
pub fn the_kind_k1_placed(placed: WhereK1PlacesIt) -> MachineKind {
    match placed {
        WhereK1PlacesIt::ItsOwnKernel => MachineKind::ASystemdMachine,
        WhereK1PlacesIt::AContainer => MachineKind::AContainer,
        WhereK1PlacesIt::NowhereItPlaces => MachineKind::TheTableDoesNotPlaceIt,
    }
}

/// **`Server Setup` R1b's comparison over the Blenders a reading found**: the
/// driver's version `Servers` H11b holds for this connection against
/// `Rendering` L23a's bar for the newest of them. **Only a reading of this
/// connection is compared** — a driver last seen on an earlier one may have
/// been replaced since. **A miss kept on the row concludes nothing is owed**
/// until the next ready action (the runner's ruling, 2026-10-11).
///
/// What comes back is what [`the_comparison_concluded`] holds: the Blenders it
/// read, where a swap is owed.
pub fn the_comparison_on(
    the_driver_on_this_connection: Option<&TheDriversOwnAnswer>,
    a_missed_swap_stands: bool,
    found: &[Version],
) -> Option<Vec<Version>> {
    let the_driver = the_driver_on_this_connection?;
    let bar = ready_action::the_bar_for_the_newest_blender_on_the_server(None, found);
    ready_action::a_swap_is_owed(
        the_kind_k1_placed(the_driver.where_k1_placed_the_machine),
        Some(&the_driver.version),
        bar,
        a_missed_swap_stands,
    )
    .then(|| found.to_vec())
}

/// **The versions every build the search found answered**, or nothing where
/// the machine could not be asked — which is no reading and concludes nothing.
pub fn the_versions_in(
    asked: &smoozy_render_settings::acting::versions::TheVersionListAsk,
) -> Option<Vec<Version>> {
    match asked {
        smoozy_render_settings::acting::versions::TheVersionListAsk::Answered(result) => Some(
            result
                .entries
                .iter()
                .filter_map(|entry| entry.answered)
                .collect(),
        ),
        smoozy_render_settings::acting::versions::TheVersionListAsk::TheMachineCouldNotBeAsked => None,
    }
}

/// **`Server Setup` R1b's comparison at one of `Server Setup` B12's moments** —
/// the Set up tab's search (the view opened, and the table a removal is offered
/// from), and `Render Settings` asking for the version list — **and after an
/// install that published a build**, whose search reads the version the
/// published build answered beside every other. **Never at connection**:
/// nothing calls this there, and it takes no search of its own.
///
/// **Where a swap is owed and nothing is live, it is made at once.**
pub fn the_search_is_compared<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    server: &str,
    asked: &smoozy_render_settings::acting::versions::TheVersionListAsk,
) {
    let Some(found) = the_versions_in(asked) else {
        return;
    };
    let held = crate::surface::set_up::the_driver_as_held_for(app, server);
    let on_this_connection = held.the_driver.as_ref().filter(|_| held.last_seen_at.is_none());
    let owed = the_comparison_on(on_this_connection, held.the_driver_missed_the_optix_bar, &found);
    let it_is_owed = owed.is_some();
    the_comparison_concluded(server, owed);
    if it_is_owed {
        an_owed_swap_is_tried(app, server, None);
    }
}

/// **`Server Setup` B10's search, taken over the server's own point for the
/// ready action** — `Server Setup` R1b: *"The ready action runs the search"*.
/// The same ask the Set up tab and `Render Settings` make
/// ([`crate::submitting::the_blenders_on_the_machine`]); Smoozy's own Blender
/// area is not a candidate where the row records a layout. **Nothing where the
/// machine could not be asked**, and the run's bar is then the staged build's
/// alone.
pub async fn the_blenders_found_now<C: crate::servers::sessions::AHeldConnection>(
    sessions: &crate::servers::sessions::ThisServersSessions<C>,
    server: &str,
    row: Option<&smoozy_servers::store::ServerRow>,
) -> Vec<Version> {
    let own_area = row
        .and_then(crate::servers::installation::the_layout_recorded_for)
        .map(|layout| smoozy_server_setup::blender::the_blender_area(&layout));
    let asked = crate::submitting::the_blenders_on_the_machine(sessions, server, own_area.as_deref()).await;
    the_versions_in(&asked).unwrap_or_default()
}

// ════════════════════════════════════════════════════════════════════════════
// The live set
// ════════════════════════════════════════════════════════════════════════════

/// **`Server Setup` R1b's live set, off one queue reading**: any job the
/// machine holds rendering, waiting or held — `Queue & History` §17's three
/// states — **except a job held for the swap** (`Server Setup` R1c): it is the
/// job the swap is for, and counting it would hold the swap back for ever.
/// *Added 2026-10-12, held-job piece A; every hold counted until then.*
///
/// **Left out here and nowhere else**: every other reader of the live set —
/// `File Browser`'s warnings, a Blender's removal warning, the program's
/// update — still counts it. **A held job whose reading says nothing of its
/// hold** (a program older than step 10c's eleventh-pass fix piece 8b) is
/// counted, since nothing says it is the driver's.
pub fn anything_is_live_in(reading: &TheMachinesWorkingState) -> bool {
    reading.jobs.iter().any(|job| {
        matches!(
            job.state,
            QueueState::Waiting | QueueState::Held { .. } | QueueState::Rendering { .. }
        ) && !a_hold_for_the_driver(job)
    })
}

/// **Whether a job on the reading is held for the driver** — held, under a
/// hold whose reason is `Rendering` S18a's own word. Nothing else is read: the
/// reason is the only thing that tells this hold from the check's.
pub fn a_hold_for_the_driver(job: &AJobAsTheMachineReadsIt) -> bool {
    matches!(job.state, QueueState::Held { .. })
        && job
            .the_hold
            .as_ref()
            .is_some_and(|hold| hold.reason == smoozy_rendering::submission::THE_DRIVER)
}

/// **A job held for the driver, with the hold it is under.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ADriverHold {
    pub job: Identity,
    pub hold: Identity,
}

/// **Every hold of the driver's reason on one reading, in the reading's
/// order.** [`in_submission_order`] puts them in the order `Server Setup` R1c
/// releases them in.
pub fn the_driver_holds_in(reading: &TheMachinesWorkingState) -> Vec<ADriverHold> {
    reading
        .jobs
        .iter()
        .filter(|job| a_hold_for_the_driver(job))
        .filter_map(|job| match &job.state {
            QueueState::Held { hold } => Some(ADriverHold {
                job: job.job.clone(),
                hold: hold.clone(),
            }),
            _ => None,
        })
        .collect()
}

/// **`Server Setup` R1c's order: the order the jobs were submitted in**, read
/// off `Queue & History` N16's submitted moment on each job's local record. A
/// job the record does not name, or names with no moment, goes after every
/// one that has one, in the reading's order — the sort is stable.
pub fn in_submission_order(
    mut holds: Vec<ADriverHold>,
    submitted: &dyn Fn(&Identity) -> Option<std::time::SystemTime>,
) -> Vec<ADriverHold> {
    holds.sort_by_key(|held| match submitted(&held.job) {
        Some(at) => (0, Some(at)),
        None => (1, None),
    });
    holds
}

// ════════════════════════════════════════════════════════════════════════════
// `Server Setup` R1c: what the submission reads
// ════════════════════════════════════════════════════════════════════════════

/// **What `Server Setup` R1c reads at submission beside the job's own
/// device and Blender** — taken at the press, for the server the act is made
/// on. **The default holds nothing**, which is every act composed where none
/// of this was taken.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TheDriverAtSubmission {
    /// **`Servers` H11b's driver reading, of this connection only** — a
    /// reading last seen on an earlier connection may have been replaced since.
    pub the_driver_on_this_connection: Option<TheDriversOwnAnswer>,
    /// **`Server Setup` R1 item 5's miss, kept on the row**: `Servers` H11b's
    /// statement stands on a machine Smoozy manages.
    pub a_missed_swap_stands: bool,
    /// **`Server Setup` R1b's comparison, held for this connected period.**
    pub a_swap_is_owed: bool,
    /// **The program on the server is this application's own release**: none
    /// of `Desktop app and delivery` V13's conditions stands for the server.
    /// **A hold with no dependency is taken only by a program from held-job
    /// piece A on**; an older one refuses the whole submission by name, so a
    /// job that would be held there is sent plain and meets `Rendering` §9
    /// item 3 as it did before. *The builder's reading, where the clause left
    /// the choice: `Server Setup` R1c's dated bullet says so.*
    pub the_program_is_this_applications_own: bool,
}

/// **The engine a job will use, as `Server Setup` R1c's bullet of 2026-11-02
/// reads it at submission.** Added 2026-11-02, the first 5.3 live pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheEngineAtSubmission {
    Cycles,
    /// **EEVEE, or a third engine the file names.** The bar is Cycles'
    /// (`Rendering` L23a).
    AnotherEngine,
    /// **No pick, and no reading of the job's scene.**
    NotKnown,
}

impl TheEngineAtSubmission {
    /// **The user's pick (`Render Settings` E1), else the file's own for the
    /// job's scene as the reading gives it (`Render Settings` E4)** — what the page's engine in
    /// force and the render's start both resolve.
    pub fn of(
        the_users_pick: Option<smoozy_render_settings::engine::OfferedEngine>,
        the_jobs_scene_as_read: Option<(
            &smoozy_render_settings::reading::SceneReading,
            &smoozy_render_settings::file_half::RecordedReading,
        )>,
    ) -> Self {
        use smoozy_render_settings::engine::{what_the_page_shows, OfferedEngine, WhatThePageShows};
        let the_engine = match (the_users_pick, the_jobs_scene_as_read) {
            (Some(pick), _) => pick,
            (None, Some((scene, reading))) => match what_the_page_shows(scene, reading) {
                WhatThePageShows::Offered(engine) => engine,
                WhatThePageShows::TheFilesOwn(_) => return TheEngineAtSubmission::AnotherEngine,
            },
            (None, None) => return TheEngineAtSubmission::NotKnown,
        };
        match the_engine {
            OfferedEngine::Cycles => TheEngineAtSubmission::Cycles,
            OfferedEngine::Eevee => TheEngineAtSubmission::AnotherEngine,
        }
    }
}

impl TheDriverAtSubmission {
    /// **`Server Setup` R1c's four, for one job, with the program's own
    /// condition before them.** `the_device` is `Queue & History` E12a's
    /// resolved device choice, nothing where it is not known at submission;
    /// `the_card_make` is `Job files and outputs` M1b's field the manifest
    /// carries; `the_jobs_blender` is the version its pick answered.
    ///
    /// **`Server Setup` R1c's bullet of 2026-11-02: only a Cycles job**, because
    /// only a Cycles job's line names `OPTIX` (`Rendering` L21's bullet of that
    /// day). **An engine not known keeps the device rule**: the desktop cannot
    /// tell what the line will name, so the job is judged on its device alone,
    /// as it was, and the machine decides the line from its own reading.
    pub fn holds_this_job(
        &self,
        the_device: Option<smoozy_render_settings::device::DeviceChoice>,
        the_engine: TheEngineAtSubmission,
        the_card_make: &smoozy_job_files::manifest::TheCardMakeAtSubmission,
        the_jobs_blender: Option<Version>,
    ) -> bool {
        let its_launch_would_name_optix = the_engine != TheEngineAtSubmission::AnotherEngine
            && the_device == Some(smoozy_render_settings::device::DeviceChoice::TheCards)
            && matches!(
                smoozy_rendering::technology::the_technology_the_manifest_names(the_card_make),
                Ok(smoozy_rendering::technology::Technology::Optix)
            );
        self.the_program_is_this_applications_own
            && ready_action::a_job_is_held_for_the_driver(
                its_launch_would_name_optix,
                self.the_driver_on_this_connection.as_ref(),
                the_jobs_blender,
                self.a_swap_is_owed,
                self.a_missed_swap_stands,
            )
    }
}

/// **What `Server Setup` R1c reads at submission, taken now for one server.**
pub fn the_driver_at_submission<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    server: &str,
) -> TheDriverAtSubmission {
    let held = crate::surface::set_up::the_driver_as_held_for(app, server);
    TheDriverAtSubmission {
        the_driver_on_this_connection: held.the_driver.filter(|_| held.last_seen_at.is_none()),
        a_missed_swap_stands: held.the_driver_missed_the_optix_bar,
        a_swap_is_owed: a_swap_is_owed_on(server),
        the_program_is_this_applications_own: crate::servers::published::what_the_server_carries(server)
            .is_none(),
    }
}

// ════════════════════════════════════════════════════════════════════════════
// `Server Setup` R1c: what item 5 came to, and the release
// ════════════════════════════════════════════════════════════════════════════

/// **What `Server Setup` R1 item 5 came to in the run being driven on one
/// server** — told by the driving as it goes, because a run's ending says
/// whether the driver missed the bar only where the run reached *done*, and
/// `Server Setup` R1c releases after a failed run too.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WhatItem5CameTo {
    /// **Item 5 compared and acted**: its driver step ran on a machine with a
    /// bar to read against, and the swap was not left out for live work — the
    /// swap was attempted, or the readings found the driver already at the bar.
    pub it_compared_and_acted: bool,
    /// **The driver's version as the driver wrote it**, from the last reading
    /// the run took. Nothing where none was read.
    pub the_drivers_version: Option<String>,
    /// **What the last check found**: below the bar or not. Nothing where the
    /// run never reached it.
    pub the_last_check_found_it_below: Option<bool>,
}

fn what_item_5_came_to_on() -> &'static Mutex<HashMap<String, WhatItem5CameTo>> {
    static HELD: OnceLock<Mutex<HashMap<String, WhatItem5CameTo>>> = OnceLock::new();
    HELD.get_or_init(|| Mutex::new(HashMap::new()))
}

/// **A run whose item 5 may compare begins on this server** — the ready
/// action, or the swap's own run. What an earlier run left is forgotten.
pub fn a_run_whose_item_5_may_compare_begins(server: &str) {
    if let Ok(mut held) = what_item_5_came_to_on().lock() {
        held.remove(server);
    }
}

/// **Item 5 compared and acted**, with the version it read before acting.
pub fn item_5_compared_and_acted(
    server: &str,
    the_drivers_version: Option<&smoozy_servers::hardware::DriverVersion>,
) {
    if let Ok(mut held) = what_item_5_came_to_on().lock() {
        let it = held.entry(server.to_string()).or_default();
        it.it_compared_and_acted = true;
        it.the_drivers_version = the_drivers_version.map(|v| v.as_the_driver_wrote_it().to_string());
    }
}

/// **Item 5's last check read the driver** — at the driver step where no
/// restart follows, or at the check on return.
pub fn item_5s_last_check_read(
    server: &str,
    the_drivers_version: Option<&smoozy_servers::hardware::DriverVersion>,
    below: bool,
) {
    if let Ok(mut held) = what_item_5_came_to_on().lock() {
        let it = held.entry(server.to_string()).or_default();
        if let Some(version) = the_drivers_version {
            it.the_drivers_version = Some(version.as_the_driver_wrote_it().to_string());
        }
        it.the_last_check_found_it_below = Some(below);
    }
}

/// **What the run's driving told of item 5, taken once at the run's end.**
pub fn what_item_5_came_to(server: &str) -> WhatItem5CameTo {
    what_item_5_came_to_on()
        .lock()
        .ok()
        .and_then(|mut held| held.remove(server))
        .unwrap_or_default()
}

/// **What a run's ending releases** — `Server Setup` R1c's release bullets,
/// judged.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheEndingReleases {
    /// **Nothing**: item 5 did not compare and act, the run backed off at the
    /// last guard, the ask was declined, or the run was cut off part-way. The
    /// swap waits again, and so do the holds.
    Nothing,
    /// **Every hold of the driver's reason on the server**, with `Queue &
    /// History` N23's mark written first on each where `the_mark` is given —
    /// the driver's version, nothing where none was read.
    EveryDriverHold { the_mark: Option<Option<String>> },
}

/// **`Server Setup` R1c: release comes at the ending of the run whose item 5
/// compared and acted**, a ready action or a swap run.
///
/// - **It reached *done***: released; with the mark where the driver missed
///   the bar.
/// - **It failed** after item 5 acted — a step failed, the restart was
///   refused, the unpack failed: released all the same, with the mark, unless
///   the last check had already read the driver at the bar.
/// - **A run that backed off at the last guard, a declined elevation, and a
///   run cut off part-way release nothing** — the connection went, the server
///   stopped answering or ended the run, the user stopped it or stopped
///   waiting for the restart, the reconnection gave up, or it stopped inside a
///   step.
pub fn what_the_ending_releases(
    came_to: &crate::setting_up::WhereTheSetUpGot,
    item_5: &WhatItem5CameTo,
) -> WhatTheEndingReleases {
    if !item_5.it_compared_and_acted {
        return WhatTheEndingReleases::Nothing;
    }
    let the_mark = || Some(item_5.the_drivers_version.clone());
    match came_to {
        WhereTheSetUpGot::TheRunEnded {
            ending: TheRunsEnding::ItReachedDone { the_swap_waits: true, .. },
            ..
        } => WhatTheEndingReleases::Nothing,
        WhereTheSetUpGot::TheRunEnded {
            ending: TheRunsEnding::ItReachedDone { the_driver_missed_the_optix_bar, .. },
            ..
        } => WhatTheEndingReleases::EveryDriverHold {
            the_mark: the_driver_missed_the_optix_bar.then(the_mark).flatten(),
        },
        WhereTheSetUpGot::TheRunEnded {
            ending:
                TheRunsEnding::AStepFailed { .. }
                | TheRunsEnding::TheRestartWasRefused { .. }
                | TheRunsEnding::TheUnpackFailed { .. }
                // A failed `Server Setup` R1 item 8, told apart from
                // `AStepFailed` on 2026-10-17 only for its words.
                | TheRunsEnding::TheTelegramMaterialWasNotComposed { .. },
            ..
        } => WhatTheEndingReleases::EveryDriverHold {
            the_mark: (item_5.the_last_check_found_it_below != Some(false))
                .then(the_mark)
                .flatten(),
        },
        _ => WhatTheEndingReleases::Nothing,
    }
}

/// **`Server Setup` R1c's release, sent**: each hold of the driver's reason by
/// `Rented-server runtime` U9's ordinary command, the answer *go on* from the
/// desk, **one at a time in the order handed in** — submission order — **each
/// sent once the one before has a recorded outcome read**, as `Rendering` S4a
/// serializes submissions. So the jobs keep their order among themselves as
/// each joins the line (`Queue & History` H3).
///
/// **Where a mark is owed it is written first** (`Queue & History` N23) —
/// `the_mark_is_written` answers for each job, true where none is owed — and
/// where that write is refused no release is sent for that job. **The
/// sequence stops at the first job whose release is not sent or whose outcome
/// is not read**, so no later job joins the line ahead of it; the rest wait for
/// the next moment that releases them.
///
/// What comes back is every answer sent, in order.
pub fn the_driver_holds_are_released(
    machine: &mut dyn crate::servers::the_machines_work::AsksTheSupervisor,
    in_submission_order: &[ADriverHold],
    mut the_mark_is_written: Option<&mut dyn FnMut(&Identity) -> bool>,
    wait: crate::servers::stop_and_clear::TheWait,
) -> Vec<(Identity, crate::servers::the_machines_work::WhatTheAnswerCameTo)> {
    use crate::servers::the_machines_work::{the_answer_to_a_hold_is_sent, WhatTheAnswerCameTo};
    let mut sent = Vec::new();
    for held in in_submission_order {
        if let Some(write) = the_mark_is_written.as_mut() {
            if !write(&held.job) {
                break;
            }
        }
        let came_to = the_answer_to_a_hold_is_sent(
            machine,
            &held.hold,
            smoozy_runtime::holds::WhatTheAnswerSays::Yes,
            wait,
        );
        let its_outcome_was_read = matches!(came_to, WhatTheAnswerCameTo::TheHoldIsAnswered(_));
        sent.push((held.job.clone(), came_to));
        if !its_outcome_was_read {
            break;
        }
    }
    sent
}

/// **The servers whose releases are in hand now**, so two moments meeting at
/// once send one sequence.
fn the_releases_in_hand() -> &'static Mutex<HashSet<String>> {
    static HELD: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();
    HELD.get_or_init(|| Mutex::new(HashSet::new()))
}

/// **`Server Setup` R1c at the ending of a ready action or a swap run on one
/// server**: what item 5 came to is taken, the ending judged, and every hold
/// of the driver's reason released where it says so. Called by both runs,
/// after [`the_ending_is_kept`].
pub fn the_holds_are_released_after_the_run<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    server: &str,
    came_to: &crate::setting_up::WhereTheSetUpGot,
) {
    let item_5 = what_item_5_came_to(server);
    if let WhatTheEndingReleases::EveryDriverHold { the_mark } = what_the_ending_releases(came_to, &item_5) {
        the_releases_are_sent(app, server, Box::new(move |_| the_mark.clone()), None);
    }
}

/// **`Server Setup` R1c's third bullet: a standing hold of the driver's reason
/// is a moment for `Server Setup` R1b's comparison** — at the queue reading of any
/// connection, against the bar for the held job's own Blender, the version its
/// record carries (`Queue & History` N19). **No search is taken.**
///
/// The judgement is `Server Setup`'s own
/// (`smoozy_server_setup::ready_action::what_a_standing_hold_comes_to`): where
/// a swap is owed it is concluded for the period and `Server Setup` R1b's starting rule
/// follows at the same reading; where the run that was owed has already ended
/// — the driver at the bar, or a miss kept — the holds are released as its
/// ending would have released them. **Nothing while a run is driven on the
/// server**: that run's own ending releases.
pub fn a_standing_driver_hold_is_compared<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    server: &str,
    reading: Option<&TheMachinesWorkingState>,
) {
    let Some(reading) = reading else {
        return;
    };
    let holds = the_driver_holds_in(reading);
    if holds.is_empty() || crate::setting_up::the_kind_being_driven_on(server).is_some() {
        return;
    }
    let held = crate::surface::set_up::the_driver_as_held_for(app, server);
    let on_this_connection = held.the_driver.as_ref().filter(|_| held.last_seen_at.is_none());
    match what_the_standing_holds_come_to(
        &holds,
        &|job| the_record_of(app, job).and_then(|record| Version::parse(&record.blender.pick.version)),
        on_this_connection,
        held.the_driver_missed_the_optix_bar,
    ) {
        WhatTheStandingHoldsComeTo::Nothing => {}
        WhatTheStandingHoldsComeTo::ASwapIsOwedFor(blenders) => {
            let mut owed = the_blenders_the_swap_is_owed_for(server).unwrap_or_default();
            owed.extend(blenders);
            the_comparison_concluded(server, Some(owed));
        }
        WhatTheStandingHoldsComeTo::TheyAreReleased { marked, the_drivers_version } => {
            the_releases_are_sent(
                app,
                server,
                Box::new(move |job| marked.contains(&job.as_text()).then(|| the_drivers_version.clone())),
                Some(reading),
            );
        }
    }
}

/// **What the holds of the driver's reason one reading found come to**, all
/// together.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheStandingHoldsComeTo {
    /// **Nothing to read them against** — no driver reading of this
    /// connection. They stand.
    Nothing,
    /// **A swap is owed**, for these Blenders — the held jobs' own.
    ASwapIsOwedFor(Vec<Version>),
    /// **The run that was owed has ended out of sight, and they are released**
    /// — each job in `marked` with `Queue & History` N23's mark, naming the
    /// driver's version.
    TheyAreReleased {
        marked: HashSet<String>,
        the_drivers_version: Option<String>,
    },
}

/// **`Server Setup` R1c's third bullet over every standing hold of one
/// reading**, each judged by `Server Setup`'s own
/// `ready_action::what_a_standing_hold_comes_to` against its own Blender
/// (`the_jobs_blender`, off its local record). **Any one owing a swap makes the
/// swap owed for all**, since the swap is the server's; **no driver reading
/// releases nothing**; otherwise every one is released, and only a job whose
/// bar the driver is still below carries the mark.
pub fn what_the_standing_holds_come_to(
    holds: &[ADriverHold],
    the_jobs_blender: &dyn Fn(&Identity) -> Option<Version>,
    the_driver_on_this_connection: Option<&TheDriversOwnAnswer>,
    a_missed_swap_stands: bool,
) -> WhatTheStandingHoldsComeTo {
    use smoozy_server_setup::ready_action::WhatAStandingHoldComesTo as Comes;
    let blenders: Vec<Option<Version>> = holds.iter().map(|held| the_jobs_blender(&held.job)).collect();
    let comes: Vec<Comes> = blenders
        .iter()
        .map(|blender| {
            ready_action::what_a_standing_hold_comes_to(the_driver_on_this_connection, a_missed_swap_stands, *blender)
        })
        .collect();
    if holds.is_empty() || comes.contains(&Comes::Nothing) {
        return WhatTheStandingHoldsComeTo::Nothing;
    }
    if comes.contains(&Comes::ASwapIsOwed) {
        return WhatTheStandingHoldsComeTo::ASwapIsOwedFor(blenders.into_iter().flatten().collect());
    }
    WhatTheStandingHoldsComeTo::TheyAreReleased {
        marked: holds
            .iter()
            .zip(&comes)
            .filter(|(_, comes)| **comes == Comes::ItIsReleasedWithTheMark)
            .map(|(held, _)| held.job.as_text())
            .collect(),
        the_drivers_version: the_driver_on_this_connection
            .map(|driver| driver.version.as_the_driver_wrote_it().to_string()),
    }
}

/// **Which mark a job's release carries**: nothing where none is owed, and
/// the driver's version — nothing where none was read — where `Queue &
/// History` N23's mark is.
type TheMarkFor = Box<dyn Fn(&Identity) -> Option<Option<String>> + Send>;

/// **One job's local record**, read through the application's record.
fn the_record_of<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    job: &Identity,
) -> Option<smoozy_queue_history::record::LocalJobRecord> {
    use tauri::Manager;
    let started = app.try_state::<Mutex<crate::start::Started>>()?;
    let started = started.lock().ok()?;
    crate::record::job_records::JobRecords::over(started.record.as_ref()?.connection())
        .read(job)
        .ok()
        .flatten()
}

/// **The release sequence for one server, off this thread** — the holds read
/// from the reading in hand, or a fresh one; put in submission order; the mark
/// written through the application's record where one is owed; and every
/// answer sent over the server's own point. **One sequence per server at a
/// time.**
fn the_releases_are_sent<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    server: &str,
    the_mark_for: TheMarkFor,
    reading: Option<&TheMachinesWorkingState>,
) {
    let Ok(mut in_hand) = the_releases_in_hand().lock() else {
        return;
    };
    if !in_hand.insert(server.to_string()) {
        return;
    }
    drop(in_hand);
    let reading = reading.cloned();
    let (app, server) = (app.clone(), server.to_string());
    let let_go = {
        let server = server.clone();
        move || {
            if let Ok(mut in_hand) = the_releases_in_hand().lock() {
                in_hand.remove(&server);
            }
        }
    };
    let spawned = std::thread::Builder::new()
        .name("smoozy driver release".to_string())
        .spawn({
            let let_go = let_go.clone();
            move || {
                let reading = reading.or_else(|| the_queue_reading_now(&app, &server));
                if let (Some(reading), Some(mut machine)) = (reading, the_machine_now(&app, &server)) {
                    let holds = in_submission_order(the_driver_holds_in(&reading), &|job| {
                        the_record_of(&app, job)
                            .and_then(|record| record.submitted_at)
                            .map(|at| at.taken_at())
                    });
                    let mut write = |job: &Identity| match the_mark_for(job) {
                        None => true,
                        Some(version) => the_mark_is_written_for(&app, job, version),
                    };
                    let sent = the_driver_holds_are_released(
                        &mut machine,
                        &holds,
                        Some(&mut write),
                        crate::servers::stop_and_clear::TheWait::THE_STOPS_OWN,
                    );
                    if !sent.is_empty() {
                        crate::surface::the_queue::the_queue_is_told_again(&app);
                    }
                }
                let_go();
            }
        });
    if spawned.is_err() {
        let_go();
    }
}

/// **`Queue & History` N23's mark, written through the application's record**
/// — whether it landed.
fn the_mark_is_written_for<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    job: &Identity,
    the_drivers_version: Option<String>,
) -> bool {
    use tauri::Manager;
    let Some(started) = app.try_state::<Mutex<crate::start::Started>>() else {
        return false;
    };
    let Ok(started) = started.lock() else {
        return false;
    };
    let Some(record) = started.record.as_ref() else {
        return false;
    };
    crate::record::job_records::JobRecords::over(record.connection())
        .the_driver_mark_is_written(job, the_drivers_version.as_deref())
        .unwrap_or(false)
}

// ════════════════════════════════════════════════════════════════════════════
// The transfers with each server
// ════════════════════════════════════════════════════════════════════════════

/// **The transfers under way between this desktop and each machine**, by the
/// local server identifier — run-held and never durable.
#[derive(Debug, Default)]
pub struct TheTransfersWithEachServer {
    /// Uploads and manual downloads whose runs hold them now.
    requests: HashMap<String, usize>,
    /// Servers whose automatic downloads still owe a file.
    the_automatic_downloads_owe: HashSet<String>,
}

impl TheTransfersWithEachServer {
    /// **Whether a transfer is under way with this server.**
    pub fn is_under_way(&self, server: &str) -> bool {
        self.requests.get(server).is_some_and(|count| *count > 0)
            || self.the_automatic_downloads_owe.contains(server)
    }

    /// An upload's or a manual download's run took its request.
    pub fn a_request_begins(&mut self, server: &str) {
        *self.requests.entry(server.to_string()).or_default() += 1;
    }

    /// **That run let it go — and whether it was the last transfer with the
    /// server**, which is `State, identity and recovery` F15's first moment.
    pub fn a_request_ends(&mut self, server: &str) -> bool {
        let was = self.is_under_way(server);
        if let Some(count) = self.requests.get_mut(server) {
            *count = count.saturating_sub(1);
            if *count == 0 {
                self.requests.remove(server);
            }
        }
        was && !self.is_under_way(server)
    }

    /// **The automatic downloads of this server owe a file, or no longer do**
    /// — and whether that was the last transfer with the server ending.
    pub fn the_automatic_downloads_owe(&mut self, server: &str, they_owe: bool) -> bool {
        let was = self.is_under_way(server);
        if they_owe {
            self.the_automatic_downloads_owe.insert(server.to_string());
        } else {
            self.the_automatic_downloads_owe.remove(server);
        }
        was && !self.is_under_way(server)
    }

    /// **The period's end**: the automatic downloads end with it. A request's
    /// own run lets it go when it returns.
    pub fn a_period_ended(&mut self, server: &str) {
        self.the_automatic_downloads_owe.remove(server);
    }
}

fn the_transfers() -> &'static Mutex<TheTransfersWithEachServer> {
    static HELD: OnceLock<Mutex<TheTransfersWithEachServer>> = OnceLock::new();
    HELD.get_or_init(|| Mutex::new(TheTransfersWithEachServer::default()))
}

/// **What is done when the last transfer with a server ends**, set once by the
/// application at its start (`src/main.rs`), where the application handle is.
type WhenTheLastEnds = Box<dyn Fn(&str) + Send + Sync>;

fn when_the_last_ends() -> &'static OnceLock<WhenTheLastEnds> {
    static HELD: OnceLock<WhenTheLastEnds> = OnceLock::new();
    &HELD
}

/// **The application says what follows the last transfer with a server
/// ending**: an owed swap is tried (`Server Setup` R1b). Set once; a second
/// setting is ignored.
pub fn when_the_last_transfer_with_a_server_ends(then: WhenTheLastEnds) {
    let _ = when_the_last_ends().set(then);
}

fn the_last_transfer_ended(server: &str) {
    if let Some(then) = when_the_last_ends().get() {
        then(server);
    }
}

/// **Whether a transfer is under way between this desktop and the machine** —
/// `Server Setup` R1b's half of the live set that is not the machine's.
pub fn a_transfer_is_under_way_with(server: &str) -> bool {
    the_transfers()
        .lock()
        .map(|transfers| transfers.is_under_way(server))
        .unwrap_or(false)
}

/// An upload's or a manual download's run took its request
/// (`crate::downloading::TheRunsTransfers::a_request_is_under_way`).
pub fn a_transfer_begins(server: &str) {
    if let Ok(mut transfers) = the_transfers().lock() {
        transfers.a_request_begins(server);
    }
}

/// That run let it go; where it was the last transfer with the server, an
/// owed swap is tried.
pub fn a_transfer_ends(server: &str) {
    let the_last = the_transfers()
        .lock()
        .map(|mut transfers| transfers.a_request_ends(server))
        .unwrap_or(false);
    if the_last {
        the_last_transfer_ended(server);
    }
}

/// **The automatic downloads of a server owe a file, or no longer do**, read
/// after each pass's fetches (`crate::servers::a_period`); where that was the
/// last transfer with the server ending, an owed swap is tried.
pub fn the_automatic_downloads_owe(server: &str, they_owe: bool) {
    let the_last = the_transfers()
        .lock()
        .map(|mut transfers| transfers.the_automatic_downloads_owe(server, they_owe))
        .unwrap_or(false);
    if the_last {
        the_last_transfer_ended(server);
    }
}

// ════════════════════════════════════════════════════════════════════════════
// Starting an owed swap
// ════════════════════════════════════════════════════════════════════════════

/// **Whether an owed swap starts now, and with which Blenders** — the whole
/// judgement, with the readings handed in so it is driven at the desk.
///
/// It starts where all of these stand: a swap is owed this period; `Server
/// Setup` E2's ask was not declined for it this period; no setup run of this
/// copy's stands on the server (`Server Setup` Y9); no transfer is under way
/// with it; and the queue reading shows nothing live. **A reading that could
/// not be had is not an empty queue**, so nothing starts on one.
pub fn the_owed_swap_starts_with(
    server: &str,
    a_run_is_driven_there: bool,
    reading: Option<&TheMachinesWorkingState>,
) -> Option<Vec<Version>> {
    if a_run_is_driven_there || the_ask_was_declined_on(server) || a_transfer_is_under_way_with(server) {
        return None;
    }
    let owed = the_blenders_the_swap_is_owed_for(server)?;
    match reading {
        Some(reading) if !anything_is_live_in(reading) => Some(owed),
        _ => None,
    }
}

/// **`Server Setup` R1b: who starts a waiting swap, and when — the desktop,
/// and only while connected**, at the queue reading that shows the last of
/// the machine's live work ended (`State, identity and recovery` F15's third
/// moment, where `Desktop app and delivery` V12 tries a waiting update again),
/// or at the end of the last transfer (F15's first), whichever comes later —
/// and at once, where a comparison concluded a swap is owed and nothing is
/// live. **Nothing polls and nothing is timed.**
///
/// `reading` is the queue reading in hand, where the caller has one; where it
/// has none, one is taken now. **The run itself goes off this thread**, and
/// `Server Setup` Y9's exclusion is taken before it does, so two moments
/// meeting at once start one run.
pub fn an_owed_swap_is_tried<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    server: &str,
    reading: Option<&TheMachinesWorkingState>,
) {
    if the_blenders_the_swap_is_owed_for(server).is_none() {
        return;
    }
    let a_run_is_driven = crate::setting_up::the_kind_being_driven_on(server).is_some();
    let taken;
    let reading = match reading {
        Some(reading) => Some(reading),
        None => {
            taken = the_queue_reading_now(app, server);
            taken.as_ref()
        }
    };
    let Some(found) = the_owed_swap_starts_with(server, a_run_is_driven, reading) else {
        return;
    };
    the_swap_run_is_started(app.clone(), server.to_string(), found);
}

/// **`Queue & History` A1's reading, taken now over the server's own point**,
/// at the package area the row records — or nothing where no connection or no
/// placement stands.
fn the_queue_reading_now<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    server: &str,
) -> Option<TheMachinesWorkingState> {
    let mut machine = the_machine_now(app, server)?;
    crate::servers::the_machines_work::the_queue_reading_from(&mut machine).ok()
}

/// **The server's own point, asked through now** — or nothing where no
/// connection or no placement stands.
fn the_machine_now<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    server: &str,
) -> Option<crate::servers::the_machines_work::OverThisServersPoint> {
    use tauri::Manager;
    let sessions = app
        .try_state::<std::sync::Arc<crate::servers::a_period::ThePeriodsThisRun>>()?
        .the_sessions_of(server)
        .filter(crate::servers::sessions::ThisServersSessions::a_connection_is_live)?;
    let started = app.try_state::<Mutex<crate::start::Started>>()?;
    let row = {
        let started = started.lock().ok()?;
        crate::record::server_rows::one(started.record.as_ref()?.connection(), server)
            .ok()
            .flatten()?
    };
    let crate::servers::installation::WhereThePackageAreaIs::At(package_area) =
        crate::servers::installation::where_the_package_area_is(&row)
    else {
        return None;
    };
    Some(crate::servers::the_machines_work::OverThisServersPoint {
        sessions,
        package_area,
    })
}

/// **The swap's own run, begun under `Server Setup` Y9's exclusion and driven
/// off this thread** — the shape the add's own setup takes
/// (`crate::setting_up::the_remembered_setup_is_carried_out`), which is also a
/// run nobody pressed for. **Its three owed things** (the fourth piece's account): it
/// registers under the ready action's own kind, which the card names; the Set
/// up tab is told it is under way, so the card reads *restarting* at its
/// restart; and it takes `Server Setup` T4's mark the way the ready action
/// does ([`crate::setting_up::the_swap_run_on`]).
///
/// **`Server Setup` R1b, as written 2026-11-03: the machine is read before the
/// exclusion is taken**, because taking it is what puts *Setting up* on the card.
/// The Sesterce pass's card flipped for seven minutes between that and a refusal,
/// one attempt after another, over a setup that had stopped part-way.
fn the_swap_run_is_started<R: tauri::Runtime>(app: tauri::AppHandle<R>, server: String, found: Vec<Version>) {
    let spawned = std::thread::Builder::new()
        .name("smoozy driver swap".to_string())
        .spawn({
            let (app, server) = (app.clone(), server.clone());
            move || {
                use tauri::Manager;
                if !a_swap_may_begin_on_the_machine(&app, &server) {
                    the_comparison_concluded(&server, None);
                    return;
                }
                // **Nobody pressed for it, so the setup's words stay** (`Server
                // Setup` Y7a as changed 2026-10-30, the twenty-third notes,
                // note 3).
                if !crate::setting_up::a_run_nobody_pressed_for_begins(&server, smoozy_server_setup::run::SetupKind::TheReadyAction) {
                    return;
                }
                crate::surface::set_up::the_run_under_way_is_told(&app, &server);
                let point = app
                    .try_state::<std::sync::Arc<crate::servers::a_period::ThePeriodsThisRun>>()
                    .and_then(|periods| periods.the_sessions_of(&server));
                let came_to = point.map(|point| crate::setting_up::the_swap_run_on(&app, &point, &server, found));
                crate::setting_up::a_run_ends(&server);
                match came_to {
                    Some(came_to) if !the_machine_refused_it_for_a_stopped_setup(&came_to) => {
                        crate::surface::set_up::the_swap_runs_ending_is_told(&app, &server, &came_to)
                    }
                    // **A refusal that arrives all the same says no second
                    // ending**: the card reads the machine again.
                    _ => crate::surface::set_up::the_run_under_way_is_told(&app, &server),
                }
            }
        });
    if spawned.is_err() {
        crate::surface::set_up::the_run_under_way_is_told(&app, &server);
    }
}

/// **Whether the swap's own run was refused because a stopped setup stands.**
fn the_machine_refused_it_for_a_stopped_setup(came_to: &WhereTheSetUpGot) -> bool {
    matches!(
        came_to,
        WhereTheSetUpGot::TheRunEnded {
            ending: TheRunsEnding::ASetupThatStoppedPartWayStands { .. },
            ..
        }
    )
}

/// **`Server Setup` R1b's two readings before a swap's run begins, taken over the
/// server's own point.** No point and no placement is no run, as the start below
/// it has always read them.
fn a_swap_may_begin_on_the_machine<R: tauri::Runtime>(app: &tauri::AppHandle<R>, server: &str) -> bool {
    let Some(machine) = the_machine_now(app, server) else {
        return false;
    };
    let Ok(runtime) = tokio::runtime::Builder::new_current_thread().enable_all().build() else {
        return false;
    };
    let drove = crate::setting_up::the_runs_this_copy_drove_on_the_confirmed_installation(app, server);
    runtime.block_on(crate::setting_up::a_swap_may_begin(&machine.sessions, &machine.package_area, &drove))
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_queue_history::readings::{AJobAsTheMachineReadsIt, WhatTheEntryHasLive};
    use smoozy_servers::hardware::DriverVersion;
    use smoozy_state::identities::{Identity, IdentityKind};

    fn v(text: &str) -> Version {
        Version::parse(text).expect("a version")
    }

    fn a_driver(version: &str, placed: WhereK1PlacesIt) -> TheDriversOwnAnswer {
        TheDriversOwnAnswer {
            version: DriverVersion::parse(version).expect("a driver version"),
            cuda_version: None,
            where_k1_placed_the_machine: placed,
        }
    }

    fn a_reading(states: Vec<QueueState>) -> TheMachinesWorkingState {
        TheMachinesWorkingState {
            jobs: states
                .into_iter()
                .map(|state| AJobAsTheMachineReadsIt {
                    job: Identity::issue(IdentityKind::Job).expect("a job"),
                    state,
                    live: WhatTheEntryHasLive::NoReadings,
                    the_hold: None,
                })
                .collect(),
            the_admission_point_stands_at: None,
            notes: Vec::new(),
            the_history_copys_count: 0,
            the_history_copys_entries: Vec::new(),
        }
    }

    fn an_operation() -> Identity {
        Identity::issue(IdentityKind::Operation).expect("an operation")
    }

    fn a_server() -> String {
        Identity::issue(IdentityKind::LocalServer).expect("a server").as_text()
    }

    /// **`Server Setup` R1b's comparison at a search moment**: a 550 driver on
    /// a machine of its own kernel, with 5.2.2 among the builds found, owes a
    /// swap — and the Blenders it was read over are what the swap's run takes.
    /// Not on a container, not on a reading of an earlier connection, not
    /// while a miss stands, and not where every Blender's bar is met.
    #[test]
    fn the_comparison_at_a_search_moment_owes_a_swap_below_the_newest_bar() {
        let found = vec![v("4.5.3"), v("5.2.2")];
        let own = a_driver("550.163.01", WhereK1PlacesIt::ItsOwnKernel);
        assert_eq!(the_comparison_on(Some(&own), false, &found), Some(found.clone()));
        let container = a_driver("550.163.01", WhereK1PlacesIt::AContainer);
        assert_eq!(the_comparison_on(Some(&container), false, &found), None);
        assert_eq!(the_comparison_on(None, false, &found), None);
        assert_eq!(the_comparison_on(Some(&own), true, &found), None);
        assert_eq!(the_comparison_on(Some(&own), false, &[v("4.5.3")]), None);
        let at = a_driver("580.178.04", WhereK1PlacesIt::ItsOwnKernel);
        assert_eq!(the_comparison_on(Some(&at), false, &found), None);
    }

    /// **`Server Setup` R1b, as written 2026-11-03: the machine is read before
    /// the exclusion is taken**, so a swap that cannot begin never puts *Setting
    /// up* on the card, and what it found ends what the comparison concluded.
    #[test]
    fn the_machine_is_read_before_a_swaps_run_takes_the_exclusion() {
        let working = include_str!("the_driver_swap.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("the working half");
        let started = working
            .split("fn the_swap_run_is_started<R: tauri::Runtime>(")
            .nth(1)
            .and_then(|rest| rest.split("\n}\n").next())
            .expect("the start");
        let read = started.find("a_swap_may_begin_on_the_machine(&app, &server)").expect("the reading");
        let forgotten = started.find("the_comparison_concluded(&server, None)").expect("it is forgotten");
        let begins = started.find("a_run_nobody_pressed_for_begins").expect("the exclusion");
        assert!(read < forgotten && forgotten < begins, "{read} {forgotten} {begins}");
    }

    /// **The refusal that still arrives is told as nothing**, so no line about a
    /// second ending stands on the card over the first.
    #[test]
    fn a_refusal_for_a_stopped_setup_is_no_ending_of_its_own() {
        let refused = WhereTheSetUpGot::TheRunEnded {
            at: smoozy_server_setup::run::Boundary::Started,
            ending: TheRunsEnding::ASetupThatStoppedPartWayStands {
                standing: an_operation(),
                said: "a setup stands".to_string(),
            },
        };
        assert!(the_machine_refused_it_for_a_stopped_setup(&refused));
        let failed = WhereTheSetUpGot::TheRunEnded {
            at: smoozy_server_setup::run::Boundary::Started,
            ending: TheRunsEnding::AStepFailed {
                step: smoozy_server_setup::ready_action::Step::Driver,
                said: "no".to_string(),
            },
        };
        assert!(!the_machine_refused_it_for_a_stopped_setup(&failed));
    }

    /// **The live set is `Queue & History` §17's three states**, and an ended
    /// job is not live.
    #[test]
    fn the_live_set_is_waiting_held_and_rendering() {
        assert!(!anything_is_live_in(&a_reading(Vec::new())));
        assert!(!anything_is_live_in(&a_reading(vec![QueueState::Ended { outcome: an_operation() }])));
        assert!(anything_is_live_in(&a_reading(vec![QueueState::Waiting])));
        assert!(anything_is_live_in(&a_reading(vec![QueueState::Held { hold: an_operation() }])));
        assert!(anything_is_live_in(&a_reading(vec![QueueState::Rendering {
            operation: an_operation()
        }])));
        // **`Server Setup` R1c: a job held for the driver is left out, and no
        // other hold is** — the check's hold, and a hold a reading from an
        // older program says nothing of, both count. Held-job piece A.
        assert!(!anything_is_live_in(&a_reading_holding(&[Some(smoozy_rendering::submission::THE_DRIVER)])));
        assert!(anything_is_live_in(&a_reading_holding(&[Some("the first-frame check")])));
        assert!(anything_is_live_in(&a_reading_holding(&[None])));
        assert!(anything_is_live_in(&a_reading_holding(&[
            Some(smoozy_rendering::submission::THE_DRIVER),
            Some("the first-frame check"),
        ])));
    }

    /// **A reading of jobs each held under a hold of its own**, with the
    /// reason given — or none, as a program older than step 10c's
    /// eleventh-pass fix piece 8b gives.
    fn a_reading_holding(reasons: &[Option<&str>]) -> TheMachinesWorkingState {
        let mut reading = a_reading(reasons.iter().map(|_| QueueState::Held { hold: an_operation() }).collect());
        for (job, reason) in reading.jobs.iter_mut().zip(reasons) {
            job.the_hold = reason.map(|reason| smoozy_queue_history::readings::WhatTheHoldSays {
                reason: reason.to_string(),
                held_behind: None,
            });
        }
        reading
    }

    /// **`Server Setup` R1c with `Server Setup` R1b: an owed swap starts with only jobs held
    /// for the driver standing**, and not while another hold stands beside
    /// them. Held-job piece A.
    #[test]
    fn an_owed_swap_starts_with_only_driver_holds_standing() {
        let server = a_server();
        the_comparison_concluded(&server, Some(vec![v("5.2.2")]));
        let driver = smoozy_rendering::submission::THE_DRIVER;
        assert_eq!(
            the_owed_swap_starts_with(&server, false, Some(&a_reading_holding(&[Some(driver), Some(driver)]))),
            Some(vec![v("5.2.2")])
        );
        assert_eq!(
            the_owed_swap_starts_with(
                &server,
                false,
                Some(&a_reading_holding(&[Some(driver), Some("the first-frame check")]))
            ),
            None
        );
        the_period_ended(&server);
    }

    /// **`Server Setup` R1c at submission, the desktop's half**: the device must be the
    /// cards, the make NVIDIA, the program this application's own — each
    /// failing once sends the job plain. The four of `Server Setup` R1c are
    /// `Server Setup`'s own test. Held-job piece A.
    #[test]
    fn a_job_is_held_only_on_the_cards_of_an_nvidia_machine_whose_program_is_ours() {
        use smoozy_job_files::manifest::TheCardMakeAtSubmission;
        use smoozy_render_settings::device::DeviceChoice;
        let at = TheDriverAtSubmission {
            the_driver_on_this_connection: Some(a_driver("550.163.01", WhereK1PlacesIt::ItsOwnKernel)),
            a_missed_swap_stands: false,
            a_swap_is_owed: true,
            the_program_is_this_applications_own: true,
        };
        let nvidia = TheCardMakeAtSubmission::TheMakes(vec![smoozy_servers::hardware::NVIDIA.to_string()]);
        assert!(at.holds_this_job(Some(DeviceChoice::TheCards), TheEngineAtSubmission::Cycles, &nvidia, Some(v("5.2.2"))));
        assert!(!at.holds_this_job(Some(DeviceChoice::TheProcessor), TheEngineAtSubmission::Cycles, &nvidia, Some(v("5.2.2"))));
        assert!(!at.holds_this_job(None, TheEngineAtSubmission::Cycles, &nvidia, Some(v("5.2.2"))), "a device not known at submission");
        // **`Server Setup` R1c's bullet of 2026-11-02.**
        assert!(!at.holds_this_job(Some(DeviceChoice::TheCards), TheEngineAtSubmission::AnotherEngine, &nvidia, Some(v("5.2.2"))));
        assert!(at.holds_this_job(Some(DeviceChoice::TheCards), TheEngineAtSubmission::NotKnown, &nvidia, Some(v("5.2.2"))), "the device rule stands");
        assert!(!at.holds_this_job(None, TheEngineAtSubmission::NotKnown, &nvidia, Some(v("5.2.2"))), "the device rule stands");
        assert!(!at.holds_this_job(
            Some(DeviceChoice::TheCards),
            TheEngineAtSubmission::Cycles,
            &TheCardMakeAtSubmission::TheMakes(vec!["AMD".to_string()]),
            Some(v("5.2.2"))
        ));
        assert!(!at.holds_this_job(Some(DeviceChoice::TheCards), TheEngineAtSubmission::Cycles, &TheCardMakeAtSubmission::NoCard, Some(v("5.2.2"))));
        let older = TheDriverAtSubmission {
            the_program_is_this_applications_own: false,
            ..at.clone()
        };
        assert!(!older.holds_this_job(Some(DeviceChoice::TheCards), TheEngineAtSubmission::Cycles, &nvidia, Some(v("5.2.2"))));
        assert!(!TheDriverAtSubmission::default().holds_this_job(Some(DeviceChoice::TheCards), TheEngineAtSubmission::Cycles, &nvidia, Some(v("5.2.2"))));
    }

    /// **`Server Setup` R1c's release bullets, judged** — every ending a run
    /// can come to. Held-job piece A.
    #[test]
    fn a_runs_ending_releases_as_r1c_says() {
        use smoozy_server_setup::ready_action::Step;
        use smoozy_server_setup::run::Boundary;
        let acted = |below: Option<bool>| WhatItem5CameTo {
            it_compared_and_acted: true,
            the_drivers_version: Some("550.163.01".to_string()),
            the_last_check_found_it_below: below,
        };
        let ended = |ending| WhereTheSetUpGot::TheRunEnded { at: Boundary::Done, ending };
        let done = |missed: bool, the_swap_waits: bool| {
            ended(TheRunsEnding::ItReachedDone {
                a_blender_was_published: false,
                the_driver_missed_the_optix_bar: missed,
                the_swap_waits,
            })
        };
        let every = |the_mark: Option<Option<String>>| WhatTheEndingReleases::EveryDriverHold { the_mark };
        let marked = Some(Some("550.163.01".to_string()));

        // *Done*: released, marked where it missed.
        assert_eq!(what_the_ending_releases(&done(false, false), &acted(Some(false))), every(None));
        assert_eq!(what_the_ending_releases(&done(true, false), &acted(Some(true))), every(marked.clone()));
        // Backed off at the last guard, and a run whose item 5 never acted.
        assert_eq!(what_the_ending_releases(&done(false, true), &acted(None)), WhatTheEndingReleases::Nothing);
        assert_eq!(
            what_the_ending_releases(&done(false, false), &WhatItem5CameTo::default()),
            WhatTheEndingReleases::Nothing
        );
        // Failed after item 5 acted: released with the mark, unless the last
        // check had read the driver at the bar.
        let failed = ended(TheRunsEnding::AStepFailed { step: Step::Restart, said: "x".into() });
        assert_eq!(what_the_ending_releases(&failed, &acted(None)), every(marked.clone()));
        assert_eq!(what_the_ending_releases(&failed, &acted(Some(true))), every(marked.clone()));
        assert_eq!(what_the_ending_releases(&failed, &acted(Some(false))), every(None));
        let refused = ended(TheRunsEnding::TheRestartWasRefused { said: "x".into() });
        assert_eq!(what_the_ending_releases(&refused, &acted(None)), every(marked.clone()));
        // A declined ask, and every way a run is cut off part-way.
        for nothing in [
            ended(TheRunsEnding::AStepNeedsAdminAccess { step: Step::Driver }),
            ended(TheRunsEnding::TheUserStoppedIt),
            ended(TheRunsEnding::TheMachineIsRestarting),
            ended(TheRunsEnding::TheServerStoppedAnswering { step: Step::Driver, said: "x".into() }),
            ended(TheRunsEnding::TheServerEndedTheRun { step: Step::Driver, said: None }),
            ended(TheRunsEnding::ItStoppedInsideAStep { step: Step::Driver }),
            WhereTheSetUpGot::TheConnectionWent,
        ] {
            assert_eq!(what_the_ending_releases(&nothing, &acted(None)), WhatTheEndingReleases::Nothing, "{nothing:?}");
        }
    }

    /// **What the driving tells of item 5 is the run's own**: a run that
    /// begins forgets what an earlier one told, and the ending takes it once.
    #[test]
    fn what_item_5_came_to_is_told_per_run_and_taken_once() {
        let server = a_server();
        let old = DriverVersion::parse("550.163.01").expect("a version");
        let new = DriverVersion::parse("580.178.04").expect("a version");
        item_5_compared_and_acted(&server, Some(&old));
        a_run_whose_item_5_may_compare_begins(&server);
        assert_eq!(what_item_5_came_to(&server), WhatItem5CameTo::default());

        item_5_compared_and_acted(&server, Some(&old));
        item_5s_last_check_read(&server, Some(&new), false);
        assert_eq!(
            what_item_5_came_to(&server),
            WhatItem5CameTo {
                it_compared_and_acted: true,
                the_drivers_version: Some("580.178.04".to_string()),
                the_last_check_found_it_below: Some(false),
            }
        );
        assert_eq!(what_item_5_came_to(&server), WhatItem5CameTo::default(), "taken once");
    }

    /// **`Server Setup` R1c: a hold placed in one connected period is picked
    /// up in the next.** The period's end forgets the owed swap; the next
    /// connection's queue reading finds the hold standing, compares the
    /// driver against the held job's own Blender, concludes the swap owed
    /// again, and `Server Setup` R1b's start follows at the same reading — the hold itself
    /// counting as nothing live. **After the swap, the next reading releases
    /// it with no mark; after a miss kept on the row, with the mark.** And no
    /// driver reading of the connection does nothing. Held-job piece A.
    #[test]
    fn a_standing_hold_is_picked_up_on_a_later_connection() {
        let server = a_server();
        the_comparison_concluded(&server, Some(vec![v("5.2.2")]));
        the_period_ended(&server);
        assert!(!a_swap_is_owed_on(&server), "the period's end forgot the swap");

        let reading = a_reading_holding(&[Some(smoozy_rendering::submission::THE_DRIVER)]);
        let holds = the_driver_holds_in(&reading);
        assert_eq!(holds.len(), 1);
        let its_blender = |_: &Identity| Some(v("5.2.2"));
        let old = a_driver("550.163.01", WhereK1PlacesIt::ItsOwnKernel);
        let new = a_driver("580.178.04", WhereK1PlacesIt::ItsOwnKernel);

        let came_to = what_the_standing_holds_come_to(&holds, &its_blender, Some(&old), false);
        assert_eq!(came_to, WhatTheStandingHoldsComeTo::ASwapIsOwedFor(vec![v("5.2.2")]));
        if let WhatTheStandingHoldsComeTo::ASwapIsOwedFor(blenders) = came_to {
            the_comparison_concluded(&server, Some(blenders));
        }
        assert_eq!(the_owed_swap_starts_with(&server, false, Some(&reading)), Some(vec![v("5.2.2")]));

        assert_eq!(
            what_the_standing_holds_come_to(&holds, &its_blender, Some(&new), false),
            WhatTheStandingHoldsComeTo::TheyAreReleased {
                marked: HashSet::new(),
                the_drivers_version: Some("580.178.04".to_string()),
            }
        );
        assert_eq!(
            what_the_standing_holds_come_to(&holds, &its_blender, Some(&old), true),
            WhatTheStandingHoldsComeTo::TheyAreReleased {
                marked: [holds[0].job.as_text()].into_iter().collect(),
                the_drivers_version: Some("550.163.01".to_string()),
            }
        );
        assert_eq!(
            what_the_standing_holds_come_to(&holds, &its_blender, None, false),
            WhatTheStandingHoldsComeTo::Nothing
        );
        the_period_ended(&server);
    }

    // ── The release, sent ───────────────────────────────────────────────────

    /// **A machine that answers each release *still in hand* once and then
    /// *released*, and writes down every event in order** — the marks written
    /// beside it, so the order of the two is read off one list.
    struct AMachineThatReleases {
        events: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
        in_hand_once: HashSet<String>,
        /// Holds it never answers with an outcome.
        never: HashSet<String>,
    }

    impl crate::servers::the_machines_work::AsksTheSupervisor for AMachineThatReleases {
        fn ask(&mut self, request: &smoozy_server_protocol::framing::Request) -> smoozy_server_protocol::asking_the_supervisor::WhatCameBack {
            use smoozy_server_protocol::framing::{Answered, Fields, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS};
            use smoozy_server_protocol::requests::{
                THE_ANSWER_FIELD, THE_ANSWER_IS_YES, THE_HOLD_FIELD, THE_RELEASE_OUTCOME_FIELD, THE_ROUTE_FIELD,
                THE_ROUTE_IS_THE_DESK,
            };
            let hold = request.fields.text(THE_HOLD_FIELD).unwrap_or_default().to_string();
            assert_eq!(request.fields.text(THE_ANSWER_FIELD), Some(THE_ANSWER_IS_YES), "the release is a *go on*");
            assert_eq!(request.fields.text(THE_ROUTE_FIELD), Some(THE_ROUTE_IS_THE_DESK));
            let answered = Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "test");
            if self.never.contains(&hold) {
                self.events.borrow_mut().push(format!("asked {hold}, in hand"));
                return smoozy_server_protocol::asking_the_supervisor::WhatCameBack::ItAnswered(
                    answered.carrying(Fields::none()),
                );
            }
            if self.in_hand_once.remove(&hold) {
                self.events.borrow_mut().push(format!("asked {hold}, in hand"));
                return smoozy_server_protocol::asking_the_supervisor::WhatCameBack::ItAnswered(
                    answered.carrying(Fields::none()),
                );
            }
            self.events.borrow_mut().push(format!("asked {hold}, released"));
            smoozy_server_protocol::asking_the_supervisor::WhatCameBack::ItAnswered(
                answered.carrying(Fields::none().and(THE_RELEASE_OUTCOME_FIELD, "released")),
            )
        }
    }

    const QUICK: crate::servers::stop_and_clear::TheWait = crate::servers::stop_and_clear::TheWait {
        deadline: std::time::Duration::from_millis(200),
        pause: std::time::Duration::from_millis(1),
    };

    fn a_driver_hold() -> ADriverHold {
        ADriverHold {
            job: Identity::issue(IdentityKind::Job).expect("a job"),
            hold: an_operation(),
        }
    }

    /// **`Server Setup` R1c's order is submission order**, read off each
    /// job's submitted moment; a job with none goes last.
    #[test]
    fn the_holds_are_put_in_the_order_they_were_submitted_in() {
        let (a, b, c) = (a_driver_hold(), a_driver_hold(), a_driver_hold());
        let at = |seconds| std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(seconds);
        let moments: HashMap<String, std::time::SystemTime> =
            [(a.job.as_text(), at(30)), (c.job.as_text(), at(10))].into_iter().collect();
        let ordered = in_submission_order(vec![a.clone(), b.clone(), c.clone()], &|job| {
            moments.get(&job.as_text()).copied()
        });
        assert_eq!(ordered, vec![c, a, b]);
    }

    /// **`Server Setup` R1c's release after a miss, through the record itself:
    /// each job's `Queue & History` N23 mark is written before its *go on*, each
    /// *go on* waits for the one before to have an outcome read, and the jobs
    /// go in the order handed in.** Held-job piece A.
    #[test]
    fn after_a_miss_each_job_is_marked_then_released_one_after_another() {
        use crate::record::job_records::JobRecords;
        let connection = crate::record::deletions::tests::a_record_store();
        let records = JobRecords::over(&connection);
        let server = Identity::issue(IdentityKind::LocalServer).expect("a server");
        let holds: Vec<ADriverHold> = (0..3)
            .map(|_| {
                let record = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen");
                records.write_the_provisional_record(&record).expect("written");
                ADriverHold {
                    job: record.job,
                    hold: an_operation(),
                }
            })
            .collect();
        let events = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let mut machine = AMachineThatReleases {
            events: std::rc::Rc::clone(&events),
            in_hand_once: holds.iter().map(|held| held.hold.as_text()).collect(),
            never: HashSet::new(),
        };
        let mut write = |job: &Identity| {
            events.borrow_mut().push(format!("marked {}", job.as_text()));
            records.the_driver_mark_is_written(job, Some("550.163.01")).unwrap_or(false)
        };
        let sent = the_driver_holds_are_released(&mut machine, &holds, Some(&mut write), QUICK);
        assert_eq!(sent.len(), 3);
        let mut expected = Vec::new();
        for held in &holds {
            expected.push(format!("marked {}", held.job.as_text()));
            expected.push(format!("asked {}, in hand", held.hold.as_text()));
            expected.push(format!("asked {}, released", held.hold.as_text()));
        }
        assert_eq!(*events.borrow(), expected, "one at a time, the mark first, the outcome read before the next");
        for held in &holds {
            let mark = records.read(&held.job).unwrap().unwrap().the_driver_was_below_the_bar;
            assert_eq!(mark.and_then(|mark| mark.the_drivers_version).as_deref(), Some("550.163.01"));
        }
    }

    /// **No release where `Queue & History` N23's write is refused** — the job stays held, and so
    /// does every job after it, so none joins the line ahead of it. **And
    /// none after one whose outcome was never read.** Held-job piece A.
    #[test]
    fn no_release_where_the_mark_is_refused_or_an_outcome_is_not_read() {
        use crate::record::job_records::JobRecords;
        let connection = crate::record::deletions::tests::a_record_store();
        let records = JobRecords::over(&connection);
        let server = Identity::issue(IdentityKind::LocalServer).expect("a server");
        let standing = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen");
        records.write_the_provisional_record(&standing).expect("written");
        // **The first job has no local record**, so its mark cannot land.
        let holds = vec![
            a_driver_hold(),
            ADriverHold {
                job: standing.job.clone(),
                hold: an_operation(),
            },
        ];
        let events = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let mut machine = AMachineThatReleases {
            events: std::rc::Rc::clone(&events),
            in_hand_once: HashSet::new(),
            never: HashSet::new(),
        };
        let mut write = |job: &Identity| records.the_driver_mark_is_written(job, None).unwrap_or(false);
        let sent = the_driver_holds_are_released(&mut machine, &holds, Some(&mut write), QUICK);
        assert!(sent.is_empty(), "a release went out with no mark: {sent:?}");
        assert!(events.borrow().is_empty());
        assert_eq!(records.read(&standing.job).unwrap().unwrap().the_driver_was_below_the_bar, None);

        // **With no mark owed**, the first goes; its outcome is never read,
        // and the second is not sent.
        let held = vec![a_driver_hold(), a_driver_hold()];
        let mut machine = AMachineThatReleases {
            events: std::rc::Rc::clone(&events),
            in_hand_once: HashSet::new(),
            never: [held[0].hold.as_text()].into_iter().collect(),
        };
        let sent = the_driver_holds_are_released(&mut machine, &held, None, QUICK);
        assert_eq!(sent.len(), 1);
        assert_eq!(sent[0].1, crate::servers::the_machines_work::WhatTheAnswerCameTo::TheWaiterGaveUp);
        assert!(!events.borrow().iter().any(|event| event.contains(&held[1].hold.as_text())));
    }

    /// **`State, identity and recovery` F15's first moment: the last transfer
    /// with a server ending is told once**, whether the last was a request or
    /// the automatic downloads, and not while another still moves.
    #[test]
    fn the_last_transfer_with_a_server_ending_is_told_once() {
        let mut transfers = TheTransfersWithEachServer::default();
        assert!(!transfers.is_under_way("box"));
        transfers.a_request_begins("box");
        transfers.a_request_begins("box");
        assert!(transfers.is_under_way("box"));
        assert!(!transfers.a_request_ends("box"), "one still moves");
        assert!(!transfers.the_automatic_downloads_owe("box", true));
        assert!(!transfers.a_request_ends("box"), "the downloads still owe a file");
        assert!(transfers.is_under_way("box"));
        assert!(transfers.the_automatic_downloads_owe("box", false), "that was the last");
        assert!(!transfers.is_under_way("box"));
        assert!(!transfers.the_automatic_downloads_owe("box", false), "told once");
        // **Another server's transfers are its own.**
        transfers.a_request_begins("other");
        assert!(!transfers.is_under_way("box"));
        // **A period's end takes the automatic downloads with it**, without
        // telling: an owed swap is lost with the period anyway.
        transfers.the_automatic_downloads_owe("box", true);
        transfers.a_period_ended("box");
        assert!(!transfers.is_under_way("box"));
    }

    /// **`State, identity and recovery` F15's first moment, as the transfers
    /// report it: the run's register of uploads and manual downloads tells this
    /// file when a request ends**, and the last one with a server reaches what
    /// the application set — once, and not while another still moves.
    #[test]
    fn the_end_of_the_last_transfer_with_a_server_is_told() {
        static TOLD: Mutex<Vec<String>> = Mutex::new(Vec::new());
        when_the_last_transfer_with_a_server_ends(Box::new(|server: &str| {
            TOLD.lock().expect("the told").push(server.to_string());
        }));
        let told_of = |server: &Identity| {
            TOLD.lock()
                .expect("the told")
                .iter()
                .filter(|told| **told == server.as_text())
                .count()
        };
        let server = Identity::issue(IdentityKind::LocalServer).expect("a server");
        let the_run = crate::downloading::TheRunsTransfers::for_this_run();
        let one = Identity::issue(IdentityKind::Operation).expect("a request");
        let two = Identity::issue(IdentityKind::Operation).expect("a request");
        let first = the_run.a_request_is_under_way(&one, &server);
        let second = the_run.a_request_is_under_way(&two, &server);
        assert!(a_transfer_is_under_way_with(&server.as_text()));
        drop(first);
        assert_eq!(told_of(&server), 0, "one still moves");
        drop(second);
        assert_eq!(told_of(&server), 1);
        assert!(!a_transfer_is_under_way_with(&server.as_text()));
    }

    /// **An owed swap starts at the empty-queue reading, and not before**: not
    /// while the reading shows work, not on no reading, not while a transfer
    /// is under way, not while another run is driven there, not after a
    /// declined ask — and only once owed.
    #[test]
    fn an_owed_swap_starts_at_the_empty_queue_reading_and_nowhere_else() {
        let server = a_server();
        let empty = a_reading(vec![QueueState::Ended { outcome: an_operation() }]);
        let busy = a_reading(vec![QueueState::Rendering { operation: an_operation() }]);
        assert_eq!(the_owed_swap_starts_with(&server, false, Some(&empty)), None, "nothing is owed");

        the_comparison_concluded(&server, Some(vec![v("5.2.2")]));
        assert!(a_swap_is_owed_on(&server));
        assert_eq!(the_owed_swap_starts_with(&server, false, Some(&busy)), None);
        assert_eq!(the_owed_swap_starts_with(&server, false, None), None);
        assert_eq!(the_owed_swap_starts_with(&server, true, Some(&empty)), None);
        a_transfer_begins(&server);
        assert_eq!(the_owed_swap_starts_with(&server, false, Some(&empty)), None);
        a_transfer_ends(&server);
        assert_eq!(the_owed_swap_starts_with(&server, false, Some(&empty)), Some(vec![v("5.2.2")]));

        // **A later comparison replaces it.**
        the_comparison_concluded(&server, None);
        assert_eq!(the_owed_swap_starts_with(&server, false, Some(&empty)), None);

        // **The period's end forgets it.**
        the_comparison_concluded(&server, Some(vec![v("5.2.2")]));
        the_period_ended(&server);
        assert!(!a_swap_is_owed_on(&server));
    }

    /// **What a run whose item 5 compared leaves for the period**: owed where
    /// the swap waited, with the Blenders its bar was taken over and the one it
    /// staged; nothing owed after any other *done*; a declined ask stops the
    /// swap being asked for again; and a run cut off leaves what stood.
    #[test]
    fn a_runs_ending_is_kept_for_the_period() {
        use smoozy_server_setup::ready_action::Step;
        use smoozy_server_setup::run::Boundary;
        let server = a_server();
        let asked_for = WhatWasAskedFor {
            install_this_blender: Some("4.5.3".to_string()),
            the_blenders_on_the_server: vec![v("5.2.2")],
            ..WhatWasAskedFor::default()
        };
        let done = |the_swap_waits: bool| WhereTheSetUpGot::TheRunEnded {
            at: Boundary::Done,
            ending: TheRunsEnding::ItReachedDone {
                a_blender_was_published: false,
                the_driver_missed_the_optix_bar: false,
                the_swap_waits,
            },
        };
        the_ending_is_kept(&server, &done(true), &asked_for);
        assert_eq!(the_blenders_the_swap_is_owed_for(&server), Some(vec![v("5.2.2"), v("4.5.3")]));
        the_ending_is_kept(&server, &WhereTheSetUpGot::TheConnectionWent, &asked_for);
        assert!(a_swap_is_owed_on(&server), "a run cut off leaves what stood");
        the_ending_is_kept(&server, &done(false), &asked_for);
        assert!(!a_swap_is_owed_on(&server));

        the_comparison_concluded(&server, Some(vec![v("5.2.2")]));
        the_ending_is_kept(
            &server,
            &WhereTheSetUpGot::TheRunEnded {
                at: Boundary::Started,
                ending: TheRunsEnding::AStepNeedsAdminAccess { step: Step::Driver },
            },
            &asked_for,
        );
        assert!(the_ask_was_declined_on(&server));
        let empty = a_reading(Vec::new());
        assert_eq!(the_owed_swap_starts_with(&server, false, Some(&empty)), None);
        the_period_ended(&server);
        assert!(!the_ask_was_declined_on(&server));
    }
}
