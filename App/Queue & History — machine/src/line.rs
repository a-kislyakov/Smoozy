//! The line: the job record's shape, the sequence keys, and every ordering
//! rule over them — `Queue & History` Q2, Q4's order rule, Q5, Q7; H3; L6,
//! L7, L8, L9, L18, L20, L21 and L22; and, from step 10c's first stop-and-clear piece, what L14's
//! clear covers and whom L13's paired stop takes, under L19's group form.
//!
//! **Constraint:** `State, identity and recovery` §3 D1 — *a job's place in a
//! machine's line* is one durable desired state, written by the supervisor
//! and by nothing else; its D2 — no desired state is ever inferred from an
//! observed reading; its §9.3 — an ordered set is one unit. Every act below
//! is a write to that desired state that the machine reconciles.
//!
//! **What is here and what is not, because §1's store is the acting half's.**
//! Q1's durable store, Q3's publication order, Q6's untracked records and Q8's
//! disk-full answer are acts and readings and land later
//! (`smoozy-queue-history`'s `arriving_later`). **What a job record holds (Q2), what the line
//! *is* (Q4's order rule), that a held record holds no key (Q5), and how a key
//! is chosen (Q7) are data shapes and arithmetic**, and every rewrite of the
//! line in §4 is a pure function over them — computed here, written durably
//! by the single writer there.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rented-server runtime` U2, U5, S1. `Job files
//! and outputs` M1. `Server Setup` B14. `Dependency Upload & Remap` K9.

use smoozy_state::identities::{Identity, IdentityKind};

/// **Q7: sequence keys are per machine, opaque outside the supervisor, and
/// unique among the records live in a machine's queue at any one moment.**
///
/// The rule binds the live line and not the machine's whole history, so no
/// durable high-water mark is kept and a value may recur after the record
/// holding it is gone. **Nothing parses a key, nothing infers submission
/// order from one, and the desktop never composes one**: it names jobs and
/// positions (L6) and the supervisor writes the keys — through the functions
/// below, which run on the supervisor's side of the wire.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SequenceKey(u64);

impl SequenceKey {
    /// **Q7's one rule for a new key: greater than every key then in the
    /// line — joining the end.** H3 assigns on the same terms and the two say
    /// one thing.
    pub fn greater_than_every_key_in(line: &[SequenceKey]) -> SequenceKey {
        SequenceKey(line.iter().map(|key| key.0).max().map_or(1, |top| top + 1))
    }

    /// The written value, for the store and for nothing else. **Opaque
    /// outside the supervisor**: the desktop never sees one.
    pub fn as_written(self) -> u64 {
        self.0
    }

    /// Read back from the store.
    pub fn read_back(written: u64) -> SequenceKey {
        SequenceKey(written)
    }
}

/// **Q2's queue state, from a closed set**, each written at the event that
/// makes it true and journalled. **The set does not move**: L12a's
/// *stopping* and L2's *cancelling* are the desktop's own statements about
/// its own outstanding commands and never a sixth state; L21's mark is a
/// field and not a state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueueState {
    Waiting,
    /// With the hold identity (`Rented-server runtime` U2).
    Held {
        hold: Identity,
    },
    /// With the render operation's identity.
    Rendering {
        operation: Identity,
    },
    /// With the recorded outcome's identity.
    Ended {
        outcome: Identity,
    },
}

/// **Q2: a job record holds, and holds nothing that is an observed reading.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JobRecord {
    /// The job identity, and the group identity where the job came from an
    /// all-scenes submission — the same pair the manifest carries (`Job files
    /// and outputs` M1), so a group is readable off the queue with no group
    /// store anywhere (L18).
    pub job: Identity,
    pub group: Option<Identity>,
    /// **The sequence key: the job's desired place in its machine's line**,
    /// `State, identity and recovery` D1's durable desired state, written by
    /// the supervisor and by nothing else. **Q5: a held record holds none**,
    /// which is what *takes no place there and is not reordered* means
    /// concretely — the type says so rather than a rule somebody remembers.
    pub sequence_key: Option<SequenceKey>,
    pub state: QueueState,
    /// The reference to the job record directory `Job files and outputs`
    /// owns. Nothing of that directory's is copied into the queue record.
    pub record_directory: String,
    /// **L21's removed-build mark**, present only where a Blender removal
    /// covered this job's recorded build path — written at that removal, read
    /// at the admission point alone, and not a queue state.
    pub removed_build: Option<RemovedBuildMark>,
    /// **`Telegram` V1c: the job's current reporting choice, where the desk
    /// changed it while the job was live** — one `Telegram` V1a value in the
    /// manifest's own form, written as JSON text. **Absent where it was never
    /// changed**, and then the manifest's field is the choice. The manifest is
    /// never rewritten; this is a second, later value over it. Added
    /// 2026-10-30, the twenty-first notes, note 7.
    pub reporting_choice_now: Option<String>,
}

