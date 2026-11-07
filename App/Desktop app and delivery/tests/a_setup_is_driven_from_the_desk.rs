//! **Step 10c piece 9's own proofs at the desk: `Server Setup` R2's order
//! driven end to end against a machine that answers, and the release admitted
//! under a root that exists only inside this file.**
//!
//! **What the far end is.** A stand-in machine that lends sessions the way a
//! real one does — a session limit, a command channel that answers a script of
//! commands, and OpenSSH's own `sftp-server` on a Unix socket pair for every
//! file-transfer session, which is the program a rental's own file-transfer
//! server is. **What it is not is an SSH connection**: this crate carries no
//! SSH server to stand one up, and the loopback SSH run is outside the
//! repository. **So what this file proves is everything above the transport**:
//! the order of `Server Setup` R2's steps, what each refusal stops, the
//! hand-over's two records, the release's admission, the placement's one lend,
//! and `Server Setup` Y7's driving of a run one step at a time.
//!
//! **The release root here is derived from a seed written out below, so it is
//! public by construction.** **No second root enters a shipped build**
//! (`Server protocol and transfers` B9, and the runner's ruling of 2026-09-19):
//! the admission takes the root as a value, this file hands it one of its own,
//! and the running application hands it
//! `smoozy_server_protocol::release::trust::THE_RELEASE_ROOT_THIS_BUILD_CARRIES`
//! and nothing else.
//!
//! **Nothing of the Creator's is touched.** Every path is inside a throwaway
//! folder made for the test, named by a fresh identity's own random bytes, and
//! **removed whole when the test ends** — the folder is the unit, not the file,
//! which is step 10c piece 8's own lesson.
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this
//! crate's own plan, `Preparation/Architecture/Desktop app and delivery.md`.
//! **Every marker of another document is written out with that document at its
//! own site and this file keeps no header list.**

#[cfg(not(unix))]
#[test]
fn the_far_end_here_wants_a_unix_socket_pair() {
    println!(
        "SKIPPED: the far end here is OpenSSH's own sftp-server on a Unix socket pair, which \
         this system ({}) has none of. The order is unproved here.",
        std::env::consts::OS
    );
}

#[cfg(unix)]
mod on_a_desk_with_a_socket_pair {
    use std::path::PathBuf;
    use std::pin::Pin;
    use std::sync::{Arc, Mutex};
    use std::task::{Context, Poll};

    use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

    use smoozy_desktop::servers::sessions::{
        lend_until_it_ends, ACommandRunning, AHeldConnection, NoSession, TheAdditionalConnection,
        ThisServersSessions,
    };
    use smoozy_desktop::setting_up::{
        a_blender_is_installed, a_blender_is_removed, a_machine_is_set_up, the_current_release,
        the_users_build_is_installed, TheRemovalAsked,
        SendsTheUsersArchiveUp,
        the_layout_this_machine_gets, what_the_opening_readings_settle, AsksForThePassword,
        a_setup_is_abandoned, AsksToStop, ComposesTheTelegramMaterial, FetchesOneSource,
        NobodyIsThereToAsk, WhatTheAbandonCameTo,
        KeepsTheServersRow, NobodyAsked, PlacesTheVerifiedRelease, ReadsTheCurrentRelease,
        TakesTheOpeningReadings, TelegramIsNotOn,
        TheMaterialFromTheRecord, TheRunsEnding, TheServersRowHere,
        WhatWasAskedFor, WhereTheSetUpGot, WhichReadingRefused,
        WHERE_THE_CURRENT_RELEASE_IS_READ_FROM,
    };
    use smoozy_server_protocol::framing::{
        read_one_request, write_result_frame, Answered, Fields,
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
    };
    use smoozy_server_protocol::placement::performing::{
        ThePlacementsOutcome, WhatThePlacementNeeds, WhatThePlacementReached,
        WhyThePlacementCouldNotBeCarriedOut,
    };
    use smoozy_server_protocol::placement::records::WhetherTheDurabilityOrderingApplied;
    use smoozy_server_protocol::release::composing::{self, test_only_keys};
    use smoozy_server_protocol::release::trust::{
        AnAllowedKey, KeySetStatement, THE_GREATEST_KEY_SET_GENERATION_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE,
        THE_GREATEST_RELEASE_SEQUENCE_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE, THE_KEY_SET_FORMAT,
    };
    use smoozy_server_protocol::release::verifying::AnOfferedRelease;
    use smoozy_server_protocol::release::{
        a_signature_over, THE_KEY_SET_SIGNING_CONTEXT, THE_MANIFEST_SIGNING_CONTEXT,
    };
    use smoozy_server_protocol::requests as asked;
    use smoozy_runtime::package::retry::AcceptedWorkStanding;
    use smoozy_server_protocol::transport::{CommandResult, CommandString, Ending};
    use smoozy_server_setup::kind::{DistributionFamily, MachineKind};
    use smoozy_server_setup::placement::{AWritableLocation, WhatTheLocationIs};
    use smoozy_server_setup::readings::TheMachineBeforeAnythingIsTouched;
    use smoozy_servers::identity::{Host, KeyReference, Port, Username};
    use smoozy_servers::store::{RecordedMachineRoot, RememberedSetup, ServerRow};
    use smoozy_state::identities::{Identity, IdentityKind, InstallationCheck};

    // ─────────────────────────────────────────────── where OpenSSH's own server is

    const WHERE_SFTP_SERVER_LIVES: &[&str] = &[
        "/usr/libexec/sftp-server",
        "/usr/lib/openssh/sftp-server",
        "/usr/libexec/openssh/sftp-server",
        "/usr/lib/ssh/sftp-server",
    ];

    fn the_machines_own_file_transfer_server() -> Option<PathBuf> {
        WHERE_SFTP_SERVER_LIVES
            .iter()
            .map(PathBuf::from)
            .find(|path| path.exists())
    }

    macro_rules! the_server_or_say_so {
        () => {
            match the_machines_own_file_transfer_server() {
                Some(path) => path,
                None => {
                    println!(
                        "SKIPPED: no OpenSSH sftp-server on this system ({}). \
                         The order is unproved here.",
                        std::env::consts::OS
                    );
                    return;
                }
            }
        };
    }

    // ───────────────────────────────────────────────────── the throwaway folder

    /// **The folder is the unit, not the file.** It takes a fresh identity's own
    /// random bytes and is removed whole, which is step 10c piece 8's lesson
    /// from two tests that composed one path out of a clock reading.
    ///
    /// **It sits in the build's own temporary folder and never the system's**,
    /// because this folder is the stand-in machine's login area — `sftp-server`
    /// runs in it, so it is the one writable location `Server Setup` R6 offers
    /// and `Job files and outputs` L1 chooses the root under. **L3 refuses a
    /// root under a shared temporary area, and on Linux the system's temporary
    /// folder is `/tmp` itself.** On a Mac it is under `/var/folders`, whose
    /// canonical form is `/private/var/folders`, so L3 never fires at the desk.
    /// **Hosted run 35508009875 is where this was found**, and run
    /// 35459394239 is where the same fault was found in a neighbouring file the
    /// day before.
    struct Scratch {
        at: PathBuf,
    }

    impl Scratch {
        fn new(named: &str) -> Scratch {
            let mine = Identity::issue(IdentityKind::Operation)
                .expect("this desk has randomness")
                .as_text();
            let at = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
                .join(format!("smoozy-p9-{named}-{mine}"));
            std::fs::create_dir_all(&at).expect("the throwaway folder is made");
            // **The machine resolves its own login area to the canonical
            // path**, so a path the machine reports and a path composed here
            // spell one folder the same way. The root this test asserts
            // against is what the machine said, and never what this side
            // composed.
            let at = std::fs::canonicalize(&at).expect("the folder canonicalises");
            Scratch { at }
        }

        fn at(&self, name: &str) -> PathBuf {
            self.at.join(name)
        }

        fn text(&self) -> String {
            self.at.to_string_lossy().into_owned()
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.at);
        }
    }

    // ───────────────────────────────────────────── the far end, and what it says

    /// A stream over a Unix socket pair, which is what each session's
    /// `sftp-server` is spoken to over.
    struct ASessionsStream(tokio::net::UnixStream);

    impl AsyncRead for ASessionsStream {
        fn poll_read(
            mut self: Pin<&mut Self>,
            context: &mut Context<'_>,
            buffer: &mut ReadBuf<'_>,
        ) -> Poll<std::io::Result<()>> {
            Pin::new(&mut self.0).poll_read(context, buffer)
        }
    }

    impl AsyncWrite for ASessionsStream {
        fn poll_write(
            mut self: Pin<&mut Self>,
            context: &mut Context<'_>,
            bytes: &[u8],
        ) -> Poll<std::io::Result<usize>> {
            Pin::new(&mut self.0).poll_write(context, bytes)
        }

        fn poll_flush(
            mut self: Pin<&mut Self>,
            context: &mut Context<'_>,
        ) -> Poll<std::io::Result<()>> {
            Pin::new(&mut self.0).poll_flush(context)
        }

        fn poll_shutdown(
            mut self: Pin<&mut Self>,
            context: &mut Context<'_>,
        ) -> Poll<std::io::Result<()>> {
            Pin::new(&mut self.0).poll_shutdown(context)
        }
    }

    /// **The machine, standing in for a rental.**
    ///
    /// Every file-transfer session is OpenSSH's own `sftp-server` in the
    /// throwaway folder; every command is answered from a script this test
    /// wrote; and every framed request to the supervisor is answered by
    /// [`TheSupervisorHere`].
    struct TheFarEnd {
        sftp_server: PathBuf,
        at: PathBuf,
        /// What each command answers, matched on the command's own text.
        said: Arc<Mutex<Vec<(String, CommandResult)>>>,
        /// Every command it was asked, in order.
        asked: Arc<Mutex<Vec<String>>>,
        supervisor: Arc<Mutex<TheSupervisorHere>>,
        children: Arc<Mutex<Vec<std::process::Child>>>,
        /// **The machine going down, which is what a restart really does to a
        /// connection.** Observed: `systemctl reboot` **returned 0 and then the
        /// connection dropped** (`Server Setup` T1). Nothing notifies this in
        /// the tests above, so their connection never ends, exactly as before.
        ///
        /// **Notified with a permit rather than to whoever is listening**, so a
        /// machine told to go down while it is answering a request still goes
        /// down when it has finished.
        goes_down: Arc<tokio::sync::Notify>,
        /// **`Server Setup` T4's mark, read at the instant the restart command
        /// reaches the machine.**
        ///
        /// **The clause is about an order, so the order is what is watched**:
        /// *"this drop is mine* is a state Smoozy sets **before the act**"*.
        /// Reading the mark afterwards would pass whether or not the mark went
        /// out first, which is exactly the fault that shipped.
        the_mark_when_the_restart_arrived:
            Option<(
                smoozy_servers::restart::TheRestartMark,
                Arc<Mutex<Option<smoozy_servers::restart::WhereTheMarkStands>>>,
            )>,
        /// **Every command, with the machine-level setup-in-progress state as
        /// it stood at the instant that command reached the machine.**
        ///
        /// **`Server Setup` Y1 is about an order** — the state is set *"before
        /// its first such step"* — so the order is what is watched. Reading the
        /// state after the run would pass whether or not it went out first,
        /// which is the same fault `Server Setup` T4's mark had.
        the_state_when_a_command_arrived: Arc<Mutex<Vec<(String, Option<String>)>>>,
        /// **Whether `systemctl reboot` takes the connection with it before it
        /// answers** — step 10c's seventh passes on Hyperstack and Sesterce,
        /// where the reboot cut the connection straight after the send and the
        /// command never came back. Added 2026-09-27, fix piece 6. **Off, the
        /// command answers from the script** and the machine goes down only
        /// when the test says so, which is the shape Fact 51 observed and every
        /// test before this date drove.
        the_restart_takes_the_connection: bool,
        /// **The boundaries the desktop had written at the instant `systemctl
        /// reboot` reached the machine** — `Server Setup` T3 and the paragraph
        /// under Y4's table are about an order, so the order is what is
        /// watched. Added 2026-09-27.
        written_when_the_restart_arrived: Arc<Mutex<Option<Vec<String>>>>,
        /// **An admin command that takes minutes, standing in** — step 10c's
        /// eighth pass on Hyperstack, where the driver install ran 52 seconds.
        /// Nothing in the tests before 2026-09-27, whose commands all answer
        /// at once. Added that day, eighth-pass fix piece 1.
        slow: Option<TheSlowCommand>,
        /// **The machine's graphics driver packages, remembered across runs and
        /// across a restart** — step 10c's eleventh-pass fix piece 10. Where
        /// it is given, every driver command is answered from what stands on
        /// the machine rather than from the script, so a run's installs change
        /// what the next command and the next run meet. **Nothing before
        /// 2026-09-30 gives it**, and those tests are answered exactly as
        /// before.
        drivers: Option<Arc<Mutex<TheDriversOnTheMachine>>>,
    }

    /// **One slow admin command, and what the machine does around it.** Added
    /// 2026-09-27, eighth-pass fix piece 1.
    #[derive(Clone)]
    struct TheSlowCommand {
        /// The command, whole.
        command: String,
        /// How long it runs before it answers.
        how_long: std::time::Duration,
        /// **The script entries it changes when it ends** — the driver check
        /// answering once the driver install has finished.
        after_it: Vec<(String, CommandResult)>,
        /// **The user pressing *Stop* as it starts.**
        the_user_stops_as_it_starts: Option<Arc<Mutex<bool>>>,
        /// **The machine ending the run as it starts**, the way its own window
        /// ends a run nobody refreshes.
        the_machine_ends_the_run_as_it_starts: bool,
        /// **What became of it**: *began*, then *ended* or *its channel was
        /// closed*.
        what_became_of_it: Arc<Mutex<Vec<(&'static str, std::time::Instant)>>>,
    }

    /// **The slow command's channel as the machine sees it**: closed from the
    /// desk where the command is given up before its end.
    struct TheSlowCommandsChannel {
        what_became_of_it: Arc<Mutex<Vec<(&'static str, std::time::Instant)>>>,
        ended: bool,
    }

    impl Drop for TheSlowCommandsChannel {
        fn drop(&mut self) {
            if !self.ended {
                self.what_became_of_it
                    .lock()
                    .expect("the slow command")
                    .push(("its channel was closed", std::time::Instant::now()));
            }
        }
    }

    impl AHeldConnection for TheFarEnd {
        type Channel = ASessionsStream;

        async fn open_a_session(&mut self) -> Result<ASessionsStream, NoSession> {
            use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
            fn copy_of(descriptor: RawFd) -> RawFd {
                let copy = unsafe { libc::dup(descriptor) };
                assert!(copy >= 0, "the descriptor could not be copied");
                copy
            }
            let (ours, theirs) = std::os::unix::net::UnixStream::pair()
                .map_err(|_| NoSession::TheConnectionIsGone)?;
            ours.set_nonblocking(true)
                .map_err(|_| NoSession::TheConnectionIsGone)?;
            let reading = unsafe { std::process::Stdio::from_raw_fd(copy_of(theirs.as_raw_fd())) };
            let writing = unsafe { std::process::Stdio::from_raw_fd(copy_of(theirs.as_raw_fd())) };
            let child = std::process::Command::new(&self.sftp_server)
                .current_dir(&self.at)
                .stdin(reading)
                .stdout(writing)
                .stderr(std::process::Stdio::null())
                .spawn()
                .map_err(|_| NoSession::TheConnectionIsGone)?;
            drop(theirs);
            self.children.lock().expect("the children").push(child);
            let ours = tokio::net::UnixStream::from_std(ours)
                .map_err(|_| NoSession::TheConnectionIsGone)?;
            Ok(ASessionsStream(ours))
        }

        async fn a_command(
            &mut self,
            command: CommandString,
            frame: Option<Vec<u8>>,
        ) -> Result<CommandResult, NoSession> {
            let asked = command.as_str().to_string();
            self.asked.lock().expect("the asks").push(asked.clone());
            {
                let mut held = self.supervisor.lock().expect("the supervisor");
                let standing = held.state.clone();
                let under_way = held.the_elevated_step_started.clone();
                held.the_step_started_when_a_command_arrived
                    .push((asked.clone(), under_way));
                drop(held);
                self.the_state_when_a_command_arrived
                    .lock()
                    .expect("the order")
                    .push((asked.clone(), standing));
            }
            if asked.contains("systemctl reboot") {
                // **The next boot takes the newest kernel**, which is the one
                // the driver install pulled in on machine 1.
                if let Some(drivers) = &self.drivers {
                    drivers.lock().expect("the drivers").the_machine_boots();
                }
                if let Some((mark, seen)) = &self.the_mark_when_the_restart_arrived {
                    *seen.lock().expect("the mark") = Some(mark.where_it_stands());
                }
                *self
                    .written_when_the_restart_arrived
                    .lock()
                    .expect("the order") = Some(
                    self.supervisor
                        .lock()
                        .expect("the supervisor")
                        .written_from_the_desktop
                        .iter()
                        .map(|(boundary, _)| boundary.clone())
                        .collect(),
                );
                // **The Hyperstack shape: the machine goes down with the
                // command still in hand**, so nothing comes back for it.
                if self.the_restart_takes_the_connection {
                    self.goes_down.notify_one();
                    return Err(NoSession::TheConnectionIsGone);
                }
            }
            // **A password carried on an elevating command's standard input**
            // (`Server Setup` E3) is not a framed request: it is recorded, and
            // the command is answered from the script like any other. Added
            // 2026-09-21 with `Server Setup` E2's offer.
            let frame = match frame {
                Some(carried) if asked.starts_with("sudo -S") => {
                    self.supervisor
                        .lock()
                        .expect("the supervisor")
                        .carried_on_standard_input
                        .push((asked.clone(), carried));
                    None
                }
                other => other,
            };
            if let Some(frame) = frame {
                // **The one constant command after hand-over** — every framed
                // request rides it, so the supervisor answers here.
                let answered = self
                    .supervisor
                    .lock()
                    .expect("the supervisor")
                    .answers(&frame);
                return Ok(CommandResult {
                    output: answered,
                    complaint: Vec::new(),
                    status: Some(0),
                });
            }
            if let Some(drivers) = &self.drivers {
                if let Some(answered) = drivers.lock().expect("the drivers").answers(&asked) {
                    return Ok(answered);
                }
            }
            let said = self.said.lock().expect("the script");
            // **The script names each command whole, and the match is the
            // whole of it. Narrowed 2026-09-21.**
            //
            // **It used to answer the first entry whose name was *part of* the
            // command**, and a fragment cannot tell two commands apart once one
            // of them grows. On 2026-09-20 `Server Setup` R1 item 3's probe
            // landed and `dpkg-query -s xz-utils` began arriving before
            // `apt-get … install -y xz-utils`; the ordering test was looking
            // for the first command holding `xz-utils`, it failed at once, and
            // it was narrowed to the install. **Nothing looked for the next
            // one, and this file held it**: `nvidia-smi` is `Server Setup`'s
            // whole driver check and the opening of `Servers`' own per-card
            // query, so one entry was answering both commands and no line of
            // this file said so.
            //
            // **Answering the wrong entry is silent, which is why it is worth
            // narrowing rather than watching for.** With the fragment match and
            // one more entry one package name away, seventeen of this file's
            // twenty-four tests go on passing and the one that fails blames the
            // application for an install the script caused.
            //
            // **A duplicated name is still refused** rather than answered by
            // whichever was written first.
            let matching: Vec<&(String, CommandResult)> = said
                .iter()
                .filter(|(named, _)| *named == asked)
                .collect();
            assert!(
                matching.len() < 2,
                "the script names {asked:?} twice over, and nothing says which was meant"
            );
            let found = matching.first().map(|(_, result)| result.clone());
            Ok(found.unwrap_or(CommandResult {
                output: Vec::new(),
                complaint: Vec::new(),
                status: Some(0),
            }))
        }

        /// **Every command is run whole here, as before, except the slow
        /// one**, which is handed back running — the way the real transport
        /// hands back a command whose request has gone out. Added 2026-09-27.
        async fn a_command_begun(
            &mut self,
            command: CommandString,
            frame: Option<Vec<u8>>,
        ) -> Result<ACommandRunning, NoSession> {
            let asked = command.as_str().to_string();
            let Some(slow) = self.slow.clone().filter(|slow| slow.command == asked) else {
                return self
                    .a_command(command, frame)
                    .await
                    .map(ACommandRunning::already_ended);
            };
            self.asked.lock().expect("the asks").push(asked.clone());
            {
                let mut held = self.supervisor.lock().expect("the supervisor");
                let standing = held.state.clone();
                let under_way = held.the_elevated_step_started.clone();
                held.the_step_started_when_a_command_arrived
                    .push((asked.clone(), under_way));
                drop(held);
                self.the_state_when_a_command_arrived
                    .lock()
                    .expect("the order")
                    .push((asked.clone(), standing));
            }
            slow.what_became_of_it
                .lock()
                .expect("the slow command")
                .push(("began", std::time::Instant::now()));
            if let Some(stop) = &slow.the_user_stops_as_it_starts {
                *stop.lock().expect("the stop") = true;
            }
            if slow.the_machine_ends_the_run_as_it_starts {
                self.supervisor
                    .lock()
                    .expect("the supervisor")
                    .the_run_is_ended_by_its_window();
            }
            let said = Arc::clone(&self.said);
            Ok(ACommandRunning::to_be_run(async move {
                let mut channel = TheSlowCommandsChannel {
                    what_became_of_it: Arc::clone(&slow.what_became_of_it),
                    ended: false,
                };
                tokio::time::sleep(slow.how_long).await;
                {
                    let mut script = said.lock().expect("the script");
                    for (named, result) in &slow.after_it {
                        script.retain(|(one, _)| one != named);
                        script.push((named.clone(), result.clone()));
                    }
                }
                channel.ended = true;
                slow.what_became_of_it
                    .lock()
                    .expect("the slow command")
                    .push(("ended", std::time::Instant::now()));
                Ok(CommandResult {
                    output: Vec::new(),
                    complaint: Vec::new(),
                    status: Some(0),
                })
            }))
        }

        async fn until_it_ends(&mut self) -> Ending {
            self.goes_down.notified().await;
            Ending::Announced
        }

        async fn torn_down(self) -> Ending {
            Ending::TornDownBySmoozy
        }
    }

    impl Drop for TheFarEnd {
        fn drop(&mut self) {
            for mut child in self.children.lock().expect("the children").drain(..) {
                let _ = child.kill();
                let _ = child.wait();
            }
        }
    }

    /// **The machine's supervisor, answering `Server Setup` Y7's three
    /// requests.**
    ///
    /// It is a stand-in and proves nothing about what the machine's own program
    /// does with the same frame; what it proves is the driving — which request
    /// goes when, what each carries, and what the desktop does with each answer.
    #[derive(Default)]
    struct TheSupervisorHere {
        /// The run it opened, and the boundary list the desktop sent it.
        run: Option<String>,
        composed: Vec<String>,
        reached: Vec<String>,
        /// Every step it was commanded, in order.
        steps: Vec<String>,
        /// The Blender version each step carried, where it carried one.
        versions: Vec<Option<String>>,
        /// **Every boundary the desktop wrote, with `Server Setup` Y4a's mark
        /// beside it** — `Server Setup` Y7's elevated-step half.
        written_from_the_desktop: Vec<(String, String)>,
        /// Every boundary write this stand-in refused, with why.
        refused: Vec<String>,
        /// **The run a recovery took over**, where one did — `Server Setup` S4
        /// item 2's take-over, as the machine's own dispatch reads it.
        took_over: Option<String>,
        /// The kind field every run carried, so a recovery carrying one would
        /// be caught rather than passed over.
        kinds: Vec<Option<String>>,
        /// Every run identity it was asked to open, in order.
        opened: Vec<String>,
        /// **`Rented-server runtime` P1's machine-level state**: the operation
        /// identity of the setup run that set it, or nothing.
        state: Option<String>,
        /// Every run that set the state, in order.
        set_by: Vec<String>,
        /// Every run whose clear this machine accepted, in order.
        cleared_by: Vec<String>,
        /// **`Rented-server runtime` W7's role desired state**, where the
        /// desktop wrote one. Nothing means it was never written, which is what
        /// a run with Telegram off must leave.
        the_roles_state: Option<bool>,
        /// **`Server Setup` Y8's stop, as the run's own durable desired state**
        /// — the run it named, where one arrived. Nothing means no stop request
        /// ever left the desk, which is what `Server Setup` Y8a requires while
        /// the machine is down.
        the_stop_was_written_for: Option<String>,
        /// **`Server Setup` R1 item 4's not-found list this machine reports**,
        /// as the last library read found it.
        not_found: Vec<String>,
        /// **Every step it was commanded, with `Server Setup` Y4a's mark where
        /// the desktop sent one** — the two whose act is the desk's.
        marks_on_steps: Vec<(String, Option<String>)>,
        /// **Every step it was commanded, with the manager's package where the
        /// step carried one** — `Server Setup` B15's second route. Step 10c's
        /// owed piece 6.
        packages_on_steps: Vec<(String, Option<String>)>,
        /// **The state as it stood at the instant `Server Setup` Y4's *done*
        /// was written.** That row guarantees *"the state (where set) is
        /// cleared and reconciliation ran"*, so a *done* written while the
        /// state still stands is a false record.
        the_state_when_done_was_written: Option<Option<String>>,
        /// **Every command that carried a value on its standard input**, with
        /// the value — `Server Setup` E3's route, and the only place a
        /// password may travel.
        carried_on_standard_input: Vec<(String, Vec<u8>)>,
        /// **`Server Setup` S2a's desk half, as it arrived**: the run, the
        /// step it named and the words, for every failure the desktop
        /// recorded. Added 2026-09-21.
        failures_recorded: Vec<(String, String, String)>,
        /// **`Server Setup` S6's abandon, as the machine's own dispatch keeps
        /// it**: the open run was marked as an abandon, and the ending its final
        /// verification left, with the words. Added 2026-09-22.
        abandons: bool,
        ending: Option<(String, Option<String>)>,
        /// **The Blender version the machine's record holds for the run**, as
        /// the machine's own program keeps it from 2026-09-22 and hands to a
        /// step whose command carries none. Nothing is a program placed before
        /// that date, which kept none.
        the_recorded_version: Option<String>,
        /// **`Server Setup` S5: how many more recoveries to refuse because the
        /// run that stopped is still running**, and how many were refused so.
        /// Added 2026-09-22.
        the_run_that_stopped_runs_for: usize,
        refused_while_it_ran: usize,
        /// **The ending the stopped run's own record holds**, answered only
        /// when that run is asked for by name — the run, the ending, the step
        /// it names and the words. A recovery's own reading does not carry it,
        /// as the machine's own take-over leaves it on the run it closed.
        /// Added 2026-09-27, fix piece 6.
        the_stopped_run_ended: Option<(String, String, String, String)>,
        /// **`Server Setup` Y7's window**, where this machine keeps one: a run
        /// whose state is not read inside it is ended *ended at its boundary,
        /// not completed*, as the machine's own program ends it. Nothing in the
        /// tests before 2026-09-27. Added that day, eighth-pass fix piece 1.
        window: Option<std::time::Duration>,
        /// **When the open run's state was last read**, which is what the
        /// machine's own program moves on every state reading of the run.
        last_refresh: Option<std::time::Instant>,
        /// **Every moment the open run's state was read**, so a test can say
        /// the run was refreshed while a slow admin command ran.
        refreshed_at: Vec<std::time::Instant>,
        /// **The open run was closed by the machine** — by its window or by a
        /// test — so nothing more is written to it.
        closed: bool,
        /// **A job this machine's supervisor is rendering**, where one is.
        /// Added 2026-09-28, ninth-pass fix piece 3.
        rendering: Option<String>,
        /// **`Server Setup` Y7's second bullet, as the machine's own program
        /// keeps it**: the boundary naming the admin step the desktop recorded
        /// as started, until a step is commanded or a boundary written. Added
        /// 2026-09-29, step 10c's transfer wave piece 12.
        the_elevated_step_started: Option<String>,
        /// **Every command with the admin step recorded as started when it
        /// reached the machine**, so a test can say the start went before the
        /// act.
        the_step_started_when_a_command_arrived: Vec<(String, Option<String>)>,
        /// **Every time the supervisor was restarted, with the job it was
        /// rendering at that instant** — which is what a placement's commit
        /// does to a real machine, and what the ninth pass on Hyperstack saw
        /// happen under *p12h9 long*. Added 2026-09-28.
        restarted_with: Vec<Option<String>>,
        /// **The 38's piece 14b: `Server Setup` B7 and `Server Setup` B7a as the machine's
        /// own program keeps them** — the run's staging area its state reading
        /// carries, the version a user's build answers below 4.5 where it
        /// holds, the question standing, the discard, and every user's archive
        /// and answer a step carried, with the step.
        staging_area: Option<String>,
        holds_at: Option<String>,
        question: Option<String>,
        discard: Option<String>,
        /// **`Server Setup` Y7b and `Server Setup` Y7c, as the machine's own program answers
        /// them**: the count its state reading carries while that step is the
        /// one last commanded — the step, the bytes so far and the whole.
        /// Nothing in the tests before 2026-10-18. Added that day, the
        /// Creator's fourteenth notes, piece U.
        the_counts: Vec<(String, u64, Option<u64>)>,
        archives: Vec<(String, Option<String>)>,
        answers: Vec<String>,
        /// **`Server Setup` B1's answer, as the machine's record carries it for
        /// a user's build** — the version `holds_at` names, or *4.6.0* where the
        /// fake build answers at or above 4.5. Step 10c's owed piece 5,
        /// 2026-10-16.
        version_answered: Option<String>,
        /// **`Server Setup` R1b: how many jobs this machine's queue reading
        /// holds waiting, at each reading in turn** — and none once the list
        /// is spent. Added 2026-10-12.
        waiting_at_each_queue_reading: Vec<usize>,
        /// **`Server Setup` R1c: how many jobs held for the driver every
        /// reading holds, after the waiting ones** — each under a hold of its
        /// own, with `Rendering` S18a's reason. Added 2026-10-12, held-job
        /// piece A.
        held_for_the_driver: usize,
        /// **How many queue readings the desktop took.**
        queue_readings: usize,
        /// **The machine's own refusal of `Rented-server runtime` W8's
        /// *enabled***, where this machine refuses it: the words it answers
        /// with, and no word about the role worker. Added 2026-10-18, the
        /// Creator's twelfth notes, note 2.
        the_role_is_refused_with: Option<&'static str>,
        /// **`Server Setup` Y7a item 2, as the machine's own program answers
        /// it**: each worker step's two streams — the step, the ordinary and
        /// the complaints — carried by the state reading with their step while
        /// that step is the one last commanded and no admin step has started
        /// since. Nothing in the tests before 2026-10-24.
        the_steps_words: Vec<(String, String, String)>,
        /// **Every frame this supervisor was handed**, whole, so a test can
        /// say what never rode one. Added 2026-10-24.
        frames: Vec<Vec<u8>>,
        /// **`Server Setup` Y7a item 2 as changed on 2026-10-24, the way the
        /// machine's own program answers it from piece 2b on: a worker step
        /// that runs across several readings**, its words printed a piece at
        /// a time — the step, then what it prints by each reading in turn, as
        /// (ordinary, complaints). The step stays in flight until every piece
        /// has been printed, and reaches its boundary at the reading after.
        /// Added 2026-10-25, the eighteenth notes, piece 2c.
        words_while_it_runs: Vec<(String, Vec<(String, String)>)>,
        /// **The most of each stream one reading carries**, where the test
        /// wants the words to come in pieces (`Server Setup` Y7a item 5).
        the_room_on_a_reading: Option<usize>,
        /// **The step running across readings now**, and what of it is
        /// printed.
        running: Option<AStepRunningAcrossReadings>,
        /// **Every state reading while such a step ran or held its words**:
        /// what the desktop named — the turn and its two lengths — whether the
        /// step was in flight, and when.
        readings_named: Vec<(Option<String>, Option<String>, Option<String>, bool, std::time::Instant)>,
        /// **A worker step whose lines print on the machine's own clock**, as
        /// the machine's own program prints its download lines once a second
        /// (`Server Setup` Y7b) whether or not anyone reads them: the step,
        /// each line with how long after the step's command it prints, and how
        /// long after the command the step reaches its boundary. Added
        /// 2026-10-26, the worldwide pass's setup faults, fault 1.
        words_on_a_clock: Option<(String, Vec<(std::time::Duration, String)>, std::time::Duration)>,
        /// **That step, running now**: its turn, when it was commanded, and
        /// whether it has reached its boundary.
        on_the_clock: Option<(String, String, std::time::Instant, bool)>,
        /// **Every request this supervisor answered, by name, and when** — so
        /// a test can count what crossed the line and when.
        answered_at: Vec<(String, std::time::Instant)>,
        /// **Each boundary reached carries its mark, its moment and its clock
        /// on the state reading**, as the machine's own program answers it —
        /// so the record the screen is told reads each step as done. Nothing
        /// in the tests before 2026-10-26; that day's fault 4 wants it.
        the_record_carries_its_moments: bool,
    }

    /// **A worker step running across several readings** in the stand-in.
    struct AStepRunningAcrossReadings {
        step: String,
        turn: String,
        pieces: Vec<(String, String)>,
        printed: usize,
        ended: bool,
    }

    impl TheSupervisorHere {
        /// **`Server Setup` Y7's ending, written the way the machine's own
        /// program writes it** — no words, the run no longer open, the
        /// machine-level state standing.
        fn the_run_is_ended_by_its_window(&mut self) {
            if self.run.is_none() || self.closed {
                return;
            }
            self.ending = Some((asked::IT_ENDED_AT_ITS_BOUNDARY_NOT_COMPLETED.to_string(), None));
            self.closed = true;
        }

        /// **The window, looked after before each request is answered** — the
        /// machine's own program looks after it on its own clock, and a request
        /// arriving after the window elapsed finds the run already ended.
        fn the_window_is_looked_after(&mut self) {
            if let (Some(window), Some(last)) = (self.window, self.last_refresh) {
                if last.elapsed() > window {
                    self.the_run_is_ended_by_its_window();
                }
            }
        }

        /// **The machine's own refusal of a write to a run it no longer holds
        /// open**, in its own words.
        fn no_run_is_open(request: &smoozy_server_protocol::framing::Request) -> Vec<u8> {
            write_result_frame(
                &Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "0.1.0").carrying(
                    Fields::none()
                        .and(asked::WHAT_BECAME_OF_IT_FIELD, asked::THERE_IS_NO_RECORD_OF_IT)
                        .and(
                            asked::THE_MACHINES_OWN_WORDS_FIELD,
                            "no setup run is open on this machine",
                        ),
                ),
            )
        }

        fn answers(&mut self, frame: &[u8]) -> Vec<u8> {
            self.frames.push(frame.to_vec());
            let mut reading = frame;
            let request =
                read_one_request(&mut reading).expect("the desktop framed a request");
            self.answered_at.push((request.name.clone(), std::time::Instant::now()));
            if std::env::var_os("SMOOZY_DESK_TELLS_EACH_ASK").is_some() {
                eprintln!("{:?} asked {}", std::time::SystemTime::now(), request.name);
            }
            self.the_window_is_looked_after();
            let writes_to_the_run = [
                asked::THE_ONE_SETUP_STEP_REQUEST,
                asked::THE_BOUNDARY_IS_WRITTEN_REQUEST,
                asked::THE_FAILED_STEP_IS_RECORDED_REQUEST,
                "an elevated step's start is recorded",
            ];
            if self.closed && writes_to_the_run.contains(&request.name.as_str()) {
                return Self::no_run_is_open(&request);
            }
            let mut fields = Fields::none();
            match request.name.as_str() {
                name if name == asked::THE_SETUP_RUN_IS_OPENED_REQUEST => {
                    if let Some(area) = &self.staging_area {
                        std::fs::create_dir_all(area).expect("the setup run makes its staging area");
                    }
                    self.run = Some(request.identity.as_text());
                    self.opened.push(request.identity.as_text());
                    self.kinds.push(
                        request
                            .fields
                            .text(asked::THE_KIND_OF_SETUP_FIELD)
                            .map(str::to_string),
                    );
                    // **`Server Setup` S4 item 2's take-over, as the machine's
                    // own dispatch performs it**: the recovery inherits the
                    // taken-over run's composed list and its last-reached
                    // marker, written into the recovery's own record, and
                    // **nothing of it is composed afresh**. So this stand-in
                    // keeps what it already holds and records that a recovery
                    // took the run over.
                    if let Some(taken) = request.fields.text(asked::THE_RUN_TAKEN_OVER_FIELD) {
                        // **`Server Setup` S5, as the machine's own program
                        // answers it from 2026-09-22**: refused for now, with
                        // nothing taken over, and said in a field of its own.
                        if self.the_run_that_stopped_runs_for > 0 {
                            self.the_run_that_stopped_runs_for -= 1;
                            self.refused_while_it_ran += 1;
                            self.opened.pop();
                            self.kinds.pop();
                            fields = fields
                                .and(asked::WHAT_BECAME_OF_IT_FIELD, asked::THERE_IS_NO_RECORD_OF_IT)
                                .and(
                                    asked::THE_MACHINES_OWN_WORDS_FIELD,
                                    "the setup run that stopped is still running on this machine",
                                )
                                .and(asked::THE_RUN_THAT_STOPPED_IS_STILL_RUNNING_FIELD, taken);
                            let answered = Answered::to(
                                &request,
                                THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
                                "0.1.0",
                            )
                            .carrying(fields);
                            return write_result_frame(&answered);
                        }
                        self.took_over = Some(taken.to_string());
                        self.abandons = request.fields.text(asked::THE_RECOVERY_ABANDONS_FIELD)
                            == Some(asked::THE_RECOVERY_ABANDONS);
                        // **`Server Setup` S4 item 2's other half: the standing
                        // state is re-pointed at the recovery run's own
                        // operation identity, in the take-over's own act and
                        // never cleared in between.** Without it
                        // `Rented-server runtime` A11a would refuse the very
                        // worker the recovery needs, and the recovery could
                        // never clear a state naming somebody else.
                        if self.state.is_some() {
                            self.state = Some(request.identity.as_text());
                            self.set_by.push(request.identity.as_text());
                        }
                        fields = fields
                            .and(asked::WHAT_BECAME_OF_IT_FIELD, asked::IT_IS_STILL_IN_HAND)
                            .and(asked::THE_SETUP_RUN_FIELD, request.identity.as_text());
                        // **The abandon said back**, as the machine's own
                        // program does from 2026-09-22.
                        if self.abandons {
                            fields = fields.and(
                                asked::THE_RECOVERY_ABANDONS_FIELD,
                                asked::THE_RECOVERY_ABANDONS,
                            );
                        }
                        let answered = Answered::to(
                            &request,
                            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
                            "0.1.0",
                        )
                        .carrying(fields);
                        return write_result_frame(&answered);
                    }
                    // **`Rented-server runtime` A5b under `Rented-server runtime` A11, as the machine's
                    // own program answers it since 2026-09-21**: a fresh run is
                    // refused while the machine-level state stands, in words,
                    // with the run the state names in V9's own field.
                    if let Some(standing) = self.state.clone() {
                        self.opened.pop();
                        self.kinds.pop();
                        self.run = None;
                        fields = fields
                            .and(asked::WHAT_BECAME_OF_IT_FIELD, asked::THERE_IS_NO_RECORD_OF_IT)
                            .and(
                                asked::THE_MACHINES_OWN_WORDS_FIELD,
                                "the setup worker was not admitted: a setup of this server did \
                                 not finish, and nothing else runs until it does",
                            )
                            .and(asked::THE_SETUP_IN_PROGRESS_FIELD, standing);
                        let answered = Answered::to(
                            &request,
                            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
                            "0.1.0",
                        )
                        .carrying(fields);
                        return write_result_frame(&answered);
                    }
                    let how_many: usize = request
                        .fields
                        .text(asked::HOW_MANY_COMPOSED_BOUNDARIES_FIELD)
                        .and_then(|text| text.parse().ok())
                        .unwrap_or(0);
                    for at in 0..how_many {
                        if let Some(named) = request.fields.text(&asked::the_field_of_item(
                            asked::A_COMPOSED_BOUNDARY,
                            at,
                            asked::THE_BOUNDARY_FIELD,
                        )) {
                            self.composed.push(named.to_string());
                        }
                    }
                    // **`Server Setup` Y4's *started* boundary is written with
                    // the record**, which is what the machine's own program
                    // does.
                    self.reached.push(asked::THE_BOUNDARY_STARTED.to_string());
                    // **The window runs from the run's opening**, as the
                    // machine's own program starts it.
                    self.last_refresh = Some(std::time::Instant::now());
                    self.closed = false;
                    fields = fields
                        .and(asked::WHAT_BECAME_OF_IT_FIELD, asked::IT_IS_STILL_IN_HAND)
                        .and(
                            asked::THE_SETUP_RUN_FIELD,
                            request.identity.as_text(),
                        );
                }
                name if name == asked::THE_ONE_SETUP_STEP_REQUEST => {
                    // **A step commanded or a boundary written ends the admin
                    // step under way**, as the machine's own program clears it.
                    self.the_elevated_step_started = None;
                    let step = request
                        .fields
                        .text(asked::THE_STEP_FIELD)
                        .unwrap_or_default()
                        .to_string();
                    self.versions.push(
                        request
                            .fields
                            .text(asked::THE_BLENDER_VERSION_FIELD)
                            .map(str::to_string),
                    );
                    self.marks_on_steps.push((
                        step.clone(),
                        request
                            .fields
                            .text(asked::WHAT_THE_STEP_DID_FIELD)
                            .map(str::to_string),
                    ));
                    self.packages_on_steps.push((
                        step.clone(),
                        request
                            .fields
                            .text(asked::THE_MANAGERS_PACKAGE_FIELD)
                            .map(str::to_string),
                    ));
                    // **`Server Setup` S6: an abandon runs its final
                    // verification and nothing else**, and the verification's
                    // report ends it *abandoned* — here with a finding that did
                    // not pass, which is the third pass's machine.
                    if self.abandons {
                        self.steps.push(step.clone());
                        if step != asked::THE_FINAL_VERIFICATION_STEP {
                            fields = fields
                                .and(asked::WHAT_BECAME_OF_IT_FIELD, asked::THERE_IS_NO_RECORD_OF_IT)
                                .and(asked::THE_MACHINES_OWN_WORDS_FIELD, "an abandon runs only its final verification");
                        } else {
                            self.ending = Some((
                                asked::IT_WAS_ABANDONED.to_string(),
                                Some("the driver did not answer".to_string()),
                            ));
                            fields = fields
                                .and(asked::WHAT_BECAME_OF_IT_FIELD, asked::IT_IS_STILL_IN_HAND)
                                .and(asked::THE_STEP_FIELD, step);
                        }
                        let answered = Answered::to(
                            &request,
                            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
                            "0.1.0",
                        )
                        .carrying(fields);
                        return write_result_frame(&answered);
                    }
                    // **`Server Setup` B7: the user's own archive, as the
                    // machine's own worker reads it** — at
                    // `<staging>/<run>-<name>`, or the unpack fails; and the
                    // publication held below 4.5 until the user's answer.
                    // The 38's piece 14b.
                    let archive = request
                        .fields
                        .text(asked::THE_USERS_OWN_ARCHIVE_FIELD)
                        .map(str::to_string);
                    self.archives.push((step.clone(), archive.clone()));
                    let answer = request.fields.text(asked::THE_USERS_ANSWER_FIELD).map(str::to_string);
                    if let Some(answer) = &answer {
                        self.answers.push(answer.clone());
                    }
                    if let Some(name) = &archive {
                        let held_back = |this: &mut Self, fields: Fields, step: String, words: Option<String>| {
                            this.steps.push(step.clone());
                            if let Some(words) = words {
                                this.ending = Some((asked::A_STEP_FAILED.to_string(), Some(words)));
                            }
                            let answered = Answered::to(&request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "0.1.0")
                                .carrying(
                                    fields
                                        .and(asked::WHAT_BECAME_OF_IT_FIELD, asked::IT_IS_STILL_IN_HAND)
                                        .and(asked::THE_STEP_FIELD, step),
                                );
                            write_result_frame(&answered)
                        };
                        if step == asked::THE_UNPACK_STEP {
                            let at = std::path::Path::new(self.staging_area.as_deref().unwrap_or_default())
                                .join(format!("{}-{name}", self.run.clone().unwrap_or_default()));
                            if !at.is_file() {
                                let words = format!("no archive at {}", at.display());
                                return held_back(self, fields, step, Some(words));
                            }
                        }
                        if step == asked::THE_BUILD_IS_PUBLISHED_STEP {
                            self.version_answered =
                                Some(self.holds_at.clone().unwrap_or_else(|| "4.6.0".to_string()));
                            match (answer.as_deref(), self.holds_at.clone()) {
                                (None, Some(version)) => {
                                    self.question = Some(version);
                                    return held_back(self, fields, step, None);
                                }
                                (Some(asked::THE_USER_DECLINED), _) => {
                                    self.question = None;
                                    self.discard = Some("the user declined".to_string());
                                    self.composed.retain(|it| it != asked::THE_BOUNDARY_BLENDER);
                                    return held_back(self, fields, step, None);
                                }
                                _ => self.question = None,
                            }
                        }
                    }
                    // **A step that reads the version and was given none
                    // fails the run, as the machine's own worker does** — the
                    // third pass on Vast's *"the unpack was commanded with no
                    // version to name its archive"*. The version is the
                    // command's, or the one the machine's record holds for the
                    // run. Added 2026-09-22.
                    let version = request
                        .fields
                        .text(asked::THE_BLENDER_VERSION_FIELD)
                        .map(str::to_string)
                        .or_else(|| self.the_recorded_version.clone());
                    if asked::THE_STEPS_THAT_READ_THE_BLENDER_VERSION.contains(&step.as_str())
                        && version.is_none()
                        && archive.is_none()
                    {
                        self.steps.push(step.clone());
                        self.ending = Some((
                            asked::A_STEP_FAILED.to_string(),
                            Some(format!("{step} was commanded with no version")),
                        ));
                        fields = fields
                            .and(asked::WHAT_BECAME_OF_IT_FIELD, asked::IT_IS_STILL_IN_HAND)
                            .and(asked::THE_STEP_FIELD, step);
                        let answered = Answered::to(
                            &request,
                            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
                            "0.1.0",
                        )
                        .carrying(fields);
                        return write_result_frame(&answered);
                    }
                    // **A step that runs across readings** reaches its
                    // boundary only once it has printed everything (piece 2c).
                    self.running = self
                        .words_while_it_runs
                        .iter()
                        .find(|(listed, _)| *listed == step)
                        .map(|(_, pieces)| AStepRunningAcrossReadings {
                            step: step.clone(),
                            turn: request.identity.as_text(),
                            pieces: pieces.clone(),
                            printed: 0,
                            ended: false,
                        });
                    // **A step whose lines print on the machine's own clock**
                    // (2026-10-26, fault 1).
                    if self.words_on_a_clock.as_ref().is_some_and(|(listed, ..)| *listed == step) {
                        self.on_the_clock =
                            Some((step.clone(), request.identity.as_text(), std::time::Instant::now(), false));
                    }
                    // The worker reaches the step's boundary before the
                    // desktop's next reading, which is the ordinary case.
                    let on_the_clock_now = self.on_the_clock.as_ref().is_some_and(|(on, ..)| *on == step);
                    if self.running.is_none() && !on_the_clock_now {
                        if let Some(boundary) = the_boundary_a_step_feeds(&step) {
                            self.reached.push(boundary.to_string());
                        }
                    }
                    self.steps.push(step.clone());
                    fields = fields
                        .and(asked::WHAT_BECAME_OF_IT_FIELD, asked::IT_IS_STILL_IN_HAND)
                        .and(asked::THE_STEP_FIELD, step);
                }
                // **`Server Setup` Y7's boundary, written from the
                // desktop.** The stand-in keeps the clause's own guards: the
                // mark is `Server Setup` Y4a's closed set of two, the boundary
                // is in the composed list, and it is after the last one
                // reached.
                name if name == asked::THE_BOUNDARY_IS_WRITTEN_REQUEST => {
                    // **A step commanded or a boundary written ends the admin
                    // step under way**, as the machine's own program clears it.
                    self.the_elevated_step_started = None;
                    let boundary = request
                        .fields
                        .text(asked::THE_BOUNDARY_FIELD)
                        .unwrap_or_default()
                        .to_string();
                    let mark = request
                        .fields
                        .text(asked::WHAT_THE_STEP_DID_FIELD)
                        .unwrap_or_default()
                        .to_string();
                    let known = asked::EVERY_SETUP_BOUNDARY.contains(&boundary.as_str());
                    let marked = mark == asked::IT_PERFORMED || mark == asked::IT_HAD_NOTHING_TO_DO;
                    let composed = self.composed.contains(&boundary);
                    let after = match (
                        self.reached.last(),
                        asked::EVERY_SETUP_BOUNDARY
                            .iter()
                            .position(|named| *named == boundary),
                    ) {
                        (Some(last), Some(at)) => asked::EVERY_SETUP_BOUNDARY
                            .iter()
                            .position(|named| named == last)
                            .is_some_and(|was| at > was),
                        _ => true,
                    };
                    if boundary == asked::THE_BOUNDARY_DONE {
                        self.the_state_when_done_was_written = Some(self.state.clone());
                    }
                    // **`Server Setup` Y3, as the machine's own program keeps
                    // it from 2026-09-22**: no boundary is written while one
                    // composed before it is unreached.
                    let at = |named: &str| {
                        asked::EVERY_SETUP_BOUNDARY.iter().position(|it| *it == named)
                    };
                    let nothing_skipped = !self.composed.iter().any(|earlier| {
                        at(earlier) < at(&boundary) && !self.reached.contains(earlier)
                    });
                    if known && marked && composed && after && nothing_skipped {
                        self.reached.push(boundary.clone());
                        self.written_from_the_desktop.push((boundary, mark));
                        fields = fields.and(
                            asked::WHAT_BECAME_OF_IT_FIELD,
                            asked::IT_HAS_A_RECORDED_OUTCOME,
                        );
                    } else {
                        self.refused.push(format!("{boundary}: {mark}"));
                        fields = fields
                            .and(
                                asked::WHAT_BECAME_OF_IT_FIELD,
                                asked::THERE_IS_NO_RECORD_OF_IT,
                            )
                            .and(
                                asked::THE_MACHINES_OWN_WORDS_FIELD,
                                "that boundary is not this run's to write next",
                            );
                    }
                }
                // **`Rented-server runtime` P3's two acts, as the machine's own
                // dispatch answers them.** P1 makes the state name the run that
                // set it; P8 refuses a clear by any other run and reports the
                // standing one.
                name if name == asked::THE_SETUP_IN_PROGRESS_IS_SET_REQUEST => {
                    let run = request
                        .fields
                        .text(asked::THE_SETUP_RUN_FIELD)
                        .unwrap_or_default()
                        .to_string();
                    self.state = Some(run.clone());
                    self.set_by.push(run.clone());
                    fields = fields.and(asked::THE_SETUP_RUN_FIELD, run);
                }
                name if name == asked::THE_SETUP_IN_PROGRESS_IS_CLEARED_REQUEST => {
                    let run = request
                        .fields
                        .text(asked::THE_SETUP_RUN_FIELD)
                        .unwrap_or_default()
                        .to_string();
                    match &self.state {
                        Some(standing) if *standing != run => {
                            fields =
                                fields.and(asked::THE_SETUP_RUN_FIELD, standing.clone());
                        }
                        _ => {
                            self.state = None;
                            self.cleared_by.push(run.clone());
                            fields = fields
                                .and(asked::THE_SETUP_RUN_THE_CLEAR_REMOVED_FIELD, run);
                        }
                    }
                }
                // **`Rented-server runtime` W8's act, as the machine's own
                // dispatch answers it**: the role's durable desired state is
                // written and the supervisor says what it did with the role
                // worker (`Rented-server runtime` W9).
                // **`Server Setup` Y8's stop on `Rented-server runtime` X8's
                // path.** The stand-in writes the desired state and sends no
                // signal, as the machine's own dispatch does.
                name if name == asked::THE_SETUP_RUN_IS_STOPPED_REQUEST => {
                    let run = request
                        .fields
                        .text(asked::THE_SETUP_RUN_FIELD)
                        .unwrap_or_default()
                        .to_string();
                    self.the_stop_was_written_for = Some(run.clone());
                    fields = fields
                        .and(
                            asked::WHAT_BECAME_OF_IT_FIELD,
                            asked::IT_HAS_A_RECORDED_OUTCOME,
                        )
                        .and(asked::THE_SETUP_RUN_FIELD, run);
                }
                // **`Server Setup` S2a's desk half on `Rented-server runtime`
                // X8's path.** The stand-in keeps what arrived and answers as
                // the machine's own dispatch does for a run it holds open.
                name if name == asked::THE_FAILED_STEP_IS_RECORDED_REQUEST => {
                    let field = |named: &str| {
                        request.fields.text(named).unwrap_or_default().to_string()
                    };
                    self.failures_recorded.push((
                        field(asked::THE_SETUP_RUN_FIELD),
                        field(asked::THE_FAILED_STEP_FIELD),
                        field(asked::THE_MACHINES_OWN_WORDS_FIELD),
                    ));
                    // **The run is ended by it**, as the machine's own
                    // dispatch ends it in the same durable act. Added
                    // 2026-09-26.
                    self.ending = Some((
                        asked::A_STEP_FAILED.to_string(),
                        Some(field(asked::THE_MACHINES_OWN_WORDS_FIELD)),
                    ));
                    fields = fields.and(
                        asked::WHAT_BECAME_OF_IT_FIELD,
                        asked::IT_HAS_A_RECORDED_OUTCOME,
                    );
                }
                // **`Server Setup` Y7's second bullet: the desktop records an
                // admin step's start against the run**, and the machine's own
                // program writes it as the step under way. Added 2026-09-29.
                name if name == "an elevated step's start is recorded" => {
                    self.the_elevated_step_started = request
                        .fields
                        .text(asked::THE_BOUNDARY_FIELD)
                        .map(str::to_string);
                    fields = fields.and(
                        asked::WHAT_BECAME_OF_IT_FIELD,
                        asked::IT_HAS_A_RECORDED_OUTCOME,
                    );
                }
                name if name == asked::THE_TELEGRAM_ROLE_IS_ENABLED_REQUEST => {
                    match self.the_role_is_refused_with {
                        Some(words) => {
                            fields = fields.and(asked::THE_MACHINES_OWN_WORDS_FIELD, words);
                        }
                        None => {
                            self.the_roles_state = Some(true);
                            fields = fields.and(asked::WHAT_THE_ROLE_WORKER_DID_FIELD, "started");
                        }
                    }
                }
                // **`Queue & History` A1's queue reading**, holding as many
                // waiting jobs as the test set for this reading. `Server Setup`
                // `Server Setup` R1b reads it before the swap and again once the state stands.
                // Added 2026-10-12.
                name if name == asked::THE_QUEUE_READING_REQUEST => {
                    let waiting = if self.waiting_at_each_queue_reading.is_empty() {
                        0
                    } else {
                        self.waiting_at_each_queue_reading.remove(0)
                    };
                    self.queue_readings += 1;
                    fields = fields.and(
                        asked::HOW_MANY_JOBS_FIELD,
                        (waiting + self.held_for_the_driver).to_string(),
                    );
                    for at in 0..waiting {
                        let job = Identity::issue(IdentityKind::Job).expect("a job");
                        fields = fields
                            .and(
                                &asked::the_field_of_item(asked::THE_JOB_FIELD, at, asked::THE_JOB_FIELD),
                                job.as_text(),
                            )
                            .and(
                                &asked::the_field_of_item(asked::THE_JOB_FIELD, at, asked::THE_QUEUE_STATE_FIELD),
                                asked::IT_IS_WAITING,
                            );
                    }
                    for at in waiting..waiting + self.held_for_the_driver {
                        let job = Identity::issue(IdentityKind::Job).expect("a job");
                        let hold = Identity::issue(IdentityKind::Operation).expect("a hold");
                        let item = |name: &str| asked::the_field_of_item(asked::THE_JOB_FIELD, at, name);
                        fields = fields
                            .and(&item(asked::THE_JOB_FIELD), job.as_text())
                            .and(&item(asked::THE_QUEUE_STATE_FIELD), asked::IT_IS_HELD)
                            .and(&item(asked::THE_HOLD_IT_IS_HELD_UNDER_FIELD), hold.as_text())
                            .and(&item(asked::THE_HOLD_REASON_FIELD), smoozy_rendering::submission::THE_DRIVER);
                    }
                }
                // **`Rented-server runtime` V9: the health answer names the run
                // the machine-level state stands for.** Added 2026-09-22 for
                // the abandon, which reads the run it takes over from here.
                name if name == asked::THE_HEALTH_ANSWER_REQUEST => {
                    if let Some(standing) = &self.state {
                        fields = fields.and(asked::THE_SETUP_IN_PROGRESS_FIELD, standing.clone());
                    }
                }
                name if name == asked::THE_SET_UP_CARDS_READING_REQUEST => {
                    if let Some(run) = &self.run {
                        fields = fields.and(asked::THE_READY_ACTIONS_RUN_FIELD, run.clone());
                    }
                }
                name if name == asked::THE_SETUP_RUNS_STATE_REQUEST => {
                    let the_run_asked = request
                        .fields
                        .text(asked::THE_SETUP_RUN_FIELD)
                        .map(str::to_string);
                    // **`Server Setup` Y7's refresh is the run's own state
                    // reading**, and the machine's own program moves the window
                    // on every one that names the open run.
                    if !self.closed && the_run_asked.is_some() && the_run_asked == self.run {
                        self.last_refresh = Some(std::time::Instant::now());
                        self.refreshed_at.push(std::time::Instant::now());
                    }
                    let its_own_ending = self
                        .the_stopped_run_ended
                        .clone()
                        .filter(|(run, ..)| Some(run) == the_run_asked.as_ref());
                    if let Some((_, ending, step, words)) = its_own_ending {
                        fields = fields
                            .and(asked::THE_RUNS_ENDING_FIELD, ending)
                            .and(asked::THE_MACHINES_OWN_WORDS_FIELD, words)
                            .and(asked::THE_FAILED_STEP_FIELD, step);
                    } else if let Some((ending, words)) = &self.ending {
                        fields = fields.and(asked::THE_RUNS_ENDING_FIELD, ending.clone());
                        if let Some(words) = words {
                            fields = fields.and(asked::THE_MACHINES_OWN_WORDS_FIELD, words.clone());
                        }
                        // **`Server Setup` S2a: a failed run's ending names the
                        // step**, where the desk recorded one.
                        if let (true, Some((_, step, _))) =
                            (ending == asked::A_STEP_FAILED, self.failures_recorded.last())
                        {
                            fields = fields.and(asked::THE_FAILED_STEP_FIELD, step.clone());
                        }
                    }
                    if let Some(area) = &self.staging_area {
                        fields = fields.and(asked::THE_SETUP_RUNS_STAGING_AREA_FIELD, area.clone());
                    }
                    if let Some(version) = &self.question {
                        fields = fields.and(asked::THE_QUESTION_STANDS_AT_VERSION_FIELD, version.clone());
                    }
                    if let Some(discard) = &self.discard {
                        fields = fields.and(asked::THE_DISCARD_FIELD, discard.clone());
                    }
                    if let Some(version) = &self.version_answered {
                        fields = fields.and(asked::THE_VERSION_ANSWERED_FIELD, version.clone());
                    }
                    // **A step whose lines print on the machine's own clock:
                    // every line whose moment has come, the step in flight
                    // until its end, and the words after the lengths the
                    // desktop names** (2026-10-26, fault 1).
                    let on_the_clock = self
                        .on_the_clock
                        .clone()
                        .filter(|(step, ..)| self.steps.last() == Some(step));
                    if let (Some((step, turn, began, ended)), Some((_, lines, ends_after))) =
                        (on_the_clock, self.words_on_a_clock.clone())
                    {
                        let now = began.elapsed();
                        if !ended && now >= ends_after {
                            self.on_the_clock = Some((step.clone(), turn.clone(), began, true));
                            if let Some(boundary) = the_boundary_a_step_feeds(&step) {
                                self.reached.push(boundary.to_string());
                            }
                        }
                        let ended = ended || now >= ends_after;
                        let named = |field: &str| request.fields.text(field).map(str::to_string);
                        let printed: String = lines
                            .iter()
                            .filter(|(at, _)| *at <= now)
                            .map(|(_, line)| format!("{line}\n"))
                            .collect();
                        let from = if named(asked::THE_STEPS_WORDS_TURN_FIELD).as_deref() == Some(turn.as_str()) {
                            named(asked::THE_ORDINARY_WORDS_HELD_FIELD)
                                .and_then(|length| length.parse::<usize>().ok())
                                .unwrap_or(0)
                        } else {
                            0
                        };
                        if !ended {
                            fields = fields.and(asked::THE_STEP_FIELD, step.clone());
                        }
                        // **The count, as the machine's own program tells it
                        // once a second while the download runs**
                        // (`Server Setup` Y7b): a whole of 383 MB, so far in
                        // proportion to the clock.
                        let whole: u64 = 383_000_000;
                        let so_far = (whole as f64 * (now.as_secs_f64() / 8.2).min(1.0)) as u64;
                        fields = fields
                            .and(asked::THE_BYTES_SO_FAR_FIELD, so_far.to_string())
                            .and(asked::THE_BYTES_IN_THE_WHOLE_FIELD, whole.to_string());
                        fields = fields
                            .and(asked::THE_STEP_THE_WORDS_BELONG_TO_FIELD, step)
                            .and(asked::THE_STEPS_WORDS_TURN_FIELD, turn)
                            .and(asked::THE_STEPS_ORDINARY_WORDS_FIELD, printed[from.min(printed.len())..].to_string())
                            .and(asked::THE_STEPS_COMPLAINT_WORDS_FIELD, String::new());
                    } else if let Some(running) = self
                        .running
                        .as_mut()
                        .filter(|running| self.steps.last() == Some(&running.step))
                    {
                        if !running.ended {
                            if running.printed < running.pieces.len() {
                                running.printed += 1;
                            } else {
                                running.ended = true;
                                if let Some(boundary) = the_boundary_a_step_feeds(&running.step) {
                                    self.reached.push(boundary.to_string());
                                }
                            }
                        }
                        let named = |field: &str| request.fields.text(field).map(str::to_string);
                        let turn = named(asked::THE_STEPS_WORDS_TURN_FIELD);
                        self.readings_named.push((
                            turn.clone(),
                            named(asked::THE_ORDINARY_WORDS_HELD_FIELD),
                            named(asked::THE_COMPLAINT_WORDS_HELD_FIELD),
                            !running.ended,
                            std::time::Instant::now(),
                        ));
                        let (ordinary, complaints): (String, String) = running.pieces[..running.printed]
                            .iter()
                            .fold((String::new(), String::new()), |(o, c), (po, pc)| (o + po, c + pc));
                        let after = |held: &str, field: &str| {
                            let from = if turn.as_deref() == Some(running.turn.as_str()) {
                                named(field).and_then(|length| length.parse::<usize>().ok()).unwrap_or(0)
                            } else {
                                0
                            };
                            held[from.min(held.len())..].to_string()
                        };
                        let mut ordinary = after(&ordinary, asked::THE_ORDINARY_WORDS_HELD_FIELD);
                        let mut complaints = after(&complaints, asked::THE_COMPLAINT_WORDS_HELD_FIELD);
                        let mut more = false;
                        if let Some(room) = self.the_room_on_a_reading {
                            for words in [&mut ordinary, &mut complaints] {
                                if words.len() > room {
                                    words.truncate(room);
                                    more = true;
                                }
                            }
                        }
                        if !running.ended {
                            fields = fields.and(asked::THE_STEP_FIELD, running.step.clone());
                        }
                        fields = fields
                            .and(asked::THE_STEP_THE_WORDS_BELONG_TO_FIELD, running.step.clone())
                            .and(asked::THE_STEPS_WORDS_TURN_FIELD, running.turn.clone())
                            .and(asked::THE_STEPS_ORDINARY_WORDS_FIELD, ordinary)
                            .and(asked::THE_STEPS_COMPLAINT_WORDS_FIELD, complaints);
                        if more {
                            fields = fields.and(asked::MORE_OF_THE_STEPS_WORDS_FIELD, asked::THEY_FOLLOW);
                        }
                    } else if let Some((step, ordinary, complaints)) = self
                        .steps
                        .last()
                        .filter(|_| self.the_elevated_step_started.is_none())
                        .and_then(|last| self.the_steps_words.iter().find(|(step, ..)| step == last))
                    {
                        fields = fields
                            .and(asked::THE_STEP_THE_WORDS_BELONG_TO_FIELD, step.clone())
                            .and(asked::THE_STEPS_ORDINARY_WORDS_FIELD, ordinary.clone())
                            .and(asked::THE_STEPS_COMPLAINT_WORDS_FIELD, complaints.clone());
                    }
                    if let Some((_, so_far, whole)) = self
                        .steps
                        .last()
                        .and_then(|last| self.the_counts.iter().find(|(step, ..)| step == last))
                    {
                        fields = fields.and(asked::THE_BYTES_SO_FAR_FIELD, so_far.to_string());
                        if let Some(whole) = whole {
                            fields = fields.and(asked::THE_BYTES_IN_THE_WHOLE_FIELD, whole.to_string());
                        }
                    }
                    fields = fields
                        .and(
                            asked::HOW_MANY_LIBRARIES_NOT_FOUND_FIELD,
                            self.not_found.len().to_string(),
                        )
                        .and(asked::ON_THIS_MACHINE_FIELD, asked::IT_IS_PRESENT)
                        .and(asked::THE_RUNS_KIND_FIELD, asked::THE_READY_ACTION)
                        .and(
                            asked::HOW_MANY_COMPOSED_BOUNDARIES_FIELD,
                            self.composed.len().to_string(),
                        )
                        .and(
                            asked::HOW_MANY_BOUNDARIES_REACHED_FIELD,
                            self.reached.len().to_string(),
                        );
                    for (at, boundary) in self.composed.iter().enumerate() {
                        fields = fields.and(
                            &asked::the_field_of_item(
                                asked::A_COMPOSED_BOUNDARY,
                                at,
                                asked::THE_BOUNDARY_FIELD,
                            ),
                            boundary.clone(),
                        );
                    }
                    for (at, boundary) in self.reached.iter().enumerate() {
                        fields = fields.and(
                            &asked::the_field_of_item(
                                asked::A_BOUNDARY_REACHED,
                                at,
                                asked::THE_BOUNDARY_FIELD,
                            ),
                            boundary.clone(),
                        );
                        if self.the_record_carries_its_moments {
                            let item = |name: &str| asked::the_field_of_item(asked::A_BOUNDARY_REACHED, at, name);
                            fields = fields
                                .and(&item(asked::WHAT_THE_STEP_DID_FIELD), asked::IT_PERFORMED)
                                .and(&item(asked::THE_BOUNDARY_AT_FIELD), "1792800000")
                                .and(&item(asked::THE_BOUNDARY_CLOCK_FIELD), "the-machines-own-clock");
                        }
                    }
                    for (at, library) in self.not_found.iter().enumerate() {
                        fields = fields.and(
                            &asked::the_field_of_item(
                                asked::A_LIBRARY_NOT_FOUND,
                                at,
                                asked::THE_LIBRARY_FIELD,
                            ),
                            library.clone(),
                        );
                    }
                }
                _ => {
                    fields = fields.and(
                        asked::WHAT_BECAME_OF_IT_FIELD,
                        asked::THERE_IS_NO_RECORD_OF_IT,
                    );
                }
            }
            let answered = Answered::to(
                &request,
                THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
                "0.1.0",
            )
            .carrying(fields);
            write_result_frame(&answered)
        }
    }

    /// Which boundary the worker's own step feeds, as the machine's own crate
    /// maps it.
    fn the_boundary_a_step_feeds(step: &str) -> Option<&'static str> {
        match step {
            s if s == asked::THE_FETCH_STEP => Some(asked::THE_BOUNDARY_FETCHED),
            s if s == asked::THE_UNPACK_STEP => Some(asked::THE_BOUNDARY_TOOLING),
            s if s == asked::THE_LIBRARY_CHAIN_IS_READ_AGAIN_STEP => {
                Some(asked::THE_BOUNDARY_LIBRARIES)
            }
            s if s == asked::THE_BUILD_IS_PUBLISHED_STEP => Some(asked::THE_BOUNDARY_BLENDER),
            s if s == asked::THE_BUILD_IS_REMOVED_STEP => Some(asked::THE_BOUNDARY_REMOVED),
            _ => None,
        }
    }

    // ───────────────────────────────────────────────────────── the release, here

    /// **The keys are derived from seeds written out here, so they are public.**
    /// **Nothing in a shipped build ever meets either of them**: the admission
    /// takes the root as a value and the application hands it the one the build
    /// carries.
    const THE_ROOT_SEED: [u8; 32] = *b"piece nine's release root, tests";
    const THE_ONLINE_SEED: [u8; 32] = *b"piece nine's online signing key!";

    /// **The source is an address nobody serves.** `example.invalid` is
    /// reserved by its own registry for exactly this.
    const THE_ARTIFACTS_SOURCE: &str = "https://example.invalid/smoozy-server/0.1.0/linux";

    struct AReleaseAsHostingWouldServeIt {
        offered: AnOfferedRelease,
        root: [u8; 32],
        artifact: Vec<u8>,
    }

    /// **It is published the way the next real release will be: one past the
    /// release sequence this build carries as its floor, at the key-set
    /// generation this build carries.** That is the release a desktop built
    /// today actually meets. *(Until 2026-09-21 it was sequence 7 and
    /// generation 1, which passed only while `Server protocol and transfers`
    /// B9's built-in pair was zero; the pair is now the first published
    /// release's, and a sequence below it is refused, as it should be.)*
    fn a_release() -> AReleaseAsHostingWouldServeIt {
        let artifact = b"the machine's own program, standing in".to_vec();
        let root = test_only_keys::a_key_pair_from_a_written_down_seed(THE_ROOT_SEED);
        let online = test_only_keys::a_key_pair_from_a_written_down_seed(THE_ONLINE_SEED);
        let statement = KeySetStatement {
            statement_format: THE_KEY_SET_FORMAT.into(),
            generation: THE_GREATEST_KEY_SET_GENERATION_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE,
            allowed: vec![AnAllowedKey {
                identity: "the online signing key".into(),
                public_key: online.public,
                valid_from_release_sequence: 1,
            }],
            revocations: Vec::new(),
        };
        let statement_bytes = statement.canonical_bytes();
        let entry = composing::compose_an_artifact_entry(
            "linux-x86_64-gnu",
            composing::the_tuple_this_build_is_made_for(),
            &artifact,
            composing::THE_FIXED_ENTRY_POINT_NAME,
            &[THE_ARTIFACTS_SOURCE.to_string()],
        );
        let manifest = composing::compose_a_manifest(
            THE_GREATEST_RELEASE_SEQUENCE_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE + 1,
            "0.1.0",
            2,
            None,
            vec![entry],
        );
        let manifest_bytes = manifest.canonical_bytes();
        AReleaseAsHostingWouldServeIt {
            offered: AnOfferedRelease {
                its_root_signature: a_signature_over(
                    &root.secret,
                    THE_KEY_SET_SIGNING_CONTEXT,
                    &statement_bytes,
                ),
                the_key_set_statements_bytes: statement_bytes,
                its_online_signature: a_signature_over(
                    &online.secret,
                    THE_MANIFEST_SIGNING_CONTEXT,
                    &manifest_bytes,
                ),
                the_manifests_bytes: manifest_bytes,
            },
            root: root.public,
            artifact,
        }
    }

    /// **What hosting would serve, as a table of source to bytes**, so the
    /// exact-source rule is held rather than assumed.
    struct WhatHostingWouldServe {
        table: Vec<(String, Vec<u8>)>,
        asked: Vec<(String, u64)>,
    }

    impl FetchesOneSource for WhatHostingWouldServe {
        fn fetch(&mut self, source: &str, ceiling: u64) -> Result<Vec<u8>, String> {
            self.asked.push((source.to_string(), ceiling));
            match self.table.iter().find(|(named, _)| named == source) {
                Some((_, bytes)) if bytes.len() as u64 > ceiling => {
                    Err(format!("{source}: it is longer than the ceiling"))
                }
                Some((_, bytes)) => Ok(bytes.clone()),
                None => Err(format!("{source}: nothing answers there")),
            }
        }
    }

    /// The trust record, held in memory.
    #[derive(Default)]
    struct TheTrustRecordHere {
        held: Option<smoozy_server_protocol::release::trust::TheTrustRecord>,
    }

    impl smoozy_server_protocol::release::trust::RemembersTheServerPackageTrustRecord
        for TheTrustRecordHere
    {
        fn read_it(
            &self,
        ) -> Result<Option<smoozy_server_protocol::release::trust::TheTrustRecord>, String> {
            Ok(self.held.clone())
        }

        fn write_it(
            &mut self,
            record: &smoozy_server_protocol::release::trust::TheTrustRecord,
        ) -> Result<(), String> {
            self.held = Some(record.clone());
            Ok(())
        }

        fn discard_it(&mut self) -> Result<(), String> {
            self.held = None;
            Ok(())
        }
    }

    // ──────────────────────────────────────────────────────── the other stand-ins

    /// The server row, held in memory, with every write recorded.
    struct TheRowHere {
        row: ServerRow,
        writes: Arc<Mutex<Vec<String>>>,
    }

    fn a_row() -> ServerRow {
        ServerRow {
            local_server_identifier: "server-1".into(),
            host: Host::what_the_user_typed("198.51.100.10").expect("a host"),
            port: Port::STANDARD_SSH,
            username: Username::what_the_user_typed("root").expect("a username"),
            key: KeyReference {
                local_key_identifier: "key-1".into(),
                fingerprint: "SHA256:aaa".into(),
            },
            label: None,
            colour: None,
            remembered_setup: Some(RememberedSetup {
                options: vec!["get-the-server-ready".into()],
            }),
            remembered_host_key: None,
            remembered_hardware_reading: None,
            recorded_machine_root: None,
            last_confirmed_installation_identity: None,
            // **A cursor from a machine that no longer exists**, so the clear
            // at the installation's own write is an assertion rather than an
            // inference.
            event_cursor: Some(97),
            machine_password_subject: None,
            chosen_fallback_folder: None,
            the_driver_missed_the_optix_bar: false,
        }
    }

    impl KeepsTheServersRow for TheRowHere {
        fn the_row(&mut self) -> Option<ServerRow> {
            Some(self.row.clone())
        }

        fn record_the_root(&mut self, recorded: RecordedMachineRoot) -> Result<(), String> {
            self.writes
                .lock()
                .expect("the writes")
                .push(format!("root {}", recorded.path));
            self.row.recorded_machine_root = Some(recorded);
            Ok(())
        }

        fn record_the_installation_and_clear_the_cursor(
            &mut self,
            identity: &Identity,
        ) -> Result<(), String> {
            self.writes
                .lock()
                .expect("the writes")
                .push("installation".to_string());
            self.row.last_confirmed_installation_identity = Some(identity.as_text());
            self.row.event_cursor = None;
            Ok(())
        }

        fn clear_the_remembered_setup(&mut self) -> Result<(), String> {
            self.row.remembered_setup = None;
            Ok(())
        }

        /// **`Servers` Q19a's moment**, recorded in the order it came —
        /// step 10c's eighth-pass fix piece 3.
        fn the_placement_committed(&mut self) {
            self.writes
                .lock()
                .expect("the writes")
                .push("the placement committed".to_string());
        }
    }

    /// The placement, standing in for the transaction — **what this proves is
    /// that it is handed the whole connection and what it is handed**, not what
    /// the transaction does with it.
    struct ThePlacementHere {
        reached: WhatThePlacementReached,
        given: Arc<Mutex<Option<WhatWasHandedToThePlacement>>>,
        /// Whether a session was open on the far end when it was lent.
        sessions_open_at_the_lend: Arc<Mutex<Option<bool>>>,
        sessions: Arc<Mutex<usize>>,
        /// **Where the placement commits on the machine the way a real one
        /// does**: the new installation identity written into the package
        /// state area in the commit (`State, identity and recovery` Y8), and
        /// the supervisor restarted under whatever it was rendering. Nothing
        /// in the tests before 2026-09-28, whose machines hold no Smoozy yet.
        commits_on_the_machine: Option<Arc<Mutex<TheSupervisorHere>>>,
        /// **Where the placement judges as the transaction does, on the phase
        /// record the machine really holds** — `Rented-server runtime` G17's
        /// *deferred* row, `Rented-server runtime` G18's carve-out,
        /// `Rented-server runtime` G21 and `Rented-server runtime` G22, through
        /// the rules half's own functions — **with the work it reads standing in for the
        /// queue reading the stand-in supervisor does not answer.** Nothing,
        /// for every test before step 10c's thirteenth-pass fix piece 2, and
        /// the placement then reaches what it was given.
        the_work_the_commit_reads: Option<Arc<Mutex<Vec<(String, AcceptedWorkStanding)>>>>,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct WhatWasHandedToThePlacement {
        package_area: String,
        installation: String,
        /// The attempt identity the placement was sent under.
        attempt: String,
    }

    /// **The transaction's judgement, on the machine's own phase record** —
    /// what `Server protocol and transfers`' performer reaches, for the rows
    /// a setup's placement meets twice: a different identity meeting
    /// *deferred* is answered with it and acts on nothing (`Rented-server
    /// runtime` G18, `Server protocol and transfers` B6); the same identity
    /// re-evaluates what blocks the commit (`Rented-server runtime` G17's
    /// *deferred* row); a fresh attempt is blocked or not by `Rented-server
    /// runtime` G21. **The record is written the
    /// way the performer writes it**, so the desktop's own inspection reads it
    /// back at the next press.
    fn the_transaction_judges(
        needs: &WhatThePlacementNeeds<'_>,
        work: &[(String, AcceptedWorkStanding)],
    ) -> WhatThePlacementReached {
        use smoozy_runtime::package::phase::{a_fresh_attempts_first_write, ClaimOutcome, Phase, PhaseRecord};
        use smoozy_runtime::package::retry::{
            what_a_different_identity_may_do, whether_the_commit_may_proceed, DifferentIdentity,
            WhetherTheCommitMayProceed,
        };
        let at = PathBuf::from(smoozy_runtime::the_program::the_record_at(
            &needs.package_area,
            smoozy_runtime::the_program::records::THE_PHASE_RECORD,
        ));
        let standing: Option<PhaseRecord> = std::fs::read(&at)
            .ok()
            .and_then(|bytes| smoozy_runtime::written_form::read(&bytes).ok());
        if let Some(standing) = &standing {
            if standing.attempt != needs.attempt
                && matches!(
                    what_a_different_identity_may_do(&ClaimOutcome::Acquired, standing.phase),
                    DifferentIdentity::AnsweredWithTheDeferredPhaseAndActsOnNothing
                )
            {
                return WhatThePlacementReached::Deferred {
                    because: standing.deferred_because.clone().unwrap_or_default(),
                };
            }
        }
        let mut record = standing
            .filter(|standing| standing.attempt == needs.attempt)
            .unwrap_or_else(|| {
                a_fresh_attempts_first_write(
                    needs.attempt.clone(),
                    smoozy_server_protocol::placement::performing::the_phase_records_descriptor_for(
                        &needs.verified.the_release_descriptors_values(),
                    ),
                    None,
                    needs.kind,
                    smoozy_server_protocol::placement::the_staging_location_for(
                        &needs.package_area,
                        &needs.attempt,
                    ),
                )
            });
        let reached = match whether_the_commit_may_proceed(work) {
            WhetherTheCommitMayProceed::Deferred { because } => {
                record.phase = Phase::Deferred;
                record.deferred_because = Some(because.clone());
                WhatThePlacementReached::Deferred { because }
            }
            WhetherTheCommitMayProceed::ItMayProceed => {
                record.phase = Phase::Verified;
                record.deferred_because = None;
                WhatThePlacementReached::Verified
            }
        };
        std::fs::write(&at, smoozy_runtime::written_form::write(&record).expect("the record writes"))
            .expect("the phase record is written");
        reached
    }

    impl<C: AHeldConnection> PlacesTheVerifiedRelease<C> for ThePlacementHere {
        fn place<'a>(
            &'a mut self,
            _connection: &'a mut C,
            needs: WhatThePlacementNeeds<'a>,
        ) -> smoozy_desktop::servers::sessions::ItsOwnFuture<
            'a,
            Result<ThePlacementsOutcome, WhyThePlacementCouldNotBeCarriedOut>,
        > {
            Box::pin(async move {
                *self.given.lock().expect("the given") = Some(WhatWasHandedToThePlacement {
                    package_area: needs.package_area.clone(),
                    installation: needs.installation.as_text(),
                    attempt: needs.attempt.as_text(),
                });
                if let Some(work) = &self.the_work_the_commit_reads {
                    let work = work.lock().expect("the work").clone();
                    self.reached = the_transaction_judges(&needs, &work);
                }
                *self
                    .sessions_open_at_the_lend
                    .lock()
                    .expect("the sessions") =
                    Some(*self.sessions.lock().expect("the count") > 0);
                if let (Some(machine), WhatThePlacementReached::Verified) =
                    (&self.commits_on_the_machine, &self.reached)
                {
                    std::fs::write(
                        PathBuf::from(&needs.package_area).join(
                            smoozy_server_protocol::reconnection::THE_RECORD_THE_IDENTITY_SITS_IN,
                        ),
                        needs.installation.as_text(),
                    )
                    .expect("the commit writes the installation");
                    let mut supervisor = machine.lock().expect("the supervisor");
                    let rendering = supervisor.rendering.clone();
                    supervisor.restarted_with.push(rendering);
                }
                Ok(ThePlacementsOutcome {
                    reached: self.reached.clone(),
                    the_durability_ordering:
                        WhetherTheDurabilityOrderingApplied::TheStagedBytesWereMadeDurableFirst,
                })
            })
        }
    }

    /// The release, as this test's own hosting serves it.
    struct TheReleaseHere {
        root: [u8; 32],
        hosting: WhatHostingWouldServe,
        store: TheTrustRecordHere,
    }

    impl ReadsTheCurrentRelease for TheReleaseHere {
        fn the_current_release(
            &mut self,
            detected: &smoozy_server_protocol::release::manifest::TargetTuple,
        ) -> Result<
            (
                smoozy_server_protocol::release::verifying::AVerifiedRelease,
                Vec<u8>,
            ),
            String,
        > {
            the_current_release(
                &self.root,
                &mut self.hosting,
                &mut self.store,
                detected,
                WHERE_THE_CURRENT_RELEASE_IS_READ_FROM,
            )
        }
    }

    /// **`Server Setup` M1 and `Server Setup` M1a's two answers, stood in
    /// for** — both name absolute paths this desk has none of, so what a
    /// machine said is handed in and **when it was asked is recorded**, which
    /// is what `Server Setup` R2's order turns on.
    struct TheOpeningReadingsHere {
        said: TheMachineBeforeAnythingIsTouched,
        asked_after: Arc<Mutex<Vec<String>>>,
        asked_at: Arc<Mutex<Option<usize>>>,
    }

    impl TakesTheOpeningReadings for TheOpeningReadingsHere {
        fn taken(
            &mut self,
        ) -> smoozy_desktop::servers::sessions::ItsOwnFuture<'_, TheMachineBeforeAnythingIsTouched>
        {
            let at = self.asked_after.lock().expect("the asks").len();
            *self.asked_at.lock().expect("the moment") = Some(at);
            let said = self.said.clone();
            Box::pin(async move { said })
        }
    }

    /// `Server Setup` Y8's stop, asked for once the run has started.
    struct AStopTheUserAsksFor {
        asked: Arc<Mutex<bool>>,
    }

    impl AsksToStop for AStopTheUserAsksFor {
        fn the_user_asked_to_stop(&self) -> bool {
            *self.asked.lock().expect("the stop")
        }
    }

    // ───────────────────────────────────────────────────────── the machine's script

    fn ended_cleanly(said: &str) -> CommandResult {
        CommandResult {
            output: said.as_bytes().to_vec(),
            complaint: Vec::new(),
            status: Some(0),
        }
    }

    fn it_refused() -> CommandResult {
        CommandResult {
            output: Vec::new(),
            complaint: b"no".to_vec(),
            status: Some(1),
        }
    }

    /// **A systemd machine on Ubuntu, as `Server Setup` M1 and
    /// `Server Setup` M1a would have read it.**
    fn an_ubuntu_systemd_machine() -> TheMachineBeforeAnythingIsTouched {
        TheMachineBeforeAnythingIsTouched::as_answered(
            MachineKind::ASystemdMachine,
            Some("ID=ubuntu\nVERSION_ID=\"24.04\"\n".to_string()),
            None,
        )
    }

    /// **A machine `Rented-server runtime` K1's table does not place.**
    fn a_machine_the_table_does_not_place() -> TheMachineBeforeAnythingIsTouched {
        TheMachineBeforeAnythingIsTouched::as_answered(
            MachineKind::TheTableDoesNotPlaceIt,
            Some("ID=ubuntu\n".to_string()),
            None,
        )
    }

    // ── The commands this stand-in is scripted against, each written whole ──
    //
    // **Each is the whole command, because the match is the whole of it.**
    // `Servers`' per-card query opens with `Server Setup`'s whole driver check,
    // so no fragment can name one without naming the other; the two are
    // separate entries here for that reason and for no other.
    //
    // **The real ones are constants of the crates that send them** — this file
    // is the machine, not the sender, and each crate's own test holds its
    // command to one spelling.

    /// `Servers`' first container limb.
    const THE_CONTAINER_MARKER: &str = "test -e /.dockerenv";
    /// `Servers`' second container limb.
    const THE_CONTAINER_TOOL: &str = "systemd-detect-virt --container";
    /// `Servers` H11: the bus scan, by vendor id.
    const THE_BUS_SCAN: &str = "lspci -d 10de:";
    /// `Server Setup` R1 item 5's driver check, whose exit is the whole of the
    /// reading.
    const THE_DRIVER_CHECK: &str = "nvidia-smi";
    /// `Servers` H5's per-card query, which opens with the driver check.
    const THE_PER_CARD_QUERY: &str =
        "nvidia-smi --query-gpu=uuid,pci.bus_id,name,memory.total --format=csv,noheader";
    /// `Server Setup` R1 item 3's probe.
    const THE_UNPACK_TOOLS_PROBE: &str = "dpkg-query -s xz-utils";
    /// **`Server Setup` R1 item 5's module reading on Ubuntu**, asked only
    /// where the driver check ended at 9. Added 2026-09-30.
    const THE_MODULE_READING: &str = "/usr/sbin/modprobe --dry-run --quiet nvidia";

    /// **A systemd machine on Ubuntu that answers every reading.**
    fn a_machine_that_answers() -> Vec<(String, CommandResult)> {
        vec![
            // `Server Setup` M1: not a container.
            (THE_CONTAINER_MARKER.to_string(), it_refused()),
            (THE_CONTAINER_TOOL.to_string(), it_refused()),
            // `Server protocol and transfers` B8's tuple.
            ("uname -s".to_string(), ended_cleanly("Linux\n")),
            ("uname -m".to_string(), ended_cleanly("x86_64\n")),
            ("getconf GNU_LIBC_VERSION".to_string(), ended_cleanly("glibc 2.39\n")),
            // `Server Setup` R1 item 1's two card readings.
            (THE_BUS_SCAN.to_string(), ended_cleanly("")),
            (THE_DRIVER_CHECK.to_string(), it_refused()),
            (THE_PER_CARD_QUERY.to_string(), it_refused()),
            // **`Server Setup` R1 item 5's reading of 2026-11-03**: a driver
            // package stands, so the route goes as it always did, and every
            // test here that means otherwise says so.
            (
                smoozy_server_setup::elevated::THE_DRIVER_PACKAGES_READING.as_str().to_string(),
                ended_cleanly("a driver stands\n"),
            ),
            // **`Server Setup` R1 item 3's probe**, answered the way the
            // package system's own manual says it answers for a package the
            // database does not hold: *"1 The requested query failed either
            // fully or partially, due to no file or package being found"*.
            (
                THE_UNPACK_TOOLS_PROBE.to_string(),
                CommandResult {
                    output: Vec::new(),
                    complaint: b"dpkg-query: package 'xz-utils' is not installed and no \
                                 information is available\n"
                        .to_vec(),
                    status: Some(1),
                },
            ),
        ]
    }

    /// **A machine that already holds the unpack tool**, answering
    /// `Server Setup` R1 item 3's probe with the status entry `dpkg-query -s`
    /// is documented to print.
    fn a_machine_that_already_holds_the_unpack_tool() -> Vec<(String, CommandResult)> {
        let mut script = a_machine_whose_driver_answers();
        script.retain(|(named, _)| named != THE_UNPACK_TOOLS_PROBE);
        script.push((
            THE_UNPACK_TOOLS_PROBE.to_string(),
            ended_cleanly(
                "Package: xz-utils\nStatus: install ok installed\nPriority: standard\nVersion: 5.6.1+really5.4.5-1ubuntu0.3\n",
            ),
        ));
        script
    }

    /// **Both of the driver's two commands answered the same way**, which is
    /// what the fragment match used to do with one entry and is kept exactly so
    /// that this narrowing changes nothing any test drives against.
    ///
    /// **It is worth saying plainly what that means**: every run in this file
    /// has answered `Servers`' per-card query with `Server Setup`'s driver
    /// check's words, and nothing said so until the two were separated.
    /// **Giving the query an answer of its own is a change to what these tests
    /// drive against and is not this sitting's.**
    fn the_driver_answers(
        mut script: Vec<(String, CommandResult)>,
        said: CommandResult,
    ) -> Vec<(String, CommandResult)> {
        script.retain(|(named, _)| named != THE_DRIVER_CHECK && named != THE_PER_CARD_QUERY);
        script.push((THE_DRIVER_CHECK.to_string(), said.clone()));
        script.push((THE_PER_CARD_QUERY.to_string(), said));
        script
    }

    /// **A systemd machine with a card whose driver already answers.**
    ///
    /// `Server Setup` R1 item 5 composes the driver step on such a machine and
    /// its check answers, so *"nothing is installed and nothing is touched"* —
    /// which is what makes all three of the driver's boundaries reachable in one
    /// run with `Server Setup` Y4a's *nothing to do* beside each.
    fn a_machine_whose_driver_answers() -> Vec<(String, CommandResult)> {
        the_driver_answers(
            a_machine_that_answers(),
            ended_cleanly("NVIDIA-SMI 580.105.08\n"),
        )
    }

    // ───────────────────────────────────────────────────────────────── the runs

    struct WhatTheRunLeft {
        came_to: WhereTheSetUpGot,
        writes: Vec<String>,
        steps: Vec<String>,
        composed: Vec<String>,
        given: Option<WhatWasHandedToThePlacement>,
        sessions_open_at_the_lend: Option<bool>,
        asked: Vec<String>,
        row: ServerRow,
        hosting_asked: Vec<(String, u64)>,
        the_readings_were_asked_at: Option<usize>,
        /// **`Server Setup` Y7's five**, as the desktop wrote them, with
        /// `Server Setup` Y4a's mark beside each.
        written_from_the_desktop: Vec<(String, String)>,
        /// Boundary writes the machine refused.
        refused: Vec<String>,
        /// **`Rented-server runtime` P1's state as the run left it.**
        state_at_the_end: Option<String>,
        /// Every run that set the state, and every run whose clear landed.
        set_by: Vec<String>,
        cleared_by: Vec<String>,
        /// **Every command with the state as it stood when that command
        /// reached the machine** — `Server Setup` Y1's *before*, watched as an
        /// order rather than read afterwards.
        the_state_when_a_command_arrived: Vec<(String, Option<String>)>,
        /// Every commanded step with `Server Setup` Y4a's mark the desktop sent
        /// with it, where it sent one.
        marks_on_steps: Vec<(String, Option<String>)>,
        /// The state at the instant `Server Setup` Y4's *done* was written.
        the_state_when_done_was_written: Option<Option<String>>,
        /// **`Rented-server runtime` W7's role desired state at the end**,
        /// where the desktop wrote one.
        the_roles_state: Option<bool>,
        /// **The run `Server Setup` Y8's durable stop named**, where one was
        /// written at all.
        the_stop_was_written_for: Option<String>,
        /// **How many times `Telegram` §20's composing was asked**, so a run
        /// with Telegram off can be shown never to reach it.
        the_composing_was_asked: usize,
        /// **What the placement answered, per file**, as `Telegram` L1's
        /// told-record bookkeeping was handed it.
        the_placement_was_answered: Vec<(String, bool)>,
        /// **Every command that carried a value on its standard input.**
        carried_on_standard_input: Vec<(String, Vec<u8>)>,
        /// **Every step `Server Setup` E2's offer was made for**, in order.
        the_offers_made: Vec<smoozy_server_setup::ready_action::Step>,
        /// **`Server Setup` S2a's desk half**: every failure the desktop wrote
        /// onto the machine, as the run, the step named and the words.
        failures_recorded: Vec<(String, String, String)>,
        /// **Every command with the admin step the machine held as started
        /// when it arrived** — `Server Setup` Y7's second bullet, watched as an
        /// order. Added 2026-09-29.
        the_step_started_when_a_command_arrived: Vec<(String, Option<String>)>,
        /// The Blender version each commanded step carried, where it carried
        /// one, in the order the steps were commanded.
        versions: Vec<Option<String>>,
        /// **The setup as the Set up tab reads it once the run is over**, where
        /// the run was asked to take that reading. Added 2026-09-26.
        the_setup_read_at_the_end: Option<smoozy_desktop::setting_up::TheSetupAsRead>,
        /// **Every moment the machine's run was refreshed**, and **the run's
        /// recorded ending** as the machine holds it at the end. Added
        /// 2026-09-27, eighth-pass fix piece 1.
        refreshed_at: Vec<std::time::Instant>,
        the_machines_ending: Option<(String, Option<String>)>,
        /// **Every restart of the machine's supervisor, with the job it was
        /// rendering then.** Added 2026-09-28, ninth-pass fix piece 3.
        the_supervisor_restarted_with: Vec<Option<String>>,
        /// **Every run the machine was asked to open, by its kind.**
        kinds: Vec<Option<String>>,
        /// **Every frame the machine's supervisor was handed.** Added
        /// 2026-10-24.
        frames: Vec<Vec<u8>>,
    }

    /// **What the run is asked for, beside the machine it is driven against.**
    struct AsThisRunIsAsked {
        /// **How elevation stands as the run is started**, and who answers
        /// `Server Setup` E2's offer. Added 2026-09-21.
        elevation: TheElevationHere,
        /// The Blender the user asked for, where they asked for one.
        install_this_blender: Option<String>,
        /// **`Server Setup` R1 item 4's not-found list this machine reports**,
        /// which is what the libraries install is selected from.
        the_libraries_not_found: Vec<String>,
        /// **Whether the user asked for Telegram**, which is the whole of what
        /// puts `Server Setup` R1 item 8 in the run (`Server Setup` G2).
        telegram_is_on: bool,
        /// **The server row as it stands before the run**, where not the
        /// ordinary never-set-up one. Added 2026-09-21.
        the_row: Option<ServerRow>,
        /// **A setup that stopped part-way, still holding the machine** — its
        /// run, its composed list and the boundaries it reached, as the
        /// machine's own record and state hold them. Added 2026-09-21.
        a_setup_that_stopped: Option<(String, Vec<String>, Vec<String>)>,
        /// **The version that stopped setup's record holds**, where the
        /// machine's program keeps one. Added 2026-09-22.
        the_version_the_stopped_setup_recorded: Option<String>,
        /// **Whether the setup is read afterwards the way the Set up tab reads
        /// it**, over the same connection. Off by default, so no other run's
        /// list of commands grows. Added 2026-09-26.
        read_the_setup_at_the_end: bool,
        /// **The ending the stopped setup's own record holds** — the ending,
        /// the step it names and the words. Added 2026-09-27.
        the_stopped_setup_ended: Option<(String, String, String)>,
        /// **Whether the run is driven under a connected period's mark**, which
        /// `Server Setup` T4 needs before any restart can be sent. Added
        /// 2026-09-27.
        a_connected_period: bool,
        /// **One admin command that takes minutes**, where the run meets one.
        /// Added 2026-09-27, eighth-pass fix piece 1.
        slow: Option<TheSlowCommand>,
        /// **`Server Setup` Y7's window on this machine**, where it keeps one.
        /// Added 2026-09-27, eighth-pass fix piece 1.
        window: Option<std::time::Duration>,
        /// **What this server's live connection read of the machine before the
        /// press** — `State, identity and recovery` Y9's check, taken at the
        /// connection and never by the act. **A bare machine reads absent**,
        /// which is every run in this file before 2026-09-28. Added that day,
        /// ninth-pass fix piece 3.
        the_live_connection_read: Option<InstallationCheck>,
        /// **A job the machine's supervisor is rendering as the act starts**,
        /// where one is. Added 2026-09-28, ninth-pass fix piece 3.
        a_job_is_rendering: Option<String>,
        /// **The work the machine's program holds, as the commit reads it**,
        /// where the placement is to judge as the transaction does. Added
        /// 2026-10-01, step 10c's thirteenth-pass fix piece 2.
        the_work_the_commit_reads: Option<Arc<Mutex<Vec<(String, AcceptedWorkStanding)>>>>,
        /// **Smoozy's own side — a real local record and a secret store — keeping
        /// the server's row and composing `Telegram` §20's list**, where the run
        /// is to be driven the way the running application drives it rather
        /// than through the two stand-ins. Added 2026-10-17, the Creator's
        /// eleventh notes, note 2.
        the_desks_own_side: Option<Arc<TheDesksOwnSide>>,
        /// **The words this machine refuses the role's *enabled* with**, where
        /// it refuses it. Added 2026-10-18, the Creator's twelfth notes, note 2.
        the_role_is_refused_with: Option<&'static str>,
        /// **`Server Setup` Y7a: the server whose run's words the screen keeps**,
        /// the way the Set up tab keeps them, where the run is to keep any.
        /// Added 2026-10-24.
        the_words_are_kept_for: Option<String>,
        /// **Each worker step's words, as this machine reports them**
        /// (`Server Setup` Y7a item 2). Added 2026-10-24.
        the_steps_words: Vec<(String, String, String)>,
    }

    impl Default for AsThisRunIsAsked {
        fn default() -> AsThisRunIsAsked {
            AsThisRunIsAsked {
                elevation: TheElevationHere::Passwordless,
                install_this_blender: Some("4.5.3".to_string()),
                the_libraries_not_found: Vec::new(),
                telegram_is_on: false,
                the_row: None,
                a_setup_that_stopped: None,
                the_version_the_stopped_setup_recorded: None,
                read_the_setup_at_the_end: false,
                the_stopped_setup_ended: None,
                a_connected_period: false,
                slow: None,
                window: None,
                the_live_connection_read: Some(InstallationCheck::Absent),
                a_job_is_rendering: None,
                the_work_the_commit_reads: None,
                the_desks_own_side: None,
                the_role_is_refused_with: None,
                the_words_are_kept_for: None,
                the_steps_words: Vec::new(),
            }
        }
    }

    /// **How elevation stands as a run is started, and who answers the offer.**
    #[derive(Clone, Copy)]
    enum TheElevationHere {
        /// The caller already knows elevation is passwordless.
        Passwordless,
        /// **A password is owed and none is held**, as the running application
        /// starts every run — so `Server Setup` M3's probe decides, and where it
        /// fails the offer is answered with this password.
        OwedAndTheOfferIsAnsweredWith(&'static str),
        /// A password is owed and nobody answers the offer.
        OwedAndNobodyAnswers,
    }

    /// **`Server Setup` E2's offer, answered at the desk**, with every step
    /// it was made for kept so a test can name which.
    struct TheOfferIsAnsweredHere {
        with: Option<&'static str>,
        made_for: Arc<Mutex<Vec<smoozy_server_setup::ready_action::Step>>>,
        /// **`Server Setup` Y7a: the server whose run's words are kept**, the
        /// Set up tab's own route, where any are. Added 2026-10-24.
        words_kept_for: Option<String>,
    }

    impl AsksForThePassword for TheOfferIsAnsweredHere {
        fn the_password_for(
            &mut self,
            step: smoozy_server_setup::ready_action::Step,
        ) -> smoozy_desktop::servers::sessions::ItsOwnFuture<
            '_,
            Option<smoozy_security_and_licence::plaintext::Origin>,
        > {
            self.made_for.lock().expect("the offers").push(step);
            let answer = self
                .with
                .map(smoozy_security_and_licence::plaintext::Origin::what_the_user_typed);
            Box::pin(async move { answer })
        }

        fn where_the_words_go(&mut self) -> Option<smoozy_desktop::setting_up::TheWordsGoHere> {
            self.words_kept_for
                .as_deref()
                .map(|server| smoozy_desktop::setting_up::the_words_go_to_the_card(server, || {}))
        }
    }

    /// **The token this test hands to `Telegram` §20's composing**, which is a
    /// made-up one and never a real bot's.
    const A_MADE_UP_TOKEN: &[u8] = b"123456:made-up-for-a-test";

    /// **`Telegram` §20's list, composed for one run at the desk.**
    ///
    /// **It stands in for the desktop's record and its secret store**, which is
    /// where the real composing reads from; what it proves is that
    /// `Server Setup` R1 item 8 places what it is handed, where it is handed it,
    /// and sets the role's state afterwards.
    struct ATelegramMaterialHere {
        /// The package area this run's hand-over landed on.
        package_area: String,
        /// **Whether the composing was asked at all**, so a run with Telegram
        /// off can be shown never to reach it.
        asked: Arc<Mutex<usize>>,
        /// What the placement answered, handed back for `Telegram` L1's
        /// told-record bookkeeping.
        answered: Arc<Mutex<Vec<(String, bool)>>>,
    }

    impl ComposesTheTelegramMaterial for ATelegramMaterialHere {
        fn composed(&mut self) -> Result<smoozy_telegram::material::TheMaterial, String> {
            *self.asked.lock().expect("the count") += 1;
            smoozy_telegram::material::TheMaterial::composed(
                &self.package_area,
                smoozy_security_and_licence::plaintext::Plaintext::new(A_MADE_UP_TOKEN.to_vec()),
                smoozy_telegram::told::TokenGeneration(1),
                smoozy_telegram::material::TheRoleConfiguration {
                    conversation: smoozy_telegram::proof::TheConversation(9001),
                    machine: "0011223344556677".to_string(),
                    colour: None,
                    label: Some("render box".to_string()),
                    reporting: vec![("a-server-has-had-nothing-to-do".to_string(), true)],
                },
            )
            .map_err(|why| format!("{why:?}"))
        }

        fn the_placement_was_answered(
            &mut self,
            material: &smoozy_telegram::material::TheMaterial,
            placed: &[(String, smoozy_server_setup::acts::HowTheActWent)],
        ) {
            let _ = material;
            let mut held = self.answered.lock().expect("the answers");
            for (path, went) in placed {
                held.push((path.clone(), went.it_was_carried_out()));
            }
        }

        /// **`Telegram` L1 and L2: the role's state, acknowledged**, kept
        /// beside the files' answers in the order it came. The 38's piece 11.
        fn the_desired_state_was_acknowledged(&mut self) {
            self.answered
                .lock()
                .expect("the answers")
                .push((THE_ROLES_STATE_ACKNOWLEDGED.to_string(), true));
        }
    }

    /// What the stand-in keeps where the role's state was acknowledged.
    const THE_ROLES_STATE_ACKNOWLEDGED: &str = "the role's desired state, acknowledged";

    fn the_order_is_driven(
        scratch: &Scratch,
        sftp_server: PathBuf,
        script: Vec<(String, CommandResult)>,
        said: TheMachineBeforeAnythingIsTouched,
        reached: WhatThePlacementReached,
        stop: Option<Arc<Mutex<bool>>>,
    ) -> WhatTheRunLeft {
        the_order_is_driven_as(
            scratch,
            sftp_server,
            script,
            said,
            reached,
            stop,
            AsThisRunIsAsked::default(),
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn the_order_is_driven_as(
        scratch: &Scratch,
        sftp_server: PathBuf,
        script: Vec<(String, CommandResult)>,
        said: TheMachineBeforeAnythingIsTouched,
        reached: WhatThePlacementReached,
        stop: Option<Arc<Mutex<bool>>>,
        as_asked: AsThisRunIsAsked,
    ) -> WhatTheRunLeft {
        let release = a_release();
        let mut hosting = WhatHostingWouldServe {
            table: vec![
                (
                    WHERE_THE_CURRENT_RELEASE_IS_READ_FROM.to_string(),
                    release.offered.canonical_bytes(),
                ),
                (THE_ARTIFACTS_SOURCE.to_string(), release.artifact.clone()),
            ],
            asked: Vec::new(),
        };
        hosting.asked.clear();

        let asked = Arc::new(Mutex::new(Vec::new()));
        let sessions = Arc::new(Mutex::new(0usize));
        let supervisor = Arc::new(Mutex::new(match as_asked.a_setup_that_stopped.clone() {
            Some((run, composed, reached)) => TheSupervisorHere {
                not_found: as_asked.the_libraries_not_found.clone(),
                run: Some(run.clone()),
                composed,
                reached,
                state: Some(run.clone()),
                set_by: vec![run.clone()],
                the_recorded_version: as_asked.the_version_the_stopped_setup_recorded.clone(),
                the_stopped_run_ended: as_asked
                    .the_stopped_setup_ended
                    .clone()
                    .map(|(ending, step, words)| (run, ending, step, words)),
                ..TheSupervisorHere::default()
            },
            None => TheSupervisorHere {
                not_found: as_asked.the_libraries_not_found.clone(),
                window: as_asked.window,
                rendering: as_asked.a_job_is_rendering.clone(),
                the_role_is_refused_with: as_asked.the_role_is_refused_with,
                the_steps_words: as_asked.the_steps_words.clone(),
                ..TheSupervisorHere::default()
            },
        }));
        let when_a_command_arrived = Arc::new(Mutex::new(Vec::new()));
        let mut far_end = TheFarEnd {
            sftp_server,
            at: scratch.at.clone(),
            said: Arc::new(Mutex::new(script)),
            asked: Arc::clone(&asked),
            supervisor: Arc::clone(&supervisor),
            children: Arc::new(Mutex::new(Vec::new())),
            goes_down: Arc::new(tokio::sync::Notify::new()),
            the_mark_when_the_restart_arrived: None,
            the_state_when_a_command_arrived: Arc::clone(&when_a_command_arrived),
            the_restart_takes_the_connection: false,
            written_when_the_restart_arrived: Arc::new(Mutex::new(None)),
            slow: as_asked.slow.clone(),
            drivers: None,
        };

        let writes = Arc::new(Mutex::new(Vec::new()));
        let mut row = TheRowHere {
            row: as_asked.the_row.clone().unwrap_or_else(a_row),
            writes: Arc::clone(&writes),
        };
        let given = Arc::new(Mutex::new(None));
        let sessions_open_at_the_lend = Arc::new(Mutex::new(None));
        let placement = ThePlacementHere {
            reached,
            given: Arc::clone(&given),
            sessions_open_at_the_lend: Arc::clone(&sessions_open_at_the_lend),
            sessions: Arc::clone(&sessions),
            commits_on_the_machine: as_asked
                .a_job_is_rendering
                .is_some()
                .then(|| Arc::clone(&supervisor)),
            the_work_the_commit_reads: as_asked.the_work_the_commit_reads.clone(),
        };
        let mut releases = TheReleaseHere {
            root: release.root,
            hosting,
            store: TheTrustRecordHere::default(),
        };

        let the_readings_were_asked_at = Arc::new(Mutex::new(None));
        let mut taking = TheOpeningReadingsHere {
            said,
            asked_after: Arc::clone(&asked),
            asked_at: Arc::clone(&the_readings_were_asked_at),
        };
        let asked_for = WhatWasAskedFor {
            install_this_blender: as_asked.install_this_blender.clone(),
            telegram_is_on: as_asked.telegram_is_on,
            ..WhatWasAskedFor::default()
        };
        let the_composing_was_asked = Arc::new(Mutex::new(0usize));
        let the_placement_was_answered = Arc::new(Mutex::new(Vec::new()));
        let mut telegram = ATelegramMaterialHere {
            // **The package area this run's own hand-over landed on**, which is
            // what `Security and licence` §10 requires the token file to sit
            // under.
            package_area: format!("{}/smoozy/package", scratch.text()),
            asked: Arc::clone(&the_composing_was_asked),
            answered: Arc::clone(&the_placement_was_answered),
        };
        // **Where Smoozy's own side is asked for, it keeps the row and composes
        // the material in place of the two stand-ins**, exactly as the running
        // application hands them to the order.
        let the_desk = as_asked.the_desks_own_side.clone();
        let mut the_desks_row = the_desk.as_deref().map(|desk| TheServersRowHere {
            record: &desk.record,
            server: desk.server.clone(),
        });
        let mut the_desks_material = the_desk.as_deref().map(TheDesksOwnSide::the_material);
        let row_keeper: &mut dyn KeepsTheServersRow = match the_desks_row.as_mut() {
            Some(it) => it,
            None => &mut row,
        };
        let composer: &mut dyn ComposesTheTelegramMaterial = match the_desks_material.as_mut() {
            Some(it) => it,
            None => &mut telegram,
        };
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("a runtime for the run");
        let point = ThisServersSessions::<TheFarEnd>::for_one_server();
        let asking = point.clone();
        let no_additional: TheAdditionalConnection<TheFarEnd> =
            Arc::new(|| Box::pin(async { None }));
        let the_offers_made = Arc::new(Mutex::new(Vec::new()));
        let (elevation, answered_with) = match as_asked.elevation {
            TheElevationHere::Passwordless => (
                smoozy_server_setup::elevated::HowElevationIsObtained::Passwordless,
                None,
            ),
            TheElevationHere::OwedAndTheOfferIsAnsweredWith(password) => (
                smoozy_server_setup::elevated::HowElevationIsObtained::APasswordIsOwedAndNoneIsHeld,
                Some(password),
            ),
            TheElevationHere::OwedAndNobodyAnswers => (
                smoozy_server_setup::elevated::HowElevationIsObtained::APasswordIsOwedAndNoneIsHeld,
                None,
            ),
        };
        let mut the_offer = TheOfferIsAnsweredHere {
            with: answered_with,
            made_for: Arc::clone(&the_offers_made),
            words_kept_for: as_asked.the_words_are_kept_for.clone(),
        };
        let read_the_setup_at_the_end = as_asked.read_the_setup_at_the_end;
        let (came_to, the_setup_read_at_the_end) = runtime.block_on(async {
            let lending = lend_until_it_ends(&point, &mut far_end, &no_additional);
            let stopper: Box<dyn AsksToStop> = match stop {
                Some(asked) => Box::new(AStopTheUserAsksFor { asked }),
                None => Box::new(NobodyAsked),
            };
            let set_up = a_machine_is_set_up(
                &asking,
                &mut taking,
                row_keeper,
                placement,
                &mut releases,
                &asked_for,
                elevation,
                composer,
                &mut the_offer,
                stopper.as_ref(),
                // **No connected period, because no restart is driven here** —
                // every machine in these runs either has no card or has a
                // driver that answers. `Server Setup` §8's three endings are
                // driven below, each with a real period's mark. **A run that
                // asks for one gets a period nobody ends**, which is enough for
                // a restart the machine refuses (2026-09-27).
                as_asked
                    .a_connected_period
                    .then(smoozy_servers::restart::TheRestartMark::none_is_set),
                as_asked.the_live_connection_read,
            );
            let driven = async {
                let came_to = set_up.await;
                let read = if read_the_setup_at_the_end {
                    smoozy_desktop::setting_up::the_setup_as_read(
                        &asking,
                        &format!("{}/smoozy/package", scratch.text()),
                        None,
                    )
                    .await
                } else {
                    None
                };
                (came_to, read)
            };
            tokio::pin!(lending, driven);
            tokio::select! {
                biased;
                _ended = &mut lending => unreachable!("the far end never ends on its own"),
                came_to = &mut driven => came_to,
            }
        });

        #[allow(clippy::type_complexity)]
        let (
            steps,
            composed,
            written_from_the_desktop,
            refused,
            state_at_the_end,
            set_by,
            cleared_by,
            marks_on_steps,
            the_state_when_done_was_written,
            the_roles_state,
            the_stop_was_written_for,
            carried_on_standard_input,
            failures_recorded,
            the_step_started_when_a_command_arrived,
        ) = {
            let held = supervisor.lock().expect("the supervisor");
            (
                held.steps.clone(),
                held.composed.clone(),
                held.written_from_the_desktop.clone(),
                held.refused.clone(),
                held.state.clone(),
                held.set_by.clone(),
                held.cleared_by.clone(),
                held.marks_on_steps.clone(),
                held.the_state_when_done_was_written.clone(),
                held.the_roles_state,
                held.the_stop_was_written_for.clone(),
                held.carried_on_standard_input.clone(),
                held.failures_recorded.clone(),
                held.the_step_started_when_a_command_arrived.clone(),
            )
        };
        let the_writes = writes.lock().expect("the writes").clone();
        let the_given = given.lock().expect("the given").clone();
        let at_the_lend = *sessions_open_at_the_lend.lock().expect("the sessions");
        let the_asks = asked.lock().expect("the asks").clone();
        let readings_at = *the_readings_were_asked_at.lock().expect("the moment");
        let the_order_of_commands = when_a_command_arrived.lock().expect("the order").clone();
        let the_composing_count = *the_composing_was_asked.lock().expect("the count");
        let the_answers = the_placement_was_answered
            .lock()
            .expect("the answers")
            .clone();
        WhatTheRunLeft {
            came_to,
            writes: the_writes,
            steps,
            composed,
            given: the_given,
            sessions_open_at_the_lend: at_the_lend,
            asked: the_asks,
            row: row.row.clone(),
            hosting_asked: releases.hosting.asked.clone(),
            the_readings_were_asked_at: readings_at,
            written_from_the_desktop,
            refused,
            state_at_the_end,
            set_by,
            cleared_by,
            the_state_when_a_command_arrived: the_order_of_commands,
            marks_on_steps,
            the_state_when_done_was_written,
            the_roles_state,
            the_stop_was_written_for,
            the_composing_was_asked: the_composing_count,
            the_placement_was_answered: the_answers,
            carried_on_standard_input,
            failures_recorded,
            the_step_started_when_a_command_arrived,
            the_offers_made: {
                let made = the_offers_made.lock().expect("the offers").clone();
                made
            },
            versions: {
                let versions = supervisor.lock().expect("the supervisor").versions.clone();
                versions
            },
            the_setup_read_at_the_end,
            refreshed_at: {
                let refreshed = supervisor.lock().expect("the supervisor").refreshed_at.clone();
                refreshed
            },
            the_machines_ending: {
                let ending = supervisor.lock().expect("the supervisor").ending.clone();
                ending
            },
            the_supervisor_restarted_with: {
                let restarted = supervisor.lock().expect("the supervisor").restarted_with.clone();
                restarted
            },
            kinds: {
                let kinds = supervisor.lock().expect("the supervisor").kinds.clone();
                kinds
            },
            frames: {
                let frames = supervisor.lock().expect("the supervisor").frames.clone();
                frames
            },
        }
    }


    // ═════════════════════════════════════════ `Server Setup` §8, the restart

    /// **A systemd machine with a card on the bus and no driver answering**,
    /// which is the one machine `Server Setup` R1 item 5 installs on and item 6
    /// restarts.
    fn a_machine_with_a_card_and_no_driver() -> Vec<(String, CommandResult)> {
        let mut script = a_machine_that_answers();
        script.retain(|(named, _)| named != THE_BUS_SCAN);
        script.push((
            THE_BUS_SCAN.to_string(),
            ended_cleanly("01:00.0 3D controller: NVIDIA Corporation Device 2684\n"),
        ));
        script
    }

    /// **A systemd machine with a card and a driver already installed on it
    /// that does not answer** — R1 item 5's *"where a driver is already
    /// installed, the restart is tried before any install"*, and the state a
    /// working-but-unbooted install is in.
    ///
    /// **The exit is what says so and nothing else is read**: NVIDIA's own
    /// documentation gives **9** as *"NVIDIA driver is not loaded"*, and the one
    /// real failure of this kind this project has watched ended at 9 and wrote
    /// its whole message to standard output.
    ///
    /// **And the running kernel has the driver's module** — `Server Setup` R1
    /// item 5 as corrected on 2026-09-30: *"A driver counts as installed for
    /// this rule only where the running kernel has its module"*. The loader's
    /// dry run finds it and ends at 0. **Written out rather than left to the
    /// script's default answer**, so this machine is the unbooted-module one on
    /// purpose.
    fn a_machine_whose_driver_is_installed_and_not_loaded() -> Vec<(String, CommandResult)> {
        let mut script = the_driver_answers(
            a_machine_with_a_card_and_no_driver(),
            CommandResult {
                output: b"NVIDIA-SMI has failed because it couldn't communicate with the NVIDIA \
                          driver. Make sure that the latest NVIDIA driver is installed and \
                          running.\n"
                    .to_vec(),
                complaint: Vec::new(),
                status: Some(
                    smoozy_server_setup::readings::
                        THE_EXIT_THAT_MEANS_A_DRIVER_IS_INSTALLED_AND_NOT_LOADED,
                ),
            },
        );
        script.push((THE_MODULE_READING.to_string(), ended_cleanly("")));
        script
    }

    /// **The same machine after the boot: its card answers.**
    fn the_machine_that_came_back() -> Vec<(String, CommandResult)> {
        the_driver_answers(
            a_machine_with_a_card_and_no_driver(),
            ended_cleanly("GPU-1111,0000:01:00.0,NVIDIA GeForce RTX 4090,24564 MiB\n"),
        )
    }

    // ───────────────────────────── machine 1 of the eleventh pass, remembered

    /// **A machine that remembers which driver packages stand on it**, so a
    /// run's installs change what the next command and the next run meet.
    /// Added 2026-09-30, step 10c's eleventh-pass fix piece 10.
    ///
    /// **What it replays is machine 1 of the eleventh pass on Hyperstack**
    /// (`Audits/Build/Step 10c/The eleventh pass — Hyperstack.md`, new fault
    /// 1): the first run cut while the driver install ran on and put branch
    /// 595 and a new kernel's modules on the machine; a second run whose
    /// install chose **580** and removed the 595 modules, and whose utilities
    /// act installed **`nvidia-utils-595`** and removed the 580 modules; a
    /// restart into the new kernel with **no module for any driver**; and a
    /// third run that restarted and failed the same way.
    ///
    /// **Each command is answered the way its own publication says it
    /// behaves, and no further.**
    ///
    /// - **`ubuntu-drivers install --gpgpu`** goes down the branches in the
    ///   detection routine's order and takes **the first whose driver package
    ///   is not already installed** — the tool's own source,
    ///   `install_gpgpu` (`ubuntu-drivers` at
    ///   `https://git.launchpad.net/ubuntu/+source/ubuntu-drivers-common/plain/ubuntu-drivers?h=ubuntu/noble-updates`,
    ///   read 2026-09-30): *"if candidate and not cache[candidate].current_ver:
    ///   to_install.append(p)"*, where the candidate is
    ///   `nvidia-headless-no-dkms-<flavour>`. **That is why the second run
    ///   chose 580**: 595's package was already there.
    /// - **`ubuntu-drivers list --gpgpu --recommended`** prints the first
    ///   branch in the same order whatever is installed (the same file,
    ///   `list`), which is why the old utilities act said 595.
    /// - **Two branches do not stand together**: installing one removed the
    ///   other's modules both ways on machine 1 (observed; `apt`'s history
    ///   there). The package pages carry no conflict field to cite, so the
    ///   stand-in removes the other branch's packages whole and says so here.
    /// - **The kernel module loads only on a boot of a kernel it was built
    ///   for**, and `nvidia-smi` answers only where its branch is the loaded
    ///   module's; otherwise it ends at 9, *"NVIDIA driver is not loaded"*.
    #[derive(Debug, Clone)]
    struct TheDriversOnTheMachine {
        /// The branches, in the detection routine's order.
        offered: Vec<&'static str>,
        /// The branches whose `nvidia-headless-no-dkms-…` package stands.
        driver_packages: Vec<&'static str>,
        /// The branch whose kernel modules stand, and the kernel they are for.
        modules: Option<(&'static str, &'static str)>,
        /// The branch whose `nvidia-utils-…` stands.
        utilities: Option<&'static str>,
        /// **The branch whose `libnvidia-gl-…` stands, and with it the OptiX
        /// library** — `Server Setup` R1 item 5's fifth act, from 2026-10-03.
        /// None on both machines below: the driver route left it out, which is
        /// what Fact 47 saw on a Hyperstack image.
        optix_library: Option<&'static str>,
        running: &'static str,
        /// The kernel the next boot takes.
        newest: &'static str,
        /// The branch whose module is loaded.
        loaded: Option<&'static str>,
        /// **Every package act the machine performed, in order**, as
        /// *driver 580* or *utilities 595*.
        performed: Vec<String>,
    }

    /// The branch's number, which is what two branches conflict on.
    fn the_number_of(branch: &str) -> &str {
        branch.split('-').next().unwrap_or(branch)
    }

    impl TheDriversOnTheMachine {
        /// **Machine 1 as the first run left it**: cut during the driver
        /// install, which ran on to its end — branch 595 and the new kernel's
        /// 595 modules, no utilities, still running the old kernel.
        fn machine_one_after_the_cut_install() -> TheDriversOnTheMachine {
            TheDriversOnTheMachine {
                offered: vec!["595-server-open", "580-server-open"],
                driver_packages: vec!["595-server-open"],
                modules: Some(("595-server-open", "7.0.0-34-generic")),
                utilities: None,
                optix_library: None,
                running: "6.11.0-26-generic",
                newest: "7.0.0-34-generic",
                loaded: None,
                performed: Vec::new(),
            }
        }

        /// **The dead state machine 1 was destroyed in**: the 595 utilities,
        /// no module for any kernel, booted into the new kernel.
        fn the_dead_state() -> TheDriversOnTheMachine {
            TheDriversOnTheMachine {
                offered: vec!["595-server-open", "580-server-open"],
                driver_packages: Vec::new(),
                modules: None,
                utilities: Some("595-server"),
                optix_library: None,
                running: "7.0.0-34-generic",
                newest: "7.0.0-34-generic",
                loaded: None,
                performed: Vec::new(),
            }
        }

        /// **A branch put on the machine, and every other branch's packages
        /// taken off.**
        fn the_other_branches_go(&mut self, kept: &str) {
            let kept = the_number_of(kept).to_string();
            self.driver_packages.retain(|branch| the_number_of(branch) == kept);
            if self.modules.is_some_and(|(branch, _)| the_number_of(branch) != kept) {
                self.modules = None;
            }
            if self.utilities.is_some_and(|branch| the_number_of(branch) != kept) {
                self.utilities = None;
            }
            if self.optix_library.is_some_and(|branch| the_number_of(branch) != kept) {
                self.optix_library = None;
            }
        }

        fn the_machine_boots(&mut self) {
            self.running = self.newest;
            self.loaded = self
                .modules
                .filter(|(_, kernel)| *kernel == self.running)
                .map(|(branch, _)| branch);
        }

        /// **The utilities package a branch's driver package goes with** —
        /// the open flavour shares the proprietary one's.
        fn the_utilities_of(branch: &'static str) -> &'static str {
            match branch {
                "595-server-open" | "595-server" => "595-server",
                "580-server-open" | "580-server" => "580-server",
                other => panic!("no utilities are published for {other}"),
            }
        }

        fn answers(&mut self, asked: &str) -> Option<CommandResult> {
            if asked == THE_DRIVER_CHECK || asked == THE_PER_CARD_QUERY {
                let Some(utilities) = self.utilities else {
                    return Some(CommandResult {
                        output: Vec::new(),
                        complaint: b"bash: line 1: nvidia-smi: command not found\n".to_vec(),
                        status: Some(127),
                    });
                };
                if self
                    .loaded
                    .is_some_and(|branch| the_number_of(branch) == the_number_of(utilities))
                {
                    return Some(ended_cleanly(
                        "GPU-1111,0000:01:00.0,NVIDIA RTX A4000,16376 MiB\n",
                    ));
                }
                return Some(CommandResult {
                    output: b"NVIDIA-SMI has failed because it couldn't communicate with the \
                              NVIDIA driver.\n"
                        .to_vec(),
                    complaint: Vec::new(),
                    status: Some(
                        smoozy_server_setup::readings::
                            THE_EXIT_THAT_MEANS_A_DRIVER_IS_INSTALLED_AND_NOT_LOADED,
                    ),
                });
            }
            if asked == smoozy_server_setup::elevated::THE_DRIVER_PACKAGES_READING.as_str() {
                // **`Server Setup` R1 item 5's reading, answered from what this
                // machine holds.**
                return Some(ended_cleanly(if self.driver_packages.is_empty() {
                    ""
                } else {
                    "a driver stands\n"
                }));
            }
            if asked.contains("modprobe") {
                // **The running kernel's own module directory, and nothing
                // else** — `modprobe(8)`: *"modprobe looks in the module
                // directory /lib/modules/`uname -r`"*.
                let there = self
                    .modules
                    .is_some_and(|(_, kernel)| kernel == self.running);
                return Some(if there {
                    ended_cleanly("")
                } else {
                    CommandResult {
                        output: Vec::new(),
                        complaint: Vec::new(),
                        status: Some(1),
                    }
                });
            }
            if asked.contains("libnvoptix.so.1") {
                // **R1 item 5's fifth act, answered the way its own command
                // would be**: where the OptiX library stands, the loader finds
                // it and nothing is installed; where it does not, the branch is
                // read off the driver package the install left, as the fourth
                // act reads it, and that branch's `libnvidia-gl-…` goes on.
                if self.optix_library.is_some() {
                    return Some(ended_cleanly(""));
                }
                assert!(
                    asked.contains("dpkg-query -W") && asked.contains("\"libnvidia-gl-$("),
                    "the OptiX act reads the branch some way this machine does not know: {asked}"
                );
                let mut standing = self.driver_packages.clone();
                standing.sort();
                let Some(branch) = standing.first().copied() else {
                    return Some(CommandResult {
                        output: Vec::new(),
                        complaint: b"E: Unable to locate package libnvidia-gl-\n".to_vec(),
                        status: Some(100),
                    });
                };
                let library = TheDriversOnTheMachine::the_utilities_of(branch);
                self.the_other_branches_go(library);
                self.optix_library = Some(library);
                self.performed.push(format!("OptiX library {}", the_number_of(library)));
                return Some(ended_cleanly(""));
            }
            if asked.contains("nvidia-utils-") {
                // **The utilities act, answered by what its own read reads.**
                let read = if asked.contains("ubuntu-drivers list --gpgpu --recommended") {
                    self.offered.first().copied()
                } else if asked.contains("dpkg-query -W") {
                    let mut standing = self.driver_packages.clone();
                    standing.sort();
                    standing.first().copied()
                } else {
                    panic!("the utilities act reads the branch some way this machine does not know: {asked}")
                };
                let Some(branch) = read else {
                    return Some(CommandResult {
                        output: Vec::new(),
                        complaint: b"E: Unable to locate package nvidia-utils-\n".to_vec(),
                        status: Some(100),
                    });
                };
                let utilities = TheDriversOnTheMachine::the_utilities_of(branch);
                self.the_other_branches_go(utilities);
                self.utilities = Some(utilities);
                self.performed.push(format!("utilities {}", the_number_of(utilities)));
                return Some(ended_cleanly(""));
            }
            if asked.contains("ubuntu-drivers install --gpgpu") {
                let chosen = self
                    .offered
                    .iter()
                    .find(|branch| !self.driver_packages.contains(branch))
                    .copied();
                let Some(branch) = chosen else {
                    return Some(ended_cleanly("All the available drivers are already installed.\n"));
                };
                self.the_other_branches_go(branch);
                self.driver_packages.push(branch);
                self.modules = Some((branch, self.newest));
                self.performed.push(format!("driver {}", the_number_of(branch)));
                return Some(ended_cleanly(""));
            }
            None
        }
    }

    /// **`Server Setup` T6's three endings, and this is which one the test
    /// drives.** They are `Servers` M6's and this file performs them on the
    /// period's own mark, which is the handle the real driving acts on.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum AfterTheRestart {
        /// **M6 item 1.** The machine answers: the connection is really
        /// re-lent and the mark clears.
        TheMachineAnswers,
        /// **M6 item 3.** The reconnect policy for the marked period gives up.
        ThePolicyGivesUp,
        /// **M6 item 2.** The user stops waiting, **which clears nothing**.
        TheUserStopsWaiting,
    }

    struct WhatTheRestartLeft {
        came_to: WhereTheSetUpGot,
        /// Every command either machine was asked, in order.
        asked: Vec<String>,
        written_from_the_desktop: Vec<(String, String)>,
        /// **Where the mark stood when the run ended**, which is T6's whole
        /// point: two of the three endings clear it and one does not.
        the_mark: smoozy_servers::restart::WhereTheMarkStands,
        /// **Where it stood at the instant `systemctl reboot` reached the
        /// machine**, which is T4's order rather than its fact.
        the_mark_when_the_restart_arrived:
            Option<smoozy_servers::restart::WhereTheMarkStands>,
        /// The run a recovery took over, where one did.
        took_over: Option<String>,
        /// The run identity each open carried a kind for.
        kinds: Vec<Option<String>>,
        /// The run's own identity at each open, in order.
        runs: Vec<String>,
        /// **The run `Server Setup` Y8's durable stop named**, where one was
        /// written at all — nothing is what `Server Setup` Y8a requires of a
        /// stop made while the machine is down.
        the_stop_was_written_for: Option<String>,
        /// **Every command with `Server Setup` Y1's state as it stood when that
        /// command reached the machine.**
        the_state_when_a_command_arrived: Vec<(String, Option<String>)>,
        /// **Every command with the admin step the machine held as started
        /// when it arrived** — `Server Setup` Y7's second bullet. Added
        /// 2026-09-29.
        the_step_started_when_a_command_arrived: Vec<(String, Option<String>)>,
        /// **The boundaries written at the instant the restart reached the
        /// machine**, where it reached it. Added 2026-09-27.
        written_when_the_restart_arrived: Option<Vec<String>>,
        /// **`Server Setup` S2a's desk half**, every failure written onto the
        /// machine. Added 2026-09-27.
        failures_recorded: Vec<(String, String, String)>,
        /// **The setup as the Set up tab reads it once the run is over**, where
        /// the machine was still up to be read. Added 2026-09-27.
        the_setup_read_at_the_end: Option<smoozy_desktop::setting_up::TheSetupAsRead>,
        /// **The step under way each time the driving told the screen**, in
        /// order. Added 2026-09-27, fix piece 9.
        the_screen_was_told: Vec<Option<smoozy_server_setup::run::Boundary>>,
        /// **Every run that set `Server Setup` Y1's state, and every run whose
        /// clear the machine took**, in order. Added 2026-10-12.
        set_by: Vec<String>,
        cleared_by: Vec<String>,
        /// **How many queue readings the desktop took** (`Server Setup` R1b).
        queue_readings: usize,
    }

    /// **The Set up tab, standing in: it keeps the step under way each time the
    /// driving tells it the run**, and declines every offer as nobody would
    /// answer it. Added 2026-09-27, fix piece 9.
    struct TheScreenHere {
        told: Arc<Mutex<Vec<Option<smoozy_server_setup::run::Boundary>>>>,
        /// **The server whose words are kept the Set up tab's way**, for a
        /// swap run alone. Added 2026-10-30, the twenty-third notes, note 3.
        words_kept_for: Option<String>,
    }

    impl AsksForThePassword for TheScreenHere {
        fn the_password_for(
            &mut self,
            _step: smoozy_server_setup::ready_action::Step,
        ) -> smoozy_desktop::servers::sessions::ItsOwnFuture<
            '_,
            Option<smoozy_security_and_licence::plaintext::Origin>,
        > {
            Box::pin(async { None })
        }

        fn the_run_is_as_read(&mut self, read: &smoozy_desktop::setting_up::TheSetupAsRead) {
            self.told.lock().expect("the screen").push(read.the_step_in_flight);
        }

        fn where_the_words_go(&mut self) -> Option<smoozy_desktop::setting_up::TheWordsGoHere> {
            self.words_kept_for
                .as_deref()
                .map(|server| smoozy_desktop::setting_up::the_words_go_to_the_card(server, || {}))
        }

        fn where_the_swaps_words_go(&mut self) -> Option<smoozy_desktop::setting_up::TheWordsGoHere> {
            self.words_kept_for.as_deref().map(smoozy_desktop::setting_up::the_swaps_words_go_apart)
        }
    }

    /// **What `systemctl reboot` does on the machine it reaches.** Added
    /// 2026-09-27, fix piece 6.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum TheRestartCommand {
        /// **Fact 51's shape: it answers 0, and then the connection drops.**
        AnswersAndThenDrops,
        /// **Step 10c's seventh passes: the reboot takes the connection with
        /// the command still in hand**, and nothing comes back for it.
        TakesTheConnection,
        /// **The runner's ruling 1 of 2026-09-27's case: the command is refused
        /// and the machine stays up** — answered from the script.
        IsRefused,
    }

    /// **`Server Setup` §8, driven against a stand-in that really drops and
    /// really comes back.**
    ///
    /// **The machine goes down for real**: the far end's connection ends, the
    /// point stops lending, and every session on it is gone — which is what a
    /// `systemctl reboot` does after returning 0 (`Server Setup` T1). **What is
    /// stood in for is `Servers`' reconnect policy**, whose four runs and ten
    /// minutes are that document's and are proved in that crate; here the
    /// ending it would declare is declared on the same handle, so what this
    /// file proves is what `Server Setup` does with each of the three.
    fn the_restart_is_driven(
        scratch: &Scratch,
        sftp_server: PathBuf,
        after: AfterTheRestart,
    ) -> WhatTheRestartLeft {
        the_restart_is_driven_on(
            scratch,
            sftp_server,
            after,
            a_machine_with_a_card_and_no_driver(),
        )
    }

    /// The same, on a machine whose script is given — **which is how
    /// `Server Setup` R1 item 5's two branches are told apart**, the branch
    /// being read off the driver check's own exit.
    fn the_restart_is_driven_on(
        scratch: &Scratch,
        sftp_server: PathBuf,
        after: AfterTheRestart,
        before_the_boot: Vec<(String, CommandResult)>,
    ) -> WhatTheRestartLeft {
        the_restart_is_driven_with(
            scratch,
            sftp_server,
            after,
            before_the_boot,
            TheRestartCommand::AnswersAndThenDrops,
        )
    }

    /// The same, with what the restart command does on the machine given.
    fn the_restart_is_driven_with(
        scratch: &Scratch,
        sftp_server: PathBuf,
        after: AfterTheRestart,
        before_the_boot: Vec<(String, CommandResult)>,
        the_restart: TheRestartCommand,
    ) -> WhatTheRestartLeft {
        the_restart_is_driven_against(scratch, sftp_server, after, before_the_boot, the_restart, None)
    }

    /// The same, **on a machine that remembers its driver packages** across
    /// the run and across the restart, where one is given. Added 2026-09-30,
    /// eleventh-pass fix piece 10.
    fn the_restart_is_driven_against(
        scratch: &Scratch,
        sftp_server: PathBuf,
        after: AfterTheRestart,
        before_the_boot: Vec<(String, CommandResult)>,
        the_restart: TheRestartCommand,
        drivers: Option<Arc<Mutex<TheDriversOnTheMachine>>>,
    ) -> WhatTheRestartLeft {
        the_restart_is_driven_in_full(
            scratch,
            sftp_server,
            after,
            before_the_boot,
            the_restart,
            drivers,
            AndBeside::default(),
        )
    }

    /// **What a run of `Server Setup` §8 may be given beside its machine.**
    /// Added 2026-10-11 for `Server Setup` R1 item 5's swap; every run before
    /// that date takes the default, which is exactly what they were driven
    /// with: no Blender, the machine that came back answering as it always
    /// did, and no command that changes the script.
    #[derive(Default)]
    struct AndBeside {
        /// The Blender the run installs.
        blender: Option<&'static str>,
        /// Entries put over the machine that came back's own script.
        came_back_also_says: Vec<(String, CommandResult)>,
        /// A command that changes the script when it ends.
        changes_the_script: Option<TheSlowCommand>,
        /// **`Server Setup` R1b: the Blenders the search found on the server**,
        /// which item 5's bar is taken over beside the one the run installs.
        /// Added 2026-10-12.
        found: Vec<&'static str>,
        /// **How many jobs the machine's queue holds waiting at each reading**,
        /// in turn.
        waiting_at_each_queue_reading: Vec<usize>,
        /// **How many jobs held for the driver every queue reading holds.**
        held_for_the_driver: usize,
        /// **The local server identifier the run is asked for**, which this
        /// desktop's transfers with the machine are read by.
        the_server: Option<String>,
        /// **Driven as `Server Setup` R1b's swap run** on a machine this copy
        /// set up, rather than as the ready action on a bare one.
        as_the_swap_run: bool,
    }

    fn the_restart_is_driven_in_full(
        scratch: &Scratch,
        sftp_server: PathBuf,
        after: AfterTheRestart,
        before_the_boot: Vec<(String, CommandResult)>,
        the_restart: TheRestartCommand,
        drivers: Option<Arc<Mutex<TheDriversOnTheMachine>>>,
        beside: AndBeside,
    ) -> WhatTheRestartLeft {
        let release = a_release();
        let hosting = WhatHostingWouldServe {
            table: vec![
                (
                    WHERE_THE_CURRENT_RELEASE_IS_READ_FROM.to_string(),
                    release.offered.canonical_bytes(),
                ),
                (THE_ARTIFACTS_SOURCE.to_string(), release.artifact.clone()),
            ],
            asked: Vec::new(),
        };

        let asked = Arc::new(Mutex::new(Vec::new()));
        let sessions = Arc::new(Mutex::new(0usize));
        let supervisor = Arc::new(Mutex::new(TheSupervisorHere {
            waiting_at_each_queue_reading: beside.waiting_at_each_queue_reading.clone(),
            held_for_the_driver: beside.held_for_the_driver,
            ..TheSupervisorHere::default()
        }));
        let goes_down = Arc::new(tokio::sync::Notify::new());
        // **`Servers` S4's mark on a standing connected period**, which is the
        // one this run is driven under. `Server Setup` T4 sets it before the
        // restart command leaves, and this file never sets it itself.
        let period = smoozy_servers::restart::TheRestartMark::none_is_set();
        let at_the_restart = Arc::new(Mutex::new(None));
        let when_a_command_arrived = Arc::new(Mutex::new(Vec::new()));
        let written_when_the_restart_arrived = Arc::new(Mutex::new(None));
        let mut before = TheFarEnd {
            sftp_server: sftp_server.clone(),
            at: scratch.at.clone(),
            said: Arc::new(Mutex::new(before_the_boot)),
            asked: Arc::clone(&asked),
            supervisor: Arc::clone(&supervisor),
            children: Arc::new(Mutex::new(Vec::new())),
            goes_down: Arc::clone(&goes_down),
            the_mark_when_the_restart_arrived: Some((
                period.clone(),
                Arc::clone(&at_the_restart),
            )),
            the_state_when_a_command_arrived: Arc::clone(&when_a_command_arrived),
            the_restart_takes_the_connection: the_restart == TheRestartCommand::TakesTheConnection,
            written_when_the_restart_arrived: Arc::clone(&written_when_the_restart_arrived),
            slow: beside.changes_the_script,
            drivers: drivers.clone(),
        };
        let mut came_back = the_machine_that_came_back();
        for (named, said) in beside.came_back_also_says {
            came_back.retain(|(one, _)| *one != named);
            came_back.push((named, said));
        }
        let mut after_the_boot = TheFarEnd {
            sftp_server,
            at: scratch.at.clone(),
            said: Arc::new(Mutex::new(came_back)),
            asked: Arc::clone(&asked),
            supervisor: Arc::clone(&supervisor),
            children: Arc::new(Mutex::new(Vec::new())),
            goes_down: Arc::new(tokio::sync::Notify::new()),
            the_mark_when_the_restart_arrived: None,
            the_state_when_a_command_arrived: Arc::clone(&when_a_command_arrived),
            the_restart_takes_the_connection: false,
            written_when_the_restart_arrived: Arc::new(Mutex::new(None)),
            slow: None,
            drivers: drivers.clone(),
        };

        let mut row = TheRowHere {
            row: if beside.as_the_swap_run {
                a_row_with_its_root_at(scratch)
            } else {
                a_row()
            },
            writes: Arc::new(Mutex::new(Vec::new())),
        };
        let placement = ThePlacementHere {
            reached: WhatThePlacementReached::Verified,
            given: Arc::new(Mutex::new(None)),
            sessions_open_at_the_lend: Arc::new(Mutex::new(None)),
            sessions: Arc::clone(&sessions),
            commits_on_the_machine: None,
            the_work_the_commit_reads: None,
        };
        let mut releases = TheReleaseHere {
            root: release.root,
            hosting,
            store: TheTrustRecordHere::default(),
        };
        let mut taking = TheOpeningReadingsHere {
            said: an_ubuntu_systemd_machine(),
            asked_after: Arc::clone(&asked),
            asked_at: Arc::new(Mutex::new(None)),
        };
        let asked_for = WhatWasAskedFor {
            install_this_blender: beside.blender.map(str::to_string),
            telegram_is_on: false,
            the_blenders_on_the_server: beside
                .found
                .iter()
                .map(|found| smoozy_server_setup::blender::Version::parse(found).expect("a version"))
                .collect(),
            the_server: beside.the_server.clone(),
        };
        let stop_was_asked = Arc::new(Mutex::new(false));
        let stopper = AStopTheUserAsksFor {
            asked: Arc::clone(&stop_was_asked),
        };

        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("a runtime for the run");
        let point = ThisServersSessions::<TheFarEnd>::for_one_server();
        let asking = point.clone();
        let no_additional: TheAdditionalConnection<TheFarEnd> =
            Arc::new(|| Box::pin(async { None }));
        let the_mark = period.clone();
        // **§8's runs never ask for Telegram**, so the step is not in the
        // composition and this is never reached.
        let mut no_telegram = TelegramIsNotOn;
        let the_screen_was_told = Arc::new(Mutex::new(Vec::new()));
        let mut nobody = TheScreenHere {
            told: Arc::clone(&the_screen_was_told),
            words_kept_for: if beside.as_the_swap_run { beside.the_server.clone() } else { None },
        };
        let package_area = format!("{}/smoozy/package", scratch.text());
        let (came_to, the_setup_read_at_the_end) = runtime.block_on(async {
            let driven: Pin<Box<dyn std::future::Future<Output = WhereTheSetUpGot> + '_>> =
                if beside.as_the_swap_run {
                    // **`Server Setup` R1b's swap run**, on a machine this copy
                    // set up: no placement, no Blender, items 5 and 6 alone.
                    Box::pin(smoozy_desktop::setting_up::the_swap_is_made(
                        &asking,
                        &mut taking,
                        &mut row,
                        &asked_for,
                        smoozy_server_setup::elevated::HowElevationIsObtained::Passwordless,
                        &mut nobody,
                        &stopper,
                        Some(the_mark.clone()),
                    ))
                } else {
                    Box::pin(a_machine_is_set_up(
                        &asking,
                        &mut taking,
                        &mut row,
                        placement,
                        &mut releases,
                        &asked_for,
                        smoozy_server_setup::elevated::HowElevationIsObtained::Passwordless,
                        &mut no_telegram,
                        &mut nobody,
                        &stopper,
                        Some(the_mark.clone()),
                        // **A bare machine, as the connection read it**: §8's
                        // restart is met on R2's whole order.
                        Some(InstallationCheck::Absent),
                    ))
                };
            tokio::pin!(driven);

            // ── The run, up to the moment the restart reaches the machine.
            //
            // **Watched as the command arriving, not as a boundary landing.**
            // Until 2026-09-27 the machine went down once *restart issued* had
            // landed, which assumed the order this fix reverses — and which
            // no real machine obliged.
            let lending = lend_until_it_ends(&point, &mut before, &no_additional);
            tokio::pin!(lending);
            let the_restart_arrived = async {
                loop {
                    let arrived = asked
                        .lock()
                        .expect("the asks")
                        .iter()
                        .any(|said| said.contains("systemctl reboot"));
                    if arrived {
                        return;
                    }
                    tokio::time::sleep(std::time::Duration::from_millis(2)).await;
                }
            };
            tokio::pin!(the_restart_arrived);
            // **The lending is polled first**, so the point lends before the run
            // asks it for anything. **Where the reboot takes the connection with
            // it, the lending ends in the same moment the command arrives**, and
            // that is the one way it may end here.
            let the_restart_has_arrived = |asked: &Arc<Mutex<Vec<String>>>| {
                asked
                    .lock()
                    .expect("the asks")
                    .iter()
                    .any(|said| said.contains("systemctl reboot"))
            };
            let mut the_lending_ended = false;
            let early = tokio::select! {
                biased;
                _ended = &mut lending => {
                    assert!(
                        the_restart_has_arrived(&asked),
                        "the machine went down before the restart"
                    );
                    the_lending_ended = true;
                    None
                }
                came_to = &mut driven => Some(came_to),
                () = &mut the_restart_arrived => None,
            };
            // **A refused restart leaves the machine up**, and the run is
            // waited on over the same connection — it may already have ended
            // by the time the command is seen to arrive — which then reads the
            // setup back as the Set up tab does.
            if the_restart == TheRestartCommand::IsRefused {
                assert!(!the_lending_ended, "a refused restart took the machine down");
                let came_to = match early {
                    Some(came_to) => came_to,
                    None => tokio::select! {
                        biased;
                        _ended = &mut lending => panic!("a refused restart took the machine down"),
                        came_to = &mut driven => came_to,
                    },
                };
                let read = {
                    let reading =
                        smoozy_desktop::setting_up::the_setup_as_read(&asking, &package_area, None);
                    tokio::pin!(reading);
                    tokio::select! {
                        biased;
                        _ended = &mut lending => panic!("a refused restart took the machine down"),
                        read = &mut reading => read,
                    }
                };
                return (came_to, read);
            }
            if let Some(came_to) = early {
                return (came_to, None);
            }

            // **The machine goes down**, which is what the restart the desktop
            // just issued does to the connection. **A permit rather than a call
            // to whoever is listening**, so a far end that is mid-request still
            // goes down when it is done. Where the restart took the connection
            // with it, the far end has already said so and this is a second
            // permit nobody reads.
            goes_down.notify_one();
            if !the_lending_ended {
                let during = tokio::select! {
                    biased;
                    _ended = &mut lending => None,
                    came_to = &mut driven => Some(came_to),
                };
                if let Some(came_to) = during {
                    return (came_to, None);
                }
            }

            // ── The wait, and how it ends.
            let came_to = match after {
                AfterTheRestart::TheMachineAnswers => {
                    let lending = lend_until_it_ends(&point, &mut after_the_boot, &no_additional);
                    tokio::pin!(lending);
                    let clears = async {
                        // **Nothing is cleared until the machine really
                        // answers**, so the lending is live first.
                        while !asking.a_connection_is_live() {
                            tokio::time::sleep(std::time::Duration::from_millis(2)).await;
                        }
                        the_mark
                            .cleared_by(smoozy_servers::restart::HowTheWaitEnds::TheMachineAnswers);
                        std::future::pending::<std::convert::Infallible>().await
                    };
                    tokio::pin!(clears);
                    tokio::select! {
                        biased;
                        _ended = &mut lending => panic!("the machine that came back went down again"),
                        came_to = &mut driven => came_to,
                        never = &mut clears => match never {},
                    }
                }
                AfterTheRestart::ThePolicyGivesUp => {
                    the_mark.cleared_by(smoozy_servers::restart::HowTheWaitEnds::ThePolicyGivesUp);
                    driven.await
                }
                AfterTheRestart::TheUserStopsWaiting => {
                    *stop_was_asked.lock().expect("the stop") = true;
                    driven.await
                }
            };
            (came_to, None)
        });

        let the_asks = asked.lock().expect("the asks").clone();
        let at_the_restart = *at_the_restart.lock().expect("the mark");
        let the_order_of_commands = when_a_command_arrived.lock().expect("the order").clone();
        let the_order_at_the_restart = written_when_the_restart_arrived
            .lock()
            .expect("the order")
            .clone();
        let the_screen_told = the_screen_was_told.lock().expect("the screen").clone();
        let held = supervisor.lock().expect("the supervisor");
        WhatTheRestartLeft {
            came_to,
            asked: the_asks,
            written_from_the_desktop: held.written_from_the_desktop.clone(),
            the_mark: period.where_it_stands(),
            the_mark_when_the_restart_arrived: at_the_restart,
            took_over: held.took_over.clone(),
            kinds: held.kinds.clone(),
            runs: held.opened.clone(),
            the_stop_was_written_for: held.the_stop_was_written_for.clone(),
            the_state_when_a_command_arrived: the_order_of_commands,
            the_step_started_when_a_command_arrived: held
                .the_step_started_when_a_command_arrived
                .clone(),
            written_when_the_restart_arrived: the_order_at_the_restart,
            the_screen_was_told: the_screen_told,
            failures_recorded: held.failures_recorded.clone(),
            the_setup_read_at_the_end,
            set_by: held.set_by.clone(),
            cleared_by: held.cleared_by.clone(),
            queue_readings: held.queue_readings,
        }
    }

    /// **The mark is set before the restart command leaves, and it is set
    /// through `Server Setup`'s own witness.**
    ///
    /// **Row 107's fault is exactly the opposite order**: a restart issued with
    /// no mark set is read by `Servers` as an ordinary lost connection, and the
    /// whole-app *could not restore* notification fires on a drop Smoozy
    /// caused. **So the order is the assertion**: the mark stood before
    /// `systemctl reboot` reached the machine.
    #[test]
    fn the_mark_is_set_before_the_restart_command_leaves() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("restart-mark");
        let left = the_restart_is_driven(&scratch, sftp_server, AfterTheRestart::ThePolicyGivesUp);

        // The restart really went out, and it is the one command `Server Setup`
        // T1 names.
        assert!(
            left.asked.iter().any(|said| said.contains("systemctl reboot")),
            "no restart was issued: {:?}",
            left.asked
        );
        // **`Server Setup` Y4's *driver placed* and *restart issued*, and both
        // were written before the command reached the machine** — the paragraph
        // under Y4's table: *restart issued* is written just before the command
        // is sent, because nothing can be written after it (`Server Setup` T3).
        // *(Until 2026-09-27 this asserted the mark was written after the
        // command returned, which is the order that lost the recovery on every
        // real machine.)*
        let written: Vec<&str> = left
            .written_from_the_desktop
            .iter()
            .map(|(boundary, _)| boundary.as_str())
            .collect();
        assert_eq!(
            written,
            vec![asked::THE_BOUNDARY_DRIVER_PLACED, asked::THE_BOUNDARY_RESTART_ISSUED],
            "the boundaries around the restart are not `Server Setup` Y4's two"
        );
        assert_eq!(
            left.written_when_the_restart_arrived,
            Some(vec![
                asked::THE_BOUNDARY_DRIVER_PLACED.to_string(),
                asked::THE_BOUNDARY_RESTART_ISSUED.to_string(),
            ]),
            "*restart issued* was not durable when the restart reached the machine"
        );
        // **The mark stood at the instant the command reached the machine.**
        // Nothing in this file sets it — the driving does, through
        // `Server Setup`'s own witness — and reading it here rather than
        // afterwards is what makes this an assertion about the order.
        assert_eq!(
            left.the_mark_when_the_restart_arrived,
            Some(smoozy_servers::restart::WhereTheMarkStands::ItStands),
            "the restart went out with no mark set, which is Row 107's fault"
        );
    }

    /// **`Server Setup` Y7's second bullet at the driver step: the driver
    /// install reaches the machine with the step's start already recorded
    /// against the run**, so a connection cut during the install leaves a
    /// record `Server Setup` S1 reads as *stopped part-way* at the driver step.
    /// Added 2026-09-29, step 10c's transfer wave piece 12 — the eighth pass on
    /// Hyperstack saw *Install the graphics driver — not started* over a driver
    /// install that had been cut.
    ///
    /// **And the restart is not given one**: its own *restart issued* is
    /// written just before its command (`Server Setup` T3), which is its start.
    #[test]
    fn the_driver_install_arrives_with_the_driver_steps_start_recorded() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("driver-step-start");
        let left = the_restart_is_driven(&scratch, sftp_server, AfterTheRestart::TheMachineAnswers);
        let install = left
            .the_step_started_when_a_command_arrived
            .iter()
            .find(|(said, _)| said.contains("ubuntu-drivers") && said.contains("install"))
            .unwrap_or_else(|| {
                panic!("no driver install went out: {:?}", left.asked)
            });
        assert_eq!(
            install.1.as_deref(),
            Some(asked::THE_BOUNDARY_DRIVER_PLACED),
            "the driver install reached the machine with no step recorded as started"
        );
        let restart = left
            .the_step_started_when_a_command_arrived
            .iter()
            .find(|(said, _)| said.contains("systemctl reboot"))
            .expect("the restart went out");
        assert_eq!(
            restart.1, None,
            "the restart arrived under the driver step's start, which *driver placed* ends"
        );
    }

    /// **`Server Setup` R1 item 5's harm, held by a test that names it: on a
    /// machine whose driver is already installed, no install command leaves the
    /// desk and the restart is tried first.**
    ///
    /// **The clause's own words say why.** *"Where a driver is already
    /// installed, the restart is tried before any install — the two observed
    /// failure mechanisms (a card still bound to `nouveau`; a module built for a
    /// kernel not yet booted) are both fixed by the boot, and **a reinstall on a
    /// working-but-unbooted install silently becomes an upgrade the running
    /// kernel does not match**."* `Server Setup` R5 names a repeated driver
    /// install as the exact shape of *running it again makes it worse*.
    ///
    /// **The driving could not take this branch at all until 2026-09-20.** It
    /// kept R1 item 5 its own way, with no already-installed case in it, so
    /// every machine whose check failed got the distribution's install —
    /// including this one, which is the one machine the clause was written for.
    #[test]
    fn a_driver_already_installed_is_restarted_and_never_reinstalled() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("driver-installed");
        let left = the_restart_is_driven_on(
            &scratch,
            sftp_server,
            AfterTheRestart::TheMachineAnswers,
            a_machine_whose_driver_is_installed_and_not_loaded(),
        );

        // **No install left the desk at all**, on either family's route.
        assert!(
            !left.asked.iter().any(|said| said.contains("ubuntu-drivers")
                || said.contains("nvidia-driver")
                || said.contains("nvidia-detect")),
            "a driver install went out on a machine that already has one: {:?}",
            left.asked
        );
        // **And the restart was tried**, which is what the branch exists for.
        assert!(
            left.asked.iter().any(|said| said.contains("systemctl reboot")),
            "no restart was tried: {:?}",
            left.asked
        );
        // **`Server Setup` Y4a's mark: the driver step performed nothing.**
        assert_eq!(
            left.written_from_the_desktop
                .iter()
                .find(|(boundary, _)| boundary == asked::THE_BOUNDARY_DRIVER_PLACED)
                .map(|(_, mark)| mark.as_str()),
            Some(asked::IT_HAD_NOTHING_TO_DO),
            "the driver step claimed an act it did not perform"
        );
        // **`Server Setup` Y1's class is still honest, and this branch is
        // where it could have stopped being.** The driver step performs nothing
        // here, so nothing is set on its account; the restart that follows is
        // one of `Server Setup` Y1's three acts, and
        // `Rented-server runtime` P4 makes the state durable **before** it.
        let at_the_restart = left
            .the_state_when_a_command_arrived
            .iter()
            .position(|(said, _)| said.contains("systemctl reboot"))
            .expect("the restart went out");
        assert!(
            left.the_state_when_a_command_arrived[at_the_restart].1.is_some(),
            "the restart reached the machine before the state was set: {:?}",
            left.the_state_when_a_command_arrived
        );
        // **Every command of the machine's own before the restart went out
        // with no state standing.** The framed requests to Smoozy's own program
        // are not the machine's acts, and from 2026-09-27 two of them rightly
        // follow the state's set and precede the restart: *restart issued*,
        // which guarantees *"the state is durable"* and is written just before
        // the command (the paragraph under `Server Setup` Y4's table), and the
        // screen's reading of the run.
        assert!(
            left.the_state_when_a_command_arrived[..at_the_restart]
                .iter()
                .filter(|(said, _)| !said.ends_with("/smoozy-server"))
                .all(|(_, standing)| standing.is_none()),
            "the state was set before the restart was the first machine-changing act: {:?}",
            left.the_state_when_a_command_arrived
        );

        // **And the run went on to *card answering* on the boot that came
        // back**, which is `Server Setup` T5 and S4 item 2 unchanged.
        let written: Vec<&str> = left
            .written_from_the_desktop
            .iter()
            .map(|(boundary, _)| boundary.as_str())
            .collect();
        assert_eq!(
            written,
            vec![
                asked::THE_BOUNDARY_DRIVER_PLACED,
                asked::THE_BOUNDARY_RESTART_ISSUED,
                asked::THE_BOUNDARY_CARD_ANSWERING,
                asked::THE_BOUNDARY_DONE,
            ],
            "the boundaries are not `Server Setup` Y4's for this run"
        );
        assert!(
            matches!(
                left.came_to,
                WhereTheSetUpGot::TheRunEnded {
                    ending: TheRunsEnding::ItReachedDone { .. },
                    ..
                }
            ),
            "the run did not reach done: {:?}",
            left.came_to
        );
    }

    /// **Machine 1 of the eleventh pass, replayed: after a cut driver install,
    /// *Set up again* installs one branch and that branch's own utilities, and
    /// the machine answers.** `Server Setup` R1 item 5's fourth act as
    /// corrected on 2026-09-30, and E7's fifth carriage. Added that day,
    /// eleventh-pass fix piece 10.
    ///
    /// **What the pass watched instead**: the second run's install chose 580,
    /// its utilities act asked the *recommended* list and installed
    /// `nvidia-utils-595`, the two removed each other's modules, and the
    /// restart came back with no module for any driver — *"the server came
    /// back and its graphics card still does not answer"*. The third run then
    /// took the restart before any install and failed the same way, so no
    /// press could mend the machine.
    ///
    /// **The first run is the machine it left** rather than a run driven here:
    /// the cut itself, and the record it leaves, are piece 12's and the
    /// transfer wave's own tests. What this proves is what the next two runs
    /// do to the machine.
    #[test]
    fn after_a_cut_install_set_up_again_installs_one_branch_and_its_own_utilities() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let machine = Arc::new(Mutex::new(
            TheDriversOnTheMachine::machine_one_after_the_cut_install(),
        ));

        // ── The second run: *Set up again*.
        let scratch = Scratch::new("machine-one-second");
        let second = the_restart_is_driven_against(
            &scratch,
            sftp_server.clone(),
            AfterTheRestart::TheMachineAnswers,
            a_machine_with_a_card_and_no_driver(),
            TheRestartCommand::AnswersAndThenDrops,
            Some(Arc::clone(&machine)),
        );
        let after_the_second = machine.lock().expect("the drivers").clone();
        // **And from 2026-10-03 the same branch's OptiX library**, `Server
        // Setup` R1 item 5's fifth act, after the utilities.
        assert_eq!(
            after_the_second.performed,
            vec![
                "driver 580".to_string(),
                "utilities 580".to_string(),
                "OptiX library 580".to_string(),
            ],
            "the install, its utilities and its OptiX library named two branches: {:?}",
            second.asked
        );
        assert_eq!(after_the_second.optix_library, Some("580-server"));
        assert_eq!(after_the_second.loaded, Some("580-server-open"));
        assert!(
            matches!(
                second.came_to,
                WhereTheSetUpGot::TheRunEnded {
                    ending: TheRunsEnding::ItReachedDone { .. },
                    ..
                }
            ),
            "the second run did not reach done: {:?}",
            second.came_to
        );

        // ── The third run: *Set up again* once more, on a machine that works.
        let scratch = Scratch::new("machine-one-third");
        let third = the_restart_is_driven_against(
            &scratch,
            sftp_server,
            AfterTheRestart::TheMachineAnswers,
            a_machine_with_a_card_and_no_driver(),
            TheRestartCommand::AnswersAndThenDrops,
            Some(Arc::clone(&machine)),
        );
        assert_eq!(
            machine.lock().expect("the drivers").performed,
            after_the_second.performed,
            "the third run put packages on a machine whose driver answered"
        );
        assert!(
            !third.asked.iter().any(|said| said.contains("systemctl reboot")),
            "the third run restarted a machine whose driver answered: {:?}",
            third.asked
        );
        assert!(
            matches!(
                third.came_to,
                WhereTheSetUpGot::TheRunEnded {
                    ending: TheRunsEnding::ItReachedDone { .. },
                    ..
                }
            ),
            "the third run did not reach done: {:?}",
            third.came_to
        );
    }

    /// **The dead state machine 1 was destroyed in — a driver's utilities
    /// with no kernel module for the running kernel — takes the install
    /// route on *Set up again*, not the restart.** `Server Setup` R1 item 5:
    /// *"A driver counts as installed for this rule only where the running
    /// kernel has its module"*, and *"where the check fails and the running
    /// kernel has no module for the driver, the install route runs"*. Added
    /// 2026-09-30, eleventh-pass fix piece 10.
    ///
    /// **Before the fix** the check's exit 9 alone read as *a driver is
    /// installed*, so every run restarted, came back with nothing to load, and
    /// ended the same way.
    #[test]
    fn a_driver_with_no_module_for_the_running_kernel_is_installed_again() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let machine = Arc::new(Mutex::new(TheDriversOnTheMachine::the_dead_state()));
        let scratch = Scratch::new("dead-state");
        let left = the_restart_is_driven_against(
            &scratch,
            sftp_server,
            AfterTheRestart::TheMachineAnswers,
            a_machine_with_a_card_and_no_driver(),
            TheRestartCommand::AnswersAndThenDrops,
            Some(Arc::clone(&machine)),
        );
        let install = left
            .asked
            .iter()
            .position(|said| said.contains("ubuntu-drivers install --gpgpu"));
        let restart = left
            .asked
            .iter()
            .position(|said| said.contains("systemctl reboot"));
        assert!(
            install.is_some() && install < restart,
            "the restart was tried before any install on a machine with no module: {:?}",
            left.asked
        );
        assert_eq!(
            machine.lock().expect("the drivers").performed,
            vec![
                "driver 595".to_string(),
                "utilities 595".to_string(),
                "OptiX library 595".to_string(),
            ]
        );
        assert!(
            matches!(
                left.came_to,
                WhereTheSetUpGot::TheRunEnded {
                    ending: TheRunsEnding::ItReachedDone { .. },
                    ..
                }
            ),
            "the run did not reach done: {:?}",
            left.came_to
        );
    }

    // ─────────────── `Server Setup` R1 item 5's swap up to the OptiX bar (2026-10-11)

    /// `Servers` H11b's version query, whole.
    const THE_VERSION_QUERY: &str = "nvidia-smi --query-gpu=driver_version --format=csv,noheader";

    /// **Hyperstack's R550 image as the OptiX check met it**: Ubuntu, the
    /// systemd kind, a card on the bus, and a driver that answers at
    /// 550.163.01.
    fn a_machine_whose_driver_answers_at_550() -> Vec<(String, CommandResult)> {
        let mut script = the_driver_answers(
            a_machine_with_a_card_and_no_driver(),
            ended_cleanly("GPU-1111,0000:01:00.0,NVIDIA RTX A6000,49140 MiB\n"),
        );
        script.push((THE_VERSION_QUERY.to_string(), ended_cleanly("550.163.01\n")));
        script
    }

    fn the_swap() -> String {
        smoozy_server_setup::elevated::THE_UBUNTU_DRIVER_SWAP
            .elevated
            .as_str()
            .to_string()
    }

    fn it_reached_done_and_missed(came_to: &WhereTheSetUpGot) -> Option<bool> {
        match came_to {
            WhereTheSetUpGot::TheRunEnded {
                ending:
                    TheRunsEnding::ItReachedDone {
                        the_driver_missed_the_optix_bar,
                        ..
                    },
                ..
            } => Some(*the_driver_missed_the_optix_bar),
            _ => None,
        }
    }

    fn the_mark_at(left: &WhatTheRestartLeft, boundary: &str) -> Option<String> {
        left.written_from_the_desktop
            .iter()
            .find(|(written, _)| written == boundary)
            .map(|(_, mark)| mark.clone())
    }

    /// **550.163.01 before, 580.178.04 after: swapped, restarted, and the
    /// check on return passes.** The swap is the one act; nothing of the
    /// family's install route goes out; `Server Setup` Y1's state stands before
    /// it; the restart follows because the old module is still loaded; and the
    /// run reaches *done* with nothing missed.
    #[test]
    fn a_driver_below_the_bar_is_swapped_restarted_and_passes() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("swap-reaches-the-bar");
        let mismatch = CommandResult {
            output: b"Failed to initialize NVML: Driver/library version mismatch\n".to_vec(),
            complaint: Vec::new(),
            status: Some(18),
        };
        let left = the_restart_is_driven_in_full(
            &scratch,
            sftp_server,
            AfterTheRestart::TheMachineAnswers,
            a_machine_whose_driver_answers_at_550(),
            TheRestartCommand::AnswersAndThenDrops,
            None,
            AndBeside {
                blender: Some("5.2.2"),
                came_back_also_says: vec![(
                    THE_VERSION_QUERY.to_string(),
                    ended_cleanly("580.178.04\n"),
                )],
                changes_the_script: Some(TheSlowCommand {
                    command: the_swap(),
                    how_long: std::time::Duration::from_millis(1),
                    after_it: the_driver_answers(Vec::new(), mismatch),
                    the_user_stops_as_it_starts: None,
                    the_machine_ends_the_run_as_it_starts: false,
                    what_became_of_it: Arc::new(Mutex::new(Vec::new())),
                }),
                ..AndBeside::default()
            },
        );
        let swap = left.asked.iter().position(|said| *said == the_swap());
        let restart = left.asked.iter().position(|said| said.contains("systemctl reboot"));
        assert!(
            swap.is_some() && swap < restart,
            "the swap did not go out before the restart: {:?}",
            left.asked
        );
        assert!(
            !left.asked.iter().any(|said| said.contains("ubuntu-drivers")),
            "the family's install route went out beside the swap: {:?}",
            left.asked
        );
        let swap = swap.expect("the swap");
        assert!(
            left.the_state_when_a_command_arrived
                .iter()
                .find(|(said, _)| *said == the_swap())
                .is_some_and(|(_, standing)| standing.is_some()),
            "the swap reached the machine before Y1's state: {:?}",
            &left.asked[..=swap]
        );
        assert_eq!(
            the_mark_at(&left, asked::THE_BOUNDARY_DRIVER_PLACED).as_deref(),
            Some(asked::IT_PERFORMED)
        );
        assert!(the_mark_at(&left, asked::THE_BOUNDARY_CARD_ANSWERING).is_some());
        assert_eq!(
            it_reached_done_and_missed(&left.came_to),
            Some(false),
            "the run did not reach done at the bar: {:?}",
            left.came_to
        );
    }

    /// **550 before and after: the swap was refused** — here, no fabric
    /// manager for the new branch in the release's archive — **so the old
    /// driver still answers.** No restart goes out, setup does not fail for it,
    /// and *done* carries that the bar was missed. **And a swap that went
    /// through and came back still below the bar** is restarted, and recorded
    /// the same way after the check on return.
    #[test]
    fn a_swap_that_misses_the_bar_is_recorded_and_setup_does_not_fail() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("swap-refused");
        let mut script = a_machine_whose_driver_answers_at_550();
        script.push((
            the_swap(),
            CommandResult {
                output: Vec::new(),
                complaint: b"E: Unable to locate package nvidia-fabricmanager-580\n".to_vec(),
                status: Some(100),
            },
        ));
        let left = the_restart_is_driven_in_full(
            &scratch,
            sftp_server.clone(),
            AfterTheRestart::TheMachineAnswers,
            script,
            TheRestartCommand::AnswersAndThenDrops,
            None,
            AndBeside {
                blender: Some("5.2.2"),
                ..AndBeside::default()
            },
        );
        assert!(left.asked.contains(&the_swap()), "{:?}", left.asked);
        assert!(
            !left.asked.iter().any(|said| said.contains("systemctl reboot")),
            "a restart went out after a swap that put nothing on: {:?}",
            left.asked
        );
        assert!(left.failures_recorded.is_empty(), "{:?}", left.failures_recorded);
        assert_eq!(
            it_reached_done_and_missed(&left.came_to),
            Some(true),
            "the missed bar did not reach the desk: {:?}",
            left.came_to
        );

        // **Went through, came back below the bar.**
        let scratch = Scratch::new("swap-still-below");
        let mut script = a_machine_whose_driver_answers_at_550();
        script.push((the_swap(), ended_cleanly("")));
        let left = the_restart_is_driven_in_full(
            &scratch,
            sftp_server,
            AfterTheRestart::TheMachineAnswers,
            script,
            TheRestartCommand::AnswersAndThenDrops,
            None,
            AndBeside {
                blender: Some("5.2.2"),
                came_back_also_says: vec![(
                    THE_VERSION_QUERY.to_string(),
                    ended_cleanly("550.163.01\n"),
                )],
                ..AndBeside::default()
            },
        );
        assert!(
            left.asked.iter().any(|said| said.contains("systemctl reboot")),
            "a swap that went through and still read below the bar was not restarted: {:?}",
            left.asked
        );
        assert_eq!(it_reached_done_and_missed(&left.came_to), Some(true), "{:?}", left.came_to);
    }

    /// **Nothing is touched where nothing is below the bar**: a run installing
    /// only 5.1 on a 550 driver reads 535's row; a driver already at the bar
    /// is left alone; and a run installing no Blender compares nothing.
    #[test]
    fn a_driver_at_its_runs_bar_is_never_touched() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let mut at_the_bar = a_machine_whose_driver_answers_at_550();
        at_the_bar.retain(|(named, _)| named != THE_VERSION_QUERY);
        at_the_bar.push((THE_VERSION_QUERY.to_string(), ended_cleanly("595.91.07\n")));
        for (name, script, blender) in [
            ("only-5-1", a_machine_whose_driver_answers_at_550(), Some("5.1.2")),
            ("at-the-bar", at_the_bar, Some("5.2.2")),
            ("no-blender", a_machine_whose_driver_answers_at_550(), None),
        ] {
            let scratch = Scratch::new(name);
            let left = the_restart_is_driven_in_full(
                &scratch,
                sftp_server.clone(),
                AfterTheRestart::TheMachineAnswers,
                script,
                TheRestartCommand::AnswersAndThenDrops,
                None,
                AndBeside {
                    blender,
                    ..AndBeside::default()
                },
            );
            assert!(
                !left.asked.iter().any(|said| *said == the_swap()
                    || said.contains("nvidia-driver")
                    || said.contains("ubuntu-drivers")
                    || said.contains("systemctl reboot")),
                "{name}: the driver was touched: {:?}",
                left.asked
            );
            assert_eq!(
                the_mark_at(&left, asked::THE_BOUNDARY_DRIVER_PLACED).as_deref(),
                Some(asked::IT_HAD_NOTHING_TO_DO),
                "{name}"
            );
            assert_eq!(it_reached_done_and_missed(&left.came_to), Some(false), "{name}: {:?}", left.came_to);
        }
    }

    // ═════════════════ `Server Setup` R1b, 2026-10-12: the driver follows every Blender

    /// **What a swap that goes through and comes back at the bar is given**:
    /// the machine that came back answers 580.178.04, and the swap leaves the
    /// old module loaded until the restart.
    fn a_swap_that_reaches_the_bar(found: Vec<&'static str>, blender: Option<&'static str>) -> AndBeside {
        let mismatch = CommandResult {
            output: b"Failed to initialize NVML: Driver/library version mismatch\n".to_vec(),
            complaint: Vec::new(),
            status: Some(18),
        };
        AndBeside {
            blender,
            found,
            came_back_also_says: vec![(THE_VERSION_QUERY.to_string(), ended_cleanly("580.178.04\n"))],
            changes_the_script: Some(TheSlowCommand {
                command: the_swap(),
                how_long: std::time::Duration::from_millis(1),
                after_it: the_driver_answers(Vec::new(), mismatch),
                the_user_stops_as_it_starts: None,
                the_machine_ends_the_run_as_it_starts: false,
                what_became_of_it: Arc::new(Mutex::new(Vec::new())),
            }),
            ..AndBeside::default()
        }
    }

    fn the_swap_waits_after(came_to: &WhereTheSetUpGot) -> Option<bool> {
        match came_to {
            WhereTheSetUpGot::TheRunEnded {
                ending: TheRunsEnding::ItReachedDone { the_swap_waits, .. },
                ..
            } => Some(*the_swap_waits),
            _ => None,
        }
    }

    /// **`Server Setup` R1b: the bar is the newest Blender on the server's,
    /// taken over every Blender the search found — and a ready action that
    /// installs none compares against them.** A 550 driver on a machine
    /// holding 5.2.2 is swapped, restarted and passes, with no Blender asked
    /// for. Before `Server Setup` R1b a run installing nothing compared nothing.
    #[test]
    fn the_bar_is_taken_over_every_blender_found_and_with_no_install() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("r1b-no-install");
        let left = the_restart_is_driven_in_full(
            &scratch,
            sftp_server,
            AfterTheRestart::TheMachineAnswers,
            a_machine_whose_driver_answers_at_550(),
            TheRestartCommand::AnswersAndThenDrops,
            None,
            a_swap_that_reaches_the_bar(vec!["4.5.3", "5.2.2"], None),
        );
        let swap = left.asked.iter().position(|said| *said == the_swap());
        let restart = left.asked.iter().position(|said| said.contains("systemctl reboot"));
        assert!(swap.is_some() && swap < restart, "no swap before the restart: {:?}", left.asked);
        assert_eq!(it_reached_done_and_missed(&left.came_to), Some(false), "{:?}", left.came_to);
        assert_eq!(the_swap_waits_after(&left.came_to), Some(false));
        assert_eq!(left.queue_readings, 2, "the live set is read before the swap and at the last guard");
    }

    /// **What the Sesterce pass's machine answered after the swap's removal
    /// stopped**: a card, no `nvidia-smi`, and the kernel half standing.
    fn a_machine_a_removal_stopped_on() -> Vec<(String, CommandResult)> {
        let mut script = the_driver_answers(
            a_machine_with_a_card_and_no_driver(),
            CommandResult {
                output: Vec::new(),
                complaint: b"bash: line 1: nvidia-smi: command not found\n".to_vec(),
                status: Some(127),
            },
        );
        let reading = smoozy_server_setup::elevated::THE_DRIVER_PACKAGES_READING.as_str().to_string();
        script.retain(|(named, _)| *named != reading);
        script.push((reading, ended_cleanly("nvidia-dkms-570-open\nnvidia-kernel-source-570-open\n")));
        script
    }

    /// **`Server Setup` R1 item 5, as written 2026-11-03: *Set up again* on a
    /// half-removed driver finishes the removal, installs, restarts and reaches
    /// *done*** — through the whole run, with no detection route. Until then this
    /// machine read *done* after *0 upgraded*, restarted, and failed its check.
    #[test]
    fn a_setup_on_a_half_removed_driver_finishes_the_removal_and_reaches_done() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("half-removed");
        let finish = smoozy_server_setup::elevated::THE_UBUNTU_DRIVER_SWAPS_FINISH.elevated.as_str().to_string();
        let mismatch = CommandResult {
            output: b"Failed to initialize NVML: Driver/library version mismatch\n".to_vec(),
            complaint: Vec::new(),
            status: Some(18),
        };
        let left = the_restart_is_driven_in_full(
            &scratch,
            sftp_server,
            AfterTheRestart::TheMachineAnswers,
            a_machine_a_removal_stopped_on(),
            TheRestartCommand::AnswersAndThenDrops,
            None,
            AndBeside {
                came_back_also_says: vec![(THE_VERSION_QUERY.to_string(), ended_cleanly("580.178.04\n"))],
                changes_the_script: Some(TheSlowCommand {
                    command: finish.clone(),
                    how_long: std::time::Duration::from_millis(1),
                    after_it: the_driver_answers(Vec::new(), mismatch),
                    the_user_stops_as_it_starts: None,
                    the_machine_ends_the_run_as_it_starts: false,
                    what_became_of_it: Arc::new(Mutex::new(Vec::new())),
                }),
                ..AndBeside::default()
            },
        );
        let finished = left.asked.iter().position(|said| *said == finish);
        let restart = left.asked.iter().position(|said| said.contains("systemctl reboot"));
        assert!(finished.is_some() && finished < restart, "no finish before the restart: {:?}", left.asked);
        assert!(
            !left.asked.iter().any(|said| said.contains("ubuntu-drivers install")),
            "the detection route went out over a stopped removal: {:?}",
            left.asked
        );
        assert!(
            matches!(left.came_to, WhereTheSetUpGot::TheRunEnded { ending: TheRunsEnding::ItReachedDone { .. }, .. }),
            "{:?}",
            left.came_to
        );
    }

    /// **`Server Setup` R1 item 5, as written 2026-11-03: a finish that fails
    /// ends the step in its words, and the machine is not restarted.**
    #[test]
    fn a_finish_that_fails_ends_the_step_and_restarts_nothing() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("half-removed-refused");
        let finish = smoozy_server_setup::elevated::THE_UBUNTU_DRIVER_SWAPS_FINISH.elevated.as_str().to_string();
        let mut script = a_machine_a_removal_stopped_on();
        script.push((
            finish.clone(),
            CommandResult {
                output: Vec::new(),
                complaint: b"E: Unable to correct problems\n".to_vec(),
                status: Some(100),
            },
        ));
        let left = the_restart_is_driven_in_full(
            &scratch,
            sftp_server,
            AfterTheRestart::TheMachineAnswers,
            script,
            TheRestartCommand::AnswersAndThenDrops,
            None,
            AndBeside::default(),
        );
        assert!(!left.asked.iter().any(|said| said.contains("systemctl reboot")), "{:?}", left.asked);
        match &left.came_to {
            WhereTheSetUpGot::TheRunEnded { ending: TheRunsEnding::AStepFailed { said, .. }, .. } => {
                assert!(said.contains("Unable to correct problems"), "{said}");
            }
            other => panic!("the step did not fail in its words: {other:?}"),
        }
    }

    /// **`Server Setup` R1b: where anything is waiting on the server, the swap
    /// is not made in this run, and it waits** — no swap, no restart, no
    /// machine-level state set for it, *driver placed* marked *nothing to do*,
    /// and *done* with the swap waiting and **nothing missed**. **The same
    /// where a transfer is under way between this desktop and the machine**,
    /// the runner's ruling of 2026-10-11.
    #[test]
    fn the_swap_is_left_out_while_work_is_live_or_a_transfer_is_under_way() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let server = Identity::issue(IdentityKind::LocalServer).expect("a server").as_text();
        for (name, waiting, a_transfer) in [("r1b-a-job-waits", vec![1], false), ("r1b-a-transfer", vec![], true)] {
            let scratch = Scratch::new(name);
            if a_transfer {
                smoozy_desktop::servers::the_driver_swap::a_transfer_begins(&server);
            }
            let left = the_restart_is_driven_in_full(
                &scratch,
                sftp_server.clone(),
                AfterTheRestart::TheMachineAnswers,
                a_machine_whose_driver_answers_at_550(),
                TheRestartCommand::AnswersAndThenDrops,
                None,
                AndBeside {
                    waiting_at_each_queue_reading: waiting,
                    the_server: Some(server.clone()),
                    ..a_swap_that_reaches_the_bar(vec!["5.2.2"], None)
                },
            );
            if a_transfer {
                smoozy_desktop::servers::the_driver_swap::a_transfer_ends(&server);
            }
            assert!(
                !left.asked.iter().any(|said| *said == the_swap() || said.contains("systemctl reboot")),
                "{name}: the swap went out over live work: {:?}",
                left.asked
            );
            assert!(left.set_by.is_empty(), "{name}: a state was set for a swap that waits");
            assert_eq!(
                the_mark_at(&left, asked::THE_BOUNDARY_DRIVER_PLACED).as_deref(),
                Some(asked::IT_HAD_NOTHING_TO_DO),
                "{name}"
            );
            assert_eq!(it_reached_done_and_missed(&left.came_to), Some(false), "{name}: {:?}", left.came_to);
            assert_eq!(the_swap_waits_after(&left.came_to), Some(true), "{name}");
        }
    }

    /// **`Server Setup` R1b's last guard: the live set is read again once the
    /// machine-level state stands, and where work arrived since the first
    /// reading the swap is not made** — the run carries on to *done* with
    /// nothing done, the state it set is cleared there, and **the bar is not
    /// recorded as missed**: the swap waits again.
    #[test]
    fn the_second_live_set_check_backs_off_without_recording_a_miss() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("r1b-last-guard");
        let left = the_restart_is_driven_in_full(
            &scratch,
            sftp_server,
            AfterTheRestart::TheMachineAnswers,
            a_machine_whose_driver_answers_at_550(),
            TheRestartCommand::AnswersAndThenDrops,
            None,
            AndBeside {
                waiting_at_each_queue_reading: vec![0, 1],
                ..a_swap_that_reaches_the_bar(vec!["5.2.2"], None)
            },
        );
        assert_eq!(left.queue_readings, 2, "the live set was not read again once the state stood");
        assert!(!left.set_by.is_empty(), "the state was not set before the last guard");
        assert_eq!(left.cleared_by, left.set_by, "the state set for the swap was not cleared at done");
        assert!(
            !left.asked.iter().any(|said| *said == the_swap() || said.contains("systemctl reboot")),
            "the swap went out after the last guard found work: {:?}",
            left.asked
        );
        assert_eq!(it_reached_done_and_missed(&left.came_to), Some(false), "{:?}", left.came_to);
        assert_eq!(the_swap_waits_after(&left.came_to), Some(true));
    }

    /// **`Server Setup` R1b and `Server Setup` B8: an install that leaves the driver below the
    /// newest Blender's bar is followed by the swap as a run of its own.** The
    /// comparison after the install owes a swap; at the empty-queue reading it
    /// starts; and the run is the ready action's own kind, of the readings,
    /// the driver, the restart and *done* — swapped, restarted and passing,
    /// with nothing fetched and nothing published. Its ending then leaves
    /// nothing owed.
    #[test]
    fn the_swap_run_follows_an_install_that_leaves_the_driver_below() {
        use smoozy_desktop::servers::the_driver_swap as swap;
        use smoozy_server_setup::blender::Version;
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let server = Identity::issue(IdentityKind::LocalServer).expect("a server").as_text();
        // **The install published 5.2.2 beside a 4.5 already there**, and the
        // driver `Servers` H11b holds for this connection reads 550 on a machine of its
        // own kernel.
        let found = vec![Version::parse("4.5.3").expect("4.5"), Version::parse("5.2.2").expect("5.2")];
        let the_driver = smoozy_servers::hardware::TheDriversOwnAnswer {
            version: smoozy_servers::hardware::DriverVersion::parse("550.163.01").expect("550"),
            cuda_version: None,
            where_k1_placed_the_machine: smoozy_servers::hardware::WhereK1PlacesIt::ItsOwnKernel,
        };
        let owed = swap::the_comparison_on(Some(&the_driver), false, &found);
        assert_eq!(owed, Some(found.clone()), "the install left no swap owed");
        swap::the_comparison_concluded(&server, owed);
        let nothing_live = smoozy_queue_history::acting::adoption::TheMachinesWorkingState {
            jobs: Vec::new(),
            the_admission_point_stands_at: None,
            notes: Vec::new(),
            the_history_copys_count: 0,
            the_history_copys_entries: Vec::new(),
        };
        let starts_with = swap::the_owed_swap_starts_with(&server, false, Some(&nothing_live))
            .expect("the owed swap did not start at the empty-queue reading");
        assert_eq!(starts_with, found);

        let scratch = Scratch::new("r1b-swap-run");
        let left = the_restart_is_driven_in_full(
            &scratch,
            sftp_server,
            AfterTheRestart::TheMachineAnswers,
            a_machine_whose_driver_answers_at_550(),
            TheRestartCommand::AnswersAndThenDrops,
            None,
            AndBeside {
                found: vec!["4.5.3", "5.2.2"],
                the_server: Some(server.clone()),
                as_the_swap_run: true,
                ..a_swap_that_reaches_the_bar(Vec::new(), None)
            },
        );
        let swap_at = left.asked.iter().position(|said| *said == the_swap());
        let restart = left.asked.iter().position(|said| said.contains("systemctl reboot"));
        assert!(swap_at.is_some() && swap_at < restart, "no swap before the restart: {:?}", left.asked);
        assert_eq!(left.kinds.first().cloned().flatten().as_deref(), Some(asked::THE_READY_ACTION));
        assert_eq!(it_reached_done_and_missed(&left.came_to), Some(false), "{:?}", left.came_to);
        assert!(matches!(
            left.came_to,
            WhereTheSetUpGot::TheRunEnded {
                ending: TheRunsEnding::ItReachedDone { a_blender_was_published: false, .. },
                ..
            }
        ));
        swap::the_ending_is_kept(
            &server,
            &left.came_to,
            &WhatWasAskedFor {
                the_blenders_on_the_server: found,
                ..WhatWasAskedFor::default()
            },
        );
        assert!(!swap::a_swap_is_owed_on(&server), "a swap made is still owed");
    }

    /// **A swap run nobody pressed for whose swap misses the bar ends *done*
    /// with the miss**, which is what the row keeps
    /// (`smoozy_desktop::setting_up::the_miss_this_run_leaves`) — the runner's
    /// ruling of 2026-10-11: a missed swap is not retried on its own.
    #[test]
    fn a_miss_from_the_swap_run_is_what_its_ending_leaves() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("r1b-swap-run-misses");
        let mut script = a_machine_whose_driver_answers_at_550();
        script.push((
            the_swap(),
            CommandResult {
                output: Vec::new(),
                complaint: b"E: Unable to locate package nvidia-fabricmanager-580\n".to_vec(),
                status: Some(100),
            },
        ));
        let left = the_restart_is_driven_in_full(
            &scratch,
            sftp_server,
            AfterTheRestart::TheMachineAnswers,
            script,
            TheRestartCommand::AnswersAndThenDrops,
            None,
            AndBeside {
                found: vec!["5.2.2"],
                as_the_swap_run: true,
                ..AndBeside::default()
            },
        );
        assert!(left.asked.contains(&the_swap()), "{:?}", left.asked);
        assert_eq!(it_reached_done_and_missed(&left.came_to), Some(true), "{:?}", left.came_to);
        assert_eq!(smoozy_desktop::setting_up::the_miss_this_run_leaves(&left.came_to), Some(true));
        assert_eq!(the_swap_waits_after(&left.came_to), Some(false));
    }

    /// **`Server Setup` R1c with `Server Setup` R1b's live set: the swap is made with only
    /// jobs held for the driver standing** — they are the jobs it is for, so
    /// neither reading of the live set counts them — **and the run's ending
    /// releases them with no mark**, the driver having come up to the bar.
    /// Held-job piece A, 2026-10-12.
    #[test]
    fn the_swap_is_made_with_only_driver_holds_standing_and_its_ending_releases_them() {
        use smoozy_desktop::servers::the_driver_swap as swap;
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let server = Identity::issue(IdentityKind::LocalServer).expect("a server").as_text();
        swap::a_run_whose_item_5_may_compare_begins(&server);
        let scratch = Scratch::new("r1c-only-driver-holds");
        let left = the_restart_is_driven_in_full(
            &scratch,
            sftp_server,
            AfterTheRestart::TheMachineAnswers,
            a_machine_whose_driver_answers_at_550(),
            TheRestartCommand::AnswersAndThenDrops,
            None,
            AndBeside {
                found: vec!["5.2.2"],
                held_for_the_driver: 2,
                the_server: Some(server.clone()),
                as_the_swap_run: true,
                ..a_swap_that_reaches_the_bar(Vec::new(), None)
            },
        );
        assert_eq!(left.queue_readings, 2, "the live set is read before the swap and at the last guard");
        let swap_at = left.asked.iter().position(|said| *said == the_swap());
        let restart = left.asked.iter().position(|said| said.contains("systemctl reboot"));
        assert!(swap_at.is_some() && swap_at < restart, "no swap over driver holds: {:?}", left.asked);
        assert_eq!(it_reached_done_and_missed(&left.came_to), Some(false), "{:?}", left.came_to);
        assert_eq!(the_swap_waits_after(&left.came_to), Some(false));
        let item_5 = swap::what_item_5_came_to(&server);
        assert!(item_5.it_compared_and_acted, "{item_5:?}");
        assert_eq!(item_5.the_last_check_found_it_below, Some(false));
        assert_eq!(item_5.the_drivers_version.as_deref(), Some("580.178.04"));
        assert_eq!(
            swap::what_the_ending_releases(&left.came_to, &item_5),
            swap::WhatTheEndingReleases::EveryDriverHold { the_mark: None }
        );
    }

    /// **`Server Setup` R1c: a run that backed off at the last guard releases
    /// nothing, and a swap that missed the bar releases every hold with `Queue
    /// & History` N23's mark, naming the driver the last check read.**
    /// Held-job piece A, 2026-10-12.
    #[test]
    fn a_backed_off_swap_releases_nothing_and_a_missed_one_releases_with_the_mark() {
        use smoozy_desktop::servers::the_driver_swap as swap;
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();

        let backs_off = Identity::issue(IdentityKind::LocalServer).expect("a server").as_text();
        swap::a_run_whose_item_5_may_compare_begins(&backs_off);
        let scratch = Scratch::new("r1c-backs-off");
        let left = the_restart_is_driven_in_full(
            &scratch,
            sftp_server.clone(),
            AfterTheRestart::TheMachineAnswers,
            a_machine_whose_driver_answers_at_550(),
            TheRestartCommand::AnswersAndThenDrops,
            None,
            AndBeside {
                waiting_at_each_queue_reading: vec![0, 1],
                held_for_the_driver: 1,
                the_server: Some(backs_off.clone()),
                ..a_swap_that_reaches_the_bar(vec!["5.2.2"], None)
            },
        );
        assert_eq!(the_swap_waits_after(&left.came_to), Some(true), "{:?}", left.came_to);
        let item_5 = swap::what_item_5_came_to(&backs_off);
        assert!(!item_5.it_compared_and_acted, "a run that backed off acted: {item_5:?}");
        assert_eq!(
            swap::what_the_ending_releases(&left.came_to, &item_5),
            swap::WhatTheEndingReleases::Nothing
        );

        let misses = Identity::issue(IdentityKind::LocalServer).expect("a server").as_text();
        swap::a_run_whose_item_5_may_compare_begins(&misses);
        let scratch = Scratch::new("r1c-misses");
        let mut script = a_machine_whose_driver_answers_at_550();
        script.push((
            the_swap(),
            CommandResult {
                output: Vec::new(),
                complaint: b"E: Unable to locate package nvidia-fabricmanager-580\n".to_vec(),
                status: Some(100),
            },
        ));
        let left = the_restart_is_driven_in_full(
            &scratch,
            sftp_server,
            AfterTheRestart::TheMachineAnswers,
            script,
            TheRestartCommand::AnswersAndThenDrops,
            None,
            AndBeside {
                found: vec!["5.2.2"],
                held_for_the_driver: 1,
                the_server: Some(misses.clone()),
                as_the_swap_run: true,
                ..AndBeside::default()
            },
        );
        assert_eq!(it_reached_done_and_missed(&left.came_to), Some(true), "{:?}", left.came_to);
        let item_5 = swap::what_item_5_came_to(&misses);
        assert!(item_5.it_compared_and_acted);
        assert_eq!(
            swap::what_the_ending_releases(&left.came_to, &item_5),
            swap::WhatTheEndingReleases::EveryDriverHold {
                the_mark: Some(Some("550.163.01".to_string()))
            }
        );
    }

    /// **`Server Setup` T5 and S4 item 2: the machine answers and the setup is
    /// picked up from the *restart issued* boundary.**
    #[test]
    fn the_machine_answering_picks_the_setup_up_by_a_recovery_run() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("restart-back");
        let left = the_restart_is_driven(&scratch, sftp_server, AfterTheRestart::TheMachineAnswers);

        // **`Server Setup` S4 item 2: a recovery run took the setup over**, and
        // it named the run it took over.
        let took_over = left.took_over.expect("no recovery run took the setup over");
        assert_eq!(
            Some(&took_over),
            left.runs.first(),
            "the recovery took over some run other than the one the restart ended"
        );
        // **`Server Setup` Y6b: a recovery carries no kind of its own**, and
        // the machine's own dispatch refuses one beside the taken-over run.
        assert_eq!(
            left.kinds,
            vec![Some(asked::THE_READY_ACTION.to_string()), None],
            "the recovery carried a kind, which the machine refuses"
        );
        // **`Server Setup` Y4's *card answering*: the check passed on this
        // boot**, and it is written after the recovery opened rather than
        // before.
        let written: Vec<&str> = left
            .written_from_the_desktop
            .iter()
            .map(|(boundary, _)| boundary.as_str())
            .collect();
        assert_eq!(
            written,
            vec![
                asked::THE_BOUNDARY_DRIVER_PLACED,
                asked::THE_BOUNDARY_RESTART_ISSUED,
                asked::THE_BOUNDARY_CARD_ANSWERING,
                asked::THE_BOUNDARY_DONE,
            ],
            "the setup did not carry on from the restart to the run's end"
        );
        assert!(
            matches!(
                left.came_to,
                WhereTheSetUpGot::TheRunEnded {
                    ending: TheRunsEnding::ItReachedDone { .. },
                    ..
                }
            ),
            "the recovered run did not reach done: {:?}",
            left.came_to
        );
        // **`Servers` M6 item 1: the machine answering is what cleared it.**
        assert_eq!(
            left.the_mark,
            smoozy_servers::restart::WhereTheMarkStands::ItWasClearedBy(
                smoozy_servers::restart::HowTheWaitEnds::TheMachineAnswers
            )
        );
    }

    /// **`Server Setup` T6 and S2: where the policy gives up, Smoozy says only
    /// what it knows.**
    #[test]
    fn a_machine_that_never_comes_back_is_reported_with_no_claim_about_why() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("restart-gone");
        let left = the_restart_is_driven(&scratch, sftp_server, AfterTheRestart::ThePolicyGivesUp);

        // **The run ends at `Server Setup` Y4's *restart issued***, which is
        // what it reached, and never at the boundary it began at.
        match &left.came_to {
            WhereTheSetUpGot::TheRunEnded {
                at,
                ending: TheRunsEnding::TheMachineDidNotComeBack { said },
            } => {
                assert_eq!(*at, smoozy_server_setup::run::Boundary::RestartIssued);
                assert!(
                    said.contains("no claim to know why"),
                    "Smoozy claimed to know why the machine did not come back: {said}"
                );
            }
            other => panic!("the run did not end at the policy giving up: {other:?}"),
        }
        // **The screen's own sentence claims nothing either** — a machine still
        // starting cannot be told from one that will never answer.
        let said = smoozy_desktop::surface::set_up::what_it_came_to(&left.came_to).said;
        assert!(
            said.contains("Could not reconnect after the restart"),
            "the screen did not say what Smoozy knows: {said}"
        );
        assert!(
            !said.to_lowercase().contains("because"),
            "the screen claimed a reason: {said}"
        );
        // **`Servers` M6 item 3: giving up clears the mark.**
        assert_eq!(
            left.the_mark,
            smoozy_servers::restart::WhereTheMarkStands::ItWasClearedBy(
                smoozy_servers::restart::HowTheWaitEnds::ThePolicyGivesUp
            )
        );
        // **Nothing was written past *restart issued***: no card answering, and
        // no done.
        let written: Vec<&str> = left
            .written_from_the_desktop
            .iter()
            .map(|(boundary, _)| boundary.as_str())
            .collect();
        assert_eq!(
            written,
            vec![asked::THE_BOUNDARY_DRIVER_PLACED, asked::THE_BOUNDARY_RESTART_ISSUED]
        );
        assert!(left.took_over.is_none(), "a recovery ran on a machine that never answered");
    }

    /// **`Server Setup` T6: the user stopping the wait clears nothing.**
    ///
    /// *"The mark stands, the machine-level state stands, `Servers`' reconnect
    /// policy for that marked period goes on running, and the setup is picked
    /// up on return; what ends is the user's waiting and nothing else."*
    #[test]
    fn a_user_who_stops_waiting_ends_the_waiting_and_clears_nothing() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("restart-stopped");
        let left =
            the_restart_is_driven(&scratch, sftp_server, AfterTheRestart::TheUserStopsWaiting);

        assert!(
            matches!(
                left.came_to,
                WhereTheSetUpGot::TheRunEnded {
                    ending: TheRunsEnding::TheMachineIsRestarting,
                    ..
                }
            ),
            "stopping the wait ended the run some other way: {:?}",
            left.came_to
        );
        // **The mark stands**, which is the whole of the clause: neither of
        // `Servers` M1's two things happened.
        assert_eq!(
            left.the_mark,
            smoozy_servers::restart::WhereTheMarkStands::ItStands,
            "the user stopping the wait cleared the mark"
        );
        assert!(left.took_over.is_none(), "a stopped wait started a recovery run");
        // **`Server Setup` Y8a, and it is respected rather than tested
        // around.** *"Where the machine is down, which is the restart wait and
        // nothing else, the durable write cannot be made and no request leaves
        // at all — so the stop reaches the desktop's waiting alone."* **No stop
        // request went out**, so there is no pending-command record to resolve
        // (`Server protocol and transfers` C3 and C20) and the earliest end
        // available to the user after a restart is the recovery run's next
        // boundary.
        assert!(
            left.the_stop_was_written_for.is_none(),
            "a stop request left the desk while the machine was down"
        );
        // **What the user is told says the setup is picked up on return**,
        // because it is.
        let said = smoozy_desktop::surface::set_up::what_it_came_to(&left.came_to).said;
        assert!(
            said.contains("Setup resumes when it reconnects"),
            "the screen did not say the setup carries on: {said}"
        );
    }

    /// **Step 10c's seventh passes on Hyperstack and Sesterce, played whole at
    /// the desk: the reboot takes the connection straight after the send, and
    /// the recovery picks the setup up.** Added 2026-09-27, fix piece 6.
    ///
    /// **What happened there.** The driver went in, `systemctl reboot` went
    /// out, and the machine went down with the command still in hand. The
    /// desktop wrote *restart issued* only after the command came back, so it
    /// was never written; the run ended at *driver placed*, `Server Setup` T5's
    /// recovery never ran, and the setup reached *done* only after *Abandon
    /// setup* and a second *Set up this server*.
    ///
    /// **What is proved.** *Restart issued* is durable before the command
    /// reaches the machine; the command never comes back; the desktop reads
    /// that as the restart going rather than failing; it waits; the machine
    /// answers; a recovery takes the run over and carries it on at the check;
    /// and the run reaches *done*. **Proved to bite** by putting the old order
    /// back in a copy outside the repository — the account of fix pieces 6, 9
    /// and 10 says what it did there.
    #[test]
    fn a_reboot_that_takes_the_connection_with_it_is_picked_up_by_the_recovery() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("restart-hyperstack");
        let left = the_restart_is_driven_with(
            &scratch,
            sftp_server,
            AfterTheRestart::TheMachineAnswers,
            a_machine_with_a_card_and_no_driver(),
            TheRestartCommand::TakesTheConnection,
        );
        assert_eq!(
            left.written_when_the_restart_arrived,
            Some(vec![
                asked::THE_BOUNDARY_DRIVER_PLACED.to_string(),
                asked::THE_BOUNDARY_RESTART_ISSUED.to_string(),
            ]),
            "*restart issued* was not durable when the reboot reached the machine"
        );
        assert!(
            matches!(
                left.came_to,
                WhereTheSetUpGot::TheRunEnded {
                    ending: TheRunsEnding::ItReachedDone { .. },
                    ..
                }
            ),
            "the recovery did not pick the setup up and carry it to done: {:?}",
            left.came_to
        );
        let took_over = left.took_over.expect("no recovery run took the setup over");
        assert_eq!(Some(&took_over), left.runs.first());
        let written: Vec<&str> = left
            .written_from_the_desktop
            .iter()
            .map(|(boundary, _)| boundary.as_str())
            .collect();
        assert_eq!(
            written,
            vec![
                asked::THE_BOUNDARY_DRIVER_PLACED,
                asked::THE_BOUNDARY_RESTART_ISSUED,
                asked::THE_BOUNDARY_CARD_ANSWERING,
                asked::THE_BOUNDARY_DONE,
            ],
            "the setup did not carry on from the restart to the run's end"
        );
        assert!(left.failures_recorded.is_empty(), "a failure was recorded: {:?}", left.failures_recorded);
        // **Fix piece 9: the screen was told the run a step at a time, the
        // restart and the check after it among them**, in `Server Setup` Y4's
        // order.
        use smoozy_server_setup::run::Boundary;
        let told: Vec<Boundary> = left.the_screen_was_told.iter().flatten().copied().collect();
        assert_eq!(
            told,
            vec![
                Boundary::DriverPlaced,
                Boundary::RestartIssued,
                Boundary::CardAnswering,
                Boundary::Done,
            ],
            "the screen was not told the run as its steps: {:?}",
            left.the_screen_was_told
        );
        assert_eq!(
            left.the_mark,
            smoozy_servers::restart::WhereTheMarkStands::ItWasClearedBy(
                smoozy_servers::restart::HowTheWaitEnds::TheMachineAnswers
            )
        );
    }

    /// **A restart command the server refuses while it stays connected, and
    /// what *Set up again* does over it** — the runner's ruling 1 of
    /// 2026-09-27, built the same day.
    ///
    /// **What is proved.** The refusal ends the run as the restart step
    /// failed, recorded on the machine as any failed step is, naming *restart
    /// issued*; the box says the restart was refused, with the machine's own
    /// words behind it; the period's mark is cleared by the machine answering;
    /// the Set up tab's own reading draws the restart as the step that did not
    /// finish, and *restart issued* as not reached. **Then *Set up again*
    /// takes the setup over and runs the restart step afresh** — the check,
    /// then the restart, refused again here — **and never goes to the card
    /// check as though the machine had gone down.**
    #[test]
    fn a_refused_restart_ends_as_the_restart_failed_and_its_mark_is_taken_back() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        const THE_MACHINES_WORDS: &str =
            "Failed to reboot: Access denied (words written for this test)\n";
        let refusing = || {
            let mut script = a_machine_with_a_card_and_no_driver();
            script.push((
                "sudo -n systemctl reboot".to_string(),
                CommandResult {
                    output: Vec::new(),
                    complaint: THE_MACHINES_WORDS.as_bytes().to_vec(),
                    status: Some(1),
                },
            ));
            script
        };
        let scratch = Scratch::new("restart-refused");
        let left = the_restart_is_driven_with(
            &scratch,
            sftp_server.clone(),
            AfterTheRestart::TheMachineAnswers,
            refusing(),
            TheRestartCommand::IsRefused,
        );
        let said = match &left.came_to {
            WhereTheSetUpGot::TheRunEnded {
                ending: TheRunsEnding::TheRestartWasRefused { said },
                ..
            } => said.clone(),
            other => panic!("the refusal did not end the run as a refused restart: {other:?}"),
        };
        assert_eq!(said, THE_MACHINES_WORDS);
        let run = left.runs.first().cloned().expect("a run was opened");
        assert_eq!(
            left.failures_recorded,
            vec![(
                run.clone(),
                asked::THE_BOUNDARY_RESTART_ISSUED.to_string(),
                THE_MACHINES_WORDS.to_string(),
            )],
            "the refusal was not recorded as the restart step failed"
        );
        let box_said = smoozy_desktop::surface::set_up::what_it_came_to(&left.came_to);
        assert!(box_said.said.contains("refused the restart"), "{}", box_said.said);
        assert_eq!(box_said.the_machines_own_words.as_deref(), Some(THE_MACHINES_WORDS));
        assert_eq!(
            left.the_mark,
            smoozy_servers::restart::WhereTheMarkStands::ItWasClearedBy(
                smoozy_servers::restart::HowTheWaitEnds::TheMachineAnswers
            ),
            "the period still reads as restarting on a machine that stayed up"
        );
        assert!(left.took_over.is_none(), "a recovery ran on a machine that never went down");
        let read = left.the_setup_read_at_the_end.expect("the setup was read back");
        assert!(
            !read
                .record
                .the_boundaries_reached
                .iter()
                .any(|it| it.boundary == smoozy_server_setup::run::Boundary::RestartIssued),
            "*restart issued* was not taken back"
        );
        assert_eq!(
            read.the_step_in_flight,
            Some(smoozy_server_setup::run::Boundary::RestartIssued)
        );
        let tab = smoozy_desktop::surface::set_up::the_tab(
            smoozy_desktop::surface::set_up::WhatTheMachineSaid::ASetupRunAlone(Box::new(read)),
            "Hyperstack A6000",
        );
        let panel = tab.the_stopped_setup.expect("the stopped-setup panel");
        let restart = panel
            .steps
            .iter()
            .find(|step| step.the_failure.is_some())
            .expect("no step carries the failure");
        assert_eq!(restart.name.as_str().replace('\u{a0}', " "), "Restart the server");
        assert!(panel.abandon.is_some() && panel.set_up_again.is_some());

        // ── *Set up again* over it.
        let scratch = Scratch::new("restart-refused-again");
        let again = the_order_is_driven_as(
            &scratch,
            sftp_server,
            refusing(),
            an_ubuntu_systemd_machine(),
            WhatThePlacementReached::Verified,
            None,
            AsThisRunIsAsked {
                install_this_blender: None,
                a_setup_that_stopped: Some((
                    run,
                    vec![
                        asked::THE_BOUNDARY_STARTED.to_string(),
                        asked::THE_BOUNDARY_DRIVER_PLACED.to_string(),
                        asked::THE_BOUNDARY_RESTART_ISSUED.to_string(),
                        asked::THE_BOUNDARY_CARD_ANSWERING.to_string(),
                        asked::THE_BOUNDARY_DONE.to_string(),
                    ],
                    vec![
                        asked::THE_BOUNDARY_STARTED.to_string(),
                        asked::THE_BOUNDARY_DRIVER_PLACED.to_string(),
                        asked::THE_BOUNDARY_RESTART_ISSUED.to_string(),
                    ],
                )),
                the_stopped_setup_ended: Some((
                    asked::A_STEP_FAILED.to_string(),
                    asked::THE_BOUNDARY_RESTART_ISSUED.to_string(),
                    THE_MACHINES_WORDS.to_string(),
                )),
                a_connected_period: true,
                ..AsThisRunIsAsked::default()
            },
        );
        // **The restart step ran afresh**: the check, then the restart.
        let the_last_check = again
            .asked
            .iter()
            .rposition(|said| said == "nvidia-smi")
            .expect("the check was not taken again");
        let the_restart = again
            .asked
            .iter()
            .rposition(|said| said.contains("systemctl reboot"))
            .expect("the restart was not sent again");
        assert!(the_last_check < the_restart, "the restart went before its check");
        // **And the card check was never written as though the machine had
        // gone down.**
        assert!(
            !again
                .written_from_the_desktop
                .iter()
                .any(|(boundary, _)| boundary == asked::THE_BOUNDARY_CARD_ANSWERING),
            "the recovery read a machine that never went down as one that did"
        );
        assert!(
            matches!(
                again.came_to,
                WhereTheSetUpGot::TheRunEnded {
                    ending: TheRunsEnding::TheRestartWasRefused { .. },
                    ..
                }
            ),
            "{:?}",
            again.came_to
        );
    }

    /// **The runner's ruling 2 of 2026-09-27: a marker at *driver placed* means
    /// the driver step finished**, so *Set up again* runs the restart step
    /// afresh — its check first — **and a marker at *restart issued* carries on
    /// at the check.** Built the same day; step 10c's seventh passes on
    /// Hyperstack and Sesterce met the refusal this replaces.
    #[test]
    fn a_setup_run_again_between_the_drivers_two_boundaries_is_carried_on() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let composed = vec![
            asked::THE_BOUNDARY_STARTED.to_string(),
            asked::THE_BOUNDARY_DRIVER_PLACED.to_string(),
            asked::THE_BOUNDARY_RESTART_ISSUED.to_string(),
            asked::THE_BOUNDARY_CARD_ANSWERING.to_string(),
            asked::THE_BOUNDARY_DONE.to_string(),
        ];
        for (reached, wanted) in [
            (
                vec![
                    asked::THE_BOUNDARY_STARTED.to_string(),
                    asked::THE_BOUNDARY_DRIVER_PLACED.to_string(),
                ],
                vec![
                    (asked::THE_BOUNDARY_RESTART_ISSUED, asked::IT_HAD_NOTHING_TO_DO),
                    (asked::THE_BOUNDARY_CARD_ANSWERING, asked::IT_PERFORMED),
                    (asked::THE_BOUNDARY_DONE, asked::IT_PERFORMED),
                ],
            ),
            (
                vec![
                    asked::THE_BOUNDARY_STARTED.to_string(),
                    asked::THE_BOUNDARY_DRIVER_PLACED.to_string(),
                    asked::THE_BOUNDARY_RESTART_ISSUED.to_string(),
                ],
                vec![
                    (asked::THE_BOUNDARY_CARD_ANSWERING, asked::IT_PERFORMED),
                    (asked::THE_BOUNDARY_DONE, asked::IT_PERFORMED),
                ],
            ),
        ] {
            let stopped = Identity::issue(IdentityKind::Operation).expect("randomness");
            let scratch = Scratch::new("between-the-driver");
            let left = the_order_is_driven_as(
                &scratch,
                sftp_server.clone(),
                a_machine_whose_driver_answers(),
                an_ubuntu_systemd_machine(),
                WhatThePlacementReached::Verified,
                None,
                AsThisRunIsAsked {
                    install_this_blender: None,
                    a_setup_that_stopped: Some((stopped.as_text(), composed.clone(), reached)),
                    ..AsThisRunIsAsked::default()
                },
            );
            assert!(
                matches!(
                    left.came_to,
                    WhereTheSetUpGot::TheRunEnded {
                        ending: TheRunsEnding::ItReachedDone { .. },
                        ..
                    }
                ),
                "the setup was not carried on: {:?}",
                left.came_to
            );
            let written: Vec<(&str, &str)> = left
                .written_from_the_desktop
                .iter()
                .map(|(boundary, mark)| (boundary.as_str(), mark.as_str()))
                .collect();
            assert_eq!(written, wanted);
            assert!(
                !left.asked.iter().any(|said| said.contains("systemctl reboot")),
                "a restart went out on a machine whose driver answers"
            );
            assert!(
                left.asked.iter().any(|said| said == "nvidia-smi"),
                "the check was not taken"
            );
            assert_eq!(left.state_at_the_end, None, "`Server Setup` S6: *done* clears the state");
        }
    }

    // ═══════════════════════════════════════════════════════════════════ the tests

    /// **`Server Setup` S6's abandon, driven against the stand-in supervisor**,
    /// with a setup that stopped part-way standing or with none. What comes
    /// back is what the abandon came to and what the machine was left holding.
    fn the_abandon_is_driven(
        scratch: &Scratch,
        sftp_server: PathBuf,
        a_setup_that_stopped: Option<(String, Vec<String>, Vec<String>)>,
    ) -> (WhatTheAbandonCameTo, Arc<Mutex<TheSupervisorHere>>) {
        the_abandon_is_driven_while(scratch, sftp_server, a_setup_that_stopped, 0)
    }

    /// The same, against a machine on which the run that stopped goes on
    /// running for `refusals` more recoveries.
    fn the_abandon_is_driven_while(
        scratch: &Scratch,
        sftp_server: PathBuf,
        a_setup_that_stopped: Option<(String, Vec<String>, Vec<String>)>,
        refusals: usize,
    ) -> (WhatTheAbandonCameTo, Arc<Mutex<TheSupervisorHere>>) {
        let supervisor = match a_setup_that_stopped {
            Some((run, composed, reached)) => TheSupervisorHere {
                run: Some(run.clone()),
                composed,
                reached,
                state: Some(run.clone()),
                set_by: vec![run],
                the_run_that_stopped_runs_for: refusals,
                ..TheSupervisorHere::default()
            },
            None => TheSupervisorHere::default(),
        };
        the_abandon_is_driven_with(scratch, sftp_server, supervisor, &mut NobodyIsThereToAsk)
    }

    /// The same, against a supervisor set up by the caller, with the screen
    /// the caller hands in. Added 2026-10-25, the eighteenth notes, note 5.
    fn the_abandon_is_driven_with(
        scratch: &Scratch,
        sftp_server: PathBuf,
        supervisor: TheSupervisorHere,
        screen: &mut dyn AsksForThePassword,
    ) -> (WhatTheAbandonCameTo, Arc<Mutex<TheSupervisorHere>>) {
        let supervisor = Arc::new(Mutex::new(supervisor));
        let mut far_end = TheFarEnd {
            sftp_server,
            at: scratch.at.clone(),
            said: Arc::new(Mutex::new(Vec::new())),
            asked: Arc::new(Mutex::new(Vec::new())),
            supervisor: Arc::clone(&supervisor),
            children: Arc::new(Mutex::new(Vec::new())),
            goes_down: Arc::new(tokio::sync::Notify::new()),
            the_mark_when_the_restart_arrived: None,
            the_state_when_a_command_arrived: Arc::new(Mutex::new(Vec::new())),
            the_restart_takes_the_connection: false,
            written_when_the_restart_arrived: Arc::new(Mutex::new(None)),
            slow: None,
            drivers: None,
        };
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("a runtime for the abandon");
        let point = ThisServersSessions::<TheFarEnd>::for_one_server();
        let asking = point.clone();
        let no_additional: TheAdditionalConnection<TheFarEnd> =
            Arc::new(|| Box::pin(async { None }));
        let package_area = format!("{}/smoozy/package", scratch.text());
        let came_to = runtime.block_on(async {
            let lending = lend_until_it_ends(&point, &mut far_end, &no_additional);
            let driven = a_setup_is_abandoned(&asking, &package_area, screen);
            tokio::pin!(lending, driven);
            tokio::select! {
                biased;
                _ended = &mut lending => unreachable!("the far end never ends on its own"),
                came_to = &mut driven => came_to,
            }
        });
        (came_to, supervisor)
    }

    /// **`Server Setup` S6's second clear, driven from the desk: *Abandon
    /// setup*.** Added 2026-09-22, out of step 10c's third pass, where a driver
    /// step that failed the same way on every *Set up again* left the machine
    /// held for good and the control that gives the setup up sent nothing.
    ///
    /// **What is proved.** The abandon takes over the run the state names, marked
    /// as an abandon; it commands the final verification and nothing else; it
    /// writes no boundary; it reports what the check found; and the state is
    /// cleared by the abandon's own run. **And where no state stands, nothing is
    /// opened at all.**
    #[test]
    fn abandon_setup_takes_the_stopped_setup_over_runs_only_the_final_check_and_clears_the_state() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let stopped = Identity::issue(IdentityKind::Operation).expect("randomness");
        let scratch = Scratch::new("abandoned");
        let (came_to, supervisor) = the_abandon_is_driven(
            &scratch,
            sftp_server.clone(),
            Some((
                stopped.as_text(),
                vec![
                    asked::THE_BOUNDARY_STARTED.to_string(),
                    asked::THE_BOUNDARY_DRIVER_PLACED.to_string(),
                    asked::THE_BOUNDARY_DONE.to_string(),
                ],
                vec![asked::THE_BOUNDARY_STARTED.to_string()],
            )),
        );
        assert_eq!(
            came_to,
            WhatTheAbandonCameTo::Abandoned {
                what_the_final_check_found: Some("the driver did not answer".to_string()),
            }
        );
        let held = supervisor.lock().expect("the supervisor");
        assert_eq!(held.took_over.as_deref(), Some(stopped.as_text().as_str()));
        assert!(held.abandons, "the recovery was not marked as an abandon");
        assert_eq!(held.steps, vec![asked::THE_FINAL_VERIFICATION_STEP.to_string()]);
        assert!(held.written_from_the_desktop.is_empty(), "an abandon wrote a boundary");
        assert_eq!(held.state, None, "`Server Setup` S6: the abandon clears the state");
        assert_eq!(held.cleared_by.len(), 1);
        assert_ne!(held.cleared_by[0], stopped.as_text(), "the clear came from the stopped run");
        drop(held);

        let scratch = Scratch::new("nothing-to-abandon");
        let (came_to, supervisor) = the_abandon_is_driven(&scratch, sftp_server, None);
        assert_eq!(came_to, WhatTheAbandonCameTo::NothingStoodToAbandon);
        let held = supervisor.lock().expect("the supervisor");
        assert!(held.opened.is_empty(), "a run was opened with nothing to abandon");
        assert!(held.cleared_by.is_empty());
    }

    /// A failed run can be read without a machine-level hold. The card must
    /// offer abandonment only for the hold that the action can clear.
    #[test]
    fn abandon_setup_is_offered_only_when_the_failed_run_holds_the_machine() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        for holds_the_machine in [false, true] {
            let stopped = Identity::issue(IdentityKind::Operation).expect("randomness");
            let scratch = Scratch::new("abandon-offer");
            let supervisor = Arc::new(Mutex::new(TheSupervisorHere {
                run: Some(stopped.as_text()),
                state: holds_the_machine.then(|| stopped.as_text()),
                composed: vec![
                    asked::THE_BOUNDARY_STARTED.to_string(),
                    asked::THE_BOUNDARY_TELEGRAM.to_string(),
                    asked::THE_BOUNDARY_DONE.to_string(),
                ],
                reached: vec![asked::THE_BOUNDARY_STARTED.to_string()],
                ending: Some((asked::A_STEP_FAILED.to_string(), Some("Token could not be saved.".into()))),
                failures_recorded: vec![(stopped.as_text(), asked::THE_BOUNDARY_TELEGRAM.to_string(), "Token could not be saved.".into())],
                closed: true,
                ..TheSupervisorHere::default()
            }));
            let mut far_end = TheFarEnd {
                sftp_server: sftp_server.clone(),
                at: scratch.at.clone(),
                said: Arc::new(Mutex::new(Vec::new())),
                asked: Arc::new(Mutex::new(Vec::new())),
                supervisor,
                children: Arc::new(Mutex::new(Vec::new())),
                goes_down: Arc::new(tokio::sync::Notify::new()),
                the_mark_when_the_restart_arrived: None,
                the_state_when_a_command_arrived: Arc::new(Mutex::new(Vec::new())),
                the_restart_takes_the_connection: false,
                written_when_the_restart_arrived: Arc::new(Mutex::new(None)),
                slow: None,
                drivers: None,
            };
            let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().expect("runtime");
            let point = ThisServersSessions::<TheFarEnd>::for_one_server();
            let no_additional: TheAdditionalConnection<TheFarEnd> = Arc::new(|| Box::pin(async { None }));
            let package_area = format!("{}/smoozy/package", scratch.text());
            let read = runtime.block_on(async {
                let lending = lend_until_it_ends(&point, &mut far_end, &no_additional);
                let reading = smoozy_desktop::setting_up::the_card_as_read(&point, &package_area, None);
                tokio::pin!(lending, reading);
                tokio::select! {
                    biased;
                    _ended = &mut lending => unreachable!("the far end stays up"),
                    read = &mut reading => read.the_run.expect("the failed run reads back"),
                }
            });
            let tab = smoozy_desktop::surface::set_up::the_tab(
                smoozy_desktop::surface::set_up::WhatTheMachineSaid::ASetupRunAlone(Box::new(read)),
                "abandon-offer",
            );
            let panel = tab.the_stopped_setup.expect("the failure stays on the card");
            assert_eq!(panel.abandon.is_some(), holds_the_machine, "the offer must match the action's hold check");
            assert!(panel.set_up_again.is_some(), "retry stays available");
            assert!(panel.steps.iter().any(|step| step.the_failure.is_some()), "the failed step stays visible");
            let paragraph = panel.the_closing_paragraph.expect("the available action is explained");
            assert_eq!(
                paragraph.the_controls_name,
                holds_the_machine.then(|| smoozy_desktop::typography::Prose::new("Abandon setup"))
            );
        }
    }

    /// **The eighteenth notes, note 5 — *Abandon setup*'s final check keeps
    /// its words among the run's words, as every other step's do**
    /// (`Server Setup` Y7a item 2; `Design/Server Setup § A long step`: *"The
    /// window and* Server output *carry what every step of the run printed"*).
    /// The machine's program carries the check's two streams on the run's
    /// state reading, and the abandon keeps them once, under the final check,
    /// as the run's words for the card. **And an abandon with nothing to
    /// abandon leaves the last run's words where they were.**
    ///
    /// **Before the piece** the abandon read the run through a loop of its
    /// own and kept nothing (the sixteenth notes, note 1: *Not built: the
    /// abandon*), so *Server output* after an abandon held no words at all.
    #[test]
    fn eighteenth_the_abandons_final_check_keeps_its_words_among_the_runs() {
        use smoozy_server_setup::ready_action::Step;
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let stopped = Identity::issue(IdentityKind::Operation).expect("randomness");
        let scratch = Scratch::new("eighteenth-abandon-words");
        let server = "eighteenth-the-abandons-words";
        let printed = "NVIDIA-SMI has failed because it couldn't communicate with the NVIDIA driver.\n";
        let complained = "ldd: warning: you do not have execution permission for `blender'\n";
        let supervisor = TheSupervisorHere {
            run: Some(stopped.as_text()),
            composed: vec![
                asked::THE_BOUNDARY_STARTED.to_string(),
                asked::THE_BOUNDARY_DRIVER_PLACED.to_string(),
                asked::THE_BOUNDARY_DONE.to_string(),
            ],
            reached: vec![asked::THE_BOUNDARY_STARTED.to_string()],
            state: Some(stopped.as_text()),
            set_by: vec![stopped.as_text()],
            the_steps_words: vec![the_words(asked::THE_FINAL_VERIFICATION_STEP, printed, complained)],
            ..TheSupervisorHere::default()
        };
        let mut screen = TheWordsKeptHere { server: server.to_string() };
        let (came_to, _) = the_abandon_is_driven_with(&scratch, sftp_server.clone(), supervisor, &mut screen);
        assert_eq!(
            came_to,
            WhatTheAbandonCameTo::Abandoned {
                what_the_final_check_found: Some("the driver did not answer".to_string()),
            }
        );
        let steps = smoozy_desktop::setting_up::the_words_of_each_step_on(server).expect("the abandon's words are kept");
        assert_eq!(
            steps.iter().map(|one| one.step).collect::<Vec<_>>(),
            vec![Step::Done],
            "the final check's words are not kept, once, under it: {steps:#?}"
        );
        assert_eq!(steps[0].the_ordinary_output, printed.as_bytes());
        assert_eq!(steps[0].the_complaints, complained.as_bytes());
        // **Both of the check's streams reach the window, in the order they
        // were kept** (from 2026-10-25, the eighteenth notes, piece 2c).
        assert_eq!(
            smoozy_desktop::setting_up::the_newest_lines_on(server)
                .iter()
                .rev()
                .take(2)
                .rev()
                .cloned()
                .collect::<Vec<_>>(),
            vec![printed.trim_end().to_string(), complained.trim_end().to_string()],
            "the card's window does not hold the check's words"
        );

        // **Nothing to abandon: no run, and the words above stay.**
        let scratch = Scratch::new("eighteenth-abandon-nothing");
        let (came_to, _) =
            the_abandon_is_driven_with(&scratch, sftp_server, TheSupervisorHere::default(), &mut screen);
        assert_eq!(came_to, WhatTheAbandonCameTo::NothingStoodToAbandon);
        assert_eq!(
            smoozy_desktop::setting_up::the_words_of_each_step_on(server).map(|steps| steps.len()),
            Some(1),
            "an abandon that opened no run let the last run's words go"
        );
    }

    /// **`Server Setup` S5's wait, built: *Abandon setup* over a run the server
    /// has not finished with waits and then goes ahead**, rather than coming back
    /// as a refusal in the machine's words. Added 2026-09-22 —
    /// `Design/Server Setup § The two ways out`: *"either one waits if the server
    /// has not finished with the run that stopped"*. *Set up again* opens its
    /// recovery through the same opening, so the same wait serves it.
    #[test]
    fn abandon_setup_waits_while_the_run_that_stopped_is_still_running() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let stopped = Identity::issue(IdentityKind::Operation).expect("randomness");
        let scratch = Scratch::new("abandon-waits");
        let (came_to, supervisor) = the_abandon_is_driven_while(
            &scratch,
            sftp_server,
            Some((
                stopped.as_text(),
                vec![
                    asked::THE_BOUNDARY_STARTED.to_string(),
                    asked::THE_BOUNDARY_DRIVER_PLACED.to_string(),
                    asked::THE_BOUNDARY_DONE.to_string(),
                ],
                vec![asked::THE_BOUNDARY_STARTED.to_string()],
            )),
            1,
        );
        assert!(
            matches!(came_to, WhatTheAbandonCameTo::Abandoned { .. }),
            "the abandon did not wait and go ahead: {came_to:?}"
        );
        let held = supervisor.lock().expect("the supervisor");
        assert_eq!(held.refused_while_it_ran, 1, "the machine was never asked while the run ran");
        assert_eq!(held.took_over.as_deref(), Some(stopped.as_text().as_str()));
        assert_eq!(held.state, None);
    }

    /// **Step 10c, 2026-09-21: a setup run again over one that stopped
    /// part-way takes it over and resumes after its marker** — `Server Setup`
    /// S3's *"when the user runs the setup again"* and S4 items 2 and 3, driven.
    ///
    /// **What it proves.** The fresh run is refused while the state stands, as
    /// the machine's own program refuses it, with the standing run in its own
    /// field; the desktop opens a recovery naming that run; the steps at or
    /// before the inherited marker are not commanded again; the run reaches
    /// *done* and the state is cleared. **Before the change the press ended at
    /// *Smoozy's program on the server would not start the setup*, for good** —
    /// the second pass on Hyperstack.
    #[test]
    fn a_setup_run_again_over_one_that_stopped_takes_it_over_and_resumes_after_its_marker() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();

        // **The composed list a fresh run of this machine gets**, taken from a
        // fresh run rather than written out here a second time.
        let first = Scratch::new("stopped-first");
        let fresh = the_order_is_driven(
            &first,
            sftp_server.clone(),
            a_machine_that_answers(),
            an_ubuntu_systemd_machine(),
            WhatThePlacementReached::Verified,
            None,
        );
        assert!(fresh.composed.contains(&asked::THE_BOUNDARY_FETCHED.to_string()));
        let reached = vec![
            asked::THE_BOUNDARY_STARTED.to_string(),
            asked::THE_BOUNDARY_FETCHED.to_string(),
        ];
        let stopped = Identity::issue(IdentityKind::Operation).expect("randomness");

        let scratch = Scratch::new("stopped-again");
        let left = the_order_is_driven_as(
            &scratch,
            sftp_server,
            a_machine_that_answers(),
            an_ubuntu_systemd_machine(),
            WhatThePlacementReached::Verified,
            None,
            AsThisRunIsAsked {
                a_setup_that_stopped: Some((
                    stopped.as_text(),
                    fresh.composed.clone(),
                    reached,
                )),
                ..AsThisRunIsAsked::default()
            },
        );
        assert!(
            matches!(
                left.came_to,
                WhereTheSetUpGot::TheRunEnded {
                    ending: TheRunsEnding::ItReachedDone { .. },
                    ..
                }
            ),
            "the setup run again did not reach done: {:?}",
            left.came_to
        );
        assert!(
            !left.steps.iter().any(|step| step == asked::THE_FETCH_STEP),
            "`Server Setup` S4 item 3: a step at the marker was commanded again: {:?}",
            left.steps
        );
        assert_eq!(left.state_at_the_end, None, "`Server Setup` S6: *done* clears the state");
        assert_eq!(
            left.set_by.first().map(String::as_str),
            Some(stopped.as_text().as_str()),
            "the state the setup took over was not the one standing"
        );
    }

    /// **A recovery refused because the setup it took over stopped genuinely
    /// inside a step: it is ended on the machine and *Abandon setup* appears.**
    /// Added 2026-09-26 as step 10c's fifth pass on Hyperstack at the desk;
    /// **narrowed 2026-09-27 by the runner's ruling 2**, which says a marker at
    /// *driver placed* is not inside the driver step where *restart issued*
    /// sits between its two boundaries — that case is now carried on, below.
    /// **So the record here holds no *restart issued* at all**, which is the
    /// one list the refusal is kept for.
    ///
    /// **What happened there.** The restart left the run at *driver placed*;
    /// the press opened a recovery, the machine took the setup over, and the
    /// desktop refused to carry it on because it stopped inside the driver step.
    /// **The recovery was left open with no ending**, so the stopped-setup panel
    /// never drew, *Abandon setup* was on no screen, and destroying the rental
    /// was the only way out.
    ///
    /// **What is proved.** The refusal is still the refusal; the recovery it
    /// opened is ended on the machine at once, with the driver step named and
    /// Smoozy's words; no boundary is written and the state is not cleared; and
    /// the setup, read afterwards as the Set up tab reads it, is a run that
    /// ended — so the tab opens on the stopped-setup panel with *Abandon setup*
    /// and *Set up again* on it.
    #[test]
    fn a_refused_recovery_after_the_restart_ends_and_abandon_setup_appears() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let stopped = Identity::issue(IdentityKind::Operation).expect("randomness");
        let scratch = Scratch::new("refused-recovery");
        let left = the_order_is_driven_as(
            &scratch,
            sftp_server,
            a_machine_that_answers(),
            an_ubuntu_systemd_machine(),
            WhatThePlacementReached::Verified,
            None,
            AsThisRunIsAsked {
                // **A run whose list holds the driver's two boundaries with
                // nothing between them**, stopped at the first — genuinely
                // inside the step.
                a_setup_that_stopped: Some((
                    stopped.as_text(),
                    vec![
                        asked::THE_BOUNDARY_STARTED.to_string(),
                        asked::THE_BOUNDARY_DRIVER_PLACED.to_string(),
                        asked::THE_BOUNDARY_CARD_ANSWERING.to_string(),
                        asked::THE_BOUNDARY_DONE.to_string(),
                    ],
                    vec![
                        asked::THE_BOUNDARY_STARTED.to_string(),
                        asked::THE_BOUNDARY_DRIVER_PLACED.to_string(),
                    ],
                )),
                read_the_setup_at_the_end: true,
                ..AsThisRunIsAsked::default()
            },
        );
        assert!(
            matches!(
                left.came_to,
                WhereTheSetUpGot::TheRunEnded {
                    ending: TheRunsEnding::ItStoppedInsideAStep {
                        step: smoozy_server_setup::ready_action::Step::Driver
                    },
                    ..
                }
            ),
            "the recovery was not refused inside the driver step: {:?}",
            left.came_to
        );
        // **The recovery is the run the state now names**, and it is the one
        // that was ended.
        let recovery = left.state_at_the_end.clone().expect("the state stands");
        assert_ne!(recovery, stopped.as_text(), "no recovery took the setup over");
        assert_eq!(
            left.failures_recorded,
            vec![(
                recovery,
                asked::THE_BOUNDARY_DRIVER_PLACED.to_string(),
                smoozy_desktop::setting_up::THE_RECOVERY_WAS_NOT_CARRIED_ON.to_string(),
            )],
            "the refused recovery was not ended on the machine"
        );
        assert!(left.written_from_the_desktop.is_empty(), "a boundary was written");
        assert!(left.steps.is_empty(), "a step was commanded: {:?}", left.steps);
        assert!(left.cleared_by.is_empty(), "`Server Setup` S6: the state was cleared");

        // **The tab's own reading and its own composition.**
        let read = left
            .the_setup_read_at_the_end
            .expect("the setup was not read back");
        assert!(!read.still_open, "the recovery is still open with no ending");
        let tab = smoozy_desktop::surface::set_up::the_tab(
            smoozy_desktop::surface::set_up::WhatTheMachineSaid::ASetupRunAlone(Box::new(read)),
            "Hyperstack A6000",
        );
        let panel = tab
            .the_stopped_setup
            .expect("the tab did not open on the stopped-setup panel");
        assert!(panel.abandon.is_some(), "*Abandon setup* is not on the panel");
        assert!(panel.set_up_again.is_some(), "*Set up again* is not on the panel");
        assert!(panel.cancel.is_none(), "nothing is running to cancel");
    }

    /// **Step 10c's third pass on Vast, at the desk: *Set up this server* over a
    /// Blender install that failed at the unpack.** Added 2026-09-22.
    ///
    /// **What happened there.** The install fetched, failed at the unpack and
    /// left its state standing; *Set up this server* took it over, kept only its
    /// own *done*, ran the final verification, wrote *done*, and the screen
    /// said *"The server is ready to render."* with no Blender on the machine.
    ///
    /// **What is proved here, against a machine whose program keeps no version
    /// in its record — which is 0.1.2's.** The recovery carries the taken-over
    /// install on rather than dropping its steps; its unpack is refused for want
    /// of a version, as the machine's own worker refuses it; so the setup stays
    /// failed, *done* is never written, the state is never cleared, and the box
    /// names the unpack and never says the server is ready to render.
    #[test]
    fn a_failed_blender_install_taken_over_by_set_up_again_stays_failed() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let stopped = Identity::issue(IdentityKind::Operation).expect("randomness");
        let scratch = Scratch::new("failed-install-taken-over");
        let left = the_order_is_driven_as(
            &scratch,
            sftp_server,
            a_machine_that_answers(),
            an_ubuntu_systemd_machine(),
            WhatThePlacementReached::Verified,
            None,
            AsThisRunIsAsked {
                install_this_blender: None,
                a_setup_that_stopped: Some((
                    stopped.as_text(),
                    vec![
                        asked::THE_BOUNDARY_STARTED.to_string(),
                        asked::THE_BOUNDARY_FETCHED.to_string(),
                        asked::THE_BOUNDARY_TOOLING.to_string(),
                        asked::THE_BOUNDARY_BLENDER.to_string(),
                        asked::THE_BOUNDARY_DONE.to_string(),
                    ],
                    vec![
                        asked::THE_BOUNDARY_STARTED.to_string(),
                        asked::THE_BOUNDARY_FETCHED.to_string(),
                    ],
                )),
                ..AsThisRunIsAsked::default()
            },
        );
        assert!(
            left.steps.iter().any(|step| step == asked::THE_UNPACK_STEP),
            "the recovery dropped the install's own steps: {:?}",
            left.steps
        );
        assert!(
            !left.steps.iter().any(|step| step == asked::THE_FINAL_VERIFICATION_STEP),
            "the final verification ran with the install unfinished: {:?}",
            left.steps
        );
        assert!(
            !left
                .written_from_the_desktop
                .iter()
                .any(|(boundary, _)| boundary == asked::THE_BOUNDARY_DONE),
            "*done* was written over a failed install"
        );
        assert!(left.cleared_by.is_empty(), "the state was cleared: {:?}", left.cleared_by);
        // **The press asked for no Blender, so the desktop had no version to
        // send**, and the machine kept none: that is what stops the unpack.
        for (step, version) in left.steps.iter().zip(&left.versions) {
            if step == asked::THE_UNPACK_STEP {
                assert_eq!(version, &None);
            }
        }
        assert!(
            matches!(
                left.came_to,
                WhereTheSetUpGot::TheRunEnded {
                    ending: TheRunsEnding::TheUnpackFailed { .. },
                    ..
                }
            ),
            "{:?}",
            left.came_to
        );
        let said = smoozy_desktop::surface::set_up::what_it_came_to(&left.came_to);
        assert!(!said.said.contains("Ready to render"), "{}", said.said);
        assert!(said.said.contains("Could not unpack Blender"), "{}", said.said);
        assert!(!said.said.contains("install the unpack tool"), "{}", said.said);
        // **What the server said is the machine's recorded reason**, not the
        // name of its ending.
        let words = said.the_machines_own_words.unwrap_or_default();
        assert!(words.contains("commanded with no version"), "{words}");
    }

    /// **The same take-over against a machine whose program keeps the version
    /// in its record** — from 2026-09-22 — **carries the install through to a
    /// published Blender, and only then says the server is ready to render.**
    #[test]
    fn a_blender_install_taken_over_with_its_version_recorded_is_carried_to_the_end() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let stopped = Identity::issue(IdentityKind::Operation).expect("randomness");
        let scratch = Scratch::new("install-taken-over-to-the-end");
        let left = the_order_is_driven_as(
            &scratch,
            sftp_server,
            a_machine_that_answers(),
            an_ubuntu_systemd_machine(),
            WhatThePlacementReached::Verified,
            None,
            AsThisRunIsAsked {
                install_this_blender: None,
                a_setup_that_stopped: Some((
                    stopped.as_text(),
                    vec![
                        asked::THE_BOUNDARY_STARTED.to_string(),
                        asked::THE_BOUNDARY_FETCHED.to_string(),
                        asked::THE_BOUNDARY_TOOLING.to_string(),
                        asked::THE_BOUNDARY_BLENDER.to_string(),
                        asked::THE_BOUNDARY_DONE.to_string(),
                    ],
                    vec![
                        asked::THE_BOUNDARY_STARTED.to_string(),
                        asked::THE_BOUNDARY_FETCHED.to_string(),
                    ],
                )),
                the_version_the_stopped_setup_recorded: Some("5.2.2".to_string()),
                ..AsThisRunIsAsked::default()
            },
        );
        assert!(
            matches!(
                left.came_to,
                WhereTheSetUpGot::TheRunEnded {
                    ending: TheRunsEnding::ItReachedDone {
                        a_blender_was_published: true,
                        the_driver_missed_the_optix_bar: false,
                        the_swap_waits: false,
                    },
                    ..
                }
            ),
            "{:?}",
            left.came_to
        );
        assert!(!left.steps.iter().any(|step| step == asked::THE_FETCH_STEP));
        for step in [asked::THE_UNPACK_STEP, asked::THE_BUILD_IS_PUBLISHED_STEP] {
            assert!(left.steps.iter().any(|commanded| commanded == step), "{step}");
        }
        let said = smoozy_desktop::surface::set_up::what_it_came_to(&left.came_to);
        assert_eq!(said.said, "Ready to render.");
    }

    /// **A setup that reaches *done* with no Blender in it never says the
    /// server is ready to render**, because a server with no Blender cannot.
    /// Added 2026-09-22 — the third pass on Vast said it after its first setup,
    /// which installed none.
    #[test]
    fn a_setup_with_no_blender_in_it_says_the_server_is_set_up_and_not_ready_to_render() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("done-with-no-blender");
        let left = the_order_is_driven_as(
            &scratch,
            sftp_server,
            a_machine_that_answers(),
            an_ubuntu_systemd_machine(),
            WhatThePlacementReached::Verified,
            None,
            AsThisRunIsAsked {
                install_this_blender: None,
                ..AsThisRunIsAsked::default()
            },
        );
        assert!(
            matches!(
                left.came_to,
                WhereTheSetUpGot::TheRunEnded {
                    ending: TheRunsEnding::ItReachedDone {
                        a_blender_was_published: false,
                        the_driver_missed_the_optix_bar: false,
                        the_swap_waits: false,
                    },
                    ..
                }
            ),
            "{:?}",
            left.came_to
        );
        let said = smoozy_desktop::surface::set_up::what_it_came_to(&left.came_to);
        assert_eq!(said.said, "Setup complete.");
    }

    /// **Step 10c, 2026-09-21: a setup run again presents the installation
    /// identity its own earlier hand-over recorded, where no placement
    /// confirmed it** — so `Rented-server runtime` G1b's own-installation
    /// take-back can match a claim a placement cut off part-way left. **And one
    /// a placement did confirm is not presented again**: the machine carries
    /// it, and `State, identity and recovery` Y8 issues a new one.
    #[test]
    fn a_setup_run_again_presents_the_installation_its_own_unconfirmed_hand_over_recorded() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let recorded = Identity::issue(IdentityKind::Installation).expect("randomness");
        let with_the_root = |confirmed: Option<String>| ServerRow {
            recorded_machine_root: Some(RecordedMachineRoot {
                path: "/home/ubuntu/smoozy".to_string(),
                the_package_areas_own_location: None,
                read_under: recorded.as_text(),
            }),
            last_confirmed_installation_identity: confirmed,
            ..a_row()
        };

        let scratch = Scratch::new("unconfirmed");
        let left = the_order_is_driven_as(
            &scratch,
            sftp_server.clone(),
            a_machine_that_answers(),
            an_ubuntu_systemd_machine(),
            WhatThePlacementReached::Verified,
            None,
            AsThisRunIsAsked {
                the_row: Some(with_the_root(None)),
                ..AsThisRunIsAsked::default()
            },
        );
        assert_eq!(
            left.given.expect("the placement was handed its values").installation,
            recorded.as_text(),
            "the unconfirmed identity was not presented again"
        );

        let scratch = Scratch::new("confirmed");
        let left = the_order_is_driven_as(
            &scratch,
            sftp_server,
            a_machine_that_answers(),
            an_ubuntu_systemd_machine(),
            WhatThePlacementReached::Verified,
            None,
            AsThisRunIsAsked {
                the_row: Some(with_the_root(Some(recorded.as_text()))),
                ..AsThisRunIsAsked::default()
            },
        );
        assert_ne!(
            left.given.expect("the placement was handed its values").installation,
            recorded.as_text(),
            "a confirmed installation's identity was presented as a new one"
        );
    }

    /// **`Server Setup` R2's whole order, driven on a machine that answers.**
    #[test]
    fn the_order_runs_from_the_readings_to_the_runs_last_step() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("whole");
        let left = the_order_is_driven(
            &scratch,
            sftp_server,
            a_machine_that_answers(),
            an_ubuntu_systemd_machine(),
            WhatThePlacementReached::Verified,
            None,
        );

        // **It reached the run's end.**
        assert!(
            matches!(
                left.came_to,
                WhereTheSetUpGot::TheRunEnded {
                    ending: TheRunsEnding::ItReachedDone { .. },
                    ..
                }
            ),
            "the run did not reach done: {:?}",
            left.came_to
        );

        // **`Server Setup` R2's order: the kind and the family are read
        // first.** They were asked for before the machine had been sent a
        // single command, and the tuple's own command came after.
        assert_eq!(
            left.the_readings_were_asked_at,
            Some(0),
            "something reached the machine before `Server Setup` M1 and `Server Setup` M1a"
        );
        assert!(
            left.asked.iter().any(|said| said.contains("uname -m")),
            "the tuple was never read"
        );

        // **`Server Setup` R7b: the root is recorded, and it is recorded before
        // the installation identity is.**
        assert_eq!(
            left.writes,
            vec![
                format!("root {}/smoozy", scratch.text()),
                "installation".to_string(),
                // **`Servers` Q19a: the commit is told once, and only after the
                // installation is recorded against the row** — so the re-take
                // reads the row that names it. Step 10c's eighth-pass fix
                // piece 3.
                "the placement committed".to_string(),
            ],
            "the two records and the commit did not land in the hand-over's own order"
        );

        // **Step 10c piece 7's owed item: the row's event cursor is cleared in
        // the same write as the new installation identity.**
        assert!(
            left.row.last_confirmed_installation_identity.is_some(),
            "the installation was not recorded"
        );
        assert_eq!(
            left.row.event_cursor, None,
            "the cursor was left pointing into a machine that no longer exists"
        );

        // **`Job files and outputs` L1: the machine's own copy of the root is
        // written where its supervisor reads it.**
        let the_roots_record = scratch.at("smoozy/package/root");
        assert_eq!(
            std::fs::read_to_string(&the_roots_record).expect("the root is recorded on the machine"),
            format!("{}/smoozy", scratch.text())
        );

        // **`Job files and outputs` L6: the five areas are made.**
        for area in ["jobs", "job-records", "assets", "blender", "package"] {
            assert!(
                scratch.at(&format!("smoozy/{area}")).is_dir(),
                "{area} was not made under the root"
            );
        }

        // **`Server protocol and transfers` B8b: the release file is asked for
        // at the one fixed address, and the artifact at the exact source the
        // verified manifest named.**
        assert_eq!(
            left.hosting_asked
                .iter()
                .map(|(source, _)| source.as_str())
                .collect::<Vec<_>>(),
            vec![WHERE_THE_CURRENT_RELEASE_IS_READ_FROM, THE_ARTIFACTS_SOURCE]
        );
        // **The artifact's ceiling is the manifest's own byte length.**
        assert_eq!(left.hosting_asked[1].1, 38);

        // **The placement was handed the package area the hand-over made, and
        // no session was open on the connection when it was lent.**
        let given = left.given.expect("the placement was performed");
        assert_eq!(given.package_area, format!("{}/smoozy/package", scratch.text()));
        assert_eq!(
            left.sessions_open_at_the_lend,
            Some(false),
            "a session was open when the whole connection was lent"
        );

        // **`Server Setup` Y7: the worker's steps were commanded one at a time,
        // in `Server Setup` R1's order.** The readings and the Telegram
        // material are not commanded — step 10c piece 8 refused both by name on
        // the machine's side.
        assert_eq!(
            left.steps,
            vec![
                asked::THE_FETCH_STEP,
                asked::THE_UNPACK_STEP,
                asked::THE_LIBRARY_CHAIN_IS_READ_STEP,
                asked::THE_LIBRARY_CHAIN_IS_READ_AGAIN_STEP,
                asked::THE_BUILD_IS_PUBLISHED_STEP,
                asked::THE_FINAL_VERIFICATION_STEP,
            ]
        );

        // **`Server Setup` Y4's composed list went with the run**, and it is
        // the list `Server Setup` R1 composed for this machine.
        assert!(left.composed.contains(&asked::THE_BOUNDARY_FETCHED.to_string()));
        assert!(left.composed.contains(&asked::THE_BOUNDARY_DONE.to_string()));
        assert!(
            !left.composed.contains(&asked::THE_BOUNDARY_TELEGRAM.to_string()),
            "Telegram is off, so `Server Setup` R1 item 8 is not in the run"
        );
    }

    /// **`Server Setup` R2: the kind and the family are read first and nothing
    /// else if either refuses.**
    #[test]
    fn a_machine_the_table_does_not_place_is_refused_before_anything_is_touched() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("refused");
        let left = the_order_is_driven(
            &scratch,
            sftp_server,
            a_machine_that_answers(),
            a_machine_the_table_does_not_place(),
            WhatThePlacementReached::Verified,
            None,
        );
        assert_eq!(
            left.came_to,
            WhereTheSetUpGot::TheKindOrTheFamilyWouldNotRead {
                which: WhichReadingRefused::TheKind
            }
        );
        // **Nothing was touched**: no record, no root on the machine, no
        // release asked for, no placement.
        assert!(left.writes.is_empty(), "something was recorded");
        assert!(left.given.is_none(), "a placement was started");
        assert!(left.hosting_asked.is_empty(), "a release was fetched");
        assert!(
            !scratch.at("smoozy").exists(),
            "a root was made on a machine Smoozy cannot set up"
        );
        // **And nothing was asked of the machine at all**, which is
        // `Server Setup` R2's *nothing else if either refuses*.
        assert!(
            left.asked.is_empty(),
            "the machine was sent commands after the kind refused: {:?}",
            left.asked
        );
    }

    /// **The worldwide pass's fault 2: a refusal reached because the
    /// connection went under the first readings is told as the connection
    /// going, never as a machine Smoozy does not support.** Added 2026-10-26.
    ///
    /// The pass on Vast: an Ubuntu 22.04 container, at the moment a
    /// three-second reconnect took the connection, was told *Smoozy cannot set
    /// this machine up. It is not a Linux server Smoozy supports. Nothing was
    /// changed on it.* — and set up on the next press. **Here both readings
    /// went under the connection**, as `Server Setup`'s own readings say it
    /// (`the_machine_before_anything_is_touched` beside a reading that never
    /// reached the machine), so the kind reads as nothing the table places.
    #[test]
    fn worldwide_fault_2_a_connection_that_went_under_the_first_readings_is_told_as_one() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("worldwide-fault-2");
        let left = the_order_is_driven(
            &scratch,
            sftp_server,
            a_machine_that_answers(),
            a_machine_the_table_does_not_place().with_a_reading_that_never_reached_the_machine(),
            WhatThePlacementReached::Verified,
            None,
        );
        assert_eq!(left.came_to, WhereTheSetUpGot::TheConnectionWent);
        // **What the card says is the connection, and not the machine.**
        let said = smoozy_desktop::surface::set_up::what_it_came_to(&left.came_to).said;
        assert!(said.contains("Connection lost."), "{said}");
        assert!(!said.contains("Unsupported operating system"), "{said}");
        // **And still nothing was touched or asked** — the readings change
        // nothing, which is what makes a press after the reconnect safe.
        assert!(left.writes.is_empty(), "something was recorded");
        assert!(left.given.is_none(), "a placement was started");
        assert!(left.asked.is_empty(), "the machine was sent commands: {:?}", left.asked);
        // **A machine that answered and is not one Smoozy prepares is still
        // refused as before**, with the same readings and no connection gone.
        let refused = smoozy_desktop::setting_up::what_the_opening_readings_came_to(
            a_machine_the_table_does_not_place(),
        );
        assert_eq!(
            refused,
            Err(WhereTheSetUpGot::TheKindOrTheFamilyWouldNotRead {
                which: WhichReadingRefused::TheKind
            })
        );
        // **And a reading that never reached the machine does not stop
        // readings that did place it.**
        assert!(smoozy_desktop::setting_up::what_the_opening_readings_came_to(
            an_ubuntu_systemd_machine().with_a_reading_that_never_reached_the_machine(),
        )
        .is_ok());
    }

    /// **`Server Setup` Y7's other half: the five boundaries the desktop
    /// writes, and a run that genuinely reaches `Server Setup` Y4's *done*.**
    ///
    /// **On this machine the driver already answers**, so `Server Setup` R1
    /// item 5's check-first branch installs nothing and item 6's restart has
    /// nothing to do — which puts *driver placed*, *restart issued* and *card
    /// answering* in one run with `Server Setup` Y4a's *nothing to do* beside
    /// each, and *done* after them.
    ///
    /// **Nothing elevated went out**, which is the half of
    /// `Server Setup` R5 this order exists to keep: a driver install repeated
    /// on a machine whose check passes silently becomes an upgrade the running
    /// kernel does not match.
    #[test]
    fn the_desktop_writes_its_own_boundaries_and_the_run_reaches_done() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("boundaries");
        let left = the_order_is_driven(
            &scratch,
            sftp_server,
            a_machine_whose_driver_answers(),
            an_ubuntu_systemd_machine(),
            WhatThePlacementReached::Verified,
            None,
        );
        assert!(
            matches!(
                left.came_to,
                WhereTheSetUpGot::TheRunEnded {
                    at: smoozy_server_setup::run::Boundary::Done,
                    ending: TheRunsEnding::ItReachedDone { .. },
                }
            ),
            "the run did not reach done: {:?}",
            left.came_to
        );
        assert!(left.refused.is_empty(), "a boundary write was refused: {:?}", left.refused);

        // **`Server Setup` Y4's order, and every one of these is the desktop's
        // under `Server Setup` Y7.**
        assert_eq!(
            left.written_from_the_desktop,
            vec![
                (
                    asked::THE_BOUNDARY_DRIVER_PLACED.to_string(),
                    asked::IT_HAD_NOTHING_TO_DO.to_string()
                ),
                (
                    asked::THE_BOUNDARY_RESTART_ISSUED.to_string(),
                    asked::IT_HAD_NOTHING_TO_DO.to_string()
                ),
                (
                    asked::THE_BOUNDARY_CARD_ANSWERING.to_string(),
                    asked::IT_HAD_NOTHING_TO_DO.to_string()
                ),
                (
                    asked::THE_BOUNDARY_DONE.to_string(),
                    asked::IT_PERFORMED.to_string()
                ),
            ]
        );

        // **`Server Setup` R5: no driver install went out on a machine whose
        // check answers.**
        assert!(
            !left
                .asked
                .iter()
                .any(|said| said.contains("ubuntu-drivers") || said.contains("reboot")),
            "something elevated went out on a machine whose driver already answers: {:?}",
            left.asked
        );

        // **The nine boundaries this machine's run is composed of**, Telegram
        // being off.
        assert_eq!(left.composed.len(), 9);
        assert!(!left.composed.contains(&asked::THE_BOUNDARY_TELEGRAM.to_string()));
    }

    /// **`Server Setup` Y1: a machine-changing run sets the machine-level
    /// state, naming itself, before its first machine-changing step — and
    /// clears it only at `Server Setup` Y4's *done*.**
    ///
    /// **The run here fetches a Blender**, so `Server Setup` R1 item 3's
    /// tooling install is its first act that reaches the package system, and
    /// `Server Setup`'s own `elevated::THE_TOOLING_INSTALL` classes it as
    /// `Server Setup` Y1's first act. **That is `Server Setup` Y4's *tooling*
    /// row word for word**: *"the machine-level setup-in-progress state is set
    /// where this run is machine-changing and this is its first such step"*.
    ///
    /// **The order is watched rather than read afterwards.** The state is
    /// recorded as it stood at the instant each command reached the machine, so
    /// a set that went out after the install would fail here.
    #[test]
    fn the_state_is_set_before_the_first_machine_changing_step_and_cleared_at_done() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("y1-state");
        let left = the_order_is_driven(
            &scratch,
            sftp_server,
            a_machine_whose_driver_answers(),
            an_ubuntu_systemd_machine(),
            WhatThePlacementReached::Verified,
            None,
        );
        assert!(
            matches!(
                left.came_to,
                WhereTheSetUpGot::TheRunEnded {
                    at: smoozy_server_setup::run::Boundary::Done,
                    ending: TheRunsEnding::ItReachedDone { .. },
                }
            ),
            "the run did not reach done: {:?}",
            left.came_to
        );

        // **`Rented-server runtime` P1: set once, naming this run.**
        assert_eq!(left.set_by.len(), 1, "the state was set {:?}", left.set_by);
        let run = left.set_by[0].clone();

        // **Before the act, and nothing before that.** The tooling install is
        // the first command that arrived with the state standing, and every
        // command before it arrived with none.
        // **The install, not the probe** — both name the package, and only one
        // of them reaches the package system. The probe arrives before the
        // state and is meant to. **From 2026-09-22 the first command to reach
        // the package system is `Server Setup` S7d's refresh in front of the
        // install**, part of the same act, so it is what the state must already
        // stand for.
        let at_the_install = left
            .the_state_when_a_command_arrived
            .iter()
            .position(|(said, _)| said.contains("apt-get"))
            .expect("the tooling install went out");
        assert!(
            left.the_state_when_a_command_arrived[at_the_install].0.ends_with(" update"),
            "the lists were not refreshed in front of the install"
        );
        assert_eq!(
            left.the_state_when_a_command_arrived[at_the_install].1,
            Some(run.clone()),
            "the tooling install reached the machine before the state was set"
        );
        assert!(
            left.the_state_when_a_command_arrived[..at_the_install]
                .iter()
                .all(|(_, standing)| standing.is_none()),
            "the state was set before the first machine-changing step: {:?}",
            left.the_state_when_a_command_arrived
        );

        // **`Server Setup` S6: cleared at *done*, by the run that set it, and
        // the clear landed before the boundary claiming it.**
        assert_eq!(left.cleared_by, vec![run]);
        assert!(
            left.state_at_the_end.is_none(),
            "the machine is still holding its own work back"
        );
        assert_eq!(
            left.the_state_when_done_was_written,
            Some(None),
            "`Server Setup` Y4's *done* was written while the state still stood"
        );

        // **`Server Setup` R1 item 3's probe went out before the install**, and
        // it said the tool was absent — so the install was issued because the
        // machine was asked, not because nothing asked.
        let at_the_probe = left
            .asked
            .iter()
            .position(|said| said.contains("dpkg-query"))
            .expect("the probe went out");
        let at_the_install = left
            .asked
            .iter()
            .position(|said| said.contains("xz-utils") && said.contains("apt-get"))
            .expect("the tooling install went out");
        assert!(
            at_the_probe < at_the_install,
            "the install went out before the machine was asked: {:?}",
            left.asked
        );
        // **`Server Setup` Y4a's mark is the probe's answer** — *performed*,
        // because it was.
        assert_eq!(
            left.marks_on_steps
                .iter()
                .find(|(step, _)| step == asked::THE_UNPACK_STEP)
                .map(|(_, mark)| mark.clone()),
            Some(Some(asked::IT_PERFORMED.to_string()))
        );
    }

    /// **`Server Setup` R1 item 3: the tool is *"installed where absent"*, and
    /// the step *"probes rather than assumes"* — so a machine that already
    /// holds it is given no install at all.**
    ///
    /// **This is the other direction of the same probe**, and with it the run
    /// performs none of `Server Setup` Y1's three acts: *"a Blender install on
    /// a machine lacking the unpack tool performs an elevated package install
    /// and is machine-changing, and one on a machine that already has the tool
    /// performs an unpack and a rename and is not."* **So no machine-level
    /// state is set**, nothing holds reconciliation or an admission back, and
    /// `Server Setup` Y4a's mark for the step is *nothing to do*.
    #[test]
    fn a_run_on_a_machine_that_already_holds_the_unpack_tool_installs_nothing_for_it() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("r1-item-3-probe");
        let left = the_order_is_driven(
            &scratch,
            sftp_server,
            a_machine_that_already_holds_the_unpack_tool(),
            an_ubuntu_systemd_machine(),
            WhatThePlacementReached::Verified,
            None,
        );
        assert!(
            matches!(
                left.came_to,
                WhereTheSetUpGot::TheRunEnded {
                    at: smoozy_server_setup::run::Boundary::Done,
                    ending: TheRunsEnding::ItReachedDone { .. },
                }
            ),
            "the run did not reach done: {:?}",
            left.came_to
        );

        // **The machine was asked**, and asked by the package system's own
        // query rather than by anything guessing.
        assert!(
            left.asked.iter().any(|said| said.contains("dpkg-query")),
            "nothing probed the machine: {:?}",
            left.asked
        );
        // **And nothing was installed for the tool.**
        assert!(
            !left.asked.iter().any(|said| said.contains("apt-get")),
            "a package install went out on a machine that needed none: {:?}",
            left.asked
        );

        // **The run fetched a Blender and is still additive**, which is the
        // whole of what the probe buys.
        assert!(
            left.steps.iter().any(|step| step == asked::THE_FETCH_STEP),
            "the run fetched no Blender, so this proves nothing: {:?}",
            left.steps
        );
        assert!(left.set_by.is_empty(), "a state was set: {:?}", left.set_by);
        assert!(
            left.cleared_by.is_empty(),
            "a clear was sent for a run that set nothing: {:?}",
            left.cleared_by
        );
        assert_eq!(left.state_at_the_end, None);

        // **The archive is still unpacked** — the step is in the run either
        // way, and it is the install that was conditional.
        assert!(
            left.steps.iter().any(|step| step == asked::THE_UNPACK_STEP),
            "the archive was never unpacked: {:?}",
            left.steps
        );
        // **`Server Setup` Y4a's mark is the probe's answer**, and the set
        // stays closed at two.
        assert_eq!(
            left.marks_on_steps
                .iter()
                .find(|(step, _)| step == asked::THE_UNPACK_STEP)
                .map(|(_, mark)| mark.clone()),
            Some(Some(asked::IT_HAD_NOTHING_TO_DO.to_string()))
        );
    }

    /// **`Server Setup` Y1: an additive run sets nothing.**
    ///
    /// **This machine's driver answers and the run fetches no Blender**, so
    /// `Server Setup` R1 item 5 installs nothing and touches nothing, item 6
    /// has nothing to do, and no step of the run reaches the package system.
    /// **Nothing elevated leaves the desk at all**, which is what makes the
    /// class honest rather than asserted.
    ///
    /// **Y1 spells out why over-setting is not the safe direction**: the state
    /// holds reconciliation and every admission back, so a run that sets it for
    /// nothing holds the interrupted job back for nothing.
    #[test]
    fn an_additive_run_sets_no_machine_level_state() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("y1-additive");
        let left = the_order_is_driven_as(
            &scratch,
            sftp_server,
            a_machine_whose_driver_answers(),
            an_ubuntu_systemd_machine(),
            WhatThePlacementReached::Verified,
            None,
            AsThisRunIsAsked {
                elevation: TheElevationHere::Passwordless,
                telegram_is_on: false,
                install_this_blender: None,
                the_libraries_not_found: Vec::new(),
                ..AsThisRunIsAsked::default()
            },
        );
        assert!(
            matches!(
                left.came_to,
                WhereTheSetUpGot::TheRunEnded {
                    at: smoozy_server_setup::run::Boundary::Done,
                    ending: TheRunsEnding::ItReachedDone { .. },
                }
            ),
            "the run did not reach done: {:?}",
            left.came_to
        );
        assert!(left.set_by.is_empty(), "a state was set: {:?}", left.set_by);
        assert!(
            left.cleared_by.is_empty(),
            "a clear was sent for a run that set nothing: {:?}",
            left.cleared_by
        );
        assert_eq!(left.state_at_the_end, None);
        assert_eq!(left.the_state_when_done_was_written, Some(None));
        // **Not one of `Server Setup` Y1's three acts went out**, which is
        // what the class rests on. The hand-over's `chown` is elevated and is
        // none of them — `Server Setup` R7's placement act, before the run
        // exists to name — and `Server Setup`'s own `elevated.rs` classes it
        // so at the command.
        // **`Server Setup` R1 item 3's probe is not one of them and is not
        // counted as one.** It reads the package database and changes nothing,
        // and it is unelevated.
        assert!(
            !left.asked.iter().any(|said| said.contains("apt-get")
                || (said.contains("dpkg") && !said.contains("dpkg-query"))
                || said.contains("ubuntu-drivers")
                || said.contains("nvidia-driver")
                || said.contains("reboot")),
            "one of Y1's three acts went out on an additive run: {:?}",
            left.asked
        );
    }

    /// **`Server Setup` R1 item 4: the whole not-found list comes back, the
    /// install is selected from it, and `Server Setup` Y4a's mark is the
    /// desktop's because the act was.**
    ///
    /// **The list rides the run's own state reading** — `Server Setup` Y7's
    /// refresh — and `Server Setup` E7's rule is kept by construction: the list
    /// *selects* one command every byte of which is fixed at build time, and no
    /// name read off the machine reaches the string.
    ///
    /// **It drives the whole list a bare machine reports, and until 2026-09-22
    /// it drove `libX11.so.6` alone.** That one name was the only key the
    /// mapping had, so this test passed while every real machine's install
    /// stopped at this step — step 10c's fourth pass on Vast is where it was
    /// finally seen. The list is
    /// `smoozy_server_setup::ready_action::WHAT_A_BARE_CONTAINER_REPORTED`,
    /// which is that pass's own reading kept in one place.
    #[test]
    fn the_libraries_install_is_selected_from_the_list_the_machine_sent_back() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("libraries-install");
        let left = the_order_is_driven_as(
            &scratch,
            sftp_server,
            a_machine_whose_driver_answers(),
            an_ubuntu_systemd_machine(),
            WhatThePlacementReached::Verified,
            None,
            AsThisRunIsAsked {
                elevation: TheElevationHere::Passwordless,
                telegram_is_on: false,
                install_this_blender: Some("4.5.3".to_string()),
                the_libraries_not_found: smoozy_server_setup::ready_action::
                    WHAT_A_BARE_CONTAINER_REPORTED
                        .iter()
                        .map(|library| library.to_string())
                        .collect(),
                ..AsThisRunIsAsked::default()
            },
        );
        assert!(
            matches!(
                left.came_to,
                WhereTheSetUpGot::TheRunEnded {
                    at: smoozy_server_setup::run::Boundary::Done,
                    ending: TheRunsEnding::ItReachedDone { .. },
                }
            ),
            "the run did not reach done: {:?}",
            left.came_to
        );
        // **The one constant the mapping's one entry selects**, with all its
        // package names in the mapping's own order and nothing composed.
        let install = left
            .asked
            .iter()
            .find(|said| said.contains("libx11-6"))
            .unwrap_or_else(|| panic!("no libraries install went out: {:?}", left.asked));
        for named in ["libxkbcommon0", "libxext6", "libsm6", "libice6", "libgl1"] {
            assert!(install.contains(named), "a partial install: {install}");
        }
        for reported in smoozy_server_setup::ready_action::WHAT_A_BARE_CONTAINER_REPORTED {
            assert!(
                !install.contains(reported),
                "a name read off the machine reached the command: {install}"
            );
        }
        // **The state stood when it arrived.**
        assert_eq!(
            left.the_state_when_a_command_arrived
                .iter()
                .find(|(said, _)| said.contains("libx11-6"))
                .map(|(_, standing)| standing.clone()),
            Some(Some(left.set_by[0].clone())),
            "the libraries install reached the machine before the state was set"
        );
        // **`Server Setup` Y4a's mark rides the re-read**, which is the step
        // that feeds the *libraries* boundary.
        assert_eq!(
            left.marks_on_steps
                .iter()
                .find(|(step, _)| step == asked::THE_LIBRARY_CHAIN_IS_READ_AGAIN_STEP)
                .map(|(_, mark)| mark.clone()),
            Some(Some(asked::IT_PERFORMED.to_string()))
        );
        // **And the first read carries none**, because it feeds no boundary.
        assert_eq!(
            left.marks_on_steps
                .iter()
                .find(|(step, _)| step == asked::THE_LIBRARY_CHAIN_IS_READ_STEP)
                .map(|(_, mark)| mark.clone()),
            Some(None)
        );
    }

    /// **`Server Setup` Y7's second bullet: *"the desktop records the step's
    /// start against the run"* — before any command of an admin step reaches
    /// the machine, naming that step.** Added 2026-09-29, step 10c's transfer
    /// wave piece 12.
    ///
    /// **Why it matters**: an admin command's process is the connection's
    /// child, so a connection cut during one leaves the machine with nothing
    /// saying the step had begun, and `Server Setup` S1's reading then says
    /// *not started* over a package install or a driver install that was cut
    /// part-way. **Undone, no start ever reached the machine** and every
    /// install below arrived with no step recorded as under way.
    #[test]
    fn every_admin_steps_commands_arrive_with_that_steps_start_recorded() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("admin-step-start");
        let left = the_order_is_driven_as(
            &scratch,
            sftp_server,
            a_machine_whose_driver_answers(),
            an_ubuntu_systemd_machine(),
            WhatThePlacementReached::Verified,
            None,
            AsThisRunIsAsked {
                elevation: TheElevationHere::Passwordless,
                telegram_is_on: false,
                install_this_blender: Some("4.5.3".to_string()),
                the_libraries_not_found: smoozy_server_setup::ready_action::
                    WHAT_A_BARE_CONTAINER_REPORTED
                        .iter()
                        .map(|library| library.to_string())
                        .collect(),
                ..AsThisRunIsAsked::default()
            },
        );
        assert!(
            matches!(
                left.came_to,
                WhereTheSetUpGot::TheRunEnded {
                    at: smoozy_server_setup::run::Boundary::Done,
                    ending: TheRunsEnding::ItReachedDone { .. },
                }
            ),
            "the run did not reach done: {:?}",
            left.came_to
        );
        let started_when = |fragment: &str| {
            left.the_step_started_when_a_command_arrived
                .iter()
                .find(|(said, _)| said.contains(fragment) && said.contains("apt-get"))
                .map(|(_, started)| started.clone())
                .unwrap_or_else(|| panic!("no install naming {fragment} went out"))
        };
        assert_eq!(
            started_when("xz-utils"),
            Some(asked::THE_BOUNDARY_TOOLING.to_string()),
            "the tooling install reached the machine with no step recorded as started"
        );
        assert_eq!(
            started_when("libx11-6"),
            Some(asked::THE_BOUNDARY_LIBRARIES.to_string()),
            "the libraries install reached the machine with no step recorded as started"
        );
        // **Every command that reaches the package system is one of an admin
        // step's**, the lists' refresh in front of each install included, and
        // none arrives before its step's start.
        let unrecorded: Vec<&String> = left
            .the_step_started_when_a_command_arrived
            .iter()
            .filter(|(said, started)| said.contains("apt-get") && started.is_none())
            .map(|(said, _)| said)
            .collect();
        assert!(
            unrecorded.is_empty(),
            "these admin commands arrived with no step recorded as started: {unrecorded:?}"
        );
    }

    /// **`Server Setup` S2a's desk half: an elevated step that fails is written
    /// onto the machine, with the step named and the machine's own words
    /// whole.** Added 2026-09-21.
    ///
    /// **The unpack tool's install is refused by the package system**, the way
    /// a real one is when another install holds its lock. **Before this, the
    /// words stayed in this side's loop**: nothing was written, the run stayed
    /// open until `Server Setup` Y7's window ended it as *cut*, and a later
    /// reading said the connection dropped.
    #[test]
    fn an_elevated_step_that_fails_is_written_onto_the_machine_with_its_words() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("elevated-failure-recorded");
        let refusal = b"E: Could not get lock /var/lib/dpkg/lock-frontend. It is held by \
                        process 4242 (apt-get)\n";
        let mut script = a_machine_whose_driver_answers();
        script.push((
            smoozy_server_setup::elevated::THE_TOOLING_INSTALL
                .elevated
                .as_str()
                .to_string(),
            CommandResult {
                output: b"Reading package lists...\n".to_vec(),
                complaint: refusal.to_vec(),
                status: Some(100),
            },
        ));
        let left = the_order_is_driven_as(
            &scratch,
            sftp_server,
            script,
            an_ubuntu_systemd_machine(),
            WhatThePlacementReached::Verified,
            None,
            AsThisRunIsAsked::default(),
        );
        let WhereTheSetUpGot::TheRunEnded {
            ending: TheRunsEnding::AStepFailed { step, said },
            ..
        } = &left.came_to
        else {
            panic!("the run did not stop at the tooling step: {:?}", left.came_to);
        };
        assert_eq!(*step, smoozy_server_setup::ready_action::Step::Tooling);
        assert_eq!(
            left.failures_recorded.len(),
            1,
            "the failure was written {:?} times",
            left.failures_recorded
        );
        let (run, named, words) = &left.failures_recorded[0];
        assert_eq!(Some(run), left.set_by.first(), "the failure named another run");
        assert_eq!(named, asked::THE_BOUNDARY_TOOLING, "the failure named another step");
        assert_eq!(words, said, "the words written are not the words this side held");
        assert!(
            words.contains("Could not get lock"),
            "the machine's own words did not travel: {words}"
        );
        // **No stop was written for it**: a failure is its own ending, told
        // apart from a stop by its kind (`Server Setup` S2a).
        assert_eq!(left.the_stop_was_written_for, None);
    }

    /// **And a run that goes well writes no failure at all.**
    #[test]
    fn a_run_that_reaches_done_records_no_failure() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("no-failure-recorded");
        let left = the_order_is_driven_as(
            &scratch,
            sftp_server,
            a_machine_whose_driver_answers(),
            an_ubuntu_systemd_machine(),
            WhatThePlacementReached::Verified,
            None,
            AsThisRunIsAsked::default(),
        );
        assert!(
            matches!(
                left.came_to,
                WhereTheSetUpGot::TheRunEnded {
                    ending: TheRunsEnding::ItReachedDone { .. },
                    ..
                }
            ),
            "{:?}",
            left.came_to
        );
        assert!(left.failures_recorded.is_empty(), "{:?}", left.failures_recorded);
    }

    /// **`Server Setup` R1 item 4's own rule: there is never a partial
    /// install.**
    ///
    /// A library the mapping does not name has no package list behind it, so
    /// **no command is composed for it and none is issued** — the step fails
    /// with the machine's own name in the words, and the run stops there rather
    /// than going on to claim a check that did not pass.
    #[test]
    fn a_library_the_mapping_does_not_name_stops_the_step_and_installs_nothing() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("libraries-unnamed");
        let left = the_order_is_driven_as(
            &scratch,
            sftp_server,
            a_machine_whose_driver_answers(),
            an_ubuntu_systemd_machine(),
            WhatThePlacementReached::Verified,
            None,
            AsThisRunIsAsked {
                elevation: TheElevationHere::Passwordless,
                telegram_is_on: false,
                install_this_blender: Some("4.5.3".to_string()),
                the_libraries_not_found: vec!["libnothing.so.9".to_string()],
                ..AsThisRunIsAsked::default()
            },
        );
        let WhereTheSetUpGot::TheRunEnded {
            ending: TheRunsEnding::AStepFailed { said, .. },
            ..
        } = &left.came_to
        else {
            panic!("the run did not stop at the libraries step: {:?}", left.came_to);
        };
        assert!(
            said.contains("libnothing.so.9"),
            "the machine's own name is not in the words: {said}"
        );
        assert!(
            !left.asked.iter().any(|said| said.contains("libx11-6")),
            "something was installed for a library the mapping does not name: {:?}",
            left.asked
        );
        // **The state still stands**, because the tooling install set it and
        // `Server Setup` S6 clears it only at *done*. That is
        // `Rented-server runtime` P7 working rather than something left
        // undone, and `Server Setup` S4's recovery is the route on.
        assert_eq!(left.state_at_the_end, Some(left.set_by[0].clone()));
        assert!(left.cleared_by.is_empty());
    }

    /// **`Server Setup` Y8: a stopped run is given no boundary past the last it
    /// passed.**
    ///
    /// The stop is read between two steps **and again before every boundary
    /// write**, because one `Server Setup` R1 step can feed more than one
    /// boundary — the driver's feeds two.
    /// **`Server Setup` R1 item 8, and `Server Setup` Y4's *telegram* row made
    /// true rather than vacuous: the material is placed and the desired state
    /// is set.**
    ///
    /// **Until 2026-09-20 nothing composed what to hand the placement**, so a
    /// run with Telegram switched on stopped at this step rather than passing
    /// quietly — which was the right ending and not a working one.
    ///
    /// **`Telegram` §20's three things are all three checked**: the token file
    /// and the role configuration really land on the far end, through OpenSSH's
    /// own file-transfer server, and the durable enabled-or-disabled desired
    /// state goes on the ordinary command path.
    #[test]
    fn a_run_with_telegram_on_places_the_material_and_sets_the_roles_state() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("telegram-on");
        let left = the_order_is_driven_as(
            &scratch,
            sftp_server,
            a_machine_whose_driver_answers(),
            an_ubuntu_systemd_machine(),
            WhatThePlacementReached::Verified,
            None,
            AsThisRunIsAsked {
                telegram_is_on: true,
                ..AsThisRunIsAsked::default()
            },
        );

        // **The two files are really on the machine**, each at the path
        // `Security and licence` §10 fixes — inside the package area, outside
        // every job folder.
        let token_file = scratch.at("smoozy/package/telegram-token");
        let configuration = scratch.at("smoozy/package/telegram-role-configuration");
        assert!(token_file.exists(), "the token file was not placed");
        assert!(
            configuration.exists(),
            "the role configuration was not placed"
        );
        assert_eq!(
            std::fs::read(&token_file).expect("the token file reads"),
            A_MADE_UP_TOKEN,
            "the token file does not hold the token it was handed"
        );
        // **The secret is in the token file alone.** `Telegram` §20's second
        // item carries the conversation, the machine and the switches, and a
        // token in it would be the same secret in two places.
        let written = std::fs::read(&configuration).expect("the configuration reads");
        assert!(
            !String::from_utf8_lossy(&written).contains("made-up-for-a-test"),
            "the token reached the role configuration"
        );

        // **§20's third item is a command and never a file.**
        assert_eq!(
            left.the_roles_state,
            Some(true),
            "the role's durable desired state was not set"
        );
        assert!(
            !scratch.at("smoozy/package/telegram-role-state").exists(),
            "the desired state was written as a file"
        );

        // **`Server Setup` Y4's *telegram* boundary, with `Server Setup` Y4a's
        // mark**, and the run goes on to *done*.
        let written: Vec<&str> = left
            .written_from_the_desktop
            .iter()
            .map(|(boundary, _)| boundary.as_str())
            .collect();
        assert!(
            written.contains(&asked::THE_BOUNDARY_TELEGRAM),
            "the telegram boundary was never written: {written:?}"
        );
        assert_eq!(
            left.written_from_the_desktop
                .iter()
                .find(|(boundary, _)| boundary == asked::THE_BOUNDARY_TELEGRAM)
                .map(|(_, mark)| mark.as_str()),
            Some(asked::IT_PERFORMED)
        );
        assert!(
            matches!(
                left.came_to,
                WhereTheSetUpGot::TheRunEnded {
                    ending: TheRunsEnding::ItReachedDone { .. },
                    ..
                }
            ),
            "the run did not reach done: {:?}",
            left.came_to
        );

        // **`Telegram` L1's told-record bookkeeping was handed what the machine
        // said about each file**, which is what moves a telling — **and then
        // the role's state, on its own acknowledgement and after both files**
        // (the 38's piece 11: until then nothing told the record the state, so
        // Settings named every server set up with Telegram as not reached).
        assert_eq!(left.the_placement_was_answered.len(), 3);
        assert!(left
            .the_placement_was_answered
            .iter()
            .all(|(_, carried_out)| *carried_out));
        assert_eq!(
            left.the_placement_was_answered[2].0, THE_ROLES_STATE_ACKNOWLEDGED,
            "the role's state was not told to the record after the files"
        );
    }

    /// **`Server Setup` G2: material is placed by whichever entry point runs
    /// while Telegram is on, and is not placed in advance — so a run with it
    /// off never meets any of this.**
    #[test]
    fn a_run_with_telegram_off_never_meets_the_step_at_all() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("telegram-off");
        let left = the_order_is_driven(
            &scratch,
            sftp_server,
            a_machine_whose_driver_answers(),
            an_ubuntu_systemd_machine(),
            WhatThePlacementReached::Verified,
            None,
        );

        // **The step is not in the run at all**, so nothing of it is composed,
        // written or asked.
        assert!(
            !left.composed.contains(&asked::THE_BOUNDARY_TELEGRAM.to_string()),
            "the telegram boundary was composed into a run with Telegram off: {:?}",
            left.composed
        );
        assert_eq!(
            left.the_composing_was_asked, 0,
            "the material was composed for a run nobody asked Telegram for"
        );
        assert_eq!(
            left.the_roles_state, None,
            "the role's desired state was written on a run with Telegram off"
        );
        assert!(!scratch.at("smoozy/package/telegram-token").exists());
        assert!(!scratch
            .at("smoozy/package/telegram-role-configuration")
            .exists());
        // **And the run is otherwise unchanged**: it still reaches done.
        assert!(
            matches!(
                left.came_to,
                WhereTheSetUpGot::TheRunEnded {
                    ending: TheRunsEnding::ItReachedDone { .. },
                    ..
                }
            ),
            "the run did not reach done: {:?}",
            left.came_to
        );
    }

    // ═══════════════════ The Creator's eleventh notes, note 2

    /// **The local record, lent the way the running application lends it.**
    struct ARecordHere(smoozy_desktop::record::LocalRecord);

    impl smoozy_desktop::keys::LendsTheRecord for ARecordHere {
        fn lend(&self, act: &mut dyn FnMut(&rusqlite::Connection)) {
            act(self.0.connection())
        }
        fn write_a_diary_line(&self, _act: &smoozy_keys::diary::Act) {}
    }

    /// **The bot token's one item, in memory** — a made-up token, never a real
    /// one — **or a store that answers a read with one of `Security and
    /// licence` §3.0 rule 4's three failures.**
    #[derive(Clone, Default)]
    struct AStoreHere {
        held: Arc<Mutex<Option<Vec<u8>>>>,
        a_read_is_answered: Option<smoozy_security_and_licence::secret_store::GetFailure>,
    }

    impl smoozy_security_and_licence::secret_store::SecretStore for AStoreHere {
        fn put(
            &self,
            _: &smoozy_security_and_licence::held::HeldSecret,
            value: smoozy_security_and_licence::plaintext::Plaintext,
            _: std::time::Duration,
        ) -> Result<(), smoozy_security_and_licence::secret_store::StoreFailure> {
            *self.held.lock().expect("the item") = Some(value.expose(|bytes| bytes.to_vec()));
            Ok(())
        }
        fn get(
            &self,
            _: &smoozy_security_and_licence::held::HeldSecret,
            _: std::time::Duration,
        ) -> Result<
            smoozy_security_and_licence::plaintext::Plaintext,
            smoozy_security_and_licence::secret_store::GetFailure,
        > {
            if let Some(failure) = self.a_read_is_answered {
                return Err(failure);
            }
            self.held
                .lock()
                .expect("the item")
                .clone()
                .map(smoozy_security_and_licence::plaintext::Plaintext::new)
                .ok_or(smoozy_security_and_licence::secret_store::GetFailure::NotThere)
        }
        fn erase(
            &self,
            _: &smoozy_security_and_licence::held::HeldSecret,
            _: std::time::Duration,
        ) -> Result<(), smoozy_security_and_licence::secret_store::StoreFailure> {
            *self.held.lock().expect("the item") = None;
            Ok(())
        }
        fn probe(&self, _: std::time::Duration) -> smoozy_security_and_licence::secret_store::ProbeOutcome {
            smoozy_security_and_licence::secret_store::ProbeOutcome::RoundTripWorked
        }
        fn enumerate(
            &self,
            _: std::time::Duration,
        ) -> Result<
            Vec<smoozy_security_and_licence::held::HeldSecret>,
            smoozy_security_and_licence::secret_store::StoreFailure,
        > {
            Ok(Vec::new())
        }
    }

    /// **The token Smoozy's own side stored**, made up for the test.
    const THE_STORED_TOKEN: &[u8] = b"7:the-token-stored-for-a-test";

    /// **Smoozy's own side for one new server, as the Creator's eleventh notes
    /// found it**: a fresh local record holding the server's row as an add
    /// leaves it — **no root recorded and no installation confirmed** — a bot
    /// proved, the record's own *token stored* fact for it, and Telegram
    /// switched on. **What the store holds is the test's to say.**
    struct TheDesksOwnSide {
        record: ARecordHere,
        store: AStoreHere,
        server: String,
    }

    impl TheDesksOwnSide {
        fn for_a_new_server(scratch: &Scratch, store: AStoreHere) -> TheDesksOwnSide {
            use smoozy_telegram::proof::{KeepsTheProvedBot, TheBot, TheConversation};
            let folder = scratch.at("the-desk");
            std::fs::create_dir_all(&folder).expect("the desk's own folder");
            let record =
                smoozy_desktop::record::LocalRecord::open_at(&folder.join("local-record.sqlite3"))
                    .expect("the record opens");
            let c = record.connection();
            // **The record reads back only a row whose identifier is an issued
            // one** (`Servers` C6), so the new server is given one.
            let row = ServerRow {
                local_server_identifier: Identity::issue(IdentityKind::LocalServer)
                    .expect("randomness")
                    .as_text(),
                ..a_row()
            };
            assert!(row.recorded_machine_root.is_none() && row.last_confirmed_installation_identity.is_none());
            smoozy_desktop::record::server_rows::add(c, &row).expect("the row is added");
            let bot = TheBot { id: 7, username: "smoozy_a7k2_bot".into() };
            let keeper = smoozy_desktop::record::telegram_bot::TheProvedBotRecord::over(c);
            keeper
                .the_proof_was_recorded(&bot, TheConversation(9001))
                .expect("the proof");
            keeper.the_token_was_stored_for(&bot).expect("the stored fact");
            smoozy_desktop::record::settings::telegram_was_switched(c, true).expect("Telegram is on");
            TheDesksOwnSide {
                server: row.local_server_identifier,
                record: ARecordHere(record),
                store,
            }
        }

        fn the_material(&self) -> TheMaterialFromTheRecord<'_> {
            TheMaterialFromTheRecord {
                record: &self.record,
                server: self.server.clone(),
                store: Some(Box::new(self.store.clone())),
            }
        }

        /// **The record's own *token stored* fact**, as it now stands.
        fn the_stored_fact(&self) -> Option<i64> {
            smoozy_desktop::record::telegram_bot::TheProvedBotRecord::over(self.record.0.connection())
                .the_bot_whose_token_is_stored()
                .expect("the stored fact reads")
        }

        /// **What Settings' Telegram section draws for the token** — its dots
        /// where `true`, *The token is not stored on this computer.* and
        /// *Replace…* where `false`.
        fn settings_draws_the_token_as_stored(&self) -> bool {
            smoozy_desktop::surface::telegram::what_the_section_holds(self.record.0.connection(), None)
                .the_token_is_held
        }
    }

    /// **A store holding the token Smoozy stored.**
    fn a_store_holding_the_token() -> AStoreHere {
        let store = AStoreHere::default();
        *store.held.lock().expect("the item") = Some(THE_STORED_TOKEN.to_vec());
        store
    }

    /// **The order driven on a new server by Smoozy's own side**, with
    /// *Telegram is on* asked of the record before the run, which is where the
    /// running application asks it (`setting_up::the_setup_on`).
    fn a_new_server_is_set_up_by(scratch: &Scratch, sftp_server: PathBuf, desk: &Arc<TheDesksOwnSide>) -> (bool, WhatTheRunLeft) {
        a_new_server_is_set_up_and_read_by(scratch, sftp_server, desk, false)
    }

    /// [`a_new_server_is_set_up_by`], **with the setup read back afterwards
    /// the way the Set up tab reads it** where `read_it_back` says so.
    fn a_new_server_is_set_up_and_read_by(
        scratch: &Scratch,
        sftp_server: PathBuf,
        desk: &Arc<TheDesksOwnSide>,
        read_it_back: bool,
    ) -> (bool, WhatTheRunLeft) {
        let telegram_is_on = desk.the_material().it_is_on();
        let left = the_order_is_driven_as(
            scratch,
            sftp_server,
            a_machine_whose_driver_answers(),
            an_ubuntu_systemd_machine(),
            WhatThePlacementReached::Verified,
            None,
            AsThisRunIsAsked {
                telegram_is_on,
                the_desks_own_side: Some(Arc::clone(desk)),
                read_the_setup_at_the_end: read_it_back,
                ..AsThisRunIsAsked::default()
            },
        );
        (telegram_is_on, left)
    }

    /// **`Specs/Telegram` and `Server Setup` G2: a new server added while
    /// Telegram is on is given Telegram by its ordinary first setup** — the
    /// Creator's eleventh notes, note 2, cause 2.
    ///
    /// **Until 2026-10-17 *Telegram is on* was answered by composing the
    /// material there and then**, before the run, on a row the hand-over had
    /// not yet given a folder or an installation — so on every new server the
    /// answer was *off* and `Server Setup` R1 item 8 was dropped without a word.
    /// **It is answered from the switch, the bot on record and the record's own
    /// *token stored* fact**, and the material is composed at the step, after
    /// the hand-over, from the row that hand-over wrote.
    #[test]
    fn a_new_server_set_up_with_a_bot_on_record_reaches_the_telegram_step() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("e1-new-server");
        let desk = Arc::new(TheDesksOwnSide::for_a_new_server(&scratch, a_store_holding_the_token()));
        let (telegram_is_on, left) = a_new_server_is_set_up_by(&scratch, sftp_server, &desk);

        assert!(
            telegram_is_on,
            "a new server with a bot on record and its token stored was set up with Telegram off"
        );
        assert!(
            left.composed.contains(&asked::THE_BOUNDARY_TELEGRAM.to_string()),
            "`Server Setup` R1 item 8 was not composed into the new server's run: {:?}",
            left.composed
        );
        let written: Vec<&str> = left
            .written_from_the_desktop
            .iter()
            .map(|(boundary, _)| boundary.as_str())
            .collect();
        assert!(
            written.contains(&asked::THE_BOUNDARY_TELEGRAM),
            "the telegram boundary was never written: {written:?}"
        );
        // **The token the store holds is the one on the machine**, at the path
        // `Security and licence` §10 fixes — composed from the root the
        // hand-over recorded in Smoozy's own record.
        assert_eq!(
            std::fs::read(scratch.at("smoozy/package/telegram-token")).expect("the token file reads"),
            THE_STORED_TOKEN
        );
        assert!(scratch.at("smoozy/package/telegram-role-configuration").exists());
        assert_eq!(left.the_roles_state, Some(true));
        assert!(
            matches!(
                left.came_to,
                WhereTheSetUpGot::TheRunEnded {
                    ending: TheRunsEnding::ItReachedDone { .. },
                    ..
                }
            ),
            "the run did not reach done: {:?}",
            left.came_to
        );
        assert_eq!(desk.the_stored_fact(), Some(7), "a placement that worked forgot nothing");
    }

    /// **`Security and licence` §3.0 rule 4: only *not there* asks the user
    /// again, as though nothing was remembered** — the Creator's eleventh
    /// notes, note 2, cause 3, and the runner's ruling on the words.
    ///
    /// **The record's *token stored* fact is forgotten**, so Settings draws
    /// *The token is not stored on this computer.* and *Replace…* (rules 4 and
    /// 6), and the card says so in one plain sentence. **Until 2026-10-17 the
    /// card said *The server could not install Telegram reporting.* with the
    /// reason's internal name behind *What the server said*, and the fact stood**
    /// — so Settings kept drawing the token as stored.
    #[test]
    fn a_token_the_store_no_longer_holds_is_said_plainly_and_forgotten() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("e1-not-there");
        let desk = Arc::new(TheDesksOwnSide::for_a_new_server(&scratch, AStoreHere::default()));
        assert!(desk.settings_draws_the_token_as_stored(), "the record said stored before the run");
        let (_, left) = a_new_server_is_set_up_by(&scratch, sftp_server, &desk);

        // **The card's own shape for a setup that stopped, with the ruled
        // sentence as its reason** — and the same sentence alone for *Install
        // it*, whose answer carries no *Setup stopped.*
        const THE_SENTENCE: &str =
            "Telegram token no longer saved on this computer. Replace it in Settings.";
        let said = smoozy_desktop::surface::set_up::what_it_came_to(&left.came_to);
        assert_eq!(
            said.said,
            format!("Setup stopped. {THE_SENTENCE}"),
            "{:?}",
            left.came_to
        );
        assert_eq!(
            smoozy_desktop::surface::set_up::what_the_telegram_placement_came_to(&left.came_to).said,
            THE_SENTENCE
        );
        assert_eq!(
            said.the_machines_own_words, None,
            "Smoozy's own sentence was put behind *What the server said*"
        );
        assert_eq!(desk.the_stored_fact(), None, "the *token stored* fact was not forgotten");
        assert!(
            !desk.settings_draws_the_token_as_stored(),
            "Settings still draws the token as stored"
        );
        assert!(!scratch.at("smoozy/package/telegram-token").exists());
        // **`Server Setup` S2a: the step that failed is written onto the
        // machine**, so its own record says *failed* rather than *cut*.
        assert!(
            left.failures_recorded.iter().any(|(_, _, words)| words == THE_SENTENCE),
            "the failed step was not written onto the machine: {:?}",
            left.failures_recorded
        );
    }

    /// **The other two of rule 4's classes are ordinary failures**: Smoozy says
    /// it could not read what it remembered, offers the act again, and
    /// **forgets nothing** — the runner's ruling on the words.
    #[test]
    fn a_store_that_will_not_give_the_token_up_keeps_it_stored_and_says_so() {
        use smoozy_security_and_licence::secret_store::GetFailure;
        let _deadline = TheTestsDeadline::started();
        for (named, failure) in [
            ("e1-locked", GetFailure::LockedOrRefused),
            ("e1-unavailable", GetFailure::StoreUnavailable),
        ] {
            let sftp_server = the_server_or_say_so!();
            let scratch = Scratch::new(named);
            let store = AStoreHere {
                a_read_is_answered: Some(failure),
                ..a_store_holding_the_token()
            };
            let desk = Arc::new(TheDesksOwnSide::for_a_new_server(&scratch, store));
            let (telegram_is_on, left) = a_new_server_is_set_up_by(&scratch, sftp_server, &desk);
            assert!(telegram_is_on);

            let said = smoozy_desktop::surface::set_up::what_it_came_to(&left.came_to);
            assert_eq!(
                said.said, "Setup stopped. Could not read the saved Telegram token.",
                "{failure:?}: {:?}",
                left.came_to
            );
            assert_eq!(said.the_machines_own_words, None);
            assert_eq!(desk.the_stored_fact(), Some(7), "{failure:?} forgot the stored fact");
            assert!(desk.settings_draws_the_token_as_stored(), "{failure:?}");
            assert!(!scratch.at("smoozy/package/telegram-token").exists());
        }
    }

    // ═══════════════════ The Creator's twelfth notes, note 2

    /// **The step that failed, as the Set up tab draws it from the setup read
    /// back off the machine**: the step's name, with spaces made plain, and
    /// its failure.
    fn the_failed_step_read_back(
        left: WhatTheRunLeft,
    ) -> (String, smoozy_desktop::surface::set_up::TheFailedStep) {
        let read = left.the_setup_read_at_the_end.expect("the setup was read back");
        let tab = smoozy_desktop::surface::set_up::the_tab(
            smoozy_desktop::surface::set_up::WhatTheMachineSaid::ASetupRunAlone(Box::new(read)),
            "3060",
        );
        let panel = tab.the_stopped_setup.expect("the stopped-setup panel");
        let mut failed = panel.steps.into_iter().filter(|step| step.the_failure.is_some());
        let step = failed.next().expect("no step carries the failure");
        assert!(failed.next().is_none(), "more than one step carries the failure");
        (
            step.name.as_str().replace('\u{a0}', " "),
            step.the_failure.expect("the failure"),
        )
    }

    /// **A Telegram step that stopped on this computer reads back as Smoozy's
    /// own sentence, with nothing behind *What the server said*** —
    /// `Server Setup` S2a's *"no tool words are claimed for it"* and Rule 8.
    ///
    /// **Until 2026-10-18 the run panel read *The server could not install
    /// Telegram reporting.*, with Smoozy's sentence behind *What the server
    /// said***: the composing failure is written onto the machine as the
    /// failed step's words, and every word read back was drawn as the
    /// server's. The server did nothing. The Creator's twelfth notes, note 2,
    /// on the server *3060*.
    #[test]
    fn a_token_the_store_no_longer_holds_reads_back_as_smoozys_own_sentence() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("w2-not-there-read-back");
        let desk = Arc::new(TheDesksOwnSide::for_a_new_server(&scratch, AStoreHere::default()));
        let (_, left) = a_new_server_is_set_up_and_read_by(&scratch, sftp_server, &desk, true);
        const THE_SENTENCE: &str =
            "Telegram token no longer saved on this computer. Replace it in Settings.";
        assert!(
            left.failures_recorded.iter().any(|(_, _, words)| words == THE_SENTENCE),
            "the failed step was not written onto the machine: {:?}",
            left.failures_recorded
        );

        let (name, failed) = the_failed_step_read_back(left);
        assert_eq!(name, "Install Telegram reporting");
        assert_eq!(
            failed.said.lead.as_str().replace('\u{a0}', " "),
            THE_SENTENCE,
            "the step's sentence is not Smoozy's own"
        );
        assert!(
            failed.the_output.is_none() && failed.the_complaint.is_none(),
            "Smoozy's own sentence was put behind *What the server said*: {failed:?}"
        );
        assert!(
            failed.what_the_server_said.is_none(),
            "*What the server said* is offered with no server words behind it"
        );
    }

    /// **A Telegram step the machine refused still reads as the step's own
    /// sentence, with the machine's words behind *What the server said*** —
    /// the matching is Smoozy's own closed list of composing sentences and
    /// nothing wider. Added 2026-10-18, the Creator's twelfth notes, note 2.
    #[test]
    fn a_telegram_step_the_machine_refused_reads_back_with_its_words() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("w2-machine-refused");
        const THE_MACHINES_WORDS: &str =
            "the role worker could not be started (words written for this test)";
        let left = the_order_is_driven_as(
            &scratch,
            sftp_server,
            a_machine_whose_driver_answers(),
            an_ubuntu_systemd_machine(),
            WhatThePlacementReached::Verified,
            None,
            AsThisRunIsAsked {
                telegram_is_on: true,
                read_the_setup_at_the_end: true,
                the_role_is_refused_with: Some(THE_MACHINES_WORDS),
                ..AsThisRunIsAsked::default()
            },
        );
        assert!(
            matches!(
                &left.came_to,
                WhereTheSetUpGot::TheRunEnded {
                    ending: TheRunsEnding::AStepFailed { said, .. },
                    ..
                } if said == THE_MACHINES_WORDS
            ),
            "the refusal did not end the run as a failed step in the machine's words: {:?}",
            left.came_to
        );

        let (name, failed) = the_failed_step_read_back(left);
        assert_eq!(name, "Install Telegram reporting");
        assert_eq!(
            failed.said.lead.as_str().replace('\u{a0}', " "),
            "Could not install Telegram reporting."
        );
        assert_eq!(failed.the_output.as_ref().map(|it| it.as_str()), Some(THE_MACHINES_WORDS));
        assert!(
            failed.what_the_server_said.is_some(),
            "the machine's words have no *What the server said* to open them"
        );
    }

    /// **`Server Setup` Y8's stop is the machine's own durable desired state,
    /// and the desktop's half is writing it.**
    ///
    /// **Until 2026-09-20 it was only the desktop's waiting**: a user who
    /// stopped a setup stopped the desktop watching it and did not stop the
    /// machine, so the setup worker went on taking whatever it was commanded and
    /// only `Server Setup` Y7's refresh window ever ended the run.
    ///
    /// **It goes out on `Rented-server runtime` X8's ordinary command path**,
    /// naming the run being stopped (that document's X7), and no signal goes
    /// anywhere — the far end here would record one, and there is none to
    /// record.
    #[test]
    fn a_stop_is_written_as_the_machines_own_durable_desired_state() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("stop-written");
        let asked_to_stop = Arc::new(Mutex::new(true));
        let left = the_order_is_driven(
            &scratch,
            sftp_server,
            a_machine_whose_driver_answers(),
            an_ubuntu_systemd_machine(),
            WhatThePlacementReached::Verified,
            Some(asked_to_stop),
        );
        assert!(
            matches!(
                left.came_to,
                WhereTheSetUpGot::TheRunEnded {
                    ending: TheRunsEnding::TheUserStoppedIt,
                    ..
                }
            ),
            "the stop was not honoured: {:?}",
            left.came_to
        );
        // **The stop reached the machine, and it named this run.**
        assert!(
            left.the_stop_was_written_for.is_some(),
            "the stop never left the desk, so the machine was never stopped"
        );
        // **`Server Setup` Y8b: nothing was cleared.** This run set no state —
        // its driver answers, so nothing machine-changing was ever issued — and
        // no clear went out either.
        assert!(
            left.cleared_by.is_empty(),
            "stopping a setup cleared the machine-level state: {:?}",
            left.cleared_by
        );
        assert!(
            left.written_from_the_desktop.is_empty(),
            "a stopped run was given a boundary: {:?}",
            left.written_from_the_desktop
        );
    }

    #[test]
    fn a_stopped_run_writes_no_boundary_from_the_desktop() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("stopped-boundary");
        let asked_to_stop = Arc::new(Mutex::new(true));
        let left = the_order_is_driven(
            &scratch,
            sftp_server,
            a_machine_whose_driver_answers(),
            an_ubuntu_systemd_machine(),
            WhatThePlacementReached::Verified,
            Some(asked_to_stop),
        );
        assert!(
            matches!(
                left.came_to,
                WhereTheSetUpGot::TheRunEnded {
                    ending: TheRunsEnding::TheUserStoppedIt,
                    ..
                }
            ),
            "the stop was not honoured: {:?}",
            left.came_to
        );
        assert!(
            left.written_from_the_desktop.is_empty(),
            "a stopped run was given a boundary: {:?}",
            left.written_from_the_desktop
        );
    }

    /// **`Server Setup` Y8: the stop is honoured at a boundary and never by a
    /// signal.**
    #[test]
    fn a_stop_ends_the_run_at_a_boundary_and_commands_no_step() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("stopped");
        let asked = Arc::new(Mutex::new(true));
        let left = the_order_is_driven(
            &scratch,
            sftp_server,
            a_machine_that_answers(),
            an_ubuntu_systemd_machine(),
            WhatThePlacementReached::Verified,
            Some(asked),
        );
        assert!(
            matches!(
                left.came_to,
                WhereTheSetUpGot::TheRunEnded {
                    ending: TheRunsEnding::TheUserStoppedIt,
                    ..
                }
            ),
            "the stop was not honoured: {:?}",
            left.came_to
        );
        // **The run was opened and no step was commanded**, because the stop is
        // read at the boundary before the first one.
        assert!(left.steps.is_empty(), "a step ran after the stop");
        assert!(
            !left.composed.is_empty(),
            "the run was never opened, so nothing was stopped"
        );
    }

    // ═══════ `Server Setup` Y7 and `Server Setup` Y8 through a slow admin step

    /// **The driver install, as the elevated command the run sends for it on
    /// a login that needs no password.**
    fn the_driver_install() -> String {
        smoozy_server_setup::elevated::THE_UBUNTU_DRIVER_INSTALL
            .elevated
            .as_str()
            .to_string()
    }

    /// **The slow driver install, with its driver answering once it is in.**
    fn a_slow_driver_install(
        how_long: std::time::Duration,
        what_became_of_it: &Arc<Mutex<Vec<(&'static str, std::time::Instant)>>>,
    ) -> TheSlowCommand {
        TheSlowCommand {
            command: the_driver_install(),
            how_long,
            after_it: the_driver_answers(
                Vec::new(),
                ended_cleanly("GPU-1111,0000:01:00.0,NVIDIA L40,46068 MiB\n"),
            ),
            the_user_stops_as_it_starts: None,
            the_machine_ends_the_run_as_it_starts: false,
            what_became_of_it: Arc::clone(what_became_of_it),
        }
    }

    /// **What became of the slow command, without the moments.**
    fn what_became_of(
        it: &Arc<Mutex<Vec<(&'static str, std::time::Instant)>>>,
    ) -> Vec<&'static str> {
        it.lock().expect("the slow command").iter().map(|(what, _)| *what).collect()
    }

    /// **Every elevated command the desk sent after the slow one** — which is
    /// what *"no further admin command"* is about. Requests to the machine's
    /// own program ride their own command and are not counted here.
    fn admin_commands_after_the_slow_one(asked: &[String]) -> Vec<String> {
        let slow = the_driver_install();
        asked
            .iter()
            .skip_while(|one| **one != slow)
            .skip(1)
            .filter(|one| one.starts_with("sudo"))
            .cloned()
            .collect()
    }

    /// **Step 10c's eighth pass on Hyperstack, played at the desk: an admin
    /// step longer than the machine's window, and the run kept alive through
    /// it.** Added 2026-09-27, eighth-pass fix piece 1.
    ///
    /// **What happened there.** The machine ends a setup run that nobody
    /// refreshes for 90 seconds (`Server Setup` Y7's window). The desk ran the
    /// driver install for 52 seconds after two other admin steps and refreshed
    /// nothing while any of them ran, so the machine ended the run at
    /// *started*; the desk sent the next admin command anyway, and then told
    /// the user the graphics driver could not be installed, with *"no setup
    /// run is open on this machine"* behind *What the server said*.
    ///
    /// **Here** the window is 20 seconds and the install 35, and the desk's
    /// own cadence is `Server Setup`'s real one. **`Server Setup` Y7's third
    /// bullet is what is proved**: the refresh goes on at its cadence through the admin step,
    /// the machine sees it while the install runs, and the run goes on to
    /// *done*. **Proved to bite** in a copy outside the repository — the
    /// account of eighth-pass fix pieces 1 and 2 says how it ended there.
    #[test]
    fn a_slow_admin_step_past_the_machines_window_ends_with_the_run_alive() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("slow-admin-step");
        let what_became_of_it = Arc::new(Mutex::new(Vec::new()));
        let left = the_order_is_driven_as(
            &scratch,
            sftp_server,
            a_machine_with_a_card_and_no_driver(),
            an_ubuntu_systemd_machine(),
            WhatThePlacementReached::Verified,
            None,
            AsThisRunIsAsked {
                install_this_blender: None,
                slow: Some(a_slow_driver_install(
                    std::time::Duration::from_secs(35),
                    &what_became_of_it,
                )),
                window: Some(std::time::Duration::from_secs(20)),
                ..AsThisRunIsAsked::default()
            },
        );
        assert!(
            matches!(
                left.came_to,
                WhereTheSetUpGot::TheRunEnded {
                    ending: TheRunsEnding::ItReachedDone { .. },
                    ..
                }
            ),
            "the run did not live through the slow admin step: {:?}",
            left.came_to
        );
        assert_eq!(what_became_of(&what_became_of_it), vec!["began", "ended"]);
        assert_ne!(
            left.the_machines_ending.as_ref().map(|(ending, _)| ending.as_str()),
            Some(asked::IT_ENDED_AT_ITS_BOUNDARY_NOT_COMPLETED),
            "the machine's window ended the run"
        );
        assert!(
            left.written_from_the_desktop
                .iter()
                .any(|(boundary, _)| boundary == asked::THE_BOUNDARY_DRIVER_PLACED),
            "the driver step's boundary was never written: {:?}",
            left.written_from_the_desktop
        );
        // **The refresh reached the machine while the install ran** — at
        // least twice in 35 seconds at a cadence of 15, which is what kept a
        // 20-second window from closing.
        let moments = what_became_of_it.lock().expect("the slow command").clone();
        let (began, ended) = (moments[0].1, moments[1].1);
        let during = left
            .refreshed_at
            .iter()
            .filter(|at| **at > began && **at < ended)
            .count();
        assert!(
            during >= 2,
            "the run was refreshed {during} times while the admin step ran"
        );
    }

    /// **`Server Setup` Y8 during an admin step: the user's *Stop* ends the
    /// driving side's part at once — the desk stops issuing and closes the
    /// step's channel — and the run ends at the last boundary passed.** Added
    /// 2026-09-27, eighth-pass fix piece 1.
    ///
    /// **Until then the stop was read between steps and at a boundary only**,
    /// so a stop pressed during a driver install waited for the install to end
    /// and the desk sent the next admin command before reading it. **Here the
    /// install would run two minutes**; the stop is pressed as it starts and
    /// read at the next refresh. **Proved to bite** in a copy outside the
    /// repository.
    #[test]
    fn stop_during_a_slow_admin_step_ends_the_run_at_its_last_boundary_and_sends_nothing_after() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("stop-in-admin-step");
        let what_became_of_it = Arc::new(Mutex::new(Vec::new()));
        let asked_to_stop = Arc::new(Mutex::new(false));
        let mut slow =
            a_slow_driver_install(std::time::Duration::from_secs(120), &what_became_of_it);
        slow.the_user_stops_as_it_starts = Some(Arc::clone(&asked_to_stop));
        let left = the_order_is_driven_as(
            &scratch,
            sftp_server,
            a_machine_with_a_card_and_no_driver(),
            an_ubuntu_systemd_machine(),
            WhatThePlacementReached::Verified,
            Some(asked_to_stop),
            AsThisRunIsAsked {
                install_this_blender: None,
                slow: Some(slow),
                ..AsThisRunIsAsked::default()
            },
        );
        assert!(
            matches!(
                left.came_to,
                WhereTheSetUpGot::TheRunEnded {
                    at: smoozy_server_setup::run::Boundary::Started,
                    ending: TheRunsEnding::TheUserStoppedIt,
                }
            ),
            "the stop did not end the run at its last boundary: {:?}",
            left.came_to
        );
        assert_eq!(
            what_became_of(&what_became_of_it),
            vec!["began", "its channel was closed"],
            "the admin step's channel was not closed"
        );
        assert_eq!(
            admin_commands_after_the_slow_one(&left.asked),
            Vec::<String>::new(),
            "an admin command went out after the stop"
        );
        assert!(
            left.written_from_the_desktop.is_empty(),
            "a boundary was written after the stop: {:?}",
            left.written_from_the_desktop
        );
        assert!(
            left.the_stop_was_written_for.is_some(),
            "the stop was never written onto the machine"
        );
        assert!(left.failures_recorded.is_empty(), "a stop was recorded as a failure");
    }

    /// **A run the machine ends while an admin step runs: nothing further is
    /// sent, the step's channel is closed, and the box tells the run's own
    /// recorded ending.** Added 2026-09-27, eighth-pass fix piece 1.
    ///
    /// **The machine ends the run as the install starts**, the way its window
    /// ends a run nobody refreshes. The desk's next refresh reads the ending;
    /// the install's channel is closed, the support binaries' install is never
    /// sent, no boundary is written, and **the machine's own ending is not
    /// written over** by a failure the desk would otherwise record. **The box
    /// says the step stopped part-way because the server ended the setup** —
    /// `Design/Server Setup § Cut off part-way` — and never that the server
    /// could not install the graphics driver. **Proved to bite** in a copy
    /// outside the repository.
    #[test]
    fn a_run_the_machine_ends_during_an_admin_step_sends_nothing_after_and_is_told_as_it_ended() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("ended-in-admin-step");
        let what_became_of_it = Arc::new(Mutex::new(Vec::new()));
        let mut slow =
            a_slow_driver_install(std::time::Duration::from_secs(120), &what_became_of_it);
        slow.the_machine_ends_the_run_as_it_starts = true;
        let left = the_order_is_driven_as(
            &scratch,
            sftp_server,
            a_machine_with_a_card_and_no_driver(),
            an_ubuntu_systemd_machine(),
            WhatThePlacementReached::Verified,
            None,
            AsThisRunIsAsked {
                install_this_blender: None,
                slow: Some(slow),
                ..AsThisRunIsAsked::default()
            },
        );
        assert!(
            matches!(
                &left.came_to,
                WhereTheSetUpGot::TheRunEnded {
                    at: smoozy_server_setup::run::Boundary::Started,
                    ending: TheRunsEnding::TheServerEndedTheRun {
                        step: smoozy_server_setup::ready_action::Step::Driver,
                        ..
                    },
                }
            ),
            "the run the machine ended was not told as it ended: {:?}",
            left.came_to
        );
        assert_eq!(
            what_became_of(&what_became_of_it),
            vec!["began", "its channel was closed"],
            "the admin step's channel was not closed"
        );
        assert_eq!(
            admin_commands_after_the_slow_one(&left.asked),
            Vec::<String>::new(),
            "an admin command went out after the run was seen ended"
        );
        assert!(left.written_from_the_desktop.is_empty());
        assert!(
            left.failures_recorded.is_empty(),
            "a failure was recorded over the machine's own ending: {:?}",
            left.failures_recorded
        );
        assert_eq!(
            left.the_machines_ending,
            Some((asked::IT_ENDED_AT_ITS_BOUNDARY_NOT_COMPLETED.to_string(), None))
        );
        let told = smoozy_desktop::surface::set_up::what_it_came_to(&left.came_to);
        assert_eq!(
            told.said,
            "Setup stopped. Installing the GPU driver stopped: the server ended the setup."
        );
    }

    /// **The same machine ending, met at the boundary write rather than by a
    /// refresh** — the admin step is over before the next tick, so the desk
    /// learns of the ending only when the machine refuses *driver placed*.
    /// **That refusal is told as the run's recorded ending too, with the
    /// machine's refusal behind *What the server said***, and not as the
    /// driver failing — which is exactly how step 10c's eighth pass on
    /// Hyperstack was told it. Added 2026-09-27, eighth-pass fix piece 1.
    #[test]
    fn a_boundary_the_machine_refuses_after_ending_the_run_is_told_as_the_runs_recorded_ending() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("refused-boundary");
        let what_became_of_it = Arc::new(Mutex::new(Vec::new()));
        let mut slow = a_slow_driver_install(std::time::Duration::ZERO, &what_became_of_it);
        slow.the_machine_ends_the_run_as_it_starts = true;
        let left = the_order_is_driven_as(
            &scratch,
            sftp_server,
            a_machine_with_a_card_and_no_driver(),
            an_ubuntu_systemd_machine(),
            WhatThePlacementReached::Verified,
            None,
            AsThisRunIsAsked {
                install_this_blender: None,
                slow: Some(slow),
                ..AsThisRunIsAsked::default()
            },
        );
        assert_eq!(
            left.came_to,
            WhereTheSetUpGot::TheRunEnded {
                at: smoozy_server_setup::run::Boundary::Started,
                ending: TheRunsEnding::TheServerEndedTheRun {
                    step: smoozy_server_setup::ready_action::Step::Driver,
                    said: Some("no setup run is open on this machine".to_string()),
                },
            }
        );
        let told = smoozy_desktop::surface::set_up::what_it_came_to(&left.came_to);
        assert!(
            !told.said.contains("Could not install the GPU driver"),
            "the box blamed the driver install: {}",
            told.said
        );
        assert_eq!(
            told.the_machines_own_words.as_deref(),
            Some("no setup run is open on this machine")
        );
    }

    /// **`Rented-server runtime` G3's phase is the caller's answer**, and a
    /// placement that did not verify stops the order before a run is opened.
    #[test]
    fn a_placement_that_did_not_verify_opens_no_run() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("deferred");
        let left = the_order_is_driven(
            &scratch,
            sftp_server,
            a_machine_that_answers(),
            an_ubuntu_systemd_machine(),
            WhatThePlacementReached::RolledBack,
            None,
        );
        assert_eq!(
            left.came_to,
            WhereTheSetUpGot::ThePlacement {
                reached: WhatThePlacementReached::RolledBack
            }
        );
        assert!(left.composed.is_empty(), "a run was opened after a rollback");
        // **The installation is not recorded against the row**, because no
        // installation of Smoozy stands on that machine.
        assert!(
            left.row.last_confirmed_installation_identity.is_none(),
            "an installation was recorded for a placement that rolled back"
        );
        assert_eq!(
            left.row.event_cursor,
            Some(97),
            "the cursor moved on a placement that rolled back"
        );
        // **`Servers` Q19a: a placement that does not commit takes nothing
        // again.**
        assert!(
            !left.writes.iter().any(|one| one == "the placement committed"),
            "a commit was told for a placement that rolled back"
        );
    }

    /// **Step 10c's thirteenth-pass fix piece 2: a setup deferred by a job
    /// that was running goes ahead at the next press once the job has
    /// ended.** `Server protocol and transfers` B6 and `Rented-server runtime`
    /// G22: a deferred result *"does not license a second attempt under a
    /// fresh identity"*, and `Rented-server runtime` G18 answers a fresh
    /// identity that meets *deferred* with that deferral and acts on nothing.
    /// **So the second press is sent under the attempt the deferral recorded**
    /// (`Rented-server runtime` G17's *deferred* row), the way `Desktop app and delivery` V12 sends an
    /// update's — read from the phase record the machine holds.
    ///
    /// **Until this piece every press issued a fresh attempt**, and the
    /// thirteenth pass on Hyperstack met it: *Set up this server* deferred at
    /// 04:09 while *p13 lion* rendered, and at 04:53, after it had ended, the
    /// second press was refused by the first deferral whatever stood on the
    /// machine.
    #[test]
    fn a_setup_deferred_by_a_running_job_goes_ahead_at_the_next_press_once_it_has_ended() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("deferred-then-again");
        let work = Arc::new(Mutex::new(vec![(
            "p13 lion".to_string(),
            AcceptedWorkStanding::Live,
        )]));

        // **The first press, while a job renders: deferred**, and no run opened.
        let first = the_order_is_driven_as(
            &scratch,
            sftp_server.clone(),
            a_machine_that_answers(),
            an_ubuntu_systemd_machine(),
            WhatThePlacementReached::Verified,
            None,
            AsThisRunIsAsked {
                the_work_the_commit_reads: Some(Arc::clone(&work)),
                ..AsThisRunIsAsked::default()
            },
        );
        assert!(
            matches!(
                first.came_to,
                WhereTheSetUpGot::ThePlacement {
                    reached: WhatThePlacementReached::Deferred { .. }
                }
            ),
            "the first press was not deferred: {:?}",
            first.came_to
        );
        assert!(first.composed.is_empty(), "a run was opened behind a deferral");
        let deferred_under = first
            .given
            .as_ref()
            .expect("the first placement was handed its values")
            .attempt
            .clone();

        // **The job ends**, and the same row presses *Set up this server*
        // again.
        work.lock().expect("the work").clear();
        let second = the_order_is_driven_as(
            &scratch,
            sftp_server,
            a_machine_that_answers(),
            an_ubuntu_systemd_machine(),
            WhatThePlacementReached::Verified,
            None,
            AsThisRunIsAsked {
                the_row: Some(first.row.clone()),
                the_work_the_commit_reads: Some(Arc::clone(&work)),
                ..AsThisRunIsAsked::default()
            },
        );
        assert!(
            matches!(
                second.came_to,
                WhereTheSetUpGot::TheRunEnded {
                    ending: TheRunsEnding::ItReachedDone { .. },
                    ..
                }
            ),
            "the second press did not set the server up: {:?}",
            second.came_to
        );
        assert_eq!(
            second.given.as_ref().map(|given| given.attempt.clone()),
            Some(deferred_under),
            "the second press was not sent under the attempt the deferral recorded"
        );
    }

    // ─────────────────────────── `Server Setup` W3's first row, judged

    /// **Whether the ready action, composed now against this machine, carries
    /// driver work or a restart** — the half of `Server Setup` W3's first row
    /// the machine answers. Asked over the far end the order itself runs on.
    fn the_ready_action_is_judged(
        scratch: &Scratch,
        sftp_server: PathBuf,
        script: Vec<(String, CommandResult)>,
        said: TheMachineBeforeAnythingIsTouched,
    ) -> (bool, Vec<String>) {
        let asked = Arc::new(Mutex::new(Vec::new()));
        let mut far_end = TheFarEnd {
            sftp_server,
            at: scratch.at.clone(),
            said: Arc::new(Mutex::new(script)),
            asked: Arc::clone(&asked),
            supervisor: Arc::new(Mutex::new(TheSupervisorHere::default())),
            children: Arc::new(Mutex::new(Vec::new())),
            goes_down: Arc::new(tokio::sync::Notify::new()),
            the_mark_when_the_restart_arrived: None,
            the_state_when_a_command_arrived: Arc::new(Mutex::new(Vec::new())),
            the_restart_takes_the_connection: false,
            written_when_the_restart_arrived: Arc::new(Mutex::new(None)),
            slow: None,
            drivers: None,
        };
        let mut taking = TheOpeningReadingsHere {
            said,
            asked_after: Arc::clone(&asked),
            asked_at: Arc::new(Mutex::new(None)),
        };
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("a runtime for the judgement");
        let point = ThisServersSessions::<TheFarEnd>::for_one_server();
        let asking = point.clone();
        let no_additional: TheAdditionalConnection<TheFarEnd> =
            Arc::new(|| Box::pin(async { None }));
        let judged = runtime.block_on(async {
            let lending = lend_until_it_ends(&point, &mut far_end, &no_additional);
            let judging = smoozy_desktop::setting_up::the_ready_action_carries_a_restart(
                &asking,
                &mut taking,
            );
            tokio::pin!(lending, judging);
            tokio::select! {
                biased;
                _ended = &mut lending => unreachable!("the far end never ends on its own"),
                judged = &mut judging => judged,
            }
        });
        let the_asks = asked.lock().expect("the asks").clone();
        (judged, the_asks)
    }

    /// **`Server Setup` W3's first row: the ready action warns where it carries
    /// driver work or a restart, and `Server Setup` R1 item 5 decides which
    /// runs those are — not a judgement here.** A card with no driver answering
    /// is installed on and restarted, and one whose driver is installed and not
    /// loaded is restarted; a card whose driver answers (the step composed and
    /// doing nothing), a machine with no card, and a machine the opening
    /// readings refuse carry neither. **Nothing machine-changing is asked on the way**: the judgement
    /// reads what the order reads before its first change. Added 2026-09-27,
    /// the eighth-pass fix for W3's warning.
    #[test]
    fn the_ready_action_carries_a_restart_only_where_its_run_would_compose_one() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("w3-judged");
        let (restarts, asked) = the_ready_action_is_judged(
            &scratch,
            sftp_server.clone(),
            a_machine_with_a_card_and_no_driver(),
            an_ubuntu_systemd_machine(),
        );
        assert!(restarts, "a card with no driver answering composed no driver step");
        assert!(
            !asked.iter().any(|said| said.contains("apt-get") || said.contains("reboot")),
            "the judgement changed the machine: {asked:?}"
        );
        let (restarts, _) = the_ready_action_is_judged(
            &scratch,
            sftp_server.clone(),
            a_machine_whose_driver_is_installed_and_not_loaded(),
            an_ubuntu_systemd_machine(),
        );
        assert!(restarts, "a driver installed and not loaded is restarted, and that was missed");
        for (script, what) in [
            (a_machine_whose_driver_answers(), "a driver that answers"),
            (a_machine_that_answers(), "no card"),
        ] {
            let (restarts, _) = the_ready_action_is_judged(
                &scratch,
                sftp_server.clone(),
                script,
                an_ubuntu_systemd_machine(),
            );
            assert!(!restarts, "{what} was judged to restart");
        }
        let (restarts, asked) = the_ready_action_is_judged(
            &scratch,
            sftp_server,
            a_machine_with_a_card_and_no_driver(),
            a_machine_the_table_does_not_place(),
        );
        assert!(!restarts, "a machine the order refuses was judged to restart");
        assert!(asked.is_empty(), "a refused machine was asked something: {asked:?}");
    }

    // ────────────────────────────────────── the judgements, driven with no machine

    /// **`Server Setup` M1a: an absent answer is not an answer, and is never
    /// read as Ubuntu.**
    #[test]
    fn a_machine_that_will_not_name_its_family_is_refused() {
        let _deadline = TheTestsDeadline::started();
        let refused = what_the_opening_readings_settle(
            TheMachineBeforeAnythingIsTouched::as_answered(
                MachineKind::ASystemdMachine,
                None,
                None,
            ),
        );
        assert_eq!(refused, Err(WhichReadingRefused::TheFamily));

        let named = what_the_opening_readings_settle(
            TheMachineBeforeAnythingIsTouched::as_answered(
                MachineKind::ASystemdMachine,
                Some("ID=ubuntu\n".to_string()),
                None,
            ),
        )
        .expect("a supported family reads");
        assert_eq!(named.family, DistributionFamily::Ubuntu);

        // **A family that is not one of the pair is refused too**, and
        // `ID_LIKE` never admits one.
        let not_the_pair = what_the_opening_readings_settle(
            TheMachineBeforeAnythingIsTouched::as_answered(
                MachineKind::ASystemdMachine,
                Some("ID=fedora\nID_LIKE=debian\n".to_string()),
                None,
            ),
        );
        assert_eq!(not_the_pair, Err(WhichReadingRefused::TheFamily));
    }

    /// **`Server Setup` R6's exclusions decide what a location is rather than
    /// what it is named.**
    #[test]
    fn a_location_r6_excludes_is_never_a_candidate() {
        let _deadline = TheTestsDeadline::started();
        let excluded = the_layout_this_machine_gets(&[AWritableLocation {
            path: "/dev/shm".to_string(),
            what_it_is: WhatTheLocationIs::AMemoryBackedFilesystem,
            the_reading: Some(smoozy_server_protocol::free_space::FreeSpaceReading {
                available_bytes: 1 << 40,
                the_path_it_was_taken_against: "/dev/shm".to_string(),
                route: smoozy_server_protocol::free_space::WhichRoute::TheStatvfsExtension,
            }),
        }]);
        assert!(
            matches!(excluded, Err(WhereTheSetUpGot::NoCandidateCouldBeOffered { .. })),
            "a memory-backed filesystem was taken as a candidate"
        );

        // **Failing to supply is a fault, and it is a different fault.**
        let nothing = the_layout_this_machine_gets(&[]);
        assert!(matches!(
            nothing,
            Err(WhereTheSetUpGot::NoCandidateCouldBeOffered { .. })
        ));
    }

    /// **The stand-in machine's login area has to be somewhere a root may sit,
    /// and that is a property of this file rather than of the desk it runs
    /// on.**
    ///
    /// The three driven runs above stand `sftp-server` in [`Scratch`], so that
    /// folder is the one writable location `Server Setup` R6 offers and
    /// `Job files and outputs` L1 chooses the root under. **When it sat in the
    /// system's own temporary folder, all three passed on a Mac and all three
    /// failed on Linux** — hosted run 35508009875 — because on Linux that
    /// folder is `/tmp` and L3 refuses a root under a shared temporary area.
    /// **Both halves are asserted here**, so the refusal is measured at the
    /// desk on a machine that cannot otherwise produce it, and so a folder
    /// moved back would fail on one test rather than on three driven orders.
    #[test]
    fn the_throwaway_folder_is_somewhere_a_root_may_sit_and_a_shared_temporary_area_is_not() {
        let _deadline = TheTestsDeadline::started();
        fn offered(path: &str) -> AWritableLocation {
            AWritableLocation {
                path: path.to_string(),
                what_it_is: WhatTheLocationIs::TheMountTheLoginsHomeAreaSitsOn,
                the_reading: Some(smoozy_server_protocol::free_space::FreeSpaceReading {
                    available_bytes: 1 << 40,
                    the_path_it_was_taken_against: path.to_string(),
                    route: smoozy_server_protocol::free_space::WhichRoute::TheStatvfsExtension,
                }),
            }
        }

        let scratch = Scratch::new("where-a-root-may-sit");
        let chosen = the_layout_this_machine_gets(&[offered(&scratch.text())])
            .expect("the throwaway folder is somewhere a root may sit");
        assert_eq!(
            chosen.root().as_path(),
            format!("{}/smoozy", scratch.text())
        );

        // **What a Linux machine would have offered from the system's own
        // temporary folder**, composed here rather than run there.
        let on_linux = the_layout_this_machine_gets(&[offered("/tmp/smoozy-p9-whole-abcdef")]);
        assert!(
            matches!(on_linux, Err(WhereTheSetUpGot::NoRootCouldBeChosen { .. })),
            "a root under a shared temporary area was chosen: {on_linux:?}"
        );
    }

    /// **`Server protocol and transfers` B9: a release the root did not sign is
    /// refused, and nothing is downloaded.**
    #[test]
    fn a_release_signed_by_another_root_is_refused_and_nothing_is_fetched() {
        let _deadline = TheTestsDeadline::started();
        let release = a_release();
        let another = test_only_keys::a_key_pair_from_a_written_down_seed(*b"some other root, not this build!");
        let mut hosting = WhatHostingWouldServe {
            table: vec![
                (
                    WHERE_THE_CURRENT_RELEASE_IS_READ_FROM.to_string(),
                    release.offered.canonical_bytes(),
                ),
                (THE_ARTIFACTS_SOURCE.to_string(), release.artifact.clone()),
            ],
            asked: Vec::new(),
        };
        let mut store = TheTrustRecordHere::default();
        let refused = the_current_release(
            &another.public,
            &mut hosting,
            &mut store,
            &composing::the_tuple_this_build_is_made_for(),
            WHERE_THE_CURRENT_RELEASE_IS_READ_FROM,
        );
        assert!(refused.is_err(), "a release another root signed was admitted");
        assert_eq!(
            hosting
                .asked
                .iter()
                .map(|(source, _)| source.as_str())
                .collect::<Vec<_>>(),
            vec![WHERE_THE_CURRENT_RELEASE_IS_READ_FROM],
            "the artifact was fetched for a release that was refused"
        );
        assert!(
            store.held.is_none(),
            "a refused release moved the trust record"
        );
    }

    /// **`Server protocol and transfers` B10: the artifact comes from an exact
    /// source the verified manifest named, and its bytes are held to the
    /// manifest's length and digest.**
    #[test]
    fn an_artifact_whose_bytes_are_not_the_manifests_is_refused() {
        let _deadline = TheTestsDeadline::started();
        let release = a_release();
        let mut hosting = WhatHostingWouldServe {
            table: vec![
                (
                    WHERE_THE_CURRENT_RELEASE_IS_READ_FROM.to_string(),
                    release.offered.canonical_bytes(),
                ),
                (
                    THE_ARTIFACTS_SOURCE.to_string(),
                    b"not the machine's own program at al".to_vec(),
                ),
            ],
            asked: Vec::new(),
        };
        let mut store = TheTrustRecordHere::default();
        let refused = the_current_release(
            &release.root,
            &mut hosting,
            &mut store,
            &composing::the_tuple_this_build_is_made_for(),
            WHERE_THE_CURRENT_RELEASE_IS_READ_FROM,
        );
        assert!(refused.is_err(), "bytes that are not the artifact were taken");
        // **The trust record moved all the same**, which is the order
        // `Server protocol and transfers` B9's
        // non-rollback rule needs: the manifest verified and was remembered
        // before anything was downloaded.
        assert!(store.held.is_some());
    }

    // ═══════════════════════ The first pass's walls, 2026-09-21: elevation

    /// `Server Setup` M3's probe, as the machine answers it.
    const THE_ELEVATION_PROBE: &str = "sudo -n true";
    /// `Server Setup` R1 item 3's install, in each of its two elevated forms.
    const THE_TOOL_INSTALL_WITHOUT_A_PASSWORD: &str =
        "sudo -n apt-get -o DPkg::Lock::Timeout=120 install -y xz-utils";
    const THE_TOOL_INSTALL_WITH_A_PASSWORD: &str =
        "sudo -S apt-get -o DPkg::Lock::Timeout=120 install -y xz-utils";
    /// **`Server Setup` S7d's refresh in front of it**, in the password form.
    /// Added 2026-09-22.
    const THE_LIST_REFRESH_WITH_A_PASSWORD: &str =
        "sudo -S apt-get -o DPkg::Lock::Timeout=120 update";
    /// `Server Setup` E4's validate form.
    const THE_VALIDATE: &str = "sudo -S -v";

    fn with_the_probe(answered: CommandResult) -> Vec<(String, CommandResult)> {
        let mut script = a_machine_that_answers();
        script.push((THE_ELEVATION_PROBE.to_string(), answered));
        script
    }

    /// **`Server Setup` M3: where the login may elevate with no password,
    /// elevated steps run — and nobody is asked for a password that does not
    /// exist.**
    ///
    /// **The run starts owing a password and holding none**, which is how the
    /// running application starts every run, and **nobody answers an offer**,
    /// so a run that made one would end there. The first pass on a real
    /// rental found every run doing exactly that, on a login where
    /// `sudo -n true` ended at 0.
    #[test]
    fn a_login_that_needs_no_password_has_its_steps_run_without_one() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("m3-probe-succeeds");
        let left = the_order_is_driven_as(
            &scratch,
            sftp_server,
            with_the_probe(ended_cleanly("")),
            an_ubuntu_systemd_machine(),
            WhatThePlacementReached::Verified,
            None,
            AsThisRunIsAsked {
                elevation: TheElevationHere::OwedAndNobodyAnswers,
                ..AsThisRunIsAsked::default()
            },
        );
        assert!(
            matches!(
                left.came_to,
                WhereTheSetUpGot::TheRunEnded {
                    at: smoozy_server_setup::run::Boundary::Done,
                    ending: TheRunsEnding::ItReachedDone { .. },
                }
            ),
            "the run did not reach done: {:?}",
            left.came_to
        );
        assert!(
            left.asked.iter().any(|said| said == THE_ELEVATION_PROBE),
            "the probe was never taken: {:?}",
            left.asked
        );
        assert!(
            left.asked.iter().any(|said| said == THE_TOOL_INSTALL_WITHOUT_A_PASSWORD),
            "the unpack tool was not installed in the passwordless form: {:?}",
            left.asked
        );
        assert!(left.the_offers_made.is_empty(), "an offer was made: {:?}", left.the_offers_made);
        assert!(
            left.carried_on_standard_input.is_empty(),
            "something travelled on a standard input with no password owed"
        );
        // **The probe was taken after `Server Setup` M1 and `Server Setup` M1a and changes
        // nothing**, so it sits after the opening readings.
        let probe_at = left
            .asked
            .iter()
            .position(|said| said == THE_ELEVATION_PROBE)
            .expect("the probe");
        assert!(left.the_readings_were_asked_at.expect("the readings") <= probe_at);
    }

    /// **`Server Setup` E2 and E4: where a password is owed, the offer is made
    /// at the step that needs it, names that step, and the step is taken again
    /// with the password — carried on standard input, validated first.**
    #[test]
    fn where_a_password_is_owed_the_offer_names_the_step_and_the_step_is_taken_again_with_it() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("e2-offer-answered");
        const A_MADE_UP_PASSWORD: &str = "made-up-for-a-test";
        let left = the_order_is_driven_as(
            &scratch,
            sftp_server,
            with_the_probe(it_refused()),
            an_ubuntu_systemd_machine(),
            WhatThePlacementReached::Verified,
            None,
            AsThisRunIsAsked {
                elevation: TheElevationHere::OwedAndTheOfferIsAnsweredWith(A_MADE_UP_PASSWORD),
                ..AsThisRunIsAsked::default()
            },
        );
        assert!(
            matches!(
                left.came_to,
                WhereTheSetUpGot::TheRunEnded {
                    at: smoozy_server_setup::run::Boundary::Done,
                    ending: TheRunsEnding::ItReachedDone { .. },
                }
            ),
            "the run did not reach done: {:?}",
            left.came_to
        );
        // **One offer, at the first step that needed it, naming it.**
        assert_eq!(
            left.the_offers_made,
            vec![smoozy_server_setup::ready_action::Step::Tooling]
        );
        // **`Server Setup` E3 and E4: the validate first, then the operation,
        // each carrying the password on standard input and never in the
        // command.**
        let carried: Vec<&str> = left
            .carried_on_standard_input
            .iter()
            .map(|(command, _)| command.as_str())
            .collect();
        // **`Server Setup` S7d, from 2026-09-22: the lists are refreshed in
        // front of the install, as an operation of its own under the same
        // validate-first order.**
        assert_eq!(
            carried,
            vec![
                THE_VALIDATE,
                THE_LIST_REFRESH_WITH_A_PASSWORD,
                THE_VALIDATE,
                THE_TOOL_INSTALL_WITH_A_PASSWORD
            ]
        );
        for (command, value) in &left.carried_on_standard_input {
            assert_eq!(value.as_slice(), A_MADE_UP_PASSWORD.as_bytes());
            assert!(!command.contains(A_MADE_UP_PASSWORD), "the password rode a command");
        }
        assert!(
            !left.asked.iter().any(|said| said == THE_TOOL_INSTALL_WITHOUT_A_PASSWORD),
            "the passwordless form went out on a login the probe said needs one"
        );
        // **Nothing was set for the refused attempt**: the state is set once,
        // by the run, before the install that went out, and cleared at done.
        assert_eq!(left.set_by.len(), 1, "the state was set {:?}", left.set_by);
        assert_eq!(left.state_at_the_end, None);
    }

    /// **`Server Setup` E4's fourth bullet: where the offer is declined,
    /// nothing elevated goes out, no machine-level state is set for it, and
    /// the run is stopped at its boundary rather than left open.**
    #[test]
    fn where_the_offer_is_declined_nothing_elevated_goes_out_and_the_run_is_stopped() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("e2-offer-declined");
        let left = the_order_is_driven_as(
            &scratch,
            sftp_server,
            with_the_probe(it_refused()),
            an_ubuntu_systemd_machine(),
            WhatThePlacementReached::Verified,
            None,
            AsThisRunIsAsked {
                elevation: TheElevationHere::OwedAndNobodyAnswers,
                ..AsThisRunIsAsked::default()
            },
        );
        match &left.came_to {
            WhereTheSetUpGot::TheRunEnded {
                ending: TheRunsEnding::AStepNeedsAdminAccess { step },
                ..
            } => assert_eq!(*step, smoozy_server_setup::ready_action::Step::Tooling),
            other => panic!("the run did not end at the offer: {other:?}"),
        }
        assert!(
            !left.asked.iter().any(|said| said.contains("apt-get")),
            "an install went out with no password: {:?}",
            left.asked
        );
        assert!(left.set_by.is_empty(), "a state was set for an act never sent");
        assert!(
            left.the_stop_was_written_for.is_some(),
            "the run was left open for the machine's own window to end"
        );
        // **And the screen's sentence names the step and says what happened,
        // with nothing of a debugging rendering in it.**
        let said = smoozy_desktop::surface::set_up::what_it_came_to(&left.came_to).said;
        assert!(said.contains("Installing the unpack tool"), "{said}");
        assert!(said.contains("admin access"), "{said}");
        assert!(!said.contains("NoPasswordIsHeldYet") && !said.contains('{'), "{said}");
    }

    // ═════════════════ The first pass's walls, 2026-09-21: a Blender install

    struct WhatTheInstallLeft {
        came_to: WhereTheSetUpGot,
        kinds: Vec<Option<String>>,
        steps: Vec<String>,
        versions: Vec<Option<String>>,
        composed: Vec<String>,
        asked: Vec<String>,
        /// **What each reading of a step running across readings named**
        /// (piece 2c, 2026-10-25).
        readings_named: Vec<(Option<String>, Option<String>, Option<String>, bool, std::time::Instant)>,
    }

    fn a_blender_install_is_driven_here(
        scratch: &Scratch,
        sftp_server: PathBuf,
        script: Vec<(String, CommandResult)>,
        row: ServerRow,
    ) -> WhatTheInstallLeft {
        a_blender_install_is_driven_counting(scratch, sftp_server, script, row, Vec::new(), &mut NobodyIsThereToAsk)
    }

    /// **The same, with the machine answering `Server Setup` Y7b's and `Server Setup` Y7c's
    /// counts on its state reading and a screen of the test's own behind the
    /// run.** Added 2026-10-18, the Creator's fourteenth notes, piece U.
    fn a_blender_install_is_driven_counting(
        scratch: &Scratch,
        sftp_server: PathBuf,
        script: Vec<(String, CommandResult)>,
        row: ServerRow,
        the_counts: Vec<(String, u64, Option<u64>)>,
        screen: &mut dyn AsksForThePassword,
    ) -> WhatTheInstallLeft {
        a_blender_install_is_driven_on(
            scratch,
            sftp_server,
            script,
            row,
            an_ubuntu_systemd_machine(),
            TheSupervisorHere {
                the_counts,
                ..TheSupervisorHere::default()
            },
            screen,
        )
    }

    /// **The same, on a machine of the test's own kind, against a supervisor
    /// the test sets** — `Server Setup` Y7a's words on its state reading among
    /// what it may hold. Added 2026-10-24, the sixteenth notes, note 1.
    fn a_blender_install_is_driven_on(
        scratch: &Scratch,
        sftp_server: PathBuf,
        script: Vec<(String, CommandResult)>,
        row: ServerRow,
        said: TheMachineBeforeAnythingIsTouched,
        supervisor: TheSupervisorHere,
        screen: &mut dyn AsksForThePassword,
    ) -> WhatTheInstallLeft {
        let asked = Arc::new(Mutex::new(Vec::new()));
        let supervisor = Arc::new(Mutex::new(supervisor));
        let mut far_end = TheFarEnd {
            sftp_server,
            at: scratch.at.clone(),
            said: Arc::new(Mutex::new(script)),
            asked: Arc::clone(&asked),
            supervisor: Arc::clone(&supervisor),
            children: Arc::new(Mutex::new(Vec::new())),
            goes_down: Arc::new(tokio::sync::Notify::new()),
            the_mark_when_the_restart_arrived: None,
            the_state_when_a_command_arrived: Arc::new(Mutex::new(Vec::new())),
            the_restart_takes_the_connection: false,
            written_when_the_restart_arrived: Arc::new(Mutex::new(None)),
            slow: None,
            drivers: None,
        };
        let mut row = TheRowHere {
            row,
            writes: Arc::new(Mutex::new(Vec::new())),
        };
        let mut taking = TheOpeningReadingsHere {
            said,
            asked_after: Arc::clone(&asked),
            asked_at: Arc::new(Mutex::new(None)),
        };
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("a runtime for the run");
        let point = ThisServersSessions::<TheFarEnd>::for_one_server();
        let asking = point.clone();
        let no_additional: TheAdditionalConnection<TheFarEnd> =
            Arc::new(|| Box::pin(async { None }));
        let came_to = runtime.block_on(async {
            let lending = lend_until_it_ends(&point, &mut far_end, &no_additional);
            let driven = a_blender_is_installed(
                &asking,
                &mut taking,
                &mut row,
                "5.2.1".to_string(),
                smoozy_server_setup::elevated::HowElevationIsObtained::Passwordless,
                screen,
                &NobodyAsked,
                None,
            );
            tokio::pin!(lending, driven);
            tokio::select! {
                biased;
                _ended = &mut lending => unreachable!("the far end never ends on its own"),
                came_to = &mut driven => came_to,
            }
        });
        let the_asks = asked.lock().expect("the asks").clone();
        let held = supervisor.lock().expect("the supervisor");
        WhatTheInstallLeft {
            came_to,
            kinds: held.kinds.clone(),
            steps: held.steps.clone(),
            versions: held.versions.clone(),
            composed: held.composed.clone(),
            asked: the_asks,
            readings_named: held.readings_named.clone(),
        }
    }

    struct WhatTheRemovalLeft {
        came_to: WhereTheSetUpGot,
        asked: Vec<String>,
        steps: Vec<String>,
        marks: Vec<(String, Option<String>)>,
        packages: Vec<(String, Option<String>)>,
        /// **`Server Setup` §12's act lines the run handed the diary.** Step
        /// 10c's owed piece 12.
        diary: Vec<smoozy_server_setup::diary::ActLine>,
    }

    /// **A removal run, driven at the desk** against the stand-in machine,
    /// whose command channel is the stand-in manager: every command it is
    /// asked is kept, in order. Step 10c's owed piece 6.
    fn a_removal_is_driven_here(
        scratch: &Scratch,
        sftp_server: PathBuf,
        script: Vec<(String, CommandResult)>,
        removal: TheRemovalAsked,
    ) -> WhatTheRemovalLeft {
        let asked = Arc::new(Mutex::new(Vec::new()));
        let supervisor = Arc::new(Mutex::new(TheSupervisorHere::default()));
        let mut far_end = TheFarEnd {
            sftp_server,
            at: scratch.at.clone(),
            said: Arc::new(Mutex::new(script)),
            asked: Arc::clone(&asked),
            supervisor: Arc::clone(&supervisor),
            children: Arc::new(Mutex::new(Vec::new())),
            goes_down: Arc::new(tokio::sync::Notify::new()),
            the_mark_when_the_restart_arrived: None,
            the_state_when_a_command_arrived: Arc::new(Mutex::new(Vec::new())),
            the_restart_takes_the_connection: false,
            written_when_the_restart_arrived: Arc::new(Mutex::new(None)),
            slow: None,
            drivers: None,
        };
        let mut row = TheRowHere {
            row: a_row_with_its_root_at(scratch),
            writes: Arc::new(Mutex::new(Vec::new())),
        };
        let mut taking = TheOpeningReadingsHere {
            said: an_ubuntu_systemd_machine(),
            asked_after: Arc::clone(&asked),
            asked_at: Arc::new(Mutex::new(None)),
        };
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("a runtime for the run");
        let point = ThisServersSessions::<TheFarEnd>::for_one_server();
        let asking = point.clone();
        let no_additional: TheAdditionalConnection<TheFarEnd> =
            Arc::new(|| Box::pin(async { None }));
        let mut nobody = NobodyIsThereToAsk;
        let mut diary = Vec::new();
        let mut to_the_diary = |line| diary.push(line);
        let came_to = runtime.block_on(async {
            let lending = lend_until_it_ends(&point, &mut far_end, &no_additional);
            let driven = a_blender_is_removed(
                &asking,
                &mut taking,
                &mut row,
                removal,
                &mut nobody,
                &NobodyAsked,
                &mut to_the_diary,
            );
            tokio::pin!(lending, driven);
            tokio::select! {
                biased;
                _ended = &mut lending => unreachable!("the far end never ends on its own"),
                came_to = &mut driven => came_to,
            }
        });
        let the_asks = asked.lock().expect("the asks").clone();
        let held = supervisor.lock().expect("the supervisor");
        WhatTheRemovalLeft {
            came_to,
            asked: the_asks,
            steps: held.steps.clone(),
            marks: held.marks_on_steps.clone(),
            packages: held.packages_on_steps.clone(),
            diary,
        }
    }

    /// **Step 10c's owed piece 12 — `Server Setup` §12's *a Blender was taken
    /// off*, written where a removal ends removed, on both of `Server Setup`
    /// B15's routes.** The plain deletion of a build under Smoozy's own Blender
    /// area hands the diary one line naming its version and *the folder
    /// deletion*; the snap's removal through the stand-in manager hands one
    /// naming *the machine's own software manager*. **A removal refused at
    /// its step hands nothing.** Undone — the line left unwritten on either
    /// route, written for a failed removal, or the routes swapped — this
    /// fails.
    #[test]
    fn a_removal_that_ends_removed_writes_one_taken_off_line_on_either_route() {
        use smoozy_server_setup::diary::ActLine;
        use smoozy_server_setup::search::RemovalRoute;
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let it_reached_done = |came_to: &WhereTheSetUpGot| {
            matches!(
                came_to,
                WhereTheSetUpGot::TheRunEnded {
                    at: smoozy_server_setup::run::Boundary::Done,
                    ending: TheRunsEnding::ItReachedDone { .. },
                }
            )
        };

        let scratch = Scratch::new("s12-the-folder-deletion");
        let in_the_area = format!("{}/blender/blender-4.5.9-linux-x64/blender", scratch.at.display());
        let left = a_removal_is_driven_here(
            &scratch,
            sftp_server.clone(),
            Vec::new(),
            TheRemovalAsked {
                executable: in_the_area,
                the_answered_version: "4.5.9".to_string(),
                covered: Vec::new(),
                the_managers_package: None,
            },
        );
        assert!(it_reached_done(&left.came_to), "the deletion came to {:?}", left.came_to);
        assert_eq!(
            left.diary,
            vec![ActLine::ABlenderWasTakenOff {
                answered: smoozy_server_setup::blender::Version::parse("4.5.9").expect("a version"),
                route: RemovalRoute::TheFolderDeletion,
            }]
        );

        let the_snap = |package: &'static str| TheRemovalAsked {
            executable: "/snap/bin/blender".to_string(),
            the_answered_version: "5.2.0".to_string(),
            covered: Vec::new(),
            the_managers_package: Some(package),
        };
        let mut script = with_the_probe(ended_cleanly(""));
        script.push(("sudo -n snap remove blender".to_string(), ended_cleanly("blender removed")));
        let scratch = Scratch::new("s12-the-managers-removal");
        let left = a_removal_is_driven_here(&scratch, sftp_server.clone(), script.clone(), the_snap("blender"));
        assert!(it_reached_done(&left.came_to), "the manager's removal came to {:?}", left.came_to);
        assert_eq!(
            left.diary,
            vec![ActLine::ABlenderWasTakenOff {
                answered: smoozy_server_setup::blender::Version::parse("5.2.0").expect("a version"),
                route: RemovalRoute::TheMachinesOwnSoftwareManager,
            }]
        );

        let scratch = Scratch::new("s12-a-refused-removal");
        let refused = a_removal_is_driven_here(&scratch, sftp_server, script, the_snap("blender-bin"));
        assert!(!it_reached_done(&refused.came_to), "{:?}", refused.came_to);
        assert!(refused.diary.is_empty(), "a refused removal wrote {:?}", refused.diary);
    }

    /// **Step 10c's owed piece 6 — `Server Setup` B15's second route, driven
    /// at the desk: the machine's own software manager is asked to remove
    /// exactly the one package, and nothing else.** The stand-in manager is
    /// the stand-in machine's command channel, which keeps every command it is
    /// asked. **On a login that needs no password** (`Server Setup` M3's probe)
    /// the removal goes out once, as the pairing's own constant, and the
    /// machine's removal step then carries the package and the mark
    /// *performed*, and the run reaches *done*. **For any other package name
    /// nothing goes to the manager**, no removal step is commanded, and the
    /// run ends failed at the removal. Undone — the snap row left on the plain
    /// deletion, or the package not checked against the pairing — this fails.
    #[test]
    fn a_managed_blender_is_removed_through_its_own_manager_with_that_package_alone() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let the_removal = |package: &'static str| TheRemovalAsked {
            executable: "/snap/bin/blender".to_string(),
            the_answered_version: "5.2.0".to_string(),
            covered: Vec::new(),
            the_managers_package: Some(package),
        };
        let mut script = with_the_probe(ended_cleanly(""));
        script.push(("sudo -n snap remove blender".to_string(), ended_cleanly("blender removed")));

        let scratch = Scratch::new("b15-the-managers-removal");
        let left = a_removal_is_driven_here(&scratch, sftp_server.clone(), script.clone(), the_removal("blender"));
        assert!(
            matches!(
                &left.came_to,
                WhereTheSetUpGot::TheRunEnded {
                    at: smoozy_server_setup::run::Boundary::Done,
                    ending: TheRunsEnding::ItReachedDone { .. },
                }
            ),
            "the removal came to {:?}",
            left.came_to
        );
        let to_the_manager: Vec<&String> =
            left.asked.iter().filter(|said| said.contains("snap") || said.contains("remove")).collect();
        assert_eq!(to_the_manager, vec!["sudo -n snap remove blender"], "{:?}", left.asked);
        assert!(left.steps.contains(&asked::THE_BUILD_IS_REMOVED_STEP.to_string()), "{:?}", left.steps);
        assert!(left.packages.contains(&(
            asked::THE_BUILD_IS_REMOVED_STEP.to_string(),
            Some("blender".to_string())
        )));
        assert!(left.marks.contains(&(
            asked::THE_BUILD_IS_REMOVED_STEP.to_string(),
            Some(asked::IT_PERFORMED.to_string())
        )));

        let scratch = Scratch::new("b15-another-package");
        let refused = a_removal_is_driven_here(&scratch, sftp_server, script, the_removal("blender-bin"));
        assert!(
            refused.asked.iter().all(|said| !said.contains("snap") && !said.contains("remove")),
            "a command went to the manager for another package: {:?}",
            refused.asked
        );
        assert!(!refused.steps.contains(&asked::THE_BUILD_IS_REMOVED_STEP.to_string()));
        match &refused.came_to {
            WhereTheSetUpGot::TheRunEnded {
                ending: TheRunsEnding::AStepFailed { step, said },
                ..
            } => {
                assert_eq!(*step, smoozy_server_setup::ready_action::Step::BlenderRemoval);
                assert!(said.contains("blender-bin"), "{said}");
            }
            other => panic!("another package's removal came to {other:?}"),
        }
    }

    /// **The 38's piece 14b: the upload route, stood in for** — the archive is
    /// copied to where the route would land it, and where it was sent is kept.
    struct TheUploadHere {
        archive: PathBuf,
        sent: Arc<Mutex<Vec<(String, String)>>>,
    }

    impl SendsTheUsersArchiveUp for TheUploadHere {
        fn the_archive_goes_up(
            &mut self,
            staging_area: String,
            landing: String,
        ) -> smoozy_desktop::servers::sessions::ItsOwnFuture<'_, Result<(), String>> {
            self.sent.lock().expect("the sends").push((staging_area.clone(), landing.clone()));
            let went = std::fs::copy(&self.archive, std::path::Path::new(&staging_area).join(&landing))
                .map(|_| ())
                .map_err(|why| why.to_string());
            Box::pin(async move { went })
        }
    }

    /// **The card's question, answered here**: every version asked about is
    /// kept, and the answer is the one the test set.
    struct TheQuestionAnsweredHere {
        install_anyway: bool,
        asked_about: Arc<Mutex<Vec<String>>>,
    }

    impl AsksForThePassword for TheQuestionAnsweredHere {
        fn the_password_for(
            &mut self,
            _step: smoozy_server_setup::ready_action::Step,
        ) -> smoozy_desktop::servers::sessions::ItsOwnFuture<'_, Option<smoozy_security_and_licence::plaintext::Origin>> {
            Box::pin(async { None })
        }

        fn the_answer_below_the_floor(
            &mut self,
            version: &str,
        ) -> smoozy_desktop::servers::sessions::ItsOwnFuture<'_, bool> {
            self.asked_about.lock().expect("the questions").push(version.to_string());
            let answer = self.install_anyway;
            Box::pin(async move { answer })
        }
    }

    struct WhatTheUsersBuildLeft {
        came_to: WhereTheSetUpGot,
        steps: Vec<String>,
        versions: Vec<Option<String>>,
        archives: Vec<(String, Option<String>)>,
        answers: Vec<String>,
        run: Option<String>,
        staging_area: String,
        sent: Vec<(String, String)>,
        asked_about: Vec<String>,
        /// **`Server Setup` §12's act lines the run handed the diary.** Step
        /// 10c's owed piece 5.
        diary: Vec<smoozy_server_setup::diary::ActLine>,
    }

    /// **The pick's run, driven at the desk** — `Server Setup` B7 with a small
    /// fake archive, the machine holding below 4.5 where `holds_at` says.
    fn the_users_build_is_driven_here(
        scratch: &Scratch,
        sftp_server: PathBuf,
        holds_at: Option<&str>,
        install_anyway: bool,
    ) -> WhatTheUsersBuildLeft {
        the_users_build_is_driven_with_an_upload_here(scratch, sftp_server, holds_at, install_anyway, false)
    }

    fn the_users_build_is_driven_with_an_upload_here(
        scratch: &Scratch,
        sftp_server: PathBuf,
        holds_at: Option<&str>,
        install_anyway: bool,
        archive_is_missing: bool,
    ) -> WhatTheUsersBuildLeft {
        let archive = scratch.at("blender-test.tar.xz");
        if !archive_is_missing {
            std::fs::write(&archive, b"a small fake archive").expect("the fake archive");
        }
        let staging_area = format!("{}/smoozy/staging", scratch.text());
        let supervisor = Arc::new(Mutex::new(TheSupervisorHere {
            staging_area: Some(staging_area.clone()),
            holds_at: holds_at.map(str::to_string),
            ..TheSupervisorHere::default()
        }));
        let asked = Arc::new(Mutex::new(Vec::new()));
        let mut far_end = TheFarEnd {
            sftp_server,
            at: scratch.at.clone(),
            said: Arc::new(Mutex::new(a_machine_with_a_card_and_no_driver())),
            asked: Arc::clone(&asked),
            supervisor: Arc::clone(&supervisor),
            children: Arc::new(Mutex::new(Vec::new())),
            goes_down: Arc::new(tokio::sync::Notify::new()),
            the_mark_when_the_restart_arrived: None,
            the_state_when_a_command_arrived: Arc::new(Mutex::new(Vec::new())),
            the_restart_takes_the_connection: false,
            written_when_the_restart_arrived: Arc::new(Mutex::new(None)),
            slow: None,
            drivers: None,
        };
        let mut row = TheRowHere {
            row: a_row_with_its_root_at(scratch),
            writes: Arc::new(Mutex::new(Vec::new())),
        };
        let mut taking = TheOpeningReadingsHere {
            said: an_ubuntu_systemd_machine(),
            asked_after: Arc::clone(&asked),
            asked_at: Arc::new(Mutex::new(None)),
        };
        let sent = Arc::new(Mutex::new(Vec::new()));
        let mut upload = TheUploadHere { archive, sent: Arc::clone(&sent) };
        let asked_about = Arc::new(Mutex::new(Vec::new()));
        let mut screen = TheQuestionAnsweredHere { install_anyway, asked_about: Arc::clone(&asked_about) };
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("a runtime for the run");
        let point = ThisServersSessions::<TheFarEnd>::for_one_server();
        let asking = point.clone();
        let no_additional: TheAdditionalConnection<TheFarEnd> = Arc::new(|| Box::pin(async { None }));
        let mut diary = Vec::new();
        let mut to_the_diary = |line| diary.push(line);
        let came_to = runtime.block_on(async {
            let lending = lend_until_it_ends(&point, &mut far_end, &no_additional);
            let driven = the_users_build_is_installed(
                &asking,
                &mut taking,
                &mut row,
                "blender-test.tar.xz".to_string(),
                &mut upload,
                smoozy_server_setup::elevated::HowElevationIsObtained::Passwordless,
                &mut screen,
                &NobodyAsked,
                None,
                &mut to_the_diary,
            );
            tokio::pin!(lending, driven);
            tokio::select! {
                biased;
                _ended = &mut lending => unreachable!("the far end never ends on its own"),
                came_to = &mut driven => came_to,
            }
        });
        let held = supervisor.lock().expect("the supervisor");
        let sent = sent.lock().expect("the sends").clone();
        let asked_about = asked_about.lock().expect("the questions").clone();
        WhatTheUsersBuildLeft {
            came_to,
            steps: held.steps.clone(),
            versions: held.versions.clone(),
            archives: held.archives.clone(),
            answers: held.answers.clone(),
            run: held.run.clone(),
            staging_area,
            sent,
            asked_about,
            diary,
        }
    }

    /// **`Server Setup` §12's *a Blender was installed*, for a build the user
    /// pointed at**, as the run hands it on.
    fn a_users_build_line(answered: &str, published: bool) -> smoozy_server_setup::diary::ActLine {
        smoozy_server_setup::diary::ActLine::ABlenderWasInstalled {
            answered: smoozy_server_setup::blender::Version::parse(answered).expect("a version"),
            route: smoozy_server_setup::blender::HowTheBuildGotHere::ABuildTheUserPointedAt,
            published,
        }
    }

    fn it_reached_done(came_to: &WhereTheSetUpGot, published: bool) -> bool {
        matches!(
            came_to,
            WhereTheSetUpGot::TheRunEnded {
                at: smoozy_server_setup::run::Boundary::Done,
                ending: TheRunsEnding::ItReachedDone { a_blender_was_published, .. },
            } if *a_blender_was_published == published
        )
    }

    /// **The 38's piece 14b: a build chosen goes up into the open run's own
    /// staging area, under `<run>-<name>`, and the run names that archive on
    /// its unpack and publication** — no fetch, no version anywhere. **Undone
    /// (the run opened after the upload, the landing name without the run,
    /// or the archive left off a step), this fails.**
    #[test]
    fn a_build_chosen_goes_up_into_the_runs_staging_and_the_run_names_it() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("b7-your-build");
        let left = the_users_build_is_driven_here(&scratch, sftp_server, None, false);
        assert!(it_reached_done(&left.came_to, true), "the run came to {:?}", left.came_to);
        let run = left.run.clone().expect("a run was opened");
        assert_eq!(
            left.sent,
            vec![(left.staging_area.clone(), format!("{run}-blender-test.tar.xz"))],
            "the archive did not go up into the run's own staging area under its name"
        );
        assert!(!left.steps.iter().any(|step| step == asked::THE_FETCH_STEP), "a fetch was commanded: {:?}", left.steps);
        for step in [asked::THE_UNPACK_STEP, asked::THE_BUILD_IS_PUBLISHED_STEP] {
            assert!(
                left.archives.contains(&(step.to_string(), Some("blender-test.tar.xz".to_string()))),
                "{step} did not name the archive: {:?}",
                left.archives
            );
        }
        assert!(left.versions.iter().all(Option::is_none), "a version was sent: {:?}", left.versions);
        assert!(left.asked_about.is_empty() && left.answers.is_empty(), "a question was asked at 4.5 or above");
        // **`Server Setup` §12: one act line, naming the version the build
        // answered** (step 10c's owed piece 5).
        assert_eq!(left.diary, vec![a_users_build_line("4.6.0", true)], "the diary was not told the build installed");
    }

    #[test]
    fn a_build_upload_that_did_not_land_has_its_own_ending_and_its_own_words() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("build-not-sent");
        let left = the_users_build_is_driven_with_an_upload_here(&scratch, sftp_server, None, false, true);
        let WhereTheSetUpGot::TheRunEnded { ending, .. } = &left.came_to else { panic!("{:?}", left.came_to) };
        assert!(matches!(ending, TheRunsEnding::TheBuildCouldNotBeSent { .. }), "the upload was blamed on a step: {ending:?}");
        assert!(left.steps.is_empty(), "an install step ran after the failed send: {:?}", left.steps);
        assert!(left.diary.is_empty());
        let came = smoozy_desktop::surface::set_up::what_the_install_came_to(&left.came_to, "blender-test.tar.xz");
        assert!(came.said.contains("Your build was not uploaded."), "{}", came.said);
        assert!(came.said.contains("No such file"), "the upload's words were lost: {}", came.said);
        assert_eq!(came.the_machines_own_words, None, "the desk's words were attributed to the server");
        let named = smoozy_desktop::surface::set_up::the_users_build_not_installed_is_named(came, &left.came_to, "blender-test.tar.xz", true);
        assert!(named.said.ends_with("Your build blender-test.tar.xz was not installed."), "{}", named.said);
    }

    #[test]
    fn the_build_upload_sender_does_not_make_a_missing_destination() {
        let scratch = Scratch::new("build-missing-destination");
        let archive = scratch.at("archive.tar.xz");
        std::fs::write(&archive, b"an archive").unwrap();
        let staging = scratch.at("never-made");
        let mut sender = TheUploadHere { archive, sent: Default::default() };
        let runtime = tokio::runtime::Builder::new_current_thread().build().unwrap();
        let sent = runtime.block_on(sender.the_archive_goes_up(staging.display().to_string(), "landing.tar.xz".to_string()));
        assert!(sent.is_err(), "the test sender hid a missing destination");
        assert!(!staging.exists());
    }

    /// **`Server Setup` B7a: below 4.5 the question is asked, and *Install
    /// anyway* reaches the machine** — the publication commanded again with
    /// the answer, and the build published. **Undone, this fails.**
    #[test]
    fn below_four_five_the_question_is_asked_and_install_anyway_reaches_the_machine() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("b7a-install-anyway");
        let left = the_users_build_is_driven_here(&scratch, sftp_server, Some("4.2.0"), true);
        assert_eq!(left.asked_about, vec!["4.2.0".to_string()]);
        assert_eq!(left.answers, vec![asked::INSTALL_ANYWAY.to_string()]);
        let publications = left.steps.iter().filter(|step| *step == asked::THE_BUILD_IS_PUBLISHED_STEP).count();
        assert_eq!(publications, 2, "the publication was not commanded again: {:?}", left.steps);
        assert!(it_reached_done(&left.came_to, true), "the run came to {:?}", left.came_to);
        assert_eq!(left.diary, vec![a_users_build_line("4.2.0", true)], "the diary was not told the build installed");
    }

    /// **`Server Setup` B7a: the decline reaches the machine too, and the run
    /// ends at *done* claiming no build.** **Undone, this fails.**
    #[test]
    fn below_four_five_the_decline_reaches_the_machine_and_claims_no_build() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("b7a-declined");
        let left = the_users_build_is_driven_here(&scratch, sftp_server, Some("4.2.0"), false);
        assert_eq!(left.asked_about, vec!["4.2.0".to_string()]);
        assert_eq!(left.answers, vec![asked::THE_USER_DECLINED.to_string()]);
        assert!(it_reached_done(&left.came_to, false), "the run came to {:?}", left.came_to);
        // **`Server Setup` §12: *declined at the version answer*** (step 10c's
        // owed piece 5).
        assert_eq!(left.diary, vec![a_users_build_line("4.2.0", false)], "the diary was not told the build was declined");
    }

    /// A row this copy has set the machine up under, with its root recorded.
    fn a_row_with_its_root_at(scratch: &Scratch) -> ServerRow {
        std::fs::create_dir_all(scratch.at("smoozy").join("package")).expect("the package area");
        ServerRow {
            recorded_machine_root: Some(RecordedMachineRoot {
                path: format!("{}/smoozy", scratch.text()),
                the_package_areas_own_location: None,
                read_under: "an installation".to_string(),
            }),
            // **A program was placed**, which the row says by the installation
            // it confirmed.
            last_confirmed_installation_identity: Some("an installation".to_string()),
            remembered_setup: None,
            ..a_row()
        }
    }

    /// **A screen that keeps every figure the driving hands it, and the build
    /// it was told the run downloads.** Piece U.
    struct TheFiguresHere {
        the_build: Arc<Mutex<Option<Option<String>>>>,
        told: Arc<Mutex<Vec<Option<smoozy_desktop::setting_up::HowFarTheStepHasGot>>>>,
    }

    impl AsksForThePassword for TheFiguresHere {
        fn the_password_for(
            &mut self,
            _step: smoozy_server_setup::ready_action::Step,
        ) -> smoozy_desktop::servers::sessions::ItsOwnFuture<
            '_,
            Option<smoozy_security_and_licence::plaintext::Origin>,
        > {
            Box::pin(async { None })
        }

        fn where_the_figure_goes(
            &mut self,
            the_build: Option<&str>,
        ) -> Option<smoozy_desktop::setting_up::TheFigureGoesHere> {
            *self.the_build.lock().expect("the build") = Some(the_build.map(str::to_string));
            let told = Arc::clone(&self.told);
            Some(Arc::new(move |figure| told.lock().expect("the figures").push(figure)))
        }
    }

    /// **`Server Setup` Y7b and `Server Setup` Y7c, driven: the count the machine's state
    /// reading carries while the download or the unpack runs is handed to the
    /// screen as that step's figure, and taken away when the step is over** —
    /// and the screen is told the build the run downloads. No other step hands
    /// it anything. The Creator's fourteenth notes, piece U.
    #[test]
    fn piece_u_the_downloads_and_the_unpacks_counts_reach_the_screen_and_go_with_their_steps() {
        use smoozy_desktop::setting_up::{HowFarTheStepHasGot, TheCountedAct, TheStepsCount};
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("piece-u-the-counts");
        let row = a_row_with_its_root_at(&scratch);
        let mut screen = TheFiguresHere {
            the_build: Arc::new(Mutex::new(None)),
            told: Arc::new(Mutex::new(Vec::new())),
        };
        let left = a_blender_install_is_driven_counting(
            &scratch,
            sftp_server,
            a_machine_with_a_card_and_no_driver(),
            row,
            vec![
                (asked::THE_FETCH_STEP.to_string(), 214_000_000, Some(352_000_000)),
                (asked::THE_UNPACK_STEP.to_string(), 144_320_000, Some(352_000_000)),
            ],
            &mut screen,
        );
        assert!(
            matches!(left.came_to, WhereTheSetUpGot::TheRunEnded { ending: TheRunsEnding::ItReachedDone { .. }, .. }),
            "the install did not reach done: {:?}",
            left.came_to
        );
        assert_eq!(
            *screen.the_build.lock().expect("the build"),
            Some(Some("5.2.1".to_string())),
            "the screen was not told the build the run downloads"
        );
        let download = HowFarTheStepHasGot {
            act: TheCountedAct::TheDownload,
            count: TheStepsCount { so_far: 214_000_000, whole: Some(352_000_000) },
        };
        let unpack = HowFarTheStepHasGot {
            act: TheCountedAct::TheUnpack,
            count: TheStepsCount { so_far: 144_320_000, whole: Some(352_000_000) },
        };
        let told = screen.told.lock().expect("the figures").clone();
        let mut without_repeats = told.clone();
        without_repeats.dedup();
        assert_eq!(
            without_repeats,
            vec![Some(download), None, Some(unpack), None],
            "the figures were not handed as the steps ran: {told:?}"
        );
    }

    // ═══════ `Server Setup` Y7a as changed — the sixteenth notes, note 1, 2026-10-24

    /// **A screen that keeps the run's words the way the Set up tab keeps
    /// them**, through the application's own route, and asks for nothing.
    struct TheWordsKeptHere {
        server: String,
    }

    impl AsksForThePassword for TheWordsKeptHere {
        fn the_password_for(
            &mut self,
            _step: smoozy_server_setup::ready_action::Step,
        ) -> smoozy_desktop::servers::sessions::ItsOwnFuture<
            '_,
            Option<smoozy_security_and_licence::plaintext::Origin>,
        > {
            Box::pin(async { None })
        }

        fn where_the_words_go(&mut self) -> Option<smoozy_desktop::setting_up::TheWordsGoHere> {
            Some(smoozy_desktop::setting_up::the_words_go_to_the_card(&self.server, || {}))
        }
    }

    /// **A container on Ubuntu 22.04, as a fresh rental reads.**
    fn a_container() -> TheMachineBeforeAnythingIsTouched {
        TheMachineBeforeAnythingIsTouched::as_answered(
            MachineKind::AContainer,
            Some("ID=ubuntu\nVERSION_ID=\"22.04\"\n".to_string()),
            None,
        )
    }

    /// **The Creator's container: the unpack tool already there and a driver
    /// that answers**, so a Blender install needs no admin step at all.
    fn a_container_that_needs_nothing_installed() -> Vec<(String, CommandResult)> {
        let mut script = a_machine_that_already_holds_the_unpack_tool();
        script.retain(|(named, _)| named != THE_CONTAINER_MARKER && named != THE_CONTAINER_TOOL);
        script.push((THE_CONTAINER_MARKER.to_string(), ended_cleanly("")));
        script.push((THE_CONTAINER_TOOL.to_string(), ended_cleanly("docker\n")));
        script
    }

    /// **What `ldd` prints for a build whose every library is there**, as this
    /// file writes it — a stand-in, never a reading of a real machine.
    const A_CLEAN_LIBRARY_CHECK: &str = "\tlinux-vdso.so.1 (0x00007ffd3a5f2000)\n\tlibX11.so.6 => \
        /lib/x86_64-linux-gnu/libX11.so.6 (0x00007f2c1e400000)\n";

    fn the_words(step: &str, ordinary: &str, complaints: &str) -> (String, String, String) {
        (step.to_string(), ordinary.to_string(), complaints.to_string())
    }

    /// **Whether `needle` is anywhere in `haystack`, byte for byte.**
    fn holds(haystack: &[u8], needle: &[u8]) -> bool {
        haystack.windows(needle.len()).any(|window| window == needle)
    }

    /// **The eighteenth notes, piece 2c — `Server Setup` Y7a items 2 and 5
    /// and `Server Setup` Y7d, driven: a worker step's words reach the card while the step is
    /// still running.** The download runs across several readings, printing
    /// its progress lines a piece at a time, and every reading carries what
    /// it printed after the lengths the desktop named — in small pieces here,
    /// so more always follows. **The desktop keeps each piece once, in order,
    /// as its reading lands**: it names a growing length while the step is
    /// still in flight; **it reads on at once for the rest once the step has
    /// ended**, before it commands the unpack; and **the readings come about a
    /// second apart while words move**. Before piece 2c a worker step's words
    /// were kept only once it had ended, whole, at readings fifteen seconds
    /// apart.
    #[test]
    fn eighteenth_a_worker_steps_words_reach_the_card_while_it_runs_in_pieces_about_a_second_apart() {
        use smoozy_server_setup::ready_action::Step;
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("eighteenth-words-while-it-runs");
        let server = "eighteenth-a-worker-steps-words-while-it-runs";
        let row = a_row_with_its_root_at(&scratch);
        let pieces = vec![
            ("blender-5.2.1-linux-x64.tar.xz: 18 of 352 MB, 17.9 MB/s\n".to_string(), String::new()),
            (
                "blender-5.2.1-linux-x64.tar.xz: 36 of 352 MB, 18.1 MB/s\nblender-5.2.1-linux-x64.tar.xz: 54 of 352 MB, 18.2 MB/s\n"
                    .to_string(),
                String::new(),
            ),
            (
                "blender-5.2.1-linux-x64.tar.xz: 352 MB downloaded in 19 s\n".to_string(),
                "a note on the error stream\n".to_string(),
            ),
        ];
        let supervisor = TheSupervisorHere {
            words_while_it_runs: vec![(asked::THE_FETCH_STEP.to_string(), pieces.clone())],
            the_room_on_a_reading: Some(40),
            ..TheSupervisorHere::default()
        };
        assert!(smoozy_desktop::setting_up::a_run_begins(
            server,
            smoozy_server_setup::run::SetupKind::ABlenderInstall
        ));
        let mut screen = TheWordsKeptHere { server: server.to_string() };
        let left = a_blender_install_is_driven_on(
            &scratch,
            sftp_server,
            a_container_that_needs_nothing_installed(),
            row,
            a_container(),
            supervisor,
            &mut screen,
        );
        smoozy_desktop::setting_up::a_run_ends(server);
        assert!(
            matches!(left.came_to, WhereTheSetUpGot::TheRunEnded { ending: TheRunsEnding::ItReachedDone { .. }, .. }),
            "the install did not reach done: {:?}",
            left.came_to
        );
        // **Every piece kept once, in order, under the download.**
        let steps = smoozy_desktop::setting_up::the_words_of_each_step_on(server).expect("the run's words are kept");
        let download = steps.iter().find(|one| one.step == Step::Fetch).expect("the download's words were kept");
        let whole_ordinary: String = pieces.iter().map(|(ordinary, _)| ordinary.as_str()).collect();
        assert_eq!(String::from_utf8_lossy(&download.the_ordinary_output), whole_ordinary, "a piece was lost or kept twice");
        assert_eq!(String::from_utf8_lossy(&download.the_complaints), "a note on the error stream\n");
        // **Kept while it ran**: the desktop named a growing length on
        // readings that found the step still in flight.
        let while_it_ran: Vec<usize> = left
            .readings_named
            .iter()
            .filter(|(_, _, _, in_flight, _)| *in_flight)
            .map(|(_, ordinary, _, _, _)| ordinary.as_deref().and_then(|it| it.parse().ok()).unwrap_or(0))
            .collect();
        assert!(
            while_it_ran.windows(2).any(|pair| pair[1] > pair[0]) && while_it_ran.iter().any(|named| *named > 0),
            "the desktop kept nothing while the step ran: {:?}",
            left.readings_named
        );
        // **Read on at once for the rest** once the step had ended, before
        // the unpack was commanded.
        let after_it_ended = left.readings_named.iter().filter(|(_, _, _, in_flight, _)| !*in_flight).count();
        assert!(after_it_ended >= 2, "the rest was not read before the next step: {:?}", left.readings_named);
        let unpack = left.steps.iter().position(|step| step == asked::THE_UNPACK_STEP).expect("the unpack ran");
        let fetch = left.steps.iter().position(|step| step == asked::THE_FETCH_STEP).expect("the download ran");
        assert!(fetch < unpack);
        // **`Server Setup` Y7d: about a second apart while words move**, never the keepalive
        // cadence's fifteen.
        let in_flight_at: Vec<std::time::Instant> = left
            .readings_named
            .iter()
            .filter(|(_, _, _, in_flight, _)| *in_flight)
            .map(|(.., at)| *at)
            .collect();
        assert!(in_flight_at.len() >= 3, "{:?}", left.readings_named);
        for pair in in_flight_at.windows(2) {
            let gap = pair[1].duration_since(pair[0]);
            assert!(gap < std::time::Duration::from_secs(3), "the readings were {gap:?} apart while words moved");
        }
    }

    /// **The Creator's case, at the desk — the sixteenth notes, note 1: a
    /// Blender install on a container where the unpack tool and every library
    /// are already there, so no admin step runs, keeps every step's words** —
    /// the readings', the library check's and Blender's version answer's, per
    /// step and in the order printed, the two streams apart. **Until
    /// 2026-10-24 this kept nothing at all**: only admin steps' words were
    /// kept, and the Creator's Vast container showed an empty window and an
    /// empty *Server output* for the whole setup.
    #[test]
    fn y7a_a_container_install_with_nothing_to_install_keeps_every_steps_words() {
        use smoozy_server_setup::ready_action::Step;
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("y7a-nothing-to-install");
        let server = "y7a-a-container-with-nothing-to-install";
        let row = a_row_with_its_root_at(&scratch);
        let the_version_answer = "Blender 5.2.1 (hash 8d6e3c7a4f21 built 2026-09-30 01:12:44)\n";
        let a_warning = "Color management: using fallback mode for management\n";
        let supervisor = TheSupervisorHere {
            the_steps_words: vec![
                the_words(asked::THE_UNPACK_STEP, "", ""),
                the_words(asked::THE_LIBRARY_CHAIN_IS_READ_STEP, A_CLEAN_LIBRARY_CHECK, ""),
                the_words(asked::THE_LIBRARY_CHAIN_IS_READ_AGAIN_STEP, A_CLEAN_LIBRARY_CHECK, ""),
                the_words(asked::THE_BUILD_IS_PUBLISHED_STEP, the_version_answer, a_warning),
                the_words(
                    asked::THE_FINAL_VERIFICATION_STEP,
                    &format!("{A_CLEAN_LIBRARY_CHECK}{the_version_answer}"),
                    "",
                ),
            ],
            ..TheSupervisorHere::default()
        };
        assert!(smoozy_desktop::setting_up::a_run_begins(
            server,
            smoozy_server_setup::run::SetupKind::ABlenderInstall
        ));
        let mut screen = TheWordsKeptHere { server: server.to_string() };
        let left = a_blender_install_is_driven_on(
            &scratch,
            sftp_server,
            a_container_that_needs_nothing_installed(),
            row,
            a_container(),
            supervisor,
            &mut screen,
        );
        smoozy_desktop::setting_up::a_run_ends(server);
        assert!(
            matches!(left.came_to, WhereTheSetUpGot::TheRunEnded { ending: TheRunsEnding::ItReachedDone { .. }, .. }),
            "the install did not reach done: {:?}",
            left.came_to
        );
        assert!(
            !left.asked.iter().any(|command| command.contains("apt-get")),
            "an admin step ran on a container that needed none: {:?}",
            left.asked
        );
        let steps = smoozy_desktop::setting_up::the_words_of_each_step_on(server).expect("the run's words are kept");
        assert_eq!(
            steps.iter().map(|one| one.step).collect::<Vec<_>>(),
            vec![Step::Readings, Step::Libraries, Step::BlenderPublication, Step::Done],
            "the steps' words are not kept per step, in order: {steps:#?}"
        );
        let ordinary = |at: usize| String::from_utf8_lossy(&steps[at].the_ordinary_output).into_owned();
        let complaints = |at: usize| String::from_utf8_lossy(&steps[at].the_complaints).into_owned();
        // **The readings' own words, as the server printed them** — the
        // driver's answer and the package system's, among them.
        assert!(ordinary(0).contains("NVIDIA-SMI 580.105.08\n"), "{:?}", ordinary(0));
        assert!(ordinary(0).contains("Status: install ok installed\n"), "{:?}", ordinary(0));
        // **The library check's, its first read and its re-read.**
        assert_eq!(ordinary(1), format!("{A_CLEAN_LIBRARY_CHECK}{A_CLEAN_LIBRARY_CHECK}"));
        // **Blender's version answer**, and its complaint apart from it.
        assert_eq!(ordinary(2), the_version_answer);
        assert_eq!(complaints(2), a_warning);
        assert_eq!(ordinary(3), format!("{A_CLEAN_LIBRARY_CHECK}{the_version_answer}"));
        // **The two streams stay apart**: the one complaint is on its own
        // stream and nowhere in the other.
        let (all_ordinary, all_complaints) =
            smoozy_desktop::setting_up::the_whole_of_what_was_printed_on(server).expect("kept");
        assert_eq!(all_complaints, a_warning);
        assert!(!all_ordinary.contains("Color management"));
        // **And the card's window holds the newest three lines of all of it.**
        assert_eq!(
            smoozy_desktop::setting_up::the_newest_lines_on(server).last().map(String::as_str),
            Some(the_version_answer.trim_end()),
        );
    }

    /// **Nothing secret is shown — the sixteenth notes, note 1,
    /// `Server Setup` Y7a item 4: a
    /// run with a known password and Telegram on with a known token.**
    /// The password rides the elevating command's standard input and the
    /// token the file-transfer subsystem, so neither byte string is in either
    /// kept stream, and neither is in any frame the machine's supervisor was
    /// handed. **The server's own prompt for the password is its words and is
    /// shown.** Not a filter: nothing here takes anything out, and the test
    /// holds where each secret goes.
    #[test]
    fn y7a_neither_the_password_nor_the_bot_token_is_among_the_runs_words_or_on_a_frame() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("y7a-no-secret-shown");
        let server = "y7a-no-secret-is-shown";
        const A_KNOWN_PASSWORD: &str = "a-known-password-for-y7a";
        const THE_PROMPT: &[u8] = b"[sudo] password for root: ";
        let prompted = |output: &[u8]| CommandResult {
            output: output.to_vec(),
            complaint: THE_PROMPT.to_vec(),
            status: Some(0),
        };
        let mut script = with_the_probe(CommandResult {
            output: Vec::new(),
            complaint: b"sudo: a password is required\n".to_vec(),
            status: Some(1),
        });
        script.push((THE_VALIDATE.to_string(), prompted(b"")));
        script.push((THE_LIST_REFRESH_WITH_A_PASSWORD.to_string(), prompted(b"Reading package lists...\n")));
        script.push((THE_TOOL_INSTALL_WITH_A_PASSWORD.to_string(), prompted(b"Setting up xz-utils ...\n")));
        assert!(smoozy_desktop::setting_up::a_run_begins(
            server,
            smoozy_server_setup::run::SetupKind::TheReadyAction
        ));
        let left = the_order_is_driven_as(
            &scratch,
            sftp_server,
            script,
            an_ubuntu_systemd_machine(),
            WhatThePlacementReached::Verified,
            None,
            AsThisRunIsAsked {
                elevation: TheElevationHere::OwedAndTheOfferIsAnsweredWith(A_KNOWN_PASSWORD),
                telegram_is_on: true,
                the_words_are_kept_for: Some(server.to_string()),
                the_steps_words: vec![
                    the_words(asked::THE_LIBRARY_CHAIN_IS_READ_STEP, A_CLEAN_LIBRARY_CHECK, ""),
                    the_words(asked::THE_BUILD_IS_PUBLISHED_STEP, "Blender 4.5.3\n", ""),
                ],
                ..AsThisRunIsAsked::default()
            },
        );
        smoozy_desktop::setting_up::a_run_ends(server);
        assert!(
            matches!(left.came_to, WhereTheSetUpGot::TheRunEnded { ending: TheRunsEnding::ItReachedDone { .. }, .. }),
            "the run did not reach done: {:?}",
            left.came_to
        );
        // **Both secrets were really in play**: the password went out on
        // standard input, and the token's material was composed and placed.
        assert!(
            left.carried_on_standard_input
                .iter()
                .any(|(_, value)| value.as_slice() == A_KNOWN_PASSWORD.as_bytes()),
            "no password went out, so this proved nothing"
        );
        assert_eq!(left.the_composing_was_asked, 1, "the token's material was never composed");
        let (ordinary, complaints) =
            smoozy_desktop::setting_up::the_whole_of_what_was_printed_on(server).expect("the run's words are kept");
        assert!(holds(complaints.as_bytes(), THE_PROMPT), "the server's prompt was not shown: {complaints:?}");
        assert!(ordinary.contains("Setting up xz-utils ...\n"), "{ordinary:?}");
        assert!(ordinary.contains("Blender 4.5.3\n"), "a worker step's words were not kept: {ordinary:?}");
        assert!(!left.frames.is_empty(), "the supervisor was handed nothing");
        for secret in [A_KNOWN_PASSWORD.as_bytes(), A_MADE_UP_TOKEN] {
            let shown = String::from_utf8_lossy(secret);
            assert!(!holds(ordinary.as_bytes(), secret), "{shown:?} is in the output");
            assert!(!holds(complaints.as_bytes(), secret), "{shown:?} is in the complaints");
            for frame in &left.frames {
                assert!(!holds(frame, secret), "{shown:?} rode a frame to the supervisor");
            }
            for command in &left.asked {
                assert!(!holds(command.as_bytes(), secret), "{shown:?} is in a command: {command}");
            }
        }
    }

    /// **The Blender section's *Install*: `Server Setup` Y6b's own kind of run,
    /// carrying the version the user picked to the fetch and the publication,
    /// on the machine's own line — and nothing of the machine's preparation.**
    /// Until 2026-09-21 the control answered that the act was not built.
    #[test]
    fn a_blender_picked_from_the_list_is_installed_by_a_run_of_its_own_kind() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("y6b-a-blender-install");
        let row = a_row_with_its_root_at(&scratch);
        let left = a_blender_install_is_driven_here(
            &scratch,
            sftp_server,
            a_machine_with_a_card_and_no_driver(),
            row,
        );
        assert!(
            matches!(
                left.came_to,
                WhereTheSetUpGot::TheRunEnded {
                    at: smoozy_server_setup::run::Boundary::Done,
                    ending: TheRunsEnding::ItReachedDone { .. },
                }
            ),
            "the install did not reach done: {:?}",
            left.came_to
        );
        assert_eq!(left.kinds, vec![Some(asked::A_BLENDER_INSTALL.to_string())]);
        // **The version rides every step that reads it, and only the
        // version**: `Server Setup` B4 derives every address on the machine.
        // **The unpack is one of them** — step 10c's third pass on Vast found
        // every install stopped there because it was sent none, and the
        // stand-in machine now fails a step that reads the version and was
        // given none, exactly as the machine's own worker does.
        for (step, version) in left.steps.iter().zip(&left.versions) {
            if asked::THE_STEPS_THAT_READ_THE_BLENDER_VERSION.contains(&step.as_str()) {
                assert_eq!(version.as_deref(), Some("5.2.1"), "{step}");
            }
        }
        for step in asked::THE_STEPS_THAT_READ_THE_BLENDER_VERSION {
            assert!(
                left.steps.iter().any(|commanded| commanded == step),
                "{step} was never commanded: {:?}",
                left.steps
            );
        }
        // **And the screen says it was installed**, which it may only say where
        // the run published a Blender.
        let said = smoozy_desktop::surface::set_up::what_the_install_came_to(&left.came_to, "5.2.1");
        assert!(said.said.contains("5.2.1 installed"), "{}", said.said);
        // **A card with no driver is on this machine, and a Blender install
        // touches none of it**: no driver step composed, no install, no
        // restart.
        assert!(
            !left
                .composed
                .iter()
                .any(|boundary| boundary == asked::THE_BOUNDARY_DRIVER_PLACED
                    || boundary == asked::THE_BOUNDARY_RESTART_ISSUED),
            "the install composed the machine's preparation: {:?}",
            left.composed
        );
        assert!(
            !left
                .asked
                .iter()
                .any(|said| said.contains("ubuntu-drivers") || said.contains("reboot")),
            "a Blender install reached the driver or the restart: {:?}",
            left.asked
        );
    }

    /// **Step 10c's owed piece 4, `Server Setup` Y9a: three Blenders ticked,
    /// and Smoozy closes after the first is begun — the reopened card names the
    /// other two with the words saying Smoozy closed first, and a fresh *Set up
    /// again* clears the line.**
    ///
    /// **The press is the card's own** ([`a_run_is_taken_from_the_card`],
    /// which every install press and the ready action's press go through), the
    /// local record is a real one in the throwaway folder, and **the first
    /// install is a real run against the stand-in machine**. **Smoozy closing
    /// is the process ending while that run is going**: the chain never sees
    /// the run end, so nothing after it runs, and the process's memory goes
    /// with it ([`a_run_ends`] stands in for that). **Then Smoozy opens
    /// again** — the record is opened afresh, as the start opens it — and the
    /// card is composed as the Set up tab's command composes it. **Undone (the
    /// press keeping nothing, or *Set up again* not clearing it), this fails.**
    #[test]
    fn three_picked_and_smoozy_closed_after_the_first_names_the_other_two_until_set_up_again() {
        use smoozy_desktop::record::LocalRecord;
        use smoozy_desktop::record::installs_not_begun::{the_ones_waiting_are_kept, the_ones_waiting_on};
        use smoozy_desktop::setting_up::{a_run_ends, a_run_is_taken_from_the_card};
        use smoozy_desktop::surface::set_up::{the_installs_smoozy_closed_on, the_tab, WhatTheMachineSaid};
        use smoozy_server_setup::run::SetupKind;

        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("y9a-closed-first");
        let record_at = scratch.at("local-record.sqlite3");
        let server = format!("y9a-{}", scratch.at.file_name().expect("named").to_string_lossy());

        // **The press: three ticked, the first begun, two kept waiting.**
        let record = LocalRecord::open_at(&record_at).expect("the record opens");
        assert!(a_run_is_taken_from_the_card(
            &server,
            SetupKind::ABlenderInstall,
            vec!["5.2.2".to_string(), "4.5.14".to_string()],
            &mut |waiting| the_ones_waiting_are_kept(record.connection(), &server, waiting).expect("kept"),
        ));
        // **The first install, a real run on the stand-in machine.**
        let row = a_row_with_its_root_at(&scratch);
        let left = a_blender_install_is_driven_here(
            &scratch,
            sftp_server,
            a_machine_with_a_card_and_no_driver(),
            row,
        );
        assert!(it_reached_done(&left.came_to, true), "the first install came to {:?}", left.came_to);
        // **Smoozy closes here**, before the chain could begin the second.
        drop(record);
        a_run_ends(&server);

        // **Smoozy opens again.**
        let record = LocalRecord::open_at(&record_at).expect("the record opens again");
        let mut tab = the_tab(WhatTheMachineSaid::ConnectedAndNeverSetUp, &server);
        tab.the_installs_never_begun = the_installs_smoozy_closed_on(record.connection(), &server);
        assert_eq!(
            tab.the_installs_never_begun.as_ref().map(|line| line.as_str().to_string()),
            Some(
                smoozy_desktop::typography::Prose::new(
                    "Not installed: Blender 5.2.2, Blender 4.5.14. Smoozy closed before they started."
                )
                .as_str()
                .to_string()
            ),
            "the reopened card does not name the two never begun"
        );

        // **A fresh *Set up again* clears the line.**
        assert!(a_run_is_taken_from_the_card(
            &server,
            SetupKind::TheReadyAction,
            Vec::new(),
            &mut |waiting| the_ones_waiting_are_kept(record.connection(), &server, waiting).expect("kept"),
        ));
        a_run_ends(&server);
        assert!(the_ones_waiting_on(record.connection(), &server).expect("read").is_empty());
        assert_eq!(the_installs_smoozy_closed_on(record.connection(), &server), None, "Set up again left the line");
    }

    /// **A Blender asked of a server this copy has no root recorded for starts
    /// nothing and says so.**
    #[test]
    fn a_blender_asked_of_a_server_never_set_up_starts_nothing() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("y6b-never-set-up");
        let left = a_blender_install_is_driven_here(
            &scratch,
            sftp_server,
            a_machine_that_answers(),
            ServerRow {
                remembered_setup: None,
                ..a_row()
            },
        );
        assert_eq!(left.came_to, WhereTheSetUpGot::TheServerIsNotSetUpYet);
        assert!(left.kinds.is_empty(), "a run was opened");
        assert!(left.steps.is_empty(), "a step was commanded");
    }

    /// **A Blender asked of a server whose setup recorded a root and then
    /// failed to place Smoozy's program starts nothing, and says the server is
    /// not set up** — rather than opening a run that no program can start and
    /// answering *"would not start the setup"*, which is what the second pass on
    /// a real rental saw on 2026-09-21 (fault 3).
    #[test]
    fn a_blender_asked_after_a_setup_whose_placement_failed_starts_nothing() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("y6b-placement-failed");
        let left = a_blender_install_is_driven_here(
            &scratch,
            sftp_server,
            a_machine_that_answers(),
            ServerRow {
                last_confirmed_installation_identity: None,
                ..a_row_with_its_root_at(&scratch)
            },
        );
        assert_eq!(left.came_to, WhereTheSetUpGot::TheServerIsNotSetUpYet);
        assert!(left.kinds.is_empty(), "a run was opened");
        assert!(left.steps.is_empty(), "a step was commanded");
    }

    /// **Fault 4's reading, at the desk: a machine whose placement failed holds
    /// no installation, and one that holds any record there does not read as
    /// empty.** `Server Setup` Y6a's *never set up* is drawn only on the first.
    #[test]
    fn the_state_area_reads_empty_only_where_no_installation_was_ever_written() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("never-set-up");
        let area = scratch.at("smoozy").join("package");
        std::fs::create_dir_all(&area).expect("the package area");
        let area_text = area.to_string_lossy().into_owned();
        let mut far_end = TheFarEnd {
            sftp_server,
            at: scratch.at.clone(),
            said: Arc::new(Mutex::new(Vec::new())),
            asked: Arc::new(Mutex::new(Vec::new())),
            supervisor: Arc::new(Mutex::new(TheSupervisorHere::default())),
            children: Arc::new(Mutex::new(Vec::new())),
            goes_down: Arc::new(tokio::sync::Notify::new()),
            the_mark_when_the_restart_arrived: None,
            the_state_when_a_command_arrived: Arc::new(Mutex::new(Vec::new())),
            the_restart_takes_the_connection: false,
            written_when_the_restart_arrived: Arc::new(Mutex::new(None)),
            slow: None,
            drivers: None,
        };
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("a runtime");
        let point = ThisServersSessions::<TheFarEnd>::for_one_server();
        let asking = point.clone();
        let no_additional: TheAdditionalConnection<TheFarEnd> =
            Arc::new(|| Box::pin(async { None }));
        let (empty, then) = runtime.block_on(async {
            let lending = lend_until_it_ends(&point, &mut far_end, &no_additional);
            let read = async {
                let empty =
                    smoozy_desktop::setting_up::the_state_area_holds_nothing(&asking, &area_text)
                        .await;
                std::fs::write(
                    area.join(smoozy_server_protocol::reconnection::THE_RECORD_THE_IDENTITY_SITS_IN),
                    b"not an identity",
                )
                .expect("a record is there");
                let then =
                    smoozy_desktop::setting_up::the_state_area_holds_nothing(&asking, &area_text)
                        .await;
                (empty, then)
            };
            tokio::pin!(lending, read);
            tokio::select! {
                biased;
                _ended = &mut lending => unreachable!("the far end never ends on its own"),
                read = &mut read => read,
            }
        });
        assert!(empty, "a machine with no installation did not read as never set up");
        assert!(!then, "a record that does not read back was taken for no installation");
    }

    // ═════ Ninth-pass fix piece 3: *Set up* on a machine this copy already set up

    /// **A machine this copy set up earlier, standing as a real one stands when
    /// the user presses *Set up* on it** — the ninth pass on Hyperstack, with
    /// *p12h9 long* at 52 of 240 frames.
    ///
    /// **In a real machine's order, not a convenient one.** The placement that
    /// set it up wrote the installation identity into the package state area
    /// in its commit (`State, identity and recovery` Y8) and the row recorded
    /// it with the root the hand-over chose (`Server Setup` R7b); a connection
    /// then read that identity and the supervisor confirmed it
    /// (`State, identity and recovery` Y9), which is what the run is handed as
    /// the live connection's reading; `Servers` Q1 step 7 read the machine's
    /// work, so the row's cursor points into this installation's journal; and
    /// a job is rendering under the supervisor when the press comes. **Nothing
    /// here is read by the act before the press** — the reading is the
    /// connection's, taken before it.
    struct AMachineAlreadySetUp {
        row: ServerRow,
        /// The installation identity the machine's state area holds.
        installation: String,
        /// Where that record sits on the machine.
        the_record: PathBuf,
    }

    /// The cursor `Servers` Q1 step 7 left, pointing into this installation's
    /// own journal.
    const THE_LIVE_CURSOR: u64 = 41;

    fn a_machine_already_set_up(scratch: &Scratch) -> AMachineAlreadySetUp {
        let installation = Identity::issue(IdentityKind::Installation)
            .expect("an installation")
            .as_text();
        for area in ["jobs", "job-records", "assets", "blender", "package"] {
            std::fs::create_dir_all(scratch.at("smoozy").join(area)).expect("the areas");
        }
        let the_record = scratch
            .at("smoozy/package")
            .join(smoozy_server_protocol::reconnection::THE_RECORD_THE_IDENTITY_SITS_IN);
        std::fs::write(&the_record, &installation).expect("the placement's commit");
        AMachineAlreadySetUp {
            row: ServerRow {
                recorded_machine_root: Some(RecordedMachineRoot {
                    path: format!("{}/smoozy", scratch.text()),
                    the_package_areas_own_location: None,
                    read_under: installation.clone(),
                }),
                last_confirmed_installation_identity: Some(installation.clone()),
                event_cursor: Some(THE_LIVE_CURSOR),
                remembered_setup: None,
                ..a_row()
            },
            installation,
            the_record,
        }
    }

    /// **The card's own *Set up*, pressed on that machine while it renders.**
    fn set_up_is_pressed_on(
        scratch: &Scratch,
        sftp_server: PathBuf,
        row: ServerRow,
        the_live_connection_read: Option<InstallationCheck>,
    ) -> WhatTheRunLeft {
        the_order_is_driven_as(
            scratch,
            sftp_server,
            a_machine_whose_driver_answers(),
            an_ubuntu_systemd_machine(),
            WhatThePlacementReached::Verified,
            None,
            AsThisRunIsAsked {
                // **The card's action asks for no Blender** (`Server Setup`
                // Y6b).
                install_this_blender: None,
                the_row: Some(row),
                the_live_connection_read,
                a_job_is_rendering: Some("p12h9 long".to_string()),
                ..AsThisRunIsAsked::default()
            },
        )
    }

    /// **Ninth-pass fix piece 3: on a machine the live connection read as
    /// *present and equal, confirmed*, the ready action opens `Server Setup`
    /// R1's run on the recorded layout and does nothing else.**
    ///
    /// **Fault 4 of the ninth pass on Hyperstack, at the desk.** *Set up* on a
    /// set-up machine ran `Server Setup` R2's whole order: a hand-over, a
    /// release, a placement under a **new installation identity** that
    /// restarted the supervisor under a live render, and the row's event cursor
    /// cleared — so *p12h9 long* started on one installation's clock and ended
    /// on another's, and History, keeping `State, identity and recovery` D5,
    /// could give it no duration. **`Server Setup` R2 places only on a machine
    /// never prepared; R1 has no placement step; `Server Setup` Y6b: *"On a
    /// machine prepared earlier, the placement replaces nothing."***
    #[test]
    fn set_up_on_a_machine_already_set_up_places_nothing_and_opens_the_run_alone() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("set-up-again-confirmed");
        let machine = a_machine_already_set_up(&scratch);
        let left = set_up_is_pressed_on(
            &scratch,
            sftp_server,
            machine.row.clone(),
            Some(InstallationCheck::PresentAndEqualConfirmed),
        );

        // **The run is `Server Setup` R1's, and it reached its end.**
        assert!(
            matches!(
                left.came_to,
                WhereTheSetUpGot::TheRunEnded {
                    ending: TheRunsEnding::ItReachedDone { .. },
                    ..
                }
            ),
            "the run did not reach done: {:?}",
            left.came_to
        );
        assert_eq!(
            left.kinds,
            vec![Some(asked::THE_READY_ACTION.to_string())],
            "the ready action's run was not the one opened"
        );

        // **Nothing was placed, and the supervisor went on rendering.**
        assert_eq!(left.given, None, "a placement was performed on a set-up machine");
        assert!(
            left.the_supervisor_restarted_with.is_empty(),
            "the supervisor was restarted under {:?}",
            left.the_supervisor_restarted_with
        );
        assert_eq!(
            std::fs::read_to_string(&machine.the_record).expect("the installation record"),
            machine.installation,
            "the machine's installation identity was replaced"
        );

        // **The row is exactly as the connection left it**: the same
        // installation, the same root, and the cursor still pointing into this
        // installation's journal (`State, identity and recovery` E2).
        assert_eq!(
            left.row.last_confirmed_installation_identity,
            Some(machine.installation.clone()),
            "a new installation identity was issued"
        );
        assert_eq!(
            left.row.event_cursor,
            Some(THE_LIVE_CURSOR),
            "the event cursor was cleared under a live installation"
        );
        assert_eq!(left.row.recorded_machine_root, machine.row.recorded_machine_root);
        assert!(
            left.writes.is_empty(),
            "the row was written: {:?}",
            left.writes
        );

        // **No release was read and no rental tuple or hand-over was taken** —
        // `Server Setup` R2's machine-changing prefix, none of it R1's.
        assert!(left.hosting_asked.is_empty(), "a release was fetched: {:?}", left.hosting_asked);
        assert!(
            !left.asked.iter().any(|said| said.contains("uname -m")),
            "the rental's tuple was read for a placement: {:?}",
            left.asked
        );
    }

    /// **Every other reading the live connection can have taken runs
    /// `Server Setup` R2 whole, as before** — ninth-pass fix piece 3's
    /// fall-backs.
    ///
    /// - **Present and different**: `State, identity and recovery` Y9's
    ///   discard; the machine is not this copy's installation.
    /// - **Present, and the supervisor did not answer**: nothing may stand in
    ///   for *present and equal*
    ///   (`State, identity and recovery` Y9), and `Server Setup` F1's ground
    ///   is left exactly as it was.
    /// - **No reading on the live connection.**
    /// - **Confirmed, and the row records no package area**: the run needs the
    ///   area, and the act falls back to R2 whole rather than guessing one.
    #[test]
    fn set_up_where_the_connection_did_not_confirm_this_installation_runs_the_whole_order() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let cases: [(&str, Option<InstallationCheck>, bool); 4] = [
            ("different", Some(InstallationCheck::PresentAndDifferent), true),
            (
                "not-answering",
                Some(InstallationCheck::PresentAndTheSupervisorDidNotAnswer),
                true,
            ),
            ("no-reading", None, true),
            ("no-area", Some(InstallationCheck::PresentAndEqualConfirmed), false),
        ];
        for (named, read, the_area_is_recorded) in cases {
            let scratch = Scratch::new(&format!("set-up-again-{named}"));
            let machine = a_machine_already_set_up(&scratch);
            let row = if the_area_is_recorded {
                machine.row.clone()
            } else {
                ServerRow {
                    recorded_machine_root: None,
                    ..machine.row.clone()
                }
            };
            let left = set_up_is_pressed_on(&scratch, sftp_server.clone(), row, read);
            assert!(
                matches!(
                    left.came_to,
                    WhereTheSetUpGot::TheRunEnded {
                        ending: TheRunsEnding::ItReachedDone { .. },
                        ..
                    }
                ),
                "{named}: the whole order did not reach done: {:?}",
                left.came_to
            );
            assert!(left.given.is_some(), "{named}: `Server Setup` R2's placement did not run");
            assert!(
                left.writes.iter().any(|write| write == "installation"),
                "{named}: the placement's installation was not recorded: {:?}",
                left.writes
            );
        }
    }

    // ------------------------------------------------------ this test's own deadline

    /// **What every test in this file waits on**, named in the message a missed
    /// deadline prints.
    const WHAT_THE_TESTS_HERE_WAIT_ON: &str =
        "OpenSSH's own sftp-server on a socket pair, and the stand-in supervisor";

    /// **No test in this file can wait for ever.** Added 2026-09-21, after hosted
    /// runs `35595657437` and `35601023619` sat three and a half hours on two tests
    /// that were waiting for an answer that could never come, and told nobody.
    ///
    /// **A test's own bound and nothing of the plan's**: five minutes, where the
    /// slowest test in the workspace waits a minute and a half on purpose and the
    /// rest take seconds. It is started as a test's first line and ends when the
    /// test does, pass or panic, because dropping it closes the channel it watches.
    /// **If it is still standing at the bound, a wait inside the test never
    /// ended**: it says which test and what it was waiting on, ends every program
    /// this binary started, and ends the binary, so the run fails instead of
    /// hanging.
    struct TheTestsDeadline {
        _held: std::sync::mpsc::Sender<()>,
    }

    impl TheTestsDeadline {
        const THE_BOUND: std::time::Duration = std::time::Duration::from_secs(300);

        fn started() -> TheTestsDeadline {
            Self::started_for(Self::THE_BOUND)
        }

        /// **The same, with a longer bound for a test that crosses a far line
        /// on purpose** (2026-10-26, the worldwide pass's setup faults).
        fn started_for(bound: std::time::Duration) -> TheTestsDeadline {
            let (held, watched) = std::sync::mpsc::channel::<()>();
            let test = std::thread::current()
                .name()
                .unwrap_or("a test in this file")
                .to_string();
            std::thread::spawn(move || {
                if watched.recv_timeout(bound)
                    == Err(std::sync::mpsc::RecvTimeoutError::Timeout)
                {
                    // **Straight to the complaints stream, not through `eprintln!`**: the
                    // test harness captures that in a test's threads and never prints what
                    // it captured when the binary is ended from inside, which is what
                    // happens next.
                    let said = format!(
                        "DEADLINE MISSED: {test} was still waiting after {:?}, on {}. Every \
                         program this test binary started is ended now, and so is the binary, so \
                         that this fails rather than hangs.\n",
                        bound,
                        WHAT_THE_TESTS_HERE_WAIT_ON
                    );
                    let _ = std::io::Write::write_all(&mut std::io::stderr(), said.as_bytes());
                    if cfg!(unix) {
                        let _ = std::process::Command::new("pkill")
                            .args(["-KILL", "-P", &std::process::id().to_string()])
                            .status();
                    }
                    std::process::exit(101);
                }
            });
            TheTestsDeadline { _held: held }
        }
    }

    // ═══════════════ `Server Setup` Y7a over a real connection — piece S
    //
    // **The Creator's fourteenth notes, note 1** (*"Server setup output doesn't
    // actually output anything"*, and then *"I didn't see anything at all"*),
    // on a fresh Vast container whose setup ran its admin installs. Added
    // 2026-10-18, piece S. **Every hop between the far end and the Set up card
    // had been proved alone and none had been proved together**, so this drives
    // the add's own setup — `a_machine_is_set_up`, the elevated channel,
    // `Server Setup`'s own `elevated::issue` and the transport's
    // `to_its_end_told_as_it_comes` — over a real SSH connection to an SSH
    // server run inside this test, with an admin command that prints over
    // seconds, and looks at the card the way the Set up tab does each time it
    // is told.
    mod over_a_real_connection {
        use super::*;

        use std::collections::HashMap;
        use std::time::{Duration, Instant};

        use russh::server::{Auth, Msg, Server as _, Session};
        use russh::{Channel, ChannelId};
        use smoozy_desktop::setting_up::{
            the_figure_goes_to_the_card, the_run_is_read_under_way, the_words_go_to_the_card, TheFigureGoesHere,
            TheSetupAsRead, TheWordsGoHere,
        };
        use smoozy_desktop::surface::set_up::{the_servers_output_on, the_setup_under_way_on};
        use smoozy_desktop::typography::Verbatim;
        use smoozy_server_protocol::transport::ssh;
        use smoozy_server_protocol::transport::{Credential, DecideHostTrust, Endpoint, Figures, HostTrust};
        use smoozy_server_setup::ready_action::Step;

        /// **One piece an admin command prints, after a wait.**
        #[derive(Clone, Copy)]
        pub(super) enum Printed {
            Output(&'static str),
            Complaint(&'static str),
        }

        /// **An admin command that prints its pieces over seconds and then
        /// ends cleanly**, the way `apt-get` does on a rental.
        #[derive(Clone)]
        pub(super) struct ACommandThatPrints {
            pub(super) command: String,
            pub(super) pieces: Vec<(Duration, Printed)>,
        }

        /// **The far end: a real SSH server, in this process.** Every command
        /// it is sent is answered by [`TheFarEnd`] — its script and its
        /// supervisor, the same as every run above — except the commands that
        /// print over seconds, which it prints itself, piece by piece. Every
        /// file-transfer session is OpenSSH's own `sftp-server` in the
        /// throwaway folder.
        #[derive(Clone)]
        struct TheMachineOverSsh {
            far_end: Arc<tokio::sync::Mutex<TheFarEnd>>,
            printing: Arc<Vec<ACommandThatPrints>>,
            /// **What each printing command did, at the far end, and when.**
            did: Arc<Mutex<Vec<(String, &'static str, Instant)>>>,
            sftp_server: PathBuf,
            at: PathBuf,
            children: Arc<Mutex<Vec<std::process::Child>>>,
            channels: HashMap<ChannelId, OnTheChannel>,
        }

        #[derive(Clone)]
        enum OnTheChannel {
            ACommand { command: String, input: Vec<u8> },
            Sftp(tokio::sync::mpsc::UnboundedSender<Vec<u8>>),
        }

        impl russh::server::Server for TheMachineOverSsh {
            type Handler = Self;
            fn new_client(&mut self, _address: Option<std::net::SocketAddr>) -> Self {
                self.clone()
            }
        }

        impl russh::server::Handler for TheMachineOverSsh {
            type Error = russh::Error;

            async fn auth_publickey(
                &mut self,
                _user: &str,
                _key: &russh::keys::PublicKey,
            ) -> Result<Auth, Self::Error> {
                Ok(Auth::Accept)
            }

            async fn channel_open_session(
                &mut self,
                _channel: Channel<Msg>,
                reply: russh::server::ChannelOpenHandle,
                _session: &mut Session,
            ) -> Result<(), Self::Error> {
                reply.accept().await;
                Ok(())
            }

            async fn exec_request(
                &mut self,
                channel: ChannelId,
                command: &[u8],
                session: &mut Session,
            ) -> Result<(), Self::Error> {
                session.channel_success(channel)?;
                self.channels.insert(
                    channel,
                    OnTheChannel::ACommand {
                        command: String::from_utf8_lossy(command).into_owned(),
                        input: Vec::new(),
                    },
                );
                Ok(())
            }

            async fn subsystem_request(
                &mut self,
                channel: ChannelId,
                name: &str,
                session: &mut Session,
            ) -> Result<(), Self::Error> {
                use std::os::unix::io::{AsRawFd, FromRawFd};
                if name != "sftp" {
                    session.channel_failure(channel)?;
                    return Ok(());
                }
                let (ours, theirs) = std::os::unix::net::UnixStream::pair()?;
                ours.set_nonblocking(true)?;
                let copy = |descriptor: std::os::unix::io::RawFd| {
                    let copy = unsafe { libc::dup(descriptor) };
                    assert!(copy >= 0, "the descriptor could not be copied");
                    unsafe { std::process::Stdio::from_raw_fd(copy) }
                };
                let child = std::process::Command::new(&self.sftp_server)
                    .current_dir(&self.at)
                    .stdin(copy(theirs.as_raw_fd()))
                    .stdout(copy(theirs.as_raw_fd()))
                    .stderr(std::process::Stdio::null())
                    .spawn()?;
                drop(theirs);
                self.children.lock().expect("the children").push(child);
                let (mut reading, mut writing) = tokio::net::UnixStream::from_std(ours)?.into_split();
                let (to_it, mut its_input) = tokio::sync::mpsc::unbounded_channel::<Vec<u8>>();
                tokio::spawn(async move {
                    use tokio::io::AsyncWriteExt;
                    while let Some(bytes) = its_input.recv().await {
                        if writing.write_all(&bytes).await.is_err() {
                            break;
                        }
                    }
                });
                let handle = session.handle();
                tokio::spawn(async move {
                    use tokio::io::AsyncReadExt;
                    let mut buffer = vec![0u8; 64 * 1024];
                    loop {
                        match reading.read(&mut buffer).await {
                            Ok(0) | Err(_) => break,
                            Ok(read) => {
                                if handle.data(channel, buffer[..read].to_vec()).await.is_err() {
                                    break;
                                }
                            }
                        }
                    }
                    let _ = handle.eof(channel).await;
                    let _ = handle.close(channel).await;
                });
                self.channels.insert(channel, OnTheChannel::Sftp(to_it));
                session.channel_success(channel)?;
                Ok(())
            }

            async fn data(
                &mut self,
                channel: ChannelId,
                data: &[u8],
                _session: &mut Session,
            ) -> Result<(), Self::Error> {
                match self.channels.get_mut(&channel) {
                    Some(OnTheChannel::ACommand { input, .. }) => input.extend_from_slice(data),
                    Some(OnTheChannel::Sftp(to_it)) => {
                        let _ = to_it.send(data.to_vec());
                    }
                    None => {}
                }
                Ok(())
            }

            async fn channel_eof(
                &mut self,
                channel: ChannelId,
                session: &mut Session,
            ) -> Result<(), Self::Error> {
                let Some(OnTheChannel::ACommand { command, input }) = self.channels.remove(&channel) else {
                    // **A file-transfer session's input ended**: dropping the
                    // way in closes the program's.
                    return Ok(());
                };
                if std::env::var_os("SMOOZY_DESK_TELLS_EACH_ASK").is_some() {
                    eprintln!("{:?} ran {}", std::time::SystemTime::now(), command.chars().take(70).collect::<String>());
                }
                let handle = session.handle();
                let printing = self.printing.iter().find(|one| one.command == command).cloned();
                let did = Arc::clone(&self.did);
                let far_end = Arc::clone(&self.far_end);
                tokio::spawn(async move {
                    if let Some(printing) = printing {
                        did.lock().expect("what it did").push((command.clone(), "began", Instant::now()));
                        for (wait, piece) in printing.pieces {
                            tokio::time::sleep(wait).await;
                            let _ = match piece {
                                Printed::Output(words) => handle.data(channel, words.as_bytes().to_vec()).await,
                                Printed::Complaint(words) => {
                                    handle.extended_data(channel, 1, words.as_bytes().to_vec()).await
                                }
                            };
                            did.lock().expect("what it did").push((command.clone(), "printed", Instant::now()));
                        }
                        did.lock().expect("what it did").push((command.clone(), "ended", Instant::now()));
                        let _ = handle.exit_status_request(channel, 0).await;
                    } else {
                        let frame = (!input.is_empty()).then_some(input);
                        let constant = CommandString::constant(Box::leak(command.into_boxed_str()));
                        let answered = {
                            let mut far_end = far_end.lock().await;
                            AHeldConnection::a_command(&mut *far_end, constant, frame).await
                        };
                        if let Ok(result) = answered {
                            if !result.output.is_empty() {
                                let _ = handle.data(channel, result.output).await;
                            }
                            if !result.complaint.is_empty() {
                                let _ = handle.extended_data(channel, 1, result.complaint).await;
                            }
                            if let Some(status) = result.status {
                                let _ = handle.exit_status_request(channel, status).await;
                            }
                        }
                    }
                    let _ = handle.eof(channel).await;
                    let _ = handle.close(channel).await;
                });
                Ok(())
            }

            async fn channel_close(
                &mut self,
                channel: ChannelId,
                _session: &mut Session,
            ) -> Result<(), Self::Error> {
                self.channels.remove(&channel);
                Ok(())
            }
        }

        /// The host key is taken as offered: what is proved here is past it.
        struct TheHostIsTaken;

        impl DecideHostTrust for TheHostIsTaken {
            fn decide(&mut self, _offered: &[u8]) -> HostTrust {
                HostTrust::Proceed
            }
        }

        /// **What the Set up card showed, each time the tab was told** — the
        /// panel `the_setup_under_way` answers the page with, composed by the
        /// application's own function.
        #[derive(Debug, Clone)]
        pub(super) struct WhatTheCardShowed {
            pub(super) at: Instant,
            pub(super) because: &'static str,
            /// Which of the card's steps was marked under way.
            pub(super) under_way: Option<String>,
            pub(super) lines: Vec<String>,
            /// **Which of the lines came from the error stream** (from
            /// 2026-10-25, the eighteenth notes, piece 2c).
            pub(super) errors: Vec<bool>,
            pub(super) server_output: bool,
            /// **Every step's mark, by the step's own name** (2026-10-26, the
            /// worldwide pass's setup faults, fault 4).
            pub(super) marks: Vec<(String, String)>,
            /// **Whether any step carried the download's or the unpack's
            /// figure** (2026-10-26, fault 1).
            pub(super) a_figure_stood: bool,
        }

        fn the_card_is_read(server: &str, because: &'static str, seen: &Mutex<Vec<WhatTheCardShowed>>) {
            let panel = the_setup_under_way_on(server, || "Vast 4090".to_string());
            let showed = WhatTheCardShowed {
                at: Instant::now(),
                because,
                under_way: panel.as_ref().and_then(|panel| {
                    panel
                        .steps
                        .iter()
                        .find(|step| step.mark == Verbatim::new("now"))
                        .map(|step| step.name.as_str().to_string())
                }),
                lines: panel
                    .as_ref()
                    .map(|panel| {
                        panel
                            .the_servers_newest_lines
                            .iter()
                            .map(|line| line.as_str().to_string())
                            .collect()
                    })
                    .unwrap_or_default(),
                errors: panel
                    .as_ref()
                    .map(|panel| panel.which_lines_are_from_the_error_stream.clone())
                    .unwrap_or_default(),
                server_output: panel.as_ref().is_some_and(|panel| panel.server_output.is_some()),
                marks: panel
                    .as_ref()
                    .map(|panel| {
                        panel
                            .steps
                            .iter()
                            .map(|step| (step.name.as_str().to_string(), step.mark.as_str().to_string()))
                            .collect()
                    })
                    .unwrap_or_default(),
                a_figure_stood: panel
                    .as_ref()
                    .is_some_and(|panel| panel.steps.iter().any(|step| step.the_figure.is_some())),
            };
            seen.lock().expect("what the card showed").push(showed);
        }

        /// **The Set up tab, at the desk**: the two things `TheSetUpTabAsks`
        /// does for the card, through the same two functions, with the tab's
        /// asking for the panel taken at each telling.
        struct TheSetUpTabHere {
            server: String,
            seen: Arc<Mutex<Vec<WhatTheCardShowed>>>,
            /// **Whether the download's and the unpack's figure reaches the
            /// card**, as `TheSetUpTabAsks` hands it — only where a test asks
            /// (2026-10-26, the worldwide pass's setup faults, fault 1).
            figures: bool,
        }

        impl AsksForThePassword for TheSetUpTabHere {
            fn the_password_for(
                &mut self,
                _step: Step,
            ) -> smoozy_desktop::servers::sessions::ItsOwnFuture<
                '_,
                Option<smoozy_security_and_licence::plaintext::Origin>,
            > {
                Box::pin(async { None })
            }

            fn the_run_is_as_read(&mut self, read: &TheSetupAsRead) {
                if the_run_is_read_under_way(&self.server, read) {
                    the_card_is_read(&self.server, "the run moved on", &self.seen);
                }
            }

            fn where_the_words_go(&mut self) -> Option<TheWordsGoHere> {
                let server = self.server.clone();
                let seen = Arc::clone(&self.seen);
                Some(the_words_go_to_the_card(&self.server, move || {
                    the_card_is_read(&server, "the server printed", &seen)
                }))
            }

            fn where_the_figure_goes(&mut self, the_build: Option<&str>) -> Option<TheFigureGoesHere> {
                if !self.figures {
                    return None;
                }
                let server = self.server.clone();
                let seen = Arc::clone(&self.seen);
                Some(the_figure_goes_to_the_card(&self.server, the_build, move || {
                    the_card_is_read(&server, "the figure moved", &seen)
                }))
            }
        }

        pub(super) struct WhatTheRealRunLeft {
            pub(super) came_to: WhereTheSetUpGot,
            pub(super) seen: Vec<WhatTheCardShowed>,
            pub(super) did: Vec<(String, &'static str, Instant)>,
            pub(super) asked: Vec<String>,
            /// *Server output*, opened once the run has ended.
            pub(super) server_output: Option<smoozy_desktop::surface::set_up::TheServersOutput>,
            /// The card once the run has ended.
            pub(super) the_card_after: Option<smoozy_desktop::surface::set_up::TheRunPanel>,
            /// **Every request the supervisor answered, by name, and when**;
            /// **when the step on the clock was commanded**, where one was;
            /// and **when the run came back** (2026-10-26, fault 1).
            pub(super) answered_at: Vec<(String, Instant)>,
            pub(super) the_clock_began: Option<Instant>,
            pub(super) the_run_came_back: Instant,
        }

        fn a_key(scratch: &Scratch, name: &str) -> PathBuf {
            let path = scratch.at(name);
            let made = std::process::Command::new("ssh-keygen")
                .args(["-q", "-t", "ed25519", "-N", "", "-C", "piece S", "-f"])
                .arg(&path)
                .output()
                .expect("ssh-keygen is on the path");
            assert!(made.status.success(), "ssh-keygen: {}", String::from_utf8_lossy(&made.stderr));
            path
        }

        /// A container on Ubuntu 22.04, as a fresh Vast rental reads.
        fn a_vast_container() -> TheMachineBeforeAnythingIsTouched {
            TheMachineBeforeAnythingIsTouched::as_answered(
                MachineKind::AContainer,
                Some("ID=ubuntu\nVERSION_ID=\"22.04\"\n".to_string()),
                None,
            )
        }

        /// **The Add dialog's *Add and set up*, carried out over a real
        /// connection** — the run taken the way the add takes it, the
        /// application's own order, the run let go at its end — with every
        /// telling of the tab looked at as the tab looks.
        pub(super) fn a_container_is_added_and_set_up(
            scratch: &Scratch,
            sftp_server: PathBuf,
            server: &str,
            script: Vec<(String, CommandResult)>,
            printing: Vec<ACommandThatPrints>,
        ) -> WhatTheRealRunLeft {
            a_container_is_added_and_set_up_over(
                scratch,
                sftp_server,
                server,
                script,
                printing,
                TheLineAndTheClock::default(),
            )
        }

        /// **How far away the machine is, and what prints on its own clock** —
        /// nothing and nothing for every run above. Added 2026-10-26, the
        /// worldwide pass's setup faults, fault 1.
        #[derive(Default)]
        pub(super) struct TheLineAndTheClock {
            /// **Every byte handed on this long after it arrived, each way**,
            /// as a far server's line does.
            pub(super) late: Duration,
            /// **A worker step's lines printed on the machine's own clock** —
            /// see `TheSupervisorHere::words_on_a_clock`.
            pub(super) words_on_a_clock: Option<(String, Vec<(Duration, String)>, Duration)>,
        }

        /// **One direction of one connection, every byte handed on `late`
        /// after it arrived** — `tests/files_list_again_once_the_connection_is_back.rs`'s
        /// line, the same shape. What arrives meanwhile is read and held, so
        /// the delay is paid once per crossing and never once per piece.
        async fn carried_late<R, W>(mut from: R, mut to: W, late: Duration)
        where
            R: tokio::io::AsyncRead + Unpin + Send + 'static,
            W: tokio::io::AsyncWrite + Unpin,
        {
            use tokio::io::{AsyncReadExt, AsyncWriteExt};
            let (tell, mut told) = tokio::sync::mpsc::unbounded_channel::<(Instant, Vec<u8>)>();
            let reading = tokio::spawn(async move {
                let mut buffer = vec![0u8; 64 * 1024];
                loop {
                    match from.read(&mut buffer).await {
                        Ok(0) | Err(_) => break,
                        Ok(got) => {
                            if tell.send((Instant::now() + late, buffer[..got].to_vec())).is_err() {
                                break;
                            }
                        }
                    }
                }
            });
            while let Some((due, bytes)) = told.recv().await {
                tokio::time::sleep_until(tokio::time::Instant::from_std(due)).await;
                if to.write_all(&bytes).await.is_err() {
                    break;
                }
            }
            let _ = to.shutdown().await;
            reading.abort();
        }

        /// **[`a_container_is_added_and_set_up`], over a line and with a
        /// clock** (2026-10-26, fault 1).
        pub(super) fn a_container_is_added_and_set_up_over(
            scratch: &Scratch,
            sftp_server: PathBuf,
            server: &str,
            script: Vec<(String, CommandResult)>,
            printing: Vec<ACommandThatPrints>,
            line_and_clock: TheLineAndTheClock,
        ) -> WhatTheRealRunLeft {
            let host_key_file = a_key(scratch, "host-key");
            let client_key_file = a_key(scratch, "client-key");
            let release = a_release();
            let asked = Arc::new(Mutex::new(Vec::new()));
            let supervisor = Arc::new(Mutex::new(TheSupervisorHere {
                the_record_carries_its_moments: line_and_clock.words_on_a_clock.is_some(),
                words_on_a_clock: line_and_clock.words_on_a_clock.clone(),
                ..TheSupervisorHere::default()
            }));
            let children = Arc::new(Mutex::new(Vec::new()));
            let far_end = TheFarEnd {
                sftp_server: sftp_server.clone(),
                at: scratch.at.clone(),
                said: Arc::new(Mutex::new(script)),
                asked: Arc::clone(&asked),
                supervisor: Arc::clone(&supervisor),
                children: Arc::clone(&children),
                goes_down: Arc::new(tokio::sync::Notify::new()),
                the_mark_when_the_restart_arrived: None,
                the_state_when_a_command_arrived: Arc::new(Mutex::new(Vec::new())),
                the_restart_takes_the_connection: false,
                written_when_the_restart_arrived: Arc::new(Mutex::new(None)),
                slow: None,
                drivers: None,
            };
            let did = Arc::new(Mutex::new(Vec::new()));
            let machine = TheMachineOverSsh {
                far_end: Arc::new(tokio::sync::Mutex::new(far_end)),
                printing: Arc::new(printing),
                did: Arc::clone(&did),
                sftp_server,
                at: scratch.at.clone(),
                children: Arc::clone(&children),
                channels: HashMap::new(),
            };
            let mut row = TheRowHere {
                row: a_row(),
                writes: Arc::new(Mutex::new(Vec::new())),
            };
            let placement = ThePlacementHere {
                reached: WhatThePlacementReached::Verified,
                given: Arc::new(Mutex::new(None)),
                sessions_open_at_the_lend: Arc::new(Mutex::new(None)),
                sessions: Arc::new(Mutex::new(0)),
                commits_on_the_machine: None,
                the_work_the_commit_reads: None,
            };
            let mut releases = TheReleaseHere {
                root: release.root,
                hosting: WhatHostingWouldServe {
                    table: vec![
                        (
                            WHERE_THE_CURRENT_RELEASE_IS_READ_FROM.to_string(),
                            release.offered.canonical_bytes(),
                        ),
                        (THE_ARTIFACTS_SOURCE.to_string(), release.artifact.clone()),
                    ],
                    asked: Vec::new(),
                },
                store: TheTrustRecordHere::default(),
            };
            let mut taking = TheOpeningReadingsHere {
                said: a_vast_container(),
                asked_after: Arc::clone(&asked),
                asked_at: Arc::new(Mutex::new(None)),
            };
            // **The Add dialog's two ticks**: the ready action, with the latest
            // stable Blender.
            let asked_for = WhatWasAskedFor {
                install_this_blender: Some("5.2.2".to_string()),
                telegram_is_on: false,
                ..WhatWasAskedFor::default()
            };
            let seen = Arc::new(Mutex::new(Vec::new()));
            let mut the_tab = TheSetUpTabHere {
                server: server.to_string(),
                seen: Arc::clone(&seen),
                figures: line_and_clock.words_on_a_clock.is_some(),
            };
            // **The add takes its run before anything is sent**, the way
            // `the_remembered_setup_is_carried_out` takes it.
            assert!(
                smoozy_desktop::setting_up::the_remembered_setup_is_taken(server, &mut |_| {}),
                "another run stood on this server"
            );
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("a runtime for the run");
            let came_to = runtime.block_on(async {
                let host_key = russh::keys::load_secret_key(&host_key_file, None)
                    .expect("ssh-keygen wrote this file a moment ago");
                let config = Arc::new(russh::server::Config {
                    keys: vec![host_key],
                    auth_rejection_time: Duration::from_millis(0),
                    auth_rejection_time_initial: Some(Duration::from_millis(0)),
                    ..russh::server::Config::default()
                });
                let socket: &'static tokio::net::TcpListener = Box::leak(Box::new(
                    tokio::net::TcpListener::bind(("127.0.0.1", 0))
                        .await
                        .expect("the loopback address takes a listener"),
                ));
                let address = socket.local_addr().expect("a bound listener has an address");
                let machine: &'static mut TheMachineOverSsh = Box::leak(Box::new(machine));
                let running = machine.run_on_socket(config, socket);
                let shut_down = running.handle();
                tokio::spawn(running);
                // **The line, where the run is far away**: a relay that hands
                // every byte on `late` after it arrived, each way.
                let address = if line_and_clock.late.is_zero() {
                    address
                } else {
                    let relay = tokio::net::TcpListener::bind(("127.0.0.1", 0))
                        .await
                        .expect("the loopback address takes a listener");
                    let relayed = relay.local_addr().expect("a bound listener has an address");
                    let late = line_and_clock.late;
                    tokio::spawn(async move {
                        while let Ok((desk, _)) = relay.accept().await {
                            let Ok(machine) = tokio::net::TcpStream::connect(address).await else {
                                break;
                            };
                            let _ = desk.set_nodelay(true);
                            let _ = machine.set_nodelay(true);
                            let (desk_reads, desk_writes) = desk.into_split();
                            let (machine_reads, machine_writes) = machine.into_split();
                            tokio::spawn(carried_late(desk_reads, machine_writes, late));
                            tokio::spawn(carried_late(machine_reads, desk_writes, late));
                        }
                    });
                    relayed
                };

                let mut connection = ssh::open(
                    &Endpoint {
                        host: address.ip().to_string(),
                        port: address.port(),
                        username: "root".to_string(),
                    },
                    Credential {
                        private_key_file: client_key_file.clone(),
                        passphrase: None,
                    },
                    Figures {
                        interval: Duration::from_secs(15),
                        unanswered_before_the_break: 3,
                    },
                    Box::new(TheHostIsTaken),
                )
                .await
                .expect("the connection opens");
                let point = ThisServersSessions::<ssh::Connection>::for_one_server();
                let asking = point.clone();
                let no_additional: TheAdditionalConnection<ssh::Connection> =
                    Arc::new(|| Box::pin(async { None }));
                let mut telegram = TelegramIsNotOn;
                let came_to = {
                    let lending = lend_until_it_ends(&point, &mut connection, &no_additional);
                    let set_up = a_machine_is_set_up(
                        &asking,
                        &mut taking,
                        &mut row,
                        placement,
                        &mut releases,
                        &asked_for,
                        // **As the running application starts every run**:
                        // `Server Setup` M3's probe decides.
                        smoozy_server_setup::elevated::HowElevationIsObtained::APasswordIsOwedAndNoneIsHeld,
                        &mut telegram,
                        &mut the_tab,
                        &NobodyAsked,
                        None,
                        Some(InstallationCheck::Absent),
                    );
                    tokio::pin!(lending, set_up);
                    tokio::select! {
                        biased;
                        _ended = &mut lending => panic!("the connection ended under the run"),
                        came_to = &mut set_up => came_to,
                    }
                };
                let the_run_came_back = Instant::now();
                if std::env::var_os("SMOOZY_DESK_TELLS_EACH_ASK").is_some() {
                    eprintln!("{:?} the run came back: {came_to:?}", std::time::SystemTime::now());
                }
                // **Bounded**, because over the line a tear-down waits on the
                // far end's close through the relay, which proves nothing here.
                let _ = tokio::time::timeout(Duration::from_secs(10), connection.tear_down()).await;
                shut_down.shutdown("done".into());
                (came_to, the_run_came_back)
            });
            let (came_to, the_run_came_back) = came_to;
            if std::env::var_os("SMOOZY_DESK_TELLS_EACH_ASK").is_some() {
                eprintln!("{:?} the connection was let go", std::time::SystemTime::now());
            }
            // **The add lets its run go at its end**, as
            // `the_remembered_setup_is_carried_out` does.
            smoozy_desktop::setting_up::a_run_ends(server);
            let the_card_after = the_setup_under_way_on(server, || "Vast 4090".to_string());
            let server_output = the_servers_output_on(server, "Vast 4090");
            for mut child in children.lock().expect("the children").drain(..) {
                let _ = child.kill();
                let _ = child.wait();
            }
            let (answered_at, the_clock_began) = {
                let supervisor = supervisor.lock().expect("the supervisor");
                (
                    supervisor.answered_at.clone(),
                    supervisor.on_the_clock.as_ref().map(|(_, _, began, _)| *began),
                )
            };
            let left = WhatTheRealRunLeft {
                came_to,
                seen: seen.lock().expect("what the card showed").clone(),
                did: did.lock().expect("what it did").clone(),
                asked: asked.lock().expect("the asks").clone(),
                server_output,
                the_card_after,
                answered_at,
                the_clock_began,
                the_run_came_back,
            };
            left
        }
    }

    /// **`Server Setup` R1 item 3's install on a fresh Vast container**: the
    /// lists refreshed, then the unpack tool installed — each printing over
    /// seconds, as `apt-get` does there.
    fn the_tooling_install_prints_over_seconds() -> Vec<over_a_real_connection::ACommandThatPrints> {
        use over_a_real_connection::{ACommandThatPrints, Printed};
        use std::time::Duration;
        let a_moment = Duration::from_millis(700);
        vec![
            ACommandThatPrints {
                command: smoozy_server_setup::elevated::THE_LIST_UPDATE.elevated.as_str().to_string(),
                pieces: vec![
                    (a_moment, Printed::Output("Get:1 http://archive.ubuntu.com/ubuntu jammy InRelease [270 kB]\n")),
                    (a_moment, Printed::Output("Get:2 http://archive.ubuntu.com/ubuntu jammy-updates InRe")),
                    (a_moment, Printed::Output("lease [128 kB]\n")),
                    (a_moment, Printed::Complaint("debconf: delaying package configuration\n")),
                    (a_moment, Printed::Output("Fetched 21.4 MB in 3s (7,130 kB/s)\n")),
                    (a_moment, Printed::Output("Reading package lists...\n")),
                ],
            },
            ACommandThatPrints {
                command: smoozy_server_setup::elevated::THE_TOOLING_INSTALL.elevated.as_str().to_string(),
                pieces: vec![
                    (a_moment, Printed::Output("Reading package lists...\n")),
                    (a_moment, Printed::Output("Unpacking xz-utils (5.2.5-2ubuntu1) ...\n")),
                    (a_moment, Printed::Output("Setting up xz-utils (5.2.5-2ubuntu1) ...\n")),
                ],
            },
        ]
    }

    /// **A fresh Vast container's answers**: a container by both limbs, no
    /// unpack tool, and a driver that answers.
    fn a_vast_containers_script() -> Vec<(String, CommandResult)> {
        let mut script = the_driver_answers(a_machine_that_answers(), ended_cleanly("NVIDIA-SMI 580.105.08\n"));
        script.retain(|(named, _)| named != THE_CONTAINER_MARKER && named != THE_CONTAINER_TOOL);
        script.push((THE_CONTAINER_MARKER.to_string(), ended_cleanly("")));
        script.push((THE_CONTAINER_TOOL.to_string(), ended_cleanly("docker\n")));
        script
    }

    /// **The fourteenth notes, piece S: on a fresh container, what an admin
    /// step prints reaches the Set up card while it prints, and *Server
    /// output* holds it after the run.** The add's own setup, driven over a
    /// real SSH connection, with the unpack tool's install printing over
    /// seconds. The card is read each time the tab is told, as the tab asks.
    #[test]
    fn an_admin_step_on_a_container_shows_its_lines_on_the_card_while_it_prints() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("piece-s-the-window");
        let server = "piece-s-a-fresh-vast-container";
        let left = over_a_real_connection::a_container_is_added_and_set_up(
            &scratch,
            sftp_server,
            server,
            a_vast_containers_script(),
            the_tooling_install_prints_over_seconds(),
        );
        assert!(
            matches!(
                left.came_to,
                WhereTheSetUpGot::TheRunEnded {
                    ending: TheRunsEnding::ItReachedDone { .. },
                    ..
                }
            ),
            "the run did not reach done: {:?}\nasked: {:#?}",
            left.came_to,
            left.asked
        );
        let update = smoozy_server_setup::elevated::THE_LIST_UPDATE.elevated.as_str();
        let ended = left
            .did
            .iter()
            .find(|(command, what, _)| command == update && *what == "ended")
            .map(|(_, _, at)| *at)
            .unwrap_or_else(|| panic!("the list refresh never ran: {:#?}", left.asked));
        // **While it printed, the card showed its lines** — the newest three
        // of either stream, in the order they arrived, the complaint marked
        // as one and the line still being printed as it stands. Until
        // 2026-10-25 only complete lines of the ordinary stream, never the
        // complaint (the eighteenth notes, piece 2c).
        let while_it_printed: Vec<&over_a_real_connection::WhatTheCardShowed> =
            left.seen.iter().filter(|seen| seen.at < ended).collect();
        assert!(
            while_it_printed
                .iter()
                .any(|seen| seen.lines.first().map(String::as_str)
                    == Some("Get:1 http://archive.ubuntu.com/ubuntu jammy InRelease [270 kB]")),
            "the card never showed the first line while the command ran: {:#?}",
            left.seen
        );
        assert!(
            while_it_printed.iter().any(|seen| seen.lines.ends_with(&[
                "Get:1 http://archive.ubuntu.com/ubuntu jammy InRelease [270 kB]".to_string(),
                "Get:2 http://archive.ubuntu.com/ubuntu jammy-updates InRe".to_string(),
            ])),
            "the card never showed the line still being printed as it stood: {:#?}",
            left.seen
        );
        assert!(
            while_it_printed.iter().any(|seen| seen.lines
                == [
                    "Get:2 http://archive.ubuntu.com/ubuntu jammy-updates InRelease [128 kB]",
                    "debconf: delaying package configuration",
                    "Fetched 21.4 MB in 3s (7,130 kB/s)",
                ]
                && seen.errors == [false, true, false]),
            "the card's lines did not move as the command printed, both streams in order: {:#?}",
            left.seen
        );
        assert!(
            left.seen.iter().all(|seen| seen.lines.len() == seen.errors.len()
                && seen.lines.iter().zip(&seen.errors).all(|(line, error)| {
                    if line.contains("debconf") || line.starts_with("dpkg-query") {
                        *error
                    } else if line.starts_with("Get:") || line.starts_with("Fetched") || line.starts_with("Reading") {
                        !*error
                    } else {
                        true
                    }
                })),
            "a line was drawn as the wrong stream's: {:#?}",
            left.seen
        );
        // **And the lines stay while the steps after it run, printing
        // nothing** — `Design/Server Setup § A long step` as changed for the
        // fourteenth notes: *"kept until another step prints or the run
        // ends"*. Until piece S they went the moment the next step was marked,
        // so a container's setup showed a few seconds of an install at most.
        let install = smoozy_server_setup::elevated::THE_TOOLING_INSTALL.elevated.as_str();
        let installed = left
            .did
            .iter()
            .find(|(command, what, _)| command == install && *what == "ended")
            .map(|(_, _, at)| *at)
            .expect("the unpack tool's install ran");
        let afterwards: Vec<&over_a_real_connection::WhatTheCardShowed> = left
            .seen
            .iter()
            .filter(|seen| seen.at > installed && seen.because == "the run moved on")
            .collect();
        assert!(afterwards.len() >= 2, "the run moved on no further: {:#?}", left.seen);
        for seen in &afterwards {
            assert_eq!(
                seen.lines,
                [
                    "Reading package lists...",
                    "Unpacking xz-utils (5.2.5-2ubuntu1) ...",
                    "Setting up xz-utils (5.2.5-2ubuntu1) ...",
                ],
                "the install's lines went while {:?} ran: {:#?}",
                seen.under_way,
                left.seen
            );
        }
        // **The control stood beside the run throughout.**
        assert!(left.seen.iter().all(|seen| seen.server_output), "{:#?}", left.seen);
        // **After the run: no window, and *Server output* holds both
        // commands' words, the streams apart.**
        // **From 2026-10-26 an ended run answers its steps as it left them,
        // marked as ended, and never as a run still going** (the worldwide
        // pass's setup faults, fault 4); until then nothing was answered.
        assert!(
            left.the_card_after.as_ref().is_none_or(|panel| panel.the_run_has_ended && panel.cancel.is_none()),
            "the run's card outlived the run"
        );
        let output = left.server_output.expect("Server output after the run");
        let record = output.tabs[0].record.as_ref().map(|it| it.as_str()).unwrap_or_default();
        for printed in [
            "Get:1 http://archive.ubuntu.com/ubuntu jammy InRelease [270 kB]\n",
            "Reading package lists...\n",
            "Setting up xz-utils (5.2.5-2ubuntu1) ...\n",
        ] {
            assert!(record.contains(printed), "Server output lacks {printed:?}: {record:?}");
        }
        // **From 2026-10-24 every step's words are kept** (`Server Setup` Y7a
        // as changed, the sixteenth notes, note 1), so the readings' own
        // complaint — the package system saying the unpack tool is not there —
        // comes first, in the order printed. Until then this read the
        // install's complaint alone.
        assert_eq!(
            output.tabs[1].record.as_ref().map(|it| it.as_str()),
            Some(
                "dpkg-query: package 'xz-utils' is not installed and no information is \
                 available\ndebconf: delaying package configuration\n"
            ),
        );
    }

    /// **The download's lines, printed once a second on the machine's own
    /// clock** (`Server Setup` Y7b), as Smoozy's program on the server prints
    /// them: nine lines, the step reaching its boundary half a second after
    /// the last.
    fn the_download_prints_once_a_second() -> (String, Vec<(std::time::Duration, String)>, std::time::Duration) {
        use std::time::Duration;
        let mut lines: Vec<(Duration, String)> = (1..=8u64)
            .map(|second| {
                (
                    Duration::from_secs(second),
                    format!("blender-5.2.2-linux-x64.tar.xz: {} of 383 MB, 47.9 MB/s", second * 47),
                )
            })
            .collect();
        lines.push((
            Duration::from_millis(8_200),
            "blender-5.2.2-linux-x64.tar.xz: 383 MB downloaded in 8 s".to_string(),
        ));
        (asked::THE_FETCH_STEP.to_string(), lines, Duration::from_millis(8_700))
    }

    /// **How long after it printed each of the download's lines first stood
    /// on the card**, in order; `None` for a line the card never showed.
    fn how_late_each_line_reached_the_card(
        left: &over_a_real_connection::WhatTheRealRunLeft,
        printed: &[(std::time::Duration, String)],
    ) -> Vec<(String, Option<std::time::Duration>)> {
        let began = left.the_clock_began.expect("the download was commanded");
        printed
            .iter()
            .map(|(at, line)| {
                let printed_at = began + *at;
                let shown = left
                    .seen
                    .iter()
                    .filter(|seen| seen.lines.iter().any(|shown| shown == line))
                    .map(|seen| seen.at)
                    .min();
                (line.clone(), shown.map(|shown| shown.saturating_duration_since(printed_at)))
            })
            .collect()
    }

    /// **The worldwide pass's fault 1, timed at the desk: on a far line, the
    /// download's lines reach the Set up card about a second after the server
    /// prints them.** Added 2026-10-26, the worldwide pass's setup faults.
    ///
    /// **The far end** is this file's real SSH server, its commands answered by
    /// the stand-in supervisor, with a line between it and the desk that hands
    /// every byte on 300 ms after it arrived, each way — the far end of the
    /// brief's 250 to 350 ms. The add's own setup runs over it, the
    /// application's own order, on a fresh container. **The download prints a
    /// line once a second on the machine's own clock**, whether or not anyone
    /// reads it, as the machine's own program does. **The card is read each
    /// time the tab is told**, as the tab asks. Each line's lateness is the
    /// moment the card first showed it, less the moment it was printed.
    ///
    /// The pass on Vast watched the card change in batches every 3 to 12
    /// seconds on servers 109 to 340 ms away.
    #[test]
    fn worldwide_fault_1_on_a_far_line_the_downloads_lines_reach_the_card_about_a_second_after_they_print() {
        let _deadline = TheTestsDeadline::started_for(std::time::Duration::from_secs(900));
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("worldwide-fault-1");
        let server = "worldwide-fault-1-a-far-container";
        let clock = the_download_prints_once_a_second();
        let printed = clock.1.clone();
        let left = over_a_real_connection::a_container_is_added_and_set_up_over(
            &scratch,
            sftp_server,
            server,
            a_vast_containers_script(),
            the_tooling_install_prints_over_seconds(),
            over_a_real_connection::TheLineAndTheClock {
                late: std::time::Duration::from_millis(300),
                words_on_a_clock: Some(clock),
            },
        );
        let began = left.the_clock_began.expect("the download was commanded");
        for (name, at) in &left.answered_at {
            println!("{:>8.2} s  {name}", at.saturating_duration_since(began).as_secs_f64() - 0.0);
        }
        let late = how_late_each_line_reached_the_card(&left, &printed);
        for (line, after) in &late {
            println!("{after:>12?}  {line}");
        }
        assert!(
            matches!(
                left.came_to,
                WhereTheSetUpGot::TheRunEnded {
                    ending: TheRunsEnding::ItReachedDone { .. },
                    ..
                }
            ),
            "the run did not reach done: {:?}",
            left.came_to
        );
        // **The download's figure stood while it ran, and never once it had
        // finished**: no reading of the card more than two seconds after the
        // download's end carries it.
        let the_download_ended = began + std::time::Duration::from_millis(8_700);
        assert!(
            left.seen.iter().any(|seen| seen.at < the_download_ended && seen.a_figure_stood),
            "the download's figure never stood: {:#?}",
            left.seen
        );
        let stood_after: Vec<std::time::Duration> = left
            .seen
            .iter()
            .filter(|seen| seen.at > the_download_ended && seen.a_figure_stood)
            .map(|seen| seen.at.saturating_duration_since(the_download_ended))
            .collect();
        println!("the figure stood after the download ended at {stood_after:?}");
        assert!(
            stood_after.iter().all(|after| *after <= std::time::Duration::from_secs(2)),
            "the download's figure stood after the download had finished: {stood_after:?}"
        );
        let too_late: Vec<&(String, Option<std::time::Duration>)> = late
            .iter()
            .filter(|(_, after)| after.is_none_or(|after| after > std::time::Duration::from_millis(2_000)))
            .collect();
        assert!(
            too_late.is_empty(),
            "lines reached the card more than two seconds after they printed, or never: {too_late:#?}"
        );
    }

    /// **The worldwide pass's fault 4, timed at the desk on the same far
    /// line: the last step is marked *done* the moment it is, and the card's
    /// steps are not held behind readings taken twice.** Added 2026-10-26.
    ///
    /// On every server of the pass the card still showed the running list for
    /// 27 to 36 seconds after the header read *Idle*, its last step, *Run
    /// Smoozy's final check*, never marked *done*; and the unpack tool's
    /// install printed its first line 18 to 30 seconds after the download said
    /// *done*. **Here**: the last panel the run told the card marks every step
    /// *done*; the panel answered once the run has ended is marked as ended,
    /// with the same; the unpack tool's install begins within eight seconds of
    /// the download's end; and the run comes back within three seconds of its
    /// last write.
    #[test]
    fn worldwide_fault_4_on_a_far_line_the_last_step_reads_done_at_once_and_no_step_waits_on_a_second_reading() {
        let _deadline = TheTestsDeadline::started_for(std::time::Duration::from_secs(900));
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("worldwide-fault-4");
        let server = "worldwide-fault-4-a-far-container";
        let clock = the_download_prints_once_a_second();
        let ends_after = clock.2;
        let left = over_a_real_connection::a_container_is_added_and_set_up_over(
            &scratch,
            sftp_server,
            server,
            a_vast_containers_script(),
            the_tooling_install_prints_over_seconds(),
            over_a_real_connection::TheLineAndTheClock {
                late: std::time::Duration::from_millis(300),
                words_on_a_clock: Some(clock),
            },
        );
        assert!(
            matches!(
                left.came_to,
                WhereTheSetUpGot::TheRunEnded {
                    ending: TheRunsEnding::ItReachedDone { .. },
                    ..
                }
            ),
            "the run did not reach done: {:?}",
            left.came_to
        );
        let began = left.the_clock_began.expect("the download was commanded");
        let the_download_ended = began + ends_after;
        let update = smoozy_server_setup::elevated::THE_LIST_UPDATE.elevated.as_str();
        let the_install_began = left
            .did
            .iter()
            .find(|(command, what, _)| command == update && *what == "began")
            .map(|(_, _, at)| *at)
            .expect("the unpack tool's install ran");
        let the_last_write = left
            .answered_at
            .iter()
            .rev()
            .find(|(name, _)| name == asked::THE_BOUNDARY_IS_WRITTEN_REQUEST)
            .map(|(_, at)| *at)
            .expect("done was written");
        let the_last_telling = left
            .seen
            .iter()
            .rev()
            .find(|seen| seen.because == "the run moved on")
            .expect("the card was told the run");
        println!(
            "the install began {:.2} s after the download ended; the run came back {:.2} s after its last write; \
             the last telling marked {:?}; afterwards the card was answered {:?}",
            the_install_began.saturating_duration_since(the_download_ended).as_secs_f64(),
            left.the_run_came_back.saturating_duration_since(the_last_write).as_secs_f64(),
            the_last_telling.marks,
            left.the_card_after.as_ref().map(|panel| (
                panel.the_run_has_ended,
                panel.steps.iter().map(|step| step.mark.as_str().to_string()).collect::<Vec<_>>()
            )),
        );
        let mut missed = Vec::new();
        if !the_last_telling.marks.iter().all(|(_, mark)| mark == "done") {
            missed.push(format!("the last telling did not mark every step done: {:?}", the_last_telling.marks));
        }
        match &left.the_card_after {
            Some(panel) if panel.the_run_has_ended
                && !panel.steps.is_empty()
                && panel.steps.iter().all(|step| step.mark.as_str() == "done") => {}
            other => missed.push(format!("the ended run's steps were not answered as it left them: {other:?}")),
        }
        if the_install_began.saturating_duration_since(the_download_ended) > std::time::Duration::from_secs(8) {
            missed.push(format!(
                "the unpack tool's install began {:?} after the download ended",
                the_install_began.saturating_duration_since(the_download_ended)
            ));
        }
        if left.the_run_came_back.saturating_duration_since(the_last_write) > std::time::Duration::from_secs(3) {
            missed.push(format!(
                "the run came back {:?} after its last write",
                left.the_run_came_back.saturating_duration_since(the_last_write)
            ));
        }
        assert!(missed.is_empty(), "{}", missed.join("\n"));
    }

    /// **The sixteenth notes, note 1, over a real connection: a container run
    /// with no admin step shows the readings' lines on the card while Blender
    /// downloads.** The add's own setup, driven over a real SSH connection, on
    /// a container whose image already holds the unpack tool, so nothing
    /// elevated runs at all. **Until 2026-10-24 the card's window stayed empty
    /// for the whole of such a run** — the Creator's Vast container.
    #[test]
    fn a_container_run_with_no_admin_step_shows_the_readings_lines_on_the_card_during_the_download() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let scratch = Scratch::new("y7a-the-readings-on-the-card");
        let server = "y7a-a-container-that-needs-nothing";
        let mut script = a_vast_containers_script();
        script.retain(|(named, _)| named != THE_UNPACK_TOOLS_PROBE);
        script.push((
            THE_UNPACK_TOOLS_PROBE.to_string(),
            ended_cleanly(
                "Package: xz-utils\nStatus: install ok installed\nPriority: standard\nVersion: 5.6.1+really5.4.5-1ubuntu0.3\n",
            ),
        ));
        let left = over_a_real_connection::a_container_is_added_and_set_up(&scratch, sftp_server, server, script, Vec::new());
        assert!(
            matches!(left.came_to, WhereTheSetUpGot::TheRunEnded { ending: TheRunsEnding::ItReachedDone { .. }, .. }),
            "the run did not reach done: {:?}\nasked: {:#?}",
            left.came_to,
            left.asked
        );
        assert!(
            !left.asked.iter().any(|command| command.contains("apt-get")),
            "an admin step ran: {:#?}",
            left.asked
        );
        let during_the_download: Vec<&over_a_real_connection::WhatTheCardShowed> = left
            .seen
            .iter()
            .filter(|seen| seen.under_way.as_deref().is_some_and(|name| name.starts_with("Download")))
            .collect();
        assert!(!during_the_download.is_empty(), "the card was never read during the download: {:#?}", left.seen);
        for seen in &during_the_download {
            assert_eq!(seen.lines.len(), 3, "the window was not full of the readings' lines: {:#?}", left.seen);
            assert_eq!(
                seen.lines.last().map(String::as_str),
                Some("Version: 5.6.1+really5.4.5-1ubuntu0.3"),
                "the readings' newest line was not in the window: {:#?}",
                left.seen
            );
            assert!(seen.server_output, "Server output was not offered");
        }
        // **And *Server output* holds the readings' words after the run.**
        let output = left.server_output.expect("Server output after the run");
        let record = output.tabs[0].record.as_ref().map(|it| it.as_str()).unwrap_or_default();
        assert!(record.contains("NVIDIA-SMI 580.105.08\n"), "{record:?}");
        assert!(record.contains("Status: install ok installed\n"), "{record:?}");
    }

    // ── The twenty-third notes, note 3: *Server output* is kept until the next
    // act the user asked for (`Server Setup` Y7a as changed 2026-10-30) ──

    /// **A piece of ordinary output, handed to a words route** the way a step
    /// hands it.
    fn twenty_third_printed(go: &smoozy_desktop::setting_up::TheWordsGoHere, step: smoozy_server_setup::ready_action::Step, words: &str) {
        go(
            step,
            &smoozy_desktop::setting_up::WhatTheServerPrinted::APiece(
                smoozy_desktop::servers::sessions::APiecePrinted::Output(words.as_bytes().to_vec()),
            ),
        );
    }

    /// **(a) An install pressed while a setup runs waits its turn, and when it
    /// begins the setup's words stay**, each act's under its own steps — the
    /// line's own calls, in the order the Set up tab makes them.
    #[test]
    fn twenty_third_note_3_a_the_next_act_in_the_line_keeps_the_setups_words() {
        use smoozy_desktop::setting_up::{self as s, ASetUpAct, HowThePressWasTaken, WhatFollowsARun};
        use smoozy_server_setup::ready_action::Step;
        let server = "twenty-third-note-3-a";
        let first = s::the_acts_are_taken_from_the_card(
            server,
            vec![ASetUpAct::TheReadyAction { a_render_was_running: false }],
            &mut |_| {},
        );
        assert_eq!(first, HowThePressWasTaken::ItBegan);
        let go = s::the_words_go_to_the_card(server, || {});
        twenty_third_printed(&go, Step::Readings, "the setup read the machine\n");
        let pressed = s::the_acts_are_taken_from_the_card(
            server,
            vec![ASetUpAct::AnInstallFromTheList("5.2.2".to_string())],
            &mut |_| {},
        );
        assert_eq!(pressed, HowThePressWasTaken::ItWaitsItsTurn);
        twenty_third_printed(&go, Step::Done, "the setup is done\n");
        assert!(matches!(s::a_run_in_the_line_ends(server, true), WhatFollowsARun::TheNextBegins(_)));
        // **The install's run asks where its words go**, as `driven` does.
        let go = s::the_words_go_to_the_card(server, || {});
        twenty_third_printed(&go, Step::Readings, "the install read the machine\n");
        let (ordinary, _) = s::the_whole_of_what_was_printed_on(server).expect("the words are kept");
        s::a_run_in_the_line_ends(server, false);
        assert!(ordinary.contains("the setup read the machine\n"), "the setup's words went: {ordinary:?}");
        assert!(ordinary.contains("the setup is done\n"), "the setup's words went: {ordinary:?}");
        assert!(ordinary.contains("the install read the machine\n"), "{ordinary:?}");
        let steps = s::the_words_of_each_step_on(server).expect("kept");
        assert_eq!(
            steps.iter().map(|one| one.step).collect::<Vec<_>>(),
            vec![Step::Readings, Step::Done, Step::Readings],
            "each act's words are not under its own steps"
        );
        // **And the next act the user asks for, with nothing standing, begins
        // its words afresh** (`Server Setup` Y7a: the words are kept until then).
        let again = s::the_acts_are_taken_from_the_card(
            server,
            vec![ASetUpAct::TheTelegramPlacement],
            &mut |_| {},
        );
        assert_eq!(again, HowThePressWasTaken::ItBegan);
        s::a_run_in_the_line_ends(server, false);
        assert_eq!(s::the_words_of_each_step_on(server).map(|steps| steps.len()), Some(0));
    }

    /// **(b) A run cut mid-step and taken up again adds to the words it had**:
    /// the run taken up asks where its words go again, as `driven` does when
    /// none was handed, and nothing printed before the cut goes.
    #[test]
    fn twenty_third_note_3_b_a_run_taken_up_again_keeps_the_words_before_the_cut() {
        use smoozy_desktop::setting_up as s;
        use smoozy_server_setup::ready_action::Step;
        let server = "twenty-third-note-3-b";
        assert!(s::a_run_begins(server, smoozy_server_setup::run::SetupKind::TheReadyAction));
        let go = s::the_words_go_to_the_card(server, || {});
        twenty_third_printed(&go, Step::Libraries, "Reading package lists... 40%");
        // **The cut**: the connection drops, and the run is taken up again.
        drop(go);
        let go = s::the_words_go_to_the_card(server, || {});
        twenty_third_printed(&go, Step::Libraries, " 100%\nSetting up xz-utils ...\n");
        let (ordinary, _) = s::the_whole_of_what_was_printed_on(server).expect("the words are kept");
        s::a_run_ends(server);
        assert_eq!(
            ordinary, "Reading package lists... 40% 100%\nSetting up xz-utils ...\n",
            "the words printed before the cut went"
        );
    }

    /// **(c) An owed driver swap after a run keeps its own words apart and
    /// never empties the setup's** — the swap run driven on the stand-in
    /// container with the Set up tab's words route.
    #[test]
    fn twenty_third_note_3_c_an_owed_swap_after_a_run_keeps_the_setups_words() {
        use smoozy_desktop::servers::the_driver_swap as swap;
        use smoozy_desktop::setting_up as s;
        use smoozy_server_setup::ready_action::Step;
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        let server = Identity::issue(IdentityKind::LocalServer).expect("a server").as_text();
        assert!(s::a_run_begins(&server, smoozy_server_setup::run::SetupKind::TheReadyAction));
        let go = s::the_words_go_to_the_card(&server, || {});
        twenty_third_printed(&go, Step::Done, "the setup's own words\n");
        s::a_run_ends(&server);
        swap::a_run_whose_item_5_may_compare_begins(&server);
        let scratch = Scratch::new("twenty-third-note-3-c");
        let left = the_restart_is_driven_in_full(
            &scratch,
            sftp_server,
            AfterTheRestart::TheMachineAnswers,
            a_machine_whose_driver_answers_at_550(),
            TheRestartCommand::AnswersAndThenDrops,
            None,
            AndBeside {
                found: vec!["5.2.2"],
                the_server: Some(server.clone()),
                as_the_swap_run: true,
                ..a_swap_that_reaches_the_bar(Vec::new(), None)
            },
        );
        assert_eq!(it_reached_done_and_missed(&left.came_to), Some(false), "{:?}", left.came_to);
        let (ordinary, _) = s::the_whole_of_what_was_printed_on(&server).expect("the setup's words are kept");
        assert_eq!(ordinary, "the setup's own words\n", "the swap emptied or joined the setup's words");
        // **And the swap's own words are kept apart.**
        let (swaps, _) = s::the_whole_of_what_the_swap_printed_on(&server).expect("the swap's words are kept");
        assert!(swaps.contains("Driver/library version mismatch"), "{swaps:?}");
    }
}
