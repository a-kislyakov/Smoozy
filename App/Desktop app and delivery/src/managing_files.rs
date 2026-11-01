//! **`File Browser` §7's acts, done on the machine** — a new folder, a rename,
//! a delete, and cut, copy and paste — through `Server protocol and transfers`
//! T5's per-server point, with §8's telling written before each act's first
//! machine effect. Built at step 10c's Files' owed acts, pieces 3 to 6.
//!
//! **This file does and does not decide.** Every judgement is `File
//! Browser`'s: the name rule (`File Browser` M9), how a rename ended (`File
//! Browser` M4), the delete order (`File Browser` M2), what a unit owes the
//! check record (`File Browser` W11, `File Browser` X7a), what is written
//! against a job (`File Browser` W9).
//! What is here is the order those judgements are taken in, the machine calls
//! between them, and what is written down.
//!
//! **Which connection** (`File Browser` §7's head, hole 15): a new folder, a
//! rename, a delete's removals and a move's renames go on the reading session
//! as operations the user is waiting on.
//!
//! **One row at a time**: every act takes a list, and the screen hands it one
//! (`File Browser` §7's head).
//!
//! **No admin-access offer** is made beside a refusal in these pieces
//! (`Preparation/Build/Plan.md`, step 10c): the elevated retry is not built.
//!
//! **How a clause marker is written here**: every marker is written with its
//! document, because this crate's own plan defines an M-, a W- and an X-series
//! of nearly every number `File Browser` does.

use smoozy_file_browser::already_exists::is_inside;
use smoozy_file_browser::managing::{
    a_typed_name_is_refused, how_the_rename_ended, HowTheRenameEnded, Pasteboard, TheMachinesFiles,
    WhatTheMachineSaid, WhatTheUnitDid, WhyTheNameIsRefused,
};
use smoozy_file_browser::warnings::{
    what_is_written_against, AJobNamed, AUnitsOutcome, TheComposedReading,
};
use smoozy_queue_history::record::{ADisturbance, DisturbanceReading};
use smoozy_server_protocol::subsystem::SubsystemFailure;
use smoozy_state::identities::Identity;
use smoozy_state::outcomes::UnitOutcome;

use crate::servers::sessions::{AHeldConnection, ThisServersSessions};
use crate::uploading::TheRecordAnUploadWrites;

// ════════════════════════════════════════════════════════════════════════════
// The acts' names, as `Queue & History` N21's mark keeps them
// ════════════════════════════════════════════════════════════════════════════

/// **`File Browser` §11.1 `File Browser` B3's closed set**, as the mark's act intended — and
/// `Design/Queue & History § The marks` reads the verb from it.
pub const A_DELETE: &str = "delete";
pub const A_RENAME: &str = "rename";
pub const A_MOVE: &str = "move";
pub const A_COPY: &str = "copy";

// ════════════════════════════════════════════════════════════════════════════
// The telling, and the mark's finish (`File Browser` W8, `File Browser` W9)
// ════════════════════════════════════════════════════════════════════════════

/// **`File Browser` W8 for an act of §7: each named job's mark, written in one
/// save before the act's first machine effect**, every one reading *may have
/// been disturbed* (`Queue & History` N21). **One mark per member each job's
/// set was reached at**, naming that member.
///
/// **Answers whether the act may be attempted**: `true` where nothing was named
/// (there is no telling where no live-job warning was pushed past) or where the
/// marks stood; `false` where the write could not be made — and then the act
/// is not attempted (`File Browser` §11's ordered pair).
///
/// *(For a rename there is no §9.4 record to carry the jobs — a rename writes
/// no outcome record (`File Browser` W11's 2026-10-02 sentence) — so the marks
/// are the whole of the telling. For a delete, a move and a copy the act's own
/// record is piece 4's `file_acts` table.)*
pub fn the_marks_stand_first(
    record: &dyn TheRecordAnUploadWrites,
    named: &[AJobNamed],
    act: &str,
) -> bool {
    if named.is_empty() {
        return true;
    }
    let mut written = false;
    let reached = record.with_the_record(&mut |connection| {
        written = the_marks_are_written(connection, named, act).is_ok();
    });
    reached && written
}

/// The marks, in one save. **Both or none.**
pub fn the_marks_are_written(
    connection: &rusqlite::Connection,
    named: &[AJobNamed],
    act: &str,
) -> rusqlite::Result<()> {
    connection.execute_batch("SAVEPOINT the_marks_stand_first")?;
    let written = (|| -> rusqlite::Result<()> {
        for job in named {
            for member in &job.the_members_reached {
                crate::record::job_records::a_mark_is_appended_to(
                    connection,
                    &job.job,
                    &a_mark(member, act),
                )?;
            }
        }
        Ok(())
    })();
    match written {
        Ok(()) => connection.execute_batch("RELEASE the_marks_stand_first"),
        Err(error) => {
            let _ = connection.execute_batch(
                "ROLLBACK TO the_marks_stand_first; RELEASE the_marks_stand_first",
            );
            Err(error)
        }
    }
}

/// One unfinished mark.
pub fn a_mark(member: &[u8], act: &str) -> ADisturbance {
    ADisturbance {
        the_member: String::from_utf8_lossy(member).into_owned(),
        the_act_intended: act.to_string(),
        reading: DisturbanceReading::MayHaveBeenDisturbed,
        how_many: None,
    }
}

/// **`File Browser` W9: each named job's mark is finished from the units that
/// touched its own members.** `units` is every path the act touched with its
/// unit's outcome (none where its ending was never learned). A member a unit
/// took effect on reads *disturbed*; a member whose every unit ended without
/// effect has its mark removed; a member with a unit never learned keeps *may
/// have been disturbed*.
///
/// **A unit touches a member where either sits inside the other** — a file
/// inside a folder the job owns, or a folder holding the job's `.blend`, the
/// same containment both ways `File Browser` W5's trigger reads. **The
/// composition is `File Browser`'s own** (`what_is_written_against`), asked one
/// member at a time so each mark is finished from its own units.
pub fn the_marks_are_finished(
    record: &dyn TheRecordAnUploadWrites,
    named: &[AJobNamed],
    act: &str,
    units: &[(Vec<u8>, Option<UnitOutcome>)],
) {
    if named.is_empty() {
        return;
    }
    let mut finishing: Vec<(Identity, String, Option<TheComposedReading>)> = Vec::new();
    for job in named {
        for member in &job.the_members_reached {
            let its_own: Vec<AUnitsOutcome> = units
                .iter()
                .filter(|(touched, _)| is_inside(touched, member) || is_inside(member, touched))
                .map(|(_, outcome)| AUnitsOutcome {
                    the_member: member.clone(),
                    outcome: outcome.clone(),
                })
                .collect();
            let one = AJobNamed {
                the_members_reached: vec![member.clone()],
                ..job.clone()
            };
            finishing.push((
                job.job.clone(),
                String::from_utf8_lossy(member).into_owned(),
                what_is_written_against(&one, &its_own),
            ));
        }
    }
    record.with_the_record(&mut |connection| {
        let jobs = crate::record::job_records::JobRecords::over(connection);
        for (job, member, reading) in &finishing {
            let _ = match reading {
                Some(TheComposedReading::Disturbed) => jobs.a_mark_is_finished(job, member, act),
                Some(TheComposedReading::MayHaveBeenDisturbed) => Ok(0),
                None => jobs.a_mark_is_removed(job, member, act),
            };
        }
    });
}

/// **A delete's marks, finished — one per delete act, saying what went.** The
/// build runner's ruling (d) of 2026-10-03, written into `Design/Queue &
/// History § The marks`. Out of the Vast pass before 0.1.9, where deleting one
/// frame left a mark naming the job's whole folder.
///
/// **A member the delete reached from inside** — every unit touching it sits
/// beneath it, as a frame inside a job's output folder does — has its one mark
/// finished as what went: **the file, where one file went; the folder it went
/// from with the count, where more did** (the deepest folder the user deleted
/// that holds them all, or else the folder they share); and the folder itself
/// where only empty folders went. **Never one mark per file, and never the
/// job's whole folder for a single file.** Which files count follows the
/// mark's own reading (`File Browser` W9): those removed where any was, those
/// never answered where none was.
///
/// **Every other member** — a delete of a folder holding the job's `.blend`,
/// or of the member itself — is finished as [`the_marks_are_finished`] always
/// finished it, naming the member.
///
/// `entries` is every path the delete walked, with whether it is a folder and
/// its unit's outcome (none where its ending was never learned); `victims` is
/// what the user chose.
pub fn the_deletes_marks_are_finished(
    record: &dyn TheRecordAnUploadWrites,
    named: &[AJobNamed],
    entries: &[(Vec<u8>, bool, Option<UnitOutcome>)],
    victims: &[Vec<u8>],
) {
    use smoozy_file_browser::already_exists::paths_are_equal;
    if named.is_empty() {
        return;
    }
    let mut finished_as: Vec<(Identity, String, String, Option<u32>, DisturbanceReading)> = Vec::new();
    let mut the_rest: Vec<AJobNamed> = Vec::new();
    for job in named {
        let mut rest_of_this_job: Vec<Vec<u8>> = Vec::new();
        for member in &job.the_members_reached {
            let touching: Vec<&(Vec<u8>, bool, Option<UnitOutcome>)> = entries
                .iter()
                .filter(|(path, _, _)| is_inside(path, member) || is_inside(member, path))
                .collect();
            let from_inside = !touching.is_empty()
                && touching
                    .iter()
                    .all(|(path, _, _)| is_inside(path, member) && !paths_are_equal(path, member));
            if !from_inside {
                rest_of_this_job.push(member.clone());
                continue;
            }
            let reading = if touching.iter().any(|(_, _, outcome)| *outcome == Some(UnitOutcome::Done)) {
                DisturbanceReading::Disturbed
            } else if touching.iter().any(|(_, _, outcome)| outcome.is_none()) {
                DisturbanceReading::MayHaveBeenDisturbed
            } else {
                // **Nothing touching it took effect**: the mark goes, as `File Browser` W9 says.
                rest_of_this_job.push(member.clone());
                continue;
            };
            let counts = |one: &(Vec<u8>, bool, Option<UnitOutcome>)| match reading {
                DisturbanceReading::Disturbed => one.2 == Some(UnitOutcome::Done),
                DisturbanceReading::MayHaveBeenDisturbed => one.2.is_none(),
            };
            let files: Vec<&Vec<u8>> =
                touching.iter().filter(|one| !one.1 && counts(one)).map(|one| &one.0).collect();
            let (what, how_many) = match files.as_slice() {
                [one] => (String::from_utf8_lossy(one).into_owned(), None),
                [] => {
                    // Only folders went: the shallowest of them is named.
                    let shallowest = touching
                        .iter()
                        .filter(|one| counts(one))
                        .map(|one| &one.0)
                        .min_by_key(|path| path.split(|byte| *byte == b'/').filter(|part| !part.is_empty()).count())
                        .cloned()
                        .unwrap_or_else(|| member.clone());
                    (String::from_utf8_lossy(&shallowest).into_owned(), None)
                }
                many => {
                    let chosen = victims
                        .iter()
                        .filter(|victim| is_inside(victim, member) && !paths_are_equal(victim, member))
                        .filter(|victim| many.iter().all(|file| is_inside(file, victim)))
                        .max_by_key(|victim| victim.len())
                        .cloned()
                        .unwrap_or_else(|| the_folder_they_share(many));
                    let mut folder = String::from_utf8_lossy(&chosen).into_owned();
                    if !folder.ends_with('/') {
                        folder.push('/');
                    }
                    (folder, Some(u32::try_from(many.len()).unwrap_or(u32::MAX)))
                }
            };
            finished_as.push((
                job.job.clone(),
                String::from_utf8_lossy(member).into_owned(),
                what,
                how_many,
                reading,
            ));
        }
        if !rest_of_this_job.is_empty() {
            the_rest.push(AJobNamed {
                the_members_reached: rest_of_this_job,
                ..job.clone()
            });
        }
    }
    if !finished_as.is_empty() {
        record.with_the_record(&mut |connection| {
            let jobs = crate::record::job_records::JobRecords::over(connection);
            for (job, member, what, how_many, reading) in &finished_as {
                let _ = jobs.a_deletes_mark_is_finished_as(job, member, what, *how_many, *reading);
            }
        });
    }
    let units: Vec<(Vec<u8>, Option<UnitOutcome>)> =
        entries.iter().map(|(path, _, outcome)| (path.clone(), outcome.clone())).collect();
    the_marks_are_finished(record, &the_rest, A_DELETE, &units);
}

/// **The deepest folder every one of `paths` sits in**, compared per component.
fn the_folder_they_share(paths: &[&Vec<u8>]) -> Vec<u8> {
    let split = |path: &[u8]| -> Vec<Vec<u8>> {
        path.split(|byte| *byte == b'/').filter(|part| !part.is_empty()).map(<[u8]>::to_vec).collect()
    };
    let mut shared: Option<Vec<Vec<u8>>> = None;
    for path in paths {
        let mut parts = split(path);
        parts.pop();
        shared = Some(match shared {
            None => parts,
            Some(held) => held.into_iter().zip(parts).take_while(|(one, other)| one == other).map(|(one, _)| one).collect(),
        });
    }
    let mut out = Vec::new();
    for part in shared.unwrap_or_default() {
        out.push(b'/');
        out.extend(part);
    }
    if out.is_empty() {
        out.push(b'/');
    }
    out
}

