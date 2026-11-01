//! §8 — machine-held work awaiting a person, and the one answer-release path.
//!
//! **U1. There is one hold record and one release path, and every place in this
//! project where the machine waits for a person uses them.** `Job files and
//! outputs` H3 states outright that **no second waiting mechanism is to be
//! invented.** None is.
//!
//! **U4. The waiting state has one home and it is this record**, and **the home
//! is the machine that holds the work** — because that is the only place both
//! answer routes can reach with Smoozy closed, and because the supervisor is the
//! single writer that makes two callers unable to change one job
//! independently.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rendering` Rows 4 and 23, and its stop. `Queue &
//! History` Rows 52, 53 and 54, and its H9 and L1. `Job files and outputs` H1,
//! H3, H4, R12, R13, R14, and its §4.3 and §10. `Telegram` Rows 275 and 276, and
//! its B1 to B15. `Server protocol and transfers` C4, C8. `State, identity and
//! recovery` §1.2, §1.3 rule 5, §2.1, §4 E1, E3, E7, E2, §6, §8.8. `Desktop app
//! and delivery` §3.3 P10d.
//!
//! **A marker this plan also defines is never listed here and is written out
//! with its document at the site**, because a header list cannot tell the two
//! apart and a bare one there would resolve to a real and wrong clause.

use smoozy_state::identities::Identity;
use smoozy_state::observed::{Clock, Reading};
use std::time::SystemTime;

/// **U1's two origins — different moments and the same machinery**, which `Queue
/// & History`'s Row 52 states in as many words: *"Two kinds, one arrangement."*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Origin {
    /// **Placed at submission.** The first-frame check's second job arrives on
    /// the machine already held; **the user's yes only releases it.**
    Submission,
    /// **Placed at the start of a render.** The render worker loads the file,
    /// evaluates the three conditions `Job files and outputs` H1 names, and where
    /// one holds it places the hold **after the file is loaded and before the
    /// render begins** (that document's H4).
    RenderStart,
}

/// **U2's subject kind, and it is what answers the scope line's *machine-held
/// jobs and commands*.**
///
/// **Both origins are jobs, and no document today places a hold whose subject is
/// a command.** The record carrying the subject kind is what lets a command hold
/// be added by the document that wants one **without a second waiting mechanism
/// and without a change here** — rather than by this document building a case
/// nobody has asked for.
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Subject {
    #[serde(with = "crate::written_form::an_identity")]
    Job(Identity),
    /// **Placed by no document today** (U1).
    #[serde(with = "crate::written_form::an_identity")]
    CommandAwaitingAPerson(Identity),
}

/// **U2. A hold record is a durable store of this document's, in the package
/// area, one per hold.**
///
/// **U3. That is a machine store kind `State, identity and recovery` §1.2's
/// inventory did not name.** It is a seam and not a licence to extend that
/// inventory here — its one home is that document's §1.2 — and it is recorded at
/// §16 item 5, which closed at the Architecture seam close.
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct HoldRecord {
    /// **The hold identity, issued by the supervisor when the hold is placed,
    /// opaque and never reused** (`State, identity and recovery` §2.1 binds it
    /// entire).
    #[serde(with = "crate::written_form::an_identity")]
    pub hold: Identity,
    pub subject: Subject,
    /// **The reason, from the closed set the placing document declares; this
    /// document carries the reason and never interprets it.**
    pub reason: String,
    pub origin: Origin,
    /// **The dependency, where the hold has one**: the identity of the work whose
    /// ending drops it, and **the release predicate the placing document
    /// declared** (§8.5).
    pub dependency: Option<Dependency>,
    /// **The release outcome, written once and never rewritten.**
    pub release: Option<Release>,
}

/// A hold's dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Dependency {
    #[serde(with = "crate::written_form::an_identity")]
    pub work: Identity,
    /// **U18. The predicate is declared by the document that placed the hold and
    /// evaluated by the supervisor; this document does not decide what counts as
    /// the dependency having ended badly.** Carried as the declaring document's
    /// own name for it.
    pub the_predicate_the_placing_document_declared: String,
}

/// **U2's release outcome — *released*, *stopped*, or *dropped* — with the
/// answer or the ending that produced it and the moment on the machine's
/// clock.**
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Release {
    pub outcome: ReleaseOutcome,
    /// What produced it: the answer, or the dependency's ending.
    pub produced_by: String,
    /// **The moment on the machine's clock** (`State, identity and recovery`
    /// D5).
    #[serde(with = "crate::written_form::a_reading_of_nothing")]
    pub at: Reading<()>,
}

