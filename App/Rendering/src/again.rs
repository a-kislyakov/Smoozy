//! Submitting again, and carrying on — §6.1 A1, A1a, A1b, A1c, A1d, A2, A3;
//! §6.2 A4–A6's arithmetic; §6.3 A7, A7a, A7b, A8, A9's judgements; §6.4
//! A10, A11 (Rows 20, 21, 294, 472, 505).
//!
//! **A1: submitting again opens the settings page filled from the history
//! entry's kept setup and submits a new job with a new identity and a new
//! folder** — a fresh attempt the user asked for, never a re-sent command
//! and never a re-run. **A1b: the Simplify offer is entry-shaped, not
//! action-shaped** — drawn where the entry ended *failed*, and the action
//! that opened the page decides nothing. **A1c: where the offer is not
//! drawn, the kept setup's Simplify entries are not loaded** — not into the
//! store, not held by the page, not composed into the payload; the file's
//! own Simplify values render as the artist saved them. **A1d: the group has
//! file values behind it and the offer's values are the page's ordinary
//! composition rather than anybody's constants** — the ranges are the
//! documented ones and the boxes hold them.
//!
//! **A4, corrected 2026-09-02: *carry on from where this stopped* opens the
//! page with the range started AT the last frame that counts as already
//! there — that frame re-rendered, exactly as `Job files and outputs` C17
//! re-renders it on the machine's own attempt.** The arithmetic is one and
//! the same on both sides and must not diverge: the frame that counts as
//! already there at the boundary is precisely the one Smoozy **cannot**
//! vouch for — C15's limit says a short-but-not-empty file still counts —
//! and a frame that may be short must never be counted as already there by
//! an action the user cannot undo.
//!
//! **A7: a machine that answers but whose outputs cannot be read is told
//! apart from a machine that did not answer and from a folder that is gone.**
//! On the command channel a stat of the path itself separates the last two
//! by exit status — 0 against 1 — and a directory test does not; A7a: where
//! the stat is itself refused, the walk of the parent components is the
//! separator. **A8: a reading that did not complete is not a reading.**
//!
//! **Markers used in this file that belong to another document, each with
//! the document it belongs to.** `Job files and outputs` C1, C4, C7, C12,
//! C13, C15, C17, C18, C18a, C19, C21, C9, C26, I9a, N10. `Queue & History`
//! N6, N7, N10, N10a,
//! N17. `Render Settings` J1, J3, J4, J5, J11, Q2, Y7, Y15, Y15a, F1, F5.
//! `File Browser` N1. `State, identity and recovery` §5.4, D4.
//!
//! **`File Browser` P1 and P3 are not listed above and are written out at
//! each site**, because this plan defines a P1 and a P3 of its own.

use smoozy_job_files::inventory::WhatTheInventoryIs;
use smoozy_job_files::reckoning::{
    the_frame_is_already_there, the_last_frame_asked_for_that_is_already_there,
    WhatTheWalkOfTheFolderIs,
};
use smoozy_queue_history::ending::Ending;
use smoozy_render_settings::store::SimplifyEntries;
use smoozy_state::identities::{Identity, IdentityKind, NoRandomness};

/// **A1: a fresh act over kept setup — a new job identity, and so a new
/// folder** (`Job files and outputs` N3 derives the folder from the
/// identity). The kept setup is the entry's (`Queue & History` N10); what
/// the page does with it is `Render Settings`'.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AFreshAct {
    pub the_old_job: Identity,
    pub the_new_job: Identity,
}

pub fn submit_again(the_old_job: &Identity) -> Result<AFreshAct, NoRandomness> {
    Ok(AFreshAct {
        the_old_job: the_old_job.clone(),
        the_new_job: Identity::issue(IdentityKind::Job)?,
    })
}

/// **A1b, performed: the offer is drawn where the entry ended *failed*,
/// and on no other entry** — whichever of the three routes opened the page.
pub fn the_simplify_offer_is_drawn(the_entrys_ending: Ending) -> bool {
    the_entrys_ending == Ending::Failed
}

/// **A1c, performed: the kept setup's Simplify entries reach the store only
/// where the offer is drawn.** Elsewhere the new job carries no Simplify
/// entry and no pair reaches the launch material.
pub fn the_simplify_entries_to_load(offer_drawn: bool, kept: &SimplifyEntries) -> SimplifyEntries {
    if offer_drawn {
        kept.clone()
    } else {
        SimplifyEntries::default()
    }
}

/// **`Render Settings` Y15's list, read off the payload the history entry
/// kept** (`Queue & History` N10) — the frame range, the sample count, the
/// resolution percentage, the engine pick, the device choice, the view-layer
/// pins and Simplify. Every one is an entry because the user answered it:
/// `Render Settings` Y1 composes none for a setting left alone. **The version
/// pick is not among them** — [`the_kept_setup_from`] adds it only where the
/// entry records it as the user's own. Each pin's layer carries the
/// kept-setup provenance, as the scene does. *Written 2026-10-03, the 38's
/// piece 2: Y15 named what is loaded and nothing turned a kept payload into
/// it.*
///
/// **`Render Settings` F1: the frame text the user typed comes from the
/// record, beside the payload, and never out of it** — a kept payload carries
/// numbers only (`Render Settings` F3). `the_typed_frames` is the text the
/// entry's record holds (`Queue & History` P5b), and the field shows it with
/// the payload's parsed form, which is the form it was accepted with. **No
/// text, or no range under it, and the field writes the parsed form**, as for
/// an old entry. *Submit again* hands it; *Carry on* composes its own range and
/// hands none. Added 2026-10-25, the eighteenth notes, piece 7.
pub fn the_entries_the_payload_kept(
    payload: &smoozy_render_settings::payload::Payload,
    the_old_job: &str,
    the_typed_frames: Option<&str>,
) -> smoozy_render_settings::store::SceneEntries {
    use smoozy_render_settings::names::{NamedAnswer, Provenance};
    smoozy_render_settings::store::SceneEntries {
        prepare_simulations: Some(payload.prepare_simulations),
        frame_range: payload.frame_range.clone(),
        frame_range_text: payload
            .frame_range
            .as_ref()
            .and(the_typed_frames)
            .map(str::trim)
            .filter(|text| !text.is_empty())
            .map(str::to_string),
        sample_count: payload.sample_count,
        resolution_percentage: payload.resolution_percentage,
        engine_pick: payload.engine.as_ref().map(|engine| engine.pick),
        device: payload.device,
        pins: payload
            .view_layer_pins
            .iter()
            .map(|pin| smoozy_render_settings::store::Pin {
                layer: NamedAnswer {
                    name: pin.layer.clone(),
                    provenance: Provenance::AKeptSetup {
                        the_old_job: the_old_job.to_string(),
                    },
                },
                on: pin.on,
            })
            .collect(),
        simplify: payload.simplify.clone(),
        what_a_loaded_range_stands_on: None,
    }
}

