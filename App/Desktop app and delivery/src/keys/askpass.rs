//! The askpass helper and the channel that feeds it — `Desktop app and
//! delivery` §9 S7 and `Security and licence` §1.4 hop 3.
//!
//! **What the route is, in one line.** `Keys` G3 and S5 run `ssh-keygen` with
//! no terminal, `SSH_ASKPASS` naming a helper and `SSH_ASKPASS_REQUIRE=force`;
//! the tool asks the helper for the passphrase; **the helper receives it over a
//! channel Smoozy opened for that one use — never as an argument, never in an
//! environment value, never through a file** (`Security and licence` §4 R2).
//!
//! **Three things are written down here because a builder meeting this route
//! discovers them the hard way otherwise, and one of them narrows a clause.**
//!
//! 1. **S7 says the helper is *started by the one-use broker*, and on this
//!    route it is not: `ssh-keygen` execs it.** What the broker starts is the
//!    **channel** and the tool run; the tool starts the helper because that is
//!    what `SSH_ASKPASS` means. Everything else in S7 holds exactly as written
//!    — the helper lives for one delivery, holds the plaintext only for the
//!    length of it, binds no listener, and takes no secret as an argument or an
//!    environment value. **The narrowing is recorded rather than smoothed
//!    over**, at `Audits/Build/Keys/Decisions record.md`.
//! 2. **The helper is Smoozy's own binary, re-entered.** `SSH_ASKPASS` names a
//!    program and the tool passes it the prompt as its one argument, so there
//!    is no room for a flag of ours; what says *be the helper* is the presence
//!    of [`CHANNEL_VARIABLE`] in the environment, whose value is **a path to a
//!    channel and never a secret**. It is set on the tool's own invocation and
//!    on nothing else.
//! 3. **The tool asks more than once for one delivery.** Observed at `Keys` G3:
//!    a generation with a passphrase invoked the askpass program **exactly
//!    twice** — the prompt and its confirmation — while the one `-y` load
//!    invoked it once. **So the channel answers every ask of that one run and
//!    is closed when the run ends**, which is one delivery of one value for one
//!    act and not a second delivery.

use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;

use smoozy_keys::generation::{ASKPASS_REQUIREMENT, ASKPASS_VARIABLE, FORCE, THE_TOOL};
use smoozy_security_and_licence::plaintext::OneUseBroker;

/// The environment value that names the channel and turns Smoozy's own binary
/// into the helper. **It carries a path and never a value** (`Security and
/// licence` §13 rules 1 and 2).
pub const CHANNEL_VARIABLE: &str = "SMOOZY_ASKPASS_CHANNEL";

/// Where one route is arranged: the area the channel is bound in, and the
/// program `SSH_ASKPASS` names.
///
/// **The helper is a value the route is given rather than one it discovers.**
/// It is Smoozy's own binary re-entered (S7), and [`TheRoute::of_this_application`]
/// is how every act names it — but *which* binary that is has to be answerable
/// by a test that must not re-enter a test harness, so the route takes the path
/// rather than reading `current_exe` behind the caller's back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheRoute {
    /// Smoozy's own temporary area — swept at application start (`Desktop app
    /// and delivery` L3a), never a directory any module scans for keys
    /// (`Security and licence` §7).
    pub channel_area: PathBuf,
    /// The program `SSH_ASKPASS` names. **A path and never a secret**, set on
    /// the tool's own invocation and on nothing else.
    pub helper: PathBuf,
}

impl TheRoute {
    /// The route as the application arranges it: **this application's own
    /// binary as the helper**, which is what S7 means and what
    /// [`this_launch_is_the_helper`] branches on when the tool execs it.
    pub fn of_this_application(channel_area: PathBuf) -> Result<TheRoute, CouldNotDeliver> {
        let helper = std::env::current_exe().map_err(CouldNotDeliver::reported)?;
        Ok(TheRoute {
            channel_area,
            helper,
        })
    }

    /// The route with a named helper. **The one caller that needs this is a
    /// test holding the built application's path**: a test binary named as the
    /// helper would have `ssh-keygen` re-enter the test harness, which answers
    /// a different question and answers it wrongly.
    pub fn with_the_helper_at(channel_area: PathBuf, helper: PathBuf) -> TheRoute {
        TheRoute {
            channel_area,
            helper,
        }
    }
}

/// What went wrong before the tool ever ran. **A delivery that could not be
/// arranged is a failed act said plainly** — never a key quietly weaker than
/// the user asked for, which is what `Keys` G4's verification then also
/// catches from the other side.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CouldNotDeliver {
    /// What the system reported, for the diary and never for the user.
    pub reported: String,
    /// **Smoozy's own sentence for the user, where Smoozy knows why** — the
    /// Windows limit, or a channel path longer than this system accepts.
    /// `None` where all Smoozy has is the system's report, which is a
    /// diagnostic and is never put on a screen.
    pub said: Option<&'static str>,
}