impl JobRecord {
    /// A fresh record at submission: joining the end of the line, waiting.
    pub fn submitted(
        job: Identity,
        group: Option<Identity>,
        record_directory: String,
        line: &[SequenceKey],
    ) -> JobRecord {
        debug_assert_eq!(job.kind(), IdentityKind::Job);
        JobRecord {
            job,
            group,
            sequence_key: Some(SequenceKey::greater_than_every_key_in(line)),
            state: QueueState::Waiting,
            record_directory,
            removed_build: None,
            reporting_choice_now: None,
        }
    }

    /// A record held from submission — no key, and the hold's identity.
    pub fn held_from_submission(
        job: Identity,
        group: Option<Identity>,
        record_directory: String,
        hold: Identity,
    ) -> JobRecord {
        JobRecord {
            job,
            group,
            sequence_key: None,
            state: QueueState::Held { hold },
            record_directory,
            removed_build: None,
            reporting_choice_now: None,
        }
    }

    pub fn is_waiting(&self) -> bool {
        matches!(self.state, QueueState::Waiting)
    }

    /// **§17's live set**: a job is live from its record's publication until
    /// its recorded outcome is written, held jobs included.
    pub fn is_live(&self) -> bool {
        !matches!(self.state, QueueState::Ended { .. })
    }
}

/// **L21: what the mark holds is Q2's fifth field: the removal operation's
/// identity, and the answered version of the build this job was submitted
/// with**, copied from that job's manifest (N19) while the manifest stands.
/// Not a queue state, not a desired state; nothing clears it and nothing
/// re-checks it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemovedBuildMark {
    pub removal: Identity,
    pub the_answered_version_the_job_was_submitted_with: String,
}

/// **Q4's order rule: the line is the job records with state *waiting*, in
/// ascending sequence-key order.** Q5: a held record is not in it. Consumed
/// at the admission point by the acting half; composed here.
pub fn the_line(records: &[JobRecord]) -> Vec<&JobRecord> {
    let mut line: Vec<&JobRecord> = records
        .iter()
        .filter(|record| record.is_waiting() && record.sequence_key.is_some())
        .collect();
    line.sort_by_key(|record| record.sequence_key);
    line
}

/// The keys now in the line, for Q7's and H3's *greater than every key*.
pub fn the_keys_in_the_line(records: &[JobRecord]) -> Vec<SequenceKey> {
    the_line(records)
        .iter()
        .filter_map(|record| record.sequence_key)
        .collect()
}

/// **H3: the released subject enters the line behind whatever is waiting at
/// the moment the release outcome is written, and the supervisor writes that
/// placement in the same act as the release.** The released job record takes
/// state *waiting* and a sequence key greater than every key then in that
/// machine's line — not the front, not its old place. The moment is the
/// release outcome's write, on the machine, so it is right whether the
/// answer came from the desk, from a phone, or hours after Smoozy closed.
pub fn the_released_placement(released: &mut JobRecord, records: &[JobRecord]) {
    debug_assert!(matches!(released.state, QueueState::Held { .. }));
    released.state = QueueState::Waiting;
    released.sequence_key = Some(SequenceKey::greater_than_every_key_in(
        &the_keys_in_the_line(records),
    ));
}

/// **L6: a reorder names jobs and positions, never indexes.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Position {
    AfterJob(Identity),
    AtTheFrontOfTheWaitingLine,
}

/// What a reorder's outcome says about a job it named.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotMoved {
    /// It started rendering or ended while the command travelled (L6, L8).
    ItIsNoLongerWaiting { job: Identity, now: QueueState },
    /// A held job has no place in the line to move (L9).
    ItIsHeld { job: Identity },
    /// The named job is on no record of this machine's.
    NoSuchJob { job: Identity },
}

/// **The one durable write a reorder or a block move produces: the new
/// sequence keys, per job, for every waiting record.** L6 and L7 rewrite them
/// in one write; the single writer serializes it against the admission point
/// (L8), so the top of the line cannot be consumed *during* the write.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rewrite {
    /// Every waiting record's key after the write, in line order.
    pub keys: Vec<(Identity, SequenceKey)>,
    /// What the outcome names as not moved, and why.
    pub not_moved: Vec<NotMoved>,
}

