//! The driving itself — opening, authenticating, holding up, tearing down.
//!
//! **This is the one file in this project that opens a socket to a rented
//! machine**, and everything it does is `Servers` K1a's *"inside its own
//! process, on the SSH transport it drives"*. No `ssh`, `sftp` or `scp`
//! process is run here at any moment.
//!
//! **Why a library rather than a client program, said once.** K1a is the
//! ruling and this is its consequence: an SSH transport is a protocol, and
//! Smoozy speaks it. The library is `russh`, pinned at an exact version in
//! `App/Cargo.toml` like every other dependency. What was read of it before it
//! was taken, with the reasoning, is in `Audits/Build/Server protocol and
//! transfers/Decisions record.md`.
//!
//! **The ordering that matters is structural here and is not a setting.** The
//! host key is compared inside key exchange, in [`Trust::check_server_key`],
//! which the protocol places before authentication is asked for at all — RFC
//! 4253 §8 step 3 and §10, quoted at `Security and licence` T6. There is no
//! path through this file where a credential goes first, and there is no
//! `known_hosts` file anywhere in it (`Security and licence` T2: *"there is no
//! client to hand one to"* — that document's T2 and not this one's, which is
//! about concurrent subsystem channels).
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Servers` D9, K1a, K2, K3, K3a, K5, K10, P8, Q1,
//! Q6, Q9. `Security and licence` R2a, T2a.
//!
//! **`Servers` P1, P2 and P3 are deliberately not listed above and are written
//! out with their document at each site**, because this plan defines a P1 and a
//! P3 of its own and the header list is not enough for a marker that is a real
//! clause of both documents.

use std::sync::Arc;

use russh::client::{self, DisconnectReason, Handle};
use russh::keys::{load_secret_key, PrivateKeyWithHashAlg};
use russh::{ChannelMsg, Disconnect};
use smoozy_security_and_licence::plaintext::Plaintext;
use tokio::sync::watch;

use super::{
    CarriesAValueOnACommandsStandardInput, CommandResult, CommandString, Credential,
    DecideHostTrust, Ending, Endpoint, Figures, HostTrust, OpenFailure,
};

/// The handler russh drives the session through.
///
/// It holds two things and no more: the caller's host-trust decision, asked
/// once during key exchange, and the sender the ending is reported on.
struct Trust {
    decide: Box<dyn DecideHostTrust>,
    ending: watch::Sender<Option<Ending>>,
}

impl client::Handler for Trust {
    type Error = russh::Error;

    /// **The comparison, at the one moment it can be made before a credential
    /// exists** — `Server protocol and transfers` A5 and `Security and licence`
    /// T2a. What goes out of here is a decision; the key value itself never
    /// leaves (`Servers` D9).
    async fn check_server_key(
        &mut self,
        offered: &russh::keys::PublicKeyOrCertificate,
    ) -> Result<bool, Self::Error> {
        let bytes = match offered {
            russh::keys::PublicKeyOrCertificate::PublicKey { key, .. } => {
                key.to_bytes().map_err(|_| russh::Error::UnknownKey)?
            }
            // A certificate is a different kind of statement about the machine
            // and nothing in this architecture records one. **Q9's shape**: no
            // claim is made about the machine, and the attempt stops.
            russh::keys::PublicKeyOrCertificate::Certificate(_) => return Ok(false),
        };
        match self.decide.decide(&bytes) {
            HostTrust::Proceed => Ok(true),
            HostTrust::Stop => Ok(false),
        }
    }

    /// **K2, both bullets, and the one ending that is not a break.**
    ///
    /// The keepalive timeout is russh's own count reaching the figure it was
    /// given, which is K3's mechanism; everything else that arrives here after
    /// key exchange is the connection having closed, which is K2's first
    /// bullet — a machine closing politely and a machine powered off were
    /// **both told to the caller in the same second**.
    async fn disconnected(
        &mut self,
        reason: DisconnectReason<Self::Error>,
    ) -> Result<(), Self::Error> {
        let ending = match reason {
            DisconnectReason::ReceivedDisconnect(_) => Ending::Announced,
            DisconnectReason::Error(russh::Error::KeepaliveTimeout) => {
                Ending::TheKeepaliveWentUnanswered
            }
            // **Everything else the transport can fail with is folded here,
            // and that is a decision rather than a default.** K2 names two
            // routes and `Servers`' `BreakRoute` has two variants; a third
            // value invented for a decode fault or a socket error would be a
            // fourth thing for every reader of that enum to handle, and the
            // plan admits none. What all of these have in common is the one
            // thing that matters downstream: the connection is over and no
            // keepalive count produced it, so `Servers` tears it down and
            // repairs, which is K5 either way.
            DisconnectReason::Error(_) => Ending::Announced,
        };
        // **A tear-down Smoozy started keeps its own name.** The far end
        // answers our disconnect by closing, which would otherwise arrive
        // here as an announced break; whoever wrote first wins, and the
        // tear-down writes before it asks.
        self.ending.send_if_modified(|held| {
            if held.is_none() {
                *held = Some(ending);
                true
            } else {
                false
            }
        });
        Ok(())
    }
}

/// A live connection, and the whole of what a caller may do with one.
pub struct Connection {
    handle: Handle<Trust>,
    ending: watch::Receiver<Option<Ending>>,
    ending_sender: watch::Sender<Option<Ending>>,
}

impl std::fmt::Debug for Connection {
    /// **§13's redaction, and `Servers` D9.** A connection knows the host key
    /// it was opened against and the path of the key it authenticated with,
    /// and neither may reach a log. What this renders is the one thing that is
    /// nobody's secret: whether it has ended, and how.
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Connection")
            .field("ended", &self.ended())
            .finish()
    }
}

impl Connection {
    /// **How a break is learned of without using the connection to learn it** —
    /// `State, identity and recovery` §8.5's requirement on `Servers`, and K10's
    /// prohibition: *"No request is made to learn whether the connection is
    /// alive."* This reads a value the transport reported on its own; it sends
    /// nothing.
    pub fn ended(&self) -> Option<Ending> {
        *self.ending.borrow()
    }

    /// Waits for the connection to end, however it ends.
    ///
    /// **This is what a caller holding a period waits on**, rather than
    /// polling. Nothing is sent, so K10 holds here too.
    pub async fn wait_until_it_ends(&mut self) -> Ending {
        loop {
            if let Some(ending) = *self.ending.borrow_and_update() {
                return ending;
            }
            if self.ending.changed().await.is_err() {
                // The session task is gone without having said why, which is
                // the connection having closed under us. K2's first bullet.
                return Ending::Announced;
            }
        }
    }

    /// **The same wait, holding nothing of the connection** — so a caller that
    /// has lent the connection itself to a piece of work can still hear its
    /// end while that work goes on. Added 2026-10-02, the fixes before 0.1.8,
    /// fault 1: `Desktop app and delivery`'s one point per server waited on
    /// whatever it was doing and heard the connection's end only between two
    /// pieces of work, so a piece of work that never ended left the server
    /// reading *connected* for good.
    ///
    /// **It answers with what the transport wrote**, exactly as
    /// [`Self::wait_until_it_ends`] does, and sends nothing (K10).
    pub fn its_ending(&self) -> impl std::future::Future<Output = Ending> + Send + 'static {
        let mut ending = self.ending.clone();
        async move {
            loop {
                if let Some(ended) = *ending.borrow_and_update() {
                    return ended;
                }
                if ending.changed().await.is_err() {
                    return Ending::Announced;
                }
            }
        }
    }

    /// One `exec` request, one command string, one correlated result — §1.1 and
    /// P1's *"One adapter invocation frames exactly one request and returns
    /// exactly one correlated result."*
    ///
    /// **The command is a [`CommandString`], so nothing of user, provider or
    /// machine origin can be in it.** That is §1.1's prohibition kept by the
    /// type rather than by care here.
    pub async fn run_one_command(
        &mut self,
        command: CommandString,
    ) -> Result<CommandResult, CommandFailure> {
        let mut channel = self.one_exec_request(command).await?;
        // **Nothing is written to the channel on this route**, and the far end
        // is told so at once: a program that reads its input waits for a close
        // it would otherwise never get.
        channel
            .eof()
            .await
            .map_err(|_| CommandFailure::TheRequestDidNotReachTheMachine)?;
        Ok(what_the_command_returned(&mut channel).await)
    }

    /// **§1.1's second carriage, driven** — one `exec` request whose value
    /// rides *"the constant command's standard input, read as data"* rather
    /// than the command string.
    ///
    /// **The command is still a [`CommandString`]**, so §1.1's prohibition on
    /// interpolation is kept by the same type on this route as on the other:
    /// what changes is where the value goes, not what a command string may
    /// hold. **The two phases differ only in what the constant string is**, and
    /// this route is available in both — before hand-over it carries the
    /// elevating command's password (§1.3), and after it carries whatever a
    /// constant command's own documentation says is data.
    ///
    /// **`Security and licence` §4 R2a's four rules bound what this costs, and
    /// three of them are the type's.** The value arrives as a [`Plaintext`]:
    /// it cannot have been a process argument or an environment value to get
    /// here, it cannot be written to a record or a log, and it erases itself.
    /// **The fourth — never handed to another module — is kept by this being
    /// the consuming endpoint** rather than a hop on the way to one:
    /// `Security and licence` §1.4 names writing the elevating command's
    /// standard input as hop 3 itself.
    ///
    /// **One copy of the bytes exists inside this call and is erased before it
    /// returns**, because the write is awaited and `Plaintext::expose` hands
    /// its bytes out only for the length of one synchronous call. **What this
    /// code cannot reach is the transport library's own send buffer**, which
    /// holds the same bytes until the packet is on the wire; that is stated
    /// rather than claimed away.
    pub async fn run_one_command_carrying_a_value_on_its_standard_input(
        &mut self,
        command: CommandString,
        value: Plaintext,
    ) -> Result<CommandResult, CommandFailure> {
        let mut channel = self.one_exec_request(command).await?;

        let mut carried = value.expose(<[u8]>::to_vec);
        let written = channel.data(&carried[..]).await;
        // **The same erasure `Security and licence`'s own `Plaintext` performs,
        // at a second site rather than as a second rule** — that document owns
        // the rule and this is the one place in this crate that holds a
        // secret's bytes in a buffer of its own. `write_volatile` so the
        // compiler may not drop the write as dead.
        for byte in carried.iter_mut() {
            unsafe { std::ptr::write_volatile(byte, 0) };
        }
        drop(carried);
        written.map_err(|_| CommandFailure::TheRequestDidNotReachTheMachine)?;

        // **The close is part of the carriage rather than tidying.** A program
        // reading its password from standard input reads until the stream ends,
        // so a route that never ends it hangs on every machine.
        channel
            .eof()
            .await
            .map_err(|_| CommandFailure::TheRequestDidNotReachTheMachine)?;
        Ok(what_the_command_returned(&mut channel).await)
    }

    /// **§1.1's second carriage again, with a framed request as the value** —
    /// one `exec` request whose bytes ride *"the constant command's standard
    /// input, read as data"*.
    ///
    /// **Why this exists beside the route above rather than instead of it.**
    /// That route takes a [`Plaintext`], which is `Security and licence` §4
    /// R2a's shape for a secret and erases itself. **A framed request is not a
    /// secret** — §2's own
    /// [`smoozy_framing::framing::NOTHING_ABOUT_A_REQUESTS_FRAMING_IS_A_SECRET`]
    /// says so — and putting one through that type would say the opposite in
    /// the one place a reader looks to find out.
    ///
    /// **What makes this route admissible is the receiving program's own
    /// documentation, which is §1.1's condition and is met here by the program
    /// being Smoozy's own.** §1.1: after hand-over Smoozy sends the active
    /// package's fixed entry-point name *"and sends every value on that entry
    /// point's standard input in this document's length-delimited frame"*.
    /// **So this is the only route the health exchange has from the desktop**,
    /// and it carries nothing else.
    ///
    /// **The close is part of the carriage rather than tidying**, exactly as it
    /// is on the route above: the adapter reads one frame and then reads to the
    /// end of its input, so a route that never ends the stream hangs.
    pub async fn run_one_command_carrying_a_framed_request_on_its_standard_input(
        &mut self,
        command: CommandString,
        frame: &[u8],
    ) -> Result<CommandResult, CommandFailure> {
        let mut channel = self.one_exec_request(command).await?;
        channel
            .data(frame)
            .await
            .map_err(|_| CommandFailure::TheRequestDidNotReachTheMachine)?;
        channel
            .eof()
            .await
            .map_err(|_| CommandFailure::TheRequestDidNotReachTheMachine)?;
        Ok(what_the_command_returned(&mut channel).await)
    }

    /// **One command begun, with what rides its standard input written and
    /// the stream closed — and the rest of it, the reading of what it returns,
    /// handed back to be done wherever the caller does it.** Added 2026-09-27,
    /// step 10c's eighth-pass fix piece 1.
    ///
    /// **Why the two halves are apart.** Every route above holds the connection
    /// for the whole of the command, because it reads the command's output
    /// while it holds it. **`Desktop app and delivery`'s one point per server
    /// (T5) serves every ask on a connection from one place**, and a driver
    /// install of 52 seconds read to its end there held that place for all 52
    /// — the run's own state reading waited behind it and the machine ended
    /// the setup run for want of one (step 10c's eighth pass on Hyperstack).
    /// **Opening the session and sending the request is what needs the
    /// connection; reading what comes back on it needs only the channel.**
    ///
    /// **Nothing about the carriage changes**: the same one `exec` request
    /// (§1.1, P1), the same [`CommandString`], the bytes on standard input as
    /// data and the stream then closed, and the same reading of output,
    /// complaint output and exit status ([`what_the_command_returned`]). **What
    /// is carried is not a secret's type here** — the point hands this route a
    /// framed request or, for an elevated step, the bytes the caller already
    /// took out of a [`Plaintext`], exactly as it hands them to
    /// [`Self::run_one_command_carrying_a_framed_request_on_its_standard_input`]
    /// today.
    pub async fn one_command_begun(
        &mut self,
        command: CommandString,
        on_its_standard_input: Option<&[u8]>,
    ) -> Result<ACommandUnderWay, CommandFailure> {
        let channel = self.one_exec_request(command).await?;
        if let Some(bytes) = on_its_standard_input {
            channel
                .data(bytes)
                .await
                .map_err(|_| CommandFailure::TheRequestDidNotReachTheMachine)?;
        }
        channel
            .eof()
            .await
            .map_err(|_| CommandFailure::TheRequestDidNotReachTheMachine)?;
        Ok(ACommandUnderWay {
            channel: Some(channel),
        })
    }

    /// The one session and the one execution request both routes open.
    ///
    /// **This document's own §2 P1 — *one adapter invocation frames exactly one
    /// request*** — is what keeps this private: a caller cannot get the channel
    /// and put a second command on it. **The document is written out because
    /// `Servers` defines a P1 too**, and a bare one here resolves to a real and
    /// wrong clause.
    async fn one_exec_request(
        &mut self,
        command: CommandString,
    ) -> Result<russh::Channel<client::Msg>, CommandFailure> {
        let channel = self
            .handle
            .channel_open_session()
            .await
            .map_err(|_| CommandFailure::NoChannelCouldBeOpened)?;
        channel
            .exec(true, command.as_str())
            .await
            .map_err(|_| CommandFailure::TheRequestDidNotReachTheMachine)?;
        Ok(channel)
    }

    /// **§1.1's second mechanism, opened on this connection** — one
    /// file-transfer subsystem session.
    ///
    /// **T1: each transfer runs on its own channel**, so a stalled or cancelled
    /// transfer cannot stall a command and a command cannot delay a transfer's
    /// bytes. **T5: every session on a connection is admitted through one point
    /// per server**, `MaxSessions` being one pool per connection that a
    /// transfer channel, a command invocation and the reading channel all draw
    /// on — **and this call does not admit itself**. Admission is the caller's,
    /// beside `Desktop app and delivery`'s one desktop-side point.
    ///
    /// **T1 is a choice and not a protocol requirement**: one channel can carry
    /// several files at once — *"There is no limit on the number of outstanding
    /// (non-acknowledged) requests that the client may send to the server"* —
    /// and what T1 buys is the isolation above and nothing else.
    pub async fn open_a_subsystem_channel(
        &self,
    ) -> Result<russh::ChannelStream<client::Msg>, CommandFailure> {
        let channel = self
            .handle
            .channel_open_session()
            .await
            .map_err(|_| CommandFailure::NoChannelCouldBeOpened)?;
        channel
            .request_subsystem(true, "sftp")
            .await
            .map_err(|_| CommandFailure::TheRequestDidNotReachTheMachine)?;
        Ok(channel.into_stream())
    }

    /// **`Servers` S10's endings, from the transport's side.** Smoozy's own
    /// tear-down, which is not a break and reports none. It is one of the six
    /// moments that document's S10 ends a period at. **That document's S10 and
    /// not this one's**, which is about a transfer's accounting.
    ///
    /// It answers with the ending it wrote, so a caller closing a period reads
    /// the same value every other ending is read as.
    pub async fn tear_down(self) -> Ending {
        // **A connection already declared broken is not asked anything on its
        // way out** — the fixes before 0.1.8, fault 1. The request goes through
        // the transport's own loop, and the break may be declared exactly
        // because that loop has stopped: then the request waits in its queue,
        // and once the queue is full it waits for good, and `Servers`' driving
        // with it — the row reading *connected* over a connection already
        // known to be gone. **Nothing at the far end would hear it either**
        // (`Servers` K5: a break declared is a break). Dropping the connection
        // is the whole tear-down.
        let a_break_was_declared = self.ended().is_some_and(Ending::is_a_break);
        // Written before the request goes out, so the far end's answering
        // close cannot rename it.
        self.ending_sender
            .send_replace(Some(Ending::TornDownBySmoozy));
        if a_break_was_declared {
            return Ending::TornDownBySmoozy;
        }
        let _ = self
            .handle
            .disconnect(Disconnect::ByApplication, "", "en-GB")
            .await;
        Ending::TornDownBySmoozy
    }
}

/// **§1.1: one `exec` request returns output, complaint output and exit
/// status** — read the same way on both of [`Connection`]'s command routes,
/// because what a command returns does not depend on how its value reached it.
async fn what_the_command_returned(channel: &mut russh::Channel<client::Msg>) -> CommandResult {
    what_the_command_returned_told_as_it_came(channel, &mut |_, _| {}).await
}

/// **Which of a command's two streams some bytes were printed on** — what
/// [`ACommandUnderWay::to_its_end_told_as_it_comes`] hands over beside the
/// bytes. Added 2026-10-18, the Creator's thirteenth notes, piece H.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheStream {
    /// The standard output.
    Output,
    /// The complaint output — RFC 4254 §5.2's extended data of type 1.
    Complaint,
}

/// **The same reading, with each piece of either stream told the moment it
/// arrives** as well as kept for the result. The result is the same one
/// [`what_the_command_returned`] makes: nothing told is taken out of it.
async fn what_the_command_returned_told_as_it_came(
    channel: &mut russh::Channel<client::Msg>,
    told: &mut (dyn FnMut(TheStream, &[u8]) + Send),
) -> CommandResult {
    let mut result = CommandResult {
        output: Vec::new(),
        complaint: Vec::new(),
        status: None,
    };
    while let Some(message) = channel.wait().await {
        match message {
            ChannelMsg::Data { data } => {
                told(TheStream::Output, &data);
                result.output.extend_from_slice(&data);
            }
            // RFC 4254 §5.2 gives standard error the extended-data type 1.
            // Anything else on that route is not the complaint output and
            // is not read as it.
            ChannelMsg::ExtendedData { data, ext: 1 } => {
                told(TheStream::Complaint, &data);
                result.complaint.extend_from_slice(&data);
            }
            ChannelMsg::ExitStatus { exit_status } => result.status = Some(exit_status),
            ChannelMsg::Close => break,
            _ => {}
        }
    }
    result
}

/// **A command whose request has gone out, and whose result has not yet been
/// read** — [`Connection::one_command_begun`]'s second half. Added 2026-09-27,
/// step 10c's eighth-pass fix piece 1.
///
/// **Given up before its end, its channel is closed.** `Server Setup` Y8: *"a
/// stop arriving during an elevated step ends the driving side's part at once
/// — the desktop stops issuing and closes the step's channel"*. **So dropping
/// this before [`Self::to_its_end`] has finished closes the channel**, which is
/// the one act that clause asks of this side; a channel read to its end was
/// closed by the machine and nothing more is sent.
pub struct ACommandUnderWay {
    /// **Present until the command's result has been read whole.**
    channel: Option<russh::Channel<client::Msg>>,
}

impl ACommandUnderWay {
    /// **The command's output, complaint output and exit status, read to the
    /// channel's close** — the same reading every route above makes.
    pub async fn to_its_end(mut self) -> CommandResult {
        let result = match self.channel.as_mut() {
            Some(channel) => what_the_command_returned(channel).await,
            None => CommandResult {
                output: Vec::new(),
                complaint: Vec::new(),
                status: None,
            },
        };
        self.channel = None;
        result
    }

    /// **The same reading, with each piece of either stream told as it
    /// arrives** — `Server Setup` Y7a: *"An elevated step's two streams are
    /// read by the desktop as they are printed"*. Added 2026-10-18, the
    /// Creator's thirteenth notes, piece H.
    ///
    /// **Nothing about the carriage or the result changes**: what comes back
    /// at the end is [`Self::to_its_end`]'s result, whole, and what was told
    /// along the way is the same bytes in the order they arrived, each with
    /// the stream it came on. **Given up before its end, the channel is
    /// closed** exactly as [`Self::to_its_end`]'s is.
    pub async fn to_its_end_told_as_it_comes(
        mut self,
        mut told: impl FnMut(TheStream, &[u8]) + Send,
    ) -> CommandResult {
        let result = match self.channel.as_mut() {
            Some(channel) => what_the_command_returned_told_as_it_came(channel, &mut told).await,
            None => CommandResult {
                output: Vec::new(),
                complaint: Vec::new(),
                status: None,
            },
        };
        self.channel = None;
        result
    }
}

impl Drop for ACommandUnderWay {
    fn drop(&mut self) {
        let Some(channel) = self.channel.take() else {
            return;
        };
        // **Closing is a message to the machine, so it is sent from the
        // runtime the caller runs on**, and nowhere where there is none — the
        // connection that carried it is then gone too, and its channels with
        // it.
        if let Ok(runtime) = tokio::runtime::Handle::try_current() {
            runtime.spawn(async move {
                let _ = channel.close().await;
            });
        }
    }
}

impl CarriesAValueOnACommandsStandardInput for Connection {
    /// **§1.1's second carriage, reached through the seam beside it.**
    fn run_carrying_a_value_on_its_standard_input(
        &mut self,
        command: CommandString,
        value: Plaintext,
    ) -> impl std::future::Future<Output = Result<CommandResult, CommandFailure>> + Send {
        self.run_one_command_carrying_a_value_on_its_standard_input(command, value)
    }
}

/// Why one command did not produce a result.
///
/// **Neither is a classification of the machine's own words** (§1.3): both are
/// the transport's knowledge of what it was doing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandFailure {
    /// The machine would not open another session. **T5 counts `MaxSessions`
    /// against this connection**, and `Servers` owns what a machine permitting
    /// too few sessions means.
    NoChannelCouldBeOpened,
    /// The channel opened and the request did not land, which is the
    /// connection having gone under it.
    TheRequestDidNotReachTheMachine,
}

impl CommandFailure {
    /// **The same, in standard software words** (Rule 9). Added 2026-09-21,
    /// step 10c: the placement's messages carried this type's debugging
    /// rendering to the screen.
    pub fn in_words(&self) -> &'static str {
        match self {
            CommandFailure::NoChannelCouldBeOpened => {
                "the server would not open another session on this connection"
            }
            CommandFailure::TheRequestDidNotReachTheMachine => {
                "the connection to the server ended before the command reached it"
            }
        }
    }
}

/// Open one connection: key exchange, the host-trust decision, then
/// authentication — in that order, and the order is the protocol's.
///
/// **`Servers` Q1 steps 1 and 3 are both here**, and step 2's key check is the
/// caller's: **`Servers` P1 to `Servers` P3** withhold the attempt where the
/// key file is
/// missing or a passphrase is needed and not available, so a call that reaches
/// this function has already passed it. **That document is written out on this
/// line rather than carried over from Q1 above it** — Q1 is `Servers`' and
/// vouches for nothing after it — because this plan defines a P1 and a P3 of
/// its own, in §2's request framing, and a bare `P1 to P3` here resolves to
/// them.
pub async fn open(
    endpoint: &Endpoint,
    credential: Credential,
    figures: Figures,
    decide: Box<dyn DecideHostTrust>,
) -> Result<Connection, OpenFailure> {
    let config = Arc::new(the_transports_configuration(&figures));

    let (ending_sender, ending) = watch::channel(None);
    let trust = Trust {
        decide,
        ending: ending_sender.clone(),
    };

    let socket = the_socket_to(endpoint, &config).await?;
    let (watched, what_the_line_carried) = TheLineWatched::over(socket);

    let mut handle = match client::connect_stream(config, watched, trust).await {
        Ok(handle) => handle,
        Err(failure) => return Err(why_the_exchange_failed(failure)),
    };

    // **The key-loading step** — `Security and licence` §1.4's third hop, and
    // §4 R2a's in-memory unlock for one authentication exchange. The file is
    // read at the path it was given; nothing copies it and nothing keeps its
    // bytes (`Servers` P8).
    let key = match credential.passphrase {
        Some(passphrase) => passphrase.expose(|bytes| match std::str::from_utf8(bytes) {
            Ok(text) => load_secret_key(&credential.private_key_file, Some(text)),
            // A passphrase that is not text cannot be the one that was typed,
            // and guessing an encoding for it would be inventing a value.
            Err(_) => Err(russh::keys::Error::KeyIsEncrypted),
        }),
        None => load_secret_key(&credential.private_key_file, None),
    };
    let key = key.map_err(|_| OpenFailure::TheKeyCouldNotBeLoaded)?;

    // The hash algorithm an RSA key is signed with is the machine's choice
    // among what it supports, asked for rather than assumed. Every other key
    // kind ignores it.
    let hash = handle
        .best_supported_rsa_hash()
        .await
        .map_err(|_| OpenFailure::TheMachineRefusedTheCredential)?
        .flatten();
    let with_hash = PrivateKeyWithHashAlg::new(Arc::new(key), hash);

    // **A4: nothing here retries an authentication.** One attempt, and its
    // outcome is reported as what it is.
    match handle
        .authenticate_publickey(endpoint.username.as_str(), with_hash)
        .await
    {
        Ok(result) if result.success() => {
            // **K3's count, kept beside the transport's own from here on** —
            // the connection is live, and K3 is about a live connection.
            tokio::spawn(the_line_is_watched(
                Arc::downgrade(&what_the_line_carried),
                figures.the_declaration_window().1,
                ending_sender.clone(),
            ));
            Ok(Connection {
                handle,
                ending,
                ending_sender,
            })
        }
        Ok(_) => Err(OpenFailure::TheMachineRefusedTheCredential),
        Err(_) => Err(OpenFailure::TheExchangeDidNotComplete),
    }
}

/// **The transport's settings for one connection.**
///
/// **K3's figures, handed to the transport as figures of Smoozy's own
/// driving** (`Servers` K1a). russh counts an unanswered request each time the
/// interval falls with nothing received, sends one, and declares the break
/// when the count passes the figure — which is the interval after the last
/// unanswered request, and is exactly K3's third rule.
fn the_transports_configuration(figures: &Figures) -> client::Config {
    client::Config {
        keepalive_interval: Some(figures.interval),
        keepalive_max: figures.unanswered_before_the_break,
        // **K10 and C18.** Nothing else times this connection out: an
        // inactivity timeout would end a connection that is perfectly alive
        // and merely quiet, which is the opposite of what §8.5 asks for.
        inactivity_timeout: None,
        // **Every write leaves at once: Nagle's algorithm is off on this
        // socket.** Added 2026-10-16, step 10c's owed piece 3. russh's own
        // default leaves it on (its `Config::nodelay`: *"disabled by default
        // (i.e. Nagle's algorithm is active)"*), and with it on the system
        // holds every small write while any earlier one is unacknowledged.
        //
        // **The case this connection was first written for** was one thing
        // at a time on it, where a request waits for its answer anyway and
        // the hold costs nothing. **The case it now also meets** is several
        // file-transfer channels on the one connection (T1 and T5): each
        // channel's request is small, so a second channel's request waited a
        // whole round trip behind the first's, and the channels took turns at
        // one request a round trip between them. On the thirteenth pass two
        // channels drained a finished job at 0.82 frames a second, the pace
        // of one `sftp` connection — 14.8 round trips a pair of frames, where
        // the two channels' own steps (a seat's two and a download's five
        // each), taken strictly in turn, are 14.
        //
        // **OpenSSH sets the same on both of its own ends**, which is what
        // that comparison ran with: observed at the desk on 2026-10-16, its
        // `sshd` on a session with no terminal and its `sftp` once the session
        // began both report the option on their sockets, and this socket did
        // not. The account is `Audits/Build/Step 10c/Owed piece 3, a finished
        // job's frames — what landed.md`.
        nodelay: true,
        ..client::Config::default()
    }
}

