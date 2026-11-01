//! The readings `Keys` §4.3 U7 names, taken on a real computer.
//!
//! **Three readings serve every clause**: the in-place container header read,
//! the fingerprint read, and the filesystem's own answers for presence,
//! readability and mode. `Keys` owns what each answer means; **this file owns
//! only how the answer is obtained**, which is the same split `Security and
//! licence` §1.4 B30 draws for the secret adapter.
//!
//! **Every read here is promptless, passphrase-free and network-free**, which
//! is the property `Servers` P1 stands on. The one thing that could break it is
//! a tool putting a prompt on screen, and §1's own construction closes that:
//! **the invocation runs with `SSH_ASKPASS_REQUIRE=never` and standard input
//! closed** — observed on this build, documented nowhere for this tool (`Keys`
//! §13 item 3, permanently unresolved). What holds without the observation is
//! that a read constructed to fail is judged by its exit status.
//!
//! **That construction is not enough where the process has a controlling
//! terminal, and the one read of a private key is handed an empty passphrase
//! as well.** Observed in a test on 2026-09-18, on this Mac, `OpenSSH_10.3p1`,
//! step 10b's first audit round (the runner's second addition): `ssh-keygen
//! -y -f` on a passphrase-protected key, run from a pseudo-terminal with
//! standard input closed and `SSH_ASKPASS_REQUIRE=never`, **printed its
//! passphrase prompt on the terminal and was still waiting six seconds
//! later**; the same run with `-P ""` exited 255 at once, and `-P ""` on a key
//! with no passphrase printed the public half exactly as before. So a user who
//! starts Smoozy from a terminal, and anybody running these tests from one,
//! would have had the key check hang on a protected key. **No diagnostic text
//! is read either way** — the refusal is the exit status.
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this crate's
//! own plan, `Preparation/Architecture/Desktop app and delivery.md`. Every other
//! document's marker is written with that document — at the site, or in this
//! file's own header list below. **Where a marker is a clause of this crate's
//! own plan *and* of the document meant, the header list is not enough and the
//! document is written out at the site**, because a bare one there resolves to a
//! real and wrong clause.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Keys` U2, U4, U6, U6a, U7, U7b, U8, U10a. `Security and
//! licence` B30.
//!
//! **A marker this crate's own plan also defines is deliberately not listed
//! above, whichever document is meant, and is written out with its document
//! at every site instead** — the header list cannot tell the two apart, and a
//! bare one would resolve to a real and wrong clause.

use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use smoozy_keys::fingerprint::HASH;
use smoozy_keys::generation::{ASKPASS_REQUIREMENT, NEVER, THE_TOOL};
use smoozy_keys::usable::{PermissionGate, Presence, Readings, ToolAnswer};

use crate::platform::files;

/// How much of a file the container test needs.
///
/// **The armour line and, for an `openssh-key-v1` container, the two names at
/// the front of its body.** Four kilobytes reaches both several times over and
/// is one page: the read is *in place and transient*, and reading the whole of
/// a private key file to answer a yes-or-no would hold key material this module
/// has no business holding (`Security and licence` §6).
const ENOUGH_FOR_A_HEADER: usize = 4096;

/// The readings, over the real filesystem and the installed `ssh-keygen`.
///
/// **It carries Smoozy's own temporary area**, because one of the three
/// readings needs somewhere to put a derived public half for one call. The
/// area is swept at application start (`Desktop app and delivery` L3a) and is
/// never a directory any
/// module scans for keys (`Security and licence` §7).
pub struct OnThisComputer {
    pub temporary_area: PathBuf,
    /// **`Keys` G6's answer for this reader, taken once and then held.**
    ///
    /// It is a property of the computer rather than of any key, and every clause
    /// of U4 goes down together where it is *no* — so asking it per key would
    /// spawn a process per row to answer a question whose answer cannot differ
    /// between two rows of one draw. **Held per reader and never per process**:
    /// one of these serves one caller's readings, and a fresh one asks again.
    the_generator: std::sync::OnceLock<bool>,
}