/// The three, and there is no fourth.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ReleaseOutcome {
    Released,
    Stopped,
    /// **U20.** With the dependency's ending as the reason.
    Dropped,
}

/// An answer arriving for a hold.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Answer {
    /// **U9. An answer is an ordinary command with its own operation identity,
    /// naming the hold identity and carrying the answer.** It is subject to
    /// `Server protocol and transfers` C4 like any other: **the supervisor records
    /// the identity durably before acting on it, and a repeat of that identity
    /// never acts again.**
    pub operation: Identity,
    pub hold: Identity,
    pub says: WhatTheAnswerSays,
    /// **U10. Both routes arrive at the same command path, and there is no second
    /// one.** The two differ in how they reach the machine and in nothing after
    /// that.
    pub route: AnswerRoute,
}

/// Yes or no.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheAnswerSays {
    Yes,
    No,
}

/// **U10's two routes.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnswerRoute {
    /// **The desk's answer arrives over SSH through the command adapter and the
    /// machine-local endpoint.**
    TheDesk,
    /// **The phone's answer arrives at the Telegram role worker**, which submits
    /// phone actions to the same supervisor command path used by the SSH adapter
    /// and **never signals a render directly.**
    ///
    /// **The phone's answer arrives at whichever machine holds the listening
    /// slot, which need not be the machine holding the hold**, and the carriage
    /// between them is stated rather than assumed: the listener proposes the
    /// answer under its operation identity to `Telegram`'s bounded board, it is
    /// accepted only after the slot holder appends the complete entry and reads
    /// the board back exactly, and **from then until acknowledgement the board,
    /// outside every rental, is its authoritative durable home.**
    ///
    /// **A listening machine's own supervisor holds no hold record and releases
    /// nothing**, so it is never the addressee of an answer it merely received.
    ThePhone,
}

/// **U11. The answer's arrival needs no connection Smoozy holds.**
///
/// Row 4's own words: *"nothing needs Smoozy to be running for it"*. **The route
/// that satisfies that is the phone's**, and §10 is what makes it available on a
/// machine that is not the one listening.
pub const THE_ANSWERS_ARRIVAL_NEEDS_NO_CONNECTION_SMOOZY_HOLDS: bool = true;

/// What applying an answer produces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatAnAnswerProduces {
    /// **U12. The supervisor applies the answer by writing the release outcome,
    /// once. The write is what releases the subject; nothing else does.** A worker
    /// is started afterwards by the ordinary admission point (A4) and **never by
    /// the answering caller.**
    TheReleaseOutcomeIsWritten(Release),
    /// **U13. The first answer to be recorded wins, and the second acts on
    /// nothing.** **U14. The one that lost is told, and it is told what happened
    /// rather than that it failed** — the command's result carries the hold's
    /// recorded release outcome: which answer was applied, and the moment on the
    /// machine's clock. **A second answer is not an error**; it is `Server
    /// protocol and transfers` C8's *recorded outcome* answer.
    ActsOnNothingAndCarriesTheRecordedOutcome(Release),
}

/// **U12, U13 and U14 in one place, because they are one rule seen three
/// ways.**
///
/// **U15. A *no* that arrives after a *yes* is not converted into a stop.** The
/// hold is gone and there is nothing to answer; what remains available is the
/// ordinary stop, which `Rendering` owns and which both the desk and the phone
/// already offer. **Nothing here invents a second way to end work that has
/// started**, and the caller is told plainly that the hold was already answered,
/// by which answer, and when.
///
/// **U16. Neither route is privileged over the other, and the order is whatever
/// the machine recorded.** There is no arrangement in which a *no* wins over a
/// *yes* regardless of order, **because that would need a window in which a *yes*
/// does not release the job**, and Row 52 promises that saying yes puts the job
/// in line. The route is not a parameter of this function for that reason.
pub fn apply(
    record: &HoldRecord,
    answer: &Answer,
    at: SystemTime,
    on: Clock,
) -> WhatAnAnswerProduces {
    if let Some(already) = &record.release {
        return WhatAnAnswerProduces::ActsOnNothingAndCarriesTheRecordedOutcome(already.clone());
    }
    let outcome = match answer.says {
        WhatTheAnswerSays::Yes => ReleaseOutcome::Released,
        WhatTheAnswerSays::No => ReleaseOutcome::Stopped,
    };
    WhatAnAnswerProduces::TheReleaseOutcomeIsWritten(Release {
        outcome,
        produced_by: format!("an answer, over {:?}", answer.route),
        // **The clock is the caller's, never one composed here.** It is the
        // machine holding the hold record, which is where both routes end, and
        // naming it from inside this function would be inventing which machine
        // that was.
        at: Reading::taken((), at, on),
    })
}

