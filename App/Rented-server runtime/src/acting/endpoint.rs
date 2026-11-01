//! The machine-local endpoint, driven.
//!
//! **[`crate::endpoint`] holds the rules** — what a refused bind plus a probe
//! means, what admits a caller, and the race this document establishes no bound
//! on. This module is the bind, the probe, the removal and the kernel read.
//!
//! # What the endpoint is
//!
//! `Security and licence` §8, taken as given: the endpoint is a filesystem
//! object in the machine's own namespace, it **binds no address**, and it is
//! therefore outside the *no inbound listener* question entirely.
//!
//! # The publication behind each act, read 2026-09-04
//!
//! `unix(7)`, `https://man7.org/linux/man-pages/man7/unix.7.html`.
//!
//! - **The object is an ordinary filesystem entry, separate from the open
//!   socket**: *"Binding to a socket with a filename creates a socket in the
//!   filesystem that must be deleted by the caller when it is no longer needed
//!   (using unlink(2))."* That is the standing arrangement Z2b's race exploits,
//!   and it is why Z4's leftover object exists at all.
//! - **The probe's refusal**: *"The remote address specified by connect(2) was
//!   not a listening socket. This error can also occur if the target pathname is
//!   not a socket"*, and, separately, *"The pathname in the remote address
//!   specified to connect(2) did not exist."* **Z12 says outright that the exact
//!   mechanism by which a connection is refused is Build's and that nothing
//!   above rests on which error it arrives as** — so both of those, and nothing
//!   else, are read as *refused*.
//! - **The credentials**: *"This read-only socket option returns the
//!   credentials of the peer process connected to this socket."*, and *"The
//!   returned credentials are those that were in effect at the time of the call
//!   to connect(2), listen(2), or socketpair(2)."* Fact 84 answers the
//!   mechanism on both machine kinds; Z7 is why it cannot be lied to.
//!
//! **The bind's own refusal is read from the standard library's own
//! classification** rather than from an error number, and Z2's correction of
//! 2026-08-20 is why nothing rests on it: `unix(7)` gives the same refusal for a
//! live holder and for a stale object, so the bind's result carries no
//! information about which, and **all of the separating is done by Z12's connect
//! probe**.
//!
//! # What this module does not do, and each is a clause rather than an omission
//!
//! **Nothing removes the object when the run ends.** Z4 is exact: the endpoint
//! ends with the run, and a run that ended without removing it leaves an object
//! Z2's second attempt removes. **A tidy-up on the way out would make Z2b's race
//! worse rather than better** — the object at that path may by then be another
//! start's, and removing a live holder's path is precisely the step the race is
//! made of.
//!
//! **Nothing sweeps, schedules or reads presence.** What Z2 acts on is a
//! refused bind, not an object it found.
//!
//! **Nothing bounds the remove-and-rebind race.** The plan's own Section 16
//! item 19 records that this document establishes no bound on it, that what
//! bounds it is the restarters rather than anything here, and that the two
//! standard arrangements which would remove it are named and refused at Z2b.
//! Neither is taken here.

use std::io::ErrorKind;
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use smoozy_framing::framing::{
    read_one_request, read_one_result, write_one_frame, write_request_frame, write_result_frame,
    Answered, Request, WhyTheFrameCouldNotBeRead,
};

use crate::endpoint::{
    admit, Admission, BindAttempt, ConnectProbe, DrivesTheEndpoint, PeerCredentials,
};

/// The endpoint as the supervisor holds it.
///
/// **The listener is held here** because Z1 makes the endpoint the supervisor's
/// own for the length of its run and nothing about the machine's state is read
/// from it. **An accepted connection is not**: it is handed back to whoever took
/// it and carried through the credential check, the read and the answer as one
/// value, because Z6's boundary is a decision about that one connection.
#[derive(Debug)]
pub struct TheEndpointOnTheMachine {
    /// Z2's recorded path, fixed for the installation and not varying with the
    /// package version.
    pub recorded_path: String,
    bound: Mutex<Option<UnixListener>>,
}

/// How long [`TheEndpointOnTheMachine::take_the_next_caller`] pauses between two
/// askings for a caller.
///
/// **A Build decision and not a plan figure.** It bounds nothing and decides
/// nothing: the deadline is the caller's own and is passed in, and this only
/// keeps the wait from being a spin. Ten milliseconds.
pub const BETWEEN_TWO_ASKINGS_FOR_A_CALLER: Duration = Duration::from_millis(10);

impl TheEndpointOnTheMachine {
    /// At the path the package area recorded at placement.
    pub fn at(recorded_path: &str) -> Self {
        TheEndpointOnTheMachine {
            recorded_path: recorded_path.to_string(),
            bound: Mutex::new(None),
        }
    }

    /// Whether this supervisor is holding the endpoint.
    pub fn is_bound(&self) -> bool {
        self.bound.lock().expect("the endpoint is not poisoned").is_some()
    }