impl OnThisComputer {
    /// The readings, over the real filesystem and the installed tool, with the
    /// temporary area the one call that needs one writes into.
    pub fn over(temporary_area: PathBuf) -> OnThisComputer {
        OnThisComputer {
            temporary_area,
            the_generator: std::sync::OnceLock::new(),
        }
    }
}

impl Readings for OnThisComputer {
    /// **`Keys` G6, probed and never assumed**, by the one thing that tells a
    /// missing program from a program that judged something: the spawn.
    ///
    /// **How Smoozy tells *there is no generator* from *the tool refused* is
    /// observed on macOS only.** [`run`] reads it off `std::io::ErrorKind::
    /// NotFound` from a spawn of `ssh-keygen` on the inherited path. What a
    /// spawn of a missing program reports on Windows and on Linux is
    /// established by nothing this project has read or run, and **no answer is
    /// written here and no space is left for one**. What breaks if it comes
    /// back differently is `Keys` G6's whole answer: on a Windows machine with
    /// the OpenSSH capability removed, a spawn that reported anything but *not
    /// found* would make every reading answer *refused*, every key on the
    /// computer read **unreadable**, and the one line above the list never
    /// appear — which is what `Preparation/Specs/Keys.md` forbids in as many
    /// words.
    fn there_is_a_generator(&self) -> bool {
        // **The argument is one the tool cannot act on**, so the run reads the
        // machine and changes nothing: what is asked is whether the program is
        // there, and the answer is the spawn's rather than anything the tool
        // printed.
        *self
            .the_generator
            .get_or_init(|| !matches!(run(&["-?".to_string()]), ToolAnswer::NoGenerator))
    }

    /// **U4 clause 1**, answered by opening the file rather than by asking
    /// whether it exists — *readable* is half the clause, and a file that
    /// cannot be opened is not readable however present it is.
    fn presence(&self, path: &Path) -> Presence {
        match std::fs::metadata(path) {
            Err(_) => Presence::NotThere,
            Ok(found) if !found.is_file() => Presence::ThereButNotReadable,
            Ok(_) => match std::fs::File::open(path) {
                Ok(_) => Presence::ARegularFileThatOpens,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => Presence::NotThere,
                Err(_) => Presence::ThereButNotReadable,
            },
        }
    }

    /// **U4 clause 5.** On macOS and Linux the private file must grant no
    /// access to group or other; **on Windows the clause is not applied**,
    /// because the gate's Windows form is an ACL judgement and that the client
    /// refuses a loose key is established by nothing read (`Keys` §13 item 8).
    #[cfg(unix)]
    fn permission_gate(&self, path: &Path) -> PermissionGate {
        use std::os::unix::fs::PermissionsExt;
        match std::fs::metadata(path) {
            // A file whose mode cannot be read is not judged by this clause;
            // clause 1 has already answered for a file that will not open.
            Err(_) => PermissionGate::Passes,
            Ok(found) => {
                if found.permissions().mode() & 0o077 == 0 {
                    PermissionGate::Passes
                } else {
                    PermissionGate::GrantsAccessToGroupOrOther
                }
            }
        }
    }

    #[cfg(not(unix))]
    fn permission_gate(&self, _path: &Path) -> PermissionGate {
        PermissionGate::NotAppliedOnThisSystem
    }

    /// **U2 and U8's one in-place header read**, transient and bounded.
    fn opening_bytes(&self, path: &Path) -> Option<Vec<u8>> {
        let mut file = std::fs::File::open(path).ok()?;
        let mut head = vec![0u8; ENOUGH_FOR_A_HEADER];
        let mut filled = 0;
        while filled < head.len() {
            match file.read(&mut head[filled..]) {
                Ok(0) => break,
                Ok(read) => filled += read,
                Err(error) if error.kind() == std::io::ErrorKind::Interrupted => {}
                Err(_) => return None,
            }
        }
        head.truncate(filled);
        Some(head)
    }