/// **L6, performed over the line as it is true on the machine** — never over
/// what the desktop last saw (`State, identity and recovery` D2). A named job
/// that is no longer waiting is not moved, and the outcome says so; a named
/// predecessor no longer waiting resolves to the front of the waiting line,
/// which is where its position went. **L9: reordering never touches a
/// rendering job and never touches a held one** — the line below holds only
/// waiting records, so neither can be moved by construction.
pub fn reorder(records: &[JobRecord], job: &Identity, position: &Position) -> Rewrite {
    let line = the_line(records);
    let Some(moving) = line.iter().position(|record| &record.job == job) else {
        return Rewrite {
            keys: keys_as_they_stand(&line),
            not_moved: vec![why_not_moved(records, job)],
        };
    };
    let mut order: Vec<&JobRecord> = line.clone();
    let taken = order.remove(moving);
    let at = insertion_point(&order, position);
    order.insert(at, taken);
    Rewrite {
        keys: fresh_keys(&order, records),
        not_moved: Vec::new(),
    }
}

/// **L7: a group block move is a gather-and-insert, one command, one
/// write.** Every *waiting* member — read from the group identity on the job
/// records (L18) — is gathered keeping its current relative order, and
/// written as one contiguous block at the named position. A member already
/// rendering stays where it is and is named in the outcome as not moved
/// because it is rendering; a member that ended is simply absent; a held
/// member has no place to move (L9). **`State, identity and recovery` §9.3
/// makes the block one partition unit**: it lands whole or the write fails
/// whole, and a half-moved group cannot exist — which is why this returns
/// one rewrite and never a sequence of them.
pub fn block_move(records: &[JobRecord], group: &Identity, position: &Position) -> Rewrite {
    debug_assert_eq!(group.kind(), IdentityKind::Group);
    let line = the_line(records);
    let (block, rest): (Vec<&JobRecord>, Vec<&JobRecord>) = line
        .iter()
        .partition(|record| record.group.as_ref() == Some(group));
    let not_moved = records
        .iter()
        .filter(|record| record.group.as_ref() == Some(group) && !record.is_waiting())
        .filter_map(|record| match &record.state {
            QueueState::Rendering { .. } => Some(NotMoved::ItIsNoLongerWaiting {
                job: record.job.clone(),
                now: record.state.clone(),
            }),
            QueueState::Held { .. } => Some(NotMoved::ItIsHeld {
                job: record.job.clone(),
            }),
            // A member that ended is simply absent.
            QueueState::Ended { .. } | QueueState::Waiting => None,
        })
        .collect();
    if block.is_empty() {
        return Rewrite {
            keys: keys_as_they_stand(&line),
            not_moved,
        };
    }
    let mut order = rest;
    let at = insertion_point(&order, position);
    for (offset, member) in block.into_iter().enumerate() {
        order.insert(at + offset, member);
    }
    Rewrite {
        keys: fresh_keys(&order, records),
        not_moved,
    }
}

/// Where a position lands in a line the moving records have been taken out
/// of. **A predecessor no longer waiting resolves to the front** (L6).
fn insertion_point(order: &[&JobRecord], position: &Position) -> usize {
    match position {
        Position::AtTheFrontOfTheWaitingLine => 0,
        Position::AfterJob(predecessor) => order
            .iter()
            .position(|record| &record.job == predecessor)
            .map_or(0, |at| at + 1),
    }
}

/// **Q7: a rewrite numbers from above every key a live record holds, not from
/// one.**
///
/// The rewrite is partial across records (§16, L24), so the old keys and the
/// new ones stand together while it lands — and Q7 binds the keys unique
/// among the records live in a machine's queue **at any one moment**, which
/// numbering from one breaks at every one of those moments. Numbering above
/// the live set keeps both that invariant and D3's — the interrupted job's
/// record still holds the lowest live key — true throughout.
///
/// **The live set is [`JobRecord::is_live`]'s** — waiting, held or rendering —
/// so a held record's absent key and a rendering record's key are both
/// accounted, and a key may still recur once the record holding it is gone.
fn fresh_keys(order: &[&JobRecord], records: &[JobRecord]) -> Vec<(Identity, SequenceKey)> {
    let live: Vec<SequenceKey> = records
        .iter()
        .filter(|record| record.is_live())
        .filter_map(|record| record.sequence_key)
        .collect();
    let above = SequenceKey::greater_than_every_key_in(&live);
    order
        .iter()
        .enumerate()
        .map(|(at, record)| (record.job.clone(), SequenceKey(above.0 + at as u64)))
        .collect()
}

fn keys_as_they_stand(line: &[&JobRecord]) -> Vec<(Identity, SequenceKey)> {
    line.iter()
        .map(|record| {
            (
                record.job.clone(),
                record
                    .sequence_key
                    .expect("a record in the line holds a key"),
            )
        })
        .collect()
}