/// **`Render Settings` Y15, from the payload the entry kept.**
/// `the_users_override` is the entry's version where `Queue & History` N10a
/// records it as the user's own pick, and nothing otherwise: `Render Settings` B4's
/// resolved-always pick is never loaded. **Nothing where the payload names no
/// scene** — Y15 writes under the kept scene name, and a job submitted with
/// none has no scene to key by; [`a_kept_setup_with_no_scene_is_loaded`] is
/// that case.
pub fn the_kept_setup_from(
    payload: &smoozy_render_settings::payload::Payload,
    the_old_job: &str,
    the_typed_frames: Option<&str>,
    the_users_override: Option<smoozy_render_settings::file_half::BlenderIdentity>,
) -> Option<smoozy_render_settings::store::KeptSetup> {
    use smoozy_render_settings::names::{NamedAnswer, Provenance};
    use smoozy_render_settings::store::{KeptSetup, KeptVersion};
    let scene = payload.scene.as_ref()?;
    Some(KeptSetup {
        scene: NamedAnswer {
            name: scene.name.clone(),
            provenance: Provenance::AKeptSetup {
                the_old_job: the_old_job.to_string(),
            },
        },
        entries: the_entries_the_payload_kept(payload, the_old_job, the_typed_frames),
        version: match the_users_override {
            Some(pick) => KeptVersion::TheUsersOverride(pick),
            None => KeptVersion::ResolvedAutomatically,
        },
    })
}

/// **`Render Settings` Y15 for an entry whose job named no scene** — a job
/// submitted before its file was read (`Queue & History` N22). With no kept
/// scene name to key by, the entries go where a page with no scene in hand
/// keeps its answers: the pre-read set (`Render Settings` J3), with no tick
/// and no pin, neither of which that set can hold. **`Render Settings` Y15a's condition is
/// [`the_simplify_entries_to_load`]'s, unchanged**, and the version pick only
/// where the entry records it as the user's own. **Anything the key already
/// held in that set is handed back first**, so the page shows the entry's
/// setup and nothing left over. *Added 2026-10-03, the 38's piece 2: Y15's
/// "the kept scene name arrives with the setup" does not hold for such an
/// entry, and this is the lean reading of it.*
pub fn a_kept_setup_with_no_scene_is_loaded(
    store: &mut smoozy_render_settings::store::Store,
    file: smoozy_render_settings::file_half::FileKey,
    entries: &smoozy_render_settings::store::SceneEntries,
    the_users_override: Option<smoozy_render_settings::file_half::BlenderIdentity>,
    offer_drawn: bool,
) {
    use smoozy_render_settings::store::{Answer, PageInHand, Where};
    // **`Render Settings` Y15b: what stood in the set, kept before it is cleared.**
    let beneath = store.what_stands_beneath_a_load_of_the_pre_read_set(&file);
    store.hand_everything_back(file.clone(), PageInHand::NoSceneInHand);
    let ticked: Vec<_> = store
        .per_file(&file)
        .map(|held| held.scenes_ticked.iter().map(|one| one.name.clone()).collect())
        .unwrap_or_default();
    for scene in ticked {
        store.untick_a_scene(file.clone(), &scene);
    }
    let simplify = the_simplify_entries_to_load(offer_drawn, &entries.simplify);
    // **`Render Settings` F1: a range the entry holds typed text for goes in
    // with that text**, the two together; one with none goes in as numbers.
    let typed_range = match (&entries.frame_range, &entries.frame_range_text) {
        (Some(form), Some(text)) => Some((form.clone(), text.clone())),
        _ => None,
    };
    let answers = [
        entries.prepare_simulations.map(Answer::PrepareSimulations),
        entries
            .frame_range
            .clone()
            .filter(|_| typed_range.is_none())
            .map(Answer::FrameRange),
        entries.sample_count.map(Answer::SampleCount),
        entries.resolution_percentage.map(Answer::ResolutionPercentage),
        entries.engine_pick.map(Answer::EnginePick),
        entries.device.map(Answer::Device),
        simplify.use_simplify.map(Answer::SimplifySwitch),
        simplify.subdivision_render.map(Answer::SimplifySubdivision),
        simplify.child_particles_render.map(Answer::SimplifyChildParticles),
        simplify.texture_limit_render.map(Answer::SimplifyTextureLimit),
    ];
    for answer in answers.into_iter().flatten() {
        store.answer(Where::ThePreReadSet { file: file.clone() }, answer);
    }
    if let Some((form, text)) = typed_range {
        store.answer_the_typed_range(Where::ThePreReadSet { file: file.clone() }, form, &text);
    }
    store.the_pre_read_set_was_loaded(file.clone(), beneath);
    if let Some(pick) = the_users_override {
        store.pick_a_version(file, pick);
    }
}