// ════════════════════════════════════════════════════════════════════════════
// The check record (`File Browser` W11, `File Browser` X7a)
// ════════════════════════════════════════════════════════════════════════════

/// **What a unit of an act owes the check record, written where the running
/// application keeps it** — `File Browser` W11 and `File Browser` X7a through that module's
/// own `at_this_unit_of_a_managing_act`. The door is the caller's, because the
/// stores it reaches are not this file's.
pub trait WhenAUnitOfAnActEnds: Send + Sync {
    fn a_unit_ended(&self, location: &[u8], did: WhatTheUnitDid, outcome: &UnitOutcome);

    /// **Many units at once** — a delete's batch, which the running
    /// application marks in one reading of the record rather than one per
    /// entry. One at a time unless the door does better.
    fn units_ended(&self, units: &[(Vec<u8>, WhatTheUnitDid, UnitOutcome)]) {
        for (location, did, outcome) in units {
            self.a_unit_ended(location, *did, outcome);
        }
    }
}

/// A door that writes nothing, for an act with no record behind it.
pub struct NothingIsMarked;

impl WhenAUnitOfAnActEnds for NothingIsMarked {
    fn a_unit_ended(&self, _: &[u8], _: WhatTheUnitDid, _: &UnitOutcome) {}
}

// ════════════════════════════════════════════════════════════════════════════
// What an act came to
// ════════════════════════════════════════════════════════════════════════════

/// **What one act of a single item came to**, as the screen says it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheActCameTo {
    Done,
    /// `File Browser` M9's refusal before any attempt.
    TheNameIsRefused(WhyTheNameIsRefused),
    /// *Taken*: the listing showed it (a new folder, `File Browser` M9) or the
    /// machine refused over an existing name (`File Browser` M4).
    TheNameIsTaken,
    /// `File Browser` P1: the login may not.
    TheLoginMayNot,
    /// The thing acted on is not there.
    ItIsNotThere,
    /// Anything else the machine refused; nothing is concluded from it.
    TheMachineRefused,
    /// `File Browser` W8: the telling could not be written, so the act was not
    /// attempted.
    TheTellingWouldNotWrite,
    /// The connection lent nothing, or went while the act ran — its outcome
    /// was never learned.
    TheConnectionWent,
}

// ════════════════════════════════════════════════════════════════════════════
// The diary's line (`File Browser` B3)
// ════════════════════════════════════════════════════════════════════════════

/// **The name `File Browser`'s diary declaration is registered under**
/// (`Desktop app and delivery` A7), and the module its act lines are written
/// as.
pub const THE_MODULE: &str = "File Browser";

/// **`File Browser` B3's counts for an act of one item, from what it came to —
/// or nothing where no line is owed.** A line is one per act the user asked
/// for, and it is written where the act's answer was learned: done, refused
/// by the machine, or not attempted because the telling could not stand
/// (`File Browser` W8). **A name refused on the row before anything was sent,
/// a new folder the listing already showed, and an act whose answer never came
/// back write no line** — nothing was asked of the machine in the first two,
/// and in the third nothing is known to write down.
pub fn the_lines_counts(came_to: &WhatTheActCameTo) -> Option<smoozy_file_browser::diary::TheThreeCounts> {
    use smoozy_file_browser::diary::TheThreeCounts;
    let one = |done, refused, not_attempted| {
        Some(TheThreeCounts {
            done,
            refused,
            not_attempted,
        })
    };
    match came_to {
        WhatTheActCameTo::Done => one(1, 0, 0),
        WhatTheActCameTo::TheNameIsTaken
        | WhatTheActCameTo::TheLoginMayNot
        | WhatTheActCameTo::ItIsNotThere
        | WhatTheActCameTo::TheMachineRefused => one(0, 1, 0),
        WhatTheActCameTo::TheTellingWouldNotWrite => one(0, 0, 1),
        WhatTheActCameTo::TheNameIsRefused(_) | WhatTheActCameTo::TheConnectionWent => None,
    }
}

/// **`File Browser` B3's one line for an act, through the module's own diary**
/// — the run's identity, a fresh operation identity for the act, the server
/// row and its installation, and the jobs the telling named (`File Browser`
/// `File Browser` W8). **A module whose declaration is missing writes nothing**, which is
/// `Desktop app and delivery` A7's and is said by the service's answer.
pub fn the_act_is_written_down(
    started: &crate::start::Started,
    row: &smoozy_servers::store::ServerRow,
    act: smoozy_file_browser::diary::FileAct,
    about: smoozy_file_browser::diary::WhatTheActWasAbout,
    destination: Option<String>,
    counts: smoozy_file_browser::diary::TheThreeCounts,
    named: &[AJobNamed],
) -> Option<crate::diary_service::Wrote> {
    use smoozy_file_browser::diary::{AFileActCompleted, HowItRan, WhatTheFileActLineNames};
    use smoozy_state::identities::IdentityKind;
    let service = started.diary.as_ref()?;
    let run = started.run.clone()?;
    let server_row = Identity::read_back(IdentityKind::LocalServer, &row.local_server_identifier)?;
    let names = WhatTheFileActLineNames {
        run: run.clone(),
        act: Identity::issue(IdentityKind::Operation).ok()?,
        server_row,
        installation: row
            .last_confirmed_installation_identity
            .as_deref()
            .and_then(|text| Identity::read_back(IdentityKind::Installation, text)),
        jobs_the_telling_named: named.iter().map(|job| job.job.clone()).collect(),
    };
    let line = AFileActCompleted {
        act,
        at: std::time::SystemTime::now(),
        server_row_label: crate::surface::servers::what_a_server_is_called(row),
        about,
        destination,
        ran: HowItRan::Ordinary,
        counts,
    }
    .line(&names);
    Some(
        crate::diary_service::ModulesDiary {
            service,
            declarations: &started.declarations,
            run,
            module: THE_MODULE,
        }
        .write(&line),
    )
}

/// The unit's outcome for `File Browser` W9 and `File Browser` W11, from what the machine said.
fn the_units_outcome(said: &Result<(), WhatTheMachineSaid>) -> Option<UnitOutcome> {
    match said {
        Ok(()) => Some(UnitOutcome::Done),
        Err(WhatTheMachineSaid::TheChannelIsGone) => None,
        Err(_) => Some(UnitOutcome::Failed),
    }
}

/// **One call on the machine's files, on the reading session, as an operation
/// the user is waiting on.** A channel the call found gone is handed back as
/// gone, so the point opens a fresh session next time rather than reusing it.
pub async fn on_the_reading_session<C, F>(
    sessions: &ThisServersSessions<C>,
    call: F,
) -> Result<(), WhatTheMachineSaid>
where
    C: AHeldConnection,
    F: for<'a> FnOnce(
            &'a mut smoozy_server_protocol::subsystem::client::Subsystem<C::Channel>,
        ) -> std::pin::Pin<
            Box<dyn std::future::Future<Output = Result<(), WhatTheMachineSaid>> + Send + 'a>,
        > + Send
        + 'static,
{
    let read = sessions
        .a_reading(true, move |subsystem| {
            Box::pin(async move {
                match call(subsystem).await {
                    Err(WhatTheMachineSaid::TheChannelIsGone) => Err(SubsystemFailure::TheChannelIsGone),
                    other => Ok(other),
                }
            })
        })
        .await;
    match read {
        Ok(Ok(said)) => said,
        _ => Err(WhatTheMachineSaid::TheChannelIsGone),
    }
}

/// The path of `name` inside `folder`.
pub fn inside(folder: &[u8], name: &[u8]) -> Vec<u8> {
    let mut path = folder.to_vec();
    while path.len() > 1 && path.last() == Some(&b'/') {
        path.pop();
    }
    if path.last() != Some(&b'/') {
        path.push(b'/');
    }
    path.extend_from_slice(name);
    path
}

/// The folder holding `path`.
pub fn the_folder_holding(path: &[u8]) -> Vec<u8> {
    let mut trimmed = path;
    while trimmed.len() > 1 && trimmed.last() == Some(&b'/') {
        trimmed = &trimmed[..trimmed.len() - 1];
    }
    match trimmed.iter().rposition(|byte| *byte == b'/') {
        Some(0) => b"/".to_vec(),
        Some(at) => trimmed[..at].to_vec(),
        None => Vec::new(),
    }
}

/// The last component of `path`.
pub fn the_name_of(path: &[u8]) -> &[u8] {
    let mut trimmed = path;
    while trimmed.len() > 1 && trimmed.last() == Some(&b'/') {
        trimmed = &trimmed[..trimmed.len() - 1];
    }
    match trimmed.iter().rposition(|byte| *byte == b'/') {
        Some(at) => &trimmed[at + 1..],
        None => trimmed,
    }
}

// ════════════════════════════════════════════════════════════════════════════
// A new folder (`File Browser` M9)
// ════════════════════════════════════════════════════════════════════════════

/// **`File Browser` M9: a new folder is the plain make-directory**, never the
/// make-parents form. *Already exists* is answered before the attempt from the
/// listing the user is looking at — `the_listing_shows`, the names the list
/// holds — and by the plain form's own refusal where the race lands. **Nothing
/// is silently merged.** Atomic entire. Step 10c's Files' owed acts piece 3.
pub async fn a_folder_is_made<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    folder: &[u8],
    name: &[u8],
    the_listing_shows: &[Vec<u8>],
) -> WhatTheActCameTo {
    if let Some(why) = a_typed_name_is_refused(name) {
        return WhatTheActCameTo::TheNameIsRefused(why);
    }
    if the_listing_shows.iter().any(|shown| shown.as_slice() == name) {
        return WhatTheActCameTo::TheNameIsTaken;
    }
    let path = inside(folder, name);
    let said = on_the_reading_session(sessions, move |subsystem| {
        Box::pin(async move { subsystem.make_one_directory(&path).await })
    })
    .await;
    match said {
        Ok(()) => WhatTheActCameTo::Done,
        Err(WhatTheMachineSaid::ItIsAlreadyThere) => WhatTheActCameTo::TheNameIsTaken,
        Err(WhatTheMachineSaid::TheLoginMayNot) => WhatTheActCameTo::TheLoginMayNot,
        Err(WhatTheMachineSaid::ThePathIsNotThere) => WhatTheActCameTo::ItIsNotThere,
        Err(WhatTheMachineSaid::TheChannelIsGone) => WhatTheActCameTo::TheConnectionWent,
        Err(WhatTheMachineSaid::ItRefused) => WhatTheActCameTo::TheMachineRefused,
    }
}

// ════════════════════════════════════════════════════════════════════════════
// A rename (`File Browser` M4)
// ════════════════════════════════════════════════════════════════════════════

/// **`File Browser` M4: a rename is the base rename, and its own refusal is
/// what reports *taken*.** The listing only hints, on the page; the rename is
/// attempted whatever it showed. Atomic per item. Step 10c's Files' owed acts
/// piece 3.
///
/// **In this order**: `File Browser` M9's name rule; `File Browser` W8's
/// marks for every job `named`, before the rename; the rename; `File Browser`
/// `File Browser` W9's finish from its one unit; and `File Browser` W11's mark at the place it
/// left (`WhatTheUnitDid::MovedAwayFromIt`), only where it took
/// effect.
pub async fn a_rename_is_made<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    record: &dyn TheRecordAnUploadWrites,
    marking: &dyn WhenAUnitOfAnActEnds,
    path: &[u8],
    name: &[u8],
    named: &[AJobNamed],
) -> WhatTheActCameTo {
    if let Some(why) = a_typed_name_is_refused(name) {
        return WhatTheActCameTo::TheNameIsRefused(why);
    }
    if the_name_of(path) == name {
        return WhatTheActCameTo::Done;
    }
    if !the_marks_stand_first(record, named, A_RENAME) {
        return WhatTheActCameTo::TheTellingWouldNotWrite;
    }
    let from = path.to_vec();
    let to = inside(&the_folder_holding(path), name);
    let said = on_the_reading_session(sessions, move |subsystem| {
        Box::pin(async move { subsystem.base_rename(&from, &to).await })
    })
    .await;
    let outcome = the_units_outcome(&said);
    the_marks_are_finished(record, named, A_RENAME, &[(path.to_vec(), outcome.clone())]);
    if let Some(outcome) = outcome.as_ref() {
        marking.a_unit_ended(path, WhatTheUnitDid::MovedAwayFromIt, outcome);
    }
    match how_the_rename_ended(said) {
        HowTheRenameEnded::Done => WhatTheActCameTo::Done,
        HowTheRenameEnded::TheNameIsTaken => WhatTheActCameTo::TheNameIsTaken,
        HowTheRenameEnded::Refused(WhatTheMachineSaid::TheLoginMayNot) => WhatTheActCameTo::TheLoginMayNot,
        HowTheRenameEnded::Refused(WhatTheMachineSaid::ThePathIsNotThere) => WhatTheActCameTo::ItIsNotThere,
        HowTheRenameEnded::Refused(WhatTheMachineSaid::TheChannelIsGone) => WhatTheActCameTo::TheConnectionWent,
        HowTheRenameEnded::Refused(_) => WhatTheActCameTo::TheMachineRefused,
    }
}

