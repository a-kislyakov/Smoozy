//! **`Rented-server runtime` §5.1's setup kind, as a worker mode of this
//! program** — and the two things a setup step needs that only a program can
//! do: a request on the machine's own line, and a process run.
//!
//! **What a setup run is, and what each step comes to, is `Server Setup`'s and
//! is called rather than re-derived.** `smoozy-server-setup-machine`'s `acting`
//! module holds `Server Setup` Y7's machine-side list; this file holds the loop that asks the
//! supervisor what to do, does it, and reports — and the two real ends of that
//! crate's seams.
//!
//! **Neither end of the words below is the desktop**, so they are this
//! program's own and are not in `smoozy_framing::requests`, which names what the
//! two ends of the connection speak. That is the same line the render worker's
//! own words are drawn on.
//!
//! **The worker performs and reports; the supervisor writes.** Nothing here
//! opens a record, a queue, a journal or the setup state.
//!
//! **Every marker of `Server Setup` in this file is written out at its site.**
//! That plan and this one share a letter series, so a header list could not
//! tell the two apart.

use std::io::Read;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use smoozy_framing::framing::{
    Fields, Request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
};
use smoozy_framing::requests as asked;
use smoozy_runtime::acting::endpoint::TheEndpointAsACaller;
use smoozy_server_setup_machine::acting::fetching::{
    MakesTheRequestOnTheMachinesOwnLine, TheBytesLand, WhatTheLineGave,
};
use smoozy_server_setup_machine::acting::steps::{
    HandsOnWhatIsPrinted, HowItWent, NothingIsHandedOn, RunsOneCommandOnTheMachine, TheStepsStream,
};
use smoozy_server_setup_machine::acting::worker::{
    perform_one_step_handing_on, TheStepsInputs, WhatTheStepCameTo, WhereTheRunWorks, WhichStep,
};
use smoozy_server_setup_machine::acting::{NoMoreOftenThan, TellsHowFarItHasGot};
use smoozy_server_setup_machine::blender::WhatTheUserAnswered;
use smoozy_server_setup_machine::run::Boundary;
use smoozy_state::identities::{Identity, IdentityKind};

// ---------------------------------------------------------------------------
// The words the setup worker and the supervisor speak to each other
// ---------------------------------------------------------------------------

/// **The worker's own question: what should this setup worker do now.**
///
/// **It is asked again and again while the run is open**, which is `Server Setup` Y7's *"the
/// worker then waits for the next command"* — the supervisor holds the step the
/// desktop commanded and hands it over when it is asked. **A pure reading**: it
/// writes nothing and journals nothing.
pub const THE_SETUP_WORKERS_OWN_WORK_REQUEST: &str = "what the setup worker should do now";

/// **The worker's report of one step**, which is what lets the supervisor write
/// `Server Setup` Y3's boundary with `Server Setup` Y4a's mark and `Server Setup` Y3's moment.
pub const THE_SETUP_STEP_ENDED_REQUEST: &str = "a setup step ended";

/// **The worker's count of how far the step in hand has got** —
/// `Server Setup` Y7b's bytes written to the staged archive, or
/// `Server Setup` Y7c's bytes fed to the unpacker — with the whole beside it
/// where it is known, in `smoozy_framing`'s own two fields, and the step in
/// [`asked::THE_STEP_FIELD`]. **Sent no more often than `Server Setup` Y7's
/// refresh cadence.** The supervisor holds it beside the step in hand and
/// answers it on the run's state reading; **nothing is written or journalled
/// for it**. Added 2026-10-18, the Creator's fourteenth notes, piece U.
pub const HOW_FAR_A_SETUP_STEP_HAS_GOT_REQUEST: &str = "how far a setup step has got";

/// **What the step in hand printed since the worker last said** —
/// `Server Setup` Y7a item 2: *"the capture streams: the worker passes each
/// stream's bytes to the supervisor as they arrive, apart, in the order the
/// commands ran, the part of a line not yet ended included."* The step in
/// [`asked::THE_STEP_FIELD`], the two pieces in
/// [`asked::THE_STEPS_ORDINARY_WORDS_FIELD`] and
/// [`asked::THE_STEPS_COMPLAINT_WORDS_FIELD`], either of which may be empty.
///
/// **What a step prints is every program it runs, the worker's own progress
/// lines (`Server Setup` Y7b, `Server Setup` Y7c) and, where the step fails with a sentence
/// of Smoozy's own, that sentence** (`Server Setup` S2a's composed text, last,
/// on the error stream). **The supervisor holds the pieces beside the step in
/// hand and never writes them**, and the run's state reading carries what
/// follows the lengths the desktop names. Added 2026-10-25, the eighteenth
/// notes, piece 2b.
pub const WHAT_A_SETUP_STEP_PRINTED_REQUEST: &str = "what a setup step printed";

/// The worker's own process identifier, as a decimal number.
pub const THE_SETUP_WORKERS_OWN_PROCESS_FIELD: &str = "setup worker process";
/// **What the supervisor has for the worker**, from the three words below.
pub const WHAT_THERE_IS_TO_DO_FIELD: &str = "what there is to do";
/// **A step, named in [`asked::THE_STEP_FIELD`]** with its own inputs beside
/// it.
pub const THERE_IS_A_STEP: &str = "a step";
/// **Nothing yet**: the run is open and the desktop has commanded no step. The
/// worker asks again.
pub const THERE_IS_NOTHING_YET: &str = "nothing yet";
/// **Beside [`THERE_IS_NOTHING_YET`]: the run is held for a person**
/// (`Server Setup` B7a) — the user's build answered below 4.5 and the
/// publication waits for *Install anyway* or the way out. Its one value is
/// [`IT_WAS_TAKEN`]'s *yes*. **A worker told it does not count the wait
/// towards [`HOW_LONG_A_WORKER_WAITS_WITH_NOTHING_TO_DO`]**: a person may take
/// longer than half an hour over the question, and the supervisor is plainly
/// alive, since it answered. Added 2026-10-16, step 10c's owed piece 5.
pub const THE_RUN_WAITS_ON_A_PERSON_FIELD: &str = "the run waits on a person";
/// **The run is over**, by any of `Server Setup` Y8's, `Server Setup` Y7's or its own completion. The worker
/// stops.
pub const THE_RUN_IS_OVER: &str = "the run is over";
/// **Where this run's staging area is**, composed by the supervisor from the
/// recorded root.
pub const THE_STAGING_AREA_FIELD: &str = "staging area";
/// **`Server Setup` B9's Blender area**, the same way.
pub const THE_BLENDER_AREA_FIELD: &str = "blender area";
/// **When the setup run started, in whole seconds since 1970 (UTC)**, which
/// `Server Setup` B8 names a published Blender's folder by. Added 2026-10-08.
///
/// **It is the start the run's record holds**, and for a recovery the start it
/// inherited with its taken-over run's boundaries, so the take-over composes
/// the same name. **Absent or unreadable, the worker is handed no start and its
/// publication is refused** rather than named by another time.
pub const THE_RUN_STARTED_FIELD: &str = "run started";
/// **The run whose staged material this step works on**, where it is not the
/// run the worker was started for. Added 2026-09-22.
///
/// **`Server Setup` S4 item 2's recovery inherits the boundary its taken-over
/// run reached, and *fetched* and *tooling* guarantee material at a staging
/// path** (`Server Setup` Y4) — a path named after the run that fetched it
/// (`Server Setup`'s own `fetching::where_the_archive_is_staged`). A recovery
/// resuming past *fetched* that looked under its own name would find nothing,
/// so the supervisor names the run the material sits under and the worker
/// stages, unpacks and publishes under that name. **Absent means the run's own
/// name**, which is every run that is not a recovery.
pub const THE_MATERIAL_IS_STAGED_UNDER_FIELD: &str = "material staged under";
/// **The run this worker was started for**, in `Rented-server runtime` P1's own
/// field.
pub use smoozy_framing::requests::THE_SETUP_RUN_FIELD;
/// **`Server Setup` Y3's boundary the step reached**, where it fed one.
pub use smoozy_framing::requests::THE_BOUNDARY_FIELD;
/// **This run's staged build, once the unpack has found one** — reported so the
/// supervisor can hand it back to the steps that follow.
pub const THE_STAGED_BUILD_FIELD: &str = "staged build";
/// Whether the report was taken: yes, or the reason.
pub const WHETHER_THE_REPORT_WAS_TAKEN_FIELD: &str = "the report was taken";
/// It was.
pub const IT_WAS_TAKEN: &str = "yes";

/// **How long the worker waits between two askings while the run is open.**
///
/// **It is not `Server Setup` Y7's refresh window and is not measured against it.** That window
/// is the *desktop's* refresh gap, held by the supervisor; this is only how
/// often a worker with nothing to do looks again, and a shorter one would spend
/// a rented machine's processor on asking.
pub const BETWEEN_TWO_ASKINGS: Duration = Duration::from_millis(250);

/// **How long one exchange with the supervisor is given.**
pub const HOW_LONG_ONE_EXCHANGE_MAY_TAKE: Duration = Duration::from_secs(30);

/// **How long the worker looks for its supervisor at its first question.**
pub const HOW_LONG_THE_WORKER_LOOKS_FOR_ITS_SUPERVISOR: Duration = Duration::from_secs(30);

/// **How long the worker goes on asking with nothing to do before it gives
/// up.**
///
/// **It is a bound on a wait rather than a rule of the plan.** `Server Setup` Y7's window ends
/// a run whose desktop stopped refreshing, and the supervisor then answers *the
/// run is over*; this exists only so that a worker whose supervisor has died
/// does not sit on a rented machine for ever.
pub const HOW_LONG_A_WORKER_WAITS_WITH_NOTHING_TO_DO: Duration = Duration::from_secs(60 * 30);

// ---------------------------------------------------------------------------
// The two real ends
// ---------------------------------------------------------------------------

/// **The machine's own line, and it is the workspace's one HTTPS client** —
/// the build runner's first ruling for step 10c, taken 2026-09-19.
///
/// **`Server Setup` B4 puts the Blender fetch on the machine's own line**, so without this the
/// setup worker cannot fetch a Blender at all and step 6's milestone cannot be
/// met. **What it costs is written in this piece's own account** rather than
/// left to be discovered at the next tree count.
///
/// # The client is built the way `Server Setup`'s own listing builds it
///
/// **Until 2026-09-21 this called the client's shorthand `ureq::get`**, whose
/// default agent names no cryptography provider — and the workspace pins the
/// client with its provider left out (`rustls-no-provider` in `App/Cargo.toml`)
/// so that nothing chooses one by accident. **So every request this made on a
/// real address stopped at *"No CryptoProvider for Rustls"*** before a byte
/// went out; it was found on the loopback against the real program, on its
/// first request to Blender's downloads. The desk test had used a copy of the
/// client pointed at a plain `http://` server, which never asks for a provider.
///
/// **It is now made by [`the_client_the_fetch_is_made_on`], in the same shape
/// as `App/Server Setup/src/listing.rs`'s `the_client_this_request_is_made_on`**:
/// the provider named at the call and never left to whatever is installed
/// process-wide, no hop off HTTPS, and a status read rather than raised. (Until
/// 2026-10-24 the processor was asked first as well; the provider became
/// `ring`, which asks nothing of any 64-bit Intel or AMD processor — the
/// Creator's eighteenth notes, note 3.) **Why the same shape rather than one shared maker** is
/// written at that function.
pub struct TheMachinesOwnLine {
    client: ureq::Agent,
}

impl TheMachinesOwnLine {
    /// **The line as the program uses it**: Blender's own downloads, trusted
    /// on the client's own published roots and nothing else.
    pub fn onto_blenders_downloads() -> TheMachinesOwnLine {
        TheMachinesOwnLine {
            client: the_client_the_fetch_is_made_on(None),
        }
    }

    /// **The same client, trusting one authority more** — for the desk test,
    /// which serves HTTPS on this computer under a certificate of its own.
    /// **Nothing else about the client differs**, which is the point: the
    /// test's far end asks the same provider for the same handshake.
    #[cfg(test)]
    fn trusting(authority: ureq::tls::Certificate<'static>) -> TheMachinesOwnLine {
        TheMachinesOwnLine {
            client: the_client_the_fetch_is_made_on(Some(ureq::tls::RootCerts::new_with_certs(&[
                authority,
            ]))),
        }
    }
}

/// **How many times Blender's downloads may hand the fetch on to another
/// address: none.**
///
/// **`Server Setup` B4: the fetch *"stays on Blender's own downloads"*.** A
/// redirect is the one way an answer could move the machine's own line off
/// them, and `Server Setup` B4's addresses are derived and never sent, so a hop is not
/// something any clause grants. **Read 2026-09-21**: the checksum file for
/// 5.2.0 answered 200 at its derived address and a version that is not
/// published answered 404, neither with a redirect. **What it costs if that
/// changes**: the fetch fails, saying the far end redirected, rather than
/// following it — which is `Server Setup` B5's *say it and substitute nothing* rather than a
/// quiet move to another host.
pub const HOW_MANY_TIMES_THE_DOWNLOADS_MAY_HAND_THE_FETCH_ON: u32 = 0;

/// **How long the fetch waits to reach Blender's downloads** — the release
/// fetcher's own figure, for the reason written at
/// [`the_client_the_fetch_is_made_on`].
pub const HOW_LONG_THE_FETCH_WAITS_TO_CONNECT: Duration = Duration::from_secs(30);

/// **How long the fetch waits for an answer to begin**, the same way.
pub const HOW_LONG_THE_FETCH_WAITS_FOR_AN_ANSWER_TO_BEGIN: Duration = Duration::from_secs(60);