/// **U12a. A *no* removes its subject, in the same act that writes its
/// release** — whenever the answer is a *no* and the release reads *stopped*,
/// whether this *no* wrote it or found it written. The removal is the
/// supervisor's to perform, as `Queue & History` L1 and `Queue & History` L5
/// remove a held job, and its result says the job went.
///
/// **A *yes* removes nothing, whatever it finds** (U15's mirror): a *yes* that
/// meets *stopped* is told what stood and the subject is left to the *no*.
/// **A *no* that finds *stopped* already written still removes**, so a
/// removal refused once, or a job an older program's *no* left held, is taken
/// by the next *no*. Added 2026-09-30, step 10c's eleventh-pass fix piece 8b.
pub fn the_answer_removes_its_subject(says: WhatTheAnswerSays, release: &Release) -> bool {
    says == WhatTheAnswerSays::No && release.outcome == ReleaseOutcome::Stopped
}

/// **U13's serialization, stated where a reader meets the race: it is on the
/// machine that holds the hold record**, because that is where both routes end —
/// the desk's over SSH, the phone's carried there by U10's instruction where it
/// arrived elsewhere — **so first-recorded-wins is true of both routes rather
/// than of one.**
///
/// **This is the race Row 53 says nobody has decided, and it is settled by the
/// serialization the shape already put in place.**
pub const THE_SERIALIZATION_IS_ON_THE_MACHINE_THAT_HOLDS_THE_RECORD: bool = true;

/// **U5. An unreleased hold removes its subject from the admission point
/// entirely.**
///
/// It is **not queued at low priority, not deferred, and not ordered behind
/// anything**: A10 means the arbitration mechanism never considers it. That is
/// `Queue & History`'s Row 52 — *"held outside its machine's line while it waits,
/// so it takes no place there and is not reordered"* — **kept by the subject not
/// being in the line rather than by a rule about how it is ordered within it.**
pub fn is_at_the_admission_point(record: &HoldRecord) -> bool {
    record.release.is_some()
}

/// **U6. A hold placed at the start of a render ends that attempt without
/// rendering, and the machine is released.**
///
/// **The render worker reports the hold and the reading it was computed from; the
/// supervisor issues the hold identity, writes the hold record and journals it**
/// (U2, U26, S1, M6); the worker writes the attempt's record — which is `Job files
/// and outputs`' own record in the job-record area (M8) and not one of the machine
/// stores S1 governs — and exits.
///
/// **It does not stay alive waiting**, because a Blender with the scene loaded
/// holds the machine's memory and its cards, and Row 52 requires that the machine
/// start the next job.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheHeldAttemptDoes {
    /// The ordinary case.
    EndsWithoutRenderingAndTheMachineIsReleased,
    /// **Where the supervisor is not answering at that moment the attempt places
    /// no hold at all**: it ends as an interrupted attempt and reconciliation
    /// takes it — **because a hold record written without a supervisor-issued
    /// identity and without a journal entry is a hold no reader on either cursor
    /// ever learns of, and U22 gives it no ending.**
    PlacesNoHoldAtAllAndEndsAsAnInterruptedAttempt,
}

/// **U6.**
pub fn what_the_held_attempt_does(the_supervisor_is_answering: bool) -> WhatTheHeldAttemptDoes {
    if the_supervisor_is_answering {
        WhatTheHeldAttemptDoes::EndsWithoutRenderingAndTheMachineIsReleased
    } else {
        WhatTheHeldAttemptDoes::PlacesNoHoldAtAllAndEndsAsAnInterruptedAttempt
    }
}

/// **U7. Release starts a fresh attempt, and that attempt is an attempt like any
/// other.**
///
/// `Job files and outputs` R12's fresh four-value check of the file is taken at
/// its start, before the map is applied, **exactly as it is for a first
/// attempt**. **Where the file is unchanged, nothing is recomputed**: the
/// recorded reading is the map in force per that document's §4.3 and R14, and the
/// folders the user was shown are the folders written into. **Where the values
/// differ, R13 governs** — the manifest's map is not applied, the worker takes a
/// fresh reading, composes a fresh map and records that the file changed — **and
/// this document adds nothing to it.**
///
/// **What the release does not do is put the question to the user again.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheFreshAttemptTakes {
    TheRecordedMapBecauseTheFileIsUnchanged,
    /// R13's, and this document adds nothing to it.
    AFreshReadingBecauseTheFourValuesDiffer,
}

