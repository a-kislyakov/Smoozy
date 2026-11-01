//! **One server's point: every session on its connections is admitted here and
//! lent out from here.**
//!
//! `Server protocol and transfers` T5: *"Every session on a connection is
//! admitted through one point per server"* — **every session, and not only the
//! transfer channels**, because `MaxSessions` is one pool per connection and
//! three things draw on it: the long-lived reading channel (§1.1 item 2 of that
//! document), one command invocation per request (§1.1 item 1), and one channel
//! per transfer (`Server protocol and transfers` T1). **Before this file the
//! connection a period held could be used by nothing but the period itself**
//! (`src/servers/a_period.rs`), and nothing counted its sessions.
//!
//! # What is lent, and how
//!
//! - **The reading session.** One file-transfer subsystem channel on the
//!   primary connection, opened at the first reading and held while the
//!   connection is live. **It is never handed out**: a reading is one operation
//!   run against it here, and the next reading waits its turn. That is what
//!   lets a command close it between two readings rather than in the middle of
//!   one.
//! - **Commands**, on the primary connection and never on any other. **At a
//!   learned limit of one, one at a time and each to its end** (T5's single
//!   lease, below). **Above one, or where no limit is learned, a command whose
//!   request has gone out runs beside the point**, which goes on serving
//!   readings, transfers and further commands while it runs — each command
//!   holding a session of its own until it ends. *(Until 2026-09-27 every
//!   command was run to its end here and nothing else was served meanwhile, so
//!   a 52-second driver install held back `Server Setup` Y7's refresh and the
//!   machine ended the setup run: step 10c's eighth pass on Hyperstack,
//!   eighth-pass fix piece 1.)*
//! - **A channel for one transfer**, handed to the caller and given back when
//!   it is dropped — on the primary where the pool has room, and otherwise on
//!   `Servers` S14's one additional connection, which carries transfers alone.
//!
//! **And one general way to ask the machine's supervisor something**,
//! [`ThisServersSessions::ask_the_supervisor`], which is
//! `smoozy_server_protocol::asking_the_supervisor` asking this point for its
//! three acts one at a time.
//!
//! # The rule this file exists to keep
//!
//! **`Server protocol and transfers` T5: at a learned limit of one, no slot is
//! reserved.** The primary connection then holds a single lease that alternates
//! between reading and a command: **a waiting command closes the reading
//! session after its current reply, takes the one session, and only then does
//! reading reopen, and no reading begins while a command waits.** Closing the
//! reading session destroys every handle open on it, and what an operation
//! holding one does about that is §1.1 item 2's re-open rule, not this file's —
//! **the lease does not wait for a handle to close**. A point that kept the
//! reading session open while a command waited would have broken the clause,
//! and
//! [`tests::at_a_learned_limit_of_one_a_waiting_command_closes_the_reading_session`]
//! is the test that says so by name.
//!
//! **Above a limit of one, one slot is reserved for commands**, and the reading
//! session is counted as held whether or not it is open, so a transfer can
//! never take the room a later reading or command needs.
//!
//! # What the limit is, and how it is learned
//!
//! **`Server protocol and transfers` T2: the limit is the server's, and it is
//! not readable before it is hit.** So it starts unknown and **a refused
//! session teaches it**: the limit is then the number of sessions this point
//! holds on that connection. **It is learned per connection and never carried
//! to the next one**, because a connection that replaced another may be to a
//! machine that was replaced under the address. Where a refusal comes with
//! nothing held at all, the machine permits no session (`Servers` S15), and
//! every ask is told so.
//!
//! **One refusal is not a lesson: the one that comes straight after this point
//! let a session go.** Closing a session is a message to the far end, and the
//! far end frees the slot only once it has handled it — so a session asked for
//! in that gap is refused by a machine whose limit has not moved. **Such a
//! refusal is asked again, a bounded number of times**, before it is taken as
//! the limit. That is a delay inside a sequence that is already running, and
//! P10b admits exactly that shape. **A fresh point starts in that state**,
//! because `Servers`' own sequence runs commands on the connection just
//! before it is lent here (step 10c piece 6's R1).
//!
//! # Where it runs
//!
//! **On the period's own thread, inside the wait for the connection to end.**
//! `Servers`' driving holds the live connection and does one thing with it for
//! as long as it is live: it waits for the transport to report that it ended.
//! **That wait is the one place the connection can be reached from, so the
//! point serves there**, and the connection's ending still wins: the moment
//! the transport reports it, the wait answers with it and everything this point
//! held goes with it. **P5 is kept by the same fact**: every session, request
//! and reading this file makes is made on that thread, and the asking side's
//! methods only send and wait for an answer.
//!
//! # What this file does not do
//!
//! - **It orders no transfers.** Which transfer goes first is the per-server
//!   pool's (`File Browser` O5 and the order it runs over) and the desktop-wide
//!   point's (`src/scheduling.rs`); a transfer reaches this point already
//!   admitted by both, and here it only waits for a session.
//! - **It raises no condition.** A request the supervisor did not answer is
//!   held against the transport's next report under
//!   `Server protocol and transfers` C17a by whoever raises the
//!   supervisor-unavailable condition.
//! - **It opens nothing while no connection is live.** An ask made then is
//!   told so at once, and an ask waiting when the connection ends is told it
//!   ended.
//!
//! **Every marker below that belongs to another document is written out with
//! that document at its own site, and this file keeps no header list** — the
//! documents it cites share most of their letters with this crate's own plan.

use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::time::Duration;

use smoozy_server_protocol::asking_the_supervisor::{self, TheTwoMechanismsLent, WhatCameBack};
use smoozy_server_protocol::carriage::{self, CarriageFailure, CarriageSite, PlantedLink};
use smoozy_server_protocol::framing::Request;
use smoozy_server_protocol::subsystem::client::Subsystem;
use smoozy_server_protocol::subsystem::SubsystemFailure;
use smoozy_server_protocol::transport::ssh::{self, CommandFailure};
use smoozy_server_protocol::transport::{CommandResult, CommandString, Ending};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio::sync::{mpsc, oneshot, watch};

/// A future that owns what it borrows for `'a`, and can cross threads.
pub type ItsOwnFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// A channel's bytes, both ways.
pub trait AStream: AsyncRead + AsyncWrite + Unpin + Send {}
impl<T: AsyncRead + AsyncWrite + Unpin + Send> AStream for T {}

/// How many times a refusal that came straight after a release is asked again.
///
/// **Not a figure of any plan's.** It bounds the gap between this point letting
/// a session go and the far end freeing its slot, which is a message and a
/// process ending on the machine — milliseconds on the loopback server this was
/// run against, and never measured on a rented machine. **Forty tries a
/// twentieth of a second apart is two seconds**, far above the gap and short
/// enough that a machine whose limit really did fall is believed quickly.
const THE_TRIES_AFTER_A_RELEASE: u32 = 40;
/// The pause between two of those tries.
const THE_PAUSE_AFTER_A_RELEASE: Duration = Duration::from_millis(50);

// ------------------------------------------------------ what a connection lends

/// Why no session was had.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoSession {
    /// **The machine would not open another** — `Server protocol and transfers`
    /// T2's refused channel, which teaches the limit.
    Refused,
    /// The connection went under.
    TheConnectionIsGone,
}

/// **What a held connection lends this point**, as a seam.
///
/// **It is a seam so the rule above is driven at the desk**: a judgement that
/// can only be reached through a socket is a judgement nobody tests. The real
/// end is `smoozy_server_protocol`'s own connection, below.
pub trait AHeldConnection: Send + 'static {
    /// One session's bytes.
    type Channel: AsyncRead + AsyncWrite + Unpin + Send + 'static;

    /// One file-transfer subsystem session.
    fn open_a_session(&mut self) -> impl Future<Output = Result<Self::Channel, NoSession>> + Send;

    /// **One command, in a session of its own that ends with it** —
    /// `Server protocol and transfers` §1.1 item 1, with one framed request on
    /// its standard input where there is one.
    fn a_command(
        &mut self,
        command: CommandString,
        frame: Option<Vec<u8>>,
    ) -> impl Future<Output = Result<CommandResult, NoSession>> + Send;

    /// **One command begun: its session opened and its request sent, with
    /// what it reads handed back to be run to its end away from the
    /// connection.** Added 2026-09-27, step 10c's eighth-pass fix piece 1.
    ///
    /// **It is what lets this point serve beside a running command** — see
    /// [`lend_until_it_ends`]. **A connection that cannot hand the rest back
    /// runs the whole command here**, which is what this seam did before and
    /// is what the provided answer does: the command is then over before the
    /// point serves anything else, as it always was.
    fn a_command_begun(
        &mut self,
        command: CommandString,
        frame: Option<Vec<u8>>,
    ) -> impl Future<Output = Result<ACommandRunning, NoSession>> + Send {
        async move {
            self.a_command(command, frame)
                .await
                .map(ACommandRunning::already_ended)
        }
    }

    /// **One command begun, with what it prints passed up as it prints** —
    /// `Server Setup` Y7a. Added 2026-10-18, the Creator's thirteenth notes,
    /// piece H.
    ///
    /// **A connection that cannot pass the pieces up as they arrive passes
    /// them up whole when the command ends**, which is what the provided answer
    /// does: the same bytes, each stream on its own, only later. The real
    /// connection passes them up as they arrive.
    fn a_command_begun_passing_up(
        &mut self,
        command: CommandString,
        frame: Option<Vec<u8>>,
        up: WhereThePiecesGo,
    ) -> impl Future<Output = Result<ACommandRunning, NoSession>> + Send {
        async move {
            self.a_command_begun(command, frame)
                .await
                .map(|begun| begun.passed_up_at_its_end(up))
        }
    }

    /// **The transport's own report that the connection ended.** Nothing is
    /// sent to learn it (`Servers` K10).
    fn until_it_ends(&mut self) -> impl Future<Output = Ending> + Send;

    /// **The same report, holding nothing of the connection**, so it can be
    /// heard while the connection itself is lent to a piece of work — see
    /// [`lend_until_it_ends`]. Added 2026-10-02, the fixes before 0.1.8,
    /// fault 1.
    ///
    /// **A connection that cannot give one is heard between pieces of work
    /// only**, which is what this point did before; the provided answer never
    /// arrives, and every stand-in connection at the desk keeps that.
    fn its_ending(&self) -> ItsOwnFuture<'static, Ending> {
        Box::pin(std::future::pending())
    }

    /// Smoozy's own tear-down.
    fn torn_down(self) -> impl Future<Output = Ending> + Send;
}

impl AHeldConnection for ssh::Connection {
    type Channel = Box<dyn AStream>;

    async fn open_a_session(&mut self) -> Result<Box<dyn AStream>, NoSession> {
        match self.open_a_subsystem_channel().await {
            Ok(stream) => Ok(Box::new(stream)),
            Err(failure) => Err(why_no_session(self, failure)),
        }
    }

    async fn a_command(
        &mut self,
        command: CommandString,
        frame: Option<Vec<u8>>,
    ) -> Result<CommandResult, NoSession> {
        let ran = match frame {
            None => self.run_one_command(command).await,
            Some(frame) => {
                self.run_one_command_carrying_a_framed_request_on_its_standard_input(
                    command, &frame,
                )
                .await
            }
        };
        ran.map_err(|failure| why_no_session(self, failure))
    }

    /// **The request sent here, and its reading handed back** — so the point
    /// serves on while a command runs. The channel is closed if the reading is
    /// given up before its end (`Server Setup` Y8).
    async fn a_command_begun(
        &mut self,
        command: CommandString,
        frame: Option<Vec<u8>>,
    ) -> Result<ACommandRunning, NoSession> {
        match self.one_command_begun(command, frame.as_deref()).await {
            Ok(under_way) => Ok(ACommandRunning::to_be_run(async move {
                Ok(under_way.to_its_end().await)
            })),
            Err(failure) => Err(why_no_session(self, failure)),
        }
    }

    /// **The request sent here, and its reading handed back telling each
    /// piece as it arrives** — `Server Setup` Y7a.
    async fn a_command_begun_passing_up(
        &mut self,
        command: CommandString,
        frame: Option<Vec<u8>>,
        up: WhereThePiecesGo,
    ) -> Result<ACommandRunning, NoSession> {
        match self.one_command_begun(command, frame.as_deref()).await {
            Ok(under_way) => Ok(ACommandRunning::to_be_run(async move {
                Ok(under_way
                    .to_its_end_told_as_it_comes(move |stream, bytes| {
                        let _ = up.send(APiecePrinted::from(stream, bytes));
                    })
                    .await)
            })),
            Err(failure) => Err(why_no_session(self, failure)),
        }
    }

    async fn until_it_ends(&mut self) -> Ending {
        ssh::Connection::wait_until_it_ends(self).await
    }

    fn its_ending(&self) -> ItsOwnFuture<'static, Ending> {
        Box::pin(ssh::Connection::its_ending(self))
    }

    async fn torn_down(self) -> Ending {
        ssh::Connection::tear_down(self).await
    }
}

/// **A command whose request has gone out** — what
/// [`AHeldConnection::a_command_begun`] hands back. Added 2026-09-27, step
/// 10c's eighth-pass fix piece 1.
pub struct ACommandRunning(WhereItIs);

enum WhereItIs {
    /// It was run whole where it was begun.
    Ended(CommandResult),
    /// **The rest of it, run to its end wherever it is polled.** Dropped
    /// before its end, it gives the command up, and the connection's own end
    /// closes the command's channel.
    ToBeRun(ItsOwnFuture<'static, Result<CommandResult, NoSession>>),
}

impl ACommandRunning {
    /// **A command already over**, with what it returned.
    pub fn already_ended(result: CommandResult) -> Self {
        ACommandRunning(WhereItIs::Ended(result))
    }

    /// **The rest of a command, to be run to its end.**
    pub fn to_be_run(
        rest: impl Future<Output = Result<CommandResult, NoSession>> + Send + 'static,
    ) -> Self {
        ACommandRunning(WhereItIs::ToBeRun(Box::pin(rest)))
    }

    /// **The same command, with what it printed passed up whole when it
    /// ends** — the provided answer of
    /// [`AHeldConnection::a_command_begun_passing_up`].
    fn passed_up_at_its_end(self, up: WhereThePiecesGo) -> Self {
        let pass = move |result: &CommandResult| {
            if !result.output.is_empty() {
                let _ = up.send(APiecePrinted::Output(result.output.clone()));
            }
            if !result.complaint.is_empty() {
                let _ = up.send(APiecePrinted::Complaint(result.complaint.clone()));
            }
        };
        match self.0 {
            WhereItIs::Ended(result) => {
                pass(&result);
                ACommandRunning(WhereItIs::Ended(result))
            }
            WhereItIs::ToBeRun(rest) => ACommandRunning::to_be_run(async move {
                let ran = rest.await;
                if let Ok(result) = &ran {
                    pass(result);
                }
                ran
            }),
        }
    }
}

/// **Some bytes a command printed, and the stream they came on** — passed up
/// as they arrive, `Server Setup` Y7a. Added 2026-10-18, the Creator's
/// thirteenth notes, piece H. **The two streams are two variants and never one
/// buffer**, so nothing downstream can merge them by accident (M13a).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum APiecePrinted {
    /// From the standard output.
    Output(Vec<u8>),
    /// From the complaint output.
    Complaint(Vec<u8>),
}

impl APiecePrinted {
    fn from(stream: ssh::TheStream, bytes: &[u8]) -> Self {
        match stream {
            ssh::TheStream::Output => APiecePrinted::Output(bytes.to_vec()),
            ssh::TheStream::Complaint => APiecePrinted::Complaint(bytes.to_vec()),
        }
    }
}

/// **Where a command's pieces go as they are printed.** Unbounded, because a
/// piece is told from inside the transport's own reading and that reading must
/// never wait on whoever keeps the words.
pub type WhereThePiecesGo = mpsc::UnboundedSender<APiecePrinted>;

/// **A session the machine would not open, told apart from a connection that
/// went under** by the transport's own report and by nothing the machine said.
///
/// The transport answers both with the same failure. **Where it has reported
/// the connection ended, the connection is gone; otherwise the machine
/// refused.** A connection dying at that very instant can be read as a refusal
/// before its ending is reported, and the cost is bounded: the limit learned
/// from it belongs to a connection whose ending arrives next.
fn why_no_session(connection: &ssh::Connection, failure: CommandFailure) -> NoSession {
    match failure {
        CommandFailure::NoChannelCouldBeOpened if connection.ended().is_none() => {
            NoSession::Refused
        }
        _ => NoSession::TheConnectionIsGone,
    }
}

