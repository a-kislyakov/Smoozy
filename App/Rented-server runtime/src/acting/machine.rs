//! Running one of the machine's own commands, and asking the machine what it
//! calls the login this code is running as.
//!
//! # What a command is here, and what it is not
//!
//! **This is not the pre-package surface.** `Server protocol and transfers` §1.1
//! owns the command channel the desktop reaches a bare machine through, its
//! constant command strings and its machine-self-derivation route. Everything
//! in this module runs **on** the machine, as Smoozy's own program, after
//! hand-over — so there is no command string riding a connection, no value
//! interpolated into one, and no shell.
//!
//! **No shell, and that is the point rather than a preference.** A program is
//! started with its arguments as a list, so nothing a machine reports back can
//! be read as syntax. `crontab(5)`'s own trap, quoted at K9, is what that
//! avoids in the small: an unescaped character truncating a command at that
//! point while the truncated prefix runs and its own ending is what gets
//! recorded.
//!
//! # The login the machine calls itself
//!
//! K6 requires the lingering grant to name **the machine's own name for the
//! effective user this connection's login maps to**, derived at execution and
//! never carried by Smoozy. Over the surface that is a shell substitution;
//! **here it is the machine's own user database asked directly**, which is the
//! same question and one step shorter — there is no command string for a name
//! to ride on.
//!
//! Two publications, both read 2026-09-04:
//!
//! - `geteuid(2)`, `https://man7.org/linux/man-pages/man2/getuid.2.html`:
//!   *"geteuid() returns the effective user ID of the calling process."*, and
//!   *"These functions are always successful and never modify errno."*
//! - `getpwuid_r(3)`, `https://man7.org/linux/man-pages/man3/getpwnam.3.html`:
//!   *"The getpwuid_r() functions obtain the same information as getpwuid(),
//!   but store the retrieved passwd structure in the space pointed to by pwd."*,
//!   with *"char \*pw_name; /\* username \*/"*, and *"On success, getpwuid_r()
//!   return zero, and set \*result to pwd. If no matching password record was
//!   found, these functions return 0 and store NULL in \*result. In case of
//!   error, an error number is returned, and NULL is stored in \*result."*
//!
//! **What the derived values are trusted for is exactly K6's and K7's list and
//! no more**: the name is consumed in the grant and its read-back, the number in
//! one environment value, and neither is read back as a value, recorded,
//! correlated, or ever evidence of which login Smoozy connected as. No rule
//! anywhere in this crate keys on either.

use std::ffi::CStr;
use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// What a command the machine ran came back with.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheMachineSaid {
    /// Whether the command ended at zero.
    pub ended_at_zero: bool,
    /// The status verbatim, where the machine reported one.
    pub status: Option<i32>,
    /// The ordinary stream, whole.
    pub ordinary: String,
    /// The complaints stream, whole.
    pub complaints: String,
}

impl WhatTheMachineSaid {
    /// The ordinary stream with its surrounding space taken off.
    pub fn named(&self) -> &str {
        self.ordinary.trim()
    }

    /// The whole of what the machine said, for a refusal that must carry it.
    pub fn as_text(&self) -> String {
        let complaints = self.complaints.trim();
        match self.status {
            Some(status) if complaints.is_empty() => format!("ended at {status}"),
            Some(status) => format!("ended at {status}: {complaints}"),
            None if complaints.is_empty() => "ended with no status".to_string(),
            None => format!("ended with no status: {complaints}"),
        }
    }
}

/// How long [`run`] pauses between two askings of whether the child has ended.
///
/// **A Build decision and not a plan figure.** It bounds nothing and decides
/// nothing: the deadline is the caller's own and is passed in, and this only
/// keeps the wait from being a spin. Ten milliseconds.
pub const BETWEEN_TWO_ASKINGS_OF_WHETHER_IT_HAS_ENDED: Duration = Duration::from_millis(10);