/// **U7.**
pub fn what_the_fresh_attempt_takes(the_file_is_unchanged: bool) -> WhatTheFreshAttemptTakes {
    if the_file_is_unchanged {
        WhatTheFreshAttemptTakes::TheRecordedMapBecauseTheFileIsUnchanged
    } else {
        WhatTheFreshAttemptTakes::AFreshReadingBecauseTheFourValuesDiffer
    }
}

/// **U8. The released subject rejoins the line in the ordinary way, behind
/// whatever is waiting at the moment the answer comes.**
///
/// **How it is ordered there is `Queue & History`'s**; what this document supplies
/// is that **it enters the line at the moment the release outcome is written and
/// not before.**
pub fn enters_the_line_at(record: &HoldRecord) -> Option<&Reading<()>> {
    record.release.as_ref().map(|release| &release.at)
}

/// **§8.5 — the dependency that drops a hold.**
///
/// **U17. A hold may name a dependency, and the ending of that dependency drops
/// the hold without any answer.** `Telegram`'s Row 275 carries the negative case:
/// where the first-frame job fails or is stopped, **the waiting job is dropped and
/// the user is told why — done by the machine with nobody connected.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PredicateMoment {
    /// **U19. The predicate is evaluated at exactly one moment: when the
    /// dependency's recorded outcome is written.** Not on a timer, not at every
    /// reconciliation, and not on a connection. **That is the only moment at
    /// which the answer can change, and evaluating it anywhere else would be a
    /// poll.**
    WhenTheDependencysRecordedOutcomeIsWritten,
    /// **U19a. A second moment, for a dependency that ends without ever
    /// running: the removal of a job record a standing hold names as its
    /// dependency.**
    ///
    /// **It is not a poll** — it is one durable act, the removal, exactly as
    /// U19's moment is one durable act. **Why it is owed**: a first-frame job
    /// cancelled before it ever ran has no operation record and therefore no
    /// recorded outcome, so U19 never fires and U21 as it stood held the waiter
    /// for ever — while the spec's own list of endings names *"or it is cancelled
    /// before it ever runs"* among them. `Queue & History` H9 supplies the moment
    /// from that side, and **the removal is the deciding write** (that document's
    /// L1), so this evaluation rides it and never precedes it.
    WhenTheDependencysJobRecordIsRemoved,
}

impl PredicateMoment {
    /// The two, and there is no third.
    pub const THE_ONLY_TWO: [PredicateMoment; 2] = [
        PredicateMoment::WhenTheDependencysRecordedOutcomeIsWritten,
        PredicateMoment::WhenTheDependencysJobRecordIsRemoved,
    ];
}

/// **U30. When the answer can change and when the reading can be taken are two
/// different questions, and the reading is produced by the dependency's own
/// ending.**
///
/// U18's predicate reads the output folder, **which the supervisor may not read**
/// — `State, identity and recovery` §1.3 rule 5, with `Job files and outputs`'
/// re-run test as the single carve-out. **So the reading the predicate is
/// evaluated against is taken by the dependency job's own worker as a step of
/// that job's ending, and carried with the outcome the worker reports** — a step
/// of a worker rather than a poll.
///
/// **The predicate is evaluated at U19's moment against that reading and never
/// against a lookup at the supervisor and never against a reading taken before
/// the dependency ran.** Which reading it is, and what it must contain, is `Queue
/// & History`'s Row 54 and `Job files and outputs` §10's; **this document writes
/// neither** and records the seam at §16 item 17.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheReadingCarriedWithTheEnding {
    /// Opaque here. **Nothing in this crate reads inside it.**
    pub what_the_dependencys_own_worker_took: Vec<u8>,
}

/// What evaluating the predicate gives.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatThePredicateGives {
    /// **U20. A drop writes the release outcome *dropped*, with the dependency's
    /// ending as the reason, and journals it.** The subject leaves the line and
    /// **never enters it.** What the user is told and where is `Queue & History`'s
    /// and `Telegram`'s; §8.7 supplies how it reaches them.
    Drop,
    /// The dependency ended well; the hold stands and waits for its answer.
    TheHoldStands,
}