/// **A4 and A5: the carried-on range, in the job's own asked order, opening
/// at the frame the page starts from.** `from` is the boundary frame A4
/// reckoned — or the figure the user typed over it, which A5 makes theirs to
/// move — and the range is every frame the job asked for from there on.
/// **The asked order is ascending** (`Render Settings` F1 tidies a list, and a
/// triple only counts forward), so a typed figure between two asked frames
/// opens at the next one. Written as one even stretch where the rest is one,
/// so the page's field reads *60-240* rather than a list of numbers.
/// **Nothing where no asked frame is at or after `from`.**
pub fn the_carried_on_range(
    the_asked_order: &[i64],
    from: i64,
) -> Option<smoozy_render_settings::frames::FrameForm> {
    use smoozy_render_settings::frames::FrameForm;
    let at = the_asked_order.iter().position(|frame| *frame >= from)?;
    let rest = &the_asked_order[at..];
    let first = rest[0];
    let last = *rest.last()?;
    if rest.len() == 1 {
        return Some(FrameForm::Triple {
            first,
            last,
            step: 1,
        });
    }
    let step = rest[1] - rest[0];
    let even = step > 0 && rest.windows(2).all(|pair| pair[1] - pair[0] == step);
    Some(if even {
        FrameForm::Triple { first, last, step }
    } else {
        FrameForm::List(rest.to_vec())
    })
}

/// **A1a: where the pick-a-file browser opens on a submit-again whose
/// `.blend` is not on a connected machine — and Smoozy picks no machine.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhereTheBrowserOpens {
    /// The machine the job ran on is still connected: the browser opens on
    /// it, A10's check answering whether the file is still there.
    TheMachineTheJobRanOn,
    /// The machine is gone and a machine is being browsed this run: the
    /// browser opens on it, and the user moves from inside it.
    TheMachineBeingBrowsedThisRun,
    /// Exactly one machine is connected: the only place a browser can
    /// open, so opening there picks nothing.
    TheOneConnectedMachine,
    /// **§9 item 24: more than one connected and none browsed this run —
    /// no document names the opening machine and none is invented here.**
    /// The cost is a visible wrong starting position and never a wrong job.
    NoDocumentNamesTheOpeningMachine { connected: usize },
    /// No machine is connected at all: the file-finding half is
    /// unavailable rather than failing; the page opens on the kept setup
    /// and waits.
    UnavailableUntilAMachineIsConnected,
}

pub fn where_the_browser_opens(
    the_jobs_machine_is_still_connected: bool,
    a_machine_is_being_browsed_this_run: bool,
    connected_machines: usize,
) -> WhereTheBrowserOpens {
    if the_jobs_machine_is_still_connected {
        return WhereTheBrowserOpens::TheMachineTheJobRanOn;
    }
    if connected_machines == 0 {
        return WhereTheBrowserOpens::UnavailableUntilAMachineIsConnected;
    }
    if a_machine_is_being_browsed_this_run {
        return WhereTheBrowserOpens::TheMachineBeingBrowsedThisRun;
    }
    if connected_machines == 1 {
        return WhereTheBrowserOpens::TheOneConnectedMachine;
    }
    WhereTheBrowserOpens::NoDocumentNamesTheOpeningMachine {
        connected: connected_machines,
    }
}

/// **A1d: the two typed boxes' ranges — documented, the same in all four
/// versions — each refusal stating the range alone**, because what Blender
/// does with an out-of-range assignment is established nowhere (§9 item
/// 23). The Texture Limit control is a pick over the file's own enumeration
/// and refuses nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateTheRange {
    /// Whole numbers from 0 to 32767 and nothing else.
    MaxSubdivisionTakesWholeNumbersFromZeroTo32767,
    /// A number from 0 to 1 and nothing else.
    ChildParticlesTakesANumberFromZeroToOne,
}

pub fn the_max_subdivision_box(typed: &str) -> Result<u32, StateTheRange> {
    let text = typed.trim();
    if text.is_empty() || !text.bytes().all(|b| b.is_ascii_digit()) {
        return Err(StateTheRange::MaxSubdivisionTakesWholeNumbersFromZeroTo32767);
    }
    match text.parse::<u32>() {
        Ok(value) if value <= 32767 => Ok(value),
        _ => Err(StateTheRange::MaxSubdivisionTakesWholeNumbersFromZeroTo32767),
    }
}

pub fn the_child_particles_box(typed: &str) -> Result<f64, StateTheRange> {
    match typed.trim().parse::<f64>() {
        Ok(value) if value.is_finite() && (0.0..=1.0).contains(&value) => Ok(value),
        _ => Err(StateTheRange::ChildParticlesTakesANumberFromZeroToOne),
    }
}

/// **A1d: the Texture Limit control is a pick over the file's own
/// enumeration, and refuses nothing** — off, or a ceiling of 128, 256, 512,
/// 1024, 2048, 4096 or 8192 pixels, the same in all four versions
/// (`Preparation/Blender Research/Comparison/What a render demands.md`,
/// question 6; each version's findings quote Cycles' own `enum_texture_limit`,
/// whose identifiers these are, in its order). The page lists these and
/// nothing else, so no free value is typable.
pub const THE_TEXTURE_LIMITS: [&str; 8] = ["OFF", "128", "256", "512", "1024", "2048", "4096", "8192"];

/// **The Texture Limit pick, taken**: one of [`THE_TEXTURE_LIMITS`], or
/// nothing where what arrived is not one of them — never a refusal, since the
/// control offers no other value (A1d).
pub fn the_texture_limit_pick(picked: &str) -> Option<&'static str> {
    THE_TEXTURE_LIMITS.iter().copied().find(|one| *one == picked)
}

/// **A1d: the render half is set and the viewport half is left exactly as
/// saved**, and **no constant of Smoozy's exists anywhere** — the offer's
/// values are the file's read-back values where a reading stands. Kept by
/// there being no starting value in this crate; stated so it can be cited.
pub const NO_STARTING_VALUE_FOR_SIMPLIFY_EXISTS_ANYWHERE: bool = true;

/// **A2: the machine-side re-run after an interruption is the other thing,
/// and this document adds nothing to it** — the machine's own act, once,
/// from durable state, reading the immutable manifest, the launch identical
/// by L4's construction, L30 standing over it.
pub const THE_MACHINES_RE_RUN_IS_NOT_A_SUBMIT_AGAIN: bool = true;