/// **The socket is made here and handed to the transport watched**, rather
/// than made by the transport itself — see [`TheLineWatched`]. What the
/// transport's own `connect` does before it hands over is done the same way,
/// the setting above included.
async fn the_socket_to(
    endpoint: &Endpoint,
    config: &client::Config,
) -> Result<tokio::net::TcpStream, OpenFailure> {
    let socket = tokio::net::TcpStream::connect((endpoint.host.as_str(), endpoint.port))
        .await
        .map_err(|_| OpenFailure::TheAddressDidNotAnswer)?;
    if config.nodelay {
        let _ = socket.set_nodelay(true);
    }
    Ok(socket)
}

/// Which of the three pre-authentication failures this was.
///
/// **Nothing here reads a message** (§1.3). It reads the kinds russh's own
/// types distinguish, which is the transport's own knowledge rather than a
/// program's words.
fn why_the_exchange_failed(failure: russh::Error) -> OpenFailure {
    match failure {
        russh::Error::IO(_) => OpenFailure::TheAddressDidNotAnswer,
        // **Q6's *It differs* path, and it is not confusable with anything
        // else.** This error has exactly two sites in the library's own
        // source — `src/client/mod.rs` lines 1852 and 1857 of russh 0.63.1,
        // read from the published crate 2026-09-03 — and both are the line
        // immediately after `check_server_key` returned false. So a refusal by
        // the handler is the only thing that produces it, and no second flag
        // is needed to tell it from an ordinary exchange failure.
        russh::Error::UnknownKey => OpenFailure::HostTrustStoppedIt,
        _ => OpenFailure::TheExchangeDidNotComplete,
    }
}