// ------------------------------------------------------------ what an ask gets

/// Why an ask was not lent what it asked for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotLent {
    /// **No connection to this server is live.** No period stands, or it is
    /// between a break and a repair. Nothing is queued for later.
    NoConnectionIsLive,
    /// **The connection ended while the ask waited or ran.**
    TheConnectionEnded,
    /// **The connection ended after a command's request had gone out to the
    /// machine and before its answer came back.** The machine may have acted
    /// on it, and nothing here says either way (`Server protocol and
    /// transfers` C17a: the silence is not yet the supervisor's). Added
    /// 2026-10-19, the dead-line desk coverage: until then this was
    /// [`NotLent::TheConnectionEnded`], which a supervisor ask reads as *never
    /// asked*, so a cancel or a clear the machine had carried out was told to
    /// the user as *Smoozy could not reach the server. Nothing was changed.*
    TheConnectionEndedAfterTheRequestWentOut,
    /// **`Servers` S15: the machine permits no session at all**, so Smoozy has
    /// no route to it.
    TheMachineAllowsNoSession,
    /// The reading session was opened and the file-transfer server did not
    /// negotiate.
    TheReadingSessionCouldNotBeOpened(SubsystemFailure),
    /// **`Servers` S14 condition 3: the additional connection could not be
    /// made**, so the transfer is unavailable on this connection rather than
    /// failed.
    UnavailableOnThisConnection,
}

/// What the reading session is after one reading.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AfterTheReading {
    StillHeld,
    /// **The channel went under during it**, so it is dropped rather than
    /// lent again.
    Gone,
}

/// One reading, run here against the reading session.
type AReading<S> = Box<
    dyn for<'a> FnOnce(Result<&'a mut Subsystem<S>, NotLent>) -> ItsOwnFuture<'a, AfterTheReading>
        + Send,
>;

/// Which connection a transfer's channel was seated on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SeatedOn {
    ThePrimary,
    /// **The additional connection, by which one** — a later one is a
    /// different connection, and a seat given back to one already gone is
    /// nobody's.
    TheAdditional(u64),
}

/// **One act that takes the whole connection**, run here against it.
///
/// It answers through a channel it carries itself, so the queue holds one shape
/// whatever the act comes back with.
type AWholeConnectionAct<C> = Box<dyn for<'a> FnOnce(&'a mut C) -> ItsOwnFuture<'a, ()> + Send>;

/// Everything the point is asked, in one queue.
enum AnAsk<C: AHeldConnection> {
    AReading {
        someone_is_waiting: bool,
        reading: AReading<C::Channel>,
    },
    ACommand(ACommand),
    ATransfer(oneshot::Sender<Result<ALentChannel<C>, NotLent>>),
    ATransferEnded(SeatedOn),
    TheAdditionalCameBack(Option<C>),
    /// **The whole connection, for one act that needs both mechanisms at
    /// once** — `Server protocol and transfers` §1.1's two, held together for
    /// the length of one transaction.
    TheWholeConnection(AWholeConnectionAct<C>),
}

struct ACommand {
    command: CommandString,
    frame: Option<Vec<u8>>,
    answer: oneshot::Sender<Result<CommandResult, NotLent>>,
    /// **Where what it prints goes as it prints**, where anyone asked —
    /// `Server Setup` Y7a. Added 2026-10-18.
    up: Option<WhereThePiecesGo>,
}

/// **A channel for one transfer**, lent until it is dropped.
///
/// It reads and writes as the channel it holds. **Dropping it gives the seat
/// back**, and a transfer waiting for one is seated then. **It is dropped
/// inside the runtime the application already runs**, because closing a
/// channel is itself a message to the machine.
///
/// **And it learns when the connection it was seated on has ended** — the
/// primary, or `Servers` S14's additional connection, whichever
/// [`SeatedOn`] names. From that moment every wait on it — a write waiting
/// for room, a read waiting for an answer — is answered at once with an
/// error, which the file-transfer subsystem reads as *the channel is gone*
/// and the byte loop as *interrupted* (`Server protocol and transfers` C18:
/// *"A deadline is never a substitute for the connection's own break
/// detection"*). Added 2026-10-01, step 10c's twelfth-pass fix piece 1.
///
/// **The case this was first written for** is a connection that ends by
/// closing: the machine powered off, or the line refusing, where the
/// transport's own channel sees the close and every wait on it ends. **The
/// case it now also meets** is a connection that ends in silence — a
/// provider's firewall dropping every packet. `Servers` K3 then declares the
/// break from the unanswered keepalive, but the transport library wakes a
/// write waiting for room only when the machine opens the window again, never
/// when the connection ends (`russh` 0.63.1, `channels/io/tx.rs`,
/// `poll_writable`). **So on the twelfth pass a 1 GB upload waited for room on
/// a connection that was already gone and already replaced**, its request
/// under way for good. **That was the moment the record and the world parted
/// company**: the point had let the connection go, and the channel lent on it
/// still said nothing had happened.
pub struct ALentChannel<C: AHeldConnection> {
    channel: C::Channel,
    seat: TheSeat<C>,
    its_connection: TheConnectionEnds,
    /// **The same ending, for the transfer that held this channel to ask
    /// after once the channel has failed** — see
    /// [`ALentChannel::the_connection_it_sits_on`]. The dead-line sweep's
    /// extra-connection repair, 2026-10-19.
    asked_after: watch::Receiver<bool>,
}

/// **Held by whoever holds a connection, for as long as the point lends on
/// it**; dropped, every channel lent on that connection learns at once.
/// Added 2026-10-01, step 10c's twelfth-pass fix piece 1.
///
/// **It says *ended* in two ways, and a channel hears either.** Dropped by
/// the point — the lending over, or `Servers` S14's additional connection let
/// go — and, for the additional connection, **its own transport reporting it
/// ended** ([`Self::ends_with`]). Until the dead-line sweep's extra-connection repair
/// (2026-10-19) only the first was heard: **an additional connection that
/// died on its own, the primary still answering, told none of its channels**,
/// and a write waiting for room on it waited for good.
struct WhileTheConnectionStands(Arc<watch::Sender<bool>>);

impl WhileTheConnectionStands {
    fn begins() -> Self {
        WhileTheConnectionStands(Arc::new(watch::channel(false).0))
    }

    /// **What a channel lent on this connection watches.**
    fn watched(&self) -> TheConnectionEnds {
        TheConnectionEnds::of(self.0.subscribe())
    }

    /// **The same, as a transfer asks after it.**
    fn asked_after(&self) -> watch::Receiver<bool> {
        self.0.subscribe()
    }

    /// **The connection's own ending is heard here too**, whoever holds it:
    /// the moment its transport reports it ended, every channel lent on it is
    /// told, without waiting for the point to hear it. **The watch ends with
    /// the connection's standing**, however that ends, so nothing outlives it.
    fn ends_with(&self, its_ending: ItsOwnFuture<'static, Ending>) {
        let told = Arc::clone(&self.0);
        let mut over = self.0.subscribe();
        tokio::spawn(async move {
            tokio::select! {
                _ = its_ending => {
                    told.send_replace(true);
                }
                _ = over.wait_for(|ended| *ended) => {}
            }
        });
    }
}

impl Drop for WhileTheConnectionStands {
    fn drop(&mut self) {
        self.0.send_replace(true);
    }
}

/// **The connection a lent channel sat on, as the transfer that held the
/// channel asks after it** — whether it has ended. Added 2026-10-19, the
/// dead-line sweep's extra-connection repair.
///
/// **What it is for.** A transfer whose channel failed asks why: the line
/// going takes the primary connection, and `crate::downloading::the_line_went_down`
/// sees the point stop lending. **`Servers` S14's additional connection can
/// die on its own while the primary answers** (`Servers` S14 condition 6: *a
/// transfer interruption and never a connection break*), and then the point
/// goes on lending and that look sees nothing — so a transfer cut that way
/// ended where the next one would have opened a fresh additional connection
/// and carried on. This is the look that sees it.
#[derive(Clone)]
pub struct TheConnectionItSatOn(watch::Receiver<bool>);

impl TheConnectionItSatOn {
    /// **Whether the connection has ended** — let go by the point, or
    /// reported ended by its own transport. Nothing is sent to learn it
    /// (`Servers` K10).
    pub fn has_ended(&self) -> bool {
        *self.0.borrow() || self.0.has_changed().is_err()
    }
}

/// **The ending of the connection a lent channel sits on**, as a lent channel
/// watches it: ready once the connection's [`WhileTheConnectionStands`] says
/// it ended, or is dropped.
struct TheConnectionEnds {
    ends: ItsOwnFuture<'static, ()>,
    ended: bool,
}

impl TheConnectionEnds {
    fn of(mut watched: watch::Receiver<bool>) -> Self {
        TheConnectionEnds {
            // **Ready on *ended*, or on its sender going** — both are the
            // connection no longer being lent.
            ends: Box::pin(async move {
                let _ = watched.wait_for(|ended| *ended).await;
            }),
            ended: false,
        }
    }

    /// **Whether the connection has ended**, asked on every wait — and the
    /// waiting task is woken when it does, so a wait that nothing else would
    /// ever end is ended by this.
    fn has_ended(&mut self, cx: &mut Context<'_>) -> bool {
        if !self.ended && self.ends.as_mut().poll(cx).is_ready() {
            self.ended = true;
        }
        self.ended
    }
}

/// What a wait on a channel whose connection has ended is answered with.
fn the_connection_has_ended() -> std::io::Error {
    std::io::Error::new(
        std::io::ErrorKind::BrokenPipe,
        "the connection this channel was on has ended",
    )
}

struct TheSeat<C: AHeldConnection> {
    on: SeatedOn,
    tell: mpsc::UnboundedSender<AnAsk<C>>,
}

impl<C: AHeldConnection> Drop for TheSeat<C> {
    fn drop(&mut self) {
        let _ = self.tell.send(AnAsk::ATransferEnded(self.on));
    }
}

impl<C: AHeldConnection> ALentChannel<C> {
    /// **Whether it rides `Servers` S14's additional connection** rather than
    /// the primary.
    pub fn is_on_the_additional_connection(&self) -> bool {
        matches!(self.seat.on, SeatedOn::TheAdditional(_))
    }

    /// **The connection this channel sits on, to be asked after once the
    /// channel has failed** — see [`TheConnectionItSatOn`].
    pub fn the_connection_it_sits_on(&self) -> TheConnectionItSatOn {
        TheConnectionItSatOn(self.asked_after.clone())
    }
}

impl<C: AHeldConnection> AsyncRead for ALentChannel<C> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        if self.its_connection.has_ended(cx) {
            return Poll::Ready(Err(the_connection_has_ended()));
        }
        Pin::new(&mut self.channel).poll_read(cx, buf)
    }
}

impl<C: AHeldConnection> AsyncWrite for ALentChannel<C> {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        if self.its_connection.has_ended(cx) {
            return Poll::Ready(Err(the_connection_has_ended()));
        }
        Pin::new(&mut self.channel).poll_write(cx, buf)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        if self.its_connection.has_ended(cx) {
            return Poll::Ready(Err(the_connection_has_ended()));
        }
        Pin::new(&mut self.channel).poll_flush(cx)
    }

    /// **Nothing is waited for on a connection that has ended**: there is no
    /// one left to tell.
    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        if self.its_connection.has_ended(cx) {
            return Poll::Ready(Ok(()));
        }
        Pin::new(&mut self.channel).poll_shutdown(cx)
    }
}

// ------------------------------------------------------------- the asking side

/// **One server's point, as the rest of the application reaches it.**
///
/// Made once per period and kept by the run's register
/// (`crate::servers::a_period::ThePeriodsThisRun::the_sessions_of`). **It holds
/// no connection**: while one is live it holds the way to ask the thread that
/// does, and while none is, it holds nothing and every ask is told so.
pub struct ThisServersSessions<C: AHeldConnection = ssh::Connection> {
    live: Arc<Mutex<Option<mpsc::UnboundedSender<AnAsk<C>>>>>,
    /// **One supervisor ask at a time on this point**, for the whole of its
    /// three acts — see [`Self::ask_the_supervisor`] — **and no placement
    /// beside one**, see [`Self::the_whole_connection`]: both reach the
    /// machine's program through the one entry-point link.
    one_supervisor_ask: Arc<tokio::sync::Mutex<()>>,
    /// **`Server protocol and transfers` §1.1's entry-point link as it stands
    /// on the live connection** — planted by one supervisor ask and found by
    /// the asks after it, so each of those is one command. Added 2026-10-26,
    /// the worldwide pass's setup faults, fault 1. See [`TheLinkAsItStands`].
    the_link_stands: Arc<Mutex<TheLinkAsItStands>>,
}

/// **The entry-point link this point planted, and on which lending.**
///
/// **Why it is kept, which is `Server protocol and transfers` §1.1 as changed
/// on 2026-10-26.** Until then every supervisor ask planted the link (three
/// file-transfer requests: the folder, the old link away, the new one), ran its
/// command, and took the link away (a fourth) — four round trips of the line
/// before and after one command of about two. **On a far server that made
/// every ask about six crossings of the line**: 3.6 seconds an ask at 300 ms
/// each way at the desk, and the setup card's one-second refresh landed every
/// 4.6 seconds — the worldwide pass on Vast watched its words arrive in batches
/// every 3 to 12 seconds. **The link points at the package area, which does
/// not move for an installation**, so the ask after it finds the same link and
/// goes straight to its command.
///
/// **It is planted again whenever it may not stand**: on every new lending
/// (a connection that ended may have met a machine that restarted, or a
/// different one), after every act that takes the whole connection (a
/// placement plants and takes away a link at the same site), after an ask
/// whose command did not end cleanly (a link someone removed answers with no
/// program), and for another package area. **A plant replaces whatever stands
/// there first**, as it always has, so a link left on the machine by an ended
/// connection is replaced by the next connection's first ask.
#[derive(Debug, Default)]
struct TheLinkAsItStands {
    /// **Which lending of this point is live**, counted from the first.
    lending: u64,
    /// **The link planted on a lending, and what it points at**; nothing where
    /// none may stand.
    planted: Option<(u64, PlantedLink)>,
}

impl TheLinkAsItStands {
    /// **Whether a link at `site` pointing at `target` stands on the lending
    /// that is live.**
    fn stands(&self, site: CarriageSite, target: &[u8]) -> bool {
        self.planted
            .as_ref()
            .is_some_and(|(on, link)| *on == self.lending && link.site == site && link.target == target)
    }
}

impl<C: AHeldConnection> Clone for ThisServersSessions<C> {
    fn clone(&self) -> Self {
        ThisServersSessions {
            live: Arc::clone(&self.live),
            one_supervisor_ask: Arc::clone(&self.one_supervisor_ask),
            the_link_stands: Arc::clone(&self.the_link_stands),
        }
    }
}

impl<C: AHeldConnection> Default for ThisServersSessions<C> {
    fn default() -> Self {
        ThisServersSessions::for_one_server()
    }
}

impl<C: AHeldConnection> ThisServersSessions<C> {
    /// A point with no connection live yet.
    pub fn for_one_server() -> Self {
        ThisServersSessions {
            live: Arc::new(Mutex::new(None)),
            one_supervisor_ask: Arc::new(tokio::sync::Mutex::new(())),
            the_link_stands: Arc::new(Mutex::new(TheLinkAsItStands::default())),
        }
    }

    /// Whether a connection is live and being lent from.
    pub fn a_connection_is_live(&self) -> bool {
        self.live.lock().map(|live| live.is_some()).unwrap_or(false)
    }

    fn ask(&self, ask: AnAsk<C>) -> Result<(), NotLent> {
        let live = self.live.lock().map_err(|_| NotLent::NoConnectionIsLive)?;
        match live.as_ref() {
            Some(tell) => tell.send(ask).map_err(|_| NotLent::TheConnectionEnded),
            None => Err(NotLent::NoConnectionIsLive),
        }
    }

