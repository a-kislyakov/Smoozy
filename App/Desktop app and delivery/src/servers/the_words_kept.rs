//! **`Queue & History` E9: the history record takes the words in at
//! reconciliation, and from then on they survive the machine** — the Creator's
//! notes on 0.1.8, piece 6d (2026-10-03).
//!
//! **What is fetched, and when.** E9: *"on a connection that learns a job's
//! ending, the desktop fetches both captured streams and the per-stream
//! collection outcomes into the job's history record, whole or not at all
//! (M11), kept apart, unedited"* — **every attempt of the job that carries a
//! capture record, never the ending attempt alone.** The fill-in that learns
//! the ending (`State, identity and recovery` V1 step 4, at §7 step 3's read
//! forward or step 4 itself, on the reconciliation route or P10d's) writes the
//! debt in the same act ([`crate::record::job_records::JobRecords::the_fill_in`]);
//! this pass pays it, on a thread of its own beside step 7 and beside the pass
//! that learned the ending, and never in front of either.
//!
//! **The built half is wired, not written again.**
//! `smoozy_queue_history::acting::words::fetch_the_words` is the fetch — every
//! attempt, each pair whole or not at all, partial across attempts — and this
//! file supplies the one thing it reaches through a trait: the machine,
//! [`TheSinksTheMachineNamed`], which reads each sink at the path and to the
//! length the machine itself named on the note reading's answer. **The desktop
//! composes no machine path** (`Queue & History` E8's rule).
//!
//! **No third route** (`Desktop app and delivery` T18, `Queue & History` N9):
//! the capture records ride the note reading the fill-in already takes, and
//! the words come over the reading session every other captured stream comes
//! back on. **Nothing is posted** from here: what landed is drawn the next
//! time History is drawn, which this asks for and nothing more.
//!
//! **A fetch that did not complete lands nothing for that attempt and the debt
//! stands**, so the next connection to the machine tries again — once per
//! connection, never in a loop. **A machine that no longer holds the job**
//! settles the debt with nothing, which is E9's own boundary: *"the words are
//! gone with it"*. **A program older than this piece sends no capture records**
//! and the debt simply stands.
//!
//! **Markers of other documents are written out with that document at every
//! site.** A bare marker means a clause of `Preparation/Architecture/Desktop app
//! and delivery.md`.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, OnceLock};

use tauri::{AppHandle, Manager, Runtime};

use smoozy_queue_history::acting::words::{
    fetch_the_words, the_attempts_with_both_outcomes, the_capture_records_read_from,
    ACaptureRecordAsCarried, ReadsTheSinkOverTheConnection, TheLiveCommentary,
};
use smoozy_queue_history::readings::WhichStream;
use smoozy_server_protocol::framing::Fields;
use smoozy_server_protocol::requests::{
    IT_IS_ABSENT, IT_IS_PRESENT, ON_THIS_MACHINE_FIELD, THE_JOB_FIELD,
    THE_PER_JOB_NOTE_READING_REQUEST,
};
use smoozy_state::identities::Identity;

use crate::keys::LendsTheRecord;
use crate::presentation::the_machines_own_words::Stream;
use crate::record::kept_words::{self, AStreamKept};
use crate::servers::a_period::ThePeriodsThisRun;
use crate::servers::sessions::ThisServersSessions;
use crate::servers::the_machines_work::{
    a_request, the_answer_to, with_the_record, AsksTheSupervisor, OverThisServersPoint,
};
use crate::start::Started;

/// **One captured stream read back whole, to the length the machine named** —
/// `Rented-server runtime` M11: *"a read of the retained prefix that did not
/// complete is not a reading."* **A short read is an error in words**, never a
/// short record.
pub trait ReadsAStreamBackWhole {
    fn whole(&self, path: &str, length: u64) -> Result<Vec<u8>, String>;
}

/// **The machine, as E9's fetch reaches it**: the sinks its own answer named,
/// read whole.
pub struct TheSinksTheMachineNamed<'a> {
    pub records: &'a [ACaptureRecordAsCarried],
    pub reads: &'a dyn ReadsAStreamBackWhole,
}