/// **The socket under one connection, with what it carries watched** — so
/// `Servers` K3's count is kept even while the transport cannot keep its own,
/// which is `Servers` K3a. Added 2026-10-02, step 10c's check-before-0.1.7 fix
/// piece 1.
///
/// **The case the transport's own count was written for** is a connection
/// whose line goes quiet while the transport is free to notice: every interval
/// with nothing received, it sends a keepalive request and counts one more
/// unanswered, and the count passing K3's figure is
/// [`Ending::TheKeepaliveWentUnanswered`]. **The case it now also meets** is a
/// line that goes silent under an upload. The transport sends from the same
/// loop that keeps the count, and it waits there until the socket takes all
/// it was given (`russh` 0.63.1, `client/mod.rs`, `run_inner`: the
/// `flush_into(stream_write).await` at the foot of every turn). **A silent line
/// never takes it**, so the loop never turns again: no keepalive is sent, none
/// is counted, and the break is never declared. **At the desk that was five
/// silences in six, through OpenSSH's own server**; on the Hyperstack check it
/// was the first of two identical cuts, which left the server reading
/// *connected* with nothing under it.
///
/// **What is watched is whether the line carried anything at all**: bytes read
/// from the machine, or bytes the socket took from the transport — the socket
/// taking bytes once its buffer is full is the machine acknowledging them.
/// **Where nothing has been carried either way for the whole of K3's window**
/// — the interval times one more than the count, measured from the last thing
/// carried, which is the very moment the transport's own count declares at —
/// the break is declared here with the transport's own word for it, and every
/// wait on the socket is answered at once, so the transport's loop ends and
/// everything waiting on the connection learns it has gone. **Where the loop
/// is free, the transport's own count declares first or together**, and the
/// first word written stands (`Trust::disconnected`'s rule).
///
/// **Nothing is sent to learn this** (`Servers` K10, and `Server protocol and
/// transfers` C18): it reads what the socket already did.
struct TheLineWatched<S> {
    socket: S,
    carried: Arc<WhatTheLineCarried>,
}

