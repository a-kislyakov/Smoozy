//! **§1.2 and §1.5 performed** — the render started by the worker as
//! Blender's own process, its recorded pair taken at the moment it starts,
//! both its streams held, and the binary asked to name its own version
//! before any of it.
//!
//! # L5, and why there is no shell and no wrapper
//!
//! **Whatever sits between the worker and Blender replaces itself with
//! Blender, so the recorded render process is Blender's own process.**
//! Observed at the promise level: the execution group is the worker, its
//! child is the recorded target — process name `blender`, 44 threads on the
//! virtual machine — and *"the launch material `exec`s itself away so
//! nothing sits between them"*; under all three signals the signal went to
//! the recorded render process every time (`The last two boxes`, Fact 2).
//!
//! **Here that is kept by there being nothing to `exec` away.** The start
//! below names the build's own path and hands it the words the line
//! composed, so the child the kernel creates **is** Blender: there is no
//! shell, no wrapper script, no `sh -c`, and no command string anywhere in
//! this file. **A shell would have been the thing that had to `exec` itself
//! away**, and the observed arrangement is the one where nothing has to.
//! `Server protocol and transfers` §1.1's rule — no value of user, provider
//! or machine origin is ever interpolated into a command string — is
//! therefore kept by construction: every word crosses as an argument.
//!
//! # L6, and the arrangement this file deliberately does not add
//!
//! **The render survives the connection, the supervisor and Smoozy by
//! construction, not by flag.** The observed negative: a render *not*
//! detached died the instant its connection dropped — *"the process ended at
//! once"* under the hang-up signal a dropped connection sends, with the
//! frame in flight lost entirely — while a render started detached survived
//! (`The rented machine`).
//!
//! **What makes it detached is the worker's own detachment, and this file
//! adds no third arrangement.** `Rented-server runtime` W6 requires the
//! worker to outlive the supervisor's own exit; `Rented-server runtime` K18
//! keeps it on a systemd machine in the unit's own kill behaviour, and says
//! outright that nothing equivalent is owed on a container, the orphaning
//! being the machine's default. The render is the worker's child, so it
//! inherits that. **`Rented-server runtime`'s own worker start already
//! refuses to add anything here for exactly this reason**, and this start
//! follows it rather than inventing a second rule: a `setsid` of this
//! module's own would be a third arrangement beside those two, in the one
//! place a mistake is a render nobody can stop.
//!
//! **What this file does keep is *held by nothing the connection owns*.**
//! The child's standard input is closed rather than inherited, both output
//! streams are pipes this worker holds, and no other descriptor crosses:
//! every descriptor this process opens is close-on-exec by the language's
//! own rule, so nothing of a connection's reaches the render.
//!
//! # L7 — both streams, always
//!
//! Nothing merges them. In 4.5 the device line's route writes to the
//! complaints stream, which also carries the failure text the user is shown,
//! while from 5.0 it is on the ordinary output; and which stream a message
//! lands on is not fixed even without the ask. **So both streams are
//! captured, always**, and the start below hands back both as pipes. What is
//! done with them is [`crate::acting::copy_loop`]'s.
//!
//! # L14 — the version, asked of the binary and never of a path
//!
//! **The confirmation is never a read of a path, a folder name or an
//! installation record**, and the thing confirmed is the binary about to be
//! launched. [`TheShortInvocationOnTheMachine`] is the ask: one headless run
//! of that binary, printing its own version between `Job files and outputs`
//! R1's two sentinels, with the failure-to-exit-status switch on the line so
//! a raise is a status rather than a silent zero.
//!
//! **Fact 105 is permanently unresolved and nothing here leaves a space for
//! it** (§9 item 16). Whether `Job files and outputs` R1's reading carries
//! the running Blender's own version identifier among its payloads is not
//! established, so this ask runs on every attempt; where a reading is ever
//! shown to carry it, the answer arrives at
//! [`crate::versions::confirm_the_version`] through the same argument and
//! nothing here moves.
//!
//! **Markers used in this file that belong to another document, each with
//! the document it belongs to.** `Rented-server runtime` M3, W6, X1, X6,
//! X12. `Job files and outputs` D7, R1. `Server protocol and transfers`
//! §1.1.
//!
//! **`Rented-server runtime` K18 is deliberately not on that list and is
//! written out at every site instead**, because this plan defines a K18 of
//! its own.

