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
//!    until it ends (`Dependency Upload & Remap` G1, piece 2's route). **A file
//!    unchanged since this run read it, with every library it names, is not read
//!    again** (`Dependency Upload & Remap` G17b, `crate::kept_readings`), and
//!    everything after this step is the same for it.
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
    group_into_rows, how_the_render_reads_it, judge, linked_item_statements,
    the_bake_output_path_makes_no_row, tidied, HowTheRenderReadsIt, ReadReference,
    TheBakeFolderNotWorkedOut,
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
use crate::kept_readings::{WhatItStoodOn, WhichReading, THE_LEAST_A_KEPT_READING_SHOWS};
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

/// **What `Dependency Upload & Remap` G9a reads a bake folder against, for one
/// whole sitting**: the Blender the job renders with, and the name of the file
/// it opens, which is the root's even for a reference inside a linked file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheRenderOpens {
    pub the_version: (u32, u32),
    pub the_blend_name: Option<Vec<u8>>,
}

impl WhatTheRenderOpens {
    pub fn of(root: &HoldingFile, with: &BlenderIdentity) -> WhatTheRenderOpens {
        WhatTheRenderOpens {
            the_version: (with.version.major, with.version.minor),
            the_blend_name: root.the_blend_name(),
        }
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
    /// **`Dependency Upload & Remap` G9a's statements.**
    pub the_bake_folders_not_worked_out: Vec<TheBakeFolderNotWorkedOut>,
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

/// **`Dependency Upload & Remap` F28's line of 2026-11-02: a folder-shaped row's
/// files and bytes**, by the send's own walk, so the figures and what goes cannot
/// differ. **Nothing where the answer is no folder here, or will not read**, and
/// the row then says what a file's row says. A figure to read and nothing more.
fn the_folders_figures(resolution: Option<&Resolution>) -> Option<(usize, u64)> {
    let ResolvedTo::AWholeLocation(at) = &resolution?.to else {
        return None;
    };
    crate::the_found_files_sent::what_a_folder_row_sends(at)
}

/// **A row's two figures, from its answers** — `Dependency Upload & Remap` G23
/// for a set, `Dependency Upload & Remap` F28's line of 2026-11-02 for a folder,
/// a file's own length otherwise. **Taken when a check lands and at every later write of its
/// answers** ([`the_rows_answers_are_written`]), so no route leaves them behind.
fn the_rows_figures(
    answers: &[LocatedAnswer],
    wanted: &[Outstanding],
    readings: &dyn Readings,
) -> (Option<usize>, Option<u64>) {
    let resolution = answers.first().and_then(|answer| answer.resolution.as_ref());
    let members = match (resolution, wanted.first().and_then(Outstanding::the_sets_pattern)) {
        (Some(resolution), Some(pattern)) => the_sets_members_in(resolution.to.location(), pattern, readings),
        _ => None,
    };
    let a_folder = the_folders_figures(resolution);
    (
        members.as_ref().map(Vec::len).or(a_folder.map(|(files, _)| files)),
        a_folder.map(|(_, bytes)| bytes).or_else(|| the_size_of(resolution, members.as_deref())),
    )
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
    what_the_render_opens: &WhatTheRenderOpens,
    known: &[LocatedAnswer],
    list: &SearchedFolderList,
    readings: &dyn Readings,
    stop: &mut dyn FnMut() -> bool,
    now: SystemTime,
) -> OneReadingLanded {
    let source_folder = the_source_folder(holding);
    // **`Dependency Upload & Remap` G9a: a bake folder this Blender cannot be
    // said to read anywhere is a statement, with no row and no answer.**
    let mut the_bake_folders_not_worked_out: Vec<TheBakeFolderNotWorkedOut> = Vec::new();
    let mut references: Vec<(&ReadReference, Option<RecordedLocation>)> = Vec::new();
    for reference in &document.references {
        match how_the_render_reads_it(
            reference.kind,
            &reference.recorded,
            what_the_render_opens.the_version,
            what_the_render_opens.the_blend_name.as_deref(),
        ) {
            HowTheRenderReadsIt::AsRecorded => references.push((reference, None)),
            HowTheRenderReadsIt::WithItsTemplatePutIn(read) => references.push((reference, Some(read))),
            HowTheRenderReadsIt::NotWorkedOut { the_expression } => {
                if !the_bake_folders_not_worked_out.iter().any(|one| one.recorded == reference.recorded) {
                    the_bake_folders_not_worked_out.push(TheBakeFolderNotWorkedOut {
                        recorded: reference.recorded.clone(),
                        the_expression,
                    });
                }
            }
        }
    }
    let mut answers: Vec<LocatedAnswer> = Vec::new();
    let mut wanted: Vec<Outstanding> = Vec::new();
    let mut proposals: BTreeMap<Vec<u8>, StoredLocation> = BTreeMap::new();
    let mut outstanding: Vec<usize> = Vec::new();

    for (reference, read) in &references {
        // **`Dependency Upload & Remap` G6d: a library's first look goes where
        // the file writes it**, and K4's key below stays what Blender read.
        let the_location_read = read
            .as_ref()
            .or(reference.the_files_own_text.as_ref())
            .unwrap_or(&reference.recorded);
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
        let one = Outstanding::read_as(
            reference.recorded.clone(),
            the_location_read.clone(),
            reference.kind,
            reference.stored_source(),
        );
        if answer.resolution.is_none() && !answer.set_aside {
            // **`Dependency Upload & Remap` F27: the recorded location itself,
            // before any search.**
            match the_recorded_locations_own_answer(
                the_location_read,
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
    let to_group: Vec<ReferenceToGroup> = references
        .iter()
        .zip(&answers)
        .map(|((reference, _), answer)| ReferenceToGroup {
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
                references
                    .iter()
                    .position(|(one, _)| &one.recorded == recorded)
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
            match &references[*at].0.scenes {
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
        let (members, size) = the_rows_figures(&its_answers, &its_wanted, readings);
        let proposal = row
            .references
            .iter()
            .find_map(|recorded| proposals.get(recorded.as_bytes()).cloned());
        rows.push(ACheckedRow {
            kind: references[first].0.kind,
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
            members,
            size,
            why_it_did_not_go_up: None,
            its_failure_was_cleared: false,
            the_part_already_up: None,
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
        the_bake_folders_not_worked_out,
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

/// **`Dependency Upload & Remap` G17a: how many folders have been added to this
/// check.**
fn the_adds_taken(held: &Mutex<WhatTheRunHolds>, key: &str) -> u32 {
    held.lock()
        .map(|held| held.the_adds_taken.get(key).copied().unwrap_or(0))
        .unwrap_or(0)
}

/// **`Dependency Upload & Remap` G14a: a send of this check ended, with files
/// arrived.** Where a sitting is running, it goes round once more when its walk
/// ends. Taken in one hold with the walk's own ending, so it is never lost between
/// the two.
pub fn a_send_ended(held: &Mutex<WhatTheRunHolds>, key: &str) {
    if let Ok(mut held) = held.lock() {
        if held.checks_running.contains_key(key) {
            held.a_round_is_owed.insert(key.to_string());
        }
    }
}

/// **How often the walk over a reading's files tells the panel its count**, so a
/// walk of hundreds of files in a moment is not hundreds of redraws. The count
/// itself is written for every file.
const HOW_OFTEN_THE_WALK_TELLS_THE_PANEL: std::time::Duration = std::time::Duration::from_millis(200);

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

/// **Each location's four values, read at most once a sitting**, none where they
/// do not answer. On that channel a file not there reads the same as a channel
/// that cannot answer (`Render Settings` Q14 item 2), and a location that is not
/// text cannot be carried. **Read before what it is compared with, so a change
/// after this read is found at the next round** (`Dependency Upload & Remap` G17b).
pub(crate) async fn the_four_values_once<M>(
    machine: &mut M,
    read: &mut BTreeMap<Vec<u8>, Option<SourceReading>>,
    at: &[u8],
) -> Option<SourceReading>
where
    M: TheMachineAFileIsReadOn + Send,
{
    if let Some(known) = read.get(at) {
        return known.clone();
    }
    let now = match std::str::from_utf8(at) {
        Ok(text) => match machine.the_four_values_of(text).await {
            FourValuesNow::Answered(values) => Some(values),
            FourValuesNow::TheChannelCannotAnswer => None,
        },
        Err(_) => None,
    };
    read.insert(at.to_vec(), now.clone());
    now
}

/// **Every library a reading names, at the location Blender looked in** —
/// `Dependency Upload & Remap` G17b. It is the whole of what the reading rests on
/// apart from the file: every library Blender reached, at any depth, is named, an
/// absent one included. **Nothing where one cannot be carried to the four-values
/// read**, and that reading is not kept.
fn the_libraries_it_names(document: &ReadingDocument, file: &MachineLocation) -> Option<Vec<Vec<u8>>> {
    let mut named: Vec<Vec<u8>> = Vec::new();
    for reference in document.references.iter().filter(|one| one.kind == Kind::Library) {
        let Some(at) = where_the_render_finds_it(file, &reference.recorded) else {
            continue;
        };
        std::str::from_utf8(at.as_bytes()).ok()?;
        if !named.iter().any(|one| one.as_slice() == at.as_bytes()) {
            named.push(at.as_bytes().to_vec());
        }
    }
    Some(named)
}

/// **`Dependency Upload & Remap` G17b: the reading this run keeps of this file, if
/// nothing it stood on has changed.** One that no longer stands is let go.
async fn the_reading_kept<M>(
    machine: &mut M,
    held: &Mutex<WhatTheRunHolds>,
    which: &WhichReading,
    file: &MachineLocation,
    read: &mut BTreeMap<Vec<u8>, Option<SourceReading>>,
) -> Option<ReadingDocument>
where
    M: TheMachineAFileIsReadOn + Send,
{
    let the_file_now = the_four_values_once(machine, read, file.as_bytes()).await;
    let (document, stood_on) = held.lock().ok()?.the_kept_readings.the_one_for(which)?;
    let mut it_stands = the_file_now.as_ref() == Some(&stood_on.the_file);
    for (at, was) in &stood_on.the_libraries {
        if !it_stands {
            break;
        }
        it_stands = &the_four_values_once(machine, read, at).await == was;
    }
    if it_stands {
        return Some(document);
    }
    held.lock().ok()?.the_kept_readings.let_go(which);
    None
}

/// **`Dependency Upload & Remap` G17b: a reading that ended in a document is kept**,
/// with what it stood on, unless a file was going up to this machine while it was
/// taken, or the file's four values did not answer.
async fn the_reading_is_kept<M>(
    machine: &mut M,
    held: &Mutex<WhatTheRunHolds>,
    of: (&MachineKey, &WhichReading, &MachineLocation),
    document: &ReadingDocument,
    the_files_going_up_before: (u64, u64),
    read: &mut BTreeMap<Vec<u8>, Option<SourceReading>>,
) where
    M: TheMachineAFileIsReadOn + Send,
{
    let (on, which, file) = of;
    if the_files_going_up_before.0 != the_files_going_up_before.1 {
        return;
    }
    let Some(the_file) = the_four_values_once(machine, read, file.as_bytes()).await else {
        return;
    };
    let Some(named) = the_libraries_it_names(document, file) else {
        return;
    };
    let mut the_libraries = Vec::with_capacity(named.len());
    for at in named {
        let values = the_four_values_once(machine, read, &at).await;
        the_libraries.push((at, values));
    }
    if let Ok(mut held) = held.lock() {
        if held.the_files_going_up.on(on) == the_files_going_up_before {
            held.the_kept_readings.keep(which.clone(), document.clone(), WhatItStoodOn { the_file, the_libraries });
        }
    }
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
    loop {
        // **`Dependency Upload & Remap` A9e's (b): this sitting answers for the
        // libraries landed before it began**, and for none after.
        let the_libraries_it_answers = held
            .lock()
            .map(|held| held.the_libraries_landed.get(&key).map_or(0, |libraries| libraries.landed))
            .unwrap_or(0);
        let came_to = the_sitting_is_run(machine, record, readings, held, sitting, &key, told).await;
        // **Where the root gave nothing, the ending is the caller's to land**, in
        // `crate::surface::the_dependency_check::the_check_gave_nothing`, which is
        // the one place an ending is handed to the diary.
        // **And the mark comes off on every way out**, so no panel goes on saying
        // the file is being read once nothing reads it.
        // **`Dependency Upload & Remap` G14a: a send that ended during this walk
        // starts one more round from the top**, decided in the hold that ends it,
        // so a send ending at this moment either owes it here or starts its own.
        let another_round = match held.lock() {
            Ok(mut held) => {
                if let Some(libraries) = held.the_libraries_landed.get_mut(&key) {
                    libraries.answered = libraries.answered.max(the_libraries_it_answers);
                }
                let owed = held.a_round_is_owed.remove(&key);
                let another_round = owed
                    && came_to == TheCheckCameTo::TheSittingStands(Completion::Completed)
                    && held.checks_running.get(&key) == Some(&false);
                if another_round {
                    held.files_being_read.insert(key.clone());
                } else {
                    held.checks_running.remove(&key);
                    held.files_being_read.remove(&key);
                }
                another_round
            }
            Err(_) => false,
        };
        if !another_round {
            return came_to;
        }
        told();
    }
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
    let the_press = std::time::Instant::now();
    // **1. The root's four values, as the reading takes it** (`Dependency
    // Upload & Remap` V10).
    let root_four_values: SourceReading = match machine.the_four_values_of(&sitting.file).await {
        FourValuesNow::Answered(values) => values,
        FourValuesNow::TheChannelCannotAnswer => {
            return TheCheckCameTo::ItGaveNothing(Ending::TheMachineCouldNotBeReached)
        }
    };
    // **`Dependency Upload & Remap` G17b: each location's four values, at most once
    // a sitting.**
    let mut four_values_read: BTreeMap<Vec<u8>, Option<SourceReading>> = BTreeMap::new();
    // **`Dependency Upload & Remap` Q5a: each local file's digest, held beside it
    // for this round.**
    let mut local_digests = crate::the_found_files_sent::TheLocalDigests::default();
    if sitting.root.as_bytes() == sitting.file.as_bytes() {
        four_values_read.insert(sitting.root.as_bytes().to_vec(), Some(root_four_values.clone()));
    }

    // **`Dependency Upload & Remap` G17a: which add the list below is of**, taken
    // before the list is read, so an add between the two is read again.
    let mut the_list_is_of_add = the_adds_taken(held, key);
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

    // **A fresh sitting on the panel**: what an earlier one found is rebuilt,
    // never carried (`Dependency Upload & Remap` G17; `Dependency Upload & Remap` G17b keeps
    // the reading and no row), and an earlier ending goes with it.
    if let Ok(mut held) = held.lock() {
        held.endings.remove(key);
        held.selections.remove(key);
        held.sittings.insert(
            key.to_string(),
            ASitting {
                read_with: sitting.with.clone(),
                linked_files_read: 0,
                still_reading_below: None,
                the_walk: None,
                completion: None,
                linked_items: Vec::new(),
                the_bake_folders_not_worked_out: Vec::new(),
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

    let what_the_render_opens = WhatTheRenderOpens::of(&sitting.holding, &sitting.with);

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
        let (holding, _recorded_as) = holdings
            .get(file.as_bytes())
            .cloned()
            .unwrap_or((sitting.holding.clone(), None));
        told();

        // **`Dependency Upload & Remap` G17b: a file unchanged since this run read
        // it is not read again**, and everything below is as it would be for a
        // fresh reading.
        let which = WhichReading::of(&sitting.machine, &file, &sitting.with);
        let the_files_going_up_before = held
            .lock()
            .map(|held| held.the_files_going_up.on(&sitting.machine))
            .unwrap_or((0, 1));
        let document = if let Some(kept) = the_reading_kept(machine, held, &which, &file, &mut four_values_read).await {
            // **`Design/Dependency Upload & Remap`: *Reading the file…* stands long
            // enough to be read.**
            if is_the_root {
                tokio::time::sleep(THE_LEAST_A_KEPT_READING_SHOWS.saturating_sub(the_press.elapsed())).await;
            }
            kept
        } else {
            let Some(reading) = OneReading::of(&file, &sitting.with) else {
                completion = Completion::TheConnectionDropped;
                break;
            };
            // **`Dependency Upload & Remap` G15: a stop asked while the reading is
            // on the machine is sent to it**, naming its operation, and the
            // reading ends here at once (the 38's piece 7).
            let left = {
                // `Dependency Upload & Remap` G15: name the file actually opened,
                // and clear every exit before judging or searching its answer.
                let _named = (!is_the_root).then(|| ALinkedReadingIsNamed::begins(held, key, &file, told));
                drive_one_reading_until_stopped(machine, &reading, &mut || a_stop_was_asked(held, key)).await
            };
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
            the_reading_is_kept(
                machine,
                held,
                (&sitting.machine, &which, &file),
                &document,
                the_files_going_up_before,
                &mut four_values_read,
            )
            .await;
            document
        };

        // **`Dependency Upload & Remap` G17a: these rows land searched through the
        // list as it stands when they land.** A folder added while they were
        // being looked for has them looked for again, before they land, in the
        // one hold of the run that lands them.
        let mut below_it: Vec<(MachineLocation, HoldingFile, RecordedLocation)>;
        let stopped_here;
        loop {
            let the_add_now = the_adds_taken(held, key);
            if the_add_now != the_list_is_of_add {
                let mut read_again = None;
                record.with_the_record(&mut |connection| {
                    read_again = the_searched_folder_list::read(connection).ok();
                });
                let Some(read_again) = read_again else {
                    return TheCheckCameTo::TheRecordWouldNotRead;
                };
                list = read_again;
                the_list_is_of_add = the_add_now;
            }

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
                &what_the_render_opens,
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
            // **`Dependency Upload & Remap` Q5a: the walk says how far it has got and *Stop* ends it**, and the
            // panel is told at most once in [`HOW_OFTEN_THE_WALK_TELLS_THE_PANEL`],
            // since a release copy hashes hundreds of small files in a moment.
            let mut last_told: Option<std::time::Instant> = None;
            let mut counted = |done: usize, whole: usize| {
                if let Ok(mut held) = held.lock() {
                    if let Some(standing) = held.sittings.get_mut(key) {
                        standing.the_walk = Some((done, whole));
                    }
                }
                if last_told.is_none_or(|told_at| told_at.elapsed() >= HOW_OFTEN_THE_WALK_TELLS_THE_PANEL) {
                    last_told = Some(std::time::Instant::now());
                    told();
                }
            };
            let mut stopped = || a_stop_was_asked(held, key);
            let the_walk_ended = crate::the_found_files_sent::what_is_already_on_the_server(
                machine,
                record,
                sitting,
                &mut landed.rows,
                &mut four_values_read,
                &mut crate::the_found_files_sent::TheWalkIsWatched {
                    stopped: &mut stopped,
                    counted: &mut counted,
                    digests: &mut local_digests,
                },
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
            below_it = Vec::new();
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
                        let its_holding = the_librarys_holding(row, &sitting.machine, &below);
                        below_it.push((below, its_holding, recorded.clone()));
                    }
                }
            }

            let the_stop_was_asked_in_the_search = landed.stopped
                || the_walk_ended == crate::the_found_files_sent::WhereTheWalkEnded::AStopWasAsked;
            let mut they_landed = true;
            if let Ok(mut held) = held.lock() {
                if held.the_adds_taken.get(key).copied().unwrap_or(0) != the_list_is_of_add {
                    they_landed = false;
                } else {
                    if let Some(standing) = held.sittings.get_mut(key) {
                        // **The rows that arrive go into their groups without moving
                        // what is above them** — `Design/Dependency Upload & Remap §
                        // While it is still reading below a linked file`.
                        standing.rows.extend(landed.rows);
                        standing.linked_items.extend(landed.linked_items);
                        for one in landed.the_bake_folders_not_worked_out {
                            if !standing.the_bake_folders_not_worked_out.iter().any(|said| said.recorded == one.recorded) {
                                standing.the_bake_folders_not_worked_out.push(one);
                            }
                        }
                        if landed.the_search.is_some() {
                            standing.the_search = landed.the_search;
                        }
                        if !is_the_root {
                            standing.linked_files_read += 1;
                        }
                        standing.still_reading_below = None;
                        standing.the_walk = None;
                    }
                    // **The file itself is read, so its rows are drawn from here** —
                    // in the same hold of the run as the rows that land.
                    if is_the_root {
                        held.files_being_read.remove(key);
                    }
                }
            }
            if they_landed {
                stopped_here = the_stop_was_asked_in_the_search;
                break;
            }
        }
        for (below, its_holding, recorded) in below_it {
            holdings
                .entry(below.as_bytes().to_vec())
                .or_insert_with(|| (its_holding, Some(recorded)));
            the_loop.and_below_it([below]);
        }
        told();
        if stopped_here {
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
    let (rows_at_completion, counts): (Vec<RowAtCompletion>, _) = match held.lock() {
        Ok(mut held) => match held.sittings.get_mut(key) {
            Some(standing) => {
                standing.completion = Some(completion);
                standing.still_reading_below = None;
                standing.the_walk = None;
                let counts = crate::surface::the_dependency_check::the_warning_counts(&standing.rows, Default::default());
                let rows = standing
                    .rows
                    .iter()
                    .map(|row| RowAtCompletion {
                        state: row.state,
                        inside_a_linked_blend: (row.holding != sitting.holding)
                            .then(|| the_fixing_of(&row.holding)),
                    })
                    .collect();
                (rows, counts)
            }
            None => (Vec::new(), Default::default()),
        },
        Err(_) => (Vec::new(), Default::default()),
    };
    let mut standing = CheckStanding::at_the_sittings_end(
        sitting.root.clone(),
        sitting.machine.clone(),
        sitting.with.clone(),
        SystemTime::now(),
        completion,
        &rows_at_completion,
        root_four_values,
    );
    standing.counts = smoozy_dependencies::standing::Counts {
        confirmed: standing.counts.confirmed,
        set_aside: standing.counts.set_aside,
        ..counts
    };
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
    readings: &dyn Readings,
) -> bool {
    if !written(record, |keeper| {
        keeper.the_answers_are_written(&answers).is_ok()
    }) {
        return false;
    }
    row.state = the_state_of(&answers);
    (row.members, row.size) = the_rows_figures(&answers, &row.wanted, readings);
    row.the_part_already_up = None;
    row.answers = answers;
    row.proposal = proposal;
    true
}

// ── `Dependency Upload & Remap` F1b: adding a search folder ─────────────────────

/// **One row an act looks for with no lock held**, as it stood when it was taken.
struct ARowTaken {
    at: usize,
    holding: HoldingFile,
    reference: RecordedLocation,
    the_real_file: Vec<u8>,
    answers: Vec<LocatedAnswer>,
    wanted: Vec<Outstanding>,
}

impl ARowTaken {
    fn of(at: usize, one: &ACheckedRow) -> ARowTaken {
        ARowTaken {
            at,
            holding: one.holding.clone(),
            reference: one.row.ordering_key().clone(),
            the_real_file: one.the_real_file(),
            answers: one.answers.clone(),
            wanted: one.wanted.clone(),
        }
    }
}

/// **What the add took, under the locks, a hold each.**
pub struct TheAddInHand {
    folder: Vec<u8>,
    list: SearchedFolderList,
    rows: Vec<ARowTaken>,
}

/// **What one row's search came to**, with its figures, all taken with no lock
/// held.
struct WhatARowsSearchFound {
    at: usize,
    holding: HoldingFile,
    reference: RecordedLocation,
    the_real_file: Vec<u8>,
    answers: Option<Vec<LocatedAnswer>>,
    proposal: Option<StoredLocation>,
    figures: (Option<usize>, Option<u64>),
}

/// **What the search came to.**
pub struct WhatTheAddFound {
    rows: Vec<WhatARowsSearchFound>,
    the_matches_came_from: Option<Vec<u8>>,
    verdicts: Vec<(Vec<u8>, FolderVerdict)>,
    searched_at: Option<SystemTime>,
}

/// **The chosen folder, from the run's memory.**
fn the_chosen_folder_is_taken(held: &mut WhatTheRunHolds, key: &str) -> Option<Vec<u8>> {
    held.sittings.get_mut(key)?.the_chosen_search_folder.take()
}

/// **Added to Settings' own list, in that list's order** — one door and one
/// writer (`Dependency Upload & Remap` F1).
fn the_folder_goes_on_the_list(folder: &[u8], record: &dyn TheCheckReachesTheRecord) -> Option<SearchedFolderList> {
    let mut list = None;
    let there = record.with_the_record(&mut |connection| {
        let Ok(read) = the_searched_folder_list::read(connection) else {
            return;
        };
        let with = crate::surface::settings::the_list_with(&read, folder.to_vec()).unwrap_or(read);
        if the_searched_folder_list::write(connection, &with).is_ok() {
            list = Some(with);
        }
    });
    list.filter(|_| there)
}

/// **The rows still missing, and nothing else** — rows already answered are not
/// re-decided — and **from this hold every reading landing after it looks through
/// the added folder** (`Dependency Upload & Remap` G17a).
fn the_rows_still_missing_are_taken(
    held: &mut WhatTheRunHolds,
    key: &str,
    folder: Vec<u8>,
    list: SearchedFolderList,
) -> Option<TheAddInHand> {
    *held.the_adds_taken.entry(key.to_string()).or_insert(0) += 1;
    let sitting = held.sittings.get_mut(key)?;
    let rows = sitting
        .rows
        .iter()
        .enumerate()
        .filter(|(_, one)| one.state == RowState::NotFound)
        .map(|(at, one)| ARowTaken::of(at, one))
        .collect();
    sitting.the_list_it_searched = list.folders.clone();
    // **The panel reads as it does for any search.**
    sitting.the_search = Some(TheSearchInHand {
        folders: list.folders.iter().map(|one| (one.clone(), None)).collect(),
        the_matches_came_from: None,
        running: true,
        searched_at: None,
    });
    Some(TheAddInHand { folder, list, rows })
}

/// **The long act, with no lock held**: the search, and each row's figures.
fn the_add_searches(in_hand: &TheAddInHand, readings: &dyn Readings, stop: &mut dyn FnMut() -> bool) -> WhatTheAddFound {
    let wanted: Vec<Outstanding> = in_hand.rows.iter().flat_map(|one| one.wanted.iter().cloned()).collect();
    let outcome = search(&in_hand.list, &wanted, readings, stop, SystemTime::now());
    let mut the_matches_came_from = None;
    let rows = in_hand
        .rows
        .iter()
        .map(|one| {
            let mut resolved = None;
            let mut proposal = None;
            for wanted in &one.wanted {
                let candidates = outcome.candidates.get(&wanted.reference).cloned().unwrap_or_default();
                if let Some(found) = the_answer_for(&in_hand.list, &candidates) {
                    if let Ok(resolution) = resolution_from_the_search(&found, readings) {
                        the_matches_came_from.get_or_insert(found.folder.clone());
                        resolved = Some(resolution);
                        break;
                    }
                }
                let rejected = one.answers.iter().find_map(|answer| answer.rejected_guess.clone());
                proposal = proposal.or_else(|| {
                    the_proposals_for(&candidates, rejected.as_ref())
                        .into_iter()
                        .next()
                        .map(|path| StoredLocation::as_the_system_returned_it(&path))
                });
            }
            let answers: Option<Vec<LocatedAnswer>> = resolved.map(|resolution| {
                one.answers
                    .iter()
                    .cloned()
                    .map(|mut answer| {
                        answer.resolution = Some(resolution.clone());
                        answer
                    })
                    .collect()
            });
            let figures = answers
                .as_ref()
                .map(|answers| the_rows_figures(answers, &one.wanted, readings))
                .unwrap_or((None, None));
            WhatARowsSearchFound {
                at: one.at,
                holding: one.holding.clone(),
                reference: one.reference.clone(),
                the_real_file: one.the_real_file.clone(),
                answers,
                proposal,
                figures,
            }
        })
        .collect();
    WhatTheAddFound {
        rows,
        the_matches_came_from,
        verdicts: outcome.report.per_folder.iter().map(|one| (one.folder.clone(), one.verdict)).collect(),
        searched_at: outcome.report.per_folder.first().map(|one| one.at.taken_at()),
    }
}

/// **Every answer found, in one save** (`Dependency Upload & Remap` K4).
fn the_adds_answers_are_written(found: &WhatTheAddFound, record: &dyn TheCheckReachesTheRecord) -> bool {
    let answers: Vec<LocatedAnswer> = found
        .rows
        .iter()
        .filter_map(|one| one.answers.clone())
        .flatten()
        .collect();
    answers.is_empty() || written(record, |keeper| keeper.the_answers_are_written(&answers).is_ok())
}

/// **What was found lands on the rows still missing**, and the panel says once
/// what the add did (`Design/Dependency Upload & Remap § The search folders on
/// this panel`), **each real file once** (`Dependency Upload & Remap` G11b). **A row that is no longer the one
/// taken is left alone**: a fresh check reads what was just written.
fn the_add_lands(held: &mut WhatTheRunHolds, key: &str, folder: Vec<u8>, list: &SearchedFolderList, found: WhatTheAddFound) {
    let Some(sitting) = held.sittings.get_mut(key) else {
        return;
    };
    let mut asked_for = std::collections::HashSet::new();
    let mut came_up = std::collections::HashSet::new();
    for one in found.rows {
        asked_for.insert(one.the_real_file.clone());
        let Some(row) = sitting.rows.get_mut(one.at).filter(|row| {
            row.state == RowState::NotFound && row.holding == one.holding && row.row.ordering_key() == &one.reference
        }) else {
            continue;
        };
        match one.answers {
            Some(answers) => {
                came_up.insert(one.the_real_file);
                row.state = the_state_of(&answers);
                (row.members, row.size) = one.figures;
                row.answers = answers;
                row.proposal = None;
                // **`Design/Dependency Upload & Remap`: adding a search folder is
                // a search, so the row it found moves at once**, as *Check again*
                // would move it.
                row.state_when_the_check_ran = row.state;
            }
            None => row.proposal = one.proposal.or(row.proposal.take()),
        }
    }
    sitting.the_folder_added = Some(crate::surface::the_dependency_check::TheFolderAdded {
        folder,
        found: came_up.len(),
        still_not_found: asked_for.len() - came_up.len(),
    });
    sitting.the_list_it_searched = list.folders.clone();
    sitting.the_search = Some(TheSearchInHand {
        folders: list
            .folders
            .iter()
            .map(|folder| {
                let verdict = found.verdicts.iter().find(|(one, _)| one == folder).map(|(_, verdict)| *verdict);
                (folder.clone(), verdict)
            })
            .collect(),
        the_matches_came_from: found.the_matches_came_from,
        running: false,
        // `Dependency Upload & Remap` F1a: the moment of this search, as its own
        // report took it.
        searched_at: found.searched_at,
    });
}

/// **The add, where the caller already holds the run** — a test's route, which
/// nobody draws under.
fn the_chosen_folder_is_added_in_one_hold(
    held: &mut WhatTheRunHolds,
    key: &str,
    record: &dyn TheCheckReachesTheRecord,
    readings: &dyn Readings,
) -> WhatADecisionCameTo {
    let Some(folder) = the_chosen_folder_is_taken(held, key) else {
        return WhatADecisionCameTo::NothingToTake;
    };
    let Some(list) = the_folder_goes_on_the_list(&folder, record) else {
        return WhatADecisionCameTo::NotSaved;
    };
    let Some(in_hand) = the_rows_still_missing_are_taken(held, key, folder, list) else {
        return WhatADecisionCameTo::NothingToTake;
    };
    let found = the_add_searches(&in_hand, readings, &mut || false);
    if !the_adds_answers_are_written(&found, record) {
        if let Some(search) = held.sittings.get_mut(key).and_then(|sitting| sitting.the_search.as_mut()) {
            search.running = false;
        }
        return WhatADecisionCameTo::NotSaved;
    }
    the_add_lands(held, key, in_hand.folder, &in_hand.list, found);
    WhatADecisionCameTo::Taken
}

/// **`Dependency Upload & Remap` F1b: the add as the application runs it** — off
/// the drawing thread, the run held for its two short acts alone and the
/// application's lock for each write alone, so no draw waits on the search. The
/// panel's *Stop* ends the search.
pub fn the_chosen_folder_is_added(
    held: &Mutex<WhatTheRunHolds>,
    key: &str,
    record: &dyn TheCheckReachesTheRecord,
    readings: &dyn Readings,
    told: &dyn Fn(),
) -> WhatADecisionCameTo {
    let Ok(Some(folder)) = held.lock().map(|mut held| the_chosen_folder_is_taken(&mut held, key)) else {
        return WhatADecisionCameTo::NothingToTake;
    };
    let Some(list) = the_folder_goes_on_the_list(&folder, record) else {
        return WhatADecisionCameTo::NotSaved;
    };
    let Ok(Some(in_hand)) = held.lock().map(|mut held| the_rows_still_missing_are_taken(&mut held, key, folder, list)) else {
        return WhatADecisionCameTo::NothingToTake;
    };
    told();
    let mut stop = || {
        held.lock()
            .map(|held| {
                !held
                    .sittings
                    .get(key)
                    .and_then(|sitting| sitting.the_search.as_ref())
                    .is_some_and(|search| search.running)
            })
            .unwrap_or(true)
    };
    let found = the_add_searches(&in_hand, readings, &mut stop);
    if !the_adds_answers_are_written(&found, record) {
        if let Ok(mut held) = held.lock() {
            if let Some(search) = held.sittings.get_mut(key).and_then(|sitting| sitting.the_search.as_mut()) {
                search.running = false;
            }
        }
        return WhatADecisionCameTo::NotSaved;
    }
    if let Ok(mut held) = held.lock() {
        the_add_lands(&mut held, key, in_hand.folder, &in_hand.list, found);
    }
    WhatADecisionCameTo::Taken
}

// ── `Dependency Upload & Remap` F1c: *Locate…* and *Choose a folder…* ──────────

/// **The rows a decision reaches**: the row pressed, or the selection where the
/// act came from the bar (`Dependency Upload & Remap` F22), and **every row of each real file among them**,
/// since locating or ignoring one file is one decision (`Dependency Upload &
/// Remap` G11b).
fn the_rows_reached(held: &WhatTheRunHolds, key: &str, row: Option<usize>) -> Vec<usize> {
    let reached: Vec<usize> = match row {
        Some(at) => vec![at],
        None => held
            .selections
            .get(key)
            .map(|chosen| chosen.iter().copied().collect())
            .unwrap_or_default(),
    };
    let Some(sitting) = held.sittings.get(key) else {
        return reached;
    };
    let the_files = crate::surface::the_dependency_check::the_real_files(&sitting.rows);
    the_files
        .iter()
        .filter(|file| file.iter().any(|at| reached.contains(at)))
        .flatten()
        .copied()
        .chain(reached.iter().copied().filter(|at| !the_files.iter().flatten().any(|one| one == at)))
        .collect()
}

/// **One row's answer, written and ready to land.** Nothing in `answers` where the
/// look only had a proposal to offer.
struct ARowsAnswer {
    at: usize,
    holding: HoldingFile,
    reference: RecordedLocation,
    /// **`Dependency Upload & Remap` F18 and `Dependency Upload & Remap` F26 reach
    /// only a row still missing**, and it is asked again as it
    /// lands; the row the user located is theirs whatever it read.
    only_while_missing: bool,
    answers: Option<Vec<LocatedAnswer>>,
    proposal: Option<StoredLocation>,
    figures: (Option<usize>, Option<u64>),
}

/// **`Dependency Upload & Remap` F22: each row's write is its own, with the
/// application's lock for that save alone.**
fn a_rows_answer_is_written(one: &ARowTaken, answers: Vec<LocatedAnswer>, only_while_missing: bool, record: &dyn TheCheckReachesTheRecord, readings: &dyn Readings) -> Option<ARowsAnswer> {
    if !written(record, |keeper| keeper.the_answers_are_written(&answers).is_ok()) {
        return None;
    }
    Some(ARowsAnswer {
        at: one.at,
        holding: one.holding.clone(),
        reference: one.reference.clone(),
        only_while_missing,
        figures: the_rows_figures(&answers, &one.wanted, readings),
        answers: Some(answers),
        proposal: None,
    })
}

/// **What was found lands**, in one hold of the run. **A row that is no longer the
/// one taken is left alone**: a fresh check reads what was just written.
fn the_answers_land(sitting: &mut ASitting, rows: Vec<ARowsAnswer>) {
    for one in rows {
        let Some(row) = sitting.rows.get_mut(one.at).filter(|row| {
            row.holding == one.holding
                && row.row.ordering_key() == &one.reference
                && (!one.only_while_missing || row.state == RowState::NotFound)
        }) else {
            continue;
        };
        match one.answers {
            Some(answers) => {
                row.state = the_state_of(&answers);
                (row.members, row.size) = one.figures;
                row.answers = answers;
                row.proposal = one.proposal;
            }
            None => row.proposal = one.proposal,
        }
    }
}

/// **What *Locate…* took, in one hold of the run**: the row pressed, the other rows
/// of its real file, and every other row still missing, which a shift may reach.
pub struct TheLocateInHand {
    path: Vec<u8>,
    pressed: ARowTaken,
    of_the_same_file: Vec<ARowTaken>,
    still_missing: Vec<ARowTaken>,
}

fn the_locate_takes_its_rows(held: &WhatTheRunHolds, key: &str, row: Option<usize>, path: &[u8]) -> Option<TheLocateInHand> {
    let at = row?;
    let reached = the_rows_reached(held, key, row);
    let sitting = held.sittings.get(key)?;
    let taken = |at: usize| sitting.rows.get(at).map(|one| ARowTaken::of(at, one));
    Some(TheLocateInHand {
        path: path.to_vec(),
        pressed: taken(at)?,
        of_the_same_file: reached.iter().copied().filter(|other| *other != at).filter_map(taken).collect(),
        still_missing: sitting
            .rows
            .iter()
            .enumerate()
            .filter(|(other, one)| one.state == RowState::NotFound && !reached.contains(other))
            .map(|(other, one)| ARowTaken::of(other, one))
            .collect(),
    })
}

/// **What the look came to**: the answers written so far, the shift it taught, and
/// what the act says.
pub struct WhatTheLookFound {
    rows: Vec<ARowsAnswer>,
    the_shift: Option<TheShiftInHand>,
    came_to: WhatADecisionCameTo,
}

/// **The long act, with no lock held**: `Dependency Upload & Remap` F16's locate
/// and the shift it may teach, `Dependency Upload & Remap` F18 looking for every row still
/// missing.
fn the_locate_looks(in_hand: &TheLocateInHand, record: &dyn TheCheckReachesTheRecord, readings: &dyn Readings) -> WhatTheLookFound {
    let mut found = WhatTheLookFound { rows: Vec::new(), the_shift: None, came_to: WhatADecisionCameTo::Taken };
    // **`Dependency Upload & Remap` F12: the whole location as stored on disk**,
    // read from the system at the moment of the act.
    let Ok(stored) = the_stored_location_of(&in_hand.path, readings) else {
        found.came_to = WhatADecisionCameTo::NothingToTake;
        return found;
    };
    let pressed = &in_hand.pressed;
    let a_folder = pressed.wanted.first().map(|wanted| wanted.shape) == Some(WantedShape::ASetsFolder);
    let resolution = Resolution {
        to: if a_folder {
            ResolvedTo::AFolderForASetRow(stored.clone())
        } else {
            ResolvedTo::AWholeLocation(stored.clone())
        },
        how: HowResolved::TheUsersOwnHand,
    };
    // **`Dependency Upload & Remap` G11b: the other rows of this real file take the
    // same answer.**
    for one in std::iter::once(pressed).chain(&in_hand.of_the_same_file) {
        let answers = one
            .answers
            .iter()
            .cloned()
            .map(|mut answer| {
                answer.resolution = Some(resolution.clone());
                answer.set_aside = false;
                answer
            })
            .collect();
        let Some(written) = a_rows_answer_is_written(one, answers, false, record, readings) else {
            found.came_to = WhatADecisionCameTo::NotSaved;
            return found;
        };
        found.rows.push(written);
    }
    // `Dependency Upload & Remap` G9a: what the render reads, the template put in,
    // which is what the user's move is measured against.
    let old = pressed.wanted.first().and_then(|wanted| {
        the_full_old_location(&wanted.the_location_read, the_source_folder(&pressed.holding).as_deref())
    });
    // **`Dependency Upload & Remap` F16: one hand-locate may teach a shift**, and
    // `Dependency Upload & Remap` F18 resolves every other row under the old root by looking.
    let Some(Ok(shift)) = old.map(|old| derive(&old, stored.as_bytes())) else {
        return found;
    };
    let mut taught = None;
    // **`Dependency Upload & Remap` K6: the shift record first**, and the answers
    // it resolves after it (`Dependency Upload & Remap` F20 needs only that every answer names
    // it).
    let _ = written(record, |keeper| {
        match keeper.a_shift_is_taught(shift.old_root.clone(), shift.new_root.clone(), SystemTime::now()) {
            Ok(record) => {
                taught = Some(record.key);
                true
            }
            Err(_) => false,
        }
    });
    let Some(shift_key) = taught else {
        return found;
    };
    found.the_shift = Some(TheShiftInHand {
        old_root: shift.old_root.clone(),
        new_root: shift.new_root.clone(),
        key: shift_key,
    });
    for row in &in_hand.still_missing {
        let source = the_source_folder(&row.holding);
        let mut resolved = None;
        for wanted in &row.wanted {
            let Some(old) = the_full_old_location(&wanted.the_location_read, source.as_deref()) else {
                continue;
            };
            if let ShiftOutcome::Resolved(stored) = apply_to_one(&shift, &old, wanted.shape, readings) {
                resolved = Some((stored, wanted.shape));
                break;
            }
        }
        let Some((stored, shape)) = resolved else {
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
        let Some(written) = a_rows_answer_is_written(row, answers, true, record, readings) else {
            found.came_to = WhatADecisionCameTo::NotSaved;
            return found;
        };
        found.rows.push(written);
    }
    found
}

/// **The long act of *Choose a folder…*, with no lock held**: each row still missing
/// looked for in that folder by `Dependency Upload & Remap` F18's own rule, and no
/// shift (`Dependency Upload & Remap` F26).
fn the_choice_looks(rows: &[ARowTaken], folder: &[u8], record: &dyn TheCheckReachesTheRecord, readings: &dyn Readings) -> WhatTheLookFound {
    let mut found = WhatTheLookFound { rows: Vec::new(), the_shift: None, came_to: WhatADecisionCameTo::NothingToTake };
    for one in rows {
        let Some(wanted) = one.wanted.first() else {
            continue;
        };
        let name = wanted.wanted_name().map(<[u8]>::to_vec).unwrap_or_else(|| {
            wanted.the_sets_pattern().map(<[u8]>::to_vec).unwrap_or_default()
        });
        match look_for(&smoozy_dependencies::bytes::join(folder, &name), wanted.shape, readings) {
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
                let Some(written) = a_rows_answer_is_written(one, answers, true, record, readings) else {
                    found.came_to = WhatADecisionCameTo::NotSaved;
                    return found;
                };
                found.rows.push(written);
                found.came_to = WhatADecisionCameTo::Taken;
            }
            ShiftOutcome::AProposal(path) => {
                found.rows.push(ARowsAnswer {
                    at: one.at,
                    holding: one.holding.clone(),
                    reference: one.reference.clone(),
                    only_while_missing: true,
                    answers: None,
                    proposal: Some(StoredLocation::as_the_system_returned_it(&path)),
                    figures: (None, None),
                });
                found.came_to = WhatADecisionCameTo::Taken;
            }
            ShiftOutcome::Unchanged | ShiftOutcome::TheSystemCannotSay => {}
        }
    }
    found
}

/// **What *Choose a folder…* took, in one hold of the run**: the rows it reaches that
/// are still missing.
fn the_choice_takes_its_rows(held: &WhatTheRunHolds, key: &str, row: Option<usize>) -> Vec<ARowTaken> {
    let reached = the_rows_reached(held, key, row);
    let Some(sitting) = held.sittings.get(key) else {
        return Vec::new();
    };
    reached
        .into_iter()
        .filter_map(|at| sitting.rows.get(at).filter(|one| one.state == RowState::NotFound).map(|one| ARowTaken::of(at, one)))
        .collect()
}

/// **What the look found lands, and what the act says.**
fn the_look_lands(held: &mut WhatTheRunHolds, key: &str, found: WhatTheLookFound) -> WhatADecisionCameTo {
    if let Some(sitting) = held.sittings.get_mut(key) {
        the_answers_land(sitting, found.rows);
        if found.the_shift.is_some() {
            sitting.the_shift = found.the_shift;
        }
    }
    found.came_to
}

/// **`Dependency Upload & Remap` F1c: the two decisions that look at the disk, as the
/// application runs them** — off the drawing thread, the run held for its two short
/// acts alone and the application's lock for each write alone, so no draw waits on
/// the look.
pub fn a_decision_that_looks_is_taken(
    held: &Mutex<WhatTheRunHolds>,
    key: &str,
    decision: &ADecision,
    row: Option<usize>,
    record: &dyn TheCheckReachesTheRecord,
    readings: &dyn Readings,
) -> WhatADecisionCameTo {
    let found = match decision {
        ADecision::Locate(path) => {
            let Ok(in_hand) = held.lock().map(|held| the_locate_takes_its_rows(&held, key, row, path)) else {
                return WhatADecisionCameTo::NothingToTake;
            };
            let Some(in_hand) = in_hand else {
                return WhatADecisionCameTo::NothingToTake;
            };
            the_locate_looks(&in_hand, record, readings)
        }
        ADecision::ChooseAFolder(folder) => {
            let Ok(rows) = held.lock().map(|held| the_choice_takes_its_rows(&held, key, row)) else {
                return WhatADecisionCameTo::NothingToTake;
            };
            the_choice_looks(&rows, folder, record, readings)
        }
        _ => return WhatADecisionCameTo::NothingToTake,
    };
    match held.lock() {
        Ok(mut held) => the_look_lands(&mut held, key, found),
        Err(_) => WhatADecisionCameTo::NothingToTake,
    }
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
    if decision == ADecision::AddTheChosenSearchFolder {
        return the_chosen_folder_is_added_in_one_hold(held, key, record, readings);
    }
    // **`Dependency Upload & Remap` F1c's two, where the caller already holds the
    // run** — a test's route, which nobody draws under. The application runs them
    // through [`a_decision_that_looks_is_taken`].
    match &decision {
        ADecision::Locate(path) => {
            let Some(in_hand) = the_locate_takes_its_rows(held, key, row, path) else {
                return WhatADecisionCameTo::NothingToTake;
            };
            let found = the_locate_looks(&in_hand, record, readings);
            return the_look_lands(held, key, found);
        }
        ADecision::ChooseAFolder(folder) => {
            let rows = the_choice_takes_its_rows(held, key, row);
            let found = the_choice_looks(&rows, folder, record, readings);
            return the_look_lands(held, key, found);
        }
        _ => {}
    }
    let reached = the_rows_reached(held, key, row);
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
                if !the_rows_answers_are_written(one, answers, proposal, record, readings) {
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
                if !the_rows_answers_are_written(one, answers, None, record, readings) {
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
                if !the_rows_answers_are_written(one, answers, None, record, readings) {
                    return WhatADecisionCameTo::NotSaved;
                }
                took = true;
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
                if !the_rows_answers_are_written(one, answers, None, record, readings) {
                    sitting.the_shift = Some(shift);
                    return WhatADecisionCameTo::NotSaved;
                }
            }
            took = true;
        }
        // **Taken above**, before the sitting is held.
        ADecision::AddTheChosenSearchFolder | ADecision::Locate(_) | ADecision::ChooseAFolder(_) => {}
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

// `Dependency Upload & Remap` G15; Design/Dependency Upload & Remap,
// *While it is still reading below a linked file*.
struct ALinkedReadingIsNamed<'a> {
    held: &'a Mutex<WhatTheRunHolds>,
    key: &'a str,
    told: &'a (dyn Fn() + Send + Sync),
}

impl<'a> ALinkedReadingIsNamed<'a> {
    fn begins(
        held: &'a Mutex<WhatTheRunHolds>,
        key: &'a str,
        file: &MachineLocation,
        told: &'a (dyn Fn() + Send + Sync),
    ) -> Self {
        if let Ok(mut h) = held.lock() {
            if let Some(s) = h.sittings.get_mut(key) {
                s.still_reading_below =
                    Some(RecordedLocation::as_the_reading_carried_it(file.as_bytes()));
            }
        }
        told();
        Self { held, key, told }
    }
}

impl Drop for ALinkedReadingIsNamed<'_> {
    fn drop(&mut self) {
        if let Ok(mut h) = self.held.lock() {
            if let Some(s) = h.sittings.get_mut(self.key) {
                s.still_reading_below = None;
            }
        }
        (self.told)();
    }
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

    /// **`Dependency Upload & Remap` F28's line of 2026-11-02: a folder-shaped
    /// row counts and sizes what goes.** The first 5.3 live pass read *Upload 1
    /// file (0 bytes)* for a bake folder of two files and 2.5 KB. Subfolders
    /// count, a link does not, and Smoozy's own staging folder does not, as the
    /// send's own walk has it. **What it bites on**: the row's size asked only
    /// whether its answer was a file.
    #[test]
    fn a_folder_rows_figures_are_the_files_that_go() {
        let root = std::env::temp_dir().join(format!("smoozy-a-folder-row-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let bake = root.join("bakes").join("bake_blend_name");
        std::fs::create_dir_all(bake.join("meta")).expect("a folder");
        std::fs::create_dir_all(bake.join("blobs")).expect("a folder");
        std::fs::write(bake.join("meta").join("1.json"), vec![1u8; 500]).expect("a file");
        std::fs::write(bake.join("blobs").join("1_0.blob"), vec![2u8; 2_060]).expect("a file");
        let staging = bake.join(smoozy_file_browser::partial::THE_RESERVED_STAGING_FOLDER);
        std::fs::create_dir_all(&staging).expect("a folder");
        std::fs::write(staging.join("x.smoozy-part"), b"part").expect("a file");
        #[cfg(unix)]
        std::os::unix::fs::symlink(root.join("elsewhere"), bake.join("a-link")).expect("a link");
        let resolved = |to: ResolvedTo| Resolution { to, how: HowResolved::TheRecordedLocationItself };
        let at = |path: &Path| StoredLocation::as_the_system_returned_it(path.to_string_lossy().as_bytes());

        let row = resolved(ResolvedTo::AWholeLocation(at(&bake)));
        assert_eq!(the_folders_figures(Some(&row)), Some((2, 2_560)));

        // **A file's row is as it was**, one file and no count of members.
        let file = bake.join("meta").join("1.json");
        let row = resolved(ResolvedTo::AWholeLocation(at(&file)));
        assert_eq!(the_folders_figures(Some(&row)), None);
        assert_eq!(the_size_of(Some(&row), None), Some(500));

        // **An empty folder sends nothing, and says so.**
        let empty = root.join("empty");
        std::fs::create_dir_all(&empty).expect("a folder");
        let row = resolved(ResolvedTo::AWholeLocation(at(&empty)));
        assert_eq!(the_folders_figures(Some(&row)), Some((0, 0)));

        // **A set's row is its members', as it was.**
        let row = resolved(ResolvedTo::AFolderForASetRow(at(&bake.join("meta"))));
        assert_eq!(the_folders_figures(Some(&row)), None);
        assert_eq!(the_size_of(Some(&row), Some(&[b"1.json".to_vec()])), Some(500));
        assert_eq!(the_folders_figures(None), None);
        let _ = std::fs::remove_dir_all(&root);
    }

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
            the_files_own_text: None,
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

    /// **`Dependency Upload & Remap` G9a: on 5.3 a bake folder's `{blend_name}` is read as the render reads
    /// it**, the recorded bytes staying the key; another variable is a statement
    /// and neither a row nor an answer; and on 5.2 the braces are the folder's
    /// own name, whatever the name is. Each folder here is what the desk's own
    /// bakes made of these words, 2026-11-02.
    #[cfg(unix)]
    #[test]
    fn a_bake_folders_template_is_read_from_5_3_and_never_reads_as_missing() {
        let folder = AFolder::made("template");
        folder.holds("project/t.blend", b"blend");
        folder.holds("project/bakes/t/123/meta/00001_00000.json", b"{}");
        folder.holds("project/bakes/{blend_name}/123/meta/00001_00000.json", b"{}");

        let mut sitting = a_sitting_of(&folder, "project/t.blend");
        sitting.with.version = Version { major: 5, minor: 3, patch: 0 };
        let check = |sitting: &TheSittingIsOf, name: &str| {
            let mut machine = AScriptedMachine::default();
            machine.documents.insert(
                sitting.root.as_bytes().to_vec(),
                Some(a_document(vec![
                    a_reference(Kind::GeometryNodeBakeFolder, b"//bakes/{blend_name}/"),
                    a_reference(Kind::GeometryNodeBakeFolder, b"//bakes/{blend_name}_{scene_name}/"),
                ])),
            );
            let record = crate::record::LocalRecord::open_at(&folder.at.join(name)).expect("the record opens");
            let held = Mutex::new(WhatTheRunHolds::default());
            let came_to = run(the_check_runs(
                &mut machine,
                &OverOneConnection(record.connection()),
                &TheUsersOwnComputer,
                &held,
                sitting,
                &|| {},
            ));
            assert_eq!(came_to, TheCheckCameTo::TheSittingStands(Completion::Completed));
            let answers = TheDependencyRecord::over(record.connection())
                .the_answers_of(&sitting.holding)
                .expect("the answers read");
            (held, answers)
        };

        let (held, answers) = check(&sitting, "five-three.sqlite3");
        let the_folder_read = folder.bytes("project/bakes/t");
        assert_eq!(answers.len(), 1, "no answer for what is not worked out: {answers:#?}");
        assert_eq!(answers[0].key.reference.as_bytes(), b"//bakes/{blend_name}/");
        assert_eq!(
            answers[0].resolution.as_ref().map(|found| found.to.location().as_bytes().to_vec()),
            Some(the_folder_read),
            "the folder the render reads"
        );
        {
            let held = held.lock().expect("the run");
            let standing = &held.sittings[&keyed(&sitting.server, &sitting.file)];
            assert_eq!(standing.rows.len(), 1, "a statement and not a row");
            assert_eq!(standing.rows[0].state, RowState::FoundOnYourComputer);
            assert_eq!(
                standing.the_bake_folders_not_worked_out,
                vec![smoozy_dependencies::check::TheBakeFolderNotWorkedOut {
                    recorded: RecordedLocation::as_the_reading_carried_it(b"//bakes/{blend_name}_{scene_name}/"),
                    the_expression: b"{scene_name}".to_vec(),
                }]
            );
            let panel = crate::surface::the_dependency_check::the_panel(&held, &sitting.server, &sitting.file, None);
            let said: Vec<String> = panel
                .the_statements
                .iter()
                .filter(|one| one.which.as_str() == "a-bake-folder-not-worked-out")
                .map(|one| format!("{:?}", one.said))
                .collect();
            assert_eq!(said.len(), 1, "{:#?}", panel.the_statements);
            let table = panel.the_table.expect("rows");
            let row = table.groups.iter().flat_map(|group| group.rows.iter()).next().expect("the row");
            assert_eq!(row.name.as_str(), "t", "the folder's name, read");
            assert_eq!(row.the_file_says.as_str(), "//bakes/{blend_name}/", "and what the file says");
        }

        // **5.2.3 made a folder of these very bytes.**
        sitting.with = a_blender();
        let (held, answers) = check(&sitting, "five-two.sqlite3");
        let the_folder_named_so = folder.bytes("project/bakes/{blend_name}");
        assert_eq!(answers.len(), 2);
        let found = answers
            .iter()
            .find(|answer| answer.key.reference.as_bytes() == b"//bakes/{blend_name}/")
            .and_then(|answer| answer.resolution.as_ref())
            .map(|found| found.to.location().as_bytes().to_vec());
        assert_eq!(found, Some(the_folder_named_so));
        let held = held.lock().expect("the run");
        let standing = &held.sittings[&keyed(&sitting.server, &sitting.file)];
        assert!(standing.the_bake_folders_not_worked_out.is_empty());
        assert_eq!(standing.rows.len(), 2, "the other folder is simply missing, as it always was");
    }

    /// **`Dependency Upload & Remap` G9a, inside a linked file: `{blend_name}` is the file Blender opens**,
    /// the sitting's root, and `//` stays the library's (observed at the desk,
    /// `a_linked_files_blend_name_is_the_file_blender_opened`).
    #[cfg(unix)]
    #[test]
    fn a_linked_files_bake_folder_takes_the_roots_name() {
        let folder = AFolder::made("template-linked");
        folder.holds("project/T.blend", b"blend");
        folder.holds("project/assets/L.blend", b"blend");
        folder.holds("project/assets/bakes/T/123/meta/00001_00000.json", b"{}");
        folder.holds("project/assets/bakes/L/123/meta/00001_00000.json", b"{}");
        let mut sitting = a_sitting_of(&folder, "project/T.blend");
        sitting.with.version = Version { major: 5, minor: 3, patch: 0 };
        let mut machine = AScriptedMachine::default();
        machine.documents.insert(
            sitting.root.as_bytes().to_vec(),
            Some(a_document(vec![a_reference(Kind::Library, b"//assets/L.blend")])),
        );
        machine.documents.insert(
            folder.bytes("project/assets/L.blend"),
            Some(a_document(vec![a_reference(Kind::GeometryNodeBakeFolder, b"//bakes/{blend_name}/")])),
        );
        let record = a_record(&folder);
        let held = Mutex::new(WhatTheRunHolds::default());
        run(the_check_runs(
            &mut machine,
            &OverOneConnection(record.connection()),
            &TheUsersOwnComputer,
            &held,
            &sitting,
            &|| {},
        ));
        let held = held.lock().expect("the run");
        let rows = &held.sittings[&keyed(&sitting.server, &sitting.file)].rows;
        let bake = rows
            .iter()
            .find(|row| row.kind == Kind::GeometryNodeBakeFolder)
            .expect("the linked file was read");
        assert_eq!(
            bake.the_answer().and_then(|answer| answer.resolution.as_ref()).map(|found| found.to.location().as_bytes().to_vec()),
            Some(folder.bytes("project/assets/bakes/T")),
        );
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
        // then says how it ended. The file is read afresh, as one changed since
        // would be (`Dependency Upload & Remap` G17b).
        held.lock().expect("the run").the_kept_readings = Default::default();
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

    #[test]
    fn a_failed_linked_read_does_not_leave_its_reading_words_standing() {
        let folder = AFolder::made("reading-words");
        folder.holds("scene.blend", b"blend");
        let sitting = a_sitting_of(&folder, "scene.blend");
        let mut machine = AScriptedMachine::default();
        machine.documents.insert(
            sitting.root.as_bytes().to_vec(),
            Some(a_document(vec![
                a_reference(Kind::Library, b"//missing.blend"),
                a_reference(Kind::Library, b"//another.blend"),
            ])),
        );
        let record = a_record(&folder);
        let held = Arc::new(Mutex::new(WhatTheRunHolds::default()));
        let key = keyed(&sitting.server, &sitting.file);
        let active = Arc::new(Mutex::new(Vec::new()));
        let seen = active.clone();
        let h = held.clone();
        let k = key.clone();
        machine.after_each_reading = Some(Arc::new(move || {
            seen.lock()
                .unwrap()
                .push(h.lock().unwrap().sittings[&k].still_reading_below.clone());
        }));
        let count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let read_count = count.clone();
        let after = machine.after_each_reading.take().unwrap();
        machine.after_each_reading = Some(Arc::new(move || {
            after();
            read_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }));
        let states = Mutex::new(Vec::new());
        let result = run(the_check_runs(
            &mut machine,
            &OverOneConnection(record.connection()),
            &TheUsersOwnComputer,
            &held,
            &sitting,
            &|| {
                let h = held.lock().unwrap();
                if let Some(s) = h.sittings.get(&key) {
                    states.lock().unwrap().push((
                        count.load(std::sync::atomic::Ordering::SeqCst),
                        s.still_reading_below.clone(),
                    ));
                }
            },
        ));
        assert_eq!(
            result,
            TheCheckCameTo::TheSittingStands(Completion::Completed)
        );
        assert_eq!(machine.read.len(), 3);
        assert!(
            states
                .lock()
                .unwrap()
                .iter()
                .any(|(n, name)| *n == 2 && name.is_none()),
            "a failed library's reading ended but its words still stand until the next reading"
        );
        assert_eq!(
            active.lock().unwrap()[1].as_ref().unwrap().as_bytes(),
            machine.read[1]
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

    /// **`Dependency Upload & Remap` G6d, the desktop's half**: Blender read
    /// `//../../lib/x.blend` where the file writes `//../../../lib/x.blend`. The
    /// first look goes where the file says, the row says what the file says,
    /// and the key stays what Blender read, because the remap meets that text.
    #[cfg(unix)]
    #[test]
    fn a_librarys_location_is_looked_for_and_said_as_the_file_writes_it() {
        let folder = AFolder::made("library-text");
        folder.holds("a/b/c/scene.blend", b"blend");
        folder.holds("lib/x.blend", b"a library");
        let sitting = a_sitting_of(&folder, "a/b/c/scene.blend");
        let mut machine = AScriptedMachine::default();
        let mut found = a_reference(Kind::Library, b"//../../lib/x.blend");
        found.the_files_own_text = Some(RecordedLocation::as_the_reading_carried_it(b"//../../../lib/x.blend"));
        let mut not_found = a_reference(Kind::Library, b"//../../gone/y.blend");
        not_found.the_files_own_text = Some(RecordedLocation::as_the_reading_carried_it(b"//../../../gone/y.blend"));
        machine.documents.insert(sitting.root.as_bytes().to_vec(), Some(a_document(vec![found, not_found])));
        let record = a_record(&folder);
        let held = Mutex::new(WhatTheRunHolds::default());
        let door = OverOneConnection(record.connection());
        let _ = run(the_check_runs(&mut machine, &door, &TheUsersOwnComputer, &held, &sitting, &|| {}));
        let held = held.lock().expect("the run");
        let standing = &held.sittings[&keyed(&sitting.server, &sitting.file)];
        let row = |ends: &[u8]| {
            standing.rows.iter().find(|one| one.row.ordering_key().as_bytes().ends_with(ends)).expect("the row")
        };
        assert_eq!(row(b"x.blend").state, RowState::FoundOnYourComputer);
        assert_eq!(row(b"y.blend").state, RowState::NotFound);
        assert_eq!(row(b"x.blend").row.ordering_key().as_bytes(), b"//../../lib/x.blend", "the key is what Blender read");
        let panel = crate::surface::the_dependency_check::the_panel(&held, &sitting.server, &sitting.file, None);
        let table = panel.the_table.expect("rows");
        let says: Vec<String> = table
            .groups
            .iter()
            .flat_map(|group| group.rows.iter())
            .map(|row| row.the_file_says.as_str().to_string())
            .collect();
        assert!(says.contains(&"//../../../lib/x.blend".to_string()), "{says:?}");
        assert!(says.contains(&"//../../../gone/y.blend".to_string()), "{says:?}");
        let the_missing_row = table.groups.iter().flat_map(|group| group.rows.iter()).find(|row| row.name.as_str() == "y.blend").expect("the row");
        let location = serde_json::to_string(&the_missing_row.location).unwrap();
        assert!(location.contains("//../../../gone/"), "{location}");
    }

    /// **`Dependency Upload & Remap` F28's bullet of 2026-11-02 and
    /// `Design/Dependency Upload & Remap`'s exception: adding a search folder
    /// reads as *Check again* would**, the live pass on real production files:
    /// *1 file to upload, 0 bytes*, under *Missing 1*. A bake folder, a set and a
    /// picture, each found only by the folder added: what the add leaves is what
    /// a fresh check of the same record reads.
    #[cfg(unix)]
    #[test]
    fn the_first_answer_after_adding_a_search_folder_is_the_answer_check_again_gives() {
        let folder = AFolder::made("search-figures");
        folder.holds("scene.blend", b"blend");
        folder.holds("library/hero_bake/meta/1.json", &[1u8; 500]);
        folder.holds("library/hero_bake/blobs/1_0.blob", &[2u8; 2_060]);
        folder.holds("library/seq/f_0001.png", &[3u8; 10]);
        folder.holds("library/seq/f_0002.png", &[3u8; 20]);
        folder.holds("library/stone.png", &[4u8; 7]);
        let sitting = a_sitting_of(&folder, "scene.blend");
        let mut machine = AScriptedMachine::default();
        machine.documents.insert(
            sitting.root.as_bytes().to_vec(),
            Some(a_document(vec![
                a_reference(Kind::GeometryNodeBakeFolder, b"//gone/hero_bake/"),
                ReadReference {
                    source: Some("SEQUENCE".to_string()),
                    ..a_reference(Kind::Image, b"//gone/seq/f_####.png")
                },
                a_reference(Kind::Image, b"//gone/stone.png"),
            ])),
        );
        let record = a_record(&folder);
        let held = Mutex::new(WhatTheRunHolds::default());
        let door = OverOneConnection(record.connection());
        let _ = run(the_check_runs(&mut machine, &door, &TheUsersOwnComputer, &held, &sitting, &|| {}));
        let key = keyed(&sitting.server, &sitting.file);
        let what_it_reads = |held: &WhatTheRunHolds| {
            let mut rows: Vec<_> = held.sittings[&key]
                .rows
                .iter()
                .map(|one| {
                    (
                        String::from_utf8_lossy(one.row.ordering_key().as_bytes()).to_string(),
                        one.state,
                        one.state_when_the_check_ran,
                        one.members,
                        one.size,
                    )
                })
                .collect();
            rows.sort_by(|a, b| a.0.cmp(&b.0));
            rows
        };
        {
            let mut held = held.lock().expect("the run");
            assert!(what_it_reads(&held).iter().all(|row| row.1 == RowState::NotFound), "{:#?}", what_it_reads(&held));
            held.sittings.get_mut(&key).expect("the sitting").the_chosen_search_folder = Some(folder.bytes("library"));
            assert_eq!(
                a_decision_is_taken(&mut held, &key, ADecision::AddTheChosenSearchFolder, None, &door, &TheUsersOwnComputer),
                WhatADecisionCameTo::Taken
            );
        }
        let after_the_add = what_it_reads(&held.lock().expect("the run"));
        let bake = after_the_add.iter().find(|row| row.0.contains("hero_bake")).expect("the bake");
        assert_eq!((bake.3, bake.4), (Some(2), Some(2_560)), "{bake:?}");
        let set = after_the_add.iter().find(|row| row.0.contains("seq")).expect("the set");
        assert_eq!((set.3, set.4), (Some(2), Some(30)), "{set:?}");
        assert!(after_the_add.iter().all(|row| row.2 == RowState::FoundOnYourComputer), "{after_the_add:#?}");

        // **The same record, checked again.**
        let again = Mutex::new(WhatTheRunHolds::default());
        let _ = run(the_check_runs(&mut machine, &door, &TheUsersOwnComputer, &again, &sitting, &|| {}));
        assert_eq!(what_it_reads(&again.lock().expect("the run")), after_the_add);
    }

    /// **A reading of the user's own folders that looks at the run's lock and at the
    /// application's, as the panel's draw would, every time it reads anything.**
    #[cfg(unix)]
    struct LooksAtTheLocks<'a> {
        held: &'a Mutex<WhatTheRunHolds>,
        record: &'a Mutex<u32>,
        looked: std::sync::atomic::AtomicUsize,
    }
    #[cfg(unix)]
    impl LooksAtTheLocks<'_> {
        fn looks(&self) {
            self.looked.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            assert!(self.held.try_lock().is_ok(), "a draw would wait on the run's lock");
            assert!(self.record.try_lock().is_ok(), "a draw would wait on the application's lock");
        }
    }
    #[cfg(unix)]
    impl Readings for LooksAtTheLocks<'_> {
        fn the_local_files_content_identity(&self, file: &StoredLocation) -> Option<ContentIdentity> {
            self.looks();
            TheUsersOwnComputer.the_local_files_content_identity(file)
        }
        fn the_four_values_now(&self, machine: &MachineKey, at: &MachineLocation) -> smoozy_dependencies::readings::FourValuesNow {
            TheUsersOwnComputer.the_four_values_now(machine, at)
        }
        fn the_machine_copys_contents(&self, machine: &MachineKey, at: &MachineLocation) -> MachineContents {
            TheUsersOwnComputer.the_machine_copys_contents(machine, at)
        }
        fn enumerate_folder(&self, folder: &[u8]) -> FolderListing {
            self.looks();
            TheUsersOwnComputer.enumerate_folder(folder)
        }
        fn enumeration_casing_of(&self, folder: &[u8]) -> EnumerationCasing {
            self.looks();
            TheUsersOwnComputer.enumeration_casing_of(folder)
        }
        fn the_stored_name_reading(&self, path: &[u8]) -> StoredNameAnswer {
            self.looks();
            TheUsersOwnComputer.the_stored_name_reading(path)
        }
        fn the_sources_prefix_digest(&self, source: &StoredLocation, up_to: u64) -> Option<Digest> {
            TheUsersOwnComputer.the_sources_prefix_digest(source, up_to)
        }
    }
    #[cfg(unix)]
    struct TheDoorLocks<'a> {
        record: &'a Mutex<u32>,
        inner: &'a dyn TheCheckReachesTheRecord,
    }
    #[cfg(unix)]
    impl TheCheckReachesTheRecord for TheDoorLocks<'_> {
        fn with_the_record(&self, act: &mut dyn FnMut(&rusqlite::Connection)) -> bool {
            let _for_this_act_alone = self.record.lock().expect("the lock");
            self.inner.with_the_record(act)
        }
    }


    #[cfg(unix)]
    /// **`Dependency Upload & Remap` F1b: no draw waits on the add's search.** A
    /// reading of the user's own folders looks at the run's lock and at the
    /// application's, as the panel's draw would, every time the search reads
    /// anything. Until this the search ran inside both, and a real window answered
    /// nothing for 2 min 11 s.
    #[test]
    fn adding_a_search_folder_holds_no_lock_while_it_searches() {
        let folder = AFolder::made("no-lock");
        folder.holds("scene.blend", b"blend");
        folder.holds("library/stone.png", b"stone");
        let sitting = a_sitting_of(&folder, "scene.blend");
        let mut machine = AScriptedMachine::default();
        machine.documents.insert(
            sitting.root.as_bytes().to_vec(),
            Some(a_document(vec![a_reference(Kind::Image, b"//maps/stone.png")])),
        );
        let the_record = a_record(&folder);
        let held_as_the_app_holds_it = Mutex::new(WhatTheRunHolds::default());
        let _ = run(the_check_runs(
            &mut machine,
            &OverOneConnection(the_record.connection()),
            &TheUsersOwnComputer,
            &held_as_the_app_holds_it,
            &sitting,
            &|| {},
        ));
        let key = keyed(&sitting.server, &sitting.file);
        held_as_the_app_holds_it.lock().expect("the run").sittings.get_mut(&key).expect("the sitting").the_chosen_search_folder =
            Some(folder.bytes("library"));
        let lock = Mutex::new(0u32);
        let readings = LooksAtTheLocks { held: &held_as_the_app_holds_it, record: &lock, looked: Default::default() };
        let door = TheDoorLocks { record: &lock, inner: &OverOneConnection(the_record.connection()) };
        let told = std::sync::atomic::AtomicUsize::new(0);
        let came_to = the_chosen_folder_is_added(&held_as_the_app_holds_it, &key, &door, &readings, &|| {
            told.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        });
        assert_eq!(came_to, WhatADecisionCameTo::Taken);
        assert!(readings.looked.load(std::sync::atomic::Ordering::SeqCst) > 0, "the search read the folder");
        assert_eq!(told.load(std::sync::atomic::Ordering::SeqCst), 1, "the panel is told the search began");
        let held = held_as_the_app_holds_it.lock().expect("the run");
        assert_eq!(held.sittings[&key].rows[0].state, RowState::FoundOnYourComputer);
        assert!(!held.sittings[&key].the_search.as_ref().expect("the search").running);
    }

    /// **`Dependency Upload & Remap` F1c: no draw waits on *Locate…*'s look**, the
    /// located file's or the shift's over every row still missing; and what lands is
    /// what the record holds.
    #[cfg(unix)]
    #[test]
    fn locating_a_file_that_teaches_a_shift_holds_no_lock_while_it_looks() {
        let folder = AFolder::made("no-lock-locate");
        folder.holds("scene.blend", b"blend");
        folder.holds("elsewhere/maps/rust.png", b"rust");
        folder.holds("elsewhere/maps/dirt.png", b"dirt");
        let sitting = a_sitting_of(&folder, "scene.blend");
        let rust = RecordedLocation::as_the_reading_carried_it(&folder.bytes("gone/maps/rust.png"));
        let dirt = RecordedLocation::as_the_reading_carried_it(&folder.bytes("gone/maps/dirt.png"));
        let mut machine = AScriptedMachine::default();
        machine.documents.insert(
            sitting.root.as_bytes().to_vec(),
            Some(a_document(vec![
                ReadReference { recorded: rust.clone(), ..a_reference(Kind::Image, b"x") },
                ReadReference { recorded: dirt.clone(), ..a_reference(Kind::Image, b"x") },
            ])),
        );
        let the_record = a_record(&folder);
        let held = Mutex::new(WhatTheRunHolds::default());
        let _ = run(the_check_runs(&mut machine, &OverOneConnection(the_record.connection()), &TheUsersOwnComputer, &held, &sitting, &|| {}));
        let key = keyed(&sitting.server, &sitting.file);
        let at = |recorded: &RecordedLocation| {
            held.lock().expect("the run").sittings[&key].rows.iter().position(|row| row.row.references.contains(recorded)).expect("the row")
        };
        let rust_at = at(&rust);
        let lock = Mutex::new(0u32);
        let readings = LooksAtTheLocks { held: &held, record: &lock, looked: Default::default() };
        let door = TheDoorLocks { record: &lock, inner: &OverOneConnection(the_record.connection()) };
        let came_to = a_decision_that_looks_is_taken(
            &held,
            &key,
            &ADecision::Locate(folder.bytes("elsewhere/maps/rust.png")),
            Some(rust_at),
            &door,
            &readings,
        );
        assert_eq!(came_to, WhatADecisionCameTo::Taken);
        assert!(readings.looked.load(std::sync::atomic::Ordering::SeqCst) > 1, "the file and the shift looked");
        let held_now = held.lock().expect("the run");
        let sitting_now = &held_now.sittings[&key];
        assert!(sitting_now.rows.iter().all(|row| row.state == RowState::FoundOnYourComputer));
        assert_eq!(sitting_now.rows.iter().filter_map(|row| row.size).sum::<u64>(), 8);
        assert!(sitting_now.the_shift.is_some());
        let answers = TheDependencyRecord::over(the_record.connection()).the_answers_of(&sitting.holding).expect("they read");
        let how = |recorded: &RecordedLocation| answers.iter().find(|one| &one.key.reference == recorded).and_then(|one| one.resolution.clone()).map(|one| one.how);
        assert_eq!(how(&rust), Some(HowResolved::TheUsersOwnHand));
        assert!(matches!(how(&dirt), Some(HowResolved::AShift(_))));
    }

    /// **`Dependency Upload & Remap` F1c: no draw waits on *Choose a folder…*'s look**,
    /// a row it does not find stays as it was, and no shift is taught
    /// (`Dependency Upload & Remap` F26).
    #[cfg(unix)]
    #[test]
    fn choosing_a_folder_holds_no_lock_while_it_looks() {
        let folder = AFolder::made("no-lock-choose");
        folder.holds("scene.blend", b"blend");
        folder.holds("library/stone.png", b"stone");
        let sitting = a_sitting_of(&folder, "scene.blend");
        let mut machine = AScriptedMachine::default();
        machine.documents.insert(
            sitting.root.as_bytes().to_vec(),
            Some(a_document(vec![a_reference(Kind::Image, b"//maps/stone.png"), a_reference(Kind::Image, b"//maps/nowhere.png")])),
        );
        let the_record = a_record(&folder);
        let held = Mutex::new(WhatTheRunHolds::default());
        let _ = run(the_check_runs(&mut machine, &OverOneConnection(the_record.connection()), &TheUsersOwnComputer, &held, &sitting, &|| {}));
        let key = keyed(&sitting.server, &sitting.file);
        held.lock().expect("the run").selections.insert(key.clone(), [0, 1].into_iter().collect());
        let lock = Mutex::new(0u32);
        let readings = LooksAtTheLocks { held: &held, record: &lock, looked: Default::default() };
        let door = TheDoorLocks { record: &lock, inner: &OverOneConnection(the_record.connection()) };
        let came_to = a_decision_that_looks_is_taken(&held, &key, &ADecision::ChooseAFolder(folder.bytes("library")), None, &door, &readings);
        assert_eq!(came_to, WhatADecisionCameTo::Taken);
        assert!(readings.looked.load(std::sync::atomic::Ordering::SeqCst) > 0, "the folder was looked in");
        let held_now = held.lock().expect("the run");
        let sitting_now = &held_now.sittings[&key];
        let state_of = |ends: &[u8]| sitting_now.rows.iter().find(|row| row.row.ordering_key().as_bytes().ends_with(ends)).expect("the row").state;
        assert_eq!(state_of(b"stone.png"), RowState::FoundOnYourComputer);
        assert_eq!(state_of(b"nowhere.png"), RowState::NotFound);
        assert!(sitting_now.the_shift.is_none(), "no shift");
    }

    /// **`Dependency Upload & Remap` F1c: a row that changed while the look ran is
    /// left alone**, as `Dependency Upload & Remap` F1b has it.
    #[cfg(unix)]
    #[test]
    fn a_row_set_aside_while_the_look_ran_stays_set_aside() {
        let folder = AFolder::made("changed-while");
        folder.holds("scene.blend", b"blend");
        folder.holds("library/stone.png", b"stone");
        let sitting = a_sitting_of(&folder, "scene.blend");
        let mut machine = AScriptedMachine::default();
        machine.documents.insert(sitting.root.as_bytes().to_vec(), Some(a_document(vec![a_reference(Kind::Image, b"//maps/stone.png")])));
        let the_record = a_record(&folder);
        let held = Mutex::new(WhatTheRunHolds::default());
        let door = OverOneConnection(the_record.connection());
        let _ = run(the_check_runs(&mut machine, &door, &TheUsersOwnComputer, &held, &sitting, &|| {}));
        let key = keyed(&sitting.server, &sitting.file);
        let rows = the_choice_takes_its_rows(&held.lock().expect("the run"), &key, Some(0));
        let found = the_choice_looks(&rows, &folder.bytes("library"), &door, &TheUsersOwnComputer);
        let mut held_now = held.lock().expect("the run");
        held_now.sittings.get_mut(&key).expect("the sitting").rows[0].state = RowState::SetAside;
        assert_eq!(the_look_lands(&mut held_now, &key, found), WhatADecisionCameTo::Taken);
        assert_eq!(held_now.sittings[&key].rows[0].state, RowState::SetAside);
    }

    /// **The search over the user's folders, and a folder added from the
    /// panel searches again over what is still not found** — `Dependency
    /// Upload & Remap` F1, F3 and `Design/Dependency Upload & Remap § The
    /// search folders on this panel`.
    ///
    /// **macOS and Linux only** — *why five of these tests run on macOS and
    /// Linux only*, above `AFolder`, says why. *(This comment and its gate were
    /// pulled off the test by an insertion above it on 2026-11-03, and the
    /// Windows run of 2026-11-06 failed on the search not finding the stone;
    /// put back 2026-11-07.)*
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

    // ------------------------------- `Dependency Upload & Remap` G17b, kept readings

    /// **One file on the machine that loads as Blender does**: its four values,
    /// the libraries it links by their whole location, and the pictures it names.
    #[cfg(unix)]
    #[derive(Clone)]
    struct AFileThere {
        values: SourceReading,
        links: Vec<Vec<u8>>,
        pictures: Vec<&'static [u8]>,
        /// A file that will not open, though it is there.
        will_not_open: bool,
    }

    /// **A machine whose readings load what is there**, shaped on what
    /// `a_files_reading_grows_as_its_libraries_land_at_the_desk` read off six
    /// Blenders: a library that is there brings the next one's reference into the
    /// reading of the file above it, at a location measured from the opened file's
    /// own folder; an absent one is named and nothing below it is; and a file that
    /// is not there gives no reading and answers no four values.
    #[cfg(unix)]
    #[derive(Clone, Default)]
    struct TheFilesThere(Arc<Mutex<BTreeMap<Vec<u8>, AFileThere>>>);

    #[cfg(unix)]
    fn four(size: u64) -> SourceReading {
        SourceReading { size, inode: "7".to_string(), modification_time: 1, change_time: 1 }
    }

    #[cfg(unix)]
    impl TheFilesThere {
        fn lands(&self, at: &[u8], links: &[&[u8]], pictures: &[&'static [u8]]) {
            self.0.lock().expect("the files").insert(
                at.to_vec(),
                AFileThere {
                    values: four(1),
                    links: links.iter().map(|one| one.to_vec()).collect(),
                    pictures: pictures.to_vec(),
                    will_not_open: false,
                },
            );
        }

        fn change(&self, at: &[u8], change: impl FnOnce(&mut AFileThere)) {
            change(self.0.lock().expect("the files").get_mut(at).expect("the file"));
        }

        fn the_reading_of(&self, opened: &[u8]) -> Option<ReadingDocument> {
            let files = self.0.lock().expect("the files");
            let top = files.get(opened).filter(|file| !file.will_not_open)?;
            let mut references = Vec::new();
            let mut seen: Vec<Vec<u8>> = Vec::new();
            let mut waiting: Vec<AFileThere> = vec![top.clone()];
            while let Some(file) = waiting.pop() {
                for picture in &file.pictures {
                    references.push(a_reference(Kind::Image, picture));
                }
                for link in &file.links {
                    if seen.contains(link) {
                        continue;
                    }
                    seen.push(link.clone());
                    references.push(a_reference(Kind::Library, &as_read_from(opened, link)));
                    if let Some(below) = files.get(link) {
                        waiting.push(below.clone());
                    }
                }
            }
            Some(a_document(references))
        }
    }

    /// **A library's location as Blender reads it**: `//`, measured from the
    /// opened file's own folder.
    #[cfg(unix)]
    fn as_read_from(opened: &[u8], library: &[u8]) -> Vec<u8> {
        let folder = folder_of(opened);
        let from: Vec<&[u8]> = folder.split(|one| *one == b'/').filter(|one| !one.is_empty()).collect();
        let to: Vec<&[u8]> = library.split(|one| *one == b'/').filter(|one| !one.is_empty()).collect();
        let shared = from.iter().zip(&to).take_while(|(one, other)| one == other).count();
        let mut read = b"//".to_vec();
        for _ in shared..from.len() {
            read.extend_from_slice(b"../");
        }
        read.extend_from_slice(&to[shared..].join(&b'/'));
        read
    }

    #[cfg(unix)]
    struct AMachineThatLoads {
        files: TheFilesThere,
        inner: AScriptedMachine,
        /// **What happens once a reading of this file has been asked for**: a send
        /// landing mid-walk.
        after_reading: Option<(Vec<u8>, Box<dyn FnOnce() + Send>)>,
    }

    #[cfg(unix)]
    impl AMachineThatLoads {
        fn over(files: &TheFilesThere) -> AMachineThatLoads {
            AMachineThatLoads { files: files.clone(), inner: AScriptedMachine::default(), after_reading: None }
        }

        /// **The files read since this was last asked, one Blender start for each**,
        /// by name.
        fn the_files_read(&mut self) -> Vec<String> {
            let mut read: Vec<String> = self
                .inner
                .read
                .drain(..)
                .map(|one| String::from_utf8_lossy(smoozy_dependencies::bytes::last_segment(&one)).to_string())
                .collect();
            read.sort();
            read
        }
    }

    #[cfg(unix)]
    impl smoozy_dependencies::acting::checking::TheMachineAReadingIsAskedOf for AMachineThatLoads {
        async fn ask(&mut self, request: &Request) -> WhatCameBack {
            if let Some(blend) = request.fields.value(asked::THE_BLEND_FIELD) {
                let blend = blend.to_vec();
                let reading = self.files.the_reading_of(&blend);
                self.inner.documents.insert(blend.clone(), reading);
                if self.after_reading.as_ref().is_some_and(|(of, _)| *of == blend) {
                    if let Some((_, then)) = self.after_reading.take() {
                        then();
                    }
                }
            }
            self.inner.ask(request).await
        }

        async fn read_the_stream_back(&mut self, path: &[u8], length: u64) -> Result<Vec<u8>, String> {
            self.inner.read_the_stream_back(path, length).await
        }
    }

    #[cfg(unix)]
    impl TheMachineAFileIsReadOn for AMachineThatLoads {
        async fn the_four_values_of(&mut self, the_blend: &str) -> FourValuesNow {
            match self.files.0.lock().expect("the files").get(the_blend.as_bytes()) {
                Some(file) => FourValuesNow::Answered(file.values.clone()),
                None => FourValuesNow::TheChannelCannotAnswer,
            }
        }
    }

    /// **What a sitting leaves on the panel**, in an order two sittings can be
    /// compared in.
    #[cfg(unix)]
    fn the_rows_standing(held: &Mutex<WhatTheRunHolds>, sitting: &TheSittingIsOf) -> Vec<String> {
        let held = held.lock().expect("the run");
        let standing = &held.sittings[&keyed(&sitting.server, &sitting.file)];
        let mut rows: Vec<String> = standing
            .rows
            .iter()
            .map(|row| format!("{:?} {:?} {:?}", row.holding, row.row.ordering_key(), row.state))
            .collect();
        rows.sort();
        rows.push(format!("linked files read: {}", standing.linked_files_read));
        rows
    }

    #[cfg(unix)]
    struct AShotOfThreeLevels {
        folder: AFolder,
        top: Vec<u8>,
        a: Vec<u8>,
        b: Vec<u8>,
        c: Vec<u8>,
    }

    #[cfg(unix)]
    fn a_shot(named: &str) -> AShotOfThreeLevels {
        let folder = AFolder::made(named);
        folder.holds("shot/top.blend", b"blend");
        let (top, a, b, c) = (
            folder.bytes("shot/top.blend"),
            folder.bytes("shot/libs/a.blend"),
            folder.bytes("shot/libs/deep/b.blend"),
            folder.bytes("other/c.blend"),
        );
        AShotOfThreeLevels { folder, top, a, b, c }
    }

    /// **`Dependency Upload & Remap` G17b: three levels of libraries, sent in
    /// order, the top one first.** Each landing reads again every file above it,
    /// and nothing else; the rows are what a run with nothing kept reads at every
    /// round; and a round with nothing new starts no Blender.
    #[cfg(unix)]
    #[test]
    fn three_levels_of_libraries_are_read_again_as_each_lands_and_nothing_else_is() {
        let shot = a_shot("three-levels");
        let there = TheFilesThere::default();
        there.lands(&shot.top, &[&shot.a], &[]);
        let sitting = a_sitting_of(&shot.folder, "shot/top.blend");
        let mut machine = AMachineThatLoads::over(&there);
        let record = a_record(&shot.folder);
        let door = OverOneConnection(record.connection());
        let held = Mutex::new(WhatTheRunHolds::default());
        let a_round = |wanted: &[&str], machine: &mut AMachineThatLoads| {
            let came_to = run(the_check_runs(machine, &door, &TheUsersOwnComputer, &held, &sitting, &|| {}));
            assert_eq!(came_to, TheCheckCameTo::TheSittingStands(Completion::Completed));
            assert_eq!(machine.the_files_read(), wanted);
            // **Against a run with nothing kept.**
            let fresh = Mutex::new(WhatTheRunHolds::default());
            let _ = run(the_check_runs(
                &mut AMachineThatLoads::over(&there),
                &door,
                &TheUsersOwnComputer,
                &fresh,
                &sitting,
                &|| {},
            ));
            assert_eq!(the_rows_standing(&held, &sitting), the_rows_standing(&fresh, &sitting));
        };
        // A library not there gives no reading and is tried each round: the
        // reading that failed is not kept.
        a_round(&["a.blend", "top.blend"], &mut machine);
        a_round(&["a.blend"], &mut machine);

        there.lands(&shot.a, &[&shot.b], &[]);
        a_round(&["a.blend", "b.blend", "top.blend"], &mut machine);
        a_round(&["b.blend"], &mut machine);

        there.lands(&shot.b, &[&shot.c], &[]);
        a_round(&["a.blend", "b.blend", "c.blend", "top.blend"], &mut machine);

        there.lands(&shot.c, &[], &[b"//c_picture.png"]);
        a_round(&["a.blend", "b.blend", "c.blend", "top.blend"], &mut machine);

        a_round(&[], &mut machine);
    }

    /// **`Dependency Upload & Remap` G14a: three levels of libraries, and a send
    /// landing mid-walk: the settled answer equals a fresh check's.** The second
    /// Sesterce pass's false calm: *5 missing*, *5 linked files*, nothing to upload,
    /// and the next *Check again*, on no change at all, found 85 missing.
    #[cfg(unix)]
    #[test]
    fn a_send_landing_during_the_walk_settles_on_what_a_fresh_check_says() {
        let shot = a_shot("a-send-mid-walk");
        let there = TheFilesThere::default();
        there.lands(&shot.top, &[&shot.a], &[]);
        let sitting = a_sitting_of(&shot.folder, "shot/top.blend");
        let mut machine = AMachineThatLoads::over(&there);
        let record = a_record(&shot.folder);
        let door = OverOneConnection(record.connection());
        let held = Arc::new(Mutex::new(WhatTheRunHolds::default()));
        // **The walk reaches the first library while it is absent, and it lands
        // straight after, with the two below it.** Its reading has already failed.
        let (there_then, held_then, key) = (there.clone(), Arc::clone(&held), keyed(&sitting.server, &sitting.file));
        let (a, b, c) = (shot.a.clone(), shot.b.clone(), shot.c.clone());
        machine.after_reading = Some((
            shot.a.clone(),
            Box::new(move || {
                there_then.lands(&a, &[&b], &[]);
                there_then.lands(&b, &[&c], &[]);
                there_then.lands(&c, &[], &[b"//c_picture.png"]);
                a_send_ended(&held_then, &key);
            }),
        ));
        let came_to = run(the_check_runs(&mut machine, &door, &TheUsersOwnComputer, &*held, &sitting, &|| {}));
        assert_eq!(came_to, TheCheckCameTo::TheSittingStands(Completion::Completed));
        let fresh = Mutex::new(WhatTheRunHolds::default());
        let _ = run(the_check_runs(&mut AMachineThatLoads::over(&there), &door, &TheUsersOwnComputer, &fresh, &sitting, &|| {}));
        assert_eq!(the_rows_standing(&held, &sitting), the_rows_standing(&fresh, &sitting));
        let held = held.lock().expect("the run");
        assert!(held.checks_running.is_empty() && held.files_being_read.is_empty(), "nothing goes on saying it reads");
        assert!(held.a_round_is_owed.is_empty(), "the round was taken");
    }

    /// **`Dependency Upload & Remap` G14a: a send that ends with no check running
    /// owes nothing, a sitting stopped takes no further round, and a press during a
    /// walk owes none.**
    #[cfg(unix)]
    #[test]
    fn only_a_send_ending_during_a_walk_owes_a_round() {
        let shot = a_shot("no-round-owed");
        let there = TheFilesThere::default();
        there.lands(&shot.top, &[], &[]);
        let sitting = a_sitting_of(&shot.folder, "shot/top.blend");
        let key = keyed(&sitting.server, &sitting.file);
        let held = Arc::new(Mutex::new(WhatTheRunHolds::default()));
        a_send_ended(&held, &key);
        assert!(held.lock().expect("the run").a_round_is_owed.is_empty(), "no check was running");

        // **Stopped during its walk, with a send ending in the same walk.**
        let mut machine = AMachineThatLoads::over(&there);
        let record = a_record(&shot.folder);
        let door = OverOneConnection(record.connection());
        let (held_then, key_then) = (Arc::clone(&held), key.clone());
        machine.after_reading = Some((
            shot.top.clone(),
            Box::new(move || {
                a_send_ended(&held_then, &key_then);
                *held_then.lock().expect("the run").checks_running.get_mut(&key_then).expect("it runs") = true;
            }),
        ));
        let _ = run(the_check_runs(&mut machine, &door, &TheUsersOwnComputer, &*held, &sitting, &|| {}));
        assert_eq!(machine.the_files_read(), ["top.blend"], "no further round");
        let held = held.lock().expect("the run");
        assert!(held.a_round_is_owed.is_empty() && held.checks_running.is_empty());
    }

    /// **`Dependency Upload & Remap` A9e's (b): a library landed holds until a
    /// sitting that began after it has ended, by any ending, and a walk that began
    /// before it does not answer.**
    #[cfg(unix)]
    #[test]
    fn a_library_landed_is_answered_only_by_a_sitting_that_began_after_it() {
        use crate::surface::the_dependency_check::{a_library_landed, what_holds_render, WhyRenderIsHeld};
        let shot = a_shot("a-library-answered");
        let there = TheFilesThere::default();
        there.lands(&shot.top, &[], &[]);
        let sitting = a_sitting_of(&shot.folder, "shot/top.blend");
        let key = keyed(&sitting.server, &sitting.file);
        let record = a_record(&shot.folder);
        let door = OverOneConnection(record.connection());
        let held = Arc::new(Mutex::new(WhatTheRunHolds::default()));
        let holds = |held: &Mutex<WhatTheRunHolds>| what_holds_render(held, &sitting.server, &sitting.file);
        assert_eq!(holds(&held), None);
        a_library_landed(&held, &key);
        assert_eq!(holds(&held), Some(WhyRenderIsHeld::CheckFirst));

        // **It lands again during a walk**: that walk answers the first, not this.
        let mut machine = AMachineThatLoads::over(&there);
        let (held_then, key_then) = (Arc::clone(&held), key.clone());
        let seen: Arc<Mutex<Option<WhyRenderIsHeld>>> = Default::default();
        let (seen_then, server, file) = (Arc::clone(&seen), sitting.server.clone(), sitting.file.clone());
        machine.after_reading = Some((
            shot.top.clone(),
            Box::new(move || {
                *seen_then.lock().expect("seen") = what_holds_render(&held_then, &server, &file);
                a_library_landed(&held_then, &key_then);
            }),
        ));
        let _ = run(the_check_runs(&mut machine, &door, &TheUsersOwnComputer, &*held, &sitting, &|| {}));
        assert_eq!(*seen.lock().expect("seen"), Some(WhyRenderIsHeld::ItsCheckIsRunning));
        assert_eq!(holds(&held), Some(WhyRenderIsHeld::CheckFirst), "that walk began before it");
        let _ = run(the_check_runs(&mut machine, &door, &TheUsersOwnComputer, &*held, &sitting, &|| {}));
        assert_eq!(holds(&held), None);

        // **A walk with nothing landed holds nothing.**
        let (held_then, server, file) = (Arc::clone(&held), sitting.server.clone(), sitting.file.clone());
        let seen_then = Arc::clone(&seen);
        there.change(&shot.top, |file| file.values.size += 1);
        machine.after_reading = Some((
            shot.top.clone(),
            Box::new(move || {
                *seen_then.lock().expect("seen") = what_holds_render(&held_then, &server, &file);
            }),
        ));
        let _ = run(the_check_runs(&mut machine, &door, &TheUsersOwnComputer, &*held, &sitting, &|| {}));
        assert_eq!(*seen.lock().expect("seen"), None);
    }

    /// **`Dependency Upload & Remap` G17b: a change in any of the four values
    /// reads the file again**, and every file above a library that changed.
    #[cfg(unix)]
    #[test]
    fn a_file_changed_in_any_of_its_four_values_is_read_again() {
        let shot = a_shot("four-values");
        let there = TheFilesThere::default();
        there.lands(&shot.top, &[&shot.a], &[]);
        there.lands(&shot.a, &[&shot.b], &[]);
        there.lands(&shot.b, &[], &[]);
        let sitting = a_sitting_of(&shot.folder, "shot/top.blend");
        let mut machine = AMachineThatLoads::over(&there);
        let record = a_record(&shot.folder);
        let door = OverOneConnection(record.connection());
        let held = Mutex::new(WhatTheRunHolds::default());
        let a_round = |machine: &mut AMachineThatLoads| {
            let _ = run(the_check_runs(machine, &door, &TheUsersOwnComputer, &held, &sitting, &|| {}));
            machine.the_files_read()
        };
        assert_eq!(a_round(&mut machine), ["a.blend", "b.blend", "top.blend"]);
        assert!(a_round(&mut machine).is_empty());

        let the_four: [fn(&mut SourceReading); 4] = [
            |values| values.size += 1,
            |values| values.inode.push('8'),
            |values| values.modification_time += 1,
            |values| values.change_time += 1,
        ];
        for one in the_four {
            there.change(&shot.top, |file| one(&mut file.values));
            assert_eq!(a_round(&mut machine), ["top.blend"]);
            assert!(a_round(&mut machine).is_empty());
            there.change(&shot.b, |file| one(&mut file.values));
            assert_eq!(a_round(&mut machine), ["a.blend", "b.blend", "top.blend"]);
            assert!(a_round(&mut machine).is_empty());
        }
    }

    /// **`Dependency Upload & Remap` G17b: a different Blender reads it again**,
    /// and the first Blender's reading still stands for that Blender. Another
    /// machine holding a file at the same location is not this machine.
    #[cfg(unix)]
    #[test]
    fn a_different_blender_reads_it_again_and_so_does_a_different_machine() {
        let shot = a_shot("two-blenders");
        let there = TheFilesThere::default();
        there.lands(&shot.top, &[], &[]);
        let sitting = a_sitting_of(&shot.folder, "shot/top.blend");
        let other_blender = TheSittingIsOf {
            with: BlenderIdentity {
                path: "/root/blender/blender-5.3.0/blender".to_string(),
                version: Version { major: 5, minor: 3, patch: 0 },
            },
            ..sitting.clone()
        };
        let same_version_other_place = TheSittingIsOf {
            with: BlenderIdentity { path: "/root/blender/other/blender".to_string(), ..sitting.with.clone() },
            ..sitting.clone()
        };
        let other_machine = a_sitting_of(&shot.folder, "shot/top.blend");
        let mut machine = AMachineThatLoads::over(&there);
        let record = a_record(&shot.folder);
        let door = OverOneConnection(record.connection());
        let held = Mutex::new(WhatTheRunHolds::default());
        let a_round = |of: &TheSittingIsOf, machine: &mut AMachineThatLoads| {
            let _ = run(the_check_runs(machine, &door, &TheUsersOwnComputer, &held, of, &|| {}));
            machine.the_files_read().len()
        };
        assert_eq!(a_round(&sitting, &mut machine), 1);
        assert_eq!(a_round(&sitting, &mut machine), 0);
        assert_eq!(a_round(&other_blender, &mut machine), 1);
        assert_eq!(a_round(&same_version_other_place, &mut machine), 1);
        assert_eq!(a_round(&other_machine, &mut machine), 1);
        assert_eq!(a_round(&sitting, &mut machine), 0);
    }

    /// **`Dependency Upload & Remap` Q5a, through the sitting: while the walk over
    /// the file's own rows runs the panel counts with *Stop* beside it, where it
    /// read *Reading the file…*, and a press ends the sitting *stopped by the
    /// user*.** The count's words are Design's and the drawing draws them.
    #[cfg(unix)]
    #[test]
    fn the_walk_counts_on_the_panel_and_a_press_of_stop_ends_the_sitting() {
        let shot = a_shot("the-walk");
        shot.folder.holds("shot/wood.png", b"wood");
        shot.folder.holds("shot/stone.png", b"stone");
        let there = TheFilesThere::default();
        there.lands(&shot.top, &[], &[b"//wood.png", b"//stone.png"]);
        let sitting = TheSittingIsOf {
            layout: Some(smoozy_job_files::layout::TheLayout::of(
                smoozy_job_files::layout::MachineRoot::chosen("/ephemeral/smoozy").expect("a root"),
                smoozy_job_files::layout::WhereThePackageAreaSits::UnderTheRoot,
            )),
            ..a_sitting_of(&shot.folder, "shot/top.blend")
        };
        let key = keyed(&sitting.server, &sitting.file);
        let record = a_record(&shot.folder);
        let door = OverOneConnection(record.connection());
        let held = Mutex::new(WhatTheRunHolds::default());
        let seen = Mutex::new(Vec::new());
        let told = || {
            let mut held = held.lock().expect("the run");
            let panel = crate::surface::the_dependency_check::the_panel(&held, &sitting.server, &sitting.file, None);
            if panel.the_walks_stop.is_some() {
                let line = panel.still_reading.expect("the count's line");
                seen.lock().expect("seen").push(format!("{line:?}"));
                *held.checks_running.get_mut(&key).expect("it runs") = true;
            }
        };
        let mut machine = AMachineThatLoads::over(&there);
        let came_to = run(the_check_runs(&mut machine, &door, &TheUsersOwnComputer, &held, &sitting, &told));
        assert_eq!(came_to, TheCheckCameTo::TheSittingStands(Completion::StoppedByTheUser));
        let seen = seen.lock().expect("seen");
        assert_eq!(seen.len(), 1, "{seen:?}");
        for part in ["Checking files on the", "Verbatim(\"0\")", "Verbatim(\"2\")"] {
            assert!(seen[0].contains(part), "{part}: {}", seen[0]);
        }
        let held = held.lock().expect("the run");
        let standing = &held.sittings[&key];
        assert_eq!(standing.the_walk, None, "the line goes when the rows land");
        assert_eq!(standing.rows.len(), 2);
        assert!(standing.rows.iter().all(|one| one.state == RowState::FoundOnYourComputer));
    }

    /// **`Dependency Upload & Remap` G17b: a reading that failed is not kept**,
    /// whether the file is the one pressed or a library below it, and what stands
    /// after it mends is kept as any other.
    #[cfg(unix)]
    #[test]
    fn a_reading_that_failed_is_not_kept() {
        let shot = a_shot("failed");
        let there = TheFilesThere::default();
        there.lands(&shot.top, &[&shot.a], &[]);
        there.lands(&shot.a, &[], &[]);
        there.change(&shot.top, |file| file.will_not_open = true);
        let sitting = a_sitting_of(&shot.folder, "shot/top.blend");
        let mut machine = AMachineThatLoads::over(&there);
        let record = a_record(&shot.folder);
        let door = OverOneConnection(record.connection());
        let held = Mutex::new(WhatTheRunHolds::default());
        let a_round = |machine: &mut AMachineThatLoads| {
            let came_to = run(the_check_runs(machine, &door, &TheUsersOwnComputer, &held, &sitting, &|| {}));
            (came_to, machine.the_files_read())
        };
        for _ in 0..2 {
            let (came_to, read) = a_round(&mut machine);
            assert_eq!(came_to, TheCheckCameTo::ItGaveNothing(Ending::TheFileCouldNotBeOpenedByThisVersion));
            assert_eq!(read, ["top.blend"]);
        }
        there.change(&shot.top, |file| file.will_not_open = false);
        there.change(&shot.a, |file| file.will_not_open = true);
        assert_eq!(a_round(&mut machine).1, ["a.blend", "top.blend"]);
        assert_eq!(a_round(&mut machine).1, ["a.blend"]);
        there.change(&shot.a, |file| file.will_not_open = false);
        assert_eq!(a_round(&mut machine).1, ["a.blend"]);
        assert!(a_round(&mut machine).1.is_empty());
    }

    /// **`Dependency Upload & Remap` G17b: a four-values read that does not answer
    /// keeps nothing.** On that channel a file not there and a channel that cannot
    /// answer read the same, so the library is read each round, as it always was.
    #[cfg(unix)]
    #[test]
    fn a_library_whose_four_values_do_not_answer_is_read_each_round() {
        struct NoValuesFor {
            machine: AMachineThatLoads,
            at: Vec<u8>,
        }
        impl smoozy_dependencies::acting::checking::TheMachineAReadingIsAskedOf for NoValuesFor {
            async fn ask(&mut self, request: &Request) -> WhatCameBack {
                self.machine.ask(request).await
            }
            async fn read_the_stream_back(&mut self, path: &[u8], length: u64) -> Result<Vec<u8>, String> {
                self.machine.read_the_stream_back(path, length).await
            }
        }
        impl TheMachineAFileIsReadOn for NoValuesFor {
            async fn the_four_values_of(&mut self, the_blend: &str) -> FourValuesNow {
                if the_blend.as_bytes() == self.at {
                    return FourValuesNow::TheChannelCannotAnswer;
                }
                self.machine.the_four_values_of(the_blend).await
            }
        }
        let shot = a_shot("no-values");
        let there = TheFilesThere::default();
        there.lands(&shot.top, &[&shot.a], &[]);
        there.lands(&shot.a, &[], &[]);
        let sitting = a_sitting_of(&shot.folder, "shot/top.blend");
        let mut machine = NoValuesFor { machine: AMachineThatLoads::over(&there), at: shot.a.clone() };
        let record = a_record(&shot.folder);
        let door = OverOneConnection(record.connection());
        let held = Mutex::new(WhatTheRunHolds::default());
        for wanted in [vec!["a.blend", "top.blend"], vec!["a.blend"], vec!["a.blend"]] {
            let _ = run(the_check_runs(&mut machine, &door, &TheUsersOwnComputer, &held, &sitting, &|| {}));
            assert_eq!(machine.machine.the_files_read(), wanted);
        }
    }

    /// **`Dependency Upload & Remap` G17b: a reading taken while a file is going up
    /// to that machine is not kept.** The library lands while Blender reads the file
    /// above it, after Blender looked. Its four values are read after the reading,
    /// so they say it is there, and were that reading kept it would never be read
    /// again. A file that began before the reading does the same.
    #[cfg(unix)]
    #[test]
    fn a_reading_taken_while_a_file_goes_up_is_not_kept() {
        let shot = a_shot("going-up");
        let there = TheFilesThere::default();
        there.lands(&shot.top, &[&shot.a], &[]);
        let sitting = a_sitting_of(&shot.folder, "shot/top.blend");
        let record = a_record(&shot.folder);
        let door = OverOneConnection(record.connection());
        let held = Arc::new(Mutex::new(WhatTheRunHolds::default()));
        let mut machine = AMachineThatLoads::over(&there);

        // **It begins and lands inside the reading of the file above it.**
        let (files, a, b, held_then, machine_key) =
            (there.clone(), shot.a.clone(), shot.b.clone(), Arc::clone(&held), sitting.machine.clone());
        let done = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let (once, once_more) = (Arc::clone(&done), Arc::clone(&done));
        machine.inner.after_each_reading = Some(Arc::new(move || {
            if !once.swap(true, std::sync::atomic::Ordering::SeqCst) {
                let mut held = held_then.lock().expect("the run");
                held.the_files_going_up.one_begins(&machine_key);
                files.lands(&a, &[&b], &[]);
                held.the_files_going_up.one_ends(&machine_key);
            }
        }));
        let _ = run(the_check_runs(&mut machine, &door, &TheUsersOwnComputer, &*held, &sitting, &|| {}));
        assert!(once_more.load(std::sync::atomic::Ordering::SeqCst));
        // That round read `a` as it stood, once it reached it.
        let _ = machine.the_files_read();
        let _ = run(the_check_runs(&mut machine, &door, &TheUsersOwnComputer, &*held, &sitting, &|| {}));
        assert_eq!(machine.the_files_read(), ["b.blend", "top.blend"], "the file above it is read again");

        // **One that began before the reading and has not ended.**
        held.lock().expect("the run").the_kept_readings = Default::default();
        held.lock().expect("the run").the_files_going_up.one_begins(&sitting.machine);
        let _ = run(the_check_runs(&mut machine, &door, &TheUsersOwnComputer, &*held, &sitting, &|| {}));
        assert_eq!(machine.the_files_read(), ["a.blend", "b.blend", "top.blend"]);
        assert_eq!(held.lock().expect("the run").the_kept_readings.how_many(), 0, "nothing is kept while one is on its way");
        held.lock().expect("the run").the_files_going_up.one_ends(&sitting.machine);
        let _ = run(the_check_runs(&mut machine, &door, &TheUsersOwnComputer, &*held, &sitting, &|| {}));
        assert_eq!(machine.the_files_read(), ["a.blend", "b.blend", "top.blend"]);
        let _ = run(the_check_runs(&mut machine, &door, &TheUsersOwnComputer, &*held, &sitting, &|| {}));
        assert_eq!(machine.the_files_read(), ["b.blend"], "the one still not there is tried");
    }

    /// **`Design/Dependency Upload & Remap`: *Check again* on a file already read
    /// still says what is going on.** No Blender starts, and *Reading the file…*
    /// stands from the press until the rows land, at least the time it takes to read.
    /// The words after it are the words a fresh reading leaves.
    #[cfg(unix)]
    #[test]
    fn check_again_on_a_file_already_read_still_says_it_is_reading() {
        let shot = a_shot("the-press");
        let there = TheFilesThere::default();
        there.lands(&shot.top, &[], &[b"//gone.png"]);
        let sitting = a_sitting_of(&shot.folder, "shot/top.blend");
        let mut machine = AMachineThatLoads::over(&there);
        let record = a_record(&shot.folder);
        let door = OverOneConnection(record.connection());
        let held = Arc::new(Mutex::new(WhatTheRunHolds::default()));
        let _ = run(the_check_runs(&mut machine, &door, &TheUsersOwnComputer, &*held, &sitting, &|| {}));
        assert_eq!(machine.the_files_read(), ["top.blend"]);
        let the_words = |held: &WhatTheRunHolds| {
            let panel = crate::surface::the_dependency_check::the_panel(held, &sitting.server, &sitting.file, None);
            (panel.still_reading.map(|words| words.said().replace('\u{a0}', " ")), format!("{:?}", panel.the_table), format!("{:?}", panel.the_summary))
        };
        let the_first_round = the_words(&held.lock().expect("the run"));

        let seen: Arc<Mutex<Vec<Option<String>>>> = Default::default();
        let (held_then, seen_then, server, file) = (Arc::clone(&held), Arc::clone(&seen), sitting.server.clone(), sitting.file.clone());
        let began = std::time::Instant::now();
        let landed_at: Arc<Mutex<Option<std::time::Duration>>> = Default::default();
        let landed_then = Arc::clone(&landed_at);
        let _ = run(the_check_runs(&mut machine, &door, &TheUsersOwnComputer, &*held, &sitting, &move || {
            let held = held_then.lock().expect("the run");
            let panel = crate::surface::the_dependency_check::the_panel(&held, &server, &file, None);
            let said = panel.still_reading.map(|words| words.said().replace('\u{a0}', " "));
            if said.is_none() {
                landed_then.lock().expect("the time").get_or_insert(began.elapsed());
            }
            seen_then.lock().expect("seen").push(said);
        }));
        assert!(machine.the_files_read().is_empty(), "no Blender started");
        let seen = seen.lock().expect("seen");
        assert_eq!(seen.first(), Some(&Some("Reading the file…".to_string())), "{seen:?}");
        assert!(landed_at.lock().expect("the time").expect("the rows landed") >= THE_LEAST_A_KEPT_READING_SHOWS);
        assert_eq!(the_words(&held.lock().expect("the run")), the_first_round);
    }

    #[allow(dead_code)]
    fn unused(_: &Path, _: StoredSource) {}
}