impl CouldNotDeliver {
    /// A refusal Smoozy has no sentence of its own for.
    fn reported(error: impl ToString) -> CouldNotDeliver {
        CouldNotDeliver {
            reported: error.to_string(),
            said: None,
        }
    }
}

/// **What the user is told where the channel's path is longer than this
/// system accepts** — `Preparation/Design/Keys.md`, *Where the passphrase
/// cannot be handed to the SSH tools*, word for word.
///
/// *(Added 2026-09-18, step 10b's audit round's loose ends, L1.)* Every
/// passphrase goes to the tool down this one channel, so on such a computer no
/// key with a passphrase can be made or used; the sentence says that and why,
/// in the shape the Windows limit is said in. It used to reach the user as
/// *Smoozy could not connect to this server.*, which says nothing true about
/// why.
pub const THE_PATH_IS_TOO_LONG: &str = "Keys with a passphrase cannot be created or used: the path to Smoozy's data folder is too long.";

/// **The longest path a channel may have on this system**: the room the
/// system's own socket address gives a path, less the one byte that ends it.
/// Read from the system's own definition rather than written down as a number.
/// *(Observed 2026-09-18 on the Creator's Mac: a channel path of 116
/// characters could not be opened, and every passphrase then failed.)*
#[cfg(unix)]
pub fn the_longest_channel_path() -> usize {
    // SAFETY: a socket address is plain bytes, and all-zero is a valid one.
    let address: libc::sockaddr_un = unsafe { std::mem::zeroed() };
    address.sun_path.len() - 1
}

/// **The channel's own name, as short as it can honestly be**: one letter and
/// a number this process has never used for a scratch name.
///
/// **No process id, and nothing removed before the bind.** Two routes in one
/// process are told apart by the number; two launches sharing one area are
/// told apart by the bind itself, which refuses a name that is already there —
/// and [`run_with_the_askpass_route`] then takes the next number rather than
/// unlinking somebody else's channel. *(2026-09-18, L1: the name was
/// `ap<process id>-<number>`, up to fourteen characters, where two are
/// enough.)*
#[cfg(unix)]
fn a_channel_name() -> String {
    format!("a{}", super::readings::next_scratch_number())
}

/// How many names a route tries before it gives up.
#[cfg(unix)]
const NAMES_TRIED: usize = 64;

/// How the tool run ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolRun {
    Finished { exit_was_success: bool },
}

