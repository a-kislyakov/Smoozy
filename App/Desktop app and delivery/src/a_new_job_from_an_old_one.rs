//! **A new job from an old one** — the route from a history entry to the
//! Render Settings page: *Submit again* and *Resume* (`Rendering` §6), the
//! pick-a-file browser a re-submission opens where its `.blend` is not where
//! the entry says (`Rendering` A1a, `Rendering` A10, `Rendering` A11, `File Browser` §12), and the
//! setup that waits for the pick (`Render Settings` Y15's no-key route). The
//! 38's piece 2, items 2 and 35 of
//! `Audits/Build/Step 10c/What the plans promise and nothing builds.md`.
//!
//! **Everything this file decides is decided elsewhere and called here.** The
//! fresh act's identities, the offer's condition, the non-load, the carry-on
//! arithmetic and the browser's opening rule are `App/Rendering/src/again.rs`;
//! `Rendering` A4's reading over the machine is `Rendering`'s own
//! `acting::page_readings::the_carry_ons_figure`, given its two readings here
//! over the server's own point; the load into the store and the held setup
//! are `Render Settings`' `Store::load_a_kept_setup` and `HeldSetup`; the
//! kept setup is the record's (`Queue & History` N10, record version 25).
//! **What is here is the joining**, and the run-held pick that waits for a
//! file.
//!
//! **The press runs off the drawing thread** (P5), because `Rendering` A4 and `Rendering` A10 read
//! the machine, and what it leads to is posted on the one queue as
//! [`A_NEW_JOB_OPENS`] — the page goes to the server, its Files tab or the
//! server list — or answered on the control that was pressed.
//!
//! # Three rulings this file carries, each the map's lean
//!
//! - **Hole 3 of the map: where several servers are connected and none
//!   holds the entry's job, the browser opens with no machine chosen** — the
//!   server list, where the user picks one, and that server's Files shows the
//!   bar. `Rendering` §9 item 24 left the opening machine to be invented, and
//!   nothing is invented. *The machine being browsed this run* has no
//!   run-held home on the desktop (`File Browser` N1), so `Rendering` A1a's second case is
//!   never met and the count decides between its other three.
//! - **Hole 4: Simplify reaches the launch.** Proved at the desk before this
//!   file was written, by
//!   `smoozy_rendering::submission::tests::simplify_loaded_from_a_failed_entry_reaches_the_launch_material_and_no_other_load_does`.
//! - **`Rendering` A10's reading is the file-transfer route's single attribute read**,
//!   `File Browser` P1's distinct statuses in one request — *no such file*
//!   for gone, *permission denied* for may not read — rather than the
//!   command channel's stat, which needs the reading session held across a
//!   command and the server's own point does not lend it that way.
//!
//! **A bare marker below means a clause of this crate's own plan,
//! `Preparation/Architecture/Desktop app and delivery.md`.** Every other is
//! written with its document.

use std::sync::{Mutex, OnceLock};

use smoozy_queue_history::record::{AnswerOrigin, LocalJobRecord};
use smoozy_render_settings::file_half::{BlenderIdentity, FileKey};
use smoozy_render_settings::frames::FrameForm;
use smoozy_render_settings::payload::Payload;
use smoozy_render_settings::store::{HeldSetup, SceneEntries, Where};
use smoozy_rendering::again::{
    a_kept_setup_with_no_scene_is_loaded, the_carried_on_range, the_entries_the_payload_kept,
    the_kept_setup_from, the_simplify_offer_is_drawn, where_the_browser_opens,
    TheCarryOnsFigure, WhereTheBrowserOpens,
};
use smoozy_state::identities::{Identity, IdentityKind};
use tauri::Manager;

use crate::events::{Event, Said, Subject as About, TheOneQueue};
use crate::surface::render_settings::{ThePageCameFrom, WhatTheRunHolds, WhereItCameFrom};
use crate::typography::Verbatim;

/// **The one kind the history page listens for**, declared beside the
/// service that posts it. `web/the-history.js` holds the same string.
pub const A_NEW_JOB_OPENS: &str = "history:a-new-job-opens";

/// The words this route says on the control that was pressed. Rule 9: each
/// says one thing, in standard words.
pub(crate) mod words {
    /// The record holds no setup for the entry — a job submitted before record
    /// version 25, or a column that will not read back.
    pub const NO_SETTINGS_WERE_KEPT: &str = "This job's settings were not saved.";
    pub const THE_RECORD_WOULD_NOT_READ: &str = "Could not read Smoozy's database.";
    /// `Rendering` A10's refusal, said as itself.
    pub const THE_BLEND_MAY_NOT_BE_READ: &str = "No permission to read this job's .blend on the server.";
    /// `Rendering` A7's *the machine did not answer*.
    pub const THE_SERVER_DID_NOT_ANSWER: &str = "Could not reach the server.";
    /// `Rendering` A1a's *no machine is connected at all*, where the entry's server is
    /// gone: the setup waits, and the bar is there once one is connected.
    pub const CONNECT_A_SERVER_TO_FIND_THE_BLEND: &str =
        "No server is connected. Connect one to find this job's .blend.";
    /// `Rendering` A5: a typed figure that leaves no frame of the job's own.
    pub const TYPE_A_FRAME_FROM_THE_RANGE: &str = "Type a frame from this job's range.";
}

/// **The two acts on an entry that make a new job** (`Design/Queue &
/// History § The two ways to render it again`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheWayIn {
    /// `Rendering` A1.
    SubmitAgain,
    /// `Rendering` A4, with the figure the entry's field held when it was
    /// pressed — the user's to move (`Rendering` A5).
    CarryOn { typed: Option<String> },
}

/// **Where the press leads**, which is what the page is told.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhereThePressLeads {
    /// The Render Settings page opens over this server on this file.
    ThePageOpens { server: String, file: String },
    /// The server's Files tab opens, with the bar across the top.
    TheBrowserOpens { server: String },
    /// Hole 3: the server list, with the pick held for whichever is opened.
    TheServersOpen,
    /// Nothing opens; the control says why.
    ItIsSaid(&'static str),
}

/// **The setup an entry kept, ready to load** — `Render Settings` Y15's
/// list as `Render Settings` Y15a conditions it. A scene-named job holds `Render Settings`'
/// own [`HeldSetup`]; a job that named no scene holds its entries for the
/// pre-read set.
#[derive(Debug, Clone, PartialEq)]
pub enum TheSetupHeld {
    AScene(HeldSetup),
    NoScene {
        entries: SceneEntries,
        the_users_override: Option<BlenderIdentity>,
        the_offer_is_drawn: bool,
    },
}

/// **Everything the press composed**, before anything is written.
#[derive(Debug, Clone, PartialEq)]
pub struct ThePressComposed {
    pub leads: WhereThePressLeads,
    pub the_setup: TheSetupHeld,
    /// **`Rendering` A4's range**, on a carry-on.
    pub the_range: Option<FrameForm>,
    pub came_from: ThePageCameFrom,
    /// **The bar the browser carries until the pick**, where one is opened.
    pub the_bar: Option<TheBar>,
}

/// Which of `File Browser` §12's two bars the browser carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheBar {
    /// `Rendering` A11: the file is gone from a machine still connected.
    Pick,
    /// `Rendering` A1a: the entry's own machine is gone (`res-find`).
    Find,
}

/// **What the entry's own server is, at the press** — `Queue & History`
/// N17's draw-time read of the copied identifier, and the period's own.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheServerIs {
    Connected,
    NotConnected,
    /// No standing row. `connected` names every server with a live
    /// connection, which `Rendering` A1a's opening rule counts.
    Gone { connected: Vec<String> },
}

/// **`Rendering` A10's answer for the `.blend` the entry names**, where its
/// machine is connected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheBlendIs {
    There,
    Gone,
    MayNotBeRead,
    TheServerDidNotAnswer,
}

/// **The press, composed from the entry, its kept setup and what was read
/// of the machine** — every decision of the route and no act. `Rendering` A4's figure
/// is handed in already read, so this is testable with nothing reached.
///
/// **For an entry whose record holds no typed frame text**; one that does is
/// [`the_press_composed_from`]'s.
#[allow(clippy::too_many_arguments)]
pub fn the_press_composed(
    record: &LocalJobRecord,
    kept: &Payload,
    way: &TheWayIn,
    server: &WhatTheServerIs,
    blend: WhatTheBlendIs,
    the_carry_ons_figure: Option<&TheCarryOnsFigure>,
    the_machines_frame_list: Option<&[i64]>,
) -> Result<ThePressComposed, &'static str> {
    the_press_composed_from(record, kept, None, way, server, blend, the_carry_ons_figure, the_machines_frame_list)
}