/// **U18, U19, U19a and U30 together — the moment, the reading, and the one thing
/// this document does not decide.**
///
/// The predicate itself is passed in, because **this document supplies the
/// evaluation moment and the record; it answers neither Row 54 nor Row 275.**
pub fn evaluate<P>(
    record: &HoldRecord,
    moment: PredicateMoment,
    reading: Option<&TheReadingCarriedWithTheEnding>,
    predicate: P,
) -> WhatThePredicateGives
where
    P: FnOnce(&TheReadingCarriedWithTheEnding) -> bool,
{
    if record.dependency.is_none() || record.release.is_some() {
        return WhatThePredicateGives::TheHoldStands;
    }
    match moment {
        // U19a's moment has no reading and needs none: the dependency ended
        // without ever running, so there is nothing for a reading to have been
        // taken of, and the waiter is dropped.
        PredicateMoment::WhenTheDependencysJobRecordIsRemoved => WhatThePredicateGives::Drop,
        PredicateMoment::WhenTheDependencysRecordedOutcomeIsWritten => match reading {
            // **A reading taken before the dependency ran is never substituted
            // for a missing one**, and neither is a lookup at the supervisor:
            // with no reading carried, the predicate is not evaluated and the
            // hold stands.
            None => WhatThePredicateGives::TheHoldStands,
            Some(reading) => {
                if predicate(reading) {
                    WhatThePredicateGives::Drop
                } else {
                    WhatThePredicateGives::TheHoldStands
                }
            }
        },
    }
}

/// **U21. A dependency whose ending is never recorded never drops the hold, save
/// for the one moment U19a names.**
///
/// Where the dependency's own operation was *given up* by reconciliation, **that
/// is a recorded outcome and U19 fires on it.** Where the machine is destroyed,
/// **nothing fires and nothing is recorded**, which is `Queue & History`'s own
/// answer for a machine that went away.
pub const A_DEPENDENCY_WHOSE_ENDING_IS_NEVER_RECORDED_NEVER_DROPS_THE_HOLD: bool = true;

/// **U22. The waiting ends in exactly one of three ways and there is no
/// fourth.**
///
/// **No age, no cap, no sweep, no schedule, and nothing that reaches out to a
/// machine to look** — the same closed list and the same reason as `State,
/// identity and recovery` §8.8.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HowTheWaitingEnds {
    /// U12: *released* or *stopped*.
    AnAnswerIsRecorded,
    /// U20.
    ItsDependencysEndingDropsIt,
    /// **Its subject is removed and the record goes with it.**
    ItsSubjectIsRemoved,
}

impl HowTheWaitingEnds {
    pub const THE_ONLY_THREE: [HowTheWaitingEnds; 3] = [
        HowTheWaitingEnds::AnAnswerIsRecorded,
        HowTheWaitingEnds::ItsDependencysEndingDropsIt,
        HowTheWaitingEnds::ItsSubjectIsRemoved,
    ];
}

/// **U23. That is the spec's promise and not a choice made here.** Row 52: *"**It
/// waits with no time limit.** Nothing expires it and nothing decides for the
/// user."*
///
/// The value is `()` because the clause is an absence: **no function in this file
/// takes an age, a deadline or a cap**, and there is nothing here a sweep could
/// be written against.
pub const NOTHING_EXPIRES_A_HOLD: () = ();

/// **U24. A hold survives every restart the machine can have.**
///
/// It is durable, **it is read by reconciliation at every supervisor start, and
/// reconciliation starts nothing whose hold is unreleased.** A reboot, a
/// supervisor crash and a container restart all leave the hold exactly as it was.
pub fn reconciliation_may_start(record: &HoldRecord) -> bool {
    record.release.is_some()
}

/// **U25. A hold blocks a package commit for as long as it stands** (G23). **That
/// is the shape's rule and U23 is why it can last indefinitely**; the caller is
/// told the upgrade is deferred and why.
pub fn blocks_a_package_commit(record: &HoldRecord) -> bool {
    record.release.is_none()
}