/// Runs one `ssh-keygen` invocation with the askpass route open for it.
///
/// **The plaintext is exposed for the length of this call and for nothing
/// else**: the broker is spent by the delivery, the value is never copied out
/// of the closure it is exposed in, and the channel is closed and removed
/// before this function returns.
///
/// `the_route` carries Smoozy's own temporary area — swept at application start
/// (`Desktop app and delivery` L3a), never a directory any module scans for keys (`Security and licence`
/// §7) — and the program `SSH_ASKPASS` names.
#[cfg(unix)]
pub fn run_with_the_askpass_route(
    arguments: &[String],
    broker: OneUseBroker,
    the_route: &TheRoute,
) -> Result<ToolRun, CouldNotDeliver> {
    use std::os::unix::net::UnixListener;

    let helper = &the_route.helper;
    let channel_area = the_route.channel_area.as_path();

    // **A short name on purpose, and a component of its own per call.** A Unix
    // socket's path has a length limit far shorter than a path's, and the
    // application-data location this area sits in carries the account name,
    // so the name adds as little to it as it can — see [`a_channel_name`].
    // This is the channel a passphrase travels down: two routes open at once
    // in one process — a generation with a passphrase while a connection
    // supplies one, or two connections — must never share a name, or one
    // run's helper reads **the other operation's passphrase**.
    //
    // **Where even the shortest name makes a path this system refuses, the
    // act fails saying so** (L1). The refusal the system gives is a diagnostic
    // and the user was told *could not connect*, which is not why.
    let mut bound = None;
    for _ in 0..NAMES_TRIED {
        let channel = channel_area.join(a_channel_name());
        if channel.as_os_str().len() > the_longest_channel_path() {
            return Err(CouldNotDeliver {
                reported: format!(
                    "the channel's path is {} and this system takes {}",
                    crate::typography::a_figure_with_its_unit(channel.as_os_str().len(), "bytes"),
                    the_longest_channel_path()
                ),
                said: Some(THE_PATH_IS_TOO_LONG),
            });
        }
        match UnixListener::bind(&channel) {
            Ok(listener) => {
                bound = Some((channel, listener));
                break;
            }
            // **Somebody else's name, or an earlier run's**: never unlinked,
            // the next number is taken instead.
            Err(error) if error.kind() == std::io::ErrorKind::AddrInUse => continue,
            Err(error) => return Err(CouldNotDeliver::reported(error)),
        }
    }
    let Some((channel, listener)) = bound else {
        return Err(CouldNotDeliver::reported(
            "every channel name tried was already in use",
        ));
    };
    if let Err(error) = listener.set_nonblocking(true) {
        drop(listener);
        let _ = std::fs::remove_file(&channel);
        return Err(CouldNotDeliver::reported(error));
    }

    let mut child = Command::new(THE_TOOL)
        .args(arguments)
        .env(ASKPASS_VARIABLE, helper)
        .env(ASKPASS_REQUIREMENT, FORCE)
        .env(CHANNEL_VARIABLE, &channel)
        // **No terminal**, which is half of what makes the route fire.
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|error| {
            let _ = std::fs::remove_file(&channel);
            CouldNotDeliver::reported(error)
        })?;

    // Hop 2, spent here: the value is exposed for the length of this one
    // delivery and the broker has no second one to make.
    let ended = broker.deliver(|value| {
        value.expose(|bytes| loop {
            match listener.accept() {
                Ok((mut asking, _)) => {
                    // The tool's own askpass protocol: the value, then a
                    // newline, on the helper's standard output. What crosses
                    // this channel is that value and nothing else.
                    let _ = asking.write_all(bytes);
                    let _ = asking.flush();
                }
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    match child.try_wait() {
                        Ok(Some(status)) => {
                            return ToolRun::Finished {
                                exit_was_success: status.success(),
                            }
                        }
                        Ok(None) => std::thread::sleep(Duration::from_millis(5)),
                        Err(_) => {
                            return ToolRun::Finished {
                                exit_was_success: false,
                            }
                        }
                    }
                }
                Err(_) => {
                    return ToolRun::Finished {
                        exit_was_success: false,
                    }
                }
            }
        })
    });

    // The channel is closed and its name removed the moment the run ends.
    drop(listener);
    let _ = std::fs::remove_file(&channel);
    Ok(ended)
}

/// **Windows: this route is not built and the absence is deliberate rather
/// than forgotten.**
///
/// **No shape for the Windows channel is named here, and one was until
/// 2026-09-03.** This said *the channel wants a named pipe*, which is a claim
/// about Windows with nothing read behind it — Rule 2 — and the `Keys` round-1
/// Verdict rejected exactly that sentence. What comes first is a reading of
/// the Windows OpenSSH port's own pages; `Keys` G3a and the Build board carry
/// the condition. Every Windows claim in `Keys` and in
/// `Security and licence` §3.2 is **documentation alone on pages this project
/// has read and no machine it has run** — `Keys` §13 item 2, permanently
/// unresolved. **What is refused here is a delivery that has never been
/// arranged on any Windows computer**, and the refusal is loud: the act that
/// wanted it is reported failed, which is the same ending G4's verification
/// gives for a system that ignores the route.
///
/// **What still works on Windows without it:** every promptless read, every
/// listing, every fingerprint, and a generation with no passphrase — all of
/// which take the `-N ""` route and open no channel.
#[cfg(not(unix))]
pub fn run_with_the_askpass_route(
    _arguments: &[String],
    _broker: OneUseBroker,
    _the_route: &TheRoute,
) -> Result<ToolRun, CouldNotDeliver> {
    Err(CouldNotDeliver {
        reported: THE_WINDOWS_LIMIT.to_string(),
        said: Some(THE_WINDOWS_LIMIT),
    })
}

/// **What a Windows user is told, and it is about their key and their computer
/// rather than about a part of Smoozy.**
///
/// This string reaches the user as the reason a key could not be made or a
/// connection could not be started. It read *"the askpass channel is not built
/// on this system"* — an internal channel, named to somebody who has no idea
/// what it is and nothing they could do about it either way.
///
/// **It says what is true and stops** (`Keys` G3a): the limit is stated, not
/// softened into a promise that the route is coming, and no act is named that
/// the user could take. Standard software words, under Rule 9.
///
/// **The words are ruled and the marker is cleared, 2026-09-13 at step 10a
/// piece 4**, under the Creator's standing delegation of 2026-08-31 and with
/// the Design stage closed. **The string below is the ruling** — it was already
/// what stood, and what the marker was waiting for was somebody to weigh it
/// rather than somebody to write it. The reasoning is in that piece's account
/// and in both decisions records.
#[cfg(not(unix))]
pub const THE_WINDOWS_LIMIT: &str =
    "Keys with a passphrase are not supported on Windows.";

