//! **The hold: publication waits while anything is running** — Z31, on the
//! Creator's ruling of 2026-09-28: *"We need to have Smoozy not update on its
//! own while jobs are running. The user should be able to override this block,
//! but by default — updating only happens with no running jobs."* Built at step
//! 11's Mac update piece. Z32's *install anyway* is the one route past it, and
//! it is the caller's (`crate::updating::the_mac`), not this file's.
//!
//! **What *running* is, read on this computer alone, at the moment publication
//! would begin** — Z31's first bullet, item by item:
//!
//! 1. **Every local job record that records no ending**, whatever its standing:
//!    unconfirmed, waiting in its machine's line, rendering, or held on the
//!    user's answer. **Waiting counts.** The ending is `Queue & History`'s own
//!    reading of the record (`LocalJobRecord::the_ending_learned`), never a
//!    second one here.
//! 2. **The record and not a queue reading**: the job record is durable and
//!    outlives the connected period, so at application start, with nothing
//!    connected, it alone still says what is running. A record on a server
//!    with no live connection counts until its ending is learned — silence is
//!    never read as an ending. **The one record with no ending that does not
//!    count is one marked *start never heard*** (`Queue & History` C9's second
//!    way in), whose server is gone and which nothing could ever end.
//! 3. **Every transfer under way, in either direction, manual or
//!    auto-download, except the update's own fetch.** Read from what this run
//!    is moving: a channel carrying bytes in either direction, a request under
//!    way between its files, and a download still being counted before it is
//!    written down. **The update's own fetch holds no channel at the moment
//!    this is read**: the fetch, the check and this reading are one sequence on
//!    one thread, and the fetch's channel is ended before anything is
//!    verified. **A setup run is not counted** (Z31: it is not a job).
//!
//! **Nothing is read from a machine**: no connection is opened, no reading is
//! refreshed, and a server that cannot be reached holds the update back exactly
//! as its last-known entries say.
//!
//! **A record that cannot be read holds the update** — the safe side of the
//! Creator's words, and the only reading that cannot publish over a job it
//! failed to see.
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this
//! crate's own plan, `Preparation/Architecture/Desktop app and delivery.md`.
//! Every other document's marker is written with that document at the site.

use std::collections::HashSet;

use smoozy_server_protocol::transfers::Direction;
use smoozy_state::identities::{Identity, IdentityKind};

use crate::downloading::TheRunsTransfers;
use crate::record::job_records::JobRecords;
use crate::record::{server_rows, RecordFailure};

/// **What this run is moving now**, as the hold asks it — Z31's third item.
pub trait TellsWhatIsMoving {
    /// How many transfers are under way, counted however this run counts them;
    /// **nought means nothing is moving**.
    fn transfers_under_way(&self) -> usize;
}

impl TellsWhatIsMoving for TheRunsTransfers {
    fn transfers_under_way(&self) -> usize {
        // **A channel carrying bytes, in either direction, of any class.** The
        // admission point publishes a reading only while a channel is running
        // (`Server protocol and transfers` R12a), so a reading is a channel.
        let channels = [Direction::Up, Direction::Down]
            .into_iter()
            .filter(|direction| self.the_reading(*direction).is_some())
            .count();
        // **A request under way between its files**, and **a download still
        // being counted** before `Download` M1 writes it down.
        channels + self.what_is_under_way().len() + self.what_is_being_counted().len()
    }
}

/// **What the hold read**: the jobs with no ending, and how many transfers
/// were moving.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WhatIsRunning {
    pub jobs: Vec<Identity>,
    pub transfers: usize,
}

impl WhatIsRunning {
    /// **Whether nothing is running** — the one condition under which the hold
    /// lifts (Z31's third bullet).
    pub fn nothing(&self) -> bool {
        self.jobs.is_empty() && self.transfers == 0
    }
}

/// **Every job record with no ending, on every server**, a *start never
/// heard* record excepted.
///
/// Two readings, joined: each server row's records with no learned ending
/// (which leaves out *start never heard* already), and the history records
/// with no ending — **the second is what reaches a job whose server row is
/// gone** after its render start was learned. That record is not marked *start
/// never heard* (the mark is for a start never learned), so Z31's words count
/// it, and it holds the update until Z32 is used. Each job is counted once.
pub fn the_jobs_with_no_ending(connection: &rusqlite::Connection) -> Result<Vec<Identity>, RecordFailure> {
    let records = JobRecords::over(connection);
    let mut seen = HashSet::new();
    let mut jobs = Vec::new();
    let mut keep = |job: Identity, jobs: &mut Vec<Identity>| {
        if seen.insert(job.as_text()) {
            jobs.push(job);
        }
    };
    for row in server_rows::all(connection)? {
        let Some(server) = Identity::read_back(IdentityKind::LocalServer, &row.local_server_identifier) else {
            continue;
        };
        for record in records.with_no_learned_ending_for(&server)? {
            keep(record.job, &mut jobs);
        }
    }
    for entry in records.in_history()?.entries {
        if entry.start_never_heard || entry.record.the_ending_learned().is_some() {
            continue;
        }
        keep(entry.record.job, &mut jobs);
    }
    Ok(jobs)
}

/// **Z31's reading, whole, at the moment it is asked.**
pub fn what_is_running(
    connection: &rusqlite::Connection,
    moving: &dyn TellsWhatIsMoving,
) -> Result<WhatIsRunning, RecordFailure> {
    Ok(WhatIsRunning {
        jobs: the_jobs_with_no_ending(connection)?,
        transfers: moving.transfers_under_way(),
    })
}