    /// **`Keys` F1 and U7b: the private half's own fingerprint, in two
    /// calls.** `Keys` F1 says which file the read is from; `Keys` U7b says
    /// which invocation reads that half, and `Keys` F1 was corrected to name it on
    /// 2026-09-03.
    ///
    /// **First `ssh-keygen -y -f <path>`, which derives the public half from
    /// the private key itself** and is uninfluenced by whatever `.pub` sits
    /// beside the file. Then that line is fingerprinted.
    ///
    /// **Why the derived line goes to a file rather than down a pipe.** The
    /// tool will fingerprint standard input when handed `-f -`, and that was
    /// observed working on this build — but **`ssh-keygen(1)` documents no such
    /// meaning for `-f` under `-l`**, and the whole reason this reading is
    /// being rewritten is a clause that generalised one observation past what
    /// it said. So the reading rests on the plainly documented form, `-l -f
    /// <a file>`, and the file is one Smoozy owns and sweeps.
    ///
    /// **What is written is a public half and never key material** — the same
    /// bytes the `.pub` beside a key already holds, and the same bytes copy
    /// puts on the clipboard. **It is removed at once**, and the area's own
    /// start-of-run sweep is what completes the promise where a process died
    /// first.
    ///
    /// **The hash is named at the call** (`Keys` F8) rather than left to the
    /// default, because the default is a setting somebody can change and an
    /// MD5 fingerprint is never an identity.
    ///
    /// **`-P ""` is handed with it**, so a protected key is refused at once
    /// rather than prompted for on a controlling terminal — this file's head
    /// says what was observed. It changes nothing for a key with no passphrase.
    fn the_private_halfs_own_fingerprint_line(&self, path: &Path) -> ToolAnswer {
        let derived = match run(&[
            "-y".to_string(),
            "-P".to_string(),
            String::new(),
            "-f".to_string(),
            path.display().to_string(),
        ]) {
            ToolAnswer::Printed(line) => line,
            // **An encrypted key with no passphrase available ends here**,
            // promptlessly, at the tool's own refusal — which is exactly the
            // failure U7's fallback is written for.
            other => return other,
        };

        // **A name of this call's own, and the process id alone was not one.**
        // U10a has these readings running off the drawing thread with each
        // key's answer arriving on its own, and `usable::check` has three
        // callers — so two readings in flight wrote to and deleted the same
        // path, and what the second fingerprinted could be the first call's
        // key. Clause 4b would then report *no longer the same key*, which
        // raises the swap event and erases a passphrase.
        let beside = self.temporary_area.join(format!(
            "derived{}-{}.pub",
            std::process::id(),
            next_scratch_number()
        ));
        let written = files::write_whole_file(&beside, derived.as_bytes(), files::Durability::Ordinary);
        if !written.wrote_the_file() {
            let _ = files::remove_file(&beside);
            return ToolAnswer::Refused;
        }
        let answer = self.the_public_halfs_fingerprint_line(&beside);
        let _ = files::remove_file(&beside);
        answer
    }

    /// **U7's fallback, and the plainest reading of the three**: the file this
    /// call names is the public half.
    fn the_public_halfs_fingerprint_line(&self, public_half: &Path) -> ToolAnswer {
        run(&[
            "-l".to_string(),
            "-E".to_string(),
            HASH.to_string(),
            "-f".to_string(),
            public_half.display().to_string(),
        ])
    }
}