/// **§8.7 — what reaches a person who is not connected.**
///
/// **U26. Every hold placed, every release and every drop is journalled as an
/// event when it is written** (`State, identity and recovery` §4 E1), so a reader
/// learns of it **by reading forward from its own durable cursor**. **Nothing
/// polls and nothing is pushed to the desktop.**
///
/// **The desktop reader's read-forward recurs while it is connected**, on
/// `Desktop app and delivery` §3.3 P10d's bounded live-reading sequence: each
/// pass is that reader reading forward from its own durable cursor, which is this
/// clause's stated route. **What *nothing polls and nothing is pushed* forbids is
/// a machine-side watcher and a push, and that sequence is neither**; nothing on
/// this machine holds a schedule for it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JournalledWhenWritten {
    HoldPlaced,
    Released,
    Dropped,
}

/// **U27. There are exactly two readers of that journal and the Telegram worker
/// is one of them** (`State, identity and recovery` E2).
///
/// **That is what lets a machine tell a person something with nobody connected**:
/// the role worker reads the event and sends it. **What it sends, and how it is
/// paced, is `Telegram`'s.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JournalReader {
    TheDesktop,
    TheTelegramRoleWorker,
}

impl JournalReader {
    pub const EXACTLY_TWO: [JournalReader; 2] =
        [JournalReader::TheDesktop, JournalReader::TheTelegramRoleWorker];
}

/// **U28. A cursor advances after the work the event asked for is done, never on
/// receipt** (`State, identity and recovery` §4 E3), **so a role worker that
/// takes an event and dies before sending destroys nothing.**
pub const A_CURSOR_ADVANCES_AFTER_THE_WORK_IS_DONE: bool = true;

/// **U29. The hold record carries no notion of having been presented, and the
/// journal carries none either** (`State, identity and recovery` §4 E7).
///
/// **A single flag cannot serve two independent readers, and an event the role
/// worker presented would read as presented to the desktop.** The value is `()`
/// because the clause is an absence: [`HoldRecord`] has no such field, and that
/// is the whole of it.
pub const NO_PRESENTED_FLAG_ANYWHERE: () = ();

/// Writing hold records and journalling them, behind a trait. **The supervisor is
/// the writer** (S1); this is the write itself.
pub trait KeepsHoldRecords {
    /// **U2.** Written atomically, by staging and one publication (§14).
    fn place(&self, record: &HoldRecord) -> Result<(), String>;
    /// **U12's single write.** It is what releases the subject.
    fn write_the_release(&self, hold: &Identity, release: &Release) -> Result<(), String>;
    fn read(&self, hold: &Identity) -> Result<Option<HoldRecord>, String>;
}

#[cfg(test)]
mod tests {
    use smoozy_state::identities::IdentityKind;

    use super::*;

    fn an_identity(kind: IdentityKind) -> Identity {
        Identity::issue(kind).expect("randomness")
    }

    fn the_machines_clock() -> Clock {
        Clock::AMachine(smoozy_state::observed::MachineClock(
            "the machine holding the hold".into(),
        ))
    }

    fn a_hold(dependency: Option<Dependency>) -> HoldRecord {
        HoldRecord {
            hold: an_identity(IdentityKind::Operation),
            subject: Subject::Job(an_identity(IdentityKind::Job)),
            reason: "the first-frame check".into(),
            origin: Origin::Submission,
            dependency,
            release: None,
        }
    }

    fn an_answer(says: WhatTheAnswerSays, route: AnswerRoute, hold: Identity) -> Answer {
        Answer {
            operation: an_identity(IdentityKind::Operation),
            hold,
            says,
            route,
        }
    }

    #[test]
    fn the_first_answer_recorded_wins_and_the_second_acts_on_nothing() {
        let mut record = a_hold(None);
        let yes = an_answer(WhatTheAnswerSays::Yes, AnswerRoute::ThePhone, record.hold.clone());
        let produced = apply(&record, &yes, SystemTime::UNIX_EPOCH, the_machines_clock());
        let release = match produced {
            WhatAnAnswerProduces::TheReleaseOutcomeIsWritten(release) => release,
            other => panic!("expected a write, got {other:?}"),
        };
        assert_eq!(release.outcome, ReleaseOutcome::Released);
        record.release = Some(release.clone());

        // U15: a no arriving after a yes is not converted into a stop.
        let no = an_answer(WhatTheAnswerSays::No, AnswerRoute::TheDesk, record.hold.clone());
        assert_eq!(
            apply(&record, &no, SystemTime::UNIX_EPOCH, the_machines_clock()),
            WhatAnAnswerProduces::ActsOnNothingAndCarriesTheRecordedOutcome(release)
        );
    }