use std::io::Read;
use std::process::{Child, ChildStderr, ChildStdout, Command, Stdio};

use smoozy_job_files_machine::reading::{THE_CLOSING_SENTINEL, THE_OPENING_SENTINEL};
use smoozy_runtime::execution_group::{RecordedPair, ValidatesTheRecordedPair};
use smoozy_server_setup_machine::blender::Version;

use crate::line::LaunchLine;
use crate::spellings::{
    Spelling, FAILURE_TO_EXIT_STATUS_SWITCH, LAUNCH_MATERIAL_ARGUMENT, NO_WINDOW_MODE,
    THE_FAILURE_STATUS,
};

/// **L4's invocation, as a value the supervisor's own worker start is
/// handed.**
///
/// `Rented-server runtime`'s `SaysHowThisWorkerIsInvoked` asks the document
/// that owns a kind what its worker is, and answers `None` for a kind whose
/// document has named none — *"nothing is guessed at from the kind"*. **This
/// is what that answer is for the render kind, and it is a shape rather than
/// a program.**
///
/// **The program for the rented machine exists**: `smoozy-server`, at
/// `App/Rented-server runtime — program/`. It refused every one of the seven
/// kinds by name until step 10c piece 5. *(Corrected 2026-09-19: this also said
/// the program could not reach this crate. It has taken this crate since step
/// 10c piece 3, which is what the crate was split out for. **Corrected again
/// the same day by step 10c piece 5's second sitting: it starts a worker of
/// this kind**, with its own path and the kind's word as the invocation, and
/// the worker asks the supervisor what it was started for rather than being
/// handed it — so this shape is not what it is started with.)* So this
/// names the two things that would be handed over the moment one is started —
/// the package's own executable, by path, and the job and attempt the worker is
/// for — and **no program is invented here**. *(Corrected 2026-09-07: this said
/// no such program existed anywhere in the workspace, which stopped being true
/// at step 8.)*
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HowTheRenderWorkerIsInvoked {
    /// The server package's own executable, by path in the package area.
    /// **Never a name to be searched for** (`Rented-server runtime` X6).
    pub the_packages_own_executable: String,
    /// What the worker is for, as arguments rather than as a line.
    pub arguments: Vec<String>,
}

/// **The launch, as the machine is handed it: a build's own path and the
/// words the line composed.**
///
/// **The words are [`LaunchLine::as_words`]'s and are not re-ordered,
/// filtered or added to here.** L8's order is the type's; this carries it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheLaunch {
    /// The build the manifest names, by path — a version alone cannot
    /// identify a build (`Job files and outputs` R1).
    pub the_blender_build_path: String,
    /// Every word after the program's own path.
    pub words: Vec<String>,
}

impl TheLaunch {
    /// **The launch, from the line and the build's path.** The one route
    /// from a [`LaunchLine`] to something that can be started.
    pub fn from(
        the_blender_build_path: &str,
        line: &LaunchLine,
    ) -> Result<TheLaunch, crate::spellings::MissingSpelling> {
        Ok(TheLaunch {
            the_blender_build_path: the_blender_build_path.to_string(),
            words: line.as_words()?,
        })
    }
}

