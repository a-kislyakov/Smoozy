//! **Check dependencies…, driven** — `Dependency Upload & Remap` §3.4's loop of
//! single-file readings over `Server protocol and transfers` T5's per-server
//! point, §4's look and search on the user's own computer, every decision the
//! user takes saved through the record's keeper, and §7.3's check standing
//! written at the sitting's end. Built at step 10c's dependency wave piece 5,
//! 2026-09-29.
//!
//! **What this file is: the caller `App/Dependency Upload & Remap/` never had.**
//! The loop (`TheLoop`), the reading (`drive_one_reading`), the judgement
//! (`judge`), the look at the recorded location, the search, the grouping, the
//! shift, the selection acts and the standing were all built there and nothing
//! in the running application called one of them. **Nothing here re-decides
//! any of them**: what a reference is, how it is found, how references group
//! into rows and what a standing stops are that module's. What is here is the
//! order they are called in, what is written when, and what the panel is told.
//!
//! # The order of one sitting
//!
//! 1. **The four values of the root's machine copy**, read first — `Dependency
//!    Upload & Remap` V10's *"as the reading took it"*. A machine that cannot
//!    answer them cannot be read either, and the check ends as `Render
//!    Settings` Q8's first ending with nothing asked of the supervisor.
//! 2. **One reading per `.blend`**, the root first —
//!    `Dependency Upload & Remap` G14 — each one request to the machine's supervisor, asked again
//!    until it ends (`Dependency Upload & Remap` G1, piece 2's route).
//! 3. **Each reading's references looked for on the user's computer** — the
//!    recorded location itself first (`Dependency Upload & Remap` F27), then
//!    the search over the user's folders in their order (`Dependency Upload &
//!    Remap` F1, F3), which the user can stop (`Dependency Upload & Remap` F6).
//!    **A reference the record already answers is not looked for again**:
//!    `Dependency Upload & Remap` G17, a fresh check starts from every located
//!    answer the record supplies and nothing is re-decided.
//! 4. **The reading's findings written in one save, before the next reading
//!    starts** — `Dependency Upload & Remap` G16 and K4, through piece 4's
//!    keeper. Then its rows reach the panel. **Until the root's reading
//!    lands, the panel says *Reading the file…* and draws no rows**
//!    (`WhatTheRunHolds::files_being_read`, the Creator's note 8 on the test
//!    copy).
//! 5. **Every linked `.blend` among them read in turn**, against the file that
//!    holds it (`Dependency Upload & Remap` G14), until the tree is walked out
//!    (`Dependency Upload & Remap` G13).
//! 6. **The check standing written** — `Dependency Upload & Remap` V10 — with
//!    the sitting's completion: completed, stopped by the user, or cut by the
//!    connection. **An incomplete sitting stops nothing** (`Dependency Upload &
//!    Remap` V10) and it is written as incomplete, never left unwritten.
//!
//! # What the 38's piece 7 added
//!
//! - **Stopping a reading already running on the machine.** The check's
//!   *Stop* sends `Server protocol and transfers` C21's stop naming the
//!   reading's operation, and the machine interrupts its recorded Blender
//!   (`Dependency Upload & Remap` G15, `Dependency Upload & Remap` O1). Where the stop reaches the root's
//!   own reading, the panel says the read was stopped (`Render Settings` Q8's
//!   *a read Smoozy ended itself*). **The desk's Mac publishes no per-process
//!   record**, so there the stop is taken and the reading runs to its end.
//! - **A linked `.blend` placed at its mirror is read there** (`Dependency
//!   Upload & Remap` G14), **and fixed at the sitting's end** — the deepest
//!   first — wherever no fixed copy stands or M16 owes one again. **A row
//!   inside it is confirmed only where its fixing record matches** (V11), read
//!   off the record rather than written owed as it was until this piece.
//! - **`Dependency Upload & Remap` Q5's second route**, the machine's own
//!   digest (Q3), is asked by `crate::the_found_files_sent` over each reading's
//!   rows where the record does not establish a file. **A file uploaded by
//!   hand through Files somewhere other than its mirror still reads *found on
//!   your computer***: no rule reaches it there (`Dependency Upload & Remap`
//!   Q9).
//!
//! **Every marker in this file that belongs to another document is written out
//! with that document at its own site, and this file keeps no header list.**

use std::collections::BTreeMap;
use std::sync::Mutex;
use std::time::SystemTime;

use smoozy_dependencies::acting::checking::{
    drive_one_reading_until_stopped, OneReading, TheLoop, TheReadingDidNotRun,
};
use smoozy_dependencies::bytes::{folder_of, MachineLocation, RecordedLocation, StoredLocation};
use smoozy_dependencies::check::{
    group_into_rows, judge, linked_item_statements, the_bake_output_path_makes_no_row, tidied,
    Attribution, Kind, Reading, ReadingDocument, ReferenceToGroup,
};
use smoozy_dependencies::contents::ContentIdentity;
use smoozy_dependencies::knowledge::sent_up_as_a_dependency_before;
use smoozy_dependencies::readings::{
    EnumerationCasing, FolderListing, MachineContents, Readings, StoredNameAnswer,
};
use smoozy_dependencies::record::{
    HoldingFile, HowResolved, LocatedAnswer, LocatedAnswerKey, MachineKey, ReadAs, Resolution,
    ResolvedTo, ShiftKey,
};
use smoozy_dependencies::rows::RowState;
use smoozy_dependencies::search::{
    resolution_from_an_accepted_proposal, resolution_from_the_search, search, the_answer_for,
    the_proposals_for, the_recorded_locations_own_answer, the_sets_members_in,
    the_stored_location_of, FolderVerdict, Outstanding, SearchedFolderList,
    TheRecordedLocationsOwnAnswer, WantedShape,
};
use smoozy_dependencies::shift::{
    apply_to_one, derive, look_for, the_full_old_location, ShiftOutcome,
};
use smoozy_dependencies::standing::{
    CheckStanding, Completion, FixingStatus, RowAtCompletion, WhatTheRecordSays,
};
use smoozy_job_files::layout::TheLayout;
use smoozy_render_settings::endings::Ending;
use smoozy_render_settings::file_half::BlenderIdentity;
use smoozy_render_settings::readings::FourValuesNow;
use smoozy_server_protocol::transfers::integrity::Digest;
use smoozy_server_protocol::transfers::SourceReading;

use crate::reading_the_file::TheMachineAFileIsReadOn;
use crate::record::dependency_records::{the_searched_folder_list, TheDependencyRecord};
use crate::surface::the_dependency_check::{
    keyed, ACheckedRow, ASitting, TheSearchInHand, TheShiftInHand, WhatTheRunHolds,
};

/// **The local record, as a sitting and a decision reach it** — one short act
/// at a time. **The application's lock is taken for each act alone** where the
/// sitting runs off the drawing thread, and **a decision reaches the connection
/// its command already holds**, so the application's lock is always taken
/// before the run's (the order every act on these pages takes them in).
pub trait TheCheckReachesTheRecord {
    /// Runs `act` against the record, and says whether the record was there.
    fn with_the_record(&self, act: &mut dyn FnMut(&rusqlite::Connection)) -> bool;
}

impl<T: crate::uploading::TheRecordAnUploadWrites + ?Sized> TheCheckReachesTheRecord for T {
    fn with_the_record(&self, act: &mut dyn FnMut(&rusqlite::Connection)) -> bool {
        crate::uploading::TheRecordAnUploadWrites::with_the_record(self, act)
    }
}

/// **The connection a command already holds**, lent to a decision.
pub struct OverOneConnection<'c>(pub &'c rusqlite::Connection);

impl TheCheckReachesTheRecord for OverOneConnection<'_> {
    fn with_the_record(&self, act: &mut dyn FnMut(&rusqlite::Connection)) -> bool {
        act(self.0);
        true
    }
}

// ════════════════════════════════════════════════════════════════════════════
// The user's own computer
// ════════════════════════════════════════════════════════════════════════════

/// **`Dependency Upload & Remap`'s readings, answered on the user's own
/// computer** by that module's platform adapter (`Dependency Upload & Remap`
/// F13), which this crate hosts.
///
/// **The two machine halves answer that they cannot**, and nothing in a check
/// asks them: the four values of a placed copy and the digest of one are
/// `Dependency Upload & Remap` Q5's routes, reached by the sending and by the
/// machine's digest, neither of which this file drives (this file's head).
pub struct TheUsersOwnComputer;

impl Readings for TheUsersOwnComputer {
    fn the_local_files_content_identity(&self, file: &StoredLocation) -> Option<ContentIdentity> {
        smoozy_dependencies::adapter::TheUsersOwnDisk.the_local_files_content_identity(file)
    }

    fn the_four_values_now(
        &self,
        _machine: &MachineKey,
        _at: &MachineLocation,
    ) -> smoozy_dependencies::readings::FourValuesNow {
        smoozy_dependencies::readings::FourValuesNow::TheChannelCannotAnswer
    }

    fn the_machine_copys_contents(
        &self,
        _machine: &MachineKey,
        _at: &MachineLocation,
    ) -> MachineContents {
        MachineContents::DidNotComplete
    }

    fn enumerate_folder(&self, folder: &[u8]) -> FolderListing {
        smoozy_dependencies::adapter::TheUsersOwnDisk.enumerate_folder(folder)
    }

    fn enumeration_casing_of(&self, folder: &[u8]) -> EnumerationCasing {
        smoozy_dependencies::adapter::TheUsersOwnDisk.enumeration_casing_of(folder)
    }

    fn the_stored_name_reading(&self, path: &[u8]) -> StoredNameAnswer {
        smoozy_dependencies::adapter::TheUsersOwnDisk.the_stored_name_reading(path)
    }

    fn the_sources_prefix_digest(&self, source: &StoredLocation, up_to: u64) -> Option<Digest> {
        smoozy_dependencies::adapter::TheUsersOwnDisk.the_sources_prefix_digest(source, up_to)
    }
}

// ════════════════════════════════════════════════════════════════════════════
// What a sitting is of
// ════════════════════════════════════════════════════════════════════════════

/// **One check, over one file on one server.**
#[derive(Debug, Clone)]
pub struct TheSittingIsOf {
    /// The server row's local identifier, which the panel is keyed by.
    pub server: String,
    /// The file as the panel keys it (`Render Settings`' key's text).
    pub file: String,
    /// **The root's machine location** — the file's exact bytes (`File Browser`
    /// L1a).
    pub root: MachineLocation,
    /// **`Dependency Upload & Remap` K3's holding file for the root**: the
    /// source path `File Browser` N6 hands over, or `Dependency Upload & Remap`
    /// K5's machine location where it finds none.
    pub holding: HoldingFile,
    /// **`Dependency Upload & Remap` G2: the Blender the job will render with.**
    pub with: BlenderIdentity,
    /// **`Dependency Upload & Remap` V10's machine**, under its installation.
    pub machine: MachineKey,
    /// The machine's layout, for `Dependency Upload & Remap` V9a's mirror.
    pub layout: Option<TheLayout>,
}

/// **The folder a holding file's relative locations are measured from** on the
/// user's computer — its own source folder (`Dependency Upload & Remap` F17
/// and `Dependency Upload & Remap` F27). **`Dependency Upload & Remap` K5's file has none**, and a relative
/// location inside it is not looked for there.
pub fn the_source_folder(holding: &HoldingFile) -> Option<Vec<u8>> {
    match holding {
        HoldingFile::OnTheUsersComputer(source) => Some(source.folder()),
        HoldingFile::OnTheMachineWithNoKnownLocalOrigin { .. } => None,
    }
}