// ════════════════════════════════════════════════════════════════════════════
// A delete (`File Browser` M1 to `File Browser` M3, `File Browser` X12) — step
// 10c's Files' owed acts piece 4
// ════════════════════════════════════════════════════════════════════════════

/// **How many removals ride the reading session at once.** The far end is a
/// whole round trip away, and a removal that waits for its answer before the
/// next is sent moves one entry per round trip — a folder of tens of thousands
/// of cache files is then that many round trips. **Order is still kept**: the
/// removals at one depth go together, and a folder is asked for only once
/// every entry deeper than it has answered, so *folders removed empty,
/// bottom-up* (`File Browser` M2) holds by construction rather than by the far
/// end's own ordering.
pub const THE_REMOVALS_IN_FLIGHT: usize = 64;

/// **One entry a delete will remove** — the victim itself or anything under
/// it, found by the walk.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnEntryToRemove {
    pub path: Vec<u8>,
    /// A folder, removed once empty. **A link is never a folder here**: it is
    /// removed as itself and never followed (`File Browser` §7's head).
    pub is_a_folder: bool,
    /// How many components deep, which is the order removals go in.
    pub depth: usize,
}

/// **The walk: the victim and everything under it, one reading per folder,
/// hidden names included, links never followed** (`File Browser` §7's head,
/// `File Browser` L21). A folder the login may not read is still an entry:
/// its own removal is then what the machine refuses.
pub async fn the_tree_under<S>(
    subsystem: &mut smoozy_server_protocol::subsystem::client::Subsystem<S>,
    victim: &[u8],
) -> Result<Vec<AnEntryToRemove>, SubsystemFailure>
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send,
{
    use smoozy_file_browser::listing::Kind;
    let depth = |path: &[u8]| smoozy_file_browser::already_exists::components(path).len();
    let attributes = subsystem.lstat(victim).await?;
    let mut found = vec![AnEntryToRemove {
        path: victim.to_vec(),
        is_a_folder: Kind::from_mode(attributes.permissions) == Kind::Folder,
        depth: depth(victim),
    }];
    let mut folders: Vec<Vec<u8>> = if found[0].is_a_folder { vec![victim.to_vec()] } else { Vec::new() };
    while let Some(folder) = folders.pop() {
        let entries = match subsystem.list(&folder).await {
            Ok(entries) => entries,
            Err(SubsystemFailure::TheChannelIsGone) => return Err(SubsystemFailure::TheChannelIsGone),
            // **A folder that will not be read is left as it stands**: its own
            // removal answers for it.
            Err(_) => continue,
        };
        for entry in entries {
            if entry.name == b"." || entry.name == b".." {
                continue;
            }
            let path = inside(&folder, &entry.name);
            let is_a_folder = Kind::from_mode(entry.attributes.permissions) == Kind::Folder;
            if is_a_folder {
                folders.push(path.clone());
            }
            let depth = depth(&path);
            found.push(AnEntryToRemove { path, is_a_folder, depth });
        }
    }
    Ok(found)
}

/// **What a delete is asked for**, as the screen hands it over.
#[derive(Debug, Clone)]
pub struct ADeleteAsked {
    pub victims: Vec<Vec<u8>>,
    pub server_row: Identity,
    pub installation: Option<Identity>,
    pub run: Identity,
}

/// **What a delete came to.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheDeleteCameTo {
    /// Nothing was removed, for the reason given — the victim not there, the
    /// login may not read it, the record would not take the act, or the line
    /// went before the walk ended.
    NothingWasRemoved(WhatTheActCameTo),
    /// **It ran.** `File Browser` M2's report, per entry, as counts: removed,
    /// refused by the machine, not attempted, and never learned.
    Ran {
        record: Identity,
        done: usize,
        refused: usize,
        not_attempted: usize,
        never_learned: usize,
    },
}

/// Why a unit was not attempted, as its outcome record names it.
pub const SOMETHING_INSIDE_IT_STAYED: &str = "something inside it was not removed";
pub const AN_UNOWNED_STAGING_ENTRY: &str = "an unowned staging entry";
pub const THE_CONNECTION_WENT_FIRST: &str = "the connection went";

/// **`File Browser` M2: a delete, per entry, bottom-up, partial across
/// entries** — through the server's own point, on the reading session.
///
/// **In this order:**
/// 1. the walk, which reads and changes nothing;
/// 2. `File Browser` X12: an entry inside a Smoozy staging folder that no
///    transfer record names is not removed by a delete of a folder above it —
///    it is an unowned staging entry, removed only when it is the victim
///    itself;
/// 3. the act's record — its enumeration, `File Browser` W8's telling and each
///    named job's mark, in one write (`State, identity and recovery` P5 item
///    1). **A write that will not go down is the delete not attempted**;
/// 4. `File Browser` X12's ordering: every live transfer record whose staging
///    is to be removed reads *stopped: staging removed by user* first, with
///    its clean-up owed, and that owed clean-up is cleared once the removal
///    took effect;
/// 5. the removals, depth by depth, [`THE_REMOVALS_IN_FLIGHT`] at a time,
///    each batch's outcomes written as it answers, and `File Browser` W11's
///    mark asked for every entry that went;
/// 6. `File Browser` W9: each named job's mark finished from its own units.
#[allow(clippy::too_many_arguments)]
pub async fn a_delete_is_made<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    record: &dyn TheRecordAnUploadWrites,
    marking: &dyn WhenAUnitOfAnActEnds,
    asked: &ADeleteAsked,
    named: &[AJobNamed],
    in_flight: usize,
    moved: &(dyn Fn(usize, usize) + Sync),
) -> WhatTheDeleteCameTo {
    use crate::record::file_acts::{AUnit, AnAct, FileActs, WhichAct, ONE_ENTRY};
    use std::collections::HashSet;

    // 1. The walk.
    let mut entries: Vec<AnEntryToRemove> = Vec::new();
    for victim in &asked.victims {
        let victim = victim.clone();
        let walked = sessions
            .a_reading(true, move |subsystem| Box::pin(async move { the_tree_under(subsystem, &victim).await }))
            .await;
        match walked {
            Ok(Ok(found)) => entries.extend(found),
            Ok(Err(SubsystemFailure::TheServerRefused(smoozy_server_protocol::subsystem::StatusCode::NoSuchFile))) => {
                continue
            }
            Ok(Err(SubsystemFailure::TheServerRefused(
                smoozy_server_protocol::subsystem::StatusCode::PermissionDenied,
            ))) => return WhatTheDeleteCameTo::NothingWasRemoved(WhatTheActCameTo::TheLoginMayNot),
            Ok(Err(SubsystemFailure::TheChannelIsGone)) | Err(_) => {
                return WhatTheDeleteCameTo::NothingWasRemoved(WhatTheActCameTo::TheConnectionWent)
            }
            Ok(Err(_)) => return WhatTheDeleteCameTo::NothingWasRemoved(WhatTheActCameTo::TheMachineRefused),
        }
    }
    if entries.is_empty() {
        return WhatTheDeleteCameTo::NothingWasRemoved(WhatTheActCameTo::ItIsNotThere);
    }
    // **Deepest first**, stable within one depth, which is the order the
    // units run in and so the order of the enumeration (`State, identity and
    // recovery` §9.3).
    entries.sort_by(|one, other| other.depth.cmp(&one.depth));

    // 2. What the transfer records name, under the machine's current
    // installation.
    let mut transfers: Vec<smoozy_server_protocol::transfers::TransferRecord> = Vec::new();
    record.with_the_record(&mut |connection| {
        transfers = crate::record::transfer_records::all(connection).unwrap_or_default();
    });
    transfers.retain(|one| one.installation.as_ref() == asked.installation.as_ref());
    let owners_of = |path: &[u8]| -> Vec<usize> {
        transfers
            .iter()
            .enumerate()
            .filter(|(_, one)| {
                one.every_staging_path_it_has_named()
                    .iter()
                    .any(|staging| smoozy_file_browser::already_exists::paths_are_equal(&staging.as_bytes(), path))
            })
            .map(|(at, _)| at)
            .collect()
    };
    let owners: Vec<Vec<usize>> = entries.iter().map(|entry| owners_of(&entry.path)).collect();
    let is_a_victim = |path: &[u8]| {
        asked
            .victims
            .iter()
            .any(|victim| smoozy_file_browser::already_exists::paths_are_equal(victim, path))
    };

    // 3. The act's record, its telling and the marks, in one write.
    let marks: Vec<(Identity, smoozy_queue_history::record::ADisturbance)> = named
        .iter()
        .flat_map(|job| job.the_members_reached.iter().map(move |member| (job.job.clone(), a_mark(member, A_DELETE))))
        .collect();
    let act = AnAct {
        act: WhichAct::Delete,
        server_row: asked.server_row.clone(),
        installation: asked.installation.clone(),
        opened_by_run: asked.run.clone(),
        units: entries
            .iter()
            .map(|entry| AUnit {
                role: ONE_ENTRY,
                source: entry.path.clone(),
                destination: None,
                staging: None,
            })
            .collect(),
        the_jobs_told: named.iter().map(|job| job.job.clone()).collect(),
    };
    let mut written: Option<Identity> = None;
    let reached = record.with_the_record(&mut |connection| {
        written = FileActs::over(connection).the_act_is_written(&act, &marks).ok();
    });
    let Some(the_record) = written.filter(|_| reached) else {
        return WhatTheDeleteCameTo::NothingWasRemoved(WhatTheActCameTo::TheTellingWouldNotWrite);
    };

    // 4. `File Browser` X12: stopped first, for every live record whose
    // staging goes.
    let mut stopped: Vec<usize> = Vec::new();
    for owned in &owners {
        for &at in owned {
            let one = &mut transfers[at];
            let live = matches!(
                one.ending,
                None | Some(smoozy_server_protocol::transfers::Ending::Interrupted)
            );
            if live && !stopped.contains(&at) {
                one.ending = Some(crate::downloading::THE_USERS_STOP);
                one.cleanup_is_owed = true;
                let one = one.clone();
                record.with_the_record(&mut |connection| {
                    let _ = crate::record::transfer_records::put(connection, &one);
                });
                stopped.push(at);
            }
        }
    }

    // 5. The removals, depth by depth.
    let total = entries.len();
    let mut blocked: HashSet<Vec<u8>> = HashSet::new();
    let mut outcomes: Vec<Option<UnitOutcome>> = vec![None; total];
    let mut gone = false;
    let in_flight = in_flight.max(1);
    let mut at = 0;
    let block_above = |blocked: &mut HashSet<Vec<u8>>, path: &[u8]| {
        let mut holder = the_folder_holding(path);
        while !holder.is_empty() {
            if !blocked.insert(holder.clone()) || holder == b"/" {
                break;
            }
            holder = the_folder_holding(&holder);
        }
    };
    while at < total {
        let depth = entries[at].depth;
        let level_ends = entries[at..].iter().position(|entry| entry.depth != depth).map_or(total, |n| at + n);
        let mut level = at;
        while level < level_ends {
            let batch_ends = (level + in_flight).min(level_ends);
            let mut ended: Vec<(usize, UnitOutcome)> = Vec::new();
            let mut to_send: Vec<(usize, Vec<u8>, bool)> = Vec::new();
            for position in level..batch_ends {
                let entry = &entries[position];
                let reason = if gone {
                    Some(THE_CONNECTION_WENT_FIRST)
                } else if entry.is_a_folder && blocked.contains(&entry.path) {
                    Some(SOMETHING_INSIDE_IT_STAYED)
                } else if smoozy_file_browser::partial::is_staging_material(&entry.path)
                    && !entry.is_a_folder
                    && owners[position].is_empty()
                    && !is_a_victim(&entry.path)
                {
                    Some(AN_UNOWNED_STAGING_ENTRY)
                } else {
                    None
                };
                match reason {
                    Some(why) => ended.push((position, UnitOutcome::NotAttempted { stopped_by: why.to_string() })),
                    None => to_send.push((position, entry.path.clone(), entry.is_a_folder)),
                }
            }
            if !to_send.is_empty() {
                let sent = to_send.clone();
                let answered = sessions
                    .a_reading(true, move |subsystem| {
                        Box::pin(async move {
                            let mut ids = Vec::new();
                            for (position, path, is_a_folder) in &sent {
                                match subsystem.send_a_removal(path, *is_a_folder).await {
                                    Ok(id) => ids.push((*position, id)),
                                    Err(failure) => return Err(failure),
                                }
                            }
                            let mut answers = Vec::new();
                            for (position, id) in ids {
                                answers.push((position, subsystem.the_removal_answered(id).await));
                            }
                            Ok(answers)
                        })
                    })
                    .await;
                match answered {
                    Ok(Ok(answers)) => {
                        for (position, answer) in answers {
                            match answer {
                                Ok(()) => ended.push((position, UnitOutcome::Done)),
                                Err(SubsystemFailure::TheChannelIsGone) => gone = true,
                                Err(_) => ended.push((position, UnitOutcome::Failed)),
                            }
                        }
                    }
                    // **The line went with removals in flight**: their
                    // outcomes were never learned and stay unwritten.
                    _ => gone = true,
                }
            }
            for (position, outcome) in &ended {
                outcomes[*position] = Some(outcome.clone());
            }
            for position in level..batch_ends {
                if outcomes[position] != Some(UnitOutcome::Done) {
                    block_above(&mut blocked, &entries[position].path);
                }
            }
            record.with_the_record(&mut |connection| {
                let _ = FileActs::over(connection).these_units_ended(&the_record, &ended);
            });
            let removed: Vec<(Vec<u8>, WhatTheUnitDid, UnitOutcome)> = ended
                .iter()
                .filter(|(_, outcome)| *outcome == UnitOutcome::Done)
                .map(|(position, outcome)| (entries[*position].path.clone(), WhatTheUnitDid::RemovedIt, outcome.clone()))
                .collect();
            marking.units_ended(&removed);
            for at in &stopped {
                let one = &mut transfers[*at];
                if one.cleanup_is_owed
                    && one.every_staging_path_it_has_named().iter().all(|staging| {
                        removed
                            .iter()
                            .any(|(path, _, _)| smoozy_file_browser::already_exists::paths_are_equal(path, &staging.as_bytes()))
                    })
                {
                    one.cleanup_is_owed = false;
                    let one = one.clone();
                    record.with_the_record(&mut |connection| {
                        let _ = crate::record::transfer_records::put(connection, &one);
                    });
                }
            }
            level = batch_ends;
            moved(outcomes.iter().filter(|one| one.is_some()).count(), total);
        }
        at = level_ends;
    }

    // 6. `File Browser` W9 — one mark per delete act, saying what went (the
    // build runner's ruling (d) of 2026-10-03).
    let units: Vec<(Vec<u8>, bool, Option<UnitOutcome>)> = entries
        .iter()
        .zip(&outcomes)
        .map(|(entry, outcome)| (entry.path.clone(), entry.is_a_folder, outcome.clone()))
        .collect();
    the_deletes_marks_are_finished(record, named, &units, &asked.victims);

    let count = |wanted: fn(&Option<UnitOutcome>) -> bool| outcomes.iter().filter(|one| wanted(one)).count();
    WhatTheDeleteCameTo::Ran {
        record: the_record,
        done: count(|one| *one == Some(UnitOutcome::Done)),
        refused: count(|one| *one == Some(UnitOutcome::Failed)),
        not_attempted: count(|one| matches!(one, Some(UnitOutcome::NotAttempted { .. }))),
        never_learned: count(Option::is_none),
    }
}

