//! **`File Browser` P6 and `File Browser` P6b at the desk: an act retried with admin access,
//! run for real against a stand-in machine.**
//!
//! **What the far end is.** A folder made for the test stands in for the
//! login's home. Every file-transfer session is **OpenSSH's own
//! `sftp-server`** started in it, as `tests/through_the_servers_own_point.rs`
//! does. Every command is run by **this computer's own `sh`**, with that folder
//! as `HOME`, so the act's constant command string is parsed by a real shell
//! and runs the real `xargs`, `rm`, `mv`, `cp` and `mkdir` — the BSD ones on a
//! Mac, the POSIX behaviour `File Browser` P6b leans on. **The one stand-in is the elevating
//! command**: a script named `sudo`, first on the path, that elevates nothing.
//! It reads a password from its standard input where told to (`-S`), checks it
//! against the test's own value, writes every argument list it was handed to a
//! log (so a test can prove the password was never one of them), and runs the
//! rest as the same user.
//!
//! **The listing's `find -printf` is GNU's and the Mac has none**, so a second
//! script stands in for `find`: it prints the four NUL-ended fields for what
//! is in the folder it is pointed at. What that proves is the driver's half —
//! the folder opened, read whole, the paths file taken away — and not GNU
//! `find`. **GNU `mv -T` has no Mac flag either**, so a third stands in for
//! `mv`: with `-T` it renames by the system's own `rename()`. **Two of the
//! stand-ins can play a racer** where a test asks: just before the act, one
//! swaps a folder on the path for a link, the other makes the landing name a
//! link — what another process could do in that moment (Sol's look,
//! 2026-11-12).
//!
//! **Nothing of the Creator's is touched.** Every path is inside a throwaway
//! folder made for the test and deleted with it. The password is a test
//! value; it rides the stand-in's environment, never a file, and no test
//! prints it.

#[cfg(not(unix))]
#[test]
fn the_far_end_here_wants_a_unix_socket_pair() {
    println!(
        "SKIPPED: the far end here is OpenSSH's own sftp-server on a Unix socket pair, which \
         this system ({}) has none of.",
        std::env::consts::OS
    );
}

#[cfg(unix)]
mod on_a_desk_with_a_socket_pair {
    use std::os::unix::fs::PermissionsExt;
    use std::path::{Path, PathBuf};
    use std::pin::Pin;
    use std::sync::Arc;
    use std::task::{Context, Poll};

    use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

    use smoozy_desktop::managing_files::{
        an_elevated_act_is_made, the_elevated_acts_are_made, WhatTheElevatedActCameTo, WhyTheFolderWasNotRead,
    };
    use smoozy_desktop::servers::sessions::{
        lend_until_it_ends, AHeldConnection, NoSession, TheAdditionalConnection, ThisServersSessions,
    };
    use smoozy_file_browser::elevated::{the_path_is_split, AnElevatedAct};

    /// **A delete of one whole path**, as the folder and the name inside it.
    fn delete(path: Vec<u8>) -> AnElevatedAct {
        let (folder, name) = the_path_is_split(&path).expect("a plain path");
        AnElevatedAct::Delete { folder, name }
    }

    /// **A new folder at one whole path.**
    fn new_folder(path: Vec<u8>) -> AnElevatedAct {
        let (folder, name) = the_path_is_split(&path).expect("a plain path");
        AnElevatedAct::NewFolder { folder, name }
    }

    /// **A rename from one whole path to another in the same folder.**
    fn rename(from: Vec<u8>, to: Vec<u8>) -> AnElevatedAct {
        let (folder, from) = the_path_is_split(&from).expect("a plain path");
        let (_, to) = the_path_is_split(&to).expect("a plain path");
        AnElevatedAct::Rename { folder, from, to }
    }
    use smoozy_security_and_licence::plaintext::Origin;
    use smoozy_server_protocol::transport::{CommandResult, CommandString, Ending};
    use smoozy_server_setup::elevated::HowElevationIsObtained;

    const WHERE_SFTP_SERVER_LIVES: &[&str] = &[
        "/usr/libexec/sftp-server",
        "/usr/lib/openssh/sftp-server",
        "/usr/libexec/openssh/sftp-server",
        "/usr/lib/ssh/sftp-server",
    ];

    /// **The test's own password.** A test value, used nowhere else.
    const THE_TEST_PASSWORD: &str = "test-only pw; with 'quotes' and $x";

    /// The names the brief asks for, in one: quotes, spaces, a semicolon, a
    /// newline and a leading dash — and two ways a shell would run a command.
    const A_HOSTILE_NAME: &str = "-rf ; touch PWNED 'it' \"is\"\nhere $(touch PWNED2) `touch PWNED3`";

    /// **The stand-in for the elevating command.** It never elevates.
    const THE_STAND_IN_SUDO: &str = r#"#!/bin/sh
printf '%s\n' "$*" >> "$SMZY_STAND_IN_LOG"
mode=
validate=
ignore_the_cache=
while [ $# -gt 0 ]; do
  case "$1" in
    -n) mode=n; shift ;;
    -S) mode=S; shift ;;
    -v) validate=1; shift ;;
    -k) ignore_the_cache=1; shift ;;
    -K)
      [ -n "$SMZY_STAND_IN_K_NEVER_ARRIVES" ] && exit 255
      [ -n "$SMZY_STAND_IN_CACHE" ] && /bin/rm -f "$SMZY_STAND_IN_CACHE"
      exit 0 ;;
    *) break ;;
  esac
done
cached=
if [ -z "$ignore_the_cache" ] && [ -n "$SMZY_STAND_IN_CACHE" ] && [ -e "$SMZY_STAND_IN_CACHE" ]; then cached=1; fi
if [ "$mode" = n ] && [ -n "$SMZY_STAND_IN_NEEDS_A_PASSWORD" ] && [ -z "$cached" ]; then
  echo "sudo: a password is required" >&2
  exit 1
fi
if [ "$mode" = S ] && [ -z "$cached" ]; then
  given=
  IFS= read -r given || true
  if [ "$given" != "$SMZY_STAND_IN_PASSWORD" ]; then
    printf 'Password:\nsudo: 1 incorrect password attempt\n' >&2
    exit 1
  fi
  if [ -z "$ignore_the_cache" ] && [ -n "$SMZY_STAND_IN_CACHE" ]; then : > "$SMZY_STAND_IN_CACHE"; fi