impl ReadsTheSinkOverTheConnection for TheSinksTheMachineNamed<'_> {
    fn the_sinks_current_bytes(
        &self,
        _job: &Identity,
        attempt: u32,
        stream: WhichStream,
    ) -> Result<TheLiveCommentary, String> {
        let record = self
            .records
            .iter()
            .find(|record| record.attempt == attempt)
            .ok_or_else(|| format!("the machine named no capture record for attempt {attempt}"))?;
        let sink = match stream {
            WhichStream::TheOrdinaryOutput => &record.sinks.ordinary,
            WhichStream::TheComplaints => &record.sinks.complaints,
        };
        // **A sink whose length the machine could not say cannot be read
        // whole**, so it is not read at all.
        let length = sink
            .length
            .ok_or_else(|| format!("{}: the machine could not say how long it is", sink.path))?;
        let bytes = if length == 0 {
            Vec::new()
        } else {
            self.reads.whole(&sink.path, length)?
        };
        Ok(TheLiveCommentary {
            attempt,
            stream,
            bytes,
        })
    }

    fn both_streams_of(
        &self,
        job: &Identity,
        attempt: u32,
    ) -> Result<(TheLiveCommentary, TheLiveCommentary), String> {
        Ok((
            self.the_sinks_current_bytes(job, attempt, WhichStream::TheOrdinaryOutput)?,
            self.the_sinks_current_bytes(job, attempt, WhichStream::TheComplaints)?,
        ))
    }
}

/// **What one pass did**, for a test and for whoever draws History again.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct WhatThePassDid {
    /// Jobs whose words landed, whole or in part.
    pub landed: usize,
    /// Jobs settled with nothing: the machine no longer holds them, or they
    /// carry no capture record.
    pub settled_with_nothing: usize,
    /// Jobs whose debt still stands after this pass.
    pub still_owed: usize,
}

fn the_record_of(record: &ACaptureRecordAsCarried, stream: Stream) -> &smoozy_queue_history::acting::words::WhatAStreamCameTo {
    match stream {
        Stream::TheOrdinaryOutput => &record.ordinary,
        Stream::TheComplaints => &record.complaints,
    }
}

fn which(stream: Stream) -> WhichStream {
    match stream {
        Stream::TheOrdinaryOutput => WhichStream::TheOrdinaryOutput,
        Stream::TheComplaints => WhichStream::TheComplaints,
    }
}

fn a_stream_kept(record: &ACaptureRecordAsCarried, stream: Stream, the_words: Vec<u8>) -> AStreamKept {
    let came_to = the_record_of(record, stream);
    AStreamKept {
        attempt: record.attempt,
        stream,
        outcome: came_to.outcome,
        the_words,
        retained: came_to.retained,
        discarded: came_to.discarded,
        why_the_rest_went: came_to.first_loss.clone(),
    }
}