/// A render that has been started: the pair X1 records, and the two streams
/// the worker holds.
///
/// **The child is held rather than waited on**, which is `Rented-server
/// runtime` X12's *the worker remains until its external processes have
/// exited* kept by the type: dropping this without reaping would leave the
/// ending unread.
#[derive(Debug)]
pub struct TheRenderAsItWasStarted {
    /// **X1's pair, recorded at the moment the render was started** — the
    /// identifier with the kernel-set start time, never the identifier
    /// alone. **The stop of §3.2 is aimed at this and at nothing found
    /// later** (`Rented-server runtime` X12).
    pub recorded: RecordedPair,
    /// The ordinary output, as a pipe the worker holds (L7).
    pub ordinary: ChildStdout,
    /// The complaints stream, as a pipe the worker holds (L7).
    pub complaints: ChildStderr,
    /// The child, for the ending step's own reap (§2 P1 step 9).
    pub child: Child,
}

/// Starting the render, behind a trait — **the one place in this crate that
/// starts a process on the machine.**
pub trait StartsTheRender {
    /// **L4, L5, L6, L7 in one act**, because they are one act: the child
    /// is Blender, its pair is recorded as it starts, and both its streams
    /// come back held.
    fn start(&self, launch: &TheLaunch) -> Result<TheRenderAsItWasStarted, String>;
}

/// The real end, on the machine.
///
/// **The kernel's own reading of a pair is `Rented-server runtime`'s and is
/// taken whole**, so there is one reading of a start time in the workspace
/// rather than two — and it arrives through that crate's own
/// [`ValidatesTheRecordedPair`] rather than through a second shape. On the
/// machine it is
/// [`smoozy_runtime::acting::execution_group::ThePairAsTheKernelReportsIt`],
/// whose per-process records are the kernel's; **taking it as a parameter is
/// what lets the whole start be run at a desk whose kernel publishes them
/// somewhere else, or nowhere.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheRenderStartedOnTheMachine<V> {
    pub kernel: V,
}

impl
    TheRenderStartedOnTheMachine<
        smoozy_runtime::acting::execution_group::ThePairAsTheKernelReportsIt,
    >
{
    /// The machine Smoozy rents.
    pub fn on_the_rented_machine() -> Self {
        TheRenderStartedOnTheMachine {
            kernel:
                smoozy_runtime::acting::execution_group::ThePairAsTheKernelReportsIt::the_rented_machine(),
        }
    }
}

impl<V: ValidatesTheRecordedPair> StartsTheRender for TheRenderStartedOnTheMachine<V> {
    fn start(&self, launch: &TheLaunch) -> Result<TheRenderAsItWasStarted, String> {
        let mut child = Command::new(&launch.the_blender_build_path)
            .args(&launch.words)
            // **Held by nothing the connection owns** (L6). Standard input
            // is closed rather than inherited: a render that read from a
            // connection's input would be a render the connection holds.
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|why| {
                format!(
                    "the render could not be started: {} could not be run: {why}",
                    launch.the_blender_build_path
                )
            })?;
        let identifier = child.id();
        // **X1's second half comes from the kernel and never from this
        // process**, and Fact 91 is why: the start time is written by the
        // kernel at fork and cannot be chosen by the process it describes.
        //
        // **A reading that does not answer is a started render with no
        // recorded group**, which is the one state the stop cannot reach —
        // so the render is ended here rather than left running unrecorded on
        // a machine the user pays for by the second. That is not the
        // interrupt of §3.2, which is a stop of a *recorded* render; it is
        // the undoing of a start that never completed.
        let at_the_moment_it_started = match self.kernel.what_is_at(identifier) {
            Ok(Some(pair)) => pair,
            other => {
                let why = match other {
                    Err(why) => format!("the kernel's own record could not be read: {why}"),
                    _ => "the kernel recorded nothing for it".to_string(),
                };
                let _ = child.kill();
                let _ = child.wait();
                return Err(format!(
                    "the render started at {identifier} and {why}, so no execution group could \
                     be recorded and no stop could ever reach it; the start was undone"
                ));
            }
        };
        let ordinary = child
            .stdout
            .take()
            .ok_or_else(|| "the render's ordinary output was not held".to_string())?;
        let complaints = child
            .stderr
            .take()
            .ok_or_else(|| "the render's complaints stream was not held".to_string())?;
        Ok(TheRenderAsItWasStarted {
            recorded: at_the_moment_it_started,
            ordinary,
            complaints,
            child,
        })
    }
}

