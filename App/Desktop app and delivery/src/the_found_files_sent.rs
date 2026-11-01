//! **The found files sent up** — `Dependency Upload & Remap` §5's uploads,
//! performed for the rows a check found on the user's own computer, with
//! `Dependency Upload & Remap` K7's placement record and V13's marks written at
//! each publication, and `Dependency Upload & Remap` Q5's first route read so
//! that a file already placed is never sent again. Built at step 10c's
//! dependency wave piece 6, 2026-09-29.
//!
//! **What this file is: the caller `acting::sending` never had.** The mirror
//! (`Dependency Upload & Remap` A4, A5), the directories made segment by
//! segment (`Dependency Upload & Remap` A7), the arrangement of one asset's
//! transfer (`Dependency Upload & Remap` A9), the retry as a new transfer
//! (`Dependency Upload & Remap` A11), the establishment (`Dependency Upload &
//! Remap` Q5), the confirmation (`Dependency Upload & Remap` Q7) and the two
//! marks (`Dependency Upload & Remap` V13, `Dependency Upload & Remap` V13a)
//! were all built in `App/Dependency Upload & Remap/`,
//! and nothing in the running application called one of them. **Nothing here
//! re-decides any of them.**
//!
//! **And the transfer is the transfer wave's, used rather than copied**:
//! `Desktop app and delivery` R17's per-file slot and the server's seat
//! (`crate::uploading::a_slot_and_a_seat`), the carry-on across a repair and
//! the user's own Stop (`crate::uploading::carried_across_repairs`), the row
//! told as the bytes move (`crate::downloading::TheRowIsTold`), the request
//! held under way in the run's register (`TheRunsTransfers`), one save per
//! checkpoint (`crate::record::transfer_records::put`), and `File Browser` K8's
//! watch and K9's gate on the source. **`Design/Transfers § The dependency
//! check's own transfers`: *"Its rows are the same rows"*.**
//!
//! # The order of one send
//!
//! 2. **The pre-flight, per file** (`Dependency Upload & Remap` Q8: *"it fires
//!    inside the send pre-flight, where Q5's establishment is already being
//!    made"*): the mirror, the local file's content identity, and — where a
//!    placement record names the mirror — the machine copy's four values and
//!    Q5 over them; **where they do not establish it, the machine's own
//!    read-through** (`Dependency Upload & Remap` Q3, since the 38's piece 7),
//!    asked only where the mirror's folder holds the name. **Already on the
//!    server: not sent.** **A different file there: the row asks replace or
//!    stop** (Q8) and nothing of it is sent until Replace. **Not established
//!    either way** — the read-through could not be taken — **the file is sent
//!    again**: *"the cost is bandwidth and never a wrong render"*.
//! 3. **One transfer per file**, in the dependency-upload class
//!    (`Dependency Upload & Remap` A9), each on its own slot and seat, **up to
//!    four at once** (step 10c's eleventh-pass fix piece 4), the mirror's
//!    folders made first (`Dependency Upload & Remap` A7) — **each once in a
//!    send** — staged and published last under `Server protocol and
//!    transfers` §7.
//! 4. **At each publication**, the four values of the published file are read
//!    and **`Dependency Upload & Remap` K7's placement record is written, with
//!    V13's marks, in one save** (`Dependency Upload & Remap` Q11: publication is
//!    the only event that creates one).
//! 5. **The row's state from its files' publications** (§10: partial across
//!    rows and across a set's members), and **the check standing brought up to
//!    date** with the sitting's rows (`Dependency Upload & Remap` V11: *"placed
//!    and published by this sitting's uploads"* is confirmed).
//!
//! # A linked `.blend`, carrying on, and the read-through — the 38's piece 7
//!
//! - **A linked `.blend` row is sent like any other** and then fixed: the
//!   check reads below it at its mirror (`Dependency Upload & Remap` G14) and
//!   produces its fixed copy at the check's end ([`a_library_is_fixed`], M13
//!   to M16), and *Try again* on a *could not be fixed* row runs the fixing
//!   alone (`Dependency Upload & Remap` M16a). **Until this piece it stayed *found on your computer***,
//!   the upload did not count it, and nothing below it was ever read or fixed.
//! - **`Dependency Upload & Remap` A10's resume of an earlier send's part is
//!   taken up** ([`the_part_an_earlier_send_left`]): a part this file left at
//!   a mirror, interrupted or ended with its run, is carried on where the
//!   source still digests to its checkpoint — `File Browser` K10 and K12's
//!   one unchanged-source answer — and abandoned and sent afresh where it
//!   does not.
//! - **Q5's second route is the machine's own digest**, so a file at its
//!   mirror that no record names reads *already on the server* where its
//!   contents are the same, and a different file there asks replace or stop.
//!   **A file the user put somewhere else through Files is still not found
//!   there**: Q9's lookup reads placement records alone, and a copy Smoozy did
//!   not place is missed — one redundant send, never a wrong render.
//!
//! # What is not done here, said rather than left to be found
//!
//! - **The live-job line over the overwrite box** reads `Dependency Upload &
//!   Remap` K9's resolved sets, which the submission writes (dependency piece
//!   7). Until it does no set names a mirror, and the line is not drawn.
//!
//! **Every marker in this file that belongs to another document is written out
//! with that document at its own site, and this file keeps no header list.**

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::SystemTime;

use smoozy_dependencies::acting::marks::what_this_publication_marks;
use smoozy_dependencies::acting::sending::{
    arrange_one_assets_upload, make_the_mirror_directories, open_an_asset_upload,
    HowTheDirectoryWent, SendingFilesUpWasOpen, TheMirrorDirectoriesSeen,
};
use smoozy_dependencies::bytes::{join, MachineLocation, StoredLocation};
use smoozy_dependencies::check::Kind;
use smoozy_dependencies::contents::{
    already_on_the_machine, confirm, set_row_membership, ContentIdentity, Establishment, Membership,
};
use smoozy_dependencies::landing::{
    a_row_is_on_its_way, is_on_its_way, mirror_of, CouldNotBeSent, WhatARowSends,
    THE_TRANSFER_PRIORITY_CLASS,
};
use smoozy_dependencies::readings::{
    EnumerationCasing, FolderListing, FourValuesNow, MachineContents, MirroredFolderListing,
    Readings, StoredNameAnswer,
};
use smoozy_dependencies::record::{
    ConfirmationOutcome, InMemoryRecord, MachineKey, PlacementRecord, Published, ResolvedTo,
};
use smoozy_dependencies::rows::RowState;
use smoozy_dependencies::standing::{
    counts_at_completion, every_row_is_confirmed_or_set_aside, CheckStanding, FixingStatus,
    RowAtCompletion, WhatTheRecordSays,
};
use smoozy_file_browser::continuation::{TheSourceUnderWatch, TheSourcesPublicationGate};
use smoozy_file_browser::partial::THE_RESERVED_STAGING_FOLDER;
use smoozy_file_browser::pool::THE_UPLOAD_DIRECTIONS_CEILING;
use smoozy_server_protocol::subsystem::{StatusCode, SubsystemFailure};
use smoozy_server_protocol::transfers::integrity::Digest;
use smoozy_server_protocol::transfers::moving::{KeepsTheRecord, LocalFailure};
use smoozy_server_protocol::transfers::publication::Intent;
use smoozy_server_protocol::transfers::{Ending, TransferRecord};
use smoozy_state::identities::{Identity, IdentityKind};

use crate::checking_a_file::{TheCheckReachesTheRecord, TheSittingIsOf, TheUsersOwnComputer};
use crate::downloading::{
    every_one_of, the_user_stopped_this_ending, TheNextFile, TheRowIsTold, TheRunsTransfers,
};
use crate::reading_the_file::TheMachineAFileIsReadOn;
use crate::record::dependency_records::TheDependencyRecord;
use crate::servers::sessions::{AHeldConnection, ThisServersSessions};
use crate::surface::the_dependency_check::{
    keyed, ACheckedRow, AnOverwrite, SeveralForOneReason, TheFolderDiffers, WhatTheRunHolds,
    WhyItDidNotGoUp,
};
use crate::transfers::ThePlatformFileAdapter;
use crate::uploading::{
    a_slot_and_a_seat, carried_across_repairs, the_folder_holding, the_refusals_this_run,
    TheRecordAnUploadWrites, TheStagingFoldersSeen, TheWayBack,
};

// ════════════════════════════════════════════════════════════════════════════
// What the run has sent — the half Transfers reads
// ════════════════════════════════════════════════════════════════════════════

/// Read the file without holding the run-list lock while taking the record.
fn the_file_of_the_send_is_saved(record: &dyn TheRecordAnUploadWrites, request: &Identity, at: usize) {
    let file = the_register().iter().find(|s| s.request == *request).and_then(|s| s.files.get(at)).cloned();
    if let Some(file) = file {
        record.with_the_record(&mut |connection| {
            let _ = crate::record::dependency_records::dependency_file_moved(connection, request, at, &file);
        });
    }
}

/// **How one file of a send ended.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HowItEnded {
    /// Published at its mirror (`Server protocol and transfers` S26).
    Published,
    /// **`File Browser` P1 and `File Browser` P6a: the machine refused the write for
    /// permission**, with its own words, which are also kept where Transfers'
    /// *What the server said* reads them.
    Refused,
    /// Any other ending that is a failure — the source changed under it
    /// (`File Browser` K8, K9), a digest that disagreed, a folder A7 could not
    /// make.
    Failed,
    /// **Interrupted, stalled, stopped by the user, or never started** — the
    /// record stands for a later carry-on and nothing about the file failed.
    DidNotFinish,
}

/// **One file of a send, as Transfers draws it.**
#[derive(Debug, Clone)]
pub struct AFileSent {
    /// **Its mirror** — `Dependency Upload & Remap` A4 — which is where it
    /// lands and what its row names.
    pub landing: Vec<u8>,
    pub size: u64,
    pub transfer: Option<Identity>,
    pub ended: Option<HowItEnded>,
}

/// **One press of the check's upload, as the run holds it** —
/// `Design/Transfers § The dependency check's own transfers`: one request,
/// named after the `.blend`, whose way back is the check.
///
/// Activity is run-held; `Server protocol and transfers` S0a's start and parent/file outcomes are saved through
/// `record::dependency_records`, so reopening does not invent a start.
#[derive(Debug, Clone)]
pub struct ASend {
    pub request: Identity,
    /// The server row's local identifier.
    pub server: String,
    /// **The check it came from, as the panel keys it** — which is what *Open
    /// the check* hands back.
    pub file: String,
    pub files: Vec<AFileSent>,
}

/// **`Dependency Upload & Remap` A9a: the landing place of every file this
/// run's dependency sends are carrying to one server, running or waiting** —
/// the per-file reading the check's rows, its foot and the upload act all
/// read, from the one register Transfers draws these sends from. **A file
/// whose transfer has ended — published, refused, failed, or stopped — is not
/// in it.** A send an earlier run left is not in the register at all, since
/// nothing in this run moves it.
pub fn the_files_on_their_way_to(server: &str) -> Vec<MachineLocation> {
    the_files_on_their_way_in(&the_register(), server)
}

fn the_files_on_their_way_in(register: &[ASend], server: &str) -> Vec<MachineLocation> {
    register
        .iter()
        .filter(|send| send.server == server)
        .flat_map(|send| send.files.iter())
        .filter(|file| file.ended.is_none())
        .map(|file| MachineLocation::composed(file.landing.clone()))
        .collect()
}

/// **What a row sends, read off its answer** — its one file, the folder whose
/// every file it sends, or its set's folder and pattern. Nothing where the row
/// has no answer.
fn what_the_row_sends(one: &ACheckedRow) -> Option<WhatARowSends<'_>> {
    match &one.the_answer()?.resolution.as_ref()?.to {
        ResolvedTo::AWholeLocation(location) => Some(WhatARowSends::TheLocation(location)),
        ResolvedTo::AFolderForASetRow(folder) => Some(WhatARowSends::TheSetIn {
            folder,
            pattern: one.wanted.first().and_then(|wanted| wanted.the_sets_pattern())?,
        }),
    }
}

/// **`Dependency Upload & Remap` A9a: whether any file a row sends is already
/// on its way to the check's server** — which the panel draws as *Uploading*
/// and the upload act leaves out. Only a row a send can carry — *found on
/// your computer*, or *could not be sent* and tried again — and never where
/// the check has no landing place on that server.
pub fn the_row_is_on_its_way(
    one: &ACheckedRow,
    layout: Option<&smoozy_job_files::layout::TheLayout>,
    on_its_way: &[MachineLocation],
) -> bool {
    if on_its_way.is_empty()
        || !matches!(one.state, RowState::FoundOnYourComputer | RowState::CouldNotBeSent)
    {
        return false;
    }
    match (layout, what_the_row_sends(one)) {
        (Some(layout), Some(row)) => a_row_is_on_its_way(layout, row, on_its_way),
        _ => false,
    }
}

/// The one register there is.
pub fn the_sends_this_run() -> &'static Mutex<Vec<ASend>> {
    static HELD: OnceLock<Mutex<Vec<ASend>>> = OnceLock::new();
    HELD.get_or_init(|| Mutex::new(Vec::new()))
}

fn the_register() -> std::sync::MutexGuard<'static, Vec<ASend>> {
    the_sends_this_run()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn a_file_of_the_send_moved(request: &Identity, at: usize, change: impl FnOnce(&mut AFileSent)) {
    let mut register = the_register();
    if let Some(file) = register
        .iter_mut()
        .find(|one| one.request == *request)
        .and_then(|one| one.files.get_mut(at))
    {
        change(file);
    }
}

