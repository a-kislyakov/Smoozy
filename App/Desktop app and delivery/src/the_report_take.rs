//! **What a reachable machine adds to a report: the per-job note, the
//! journal's tail and Blender's kept crash files** — `Reporting a problem` §6,
//! built at step 11's pieces R3a and R3b.
//!
//! **When it runs, and the one thing that starts it.** `Reporting a problem`
//! V5a's order: the user's press of *Save report* starts the take; the take
//! ends, or states its outcome per part; then the file is composed; and only
//! then is it written. **Nothing here reaches a machine before the press**,
//! because the report box calls this take only from that press.
//!
//! **Only for a report that names a job** (`Reporting a problem` G1): the
//! failed-job slice, or a crash report whose named failure is a job's. **A
//! report that names no job touches no machine at all**, and [`the_take`]
//! answers nothing for it without asking anything — half of what the menu
//! entry point's *always* needs (`Reporting a problem` X2).
//!
//! **Over the ordinary connection, one request at a time, through the
//! server's own point.** Every read is an ordinary command-path request asked
//! through [`crate::servers::the_machines_work::AsksTheSupervisor`], whose one
//! implementation in the running application is that server's point
//! (`crate::servers::sessions`, `Server protocol and transfers` T5) — and that
//! point already lets one supervisor ask through at a time, whoever asks. **No
//! new channel**: a machine mid-render meets the note, tail and crash-file
//! reads. **The desktop-wide point in `crate::scheduling` is not asked**,
//! because it admits what moves bytes across the line and gates no command
//! (`Server protocol and transfers` T5a).
//!
//! # The three parts, each with one of `Reporting a problem` S5's states
//!
//! - **The per-job note, whole** (`Reporting a problem` G3): the note reading
//!   `Queue & History` N1 answers, carried as the record it is — every field
//!   of the note the answer carries, in the answer's own order, its moments
//!   said to be on the machine's clock. *Absent* where the machine says it
//!   holds no note on the job. **The capture records that ride the same answer
//!   are not part of the note** and are not carried here: how each stream's
//!   collection was recorded is the report's own *captured output* part,
//!   which this take does not supply.
//! - **The journal's last 512 entries** (`Reporting a problem` G2): the
//!   existing read forward, asked first from past every sequence number to
//!   learn where the journal ends, then from the newest minus 512. **No cursor
//!   is registered and none is moved**: the read forward registers nothing on
//!   the machine (`State, identity and recovery` E3), and the one request that
//!   moves the desktop's cursor is never sent from here. **The span held and
//!   the journal's own oldest retained number** travel with the entries, which
//!   is `State, identity and recovery` E5's honesty kept by statement where it
//!   cannot be kept by cursor. *Absent* where the journal holds no entry.
//!
//! **A server not connected at the press is *unreachable* for all three parts**,
//! and nothing is asked. **A server connected at the press whose read then did
//! not finish** — the connection ended during the take, or an answer would not
//! read — **is *incomplete at the cut* for that part**, with what arrived
//! carried and named, and never *included* over a short tail or *absent* over a
//! note that may exist (`Reporting a problem` S5 and `Reporting a problem` S7).
//!
//! **Blender's kept crash files** (`Reporting a problem` G5 and
//! `Reporting a problem` G5a) are the third read, for this job only. Each
//! attempt's name and whole copy, or its recorded reason for having none,
//! travel together. Nothing is read as evidence about how the render ended.
//! A machine that explicitly does not answer this reading is *absent*, with
//! that reason; a broken read or a copy not served whole is *incomplete*.
//!
//! **Where it may be called from.** Off the drawing thread and off the period's
//! own thread: each ask waits on the server's point, and that point serves from
//! the period's thread, so an ask made from there would wait for itself.
//!
//! **Every marker in this file is written with its document beside it**,
//! because nearly every one belongs to another plan.

use std::time::{Duration, UNIX_EPOCH};

use tauri::{AppHandle, Manager, Runtime};

use smoozy_reporting::report_file::{the_moment_in_utc, TheCrashFile, TheJournalTail, Taken};
use smoozy_server_protocol::asking_the_supervisor::WhatCameBack;
use smoozy_server_protocol::framing::Fields;
use smoozy_server_protocol::requests::{
    the_field_of_item, AN_EVENT, A_RE_RUN, HOW_MANY_EVENTS_FIELD, HOW_MANY_RE_RUNS_FIELD,
    IT_IS_ABSENT, IT_IS_PRESENT, ON_THIS_MACHINE_FIELD, THE_ABOUT_FIELD,
    THE_KEPT_CRASH_FILES_REQUEST, HOW_MANY_CRASH_FILE_STATEMENTS_FIELD,
    A_CRASH_FILE_STATEMENT, THE_CRASH_FILES_OWN_NAME_FIELD,
    WHAT_BECAME_OF_THE_CRASH_FILE_FIELD, THE_CRASH_FILE_WAS_KEPT,
    NO_CRASH_FILE_WAS_KEPT, THE_CRASH_FILE_WHOLE_FIELD,
    WHY_NO_CRASH_FILE_WAS_KEPT_FIELD, THE_KEPT_CRASH_FILE_DID_NOT_READ_BACK,
    THE_KEPT_CRASH_FILE_DID_NOT_FIT, THE_MACHINES_OWN_WORDS_FIELD,
    THE_ATTEMPTS_FIELD, THE_ATTEMPT_FIELD, THE_ATTEMPT_THAT_HOLDS_THE_WORDS_FIELD, THE_CURSOR_FIELD,
    THE_DEVICE_LINE_FIELD, THE_ENDED_CLOCK_FIELD, THE_ENDED_FIELD, THE_ENDING_FIELD,
    THE_EXIT_STATUS_FIELD, THE_FILES_ABSENT_FIELD, THE_FILES_EXPECTED_FIELD,
    THE_FILES_PRESENT_AT_ZERO_BYTES_FIELD, THE_FILES_PRESENT_FIELD,
    THE_FRAME_IT_PROCEEDED_FROM_FIELD, THE_FRAME_IT_WAS_GIVEN_UP_AT_FIELD,
    THE_HAPPENED_AT_FIELD, THE_HAPPENED_ON_CLOCK_FIELD, THE_JOB_FIELD, THE_KIND_FIELD,
    THE_LABEL_FIELD, THE_LAST_FRAME_FIELD, THE_NEXT_SEQUENCE_FIELD, THE_OLDEST_RETAINED_FIELD,
    THE_PER_JOB_NOTE_READING_REQUEST, THE_READ_FORWARD_REQUEST,
    THE_REASON_THE_MACHINE_RECORDED_FIELD, THE_SAYS_FIELD, THE_SEQUENCE_FIELD,
    THE_STARTED_CLOCK_FIELD, THE_STARTED_FIELD, WHY_NO_COMPLETENESS_STATEMENT_FIELD,
};
use smoozy_state::identities::Identity;

use crate::servers::the_machines_work::{a_request, AsksTheSupervisor, OverThisServersPoint};

/// **The tail's length** — `Reporting a problem` G2: *"the last 512 entries,
/// fixed rather than read"*, a sixty-fourth of the journal's cap. One constant
/// in one place, and the runner's to overrule.
pub const THE_TAILS_LENGTH: u64 = 512;

/// **A cursor past every sequence number the journal can hold.** The read
/// forward from it carries no entry and still says where the journal ends and
/// what it retains, which is how the take learns the newest without moving or
/// registering anything.
const PAST_EVERY_SEQUENCE: u64 = u64::MAX;