fi
[ -n "$validate" ] && exit 0
[ -n "$SMZY_STAND_IN_PAUSE" ] && sleep "$SMZY_STAND_IN_PAUSE"
[ -n "$SMZY_STAND_IN_TAKES_THE_PATHS_FILE" ] && /bin/rm -f "$HOME/.smoozy-carriage/files-elevated-paths"
SUDO_UID=$(id -u); export SUDO_UID
SUDO_GID=${SMZY_STAND_IN_GID:-$(id -g)}; export SUDO_GID
if [ -n "$SMZY_STAND_IN_INPUT_LOG" ]; then
  rest=$(mktemp "$SMZY_STAND_IN_SCRATCH/input.XXXXXX")
  cat > "$rest"
  cat "$rest" >> "$SMZY_STAND_IN_INPUT_LOG"
  "$@" < "$rest"
  status=$?
  /bin/rm -f "$rest"
  exit $status
fi
exec "$@"
"#;

    /// **The stand-in for GNU `mv -T`**, which the Mac's `mv` has no flag
    /// for: with `-T` it renames by the system's own `rename()`, which never
    /// moves into a folder or follows a link at the landing name; without it,
    /// it is the Mac's own `mv`. **And a racer, where a test asks for one**:
    /// just before the move it makes the landing name a link to a folder
    /// outside, as another process could in that moment.
    const THE_STAND_IN_MV: &str = r#"#!/bin/sh
t=
[ "$1" = -T ] && { t=1; shift; }
[ "$1" = -- ] && shift
[ $# -eq 2 ] || exec /bin/mv "$@"
if [ -n "$SMZY_RACE_LANDING_LINK_TO" ]; then /bin/ln -s "$SMZY_RACE_LANDING_LINK_TO" "$2"; fi
if [ -n "$t" ]; then
  exec perl -e 'rename($ARGV[0], $ARGV[1]) or do { print STDERR "mv: $!\n"; exit 1 }' -- "$1" "$2"
fi
exec /bin/mv -- "$1" "$2"
"#;

    /// **The Mac's own `mkdir`, and a racer where a test asks for one**: just
    /// after the folder is made, it takes it away and moves a folder that
    /// holds something into its place, as someone who can write the parent
    /// could (Sol's second look).
    const THE_STAND_IN_MKDIR: &str = r#"#!/bin/sh
/bin/mkdir "$@" || exit $?
if [ -n "$SMZY_RACE_NEW_FOLDER_FROM" ]; then
  for last; do :; done
  /bin/rmdir "$last"
  /bin/mv "$SMZY_RACE_NEW_FOLDER_FROM" "$last"
fi
"#;

    /// **The Mac's own `rm`, and a racer where a test asks for one**: just
    /// before the removal it moves a folder on the path aside and puts a link
    /// to a folder outside in its place, as another process could.
    const THE_STAND_IN_RM: &str = r#"#!/bin/sh
[ -n "$SMZY_RM_LOG" ] && printf '%s\n' "$*" >> "$SMZY_RM_LOG"
# GNU rm's --one-file-system, which the Mac's rm has no long option for.
first=1
for one; do
  [ "$first" = 1 ] && { set --; first=0; }
  [ "$one" = --one-file-system ] || set -- "$@" "$one"
done
if [ -n "$SMZY_RACE_SWAP_FOLDER" ]; then
  /bin/mv "$SMZY_RACE_SWAP_FOLDER" "$SMZY_RACE_SWAP_FOLDER.aside"
  /bin/ln -s "$SMZY_RACE_SWAP_TO" "$SMZY_RACE_SWAP_FOLDER"
fi
exec /bin/rm "$@"
"#;

    /// **The stand-in for GNU `find`**, for the listing's one form only.
    const THE_STAND_IN_FIND: &str = r#"#!/bin/sh
if [ -n "$SMZY_FIND_HAS_NO_PRINTF" ]; then echo "find: unknown predicate \`-printf'" >&2; exit 1; fi
[ "$1" = . ] && [ "$2" = -mindepth ] || exit 2
folder=.
[ -d "$folder" ] || { echo "find: not a folder" >&2; exit 1; }
for entry in "$folder"/* "$folder"/.[!.]*; do
  [ -e "$entry" ] || [ -L "$entry" ] || continue
  name="${entry##*/}"
  if [ -L "$entry" ]; then kind=l; size=0
  elif [ -d "$entry" ]; then kind=d; size=0
  else kind=f; size=$(wc -c < "$entry" | tr -d ' ')
  fi
  printf '%s\0%s\0%s\0%s\0' "$name" "$size" "1731400000.25" "$kind"