/// What [`TheLineWatched`] has seen, shared with the watch that reads it.
struct WhatTheLineCarried {
    /// The last moment the line carried anything, either way.
    last: std::sync::Mutex<tokio::time::Instant>,
    /// **The break was declared from here**, and the socket answers nothing
    /// more.
    given_up: std::sync::atomic::AtomicBool,
    /// The read and the write waiting on the socket, woken when it is given up.
    waiting: std::sync::Mutex<[Option<std::task::Waker>; 2]>,
}

const A_READ: usize = 0;
const A_WRITE: usize = 1;

impl WhatTheLineCarried {
    fn carried_something(&self) {
        if let Ok(mut last) = self.last.lock() {
            *last = tokio::time::Instant::now();
        }
    }

    fn last_carried(&self) -> tokio::time::Instant {
        self.last
            .lock()
            .map(|last| *last)
            .unwrap_or_else(|_| tokio::time::Instant::now())
    }

    fn is_given_up(&self) -> bool {
        self.given_up.load(std::sync::atomic::Ordering::SeqCst)
    }

    fn waits(&self, which: usize, waker: &std::task::Waker) {
        if let Ok(mut waiting) = self.waiting.lock() {
            waiting[which] = Some(waker.clone());
        }
    }

    fn is_given_up_now(&self) {
        self.given_up
            .store(true, std::sync::atomic::Ordering::SeqCst);
        let woken = match self.waiting.lock() {
            Ok(mut waiting) => [waiting[A_READ].take(), waiting[A_WRITE].take()],
            Err(_) => [None, None],
        };
        for waker in woken.into_iter().flatten() {
            waker.wake();
        }
    }
}