/// Run one of the machine's own commands, with its arguments as a list, **inside
/// the caller's own deadline.**
///
/// **The environment additions are a list too**, and there is exactly one
/// caller that uses them: K7's runtime-directory value. Nothing here passes a
/// secret as an argument or as an environment value, which is
/// `Security and licence` §13's rule 1 and is a rule about what Smoozy may
/// hand a process rather than about what it may print.
///
/// **The deadline is an argument and never a number chosen here**, which is
/// K17: a caller waits on its own deadline and **never on a timer that assumed a
/// number**, the measured spread across sellers being an order of magnitude.
/// This is the route every machine command in this module takes — the kind
/// detection, the lingering grant, the linger read-back, the unit enable, the
/// supervisor start and the driver's own query — so a command that never returns
/// would otherwise hold whichever of those issued it for ever.
///
/// **A command that outlasts the deadline is ended and reported**, and the
/// report says it did not answer rather than carrying a status the machine never
/// gave. Both pipes are read on threads of their own: a command that fills one
/// of them while nothing reads it stops there, and a reader taking them in turn
/// would wait on the second while the first was the full one.
pub fn run(
    program: &str,
    arguments: &[&str],
    environment: &[(&str, String)],
    inside: Duration,
) -> Result<WhatTheMachineSaid, String> {
    let mut command = Command::new(program);
    command.args(arguments);
    for (name, value) in environment {
        command.env(name, value);
    }
    command.stdin(Stdio::null());
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());
    let mut started = command
        .spawn()
        .map_err(|why| format!("{program} could not be run: {why}"))?;
    let ordinary = read_it_whole(started.stdout.take());
    let complaints = read_it_whole(started.stderr.take());

    let asked_at = Instant::now();
    let ended = loop {
        match started.try_wait() {
            Ok(Some(ended)) => break ended,
            Ok(None) => {}
            Err(why) => return Err(format!("{program} could not be waited on: {why}")),
        }
        if asked_at.elapsed() >= inside {
            // **Ended, and then reported.** The pipes close with the child, so
            // the two readers finish and neither is left holding a thread.
            started.kill().ok();
            started.wait().ok();
            let _ = ordinary.join();
            let _ = complaints.join();
            return Err(format!(
                "{program} did not answer inside {inside:?} and was ended"
            ));
        }
        std::thread::sleep(BETWEEN_TWO_ASKINGS_OF_WHETHER_IT_HAS_ENDED);
    };
    Ok(WhatTheMachineSaid {
        ended_at_zero: ended.success(),
        status: ended.code(),
        ordinary: ordinary.join().unwrap_or_default(),
        complaints: complaints.join().unwrap_or_default(),
    })
}

/// One pipe, read to its end on a thread of its own.
fn read_it_whole(
    pipe: Option<impl Read + Send + 'static>,
) -> std::thread::JoinHandle<String> {
    std::thread::spawn(move || {
        let mut whole = Vec::new();
        if let Some(mut pipe) = pipe {
            let _ = pipe.read_to_end(&mut whole);
        }
        String::from_utf8_lossy(&whole).into_owned()
    })
}

/// What the machine calls the login this program runs as.
///
/// **Both halves are the machine's own claim about itself** and are trusted for
/// exactly what K6 and K7 need. K6 states the bound plainly: on an honest
/// machine the derivation names the one login the grant is permitted to name —
/// the caller's own, any other being refused outright — and a derivation naming
/// no login the machine holds fails the grant and K5's read-back loudly, before
/// the commit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheMachinesOwnLogin {
    /// The effective user number, `geteuid(2)`'s.
    pub number: u32,
    /// The name the machine's own user database gives that number.
    pub name: String,
}

impl TheMachinesOwnLogin {
    /// Ask the machine.
    pub fn asked() -> Result<TheMachinesOwnLogin, String> {
        // `geteuid` is documented as always successful and never setting an
        // error, so there is no failure branch to write for it.
        let number = unsafe { libc::geteuid() };
        Ok(TheMachinesOwnLogin {
            number,
            name: the_name_the_database_gives(number)?,
        })
    }

    /// **K7's runtime-directory value**, which is a fixed literal prefix
    /// completed by the numeric effective user identifier.
    ///
    /// The prefix's form is the observed box's — `/run/user/1001` there — and
    /// K7 says outright that it *"stands on the observed box alone"*, under the
    /// same one-or-two-boxes caveat every machine observation in this plan
    /// carries. **A wrong prefix fails loudly rather than silently**: the
    /// user-level control tool cannot reach a bus over a path that is not the
    /// runtime directory, so the grant or the read-back errors and K5 fails the
    /// placement before the commit.
    pub fn the_runtime_directory(&self) -> String {
        format!("/run/user/{}", self.number)
    }
}