/// **Every transfer record a send's unfinished files name** — what a
/// leftover row of `Design/Transfers § What a past sitting left behind` is
/// made of, read the way `crate::downloading::the_transfers_a_request_left`
/// reads an upload's.
pub(crate) fn the_transfers_a_send_left(
    connection: &rusqlite::Connection,
    request: &str,
) -> Vec<TransferRecord> {
    let mut transfers: Vec<Identity> = the_register()
        .iter()
        .filter(|one| one.request.as_text() == request)
        .flat_map(|one| one.files.iter())
        .filter(|file| file.ended != Some(HowItEnded::Published))
        .filter_map(|file| file.transfer.clone())
        .collect();
    // `Server protocol and transfers` S0a's saved parent still names its unfinished files after reopening.
    if let Ok(mut query) = connection.prepare_cached(
        "SELECT transfer FROM dependency_upload_file WHERE request = ?1 AND ending IS NOT 'published' AND transfer IS NOT NULL") {
        if let Ok(rows) = query.query_map([request], |r| r.get::<_, String>(0)) {
            for text in rows.flatten() {
                if let Some(id) = Identity::read_back(IdentityKind::Transfer, &text) {
                    if !transfers.contains(&id) { transfers.push(id); }
                }
            }
        }
    }
    transfers
        .iter()
        .filter_map(|transfer| {
            crate::record::transfer_records::read(connection, transfer)
                .ok()
                .flatten()
        })
        .collect()
}

// ════════════════════════════════════════════════════════════════════════════
// `Dependency Upload & Remap` Q5 and Q7, over what was read
// ════════════════════════════════════════════════════════════════════════════

/// **The readings Q5 and Q7 turn on, for one file, taken before the judgement**
/// — the local file's content identity and the machine copy's four values.
/// The judgement is the dependency module's own (`contents::already_on_the_machine`,
/// `contents::confirm`) and nothing here re-decides it.
///
/// **And the machine copy's contents, read through on the machine**
/// (`Dependency Upload & Remap` Q3, the 38's piece 7) — *did not complete*
/// where the read-through was not asked, which [`already_on_the_machine`]
/// reads as establishing nothing.
struct WhatWasRead {
    local: Option<ContentIdentity>,
    now: FourValuesNow,
    contents: MachineContents,
}

impl Readings for WhatWasRead {
    fn the_local_files_content_identity(&self, _file: &StoredLocation) -> Option<ContentIdentity> {
        self.local.clone()
    }
    fn the_four_values_now(&self, _machine: &MachineKey, _at: &MachineLocation) -> FourValuesNow {
        self.now.clone()
    }
    fn the_machine_copys_contents(
        &self,
        _machine: &MachineKey,
        _at: &MachineLocation,
    ) -> MachineContents {
        self.contents.clone()
    }
    fn enumerate_folder(&self, _folder: &[u8]) -> FolderListing {
        FolderListing::CouldNotBeRead
    }
    fn enumeration_casing_of(&self, _folder: &[u8]) -> EnumerationCasing {
        EnumerationCasing::IsTheStoredName
    }
    fn the_stored_name_reading(&self, _path: &[u8]) -> StoredNameAnswer {
        StoredNameAnswer::NotThere
    }
    fn the_sources_prefix_digest(&self, _source: &StoredLocation, _up_to: u64) -> Option<Digest> {
        None
    }
}

/// **`Render Settings` Q14 item 2's four values, as the dependency module
/// reads them.** A reading that did not come back claims nothing.
fn as_the_module_reads_it(now: smoozy_render_settings::readings::FourValuesNow) -> FourValuesNow {
    match now {
        smoozy_render_settings::readings::FourValuesNow::Answered(values) => {
            FourValuesNow::Answered(values)
        }
        smoozy_render_settings::readings::FourValuesNow::TheChannelCannotAnswer => {
            FourValuesNow::TheChannelCannotAnswer
        }
    }
}

/// **The placement record that names a mirror under the current
/// installation**, where one stands (`Dependency Upload & Remap` K7).
fn the_placement_at<'a>(
    snapshot: &'a InMemoryRecord,
    machine: &MachineKey,
    mirror: &MachineLocation,
) -> Option<&'a PlacementRecord> {
    snapshot.placements.iter().find(|placement| {
        &placement.mirrored_location == mirror && placement.stands_under(&machine.installation)
    })
}

/// **Whether a location an answer names is still on this computer** — a
/// file or a folder, read without following a link. Bytes this system cannot
/// be asked about are taken as still there, so nothing is said that was not
/// seen.
fn is_still_on_this_computer(at: &StoredLocation) -> bool {
    match a_local_path(at.as_bytes()) {
        Some(path) => std::fs::symlink_metadata(path).is_ok(),
        None => true,
    }
}

/// **Every placement record standing under this machine's installation at the
/// mirror of `at`, or under it** — the one file a whole-location row sent, or
/// every file a folder row sent. **Nothing where the sitting has no layout**,
/// or the location will not fit the machine.
fn the_placements_at_or_under<'a>(
    snapshot: &'a InMemoryRecord,
    of: &TheSittingIsOf,
    at: &StoredLocation,
) -> impl Iterator<Item = &'a PlacementRecord> + 'a {
    let mirror = of.layout.as_ref().and_then(|layout| mirror_of(layout, at).ok());
    let installation = of.machine.installation.clone();
    snapshot.placements.iter().filter(move |placement| {
        let Some(mirror) = mirror.as_ref() else {
            return false;
        };
        let placed = placement.mirrored_location.as_bytes();
        let folder = mirror.as_bytes().strip_suffix(b"/").unwrap_or(mirror.as_bytes());
        let under = placed.len() > folder.len()
            && placed.starts_with(folder)
            && placed[folder.len()] == b'/';
        (placed == mirror.as_bytes() || under) && placement.stands_under(&installation)
    })
}

/// **The one file a whole-location row resolved to, and its mirror** — or
/// nothing for a set row or a row with no resolution. **A linked `.blend` is
/// one file like any other** since the 38's piece 7, when its fixing was
/// wired: it is sent to its mirror and fixed from there (`Dependency Upload &
/// Remap` M13, A3).
pub(crate) fn the_file_and_its_mirror(
    one: &ACheckedRow,
    of: &TheSittingIsOf,
) -> Option<(StoredLocation, MachineLocation)> {
    let layout = of.layout.as_ref()?;
    match &one.the_answer()?.resolution.as_ref()?.to {
        ResolvedTo::AWholeLocation(source) => {
            let mirror = mirror_of(layout, source).ok()?;
            Some((source.clone(), mirror))
        }
        ResolvedTo::AFolderForASetRow(_) => None,
    }
}

/// **The files under a folder row's answer, each with its mirror** — or
/// nothing where the row's answer is not a folder on this computer, or a file
/// under it will not fit the machine.
fn the_folders_files_and_their_mirrors(
    one: &ACheckedRow,
    of: &TheSittingIsOf,
) -> Option<Vec<(StoredLocation, MachineLocation)>> {
    if one.kind == Kind::Library {
        return None;
    }
    let layout = of.layout.as_ref()?;
    let ResolvedTo::AWholeLocation(folder) = &one.the_answer()?.resolution.as_ref()?.to else {
        return None;
    };
    if !is_a_folder_here(folder) {
        return None;
    }
    let files = the_files_under(folder).unwrap_or_default();
    Some(
        files
            .into_iter()
            .filter_map(|source| mirror_of(layout, &source).ok().map(|mirror| (source, mirror)))
            .collect(),
    )
}

/// **The names each mirror folder holds, read once per folder for one pass**
/// — what Q5's read-through asks first, so a file not at its mirror costs one
/// listing of its folder and no worker. `None` for a folder that would not
/// read.
#[derive(Default)]
pub(crate) struct TheMirrorFoldersRead(BTreeMap<Vec<u8>, Option<Vec<Vec<u8>>>>);

/// **`Dependency Upload & Remap` Q5 route 2, the read-through, at one
/// mirror** — *nothing there* where the mirror's folder holds no such name,
/// else the machine's own digest of the copy (Q3, the 38's piece 7). **A name
/// in a listing is never taken for the file** (Q10): it only decides whether
/// the digest is asked for.
async fn the_machine_copy_read_through<M>(
    machine: &mut M,
    folders: &mut TheMirrorFoldersRead,
    mirror: &MachineLocation,
) -> MachineContents
where
    M: TheMachineAFileIsReadOn + Send,
{
    let folder = smoozy_dependencies::bytes::folder_of(mirror.as_bytes());
    let names = match folders.0.get(&folder) {
        Some(names) => names.clone(),
        None => {
            let names = machine.the_names_in(&folder).await;
            folders.0.insert(folder, names.clone());
            names
        }
    };
    let Some(names) = names else {
        return MachineContents::DidNotComplete;
    };
    let name = smoozy_dependencies::bytes::last_segment(mirror.as_bytes());
    if !names.iter().any(|one| one.as_slice() == name) {
        return MachineContents::NothingThere;
    }
    smoozy_dependencies::acting::machine::the_machine_copys_contents(machine, mirror).await
}

/// **`Dependency Upload & Remap` Q5 over one file at its mirror**, both
/// routes: the record's four values where a placement record names the
/// mirror, and **the read-through where they do not establish it** — no
/// record at all, values that moved, or a channel that cannot answer (the
/// 38's piece 7; until then this asked the first route alone and only where a
/// record stood).
async fn what_q5_establishes<M>(
    machine: &mut M,
    of: &TheSittingIsOf,
    snapshot: &InMemoryRecord,
    folders: &mut TheMirrorFoldersRead,
    source: &StoredLocation,
    mirror: &MachineLocation,
) -> Establishment
where
    M: TheMachineAFileIsReadOn + Send,
{
    let local = smoozy_dependencies::adapter::TheUsersOwnDisk.the_local_files_content_identity(source);
    if local.is_none() {
        return Establishment::TheLocalFileCouldNotBeRead;
    }
    let now = match (
        the_placement_at(snapshot, &of.machine, mirror).is_some(),
        std::str::from_utf8(mirror.as_bytes()),
    ) {
        (true, Ok(mirror_text)) => as_the_module_reads_it(machine.the_four_values_of(mirror_text).await),
        _ => FourValuesNow::TheChannelCannotAnswer,
    };
    let by_record = already_on_the_machine(
        source,
        &of.machine,
        mirror,
        &of.machine.installation,
        snapshot,
        &WhatWasRead {
            local: local.clone(),
            now: now.clone(),
            contents: MachineContents::DidNotComplete,
        },
    );
    if !matches!(by_record, Establishment::NotEstablished) {
        return by_record;
    }
    let contents = the_machine_copy_read_through(machine, folders, mirror).await;
    already_on_the_machine(
        source,
        &of.machine,
        mirror,
        &of.machine.installation,
        snapshot,
        &WhatWasRead { local, now, contents },
    )
}

/// **`Dependency Upload & Remap` Q5's first route at the check** — so a file
/// this module already placed reads *already on the server* and the upload
/// control does not offer it again, and **`Dependency Upload & Remap` Q7's
/// state**, *uploaded before, not confirmed*, where the record says it was sent
/// and the local original is no longer there to compare.
///
/// **Called by `crate::checking_a_file` over each reading's rows, before they
/// reach the panel and before the standing is written**, so the standing
/// counts them confirmed (`Dependency Upload & Remap` V11). **Only a row a
/// placement record names is asked about**, so a file never sent costs the
/// machine nothing.
///
/// **The case the check was first written for** is a machine nothing had been
/// sent to, so every found row was *found on your computer*. **The case it now
/// also meets** is the same file checked a second time after its found files
/// went up.
pub async fn what_is_already_on_the_server<M>(
    machine: &mut M,
    record: &dyn TheCheckReachesTheRecord,
    of: &TheSittingIsOf,
    rows: &mut [ACheckedRow],
) where
    M: TheMachineAFileIsReadOn + Send,
{
    let snapshot = the_snapshot(record, &of.machine);
    // **A row whose answer is no longer on this computer is never *found on
    // your computer***. Sent here before, it reads *uploaded before, not
    // confirmed* (`Dependency Upload & Remap` Q7, and the spec: *"That state is
    // what covers a file the user has moved or deleted on their own
    // computer"*); never sent, nothing is found, so it reads *not found*.
    // **The record's answer is not touched** — it says where the file lived
    // (`Dependency Upload & Remap` K11), and only the row says what is there
    // now. The case this was first written for was a file still in place.
    // The case it now also meets is a cache folder moved away after the
    // check found it: the pass before 0.1.9 on Vast, new fault 5, read the
    // old place as found twice, and only the send said otherwise.
    for one in rows.iter_mut() {
        if one.state != RowState::FoundOnYourComputer {
            continue;
        }
        let Some(at) = one
            .the_answer()
            .and_then(|answer| answer.resolution.as_ref())
            .map(|resolution| resolution.to.location().clone())
        else {
            continue;
        };
        if is_still_on_this_computer(&at) {
            continue;
        }
        let sent_here = snapshot.as_ref().is_some_and(|snapshot| {
            the_placements_at_or_under(snapshot, of, &at).next().is_some()
        });
        one.state = if sent_here { RowState::SentBeforeNotConfirmed } else { RowState::NotFound };
        one.state_when_the_check_ran = one.state;
    }
    let Some(snapshot) = snapshot else {
        return;
    };
    let mut folders = TheMirrorFoldersRead::default();
    for one in rows.iter_mut() {
        if one.state != RowState::FoundOnYourComputer {
            continue;
        }
        // **A folder row is on the server where every file under it is**,
        // each established as one file is. After 0.1.8, the checks'
        // leftovers, item 1: a folder sent whole is read so on the next
        // check, and the warning it stopped does not come back.
        if let Some(files) = the_folders_files_and_their_mirrors(one, of) {
            let mut every_one = !files.is_empty();
            for (source, mirror) in files {
                if !every_one {
                    break;
                }
                every_one = matches!(
                    what_q5_establishes(machine, of, &snapshot, &mut folders, &source, &mirror).await,
                    Establishment::AlreadyOnTheMachine(_)
                );
            }
            if every_one {
                one.state = RowState::AlreadyOnTheMachine;
                one.state_when_the_check_ran = one.state;
            }
            continue;
        }
        let Some((source, mirror)) = the_file_and_its_mirror(one, of) else {
            continue;
        };
        if the_placement_at(&snapshot, &of.machine, &mirror).is_some()
            && smoozy_dependencies::adapter::TheUsersOwnDisk
                .the_local_files_content_identity(&source)
                .is_none()
        {
            // **`Dependency Upload & Remap` Q7: the record says a file was sent
            // for this reference to this machine and the local original is no
            // longer there to compare.**
            one.state = RowState::SentBeforeNotConfirmed;
            one.state_when_the_check_ran = one.state;
            continue;
        }
        // **Both routes** — the record's, and the machine's own read-through
        // where the record does not establish it (`Dependency Upload & Remap`
        // Q5; the read-through since the 38's piece 7). **A file at its mirror
        // that no record names is *already on the server* where its contents
        // are the same**: a record lost, another computer, or a copy put there
        // through Files.
        if let Establishment::AlreadyOnTheMachine(_) =
            what_q5_establishes(machine, of, &snapshot, &mut folders, &source, &mirror).await
        {
            one.state = RowState::AlreadyOnTheMachine;
            one.state_when_the_check_ran = one.state;
        }
    }
}