    /// The next caller, or **nothing where none arrived inside the bound**.
    ///
    /// **The two are separated here and nowhere else**, so that a supervisor's
    /// own turn loop can go round again on a quiet machine while an endpoint
    /// that has genuinely stopped working ends the run. The bound is the
    /// caller's own either way (K17) and no number is chosen here.
    ///
    /// **The accept is bounded.** It is taken while the bind is held, so an
    /// unbounded one parks a supervisor nobody calls for ever and holds the bind
    /// with it. The listener is asked without waiting and the pause between two
    /// askings is [`BETWEEN_TWO_ASKINGS_FOR_A_CALLER`], which bounds nothing.
    ///
    /// **The connection is put back to blocking before it is handed over**,
    /// rather than assumed to have been: whether an accepted connection carries
    /// the listener's own mode is a property of the system underneath, and this
    /// crate states it rather than relying on it.
    fn the_next_caller_or_none(&self, within: Duration) -> Result<Option<UnixStream>, String> {
        let held = self.bound.lock().expect("the endpoint is not poisoned");
        let listener = held
            .as_ref()
            .ok_or_else(|| "this supervisor does not hold the endpoint".to_string())?;
        listener
            .set_nonblocking(true)
            .map_err(|why| format!("{}: {why}", self.recorded_path))?;
        let asked_at = Instant::now();
        let taken = loop {
            match listener.accept() {
                Ok((stream, _)) => break Ok(Some(stream)),
                Err(why) if why.kind() == ErrorKind::WouldBlock => {
                    if asked_at.elapsed() >= within {
                        break Ok(None);
                    }
                    std::thread::sleep(BETWEEN_TWO_ASKINGS_FOR_A_CALLER);
                }
                Err(why) => break Err(format!("{}: {why}", self.recorded_path)),
            }
        };
        listener.set_nonblocking(false).ok();
        let Some(stream) = taken? else {
            return Ok(None);
        };
        stream
            .set_nonblocking(false)
            .map_err(|why| format!("{}: {why}", self.recorded_path))?;
        Ok(Some(stream))
    }

    /// **The supervisor's own side of one framed exchange: take the next caller,
    /// admit it, read one request, answer it, and let it go.**
    ///
    /// **Z6 first and the frame second.** The caller's login comes from the
    /// accepted connection, `admit` decides, and **Z9 puts the refusal before the
    /// request is framed** — a refused caller is let go without a byte being read
    /// off it or written to it, so it learns nothing about the protocol.
    ///
    /// **One request and one result, which is `Server protocol and transfers` P1
    /// at this end**: the connection is read once and written once, and it is let
    /// go afterwards. A caller that wants a second request opens a second
    /// connection.
    ///
    /// **What the request is and what answers it are not decided here.** The
    /// answering function is handed in, because the request set belongs to
    /// whichever document defines each request — G12's health answer is this
    /// document's, and the readiness the entry point carries is
    /// `Server protocol and transfers` §1.1's.
    ///
    /// **A caller that connects and sends nothing does not hold the supervisor**:
    /// `how_long_a_request_may_take` bounds the read, and a caller that outlasts
    /// it is let go the same way a refused one is.
    ///
    /// **The connection is one value from the accept to the answer.** It is
    /// never parked where a second accept could reach it, so the request that is
    /// read is always the request of the connection whose credentials were
    /// checked — which is the boundary Z6 exists to hold.
    ///
    /// **Both waits are the caller's own** (K17): how long a caller is waited
    /// for, and how long its request may take.
    pub fn answer_one_caller(
        &self,
        the_login_the_supervisor_runs_as: u32,
        how_long_a_caller_is_waited_for: Duration,
        how_long_a_request_may_take: Duration,
        answer: impl FnOnce(&Request) -> Answered,
    ) -> Result<WhatHappenedToTheCaller, String> {
        self.answer_one_caller_knowing_its_process(
            the_login_the_supervisor_runs_as,
            how_long_a_caller_is_waited_for,
            how_long_a_request_may_take,
            |request, _| answer(request),
        )
    }

    /// **The same exchange, with the caller's process handed to the answer
    /// beside its request.** Added 2026-10-16, for §5.3's paragraph after W7:
    /// the role worker's own requests are answered only from the role worker
    /// this run started, and *"the process the endpoint reads the caller from,
    /// matched against the group this run recorded"* is what singles it out.
    ///
    /// **The process is the one Z6 already read for the admission**, from the
    /// same connection, so nothing about the caller is read twice and nothing
    /// is looked up by it (Z10). What the answer does with it is the answer's.
    pub fn answer_one_caller_knowing_its_process(
        &self,
        the_login_the_supervisor_runs_as: u32,
        how_long_a_caller_is_waited_for: Duration,
        how_long_a_request_may_take: Duration,
        answer: impl FnOnce(&Request, u32) -> Answered,
    ) -> Result<WhatHappenedToTheCaller, String> {
        let Some(mut caller) = self.the_next_caller_or_none(how_long_a_caller_is_waited_for)?
        else {
            return Ok(WhatHappenedToTheCaller::NoCallerArrivedInsideTheDeadline);
        };
        let credentials = match self.peer_credentials(&caller) {
            Ok(credentials) => credentials,
            Err(why) => {
                let_the_refused_caller_go(caller);
                return Ok(WhatHappenedToTheCaller::ItsCredentialsCouldNotBeRead(why));
            }
        };
        if admit(credentials, the_login_the_supervisor_runs_as)
            == Admission::RefusedBeforeTheRequestIsFramed
        {
            let_the_refused_caller_go(caller);
            return Ok(WhatHappenedToTheCaller::RefusedBeforeTheRequestIsFramed);
        }
        caller
            .set_read_timeout(Some(how_long_a_request_may_take))
            .map_err(|why| format!("{}: {why}", self.recorded_path))?;
        caller
            .set_write_timeout(Some(how_long_a_request_may_take))
            .map_err(|why| format!("{}: {why}", self.recorded_path))?;
        let what_happened = match read_one_request(&mut caller) {
            Err(why) => WhatHappenedToTheCaller::ItSentNoReadableFrame(why.as_text()),
            Ok(request) => {
                let answered = answer(&request, credentials.process);
                match write_one_frame(&mut caller, &write_result_frame(&answered)) {
                    Ok(()) => WhatHappenedToTheCaller::ItWasAnswered(request),
                    Err(why) => WhatHappenedToTheCaller::ItsResultCouldNotBeWritten(why.as_text()),
                }
            }
        };
        let_the_refused_caller_go(caller);
        Ok(what_happened)
    }
}

