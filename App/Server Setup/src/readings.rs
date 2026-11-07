//! The readings this module takes of a machine, and the real ends of the two
//! readings the rules half recorded as reaching none.
//!
//! **This is the acting half's reading side.** Every judgement over what comes
//! back is next door, in the module that owns the clause, and none of it is
//! retaken here. What this module does is ask the machine, turn each answer
//! into the value the judgement takes, and hand it over — the same split
//! `Servers` and `File Browser` already use.
//!
//! **Two surfaces, and which one a reading rides is decided by the value it
//! carries, never by convenience.** A reading whose command names nothing of
//! user, provider or machine origin rides the command channel. A reading that
//! names a path rides the file-transfer subsystem, where the path is a protocol
//! field — `Server protocol and transfers` 1.1's first carriage, *"length-
//! delimited data on a route no shell parses"*. **Every command string below is
//! a constant**, which the type enforces.
//!
//! **Nothing here classifies a failure on an exit status where 255 is one of
//! the possibilities** (`Server protocol and transfers` §1.3). `Servers` K1a
//! runs no command-line client, so a transport failure arrives as its own value
//! and never as a status; every status read below was produced by the machine's
//! own shell.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rented-server runtime` K1, K2, K3, V6, G28.
//! `Server protocol and transfers` §1.1, §1.3. `Servers` H11, K1a, Q12. `File
//! Browser` P5.
//!
//! **`Server protocol and transfers` B8 is not in the list above and is written
//! out at every site**, because this plan defines a B8 of its own and a bare one
//! could mean either.

use std::future::Future;

use smoozy_server_protocol::subsystem::{StatusCode, SubsystemFailure};
use smoozy_server_protocol::transport::ssh::CommandFailure;
use smoozy_server_protocol::transport::{CommandResult, CommandString};
use smoozy_servers::readings::CommandChannel;

use crate::kind::{
    MachineKind, Probe, ReadsTheMachineBeforeAnythingIsTouched, TheRentalsTuple,
    WhichOsReleaseFile, THE_NON_INTERACTIVE_PROBE,
};

// ------------------------------------------------------- the file surface

/// What the machine said about one path.
///
/// **`Server protocol and transfers` §1.3's separation, read rather than
/// derived**: the status code is carried as itself by that document and this
/// only names the ones it separates. **Nothing parses a message.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheMachineSaid {
    /// The path is not there.
    ThePathIsNotThere,
    /// The login may not.
    TheLoginMayNot,
    /// It is already there — the plain make-directory's own refusal.
    ItIsAlreadyThere,
    /// The channel is gone.
    TheChannelIsGone,
    /// A replacement set did not pass its check.
    TheReplacementCouldNotBeVerified,
    /// Anything else the machine refused with.
    ItRefused,
}

impl WhatTheMachineSaid {
    /// **The refusal is read as the machine's own and never mined for a
    /// cause.** `Rented-server runtime` M24's discipline and `Security and
    /// licence` §13 rule 5's, cited rather than restated.
    pub fn from(failure: SubsystemFailure) -> WhatTheMachineSaid {
        match failure {
            SubsystemFailure::TheChannelIsGone => WhatTheMachineSaid::TheChannelIsGone,
            SubsystemFailure::TheServerRefused(StatusCode::NoSuchFile) => {
                WhatTheMachineSaid::ThePathIsNotThere
            }
            SubsystemFailure::TheServerRefused(StatusCode::PermissionDenied) => {
                WhatTheMachineSaid::TheLoginMayNot
            }
            _ => WhatTheMachineSaid::ItRefused,
        }
    }

    /// **What the file-transfer subsystem answered, in standard software
    /// words** (Rule 9) — and nothing more than the status the protocol
    /// carried, so no cause is claimed that the answer does not carry (Rule 8).
    /// Added 2026-09-21: until then the only rendering of this was a debugging
    /// one, which reached the screen.
    pub fn in_words(&self) -> &'static str {
        match self {
            WhatTheMachineSaid::ThePathIsNotThere => "the server said the file or folder is not there",
            WhatTheMachineSaid::TheLoginMayNot => "the server said permission was denied",
            WhatTheMachineSaid::ItIsAlreadyThere => "the server said it is already there",
            WhatTheMachineSaid::TheChannelIsGone => "the connection to the server ended",
            WhatTheMachineSaid::ItRefused => "the server refused it",
            WhatTheMachineSaid::TheReplacementCouldNotBeVerified => "file verification failed",
        }
    }
}

/// One entry of a listed directory, cut to what this module reads of one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AListedThing {
    pub name: Vec<u8>,
    pub it_is_a_directory: bool,
}

/// **The file-transfer subsystem, as this module reaches it.**
///
/// It is a trait for the reason `Servers` put its command channel behind one:
/// the acts above it are worth exercising and an act that can only be reached
/// through a socket is an act nobody exercises. **The real end is
/// [`crate::wiring`]**, which is the only file in this crate that knows a
/// protocol exists.
pub trait TheMachinesFileSurface {
    fn read_whole(
        &mut self,
        path: &str,
    ) -> impl Future<Output = Result<Vec<u8>, WhatTheMachineSaid>> + Send;

    fn write_whole(
        &mut self,
        path: &str,
        bytes: &[u8],
    ) -> impl Future<Output = Result<(), WhatTheMachineSaid>> + Send;

    /// A replacement is one rename; never remove the live name first.
    fn replace_by_rename(
        &mut self,
        _from: &str,
        _to: &str,
    ) -> impl Future<Output = Result<(), WhatTheMachineSaid>> + Send {
        std::future::ready(Err(WhatTheMachineSaid::ItRefused))
    }

    fn read_link(
        &mut self,
        _path: &str,
    ) -> impl Future<Output = Result<Vec<u8>, WhatTheMachineSaid>> + Send {
        std::future::ready(Err(WhatTheMachineSaid::ItRefused))
    }

    fn make_one_directory(
        &mut self,
        path: &str,
    ) -> impl Future<Output = Result<(), WhatTheMachineSaid>> + Send;

    fn remove_one_file(
        &mut self,
        path: &str,
    ) -> impl Future<Output = Result<(), WhatTheMachineSaid>> + Send;

    fn remove_one_empty_directory(
        &mut self,
        path: &str,
    ) -> impl Future<Output = Result<(), WhatTheMachineSaid>> + Send;

    fn list_one_directory(
        &mut self,
        path: &str,
    ) -> impl Future<Output = Result<Vec<AListedThing>, WhatTheMachineSaid>> + Send;

    /// **The planted-link carriage's own operation** — `Server protocol and
    /// transfers` 1.1's third carriage, which is the one route a machine-origin
    /// path has into a constant command.
    fn plant_a_link(
        &mut self,
        target: &str,
        at: &str,
    ) -> impl Future<Output = Result<(), WhatTheMachineSaid>> + Send;
}

// ------------------------------------------------------------ the commands

/// **`Rented-server runtime` K1's first row, sent by `Servers` and named here so
/// a reader of this file sees the whole surface.**
///
/// `Servers` performs both limbs — the container marker test and the tool's own
/// container-only form — and [`read_the_machine_kind`] calls that performance
/// rather than sending them again. **These are the strings it sends**, and they
/// are recorded here because a reader asking *what does this module put on a
/// machine* must be able to answer it from one page.
pub const WHAT_SERVERS_SENDS_FOR_THE_CONTAINER_ROW: [&str; 2] =
    ["test -e /.dockerenv", "systemd-detect-virt --container"];

/// **M3's probe, at the moment a step is refused and never before.**
///
/// The constant itself is [`crate::kind::THE_NON_INTERACTIVE_PROBE`]; it is
/// re-exported through this module's own list so that a reader of the acting
/// half sees the whole set of commands in one place.
pub const THE_PROBE: CommandString = THE_NON_INTERACTIVE_PROBE;

/// **Whether the connection's login is the machine's root** — M2 turns on it,
/// and `File Browser` P5's first row is decided by it on a container.
///
/// **A number, not a name.** The login's own name is a value this module has no
/// use for and would have nowhere to carry; what the judgement asks is whether
/// the login is root, which is the number the machine prints.
pub const WHICH_LOGIN_IS_THIS: CommandString = CommandString::constant("id -u");

/// **R1 item 5's check, run before anything is installed and again after.**
pub const THE_DRIVER_CHECK: CommandString = crate::ready_action::THE_DRIVER_CHECK;

/// **R1 item 3's probe: the package system is asked whether the unpack tool's
/// package is on the machine, before anything is installed.**
///
/// *"The one unpack tool Blender's documentation names, **installed where
/// absent**"*, and *"the step **probes rather than assumes**"*. **This is that
/// probe**, and until it landed the install was issued on every run that
/// fetched a Blender.
///
/// **Documented, and the command is the package system's own** — `dpkg-query(1)`,
/// dpkg **1.21.22**, Debian bookworm, read **2026-09-20** at
/// `https://manpages.debian.org/bookworm/dpkg/dpkg-query.1.en.html`:
/// *"dpkg-query - a tool to query the dpkg database"*, and of this command,
/// *"-s, --status \[package-name…\] Report status of specified packages. This
/// just displays the entry in the installed package status database."* **The
/// same page on Ubuntu carries the same words** — dpkg 1.22.6ubuntu6.6, read
/// **2026-09-20** at
/// `https://manpages.ubuntu.com/manpages/noble/en/man1/dpkg-query.1.html` — so
/// **M1a's pair is one command rather than two**, both families carrying the
/// same package system.
///
/// **The exit is documented too, and it is what tells a package the database
/// has never heard of from one it has**: *"0 The requested query was
/// successfully performed."*, *"1 The requested query failed either fully or
/// partially, due to no file or package being found"* (both pages, same read).
///
/// **It is elevated by nothing.** The probe reads the package database and
/// changes nothing, so it rides the ordinary command channel like every other
/// reading here, and a machine that already holds the tool now performs **no**
/// elevated act for it at all.
///
/// **Why the status is asked for rather than the version.** `--showformat`
/// would name the field directly, and its format string needs a dollar and a
/// quote — characters
/// [`tests::every_command_this_module_sends_is_a_constant_naming_no_value`]
/// refuses in a reading command, and rightly, because they are what a composed
/// command looks like. **The status entry needs neither.**
pub const THE_UNPACK_TOOLS_PACKAGE_PROBE: CommandString =
    CommandString::constant("dpkg-query -s xz-utils");

/// **The field of the status entry the probe reads**, in the entry's own
/// spelling.
///
/// **Documented** — `dpkg-query(1)`, same two pages and same date: the `Status`
/// field is the one the three virtual fields are parts of — *"db:Status-Want It
/// contains the package wanted status, **part of the Status field**"*,
/// *"db:Status-Status It contains the **package status word**, part of the
/// Status field"*, *"db:Status-Eflag It contains the package status error flag,
/// part of the Status field"*.
pub const THE_STATUS_FIELD: &str = "Status:";

/// **The one status word that means the tool is on the machine and usable.**
///
/// **Documented** — `dpkg(1)`, dpkg 1.21.22, Debian bookworm, read
/// **2026-09-20** at `https://manpages.debian.org/bookworm/dpkg.1.en.html`, and
/// the same words at dpkg 1.22.6ubuntu6.6, read **2026-09-20** at
/// `https://manpages.ubuntu.com/manpages/noble/en/man1/dpkg.1.html`. Under
/// *Package states*: *"installed — The package is correctly unpacked and
/// configured."* The other seven are *"not-installed"*, *"config-files"*,
/// *"half-installed"*, *"unpacked"*, *"half-configured"*, *"triggers-awaited"*
/// and *"triggers-pending"*, and **none of them is this word**: the two that
/// carry it carry it inside a longer word, which is why the reading matches a
/// whole word and never a fragment.
///
/// **The word is looked for anywhere in the field rather than at a fixed place
/// in it, and that is deliberate.** The publication read gives the three parts
/// of the `Status` field and **does not fix the order they are written in**, so
/// a reading that counted words would be resting on something nobody here has
/// read. It does not need to: the same two pages give the selection states —
/// *"install"*, *"hold"*, *"deinstall"*, *"purge"*, *"unknown"* — and the flags
/// — *"ok"*, *"reinstreq"* — and **this word is in neither list**, so finding it
/// as a whole word in the field can mean only the package state.
pub const THE_INSTALLED_STATE: &str = "installed";