done
"#;

    struct AMachine {
        sftp_server: PathBuf,
        /// The scratch folder: the home, the stand-ins and the log beside it.
        scratch: PathBuf,
        home: PathBuf,
        stand_ins: PathBuf,
        log: PathBuf,
        needs_a_password: bool,
        /// **What a test sets for the stand-ins**: a cached credential, a
        /// racer, a pause.
        settings: std::sync::Mutex<Vec<(String, std::ffi::OsString)>>,
    }

    impl AMachine {
        fn standing(named: &str, needs_a_password: bool) -> Option<Arc<AMachine>> {
            let sftp_server = WHERE_SFTP_SERVER_LIVES.iter().map(PathBuf::from).find(|path| path.exists())?;
            let scratch = std::env::temp_dir().join(format!("smzy-admin-{named}-{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&scratch);
            let home = scratch.join("home");
            let stand_ins = scratch.join("stand-ins");
            std::fs::create_dir_all(&home).expect("the login's home");
            std::fs::create_dir_all(&stand_ins).expect("the stand-ins' folder");
            let scratch = std::fs::canonicalize(&scratch).expect("the scratch is there");
            let home = scratch.join("home");
            let stand_ins = scratch.join("stand-ins");
            for (name, body) in [
                ("sudo", THE_STAND_IN_SUDO),
                ("find", THE_STAND_IN_FIND),
                ("mv", THE_STAND_IN_MV),
                ("rm", THE_STAND_IN_RM),
                ("mkdir", THE_STAND_IN_MKDIR),
            ] {
                let at = stand_ins.join(name);
                std::fs::write(&at, body).expect("a stand-in is written");
                std::fs::set_permissions(&at, std::fs::Permissions::from_mode(0o755)).expect("it runs");
            }
            let log = scratch.join("stand-in-sudo.log");
            std::fs::write(&log, b"").expect("the log");
            Some(Arc::new(AMachine {
                sftp_server,
                scratch,
                home,
                stand_ins,
                log,
                needs_a_password,
                settings: std::sync::Mutex::new(Vec::new()),
            }))
        }

        /// **One setting for the stand-ins**, from now on.
        fn set(&self, name: &str, value: impl Into<std::ffi::OsString>) {
            self.settings.lock().expect("the settings").push((name.to_string(), value.into()));
        }

        /// **Where the stand-in keeps its cached credential**, as `sudo`
        /// would: made by a password that passed, ended by `-K`.
        fn the_cache(&self) -> PathBuf {
            self.scratch.join("the-cached-credential")
        }

        /// **What the act's own input held**, as `sudo`'s input log would.
        fn the_input_log(&self) -> PathBuf {
            self.scratch.join("the-acts-input.log")
        }

        fn in_home(&self, relative: &str) -> PathBuf {
            self.home.join(relative)
        }

        fn path_bytes(&self, relative: &str) -> Vec<u8> {
            use std::os::unix::ffi::OsStrExt;
            self.in_home(relative).as_os_str().as_bytes().to_vec()
        }

        fn what_the_stand_in_was_handed(&self) -> String {
            std::fs::read_to_string(&self.log).unwrap_or_default()
        }

        /// **Every byte of every file under the scratch folder**, the
        /// stand-ins' own scripts aside, so a test can say none holds the
        /// password.
        fn every_file_written(&self) -> Vec<(PathBuf, Vec<u8>)> {
            fn walk(at: &Path, skip: &Path, found: &mut Vec<(PathBuf, Vec<u8>)>) {
                let Ok(entries) = std::fs::read_dir(at) else { return };
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.starts_with(skip) {
                        continue;
                    }
                    let Ok(kind) = std::fs::symlink_metadata(&path) else { continue };
                    if kind.is_dir() {
                        walk(&path, skip, found);
                    } else if kind.is_file() {
                        found.push((path.clone(), std::fs::read(&path).unwrap_or_default()));
                    }
                }
            }
            let mut found = Vec::new();
            walk(&self.scratch, &self.stand_ins, &mut found);
            found
        }
    }

    impl Drop for AMachine {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.scratch);
        }
    }

    struct ASession {
        stream: tokio::net::UnixStream,
        child: std::process::Child,
    }

    impl Drop for ASession {
        fn drop(&mut self) {
            let _ = self.child.kill();
            let _ = self.child.wait();
        }
    }

    impl AsyncRead for ASession {
        fn poll_read(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<std::io::Result<()>> {
            Pin::new(&mut self.stream).poll_read(cx, buf)
        }
    }

    impl AsyncWrite for ASession {
        fn poll_write(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<std::io::Result<usize>> {
            Pin::new(&mut self.stream).poll_write(cx, buf)
        }
        fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
            Pin::new(&mut self.stream).poll_flush(cx)
        }
        fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
            Pin::new(&mut self.stream).poll_shutdown(cx)
        }
    }

    struct AConnectionToIt(Arc<AMachine>);

    impl AHeldConnection for AConnectionToIt {
        type Channel = ASession;

        async fn open_a_session(&mut self) -> Result<ASession, NoSession> {
            use std::os::unix::io::OwnedFd;
            let (ours, theirs) = std::os::unix::net::UnixStream::pair().expect("a socket pair");
            ours.set_nonblocking(true).expect("the end we drive");
            let reading: OwnedFd = theirs.try_clone().expect("its reading end").into();
            let writing: OwnedFd = theirs.into();
            let child = std::process::Command::new(&self.0.sftp_server)
                .current_dir(&self.0.home)
                .stdin(std::process::Stdio::from(reading))
                .stdout(std::process::Stdio::from(writing))
                .stderr(std::process::Stdio::null())
                .spawn()
                .expect("the machine's own file-transfer server starts");
            Ok(ASession { stream: tokio::net::UnixStream::from_std(ours).expect("the end we drive"), child })
        }

        /// **The command string, parsed by a real `sh`**, its standard input
        /// the frame and then closed — as an SSH `exec` request is.
        async fn a_command(&mut self, command: CommandString, frame: Option<Vec<u8>>) -> Result<CommandResult, NoSession> {
            use std::io::Write;
            let path = format!("{}:/usr/bin:/bin:/usr/sbin:/sbin", self.0.stand_ins.display());
            let mut shell = std::process::Command::new("/bin/sh");
            shell
                .arg("-c")
                .arg(command.as_str())
                .current_dir(&self.0.home)
                .env_clear()
                .env("HOME", &self.0.home)
                .env("PATH", path)
                .env("SMZY_STAND_IN_LOG", &self.0.log)
                .env("SMZY_STAND_IN_PASSWORD", THE_TEST_PASSWORD)
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped());
            if self.0.needs_a_password {
                shell.env("SMZY_STAND_IN_NEEDS_A_PASSWORD", "1");
            }
            shell.env("SMZY_STAND_IN_SCRATCH", &self.0.scratch);
            for (name, value) in self.0.settings.lock().expect("the settings").iter() {
                shell.env(name, value);
            }
            let mut child = shell.spawn().map_err(|_| NoSession::Refused)?;
            {
                let mut input = child.stdin.take().expect("its input");
                if let Some(frame) = frame {
                    let _ = input.write_all(&frame);
                }
            }
            let done = child.wait_with_output().map_err(|_| NoSession::Refused)?;
            Ok(CommandResult { output: done.stdout, complaint: done.stderr, status: done.status.code().map(|code| code as u32) })
        }

        async fn until_it_ends(&mut self) -> Ending {
            std::future::pending().await
        }

        async fn torn_down(self) -> Ending {
            Ending::TornDownBySmoozy
        }
    }

    struct APointLending {
        sessions: ThisServersSessions<AConnectionToIt>,
        lending: tokio::task::JoinHandle<Ending>,
    }

    impl APointLending {
        async fn over(machine: &Arc<AMachine>) -> APointLending {
            let sessions = ThisServersSessions::<AConnectionToIt>::for_one_server();
            let more = Arc::clone(machine);
            let additional: TheAdditionalConnection<AConnectionToIt> = Arc::new(move || {
                let more = Arc::clone(&more);
                Box::pin(async move { Some(AConnectionToIt(more)) })
            });
            let mut primary = AConnectionToIt(Arc::clone(machine));
            let lending = tokio::spawn({
                let sessions = sessions.clone();
                async move { lend_until_it_ends(&sessions, &mut primary, &additional).await }
            });
            while !sessions.a_connection_is_live() {
                tokio::task::yield_now().await;
            }
            APointLending { sessions, lending }
        }
    }

    impl Drop for APointLending {
        fn drop(&mut self) {
            self.lending.abort();
        }
    }

    fn nothing_ran(machine: &AMachine) {
        for name in ["PWNED", "PWNED2", "PWNED3"] {
            assert!(!machine.in_home(name).exists(), "{name} in the home: a name ran as a command");
            assert!(!machine.in_home("data").join(name).exists(), "{name} in the folder: a name ran");
        }
    }

    fn the_carriage_is_taken_away(machine: &AMachine) {
        for token in ["files-elevated-folder", "files-elevated-paths"] {
            let at = machine.in_home(".smoozy-carriage").join(token);
            assert!(std::fs::symlink_metadata(&at).is_err(), "{token} was left on the machine");
        }
    }

    /// **A name with quotes, spaces, a semicolon, a newline and a leading dash
    /// is deleted whole, and nothing in it runs.** Its neighbour stays.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_hostile_name_is_deleted_whole_and_nothing_in_it_runs() {
        let Some(machine) = AMachine::standing("delete", false) else {
            println!("SKIPPED: no sftp-server on this system");
            return;
        };
        std::fs::create_dir_all(machine.in_home("data").join(A_HOSTILE_NAME)).expect("the hostile folder");
        std::fs::write(machine.in_home("data").join(A_HOSTILE_NAME).join("inside.txt"), b"x").expect("a file in it");
        std::fs::write(machine.in_home("data/keep.txt"), b"keep").expect("its neighbour");
        std::fs::write(machine.in_home("data/-rf"), b"keep").expect("a neighbour named like the option");
        let point = APointLending::over(&machine).await;
        let path = machine.path_bytes(&format!("data/{A_HOSTILE_NAME}"));
        let came = an_elevated_act_is_made(&point.sessions, &delete(path), HowElevationIsObtained::Passwordless).await;
        assert_eq!(came, WhatTheElevatedActCameTo::Done);
        assert!(!machine.in_home("data").join(A_HOSTILE_NAME).exists(), "the hostile folder is gone");
        assert_eq!(std::fs::read(machine.in_home("data/keep.txt")).expect("kept"), b"keep");
        assert_eq!(std::fs::read(machine.in_home("data/-rf")).expect("kept"), b"keep");
        nothing_ran(&machine);
        the_carriage_is_taken_away(&machine);
        assert!(machine.what_the_stand_in_was_handed().starts_with("-k -n sh -c "), "the passwordless form, ignoring any cached credential");
    }

    /// **A new folder with the hostile name is made with exactly that name**,
    /// and a rename carries it whole.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_hostile_name_is_made_and_renamed_whole() {
        let Some(machine) = AMachine::standing("make", false) else {
            println!("SKIPPED: no sftp-server on this system");
            return;
        };
        std::fs::create_dir_all(machine.in_home("data")).expect("the folder");
        let point = APointLending::over(&machine).await;
        let made = machine.path_bytes(&format!("data/{A_HOSTILE_NAME}"));
        let came = an_elevated_act_is_made(&point.sessions, &new_folder(made.clone()), HowElevationIsObtained::Passwordless).await;
        assert_eq!(came, WhatTheElevatedActCameTo::Done);
        assert!(machine.in_home("data").join(A_HOSTILE_NAME).is_dir());
        let moved = machine.path_bytes(&format!("data/-moved {A_HOSTILE_NAME}"));
        let came = an_elevated_act_is_made(&point.sessions, &rename(made, moved), HowElevationIsObtained::Passwordless).await;
        assert_eq!(came, WhatTheElevatedActCameTo::Done);
        assert!(!machine.in_home("data").join(A_HOSTILE_NAME).exists());
        assert!(machine.in_home("data").join(format!("-moved {A_HOSTILE_NAME}")).is_dir());
        // One entry and nothing else: no name was split into two.
        assert_eq!(std::fs::read_dir(machine.in_home("data")).expect("the folder").count(), 1);
        nothing_ran(&machine);
        the_carriage_is_taken_away(&machine);
    }

    /// **A link planted inside the folder is never followed out of it** — not
    /// by a delete or a rename. What it points at is untouched, and a renamed
    /// folder still holds the link, not what it names.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_planted_link_inside_is_never_followed_out() {
        let Some(machine) = AMachine::standing("link", false) else {
            println!("SKIPPED: no sftp-server on this system");
            return;
        };
        std::fs::create_dir_all(machine.in_home("outside")).expect("outside");
        std::fs::write(machine.in_home("outside/precious"), b"precious").expect("what the link names");
        for victim in ["one", "three"] {
            std::fs::create_dir_all(machine.in_home(&format!("data/{victim}"))).expect("the folder");
            std::os::unix::fs::symlink(machine.in_home("outside"), machine.in_home(&format!("data/{victim}/escape"))).expect("the planted link");
        }
        std::os::unix::fs::symlink(machine.in_home("outside"), machine.in_home("data/escape")).expect("a link named as the act's item");
        let point = APointLending::over(&machine).await;
        let came = an_elevated_act_is_made(&point.sessions, &delete(machine.path_bytes("data/one")), HowElevationIsObtained::Passwordless).await;
        assert_eq!(came, WhatTheElevatedActCameTo::Done);
        assert!(!machine.in_home("data/one").exists());
        let came = an_elevated_act_is_made(&point.sessions, &delete(machine.path_bytes("data/escape")), HowElevationIsObtained::Passwordless).await;
        assert_eq!(came, WhatTheElevatedActCameTo::Done);
        assert!(std::fs::symlink_metadata(machine.in_home("data/escape")).is_err(), "the link itself went");
        let came = an_elevated_act_is_made(
            &point.sessions,
            &rename(machine.path_bytes("data/three"), machine.path_bytes("data/moved")),
            HowElevationIsObtained::Passwordless,
        )
        .await;
        assert_eq!(came, WhatTheElevatedActCameTo::Done);
        assert!(std::fs::symlink_metadata(machine.in_home("data/moved/escape")).expect("moved").file_type().is_symlink());
        assert_eq!(std::fs::read(machine.in_home("outside/precious")).expect("untouched"), b"precious");
        assert_eq!(std::fs::read_dir(machine.in_home("outside")).expect("outside").count(), 1);
    }

    /// **A rename never lands on something already there**: not replaced,
    /// not merged, not moved into it. Nothing changes.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_rename_never_lands_on_something_there() {
        let Some(machine) = AMachine::standing("taken", false) else {
            println!("SKIPPED: no sftp-server on this system");
            return;
        };
        std::fs::create_dir_all(machine.in_home("data/folder")).expect("an existing folder");
        std::fs::write(machine.in_home("data/a.txt"), b"a").expect("a");
        std::fs::write(machine.in_home("data/b.txt"), b"b").expect("b");
        let point = APointLending::over(&machine).await;
        for act in [
            rename(machine.path_bytes("data/a.txt"), machine.path_bytes("data/b.txt")),
            rename(machine.path_bytes("data/a.txt"), machine.path_bytes("data/folder")),
        ] {
            let came = an_elevated_act_is_made(&point.sessions, &act, HowElevationIsObtained::Passwordless).await;
            assert_eq!(came, WhatTheElevatedActCameTo::TheLandingIsTaken, "{act:?}");
        }
        assert_eq!(std::fs::read(machine.in_home("data/a.txt")).expect("a"), b"a");
        assert_eq!(std::fs::read(machine.in_home("data/b.txt")).expect("b"), b"b");
        assert_eq!(std::fs::read_dir(machine.in_home("data/folder")).expect("folder").count(), 0);
        the_carriage_is_taken_away(&machine);
    }

    /// **The password rides the elevating command's input and nothing else**:
    /// never in an argument list, never in a file, never in what came back.
    /// It is sent once, to the act itself under `sudo -k` (Sol's finding 5),
    /// and `sudo -K` follows as a second guard.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn the_password_rides_only_the_elevating_commands_input() {
        let Some(machine) = AMachine::standing("password", true) else {
            println!("SKIPPED: no sftp-server on this system");
            return;
        };
        std::fs::create_dir_all(machine.in_home("data")).expect("the folder");
        let point = APointLending::over(&machine).await;
        let password = Origin::what_the_user_typed(THE_TEST_PASSWORD.as_bytes());
        let came = an_elevated_act_is_made(
            &point.sessions,
            &new_folder(machine.path_bytes("data/made")),
            HowElevationIsObtained::APasswordIsHeld(&password),
        )
        .await;
        assert_eq!(came, WhatTheElevatedActCameTo::Done);
        assert!(machine.in_home("data/made").is_dir());
        let handed = machine.what_the_stand_in_was_handed();
        assert!(!handed.contains(THE_TEST_PASSWORD), "the password was in an argument list");
        let lines: Vec<&str> = handed.lines().collect();
        assert!(lines.first().is_some_and(|line| line.starts_with("-k -S sh -c ")), "the act, and no validate: {lines:?}");
        assert_eq!(lines.last().copied(), Some("-K"), "and the second guard last");
        assert_eq!(lines.len(), 2, "{lines:?}");
        for (path, bytes) in machine.every_file_written() {
            assert!(
                !bytes.windows(THE_TEST_PASSWORD.len()).any(|window| window == THE_TEST_PASSWORD.as_bytes()),
                "the password was written to {}",
                path.display()
            );
        }
        the_carriage_is_taken_away(&machine);
    }

    /// **`File Browser` P6b, *two more rules after Fable's fresh look*
    /// (finding 5, 2026-11-12): the delete with admin access stays on the
    /// file system of the item named.** As root nothing under the folder is
    /// refused, so a second disk mounted inside it was emptied too. A second
    /// disk cannot be mounted at the desk; what is checked is that GNU `rm`'s
    /// own guard is on the command the server is sent, and that the delete
    /// still deletes.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn fresh_look_5_the_delete_with_admin_access_stays_on_one_file_system() {
        let Some(machine) = AMachine::standing("one-file-system", true) else {
            println!("SKIPPED: no sftp-server on this system");
            return;
        };
        std::fs::create_dir_all(machine.in_home("data/old/inside")).expect("the folder");
        std::fs::write(machine.in_home("data/old/inside/a-file"), b"x").expect("a file");
        let log = machine.scratch.join("rm.log");
        machine.set("SMZY_RM_LOG", &log);
        let point = APointLending::over(&machine).await;
        let password = Origin::what_the_user_typed(THE_TEST_PASSWORD.as_bytes());
        let came = an_elevated_act_is_made(
            &point.sessions,
            &delete(machine.path_bytes("data/old")),
            HowElevationIsObtained::APasswordIsHeld(&password),
        )
        .await;
        assert_eq!(came, WhatTheElevatedActCameTo::Done);
        assert!(!machine.in_home("data/old").exists());
        let handed = std::fs::read_to_string(&log).unwrap_or_default();
        assert_eq!(
            handed.trim(),
            "-r -f --one-file-system -- ./old",
            "the delete would go into other disks mounted under the folder"
        );
    }

    /// **`File Browser` P6b, the second of those rules (finding 6): a
    /// password the user typed is sent where the act has just shown it needs
    /// one.** This login runs nothing as admin without a password, yet P5's
    /// probe can pass on such a login, so the press starts with no password.
    /// **Before the fix** the typed password was dropped, the act was refused
    /// again, and the box had asked for something that could not help.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn fresh_look_6_a_typed_password_is_sent_where_the_act_showed_it_needs_one() {
        let Some(machine) = AMachine::standing("typed-after-all", true) else {
            println!("SKIPPED: no sftp-server on this system");
            return;
        };
        std::fs::create_dir_all(machine.in_home("data")).expect("the folder");
        let point = APointLending::over(&machine).await;
        let acts = [new_folder(machine.path_bytes("data/made"))];
        // **Nothing typed: the act needs a password, and the box will ask.**
        let (ended, sent) =
            the_elevated_acts_are_made(&point.sessions, &acts, HowElevationIsObtained::Passwordless, None).await;
        assert_eq!((ended, sent), (vec![WhatTheElevatedActCameTo::APasswordIsNeeded], false));
        // **Typed at the box: sent once more, with it.**
        let typed = Origin::what_the_user_typed(THE_TEST_PASSWORD.as_bytes());
        let (ended, sent) =
            the_elevated_acts_are_made(&point.sessions, &acts, HowElevationIsObtained::Passwordless, Some(&typed)).await;
        assert_eq!(
            (ended, sent),
            (vec![WhatTheElevatedActCameTo::Done], true),
            "the password typed at the box was thrown away"
        );
        assert!(machine.in_home("data/made").is_dir());
        the_carriage_is_taken_away(&machine);
    }

    /// **And where the server asks nothing, a typed password is never sent**
    /// — the rule after Sol's second look, kept.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn fresh_look_6_a_typed_password_is_not_sent_where_the_act_needs_none() {
        let Some(machine) = AMachine::standing("typed-not-needed", false) else {
            println!("SKIPPED: no sftp-server on this system");
            return;
        };
        std::fs::create_dir_all(machine.in_home("data")).expect("the folder");
        let point = APointLending::over(&machine).await;
        let acts = [new_folder(machine.path_bytes("data/made"))];
        let typed = Origin::what_the_user_typed(THE_TEST_PASSWORD.as_bytes());
        let (ended, sent) =
            the_elevated_acts_are_made(&point.sessions, &acts, HowElevationIsObtained::Passwordless, Some(&typed)).await;
        assert_eq!((ended, sent), (vec![WhatTheElevatedActCameTo::Done], false));
        assert!(
            !machine.what_the_stand_in_was_handed().lines().any(|line| line.starts_with("-k -S ")),
            "a password was sent to a server that asked for none"
        );
    }

    /// **A wrong password runs nothing and says so**; a password owed and not
    /// held sends nothing at all.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_wrong_password_runs_nothing_and_none_held_sends_nothing() {
        let Some(machine) = AMachine::standing("wrong", true) else {
            println!("SKIPPED: no sftp-server on this system");
            return;
        };
        std::fs::create_dir_all(machine.in_home("data")).expect("the folder");
        let point = APointLending::over(&machine).await;
        let act = new_folder(machine.path_bytes("data/made"));
        let came = an_elevated_act_is_made(&point.sessions, &act, HowElevationIsObtained::APasswordIsOwedAndNoneIsHeld).await;
        assert_eq!(came, WhatTheElevatedActCameTo::APasswordIsNeeded);
        assert_eq!(machine.what_the_stand_in_was_handed(), "", "nothing was sent");
        let wrong = Origin::what_the_user_typed(b"not it".to_vec());
        let came = an_elevated_act_is_made(&point.sessions, &act, HowElevationIsObtained::APasswordIsHeld(&wrong)).await;
        assert_eq!(came, WhatTheElevatedActCameTo::TheWrongPassword);
        assert!(!machine.in_home("data/made").exists());
        assert!(!machine.what_the_stand_in_was_handed().contains("not it"));
        the_carriage_is_taken_away(&machine);
    }

    /// **A folder is read with admin access inside itself, opened once**, its
    /// hostile name whole, and the paths file taken away after.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_folder_is_read_inside_itself_and_a_hostile_name_comes_back_whole() {
        let Some(machine) = AMachine::standing("listing", false) else {
            println!("SKIPPED: no sftp-server on this system");
            return;
        };
        std::fs::create_dir_all(machine.in_home("data/sub")).expect("the folder");
        std::fs::write(machine.in_home("data").join(A_HOSTILE_NAME), b"twelve bytes").expect("a hostile file");
        let point = APointLending::over(&machine).await;
        let came = an_elevated_act_is_made(&point.sessions, &AnElevatedAct::AListing { folder: machine.path_bytes("data") }, HowElevationIsObtained::Passwordless).await;
        let WhatTheElevatedActCameTo::TheRows(mut rows) = came else {
            panic!("no rows: {came:?}");
        };
        rows.sort_by(|one, other| one.name.cmp(&other.name));
        let names: Vec<&[u8]> = rows.iter().map(|row| row.name.as_slice()).collect();
        assert_eq!(names, vec![A_HOSTILE_NAME.as_bytes(), b"sub".as_slice()]);
        assert_eq!(rows[0].attributes.size, Some(12));
        nothing_ran(&machine);
        the_carriage_is_taken_away(&machine);
    }

    /// **A path that could name somewhere else is refused on the desk**, and
    /// nothing reaches the machine.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_dot_path_or_the_root_is_refused_before_anything_is_sent() {
        let Some(machine) = AMachine::standing("refused", false) else {
            println!("SKIPPED: no sftp-server on this system");
            return;
        };
        let point = APointLending::over(&machine).await;
        for act in [
            AnElevatedAct::Delete { folder: b"/".to_vec(), name: b"..".to_vec() },
            AnElevatedAct::Delete { folder: machine.path_bytes("data/.."), name: b"outside".to_vec() },
            AnElevatedAct::Delete { folder: b"relative".to_vec(), name: b"x".to_vec() },
            AnElevatedAct::Rename { folder: machine.path_bytes("data"), from: b"a".to_vec(), to: b"../b".to_vec() },
        ] {
            let came = an_elevated_act_is_made(&point.sessions, &act, HowElevationIsObtained::Passwordless).await;
            assert!(matches!(came, WhatTheElevatedActCameTo::ThePathIsRefused(_)), "{came:?}");
        }
        assert_eq!(machine.what_the_stand_in_was_handed(), "");
        assert!(std::fs::symlink_metadata(machine.in_home(".smoozy-carriage")).is_err(), "nothing crossed");
    }

    // ── Sol's look, 2026-11-12: the races and the password ─────────────────

    /// **Sol's finding 1: two acts at once never take each other's paths.**
    /// A delete and a rename on one server, started together, with the
    /// elevating command slowed so the second act's paths are written while
    /// the first waits. Each does what was asked of it and nothing else.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn two_acts_at_once_never_take_each_others_paths() {
        let Some(machine) = AMachine::standing("overlap", false) else {
            println!("SKIPPED: no sftp-server on this system");
            return;
        };
        std::fs::create_dir_all(machine.in_home("data")).expect("the folder");
        std::fs::write(machine.in_home("data/a.txt"), b"a").expect("a");
        std::fs::write(machine.in_home("data/b.txt"), b"b").expect("b");
        machine.set("SMZY_STAND_IN_PAUSE", "0.4");
        let point = APointLending::over(&machine).await;
        let delete = delete(machine.path_bytes("data/a.txt"));
        let rename = rename(machine.path_bytes("data/b.txt"), machine.path_bytes("data/c.txt"));
        let (deleted, renamed) = tokio::join!(
            an_elevated_act_is_made(&point.sessions, &delete, HowElevationIsObtained::Passwordless),
            async {
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                an_elevated_act_is_made(&point.sessions, &rename, HowElevationIsObtained::Passwordless).await
            }
        );
        assert_eq!(deleted, WhatTheElevatedActCameTo::Done);
        assert_eq!(renamed, WhatTheElevatedActCameTo::Done);
        assert!(!machine.in_home("data/a.txt").exists(), "the delete took its own path");
        assert!(!machine.in_home("data/b.txt").exists(), "the rename moved its own path");
        assert_eq!(std::fs::read(machine.in_home("data/c.txt")).expect("renamed"), b"b");
    }

    /// **Sol's finding 3: a folder on the path swapped for a link while the
    /// act runs moves nothing outside.** A racer moves `data` aside and puts
    /// a link to `outside` in its place just before the removal; the file of
    /// the same name outside is untouched.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_folder_swapped_for_a_link_during_the_act_sends_nothing_outside() {
        let Some(machine) = AMachine::standing("swap", false) else {
            println!("SKIPPED: no sftp-server on this system");
            return;
        };
        std::fs::create_dir_all(machine.in_home("data")).expect("the folder");
        std::fs::create_dir_all(machine.in_home("outside")).expect("outside");
        std::fs::write(machine.in_home("data/protected"), b"the user's").expect("the victim");
        std::fs::write(machine.in_home("outside/protected"), b"precious").expect("what the link names");
        machine.set("SMZY_RACE_SWAP_FOLDER", machine.in_home("data"));
        machine.set("SMZY_RACE_SWAP_TO", machine.in_home("outside"));
        let point = APointLending::over(&machine).await;
        let _ = an_elevated_act_is_made(
            &point.sessions,
            &delete(machine.path_bytes("data/protected")),
            HowElevationIsObtained::Passwordless,
        )
        .await;
        assert_eq!(std::fs::read(machine.in_home("outside/protected")).expect("untouched"), b"precious");
        assert!(!machine.in_home("data.aside/protected").exists(), "the act was done in the folder it opened");
    }

    /// **Sol's finding 3, the other half: a folder that is a link when the act
    /// starts is not acted in.** Nothing outside changes.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_folder_that_is_a_link_is_never_acted_in() {
        let Some(machine) = AMachine::standing("linked", false) else {
            println!("SKIPPED: no sftp-server on this system");
            return;
        };
        std::fs::create_dir_all(machine.in_home("outside")).expect("outside");
        std::fs::write(machine.in_home("outside/protected"), b"precious").expect("what the link names");
        std::os::unix::fs::symlink(machine.in_home("outside"), machine.in_home("data")).expect("the swapped folder");
        let point = APointLending::over(&machine).await;
        let came = an_elevated_act_is_made(
            &point.sessions,
            &delete(machine.path_bytes("data/protected")),
            HowElevationIsObtained::Passwordless,
        )
        .await;
        assert_eq!(came, WhatTheElevatedActCameTo::ThePathGoesThroughALink);
        assert_eq!(std::fs::read(machine.in_home("outside/protected")).expect("untouched"), b"precious");
    }

    /// **Sol's finding 4: a landing name made in the moment before a rename
    /// is never followed.** A racer makes it a link to a folder outside just
    /// before the move; nothing lands outside.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_landing_made_just_before_a_rename_is_never_followed() {
        let Some(machine) = AMachine::standing("landing", false) else {
            println!("SKIPPED: no sftp-server on this system");
            return;
        };
        std::fs::create_dir_all(machine.in_home("data")).expect("the folder");
        std::fs::create_dir_all(machine.in_home("outside")).expect("outside");
        std::fs::write(machine.in_home("data/a.txt"), b"a").expect("a");
        machine.set("SMZY_RACE_LANDING_LINK_TO", machine.in_home("outside"));
        let point = APointLending::over(&machine).await;
        let _ = an_elevated_act_is_made(
            &point.sessions,
            &rename(machine.path_bytes("data/a.txt"), machine.path_bytes("data/b.txt")),
            HowElevationIsObtained::Passwordless,
        )
        .await;
        assert_eq!(std::fs::read_dir(machine.in_home("outside")).expect("outside").count(), 0, "nothing landed outside");
    }

    /// **Sol's finding 5: the password never reaches the act's own input**,
    /// even where a credential cached elsewhere would let the elevating
    /// command skip reading it, and it is sent once.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn the_password_never_reaches_the_acts_input_and_is_sent_once() {
        let Some(machine) = AMachine::standing("input", true) else {
            println!("SKIPPED: no sftp-server on this system");
            return;
        };
        std::fs::create_dir_all(machine.in_home("data")).expect("the folder");
        std::fs::write(machine.the_cache(), b"").expect("a credential cached by something else");
        machine.set("SMZY_STAND_IN_CACHE", machine.the_cache());
        machine.set("SMZY_STAND_IN_INPUT_LOG", machine.the_input_log());
        let point = APointLending::over(&machine).await;
        let password = Origin::what_the_user_typed(THE_TEST_PASSWORD.as_bytes());
        let came = an_elevated_act_is_made(
            &point.sessions,
            &new_folder(machine.path_bytes("data/made")),
            HowElevationIsObtained::APasswordIsHeld(&password),
        )
        .await;
        assert_eq!(came, WhatTheElevatedActCameTo::Done);
        let input = std::fs::read(machine.the_input_log()).unwrap_or_default();
        assert!(
            !input.windows(THE_TEST_PASSWORD.len()).any(|window| window == THE_TEST_PASSWORD.as_bytes()),
            "the password reached the act's own input"
        );
        let carried = machine.what_the_stand_in_was_handed().lines().filter(|line| line.contains("-S")).count();
        assert_eq!(carried, 1, "the password was sent once");
    }

    /// **Sol's finding 6: no cached credential outlives the act, even where
    /// the clean-up after it never arrives.**
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn no_cached_credential_outlives_an_act_whose_clean_up_never_arrives() {
        let Some(machine) = AMachine::standing("cache", true) else {
            println!("SKIPPED: no sftp-server on this system");
            return;
        };
        std::fs::create_dir_all(machine.in_home("data")).expect("the folder");
        machine.set("SMZY_STAND_IN_CACHE", machine.the_cache());
        machine.set("SMZY_STAND_IN_K_NEVER_ARRIVES", "1");
        let point = APointLending::over(&machine).await;
        let password = Origin::what_the_user_typed(THE_TEST_PASSWORD.as_bytes());
        let came = an_elevated_act_is_made(
            &point.sessions,
            &new_folder(machine.path_bytes("data/made")),
            HowElevationIsObtained::APasswordIsHeld(&password),
        )
        .await;
        assert_eq!(came, WhatTheElevatedActCameTo::Done);
        assert!(!machine.the_cache().exists(), "a cached credential was left behind");
    }

    /// **Sol's finding 6, the other half: a credential cached by something
    /// else is never used as passwordless elevation.** The act asks for a
    /// password instead, and nothing is made.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_credential_cached_elsewhere_is_never_used_for_an_act() {
        let Some(machine) = AMachine::standing("borrowed", true) else {
            println!("SKIPPED: no sftp-server on this system");
            return;
        };
        std::fs::create_dir_all(machine.in_home("data")).expect("the folder");
        std::fs::write(machine.the_cache(), b"").expect("a credential cached by something else");
        machine.set("SMZY_STAND_IN_CACHE", machine.the_cache());
        let point = APointLending::over(&machine).await;
        let came = an_elevated_act_is_made(
            &point.sessions,
            &new_folder(machine.path_bytes("data/made")),
            HowElevationIsObtained::Passwordless,
        )
        .await;
        assert_eq!(came, WhatTheElevatedActCameTo::APasswordIsNeeded);
        assert!(!machine.in_home("data/made").exists());
    }

    /// **The runner's ruling 1: a folder made with admin access is handed to
    /// the login that asked**, and only that folder. The stand-in's
    /// `SUDO_GID` is a group the login is in that the parent does not carry,
    /// so the hand-over shows.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_folder_made_with_admin_access_is_handed_to_the_login() {
        use std::os::unix::fs::MetadataExt;
        let Some(machine) = AMachine::standing("handed", false) else {
            println!("SKIPPED: no sftp-server on this system");
            return;
        };
        std::fs::create_dir_all(machine.in_home("data")).expect("the folder");
        let parents = std::fs::metadata(machine.in_home("data")).expect("the folder").gid();
        let groups = std::process::Command::new("id").arg("-G").output().expect("the login's groups");
        let Some(other) = String::from_utf8_lossy(&groups.stdout)
            .split_whitespace()
            .filter_map(|one| one.parse::<u32>().ok())
            .find(|one| *one != parents)
        else {
            println!("SKIPPED: the login is in one group only");
            return;
        };
        machine.set("SMZY_STAND_IN_GID", other.to_string());
        let point = APointLending::over(&machine).await;
        let came = an_elevated_act_is_made(
            &point.sessions,
            &new_folder(machine.path_bytes("data/made")),
            HowElevationIsObtained::Passwordless,
        )
        .await;
        assert_eq!(came, WhatTheElevatedActCameTo::Done);
        assert_eq!(std::fs::metadata(machine.in_home("data/made")).expect("made").gid(), other, "handed over");
        assert_eq!(std::fs::metadata(machine.in_home("data")).expect("the folder").gid(), parents, "only that folder");
    }

    /// **Sol's finding 9: an elevated listing that fails says why** — here a
    /// `find` with no `-printf`, as a machine without GNU's would answer.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_listing_that_fails_keeps_the_servers_reason() {
        let Some(machine) = AMachine::standing("reason", false) else {
            println!("SKIPPED: no sftp-server on this system");
            return;
        };
        std::fs::create_dir_all(machine.in_home("data")).expect("the folder");
        machine.set("SMZY_FIND_HAS_NO_PRINTF", "1");
        let point = APointLending::over(&machine).await;
        let came = an_elevated_act_is_made(
            &point.sessions,
            &AnElevatedAct::AListing { folder: machine.path_bytes("data") },
            HowElevationIsObtained::Passwordless,
        )
        .await;
        let WhatTheElevatedActCameTo::TheFolderWasNotRead(WhyTheFolderWasNotRead::TheServerSaid(said)) = came else {
            panic!("no reason: {came:?}");
        };
        assert!(said.contains("find: unknown predicate `-printf'"), "{said}");
    }

    /// **Sol's second look: a folder put in place of the new one in the moment
    /// after it is made is never handed to the login.** The racer moves a
    /// folder that holds a file into the new name; its group stays as it was.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_folder_put_in_place_of_the_new_one_is_never_handed_over() {
        use std::os::unix::fs::MetadataExt;
        let Some(machine) = AMachine::standing("substitute", false) else {
            println!("SKIPPED: no sftp-server on this system");
            return;
        };
        std::fs::create_dir_all(machine.in_home("data/holding")).expect("the racer's folder");
        std::fs::write(machine.in_home("data/holding/inside.txt"), b"x").expect("what it holds");
        let theirs = std::fs::metadata(machine.in_home("data/holding")).expect("theirs").gid();
        let groups = std::process::Command::new("id").arg("-G").output().expect("the login's groups");
        let Some(other) = String::from_utf8_lossy(&groups.stdout)
            .split_whitespace()
            .filter_map(|one| one.parse::<u32>().ok())
            .find(|one| *one != theirs)
        else {
            println!("SKIPPED: the login is in one group only");
            return;
        };
        machine.set("SMZY_STAND_IN_GID", other.to_string());
        machine.set("SMZY_RACE_NEW_FOLDER_FROM", machine.in_home("data/holding"));
        let point = APointLending::over(&machine).await;
        let came = an_elevated_act_is_made(
            &point.sessions,
            &new_folder(machine.path_bytes("data/made")),
            HowElevationIsObtained::Passwordless,
        )
        .await;
        assert_eq!(came, WhatTheElevatedActCameTo::MadeButOwnedByRoot);
        assert_eq!(std::fs::metadata(machine.in_home("data/made")).expect("the racer's").gid(), theirs, "not handed over");
    }

    /// **Sol's second look, point 9: a failure after the elevating command
    /// let the act in, before the act began, is said in the server's words** —
    /// never as a password needed or admin access refused. Here the paths file
    /// is gone by the time root opens it.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_failure_before_the_act_began_is_never_read_as_a_refusal() {
        let Some(machine) = AMachine::standing("before", false) else {
            println!("SKIPPED: no sftp-server on this system");
            return;
        };
        std::fs::create_dir_all(machine.in_home("data")).expect("the folder");
        std::fs::write(machine.in_home("data/a.txt"), b"a").expect("a");
        machine.set("SMZY_STAND_IN_TAKES_THE_PATHS_FILE", "1");
        let point = APointLending::over(&machine).await;
        let came = an_elevated_act_is_made(&point.sessions, &delete(machine.path_bytes("data/a.txt")), HowElevationIsObtained::Passwordless).await;
        let WhatTheElevatedActCameTo::Failed { the_servers_words } = came else {
            panic!("read as something else: {came:?}");
        };
        assert!(the_servers_words.contains("files-elevated-paths"), "{the_servers_words}");
        assert!(machine.in_home("data/a.txt").exists());
    }

    /// **Sol's second look, points 5 and 6: Files' probe never leans on a
    /// credential cached by something else.** Every `sudo` the offer's
    /// readings send carries `-k`; with only a cached credential to go on,
    /// no offer is drawn as passwordless.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn the_offers_probe_never_leans_on_a_cached_credential() {
        let Some(machine) = AMachine::standing("probe", true) else {
            println!("SKIPPED: no sftp-server on this system");
            return;
        };
        std::fs::write(machine.the_cache(), b"").expect("a credential cached by something else");
        machine.set("SMZY_STAND_IN_CACHE", machine.the_cache());
        let point = APointLending::over(&machine).await;
        let offer = smoozy_desktop::surface::files::the_offer_at_the_refusal(
            &point.sessions,
            smoozy_file_browser::permission::ElevatableAct::Remove,
        )
        .await;
        assert_ne!(offer, smoozy_file_browser::permission::Offer::OfferedAndNothingIsAsked);
        let handed = machine.what_the_stand_in_was_handed();
        assert!(handed.lines().any(|line| line == "-k -n true"), "the probe was sent: {handed:?}");
        assert!(handed.lines().all(|line| line.starts_with("-k ")), "a probe without -k: {handed:?}");
    }
}
