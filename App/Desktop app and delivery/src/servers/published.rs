//! The real end of `Servers`' publishing seam — where the values that module
//! supplies go.
//!
//! **`Servers` K7 governs this whole file**: *"`Servers` supplies the observed
//! value and the transitions, and presents neither."* So nothing here draws
//! anything, nothing here decides what a value is called on a screen, and
//! nothing here re-takes a judgement that module already took. **Every
//! consumer of the six calls below was built before this file existed** — the
//! repairing server's three readings, the countdown that ticks against the
//! moment, the could-not-restore notification, the changed-identity dialog and
//! the server row's own host-key column. **This is the join and nothing more.**
//!
//! # Where each of the six goes
//!
//! - **The observed value** (`Servers` K6) and **the moment the next attempt is
//!   due** (`Servers` N10a) are **run-held** — process memory for the
//!   application run and nowhere else, which is `State, identity and recovery`
//!   J2's class, `Servers` S3's rule for a period and `Queue & History` C6's
//!   for the reading beside it. **Nothing about either reaches the local
//!   record**, and [`tests::the_run_held_readings_reach_no_durable_store`] is
//!   what says so.
//! - **Raising the connection condition** (`Servers` N5) writes
//!   `State, identity and recovery` §8's condition record and then tells the
//!   operating system, which is `Desktop app and delivery` T13 — *raised from
//!   the condition record and never from a request failing*.
//! - **Withdrawing it** (`Servers` N10) is the observed value reaching
//!   *connected*, and the record is withdrawn by that change alone.
//! - **The changed-identity warning** (`Servers` Q6's third path) goes to the
//!   page as an event. **How often it may be raised is `Servers` Q17's and is
//!   already bounded on that side**, so nothing here counts anything.
//! - **The scanned host key** (`Servers` D8) is the one durable write of a
//!   value this seam carries, and `Servers` D9 bars every other destination:
//!   never shown, never logged, never handed to another module.
//!
//! # Everything that reaches the page reaches it as an event
//!
//! `Desktop app and delivery` P6: results reach the interface as ordered
//! events, and the interface holds no other route to a module. **Nothing here
//! is answering a command**, so there is no caller to return a reading to and
//! the queue is the only way out. The kinds are declared below, beside the
//! service that posts them, which is the rule `src/surface/`'s head states and
//! `src/start.rs` already applies to a service the page does not call.
//!
//! **P7's two doors are both used and the choice between them is not a
//! preference.** The raise and the withdrawal are durable acts, so they go
//! through the write door and an act that did not land posts nothing. The two
//! run-held readings go through the read door, and `Servers` N10a says in its
//! own words why that is not a fudge: *"It is a reading and not an outcome, so
//! `Desktop app and delivery` P7's write-then-post rule has nothing to bind
//! here: there is no durable state behind it to write first."*
//!
//! # What this file does not build, because it exists
//!
//! It draws no failure. A raised record reaches a screen through
//! [`crate::surface::the_failure_shape`], which reads the records back and
//! composes the drawing; a second composition here would be the same fact drawn
//! twice. It bounds no warning, counts no attempt, and derives no presentation
//! — `Servers` K8 and `Servers` K8a's two presentations and
//! `Desktop app and delivery` M29's column word are all composed where they are
//! drawn, out of the value this file publishes.
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this
//! crate's own plan, `Preparation/Architecture/Desktop app and delivery.md`.
//! Every other document's marker is written with that document — at the site,
//! or in this file's own header list below. **Where a marker is a clause of
//! this crate's own plan *and* of the document meant, the header list is not
//! enough and the document is written out at the site**, because a bare one
//! there resolves to a real and wrong clause.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Servers` N5, N8b, N10, N10a, Q17, D8, D9, D22.
//!
//! **A marker this crate's own plan also defines is deliberately not listed
//! above, whichever document is meant, and is written out with its document at
//! every site instead** — the header list cannot tell the two apart, and a bare
//! one would resolve to a real and wrong clause.

use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use smoozy_servers::acting::WhatServersPublishes;
use smoozy_servers::condition::ObservedValue;
use smoozy_servers::repair::{NextAttemptDue, WhatTheRecordSays};
use smoozy_servers::store::RememberedHostKey;
use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_state::observed::{Clock, Reading};

use crate::events::{Event, Said, Subject, TheOneQueue};
use crate::notifications::{HowTheRecordEnded, TheNotificationService};
use crate::presentation::as_of_that_moment::AsOfThatMoment;
use crate::presentation::declarations::{ACondition, Declared, TheDeclarations};
use crate::presentation::said_once::TheThingsSaidOnce;
use crate::record::failure_records::{ConditionKey, DurableSubject, FailureRecords};
use crate::record::server_rows;
use crate::start::Started;
use crate::typography::Verbatim;

// --------------------------------------------------------- what the page hears

/// **The observed value and the moment the next attempt is due, together, as
/// one reading of one server.**
///
/// **One kind for the two because they are one answer to one question** — what
/// Smoozy now observes about this server's connection — and because the two are
/// held together and read out together. `Desktop app and delivery` M28's first
/// and second readings are composed from this event and from nothing else.
pub const WHAT_IS_OBSERVED: &str = "servers:what-is-observed";

/// **A connection condition raised or withdrawn** — `Servers` N5 and N10.
///
/// **What the user reads about it is not on this event.** The record's own
/// drawing is [`crate::surface::the_failure_shape`]'s, read back out of the
/// local record; this says the durable fact moved, and it carries the condition
/// identity so that whatever is showing one can tell which.
pub const THE_CONNECTION_CONDITION: &str = "servers:the-connection-condition";

/// **`Servers` Q6's third path: the machine offered an identity that is not the
/// one remembered, and the user decides.**
pub const THE_CHANGED_IDENTITY_WARNING: &str = "servers:the-changed-identity-warning";

/// What [`THE_CONNECTION_CONDITION`] says happened. Two words and there is no
/// third: `State, identity and recovery` §8.8's other two endings are not this
/// seam's.
const RAISED: &str = "raised";
const WITHDRAWN: &str = "withdrawn";

/// **`Servers` N10a's *absent*, written out.**
///
/// *"Absent is a value rather than a gap."* The event carries the whole reading
/// every time, so the moment's field is always present and an empty one is the
/// absent value rather than a field nobody filled in.
const ABSENT: &str = "";

// ------------------------------------------------- what this module declares

/// The module name `State, identity and recovery` F12's named supplier is
/// written as, and the name `src/start.rs` registers the declaration under.
pub const THE_MODULE: &str = "Servers";

/// **The condition's kind, in `State, identity and recovery` §8.3's own
/// words.** It is part of the condition key, and it is what makes this
/// condition and a supervisor that is not answering two conditions on one
/// server row rather than one.
pub const THE_KIND: &str = "the server connection";

/// **`Servers` K6's three values, written the way the condition record writes
/// them.**
///
/// **The written form is here and in no second place.** The set the condition
/// holds over and the value it is withdrawn on are both composed from this
/// function below, so a value's spelling cannot drift between the raise, the
/// withdrawal and the event.
pub const fn the_written_form(value: ObservedValue) -> &'static str {
    match value {
        ObservedValue::Connected => "connected",
        ObservedValue::Reconnecting => "reconnecting",
        ObservedValue::Disconnected => "disconnected",
    }
}

/// **`Servers` K7: it holds over *disconnected* and *reconnecting*.**
const HOLDS_OVER: &[&str] = &[
    the_written_form(ObservedValue::Disconnected),
    the_written_form(ObservedValue::Reconnecting),
];

/// **`Servers` K7 and `Servers` N10: withdrawn on *connected*.**
const WITHDRAWN_ON: &str = the_written_form(ObservedValue::Connected);

/// **`State, identity and recovery` F12's four columns, all of them.**
///
/// A condition with no named supplier cannot be withdrawn and must not be
/// raised, and the store refuses a raise this register does not carry. **It
/// lives beside the raiser** so that the declaration and the values the raise
/// hands over are the same two constants.
pub const THE_CONNECTION_CONDITION_IS_DECLARED: ACondition = ACondition {
    kind: THE_KIND,
    supplied_by: THE_MODULE,
    holds_over: HOLDS_OVER,
    withdrawn_on: WITHDRAWN_ON,
};

/// **What `Servers` declares that this document's services read** — §4 A7.
///
/// **One condition, no dismissible warning and no partition unit.** A7's
/// hosting rule is that a module with no declaration answers *no* to
/// everything, so nothing absent from here is a default: it is a thing this
/// module does not do through these services.
pub fn what_servers_declares() -> Declared {
    Declared {
        conditions: vec![THE_CONNECTION_CONDITION_IS_DECLARED],
        dismissible_warnings: Vec::new(),
        partition_units: Vec::new(),
    }
}

// --------------------------- the condition the fourth branch raises

/// **`State, identity and recovery` §8.3's fourth row: *the supervisor
/// unavailable while SSH is live*, supplied by `Rented-server runtime`.**
///
/// **It is raised here because the fourth branch is met here** —
/// `State, identity and recovery` Y9: *"The supervisor-unavailable condition of
/// 8.3 is raised"*, and that document's V1 step 1 says the same. **Declared the
/// way `Servers`' own is** ([`THE_CONNECTION_CONDITION_IS_DECLARED`]), under
/// the named supplier §8.3 gives, so `State, identity and recovery` F12's rule
/// — *a condition with no named supplier must not be raised* — is met by the
/// register rather than asserted. *(Added 2026-09-18, step 10b's first audit
/// round, item 8.)*
pub const THE_SUPERVISORS_SUPPLIER: &str = "Rented-server runtime";

/// The condition's kind, in §8.3's own words.
pub const THE_SUPERVISORS_KIND: &str = "the supervisor unavailable while SSH is live";

/// §8.3's *holds over* and *withdrawn on*, in its own words.
const THE_SUPERVISOR_NOT_ANSWERING: &str = "the supervisor not answering";
const THE_SUPERVISOR_ANSWERING: &str = "the supervisor answering";
const THE_SUPERVISOR_HOLDS_OVER: &[&str] = &[THE_SUPERVISOR_NOT_ANSWERING];

pub const THE_SUPERVISORS_CONDITION_IS_DECLARED: ACondition = ACondition {
    kind: THE_SUPERVISORS_KIND,
    supplied_by: THE_SUPERVISORS_SUPPLIER,
    holds_over: THE_SUPERVISOR_HOLDS_OVER,
    withdrawn_on: THE_SUPERVISOR_ANSWERING,
};

/// **Whether a standing failure record is this condition** — a condition held
/// over §8.3's *the supervisor not answering* and nothing else. **The record
/// carries no kind of its own** (§8.2 F10's key holds it and nothing reads the
/// key back), so the one value it is held over is what says which it is; the
/// connection condition is held over *disconnected* and *reconnecting*.
/// Added 2026-09-27, step 10c's eighth-pass fix piece 6.
pub fn is_the_supervisors_condition(record: &crate::record::failure_records::Standing) -> bool {
    record.kind == crate::record::failure_records::Kind::Condition
        && record.the_values_it_was_raised_over == [THE_SUPERVISOR_NOT_ANSWERING]
}

/// **What the supplier declares through this document's services**, and it is
/// the one condition.
pub fn what_the_supervisors_supplier_declares() -> Declared {
    Declared {
        conditions: vec![THE_SUPERVISORS_CONDITION_IS_DECLARED],
        dismissible_warnings: Vec::new(),
        partition_units: Vec::new(),
    }
}

// ------------------ V13: what a server carries while its program is not ours

/// **V13's five conditions, each of its own and never V1's.** Added
/// 2026-09-28, step 10c's transfer wave piece 8.
///
/// **`Server protocol and transfers` P3b is why none of them is *the supervisor
/// not answering*.** Each is presented against the server, withdrawn when the
/// condition ends, and worded by Design — `Design/Servers § A row`'s five row
/// lines, which are the next piece's to draw (the transfer wave's piece 9).
/// **This file raises and withdraws; it draws nothing** (`Servers` K7's rule for
/// the whole file).
///
/// **Run-held, one per server, and nothing durable.** Four of the five are
/// readings: *too old* and *too new* come from package inspection, which is
/// never durable state on this side (V8, and `Server protocol and transfers`
/// B5); *waits* and *under way* from the queue reading and a request in hand,
/// both period-held. **The fifth, *the update failed*, is held the same way,
/// and what that costs is written here rather than found**: V13 calls it a
/// failure record carrying the transaction's own words, and a record of that
/// kind has no declared supplier and no drawing yet. Held for the run, it is
/// what the row line is drawn from; a later update landing withdraws it
/// (V13's own ending), and Smoozy closing drops it — the next connection tries
/// the update again under V12 and either lands or fails again, so nothing is
/// silent for longer than one run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatSmoozyOnTheServerIs {
    /// **The update waits**: the machine is older, work stands on it, and that
    /// work can be controlled from this desktop.
    TheUpdateWaits,
    /// **Too old, and its work cannot be controlled from here**: the protocol
    /// intervals do not overlap and no retained adapter reaches it
    /// (`Server protocol and transfers` P3a). Nothing is offered that would
    /// need one.
    TooOldAndItsWorkCannotBeControlledFromHere,
    /// **Too new** — V14. The way out is this desktop's own update.
    TooNew,
    /// **The update is under way**, from the request's send to its terminal
    /// phase — Row 291's *anything that isn't instant shows that it's
    /// underway*. **One request is outstanding per server** (V4), and this is
    /// what says one is.
    TheUpdateIsUnderWay,
    /// **The update failed**: the transaction ended *rolled back* or *failed
    /// before commit*, or could not be carried out, with the transaction's own
    /// words. The machine is on the release it had.
    TheUpdateFailed { said: String },
}