// ════════════════════════════════════════════════════════════════════════════
// The pasteboard and a paste after a cut (`File Browser` M5, `File Browser` M6, `File Browser` M8, `File Browser` M11 to
// `File Browser` M18) — step 10c's Files' owed acts piece 5
// ════════════════════════════════════════════════════════════════════════════

/// **`File Browser` M11: one pasteboard, holding one thing, for the run** —
/// and `File Browser` M13: its life is the connected period's, so it is let go of when that
/// server's period ends ([`the_pasteboard_goes_with_the_period`]). **A drop
/// does not clear it, and nor does a paste after a copy; a paste after a cut
/// lets it go once the move has run** (`File Browser` M14,
/// [`the_paste_has_come_to`]). Nothing durable is written from it.
fn the_pasteboard() -> &'static std::sync::Mutex<Option<(String, Pasteboard)>> {
    static THE_ONE: std::sync::Mutex<Option<(String, Pasteboard)>> = std::sync::Mutex::new(None);
    &THE_ONE
}

/// **Cut or Copy** puts the entries on the pasteboard, replacing whatever was
/// there (`File Browser` M11, `File Browser` M12): the server row and installation it was
/// taken under, which act, and the paths as the listing gave them.
pub fn the_pasteboard_takes(server: &str, pasteboard: Pasteboard) {
    if let Ok(mut held) = the_pasteboard().lock() {
        *held = Some((server.to_string(), pasteboard));
    }
}

/// What the pasteboard holds, where it holds anything.
pub fn what_the_pasteboard_holds() -> Option<(String, Pasteboard)> {
    the_pasteboard().lock().ok().and_then(|held| held.clone())
}

/// **`File Browser` M13: the pasteboard ends with the connected period of the
/// server it was taken on**, and with nothing else.
pub fn the_pasteboard_goes_with_the_period(server: &str) {
    if let Ok(mut held) = the_pasteboard().lock() {
        if held.as_ref().is_some_and(|(on, _)| on == server) {
            *held = None;
        }
    }
}

/// **`File Browser` M14: a paste after a cut lets go of the pasteboard once the
/// move has run**, because what was cut is no longer where it was. Cancel at
/// the question, a refusal before anything ran, and a move that never started
/// keep it; a paste after a copy never clears it. Only the cut that ran is let
/// go of: one taken since, or another server's, stays.
pub fn the_paste_has_come_to(server: &str, pasteboard: &Pasteboard, came_to: &WhatThePasteCameTo) {
    if pasteboard.act != smoozy_file_browser::managing::CutOrCopy::Cut
        || !matches!(came_to, WhatThePasteCameTo::Ran { .. })
    {
        return;
    }
    if let Ok(mut held) = the_pasteboard().lock() {
        if held.as_ref().is_some_and(|(on, now)| on == server && now == pasteboard) {
            *held = None;
        }
    }
}

/// **`File Browser` M15: how many entries Paste carries on this server, or
/// nothing where it is not offered at all** — the browsing server row and
/// installation identity must be the pasteboard's.
pub fn paste_is_offered_with(server_row: &Identity, installation_now: Option<&Identity>) -> Option<usize> {
    let (_, pasteboard) = what_the_pasteboard_holds()?;
    pasteboard
        .paste_is_offered(server_row, installation_now)
        .then_some(pasteboard.entries.len())
}

/// **Why a paste is refused before anything is sent** — `Design/File Browser
/// § What the screen says around an act`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyThePasteIsRefused {
    /// A folder pasted into itself, or into a folder inside it.
    AFolderIntoItself,
}

/// **One rename a move will make** — a top-level item, or an item inside a
/// folder that merges (`File Browser` M5's paragraph of 2026-10-02).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AMove {
    pub from: Vec<u8>,
    pub to: Vec<u8>,
    /// Whether something stands at `to` — a collision, decided by the answer.
    pub collides: bool,
    /// **`File Browser` M8's 2026-10-02 paragraph**: a file onto a folder or a
    /// folder onto a file is never replaced.
    pub the_kinds_differ: bool,
    pub is_a_folder: bool,
}

/// **What a paste will do, read on the machine at the moment of the act**
/// (`File Browser` M8, `File Browser` X5).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ThePastePlanned {
    pub moves: Vec<AMove>,
    /// Folders a merge empties, deepest first, removed once their contents
    /// moved — and only if empty.
    pub emptied: Vec<Vec<u8>>,
    /// Entries no longer there (`File Browser` M14: a cut pasted twice finds
    /// its source gone).
    pub gone: Vec<Vec<u8>>,
}

impl ThePastePlanned {
    /// The collisions, as `File Browser` X3's comparison the box is drawn
    /// from.
    pub fn the_collisions(&self) -> Vec<smoozy_file_browser::already_exists::AComparison> {
        use smoozy_file_browser::already_exists::{AComparison, Comparison};
        self.moves
            .iter()
            .map(|one| AComparison {
                landing_path: one.to.clone(),
                what: match (one.collides, one.the_kinds_differ, one.is_a_folder) {
                    (false, _, _) => Comparison::LandsFresh,
                    (true, false, _) => Comparison::CollidesWithAFile,
                    (true, true, true) => Comparison::AFolderOnAFile,
                    (true, true, false) => Comparison::LandsOnASameNamedFolder,
                },
            })
            .collect()
    }
}

/// **The plan, read on the reading session**: each entry's kind, then what
/// stands at its name in the destination. A folder onto a same-named folder
/// goes down a level and plans each item inside by the same rule.
pub async fn the_paste_is_planned<S>(
    subsystem: &mut smoozy_server_protocol::subsystem::client::Subsystem<S>,
    entries: &[Vec<u8>],
    into: &[u8],
) -> Result<ThePastePlanned, SubsystemFailure>
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send,
{
    use smoozy_file_browser::listing::Kind;
    let mut planned = ThePastePlanned::default();
    let mut pending: Vec<(Vec<u8>, Vec<u8>)> = entries
        .iter()
        .map(|entry| (entry.clone(), into.to_vec()))
        .collect();
    while let Some((from, into)) = pending.pop() {
        let already_here = smoozy_file_browser::already_exists::paths_are_equal(&the_folder_holding(&from), &into);
        let source = match subsystem.lstat(&from).await {
            Ok(attributes) => Kind::from_mode(attributes.permissions),
            Err(SubsystemFailure::TheChannelIsGone) => return Err(SubsystemFailure::TheChannelIsGone),
            Err(_) => {
                planned.gone.push(from);
                continue;
            }
        };
        // **Already in the folder it would be pasted into, and still there**:
        // nothing to move (`Design/File Browser § What the screen says around
        // an act`). One that is gone was counted just above.
        if already_here {
            continue;
        }
        let to = inside(&into, the_name_of(&from));
        let there = match subsystem.lstat(&to).await {
            Ok(attributes) => Some(Kind::from_mode(attributes.permissions)),
            Err(SubsystemFailure::TheChannelIsGone) => return Err(SubsystemFailure::TheChannelIsGone),
            Err(_) => None,
        };
        let is_a_folder = source == Kind::Folder;
        match there {
            Some(Kind::Folder) if is_a_folder => {
                // **A merge**: down a level, each item inside by the same rule.
                let inside_it = subsystem.list(&from).await?;
                for entry in inside_it {
                    if entry.name == b"." || entry.name == b".." {
                        continue;
                    }
                    pending.push((self::inside(&from, &entry.name), to.clone()));
                }
                planned.emptied.push(from);
            }
            Some(kind) => planned.moves.push(AMove {
                from,
                to,
                collides: true,
                the_kinds_differ: (kind == Kind::Folder) != is_a_folder,
                is_a_folder,
            }),
            None => planned.moves.push(AMove {
                from,
                to,
                collides: false,
                the_kinds_differ: false,
                is_a_folder,
            }),
        }
    }
    // Deepest first, so a folder emptied inside another is removed before it.
    planned
        .emptied
        .sort_by_key(|folder| std::cmp::Reverse(smoozy_file_browser::already_exists::components(folder).len()));
    Ok(planned)
}

/// **What a paste after a cut came to.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatThePasteCameTo {
    /// Refused before anything was sent.
    Refused(WhyThePasteIsRefused),
    /// **A cut pasted back into the folder it came from**: nothing to move.
    NothingToMove,
    /// **The question** (`File Browser` M8): some items collide, and the user
    /// answers Replace or Cancel (a copy also Keep both).
    AsksFirst(Vec<smoozy_file_browser::already_exists::AComparison>),
    /// The user answered Cancel.
    Cancelled,
    /// Nothing was moved, for the reason given.
    NothingWasMoved(WhatTheActCameTo),
    /// **It ran**: moved, refused (a taken name the machine refused, or
    /// anything else it refused), not done (a different kind under Replace, a
    /// crossing, or a source gone), and never learned.
    Ran {
        record: Identity,
        done: usize,
        refused: usize,
        not_done: usize,
        never_learned: usize,
        /// How many of the not done were crossings between filesystems.
        crossings: usize,
        /// **Smoozy on the server is older than the copy request** — Design's
        /// *Update* rather than *Set up* in the crossings' sentence.
        crossings_want_an_update: bool,
    },
}

/// Why a move's unit was not done, as its outcome record names it.
pub const ON_ANOTHER_FILESYSTEM: &str = "it is on another disk";
pub const THE_KINDS_DIFFER: &str = "a file and a folder are never replaced by each other";
pub const THE_SOURCE_IS_GONE: &str = "it is not there any more";