fn the_name_the_database_gives(number: u32) -> Result<String, String> {
    // The buffer the database's own record is copied into. The size is asked
    // for rather than guessed, and where the machine will not say, a plain
    // starting size is grown until the call stops asking for more.
    let mut wanted = match unsafe { libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX) } {
        size if size > 0 => size as usize,
        _ => 1024,
    };
    loop {
        let mut room = vec![0i8; wanted];
        let mut record: libc::passwd = unsafe { std::mem::zeroed() };
        let mut found: *mut libc::passwd = std::ptr::null_mut();
        let answered = unsafe {
            libc::getpwuid_r(
                number as libc::uid_t,
                &mut record,
                room.as_mut_ptr() as *mut libc::c_char,
                room.len(),
                &mut found,
            )
        };
        if answered == libc::ERANGE && wanted < 1 << 20 {
            wanted *= 2;
            continue;
        }
        if answered != 0 {
            return Err(format!(
                "the machine's user database refused the effective user {number}: error {answered}"
            ));
        }
        if found.is_null() {
            // Documented: zero with a null result is *no matching record*, and
            // that is a machine whose grant would name no login it holds. It is
            // said rather than filled in.
            return Err(format!(
                "the machine's user database holds no record for the effective user {number}"
            ));
        }
        let name = unsafe { CStr::from_ptr(record.pw_name) };
        return name
            .to_str()
            .map(str::to_string)
            .map_err(|why| format!("the machine's name for the effective user is unreadable: {why}"));
    }
}

/// Whether a path the machine owns is there.
///
/// **This is a reading of the machine and never a reading of Smoozy's own
/// state.** `State, identity and recovery` §1.3 rule 5 forbids reading a file's
/// presence, name or size **as state** — a fact anything of Smoozy's branches
/// on. K1 names the container marker file as one of its two readings and
/// `sd_booted(3)` names a directory as the systemd one; both are the machine
/// telling Smoozy what the machine is, which is the same class of thing as a
/// tool's exit status.
pub fn the_machine_holds(path: &Path) -> bool {
    path.exists()
}

/// **The login number this crate's tests give the machine, and it is chosen so
/// that no machine running the tests can supply the answer they are asking
/// for.**
///
/// K7's derived value is `/run/user/<number>`, and the tests that watch a
/// user-level command read what the stand-in tool actually saw in
/// `XDG_RUNTIME_DIR`. **On a Linux machine that variable is already in the test
/// program's own environment**, and a child started from here inherits it — so a
/// test written around a number a real login could have cannot tell the value
/// K7 exported from the value the machine happened to be carrying.
///
/// **Both directions of that had already been written and both were wrong.**
/// The *nothing is exported* test asserted the child saw nothing, which was true
/// only because macOS sets the variable at all: it passed at the desk for six
/// days and failed the first time it ran on Linux, where the session's own
/// `/run/user/1001` arrived instead. The *exported by hand* test asserted the
/// child saw `/run/user/1001` — the hosted runner's own value — so on that
/// machine it would have passed with the export removed altogether.
///
/// **1_234_567 is the same choice [`the_name_the_database_gives`]'s own test
/// makes** and for the same reason: a plain number nothing is expected to claim.
/// It is deliberately not the largest, which the desk's database holds `nobody`
/// at.
#[cfg(test)]
pub(crate) const THE_TEST_LOGINS_NUMBER: u32 = 1_234_567;