/// **What the take brought back**, part by part, each in one of
/// `Reporting a problem` S5's states — the shape step 11's report cut composes
/// the file from (`smoozy_reporting::report_file::WhatTheMachineAdds`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheTake {
    /// `Reporting a problem` G3: the note, whole, as the record it is.
    pub the_per_job_note: Taken<String>,
    /// `Reporting a problem` G2: the last 512 entries, with the span held and
    /// the journal's oldest retained number.
    pub the_journal_tail: Taken<TheJournalTail>,
    /// `Reporting a problem` G5: each attempt's kept file, whole, under its
    /// own name. The report's one crash-file part holds all attempt statements.
    pub the_crash_file: Taken<TheCrashFile>,
}

impl TheTake {
    /// **`Reporting a problem` S5's *unreachable*, for every part**: the server
    /// could not be reached when the report was cut, so nothing was taken.
    pub fn unreachable() -> TheTake {
        TheTake {
            the_per_job_note: Taken::Unreachable,
            the_journal_tail: Taken::Unreachable,
            the_crash_file: Taken::Unreachable,
        }
    }
}

/// **The take, for the report the user has just asked to save.**
///
/// **Nothing, and nothing asked, where the report names no job**
/// (`Reporting a problem` G1). **All three parts unreachable, and nothing asked,
/// where the job's server was not connected at the press** — `machine` is then
/// nothing. Otherwise the note, the tail, then the crash files, one request at a time.
///
/// **The note goes first** because it is the smaller part and the one that
/// carries the ending and the reason a render failed: where the connection
/// ends part-way, it is the part worth having.
pub fn the_take(
    the_report_names: Option<&Identity>,
    machine: Option<&mut dyn AsksTheSupervisor>,
) -> Option<TheTake> {
    let job = the_report_names?;
    let Some(machine) = machine else {
        return Some(TheTake::unreachable());
    };
    let the_per_job_note = the_note_taken(machine, job);
    let the_journal_tail = the_tail_taken(machine);
    let the_crash_file = the_crash_files_taken(machine, job);
    Some(TheTake {
        the_per_job_note,
        the_journal_tail,
        the_crash_file,
    })
}

/// **A read that began on a live connection and did not finish, with nothing
/// arrived.**
fn did_not_finish<T>() -> Taken<T> {
    Taken::BeganAndDidNotComplete { what_arrived: None }
}

/// The answer's fields, where the supervisor answered this very request.
/// Nothing where it refused it by name, where nothing that answers it came
/// back, or where the request could not be made.
fn the_answer(machine: &mut dyn AsksTheSupervisor, name: &str, fields: Fields) -> Option<Fields> {
    let request = a_request(name, fields)?;
    match machine.ask(&request) {
        WhatCameBack::ItAnswered(answered)
            if answered.the_request_name_it_says_it_does_not_answer().is_none() =>
        {
            Some(answered.fields)
        }
        _ => None,
    }
}

// ── The kept crash files ─────────────────────────────────────────────────────

/// `Reporting a problem` G5: the job's kept files, whole or not at all.
/// Only an explicit refusal of this request is the older-program case. A
/// silence, unreadable answer or other refusal is an unfinished read.
fn the_crash_files_taken(machine: &mut dyn AsksTheSupervisor, job: &Identity) -> Taken<TheCrashFile> {
    let Some(request) = a_request(
        THE_KEPT_CRASH_FILES_REQUEST,
        Fields::none().and(THE_JOB_FIELD, job.as_text()),
    ) else {
        return did_not_finish();
    };
    let WhatCameBack::ItAnswered(answer) = machine.ask(&request) else {
        return did_not_finish();
    };
    match answer.the_request_name_it_says_it_does_not_answer() {
        Some(THE_KEPT_CRASH_FILES_REQUEST) => Taken::NothingThere {
            why: Some("this server's Smoozy version cannot send crash files".to_string()),
        },
        Some(_) => did_not_finish(),
        None => crash_files_as_taken(&answer.fields).unwrap_or_else(did_not_finish),
    }
}

/// The report has one crash-file part. Keep every attempt in it, in the
/// answer's order, with headings around the copies, never cuts inside them.
/// `Reporting a problem` S4a drops this entire part if it exceeds the ceiling.
/// Names and copies must read as text: no lossy conversion of a file's bytes.
fn crash_files_as_taken(fields: &Fields) -> Option<Taken<TheCrashFile>> {
    match fields.text(ON_THIS_MACHINE_FIELD)? {
        IT_IS_ABSENT => return Some(Taken::NothingThere { why: None }),
        IT_IS_PRESENT => {},
        _ => return None,
    }
    let count = fields.text(HOW_MANY_CRASH_FILE_STATEMENTS_FIELD)?.parse::<usize>().ok()?;
    // Each statement needs at least its attempt and state. An invalid count
    // must not make the reader iterate beyond what the answer could contain.
    if count > fields.how_many() / 2 {
        return None;
    }
    if count == 0 {
        return Some(Taken::NothingThere { why: None });
    }
    let mut names = Vec::new();
    let mut text = String::new();
    let mut absent = Vec::new();
    let mut has_file = false;
    let mut incomplete = false;
    let mut attempts = std::collections::HashSet::new();
    for at in 0..count {
        let item = |field: &str| the_field_of_item(A_CRASH_FILE_STATEMENT, at, field);
        let attempt = fields.text(&item(THE_ATTEMPT_FIELD))?.parse::<u64>().ok()?;
        if attempt == 0 || !attempts.insert(attempt) {
            return None;
        }
        let name_field = item(THE_CRASH_FILES_OWN_NAME_FIELD);
        let name = match fields.value(&name_field) {
            Some(bytes) => Some(std::str::from_utf8(bytes).ok()?),
            None => None,
        };
        let label = match name {
            Some(name) => format!("attempt {attempt}, {name}"),
            None => format!("attempt {attempt}"),
        };
        text.push_str(&format!("== {label} ==\n"));
        match fields.text(&item(WHAT_BECAME_OF_THE_CRASH_FILE_FIELD))? {
            THE_CRASH_FILE_WAS_KEPT => {
                let name = name?;
                let whole = fields.text(&item(THE_CRASH_FILE_WHOLE_FIELD))?;
                names.push(name.to_string());
                text.push_str(whole);
                if !whole.ends_with('\n') {
                    text.push('\n');
                }
                has_file = true;
            }
            NO_CRASH_FILE_WAS_KEPT => {
                let why = fields.text(&item(WHY_NO_CRASH_FILE_WAS_KEPT_FIELD))?;
                text.push_str(&format!("absent: {why}\n"));
                absent.push(format!("{label}: {why}"));
            }
            THE_KEPT_CRASH_FILE_DID_NOT_READ_BACK => {
                let why = fields.text(&item(THE_MACHINES_OWN_WORDS_FIELD))?;
                text.push_str(&format!("incomplete: {why}\n"));
                incomplete = true;
            }
            THE_KEPT_CRASH_FILE_DID_NOT_FIT => {
                text.push_str("incomplete: crash file too large to send; not included\n");
                incomplete = true;
            }
            _ => return None,
        }
    }
    if !has_file && !incomplete {
        return Some(Taken::NothingThere { why: Some(absent.join("; ")) });
    }
    let files = TheCrashFile {
        name: if names.is_empty() { "attempt statements".to_string() } else { names.join(", ") },
        text,
    };
    Some(if incomplete {
        Taken::BeganAndDidNotComplete { what_arrived: Some(files) }
    } else {
        Taken::Whole(files)
    })
}

// ── The note ─────────────────────────────────────────────────────────────────