/// **The client the fetch is made on.**
///
/// **It is the listing's recipe, and it is written a second time here rather
/// than shared, which is a choice with a cost, so the cost is said.** The
/// listing's maker sits in `smoozy-server-setup`, which this program may never
/// reach (it reaches `smoozy-server-protocol`); the one crate both reach and
/// that already owns the cryptography's rules is `smoozy-security-and-licence`,
/// which carries no dependency at all and would have to take the client to
/// hold a maker. **Moving the maker there is a change to four makers in four
/// modules and to that crate's manifest**, and it is recorded as owed in this
/// module's decisions record rather than made from a brief that holds none of
/// them. **What keeps the two from coming apart meanwhile** is that each is one
/// small function whose every setting is named, and the one thing that went
/// wrong — the provider — is now asked for in both the same way and held by a
/// test that makes a real HTTPS request.
///
/// **What differs from the listing, and why.** No whole-request bound: the
/// listing is one 20-kilobyte read, and a Blender archive is hundreds of
/// megabytes on a line nobody here has measured for the whole of one, so a
/// bound would be a guess that ends good fetches. **Instead the two bounds the
/// release fetcher already sets for a large fetch** —
/// `smoozy_server_protocol::release::fetching`'s thirty seconds to connect and
/// sixty for the answer to begin — **so a far end that never answers ends the
/// step with words** rather than holding a setup worker, and the run with it,
/// for ever. They are decided rather than measured, as that fetcher's are, and
/// what they cost if wrong is a step that fails on a line slower to answer than
/// that, saying so. And no redirect at all, for the reason at
/// [`HOW_MANY_TIMES_THE_DOWNLOADS_MAY_HAND_THE_FETCH_ON`].
fn the_client_the_fetch_is_made_on(trusting: Option<ureq::tls::RootCerts>) -> ureq::Agent {
    let mut tls = ureq::tls::TlsConfig::builder()
        .provider(ureq::tls::TlsProvider::Rustls)
        .unversioned_rustls_crypto_provider(std::sync::Arc::new(
            rustls::crypto::ring::default_provider(),
        ));
    if let Some(roots) = trusting {
        tls = tls.root_certs(roots);
    }
    ureq::Agent::config_builder()
        // **No hop may leave HTTPS**, and there are no hops.
        .https_only(true)
        .max_redirects(HOW_MANY_TIMES_THE_DOWNLOADS_MAY_HAND_THE_FETCH_ON)
        // **The status is read rather than raised**: `Server Setup` B5's case —
        // the derived address has no file — is an answer, and it has to reach
        // that judgement whole.
        .http_status_as_error(false)
        .timeout_connect(Some(HOW_LONG_THE_FETCH_WAITS_TO_CONNECT))
        .timeout_recv_response(Some(HOW_LONG_THE_FETCH_WAITS_FOR_AN_ANSWER_TO_BEGIN))
        .tls_config(tls.build())
        .build()
        .new_agent()
}

/// **How much of a body is taken at a time.**
const A_PIECE_OF_THE_BODY: usize = 1 << 16;

impl MakesTheRequestOnTheMachinesOwnLine for TheMachinesOwnLine {
    fn fetch_into(
        &self,
        address: &str,
        from_byte: u64,
        into: &mut dyn TheBytesLand,
    ) -> Result<WhatTheLineGave, String> {
        let mut asking = self.client.get(address);
        if from_byte > 0 {
            // `Server Setup` B4's resume: by byte range into the same staging path.
            asking = asking.header("Range", &format!("bytes={from_byte}-"));
        }
        let answered = asking
            .call()
            .map_err(|why| format!("{address}: {why}"))?;
        let status = answered.status().as_u16();
        let gave = match status {
            206 => WhatTheLineGave::TheRangeAsked,
            200 => WhatTheLineGave::TheWholeFile,
            status => WhatTheLineGave::Neither { status },
        };
        into.the_answer_is(gave)?;
        if let WhatTheLineGave::Neither { .. } = gave {
            return Ok(gave);
        }
        // **`Server Setup` Y7b's whole is the length the answer announced**,
        // and nothing where it announced none.
        if let Some(length) = answered.body().content_length() {
            into.the_body_is(length);
        }
        let mut body = answered.into_body().into_reader();
        let mut held = vec![0u8; A_PIECE_OF_THE_BODY];
        loop {
            let read = body
                .read(&mut held)
                .map_err(|why| format!("{address}: {why}"))?;
            if read == 0 {
                break;
            }
            into.take(&held[..read])?;
        }
        Ok(gave)
    }
}

/// **The machine's own tools, run as child processes.**
///
/// **`Server Setup` B1's time limit is honoured here and what the kill reaches
/// is `Server Setup` B1's rule: the invoked process and nothing above it** — never the setup
/// worker, never a shell around it, never anything found by name, pattern or
/// process search. **There is no shell**: the program and its arguments are
/// handed to the system directly, so nothing composes a command string and
/// nothing is word-split.
///
/// **This closes `Server Setup`'s own
/// `blender::B1S_KILL_HALF_IS_UNKEPT_AND_WHAT_BREAKS_IS_A_BLENDER_LEFT_RUNNING`
/// for every Blender run this worker makes.**
///
/// # The one thing the kill does not reach, said here rather than discovered
///
/// **Where the invoked program leaves a child behind holding its two streams,
/// this returns when that child ends rather than when the limit fires.** A
/// pipe's read ends when every write end is closed, and killing one process
/// does not close a copy another process holds. **The kill itself still lands
/// at the limit**; what waits is the reading of what was printed.
///
/// **Nothing is built to reach that child and that is deliberate.**
/// `Server Setup` B1 names the target as the invoked process and forbids
/// *"anything found by name, pattern or process search"* outright, and a
/// process-group kill is a targeting rule no clause states. **What breaks
/// while it stands**: an invoked build that leaves a child behind holds a setup
/// step open on a machine being paid for until that child ends. **What holds
/// without it**: the one invocation this worker makes under a limit is a
/// staged Blender asked its version, which prints and exits, and nothing this
/// project has observed leaves a child behind. **It is pinned by a test** —
/// `a_child_that_outlives_the_kill_holds_the_streams_and_the_run_waits_for_it`
/// — so it is a recorded fact rather than a surprise on a rental.
pub struct TheMachinesOwnTools;

/// **How often a limited run is looked at while it is waited for.**
const BETWEEN_TWO_LOOKS_AT_A_RUNNING_PROCESS: Duration = Duration::from_millis(50);

impl RunsOneCommandOnTheMachine for TheMachinesOwnTools {
    fn run(
        &self,
        program: &str,
        arguments: &[&str],
        a_time_limit: Option<Duration>,
    ) -> Result<HowItWent, String> {
        self.run_handing_on(program, arguments, a_time_limit, &NothingIsHandedOn)
    }

    fn run_fed(
        &self,
        program: &str,
        arguments: &[&str],
        fed: &mut dyn std::io::Read,
    ) -> Result<HowItWent, String> {
        self.run_fed_handing_on(program, arguments, fed, &NothingIsHandedOn)
    }

    /// **The program run as a child process, each piece of its two streams
    /// handed on the moment it is read** — `Server Setup` Y7a item 2's
    /// *"the capture streams"*, from 2026-10-25 (the eighteenth notes, piece
    /// 2b). Until then each stream was read whole and handed back when the
    /// program ended. **What comes back is the same whole answer.**
    fn run_handing_on(
        &self,
        program: &str,
        arguments: &[&str],
        a_time_limit: Option<Duration>,
        on: &dyn HandsOnWhatIsPrinted,
    ) -> Result<HowItWent, String> {
        use std::process::{Command, Stdio};
        let mut started = Command::new(program)
            .args(arguments)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|why| format!("{program}: {why}"))?;
        let ordinary = started.stdout.take();
        let error = started.stderr.take();
        std::thread::scope(|scope| {
            // **Both streams are read on threads of their own**, because a
            // process that fills a pipe nobody is reading stops, and `Server
            // Setup` B1 captures both streams always.
            let ordinary = ordinary.map(|stream| {
                scope.spawn(move || the_stream_handed_on(stream, TheStepsStream::TheOrdinary, on))
            });
            let error = error.map(|stream| {
                scope.spawn(move || the_stream_handed_on(stream, TheStepsStream::TheErrors, on))
            });
            let mut it_was_killed_at_its_limit = false;
            let exit = match a_time_limit {
                None => started
                    .wait()
                    .map_err(|why| format!("{program}: {why}"))?
                    .code(),
                Some(limit) => {
                    let asked_at = Instant::now();
                    loop {
                        match started.try_wait().map_err(|why| format!("{program}: {why}"))? {
                            Some(status) => break status.code(),
                            None if asked_at.elapsed() >= limit => {
                                // `Server Setup` B1's kill: this process and nothing above it.
                                started.kill().map_err(|why| format!("{program}: {why}"))?;
                                started.wait().map_err(|why| format!("{program}: {why}"))?;
                                it_was_killed_at_its_limit = true;
                                break None;
                            }
                            None => std::thread::sleep(BETWEEN_TWO_LOOKS_AT_A_RUNNING_PROCESS),
                        }
                    }
                }
            };
            Ok(HowItWent {
                exit,
                ordinary: ordinary.map(|held| held.join().unwrap_or_default()).unwrap_or_default(),
                error: error.map(|held| held.join().unwrap_or_default()).unwrap_or_default(),
                it_was_killed_at_its_limit,
            })
        })
    }

    /// **`Server Setup` Y7c: the program run with `fed` on its standard
    /// input**, written in from this thread while its two streams are read on
    /// threads of their own and handed on as they are read, and the input
    /// closed when the feed ends so the program sees the end. Added
    /// 2026-10-18, the Creator's fourteenth notes, piece U; handing on
    /// 2026-10-25, the eighteenth notes, piece 2b.
    ///
    /// **A program that ends before reading the whole** closes its input, and
    /// the write that meets the closed input stops the feed: what the program
    /// printed and how it exited are its own account and are answered as they
    /// are. **A feed that would not read** is said in Smoozy's own words once
    /// the program has ended, because what the program then made of a cut
    /// input is no account of why.
    fn run_fed_handing_on(
        &self,
        program: &str,
        arguments: &[&str],
        fed: &mut dyn std::io::Read,
        on: &dyn HandsOnWhatIsPrinted,
    ) -> Result<HowItWent, String> {
        use std::process::{Command, Stdio};
        let mut started = Command::new(program)
            .args(arguments)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|why| format!("{program}: {why}"))?;
        let ordinary = started.stdout.take();
        let error = started.stderr.take();
        let input = started.stdin.take();
        std::thread::scope(|scope| {
            let ordinary = ordinary.map(|stream| {
                scope.spawn(move || the_stream_handed_on(stream, TheStepsStream::TheOrdinary, on))
            });
            let error = error.map(|stream| {
                scope.spawn(move || the_stream_handed_on(stream, TheStepsStream::TheErrors, on))
            });
            let mut the_feed_would_not_read = None;
            if let Some(mut input) = input {
                let mut held = vec![0u8; A_PIECE_OF_THE_BODY];
                loop {
                    let read = match fed.read(&mut held) {
                        Ok(0) => break,
                        Ok(read) => read,
                        Err(why) if why.kind() == std::io::ErrorKind::Interrupted => continue,
                        Err(why) => {
                            the_feed_would_not_read = Some(why.to_string());
                            break;
                        }
                    };
                    if std::io::Write::write_all(&mut input, &held[..read]).is_err() {
                        // The program closed its input: it ended, and says why.
                        break;
                    }
                }
                // **Dropped here, so the program reads the end.**
            }
            let exit = started
                .wait()
                .map_err(|why| format!("{program}: {why}"))?
                .code();
            let went = HowItWent {
                exit,
                ordinary: ordinary.map(|held| held.join().unwrap_or_default()).unwrap_or_default(),
                error: error.map(|held| held.join().unwrap_or_default()).unwrap_or_default(),
                it_was_killed_at_its_limit: false,
            };
            match the_feed_would_not_read {
                Some(why) => Err(format!("{program} was not handed the whole archive: {why}")),
                None => Ok(went),
            }
        })
    }
}

/// **How much of a stream is read at a time.** A piece is handed on as soon as
/// it is read, so this bounds a piece and never delays one: a read returns
/// whatever the pipe holds.
const A_PIECE_OF_A_STREAM: usize = 1 << 13;

/// **One stream, read to its end a piece at a time, each piece handed on the
/// moment it is read and the whole kept for the answer.**
fn the_stream_handed_on(mut stream: impl Read, which: TheStepsStream, on: &dyn HandsOnWhatIsPrinted) -> String {
    let mut whole = Vec::new();
    let mut piece = vec![0u8; A_PIECE_OF_A_STREAM];
    loop {
        match stream.read(&mut piece) {
            Ok(0) => break,
            Ok(read) => {
                on.printed(which, &piece[..read]);
                whole.extend_from_slice(&piece[..read]);
            }
            Err(why) if why.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(_) => break,
        }
    }
    String::from_utf8_lossy(&whole).to_string()
}

// ---------------------------------------------------------------------------
// W4's per-kind field, for the setup kind
// ---------------------------------------------------------------------------