/// [`the_press_composed`], **with the frame text the entry's record holds** —
/// record version 51, `Queue & History` P5b. **`Render Settings` F1: a page
/// opened by *Submit again* takes it as the field's text**, together with the
/// kept setup's parsed form, so *1,2-4* stays *1,2-4* (`Design/Queue &
/// History`, *The two ways to render it again*). **`Rendering` A4's *Carry on*
/// composes its own range and takes none**: the field then writes that range's
/// numbers. The eighteenth notes, piece 7.
#[allow(clippy::too_many_arguments)]
pub fn the_press_composed_from(
    record: &LocalJobRecord,
    kept: &Payload,
    the_typed_frames: Option<&str>,
    way: &TheWayIn,
    server: &WhatTheServerIs,
    blend: WhatTheBlendIs,
    the_carry_ons_figure: Option<&TheCarryOnsFigure>,
    the_machines_frame_list: Option<&[i64]>,
) -> Result<ThePressComposed, &'static str> {
    let the_typed_frames = the_typed_frames.filter(|_| matches!(way, TheWayIn::SubmitAgain));
    let job = record.job.as_text();
    let label = crate::surface::the_queue::what_the_job_is_called(record);
    // **`Rendering` A1b: the offer is the entry's, drawn where it ended
    // failed, whichever route opened the page.**
    let offer = record
        .the_ending_learned()
        .is_some_and(the_simplify_offer_is_drawn);
    // **`Queue & History` N10a: the version loads only where it was the user's
    // own pick** — and never onto another machine, whose Blender is its own
    // (`Rendering` A1a's three things the move changes).
    let the_users_override = (record.blender.origin == AnswerOrigin::TheUsersOwnOverride
        && !matches!(server, WhatTheServerIs::Gone { .. }))
    .then(|| {
        smoozy_server_setup::blender::Version::parse(&record.blender.pick.version).map(|version| {
            BlenderIdentity {
                path: record.blender.pick.build_path.clone(),
                version,
            }
        })
    })
    .flatten();
    let the_setup = match the_kept_setup_from(kept, &job, the_typed_frames, the_users_override.clone()) {
        Some(setup) => TheSetupHeld::AScene(HeldSetup::from_the_entry(setup, offer)),
        None => TheSetupHeld::NoScene {
            entries: the_entries_the_payload_kept(kept, &job, the_typed_frames),
            the_users_override,
            the_offer_is_drawn: offer,
        },
    };
    let when = record
        .the_four_moments()
        .the_submitted_moment()
        .and_then(|at| at.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|since| since.as_secs().to_string())
        .unwrap_or_default();
    let the_blend_path = record.blend_path_on_the_machine.clone().unwrap_or_default();
    let server_text = record.server.local_server.as_text();

    // **The carry-on's figure and range** (`Rendering` A4 to `Rendering` A6, `Rendering` A9).
    let (band, the_range) = match way {
        TheWayIn::SubmitAgain => (
            if matches!(server, WhatTheServerIs::Connected)
                || matches!(server, WhatTheServerIs::Gone { connected } if !connected.is_empty())
            {
                WhereItCameFrom::SubmittingAgain {
                    the_jobs_name: label.clone(),
                    when: when.clone(),
                }
            } else {
                WhereItCameFrom::SubmittingAgainWithNothingConnected {
                    the_jobs_name: label.clone(),
                    when: when.clone(),
                }
            },
            None,
        ),
        TheWayIn::CarryOn { typed } => {
            if matches!(server, WhatTheServerIs::Gone { .. }) {
                return Err(crate::surface::the_history::THE_SERVER_IS_GONE_SO_NO_CARRY_ON);
            }
            let kept_figure = record
                .what_the_note_carried
                .last_frame_figure
                .or(record.what_the_note_carried.given_up_where_it_was);
            let (figure, checked) = match the_carry_ons_figure {
                Some(TheCarryOnsFigure::ReadFromTheMachine(figure)) => (*figure, true),
                Some(TheCarryOnsFigure::TheMachineCouldNotBeChecked { case, .. }) => {
                    (case.the_kept_figure, false)
                }
                None => (kept_figure, false),
            };
            // **`Rendering` A6: no frame to start at, and the action is not offered.**
            let Some(figure) = figure else {
                return Err(crate::surface::the_history::NO_FRAME_FINISHED_SO_NOTHING_TO_START_FROM);
            };
            // **`Rendering` A5: the figure is a starting point the user can type over.**
            // The field held the entry's own figure, so a figure the same as
            // that is no answer of theirs and the reading stands.
            let typed = typed
                .as_deref()
                .map(str::trim)
                .filter(|text| !text.is_empty());
            let from = match typed {
                None => figure,
                Some(text) => match text.parse::<i64>() {
                    Ok(number) if Some(number) == kept_figure => figure,
                    Ok(number) => number,
                    Err(_) => return Err(words::TYPE_A_FRAME_FROM_THE_RANGE),
                },
            };
            let order: Vec<i64> = match (&record.the_frames.asked, the_machines_frame_list) {
                (Some(asked), _) => asked.the_order(),
                (None, Some(list)) => list.to_vec(),
                (None, None) => {
                    return Err(crate::surface::the_history::NO_FRAME_TO_START_FROM)
                }
            };
            let Some(range) = the_carried_on_range(&order, from) else {
                return Err(words::TYPE_A_FRAME_FROM_THE_RANGE);
            };
            let starts_at = match &range {
                FrameForm::Triple { first, .. } => *first,
                FrameForm::List(list) => list.first().copied().unwrap_or(from),
            };
            (
                if checked {
                    WhereItCameFrom::CarryingOn {
                        the_jobs_name: label.clone(),
                        when: when.clone(),
                        the_last_frame_written: figure,
                        the_range_now_starts_at: starts_at,
                    }
                } else {
                    WhereItCameFrom::CarryingOnWithTheOutputsUnread {
                        the_jobs_name: label.clone(),
                        when: when.clone(),
                        the_range_now_starts_at: starts_at,
                    }
                },
                Some(range),
            )
        }
    };
    let came_from = ThePageCameFrom {
        band,
        the_simplify_offer_is_drawn: offer,
        the_entry: record.job.clone(),
        the_entrys_label: label,
        the_moment: Some(when).filter(|when| !when.is_empty()),
    };

    let (leads, the_bar) = match server {
        WhatTheServerIs::NotConnected => (
            WhereThePressLeads::ThePageOpens {
                server: server_text,
                file: the_blend_path,
            },
            None,
        ),
        WhatTheServerIs::Connected => blend_answer(the_blend_path, server_text, blend)?,
        WhatTheServerIs::Gone { connected } => {
            match where_the_browser_opens(false, false, connected.len()) {
                WhereTheBrowserOpens::TheOneConnectedMachine
                | WhereTheBrowserOpens::TheMachineBeingBrowsedThisRun
                | WhereTheBrowserOpens::TheMachineTheJobRanOn => (
                    WhereThePressLeads::TheBrowserOpens {
                        server: connected.first().cloned().unwrap_or_default(),
                    },
                    Some(TheBar::Find),
                ),
                WhereTheBrowserOpens::NoDocumentNamesTheOpeningMachine { .. } => {
                    (WhereThePressLeads::TheServersOpen, Some(TheBar::Find))
                }
                WhereTheBrowserOpens::UnavailableUntilAMachineIsConnected => (
                    WhereThePressLeads::ItIsSaid(words::CONNECT_A_SERVER_TO_FIND_THE_BLEND),
                    Some(TheBar::Find),
                ),
            }
        }
    };
    Ok(ThePressComposed {
        leads,
        the_setup,
        the_range,
        came_from,
        the_bar,
    })
}

/// **`Rendering` A10 and `Rendering` A11 on a connected machine**: the file there opens
/// the page; gone opens the browser on that machine with the pick bar; the
/// other two are said as themselves.
fn blend_answer(
    blend: String,
    server: String,
    is: WhatTheBlendIs,
) -> Result<(WhereThePressLeads, Option<TheBar>), &'static str> {
    match is {
        WhatTheBlendIs::There => Ok((WhereThePressLeads::ThePageOpens { server, file: blend }, None)),
        WhatTheBlendIs::Gone => Ok((WhereThePressLeads::TheBrowserOpens { server }, Some(TheBar::Pick))),
        WhatTheBlendIs::MayNotBeRead => Err(words::THE_BLEND_MAY_NOT_BE_READ),
        WhatTheBlendIs::TheServerDidNotAnswer => Err(words::THE_SERVER_DID_NOT_ANSWER),
    }
}

// ════════════════════════════════════════════════════════════════════════════
// The pick in hand, held for the run
// ════════════════════════════════════════════════════════════════════════════

