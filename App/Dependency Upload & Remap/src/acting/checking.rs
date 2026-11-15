//! **§3.1 and §3.4 performed — G1, G14 to G17.** One reading of one `.blend`,
//! driven through the supervisor as a file-inspection worker; and the loop that
//! issues the next reading as files arrive, cancels the in-flight one, and
//! leaves the sitting standing when the connection goes.
//!
//! **G14: the check is a desktop-driven loop of single-file readings, not one
//! long machine-side operation.** That is why the loop is here and not on the
//! machine, and why G15's *"there is no unbounded machine-side loop to stop"*
//! is true by construction: stopping is the desktop ceasing to issue readings.
//!
//! **G16: the sitting is durable as it happens**, so every finding of every
//! completed reading is handed to the caller the moment it lands rather than
//! gathered and returned at the end. [`TheSittingsWriter`] is that hand-over,
//! and it is a trait because K4's durable write is the local record's.
//!
//! **G17: nothing carries on by itself when the connection comes back.** This
//! loop is driven by the user's sitting and has no resumption of its own.
//!
//! # The route, from 2026-09-29 — step 10c's dependency wave piece 2
//!
//! **G1: the reading is *"driven as a file-inspection worker through the
//! supervisor"*, and `Render Settings` Q4: the bare-connection invocation *"is
//! the mechanism's shape, not a route around the admission point"*.** Until
//! this piece the reading planted a link to the Blender program, wrote its
//! script to a fixed site and ran a bare command naming the link — the route
//! both clauses rule out, and the one that crashed Blender at the desk. **It is
//! one request to the machine's supervisor now**, which admits a
//! file-inspection worker under the declarations of `Render Settings` W1, in
//! that document's §10, at `Rented-server runtime` A4's one admission point;
//! the worker writes the script into its
//! own operation's folder, starts Blender by the build's own path with the
//! check's own line, keeps both streams, reaps it and reports.
//!
//! **The document comes back through the captured stream and never inside the
//! answer — the runner's ruling of 2026-09-28.** One message's ceiling is one
//! MiB and a reading's document can be longer, so the answer says where the
//! worker's two streams sit, how long each is, the exit status, whether the
//! start mark was seen and whether Smoozy ended the run, and this file reads
//! the streams back over the file session and finds the document between the
//! marks itself.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Job files and outputs` R1. `Server protocol and
//! transfers` C1, C4, C8, C17a.
//!
//! **`Rented-server runtime` A4 is not listed above and is written out at the
//! site**, because this plan defines an A4 of its own — the same care
//! [`crate::arriving_later`] already takes with it.

use std::collections::VecDeque;
use std::future::Future;
use std::time::Duration;

use smoozy_render_settings::file_half::{version_text, BlenderIdentity};
use smoozy_server_protocol::asking_the_supervisor::WhatCameBack;
use smoozy_server_protocol::framing::{
    Answered, Fields, Request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
};
use smoozy_server_protocol::requests as asked;
use smoozy_server_protocol::subsystem::client::Subsystem;
use smoozy_state::identities::{Identity, IdentityKind};
use tokio::io::{AsyncRead, AsyncWrite};

use super::blender;
use crate::bytes::MachineLocation;
use crate::check::{Reading, ReadingDocument, Visited, WhatTheReadingLeft};

/// **G15: each reading is one operation with its own operation identity,
/// admitted through arbitration on its own.**
///
/// The identity is issued before the request leaves, which is `Server protocol
/// and transfers` C1's rule and `Rented-server runtime` A4's requirement for a
/// per-operation worker. **That A4 is that document's and not this plan's**,
/// which defines an A4 of its own about the mirror rule. **It is the request's
/// own identity on the wire**, so the machine's operation record is written
/// under it and a repeat of the request reads where that record stands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OneReading {
    pub operation: Identity,
    pub of: MachineLocation,
    pub with: BlenderIdentity,
}

impl OneReading {
    pub fn of(file: &MachineLocation, with: &BlenderIdentity) -> Option<OneReading> {
        Some(OneReading {
            operation: Identity::issue(IdentityKind::Operation).ok()?,
            of: file.clone(),
            with: with.clone(),
        })
    }

    /// **The one request, *a reading of a `.blend`*, for this reading** —
    /// `smoozy_server_protocol::requests`' own, whose documentation says why
    /// one name serves both readings and why a repeat of it is how the end is
    /// learnt.
    ///
    /// **Every path is a protocol field and never a word of a command**
    /// (A8, `Security and licence` §10): the build's and the `.blend`'s cross
    /// as the bytes they are.
    pub fn the_request(&self) -> Request {
        Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            self.operation.clone(),
            asked::THE_READING_OF_A_BLEND_REQUEST,
        )
        .carrying(
            Fields::none()
                .and(asked::THE_WHICH_READING_FIELD, asked::THE_CHECKS_READING)
                .and(asked::THE_BUILD_FIELD, self.with.path.as_bytes())
                .and(asked::THE_BLEND_FIELD, self.of.as_bytes())
                .and(
                    asked::THE_BLENDER_VERSION_FIELD,
                    version_text(self.with.version),
                ),
        )
    }
}

/// **The machine a reading is asked of: its supervisor, and its file
/// session.**
///
/// **A seam and not a stand-in.** On the desktop both halves are
/// `Server protocol and transfers` T5's per-server point — the supervisor
/// asked through `smoozy_server_protocol::asking_the_supervisor`, and the
/// stream read over the reading session with
/// [`read_a_stream_back_over_the_file_session`]. The tests answer it from a
/// scripted machine.
pub trait TheMachineAReadingIsAskedOf {
    /// **One framed request to the supervisor, and what came back** (§1.1 item
    /// 1 of `Server protocol and transfers`).
    fn ask(&mut self, request: &Request) -> impl Future<Output = WhatCameBack> + Send;

    /// **The whole of one captured stream, read back over the file session**:
    /// `length` bytes from the start of the file at `path`, and an error in
    /// the session's own words where fewer could be read.
    fn read_the_stream_back(
        &mut self,
        path: &[u8],
        length: u64,
    ) -> impl Future<Output = Result<Vec<u8>, String>> + Send;
}

/// Why a reading did not complete — **which is not the same as a reading that
/// returned nothing** (G3). Every one ends as *could not be read*, and they
/// are kept apart here so the caller can say which happened.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheReadingDidNotRun {
    /// **The supervisor was never asked** — the link to the entry point could
    /// not be planted, or the command never reached the machine.
    TheRequestDidNotReachTheMachine,
    /// **The command ran and no correlated answer came back.** `Server protocol
    /// and transfers` C17a: the silence is not yet the supervisor's.
    NoAnswerCameBack,
    /// **The machine's program refused the request by name** — a program
    /// placed before this release, which knows no reading of a `.blend`.
    TheMachinesProgramDoesNotTakeIt,
    /// **The machine did not take the reading, in its own words** — not
    /// admitted while a setup holds the machine, a field it could not read, a
    /// store that refused. `Server protocol and transfers` C8's *no record of
    /// that identity*: the work did not happen.
    TheMachineRefusedIt(String),
    /// The operation passed its outer limit without ending — **only the
    /// contents reading and M14's fixing carry one**,
    /// [`THE_OUTER_LIMIT_ON_A_CONTENTS_READING_OR_A_FIXING`], over the whole
    /// operation. **The wait ended, not the run** — see
    /// [`super::blender`].
    ItPassedTheTimeLimit,
    /// **One ask went [`THE_LONGEST_AN_ASK_WAITS`] with no answer**: the
    /// server stopped answering. A reading the server keeps answering is never
    /// this — `Dependency Upload & Remap` G15's build detail of 2026-11-13, the
    /// Creator's note 4 on the test copy of 2026-11-12. **The wait ended, not
    /// the run.**
    AnAskWentUnanswered,
    /// **`Render Settings` Q8: a read Smoozy ended itself** is none of the
    /// endings that say something about the file, and nothing about the file
    /// is claimed. It is the answer's own field read and never an inference.
    SmoozyEndedIt,
    /// **The user stopped the check while this reading was on the machine**,
    /// and the stop was sent naming it — `Dependency Upload & Remap` G15's
    /// *cancelling the in-flight one*, under that document's O1. **The
    /// reading is not waited for**: whether the machine's interrupt reached
    /// its Blender is the machine's to record, and nothing about the file is
    /// claimed either way. Added 2026-10-04, the 38's piece 7.
    TheUserStoppedIt,
    /// **The captured streams could not be read back over the file session**,
    /// in the session's own words.
    TheStreamsCouldNotBeReadBack(String),
}