/// **`Rented-server runtime` W4's per-kind field, for the setup kind** —
/// *"where a setup run's safe-boundary marker lives"*, and `Server Setup` Y3
/// fixes its meaning: *"a named marker in the setup operation record's per-kind
/// field, written durably by the supervisor when the step it names has
/// completed and its verification read has passed, together with the moment on
/// the machine's clock."*
///
/// **It holds two lists and they answer different questions.**
/// `Server Setup` Y4's composed list is *what the run consisted of*, composed
/// at run start;
/// `Server Setup` Y3's reached list is what it managed, **and the last of those is the
/// marker**. `Server Setup`'s own `run::TheSetupRecord` carries the same two for
/// the same reason, and one field holding both was the fault that document's
/// own note records.
///
/// **The failed step's words live here too** (`Server Setup` S2a): *"the setup
/// operation record's per-kind field, beside `Server Setup` Y4's boundary list, written in the
/// same durable act as the ending that names the failed step"* — never the
/// event journal, whose entries are structured at a 512-byte budget, and never
/// the capture sinks, which are per render attempt.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TheSetupKindsOwnField {
    /// `Server Setup` Y6b's kind, in the wire's own words.
    pub kind: String,
    /// `Server Setup` Y4's composed list, in the wire's own words.
    pub composed: Vec<String>,
    /// `Server Setup` Y3's boundaries reached, in order. **The last is the
    /// marker.**
    pub reached: Vec<ABoundaryAsRecorded>,
    /// **The run's recorded ending**, where one was recorded.
    pub ending: Option<String>,
    /// **`Server Setup` S2a's words for the step that failed**, verbatim and
    /// whole, or absent where none were printed.
    pub the_failed_steps_words: Option<String>,
    /// **Which step failed, where the ending is *a step failed*** —
    /// `Server Setup` S2a's *"an ending that names it"*, in the wire's own
    /// [`asked::THE_FAILED_STEP_FIELD`] words: the first boundary that step
    /// reaches. Added 2026-09-21.
    ///
    /// **Until then the ending named no step at all.** A worker's failed step
    /// was cleared from [`Self::the_step_in_flight`] before the ending was
    /// written, and an elevated step's failure was never written, so a reader
    /// of the record could not say which step it was. **A record written before
    /// this field existed reads it as absent**, which is what it was.
    #[serde(default)]
    pub the_failed_step: Option<String>,
    /// **The step that was in flight when the run ended**, which
    /// `Server Setup` S1 reads to tell *a step never started* from *a step that
    /// stopped part-way*.
    pub the_step_in_flight: Option<String>,
    /// **The Blender version this run was commanded to fetch, publish or
    /// unpack**, in the three-part form the wire carries. Added 2026-09-22.
    ///
    /// **Durable because a recovery reads it.** `Server Setup` S4 item 2's
    /// recovery resumes at the inherited boundary and its desktop may be a
    /// press that asked for no Blender — *Set up again* over a Blender install
    /// that stopped at the unpack, which is step 10c's third pass on Vast. The
    /// unpack and the publication read the version, so without it a resumed
    /// run could never finish. **A record written before this field existed
    /// reads it as absent**, which is what it was.
    #[serde(default)]
    pub the_blender_version: Option<String>,
    /// **The run whose staging area holds this run's material**, where that is
    /// not this run — [`THE_MATERIAL_IS_STAGED_UNDER_FIELD`]'s reason. Added
    /// 2026-09-22. Carried from record to record across a chain of recoveries,
    /// so it always names the run that fetched.
    #[serde(default)]
    pub staged_under: Option<String>,
    /// **`Server Setup` B7: the user's own archive this run installs**, its
    /// bare name in the run's staging area — absent on the listing's route.
    /// Added 2026-10-08, the 38's piece 14a; a record written before reads it
    /// as absent.
    #[serde(default)]
    pub the_users_own_archive: Option<String>,
    /// **`Server Setup` B7a: the below-4.5 version the run is holding on**,
    /// while the question stands, in the three-part form.
    #[serde(default)]
    pub the_question_stands_at: Option<String>,
    /// **`Server Setup` B1's answer, as the user's build gave it in staging**
    /// — the version it was published under, or the one it was declined at —
    /// in the three-part form. **The desktop's diary line *a Blender was
    /// installed* names it** (`Server Setup` §12): a build the user pointed at
    /// has no version until it is asked. Added 2026-10-16, step 10c's owed
    /// piece 5; a record written before reads it as absent.
    #[serde(default)]
    pub the_version_answered: Option<String>,
    /// **`Server Setup` B7a: the discard recorded in the publication's place**
    /// with its reason, where the user declined.
    #[serde(default)]
    pub the_discard: Option<String>,
    /// **`Server Setup` B8a: the install record** — the published build's
    /// executable and the route that placed it, and nothing else about it.
    /// **It stands for as long as this operation record does.**
    #[serde(default)]
    pub the_install: Option<AnInstallAsRecorded>,
    /// **`Server Setup` B15's removal, as its recorded outcome** — the
    /// executable whose install folder was deleted and the version it
    /// answered. **It ends the install record that names that executable**
    /// (`Server Setup` B8a), and `Queue & History` L21's mark rides it. Added
    /// 2026-10-10, the 38's piece 14c; a record written before reads it as
    /// absent.
    #[serde(default)]
    pub the_removal: Option<ARemovalAsRecorded>,
}

/// **`Server Setup` B15's removal, as the removal run's own field holds it.**
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ARemovalAsRecorded {
    pub executable: String,
    pub the_answered_version: String,
}

/// **`Server Setup` B8a's install record, as the setup run's own field holds
/// it.** The join is by executable path; the route is one of
/// `smoozy_framing`'s [`asked::FROM_THE_PUBLISHED_LISTING`] and
/// [`asked::YOUR_BUILD`].
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AnInstallAsRecorded {
    pub executable: String,
    pub route: String,
}

/// One boundary as the per-kind field holds it.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ABoundaryAsRecorded {
    pub boundary: String,
    /// `Server Setup` Y4a's mark, from its closed set of two.
    pub what_the_step_did: String,
    /// `Server Setup` Y3's moment, whole seconds since 1970.
    pub at: u64,
    /// The clock it was taken on (`State, identity and recovery` D5).
    pub clock: String,
}

impl TheSetupKindsOwnField {
    pub fn as_written(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("the field is composed of values that always render")
    }

    pub fn read_back(bytes: &[u8]) -> Option<TheSetupKindsOwnField> {
        serde_json::from_slice(bytes).ok()
    }

    /// **`Server Setup` Y3: the marker is the last boundary reached**, and
    /// never the end of the composed list — a composed list always ends at
    /// *done*, so reading its end as the marker would make every run read as
    /// finished whatever happened.
    pub fn the_marker(&self) -> Option<Boundary> {
        self.reached
            .last()
            .and_then(|recorded| the_boundary_named(&recorded.boundary))
    }
}

// ---------------------------------------------------------------------------
// The worker itself
// ---------------------------------------------------------------------------

/// **What the supervisor answered the worker's question with.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatThereIsToDo {
    /// A step, with its own inputs and where the run works.
    AStep {
        step: WhichStep,
        inputs: TheStepsInputs,
        works: WhereTheRunWorks,
        the_marker: Option<Boundary>,
        /// **`Server Setup` Y4's composed list**, what the run consists of —
        /// the supervisor's own held copy, inherited by a recovery with its
        /// marker (`Server Setup` S4 item 2). Added 2026-10-04.
        composed: Vec<Boundary>,
        /// **Where the publication put this run's build**, where it has run —
        /// the supervisor's own held answer (`Server Setup` R1 item 7), which
        /// is what a worker that did not perform the publication itself has no
        /// other way of knowing. Added 2026-09-23.
        the_published_folder: Option<PathBuf>,
        /// **The run whose name the staged material and the published folder
        /// carry** — the run this worker is performing for, or, in a recovery
        /// that inherited material, the run that staged it
        /// ([`THE_MATERIAL_IS_STAGED_UNDER_FIELD`]).
        run: String,
    },
    /// Nothing yet — the worker asks again.
    NothingYet,
    /// **Nothing yet, because the run is held for a person**
    /// ([`THE_RUN_WAITS_ON_A_PERSON_FIELD`]) — the worker asks again, and this
    /// wait never runs out. Added 2026-10-16, step 10c's owed piece 5.
    NothingWhileAPersonIsAsked,
    /// The run is over — the worker stops.
    TheRunIsOver,
}

/// **The supervisor, as the setup worker reaches it.**
///
/// **The worker names itself by its own process identifier and never by the
/// run**, because `Rented-server runtime` M6's one channel is what tells the
/// supervisor which worker is asking, and the invocation that starts a worker
/// carries the worker word and its kind and refuses a third operand. **So the
/// run comes back in the answer** rather than going out in the question.
pub struct TheSupervisorAsTheSetupWorkerReachesIt {
    endpoint: TheEndpointAsACaller,
}

impl TheSupervisorAsTheSetupWorkerReachesIt {
    pub fn at(recorded_endpoint_path: &str) -> Self {
        TheSupervisorAsTheSetupWorkerReachesIt {
            endpoint: TheEndpointAsACaller::at(recorded_endpoint_path, HOW_LONG_ONE_EXCHANGE_MAY_TAKE),
        }
    }

    fn a_request(&self, name: &str, fields: Fields) -> Result<Request, String> {
        let identity = Identity::issue(IdentityKind::Operation)
            .map_err(|_| "smoozy-server: the system would not supply random bytes".to_string())?;
        Ok(
            Request::speaking(THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, identity, name).carrying(
                fields.and(
                    THE_SETUP_WORKERS_OWN_PROCESS_FIELD,
                    std::process::id().to_string(),
                ),
            ),
        )
    }

    /// **The question, asked with this process's own identifier.**
    pub fn what_there_is_to_do(&self, waiting: bool) -> Result<WhatThereIsToDo, String> {
        let request = self.a_request(THE_SETUP_WORKERS_OWN_WORK_REQUEST, Fields::none())?;
        let answered = if waiting {
            self.endpoint
                .ask_waiting_for_a_supervisor(&request, HOW_LONG_THE_WORKER_LOOKS_FOR_ITS_SUPERVISOR)
        } else {
            self.endpoint.ask(&request)
        }
        .map_err(|why| format!("smoozy-server: the setup worker could not ask: {why}"))?
        .ok_or_else(|| {
            "smoozy-server: the setup worker (`Server Setup`'s kind) asked what it should do and \
             no supervisor answered, so it does nothing; its operation record is \
             reconciliation's"
                .to_string()
        })?;
        what_there_is_to_do_in(&answered.fields)
    }

    /// **`Server Setup` Y7b and `Server Setup` Y7c: how far the step in hand has got, told to
    /// the supervisor**, which holds it for the run's state reading. **A count
    /// not taken changes nothing about the step**, so what the supervisor
    /// answered is handed back for the caller to drop.
    pub fn report_how_far(&self, step: WhichStep, bytes: u64, whole: Option<u64>) -> Result<(), String> {
        let request = self.a_request(HOW_FAR_A_SETUP_STEP_HAS_GOT_REQUEST, how_far_as_fields(step, bytes, whole))?;
        match self.endpoint.ask(&request) {
            Ok(Some(answered)) => match answered.fields.text(WHETHER_THE_REPORT_WAS_TAKEN_FIELD) {
                Some(IT_WAS_TAKEN) => Ok(()),
                _ => Err("the supervisor did not take the count".to_string()),
            },
            Ok(None) => Err("no supervisor answered the setup worker's count".to_string()),
            Err(why) => Err(why),
        }
    }

    /// **`Server Setup` Y7a item 2: what the step in hand printed since the
    /// last piece, told to the supervisor**, which holds it beside the step
    /// for the run's state reading. **`Ok(true)` is taken, `Ok(false)` is
    /// refused** — the step is no longer the one in hand — and `Err` is no
    /// answer at all. Added 2026-10-25, the eighteenth notes, piece 2b.
    pub fn report_what_was_printed(&self, step: WhichStep, ordinary: &str, complaints: &str) -> Result<bool, String> {
        let request = self.a_request(
            WHAT_A_SETUP_STEP_PRINTED_REQUEST,
            Fields::none()
                .and(asked::THE_STEP_FIELD, step.as_asked())
                .and(asked::THE_STEPS_ORDINARY_WORDS_FIELD, ordinary)
                .and(asked::THE_STEPS_COMPLAINT_WORDS_FIELD, complaints),
        )?;
        match self.endpoint.ask(&request) {
            Ok(Some(answered)) => Ok(answered.fields.text(WHETHER_THE_REPORT_WAS_TAKEN_FIELD) == Some(IT_WAS_TAKEN)),
            Ok(None) => Err("no supervisor answered the setup worker's words".to_string()),
            Err(why) => Err(why),
        }
    }

    /// **The report of one step, its words already handed on** — `Server
    /// Setup` Y7a item 2, from 2026-10-25: the supervisor holds what the step
    /// printed from the pieces [`TheWordsGoToTheSupervisor`] sent as they
    /// printed, so the report carries the step's own fields and no words. The
    /// supervisor writes the boundary.
    pub fn report_the_step_after_its_words(&self, step: WhichStep, came: &WhatTheStepCameTo) -> Result<(), String> {
        let request = self.a_request(THE_SETUP_STEP_ENDED_REQUEST, the_step_as_reported_after_its_words(step, came))?;
        match self.endpoint.ask(&request) {
            Ok(Some(answered)) => match answered.fields.text(WHETHER_THE_REPORT_WAS_TAKEN_FIELD) {
                Some(IT_WAS_TAKEN) => Ok(()),
                _ => Err(answered
                    .fields
                    .text(asked::THE_MACHINES_OWN_WORDS_FIELD)
                    .unwrap_or("the supervisor did not take the report")
                    .to_string()),
            },
            Ok(None) => Err("no supervisor answered the setup worker's report".to_string()),
            Err(why) => Err(why),
        }
    }
}

/// **`Server Setup` Y7b's and `Server Setup` Y7c's count, as fields** — the step, the bytes
/// so far, and the whole where it is known.
pub fn how_far_as_fields(step: WhichStep, bytes: u64, whole: Option<u64>) -> Fields {
    let fields = Fields::none()
        .and(asked::THE_STEP_FIELD, step.as_asked())
        .and(asked::THE_BYTES_SO_FAR_FIELD, bytes.to_string());
    match whole {
        Some(whole) => fields.and(asked::THE_BYTES_IN_THE_WHOLE_FIELD, whole.to_string()),
        None => fields,
    }
}

/// **The count, on its way to the supervisor** — one step's, sent as it is
/// told. **How often it is told is [`NoMoreOftenThan`]'s**, at
/// `Server Setup` Y7's refresh cadence ([`the_setup_worker`]).
pub struct TheCountGoesToTheSupervisor<'a> {
    pub supervisor: &'a TheSupervisorAsTheSetupWorkerReachesIt,
    pub step: WhichStep,
}

impl TellsHowFarItHasGot for TheCountGoesToTheSupervisor<'_> {
    fn so_far(&self, bytes: u64, whole: Option<u64>) {
        // **A count that did not land is dropped**: the next one is newer, and
        // the step is not the count's to stop.
        let _ = self.supervisor.report_how_far(self.step, bytes, whole);
    }
}

/// **The most of one stream one [`WHAT_A_SETUP_STEP_PRINTED_REQUEST`] carries**,
/// in bytes of text — a quarter of the largest frame, so the two together and
/// the request's own fields always fit. A longer stretch goes in several
/// requests, in order.
pub const THE_MOST_OF_A_STREAM_ONE_PIECE_CARRIES: usize = smoozy_framing::framing::THE_LARGEST_FRAME / 4;

/// **`Server Setup` Y7a item 2: one step's words, on their way to the
/// supervisor as they print.** Added 2026-10-25, the eighteenth notes, piece
/// 2b.
///
/// **Every piece is sent the moment it is handed in**, so the supervisor holds
/// what the step printed up to now and the desktop's next reading carries it.
/// **The two streams stay apart** and each keeps its own order; the one lock
/// is held across a send, so two pieces handed in from the two stream threads
/// leave in the order they were handed in.
///
/// **Words are text on the wire**, so a piece that ends part-way through a
/// character keeps that character's first bytes back until the rest arrives;
/// bytes that are not text at all are read with the replacement mark, the way
/// the whole capture always was. **A piece that does not reach the supervisor
/// is kept and sent with the next**, so nothing is cut out of the middle; one
/// the supervisor refuses — a step no longer in hand — is let go, because
/// nobody will read it. **Nothing about the step waits on its words**: a
/// supervisor that never answers costs the words, never the step.
pub struct TheWordsGoToTheSupervisor<'a> {
    supervisor: &'a TheSupervisorAsTheSetupWorkerReachesIt,
    step: WhichStep,
    held: std::sync::Mutex<TheWordsOnTheirWay>,
}

/// **What has been handed in and has not reached the supervisor yet**: bytes
/// that do not yet end a character, then text not yet sent, one of each per
/// stream.
#[derive(Default)]
struct TheWordsOnTheirWay {
    bytes: [Vec<u8>; 2],
    text: [String; 2],
}