fn why_not_moved(records: &[JobRecord], job: &Identity) -> NotMoved {
    match records.iter().find(|record| &record.job == job) {
        None => NotMoved::NoSuchJob { job: job.clone() },
        Some(record) => match &record.state {
            QueueState::Held { .. } => NotMoved::ItIsHeld { job: job.clone() },
            _ => NotMoved::ItIsNoLongerWaiting {
                job: job.clone(),
                now: record.state.clone(),
            },
        },
    }
}

/// **L8: the race is closed by serialization, not by care.** Starting is the
/// supervisor's own act at the admission point, the reorder is the
/// supervisor's own write, and one writer does one thing at a time
/// (`Rented-server runtime` S1). A job taken by the admission point before
/// the reorder's write is a job the reorder reports as not moved; a job
/// still waiting when the write lands is moved. **There is no interleaving in
/// which a moved job renders in the old order** — and [`reorder`] is computed
/// over the records as they stand at the write, which is what makes that
/// true rather than hoped.
pub const THE_RACE_IS_CLOSED_BY_THE_SINGLE_WRITER: bool = true;

/// **L18: a group's membership is held in exactly two places, both per-job,
/// and there is no group store.** On the machine, the group identity on each
/// member's job record; on the desktop, on each member's local job record.
/// Membership is data on the members, so it survives scattering, a reorder,
/// the cancel or stop of any member, Smoozy being closed and the machine
/// going down.
pub fn the_members_of<'a>(records: &'a [JobRecord], group: &Identity) -> Vec<&'a JobRecord> {
    records
        .iter()
        .filter(|record| record.group.as_ref() == Some(group))
        .collect()
}

/// **L20: a group is drawn as a group from the group identity on the
/// members' own local records, joined to the period-held reading by job
/// identity, composed at the draw** — and the queue reading carries no group
/// field. So a group whose machine cannot be read is still drawn as a group.
/// The input is the desktop's copies; the output is the grouping, stored
/// never.
pub fn group_the_rows<Row>(
    local_records: &[(Identity, Option<Identity>, Row)],
) -> Vec<(Option<&Identity>, Vec<&Row>)> {
    let mut groups: Vec<(Option<&Identity>, Vec<&Row>)> = Vec::new();
    for (_, group, row) in local_records {
        match groups.iter_mut().find(|(held, _)| *held == group.as_ref()) {
            Some((_, rows)) => rows.push(row),
            None => groups.push((group.as_ref(), vec![row])),
        }
    }
    groups
}

/// **L21: the one ground a job's chosen Blender may be read as gone is a
/// removal Smoozy performed and recorded.** The supervisor writes the mark,
/// in the same act in which the removal's recorded outcome is written, on
/// every live job record `Server Setup` B14's predicate covers — the live
/// set being the records with state *waiting*, *held* or *rendering*. **No
/// clause here reads the machine for a build**: the set is handed in,
/// composed once by that predicate, and this only writes the field.
pub fn mark_the_removed_build(
    records: &mut [JobRecord],
    covered: &[Identity],
    removal: &Identity,
    the_answered_version: &str,
) {
    for record in records.iter_mut() {
        if record.is_live() && covered.contains(&record.job) {
            record.removed_build = Some(RemovedBuildMark {
                removal: removal.clone(),
                the_answered_version_the_job_was_submitted_with: the_answered_version.to_string(),
            });
        }
    }
}

/// What the admission point does in one serialized act (Q4 and L22).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtTheAdmissionPoint {
    /// **L22: each marked record it reached, dropped in the same act** —
    /// removed, journalled, its material cleanup following per L1 — until a
    /// startable one or the line's end.
    pub dropped: Vec<Dropped>,
    /// The lowest-keyed *waiting* record without the mark, which the
    /// supervisor starts; `None` where the line is empty after the drops.
    pub started: Option<Identity>,
}

/// **L22's telling comes off the journal event, because every record of
/// that job's is removed in the same act.** It carries the job identity, a
/// bounded rendering of the label, and the reason — the Blender the job was
/// submitted with was removed from the machine, named by the answered
/// version L21's mark holds. **It names no record and carries no path.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dropped {
    pub job: Identity,
    pub a_bounded_rendering_of_the_label: String,
    pub the_answered_version_the_job_was_submitted_with: String,
}

/// **Q4's admission choice with L22's drop, as one pure act over the line.**
/// When no render operation is running, the supervisor takes the lowest-keyed
/// *waiting* record; where it carries L21's mark it is dropped rather than
/// started and the next is taken in the same act. **The mark fires on a
/// *waiting* record and on no other** — a record marked while rendering never
/// returns to waiting, and a held one takes its place in the line at release
/// (H3) and is dropped when its turn comes.
pub fn the_admission_point(
    records: &[JobRecord],
    a_bounded_label_for: impl Fn(&Identity) -> String,
) -> AtTheAdmissionPoint {
    let mut dropped = Vec::new();
    for record in the_line(records) {
        match &record.removed_build {
            Some(mark) => dropped.push(Dropped {
                job: record.job.clone(),
                a_bounded_rendering_of_the_label: a_bounded_label_for(&record.job),
                the_answered_version_the_job_was_submitted_with: mark
                    .the_answered_version_the_job_was_submitted_with
                    .clone(),
            }),
            None => {
                return AtTheAdmissionPoint {
                    dropped,
                    started: Some(record.job.clone()),
                }
            }
        }
    }
    AtTheAdmissionPoint {
        dropped,
        started: None,
    }
}