/// **Z9.** A refused caller receives nothing that describes the protocol.
///
/// The connection is closed and not one byte is written on it, which is the
/// whole of what *before the request is framed* means at this end. **It takes
/// the connection by value** because letting a caller go is the end of that
/// connection and of nothing else.
pub fn let_the_refused_caller_go(caller: UnixStream) {
    drop(caller);
}

/// What one turn of [`TheEndpointOnTheMachine::answer_one_caller`] did.
///
/// **None of these is an error of the supervisor's.** A caller of another login,
/// a caller that sent nothing, and a caller that went away before its result
/// could be written are all things the supervisor observed rather than things
/// that went wrong with it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatHappenedToTheCaller {
    /// The request that was read and answered.
    ItWasAnswered(Request),
    /// **Z9.** The caller's login was not the supervisor's own, and it received
    /// nothing that describes the protocol.
    RefusedBeforeTheRequestIsFramed,
    /// The caller was admitted and sent no frame this side could read.
    ItSentNoReadableFrame(String),
    /// The request was read and answered and the result could not be put on the
    /// connection. **The answer was composed; what failed is the writing.**
    ItsResultCouldNotBeWritten(String),
    /// **No caller arrived inside the deadline this turn was given.**
    ///
    /// **It is an outcome rather than a failure**, and it is here so that a
    /// supervisor's own turn loop can tell an idle machine from an endpoint that
    /// has stopped working. A turn that could not tell them apart would either
    /// end the run on every quiet stretch or go round for ever over a real
    /// refusal.
    NoCallerArrivedInsideTheDeadline,
    /// **Z6's reading could not be taken, because the caller was gone by the time
    /// it was asked for.**
    ///
    /// **It is an outcome and not a failure of the supervisor's**, for the same
    /// reason the three above it are: a caller that connects and goes away is a
    /// thing the supervisor observed. **Z10 is why it is ordinary rather than
    /// rare** — the recorded identifier the kernel reports *"is a stale handle the
    /// instant the caller finishes"* — and Z12's own probe is exactly such a
    /// caller, connecting to learn whether anything holds the endpoint and
    /// sending nothing.
    ///
    /// **Nothing is admitted on it.** The caller is let go without a byte being
    /// read or written, which is what a refusal is; what differs from
    /// [`Self::RefusedBeforeTheRequestIsFramed`] is only that no identity was
    /// established to refuse it on.
    ItsCredentialsCouldNotBeRead(String),
}

/// **The endpoint as a caller of it, which is the other half of §4 and the one
/// the machine's own three callers use.**
///
/// **`Server protocol and transfers` P1's unit is one connection carrying one
/// framed request and one correlated result**, and that is what [`Self::ask`]
/// does: it opens, writes one frame, reads one, and lets the connection go.
///
/// **Nothing here is Z12's probe and nothing here reads presence.** The probe
/// asks whether anything holds the endpoint, which is a reading of the
/// connection; this asks a question and waits for an answer, which is what V4
/// says its observed value is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheEndpointAsACaller {
    /// Z2's recorded path. **It does not vary with the package version and
    /// outlives any one supervisor run, though the object at it does not** (Z1),
    /// which is the property W12's route rests on.
    pub recorded_path: String,
    /// How long a written request may take to be answered before the far end
    /// counts as not having answered.
    ///
    /// **It is carried rather than defaulted**, because nothing in this crate
    /// takes a value nobody chose.
    pub how_long_a_result_may_take: Duration,
}

impl TheEndpointAsACaller {
    pub fn at(recorded_path: &str, how_long_a_result_may_take: Duration) -> Self {
        TheEndpointAsACaller {
            recorded_path: recorded_path.to_string(),
            how_long_a_result_may_take,
        }
    }

    /// **One connection, one framed request, one correlated result.**
    ///
    /// **`Ok(None)` is the far end not answering, and it is one answer with three
    /// ways in**: nothing holds the endpoint at the recorded path; the connection
    /// was made and the result did not arrive inside
    /// [`Self::how_long_a_result_may_take`]; or something arrived that this
    /// request could not be correlated with. **The third is
    /// `Server protocol and transfers` C2** — a result is matched to a request by
    /// its identity and never by arrival order, never by channel and never by the
    /// invocation that carried it — so a frame carrying another identity is not
    /// this request's answer and is not read as one.
    ///
    /// **`Err` is kept for a refusal that is not silence**: the system would not
    /// let this process reach the path at all.
    pub fn ask(&self, request: &Request) -> Result<Option<Answered>, String> {
        let mut stream = match UnixStream::connect(&self.recorded_path) {
            Ok(stream) => stream,
            // Nothing holds it. That is the far end not answering, and it is not
            // a reading of presence: no object was looked for and none was read.
            Err(why)
                if why.kind() == ErrorKind::ConnectionRefused
                    || why.kind() == ErrorKind::NotFound =>
            {
                return Ok(None)
            }
            Err(why) => return Err(format!("{}: {why}", self.recorded_path)),
        };
        stream
            .set_read_timeout(Some(self.how_long_a_result_may_take))
            .map_err(|why| format!("{}: {why}", self.recorded_path))?;
        stream
            .set_write_timeout(Some(self.how_long_a_result_may_take))
            .map_err(|why| format!("{}: {why}", self.recorded_path))?;
        if let Err(why) = write_one_frame(&mut stream, &write_request_frame(request)) {
            return match why {
                WhyTheFrameCouldNotBeRead::TheSystemRefusedIt(_) => Ok(None),
                other => Err(other.as_text()),
            };
        }
        match read_one_result(&mut stream) {
            Ok(answered) if answered.answers(request) => Ok(Some(answered)),
            // A frame that correlates with nothing is not this request's answer.
            Ok(_) => Ok(None),
            Err(WhyTheFrameCouldNotBeRead::ItEndedPartWayThrough) => Ok(None),
            Err(WhyTheFrameCouldNotBeRead::TheSystemRefusedIt(_)) => Ok(None),
            Err(other) => Err(other.as_text()),
        }
    }