/// **B10's whole-disk search, and every byte of it is fixed at build time** —
/// E7's own bullet for the elevated retry: *"the same constant search command as
/// the unprivileged pass: fixed name, fixed root"*.
///
/// **It crosses filesystems**, which is B10's rule: restricting the search to
/// one silently loses a snap mounted from a loop device. **The refusals are
/// counted rather than hidden**, which is why the complaint stream is kept and
/// B11's shortfall is composed from it.
///
/// **It is not restricted by kind of file either, and that restriction came out
/// on 2026-09-04.** The search carried `-type f` in both its plain and its
/// elevated form. B10 states *"a whole-disk file search for Blender's executable
/// name, crossing filesystems"* and names no restriction by kind of file; B11
/// names exactly two honest limits — a renamed binary is invisible, and a folder
/// the login may not read is skipped and counted — and neither is this. **So the
/// narrowing was the code's own and the answer did not declare it.**
///
/// **Why it mattered more than an undeclared limit usually would.** B10 spends a
/// sentence on one trap: *"deduplication is never by resolved path — a snap's
/// path resolves to the snap launcher, so the path-and-answer pair is the
/// entry"*, and this project's own evidence behind it records `/snap/bin/blender`
/// **resolving to** `/usr/bin/snap`. **A path that resolves to another path is
/// not a plain file** — so the restriction may have been closing the one trap the
/// clause exists to handle, by losing the case rather than by handling it, and
/// B11's answer would then have said the search was complete while a build the
/// user owns was invisible.
///
/// **What `-type f` selects on a rented machine's `find` is not said here.**
/// That is a fact about a tool, nobody in this project has read its publication,
/// and Rule 2 forbids answering it from memory; the Creator's Mac cannot settle
/// it either, being a different `find` from a different family. **So the safe
/// direction is the loud one and the restriction is out.**
///
/// **What it costs, stated rather than glossed.** More candidates are found;
/// **each costs one asking-run** under B1's discipline and its time limit; and
/// **folders named `blender` land on B11's did-not-answer list, where the user
/// can see them.** A narrower form can be put back later on evidence rather than
/// on memory.
///
/// **A narrower form was put back on 2026-09-26, on evidence: directories are
/// left out, and nothing else is.** Step 10c's sixth pass on Vast found the
/// search offering `/root/.cache/thumbnails/fail/blender` — a thumbnail cache
/// folder Blender's own reading run makes, whose name happens to be `blender` —
/// beside the fifth pass's `io_scene_gltf2/blender`, a folder of an add-on.
/// Both reached the user as Blenders that would not say what they were. **The
/// evidence is the published definition of `find`, not memory**: POSIX.1-2017
/// (The Open Group Base Specifications Issue 7, 2018 edition), `find`, read
/// 2026-09-26 at `https://pubs.opengroup.org/onlinepubs/9699919799/utilities/find.html`
/// — *"-type c: The primary shall evaluate as true if the type of the file is c,
/// where c is 'b', 'c', 'd', 'l', 'p', 'f', or 's' for block special file,
/// character special file, directory, symbolic link, FIFO, regular file, or
/// socket, respectively"*; *"! expression: Negation of a primary"*; and, with
/// neither `-H` nor `-L`, *"the file information and type for symbolic links
/// … shall be that of the link itself"*. **So `! -type d` drops a directory and
/// keeps a link whatever it points at** — which is why this is not `-type f`
/// put back: `/snap/bin/blender`, a link to the snap launcher, stays a
/// candidate, and B10's second trap is still met by handling the case rather
/// than losing it. **The bound, stated**: that is the standard's definition,
/// and the `find` a rented machine carries was not read; a `find` that departed
/// from it on this primary would offer folders again, which is this search's
/// cost before this date and nothing worse.
///
/// **And it does not descend into `/proc` or `/sys` — B10, added 2026-10-02**
/// out of the Creator's note 13 on 0.1.7 and that day's Sesterce pass, where an
/// ordinary login's search counted 2,679 folders it could not read, 2,578 of
/// them under `/proc`. The Filesystem Hierarchy Standard 3.0 names `/proc` the
/// *"Kernel and process information virtual filesystem"* (§6.1.5) and `/sys`
/// the *"Kernel and system information virtual filesystem"* (§6.1.7) — the
/// kernel's own information, not files. **The same POSIX page, read again
/// 2026-10-02**: *"-path pattern: The primary shall evaluate as true if the
/// current pathname matches pattern"*; *"-prune: The primary shall always
/// evaluate as true; it shall cause find not to descend the current pathname if
/// it is a directory"*; and the AND implied by juxtaposition binds tighter than
/// *"expression -o expression … the OR operator"*. **So each `-path … -prune`
/// stops one tree, and everything else falls through to the search as it was.**
/// **This is not a restriction to one filesystem**: every other mount, a
/// snap's loop device included, is still crossed. The shape was run on this
/// desk's own `find` over a folder of its own, and found the one build outside
/// the two pruned trees; a rented machine's `find` was not run.
pub const THE_SEARCH: CommandString = CommandString::constant(
    "find / -path /proc -prune -o -path /sys -prune -o -name blender ! -type d -print",
);

/// **B1's version ask, through the planted-link carriage.**
///
/// A build's path is machine-origin, so it cannot be written into a command
/// string in either phase (`Server protocol and transfers` 1.1). It rides the
/// planted link and **the command names the token**.
pub const ASK_THE_BUILD_ITS_VERSION: CommandString =
    CommandString::constant("~/.smoozy-carriage/blender-candidate --version");

/// The carriage site B1's ask names.
pub const THE_VERSION_ASKS_CARRIAGE_SITE: smoozy_server_protocol::carriage::CarriageSite =
    smoozy_server_protocol::carriage::CarriageSite {
        link_path_on_the_subsystem: ".smoozy-carriage/blender-candidate",
        named_in_a_command_as: "~/.smoozy-carriage/blender-candidate",
    };

/// **`Server protocol and transfers` B8's processor-architecture half, asked of
/// the machine as a value.**
///
/// **Documented** — `uname(1)`, GNU coreutils 9.1, dated September 2022, read
/// 2026-09-07 at
/// `https://manpages.debian.org/bookworm/coreutils/uname.1.en.html`, where the
/// `-m` option is *"print the machine hardware name"*.
///
/// **The answer is carried into the tuple as the machine's own word, untouched
/// beyond a trim.** Nothing spells it again, nothing translates it, and nothing
/// tries a second spelling where the first does not match: a name a manifest
/// does not carry is an unsupported machine.
pub const NAME_YOUR_PROCESSOR_ARCHITECTURE: CommandString = CommandString::constant("uname -m");

/// **`Server protocol and transfers` B8's ABI half — the machine's C library,
/// asked of that library's own configuration variable.**
///
/// **Documented, in two publications.** The variable is the GNU C library's own
/// and names the library the machine carries: `confstr(3)`, Linux man-pages
/// through `manpages-dev` 6.03-2, dated 2023-02-05, read 2026-09-07 at
/// `https://manpages.debian.org/bookworm/manpages-dev/confstr.3.en.html`, where
/// the GNU C library version variable is *"A string which identifies the GNU C
/// library version on this system (e.g., "glibc 2.3.4")"* and exists on the
/// *"GNU C library only"*. And the name this command gives that variable is the
/// variable's own with its prefix taken off: `getconf`, POSIX.1-2017, read
/// 2026-09-07 at
/// `https://pubs.opengroup.org/onlinepubs/9699919799/utilities/getconf.html`,
/// where the operand is *"The names of the symbolic constants used as the name
/// argument of the confstr() function … without the _CS_ prefix"*.
///
/// **What is done with the answer is [`crate::kind::the_tuple_the_machine_named`]'s**,
/// and the release the answer carries beside the library's name is read and used
/// for nothing.
pub const NAME_YOUR_C_LIBRARY: CommandString = CommandString::constant("getconf GNU_LIBC_VERSION");

/// Every reading command this crate sends, so a reader sees the set at once and
/// a test can assert that not one of them carries a value.
///
/// **It is the readings and not the whole surface.** The elevated commands are
/// [`crate::elevated::THE_ELEVATED_STEPS`], which is E7's own enumeration
/// performed, and the container row's two limbs are `Servers`' and are named at
/// [`WHAT_SERVERS_SENDS_FOR_THE_CONTAINER_ROW`]. **Between the three lists a
/// reader has every command this module puts on a machine.**
pub const EVERY_COMMAND_THIS_MODULE_SENDS: [CommandString; 10] = [
    THE_PROBE,
    WHICH_LOGIN_IS_THIS,
    THE_DRIVER_CHECK,
    THE_MODULE_READING_ON_UBUNTU,
    THE_MODULE_READING_ON_DEBIAN,
    THE_UNPACK_TOOLS_PACKAGE_PROBE,
    THE_SEARCH,
    ASK_THE_BUILD_ITS_VERSION,
    NAME_YOUR_PROCESSOR_ARCHITECTURE,
    NAME_YOUR_C_LIBRARY,
];

/// **`Rented-server runtime` K1's second row, read without a command at all.**
///
/// That table's second row is *"neither of those, **and** systemd is running as
/// the machine's first process"*, and this is the file that answers it. **It is
/// a path, so it rides the file-transfer subsystem** and no command string is
/// composed for it — which also means the reading needs no package installed on
/// the machine.
///
/// **Documented** — Linux man-pages 6.9.1, dated 2024-05-02, read 2026-09-04 at
/// `https://manpages.debian.org/trixie/manpages/proc_pid_comm.5.en.html`:
/// *"/proc/pid/comm (since Linux 2.6.33) This file exposes the process's comm
/// value—that is, the command name associated with the process."* The first
/// process is process 1, so this path is the command name of the machine's first
/// process.
pub const THE_FIRST_PROCESSS_NAME: &str = "/proc/1/comm";

/// The name the first process carries where the machine was booted with
/// systemd.
pub const SYSTEMDS_OWN_NAME: &str = "systemd";

// -------------------------------------------------- the opening readings

/// **M1 and M1a's pair, taken over the connection before anything is touched.**
///
/// **The two readings are held rather than re-asked**, and that is forced
/// rather than chosen: [`ReadsTheMachineBeforeAnythingIsTouched`] is a plain
/// trait taken by the judgement next door, and a machine is reached
/// asynchronously. So the acting half takes the answers and this carries them.
/// **Nothing here composes an answer** — every field below is what the machine
/// said, and a reading that could not be taken is an absence rather than a
/// value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheMachineBeforeAnythingIsTouched {
    kind: MachineKind,
    etc: Option<String>,
    usr_lib: Option<String>,
    /// **A reading that never reached the machine**, because the connection
    /// went under it: a command whose request did not land, or a file read
    /// whose channel was gone. Added 2026-10-26, the worldwide pass's setup
    /// faults, fault 2.
    a_reading_never_reached_the_machine: bool,
}

impl TheMachineBeforeAnythingIsTouched {
    /// The answers, as a value, for a caller that took them some other way.
    pub fn as_answered(
        kind: MachineKind,
        etc: Option<String>,
        usr_lib: Option<String>,
    ) -> TheMachineBeforeAnythingIsTouched {
        TheMachineBeforeAnythingIsTouched {
            kind,
            etc,
            usr_lib,
            a_reading_never_reached_the_machine: false,
        }
    }

    /// **The same answers, with a reading among them that never reached the
    /// machine** — for a caller that took them some other way and met a
    /// connection going under one. Added 2026-10-26, fault 2.
    pub fn with_a_reading_that_never_reached_the_machine(mut self) -> TheMachineBeforeAnythingIsTouched {
        self.a_reading_never_reached_the_machine = true;
        self
    }

    pub fn kind(&self) -> MachineKind {
        self.kind
    }

    /// **Whether any of the readings never reached the machine.** An absent
    /// answer is still never read as a family or as systemd — M1a's rule and
    /// the kind's own — but **a refusal reached that way is not a reading of
    /// the machine at all**: the machine was never asked. So whoever tells the
    /// refusal asks this first, and a connection that went is told as one,
    /// never as a machine Smoozy does not support (Rule 8). The worldwide pass
    /// on Vast, 2026-10-26: an Ubuntu 22.04 container was told *not a Linux
    /// server Smoozy supports* at the moment a three-second reconnect took the
    /// connection from under the first reading.
    pub fn a_reading_never_reached_the_machine(&self) -> bool {
        self.a_reading_never_reached_the_machine
    }
}

impl ReadsTheMachineBeforeAnythingIsTouched for TheMachineBeforeAnythingIsTouched {
    fn the_kind(&mut self) -> MachineKind {
        self.kind
    }

    fn os_release(&mut self, which: WhichOsReleaseFile) -> Option<String> {
        match which {
            WhichOsReleaseFile::Etc => self.etc.clone(),
            WhichOsReleaseFile::UsrLib => self.usr_lib.clone(),
        }
    }
}