/// **L22: nothing is written to history and no tombstone is written for a
/// dropped job.** It never started rendering, so it never entered history
/// (C9); the machine-side record goes in the drop's own act, so no erase is
/// owed. On the desktop the local job record is removed when the drop event
/// is applied, provisional or confirmed alike — a job that never rendered
/// leaves no trace.
pub const A_DROPPED_JOB_LEAVES_NO_HISTORY_ENTRY_AND_NO_TOMBSTONE: bool = true;

/// **§17's live-job supply**: the live set per machine is the job records
/// with state *waiting*, *held* or *rendering*; what a live job depends on is
/// the manifest's `.blend` and the per-job resolved set `Dependency Upload &
/// Remap` K9 fixes — read through the record directory, never re-derived.
pub fn the_live_set(records: &[JobRecord]) -> Vec<&JobRecord> {
    records.iter().filter(|record| record.is_live()).collect()
}

// ---------------------------------------------------------------------------
// L13, L14 and L19 — what the clear covers and whom the paired stop takes
// ---------------------------------------------------------------------------
//
// **Moved here from `smoozy-queue-history`'s `stop_and_clear` on 2026-09-29,
// step 10c's first stop-and-clear piece, unchanged**, because the program a rented machine runs is what
// applies them and it cannot reach that crate. `stop_and_clear` re-exports all
// four at the path they had, and its tests over them stay there.

/// **The operation holding the exclusive render at the moment the clear's
/// write lands**, as the supervisor knows it inside the same serialized
/// sequence (`Rented-server runtime` S1). The one input to L14's exemption
/// and L13's resolution, and it is the machine's own and never the desktop's.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheRunningOne {
    pub operation: Identity,
    pub job: Identity,
}

/// **L14: the clear comes first and is one durable write removing every job
/// record it covers.** It covers every job record on that machine that is not
/// the running one — waiting in the line and held outside it alike, because
/// the promise is an emptied machine. ***The running one* has one reading
/// and only one**: the operation holding the exclusive render at the moment
/// of this write — never the operation the desktop's reading last showed. So
/// a job the admission point took after the desktop composed the pair is
/// the one the clear exempts and the one the stop then takes.
///
/// **L19: cancel-group reads *the line* as the group's members.** Non-members
/// stay, and the admission point can and should take them.
///
/// **The exemption is by identity and never by state.** A record whose state
/// reads *rendering* and whose operation is not the resolved running one is
/// covered: it holds no worker — the machine runs one exclusive render — and
/// leaving it behind would leave a job on a machine the user asked to have
/// emptied, which is the one thing L14 says never happens. A record left
/// *rendering* by an interruption is exactly that shape.
pub fn what_the_clear_covers<'a>(
    records: &'a [JobRecord],
    running: Option<&TheRunningOne>,
    scoped_to: Option<&Identity>,
) -> Vec<&'a JobRecord> {
    records
        .iter()
        .filter(|record| match &record.state {
            QueueState::Waiting | QueueState::Held { .. } => true,
            // **The exemption is by identity, not by state.** *Not covered*
            // only where the resolved running operation names this record;
            // a record reading *rendering* that is not the resolved one holds
            // no worker, and L14's promise is an emptied machine. **The rule
            // is the branch itself rather than a `debug_assert!`**, so it
            // holds in the build the user runs rather than only in a test
            // build.
            QueueState::Rendering { operation } => {
                !running.is_some_and(|running| &running.operation == operation)
            }
            QueueState::Ended { .. } => false,
        })
        .filter(|record| scoped_to.is_none_or(|group| record.group.as_ref() == Some(group)))
        .collect()
}

/// **L13's resolution: the stop's subject is whichever operation holds the
/// exclusive render at the moment the clear's write lands**, resolved by the
/// supervisor inside the same serialized sequence as that write, and written
/// into the stop's command record at resolution. **L19**: under the
/// group-scoped form the subject is that operation only where its job record
/// carries this group identity; where the operation running at that moment
/// is not a member, no stop is attempted and the outcome says so.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheStopsSubject {
    Resolved {
        operation: Identity,
        /// **Where the resolved operation is not the one the desktop's
        /// reading last showed, the outcome names both.**
        the_desktop_last_showed: Option<Identity>,
    },
    /// Nothing was rendering at the clear's write: there is nothing to stop.
    NothingWasRendering,
    /// L19: the running operation is not a member of the group.
    NotAttemptedTheRunningJobIsNotAMember { running: Identity },
}