    /// **One reading on the reading session.**
    ///
    /// `someone_is_waiting` is `Server protocol and transfers` §1.1 item 2's
    /// ordering: **a reading the user is waiting on goes before one nobody is
    /// waiting on**, and otherwise readings go in the order they were asked.
    /// **A reading that holds a handle across two asks must expect it to be
    /// gone** — a command may close the session between them, and §1.1 item 2
    /// has it re-open rather than re-use.
    pub async fn a_reading<T, F>(
        &self,
        someone_is_waiting: bool,
        reading: F,
    ) -> Result<Result<T, SubsystemFailure>, NotLent>
    where
        T: Send + 'static,
        F: for<'a> FnOnce(
                &'a mut Subsystem<C::Channel>,
            ) -> ItsOwnFuture<'a, Result<T, SubsystemFailure>>
            + Send
            + 'static,
    {
        let (tell, told) = oneshot::channel();
        let reading: AReading<C::Channel> = Box::new(move |lent| {
            Box::pin(async move {
                match lent {
                    Err(why) => {
                        let _ = tell.send(Err(why));
                        AfterTheReading::StillHeld
                    }
                    Ok(subsystem) => {
                        let read = reading(subsystem).await;
                        let after = if matches!(read, Err(SubsystemFailure::TheChannelIsGone)) {
                            AfterTheReading::Gone
                        } else {
                            AfterTheReading::StillHeld
                        };
                        let _ = tell.send(Ok(read));
                        after
                    }
                }
            })
        });
        self.ask(AnAsk::AReading {
            someone_is_waiting,
            reading,
        })?;
        told.await.unwrap_or(Err(NotLent::TheConnectionEnded))
    }

    /// **One command**, with one framed request on its standard input where
    /// `frame` carries one.
    pub async fn a_command(
        &self,
        command: CommandString,
        frame: Option<Vec<u8>>,
    ) -> Result<CommandResult, NotLent> {
        self.a_command_carried(command, frame, None).await
    }

    /// **One command, with what it prints passed up to `up` as it prints** —
    /// `Server Setup` Y7a: *"An elevated step's two streams are read by the
    /// desktop as they are printed"*. Added 2026-10-18, the Creator's
    /// thirteenth notes, piece H. **The answer is the same whole result
    /// [`Self::a_command`] gives**; every piece is sent before it.
    pub async fn a_command_passing_up(
        &self,
        command: CommandString,
        frame: Option<Vec<u8>>,
        up: WhereThePiecesGo,
    ) -> Result<CommandResult, NotLent> {
        self.a_command_carried(command, frame, Some(up)).await
    }

    async fn a_command_carried(
        &self,
        command: CommandString,
        frame: Option<Vec<u8>>,
        up: Option<WhereThePiecesGo>,
    ) -> Result<CommandResult, NotLent> {
        let (answer, told) = oneshot::channel();
        self.ask(AnAsk::ACommand(ACommand {
            command,
            frame,
            answer,
            up,
        }))?;
        told.await.unwrap_or(Err(NotLent::TheConnectionEnded))
    }

    /// **The whole connection, lent for the length of one transaction, and
    /// nothing else lent while it stands.**
    ///
    /// **It exists for one act and the act is the package placement.**
    /// `Server protocol and transfers` §5's transaction holds a negotiated
    /// file-transfer session and runs user-level commands **inside** it — the
    /// health exchange plants a link, runs the entry point and takes the link
    /// away — so it cannot be a reading and a command taking turns. **Handing
    /// it the connection whole is what keeps T5 rather than what works around
    /// it**: every session it opens is still one this point let it have, and
    /// no other session is lent while it stands.
    ///
    /// **The reading session is closed first.** At a learned limit of one a
    /// transaction that met a reading session still open would be refused its
    /// own, and the refusal would say nothing about why.
    ///
    /// **What it costs is stated rather than hidden**: every reading, command
    /// and transfer asked while it stands waits, including the standing
    /// repair's. A placement is a bounded act on a machine the user is setting
    /// up, and the alternative — two sessions from two points on one
    /// connection — is the thing T5 exists to forbid.
    ///
    /// **It takes the supervisor asks' turn for its whole length. Added
    /// 2026-09-30, step 10c's eleventh-pass fix piece 4c.** The placement's
    /// health exchange, and its ask that the running supervisor end, plant and
    /// take away the same entry-point link every supervisor ask plants
    /// (`Server protocol and transfers` §1.1), and this act goes before every
    /// command waiting. **So a placement asked between an ask's link and its
    /// command took that link away**, and the ask's command found no program:
    /// nothing correlated came back, which a job send reads as *could not
    /// tell*. Watched at the desk
    /// (`a_placement_does_not_come_between_a_supervisor_asks_link_and_its_command`).
    /// **What it costs**: a placement waits for a supervisor ask already under
    /// way to end — one short request — and asks wait for the placement, as
    /// every reading and command already did.
    pub async fn the_whole_connection<T, F>(&self, act: F) -> Result<T, NotLent>
    where
        T: Send + 'static,
        F: for<'a> FnOnce(&'a mut C) -> ItsOwnFuture<'a, T> + Send + 'static,
    {
        let _no_supervisor_ask_beside = self.one_supervisor_ask.lock().await;
        let (tell, told) = oneshot::channel();
        let carried: AWholeConnectionAct<C> = Box::new(move |connection| {
            Box::pin(async move {
                let came_to = act(connection).await;
                let _ = tell.send(came_to);
            })
        });
        let asked = self.ask(AnAsk::TheWholeConnection(carried));
        let came_to = match asked {
            Ok(()) => told.await.map_err(|_| NotLent::TheConnectionEnded),
            Err(not_lent) => Err(not_lent),
        };
        // **A placement plants and takes away a link at the entry point's own
        // site**, so the link the asks before it left is not taken to stand
        // after it (2026-10-26, fault 1).
        self.the_link_may_not_stand();
        came_to
    }

    /// **The entry-point link is not taken to stand any more**, so the next
    /// supervisor ask plants it again. Added 2026-10-26, fault 1.
    fn the_link_may_not_stand(&self) {
        if let Ok(mut stands) = self.the_link_stands.lock() {
            stands.planted = None;
        }
    }

    /// **A channel for one transfer**, lent until it is dropped.
    pub async fn a_transfer_channel(&self) -> Result<ALentChannel<C>, NotLent> {
        let (answer, told) = oneshot::channel();
        self.ask(AnAsk::ATransfer(answer))?;
        told.await.unwrap_or(Err(NotLent::TheConnectionEnded))
    }

    /// **One request to the machine's supervisor, and the answer that carries
    /// its identity** — `smoozy_server_protocol::asking_the_supervisor`, asking
    /// this point for the reading session twice and a command once.
    ///
    /// **The package area is the caller's**, from the server row's recorded
    /// root, because it is `Job files and outputs` L4a's per installation and
    /// this point knows nothing about installations.
    ///
    /// **One ask at a time, for the whole of its three acts. Added 2026-09-27,
    /// step 10c's eighth-pass fix piece 4.** Every ask plants the one link
    /// `Server protocol and transfers` §1.1 names, runs the one constant
    /// command through it, and takes the link away — and taking it away
    /// removes whatever link stands there, **including one another ask has
    /// just planted.** The three acts are three separate asks of this point,
    /// so without this two asks could interleave: one planted, the other took
    /// its own link away, and the first's command found no program and never
    /// reached the supervisor. **That was the eighth pass's stall on
    /// Hyperstack** — the job's material on the machine, nothing from the desk
    /// in the supervisor's journal, *could not tell* — reproduced at the desk
    /// beside `Desktop app and delivery` P10d's own asks
    /// (`tests/a_job_is_submitted_through_the_servers_own_point.rs`).
    ///
    /// **Nothing else waits on it but a placement** ([`Self::the_whole_connection`],
    /// step 10c's eleventh-pass fix piece 4c): readings, transfers and every other
    /// command are served as before, and at a learned limit of one T5's single
    /// lease still alternates between the three acts exactly as it did.
    /// **What it costs** is that two supervisor asks to one server take turns
    /// rather than overlap. Each is a short request the supervisor answers
    /// promptly — a setup's worker steps are commanded and then read, never
    /// waited on inside one ask — so nothing that must go on beside a long
    /// command, `Server Setup` Y7's refresh included, is held behind another
    /// ask for more than one ask's length.
    ///
    /// **From 2026-10-26 the link stands between asks on one connection**
    /// (`Server protocol and transfers` §1.1 as changed that day, the worldwide
    /// pass's setup faults, fault 1), so an ask that finds it is one command
    /// and no file-transfer request at all — see [`TheLinkAsItStands`]. **The
    /// turn is still one ask at a time**: a link being planted or replaced is
    /// still a link another ask's command could miss.
    pub async fn ask_the_supervisor(&self, package_area: &str, request: &Request) -> WhatCameBack {
        let _one_at_a_time = self.one_supervisor_ask.lock().await;
        let mut lent = TheTwoMechanismsHere {
            sessions: self,
            planted: None,
        };
        asking_the_supervisor::ask_the_supervisor(&mut lent, package_area, request).await
    }
}

/// **The sender's three acts, asked of this point.** It keeps the link it
/// planted, so the link it takes away is its own.
struct TheTwoMechanismsHere<'s, C: AHeldConnection> {
    sessions: &'s ThisServersSessions<C>,
    planted: Option<PlantedLink>,
}

impl<C: AHeldConnection> TheTwoMechanismsLent for TheTwoMechanismsHere<'_, C> {
    async fn plant(&mut self, site: CarriageSite, target: Vec<u8>) -> bool {
        // **The link an earlier ask on this connection planted is found and
        // nothing is sent** (2026-10-26, fault 1).
        let standing = self.sessions.the_link_stands.lock().ok().and_then(|stands| {
            stands
                .stands(site, &target)
                .then(|| stands.planted.as_ref().map(|(_, link)| link.clone()))
                .flatten()
        });
        if let Some(standing) = standing {
            // **Held as this ask's own**, so where its command does not end
            // cleanly the take-away below removes it exactly as it removed a
            // link this ask planted.
            self.planted = Some(standing);
            return true;
        }
        let lending = self.sessions.the_link_stands.lock().map(|stands| stands.lending).ok();
        let planted = self
            .sessions
            .a_reading(true, move |subsystem| {
                Box::pin(async move {
                    carriage::plant(subsystem, site, &target)
                        .await
                        .map_err(|failure| match failure {
                            CarriageFailure::TheDirectoryCouldNotBeMade(why)
                            | CarriageFailure::TheLinkCouldNotBePlanted(why) => why,
                        })
                })
            })
            .await;
        match planted {
            Ok(Ok(planted)) => {
                // **Kept as standing on the lending it was planted on**, for
                // the asks after this one.
                if let (Some(lending), Ok(mut stands)) = (lending, self.sessions.the_link_stands.lock()) {
                    stands.planted = Some((lending, planted.clone()));
                }
                self.planted = Some(planted);
                true
            }
            _ => false,
        }
    }

    /// **The link is left standing for the asks after this one on this
    /// connection** — `Server protocol and transfers` §1.1 as changed on
    /// 2026-10-26 — **and taken away only where it may not stand**: where this
    /// ask's command did not end cleanly, which is the one case its record was
    /// dropped — whether this ask planted it or found it standing, so a failed
    /// ask goes on exactly as every ask did before that day. A removal that
    /// could not run is kept as owed, as before, and the next plant replaces
    /// it.
    async fn take_it_away(&mut self) {
        let standing = self.sessions.the_link_stands.lock().is_ok_and(|stands| stands.planted.is_some());
        if standing {
            self.planted = None;
            return;
        }
        let Some(planted) = self.planted.take() else {
            return;
        };
        let kept = planted.clone();
        let taken = self
            .sessions
            .a_reading(true, move |subsystem| {
                Box::pin(async move {
                    let mut planted = planted;
                    carriage::take_it_away(subsystem, &mut planted).await;
                    Ok(planted)
                })
            })
            .await;
        // **A removal that could not run is kept as owed rather than
        // forgotten**, and the next plant at the same site replaces it.
        self.planted = match taken {
            Ok(Ok(planted)) if !planted.the_removal_is_owed() => None,
            Ok(Ok(planted)) => Some(planted),
            _ => Some(kept),
        };
    }

    async fn one_command_carrying_a_frame(
        &mut self,
        command: CommandString,
        frame: Vec<u8>,
    ) -> Option<CommandResult> {
        let came_to = self.sessions.a_command(command, Some(frame)).await;
        // **A command that did not end cleanly may have met no link** — one
        // somebody removed, or a machine that restarted under it — so the next
        // ask plants it again (2026-10-26, fault 1).
        if !came_to.as_ref().is_ok_and(CommandResult::ended_cleanly) {
            self.sessions.the_link_may_not_stand();
        }
        match came_to {
            Ok(said) => Some(said),
            // **The request went out and its answer never came back**: the
            // command ran as far as anyone here can tell, and it ended with no
            // clean ending — which the sender reads as *nothing correlated
            // came back*, never as *never asked*. The dead-line desk coverage,
            // 2026-10-19: a cancel the machine carried out, its answer lost to
            // a line gone silent, was told as *Nothing was changed.*
            Err(NotLent::TheConnectionEndedAfterTheRequestWentOut) => Some(CommandResult {
                output: Vec::new(),
                complaint: Vec::new(),
                status: None,
            }),
            Err(_) => None,
        }
    }
}

// ------------------------------------------------------------- the lending side

/// **What opens `Servers` S14's additional connection**, when the point asks.
///
/// **`Servers` owns opening it** — `Servers` S14, and `Servers` Q16: it runs
/// `Servers` Q1 steps 1 to 3 and no others — and this point owns only when it
/// is asked for and what it carries. **Nothing** where it could not be made.
pub type TheAdditionalConnection<C> =
    Arc<dyn Fn() -> ItsOwnFuture<'static, Option<C>> + Send + Sync>;

/// **Lends one live connection until the transport reports that it ended**,
/// and answers with that ending.
///
/// **This is the wait `Servers`' driving makes on a live connection**, so the
/// connection's ending is the one thing that ends it — and when it does, the
/// reading session, the additional connection and every ask still waiting go
/// with it. **The ending is always asked first**, so no ask is served on a
/// connection the transport has already reported gone — **and it is heard
/// while a piece of work is under way too**, which gives that work up
/// (the fixes before 0.1.8, fault 1).
pub async fn lend_until_it_ends<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    connection: &mut C,
    the_additional: &TheAdditionalConnection<C>,
) -> Ending {
    // **Every channel lent on this connection learns when the lending ends**
    // — step 10c's twelfth-pass fix piece 1. **Made first so it goes last**:
    // the asking side sees no live connection before a channel is told its
    // connection has gone, so a transfer that ends on that news finds the
    // line down (`crate::downloading::the_line_went_down`) and waits for it
    // rather than ending.
    let the_primary_stands = WhileTheConnectionStands::begins();
    let (tell, mut asks) = mpsc::unbounded_channel();
    let mut point = ThePoint::for_one_connection(tell.clone(), &the_primary_stands);
    // **Commands running beside the point's own work**, polled here and on
    // this thread and nowhere else (P5). Added 2026-09-27, step 10c's
    // eighth-pass fix piece 1.
    let mut beside = CommandsBeside::default();
    // **Made after the point and the commands beside, so it goes before
    // them**: when the lending ends, the asking side sees no live connection
    // before any ask still waiting is answered that the connection went —
    // which is what a caller weighing a lost answer reads (`crate::managing_files`'
    // copy: *never answered* only where the point no longer lends). Moved
    // 2026-10-26, the worldwide pass's setup faults, fault 1: until then the
    // answers went first, and only the take-away that followed every ask gave
    // the lending time to end before the caller looked; with the entry point's
    // link standing there is no take-away, and a copy cut with its bytes read
    // *refused by the server* under load.
    let _live = WhileItIsLive::begins(sessions, tell);
    // **The connection's end, heard while a piece of work holds the connection
    // too** — the fixes before 0.1.8, fault 1. Until then the end was heard only
    // here at the top, between two pieces of work, so a piece of work that
    // never ended — a reading waiting on an answer a stopped transport would
    // never hand it — kept the point from ever hearing it: the transport had
    // declared the break, and the server went on reading *connected*, the
    // Files page *Reading*, and a setup *waiting for it to restart*, for as
    // long as the run lasted (the check before 0.1.8 on Hyperstack).
    let mut its_ending = connection.its_ending();
    loop {
        let there_is_work = point.there_is_work_it_can_do();
        tokio::select! {
            biased;
            ending = connection.until_it_ends() => return ending,
            // **`Servers` S14's additional connection ending on its own**,
            // the primary still up — the dead-line sweep's extra-connection repair. Its
            // channels were told by its own watch the moment its transport
            // reported it; here the point lets it go, so the next transfer
            // opens a fresh one (`Servers` S14 condition 6) rather than
            // asking a connection that is gone.
            () = point.the_additional_has_ended() => point.the_additional_is_gone(),
            Some(ask) = asks.recv() => point.arrives(ask),
            ended = beside.some_end() => point.commands_beside_ended(ended),
            _ = std::future::ready(()), if there_is_work => {
                // **The commands beside go on while the point does its one
                // thing** — a reading, a transfer's seat or a command begun —
                // so a command's output is read while a walk holds the reading
                // session, and nothing it returns backs up behind the work.
                //
                // **And the connection's end wins over the work.** Where it
                // arrives first, the work is given up where it stands, and
                // whoever asked for it is told the connection ended — the same
                // answer every ask still waiting gets when the lending ends.
                let worked = tokio::select! {
                    biased;
                    ending = &mut its_ending => Err(ending),
                    worked = beside.go_on_while(point.does_one_thing(connection, the_additional)) => Ok(worked),
                };
                let (started, ended) = match worked {
                    Ok(worked) => worked,
                    Err(ending) => return ending,
                };
                point.commands_beside_ended(ended);
                if let Some(started) = started {
                    beside.0.push(started);
                }
            }
        }
    }
}