// ---------------------------------------------------------------------------
// L14 — the version confirmation's ask
// ---------------------------------------------------------------------------

/// **The property the ask reads, quoted from Blender's own publication.**
///
/// Blender's bundled Python API reference, `api/bpy.app.rst`, read
/// **2026-09-04**: *"version — The Blender version as a tuple of 3 numbers
/// (major, minor, micro). eg. (4, 3, 1)"*, typed
/// `tuple[int, int, int]`. **Three components, which is exactly what
/// `Server Setup` B2's [`Version`] parses**, and the reason that document
/// compares versions as components and never as strings.
pub const THE_PROPERTY_THE_ASK_READS: &str = "bpy.app.version";

/// **The script the short invocation runs.**
///
/// It prints the three numbers between `Job files and outputs` R1's own two
/// sentinels, because Blender's own chatter shares the stream the answer
/// comes back on — the same reason that document gives for the sentinels,
/// and the same pair of marks, so there is one convention rather than two.
///
/// **It is Smoozy's own text at a path Smoozy chose**, so nothing of the
/// machine's or the user's is written into it.
pub fn the_asks_script() -> String {
    format!(
        "import bpy\nimport sys\nsys.stdout.write('{}%d.%d.%d{}' % {})\n",
        String::from_utf8_lossy(THE_OPENING_SENTINEL),
        String::from_utf8_lossy(THE_CLOSING_SENTINEL),
        THE_PROPERTY_THE_ASK_READS,
    )
}

/// **The answer, read out from between the sentinels.**
///
/// **Nothing is guessed from a stream that does not carry both marks**, and
/// nothing is guessed from one that carries something else between them —
/// which is `Job files and outputs` C9's discipline, that a reading which
/// did not complete is not a reading. Both come back as `None`, which
/// [`crate::versions::confirm_the_version`] refuses as *the binary did not
/// answer* rather than reading as a match.
pub fn the_version_in(stream: &[u8]) -> Option<Version> {
    let opens = find(stream, THE_OPENING_SENTINEL)? + THE_OPENING_SENTINEL.len();
    let closes = find(&stream[opens..], THE_CLOSING_SENTINEL)?;
    Version::parse(std::str::from_utf8(&stream[opens..opens + closes]).ok()?)
}

fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

/// Asking a Blender to name its own version, behind a trait.
pub trait AsksABlenderToNameItsVersion {
    /// **L14's ask.** `None` where the binary did not answer with a version
    /// — a silence, a stream with no marks, or a run that never started.
    /// **A silence is never read as a match.**
    fn what_version_it_names(&self, the_blender_build_path: &str) -> Option<Version>;
}

/// The real end: one short headless invocation of the binary about to be
/// launched.
///
/// **The line it composes is three of this contract's own arguments and
/// nothing else** — the no-window mode (L3), the failure-to-exit-status
/// switch (L8a), and the script argument (L8 item 7) — each taken from
/// [`crate::spellings`] rather than written out here, so the ask and the
/// render name the same switches or neither does.
///
/// **No factory-startup switch** (L9), and the reason reaches here too: the
/// contract names none anywhere, and a version is not a thing the machine's
/// saved preferences hold.
///
/// **No bare double dash and nothing after one.** Past that dash the line is
/// Cycles' and Blender reads nothing of its own there (L8's third edge), and
/// this invocation has nothing to say to Cycles.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheShortInvocationOnTheMachine {
    /// Where the ask's script is written. **Smoozy's own path** — it carries
    /// no value of the machine's or the user's.
    pub the_scripts_path: String,
}