pub fn resolve_the_stops_subject(
    running: Option<&TheRunningOne>,
    running_jobs_group: Option<&Identity>,
    scoped_to: Option<&Identity>,
    the_desktop_last_showed: Option<&Identity>,
) -> TheStopsSubject {
    let Some(running) = running else {
        return TheStopsSubject::NothingWasRendering;
    };
    if let Some(group) = scoped_to {
        if running_jobs_group != Some(group) {
            return TheStopsSubject::NotAttemptedTheRunningJobIsNotAMember {
                running: running.operation.clone(),
            };
        }
    }
    TheStopsSubject::Resolved {
        operation: running.operation.clone(),
        the_desktop_last_showed: the_desktop_last_showed
            .filter(|shown| *shown != &running.operation)
            .cloned(),
    }
}

/// **The jobs, groups, operations and lines the tests build from, written once
/// for both folders of this module.**
///
/// `smoozy-queue-history` keeps the tests over what stayed there — the idle
/// half hour, the catch-up, the stop-and-clear, the two records and the local
/// job record — and they build from these. A test-only module cannot be reached
/// from another crate, so this one is compiled for this crate's own tests and,
/// behind the `testing` switch, for that crate's tests, which turn it on as a
/// development dependency. **Nothing outside a test ever turns it on**, and the
/// program a rented machine runs does not. Moved here from this file's tests on
/// 2026-09-19, unchanged.
#[cfg(any(test, feature = "testing"))]
pub mod testing {
    use super::*;

    pub fn a_job() -> Identity {
        Identity::issue(IdentityKind::Job).unwrap()
    }

    pub fn a_group() -> Identity {
        Identity::issue(IdentityKind::Group).unwrap()
    }

    pub fn an_operation() -> Identity {
        Identity::issue(IdentityKind::Operation).unwrap()
    }