/// **Whether publication is held now** — yes where anything is running, and
/// yes where the record would not say (the safe side).
pub fn publication_is_held(connection: &rusqlite::Connection, moving: &dyn TellsWhatIsMoving) -> bool {
    match what_is_running(connection, moving) {
        Ok(running) => !running.nothing(),
        Err(_) => true,
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::record::deletions::tests::{a_record_in_history, a_record_store};
    use smoozy_queue_history::acting::note::{TheEndingOnTheNote, TheOutcomeAsWritten};
    use smoozy_queue_history::record::Standing;

    /// A run moving a set number of transfers.
    pub(crate) struct Moving(pub usize);

    impl TellsWhatIsMoving for Moving {
        fn transfers_under_way(&self) -> usize {
            self.0
        }
    }

    /// One server row, saved, with the identity it is keyed on.
    pub(crate) fn a_server_row(connection: &rusqlite::Connection) -> Identity {
        use smoozy_servers::identity::{Host, KeyReference, Port, Username};
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let row = smoozy_servers::store::ServerRow::saved(
            server.as_text(),
            Host::what_the_user_typed("5.180.183.77").unwrap(),
            Port::what_the_user_typed(22).unwrap(),
            Username::what_the_user_typed("root").unwrap(),
            KeyReference {
                local_key_identifier: "key-1".into(),
                fingerprint: "SHA256:aaa".into(),
            },
        );
        server_rows::add(connection, &row).unwrap();
        server
    }

    fn ended(mut record: smoozy_queue_history::record::LocalJobRecord) -> smoozy_queue_history::record::LocalJobRecord {
        record.what_the_note_carried.ending = Some(TheEndingOnTheNote {
            outcome: TheOutcomeAsWritten::Finished,
            exit_status: Some(0),
            the_attempt_whose_capture_record_holds_the_words: 1,
            the_reason_the_machine_recorded: None,
            the_device_line_as_observed: None,
        });
        record
    }

    /// **Z31: held while a job runs, and only while one does.** A rendering
    /// job, a job still waiting in its machine's line, and a job never
    /// confirmed each hold it; a job whose ending was learned does not; a job
    /// marked *start never heard* does not; a started job whose server row is
    /// gone still does.
    #[test]
    fn publication_is_held_while_a_job_has_no_ending() {
        let connection = a_record_store();
        let store = JobRecords::over(&connection);
        let server = a_server_row(&connection);
        assert!(!publication_is_held(&connection, &Moving(0)), "nothing at all holds it");

        let finished = ended(a_record_in_history(&server, "Kitchen — finished"));
        store.write_the_provisional_record(&finished).unwrap();
        assert!(!publication_is_held(&connection, &Moving(0)), "a finished job held it");

        let rendering = a_record_in_history(&server, "Kitchen — rendering");
        store.write_the_provisional_record(&rendering).unwrap();
        let running = what_is_running(&connection, &Moving(0)).unwrap();
        assert_eq!(running.jobs, vec![rendering.job.clone()]);
        assert!(publication_is_held(&connection, &Moving(0)));
        store.remove(&rendering.job).unwrap();

        let mut waiting = a_record_in_history(&server, "Kitchen — waiting in line");
        waiting.the_render_start_has_been_learned = false;
        store.write_the_provisional_record(&waiting).unwrap();
        assert!(publication_is_held(&connection, &Moving(0)), "a job waiting in line did not hold it");
        store.remove(&waiting.job).unwrap();

        let mut unconfirmed = a_record_in_history(&server, "Kitchen — just sent");
        unconfirmed.the_render_start_has_been_learned = false;
        unconfirmed.standing = Standing::Provisional;
        store.write_the_provisional_record(&unconfirmed).unwrap();
        assert!(publication_is_held(&connection, &Moving(0)), "a job just sent did not hold it");
        // **Its server is gone**: the record is marked *start never heard*,
        // and nothing could end it, so it no longer holds.
        store.the_server_is_gone(&server).unwrap();
        assert!(!publication_is_held(&connection, &Moving(0)), "a start-never-heard record held it");

        // **A started job on a server row that is gone** still counts.
        let gone = Identity::issue(IdentityKind::LocalServer).unwrap();
        let orphan = a_record_in_history(&gone, "Kitchen — on a removed server");
        store.write_the_provisional_record(&orphan).unwrap();
        assert_eq!(what_is_running(&connection, &Moving(0)).unwrap().jobs, vec![orphan.job.clone()]);
    }

    /// **Z31: held while a transfer runs**, in either direction — and a
    /// record that will not answer holds it too.
    #[test]
    fn publication_is_held_while_a_transfer_runs_and_where_the_record_will_not_say() {
        let connection = a_record_store();
        assert!(!publication_is_held(&connection, &Moving(0)));
        assert!(publication_is_held(&connection, &Moving(1)), "a transfer under way did not hold it");
        let running = what_is_running(&connection, &Moving(2)).unwrap();
        assert_eq!(running.transfers, 2);
        assert!(!running.nothing());

        // The run's own reading: a channel carrying bytes is a transfer.
        let run = TheRunsTransfers::for_this_run();
        assert_eq!(run.transfers_under_way(), 0);
        let manual = smoozy_file_browser::pool::THE_DECLARED_ORDER[1].name;
        let channel = run.admit_to_carry(Direction::Down, &[manual], manual).expect("a channel");
        assert_eq!(run.transfers_under_way(), 1, "a download's channel is not a transfer under way");
        drop(channel);
        assert_eq!(run.transfers_under_way(), 0);

        connection.execute_batch("DROP TABLE job_record").unwrap();
        assert!(publication_is_held(&connection, &Moving(0)), "an unreadable record let it go");
    }
}
