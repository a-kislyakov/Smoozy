//! §1.2 and §1.3's boot registrations, composed — the user unit on a systemd
//! machine and the provider's hook on a container.
//!
//! # Why this file exists
//!
//! **K16 makes Smoozy the author of the registration's content by value**, and
//! until this file was written nothing in the workspace composed that text.
//! [`crate::registration`] holds what a registration **is** and what confirms one;
//! [`crate::acting::registration`] writes it, enables it, grants lingering and
//! reads it back. **Both took the content as something they were handed**, and the
//! only thing in the workspace resembling a service unit was two test fixtures
//! reading `[Service]` and nothing else.
//!
//! # The two are one shape and two texts
//!
//! **K4** on a systemd machine: a user unit under the supervisor login's own
//! configuration directory, enabled, with lingering granted to that login by name.
//! **K10** on a container: the provider's own boot hook at the path the provider
//! runs, installed by root, starting S5's keeper.
//!
//! **Both name the active-package pointer and neither names a package version.**
//! That is G7 — *"the boot registration is written to start whatever the
//! active-package pointer names, rather than to name a package version"* — and it
//! is what makes the commit one deciding write instead of two.
//!
//! # What each text must do, clause by clause
//!
//! - **K4**, the unit itself, enabled, needing no elevated access.
//! - **K18**, and it is a requirement of the registration rather than a detail of
//!   it: *"The unit is written so that ending the supervisor does not end the
//!   workers it started."* The service manager's default takes the whole control
//!   group down with the main process, so a unit written to K4 alone destroys a
//!   running render on every supervisor restart. **The plan requires the first of
//!   the machine's two routes** — the unit's kill behaviour confined to the main
//!   process — *"because it keeps K4's arrangement exactly as it stands and adds
//!   one directive"*.
//! - **S4**, the unconditional restart directive, *"correct for the supervisor and
//!   forbidden for a worker"*.
//! - **K10 and K11** on a container: root installs the hook, and **the hook drops
//!   privilege**.
//! - **S5**, the keeper: *"its only job is to start the supervisor and start it
//!   again after an unexpected exit"*, and it *"holds no policy of its own"*.
//!
//! # Why the container's keeper is a shell loop and not a role of the program
//!
//! **Three reasons, and the third is the one that settles it.** `The shape` names
//! three executable classes and a keeper is not one of them, so a fourth would be
//! invented here. S5 gives the keeper no policy at all, which is a loop rather
//! than a program. **And the keeper must outlive the package it starts**: G25 ends
//! the running supervisor after the pointer has moved, and a keeper that was
//! itself the package's own program would be a second thing the commit had to
//! replace. A loop naming the pointer starts whatever the pointer now names,
//! which is the same property G7 gives the unit.
//!
//! # What is deliberately not written into the unit
//!
//! **The restart limiter's two settings.** §16 item 24 records that *"the
//! placement never chooses the supervisor unit's restart-limiter settings, and two
//! other documents have been sized on the default"*, and that **this document must
//! say what the placement writes** before anything is resized on it. It does not
//! say. So nothing is chosen here: the unit carries neither setting and the
//! machine's own default stands, which keeps §3.2's asymmetry between the two
//! machine kinds exactly as the plan records it — observed rather than arranged.
//! **What that costs is item 12's recorded break and nothing new**: on a systemd
//! machine whose state mount fills, the limiter gives up and the unit does not
//! recover when space is freed, and `Desktop app and delivery`'s automatic
//! supervisor-unavailable repair is the whole of the user's recovery.
//!
//! # The publications behind the two texts, all read 2026-09-05
//!
//! - `systemd.service(5)`,
//!   `https://man7.org/linux/man-pages/man5/systemd.service.5.html`. Of `Type=`:
//!   *"the service manager will consider the unit started immediately after the
//!   main service binary has been executed … systemctl start command lines for
//!   exec services will report failure when the service's binary cannot be invoked
//!   successfully"*, against `simple`, which *"will report success even if the
//!   service's binary cannot be invoked successfully"*. Of `Restart=`: *"If set to
//!   always, the service will be restarted regardless of whether it exited cleanly
//!   or not, got terminated abnormally by a signal, or hit a timeout."* Of
//!   `ExecStart=`: *"Unless Type= is oneshot, exactly one command must be given"*
//!   and *"the process started via this command line will be considered the main
//!   process of the daemon"*.
//! - `systemd.kill(5)`,
//!   `https://man7.org/linux/man-pages/man5/systemd.kill.5.html`. *"If set to
//!   control-group, all remaining processes in the control group of this unit will
//!   be killed on unit stop … If set to process, only the main process itself is
//!   killed (not recommended!)"*, and *"Defaults to control-group."*
//! - `systemd.unit(5)`,
//!   `https://man7.org/linux/man-pages/man5/systemd.unit.5.html`. The user unit
//!   search path begins `~/.config/systemd/user/*`. Of `WantedBy=`: *"A symbolic
//!   link is created in the .wants/ … directory of each of the listed units when
//!   this unit is installed by systemctl enable."* And of specifiers: *"Many
//!   settings resolve specifiers which may be used to write generic unit files
//!   referring to runtime or unit parameters that are replaced when the unit files
//!   are loaded."*
//! - `systemd.special(7)`,
//!   `https://man7.org/linux/man-pages/man7/systemd.special.7.html`, of
//!   `default.target` under the user service manager: *"This is the main target of
//!   the user service manager, started by default when the service manager is
//!   invoked … default.target is similar to multi-user.target in the system
//!   instance, but it is a real unit, not an alias."*
//! - `su(1)`, `https://man7.org/linux/man-pages/man1/su.1.html`. *"su - run a
//!   command with substitute user and group ID"*; of `-c`: *"Pass command to the
//!   shell with the -c option. Creates a new session via setsid(2)."*; of `-s`:
//!   *"Run the specified shell instead of the default. If the target user has a
//!   restricted shell … the --shell option and the SHELL environment variables are
//!   ignored unless the calling user is root."*; and of the default form: *"su
//!   defaults to not change the current directory and to only set the environment
//!   variables HOME and SHELL (plus USER and LOGNAME if the target user is not
//!   root)."*
//!   **That address is a mirror rather than the project's own publication.** When
//!   the section further down was written the four sentences above were checked
//!   against util-linux's own copy of the page, at tag **v2.41**, and **three of
//!   them are there word for word** — the name, the `-s` paragraph and the default
//!   form. **The fourth is not, and which util-linux it is of is worth saying**:
//!   the `-c` entry gained *"Creates a new session via setsid(2). Refer to
//!   --session-command to keep the same session."* after v2.41 and it is in no
//!   released tag this project checked — v2.37.4, v2.39.3, v2.40.2 and v2.41 all
//!   carry the one line *"Pass command to the shell with the -c option."* **The
//!   behaviour is in the released source all the same**, which is where the claim
//!   now rests: `login-utils/su-common.c` at v2.41 sets *same session* only where
//!   `--session-command` was given, or no command was, or the target login is root,
//!   and calls `setsid()` in every other case — which is this hook's case, a command
//!   given to a non-root login. **util-linux's own address is the one to cite for
//!   anything new**, and it is written out in that section.
//!
//! # The two programs the keeper's own trimming runs, read 2026-09-06
//!
//! - `tail(1)`, `https://man7.org/linux/man-pages/man1/tail.1.html`. Of the
//!   program: *"tail - output the last part of files"*. Of `-c`: *"-c,
//!   --bytes=[+]NUM output the last NUM bytes; or use -c +NUM to output starting
//!   with byte NUM of each file"*. **Without the plus it is the last NUM bytes**,
//!   which is what makes *oldest bytes dropped* one command rather than a
//!   program.
//! - `mv(1)`, `https://man7.org/linux/man-pages/man1/mv.1.html`. *"mv - move
//!   (rename) files"*, with the form `mv [OPTION]... [-T] SOURCE DEST`.
//!
//! **That either is on the machine is the same unestablished per-image ground
//! `su` stands on**, below, and it fails the same way: the trim does nothing, the
//! supervisor's words keep being appended, and nothing else is affected — the
//! keeper still starts the supervisor and the supervisor's words still land in
//! the file. **The failure is a file that grows rather than a machine that stops.**
//!
//! **`--login` is deliberately not used, against that page's own recommendation,
//! and the reason is this project's own observation.** On a container rental
//! *"`su - smoozy` … reports `-bash: /root/.bash_profile: Permission denied`, so
//! the ordinary login's environment is not cleanly its own"* — the hook runs as
//! root, and the login form reads root's own start-up material. The keeper needs
//! nothing a login shell supplies, and the default form's four environment
//! variables are exactly what the page above says it sets. Label: **observed in a
//! test**, on one rented container, 2026-08-16.
//!
//! **That `su` is on the machine at all is the same unestablished per-image ground
//! every command-channel step already stands on** (`Server protocol and transfers`
//! §12 items 3 and 11). It was present on the container image read on 2026-08-22
//! and on the one read on 2026-08-16, and a program that is not there answers with
//! its absence.
//!
//! # Does `su` read its options when they come after the user name? It does
//!
//! **Why it is asked, because the whole of the container boot return rests on it.**
//! The keeper's line is `su <login> -s /bin/sh -c '<loop>'`, with both options
//! written **after** the user operand, and the only form this project has ever
//! observed on a real container is the other one — `su - smoozy`, options first.
//! **If the options were not read as options they would be handed to the login's
//! own shell as arguments.** A shell given `-s` reads its commands from standard
//! input, which the hook has pointed at `/dev/null`, so it would do nothing and end
//! at zero: the keeper would never start, the supervisor would never run, and
//! **K15's read-back would confirm the placement anyway**, the hook's bytes reading
//! back exactly as written. That is K20's blind spot rather than a new one.
//!
//! **The reading, and it takes three steps because the synopsis alone does not
//! settle it.** All read 2026-09-06.
//!
//! 1. **util-linux's own manual page for `su`, its SYNOPSIS**, read from that
//!    project's own repository at tag **v2.41**, the page's own source, at
//!    `https://raw.githubusercontent.com/util-linux/util-linux/v2.41/login-utils/su.1.adoc`.
//!    The line is written `*su* [options] [*-*] [_user_ [_argument_...]]` and
//!    renders as `su [options] [-] [user [argument...]]`, and the DESCRIPTION adds
//!    *"When _user_ is specified, additional __argument__s can be supplied, in which
//!    case they are passed to the shell."*
//!    **Said plainly, and not resolved by inference: that is ambiguous.** It writes
//!    the options before the user and it says what becomes of *arguments* written
//!    after the user. **It says nothing at all about an *option* written after the
//!    user**, and it is not read here as forbidding one.
//! 2. **The same page settles it by its own worked example.** Of `--pty`: *"The
//!    entire session can also be moved to the background (e.g., `su --pty - user -c
//!    application &`)."* — written in the source as
//!    `*su --pty* **-** __user__ *-c* _application_ *&*`. **That puts `-c` after the
//!    user operand**, which is the hook's own shape, in util-linux's own words.
//! 3. **And that project's source says why it works**, which is where the reading is
//!    allowed to go next and is still that project's publication.
//!    `login-utils/su-common.c` at the same tag parses with
//!    `getopt_long(argc, argv, "c:fg:G:lmpPTs:u:hVw:", longopts, NULL)` and only
//!    afterwards takes the user from `argv[optind]`. **The option string begins with
//!    a letter — not a `+` and not a `-` — and nothing in that file sets
//!    `POSIXLY_CORRECT`.** The same call with the same absence of a `+` is in
//!    v2.37.4, v2.39.3 and v2.40.2, which spans what Debian and Ubuntu carry.
//!
//! **What that absence means is the C library's, and it says it in one sentence.**
//! The GNU C Library manual, **glibc 2.44**, 26.2.1 *Using the getopt function*, at
//! `https://sourceware.org/glibc/manual/latest/html_node/Using-Getopt.html`, read
//! 2026-09-06: *"The default is to permute the contents of argv while scanning it so
//! that eventually all the non-options are at the end. This allows options to be
//! given in any order, even with programs that were not written to expect this."*
//! And the only two ways out of that default: *"POSIX demands the following
//! behavior: the first non-option stops option processing. This mode is selected by
//! either setting the environment variable POSIXLY_CORRECT or beginning the options
//! argument string with a plus sign ('+')."* `getopt_long` takes the same string on
//! the same terms — 26.2.3: *"The argument shortopts describes the short options to
//! accept, just as it does in getopt."* `Server Setup` M1a limits the supported
//! families to Ubuntu and Debian, and both are that library's.
//!
//! **So the composed form is right and nothing in it changes.** Label:
//! **documented**, from util-linux's and the GNU C Library's own publications, read
//! 2026-09-06.
//!
//! **The one thing that would break it, written down rather than assumed away.**
//! `POSIXLY_CORRECT` set in the environment the hook runs under stops the
//! permuting, and then the two options become the login shell's own arguments and
//! the keeper never starts. **Nothing Smoozy does sets it**; the hook is run by the
//! provider's boot chain, and what that chain's environment holds is the same
//! unestablished per-image ground `su`'s own presence stands on. The failure would
//! be silent on the machine, which is why it is recorded here.
//!
//! # Where a supervisor's own words go on a systemd machine
//!
//! **The unit writes no `StandardOutput=` and no `StandardError=`**, so where the
//! supervisor's two streams land is the service manager's own default — and this is
//! a **user** unit, which is the whole of the question. **They go to the journal**,
//! and they go nowhere near `/dev/null`. Read 2026-09-06 from systemd's own manual
//! pages, at tag **v257** of systemd's own repository, which is where those pages
//! are written.
//!
//! - `systemd.exec(5)`,
//!   `https://raw.githubusercontent.com/systemd/systemd/v257/man/systemd.exec.xml`.
//!   Of `StandardOutput=`: *"In other cases, this setting defaults to the value set
//!   with DefaultStandardOutput= in systemd-system.conf(5), which defaults to
//!   journal."* The *other cases* are everything but the branch stated just above
//!   it — *"If StandardInput= is set to one of tty, tty-force, tty-fail, socket, or
//!   fd:name, this setting defaults to inherit"* — and **this unit sets no
//!   `StandardInput=`**, whose own default the same page gives as *"This setting
//!   defaults to null"*. Of `StandardError=`: *"This setting defaults to the value
//!   set with DefaultStandardError= in systemd-system.conf(5), which defaults to
//!   inherit"*, and `inherit` there is *"the file descriptor used for standard
//!   output is duplicated for standard error"* — **so both streams end at the same
//!   place**. Of `journal` itself: *"connects standard output with the journal,
//!   which is accessible via journalctl(1)"*.
//! - **The user half, which is the part the system case cannot answer.** The page
//!   that supplies the default is the same page for both managers: its own names
//!   are `systemd-system.conf` **and `systemd-user.conf`**, at
//!   `https://raw.githubusercontent.com/systemd/systemd/v257/man/systemd-system.conf.xml`,
//!   and it says *"when run as a user instance, it interprets the configuration
//!   file user.conf"*. **The defaults are written into the option names it lists** —
//!   `DefaultStandardOutput=journal` and `DefaultStandardError=inherit` — with no
//!   second value for the user manager. **Where a setting on that page does differ
//!   between the two managers it says so in terms**, as the default timeouts do,
//!   given as one value *"in the system manager"* and another *"in the user
//!   manager"*; these two are not among them.
//!
//! **So no directive is added to the unit and none is wanted.** Label:
//! **documented**, from systemd's own publication, read 2026-09-06.
//!
//! **Two limits, recorded rather than glossed.** Reading a user's own journal back
//! wants persistent storage: `journalctl(1)`, same tag, *"Note that --user only
//! works if persistent logging is enabled, via the Storage= setting in
//! journald.conf(5)."* **And nothing in Smoozy reads that journal** — Smoozy's own
//! machine-side event journal is `State, identity and recovery` §4's and is a
//! different store, which the supervisor writes into and which no process stream
//! reaches — so this is where a person looking at the machine finds the
//! supervisor's words, not where Smoozy finds them. **That is exactly the standing the
//! container's own bounded file has**, and neither is claimed as more.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Job files and outputs` L13. `Desktop app and
//! delivery` — the automatic supervisor-unavailable repair.
//!
//! **A marker this plan also defines is never listed here and is written out with
//! its document at the site**, because a header list cannot tell the two apart and
//! a bare one there would resolve to a real and wrong clause.