    /// A line of `count` waiting jobs, keys ascending in submission order.
    pub fn a_line_of(count: usize) -> Vec<JobRecord> {
        let mut records: Vec<JobRecord> = Vec::new();
        for at in 0..count {
            let keys = the_keys_in_the_line(&records);
            records.push(JobRecord::submitted(
                a_job(),
                None,
                format!("/ephemeral/smoozy/records/job-{at}"),
                &keys,
            ));
        }
        records
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    pub(crate) use super::testing::*;

    fn order_of(rewrite: &Rewrite) -> Vec<Identity> {
        let mut keys = rewrite.keys.clone();
        keys.sort_by_key(|(_, key)| *key);
        keys.into_iter().map(|(job, _)| job).collect()
    }

    #[test]
    fn a_new_job_joins_the_end_and_a_held_one_holds_no_key() {
        let records = a_line_of(3);
        let line = the_line(&records);
        assert_eq!(line.len(), 3);
        assert!(line
            .windows(2)
            .all(|pair| pair[0].sequence_key < pair[1].sequence_key));
        let held = JobRecord::held_from_submission(
            a_job(),
            None,
            "/ephemeral/smoozy/records/held".to_string(),
            an_operation(),
        );
        assert_eq!(held.sequence_key, None);
        let mut with_held = records.clone();
        with_held.push(held);
        assert_eq!(
            the_line(&with_held).len(),
            3,
            "a held record is not in the line"
        );
    }

    #[test]
    fn a_key_may_recur_once_the_record_holding_it_is_gone() {
        // Q7 binds the live line and not the machine's whole history.
        let records = a_line_of(2);
        let top = SequenceKey::greater_than_every_key_in(&the_keys_in_the_line(&records));
        let emptied: Vec<JobRecord> = Vec::new();
        assert_eq!(
            SequenceKey::greater_than_every_key_in(&the_keys_in_the_line(&emptied)),
            SequenceKey(1)
        );
        assert!(top > SequenceKey(1));
    }

    #[test]
    fn a_released_job_enters_behind_whatever_is_waiting_at_the_release() {
        let records = a_line_of(2);
        let mut held = JobRecord::held_from_submission(
            a_job(),
            None,
            "/ephemeral/smoozy/records/held".to_string(),
            an_operation(),
        );
        the_released_placement(&mut held, &records);
        assert_eq!(held.state, QueueState::Waiting);
        let every_key = the_keys_in_the_line(&records);
        assert!(every_key.iter().all(|key| Some(*key) < held.sequence_key));
    }

    #[test]
    fn a_reorder_moves_one_job_after_another_or_to_the_front_and_names_no_index() {
        let records = a_line_of(4);
        let jobs: Vec<Identity> = records.iter().map(|record| record.job.clone()).collect();
        let moved = reorder(&records, &jobs[3], &Position::AfterJob(jobs[0].clone()));
        assert_eq!(
            order_of(&moved),
            vec![
                jobs[0].clone(),
                jobs[3].clone(),
                jobs[1].clone(),
                jobs[2].clone()
            ]
        );
        assert!(moved.not_moved.is_empty());
        let front = reorder(&records, &jobs[2], &Position::AtTheFrontOfTheWaitingLine);
        assert_eq!(order_of(&front)[0], jobs[2]);
    }

    #[test]
    fn a_job_no_longer_waiting_is_not_moved_and_the_outcome_says_why() {
        let mut records = a_line_of(3);
        let taken = records[0].job.clone();
        records[0].state = QueueState::Rendering {
            operation: an_operation(),
        };
        let rewrite = reorder(&records, &taken, &Position::AtTheFrontOfTheWaitingLine);
        assert_eq!(
            rewrite.keys.len(),
            2,
            "the rendering job is not in the rewrite"
        );
        assert!(matches!(
            rewrite.not_moved.as_slice(),
            [NotMoved::ItIsNoLongerWaiting { job, now: QueueState::Rendering { .. } }] if job == &taken
        ));
    }

    /// **Q7 and D3, both of which numbering from one quietly broke.**
    ///
    /// **The rewrite is partial across records**, so the old keys and the new
    /// ones stand together while it lands. Numbering from one made a moved
    /// record take a key a record that had not moved yet was still holding —
    /// two live records with one key, which Q7 forbids — and gave a fresh key
    /// below the interrupted record's, which D3 rests on being the lowest.
    #[test]
    fn a_rewrite_numbers_from_above_every_key_a_live_record_holds() {
        let mut records = a_line_of(3);
        // A record rendering, with the lowest key: D3's interrupted job.
        records[0].state = QueueState::Rendering {
            operation: an_operation(),
        };
        let jobs: Vec<Identity> = records.iter().map(|record| record.job.clone()).collect();
        let rewrite = reorder(&records, &jobs[2], &Position::AtTheFrontOfTheWaitingLine);
        let highest_live = records
            .iter()
            .filter(|record| record.is_live())
            .filter_map(|record| record.sequence_key)
            .max()
            .expect("a live key");
        assert!(
            rewrite.keys.iter().all(|(_, key)| *key > highest_live),
            "every fresh key is above every key a live record holds"
        );
        // **D3: the interrupted record still holds the lowest live key.**
        assert!(rewrite
            .keys
            .iter()
            .all(|(_, key)| Some(*key) > records[0].sequence_key));
    }

    #[test]
    fn a_predecessor_no_longer_waiting_resolves_to_the_front() {
        let mut records = a_line_of(3);
        let gone = records[0].job.clone();
        records[0].state = QueueState::Ended {
            outcome: an_operation(),
        };
        let last = records[2].job.clone();
        let rewrite = reorder(&records, &last, &Position::AfterJob(gone));
        assert_eq!(order_of(&rewrite)[0], last);
    }

    #[test]
    fn a_held_job_is_never_moved_because_it_has_no_place_to_move() {
        let mut records = a_line_of(2);
        let held = JobRecord::held_from_submission(
            a_job(),
            None,
            "/ephemeral/smoozy/records/held".to_string(),
            an_operation(),
        );
        let held_job = held.job.clone();
        records.push(held);
        let rewrite = reorder(&records, &held_job, &Position::AtTheFrontOfTheWaitingLine);
        assert_eq!(
            rewrite.not_moved,
            vec![NotMoved::ItIsHeld { job: held_job }]
        );
        assert_eq!(rewrite.keys.len(), 2);
    }

    #[test]
    fn a_block_move_gathers_the_waiting_members_in_their_order_into_one_contiguous_block() {
        let group = a_group();
        let mut records = a_line_of(5);
        // Members scattered at positions 1 and 3.
        records[1].group = Some(group.clone());
        records[3].group = Some(group.clone());
        let jobs: Vec<Identity> = records.iter().map(|record| record.job.clone()).collect();
        let rewrite = block_move(&records, &group, &Position::AtTheFrontOfTheWaitingLine);
        assert_eq!(
            order_of(&rewrite),
            vec![
                jobs[1].clone(),
                jobs[3].clone(),
                jobs[0].clone(),
                jobs[2].clone(),
                jobs[4].clone()
            ]
        );
        assert!(rewrite.not_moved.is_empty());
        // And after a named job, keeping the block whole.
        let after = block_move(&records, &group, &Position::AfterJob(jobs[4].clone()));
        assert_eq!(
            order_of(&after),
            vec![
                jobs[0].clone(),
                jobs[2].clone(),
                jobs[4].clone(),
                jobs[1].clone(),
                jobs[3].clone()
            ]
        );
    }

    #[test]
    fn a_rendering_member_stays_and_is_named_and_an_ended_member_is_simply_absent() {
        let group = a_group();
        let mut records = a_line_of(4);
        for record in records.iter_mut().take(3) {
            record.group = Some(group.clone());
        }
        let rendering = records[0].job.clone();
        records[0].state = QueueState::Rendering {
            operation: an_operation(),
        };
        records[1].state = QueueState::Ended {
            outcome: an_operation(),
        };
        let rewrite = block_move(&records, &group, &Position::AtTheFrontOfTheWaitingLine);
        assert_eq!(rewrite.keys.len(), 2);
        assert_eq!(order_of(&rewrite)[0], records[2].job);
        assert!(matches!(
            rewrite.not_moved.as_slice(),
            [NotMoved::ItIsNoLongerWaiting { job, .. }] if job == &rendering
        ));
    }

    #[test]
    fn membership_is_read_off_the_members_and_a_group_needs_no_store() {
        let group = a_group();
        let mut records = a_line_of(3);
        records[0].group = Some(group.clone());
        records[2].group = Some(group.clone());
        assert_eq!(the_members_of(&records, &group).len(), 2);
        // L20: drawn from the local record's copy, machine or no machine.
        let local: Vec<(Identity, Option<Identity>, &str)> = vec![
            (a_job(), Some(group.clone()), "row 1"),
            (a_job(), None, "row 2"),
            (a_job(), Some(group.clone()), "row 3"),
        ];
        let grouped = group_the_rows(&local);
        assert_eq!(grouped.len(), 2);
        assert_eq!(grouped[0].0, Some(&group));
        assert_eq!(grouped[0].1, vec![&"row 1", &"row 3"]);
    }

    #[test]
    fn the_mark_lands_on_the_covered_live_records_and_on_no_ended_one() {
        let mut records = a_line_of(3);
        records[2].state = QueueState::Ended {
            outcome: an_operation(),
        };
        let covered = vec![records[0].job.clone(), records[2].job.clone()];
        mark_the_removed_build(&mut records, &covered, &an_operation(), "4.5.12");
        assert!(records[0].removed_build.is_some());
        assert!(records[1].removed_build.is_none());
        assert!(
            records[2].removed_build.is_none(),
            "an ended record is not live"
        );
        // Not a queue state: the set does not move.
        assert_eq!(records[0].state, QueueState::Waiting);
    }

    #[test]
    fn the_admission_point_drops_each_marked_record_it_reaches_until_a_startable_one() {
        let mut records = a_line_of(4);
        let removal = an_operation();
        let covered = vec![records[0].job.clone(), records[1].job.clone()];
        mark_the_removed_build(&mut records, &covered, &removal, "4.5.12");
        let act = the_admission_point(&records, |_| "Kitchen".to_string());
        assert_eq!(act.dropped.len(), 2);
        assert_eq!(act.dropped[0].job, records[0].job);
        assert_eq!(
            act.dropped[0].the_answered_version_the_job_was_submitted_with,
            "4.5.12"
        );
        assert_eq!(act.started, Some(records[2].job.clone()));
    }

    #[test]
    fn a_line_of_nothing_but_marked_records_drops_them_all_and_starts_nothing() {
        let mut records = a_line_of(2);
        let covered: Vec<Identity> = records.iter().map(|record| record.job.clone()).collect();
        mark_the_removed_build(&mut records, &covered, &an_operation(), "5.0.3");
        let act = the_admission_point(&records, |_| String::new());
        assert_eq!(act.dropped.len(), 2);
        assert_eq!(act.started, None);
    }

    #[test]
    fn the_mark_fires_on_a_waiting_record_and_on_no_other() {
        let mut records = a_line_of(2);
        let covered = vec![records[0].job.clone()];
        mark_the_removed_build(&mut records, &covered, &an_operation(), "5.0.3");
        // Marked while rendering: never returns to waiting, never reached.
        records[0].state = QueueState::Rendering {
            operation: an_operation(),
        };
        let act = the_admission_point(&records, |_| String::new());
        assert!(act.dropped.is_empty());
        assert_eq!(act.started, Some(records[1].job.clone()));
    }

    #[test]
    fn the_live_set_is_waiting_held_and_rendering_and_never_ended() {
        let mut records = a_line_of(3);
        records[1].state = QueueState::Ended {
            outcome: an_operation(),
        };
        records[2].state = QueueState::Held {
            hold: an_operation(),
        };
        let live = the_live_set(&records);
        assert_eq!(live.len(), 2);
    }
}