    /// **G25's own deadline: the same one exchange, waiting up to `until` for a
    /// supervisor to be holding the endpoint at all.**
    ///
    /// G25 orders the running supervisor ended, the restarter asked, and only
    /// then the exchange attempted — and **where the restarter produces no
    /// answering supervisor inside the caller's own deadline, that is G13's *did
    /// not answer* branch and nothing here softens it.** So a connection nothing
    /// accepts is tried again until the deadline and then answered `Ok(None)`,
    /// which is exactly that branch.
    pub fn ask_waiting_for_a_supervisor(
        &self,
        request: &Request,
        until: Duration,
    ) -> Result<Option<Answered>, String> {
        let started = Instant::now();
        loop {
            match self.ask(request)? {
                Some(answered) => return Ok(Some(answered)),
                None => {
                    if started.elapsed() >= until {
                        return Ok(None);
                    }
                    std::thread::sleep(BETWEEN_TWO_ATTEMPTS_TO_REACH_A_SUPERVISOR);
                }
            }
        }
    }
}

/// How long [`TheEndpointAsACaller::ask_waiting_for_a_supervisor`] pauses between
/// two attempts.
///
/// **A Build decision and not a plan figure.** It bounds nothing and decides
/// nothing: G25's deadline is the caller's and is passed in, and this only keeps
/// the wait from being a spin. Fifty milliseconds.
pub const BETWEEN_TWO_ATTEMPTS_TO_REACH_A_SUPERVISOR: Duration = Duration::from_millis(50);

impl DrivesTheEndpoint for TheEndpointOnTheMachine {
    type AcceptedCaller = UnixStream;

    /// **Z2's bind**, at the recorded path.
    ///
    /// Z3 is performed with it: the object carries the ownership and permission
    /// of the login the supervisor runs as, and permits that login alone.
    fn bind_at(&self, path: &str) -> Result<BindAttempt, String> {
        match UnixListener::bind(path) {
            Ok(listener) => {
                crate::acting::durable::permit_that_login_alone(std::path::Path::new(path))
                    .map_err(|why| why.as_text())?;
                *self.bound.lock().expect("the endpoint is not poisoned") = Some(listener);
                Ok(BindAttempt::Bound)
            }
            Err(why) if why.kind() == ErrorKind::AddrInUse => {
                Ok(BindAttempt::RefusedTheAddressIsInUse)
            }
            Err(why) => Err(format!("{path}: {why}")),
        }
    }

    /// **Z12's probe**, as an ordinary caller.
    ///
    /// The connection, where one is made, is dropped at once: the probe asks
    /// whether anything holds the endpoint and sends nothing.
    fn probe_as_a_caller(&self, path: &str) -> Result<ConnectProbe, String> {
        match UnixStream::connect(path) {
            Ok(_) => Ok(ConnectProbe::Accepted),
            Err(why)
                if why.kind() == ErrorKind::ConnectionRefused
                    || why.kind() == ErrorKind::NotFound =>
            {
                Ok(ConnectProbe::Refused)
            }
            // Anything else is a probe that could not be taken, and
            // `crate::endpoint::what_the_start_does` reads that in the safe
            // direction — the one that removes nothing.
            Err(why) => Err(format!("{path}: {why}")),
        }
    }

    /// **Z2's second branch.** The object at this supervisor's own recorded
    /// path, and nowhere else.
    fn remove_its_own_object(&self) -> Result<(), String> {
        match std::fs::remove_file(&self.recorded_path) {
            Ok(()) => Ok(()),
            Err(why) if why.kind() == ErrorKind::NotFound => Ok(()),
            Err(why) => Err(format!("{}: {why}", self.recorded_path)),
        }
    }

    /// **Take the next caller, inside the caller's own deadline, and hand the
    /// connection back.**
    ///
    /// **The credentials are read from the connection and never from anything
    /// the caller sends**, which is Z8: no request field names its caller, and
    /// any such field is ignored rather than checked. Handing the connection
    /// back is what makes that a fact about *this* connection.
    ///
    /// **The accept is bounded.** It is taken while the bind is held, so an
    /// unbounded one parks a supervisor nobody calls for ever and holds the bind
    /// with it. The listener is asked without waiting and the pause between two
    /// askings is [`BETWEEN_TWO_ASKINGS_FOR_A_CALLER`], which bounds nothing —
    /// the deadline is the argument.
    ///
    /// **The connection is put back to blocking before it is handed over**,
    /// rather than assumed to have been: whether an accepted connection carries
    /// the listener's own mode is a property of the system underneath, and this
    /// crate states it rather than relying on it.
    fn take_the_next_caller(&self, within: Duration) -> Result<UnixStream, String> {
        self.the_next_caller_or_none(within)?.ok_or_else(|| {
            format!(
                "{}: no caller arrived inside {within:?}",
                self.recorded_path
            )
        })
    }

    /// **Z6.** The credentials of the accepted connection, read from the kernel.
    ///
    /// **Where the system does not report the caller's identifier, that is a
    /// refusal rather than a zero.** Z10 already says nothing here reads
    /// anything further about a caller by its identifier, so the number is
    /// carried and never used to look anything up — but carrying a number the
    /// system did not give would be a value Smoozy made up about its own
    /// caller, and this crate makes none of those.
    fn peer_credentials(&self, caller: &UnixStream) -> Result<PeerCredentials, String> {
        the_kernels_answer_about(caller)
    }
}