use crate::machine_kind::MachineKind;
use crate::registration::{
    BootRegistrationRecord, TheContainerRegistration, TheSystemdRegistration,
};
use crate::the_program::THE_SUPERVISOR_WORD;

/// **The unit's own name**, which is what K4's enable and G25's start both name.
///
/// **It carries no package version**, for G7's reason: the registration must not
/// change when the version does.
pub const THE_UNIT_NAME: &str = "smoozy-supervisor.service";

/// **Where a user unit is written**, relative to the supervisor login's own home.
///
/// K4: *"a user unit under the supervisor login's own configuration directory"*.
/// `systemd.unit(5)`'s user search path begins `~/.config/systemd/user/*`.
pub const THE_USER_UNIT_DIRECTORY_UNDER_A_HOME: &str = ".config/systemd/user";

/// **The file a supervisor's own words go to on a container, and it is a
/// stand-in.**
///
/// **The keeper closes its own two streams off and the supervisor inherits
/// them**, so before this file was named every reason a supervisor refused to
/// start — and [`crate::the_program::THE_FIXED_ENTRY_POINT_NAME`]'s own
/// still-owes sentence, written precisely so the absence could not be missed —
/// was written nowhere at all.
///
/// **What would really carry a supervisor's words to a reader is the machine-side
/// event journal**, and this file is a stand-in for it: a plain file with a fixed
/// name, in the package area, bounded, that nothing in Smoozy reads. **The stand-in
/// is still needed now that the journal is built, and the reason has changed** —
/// this said the journal was not built until 2026-09-12, and it is; what a
/// process's own two streams carry is every reason a supervisor refused to start
/// before it had a journal to write into, and `State, identity and recovery` E6a
/// caps an entry too tightly for the standing declaration besides. **It is not `Reporting a problem`'s journal tail**, which is a request
/// over the ordinary connection against Smoozy's own event store.
///
/// **The package area is where it goes because that is where this crate already
/// puts everything the machine keeps about its own package** — the phase record
/// and the endpoint's recorded path are both [`crate::the_program::records`],
/// and a supervisor's own account of itself is the same kind of thing. It is
/// deliberately not a job's material, which is `Job files and outputs`' and is
/// somewhere else.
pub const THE_FILE_A_SUPERVISORS_OWN_WORDS_GO_TO: &str = "supervisor-words";