/// **A paste after a cut is a move** (`File Browser` M5): one rename per
/// top-level item, or per item inside a folder that merges; create-intended
/// where nothing collides, replace-intended where the user answered Replace.
///
/// **In this order:** the refusals before anything is sent; the plan, read on
/// the machine; the question where anything collides and no answer was
/// given; the act's record with `File Browser` W8's telling and marks; the
/// renames, each read under `File Browser` M5's rule where it is refused; `File Browser`
/// `File Browser` W11's and `File Browser` X7a's marks for each move that took effect — the place it left,
/// and its landing; the folders a merge emptied, removed only if empty; and
/// `File Browser` W9's finish.
///
/// **A move across filesystems** is `File Browser` M6's ordered copy-then-delete pair, which
/// waits on the copy (piece 6); **until then it is reported not done, and
/// nothing is lost.**
#[allow(clippy::too_many_arguments)]
pub async fn a_move_is_made<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    record: &dyn TheRecordAnUploadWrites,
    marking: &dyn WhenAUnitOfAnActEnds,
    pasteboard: &Pasteboard,
    into: &[u8],
    answer: Option<smoozy_file_browser::already_exists::CollisionAnswer>,
    named: &[AJobNamed],
    run: &Identity,
    package_area: Option<&str>,
) -> WhatThePasteCameTo {
    use crate::record::file_acts::{AUnit, AnAct, FileActs, WhichAct, ONE_TOP_LEVEL_ITEM};
    use smoozy_file_browser::already_exists::{is_inside, CollisionAnswer};
    use smoozy_file_browser::managing::{how_the_move_ended, HowTheMoveEnded};
    use smoozy_server_protocol::transfers::publication::Intent;

    // **Refused before anything is sent.**
    if pasteboard.entries.iter().any(|entry| is_inside(into, entry)) {
        return WhatThePasteCameTo::Refused(WhyThePasteIsRefused::AFolderIntoItself);
    }
    // **A cut pasted back where it came from is not decided here**: the plan
    // below reads the machine, and only an item still there is *already in
    // this folder*. The case this was first written for was an item still
    // in its folder. The case it now also meets is a cut already moved
    // elsewhere, pasted back: the pass before 0.1.9 on Vast read *It is
    // already in this folder* when the file was in another one.

    // The plan, on the machine, at the moment of the act.
    let entries = pasteboard.entries.clone();
    let into_owned = into.to_vec();
    let planned = sessions
        .a_reading(true, move |subsystem| {
            Box::pin(async move { the_paste_is_planned(subsystem, &entries, &into_owned).await })
        })
        .await;
    let planned = match planned {
        Ok(Ok(planned)) => planned,
        _ => return WhatThePasteCameTo::NothingWasMoved(WhatTheActCameTo::TheConnectionWent),
    };
    let collisions = planned.moves.iter().filter(|one| one.collides).count();
    match answer {
        _ if collisions == 0 => {}
        None => return WhatThePasteCameTo::AsksFirst(planned.the_collisions()),
        Some(CollisionAnswer::Stop) => return WhatThePasteCameTo::Cancelled,
        Some(_) => {}
    }
    let replace = answer == Some(CollisionAnswer::Replace);
    if planned.moves.is_empty() && planned.gone.is_empty() {
        return WhatThePasteCameTo::NothingToMove;
    }

    // The act's record, with `File Browser` W8's telling and the marks, in one write.
    let marks: Vec<(Identity, smoozy_queue_history::record::ADisturbance)> = named
        .iter()
        .flat_map(|job| job.the_members_reached.iter().map(move |member| (job.job.clone(), a_mark(member, A_MOVE))))
        .collect();
    let units: Vec<AUnit> = planned
        .moves
        .iter()
        .map(|one| AUnit {
            role: ONE_TOP_LEVEL_ITEM,
            source: one.from.clone(),
            destination: Some(one.to.clone()),
            staging: None,
        })
        .chain(planned.gone.iter().map(|gone| AUnit {
            role: ONE_TOP_LEVEL_ITEM,
            source: gone.clone(),
            destination: None,
            staging: None,
        }))
        .collect();
    let act = AnAct {
        act: WhichAct::Move,
        server_row: pasteboard.server_row.clone(),
        installation: pasteboard.installation.clone(),
        opened_by_run: run.clone(),
        units,
        the_jobs_told: named.iter().map(|job| job.job.clone()).collect(),
    };
    let mut written: Option<Identity> = None;
    let reached = record.with_the_record(&mut |connection| {
        written = FileActs::over(connection).the_act_is_written(&act, &marks).ok();
    });
    let Some(the_record) = written.filter(|_| reached) else {
        return WhatThePasteCameTo::NothingWasMoved(WhatTheActCameTo::TheTellingWouldNotWrite);
    };

    // The renames.
    let mut ended: Vec<(usize, UnitOutcome)> = Vec::new();
    let mut touched: Vec<(Vec<u8>, Option<UnitOutcome>)> = Vec::new();
    let mut crossings = 0;
    let mut crossings_want_an_update = false;
    let mut gone = false;
    for (position, one) in planned.moves.iter().enumerate() {
        if gone {
            ended.push((position, UnitOutcome::NotAttempted { stopped_by: THE_CONNECTION_WENT_FIRST.to_string() }));
            continue;
        }
        if one.collides && (!replace || one.the_kinds_differ) {
            ended.push((position, UnitOutcome::NotAttempted { stopped_by: THE_KINDS_DIFFER.to_string() }));
            continue;
        }
        let intent = if one.collides { Intent::ReplaceIntended } else { Intent::CreateIntended };
        let (from, to) = (one.from.clone(), one.to.clone());
        let read = sessions
            .a_reading(true, move |subsystem| {
                Box::pin(async move {
                    use smoozy_file_browser::managing::TheMachinesFiles;
                    let said = match intent {
                        Intent::ReplaceIntended => subsystem.replacing_rename(&from, &to).await,
                        Intent::CreateIntended => subsystem.base_rename(&from, &to).await,
                    };
                    let something_is_there = match &said {
                        Ok(()) => true,
                        Err(_) => subsystem.lstat(&to).await.is_ok(),
                    };
                    Ok((said, something_is_there))
                })
            })
            .await;
        let Ok(Ok((said, something_is_there))) = read else {
            gone = true;
            touched.push((one.from.clone(), None));
            continue;
        };
        let outcome = match how_the_move_ended(said, intent, something_is_there) {
            HowTheMoveEnded::Done => UnitOutcome::Done,
            // **`File Browser` M6's ordered pair** (step 10c's Files' owed acts
            // piece 6): copied under `File Browser` M7's rule, then the source removed only
            // where its copy published.
            HowTheMoveEnded::ItCrossesFilesystems => {
                let pair = the_pair_is_made(
                    sessions,
                    record,
                    marking,
                    pasteboard,
                    &one.from,
                    &the_folder_holding(&one.to),
                    intent == Intent::ReplaceIntended,
                    run,
                    package_area,
                )
                .await;
                if let UnitOutcome::NotAttempted { stopped_by } = &pair {
                    crossings += 1;
                    crossings_want_an_update |= stopped_by == SMOOZY_THERE_IS_OLDER;
                }
                pair
            }
            HowTheMoveEnded::Refused(WhatTheMachineSaid::TheChannelIsGone) => {
                gone = true;
                touched.push((one.from.clone(), None));
                continue;
            }
            HowTheMoveEnded::TheNameIsTaken | HowTheMoveEnded::Refused(_) => UnitOutcome::Failed,
        };
        if outcome == UnitOutcome::Done {
            marking.units_ended(&[
                (one.from.clone(), WhatTheUnitDid::MovedAwayFromIt, UnitOutcome::Done),
                (one.to.clone(), WhatTheUnitDid::PublishedOverIt, UnitOutcome::Done),
            ]);
        }
        touched.push((one.from.clone(), Some(outcome.clone())));
        touched.push((one.to.clone(), Some(outcome.clone())));
        ended.push((position, outcome));
        record.with_the_record(&mut |connection| {
            let _ = FileActs::over(connection).these_units_ended(&the_record, &ended[ended.len() - 1..]);
        });
    }
    let base = planned.moves.len();
    for (at, _) in planned.gone.iter().enumerate() {
        ended.push((base + at, UnitOutcome::NotAttempted { stopped_by: THE_SOURCE_IS_GONE.to_string() }));
    }
    record.with_the_record(&mut |connection| {
        let _ = FileActs::over(connection).these_units_ended(&the_record, &ended);
    });

    // The folders a merge emptied, only if empty — the plain removal refuses
    // one that is not.
    if !gone {
        for folder in &planned.emptied {
            let path = folder.clone();
            let removed = sessions
                .a_reading(true, move |subsystem| Box::pin(async move { Ok(subsystem.remove_directory(&path).await) }))
                .await;
            if matches!(removed, Ok(Ok(Ok(())))) {
                marking.units_ended(&[(folder.clone(), WhatTheUnitDid::MovedAwayFromIt, UnitOutcome::Done)]);
            }
        }
    }

    the_marks_are_finished(record, named, A_MOVE, &touched);
    let count = |wanted: fn(&UnitOutcome) -> bool| ended.iter().filter(|(_, one)| wanted(one)).count();
    WhatThePasteCameTo::Ran {
        record: the_record,
        done: count(|one| *one == UnitOutcome::Done),
        refused: count(|one| *one == UnitOutcome::Failed),
        not_done: count(|one| matches!(one, UnitOutcome::NotAttempted { .. })),
        never_learned: planned.moves.len() + planned.gone.len() - ended.len(),
        crossings,
        crossings_want_an_update,
    }
}

// ════════════════════════════════════════════════════════════════════════════
// A copy on the machine, and a move across disks (`File Browser` M6, `File Browser` M7, `File Browser` M8,
// `File Browser` X9) — step 10c's Files' owed acts piece 6
// ════════════════════════════════════════════════════════════════════════════

/// **One file or link a copy will write** — `File Browser` §11's unit, *one
/// file*.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ACopyUnit {
    pub from: Vec<u8>,
    pub to: Vec<u8>,
    /// A link is copied as a link to the same target (`File Browser` §7's
    /// head).
    pub is_a_link: bool,
    /// Something stands at `to`; the answer decides.
    pub collides: bool,
    /// **`File Browser` M8's 2026-10-02 paragraph**: never replaced by the
    /// other kind.
    pub the_kinds_differ: bool,
}

/// **What a copy will do, read on the machine at the moment of the act**.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TheCopyPlanned {
    /// Folders to make, shallowest first, with the plain make-folder; one
    /// already there is a merge and is left as it is.
    pub folders: Vec<Vec<u8>>,
    pub units: Vec<ACopyUnit>,
    /// Top-level items no longer there.
    pub gone: Vec<Vec<u8>>,
    /// Top-level folders that collide with a file, or fall under Keep both —
    /// collisions the plan can name but that hold more than one unit.
    pub folder_collisions: Vec<Vec<u8>>,
}

impl TheCopyPlanned {
    /// The collisions, as `File Browser` X3's comparison the box is drawn from.
    pub fn the_collisions(&self) -> Vec<smoozy_file_browser::already_exists::AComparison> {
        use smoozy_file_browser::already_exists::{AComparison, Comparison};
        self.units
            .iter()
            .filter(|one| one.collides)
            .map(|one| AComparison {
                landing_path: one.to.clone(),
                what: if one.the_kinds_differ {
                    Comparison::LandsOnASameNamedFolder
                } else {
                    Comparison::CollidesWithAFile
                },
            })
            .chain(self.folder_collisions.iter().map(|to| AComparison {
                landing_path: to.clone(),
                what: Comparison::AFolderOnAFile,
            }))
            .collect()
    }
}

/// **`File Browser` X9's counted name for Keep both**, found free by reading
/// the destination: `name (2)`, `name (3)`, … with a file's extension kept
/// after the count.
async fn a_free_name_beside<S>(
    subsystem: &mut smoozy_server_protocol::subsystem::client::Subsystem<S>,
    to: &[u8],
    is_a_folder: bool,
) -> Result<Vec<u8>, SubsystemFailure>
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send,
{
    use smoozy_file_browser::keep_both::split;
    use smoozy_file_browser::listing::EntryName;
    let folder = the_folder_holding(to);
    let split = split(&EntryName::from_the_machine(the_name_of(to).to_vec()), is_a_folder);
    for counter in 2..10_000u32 {
        let mut name = split.stem.clone();
        name.extend_from_slice(format!(" ({counter})").as_bytes());
        name.extend_from_slice(&split.rest);
        let candidate = inside(&folder, &name);
        match subsystem.lstat(&candidate).await {
            Ok(_) => continue,
            Err(SubsystemFailure::TheChannelIsGone) => return Err(SubsystemFailure::TheChannelIsGone),
            Err(_) => return Ok(candidate),
        }
    }
    Err(SubsystemFailure::TheAnswerCouldNotBeRead)
}

/// **The copy's plan, read on the reading session** (`File Browser` M8, `File Browser` X5).
/// A folder onto a same-named folder merges, as the spec says a copy does;
/// **a copy onto itself** — pasted into its own folder — is the ordinary
/// question, and Keep both is Design's duplicate. `keep_both` composes the
/// counted name for whatever collides.
pub async fn the_copy_is_planned<S>(
    subsystem: &mut smoozy_server_protocol::subsystem::client::Subsystem<S>,
    entries: &[Vec<u8>],
    into: &[u8],
    keep_both: bool,
) -> Result<TheCopyPlanned, SubsystemFailure>
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send,
{
    use smoozy_file_browser::listing::Kind;
    let mut planned = TheCopyPlanned::default();
    // (from, to, whether `to` may already stand and must be read)
    let mut pending: Vec<(Vec<u8>, Vec<u8>, bool, bool)> = entries
        .iter()
        .rev()
        .map(|entry| (entry.clone(), inside(into, the_name_of(entry)), true, true))
        .collect();
    while let Some((from, to, read_it, top_level)) = pending.pop() {
        let kind = match subsystem.lstat(&from).await {
            Ok(attributes) => Kind::from_mode(attributes.permissions),
            Err(SubsystemFailure::TheChannelIsGone) => return Err(SubsystemFailure::TheChannelIsGone),
            Err(_) if top_level => {
                planned.gone.push(from);
                continue;
            }
            Err(_) => continue,
        };
        let is_a_folder = kind == Kind::Folder;
        let there = if read_it {
            match subsystem.lstat(&to).await {
                Ok(attributes) => Some(Kind::from_mode(attributes.permissions)),
                Err(SubsystemFailure::TheChannelIsGone) => return Err(SubsystemFailure::TheChannelIsGone),
                Err(_) => None,
            }
        } else {
            None
        };
        let onto_itself = smoozy_file_browser::already_exists::paths_are_equal(&from, &to);
        let (to, there) = match there {
            Some(_) if keep_both && (onto_itself || !(is_a_folder && there == Some(Kind::Folder))) => {
                (a_free_name_beside(subsystem, &to, is_a_folder).await?, None)
            }
            other => (to, other),
        };
        if is_a_folder {
            match there {
                // **A merge**: down a level, reading each name inside.
                Some(Kind::Folder) if !onto_itself => {}
                Some(_) => {
                    planned.folder_collisions.push(to);
                    continue;
                }
                None => planned.folders.push(to.clone()),
            }
            let merging = there.is_some();
            let mut inside_it = subsystem.list(&from).await?;
            inside_it.retain(|entry| entry.name != b"." && entry.name != b"..");
            for entry in inside_it.iter().rev() {
                pending.push((self::inside(&from, &entry.name), self::inside(&to, &entry.name), merging, false));
            }
            continue;
        }
        planned.units.push(ACopyUnit {
            from,
            to,
            is_a_link: kind == Kind::Link,
            collides: there.is_some(),
            the_kinds_differ: there == Some(Kind::Folder),
        });
    }
    Ok(planned)
}