/// **Be the helper.** Called as the very first thing in `main`, before any
/// window, any record and any adapter — a helper that had built an application
/// on its way to printing one line would be a second Smoozy running.
///
/// **It binds no listener** (S7): it connects to the channel Smoozy opened,
/// reads the value, writes it to standard output and exits. It holds the value
/// for the length of that write and for nothing beyond it.
///
/// **A failure exits without printing.** An empty line would be read by the
/// tool as an empty passphrase, which is a wrong answer rather than no answer.
pub fn be_the_helper(channel: &Path) -> i32 {
    #[cfg(unix)]
    {
        use std::os::unix::net::UnixStream;

        let Ok(mut smoozy) = UnixStream::connect(channel) else {
            return 1;
        };
        let mut value = Vec::new();
        if smoozy.read_to_end(&mut value).is_err() {
            return 1;
        }
        let mut out = std::io::stdout();
        // The tool reads one line: the value and a newline, which is the whole
        // of the askpass program's output contract.
        if out.write_all(&value).is_err() || out.write_all(b"\n").is_err() {
            return 1;
        }
        if out.flush().is_err() {
            return 1;
        }
        // Nothing is held past the write.
        value.iter_mut().for_each(|byte| *byte = 0);
        0
    }
    #[cfg(not(unix))]
    {
        let _ = channel;
        1
    }
}

/// Whether this launch is a helper's, and where its channel is.
///
/// **The variable's presence is the whole of the test**, and it is set on the
/// tool's own invocation and on nothing else — so an ordinary launch of Smoozy
/// never takes this branch.
pub fn this_launch_is_the_helper() -> Option<PathBuf> {
    std::env::var_os(CHANNEL_VARIABLE).map(PathBuf::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(name: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-askpass-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("a scratch directory");
        path
    }

    #[test]
    fn an_ordinary_launch_is_never_the_helper() {
        assert!(this_launch_is_the_helper().is_none());
    }

    // **The whole route — the real tool, the real helper and a real key — is
    // proved by `tests/askpass_route.rs` rather than here.** *(The name was
    // corrected 2026-09-07: this read `tests/the askpass route.rs`, which is
    // no file. Four other places in this crate had it right.)* The helper is
    // Smoozy's own binary, and only an integration test is handed the built
    // application's path; a unit test naming itself as the helper would have
    // the tool re-enter the test harness, which answers a different question
    // and answers it wrongly.

    #[cfg(unix)]
    #[test]
    fn the_helper_prints_what_the_channel_hands_it_and_holds_nothing_after() {
        use std::os::unix::net::UnixListener;

        let area = scratch("helper");
        let channel = area.join("ap");
        let listener = UnixListener::bind(&channel).expect("a channel");

        let serving = std::thread::spawn(move || {
            let (mut asking, _) = listener.accept().expect("one connection");
            asking.write_all(b"open sesame").expect("written");
        });

        // The helper's own read half, without its print: what it obtains is
        // the value and it obtains it over the channel rather than from an
        // argument or an environment value.
        use std::os::unix::net::UnixStream;
        let mut smoozy = UnixStream::connect(&channel).expect("connected");
        let mut got = Vec::new();
        smoozy.read_to_end(&mut got).expect("read");
        assert_eq!(got, b"open sesame");

        serving.join().expect("served");
        let _ = std::fs::remove_dir_all(&area);
    }

    /// **L1 (2026-09-18): a channel path longer than the system takes is
    /// refused before anything runs, with Smoozy's own sentence** — never the
    /// system's diagnostic, and never a tool started with no channel behind it.
    #[cfg(unix)]
    #[test]
    fn a_channel_path_the_system_would_refuse_is_refused_with_smoozys_sentence() {
        use smoozy_security_and_licence::plaintext::Plaintext;

        let base = scratch("too-long");
        let area = base.join("x".repeat(the_longest_channel_path()));
        std::fs::create_dir_all(&area).expect("the deep area");
        let the_route = TheRoute::with_the_helper_at(area, PathBuf::from("/nonexistent/helper"));

        let refused = run_with_the_askpass_route(
            &["-y".to_string()],
            OneUseBroker::holding(Plaintext::new(b"open sesame".to_vec())),
            &the_route,
        )
        .expect_err("no channel fits");

        assert_eq!(refused.said, Some(THE_PATH_IS_TOO_LONG));
        let _ = std::fs::remove_dir_all(&base);
    }

    #[cfg(unix)]
    #[test]
    fn a_helper_with_no_channel_to_connect_to_prints_nothing_and_fails() {
        let area = scratch("no-channel");
        assert_eq!(be_the_helper(&area.join("nothing-here")), 1);
        let _ = std::fs::remove_dir_all(&area);
    }
}