/// **The setup that waits for a file** — `Render Settings` Y15's no-key
/// route: held by the page, outside the store, and written into the store in
/// one act at the moment the user's pick lands (`File Browser` §12). **Held
/// for the run and nowhere else**; one at a time, a later press replacing an
/// earlier.
#[derive(Debug, Clone, PartialEq)]
pub struct ThePickInHand {
    pub the_bar: TheBar,
    pub the_setup: TheSetupHeld,
    pub the_range: Option<FrameForm>,
    pub came_from: ThePageCameFrom,
}

fn the_pick_this_run() -> &'static Mutex<Option<ThePickInHand>> {
    static HELD: OnceLock<Mutex<Option<ThePickInHand>>> = OnceLock::new();
    HELD.get_or_init(|| Mutex::new(None))
}

/// **The bar the Files tab carries while a pick is in hand** — `File
/// Browser` §12's entry point, worded by `crate::surface::files`.
pub fn the_bar_in_hand() -> Option<crate::surface::files::ABarAcrossTheTop> {
    let held = the_pick_this_run().lock().ok()?;
    let pick = held.as_ref()?;
    Some(match pick.the_bar {
        TheBar::Pick => crate::surface::files::the_pick_bar(&pick.came_from.the_entrys_label),
        TheBar::Find => crate::surface::files::the_find_bar(&pick.came_from.the_entrys_label),
    })
}

/// **The way out on the bar, or the user leaving the browser** — `Design/File
/// Browser § The browser opened so the user can point at a file`: *"the
/// caller is told nothing was picked"*. Nothing is written.
pub fn the_pick_is_dropped() {
    if let Ok(mut held) = the_pick_this_run().lock() {
        *held = None;
    }
}

/// **The pick lands** — a `.blend` chosen in the browser while a pick is in
/// hand. The held setup is written into the store under the key that now
/// exists, in one act, and the page that opens on the file says where it came
/// from. **Nothing happens where no pick is in hand**: an ordinary choice of a
/// file is an ordinary page.
pub fn the_pick_lands(held: &mut WhatTheRunHolds, server: &str, file: &str) {
    let Some(mut pick) = the_pick_this_run().lock().ok().and_then(|mut held| held.take()) else {
        return;
    };
    // `Rendering` A1a: the chosen file is on a connected server. A pick
    // held while none was connected must no longer say that none is.
    if let WhereItCameFrom::SubmittingAgainWithNothingConnected { the_jobs_name, when } = pick.came_from.band {
        pick.came_from.band = WhereItCameFrom::SubmittingAgain { the_jobs_name, when };
    }
    the_setup_is_loaded(held, server, file, pick.the_setup, pick.the_range.as_ref(), pick.came_from);
}

/// **The one write into the store**, at the open or at the pick: `Render
/// Settings` `Render Settings` Y15's load under the key, `Rendering` A4's range over it on a carry-on, and
/// where the page came from.
pub fn the_setup_is_loaded(
    held: &mut WhatTheRunHolds,
    server: &str,
    file: &str,
    setup: TheSetupHeld,
    range: Option<&FrameForm>,
    came_from: ThePageCameFrom,
) {
    let key = FileKey {
        server: crate::surface::render_settings::the_servers_identity(server),
        path: file.to_string(),
    };
    let at = match setup {
        TheSetupHeld::AScene(setup) => {
            let scene = setup.working_set().scene.name.clone();
            held.store.write_the_held_setup_at_the_pick(key.clone(), setup);
            Where::AScene {
                file: key,
                scene,
            }
        }
        TheSetupHeld::NoScene {
            entries,
            the_users_override,
            the_offer_is_drawn,
        } => {
            a_kept_setup_with_no_scene_is_loaded(
                &mut held.store,
                key.clone(),
                &entries,
                the_users_override,
                the_offer_is_drawn,
            );
            Where::ThePreReadSet { file: key }
        }
    };
    // **`Render Settings` Y15b: written as a loaded range, so it belongs to the
    // one job** and what stood there comes back when that job goes.
    if let Some(range) = range {
        held.store.a_range_is_loaded(at, range.clone());
    }
    crate::surface::render_settings::the_page_came_from(held, server, file, came_from);
}

// ════════════════════════════════════════════════════════════════════════════
// The press, performed
// ════════════════════════════════════════════════════════════════════════════

/// **The press, performed** — the entry read from the record, the machine
/// read where it is connected (`Rendering` A10, then `Rendering` A4 on a carry-on), the press
/// composed, and the store or the pick written. **Runs off the drawing
/// thread**; the caller posts what it leads to.
pub fn the_press_is_taken<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    job: &str,
    way: &TheWayIn,
) -> WhereThePressLeads {
    let Some(job) = Identity::read_back(IdentityKind::Job, job) else {
        return WhereThePressLeads::ItIsSaid(words::THE_RECORD_WOULD_NOT_READ);
    };
    // **The record, read and let go of** before the machine is asked.
    let Some(read) = what_the_record_holds(app, &job) else {
        return WhereThePressLeads::ItIsSaid(words::THE_RECORD_WOULD_NOT_READ);
    };
    let WhatTheRecordHolds {
        record,
        kept,
        typed_frames,
        the_server_stands,
        connected,
        row,
    } = read;
    let Some(kept) = kept else {
        return WhereThePressLeads::ItIsSaid(words::NO_SETTINGS_WERE_KEPT);
    };
    let the_server = record.server.local_server.as_text();
    let point = app
        .try_state::<std::sync::Arc<crate::servers::a_period::ThePeriodsThisRun>>()
        .and_then(|periods| periods.the_sessions_of(&the_server))
        .filter(crate::servers::sessions::ThisServersSessions::a_connection_is_live);
    let server = match (the_server_stands, &point) {
        (false, _) => WhatTheServerIs::Gone { connected },
        (true, Some(_)) => WhatTheServerIs::Connected,
        (true, None) => WhatTheServerIs::NotConnected,
    };
    let mut blend = WhatTheBlendIs::There;
    let mut figure = None;
    let mut frame_list = None;
    if let (WhatTheServerIs::Connected, Some(point)) = (&server, &point) {
        if let Some(path) = record.blend_path_on_the_machine.clone() {
            blend = the_blend_is_read(point, &path);
        }
        if let (TheWayIn::CarryOn { .. }, Some(row)) = (way, row.as_ref()) {
            let read = the_carry_on_is_read(point.clone(), row, &record);
            figure = Some(read.0);
            frame_list = read.1;
        }
    }
    let composed = match the_press_composed_from(
        &record,
        &kept,
        typed_frames.as_deref(),
        way,
        &server,
        blend,
        figure.as_ref(),
        frame_list.as_deref(),
    ) {
        Ok(composed) => composed,
        Err(said) => return WhereThePressLeads::ItIsSaid(said),
    };
    match (&composed.leads, composed.the_bar) {
        (WhereThePressLeads::ThePageOpens { server, file }, _) => {
            if let Ok(mut held) = crate::surface::render_settings::the_run_holds().lock() {
                the_setup_is_loaded(
                    &mut held,
                    server,
                    file,
                    composed.the_setup.clone(),
                    composed.the_range.as_ref(),
                    composed.came_from.clone(),
                );
            } else {
                return WhereThePressLeads::ItIsSaid(words::THE_RECORD_WOULD_NOT_READ);
            }
        }
        (_, Some(bar)) => {
            if let Ok(mut held) = the_pick_this_run().lock() {
                *held = Some(ThePickInHand {
                    the_bar: bar,
                    the_setup: composed.the_setup.clone(),
                    the_range: composed.the_range.clone(),
                    came_from: composed.came_from.clone(),
                });
            }
        }
        _ => {}
    }
    composed.leads
}

/// What the press reads out of the local record before the machine is asked.
struct WhatTheRecordHolds {
    record: LocalJobRecord,
    kept: Option<Payload>,
    /// **`Queue & History` P5b's typed frame text**, record version 51.
    typed_frames: Option<String>,
    the_server_stands: bool,
    /// Every server with a live connection, by its identifier.
    connected: Vec<String>,
    row: Option<smoozy_servers::store::ServerRow>,
}

fn what_the_record_holds<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    job: &Identity,
) -> Option<WhatTheRecordHolds> {
    use crate::record::job_records::{JobRecords, TheKeptSetup};
    let started = app.try_state::<Mutex<crate::start::Started>>()?;
    let started = started.lock().ok()?;
    let local = started.record.as_ref()?;
    let records = JobRecords::over(local.connection());
    let record = records.read(job).ok().flatten()?;
    let kept = match records.the_kept_setup_of(job).ok().flatten() {
        Some(TheKeptSetup::Kept(payload)) => Some(payload),
        _ => None,
    };
    let typed_frames = records.the_typed_frames_of(job).ok().flatten();
    let rows = crate::record::server_rows::all(local.connection()).ok()?;
    let the_server = record.server.local_server.as_text();
    let row = rows
        .iter()
        .find(|row| row.local_server_identifier == the_server)
        .cloned();
    let periods = app.try_state::<std::sync::Arc<crate::servers::a_period::ThePeriodsThisRun>>();
    let connected = rows
        .iter()
        .map(|row| row.local_server_identifier.clone())
        .filter(|server| {
            periods
                .as_ref()
                .and_then(|periods| periods.the_sessions_of(server))
                .is_some_and(|point| point.a_connection_is_live())
        })
        .collect();
    Some(WhatTheRecordHolds {
        the_server_stands: row.is_some(),
        record,
        kept,
        typed_frames,
        connected,
        row,
    })
}