/// **A3: *try it again* from a phone stays unclaimed here** — the tap
/// composes no settings page and reaches no composition of this crate's.
/// **The ruling of 2026-09-02 reopens a spine question — the desktop as the
/// issuer of identities for a tap made with nobody connected (`State,
/// identity and recovery` §2.2) — and routes it; nothing is built for it
/// here and no clause of §4 is relaxed.** Noted for the runner.
pub const A3_ROUTES_A_SPINE_QUESTION_AND_NOTHING_IS_BUILT_FOR_IT: bool = true;

/// **A4, performed: the range opens AT the last frame that counts as
/// already there.** `None` where there is no such frame, which is A6's
/// not-offered case.
pub fn the_carry_on_range_starts_at(the_last_frame_already_there: Option<i64>) -> Option<i64> {
    the_last_frame_already_there
}

/// **A4: the figure is read from the machine when the action is taken,
/// never remembered from when the job ended**, and the inventory is the one
/// for the map in force — read from the latest attempt record that carries
/// a frame list, the same record P1 step 6 reads. The reading is
/// period-held and nothing durable is written from it.
pub const THE_FIGURE_IS_READ_WHEN_THE_ACTION_IS_TAKEN_AND_NOTHING_DURABLE_IS_WRITTEN: bool = true;

/// **A4's two readings of the machine, as the acting half hands them
/// back** — the walk of the job output folder, and the map in force taken
/// off the machine's own latest attempt record that carries a frame list.
///
/// **Both are readings taken at the moment the action is taken**, never
/// values remembered from when the job ended, which is the whole of what
/// A4 asks for and the whole of what
/// [`the_last_frame_already_there_on_the_machine`] reckons over.
#[derive(Debug, Clone, Copy)]
pub struct WhatTheCarryOnRead<'a> {
    /// **`Job files and outputs` C7's one walk of the job output folder and
    /// everything under it**, taken by the desktop while it is connected —
    /// which C4 of that document grants in as many words, as a reading it
    /// shows and never acts on.
    pub the_walk: &'a WhatTheWalkOfTheFolderIs,
    /// The folder the walk's paths are relative to.
    pub the_job_output_folder: &'a str,
    /// **The inventory for the map in force on the attempt being reckoned**
    /// (`Job files and outputs` C7). **Composed by that crate and never
    /// here**, from the map the attempt record below carries.
    pub the_inventory_for_the_map_in_force: &'a WhatTheInventoryIs,
    /// **That record's frame list, in the order it was written** — the same
    /// record P1 step 6 reads the list from, found by `Job files and
    /// outputs` C18a's rule. **The order is what the answer rests on**
    /// (C19): the highest-numbered frame present need not be the last one
    /// finished.
    pub the_frame_list_asked_for: &'a [i64],
}

/// **Why A4's reading could not be completed, so A9's could-not-check case
/// takes it.** A8: a reading that did not complete is not a reading.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheMachineCouldNotBeChecked {
    /// **A8, and `Job files and outputs` C9**: the walk did not complete, so
    /// there is nothing to reckon against — and never a folder found empty.
    TheWalkOfTheJobOutputFolderDidNotComplete,
    /// **A4: the map in force is read from the machine's own latest attempt
    /// record that carries a frame list**, and no record on the machine
    /// carries one. There is no list to walk backwards and no map to take
    /// the inventory for.
    NoAttemptRecordOnTheMachineCarriesAFrameList,
    /// **A4: no figure is composed against a guessed inventory.** The names
    /// the frames' files would carry could not be predicted (`Job files and
    /// outputs` I9a), so no frame can be called already there. **The
    /// alternative is composing the figure as *no frame*, which A6 turns
    /// into *the job finished no frame* — a claim about the render that
    /// this reading does not support.**
    TheInventoryForTheMapInForceCouldNotBeComposed,
}

/// **A4, performed: the last frame that counts as already there, reckoned
/// on the desk against the machine's own walk and the machine's own
/// inventory.**
///
/// **The arithmetic is one and the same on both sides and must not
/// diverge**, so it is `Job files and outputs`' own — C12 and C13's
/// already-there rule with the zero-byte refusal, and C17 and C18's walk
/// backwards through the list Smoozy asked for. **Nothing here is a second
/// copy of either.**
///
/// **What it is not.** This is A4's boundary figure and never K18's
/// completeness statement: [`crate::which_frames`] holds that one, K19
/// composes it once on the machine at the ending, and no set is produced
/// here for anything to mistake for it.
///
/// `Ok(None)` is A6's *the job finished no frame*: the reading completed
/// and named no frame already there.
pub fn the_last_frame_already_there_on_the_machine(
    read: WhatTheCarryOnRead<'_>,
) -> Result<Option<i64>, TheMachineCouldNotBeChecked> {
    let WhatTheWalkOfTheFolderIs::ItCompleted(entries) = read.the_walk else {
        return Err(TheMachineCouldNotBeChecked::TheWalkOfTheJobOutputFolderDidNotComplete);
    };
    let WhatTheInventoryIs::It(inventory) = read.the_inventory_for_the_map_in_force else {
        return Err(TheMachineCouldNotBeChecked::TheInventoryForTheMapInForceCouldNotBeComposed);
    };
    if read.the_frame_list_asked_for.is_empty() {
        return Err(TheMachineCouldNotBeChecked::NoAttemptRecordOnTheMachineCarriesAFrameList);
    }
    Ok(the_last_frame_asked_for_that_is_already_there(
        read.the_frame_list_asked_for,
        |frame| {
            the_frame_is_already_there(inventory, frame, entries, read.the_job_output_folder)
        },
    ))
}

/// **A4 and A9 together: which of the two routes the page's figure came
/// from**, so a remembered figure is never presented as a checked one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheCarryOnsFigure {
    /// **A4's own route**: read from the machine when the action was taken.
    /// `None` is A6's *the job finished no frame*.
    ReadFromTheMachine(Option<i64>),
    /// **A9's exception, and it stopped being the whole of the behaviour on
    /// 2026-09-05.** Until then nothing in this crate walked a folder or
    /// read an attempt record, so this was the only figure the page could
    /// ever open on — Smoozy told every user it could not check a machine
    /// that was there and readable.
    TheMachineCouldNotBeChecked {
        why: TheMachineCouldNotBeChecked,
        case: TheCouldNotCheckCase,
    },
}