fn the_line_was_given_up() -> std::io::Error {
    std::io::Error::new(
        std::io::ErrorKind::TimedOut,
        "the line carried nothing for the whole of the break's window",
    )
}

impl<S> TheLineWatched<S> {
    fn over(socket: S) -> (TheLineWatched<S>, Arc<WhatTheLineCarried>) {
        let carried = Arc::new(WhatTheLineCarried {
            last: std::sync::Mutex::new(tokio::time::Instant::now()),
            given_up: std::sync::atomic::AtomicBool::new(false),
            waiting: std::sync::Mutex::new([None, None]),
        });
        (
            TheLineWatched {
                socket,
                carried: Arc::clone(&carried),
            },
            carried,
        )
    }
}

impl<S: tokio::io::AsyncRead + Unpin> tokio::io::AsyncRead for TheLineWatched<S> {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        if self.carried.is_given_up() {
            return std::task::Poll::Ready(Err(the_line_was_given_up()));
        }
        self.carried.waits(A_READ, cx.waker());
        let before = buf.filled().len();
        let read = std::pin::Pin::new(&mut self.socket).poll_read(cx, buf);
        if matches!(read, std::task::Poll::Ready(Ok(()))) && buf.filled().len() > before {
            self.carried.carried_something();
        }
        read
    }
}