/// **M1a's last bullet, performed: both readings are taken before the
/// hand-over rather than at it.**
///
/// *"`/etc/os-release` is a plain read over the file-transfer subsystem,
/// needing no elevation, nothing placed and no hand-over, and the kind
/// detection is a command-channel reading of the same standing."* **Nothing on
/// the machine is changed by any of it**, which is what makes M1a's refusal a
/// reading rather than a part-finished run.
///
/// **Both files are read here and the precedence is applied next door**, at
/// [`crate::kind::read_before_anything_is_touched`], which is the standard's
/// own: `/etc/os-release` takes precedence and `/usr/lib/os-release` is
/// consulted only where the first is missing. Reading a second small file the
/// judgement may discard costs one subsystem round trip and keeps the
/// precedence in the one place that states it.
pub async fn take_the_opening_readings(
    channel: &mut impl CommandChannel,
    files: &mut impl TheMachinesFileSurface,
) -> TheMachineBeforeAnythingIsTouched {
    // **Every reading is watched for one that never reached the machine**
    // (2026-10-26, the worldwide pass's setup faults, fault 2).
    let mut noting = NotingWhatNeverReachedTheMachine {
        channel,
        never: false,
    };
    let (kind, the_first_process_never_reached) =
        read_the_machine_kind_noting(&mut noting, files).await;
    let (etc, etc_never_reached) = read_an_os_release(files, WhichOsReleaseFile::Etc).await;
    let (usr_lib, usr_lib_never_reached) = match etc {
        Some(_) => (None, false),
        None => read_an_os_release(files, WhichOsReleaseFile::UsrLib).await,
    };
    TheMachineBeforeAnythingIsTouched {
        kind,
        etc,
        usr_lib,
        a_reading_never_reached_the_machine: noting.never
            || the_first_process_never_reached
            || etc_never_reached
            || usr_lib_never_reached,
    }
}

/// **One os-release file, and whether its read never reached the machine** —
/// the channel gone under it, which is the connection's and says nothing about
/// the file.
async fn read_an_os_release(
    files: &mut impl TheMachinesFileSurface,
    which: WhichOsReleaseFile,
) -> (Option<String>, bool) {
    match files.read_whole(which.path()).await {
        // **A file that is not text is a file that could not be read**, which
        // M1a meets as *a machine that will not say* rather than as a family.
        // An absent answer is never read as Ubuntu.
        Ok(bytes) => (String::from_utf8(bytes).ok(), false),
        Err(WhatTheMachineSaid::TheChannelIsGone) => (None, true),
        Err(_) => (None, false),
    }
}

/// **The command channel, with every command whose request never reached the
/// machine noted** — the transport's own knowledge, `CommandFailure`'s second
/// value, and never a reading of the machine's words. Added 2026-10-26, fault
/// 2.
struct NotingWhatNeverReachedTheMachine<'c, C: CommandChannel> {
    channel: &'c mut C,
    never: bool,
}

impl<C: CommandChannel> CommandChannel for NotingWhatNeverReachedTheMachine<'_, C> {
    fn run(
        &mut self,
        command: CommandString,
    ) -> impl Future<Output = Result<CommandResult, CommandFailure>> + Send {
        let never = &mut self.never;
        let running = self.channel.run(command);
        async move {
            let ran = running.await;
            if matches!(ran, Err(CommandFailure::TheRequestDidNotReachTheMachine)) {
                *never = true;
            }
            ran
        }
    }
}

/// **M1: `Rented-server runtime` K1's table, read against this machine.**
///
/// **The table is that document's and is not re-decided here.** §1's own
/// Constraint says the detection *"is `Rented-server runtime` K1's and is not
/// re-invented"*, and what that settles is **ownership**: no document claims the
/// table and none of them re-decides it, so a performance in code is a reading
/// of one owned table rather than a second owner. `Servers` already holds one
/// for its own hardware reading and **the two must stay identical**.
///
/// **The container rows are `Servers`' performance, called rather than written
/// again.** That crate sends the container marker test and the tool's own
/// container-only form, and its own comment gives the reason the option is read
/// rather than a list of technology names matched: a list of names has to be
/// kept in step with a tool that adds them, and the option answers the same
/// question from the tool itself. **It also disposes of K1's own near-miss** — a
/// container inside a virtual machine — because a reading limited to container
/// virtualization cannot report the outer one.
///
/// **What is added here is the table's second row**, which `Servers` has no use
/// for: *"neither of those, **and** systemd is running as the machine's first
/// process"*. That reading is [`THE_FIRST_PROCESSS_NAME`] and it is a path, so
/// it needs no command and no package installed on the machine.
///
/// **K2's re-taking rule is kept by this being the only route to the kind**: it
/// is taken at every placement and nothing carries one forward.
pub async fn read_the_machine_kind(
    channel: &mut impl CommandChannel,
    files: &mut impl TheMachinesFileSurface,
) -> MachineKind {
    read_the_machine_kind_noting(channel, files).await.0
}

/// **[`read_the_machine_kind`], saying beside it whether the first process's
/// read never reached the machine** (2026-10-26, fault 2). The kind is decided
/// exactly as before.
async fn read_the_machine_kind_noting(
    channel: &mut impl CommandChannel,
    files: &mut impl TheMachinesFileSurface,
) -> (MachineKind, bool) {
    if matches!(
        smoozy_servers::readings::read_the_machine_kind(channel).await,
        smoozy_servers::hardware::MachineKind::AContainer
    ) {
        return (MachineKind::AContainer, false);
    }

    let (systemd_is_the_first_process, never_reached) =
        match files.read_whole(THE_FIRST_PROCESSS_NAME).await {
            Ok(bytes) => (
                String::from_utf8(bytes)
                    .map(|name| name.trim() == SYSTEMDS_OWN_NAME)
                    .unwrap_or(false),
                false,
            ),
            // **An unreadable answer is never read as systemd**, which is the
            // same discipline M1a holds to for the family: an absent answer is
            // not an answer, and `Rented-server runtime` K3's refusal is where
            // it lands.
            Err(said) => (false, said == WhatTheMachineSaid::TheChannelIsGone),
        };

    if systemd_is_the_first_process {
        (MachineKind::ASystemdMachine, never_reached)
    } else {
        (MachineKind::TheTableDoesNotPlaceIt, never_reached)
    }
}

// ------------------------------------------------- the rental's own tuple

/// **`Server protocol and transfers` B8's detected rental tuple, taken on the
/// pre-package surface.**
///
/// That clause names this module the supplier — *"`Server Setup` supplies the
/// detected rental tuple"* — and this is where the machine is asked.
///
/// **It needs nothing on the machine and that is the whole point.** Both of this
/// module's commands are constants naming programs the machine already carries,
/// and the operating-system half is `Servers` Q12's own performance, called
/// rather than sent a second time. So the tuple is read **before any package
/// byte exists on the machine**, which is what lets `Rented-server runtime` G28
/// keep its own sentence: no exact match is an unsupported machine reported
/// before any package byte is executed.
///
/// **Nothing here selects anything and nothing here judges the machine.** The
/// selection is the desktop's, on whole-value equality against a verified
/// manifest, and this hands it either a value or nothing at all.
///
/// **An answer that could not be read is an absence rather than a value**, which
/// is the discipline every other reading in this file holds to: a command that
/// did not end cleanly, a channel that was gone and output that is not text are
/// one outcome, because Smoozy cannot tell them apart.
pub async fn read_the_rentals_tuple(channel: &mut impl CommandChannel) -> TheRentalsTuple {
    let operating_system = smoozy_servers::readings::read_the_operating_system(channel).await;
    let processor = read_one_value(channel, NAME_YOUR_PROCESSOR_ARCHITECTURE).await;
    let c_library = read_one_value(channel, NAME_YOUR_C_LIBRARY).await;
    crate::kind::the_tuple_the_machine_named(
        operating_system,
        processor.as_deref(),
        c_library.as_deref(),
    )
}

/// One reading taken as a value, and an absence wherever it was not.
///
/// **A command that did not end cleanly is an absence and never an empty
/// value**, so a machine that refused the reading is never read as a machine
/// that answered nothing.
async fn read_one_value(
    channel: &mut impl CommandChannel,
    command: CommandString,
) -> Option<String> {
    match channel.run(command).await {
        Ok(result) if result.ended_cleanly() => {
            result.output_as_text().map(|text| text.trim().to_string())
        }
        _ => None,
    }
}

// ------------------------------------------------- the readings at a refusal

/// **`File Browser` P5's two readings, taken at the refusal.**
///
/// **The shape is forced rather than chosen**, and it is worth saying why:
/// `File Browser` P5 takes its readings through a plain trait object, so
/// [`crate::elevation::ReadsTheMachineAtARefusal`] cannot be asynchronous and a
/// machine cannot be reached from inside it. So the acting half takes the three
/// answers at the refusal and this carries them into the judgement.
/// **Nothing is composed**: every field is what the machine said at that
/// moment, and the value is made at the refusal and never before one (P5's
/// timing rule, which is the spec's own).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheMachineAtARefusal {
    kind: MachineKind,
    login_is_root: bool,
    probe: Probe,
}

impl TheMachineAtARefusal {
    /// The three answers, as a value.
    pub fn as_answered(
        kind: MachineKind,
        login_is_root: bool,
        probe: Probe,
    ) -> TheMachineAtARefusal {
        TheMachineAtARefusal {
            kind,
            login_is_root,
            probe,
        }
    }
}

impl crate::elevation::ReadsTheMachineAtARefusal for TheMachineAtARefusal {
    fn the_kind(&mut self) -> MachineKind {
        self.kind
    }

    fn the_login_is_root(&mut self) -> bool {
        self.login_is_root
    }

    fn the_non_interactive_probe(&mut self) -> Probe {
        self.probe
    }
}

/// **E1's weight and M3's probe, taken at a refusal — and E2's timing rule is
/// what makes this a function called at that moment rather than at a
/// connection.**
///
/// *"Elevation is offered only at the moment a step is refused for lack of
/// permission, never before."*
pub async fn take_the_readings_at_a_refusal(
    channel: &mut impl CommandChannel,
    files: &mut impl TheMachinesFileSurface,
) -> TheMachineAtARefusal {
    let kind = read_the_machine_kind(channel, files).await;
    let login_is_root = read_whether_the_login_is_root(channel).await;
    let probe = run_the_probe(channel).await;
    TheMachineAtARefusal {
        kind,
        login_is_root,
        probe,
    }
}

/// **M2's own discriminator on a container**, read as the number the machine
/// prints.
///
/// **A reading that did not answer is not root.** The safe direction is the
/// one that offers the elevation flow rather than the one that assumes the
/// login already has what it needs.
pub async fn read_whether_the_login_is_root(channel: &mut impl CommandChannel) -> bool {
    match channel.run(WHICH_LOGIN_IS_THIS).await {
        Ok(result) if result.ended_cleanly() => result
            .output_as_text()
            .map(|text| text.trim() == "0")
            .unwrap_or(false),
        _ => false,
    }
}

/// **M3: elevation is probed before it is offered, with the elevating command's
/// non-interactive form.**
///
/// **The two outcomes are the whole set and there is no third.** A probe that
/// did not reach the machine is a probe that did not succeed, which is the
/// direction that asks rather than the one that assumes.
pub async fn run_the_probe(channel: &mut impl CommandChannel) -> Probe {
    match channel.run(THE_PROBE).await {
        Ok(result) if result.ended_cleanly() => Probe::Succeeded,
        _ => Probe::Failed,
    }
}

/// **What R1 item 3's probe came to, and the three are kept apart.**
///
/// **A probe that did not run is not a machine without the tool**, which is the
/// same discipline R1 item 1 states for the bus scan — *"the scan returned empty
/// and the scan did not run are two different readings and are never
/// collapsed"*. Here they lead to the same act, and they are still told apart,
/// because what the run may claim about the machine differs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhetherTheUnpackToolIsPresent {
    /// The package database says the package is installed.
    ItIs,
    /// The package database answered and said something else — the package is
    /// absent, or is on the machine in one of the seven states that is not
    /// *installed*.
    ItIsNot,
    /// **The probe did not answer at all**, so nothing was established.
    TheProbeDidNotAnswer,
}

impl WhetherTheUnpackToolIsPresent {
    /// **The question R1 item 3's *where absent* asks, and the safe direction
    /// is spelled out rather than implied.**
    ///
    /// **Only an answer saying the package is installed keeps the install
    /// back.** A probe that did not answer establishes nothing, so the tool is
    /// treated as absent and the install goes out — which costs a machine that
    /// already held it one install it did not need, and the other direction
    /// would cost a run its unpack: with the tool absent, all four versions'
    /// archives refused *"at exit code 2 with nothing landing on disk"*
    /// ([`crate::ready_action::THE_ABSENT_TOOL_FAILURE_IS_LOUD_AND_TOTAL`]).
    pub fn the_tool_is_there(&self) -> bool {
        matches!(self, WhetherTheUnpackToolIsPresent::ItIs)
    }
}