/// **What a copy came to.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheCopyCameTo {
    Refused(WhyThePasteIsRefused),
    AsksFirst(Vec<smoozy_file_browser::already_exists::AComparison>),
    Cancelled,
    /// **`File Browser` M7's paragraph of 2026-10-02**: copy-data is not
    /// announced and nothing on the machine declares the copy request, so
    /// nothing is attempted — and which of the two it is, because Design says
    /// *Set up* for one and *Update* for the other.
    TheServerCannotCopy(WhyTheServerCannotCopy),
    NothingWasCopied(WhatTheActCameTo),
    Ran {
        record: Identity,
        done: usize,
        refused: usize,
        not_done: usize,
        never_learned: usize,
    },
}

/// Why a copy's unit was not done, as its outcome record names it.
pub const THE_KINDS_ARE_NOT_SWAPPED: &str = THE_KINDS_DIFFER;
pub const IT_WAS_NOT_ANSWERED: &str = "nothing was answered for it";
/// **Why a server cannot copy files on itself** (`File Browser` M7's
/// paragraph of 2026-10-02), as the outcome record of a move's unit names it.
pub const SMOOZY_IS_NOT_SET_UP_THERE: &str = "the server cannot copy files on itself";
pub const SMOOZY_THERE_IS_OLDER: &str =
    "the server cannot copy files on itself until Smoozy there is updated";

/// **Which of `File Browser` M7's two absences it is**: no copy-data, and
/// either no Smoozy program on the machine to ask, or one older than the copy
/// request, which refuses it by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyTheServerCannotCopy {
    /// Before hand-over: there is no program to ask.
    SmoozyIsNotSetUpThere,
    /// A program placed before the request existed — Design's *Update*.
    SmoozyThereIsOlder,
}

impl WhyTheServerCannotCopy {
    /// The words a move's unit's outcome record keeps.
    pub fn as_recorded(self) -> &'static str {
        match self {
            WhyTheServerCannotCopy::SmoozyIsNotSetUpThere => SMOOZY_IS_NOT_SET_UP_THERE,
            WhyTheServerCannotCopy::SmoozyThereIsOlder => SMOOZY_THERE_IS_OLDER,
        }
    }
}

/// **How a copy's bytes move on the machine** — `File Browser` M7: the file
/// service's copy-data where the connection announced it, and otherwise
/// `Server protocol and transfers` §1.1's request *a file is copied on the
/// machine*, asked of Smoozy's own program at this package area. **Never
/// across the wire.** Step 10c's Files' owed acts piece 7 added the second.
pub enum TheCopysRoute<'a, C: AHeldConnection> {
    CopyData,
    TheProgram {
        sessions: &'a ThisServersSessions<C>,
        package_area: &'a str,
    },
}

/// **How long the desktop waits on one file's copy by the program** —
/// `Server protocol and transfers` C16's per-waiter deadline, which the plan
/// pass asked to be sized for a long copy rather than an ordinary answer.
/// **A Build decision**: one hour per file, where 1 GB took 1.4 s at the desk
/// on the file service's own copy. **Its expiry changes nothing durable**
/// (`Server protocol and transfers` C17): the unit is never learned, its
/// staging is left where the program may still be writing it, and that is
/// `File Browser` X12's to clear.
pub const HOW_LONG_ONE_FILES_COPY_IS_WAITED_FOR: std::time::Duration =
    std::time::Duration::from_secs(60 * 60);

/// Why one unit of a copy did not land.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ItDidNotLand {
    /// The channel went: nothing more is learned on it.
    TheChannelWent,
    /// The machine refused or failed it, and the staging was removed.
    Refused,
    /// **The program on the machine refused the copy request by name** — it
    /// is older than the request.
    TheProgramIsOlder,
    /// **The waiter's deadline passed** with no answer, so the staging is left.
    NeverAnswered,
}

impl From<SubsystemFailure> for ItDidNotLand {
    fn from(failure: SubsystemFailure) -> Self {
        match failure {
            SubsystemFailure::TheChannelIsGone => ItDidNotLand::TheChannelWent,
            _ => ItDidNotLand::Refused,
        }
    }
}

/// **`Server protocol and transfers` §1.1's request, asked of the program at
/// the package area**: the source copied to the staging path, flushed to disk,
/// and answered. The entry point answers it itself (the paragraph after
/// `Rented-server runtime` W2), so no supervisor need be running.
async fn the_program_copies<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    package_area: &str,
    from: &[u8],
    staged: &[u8],
) -> Result<(), ItDidNotLand> {
    use smoozy_server_protocol::asking_the_supervisor::WhatCameBack;
    use smoozy_server_protocol::framing::{Fields, Request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS};
    use smoozy_server_protocol::requests as asked;
    let Ok(identity) = Identity::issue(smoozy_state::identities::IdentityKind::Operation) else {
        return Err(ItDidNotLand::Refused);
    };
    let request = Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        identity,
        asked::THE_FILE_IS_COPIED_ON_THE_MACHINE_REQUEST,
    )
    .carrying(
        Fields::none()
            .and(asked::THE_COPYS_SOURCE_FIELD, from)
            .and(asked::THE_COPYS_STAGING_PATH_FIELD, staged),
    );
    let asked_it = sessions.ask_the_supervisor(package_area, &request);
    match tokio::time::timeout(HOW_LONG_ONE_FILES_COPY_IS_WAITED_FOR, asked_it).await {
        Err(_) => Err(ItDidNotLand::NeverAnswered),
        Ok(WhatCameBack::ItAnswered(answered))
            if answered.the_request_name_it_says_it_does_not_answer().is_some() =>
        {
            Err(ItDidNotLand::TheProgramIsOlder)
        }
        Ok(WhatCameBack::ItAnswered(answered))
            if answered.fields.text(asked::THE_COPYS_OUTCOME_FIELD) == Some(asked::IT_WAS_COPIED) =>
        {
            Ok(())
        }
        // **The connection went with the ask out, so its answer was never
        // learned** — added 2026-10-19, the dead-line desk coverage, first
        // half. The ask comes back with nothing on a line that went silent
        // after the program had copied, and that was read as the machine
        // refusing: the unit recorded *refused by the server* and its staging
        // removed, though the copy may stand whole. **It is the waiter's
        // expiry's case instead** (`Server protocol and transfers` C17): the
        // unit is never learned and its staging is left for `File Browser`
        // X12's sweep. **Only where the point no longer lends**, so a link
        // that could not be planted on a live connection is still a refusal.
        Ok(WhatCameBack::NothingCorrelatedCameBack | WhatCameBack::ItWasNeverAsked)
            if !sessions.a_connection_is_live() =>
        {
            Err(ItDidNotLand::NeverAnswered)
        }
        Ok(_) => Err(ItDidNotLand::Refused),
    }
}

/// **How long the desktop waits on one folder's count** — `Server protocol
/// and transfers` C16's per-waiter deadline, chosen knowing the walk reports
/// nothing until it ends (`File Browser` L28). **A Build decision**: the same
/// hour as one file's copy. Its expiry changes nothing: the row is left
/// uncounted and says so. Added 2026-10-30, the twenty-third notes, note 2.
pub const HOW_LONG_ONE_FOLDERS_COUNT_IS_WAITED_FOR: std::time::Duration =
    std::time::Duration::from_secs(60 * 60);

/// **What one folder's count found** — `File Browser` L25's four figures.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhatTheCountFound {
    pub bytes: u64,
    pub files: u64,
    pub folders: u64,
    /// A folder below could not be read, so the figures are a lower bound.
    pub incomplete: bool,
}

/// **What asking for one folder's count came to.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheCountCameTo {
    Counted(WhatTheCountFound),
    /// Nothing is at the path any more.
    NothingThere,
    /// The path is not a folder.
    NotAFolder,
    /// The folder itself could not be read, in the machine's own words.
    NotRead(String),
    /// The program on the machine refused the request by name.
    TheProgramIsOlder,
    /// The deadline passed, or the line went with the ask out.
    NeverAnswered,
    /// Anything else the machine answered.
    Refused,
}

/// **`Server protocol and transfers` §1.1's *a folder's size is counted on the
/// machine*, asked of the program at the package area** — `File Browser` L25.
/// The entry point answers it itself, so no supervisor need be running.
pub async fn the_program_counts_a_folder<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    package_area: &str,
    folder: &[u8],
) -> WhatTheCountCameTo {
    use smoozy_server_protocol::asking_the_supervisor::WhatCameBack;
    use smoozy_server_protocol::framing::{Fields, Request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS};
    use smoozy_server_protocol::requests as asked;
    let Ok(identity) = Identity::issue(smoozy_state::identities::IdentityKind::Operation) else {
        return WhatTheCountCameTo::Refused;
    };
    let request = Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        identity,
        asked::THE_FOLDERS_SIZE_IS_COUNTED_REQUEST,
    )
    .carrying(Fields::none().and(asked::THE_COUNTED_FOLDER_FIELD, folder));
    let asked_it = sessions.ask_the_supervisor(package_area, &request);
    match tokio::time::timeout(HOW_LONG_ONE_FOLDERS_COUNT_IS_WAITED_FOR, asked_it).await {
        Err(_) => WhatTheCountCameTo::NeverAnswered,
        Ok(WhatCameBack::ItAnswered(answered)) => what_the_count_answer_says(&answered),
        Ok(WhatCameBack::NothingCorrelatedCameBack | WhatCameBack::ItWasNeverAsked)
            if !sessions.a_connection_is_live() =>
        {
            WhatTheCountCameTo::NeverAnswered
        }
        Ok(_) => WhatTheCountCameTo::Refused,
    }
}

/// **One count's answer, read** — kept apart from the asking so it can be
/// driven without a machine.
pub fn what_the_count_answer_says(answered: &smoozy_server_protocol::framing::Answered) -> WhatTheCountCameTo {
    use smoozy_server_protocol::requests as asked;
    if answered.the_request_name_it_says_it_does_not_answer().is_some() {
        return WhatTheCountCameTo::TheProgramIsOlder;
    }
    let figure = |field: &str| answered.fields.text(field).and_then(|text| text.parse::<u64>().ok());
    match answered.fields.text(asked::THE_COUNTS_OUTCOME_FIELD) {
        Some(outcome) if outcome == asked::IT_WAS_COUNTED => match (
            figure(asked::THE_BYTES_COUNTED_FIELD),
            figure(asked::THE_FILES_COUNTED_FIELD),
            figure(asked::THE_FOLDERS_COUNTED_FIELD),
        ) {
            (Some(bytes), Some(files), Some(folders)) => WhatTheCountCameTo::Counted(WhatTheCountFound {
                bytes,
                files,
                folders,
                incomplete: answered.fields.text(asked::THE_COUNT_IS_INCOMPLETE_FIELD)
                    != Some(asked::THE_COUNT_IS_WHOLE),
            }),
            _ => WhatTheCountCameTo::Refused,
        },
        Some(outcome) if outcome == asked::NOTHING_IS_THERE => WhatTheCountCameTo::NothingThere,
        Some(outcome) if outcome == asked::IT_IS_NOT_A_FOLDER => WhatTheCountCameTo::NotAFolder,
        Some(outcome) if outcome == asked::IT_WAS_NOT_READ => WhatTheCountCameTo::NotRead(
            answered
                .fields
                .text(asked::THE_MACHINES_OWN_WORDS_FIELD)
                .unwrap_or_default()
                .to_string(),
        ),
        _ => WhatTheCountCameTo::Refused,
    }
}