impl<S: tokio::io::AsyncWrite + Unpin> tokio::io::AsyncWrite for TheLineWatched<S> {
    fn poll_write(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        if self.carried.is_given_up() {
            return std::task::Poll::Ready(Err(the_line_was_given_up()));
        }
        self.carried.waits(A_WRITE, cx.waker());
        let wrote = std::pin::Pin::new(&mut self.socket).poll_write(cx, buf);
        if matches!(wrote, std::task::Poll::Ready(Ok(taken)) if taken > 0) {
            self.carried.carried_something();
        }
        wrote
    }

    fn poll_flush(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        if self.carried.is_given_up() {
            return std::task::Poll::Ready(Err(the_line_was_given_up()));
        }
        self.carried.waits(A_WRITE, cx.waker());
        std::pin::Pin::new(&mut self.socket).poll_flush(cx)
    }

    fn poll_shutdown(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        std::pin::Pin::new(&mut self.socket).poll_shutdown(cx)
    }
}

/// **The watch over one live connection's line** — [`TheLineWatched`]'s other
/// half. It declares the break where nothing has been carried for `window`,
/// and it ends the moment the connection has ended however it ended, or its
/// socket has gone with the transport.
async fn the_line_is_watched(
    carried: std::sync::Weak<WhatTheLineCarried>,
    window: std::time::Duration,
    ending: watch::Sender<Option<Ending>>,
) {
    loop {
        if ending.borrow().is_some() {
            return;
        }
        let Some(line) = carried.upgrade() else {
            return;
        };
        let due = line.last_carried() + window;
        if tokio::time::Instant::now() < due {
            drop(line);
            tokio::time::sleep_until(due).await;
            continue;
        }
        // **K3's own word, written first** — so the transport's loop ending
        // on the answered waits below cannot rename it (`Trust::disconnected`
        // keeps the first word written).
        ending.send_if_modified(|held| {
            if held.is_none() {
                *held = Some(Ending::TheKeepaliveWentUnanswered);
                true
            } else {
                false
            }
        });
        line.is_given_up_now();
        return;
    }
}