/// **The most a supervisor's own words keep, in bytes, oldest dropped first.**
///
/// **A Build decision and not a plan figure**, the same standing as the interval
/// below. What it is for is that a machine nobody is watching cannot fill its own
/// disk with its own complaints: the keeper starts the supervisor again for as
/// long as the rental lasts, and a supervisor that refuses to start writes the
/// same paragraph every second. One mebibyte, which holds several thousand of
/// those lines and is nothing against a rented machine's disk.
pub const THE_MOST_A_SUPERVISORS_OWN_WORDS_KEEP_IN_BYTES: u64 = 1024 * 1024;

/// **How long S5's keeper waits between two starts of the supervisor.**
///
/// **A Build decision and not a plan figure**, the same standing every other
/// interval in this crate carries. It decides nothing and bounds nothing: S5 gives
/// the keeper no policy, and this only keeps a supervisor that cannot start from
/// being started as fast as the machine can fork. One second.
pub const BETWEEN_TWO_STARTS_BY_THE_KEEPER_IN_SECONDS: u32 = 1;

/// Why a value could not go into a registration's text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThatValueDoesNotGoIntoARegistration {
    /// What was handed in.
    pub value: String,
    /// What is wrong with it, in the machine's terms rather than in a rule's.
    pub why: String,
}

impl ThatValueDoesNotGoIntoARegistration {
    pub fn as_text(&self) -> String {
        format!("{}: {}", self.value, self.why)
    }
}

/// **The one alphabet both texts admit, and it is deliberately narrower than
/// either would allow on its own.**
///
/// A unit file resolves specifiers, so a `%` in a path is replaced rather than
/// read; a unit's `ExecStart=` splits its command line on whitespace; and the
/// keeper's loop sits inside a single-quoted shell word, which a quote of its own
/// would end. **Rather than three escaping rules the refusal is one**: a path
/// carries lower-case and upper-case letters, digits, the separator, a hyphen, an
/// underscore and a dot, and nothing else.
///
/// **Nothing Smoozy composes needs more.** `Server protocol and transfers` §1.1
/// draws a synthesised path from *"lower-case letters, digits, hyphens and the
/// path separator under an installation-owned prefix"*, which this admits whole.
/// **A path outside it is refused before the registration is written**, which is a
/// loud failure before the commit rather than a unit that starts the wrong thing.
fn a_path_that_goes_into_a_registration(
    path: &str,
) -> Result<&str, ThatValueDoesNotGoIntoARegistration> {
    let refuse = |why: &str| {
        Err(ThatValueDoesNotGoIntoARegistration {
            value: path.to_string(),
            why: why.to_string(),
        })
    };
    if !path.starts_with('/') {
        return refuse(
            "a boot registration names an absolute path, and this one does not begin at the root",
        );
    }
    if path.ends_with('/') {
        return refuse("this names a directory rather than the program to start");
    }
    if let Some(bad) = path
        .chars()
        .find(|held| !(held.is_ascii_alphanumeric() || matches!(held, '/' | '-' | '_' | '.')))
    {
        return refuse(&format!(
            "the character {bad:?} does not go into a unit file's command line or into the \
             keeper's own loop, and a registration is never escaped into working"
        ));
    }
    Ok(path)
}