/// **A4 with A9 behind it: the figure the page opens on.**
///
/// The kept figure is `Queue & History` N6's and is read only where the
/// machine could not be checked — **never as a fallback the user is not
/// told about**, which is what [`TheCouldNotCheckCase`]'s two statements
/// carry.
pub fn the_carry_ons_figure_over(
    read: WhatTheCarryOnRead<'_>,
    the_kept_figure: Option<i64>,
) -> TheCarryOnsFigure {
    match the_last_frame_already_there_on_the_machine(read) {
        Ok(frame) => TheCarryOnsFigure::ReadFromTheMachine(frame),
        Err(why) => TheCarryOnsFigure::TheMachineCouldNotBeChecked {
            why,
            case: the_could_not_check_case(the_kept_figure),
        },
    }
}

/// **A5: the machine's own reckoning at the fresh attempt's start is the
/// one the render uses**; the page's figure is a starting point the user
/// can type over, shown with what it rests on, C15's limit carried in the
/// spec's own words.
pub const THE_PAGES_FIGURE_IS_A_STARTING_POINT_AND_C15S_LIMIT_IS_CARRIED_ON_IT: bool = true;

/// **A6: where there is no frame to start at, the action is not offered,
/// and the two cases are told apart by records.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyNotOffered {
    /// The reckoning's answer: no frame finished.
    TheJobFinishedNoFrame,
    /// N7's no-figure case: no ending Smoozy ever learned.
    NoEndingWasEverLearned,
}

pub fn is_the_carry_on_offered(
    an_ending_was_learned: bool,
    the_last_frame_already_there: Option<i64>,
) -> Result<i64, WhyNotOffered> {
    if !an_ending_was_learned {
        return Err(WhyNotOffered::NoEndingWasEverLearned);
    }
    the_carry_on_range_starts_at(the_last_frame_already_there)
        .ok_or(WhyNotOffered::TheJobFinishedNoFrame)
}

/// **A7's inputs: what the reading of the job output folder gave** — a
/// raised condition where the request was not answered, else the stat's
/// exit status on the path itself and whether the listing was refused, with
/// A7a's walk where the stat was itself refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheReadingGave {
    /// The request was not answered — the connection or supervisor
    /// condition, raised, never inferred.
    TheRequestWasNotAnswered,
    /// The stat of the path itself, and the listing.
    TheStat {
        exit_status: i32,
        the_listing_was_refused: bool,
        /// A7a: the walk of the parent components, where the stat exited
        /// 1 — which parent stopped it, and whether it is a directory.
        the_walk: Option<TheWalk>,
    },
}

/// **A7a's walk answer, and the plan gained its third on 2026-09-05.**
///
/// The clause named a refusal and an absence and named nothing for the
/// commonest case there is — every parent an ordinary readable directory over
/// a path that is simply not there — and it read an exit-1 answer on a parent
/// as *this parent is not a directory*. **Its own recorded evidence says the
/// opposite of both**, and the repaired clause is what these two answers are.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheWalk {
    /// **A parent's own directory test did not answer *true*, which is
    /// evidence about what sits above that parent and never about the
    /// parent.** The macOS run of 2026-08-21 put the file test on a path
    /// *under* an unreadable parent and it exited 1 either way, so the walk
    /// could not reach past something higher up: a refusal.
    AParentDidNotAnswerSoSomethingAboveItRefuses,
    /// **Every parent answered as a directory and the stat still exited 1,
    /// so the path itself is absent.** A7's own evidence in as many words:
    /// stat 0 for the folder present and unreadable, stat 1 for the folder
    /// gone.
    EveryParentAnsweredAndThePathItselfIsAbsent,
}

/// **The three-way distinction, and each said as itself.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreeWay {
    TheMachineDidNotAnswer,
    TheFolderIsGone,
    ThisLoginMayNotReadIt,
    /// The stat exited 1 and no walk was taken: *gone, or refused higher
    /// up*, and the walk is what names which.
    GoneOrRefusedHigherUpAndTheWalkNamesWhich,
    /// The read succeeded.
    ItIsThere,
}

/// **A7 and A7a, performed by exit status alone; no wording of the
/// machine's is mined for the cause** (`Job files and outputs` C26).
pub fn separate(reading: &WhatTheReadingGave) -> ThreeWay {
    match reading {
        WhatTheReadingGave::TheRequestWasNotAnswered => ThreeWay::TheMachineDidNotAnswer,
        WhatTheReadingGave::TheStat {
            exit_status: 0,
            the_listing_was_refused: true,
            ..
        } => ThreeWay::ThisLoginMayNotReadIt,
        WhatTheReadingGave::TheStat {
            exit_status: 0,
            the_listing_was_refused: false,
            ..
        } => ThreeWay::ItIsThere,
        WhatTheReadingGave::TheStat { the_walk, .. } => match the_walk {
            Some(TheWalk::AParentDidNotAnswerSoSomethingAboveItRefuses) => {
                ThreeWay::ThisLoginMayNotReadIt
            }
            Some(TheWalk::EveryParentAnsweredAndThePathItselfIsAbsent) => ThreeWay::TheFolderIsGone,
            None => ThreeWay::GoneOrRefusedHigherUpAndTheWalkNamesWhich,
        },
    }
}

/// **A8: a reading that did not complete is not a reading** — the
/// reckoning and the figure proceed only on a reading that ended
/// successfully; anything else is A9's could-not-check case.
pub fn a_reading_that_did_not_complete_is_not_a_reading(ended_successfully: bool) -> bool {
    !ended_successfully
}

/// **A9: where the machine cannot be checked, the page opens on the figure
/// `Queue & History` kept and says plainly that it could not check the
/// machine and this is what the entry recorded** — read by A4's arithmetic,
/// at that frame. **The action is not refused, and a remembered figure is
/// never presented as a checked one. This is Row 21.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheCouldNotCheckCase {
    pub the_kept_figure: Option<i64>,
    pub says_it_could_not_check_the_machine: bool,
    pub presented_as_checked: bool,
}