/// The figures `Servers` K3 states, made reachable to a test in this crate
/// without that crate's types.
///
/// **This is not a second home for them.** `Servers` owns K3 and this is a
/// constant used by this crate's own tests, named so that a reader looking for
/// a duplicated rule finds this sentence instead.
#[cfg(test)]
pub(crate) const K3_AT_ITS_OWN_FIGURES: Figures = Figures {
    interval: std::time::Duration::from_secs(15),
    unanswered_before_the_break: 3,
};

#[cfg(test)]
mod tests {
    use super::*;

    /// **The socket the transport drives sends every write at once** — step
    /// 10c's owed piece 3. With Nagle's algorithm on, a second file-transfer
    /// channel's request waited a round trip behind the first's on a real
    /// line, and two channels moved what one moves. **A desk line cannot show
    /// that hold** — the loopback acknowledges at once — **so the setting
    /// itself is read back off the socket**, made the way [`open`] makes it.
    #[tokio::test]
    async fn the_socket_the_transport_drives_holds_no_write_back() {
        let listener = tokio::net::TcpListener::bind(("127.0.0.1", 0))
            .await
            .expect("the loopback address takes a listener");
        let endpoint = Endpoint {
            host: "127.0.0.1".to_string(),
            port: listener.local_addr().expect("an address").port(),
            username: "nobody".to_string(),
        };
        let config = the_transports_configuration(&K3_AT_ITS_OWN_FIGURES);
        let socket = the_socket_to(&endpoint, &config)
            .await
            .expect("the loopback answers");
        assert!(
            socket.nodelay().expect("the socket's setting reads"),
            "the transport's socket holds a small write while an earlier one is unacknowledged"
        );
    }