/// **The pace between two asks of the same reading**, while the machine says
/// it is still in hand. **A pace and not a deadline**: the deadline is the
/// desk-side bound on a Blender run, over the whole reading. A Build figure,
/// chosen so a reading that takes a second is learnt about within one more
/// and a slow one costs one short request a second.
pub const BETWEEN_TWO_ASKS_OF_ONE_READING: Duration = Duration::from_secs(1);

/// **The longest one ask of a reading waits for its answer** — `Render
/// Settings` Q10's build detail and `Dependency Upload & Remap` G15's, both of
/// 2026-11-13. **A deadline on each ask and never on the whole reading**: the
/// server answers each ask at once from the operation's record, so a server
/// that has not answered one in a minute has stopped answering, while a
/// reading it keeps answering *still in hand* goes on for as long as Blender
/// takes. A Build figure, a minute, the same as `Servers` K3's slowest
/// declaration of a broken line. Until that date `Server Setup` B1's minute
/// bounded the whole reading, and a long read failed at a minute on a server
/// still answering (the Creator's note 4 on the test copy of 2026-11-12).
pub const THE_LONGEST_AN_ASK_WAITS: Duration = Duration::from_secs(60);

/// **The outer limit on the contents reading (Q5 route 2) and M14's fixing**
/// — `Dependency Upload & Remap` G15's build detail as changed 2026-11-13, the
/// runner's ruling 1 at the foot of `Reading the file, past a minute — what
/// landed.md`. Each goes on while the server says it is still working, as the
/// readings do, with [`THE_LONGEST_AN_ASK_WAITS`] on each ask; **and, since no
/// user can stop either, a server that says *still working* for this long has
/// them end as not completed.** A Build figure. Until that date both stopped at
/// `Server Setup` B1's minute over the whole operation, so a large `.blend`
/// failed its fixing or its contents check on a server still working.
pub const THE_OUTER_LIMIT_ON_A_CONTENTS_READING_OR_A_FIXING: Duration = Duration::from_secs(30 * 60);

/// **G1: one reading of one `.blend`, by a file-inspection worker on the
/// machine, returning one document between sentinels.**
///
/// **The request is asked, and asked again under its own identity until the
/// machine says it has a recorded outcome** (`Server protocol and transfers`
/// C4 and C8). Then the two captured streams the answer names are read back
/// over the file session, **each held to the length the answer states**, and
/// judged by [`what_the_streams_left`] exactly as before. What comes back is
/// judged by [`crate::check::judge`], which the caller applies — this
/// function **never turns a missing answer into an empty one**.
///
/// **No site on the machine is written by this act**, so it takes no
/// [`blender::TheSitesHeld`] and several readings may run on one machine at
/// once — which is what `Dependency Upload & Remap` O4's *nothing declared*
/// says the machine may do.
pub async fn drive_one_reading<M>(
    machine: &mut M,
    reading: &OneReading,
) -> Result<WhatTheReadingLeft, TheReadingDidNotRun>
where
    M: TheMachineAReadingIsAskedOf,
{
    drive_one_reading_until_stopped(machine, reading, &mut || false).await
}

/// **The same, and the user may stop it while it is on the machine** —
/// `Dependency Upload & Remap` G15: *"stopping the check is the desktop ceasing
/// to issue readings, cancelling the in-flight one under Section 9's
/// contract"*. `stopped` is asked before every ask of the reading; once it says
/// yes, `Server protocol and transfers` C21's stop is sent once, naming the
/// reading's own operation, and the reading ends here as
/// [`TheReadingDidNotRun::TheUserStoppedIt`]. Added 2026-10-04, the 38's
/// piece 7; until then a stop threw the in-flight answer away when it landed
/// and the Blender read on.
pub async fn drive_one_reading_until_stopped<M>(
    machine: &mut M,
    reading: &OneReading,
    stopped: &mut (dyn FnMut() -> bool + Send),
) -> Result<WhatTheReadingLeft, TheReadingDidNotRun>
where
    M: TheMachineAReadingIsAskedOf,
{
    let ended = asked_until_it_ends(
        machine,
        &reading.the_request(),
        THE_LONGEST_AN_ASK_WAITS,
        None,
        BETWEEN_TWO_ASKS_OF_ONE_READING,
        stopped,
    )
    .await?;
    // **Stop is watched while the output is read back too** (Sol's look at
    // the window and read points, point 1, 2026-11-13): the reading has ended
    // on the machine, so nothing is sent; the wait ends and nothing about the
    // file is claimed.
    tokio::select! {
        biased;
        left = what_the_ended_reading_left(machine, &ended) => left,
        () = until_stopped(stopped) => Err(TheReadingDidNotRun::TheUserStoppedIt),
    }
}

/// **`Server protocol and transfers` C21's stop, naming an operation on the
/// machine** — its own command with its own operation identity, issued before
/// it leaves (C1). What came back is not waited on beyond the one answer.
pub fn the_stop_of(operation: &Identity) -> Option<Request> {
    Some(
        Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            Identity::issue(IdentityKind::Operation).ok()?,
            asked::THE_STOP_REQUEST,
        )
        .carrying(Fields::none().and(asked::THE_OPERATION_TO_STOP_FIELD, operation.as_text())),
    )
}

/// **One request asked, and asked again under its own identity until the
/// machine says it has a recorded outcome** (`Server protocol and transfers`
/// C4 and C8) — every file-inspection operation this module drives: the
/// reading, the contents reading (`Dependency Upload & Remap` Q3) and the
/// fixing (M14). **`each_ask` is a deadline on one ask's answer** — an ask
/// unanswered past it is [`TheReadingDidNotRun::AnAskWentUnanswered`] — and
/// **`whole`, where one is given, is a deadline over the whole operation**:
/// the readings carry none and go on while the server answers that they are
/// still running (G15's build detail, 2026-11-13); the contents reading and
/// the fixing, which no user can stop, carry
/// [`THE_OUTER_LIMIT_ON_A_CONTENTS_READING_OR_A_FIXING`]. **The
/// pace is not a deadline.** Where `stopped` says yes, the stop is sent once
/// and the wait ends at once ([`TheReadingDidNotRun::TheUserStoppedIt`]).
pub async fn asked_until_it_ends<M>(
    machine: &mut M,
    request: &Request,
    each_ask: Duration,
    whole: Option<Duration>,
    pace: Duration,
    stopped: &mut (dyn FnMut() -> bool + Send),
) -> Result<Answered, TheReadingDidNotRun>
where
    M: TheMachineAReadingIsAskedOf,
{
    let deadline = whole.map(|whole| tokio::time::Instant::now() + whole);
    let mut first = true;
    loop {
        if !first && stopped() {
            return Err(the_stop_is_sent(machine, request).await);
        }
        first = false;
        let this_ask = tokio::time::Instant::now() + each_ask;
        let (wait_until, past_it) = match deadline {
            Some(deadline) if deadline <= this_ask => {
                (deadline, TheReadingDidNotRun::ItPassedTheTimeLimit)
            }
            _ => (this_ask, TheReadingDidNotRun::AnAskWentUnanswered),
        };
        // **Stop is looked for while the ask waits**, so a Stop pressed while
        // the server does not answer ends the reading at once rather than at
        // the end of the ask's minute (after Sol's look at reading the file
        // past a minute, point 3). An answer already in hand is taken first.
        let came_back = tokio::select! {
            biased;
            came_back = tokio::time::timeout_at(wait_until, machine.ask(request)) => Some(came_back),
            () = until_stopped(stopped) => None,
        };
        let Some(came_back) = came_back else {
            return Err(the_stop_is_sent(machine, request).await);
        };
        let answered = match came_back {
            Err(_) => return Err(past_it),
            Ok(WhatCameBack::ItAnswered(answered)) => answered,
            Ok(WhatCameBack::NothingCorrelatedCameBack) => {
                return Err(TheReadingDidNotRun::NoAnswerCameBack)
            }
            Ok(WhatCameBack::ItWasNeverAsked) => {
                return Err(TheReadingDidNotRun::TheRequestDidNotReachTheMachine)
            }
        };
        if answered
            .the_request_name_it_says_it_does_not_answer()
            .is_some()
        {
            return Err(TheReadingDidNotRun::TheMachinesProgramDoesNotTakeIt);
        }
        match answered.fields.text(asked::WHAT_BECAME_OF_IT_FIELD) {
            Some(asked::IT_HAS_A_RECORDED_OUTCOME) => return Ok(answered),
            Some(asked::IT_IS_STILL_IN_HAND) => {}
            _ => {
                return Err(TheReadingDidNotRun::TheMachineRefusedIt(
                    answered
                        .fields
                        .text(asked::THE_MACHINES_OWN_WORDS_FIELD)
                        .unwrap_or("the machine did not say what became of the request")
                        .to_string(),
                ))
            }
        }
        if deadline.is_some_and(|deadline| tokio::time::Instant::now() + pace >= deadline) {
            return Err(TheReadingDidNotRun::ItPassedTheTimeLimit);
        }
        tokio::select! {
            biased;
            () = tokio::time::sleep(pace) => {}
            () = until_stopped(stopped) => return Err(the_stop_is_sent(machine, request).await),
        }
    }
}