/// The register, one entry per server, for the run.
fn what_each_server_carries() -> &'static Mutex<HashMap<String, WhatSmoozyOnTheServerIs>> {
    static HELD: std::sync::OnceLock<Mutex<HashMap<String, WhatSmoozyOnTheServerIs>>> =
        std::sync::OnceLock::new();
    HELD.get_or_init(|| Mutex::new(HashMap::new()))
}

/// **V13: the server carries this condition now, or — with nothing — none of
/// the five.** A condition replaces whichever of the five stood; withdrawing is
/// the condition ending. Answers whether anything moved.
pub fn the_server_carries(server: &str, now: Option<WhatSmoozyOnTheServerIs>) -> bool {
    let Ok(mut held) = what_each_server_carries().lock() else {
        return false;
    };
    let before = held.get(server).cloned();
    match now.clone() {
        Some(condition) => {
            held.insert(server.to_string(), condition);
        }
        None => {
            held.remove(server);
        }
    }
    before != now
}

/// **What the server carries now, of V13's five**, or nothing. The row lines
/// are drawn from this (the transfer wave's piece 9).
pub fn what_the_server_carries(server: &str) -> Option<WhatSmoozyOnTheServerIs> {
    what_each_server_carries()
        .lock()
        .ok()
        .and_then(|held| held.get(server).cloned())
}

/// **V4: a request is taken in hand for this server, and nothing where one
/// already is.** The update is under way from here until its ending is told.
pub fn an_update_is_taken_in_hand(server: &str) -> bool {
    let Ok(mut held) = what_each_server_carries().lock() else {
        return false;
    };
    if held.get(server) == Some(&WhatSmoozyOnTheServerIs::TheUpdateIsUnderWay) {
        return false;
    }
    held.insert(server.to_string(), WhatSmoozyOnTheServerIs::TheUpdateIsUnderWay);
    true
}

/// **`Telegram` V1c: whether this server's program takes a change of a live
/// job's reporting**, read off the release sequences step 9 compares — run-held,
/// one per server, nothing durable, like the five above. Added 2026-10-30, the
/// twenty-first notes, note 7.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ALiveJobsReportingCanBeChanged {
    /// The program's release is at or above the one that brings the request.
    Yes,
    /// **Older**: the queue says *Update Smoozy on this server to change
    /// this.* and offers no control. Nothing is guessed from a refusal.
    TheProgramIsOlder,
}

fn what_each_servers_program_takes() -> &'static Mutex<HashMap<String, ALiveJobsReportingCanBeChanged>> {
    static HELD: std::sync::OnceLock<Mutex<HashMap<String, ALiveJobsReportingCanBeChanged>>> =
        std::sync::OnceLock::new();
    HELD.get_or_init(|| Mutex::new(HashMap::new()))
}

/// **The rule, on one machine's active release sequence.** At or above
/// [`smoozy_server_protocol::requests::THE_FIRST_RELEASE_THAT_CHANGES_A_LIVE_JOBS_REPORTING`],
/// or a program built at a desk (`Server protocol and transfers` B10a, placed
/// under sequence 0, which is a development build from the tree and never a
/// published release). A machine with no active package has no program and
/// takes nothing.
pub fn a_program_of_this_release_changes_a_live_jobs_reporting(
    the_machines_release: Option<u64>,
) -> ALiveJobsReportingCanBeChanged {
    let a_desk_program = |sequence: u64| {
        // **A release build has no desk program and no route to one** (`Server protocol and transfers` B10a).
        #[cfg(debug_assertions)]
        {
            sequence == crate::the_desk_program::THE_SEQUENCE_A_DESK_PROGRAM_IS_PLACED_UNDER
        }
        #[cfg(not(debug_assertions))]
        {
            let _ = sequence;
            false
        }
    };
    match the_machines_release {
        Some(sequence)
            if sequence
                >= smoozy_server_protocol::requests::THE_FIRST_RELEASE_THAT_CHANGES_A_LIVE_JOBS_REPORTING
                || a_desk_program(sequence) =>
        {
            ALiveJobsReportingCanBeChanged::Yes
        }
        _ => ALiveJobsReportingCanBeChanged::TheProgramIsOlder,
    }
}

/// **Step 9 read the machine's active release** — or, after an update landed,
/// the machine now runs this application's own.
pub fn the_servers_release_is_read(server: &str, the_machines_release: Option<u64>) {
    if let Ok(mut held) = what_each_servers_program_takes().lock() {
        held.insert(
            server.to_string(),
            a_program_of_this_release_changes_a_live_jobs_reporting(the_machines_release),
        );
    }
    if let Ok(mut held) = what_each_servers_program_counts().lock() {
        held.insert(
            server.to_string(),
            a_program_of_this_release_counts_a_folders_size(the_machines_release),
        );
    }
}

/// **`File Browser` L28: whether each server's program counts a folder's
/// size**, read off the same release as note 7's answer above — run-held,
/// nothing durable. Added 2026-10-30, the twenty-third notes, note 2.
fn what_each_servers_program_counts() -> &'static Mutex<HashMap<String, bool>> {
    static HELD: std::sync::OnceLock<Mutex<HashMap<String, bool>>> = std::sync::OnceLock::new();
    HELD.get_or_init(|| Mutex::new(HashMap::new()))
}

/// **The rule, on one machine's active release sequence**: at or above
/// [`smoozy_server_protocol::requests::THE_FIRST_RELEASE_THAT_COUNTS_A_FOLDERS_SIZE`],
/// or a program built at a desk, exactly as
/// [`a_program_of_this_release_changes_a_live_jobs_reporting`] rules.
pub fn a_program_of_this_release_counts_a_folders_size(the_machines_release: Option<u64>) -> bool {
    #[cfg(debug_assertions)]
    let a_desk_program = |sequence: u64| {
        sequence == crate::the_desk_program::THE_SEQUENCE_A_DESK_PROGRAM_IS_PLACED_UNDER
    };
    #[cfg(not(debug_assertions))]
    let a_desk_program = |_: u64| false;
    the_machines_release.is_some_and(|sequence| {
        sequence >= smoozy_server_protocol::requests::THE_FIRST_RELEASE_THAT_COUNTS_A_FOLDERS_SIZE
            || a_desk_program(sequence)
    })
}

/// **Whether this server's program counts a folder's size**, or nothing where
/// its release has not been read in this run.
pub fn a_folders_size_can_be_counted_on(server: &str) -> Option<bool> {
    what_each_servers_program_counts()
        .lock()
        .ok()
        .and_then(|held| held.get(server).copied())
}

/// **`Server Setup` G2a: the Telegram role's reading, per server** — run-held,
/// nothing durable, and gone with the connection. `asked` numbers the readings
/// so only the newest lands: one asked before a telling must not stand over one
/// asked after it. `failed` is what the dedicated entry point last failed with.
#[derive(Default)]
struct TheTelegramRoleHeld {
    asked: u64,
    enabled: Option<bool>,
    failed: Option<String>,
}

fn the_telegram_roles() -> &'static Mutex<HashMap<String, TheTelegramRoleHeld>> {
    static HELD: std::sync::OnceLock<Mutex<HashMap<String, TheTelegramRoleHeld>>> = std::sync::OnceLock::new();
    HELD.get_or_init(|| Mutex::new(HashMap::new()))
}

/// **A reading of the role is asked for.** What comes back names this number.
pub fn a_reading_of_the_telegram_role_begins(server: &str) -> u64 {
    let Ok(mut held) = the_telegram_roles().lock() else {
        return 0;
    };
    let role = held.entry(server.to_string()).or_default();
    role.asked += 1;
    role.asked
}

/// **A reading came back**: `Some` of whether the role is enabled, or nothing
/// where it could not be read. **Kept only where no later one was asked, and
/// where the connection it was asked on still stands.** Answers whether it moved.
pub fn a_reading_of_the_telegram_role_came_back(server: &str, asked: u64, enabled: Option<bool>) -> bool {
    let Ok(mut held) = the_telegram_roles().lock() else {
        return false;
    };
    let Some(role) = held.get_mut(server).filter(|role| role.asked == asked) else {
        return false;
    };
    let moved = role.enabled != enabled;
    role.enabled = enabled;
    moved
}

/// **Whether this server's Telegram role is enabled**, or nothing where it has
/// not been read, or could not be, on this connection.
pub fn the_telegram_role_on(server: &str) -> Option<bool> {
    the_telegram_roles().lock().ok()?.get(server)?.enabled
}

/// **What the dedicated entry point failed with**, until the next press.
pub fn the_telegram_install_failed_with(server: &str, said: Option<String>) {
    if let Ok(mut held) = the_telegram_roles().lock() {
        held.entry(server.to_string()).or_default().failed = said;
    }
}

/// [`the_telegram_install_failed_with`]'s sentence, where one stands.
pub fn what_the_telegram_install_failed_with(server: &str) -> Option<String> {
    the_telegram_roles().lock().ok()?.get(server)?.failed.clone()
}

/// **The connection ended**: nothing read over it stands, and a reading still on
/// its way does not land.
pub fn the_telegram_role_goes_with_the_connection(server: &str) {
    if let Ok(mut held) = the_telegram_roles().lock() {
        if let Some(role) = held.get_mut(server) {
            role.asked += 1;
            role.enabled = None;
            role.failed = None;
        }
    }
}

/// **Whether this server's program takes the change**, or nothing where its
/// release has not been read in this run.
pub fn a_live_jobs_reporting_can_be_changed_on(server: &str) -> Option<ALiveJobsReportingCanBeChanged> {
    what_each_servers_program_takes()
        .lock()
        .ok()
        .and_then(|held| held.get(server).copied())
}

/// **`Server protocol and transfers` P3b's direction: the machine's active
/// release sequence against this application's own**, and nothing else.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheDirection {
    /// The same release.
    TheSame,
    /// Below this application's own: *too old*, or simply older.
    TheMachineIsOlder,
    /// Above it: *too new*.
    TheMachineIsNewer,
}

/// **The direction, where the machine has an active package at all.** A
/// machine with none has no program to be older or newer than anything, and
/// answers nothing — which is `Server Setup` R2's machine to place on, not V11's
/// to update.
pub fn the_direction(machines: Option<u64>, own: u64) -> Option<TheDirection> {
    let machines = machines?;
    Some(match machines.cmp(&own) {
        std::cmp::Ordering::Equal => TheDirection::TheSame,
        std::cmp::Ordering::Less => TheDirection::TheMachineIsOlder,
        std::cmp::Ordering::Greater => TheDirection::TheMachineIsNewer,
    })
}

/// **What the user is told where this desktop will not place a release below
/// the machine's own** — V14, in the words `Design/Servers § A row` gives the
/// too-new line. It is the one sentence of the five this piece puts in front
/// of a user, because `Server Setup` F1's refusal is said in the Set up box and
/// has to say something.
pub const THE_MACHINE_IS_NEWER: &str =
    "This server has a newer version of Smoozy. Update Smoozy on this computer to use it.";

// ------------------------------------------------ what the run lends this file

/// **The three run-wide things a publish reaches, lent for the length of one
/// act.**
///
/// The record, the register of what the operating system has already been told
/// in this run (§14 Q13), and the declaration register. **They are lent
/// together because the raise needs all three at one moment** and taking the
/// application's one lock three times to do one thing is three chances for the
/// state to move underneath.
pub struct WhatTheRunLends<'a> {
    pub record: &'a rusqlite::Connection,
    pub said_once: &'a TheThingsSaidOnce,
    pub declarations: &'a TheDeclarations,
}