    struct Refuses;
    impl DecideHostTrust for Refuses {
        fn decide(&mut self, _offered: &[u8]) -> HostTrust {
            HostTrust::Stop
        }
    }

    /// **Q9: a scan that gets no answer is not a changed identity.** An address
    /// that answers nothing must report a connection that could not be made,
    /// with no claim about the machine — and in particular it must not report
    /// the host-trust refusal, which would say the machine's identity had
    /// changed when nothing was ever offered.
    #[tokio::test]
    async fn an_address_that_does_not_answer_claims_nothing_about_the_machine() {
        // Port 0 cannot be connected to on any of the three systems, which
        // makes this a desk test rather than one that needs a machine.
        let endpoint = Endpoint {
            host: "127.0.0.1".to_string(),
            port: 0,
            username: "nobody".to_string(),
        };
        let credential = Credential {
            private_key_file: std::path::PathBuf::from("/nonexistent"),
            passphrase: None,
        };
        let failure = open(
            &endpoint,
            credential,
            K3_AT_ITS_OWN_FIGURES,
            Box::new(Refuses),
        )
        .await
        .expect_err("no machine is listening on port 0");
        assert!(matches!(failure, OpenFailure::TheAddressDidNotAnswer));
    }

    /// **A line that carries nothing either way for the whole window is a
    /// break, declared with K3's own word, and the wait on it is answered at
    /// once** — [`TheLineWatched`]. Step 10c's check-before-0.1.7 fix piece 1.
    /// **The write here waits the way the transport's does on a silent line**:
    /// the far end reads nothing, so once the buffer is full the write never
    /// completes. Before the fix nothing answered it, and the transport's own
    /// count, kept in the loop that waits there, never turned again.
    #[tokio::test]
    async fn a_line_that_carries_nothing_for_the_window_is_a_break_and_its_wait_is_answered() {
        use tokio::io::AsyncWriteExt;
        let (ours, _theirs) = tokio::io::duplex(64);
        let (mut watched, carried) = TheLineWatched::over(ours);
        let (sender, ending) = watch::channel(None);
        let window = std::time::Duration::from_millis(300);
        tokio::spawn(the_line_is_watched(
            Arc::downgrade(&carried),
            window,
            sender,
        ));
        let began = std::time::Instant::now();
        let wrote = tokio::time::timeout(
            std::time::Duration::from_secs(5),
            watched.write_all(&[0u8; 4096]),
        )
        .await
        .expect("the waiting write is answered");
        assert!(wrote.is_err(), "a write on a line given up went through");
        assert!(
            began.elapsed() >= window,
            "declared before the window: {:?}",
            began.elapsed()
        );
        assert_eq!(*ending.borrow(), Some(Ending::TheKeepaliveWentUnanswered));
    }

    /// **A line that carries something inside every window is no break** —
    /// here the far end reads a little at a time, slower than the write wants
    /// and faster than the window, so the socket goes on taking bytes; that is
    /// the machine acknowledging them, and it is counted.
    #[tokio::test]
    async fn a_line_that_carries_something_inside_every_window_is_not_a_break() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let (ours, mut theirs) = tokio::io::duplex(64);
        let (mut watched, carried) = TheLineWatched::over(ours);
        let (sender, ending) = watch::channel(None);
        let window = std::time::Duration::from_millis(300);
        tokio::spawn(the_line_is_watched(
            Arc::downgrade(&carried),
            window,
            sender,
        ));
        tokio::spawn(async move {
            let mut a_little = [0u8; 64];
            loop {
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                if matches!(theirs.read(&mut a_little).await, Ok(0) | Err(_)) {
                    return;
                }
            }
        });
        let wrote = tokio::time::timeout(
            std::time::Duration::from_secs(10),
            watched.write_all(&[0u8; 1024]),
        )
        .await
        .expect("the slow write ends");
        assert!(wrote.is_ok(), "a slow line was given up: {wrote:?}");
        assert_eq!(*ending.borrow(), None);
    }

    /// **The ordering, asserted rather than described.** The key file named
    /// here does not exist, so if authentication were reached first this would
    /// fail at the key. It fails at the address instead, because nothing is
    /// loaded until key exchange and the host-trust decision have both
    /// happened.
    #[tokio::test]
    async fn no_credential_is_touched_before_the_machine_is_reached() {
        let endpoint = Endpoint {
            host: "127.0.0.1".to_string(),
            port: 0,
            username: "nobody".to_string(),
        };
        let credential = Credential {
            private_key_file: std::path::PathBuf::from("/definitely/not/a/key"),
            passphrase: None,
        };
        let failure = open(
            &endpoint,
            credential,
            K3_AT_ITS_OWN_FIGURES,
            Box::new(Refuses),
        )
        .await
        .expect_err("no machine is listening on port 0");
        assert!(!matches!(failure, OpenFailure::TheKeyCouldNotBeLoaded));
    }
}