/// **The keeper's snapshot of one machine's records**, or nothing where the
/// record would not read — which asks nothing of the machine rather than
/// reading an unread record as *nothing was placed* (L8a).
fn the_snapshot<R: TheCheckReachesTheRecord + ?Sized>(
    record: &R,
    machine: &MachineKey,
) -> Option<InMemoryRecord> {
    let mut snapshot = None;
    record.with_the_record(&mut |connection| {
        snapshot = TheDependencyRecord::over(connection)
            .what_it_holds_for(machine)
            .ok();
    });
    snapshot
}

/// **`Dependency Upload & Remap` Q7, the row's Confirm** — the four-value fast
/// path against the placement record; where the file is absent, back to *not
/// found* and the placement record marked gone; the outcome written into the
/// placement record with its moment. **Where the four values moved, the copy
/// is read through on the machine** against the recorded content identity
/// (the 38's piece 7; until then such a row was *not confirmed* and stayed as
/// it was).
pub async fn a_row_is_confirmed<C: AHeldConnection, R: TheCheckReachesTheRecord + ?Sized>(
    sessions: &ThisServersSessions<C>,
    record: &R,
    held: &Mutex<WhatTheRunHolds>,
    of: &TheSittingIsOf,
    at: usize,
) -> bool {
    let key = keyed(&of.server, &of.file);
    // **The place the row's answer names**, a file or a folder. A folder row
    // is confirmed file by file, every placement under its mirror, the way a
    // folder row is read *already on the server* (after 0.1.8, the checks'
    // leftovers, item 1). The case this was first written for was one file;
    // the case it now also meets is a cache folder moved away after it went
    // up (the pass before 0.1.9 on Vast, new fault 5), which until then had
    // no Confirm that did anything.
    let location = {
        let Ok(held) = held.lock() else {
            return false;
        };
        let Some(one) = held
            .sittings
            .get(&key)
            .and_then(|sitting| sitting.rows.get(at))
        else {
            return false;
        };
        if one.state != RowState::SentBeforeNotConfirmed {
            return false;
        }
        match one.the_answer().and_then(|answer| answer.resolution.as_ref()) {
            Some(resolution) => resolution.to.location().clone(),
            None => return false,
        }
    };
    let Some(snapshot) = the_snapshot(record, &of.machine) else {
        return false;
    };
    let placements: Vec<PlacementRecord> =
        the_placements_at_or_under(&snapshot, of, &location).cloned().collect();
    if placements.is_empty() {
        return false;
    }
    let mut outcome = ConfirmationOutcome::Confirmed;
    for placement in placements {
        let now = the_four_values_at(sessions, &of.server, &placement.mirrored_location).await;
        let mut this_one = confirm(
            &placement,
            &WhatWasRead {
                local: None,
                now: now.clone(),
                contents: MachineContents::DidNotComplete,
            },
        );
        if this_one == ConfirmationOutcome::NotConfirmed {
            if let Some(layout) = of.layout.as_ref() {
                let package_area = layout.area(smoozy_job_files::layout::Area::Package);
                let mut machine = crate::reading_the_file::ThisServersMachine {
                    sessions,
                    package_area: &package_area,
                    server: &of.server,
                };
                let contents = smoozy_dependencies::acting::machine::the_machine_copys_contents(
                    &mut machine,
                    &placement.mirrored_location,
                )
                .await;
                this_one = confirm(&placement, &WhatWasRead { local: None, now, contents });
            }
        }
        let placement = placement.confirmed(this_one, SystemTime::now());
        let mut written = false;
        record.with_the_record(&mut |connection| {
            written = TheDependencyRecord::over(connection)
                .a_placement_is_written(&placement)
                .is_ok();
        });
        if !written {
            return false;
        }
        // **One copy gone is the row gone; one not confirmed leaves the row
        // as it was** — Q7's three outcomes over the set.
        outcome = match (outcome, this_one) {
            (ConfirmationOutcome::Gone, _) | (_, ConfirmationOutcome::Gone) => ConfirmationOutcome::Gone,
            (ConfirmationOutcome::NotConfirmed, _) | (_, ConfirmationOutcome::NotConfirmed) => {
                ConfirmationOutcome::NotConfirmed
            }
            _ => ConfirmationOutcome::Confirmed,
        };
    }
    if let Ok(mut held) = held.lock() {
        if let Some(one) = held
            .sittings
            .get_mut(&key)
            .and_then(|sitting| sitting.rows.get_mut(at))
        {
            match outcome {
                ConfirmationOutcome::Confirmed => one.state = RowState::AlreadyOnTheMachine,
                ConfirmationOutcome::Gone => one.state = RowState::NotFound,
                ConfirmationOutcome::NotConfirmed => {}
            }
        }
    }
    the_standing_is_brought_up_to_date(record, held, of);
    true
}