/// **E9, paid for one server**: every job of it whose words are owed and not
/// in `passed_over`, asked about once.
///
/// **Per job**: the note reading's answer names the capture records; the
/// attempts whose two outcomes are written are handed to
/// [`fetch_the_words`]; an attempt with an outcome not yet written lands as
/// that (M10a) with no words, and keeps the debt standing; every attempt
/// already kept with its outcomes is not fetched again.
pub fn the_words_owed_are_taken_in(
    server: &Identity,
    record: &dyn LendsTheRecord,
    machine: &mut dyn AsksTheSupervisor,
    reads: &dyn ReadsAStreamBackWhole,
    passed_over: &HashSet<String>,
) -> (WhatThePassDid, Vec<Identity>) {
    let mut did = WhatThePassDid::default();
    let owed = with_the_record(record, |connection| {
        kept_words::the_jobs_owed_their_words(connection, server).unwrap_or_default()
    })
    .unwrap_or_default();
    let mut asked = Vec::new();
    for job in owed
        .into_iter()
        .filter(|job| !passed_over.contains(&job.as_text()))
    {
        asked.push(job.clone());
        let answer = the_answer_to(
            machine,
            a_request(
                THE_PER_JOB_NOTE_READING_REQUEST,
                Fields::none().and(THE_JOB_FIELD, job.as_text()),
            ),
        );
        let Some(answer) = answer else {
            did.still_owed += 1;
            continue;
        };
        match answer.text(ON_THIS_MACHINE_FIELD) {
            // **E9's boundary: the machine no longer holds the job, so its
            // words are gone with it** — the debt is settled with nothing.
            Some(IT_IS_ABSENT) => {
                let _ = with_the_record(record, |connection| {
                    kept_words::the_words_land(connection, &job, &[], true)
                });
                did.settled_with_nothing += 1;
                continue;
            }
            Some(IT_IS_PRESENT) => {}
            _ => {
                did.still_owed += 1;
                continue;
            }
        }
        let Some(records) = the_capture_records_read_from(&answer) else {
            did.still_owed += 1;
            continue;
        };
        if records.is_empty() {
            let _ = with_the_record(record, |connection| {
                kept_words::the_words_land(connection, &job, &[], true)
            });
            did.settled_with_nothing += 1;
            continue;
        }
        let already = with_the_record(record, |connection| {
            kept_words::the_words_kept_for(connection, &job).unwrap_or_default()
        })
        .unwrap_or_default();
        let is_kept_whole = |attempt: u32| {
            Stream::BOTH.iter().all(|stream| {
                already.iter().any(|one| {
                    one.attempt == attempt && one.stream == *stream && one.outcome.is_some()
                })
            })
        };
        let to_fetch: Vec<&ACaptureRecordAsCarried> =
            records.iter().filter(|one| !is_kept_whole(one.attempt)).collect();
        let the_machine = TheSinksTheMachineNamed {
            records: &records,
            reads,
        };

        let mut arrived = Vec::new();
        let mut nothing_more_is_owed = true;
        // **The attempts whose two outcomes are written: E9's fetch itself**,
        // each pair whole or not at all.
        let both: Vec<_> = the_attempts_with_both_outcomes(&records)
            .into_iter()
            .filter(|(attempt, _, _)| !is_kept_whole(*attempt))
            .collect();
        let left = fetch_the_words(&the_machine, &job, &both);
        for fetched in left.fetched {
            let Some(carried) = records.iter().find(|one| one.attempt == fetched.words.attempt) else {
                continue;
            };
            arrived.push(a_stream_kept(carried, Stream::TheOrdinaryOutput, fetched.ordinary_output));
            arrived.push(a_stream_kept(carried, Stream::TheComplaints, fetched.complaints));
        }
        if !left.could_not_be_fetched.is_empty() {
            nothing_more_is_owed = false;
        }
        // **An attempt with an outcome not yet written** (M10a): the stream
        // that has one is read whole as E9's fetch would, the one that has
        // none lands as that, and the debt stands until both are written.
        for carried in to_fetch
            .into_iter()
            .filter(|one| one.ordinary.outcome.is_none() || one.complaints.outcome.is_none())
        {
            nothing_more_is_owed = false;
            let mut this_attempt = Vec::new();
            let mut whole = true;
            for stream in Stream::BOTH {
                if the_record_of(carried, stream).outcome.is_none() {
                    this_attempt.push(a_stream_kept(carried, stream, Vec::new()));
                    continue;
                }
                match the_machine.the_sinks_current_bytes(&job, carried.attempt, which(stream)) {
                    Ok(read) => this_attempt.push(a_stream_kept(carried, stream, read.bytes)),
                    Err(_) => whole = false,
                }
            }
            if whole {
                arrived.extend(this_attempt);
            }
        }
        let landed = with_the_record(record, |connection| {
            kept_words::the_words_land(connection, &job, &arrived, nothing_more_is_owed)
        });
        if !arrived.is_empty() && matches!(landed, Some(Ok(kept_words::WhatTheLandingDid::ItLanded))) {
            did.landed += 1;
        }
        if !nothing_more_is_owed || !matches!(landed, Some(Ok(_))) {
            did.still_owed += 1;
        }
    }
    (did, asked)
}

// ════════════════════════════════════════════════════════════════════════════
// On the running application
// ════════════════════════════════════════════════════════════════════════════

/// **Each server's own state for this run**: whether a pass is in hand, and
/// which jobs were asked about on which connection. **A job asked about on a
/// connection is not asked about again on that connection** — the debt stands
/// for the next one — so a machine that cannot answer costs one ask per
/// connection, never one per pass.
#[derive(Default)]
struct ThisServersPasses {
    in_hand: bool,
    connection: u64,
    asked: HashSet<String>,
}