/// **`Rendering` A10: one attribute read of the `.blend`'s own path**, on the
/// file-transfer route, where its distinct statuses tell gone from refused in
/// one request (`File Browser` P1). **Somebody is waiting on it** — the user
/// pressed — so it goes before readings nobody waits on (`Server protocol
/// and transfers` §1.1 item 2).
fn the_blend_is_read(point: &crate::servers::sessions::ThisServersSessions, path: &str) -> WhatTheBlendIs {
    use smoozy_server_protocol::subsystem::{StatusCode, SubsystemFailure};
    let path = path.as_bytes().to_vec();
    let answered = crate::servers::the_machines_work::waited_for(|| async move {
        point
            .a_reading(true, move |subsystem| {
                Box::pin(async move { subsystem.stat(&path).await.map(|_| ()) })
            })
            .await
    });
    match answered {
        Ok(Ok(())) => WhatTheBlendIs::There,
        Ok(Err(SubsystemFailure::TheServerRefused(StatusCode::NoSuchFile))) => WhatTheBlendIs::Gone,
        Ok(Err(SubsystemFailure::TheServerRefused(StatusCode::PermissionDenied))) => {
            WhatTheBlendIs::MayNotBeRead
        }
        _ => WhatTheBlendIs::TheServerDidNotAnswer,
    }
}

/// **`Rendering` A4, read over the server's own point**: `Rendering`'s own
/// [`smoozy_rendering::acting::page_readings::the_carry_ons_figure`], with its
/// two machine readings given here. The frame list the attempt record
/// carries comes back beside the figure, for a record whose asked order was
/// never filled in.
fn the_carry_on_is_read(
    point: crate::servers::sessions::ThisServersSessions,
    row: &smoozy_servers::store::ServerRow,
    record: &LocalJobRecord,
) -> (TheCarryOnsFigure, Option<Vec<i64>>) {
    let kept = record
        .what_the_note_carried
        .last_frame_figure
        .or(record.what_the_note_carried.given_up_where_it_was);
    let could_not = |kept| TheCarryOnsFigure::TheMachineCouldNotBeChecked {
        why: smoozy_rendering::again::TheMachineCouldNotBeChecked::NoAttemptRecordOnTheMachineCarriesAFrameList,
        case: smoozy_rendering::again::the_could_not_check_case(kept),
    };
    let (Some(layout), crate::servers::installation::WhereThePackageAreaIs::At(package_area)) = (
        crate::servers::installation::the_layout_recorded_for(row),
        crate::servers::installation::where_the_package_area_is(row),
    ) else {
        return (could_not(kept), None);
    };
    let Ok(paths) =
        smoozy_job_files::publication::AJobsPaths::composed(&layout, &record.job, record.label.as_deref())
    else {
        return (could_not(kept), None);
    };
    let readings = OverThePoint {
        machine: std::cell::RefCell::new(crate::servers::the_machines_work::OverThisServersPoint {
            sessions: point,
            package_area,
        }),
        record_directory: paths.job_record_directory.clone(),
        attempts: record.what_the_note_carried.attempts,
        kept,
        read: std::cell::RefCell::new(None),
    };
    // **The folder the manifest names is the job's folder** (`Job files and
    // outputs` M1) — or, from 2026-10-04, the one the render worker chose at
    // render start (`Job files and outputs` P21) — read with the attempt
    // record; **the local record's own stands in only where neither reads
    // back**, and a job whose folder was never chosen has none to check.
    let Some(folder) = readings
        .the_map_read()
        .and_then(|(_, folder)| folder)
        .or_else(|| record.the_job_output_folder(&layout))
    else {
        return (could_not(kept), None);
    };
    let figure =
        smoozy_rendering::acting::page_readings::the_carry_ons_figure(&readings, &record.job, &folder);
    let list = readings
        .the_map_read()
        .map(|(map, _)| map.the_frame_list_asked_for);
    (figure, list)
}

/// **`Rendering`'s readings trait, over the server's own point** — the two `Rendering` A4
/// names read here; the three others are the record's and are read from it.
struct OverThePoint {
    machine: std::cell::RefCell<crate::servers::the_machines_work::OverThisServersPoint>,
    record_directory: String,
    attempts: u32,
    kept: Option<i64>,
    read: std::cell::RefCell<Option<Option<(smoozy_rendering::acting::page_readings::TheMapInForceRead, Option<String>)>>>,
}

impl OverThePoint {
    /// The attempt record's read, taken once and held for the press.
    fn the_map_read(
        &self,
    ) -> Option<(smoozy_rendering::acting::page_readings::TheMapInForceRead, Option<String>)> {
        if self.read.borrow().is_none() {
            let read = crate::servers::the_machines_work::the_map_in_force_on_the_machine(
                &mut *self.machine.borrow_mut(),
                &self.record_directory,
                self.attempts,
            );
            *self.read.borrow_mut() = Some(read);
        }
        self.read.borrow().clone().flatten()
    }
}

impl smoozy_rendering::acting::page_readings::Readings for OverThePoint {
    /// **Not read on this route**: the carry-on's figure stands on the walk
    /// and the attempt record alone, and no warning is composed here. The
    /// honest answer for a reading not taken is the one that claims nothing.
    fn the_check_standing(
        &self,
        _: &str,
        _: &Identity,
        _: &str,
    ) -> smoozy_dependencies::standing::WhatTheRecordSays {
        smoozy_dependencies::standing::WhatTheRecordSays::NobodyHasEverChecked
    }

    /// **Not read on this route**, for the same reason: nothing is composed
    /// for a submission here.
    fn the_app_wide_reporting_item(&self) -> Option<smoozy_render_settings::reporting::ReportingChoice> {
        None
    }

    fn the_figure_the_entry_kept(&self, _: &Identity) -> Option<i64> {
        self.kept
    }

    fn walk_the_job_output_folder(
        &self,
        _: &Identity,
        the_job_output_folder: &str,
    ) -> smoozy_job_files::reckoning::WhatTheWalkOfTheFolderIs {
        use smoozy_download::watch::AWalk;
        use smoozy_job_files::reckoning::{AWalkEntry, WhatKindOfEntry, WhatTheWalkOfTheFolderIs};
        let sessions = self.machine.borrow().sessions.clone();
        let folder = the_job_output_folder.as_bytes().to_vec();
        let answer = crate::servers::the_machines_work::waited_for(|| async move {
            let mut route = crate::downloading::TheWalksReadingRoute::on(&sessions);
            smoozy_download::acting::walking::walk_the_folder_and_everything_under_it(&mut route, &folder).await
        });
        match answer.walk {
            AWalk::Complete(entries) => WhatTheWalkOfTheFolderIs::ItCompleted(
                entries
                    .into_iter()
                    .map(|entry| AWalkEntry {
                        kind: if entry.is_a_folder {
                            WhatKindOfEntry::ADirectory
                        } else if answer.the_plain_files.contains(&entry.path) {
                            WhatKindOfEntry::AFile
                        } else {
                            WhatKindOfEntry::SomethingElse
                        },
                        path_under_the_job_output_folder: entry.path,
                        size: entry.size,
                    })
                    .collect(),
            ),
            AWalk::Discarded(_) => WhatTheWalkOfTheFolderIs::ItDidNotComplete,
        }
    }

    fn the_map_in_force_on_the_machine(
        &self,
        _: &Identity,
    ) -> Option<smoozy_rendering::acting::page_readings::TheMapInForceRead> {
        self.the_map_read().map(|(map, _)| map)
    }
}

/// **What the press leads to, posted for the history page** — the page goes
/// where the press leads (`web/the-history.js`).
pub fn the_page_is_told(queue: &TheOneQueue, leads: &WhereThePressLeads) {
    let event = match leads {
        WhereThePressLeads::ThePageOpens { server, file } => Event::about(About::TheRun, A_NEW_JOB_OPENS)
            .saying("goesTo", Said::Value(Verbatim::new("the-page")))
            .saying("server", Said::Value(Verbatim::new(server.clone())))
            .saying("file", Said::Value(Verbatim::new(file.clone()))),
        WhereThePressLeads::TheBrowserOpens { server } => Event::about(About::TheRun, A_NEW_JOB_OPENS)
            .saying("goesTo", Said::Value(Verbatim::new("the-browser")))
            .saying("server", Said::Value(Verbatim::new(server.clone()))),
        WhereThePressLeads::TheServersOpen => Event::about(About::TheRun, A_NEW_JOB_OPENS)
            .saying("goesTo", Said::Value(Verbatim::new("the-servers"))),
        WhereThePressLeads::ItIsSaid(_) => return,
    };
    let _ = queue.read_then_post::<Event, std::convert::Infallible>(|| Ok(event.clone()), |event| event.clone());
}