/// **How often a wait looks for the user's Stop.** A Build figure, short
/// enough that Stop reads as at once.
const HOW_OFTEN_A_WAIT_LOOKS_FOR_STOP: Duration = Duration::from_millis(100);

/// **The longest the stop's own send is waited on** — five seconds, as
/// `Render Settings`' read waits on its own (`crate` `reading_the_file`'s
/// Stop), after Sol's look at reading the file past a minute, point 3. The
/// reading ends here either way: whether the stop reached the server's Blender
/// is the server's to record, and nothing about the file is claimed.
pub const THE_LONGEST_A_STOP_IS_WAITED_ON: Duration = Duration::from_secs(5);

/// **Ends when `stopped` says yes**, looking every
/// [`HOW_OFTEN_A_WAIT_LOOKS_FOR_STOP`].
async fn until_stopped(stopped: &mut (dyn FnMut() -> bool + Send)) {
    while !stopped() {
        tokio::time::sleep(HOW_OFTEN_A_WAIT_LOOKS_FOR_STOP).await;
    }
}

/// **`Server protocol and transfers` C21's stop, sent once and waited on
/// [`THE_LONGEST_A_STOP_IS_WAITED_ON`] at most**, naming the reading's own
/// operation; the reading ends as stopped by the user.
async fn the_stop_is_sent<M>(machine: &mut M, request: &Request) -> TheReadingDidNotRun
where
    M: TheMachineAReadingIsAskedOf,
{
    if let Some(stop) = the_stop_of(&request.identity) {
        let _ = tokio::time::timeout(THE_LONGEST_A_STOP_IS_WAITED_ON, machine.ask(&stop)).await;
    }
    TheReadingDidNotRun::TheUserStoppedIt
}

/// The same, with the wait on each ask and the pace handed in.
///
/// **Both are parameters so a test can watch the deadline fire** without
/// waiting a minute, and for no other reason: [`drive_one_reading`] hands in
/// [`THE_LONGEST_AN_ASK_WAITS`]. **The bound is on each ask, never on the
/// whole reading** (G15's build detail, 2026-11-13).
pub async fn drive_one_reading_within<M>(
    machine: &mut M,
    reading: &OneReading,
    each_ask: Duration,
    pace: Duration,
) -> Result<WhatTheReadingLeft, TheReadingDidNotRun>
where
    M: TheMachineAReadingIsAskedOf,
{
    let ended =
        asked_until_it_ends(machine, &reading.the_request(), each_ask, None, pace, &mut || false).await?;
    what_the_ended_reading_left(machine, &ended).await
}

/// **What a reading with a recorded outcome left**, from the answer's own
/// fields and the two streams it names.
///
/// **A failing status is the exit status read as the machine reaped it**: a
/// status other than nought, or none at all (a run ended by a signal, or a
/// Blender that could not be started), is not a clean ending. **A reading
/// whose worker was never started names no stream**, and is a failing status
/// without anything read back — the streams are not invented as empty.
pub async fn what_the_ended_reading_left<M>(
    machine: &mut M,
    ended: &Answered,
) -> Result<WhatTheReadingLeft, TheReadingDidNotRun>
where
    M: TheMachineAReadingIsAskedOf,
{
    // **A reading the server says was interrupted is Smoozy's ending too** —
    // its worker went with a restart of the server's Smoozy, and nothing about
    // the file is claimed (after Sol's look at reading the file past a minute,
    // point 4).
    if ended.fields.text(asked::THE_RUN_WAS_ENDED_BY_SMOOZY_FIELD) == Some(asked::THE_ANSWER_IS_YES)
        || ended.fields.text(asked::THE_READING_WAS_INTERRUPTED_FIELD) == Some(asked::THE_ANSWER_IS_YES)
    {
        return Err(TheReadingDidNotRun::SmoozyEndedIt);
    }
    let ended_cleanly = ended.fields.text(asked::THE_EXIT_STATUS_FIELD) == Some("0");
    let stream = |at: &str, length: &str| -> Option<(Vec<u8>, u64)> {
        let path = ended.fields.value(at)?.to_vec();
        let length = ended.fields.text(length)?.parse().ok()?;
        Some((path, length))
    };
    let (Some(ordinary), Some(complaints)) = (
        stream(
            asked::THE_ORDINARY_STREAM_FIELD,
            asked::THE_ORDINARY_STREAM_LENGTH_FIELD,
        ),
        stream(
            asked::THE_COMPLAINTS_STREAM_FIELD,
            asked::THE_COMPLAINTS_STREAM_LENGTH_FIELD,
        ),
    ) else {
        return Ok(WhatTheReadingLeft::AFailingStatus);
    };
    let ordinary = machine
        .read_the_stream_back(&ordinary.0, ordinary.1)
        .await
        .map_err(TheReadingDidNotRun::TheStreamsCouldNotBeReadBack)?;
    let complaints = machine
        .read_the_stream_back(&complaints.0, complaints.1)
        .await
        .map_err(TheReadingDidNotRun::TheStreamsCouldNotBeReadBack)?;
    Ok(what_the_streams_left(&ordinary, &complaints, ended_cleanly))
}

/// **How much one read of the file session asks for.** A Build figure: the
/// session answers with at most what it holds, so a larger ask is split by the
/// session and a smaller one costs more round trips.
pub const ONE_READ_OF_A_STREAM_ASKS_FOR: usize = 64 * 1024;

/// **One captured stream, read back over the file session, to the length the
/// answer stated** — the desktop's half of the runner's ruling of 2026-09-28.
///
/// **A stream that ends before its stated length is an error in words**, never
/// a short document: the machine stated how many bytes it retained, and fewer
/// arriving means the read was cut, which G3 makes *not a reading* one level
/// up. Bytes past the stated length are not asked for.
pub async fn read_a_stream_back_over_the_file_session<S>(
    subsystem: &mut Subsystem<S>,
    path: &[u8],
    length: u64,
) -> Result<Vec<u8>, String>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    read_a_streams_bytes_over_the_file_session(subsystem, path, 0, length).await
}

/// **The bytes of a captured stream from `from` up to `to`, over the file
/// session** — the same read as [`read_a_stream_back_over_the_file_session`],
/// which is this from nought. **`Queue & History` E8's live commentary asks
/// for only the bytes past the length it last read**, up to the length the
/// queue reading stated, so a refresh costs the new bytes and never the whole
/// stream again. Added 2026-10-03, the Creator's notes on 0.1.8, piece 6c.
///
/// **A stream that ends before `to` is an error in words**, as it is for the
/// whole read: the machine stated how many bytes stood, and fewer arriving
/// means the read was cut. Nothing is asked for where `to` is not past `from`.
pub async fn read_a_streams_bytes_over_the_file_session<S>(
    subsystem: &mut Subsystem<S>,
    path: &[u8],
    from: u64,
    to: u64,
) -> Result<Vec<u8>, String>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    if to <= from && from > 0 {
        return Ok(Vec::new());
    }
    let handle = subsystem
        .open_to_read(path)
        .await
        .map_err(|why| format!("{}: {why:?}", String::from_utf8_lossy(path)))?;
    let mut read = Vec::new();
    let mut outcome = Ok(());
    while from + (read.len() as u64) < to {
        let at = from + read.len() as u64;
        let wanted = ((to - at) as usize).min(ONE_READ_OF_A_STREAM_ASKS_FOR);
        match subsystem.read_at(&handle, at, wanted).await {
            Ok(bytes) if bytes.is_empty() => {
                outcome = Err(format!(
                    "{}: the stream ended at {} of the {to} bytes the machine stated",
                    String::from_utf8_lossy(path),
                    at
                ));
                break;
            }
            Ok(bytes) => read.extend_from_slice(&bytes),
            Err(why) => {
                outcome = Err(format!("{}: {why:?}", String::from_utf8_lossy(path)));
                break;
            }
        }
    }
    let _ = subsystem.close(&handle).await;
    outcome.map(|()| read)
}