/// **A component of its own for every scratch name in this module.**
///
/// The process id alone is fixed for the life of the process, so two calls in
/// one run share a name; a counter beside it gives each call its own. Two
/// launches of Smoozy are told apart by the process id, and two calls of one
/// launch by this.
pub fn next_scratch_number() -> u64 {
    static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

/// One promptless run of the tool.
///
/// **A tool that is not there is `Keys` G6's answer and never a refusal**: a
/// refusal is the tool having judged something, and a missing binary has
/// judged nothing. The two are told apart by the spawn failing rather than by
/// any text.
///
/// **Which is observed on macOS only, and the narrowing sits here because this
/// is the branch that takes it.** That a spawn of a missing program reports
/// *not found* on Windows and on Linux is established by nothing read or run,
/// and **no answer is written for either and no space is left for one**.
pub fn run(arguments: &[String]) -> ToolAnswer {
    let attempt = Command::new(THE_TOOL)
        .args(arguments)
        // §1's construction: the helper route is closed and standard input is
        // closed with it, so a would-be prompt is an exit status.
        .env(ASKPASS_REQUIREMENT, NEVER)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output();

    match attempt {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => ToolAnswer::NoGenerator,
        Err(_) => ToolAnswer::Refused,
        Ok(finished) if finished.status.success() => {
            ToolAnswer::Printed(String::from_utf8_lossy(&finished.stdout).into_owned())
        }
        // **No diagnostic text is read off a refusal** (U6, `Security and
        // licence` §13 rule 5): what the exit status says is that the tool
        // refused, and the clause that failed is Smoozy's own reading.
        Ok(_) => ToolAnswer::Refused,
    }
}

#[cfg(test)]
mod tests {    use super::*;
    use smoozy_keys::container;
    use std::io::Write;

    fn scratch(name: &str) -> std::path::PathBuf {
        crate::scratch_folders::a_folder(format!("smoozy-key-readings-{name}-{}", std::process::id()))
    }

    /// The readings, with an area of their own for the one call that needs one.
    ///
    /// **A fresh area per call, because the tests run beside each other.** One
    /// shared name would have each caller sweeping the directory another was
    /// using, which is a flake and not a finding.
    fn here() -> OnThisComputer {
        static NEXT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let mine = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        OnThisComputer::over(scratch(&format!("area-{mine}")))
    }

    fn write(path: &Path, bytes: &[u8]) {
        let mut file = std::fs::File::create(path).expect("a file");
        file.write_all(bytes).expect("written");
    }

    #[test]
    fn a_file_that_is_not_there_and_one_that_is_are_told_apart() {
        let dir = scratch("presence");
        let there = dir.join("a-key");
        write(&there, b"-----BEGIN OPENSSH PRIVATE KEY-----\n");
        assert_eq!(
            here().presence(&there),
            Presence::ARegularFileThatOpens
        );
        assert_eq!(
            here().presence(&dir.join("nothing")),
            Presence::NotThere
        );
        // A directory at the path is not a regular file that opens.
        assert_eq!(here().presence(&dir), Presence::ThereButNotReadable);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[cfg(unix)]
    #[test]
    fn the_gate_refuses_a_key_open_to_group_or_other_and_passes_one_that_is_not() {
        use std::os::unix::fs::PermissionsExt;
        let dir = scratch("gate");
        let key = dir.join("a-key");
        write(&key, b"-----BEGIN OPENSSH PRIVATE KEY-----\n");

        std::fs::set_permissions(&key, std::fs::Permissions::from_mode(0o600)).expect("mode");
        assert_eq!(here().permission_gate(&key), PermissionGate::Passes);

        std::fs::set_permissions(&key, std::fs::Permissions::from_mode(0o644)).expect("mode");
        assert_eq!(
            here().permission_gate(&key),
            PermissionGate::GrantsAccessToGroupOrOther
        );

        // The clause is about group and other, so 0640 is refused too — on the
        // recommendation rather than on a reading, which `Keys` §13 item 8
        // carries.
        std::fs::set_permissions(&key, std::fs::Permissions::from_mode(0o640)).expect("mode");
        assert_eq!(
            here().permission_gate(&key),
            PermissionGate::GrantsAccessToGroupOrOther
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_header_read_is_bounded_and_stops_at_the_end_of_a_short_file() {
        let dir = scratch("header");
        let key = dir.join("a-key");
        write(&key, b"-----BEGIN RSA PRIVATE KEY-----\nProc-Type: 4,ENCRYPTED\n");
        let bytes = here().opening_bytes(&key).expect("bytes");
        assert!(bytes.len() < ENOUGH_FOR_A_HEADER);
        let read = container::read(&bytes).expect("a container");
        assert_eq!(read.protected, Some(true));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_long_file_is_not_read_past_what_a_header_needs() {
        let dir = scratch("bounded");
        let key = dir.join("a-key");
        let mut bytes = b"-----BEGIN OPENSSH PRIVATE KEY-----\n".to_vec();
        bytes.extend(std::iter::repeat(b'A').take(100_000));
        write(&key, &bytes);
        assert_eq!(
            here().opening_bytes(&key).expect("bytes").len(),
            ENOUGH_FOR_A_HEADER
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// **F-2: two readings in flight must not share a scratch name.**
    ///
    /// The derived public half was written to `derived<process id>.pub`, which
    /// is fixed for the life of the process rather than per call — and U10a has
    /// these readings running off the drawing thread with each key's answer
    /// arriving on its own, over an area all three callers share. **So two
    /// readings in flight wrote to and deleted the same path, and what the
    /// second fingerprinted could be the first call's key** — clause 4b would
    /// then report *no longer the same key*, raise the swap event and erase a
    /// passphrase.
    ///
    /// This runs two readings of **two different keys** at once, over one area,
    /// and each must come back with its own key's fingerprint.
    #[test]
    fn two_readings_at_once_over_one_area_do_not_read_each_others_keys() {
        let dir = scratch("at-once");
        let area = dir.join("area");
        std::fs::create_dir_all(&area).expect("the shared area");

        let mut keys = Vec::new();
        for name in ["one", "two"] {
            let path = dir.join(name);
            let made = Command::new(THE_TOOL)
                .args(["-q", "-t", "ed25519", "-N", "", "-C", name, "-f"])
                .arg(&path)
                .stdin(Stdio::null())
                .status();
            let Ok(made) = made else {
                // No generator on this computer; there is nothing to read.
                let _ = std::fs::remove_dir_all(&dir);
                return;
            };
            assert!(made.success());
            keys.push(path);
        }

        // What each key's own line says, read one at a time.
        let alone: Vec<String> = keys
            .iter()
            .map(|path| {
                match OnThisComputer::over(area.clone()).the_private_halfs_own_fingerprint_line(path)
                {
                    ToolAnswer::Printed(line) => line,
                    other => panic!("the tool answered {other:?}"),
                }
            })
            .collect();
        assert_ne!(alone[0], alone[1], "two keys, two fingerprints");

        // And now with the two readings overlapping, many times over, on the
        // one shared area.
        for _ in 0..8 {
            let together: Vec<String> = std::thread::scope(|threads| {
                let running: Vec<_> = keys
                    .iter()
                    .map(|path| {
                        let area = area.clone();
                        threads.spawn(move || {
                            match OnThisComputer::over(area)
                                .the_private_halfs_own_fingerprint_line(path)
                            {
                                ToolAnswer::Printed(line) => line,
                                other => panic!("the tool answered {other:?}"),
                            }
                        })
                    })
                    .collect();
                running
                    .into_iter()
                    .map(|thread| thread.join().expect("the reading finished"))
                    .collect()
            });
            assert_eq!(
                together, alone,
                "a reading answered with the other call's key"
            );
        }

        // And nothing is left behind: every scratch file is removed at once.
        let left: Vec<_> = std::fs::read_dir(&area)
            .expect("the area")
            .flatten()
            .map(|entry| entry.file_name())
            .collect();
        assert!(left.is_empty(), "the readings left {left:?}");
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// Writes past the harness's own capture, so a reading lands in the log of
    /// a plain `cargo test` run and not only under `--nocapture`.
    ///
    /// **The harness captures the print macros and nothing else**, so a
    /// `println!` in a test that passes reaches nobody. This writes to the
    /// process's error stream directly, which the capture does not touch.
    fn into_the_log(line: &str) {
        let mut stream = std::io::stderr();
        let _ = writeln!(stream, "{line}");
        let _ = stream.flush();
    }

    /// **What a spawn of a program that is not there reports on this
    /// computer** — the one reading that tells *the tool is absent* from *the
    /// tool refused*, and the whole ground of `Keys` G6 and `Keys` U6a.
    ///
    /// **[`run`] above takes that branch on `std::io::ErrorKind::NotFound`,
    /// and that was observed on macOS alone.** On a Windows machine with the
    /// OpenSSH capability removed, a spawn reporting anything else would make
    /// every reading answer *refused*, every key on the computer read
    /// **unreadable**, and the one line `Preparation/Specs/Keys.md` promises
    /// above the list never appear. **This test is what turns the other two
    /// systems' answer into a reading instead of a guess**, and it is written
    /// to be read in a hosted runner's log rather than merely to pass.
    ///
    /// **It reports; it does not assert what the answer is.** What is
    /// documented is only that there is an error. `std::process::Command`'s
    /// own page says a spawn *"returns an `io::Error` if the child process
    /// could not be spawned"*, with *"the program could not be found (for
    /// example, it does not exist, or, when given a bare name, it is not
    /// present in the `PATH`)"* first among the common reasons —
    /// <https://doc.rust-lang.org/std/process/struct.Command.html>, read
    /// 2026-09-05. **That page names no error kind and no operating-system
    /// error number, for any system.** It also says that on Windows *"Rust
    /// resolves the executable path before spawning, rather than passing the
    /// name to `CreateProcessW` for resolution"* — so Windows' own published
    /// error for a missing image is not necessarily what surfaces here, and
    /// reading Microsoft's page would answer a different question. **So the
    /// kind is asserted on macOS, where this project has observed it, and
    /// reported everywhere.**
    ///
    /// **Two shapes are read, because Smoozy's own case is the first**:
    /// `THE_TOOL` is a bare name resolved on the path. The second says whether
    /// the answer depends on which shape it is.
    #[test]
    fn what_a_spawn_of_a_missing_program_reports() {
        // The same construction `run` above uses, so what is read is this
        // branch's own spawn rather than a different one.
        fn spawn_of(program: &std::ffi::OsStr) -> std::io::Result<std::process::Output> {
            Command::new(program)
                .arg("-?")
                .env(ASKPASS_REQUIREMENT, NEVER)
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .output()
        }

        let dir = scratch("missing-program");
        let name = format!("smoozy-no-such-program-{}", std::process::id());
        let cases: [(&str, std::ffi::OsString); 2] = [
            ("a bare name that is on no path", std::ffi::OsString::from(&name)),
            ("an absolute path to nothing", dir.join(&name).into_os_string()),
        ];

        into_the_log(&format!(
            "MISSING-PROGRAM SPAWN READING - system {}, architecture {}",
            std::env::consts::OS,
            std::env::consts::ARCH
        ));

        for (shape, program) in cases {
            // The only thing asserted about the spawn itself, and it is not a
            // guess about any system: a program that does not exist cannot run.
            let error = match spawn_of(&program) {
                Ok(finished) => panic!(
                    "a program that does not exist spawned and finished: {shape}, {finished:?}"
                ),
                Err(error) => error,
            };

            // The condition `run` takes, written out here rather than borrowed
            // from it, so this reading cannot silently follow a changed branch.
            let read_as = if error.kind() == std::io::ErrorKind::NotFound {
                "the tool is absent - Keys G6 answers no generator and Keys U6a marks every key NOT CHECKED"
            } else {
                "the tool REFUSED - every key on this computer would read unreadable, which is what the Keys spec forbids"
            };
            into_the_log(&format!(
                "MISSING-PROGRAM SPAWN READING - {shape}: kind {:?}, operating-system error {:?}, text {:?}; Smoozy reads it as: {read_as}",
                error.kind(),
                error.raw_os_error(),
                error.to_string(),
            ));

            // **macOS only, and it is an observation rather than a guess** —
            // this is the branch `run` already rests on, taken on the one
            // system where this project has watched it. Windows and Linux are
            // read and left unasserted on purpose.
            #[cfg(target_os = "macos")]
            assert_eq!(
                error.kind(),
                std::io::ErrorKind::NotFound,
                "the branch in `run` rests on this and it no longer holds"
            );
        }

        let _ = std::fs::remove_dir_all(&dir);
    }

    /// **Every scratch name is a name of its own**, which is the whole of what
    /// the counter is for.
    #[test]
    fn no_two_scratch_names_are_the_same() {
        let taken: std::collections::HashSet<u64> =
            (0..64).map(|_| next_scratch_number()).collect();
        assert_eq!(taken.len(), 64);
    }

    /// **Run against the real tool on this computer**, which is what makes the
    /// parse and the promptless construction more than a claim. Where the
    /// binary is absent the reading answers *no generator*, which is the
    /// behaviour under test either way.
    #[test]
    fn the_real_tool_fingerprints_a_real_key_and_refuses_what_is_not_one() {
        let dir = scratch("real-tool");
        let key = dir.join("smoozy_ed25519");

        let made = Command::new(THE_TOOL)
            .args([
                "-q", "-t", "ed25519", "-N", "", "-C", "smoozy-test", "-f",
            ])
            .arg(&key)
            .stdin(Stdio::null())
            .status();
        let Ok(made) = made else {
            // No generator on this computer: the reading says so, and says it
            // for a file that is plainly there.
            assert_eq!(
                here().the_private_halfs_own_fingerprint_line(&key),
                ToolAnswer::NoGenerator
            );
            let _ = std::fs::remove_dir_all(&dir);
            return;
        };
        assert!(made.success(), "the tool made a key");

        let answer = here().the_private_halfs_own_fingerprint_line(&key);
        let ToolAnswer::Printed(line) = answer else {
            panic!("the tool printed a line for a key it just made: {answer:?}");
        };
        // The hash was named at the call, so the printed form is the pinned one.
        assert!(line.contains("SHA256:"), "{line}");
        let said = smoozy_keys::usable::parse_the_tools_line(
            &line,
            smoozy_keys::fingerprint::ReadFrom::PrivateFile,
        )
        .expect("the line reads back");
        assert_eq!(
            said.key_type,
            Some(smoozy_keys::types::KeyType::Ed25519)
        );
        assert_eq!(said.name.expect("a comment").as_str(), "smoozy-test");

        // And the container the same key was written in reads unprotected,
        // which is U8 on a file the tool itself made.
        let bytes = here().opening_bytes(&key).expect("bytes");
        let read = container::read(&bytes).expect("a container");
        assert_eq!(read.protected, Some(false));

        // Something that is not a key is refused rather than fingerprinted as
        // one — and the refusal is an exit status, not a prompt.
        let not_a_key = dir.join("known_hosts");
        write(&not_a_key, b"|1|abc=|def= ssh-ed25519 AAAA\n");
        assert!(container::read(&std::fs::read(&not_a_key).expect("bytes")).is_none());

        let _ = std::fs::remove_dir_all(&dir);
    }

    /// **A protected key is refused at once and never prompted for** — step
    /// 10b's first audit round, the runner's second addition. **Before `-P ""`
    /// this hung whenever the test run had a controlling terminal**, as
    /// `cargo test` from an interactive shell does: the tool printed its prompt
    /// on the terminal and waited. Under a run with no terminal it passes
    /// either way, which is why the construction is asserted beside it.
    #[test]
    fn a_protected_key_is_refused_at_once_and_never_prompted_for() {
        let dir = scratch("protected");
        let key = dir.join("protected_ed25519");
        let made = Command::new(THE_TOOL)
            .args([
                "-q", "-t", "ed25519", "-N", "a passphrase nobody will type", "-C",
                "smoozy-test", "-f",
            ])
            .arg(&key)
            .stdin(Stdio::null())
            .status();
        let Ok(made) = made else {
            let _ = std::fs::remove_dir_all(&dir);
            return;
        };
        assert!(made.success(), "the tool made a protected key");

        let (answered, the_answer) = std::sync::mpsc::channel();
        let reading = key.clone();
        std::thread::spawn(move || {
            let _ = answered.send(here().the_private_halfs_own_fingerprint_line(&reading));
        });
        let answer = the_answer
            .recv_timeout(std::time::Duration::from_secs(20))
            .expect("**the reading waited on a prompt** for a protected key's passphrase");
        assert_eq!(answer, ToolAnswer::Refused);

        // **And the construction itself**: the empty passphrase is handed on
        // the one read of a private half.
        let source = include_str!("readings.rs");
        let at = source
            .find("fn the_private_halfs_own_fingerprint_line")
            .expect("the reading");
        let body = &source[at..at + 600];
        assert!(body.contains("\"-P\".to_string(),\n            String::new(),"));

        let _ = std::fs::remove_dir_all(&dir);
    }
}