impl<'a> TheWordsGoToTheSupervisor<'a> {
    pub fn for_the_step(supervisor: &'a TheSupervisorAsTheSetupWorkerReachesIt, step: WhichStep) -> Self {
        TheWordsGoToTheSupervisor {
            supervisor,
            step,
            held: std::sync::Mutex::new(TheWordsOnTheirWay::default()),
        }
    }

    /// **Everything still held is sent**, any last bytes that end no character
    /// included — once the step has ended and before its report, so the
    /// supervisor holds the step's last words before it learns the step
    /// ended.
    pub fn everything_left_is_sent(&self) {
        let Ok(mut held) = self.held.lock() else {
            return;
        };
        for at in 0..2 {
            let bytes = std::mem::take(&mut held.bytes[at]);
            held.text[at].push_str(&String::from_utf8_lossy(&bytes));
        }
        self.what_is_held_is_sent(&mut held);
    }

    fn what_is_held_is_sent(&self, held: &mut TheWordsOnTheirWay) {
        while !held.text[0].is_empty() || !held.text[1].is_empty() {
            let ordinary = the_text_up_to(&held.text[0], THE_MOST_OF_A_STREAM_ONE_PIECE_CARRIES);
            let complaints = the_text_up_to(&held.text[1], THE_MOST_OF_A_STREAM_ONE_PIECE_CARRIES);
            let (ordinary_length, complaints_length) = (ordinary.len(), complaints.len());
            match self.supervisor.report_what_was_printed(self.step, ordinary, complaints) {
                // **Taken, or refused for a step no longer in hand**: either
                // way it is not sent again.
                Ok(_) => {
                    held.text[0].drain(..ordinary_length);
                    held.text[1].drain(..complaints_length);
                }
                // **Not reached**: kept for the next piece.
                Err(_) => return,
            }
        }
    }
}

impl HandsOnWhatIsPrinted for TheWordsGoToTheSupervisor<'_> {
    fn printed(&self, stream: TheStepsStream, bytes: &[u8]) {
        if bytes.is_empty() {
            return;
        }
        let Ok(mut held) = self.held.lock() else {
            return;
        };
        let at = match stream {
            TheStepsStream::TheOrdinary => 0,
            TheStepsStream::TheErrors => 1,
        };
        held.bytes[at].extend_from_slice(bytes);
        let ready = the_bytes_that_end_a_character(&held.bytes[at]);
        let text = String::from_utf8_lossy(&held.bytes[at][..ready]).into_owned();
        held.bytes[at].drain(..ready);
        held.text[at].push_str(&text);
        self.what_is_held_is_sent(&mut held);
    }
}

/// **How many of `bytes` end a whole character**: all of them, but for the
/// first bytes of one character still on its way (at most three, the longest
/// a character's first bytes run before it is whole).
fn the_bytes_that_end_a_character(bytes: &[u8]) -> usize {
    for back in 1..=3.min(bytes.len()) {
        let at = bytes.len() - back;
        let byte = bytes[at];
        if byte & 0b1100_0000 == 0b1000_0000 {
            // A continuing byte: look further back for the first.
            continue;
        }
        let its_length = match byte {
            0x00..=0x7F => 1,
            0xC0..=0xDF => 2,
            0xE0..=0xEF => 3,
            0xF0..=0xF7 => 4,
            _ => 1,
        };
        return if its_length > back { at } else { bytes.len() };
    }
    bytes.len()
}

/// **At most `most` bytes of `text`, ending on a character's edge.**
fn the_text_up_to(text: &str, most: usize) -> &str {
    if text.len() <= most {
        return text;
    }
    let mut at = most;
    while !text.is_char_boundary(at) {
        at -= 1;
    }
    &text[..at]
}

/// **Kept for the desk only from 2026-10-25**: a worker of this program hands
/// its step's words on as they print ([`TheWordsGoToTheSupervisor`]) and
/// reports the step without them ([`the_step_as_reported_after_its_words`]).
/// The supervisor still takes a report carrying them whole where no piece
/// came, which is what the tests drive it with.
///
/// **Whether a request is past the largest frame either end reads**, which
/// is where a step's words go back as the mark instead (`Server Setup` Y7a
/// item 5).
#[cfg(test)]
pub fn it_would_pass_the_largest_frame(request: &Request) -> bool {
    smoozy_framing::framing::write_request_frame(request).len() > smoozy_framing::framing::THE_LARGEST_FRAME
}

/// **`Server Setup` Y7a item 2: a step's two streams, each every command's in
/// the order the commands ran**, with nothing put between two commands and
/// nothing taken out — the same way the desktop keeps an admin step's, which
/// is every command of the step on one stream (`Server Setup`'s
/// `elevated::WhatTheRunPrinted`). Added 2026-10-24.
#[cfg(test)]
pub fn the_steps_two_streams(came: &WhatTheStepCameTo) -> (String, String) {
    let mut ordinary = String::new();
    let mut complaints = String::new();
    for one in &came.what_its_commands_printed {
        ordinary.push_str(&one.ordinary);
        complaints.push_str(&one.error);
    }
    (ordinary, complaints)
}

/// **One step's report, as fields** — with `Server Setup` Y7a's two streams.
/// For the desk only, from 2026-10-25 (above).
#[cfg(test)]
pub fn the_step_as_fields(step: WhichStep, came: &WhatTheStepCameTo) -> Fields {
    let (ordinary, complaints) = the_steps_two_streams(came);
    the_step_as_fields_without_its_words(step, came)
        .and(asked::THE_STEPS_ORDINARY_WORDS_FIELD, ordinary)
        .and(asked::THE_STEPS_COMPLAINT_WORDS_FIELD, complaints)
}

/// **One step's report with the mark that its words were not kept in their
/// place** — `Server Setup` Y7a item 5, never a part of either stream. For
/// the desk only, from 2026-10-25 (above).
#[cfg(test)]
pub fn the_step_as_fields_with_its_words_not_kept(step: WhichStep, came: &WhatTheStepCameTo) -> Fields {
    the_step_as_fields_without_its_words(step, came)
        .and(asked::THE_STEPS_WORDS_WERE_NOT_KEPT_FIELD, asked::THE_WORDS_WERE_NOT_KEPT)
}

/// **One step's report where its words went to the supervisor as they
/// printed** (`Server Setup` Y7a item 2, from 2026-10-25): the report's own
/// fields, and no words.
pub fn the_step_as_reported_after_its_words(step: WhichStep, came: &WhatTheStepCameTo) -> Fields {
    the_step_as_fields_without_its_words(step, came)
}

/// **The report's own fields, the words apart.**
fn the_step_as_fields_without_its_words(step: WhichStep, came: &WhatTheStepCameTo) -> Fields {
    let mut fields = Fields::none()
        .and(asked::THE_STEP_FIELD, step.as_asked())
        .and(
            asked::THE_STEP_FINISHED_FIELD,
            if came.it_ran_to_its_end {
                asked::THE_STEP_RAN_TO_ITS_END
            } else if came.held_for_a_person {
                // `Server Setup` B7a: neither ran nor failed. Added 2026-10-08.
                asked::THE_STEP_IS_HELD_FOR_A_PERSON
            } else {
                asked::THE_STEP_DID_NOT
            },
        )
        .and(
            asked::WHAT_THE_STEP_DID_FIELD,
            if came.it_performed {
                asked::IT_PERFORMED
            } else {
                asked::IT_HAD_NOTHING_TO_DO
            },
        );
    if let Some(boundary) = came.the_boundary {
        fields = fields.and(THE_BOUNDARY_FIELD, the_word_for_boundary(boundary));
    }
    if let Some(words) = &came.the_machines_own_words {
        fields = fields.and(asked::THE_MACHINES_OWN_WORDS_FIELD, words.clone());
    }
    if let Some(why) = &came.why_not {
        fields = fields.and(asked::WHY_NO_COMPLETENESS_STATEMENT_FIELD, why.clone());
    }
    fields = fields.and(
        asked::HOW_MANY_LIBRARIES_NOT_FOUND_FIELD,
        came.the_libraries_not_found.len().to_string(),
    );
    for (at, library) in came.the_libraries_not_found.iter().enumerate() {
        fields = fields.and(
            &asked::the_field_of_item(asked::A_LIBRARY_NOT_FOUND, at, asked::THE_LIBRARY_FIELD),
            library.clone(),
        );
    }
    if let Some(answered) = came.the_version_answered {
        fields = fields.and(
            asked::THE_VERSION_ANSWERED_FIELD,
            format!("{}.{}.{}", answered.major, answered.minor, answered.patch),
        );
    }
    if let Some(at) = &came.published_at {
        fields = fields.and(asked::THE_PUBLISHED_AT_FIELD, at.display().to_string());
    }
    if let Some(at) = &came.the_staged_executable {
        fields = fields.and(THE_STAGED_BUILD_FIELD, at.display().to_string());
    }
    if let Some(discard) = &came.the_discard {
        fields = fields.and(asked::THE_DISCARD_FIELD, discard.clone());
    }
    fields
}

/// **`Server Setup` Y4's boundaries, as the wire spells them** — one mapping,
/// held to `smoozy-framing`'s list by a test below.
pub fn the_word_for_boundary(boundary: Boundary) -> &'static str {
    match boundary {
        Boundary::Started => asked::THE_BOUNDARY_STARTED,
        Boundary::Fetched => asked::THE_BOUNDARY_FETCHED,
        Boundary::Tooling => asked::THE_BOUNDARY_TOOLING,
        Boundary::Libraries => asked::THE_BOUNDARY_LIBRARIES,
        Boundary::DriverPlaced => asked::THE_BOUNDARY_DRIVER_PLACED,
        Boundary::RestartIssued => asked::THE_BOUNDARY_RESTART_ISSUED,
        Boundary::CardAnswering => asked::THE_BOUNDARY_CARD_ANSWERING,
        Boundary::Blender => asked::THE_BOUNDARY_BLENDER,
        Boundary::Removed => asked::THE_BOUNDARY_REMOVED,
        Boundary::Telegram => asked::THE_BOUNDARY_TELEGRAM,
        Boundary::Done => asked::THE_BOUNDARY_DONE,
    }
}

/// **The boundary that word names**, and nothing else is read as a boundary.
pub fn the_boundary_named(word: &str) -> Option<Boundary> {
    Boundary::IN_ORDER
        .into_iter()
        .find(|boundary| the_word_for_boundary(*boundary) == word)
}

/// **`Server Setup` Y4's composed list, read off a set of fields** — the
/// desktop's opening of a run and the supervisor's answer to its worker carry
/// it in the same words, and this is the one reader of both.
///
/// **A list that is not there is refused, never read as empty**: an empty list
/// would tell the final check that the run had no Blender in it, which is the
/// direction that reports a check of nothing as a pass (`Server Setup` R4).
pub fn the_composed_list_in(fields: &Fields) -> Result<Vec<Boundary>, String> {
    let how_many: usize = fields
        .text(asked::HOW_MANY_COMPOSED_BOUNDARIES_FIELD)
        .and_then(|text| text.parse().ok())
        .ok_or_else(|| "the run carries no composed boundary list".to_string())?;
    let mut composed = Vec::with_capacity(how_many);
    for at in 0..how_many {
        let named = fields
            .text(&asked::the_field_of_item(
                asked::A_COMPOSED_BOUNDARY,
                at,
                asked::THE_BOUNDARY_FIELD,
            ))
            .ok_or_else(|| format!("the composed list is missing its entry at {at}"))?;
        composed.push(
            the_boundary_named(named)
                .ok_or_else(|| format!("{named} is no boundary this machine knows"))?,
        );
    }
    Ok(composed)
}

/// **`Server Setup` Y4's composed list, carried** in the words
/// [`the_composed_list_in`] reads. Added 2026-10-04: the supervisor hands it to
/// its worker with every step, because the final check's judgement of whether
/// the run published a Blender reads it (`Server Setup — machine`'s
/// `acting::worker::the_run_published_a_blender`).
pub fn the_composed_list_carried(fields: Fields, composed: &[Boundary]) -> Fields {
    let mut fields = fields.and(
        asked::HOW_MANY_COMPOSED_BOUNDARIES_FIELD,
        composed.len().to_string(),
    );
    for (at, boundary) in composed.iter().enumerate() {
        fields = fields.and(
            &asked::the_field_of_item(asked::A_COMPOSED_BOUNDARY, at, asked::THE_BOUNDARY_FIELD),
            the_word_for_boundary(*boundary),
        );
    }
    fields
}