/// **`Reporting a problem` G3: the note, taken whole.**
///
/// *Absent* only where the machine's answer says it holds no note on the job.
/// **Its *cannot say*, a refusal, or an answer with no word in it is a read
/// that did not finish**: the note may exist, and *absent* over a note that
/// exists is the quiet trim `Reporting a problem` S7 forbids.
fn the_note_taken(machine: &mut dyn AsksTheSupervisor, job: &Identity) -> Taken<String> {
    let Some(fields) = the_answer(
        machine,
        THE_PER_JOB_NOTE_READING_REQUEST,
        Fields::none().and(THE_JOB_FIELD, job.as_text()),
    ) else {
        return did_not_finish();
    };
    match fields.text(ON_THIS_MACHINE_FIELD) {
        Some(IT_IS_PRESENT) => Taken::Whole(the_note_as_written(&fields)),
        Some(IT_IS_ABSENT) => Taken::NothingThere { why: None },
        _ => did_not_finish(),
    }
}

/// **The note as the record it is**: one line per field the answer names, the
/// field's own name and its value verbatim, in the order the note's own crate
/// composes the answer (`Queue & History — machine`'s `as_the_answers_fields`).
/// **Its two moments are written as moments, each on the clock the answer
/// names** (`Reporting a problem` G3). Nothing is read for a cause and nothing
/// is left out but what the answer does not carry.
fn the_note_as_written(fields: &Fields) -> String {
    let mut text = String::new();
    let a_field = |text: &mut String, name: &str| {
        if let Some(value) = fields.text(name) {
            a_line(text, name, value);
        }
    };
    for name in [THE_LABEL_FIELD, THE_ATTEMPTS_FIELD, HOW_MANY_RE_RUNS_FIELD] {
        a_field(&mut text, name);
    }
    for (at, clock) in [
        (THE_STARTED_FIELD, THE_STARTED_CLOCK_FIELD),
        (THE_ENDED_FIELD, THE_ENDED_CLOCK_FIELD),
    ] {
        if let Some(seconds) = fields.text(at) {
            a_line(&mut text, at, &a_moment(seconds, fields.text(clock)));
        }
    }
    for name in [
        THE_ENDING_FIELD,
        THE_ATTEMPT_THAT_HOLDS_THE_WORDS_FIELD,
        THE_EXIT_STATUS_FIELD,
        THE_REASON_THE_MACHINE_RECORDED_FIELD,
        THE_DEVICE_LINE_FIELD,
    ] {
        a_field(&mut text, name);
    }
    let re_runs = fields
        .text(HOW_MANY_RE_RUNS_FIELD)
        .and_then(|text| text.parse::<usize>().ok())
        .unwrap_or(0);
    for at in 0..re_runs {
        for name in [THE_ATTEMPT_FIELD, THE_FRAME_IT_PROCEEDED_FROM_FIELD] {
            a_field(&mut text, &the_field_of_item(A_RE_RUN, at, name));
        }
    }
    for name in [
        THE_FRAME_IT_WAS_GIVEN_UP_AT_FIELD,
        THE_LAST_FRAME_FIELD,
        THE_FILES_EXPECTED_FIELD,
        THE_FILES_PRESENT_FIELD,
        THE_FILES_ABSENT_FIELD,
        THE_FILES_PRESENT_AT_ZERO_BYTES_FIELD,
        WHY_NO_COMPLETENESS_STATEMENT_FIELD,
    ] {
        a_field(&mut text, name);
    }
    text
}

fn a_line(text: &mut String, name: &str, value: &str) {
    text.push_str(name);
    text.push_str(": ");
    text.push_str(value);
    text.push('\n');
}

/// **A moment the machine sent as whole seconds and the name of its clock**,
/// written as a complete UTC moment on that clock (`State, identity and
/// recovery` D5: no moment without its clock). **A value that is not whole
/// seconds is carried as it came**, never guessed at.
fn a_moment(seconds: &str, clock: Option<&str>) -> String {
    let at = match seconds.parse::<u64>() {
        Ok(seconds) => the_moment_in_utc(UNIX_EPOCH + Duration::from_secs(seconds)),
        Err(_) => seconds.to_string(),
    };
    match clock {
        Some(clock) => format!("{at} on the clock of {clock}"),
        None => format!("{at} (server clock unknown)"),
    }
}

// ── The journal's tail ───────────────────────────────────────────────────────

/// One journal entry as one read forward carried it.
struct AnEntryRead {
    sequence: u64,
    line: String,
}

/// What one read forward carried.
struct OneRead {
    entries: Vec<AnEntryRead>,
    /// One past the newest sequence the journal holds.
    next: u64,
    /// `State, identity and recovery` E5's number, absent where the journal
    /// holds nothing.
    oldest_retained: Option<u64>,
}

/// **One read forward from `cursor`** — the existing request, carrying the
/// cursor and nothing else, so nothing on the machine is registered or moved
/// (`State, identity and recovery` E3).
///
/// **An entry that will not read back makes the whole read unreadable**, as
/// the read forward's own reader rules in `crate::servers::the_machines_work`:
/// keeping the entries around it would lose it without a word.
fn one_read_forward(machine: &mut dyn AsksTheSupervisor, cursor: u64) -> Option<OneRead> {
    let fields = the_answer(
        machine,
        THE_READ_FORWARD_REQUEST,
        Fields::none().and(THE_CURSOR_FIELD, cursor.to_string()),
    )?;
    let a_number = |text: Option<&str>| text?.parse::<u64>().ok();
    let how_many = a_number(fields.text(HOW_MANY_EVENTS_FIELD))?;
    let next = a_number(fields.text(THE_NEXT_SEQUENCE_FIELD))?;
    let mut entries = Vec::new();
    for at in 0..how_many as usize {
        let item = |field: &str| fields.text(&the_field_of_item(AN_EVENT, at, field));
        let sequence = a_number(item(THE_SEQUENCE_FIELD))?;
        let kind = item(THE_KIND_FIELD)?;
        entries.push(AnEntryRead {
            sequence,
            line: an_entry_as_written(
                sequence,
                item(THE_HAPPENED_AT_FIELD).map(|seconds| a_moment(seconds, item(THE_HAPPENED_ON_CLOCK_FIELD))),
                kind,
                item(THE_ABOUT_FIELD),
                item(THE_SAYS_FIELD),
            ),
        });
    }
    Some(OneRead {
        entries,
        next,
        oldest_retained: a_number(fields.text(THE_OLDEST_RETAINED_FIELD)),
    })
}

/// **One entry, one line**: its sequence number, its moment on the machine's
/// clock, its kind, what it is about and the machine's own words for it — each
/// as the machine sent it (`Reporting a problem` G4).
fn an_entry_as_written(
    sequence: u64,
    moment: Option<String>,
    kind: &str,
    about: Option<&str>,
    says: Option<&str>,
) -> String {
    let mut line = format!(
        "{sequence} \u{b7} {} \u{b7} {kind}",
        moment.unwrap_or_else(|| "no time given".to_string())
    );
    if let Some(about) = about {
        line.push_str(&format!(" \u{b7} about {about}"));
    }
    if let Some(says) = says.filter(|says| !says.is_empty()) {
        line.push_str(&format!(" \u{b7} {says}"));
    }
    line.push('\n');
    line
}