/// **R1 item 3's probe, performed: the package system asked before anything is
/// installed.**
///
/// **Nothing is parsed for a cause and nothing is read off the complaint
/// stream.** What is read is the exit and the one field
/// [`THE_STATUS_FIELD`] names, and the field is matched by whole word, for the
/// reason written at [`THE_INSTALLED_STATE`].
pub async fn probe_the_unpack_tool(
    channel: &mut impl CommandChannel,
) -> WhetherTheUnpackToolIsPresent {
    let Ok(result) = channel.run(THE_UNPACK_TOOLS_PACKAGE_PROBE).await else {
        return WhetherTheUnpackToolIsPresent::TheProbeDidNotAnswer;
    };
    if !result.ended_cleanly() {
        // **The documented exit for a package the database does not hold** —
        // *"1 The requested query failed either fully or partially, due to no
        // file or package being found"*. The machine answered and the answer is
        // *absent*.
        return WhetherTheUnpackToolIsPresent::ItIsNot;
    }
    let Some(text) = result.output_as_text() else {
        return WhetherTheUnpackToolIsPresent::TheProbeDidNotAnswer;
    };
    // **The field is a whole line of the entry**, so the line it opens is the
    // only one read; a word of that spelling anywhere else in the entry — a
    // description, a maintainer's name — is not the status.
    let field = text
        .lines()
        .find(|line| line.starts_with(THE_STATUS_FIELD))
        .map(|line| &line[THE_STATUS_FIELD.len()..]);
    match field {
        Some(said) if said.split_whitespace().any(|word| word == THE_INSTALLED_STATE) => {
            WhetherTheUnpackToolIsPresent::ItIs
        }
        // **The entry is there and its status is one of the other seven.**
        Some(_) => WhetherTheUnpackToolIsPresent::ItIsNot,
        // **A clean exit with no status field is an answer nothing here can
        // read**, so nothing is established from it.
        None => WhetherTheUnpackToolIsPresent::TheProbeDidNotAnswer,
    }
}

// ------------------------------------------------------- the card readings

/// **R1 item 1's card presence, and the two readings are never collapsed.**
///
/// *"So **the scan returned empty** and **the scan did not run** are two
/// different readings and are never collapsed"*, because only the first
/// concludes *no card*. The scan and the driver query are `Servers` H11's own
/// performances and are called rather than written again.
pub async fn read_the_cards(
    channel: &mut impl CommandChannel,
    kind: MachineKind,
) -> smoozy_servers::hardware::Cards {
    let scan = smoozy_servers::readings::scan_the_bus(channel).await;
    let driver = smoozy_servers::readings::ask_the_driver(channel).await;
    crate::ready_action::read_card_presence(scan, driver, kind)
}

/// **R1 item 5's check-first order, as one reading.**
///
/// *"`nvidia-smi` run and its exit read; where it answers, nothing is installed
/// and nothing is touched."* **The exit is the whole of the reading** and no
/// message is parsed for a cause.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheDriverCheckSaid {
    /// The driver answered.
    ItAnswered,
    /// It did not, with the machine's own words kept whole — **both streams,
    /// kept apart.**
    ItDidNot {
        /// **The exit the check ended at**, which R1 item 5 calls the whole of
        /// the reading — *"`nvidia-smi` run and **its exit read**"*.
        ///
        /// **It is here so that R1 item 5's other branch can be decided without
        /// a second reading of the machine**, and it is read by
        /// [`WhatTheDriverCheckSaid::a_driver_is_already_installed`] and by
        /// nothing else. **No message is parsed for a cause** — the two streams
        /// beside this are kept for the user and are never read for a decision,
        /// which is `Security and licence` §13 rule 5.
        ///
        /// **Nothing where the command did not reach the machine at all.**
        the_exit: Option<u32>,
        /// **What it wrote to its complaint stream.**
        the_machines_own_words: String,
        /// **What it wrote to standard output**, kept apart from the complaint
        /// stream and never merged with it — S2a's own discipline, the same
        /// shape [`crate::elevated::WhatTheElevatedStepSaid`] already carries.
        ///
        /// **This field is here because the reading kept only the complaint
        /// stream and the real failure wrote nothing to it. Added 2026-09-04,
        /// out of a build sitting on rented machines.** **Observed in a
        /// controlled test** on Hyperstack `Ubuntu Server 22.04 LTS R570 CUDA
        /// 12.8`, 2026-09-04, with the driver installed and its kernel modules
        /// unloaded — the state a working-but-unbooted install is in: the tool
        /// ended at **9** and wrote its whole 145-byte message, *"NVIDIA-SMI has
        /// failed because it couldn't communicate with the NVIDIA driver…"*, to
        /// **standard output**, with **nothing** on the complaint stream. **So
        /// W1's *Could not be made ready* — which promises the machine's own
        /// words behind it — would have reached the user with none.**
        ///
        /// **Nothing here says where this tool writes in general**, on this
        /// image or any other, and nothing is decided from which stream carried
        /// the text. Both are kept and the exit stays the whole of the reading.
        what_it_wrote_to_standard_output: String,
    },
}

impl WhatTheDriverCheckSaid {
    /// **W1's promise, asked of the reading rather than assumed.**
    ///
    /// *Could not be made ready* is *"named with the machine's own words behind
    /// it"*, and whether there are any is a property of what came back on
    /// **either** stream. **Nothing is merged to answer it** — a caller that
    /// shows the words shows each stream as its own, which is S2a's rule and the
    /// design's drawing.
    pub fn any_words_at_all(&self) -> bool {
        match self {
            WhatTheDriverCheckSaid::ItAnswered => false,
            WhatTheDriverCheckSaid::ItDidNot {
                the_machines_own_words,
                what_it_wrote_to_standard_output,
                ..
            } => {
                !the_machines_own_words.trim().is_empty()
                    || !what_it_wrote_to_standard_output.trim().is_empty()
            }
        }
    }

    /// **R1 item 5's other branch, decided off the check this run has already
    /// taken and off no second reading of the machine.**
    ///
    /// **What the clause turns on.** *"Where it fails and a card is present on
    /// the bus: **where a driver is already installed, the restart is tried
    /// before any install**"* — and *"where **no driver is installed**, the
    /// driver is installed by the distribution's own detection route"*. **No
    /// clause anywhere names a reading for which of the two a machine is in**,
    /// and until this was written the value arrived as a parameter nothing in
    /// the application ever supplied.
    ///
    /// **The ruling: the exit is the reading.** The check's own tool publishes
    /// an exit that means exactly *a driver is installed and is not loaded*, and
    /// it is one of the two failure mechanisms R1 item 5 names — the other, a
    /// card still bound to `nouveau`, leaves the driver equally installed and
    /// equally unloaded. **Documented** — NVIDIA's own `nvidia-smi`
    /// documentation, read **2026-09-20** at
    /// `https://docs.nvidia.com/deploy/nvidia-smi/index.html`, under *RETURN
    /// VALUE*: *"9 NVIDIA driver is not loaded"*, beside *"0 Success"*, *"2 A
    /// supplied argument or flag is invalid"*, *"3 The requested operation is
    /// not available on target device"*, *"4 The current user does not have
    /// permission to access this device or perform this operation"*, *"6 A query
    /// to find an object was unsuccessful"*, *"8 A device's external power
    /// cables are not properly attached"*, *"10 NVIDIA Kernel detected an
    /// interrupt issue with a GPU"*, *"12 NVML Shared Library couldn't be found
    /// or loaded"*, *"13 Local version of NVML doesn't implement this
    /// function"*, *"14 infoROM is corrupted"*, *"15 The GPU has fallen off the
    /// bus or has otherwise become inaccessible"* and *"255 Other error or
    /// internal driver error occurred"*. **Not one of the other twelve says a
    /// driver is installed**, which is why the reading is this one number and
    /// never a range.
    ///
    /// **Observed in a test, and it is this project's own.** On Hyperstack
    /// `Ubuntu Server 22.04 LTS R570 CUDA 12.8`, 2026-09-04, **with the driver
    /// installed and its kernel modules unloaded — the state a
    /// working-but-unbooted install is in — the tool ended at 9** and wrote
    /// its whole message. That observation is already recorded beside
    /// [`WhatTheDriverCheckSaid::ItDidNot`]'s standard-output field and was
    /// taken for a different reason; **the exit it names is the one the
    /// publication above names.**
    ///
    /// **It costs no round trip.** R1 item 5 runs this check first whatever
    /// happens, and R1 item 1's readings run it once more before the run is
    /// composed, so the value is a second thing read off a reading already
    /// taken rather than a question asked of the machine.
    ///
    /// **The safe direction, and it is not symmetric.** Every other outcome —
    /// any other exit, and a command that did not reach the machine — reads as
    /// **no driver installed**, because the tool itself is what a driver
    /// install puts there: a machine with no driver at all has nothing to run,
    /// so it cannot produce this exit. **A wrong *a driver is installed* costs a
    /// paid machine a restart of 9 to 90 seconds that fixes nothing, and then a
    /// check that still fails** — the machine is never made ready. **A wrong *no
    /// driver installed* costs one install**, which is R1 item 5's own
    /// reinstall-becomes-upgrade harm. **Neither is free, and the direction is
    /// chosen by which one is reachable**: the exit is published and exact, so
    /// the false negative only arises where the tool is gone, which is the
    /// driverless machine the install branch is for.
    ///
    /// **Narrowed 2026-09-30, `Server Setup` R1 item 5 as corrected that day:
    /// *"A driver counts as installed for this rule only where the running
    /// kernel has its module."*** Exit 9 was read alone, and on machine 1 of
    /// step 10c's eleventh pass a driver's utilities stood with **no module for
    /// any kernel**: the tool ended at 9, every *Set up again* took the restart
    /// before any install, came back with nothing to load, and ended the same
    /// way — no press in Smoozy could mend the machine. **The code was written
    /// for a module that is there and not loaded; it now also meets a module
    /// that is not there at all**, and the moment the reading and the machine
    /// parted company was a driver this called installed with no module under
    /// it. So exit 9 now counts only beside [`TheRunningKernelsModule::IsThere`],
    /// read by [`read_the_running_kernels_module`] — and only where the exit
    /// was 9, so no other machine is asked anything more.
    pub fn a_driver_is_already_installed(&self, module: TheRunningKernelsModule) -> bool {
        self.the_exit_says_a_driver_is_installed() && module == TheRunningKernelsModule::IsThere
    }

    /// **Whether the check ended at the one exit that says a driver is
    /// installed and not loaded** — the half of the reading the check alone
    /// gives, and the only case in which the module is worth reading.
    pub fn the_exit_says_a_driver_is_installed(&self) -> bool {
        matches!(
            self,
            WhatTheDriverCheckSaid::ItDidNot {
                the_exit: Some(THE_EXIT_THAT_MEANS_A_DRIVER_IS_INSTALLED_AND_NOT_LOADED),
                ..
            }
        )
    }
}

/// **`Server Setup` R1 item 5: whether the running kernel has the driver's
/// module.** Added 2026-09-30, eleventh-pass fix piece 10.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheRunningKernelsModule {
    /// The kernel's own module directory holds a module by the driver's name.
    IsThere,
    /// **It holds none, or the reading established nothing.** Both read as
    /// *not there*, which sends the machine down the install route — the route
    /// a bare machine takes. **The direction is R1 item 5's own**: the restart
    /// *"cannot fix an absent"* module, and a wrong *there* is the fault this
    /// reading was added for, a machine no press can mend. A wrong *not there*
    /// costs one pass of the family's install route, whose commands a machine
    /// already holding the driver answers with nothing to do (below).
    IsNotThere,
}