/// **A login name that goes into the container's hook.**
///
/// The name is the machine's own — the effective user this connection's login maps
/// to, as the machine's user database gives it — and it crosses into the hook as
/// **file content** rather than on a command string, so `Server protocol and
/// transfers` §1.1's carriage rule is not what governs it. What governs it is the
/// same one-alphabet refusal above: a name carrying a quote would end the keeper's
/// own quoted word.
fn a_login_that_goes_into_a_registration(
    login: &str,
) -> Result<&str, ThatValueDoesNotGoIntoARegistration> {
    let refuse = |why: &str| {
        Err(ThatValueDoesNotGoIntoARegistration {
            value: login.to_string(),
            why: why.to_string(),
        })
    };
    if login.is_empty() {
        return refuse("the machine named no login for the hook to drop privilege to");
    }
    if let Some(bad) = login
        .chars()
        .find(|held| !(held.is_ascii_alphanumeric() || matches!(held, '-' | '_' | '.')))
    {
        return refuse(&format!(
            "the character {bad:?} does not go into the hook's own quoted command"
        ));
    }
    Ok(login)
}

/// **K4 and K18 and S4: the systemd machine's unit, composed.**
///
/// `pointer_path` is the active-package pointer's own path, which G7 makes the one
/// thing the registration names.
///
/// **`Type=exec` rather than `simple`, and the difference is loud against silent.**
/// The publication above says a `simple` start *"will report success even if the
/// service's binary cannot be invoked successfully"* while an `exec` start *"will
/// report failure"*. G8 refuses a package version that cannot be started through
/// the fixed path the registration names, and G25's start is what G13 reads as
/// *did not answer* where it produces no supervisor — so a start that reported
/// success for a binary that never ran would hide exactly the failure both clauses
/// exist to catch.
///
/// **`KillMode=process` is K18 and the publication's own caution rides with it.**
/// `systemd.kill(5)` says of that value *"only the main process itself is killed
/// (not recommended!)"* and warns that it *"allows processes to escape the service
/// manager's lifecycle and resource management"*. **That escape is the
/// requirement**: a render that outlives the supervisor that started it is what
/// K18 was written for, observed both ways — with the default in force, killing
/// the supervisor under a live render took the worker and Blender with it; with
/// the unit corrected, the render *"finished under a supervisor it never met"*,
/// byte-for-byte identical to an uninterrupted run.
///
/// **No `User=` is written and none is wanted.** This is the login's own user
/// unit, so the service manager is already that login's, and every worker it
/// starts runs as the same login (`Job files and outputs` L13).
pub fn the_systemd_unit(
    pointer_path: &str,
) -> Result<Vec<u8>, ThatValueDoesNotGoIntoARegistration> {
    let pointer = a_path_that_goes_into_a_registration(pointer_path)?;
    Ok(format!(
        "[Unit]\n\
         Description=Smoozy server supervisor\n\
         \n\
         [Service]\n\
         Type=exec\n\
         ExecStart={pointer} {THE_SUPERVISOR_WORD}\n\
         Restart=always\n\
         KillMode=process\n\
         \n\
         [Install]\n\
         WantedBy=default.target\n"
    )
    .into_bytes())
}

/// **K10, K11 and S5: the container's boot hook, composed.**
///
/// `pointer_path` is the active-package pointer's own path; `the_supervisors_login`
/// is the machine's own name for the login the supervisor runs as.
///
/// **The hook runs as root** — observed on the one provider that runs a hook at a
/// path at all — **and drops privilege**, which is the whole of what K11 says the
/// clause is answerable as. Every worker the hook's supervisor starts runs as that
/// same non-root login (`Job files and outputs` L13).
///
/// **What it starts is S5's keeper rather than the supervisor directly**, and the
/// keeper is the loop: start the supervisor, and start it again when it ends. It
/// never starts, stops or restarts a worker, never reads a store, and never
/// decides an outcome.
///
/// **It does not distinguish a crash from a deliberate end, and that is S6.** The
/// transaction ends the running supervisor after the pointer has moved and expects
/// the restarter to start the one the pointer now names, so a keeper that declined
/// to restart after a clean exit would strand every runtime upgrade at G13's *did
/// not answer* branch.
///
/// **It is detached and it returns.** The hook is one line in the provider's own
/// boot chain and other things may follow it, so the keeper is backgrounded and
/// its own standard streams are closed off. `su -c` *"Creates a new session via
/// setsid(2)"*, which is the same detachment observed to outlive a connection on
/// this machine kind — a process took its own session, the connection was closed,
/// and on a new connection it was still running, reparented to the first process.
///
/// **The keeper's own detachment stays exactly as it is, and the supervisor's two
/// streams are taken out from under it.** Before that they were the keeper's, so
/// they went to nothing: `WHAT_A_SUPERVISOR_STILL_OWES` and every reason a
/// supervisor refused to start were written nowhere on this machine kind. They now
/// go to [`THE_FILE_A_SUPERVISORS_OWN_WORDS_GO_TO`] in the package area,
/// **appended, and trimmed back to [`THE_MOST_A_SUPERVISORS_OWN_WORDS_KEEP_IN_BYTES`]
/// after every supervisor ends**, so the oldest bytes are the ones that go.
/// **It is a stand-in for the machine-side event journal**, for the reason written
/// at [`THE_FILE_A_SUPERVISORS_OWN_WORDS_GO_TO`], and the hook says so in its own
/// text.
///
/// **This is the whole text of a hook that holds nothing but Smoozy's**, which is
/// what a provider that shipped no hook at all gets. Where the provider's own
/// file already holds something, [`the_container_hook_keeping`] is what is
/// written, and Smoozy's part of it is the same [`the_smoozy_part_of_a_hook`].
pub fn the_container_hook(
    pointer_path: &str,
    the_supervisors_login: &str,
) -> Result<Vec<u8>, ThatValueDoesNotGoIntoARegistration> {
    the_container_hook_keeping(b"", pointer_path, the_supervisors_login)
}

/// **K10's path: the provider's own boot hook, and no other file.**
///
/// **Vast alone documents a file at a path**, and this is it: *"Place startup
/// commands in `/root/onstart.sh`. This script runs automatically on container
/// startup."* — `https://docs.vast.ai/guides/reference/faq/instances`, read
/// 2026-08-18 by the plan and again 2026-09-21 at step 10c, where the page reads
/// the same. **A provider that runs no hook at this path is caught by nothing**,
/// because the content reads back exactly as written: `Rented-server runtime` §16
/// item 16, K10b and K14, and it is not answered here.
pub const THE_PROVIDERS_HOOK_PATH: &str = "/root/onstart.sh";