/// **G3, over the two streams and the status.** A failing status is *could not
/// be read*; so is a stream carrying no sentinel-framed document; so is a
/// document that will not read back. **An empty answer is never inferred from a
/// missing one.**
pub fn what_the_streams_left(
    ordinary: &[u8],
    complaints: &[u8],
    ended_cleanly: bool,
) -> WhatTheReadingLeft {
    let framed = blender::between_the_sentinels(ordinary)
        .or_else(|| blender::between_the_sentinels(complaints));
    let Some(framed) = framed else {
        return if ended_cleanly {
            WhatTheReadingLeft::NoSentinelFramedDocument
        } else {
            WhatTheReadingLeft::AFailingStatus
        };
    };
    // **A status that did not end cleanly is a failing status even where a
    // document was framed.** `--python-exit-code 1` makes a raised script end
    // at 1, and a document written before the raise is half a reading — which
    // §10's own test 2 calls the check lying.
    if !ended_cleanly {
        return WhatTheReadingLeft::AFailingStatus;
    }
    match serde_json::from_slice::<ReadingDocument>(framed) {
        // **A colour name the reading Blender lacks** (*Colour spaces and image
        // sequences*): Blender says it on either stream as it opens the file,
        // outside the marks, and the script cannot read it back.
        Ok(mut document) => {
            document.colour_names_not_found =
                crate::check::the_colour_names_not_found(&[ordinary, complaints]);
            WhatTheReadingLeft::ASentinelFramedDocument(document)
        }
        Err(_) => WhatTheReadingLeft::NoSentinelFramedDocument,
    }
}

/// **G13: finiteness is a visited set, not a depth limit** — and G14's loop is
/// what carries it. A file already read in this sitting is not walked again,
/// keyed by its resolved machine location; a chain that comes back round stops
/// there.
///
/// **The loop is a value rather than a function that runs to the end**, because
/// G14 drives it from the user's sitting: the caller takes one file at a time,
/// hands back what the reading found, and stops whenever it likes. **F6 and
/// G16's *what was found stays on screen* need nothing of the loop for that** —
/// every finding was already written when it landed.
#[derive(Debug, Default)]
pub struct TheLoop {
    visited: Visited,
    waiting: VecDeque<MachineLocation>,
}

impl TheLoop {
    /// **Pass one reads the file the user pointed at.**
    pub fn over(root: MachineLocation) -> TheLoop {
        let mut waiting = VecDeque::new();
        waiting.push_back(root);
        TheLoop {
            visited: Visited::default(),
            waiting,
        }
    }

    /// The next file to read, or nothing where the tree is walked out.
    ///
    /// **G13's visited set is applied here and nowhere else**, so a file cannot
    /// be walked twice by any route into this loop.
    pub fn next_file(&mut self) -> Option<MachineLocation> {
        while let Some(next) = self.waiting.pop_front() {
            if self.visited.first_time(&next) {
                return Some(next);
            }
        }
        None
    }

    /// **G14: every location found below the top file is read against the file
    /// that holds it.** The caller resolves each of this reading's references to
    /// a machine location — that is the record's and the landing rule's work,
    /// not the loop's — and hands back the linked `.blend`s among them.
    ///
    /// **The readable chain stops at the first absent file**: a location that
    /// resolved to nothing is not offered here, so a missing linked `.blend`
    /// hides what is below it and the row says so. A short list is never taken
    /// for the whole.
    pub fn and_below_it(&mut self, linked_blends: impl IntoIterator<Item = MachineLocation>) {
        for one in linked_blends {
            self.waiting.push_back(one);
        }
    }

    /// **G14: when an upload puts a missing linked file on the machine, that
    /// file is read — its own reading, its own operation.** Same door as
    /// [`Self::and_below_it`], named separately because it is the case the
    /// clause is about.
    pub fn a_file_arrived(&mut self, at: MachineLocation) {
        self.waiting.push_back(at);
    }

    /// How many files are still waiting. **G15: the number of passes is bounded
    /// by the number of files, which the visited set makes finite.**
    pub fn still_waiting(&self) -> usize {
        self.waiting.len()
    }
}

/// **G16 and K4: every finding of every completed reading, and every user
/// decision, is written at the moment it lands** — never batched to the end of
/// a sitting. The writing is the local record's, so this is the seam.
pub trait TheSittingsWriter {
    /// One completed reading's document, written as it lands.
    fn a_reading_landed(&mut self, of: &MachineLocation, reading: &Reading);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::check::{Attribution, Kind, ReadReference};

    fn at(path: &[u8]) -> MachineLocation {
        MachineLocation::composed(path.to_vec())
    }

    fn a_document() -> ReadingDocument {
        ReadingDocument {
            references: vec![ReadReference {
                kind: Kind::Image,
                datablock: "Tex".to_string(),
                setting: "filepath".to_string(),
                recorded: crate::bytes::RecordedLocation::as_the_reading_carried_it(
                    b"//tex/METallic.png",
                ),
                scenes: Attribution::Established(vec!["Interior".to_string()]),
                source: Some("FILE".to_string()),
                the_files_own_text: None,
                sequence: None,
            }],
            flat_list: Vec::new(),
            weak_references: Vec::new(),
            observed_stand_ins: Vec::new(),
            colour_names_not_found: Vec::new(),
        }
    }

    fn framed(body: &[u8], ordinary: bool) -> (Vec<u8>, Vec<u8>) {
        let stream = [
            b"Blender quit\n".as_slice(),
            blender::THE_OPENING_SENTINEL,
            body,
            blender::THE_CLOSING_SENTINEL,
            b"\n",
        ]
        .concat();
        if ordinary {
            (stream, Vec::new())
        } else {
            (Vec::new(), stream)
        }
    }

    /// **The document the script writes reads back into the shape the rules
    /// half judges.** This is the one seam between the two halves, and it is
    /// asserted over the exact bytes rather than over a hand-built value.
    #[test]
    fn the_documents_written_form_reads_back_into_the_readings_own_shape() {
        let written = serde_json::to_vec(&a_document()).expect("it renders");
        let (ordinary, complaints) = framed(&written, true);
        assert_eq!(
            what_the_streams_left(&ordinary, &complaints, true),
            WhatTheReadingLeft::ASentinelFramedDocument(a_document())
        );
    }

    /// **A recorded location that is not text survives the crossing whole**,
    /// which is G3's carriage requirement and `Job files and outputs` R4a's
    /// rule met at this end: the document carries byte values, so nothing has
    /// to decode.
    #[test]
    fn a_location_that_is_not_text_crosses_whole() {
        let raw = [b'/', 0xff, 0xfe, b'/', b'x', b'.', b'p', b'n', b'g'];
        let mut document = a_document();
        document.references[0].recorded =
            crate::bytes::RecordedLocation::as_the_reading_carried_it(&raw);
        let written = serde_json::to_vec(&document).expect("it renders");
        assert!(
            std::str::from_utf8(&written).is_ok(),
            "the written form is text even where the location is not"
        );
        let (ordinary, complaints) = framed(&written, true);
        let WhatTheReadingLeft::ASentinelFramedDocument(back) =
            what_the_streams_left(&ordinary, &complaints, true)
        else {
            panic!("the document reads back");
        };
        assert_eq!(back.references[0].recorded.as_bytes(), &raw);
    }

    /// **G3, four ways, and every one of them is *could not be read* rather
    /// than an empty list.**
    #[test]
    fn a_reading_that_did_not_complete_is_never_an_empty_one() {
        let written = serde_json::to_vec(&a_document()).expect("it renders");
        // A failing status, with a whole document on the stream.
        let (ordinary, complaints) = framed(&written, true);
        assert_eq!(
            what_the_streams_left(&ordinary, &complaints, false),
            WhatTheReadingLeft::AFailingStatus
        );
        // A clean status with no marks at all.
        assert_eq!(
            what_the_streams_left(b"Blender quit\n", b"", true),
            WhatTheReadingLeft::NoSentinelFramedDocument
        );
        // A clean status with a stream cut short before the closing mark.
        let cut = [b"x".as_slice(), blender::THE_OPENING_SENTINEL, &written].concat();
        assert_eq!(
            what_the_streams_left(&cut, b"", true),
            WhatTheReadingLeft::NoSentinelFramedDocument
        );
        // Marks around something that is not a document.
        let (rubbish, _) = framed(b"not a document", true);
        assert_eq!(
            what_the_streams_left(&rubbish, b"", true),
            WhatTheReadingLeft::NoSentinelFramedDocument
        );
    }