/// **`Server Setup` R1 item 5's module reading on Ubuntu**: the kernel's own
/// module loader asked, without loading anything, whether it can find the
/// driver's module for the running kernel.
///
/// **Documented** — `modprobe(8)`, kmod 31+20240202-2ubuntu7.2, Ubuntu noble,
/// `https://manpages.ubuntu.com/manpages/noble/en/man8/modprobe.8.html`, read
/// **2026-09-30**:
/// - *"modprobe looks in the module directory /lib/modules/`uname -r` for all
///   the modules and other files, except for the optional configuration files
///   in the /etc/modprobe.d directory"* — **the running kernel's directory and
///   no other**, which is exactly R1 item 5's *"the running kernel has its
///   module"*;
/// - *"-n, --dry-run, --show: This option does everything but actually insert
///   or delete the modules (or run the install or remove commands)"* — **so
///   nothing is loaded and the machine is not touched**; R5 forbids a module
///   unload and reload in place of the restart, and this is neither;
/// - *"-q, --quiet: With this flag, modprobe won't print an error message if
///   you try to remove or insert a module it can't find … However, it will
///   still return with a non-zero exit status."*
///
/// **And its own source says the found case ends at 0.** kmod's
/// `tools/modprobe.c` at tag v31 (`https://raw.githubusercontent.com/kmod-project/kmod/v31/tools/modprobe.c`,
/// read **2026-09-30**): a name it cannot look up returns *"-ENOENT"* after
/// *"Module %s not found in directory %s"*, and the program ends *"return err
/// >= 0 ? EXIT_SUCCESS : EXIT_FAILURE;"*. **It reads the configuration
/// directory**, so an alias a family's driver sets for the name `nvidia` is
/// followed — which is why this is `modprobe` and not `modinfo`, which the
/// same page says *"knows nothing of aliases or install commands"*.
///
/// **Where the program is.** Ubuntu's own file index for `kmod` in noble,
/// `https://packages.ubuntu.com/noble/amd64/kmod/filelist`, read
/// **2026-09-30**, lists `/usr/sbin/modprobe`. **The path is written whole**
/// because whether an unprivileged login's search path reaches `/usr/sbin` on
/// an arbitrary rental is read nowhere here, and a program not found would read
/// as *no module* on every machine and take away R1 item 5's restart-first
/// branch without a word.
///
/// **Not elevated**: the reading loads nothing and the source read above checks
/// no user. **The name is the driver's module's own, `nvidia`.** Ubuntu's own
/// contents index,
/// `https://packages.ubuntu.com/search?searchon=contents&keywords=nvidia.ko&mode=exactfilename&suite=noble-updates&arch=amd64`,
/// read **2026-09-30**, gives for example
/// `/lib/modules/6.17.0-40-generic/kernel/nvidia-610-open/nvidia.ko` in
/// `linux-modules-nvidia-610-open-6.17.0-40-generic` — the kind of package the
/// detection route installs beside the driver — so the module is `nvidia` and
/// it sits in one kernel's own directory. On Debian the name is an alias, which
/// R1 item 5's own record of that family shows followed: *"`modprobe nvidia`
/// reports `FATAL: Module nvidia-current not found in directory …`"*.
pub const THE_MODULE_READING_ON_UBUNTU: CommandString =
    CommandString::constant("/usr/sbin/modprobe --dry-run --quiet nvidia");

/// **The same reading on Debian**, at the path that family's own file index
/// gives: `https://packages.debian.org/bookworm/amd64/kmod/filelist`, read
/// **2026-09-30**, lists `/sbin/modprobe`. Everything else is
/// [`THE_MODULE_READING_ON_UBUNTU`]'s.
pub const THE_MODULE_READING_ON_DEBIAN: CommandString =
    CommandString::constant("/sbin/modprobe --dry-run --quiet nvidia");

/// **`Server Setup` R1 item 5's module reading, for one family.**
pub fn the_module_reading_for(family: crate::kind::DistributionFamily) -> CommandString {
    match family {
        crate::kind::DistributionFamily::Ubuntu => THE_MODULE_READING_ON_UBUNTU,
        crate::kind::DistributionFamily::Debian => THE_MODULE_READING_ON_DEBIAN,
    }
}

/// **`Server Setup` R1 item 5: whether the running kernel has the driver's
/// module, asked only where the check ended at 9.** Added 2026-09-30,
/// eleventh-pass fix piece 10.
///
/// **The exit is the whole of the reading**, as it is for the check: 0 is
/// *there*, and any other exit, or a command that never reached the machine,
/// is *not there* ([`TheRunningKernelsModule::IsNotThere`] says why that is
/// the direction). **Where the check did not end at 9 nothing is asked**,
/// because the answer could not change what the step does.
pub async fn read_the_running_kernels_module(
    channel: &mut impl CommandChannel,
    family: crate::kind::DistributionFamily,
    check: &WhatTheDriverCheckSaid,
) -> TheRunningKernelsModule {
    if !check.the_exit_says_a_driver_is_installed() {
        return TheRunningKernelsModule::IsNotThere;
    }
    match channel.run(the_module_reading_for(family)).await {
        Ok(result) if result.ended_cleanly() => TheRunningKernelsModule::IsThere,
        _ => TheRunningKernelsModule::IsNotThere,
    }
}

/// **`Server Setup` R1 item 5: what the package system holds of a driver**,
/// read where the check failed and no restart is owed. Added 2026-11-03, out of
/// the Sesterce live pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheDriverPackagesSay {
    /// **A driver package is installed**: the module may want a boot, and the
    /// route goes as it always did.
    ADriverPackageStands,
    /// **The kernel half stands with no driver package on it**: a removal
    /// stopped part-way, and the step finishes it.
    AKernelHalfIsLeftBehind,
    /// **Neither.** A machine with nothing to remove, or a removal that stopped
    /// later than the kernel half, takes the detection route; and after that
    /// route, this is what says it left nothing.
    NoDriverPackage,
    /// **The reading could not be had, or was not taken.** Nothing is concluded
    /// from it, and everything goes as it did before this reading existed.
    NotRead,
}

/// **The answer to [`crate::elevated::THE_DRIVER_PACKAGES_READING`], whole.**
pub fn what_the_driver_packages_say(output: &str) -> WhatTheDriverPackagesSay {
    let output = output.trim();
    if output.is_empty() {
        WhatTheDriverPackagesSay::NoDriverPackage
    } else if output == "a driver stands" {
        WhatTheDriverPackagesSay::ADriverPackageStands
    } else {
        WhatTheDriverPackagesSay::AKernelHalfIsLeftBehind
    }
}

/// **`Server Setup` R1 item 5's reading, taken on Ubuntu and nowhere else.**
/// A command that did not reach the machine, or did not end cleanly, is
/// [`WhatTheDriverPackagesSay::NotRead`].
pub async fn read_the_driver_packages(
    channel: &mut impl CommandChannel,
    family: crate::kind::DistributionFamily,
) -> WhatTheDriverPackagesSay {
    if family != crate::kind::DistributionFamily::Ubuntu {
        return WhatTheDriverPackagesSay::NotRead;
    }
    match channel.run(crate::elevated::THE_DRIVER_PACKAGES_READING).await {
        Ok(result) if result.ended_cleanly() => {
            what_the_driver_packages_say(&String::from_utf8_lossy(&result.output))
        }
        _ => WhatTheDriverPackagesSay::NotRead,
    }
}

/// **The published exit that means a driver is installed on the machine and is
/// not loaded**, which is R1 item 5's *"where a driver is already installed"* in
/// the tool's own words.
///
/// **Documented** — NVIDIA's own `nvidia-smi` documentation, read **2026-09-20**
/// at `https://docs.nvidia.com/deploy/nvidia-smi/index.html`, *RETURN VALUE*:
/// *"9 NVIDIA driver is not loaded"*. The whole list, and why no other value
/// carries the meaning, is at
/// [`WhatTheDriverCheckSaid::a_driver_is_already_installed`].
pub const THE_EXIT_THAT_MEANS_A_DRIVER_IS_INSTALLED_AND_NOT_LOADED: u32 = 9;