/// **The commands running beside the point**, each answering its own asker
/// when it ends. Added 2026-09-27, step 10c's eighth-pass fix piece 1.
///
/// **Polled by the lending itself rather than handed to the runtime**, so
/// every session this point uses is still used on the thread lending the
/// connection (P5).
#[derive(Default)]
struct CommandsBeside(Vec<ItsOwnFuture<'static, ()>>);

impl CommandsBeside {
    /// **Answers once at least one has ended, with how many.** Never, while
    /// none runs.
    fn some_end(&mut self) -> impl Future<Output = usize> + '_ {
        std::future::poll_fn(move |cx| {
            let before = self.0.len();
            self.0.retain_mut(|one| one.as_mut().poll(cx).is_pending());
            match before - self.0.len() {
                0 => Poll::Pending,
                ended => Poll::Ready(ended),
            }
        })
    }

    /// **`work` done, with the commands beside going on meanwhile**, and how
    /// many of them ended while it was done.
    async fn go_on_while<T>(&mut self, work: impl Future<Output = T>) -> (T, usize) {
        tokio::pin!(work);
        let mut ended = 0;
        loop {
            tokio::select! {
                biased;
                done = &mut work => return (done, ended),
                more = self.some_end() => ended += more,
            }
        }
    }
}

/// **The rest of one command, run to its end, and its asker answered** — or,
/// where the asker has stopped waiting, given up, which closes the command's
/// channel (`Server Setup` Y8: *"the desktop stops issuing and closes the
/// step's channel"*). Added 2026-09-27, step 10c's eighth-pass fix piece 1.
///
/// **Its request has gone out by the time this runs**, so an asker it cannot
/// answer with the command's result — the connection's ending drops it part
/// way, or the rest itself fails — is told
/// [`NotLent::TheConnectionEndedAfterTheRequestWentOut`], never that the
/// command did not run (the dead-line desk coverage, 2026-10-19).
fn the_rest_is_run(
    rest: ItsOwnFuture<'static, Result<CommandResult, NoSession>>,
    answer: oneshot::Sender<Result<CommandResult, NotLent>>,
) -> ItsOwnFuture<'static, ()> {
    /// **The asker's answer, owed until it is given**, and given as *the
    /// request went out* where this is dropped first.
    struct TheAnswerOwed(Option<oneshot::Sender<Result<CommandResult, NotLent>>>);
    impl Drop for TheAnswerOwed {
        fn drop(&mut self) {
            if let Some(answer) = self.0.take() {
                let _ = answer.send(Err(NotLent::TheConnectionEndedAfterTheRequestWentOut));
            }
        }
    }
    let mut owed = TheAnswerOwed(Some(answer));
    Box::pin(async move {
        let ran = {
            let Some(answer) = owed.0.as_mut() else {
                return;
            };
            tokio::select! {
                ran = rest => Some(ran),
                () = answer.closed() => None,
            }
        };
        // **Where the asker stopped waiting** nobody is told anything.
        if let (Some(ran), Some(answer)) = (ran, owed.0.take()) {
            let _ = answer.send(ran.map_err(|_| NotLent::TheConnectionEndedAfterTheRequestWentOut));
        }
    })
}

/// **The asking side sees a live connection for exactly as long as this
/// stands**, and nothing after.
struct WhileItIsLive<'s, C: AHeldConnection> {
    sessions: &'s ThisServersSessions<C>,
}

impl<'s, C: AHeldConnection> WhileItIsLive<'s, C> {
    fn begins(sessions: &'s ThisServersSessions<C>, tell: mpsc::UnboundedSender<AnAsk<C>>) -> Self {
        // **A new lending finds no link standing** (2026-10-26, fault 1): the
        // connection is new, and what the last one left on the machine is not
        // read as this one's.
        if let Ok(mut stands) = sessions.the_link_stands.lock() {
            stands.lending += 1;
            stands.planted = None;
        }
        if let Ok(mut live) = sessions.live.lock() {
            *live = Some(tell);
        }
        WhileItIsLive { sessions }
    }
}

impl<C: AHeldConnection> Drop for WhileItIsLive<'_, C> {
    fn drop(&mut self) {
        if let Ok(mut live) = self.sessions.live.lock() {
            *live = None;
        }
        self.sessions.the_link_may_not_stand();
    }
}

/// What the point knows about one connection's pool.
#[derive(Debug, Default)]
struct ThePool {
    /// **`Server protocol and transfers` T2's learned limit**, absent until a
    /// refusal teaches it.
    learned: Option<usize>,
    /// Transfer channels seated on it and not yet given back.
    transfers: usize,
    /// **This point let a session go and the far end may not have freed it
    /// yet.** A refusal while this stands is asked again rather than learned
    /// from.
    a_release_is_in_flight: bool,
}

impl ThePool {
    /// **A refusal teaches the limit, and it only ever falls.**
    fn learns(&mut self, held: usize) {
        self.learned = Some(self.learned.map_or(held, |was| was.min(held)));
    }
}

/// `Servers` S14's additional connection, while it stands.
struct TheAdditional<C> {
    connection: C,
    pool: ThePool,
    which: u64,
    /// **Dropped with it**, so every channel lent on it learns it has gone
    /// — let go idle, lost on a session asked of it, or gone with the point.
    /// Step 10c's twelfth-pass fix piece 1. **And told by its own transport's
    /// report** (the dead-line sweep's extra-connection repair).
    stands: WhileTheConnectionStands,
    /// **Its own ending, as the point hears it** — see
    /// [`ThePoint::the_additional_has_ended`].
    its_end: TheConnectionEnds,
}

/// The point, for one live primary connection.
struct ThePoint<C: AHeldConnection> {
    tell: mpsc::UnboundedSender<AnAsk<C>>,
    /// **What a channel lent on the primary watches for its connection's
    /// end.** The primary's own [`WhileTheConnectionStands`] is held by
    /// [`lend_until_it_ends`], which outlives the point.
    the_primary_ends: watch::Receiver<bool>,
    reading: Option<Subsystem<C::Channel>>,
    primary: ThePool,
    additional: Option<TheAdditional<C>>,
    the_additional_is_being_opened: bool,
    additional_connections_made: u64,
    commands: VecDeque<ACommand>,
    /// **Acts that take the whole connection**, one at a time and before
    /// anything else that would open a session on it.
    whole_connection_acts: VecDeque<AWholeConnectionAct<C>>,
    readings: VecDeque<(bool, AReading<C::Channel>)>,
    transfers: VecDeque<oneshot::Sender<Result<ALentChannel<C>, NotLent>>>,
    /// **Every session the primary allows is held by transfers**, so the next
    /// command waits for one to be given back.
    commands_wait_for_a_release: bool,
    /// The same, for the reading session.
    readings_wait_for_a_release: bool,
    /// **Commands running beside the point on the primary**, each holding a
    /// session of it until it ends. Added 2026-09-27, step 10c's eighth-pass
    /// fix piece 1.
    commands_beside: usize,
    /// **No seat can be had for a transfer on either connection now**, so the
    /// next waits for a release or for the additional connection to come back.
    transfers_wait: bool,
}

/// Where a transfer was seated, or why it was not.
enum Seating<C: AHeldConnection> {
    Seated(ALentChannel<C>),
    Waits,
    Refused(NotLent),
}

impl<C: AHeldConnection> ThePoint<C> {
    fn for_one_connection(
        tell: mpsc::UnboundedSender<AnAsk<C>>,
        the_primary_stands: &WhileTheConnectionStands,
    ) -> Self {
        ThePoint {
            tell,
            the_primary_ends: the_primary_stands.asked_after(),
            reading: None,
            // **A fresh point starts as if a session had just been let go.**
            // `Servers`' own sequence runs its commands on this connection
            // before the point is lent it — step 4's readings on a first
            // connection — and the machine frees each command's session a
            // moment after the command ends. **A refusal met in that gap, with
            // nothing held, would otherwise be taken as a limit of nought**
            // (`Servers` S15's *the machine permits no session*) on a machine
            // whose limit is one, and the supervisor would be called
            // unavailable once before `Servers` Q19's re-take settled it. So
            // the first refusal is asked again under the same bound as any
            // refusal straight after a release — step 10c piece 4's finding,
            // repaired at step 10c piece 6 (R1).
            primary: ThePool {
                a_release_is_in_flight: true,
                ..ThePool::default()
            },
            additional: None,
            the_additional_is_being_opened: false,
            additional_connections_made: 0,
            commands: VecDeque::new(),
            whole_connection_acts: VecDeque::new(),
            readings: VecDeque::new(),
            transfers: VecDeque::new(),
            commands_wait_for_a_release: false,
            readings_wait_for_a_release: false,
            commands_beside: 0,
            transfers_wait: false,
        }
    }

    /// Sessions held on the primary now, not counting the asking command's
    /// own — **and counting every command running beside**, so a refusal met
    /// while they run teaches the limit they are part of.
    fn held_on_the_primary(&self) -> usize {
        usize::from(self.reading.is_some()) + self.primary.transfers + self.commands_beside
    }

    /// **Commands running beside ended**, and each gave its session back —
    /// which the far end frees a moment later, as it does for any command.
    fn commands_beside_ended(&mut self, how_many: usize) {
        if how_many == 0 {
            return;
        }
        self.commands_beside = self.commands_beside.saturating_sub(how_many);
        self.primary.a_release_is_in_flight = true;
        self.a_seat_came_free();
    }

    /// **An ask arrives.** Nothing is opened here; it joins its queue, and a
    /// seat given back is counted at once.
    fn arrives(&mut self, ask: AnAsk<C>) {
        match ask {
            AnAsk::AReading {
                someone_is_waiting,
                reading,
            } => self.readings.push_back((someone_is_waiting, reading)),
            AnAsk::ACommand(command) => self.commands.push_back(command),
            AnAsk::TheWholeConnection(act) => self.whole_connection_acts.push_back(act),
            AnAsk::ATransfer(answer) => self.transfers.push_back(answer),
            AnAsk::ATransferEnded(SeatedOn::ThePrimary) => {
                self.primary.transfers = self.primary.transfers.saturating_sub(1);
                self.primary.a_release_is_in_flight = true;
                self.a_seat_came_free();
            }
            AnAsk::ATransferEnded(SeatedOn::TheAdditional(which)) => {
                if let Some(additional) = self.additional.as_mut().filter(|it| it.which == which) {
                    additional.pool.transfers = additional.pool.transfers.saturating_sub(1);
                    additional.pool.a_release_is_in_flight = true;
                }
                self.a_seat_came_free();
            }
            AnAsk::TheAdditionalCameBack(connection) => {
                self.the_additional_is_being_opened = false;
                self.transfers_wait = false;
                match connection {
                    Some(connection) => {
                        self.additional_connections_made += 1;
                        let stands = WhileTheConnectionStands::begins();
                        stands.ends_with(connection.its_ending());
                        let its_end = stands.watched();
                        self.additional = Some(TheAdditional {
                            connection,
                            pool: ThePool::default(),
                            which: self.additional_connections_made,
                            stands,
                            its_end,
                        });
                    }
                    // **`Servers` S14 condition 3: not made, so every transfer
                    // waiting for it is unavailable on this connection rather
                    // than failed.**
                    None => {
                        for waiting in self.transfers.drain(..) {
                            let _ = waiting.send(Err(NotLent::UnavailableOnThisConnection));
                        }
                    }
                }
            }
        }
    }

    fn a_seat_came_free(&mut self) {
        self.commands_wait_for_a_release = false;
        self.readings_wait_for_a_release = false;
        self.transfers_wait = false;
    }