    /// **Which stream carries the document was never recorded, so both are
    /// read.** A document on the complaints stream is the same reading.
    #[test]
    fn the_document_is_found_on_either_stream() {
        let written = serde_json::to_vec(&a_document()).expect("it renders");
        let (ordinary, complaints) = framed(&written, false);
        assert_eq!(
            what_the_streams_left(&ordinary, &complaints, true),
            WhatTheReadingLeft::ASentinelFramedDocument(a_document())
        );
    }

    /// **G13 and G15: a chain that comes back round stops, and the number of
    /// passes is bounded by the number of files.**
    #[test]
    fn the_loop_reads_each_file_once_and_a_cycle_stops_there() {
        let top = at(b"/root/assets/files/Users/a/top.blend");
        let below = at(b"/root/assets/files/Users/a/lib.blend");
        let mut walk = TheLoop::over(top.clone());
        assert_eq!(walk.next_file(), Some(top.clone()));
        // The library the top file links, and the top file itself coming back
        // round because the library links it in turn.
        walk.and_below_it([below.clone(), top.clone()]);
        assert_eq!(walk.next_file(), Some(below.clone()));
        assert_eq!(
            walk.next_file(),
            None,
            "the cycle stopped at the visited set"
        );

        // **G14: a file that arrives by an upload is read, its own reading.**
        // It is a file the loop has not seen, so it is offered.
        let arrived = at(b"/root/assets/files/Users/a/missing.blend");
        walk.a_file_arrived(arrived.clone());
        assert_eq!(walk.still_waiting(), 1);
        assert_eq!(walk.next_file(), Some(arrived));
        assert_eq!(walk.next_file(), None);
    }

    // ------------------------------------------------ the route, from piece 2

    /// **A scripted machine**: it answers each ask of the reading from a list,
    /// the last answer standing for every ask after it, and holds the streams
    /// the answers name. It writes down every request it was asked.
    struct AScriptedMachine {
        answers: Vec<Fields>,
        asked: Vec<Request>,
        streams: std::collections::HashMap<Vec<u8>, Vec<u8>>,
        refused_by_name: bool,
    }

    impl AScriptedMachine {
        fn answering(answers: Vec<Fields>) -> AScriptedMachine {
            AScriptedMachine {
                answers,
                asked: Vec::new(),
                streams: std::collections::HashMap::new(),
                refused_by_name: false,
            }
        }
    }

    impl TheMachineAReadingIsAskedOf for AScriptedMachine {
        async fn ask(&mut self, request: &Request) -> WhatCameBack {
            self.asked.push(request.clone());
            let empty = Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "0.1.7");
            if self.refused_by_name {
                return WhatCameBack::ItAnswered(
                    empty.saying_it_does_not_answer_that_request(&request.name),
                );
            }
            let at = (self.asked.len() - 1).min(self.answers.len() - 1);
            WhatCameBack::ItAnswered(empty.carrying(self.answers[at].clone()))
        }