/// **How this file reaches the run, and the lock is taken inside rather than
/// held across anything.**
///
/// This is the shape `src/keys/mod.rs` already uses for the same problem and
/// for the same reason: a publish is reached from whatever thread the period
/// runs on, the record has one writer, and a lock held across a connection's
/// work is a lock held for seconds.
///
/// **A run that lends nothing — no record, or a lock that will not take — makes
/// the durable half of a publish not happen**, which is the same answer a
/// record that refused the write would give. The run-held half is unaffected,
/// because it is this file's own.
pub trait LendsWhatTheRunHolds {
    fn lend(&self, act: &mut dyn FnMut(WhatTheRunLends<'_>));
}

/// **The run itself**, from the application-wide spine.
pub struct TheRunItself<'a> {
    pub started: &'a Mutex<Started>,
}

impl LendsWhatTheRunHolds for TheRunItself<'_> {
    fn lend(&self, act: &mut dyn FnMut(WhatTheRunLends<'_>)) {
        let Ok(started) = self.started.lock() else {
            return;
        };
        let Some(record) = started.record.as_ref() else {
            return;
        };
        act(WhatTheRunLends {
            record: record.connection(),
            said_once: &started.said_once,
            declarations: &started.what_modules_declare,
        });
    }
}

// ------------------------------------------------------------ what is held

/// **What is held about one server for the life of the run, and it is three
/// things.**
///
/// **`Servers` D22 fixes where it starts**: nothing durable records that a
/// server was connected, so a fresh run knows *disconnected* about every server
/// and nothing else. **`Servers` N10a fixes the second**: a fresh launch knows
/// no moment until a break is declared again, which is *absent*.
#[derive(Debug, Clone, PartialEq, Eq)]
struct WhatIsHeldAboutOneServer {
    /// **`Servers` K6's value, run-held** (`Servers` S3, and
    /// `State, identity and recovery` J2's class).
    the_observed_value: ObservedValue,
    /// **`Servers` N10a's moment, run-held beside the condition record and
    /// never inside it** — §8.2 fixes what a condition record holds and a
    /// moment is neither. `Queue & History` C6 is the same rule for the reading
    /// beside it.
    the_next_attempt_is_due: NextAttemptDue,
    /// **The condition identity this run raised, where it raised one.**
    ///
    /// It is held so that the withdrawal can take the operating system's post
    /// down on the key it was made on (T24a). **It is not a second copy of the
    /// record** — §8.2 F5 forbids the record holding a second copy of the
    /// *value*, and this is the identity, which is what §14 Q13's own run-held
    /// register is keyed on too. **A record raised in an earlier run leaves
    /// this absent and that is right**: nothing of that run's posts is standing
    /// in this one.
    the_condition_this_run_raised: Option<Identity>,
    /// Whether the queue has been told this durable subject exists (P8).
    the_queue_has_been_told: bool,
}

impl WhatIsHeldAboutOneServer {
    fn afresh() -> Self {
        WhatIsHeldAboutOneServer {
            the_observed_value: ObservedValue::Disconnected,
            the_next_attempt_is_due: NextAttemptDue::Absent,
            the_condition_this_run_raised: None,
            the_queue_has_been_told: false,
        }
    }
}

// ------------------------------------------------------------ the publisher

/// **The real end of `Servers`' publishing seam.**
///
/// **One per run rather than one per server**, because every call names the
/// server it is about and the run-held readings of every server are one table.
pub struct WhatTheDesktopPublishes<R> {
    the_run: R,
    queue: Arc<TheOneQueue>,
    notifications: Arc<TheNotificationService>,
    held: Mutex<HashMap<String, WhatIsHeldAboutOneServer>>,
}

impl<R: LendsWhatTheRunHolds> WhatTheDesktopPublishes<R> {
    pub fn new(
        the_run: R,
        queue: Arc<TheOneQueue>,
        notifications: Arc<TheNotificationService>,
    ) -> Self {
        WhatTheDesktopPublishes {
            the_run,
            queue,
            notifications,
            held: Mutex::new(HashMap::new()),
        }
    }

    /// Change what is held about one server and read the whole of it back.
    fn hold(
        &self,
        server: &str,
        change: impl FnOnce(&mut WhatIsHeldAboutOneServer),
    ) -> Option<WhatIsHeldAboutOneServer> {
        let mut held = self.held.lock().ok()?;
        let about = held
            .entry(server.to_string())
            .or_insert_with(WhatIsHeldAboutOneServer::afresh);
        change(about);
        Some(about.clone())
    }

    /// **What the run holds about one server, taken without changing it.**
    pub fn what_is_held_about(&self, server: &str) -> Option<(ObservedValue, NextAttemptDue)> {
        let held = self.held.lock().ok()?;
        let about = held.get(server)?;
        Some((about.the_observed_value, about.the_next_attempt_is_due))
    }

    /// **The server row's identity, where the identifier is one.**
    fn the_subject(server: &str) -> Option<DurableSubject> {
        Identity::read_back(IdentityKind::LocalServer, server).map(DurableSubject::ServerRow)
    }

    /// **P8: the queue refuses an event about a durable subject it has not been
    /// told exists, and it never discovers one on its own.**
    ///
    /// **A subject read back out of the local record is a subject that
    /// exists**, which is the rule [`crate::surface::the_failure_shape`] states
    /// and the same fact rather than a second answer to it. It is read once per
    /// server per run; a row that is not there is a server nothing may be
    /// posted about, and no event is composed for one.
    fn the_subject_is_known(&self, server: &str) -> Option<Identity> {
        let identity = Identity::read_back(IdentityKind::LocalServer, server)?;
        let already = self
            .held
            .lock()
            .ok()
            .and_then(|held| held.get(server).map(|about| about.the_queue_has_been_told))
            .unwrap_or(false);
        if already {
            return Some(identity);
        }
        let mut found = false;
        self.the_run.lend(&mut |lent| {
            found = matches!(server_rows::one(lent.record, server), Ok(Some(_)));
        });
        if !found {
            return None;
        }
        self.queue.the_subject_exists(identity.clone());
        self.hold(server, |about| about.the_queue_has_been_told = true);
        Some(identity)
    }

    /// **The two run-held readings, as the page receives them.**
    ///
    /// **Each carries its own moment and the clock it was taken on** —
    /// §16.1 M3a — and both are taken at the one moment this event is composed,
    /// so one pair of moment and clock serves the two. The written form of both
    /// is `src/presentation/as_of_that_moment.rs`'s, read rather than repeated:
    /// a second spelling of a moment here would be the fact in two places.
    fn what_is_observed(about: Subject, held: WhatIsHeldAboutOneServer, at: SystemTime) -> Event {
        let taken = AsOfThatMoment::from(&Reading::taken(
            the_written_form(held.the_observed_value).to_string(),
            at,
            Clock::ThisComputer,
        ));
        let due = match held.the_next_attempt_is_due {
            NextAttemptDue::Absent => ABSENT.to_string(),
            NextAttemptDue::At(moment) => moment
                .duration_since(SystemTime::UNIX_EPOCH)
                .map(|since| since.as_secs().to_string())
                .unwrap_or_else(|_| ABSENT.to_string()),
        };
        Event::about(about, WHAT_IS_OBSERVED)
            .saying("theConditionsValue", Said::Value(taken.value.clone()))
            .saying("theNextAttemptIsDue", Said::Value(Verbatim::new(due)))
            .saying(
                "takenAt",
                Said::Value(Verbatim::new(
                    taken
                        .taken_at
                        .map(|seconds| seconds.to_string())
                        .unwrap_or_default(),
                )),
            )
            .saying(
                "onTheClock",
                Said::Value(Verbatim::new(taken.on_the_clock.clone())),
            )
    }

    /// Post the pair. **Nothing durable is read or written on this path**, which
    /// is what makes the readings run-held rather than merely called so.
    fn publish_what_is_observed(&self, server: &str, held: WhatIsHeldAboutOneServer) {
        let Some(identity) = self.the_subject_is_known(server) else {
            return;
        };
        let about = Subject::Durable(identity);
        let at = SystemTime::now();
        let _ = self
            .queue
            .read_then_post::<WhatIsHeldAboutOneServer, Infallible>(
                || Ok(held.clone()),
                |held| Self::what_is_observed(about.clone(), held.clone(), at),
            );
    }

    /// **The condition record's own event.** It carries the condition identity
    /// by the verbatim route and says nothing about why — `Servers` N5: the
    /// record *"carries no claim about why"*, because Smoozy cannot tell a
    /// network problem from a machine switched off from a machine that no
    /// longer exists.
    fn the_condition_moved(about: Subject, what_happened: &'static str, condition: &str) -> Event {
        Event::about(about, THE_CONNECTION_CONDITION)
            .saying("whatHappened", Said::Value(Verbatim::new(what_happened)))
            .saying("condition", Said::Value(Verbatim::new(condition)))
    }
}

// ------------------------------------- a record that ends takes its drawing

/// **A condition record against a server row ended, so every drawing of it
/// goes** — `Desktop app and delivery` M4 (*"shown while, and only while"* it
/// stands) and `State, identity and recovery` F25's first and third endings,
/// at that document's §8.8.
///
/// **This is the failure shape's own clearing event**,
/// [`crate::surface::the_failure_shape::A_FAILURE_WAS_CLEARED`], with the four
/// values the user's own clearing carries, so every page that draws a failure
/// hears one ending however the record ended. **The window's foot listens for
/// it and for nothing else**: `web/the-failure-shape.js` draws every standing
/// failure the list is told about and takes one away only on this event.
///
/// **Written for**: the user's cross, the one ending that posted it (step 10a,
/// 2026-09-12). **Now also met**: the withdrawal ([`Self::withdraw_the_connection_condition`],
/// [`WhatTheDesktopPublishes::the_supervisor_answered`]) and the row's removal
/// ([`the_drawings_go_with_the_row`]). **Where the record and the world parted
/// company**: until 2026-10-01 those two ended the record and told the page
/// only `servers:the-connection-condition`, which the list hears and the foot
/// does not — so the foot kept *Something is not working.* for the rest of the
/// run with nothing in the record behind it (step 10c's twelfth pass, fault
/// 13).
///
/// **The key is the presentation key of a condition** (§13's
/// [`ThePresentationKey::ACondition`]), the same one the shape's own composer
/// gives a condition record and the operating system's post is made on. A
/// server row carries condition records only — `State, identity and recovery`
/// §8.7 names no ending against one — so there is no other kind to key here.
/// The test `the_foot_says_only_what_the_record_holds_through_a_cut_a_removal_and_a_re_add`
/// holds this key and the drawn one together through the real page.
fn the_drawing_goes(server: &Identity, condition: &Identity) -> Event {
    let subject = DurableSubject::ServerRow(server.clone());
    Event::about(
        Subject::Durable(server.clone()),
        crate::surface::the_failure_shape::A_FAILURE_WAS_CLEARED,
    )
    .saying("subject", Said::Value(Verbatim::new(subject.as_text())))
    .saying("subjectKind", Said::Value(Verbatim::new(subject.kind_as_text())))
    .saying("identityOrMark", Said::Value(Verbatim::new(condition.as_text())))
    .saying(
        "key",
        Said::Value(Verbatim::new(
            crate::presentation::the_presentation_key::ThePresentationKey::ACondition(
                condition.clone(),
            )
            .as_text(),
        )),
    )
}

/// **Each drawing of the conditions that ended goes**, posted after the act
/// that ended them has landed (P7). Nothing durable is read or written here —
/// the ending has already happened — so it goes through the reading door with
/// nothing to read, as `src/surface/servers.rs` posts its answers.
fn the_drawings_go(queue: &TheOneQueue, server: &Identity, ended: &[Identity]) {
    for condition in ended {
        let _ = queue.read_then_post::<(), Infallible>(
            || Ok(()),
            |()| the_drawing_goes(server, condition),
        );
    }
}

/// **The condition records standing against a row, by identity** — read
/// before an act that may end them, so the drawings can be taken down by name
/// once it has.
fn the_conditions_against(record: &rusqlite::Connection, subject: &DurableSubject) -> Vec<Identity> {
    FailureRecords::over(record)
        .standing_against(subject)
        .unwrap_or_default()
        .into_iter()
        .filter(|standing| standing.kind == crate::record::failure_records::Kind::Condition)
        .filter_map(|standing| Identity::read_back(IdentityKind::Condition, &standing.identity_or_mark))
        .collect()
}

/// **The ones of `before` that no longer stand.**
fn the_ones_that_went(record: &rusqlite::Connection, subject: &DurableSubject, before: Vec<Identity>) -> Vec<Identity> {
    let after = the_conditions_against(record, subject);
    before.into_iter().filter(|one| !after.contains(one)).collect()
}

/// **What stands against a row just before its local forget** — `Servers`
/// D17, and `State, identity and recovery` F25's first ending. Read under the
/// same hold of the run the forget is performed under, so nothing can be
/// raised between the reading and the forget.
pub fn what_the_forget_will_take(started: &Started, server: &str) -> Vec<Identity> {
    let (Some(record), Some(identity)) = (
        started.record.as_ref(),
        Identity::read_back(IdentityKind::LocalServer, server),
    ) else {
        return Vec::new();
    };
    the_conditions_against(record.connection(), &DurableSubject::ServerRow(identity))
}

/// **The row's local forget landed, so the drawings of what it took go with
/// it** — `State, identity and recovery` F25's first ending, *"its subject
/// is removed"*, on every page that
/// drew one. Called by the forget's callers after it answers that it landed,
/// with what [`what_the_forget_will_take`] read before it.
pub fn the_drawings_go_with_the_row(queue: &TheOneQueue, server: &str, taken: &[Identity]) {
    let Some(identity) = Identity::read_back(IdentityKind::LocalServer, server) else {
        return;
    };
    the_drawings_go(queue, &identity, taken);
}

impl<R: LendsWhatTheRunHolds> WhatTheDesktopPublishes<R> {
    /// **What a connection read has reached the row, so the page is told the
    /// server's reading again** — the same event, the same run-held values, and
    /// nothing durable read or written here. The list's hardware column is
    /// drawn from the row, and without this it went on saying nothing was
    /// known about a server whose reading had just been kept (seen in a
    /// throwaway copy, step 10b's first audit round, item 8).
    pub(crate) fn the_server_is_told_again(&self, server: &str) {
        let Some(held) = self.hold(server, |_| {}) else {
            return;
        };
        self.publish_what_is_observed(server, held);
    }