/// **The line Smoozy's part of a hook begins with, and the line it ends with.**
///
/// **Whatever the provider's own file already held is kept**, and these two lines
/// are what let a later placement find Smoozy's part again and replace it rather
/// than add a second one. **This is K10c** — *"Whatever the provider's hook
/// already holds is kept, and Smoozy's part is placed within it rather than over
/// it"* — ~~a Build decision, recorded in `Rented-server runtime`'s decisions
/// record~~, **written into the plan on 2026-09-21**, when no clause said it. Its
/// ground, as it was and as K10c gives it: the plan's own reading of the provider's page is
/// *"Add to `/root/onstart.sh` for persistence across restarts"* (`Six machines`),
/// the file is written from the rental's own start-up field (K10b), and a
/// placement that wrote over it would take away whatever the person renting the
/// machine had put there.
pub const THE_LINE_SMOOZYS_PART_BEGINS_WITH: &str =
    "# >>> Smoozy's boot registration begins. Everything outside these two lines is not Smoozy's. >>>";
/// See [`THE_LINE_SMOOZYS_PART_BEGINS_WITH`].
pub const THE_LINE_SMOOZYS_PART_ENDS_WITH: &str = "# <<< Smoozy's boot registration ends. <<<";

/// **The file S5's keeper takes its lock on, in the package area.**
///
/// `Rented-server runtime` S5: *"the keeper takes an exclusive open-file-description
/// lock on one fixed file in the package area before it starts anything, and holds
/// it for as long as it runs. A keeper that cannot take the lock ends at once and
/// starts nothing."* **It is the keeper's own and not G1a's package-transaction
/// lock file**, which a keeper holding for its whole life would make every later
/// transaction wait on — so it has a name of its own.
pub const THE_FILE_THE_KEEPER_LOCKS: &str = "keeper-lock";

/// **S5's keeper, written once — the loop both the hook and G25's start step
/// run.**
///
/// `pointer_path` is the active-package pointer's own path. **The text is a
/// single line with no single quote in it**, because the hook carries it inside
/// `su`'s own quoted command.
///
/// # The lock, and every outside command in it read from its own publication
///
/// **The command S5 leaves to Build is util-linux's `flock`, in the form its own
/// page gives for shell scripts.** `flock(1)`, util-linux's own manual source at
/// tag **v2.41**,
/// `https://raw.githubusercontent.com/util-linux/util-linux/v2.41/sys-utils/flock.1.adoc`,
/// read 2026-09-21:
///
/// - the third synopsis form, *"`flock [options] number`"*, which *"uses an open
///   file by its file descriptor number"*;
/// - of `-n`: *"Fail rather than wait if the lock cannot be immediately
///   acquired"*, the exit status then being *"1 by default"*;
/// - and the page's own example, which is this text's shape: *"`(; flock -n 9 ||
///   exit 1; # ... commands executed under lock ...; ) 9>/var/lock/mylockfile`
///   — The form is convenient inside shell scripts. … using > or >> allows the
///   lockfile to be created if it does not already exist"*.
///
/// **`flock(1)` manages `flock(2)` locks** (*"This utility manages flock(2) locks
/// from within shell scripts"*), which is the lock S5 names and G1a's own source
/// already reads: *"Locks created by flock() are associated with an open file
/// description"*, released *"when all such file descriptors have been closed"*.
/// So **a keeper that dies leaves the lock free**, with no timer. **A keeper that
/// cannot take it ends at zero** (`|| exit 0`): a second keeper is not a failure,
/// it is the same restarter reached by the other door.
///
/// **The supervisor is started with descriptor 9 closed**, which is what keeps the
/// lock the keeper's alone. The POSIX shell standard, *Shell Command Language*,
/// `https://pubs.opengroup.org/onlinepubs/9799919799/utilities/V3_chap02.html`,
/// read 2026-09-21: of a compound command, *"Each redirection shall apply to all
/// the commands within the compound command that do not explicitly override that
/// redirection"* — so every command inside inherits descriptor 9 — and of
/// `[n]>&word`, *"If word evaluates to '-', file descriptor n … is closed."*
/// **Without `9>&-` a supervisor would hold the lock on with its own copy of the
/// descriptor**, and so would every worker it starts, so a keeper that died
/// under a live supervisor could never be replaced.
///
/// `tail(1)`, `mv(1)` and `su(1)` are read at this file's head; `sleep` is the
/// shell's own ordinary use, unchanged since 2026-09-06.
///
/// **That `flock` is on the machine is the same unestablished per-image ground
/// `su` stands on**, and both are util-linux's. **Where it is absent the keeper
/// ends at once and starts nothing**, which on a first placement is G13's *did
/// not answer* at the health exchange — a loud failure before anything is
/// committed, not a silent one after.
pub fn the_keepers_loop(pointer_path: &str) -> Result<String, ThatValueDoesNotGoIntoARegistration> {
    let pointer = a_path_that_goes_into_a_registration(pointer_path)?;
    let words = the_file_a_supervisors_own_words_go_to_beside(pointer);
    let lock = the_file_beside(pointer, THE_FILE_THE_KEEPER_LOCKS);
    let pause = BETWEEN_TWO_STARTS_BY_THE_KEEPER_IN_SECONDS;
    let cap = THE_MOST_A_SUPERVISORS_OWN_WORDS_KEEP_IN_BYTES;
    Ok(format!(
        "( flock -n 9 || exit 0; while :; do {pointer} {THE_SUPERVISOR_WORD} 9>&- >>{words} 2>&1; \
         tail -c {cap} {words} >{words}.kept 2>/dev/null && mv {words}.kept {words}; \
         sleep {pause}; done ) 9>{lock}"
    ))
}

/// **Smoozy's part of a container hook**, between its two marker lines, ending
/// in a newline.
fn the_smoozy_part_of_a_hook(
    pointer_path: &str,
    the_supervisors_login: &str,
) -> Result<String, ThatValueDoesNotGoIntoARegistration> {
    let login = a_login_that_goes_into_a_registration(the_supervisors_login)?;
    let keeper = the_keepers_loop(pointer_path)?;
    let cap = THE_MOST_A_SUPERVISORS_OWN_WORDS_KEEP_IN_BYTES;
    Ok(format!(
        "{THE_LINE_SMOOZYS_PART_BEGINS_WITH}\n\
         # Smoozy's boot registration on a container rental.\n\
         # Root installs this hook and the hook drops privilege; what it starts is\n\
         # the keeper, whose only job is to start the supervisor and start it\n\
         # again after it ends. The path below is the active-package pointer, so\n\
         # this text does not change when the package version does.\n\
         # The supervisor's own words go to the bounded file beside it, trimmed\n\
         # to its last {cap} bytes after every run. Nothing in Smoozy reads that\n\
         # file: it stands in for the machine-side event journal, which is a\n\
         # different store and holds no process stream.\n\
         su {login} -s /bin/sh -c '{keeper}' \
         </dev/null >/dev/null 2>&1 &\n\
         {THE_LINE_SMOOZYS_PART_ENDS_WITH}\n"
    ))
}