        async fn read_the_stream_back(
            &mut self,
            path: &[u8],
            length: u64,
        ) -> Result<Vec<u8>, String> {
            let held = self.streams.get(path).ok_or("no such stream")?;
            if (held.len() as u64) < length {
                return Err("the stream ended before its stated length".to_string());
            }
            Ok(held[..length as usize].to_vec())
        }
    }

    /// **The 38's piece 7, item 36: a stop asked while a reading is on the
    /// machine sends `Server protocol and transfers` C21's stop, once, naming
    /// the reading's own operation — and the reading ends here at once.** Until
    /// then a stop threw the answer away when it landed and the Blender read
    /// on.
    #[tokio::test]
    async fn a_stop_asked_while_a_reading_is_on_the_machine_sends_the_stop_naming_it() {
        let mut machine = AScriptedMachine::answering(vec![still_in_hand()]);
        let reading = one_reading();
        let mut asks = 0;
        let mut stopped = || {
            asks += 1;
            asks >= 3
        };
        let left = drive_one_reading_until_stopped(&mut machine, &reading, &mut stopped).await;
        assert_eq!(left, Err(TheReadingDidNotRun::TheUserStoppedIt));
        let stops: Vec<&Request> = machine
            .asked
            .iter()
            .filter(|request| request.name == asked::THE_STOP_REQUEST)
            .collect();
        assert_eq!(stops.len(), 1, "one stop, sent once");
        assert_eq!(
            stops[0].fields.text(asked::THE_OPERATION_TO_STOP_FIELD),
            Some(reading.operation.as_text().as_str())
        );
        assert_ne!(stops[0].identity, reading.operation, "the stop has its own identity");
        assert_eq!(machine.asked.last().map(|last| last.name.as_str()), Some(asked::THE_STOP_REQUEST));

        // **A stop asked before the first ask asks nothing of the machine
        // but the reading itself first**: the reading is asked once.
        let mut machine = AScriptedMachine::answering(vec![still_in_hand()]);
        let left = drive_one_reading_until_stopped(&mut machine, &one_reading(), &mut || true).await;
        assert_eq!(left, Err(TheReadingDidNotRun::TheUserStoppedIt));
        assert_eq!(machine.asked.len(), 2, "the reading once, then the stop");
    }

    /// **The contents reading and the fixing go on past a minute while the
    /// server says it is still working** — `Dependency Upload & Remap` G15's
    /// build detail as changed 2026-11-13, the runner's ruling 1 at the foot of
    /// `Reading the file, past a minute — what landed.md`. The server answers
    /// every ask at once and says *still working* for seventy seconds, then
    /// gives its ending. On a paused clock, at the real figures. Until then the
    /// minute was over the whole operation, and both ended as not completed.
    #[tokio::test(start_paused = true)]
    async fn the_contents_reading_and_the_fixing_go_on_past_a_minute_while_the_server_is_still_working() {
        let mut answers = vec![still_in_hand(); 70];
        answers.push(
            Fields::none()
                .and(asked::WHAT_BECAME_OF_IT_FIELD, asked::IT_HAS_A_RECORDED_OUTCOME)
                .and(asked::THE_DIGESTS_OUTCOME_FIELD, asked::NOTHING_IS_THERE),
        );
        let mut machine = AScriptedMachine::answering(answers);
        let started = tokio::time::Instant::now();
        let contents =
            super::super::machine::the_machine_copys_contents(&mut machine, &at(b"/root/assets/files/a.blend")).await;
        assert_eq!(contents, crate::readings::MachineContents::NothingThere, "the contents reading");
        assert!(started.elapsed() >= Duration::from_secs(70), "{:?}", started.elapsed());

        let mut answers = vec![still_in_hand(); 70];
        answers.push(ended(b"", b"", Some("0")));
        let mut machine = holding(AScriptedMachine::answering(answers), Vec::new(), Vec::new());
        let fixing = super::super::fixing_worker::OneFixing::of(
            &at(b"/root/assets/files/a.blend"),
            &at(b"/root/assets/fixed/assets/files/a.blend"),
            &at(b"/root/assets/fixed/assets/files/.s/a.blend.fixing"),
            &one_reading().with,
            Vec::new(),
        )
        .expect("randomness");
        let started = tokio::time::Instant::now();
        let fixed = super::super::fixing_worker::produce_one_fixed_copy(&mut machine, &fixing).await;
        assert_eq!(fixed, Ok(()), "the fixing");
        assert!(started.elapsed() >= Duration::from_secs(70), "{:?}", started.elapsed());
    }

    /// **And both still end at the outer limit**, since no user can stop
    /// them: a server that says *still working* for ever ends each in its
    /// thirtieth minute (the last ask that would run past the limit is not
    /// waited for), never before and never after.
    #[tokio::test(start_paused = true)]
    async fn the_contents_reading_and_the_fixing_end_at_thirty_minutes_of_still_working() {
        let mut machine = AScriptedMachine::answering(vec![still_in_hand()]);
        let started = tokio::time::Instant::now();
        let contents =
            super::super::machine::the_machine_copys_contents(&mut machine, &at(b"/root/assets/files/a.blend")).await;
        assert_eq!(contents, crate::readings::MachineContents::DidNotComplete);
        let took = started.elapsed();
        assert!(took > Duration::from_secs(29 * 60) && took <= Duration::from_secs(30 * 60), "{took:?}");

        let mut machine = AScriptedMachine::answering(vec![still_in_hand()]);
        let fixing = super::super::fixing_worker::OneFixing::of(
            &at(b"/root/assets/files/a.blend"),
            &at(b"/root/assets/fixed/assets/files/a.blend"),
            &at(b"/root/assets/fixed/assets/files/.s/a.blend.fixing"),
            &one_reading().with,
            Vec::new(),
        )
        .expect("randomness");
        let started = tokio::time::Instant::now();
        let fixed = super::super::fixing_worker::produce_one_fixed_copy(&mut machine, &fixing).await;
        assert!(fixed.is_err());
        let took = started.elapsed();
        assert!(took > Duration::from_secs(29 * 60) && took <= Duration::from_secs(30 * 60), "{took:?}");
    }

    /// **A reading the server says was interrupted is Smoozy's ending, never a
    /// failing status** — after Sol's look at reading the file past a minute,
    /// point 4. Its worker went with a restart of the server's Smoozy: no exit
    /// status, no start mark, no stream. Read as a failing status, the check's
    /// root said *This Blender could not open the file*.
    #[tokio::test]
    async fn an_interrupted_reading_is_smoozys_ending_and_never_a_failing_status() {
        let mut machine = AScriptedMachine::answering(vec![Fields::none()
            .and(asked::WHAT_BECAME_OF_IT_FIELD, asked::IT_HAS_A_RECORDED_OUTCOME)
            .and(asked::THE_START_MARK_SEEN_FIELD, asked::THE_ANSWER_IS_NO)
            .and(asked::THE_RUN_WAS_ENDED_BY_SMOOZY_FIELD, asked::THE_ANSWER_IS_NO)
            .and(asked::THE_READING_WAS_INTERRUPTED_FIELD, asked::THE_ANSWER_IS_YES)]);
        let left = drive_one_reading(&mut machine, &one_reading()).await;
        assert_eq!(left, Err(TheReadingDidNotRun::SmoozyEndedIt));
    }

    /// **A machine that may never answer**: the reading's asks, the stop, or
    /// both. Where it answers the reading, it says *still working*.
    struct AMachineThatMayNotAnswer {
        asked: Vec<Request>,
        answers_the_reading: bool,
        answers_the_stop: bool,
    }

    impl TheMachineAReadingIsAskedOf for AMachineThatMayNotAnswer {
        async fn ask(&mut self, request: &Request) -> WhatCameBack {
            self.asked.push(request.clone());
            let is_the_stop = request.name == asked::THE_STOP_REQUEST;
            if (is_the_stop && !self.answers_the_stop) || (!is_the_stop && !self.answers_the_reading) {
                std::future::pending::<()>().await;
            }
            let empty = Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "0.1.7");
            WhatCameBack::ItAnswered(empty.carrying(still_in_hand()))
        }

        async fn read_the_stream_back(&mut self, _: &[u8], _: u64) -> Result<Vec<u8>, String> {
            Err("no stream".to_string())
        }
    }

    /// **Stop acts at once, and its own send waits five seconds at most** —
    /// `Dependency Upload & Remap` G15's build detail as changed 2026-11-13,
    /// after Sol's look at reading the file past a minute, point 3. Stop was
    /// read only between asks, so one pressed while an ask went unanswered
    /// waited out that ask's minute; and the stop's own send was waited on
    /// with no limit. On a paused clock, at the real figures:
    /// - Stop pressed two seconds into an ask the server does not answer ends
    ///   the reading then, and the stop is sent naming it.
    /// - Stop pressed while the server says *still working*, to a server that
    ///   never answers the stop: the reading ends five seconds after the stop
    ///   is sent.
    #[tokio::test(start_paused = true)]
    async fn stop_acts_at_once_and_its_own_send_waits_five_seconds_at_most() {
        let reading = one_reading();
        let mut machine = AMachineThatMayNotAnswer { asked: Vec::new(), answers_the_reading: false, answers_the_stop: true };
        let started = tokio::time::Instant::now();
        let mut stopped = || started.elapsed() >= Duration::from_secs(2);
        let left = tokio::time::timeout(
            Duration::from_secs(600),
            drive_one_reading_until_stopped(&mut machine, &reading, &mut stopped),
        )
        .await
        .expect("the reading ended");
        assert_eq!(left, Err(TheReadingDidNotRun::TheUserStoppedIt));
        let took = started.elapsed();
        assert!(took >= Duration::from_secs(2) && took < Duration::from_secs(3), "Stop waited {took:?}");
        assert_eq!(machine.asked.last().map(|last| last.name.as_str()), Some(asked::THE_STOP_REQUEST));

        let mut machine = AMachineThatMayNotAnswer { asked: Vec::new(), answers_the_reading: true, answers_the_stop: false };
        let started = tokio::time::Instant::now();
        let mut stopped = || started.elapsed() >= Duration::from_millis(1500);
        let left = tokio::time::timeout(
            Duration::from_secs(600),
            drive_one_reading_until_stopped(&mut machine, &reading, &mut stopped),
        )
        .await
        .expect("the stop's own send was waited on with no limit");
        assert_eq!(left, Err(TheReadingDidNotRun::TheUserStoppedIt));
        let took = started.elapsed();
        assert!(took <= Duration::from_secs(8), "the stop's send waited {took:?}");
    }

    /// **A fixing whose worker ends at 29 minutes 50 seconds, and whose report
    /// takes 30 seconds to come back.**
    struct AReportThatComesLate {
        ends_at: tokio::time::Instant,
    }

    impl TheMachineAReadingIsAskedOf for AReportThatComesLate {
        async fn ask(&mut self, request: &Request) -> WhatCameBack {
            let empty = Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "0.1.7");
            if tokio::time::Instant::now() < self.ends_at {
                return WhatCameBack::ItAnswered(empty.carrying(still_in_hand()));
            }
            WhatCameBack::ItAnswered(empty.carrying(ended(b"", b"", Some("0"))))
        }

        async fn read_the_stream_back(&mut self, _: &[u8], _: u64) -> Result<Vec<u8>, String> {
            tokio::time::sleep(Duration::from_secs(30)).await;
            Ok(Vec::new())
        }
    }

    /// **Sol's look at the window and read points, point 3 (the scout's
    /// list, item 6): one 30-minute deadline covers the rewrite's report
    /// too.** A worker that ends at 29:50 and whose report takes 30 seconds
    /// published past the half hour the whole operation is promised.
    #[tokio::test(start_paused = true)]
    async fn the_fixings_half_hour_covers_its_report() {
        let mut machine =
            AReportThatComesLate { ends_at: tokio::time::Instant::now() + Duration::from_secs(29 * 60 + 50) };
        let fixing = super::super::fixing_worker::OneFixing::of(
            &at(b"/root/assets/files/a.blend"),
            &at(b"/root/assets/fixed/assets/files/a.blend"),
            &at(b"/root/assets/fixed/assets/files/.s/a.blend.fixing"),
            &one_reading().with,
            Vec::new(),
        )
        .expect("randomness");
        let started = tokio::time::Instant::now();
        let fixed = super::super::fixing_worker::produce_one_fixed_copy(&mut machine, &fixing).await;
        assert!(fixed.is_err(), "a fixing past its half hour is not published");
        assert!(started.elapsed() <= Duration::from_secs(30 * 60), "{:?}", started.elapsed());
    }

    /// **A server that answers the reading's ending and never sends its
    /// output back.**
    struct AnOutputThatNeverComes;

    impl TheMachineAReadingIsAskedOf for AnOutputThatNeverComes {
        async fn ask(&mut self, request: &Request) -> WhatCameBack {
            let empty = Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "0.1.7");
            WhatCameBack::ItAnswered(empty.carrying(ended(b"output", b"", Some("0"))))
        }

        async fn read_the_stream_back(&mut self, _: &[u8], _: u64) -> Result<Vec<u8>, String> {
            std::future::pending().await
        }
    }

    /// **Sol's look at the window and read points, point 1 (the scout's
    /// list, item 6): Stop ends a reading whose output read is unanswered.**
    /// After Blender finished, its captured output was read with no eye on
    /// Stop, so a server that did not send it held the check after Stop.
    #[tokio::test(start_paused = true)]
    async fn stop_ends_a_reading_whose_output_never_comes() {
        let started = tokio::time::Instant::now();
        let mut stopped = || started.elapsed() >= Duration::from_secs(2);
        let left = tokio::time::timeout(
            Duration::from_secs(600),
            drive_one_reading_until_stopped(&mut AnOutputThatNeverComes, &one_reading(), &mut stopped),
        )
        .await
        .expect("Stop did not end the output read");
        assert_eq!(left, Err(TheReadingDidNotRun::TheUserStoppedIt));
        assert!(started.elapsed() < Duration::from_secs(3), "{:?}", started.elapsed());
    }

    fn still_in_hand() -> Fields {
        Fields::none().and(asked::WHAT_BECAME_OF_IT_FIELD, asked::IT_IS_STILL_IN_HAND)
    }

    fn ended(ordinary: &[u8], complaints: &[u8], status: Option<&str>) -> Fields {
        let mut fields = Fields::none()
            .and(
                asked::WHAT_BECAME_OF_IT_FIELD,
                asked::IT_HAS_A_RECORDED_OUTCOME,
            )
            .and(
                asked::THE_ORDINARY_STREAM_FIELD,
                b"/pkg/captured-streams/op/ordinary".to_vec(),
            )
            .and(
                asked::THE_ORDINARY_STREAM_LENGTH_FIELD,
                ordinary.len().to_string(),
            )
            .and(
                asked::THE_COMPLAINTS_STREAM_FIELD,
                b"/pkg/captured-streams/op/complaints".to_vec(),
            )
            .and(
                asked::THE_COMPLAINTS_STREAM_LENGTH_FIELD,
                complaints.len().to_string(),
            )
            .and(asked::THE_START_MARK_SEEN_FIELD, asked::THE_ANSWER_IS_YES)
            .and(
                asked::THE_RUN_WAS_ENDED_BY_SMOOZY_FIELD,
                asked::THE_ANSWER_IS_NO,
            );
        if let Some(status) = status {
            fields = fields.and(asked::THE_EXIT_STATUS_FIELD, status);
        }
        fields
    }

    fn holding(
        mut machine: AScriptedMachine,
        ordinary: Vec<u8>,
        complaints: Vec<u8>,
    ) -> AScriptedMachine {
        machine
            .streams
            .insert(b"/pkg/captured-streams/op/ordinary".to_vec(), ordinary);
        machine
            .streams
            .insert(b"/pkg/captured-streams/op/complaints".to_vec(), complaints);
        machine
    }

    fn one_reading() -> OneReading {
        OneReading::of(
            &at(b"/root/assets/files/Users/a/\xd0\xbb\xd0\xb5\xd0\xb2.blend"),
            &BlenderIdentity {
                path: "/root/blender/5.2.2/blender".to_string(),
                version: smoozy_server_setup::blender::Version {
                    major: 5,
                    minor: 2,
                    patch: 2,
                },
            },
        )
        .expect("randomness")
    }

    /// **G1 and `Render Settings` Q4: the check's reading is one request to the
    /// supervisor, asked again under its own identity until it ends — and
    /// nothing else.** No link is planted and no command is run: the machine
    /// here has no way to take either, so a reading that tried would not
    /// compile against it.
    #[tokio::test]
    async fn the_reading_is_one_request_asked_until_it_ends_and_its_document_is_read_back() {
        let written = serde_json::to_vec(&a_document()).expect("it renders");
        let (ordinary, complaints) = framed(&written, true);
        let mut machine = holding(
            AScriptedMachine::answering(vec![
                still_in_hand(),
                still_in_hand(),
                ended(&ordinary, &complaints, Some("0")),
            ]),
            ordinary,
            complaints,
        );
        let reading = one_reading();
        let left = drive_one_reading_within(
            &mut machine,
            &reading,
            Duration::from_secs(5),
            Duration::from_millis(1),
        )
        .await;
        assert_eq!(
            left,
            Ok(WhatTheReadingLeft::ASentinelFramedDocument(a_document()))
        );
        assert_eq!(machine.asked.len(), 3);
        for request in &machine.asked {
            assert_eq!(request.name, asked::THE_READING_OF_A_BLEND_REQUEST);
            assert_eq!(
                request.identity, reading.operation,
                "the same identity every time (C4)"
            );
            assert_eq!(
                request.fields.text(asked::THE_WHICH_READING_FIELD),
                Some(asked::THE_CHECKS_READING)
            );
            assert_eq!(
                request.fields.text(asked::THE_BUILD_FIELD),
                Some("/root/blender/5.2.2/blender")
            );
            assert_eq!(
                request.fields.value(asked::THE_BLEND_FIELD),
                Some(reading.of.as_bytes())
            );
            assert_eq!(
                request.fields.text(asked::THE_BLENDER_VERSION_FIELD),
                Some("5.2.2")
            );
        }
    }

    /// **The runner's ruling of 2026-09-28: a document over one frame's
    /// ceiling comes back whole, through the stream.** One frame holds at most
    /// `THE_LARGEST_FRAME` and a long animation's reading passes it; carried in
    /// the answer, it could never be read.
    #[tokio::test]
    async fn a_document_over_one_frames_ceiling_comes_back_whole_through_the_stream() {
        let mut document = a_document();
        let one = document.references[0].clone();
        let each = serde_json::to_vec(&one).expect("it renders").len();
        for _ in 0..=(smoozy_server_protocol::framing::THE_LARGEST_FRAME / each) {
            document.references.push(one.clone());
        }
        let written = serde_json::to_vec(&document).expect("it renders");
        assert!(written.len() > smoozy_server_protocol::framing::THE_LARGEST_FRAME);
        let (ordinary, complaints) = framed(&written, true);
        let mut machine = holding(
            AScriptedMachine::answering(vec![ended(&ordinary, &complaints, Some("0"))]),
            ordinary,
            complaints,
        );
        let left = drive_one_reading_within(
            &mut machine,
            &one_reading(),
            Duration::from_secs(5),
            Duration::from_millis(1),
        )
        .await;
        assert_eq!(
            left,
            Ok(WhatTheReadingLeft::ASentinelFramedDocument(document))
        );
    }

    /// **G3 over the answer's own fields**: a status other than nought, a
    /// status the machine could not reap, and a worker that was never started
    /// are all *a failing status*, never an empty reading.
    #[tokio::test]
    async fn an_ending_that_is_not_clean_is_a_failing_status_and_never_an_empty_reading() {
        let written = serde_json::to_vec(&a_document()).expect("it renders");
        let (ordinary, complaints) = framed(&written, true);
        for status in [Some("1"), None] {
            let mut machine = holding(
                AScriptedMachine::answering(vec![ended(&ordinary, &complaints, status)]),
                ordinary.clone(),
                complaints.clone(),
            );
            let left = drive_one_reading_within(
                &mut machine,
                &one_reading(),
                Duration::from_secs(5),
                Duration::from_millis(1),
            )
            .await;
            assert_eq!(left, Ok(WhatTheReadingLeft::AFailingStatus), "{status:?}");
        }
        let never_started = Fields::none()
            .and(
                asked::WHAT_BECAME_OF_IT_FIELD,
                asked::IT_HAS_A_RECORDED_OUTCOME,
            )
            .and(asked::THE_START_MARK_SEEN_FIELD, asked::THE_ANSWER_IS_NO)
            .and(
                asked::THE_MACHINES_OWN_WORDS_FIELD,
                "/root/blender could not be run",
            );
        let mut machine = AScriptedMachine::answering(vec![never_started]);
        assert_eq!(
            drive_one_reading_within(
                &mut machine,
                &one_reading(),
                Duration::from_secs(5),
                Duration::from_millis(1)
            )
            .await,
            Ok(WhatTheReadingLeft::AFailingStatus)
        );
    }

    /// **`Render Settings` Q8: a read Smoozy ended itself is none of the
    /// endings about the file**, and nothing is read back for it.
    #[tokio::test]
    async fn a_reading_smoozy_ended_says_so_and_claims_nothing_about_the_file() {
        let fields = Fields::none()
            .and(
                asked::WHAT_BECAME_OF_IT_FIELD,
                asked::IT_HAS_A_RECORDED_OUTCOME,
            )
            .and(
                asked::THE_RUN_WAS_ENDED_BY_SMOOZY_FIELD,
                asked::THE_ANSWER_IS_YES,
            );
        let mut machine = AScriptedMachine::answering(vec![fields]);
        assert_eq!(
            drive_one_reading_within(
                &mut machine,
                &one_reading(),
                Duration::from_secs(5),
                Duration::from_millis(1)
            )
            .await,
            Err(TheReadingDidNotRun::SmoozyEndedIt)
        );
    }

    /// **A program that does not know the request, a machine that did not take
    /// it, and a stream shorter than stated** are each their own reason and
    /// never a reading.
    #[tokio::test]
    async fn a_refusal_and_a_short_stream_are_each_their_own_reason() {
        let mut older = AScriptedMachine::answering(vec![still_in_hand()]);
        older.refused_by_name = true;
        assert_eq!(
            drive_one_reading_within(
                &mut older,
                &one_reading(),
                Duration::from_secs(5),
                Duration::from_millis(1)
            )
            .await,
            Err(TheReadingDidNotRun::TheMachinesProgramDoesNotTakeIt)
        );

        let refused = Fields::none()
            .and(
                asked::WHAT_BECAME_OF_IT_FIELD,
                asked::THERE_IS_NO_RECORD_OF_IT,
            )
            .and(
                asked::THE_MACHINES_OWN_WORDS_FIELD,
                "not admitted: a setup holds the machine",
            );
        let mut machine = AScriptedMachine::answering(vec![refused]);
        assert_eq!(
            drive_one_reading_within(
                &mut machine,
                &one_reading(),
                Duration::from_secs(5),
                Duration::from_millis(1)
            )
            .await,
            Err(TheReadingDidNotRun::TheMachineRefusedIt(
                "not admitted: a setup holds the machine".to_string()
            ))
        );

        let written = serde_json::to_vec(&a_document()).expect("it renders");
        let (ordinary, complaints) = framed(&written, true);
        let mut short = holding(
            AScriptedMachine::answering(vec![ended(&ordinary, &complaints, Some("0"))]),
            ordinary[..ordinary.len() - 1].to_vec(),
            complaints,
        );
        assert!(matches!(
            drive_one_reading_within(
                &mut short,
                &one_reading(),
                Duration::from_secs(5),
                Duration::from_millis(1)
            )
            .await,
            Err(TheReadingDidNotRun::TheStreamsCouldNotBeReadBack(_))
        ));
    }

    /// **The bound is a deadline on each ask** (G15's build detail,
    /// 2026-11-13): a machine that answered *still in hand* and then stops
    /// answering is given up on one bound after the ask it left unanswered,
    /// and the call returns rather than hanging. Until that date this test
    /// held a machine saying *still in hand* for ever to a bound over the
    /// whole reading.
    ///
    /// **A short bound and a short pace, because what is under test is the
    /// deadline and not its length**, which is [`THE_LONGEST_AN_ASK_WAITS`].
    #[tokio::test]
    async fn a_reading_whose_machine_stops_answering_is_given_up_at_the_bound() {
        struct ItStopsAnswering(usize);
        impl TheMachineAReadingIsAskedOf for ItStopsAnswering {
            async fn ask(&mut self, request: &Request) -> WhatCameBack {
                self.0 += 1;
                if self.0 > 3 {
                    return std::future::pending().await;
                }
                WhatCameBack::ItAnswered(
                    Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "0.1.7")
                        .carrying(still_in_hand()),
                )
            }
            async fn read_the_stream_back(&mut self, _: &[u8], _: u64) -> Result<Vec<u8>, String> {
                Err("no stream".to_string())
            }
        }
        let mut machine = ItStopsAnswering(0);
        assert_eq!(
            drive_one_reading_within(
                &mut machine,
                &one_reading(),
                Duration::from_millis(60),
                Duration::from_millis(5)
            )
            .await,
            Err(TheReadingDidNotRun::AnAskWentUnanswered)
        );
        assert_eq!(machine.0, 4, "it was asked again while it ran");
    }

    /// **The Creator's note 4 on the test copy of 2026-11-12, for the check's
    /// reading: a reading the machine is still running past the bound lands.**
    /// The machine answers every ask at once and says *still in hand* for well
    /// past the bound, then hands back a document. **A short bound and a short
    /// pace**, as above: what is under test is that the bound is on each ask
    /// and not on the whole reading.
    #[tokio::test]
    async fn a_reading_still_in_hand_past_the_bound_lands_while_the_machine_answers() {
        let written = serde_json::to_vec(&a_document()).expect("it renders");
        let (ordinary, complaints) = framed(&written, true);
        let mut answers = vec![still_in_hand(); 40];
        answers.push(ended(&ordinary, &complaints, Some("0")));
        let mut machine = holding(AScriptedMachine::answering(answers), ordinary, complaints);
        let started = std::time::Instant::now();
        let left = drive_one_reading_within(
            &mut machine,
            &one_reading(),
            Duration::from_millis(60),
            Duration::from_millis(5),
        )
        .await;
        assert_eq!(left, Ok(WhatTheReadingLeft::ASentinelFramedDocument(a_document())));
        assert!(started.elapsed() > Duration::from_millis(60), "{:?}", started.elapsed());
        assert_eq!(machine.asked.len(), 41);
    }

    /// **The read back over a real file session**, served by OpenSSH's own
    /// `sftp-server`, of a stream over one frame's ceiling — held to the length
    /// the machine stated, and an error in words where the file is shorter.
    ///
    /// **Where `sftp-server` is not on this system the test says so and
    /// stops.**
    #[cfg(unix)]
    #[tokio::test]
    async fn a_stream_is_read_back_over_a_real_file_session_to_its_stated_length() {
        use std::os::fd::OwnedFd;
        let Some(server) = [
            "/usr/libexec/sftp-server",
            "/usr/lib/openssh/sftp-server",
            "/usr/libexec/openssh/sftp-server",
            "/usr/lib/ssh/sftp-server",
        ]
        .into_iter()
        .find(|path| std::path::Path::new(path).exists()) else {
            println!(
                "SKIPPED: no OpenSSH sftp-server on this system. The read back is unproved here."
            );
            return;
        };
        let folder = std::env::temp_dir().join(format!(
            "smoozy-a-stream-{}",
            Identity::issue(IdentityKind::Operation)
                .expect("randomness")
                .as_text()
        ));
        std::fs::create_dir_all(&folder).expect("the folder");
        let stream: Vec<u8> = (0..(smoozy_server_protocol::framing::THE_LARGEST_FRAME + 12_345))
            .map(|at| (at % 251) as u8)
            .collect();
        let path = folder.join("ordinary");
        std::fs::write(&path, &stream).expect("the stream is written");

        let mut child = std::process::Command::new(server)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .spawn()
            .expect("sftp-server starts");
        let into: OwnedFd = child.stdin.take().expect("its input").into();
        let out: OwnedFd = child.stdout.take().expect("its output").into();
        let sender = tokio::net::unix::pipe::Sender::from_owned_fd(into).expect("a sender");
        let receiver = tokio::net::unix::pipe::Receiver::from_owned_fd(out).expect("a receiver");
        let mut subsystem = Subsystem::negotiate(tokio::io::join(receiver, sender))
            .await
            .expect("the session is negotiated");

        let path_bytes = path.display().to_string().into_bytes();
        let read = read_a_stream_back_over_the_file_session(
            &mut subsystem,
            &path_bytes,
            stream.len() as u64,
        )
        .await
        .expect("the whole stream is read back");
        assert_eq!(read, stream);
        let short = read_a_stream_back_over_the_file_session(
            &mut subsystem,
            &path_bytes,
            stream.len() as u64 + 1,
        )
        .await;
        assert!(
            short.is_err(),
            "a stream shorter than stated is never a short document"
        );
        // **`Queue & History` E8's ranged read**: only the bytes past the
        // length last read, up to the length stated now, across a frame's
        // ceiling — and nothing at all where nothing new stands. Piece 6c.
        let from = 70_000u64;
        let more = read_a_streams_bytes_over_the_file_session(
            &mut subsystem,
            &path_bytes,
            from,
            stream.len() as u64,
        )
        .await
        .expect("the new bytes are read");
        assert_eq!(more, stream[from as usize..]);
        let nothing_new = read_a_streams_bytes_over_the_file_session(
            &mut subsystem,
            &path_bytes,
            stream.len() as u64,
            stream.len() as u64,
        )
        .await
        .expect("nothing new is no error");
        assert!(nothing_new.is_empty());

        drop(subsystem);
        let _ = child.kill();
        let _ = child.wait();
        std::fs::remove_dir_all(&folder).ok();
    }

    /// **G13 again, from the other side**: the same file offered twice by two
    /// routes is read once.
    #[test]
    fn one_file_reached_two_ways_is_read_once() {
        let one = at(b"/root/assets/files/x.blend");
        let mut walk = TheLoop::over(one.clone());
        walk.and_below_it([one.clone(), one.clone()]);
        assert_eq!(walk.next_file(), Some(one));
        assert_eq!(walk.next_file(), None);
    }
}