    /// U16: the race runs both ways and neither route is privileged.
    #[test]
    fn a_yes_at_the_desk_after_a_no_from_a_phone_reaches_the_same_place() {
        let mut record = a_hold(None);
        let no = an_answer(WhatTheAnswerSays::No, AnswerRoute::ThePhone, record.hold.clone());
        let release = match apply(&record, &no, SystemTime::UNIX_EPOCH, the_machines_clock()) {
            WhatAnAnswerProduces::TheReleaseOutcomeIsWritten(release) => release,
            other => panic!("expected a write, got {other:?}"),
        };
        assert_eq!(release.outcome, ReleaseOutcome::Stopped);
        record.release = Some(release.clone());
        let yes = an_answer(WhatTheAnswerSays::Yes, AnswerRoute::TheDesk, record.hold.clone());
        assert_eq!(
            apply(&record, &yes, SystemTime::UNIX_EPOCH, the_machines_clock()),
            WhatAnAnswerProduces::ActsOnNothingAndCarriesTheRecordedOutcome(release)
        );
    }

    #[test]
    fn an_unreleased_hold_is_not_at_the_admission_point_and_blocks_a_commit() {
        let record = a_hold(None);
        assert!(!is_at_the_admission_point(&record));
        assert!(blocks_a_package_commit(&record));
        assert!(!reconciliation_may_start(&record));
    }

    #[test]
    fn a_released_hold_enters_the_line_at_the_moment_the_outcome_was_written() {
        let mut record = a_hold(None);
        assert!(enters_the_line_at(&record).is_none());
        record.release = Some(Release {
            outcome: ReleaseOutcome::Released,
            produced_by: "an answer".into(),
            at: Reading::taken((), SystemTime::UNIX_EPOCH, the_machines_clock()),
        });
        assert!(enters_the_line_at(&record).is_some());
        assert!(!blocks_a_package_commit(&record));
    }

    #[test]
    fn a_hold_placed_with_no_answering_supervisor_is_no_hold_at_all() {
        assert_eq!(
            what_the_held_attempt_does(false),
            WhatTheHeldAttemptDoes::PlacesNoHoldAtAllAndEndsAsAnInterruptedAttempt
        );
        assert_eq!(
            what_the_held_attempt_does(true),
            WhatTheHeldAttemptDoes::EndsWithoutRenderingAndTheMachineIsReleased
        );
    }

    #[test]
    fn a_release_starts_a_fresh_attempt_that_takes_the_recorded_map_where_the_file_is_unchanged() {
        assert_eq!(
            what_the_fresh_attempt_takes(true),
            WhatTheFreshAttemptTakes::TheRecordedMapBecauseTheFileIsUnchanged
        );
        assert_eq!(
            what_the_fresh_attempt_takes(false),
            WhatTheFreshAttemptTakes::AFreshReadingBecauseTheFourValuesDiffer
        );
    }

    #[test]
    fn the_predicate_is_evaluated_at_two_moments_and_no_others() {
        // **The set is closed by the compiler rather than by a count.** The
        // match is total and carries no default arm, so a moment added to the
        // enum has none, the crate stops compiling, and whoever added it is sent
        // to this list — where the length assertion this replaced could never
        // have been false, the array's own type having fixed it.
        for (at, moment) in PredicateMoment::THE_ONLY_TWO.iter().enumerate() {
            let where_the_list_puts_it = match moment {
                PredicateMoment::WhenTheDependencysRecordedOutcomeIsWritten => 0,
                PredicateMoment::WhenTheDependencysJobRecordIsRemoved => 1,
            };
            assert_eq!(at, where_the_list_puts_it, "{moment:?} is not where the list puts it");
        }
    }

    #[test]
    fn a_dependency_that_ended_badly_drops_the_hold_at_the_outcomes_write() {
        let record = a_hold(Some(Dependency {
            work: an_identity(IdentityKind::Job),
            the_predicate_the_placing_document_declared: "`Queue & History` Row 54".into(),
        }));
        let reading = TheReadingCarriedWithTheEnding {
            what_the_dependencys_own_worker_took: b"no frame".to_vec(),
        };
        assert_eq!(
            evaluate(
                &record,
                PredicateMoment::WhenTheDependencysRecordedOutcomeIsWritten,
                Some(&reading),
                |_| true
            ),
            WhatThePredicateGives::Drop
        );
    }