/// **`Reporting a problem` G2: the last 512 entries, read without a cursor.**
///
/// **First, where the journal ends**: a read forward from past every sequence
/// number carries no entry and says the newest and the oldest retained. **Then
/// the tail**, from the newest minus 512, asked again from past the last entry
/// given wherever one answer carries fewer than were asked for — the machine
/// bounds one answer by its frame, not by the journal. **Each read is one
/// request, made after the one before it has ended.** Entries the machine
/// writes after the first read are past the tail's end and are not taken, so
/// a machine mid-render does not move the window.
///
/// **A read that does not finish ends the take of this part**, with every
/// entry already held carried and named as incomplete. A read that finishes
/// and carries nothing new ends it too, whole: the machine has said there is
/// no more, and the span the tail states is the truth about it.
fn the_tail_taken(machine: &mut dyn AsksTheSupervisor) -> Taken<TheJournalTail> {
    let Some(where_it_ends) = one_read_forward(machine, PAST_EVERY_SEQUENCE) else {
        return did_not_finish();
    };
    let the_end = where_it_ends.next;
    let mut oldest_retained = where_it_ends.oldest_retained;
    if oldest_retained.is_none() || the_end == 0 {
        return Taken::NothingThere { why: None };
    }
    let mut held: Vec<AnEntryRead> = Vec::new();
    let mut from = the_end.saturating_sub(THE_TAILS_LENGTH);
    while (held.len() as u64) < THE_TAILS_LENGTH && from < the_end {
        let Some(read) = one_read_forward(machine, from) else {
            return Taken::BeganAndDidNotComplete {
                what_arrived: the_tail_of(held, oldest_retained),
            };
        };
        if read.oldest_retained.is_some() {
            oldest_retained = read.oldest_retained;
        }
        let before = held.len();
        held.extend(
            read.entries
                .into_iter()
                .filter(|entry| entry.sequence >= from && entry.sequence < the_end)
                .take(THE_TAILS_LENGTH as usize - before),
        );
        let Some(last) = held.last().filter(|_| held.len() > before) else {
            break;
        };
        from = last.sequence + 1;
    }
    if held.is_empty() {
        return Taken::NothingThere { why: None };
    }
    match the_tail_of(held, oldest_retained) {
        Some(tail) => Taken::Whole(tail),
        // **Entries with no oldest retained number cannot state the window
        // they are** (`State, identity and recovery` E5), so they are not
        // passed off as one.
        None => did_not_finish(),
    }
}

/// The entries held, as the tail the report states: its span and the
/// journal's oldest retained number beside them. Nothing where none is held,
/// or where the machine named no oldest retained number.
fn the_tail_of(held: Vec<AnEntryRead>, oldest_retained: Option<u64>) -> Option<TheJournalTail> {
    let first = held.first()?.sequence;
    let last = held.last()?.sequence;
    Some(TheJournalTail {
        the_first_sequence_held: first,
        the_last_sequence_held: last,
        the_journals_oldest_retained: oldest_retained?,
        entries: held.into_iter().map(|entry| entry.line).collect(),
    })
}

/// **The take in the running application**, made at the press of *Save the
/// report* and at no other moment.
///
/// **The job's server and its package area are read from the local record and
/// the record is let go of before anything is asked.** The server is reached
/// through its own point only where that point has a live connection at this
/// moment; a job whose record, server row or recorded placement is not there
/// has no server to reach, which is *unreachable* as surely as a server not
/// connected.
pub fn the_take_at_the_press<R: Runtime>(
    app: &AppHandle<R>,
    the_report_names: Option<&Identity>,
) -> Option<TheTake> {
    let job = the_report_names?;
    let mut point = the_jobs_point(app, job);
    the_take(
        Some(job),
        point.as_mut().map(|point| point as &mut dyn AsksTheSupervisor),
    )
}