    /// **Answers once `Servers` S14's additional connection has ended on its
    /// own** — its transport's report, heard through its watch. Never, while
    /// none stands. Added 2026-10-19, the dead-line sweep's extra-connection repair.
    fn the_additional_has_ended(&mut self) -> impl Future<Output = ()> + '_ {
        std::future::poll_fn(move |cx| match self.additional.as_mut() {
            Some(additional) => {
                if additional.its_end.has_ended(cx) {
                    Poll::Ready(())
                } else {
                    Poll::Pending
                }
            }
            None => Poll::Pending,
        })
    }

    /// **The additional connection is let go, having ended on its own.**
    /// Every channel lent on it was told already; **the primary is untouched**
    /// (`Servers` S14 condition 6), and a transfer waiting for a seat is
    /// seated on a fresh one.
    fn the_additional_is_gone(&mut self) {
        self.additional = None;
        self.transfers_wait = false;
    }

    /// **`Servers` S14's additional connection asked for**, where it is not
    /// being opened already. **`Servers` owns opening it**; what comes back
    /// arrives as an ask.
    fn the_additional_is_asked_for(&mut self, the_additional: &TheAdditionalConnection<C>) {
        if !self.the_additional_is_being_opened {
            self.the_additional_is_being_opened = true;
            let opening = the_additional();
            let tell = self.tell.clone();
            tokio::spawn(async move {
                let _ = tell.send(AnAsk::TheAdditionalCameBack(opening.await));
            });
        }
    }

    /// **`Servers` S14 condition 5: closed when the last transfer on it ends**,
    /// and never held idle.
    fn the_additional_is_idle(&self) -> bool {
        self.additional
            .as_ref()
            .is_some_and(|it| it.pool.transfers == 0)
            && self.transfers.is_empty()
    }

    fn there_is_work_it_can_do(&self) -> bool {
        // **An act that takes the whole connection waits for every command
        // running beside to end, and nothing else starts while it waits** —
        // it goes before anything else that would open a session on the
        // connection, as it always has.
        if !self.whole_connection_acts.is_empty() {
            return self.the_additional_is_idle() || self.commands_beside == 0;
        }
        self.the_additional_is_idle()
            || (!self.commands.is_empty() && !self.commands_wait_for_a_release)
            || (!self.transfers.is_empty() && !self.transfers_wait)
            // **`Server protocol and transfers` T5: no reading begins while a
            // command waits.**
            || (!self.readings.is_empty()
                && self.commands.is_empty()
                && !self.readings_wait_for_a_release)
    }

    /// **One piece of work, in the clause's order**: an idle additional
    /// connection closed, then a command, then a transfer's seat, then a
    /// reading.
    ///
    /// **A command begun here that goes on running is handed back**, to be
    /// run beside the point's later work — see [`Self::a_command_is_run`].
    async fn does_one_thing(
        &mut self,
        connection: &mut C,
        the_additional: &TheAdditionalConnection<C>,
    ) -> Option<ItsOwnFuture<'static, ()>> {
        if self.the_additional_is_idle() {
            if let Some(idle) = self.additional.take() {
                idle.connection.torn_down().await;
            }
            return None;
        }
        // **The whole connection goes before anything else that would open a
        // session on it.** The act holds both of §1.1's mechanisms for its own
        // length, and the reading session is let go first so a machine at a
        // limit of one has a session to give it. **It waits for every command
        // running beside to end first**, and nothing else is started while it
        // waits.
        if !self.whole_connection_acts.is_empty() {
            if self.commands_beside == 0 {
                if let Some(act) = self.whole_connection_acts.pop_front() {
                    self.the_reading_session_is_closed();
                    act(connection).await;
                }
            }
            return None;
        }
        if !self.commands_wait_for_a_release {
            if let Some(command) = self.commands.pop_front() {
                return self.a_command_is_run(connection, command).await;
            }
        }
        if !self.transfers_wait {
            if let Some(answer) = self.transfers.pop_front() {
                match self.a_transfer_is_seated(connection, the_additional).await {
                    Seating::Seated(channel) => {
                        let _ = answer.send(Ok(channel));
                    }
                    Seating::Refused(why) => {
                        let _ = answer.send(Err(why));
                    }
                    Seating::Waits => {
                        self.transfers.push_front(answer);
                        self.transfers_wait = true;
                    }
                }
                return None;
            }
        }
        if self.commands.is_empty() && !self.readings_wait_for_a_release {
            // **`Server protocol and transfers` §1.1 item 2: a reading the user
            // is waiting on goes first**, and otherwise the first asked.
            let next = self
                .readings
                .iter()
                .position(|(someone_is_waiting, _)| *someone_is_waiting)
                .unwrap_or(0);
            if let Some((someone_is_waiting, reading)) = self.readings.remove(next) {
                self.a_reading_is_run(connection, someone_is_waiting, reading)
                    .await;
            }
        }
        None
    }

    /// **One command, on the primary.**
    ///
    /// **`Server protocol and transfers` T5 at a learned limit of one: the
    /// reading session is closed before the command asks for the session, and
    /// no slot is reserved.** Where the limit is not yet known the command asks
    /// first, and a refusal teaches the limit and closes the reading session
    /// the same way.
    ///
    /// **Where it is begun and goes on running, what happens next turns on the
    /// limit — step 10c's eighth-pass fix piece 1, 2026-09-27.** **At a learned
    /// limit of one it is run to its end here**, holding T5's single lease for
    /// its whole length, exactly as every command was run before. **Above one,
    /// or where no limit is learned, it is handed back to run beside the
    /// point**, which goes on serving: T5 reserves a slot for commands there and
    /// does not say one command at a time, and `Server Setup` §13 item 7's
    /// paragraph of 2026-09-27 has `Server Setup` Y7's refresh go out on its
    /// own session beside an admin step. **What it costs is one more session
    /// from the pool**, counted in [`Self::held_on_the_primary`], and a machine
    /// that refuses it teaches the limit the ordinary way.
    async fn a_command_is_run(
        &mut self,
        connection: &mut C,
        ask: ACommand,
    ) -> Option<ItsOwnFuture<'static, ()>> {
        loop {
            if let Some(limit) = self.primary.learned {
                if self.held_on_the_primary() + 1 > limit && self.reading.is_some() {
                    self.the_reading_session_is_closed();
                }
            }
            let mut tries = 0;
            let begun = loop {
                let begun = match &ask.up {
                    None => connection.a_command_begun(ask.command, ask.frame.clone()).await,
                    Some(up) => {
                        connection
                            .a_command_begun_passing_up(ask.command, ask.frame.clone(), up.clone())
                            .await
                    }
                };
                match begun {
                    Err(NoSession::Refused)
                        if self.primary.a_release_is_in_flight
                            && tries < THE_TRIES_AFTER_A_RELEASE =>
                    {
                        tries += 1;
                        tokio::time::sleep(THE_PAUSE_AFTER_A_RELEASE).await;
                    }
                    other => break other,
                }
            };
            self.primary.a_release_is_in_flight = false;
            match begun {
                Ok(ACommandRunning(WhereItIs::Ended(result))) => {
                    // **The command's own session closed as it ended**, and
                    // the far end frees it a moment later.
                    self.primary.a_release_is_in_flight = true;
                    let _ = ask.answer.send(Ok(result));
                    return None;
                }
                Ok(ACommandRunning(WhereItIs::ToBeRun(rest))) => {
                    let rest = the_rest_is_run(rest, ask.answer);
                    if self.primary.learned == Some(1) {
                        rest.await;
                        self.primary.a_release_is_in_flight = true;
                        return None;
                    }
                    self.commands_beside += 1;
                    return Some(rest);
                }
                Err(NoSession::TheConnectionIsGone) => {
                    let _ = ask.answer.send(Err(NotLent::TheConnectionEnded));
                    return None;
                }
                Err(NoSession::Refused) => {
                    let held = self.held_on_the_primary();
                    self.primary.learns(held);
                    if self.reading.is_some() {
                        self.the_reading_session_is_closed();
                        continue;
                    }
                    if held == 0 {
                        let _ = ask.answer.send(Err(NotLent::TheMachineAllowsNoSession));
                        return None;
                    }
                    // **Transfers, or commands running beside, hold every
                    // session the machine allows** — transfers only where they
                    // were seated before the limit was known. The command goes
                    // first once one is given back.
                    self.commands.push_front(ask);
                    self.commands_wait_for_a_release = true;
                    return None;
                }
            }
        }
    }

    /// **The reading session is closed**, destroying every handle open on it —
    /// `Server protocol and transfers` §1.1 item 2's re-open rule is what an
    /// operation holding one does about that.
    fn the_reading_session_is_closed(&mut self) {
        if self.reading.take().is_some() {
            self.primary.a_release_is_in_flight = true;
        }
    }

    /// **One reading on the reading session**, opening it first where it is
    /// not open.
    async fn a_reading_is_run(
        &mut self,
        connection: &mut C,
        someone_is_waiting: bool,
        reading: AReading<C::Channel>,
    ) {
        if self.reading.is_none() {
            let fits = self
                .primary
                .learned
                .is_none_or(|limit| self.primary.transfers + self.commands_beside < limit);
            let opened = if fits {
                a_session_on(connection, &mut self.primary).await
            } else {
                Err(NoSession::Refused)
            };
            match opened {
                Ok(channel) => match Subsystem::negotiate(channel).await {
                    Ok(subsystem) => self.reading = Some(subsystem),
                    Err(why) => {
                        reading(Err(NotLent::TheReadingSessionCouldNotBeOpened(why))).await;
                        return;
                    }
                },
                Err(NoSession::TheConnectionIsGone) => {
                    reading(Err(NotLent::TheConnectionEnded)).await;
                    return;
                }
                Err(NoSession::Refused) => {
                    let held = self.held_on_the_primary();
                    self.primary.learns(held);
                    if held == 0 {
                        reading(Err(NotLent::TheMachineAllowsNoSession)).await;
                        return;
                    }
                    self.readings.push_front((someone_is_waiting, reading));
                    self.readings_wait_for_a_release = true;
                    return;
                }
            }
        }
        let Some(subsystem) = self.reading.as_mut() else {
            return;
        };
        if reading(Ok(subsystem)).await == AfterTheReading::Gone {
            self.reading = None;
        }
    }

    /// **A seat for one transfer.**
    ///
    /// **On the primary where the pool has room for it beside the reading
    /// session and one command** — the reading session is counted whether or
    /// not it is open. **Otherwise on `Servers` S14's additional connection**,
    /// which is opened on this refused admission if it is not open, carries
    /// transfers alone, and is one at most.
    async fn a_transfer_is_seated(
        &mut self,
        connection: &mut C,
        the_additional: &TheAdditionalConnection<C>,
    ) -> Seating<C> {
        let the_primary_has_room = self
            .primary
            .learned
            .is_none_or(|limit| self.primary.transfers + 3 <= limit);
        if the_primary_has_room {
            match a_session_on(connection, &mut self.primary).await {
                Ok(channel) => {
                    self.primary.transfers += 1;
                    let its_connection = TheConnectionEnds::of(self.the_primary_ends.clone());
                    let asked_after = self.the_primary_ends.clone();
                    return Seating::Seated(self.lent(
                        channel,
                        SeatedOn::ThePrimary,
                        (its_connection, asked_after),
                    ));
                }
                Err(NoSession::TheConnectionIsGone) => {
                    return Seating::Refused(NotLent::TheConnectionEnded)
                }
                Err(NoSession::Refused) => {
                    let held = self.held_on_the_primary();
                    self.primary.learns(held);
                }
            }
        }

        let Some(additional) = self.additional.as_mut() else {
            self.the_additional_is_asked_for(the_additional);
            return Seating::Waits;
        };
        let it_has_room = additional
            .pool
            .learned
            .is_none_or(|limit| additional.pool.transfers < limit);
        if !it_has_room {
            return Seating::Waits;
        }
        let which = additional.which;
        let its_connection = additional.stands.watched();
        let asked_after = additional.stands.asked_after();
        match a_session_on(&mut additional.connection, &mut additional.pool).await {
            Ok(channel) => {
                additional.pool.transfers += 1;
                Seating::Seated(self.lent(
                    channel,
                    SeatedOn::TheAdditional(which),
                    (its_connection, asked_after),
                ))
            }
            Err(NoSession::Refused) => {
                let held = additional.pool.transfers;
                additional.pool.learns(held);
                if held == 0 {
                    Seating::Refused(NotLent::UnavailableOnThisConnection)
                } else {
                    Seating::Waits
                }
            }
            // **`Servers` S14 condition 6: losing the additional connection is
            // a transfer interruption and never a break**, and the next
            // transfer opens a fresh one. **This transfer is that next
            // one**: it waits for the fresh connection rather than being told
            // the route is unavailable — corrected 2026-10-19, the dead-line
            // sweep's extra-connection repair. Until then a transfer that met a connection
            // just gone was refused, while the very next one was carried.
            Err(NoSession::TheConnectionIsGone) => {
                self.additional = None;
                self.the_additional_is_asked_for(the_additional);
                Seating::Waits
            }
        }
    }

    fn lent(
        &self,
        channel: C::Channel,
        on: SeatedOn,
        (its_connection, asked_after): (TheConnectionEnds, watch::Receiver<bool>),
    ) -> ALentChannel<C> {
        ALentChannel {
            channel,
            seat: TheSeat {
                on,
                tell: self.tell.clone(),
            },
            its_connection,
            asked_after,
        }
    }
}