pub fn the_could_not_check_case(the_kept_figure: Option<i64>) -> TheCouldNotCheckCase {
    TheCouldNotCheckCase {
        the_kept_figure: the_carry_on_range_starts_at(the_kept_figure),
        says_it_could_not_check_the_machine: true,
        presented_as_checked: false,
    }
}

/// **A10 and A11: whether the `.blend` is where the manifest says is
/// answered by a file-attribute read of that one path, under the same
/// three-way distinction; where the file is gone and the machine is
/// otherwise there, Smoozy says so and opens the file browser on that
/// machine** — the same entry point A1a uses, invoked with the machine and
/// nothing else decided inside it.
pub fn what_a_gone_blend_leads_to(reading: &WhatTheReadingGave) -> Option<WhereTheBrowserOpens> {
    match separate(reading) {
        ThreeWay::TheFolderIsGone => Some(WhereTheBrowserOpens::TheMachineTheJobRanOn),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn submitting_again_issues_a_new_job_identity_and_so_a_new_folder() {
        let old = Identity::issue(IdentityKind::Job).unwrap();
        let fresh = submit_again(&old).unwrap();
        assert_eq!(fresh.the_old_job, old);
        assert_ne!(fresh.the_new_job, old);
        assert_eq!(fresh.the_new_job.kind(), IdentityKind::Job);
        const _: () = assert!(THE_MACHINES_RE_RUN_IS_NOT_A_SUBMIT_AGAIN);
        const _: () = assert!(A3_ROUTES_A_SPINE_QUESTION_AND_NOTHING_IS_BUILT_FOR_IT);
    }

    /// **A1b and A1c together**: a carry-on from a stopped entry draws no
    /// offer and loads no Simplify; a failed entry draws it.
    #[test]
    fn the_offer_is_entry_shaped_and_the_kept_simplify_entries_load_only_where_it_is_drawn() {
        assert!(the_simplify_offer_is_drawn(Ending::Failed));
        for ending in [Ending::Finished, Ending::Stopped, Ending::GivenUp] {
            assert!(!the_simplify_offer_is_drawn(ending));
        }
        let kept = SimplifyEntries {
            use_simplify: Some(true),
            subdivision_render: Some(2),
            child_particles_render: Some(0.5),
            texture_limit_render: Some("1024".to_string()),
        };
        assert_eq!(the_simplify_entries_to_load(true, &kept), kept);
        assert_eq!(
            the_simplify_entries_to_load(false, &kept),
            SimplifyEntries::default()
        );
        assert!(the_simplify_entries_to_load(false, &kept).is_empty());
    }

    fn a_kept_payload() -> smoozy_render_settings::payload::Payload {
        use smoozy_render_settings::names::{AnchorState, ExactName, Provenance};
        use smoozy_render_settings::payload::{Payload, SceneEntry, VersionPick};
        use smoozy_render_settings::view_layers::PinEntry;
        let to_anchor = AnchorState::ToAnchor {
            provenance: Provenance::AKeptSetup {
                the_old_job: "job-0".to_string(),
            },
        };
        let mut payload = Payload {
            prepare_simulations: false,
            version_pick: VersionPick::of(&crate::material::testing::a_blender()),
            scene: Some(SceneEntry {
                name: ExactName::as_the_reading_produced_it("Kitchen"),
                anchor: to_anchor.clone(),
            }),
            frame_range: Some(
                smoozy_render_settings::frames::parse_the_frame_field("1-240").unwrap(),
            ),
            sample_count: Some(512),
            resolution_percentage: Some(50),
            engine: None,
            device: Some(smoozy_render_settings::device::DeviceChoice::TheProcessor),
            view_layer_pins: vec![PinEntry {
                layer: ExactName::as_the_reading_produced_it("FX"),
                anchor: to_anchor,
                on: false,
            }],
            simplify: SimplifyEntries::default(),
            the_check_is_written_as_a_picture: false,
            the_check_writes_into: None,
        };
        payload.simplify.use_simplify = Some(true);
        payload
    }

    /// **`Render Settings` Y15's list off the kept payload**: every
    /// answered entry, the scene and each pin with the kept-setup provenance,
    /// and the version pick only as the user's own.
    #[test]
    fn the_kept_payload_is_y15s_list_and_the_version_loads_only_where_it_was_the_users() {
        use smoozy_render_settings::names::{ExactName, NamedAnswer, Provenance};
        use smoozy_render_settings::store::KeptVersion;
        let blender = crate::material::testing::a_blender();
        let mut payload = a_kept_payload();
        let kept = the_kept_setup_from(&payload, "job-1", None, None).unwrap();
        let kept_name = |name: &str| NamedAnswer {
            name: ExactName::as_the_reading_produced_it(name),
            provenance: Provenance::AKeptSetup {
                the_old_job: "job-1".to_string(),
            },
        };
        assert_eq!(kept.scene, kept_name("Kitchen"));
        assert_eq!(kept.version, KeptVersion::ResolvedAutomatically);
        assert_eq!(kept.entries.frame_range, payload.frame_range);
        assert_eq!(kept.entries.prepare_simulations, Some(false));
        assert_eq!(kept.entries.sample_count, Some(512));
        assert_eq!(kept.entries.resolution_percentage, Some(50));
        assert_eq!(kept.entries.simplify.use_simplify, Some(true));
        assert_eq!(kept.entries.pins.len(), 1);
        assert_eq!(kept.entries.pins[0].layer, kept_name("FX"));
        assert!(!kept.entries.pins[0].on);
        assert_eq!(
            the_kept_setup_from(&payload, "job-1", None, Some(blender.clone()))
                .unwrap()
                .version,
            KeptVersion::TheUsersOverride(blender)
        );
        payload.scene = None;
        assert_eq!(the_kept_setup_from(&payload, "job-1", None, None), None);
    }

    /// **A no-scene entry loads into the pre-read set**, under Y15a's
    /// condition, with no tick, no pin and nothing left over from before.
    #[test]
    fn a_kept_setup_with_no_scene_lands_in_the_pre_read_set_under_y15as_condition() {
        use smoozy_render_settings::file_half::FileKey;
        use smoozy_render_settings::store::{Answer, Store, Where};
        let file = FileKey {
            server: Identity::issue(IdentityKind::LocalServer).unwrap(),
            path: "/root/kitchen.blend".to_string(),
        };
        let entries = the_entries_the_payload_kept(&a_kept_payload(), "job-1", None);
        let mut store = Store::new();
        store.answer(
            Where::ThePreReadSet { file: file.clone() },
            Answer::ResolutionPercentage(10),
        );
        a_kept_setup_with_no_scene_is_loaded(&mut store, file.clone(), &entries, None, false);
        let read = store.the_answers_composition_reads(&file, None);
        assert_eq!(read.prepare_simulations, Some(false));
        assert_eq!(read.sample_count, Some(512));
        assert_eq!(read.resolution_percentage, Some(50));
        assert!(read.simplify.is_empty());
        assert!(read.pins.is_empty());
        assert_eq!(store.per_file(&file).unwrap().version_pick, None);
        let blender = crate::material::testing::a_blender();
        let mut store = Store::new();
        a_kept_setup_with_no_scene_is_loaded(&mut store, file.clone(), &entries, Some(blender.clone()), true);
        assert_eq!(
            store.the_answers_composition_reads(&file, None).simplify.use_simplify,
            Some(true)
        );
        assert_eq!(store.per_file(&file).unwrap().version_pick, Some(blender));
    }

    /// **`Render Settings` F1 and `Queue & History` P5b: the frame text the
    /// entry's record holds comes in with the payload's parsed form**, on both
    /// routes — a scene-named setup and the pre-read set — so the field shows
    /// *1,2-4* as typed; **with no text, or no range under it, the field writes
    /// the parsed form**, as for an old entry. The eighteenth notes, piece 7.
    #[test]
    fn the_typed_frames_come_in_with_the_parsed_form_and_without_them_the_numbers() {
        use smoozy_render_settings::file_half::FileKey;
        use smoozy_render_settings::frames::FrameForm;
        use smoozy_render_settings::store::Store;
        let mut payload = a_kept_payload();
        payload.frame_range = Some(FrameForm::List(vec![1, 2, 3, 4]));
        // A scene-named setup.
        let kept = the_kept_setup_from(&payload, "job-1", Some(" 1,2-4 "), None).unwrap();
        assert_eq!(kept.entries.frame_range, payload.frame_range);
        assert_eq!(kept.entries.frame_range_text.as_deref(), Some("1,2-4"));
        let file = FileKey {
            server: Identity::issue(IdentityKind::LocalServer).unwrap(),
            path: "/root/kitchen.blend".to_string(),
        };
        let mut store = Store::new();
        store.load_a_kept_setup(file.clone(), kept.clone(), false);
        let read = store.the_answers_composition_reads(&file, Some(&kept.scene.name));
        assert_eq!(read.frame_range, payload.frame_range);
        assert_eq!(read.frame_range_text.as_deref(), Some("1,2-4"));
        // No text: numbers only.
        assert_eq!(the_kept_setup_from(&payload, "job-1", None, None).unwrap().entries.frame_range_text, None);
        // A text with no range under it is no text.
        let mut no_range = payload.clone();
        no_range.frame_range = None;
        assert_eq!(the_entries_the_payload_kept(&no_range, "job-1", Some("1,2-4")).frame_range_text, None);
        // The pre-read set, for an entry that named no scene.
        let entries = the_entries_the_payload_kept(&payload, "job-1", Some("1,2-4"));
        let mut store = Store::new();
        a_kept_setup_with_no_scene_is_loaded(&mut store, file.clone(), &entries, None, false);
        let read = store.the_answers_composition_reads(&file, None);
        assert_eq!(read.frame_range, payload.frame_range);
        assert_eq!(read.frame_range_text.as_deref(), Some("1,2-4"));
        let entries = the_entries_the_payload_kept(&payload, "job-1", None);
        let mut store = Store::new();
        a_kept_setup_with_no_scene_is_loaded(&mut store, file.clone(), &entries, None, false);
        let read = store.the_answers_composition_reads(&file, None);
        assert_eq!(read.frame_range, payload.frame_range);
        assert_eq!(read.frame_range_text, None);
    }

    /// **A4 and A5: the range opens at the frame, in the asked order, and is
    /// written as a stretch where it is one.**
    #[test]
    fn the_carried_on_range_opens_at_the_frame_and_keeps_the_asked_order() {
        use smoozy_render_settings::frames::FrameForm;
        let order: Vec<i64> = (1..=240).collect();
        assert_eq!(
            the_carried_on_range(&order, 60),
            Some(FrameForm::Triple {
                first: 60,
                last: 240,
                step: 1
            })
        );
        let stepped: Vec<i64> = (0..10).map(|at| 1 + at * 5).collect();
        // A typed figure between two asked frames opens at the next one.
        assert_eq!(
            the_carried_on_range(&stepped, 12),
            Some(FrameForm::Triple {
                first: 16,
                last: 46,
                step: 5
            })
        );
        assert_eq!(
            the_carried_on_range(&[1, 2, 7, 9], 2),
            Some(FrameForm::List(vec![2, 7, 9]))
        );
        assert_eq!(
            the_carried_on_range(&[1, 2, 3], 3),
            Some(FrameForm::Triple {
                first: 3,
                last: 3,
                step: 1
            })
        );
        assert_eq!(the_carried_on_range(&[1, 2, 3], 4), None);
    }

    #[test]
    fn where_the_browser_opens_and_the_one_case_no_document_names() {
        assert_eq!(
            where_the_browser_opens(true, false, 3),
            WhereTheBrowserOpens::TheMachineTheJobRanOn
        );
        assert_eq!(
            where_the_browser_opens(false, true, 3),
            WhereTheBrowserOpens::TheMachineBeingBrowsedThisRun
        );
        assert_eq!(
            where_the_browser_opens(false, false, 1),
            WhereTheBrowserOpens::TheOneConnectedMachine
        );
        assert_eq!(
            where_the_browser_opens(false, false, 2),
            WhereTheBrowserOpens::NoDocumentNamesTheOpeningMachine { connected: 2 }
        );
        assert_eq!(
            where_the_browser_opens(false, false, 0),
            WhereTheBrowserOpens::UnavailableUntilAMachineIsConnected
        );
    }

    #[test]
    fn the_two_typed_boxes_hold_the_documented_ranges_and_state_the_range_alone() {
        assert_eq!(the_max_subdivision_box("0"), Ok(0));
        assert_eq!(the_max_subdivision_box(" 32767 "), Ok(32767));
        for wrong in ["32768", "-1", "2.5", "", "two"] {
            assert_eq!(
                the_max_subdivision_box(wrong),
                Err(StateTheRange::MaxSubdivisionTakesWholeNumbersFromZeroTo32767),
                "{wrong}"
            );
        }
        assert_eq!(the_child_particles_box("0"), Ok(0.0));
        assert_eq!(the_child_particles_box("0.5"), Ok(0.5));
        assert_eq!(the_child_particles_box("1"), Ok(1.0));
        for wrong in ["1.01", "-0.1", "", "half", "inf", "NaN"] {
            assert_eq!(
                the_child_particles_box(wrong),
                Err(StateTheRange::ChildParticlesTakesANumberFromZeroToOne),
                "{wrong}"
            );
        }
        const _: () = assert!(NO_STARTING_VALUE_FOR_SIMPLIFY_EXISTS_ANYWHERE);
    }

    /// **A1d: the Texture Limit pick takes the enumeration's own identifiers
    /// and nothing else**, refusing nothing.
    #[test]
    fn the_texture_limit_pick_takes_the_enumeration_and_nothing_else() {
        for one in THE_TEXTURE_LIMITS {
            assert_eq!(the_texture_limit_pick(one), Some(one));
        }
        for not_one in ["", "off", "No Limit", "100", "16384", " 1024"] {
            assert_eq!(the_texture_limit_pick(not_one), None, "{not_one}");
        }
    }

    /// **The fourth of the four named faults, from the carry-on's side: a
    /// frame counted as already there.** The range opens AT the boundary
    /// frame — the one C17 re-renders — and never after it.
    #[test]
    fn the_carry_on_opens_at_the_last_frame_already_there_and_never_after_it() {
        assert_eq!(the_carry_on_range_starts_at(Some(17)), Some(17));
        assert_ne!(the_carry_on_range_starts_at(Some(17)), Some(18));
        assert_eq!(the_carry_on_range_starts_at(None), None);
        assert_eq!(is_the_carry_on_offered(true, Some(17)), Ok(17));
        assert_eq!(
            is_the_carry_on_offered(true, None),
            Err(WhyNotOffered::TheJobFinishedNoFrame)
        );
        assert_eq!(
            is_the_carry_on_offered(false, Some(17)),
            Err(WhyNotOffered::NoEndingWasEverLearned)
        );
        const _: () =
            assert!(THE_FIGURE_IS_READ_WHEN_THE_ACTION_IS_TAKEN_AND_NOTHING_DURABLE_IS_WRITTEN);
        const _: () = assert!(THE_PAGES_FIGURE_IS_A_STARTING_POINT_AND_C15S_LIMIT_IS_CARRIED_ON_IT);
    }

    /// **A7 and A7a**: the stat's exit status separates the folder present
    /// and unreadable from the folder gone, and the walk names which where
    /// the stat is itself refused.
    #[test]
    fn the_three_way_distinction_is_by_exit_status_and_the_walk_and_never_by_wording() {
        assert_eq!(
            separate(&WhatTheReadingGave::TheRequestWasNotAnswered),
            ThreeWay::TheMachineDidNotAnswer
        );
        assert_eq!(
            separate(&WhatTheReadingGave::TheStat {
                exit_status: 0,
                the_listing_was_refused: true,
                the_walk: None
            }),
            ThreeWay::ThisLoginMayNotReadIt
        );
        assert_eq!(
            separate(&WhatTheReadingGave::TheStat {
                exit_status: 0,
                the_listing_was_refused: false,
                the_walk: None
            }),
            ThreeWay::ItIsThere
        );
        assert_eq!(
            separate(&WhatTheReadingGave::TheStat {
                exit_status: 1,
                the_listing_was_refused: true,
                the_walk: Some(TheWalk::EveryParentAnsweredAndThePathItselfIsAbsent)
            }),
            ThreeWay::TheFolderIsGone
        );
        assert_eq!(
            separate(&WhatTheReadingGave::TheStat {
                exit_status: 1,
                the_listing_was_refused: true,
                the_walk: Some(TheWalk::AParentDidNotAnswerSoSomethingAboveItRefuses)
            }),
            ThreeWay::ThisLoginMayNotReadIt
        );
        assert_eq!(
            separate(&WhatTheReadingGave::TheStat {
                exit_status: 1,
                the_listing_was_refused: true,
                the_walk: None
            }),
            ThreeWay::GoneOrRefusedHigherUpAndTheWalkNamesWhich
        );
    }

    #[test]
    fn a_reading_that_did_not_complete_is_the_could_not_check_case_and_is_never_presented_as_checked(
    ) {
        assert!(a_reading_that_did_not_complete_is_not_a_reading(false));
        assert!(!a_reading_that_did_not_complete_is_not_a_reading(true));
        let case = the_could_not_check_case(Some(9));
        assert_eq!(case.the_kept_figure, Some(9));
        assert!(case.says_it_could_not_check_the_machine);
        assert!(!case.presented_as_checked);
    }

    #[test]
    fn a_gone_blend_on_a_machine_that_is_there_opens_the_browser_on_that_machine() {
        let gone = WhatTheReadingGave::TheStat {
            exit_status: 1,
            the_listing_was_refused: true,
            the_walk: Some(TheWalk::EveryParentAnsweredAndThePathItselfIsAbsent),
        };
        assert_eq!(
            what_a_gone_blend_leads_to(&gone),
            Some(WhereTheBrowserOpens::TheMachineTheJobRanOn)
        );
        assert_eq!(
            what_a_gone_blend_leads_to(&WhatTheReadingGave::TheRequestWasNotAnswered),
            None
        );
    }
}