/// **One copy per file, on a transfer channel of its own, never across the
/// wire** — `File Browser` M7: opened to read; the staging path made with the
/// exclusive create in Smoozy's staging folder beside the destination; the
/// bytes copied by the server (copy-data) or by Smoozy's own program on it
/// (piece 7's route); flushed to disk; then **published by one rename**, base
/// where nothing collides and replacing where Replace was answered. A link is
/// planted at the staging path pointing where the source points, and published
/// the same way, on either route. **A failure anywhere removes the staging and
/// the unit is refused** — except where no answer came, when the program may
/// still be writing it.
async fn one_unit_is_copied<S, C>(
    subsystem: &mut smoozy_server_protocol::subsystem::client::Subsystem<S>,
    route: &TheCopysRoute<'_, C>,
    unit: &ACopyUnit,
    staging: &smoozy_file_browser::managing::CopyStagingPath,
    replace: bool,
) -> Result<(), ItDidNotLand>
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send,
    C: AHeldConnection,
{
    let staged = staging.as_bytes();
    match subsystem.make_directory(staging.directory().as_bytes()).await {
        Ok(()) | Err(SubsystemFailure::TheServerRefused(smoozy_server_protocol::subsystem::StatusCode::Failure)) => {}
        Err(failure) => return Err(failure.into()),
    }
    let written: Result<(), ItDidNotLand> = async {
        if unit.is_a_link {
            let target = subsystem.read_link(&unit.from).await?;
            subsystem.plant_a_link(&target, &staged).await?;
        } else if let TheCopysRoute::TheProgram { sessions, package_area } = route {
            the_program_copies(sessions, package_area, &unit.from, &staged).await?;
        } else {
            let from = subsystem.open_to_read(&unit.from).await?;
            let to = match subsystem.create_exclusively(&staged).await {
                Ok(handle) => handle,
                Err(failure) => {
                    let _ = subsystem.close(&from).await;
                    return Err(failure.into());
                }
            };
            let copied = subsystem.copy_data(&from, &to).await;
            let durable = match copied {
                Ok(()) => match subsystem.make_durable(&to).await {
                    Ok(()) | Err(SubsystemFailure::ThatCapabilityWasNotAnnounced) => Ok(()),
                    Err(failure) => Err(failure),
                },
                Err(failure) => Err(failure),
            };
            let _ = subsystem.close(&from).await;
            let _ = subsystem.close(&to).await;
            durable?;
        }
        if replace {
            subsystem.posix_rename(&staged, &unit.to).await?;
        } else {
            subsystem.rename(&staged, &unit.to).await?;
        }
        Ok(())
    }
    .await;
    if let Err(why) = &written {
        if matches!(why, ItDidNotLand::Refused | ItDidNotLand::TheProgramIsOlder) {
            let _ = subsystem.remove(&staged).await;
        }
    }
    let _ = subsystem.remove_directory(staging.directory().as_bytes()).await;
    written
}

/// **What the copy itself did**, unit by unit — shared by a paste after a
/// copy and the copy half of `File Browser` M6's pair.
struct TheCopyRan {
    outcomes: Vec<Option<UnitOutcome>>,
    /// **The program on the machine refused the copy request by name** at
    /// the first file, so nothing was copied and the rest were not tried.
    the_program_is_older: bool,
}

/// **The units, run**: folders made first, then each unit in order on one
/// transfer channel, each outcome written as it ends, `File Browser` X7a and
/// `File Browser` W11 asked for every landing.
#[allow(clippy::too_many_arguments)]
async fn the_copy_runs<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    record: &dyn TheRecordAnUploadWrites,
    marking: &dyn WhenAUnitOfAnActEnds,
    the_record: &Identity,
    planned: &TheCopyPlanned,
    staging: &[smoozy_file_browser::managing::CopyStagingPath],
    replace: bool,
    route: &TheCopysRoute<'_, C>,
    moved: &(dyn Fn(usize, usize) + Sync),
) -> TheCopyRan {
    use crate::record::file_acts::FileActs;
    let total = planned.units.len();
    let mut outcomes: Vec<Option<UnitOutcome>> = vec![None; total];
    let mut the_program_is_older = false;
    let folders = planned.folders.clone();
    let made = sessions
        .a_reading(true, move |subsystem| {
            Box::pin(async move {
                for folder in &folders {
                    match subsystem.make_directory(folder).await {
                        Ok(())
                        | Err(SubsystemFailure::TheServerRefused(
                            smoozy_server_protocol::subsystem::StatusCode::Failure,
                        )) => {}
                        Err(failure) => return Ok(Some(failure)),
                    }
                }
                Ok(None)
            })
        })
        .await;
    let the_folders_stood = matches!(made, Ok(Ok(None)));
    let mut channel = match sessions.a_transfer_channel().await {
        Ok(channel) => smoozy_server_protocol::subsystem::client::Subsystem::negotiate(channel).await.ok(),
        Err(_) => None,
    };
    for (position, unit) in planned.units.iter().enumerate() {
        let outcome = if unit.collides && (!replace || unit.the_kinds_differ) {
            Some(UnitOutcome::NotAttempted {
                stopped_by: if unit.the_kinds_differ { THE_KINDS_ARE_NOT_SWAPPED } else { IT_WAS_NOT_ANSWERED }.to_string(),
            })
        } else if !the_folders_stood {
            Some(UnitOutcome::NotAttempted { stopped_by: "its folder could not be made".to_string() })
        } else if the_program_is_older && !unit.is_a_link {
            Some(UnitOutcome::NotAttempted { stopped_by: SMOOZY_THERE_IS_OLDER.to_string() })
        } else if let Some(subsystem) = channel.as_mut() {
            match one_unit_is_copied(subsystem, route, unit, &staging[position], unit.collides).await {
                Ok(()) => Some(UnitOutcome::Done),
                Err(ItDidNotLand::TheChannelWent) => {
                    channel = None;
                    None
                }
                Err(ItDidNotLand::NeverAnswered) => None,
                Err(ItDidNotLand::TheProgramIsOlder) => {
                    the_program_is_older = true;
                    Some(UnitOutcome::NotAttempted { stopped_by: SMOOZY_THERE_IS_OLDER.to_string() })
                }
                Err(ItDidNotLand::Refused) => Some(UnitOutcome::Failed),
            }
        } else {
            Some(UnitOutcome::NotAttempted { stopped_by: THE_CONNECTION_WENT_FIRST.to_string() })
        };
        if let Some(outcome) = &outcome {
            record.with_the_record(&mut |connection| {
                let _ = FileActs::over(connection).these_units_ended(the_record, &[(position, outcome.clone())]);
            });
            if *outcome == UnitOutcome::Done {
                marking.units_ended(&[(unit.to.clone(), WhatTheUnitDid::PublishedOverIt, UnitOutcome::Done)]);
            }
        }
        outcomes[position] = outcome;
        moved(position + 1, total);
    }
    TheCopyRan { outcomes, the_program_is_older }
}

/// **How this server's copy will be made, or why it cannot be** — `File
/// Browser` M7's paragraph of 2026-10-02. Whether the server announces
/// copy-data is read off a transfer channel's own opening — `Server protocol
/// and transfers` §1.2: held against the period, never guessed. **Where it
/// does not, Smoozy's own program is asked**, where the row records a package
/// area; **where it records none, Smoozy is not set up there** and nothing is
/// attempted. **Nothing** where no channel could be had.
async fn how_the_copy_is_made<'a, C: AHeldConnection>(
    sessions: &'a ThisServersSessions<C>,
    package_area: Option<&'a str>,
) -> Option<Result<TheCopysRoute<'a, C>, WhyTheServerCannotCopy>> {
    let channel = sessions.a_transfer_channel().await.ok()?;
    let subsystem = smoozy_server_protocol::subsystem::client::Subsystem::negotiate(channel).await.ok()?;
    if subsystem
        .announcement()
        .announces(smoozy_server_protocol::subsystem::extension::COPY_DATA)
    {
        return Some(Ok(TheCopysRoute::CopyData));
    }
    Some(match package_area {
        Some(package_area) => Ok(TheCopysRoute::TheProgram { sessions, package_area }),
        None => Err(WhyTheServerCannotCopy::SmoozyIsNotSetUpThere),
    })
}

/// **A paste after a copy** — `File Browser` M7, `File Browser` M8 and `File Browser` X9: copies on the
/// machine, never across the wire; *Keep both* gives the counted name, and a
/// copy pasted into its own folder is the duplicate. **In this order:** the
/// refusal of a folder into itself; whether the server can copy; the plan;
/// the question where anything collides and no answer was given; the act's
/// record, its telling and marks; the copy, unit by unit; and `File Browser`
/// `File Browser` W9's finish. **A single file reports its two ends and nothing between**
/// (`File Browser` M7); many report per file.
///
/// **`package_area` is where Smoozy's own program sits on that server**, from
/// the row, and nothing where the row records none — the route a copy takes
/// where copy-data is not announced (step 10c's Files' owed acts piece 7).
#[allow(clippy::too_many_arguments)]
pub async fn a_copy_is_made<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    record: &dyn TheRecordAnUploadWrites,
    marking: &dyn WhenAUnitOfAnActEnds,
    pasteboard: &Pasteboard,
    into: &[u8],
    answer: Option<smoozy_file_browser::already_exists::CollisionAnswer>,
    named: &[AJobNamed],
    run: &Identity,
    package_area: Option<&str>,
    moved: &(dyn Fn(usize, usize) + Sync),
) -> WhatTheCopyCameTo {
    use crate::record::file_acts::{AUnit, AnAct, FileActs, WhichAct, ONE_FILE};
    use smoozy_file_browser::already_exists::{is_inside, CollisionAnswer};
    if pasteboard
        .entries
        .iter()
        .any(|entry| is_inside(into, entry))
    {
        return WhatTheCopyCameTo::Refused(WhyThePasteIsRefused::AFolderIntoItself);
    }
    let route = match how_the_copy_is_made(sessions, package_area).await {
        Some(Ok(route)) => route,
        Some(Err(why)) => return WhatTheCopyCameTo::TheServerCannotCopy(why),
        None => return WhatTheCopyCameTo::NothingWasCopied(WhatTheActCameTo::TheConnectionWent),
    };
    if answer == Some(CollisionAnswer::Stop) {
        return WhatTheCopyCameTo::Cancelled;
    }
    let keep_both = answer == Some(CollisionAnswer::KeepBoth);
    let entries = pasteboard.entries.clone();
    let into_owned = into.to_vec();
    let planned = sessions
        .a_reading(true, move |subsystem| {
            Box::pin(async move { the_copy_is_planned(subsystem, &entries, &into_owned, keep_both).await })
        })
        .await;
    let Ok(Ok(planned)) = planned else {
        return WhatTheCopyCameTo::NothingWasCopied(WhatTheActCameTo::TheConnectionWent);
    };
    let collisions = planned.units.iter().filter(|one| one.collides).count() + planned.folder_collisions.len();
    if collisions > 0 && answer.is_none() {
        return WhatTheCopyCameTo::AsksFirst(planned.the_collisions());
    }
    let replace = answer == Some(CollisionAnswer::Replace);

    let marks: Vec<(Identity, smoozy_queue_history::record::ADisturbance)> = named
        .iter()
        .flat_map(|job| job.the_members_reached.iter().map(move |member| (job.job.clone(), a_mark(member, A_COPY))))
        .collect();
    let staging: Vec<smoozy_file_browser::managing::CopyStagingPath> = planned
        .units
        .iter()
        .map(|unit| {
            let one = Identity::issue(smoozy_state::identities::IdentityKind::Operation)
                .unwrap_or_else(|_| run.clone());
            smoozy_file_browser::managing::CopyStagingPath::synthesise(
                &String::from_utf8_lossy(&the_folder_holding(&unit.to)),
                &one,
            )
        })
        .collect();
    let act = AnAct {
        act: WhichAct::Copy,
        server_row: pasteboard.server_row.clone(),
        installation: pasteboard.installation.clone(),
        opened_by_run: run.clone(),
        units: planned
            .units
            .iter()
            .zip(&staging)
            .map(|(unit, staging)| AUnit {
                role: ONE_FILE,
                source: unit.from.clone(),
                destination: Some(unit.to.clone()),
                staging: Some(staging.as_bytes()),
            })
            .collect(),
        the_jobs_told: named.iter().map(|job| job.job.clone()).collect(),
    };
    let mut written: Option<Identity> = None;
    let reached = record.with_the_record(&mut |connection| {
        written = FileActs::over(connection).the_act_is_written(&act, &marks).ok();
    });
    let Some(the_record) = written.filter(|_| reached) else {
        return WhatTheCopyCameTo::NothingWasCopied(WhatTheActCameTo::TheTellingWouldNotWrite);
    };
    let ran = the_copy_runs(sessions, record, marking, &the_record, &planned, &staging, replace, &route, moved).await;
    let units: Vec<(Vec<u8>, Option<UnitOutcome>)> = planned
        .units
        .iter()
        .zip(&ran.outcomes)
        .map(|(unit, outcome)| (unit.to.clone(), outcome.clone()))
        .collect();
    the_marks_are_finished(record, named, A_COPY, &units);
    let count = |wanted: fn(&Option<UnitOutcome>) -> bool| ran.outcomes.iter().filter(|one| wanted(one)).count();
    // **An older program copied nothing**, and Design's *Update* sentence is
    // the truer answer than a count of files not done. The act's record keeps
    // each unit as not attempted, naming why.
    if ran.the_program_is_older && count(|one| *one == Some(UnitOutcome::Done)) == 0 {
        return WhatTheCopyCameTo::TheServerCannotCopy(WhyTheServerCannotCopy::SmoozyThereIsOlder);
    }
    WhatTheCopyCameTo::Ran {
        record: the_record,
        done: count(|one| *one == Some(UnitOutcome::Done)),
        refused: count(|one| *one == Some(UnitOutcome::Failed)),
        not_done: count(|one| matches!(one, Some(UnitOutcome::NotAttempted { .. })))
            + planned.gone.len()
            + planned.folder_collisions.len(),
        never_learned: count(Option::is_none),
    }
}