/// **The folder the grouping measures a relative location from** —
/// `Dependency Upload & Remap` G11 — and **the `.blend`'s own folder that
/// DW0-2's exception is judged against** (`Dependency Upload & Remap` G6): the
/// source folder, or `Dependency Upload & Remap` K5's machine folder.
fn the_holder_folder(holding: &HoldingFile) -> Vec<u8> {
    match holding {
        HoldingFile::OnTheUsersComputer(source) => source.folder(),
        HoldingFile::OnTheMachineWithNoKnownLocalOrigin { location, .. } => {
            folder_of(location.as_bytes())
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// One reading's findings, composed
// ════════════════════════════════════════════════════════════════════════════

/// **What one landed reading came to**, before anything is written.
pub struct OneReadingLanded {
    /// Every reference's located answer, composed from what the record held
    /// (`Dependency Upload & Remap` G17) and what this reading found.
    pub answers: Vec<LocatedAnswer>,
    pub rows: Vec<ACheckedRow>,
    /// `Dependency Upload & Remap` F7's per-folder outcome, where a search ran.
    pub the_search: Option<TheSearchInHand>,
    /// **`Dependency Upload & Remap` F6: the user stopped the search**, so a
    /// *not found* it left is not the disk's answer.
    pub stopped: bool,
    /// `Dependency Upload & Remap` G6a.
    pub linked_items: Vec<smoozy_dependencies::check::LinkedItemStatement>,
}

/// **`Dependency Upload & Remap` K4's answer for one reference**, as the record
/// holds it or fresh.
fn the_answer_held_for(known: &[LocatedAnswer], key: &LocatedAnswerKey) -> LocatedAnswer {
    known
        .iter()
        .find(|answer| &answer.key == key)
        .cloned()
        .unwrap_or_else(|| LocatedAnswer {
            key: key.clone(),
            resolution: None,
            set_aside: false,
            rejected_guess: None,
            read_as: Vec::new(),
        })
}

/// **A proposal is offered unless it is the guess the user rejected** —
/// `Dependency Upload & Remap` K4: *"kept so it is not put forward again by
/// itself"*.
fn a_proposal_unless_rejected(answer: &LocatedAnswer, proposal: Vec<u8>) -> Option<StoredLocation> {
    match &answer.rejected_guess {
        Some(rejected) if rejected.as_bytes() == proposal.as_slice() => None,
        _ => Some(StoredLocation::as_the_system_returned_it(&proposal)),
    }
}

/// **The row's state, from its references' answers** — `Dependency Upload &
/// Remap` Rows 403 and 404 as far as this piece reaches.
///
/// - **Set aside** where every reference behind it carries `Dependency Upload &
///   Remap` K4's mark.
/// - **Found on your computer** where every reference resolved — a row groups
///   references by the one file they resolved to
///   (`Dependency Upload & Remap` G11).
/// - **Not found** otherwise: `Dependency Upload & Remap` F7's absence of a
///   located answer.
///
/// **Never *already on the server* here**: `Dependency Upload & Remap` Q5
/// needs the machine, and it is read afterwards over the rows a placement
/// record names (`crate::the_found_files_sent::what_is_already_on_the_server`).
pub fn the_state_of(answers: &[LocatedAnswer]) -> RowState {
    if !answers.is_empty() && answers.iter().all(|answer| answer.set_aside) {
        RowState::SetAside
    } else if !answers.is_empty() && answers.iter().all(|answer| answer.resolution.is_some()) {
        RowState::FoundOnYourComputer
    } else {
        RowState::NotFound
    }
}

/// **A located answer's bytes, as a path this system can be asked about.** On
/// macOS and Linux a path is bytes and nothing is rewritten.
#[cfg(unix)]
pub(crate) fn a_path(bytes: &[u8]) -> Option<std::path::PathBuf> {
    use std::os::unix::ffi::OsStrExt;
    Some(std::path::PathBuf::from(std::ffi::OsStr::from_bytes(bytes)))
}

/// **On Windows a path is UTF-16 and these answers are carried as bytes**, so
/// bytes that are not text name no file this system can be asked about, and
/// the row shows no size — the rule `Dependency Upload & Remap`'s own adapter
/// already keeps for the same bytes.
#[cfg(not(unix))]
pub(crate) fn a_path(bytes: &[u8]) -> Option<std::path::PathBuf> {
    Some(std::path::PathBuf::from(std::str::from_utf8(bytes).ok()?))
}

/// **The size a row shows** — a file's own length, or a set's members' sum.
/// **A figure to read and nothing more**: nothing decides anything from it.
fn the_size_of(resolution: Option<&Resolution>, members: Option<&[Vec<u8>]>) -> Option<u64> {
    let resolution = resolution?;
    let at = resolution.to.location().as_bytes();
    match (&resolution.to, members) {
        (ResolvedTo::AFolderForASetRow(_), Some(members)) => Some(
            members
                .iter()
                .filter_map(|member| {
                    std::fs::metadata(a_path(&smoozy_dependencies::bytes::join(at, member))?)
                        .ok()
                        .map(|found| found.len())
                })
                .sum(),
        ),
        (ResolvedTo::AWholeLocation(_), _) => std::fs::metadata(a_path(at)?)
            .ok()
            .filter(|found| found.is_file())
            .map(|found| found.len()),
        _ => None,
    }
}

/// **One reading's references, answered and grouped** —
/// `Dependency Upload & Remap` F27, then `Dependency Upload & Remap` F3 over
/// `Dependency Upload & Remap` F1's list, then `Dependency Upload & Remap` G10
/// and `Dependency Upload & Remap` G11.
///
/// **What the record holds is the starting point**
/// (`Dependency Upload & Remap` G17): a reference already resolved, or set aside, is not looked for
/// again. **Only a reference with no answer is looked for**, the recorded
/// location first and the search after it.
#[allow(clippy::too_many_arguments)]
pub fn the_rows_of_one_reading(
    document: &ReadingDocument,
    holding: &HoldingFile,
    known: &[LocatedAnswer],
    list: &SearchedFolderList,
    readings: &dyn Readings,
    stop: &mut dyn FnMut() -> bool,
    now: SystemTime,
) -> OneReadingLanded {
    let source_folder = the_source_folder(holding);
    let mut answers: Vec<LocatedAnswer> = Vec::new();
    let mut wanted: Vec<Outstanding> = Vec::new();
    let mut proposals: BTreeMap<Vec<u8>, StoredLocation> = BTreeMap::new();
    let mut outstanding: Vec<usize> = Vec::new();

    for reference in &document.references {
        let key = LocatedAnswerKey {
            holding_file: holding.clone(),
            reference: reference.recorded.clone(),
        };
        let mut answer = the_answer_held_for(known, &key);
        // **`Dependency Upload & Remap` K4's last limb: what this reading
        // carried the reference as**, written afresh by every reading, so the
        // remap's pairs are composed from the record at submission in any run
        // (step 10c's dependency wave piece 7b).
        answer.read_as = ReadAs::in_the_reading(&document.references, &reference.recorded);
        // `Dependency Upload & Remap` F28 with DW0-3: the kind and the stored
        // source decide the shape.
        let one = Outstanding::of(
            reference.recorded.clone(),
            reference.kind,
            reference.stored_source(),
        );
        if answer.resolution.is_none() && !answer.set_aside {
            // **`Dependency Upload & Remap` F27: the recorded location itself,
            // before any search.**
            match the_recorded_locations_own_answer(
                &reference.recorded,
                one.shape,
                source_folder.as_deref(),
                readings,
            ) {
                TheRecordedLocationsOwnAnswer::Resolved(resolution) => {
                    answer.resolution = Some(resolution)
                }
                TheRecordedLocationsOwnAnswer::AProposal(path) => {
                    if let Some(proposal) = a_proposal_unless_rejected(&answer, path) {
                        proposals.insert(reference.recorded.as_bytes().to_vec(), proposal);
                    }
                    outstanding.push(answers.len());
                }
                TheRecordedLocationsOwnAnswer::NothingIsThere
                | TheRecordedLocationsOwnAnswer::TheSystemCannotSay
                | TheRecordedLocationsOwnAnswer::Refused(_) => outstanding.push(answers.len()),
            }
        }
        answers.push(answer);
        wanted.push(one);
    }

    // **`Dependency Upload & Remap` F3 over F1's list**, for what the recorded
    // location did not answer — the user's folders, in their order.
    let mut the_search = None;
    let mut stopped = false;
    let to_search: Vec<Outstanding> = outstanding.iter().map(|at| wanted[*at].clone()).collect();
    if !list.folders.is_empty() && !to_search.is_empty() {
        let outcome = search(list, &to_search, readings, stop, now);
        stopped = !outcome.report.finished;
        let mut the_matches_came_from = None;
        for at in &outstanding {
            let candidates = outcome
                .candidates
                .get(&wanted[*at].reference)
                .cloned()
                .unwrap_or_default();
            if let Some(found) = the_answer_for(list, &candidates) {
                if let Ok(resolution) = resolution_from_the_search(&found, readings) {
                    the_matches_came_from.get_or_insert(found.folder.clone());
                    answers[*at].resolution = Some(resolution);
                    proposals.remove(wanted[*at].reference.as_bytes());
                    continue;
                }
            }
            let rejected = answers[*at].rejected_guess.clone();
            if let Some(proposal) = the_proposals_for(&candidates, rejected.as_ref())
                .into_iter()
                .next()
            {
                proposals
                    .entry(wanted[*at].reference.as_bytes().to_vec())
                    .or_insert_with(|| StoredLocation::as_the_system_returned_it(&proposal));
            }
        }
        the_search = Some(TheSearchInHand {
            folders: list
                .folders
                .iter()
                .map(|folder| {
                    let verdict = outcome
                        .report
                        .per_folder
                        .iter()
                        .find(|one| &one.folder == folder)
                        .map(|one| one.verdict)
                        .or(stopped.then_some(FolderVerdict::StoppedPartWay));
                    (folder.clone(), verdict)
                })
                .collect(),
            the_matches_came_from,
            running: false,
            // `Dependency Upload & Remap` F1a: the moment of this search, as
            // its own report took it.
            searched_at: outcome.report.per_folder.first().map(|one| one.at.taken_at()),
        });
    }

    // **`Dependency Upload & Remap` G10 and `Dependency Upload & Remap` G11: grouped into rows**, with the
    // `.blend`'s own folder as the holder folder so DW0-2 can fire
    // (`Dependency Upload & Remap` G6).
    let holder_folder = the_holder_folder(holding);
    let to_group: Vec<ReferenceToGroup> = document
        .references
        .iter()
        .zip(&answers)
        .map(|(reference, answer)| ReferenceToGroup {
            kind: reference.kind,
            recorded: reference.recorded.clone(),
            holder_folder: holder_folder.clone(),
            resolved: answer
                .resolution
                .as_ref()
                .map(|resolution| resolution.to.location().clone()),
        })
        .collect();
    let mut rows = Vec::new();
    for row in group_into_rows(&to_group) {
        let at: Vec<usize> = row
            .references
            .iter()
            .filter_map(|recorded| {
                document
                    .references
                    .iter()
                    .position(|one| &one.recorded == recorded)
            })
            .collect();
        let Some(first) = at.first().copied() else {
            continue;
        };
        let its_answers: Vec<LocatedAnswer> = at.iter().map(|at| answers[*at].clone()).collect();
        let its_wanted: Vec<Outstanding> = at.iter().map(|at| wanted[*at].clone()).collect();
        // **`Dependency Upload & Remap` G20a: every scene the walk reached**,
        // and nothing where any reference's walk did not complete.
        let mut scenes = Vec::new();
        let mut established = true;
        for at in &at {
            match &document.references[*at].scenes {
                Attribution::Established(names) => {
                    for name in names {
                        if !scenes.contains(name) {
                            scenes.push(name.clone());
                        }
                    }
                }
                Attribution::NotEstablished => established = false,
            }
        }
        let state = the_state_of(&its_answers);
        let resolution = its_answers[0].resolution.clone();
        let members = match (&resolution, its_wanted[0].the_sets_pattern()) {
            (Some(resolution), Some(pattern)) => {
                the_sets_members_in(resolution.to.location(), pattern, readings)
            }
            _ => None,
        };
        let proposal = row
            .references
            .iter()
            .find_map(|recorded| proposals.get(recorded.as_bytes()).cloned());
        rows.push(ACheckedRow {
            kind: document.references[first].kind,
            scenes: if established {
                Attribution::Established(scenes)
            } else {
                Attribution::NotEstablished
            },
            state_when_the_check_ran: state,
            state,
            holding: holding.clone(),
            answers: its_answers,
            wanted: its_wanted,
            proposal,
            members: members.as_ref().map(Vec::len),
            size: the_size_of(resolution.as_ref(), members.as_deref()),
            why_it_did_not_go_up: None,
            its_failure_was_cleared: false,
            row,
        });
    }

    // **DW0-2: no located answer is written for a bake output path that makes
    // no row** (`Dependency Upload & Remap` G6) — an answer written for it
    // would name the `.blend`'s own folder as a found dependency in the one
    // place every later act reads, though no row ever showed it.
    let answers = answers
        .into_iter()
        .zip(&to_group)
        .filter(|(_, grouped)| !the_bake_output_path_makes_no_row(grouped))
        .map(|(answer, _)| answer)
        .collect();
    OneReadingLanded {
        answers,
        rows,
        the_search,
        stopped,
        linked_items: linked_item_statements(document),
    }
}

/// **`Dependency Upload & Remap` G14: a linked `.blend` is read against the
/// file that holds it** — where the render would find it on the machine: a
/// relative location measured from the holding file's own machine folder, a
/// full one as it stands. **A location of neither form is not read**, and the
/// readable chain stops there.
pub fn where_the_render_finds_it(
    holding_file_on_the_machine: &MachineLocation,
    recorded: &RecordedLocation,
) -> Option<MachineLocation> {
    let bytes = recorded.as_bytes();
    if bytes.starts_with(b"//") || bytes.starts_with(b"/") {
        Some(MachineLocation::composed(tidied(
            &folder_of(holding_file_on_the_machine.as_bytes()),
            recorded,
        )))
    } else {
        None
    }
}

/// **A library row's own holding file**, for the reading below it —
/// `Dependency Upload & Remap` K3: *"for a linked file below it, the source
/// location its own resolution recorded"*, or `Dependency Upload & Remap` K5's
/// machine location where it resolved to nothing on the user's computer.
fn the_librarys_holding(
    row: &ACheckedRow,
    machine: &MachineKey,
    on_the_machine: &MachineLocation,
) -> HoldingFile {
    match row
        .the_answer()
        .and_then(|answer| answer.resolution.as_ref())
    {
        Some(resolution) => HoldingFile::OnTheUsersComputer(resolution.to.location().clone()),
        None => HoldingFile::OnTheMachineWithNoKnownLocalOrigin {
            machine: machine.clone(),
            location: on_the_machine.clone(),
        },
    }
}

// ════════════════════════════════════════════════════════════════════════════
// The sitting, driven
// ════════════════════════════════════════════════════════════════════════════

/// **What a sitting came to.**
#[derive(Debug, Clone, PartialEq)]
pub enum TheCheckCameTo {
    /// **The sitting stands**, with its completion — and its standing written.
    TheSittingStands(Completion),
    /// **The root gave nothing**: one of `Render Settings` Q8's endings, held
    /// beside the panel (`Dependency Upload & Remap` G3).
    ItGaveNothing(Ending),
    /// **A check of this file is already running**, and this press joined it.
    ItJoinedTheOneRunning,
    /// **The record refused a write**, so what was found is on screen and not
    /// kept (`Dependency Upload & Remap` G16 broken, and said).
    TheRecordRefusedAWrite,
    /// **The record would not read** — the answers already decided, or the
    /// search folders — so nothing was looked for (L8a).
    TheRecordWouldNotRead,
}

/// **Whether the user asked this sitting to stop**, read from the run's memory.
fn a_stop_was_asked(held: &Mutex<WhatTheRunHolds>, key: &str) -> bool {
    held.lock()
        .map(|held| held.checks_running.get(key).copied().unwrap_or(false))
        .unwrap_or(true)
}

/// **One save through the keeper**, answering whether it was written.
fn written(
    record: &dyn TheCheckReachesTheRecord,
    mut act: impl FnMut(&TheDependencyRecord<'_>) -> bool,
) -> bool {
    let mut done = false;
    let there = record.with_the_record(&mut |connection| {
        done = act(&TheDependencyRecord::over(connection));
    });
    there && done
}

/// **The whole sitting** — see this file's head for the order.
///
/// `told` is called whenever the panel has something new to draw; the
/// application posts the panel's own event from it (`Desktop app and delivery`
/// P7), and a test counts the calls.
#[allow(clippy::too_many_arguments)]
pub async fn the_check_runs<M>(
    machine: &mut M,
    record: &dyn TheCheckReachesTheRecord,
    readings: &dyn Readings,
    held: &Mutex<WhatTheRunHolds>,
    sitting: &TheSittingIsOf,
    told: &(dyn Fn() + Send + Sync),
) -> TheCheckCameTo
where
    M: TheMachineAFileIsReadOn + Send,
{
    let key = keyed(&sitting.server, &sitting.file);

    // **One sitting per file at a time**: a second press joins the first,
    // the way `Render Settings` Q5 joins a second read.
    match held.lock() {
        Ok(mut held) => {
            if held.checks_running.contains_key(&key) {
                return TheCheckCameTo::ItJoinedTheOneRunning;
            }
            held.checks_running.insert(key.clone(), false);
            // **The file itself is being read until its reading lands** —
            // `Design/Dependency Upload & Remap § While it is still reading
            // below a linked file`, its closing paragraph. The Creator's note
            // 8 on the test copy.
            held.files_being_read.insert(key.clone());
        }
        Err(_) => return TheCheckCameTo::TheRecordWouldNotRead,
    }
    let came_to = the_sitting_is_run(machine, record, readings, held, sitting, &key, told).await;
    // **Where the root gave nothing, the ending is the caller's to land**, in
    // `crate::surface::the_dependency_check::the_check_gave_nothing`, which is
    // the one place an ending is handed to the diary.
    // **And the mark comes off on every way out**, so no panel goes on saying
    // the file is being read once nothing reads it.
    if let Ok(mut held) = held.lock() {
        held.checks_running.remove(&key);
        held.files_being_read.remove(&key);
    }
    came_to
}

#[allow(clippy::too_many_arguments)]
async fn the_sitting_is_run<M>(
    machine: &mut M,
    record: &dyn TheCheckReachesTheRecord,
    readings: &dyn Readings,
    held: &Mutex<WhatTheRunHolds>,
    sitting: &TheSittingIsOf,
    key: &str,
    told: &(dyn Fn() + Send + Sync),
) -> TheCheckCameTo
where
    M: TheMachineAFileIsReadOn + Send,
{
    // **1. The root's four values, as the reading takes it** (`Dependency
    // Upload & Remap` V10).
    let root_four_values: SourceReading = match machine.the_four_values_of(&sitting.file).await {
        FourValuesNow::Answered(values) => values,
        FourValuesNow::TheChannelCannotAnswer => {
            return TheCheckCameTo::ItGaveNothing(Ending::TheMachineCouldNotBeReached)
        }
    };

    // **`Dependency Upload & Remap` V9a**, from the record of the act, and
    // **F1's list in the user's order**.
    let mut sent_up_before = false;
    let mut list = SearchedFolderList::default();
    let mut the_record_read = true;
    record.with_the_record(&mut |connection| {
        let keeper = TheDependencyRecord::over(connection);
        match (&sitting.layout, keeper.what_it_holds_for(&sitting.machine)) {
            (Some(layout), Ok(snapshot)) => {
                sent_up_before = sent_up_as_a_dependency_before(
                    &sitting.holding,
                    layout,
                    &sitting.machine,
                    &sitting.machine.installation,
                    &snapshot,
                )
                .is_some();
            }
            (_, Err(_)) => the_record_read = false,
            (None, Ok(_)) => {}
        }
        match the_searched_folder_list::read(connection) {
            Ok(read) => list = read,
            Err(_) => the_record_read = false,
        }
    });
    if !the_record_read {
        return TheCheckCameTo::TheRecordWouldNotRead;
    }

    // **A fresh sitting on the panel**: what an earlier one found is re-read,
    // never carried (`Dependency Upload & Remap` G17), and an earlier ending
    // goes with it.
    if let Ok(mut held) = held.lock() {
        held.endings.remove(key);
        held.selections.remove(key);
        held.sittings.insert(
            key.to_string(),
            ASitting {
                read_with: sitting.with.clone(),
                linked_files_read: 0,
                still_reading_below: None,
                completion: None,
                linked_items: Vec::new(),
                sent_up_as_a_dependency_before: sent_up_before,
                the_search: None,
                the_shift: None,
                the_once: None,
                rows: Vec::new(),
                the_chosen_search_folder: None,
                // **Settings' list as this sitting read it**, which the
                // folders line is compared with — the Creator's note 5 on
                // 0.1.8.
                the_list_it_searched: list.folders.clone(),
                the_folder_added: None,
                of: Some(sitting.clone()),
            },
        );
    }
    told();

    // **2. to 5. The loop** (`Dependency Upload & Remap` G13 to `Dependency Upload & Remap` G17).
    let mut the_loop = TheLoop::over(sitting.root.clone());
    let mut holdings: BTreeMap<Vec<u8>, (HoldingFile, Option<RecordedLocation>)> = BTreeMap::new();
    holdings.insert(
        sitting.root.as_bytes().to_vec(),
        (sitting.holding.clone(), None),
    );
    let mut completion = Completion::Completed;
    while let Some(file) = the_loop.next_file() {
        if a_stop_was_asked(held, key) {
            completion = Completion::StoppedByTheUser;
            break;
        }
        let is_the_root = file == sitting.root;
        let (holding, recorded_as) = holdings
            .get(file.as_bytes())
            .cloned()
            .unwrap_or((sitting.holding.clone(), None));
        if let (Ok(mut held), Some(recorded_as)) = (held.lock(), recorded_as.as_ref()) {
            if let Some(standing) = held.sittings.get_mut(key) {
                standing.still_reading_below = Some(recorded_as.clone());
            }
        }
        told();

        let Some(reading) = OneReading::of(&file, &sitting.with) else {
            completion = Completion::TheConnectionDropped;
            break;
        };
        // **`Dependency Upload & Remap` G15: a stop asked while the reading is
        // on the machine is sent to it**, naming its operation, and the
        // reading ends here at once (the 38's piece 7).
        let left = drive_one_reading_until_stopped(machine, &reading, &mut || {
            a_stop_was_asked(held, key)
        })
        .await;
        // **A stop asked while the reading ran throws its answer away.**
        // **Where the stop reached the root's reading on the machine**, nothing
        // was read, so the panel says the read was stopped (`Render Settings`
        // Q8's *a read Smoozy ended itself*, drawn `chk-unstop`); otherwise the
        // sitting stands stopped with what it found.
        if is_the_root && matches!(left, Err(TheReadingDidNotRun::TheUserStoppedIt)) {
            return TheCheckCameTo::ItGaveNothing(Ending::SmoozyStoppedIt);
        }
        if a_stop_was_asked(held, key) || matches!(left, Err(TheReadingDidNotRun::TheUserStoppedIt)) {
            completion = Completion::StoppedByTheUser;
            break;
        }
        let document = match left {
            Ok(left) => match judge(left, &sitting.with) {
                Reading::Read(document) => document,
                // **`Dependency Upload & Remap` G3** at the root; below it,
                // **the readable chain stops at that file**
                // (`Dependency Upload & Remap` G14) and its row keeps saying it
                // may add more.
                Reading::CouldNotBeRead { .. } if is_the_root => {
                    return TheCheckCameTo::ItGaveNothing(
                        Ending::TheFileCouldNotBeOpenedByThisVersion,
                    )
                }
                Reading::CouldNotBeRead { .. } => continue,
            },
            Err(
                TheReadingDidNotRun::TheRequestDidNotReachTheMachine
                | TheReadingDidNotRun::NoAnswerCameBack
                | TheReadingDidNotRun::ItPassedTheTimeLimit
                | TheReadingDidNotRun::TheStreamsCouldNotBeReadBack(_),
            ) => {
                if is_the_root {
                    return TheCheckCameTo::ItGaveNothing(Ending::TheMachineCouldNotBeReached);
                }
                // **`Dependency Upload & Remap` G16: the sitting stands**, what
                // was found stays, and it is said to be incomplete.
                completion = Completion::TheConnectionDropped;
                break;
            }
            // **The machine took no reading and ran no Blender** — a program
            // placed before the reading existed, or a reading not admitted.
            // `Render Settings`' frame puts that on the arm that claims nothing
            // about the file.
            Err(
                TheReadingDidNotRun::TheMachinesProgramDoesNotTakeIt
                | TheReadingDidNotRun::TheMachineRefusedIt(_),
            ) => {
                if is_the_root {
                    return TheCheckCameTo::ItGaveNothing(
                        Ending::NotAReadingAndNoneOfTheThreeEndingsFits,
                    );
                }
                continue;
            }
            Err(TheReadingDidNotRun::SmoozyEndedIt | TheReadingDidNotRun::TheUserStoppedIt) => {
                if is_the_root {
                    return TheCheckCameTo::ItGaveNothing(Ending::SmoozyStoppedIt);
                }
                continue;
            }
        };

        // **3. Looked for on the user's computer**, from what the record holds.
        let mut known = Vec::new();
        let mut the_known_read = false;
        record.with_the_record(&mut |connection| {
            if let Ok(read) = TheDependencyRecord::over(connection).the_answers_of(&holding) {
                known = read;
                the_known_read = true;
            }
        });
        if !the_known_read {
            return TheCheckCameTo::TheRecordWouldNotRead;
        }
        let mut stop = || a_stop_was_asked(held, key);
        let mut landed = the_rows_of_one_reading(
            &document,
            &holding,
            &known,
            &list,
            readings,
            &mut stop,
            SystemTime::now(),
        );

        // **`Dependency Upload & Remap` Q5's first route, over the rows a
        // placement record names** — so a file this module already sent reads
        // *already on the server*, and Q7's *uploaded before, not confirmed*
        // where its local original is gone. Step 10c's dependency wave piece 6.
        crate::the_found_files_sent::what_is_already_on_the_server(
            machine,
            record,
            sitting,
            &mut landed.rows,
        )
        .await;

        // **4. Written in one save, before the next reading** (`Dependency
        // Upload & Remap` G16, K4).
        if !written(record, |keeper| {
            keeper.the_answers_are_written(&landed.answers).is_ok()
        }) {
            return TheCheckCameTo::TheRecordRefusedAWrite;
        }

        // **5. The linked `.blend`s below it**, each against this file — or,
        // **where this module placed it, at its mirror** (`Dependency Upload &
        // Remap` G14: *"When an upload puts a missing linked file on the
        // machine, that file is read"*), which is the copy its fixed copy is
        // derived from (`Dependency Upload & Remap` A3). The 38's piece 7.
        for row in landed.rows.iter().filter(|row| row.kind == Kind::Library) {
            let at_its_mirror = (row.state == RowState::AlreadyOnTheMachine)
                .then(|| crate::the_found_files_sent::the_file_and_its_mirror(row, sitting))
                .flatten()
                .map(|(_, mirror)| mirror);
            for recorded in &row.row.references {
                let below = at_its_mirror
                    .clone()
                    .or_else(|| where_the_render_finds_it(&file, recorded));
                if let Some(below) = below {
                    holdings
                        .entry(below.as_bytes().to_vec())
                        .or_insert_with(|| {
                            (
                                the_librarys_holding(row, &sitting.machine, &below),
                                Some(recorded.clone()),
                            )
                        });
                    the_loop.and_below_it([below]);
                }
            }
        }

        if let Ok(mut held) = held.lock() {
            if let Some(standing) = held.sittings.get_mut(key) {
                // **The rows that arrive go into their groups without moving
                // what is above them** — `Design/Dependency Upload & Remap §
                // While it is still reading below a linked file`.
                standing.rows.extend(landed.rows);
                standing.linked_items.extend(landed.linked_items);
                if landed.the_search.is_some() {
                    standing.the_search = landed.the_search;
                }
                if !is_the_root {
                    standing.linked_files_read += 1;
                }
                standing.still_reading_below = None;
            }
            // **The file itself is read, so its rows are drawn from here** —
            // in the same hold of the run as the rows that land.
            if is_the_root {
                held.files_being_read.remove(key);
            }
        }
        told();
        if landed.stopped {
            completion = Completion::StoppedByTheUser;
            break;
        }
    }

    // **M13 to M16: every linked `.blend` on the server at its mirror gets
    // its fixed copy, produced where none stands or one is owed again** — the
    // deepest first, so a fixed copy's own library pairs find the fixed
    // copies below it (M13). A fixing that fails leaves its row *could not be
    // fixed* with the reason (`Dependency Upload & Remap` M16a). The 38's piece 7.
    if completion == Completion::Completed {
        let libraries: Vec<(usize, ACheckedRow)> = match held.lock() {
            Ok(held) => held
                .sittings
                .get(key)
                .map(|standing| {
                    standing
                        .rows
                        .iter()
                        .enumerate()
                        .filter(|(_, row)| {
                            row.kind == Kind::Library && row.state == RowState::AlreadyOnTheMachine
                        })
                        .map(|(at, row)| (at, row.clone()))
                        .collect()
                })
                .unwrap_or_default(),
            Err(_) => Vec::new(),
        };
        for (at, row) in libraries.into_iter().rev() {
            if let Err(why) =
                crate::the_found_files_sent::a_library_is_fixed(machine, record, sitting, &row, false).await
            {
                if let Ok(mut held) = held.lock() {
                    if let Some(one) = held.sittings.get_mut(key).and_then(|standing| standing.rows.get_mut(at)) {
                        one.state = RowState::CouldNotBeFixed;
                        one.state_when_the_check_ran = RowState::CouldNotBeFixed;
                        one.why_it_did_not_go_up = Some(crate::the_found_files_sent::why_it_did_not_go_up(why));
                    }
                }
            }
            told();
        }
    }

    // **V11: a row inside a linked `.blend` is confirmed only where that
    // file's fixing record matches the current located answers and no M16
    // trigger has left its re-production owed** — read off the record, now
    // that the fixings are written (the 38's piece 7; until then every such
    // row was written owed, since nothing fixed anything).
    // **Read with no lock of the run's held**: the record is the
    // application's, and its lock is always taken before the run's.
    let holdings_below: Vec<HoldingFile> = held
        .lock()
        .ok()
        .and_then(|held| {
            held.sittings.get(key).map(|standing| {
                let mut below: Vec<HoldingFile> = Vec::new();
                for row in &standing.rows {
                    if row.holding != sitting.holding && !below.contains(&row.holding) {
                        below.push(row.holding.clone());
                    }
                }
                below
            })
        })
        .unwrap_or_default();
    let statuses: Vec<(HoldingFile, FixingStatus)> = holdings_below
        .into_iter()
        .map(|holding| {
            let status = crate::the_found_files_sent::the_fixing_status_of(record, sitting, &holding);
            (holding, status)
        })
        .collect();
    let the_fixing_of = |holding: &HoldingFile| -> FixingStatus {
        statuses
            .iter()
            .find(|(one, _)| one == holding)
            .map(|(_, status)| *status)
            .unwrap_or(FixingStatus {
                the_fixing_matches_the_current_located_answers: false,
                a_re_production_is_owed: true,
            })
    };

    // **6. The check standing** (`Dependency Upload & Remap` V10 to V12).
    let rows_at_completion: Vec<RowAtCompletion> = match held.lock() {
        Ok(mut held) => match held.sittings.get_mut(key) {
            Some(standing) => {
                standing.completion = Some(completion);
                standing.still_reading_below = None;
                standing
                    .rows
                    .iter()
                    .map(|row| RowAtCompletion {
                        state: row.state,
                        inside_a_linked_blend: (row.holding != sitting.holding)
                            .then(|| the_fixing_of(&row.holding)),
                    })
                    .collect()
            }
            None => Vec::new(),
        },
        Err(_) => Vec::new(),
    };
    let standing = CheckStanding::at_the_sittings_end(
        sitting.root.clone(),
        sitting.machine.clone(),
        sitting.with.clone(),
        SystemTime::now(),
        completion,
        &rows_at_completion,
        root_four_values,
    );
    if !written(record, |keeper| {
        keeper.a_standing_is_written(&standing).is_ok()
    }) {
        return TheCheckCameTo::TheRecordRefusedAWrite;
    }
    // **The Render Settings band reads the standing from the run's memory**,
    // not from the record yet (`src/surface/render_settings.rs`,
    // `the_dependency_band`, whose own comment names the line that changes
    // when the record's reader lands). So the standing just written is handed
    // to that memory too, under the same key, until that line changes.
    if let Ok(mut page) = crate::surface::render_settings::the_run_holds().lock() {
        page.the_check_record.insert(
            key.to_string(),
            WhatTheRecordSays::AStanding(Box::new(standing)),
        );
    }
    TheCheckCameTo::TheSittingStands(completion)
}

// ════════════════════════════════════════════════════════════════════════════
// Every decision about the user's own computer, saved
// ════════════════════════════════════════════════════════════════════════════

/// **A decision the user takes on the panel** — `Dependency Upload & Remap`
/// K4's list, and `Dependency Upload & Remap` §4.4 and §4.5's acts. Each is
/// about the user's own computer and needs no machine (`Design/Dependency
/// Upload & Remap § When the connection drops mid-check`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ADecision {
    SetAside,
    Restore,
    /// **Reject match** (once *Undo Smoozy's guess*): the found location becomes `Dependency Upload &
    /// Remap` K4's rejected guess and the row is not found.
    UndoTheGuess,
    /// `Dependency Upload & Remap` F10 and `Dependency Upload & Remap` F24:
    /// the row's own proposal taken.
    Accept,
    /// **Use it**: the rejected guess reinstated and taken.
    UseIt,
    /// `Dependency Upload & Remap` F16: a file (or a set row's folder) pointed
    /// at by hand, which may teach a shift.
    Locate(Vec<u8>),
    /// `Dependency Upload & Remap` F26: a folder, over a selection, each row
    /// resolved inside it by F18's own rule, and no shift.
    ChooseAFolder(Vec<u8>),
    /// `Dependency Upload & Remap` F20.
    UndoTheShift,
    /// `Design/Dependency Upload & Remap § The search folders on this panel`:
    /// the chosen folder added to Settings' list, and the search run again
    /// over the rows still not found.
    AddTheChosenSearchFolder,
}

/// **What a decision came to.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatADecisionCameTo {
    Taken,
    /// **Nothing it reaches is on the panel** — no sitting, or no row it fits.
    NothingToTake,
    /// **The record refused the write**, and the row is left as it was.
    NotSaved,
}

/// **One row's answers written in one save** (`Dependency Upload & Remap` F22:
/// one write per row as the act reaches it), and the row brought to them.
fn the_rows_answers_are_written(
    row: &mut ACheckedRow,
    answers: Vec<LocatedAnswer>,
    proposal: Option<StoredLocation>,
    record: &dyn TheCheckReachesTheRecord,
) -> bool {
    if !written(record, |keeper| {
        keeper.the_answers_are_written(&answers).is_ok()
    }) {
        return false;
    }
    row.state = the_state_of(&answers);
    row.answers = answers;
    row.proposal = proposal;
    true
}

/// **Every decision, taken over the rows it reaches** — the row pressed, or
/// the selection where the act came from the bar
/// (`Dependency Upload & Remap` F22).
pub fn a_decision_is_taken(
    held: &mut WhatTheRunHolds,
    key: &str,
    decision: ADecision,
    row: Option<usize>,
    record: &dyn TheCheckReachesTheRecord,
    readings: &dyn Readings,
) -> WhatADecisionCameTo {
    let reached: Vec<usize> = match row {
        Some(at) => vec![at],
        None => held
            .selections
            .get(key)
            .map(|chosen| chosen.iter().copied().collect())
            .unwrap_or_default(),
    };
    let Some(sitting) = held.sittings.get_mut(key) else {
        return WhatADecisionCameTo::NothingToTake;
    };
    let mut took = false;
    match decision {
        ADecision::SetAside | ADecision::Restore => {
            let mark = decision == ADecision::SetAside;
            for at in reached {
                let Some(one) = sitting.rows.get_mut(at) else {
                    continue;
                };
                let answers = one
                    .answers
                    .iter()
                    .cloned()
                    .map(|mut answer| {
                        answer.set_aside = mark;
                        answer
                    })
                    .collect();
                let proposal = one.proposal.clone();
                if !the_rows_answers_are_written(one, answers, proposal, record) {
                    return WhatADecisionCameTo::NotSaved;
                }
                took = true;
            }
        }
        ADecision::UndoTheGuess => {
            for at in reached {
                let Some(one) = sitting.rows.get_mut(at) else {
                    continue;
                };
                // **Only a guess is undone** — a location the search or a
                // shift supplied (`Dependency Upload & Remap` K4).
                let a_guess = one.answers.iter().any(|answer| {
                    matches!(
                        answer.resolution.as_ref().map(|resolution| &resolution.how),
                        Some(HowResolved::TheSearch { .. }) | Some(HowResolved::AShift(_))
                    )
                });
                if !a_guess {
                    continue;
                }
                let answers = one
                    .answers
                    .iter()
                    .cloned()
                    .map(|mut answer| {
                        if let Some(resolution) = answer.resolution.take() {
                            answer.rejected_guess = Some(resolution.to.location().clone());
                        }
                        answer
                    })
                    .collect();
                if !the_rows_answers_are_written(one, answers, None, record) {
                    return WhatADecisionCameTo::NotSaved;
                }
                took = true;
            }
        }
        ADecision::Accept | ADecision::UseIt => {
            for at in reached {
                let Some(one) = sitting.rows.get_mut(at) else {
                    continue;
                };
                if one.state == RowState::SetAside {
                    continue;
                }
                let taken = if decision == ADecision::Accept {
                    one.proposal.clone()
                } else {
                    one.answers
                        .iter()
                        .find_map(|answer| answer.rejected_guess.clone())
                };
                // **`Dependency Upload & Remap` F24: a row with nothing to take
                // is untouched**, and never a rule that folds.
                let Some(taken) = taken else { continue };
                let Ok(resolution) =
                    resolution_from_an_accepted_proposal(taken.as_bytes(), readings)
                else {
                    continue;
                };
                let answers = one
                    .answers
                    .iter()
                    .cloned()
                    .map(|mut answer| {
                        answer.resolution = Some(resolution.clone());
                        answer.rejected_guess = None;
                        answer
                    })
                    .collect();
                if !the_rows_answers_are_written(one, answers, None, record) {
                    return WhatADecisionCameTo::NotSaved;
                }
                took = true;
            }
        }
        ADecision::Locate(path) => {
            let Some(at) = row else {
                return WhatADecisionCameTo::NothingToTake;
            };
            let Some(one) = sitting.rows.get_mut(at) else {
                return WhatADecisionCameTo::NothingToTake;
            };
            // **`Dependency Upload & Remap` F12: the whole location as stored
            // on disk**, read from the system at the moment of the act.
            let Ok(stored) = the_stored_location_of(&path, readings) else {
                return WhatADecisionCameTo::NothingToTake;
            };
            let a_folder =
                one.wanted.first().map(|wanted| wanted.shape) == Some(WantedShape::ASetsFolder);
            let resolution = Resolution {
                to: if a_folder {
                    ResolvedTo::AFolderForASetRow(stored.clone())
                } else {
                    ResolvedTo::AWholeLocation(stored.clone())
                },
                how: HowResolved::TheUsersOwnHand,
            };
            let answers: Vec<LocatedAnswer> = one
                .answers
                .iter()
                .cloned()
                .map(|mut answer| {
                    answer.resolution = Some(resolution.clone());
                    answer.set_aside = false;
                    answer
                })
                .collect();
            let old = one.answers.first().and_then(|answer| {
                the_full_old_location(
                    &answer.key.reference,
                    the_source_folder(&one.holding).as_deref(),
                )
            });
            if !the_rows_answers_are_written(one, answers, None, record) {
                return WhatADecisionCameTo::NotSaved;
            }
            took = true;
            // **`Dependency Upload & Remap` F16: one hand-locate may teach a
            // shift**, and F18 resolves every other row under the old root by
            // looking.
            if let Some(Ok(shift)) = old.map(|old| derive(&old, stored.as_bytes())) {
                let mut taught = None;
                // **`Dependency Upload & Remap` K6: the shift record first**,
                // and the answers it resolves after it (F20 needs only that
                // every answer names it).
                let _ = written(record, |keeper| {
                    match keeper.a_shift_is_taught(
                        shift.old_root.clone(),
                        shift.new_root.clone(),
                        SystemTime::now(),
                    ) {
                        Ok(record) => {
                            taught = Some(record.key);
                            true
                        }
                        Err(_) => false,
                    }
                });
                if let Some(shift_key) = taught {
                    sitting.the_shift = Some(TheShiftInHand {
                        old_root: shift.old_root.clone(),
                        new_root: shift.new_root.clone(),
                        key: shift_key,
                    });
                    for (other, row) in sitting.rows.iter_mut().enumerate() {
                        if other == at || row.state != RowState::NotFound {
                            continue;
                        }
                        let source = the_source_folder(&row.holding);
                        let mut found = None;
                        for (answer, wanted) in row.answers.iter().zip(&row.wanted) {
                            let Some(old) =
                                the_full_old_location(&answer.key.reference, source.as_deref())
                            else {
                                continue;
                            };
                            if let ShiftOutcome::Resolved(stored) =
                                apply_to_one(&shift, &old, wanted.shape, readings)
                            {
                                found = Some((stored, wanted.shape));
                                break;
                            }
                        }
                        let Some((stored, shape)) = found else {
                            continue;
                        };
                        let resolution = Resolution {
                            to: if shape == WantedShape::ASetsFolder {
                                ResolvedTo::AFolderForASetRow(stored)
                            } else {
                                ResolvedTo::AWholeLocation(stored)
                            },
                            how: HowResolved::AShift(shift_key),
                        };
                        let answers = row
                            .answers
                            .iter()
                            .cloned()
                            .map(|mut answer| {
                                answer.resolution = Some(resolution.clone());
                                answer
                            })
                            .collect();
                        if !the_rows_answers_are_written(row, answers, None, record) {
                            return WhatADecisionCameTo::NotSaved;
                        }
                    }
                }
            }
        }
        ADecision::ChooseAFolder(folder) => {
            for at in reached {
                let Some(one) = sitting.rows.get_mut(at) else {
                    continue;
                };
                if one.state != RowState::NotFound {
                    continue;
                }
                let Some(wanted) = one.wanted.first().cloned() else {
                    continue;
                };
                let name = wanted.wanted_name().map(<[u8]>::to_vec).unwrap_or_else(|| {
                    wanted
                        .the_sets_pattern()
                        .map(<[u8]>::to_vec)
                        .unwrap_or_default()
                });
                match look_for(
                    &smoozy_dependencies::bytes::join(&folder, &name),
                    wanted.shape,
                    readings,
                ) {
                    ShiftOutcome::Resolved(stored) => {
                        let resolution = Resolution {
                            to: if wanted.shape == WantedShape::ASetsFolder {
                                ResolvedTo::AFolderForASetRow(stored)
                            } else {
                                ResolvedTo::AWholeLocation(stored)
                            },
                            how: HowResolved::TheUsersOwnHand,
                        };
                        let answers = one
                            .answers
                            .iter()
                            .cloned()
                            .map(|mut answer| {
                                answer.resolution = Some(resolution.clone());
                                answer
                            })
                            .collect();
                        if !the_rows_answers_are_written(one, answers, None, record) {
                            return WhatADecisionCameTo::NotSaved;
                        }
                        took = true;
                    }
                    ShiftOutcome::AProposal(path) => {
                        one.proposal = Some(StoredLocation::as_the_system_returned_it(&path));
                        took = true;
                    }
                    ShiftOutcome::Unchanged | ShiftOutcome::TheSystemCannotSay => {}
                }
            }
        }
        ADecision::UndoTheShift => {
            let Some(shift) = sitting.the_shift.take() else {
                return WhatADecisionCameTo::NothingToTake;
            };
            for one in sitting.rows.iter_mut() {
                if !one
                    .answers
                    .iter()
                    .any(|answer| answer.was_produced_by(shift.key))
                {
                    continue;
                }
                let answers = one
                    .answers
                    .iter()
                    .cloned()
                    .map(|mut answer| {
                        if answer.was_produced_by(shift.key) {
                            answer.resolution = None;
                        }
                        answer
                    })
                    .collect();
                if !the_rows_answers_are_written(one, answers, None, record) {
                    sitting.the_shift = Some(shift);
                    return WhatADecisionCameTo::NotSaved;
                }
            }
            took = true;
        }
        ADecision::AddTheChosenSearchFolder => {
            let Some(folder) = sitting.the_chosen_search_folder.take() else {
                return WhatADecisionCameTo::NothingToTake;
            };
            // **Added to Settings' own list, in that list's order** — one door
            // and one writer (`Dependency Upload & Remap` F1).
            let mut list = None;
            let there = record.with_the_record(&mut |connection| {
                let Ok(read) = the_searched_folder_list::read(connection) else {
                    return;
                };
                let with =
                    crate::surface::settings::the_list_with(&read, folder.clone()).unwrap_or(read);
                if the_searched_folder_list::write(connection, &with).is_ok() {
                    list = Some(with);
                }
            });
            let Some(list) = list.filter(|_| there) else {
                return WhatADecisionCameTo::NotSaved;
            };
            // **The search runs again over the rows still not found, and
            // nothing else** — rows already answered are not re-decided.
            let open: Vec<usize> = (0..sitting.rows.len())
                .filter(|at| sitting.rows[*at].state == RowState::NotFound)
                .collect();
            let wanted: Vec<Outstanding> = open
                .iter()
                .flat_map(|at| sitting.rows[*at].wanted.iter().cloned())
                .collect();
            let outcome = search(&list, &wanted, readings, &mut || false, SystemTime::now());
            let mut the_matches_came_from = None;
            // **What the add found, counted over the rows it searched for** —
            // said once on the panel (`Design/Dependency Upload & Remap § The
            // search folders on this panel`, the Creator's note 5 on 0.1.8).
            let searched_for = open.len();
            let mut found = 0;
            for at in open {
                let one = &mut sitting.rows[at];
                let mut resolved = None;
                let mut proposal = None;
                for wanted in &one.wanted {
                    let candidates = outcome
                        .candidates
                        .get(&wanted.reference)
                        .cloned()
                        .unwrap_or_default();
                    if let Some(found) = the_answer_for(&list, &candidates) {
                        if let Ok(resolution) = resolution_from_the_search(&found, readings) {
                            the_matches_came_from.get_or_insert(found.folder.clone());
                            resolved = Some(resolution);
                            break;
                        }
                    }
                    let rejected = one
                        .answers
                        .iter()
                        .find_map(|answer| answer.rejected_guess.clone());
                    proposal = proposal.or_else(|| {
                        the_proposals_for(&candidates, rejected.as_ref())
                            .into_iter()
                            .next()
                            .map(|path| StoredLocation::as_the_system_returned_it(&path))
                    });
                }
                match resolved {
                    Some(resolution) => {
                        let answers = one
                            .answers
                            .iter()
                            .cloned()
                            .map(|mut answer| {
                                answer.resolution = Some(resolution.clone());
                                answer
                            })
                            .collect();
                        if !the_rows_answers_are_written(one, answers, None, record) {
                            return WhatADecisionCameTo::NotSaved;
                        }
                        found += 1;
                    }
                    None => one.proposal = proposal.or(one.proposal.clone()),
                }
            }
            sitting.the_folder_added = Some(crate::surface::the_dependency_check::TheFolderAdded {
                folder: folder.clone(),
                found,
                still_not_found: searched_for - found,
            });
            sitting.the_list_it_searched = list.folders.clone();
            sitting.the_search = Some(TheSearchInHand {
                folders: list
                    .folders
                    .iter()
                    .map(|folder| {
                        let verdict = outcome
                            .report
                            .per_folder
                            .iter()
                            .find(|one| &one.folder == folder)
                            .map(|one| one.verdict);
                        (folder.clone(), verdict)
                    })
                    .collect(),
                the_matches_came_from,
                running: false,
                // `Dependency Upload & Remap` F1a: the moment of this search,
                // as its own report took it.
                searched_at: outcome.report.per_folder.first().map(|one| one.at.taken_at()),
            });
            took = true;
        }
    }
    if took {
        WhatADecisionCameTo::Taken
    } else {
        WhatADecisionCameTo::NothingToTake
    }
}

/// **The key a shift record was written under**, for a test that reads one
/// back.
pub fn the_shift_on_the_panel(held: &WhatTheRunHolds, key: &str) -> Option<ShiftKey> {
    held.sittings
        .get(key)
        .and_then(|sitting| sitting.the_shift.as_ref())
        .map(|shift| shift.key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_dependencies::check::{ReadReference, StoredSource};
    use smoozy_server_protocol::asking_the_supervisor::WhatCameBack;
    use smoozy_server_protocol::framing::{
        Answered, Fields, Request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
    };
    use smoozy_server_protocol::requests as asked;
    use smoozy_server_setup::blender::Version;
    use smoozy_state::identities::{Identity, IdentityKind};
    use std::path::{Path, PathBuf};
    use std::sync::Arc;

    // **Why five of these tests run on macOS and Linux only**, added after
    // hosted run `36742065126` of 2026-09-30, their first on Windows. Each
    // proves a row resolved to a whole location on the user's computer — by the
    // recorded location itself (`Dependency Upload & Remap` F27), by the search
    // (`Dependency Upload & Remap` F3), or as the key a linked library's own
    // rows are written under (`Dependency Upload & Remap` K3). **On Windows
    // none of that is built, on purpose**: the plan holds a location on a
    // Windows computer permanently unresolved (`Dependency Upload & Remap` A4,
    // and that plan's §12 item 3) and leaves the shape of a drive-lettered
    // location undecided. The code refuses a recorded location that does not
    // start at `/` as not absolute, which every Windows location is, and the
    // search composes with `/`. So these five failed there by asserting what
    // only macOS and Linux have. The other tests here run on every system.

    // ---------------------------------------------------------------- a folder

    struct AFolder {
        at: PathBuf,
    }

    impl AFolder {
        fn made(named: &str) -> AFolder {
            let at = std::env::temp_dir()
                .join(format!("smoozy-dep-piece-5-{named}-{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&at);
            std::fs::create_dir_all(&at).expect("the folder is made");
            AFolder {
                at: std::fs::canonicalize(&at).expect("it is there"),
            }
        }

        fn holds(&self, relative: &str, bytes: &[u8]) -> PathBuf {
            let path = self.at.join(relative);
            std::fs::create_dir_all(path.parent().expect("a parent")).expect("the parent is made");
            std::fs::write(&path, bytes).expect("the file is written");
            path
        }

        /// A path as the answers carry it: its own bytes on macOS and Linux,
        /// its text on Windows, where a path is not bytes.
        fn bytes(&self, relative: &str) -> Vec<u8> {
            #[cfg(unix)]
            {
                use std::os::unix::ffi::OsStrExt;
                self.at.join(relative).as_os_str().as_bytes().to_vec()
            }
            #[cfg(not(unix))]
            {
                self.at
                    .join(relative)
                    .to_str()
                    .expect("the scratch path is text")
                    .as_bytes()
                    .to_vec()
            }
        }
    }

    impl Drop for AFolder {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.at);
        }
    }

    // ---------------------------------------------------------------- a machine

    /// **A machine that answers a reading of a `.blend` from a script**: each
    /// file's document, a failing status, or — after a number of readings —
    /// nothing at all, the line having gone.
    #[derive(Default)]
    struct AScriptedMachine {
        documents: BTreeMap<Vec<u8>, Option<ReadingDocument>>,
        streams: BTreeMap<Vec<u8>, Vec<u8>>,
        read: Vec<Vec<u8>>,
        the_line_goes_after: Option<usize>,
        no_four_values: bool,
        after_each_reading: Option<Arc<dyn Fn() + Send + Sync>>,
    }

    impl smoozy_dependencies::acting::checking::TheMachineAReadingIsAskedOf for AScriptedMachine {
        async fn ask(&mut self, request: &Request) -> WhatCameBack {
            if request.name != asked::THE_READING_OF_A_BLEND_REQUEST {
                return WhatCameBack::NothingCorrelatedCameBack;
            }
            if self
                .the_line_goes_after
                .is_some_and(|after| self.read.len() >= after)
            {
                return WhatCameBack::ItWasNeverAsked;
            }
            let blend = request
                .fields
                .value(asked::THE_BLEND_FIELD)
                .expect("the blend")
                .to_vec();
            self.read.push(blend.clone());
            let at = format!("/streams/{}", self.read.len()).into_bytes();
            let (stream, status) = match self.documents.get(&blend) {
                Some(Some(document)) => (
                    [
                        b"Blender quit\n".as_slice(),
                        smoozy_dependencies::acting::blender::THE_OPENING_SENTINEL,
                        &serde_json::to_vec(document).expect("it writes"),
                        smoozy_dependencies::acting::blender::THE_CLOSING_SENTINEL,
                        b"\n",
                    ]
                    .concat(),
                    "0",
                ),
                _ => (b"Error: cannot read file\n".to_vec(), "1"),
            };
            self.streams.insert(at.clone(), stream.clone());
            let mut empty = at.clone();
            empty.extend_from_slice(b"-complaints");
            self.streams.insert(empty.clone(), Vec::new());
            if let Some(after) = &self.after_each_reading {
                after();
            }
            WhatCameBack::ItAnswered(
                Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "0.1.0").carrying(
                    Fields::none()
                        .and(
                            asked::WHAT_BECAME_OF_IT_FIELD,
                            asked::IT_HAS_A_RECORDED_OUTCOME,
                        )
                        .and(asked::THE_EXIT_STATUS_FIELD, status)
                        .and(asked::THE_ORDINARY_STREAM_FIELD, at)
                        .and(
                            asked::THE_ORDINARY_STREAM_LENGTH_FIELD,
                            stream.len().to_string(),
                        )
                        .and(asked::THE_COMPLAINTS_STREAM_FIELD, empty)
                        .and(asked::THE_COMPLAINTS_STREAM_LENGTH_FIELD, "0"),
                ),
            )
        }

        async fn read_the_stream_back(
            &mut self,
            path: &[u8],
            length: u64,
        ) -> Result<Vec<u8>, String> {
            let stream = self.streams.get(path).cloned().unwrap_or_default();
            Ok(stream[..(length as usize).min(stream.len())].to_vec())
        }
    }

    impl TheMachineAFileIsReadOn for AScriptedMachine {
        async fn the_four_values_of(&mut self, _the_blend: &str) -> FourValuesNow {
            if self.no_four_values {
                return FourValuesNow::TheChannelCannotAnswer;
            }
            FourValuesNow::Answered(SourceReading {
                size: 1,
                inode: "7".to_string(),
                modification_time: 1,
                change_time: 1,
            })
        }
    }

    // ---------------------------------------------------------------- a reading

    fn a_reference(kind: Kind, recorded: &[u8]) -> ReadReference {
        ReadReference {
            kind,
            datablock: String::from_utf8_lossy(recorded).to_string(),
            setting: "filepath".to_string(),
            recorded: RecordedLocation::as_the_reading_carried_it(recorded),
            scenes: Attribution::Established(vec!["Scene".to_string()]),
            source: (kind == Kind::Image).then(|| "FILE".to_string()),
        }
    }

    fn a_document(references: Vec<ReadReference>) -> ReadingDocument {
        ReadingDocument {
            references,
            flat_list: Vec::new(),
            weak_references: Vec::new(),
            observed_stand_ins: Vec::new(),
        }
    }

    fn a_blender() -> BlenderIdentity {
        BlenderIdentity {
            path: "/root/blender/blender-5.2.2/blender".to_string(),
            version: Version {
                major: 5,
                minor: 2,
                patch: 2,
            },
        }
    }

    fn a_machine_key() -> MachineKey {
        MachineKey::of(
            Identity::issue(IdentityKind::LocalServer).expect("randomness"),
            Identity::issue(IdentityKind::Installation).expect("randomness"),
        )
    }

    fn a_sitting_of(folder: &AFolder, root: &str) -> TheSittingIsOf {
        let machine = a_machine_key();
        TheSittingIsOf {
            server: machine.server_row.as_text(),
            file: String::from_utf8(folder.bytes(root)).expect("text"),
            root: MachineLocation::composed(folder.bytes(root)),
            holding: HoldingFile::OnTheUsersComputer(StoredLocation::as_the_system_returned_it(
                &folder.bytes(root),
            )),
            with: a_blender(),
            machine,
            layout: None,
        }
    }

    fn a_record(folder: &AFolder) -> crate::record::LocalRecord {
        crate::record::LocalRecord::open_at(&folder.at.join("record.sqlite3"))
            .expect("the record opens")
    }

    fn run<F: std::future::Future>(future: F) -> F::Output {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("a runtime")
            .block_on(future)
    }

    /// **A machine whose reading runs on**: every ask of a reading says it is
    /// still in hand, and the first one presses the check's own *Stop* — the
    /// way a user meets a reading of a heavy file. Every stop it is sent is
    /// written down.
    struct AReadingThatRunsOn {
        held: Arc<Mutex<WhatTheRunHolds>>,
        key: String,
        readings_asked: Vec<Identity>,
        stops: Vec<Request>,
    }

    impl smoozy_dependencies::acting::checking::TheMachineAReadingIsAskedOf for AReadingThatRunsOn {
        async fn ask(&mut self, request: &Request) -> WhatCameBack {
            let empty = Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "0.1.8");
            if request.name == asked::THE_STOP_REQUEST {
                self.stops.push(request.clone());
                return WhatCameBack::ItAnswered(
                    empty.carrying(Fields::none().and(asked::WHAT_BECAME_OF_IT_FIELD, asked::IT_IS_STILL_IN_HAND)),
                );
            }
            self.readings_asked.push(request.identity.clone());
            if let Ok(mut held) = self.held.lock() {
                if let Some(stop) = held.checks_running.get_mut(&self.key) {
                    *stop = true;
                }
            }
            WhatCameBack::ItAnswered(
                empty.carrying(Fields::none().and(asked::WHAT_BECAME_OF_IT_FIELD, asked::IT_IS_STILL_IN_HAND)),
            )
        }

        async fn read_the_stream_back(&mut self, _path: &[u8], _length: u64) -> Result<Vec<u8>, String> {
            Err("nothing ended".to_string())
        }
    }

    impl TheMachineAFileIsReadOn for AReadingThatRunsOn {
        async fn the_four_values_of(&mut self, _the_blend: &str) -> FourValuesNow {
            FourValuesNow::Answered(SourceReading {
                size: 5,
                inode: "1".to_string(),
                modification_time: 1,
                change_time: 1,
            })
        }
    }

    /// **The 38's piece 7, item 36: *Stop* while the file itself is being read
    /// sends the machine the stop, naming that reading** (`Dependency Upload &
    /// Remap` G15, `Server protocol and transfers` C21), **and the panel says
    /// the read was stopped** (`Render Settings` Q8's *a read Smoozy ended
    /// itself*, drawn `chk-unstop`). Until then the stop was never sent and the
    /// Blender read on.
    #[cfg(unix)]
    #[test]
    fn a_stop_while_the_file_itself_is_read_is_sent_to_the_machine_and_the_read_says_it_was_stopped() {
        let folder = AFolder::made("stopped");
        folder.holds("project/scene.blend", b"blend");
        let sitting = a_sitting_of(&folder, "project/scene.blend");
        let record = a_record(&folder);
        let held = Arc::new(Mutex::new(WhatTheRunHolds::default()));
        let mut machine = AReadingThatRunsOn {
            held: Arc::clone(&held),
            key: keyed(&sitting.server, &sitting.file),
            readings_asked: Vec::new(),
            stops: Vec::new(),
        };
        let came_to = run(the_check_runs(
            &mut machine,
            &OverOneConnection(record.connection()),
            &TheUsersOwnComputer,
            &held,
            &sitting,
            &|| {},
        ));
        assert_eq!(came_to, TheCheckCameTo::ItGaveNothing(Ending::SmoozyStoppedIt));
        assert_eq!(machine.stops.len(), 1, "one stop, sent once");
        assert_eq!(
            machine.stops[0].fields.text(asked::THE_OPERATION_TO_STOP_FIELD),
            Some(machine.readings_asked[0].as_text().as_str()),
            "the stop names the reading on the machine"
        );
        let held = held.lock().expect("the run");
        assert!(held.checks_running.is_empty() && held.files_being_read.is_empty());
    }

    /// **The whole sitting, on a real record and the user's real disk**:
    /// every finding written as it lands, the rows on the panel, and the
    /// standing written at the end — and **DW0-2's bake output path writes no
    /// answer and makes no row**.
    ///
    /// **macOS and Linux only** — *why five of these tests run on macOS and
    /// Linux only*, above `AFolder`, says why.
    #[cfg(unix)]
    #[test]
    fn a_sitting_writes_every_finding_and_its_standing_and_the_panel_draws_its_rows() {
        let folder = AFolder::made("whole");
        folder.holds("project/scene.blend", b"blend");
        folder.holds("project/tex/wood.png", b"wood");
        let sitting = a_sitting_of(&folder, "project/scene.blend");
        let mut machine = AScriptedMachine::default();
        machine.documents.insert(
            sitting.root.as_bytes().to_vec(),
            Some(a_document(vec![
                a_reference(Kind::Image, b"//tex/wood.png"),
                a_reference(Kind::Image, b"//tex/gone.png"),
                a_reference(Kind::BakeFolder, b"//"),
            ])),
        );
        let record = a_record(&folder);
        let held = Mutex::new(WhatTheRunHolds::default());
        let told = std::sync::atomic::AtomicUsize::new(0);
        let came_to = run(the_check_runs(
            &mut machine,
            &OverOneConnection(record.connection()),
            &TheUsersOwnComputer,
            &held,
            &sitting,
            &|| {
                told.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            },
        ));
        assert_eq!(
            came_to,
            TheCheckCameTo::TheSittingStands(Completion::Completed)
        );
        assert!(
            told.load(std::sync::atomic::Ordering::SeqCst) >= 2,
            "the page was not told as rows landed"
        );

        let keeper = TheDependencyRecord::over(record.connection());
        let answers = keeper
            .the_answers_of(&sitting.holding)
            .expect("the answers read");
        assert_eq!(
            answers.len(),
            2,
            "every finding but the dropped bake output path: {answers:#?}"
        );
        assert!(answers
            .iter()
            .all(|answer| answer.key.reference.as_bytes() != b"//"));
        let wood = answers
            .iter()
            .find(|answer| answer.key.reference.as_bytes() == b"//tex/wood.png")
            .expect("wood's answer");
        assert_eq!(
            wood.resolution
                .as_ref()
                .map(|resolution| resolution.how.clone()),
            Some(HowResolved::TheRecordedLocationItself)
        );

        let standing = keeper
            .the_standing(&sitting.root, &sitting.machine)
            .expect("it reads")
            .expect("a standing was written");
        assert_eq!(standing.completion, Completion::Completed);
        assert_eq!(standing.counts.not_found, 1);
        assert_eq!(standing.counts.found_but_not_sent, 1);
        assert!(!standing.every_row_confirmed_or_set_aside);

        let held = held.lock().expect("the run");
        let key = keyed(&sitting.server, &sitting.file);
        let panel = crate::surface::the_dependency_check::the_panel(
            &held,
            &sitting.server,
            &sitting.file,
            None,
        );
        let table = panel.the_table.expect("rows");
        let drawn: Vec<(&str, &str)> = table
            .groups
            .iter()
            .flat_map(|group| {
                group
                    .rows
                    .iter()
                    .map(move |row| (group.which.as_str(), row.the_file_says.as_str()))
            })
            .collect();
        assert_eq!(
            drawn,
            vec![("not-found", "//tex/gone.png"), ("found", "//tex/wood.png")]
        );
        assert!(
            held.checks_running.get(&key).is_none(),
            "the sitting is still marked running"
        );
        let page = crate::surface::render_settings::the_run_holds()
            .lock()
            .expect("the page");
        assert!(matches!(
            page.the_check_record.get(&key),
            Some(WhatTheRecordSays::AStanding(_))
        ));
    }

    /// **While the file itself is being read, the panel says *Reading the
    /// file…* and draws no table, no tools and no foot; once its reading
    /// lands, the rows** — `Design/Dependency Upload & Remap § While it is
    /// still reading below a linked file`, its closing paragraph, and the
    /// Creator's note 8 on the test copy: until then the panel drew an empty
    /// table for as long as the root was read, the empty list `Dependency
    /// Upload & Remap` G3 forbids. **The mark comes off on every way out**: a
    /// root that gave nothing, and a sitting that stands.
    ///
    /// **macOS and Linux only**, as the test above.
    #[cfg(unix)]
    #[test]
    fn a_check_reading_its_own_file_says_so_and_draws_no_table_until_it_lands() {
        let folder = AFolder::made("reading-the-file");
        folder.holds("project/scene.blend", b"blend");
        let sitting = a_sitting_of(&folder, "project/scene.blend");
        let held = Arc::new(Mutex::new(WhatTheRunHolds::default()));
        let seen = Arc::new(Mutex::new(Vec::<String>::new()));
        // **What the panel composes at a moment**: its header's own line, and
        // whether a table, tools or a foot is drawn.
        let composed = {
            let (held, server, file) = (Arc::clone(&held), sitting.server.clone(), sitting.file.clone());
            move || {
                let held = held.lock().expect("the run");
                let panel =
                    crate::surface::the_dependency_check::the_panel(&held, &server, &file, None);
                format!(
                    "{} | table {} | tools {} | foot {} | nothing was read {}",
                    panel
                        .still_reading
                        .as_ref()
                        .map(|words| words.said().replace('\u{a0}', " "))
                        .unwrap_or_else(|| "no line".to_string()),
                    panel.the_table.as_ref().map_or("none".to_string(), |table| {
                        table.groups.iter().map(|group| group.rows.len()).sum::<usize>().to_string()
                    }),
                    panel.the_tools.is_some(),
                    panel.the_foot.is_some(),
                    panel.nothing_was_read.is_some(),
                )
            }
        };
        let mut machine = AScriptedMachine::default();
        machine.documents.insert(
            sitting.root.as_bytes().to_vec(),
            Some(a_document(vec![a_reference(Kind::Image, b"//tex/gone.png")])),
        );
        // **The moment the machine has answered the root's reading and it has
        // not yet landed** — what the user sees for as long as Blender reads.
        let (while_reading, seen_then) = (composed.clone(), Arc::clone(&seen));
        machine.after_each_reading = Some(Arc::new(move || {
            let now = while_reading();
            seen_then.lock().expect("seen").push(now);
        }));
        let record = a_record(&folder);
        // **And each time the page is told**, which is when it asks again.
        let (as_told, seen_when_told) = (composed.clone(), Arc::clone(&seen));
        let came_to = run(the_check_runs(
            &mut machine,
            &OverOneConnection(record.connection()),
            &TheUsersOwnComputer,
            &held,
            &sitting,
            &move || {
                let now = as_told();
                seen_when_told.lock().expect("seen").push(now);
            },
        ));
        assert_eq!(came_to, TheCheckCameTo::TheSittingStands(Completion::Completed));
        let reading = "Reading the file… | table none | tools false | foot false | nothing was read false";
        let landed = "no line | table 1 | tools true | foot false | nothing was read false";
        assert_eq!(
            seen.lock().expect("seen").as_slice(),
            [
                // Told: the fresh sitting is in, and the root is next.
                reading.to_string(),
                reading.to_string(),
                // The machine has answered, and the reading has not landed.
                reading.to_string(),
                // Told: the root's reading landed.
                landed.to_string(),
            ],
            "while the file itself was read, and once its reading landed"
        );
        assert_eq!(composed(), landed, "once the sitting stands");

        // **A root that gave nothing takes the mark off too**, and the panel
        // then says how it ended.
        let mut machine = AScriptedMachine::default();
        machine.documents.insert(sitting.root.as_bytes().to_vec(), None);
        let came_to = run(the_check_runs(
            &mut machine,
            &OverOneConnection(record.connection()),
            &TheUsersOwnComputer,
            &held,
            &sitting,
            &|| {},
        ));
        assert!(matches!(came_to, TheCheckCameTo::ItGaveNothing(_)));
        assert!(
            composed().starts_with("no line"),
            "a root that gave nothing is still said to be reading: {}",
            composed()
        );
    }

    /// **A file that holds nothing outside itself: complete, nothing
    /// unfound, and the standing says so** — which stops `Rendering`'s
    /// warning for the Blender it read with.
    #[test]
    fn a_file_with_nothing_outside_it_stands_complete_and_stops_the_warning() {
        let folder = AFolder::made("nothing");
        folder.holds("scene.blend", b"blend");
        let sitting = a_sitting_of(&folder, "scene.blend");
        let mut machine = AScriptedMachine::default();
        machine.documents.insert(
            sitting.root.as_bytes().to_vec(),
            Some(a_document(vec![a_reference(Kind::BakeFolder, b"//")])),
        );
        let record = a_record(&folder);
        let held = Mutex::new(WhatTheRunHolds::default());
        let came_to = run(the_check_runs(
            &mut machine,
            &OverOneConnection(record.connection()),
            &TheUsersOwnComputer,
            &held,
            &sitting,
            &|| {},
        ));
        assert_eq!(
            came_to,
            TheCheckCameTo::TheSittingStands(Completion::Completed)
        );
        let standing = TheDependencyRecord::over(record.connection())
            .the_standing(&sitting.root, &sitting.machine)
            .expect("it reads")
            .expect("written");
        assert!(standing.every_row_confirmed_or_set_aside);
        assert_eq!(
            smoozy_rendering::warning::evaluate(
                &WhatTheRecordSays::AStanding(Box::new(standing)),
                &a_blender()
            ),
            smoozy_rendering::warning::TheWarningsCase::Absent,
        );
    }

    /// **`Dependency Upload & Remap` G16: a connection cut below the root
    /// leaves the sitting standing**, what was found on screen and in the
    /// record, the band saying it is incomplete, and the standing written
    /// incomplete — which stops nothing.
    #[test]
    fn a_connection_cut_below_the_root_leaves_the_sitting_standing_and_incomplete() {
        let folder = AFolder::made("cut");
        folder.holds("scene.blend", b"blend");
        folder.holds("lib/props.blend", b"library");
        folder.holds("tex/wood.png", b"wood");
        let sitting = a_sitting_of(&folder, "scene.blend");
        let mut machine = AScriptedMachine::default();
        machine.documents.insert(
            sitting.root.as_bytes().to_vec(),
            Some(a_document(vec![
                a_reference(Kind::Library, b"//lib/props.blend"),
                a_reference(Kind::Image, b"//tex/wood.png"),
            ])),
        );
        machine.the_line_goes_after = Some(1);
        let record = a_record(&folder);
        let held = Mutex::new(WhatTheRunHolds::default());
        let came_to = run(the_check_runs(
            &mut machine,
            &OverOneConnection(record.connection()),
            &TheUsersOwnComputer,
            &held,
            &sitting,
            &|| {},
        ));
        assert_eq!(
            came_to,
            TheCheckCameTo::TheSittingStands(Completion::TheConnectionDropped)
        );
        assert_eq!(
            machine.read,
            vec![sitting.root.as_bytes().to_vec()],
            "the library was never read"
        );
        let keeper = TheDependencyRecord::over(record.connection());
        assert_eq!(
            keeper
                .the_answers_of(&sitting.holding)
                .expect("they read")
                .len(),
            2
        );
        let standing = keeper
            .the_standing(&sitting.root, &sitting.machine)
            .expect("it reads")
            .expect("an incomplete standing is still written");
        assert_eq!(standing.completion, Completion::TheConnectionDropped);
        let held = held.lock().expect("the run");
        let panel = crate::surface::the_dependency_check::the_panel(
            &held,
            &sitting.server,
            &sitting.file,
            None,
        );
        assert!(panel
            .the_statements
            .iter()
            .any(|statement| statement.which.as_str() == "the-connection-dropped"));
        assert_eq!(
            panel
                .the_table
                .expect("rows")
                .groups
                .iter()
                .map(|group| group.rows.len())
                .sum::<usize>(),
            2
        );
    }

    /// **The library below the root is read against the file that holds it**
    /// — `Dependency Upload & Remap` G14 — and its own rows are keyed to the
    /// library's own source location (`Dependency Upload & Remap` K3) and never stop the warning (the runner's second ruling in the dependency map).
    ///
    /// **macOS and Linux only** — *why five of these tests run on macOS and
    /// Linux only*, above `AFolder`, says why.
    #[cfg(unix)]
    #[test]
    fn a_linked_blend_is_read_against_the_file_that_holds_it() {
        let folder = AFolder::made("below");
        folder.holds("scene.blend", b"blend");
        folder.holds("lib/props.blend", b"library");
        folder.holds("lib/tex/chair.png", b"chair");
        let sitting = a_sitting_of(&folder, "scene.blend");
        let mut machine = AScriptedMachine::default();
        machine.documents.insert(
            sitting.root.as_bytes().to_vec(),
            Some(a_document(vec![a_reference(
                Kind::Library,
                b"//lib/props.blend",
            )])),
        );
        machine.documents.insert(
            folder.bytes("lib/props.blend"),
            Some(a_document(vec![a_reference(
                Kind::Image,
                b"//tex/chair.png",
            )])),
        );
        let record = a_record(&folder);
        let held = Mutex::new(WhatTheRunHolds::default());
        let came_to = run(the_check_runs(
            &mut machine,
            &OverOneConnection(record.connection()),
            &TheUsersOwnComputer,
            &held,
            &sitting,
            &|| {},
        ));
        assert_eq!(
            came_to,
            TheCheckCameTo::TheSittingStands(Completion::Completed)
        );
        assert_eq!(machine.read.len(), 2);
        let library = HoldingFile::OnTheUsersComputer(StoredLocation::as_the_system_returned_it(
            &folder.bytes("lib/props.blend"),
        ));
        let below = TheDependencyRecord::over(record.connection())
            .the_answers_of(&library)
            .expect("they read");
        assert_eq!(below.len(), 1);
        assert!(
            below[0].resolution.is_some(),
            "chair.png was measured from the library's own folder"
        );
        let held = held.lock().expect("the run");
        let sitting_held = &held.sittings[&keyed(&sitting.server, &sitting.file)];
        assert_eq!(sitting_held.linked_files_read, 1);
        assert_eq!(sitting_held.rows.len(), 2);
    }

    /// **Nothing to read the four values with is `Render Settings` Q8's first
    /// ending, and nothing is asked of the supervisor**; **a failing status at
    /// the root is *could not be read* with the version** (`Dependency Upload
    /// & Remap` G3), never an empty list.
    #[test]
    fn a_root_that_gives_nothing_is_one_of_the_endings_and_never_an_empty_list() {
        let folder = AFolder::made("nothing-read");
        folder.holds("scene.blend", b"blend");
        let sitting = a_sitting_of(&folder, "scene.blend");
        let record = a_record(&folder);

        let mut unreachable = AScriptedMachine {
            no_four_values: true,
            ..AScriptedMachine::default()
        };
        let held = Mutex::new(WhatTheRunHolds::default());
        let came_to = run(the_check_runs(
            &mut unreachable,
            &OverOneConnection(record.connection()),
            &TheUsersOwnComputer,
            &held,
            &sitting,
            &|| {},
        ));
        assert_eq!(
            came_to,
            TheCheckCameTo::ItGaveNothing(Ending::TheMachineCouldNotBeReached)
        );
        assert!(unreachable.read.is_empty());

        let mut failing = AScriptedMachine::default();
        failing
            .documents
            .insert(sitting.root.as_bytes().to_vec(), None);
        let came_to = run(the_check_runs(
            &mut failing,
            &OverOneConnection(record.connection()),
            &TheUsersOwnComputer,
            &held,
            &sitting,
            &|| {},
        ));
        assert_eq!(
            came_to,
            TheCheckCameTo::ItGaveNothing(Ending::TheFileCouldNotBeOpenedByThisVersion)
        );
        assert!(
            TheDependencyRecord::over(record.connection())
                .the_standing(&sitting.root, &sitting.machine)
                .expect("it reads")
                .is_none(),
            "a reading that gave nothing wrote a standing"
        );
    }

    /// **A stop asked while a reading runs throws its answer away and the
    /// sitting is written stopped** — `Dependency Upload & Remap` G15 and V10.
    #[test]
    fn a_stop_asked_while_a_reading_runs_ends_the_sitting_stopped() {
        let folder = AFolder::made("stop");
        folder.holds("scene.blend", b"blend");
        folder.holds("tex/wood.png", b"wood");
        let sitting = a_sitting_of(&folder, "scene.blend");
        let held: &'static Mutex<WhatTheRunHolds> =
            Box::leak(Box::new(Mutex::new(WhatTheRunHolds::default())));
        let key = keyed(&sitting.server, &sitting.file);
        let mut machine = AScriptedMachine::default();
        machine.documents.insert(
            sitting.root.as_bytes().to_vec(),
            Some(a_document(vec![a_reference(
                Kind::Image,
                b"//tex/wood.png",
            )])),
        );
        let asked_to_stop = key.clone();
        machine.after_each_reading = Some(Arc::new(move || {
            if let Ok(mut held) = held.lock() {
                crate::surface::the_dependency_check::the_act_is_taken(
                    &mut held,
                    asked_to_stop.split('\u{1f}').next().unwrap_or_default(),
                    asked_to_stop.split('\u{1f}').nth(1).unwrap_or_default(),
                    crate::surface::the_dependency_check::TheAct::StopTheSearch,
                    None,
                );
            }
        }));
        let record = a_record(&folder);
        let came_to = run(the_check_runs(
            &mut machine,
            &OverOneConnection(record.connection()),
            &TheUsersOwnComputer,
            held,
            &sitting,
            &|| {},
        ));
        assert_eq!(
            came_to,
            TheCheckCameTo::TheSittingStands(Completion::StoppedByTheUser)
        );
        assert!(
            TheDependencyRecord::over(record.connection())
                .the_answers_of(&sitting.holding)
                .expect("they read")
                .is_empty(),
            "the answer of a reading stopped in flight was kept"
        );
        assert_eq!(
            TheDependencyRecord::over(record.connection())
                .the_standing(&sitting.root, &sitting.machine)
                .expect("it reads")
                .expect("written")
                .completion,
            Completion::StoppedByTheUser
        );
    }

    /// **Every decision about the user's own computer is written and read
    /// back, and a fresh check re-decides none of them** — `Dependency Upload
    /// & Remap` K4, `Dependency Upload & Remap` G17, `Dependency Upload & Remap`
    /// F16, `Dependency Upload & Remap` F20 and `Dependency Upload & Remap` F22.
    ///
    /// **macOS and Linux only** — *why five of these tests run on macOS and
    /// Linux only*, above `AFolder`, says why.
    #[cfg(unix)]
    #[test]
    fn every_decision_is_saved_and_a_fresh_check_re_decides_none() {
        let folder = AFolder::made("decisions");
        folder.holds("scene.blend", b"blend");
        folder.holds("elsewhere/maps/rust.png", b"rust");
        folder.holds("elsewhere/maps/dirt.png", b"dirt");
        let sitting = a_sitting_of(&folder, "scene.blend");
        let mut old = folder.bytes("gone/maps/rust.png");
        let rust = RecordedLocation::as_the_reading_carried_it(&old);
        old = folder.bytes("gone/maps/dirt.png");
        let dirt = RecordedLocation::as_the_reading_carried_it(&old);
        let document = a_document(vec![
            ReadReference {
                recorded: rust.clone(),
                ..a_reference(Kind::Image, b"x")
            },
            ReadReference {
                recorded: dirt.clone(),
                ..a_reference(Kind::Image, b"x")
            },
            a_reference(Kind::Image, b"//nowhere.png"),
        ]);
        let mut machine = AScriptedMachine::default();
        machine
            .documents
            .insert(sitting.root.as_bytes().to_vec(), Some(document));
        let record = a_record(&folder);
        let held = Mutex::new(WhatTheRunHolds::default());
        let door = OverOneConnection(record.connection());
        let _ = run(the_check_runs(
            &mut machine,
            &door,
            &TheUsersOwnComputer,
            &held,
            &sitting,
            &|| {},
        ));
        let key = keyed(&sitting.server, &sitting.file);
        let at = |held: &WhatTheRunHolds, recorded: &RecordedLocation| {
            held.sittings[&key]
                .rows
                .iter()
                .position(|row| row.row.references.contains(recorded))
                .expect("the row")
        };

        // **Locate… one file, and the shift finds its sibling by looking.**
        {
            let mut held = held.lock().expect("the run");
            let rust_at = at(&held, &rust);
            assert_eq!(
                a_decision_is_taken(
                    &mut held,
                    &key,
                    ADecision::Locate(folder.bytes("elsewhere/maps/rust.png")),
                    Some(rust_at),
                    &door,
                    &TheUsersOwnComputer,
                ),
                WhatADecisionCameTo::Taken
            );
            let dirt_at = at(&held, &dirt);
            assert_eq!(
                held.sittings[&key].rows[dirt_at].state,
                RowState::FoundOnYourComputer
            );
            assert!(held.sittings[&key].the_shift.is_some());
            // **Set aside the one nothing found.**
            let nowhere = RecordedLocation::as_the_reading_carried_it(b"//nowhere.png");
            let nowhere_at = at(&held, &nowhere);
            assert_eq!(
                a_decision_is_taken(
                    &mut held,
                    &key,
                    ADecision::SetAside,
                    Some(nowhere_at),
                    &door,
                    &TheUsersOwnComputer
                ),
                WhatADecisionCameTo::Taken
            );
        }
        let answers = TheDependencyRecord::over(record.connection())
            .the_answers_of(&sitting.holding)
            .expect("they read");
        let of = |recorded: &RecordedLocation| {
            answers
                .iter()
                .find(|answer| &answer.key.reference == recorded)
                .cloned()
                .expect("an answer")
        };
        assert_eq!(
            of(&rust).resolution.map(|resolution| resolution.how),
            Some(HowResolved::TheUsersOwnHand)
        );
        assert!(matches!(
            of(&dirt).resolution.map(|resolution| resolution.how),
            Some(HowResolved::AShift(_))
        ));
        assert!(
            of(&RecordedLocation::as_the_reading_carried_it(
                b"//nowhere.png"
            ))
            .set_aside
        );

        // **A fresh check starts from what is known**: nothing re-decided.
        let _ = run(the_check_runs(
            &mut machine,
            &door,
            &TheUsersOwnComputer,
            &held,
            &sitting,
            &|| {},
        ));
        {
            let held = held.lock().expect("the run");
            let states: Vec<RowState> = held.sittings[&key]
                .rows
                .iter()
                .map(|row| row.state)
                .collect();
            assert_eq!(
                states
                    .iter()
                    .filter(|state| **state == RowState::FoundOnYourComputer)
                    .count(),
                2,
                "{states:?}"
            );
            assert!(states.contains(&RowState::SetAside));
        }

        // **Undo the shift: every row it resolved is not found again.**
        {
            let mut held = held.lock().expect("the run");
            // The shift stood on the earlier sitting's panel; a fresh sitting
            // shows none, so it is put back as the panel held it.
            let shift_key = match of(&dirt).resolution.map(|resolution| resolution.how) {
                Some(HowResolved::AShift(key)) => key,
                _ => unreachable!(),
            };
            held.sittings.get_mut(&key).expect("the sitting").the_shift = Some(TheShiftInHand {
                old_root: Vec::new(),
                new_root: Vec::new(),
                key: shift_key,
            });
            assert_eq!(
                a_decision_is_taken(
                    &mut held,
                    &key,
                    ADecision::UndoTheShift,
                    None,
                    &door,
                    &TheUsersOwnComputer
                ),
                WhatADecisionCameTo::Taken
            );
            let dirt_at = at(&held, &dirt);
            assert_eq!(held.sittings[&key].rows[dirt_at].state, RowState::NotFound);
        }
        let answers = TheDependencyRecord::over(record.connection())
            .the_answers_of(&sitting.holding)
            .expect("they read");
        assert!(answers
            .iter()
            .find(|answer| answer.key.reference == dirt)
            .expect("dirt")
            .resolution
            .is_none());
    }

    /// **The search over the user's folders, and a folder added from the
    /// panel searches again over what is still not found** — `Dependency
    /// Upload & Remap` F1, F3 and `Design/Dependency Upload & Remap § The
    /// search folders on this panel`.
    ///
    /// **macOS and Linux only** — *why five of these tests run on macOS and
    /// Linux only*, above `AFolder`, says why.
    #[cfg(unix)]
    #[test]
    fn a_search_folder_added_from_the_panel_is_kept_and_searches_what_is_still_not_found() {
        let folder = AFolder::made("search");
        folder.holds("scene.blend", b"blend");
        folder.holds("library/stone.png", b"stone");
        // **A row the user answered by hand** (the Creator's note 5 on 0.1.8:
        // `Dependency Upload & Remap` K4, pinned): the added folder holds a
        // file of the same name, and the hand's answer stands.
        folder.holds("library/wood.png", b"wood in the library");
        folder.holds("elsewhere/wood.png", b"wood by hand");
        let sitting = a_sitting_of(&folder, "scene.blend");
        let mut machine = AScriptedMachine::default();
        machine.documents.insert(
            sitting.root.as_bytes().to_vec(),
            Some(a_document(vec![
                a_reference(Kind::Image, b"//maps/stone.png"),
                a_reference(Kind::Image, b"//maps/wood.png"),
            ])),
        );
        let record = a_record(&folder);
        let held = Mutex::new(WhatTheRunHolds::default());
        let door = OverOneConnection(record.connection());
        let _ = run(the_check_runs(
            &mut machine,
            &door,
            &TheUsersOwnComputer,
            &held,
            &sitting,
            &|| {},
        ));
        let key = keyed(&sitting.server, &sitting.file);
        let mut held = held.lock().expect("the run");
        let at_of = |held: &WhatTheRunHolds, name: &[u8]| {
            held.sittings[&key]
                .rows
                .iter()
                .position(|one| one.row.ordering_key().as_bytes().ends_with(name))
                .expect("the row")
        };
        let (stone, wood) = (at_of(&held, b"stone.png"), at_of(&held, b"wood.png"));
        assert_eq!(held.sittings[&key].rows[stone].state, RowState::NotFound);
        assert!(held.sittings[&key].the_list_it_searched.is_empty(), "the sitting read an empty list");
        assert_eq!(
            a_decision_is_taken(
                &mut held,
                &key,
                ADecision::Locate(folder.bytes("elsewhere/wood.png")),
                Some(wood),
                &door,
                &TheUsersOwnComputer
            ),
            WhatADecisionCameTo::Taken
        );
        let by_hand = held.sittings[&key].rows[wood].answers.clone();
        held.sittings
            .get_mut(&key)
            .expect("the sitting")
            .the_chosen_search_folder = Some(folder.bytes("library"));
        assert_eq!(
            a_decision_is_taken(
                &mut held,
                &key,
                ADecision::AddTheChosenSearchFolder,
                None,
                &door,
                &TheUsersOwnComputer
            ),
            WhatADecisionCameTo::Taken
        );
        assert_eq!(
            held.sittings[&key].rows[stone].state,
            RowState::FoundOnYourComputer
        );
        assert_eq!(
            the_searched_folder_list::read(record.connection())
                .expect("it reads")
                .folders,
            vec![folder.bytes("library")]
        );
        let answers = TheDependencyRecord::over(record.connection())
            .the_answers_of(&sitting.holding)
            .expect("they read");
        let stones = answers
            .iter()
            .find(|answer| answer.key.reference.as_bytes().ends_with(b"stone.png"))
            .expect("the stone's answer");
        assert!(matches!(
            stones.resolution.as_ref().map(|resolution| &resolution.how),
            Some(HowResolved::TheSearch { .. })
        ));
        // **K4: the row answered by hand is not re-decided** by the add's
        // search, on screen or in the record.
        assert_eq!(held.sittings[&key].rows[wood].answers, by_hand, "the add's search re-decided an answered row");
        let woods = answers
            .iter()
            .find(|answer| answer.key.reference.as_bytes().ends_with(b"wood.png"))
            .expect("the wood's answer");
        assert!(matches!(
            woods.resolution.as_ref().map(|resolution| &resolution.how),
            Some(HowResolved::TheUsersOwnHand)
        ));
        // **What the add found is held for the panel to say once**, counted
        // over the rows it searched for, and the list the sitting compares
        // with is the one it now searched (the Creator's note 5 on 0.1.8).
        assert_eq!(
            held.sittings[&key].the_folder_added,
            Some(crate::surface::the_dependency_check::TheFolderAdded {
                folder: folder.bytes("library"),
                found: 1,
                still_not_found: 0,
            })
        );
        assert_eq!(held.sittings[&key].the_list_it_searched, vec![folder.bytes("library")]);
    }

    /// **`Dependency Upload & Remap` F3 over F1's list, at the reading**: what
    /// the recorded location did not answer is looked for in the user's
    /// folders, and the row says the search found it.
    ///
    /// **macOS and Linux only** — *why five of these tests run on macOS and
    /// Linux only*, above `AFolder`, says why.
    #[cfg(unix)]
    #[test]
    fn the_search_runs_over_the_users_folders_for_what_the_recorded_location_did_not_answer() {
        let folder = AFolder::made("searched");
        folder.holds("scene.blend", b"blend");
        folder.holds("shelf/deep/stone.png", b"stone");
        let sitting = a_sitting_of(&folder, "scene.blend");
        let mut machine = AScriptedMachine::default();
        machine.documents.insert(
            sitting.root.as_bytes().to_vec(),
            Some(a_document(vec![a_reference(Kind::Image, b"//maps/stone.png")])),
        );
        let record = a_record(&folder);
        the_searched_folder_list::write(
            record.connection(),
            &SearchedFolderList {
                folders: vec![folder.bytes("shelf")],
            },
        )
        .expect("the list is written");
        let held = Mutex::new(WhatTheRunHolds::default());
        let _ = run(the_check_runs(
            &mut machine,
            &OverOneConnection(record.connection()),
            &TheUsersOwnComputer,
            &held,
            &sitting,
            &|| {},
        ));
        let held = held.lock().expect("the run");
        let sitting_held = &held.sittings[&keyed(&sitting.server, &sitting.file)];
        assert_eq!(sitting_held.rows[0].state, RowState::FoundOnYourComputer);
        assert_eq!(
            sitting_held.the_search.as_ref().and_then(|search| search.the_matches_came_from.clone()),
            Some(folder.bytes("shelf"))
        );
    }

    /// **`Dependency Upload & Remap` F1a: the verdict carries the moment of
    /// the search it came from.** The search's own report takes the moment;
    /// the run-held store keeps it beside the verdicts, so Settings can say
    /// *could not be read at 09:20* (step 10c's piece 17 of the 38).
    ///
    /// **macOS and Linux only** — *why five of these tests run on macOS and
    /// Linux only*, above `AFolder`, says why.
    #[cfg(unix)]
    #[test]
    fn the_search_in_hand_keeps_the_moment_of_its_search() {
        let folder = AFolder::made("searched-at");
        folder.holds("scene.blend", b"blend");
        folder.holds("shelf/deep/stone.png", b"stone");
        let sitting = a_sitting_of(&folder, "scene.blend");
        let mut machine = AScriptedMachine::default();
        machine.documents.insert(
            sitting.root.as_bytes().to_vec(),
            Some(a_document(vec![a_reference(Kind::Image, b"//maps/stone.png")])),
        );
        let record = a_record(&folder);
        the_searched_folder_list::write(
            record.connection(),
            &SearchedFolderList {
                folders: vec![folder.bytes("gone"), folder.bytes("shelf")],
            },
        )
        .expect("the list is written");
        let held = Mutex::new(WhatTheRunHolds::default());
        let before = SystemTime::now();
        let _ = run(the_check_runs(
            &mut machine,
            &OverOneConnection(record.connection()),
            &TheUsersOwnComputer,
            &held,
            &sitting,
            &|| {},
        ));
        let after = SystemTime::now();
        let held = held.lock().expect("the run");
        let search = held.sittings[&keyed(&sitting.server, &sitting.file)]
            .the_search
            .as_ref()
            .expect("a search ran");
        assert_eq!(search.folders[0], (folder.bytes("gone"), Some(FolderVerdict::CouldNotBeRead)));
        let moment = search.searched_at.expect("the search's moment is kept");
        assert!(before <= moment && moment <= after, "the moment is not this search's");
    }

    #[allow(dead_code)]
    fn unused(_: &Path, _: StoredSource) {}
}