/// **The hook as it is written: what the provider's file already held, kept,
/// with Smoozy's part in it.**
///
/// **K10c, all three of its sentences, kept here**: the file's content is read
/// first and kept, Smoozy's part sits between its two marker lines and a later
/// placement replaces it, and it goes near the top.
///
/// - **Any Smoozy part already there is taken out first**, from its first
///   marker line to its last, so a second placement replaces Smoozy's part
///   rather than adding another. Nothing outside the two lines is touched.
/// - **Smoozy's part goes near the top** — after the first line where that line
///   names an interpreter (`#!`), otherwise first of all. **The provider's own
///   start-up commands may never return** — the page's own example is a training
///   run — and the provider hands the file to a shell that runs it line by line,
///   so a part put after them would never be reached. Smoozy's part backgrounds
///   the keeper and returns at once, so what follows it still runs.
/// - **A file that held nothing gets an interpreter line of its own**, the one
///   the hook always carried.
///
/// **The result is what K16's record holds by value**, so K15 compares the whole
/// file and not only Smoozy's part: `Rented-server runtime` K15, *"a boot
/// registration is confirmed by reading its own content back"*, and the recycle
/// case replaces the whole file.
pub fn the_container_hook_keeping(
    what_the_providers_file_held: &[u8],
    pointer_path: &str,
    the_supervisors_login: &str,
) -> Result<Vec<u8>, ThatValueDoesNotGoIntoARegistration> {
    let ours = the_smoozy_part_of_a_hook(pointer_path, the_supervisors_login)?;
    let theirs = without_smoozys_part(what_the_providers_file_held);
    if theirs.iter().all(|byte| byte.is_ascii_whitespace()) {
        let mut out = b"#!/bin/sh\n".to_vec();
        out.extend_from_slice(ours.as_bytes());
        return Ok(out);
    }
    let mut out = Vec::with_capacity(theirs.len() + ours.len() + 1);
    let first_line_ends = theirs
        .iter()
        .position(|byte| *byte == b'\n')
        .map(|at| at + 1)
        .unwrap_or(theirs.len());
    let (head, rest) = if theirs.starts_with(b"#!") {
        theirs.split_at(first_line_ends)
    } else {
        theirs.split_at(0)
    };
    out.extend_from_slice(head);
    if !head.is_empty() && !head.ends_with(b"\n") {
        out.push(b'\n');
    }
    out.extend_from_slice(ours.as_bytes());
    out.extend_from_slice(rest);
    Ok(out)
}

/// What a hook holds with Smoozy's part taken out — every line from a line
/// that is exactly [`THE_LINE_SMOOZYS_PART_BEGINS_WITH`] to the next line that
/// is exactly [`THE_LINE_SMOOZYS_PART_ENDS_WITH`], inclusive. **A begin line with
/// no end after it takes the rest of the file with it**, which is only ever a
/// part Smoozy itself wrote and a write cut short.
fn without_smoozys_part(held: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(held.len());
    let mut inside = false;
    for line in held.split_inclusive(|byte| *byte == b'\n') {
        let bare = line.strip_suffix(b"\n").unwrap_or(line);
        let bare = bare.strip_suffix(b"\r").unwrap_or(bare);
        if !inside && bare == THE_LINE_SMOOZYS_PART_BEGINS_WITH.as_bytes() {
            inside = true;
            continue;
        }
        if inside {
            if bare == THE_LINE_SMOOZYS_PART_ENDS_WITH.as_bytes() {
                inside = false;
            }
            continue;
        }
        out.extend_from_slice(line);
    }
    out
}

/// **G25's start step on a container, as a file: the keeper and nothing else.**
///
/// **The same loop the hook runs** — [`the_keepers_loop`] — without the hook's
/// `su`, because the start step already runs as the supervisor's own login.
/// **It names the package area's real paths and never the carriage link**, which
/// is taken away after use.
pub fn the_keepers_start_file(
    pointer_path: &str,
) -> Result<Vec<u8>, ThatValueDoesNotGoIntoARegistration> {
    Ok(format!(
        "#!/bin/sh\n\
         # Smoozy's keeper, started once by the placement's own start step.\n\
         # If a keeper already holds the lock this one ends at once.\n\
         {}\n",
        the_keepers_loop(pointer_path)?
    )
    .into_bytes())
}

/// **Where a supervisor's own words go, beside the pointer that starts it.**
///
/// The package area is the directory the active-package pointer sits in, which is
/// [`crate::the_program`]'s own rule for where the entry point finds itself. A
/// pointer at the root gives a file at the root, which is the same directory.
///
/// **It goes through no alphabet check of its own** because it cannot fail one:
/// the pointer has already passed
/// [`a_path_that_goes_into_a_registration`], and this drops that path's last
/// component and appends a constant made of letters and one hyphen.
fn the_file_a_supervisors_own_words_go_to_beside(pointer_path: &str) -> String {
    the_file_beside(pointer_path, THE_FILE_A_SUPERVISORS_OWN_WORDS_GO_TO)
}

/// A file of a constant name in the directory the pointer sits in. **No alphabet
/// check of its own**, for the reason the function above gives.
fn the_file_beside(pointer_path: &str, name: &str) -> String {
    let area = match pointer_path.rsplit_once('/') {
        Some((area, _)) => area,
        None => "",
    };
    format!("{area}/{name}")
}

/// The systemd registration this placement is about to make, with its content
/// composed and its two read-backs not yet taken.
///
/// **Not complete and not claiming to be**: K5 says the registration is not
/// complete until lingering is read back and found granted, and
/// [`TheSystemdRegistration::is_complete`] holds it to that.
pub fn the_systemd_registration_to_be_made(
    pointer_path: &str,
) -> Result<TheSystemdRegistration, ThatValueDoesNotGoIntoARegistration> {
    Ok(TheSystemdRegistration {
        unit_content: the_systemd_unit(pointer_path)?,
        the_kill_behaviour_is_confined_to_the_main_process: true,
        enabled: false,
        the_linger_read_back: None,
    })
}

/// The container registration this placement is about to make, at the path K10
/// fixes — **the provider's, not Smoozy's**.
///
/// `the_read_back_login_can_reach_it` is K19's second obligation and is the root
/// step's to satisfy, so it is carried in rather than assumed: written as K11
/// alone the placement *"fails at K15 every time, before the commit"*.
pub fn the_container_registration_to_be_made(
    the_providers_hook_path: &str,
    pointer_path: &str,
    the_supervisors_login: &str,
    the_read_back_login_can_reach_it: bool,
) -> Result<TheContainerRegistration, ThatValueDoesNotGoIntoARegistration> {
    Ok(TheContainerRegistration {
        the_providers_hook_path: the_providers_hook_path.to_string(),
        content: the_container_hook(pointer_path, the_supervisors_login)?,
        the_read_back_login_can_reach_it,
    })
}