/// **The caller's own login, from the kernel, and it is what Z6 turns on.**
///
/// Z6 refuses any caller whose identity is not the login the supervisor runs
/// as, and that comparison is of the user and of nothing else. This is
/// separated from [`the_kernels_answer_about`] because **the identifier and the
/// login come from one option on the machine Smoozy rents and from two
/// different ones at the desk**, and the half every rule rests on is available
/// on both.
pub fn the_callers_login(stream: &UnixStream) -> Result<(u32, u32), String> {
    #[cfg(target_os = "linux")]
    {
        the_whole_of_what_linux_answers(stream).map(|(_, user, group)| (user, group))
    }
    #[cfg(not(target_os = "linux"))]
    {
        // `getpeereid(3)`, read on the Creator's own Mac with `man 3
        // getpeereid` on 2026-09-04, macOS 26.6.2 build 25G83: *"The
        // getpeereid() function returns the effective user and group IDs of the
        // peer connected to a UNIX-domain socket."*, and *"This mechanism is
        // reliable; there is no way for either side to influence the
        // credentials returned to its peer except by calling the appropriate
        // system call ... under different effective credentials."* — which is
        // Z7's own property stated by a second system.
        use std::os::fd::AsRawFd;
        let mut user: libc::uid_t = 0;
        let mut group: libc::gid_t = 0;
        let answered = unsafe { libc::getpeereid(stream.as_raw_fd(), &mut user, &mut group) };
        if answered != 0 {
            return Err(format!(
                "the kernel refused the caller's credentials: {}",
                std::io::Error::last_os_error()
            ));
        }
        Ok((user, group))
    }
}

#[cfg(target_os = "linux")]
fn the_whole_of_what_linux_answers(stream: &UnixStream) -> Result<(u32, u32, u32), String> {
    // `unix(7)`'s own option, quoted at the head of this file. The structure is
    // the kernel's and carries the identifier beside the login, so the machine
    // Smoozy rents answers both in one call.
    use std::os::fd::AsRawFd;
    let mut answered: libc::ucred = unsafe { std::mem::zeroed() };
    let mut room = std::mem::size_of::<libc::ucred>() as libc::socklen_t;
    let got = unsafe {
        libc::getsockopt(
            stream.as_raw_fd(),
            libc::SOL_SOCKET,
            libc::SO_PEERCRED,
            &mut answered as *mut libc::ucred as *mut libc::c_void,
            &mut room,
        )
    };
    if got != 0 {
        return Err(format!(
            "the kernel refused the caller's credentials: {}",
            std::io::Error::last_os_error()
        ));
    }
    Ok((answered.pid as u32, answered.uid, answered.gid))
}

/// The whole of what the kernel says about the caller.
///
/// **On the machine Smoozy rents this is one option**, `unix(7)`'s, quoted at
/// the head of this file. **At the desk it is two**, because that system carries
/// the login and the identifier under different options: `getpeereid(3)` for the
/// login, and the socket option its own header names — `sys/un.h`, shipped in
/// the system's development files and read on 2026-09-04 at
/// `/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/un.h`:
/// *"#define SOL_LOCAL 0"*, and *"#define LOCAL_PEERPID 0x002 /\* retrieve peer
/// pid \*/"*.
pub fn the_kernels_answer_about(stream: &UnixStream) -> Result<PeerCredentials, String> {
    #[cfg(target_os = "linux")]
    {
        let (process, user, group) = the_whole_of_what_linux_answers(stream)?;
        Ok(PeerCredentials {
            process,
            user,
            group,
        })
    }
    #[cfg(not(target_os = "linux"))]
    {
        use std::os::fd::AsRawFd;
        let (user, group) = the_callers_login(stream)?;
        let mut process: libc::pid_t = 0;
        let mut room = std::mem::size_of::<libc::pid_t>() as libc::socklen_t;
        let got = unsafe {
            libc::getsockopt(
                stream.as_raw_fd(),
                libc::SOL_LOCAL,
                libc::LOCAL_PEERPID,
                &mut process as *mut libc::pid_t as *mut libc::c_void,
                &mut room,
            )
        };
        if got != 0 {
            return Err(format!(
                "the kernel refused the caller's identifier: {}",
                std::io::Error::last_os_error()
            ));
        }
        Ok(PeerCredentials {
            process: process as u32,
            user,
            group,
        })
    }
}

/// A supervisor holding the endpoint and answering callers, for the tests of
/// this crate's three framed askings.
///
/// **It is here rather than in each of them** because the three tests are of
/// three different askings over one arrangement, and three copies of the
/// arrangement would be three chances for one of them to drift into proving
/// something else.
#[cfg(test)]
pub(crate) mod as_a_supervisor {
    use super::*;

    /// Bind at `path`, then answer `how_many` callers on a thread of its own.
    ///
    /// The bind happens before the thread starts, so a caller cannot arrive
    /// before there is anything to arrive at.
    pub(crate) fn answering(
        path: &str,
        how_many: usize,
        answer: impl Fn(&Request) -> Answered + Send + 'static,
    ) -> std::thread::JoinHandle<()> {
        let endpoint = TheEndpointOnTheMachine::at(path);
        assert_eq!(endpoint.bind_at(path).unwrap(), BindAttempt::Bound);
        std::thread::spawn(move || {
            for _ in 0..how_many {
                endpoint
                    .answer_one_caller(
                        unsafe { libc::geteuid() },
                        Duration::from_secs(10),
                        Duration::from_millis(2000),
                        |request| answer(request),
                    )
                    .expect("the caller was reached");
            }
        })
    }