/// **`File Browser` M6's ordered pair, for one top-level item a move found
/// crossing filesystems**: the item copied under `File Browser` M7's rule, then each source
/// file removed **only where its copy published** — the delete is not
/// attempted where the copy did not land, naming the copy — and the emptied
/// source folders removed bottom-up, only if empty. Answers the move unit's
/// outcome: done where every pair completed, refused otherwise, and *not
/// attempted* where the server cannot copy.
#[allow(clippy::too_many_arguments)]
async fn the_pair_is_made<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    record: &dyn TheRecordAnUploadWrites,
    marking: &dyn WhenAUnitOfAnActEnds,
    pasteboard: &Pasteboard,
    from: &[u8],
    into: &[u8],
    replace: bool,
    run: &Identity,
    package_area: Option<&str>,
) -> UnitOutcome {
    use smoozy_file_browser::already_exists::CollisionAnswer;
    use smoozy_file_browser::managing::{what_the_second_half_does, WhatTheSecondHalfOfThePairDoes};
    let one = Pasteboard {
        entries: vec![from.to_vec()],
        act: smoozy_file_browser::managing::CutOrCopy::Copy,
        ..pasteboard.clone()
    };
    // **The pair's copy runs without a telling of its own**: the move's
    // telling already stands for the jobs it reaches.
    let copied = a_copy_is_made(
        sessions,
        record,
        marking,
        &one,
        into,
        Some(if replace { CollisionAnswer::Replace } else { CollisionAnswer::SkipExisting }),
        &[],
        run,
        package_area,
        &|_, _| {},
    )
    .await;
    let WhatTheCopyCameTo::Ran { record: copy_record, .. } = copied else {
        return UnitOutcome::NotAttempted {
            stopped_by: match copied {
                WhatTheCopyCameTo::TheServerCannotCopy(why) => why.as_recorded().to_string(),
                _ => "the copy".to_string(),
            },
        };
    };
    let mut read = None;
    record.with_the_record(&mut |connection| {
        read = crate::record::file_acts::FileActs::over(connection).read(&copy_record).ok().flatten();
    });
    let Some(read) = read else {
        return UnitOutcome::Failed;
    };
    let mut every_pair_completed = true;
    let mut removed: Vec<(Vec<u8>, WhatTheUnitDid, UnitOutcome)> = Vec::new();
    for (unit, outcome) in &read.units {
        match what_the_second_half_does(*outcome == Some(UnitOutcome::Done)) {
            WhatTheSecondHalfOfThePairDoes::DeleteTheSource => {
                let path = unit.source.clone();
                let gone = sessions
                    .a_reading(true, move |subsystem| Box::pin(async move { Ok(subsystem.remove(&path).await) }))
                    .await;
                if matches!(gone, Ok(Ok(Ok(())))) {
                    removed.push((unit.source.clone(), WhatTheUnitDid::RemovedIt, UnitOutcome::Done));
                } else {
                    every_pair_completed = false;
                }
            }
            WhatTheSecondHalfOfThePairDoes::NotAttempted { .. } => every_pair_completed = false,
        }
    }
    marking.units_ended(&removed);
    // The emptied source folders, deepest first, only if empty.
    let from_owned = from.to_vec();
    let folders = sessions
        .a_reading(true, move |subsystem| {
            Box::pin(async move {
                let mut folders = Vec::new();
                for entry in the_tree_under(subsystem, &from_owned).await? {
                    if entry.is_a_folder {
                        folders.push(entry);
                    }
                }
                folders.sort_by(|one, other| other.depth.cmp(&one.depth));
                let mut left = false;
                for folder in &folders {
                    left |= subsystem.remove_directory(&folder.path).await.is_err();
                }
                Ok(left)
            })
        })
        .await;
    if !matches!(folders, Ok(Ok(false)) | Ok(Err(SubsystemFailure::TheServerRefused(_)))) {
        every_pair_completed = false;
    }
    if every_pair_completed {
        UnitOutcome::Done
    } else {
        UnitOutcome::Failed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **A count's answer, read**: the four figures on *counted*, *at least*
    /// where the machine said incomplete, each refusal told apart, and a
    /// program that does not know the request read as older. The twenty-third
    /// notes, note 2.
    #[test]
    fn twenty_third_2_a_counts_answer_is_read() {
        use smoozy_server_protocol::framing::{Answered, Fields, Request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS};
        use smoozy_server_protocol::requests as asked;
        let request = Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            Identity::issue(smoozy_state::identities::IdentityKind::Operation).expect("randomness"),
            asked::THE_FOLDERS_SIZE_IS_COUNTED_REQUEST,
        );
        let answer = |fields: Fields| {
            Answered::to(&request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "0.1.9").carrying(fields)
        };
        let counted = |incomplete: &str| {
            answer(
                Fields::none()
                    .and(asked::THE_COUNTS_OUTCOME_FIELD, asked::IT_WAS_COUNTED)
                    .and(asked::THE_BYTES_COUNTED_FIELD, "123")
                    .and(asked::THE_FILES_COUNTED_FIELD, "3")
                    .and(asked::THE_FOLDERS_COUNTED_FIELD, "4")
                    .and(asked::THE_COUNT_IS_INCOMPLETE_FIELD, incomplete),
            )
        };
        assert_eq!(
            what_the_count_answer_says(&counted(asked::THE_COUNT_IS_WHOLE)),
            WhatTheCountCameTo::Counted(WhatTheCountFound { bytes: 123, files: 3, folders: 4, incomplete: false })
        );
        assert_eq!(
            what_the_count_answer_says(&counted(asked::THE_ANSWER_IS_YES)),
            WhatTheCountCameTo::Counted(WhatTheCountFound { bytes: 123, files: 3, folders: 4, incomplete: true })
        );
        assert_eq!(
            what_the_count_answer_says(&answer(Fields::none().and(asked::THE_COUNTS_OUTCOME_FIELD, asked::NOTHING_IS_THERE))),
            WhatTheCountCameTo::NothingThere
        );
        assert_eq!(
            what_the_count_answer_says(&answer(Fields::none().and(asked::THE_COUNTS_OUTCOME_FIELD, asked::IT_IS_NOT_A_FOLDER))),
            WhatTheCountCameTo::NotAFolder
        );
        assert_eq!(
            what_the_count_answer_says(&answer(
                Fields::none()
                    .and(asked::THE_COUNTS_OUTCOME_FIELD, asked::IT_WAS_NOT_READ)
                    .and(asked::THE_MACHINES_OWN_WORDS_FIELD, "Permission denied")
            )),
            WhatTheCountCameTo::NotRead("Permission denied".to_string())
        );
        let older = Answered::to(&request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "0.1.8")
            .saying_it_does_not_answer_that_request(asked::THE_FOLDERS_SIZE_IS_COUNTED_REQUEST);
        assert_eq!(what_the_count_answer_says(&older), WhatTheCountCameTo::TheProgramIsOlder);
    }

    /// **The pasteboard is one for the run**, so the tests that fill it take
    /// turns.
    static THE_PASTEBOARD_TESTS: std::sync::Mutex<()> = std::sync::Mutex::new(());

    /// **`File Browser` M11 to `File Browser` M16, as this run holds the pasteboard**: one
    /// thing at a time, replaced by the next cut or copy; Paste offered only on
    /// the row and installation it was taken under; kept across a drop, and let
    /// go of when that server's period ends — never another's.
    #[test]
    fn the_pasteboard_holds_one_thing_for_its_own_machine_until_the_period_ends() {
        let _turn = THE_PASTEBOARD_TESTS.lock().unwrap_or_else(|held| held.into_inner());
        use smoozy_file_browser::managing::{CutOrCopy, Pasteboard};
        use smoozy_state::identities::IdentityKind;
        let issue = |kind| Identity::issue(kind).expect("randomness");
        let (row, installation) = (issue(IdentityKind::LocalServer), issue(IdentityKind::Installation));
        let server = format!("pasteboard-test-{}", row.as_text());
        the_pasteboard_takes(
            &server,
            Pasteboard {
                server_row: row.clone(),
                installation: Some(installation.clone()),
                act: CutOrCopy::Cut,
                entries: vec![b"/home/ubuntu/a.png".to_vec(), b"/home/ubuntu/b.png".to_vec()],
            },
        );
        assert_eq!(paste_is_offered_with(&row, Some(&installation)), Some(2));
        assert_eq!(paste_is_offered_with(&issue(IdentityKind::LocalServer), Some(&installation)), None);
        assert_eq!(paste_is_offered_with(&row, Some(&issue(IdentityKind::Installation))), None, "a recycled machine");
        the_pasteboard_goes_with_the_period("another server");
        assert_eq!(paste_is_offered_with(&row, Some(&installation)), Some(2), "another server's period took it");
        the_pasteboard_takes(
            &server,
            Pasteboard {
                server_row: row.clone(),
                installation: Some(installation.clone()),
                act: CutOrCopy::Copy,
                entries: vec![b"/home/ubuntu/c.png".to_vec()],
            },
        );
        assert_eq!(paste_is_offered_with(&row, Some(&installation)), Some(1), "the next copy replaces the cut");
        the_pasteboard_goes_with_the_period(&server);
        assert_eq!(paste_is_offered_with(&row, Some(&installation)), None);
    }

    /// **`File Browser` M14, the Creator's seventh notes, note 11: a cut is let
    /// go of once its move has run**; Cancel at the question, a refusal before
    /// anything ran, the question itself, a cut pasted back where it is, and a
    /// move that never started all keep it; a copy stays held after its paste.
    #[test]
    fn a_cut_is_let_go_of_once_its_move_has_run_and_a_copy_is_kept() {
        use smoozy_file_browser::managing::{CutOrCopy, Pasteboard};
        use smoozy_state::identities::IdentityKind;
        let _turn = THE_PASTEBOARD_TESTS.lock().unwrap_or_else(|held| held.into_inner());
        let issue = |kind| Identity::issue(kind).expect("randomness");
        let (row, installation) = (issue(IdentityKind::LocalServer), issue(IdentityKind::Installation));
        let server = format!("paste-release-test-{}", row.as_text());
        let taken = |act| Pasteboard {
            server_row: row.clone(),
            installation: Some(installation.clone()),
            act,
            entries: vec![b"/home/ubuntu/a.png".to_vec(), b"/home/ubuntu/b.png".to_vec()],
        };
        let ran = WhatThePasteCameTo::Ran {
            record: issue(IdentityKind::Run),
            done: 2,
            refused: 0,
            not_done: 0,
            never_learned: 0,
            crossings: 0,
            crossings_want_an_update: false,
        };
        let cut = taken(CutOrCopy::Cut);
        for kept in [
            WhatThePasteCameTo::Cancelled,
            WhatThePasteCameTo::Refused(WhyThePasteIsRefused::AFolderIntoItself),
            WhatThePasteCameTo::AsksFirst(Vec::new()),
            WhatThePasteCameTo::NothingToMove,
            WhatThePasteCameTo::NothingWasMoved(WhatTheActCameTo::TheLoginMayNot),
        ] {
            the_pasteboard_takes(&server, cut.clone());
            the_paste_has_come_to(&server, &cut, &kept);
            assert_eq!(paste_is_offered_with(&row, Some(&installation)), Some(2), "{kept:?} keeps the cut");
        }
        the_paste_has_come_to("another server", &cut, &ran);
        assert_eq!(paste_is_offered_with(&row, Some(&installation)), Some(2), "another server's paste");
        let a_later_cut = Pasteboard { entries: vec![b"/home/ubuntu/c.png".to_vec()], ..cut.clone() };
        the_pasteboard_takes(&server, a_later_cut.clone());
        the_paste_has_come_to(&server, &cut, &ran);
        assert_eq!(paste_is_offered_with(&row, Some(&installation)), Some(1), "a cut taken since is not the one that ran");
        the_pasteboard_takes(&server, cut.clone());
        the_paste_has_come_to(&server, &cut, &ran);
        assert_eq!(paste_is_offered_with(&row, Some(&installation)), None, "the cut ran, so nothing is left to paste");
        let copy = taken(CutOrCopy::Copy);
        the_pasteboard_takes(&server, copy.clone());
        the_paste_has_come_to(&server, &copy, &ran);
        assert_eq!(paste_is_offered_with(&row, Some(&installation)), Some(2), "a copy still pastes");
        the_pasteboard_goes_with_the_period(&server);
    }

    #[test]
    fn paths_are_joined_and_split_on_their_components() {
        assert_eq!(inside(b"/home/ubuntu", b"a.png"), b"/home/ubuntu/a.png".to_vec());
        assert_eq!(inside(b"/home/ubuntu/", b"a.png"), b"/home/ubuntu/a.png".to_vec());
        assert_eq!(inside(b"/", b"a.png"), b"/a.png".to_vec());
        assert_eq!(the_folder_holding(b"/home/ubuntu/a.png"), b"/home/ubuntu".to_vec());
        assert_eq!(the_folder_holding(b"/a.png"), b"/".to_vec());
        assert_eq!(the_name_of(b"/home/ubuntu/kitchen/"), b"kitchen");
    }
}