/// **K16's record, from the content just composed.**
///
/// It holds the registration's content **by value**, and nothing derives it from
/// what is on the machine at the moment it is read. K15 compares this against what
/// reads back.
pub fn the_record_of(kind: MachineKind, at_path: &str, content: &[u8]) -> BootRegistrationRecord {
    BootRegistrationRecord {
        kind,
        at_path: at_path.to_string(),
        content: content.to_vec(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const A_POINTER: &str = "/ephemeral/smoozy/package/smoozy-server";

    fn unit() -> String {
        String::from_utf8(the_systemd_unit(A_POINTER).expect("the pointer is admissible"))
            .expect("the unit is text")
    }

    fn hook() -> String {
        String::from_utf8(
            the_container_hook(A_POINTER, "smoozy").expect("both values are admissible"),
        )
        .expect("the hook is text")
    }

    /// **K18, and it is the directive the whole clause is about.** A unit written
    /// to K4 and nothing further destroys a running render on every supervisor
    /// restart.
    #[test]
    fn the_unit_confines_the_kill_to_the_main_process() {
        assert!(unit().contains("\nKillMode=process\n"));
    }

    /// **S4's unconditional restart directive.**
    #[test]
    fn the_unit_restarts_the_supervisor_whatever_ended_it() {
        assert!(unit().contains("\nRestart=always\n"));
    }

    /// **G8 and G25 want a start that fails loudly**, which `Type=exec` gives and
    /// `simple` does not.
    #[test]
    fn the_unit_reports_a_binary_that_could_not_be_invoked() {
        assert!(unit().contains("\nType=exec\n"));
        assert!(!unit().contains("Type=simple"));
    }

    /// **K4's second act has something to enable.** Without an install section
    /// `systemctl enable` creates no link at all.
    #[test]
    fn the_unit_carries_an_install_section_the_enable_can_act_on() {
        assert!(unit().contains("\n[Install]\nWantedBy=default.target\n"));
    }

    /// **G7: neither text names a package version, and both name the pointer.**
    ///
    /// This is the property that makes the commit one deciding write. A
    /// registration naming a version would have to be rewritten at every upgrade,
    /// which is the two-write pair `State, identity and recovery`'s own filesystem
    /// evidence says cannot be made atomic.
    #[test]
    fn neither_text_names_a_package_version_and_both_name_the_pointer() {
        for text in [unit(), hook()] {
            assert!(text.contains(A_POINTER));
            assert!(
                !text.contains(env!("CARGO_PKG_VERSION")),
                "the registration must not change when the version does"
            );
        }
    }

    /// **The two texts start the same role**, so a supervisor is what comes back
    /// on both machine kinds rather than an adapter reading an empty standard
    /// input for ever.
    #[test]
    fn both_texts_start_the_supervisor_role() {
        for text in [unit(), hook()] {
            assert!(text.contains(&format!("{A_POINTER} {THE_SUPERVISOR_WORD}")));
        }
    }

    /// **S5: the keeper starts the supervisor again after it ends**, and **S6**:
    /// it does not distinguish a crash from a deliberate end.
    #[test]
    fn the_hook_starts_a_keeper_that_starts_the_supervisor_again() {
        let hook = hook();
        assert!(hook.contains("while :; do"));
        assert!(hook.contains("done"));
        assert!(hook.contains(&format!(
            "sleep {BETWEEN_TWO_STARTS_BY_THE_KEEPER_IN_SECONDS}"
        )));
    }

    /// **K11: the hook drops privilege**, to the login the machine named.
    #[test]
    fn the_hook_drops_privilege_to_the_supervisors_login() {
        assert!(hook().contains("su smoozy -s /bin/sh -c "));
    }

    /// **The hook returns rather than holding the provider's boot chain**, and its
    /// own standard streams are closed off.
    #[test]
    fn the_hook_detaches_the_keeper_and_returns() {
        let hook = hook();
        let su_line = hook
            .lines()
            .find(|line| line.starts_with("su "))
            .expect("the hook has its su line");
        assert!(su_line.ends_with("</dev/null >/dev/null 2>&1 &"));
    }

    /// **The supervisor's own two streams go to the bounded file in the package
    /// area**, rather than into the keeper's own closed-off ones.
    #[test]
    fn the_supervisors_own_words_go_to_the_bounded_file_beside_the_pointer() {
        let words = "/ephemeral/smoozy/package/supervisor-words";
        assert!(hook().contains(&format!(
            "{A_POINTER} {THE_SUPERVISOR_WORD} 9>&- >>{words} 2>&1"
        )));
        // **And the keeper's own detachment is untouched**, which both audits'
        // leave-alone lists say is right.
        assert!(hook().contains("</dev/null >/dev/null 2>&1 &"));
    }

    /// **Oldest bytes are the ones that go**, at a fixed cap, after every run —
    /// so a machine nobody is watching cannot fill its own disk with its own
    /// complaints.
    #[test]
    fn the_bounded_file_is_trimmed_to_its_last_bytes_after_every_supervisor_run() {
        let words = "/ephemeral/smoozy/package/supervisor-words";
        let cap = THE_MOST_A_SUPERVISORS_OWN_WORDS_KEEP_IN_BYTES;
        assert!(hook().contains(&format!(
            "tail -c {cap} {words} >{words}.kept 2>/dev/null && mv {words}.kept {words}"
        )));
    }

    /// **The file sits beside the pointer**, which is the package area, and a
    /// pointer at the root gives a file at the root.
    #[test]
    fn the_words_file_sits_in_the_package_area_the_pointer_names() {
        assert_eq!(
            the_file_a_supervisors_own_words_go_to_beside(A_POINTER),
            "/ephemeral/smoozy/package/supervisor-words"
        );
        assert_eq!(
            the_file_a_supervisors_own_words_go_to_beside("/smoozy-server"),
            "/supervisor-words"
        );
    }

    /// **The hook says what the file is**, so a reader who finds it does not
    /// take it for something Smoozy reads.
    #[test]
    fn the_hook_says_the_bounded_file_stands_in_for_a_journal_nobody_built() {
        let hook = hook();
        assert!(hook.contains("stands in for the machine-side event journal"));
        assert!(hook.contains("Nothing in Smoozy reads that"));
    }

    /// **§16 item 24, met and not filled in.** The placement never chooses the
    /// restart limiter's settings and this document does not say what it writes,
    /// so nothing is chosen here.
    #[test]
    fn the_unit_chooses_neither_restart_limiter_setting() {
        assert!(!unit().contains("StartLimit"));
    }

    /// **A path that would be escaped into working is refused instead**, before
    /// the commit rather than after it.
    #[test]
    fn a_path_a_unit_file_or_a_shell_would_read_differently_is_refused() {
        for bad in [
            "/ephemeral/smoozy pack/smoozy-server",
            "/ephemeral/%n/smoozy-server",
            "/ephemeral/it's/smoozy-server",
            "/ephemeral/\"quoted\"/smoozy-server",
            "ephemeral/smoozy-server",
            "/ephemeral/smoozy/package/",
            "/ephemeral/smoozy/pack\nage/smoozy-server",
        ] {
            assert!(
                the_systemd_unit(bad).is_err(),
                "{bad} must be refused rather than written"
            );
            assert!(the_container_hook(bad, "smoozy").is_err());
        }
    }

    /// A login the hook's own quoted command could not carry is refused too.
    #[test]
    fn a_login_the_hooks_quoted_command_could_not_carry_is_refused() {
        for bad in ["", "smo ozy", "smo'ozy", "smoozy;rm"] {
            assert!(the_container_hook(A_POINTER, bad).is_err());
        }
    }

    /// **`Rented-server runtime` S5's lock: the keeper takes it before it starts
    /// anything, ends at once where it cannot, and the lock is its own file in
    /// the package area rather than G1a's.**
    #[test]
    fn the_keeper_takes_its_own_lock_before_it_starts_anything() {
        let keeper = the_keepers_loop(A_POINTER).expect("the pointer is admissible");
        assert!(keeper.starts_with("( flock -n 9 || exit 0; while :; do "));
        assert!(keeper.ends_with(" ) 9>/ephemeral/smoozy/package/keeper-lock"));
        assert_ne!(THE_FILE_THE_KEEPER_LOCKS, "package-transaction");
        // **The supervisor does not inherit the lock**, so a keeper that dies
        // under a live supervisor can be replaced.
        assert!(keeper.contains(&format!("{A_POINTER} {THE_SUPERVISOR_WORD} 9>&- ")));
        // **It goes inside `su`'s own quoted command**, so it may carry no
        // quote of its own.
        assert!(!keeper.contains('\''));
        assert!(!keeper.contains('\n'));
    }

    /// **The keeper is written once**: the hook and G25's start step run the
    /// same loop, and only the hook drops privilege.
    #[test]
    fn the_hook_and_the_start_step_run_the_one_keeper() {
        let keeper = the_keepers_loop(A_POINTER).expect("the pointer is admissible");
        assert!(hook().contains(&format!("su smoozy -s /bin/sh -c '{keeper}' ")));
        let start = String::from_utf8(the_keepers_start_file(A_POINTER).expect("it composes"))
            .expect("text");
        assert!(start.starts_with("#!/bin/sh\n"));
        assert!(start.contains(&format!("\n{keeper}\n")));
        assert!(!start.contains("su "));
        // **Nothing in the keeper names the carriage link.**
        assert!(!start.contains(".smoozy-carriage"));
    }

    /// **What the provider's own file held is kept**, Smoozy's part goes
    /// after its interpreter line and before its commands, and a second
    /// placement replaces Smoozy's part rather than adding another.
    #[test]
    fn the_providers_own_hook_is_kept_and_smoozys_part_is_replaced_not_added() {
        // Vast's own default, as the second pass on 2026-09-21 read it: a
        // shebang and a comment, 82 bytes.
        let theirs = b"#!/bin/bash\n# This file is run on instance start. Output in /var/log/onstart.log\n";
        let once = the_container_hook_keeping(theirs, A_POINTER, "root").expect("it composes");
        let text = String::from_utf8(once.clone()).expect("text");
        assert!(text.starts_with("#!/bin/bash\n"));
        assert!(text.ends_with(
            "# <<< Smoozy's boot registration ends. <<<\n# This file is run on instance start. Output in /var/log/onstart.log\n"
        ));
        assert_eq!(text.matches(THE_LINE_SMOOZYS_PART_BEGINS_WITH).count(), 1);

        // **Placed again**, over what the first placement left: the same bytes.
        let twice = the_container_hook_keeping(&once, A_POINTER, "root").expect("it composes");
        assert_eq!(twice, once);

        // **A provider's long-running command stays after Smoozy's part.**
        let a_training_run = b"cd /workspace\npython train.py --resume\n";
        let kept = String::from_utf8(
            the_container_hook_keeping(a_training_run, A_POINTER, "root").expect("it composes"),
        )
        .expect("text");
        assert!(kept.starts_with(THE_LINE_SMOOZYS_PART_BEGINS_WITH));
        assert!(kept.ends_with("\ncd /workspace\npython train.py --resume\n"));

        // **Nothing there** gets the interpreter line the hook always carried.
        assert_eq!(
            the_container_hook_keeping(b"", A_POINTER, "smoozy").expect("it composes"),
            the_container_hook(A_POINTER, "smoozy").expect("it composes")
        );
        assert!(hook().starts_with("#!/bin/sh\n"));
    }

    /// **K5: a composed registration is not a complete one.**
    #[test]
    fn a_composed_systemd_registration_is_not_yet_complete() {
        let made = the_systemd_registration_to_be_made(A_POINTER).expect("it composes");
        assert!(!made.is_complete());
        assert!(made.the_kill_behaviour_is_confined_to_the_main_process);
    }

    /// **K16: the record holds the content by value**, which is the half K15
    /// compares.
    #[test]
    fn the_record_holds_the_content_by_value() {
        let content = the_systemd_unit(A_POINTER).expect("it composes");
        let record = the_record_of(
            MachineKind::Systemd,
            "/home/smoozy/.config/systemd/user/smoozy-supervisor.service",
            &content,
        );
        assert_eq!(record.content, content);
        assert_eq!(record.kind, MachineKind::Systemd);
    }

    /// **The whole of what this file is for, run rather than reasoned: the unit
    /// is composed, written at the path K4 puts it, read back, and confirmed
    /// against the record K16 holds by value.**
    ///
    /// It is run at the desk, which is a unix, and the acts are the real ones —
    /// [`crate::acting::registration`]'s own write and its own read. **What the
    /// desk cannot exercise is the enable, the lingering grant and its read-back**,
    /// which are the machine's own tools and are refused here by their absence
    /// rather than answered wrongly.
    #[cfg(unix)]
    #[test]
    fn the_composed_unit_is_written_read_back_and_confirmed() {
        use crate::acting::registration::TheRegistrationOnTheMachine;
        use crate::registration::PlacesTheRegistration;
        use crate::registration::{confirm, ReadsTheRegistrationBack, RegistrationConfirmation};

        let area = std::env::temp_dir().join(format!(
            "smoozy-registration-content-{}",
            std::process::id()
        ));
        std::fs::remove_dir_all(&area).ok();
        let at = area
            .join("home")
            .join(THE_USER_UNIT_DIRECTORY_UNDER_A_HOME)
            .join(THE_UNIT_NAME);

        let content = the_systemd_unit(A_POINTER).expect("the pointer is admissible");
        let record = the_record_of(MachineKind::Systemd, &at.display().to_string(), &content);

        let placement = TheRegistrationOnTheMachine::on(
            MachineKind::Systemd,
            &area.join("package"),
            THE_UNIT_NAME,
            std::time::Duration::from_secs(5),
        )
        .expect("the desk answers for its own login");
        placement
            .place_the_registration(&record)
            .expect("the unit is written");

        // **K16's record went with it, in the same act**, and it holds the
        // content by value.
        let recorded: BootRegistrationRecord = crate::written_form::read(
            &crate::acting::durable::read(&placement.the_boot_registration_record_sits_at())
                .expect("the record is readable")
                .expect("the record is there"),
        )
        .expect("it reads back");
        assert_eq!(recorded, record);

        let held = placement
            .the_content_at(&record.at_path)
            .expect("it reads back");
        assert_eq!(
            confirm(&record, Ok(&held)),
            RegistrationConfirmation::ContentMatchesTheRecord
        );

        // **K15's own case**: the bytes are replaced and the read-back refuses
        // loudly, before the commit.
        std::fs::write(&at, b"the provider's own default").expect("it is replaced");
        let held = placement
            .the_content_at(&record.at_path)
            .expect("it reads back");
        assert!(matches!(
            confirm(&record, Ok(&held)),
            RegistrationConfirmation::ContentDiffers { .. }
        ));
        std::fs::remove_dir_all(&area).ok();
    }

    /// **K19 is carried rather than assumed**, because written as K11 alone the
    /// placement fails at K15 every time and the failure is a permanent loop.
    #[test]
    fn the_container_registration_carries_k19s_obligation() {
        let cannot =
            the_container_registration_to_be_made("/root/onstart.sh", A_POINTER, "smoozy", false)
                .expect("it composes");
        assert!(!cannot.the_root_step_left_it_readable());
    }
}