/// **R1 item 5 and item 6: the check, whose failure is the only route to a
/// restart.**
///
/// **Fact 80's rule as code**: restart only because this check failed, never on
/// the package manager's word and never unconditionally.
pub async fn run_the_driver_check(channel: &mut impl CommandChannel) -> WhatTheDriverCheckSaid {
    match channel.run(THE_DRIVER_CHECK).await {
        Ok(result) if result.ended_cleanly() => WhatTheDriverCheckSaid::ItAnswered,
        // **Whole or not at all, streams kept apart** — S2a's discipline. **Both
        // are kept**: this reading used to keep the complaint stream alone, on
        // the strength of a sentence saying that is where the tool's refusal
        // lands, and the one real failure anyone has watched wrote its whole
        // message to standard output instead. Neither stream is a fact about
        // this tool that anybody here has read, so neither is dropped.
        Ok(result) => WhatTheDriverCheckSaid::ItDidNot {
            // **The exit, which R1 item 5 calls the whole of the reading**, kept
            // so that the clause's *where a driver is already installed* branch
            // is decided without asking the machine a second question.
            the_exit: result.status,
            the_machines_own_words: String::from_utf8_lossy(&result.complaint).into_owned(),
            what_it_wrote_to_standard_output: String::from_utf8_lossy(&result.output).into_owned(),
        },
        // **The request did not reach the machine, so there are no words on
        // either stream** — and [`WhatTheDriverCheckSaid::any_words_at_all`]
        // says so rather than the caller assuming there are some.
        Err(_) => WhatTheDriverCheckSaid::ItDidNot {
            // **Nothing reached the machine, so there is no exit either**, and
            // a run that cannot reach the machine establishes nothing about
            // what is installed on it.
            the_exit: None,
            the_machines_own_words: String::new(),
            what_it_wrote_to_standard_output: String::new(),
        },
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::kind::DistributionFamily;
    use smoozy_server_protocol::transport::ssh::CommandFailure;
    use smoozy_server_protocol::transport::CommandResult;

    /// A machine whose answers are written by hand, which is what keeps every
    /// act above testable at the desk.
    ///
    /// **It refuses a write, a make-directory and a planted link into a folder
    /// it has not been told about, and that is the whole point of
    /// [`AMachine::directories`].** Until 2026-09-04 this stand-in answered
    /// `Ok(())` to a make-directory for any path at all and to a write for any
    /// path at all — **which is why the placement completed here on a machine
    /// with nothing on it, while nothing in the application created Smoozy's own
    /// root** (R7, repaired the same day). **The plant went on answering `Ok(())`
    /// for any path at all after that, and hid the same fault a second time**:
    /// every carriage token is written under a Smoozy-named directory that
    /// nothing in this crate made. A stand-in that says yes to everything is a
    /// stand-in that can prove nothing.
    #[derive(Default)]
    pub(crate) struct AMachine {
        pub answers: Vec<(&'static str, CommandResult)>,
        pub files: Vec<(String, Vec<u8>)>,
        /// **The directories the machine actually holds.** The filesystem root
        /// and the login's own starting directory are always there; everything
        /// else has to have been made.
        pub directories: Vec<String>,
        /// **The links the machine actually holds**, each with what it points
        /// at, so a removal of one is a removal rather than a miss.
        pub planted: Vec<(String, String)>,
        pub asked: Vec<&'static str>,
        /// **What arrived on a command's standard input, command by command**
        /// — `Server protocol and transfers` 1.1's second carriage, so a test
        /// can say the value reached the command that way and not another.
        pub carried_on_standard_input: Vec<(&'static str, Vec<u8>)>,
        pub written: Vec<(String, Vec<u8>)>,
        pub the_channel_is_gone: bool,
    }

    impl AMachine {
        pub(crate) fn answering(answers: Vec<(&'static str, CommandResult)>) -> AMachine {
            AMachine {
                answers,
                ..AMachine::default()
            }
        }

        /// A file the machine already holds, **with the folders above it**,
        /// which is what a machine holding a file actually looks like.
        pub(crate) fn holding(mut self, path: &str, bytes: &str) -> AMachine {
            if let Some(folder) = the_folder_above(path) {
                self = self.with_directory(folder);
            }
            self.files.push((path.to_string(), bytes.as_bytes().to_vec()));
            self
        }

        /// A folder the machine already holds, **and the folders above it**.
        pub(crate) fn with_directory(mut self, path: &str) -> AMachine {
            if let Some(above) = the_folder_above(path) {
                self = self.with_directory(above);
            }
            if !self.directories.iter().any(|held| held == path) {
                self.directories.push(path.to_string());
            }
            self
        }

        fn it_holds_the_folder(&self, path: &str) -> bool {
            match the_folder_above(path) {
                // The filesystem root and the login's own starting directory
                // are the two that are always there.
                None => true,
                Some(above) => self.directories.iter().any(|held| held == above),
            }
        }
    }

    /// The folder a path sits in, or nothing where the path sits directly in
    /// the filesystem root or in the login's own starting directory.
    pub(crate) fn the_folder_above(path: &str) -> Option<&str> {
        match path.rfind('/') {
            None | Some(0) => None,
            Some(at) => Some(&path[..at]),
        }
    }

    pub(crate) fn said(output: &str, status: u32) -> CommandResult {
        CommandResult {
            output: output.as_bytes().to_vec(),
            complaint: Vec::new(),
            status: Some(status),
        }
    }

    pub(crate) fn complained(complaint: &str, status: u32) -> CommandResult {
        CommandResult {
            output: Vec::new(),
            complaint: complaint.as_bytes().to_vec(),
            status: Some(status),
        }
    }

    impl CommandChannel for AMachine {
        async fn run(
            &mut self,
            command: CommandString,
        ) -> Result<CommandResult, CommandFailure> {
            if self.the_channel_is_gone {
                return Err(CommandFailure::TheRequestDidNotReachTheMachine);
            }
            self.asked.push(command.as_str());
            match self
                .answers
                .iter()
                .find(|(named, _)| *named == command.as_str())
            {
                Some((_, result)) => Ok(result.clone()),
                None => Err(CommandFailure::TheRequestDidNotReachTheMachine),
            }
        }
    }

    impl smoozy_server_protocol::transport::CarriesAValueOnACommandsStandardInput for AMachine {
        /// **The second carriage, driven by hand.** The value is recorded
        /// against the command it rode with, so a test can assert both that it
        /// arrived and that it never entered the command string.
        async fn run_carrying_a_value_on_its_standard_input(
            &mut self,
            command: CommandString,
            value: smoozy_security_and_licence::plaintext::Plaintext,
        ) -> Result<CommandResult, CommandFailure> {
            if self.the_channel_is_gone {
                return Err(CommandFailure::TheRequestDidNotReachTheMachine);
            }
            self.asked.push(command.as_str());
            self.carried_on_standard_input
                .push((command.as_str(), value.expose(<[u8]>::to_vec)));
            match self
                .answers
                .iter()
                .find(|(named, _)| *named == command.as_str())
            {
                Some((_, result)) => Ok(result.clone()),
                None => Err(CommandFailure::TheRequestDidNotReachTheMachine),
            }
        }
    }

    impl TheMachinesFileSurface for AMachine {
        async fn read_whole(&mut self, path: &str) -> Result<Vec<u8>, WhatTheMachineSaid> {
            // **The channel gone answers every read so** — the connection's,
            // and nothing about the path (2026-10-26, fault 2).
            if self.the_channel_is_gone {
                return Err(WhatTheMachineSaid::TheChannelIsGone);
            }
            match self.files.iter().find(|(held, _)| held == path) {
                Some((_, bytes)) => Ok(bytes.clone()),
                None => Err(WhatTheMachineSaid::ThePathIsNotThere),
            }
        }

        async fn write_whole(
            &mut self,
            path: &str,
            bytes: &[u8],
        ) -> Result<(), WhatTheMachineSaid> {
            // **A write into a folder nobody made is refused**, which is what a
            // real machine does and what this stand-in used to hide.
            if !self.it_holds_the_folder(path) {
                return Err(WhatTheMachineSaid::ThePathIsNotThere);
            }
            self.written.push((path.to_string(), bytes.to_vec()));
            self.files.retain(|(held, _)| held != path);
            self.files.push((path.to_string(), bytes.to_vec()));
            Ok(())
        }

        async fn read_link(&mut self, path: &str) -> Result<Vec<u8>, WhatTheMachineSaid> {
            self.planted.iter().find(|(at, _)| at == path)
                .map(|(_, target)| target.as_bytes().to_vec()).ok_or(WhatTheMachineSaid::ThePathIsNotThere)
        }

        async fn replace_by_rename(&mut self, from: &str, to: &str) -> Result<(), WhatTheMachineSaid> {
            let Some((_, target)) = self.planted.iter().find(|(at, _)| at == from).cloned() else {
                return Err(WhatTheMachineSaid::ThePathIsNotThere);
            };
            self.planted.retain(|(at, _)| at != from && at != to);
            self.files.retain(|(at, _)| at != to);
            self.planted.push((to.to_string(), target));
            Ok(())
        }

        async fn make_one_directory(&mut self, path: &str) -> Result<(), WhatTheMachineSaid> {
            if self.directories.iter().any(|held| held == path) {
                // The plain make-directory's own refusal over an existing
                // folder, which `crate::wiring` reads as this value.
                return Err(WhatTheMachineSaid::ItIsAlreadyThere);
            }
            // **No make-parents form exists on the subsystem**, so a directory
            // whose parent is not there is refused rather than made deep.
            if !self.it_holds_the_folder(path) {
                return Err(WhatTheMachineSaid::ThePathIsNotThere);
            }
            self.directories.push(path.to_string());
            Ok(())
        }

        async fn remove_one_file(&mut self, path: &str) -> Result<(), WhatTheMachineSaid> {
            let held = self.files.iter().any(|(held, _)| held == path)
                || self.planted.iter().any(|(held, _)| held == path);
            self.files.retain(|(name, _)| name != path);
            self.planted.retain(|(name, _)| name != path);
            if held {
                Ok(())
            } else {
                Err(WhatTheMachineSaid::ThePathIsNotThere)
            }
        }

        async fn remove_one_empty_directory(
            &mut self,
            path: &str,
        ) -> Result<(), WhatTheMachineSaid> {
            self.directories.retain(|held| held != path);
            Ok(())
        }

        async fn list_one_directory(
            &mut self,
            path: &str,
        ) -> Result<Vec<AListedThing>, WhatTheMachineSaid> {
            let prefix = format!("{path}/");
            let mut out = Vec::new();
            for (held, _) in &self.files {
                if let Some(rest) = held.strip_prefix(&prefix) {
                    if !rest.contains('/') {
                        out.push(AListedThing {
                            name: rest.as_bytes().to_vec(),
                            it_is_a_directory: false,
                        });
                    }
                }
            }
            Ok(out)
        }

        /// **A link planted into a folder nobody made is refused**, exactly as a
        /// write into one is.
        ///
        /// **This answered `Ok(())` for any path at all until 2026-09-04**, and
        /// that is the same shape as the make-directory and the write it sat
        /// beside: a stand-in saying yes to every write is what hid the root's
        /// missing creation, and this one hid the carriage directory's. Every
        /// carriage token is written under a Smoozy-named directory the login's
        /// own area has none of until something makes it.
        async fn plant_a_link(&mut self, target: &str, at: &str) -> Result<(), WhatTheMachineSaid> {
            if !self.it_holds_the_folder(at) {
                return Err(WhatTheMachineSaid::ThePathIsNotThere);
            }
            // A site carries one token, so a link left at it is replaced.
            self.planted.retain(|(held, _)| held != at);
            self.planted.push((at.to_string(), target.to_string()));
            Ok(())
        }
    }

    /// **The time driver is on**, because B1's bound on the version ask is a
    /// real deadline and a runtime without one panics rather than waiting.
    pub(crate) fn block_on<F: std::future::Future>(future: F) -> F::Output {
        tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .build()
            .expect("a runtime for the test")
            .block_on(future)
    }

    /// **The same runtime with its clock paused**, so a test of a deadline ends
    /// at once instead of waiting the deadline out.
    ///
    /// **This is what keeps a deadline's own test from hanging**: with time
    /// paused the runtime advances the clock itself the moment there is nothing
    /// left to do, so a bound that fires is a test that finishes and a bound
    /// that does not fire is a test that fails rather than a test that runs
    /// forever. **Two tests in this build step were found hanging rather than
    /// failing when their subject was broken, and both were deadlines.**
    pub(crate) fn block_on_with_time_paused<F: std::future::Future>(future: F) -> F::Output {
        tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .start_paused(true)
            .build()
            .expect("a runtime for the test")
            .block_on(future)
    }

    /// A machine that takes the command and never answers — the shape B1's
    /// bound exists for.
    #[derive(Default)]
    pub(crate) struct AMachineThatNeverAnswers;

    impl CommandChannel for AMachineThatNeverAnswers {
        async fn run(&mut self, _command: CommandString) -> Result<CommandResult, CommandFailure> {
            std::future::pending().await
        }
    }

    impl smoozy_server_protocol::transport::CarriesAValueOnACommandsStandardInput
        for AMachineThatNeverAnswers
    {
        async fn run_carrying_a_value_on_its_standard_input(
            &mut self,
            _command: CommandString,
            _value: smoozy_security_and_licence::plaintext::Plaintext,
        ) -> Result<CommandResult, CommandFailure> {
            std::future::pending().await
        }
    }

    impl TheMachinesFileSurface for AMachineThatNeverAnswers {
        async fn read_whole(&mut self, _path: &str) -> Result<Vec<u8>, WhatTheMachineSaid> {
            Err(WhatTheMachineSaid::ThePathIsNotThere)
        }
        async fn write_whole(
            &mut self,
            _path: &str,
            _bytes: &[u8],
        ) -> Result<(), WhatTheMachineSaid> {
            Err(WhatTheMachineSaid::ThePathIsNotThere)
        }
        /// **The one file act this stand-in answers, and it is answered so the
        /// bound is still what its test proves.** The carriage directory is made
        /// before a token is written into it, so a refusal here would end the
        /// version ask before a command ever left and the deadline would be
        /// tested by nothing.
        async fn make_one_directory(&mut self, _path: &str) -> Result<(), WhatTheMachineSaid> {
            Ok(())
        }
        async fn remove_one_file(&mut self, _path: &str) -> Result<(), WhatTheMachineSaid> {
            Ok(())
        }
        async fn remove_one_empty_directory(
            &mut self,
            _path: &str,
        ) -> Result<(), WhatTheMachineSaid> {
            Ok(())
        }
        async fn list_one_directory(
            &mut self,
            _path: &str,
        ) -> Result<Vec<AListedThing>, WhatTheMachineSaid> {
            Ok(Vec::new())
        }
        async fn plant_a_link(&mut self, _target: &str, _at: &str) -> Result<(), WhatTheMachineSaid> {
            Ok(())
        }
    }

    #[test]
    fn every_command_this_module_sends_is_a_constant_naming_no_value() {
        // **`Server protocol and transfers` 1.1's prohibition, checked over the
        // whole set rather than at each call site.** A command holding a path, a
        // version, a package name read off the machine or a login would show up
        // here as a character no fixed command needs.
        for command in EVERY_COMMAND_THIS_MODULE_SENDS {
            let text = command.as_str();
            assert!(!text.is_empty());
            assert!(
                !text.contains('$') && !text.contains('`') && !text.contains('\''),
                "{text} looks composed rather than constant"
            );
        }
        // The one command naming a carriage names its token and nothing else.
        assert!(ASK_THE_BUILD_ITS_VERSION
            .as_str()
            .starts_with(THE_VERSION_ASKS_CARRIAGE_SITE.named_in_a_command_as));
    }

    #[test]
    fn neither_form_of_the_search_restricts_it_by_kind_of_file_beyond_leaving_folders_out() {
        // **B10 names one restriction and it is by name**; B11 names exactly two
        // honest limits and neither is a kind of file. **Both forms carried
        // `-type f` until 2026-09-04**, and the elevated retry is *"the same
        // constant search command as the unprivileged pass"*, so a retry that
        // kept it would not be the same command at all.
        //
        // **From 2026-09-26 both leave directories out and nothing else**, on
        // POSIX's own definition of `find` (cited at [`THE_SEARCH`]): a folder
        // is not a build, and step 10c's sixth pass on Vast watched a thumbnail
        // cache folder offered as one. **`-type f` stays forbidden**, because a
        // link to a build — `/snap/bin/blender` — is not a regular file and must
        // stay a candidate.
        for text in [
            THE_SEARCH.as_str(),
            crate::elevated::THE_ELEVATED_SEARCH.plain.as_str(),
            crate::elevated::THE_ELEVATED_SEARCH.elevated.as_str(),
            crate::elevated::THE_ELEVATED_SEARCH
                .elevated_reading_the_password_from_standard_input
                .as_str(),
        ] {
            assert_eq!(text.matches("-type").count(), 1, "{text}");
            assert!(text.contains("! -type d"), "{text} does not leave folders out");
            assert!(!text.contains("-type f"), "{text} restricts to regular files");
            assert!(text.contains("-name blender"));
            assert!(text.contains("find / "));
            // **B10, 2026-10-02: the kernel's two trees are pruned and nothing
            // else is** — no `-xdev`, no `-mount`, no other `-path`.
            assert!(text.contains("-path /proc -prune -o "), "{text} walks /proc");
            assert!(text.contains("-path /sys -prune -o "), "{text} walks /sys");
            assert_eq!(text.matches("-path ").count(), 2, "{text}");
            assert!(!text.contains("-xdev") && !text.contains("-mount"), "{text} stays on one filesystem");
        }
    }

    #[test]
    fn the_container_marker_places_a_container() {
        let mut machine = AMachine::answering(vec![
            (WHAT_SERVERS_SENDS_FOR_THE_CONTAINER_ROW[0], said("", 0)),
        ]);
        let mut files = AMachine::default();
        assert_eq!(
            block_on(read_the_machine_kind(&mut machine, &mut files)),
            MachineKind::AContainer
        );
    }

    #[test]
    fn systemd_as_the_first_process_places_a_systemd_machine() {
        let mut machine = AMachine::answering(vec![
            (WHAT_SERVERS_SENDS_FOR_THE_CONTAINER_ROW[0], said("", 1)),
            (WHAT_SERVERS_SENDS_FOR_THE_CONTAINER_ROW[1], said("none\n", 1)),
        ]);
        let mut files = AMachine::default().holding(THE_FIRST_PROCESSS_NAME, "systemd\n");
        assert_eq!(
            block_on(read_the_machine_kind(&mut machine, &mut files)),
            MachineKind::ASystemdMachine
        );
    }

    #[test]
    fn a_machine_whose_first_process_is_not_systemd_is_not_placed() {
        // **`Rented-server runtime` K3's row, reached rather than guessed at.**
        let mut machine = AMachine::answering(vec![
            (WHAT_SERVERS_SENDS_FOR_THE_CONTAINER_ROW[0], said("", 1)),
            (WHAT_SERVERS_SENDS_FOR_THE_CONTAINER_ROW[1], said("none\n", 1)),
        ]);
        let mut files = AMachine::default().holding(THE_FIRST_PROCESSS_NAME, "init\n");
        assert_eq!(
            block_on(read_the_machine_kind(&mut machine, &mut files)),
            MachineKind::TheTableDoesNotPlaceIt
        );
    }

    #[test]
    fn an_unreadable_first_process_is_never_read_as_systemd() {
        // The file surface holds nothing, so the read refuses. M1a's rule
        // applied to the kind: an absent answer is not an answer.
        let mut machine = AMachine::answering(vec![
            (WHAT_SERVERS_SENDS_FOR_THE_CONTAINER_ROW[0], said("", 1)),
            (WHAT_SERVERS_SENDS_FOR_THE_CONTAINER_ROW[1], said("", 1)),
        ]);
        let mut files = AMachine::default();
        assert_eq!(
            block_on(read_the_machine_kind(&mut machine, &mut files)),
            MachineKind::TheTableDoesNotPlaceIt
        );
    }

    #[test]
    fn the_opening_reading_reads_the_second_file_only_where_the_first_is_absent() {
        let mut machine = AMachine::answering(vec![
            (WHAT_SERVERS_SENDS_FOR_THE_CONTAINER_ROW[0], said("", 1)),
            (WHAT_SERVERS_SENDS_FOR_THE_CONTAINER_ROW[1], said("none", 1)),
        ]);
        let mut files = AMachine::default()
            .holding(THE_FIRST_PROCESSS_NAME, "systemd")
            .holding(WhichOsReleaseFile::Etc.path(), "ID=ubuntu\n")
            .holding(WhichOsReleaseFile::UsrLib.path(), "ID=debian\n");
        let mut taken = block_on(take_the_opening_readings(&mut machine, &mut files));
        let reading = crate::kind::read_before_anything_is_touched(&mut taken);
        assert_eq!(reading.kind, MachineKind::ASystemdMachine);
        assert_eq!(reading.family.family(), Some(DistributionFamily::Ubuntu));
        assert!(reading.a_run_may_be_composed());
    }

    /// **The worldwide pass's fault 2: a reading that never reached the
    /// machine is said beside the readings**, so whoever tells a refusal can
    /// tell a connection that went from a machine Smoozy does not support. The
    /// kind and the family are judged exactly as before — an absent answer is
    /// still neither systemd nor a family. Added 2026-10-26.
    #[test]
    fn worldwide_fault_2_a_reading_that_never_reached_the_machine_is_said_beside_them() {
        // **The connection gone under every reading**: the container's two
        // commands never land, and every file read finds its channel gone.
        let mut machine = AMachine {
            the_channel_is_gone: true,
            ..AMachine::default()
        };
        let mut files = AMachine {
            the_channel_is_gone: true,
            ..AMachine::default()
        };
        let mut taken = block_on(take_the_opening_readings(&mut machine, &mut files));
        assert!(taken.a_reading_never_reached_the_machine(), "{taken:?}");
        let reading = crate::kind::read_before_anything_is_touched(&mut taken);
        assert_eq!(reading.kind, MachineKind::TheTableDoesNotPlaceIt, "an absent answer is still no kind");
        assert!(!reading.a_run_may_be_composed());

        // **The kind read, and the family's read gone under it** — the order a
        // reconnect part-way through the readings takes.
        let mut machine = AMachine::answering(vec![(WHAT_SERVERS_SENDS_FOR_THE_CONTAINER_ROW[0], said("", 0))]);
        let mut files = AMachine {
            the_channel_is_gone: true,
            ..AMachine::default()
        };
        let taken = block_on(take_the_opening_readings(&mut machine, &mut files));
        assert_eq!(taken.kind(), MachineKind::AContainer);
        assert!(taken.a_reading_never_reached_the_machine(), "{taken:?}");

        // **A machine that answered every reading says nothing of the kind**,
        // and nor does one whose files are simply not there.
        let mut machine = AMachine::answering(vec![(WHAT_SERVERS_SENDS_FOR_THE_CONTAINER_ROW[0], said("", 0))]);
        let mut files = AMachine::default().holding(WhichOsReleaseFile::Etc.path(), "ID=ubuntu\nVERSION_ID=\"22.04\"\n");
        assert!(!block_on(take_the_opening_readings(&mut machine, &mut files)).a_reading_never_reached_the_machine());
        let mut machine = AMachine::answering(vec![
            (WHAT_SERVERS_SENDS_FOR_THE_CONTAINER_ROW[0], said("", 1)),
            (WHAT_SERVERS_SENDS_FOR_THE_CONTAINER_ROW[1], said("none", 1)),
        ]);
        let mut files = AMachine::default();
        let taken = block_on(take_the_opening_readings(&mut machine, &mut files));
        assert!(!taken.a_reading_never_reached_the_machine(), "{taken:?}");
        assert_eq!(taken.kind(), MachineKind::TheTableDoesNotPlaceIt);
        // **And the value a caller composes** carries it only when asked to.
        let composed = TheMachineBeforeAnythingIsTouched::as_answered(MachineKind::AContainer, None, None);
        assert!(!composed.a_reading_never_reached_the_machine());
        assert!(composed.with_a_reading_that_never_reached_the_machine().a_reading_never_reached_the_machine());
    }

    #[test]
    fn a_machine_that_will_not_say_which_family_refuses_the_run() {
        let mut machine = AMachine::answering(vec![
            (WHAT_SERVERS_SENDS_FOR_THE_CONTAINER_ROW[0], said("", 1)),
            (WHAT_SERVERS_SENDS_FOR_THE_CONTAINER_ROW[1], said("none", 1)),
        ]);
        let mut files = AMachine::default().holding(THE_FIRST_PROCESSS_NAME, "systemd");
        let mut taken = block_on(take_the_opening_readings(&mut machine, &mut files));
        let reading = crate::kind::read_before_anything_is_touched(&mut taken);
        assert!(!reading.a_run_may_be_composed());
    }

    /// A machine answering the three readings the tuple is composed from.
    fn a_machine_that_says(
        operating_system: &'static str,
        processor: &'static str,
        c_library: &'static str,
    ) -> AMachine {
        AMachine::answering(vec![
            ("uname -s", said(operating_system, 0)),
            (
                NAME_YOUR_PROCESSOR_ARCHITECTURE.as_str(),
                said(processor, 0),
            ),
            (NAME_YOUR_C_LIBRARY.as_str(), said(c_library, 0)),
        ])
    }

    /// One manifest naming one artifact, at the tuple this build produced.
    fn a_manifest_of_this_build() -> smoozy_server_protocol::release::manifest::ReleaseManifest {
        use smoozy_server_protocol::release::composing;
        composing::compose_a_manifest(
            1,
            "0.1.0",
            2,
            None,
            vec![composing::compose_an_artifact_entry(
                "linux-x86-64-gnu",
                composing::the_tuple_this_build_is_made_for(),
                b"a machine program, for the sake of the argument",
                composing::THE_FIXED_ENTRY_POINT_NAME,
                &["https://example.invalid/smoozy-server".into()],
            )],
        )
    }

    /// **The tuple is read with three constant commands and nothing of
    /// Smoozy's, so it is read before any package byte exists on the machine.**
    ///
    /// **And the two spellings are held together here.** This crate writes the
    /// words a manifest's tuple carries and the crate that composes a manifest
    /// writes them too, so a machine's own answers are turned into a value and
    /// compared with what this build actually produced. **A change to either
    /// side that the other does not follow fails here** rather than in the field,
    /// where it would read as every machine being unsupported.
    #[test]
    fn a_captured_answer_becomes_the_tuple_this_build_was_made_for() {
        use smoozy_server_protocol::release::composing;
        let mut machine = a_machine_that_says("Linux\n", "x86_64\n", "glibc 2.35\n");
        let read = block_on(read_the_rentals_tuple(&mut machine));
        assert_eq!(
            read.as_read(),
            Some(&composing::the_tuple_this_build_is_made_for())
        );
        assert_eq!(
            machine.asked,
            vec!["uname -s", "uname -m", "getconf GNU_LIBC_VERSION"],
            "the tuple is read with these and nothing else"
        );
    }

    /// **`Server protocol and transfers` B8's exact match, run rather than
    /// argued: it selects, and a machine outside it is an unsupported machine.**
    #[test]
    fn an_exact_match_selects_and_another_machine_is_an_unsupported_one() {
        use smoozy_server_protocol::release::manifest::{select, WhySelectionFailed};
        let manifest = a_manifest_of_this_build();

        let mut matching = a_machine_that_says("Linux\n", "x86_64\n", "glibc 2.35\n");
        let read = block_on(read_the_rentals_tuple(&mut matching));
        let chosen = select(
            &manifest,
            read.as_read().expect("the machine said all three"),
        )
        .expect("the exact match selects");
        assert_eq!(chosen.entry_point_name, "smoozy-server");

        // **A processor no entry names.** Nothing takes the nearest entry, and
        // nothing takes the first: there is one entry and it is not a candidate.
        let mut elsewhere = a_machine_that_says("Linux\n", "aarch64\n", "glibc 2.35\n");
        let read = block_on(read_the_rentals_tuple(&mut elsewhere));
        assert_eq!(
            select(
                &manifest,
                read.as_read().expect("the machine said all three")
            ),
            Err(WhySelectionFailed::ThisMachineIsNotSupported)
        );
    }

    /// **A machine that would not answer selects nothing at all**, rather than
    /// being read as the machine this build was made for.
    ///
    /// **This is the reading that would otherwise make the exact match useless.**
    /// Every selection made before this piece landed was made against the
    /// constant naming what this build produced, so it always matched.
    #[test]
    fn a_machine_that_will_not_say_composes_no_tuple_and_selects_nothing() {
        let mut gone = AMachine {
            the_channel_is_gone: true,
            ..AMachine::default()
        };
        let read = block_on(read_the_rentals_tuple(&mut gone));
        assert_eq!(read.as_read(), None);
        assert_eq!(
            read,
            TheRentalsTuple::ItWouldNotSay(
                crate::kind::WhyNoTupleWasRead::ItWouldNotNameItsOperatingSystem
            )
        );

        // **A command that ran and refused is an absence too**, and never an
        // answer of its own.
        let mut refused = AMachine::answering(vec![
            ("uname -s", said("Linux\n", 0)),
            (
                NAME_YOUR_PROCESSOR_ARCHITECTURE.as_str(),
                complained("uname: not found", 127),
            ),
            (NAME_YOUR_C_LIBRARY.as_str(), said("glibc 2.35\n", 0)),
        ]);
        assert_eq!(
            block_on(read_the_rentals_tuple(&mut refused)),
            TheRentalsTuple::ItWouldNotSay(
                crate::kind::WhyNoTupleWasRead::ItWouldNotNameItsProcessorArchitecture
            )
        );
    }

    /// **The desk's own answers, captured and replayed, and it is refused.**
    ///
    /// **Observed on the Creator's Mac, 2026-09-07**, by running the three
    /// commands on it: `uname -s` answered `Darwin`, `uname -m` answered
    /// `arm64`, and the C library reading refused at exit 64 — *"no such
    /// configuration parameter"*. **So this desk is not a machine any manifest
    /// of this build names, and the reading says so at the first half rather
    /// than composing a tuple out of two-thirds of an answer.** It is recorded
    /// because it is the honest measure of what this desk can prove: **no rented
    /// Linux machine has ever answered either command**, and this one gives the
    /// wrong tuple by construction.
    #[test]
    fn this_desks_own_answers_compose_no_tuple() {
        let mut this_mac = AMachine::answering(vec![
            ("uname -s", said("Darwin\n", 0)),
            (
                NAME_YOUR_PROCESSOR_ARCHITECTURE.as_str(),
                said("arm64\n", 0),
            ),
            (
                NAME_YOUR_C_LIBRARY.as_str(),
                complained("getconf: no such configuration parameter", 64),
            ),
        ]);
        assert_eq!(
            block_on(read_the_rentals_tuple(&mut this_mac)),
            TheRentalsTuple::ItWouldNotSay(
                crate::kind::WhyNoTupleWasRead::TheMachineIsNotALinuxMachine
            )
        );
    }

    #[test]
    fn the_probe_that_did_not_reach_the_machine_is_a_probe_that_failed() {
        let mut machine = AMachine {
            the_channel_is_gone: true,
            ..AMachine::default()
        };
        assert_eq!(block_on(run_the_probe(&mut machine)), Probe::Failed);
    }

    #[test]
    fn the_probe_reads_its_exit_and_nothing_else() {
        let mut succeeded = AMachine::answering(vec![(THE_PROBE.as_str(), said("", 0))]);
        assert_eq!(block_on(run_the_probe(&mut succeeded)), Probe::Succeeded);
        let mut refused = AMachine::answering(vec![(
            THE_PROBE.as_str(),
            complained("sudo: a password is required", 1),
        )]);
        assert_eq!(block_on(run_the_probe(&mut refused)), Probe::Failed);
    }

    #[test]
    fn the_login_is_root_only_where_the_machine_said_zero() {
        let mut root = AMachine::answering(vec![(WHICH_LOGIN_IS_THIS.as_str(), said("0\n", 0))]);
        assert!(block_on(read_whether_the_login_is_root(&mut root)));
        let mut not = AMachine::answering(vec![(WHICH_LOGIN_IS_THIS.as_str(), said("1000\n", 0))]);
        assert!(!block_on(read_whether_the_login_is_root(&mut not)));
        let mut silent = AMachine::default();
        assert!(!block_on(read_whether_the_login_is_root(&mut silent)));
    }

    #[test]
    fn the_refusal_reading_ends_file_browsers_seam_with_the_machines_own_answers() {
        use smoozy_file_browser::permission::{
            the_offer, ElevatableAct, MachineKind as Theirs, Offer, ReadsWhatElevationWouldDo,
        };
        let mut machine = AMachine::answering(vec![
            (WHAT_SERVERS_SENDS_FOR_THE_CONTAINER_ROW[0], said("", 0)),
            (WHICH_LOGIN_IS_THIS.as_str(), said("1000", 0)),
            (THE_PROBE.as_str(), complained("a password is required", 1)),
        ]);
        let mut files = AMachine::default();
        let taken = block_on(take_the_readings_at_a_refusal(&mut machine, &mut files));
        let mut end = crate::elevation::WhatElevationWouldDo(taken);
        assert_eq!(end.the_machine_kind(), Theirs::AContainerUnderANonRootLogin);
        // **`File Browser` P5's first row, reached from the machine's own
        // answers rather than from anything composed here.**
        assert!(matches!(
            the_offer(ElevatableAct::AListing, false, &mut end),
            Offer::NoOffer(_)
        ));
    }

    #[test]
    fn the_driver_check_keeps_the_machines_own_words_and_parses_none_of_them() {
        let mut answered =
            AMachine::answering(vec![(THE_DRIVER_CHECK.as_str(), said("a card", 0))]);
        assert_eq!(
            block_on(run_the_driver_check(&mut answered)),
            WhatTheDriverCheckSaid::ItAnswered
        );
        let mut did_not = AMachine::answering(vec![(
            THE_DRIVER_CHECK.as_str(),
            complained("NVIDIA-SMI has failed", 9),
        )]);
        assert_eq!(
            block_on(run_the_driver_check(&mut did_not)),
            WhatTheDriverCheckSaid::ItDidNot {
                the_exit: Some(THE_EXIT_THAT_MEANS_A_DRIVER_IS_INSTALLED_AND_NOT_LOADED),
                the_machines_own_words: "NVIDIA-SMI has failed".to_string(),
                what_it_wrote_to_standard_output: String::new(),
            }
        );
    }

    /// **S2a and W1: a failure that wrote its words to standard output still has
    /// the machine's own words behind it.**
    ///
    /// **`Server Setup` R1 item 5's module reading: asked only where the check
    /// ended at 9, at each family's own path, and read by its exit alone.**
    /// Added 2026-09-30, eleventh-pass fix piece 10.
    #[test]
    fn the_running_kernels_module_is_asked_only_after_a_nine_and_read_by_its_exit() {
        use crate::kind::DistributionFamily;
        let nine = WhatTheDriverCheckSaid::ItDidNot {
            the_exit: Some(THE_EXIT_THAT_MEANS_A_DRIVER_IS_INSTALLED_AND_NOT_LOADED),
            the_machines_own_words: String::new(),
            what_it_wrote_to_standard_output: String::new(),
        };
        let absent = WhatTheDriverCheckSaid::ItDidNot {
            the_exit: Some(127),
            the_machines_own_words: "nvidia-smi: command not found".to_string(),
            what_it_wrote_to_standard_output: String::new(),
        };

        // **Another exit, or the driver answering: nothing is asked.**
        for check in [&absent, &WhatTheDriverCheckSaid::ItAnswered] {
            let mut machine = AMachine::answering(vec![]);
            assert_eq!(
                block_on(read_the_running_kernels_module(
                    &mut machine,
                    DistributionFamily::Ubuntu,
                    check
                )),
                TheRunningKernelsModule::IsNotThere
            );
            assert!(machine.asked.is_empty(), "a module was asked for after {check:?}");
        }

        // **Each family at its own path**, found and not found.
        for (family, command) in [
            (DistributionFamily::Ubuntu, "/usr/sbin/modprobe --dry-run --quiet nvidia"),
            (DistributionFamily::Debian, "/sbin/modprobe --dry-run --quiet nvidia"),
        ] {
            assert_eq!(the_module_reading_for(family).as_str(), command);
            let mut found = AMachine::answering(vec![(command, said("", 0))]);
            assert_eq!(
                block_on(read_the_running_kernels_module(&mut found, family, &nine)),
                TheRunningKernelsModule::IsThere
            );
            assert_eq!(found.asked, vec![command]);
            let mut not_found = AMachine::answering(vec![(command, said("", 1))]);
            assert_eq!(
                block_on(read_the_running_kernels_module(&mut not_found, family, &nine)),
                TheRunningKernelsModule::IsNotThere
            );
        }

        // **A reading that reached nothing establishes nothing, and reads as
        // not there** — the install route's direction.
        let mut gone = AMachine {
            the_channel_is_gone: true,
            ..AMachine::default()
        };
        assert_eq!(
            block_on(read_the_running_kernels_module(
                &mut gone,
                DistributionFamily::Ubuntu,
                &nine
            )),
            TheRunningKernelsModule::IsNotThere
        );
    }

    /// **This is the test the fault of 2026-09-04 did not have.** The one real
    /// driver failure this project has watched — an installed driver with its
    /// kernel modules unloaded, on a rented machine — ended at 9 and wrote its
    /// whole message to **standard output**, with nothing on the complaint
    /// stream. The reading kept the complaint stream alone, so *Could not be made
    /// ready* would have reached the user with no words behind it. **Every desk
    /// test of this reading had put the words on the stream the reading kept.**
    #[test]
    fn a_driver_failure_that_wrote_to_standard_output_still_has_words_behind_it() {
        let mut on_standard_output = AMachine::answering(vec![(
            THE_DRIVER_CHECK.as_str(),
            smoozy_server_protocol::transport::CommandResult {
                output: b"NVIDIA-SMI has failed because it couldn't communicate with the \
                          NVIDIA driver. Make sure that the latest NVIDIA driver is installed \
                          and running.\n"
                    .to_vec(),
                complaint: Vec::new(),
                status: Some(9),
            },
        )]);
        let said = block_on(run_the_driver_check(&mut on_standard_output));
        assert!(said.any_words_at_all());
        let WhatTheDriverCheckSaid::ItDidNot {
            the_exit,
            the_machines_own_words,
            what_it_wrote_to_standard_output,
        } = &said
        else {
            panic!("an exit of 9 is not the driver answering: {said:?}");
        };
        // **The streams are kept apart and neither is merged into the other.**
        assert!(the_machines_own_words.is_empty());
        assert!(what_it_wrote_to_standard_output.starts_with("NVIDIA-SMI has failed"));
        // **And the exit is what R1 item 5's other branch is decided on** —
        // this machine is exactly the one the clause's *a driver is already
        // installed* case describes.
        assert_eq!(
            *the_exit,
            Some(THE_EXIT_THAT_MEANS_A_DRIVER_IS_INSTALLED_AND_NOT_LOADED)
        );
        assert!(said.the_exit_says_a_driver_is_installed());

        // **And a check that reached nothing has no words on either stream**,
        // which the same question answers honestly rather than by silence.
        let mut gone = AMachine {
            the_channel_is_gone: true,
            ..AMachine::default()
        };
        assert!(!block_on(run_the_driver_check(&mut gone)).any_words_at_all());
    }

    /// **R1 item 3's probe, read against the status entry the package system's
    /// own manual describes.**
    ///
    /// The entry below is the shape `dpkg-query -s` is documented to print —
    /// *"the entry in the installed package status database"* — and the reading
    /// takes one field of it.
    #[test]
    fn the_probe_says_the_tool_is_there_only_where_the_status_word_is_installed() {
        let installed = |entry: &'static str| {
            let mut machine =
                AMachine::answering(vec![(THE_UNPACK_TOOLS_PACKAGE_PROBE.as_str(), said(entry, 0))]);
            block_on(probe_the_unpack_tool(&mut machine))
        };

        assert_eq!(
            installed(
                "Package: xz-utils\nStatus: install ok installed\nVersion: 5.6.1\nDescription: XZ-format compression utilities\n"
            ),
            WhetherTheUnpackToolIsPresent::ItIs
        );

        // **The seven other package states are not this one**, and the two that
        // carry the word inside a longer word are the reason the field is read
        // by whole word.
        for state in [
            "not-installed",
            "config-files",
            "half-installed",
            "unpacked",
            "half-configured",
            "triggers-awaited",
            "triggers-pending",
        ] {
            let entry = format!("Package: xz-utils\nStatus: deinstall ok {state}\n");
            let mut machine = AMachine::answering(vec![(
                THE_UNPACK_TOOLS_PACKAGE_PROBE.as_str(),
                CommandResult {
                    output: entry.into_bytes(),
                    complaint: Vec::new(),
                    status: Some(0),
                },
            )]);
            assert_eq!(
                block_on(probe_the_unpack_tool(&mut machine)),
                WhetherTheUnpackToolIsPresent::ItIsNot,
                "{state} was read as the tool being on the machine"
            );
        }

        // **A word of that spelling outside the field is not the status.**
        assert_eq!(
            installed("Package: xz-utils\nStatus: deinstall ok config-files\nDescription: it can be installed\n"),
            WhetherTheUnpackToolIsPresent::ItIsNot
        );
    }

    /// **The documented exit for a package the database has never held, and the
    /// two readings that establish nothing.**
    #[test]
    fn a_probe_that_established_nothing_is_never_read_as_the_tool_being_there() {
        // *"1 The requested query failed either fully or partially, due to no
        // file or package being found"* — the machine answered, and the answer
        // is *absent*.
        let mut never_held = AMachine::answering(vec![(
            THE_UNPACK_TOOLS_PACKAGE_PROBE.as_str(),
            complained(
                "dpkg-query: package 'xz-utils' is not installed and no information is available\n",
                1,
            ),
        )]);
        assert_eq!(
            block_on(probe_the_unpack_tool(&mut never_held)),
            WhetherTheUnpackToolIsPresent::ItIsNot
        );

        // **A clean answer with no status field in it establishes nothing**,
        // and neither does a channel that is gone.
        let mut nothing_readable = AMachine::answering(vec![(
            THE_UNPACK_TOOLS_PACKAGE_PROBE.as_str(),
            said("Package: xz-utils\n", 0),
        )]);
        assert_eq!(
            block_on(probe_the_unpack_tool(&mut nothing_readable)),
            WhetherTheUnpackToolIsPresent::TheProbeDidNotAnswer
        );
        let mut gone = AMachine {
            the_channel_is_gone: true,
            ..AMachine::default()
        };
        assert_eq!(
            block_on(probe_the_unpack_tool(&mut gone)),
            WhetherTheUnpackToolIsPresent::TheProbeDidNotAnswer
        );

        // **Neither of the two establishes the tool is there**, so the install
        // still goes out — the direction whose cost is one install nobody
        // needed rather than a run that cannot unpack what it fetched.
        for established_nothing in [
            WhetherTheUnpackToolIsPresent::TheProbeDidNotAnswer,
            WhetherTheUnpackToolIsPresent::ItIsNot,
        ] {
            assert!(!established_nothing.the_tool_is_there());
        }
        assert!(WhetherTheUnpackToolIsPresent::ItIs.the_tool_is_there());
    }

    /// **The probe names the package the install names, and the name lives in
    /// one place.**
    #[test]
    fn the_probe_asks_after_the_package_the_tooling_install_puts_on() {
        assert!(THE_UNPACK_TOOLS_PACKAGE_PROBE
            .as_str()
            .ends_with(crate::ready_action::THE_UNPACK_TOOLS_PACKAGE));
        assert!(crate::elevated::THE_TOOLING_INSTALL
            .plain
            .as_str()
            .ends_with(crate::ready_action::THE_UNPACK_TOOLS_PACKAGE));
    }
}