fn the_passes_this_run() -> &'static Mutex<HashMap<String, ThisServersPasses>> {
    static HELD: OnceLock<Mutex<HashMap<String, ThisServersPasses>>> = OnceLock::new();
    HELD.get_or_init(Default::default)
}

/// **The reading session, as `Queue & History` E9's fetch reads a sink whole** — the same
/// read-back every captured stream comes home by, waited on from this pass's
/// own thread.
struct OverTheReadingSession {
    sessions: ThisServersSessions,
}

impl ReadsAStreamBackWhole for OverTheReadingSession {
    fn whole(&self, path: &str, length: u64) -> Result<Vec<u8>, String> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|why| why.to_string())?;
        let path = path.as_bytes().to_vec();
        // **Nobody is waiting on it** (`Server protocol and transfers` §1.1
        // item 2): a reading the user waits on goes first.
        let read = runtime.block_on(self.sessions.a_reading(false, move |subsystem| {
            Box::pin(async move {
                Ok(
                    smoozy_dependencies::acting::checking::read_a_stream_back_over_the_file_session(
                        subsystem, &path, length,
                    )
                    .await,
                )
            })
        }));
        match read {
            Ok(Ok(read)) => read,
            Ok(Err(failure)) => Err(format!("{failure:?}")),
            Err(not_lent) => Err(format!("{not_lent:?}")),
        }
    }
}

/// **Started beside the act that may have learned an ending** — `Servers` Q1
/// step 7, and every P10d read forward that applied something. Returns at
/// once; **a pass already in hand for this server is not doubled**, and it
/// looks again before it ends, so a debt written meanwhile is not left for
/// the next connection.
pub fn are_taken_in_beside<R: Runtime>(
    app: &AppHandle<R>,
    server: &Identity,
    connection: u64,
    sessions: &ThisServersSessions,
    package_area: &str,
) {
    let server_text = server.as_text();
    {
        let Ok(mut passes) = the_passes_this_run().lock() else {
            return;
        };
        let this = passes.entry(server_text.clone()).or_default();
        if this.connection != connection {
            this.connection = connection;
            this.asked.clear();
        }
        if this.in_hand {
            return;
        }
        this.in_hand = true;
    }
    let (app, server, sessions, package_area) =
        (app.clone(), server.clone(), sessions.clone(), package_area.to_string());
    let spawned = std::thread::Builder::new()
        .name("smoozy kept words".to_string())
        .spawn(move || {
            the_passes(&app, &server, connection, sessions, package_area);
            if let Ok(mut passes) = the_passes_this_run().lock() {
                if let Some(this) = passes.get_mut(&server.as_text()) {
                    this.in_hand = false;
                }
            }
        });
    if spawned.is_err() {
        if let Ok(mut passes) = the_passes_this_run().lock() {
            if let Some(this) = passes.get_mut(&server_text) {
                this.in_hand = false;
            }
        }
    }
}