/// **One session, with a refusal straight after a release asked again** — the
/// bounded delay the file's head describes.
async fn a_session_on<C: AHeldConnection>(
    connection: &mut C,
    pool: &mut ThePool,
) -> Result<C::Channel, NoSession> {
    let mut tries = 0;
    let opened = loop {
        match connection.open_a_session().await {
            Err(NoSession::Refused)
                if pool.a_release_is_in_flight && tries < THE_TRIES_AFTER_A_RELEASE =>
            {
                tries += 1;
                tokio::time::sleep(THE_PAUSE_AFTER_A_RELEASE).await;
            }
            other => break other,
        }
    };
    pool.a_release_is_in_flight = false;
    opened
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use smoozy_server_protocol::framing::{
        read_one_request, write_result_frame, Answered, Fields,
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
    };
    use smoozy_server_protocol::placement::THE_HEALTH_EXCHANGE_COMMAND;
    use smoozy_server_protocol::transport::CommandString;
    use smoozy_state::identities::{Identity, IdentityKind};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use tokio::io::{AsyncReadExt, AsyncWriteExt, DuplexStream};
    use tokio::sync::watch;

    // ------------------------------------------------ a machine, standing in

    /// **A machine with a session limit, standing in for `MaxSessions`.** It
    /// frees a session **after** the channel is dropped, on a task of its own,
    /// the way a real server frees one after handling the close — so the
    /// refusal straight after a release is met here too.
    pub(crate) struct AMachine {
        limit: usize,
        open: AtomicUsize,
        /// Sessions the point opened, and sessions it let go of.
        opened: AtomicUsize,
        let_go: AtomicUsize,
        what_happened: Mutex<Vec<String>>,
        /// **[`A_SLOW_COMMAND`] runs until this says it may end** — an admin
        /// step that takes minutes, standing in. Added 2026-09-27, step 10c's
        /// eighth-pass fix piece 1.
        the_slow_command_may_end: watch::Sender<bool>,
        /// **Whether a link stands at the carriage site**, planted and taken
        /// away through the file server — so the health exchange's command
        /// runs through it or finds no program, as a real shell does. Added
        /// 2026-09-30, step 10c's eleventh-pass fix piece 4c.
        a_link_stands: std::sync::atomic::AtomicBool,
        /// **A link asked for lands only once this says it may**, so a test
        /// can hold a supervisor ask between its link and its command.
        links_may_land: watch::Sender<bool>,
        /// **The line under this machine's sessions goes silent**: its file
        /// server stops reading what is sent and answers nothing, and closes
        /// nothing — so a write on a channel to it waits for room. Added
        /// 2026-10-19, the dead-line sweep's extra-connection repair.
        pub(crate) goes_silent: watch::Sender<bool>,
        /// **The supervisor's command goes silent from this ask on** —
        /// begun, and never ended — counting asks from one. Nought, the
        /// default, is never. Added 2026-10-19, the dead-line sweep's two-minute-wait
        /// repair.
        pub(crate) the_supervisor_is_silent_from: AtomicUsize,
        /// The supervisor's commands begun, counted.
        pub(crate) supervisor_asks: AtomicUsize,
        /// **When each was begun**, in order.
        pub(crate) supervisor_asked_at: Mutex<Vec<std::time::Instant>>,
    }

    impl AMachine {
        pub(crate) fn with_a_limit_of(limit: usize) -> Arc<AMachine> {
            Arc::new(AMachine {
                limit,
                open: AtomicUsize::new(0),
                opened: AtomicUsize::new(0),
                let_go: AtomicUsize::new(0),
                what_happened: Mutex::new(Vec::new()),
                the_slow_command_may_end: watch::channel(false).0,
                a_link_stands: std::sync::atomic::AtomicBool::new(false),
                links_may_land: watch::channel(true).0,
                goes_silent: watch::channel(false).0,
                the_supervisor_is_silent_from: AtomicUsize::new(0),
                supervisor_asks: AtomicUsize::new(0),
                supervisor_asked_at: Mutex::new(Vec::new()),
            })
        }

        /// The slow command is let finish.
        fn lets_the_slow_command_end(&self) {
            self.the_slow_command_may_end.send_replace(true);
        }

        /// **Waits, with a bound, for the machine to have said `what`.**
        async fn has_said(&self, what: &str) {
            let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
            while !self.what_happened().iter().any(|it| it.contains(what)) {
                assert!(
                    tokio::time::Instant::now() < deadline,
                    "the machine never said {what:?}: {:#?}",
                    self.what_happened()
                );
                tokio::time::sleep(Duration::from_millis(2)).await;
            }
        }

        fn says(&self, what: String) {
            self.what_happened.lock().unwrap().push(what);
        }

        /// What the point itself holds open on this machine now.
        fn the_point_holds(&self) -> usize {
            self.opened.load(Ordering::SeqCst) - self.let_go.load(Ordering::SeqCst)
        }

        fn what_happened(&self) -> Vec<String> {
            self.what_happened.lock().unwrap().clone()
        }

        fn a_session_if_there_is_room(&self) -> bool {
            self.open
                .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |open| {
                    (open < self.limit).then_some(open + 1)
                })
                .is_ok()
        }
    }

    /// **The point's own end of a session**, counted the instant the point
    /// lets it go — before the machine has noticed — so a test can say what the
    /// point held when it asked.
    pub(crate) struct OurEnd {
        stream: DuplexStream,
        machine: Arc<AMachine>,
    }

    impl Drop for OurEnd {
        fn drop(&mut self) {
            self.machine.let_go.fetch_add(1, Ordering::SeqCst);
        }
    }

    impl AsyncRead for OurEnd {
        fn poll_read(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut ReadBuf<'_>,
        ) -> Poll<std::io::Result<()>> {
            Pin::new(&mut self.stream).poll_read(cx, buf)
        }
    }

    impl AsyncWrite for OurEnd {
        fn poll_write(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &[u8],
        ) -> Poll<std::io::Result<usize>> {
            Pin::new(&mut self.stream).poll_write(cx, buf)
        }
        fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
            Pin::new(&mut self.stream).poll_flush(cx)
        }
        fn poll_shutdown(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
        ) -> Poll<std::io::Result<()>> {
            Pin::new(&mut self.stream).poll_shutdown(cx)
        }
    }

    pub(crate) struct AConnectionToIt {
        machine: Arc<AMachine>,
        name: &'static str,
        ends: watch::Receiver<bool>,
    }

    impl AHeldConnection for AConnectionToIt {
        type Channel = OurEnd;

        async fn open_a_session(&mut self) -> Result<OurEnd, NoSession> {
            if *self.ends.borrow() {
                return Err(NoSession::TheConnectionIsGone);
            }
            if !self.machine.a_session_if_there_is_room() {
                self.machine
                    .says(format!("{}: a session was refused", self.name));
                return Err(NoSession::Refused);
            }
            self.machine.says(format!(
                "{}: a session opened, {} open",
                self.name,
                self.machine.open.load(Ordering::SeqCst)
            ));
            let (ours, theirs) = tokio::io::duplex(64 * 1024);
            tokio::spawn(a_file_server(theirs, Arc::clone(&self.machine), self.name));
            self.machine.opened.fetch_add(1, Ordering::SeqCst);
            Ok(OurEnd {
                stream: ours,
                machine: Arc::clone(&self.machine),
            })
        }

        /// **The whole command, as the point ran every command before
        /// 2026-09-27** — begun, and run to its end here.
        async fn a_command(
            &mut self,
            command: CommandString,
            frame: Option<Vec<u8>>,
        ) -> Result<CommandResult, NoSession> {
            match self.a_command_begun(command, frame).await?.0 {
                WhereItIs::Ended(result) => Ok(result),
                WhereItIs::ToBeRun(rest) => rest.await,
            }
        }

        /// **The slow command prints a line as it begins**, the way a long
        /// admin step prints while it runs, and the rest at its end — so a
        /// test can tell a piece passed up as it printed from one passed up
        /// with the answer (`Server Setup` Y7a). Every other command takes the
        /// provided route. Added 2026-10-18.
        async fn a_command_begun_passing_up(
            &mut self,
            command: CommandString,
            frame: Option<Vec<u8>>,
            up: WhereThePiecesGo,
        ) -> Result<ACommandRunning, NoSession> {
            let slow = command.as_str() == A_SLOW_COMMAND.as_str();
            let begun = self.a_command_begun(command, frame).await?;
            if !slow {
                return Ok(begun.passed_up_at_its_end(up));
            }
            let _ = up.send(APiecePrinted::Output(b"Unpacking nvidia-driver-595 ...\n".to_vec()));
            let _ = up.send(APiecePrinted::Complaint(b"debconf: delaying\n".to_vec()));
            Ok(begun)
        }

        async fn a_command_begun(
            &mut self,
            command: CommandString,
            frame: Option<Vec<u8>>,
        ) -> Result<ACommandRunning, NoSession> {
            if *self.ends.borrow() {
                return Err(NoSession::TheConnectionIsGone);
            }
            self.machine.says(format!(
                "{}: a command was asked for, the point holding {}",
                self.name,
                self.machine.the_point_holds()
            ));
            if !self.machine.a_session_if_there_is_room() {
                self.machine
                    .says(format!("{}: a command was refused", self.name));
                return Err(NoSession::Refused);
            }
            self.machine.says(format!(
                "{}: a command ran, {} open",
                self.name,
                self.machine.open.load(Ordering::SeqCst)
            ));
            // **The slow command holds its session until it is let end**, and
            // is handed back running — the way the real transport hands back a
            // command whose request has gone out.
            if command.as_str() == A_SLOW_COMMAND.as_str() {
                self.machine
                    .says(format!("{}: the slow command began", self.name));
                let machine = Arc::clone(&self.machine);
                let name = self.name;
                let mut may_end = machine.the_slow_command_may_end.subscribe();
                return Ok(ACommandRunning::to_be_run(async move {
                    // **Given up before its end, its channel is closed** — which
                    // is what the real transport's own does when dropped.
                    let mut closed = ItsChannel {
                        machine: Arc::clone(&machine),
                        name,
                        ended: false,
                    };
                    let _ = may_end.wait_for(|it| *it).await;
                    closed.ended = true;
                    machine.says(format!("{name}: the slow command ended"));
                    machine.open.fetch_sub(1, Ordering::SeqCst);
                    Ok(CommandResult {
                        output: Vec::new(),
                        complaint: Vec::new(),
                        status: Some(0),
                    })
                }));
            }
            // **The entry point runs through the planted link or not at all.**
            if command.as_str() == THE_HEALTH_EXCHANGE_COMMAND.as_str()
                && !self.machine.a_link_stands.load(Ordering::SeqCst)
            {
                self.machine
                    .says(format!("{}: the entry point found no program", self.name));
                let machine = Arc::clone(&self.machine);
                tokio::spawn(async move {
                    tokio::task::yield_now().await;
                    machine.open.fetch_sub(1, Ordering::SeqCst);
                });
                return Ok(ACommandRunning::already_ended(CommandResult {
                    output: Vec::new(),
                    complaint: b"smoozy-server: No such file or directory".to_vec(),
                    status: Some(127),
                }));
            }
            // **A supervisor ask met by a silent line** — begun, and never
            // ended — from the ask the test names on (the dead-line sweep's
            // two-minute-wait repair). **Given up before its end, its channel is closed**,
            // as the real transport's own is.
            if frame.is_some() {
                let asked = self.machine.supervisor_asks.fetch_add(1, Ordering::SeqCst) + 1;
                self.machine
                    .supervisor_asked_at
                    .lock()
                    .unwrap()
                    .push(std::time::Instant::now());
                let silent_from = self.machine.the_supervisor_is_silent_from.load(Ordering::SeqCst);
                if silent_from != 0 && asked >= silent_from {
                    self.machine
                        .says(format!("{}: the supervisor's ask met a silent line", self.name));
                    let machine = Arc::clone(&self.machine);
                    let name = self.name;
                    return Ok(ACommandRunning::to_be_run(async move {
                        let _closed = ItsChannel {
                            machine,
                            name,
                            ended: false,
                        };
                        std::future::pending::<Result<CommandResult, NoSession>>().await
                    }));
                }
            }
            // **The supervisor, standing in**: one framed request in, the
            // result carrying its identity out.
            let output = match frame {
                Some(frame) => {
                    let request = read_one_request(&mut &frame[..]).expect("one framed request");
                    write_result_frame(
                        &Answered::to(&request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "1.0.0")
                            .carrying(Fields::none().and("command", command.as_str())),
                    )
                }
                None => Vec::new(),
            };
            let machine = Arc::clone(&self.machine);
            tokio::spawn(async move {
                tokio::task::yield_now().await;
                machine.open.fetch_sub(1, Ordering::SeqCst);
            });
            Ok(ACommandRunning::already_ended(CommandResult {
                output,
                complaint: Vec::new(),
                status: Some(0),
            }))
        }

        async fn until_it_ends(&mut self) -> Ending {
            let _ = self.ends.wait_for(|ended| *ended).await;
            Ending::Announced
        }

        /// **The transport's own report, heard apart from the connection** —
        /// what the real one gives. **A connection nobody can end never
        /// reports**: its sender is gone, and that is not an ending.
        fn its_ending(&self) -> ItsOwnFuture<'static, Ending> {
            let mut ends = self.ends.clone();
            Box::pin(async move {
                let ended = ends.wait_for(|ended| *ended).await.is_ok();
                if !ended {
                    std::future::pending::<()>().await;
                }
                Ending::Announced
            })
        }

        async fn torn_down(self) -> Ending {
            self.machine.says(format!("{}: torn down", self.name));
            Ending::TornDownBySmoozy
        }
    }

    /// **The slow command's channel, as the machine sees it**: closed from
    /// this side where the command is given up before its end.
    struct ItsChannel {
        machine: Arc<AMachine>,
        name: &'static str,
        ended: bool,
    }

    impl Drop for ItsChannel {
        fn drop(&mut self) {
            if !self.ended {
                self.machine.says(format!(
                    "{}: the slow command's channel was closed",
                    self.name
                ));
                self.machine.open.fetch_sub(1, Ordering::SeqCst);
            }
        }
    }

    /// **The smallest file-transfer server a reading needs**: it answers the
    /// opening with revision 3 and nothing announced, an attribute request with
    /// empty attributes, and everything else with success. **The session is
    /// freed when the channel's far end goes**, after it goes.
    async fn a_file_server(mut stream: DuplexStream, machine: Arc<AMachine>, name: &'static str) {
        let mut silent = machine.goes_silent.subscribe();
        loop {
            // **A silent line reads nothing, answers nothing and closes
            // nothing** until it is carried again (the dead-line sweep's
            // extra-connection repair).
            let _ = silent.wait_for(|it| !*it).await;
            let mut length = [0u8; 4];
            let read = tokio::select! {
                read = stream.read_exact(&mut length) => read.map(|_| ()),
                _ = silent.wait_for(|it| *it) => continue,
            };
            if read.is_err() {
                break;
            }
            let mut body = vec![0u8; u32::from_be_bytes(length) as usize];
            if stream.read_exact(&mut body).await.is_err() {
                break;
            }
            let (kind, id) = (
                body[0],
                body.get(1..5).map(<[u8]>::to_vec).unwrap_or_default(),
            );
            // **The carriage's own two acts, kept**: a removal takes the link
            // away, and a link lands once the test lets it.
            if kind == 13 {
                machine.a_link_stands.store(false, Ordering::SeqCst);
            }
            if kind == 20 {
                machine.says(format!("{name}: a link was asked for"));
                let _ = machine.links_may_land.subscribe().wait_for(|it| *it).await;
                machine.a_link_stands.store(true, Ordering::SeqCst);
            }
            let (answer_kind, answer): (u8, Vec<u8>) = match kind {
                1 => (2, 3u32.to_be_bytes().to_vec()),
                7 | 17 => (105, [id, 0u32.to_be_bytes().to_vec()].concat()),
                _ => (101, [id, vec![0; 4], vec![0; 4], vec![0; 4]].concat()),
            };
            let mut packet = ((answer.len() + 1) as u32).to_be_bytes().to_vec();
            packet.push(answer_kind);
            packet.extend(answer);
            if stream.write_all(&packet).await.is_err() {
                break;
            }
        }
        tokio::task::yield_now().await;
        machine.open.fetch_sub(1, Ordering::SeqCst);
        machine.says(format!("{name}: a session closed"));
    }

    /// A point lending one connection to `machine`, on a task of its own.
    pub(crate) struct APointLending {
        pub(crate) sessions: ThisServersSessions<AConnectionToIt>,
        end_it: watch::Sender<bool>,
        lending: tokio::task::JoinHandle<Ending>,
    }

    pub(crate) fn additional_connections_to(
        machine: &Arc<AMachine>,
        made: bool,
    ) -> TheAdditionalConnection<AConnectionToIt> {
        let machine = Arc::clone(machine);
        Arc::new(move || {
            let machine = Arc::clone(&machine);
            Box::pin(async move {
                machine.says("the additional connection was asked for".to_string());
                made.then(|| AConnectionToIt {
                    machine,
                    name: "additional",
                    ends: watch::channel(false).1,
                })
            })
        })
    }

    /// **Additional connections the test can end one by one** — each its
    /// transport's own report, the primary untouched. The senders are kept in
    /// the order the connections were made. Added 2026-10-19, the dead-line
    /// sweep's extra-connection repair.
    type TheirEnds = Arc<Mutex<Vec<watch::Sender<bool>>>>;

    fn endable_additional_connections_to(
        machine: &Arc<AMachine>,
    ) -> (TheAdditionalConnection<AConnectionToIt>, TheirEnds) {
        let made: TheirEnds = Arc::new(Mutex::new(Vec::new()));
        let kept = Arc::clone(&made);
        let machine = Arc::clone(machine);
        let opens: TheAdditionalConnection<AConnectionToIt> = Arc::new(move || {
            let machine = Arc::clone(&machine);
            let kept = Arc::clone(&kept);
            Box::pin(async move {
                machine.says("the additional connection was asked for".to_string());
                let (end_it, ends) = watch::channel(false);
                kept.lock().unwrap().push(end_it);
                Some(AConnectionToIt {
                    machine,
                    name: "additional",
                    ends,
                })
            })
        });
        (opens, made)
    }

    pub(crate) async fn a_point_lending(
        machine: &Arc<AMachine>,
        additional: TheAdditionalConnection<AConnectionToIt>,
    ) -> APointLending {
        let sessions = ThisServersSessions::for_one_server();
        let (end_it, ends) = watch::channel(false);
        let mut connection = AConnectionToIt {
            machine: Arc::clone(machine),
            name: "primary",
            ends,
        };
        let lending = tokio::spawn({
            let sessions = sessions.clone();
            async move { lend_until_it_ends(&sessions, &mut connection, &additional).await }
        });
        while !sessions.a_connection_is_live() {
            tokio::task::yield_now().await;
        }
        APointLending {
            sessions,
            end_it,
            lending,
        }
    }

    impl APointLending {
        pub(crate) async fn ends(self) -> Ending {
            self.end_it.send(true).expect("the point is lending");
            self.lending.await.expect("the point ended by itself")
        }
    }

    async fn a_stat(
        sessions: &ThisServersSessions<AConnectionToIt>,
    ) -> Result<Result<(), SubsystemFailure>, NotLent> {
        sessions
            .a_reading(true, |subsystem| {
                Box::pin(async move { subsystem.stat(b".").await.map(|_| ()) })
            })
            .await
    }

    const A_COMMAND: CommandString = CommandString::constant("true");
    /// **A command that runs until the machine lets it end** — see
    /// [`AMachine::the_slow_command_may_end`].
    const A_SLOW_COMMAND: CommandString = CommandString::constant("a slow command");

    /// **Polled once, so the ask is made now** — an ask is sent before its
    /// first wait, and a test that spawned it instead would race its own
    /// asking.
    async fn asked_now<F: Future + Unpin>(asking: &mut F) {
        std::future::poll_fn(|cx| {
            let _ = Pin::new(&mut *asking).poll(cx);
            Poll::Ready(())
        })
        .await;
    }

    // ------------------------------------------------------------ the tests

    /// **`Server protocol and transfers` T5, by name: at a learned limit of
    /// one, no slot is reserved — a waiting command closes the reading session,
    /// takes the one session, and reading reopens only afterwards.**
    ///
    /// **The machine is what proves it**: it allows one session and refuses a
    /// second, so a command that ran while the reading session stayed open
    /// would have been refused. A point that kept the reading session open
    /// while a command waited would fail here.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn at_a_learned_limit_of_one_a_waiting_command_closes_the_reading_session() {
        let machine = AMachine::with_a_limit_of(1);
        let point = a_point_lending(&machine, additional_connections_to(&machine, false)).await;

        assert_eq!(
            a_stat(&point.sessions).await,
            Ok(Ok(())),
            "the reading session opened"
        );
        let ran = point.sessions.a_command(A_COMMAND, None).await;
        assert!(ran.is_ok(), "the command took the one session: {ran:?}");
        assert_eq!(
            a_stat(&point.sessions).await,
            Ok(Ok(())),
            "reading reopened after it"
        );
        // **Now the limit is known**, and a second command closes the reading
        // session before it asks — no refusal is needed to teach it again.
        let ran = point.sessions.a_command(A_COMMAND, None).await;
        assert!(ran.is_ok(), "{ran:?}");
        point.ends().await;

        let happened = machine.what_happened();
        let commands: Vec<&String> = happened
            .iter()
            .filter(|it| it.contains("a command ran"))
            .collect();
        assert_eq!(commands.len(), 2);
        for ran in commands {
            assert_eq!(
                ran, "primary: a command ran, 1 open",
                "no other session was open: {happened:#?}"
            );
        }
        // **What the point held when it asked**, counted at its own end: after
        // the first command taught the limit, every command was asked with the
        // reading session already let go. A refusal after that is the machine
        // still freeing the session, and it is asked again rather than learned
        // from.
        let asked: Vec<&String> = happened
            .iter()
            .filter(|it| it.contains("a command was asked for"))
            .collect();
        assert_eq!(
            asked.first().map(|it| it.as_str()),
            Some("primary: a command was asked for, the point holding 1")
        );
        let after_the_limit_was_learned: Vec<&&String> = asked
            .iter()
            .skip_while(|it| it.ends_with("holding 1"))
            .collect();
        assert!(!after_the_limit_was_learned.is_empty());
        for it in &after_the_limit_was_learned {
            assert_eq!(
                it.as_str(),
                "primary: a command was asked for, the point holding 0",
                "once the limit is learned, the reading session is closed before the command \
                 asks: {happened:#?}"
            );
        }
    }

    /// **`Server protocol and transfers` T5: no reading begins while a command
    /// waits.** A command asked behind a reading goes before a reading asked
    /// after it.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_waiting_command_goes_before_a_reading_asked_before_it_was_served() {
        let machine = AMachine::with_a_limit_of(1);
        let point = a_point_lending(&machine, additional_connections_to(&machine, false)).await;

        // The first reading holds the point until it is let go.
        let (let_go, held) = oneshot::channel::<()>();
        let mut first = Box::pin(point.sessions.a_reading(true, {
            let machine = Arc::clone(&machine);
            move |subsystem| {
                Box::pin(async move {
                    let _ = held.await;
                    machine.says("the first reading".to_string());
                    subsystem.stat(b".").await.map(|_| ())
                })
            }
        }));
        asked_now(&mut first).await;
        while !machine
            .what_happened()
            .iter()
            .any(|it| it.contains("a session opened"))
        {
            tokio::task::yield_now().await;
        }
        let mut second = Box::pin(point.sessions.a_reading(true, {
            let machine = Arc::clone(&machine);
            move |subsystem| {
                Box::pin(async move {
                    machine.says("a reading asked before the command".to_string());
                    subsystem.stat(b".").await.map(|_| ())
                })
            }
        }));
        asked_now(&mut second).await;
        let mut command = Box::pin(point.sessions.a_command(A_COMMAND, None));
        asked_now(&mut command).await;
        let _ = let_go.send(());
        assert!(first.await.is_ok());
        assert!(command.await.is_ok());
        assert!(second.await.is_ok());
        point.ends().await;
        let happened: Vec<String> = machine
            .what_happened()
            .into_iter()
            .filter(|it| it.contains("reading") || it.contains("a command ran"))
            .collect();
        assert_eq!(
            happened,
            vec![
                "the first reading",
                "primary: a command ran, 1 open",
                "a reading asked before the command",
            ]
        );
    }

    /// **Step 10c's eighth pass on Hyperstack, at the point: above a learned
    /// limit of one, or where none is learned, a second ask is not held behind a
    /// command that is still running.** Added 2026-09-27, eighth-pass fix piece
    /// 1.
    ///
    /// **What happened there.** The driver install ran 52 seconds as one command
    /// through this point, and the point served nothing else until it ended —
    /// so the run's own state reading (`Server Setup` Y7's refresh), which is a
    /// reading and a command of its own, waited behind it, and the machine
    /// ended the run for want of one. **`Server protocol and transfers` T5
    /// reserves a slot for commands above a limit of one and does not say one
    /// command at a time**, and `Server Setup` §13 item 7's paragraph of
    /// 2026-09-27 says the refresh goes out on its own session beside the admin
    /// step there.
    ///
    /// **The machine allows ten sessions and the point has learned no limit**,
    /// which is every machine on record. The reading and a second command are
    /// each answered while the slow command still runs, and the slow command
    /// is answered after.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn above_a_learned_limit_of_one_a_second_ask_is_not_held_behind_a_running_command() {
        let machine = AMachine::with_a_limit_of(10);
        let point = a_point_lending(&machine, additional_connections_to(&machine, false)).await;

        let mut slow = Box::pin(point.sessions.a_command(A_SLOW_COMMAND, None));
        asked_now(&mut slow).await;
        machine.has_said("the slow command began").await;

        let read = tokio::time::timeout(Duration::from_secs(5), a_stat(&point.sessions)).await;
        assert_eq!(
            read,
            Ok(Ok(Ok(()))),
            "the reading was held behind the command still running: {:#?}",
            machine.what_happened()
        );
        let beside = tokio::time::timeout(
            Duration::from_secs(5),
            point.sessions.a_command(A_COMMAND, None),
        )
        .await;
        assert!(
            matches!(beside, Ok(Ok(_))),
            "a second command was held behind the one still running: {beside:?} {:#?}",
            machine.what_happened()
        );
        assert!(
            !machine
                .what_happened()
                .iter()
                .any(|it| it.contains("the slow command ended")),
            "the slow command had ended, so nothing was asked beside it"
        );

        machine.lets_the_slow_command_end();
        assert!(slow.await.is_ok(), "the slow command was answered");
        point.ends().await;
    }

    /// **`Server Setup` Y7a at the point: what a command prints is passed up
    /// as it prints**, each stream on its own, while the command is still
    /// running — and the answer is the same whole result. **A connection that
    /// cannot pass pieces up as they arrive passes them up whole at the end**,
    /// before the answer. Added 2026-10-18, the Creator's thirteenth notes,
    /// piece H. Before it, no ask could be told anything before its answer.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn y7a_what_a_command_prints_is_passed_up_while_it_runs() {
        let machine = AMachine::with_a_limit_of(10);
        let point = a_point_lending(&machine, additional_connections_to(&machine, false)).await;

        let (up, mut pieces) = mpsc::unbounded_channel();
        let mut slow = Box::pin(point.sessions.a_command_passing_up(A_SLOW_COMMAND, None, up));
        asked_now(&mut slow).await;
        let first = tokio::time::timeout(Duration::from_secs(5), async {
            loop {
                tokio::select! {
                    piece = pieces.recv() => return piece,
                    _ = &mut slow => panic!("the command was answered before anything it printed was passed up"),
                }
            }
        })
        .await
        .expect("nothing the command printed was passed up while it ran");
        assert_eq!(first, Some(APiecePrinted::Output(b"Unpacking nvidia-driver-595 ...\n".to_vec())));
        assert_eq!(
            pieces.recv().await,
            Some(APiecePrinted::Complaint(b"debconf: delaying\n".to_vec())),
            "the complaints stream is passed up on its own"
        );
        assert!(
            !machine.what_happened().iter().any(|it| it.contains("the slow command ended")),
            "the pieces came after the command ended, so this proves nothing"
        );
        machine.lets_the_slow_command_end();
        assert!(slow.await.is_ok(), "the command was answered");

        // **The provided route: whole, at the end, before the answer.** The
        // entry point with no link standing complains, as a shell does.
        let (up, mut pieces) = mpsc::unbounded_channel();
        let ran = point
            .sessions
            .a_command_passing_up(THE_HEALTH_EXCHANGE_COMMAND, None, up)
            .await
            .expect("the command ran");
        let mut told = Vec::new();
        while let Ok(piece) = pieces.try_recv() {
            told.push(piece);
        }
        let mut expected = Vec::new();
        if !ran.output.is_empty() {
            expected.push(APiecePrinted::Output(ran.output.clone()));
        }
        if !ran.complaint.is_empty() {
            expected.push(APiecePrinted::Complaint(ran.complaint.clone()));
        }
        assert!(!expected.is_empty(), "the command printed nothing, so this proves nothing");
        assert_eq!(told, expected, "the whole result was not passed up before the answer");
        point.ends().await;
    }

    /// **`Server Setup` Y8 at the point: a command whose asker stops waiting
    /// is given up, and its channel closed** — *"the desktop stops issuing and
    /// closes the step's channel"*. Added 2026-09-27, eighth-pass fix piece 1.
    ///
    /// **Its session comes back with it**, so a command asked after it runs.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_running_command_nobody_waits_for_any_more_has_its_channel_closed() {
        let machine = AMachine::with_a_limit_of(10);
        let point = a_point_lending(&machine, additional_connections_to(&machine, false)).await;
        let mut slow = Box::pin(point.sessions.a_command(A_SLOW_COMMAND, None));
        asked_now(&mut slow).await;
        machine.has_said("the slow command began").await;
        drop(slow);
        machine
            .has_said("the slow command's channel was closed")
            .await;
        assert!(point.sessions.a_command(A_COMMAND, None).await.is_ok());
        point.ends().await;
        assert!(
            !machine
                .what_happened()
                .iter()
                .any(|it| it.contains("the slow command ended")),
            "the command given up was run to its end anyway"
        );
    }

    /// **`Server protocol and transfers` T5 at a learned limit of one, kept
    /// exactly: the one lease is the running command's for its whole length,
    /// and a reading asked meanwhile waits for it to end.** Added 2026-09-27,
    /// eighth-pass fix piece 1, beside the test above.
    ///
    /// **That is `Server Setup` §13 item 7's residue as the plan pass wrote
    /// it**: on such a machine an admin step longer than the window holds the
    /// only session, no refresh can go out beside it, and the window ends the
    /// run — a recovery run's cost and never harm.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn at_a_learned_limit_of_one_a_reading_waits_for_the_running_command() {
        let machine = AMachine::with_a_limit_of(1);
        let point = a_point_lending(&machine, additional_connections_to(&machine, false)).await;
        assert_eq!(a_stat(&point.sessions).await, Ok(Ok(())));
        // **The refusal that teaches the limit.**
        assert!(point.sessions.a_command(A_COMMAND, None).await.is_ok());

        let mut slow = Box::pin(point.sessions.a_command(A_SLOW_COMMAND, None));
        asked_now(&mut slow).await;
        machine.has_said("the slow command began").await;

        let mut reading = Box::pin(a_stat(&point.sessions));
        let held = tokio::time::timeout(Duration::from_millis(500), &mut reading).await;
        assert!(
            held.is_err(),
            "a reading was served beside the one lease at a limit of one: {:#?}",
            machine.what_happened()
        );
        machine.lets_the_slow_command_end();
        assert!(slow.await.is_ok());
        assert_eq!(reading.await, Ok(Ok(())));
        point.ends().await;

        let happened = machine.what_happened();
        let ended = happened
            .iter()
            .position(|it| it.contains("the slow command ended"))
            .expect("the slow command ended");
        let reopened = happened
            .iter()
            .rposition(|it| it.contains("a session opened"))
            .expect("the reading session reopened");
        assert!(
            ended < reopened,
            "the reading session opened before the command holding the one lease ended: \
             {happened:#?}"
        );
    }

    /// **`Server protocol and transfers` §1.1 item 2: a reading the user is
    /// waiting on goes before one nobody is waiting on.**
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_reading_someone_waits_on_goes_before_one_nobody_waits_on() {
        let machine = AMachine::with_a_limit_of(10);
        let point = a_point_lending(&machine, additional_connections_to(&machine, false)).await;
        let (let_go, held) = oneshot::channel::<()>();
        let mut first = Box::pin(point.sessions.a_reading(true, move |subsystem| {
            Box::pin(async move {
                let _ = held.await;
                subsystem.stat(b".").await.map(|_| ())
            })
        }));
        asked_now(&mut first).await;
        while !machine
            .what_happened()
            .iter()
            .any(|it| it.contains("a session opened"))
        {
            tokio::task::yield_now().await;
        }
        let a_reading_called = |someone_is_waiting: bool, called: &'static str| {
            let machine = Arc::clone(&machine);
            Box::pin(
                point
                    .sessions
                    .a_reading(someone_is_waiting, move |subsystem| {
                        Box::pin(async move {
                            machine.says(called.to_string());
                            subsystem.stat(b".").await.map(|_| ())
                        })
                    }),
            )
        };
        let mut walk = a_reading_called(false, "a walk");
        asked_now(&mut walk).await;
        let mut listing = a_reading_called(true, "the user's listing");
        asked_now(&mut listing).await;
        let _ = let_go.send(());
        assert!(first.await.is_ok());
        assert!(walk.await.is_ok());
        assert!(listing.await.is_ok());
        point.ends().await;
        let happened: Vec<String> = machine
            .what_happened()
            .into_iter()
            .filter(|it| !it.contains("session"))
            .collect();
        assert_eq!(happened, vec!["the user's listing", "a walk"]);
    }

    /// **`Servers` S14 and `Servers` Q16, and `Server protocol and transfers` T2: at a
    /// limit of one a transfer takes the additional connection, which carries
    /// transfers alone and is closed when the last one on it ends** — while a
    /// command still runs on the primary.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn at_a_limit_of_one_a_transfer_rides_the_additional_connection_and_it_closes_after() {
        let machine = AMachine::with_a_limit_of(1);
        let additional = AMachine::with_a_limit_of(1);
        let point = a_point_lending(&machine, additional_connections_to(&additional, true)).await;

        assert_eq!(a_stat(&point.sessions).await, Ok(Ok(())));
        let channel = point.sessions.a_transfer_channel().await.expect("a seat");
        assert!(channel.is_on_the_additional_connection());
        assert!(point.sessions.a_command(A_COMMAND, None).await.is_ok());
        assert_eq!(
            additional
                .what_happened()
                .iter()
                .filter(|it| it.contains("command"))
                .count(),
            0
        );
        drop(channel);
        while !additional
            .what_happened()
            .iter()
            .any(|it| it.contains("torn down"))
        {
            tokio::task::yield_now().await;
        }
        point.ends().await;
    }

    /// **`Servers` S14 condition 3: where the additional connection cannot be
    /// made, the transfer is unavailable on this connection rather than
    /// failed.**
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_transfer_with_no_additional_connection_to_be_had_is_unavailable() {
        let machine = AMachine::with_a_limit_of(1);
        let point = a_point_lending(&machine, additional_connections_to(&machine, false)).await;
        assert_eq!(a_stat(&point.sessions).await, Ok(Ok(())));
        assert_eq!(
            point.sessions.a_transfer_channel().await.err(),
            Some(NotLent::UnavailableOnThisConnection)
        );
        point.ends().await;
    }

    /// **Above a limit of one, a transfer never takes the slot kept for a
    /// command**, and the reading session is counted whether or not it is
    /// open.
    #[test]
    fn above_a_limit_of_one_the_primary_keeps_room_for_the_reading_session_and_a_command() {
        for (limit, transfers, has_room) in [
            (10, 8, false),
            (10, 7, true),
            (3, 0, true),
            (3, 1, false),
            (2, 0, false),
        ] {
            let pool = ThePool {
                learned: Some(limit),
                transfers,
                a_release_is_in_flight: false,
            };
            assert_eq!(
                pool.learned.is_none_or(|limit| pool.transfers + 3 <= limit),
                has_room,
                "a limit of {limit} with {transfers} transfers seated"
            );
        }
    }

    /// **`Servers` S15: a machine that permits no session is said to, and
    /// nothing else is claimed.**
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_machine_that_permits_no_session_is_said_to() {
        let machine = AMachine::with_a_limit_of(0);
        let point = a_point_lending(&machine, additional_connections_to(&machine, false)).await;
        assert_eq!(
            a_stat(&point.sessions).await,
            Err(NotLent::TheMachineAllowsNoSession)
        );
        assert_eq!(
            point.sessions.a_command(A_COMMAND, None).await,
            Err(NotLent::TheMachineAllowsNoSession)
        );
        point.ends().await;
    }

    /// **A fresh point meeting a refusal with nothing held asks again before it
    /// takes the refusal as the machine's limit** — step 10c piece 4's finding,
    /// and R1 of piece 6's brief.
    ///
    /// **The machine is what makes it the case**: it allows one session and
    /// still holds one when the point starts — the session of a command
    /// `Servers`' own sequence ran on this connection a moment before, which
    /// the far end frees a moment after. **A point that took the first refusal
    /// as a limit of nought would tell this reading that the machine allows no
    /// session**, and that is what this test fails on.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_fresh_point_asks_again_before_it_calls_a_just_used_session_the_machines_limit() {
        let machine = AMachine::with_a_limit_of(1);
        // **The command's own session, not yet freed by the far end.**
        machine.open.fetch_add(1, Ordering::SeqCst);
        let point = a_point_lending(&machine, additional_connections_to(&machine, false)).await;
        let freeing = tokio::spawn({
            let machine = Arc::clone(&machine);
            async move {
                tokio::time::sleep(Duration::from_millis(150)).await;
                machine.open.fetch_sub(1, Ordering::SeqCst);
            }
        });
        assert_eq!(
            a_stat(&point.sessions).await,
            Ok(Ok(())),
            "the first reading was told the machine allows no session: {:#?}",
            machine.what_happened()
        );
        freeing.await.expect("the session was freed");
        point.ends().await;
        assert!(
            machine
                .what_happened()
                .iter()
                .any(|it| it == "primary: a session was refused"),
            "the refusal this test is about never happened, so it proves nothing"
        );
    }

    /// **Nothing is lent while no connection is live**, and an ask the ending
    /// overtakes is told the connection ended.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn nothing_is_lent_while_no_connection_is_live() {
        let sessions = ThisServersSessions::<AConnectionToIt>::for_one_server();
        assert_eq!(a_stat(&sessions).await, Err(NotLent::NoConnectionIsLive));

        let machine = AMachine::with_a_limit_of(1);
        let point = a_point_lending(&machine, additional_connections_to(&machine, false)).await;
        let sessions = point.sessions.clone();
        let (let_go, held) = oneshot::channel::<()>();
        let holding = tokio::spawn({
            let sessions = sessions.clone();
            async move {
                sessions
                    .a_reading(true, move |_subsystem| {
                        Box::pin(async move {
                            let _ = held.await;
                            Ok(())
                        })
                    })
                    .await
            }
        });
        while machine.what_happened().is_empty() {
            tokio::task::yield_now().await;
        }
        // **Polled once, so the ask is made before the ending** and the test
        // does not race its own asking.
        let mut behind = Box::pin(sessions.a_command(A_COMMAND, None));
        std::future::poll_fn(|cx| {
            let _ = behind.as_mut().poll(cx);
            Poll::Ready(())
        })
        .await;
        // **The ending is asked before any waiting work**, so once the reading
        // in hand is done the command behind it is never run.
        point.end_it.send(true).expect("the point is lending");
        drop(let_go);
        assert_eq!(point.lending.await.expect("it ended"), Ending::Announced);
        assert_eq!(behind.await, Err(NotLent::TheConnectionEnded));
        let _ = holding.await;
        assert_eq!(a_stat(&sessions).await, Err(NotLent::NoConnectionIsLive));
    }

    /// **P5: every session, request and reading the point makes is made on the
    /// thread lending the connection**, and never on the thread that asked.
    #[test]
    fn the_work_is_done_on_the_lending_thread_and_never_on_the_asking_one() {
        let machine = AMachine::with_a_limit_of(10);
        let sessions = ThisServersSessions::<AConnectionToIt>::for_one_server();
        let (end_it, ends) = watch::channel(false);
        let (lending_thread_is, lending_thread) = std::sync::mpsc::channel();
        let lending = std::thread::spawn({
            let sessions = sessions.clone();
            let machine = Arc::clone(&machine);
            move || {
                lending_thread_is.send(std::thread::current().id()).unwrap();
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap();
                let mut connection = AConnectionToIt {
                    machine: Arc::clone(&machine),
                    name: "primary",
                    ends,
                };
                runtime.block_on(lend_until_it_ends(
                    &sessions,
                    &mut connection,
                    &additional_connections_to(&machine, false),
                ))
            }
        });
        let lending_thread = lending_thread.recv().unwrap();
        let asking = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let ran_on = asking.block_on(async {
            while !sessions.a_connection_is_live() {
                tokio::task::yield_now().await;
            }
            sessions
                .a_reading(true, |subsystem| {
                    Box::pin(async move {
                        subsystem
                            .stat(b".")
                            .await
                            .map(|_| std::thread::current().id())
                    })
                })
                .await
        });
        assert_eq!(ran_on, Ok(Ok(lending_thread)));
        assert_ne!(lending_thread, std::thread::current().id());
        end_it.send(true).unwrap();
        lending.join().unwrap();
    }

    /// **A channel lent for a transfer learns when its connection ends, and a
    /// wait on it is answered at once** — step 10c's twelfth-pass fix piece 1,
    /// on the primary and on `Servers` S14's additional connection.
    ///
    /// **The case this bites on is a connection that ends in silence**: the
    /// machine's end of the channel is still there and says nothing, so a
    /// wait on it is ended by nothing the channel itself will ever see — on
    /// the twelfth pass, an upload's write waiting for room on a connection
    /// `Servers` K3 had already declared broken and replaced. **The stand-in's
    /// file server here never speaks unasked**, so a read on the channel
    /// waits exactly that way, and nothing about the channel changes when
    /// the lending ends.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_lent_channel_learns_its_connection_has_ended_on_either_connection() {
        use tokio::io::AsyncReadExt as _;
        for (limit, on_the_additional) in [(10, false), (1, true)] {
            let machine = AMachine::with_a_limit_of(limit);
            let additional = AMachine::with_a_limit_of(1);
            let point =
                a_point_lending(&machine, additional_connections_to(&additional, true)).await;
            if on_the_additional {
                assert_eq!(a_stat(&point.sessions).await, Ok(Ok(())));
            }
            let mut channel = point.sessions.a_transfer_channel().await.expect("a seat");
            assert_eq!(channel.is_on_the_additional_connection(), on_the_additional);
            let waiting = tokio::spawn(async move {
                let mut one = [0u8; 1];
                let read = channel.read(&mut one).await;
                (read.map_err(|failure| failure.kind()), channel)
            });
            tokio::time::sleep(Duration::from_millis(100)).await;
            assert!(!waiting.is_finished(), "the read did not wait");
            point.ends().await;
            let (read, channel) = tokio::time::timeout(Duration::from_secs(5), waiting)
                .await
                .unwrap_or_else(|_| {
                    panic!(
                        "a read on a channel lent on the {} connection went on waiting after it ended",
                        if on_the_additional { "additional" } else { "primary" }
                    )
                })
                .expect("the read did not panic");
            assert_eq!(read, Err(std::io::ErrorKind::BrokenPipe));
            drop(channel);
        }
    }

    /// **`Servers` S14 condition 6: the additional connection ending on its
    /// own, the primary still answering** — the dead-line sweep's extra-connection repair,
    /// 2026-10-19 (`Audits/Build/Step 10c/A dead line under every server act —
    /// the sweep.md`, finding 1).
    ///
    /// **The case it bites on.** The line under the additional connection goes
    /// silent while a transfer's write waits for room on it, and its transport
    /// then reports the connection ended. **Until the repair nothing told the
    /// channel**: only the point letting the connection go did, and the point
    /// heard only the primary's ending — so the write waited for good, though
    /// the primary answered throughout. **And the next transfer was refused**:
    /// the point asked the dead connection for a session, was told it was
    /// gone, and called the route unavailable rather than opening a fresh one.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn the_additional_connection_ending_alone_tells_its_channels_and_leaves_the_primary_lending() {
        use tokio::io::AsyncWriteExt as _;
        let machine = AMachine::with_a_limit_of(1);
        let additional = AMachine::with_a_limit_of(10);
        let (opens, their_ends) = endable_additional_connections_to(&additional);
        let point = a_point_lending(&machine, opens).await;
        assert_eq!(a_stat(&point.sessions).await, Ok(Ok(())));
        let mut channel = point.sessions.a_transfer_channel().await.expect("a seat");
        assert!(channel.is_on_the_additional_connection());
        let its_connection = channel.the_connection_it_sits_on();

        // **The line under the additional connection goes silent**, and a
        // write uses up its room and waits.
        additional.goes_silent.send_replace(true);
        let writing = tokio::spawn(async move {
            let wrote = channel.write_all(&vec![7u8; 1024 * 1024]).await;
            (wrote.map_err(|failure| failure.kind()), channel)
        });
        tokio::time::sleep(Duration::from_millis(200)).await;
        assert!(!writing.is_finished(), "the write did not wait for room");
        assert!(!its_connection.has_ended());

        // **Its transport reports it ended**, and nothing else changes.
        let declared = std::time::Instant::now();
        their_ends.lock().unwrap()[0].send_replace(true);
        let (wrote, channel) = tokio::time::timeout(Duration::from_secs(5), writing)
            .await
            .expect("a write on the additional connection went on waiting after it ended")
            .expect("the write did not panic");
        assert_eq!(wrote, Err(std::io::ErrorKind::BrokenPipe));
        assert!(
            declared.elapsed() < Duration::from_millis(500),
            "the write ended {:?} after the declaration",
            declared.elapsed()
        );
        assert!(its_connection.has_ended());

        // **The primary is undisturbed**: still lending, still reading.
        assert!(point.sessions.a_connection_is_live());
        assert_eq!(a_stat(&point.sessions).await, Ok(Ok(())));
        assert!(!point.lending.is_finished(), "the primary's lending ended");

        // **The next transfer is seated on a fresh additional connection** —
        // asked while the cut one's channel is still held, so the point has
        // not let the dead connection go as idle.
        additional.goes_silent.send_replace(false);
        let fresh = tokio::time::timeout(Duration::from_secs(5), point.sessions.a_transfer_channel())
            .await
            .expect("the next transfer waited for good")
            .expect("the next transfer was refused a seat");
        assert!(fresh.is_on_the_additional_connection());
        assert!(!fresh.the_connection_it_sits_on().has_ended());
        assert_eq!(their_ends.lock().unwrap().len(), 2, "no fresh additional connection was made");
        drop(channel);
        drop(fresh);
        point.ends().await;
    }

    /// **The supervisor asked through the point at a limit of one** — the
    /// link planted on the reading session, the reading session closed for the
    /// command, and reopened to take the link away. **`Server protocol and
    /// transfers` C2: the answer that comes back carries the request's
    /// identity.**
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn the_supervisor_is_asked_through_the_point_at_a_limit_of_one() {
        let machine = AMachine::with_a_limit_of(1);
        let point = a_point_lending(&machine, additional_connections_to(&machine, false)).await;
        let request = Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            Identity::issue(IdentityKind::Operation).expect("randomness"),
            "a request",
        );
        let came_back = point
            .sessions
            .ask_the_supervisor("/srv/smoozy/package", &request)
            .await;
        point.ends().await;
        let WhatCameBack::ItAnswered(answered) = came_back else {
            panic!("{came_back:?}: {:#?}", machine.what_happened());
        };
        assert!(answered.answers(&request));
        assert_eq!(
            answered.fields.text("command"),
            Some("~/.smoozy-carriage/package-area/smoozy-server")
        );
        let happened = machine.what_happened();
        assert_eq!(
            happened
                .iter()
                .filter(|it| it.contains("a command ran"))
                .collect::<Vec<_>>(),
            vec!["primary: a command ran, 1 open"],
            "{happened:#?}"
        );
    }

    /// **A placement asked while a supervisor ask stands between its link and
    /// its command.** Added 2026-09-30, step 10c's eleventh-pass fix piece 4c,
    /// out of piece 4b's sweep of every carriage site (its `entry-point` row).
    ///
    /// The placement takes the whole connection, and a whole-connection act
    /// goes before every command waiting — so where it is asked after a
    /// supervisor ask has planted `Server protocol and transfers` §1.1's one
    /// entry-point link and before that ask's command runs, the placement's
    /// own health exchange plants and takes away a link at the same site, and
    /// **the ask's command then finds no program**: nothing correlated comes
    /// back, which a job send reads as *could not tell* (the eighth pass's
    /// stall on Hyperstack, in another shape).
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_placement_does_not_come_between_a_supervisor_asks_link_and_its_command() {
        let machine = AMachine::with_a_limit_of(10);
        machine.links_may_land.send_replace(false);
        let point = a_point_lending(&machine, additional_connections_to(&machine, false)).await;
        let request = Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            Identity::issue(IdentityKind::Operation).expect("randomness"),
            "a request",
        );
        let asking = tokio::spawn({
            let sessions = point.sessions.clone();
            let request = request.clone();
            async move {
                sessions
                    .ask_the_supervisor("/srv/smoozy/package", &request)
                    .await
            }
        });
        machine.has_said("a link was asked for").await;

        // **The placement is asked now**, polled once so its ask is made while
        // the supervisor ask's link is landing, and the test does not race its
        // own asking.
        let mut placing = Box::pin(point.sessions.the_whole_connection({
            let machine = Arc::clone(&machine);
            move |_connection| {
                Box::pin(async move {
                    // **The placement's own health exchange at the same
                    // site**: its link planted and taken away.
                    machine.a_link_stands.store(true, Ordering::SeqCst);
                    machine.says("the placement's link was planted and taken away".to_string());
                    machine.a_link_stands.store(false, Ordering::SeqCst);
                })
            }
        }));
        std::future::poll_fn(|cx| {
            let _ = placing.as_mut().poll(cx);
            Poll::Ready(())
        })
        .await;

        machine.links_may_land.send_replace(true);
        let came_back = asking.await.expect("the ask ended");
        placing
            .await
            .expect("the placement was lent the connection");
        point.ends().await;
        assert!(
            matches!(&came_back, WhatCameBack::ItAnswered(answered) if answered.answers(&request)),
            "{came_back:?}: {:#?}",
            machine.what_happened()
        );
    }

    /// **The worldwide pass's fault 1: the entry point's link stands between
    /// supervisor asks on one connection**, so the asks after the first are
    /// one command each and no file-transfer request. Added 2026-10-26. **It
    /// is planted again wherever it may not stand**: after an act that takes
    /// the whole connection, after an ask whose command found no program, and
    /// on a new lending. Before this, every ask planted it and took it away —
    /// four of the line's round trips beside one command of about two.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn worldwide_fault_1_the_entry_points_link_stands_between_asks_on_one_connection() {
        let machine = AMachine::with_a_limit_of(10);
        let planted = |machine: &AMachine| {
            machine
                .what_happened()
                .iter()
                .filter(|it| it.contains("a link was asked for"))
                .count()
        };
        let asked = |sessions: ThisServersSessions<AConnectionToIt>| async move {
            let request = Request::speaking(
                THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
                Identity::issue(IdentityKind::Operation).expect("randomness"),
                "a request",
            );
            matches!(
                sessions.ask_the_supervisor("/srv/smoozy/package", &request).await,
                WhatCameBack::ItAnswered(answered) if answered.answers(&request)
            )
        };
        let point = a_point_lending(&machine, additional_connections_to(&machine, false)).await;
        for _ in 0..3 {
            assert!(asked(point.sessions.clone()).await, "{:#?}", machine.what_happened());
        }
        assert_eq!(planted(&machine), 1, "three asks planted more than once: {:#?}", machine.what_happened());
        assert!(machine.a_link_stands.load(Ordering::SeqCst), "the link was taken away");

        // **An act that takes the whole connection**, which plants and takes
        // away a link at the same site: the next ask plants again.
        point
            .sessions
            .the_whole_connection({
                let machine = Arc::clone(&machine);
                move |_connection| {
                    Box::pin(async move {
                        machine.a_link_stands.store(false, Ordering::SeqCst);
                    })
                }
            })
            .await
            .expect("the whole connection was lent");
        assert!(asked(point.sessions.clone()).await, "{:#?}", machine.what_happened());
        assert_eq!(planted(&machine), 2, "{:#?}", machine.what_happened());

        // **A link taken away behind the point's back**: that ask's command
        // finds no program and brings nothing correlated back, and the next
        // ask plants again.
        machine.a_link_stands.store(false, Ordering::SeqCst);
        assert!(!asked(point.sessions.clone()).await, "an ask through no link was answered");
        assert!(asked(point.sessions.clone()).await, "{:#?}", machine.what_happened());
        assert_eq!(planted(&machine), 3, "{:#?}", machine.what_happened());
        let sessions = point.sessions.clone();
        point.ends().await;

        // **A new lending of the same point finds nothing standing.**
        let (end_it, ends) = watch::channel(false);
        let mut connection = AConnectionToIt {
            machine: Arc::clone(&machine),
            name: "primary",
            ends,
        };
        let additional = additional_connections_to(&machine, false);
        let lending = tokio::spawn({
            let sessions = sessions.clone();
            async move { lend_until_it_ends(&sessions, &mut connection, &additional).await }
        });
        while !sessions.a_connection_is_live() {
            tokio::task::yield_now().await;
        }
        assert!(asked(sessions.clone()).await, "{:#?}", machine.what_happened());
        assert_eq!(planted(&machine), 4, "a new lending found the old one's link: {:#?}", machine.what_happened());
        end_it.send(true).expect("the point is lending");
        lending.await.expect("the point ended by itself");
    }

}