/// **`Queue & History` N20's way, taken** — History opens on the earlier
/// entry (`web/the-history.js`).
pub fn the_earlier_entry_is_told(queue: &TheOneQueue, earlier: &Identity) {
    let event = Event::about(About::TheRun, A_NEW_JOB_OPENS)
        .saying("goesTo", Said::Value(Verbatim::new("the-history")))
        .saying("job", Said::Value(Verbatim::new(earlier.as_text())));
    let _ = queue.read_then_post::<Event, std::convert::Infallible>(|| Ok(event.clone()), |event| event.clone());
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_queue_history::acting::note::{TheEndingOnTheNote, TheOutcomeAsWritten};
    use smoozy_queue_history::record::TheFramesAsked;
    use smoozy_render_settings::names::{AnchorState, ExactName, Provenance};
    use smoozy_render_settings::payload::{SceneEntry, VersionPick};
    use smoozy_render_settings::store::KeptVersion;

    /// A job of frames 1 to 240 on `server`, ended `outcome` with the figure
    /// `figure`, its `.blend` at `/root/kitchen.blend`.
    /// **`Render Settings` Y15b: a *Resume*'s range belongs to that one job.** The
    /// page held 11–40 for the next job in the second 5.3 live pass. Once it goes,
    /// the page reads as it did before: the range, the band, and the carry-on
    /// mark; every other answer stays as `Render Settings` Y15 has it. **Undone (the range and the
    /// band left standing), this fails.**
    #[test]
    fn a_resumes_range_goes_with_its_job_and_the_page_reads_as_it_did() {
        use crate::surface::render_settings::{the_loaded_range_goes, the_panel};
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let file = "/root/kitchen.blend";
        let key = FileKey { server: server.clone(), path: file.to_string() };
        let scene = ExactName::as_the_reading_produced_it("Kitchen");
        let composed = the_press_composed(
            &an_entry(&server, TheOutcomeAsWritten::LeftStopped, Some(60)),
            &a_kept_setup(true),
            &TheWayIn::CarryOn { typed: None },
            &WhatTheServerIs::NotConnected,
            WhatTheBlendIs::There,
            None,
            None,
        )
        .unwrap();
        let loaded = |held: &mut WhatTheRunHolds| {
            the_setup_is_loaded(
                held,
                &server.as_text(),
                file,
                composed.the_setup.clone(),
                composed.the_range.as_ref(),
                composed.came_from.clone(),
            );
        };
        let a_range = FrameForm::Triple { first: 60, last: 240, step: 1 };
        let mut held = WhatTheRunHolds::default();
        loaded(&mut held);
        let the_range = |held: &WhatTheRunHolds| held.store.the_answers_composition_reads(&key, Some(&scene));
        assert_eq!(the_range(&held).frame_range, Some(a_range));
        assert!(!the_panel(&held, &server.as_text(), file).where_it_came_from.is_empty());
        assert!(held.where_it_came_from.values().next().unwrap().the_carry_on_mark().is_some());

        the_loaded_range_goes(&mut held, &server.as_text(), file);
        assert_eq!(the_range(&held).frame_range, None, "the page reads the file's own range again");
        assert_eq!(the_range(&held).sample_count, Some(512), "every other answer stays as the plan has it");
        let panel = the_panel(&held, &server.as_text(), file);
        assert!(panel.where_it_came_from.is_empty(), "the band went with the job");
        assert!(panel.simplify.is_none());
        assert!(held.where_it_came_from.is_empty(), "the next job carries no carry-on mark");

        // **A range typed before the press comes back, and one typed after stays.**
        let typed = FrameForm::Triple { first: 5, last: 9, step: 1 };
        let at = Where::AScene { file: key.clone(), scene: scene.clone() };
        let mut held = WhatTheRunHolds::default();
        held.store.answer_the_typed_range(at.clone(), typed.clone(), "5-9");
        loaded(&mut held);
        the_loaded_range_goes(&mut held, &server.as_text(), file);
        assert_eq!(the_range(&held).frame_range, Some(typed.clone()));
        assert_eq!(the_range(&held).frame_range_text.as_deref(), Some("5-9"));

        let mut held = WhatTheRunHolds::default();
        loaded(&mut held);
        held.store.answer_the_typed_range(at, typed.clone(), "5-9");
        the_loaded_range_goes(&mut held, &server.as_text(), file);
        assert_eq!(the_range(&held).frame_range, Some(typed), "a range the user typed stays");
    }

    /// **`Render Settings` Y15b: *Submit again* loads a range the same way, the
    /// same rule**, on an entry that named a scene and on one that named none.
    /// **`Rendering` A1c: a failed entry's Simplify stays drawn with its entries**,
    /// the band gone, so nothing is composed unshown. **Undone, this fails.**
    #[test]
    fn submit_agains_range_goes_with_its_job_and_a_failed_entrys_offer_stays_drawn() {
        use crate::surface::render_settings::{the_loaded_range_goes, the_panel};
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let file = "/root/kitchen.blend";
        let key = FileKey { server: server.clone(), path: file.to_string() };
        let scene = ExactName::as_the_reading_produced_it("Kitchen");
        for (named, the_scene) in [(true, Some(&scene)), (false, None)] {
            let mut kept = a_kept_setup(named);
            kept.frame_range = Some(FrameForm::Triple { first: 11, last: 40, step: 1 });
            let composed = the_press_composed(
                &an_entry(&server, TheOutcomeAsWritten::Finished, None),
                &kept,
                &TheWayIn::SubmitAgain,
                &WhatTheServerIs::NotConnected,
                WhatTheBlendIs::There,
                None,
                None,
            )
            .unwrap();
            let mut held = WhatTheRunHolds::default();
            the_setup_is_loaded(&mut held, &server.as_text(), file, composed.the_setup, None, composed.came_from);
            assert_eq!(held.store.the_answers_composition_reads(&key, the_scene).frame_range, kept.frame_range, "scene {named}");
            the_loaded_range_goes(&mut held, &server.as_text(), file);
            assert_eq!(held.store.the_answers_composition_reads(&key, the_scene).frame_range, None, "scene {named}");
            assert!(the_panel(&held, &server.as_text(), file).where_it_came_from.is_empty(), "scene {named}");
        }

        let composed = the_press_composed(
            &an_entry(&server, TheOutcomeAsWritten::Failed, Some(60)),
            &a_kept_setup(true),
            &TheWayIn::SubmitAgain,
            &WhatTheServerIs::NotConnected,
            WhatTheBlendIs::There,
            None,
            None,
        )
        .unwrap();
        let mut held = WhatTheRunHolds::default();
        the_setup_is_loaded(&mut held, &server.as_text(), file, composed.the_setup, None, composed.came_from);
        assert!(the_panel(&held, &server.as_text(), file).simplify.is_some());
        the_loaded_range_goes(&mut held, &server.as_text(), file);
        let panel = the_panel(&held, &server.as_text(), file);
        assert!(panel.simplify.is_some(), "the entries stand, so the offer stays drawn");
        assert!(panel.where_it_came_from.is_empty(), "the band went");
    }

    fn an_entry(server: &Identity, outcome: TheOutcomeAsWritten, figure: Option<i64>) -> LocalJobRecord {
        let mut record = crate::record::deletions::tests::a_record_in_history(server, "Kitchen — final");
        record.blend_path_on_the_machine = Some("/root/kitchen.blend".to_string());
        record.the_frames.asked = TheFramesAsked::from_the_order(&(1..=240).collect::<Vec<i64>>());
        record.what_the_note_carried.ending = Some(TheEndingOnTheNote {
            outcome,
            exit_status: None,
            the_attempt_whose_capture_record_holds_the_words: 1,
            the_reason_the_machine_recorded: None,
            no_render_began: false,
            the_device_line_as_observed: None,
        });
        record.what_the_note_carried.last_frame_figure = figure;
        record
    }

    /// The setup the entry kept: scene *Kitchen*, 512 samples, and Simplify on.
    fn a_kept_setup(scene: bool) -> Payload {
        let mut payload = Payload {
            prepare_simulations: true,
            version_pick: VersionPick {
                build_path: "/b".to_string(),
                version: "4.5.12".to_string(),
            },
            scene: scene.then(|| SceneEntry {
                name: ExactName::as_the_reading_produced_it("Kitchen"),
                anchor: AnchorState::ToAnchor {
                    provenance: Provenance::AKeptSetup {
                        the_old_job: "earlier".to_string(),
                    },
                },
            }),
            frame_range: None,
            sample_count: Some(512),
            resolution_percentage: None,
            engine: None,
            device: None,
            view_layer_pins: Vec::new(),
            simplify: Default::default(),
            the_check_is_written_as_a_picture: false,
            the_check_writes_into: None,
        };
        payload.simplify.use_simplify = Some(true);
        payload
    }

    fn the_entries_of(setup: &TheSetupHeld) -> SceneEntries {
        match setup {
            TheSetupHeld::AScene(held) => held.working_set().entries.clone(),
            TheSetupHeld::NoScene { entries, .. } => entries.clone(),
        }
    }

    /// **`Render Settings` F1 and `Queue & History` P5b: *Submit again* opens
    /// the page with the frames as the user typed them**, the field reading
    /// *1,2-4* over the kept setup's parsed form, on a scene-named entry and on
    /// one that named no scene; **a record with no text writes the numbers**,
    /// and **`Rendering` A4's *Carry on* takes none**. The eighteenth notes,
    /// piece 7, and the drawing's `res-againlist`.
    #[test]
    fn submit_again_opens_the_page_with_the_frames_as_typed_and_carry_on_with_its_own_numbers() {
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let finished = an_entry(&server, TheOutcomeAsWritten::Finished, None);
        let file = "/root/kitchen.blend";
        let key = FileKey { server: server.clone(), path: file.to_string() };
        let the_field_after = |kept: &Payload, typed: Option<&str>, way: TheWayIn, record: &LocalJobRecord| {
            let composed = the_press_composed_from(
                record,
                kept,
                typed,
                &way,
                &WhatTheServerIs::Connected,
                WhatTheBlendIs::There,
                Some(&TheCarryOnsFigure::ReadFromTheMachine(Some(3))),
                None,
            )
            .unwrap();
            let mut held = WhatTheRunHolds::default();
            the_setup_is_loaded(
                &mut held,
                &server.as_text(),
                file,
                composed.the_setup.clone(),
                composed.the_range.as_ref(),
                composed.came_from.clone(),
            );
            let scene = kept.scene.as_ref().map(|scene| scene.name.clone());
            let answers = held.store.the_answers_composition_reads(&key, scene.as_ref());
            let field = crate::surface::render_settings::the_panel(&held, &server.as_text(), file)
                .the_range
                .value
                .map(|value| value.as_str().to_string());
            (answers, field)
        };
        for scene in [true, false] {
            let mut kept = a_kept_setup(scene);
            kept.frame_range = Some(FrameForm::List(vec![1, 2, 3, 4]));
            let (answers, field) = the_field_after(&kept, Some("1,2-4"), TheWayIn::SubmitAgain, &finished);
            assert_eq!(answers.frame_range, kept.frame_range, "scene {scene}");
            assert_eq!(answers.frame_range_text.as_deref(), Some("1,2-4"), "scene {scene}");
            assert_eq!(field.as_deref(), Some("1,2-4"), "scene {scene}: the field reads as typed");
            let (answers, field) = the_field_after(&kept, None, TheWayIn::SubmitAgain, &finished);
            assert_eq!(answers.frame_range_text, None, "scene {scene}");
            assert_eq!(field.as_deref(), Some("1,2,3,4"), "scene {scene}: no text writes the numbers");
        }
        // **Carry on composes its own range and hands no text.**
        let mut stopped = an_entry(&server, TheOutcomeAsWritten::LeftStopped, Some(3));
        stopped.the_frames.asked = TheFramesAsked::from_the_order(&[1, 2, 3, 4]);
        let mut kept = a_kept_setup(true);
        kept.frame_range = Some(FrameForm::List(vec![1, 2, 3, 4]));
        let (answers, field) = the_field_after(&kept, Some("1,2-4"), TheWayIn::CarryOn { typed: None }, &stopped);
        assert_eq!(answers.frame_range_text, None);
        assert_eq!(answers.frame_range, Some(FrameForm::Triple { first: 3, last: 4, step: 1 }));
        assert_eq!(field.as_deref(), Some("3-4"));
    }

    /// **`Rendering` A1, `Rendering` A1b and `Rendering` A1c on a connected machine whose `.blend` is
    /// there**: the page opens over that server on that file, filled from the
    /// kept setup, and Simplify comes with it only from a failed entry.
    #[test]
    fn submit_again_opens_the_page_on_the_entrys_file_and_simplify_only_from_a_failed_entry() {
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let finished = an_entry(&server, TheOutcomeAsWritten::Finished, None);
        let composed = the_press_composed(
            &finished,
            &a_kept_setup(true),
            &TheWayIn::SubmitAgain,
            &WhatTheServerIs::Connected,
            WhatTheBlendIs::There,
            None,
            None,
        )
        .unwrap();
        assert_eq!(
            composed.leads,
            WhereThePressLeads::ThePageOpens {
                server: server.as_text(),
                file: "/root/kitchen.blend".to_string(),
            }
        );
        assert_eq!(composed.the_range, None);
        assert_eq!(composed.the_bar, None);
        assert!(!composed.came_from.the_simplify_offer_is_drawn);
        assert!(matches!(composed.came_from.band, WhereItCameFrom::SubmittingAgain { .. }));
        assert_eq!(composed.came_from.the_carry_on_mark(), None);
        let entries = the_entries_of(&composed.the_setup);
        assert_eq!(entries.sample_count, Some(512));
        assert!(entries.simplify.is_empty(), "`Rendering` A1c: no offer, no Simplify loaded");
        let TheSetupHeld::AScene(held) = &composed.the_setup else {
            panic!("a scene-named job holds a scene");
        };
        assert_eq!(held.working_set().version, KeptVersion::ResolvedAutomatically);

        let failed = an_entry(&server, TheOutcomeAsWritten::Failed, Some(60));
        let composed = the_press_composed(
            &failed,
            &a_kept_setup(true),
            &TheWayIn::SubmitAgain,
            &WhatTheServerIs::Connected,
            WhatTheBlendIs::There,
            None,
            None,
        )
        .unwrap();
        assert!(composed.came_from.the_simplify_offer_is_drawn);
        assert_eq!(the_entries_of(&composed.the_setup).simplify.use_simplify, Some(true));
    }

    /// **`Rendering` A1a with the entry's server standing but not connected**: the page
    /// opens on the kept setup and waits; **a job that named no scene loads
    /// into the pre-read set**.
    #[test]
    fn with_its_server_not_connected_the_page_opens_and_waits() {
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let entry = an_entry(&server, TheOutcomeAsWritten::Finished, None);
        let composed = the_press_composed(
            &entry,
            &a_kept_setup(false),
            &TheWayIn::SubmitAgain,
            &WhatTheServerIs::NotConnected,
            WhatTheBlendIs::TheServerDidNotAnswer,
            None,
            None,
        )
        .unwrap();
        assert!(matches!(composed.leads, WhereThePressLeads::ThePageOpens { .. }));
        assert!(matches!(
            composed.came_from.band,
            WhereItCameFrom::SubmittingAgainWithNothingConnected { .. }
        ));
        assert!(matches!(composed.the_setup, TheSetupHeld::NoScene { .. }));
    }

    /// **`Rendering` A10 and `Rendering` A11 on a connected machine**: a file gone opens the browser
    /// there with the pick bar; a refusal and no answer are each said as
    /// themselves.
    #[test]
    fn a_blend_gone_opens_the_browser_and_a_refusal_or_no_answer_is_said() {
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let entry = an_entry(&server, TheOutcomeAsWritten::Finished, None);
        let press = |blend| {
            the_press_composed(
                &entry,
                &a_kept_setup(true),
                &TheWayIn::SubmitAgain,
                &WhatTheServerIs::Connected,
                blend,
                None,
                None,
            )
        };
        let gone = press(WhatTheBlendIs::Gone).unwrap();
        assert_eq!(gone.leads, WhereThePressLeads::TheBrowserOpens { server: server.as_text() });
        assert_eq!(gone.the_bar, Some(TheBar::Pick));
        assert_eq!(press(WhatTheBlendIs::MayNotBeRead), Err(words::THE_BLEND_MAY_NOT_BE_READ));
        assert_eq!(press(WhatTheBlendIs::TheServerDidNotAnswer), Err(words::THE_SERVER_DID_NOT_ANSWER));
    }

    /// **`Rendering` A1a with the entry's own server gone**: the find bar on the one
    /// connected machine, the server list where several are connected and
    /// none is named (hole 3), and the setup held where none is — **and the
    /// version the user picked is not carried to another machine**.
    #[test]
    fn with_its_server_gone_the_browser_finds_the_blend_and_smoozy_picks_no_machine() {
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let mut entry = an_entry(&server, TheOutcomeAsWritten::Finished, None);
        entry.blender.origin = AnswerOrigin::TheUsersOwnOverride;
        let press = |connected: Vec<String>| {
            the_press_composed(
                &entry,
                &a_kept_setup(true),
                &TheWayIn::SubmitAgain,
                &WhatTheServerIs::Gone { connected },
                WhatTheBlendIs::There,
                None,
                None,
            )
            .unwrap()
        };
        let one = press(vec!["other".to_string()]);
        assert_eq!(one.leads, WhereThePressLeads::TheBrowserOpens { server: "other".to_string() });
        assert_eq!(one.the_bar, Some(TheBar::Find));
        let TheSetupHeld::AScene(held) = &one.the_setup else {
            panic!("a scene-named job holds a scene");
        };
        assert_eq!(held.working_set().version, KeptVersion::ResolvedAutomatically);
        let several = press(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(several.leads, WhereThePressLeads::TheServersOpen);
        let none = press(Vec::new());
        assert_eq!(
            none.leads,
            WhereThePressLeads::ItIsSaid(words::CONNECT_A_SERVER_TO_FIND_THE_BLEND)
        );
        assert_eq!(none.the_bar, Some(TheBar::Find));
        // **Where the server stands, the user's own version comes with it.**
        let standing = the_press_composed(
            &entry,
            &a_kept_setup(true),
            &TheWayIn::SubmitAgain,
            &WhatTheServerIs::Connected,
            WhatTheBlendIs::There,
            None,
            None,
        )
        .unwrap();
        let TheSetupHeld::AScene(held) = &standing.the_setup else {
            panic!("a scene-named job holds a scene");
        };
        assert!(matches!(held.working_set().version, KeptVersion::TheUsersOverride(_)));
    }

    /// `Design/Render Settings`: the waiting band is only for no connected
    /// server. A gone server does not make the other connections disappear.
    #[test]
    fn submitting_again_uses_the_connected_band_when_another_server_is_connected() {
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let entry = an_entry(&server, TheOutcomeAsWritten::Finished, None);
        for connected in [vec!["other".to_string()], vec!["a".to_string(), "b".to_string()], Vec::new()] {
            let waiting = connected.is_empty();
            let composed = the_press_composed(
                &entry,
                &a_kept_setup(true),
                &TheWayIn::SubmitAgain,
                &WhatTheServerIs::Gone { connected },
                WhatTheBlendIs::There,
                None,
                None,
            )
            .unwrap();
            assert_eq!(
                matches!(composed.came_from.band, WhereItCameFrom::SubmittingAgainWithNothingConnected { .. }),
                waiting,
            );
            if !waiting {
                assert!(matches!(composed.came_from.band, WhereItCameFrom::SubmittingAgain { .. }));
            }
        }
    }

    /// **`Rendering` A4, `Rendering` A5, `Rendering` A6 and `Rendering` A9: the carry-on opens at the figure the server gave,
    /// at the figure the entry kept where the server could not be checked, at
    /// a figure the user typed over either, and not at all where no frame
    /// finished or the server is gone.** Its job carries the mark naming the
    /// entry (`Queue & History` N15).
    #[test]
    fn carry_on_opens_at_the_figure_and_the_new_job_carries_the_mark() {
        use smoozy_rendering::again::the_could_not_check_case;
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let stopped = an_entry(&server, TheOutcomeAsWritten::LeftStopped, Some(60));
        let press = |typed: Option<&str>, figure: Option<TheCarryOnsFigure>, at: &WhatTheServerIs| {
            the_press_composed(
                &stopped,
                &a_kept_setup(true),
                &TheWayIn::CarryOn {
                    typed: typed.map(str::to_string),
                },
                at,
                WhatTheBlendIs::There,
                figure.as_ref(),
                None,
            )
        };
        let read = press(
            Some("60"),
            Some(TheCarryOnsFigure::ReadFromTheMachine(Some(58))),
            &WhatTheServerIs::Connected,
        )
        .unwrap();
        assert_eq!(
            read.the_range,
            Some(FrameForm::Triple {
                first: 58,
                last: 240,
                step: 1
            })
        );
        assert_eq!(
            read.came_from.band,
            WhereItCameFrom::CarryingOn {
                the_jobs_name: "Kitchen — final".to_string(),
                when: "1754600000".to_string(),
                the_last_frame_written: 58,
                the_range_now_starts_at: 58,
            }
        );
        let mark = read.came_from.the_carry_on_mark().unwrap();
        assert_eq!(mark.earlier_job, stopped.job);
        assert_eq!(mark.the_label_as_it_stood, "Kitchen — final");
        assert!(!read.came_from.the_simplify_offer_is_drawn, "a stop is not a failure");

        // **`Rendering` A9: the kept figure, said as kept.**
        let unread = press(
            None,
            Some(TheCarryOnsFigure::TheMachineCouldNotBeChecked {
                why: smoozy_rendering::again::TheMachineCouldNotBeChecked::TheWalkOfTheJobOutputFolderDidNotComplete,
                case: the_could_not_check_case(Some(60)),
            }),
            &WhatTheServerIs::Connected,
        )
        .unwrap();
        assert!(matches!(
            unread.came_from.band,
            WhereItCameFrom::CarryingOnWithTheOutputsUnread {
                the_range_now_starts_at: 60,
                ..
            }
        ));
        // Not connected: nothing read, the kept figure.
        let away = press(None, None, &WhatTheServerIs::NotConnected).unwrap();
        assert!(matches!(
            away.came_from.band,
            WhereItCameFrom::CarryingOnWithTheOutputsUnread { .. }
        ));

        // **`Rendering` A5: the user's own figure.**
        let typed = press(
            Some("50"),
            Some(TheCarryOnsFigure::ReadFromTheMachine(Some(58))),
            &WhatTheServerIs::Connected,
        )
        .unwrap();
        assert_eq!(
            typed.the_range,
            Some(FrameForm::Triple {
                first: 50,
                last: 240,
                step: 1
            })
        );
        assert_eq!(
            press(Some("sixty"), None, &WhatTheServerIs::Connected),
            Err(words::TYPE_A_FRAME_FROM_THE_RANGE)
        );
        assert_eq!(
            press(Some("999"), None, &WhatTheServerIs::Connected),
            Err(words::TYPE_A_FRAME_FROM_THE_RANGE)
        );

        // **`Rendering` A6: no frame finished.**
        assert_eq!(
            press(
                None,
                Some(TheCarryOnsFigure::ReadFromTheMachine(None)),
                &WhatTheServerIs::Connected
            ),
            Err(crate::surface::the_history::NO_FRAME_FINISHED_SO_NOTHING_TO_START_FROM)
        );
        // **`Queue & History` N17: the server gone, no carry-on.**
        assert_eq!(
            press(None, None, &WhatTheServerIs::Gone { connected: Vec::new() }),
            Err(crate::surface::the_history::THE_SERVER_IS_GONE_SO_NO_CARRY_ON)
        );
    }

    /// **The one write: `Render Settings` Y15's load under the key, `Rendering` A4's range over it, where
    /// the page came from — and the page then draws Simplify only from a
    /// failed entry.** Then the pick: held until a file is chosen, written at
    /// the pick under the chosen file's key, dropped by the way out.
    #[test]
    fn the_setup_lands_in_the_store_and_a_pick_is_held_until_a_file_is_chosen() {
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let failed = an_entry(&server, TheOutcomeAsWritten::Failed, Some(60));
        let composed = the_press_composed(
            &failed,
            &a_kept_setup(true),
            &TheWayIn::CarryOn { typed: None },
            &WhatTheServerIs::NotConnected,
            WhatTheBlendIs::There,
            None,
            None,
        )
        .unwrap();
        let mut held = WhatTheRunHolds::default();
        let file = "/root/kitchen.blend";
        the_setup_is_loaded(
            &mut held,
            &server.as_text(),
            file,
            composed.the_setup.clone(),
            composed.the_range.as_ref(),
            composed.came_from.clone(),
        );
        let key = FileKey {
            server: server.clone(),
            path: file.to_string(),
        };
        let scene = ExactName::as_the_reading_produced_it("Kitchen");
        let answers = held.store.the_answers_composition_reads(&key, Some(&scene));
        assert_eq!(answers.sample_count, Some(512));
        assert_eq!(
            answers.frame_range,
            Some(FrameForm::Triple {
                first: 60,
                last: 240,
                step: 1
            })
        );
        let panel = crate::surface::render_settings::the_panel(&held, &server.as_text(), file);
        assert!(!panel.where_it_came_from.is_empty());
        assert_eq!(panel.where_it_came_from_moment.as_deref(), Some("1754600000"));
        assert!(panel.simplify.is_some(), "`Rendering` A1b: a failed entry draws the offer");

        // **A finished entry's page draws none.**
        let finished = an_entry(&server, TheOutcomeAsWritten::Finished, None);
        let composed = the_press_composed(
            &finished,
            &a_kept_setup(true),
            &TheWayIn::SubmitAgain,
            &WhatTheServerIs::NotConnected,
            WhatTheBlendIs::There,
            None,
            None,
        )
        .unwrap();
        let mut held = WhatTheRunHolds::default();
        the_setup_is_loaded(&mut held, &server.as_text(), file, composed.the_setup, None, composed.came_from);
        let panel = crate::surface::render_settings::the_panel(&held, &server.as_text(), file);
        assert!(panel.simplify.is_none(), "`Rendering` A1b: no offer on a finished entry's page");

        // **The pick.** Nothing held: an ordinary choice changes nothing.
        the_pick_is_dropped();
        assert_eq!(the_bar_in_hand(), None);
        let mut held = WhatTheRunHolds::default();
        the_pick_lands(&mut held, "elsewhere", "/srv/found.blend");
        assert!(held.where_it_came_from.is_empty());
        // Held: the bar names the job, and the pick writes under the chosen key.
        let gone = the_press_composed(
            &failed,
            &a_kept_setup(true),
            &TheWayIn::SubmitAgain,
            &WhatTheServerIs::Gone {
                connected: vec!["elsewhere".to_string()],
            },
            WhatTheBlendIs::There,
            None,
            None,
        )
        .unwrap();
        *the_pick_this_run().lock().unwrap() = Some(ThePickInHand {
            the_bar: gone.the_bar.unwrap(),
            the_setup: gone.the_setup,
            the_range: gone.the_range,
            came_from: gone.came_from,
        });
        assert_eq!(
            the_bar_in_hand(),
            Some(crate::surface::files::the_find_bar("Kitchen — final"))
        );
        let elsewhere = Identity::issue(IdentityKind::LocalServer).unwrap().as_text();
        the_pick_lands(&mut held, &elsewhere, "/srv/found.blend");
        assert_eq!(the_bar_in_hand(), None, "the pick is taken once");
        let key = FileKey {
            server: crate::surface::render_settings::the_servers_identity(&elsewhere),
            path: "/srv/found.blend".to_string(),
        };
        assert_eq!(
            held.store.the_answers_composition_reads(&key, Some(&scene)).sample_count,
            Some(512)
        );
        assert_eq!(held.where_it_came_from.len(), 1);
        let panel = crate::surface::render_settings::the_panel(&held, &elsewhere, "/srv/found.blend");
        assert!(!panel.where_it_came_from_is_waiting, "the pick landed on a connected server");
        assert_eq!(panel.where_it_came_from[0].lead, crate::typography::Prose::new("Submitting again."));

        // No connection at the press: after one is connected, the pick must
        // replace the waiting band without losing the job or its moment.
        let waiting = the_press_composed(
            &finished,
            &a_kept_setup(true),
            &TheWayIn::SubmitAgain,
            &WhatTheServerIs::Gone { connected: Vec::new() },
            WhatTheBlendIs::There,
            None,
            None,
        )
        .unwrap();
        let moment = waiting.came_from.the_moment.clone();
        *the_pick_this_run().lock().unwrap() = Some(ThePickInHand {
            the_bar: waiting.the_bar.unwrap(),
            the_setup: waiting.the_setup,
            the_range: waiting.the_range,
            came_from: waiting.came_from,
        });
        the_pick_lands(&mut held, &elsewhere, "/srv/later.blend");
        let panel = crate::surface::render_settings::the_panel(&held, &elsewhere, "/srv/later.blend");
        assert!(!panel.where_it_came_from_is_waiting);
        assert_eq!(panel.where_it_came_from[0].lead, crate::typography::Prose::new("Submitting again."));
        assert_eq!(panel.where_it_came_from_moment, moment);
        // **The way out lets it go and writes nothing.**
        *the_pick_this_run().lock().unwrap() = Some(ThePickInHand {
            the_bar: TheBar::Pick,
            the_setup: TheSetupHeld::NoScene {
                entries: SceneEntries::default(),
                the_users_override: None,
                the_offer_is_drawn: false,
            },
            the_range: None,
            came_from: the_press_composed(
                &failed,
                &a_kept_setup(false),
                &TheWayIn::SubmitAgain,
                &WhatTheServerIs::Connected,
                WhatTheBlendIs::There,
                None,
                None,
            )
            .unwrap()
            .came_from,
        });
        the_pick_is_dropped();
        assert_eq!(the_bar_in_hand(), None);
    }
    /// **The page goes where the press leads, and nowhere it was not told.**
    /// `web/a-new-job.js`, run under Node over stand-ins that record every
    /// call: the page over the server on the file's bytes; the server's Files
    /// with the bar asked for; the server list; the earlier entry; and — step
    /// 10c's *Files on the server* piece — that server's Files at the job's
    /// folder, handed over as its bytes before the tab is shown. **Since step
    /// 10c's owed piece 2 the page is handed the saved-with version Files read
    /// for that file** (`theSavedWithVersionReadFor`, stood in to answer for
    /// `s1`), where it was handed `""`. **Skipped, and says so, where no Node
    /// is installed.**
    #[test]
    fn the_page_goes_where_the_press_leads() {
        use std::fs;
        let folder = std::env::temp_dir().join(format!("smoozy-a-new-job-{}", std::process::id()));
        let _ = fs::remove_dir_all(&folder);
        fs::create_dir_all(&folder).expect("a scratch folder");
        let web = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        fs::copy(web.join("a-new-job.js"), folder.join("a-new-job.js")).expect("the page is copied");
        let calls = "(globalThis.calls ||= []).push";
        for (name, text) in [
            ("package.json", r#"{ "type": "module" }"#.to_string()),
            (
                "event-route.js",
                "export function whenTold(kind, handler) { ((globalThis.told ||= {})[kind] ||= []).push(handler); }"
                    .to_string(),
            ),
            (
                "the-rail.js",
                format!(
                    "export const A_SERVER = 'machineview'; export const ALL_SERVERS = 'serversview'; \
                     export function theCurrentPlaceIs(...args) {{ {calls}(['place', ...args]); }}"
                ),
            ),
            (
                "a-server.js",
                format!(
                    "export const FILES = 'files'; \
                     export function theServerWillOpenOn(...args) {{ {calls}(['tab', ...args]); }} \
                     export function thePanelOverItIsCollapsed(...args) {{ {calls}(['collapsed', ...args]); }}"
                ),
            ),
            (
                "render-settings.js",
                format!(
                    "export async function theSettingsAreOpenedOn(...args) {{ {calls}(['settings', ...args]); }}"
                ),
            ),
            (
                "files.js",
                format!(
                    "export function theBrowserIsAskedAgain() {{ {calls}(['browser']); }} \
                     export function theBrowserIsOpenedAt(...args) {{ {calls}(['opened-at', ...args]); }} \
                     export function theSavedWithVersionReadFor(server) {{ return server === 's1' ? '5.1.0' : ''; }}"
                ),
            ),
            (
                "the-history.js",
                format!("export function theHistoryEntryIsOpened(...args) {{ {calls}(['history', ...args]); }}"),
            ),
        ] {
            fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        let harness = format!(
            r#"
await import("./a-new-job.js");
const tell = (said) => globalThis.told["{A_NEW_JOB_OPENS}"].forEach((handler) => handler({{ said }}));
tell({{ goesTo: "the-page", server: "s1", file: "/root/é.blend" }});
tell({{ goesTo: "the-browser", server: "s2" }});
tell({{ goesTo: "the-servers" }});
tell({{ goesTo: "the-history", job: "j0" }});
tell({{ goesTo: "the-folder", server: "s4", folder: "/home/é" }});
tell({{ goesTo: "somewhere-else", server: "s3" }});
console.log(JSON.stringify(globalThis.calls));
"#
        );
        fs::write(folder.join("harness.mjs"), harness).expect("the harness is written");
        let ran = std::process::Command::new("node")
            .arg(folder.join("harness.mjs"))
            .current_dir(&folder)
            .output();
        let _ = fs::remove_dir_all(&folder);
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so web/a-new-job.js was not run");
            return;
        };
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value = serde_json::from_slice(&ran.stdout).expect("one answer");
        assert_eq!(
            got,
            serde_json::json!([
                ["place", "machineview", "s1"],
                ["settings", "s1", [47, 114, 111, 111, 116, 47, 195, 169, 46, 98, 108, 101, 110, 100], "5.1.0"],
                ["tab", "s2", "files"],
                ["place", "machineview", "s2"],
                ["browser"],
                ["place", "serversview"],
                ["history", "j0"],
                ["opened-at", "s4", [47, 104, 111, 109, 101, 47, 195, 169]],
                ["collapsed", "s4"],
                ["tab", "s4", "files"],
                ["place", "machineview", "s4"],
            ])
        );
    }

}