fn the_passes<R: Runtime>(
    app: &AppHandle<R>,
    server: &Identity,
    connection: u64,
    sessions: ThisServersSessions,
    package_area: String,
) {
    let (Some(periods), Some(started)) = (
        app.try_state::<Arc<ThePeriodsThisRun>>(),
        app.try_state::<Mutex<Started>>(),
    ) else {
        return;
    };
    let record = crate::keys::TheRunsRecord {
        started: started.inner(),
    };
    let server_text = server.as_text();
    let holds = periods.the_machines_work();
    let reads = OverTheReadingSession {
        sessions: sessions.clone(),
    };
    loop {
        if !(sessions.a_connection_is_live() && holds.is_current(&server_text, connection)) {
            return;
        }
        let passed_over = the_passes_this_run()
            .lock()
            .ok()
            .and_then(|passes| passes.get(&server_text).map(|this| this.asked.clone()))
            .unwrap_or_default();
        let mut machine = OverThisServersPoint {
            sessions: sessions.clone(),
            package_area: package_area.clone(),
        };
        let (did, asked) =
            the_words_owed_are_taken_in(server, &record, &mut machine, &reads, &passed_over);
        if let Ok(mut passes) = the_passes_this_run().lock() {
            if let Some(this) = passes.get_mut(&server_text) {
                this.asked.extend(asked.iter().map(Identity::as_text));
            }
        }
        if did.landed > 0 || did.settled_with_nothing > 0 {
            crate::surface::the_history::the_history_is_told_again(app);
        }
        if asked.is_empty() {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::collections::VecDeque;

    use super::*;
    use crate::record::kept_words::tests::{a_finished_note, a_job_on, a_record_store};
    use crate::record::job_records::JobRecords;
    use smoozy_queue_history::acting::words::{the_capture_records_as_fields, WhatAStreamCameTo};
    use smoozy_queue_history::readings::{ASink, TheSinksOfARenderingJob};
    use smoozy_runtime::capture::CollectionOutcome;
    use smoozy_server_protocol::asking_the_supervisor::WhatCameBack;
    use smoozy_server_protocol::framing::{Answered, Request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS};
    use smoozy_state::identities::IdentityKind;

    /// The record, lent from a connection held for the test.
    struct ARecordInHand(Mutex<rusqlite::Connection>);

    impl LendsTheRecord for ARecordInHand {
        fn lend(&self, act: &mut dyn FnMut(&rusqlite::Connection)) {
            act(&self.0.lock().unwrap());
        }
        fn write_a_diary_line(&self, _act: &smoozy_keys::diary::Act) {}
    }

    /// A machine whose note reading answers from a script, one answer per ask.
    struct AMachineInHand {
        answers: VecDeque<Fields>,
        asked: usize,
    }

    impl AsksTheSupervisor for AMachineInHand {
        fn ask(&mut self, request: &Request) -> WhatCameBack {
            self.asked += 1;
            assert_eq!(request.name, THE_PER_JOB_NOTE_READING_REQUEST);
            match self.answers.pop_front() {
                Some(fields) => WhatCameBack::ItAnswered(
                    Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "test")
                        .carrying(fields),
                ),
                None => WhatCameBack::NothingCorrelatedCameBack,
            }
        }
    }

    /// Sinks held in memory, by path; a path named in `cut` is a read that
    /// did not complete.
    struct SinksInHand {
        held: HashMap<String, Vec<u8>>,
        cut: Vec<String>,
        reads: RefCell<Vec<String>>,
    }

    impl ReadsAStreamBackWhole for SinksInHand {
        fn whole(&self, path: &str, length: u64) -> Result<Vec<u8>, String> {
            self.reads.borrow_mut().push(path.to_string());
            if self.cut.iter().any(|cut| cut == path) {
                return Err(format!("{path}: the stream ended at 3 of the {length} bytes"));
            }
            let held = self.held.get(path).cloned().ok_or("no such file")?;
            assert_eq!(held.len() as u64, length, "read to the length the machine named");
            Ok(held)
        }
    }

    fn a_capture(attempt: u32, ordinary: &[u8], complaints: &[u8]) -> ACaptureRecordAsCarried {
        let came_to = |bytes: &[u8]| WhatAStreamCameTo {
            outcome: Some(if bytes.is_empty() {
                CollectionOutcome::CollectedAndTheStreamWasEmpty
            } else {
                CollectionOutcome::Collected
            }),
            retained: Some(bytes.len() as u64),
            discarded: Some(0),
            first_loss: None,
        };
        ACaptureRecordAsCarried {
            attempt,
            sinks: TheSinksOfARenderingJob {
                ordinary: ASink {
                    path: format!("/pkg/streams/attempt-{attempt}/ordinary"),
                    length: Some(ordinary.len() as u64),
                },
                complaints: ASink {
                    path: format!("/pkg/streams/attempt-{attempt}/complaints"),
                    length: Some(complaints.len() as u64),
                },
            },
            ordinary: came_to(ordinary),
            complaints: came_to(complaints),
        }
    }

    fn the_note_with(records: &[ACaptureRecordAsCarried]) -> Fields {
        the_capture_records_as_fields(a_finished_note().as_the_answers_fields(), records)
    }

    fn sinks_for(records: &[(&ACaptureRecordAsCarried, &[u8], &[u8])]) -> SinksInHand {
        let mut held = HashMap::new();
        for (record, ordinary, complaints) in records {
            held.insert(record.sinks.ordinary.path.clone(), ordinary.to_vec());
            held.insert(record.sinks.complaints.path.clone(), complaints.to_vec());
        }
        SinksInHand {
            held,
            cut: Vec::new(),
            reads: RefCell::new(Vec::new()),
        }
    }

    /// A job on a server whose ending the fill-in has just learned.
    fn an_ended_job() -> (ARecordInHand, Identity, Identity) {
        let connection = a_record_store();
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let job = a_job_on(&connection, &server);
        JobRecords::over(&connection)
            .the_fill_in(&job, &a_finished_note())
            .unwrap();
        (ARecordInHand(Mutex::new(connection)), server, job)
    }

    fn kept(record: &ARecordInHand, job: &Identity) -> Vec<AStreamKept> {
        with_the_record(record, |connection| kept_words::the_words_kept_for(connection, job).unwrap())
            .unwrap()
    }

    fn owed(record: &ARecordInHand, server: &Identity) -> Vec<Identity> {
        with_the_record(record, |connection| {
            kept_words::the_jobs_owed_their_words(connection, server).unwrap()
        })
        .unwrap()
    }

    /// **E9 performed: a finished job's words land in History whole** — every
    /// attempt that carries a capture record, never the ending attempt alone,
    /// each stream apart and byte for byte, each with its outcome — **and the
    /// debt is settled.** The map's test *"A finished job's words land in
    /// History whole"*.
    #[test]
    fn a_finished_jobs_words_land_whole_for_every_attempt_and_settle_the_debt() {
        let (record, server, job) = an_ended_job();
        let first = a_capture(1, b"Fra:61 Mem:9G | Sample 12/64", b"");
        let second = a_capture(2, b"Fra:240 | Saved: '/out/0240.exr'", b"a complaint\n");
        let reads = sinks_for(&[
            (&first, b"Fra:61 Mem:9G | Sample 12/64", b""),
            (&second, b"Fra:240 | Saved: '/out/0240.exr'", b"a complaint\n"),
        ]);
        let mut machine = AMachineInHand {
            answers: VecDeque::from([the_note_with(&[first.clone(), second.clone()])]),
            asked: 0,
        };
        let (did, asked) =
            the_words_owed_are_taken_in(&server, &record, &mut machine, &reads, &HashSet::new());
        assert_eq!(asked, vec![job.clone()]);
        assert_eq!(did, WhatThePassDid { landed: 1, settled_with_nothing: 0, still_owed: 0 });
        let kept = kept(&record, &job);
        assert_eq!(kept.len(), 4, "two streams of two attempts, kept apart");
        assert_eq!(kept[0].the_words, b"Fra:61 Mem:9G | Sample 12/64");
        assert_eq!(kept[1].outcome, Some(CollectionOutcome::CollectedAndTheStreamWasEmpty));
        assert_eq!(kept[3].the_words, b"a complaint\n");
        assert!(
            !reads.reads.borrow().iter().any(|path| path == &first.sinks.complaints.path),
            "an empty sink is not asked for"
        );
        assert!(owed(&record, &server).is_empty(), "the debt is settled");
    }

    /// **Whole or not at all** (`Rented-server runtime` M11): **a cut fetch
    /// lands nothing for that attempt** and the debt stands; the next
    /// connection fetches it and never fetches again what is already kept.
    /// The map's test *"a cut fetch lands nothing"*.
    #[test]
    fn a_cut_fetch_lands_nothing_for_its_attempt_and_the_next_connection_finishes_it() {
        let (record, server, job) = an_ended_job();
        let first = a_capture(1, b"the first attempt", b"");
        let second = a_capture(2, b"the second attempt", b"a complaint");
        let mut reads = sinks_for(&[
            (&first, b"the first attempt", b""),
            (&second, b"the second attempt", b"a complaint"),
        ]);
        reads.cut = vec![second.sinks.complaints.path.clone()];
        let mut machine = AMachineInHand {
            answers: VecDeque::from([the_note_with(&[first.clone(), second.clone()])]),
            asked: 0,
        };
        let (did, _) =
            the_words_owed_are_taken_in(&server, &record, &mut machine, &reads, &HashSet::new());
        assert_eq!(did.still_owed, 1);
        let after_the_cut = kept(&record, &job);
        assert_eq!(
            after_the_cut.iter().map(|one| one.attempt).collect::<Vec<_>>(),
            vec![1, 1],
            "the cut attempt landed nothing, not even its ordinary stream"
        );
        assert_eq!(owed(&record, &server), vec![job.clone()]);

        // **The next connection.**
        reads.cut.clear();
        reads.reads.borrow_mut().clear();
        let mut machine = AMachineInHand {
            answers: VecDeque::from([the_note_with(&[first.clone(), second.clone()])]),
            asked: 0,
        };
        let (did, _) =
            the_words_owed_are_taken_in(&server, &record, &mut machine, &reads, &HashSet::new());
        assert_eq!(did.still_owed, 0);
        assert_eq!(kept(&record, &job).len(), 4);
        assert!(
            !reads.reads.borrow().iter().any(|path| path.contains("attempt-1")),
            "what is already kept is not fetched again"
        );
        assert!(owed(&record, &server).is_empty());
    }

    /// **E9's boundary**: a machine that no longer holds the job settles the
    /// debt with nothing — its words are gone with it — and **a machine that
    /// cannot say, or a program that sends no capture records, leaves the
    /// debt standing**; **a job asked about on this connection is not asked
    /// about again on it.**
    #[test]
    fn a_machine_without_the_job_settles_with_nothing_and_one_that_cannot_say_leaves_the_debt() {
        use smoozy_server_protocol::requests::THE_MACHINE_CANNOT_SAY;
        let reads = sinks_for(&[]);

        let (record, server, job) = an_ended_job();
        let mut machine = AMachineInHand {
            answers: VecDeque::from([
                Fields::none().and(ON_THIS_MACHINE_FIELD, THE_MACHINE_CANNOT_SAY),
                a_finished_note().as_the_answers_fields(),
            ]),
            asked: 0,
        };
        for _ in 0..2 {
            let (did, _) =
                the_words_owed_are_taken_in(&server, &record, &mut machine, &reads, &HashSet::new());
            assert_eq!(did.still_owed, 1);
        }
        assert_eq!(owed(&record, &server), vec![job.clone()]);
        let passed = HashSet::from([job.as_text()]);
        let (_, asked) = the_words_owed_are_taken_in(&server, &record, &mut machine, &reads, &passed);
        assert!(asked.is_empty());
        assert_eq!(machine.asked, 2, "not asked again on the same connection");

        let mut machine = AMachineInHand {
            answers: VecDeque::from([Fields::none().and(ON_THIS_MACHINE_FIELD, IT_IS_ABSENT)]),
            asked: 0,
        };
        let (did, _) =
            the_words_owed_are_taken_in(&server, &record, &mut machine, &reads, &HashSet::new());
        assert_eq!(did.settled_with_nothing, 1);
        assert!(owed(&record, &server).is_empty());
        assert!(kept(&record, &job).is_empty(), "nothing is invented for the silence");
    }

    /// **`Desktop app and delivery` M10a**: an attempt whose outcome is not
    /// yet written lands as that, with no words — never as one of the four —
    /// and the debt stands until it is written.
    #[test]
    fn an_outcome_not_yet_written_lands_as_that_and_the_debt_stands() {
        let (record, server, job) = an_ended_job();
        let mut pending = a_capture(1, b"Fra:3", b"");
        pending.complaints.outcome = None;
        let reads = sinks_for(&[(&pending, b"Fra:3", b"")]);
        let mut machine = AMachineInHand {
            answers: VecDeque::from([the_note_with(&[pending.clone()])]),
            asked: 0,
        };
        let (did, _) =
            the_words_owed_are_taken_in(&server, &record, &mut machine, &reads, &HashSet::new());
        assert_eq!(did.still_owed, 1);
        let kept = kept(&record, &job);
        assert_eq!(kept.len(), 2);
        assert_eq!(kept[0].the_words, b"Fra:3");
        assert_eq!(kept[0].outcome, Some(CollectionOutcome::Collected));
        assert_eq!(kept[1].outcome, None);
        assert!(kept[1].the_words.is_empty());
        assert_eq!(owed(&record, &server), vec![job]);
    }
}