impl TheShortInvocationOnTheMachine {
    /// The words after the program's own path, from the spellings.
    pub fn words(&self) -> Result<Vec<String>, crate::spellings::MissingSpelling> {
        fn text(spelling: Spelling) -> Result<String, crate::spellings::MissingSpelling> {
            spelling.text().map(str::to_string)
        }
        Ok(vec![
            text(NO_WINDOW_MODE)?,
            text(FAILURE_TO_EXIT_STATUS_SWITCH)?,
            THE_FAILURE_STATUS.to_string(),
            text(LAUNCH_MATERIAL_ARGUMENT)?,
            self.the_scripts_path.clone(),
        ])
    }
}

impl AsksABlenderToNameItsVersion for TheShortInvocationOnTheMachine {
    fn what_version_it_names(&self, the_blender_build_path: &str) -> Option<Version> {
        std::fs::write(&self.the_scripts_path, the_asks_script()).ok()?;
        let mut started = Command::new(the_blender_build_path)
            .args(self.words().ok()?)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .ok()?;
        let mut answered = Vec::new();
        started.stdout.take()?.read_to_end(&mut answered).ok()?;
        // **The ending is reaped rather than left**, so the ask leaves no
        // process behind whatever it answers.
        let _ = started.wait();
        the_version_in(&answered)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::versions::{confirm_the_version, VersionRefused};

    /// A kernel that answers, so the whole start can be exercised at a desk
    /// whose own kernel publishes its records somewhere this reading cannot
    /// reach. **It stands in for the kernel and never for the judgement**:
    /// what is done with a pair, and with an absent one, is the code above.
    #[derive(Debug, Clone, Copy)]
    pub(crate) struct AKernelThat(pub(crate) Option<u64>);

    impl ValidatesTheRecordedPair for AKernelThat {
        fn what_is_at(&self, identifier: u32) -> Result<Option<RecordedPair>, String> {
            Ok(self.0.map(|start_time| RecordedPair {
                identifier,
                start_time,
            }))
        }
    }

    #[test]
    fn the_launch_carries_the_lines_own_words_in_the_lines_own_order() {
        let line = crate::line::LaunchLine::compose(crate::line::tests::inputs(None))
            .expect("a composed line");
        let launch = TheLaunch::from("/opt/blender-5.2/blender", &line).expect("every spelling");
        assert_eq!(launch.words, line.as_words().expect("every spelling"));
        // **Nothing is added and nothing is re-ordered**: the words are the
        // line's, and the line is the only thing that can order them.
        assert_eq!(launch.the_blender_build_path, "/opt/blender-5.2/blender");
    }

    #[test]
    fn the_asks_script_prints_blenders_own_property_between_the_reading_machinerys_sentinels() {
        let script = the_asks_script();
        assert!(script.contains(THE_PROPERTY_THE_ASK_READS));
        assert!(script.contains(&String::from_utf8_lossy(THE_OPENING_SENTINEL).to_string()));
        assert!(script.contains(&String::from_utf8_lossy(THE_CLOSING_SENTINEL).to_string()));
    }

    #[test]
    fn a_version_is_read_only_from_between_both_marks() {
        let both = b"chatter<<<SMOOZY-READING>>>5.2.0<<<END-SMOOZY-READING>>>more".to_vec();
        assert_eq!(
            the_version_in(&both),
            Some(Version {
                major: 5,
                minor: 2,
                patch: 0
            })
        );
        // One mark alone, and a stream cut short before the closing mark:
        // no reading, never an empty one.
        assert_eq!(the_version_in(b"<<<SMOOZY-READING>>>5.2.0"), None);
        assert_eq!(the_version_in(b"5.2.0"), None);
        // Both marks with something that is not a version between them.
        assert_eq!(
            the_version_in(b"<<<SMOOZY-READING>>>blender<<<END-SMOOZY-READING>>>"),
            None
        );
    }

    #[test]
    fn a_silence_from_the_binary_fails_the_attempt_rather_than_reading_as_a_match() {
        let named = Version {
            major: 5,
            minor: 2,
            patch: 0,
        };
        assert_eq!(
            confirm_the_version(named, the_version_in(b"nothing of the kind")),
            Err(VersionRefused::TheBinaryDidNotAnswer {
                the_manifest_names: named
            })
        );
    }

    /// **The ask's line carries the two process options and the script, and
    /// no bare double dash.** A dash here would put the script argument
    /// where Blender reads nothing of its own.
    #[test]
    fn the_asks_line_is_three_arguments_and_carries_no_bare_double_dash() {
        let words = TheShortInvocationOnTheMachine {
            the_scripts_path: "/var/lib/smoozy/ask-the-version.py".to_string(),
        }
        .words()
        .expect("every spelling");
        assert_eq!(
            words,
            vec![
                "--background".to_string(),
                "--python-exit-code".to_string(),
                "63".to_string(),
                "--python".to_string(),
                "/var/lib/smoozy/ask-the-version.py".to_string(),
            ]
        );
        assert!(!words
            .iter()
            .any(|word| word == crate::spellings::THE_BARE_DOUBLE_DASH));
        // **No factory-startup switch anywhere** (L9).
        assert!(!words.iter().any(|word| word.contains("factory")));
    }

    /// A program that prints on both streams and ends at a status of its
    /// own, run where a build's path goes.
    pub(crate) fn a_program_that_prints_on_both_streams() -> TheLaunch {
        TheLaunch {
            the_blender_build_path: "/bin/sh".to_string(),
            words: vec![
                "-c".to_string(),
                "printf 'ordinary\\n'; printf 'complaints\\n' >&2; exit 7".to_string(),
            ],
        }
    }

    /// **The start is run for real at the desk**, with its pair recorded and
    /// both streams held.
    #[test]
    fn a_render_is_started_as_its_own_process_with_its_pair_recorded_and_both_streams_held() {
        let start = TheRenderStartedOnTheMachine {
            kernel: AKernelThat(Some(19186)),
        };
        let mut started = start
            .start(&a_program_that_prints_on_both_streams())
            .expect("the start");
        assert!(started.recorded.identifier > 0);
        assert_eq!(started.recorded.start_time, 19186, "the kernel's own half");
        let mut ordinary = String::new();
        let mut complaints = String::new();
        started
            .ordinary
            .read_to_string(&mut ordinary)
            .expect("held");
        started
            .complaints
            .read_to_string(&mut complaints)
            .expect("held");
        assert_eq!(ordinary, "ordinary\n");
        assert_eq!(complaints, "complaints\n", "both streams, always");
        let ended = started.child.wait().expect("the ending is reaped");
        assert_eq!(ended.code(), Some(7));
    }

    /// **A started render the kernel recorded nothing for is undone rather
    /// than left running**, because a render with no recorded pair is one no
    /// stop can ever reach — the fault this branch exists against, checked
    /// by removing nothing and introducing the absence.
    #[test]
    fn a_render_with_no_recorded_pair_is_undone_and_the_caller_is_told_why() {
        let start = TheRenderStartedOnTheMachine {
            kernel: AKernelThat(None),
        };
        let launch = TheLaunch {
            the_blender_build_path: "/bin/sh".to_string(),
            words: vec!["-c".to_string(), "sleep 30".to_string()],
        };
        let why = start.start(&launch).expect_err("no pair, no start");
        assert!(
            why.contains("no execution group could be recorded"),
            "{why}"
        );
        assert!(why.contains("the start was undone"), "{why}");
    }

    /// A build path that is not there is a refusal naming it, and never a
    /// started render nobody holds.
    #[test]
    fn a_build_that_cannot_be_run_is_a_refusal_naming_it() {
        let start = TheRenderStartedOnTheMachine {
            kernel: AKernelThat(Some(1)),
        };
        let why = start
            .start(&TheLaunch {
                the_blender_build_path: "/no/such/blender".to_string(),
                words: Vec::new(),
            })
            .expect_err("nothing to run");
        assert!(why.contains("/no/such/blender"), "{why}");
    }
}