    /// **The fourth branch was met on a live connection: the condition is
    /// raised** — `State, identity and recovery` Y9 and V1 step 1.
    ///
    /// **Nothing is told to the operating system.** `Desktop app and delivery`
    /// T13's notification is the connection condition's, and no clause gives
    /// this one a notification of its own; the record is the statement, and
    /// the list draws it through the one failure shape. A raise against a key
    /// that already carries an identity is that identity (`State, identity and
    /// recovery` F11), so a re-take that meets the branch again raises nothing
    /// new.
    pub fn the_supervisor_is_unavailable(&self, server: &str) {
        let Some(identity) = self.the_subject_is_known(server) else {
            return;
        };
        let subject = DurableSubject::ServerRow(identity.clone());
        let about = Subject::Durable(identity);
        let _ = self.queue.write_then_post(
            || {
                let mut raised: Result<Identity, ItDidNotLand> = Err(ItDidNotLand);
                self.the_run.lend(&mut |lent| {
                    let declared = lent
                        .declarations
                        .the_condition_is_declared(THE_SUPERVISORS_SUPPLIER, THE_SUPERVISORS_KIND);
                    let key = ConditionKey::over(&subject, THE_SUPERVISORS_KIND, None);
                    let mut records = FailureRecords::over(lent.record);
                    if let Ok(condition) = records.a_condition_was_raised(
                        &key,
                        &subject,
                        THE_SUPERVISOR_HOLDS_OVER,
                        declared,
                    ) {
                        raised = Ok(condition);
                    }
                });
                raised
            },
            |condition| Self::the_condition_moved(about.clone(), RAISED, &condition.as_text()),
        );
    }

    /// **Whether §8.3's supervisor-unavailable condition stands against this
    /// server**, read from the record that holds it and from nothing kept
    /// beside it. Added 2026-09-27, step 10c's eighth-pass fix piece 6.
    pub fn the_supervisors_condition_stands(&self, server: &str) -> bool {
        let Some(identity) = Identity::read_back(IdentityKind::LocalServer, server) else {
            return false;
        };
        let subject = DurableSubject::ServerRow(identity);
        let mut stands = false;
        self.the_run.lend(&mut |lent| {
            stands = FailureRecords::over(lent.record)
                .standing_against(&subject)
                .map(|standing| standing.iter().any(is_the_supervisors_condition))
                .unwrap_or(false);
        });
        stands
    }

    /// **The supervisor answered: the condition is withdrawn by that value
    /// alone** — §8.3's *withdrawn on*, and §8.2's derivation.
    pub fn the_supervisor_answered(&self, server: &str) {
        let Some(identity) = self.the_subject_is_known(server) else {
            return;
        };
        let subject = DurableSubject::ServerRow(identity.clone());
        let about = Subject::Durable(identity.clone());
        // **The records it took, by identity** — the same reading the
        // connection condition's withdrawal takes, for the same reason.
        let mut went: Vec<Identity> = Vec::new();
        let posted = self.queue.write_then_post(
            || {
                let mut withdrew: Result<(), ItDidNotLand> = Err(ItDidNotLand);
                self.the_run.lend(&mut |lent| {
                    let key = ConditionKey::over(&subject, THE_SUPERVISORS_KIND, None);
                    let before = the_conditions_against(lent.record, &subject);
                    let mut records = FailureRecords::over(lent.record);
                    if let Ok(true) = records.the_observed_value_moved(&key, THE_SUPERVISOR_ANSWERING) {
                        went = the_ones_that_went(lent.record, &subject, before);
                        withdrew = Ok(());
                    }
                });
                withdrew
            },
            |()| Self::the_condition_moved(about.clone(), WITHDRAWN, ""),
        );
        // **M4, as for the connection condition**: *Smoozy isn't responding.*
        // leaves every page that drew it once the supervisor answers.
        if posted.is_ok() {
            the_drawings_go(&self.queue, &identity, &went);
        }
    }
}

/// Why a durable half did not happen. **Nothing branches on it** — it is here
/// so that P7's door has a failure to refuse the post on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItDidNotLand;

/// **The seam's real end, on a shared reference.**
///
/// **Nothing here needs `&mut`**: what the run holds about each server is behind
/// this publisher's own short lock, taken for one change and given back. So the
/// publisher every period shares is reached without a lock of its own around
/// it — which is what keeps it from holding one lock while it asks for the
/// application's. *(Repaired 2026-09-18, step 10b's first audit round, item 2:
/// a period's publisher held its own lock and then took the application's
/// through a lend, while two screen commands held the application's and then
/// took the publisher's; the two could meet and freeze the window for good.)*
impl<R: LendsWhatTheRunHolds> WhatServersPublishes for &WhatTheDesktopPublishes<R> {
    /// **`Servers` K6's three, as they move.**
    fn the_observed_value(&mut self, server: &str, value: ObservedValue) {
        let Some(held) = self.hold(server, |about| about.the_observed_value = value) else {
            return;
        };
        self.publish_what_is_observed(server, held);
    }

    /// **`Servers` N10a: the moment the next attempt is due, or absent.**
    fn the_next_attempt_due(&mut self, server: &str, reading: NextAttemptDue) {
        let Some(held) = self.hold(server, |about| about.the_next_attempt_is_due = reading) else {
            return;
        };
        self.publish_what_is_observed(server, held);
    }

    /// **`Servers` N5: raised at the run's failure and never at the break
    /// declaration**, and `Desktop app and delivery` T13 posts the
    /// could-not-restore notification from the record this raises and never
    /// from a request failing.
    ///
    /// **What the record holds is `State, identity and recovery` §8.2 F5's two
    /// things** — the condition identity and the set of observed values it was
    /// raised over — so `says` is read and not stored. Its own two fields are
    /// what N5 already fixes: the connection is down, and there is no claim
    /// about why. **A raise against a key that already carries an identity is
    /// that identity**, which is F11, so a repair that fails run after run
    /// raises one record and tells the user once.
    fn raise_the_connection_condition(&mut self, server: &str, says: WhatTheRecordSays) {
        debug_assert!(
            says.the_connection_is_down && !says.any_claim_about_why,
            "**`Servers` N5**: the record says only that the connection is down \
             and carries no claim about why"
        );
        let Some(identity) = self.the_subject_is_known(server) else {
            return;
        };
        let subject = DurableSubject::ServerRow(identity.clone());
        let the_server_row = identity.clone();
        let about = Subject::Durable(identity);
        let queue = &self.queue;
        let posted = queue.write_then_post(
            || {
                let mut raised: Result<Identity, ItDidNotLand> = Err(ItDidNotLand);
                self.the_run.lend(&mut |lent| {
                    // **`State, identity and recovery` F12's teeth, asked of the
                    // register rather than asserted here.** A condition with no
                    // named supplier cannot be withdrawn and must not be raised.
                    let declared = lent
                        .declarations
                        .the_condition_is_declared(THE_MODULE, THE_KIND);
                    let key = ConditionKey::over(&subject, THE_KIND, None);
                    let mut records = FailureRecords::over(lent.record);
                    let Ok(condition) =
                        records.a_condition_was_raised(&key, &subject, HOLDS_OVER, declared)
                    else {
                        return;
                    };
                    // **T13, and the gate is inside the route rather than beside
                    // it**: a later failure resolving to a standing identity is
                    // met with silence, which `src/surface/said_once.rs` keeps so
                    // that no raiser can get it wrong by forgetting to ask.
                    let named = server_rows::one(lent.record, server)
                        .ok()
                        .flatten()
                        .map(|row| crate::surface::servers::what_a_server_is_called(&row))
                        .unwrap_or_default();
                    crate::surface::said_once::a_condition_was_raised_is_told_to_the_operating_system(
                        lent.said_once,
                        &self.notifications,
                        &condition,
                        &the_server_row,
                        Verbatim::new(named),
                    );
                    raised = Ok(condition);
                });
                if let Ok(condition) = raised.as_ref() {
                    self.hold(server, |about| {
                        about.the_condition_this_run_raised = Some(condition.clone())
                    });
                }
                raised
            },
            |condition| WhatTheDesktopPublishes::<R>::the_condition_moved(about.clone(), RAISED, &condition.as_text()),
        );
        let _ = posted;
    }

    /// **`Servers` N10: withdrawn by the observed value reaching *connected*.**
    ///
    /// **It is §8.2's derivation and needs no act of `Servers`'.** What happens
    /// here is that the value is reported to the record, which withdraws itself
    /// where the value has moved outside the set it was raised over — nothing
    /// sweeps, nothing polls and nothing decides here whether the condition
    /// still holds.
    ///
    /// **The operating system's post comes down on the identity this run
    /// raised** (T24a, and `State, identity and recovery` F25's first of three
    /// endings). A record raised in an earlier run has no post standing in this
    /// one, so there is nothing to take down and nothing is invented to take
    /// down.
    fn withdraw_the_connection_condition(&mut self, server: &str) {
        let Some(identity) = self.the_subject_is_known(server) else {
            return;
        };
        let subject = DurableSubject::ServerRow(identity.clone());
        let about = Subject::Durable(identity);
        let raised_this_run = self.held.lock().ok().and_then(|held| {
            held.get(server)
                .and_then(|a| a.the_condition_this_run_raised.clone())
        });
        let queue = &self.queue;
        // **The records the withdrawal took, by identity**, so their drawings
        // can go by name — see [`the_drawing_goes`]. Read inside the same lend
        // as the withdrawal, before and after it.
        let mut went: Vec<Identity> = Vec::new();
        let posted = queue.write_then_post(
            || {
                let mut withdrew: Result<(), ItDidNotLand> = Err(ItDidNotLand);
                self.the_run.lend(&mut |lent| {
                    let key = ConditionKey::over(&subject, THE_KIND, None);
                    let before = the_conditions_against(lent.record, &subject);
                    let mut records = FailureRecords::over(lent.record);
                    let Ok(took_one_away) = records.the_observed_value_moved(&key, WITHDRAWN_ON)
                    else {
                        return;
                    };
                    if !took_one_away {
                        return;
                    }
                    went = the_ones_that_went(lent.record, &subject, before);
                    if let Some(condition) = raised_this_run.as_ref() {
                        crate::surface::said_once::the_condition_record_ended(
                            lent.said_once,
                            &self.notifications,
                            condition,
                            HowTheRecordEnded::TheObservedValueMovedOutOfTheSet,
                        );
                    }
                    withdrew = Ok(());
                });
                if withdrew.is_ok() {
                    self.hold(server, |about| about.the_condition_this_run_raised = None);
                }
                withdrew
            },
            |()| {
                WhatTheDesktopPublishes::<R>::the_condition_moved(
                    about.clone(),
                    WITHDRAWN,
                    raised_this_run
                        .as_ref()
                        .map(Identity::as_text)
                        .unwrap_or_default()
                        .as_str(),
                )
            },
        );
        // **M4: withdrawn by that change alone and never shown again** — on
        // every page that drew it, the window's foot included.
        if let (Ok(_), Some(server_identity)) = (posted, subject.identity()) {
            the_drawings_go(queue, server_identity, &went);
        }
    }

    /// **`Servers` Q6's third path, put to the user.**
    ///
    /// **Nothing durable happens here and nothing is counted.** The record is
    /// left exactly as it is until the user accepts, which is `Servers` Q6's
    /// own rule;
    /// and **how often the warning may be raised under a standing repair is
    /// `Servers` Q17's and is decided on that side**, so a second bound here
    /// would be one document's answer applied twice.
    fn the_changed_identity_warning(&mut self, server: &str) {
        let Some(identity) = self.the_subject_is_known(server) else {
            return;
        };
        let about = Subject::Durable(identity);
        let _ = self.queue.read_then_post::<(), Infallible>(
            || Ok(()),
            |()| Event::about(about.clone(), THE_CHANGED_IDENTITY_WARNING),
        );
    }

    /// **`Servers` D8: the one durable write of a value this seam carries.**
    ///
    /// It is one field of the server row, rewritable one entry at a time, and
    /// it is written on `Servers` Q6's first path — the scanned key recorded against a
    /// server that had nothing remembered — and again on an accept, where it
    /// replaces what was there.
    ///
    /// **`Servers` D9 bars every other destination: never shown, never logged,
    /// never handed to another module.** So **no event is posted from here at
    /// all**, which is also `Servers` Q6's first path in its own words: *"Nothing is asked
    /// and nothing is shown."* The value reaches the local record and stops.
    fn record_the_scanned_host_key(&mut self, server: &str, key: RememberedHostKey) {
        self.the_run.lend(&mut |lent| {
            let Ok(Some(mut row)) = server_rows::one(lent.record, server) else {
                return;
            };
            row.remembered_host_key = Some(key.clone());
            let _ = server_rows::write(lent.record, &row);
        });
    }
}

/// **The same seam on the publisher itself**, for a caller that owns one.
impl<R: LendsWhatTheRunHolds> WhatServersPublishes for WhatTheDesktopPublishes<R> {
    fn the_observed_value(&mut self, server: &str, value: ObservedValue) {
        (&*self).the_observed_value(server, value)
    }

    fn the_next_attempt_due(&mut self, server: &str, reading: NextAttemptDue) {
        (&*self).the_next_attempt_due(server, reading)
    }

    fn raise_the_connection_condition(&mut self, server: &str, says: WhatTheRecordSays) {
        (&*self).raise_the_connection_condition(server, says)
    }

    fn withdraw_the_connection_condition(&mut self, server: &str) {
        (&*self).withdraw_the_connection_condition(server)
    }

    fn the_changed_identity_warning(&mut self, server: &str) {
        (&*self).the_changed_identity_warning(server)
    }