    /// A path nothing has ever bound, unique within this run.
    pub(crate) fn a_path() -> String {
        static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let at = format!(
            "/tmp/smz-asking-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        );
        std::fs::remove_file(&at).ok();
        at
    }

    /// Run `doing` with a bound, so that **a wait with no bound fails rather than
    /// hangs**.
    ///
    /// **This exists because of a shape this project has already met**: a
    /// deadline taken out of a blocking read makes no test fail — it makes every
    /// test that reaches it wait for ever, which reads as a stuck run rather than
    /// as a broken promise. A test that cannot fail is not a test.
    pub(crate) fn within<T: Send + 'static>(
        how_long: Duration,
        doing: impl FnOnce() -> T + Send + 'static,
    ) -> T {
        let (say, hear) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let _ = say.send(doing());
        });
        hear.recv_timeout(how_long)
            .expect("it did not finish inside its bound: a wait with no bound hangs")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::acting::endpoint::as_a_supervisor;
    use crate::endpoint::{what_the_start_does, WhatTheStartDoes};
    use smoozy_framing::framing::{Fields, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS};
    use smoozy_state::identities::{Identity, IdentityKind};
    use std::path::PathBuf;

    /// A short path: a filesystem-named socket has a length bound of its own on
    /// every unix, and the desk's temporary directory is long.
    fn a_path() -> PathBuf {
        // **A counter rather than a clock.** The clock's own resolution let two
        // tests of this file compute the same path in one run, and the second
        // one's bind then met the first one's object; a counter cannot.
        static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let at = PathBuf::from("/tmp").join(format!(
            "smz-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        ));
        std::fs::remove_file(&at).ok();
        at
    }

    #[test]
    fn a_first_start_binds_and_the_object_permits_that_login_alone() {
        use std::os::unix::fs::PermissionsExt;
        let at = a_path();
        let endpoint = TheEndpointOnTheMachine::at(&at.display().to_string());
        assert_eq!(
            endpoint.bind_at(&at.display().to_string()).unwrap(),
            BindAttempt::Bound
        );
        let mode = std::fs::metadata(&at).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o700);
        endpoint.remove_its_own_object().unwrap();
    }

    /// **The interlock, run rather than reasoned.** A live holder refuses the
    /// second start's bind and accepts its probe, so that start ends without
    /// touching a store — which is S3.
    #[test]
    fn a_live_holder_refuses_the_bind_and_accepts_the_probe() {
        let at = a_path();
        let path = at.display().to_string();
        let first = TheEndpointOnTheMachine::at(&path);
        assert_eq!(first.bind_at(&path).unwrap(), BindAttempt::Bound);

        let second = TheEndpointOnTheMachine::at(&path);
        let bind = second.bind_at(&path).unwrap();
        assert_eq!(bind, BindAttempt::RefusedTheAddressIsInUse);
        let probe = second.probe_as_a_caller(&path).unwrap();
        assert_eq!(probe, ConnectProbe::Accepted);
        assert_eq!(
            what_the_start_does(bind, Some(probe)),
            WhatTheStartDoes::ThisStartEndsAnotherSupervisorHoldsIt
        );
        assert!(!second.is_bound());
        first.remove_its_own_object().unwrap();
    }

    /// **Z4's ordinary case after a crash**: an object left behind refuses the
    /// bind exactly as a live holder does, and the probe is what separates them.
    #[test]
    fn a_stale_object_refuses_the_bind_and_refuses_the_probe() {
        let at = a_path();
        let path = at.display().to_string();
        {
            let crashed = TheEndpointOnTheMachine::at(&path);
            assert_eq!(crashed.bind_at(&path).unwrap(), BindAttempt::Bound);
            // The run ends without removing it, which is Z4's own sentence.
        }
        assert!(at.exists(), "the object outlives the run that made it");

        let next = TheEndpointOnTheMachine::at(&path);
        let bind = next.bind_at(&path).unwrap();
        assert_eq!(bind, BindAttempt::RefusedTheAddressIsInUse);
        let probe = next.probe_as_a_caller(&path).unwrap();
        assert_eq!(probe, ConnectProbe::Refused);
        assert_eq!(
            what_the_start_does(bind, Some(probe)),
            WhatTheStartDoes::RemoveTheObjectAtItsOwnRecordedPathAndBindAgain
        );
        next.remove_its_own_object().unwrap();
        assert_eq!(next.bind_at(&path).unwrap(), BindAttempt::Bound);
        next.remove_its_own_object().unwrap();
    }

    #[test]
    fn a_probe_at_a_path_with_nothing_there_is_refused_rather_than_unreadable() {
        let endpoint = TheEndpointOnTheMachine::at("/tmp/smz-nothing-is-here");
        assert_eq!(
            endpoint.probe_as_a_caller("/tmp/smz-nothing-is-here").unwrap(),
            ConnectProbe::Refused
        );
    }

    /// **Z6 and the admission it feeds**, taken from a real accepted
    /// connection's own kernel-reported credentials.
    #[test]
    fn the_callers_identity_comes_from_the_connection_and_admits_the_supervisors_own_login() {
        let at = a_path();
        let path = at.display().to_string();
        let endpoint = TheEndpointOnTheMachine::at(&path);
        endpoint.bind_at(&path).unwrap();

        let calling = std::thread::spawn({
            let path = path.clone();
            move || {
                let _caller = UnixStream::connect(&path).expect("the caller reaches it");
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
        });
        let caller = endpoint.take_the_next_caller(Duration::from_secs(5)).unwrap();
        let credentials = endpoint.peer_credentials(&caller).unwrap();
        assert_eq!(credentials.user, unsafe { libc::geteuid() });
        assert_eq!(
            admit(credentials, unsafe { libc::geteuid() }),
            Admission::Admitted
        );
        // A caller of any other login is refused before the request is framed.
        assert_eq!(
            admit(credentials, credentials.user.wrapping_add(1)),
            Admission::RefusedBeforeTheRequestIsFramed
        );
        let_the_refused_caller_go(caller);
        calling.join().ok();
        endpoint.remove_its_own_object().unwrap();
    }

    /// **A supervisor that does not hold the endpoint takes no caller**, and
    /// says so rather than answering about a connection it never had.
    #[test]
    fn a_supervisor_that_does_not_hold_the_endpoint_takes_no_caller() {
        let endpoint = TheEndpointOnTheMachine::at("/tmp/smz-never-bound");
        let why = endpoint
            .take_the_next_caller(Duration::from_millis(50))
            .expect_err("it holds nothing");
        assert!(why.contains("does not hold the endpoint"));
    }

    /// **The accept is bounded, so a supervisor nobody calls is not parked for
    /// ever holding its own bind.**
    ///
    /// **The whole call is put inside a bound of its own**, because a deadline
    /// taken out of an accept makes no test fail — it makes the run stop instead
    /// of report.
    #[test]
    fn an_accept_no_caller_arrives_for_ends_at_its_deadline_rather_than_parking() {
        let at = a_path();
        let path = at.display().to_string();
        let endpoint = TheEndpointOnTheMachine::at(&path);
        assert_eq!(endpoint.bind_at(&path).unwrap(), BindAttempt::Bound);
        let why = as_a_supervisor::within(Duration::from_secs(5), move || {
            let why = endpoint
                .take_the_next_caller(Duration::from_millis(150))
                .expect_err("nobody called");
            // The bind is still this supervisor's afterwards, and a caller
            // arriving later is still taken.
            assert!(endpoint.is_bound());
            endpoint.remove_its_own_object().unwrap();
            why
        });
        assert!(why.contains("no caller arrived inside"));
    }

    #[test]
    fn removing_an_object_that_is_not_there_is_not_a_refusal() {
        let endpoint = TheEndpointOnTheMachine::at("/tmp/smz-never-made");
        endpoint.remove_its_own_object().expect("it is not a refusal");
    }

    /// **Z2b's race, performed.** The plan's own record of it was taken on the
    /// Creator's Mac; this is the same three steps through this crate's own
    /// code, and it is here so that nobody reads the race as reasoning.
    #[test]
    fn a_live_holders_object_can_be_removed_and_rebound_underneath_it() {
        let at = a_path();
        let path = at.display().to_string();
        let first = TheEndpointOnTheMachine::at(&path);
        assert_eq!(first.bind_at(&path).unwrap(), BindAttempt::Bound);
        assert_eq!(first.probe_as_a_caller(&path).unwrap(), ConnectProbe::Accepted);

        // A second start, having concluded stale, removes and binds.
        let second = TheEndpointOnTheMachine::at(&path);
        second.remove_its_own_object().unwrap();
        assert_eq!(second.bind_at(&path).unwrap(), BindAttempt::Bound);

        // Both are listening. New callers reach the second; the first is alive
        // and unreachable, which is the condition S1 says makes every rule in
        // this document a rule rather than a hope.
        assert!(first.is_bound() && second.is_bound());
        assert_eq!(second.probe_as_a_caller(&path).unwrap(), ConnectProbe::Accepted);
        second.remove_its_own_object().unwrap();
    }

    /// A supervisor holding the endpoint and answering exactly one caller.
    ///
    /// **The bind happens before the thread starts**, so a caller in the test
    /// cannot arrive before there is anything to arrive at.
    fn a_supervisor_answering_one_caller(
        path: &str,
        the_login_it_runs_as: u32,
        answer: impl FnOnce(&Request) -> Answered + Send + 'static,
    ) -> std::thread::JoinHandle<Result<WhatHappenedToTheCaller, String>> {
        let endpoint = TheEndpointOnTheMachine::at(path);
        assert_eq!(endpoint.bind_at(path).unwrap(), BindAttempt::Bound);
        std::thread::spawn(move || {
            endpoint.answer_one_caller(
                the_login_it_runs_as,
                Duration::from_secs(10),
                Duration::from_millis(2000),
                answer,
            )
        })
    }

    fn a_request(name: &str) -> Request {
        Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            Identity::issue(IdentityKind::Operation).expect("randomness"),
            name,
        )
    }

    /// **One connection, one framed request, one correlated result, over a real
    /// endpoint.** This is `Server protocol and transfers` P1 performed rather
    /// than reasoned about.
    #[test]
    fn one_framed_request_goes_over_a_real_endpoint_and_its_result_correlates() {
        let at = a_path();
        let path = at.display().to_string();
        let supervisor = a_supervisor_answering_one_caller(&path, unsafe { libc::geteuid() }, |request| {
            Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "2026.09.04")
                .carrying(Fields::none().and("said", request.name.as_str()))
        });

        let caller = TheEndpointAsACaller::at(&path, Duration::from_millis(2000));
        let request = a_request("readiness");
        let answered = caller
            .ask(&request)
            .expect("the system let it through")
            .expect("the supervisor answered");
        assert!(answered.answers(&request));
        assert_eq!(answered.package_version, "2026.09.04");
        assert_eq!(answered.fields.text("said"), Some("readiness"));

        match supervisor.join().unwrap().unwrap() {
            WhatHappenedToTheCaller::ItWasAnswered(read) => assert_eq!(read.name, "readiness"),
            other => panic!("{other:?}"),
        }
        std::fs::remove_file(&at).ok();
    }

    /// **Z9, performed.** A caller of another login receives nothing that
    /// describes the protocol — so from the caller's own side it is silence, and
    /// silence is what `ask` answers.
    #[test]
    fn a_caller_of_another_login_gets_nothing_that_describes_the_protocol() {
        let at = a_path();
        let path = at.display().to_string();
        let supervisor = a_supervisor_answering_one_caller(
            &path,
            unsafe { libc::geteuid() }.wrapping_add(1),
            |_| panic!("the request must never be framed"),
        );

        let caller = TheEndpointAsACaller::at(&path, Duration::from_millis(2000));
        assert_eq!(caller.ask(&a_request("readiness")).unwrap(), None);
        // **Bounded, so that a supervisor left sitting in its accept fails
        // rather than hangs.** The assertion above passes just as happily if the
        // caller never connected at all, and an unbounded join on that thread
        // stops the run instead of reporting.
        assert_eq!(
            as_a_supervisor::within(Duration::from_secs(5), move || supervisor.join())
                .unwrap()
                .unwrap(),
            WhatHappenedToTheCaller::RefusedBeforeTheRequestIsFramed
        );
        std::fs::remove_file(&at).ok();
    }

    /// **Nothing at the path is the far end not answering**, and it is not a
    /// reading of presence: no object is looked for and none is read.
    #[test]
    fn a_request_at_a_path_nothing_holds_is_not_answered() {
        let caller = TheEndpointAsACaller::at(
            "/tmp/smz-nobody-holds-this",
            Duration::from_millis(200),
        );
        assert_eq!(caller.ask(&a_request("readiness")).unwrap(), None);
    }

    /// **The one that would hang rather than fail.** A supervisor that holds its
    /// endpoint and answers nothing is reached, written to, and then waited on —
    /// and the wait is bounded, so it ends as *not answered* rather than never.
    #[test]
    fn a_supervisor_that_holds_the_endpoint_and_answers_nothing_is_not_answered() {
        let at = a_path();
        let path = at.display().to_string();
        let holder = TheEndpointOnTheMachine::at(&path);
        assert_eq!(holder.bind_at(&path).unwrap(), BindAttempt::Bound);
        // It never accepts anything.

        let caller = TheEndpointAsACaller::at(&path, Duration::from_millis(150));
        let request = a_request("readiness");
        let answered = as_a_supervisor::within(Duration::from_secs(3), move || caller.ask(&request));
        assert_eq!(answered.unwrap(), None);
        holder.remove_its_own_object().unwrap();
    }

    /// **`Server protocol and transfers` C2.** A result carrying another
    /// identity is not this request's answer,
    /// and it is not read as one.
    #[test]
    fn a_result_that_correlates_with_nothing_is_not_this_requests_answer() {
        let at = a_path();
        let path = at.display().to_string();
        let supervisor = a_supervisor_answering_one_caller(&path, unsafe { libc::geteuid() }, |_| {
            let somebody_elses = Request::speaking(
                THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
                Identity::issue(IdentityKind::Operation).expect("randomness"),
                "readiness",
            );
            Answered::to(
                &somebody_elses,
                THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
                "2026.09.04",
            )
        });

        let caller = TheEndpointAsACaller::at(&path, Duration::from_millis(2000));
        assert_eq!(caller.ask(&a_request("readiness")).unwrap(), None);
        // **Bounded, for the same reason as above**: the assertion holds whether
        // or not the caller ever reached the supervisor, so the join is where an
        // unbounded wait would turn a broken promise into a stuck run.
        as_a_supervisor::within(Duration::from_secs(5), move || supervisor.join())
            .unwrap()
            .unwrap();
        std::fs::remove_file(&at).ok();
    }


    /// **The other side of the same bound.** A caller that connects and then
    /// sends nothing does not hold the supervisor: the read has a deadline, and
    /// the caller is let go the way a refused one is.
    #[test]
    fn a_caller_that_connects_and_sends_nothing_does_not_hold_the_supervisor() {
        let at = a_path();
        let path = at.display().to_string();
        let endpoint = TheEndpointOnTheMachine::at(&path);
        assert_eq!(endpoint.bind_at(&path).unwrap(), BindAttempt::Bound);

        let (stop, sit_until) = std::sync::mpsc::channel::<()>();
        let sitting = std::thread::spawn({
            let path = path.clone();
            move || {
                let _held = UnixStream::connect(&path).expect("it reaches the endpoint");
                let _ = sit_until.recv_timeout(Duration::from_secs(10));
            }
        });

        let what_happened = as_a_supervisor::within(Duration::from_secs(5), move || {
            endpoint.answer_one_caller(
                unsafe { libc::geteuid() },
                Duration::from_secs(5),
                Duration::from_millis(150),
                |_| panic!("nothing was sent, so nothing is framed"),
            )
        });
        assert!(matches!(
            what_happened,
            Ok(WhatHappenedToTheCaller::ItSentNoReadableFrame(_))
        ));
        let _ = stop.send(());
        sitting.join().ok();
        std::fs::remove_file(&at).ok();
    }

    /// **G25's deadline, both ways.** A supervisor the restarter has not produced
    /// yet is waited for; one that never arrives inside the deadline is G13's
    /// *did not answer* branch.
    #[test]
    fn a_supervisor_that_arrives_late_is_waited_for_and_one_that_never_arrives_is_not() {
        let at = a_path();
        let path = at.display().to_string();

        let caller = TheEndpointAsACaller::at(&path, Duration::from_millis(2000));
        assert_eq!(
            caller
                .ask_waiting_for_a_supervisor(&a_request("readiness"), Duration::from_millis(150))
                .unwrap(),
            None,
            "nothing ever bound, so nothing answered"
        );

        let starting = std::thread::spawn({
            let path = path.clone();
            move || {
                std::thread::sleep(Duration::from_millis(200));
                let endpoint = TheEndpointOnTheMachine::at(&path);
                endpoint.bind_at(&path).expect("it binds");
                endpoint
                    .answer_one_caller(
                        unsafe { libc::geteuid() },
                        Duration::from_secs(10),
                        Duration::from_millis(2000),
                        |request| {
                            Answered::to(
                                request,
                                THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
                                "2026.09.04",
                            )
                        },
                    )
                    .expect("it answered")
            }
        });
        let request = a_request("readiness");
        let answered = caller
            .ask_waiting_for_a_supervisor(&request, Duration::from_secs(5))
            .unwrap()
            .expect("the late supervisor answered");
        assert!(answered.answers(&request));
        starting.join().unwrap();
        std::fs::remove_file(&at).ok();
    }
}