/// The point of the server the job ran on, where it has a live connection.
fn the_jobs_point<R: Runtime>(app: &AppHandle<R>, job: &Identity) -> Option<OverThisServersPoint> {
    let (server, package_area) = {
        let started = app.try_state::<std::sync::Mutex<crate::start::Started>>()?;
        let started = started.lock().ok()?;
        let local = started.record.as_ref()?;
        let record = crate::record::job_records::JobRecords::over(local.connection())
            .read(job)
            .ok()
            .flatten()?;
        let server = record.server.local_server.as_text();
        let row = crate::record::server_rows::all(local.connection())
            .ok()?
            .into_iter()
            .find(|row| row.local_server_identifier == server)?;
        match crate::servers::installation::where_the_package_area_is(&row) {
            crate::servers::installation::WhereThePackageAreaIs::At(area) => (server, area),
            _ => return None,
        }
    };
    let sessions = app
        .try_state::<std::sync::Arc<crate::servers::a_period::ThePeriodsThisRun>>()?
        .the_sessions_of(&server)
        .filter(crate::servers::sessions::ThisServersSessions::a_connection_is_live)?;
    Some(OverThisServersPoint {
        sessions,
        package_area,
    })
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::time::SystemTime;

    use super::*;
    use smoozy_server_protocol::framing::{Answered, Request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS};
    use smoozy_server_protocol::requests::{
        IT_FAILED, THE_CURSOR_MOVES_FORWARD_REQUEST, THE_MACHINE_CANNOT_SAY,
        THE_KEPT_CRASH_FILES_REQUEST, HOW_MANY_CRASH_FILE_STATEMENTS_FIELD,
        A_CRASH_FILE_STATEMENT, THE_CRASH_FILES_OWN_NAME_FIELD,
        WHAT_BECAME_OF_THE_CRASH_FILE_FIELD, THE_CRASH_FILE_WAS_KEPT,
        NO_CRASH_FILE_WAS_KEPT, THE_CRASH_FILE_WHOLE_FIELD,
        WHY_NO_CRASH_FILE_WAS_KEPT_FIELD,

    };
    use smoozy_state::identities::IdentityKind;

    // ── A machine, stood up for the test ─────────────────────────────────────

    /// One entry the stand-in machine's journal holds.
    #[derive(Debug, Clone)]
    struct AnEntry {
        sequence: u64,
        kind: String,
        about: Option<String>,
        says: String,
        at: u64,
    }

    /// **A machine with a journal and a note**, answering the two readings the
    /// way the machine's program does (`main.rs`'s read forward: the events
    /// from the cursor on, at most a frame's worth, with where the journal ends
    /// and what it retains beside them). **It writes down every request it was
    /// asked, in order**, and keeps the one registration the desktop's cursor
    /// move would make, so a test can see that nothing was left behind.
    struct AMachineInHand {
        journal: Vec<AnEntry>,
        /// One past the newest entry.
        next: u64,
        /// The most entries one read carries — the machine's own frame limit.
        per_read: usize,
        /// What the note reading answers; nothing is a silence.
        note: Option<Fields>,
        crash_files: Option<Fields>,
        /// After this many asks the connection ends and nothing more comes back.
        the_line_ends_after: Option<usize>,
        /// New entries appended after each read, as a machine mid-render does.
        grows_by_after_each_read: u64,
        /// The desktop's registered cursor, which only the cursor move sets.
        registered: Option<u64>,
        asked: Vec<(String, Fields)>,
    }

    const THE_CLOCK: &str = "server-clock-1";

    fn an_entry(sequence: u64) -> AnEntry {
        AnEntry {
            sequence,
            kind: if sequence % 2 == 0 { "the job record written" } else { "a render attempt started" }
                .to_string(),
            about: (sequence % 3 == 0).then(|| format!("job-{sequence}")),
            says: format!("entry {sequence} in the machine's own words"),
            at: 1_761_300_000 + sequence,
        }
    }

    impl AMachineInHand {
        /// A journal retaining `oldest` to one before `next`.
        fn holding(oldest: u64, next: u64) -> AMachineInHand {
            AMachineInHand {
                journal: (oldest..next).map(an_entry).collect(),
                next,
                per_read: 512,
                note: Some(a_note()),
                crash_files: Some(crash_answer(&[])),
                the_line_ends_after: None,
                grows_by_after_each_read: 0,
                registered: None,
                asked: Vec::new(),
            }
        }

        fn reads_forward_asked(&self) -> Vec<u64> {
            self.asked
                .iter()
                .filter(|(name, _)| name == THE_READ_FORWARD_REQUEST)
                .map(|(_, fields)| fields.text(THE_CURSOR_FIELD).unwrap().parse().unwrap())
                .collect()
        }

        fn the_read_forward(&mut self, request: &Request) -> Fields {
            let cursor: u64 = request.fields.text(THE_CURSOR_FIELD).unwrap().parse().unwrap();
            let events: Vec<AnEntry> = self
                .journal
                .iter()
                .filter(|entry| entry.sequence >= cursor)
                .take(self.per_read)
                .cloned()
                .collect();
            let mut fields = Fields::none()
                .and(HOW_MANY_EVENTS_FIELD, events.len().to_string())
                .and(THE_NEXT_SEQUENCE_FIELD, self.next.to_string());
            if let Some(oldest) = self.journal.first() {
                fields = fields.and(THE_OLDEST_RETAINED_FIELD, oldest.sequence.to_string());
            }
            for (at, entry) in events.iter().enumerate() {
                let item = |field: &str| the_field_of_item(AN_EVENT, at, field);
                fields = fields
                    .and(&item(THE_SEQUENCE_FIELD), entry.sequence.to_string())
                    .and(&item(THE_KIND_FIELD), entry.kind.as_str())
                    .and(&item(THE_SAYS_FIELD), entry.says.as_str())
                    .and(&item(THE_HAPPENED_AT_FIELD), entry.at.to_string())
                    .and(&item(THE_HAPPENED_ON_CLOCK_FIELD), THE_CLOCK);
                if let Some(about) = &entry.about {
                    fields = fields.and(&item(THE_ABOUT_FIELD), about.as_str());
                }
            }
            for _ in 0..self.grows_by_after_each_read {
                self.journal.push(an_entry(self.next));
                self.next += 1;
            }
            fields
        }
    }

    impl AsksTheSupervisor for AMachineInHand {
        fn ask(&mut self, request: &Request) -> WhatCameBack {
            self.asked.push((request.name.clone(), request.fields.clone()));
            if self.the_line_ends_after.is_some_and(|after| self.asked.len() > after) {
                return WhatCameBack::NothingCorrelatedCameBack;
            }
            let answered = Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "test");
            let fields = match request.name.as_str() {
                THE_PER_JOB_NOTE_READING_REQUEST => match &self.note {
                    Some(note) => note.clone(),
                    None => return WhatCameBack::NothingCorrelatedCameBack,
                },
                THE_READ_FORWARD_REQUEST => self.the_read_forward(request),
                THE_KEPT_CRASH_FILES_REQUEST if self.crash_files.is_some() => {
                    self.crash_files.clone().unwrap()
                }
                THE_CURSOR_MOVES_FORWARD_REQUEST => {
                    self.registered = request.fields.text(THE_CURSOR_FIELD).and_then(|text| text.parse().ok());
                    Fields::none()
                }
                _ => {
                    return WhatCameBack::ItAnswered(
                        answered.saying_it_does_not_answer_that_request(&request.name),
                    )
                }
            };
            WhatCameBack::ItAnswered(answered.carrying(fields))
        }
    }

    fn a_note() -> Fields {
        Fields::none()
            .and(ON_THIS_MACHINE_FIELD, IT_IS_PRESENT)
            .and(THE_LABEL_FIELD, "Kitchen, take 3")
            .and(THE_ATTEMPTS_FIELD, "2")
            .and(HOW_MANY_RE_RUNS_FIELD, "1")
            .and(THE_STARTED_FIELD, "1761300000")
            .and(THE_STARTED_CLOCK_FIELD, THE_CLOCK)
            .and(THE_ENDED_FIELD, "1761303600")
            .and(THE_ENDED_CLOCK_FIELD, THE_CLOCK)
            .and(THE_ENDING_FIELD, IT_FAILED)
            .and(THE_ATTEMPT_THAT_HOLDS_THE_WORDS_FIELD, "2")
            .and(THE_EXIT_STATUS_FIELD, "-11")
            .and(THE_REASON_THE_MACHINE_RECORDED_FIELD, "Blender stopped: signal 11\nsecond line")
            .and(&the_field_of_item(A_RE_RUN, 0, THE_ATTEMPT_FIELD), "2")
            .and(&the_field_of_item(A_RE_RUN, 0, THE_FRAME_IT_PROCEEDED_FROM_FIELD), "121")
            .and(THE_LAST_FRAME_FIELD, "120")
            .and(WHY_NO_COMPLETENESS_STATEMENT_FIELD, "the output folder could not be walked")
    }

    fn a_job() -> Identity {
        Identity::issue(IdentityKind::Job).unwrap()
    }

    fn the_take_from(machine: &mut AMachineInHand) -> TheTake {
        the_take(Some(&a_job()), Some(machine)).expect("a report naming a job has a take")
    }

    fn whole<T: Clone + std::fmt::Debug>(taken: &Taken<T>) -> T {
        match taken {
            Taken::Whole(content) => content.clone(),
            other => panic!("expected the part whole, and it was {other:?}"),
        }
    }

    // ── A reachable machine ──────────────────────────────────────────────────

    /// **A reachable machine gives the note whole and the journal's last 512
    /// entries**, with the span held and the journal's own oldest retained
    /// number — `Reporting a problem` G2 and `Reporting a problem` G3.
    #[test]
    fn a_reachable_machine_gives_the_note_whole_and_the_last_512_entries() {
        let mut machine = AMachineInHand::holding(300, 2000);
        let take = the_take_from(&mut machine);

        let tail = whole(&take.the_journal_tail);
        assert_eq!(tail.the_first_sequence_held, 2000 - 512);
        assert_eq!(tail.the_last_sequence_held, 1999);
        assert_eq!(
            tail.the_journals_oldest_retained, 300,
            "the journal's own oldest retained number travels with the tail, not the first entry held"
        );
        let lines: Vec<&str> = tail.entries.lines().collect();
        assert_eq!(lines.len(), 512);
        assert!(lines[0].starts_with("1488 "), "{}", lines[0]);
        assert!(lines[511].starts_with("1999 "), "{}", lines[511]);
        // **Nothing reworded** (`Reporting a problem` G4): the machine's own
        // words, its kind and what it is about, and the moment on its clock.
        let one = an_entry(1500);
        let written = lines.iter().find(|line| line.starts_with("1500 ")).unwrap();
        assert!(written.contains(&one.says));
        assert!(written.contains(&one.kind));
        assert!(written.contains("about job-1500"));
        assert!(written.contains(&format!("on the clock of {THE_CLOCK}")));
        assert!(written.contains(&the_moment_in_utc(UNIX_EPOCH + Duration::from_secs(one.at))));

        let note = whole(&take.the_per_job_note);
        for (name, value) in [
            (THE_LABEL_FIELD, "Kitchen, take 3"),
            (THE_ENDING_FIELD, IT_FAILED),
            (THE_EXIT_STATUS_FIELD, "-11"),
            (THE_LAST_FRAME_FIELD, "120"),
            (WHY_NO_COMPLETENESS_STATEMENT_FIELD, "the output folder could not be walked"),
        ] {
            assert!(note.contains(&format!("{name}: {value}\n")), "{name} missing from:\n{note}");
        }
        assert!(
            note.contains("the reason the machine recorded: Blender stopped: signal 11\nsecond line\n"),
            "the machine's recorded reason is carried verbatim, line break and all:\n{note}"
        );
        assert!(note.contains(&format!(
            "{}: {} on the clock of {THE_CLOCK}",
            THE_STARTED_FIELD,
            the_moment_in_utc(UNIX_EPOCH + Duration::from_secs(1_761_300_000))
        )));
        assert!(note.contains(&format!(
            "{}: 121",
            the_field_of_item(A_RE_RUN, 0, THE_FRAME_IT_PROCEEDED_FROM_FIELD)
        )));

        // **One request at a time, in the take `Reporting a problem` V5a orders and no other**: the note, then
        // where the journal ends, then the tail from the newest minus 512.
        let names: Vec<&str> = machine.asked.iter().map(|(name, _)| name.as_str()).collect();
        assert_eq!(
            names,
            [THE_PER_JOB_NOTE_READING_REQUEST, THE_READ_FORWARD_REQUEST, THE_READ_FORWARD_REQUEST, THE_KEPT_CRASH_FILES_REQUEST]
        );
        assert_eq!(machine.reads_forward_asked(), [PAST_EVERY_SEQUENCE, 2000 - 512]);
    }

    /// **A machine whose answers carry fewer than 512 at a time is asked again
    /// from past the last one it gave**, one request after another, and the
    /// tail is still the whole 512.
    #[test]
    fn a_tail_larger_than_one_answer_is_asked_for_again_until_it_is_whole() {
        let mut machine = AMachineInHand::holding(0, 5000);
        machine.per_read = 100;
        let take = the_take_from(&mut machine);
        let tail = whole(&take.the_journal_tail);
        assert_eq!((tail.the_first_sequence_held, tail.the_last_sequence_held), (4488, 4999));
        assert_eq!(tail.entries.lines().count(), 512);
        assert_eq!(
            machine.reads_forward_asked(),
            [PAST_EVERY_SEQUENCE, 4488, 4588, 4688, 4788, 4888, 4988]
        );
    }

    /// **A machine mid-render keeps journalling while the take reads.** The
    /// tail is the 512 entries that ended at the newest the take first saw,
    /// and nothing written after it is taken.
    #[test]
    fn entries_written_during_the_take_do_not_move_the_tail() {
        let mut machine = AMachineInHand::holding(0, 1000);
        machine.per_read = 200;
        machine.grows_by_after_each_read = 7;
        let take = the_take_from(&mut machine);
        let tail = whole(&take.the_journal_tail);
        assert_eq!((tail.the_first_sequence_held, tail.the_last_sequence_held), (488, 999));
        assert_eq!(tail.entries.lines().count(), 512);
    }

    // ── A journal shorter than 512 ───────────────────────────────────────────

    /// **A journal shorter than 512 gives what it holds, whole**, and says so
    /// by its span — never *incomplete* for being short.
    #[test]
    fn a_journal_shorter_than_512_gives_what_it_holds_whole() {
        let mut machine = AMachineInHand::holding(0, 40);
        let take = the_take_from(&mut machine);
        let tail = whole(&take.the_journal_tail);
        assert_eq!(
            (tail.the_first_sequence_held, tail.the_last_sequence_held, tail.the_journals_oldest_retained),
            (0, 39, 0)
        );
        assert_eq!(tail.entries.lines().count(), 40);
        assert_eq!(machine.reads_forward_asked(), [PAST_EVERY_SEQUENCE, 0]);

        // **And one whose oldest entries were already dropped** starts where
        // the journal now starts, and says that is where it starts.
        let mut machine = AMachineInHand::holding(900, 1100);
        let tail = whole(&the_take_from(&mut machine).the_journal_tail);
        assert_eq!(
            (tail.the_first_sequence_held, tail.the_last_sequence_held, tail.the_journals_oldest_retained),
            (900, 1099, 900)
        );
    }

    /// **A journal with no entries is *absent***, and the note is untouched by
    /// it.
    #[test]
    fn a_journal_with_no_entries_is_absent() {
        let mut machine = AMachineInHand::holding(0, 0);
        let take = the_take_from(&mut machine);
        assert_eq!(take.the_journal_tail, Taken::NothingThere { why: None });
        assert!(matches!(take.the_per_job_note, Taken::Whole(_)));
        assert_eq!(machine.reads_forward_asked(), [PAST_EVERY_SEQUENCE]);
    }

    // ── The note's own states ────────────────────────────────────────────────

    /// **The note's states, each the one `Reporting a problem` S5 gives it.**
    /// No note on the machine is *absent*; a machine that cannot say, or that
    /// refuses the reading, did not finish the read — **never *absent* over a
    /// note that may exist**.
    #[test]
    fn the_note_is_absent_only_where_the_machine_says_it_holds_none() {
        let mut machine = AMachineInHand::holding(0, 10);
        machine.note = Some(Fields::none().and(ON_THIS_MACHINE_FIELD, IT_IS_ABSENT));
        assert_eq!(the_take_from(&mut machine).the_per_job_note, Taken::NothingThere { why: None });

        for answer in [
            Fields::none().and(ON_THIS_MACHINE_FIELD, THE_MACHINE_CANNOT_SAY),
            Fields::none(),
            Fields::none().and(ON_THIS_MACHINE_FIELD, "a word this build does not know"),
        ] {
            let mut machine = AMachineInHand::holding(0, 10);
            machine.note = Some(answer);
            assert_eq!(
                the_take_from(&mut machine).the_per_job_note,
                Taken::BeganAndDidNotComplete { what_arrived: None }
            );
        }
    }

    // ── An unreachable machine ───────────────────────────────────────────────

    /// **A server not connected at the press is *unreachable* for both
    /// parts**, which is a statement about the cut and not *absent*
    /// (`Reporting a problem` S5 and `Reporting a problem` S6).
    #[test]
    fn an_unreachable_machine_is_unreachable_for_both_parts() {
        let take = the_take(Some(&a_job()), None).expect("a report naming a job has a take");
        assert_eq!(take, TheTake::unreachable());
        assert_eq!(take.the_per_job_note, Taken::Unreachable);
        assert_eq!(take.the_journal_tail, Taken::Unreachable);
        assert_eq!(take.the_crash_file, Taken::Unreachable);
    }

    /// **A report that names no job touches no machine at all**
    /// (`Reporting a problem` G1), even with one connected and in hand.
    #[test]
    fn a_report_naming_no_job_asks_nothing() {
        let mut machine = AMachineInHand::holding(0, 100);
        assert_eq!(the_take(None, Some(&mut machine)), None);
        assert!(machine.asked.is_empty());
    }

    // ── A cut mid-take ───────────────────────────────────────────────────────

    /// **The connection ends after the note**: the note is whole, and the
    /// tail, whose read began on a live connection, is *incomplete at the cut*
    /// with nothing arrived — never *unreachable*, which would say the server
    /// could not be reached when the report was cut.
    #[test]
    fn a_cut_after_the_note_leaves_the_tail_incomplete() {
        let mut machine = AMachineInHand::holding(0, 2000);
        machine.the_line_ends_after = Some(1);
        let take = the_take_from(&mut machine);
        assert!(matches!(take.the_per_job_note, Taken::Whole(_)));
        assert_eq!(take.the_journal_tail, Taken::BeganAndDidNotComplete { what_arrived: None });
    }

    /// **The connection ends part-way through the tail**: what arrived is
    /// carried, its span stated, and the part is *incomplete at the cut* — not
    /// *included* over a short tail.
    #[test]
    fn a_cut_inside_the_tail_carries_what_arrived_and_says_it_did_not_finish() {
        let mut machine = AMachineInHand::holding(100, 5000);
        machine.per_read = 100;
        // The note, the probe, then three reads of a hundred, then nothing.
        machine.the_line_ends_after = Some(5);
        let take = the_take_from(&mut machine);
        match take.the_journal_tail {
            Taken::BeganAndDidNotComplete { what_arrived: Some(tail) } => {
                assert_eq!(tail.the_first_sequence_held, 4488);
                assert_eq!(tail.the_last_sequence_held, 4787);
                assert_eq!(tail.the_journals_oldest_retained, 100);
                assert_eq!(tail.entries.lines().count(), 300);
            }
            other => panic!("expected an incomplete tail with what arrived, got {other:?}"),
        }
    }

    /// **The connection ends at the first ask**: both parts began on a live
    /// connection and neither finished.
    #[test]
    fn a_cut_at_the_first_ask_leaves_both_parts_incomplete() {
        let mut machine = AMachineInHand::holding(0, 2000);
        machine.the_line_ends_after = Some(0);
        let take = the_take_from(&mut machine);
        assert_eq!(take.the_per_job_note, Taken::BeganAndDidNotComplete { what_arrived: None });
        assert_eq!(take.the_journal_tail, Taken::BeganAndDidNotComplete { what_arrived: None });
    }

    /// **An answer whose entry will not read back is a read that did not
    /// finish**, as the read forward's own reader rules: taking the entries
    /// around it would lose it without saying so.
    #[test]
    fn an_entry_that_will_not_read_back_makes_the_read_incomplete() {
        struct OneBadEntry(AMachineInHand);
        impl AsksTheSupervisor for OneBadEntry {
            fn ask(&mut self, request: &Request) -> WhatCameBack {
                match self.0.ask(request) {
                    WhatCameBack::ItAnswered(answered)
                        if request.name == THE_READ_FORWARD_REQUEST
                            && answered.fields.text(HOW_MANY_EVENTS_FIELD) != Some("0") =>
                    {
                        // **The second entry's sequence number is not a number.**
                        WhatCameBack::ItAnswered(Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "test").carrying(
                            Fields::none()
                                .and(HOW_MANY_EVENTS_FIELD, "2")
                                .and(THE_NEXT_SEQUENCE_FIELD, answered.fields.text(THE_NEXT_SEQUENCE_FIELD).unwrap())
                                .and(THE_OLDEST_RETAINED_FIELD, "0")
                                .and(&the_field_of_item(AN_EVENT, 0, THE_SEQUENCE_FIELD), "8")
                                .and(&the_field_of_item(AN_EVENT, 0, THE_KIND_FIELD), "a kind")
                                .and(&the_field_of_item(AN_EVENT, 1, THE_SEQUENCE_FIELD), "not a number"),
                        ))
                    }
                    other => other,
                }
            }
        }
        let mut machine = OneBadEntry(AMachineInHand::holding(0, 10));
        let take = the_take(Some(&a_job()), Some(&mut machine)).unwrap();
        assert_eq!(take.the_journal_tail, Taken::BeganAndDidNotComplete { what_arrived: None });
    }

    // ── No cursor left behind ────────────────────────────────────────────────

    /// **The take registers no cursor and moves none** (`Reporting a problem`
    /// G2): the cursor move is never sent, every read forward carries the
    /// cursor field and nothing else, and the machine's registration is where
    /// it was — under every shape of take above.
    #[test]
    fn no_cursor_is_left_behind() {
        let shapes: [fn(&mut AMachineInHand); 5] = [
            |_| {},
            |machine| machine.per_read = 64,
            |machine| machine.the_line_ends_after = Some(3),
            |machine| machine.grows_by_after_each_read = 3,
            |machine| machine.note = None,
        ];
        for shape in shapes {
            let mut machine = AMachineInHand::holding(10, 3000);
            shape(&mut machine);
            let _ = the_take_from(&mut machine);
            assert_eq!(machine.registered, None, "the take left a cursor registered");
            assert_eq!(
                machine.how_many_times(THE_CURSOR_MOVES_FORWARD_REQUEST),
                0,
                "the take sent the request that moves the desktop's cursor"
            );
            let only: HashSet<&str> = machine.asked.iter().map(|(name, _)| name.as_str()).collect();
            assert!(only.is_subset(&HashSet::from([
                THE_PER_JOB_NOTE_READING_REQUEST,
                THE_READ_FORWARD_REQUEST,
                THE_KEPT_CRASH_FILES_REQUEST
            ])));
            for (name, fields) in &machine.asked {
                if name == THE_READ_FORWARD_REQUEST {
                    assert_eq!(fields.how_many(), 1, "a read forward carries the cursor and nothing else");
                    assert!(fields.text(THE_CURSOR_FIELD).is_some());
                }
            }
        }
    }

    impl AMachineInHand {
        fn how_many_times(&self, name: &str) -> usize {
            self.asked.iter().filter(|(asked, _)| asked == name).count()
        }
    }

    // ── The parts arrive in the shape the report is composed from ─────────────

    /// **The take's parts are the report file's own** — step 11's report cut
    /// composes from exactly these, so the file carries the tail's span and
    /// the note's text unchanged.
    #[test]
    fn the_take_composes_into_the_report_as_it_was_taken() {
        use smoozy_reporting::lines::{Line, LineKind};
        use smoozy_reporting::report::Slice;
        use smoozy_reporting::report_file::{compose, OpenedFrom, WhatTheMachineAdds, WhatTheReportIsMadeFrom};
        use smoozy_state::observed::Clock;

        let job = a_job();
        let mut machine = AMachineInHand::holding(300, 2000);
        let take = the_take(Some(&job), Some(&mut machine)).unwrap();
        let run = Identity::issue(IdentityKind::Run).unwrap();
        let diary = vec![Line {
            run,
            kind: LineKind::Failure,
            at: SystemTime::now(),
            clock: Clock::ThisComputer,
            concerning: vec![job.clone()],
            text: "Render failed.".to_string(),
        }];
        let adds = WhatTheMachineAdds {
            the_captured_output: Vec::new(),
            the_journal_tail: take.the_journal_tail.clone(),
            the_crash_file: take.the_crash_file.clone(),
            the_per_job_note: take.the_per_job_note.clone(),
        };
        let file = compose(&WhatTheReportIsMadeFrom {
            the_users_sentence: "It stopped.",
            opened_from: &OpenedFrom::TheMenu,
            smoozys_version: "0.1.8",
            the_system: "macOS",
            made_at: SystemTime::now(),
            the_diary: &diary,
            the_cap_has_dropped_older_lines: false,
            the_slice: &Slice::AFailedJob { job: job.clone() },
            what_the_machine_adds: Some(&adds),
        });
        assert!(file.text.contains("entries 1488 to 1999; the log's oldest retained entry is 300"));
        assert!(file.text.contains(&whole(&take.the_journal_tail).entries));
        assert!(file.text.contains(&whole(&take.the_per_job_note)));
    }
    fn crash_answer(attempts: &[(u64, &str, &str, &str)]) -> Fields {
        let mut fields = Fields::none()
            .and(ON_THIS_MACHINE_FIELD, IT_IS_PRESENT)
            .and(HOW_MANY_CRASH_FILE_STATEMENTS_FIELD, attempts.len().to_string());
        for (at, (attempt, name, state, content)) in attempts.iter().enumerate() {
            let item = |field: &str| the_field_of_item(A_CRASH_FILE_STATEMENT, at, field);
            fields = fields
                .and(&item(THE_ATTEMPT_FIELD), attempt.to_string())
                .and(&item(THE_CRASH_FILES_OWN_NAME_FIELD), *name)
                .and(&item(WHAT_BECAME_OF_THE_CRASH_FILE_FIELD), *state)
                .and(&item(if *state == THE_CRASH_FILE_WAS_KEPT {
                    THE_CRASH_FILE_WHOLE_FIELD
                } else {
                    WHY_NO_CRASH_FILE_WAS_KEPT_FIELD
                }), *content);
        }
        fields
    }

    fn report_with_crash_answer(answer: Option<Fields>) -> (String, Vec<(String, Fields)>, Identity) {
        use smoozy_reporting::report::Slice;
        use smoozy_reporting::report_file::{compose, OpenedFrom, WhatTheReportIsMadeFrom};
        let job = a_job();
        let mut machine = AMachineInHand::holding(0, 10);
        machine.crash_files = answer;
        let take = the_take(Some(&job), Some(&mut machine)).unwrap();
        // The report box's real handover, so a take the box ignores fails too.
        let adds = crate::surface::reporting::what_the_machine_adds(take, Vec::new());
        let file = compose(&WhatTheReportIsMadeFrom {
            the_users_sentence: "It stopped.",
            opened_from: &OpenedFrom::TheMenu,
            smoozys_version: "0.1.8",
            the_system: "macOS",
            made_at: SystemTime::now(),
            the_diary: &[],
            the_cap_has_dropped_older_lines: false,
            the_slice: &Slice::AFailedJob { job: job.clone() },
            what_the_machine_adds: Some(&adds),
        });
        (file.text, machine.asked, job)
    }

    #[test]
    fn a_jobs_report_carries_each_kept_crash_file_whole_under_its_name_and_attempt() {
        let content = "# Blender version\nbacktrace /library/module 0x123\n\nPython: line 42\n";
        let older = "earlier backtrace\r\nno final newline";
        let (text, asked, job) = report_with_crash_answer(Some(crash_answer(&[
            (3, "Kitchen final.crash.txt", THE_CRASH_FILE_WAS_KEPT, content),
            (2, "Kitchen final.crash.txt", NO_CRASH_FILE_WAS_KEPT, "written before this attempt's launch"),
            (1, "Kitchen final.crash.txt", THE_CRASH_FILE_WAS_KEPT, older),
        ])));
        assert!(text.contains(content));
        assert!(text.contains(older));
        assert!(text.contains("Kitchen final.crash.txt"));
        for words in ["attempt 3", "attempt 2", "attempt 1", "written before this attempt's launch"] {
            assert!(text.contains(words), "missing {words}: {text}");
        }
        let reads: Vec<_> = asked.iter().filter(|(name, _)| name == THE_KEPT_CRASH_FILES_REQUEST).collect();
        assert_eq!(reads.len(), 1);
        assert_eq!(reads[0].1.text(THE_JOB_FIELD), Some(job.as_text().as_str()));
        assert_eq!(reads[0].1.how_many(), 1);
    }

    #[test]
    fn a_jobs_report_without_a_crash_file_says_absent_and_carries_the_reason() {
        for answer in [
            crash_answer(&[]),
            Fields::none().and(ON_THIS_MACHINE_FIELD, IT_IS_ABSENT),
            crash_answer(&[(1, "Kitchen.crash.txt", NO_CRASH_FILE_WAS_KEPT, "nothing stands at /tmp/Kitchen.crash.txt")]),
        ] {
            let (text, asked, _) = report_with_crash_answer(Some(answer.clone()));
            assert!(asked.iter().any(|(name, _)| name == THE_KEPT_CRASH_FILES_REQUEST));
            assert!(text.contains("absent"));
            assert!(text.contains("This does not show how the render ended"));
            assert!(!text.contains("== Blender's crash file,"));
            if answer.text(HOW_MANY_CRASH_FILE_STATEMENTS_FIELD) == Some("1") {
                assert!(text.contains("nothing stands at /tmp/Kitchen.crash.txt"));
                assert!(text.contains("attempt 1"));
            }
        }
    }

    #[test]
    fn an_oversize_crash_file_is_dropped_whole_and_said_so_in_the_report() {
        use smoozy_reporting::report::CEILING_BYTES;
        // Deliberately larger than a real answer allows: exercise the report
        // ceiling through the take and the report box's handover together.
        let content = format!("FIRST CRASH BYTE{}LAST CRASH BYTE", "x".repeat(CEILING_BYTES as usize));
        let (text, _, _) = report_with_crash_answer(Some(crash_answer(&[
            (1, "large.crash.txt", THE_CRASH_FILE_WAS_KEPT, &content),
        ])));
        assert!(text.len() <= CEILING_BYTES as usize);
        assert!(text.contains("omitted in full to keep this report under 5 MB"), "{text}");
        assert!(!text.contains("FIRST CRASH BYTE"));
        assert!(!text.contains("LAST CRASH BYTE"));
        assert!(!text.contains("== Blender's crash file,"));
    }

    #[test]
    fn an_older_machine_without_the_crash_reading_still_gives_a_report() {
        let (text, asked, _) = report_with_crash_answer(None);
        assert!(asked.iter().any(|(name, _)| name == THE_KEPT_CRASH_FILES_REQUEST));
        assert!(text.contains("absent"));
        assert!(text.contains("cannot send crash files"), "{text}");
        assert!(text.contains("This does not show how the render ended"));
        assert!(text.contains("reason the machine recorded: Blender stopped: signal 11"));
        assert!(text.contains("entries 0 to 9"));
    }

    #[test]
    fn a_crash_read_cut_after_the_tail_leaves_the_other_parts_whole() {
        let mut machine = AMachineInHand::holding(0, 10);
        machine.the_line_ends_after = Some(3);
        let take = the_take_from(&mut machine);
        assert!(matches!(take.the_per_job_note, Taken::Whole(_)));
        assert!(matches!(take.the_journal_tail, Taken::Whole(_)));
        assert_eq!(take.the_crash_file, Taken::BeganAndDidNotComplete { what_arrived: None });
    }

    #[test]
    fn a_copy_not_served_whole_is_incomplete_with_the_other_attempts_kept_whole() {
        for (state, why) in [
            (THE_KEPT_CRASH_FILE_DID_NOT_FIT, None),
            (THE_KEPT_CRASH_FILE_DID_NOT_READ_BACK, Some("the copy is shorter than recorded")),
        ] {
            let mut answer = crash_answer(&[
                (2, "Kitchen.crash.txt", state, ""),
                (1, "Kitchen.crash.txt", THE_CRASH_FILE_WAS_KEPT, "WHOLE EARLIER COPY\n"),
            ]);
            if let Some(why) = why {
                answer = answer.and(&the_field_of_item(A_CRASH_FILE_STATEMENT, 0, THE_MACHINES_OWN_WORDS_FIELD), why);
            }
            let (text, _, _) = report_with_crash_answer(Some(answer));
            assert!(text.contains("incomplete"));
            assert!(text.contains("WHOLE EARLIER COPY\n"));
            assert!(text.contains("attempt 2"));
            assert!(!text.contains("Blender's crash file: absent"));
            assert!(!text.contains("Blender's crash file: included"));
            assert!(text.contains(why.unwrap_or("too large to send")));
        }
    }

    #[test]
    fn an_unreadable_crash_answer_is_never_absent_or_lossily_converted() {
        for answer in [
            Fields::none(),
            Fields::none().and(ON_THIS_MACHINE_FIELD, THE_MACHINE_CANNOT_SAY),
            Fields::none().and(ON_THIS_MACHINE_FIELD, IT_IS_PRESENT)
                .and(HOW_MANY_CRASH_FILE_STATEMENTS_FIELD, usize::MAX.to_string()),
            Fields::none().and(ON_THIS_MACHINE_FIELD, IT_IS_PRESENT)
                .and(HOW_MANY_CRASH_FILE_STATEMENTS_FIELD, "1")
                .and(&the_field_of_item(A_CRASH_FILE_STATEMENT, 0, THE_ATTEMPT_FIELD), "1")
                .and(&the_field_of_item(A_CRASH_FILE_STATEMENT, 0, THE_CRASH_FILES_OWN_NAME_FIELD), "Kitchen.crash.txt")
                .and(&the_field_of_item(A_CRASH_FILE_STATEMENT, 0, WHAT_BECAME_OF_THE_CRASH_FILE_FIELD), THE_CRASH_FILE_WAS_KEPT)
                .and(&the_field_of_item(A_CRASH_FILE_STATEMENT, 0, THE_CRASH_FILE_WHOLE_FIELD), vec![0xff, 0xfe]),
            crash_answer(&[(1, "Kitchen.crash.txt", "unknown future state", "")]),
        ] {
            let (text, _, _) = report_with_crash_answer(Some(answer));
            assert!(text.contains("Blender's crash file: incomplete"), "{text}");
            assert!(!text.contains('\u{fffd}'));
        }
    }

}