/// K7's derived value for [`THE_TEST_LOGINS_NUMBER`], composed by the same
/// method the code uses rather than written out.
#[cfg(test)]
pub(crate) fn the_test_logins_runtime_directory() -> String {
    TheMachinesOwnLogin {
        number: THE_TEST_LOGINS_NUMBER,
        name: "the machines own name".into(),
    }
    .the_runtime_directory()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A deadline no ordinary command at the desk comes near, for the tests
    /// that are not about the deadline itself.
    const LONG_ENOUGH: Duration = Duration::from_secs(30);

    #[test]
    fn a_command_that_ends_at_zero_carries_what_it_printed() {
        let said = run("/bin/echo", &["a line"], &[], LONG_ENOUGH).expect("echo runs at the desk");
        assert!(said.ended_at_zero);
        assert_eq!(said.status, Some(0));
        assert_eq!(said.named(), "a line");
    }

    #[test]
    fn a_command_that_ends_above_zero_is_not_read_as_a_yes() {
        let said =
            run("/bin/sh", &["-c", "exit 3"], &[], LONG_ENOUGH).expect("the shell runs at the desk");
        assert!(!said.ended_at_zero);
        assert_eq!(said.status, Some(3));
        assert!(said.as_text().contains("ended at 3"));
    }

    #[test]
    fn a_command_that_is_not_there_is_a_refusal_and_never_a_no() {
        let why = run("/no/such/program/at/all", &[], &[], LONG_ENOUGH)
            .expect_err("there is no such program");
        assert!(why.contains("could not be run"));
    }

    /// **The arguments are a list and never a line**, so a value that looks
    /// like syntax is a value.
    #[test]
    fn an_argument_that_looks_like_syntax_is_an_argument() {
        let said =
            run("/bin/echo", &["; rm -rf /", "$(whoami)"], &[], LONG_ENOUGH).expect("echo runs");
        assert_eq!(said.named(), "; rm -rf / $(whoami)");
    }

    #[test]
    fn the_environment_reaches_the_one_command_and_nothing_else() {
        let said = run(
            "/bin/sh",
            &["-c", "printf %s \"$SMOOZY_A_TEST_VALUE\""],
            &[("SMOOZY_A_TEST_VALUE", "the value".to_string())],
            LONG_ENOUGH,
        )
        .expect("the shell runs");
        assert_eq!(said.named(), "the value");
        assert!(std::env::var("SMOOZY_A_TEST_VALUE").is_err());
    }

    /// **K17's bound, run against a program that never returns.** It is ended
    /// and reported rather than waited on.
    ///
    /// **The whole call is put inside a bound of its own**, because a deadline
    /// taken out of a wait makes no test fail — it makes the run stop instead of
    /// report, which is the shape this step has already been bitten by twice.
    #[test]
    fn a_command_that_never_returns_is_ended_and_reported_rather_than_waited_on() {
        use crate::acting::endpoint::as_a_supervisor;
        let why = as_a_supervisor::within(Duration::from_secs(10), || {
            run(
                "/bin/sleep",
                &["300"],
                &[],
                Duration::from_millis(150),
            )
            .expect_err("it never returns")
        });
        assert!(why.contains("did not answer inside"));
    }

    /// **A command that prints more than one pipe holds is still read whole**,
    /// which is why both are read on threads of their own rather than in turn.
    #[test]
    fn a_command_that_prints_more_than_a_pipe_holds_is_read_whole_and_does_not_stop() {
        let said = as_a_supervisor_within_ten(|| {
            run(
                "/bin/sh",
                &["-c", "i=0; while [ $i -lt 4000 ]; do echo 'a line on the complaints stream' 1>&2; i=$((i+1)); done; echo done"],
                &[],
                Duration::from_secs(30),
            )
            .expect("the shell runs")
        });
        assert!(said.ended_at_zero);
        assert_eq!(said.named(), "done");
        assert!(said.complaints.len() > 64 * 1024);
    }

    fn as_a_supervisor_within_ten<T: Send + 'static>(doing: impl FnOnce() -> T + Send + 'static) -> T {
        crate::acting::endpoint::as_a_supervisor::within(Duration::from_secs(10), doing)
    }

    #[test]
    fn the_machine_names_the_login_this_program_runs_as() {
        let login = TheMachinesOwnLogin::asked().expect("the desk's own database answers");
        assert!(!login.name.is_empty());
        assert_eq!(login.the_runtime_directory(), format!("/run/user/{}", login.number));
    }

    /// **A number the database holds no record for is said rather than filled
    /// in**, which is what keeps a grant from naming a login the machine does
    /// not hold.
    #[test]
    fn a_number_the_database_does_not_hold_is_refused_rather_than_guessed() {
        // Not the largest number: the desk's own database holds `nobody` at
        // that one, which is a machine fact rather than a fault and is why the
        // number here is a plain one nothing is expected to claim.
        let why =
            the_name_the_database_gives(1_234_567).expect_err("no database holds that number");
        assert!(why.contains("1234567"));
    }
}