    /// U30: with no reading carried, the predicate is not evaluated against a
    /// lookup at the supervisor and the hold stands.
    #[test]
    fn no_reading_carried_means_the_hold_stands_rather_than_a_lookup() {
        let record = a_hold(Some(Dependency {
            work: an_identity(IdentityKind::Job),
            the_predicate_the_placing_document_declared: "`Queue & History` Row 54".into(),
        }));
        assert_eq!(
            evaluate(
                &record,
                PredicateMoment::WhenTheDependencysRecordedOutcomeIsWritten,
                None,
                |_| true
            ),
            WhatThePredicateGives::TheHoldStands
        );
    }

    /// U19a: a dependency cancelled before it ever ran drops the waiter at the
    /// removal, which is the moment U19 never reaches.
    #[test]
    fn a_dependency_removed_before_it_ever_ran_drops_the_waiter() {
        let record = a_hold(Some(Dependency {
            work: an_identity(IdentityKind::Job),
            the_predicate_the_placing_document_declared: "`Queue & History` Row 54".into(),
        }));
        assert_eq!(
            evaluate(
                &record,
                PredicateMoment::WhenTheDependencysJobRecordIsRemoved,
                None,
                |_| false
            ),
            WhatThePredicateGives::Drop
        );
    }

    /// **A hold with no dependency is never dropped, and its predicate is
    /// never run**, at either moment, with or without a reading carried —
    /// `Rendering` S18a's hold for the driver is such a hold, released only by
    /// an answer (U22). *The second moment and the predicate that may not be
    /// run were added by held-job piece A, 2026-10-12.*
    #[test]
    fn a_hold_with_no_dependency_is_never_dropped_by_a_predicate() {
        let record = a_hold(None);
        let carried = TheReadingCarriedWithTheEnding {
            what_the_dependencys_own_worker_took: b"no frame".to_vec(),
        };
        for moment in [
            PredicateMoment::WhenTheDependencysJobRecordIsRemoved,
            PredicateMoment::WhenTheDependencysRecordedOutcomeIsWritten,
        ] {
            for reading in [None, Some(&carried)] {
                assert_eq!(
                    evaluate(&record, moment, reading, |_| -> bool {
                        panic!("a predicate was run for a hold with no dependency")
                    }),
                    WhatThePredicateGives::TheHoldStands
                );
            }
        }
    }

    #[test]
    fn the_waiting_ends_three_ways_and_the_journal_has_two_readers() {
        // **Both sets are closed by the compiler rather than by a count**, each
        // match being total and carrying no default arm.
        for (at, ending) in HowTheWaitingEnds::THE_ONLY_THREE.iter().enumerate() {
            let where_the_list_puts_it = match ending {
                HowTheWaitingEnds::AnAnswerIsRecorded => 0,
                HowTheWaitingEnds::ItsDependencysEndingDropsIt => 1,
                HowTheWaitingEnds::ItsSubjectIsRemoved => 2,
            };
            assert_eq!(at, where_the_list_puts_it, "{ending:?} is not where the list puts it");
        }
        for (at, reader) in JournalReader::EXACTLY_TWO.iter().enumerate() {
            let where_the_list_puts_it = match reader {
                JournalReader::TheDesktop => 0,
                JournalReader::TheTelegramRoleWorker => 1,
            };
            assert_eq!(at, where_the_list_puts_it, "{reader:?} is not where the list puts it");
        }
    }

    #[test]
    fn a_command_subject_is_expressible_and_no_document_places_one_today() {
        let record = HoldRecord {
            subject: Subject::CommandAwaitingAPerson(an_identity(IdentityKind::Operation)),
            ..a_hold(None)
        };
        assert!(matches!(
            record.subject,
            Subject::CommandAwaitingAPerson(_)
        ));
    }

    #[test]
    fn the_three_journalled_moments_are_the_placing_the_release_and_the_drop() {
        let all = [
            JournalledWhenWritten::HoldPlaced,
            JournalledWhenWritten::Released,
            JournalledWhenWritten::Dropped,
        ];
        // **A fourth journalled moment fails the build here**, the match being
        // total and carrying no default arm — where the length of a list written
        // out inside the test could never have been false.
        for (at, moment) in all.iter().enumerate() {
            let where_the_list_puts_it = match moment {
                JournalledWhenWritten::HoldPlaced => 0,
                JournalledWhenWritten::Released => 1,
                JournalledWhenWritten::Dropped => 2,
            };
            assert_eq!(at, where_the_list_puts_it, "{moment:?} is not where the list puts it");
        }
    }
}