/// **The four values of a machine file, and *nothing there* where the file
/// transfer surface says so** — the command-channel reading first
/// (`Dependency Upload & Remap` Q5 names the channel), and where it cannot
/// answer, the one answer the file-transfer surface gives without doubt:
/// no such file.
async fn the_four_values_at<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    server: &str,
    at: &MachineLocation,
) -> FourValuesNow {
    let Ok(text) = std::str::from_utf8(at.as_bytes()) else {
        return FourValuesNow::TheChannelCannotAnswer;
    };
    // **In its turn at the one link site on this server, among every reader
    // in the application and not only the sends** (step 10c's eleventh-pass
    // fix piece 4b) — the turn is the reading's own, in `Render Settings`
    // (`smoozy_render_settings::acting::four_values::read_the_four_values`).
    // Piece 4 held one here among the sends alone, and a submission or the
    // page reading beside a send still met it at the site.
    match as_the_module_reads_it(
        crate::submitting::the_four_values_of(sessions, server, text).await,
    ) {
        FourValuesNow::Answered(values) => FourValuesNow::Answered(values),
        _ => {
            let path = at.as_bytes().to_vec();
            match sessions
                .a_reading(true, move |subsystem| {
                    Box::pin(async move { Ok(subsystem.stat(&path).await) })
                })
                .await
            {
                Ok(Ok(Err(SubsystemFailure::TheServerRefused(StatusCode::NoSuchFile)))) => {
                    FourValuesNow::NothingThere
                }
                _ => FourValuesNow::TheChannelCannotAnswer,
            }
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// `Dependency Upload & Remap` M13 to `Dependency Upload & Remap` M16a — a linked `.blend` fixed
// ════════════════════════════════════════════════════════════════════════════

/// **The pairs a linked `.blend`'s fixing writes, and what the fixing points
/// at** — M10's composition run over the library's own located answers, with
/// the library as the top file (M13: *"a fixed copy's own library references
/// point at other fixed copies; every other reference in a fixed copy points
/// at mirrors"*). Nothing where the record would not read.
fn the_pairs_a_library_is_fixed_with<R: TheCheckReachesTheRecord + ?Sized>(
    record: &R,
    of: &TheSittingIsOf,
    source: &StoredLocation,
    snapshot: &InMemoryRecord,
) -> Option<Vec<smoozy_dependencies::remap::Pair>> {
    let layout = of.layout.as_ref()?;
    let holding = smoozy_dependencies::record::HoldingFile::OnTheUsersComputer(source.clone());
    let mut answers = None;
    record.with_the_record(&mut |connection| {
        answers = TheDependencyRecord::over(connection).the_answers_of(&holding).ok();
    });
    let (to_compose, _) = smoozy_dependencies::remap::the_answers_to_compose(answers?);
    let fixings: Vec<smoozy_dependencies::record::FixingRecord> = snapshot
        .fixings
        .iter()
        .filter(|fixing| fixing.machine.is_current_under(&of.machine.installation))
        .cloned()
        .collect();
    Some(smoozy_dependencies::remap::compose_the_pairs(layout, &holding, &to_compose, &fixings).pairs)
}

fn what_it_points_at(
    pairs: &[smoozy_dependencies::remap::Pair],
) -> Vec<(smoozy_dependencies::bytes::RecordedLocation, MachineLocation)> {
    pairs
        .iter()
        .map(|pair| (pair.expected.clone(), pair.new_location.clone()))
        .collect()
}

/// **M13 and M14 for one linked `.blend` this module placed at its mirror** —
/// its fixed copy produced where none stands, or where M16's second trigger
/// (the located answers moved) or `Dependency Upload & Remap` M16b's third (the copy it is derived from
/// was published over) owes one again; **and, for `Dependency Upload & Remap` M16a's *Try again*,
/// whatever stands**. The copy is staged beside A3's address, produced by the
/// machine's file-inspection worker, published by one rename, and K8 written
/// at publication. The 38's piece 7.
///
/// **What it is derived from is the mirror**, with the placement record's
/// content identity (K7) — the copy this module sent. **If the placement
/// reading was lost after upload, `Dependency Upload & Remap` Q5's read-through supplies the content
/// again**. No placement is invented from presence alone.
pub async fn a_library_is_fixed<M, R>(
    machine: &mut M,
    record: &R,
    of: &TheSittingIsOf,
    row: &ACheckedRow,
    even_where_none_is_owed: bool,
) -> Result<(), smoozy_dependencies::fixing::WhyItCouldNotBeFixed>
where
    M: TheMachineAFileIsReadOn + Send,
    R: TheCheckReachesTheRecord + ?Sized,
{
    use smoozy_dependencies::fixing::{re_production_is_owed, WhyItCouldNotBeFixed};
    let did_not = WhyItCouldNotBeFixed::TheProductionDidNotPublish;
    let layout = of.layout.as_ref().ok_or(did_not)?;
    let (source, mirror) =
        the_file_and_its_mirror(row, of).ok_or(WhyItCouldNotBeFixed::TheMachineCopyIsNoLongerThere)?;
    let snapshot = the_snapshot(record, &of.machine).ok_or(did_not)?;
    let content = match the_placement_at(&snapshot, &of.machine, &mirror) {
        Some(placement) => placement.content.clone(),
        None => match smoozy_dependencies::acting::machine::the_machine_copys_contents(machine, &mirror).await {
            MachineContents::Read(content) => content,
            MachineContents::NothingThere | MachineContents::DidNotComplete => {
                return Err(WhyItCouldNotBeFixed::TheMachineCopyIsNoLongerThere);
            }
        },
    };
    // **A3: the address, off the machine copy's own path** — refused, never
    // truncated, where it will not fit.
    let address = smoozy_dependencies::landing::fixed_copy_address(layout, &mirror)
        .map_err(WhyItCouldNotBeFixed::TheAddressWillNotFit)?;
    let pairs = the_pairs_a_library_is_fixed_with(record, of, &source, &snapshot).ok_or(did_not)?;
    let points_at = what_it_points_at(&pairs);
    if !even_where_none_is_owed {
        let standing = snapshot.fixings.iter().find(|fixing| {
            fixing.derived_from == mirror
                && fixing.fixed_copy == address
                && fixing.machine.is_current_under(&of.machine.installation)
        });
        if let Some(standing) = standing {
            if re_production_is_owed(standing, &points_at, Some(&content)).is_none() {
                return Ok(());
            }
        }
    }
    // **Staged beside the address**, in `File Browser` X12's reserved folder,
    // so the publication's rename does not cross a mount — and under a name of
    // its own, so two productions never share one.
    let operation = Identity::issue(IdentityKind::Operation).map_err(|_| did_not)?;
    let mut staging = smoozy_dependencies::bytes::folder_of(address.as_bytes());
    staging.push(b'/');
    staging.extend_from_slice(THE_RESERVED_STAGING_FOLDER.as_bytes());
    staging.push(b'/');
    staging.extend_from_slice(smoozy_dependencies::bytes::last_segment(address.as_bytes()));
    staging.extend_from_slice(format!(".{}.fixing", operation.as_text()).as_bytes());
    let staging = MachineLocation::composed(staging);
    let fixing = smoozy_dependencies::acting::fixing_worker::OneFixing::of(
        &mirror, &address, &staging, &of.with, pairs,
    )
    .ok_or(did_not)?;
    if !machine.the_folders_above_are_made(staging.as_bytes()).await {
        return Err(did_not);
    }
    if let Err(why) = smoozy_dependencies::acting::fixing_worker::produce_one_fixed_copy(machine, &fixing).await {
        machine.the_staged_copy_is_removed(staging.as_bytes()).await;
        return Err(why);
    }
    match machine.the_fixed_copy_is_published(&fixing).await {
        smoozy_dependencies::acting::fixing_worker::HowTheFixingWent::Published => {}
        smoozy_dependencies::acting::fixing_worker::HowTheFixingWent::ItCouldNotBeFixed(why) => {
            return Err(why)
        }
    }
    // **K8, written at publication**, with what the fixing points at.
    let written = smoozy_dependencies::record::FixingRecord {
        machine: of.machine.clone(),
        derived_from: mirror,
        derived_from_content: content,
        fixed_copy: address,
        points_at,
    };
    let mut kept = false;
    record.with_the_record(&mut |connection| {
        kept = TheDependencyRecord::over(connection)
            .a_fixing_is_written(&written)
            .is_ok();
    });
    if kept {
        Ok(())
    } else {
        Err(did_not)
    }
}

/// **V11 for the rows inside one linked `.blend`**, from the record alone: the
/// file's fixing record matches the current located answers (M16's second
/// trigger has not fired), and no `Dependency Upload & Remap` M16b re-production is owed. **No fixing
/// record is neither**, which is what every such row was written as until the
/// 38's piece 7.
pub fn the_fixing_status_of<R: TheCheckReachesTheRecord + ?Sized>(
    record: &R,
    of: &TheSittingIsOf,
    holding: &smoozy_dependencies::record::HoldingFile,
) -> FixingStatus {
    let none = FixingStatus {
        the_fixing_matches_the_current_located_answers: false,
        a_re_production_is_owed: true,
    };
    let (Some(layout), smoozy_dependencies::record::HoldingFile::OnTheUsersComputer(source)) =
        (of.layout.as_ref(), holding)
    else {
        return none;
    };
    let Ok(mirror) = mirror_of(layout, source) else {
        return none;
    };
    let Some(snapshot) = the_snapshot(record, &of.machine) else {
        return none;
    };
    let Some(fixing) = snapshot.fixings.iter().find(|fixing| {
        fixing.derived_from == mirror && fixing.machine.is_current_under(&of.machine.installation)
    }) else {
        return none;
    };
    let Some(pairs) = the_pairs_a_library_is_fixed_with(record, of, source, &snapshot) else {
        return none;
    };
    let last = the_placement_at(&snapshot, &of.machine, &mirror).map(|placement| placement.content.clone());
    FixingStatus {
        the_fixing_matches_the_current_located_answers:
            !smoozy_dependencies::fixing::the_located_answers_moved(fixing, &what_it_points_at(&pairs)),
        a_re_production_is_owed: smoozy_dependencies::fixing::the_machine_copy_was_published_over(
            fixing,
            last.as_ref(),
        ),
    }
}

/// **A row's *could not be fixed* reason, as the row says it** — the address
/// that will not fit is the path too long; a copy whose pairs did not all
/// apply could not have its paths remapped; anything else, Smoozy could not
/// open it to remap them.
pub fn why_it_did_not_go_up(why: smoozy_dependencies::fixing::WhyItCouldNotBeFixed) -> WhyItDidNotGoUp {
    use smoozy_dependencies::fixing::WhyItCouldNotBeFixed;
    match why {
        WhyItCouldNotBeFixed::TheAddressWillNotFit(_) => WhyItDidNotGoUp::ThePathIsTooLong,
        WhyItCouldNotBeFixed::TheReportSaysThePairsDidNotAllApply { .. } => {
            WhyItDidNotGoUp::SmoozyCouldNotRemapItsPaths
        }
        WhyItCouldNotBeFixed::TheMachineCopyIsNoLongerThere
        | WhyItCouldNotBeFixed::TheProductionDidNotPublish => {
            WhyItDidNotGoUp::SmoozyCouldNotOpenItToRemapItsPaths
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// The send
// ════════════════════════════════════════════════════════════════════════════

/// **What one press sends** — the check it came from, the application run,
/// and the rows.
#[derive(Debug, Clone)]
pub struct WhatIsSent {
    pub of: TheSittingIsOf,
    /// **This application run**, which `Server protocol and transfers` C11
    /// step 4 reads before a record is carried on.
    pub run: Identity,
    /// The rows, by their place on the panel.
    pub rows: Vec<usize>,
    /// **`Dependency Upload & Remap` Q8 answered *Replace* for these rows.**
    pub replacing: bool,
}

/// **What a send came to.**
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WhatTheSendCameTo {
    /// The request Transfers shows, where anything was sent at all.
    pub request: Option<Identity>,
    pub published: usize,
    /// Files `Dependency Upload & Remap` Q5 established already on the server,
    /// which were not sent.
    pub already_there: usize,
    pub failed: usize,
    pub did_not_finish: usize,
    /// **The row the overwrite box now asks about**, where Q8 fired.
    pub asks_to_replace: Option<usize>,
    /// Legacy closed-act result; the free-software send does not set it.
    pub the_lock_closed_it: bool,
    /// **Linked `.blend`s this send published** — each a file nobody has
    /// read below yet, which the check reads next (`Dependency Upload & Remap`
    /// G14). The 38's piece 7.
    pub libraries_published: usize,
}

/// **The checks a send is running for**, held while it runs.
struct ASendRuns(String);

fn the_checks_sending() -> std::sync::MutexGuard<'static, std::collections::BTreeSet<String>> {
    static RUNNING: OnceLock<Mutex<std::collections::BTreeSet<String>>> = OnceLock::new();
    RUNNING
        .get_or_init(Default::default)
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

impl ASendRuns {
    fn for_the_check(key: &str) -> Option<ASendRuns> {
        the_checks_sending()
            .insert(key.to_string())
            .then(|| ASendRuns(key.to_string()))
    }
}

impl Drop for ASendRuns {
    fn drop(&mut self) {
        the_checks_sending().remove(&self.0);
    }
}

/// **A send let go of before its files have ended has ended**, and its files
/// with it: each one not yet ended is marked *did not finish* in the run's
/// register as the send is let go. **Without this, a send cut short inside a
/// run — its task ended, or a channel that panicked — would read as on its way
/// for the rest of the run** (`Dependency Upload & Remap` A9a), and no press
/// would ever send its files again. A send that ran to its end has nothing
/// left to mark. Only the run's memory is changed; the record keeps what each
/// file's own save wrote, as quitting leaves it.
struct TheSendEndsIfLetGo(Identity);

impl Drop for TheSendEndsIfLetGo {
    fn drop(&mut self) {
        for send in the_register().iter_mut().filter(|send| send.request == self.0) {
            for file in send.files.iter_mut().filter(|file| file.ended.is_none()) {
                file.ended = Some(HowItEnded::DidNotFinish);
            }
        }
    }
}

/// One file the pre-flight sends.
struct AFileToSend {
    row: usize,
    source: StoredLocation,
    mirror: MachineLocation,
    size: u64,
    /// **A linked `.blend`**, which the check reads below once it lands.
    a_library: bool,
}

/// What one row is, as the pre-flight reads it.
enum TheRowIs {
    /// Every file of it established already on the server.
    AlreadyThere,
    /// These files go.
    Sent(Vec<AFileToSend>),
    /// `Dependency Upload & Remap` Q8 fired.
    AskedToReplace(AnOverwrite),
    /// It could not be sent, for this reason.
    CouldNotBeSent(WhyItDidNotGoUp),
    /// Nothing of it is this send's.
    NotSent,
}

/// **The rows a press of the upload control sends** — every row *found on
/// your computer*, a linked `.blend` among them since the 38's piece 7 (this
/// file's head).
pub fn the_rows_the_upload_sends(rows: &[ACheckedRow]) -> Vec<usize> {
    rows.iter()
        .enumerate()
        .filter(|(_, one)| one.state == RowState::FoundOnYourComputer)
        .map(|(at, _)| at)
        .collect()
}

/// **`Dependency Upload & Remap` A9a: the rows the upload control counts** —
/// every row [`the_rows_the_upload_sends`] would take, less every row whose
/// file is already on its way (`Design/Dependency Upload & Remap § Along the
/// bottom`: *"It counts and sends only the rows not already on their way"*).
/// **What a press then sends is decided again at the press** from the same
/// reading ([`the_found_files_go_up`]), never from what the panel last drew.
pub fn the_rows_left_to_upload(
    rows: &[ACheckedRow],
    layout: Option<&smoozy_job_files::layout::TheLayout>,
    on_its_way: &[MachineLocation],
) -> Vec<usize> {
    the_rows_the_upload_sends(rows)
        .into_iter()
        .filter(|at| !the_row_is_on_its_way(&rows[*at], layout, on_its_way))
        .collect()
}

/// **The pre-flight over one row.** `Dependency Upload & Remap` §10: one set
/// row's sending is partial across members, so each member is its own file.
#[allow(clippy::too_many_arguments)]
async fn the_pre_flight<C: AHeldConnection>(
    one: &ACheckedRow,
    at: usize,
    asked: &WhatIsSent,
    snapshot: &InMemoryRecord,
    holds: Option<&crate::record::neighbouring_records::WhatTheLocalRecordHolds>,
    sessions: &ThisServersSessions<C>,
    folders: &mut TheMirrorFoldersRead,
) -> TheRowIs {
    let of = &asked.of;
    let Some(layout) = of.layout.as_ref() else {
        return TheRowIs::NotSent;
    };
    let wanted = matches!(
        one.state,
        RowState::FoundOnYourComputer | RowState::CouldNotBeSent
    );
    if !wanted {
        return TheRowIs::NotSent;
    }
    let package_area = layout.area(smoozy_job_files::layout::Area::Package);
    let Some(resolution) = one
        .the_answer()
        .and_then(|answer| answer.resolution.clone())
    else {
        return TheRowIs::NotSent;
    };
    // **The files the row stands for** — its one file, or a set's members as
    // they stand in the folder now (`Dependency Upload & Remap` G22,
    // `Dependency Upload & Remap` F28).
    let (sources, the_set): (
        Vec<StoredLocation>,
        Option<(StoredLocation, Vec<u8>, Vec<Vec<u8>>)>,
    ) = match &resolution.to {
        // **A folder-shaped row's answer is a folder** (`Dependency Upload &
        // Remap` F28) — a cache or a bake folder — **and every file under it
        // goes, each to its own mirror** (`Dependency Upload & Remap` A4), so
        // the folder stands whole at the folder's mirror, where the machine's
        // copy reads it (`Dependency Upload & Remap` M22c). After 0.1.8, the
        // checks' leftovers, item 1: until then the folder was read as one
        // file, and every cache folder the check found failed to send with
        // *Smoozy could not read this file on your computer*.
        ResolvedTo::AWholeLocation(source) if is_a_folder_here(source) => {
            match the_files_under(source) {
                Some(files) if !files.is_empty() => (files, None),
                Some(_) => return TheRowIs::NotSent,
                None => return TheRowIs::CouldNotBeSent(WhyItDidNotGoUp::ItCouldNotBeReadHere),
            }
        }
        ResolvedTo::AWholeLocation(source) => (vec![source.clone()], None),
        ResolvedTo::AFolderForASetRow(folder) => {
            let Some(pattern) = one
                .wanted
                .first()
                .and_then(|wanted| wanted.the_sets_pattern())
            else {
                return TheRowIs::CouldNotBeSent(WhyItDidNotGoUp::ItCouldNotBeReadHere);
            };
            let Some(members) = smoozy_dependencies::search::the_sets_members_in(
                folder,
                pattern,
                &TheUsersOwnComputer,
            ) else {
                return TheRowIs::CouldNotBeSent(WhyItDidNotGoUp::ItCouldNotBeReadHere);
            };
            (
                members
                    .iter()
                    .map(|member| {
                        StoredLocation::as_the_system_returned_it(&join(folder.as_bytes(), member))
                    })
                    .collect(),
                Some((folder.clone(), pattern.to_vec(), members)),
            )
        }
    };

    let mut to_send = Vec::new();
    let mut differ = Vec::new();
    for source in sources {
        // **`Dependency Upload & Remap` A4 and A5: the mirror, and both byte
        // checks, before anything is arranged.**
        let mirror = match mirror_of(layout, &source) {
            Ok(mirror) => mirror,
            Err(CouldNotBeSent::TheMirrorWillNotFit(_)) => {
                return TheRowIs::CouldNotBeSent(WhyItDidNotGoUp::ThePathIsTooLong)
            }
            Err(CouldNotBeSent::TheLocationsFormIsNotOneTheLandingRuleCovers) => {
                return TheRowIs::CouldNotBeSent(WhyItDidNotGoUp::TheUploadFailed)
            }
        };
        let local =
            smoozy_dependencies::adapter::TheUsersOwnDisk.the_local_files_content_identity(&source);
        let Some(size) = local.as_ref().map(|local| local.size) else {
            return TheRowIs::CouldNotBeSent(WhyItDidNotGoUp::ItCouldNotBeReadHere);
        };
        // **`Dependency Upload & Remap` Q5, both routes** — the record's
        // where a placement record names the mirror, and the read-through on
        // the machine where it does not establish it (the 38's piece 7: until
        // then a file at its mirror that no record named was sent again, and a
        // different file there was written over without the question).
        let mut machine = crate::reading_the_file::ThisServersMachine {
            sessions,
            package_area: &package_area,
            server: &of.server,
        };
        match what_q5_establishes(&mut machine, of, snapshot, folders, &source, &mirror).await {
            Establishment::AlreadyOnTheMachine(_) => continue,
            // **`Dependency Upload & Remap` Q8: a different file is already
            // at that location.**
            Establishment::NotAlreadyThere {
                occupied_by_a_different_file: true,
            } if !asked.replacing => {
                differ.push(mirror.clone());
                continue;
            }
            _ => {}
        }
        to_send.push(AFileToSend {
            row: at,
            source,
            mirror,
            size,
            a_library: one.kind == Kind::Library,
        });
    }

    if let Some(first) = differ.first() {
        // **Over a set row the question widens to the folder** — the members
        // a send would write over, and the members the mirrored folder holds
        // that the local set does not (`Dependency Upload & Remap` Q5's
        // membership reading, Q8).
        let the_folder = match &the_set {
            Some((folder, pattern, members)) => {
                let mirrored = mirror_of(layout, folder).ok();
                let listed = match mirrored {
                    Some(mirrored) => {
                        let path = mirrored.as_bytes().to_vec();
                        sessions
                            .a_reading(true, move |subsystem| {
                                Box::pin(async move { Ok(subsystem.list(&path).await) })
                            })
                            .await
                            .ok()
                            .and_then(|read| read.ok())
                            .and_then(|read| read.ok())
                            .map(|entries| {
                                entries
                                    .into_iter()
                                    .map(|entry| entry.name)
                                    .collect::<Vec<_>>()
                            })
                    }
                    None => None,
                };
                let only_on_the_server = match listed {
                    Some(listed) => match set_row_membership(
                        pattern,
                        members,
                        &MirroredFolderListing { members: listed },
                    ) {
                        Membership::Equal => 0,
                        Membership::Differs {
                            surplus_on_the_machine,
                            ..
                        } => surplus_on_the_machine.len(),
                    },
                    None => 0,
                };
                Some(TheFolderDiffers {
                    differ: differ.len(),
                    members: members.len(),
                    only_on_the_server,
                })
            }
            None => None,
        };
        let location = match &the_set {
            Some((folder, _, _)) => mirror_of(layout, folder)
                .map(|mirror| mirror.as_bytes().to_vec())
                .unwrap_or_else(|_| first.as_bytes().to_vec()),
            None => first.as_bytes().to_vec(),
        };
        // **`File Browser` W5's live jobs over the files a *Replace* would
        // write over**, read from the local record through the same
        // composition every box in Files reads. Until step 10c's Files' owed
        // acts piece 1 this box was built with no jobs at all.
        let (jobs, labels) = match holds {
            Some(holds) => (
                smoozy_file_browser::warnings::the_live_job_warning(
                    &differ
                        .iter()
                        .map(|mirror| mirror.as_bytes().to_vec())
                        .collect::<Vec<_>>(),
                    &holds.knows(),
                ),
                holds.the_labels(),
            ),
            None => (Vec::new(), Vec::new()),
        };
        return TheRowIs::AskedToReplace(AnOverwrite {
            row: at,
            location,
            jobs,
            labels,
            the_folder,
        });
    }
    if to_send.is_empty() {
        TheRowIs::AlreadyThere
    } else {
        TheRowIs::Sent(to_send)
    }
}

/// **Whether a row's answer is a folder on this computer**, which only a
/// folder-shaped kind's can be (`Dependency Upload & Remap` F28).
fn is_a_folder_here(at: &StoredLocation) -> bool {
    a_local_path(at.as_bytes()).is_some_and(|path| path.is_dir())
}

/// **Every file under a folder on this computer, subfolders included**, in a
/// fixed order — or nothing where the folder, or a folder under it, will not
/// read: a folder sent part-way would stand on the machine as a cache with
/// files missing. **A link is not followed** and nothing outside the folder
/// goes. **Smoozy's own staging folder is stepped over** (`File Browser`
/// X12's reserved name), as both download walks step over it.
fn the_files_under(folder: &StoredLocation) -> Option<Vec<StoredLocation>> {
    fn walk(path: &std::path::Path, into: &mut Vec<std::path::PathBuf>) -> Option<()> {
        let mut entries: Vec<std::fs::DirEntry> =
            std::fs::read_dir(path).ok()?.collect::<Result<_, _>>().ok()?;
        entries.sort_by_key(|entry| entry.file_name());
        for entry in entries {
            if entry.file_name() == THE_RESERVED_STAGING_FOLDER {
                continue;
            }
            let kind = entry.file_type().ok()?;
            if kind.is_dir() {
                walk(&entry.path(), into)?;
            } else if kind.is_file() {
                into.push(entry.path());
            }
        }
        Some(())
    }
    let mut found = Vec::new();
    walk(&a_local_path(folder.as_bytes())?, &mut found)?;
    Some(
        found
            .iter()
            .filter_map(|path| path_bytes(path))
            .map(|bytes| StoredLocation::as_the_system_returned_it(&bytes))
            .collect(),
    )
}

#[cfg(unix)]
fn a_local_path(bytes: &[u8]) -> Option<std::path::PathBuf> {
    use std::os::unix::ffi::OsStrExt;
    Some(std::path::PathBuf::from(std::ffi::OsStr::from_bytes(bytes)))
}

#[cfg(not(unix))]
fn a_local_path(bytes: &[u8]) -> Option<std::path::PathBuf> {
    Some(std::path::PathBuf::from(std::str::from_utf8(bytes).ok()?))
}

#[cfg(unix)]
fn path_bytes(path: &std::path::Path) -> Option<Vec<u8>> {
    use std::os::unix::ffi::OsStrExt;
    Some(path.as_os_str().as_bytes().to_vec())
}

#[cfg(not(unix))]
fn path_bytes(path: &std::path::Path) -> Option<Vec<u8>> {
    path.to_str().map(|text| text.as_bytes().to_vec())
}

/// How one row's files are going.
#[derive(Default)]
struct HowTheRowGoes {
    left: usize,
    refused_in: Option<Vec<u8>>,
    failed: Option<WhyItDidNotGoUp>,
    did_not_finish: bool,
}

/// **The found files go up** — see this file's head for the order.
///
/// `held` is the panel's own run memory, where each row's state is brought to
/// its files' publications as they end; `record` is the application's door,
/// whose `a_transfer_moved` tells both the panel and Transfers.
pub async fn the_found_files_go_up<C: AHeldConnection>(
    asked: &WhatIsSent,
    sessions: &ThisServersSessions<C>,
    the_run: &TheRunsTransfers,
    record: Arc<dyn TheRecordAnUploadWrites>,
    held: &Mutex<WhatTheRunHolds>,
) -> WhatTheSendCameTo {
    let mut came_to = WhatTheSendCameTo::default();
    let Ok(witness) = open_an_asset_upload();
    let of = &asked.of;
    let key = keyed(&of.server, &of.file);
    // **One pre-flight per check at a time**: a second press while one is
    // still deciding what to arrange joins it, the way a second press of
    // *Check dependencies…* joins a sitting. **Once its files are arranged the
    // hold is let go**, and a further press sends only what is not already on
    // its way (`Dependency Upload & Remap` A9a) — until the eighteenth notes'
    // piece 3 the hold lasted the whole send, and a press for rows found
    // after the first had begun did nothing.
    let Some(one_at_a_time) = ASendRuns::for_the_check(&key) else {
        return came_to;
    };
    let Some(snapshot) = the_snapshot(record.as_ref(), &of.machine) else {
        return came_to;
    };
    // **What the local record holds for the live-job line**, read once for the
    // whole press (`File Browser` W5, W2's moment).
    let holds = of.layout.as_ref().and_then(|layout| {
        let reading = record.the_last_queue_reading(&of.server);
        let mut holds = None;
        record.with_the_record(&mut |connection| {
            holds = crate::record::neighbouring_records::WhatTheLocalRecordHolds::read(
                connection,
                &of.machine,
                layout.clone(),
                reading.as_ref(),
            )
            .ok();
        });
        holds
    });
    let the_rows: Vec<(usize, ACheckedRow)> = {
        let Ok(held) = held.lock() else {
            return came_to;
        };
        let Some(sitting) = held.sittings.get(&key) else {
            return came_to;
        };
        asked
            .rows
            .iter()
            .filter_map(|at| sitting.rows.get(*at).map(|one| (*at, one.clone())))
            .collect()
    };
    // **`Dependency Upload & Remap` A9a: what goes is decided here, at the
    // press, from what is on its way now** — every row whose file a running
    // or waiting send of this class is already carrying to the same landing
    // place is left out, so a press from a panel drawn before a send began, or
    // a second press, arranges nothing for a file already going. Read once more
    // where the files are arranged, below, so two presses cannot race past it.
    let the_rows: Vec<(usize, ACheckedRow)> = {
        let on_its_way = the_files_on_their_way_to(&of.server);
        the_rows
            .into_iter()
            .filter(|(_, one)| !the_row_is_on_its_way(one, of.layout.as_ref(), &on_its_way))
            .collect()
    };

    // **`Dependency Upload & Remap` M16a: a *could not be fixed* row is tried again on its own** — M14's
    // production run once more for that one file, no check re-run and nothing
    // re-sent. The 38's piece 7.
    if let Some(layout) = of.layout.as_ref() {
        let package_area = layout.area(smoozy_job_files::layout::Area::Package);
        for (at, one) in the_rows.iter().filter(|(_, one)| one.state == RowState::CouldNotBeFixed) {
            let mut machine = crate::reading_the_file::ThisServersMachine {
                sessions,
                package_area: &package_area,
                server: &of.server,
            };
            match a_library_is_fixed(&mut machine, record.as_ref(), of, one, true).await {
                Ok(()) => the_row_became(held, &key, *at, RowState::AlreadyOnTheMachine, None),
                Err(why) => {
                    came_to.failed += 1;
                    the_row_became(held, &key, *at, RowState::CouldNotBeFixed, Some(why_it_did_not_go_up(why)));
                }
            }
        }
    }

    // **The pre-flight, row by row.**
    let mut files: Vec<AFileToSend> = Vec::new();
    let mut going: BTreeMap<usize, HowTheRowGoes> = BTreeMap::new();
    let mut folders = TheMirrorFoldersRead::default();
    for (at, one) in &the_rows {
        match the_pre_flight(one, *at, asked, &snapshot, holds.as_ref(), sessions, &mut folders).await {
            TheRowIs::NotSent => {}
            TheRowIs::AlreadyThere => {
                came_to.already_there += 1;
                the_row_became(held, &key, *at, RowState::AlreadyOnTheMachine, None);
            }
            TheRowIs::CouldNotBeSent(why) => {
                came_to.failed += 1;
                the_row_became(held, &key, *at, RowState::CouldNotBeSent, Some(why));
            }
            TheRowIs::AskedToReplace(overwrite) => {
                if came_to.asks_to_replace.is_none() {
                    came_to.asks_to_replace = Some(*at);
                    if let Ok(mut held) = held.lock() {
                        held.overwrites.insert(key.clone(), overwrite);
                    }
                }
            }
            TheRowIs::Sent(these) => {
                going.entry(*at).or_default().left = these.len();
                files.extend(these);
            }
        }
    }
    record.a_transfer_moved();
    if files.is_empty() {
        the_standing_is_brought_up_to_date(record.as_ref(), held, of);
        return came_to;
    }

    // **One request, and it is under way while this runs** (`File Browser`
    // U7's shape, which the Transfers row reads).
    let Ok(request) = Identity::issue(IdentityKind::Operation) else {
        return came_to;
    };
    // **`Dependency Upload & Remap` A9a once more, under the register's own
    // lock, and the send entered in the same hold** — so another press whose
    // pre-flight ran beside this one cannot arrange a file this one is about
    // to: a row with any file already on its way is left out whole, as at the
    // press. The record is written after the hold is let go, since its lock
    // is never taken inside the register's.
    let send = {
        let mut register = the_register();
        if let Some(layout) = of.layout.as_ref() {
            let on_its_way = the_files_on_their_way_in(&register, &of.server);
            let rows_on_their_way: std::collections::BTreeSet<usize> = files
                .iter()
                .filter(|file| is_on_its_way(layout, &file.source, &on_its_way))
                .map(|file| file.row)
                .collect();
            files.retain(|file| !rows_on_their_way.contains(&file.row));
            going.retain(|row, _| !rows_on_their_way.contains(row));
        }
        (!files.is_empty()).then(|| {
            let send = ASend {
                request: request.clone(),
                server: of.server.clone(),
                file: of.file.clone(),
                files: files
                    .iter()
                    .map(|file| AFileSent {
                        landing: file.mirror.as_bytes().to_vec(),
                        size: file.size,
                        transfer: None,
                        ended: None,
                    })
                    .collect(),
            };
            register.push(send.clone());
            send
        })
    };
    let Some(send) = send else {
        the_standing_is_brought_up_to_date(record.as_ref(), held, of);
        return came_to;
    };
    drop(one_at_a_time);
    let _ended_if_let_go = TheSendEndsIfLetGo(request.clone());
    let mut saved = false;
    record.with_the_record(&mut |connection| {
        saved = crate::record::dependency_records::save_dependency_send(connection, &send).is_ok();
    });
    if !saved {
        the_register().retain(|one| one.request != request);
        return came_to;
    }
    came_to.request = Some(request.clone());
    let under_way = the_run.a_request_goes_under_way(&request, &of.machine.server_row, sessions);
    let tell_it = Arc::clone(&record);
    let tell = move || tell_it.a_transfer_is_moving();
    record.a_transfer_moved();

    // **`Dependency Upload & Remap` A9's range, filled** — up to four files of
    // the send at once, each channel taking the next when its own ends
    // (`crate::downloading::every_one_of`, step 10c's eleventh-pass fix piece
    // 4). **The files start in the rows' order**; one that lands before a file
    // above it is harmless, since each lands at its own mirror through its own
    // publication and each row is brought to what its own files came to.
    let running = TheSendRunning {
        files: &files,
        next: TheNextFile::default(),
        going: Mutex::new(going),
        came_to: Mutex::new(came_to),
        the_mirror_directories: TheMirrorDirectoriesSeen::default(),
        the_staging_folders: TheStagingFoldersSeen::default(),
    };
    let workers = (0..THE_UPLOAD_DIRECTIONS_CEILING)
        .map(|_| {
            a_channel_of_the_send(
                &running, asked, &witness, sessions, the_run, &record, &request, &tell, held, &key,
            )
        })
        .collect();
    every_one_of(workers).await;
    let TheSendRunning { going, came_to, .. } = running;
    let going = going
        .into_inner()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let came_to = came_to
        .into_inner()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    drop(under_way);

    // **`Design/Whole app`'s once rule: several refused for one reason are said
    // once, above their rows** — the folder that refused the most of them.
    let mut refused_by_folder: BTreeMap<Vec<u8>, Vec<usize>> = BTreeMap::new();
    for (row, how) in &going {
        if let Some(folder) = &how.refused_in {
            refused_by_folder
                .entry(folder.clone())
                .or_default()
                .push(*row);
        }
    }
    if let Some((folder, rows)) = refused_by_folder
        .into_iter()
        .filter(|(_, rows)| rows.len() > 1)
        .max_by_key(|(_, rows)| rows.len())
    {
        if let Ok(mut held) = held.lock() {
            if let Some(sitting) = held.sittings.get_mut(&key) {
                sitting.the_once = Some(SeveralForOneReason {
                    folder,
                    rows,
                    cleared: false,
                });
            }
        }
    }

    the_standing_is_brought_up_to_date(record.as_ref(), held, of);
    record.a_transfer_moved();
    came_to
}

/// **One send while its files run**, shared by its channels. Added
/// 2026-09-30, step 10c's eleventh-pass fix piece 4. Every lock here is held
/// for one short act and never across a wait.
struct TheSendRunning<'f> {
    files: &'f [AFileToSend],
    next: TheNextFile,
    going: Mutex<BTreeMap<usize, HowTheRowGoes>>,
    came_to: Mutex<WhatTheSendCameTo>,
    /// **`Dependency Upload & Remap` A7's directories this send has seen
    /// standing**, so a folder is not asked for again in the same send.
    the_mirror_directories: TheMirrorDirectoriesSeen,
    /// **The staging folders this send has seen made or standing** (`Server
    /// protocol and transfers` S1), asked for once per folder.
    the_staging_folders: TheStagingFoldersSeen,
}

/// **One channel of a send**: the next file, to its ending, then the next —
/// and after each, the file's ending in the run's register, the send's count,
/// and its row brought to what its files came to once its last one has ended.
#[allow(clippy::too_many_arguments)]
async fn a_channel_of_the_send<C: AHeldConnection>(
    running: &TheSendRunning<'_>,
    asked: &WhatIsSent,
    witness: &SendingFilesUpWasOpen,
    sessions: &ThisServersSessions<C>,
    the_run: &TheRunsTransfers,
    record: &Arc<dyn TheRecordAnUploadWrites>,
    request: &Identity,
    tell: &(dyn Fn() + Sync),
    held: &Mutex<WhatTheRunHolds>,
    key: &str,
) {
    let request_text = request.as_text();
    while let Some(at) = running.next.of(running.files.len()) {
        let file = &running.files[at];
        let ended = if the_run.the_user_stopped_it(&request_text)
            || the_run.the_user_stopped_this(&request_text, file.mirror.as_bytes())
        {
            HowItEnded::DidNotFinish
        } else {
            one_file_goes_up(
                asked, witness, file, at, running, sessions, the_run, record, request, tell,
            )
            .await
        };
        a_file_of_the_send_moved(request, at, |sent| sent.ended = Some(ended.clone()));
        the_file_of_the_send_is_saved(record.as_ref(), request, at);
        {
            let mut came_to = running
                .came_to
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            match ended {
                HowItEnded::Published => {
                    came_to.published += 1;
                    if file.a_library {
                        came_to.libraries_published += 1;
                    }
                }
                HowItEnded::Refused | HowItEnded::Failed => came_to.failed += 1,
                HowItEnded::DidNotFinish => came_to.did_not_finish += 1,
            }
        }
        // **The row's state from its files' publications**, once its last
        // file has ended.
        let row = file.row;
        let became = {
            let mut going = running
                .going
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            let how = going.entry(row).or_default();
            how.left = how.left.saturating_sub(1);
            match ended {
                HowItEnded::Published => {}
                HowItEnded::Refused => {
                    how.refused_in = Some(the_folder_holding(file.mirror.as_bytes()));
                    how.failed = Some(WhyItDidNotGoUp::TheServerRefusedTheWrite);
                }
                HowItEnded::Failed => {
                    how.failed.get_or_insert(WhyItDidNotGoUp::TheUploadFailed);
                }
                HowItEnded::DidNotFinish => how.did_not_finish = true,
            }
            if how.left == 0 {
                match (&how.failed, how.did_not_finish) {
                    (Some(why), _) => Some((RowState::CouldNotBeSent, Some(*why))),
                    (None, true) => None,
                    (None, false) => Some((RowState::AlreadyOnTheMachine, None)),
                }
            } else {
                None
            }
        };
        if let Some((state, why)) = became {
            the_row_became(held, key, row, state, why);
        }
        record.a_transfer_moved();
    }
}

/// **A row brought to what its files came to.** It stays in the group it was
/// drawn in (`Design/Dependency Upload & Remap § How the rows are grouped and
/// sorted`: *"Nothing re-sorts under the user's hand"*).
fn the_row_became(
    held: &Mutex<WhatTheRunHolds>,
    key: &str,
    at: usize,
    state: RowState,
    why: Option<WhyItDidNotGoUp>,
) {
    if let Ok(mut held) = held.lock() {
        if let Some(one) = held
            .sittings
            .get_mut(key)
            .and_then(|sitting| sitting.rows.get_mut(at))
        {
            one.state = state;
            one.why_it_did_not_go_up = why;
            one.its_failure_was_cleared = false;
        }
    }
}

/// **One file up** — the slot and the seat, `Dependency Upload & Remap` A7's
/// folders, the staging folder asked for first (`File Browser` P1 and `File
/// Browser` P6a: a refused write is a failure told as one), the arrangement
/// (`Dependency Upload & Remap` A9), and
/// `Server protocol and transfers` §7's loop carried across a repair; then, at
/// publication, K7's placement record and V13's marks.
#[allow(clippy::too_many_arguments)]
async fn one_file_goes_up<C: AHeldConnection>(
    asked: &WhatIsSent,
    witness: &SendingFilesUpWasOpen,
    file: &AFileToSend,
    at: usize,
    running: &TheSendRunning<'_>,
    sessions: &ThisServersSessions<C>,
    the_run: &TheRunsTransfers,
    record: &Arc<dyn TheRecordAnUploadWrites>,
    request: &Identity,
    tell: &(dyn Fn() + Sync),
) -> HowItEnded {
    let of = &asked.of;
    let request_text = request.as_text();
    let landing = file.mirror.as_bytes();
    // **`Desktop app and delivery` R17: the slot is asked for per file**, and
    // the seat with it; while the line is down and the period stands, it waits.
    let Some((mut draw, mut subsystem)) = a_slot_and_a_seat(
        THE_TRANSFER_PRIORITY_CLASS,
        sessions,
        the_run,
        (&request_text, Some(at), tell),
    )
    .await
    else {
        return HowItEnded::DidNotFinish;
    };
    let let_go = |draw: &mut crate::scheduling::TheChannelsDraw| {
        let _ = draw.the_channel_ended(std::time::Instant::now());
    };
    // **Stopped while it waited for its turn** (`File Browser` U6): with four
    // files of the send at once, a file may wait for the slot while the send
    // is stopped, and a file the user stopped is never arranged. Step 10c's
    // eleventh-pass fix piece 4.
    if the_run.the_user_stopped_this(&request_text, landing) {
        let_go(&mut draw);
        return HowItEnded::DidNotFinish;
    }

    // **`Dependency Upload & Remap` A7: the mirror's folders, segment by
    // segment, with the plain form** — **and only those this send has not
    // already seen standing** (A7's own *a directory seen standing stays
    // standing*; step 10c's eleventh-pass fix piece 4).
    if make_the_mirror_directories(
        witness,
        &mut subsystem,
        &file.mirror,
        &running.the_mirror_directories,
    )
    .await
        == HowTheDirectoryWent::TheRowFailsAsCouldNotBeSent
    {
        let_go(&mut draw);
        return HowItEnded::Failed;
    }
    // **`Server protocol and transfers` S1: staging sits beside the final
    // name**, in the reserved folder `File Browser` X12 names — asked for
    // first, so a folder the login may not write is the failure it is, and
    // **once per folder in a send** (`crate::uploading::TheStagingFoldersSeen`).
    let mut staging = the_folder_holding(landing);
    staging.push(b'/');
    staging.extend_from_slice(THE_RESERVED_STAGING_FOLDER.as_bytes());
    let made_by_this_send = match running
        .the_staging_folders
        .asked_for(&mut subsystem, &staging)
        .await
    {
        Ok(made) => made,
        Err(refused) => {
            let_go(&mut draw);
            if let Ok(mut held) = the_refusals_this_run().lock() {
                held.insert((request_text.clone(), landing.to_vec()), refused.in_words());
            }
            return HowItEnded::Refused;
        }
    };
    // **The intent** (`Server protocol and transfers` S15a): a file already at
    // the mirror is replaced — the pre-flight either established it is not the
    // file this reference means and the user answered *Replace*, or could not
    // establish it either way and the file is sent again; **with nothing
    // there, the publication refuses to overwrite.**
    let intent = match subsystem.stat(landing).await {
        Ok(_) => Intent::ReplaceIntended,
        Err(_) => Intent::CreateIntended,
    };

    // **`File Browser` K10 and K12: a part an earlier send left at this
    // mirror is carried on** — the same act as Files' own upload, taken up
    // through `Server protocol and transfers` S12 entire and K10's two gates
    // (the source's identity, and the staged prefix by contents), and started
    // over from nought where either fails. The 38's piece 7; until then every
    // send arranged afresh and the part-sent file went again from its start.
    // **`Dependency Upload & Remap` A10, adopted by `File Browser` K10 as its
    // clause 2: the source on this computer, read up to the part's highest
    // checkpoint, has to digest to that checkpoint's running digest** — so
    // carrying on writes exactly the file as it is now. Where it does not,
    // the part is written *stopped, abandoned by its request* first (`Server
    // protocol and transfers` S28a, so the sweep reaches it) and the file
    // goes again from its start.
    let earlier = the_part_an_earlier_send_left(record.as_ref(), of, &asked.run, file).filter(|part| {
        let carries_on = matches!(
            smoozy_dependencies::landing::a_resume_of_this_modules_upload(
                u64::MAX,
                &part.checkpoints,
                &file.source,
                &crate::checking_a_file::TheUsersOwnComputer,
            ),
            smoozy_dependencies::landing::ResumeVerdict::Proceeds { .. }
        );
        if !carries_on {
            let the_door = |act: &mut dyn FnMut(&rusqlite::Connection)| record.with_the_record(act);
            let _ = crate::downloading::a_part_is_abandoned(&the_door, the_run, part);
        }
        carries_on
    });
    // **A fresh file** (`Server protocol and transfers` S12a, step 10c's
    // owed piece 1): its staging folder made by this
    // send, and no part an earlier send left taken up.
    let fresh = made_by_this_send && earlier.is_none();
    let (transfer, mut arranged) = match earlier {
        Some(mut part) => {
            part.written_by_run = asked.run.clone();
            part.ending = None;
            the_run.every_leftover_is_read_again();
            (part.transfer.clone(), part)
        }
        None => {
            let (Ok(transfer), Ok(staging_text)) = (
                Identity::issue(IdentityKind::Transfer),
                String::from_utf8(staging),
            ) else {
                let_go(&mut draw);
                return HowItEnded::Failed;
            };
            // **`Dependency Upload & Remap` A9 and `Dependency Upload & Remap`
            // A11: one asset's upload, a new transfer under a new identity.**
            let Ok(arranged) = arrange_one_assets_upload(
                witness,
                transfer.clone(),
                of.machine.server_row.clone(),
                of.machine.installation.clone(),
                asked.run.clone(),
                &file.source,
                &file.mirror,
                &staging_text,
                file.size,
            ) else {
                let_go(&mut draw);
                return HowItEnded::Failed;
            };
            (transfer, arranged)
        }
    };
    a_file_of_the_send_moved(request, at, |sent| sent.transfer = Some(transfer.clone()));
    the_file_of_the_send_is_saved(record.as_ref(), request, at);

    let waiting = crate::downloading::the_next_waiting(running.files, at, |one| {
        Some(one.mirror.as_bytes().to_vec())
    });
    the_run.a_file_goes(&request_text, at, landing.to_vec(), file.size, waiting);
    let mut watcher = TheRowIsTold {
        the_run,
        request: request_text.clone(),
        entry: at,
        tell,
        started_over: None,
    };
    let mut keeper = TheSendsRecords {
        record: Arc::clone(record),
        the_run,
        request: request_text.clone(),
        at,
        what_was_staged: None,
    };
    let source = arranged.source_path.clone();
    let mut reading_for_the_gate = ThePlatformFileAdapter;
    let mut gate =
        TheSourcesPublicationGate::over(&mut reading_for_the_gate, source.clone(), file.size);
    let mut local = ThePlatformFileAdapter;
    let mut identities = ThePlatformFileAdapter;
    let way_back = TheWayBack {
        sessions,
        the_run,
        class: THE_TRANSFER_PRIORITY_CLASS,
        request: &request_text,
        entry: at,
        tell,
        this_run: asked.run.clone(),
        landing,
        record,
    };
    // **`File Browser` K7 and K8: the source is watched where its identity can
    // be read**, and K9's gate covers it where it cannot.
    let ending = match TheSourceUnderWatch::arranged(&mut local, &mut identities, source.clone()) {
        Some(mut watched) => {
            carried_across_repairs(
                &way_back,
                (&mut subsystem, &mut draw),
                &mut watched,
                &mut keeper,
                &mut watcher,
                intent,
                &mut arranged,
                &mut gate,
                fresh,
            )
            .await
        }
        None => {
            let mut plain = ThePlatformFileAdapter;
            carried_across_repairs(
                &way_back,
                (&mut subsystem, &mut draw),
                &mut plain,
                &mut keeper,
                &mut watcher,
                intent,
                &mut arranged,
                &mut gate,
                fresh,
            )
            .await
        }
    };
    let_go(&mut draw);
    drop(draw);
    drop(subsystem);
    drop(watcher);
    the_run.a_file_stopped(&request_text, at);
    let what_was_staged = keeper.what_was_staged.take();
    drop(keeper);

    if the_user_stopped_this_ending(the_run, &request_text, landing, &ending) {
        return HowItEnded::DidNotFinish;
    }
    match ending {
        Ending::Published => {
            // **`Dependency Upload & Remap` K7 and Q11: the placement record,
            // written at publication**, with the four values read off the
            // published file and the transfer's own digest (Q1, `Server
            // protocol and transfers` S22). Where the four values cannot be
            // read, no record is written: nothing could ever compare against
            // it, and a later check sends the file again.
            if let (Some(content), FourValuesNow::Answered(four_values)) = (
                what_was_staged,
                the_four_values_at(sessions, &of.server, &file.mirror).await,
            ) {
                the_publication_is_written(
                    record.as_ref(),
                    of,
                    Published {
                        at: file.mirror.clone(),
                        source: file.source.clone(),
                        content,
                        four_values,
                    },
                    intent == Intent::ReplaceIntended,
                );
            }
            HowItEnded::Published
        }
        Ending::Stopped(_) | Ending::Failed => HowItEnded::Failed,
        Ending::Interrupted | Ending::Stalled => HowItEnded::DidNotFinish,
    }
}

/// **The part an earlier send left at this file's mirror, where one may be
/// taken up** — from this source, under this installation, and either
/// *interrupted* (`Server protocol and transfers` S26b: the one ending whose
/// work is waiting to be carried on) or **left with no ending by a run that is
/// over**, which is what quitting mid-file leaves (`Server protocol and
/// transfers` C13: *"a transfer that ended with the application run"*). **A
/// record of this run with no ending is moving now** and is never taken. The
/// newest such part is taken; whether it may carry on is K10's, at the resume.
fn the_part_an_earlier_send_left(
    record: &dyn TheRecordAnUploadWrites,
    of: &TheSittingIsOf,
    this_run: &Identity,
    file: &AFileToSend,
) -> Option<TransferRecord> {
    let landing = std::str::from_utf8(file.mirror.as_bytes()).ok()?;
    let mut parts = Vec::new();
    record.with_the_record(&mut |connection| {
        parts = crate::record::transfer_records::the_upload_parts_at(
            connection,
            &of.machine.server_row,
            landing,
        )
        .unwrap_or_default();
    });
    parts.into_iter().rev().find(|part| {
        part.source_path.as_bytes() == file.source.as_bytes()
            && part.installation.as_ref() == Some(&of.machine.installation)
            && match &part.ending {
                Some(Ending::Interrupted) => true,
                None => &part.written_by_run != this_run,
                Some(_) => false,
            }
    })
}

/// **`Server protocol and transfers` S8's writer for a send** — one save per
/// write through the application's own record (the transfer wave's piece 1),
/// and **the content identity the loop staged, remembered from `Server
/// protocol and transfers` S15's pre-state**, which is what the placement
/// record's content is (`Dependency Upload & Remap` Q1: the pass-through
/// digest **is** the content identity of what was published).
struct TheSendsRecords<'a> {
    record: Arc<dyn TheRecordAnUploadWrites>,
    the_run: &'a TheRunsTransfers,
    request: String,
    at: usize,
    what_was_staged: Option<ContentIdentity>,
}

impl KeepsTheRecord for TheSendsRecords<'_> {
    fn write_it_down(&mut self, record: &TransferRecord) -> Result<(), LocalFailure> {
        if let Some(pending) = &record.publication_pending {
            self.what_was_staged = Some(ContentIdentity {
                size: pending.staged_byte_length,
                digest: pending.staged_digest.clone(),
            });
        }
        let mut written = Err(LocalFailure("the local record would not open".into()));
        let (the_run, request, at) = (self.the_run, &self.request, self.at);
        self.record.with_the_record(&mut |connection| {
            written = crate::record::transfer_records::put(connection, record)
                .map_err(|failure| LocalFailure(format!("{failure:?}")));
            // **The file stops moving in the same hold as its ending goes
            // down**, so a Transfers row composed between never counts it
            // twice (the transfer wave's piece 5).
            if written.is_ok() && record.ending.is_some() {
                the_run.a_file_stopped(request, at);
            }
        });
        written
    }
}

/// **`Dependency Upload & Remap` K7's placement record and V13's marks, in one
/// save, at a publication.**
///
/// - **V13's first trigger**: every standing whose root is the path this
///   publication landed on is marked *replaced since the check*.
/// - **`Dependency Upload & Remap` V13a**: where this publication replaced a file, every standing whose
///   tree resolved the location is marked *disturbed* — **except the check
///   whose own rows sent it**, since what this sitting published is what its
///   check described.
fn the_publication_is_written(
    record: &dyn TheRecordAnUploadWrites,
    of: &TheSittingIsOf,
    published: Published,
    it_replaced_a_file: bool,
) {
    let Some(layout) = of.layout.as_ref() else {
        return;
    };
    let landed_at = published.at.clone();
    let placement = PlacementRecord::at_publication(of.machine.clone(), published);
    let mut marked: Vec<CheckStanding> = Vec::new();
    record.with_the_record(&mut |connection| {
        if connection
            .execute_batch("SAVEPOINT a_placement_and_its_marks")
            .is_err()
        {
            return;
        }
        let keeper = TheDependencyRecord::over(connection);
        let mut all_went_down = keeper.a_placement_is_written(&placement).is_ok();
        let standings = keeper.the_standings_on(&of.machine).unwrap_or_default();
        let roots: Vec<MachineLocation> = standings
            .iter()
            .map(|standing| standing.root.clone())
            .collect();
        if let Ok(snapshot) = keeper.what_it_holds_for(&of.machine) {
            // **`File Browser` N6's answer, handed to the walk**, so a root
            // uploaded through Files is found the way its check keyed it
            // (step 10c's dependency wave piece 7b).
            let files_says =
                crate::record::dependency_records::WhatFilesSays::read(connection, &of.machine);
            let marks = what_this_publication_marks(
                &landed_at,
                &of.machine,
                layout,
                &snapshot,
                &roots,
                &|file| files_says.where_it_came_from(file),
            );
            for standing in standings {
                let replaced = marks.replaced_since_check.contains(&standing.root);
                let disturbed = it_replaced_a_file
                    && standing.root != of.root
                    && marks.disturbed.contains(&standing.root);
                if !replaced && !disturbed {
                    continue;
                }
                let mut standing = standing;
                if replaced {
                    standing = standing.replaced_since_check();
                }
                if disturbed {
                    standing = standing.disturbed();
                }
                all_went_down &= keeper.a_standing_is_written(&standing).is_ok();
                marked.push(standing);
            }
        }
        if all_went_down {
            let _ = connection.execute_batch("RELEASE a_placement_and_its_marks");
        } else {
            marked.clear();
            let _ = connection.execute_batch(
                "ROLLBACK TO a_placement_and_its_marks; RELEASE a_placement_and_its_marks",
            );
        }
    });
    for standing in marked {
        the_band_is_told(&of.server, standing);
    }
}

/// **The Render Settings band reads the standing from the run's memory** until
/// dependency piece 7 moves it to the record (`src/surface/render_settings.rs`,
/// `the_dependency_band`), so a standing written here is handed to it too, the
/// way `crate::checking_a_file` hands it the one it writes.
fn the_band_is_told(server: &str, standing: CheckStanding) {
    let file = crate::surface::render_settings::the_path_as_it_is_keyed(standing.root.as_bytes());
    if let Ok(mut page) = crate::surface::render_settings::the_run_holds().lock() {
        page.the_check_record.insert(
            keyed(server, &file),
            WhatTheRecordSays::AStanding(Box::new(standing)),
        );
    }
}

/// **The check standing brought up to date with the sitting's rows** —
/// `Dependency Upload & Remap` V11: *"Confirmed on the machine is Q5's
/// establishment — placed and published by this sitting's uploads, or already
/// there by record"*. **Its completion, its moment, its version, the root's four
/// values and its marks are the sitting's and are kept**; only what the rows
/// came to moves.
///
/// **A row inside a linked `.blend` is confirmed only where that file's
/// fixing record matches the current located answers and no re-production is
/// owed** (V11) — read off the record with [`the_fixing_status_of`], the same
/// reading `crate::checking_a_file` writes at the sitting's end. *(Until the
/// 38's piece 7 such a row was never confirmed: nothing fixed anything.)*
pub(crate) fn the_standing_is_brought_up_to_date<R: TheCheckReachesTheRecord + ?Sized>(
    record: &R,
    held: &Mutex<WhatTheRunHolds>,
    of: &TheSittingIsOf,
) {
    let states: Vec<(RowState, smoozy_dependencies::record::HoldingFile)> = match held.lock() {
        Ok(held) => match held.sittings.get(&keyed(&of.server, &of.file)) {
            Some(sitting) => sitting
                .rows
                .iter()
                .map(|row| (row.state, row.holding.clone()))
                .collect(),
            None => return,
        },
        Err(_) => return,
    };
    // **Read with no lock of the run's held**: the record's lock comes first.
    let mut statuses: Vec<(smoozy_dependencies::record::HoldingFile, FixingStatus)> = Vec::new();
    for (_, holding) in &states {
        if holding != &of.holding && !statuses.iter().any(|(one, _)| one == holding) {
            statuses.push((holding.clone(), the_fixing_status_of(record, of, holding)));
        }
    }
    let rows: Vec<RowAtCompletion> = states
        .into_iter()
        .map(|(state, holding)| RowAtCompletion {
            state,
            inside_a_linked_blend: (holding != of.holding).then(|| {
                statuses
                    .iter()
                    .find(|(one, _)| one == &holding)
                    .map(|(_, status)| *status)
                    .unwrap_or(FixingStatus {
                        the_fixing_matches_the_current_located_answers: false,
                        a_re_production_is_owed: true,
                    })
            }),
        })
        .collect();
    let mut brought = None;
    record.with_the_record(&mut |connection| {
        let keeper = TheDependencyRecord::over(connection);
        let Ok(Some(standing)) = keeper.the_standing(&of.root, &of.machine) else {
            return;
        };
        let standing = CheckStanding {
            counts: counts_at_completion(&rows),
            every_row_confirmed_or_set_aside: every_row_is_confirmed_or_set_aside(&rows),
            ..standing
        };
        if keeper.a_standing_is_written(&standing).is_ok() {
            brought = Some(standing);
        }
    });
    if let Some(standing) = brought {
        the_band_is_told(&of.server, standing);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_dependencies::bytes::RecordedLocation;
    use smoozy_dependencies::record::{
        HoldingFile, HowResolved, LocatedAnswer, LocatedAnswerKey, Resolution,
    };
    use smoozy_dependencies::standing::Completion;
    use smoozy_job_files::layout::{MachineRoot, TheLayout, WhereThePackageAreaSits};
    use smoozy_render_settings::file_half::BlenderIdentity;
    use smoozy_server_protocol::transfers::SourceReading;
    use smoozy_server_setup::blender::Version;

    struct TheRecordHere(Mutex<crate::record::LocalRecord>);

    impl TheRecordAnUploadWrites for TheRecordHere {
        fn with_the_record(&self, act: &mut dyn FnMut(&rusqlite::Connection)) -> bool {
            act(self.0.lock().expect("the record").connection());
            true
        }
    }

    fn a_layout() -> TheLayout {
        TheLayout::of(
            MachineRoot::chosen("/ephemeral/smoozy").expect("a root"),
            WhereThePackageAreaSits::UnderTheRoot,
        )
    }

    fn stored(bytes: &[u8]) -> StoredLocation {
        StoredLocation::as_the_system_returned_it(bytes)
    }

    /// **`Dependency Upload & Remap` A9a's per-file reading of the register**:
    /// every file of this server's sends that is running or waiting, and none
    /// that has ended — published, refused, failed, or stopped — and none of
    /// another server's.
    #[test]
    fn the_files_on_their_way_are_this_servers_files_still_running_or_waiting() {
        let a_file = |landing: &[u8], ended: Option<HowItEnded>| AFileSent {
            landing: landing.to_vec(),
            size: 1,
            transfer: None,
            ended,
        };
        let a_send = |server: &str, files: Vec<AFileSent>| ASend {
            request: Identity::issue(IdentityKind::Operation).expect("randomness"),
            server: server.to_string(),
            file: "/Users/a/scene.blend".to_string(),
            files,
        };
        let register = vec![
            a_send(
                "here",
                vec![
                    a_file(b"/m/waiting.png", None),
                    a_file(b"/m/published.png", Some(HowItEnded::Published)),
                    a_file(b"/m/refused.png", Some(HowItEnded::Refused)),
                    a_file(b"/m/failed.png", Some(HowItEnded::Failed)),
                    a_file(b"/m/stopped.png", Some(HowItEnded::DidNotFinish)),
                ],
            ),
            a_send("elsewhere", vec![a_file(b"/m/theirs.png", None)]),
            a_send("here", vec![a_file(b"/m/second.png", None)]),
        ];
        let going: Vec<Vec<u8>> = the_files_on_their_way_in(&register, "here")
            .into_iter()
            .map(|landing| landing.as_bytes().to_vec())
            .collect();
        assert_eq!(going, vec![b"/m/waiting.png".to_vec(), b"/m/second.png".to_vec()]);
        assert!(the_files_on_their_way_in(&register, "nowhere").is_empty());
    }

    fn four(size: u64) -> SourceReading {
        SourceReading {
            size,
            inode: "77".to_string(),
            modification_time: 100,
            change_time: 100,
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

    /// **A root placed by this module, whose own reference resolved to the
    /// texture**, with its standing written — the shape
    /// `Dependency Upload & Remap` V13a's walk reaches through the record.
    fn a_root_that_resolved_the_texture(
        keeper: &TheDependencyRecord<'_>,
        machine: &MachineKey,
        root_source: &[u8],
        texture_source: &[u8],
    ) -> MachineLocation {
        let root = mirror_of(&a_layout(), &stored(root_source)).expect("fits");
        keeper
            .a_placement_is_written(&PlacementRecord::at_publication(
                machine.clone(),
                Published {
                    at: root.clone(),
                    source: stored(root_source),
                    content: ContentIdentity::of(b"a scene"),
                    four_values: four(7),
                },
            ))
            .expect("the root's placement");
        keeper
            .the_answers_are_written(&[LocatedAnswer {
                key: LocatedAnswerKey {
                    holding_file: HoldingFile::OnTheUsersComputer(stored(root_source)),
                    reference: RecordedLocation::as_the_reading_carried_it(b"//tex/stone.png"),
                },
                resolution: Some(Resolution {
                    to: ResolvedTo::AWholeLocation(stored(texture_source)),
                    how: HowResolved::TheRecordedLocationItself,
                }),
                set_aside: false,
                rejected_guess: None,
                read_as: Vec::new(),
            }])
            .expect("the root's answer");
        keeper
            .a_standing_is_written(&CheckStanding::at_the_sittings_end(
                root.clone(),
                machine.clone(),
                a_blender(),
                SystemTime::now(),
                Completion::Completed,
                &[],
                four(7),
            ))
            .expect("the root's standing");
        root
    }

    /// **`Dependency Upload & Remap` V13 and `Dependency Upload & Remap` V13a, written at a publication by
    /// the module that published.** A publication landing on a root's own
    /// path marks that standing replaced; a publication that replaced a file a
    /// root's tree resolved marks that root disturbed — **but never the check
    /// whose own rows sent it**, and a first placement, which replaced
    /// nothing, disturbs nobody. K7's record is written in the same save.
    #[test]
    fn a_publication_writes_its_placement_and_marks_the_standings_the_plan_names() {
        for it_replaced_a_file in [true, false] {
            let folder = std::env::temp_dir().join(format!(
                "smoozy-found-files-sent-{}-{it_replaced_a_file}",
                std::process::id()
            ));
            let _ = std::fs::remove_dir_all(&folder);
            std::fs::create_dir_all(&folder).expect("a folder");
            let record = TheRecordHere(Mutex::new(
                crate::record::LocalRecord::open_at(&folder.join("record.sqlite3"))
                    .expect("it opens"),
            ));
            let machine = MachineKey::of(
                Identity::issue(IdentityKind::LocalServer).expect("randomness"),
                Identity::issue(IdentityKind::Installation).expect("randomness"),
            );
            let texture_source = b"/Users/a/tex/stone.png";
            let texture = mirror_of(&a_layout(), &stored(texture_source)).expect("fits");
            let (own, another, the_textures_own) = {
                let held = record.0.lock().expect("the record");
                let keeper = TheDependencyRecord::over(held.connection());
                let own = a_root_that_resolved_the_texture(
                    &keeper,
                    &machine,
                    b"/Users/a/own.blend",
                    texture_source,
                );
                let another = a_root_that_resolved_the_texture(
                    &keeper,
                    &machine,
                    b"/Users/a/another.blend",
                    texture_source,
                );
                // **A standing whose root is the very path the publication lands on.**
                keeper
                    .a_standing_is_written(&CheckStanding::at_the_sittings_end(
                        texture.clone(),
                        machine.clone(),
                        a_blender(),
                        SystemTime::now(),
                        Completion::Completed,
                        &[],
                        four(5),
                    ))
                    .expect("a standing");
                (own, another, texture.clone())
            };
            let of = TheSittingIsOf {
                server: machine.server_row.as_text(),
                file: String::from_utf8_lossy(own.as_bytes()).into_owned(),
                root: own.clone(),
                holding: HoldingFile::OnTheUsersComputer(stored(b"/Users/a/own.blend")),
                with: a_blender(),
                machine: machine.clone(),
                layout: Some(a_layout()),
            };
            the_publication_is_written(
                &record,
                &of,
                Published {
                    at: texture.clone(),
                    source: stored(texture_source),
                    content: ContentIdentity::of(b"stone"),
                    four_values: four(5),
                },
                it_replaced_a_file,
            );
            let held = record.0.lock().expect("the record");
            let keeper = TheDependencyRecord::over(held.connection());
            let standing = |root: &MachineLocation| {
                keeper
                    .the_standing(root, &machine)
                    .expect("reads")
                    .expect("stands")
            };
            assert!(
                keeper
                    .the_placements_on(&machine)
                    .expect("reads")
                    .iter()
                    .any(|placement| placement.mirrored_location == texture),
                "K7's record is written at publication"
            );
            assert!(
                standing(&the_textures_own).replaced_since_check,
                "the replaced mark"
            );
            assert_eq!(
                standing(&another).disturbed,
                it_replaced_a_file,
                "the disturbance mark, replaced {it_replaced_a_file}"
            );
            assert!(
                !standing(&own).disturbed,
                "the check whose rows sent it is not disturbed by its own send"
            );
            drop(held);
            let _ = std::fs::remove_dir_all(&folder);
        }
    }
}

/// **Two readings of the four values on one server at once, from two
/// callers** — step 10c's eleventh-pass fix piece 4b.
///
/// **The far end is OpenSSH's own `sftp-server`**, one per session over a Unix
/// socket pair, so the link is planted, replaced and removed by the program a
/// rental's own file-transfer server is. **The command is a stand-in**: it
/// answers GNU `stat`'s four fields for whatever the link names when it runs,
/// **a round trip after it was asked** — the moment the real command runs on a
/// real machine, and the moment another act's link can land first. The point
/// is the real one, lending the stand-in's connection, with a command handed
/// back running the way the real transport hands one back.
#[cfg(all(test, unix))]
mod two_readings_on_one_server {
    use std::os::fd::OwnedFd;
    use std::os::unix::ffi::OsStrExt;
    use std::path::{Path, PathBuf};
    use std::process::{Command, Stdio};
    use std::sync::Arc;
    use std::time::Duration;

    use smoozy_dependencies::bytes::MachineLocation;
    use smoozy_dependencies::readings::FourValuesNow;
    use smoozy_server_protocol::transfers::SourceReading;
    use smoozy_server_protocol::transport::{CommandResult, CommandString, Ending};

    use crate::reading_the_file::{TheMachineAFileIsReadOn, ThisServersMachine};
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

    /// **Local server identifiers as the record writes them**, the key every
    /// reader of a server names it by — **one per test**, because the tests of
    /// this binary run at once and two of them naming one server would take
    /// turns with each other rather than show anything.
    const THE_SUBMISSIONS_SERVER: &str = "5e7e5e7e5e7e5e7e5e7e5e7e5e7e5e71";
    const THE_PAGES_SERVER: &str = "5e7e5e7e5e7e5e7e5e7e5e7e5e7e5e72";

    /// **How long after it is asked the command runs** — a round trip.
    const A_ROUND_TRIP: Duration = Duration::from_millis(150);

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
            let home = std::env::temp_dir().join(format!("smzy-4b-{named}-{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&home);
            std::fs::create_dir_all(home.join("scenes")).expect("the login's home");
            let home = std::fs::canonicalize(&home).expect("the home is there");
            Some(Arc::new(AMachine { sftp_server, home }))
        }

        /// A `.blend` of its own size, so two files' four values differ.
        fn a_file(&self, named: &str, size: usize) -> PathBuf {
            let at = self.home.join("scenes").join(named);
            std::fs::write(&at, vec![7u8; size]).expect("the file is written");
            at
        }

        /// **What the machine's own `stat -L` would say of the link, now.**
        fn the_command_runs(&self, command: &str) -> CommandResult {
            use std::os::unix::fs::MetadataExt;
            let Some(link) = command.strip_prefix("stat -L -c '%s %i %Y %Z' ~/") else {
                return CommandResult {
                    output: Vec::new(),
                    complaint: b"no such command here".to_vec(),
                    status: Some(127),
                };
            };
            match std::fs::metadata(self.home.join(link)) {
                Ok(found) => CommandResult {
                    output: format!(
                        "{} {} {} {}\n",
                        found.size(),
                        found.ino(),
                        found.mtime(),
                        found.ctime()
                    )
                    .into_bytes(),
                    complaint: Vec::new(),
                    status: Some(0),
                },
                Err(_) => CommandResult {
                    output: Vec::new(),
                    complaint: b"stat: cannot statx: No such file or directory".to_vec(),
                    status: Some(1),
                },
            }
        }
    }

    impl Drop for AMachine {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.home);
        }
    }

    fn the_four_values_of_the_file(at: &Path) -> SourceReading {
        use std::os::unix::fs::MetadataExt;
        let found = std::fs::metadata(at).expect("the file is there");
        SourceReading {
            size: found.size(),
            inode: found.ino().to_string(),
            modification_time: found.mtime(),
            change_time: found.ctime(),
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
            // **It ends when our end closes**, and is waited for so it
            // leaves nothing behind.
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
        /// command whose request has gone out — so the point serves the
        /// other act's link while this command is on its way.
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

    /// **The point, lending the stand-in's connection.**
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

    fn text_of(at: &Path) -> String {
        String::from_utf8(at.as_os_str().as_bytes().to_vec()).expect("a path in words")
    }

    /// **The send's reading at a publication** — [`super::the_four_values_at`].
    async fn the_sends_reading(
        sessions: &ThisServersSessions<AConnectionToIt>,
        server: &str,
        of: &Path,
    ) -> FourValuesNow {
        super::the_four_values_at(
            sessions,
            server,
            &MachineLocation::composed(of.as_os_str().as_bytes().to_vec()),
        )
        .await
    }

    /// **A submission's reading** — `Job files and outputs` M1's identity at
    /// submission, the call `crate::submitting::the_act_is_performed` makes.
    async fn a_submissions_reading(
        sessions: &ThisServersSessions<AConnectionToIt>,
        server: &str,
        of: &Path,
    ) -> smoozy_render_settings::readings::FourValuesNow {
        crate::submitting::the_four_values_of(sessions, server, &text_of(of)).await
    }

    /// **The Render Settings page's reading** — `Render Settings` Q14 item 2,
    /// through the machine the page's read hands its reading.
    async fn the_pages_reading(
        sessions: &ThisServersSessions<AConnectionToIt>,
        server: &str,
        of: &Path,
    ) -> smoozy_render_settings::readings::FourValuesNow {
        ThisServersMachine {
            sessions,
            package_area: "/ephemeral/smoozy/package",
            server,
        }
        .the_four_values_of(&text_of(of))
        .await
    }

    fn the_site_is_empty(machine: &AMachine) -> bool {
        std::fs::symlink_metadata(
            machine.home.join(
                smoozy_render_settings::acting::four_values::THE_BLENDS_CARRIAGE_SITE
                    .link_path_on_the_subsystem,
            ),
        )
        .is_err()
    }

    #[tokio::test]
    async fn a_sends_reading_beside_a_submissions_reads_each_its_own_file() {
        let Some(machine) = AMachine::standing("submission") else {
            println!("SKIPPED: no sftp-server on this system");
            return;
        };
        let (published, submitted) = (
            machine.a_file("texture.blend", 1_000),
            machine.a_file("shot.blend", 3_000),
        );
        let point = APointLending::over(&machine).await;

        let (the_send_read, the_submission_read) = tokio::join!(
            the_sends_reading(&point.sessions, THE_SUBMISSIONS_SERVER, &published),
            a_submissions_reading(&point.sessions, THE_SUBMISSIONS_SERVER, &submitted),
        );

        // **Both together, so a failure shows what each came back with**: the
        // send's placement record and the submission's file identity would
        // carry these.
        assert_eq!(
            (the_send_read, the_submission_read),
            (
                FourValuesNow::Answered(the_four_values_of_the_file(&published)),
                smoozy_render_settings::readings::FourValuesNow::Answered(
                    the_four_values_of_the_file(&submitted)
                ),
            ),
            "(the send's reading, the submission's reading)"
        );
        assert!(the_site_is_empty(&machine), "the link is taken away");
    }

    #[tokio::test]
    async fn a_sends_reading_beside_the_render_settings_pages_reads_each_its_own_file() {
        let Some(machine) = AMachine::standing("page") else {
            println!("SKIPPED: no sftp-server on this system");
            return;
        };
        let (published, opened) = (
            machine.a_file("texture.blend", 1_000),
            machine.a_file("shot.blend", 3_000),
        );
        let point = APointLending::over(&machine).await;

        let (the_send_read, the_page_read) = tokio::join!(
            the_sends_reading(&point.sessions, THE_PAGES_SERVER, &published),
            the_pages_reading(&point.sessions, THE_PAGES_SERVER, &opened),
        );

        // **Both together**: the send's placement record would carry the
        // first, and the page would compare against the second.
        assert_eq!(
            (the_send_read, the_page_read),
            (
                FourValuesNow::Answered(the_four_values_of_the_file(&published)),
                smoozy_render_settings::readings::FourValuesNow::Answered(
                    the_four_values_of_the_file(&opened)
                ),
            ),
            "(the send's reading, the page's reading)"
        );
        assert!(the_site_is_empty(&machine), "the link is taken away");
    }
}