    fn record_the_scanned_host_key(&mut self, server: &str, key: RememberedHostKey) {
        (&*self).record_the_scanned_host_key(server, key)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    /// **`Telegram` V1c's release rule**: at or above the first release that
    /// brings the request, or a desk-built program; older, or no program,
    /// takes nothing. The twenty-first notes, note 7.
    #[test]
    fn the_telegram_roles_newest_reading_lands_and_goes_with_the_connection() {
        let server = "server-g2a-reading";
        assert_eq!(the_telegram_role_on(server), None, "nothing read claims nothing");
        let older = a_reading_of_the_telegram_role_begins(server);
        let newer = a_reading_of_the_telegram_role_begins(server);
        assert!(a_reading_of_the_telegram_role_came_back(server, newer, Some(true)));
        assert!(!a_reading_of_the_telegram_role_came_back(server, older, Some(false)), "an older reading landed");
        assert_eq!(the_telegram_role_on(server), Some(true));
        assert!(!a_reading_of_the_telegram_role_came_back(server, newer, Some(true)), "it did not move");

        let on_its_way = a_reading_of_the_telegram_role_begins(server);
        the_telegram_install_failed_with(server, Some("It failed.".to_string()));
        the_telegram_role_goes_with_the_connection(server);
        assert_eq!(the_telegram_role_on(server), None);
        assert_eq!(what_the_telegram_install_failed_with(server), None);
        assert!(!a_reading_of_the_telegram_role_came_back(server, on_its_way, Some(false)));
        assert_eq!(the_telegram_role_on(server), None, "a reading from the old connection landed");
    }

    #[test]
    fn twenty_first_7_only_a_program_of_the_release_that_brings_it_takes_a_change() {
        use smoozy_server_protocol::requests::THE_FIRST_RELEASE_THAT_CHANGES_A_LIVE_JOBS_REPORTING as FIRST;
        assert_eq!(a_program_of_this_release_changes_a_live_jobs_reporting(Some(FIRST)), ALiveJobsReportingCanBeChanged::Yes);
        assert_eq!(
            a_program_of_this_release_changes_a_live_jobs_reporting(Some(FIRST - 1)),
            ALiveJobsReportingCanBeChanged::TheProgramIsOlder
        );
        assert_eq!(
            a_program_of_this_release_changes_a_live_jobs_reporting(None),
            ALiveJobsReportingCanBeChanged::TheProgramIsOlder
        );
        // **A desk program is a development build's alone** (`Server protocol
        // and transfers` B10a), so this line is compiled only where it is. In a
        // release build, nought — what a desk program is placed under — is an
        // older program like any other.
        #[cfg(debug_assertions)]
        assert_eq!(
            a_program_of_this_release_changes_a_live_jobs_reporting(Some(
                crate::the_desk_program::THE_SEQUENCE_A_DESK_PROGRAM_IS_PLACED_UNDER
            )),
            ALiveJobsReportingCanBeChanged::Yes
        );
        #[cfg(not(debug_assertions))]
        assert_eq!(
            a_program_of_this_release_changes_a_live_jobs_reporting(Some(0)),
            ALiveJobsReportingCanBeChanged::TheProgramIsOlder,
            "a release build took a program at nought as a desk program"
        );
        assert_eq!(FIRST, 1813, "0.1.8 went out at 1812 without it");
    }

    /// **`File Browser` L28: only a program of the release that first carries
    /// the count is offered it** — the same rule, its own constant. The
    /// twenty-third notes, note 2.
    #[test]
    fn twenty_third_2_only_a_program_of_the_release_that_brings_it_counts_a_folder() {
        use smoozy_server_protocol::requests::THE_FIRST_RELEASE_THAT_COUNTS_A_FOLDERS_SIZE as FIRST;
        assert!(a_program_of_this_release_counts_a_folders_size(Some(FIRST)));
        assert!(!a_program_of_this_release_counts_a_folders_size(Some(FIRST - 1)));
        assert!(!a_program_of_this_release_counts_a_folders_size(None));
        // **A development build's alone**, as above; a release build counts
        // nought as older.
        #[cfg(debug_assertions)]
        assert!(a_program_of_this_release_counts_a_folders_size(Some(
            crate::the_desk_program::THE_SEQUENCE_A_DESK_PROGRAM_IS_PLACED_UNDER
        )));
        #[cfg(not(debug_assertions))]
        assert!(
            !a_program_of_this_release_counts_a_folders_size(Some(0)),
            "a release build took a program at nought as a desk program"
        );
        assert_eq!(FIRST, 1813, "0.1.8 went out at 1812 without it, and nothing since");
        let server = "twenty-third-2-gate";
        assert_eq!(a_folders_size_can_be_counted_on(server), None);
        the_servers_release_is_read(server, Some(1812));
        assert_eq!(a_folders_size_can_be_counted_on(server), Some(false));
        the_servers_release_is_read(server, Some(FIRST));
        assert_eq!(a_folders_size_can_be_counted_on(server), Some(true));
    }

    use smoozy_server_protocol::transport::ssh::CommandFailure;
    use smoozy_server_protocol::transport::{
        CommandResult, CommandString, Credential, DecideHostTrust, Ending, Endpoint, HostTrust,
        OpenFailure,
    };
    use smoozy_servers::acting::{
        OpenConnections, SupplyTheCredential, TheActingHalf, TheIdentityRead,
        TheInstallationIdentityRead, TheServerBeingConnectedTo, Wait,
    };
    use smoozy_servers::first_connection::ChangedIdentityWarning;
    use smoozy_servers::hardware::FreeSpace;
    use smoozy_servers::identity::{Host, KeyReference, Port, Username};
    use smoozy_servers::key_moments::{TheTwoAnswers, WhoStartedIt};
    use smoozy_servers::readings::{CommandChannel, TakesAFreeSpaceReading};
    use smoozy_servers::repair::PermanentRefusal;
    use smoozy_servers::sequence::{self, AnAttempt, Attempt, ConnectionRole};
    use smoozy_servers::store::ServerRow;
    use smoozy_state::identities::InstallationCheck;

    use crate::events::{Delivers, Delivery};
    use crate::notifications::permission::WhatTheSystemSays;
    use crate::notifications::{APost, TheMachinery, ThePlatform};
    use crate::presentation::the_presentation_key::ThePresentationKey;
    use crate::record::settings::TheSwitches;
    use crate::record::LocalRecord;

    use super::*;

    // ------------------------------------------------------------- the desk

    /// A record of this test's own, in a name no second run of these tests can
    /// meet.
    fn a_record(name: &str) -> LocalRecord {
        let path = crate::scratch_folders::a_file(format!("smoozy-servers-published-{name}-{}.sqlite3", std::process::id()));
        LocalRecord::open_at(&path).expect("the desk can make a record")
    }

    fn a_row(identifier: &str) -> ServerRow {
        ServerRow {
            local_server_identifier: identifier.to_string(),
            host: Host::what_the_user_typed("198.51.100.10").expect("a host"),
            port: Port::STANDARD_SSH,
            username: Username::what_the_user_typed("root").expect("a username"),
            key: KeyReference {
                local_key_identifier: "key-1".into(),
                fingerprint: "SHA256:aaa".into(),
            },
            label: Some("Old 4080".into()),
            colour: None,
            remembered_setup: None,
            remembered_host_key: None,
            remembered_hardware_reading: None,
            recorded_machine_root: None,
            last_confirmed_installation_identity: None,
            event_cursor: None,
            machine_password_subject: None,
            chosen_fallback_folder: None,
            the_driver_missed_the_optix_bar: false,
        }
    }

    /// **The run, at the desk**: one open record, the run-held register, and a
    /// declaration register that carries what this module declares.
    struct TheRunAtTheDesk {
        record: LocalRecord,
        said_once: TheThingsSaidOnce,
        declarations: TheDeclarations,
    }

    impl TheRunAtTheDesk {
        fn holding(record: LocalRecord) -> Self {
            let mut declarations = TheDeclarations::new();
            declarations.module_declares(THE_MODULE, what_servers_declares());
            declarations.module_declares(
                THE_SUPERVISORS_SUPPLIER,
                what_the_supervisors_supplier_declares(),
            );
            TheRunAtTheDesk {
                record,
                said_once: TheThingsSaidOnce::new(),
                declarations,
            }
        }

        /// The same run with nothing declared, which is A7's hosting fault.
        fn declaring_nothing(record: LocalRecord) -> Self {
            TheRunAtTheDesk {
                record,
                said_once: TheThingsSaidOnce::new(),
                declarations: TheDeclarations::new(),
            }
        }
    }

    /// **A test that leaves its record behind is litter**, and the system
    /// temporary folder already carries thousands of Smoozy's. The file goes
    /// when the desk does, with the engine's own two side files.
    impl Drop for TheRunAtTheDesk {
        fn drop(&mut self) {
            let path = self.record.path().to_path_buf();
            for beside in ["", "-wal", "-shm"] {
                let mut name = path.clone().into_os_string();
                name.push(beside);
                let _ = std::fs::remove_file(std::path::PathBuf::from(name));
            }
        }
    }

    impl LendsWhatTheRunHolds for TheRunAtTheDesk {
        fn lend(&self, act: &mut dyn FnMut(WhatTheRunLends<'_>)) {
            act(WhatTheRunLends {
                record: self.record.connection(),
                said_once: &self.said_once,
                declarations: &self.declarations,
            });
        }
    }

    /// A run that lends nothing, which is a record that would not open.
    struct ARunThatLendsNothing;

    impl LendsWhatTheRunHolds for ARunThatLendsNothing {
        fn lend(&self, _act: &mut dyn FnMut(WhatTheRunLends<'_>)) {}
    }

    #[derive(Default)]
    struct WhatThePageGot(Arc<Mutex<Vec<Delivery>>>);

    impl Delivers for WhatThePageGot {
        fn deliver(&self, delivery: &Delivery) {
            self.0
                .lock()
                .expect("the deliveries")
                .push(delivery.clone());
        }
    }

    /// Every post the operating system was asked for, and every take-down.
    #[derive(Default)]
    struct WhatTheSystemWasAsked {
        posts: Arc<Mutex<Vec<String>>>,
        taken_down: Arc<Mutex<Vec<String>>>,
    }

    impl ThePlatform for WhatTheSystemWasAsked {
        fn the_machinery(&self) -> TheMachinery {
            TheMachinery::ItIsInPlace
        }
        fn the_permission(&self) -> WhatTheSystemSays {
            WhatTheSystemSays::ItIsGranted
        }
        fn ask_for_permission_now(&self) {}
        fn post(&self, post: &APost) {
            self.posts.lock().expect("the posts").push(post.headline());
        }
        fn take_down(&self, key: &ThePresentationKey) {
            self.taken_down
                .lock()
                .expect("the take-downs")
                .push(key.as_text());
        }
    }

    /// One publisher over one record, with everything it reaches held so the
    /// test can read it back.
    struct ADesk {
        publishes: WhatTheDesktopPublishes<TheRunAtTheDesk>,
        page: Arc<Mutex<Vec<Delivery>>>,
        posts: Arc<Mutex<Vec<String>>>,
        taken_down: Arc<Mutex<Vec<String>>>,
        server: String,
    }

    fn a_desk(name: &str) -> ADesk {
        a_desk_over(name, true)
    }

    fn a_desk_over(name: &str, declared: bool) -> ADesk {
        let record = a_record(name);
        let identity = Identity::issue(IdentityKind::LocalServer).expect("randomness");
        let server = identity.as_text();
        server_rows::add(record.connection(), &a_row(&server)).expect("the row goes down");

        let queue = Arc::new(TheOneQueue::new());
        let page = Arc::new(Mutex::new(Vec::new()));
        queue.the_page_is_ready(Box::new(WhatThePageGot(page.clone())));

        let asked = WhatTheSystemWasAsked::default();
        let posts = asked.posts.clone();
        let taken_down = asked.taken_down.clone();
        let notifications = Arc::new(crate::notifications::TheNotificationService::on(Box::new(
            asked,
        )));
        // **T8 and T17a: a kind the user switched off is not attempted**, and a
        // record that has not answered attempts none — so the switches are
        // handed over here or nothing below would be posted at all.
        notifications.the_switches_are(Some(TheSwitches::nobody_has_turned_one()));

        let run = if declared {
            TheRunAtTheDesk::holding(record)
        } else {
            TheRunAtTheDesk::declaring_nothing(record)
        };
        ADesk {
            publishes: WhatTheDesktopPublishes::new(run, queue, notifications),
            page,
            posts,
            taken_down,
            server,
        }
    }

    impl ADesk {
        fn deliveries(&self) -> Vec<Delivery> {
            self.page.lock().expect("the deliveries").clone()
        }

        fn of_kind(&self, kind: &str) -> Vec<Delivery> {
            self.deliveries()
                .into_iter()
                .filter(|delivery| delivery.kind == kind)
                .collect()
        }

        fn said(delivery: &Delivery, name: &str) -> String {
            match delivery.said.get(name) {
                Some(Said::Value(value)) => value.as_str().to_string(),
                Some(Said::Prose(prose)) => prose.as_str().to_string(),
                None => panic!("the event says nothing called {name}"),
            }
        }

        /// The whole durable record, as text, so that *nothing was written* is
        /// an assertion over every table rather than over the ones a test
        /// thought to look at.
        fn the_whole_record(&self) -> Vec<String> {
            let mut out = Vec::new();
            self.publishes.the_run.lend(&mut |lent| {
                let mut names = Vec::new();
                {
                    let mut asking = lent
                        .record
                        .prepare(
                            "SELECT name FROM sqlite_master \
                             WHERE type = 'table' AND name NOT LIKE 'sqlite_%' ORDER BY name",
                        )
                        .expect("the tables");
                    let rows = asking
                        .query_map([], |row| row.get::<_, String>(0))
                        .expect("the tables read");
                    for row in rows {
                        names.push(row.expect("a table name"));
                    }
                }
                for table in names {
                    let mut columns = Vec::new();
                    {
                        let mut asking = lent
                            .record
                            .prepare(&format!("PRAGMA table_info(\"{table}\")"))
                            .expect("the columns");
                        let rows = asking
                            .query_map([], |row| row.get::<_, String>(1))
                            .expect("the columns read");
                        for row in rows {
                            columns.push(row.expect("a column name"));
                        }
                    }
                    let quoted = columns
                        .iter()
                        .map(|column| format!("quote(\"{column}\")"))
                        .collect::<Vec<String>>()
                        .join(" || '\u{1f}' || ");
                    let mut asking = lent
                        .record
                        .prepare(&format!(
                            "SELECT {quoted} AS line FROM \"{table}\" ORDER BY line"
                        ))
                        .expect("the rows");
                    let rows = asking
                        .query_map([], |row| row.get::<_, String>(0))
                        .expect("the rows read");
                    for row in rows {
                        out.push(format!("{table}: {}", row.expect("a row")));
                    }
                }
            });
            out
        }
    }

    // ------------------------------------------------- the two run-held ones

    /// **`Servers` K6's three, as they move, and each carries its own moment
    /// and clock** — §16.1 M3a.
    #[test]
    fn every_one_of_the_three_observed_values_reaches_the_page() {
        let mut desk = a_desk("observed-values");
        let server = desk.server.clone();
        for value in ObservedValue::ALL {
            desk.publishes.the_observed_value(&server, value);
        }
        let seen = desk.of_kind(WHAT_IS_OBSERVED);
        assert_eq!(seen.len(), 3);
        for (delivery, value) in seen.iter().zip(ObservedValue::ALL) {
            assert_eq!(
                ADesk::said(delivery, "theConditionsValue"),
                the_written_form(value)
            );
            assert_eq!(ADesk::said(delivery, "onTheClock"), "this computer");
            assert!(!ADesk::said(delivery, "takenAt").is_empty());
        }
    }

    /// **`Servers` N10a: absent is a value rather than a gap**, and the event
    /// carries the field either way.
    #[test]
    fn the_moment_the_next_attempt_is_due_is_carried_and_so_is_its_absence() {
        let mut desk = a_desk("next-attempt");
        let server = desk.server.clone();
        let due = SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_112);
        desk.publishes
            .the_next_attempt_due(&server, NextAttemptDue::At(due));
        desk.publishes
            .the_next_attempt_due(&server, NextAttemptDue::Absent);
        let seen = desk.of_kind(WHAT_IS_OBSERVED);
        assert_eq!(ADesk::said(&seen[0], "theNextAttemptIsDue"), "1700000112");
        assert_eq!(ADesk::said(&seen[1], "theNextAttemptIsDue"), ABSENT);
    }

    /// **`Servers` D22 and N10a: a run knows *disconnected* and *absent* about
    /// every server until something says otherwise**, and neither comes from
    /// anything durable.
    #[test]
    fn a_run_starts_every_server_disconnected_with_no_moment() {
        let mut desk = a_desk("afresh");
        let server = desk.server.clone();
        desk.publishes
            .the_observed_value(&server, ObservedValue::Disconnected);
        let seen = desk.of_kind(WHAT_IS_OBSERVED);
        assert_eq!(ADesk::said(&seen[0], "theConditionsValue"), "disconnected");
        assert_eq!(ADesk::said(&seen[0], "theNextAttemptIsDue"), ABSENT);
    }

    /// **The half a later agent breaks without noticing: the run-held readings
    /// reach no durable store at all.**
    ///
    /// `Servers` S3, `Servers` N10a's *run-held*, `Queue & History` C6 and
    /// `State, identity and recovery` J2 all say the same thing about these two
    /// values, and none of them is enforced by a type. **So it is asserted over
    /// the whole record**, every table, before and after — a column added for
    /// one of them later fails here rather than passing quietly.
    #[test]
    fn the_run_held_readings_reach_no_durable_store() {
        let mut desk = a_desk("nothing-durable");
        let server = desk.server.clone();
        let before = desk.the_whole_record();
        for value in ObservedValue::ALL {
            desk.publishes.the_observed_value(&server, value);
            desk.publishes.the_next_attempt_due(
                &server,
                NextAttemptDue::At(SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_112)),
            );
            desk.publishes
                .the_next_attempt_due(&server, NextAttemptDue::Absent);
        }
        let after = desk.the_whole_record();
        assert_eq!(
            before, after,
            "a run-held reading reached the local record. **`Servers` N10a: \
             run-held, beside the condition record and never inside it**, and \
             **`Servers` S3**: written nowhere durable"
        );
        // And they are held, so the holding is real rather than a claim.
        assert_eq!(
            desk.publishes.what_is_held_about(&server),
            Some((ObservedValue::Disconnected, NextAttemptDue::Absent))
        );
    }

    // --------------------------------------------------- raising and withdrawing

    /// **`Servers` N5 and `Desktop app and delivery` T13 in one test**: the
    /// record goes down, the operating system is told from that record, and the
    /// page is told.
    #[test]
    fn a_raise_writes_the_record_tells_the_system_and_tells_the_page() {
        let mut desk = a_desk("raise");
        let server = desk.server.clone();
        desk.publishes
            .raise_the_connection_condition(&server, WhatTheRecordSays::raised());

        let seen = desk.of_kind(THE_CONNECTION_CONDITION);
        assert_eq!(seen.len(), 1);
        assert_eq!(ADesk::said(&seen[0], "whatHappened"), RAISED);
        let condition = ADesk::said(&seen[0], "condition");
        assert!(Identity::read_back(IdentityKind::Condition, &condition).is_some());

        // **T13: the post names the server and says the connection is down**,
        // and Design's own words are the notification's.
        assert_eq!(
            desk.posts.lock().expect("the posts").as_slice(),
            ["Old 4080 disconnected"]
        );

        // The record holds it, over both of `Servers` K7's values and no other.
        desk.publishes.the_run.lend(&mut |lent| {
            let subject = DurableSubject::ServerRow(
                Identity::read_back(IdentityKind::LocalServer, &server).expect("an identity"),
            );
            let standing = FailureRecords::over(lent.record)
                .standing_against(&subject)
                .expect("the records read");
            assert_eq!(standing.len(), 1);
            let mut values = standing[0].the_values_it_was_raised_over.clone();
            values.sort();
            assert_eq!(values, vec!["disconnected", "reconnecting"]);
            assert!(standing[0].still_holds_at("reconnecting"));
            assert!(!standing[0].still_holds_at("connected"));
        });
    }

    /// **§14 Q13: a repair that fails run after run raises one record and the
    /// user is told once.**
    #[test]
    fn a_second_raise_is_the_same_record_and_the_system_is_told_once() {
        let mut desk = a_desk("raised-twice");
        let server = desk.server.clone();
        for _ in 0..4 {
            desk.publishes
                .raise_the_connection_condition(&server, WhatTheRecordSays::raised());
        }
        let seen = desk.of_kind(THE_CONNECTION_CONDITION);
        let first = ADesk::said(&seen[0], "condition");
        for delivery in &seen {
            assert_eq!(ADesk::said(delivery, "condition"), first);
        }
        assert_eq!(
            desk.posts.lock().expect("the posts").len(),
            1,
            "**§14 Q13: a later failure resolving to a standing identity needs \
             no post at all, and the repeat's correct treatment on the \
             notification surface is silence**"
        );
    }

    /// **`Servers` N10: the record is withdrawn by the observed value reaching
    /// *connected*, and the post comes down with it** (T24a).
    #[test]
    fn a_withdrawal_takes_the_record_and_the_post_with_it() {
        let mut desk = a_desk("withdraw");
        let server = desk.server.clone();
        desk.publishes
            .raise_the_connection_condition(&server, WhatTheRecordSays::raised());
        let raised = ADesk::said(&desk.of_kind(THE_CONNECTION_CONDITION)[0], "condition");
        desk.publishes.withdraw_the_connection_condition(&server);

        let seen = desk.of_kind(THE_CONNECTION_CONDITION);
        assert_eq!(seen.len(), 2);
        assert_eq!(ADesk::said(&seen[1], "whatHappened"), WITHDRAWN);
        assert_eq!(ADesk::said(&seen[1], "condition"), raised);
        assert_eq!(
            desk.taken_down.lock().expect("the take-downs").as_slice(),
            [format!("condition:{raised}")]
        );
        desk.publishes.the_run.lend(&mut |lent| {
            let subject = DurableSubject::ServerRow(
                Identity::read_back(IdentityKind::LocalServer, &server).expect("an identity"),
            );
            assert!(FailureRecords::over(lent.record)
                .standing_against(&subject)
                .expect("the records read")
                .is_empty());
        });
    }

    /// **Step 10b's first audit round, item 8: `State, identity and recovery`
    /// Y9's fourth branch raises §8.3's supervisor-unavailable condition**,
    /// under the supplier that row names, **and the supervisor answering
    /// withdraws it.** A re-take that meets the branch again is the same
    /// record (§8.3 F11), and nothing is told to the operating system — no
    /// clause gives this condition a notification.
    #[test]
    fn the_fourth_branch_raises_the_supervisors_condition_and_its_answer_withdraws_it() {
        let desk = a_desk("supervisor");
        let server = desk.server.clone();
        let standing = |desk: &ADesk| {
            let mut found = Vec::new();
            desk.publishes.the_run.lend(&mut |lent| {
                let subject = DurableSubject::ServerRow(
                    Identity::read_back(IdentityKind::LocalServer, &server).expect("an identity"),
                );
                found = FailureRecords::over(lent.record)
                    .standing_against(&subject)
                    .expect("the records read");
            });
            found
        };

        desk.publishes.the_supervisor_is_unavailable(&server);
        desk.publishes.the_supervisor_is_unavailable(&server);
        let raised = standing(&desk);
        assert_eq!(raised.len(), 1, "one record, however often the branch is met");
        assert!(raised[0].still_holds_at("the supervisor not answering"));
        assert!(!raised[0].still_holds_at("the supervisor answering"));
        assert_eq!(desk.of_kind(THE_CONNECTION_CONDITION).len(), 2);
        assert!(desk.posts.lock().expect("the posts").is_empty());

        desk.publishes.the_supervisor_answered(&server);
        assert!(standing(&desk).is_empty(), "the supervisor answering withdraws it");

        // **M4: the drawing goes with the record**, on the failure shape's own
        // clearing and on the key a condition is drawn on — or *Smoozy isn't
        // responding.* stays on the window's foot with nothing behind it.
        // Step 10c's twelfth-pass fix piece 7.
        let cleared = desk.of_kind(crate::surface::the_failure_shape::A_FAILURE_WAS_CLEARED);
        assert_eq!(cleared.len(), 1, "the supervisor answering told no page its drawing goes");
        assert_eq!(
            ADesk::said(&cleared[0], "key"),
            format!("condition:{}", raised[0].identity_or_mark)
        );
    }

    /// **The supervisor's condition is refused where its supplier is not
    /// declared** — `State, identity and recovery` F12.
    #[test]
    fn the_supervisors_condition_is_not_raised_undeclared() {
        let desk = a_desk_over("supervisor-undeclared", false);
        let server = desk.server.clone();
        desk.publishes.the_supervisor_is_unavailable(&server);
        assert!(desk.of_kind(THE_CONNECTION_CONDITION).is_empty());
    }

    /// **A withdrawal with nothing standing writes nothing and posts nothing.**
    /// `State, identity and recovery` F7: a withdrawn record is spent, and the
    /// value returning does not bring it back.
    #[test]
    fn withdrawing_nothing_says_nothing() {
        let mut desk = a_desk("withdraw-nothing");
        let server = desk.server.clone();
        desk.publishes.withdraw_the_connection_condition(&server);
        assert!(desk.of_kind(THE_CONNECTION_CONDITION).is_empty());
        assert!(desk.taken_down.lock().expect("the take-downs").is_empty());
    }

    // ------------------------------------- the window's foot, as a person meets it

    /// A start report holding the record and the declarations this file's
    /// raises are checked against — **the application has no other
    /// constructor for one**, so it is written here, as
    /// `src/surface/the_failure_shape.rs`'s own tests write theirs.
    fn a_started_over(record: LocalRecord) -> Arc<Mutex<Started>> {
        let mut what_modules_declare = TheDeclarations::new();
        what_modules_declare.module_declares(THE_MODULE, what_servers_declares());
        what_modules_declare.module_declares(
            THE_SUPERVISORS_SUPPLIER,
            what_the_supervisors_supplier_declares(),
        );
        Arc::new(Mutex::new(Started {
            the_sweep_left_behind: Vec::new(),
            record: Some(record),
            record_trouble: None,
            diary: None,
            declarations: smoozy_reporting::lines::Declarations::new(),
            what_modules_declare,
            said_once: TheThingsSaidOnce::new(),
            run: None,
            the_previous_run: smoozy_state::run_mark::WhatTheLaunchFound::NoPreviousRun,

        }))
    }

    /// **The real page's foot, fed the real deliveries in their one order**:
    /// `web/event-route.js` and `web/the-failure-shape.js` as they ship, over a
    /// small stand-in for the document. Answers, per step, the sentences the
    /// foot (`#the-failures`) holds — or nothing where this computer has no
    /// Node, which the caller says rather than passes over.
    fn what_the_foot_holds(steps: &[(&str, Vec<Delivery>)]) -> Option<Vec<String>> {
        let mut folder = std::env::temp_dir();
        folder.push(format!("smoozy-the-foot-{}-{}", std::process::id(), line!()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a folder for the page");
        let web = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for page in ["event-route.js", "the-failure-shape.js", "the-value-drawn.js"] {
            std::fs::copy(web.join(page), folder.join(page)).expect("the page file is copied");
        }
        let steps: Vec<serde_json::Value> = steps
            .iter()
            .map(|(name, deliveries)| serde_json::json!({ "step": name, "deliveries": deliveries }))
            .collect();
        std::fs::write(
            folder.join("steps.json"),
            serde_json::to_string(&steps).expect("the deliveries serialise"),
        )
        .expect("the steps are written");
        std::fs::write(folder.join("harness.mjs"), THE_FOOTS_HARNESS).expect("the harness");
        let ran = std::process::Command::new("node")
            .arg(folder.join("harness.mjs"))
            .current_dir(&folder)
            .output();
        let _ = std::fs::remove_dir_all(&folder);
        let ran = ran.ok()?;
        assert!(
            ran.status.success(),
            "the page did not run: {}",
            String::from_utf8_lossy(&ran.stderr)
        );
        Some(
            String::from_utf8_lossy(&ran.stdout)
                .lines()
                .map(str::to_string)
                .collect(),
        )
    }

    const THE_FOOTS_HARNESS: &str = r#"
import { readFileSync } from "node:fs";
let byId = new Map();
function anElement(tag) {
  const element = {
    tag, children: [], parentNode: null, dataset: {}, className: "", text: "",
    set id(value) { byId.set(value, element); element._id = value; },
    get id() { return element._id; },
    set textContent(value) { element.children = []; element.text = String(value); },
    get textContent() { return element.text + element.children.map((one) => one.textContent || "").join(""); },
    get childElementCount() { return element.children.length; },
    appendChild(child) { child.parentNode = element; element.children.push(child); return child; },
    remove() {
      if (!element.parentNode) return;
      element.parentNode.children = element.parentNode.children.filter((one) => one !== element);
      element.parentNode = null;
    },
    replaceWith(fresh) {
      const parent = element.parentNode;
      if (!parent) return;
      parent.children = parent.children.map((one) => (one === element ? fresh : one));
      fresh.parentNode = parent;
      element.parentNode = null;
    },
    setAttribute() {}, addEventListener() {}, querySelector() { return null; },
  };
  return element;
}
const pane = anElement("div");
pane.id = "pane";
globalThis.document = {
  createElement: anElement,
  createTextNode: (text) => ({ text, children: [], textContent: text }),
  getElementById: (id) => {
    const found = byId.get(id);
    return found && (found === pane || attached(found)) ? found : null;
  },
  documentElement: { dataset: {} },
};
function attached(element) {
  for (let at = element; at; at = at.parentNode) if (at === pane) return true;
  return false;
}
let receive = null;
globalThis.window = {
  __TAURI__: {
    event: { listen: async (_name, handler) => { receive = handler; } },
    core: { invoke: () => Promise.resolve(null) },
  },
};
const route = await import("./event-route.js");
await import("./the-failure-shape.js");
await route.theRouteIsOpen();
function sentences(element, out) {
  if (element.className && String(element.className).split(" ").includes("failure-said")) {
    out.push(element.textContent);
    return out;
  }
  for (const child of element.children || []) sentences(child, out);
  return out;
}
for (const { step, deliveries } of JSON.parse(readFileSync("steps.json", "utf8"))) {
  for (const delivery of deliveries) receive({ payload: delivery });
  const foot = document.getElementById("the-failures");
  const said = foot ? sentences(foot, []) : [];
  console.log(step + ": " + said.length + (said.length ? " — " + said.join(" | ") : ""));
}
"#;

    /// **T24b: a removal takes down the posts raised from the condition
    /// records it took**, on the identity each was posted on — the third
    /// finding of step 12's destroy scenario, ruled 2026-10-24. The forget
    /// deleted the record with the row and nothing took its *could not be
    /// restored* down.
    #[test]
    fn a_removal_takes_down_the_posts_raised_from_the_records_it_took() {
        let started = a_started_over(a_record("the-removal-takes-it-down"));
        let queue = Arc::new(TheOneQueue::new());
        let asked = WhatTheSystemWasAsked::default();
        let posts = asked.posts.clone();
        let taken_down = asked.taken_down.clone();
        let notifications = Arc::new(crate::notifications::TheNotificationService::on(Box::new(asked)));
        notifications.the_switches_are(Some(TheSwitches::nobody_has_turned_one()));
        let publishes = WhatTheDesktopPublishes::new(
            crate::servers::a_period::TheRunThroughTheApp::lending_the_record_of(started.clone()),
            queue.clone(),
            notifications.clone(),
        );
        let server = Identity::issue(IdentityKind::LocalServer).expect("randomness").as_text();
        {
            let held = started.lock().expect("the start");
            let record = held.record.as_ref().expect("a record");
            server_rows::add(record.connection(), &a_row(&server)).expect("the row goes down");
        }

        // **The repair run fails** (`Servers` N5) and T13 posts from the raise.
        (&publishes).raise_the_connection_condition(&server, WhatTheRecordSays::raised());
        assert_eq!(posts.lock().expect("the posts").len(), 1, "the could-not-restore notification");
        let conditions = what_the_forget_will_take(&started.lock().expect("the start"), &server);
        assert_eq!(conditions.len(), 1);

        let answer = crate::surface::servers::the_removal_is_performed_and_its_drawings_go(
            &started.lock().expect("the start"),
            None,
            Some(&queue),
            Some(&notifications),
            &server,
            false,
        );
        assert_eq!(answer, crate::surface::servers::TheAnswer::ItWasSaved);
        assert_eq!(
            *taken_down.lock().expect("the take-downs"),
            vec![ThePresentationKey::ACondition(conditions[0].clone()).as_text()],
            "the post comes down on the identity it was posted on"
        );
    }

    /// **The twelfth pass's fault 13, in the order the pass met it** —
    /// *Something is not working. — Report a problem…* at the window's foot
    /// with no failure record in the local record.
    ///
    /// Every durable act is the application's own: the repair run's failure
    /// raising the connection condition, the line coming back withdrawing it,
    /// the list's own ask of what stands against each row, the removal the
    /// press performs (the same forget *Stop and clear first* ends with), and a
    /// new row for the same machine. **The foot is the real page**, fed every
    /// delivery the queue made in its one order. After each step the foot and
    /// the failure store are read together, and **they must agree**:
    /// `Desktop app and delivery` M4 shows a condition record *"while, and only
    /// while"* it stands, and `State, identity and recovery` F25 ends it at a
    /// withdrawal and at its subject's removal.
    #[test]
    fn the_foot_says_only_what_the_record_holds_through_a_cut_a_removal_and_a_re_add() {
        let started = a_started_over(a_record("the-foot"));
        let queue = Arc::new(TheOneQueue::new());
        let page = Arc::new(Mutex::new(Vec::new()));
        queue.the_page_is_ready(Box::new(WhatThePageGot(page.clone())));
        let notifications = Arc::new(crate::notifications::TheNotificationService::on(Box::new(
            WhatTheSystemWasAsked::default(),
        )));
        notifications.the_switches_are(Some(TheSwitches::nobody_has_turned_one()));
        let publishes = WhatTheDesktopPublishes::new(
            crate::servers::a_period::TheRunThroughTheApp::lending_the_record_of(started.clone()),
            queue.clone(),
            notifications,
        );

        let a_new_row = || {
            let server = Identity::issue(IdentityKind::LocalServer).expect("randomness").as_text();
            let held = started.lock().expect("the start");
            let record = held.record.as_ref().expect("a record");
            server_rows::add(record.connection(), &a_row(&server)).expect("the row goes down");
            server
        };
        let subject_of = |server: &str| {
            DurableSubject::ServerRow(
                Identity::read_back(IdentityKind::LocalServer, server).expect("an identity"),
            )
        };
        // **The list's own ask**, which `web/servers.js` makes for every row
        // each time it is fetched — and it is fetched on every connection
        // change, whether or not the list is the screen in front.
        let the_list_asks = |rows: &[&str]| {
            let mut held = started.lock().expect("the start");
            for server in rows {
                crate::surface::the_failure_shape::the_failures_are_posted(
                    &mut held,
                    &queue,
                    &subject_of(server),
                );
            }
        };
        let the_store_holds = |server: &str| {
            let held = started.lock().expect("the start");
            let record = held.record.as_ref().expect("a record");
            FailureRecords::over(record.connection())
                .standing_against(&subject_of(server))
                .expect("the records read")
                .len()
        };
        let mut steps: Vec<(&str, Vec<Delivery>)> = Vec::new();
        let mut stored: Vec<String> = Vec::new();
        let mut a_step = |name: &'static str, held: usize, steps: &mut Vec<(&str, Vec<Delivery>)>| {
            let delivered = std::mem::take(&mut *page.lock().expect("the deliveries"));
            steps.push((name, delivered));
            stored.push(format!("{name}: {held}"));
        };

        let server = a_new_row();
        the_list_asks(&[&server]);
        a_step("the row is added", the_store_holds(&server), &mut steps);

        // **The line is cut long enough for a repair run to fail** (`Servers`
        // N5), and the list is asked again, as a connection change makes it.
        (&publishes).raise_the_connection_condition(&server, WhatTheRecordSays::raised());
        the_list_asks(&[&server]);
        a_step("the line is cut", the_store_holds(&server), &mut steps);

        // **The line is back** (`Servers` N10).
        (&publishes).withdraw_the_connection_condition(&server);
        the_list_asks(&[&server]);
        a_step("the line is back", the_store_holds(&server), &mut steps);

        // **Cut again, and the server removed while the condition stands** —
        // the thirteenth pass's *Remove while the line is cut*.
        (&publishes).raise_the_connection_condition(&server, WhatTheRecordSays::raised());
        the_list_asks(&[&server]);
        a_step("the line is cut again", the_store_holds(&server), &mut steps);

        // **The removal as the press performs it** — the same function both
        // removals end in, *Stop and clear first*'s third unit among them.
        let answer = crate::surface::servers::the_removal_is_performed_and_its_drawings_go(
            &started.lock().expect("the start"),
            None,
            Some(&queue),
            None,
            &server,
            false,
        );
        assert_eq!(answer, crate::surface::servers::TheAnswer::ItWasSaved);
        the_list_asks(&[]);
        a_step("the server is removed", the_store_holds(&server), &mut steps);

        // **The same machine added again**, as a new row.
        let again = a_new_row();
        the_list_asks(&[&again]);
        a_step("the machine is added again", the_store_holds(&again), &mut steps);

        let Some(foot) = what_the_foot_holds(&steps) else {
            eprintln!("SKIPPED: no Node on this computer, so the window's foot was not read");
            return;
        };
        let foot_counts: Vec<String> = foot
            .iter()
            .map(|line| line.split(" — ").next().unwrap_or_default().to_string())
            .collect();
        assert_eq!(
            foot_counts, stored,
            "**the foot and the failure store disagree** — the foot read:\n{}",
            foot.join("\n")
        );
    }

    /// **`State, identity and recovery` F12, and `Desktop app and delivery`
    /// A7's hosting rule from the other side: a module this register does not
    /// carry raises nothing**, and the failure is silent on the durable side
    /// rather than guessed past.
    #[test]
    fn a_condition_this_run_does_not_declare_is_not_raised() {
        let mut desk = a_desk_over("undeclared", false);
        let server = desk.server.clone();
        desk.publishes
            .raise_the_connection_condition(&server, WhatTheRecordSays::raised());
        assert!(
            desk.of_kind(THE_CONNECTION_CONDITION).is_empty(),
            "**P7: a durable act that failed posts nothing**"
        );
        assert!(desk.posts.lock().expect("the posts").is_empty());
    }

    /// **P7, over this file's own doors: a run that lends nothing writes
    /// nothing and posts nothing.**
    #[test]
    fn a_run_that_lends_nothing_posts_nothing() {
        let queue = Arc::new(TheOneQueue::new());
        let page = Arc::new(Mutex::new(Vec::new()));
        queue.the_page_is_ready(Box::new(WhatThePageGot(page.clone())));
        let notifications = Arc::new(crate::notifications::TheNotificationService::on(Box::new(
            WhatTheSystemWasAsked::default(),
        )));
        let mut publishes =
            WhatTheDesktopPublishes::new(ARunThatLendsNothing, queue, notifications);
        let server = Identity::issue(IdentityKind::LocalServer)
            .expect("randomness")
            .as_text();
        publishes.the_observed_value(&server, ObservedValue::Connected);
        publishes.raise_the_connection_condition(&server, WhatTheRecordSays::raised());
        publishes.the_changed_identity_warning(&server);
        publishes.record_the_scanned_host_key(&server, RememberedHostKey::approved(b"k".to_vec()));
        assert!(page.lock().expect("the deliveries").is_empty());
    }

    /// **P8: an event is never queued about a durable subject that does not
    /// exist**, and a server row that is not in the record is exactly that.
    #[test]
    fn nothing_is_posted_about_a_server_the_record_does_not_hold() {
        let mut desk = a_desk("no-such-row");
        let missing = Identity::issue(IdentityKind::LocalServer)
            .expect("randomness")
            .as_text();
        desk.publishes
            .the_observed_value(&missing, ObservedValue::Connected);
        assert!(desk.deliveries().is_empty());
    }

    // ------------------------------------------------- the changed identity

    /// **`Servers` Q6's third path reaches the page and writes nothing.**
    #[test]
    fn the_changed_identity_warning_is_an_event_and_changes_no_record() {
        let mut desk = a_desk("changed-identity");
        let server = desk.server.clone();
        let before = desk.the_whole_record();
        desk.publishes.the_changed_identity_warning(&server);
        assert_eq!(desk.of_kind(THE_CHANGED_IDENTITY_WARNING).len(), 1);
        assert_eq!(
            before,
            desk.the_whole_record(),
            "**`Servers` Q6: declining keeps the old record**, so raising the \
             question changes nothing durable"
        );
    }

    /// **`Servers` Q17 is not applied twice.** The bound on how often the
    /// warning is raised under a standing repair is that clause's and is kept
    /// on `Servers`' side; this file publishes every raise it is handed.
    #[test]
    fn nothing_here_bounds_how_often_the_warning_is_raised() {
        let mut desk = a_desk("q17");
        let server = desk.server.clone();
        for _ in 0..3 {
            desk.publishes.the_changed_identity_warning(&server);
        }
        assert_eq!(desk.of_kind(THE_CHANGED_IDENTITY_WARNING).len(), 3);
    }

    // ------------------------------------------------------- D8 and D9

    /// **`Servers` D8: the scanned key is recorded against the server row**, and
    /// an accept replaces what was there.
    #[test]
    fn the_scanned_host_key_is_written_and_a_later_one_replaces_it() {
        let mut desk = a_desk("host-key");
        let server = desk.server.clone();
        desk.publishes
            .record_the_scanned_host_key(&server, RememberedHostKey::approved(b"first".to_vec()));
        desk.publishes
            .record_the_scanned_host_key(&server, RememberedHostKey::approved(b"second".to_vec()));
        desk.publishes.the_run.lend(&mut |lent| {
            let row = server_rows::one(lent.record, &server)
                .expect("the row reads")
                .expect("the row is there");
            let held = row.remembered_host_key.expect("a key is remembered");
            assert!(held.matches(b"second"));
            assert!(!held.matches(b"first"));
        });
    }

    /// **`Servers` D9: never shown, never logged, never handed to another
    /// module** — so the write posts nothing, and the value is in no event and
    /// in no notification.
    #[test]
    fn the_host_key_reaches_the_record_and_no_other_destination() {
        let mut desk = a_desk("d9");
        let server = desk.server.clone();
        desk.publishes.record_the_scanned_host_key(
            &server,
            RememberedHostKey::approved(b"ssh-ed25519 AAAAsecret".to_vec()),
        );
        assert!(
            desk.deliveries().is_empty(),
            "**`Servers` Q6's first path: nothing is asked and nothing is \
             shown**, and **D9** bars the value from every destination but the \
             record"
        );
        let said = format!("{:?}", desk.deliveries());
        assert!(!said.contains("AAAAsecret"), "{said}");
        assert!(desk.posts.lock().expect("the posts").is_empty());
        // **D9's *never logged*, kept by the type**: the value has no rendering.
        let rendered = format!(
            "{:?}",
            RememberedHostKey::approved(b"ssh-ed25519 AAAAsecret".to_vec())
        );
        assert!(!rendered.contains("AAAAsecret"), "{rendered}");
    }

    /// **The one durable write of a value this seam carries, and the only
    /// column of the server row any of the six touches.**
    ///
    /// **What the raise writes is not this seam's value and not this row**: it
    /// is `State, identity and recovery` §8's condition record, whose durable
    /// shape that document fixes and whose contents are an identity and a set of
    /// value names. **So the assertion is written as two**: the server row moves
    /// in one column and no other, and the only other table any of the six
    /// touches is the failure record's own pair.
    #[test]
    fn d8s_write_is_the_only_one_that_reaches_a_servers_own_row() {
        let mut desk = a_desk("only-durable");
        let server = desk.server.clone();
        let before = desk.the_whole_record();

        for value in ObservedValue::ALL {
            desk.publishes.the_observed_value(&server, value);
        }
        desk.publishes
            .the_next_attempt_due(&server, NextAttemptDue::Absent);
        desk.publishes.the_changed_identity_warning(&server);
        desk.publishes
            .raise_the_connection_condition(&server, WhatTheRecordSays::raised());
        desk.publishes
            .record_the_scanned_host_key(&server, RememberedHostKey::approved(b"scanned".to_vec()));

        let after = desk.the_whole_record();
        let moved: Vec<&String> = after.iter().filter(|line| !before.contains(line)).collect();
        let tables: Vec<&str> = moved
            .iter()
            .filter_map(|line| line.split(':').next())
            .collect();
        for table in &tables {
            assert!(
                matches!(
                    *table,
                    "server_row" | "failure_record" | "failure_record_raised_over"
                ),
                "a publish wrote to {table}. **`Servers` D8 is the one durable \
                 write of a value this seam carries**, and the condition \
                 record's own two tables are `State, identity and recovery` \
                 §8's rather than this seam's"
            );
        }
        assert!(tables.contains(&"server_row"));

        // **The row moved in the host-key column and in no other.**
        desk.publishes.the_run.lend(&mut |lent| {
            let row = server_rows::one(lent.record, &server)
                .expect("the row reads")
                .expect("the row is there");
            let was = a_row(&server);
            assert!(row.remembered_host_key.is_some());
            assert_eq!(row.host.as_str(), was.host.as_str());
            assert_eq!(row.username.as_str(), was.username.as_str());
            assert_eq!(row.label, was.label);
            assert_eq!(row.key.fingerprint, was.key.fingerprint);
            assert_eq!(row.remembered_setup, was.remembered_setup);
            assert_eq!(
                row.remembered_hardware_reading,
                was.remembered_hardware_reading
            );
            assert_eq!(row.recorded_machine_root, was.recorded_machine_root);
            assert_eq!(
                row.last_confirmed_installation_identity,
                was.last_confirmed_installation_identity
            );
            assert_eq!(row.event_cursor, was.event_cursor);
            assert_eq!(row.machine_password_subject, was.machine_password_subject);
        });
    }

    // ---------------------------------------------------- the declaration

    /// **`Servers` K6's set is closed at three, and the two columns are
    /// composed from it rather than written again.**
    #[test]
    fn the_declared_values_are_k6s_three_and_nothing_else() {
        let mut every: Vec<&str> = HOLDS_OVER.to_vec();
        every.push(WITHDRAWN_ON);
        every.sort_unstable();
        let mut k6: Vec<&str> = ObservedValue::ALL
            .iter()
            .map(|v| the_written_form(*v))
            .collect();
        k6.sort_unstable();
        assert_eq!(
            every, k6,
            "**`Servers` K6: the connection's observed value has exactly three \
             values and the set is closed**, and every one of them is either a \
             value the condition holds over or the value it is withdrawn on"
        );
        assert_eq!(
            THE_CONNECTION_CONDITION_IS_DECLARED.supplied_by, THE_MODULE,
            "**`State, identity and recovery` F12: a condition with no named \
             supplier cannot be withdrawn and must not be raised**"
        );
    }

    // ------------------------------------------- the sequence, run on it

    struct AConnection {
        answers: HashMap<&'static str, CommandResult>,
    }

    impl CommandChannel for AConnection {
        async fn run(&mut self, command: CommandString) -> Result<CommandResult, CommandFailure> {
            self.answers
                .get(command.as_str())
                .cloned()
                .ok_or(CommandFailure::TheRequestDidNotReachTheMachine)
        }
    }

    impl TakesAFreeSpaceReading for AConnection {
        async fn free_space(&mut self, _path: &str) -> Option<FreeSpace> {
            None
        }
    }

    /// The opener, which offers a host key nothing remembers — `Servers` Q6's
    /// first path.
    struct TheOpener {
        offers: &'static [u8],
    }

    impl OpenConnections for TheOpener {
        type Connection = AConnection;

        async fn open(
            &mut self,
            _endpoint: Endpoint,
            _credential: Credential,
            mut decide: Box<dyn DecideHostTrust>,
            _deadline: Duration,
        ) -> Result<AConnection, OpenFailure> {
            if decide.decide(self.offers) == HostTrust::Stop {
                return Err(OpenFailure::HostTrustStoppedIt);
            }
            let mut answers = HashMap::new();
            answers.insert(
                "uname -s",
                CommandResult {
                    output: b"Linux\n".to_vec(),
                    complaint: Vec::new(),
                    status: Some(0),
                },
            );
            Ok(AConnection { answers })
        }

        fn ended(&mut self, _connection: &AConnection) -> Option<Ending> {
            None
        }

        async fn wait_until_it_ends(&mut self, _connection: &mut AConnection) -> Ending {
            Ending::TornDownBySmoozy
        }

        async fn tear_down(&mut self, _connection: AConnection) -> Ending {
            Ending::TornDownBySmoozy
        }
    }

    struct TheCredentials;

    impl SupplyTheCredential for TheCredentials {
        fn the_two_answers(&mut self, _key: &KeyReference) -> TheTwoAnswers {
            TheTwoAnswers {
                the_key_is_usable: true,
                a_passphrase_is_available_without_asking: true,
            }
        }

        fn for_one_attempt(
            &mut self,
            _key: &KeyReference,
            _who: WhoStartedIt,
            _naming: TheServerBeingConnectedTo<'_>,
        ) -> Option<Credential> {
            Some(Credential {
                private_key_file: std::path::PathBuf::from("/nowhere/id_ed25519"),
                passphrase: None,
            })
        }

        fn the_refusal_is_permanent(&mut self, _key: &KeyReference) -> PermanentRefusal {
            PermanentRefusal {
                the_store_standing_reads_never_answered: false,
                the_rows_key_is_protected: false,
            }
        }
    }

    struct TheReadStoodInFor;

    impl<C: Send> TheInstallationIdentityRead<C> for TheReadStoodInFor {
        async fn read(&mut self, _channel: &mut C) -> TheIdentityRead {
            TheIdentityRead {
                check: InstallationCheck::Absent,
                identity_on_the_machine: None,
            }
        }
    }

    struct TheWaiter;

    impl Wait for TheWaiter {
        async fn until(&mut self, _how_long: Duration) {}
        fn now(&mut self) -> SystemTime {
            SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000)
        }
    }

    /// **`Servers`' own sequence, run through the real publisher.**
    ///
    /// **`App/Servers/src/testing.rs` cannot be swapped into from here** — it is
    /// `#[cfg(test)]` and crate-private, so it compiles only while that crate is
    /// under test, and that crate's own integration tests write the same
    /// stand-ins again for the same reason. **What the swap has to show is
    /// unchanged**: the real publisher goes into `TheActingHalf` where a
    /// stand-in went, one field, and the sequence runs on it.
    ///
    /// **Two of the six are driven by one run**: `Servers` Q1 step 3 publishes
    /// *connected* the moment the transport reports the connection
    /// authenticated and usable, and `Servers` Q6's first path records the scanned key
    /// against a server that had nothing remembered.
    #[test]
    fn the_sequence_publishes_through_the_real_publisher() {
        let desk = a_desk("the-sequence");
        let server = desk.server.clone();
        let row = {
            let mut read = None;
            desk.publishes.the_run.lend(&mut |lent| {
                read = server_rows::one(lent.record, &server).expect("the row reads");
            });
            read.expect("the row is there")
        };
        let offered: &'static [u8] = b"ssh-ed25519 AAAAC3NzaC1lZDI1NTE5";
        let mut half = TheActingHalf {
            opener: TheOpener { offers: offered },
            credentials: TheCredentials,
            installation: TheReadStoodInFor,
            publishes: desk.publishes,
            wait: TheWaiter,
        };
        let attempt = AnAttempt {
            row: &row,
            who: WhoStartedIt::TheUser,
            the_first_connection_to_this_server: true,
            role: ConnectionRole::Primary,
            deadline: Duration::from_secs(20),
        };
        let ended = tauri::async_runtime::block_on(sequence::run(
            attempt,
            &mut half,
            &mut ChangedIdentityWarning::none_yet(),
            &mut |publishes| publishes.the_observed_value(&server, ObservedValue::Connected),
        ));
        assert!(matches!(ended, Attempt::ItIsLive(_)));

        let published = half.publishes;
        let seen: Vec<Delivery> = desk
            .page
            .lock()
            .expect("the deliveries")
            .iter()
            .filter(|delivery| delivery.kind == WHAT_IS_OBSERVED)
            .cloned()
            .collect();
        assert_eq!(
            ADesk::said(&seen[0], "theConditionsValue"),
            the_written_form(ObservedValue::Connected),
            "**`Servers` Q1 step 3: *connected* is published the moment the \
             transport reports the connection authenticated and usable**"
        );
        published.the_run.lend(&mut |lent| {
            let row = server_rows::one(lent.record, &server)
                .expect("the row reads")
                .expect("the row is there");
            let held = row
                .remembered_host_key
                .expect("**`Servers` Q6's first path: the scanned key is recorded**");
            assert!(held.matches(offered));
        });
    }
}