/// **What the supervisor answered, read back.**
pub fn what_there_is_to_do_in(fields: &Fields) -> Result<WhatThereIsToDo, String> {
    match fields.text(WHAT_THERE_IS_TO_DO_FIELD) {
        Some(THERE_IS_NOTHING_YET) if fields.text(THE_RUN_WAITS_ON_A_PERSON_FIELD) == Some(IT_WAS_TAKEN) => {
            Ok(WhatThereIsToDo::NothingWhileAPersonIsAsked)
        }
        Some(THERE_IS_NOTHING_YET) => Ok(WhatThereIsToDo::NothingYet),
        Some(THE_RUN_IS_OVER) => Ok(WhatThereIsToDo::TheRunIsOver),
        Some(THERE_IS_A_STEP) => {
            let step = fields
                .text(asked::THE_STEP_FIELD)
                .and_then(WhichStep::read_back)
                .ok_or_else(|| "the supervisor named no step this worker knows".to_string())?;
            let works = WhereTheRunWorks {
                staging: PathBuf::from(
                    fields
                        .text(THE_STAGING_AREA_FIELD)
                        .ok_or_else(|| "the supervisor named no staging area".to_string())?,
                ),
                the_blender_area: PathBuf::from(
                    fields
                        .text(THE_BLENDER_AREA_FIELD)
                        .ok_or_else(|| "the supervisor named no blender area".to_string())?,
                ),
                the_run_started: fields
                    .text(THE_RUN_STARTED_FIELD)
                    .and_then(|text| text.parse::<u64>().ok())
                    .map(|seconds| std::time::UNIX_EPOCH + std::time::Duration::from_secs(seconds)),
            };
            // **The run's material is named by the run that staged it**, which
            // is this run's own unless it is a recovery that inherited
            // material ([`THE_MATERIAL_IS_STAGED_UNDER_FIELD`]).
            let run = fields
                .text(THE_SETUP_RUN_FIELD)
                .ok_or_else(|| "the supervisor named no run".to_string())?;
            Ok(WhatThereIsToDo::AStep {
                run: fields
                    .text(THE_MATERIAL_IS_STAGED_UNDER_FIELD)
                    .unwrap_or(run)
                    .to_string(),
                step,
                // **The version as asked, read once**: the version, and for an
                // unfinished build the build service's bare file name beside it
                // (`Server Setup` B4, 2026-10-02). A second word that is not a
                // bare name of the one archive form reads as no version at all,
                // and the step says so rather than fetching anything.
                inputs: {
                    let asked = fields
                        .text(asked::THE_BLENDER_VERSION_FIELD)
                        .and_then(smoozy_server_setup_machine::blender::TheVersionAsked::read);
                    TheStepsInputs {
                        version: asked.as_ref().map(|it| it.version),
                        the_build_services_file: asked.and_then(|it| it.the_build_services_file),
                        executable: fields.text(asked::THE_EXECUTABLE_FIELD).map(PathBuf::from),
                        // **`Server Setup` B7 and `Server Setup` B7a, the user's own build**:
                        // the archive's bare name and, after a hold, the
                        // user's answer. Added 2026-10-08, the 38's piece 14a.
                        the_users_own_archive: fields
                            .text(asked::THE_USERS_OWN_ARCHIVE_FIELD)
                            .map(str::to_string),
                        the_users_answer: match fields.text(asked::THE_USERS_ANSWER_FIELD) {
                            Some(asked::INSTALL_ANYWAY) => Some(WhatTheUserAnswered::InstallAnyway),
                            Some(asked::THE_USER_DECLINED) => Some(WhatTheUserAnswered::Decline),
                            _ => None,
                        },
                        // **`Server Setup` B15's second route**: the package
                        // the manager's removal named, forwarded from the
                        // command. Step 10c's owed piece 6.
                        the_managers_package: fields
                            .text(asked::THE_MANAGERS_PACKAGE_FIELD)
                            .map(str::to_string),
                    }
                },
                works,
                the_marker: fields.text(THE_BOUNDARY_FIELD).and_then(the_boundary_named),
                composed: the_composed_list_in(fields)?,
                the_published_folder: fields
                    .text(asked::THE_PUBLISHED_AT_FIELD)
                    .map(PathBuf::from),
            })
        }
        _ => Err("the supervisor answered nothing this worker knows".to_string()),
    }
}

/// **Whether the worker gives up, given what the supervisor last answered and
/// how long it has had nothing to do.** Only a plain *nothing yet* past
/// [`HOW_LONG_A_WORKER_WAITS_WITH_NOTHING_TO_DO`] stops it: a run held for a
/// person (`Server Setup` B7a) never does, because the person may take longer
/// than the bound over the question and the supervisor that said so is alive.
/// The bound exists for a supervisor that died, and one that answers has not.
/// Added 2026-10-16, step 10c's owed piece 5.
pub fn it_stops_for_want_of_work(what: &WhatThereIsToDo, idle_for: Duration) -> bool {
    *what == WhatThereIsToDo::NothingYet && idle_for >= HOW_LONG_A_WORKER_WAITS_WITH_NOTHING_TO_DO
}

/// **The setup worker, whole: ask, perform, report, repeat.**
///
/// **`Server Setup` Y7's shape exactly.** Each step runs to its boundary and the worker then
/// waits for the next command — which here is asking the supervisor again, the
/// supervisor holding whatever the desktop last commanded.
///
/// **The run this worker is for comes back in the supervisor's answer**, not in
/// its invocation: the library that composes a worker's invocation carries the
/// worker word and its kind and refuses a third operand, and
/// `Rented-server runtime` M6's one channel is what tells the supervisor which
/// worker is asking.
pub fn the_setup_worker(recorded_endpoint_path: &str) -> Result<(), String> {
    let supervisor = TheSupervisorAsTheSetupWorkerReachesIt::at(recorded_endpoint_path);
    let line = TheMachinesOwnLine::onto_blenders_downloads();
    let tools = TheMachinesOwnTools;
    let mut the_staged_executable: Option<PathBuf> = None;
    // **The publication's own answer, held across steps.** `Server Setup` R1
    // item 7 renames the staged folder out of staging, so this is the only
    // place the build is after it — and the final verification reads it.
    // **It was held nowhere until 2026-09-23**, which is why step 10c's fifth
    // pass on Vast watched every Blender install fail its last step at a
    // staging path the publication had taken away.
    let mut the_published_folder: Option<PathBuf> = None;
    let mut first = true;
    let mut idle_since = Instant::now();
    loop {
        let what = supervisor.what_there_is_to_do(first)?;
        match what {
            WhatThereIsToDo::TheRunIsOver => return Ok(()),
            WhatThereIsToDo::NothingYet | WhatThereIsToDo::NothingWhileAPersonIsAsked => {
                first = false;
                // **`Server Setup` B7a's question is a person's to answer**, so
                // the time it stands is not time with nothing to do. Added
                // 2026-10-16, step 10c's owed piece 5.
                if what == WhatThereIsToDo::NothingWhileAPersonIsAsked {
                    idle_since = Instant::now();
                }
                if it_stops_for_want_of_work(&what, idle_since.elapsed()) {
                    return Err(format!(
                        "smoozy-server: the setup worker was given nothing to do for {} minutes \
                         and stopped; its operation record is reconciliation's",
                        HOW_LONG_A_WORKER_WAITS_WITH_NOTHING_TO_DO.as_secs() / 60
                    ));
                }
                std::thread::sleep(BETWEEN_TWO_ASKINGS);
            }
            WhatThereIsToDo::AStep {
                step,
                inputs,
                works,
                the_marker,
                composed,
                the_published_folder: the_supervisors_answer,
                run,
            } => {
                first = false;
                idle_since = Instant::now();
                // **The supervisor's answer wins where it has one**, because it
                // is the side that survives a worker restart and a recovery
                // that inherited a run; this worker's own memory carries it
                // between two steps of one run.
                if the_supervisors_answer.is_some() {
                    the_published_folder = the_supervisors_answer;
                }
                // **`Server Setup` Y7b and `Server Setup` Y7c: the fetch's and the unpack's
                // count goes to the supervisor as the step runs, no more often
                // than once a second** (`Server Setup` Y7d, from 2026-10-25;
                // it was the keepalive cadence). Every other step tells
                // nothing.
                let to_the_supervisor = TheCountGoesToTheSupervisor {
                    supervisor: &supervisor,
                    step,
                };
                let every = smoozy_server_setup_machine::driving::THE_REFRESH_CADENCE_WHILE_WORDS_MOVE;
                let told = NoMoreOftenThan::new(every, &to_the_supervisor);
                // **`Server Setup` Y7a item 2: what the step prints goes to the
                // supervisor as it prints** — its programs' two streams, the
                // worker's own progress lines, and a failing step's own
                // sentence. Added 2026-10-25, the eighteenth notes, piece 2b.
                let words = TheWordsGoToTheSupervisor::for_the_step(&supervisor, step);
                let came = perform_one_step_handing_on(
                    step,
                    the_marker,
                    &composed,
                    &inputs,
                    &works,
                    &run,
                    the_staged_executable.as_deref(),
                    the_published_folder.as_deref(),
                    &line,
                    &tools,
                    &told,
                    &words,
                    every,
                );
                // **The step's last words reach the supervisor before its
                // report**, so the reading that shows the step ended already
                // carries them.
                words.everything_left_is_sent();
                if let Some(staged) = &came.the_staged_executable {
                    the_staged_executable = Some(staged.clone());
                }
                if let Some(published) = &came.published_at {
                    the_published_folder = Some(published.clone());
                }
                if let Err(why) = supervisor.report_the_step_after_its_words(step, &came) {
                    eprintln!("smoozy-server: the setup worker's step was not taken: {why}");
                }
            }
        }
    }
}

#[cfg(test)]
mod the_fetch_on_the_machines_own_line {
    // **`Server Setup` B4's fetch, proved over HTTPS against a server on this
    // computer, with the program's own client — and against nothing else.**
    //
    // **Nothing here reaches Blender's downloads and nothing is rented.** The
    // server below is a few lines of this file: it speaks HTTPS under a
    // certificate made for this test alone, holds two files in memory, answers a
    // byte range at 206, answers the whole file at 200, and answers 404 for
    // anything it does not hold. **That is enough to pose every case `Server Setup` B4 names** —
    // a resume that is honoured, a resume that is not, a derived address with no
    // file behind it, and a digest that does not match.
    //
    // # Why this moved here from `tests/`, on 2026-09-21
    //
    // **It stood in `tests/the_fetch_on_the_machines_own_line.rs` and tested a
    // copy of the client**, because a test outside a program cannot reach the
    // program's own types. **The copy was pointed at a plain `http://` server**,
    // and a plain request never asks for a cryptography provider — so the one
    // fault that stopped every real fetch, a client built with no provider, was
    // the one fault the test could not see. **It was found on the loopback
    // against the real program** instead, on its first request to Blender's
    // downloads. **Here the client is [`super::TheMachinesOwnLine`] itself**, and
    // the far end speaks HTTPS, so a client that cannot make an HTTPS request
    // fails here.
    //
    // **The one thing the test changes is what the client trusts**: the
    // authority that signed this server's certificate is added, through
    // `TheMachinesOwnLine::trusting`, and nothing else about the client is
    // different. **The certificate, its key and its authority are fixtures**, in
    // `tests/the fetch's desk certificate/`, made on 2026-09-21 with OpenSSL for
    // `127.0.0.1` and valid for a hundred years. **The key signs nothing but this
    // test's handshakes** and is not a secret of anyone's.
    //
    // **What it cannot pose is Blender's own downloads honouring a byte range**,
    // which is `Server Setup` B4's own observation (`http_code=206`) and is a reading of that
    // service rather than of this code. **A real fetch is the passes' work.**
    //
    //
    // **Every marker here is written out at its site**, as in the rest of this
    // file.

    use std::io::{BufRead, BufReader, Write};
    use std::net::{TcpListener, TcpStream};
    use std::path::PathBuf;
    use std::sync::{mpsc, Arc};

    use smoozy_server_setup_machine::acting::fetching::{
        the_archive_is_fetched, the_archive_is_fetched_telling, the_archives_file_name,
        the_build_is_fetched, the_digest_of, MakesTheRequestOnTheMachinesOwnLine, TheBytesLand, TheFetchDidNot, WhatTheLineGave,
        THE_DOWNLOADS,
    };
    use smoozy_server_setup_machine::blender::{TheVersionAsked, Version, THE_BUILD_SERVICES_DOWNLOADS};
    use smoozy_state::identities::{Identity, IdentityKind};

    use super::TheMachinesOwnLine;
    use super::{asked, Fields, RunsOneCommandOnTheMachine};

    /// The authority this test's server certificate is signed by.
    const THE_AUTHORITY: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/the fetch's desk certificate/the-authority.der"
    ));
    /// The server's certificate, for `127.0.0.1`.
    const THE_SERVER: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/the fetch's desk certificate/the-server.der"
    ));
    /// The server's key.
    const THE_SERVERS_KEY: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/the fetch's desk certificate/the-servers-key.pk8.der"
    ));

    /// **The version every case below fetches.** Nothing turns on which it is: the
    /// address is derived from it and the server answers whatever the derived path
    /// asks for.
    fn a_version() -> Version {
        Version {
            major: 5,
            minor: 2,
            patch: 0,
        }
    }

    /// **The archive's bytes**, which are this test's own and are not an archive of
    /// anything — the fetch never unpacks and never looks inside.
    fn some_bytes() -> Vec<u8> {
        (0u32..200_000).map(|at| (at % 251) as u8).collect()
    }

    /// **The program's own line, trusting this test's authority.**
    fn the_programs_own_line() -> TheMachinesOwnLine {
        TheMachinesOwnLine::trusting(ureq::tls::Certificate::from_der(THE_AUTHORITY).to_owned())
    }

    /// **An HTTPS file server on this computer, and it is the only far end any test
    /// here reaches.**
    struct AFileServerOnThisComputer {
        at: String,
        /// Whether it honours a byte range. **Turned off to pose the other half of
        /// `Server Setup` B4's resume**: a far end that answers the whole file to a range request.
        honours_ranges: bool,
    }

    impl AFileServerOnThisComputer {
        fn holding(archive: Vec<u8>, checksums: String, honours_ranges: bool) -> AFileServerOnThisComputer {
            let listener = TcpListener::bind("127.0.0.1:0").expect("a port on this computer");
            let at = format!("https://{}", listener.local_addr().expect("its address"));
            let config = the_servers_own_config();
            let (ready, started) = mpsc::channel();
            std::thread::spawn(move || {
                ready.send(()).ok();
                for caller in listener.incoming() {
                    let Ok(caller) = caller else { break };
                    let (config, archive, checksums) =
                        (config.clone(), archive.clone(), checksums.clone());
                    // **Each caller on its own thread**, so a handshake that fails
                    // cannot hold the next one up.
                    std::thread::spawn(move || {
                        answer_one(caller, config, &archive, &checksums, honours_ranges)
                    });
                }
            });
            started.recv().expect("the server started");
            AFileServerOnThisComputer { at, honours_ranges }
        }
    }

    /// **The server's half of the handshake, on the same provider the client
    /// names**, so the test adds no cryptography the program does not carry.
    fn the_servers_own_config() -> Arc<rustls::ServerConfig> {
        use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
        Arc::new(
            rustls::ServerConfig::builder_with_provider(Arc::new(rustls::crypto::ring::default_provider()))
                .with_safe_default_protocol_versions()
                .expect("the provider offers the default versions")
                .with_no_client_auth()
                .with_single_cert(
                    vec![CertificateDer::from(THE_SERVER.to_vec())],
                    PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(THE_SERVERS_KEY.to_vec())),
                )
                .expect("the certificate and its key are a pair"),
        )
    }

    /// **One exchange**, read as a request line and headers and answered.
    fn answer_one(
        caller: TcpStream,
        config: Arc<rustls::ServerConfig>,
        archive: &[u8],
        checksums: &str,
        honours_ranges: bool,
    ) {
        let Ok(connection) = rustls::ServerConnection::new(config) else {
            return;
        };
        let mut reading = BufReader::new(rustls::StreamOwned::new(connection, caller));
        let mut first = String::new();
        if reading.read_line(&mut first).is_err() {
            return;
        }
        let path = first.split_whitespace().nth(1).unwrap_or("/").to_string();
        let mut from_byte = 0usize;
        loop {
            let mut line = String::new();
            if reading.read_line(&mut line).is_err() || line.trim().is_empty() {
                break;
            }
            if let Some(range) = line.to_ascii_lowercase().strip_prefix("range: bytes=") {
                from_byte = range.trim().trim_end_matches('-').parse().unwrap_or(0);
            }
        }
        let tls = reading.get_mut();
        let body: Vec<u8> = if path.ends_with(".sha256") {
            checksums.as_bytes().to_vec()
        } else if path.ends_with(".tar.xz") {
            archive.to_vec()
        } else {
            let _ = tls.write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n");
            tls.conn.send_close_notify();
            let _ = tls.flush();
            return;
        };
        let (status, body) = if from_byte > 0 && honours_ranges && from_byte < body.len() {
            ("206 Partial Content", body[from_byte..].to_vec())
        } else {
            ("200 OK", body)
        };
        let _ = tls.write_all(
            format!(
                "HTTP/1.1 {status}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                body.len()
            )
            .as_bytes(),
        );
        let _ = tls.write_all(&body);
        tls.conn.send_close_notify();
        let _ = tls.flush();
    }

    /// **The program's own line, with the address's beginning pointed at the
    /// server above.**
    ///
    /// **Only the address changes**, because [`THE_DOWNLOADS`] is a build constant
    /// and never arrives from anywhere (which is the point of it). **The client
    /// that makes the request is the program's, untouched** — this is not a copy
    /// of it, which is what the test it replaces got wrong.
    struct TheProgramsLinePointedAtThisComputer {
        line: TheMachinesOwnLine,
        server: String,
    }

    impl MakesTheRequestOnTheMachinesOwnLine for TheProgramsLinePointedAtThisComputer {
        fn fetch_into(
            &self,
            address: &str,
            from_byte: u64,
            into: &mut dyn TheBytesLand,
        ) -> Result<WhatTheLineGave, String> {
            let here = address
                .replace(THE_DOWNLOADS, &format!("{}/", self.server))
                .replace(THE_BUILD_SERVICES_DOWNLOADS, &format!("{}/", self.server));
            self.line.fetch_into(&here, from_byte, into)
        }
    }

    fn pointed_at(server: &AFileServerOnThisComputer) -> TheProgramsLinePointedAtThisComputer {
        TheProgramsLinePointedAtThisComputer {
            line: the_programs_own_line(),
            server: server.at.clone(),
        }
    }

    /// **A folder this call owns alone, and nothing else writes into it** — a
    /// fresh identity each, under the build folder's own temporary area, removed
    /// whole when the test ends. **Why not a clock reading** is the test this
    /// replaced's own finding: two tests starting in the same microsecond composed
    /// one path.
    struct ItsOwnFolder {
        at: PathBuf,
    }

    impl ItsOwnFolder {
        fn made(named: &str) -> ItsOwnFolder {
            let at = the_build_folders_own_temporary_area().join(format!(
                "the-fetch-{named}-{}",
                Identity::issue(IdentityKind::Operation)
                    .expect("randomness")
                    .as_text()
            ));
            std::fs::create_dir_all(&at).expect("the folder is made");
            ItsOwnFolder { at }
        }

        /// Where this test's archive is staged, inside its own folder.
        fn a_staging_path(&self) -> PathBuf {
            self.at.join(the_archives_file_name(a_version()))
        }
    }

    impl Drop for ItsOwnFolder {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.at).ok();
        }
    }

    /// **The published checksum of some bytes**, computed with the module's own
    /// digest over a file in a folder of its own.
    fn the_published_checksum(bytes: &[u8]) -> String {
        let its_own = ItsOwnFolder::made("the-bytes-to-digest");
        let at = its_own.at.join("bytes");
        std::fs::write(&at, bytes).expect("the file is written");
        the_digest_of(&at).expect("it reads")
    }

    fn a_checksum_file(digest: &str) -> String {
        format!(
            "{digest}  {}\n0000000000000000000000000000000000000000000000000000000000000000  blender-5.2.0-windows-x64.zip\n",
            the_archives_file_name(a_version())
        )
    }

    // ── The eighteenth notes, piece 2b: a real fetch step's words, on their
    // way to the supervisor while the step runs — 2026-10-25.

    /// **A server on this computer that hands the archive over slowly**,
    /// three quarters of it a piece at a time, **and then holds the rest until
    /// the test lets it go** — so the test can look at what reached the
    /// supervisor while the step is certainly still running. Everything else
    /// is [`answer_one`]'s.
    fn a_slow_server_holding_its_last_quarter(
        archive: Vec<u8>,
        checksums: String,
        let_go: mpsc::Receiver<()>,
    ) -> String {
        let listener = TcpListener::bind("127.0.0.1:0").expect("a port on this computer");
        let at = format!("https://{}", listener.local_addr().expect("its address"));
        let config = the_servers_own_config();
        let let_go = Arc::new(std::sync::Mutex::new(let_go));
        std::thread::spawn(move || {
            for caller in listener.incoming() {
                let Ok(caller) = caller else { break };
                let (config, archive, checksums, let_go) =
                    (config.clone(), archive.clone(), checksums.clone(), let_go.clone());
                std::thread::spawn(move || {
                    let Ok(connection) = rustls::ServerConnection::new(config) else {
                        return;
                    };
                    let mut reading = BufReader::new(rustls::StreamOwned::new(connection, caller));
                    let mut first = String::new();
                    if reading.read_line(&mut first).is_err() {
                        return;
                    }
                    let path = first.split_whitespace().nth(1).unwrap_or("/").to_string();
                    loop {
                        let mut line = String::new();
                        if reading.read_line(&mut line).is_err() || line.trim().is_empty() {
                            break;
                        }
                    }
                    let tls = reading.get_mut();
                    let body = if path.ends_with(".sha256") { checksums.into_bytes() } else { archive };
                    let _ = tls.write_all(
                        format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", body.len())
                            .as_bytes(),
                    );
                    if path.ends_with(".tar.xz") {
                        let three_quarters = body.len() * 3 / 4;
                        for piece in body[..three_quarters].chunks(1 << 16) {
                            let _ = tls.write_all(piece);
                            let _ = tls.flush();
                            std::thread::sleep(std::time::Duration::from_millis(20));
                        }
                        let _ = let_go
                            .lock()
                            .expect("the hold")
                            .recv_timeout(std::time::Duration::from_secs(60));
                        let _ = tls.write_all(&body[three_quarters..]);
                    } else {
                        let _ = tls.write_all(&body);
                    }
                    tls.conn.send_close_notify();
                    let _ = tls.flush();
                });
            }
        });
        at
    }

    /// **A supervisor stand-in on this computer, at an endpoint path of its
    /// own**: it takes every request the worker sends, answers each *taken*,
    /// and hands what it was sent to the test as it arrives.
    fn a_supervisor_stand_in(at: &std::path::Path) -> mpsc::Receiver<(String, String, String)> {
        let listener = std::os::unix::net::UnixListener::bind(at).expect("an endpoint on this computer");
        let (heard, hearing) = mpsc::channel();
        std::thread::spawn(move || {
            for caller in listener.incoming() {
                let Ok(mut caller) = caller else { break };
                let Ok(request) = smoozy_framing::framing::read_one_request(&mut caller) else {
                    continue;
                };
                let text = |name: &str| request.fields.text(name).unwrap_or_default().to_string();
                let _ = heard.send((
                    request.name.clone(),
                    text(asked::THE_STEPS_ORDINARY_WORDS_FIELD),
                    text(asked::THE_STEPS_COMPLAINT_WORDS_FIELD),
                ));
                let answered = smoozy_framing::framing::Answered::to(
                    &request,
                    smoozy_framing::framing::THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
                    "stand-in",
                )
                .carrying(Fields::none().and(super::WHETHER_THE_REPORT_WAS_TAKEN_FIELD, super::IT_WAS_TAKEN));
                let _ = smoozy_framing::framing::write_one_frame(
                    &mut caller,
                    &smoozy_framing::framing::write_result_frame(&answered),
                );
            }
        });
        hearing
    }

    /// **`Server Setup` Y7a item 2 and `Server Setup` Y7b, proved on a real setup step: the
    /// fetch, performed by the worker's own act on the program's own HTTPS
    /// line against a server on this computer, prints its download lines,
    /// and the program's own words-sink carries each to the supervisor while
    /// the step is still running** — the server is holding the archive's last
    /// quarter when the lines are read. Then the step ends, its last line
    /// says how much was downloaded, and nothing went on the error stream.
    /// Before piece 2b the fetch handed nothing on until its report, and then
    /// nothing, since it ran no program — note 5's *"completely silent"*.
    #[test]
    fn eighteenth_a_real_fetch_steps_download_lines_reach_the_supervisor_while_it_runs() {
        use smoozy_server_setup_machine::acting::worker::{perform_one_step_handing_on, TheStepsInputs, WhereTheRunWorks, WhichStep};
        use smoozy_server_setup_machine::run::Boundary;
        let archive: Vec<u8> = (0u32..2_500_000).map(|at| (at % 251) as u8).collect();
        let (let_go, held) = mpsc::channel();
        let server = a_slow_server_holding_its_last_quarter(
            archive.clone(),
            a_checksum_file(&the_published_checksum(&archive)),
            held,
        );
        let line = TheProgramsLinePointedAtThisComputer { line: the_programs_own_line(), server };
        let folder = ItsOwnFolder::made("eighteenth-words");
        let endpoint = std::env::temp_dir().join(format!(
            "smz-{}.sock",
            &Identity::issue(IdentityKind::Operation).expect("randomness").as_text()[..12]
        ));
        let hearing = a_supervisor_stand_in(&endpoint);
        let supervisor = super::TheSupervisorAsTheSetupWorkerReachesIt::at(&endpoint.display().to_string());
        let words = super::TheWordsGoToTheSupervisor::for_the_step(&supervisor, WhichStep::TheFetch);
        let inputs = TheStepsInputs {
            version: Some(a_version()),
            ..Default::default()
        };
        let works = WhereTheRunWorks {
            staging: folder.at.clone(),
            the_blender_area: folder.at.join("blender"),
            the_run_started: None,
        };
        let the_step_ended = std::sync::atomic::AtomicBool::new(false);
        let mut while_it_ran = Vec::new();
        let came = std::thread::scope(|scope| {
            let step = scope.spawn(|| {
                let came = perform_one_step_handing_on(
                    WhichStep::TheFetch,
                    Some(Boundary::Started),
                    &[Boundary::Started, Boundary::Fetched, Boundary::Done],
                    &inputs,
                    &works,
                    "run-a",
                    None,
                    None,
                    &line,
                    &super::TheMachinesOwnTools,
                    &smoozy_server_setup_machine::acting::NobodyIsTold,
                    &words,
                    std::time::Duration::from_millis(50),
                );
                the_step_ended.store(true, std::sync::atomic::Ordering::SeqCst);
                came
            });
            // **Two download lines, heard while the server still holds the
            // last quarter of the archive** — so the step is still running.
            while while_it_ran.len() < 2 {
                let (name, ordinary, _) = hearing
                    .recv_timeout(std::time::Duration::from_secs(30))
                    .expect("no words reached the supervisor while the step ran");
                assert_eq!(name, super::WHAT_A_SETUP_STEP_PRINTED_REQUEST);
                if ordinary.contains(" MB/s\n") {
                    while_it_ran.push(ordinary);
                }
            }
            assert!(
                !the_step_ended.load(std::sync::atomic::Ordering::SeqCst),
                "the lines arrived only after the step had ended"
            );
            let_go.send(()).expect("the server lets the rest go");
            step.join().expect("the step's thread")
        });
        words.everything_left_is_sent();
        assert!(came.it_ran_to_its_end, "{:?}", came.why_not);
        for said in &while_it_ran {
            assert!(said.starts_with("blender-5.2.0-linux-x64.tar.xz: "), "{said:?}");
            assert!(said.contains(" of 2 MB, "), "{said:?}");
        }
        let mut the_rest = String::new();
        let mut errors = String::new();
        while let Ok((_, ordinary, complaints)) = hearing.recv_timeout(std::time::Duration::from_millis(500)) {
            the_rest.push_str(&ordinary);
            errors.push_str(&complaints);
        }
        assert!(
            the_rest.lines().last().is_some_and(|last| last.starts_with("blender-5.2.0-linux-x64.tar.xz: 2 MB downloaded in ")),
            "the fetch did not say once more that it ended: {the_rest:?}"
        );
        assert_eq!(errors, "", "the fetch put words on its error stream");
        std::fs::remove_file(&endpoint).ok();
    }

    /// **The program's own tools hand a process's words on as it prints
    /// them**, each stream apart — a piece printed before the process sleeps
    /// is handed on before it ends — and the answer is still both streams
    /// whole. Piece 2b, 2026-10-25.
    #[test]
    fn eighteenth_the_machines_own_tools_hand_each_piece_on_as_it_is_printed() {
        use smoozy_server_setup_machine::acting::steps::{HandsOnWhatIsPrinted, TheStepsStream};
        struct Heard(std::sync::Mutex<Vec<(std::time::Instant, TheStepsStream, String)>>);
        impl HandsOnWhatIsPrinted for Heard {
            fn printed(&self, stream: TheStepsStream, bytes: &[u8]) {
                self.0.lock().expect("heard").push((
                    std::time::Instant::now(),
                    stream,
                    String::from_utf8_lossy(bytes).into_owned(),
                ));
            }
        }
        let heard = Heard(std::sync::Mutex::new(Vec::new()));
        let went = super::TheMachinesOwnTools
            .run_handing_on(
                "sh",
                &["-c", "printf 'one\\n'; printf 'oops' >&2; sleep 1; printf 'two\\n'"],
                None,
                &heard,
            )
            .expect("sh runs");
        let ended = std::time::Instant::now();
        assert_eq!(went.ordinary, "one\ntwo\n");
        assert_eq!(went.error, "oops");
        let heard = heard.0.into_inner().expect("heard");
        let first = heard
            .iter()
            .find(|(_, stream, said)| *stream == TheStepsStream::TheOrdinary && said.starts_with("one"))
            .expect("the first line was handed on");
        assert!(
            ended.duration_since(first.0) >= std::time::Duration::from_millis(500),
            "the first line was handed on only as the process ended"
        );
        assert!(heard.iter().any(|(_, stream, said)| *stream == TheStepsStream::TheErrors && said == "oops"));
        let ordinary: String = heard
            .iter()
            .filter(|(_, stream, _)| *stream == TheStepsStream::TheOrdinary)
            .map(|(_, _, said)| said.as_str())
            .collect();
        assert_eq!(ordinary, "one\ntwo\n");
    }

    /// **Words are text on the wire**: a character split across two pieces is
    /// held back until it is whole, and never sent as two broken halves.
    #[test]
    fn eighteenth_a_character_split_across_two_pieces_is_held_until_whole() {
        let euro = "€".as_bytes();
        assert_eq!(super::the_bytes_that_end_a_character(b"abc"), 3);
        assert_eq!(super::the_bytes_that_end_a_character(&[b'a', euro[0]]), 1);
        assert_eq!(super::the_bytes_that_end_a_character(&[b'a', euro[0], euro[1]]), 1);
        assert_eq!(super::the_bytes_that_end_a_character(&[b'a', euro[0], euro[1], euro[2]]), 4);
        assert_eq!(super::the_text_up_to("a€b", 2), "a");
        assert_eq!(super::the_text_up_to("a€b", 9), "a€b");
    }

    /// **The fault this file was moved to catch: the program's own client makes an
    /// HTTPS request and reads the answer.**
    ///
    /// **With the client's shorthand in place of the program's maker** — the code
    /// as it stood until 2026-09-21 — **this fails with *"No CryptoProvider for
    /// Rustls"***, which is what the loopback run met on Blender's downloads.
    #[test]
    fn the_programs_own_client_makes_an_https_request() {
        let server = AFileServerOnThisComputer::holding(Vec::new(), "the published sums\n".to_string(), true);
        let (gave, bytes) = the_programs_own_line()
            .fetch_whole(&format!("{}/Blender5.2/blender-5.2.0.sha256", server.at))
            .expect("the client makes an HTTPS request");
        assert_eq!(gave, WhatTheLineGave::TheWholeFile);
        assert_eq!(bytes, b"the published sums\n");
    }

    /// **And it makes no other kind**: a plain address is refused before anything
    /// goes out, so an answer can never move the fetch into clear text.
    #[test]
    fn a_plain_address_is_refused() {
        let why = the_programs_own_line()
            .fetch_whole("http://127.0.0.1:9/Blender5.2/blender-5.2.0.sha256")
            .expect_err("a plain address is not fetched");
        assert!(why.contains("http://127.0.0.1:9"), "the words name the address: {why}");
    }

    /// **`Server Setup` B4, whole, over the program's client and a real far end: the checksum is
    /// fetched first, the archive is staged, and the staged whole is verified
    /// against the published checksum before anything is unpacked.**
    #[test]
    fn the_archive_lands_staged_and_is_verified_against_the_published_checksum() {
        let bytes = some_bytes();
        let server = AFileServerOnThisComputer::holding(
            bytes.clone(),
            a_checksum_file(&the_published_checksum(&bytes)),
            true,
        );
        let its_own = ItsOwnFolder::made("whole");
        let staged_at = its_own.a_staging_path();

        let landed = the_archive_is_fetched(&pointed_at(&server), &staged_at, a_version())
            .expect("it lands");
        assert_eq!(landed.bytes, bytes.len() as u64);
        assert!(!landed.it_resumed);
        // **`Server protocol and transfers` §7.1: the archive is staged and never at a final name.**
        assert_eq!(std::fs::read(&staged_at).expect("it is there"), bytes);
    }

    /// **`Server Setup` B4's resume: by byte range into the same staging path, and
    /// the published checksum is the only verification** — a whole-file checksum
    /// being unable to verify a prefix.
    #[test]
    fn a_part_fetched_archive_resumes_by_byte_range_into_the_same_staging_path() {
        let bytes = some_bytes();
        let server = AFileServerOnThisComputer::holding(
            bytes.clone(),
            a_checksum_file(&the_published_checksum(&bytes)),
            true,
        );
        assert!(server.honours_ranges);
        let its_own = ItsOwnFolder::made("resumed");
        let staged_at = its_own.a_staging_path();
        // A prefix of the same archive, as an earlier fetch would have left.
        std::fs::write(&staged_at, &bytes[..50_000]).expect("the prefix is staged");

        let landed = the_archive_is_fetched(&pointed_at(&server), &staged_at, a_version())
            .expect("it lands");
        assert!(landed.it_resumed, "the far end honoured the range");
        assert_eq!(landed.bytes, bytes.len() as u64);
        assert_eq!(std::fs::read(&staged_at).expect("it is there"), bytes);
    }

    /// **`Server Setup` Y7b over the program's own line: the whole is the length
    /// the answer announced, and a resume counts from what was staged** — the
    /// first count is the staged prefix and the first piece, and the whole is
    /// the archive's, not the range's. The Creator's fourteenth notes, piece U.
    #[test]
    fn piece_u_the_programs_line_announces_the_whole_and_a_resume_counts_what_was_staged() {
        struct Heard(std::cell::RefCell<Vec<(u64, Option<u64>)>>);
        impl smoozy_server_setup_machine::acting::TellsHowFarItHasGot for Heard {
            fn so_far(&self, bytes: u64, whole: Option<u64>) {
                self.0.borrow_mut().push((bytes, whole));
            }
        }
        let bytes = some_bytes();
        let server = AFileServerOnThisComputer::holding(
            bytes.clone(),
            a_checksum_file(&the_published_checksum(&bytes)),
            true,
        );
        let whole = Some(bytes.len() as u64);

        // **A whole fetch**: rising from the first piece to the whole.
        let its_own = ItsOwnFolder::made("counted-whole");
        let heard = Heard(std::cell::RefCell::new(Vec::new()));
        the_archive_is_fetched_telling(&pointed_at(&server), &its_own.a_staging_path(), a_version(), &heard)
            .expect("it lands");
        let counted = heard.0.into_inner();
        assert!(counted.len() >= 2, "{counted:?}");
        assert!(counted.iter().all(|(_, said)| *said == whole), "{counted:?}");
        assert!(counted.windows(2).all(|pair| pair[1].0 > pair[0].0), "{counted:?}");
        assert_eq!(counted.last().map(|it| it.0), whole);

        // **A resume**: from the staged prefix, against the archive's whole.
        let its_own = ItsOwnFolder::made("counted-resumed");
        let staged_at = its_own.a_staging_path();
        std::fs::write(&staged_at, &bytes[..50_000]).expect("the prefix is staged");
        let heard = Heard(std::cell::RefCell::new(Vec::new()));
        let landed = the_archive_is_fetched_telling(&pointed_at(&server), &staged_at, a_version(), &heard)
            .expect("it lands");
        assert!(landed.it_resumed);
        let counted = heard.0.into_inner();
        let (first, said) = counted.first().copied().expect("a count was told");
        assert!(first > 50_000, "the first count did not start from what was staged: {first}");
        assert_eq!(said, whole, "the whole was the range's, not the archive's");
        assert_eq!(counted.last().map(|it| it.0), whole);
    }

    /// **And the other half: a far end that answers the whole file to a range
    /// request starts the staged file over rather than appending.**
    #[test]
    fn a_far_end_that_ignores_the_range_starts_the_staged_file_over() {
        let bytes = some_bytes();
        let server = AFileServerOnThisComputer::holding(
            bytes.clone(),
            a_checksum_file(&the_published_checksum(&bytes)),
            false,
        );
        let its_own = ItsOwnFolder::made("started-over");
        let staged_at = its_own.a_staging_path();
        std::fs::write(&staged_at, &bytes[..50_000]).expect("the prefix is staged");

        let landed = the_archive_is_fetched(&pointed_at(&server), &staged_at, a_version())
            .expect("it lands");
        assert!(!landed.it_resumed, "nothing was resumed");
        assert_eq!(landed.bytes, bytes.len() as u64);
        assert_eq!(std::fs::read(&staged_at).expect("it is there"), bytes);
    }

    /// **`Server protocol and transfers` S24: on a mismatch the staged material is
    /// abandoned and the fetch starts over into fresh staging, never repaired in
    /// place.**
    #[test]
    fn a_digest_that_does_not_match_abandons_the_staged_material() {
        let bytes = some_bytes();
        let server = AFileServerOnThisComputer::holding(
            bytes.clone(),
            // The published checksum of something else entirely.
            a_checksum_file(&the_published_checksum(b"not these bytes")),
            true,
        );
        let its_own = ItsOwnFolder::made("mismatched");
        let staged_at = its_own.a_staging_path();

        let why = the_archive_is_fetched(&pointed_at(&server), &staged_at, a_version())
            .expect_err("it does not land");
        match why {
            TheFetchDidNot::TheDigestDidNotMatch {
                the_staged_material_was_abandoned,
                ..
            } => assert!(the_staged_material_was_abandoned),
            other => panic!("{other:?}"),
        }
        assert!(
            !staged_at.exists(),
            "`Server protocol and transfers` S24: the staged material is abandoned rather than repaired in place"
        );
    }

    /// **`Server Setup` B5's observed case: the list can name a release Smoozy then cannot
    /// fetch.** *"Smoozy says the build could not be fetched and names it, and
    /// never quietly substitutes a different version."* **The status reaches the
    /// judgement as an answer**, which is why the client reads it rather than
    /// raising it.
    #[test]
    fn a_derived_address_with_no_file_behind_it_is_said_and_nothing_is_substituted() {
        let server = AFileServerOnThisComputer::holding(
            Vec::new(),
            a_checksum_file(&the_published_checksum(b"")),
            true,
        );
        let asked = the_programs_own_line()
            .fetch_whole(&format!("{}/nothing-is-here", server.at))
            .expect("the client answers");
        assert_eq!(asked.0, WhatTheLineGave::Neither { status: 404 });
    }

    /// **`Server Setup` B4's second route, whole, over the program's client and a
    /// real far end: an unfinished build is fetched from the build service's
    /// constant base and its bare name, against a checksum that is one bare
    /// digest** — the shape read 2026-10-02 (the Creator's notes 5 and 7 on
    /// 0.1.7). **And a release folder's per-file checksum is not read as one**:
    /// the step says the published checksum could not be read.
    #[test]
    fn an_unfinished_build_lands_from_the_build_service_against_its_bare_digest() {
        let bytes = some_bytes();
        let alpha = "blender-5.3.0-alpha+main.64d181010e0d-linux.x86_64-release.tar.xz";
        let asked = TheVersionAsked::read(&format!("5.3.0 {alpha}")).expect("it reads");
        let server = AFileServerOnThisComputer::holding(bytes.clone(), the_published_checksum(&bytes), true);
        let its_own = ItsOwnFolder::made("from-the-build-service");
        let staged_at = its_own.a_staging_path();
        let landed = the_build_is_fetched(&pointed_at(&server), &staged_at, &asked).expect("it lands");
        assert_eq!(landed.bytes, bytes.len() as u64);
        assert_eq!(std::fs::read(&staged_at).expect("it is there"), bytes);

        let per_file = AFileServerOnThisComputer::holding(
            bytes.clone(),
            format!("{}  {alpha}\n", the_published_checksum(&bytes)),
            true,
        );
        let its_own = ItsOwnFolder::made("a-per-file-checksum");
        let why = the_build_is_fetched(&pointed_at(&per_file), &its_own.a_staging_path(), &asked)
            .expect_err("a per-file line is not a bare digest");
        assert!(matches!(why, TheFetchDidNot::TheChecksumWasNotPublished { .. }), "{why:?}");
    }

    /// **`Server Setup` B4: a name that is not a bare name of the one archive
    /// form is refused before any request** — the machine is never handed an
    /// address, and a base that arrives from outside is what the constant base
    /// forbids.
    #[test]
    fn a_build_service_name_that_is_not_bare_is_refused_before_any_request() {
        struct NothingMayBeAsked(std::cell::Cell<u32>);
        impl MakesTheRequestOnTheMachinesOwnLine for NothingMayBeAsked {
            fn fetch_into(&self, _: &str, _: u64, _: &mut dyn TheBytesLand) -> Result<WhatTheLineGave, String> {
                self.0.set(self.0.get() + 1);
                Err("asked".to_string())
            }
        }
        let line = NothingMayBeAsked(std::cell::Cell::new(0));
        let its_own = ItsOwnFolder::made("refused-before-asking");
        for name in [
            "evil.example/blender-5.3.0-alpha-linux.x86_64-release.tar.xz",
            "../blender-5.3.0-alpha-linux.x86_64-release.tar.xz",
            "blender-5.3.0-alpha+main.64d181010e0d-windows.amd64-release.zip",
        ] {
            let asked = TheVersionAsked {
                version: a_version(),
                the_build_services_file: Some(name.to_string()),
            };
            let why = the_build_is_fetched(&line, &its_own.a_staging_path(), &asked)
                .expect_err("it is refused");
            assert!(matches!(why, TheFetchDidNot::ItWasRefused { .. }), "{name}: {why:?}");
        }
        assert_eq!(line.0.get(), 0, "a request went out");
    }

    /// **The build folder's own temporary area**, for a unit test that has no
    /// `CARGO_TARGET_TMPDIR` — Cargo sets that for the tests under `tests/` only.
    /// **It is found from where this test program was built**, three folders up
    /// from it, which is the build folder whatever it is called. **Nothing here
    /// writes to the system's temporary folder.**
    fn the_build_folders_own_temporary_area() -> PathBuf {
        let this_test = std::env::current_exe().expect("this test knows where it is");
        this_test
            .ancestors()
            .nth(3)
            .expect("a test program sits three folders inside the build folder")
            .join("tmp")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_server_setup_machine::blender::Version;

    /// **`Server Setup` B4's second route reaches the worker from the one
    /// carried field**: the version and the build service's bare file name
    /// beside it, and a second word that is not a bare name of the one archive
    /// form reads as no version at all — the step then says it was commanded
    /// with none, and nothing is fetched. The Creator's notes 5 and 7 on 0.1.7.
    #[test]
    fn the_build_services_file_is_read_off_the_carried_version_and_nothing_else() {
        let alpha = "blender-5.3.0-alpha+main.64d181010e0d-linux.x86_64-release.tar.xz";
        let with = |carried: &str| {
            Fields::none()
                .and(WHAT_THERE_IS_TO_DO_FIELD, THERE_IS_A_STEP)
                .and(asked::THE_STEP_FIELD, asked::THE_FETCH_STEP)
                .and(THE_SETUP_RUN_FIELD, "a-run")
                .and(THE_STAGING_AREA_FIELD, "/root/staging")
                .and(THE_BLENDER_AREA_FIELD, "/root/blender")
                .and(asked::THE_BLENDER_VERSION_FIELD, carried)
                .and(THE_BOUNDARY_FIELD, asked::THE_BOUNDARY_STARTED)
                .and(asked::HOW_MANY_COMPOSED_BOUNDARIES_FIELD, "0")
        };
        let inputs = |carried: &str| match what_there_is_to_do_in(&with(carried)).expect("it reads") {
            WhatThereIsToDo::AStep { inputs, .. } => inputs,
            other => panic!("{other:?}"),
        };
        let read = inputs(&format!("5.3.0 {alpha}"));
        assert_eq!(read.version, Some(Version { major: 5, minor: 3, patch: 0 }));
        assert_eq!(read.the_build_services_file.as_deref(), Some(alpha));
        let read = inputs("5.3.0 https://evil.example/blender-5.3.0-linux.x86_64-release.tar.xz");
        assert_eq!((read.version, read.the_build_services_file), (None, None));
    }

    /// **Every boundary has a word and every word a boundary**, so neither side
    /// can gain one alone.
    #[test]
    fn the_boundaries_and_their_wire_words_are_one_list() {
        let mine: Vec<&str> = Boundary::IN_ORDER
            .iter()
            .map(|it| the_word_for_boundary(*it))
            .collect();
        assert_eq!(mine, asked::EVERY_SETUP_BOUNDARY.to_vec());
        for word in asked::EVERY_SETUP_BOUNDARY {
            assert!(the_boundary_named(word).is_some(), "{word} reads back");
        }
        assert_eq!(the_boundary_named("nearly done"), None);
    }

    /// **The answer reads back as it was composed**, and an answer naming a
    /// step this worker does not know is refused rather than performed.
    #[test]
    fn what_there_is_to_do_reads_back_and_an_unknown_step_is_refused() {
        let fields = Fields::none()
            .and(WHAT_THERE_IS_TO_DO_FIELD, THERE_IS_A_STEP)
            .and(asked::THE_STEP_FIELD, asked::THE_FETCH_STEP)
            .and(THE_SETUP_RUN_FIELD, "a-run")
            .and(THE_STAGING_AREA_FIELD, "/root/staging")
            .and(THE_BLENDER_AREA_FIELD, "/root/blender")
            .and(THE_RUN_STARTED_FIELD, "1791300732")
            .and(asked::THE_BLENDER_VERSION_FIELD, "5.2.0")
            .and(THE_BOUNDARY_FIELD, asked::THE_BOUNDARY_STARTED);
        let a_blender_install = [Boundary::Started, Boundary::Fetched, Boundary::Blender, Boundary::Done];
        // **`Server Setup` Y4's composed list is required**: a step answered
        // without it is refused rather than read as a run with nothing in it.
        assert!(what_there_is_to_do_in(&fields).is_err());
        let fields = the_composed_list_carried(fields, &a_blender_install);
        let read = what_there_is_to_do_in(&fields).expect("it reads");
        assert_eq!(
            read,
            WhatThereIsToDo::AStep {
                run: "a-run".to_string(),
                step: WhichStep::TheFetch,
                inputs: TheStepsInputs {
                    version: Some(Version {
                        major: 5,
                        minor: 2,
                        patch: 0
                    }),
                    the_build_services_file: None,
                    executable: None,
                    the_users_own_archive: None,
                    the_users_answer: None,
                    the_managers_package: None,
                },
                works: WhereTheRunWorks {
                    staging: PathBuf::from("/root/staging"),
                    the_blender_area: PathBuf::from("/root/blender"),
                    the_run_started: Some(
                        std::time::UNIX_EPOCH + std::time::Duration::from_secs(1_791_300_732)
                    ),
                },
                the_marker: Some(Boundary::Started),
                composed: a_blender_install.to_vec(),
                the_published_folder: None,
            }
        );

        // **And the publication's own answer reads back with the step**, which
        // is what carries the build from the publication to the final check
        // across a worker that did not perform the publication itself.
        let published = fields.and(asked::THE_PUBLISHED_AT_FIELD, "/root/blender/blender-5.2.0-20261006-153212");
        let read = what_there_is_to_do_in(&published).expect("it reads");
        let WhatThereIsToDo::AStep {
            the_published_folder,
            ..
        } = read
        else {
            panic!("it is a step");
        };
        assert_eq!(
            the_published_folder,
            Some(PathBuf::from("/root/blender/blender-5.2.0-20261006-153212"))
        );
        let wrong = Fields::none()
            .and(WHAT_THERE_IS_TO_DO_FIELD, THERE_IS_A_STEP)
            .and(asked::THE_STEP_FIELD, "the unpacking")
            .and(THE_SETUP_RUN_FIELD, "a-run")
            .and(THE_STAGING_AREA_FIELD, "/root/staging")
            .and(THE_BLENDER_AREA_FIELD, "/root/blender");
        assert!(what_there_is_to_do_in(&wrong).is_err());
        assert_eq!(
            what_there_is_to_do_in(
                &Fields::none().and(WHAT_THERE_IS_TO_DO_FIELD, THE_RUN_IS_OVER)
            )
            .expect("it reads"),
            WhatThereIsToDo::TheRunIsOver
        );
    }

    /// **A recovery's step works on the material of the run that staged it.**
    /// Added 2026-09-22: named by the run the supervisor says the material sits
    /// under, and by the worker's own run where it says nothing.
    #[test]
    fn a_recoverys_step_works_on_the_material_its_taken_over_run_staged() {
        let fields = Fields::none()
            .and(WHAT_THERE_IS_TO_DO_FIELD, THERE_IS_A_STEP)
            .and(asked::THE_STEP_FIELD, asked::THE_UNPACK_STEP)
            .and(THE_SETUP_RUN_FIELD, "the-recovery")
            .and(THE_MATERIAL_IS_STAGED_UNDER_FIELD, "the-run-that-fetched")
            .and(THE_STAGING_AREA_FIELD, "/root/staging")
            .and(THE_BLENDER_AREA_FIELD, "/root/blender")
            .and(asked::THE_BLENDER_VERSION_FIELD, "5.2.2");
        let fields = the_composed_list_carried(fields, &[Boundary::Started, Boundary::Done]);
        match what_there_is_to_do_in(&fields).expect("it reads") {
            // **No start was sent, so none is handed on**, and the publication
            // will refuse rather than name the folder by another time.
            WhatThereIsToDo::AStep { run, works, .. } => {
                assert_eq!(run, "the-run-that-fetched");
                assert_eq!(works.the_run_started, None);
            }
            other => panic!("{other:?}"),
        }
    }

    /// **A record written before the version and the staging name were kept
    /// reads both as absent**, which is what they were.
    #[test]
    fn a_record_from_before_the_two_new_values_reads_them_as_absent() {
        let older = br#"{"kind":"a blender install","composed":["started","fetched","done"],"reached":[],"ending":null,"the_failed_steps_words":null,"the_step_in_flight":null}"#;
        let read = TheSetupKindsOwnField::read_back(older).expect("an older record reads back");
        assert_eq!(read.the_blender_version, None);
        assert_eq!(read.staged_under, None);
    }

    /// **The report carries the boundary, the mark and the whole not-found
    /// list**, so the supervisor writes `Server Setup` Y4a's mark rather than deriving one.
    #[test]
    fn a_reported_step_carries_its_boundary_its_mark_and_its_list() {
        let came = WhatTheStepCameTo {
            it_ran_to_its_end: true,
            the_boundary: Some(Boundary::Libraries),
            it_performed: true,
            the_machines_own_words: Some("what it printed".to_string()),
            why_not: None,
            // **The whole list a bare container reports, rather than one
            // library.** The fields are indexed, and a list of one could never
            // have shown that the second and the ninth arrive.
            the_libraries_not_found: smoozy_server_setup_machine::ready_action::
                WHAT_A_BARE_CONTAINER_REPORTED
                    .iter()
                    .map(|library| library.to_string())
                    .collect(),
            held_for_a_person: false,
            the_discard: None,
            the_version_answered: None,
            published_at: None,
            the_staged_executable: None,
            what_its_commands_printed: Vec::new(),
        };
        let fields = the_step_as_fields(WhichStep::TheLibraryChainIsReadAgain, &came);
        assert_eq!(
            fields.text(asked::THE_STEP_FIELD),
            Some(asked::THE_LIBRARY_CHAIN_IS_READ_AGAIN_STEP)
        );
        assert_eq!(
            fields.text(THE_BOUNDARY_FIELD),
            Some(asked::THE_BOUNDARY_LIBRARIES)
        );
        assert_eq!(
            fields.text(asked::WHAT_THE_STEP_DID_FIELD),
            Some(asked::IT_PERFORMED)
        );
        assert_eq!(fields.text(asked::HOW_MANY_LIBRARIES_NOT_FOUND_FIELD), Some("9"));
        for (at, library) in smoozy_server_setup_machine::ready_action::
            WHAT_A_BARE_CONTAINER_REPORTED
                .iter()
                .enumerate()
        {
            assert_eq!(
                fields.text(&asked::the_field_of_item(
                    asked::A_LIBRARY_NOT_FOUND,
                    at,
                    asked::THE_LIBRARY_FIELD
                )),
                Some(*library),
                "item {at} of the not-found list did not arrive"
            );
        }
    }

    /// **Both streams come back whole and apart**, which is `Server Setup` B1's
    /// *both streams are captured, always*.
    #[test]
    fn both_streams_come_back_whole_and_apart() {
        let went = TheMachinesOwnTools
            .run("/bin/sh", &["-c", "printf out; printf err >&2"], None)
            .expect("it ran");
        assert_eq!(went.ordinary, "out");
        assert_eq!(went.error, "err");
        assert!(!went.it_was_killed_at_its_limit);
        assert!(went.it_exited_cleanly());
    }

    /// **A run with a time limit is killed at it and the run ends there** —
    /// `Server Setup` B1's *"something outside Blender ready to kill it"*.
    ///
    /// **The program is invoked directly, with no shell anywhere, because that
    /// is the shape this runner actually has**: it takes a program and its
    /// arguments and hands them to the system, and the one thing it ever
    /// invokes under a limit is a staged Blender asked its version.
    ///
    /// **This test used to go through `/bin/sh -c` and it was red on Linux and
    /// green here**, which is what a shell buys: on this desk the shell
    /// replaced itself with the command, so the process spawned **was** the
    /// command and the kill reached it; on the hosted Linux runner it did not,
    /// the kill reached the shell, and what the shell had started went on
    /// holding the streams. **The test had put a second process between the
    /// runner and the thing being timed — a shape the real path never has.**
    /// The limit that exposed is real and is pinned by the test below.
    #[test]
    fn a_run_is_killed_at_its_limit_and_the_run_ends_there() {
        let started = Instant::now();
        let went = TheMachinesOwnTools
            .run("sleep", &["30"], Some(Duration::from_millis(200)))
            .expect("it ran");
        assert!(went.it_was_killed_at_its_limit);
        assert_eq!(went.exit, None, "a killed run reaps no status of its own");
        assert!(
            started.elapsed() < Duration::from_secs(5),
            "the limit is what ended it, and it took {:?}",
            started.elapsed()
        );
    }

    /// **The limit of the kill, pinned by a test rather than met on a rented
    /// machine: a process that outlives the kill and holds the streams keeps
    /// the run from ending at its limit.**
    ///
    /// **This is a recorded limit and not desired behaviour.** A pipe's read
    /// ends when every write end is closed, and killing one process does not
    /// close a copy another process holds — so where the invoked program
    /// leaves a child behind, the run returns when that child ends rather than
    /// when the limit fires. **The kill itself is correct and lands at the
    /// limit**, which is what the first assertion holds.
    ///
    /// **Nothing is built to reach the child, and that is deliberate.**
    /// `Server Setup` B1 names the target as the invoked process and forbids
    /// *"anything found by name, pattern or process search"* outright, and a
    /// process-group kill is a targeting rule no clause states. **What breaks
    /// while it stands**: an invoked build that leaves a child behind holds a
    /// setup step open on a machine being paid for until that child ends.
    /// **What holds without it**: the one invocation this worker makes under a
    /// limit is a staged Blender asked its version, which prints and exits, and
    /// nothing this project has observed leaves a child behind.
    ///
    /// **It is written with a child of its own rather than a shell's
    /// accident**, so it poses the same shape on every system.
    #[test]
    fn a_child_that_outlives_the_kill_holds_the_streams_and_the_run_waits_for_it() {
        let started = Instant::now();
        let went = TheMachinesOwnTools
            .run("/bin/sh", &["-c", "sleep 1 & wait"], Some(Duration::from_millis(200)))
            .expect("it ran");
        assert!(went.it_was_killed_at_its_limit, "the kill lands at the limit");
        assert!(
            started.elapsed() >= Duration::from_millis(600),
            "the run did not end at its limit: the child still held the streams"
        );
        assert!(
            started.elapsed() < Duration::from_secs(20),
            "and it ended when that child did, not never"
        );
    }

    /// **`Server Setup` Y7a item 4, the worker's half — the Creator's ruling
    /// on the sixteenth notes, note 1: nothing secret is shown, because no
    /// program the setup worker runs reads or prints one.** The secret is
    /// kept out by never being emitted, never by a filter, so this holds the
    /// closed set the worker runs against that rule: the unpack tool and every
    /// option it is told, `ldd`, the shell and every loader lookup it is
    /// handed, a build's version switch. **None names the environment, a
    /// process's environment file, the login's keys or the bot token's file**
    /// (`Telegram`'s own name for it). The desktop's half — every command
    /// string a run sends over the command channel — is held in
    /// `Desktop app and delivery`'s `setting_up`.
    #[test]
    fn y7a_no_program_the_setup_worker_runs_reads_or_prints_a_secret() {
        use smoozy_server_setup_machine::acting::steps;
        let mut every: Vec<String> = vec![
            steps::THE_UNPACK_TOOL.to_string(),
            steps::THE_LIBRARY_CHAIN_TOOL.to_string(),
            steps::THE_SHELL.to_string(),
            steps::THE_VERSION_SWITCH.to_string(),
            smoozy_server_setup_machine::the_optix_library::THE_LOADERS_OWN_LOOKUP.to_string(),
        ];
        for mark in [
            &[0xFD, b'7', b'z', b'X', b'Z', 0x00][..],
            &[0x1F, 0x8B][..],
            &b"BZh"[..],
            &[0x28, 0xB5, 0x2F, 0xFD][..],
            &b"LZIP"[..],
            &[0x5D, 0x00, 0x00][..],
        ] {
            every.push(steps::the_unpack_tools_option_for(mark).expect("a wrapping it names").to_string());
        }
        every.extend(["-x", "-f", "-", "-C", "-c"].map(str::to_string));
        for opened in smoozy_server_setup_machine::ready_action::WHAT_BLENDER_OPENS_BY_NAME {
            every.push(opened.the_lookup.to_string());
        }
        for one in &every {
            for word in one.split(|c: char| c.is_whitespace() || ";|&()$\"'`".contains(c)) {
                assert!(word != "env" && word != "printenv", "{one:?} prints the environment");
            }
            for named in ["/environ", "~/.ssh", ".ssh/", smoozy_telegram::material::THE_TOKEN_FILES_NAME] {
                assert!(!one.contains(named), "{one:?} names {named}");
            }
        }
    }
}
