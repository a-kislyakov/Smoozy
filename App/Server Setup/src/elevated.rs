//! The elevated route, performed — both of its forms.
//!
//! **E3 fixes the route and this module does not re-choose it**: the machine's
//! own elevating command, with the password on its standard input and never as
//! an argument, because a command argument is readable by any other login on the
//! machine (Fact 60) and `Server protocol and transfers` §1.3 settled the route
//! before this document was written.
//!
//! **Two forms, and both of them are performed here.**
//!
//! - **Where M3's probe succeeded, elevation is passwordless** and the step is
//!   the elevating command's own non-interactive form in front of a constant
//!   command. **No password is asked for, held, or sent**, so E4's validate-first
//!   separation is not engaged at all: there is no authentication step to
//!   separate from the operation.
//! - **Where a password is held, the route is standard input** — the elevating
//!   command's own read-from-standard-input form, with the password carried on
//!   `Server protocol and transfers` 1.1's second carriage. **The validate runs
//!   first and runs no command at all**, so its result classifies the
//!   authentication and never the operation (E4); where it passes, the step
//!   itself follows, carrying the password again because the credential cache is
//!   not relied on.
//! - **Where a password is owed and none is held**, this sends nothing. That is
//!   E4's fourth bullet — the moment the offer is made — and it is a state of
//!   the run rather than a missing route.
//!
//! **What used to stand here.** The password form had no route at all: this
//! crate's transport published no way to write a remote command's standard
//! input, so a machine whose login must be asked for a password could not be
//! elevated and every such step reached W1's *Refused*. **The route landed on
//! 2026-09-04** at
//! [`smoozy_server_protocol::transport::CarriesAValueOnACommandsStandardInput`],
//! and **no second route was opened to get here** (E6: *no password in an
//! argument or environment value ever, and no second route beside the elevating
//! command*).
//!
//! **Every command below is a constant, in all three forms**, which is E7's
//! rule: each step's values ride one of `Server protocol and transfers` 1.1's
//! carriages or are fixed at build time. **The elevated form of a command is
//! itself a constant rather than a string built at run time** — a command string
//! may hold only a fixed literal, so the pairing is a table a test checks rather
//! than a concatenation nobody can see. **The password never enters any of the
//! three**: it rides the carriage, which is the whole of why E3 fixes that
//! route.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` §1.1, §1.3.
//! `Security and licence` §4, §13. `Rented-server runtime` M7, M8, K10, K11,
//! K19.
//! `Job files and outputs` L4a.

use smoozy_security_and_licence::plaintext::Origin;
use smoozy_server_protocol::transport::{CarriesAValueOnACommandsStandardInput, CommandString};
use smoozy_servers::readings::CommandChannel;

use crate::elevation::{what_a_failed_validate_means, WhatTheStepMet, THE_VALIDATE_FORM};
use crate::kind::Probe;

/// **The elevating command's own non-interactive form**, which is what every
/// elevated command below is prefixed with.
///
/// **Documented** — `sudo(8)`, `-n, --non-interactive`: *"Avoid prompting the
/// user for input of any kind. If a password is required for the command to
/// run, sudo will display an error message and exit."* The same option is word
/// for word in the second implementation Ubuntu 25.10 and later install by
/// default, so **neither form is at risk** (M3's own record of both readings).
pub const THE_NON_INTERACTIVE_PREFIX: &str = "sudo -n ";

/// **E3's own form: the elevating command reading the password from standard
/// input.**
///
/// **Documented, and the quotation is E3's own** — `sudo(8)` 1.9.17p2, read
/// 2026-08-19: *"Write the prompt to the standard error and read the password
/// from the standard input instead of using the terminal device"*. The citation
/// is in the plan's own sources file at
/// `Preparation/Architecture/Sources/Server Setup.sources.md`. **Because a
/// command argument is readable by any other login on the machine** (Fact 60),
/// and `Server protocol and transfers` §1.3 fixed this route before this
/// document was written.
pub const THE_STANDARD_INPUT_PREFIX: &str = "sudo -S ";

/// One elevated step, in all three forms.
///
/// **All three are `&'static str` and that is the mechanism.** Neither elevated
/// form is composed from the plain one at run time, because a command string may
/// hold only a fixed literal; the three are written out and
/// [`tests::every_elevated_form_is_its_plain_form_behind_the_prefix`] is what
/// keeps them in step.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnElevatedStep {
    /// What the step does, in the clause's own terms.
    pub what_it_does: &'static str,
    /// The command as it stands without elevation.
    pub plain: CommandString,
    /// The same command behind the elevating command's non-interactive form.
    pub elevated: CommandString,
    /// **The same command behind E3's form**, which reads the password from
    /// standard input. **The password is not in it and never will be** — it
    /// rides `Server protocol and transfers` 1.1's second carriage.
    pub elevated_reading_the_password_from_standard_input: CommandString,
    /// **Y1's test, applied to the act this command performs.**
    ///
    /// **It sits beside the command rather than anywhere else, because Y1
    /// decides the class *"by the act a step performs, never by the step's
    /// number"*** — and the command is the act. A judgement kept anywhere else
    /// would be a second copy of this one, which is the fault this stage keeps
    /// paying for.
    pub what_it_does_to_the_machine: WhatAnActDoesToTheMachine,
}

/// **Y1's three acts, and a fourth word for an elevated command that is none of
/// them.**
///
/// *"A run is machine-changing exactly where it performs an elevated package
/// install, a driver act, or a restart."* **The set is closed at three and the
/// fourth word is not a third state of the run** — it says that this particular
/// command is not one of the three, and a run made only of such commands is Y1's
/// additive class.
///
/// **Which word a command carries is a judgement about that command and is
/// written at the command**, so a reader meets the reason where the string is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatAnActDoesToTheMachine {
    /// **Y1's first act.** The command reaches the machine's package system to
    /// put a package on it.
    AnElevatedPackageInstall,
    /// **Y1's second act.** The command is part of R1 item 5's driver route —
    /// M1b's per-family route, and the two commands that route needs before it
    /// can run on Debian.
    ADriverAct,
    /// **Y1's third act.** The machine goes down.
    ARestart,
    /// **None of Y1's three**, so an act of this kind never sets the
    /// machine-level state.
    NoneOfY1sThree,
}

impl WhatAnActDoesToTheMachine {
    /// **Y1's class for one act.**
    ///
    /// **This is the whole of the test and it is not widened.** An elevated
    /// command that makes a directory, hands a directory over, or takes a build
    /// off the machine is elevated and is not one of Y1's three — and Y1 spells
    /// out at length why over-setting is not the safe direction: the state holds
    /// reconciliation and every admission back, so a run that sets it for
    /// nothing holds the interrupted job back for nothing.
    pub const fn is_machine_changing(self) -> bool {
        !matches!(self, WhatAnActDoesToTheMachine::NoneOfY1sThree)
    }
}

/// **R7 and R7a's hand-over of the chosen root**, through its own planted link.
pub const THE_HAND_OVER_OF_THE_ROOT: AnElevatedStep = AnElevatedStep {
    what_it_does: "hand the chosen root over to the connection's own login",
    // **Y1's class for this act.** A change of owner on a directory Smoozy just
    // made. **Not one of Y1's three**, and it runs before any setup run exists
    // to name.
    what_it_does_to_the_machine: WhatAnActDoesToTheMachine::NoneOfY1sThree,
    plain: crate::placement::THE_RECURSIVE_HAND_OVER,
    elevated: CommandString::constant(
        "sudo -n chown -R -H \"$(id -u):$(id -g)\" ~/.smoozy-carriage/hand-over-root",
    ),
    elevated_reading_the_password_from_standard_input: CommandString::constant(
        "sudo -S chown -R -H \"$(id -u):$(id -g)\" ~/.smoozy-carriage/hand-over-root",
    ),
};

/// **R7a's second carriage, where `Job files and outputs` L4a placed the
/// package area on a mount of its own.**
///
/// **Nothing about the second mount is inferred from the first's exit or from
/// the first's landing check**, which is why it is a second command rather than
/// a second argument.
pub const THE_HAND_OVER_OF_THE_PACKAGE_AREA: AnElevatedStep = AnElevatedStep {
    what_it_does: "hand the package area's own mount over, where there is one",
    // **Y1's class for this act.** The same act on the second mount.
    what_it_does_to_the_machine: WhatAnActDoesToTheMachine::NoneOfY1sThree,
    plain: crate::placement::THE_RECURSIVE_HAND_OVER_OF_THE_PACKAGE_AREA,
    elevated: CommandString::constant(
        "sudo -n chown -R -H \"$(id -u):$(id -g)\" ~/.smoozy-carriage/hand-over-package-area",
    ),
    elevated_reading_the_password_from_standard_input: CommandString::constant(
        "sudo -S chown -R -H \"$(id -u):$(id -g)\" ~/.smoozy-carriage/hand-over-package-area",
    ),
};

/// **R7's creation of the root's own directory, where the plain create over the
/// file subsystem is refused.**
///
/// **The plain create is tried first** (R3, M2: on a container whose login
/// cannot install packages nothing elevated runs at all), and this is what the
/// refusal falls to. **It rides R7a's carriage a second time**: the link's
/// target is the chosen mount, and the command names the token and `Job files
/// and outputs`' own root segment, **which is that document's build constant and
/// not a machine-origin value** — so every byte of the string is fixed at build
/// time.
pub const THE_CREATION_OF_THE_ROOT: AnElevatedStep = AnElevatedStep {
    what_it_does: "create Smoozy's own root directory under the chosen mount, where the login may not",
    // **Y1's class for this act.** A directory is made. **Not one of Y1's
    // three**, and it runs before any setup run exists to name.
    what_it_does_to_the_machine: WhatAnActDoesToTheMachine::NoneOfY1sThree,
    plain: crate::placement::THE_ROOTS_CREATION,
    elevated: CommandString::constant("sudo -n mkdir ~/.smoozy-carriage/create-root/smoozy"),
    elevated_reading_the_password_from_standard_input: CommandString::constant(
        "sudo -S mkdir ~/.smoozy-carriage/create-root/smoozy",
    ),
};

/// **R7's creation of the package area's own location**, where `Job files and
/// outputs` L4a placed it on a second mount. **A second command rather than a
/// second argument**, because R7a's carriage is per mount.
pub const THE_CREATION_OF_THE_PACKAGE_AREAS_LOCATION: AnElevatedStep = AnElevatedStep {
    what_it_does: "create the package area's own location under its second mount, where the login may not",
    // **Y1's class for this act.** The same act on the second mount.
    what_it_does_to_the_machine: WhatAnActDoesToTheMachine::NoneOfY1sThree,
    plain: crate::placement::THE_CREATION_OF_THE_PACKAGE_AREAS_LOCATION,
    elevated: CommandString::constant("sudo -n mkdir ~/.smoozy-carriage/create-package-area/smoozy"),
    elevated_reading_the_password_from_standard_input: CommandString::constant(
        "sudo -S mkdir ~/.smoozy-carriage/create-package-area/smoozy",
    ),
};

/// **R1 item 3's tooling install** — the one unpack tool Blender's archives
/// need, a constant package name, installed where absent.
///
/// **The bounded wait rides it**, which is S7b's rule for every package step of
/// a setup run rather than apt's own default of `0`, at which the lock
/// acquisition returns immediately.
pub const THE_TOOLING_INSTALL: AnElevatedStep = AnElevatedStep {
    what_it_does: "install the one unpack tool, where absent",
    // **Y1's class for this act.** **Y1's own first example**: *"a Blender
    // install on a machine lacking the unpack tool performs an elevated package
    // install and is machine-changing"*.
    what_it_does_to_the_machine: WhatAnActDoesToTheMachine::AnElevatedPackageInstall,
    plain: CommandString::constant("apt-get -o DPkg::Lock::Timeout=120 install -y xz-utils"),
    elevated: CommandString::constant(
        "sudo -n apt-get -o DPkg::Lock::Timeout=120 install -y xz-utils",
    ),
    elevated_reading_the_password_from_standard_input: CommandString::constant(
        "sudo -S apt-get -o DPkg::Lock::Timeout=120 install -y xz-utils",
    ),
};

/// **R1 item 4's install for the mapping's one entry, and E7's rule kept by
/// construction.**
///
/// *"The machine's `ldd` list **selects, and never composes, the command**"* —
/// so the not-found list picks this constant or it picks nothing, and **every
/// byte of the string is fixed at build time**. The eleven package names are
/// [`crate::ready_action::THE_MAPPING`]'s one entry, in its own order, and
/// [`the_packages_this_step_names_are_the_entrys_own`] is what keeps the two
/// from drifting.
///
/// **`libxext6` joined the list on 2026-09-22**, with the entry's keys, out of
/// step 10c's fourth pass — the entry's own source has the reading and the
/// publication behind each name.
///
/// **What a second mapping entry would need is a second constant of its own**,
/// not a merged command: a string built from two entries is a string nobody
/// fixed at build time, which is exactly what E7 forbids. [`the_libraries_install`]
/// is where that is enforced.
pub const THE_LIBRARIES_INSTALL_FOR_THE_FIRST_ENTRY: AnElevatedStep = AnElevatedStep {
    what_it_does: "install the packages the mapping's first entry names, all of them at once",
    // **Y1's class for this act.** **Y1 names this one too**: *"A run whose
    // libraries step must install anything is machine-changing by the same
    // test."*
    what_it_does_to_the_machine: WhatAnActDoesToTheMachine::AnElevatedPackageInstall,
    plain: CommandString::constant(
        "apt-get -o DPkg::Lock::Timeout=120 install -y libx11-6 libxi6 libxxf86vm1 libxfixes3 \
         libxrender1 libgl1 libegl1 libsm6 libice6 libxkbcommon0 libxext6",
    ),
    elevated: CommandString::constant(
        "sudo -n apt-get -o DPkg::Lock::Timeout=120 install -y libx11-6 libxi6 libxxf86vm1 \
         libxfixes3 libxrender1 libgl1 libegl1 libsm6 libice6 libxkbcommon0 libxext6",
    ),
    elevated_reading_the_password_from_standard_input: CommandString::constant(
        "sudo -S apt-get -o DPkg::Lock::Timeout=120 install -y libx11-6 libxi6 libxxf86vm1 \
         libxfixes3 libxrender1 libgl1 libegl1 libsm6 libice6 libxkbcommon0 libxext6",
    ),
};

/// **M1b's Ubuntu driver route, its first act: the detection tool made
/// reachable.** R1 item 5 as corrected on 2026-09-21: `ubuntu-drivers` is in
/// the package `ubuntu-drivers-common` — Ubuntu's own package index,
/// `https://packages.ubuntu.com/noble/amd64/ubuntu-drivers-common/filelist`
/// and the same for `jammy`, read 2026-09-21, both list
/// `/usr/bin/ubuntu-drivers` — and **a plain Ubuntu image need not carry it**:
/// Hyperstack's plain 24.04 answered *"sudo: ubuntu-drivers: command not
/// found"* on the second pass of step 10c. **S7b's bounded wait rides it** like
/// every other package step, and on an image that already has the tool apt
/// installs nothing.
pub const THE_UBUNTU_DRIVER_TOOL_INSTALL: AnElevatedStep = AnElevatedStep {
    what_it_does: "install the distribution's own driver detection tool",
    // **Y1's class for this act.** R1 item 5's Ubuntu route, issued only as
    // part of it, the same way M1b's Debian components step is a driver act.
    what_it_does_to_the_machine: WhatAnActDoesToTheMachine::ADriverAct,
    plain: CommandString::constant(
        "apt-get -o DPkg::Lock::Timeout=120 install -y ubuntu-drivers-common",
    ),
    elevated: CommandString::constant(
        "sudo -n apt-get -o DPkg::Lock::Timeout=120 install -y ubuntu-drivers-common",
    ),
    elevated_reading_the_password_from_standard_input: CommandString::constant(
        "sudo -S apt-get -o DPkg::Lock::Timeout=120 install -y ubuntu-drivers-common",
    ),
};

/// **M1b's Ubuntu driver route** — Canonical's own documented and recommended
/// command, which *"will install the driver that is considered the best match
/// for your hardware"*. **Naming a flavour here is forbidden.**
///
/// **This is the one package command in the crate with no bounded wait, and the
/// gap is recorded rather than guessed at.** Every other one carries
/// `-o DPkg::Lock::Timeout=120`, the Debian driver install included, which is
/// S7b's rule for every package step of a setup run. **Whether this tool accepts
/// an apt option, or passes one to the apt it drives, is established nowhere in
/// this project**, and writing one in from memory is what Rule 2 forbids — so
/// none is written.
///
/// **What breaks while it is unwritten**: on Ubuntu the driver install has no
/// bounded wait, so S7b's busy-before-broken rule is defeated for that one step
/// — **a machine two minutes into its uptime, running its own automatic updates,
/// fails the step at once for a reason that is not a fault.** Ubuntu is the
/// family whose own documentation S7b quotes for the busy case, so the family
/// where the collision is documented is the family whose driver install is
/// unprotected against it.
///
/// **What closes it** is one reading of that tool's own publication, which this
/// module's round-1 code audit parks at
/// `Preparation/Groundwork/Open Threads.md`. **Nothing else waits on it.**
pub const THE_UBUNTU_DRIVER_INSTALL: AnElevatedStep = AnElevatedStep {
    what_it_does: "install the driver through the distribution's own detection route",
    // **Y1's class for this act.** R1 item 5's Ubuntu route.
    what_it_does_to_the_machine: WhatAnActDoesToTheMachine::ADriverAct,
    plain: CommandString::constant("ubuntu-drivers install --gpgpu"),
    elevated: CommandString::constant("sudo -n ubuntu-drivers install --gpgpu"),
    elevated_reading_the_password_from_standard_input: CommandString::constant("sudo -S ubuntu-drivers install --gpgpu"),
};

/// **M1b's Ubuntu driver route, its last act: the driver support binaries the
/// check is run from.** Added 2026-09-23, out of step 10c's fifth pass on
/// Hyperstack.
///
/// **What the pass found.** `ubuntu-drivers install --gpgpu` ran to its end,
/// thirteen packages landed, the kernel module loaded, `/dev/nvidia0` appeared
/// and the card answered — **and `nvidia-smi` was on the machine nowhere**,
/// which is the one thing R1 item 5 checks a driver with and the one thing
/// `Servers` H11's driver column reads the cards with. `apt-cache policy
/// nvidia-utils-595-server` read *Installed: (none)*.
///
/// **The publication says so itself, and this act is its own instruction.**
/// Ubuntu Server's *NVIDIA drivers installation*,
/// `https://documentation.ubuntu.com/server/how-to/graphics/install-nvidia-drivers/`,
/// read **2026-09-23**, under *Installing the drivers on servers and/or for
/// computing purposes*, prints `sudo ubuntu-drivers install --gpgpu` and then,
/// immediately after it: *"You will also want to install the following
/// additional components:"* — `sudo apt install nvidia-utils-535-server`. **So
/// the gpgpu route is documented as incomplete without this second install**,
/// and nothing here is invented: the route is the publication's, in the
/// publication's order.
///
/// **And the binary is in no other package.** Ubuntu's own file index,
/// `https://packages.ubuntu.com/search?searchon=contents&mode=exactfilename&suite=noble&arch=amd64&keywords=nvidia-smi`,
/// read **2026-09-23**: *"Found 1 results"*, `/usr/bin/nvidia-smi`, published by
/// `nvidia-utils-470`, `nvidia-utils-470-server`, `nvidia-utils-535`,
/// `nvidia-utils-535-server`, `nvidia-utils-550` and `nvidia-utils-550-server`
/// — **the branch-numbered utilities packages and nothing else**, with no
/// unversioned or virtual package among them and no `-open` variant of any of
/// them.
///
/// **Why the branch is derived on the machine and never by Smoozy.** The
/// publication's command carries a branch number, and which branch a machine
/// gets is the detection route's own choice — R1 item 5 forbids this document
/// naming a flavour, and a build-time list of branch numbers would be twenty-two
/// flavours named here that go stale the day Canonical ships the next one
/// (`nvidia-utils-610` is already in `noble-updates`, read 2026-09-23). **So the
/// command string is constant entire and the branch never crosses Smoozy at
/// all**: the branch of the driver package the install has just left is read
/// on the machine, inside the command, and becomes an argument there. That is
/// E7's fifth carriage (`Server Setup` E7; `Server protocol and transfers`
/// §1.1's fifth route), whose bound is that **the tool read is named in the
/// constant string** — here `dpkg-query` — and that **its own publication is
/// quoted for what it prints**, which is below.
///
/// **What is read, and why that and not the recommended list. Changed
/// 2026-09-30, `Server Setup` R1 item 5 as corrected that day, from step
/// 10c's eleventh pass.** This act used to ask `ubuntu-drivers list --gpgpu
/// --recommended` which driver the machine should have. **On machine 1 of that
/// pass the install chose 580 and the list said 595**, and each act's package
/// removed the other's kernel module: the restart came back with no module for
/// any driver. **The tool's own source says why the two differ**
/// (`ubuntu-drivers` at
/// `https://git.launchpad.net/ubuntu/+source/ubuntu-drivers-common/plain/ubuntu-drivers?h=ubuntu/noble-updates`,
/// read **2026-09-30**): `install_gpgpu` walks the branches in the detection
/// routine's order and takes the first whose driver package is not already
/// there — *"if candidate and not cache[candidate].current_ver:
/// to_install.append(p)"* — while `list` prints the first branch whatever is
/// installed, *"print('%s %s' % (package, linux_modules))"* then *"break"*. **So
/// on a machine already holding the first branch's package, the install
/// installs the next branch down and the list still names the first.** The
/// act now reads what the install left instead of what the list recommends.
///
/// **Which package the install leaves.** The same function's candidate is
/// the driver's headless metapackage — `UbuntuDrivers/detect.py`,
/// `_get_headless_no_dkms_metapackage` (same repository and branch, read
/// **2026-09-30**): *"candidate = 'nvidia-headless-no-dkms-%s' %
/// (candidate_flavour)"* — and the install sends it to `apt-get` with the
/// kernel modules package beside it. **Observed**: every pass that watched the
/// install saw `nvidia-headless-no-dkms-595-server-open` placed (step 10c's
/// fifth, seventh, ninth and tenth passes).
///
/// **How it is read.** `dpkg-query(1)` (dpkg 1.22.6ubuntu6.6, Ubuntu noble,
/// `https://manpages.ubuntu.com/manpages/noble/en/man1/dpkg-query.1.html`,
/// read **2026-09-30**): `-W, --show` *"will list all packages matching the
/// given patterns. However the output can be customized using the
/// --showformat option"*; *"Normal shell wildcard characters are allowed in
/// package-name-pattern. Please note you will probably have to quote
/// package-name-pattern to prevent the shell from performing filename
/// expansion"* — so the pattern is double-quoted; `db:Status-Abbrev` *"contains
/// the abbreviated package status (as three characters), such as “ii ”"*, whose
/// second letter is the package status, *"i = Installed"*, beside *"c =
/// Config-files"* for a package removed with its configuration left. **So
/// `grep "^.i"` keeps only a package that stands** — POSIX `grep`
/// (`https://pubs.opengroup.org/onlinepubs/9699919799/utilities/grep.html`,
/// read 2026-09-30): *"an input line shall be selected if any pattern … matches
/// any part of the line"*, *"each selected input line shall be written to the
/// standard output"* — and the removed branch machine 1 carried at `rc` is not
/// read. In the format, `\$` is the shell's own escape inside double quotes,
/// so `dpkg-query` receives `${db:Status-Abbrev}${Package}\n`.
///
/// **The whole of the derivation.** The first standing line is taken, for
/// example `ii nvidia-headless-no-dkms-580-server-open`; everything up to and
/// including the last `-` before the branch number is dropped, which drops the
/// status letters with it; a trailing `-open` is dropped, because the open
/// kernel flavour shares the proprietary flavour's utilities package and
/// Ubuntu publishes no `-open` one. That gives `580-server`, and
/// `nvidia-utils-580-server` is a package the index above lists.
///
/// **Unresolved, with what breaks: which line is first where two branches'
/// packages both stand.** No publication read here orders `dpkg-query -W`'s
/// output, and the package pages carry no conflict field to show two branches
/// cannot stand together. **What holds**: on machine 1 each branch's install
/// removed the other's module package (observed, `/var/log/apt/history.log`
/// there), so one branch standing is what was watched. **What breaks** if two
/// ever stand: the utilities may follow the other one, which is the fault this
/// act was changed for, and the check after it fails rather than a wrong
/// machine being called ready.
///
/// **Where nothing is read, the step fails honestly.** The substitution is
/// empty, apt is asked for `nvidia-utils-` and answers in its own words that
/// there is no such package, and the caller ends the run with those words
/// behind it. **Nothing partial is installed and no restart goes out** — the
/// same rule the tool install above keeps, and for the same reason: a restart
/// cannot put a missing package on a machine. *Not established since
/// 2026-11-03: `apt-get(8)` reads a name ending in a hyphen as a removal, and
/// the Sesterce pass's route read done with no driver package standing. Which
/// act printed its *0 upgraded* is not established.
/// [`crate::acts::carry_out_the_driver_step`]'s reading after the route is what
/// holds, whichever it was.*
///
/// **S7b's bounded wait rides it** like every other apt install of a setup run.
pub const THE_UBUNTU_DRIVER_SUPPORT_BINARIES_INSTALL: AnElevatedStep = AnElevatedStep {
    what_it_does: "install the driver support binaries the driver check is run from",
    // **Y1's class for this act.** R1 item 5's Ubuntu route, issued only as
    // part of it, exactly like the tool install and the driver install beside
    // it.
    what_it_does_to_the_machine: WhatAnActDoesToTheMachine::ADriverAct,
    plain: CommandString::constant(
        "sh -c 'apt-get -o DPkg::Lock::Timeout=120 install -y \
         \"nvidia-utils-$(dpkg-query -W -f \"\\${db:Status-Abbrev}\\${Package}\\n\" \
         \"nvidia-headless-no-dkms-*\" | grep \"^.i\" | head -n 1 | \
         sed -e \"s/^.*-\\([0-9][0-9]*\\)/\\1/\" -e \"s/-open//\")\"'",
    ),
    elevated: CommandString::constant(
        "sudo -n sh -c 'apt-get -o DPkg::Lock::Timeout=120 install -y \
         \"nvidia-utils-$(dpkg-query -W -f \"\\${db:Status-Abbrev}\\${Package}\\n\" \
         \"nvidia-headless-no-dkms-*\" | grep \"^.i\" | head -n 1 | \
         sed -e \"s/^.*-\\([0-9][0-9]*\\)/\\1/\" -e \"s/-open//\")\"'",
    ),
    elevated_reading_the_password_from_standard_input: CommandString::constant(
        "sudo -S sh -c 'apt-get -o DPkg::Lock::Timeout=120 install -y \
         \"nvidia-utils-$(dpkg-query -W -f \"\\${db:Status-Abbrev}\\${Package}\\n\" \
         \"nvidia-headless-no-dkms-*\" | grep \"^.i\" | head -n 1 | \
         sed -e \"s/^.*-\\([0-9][0-9]*\\)/\\1/\" -e \"s/-open//\")\"'",
    ),
};

/// **R1 item 5's Ubuntu route, its fifth act: the OptiX library put on the
/// machine where the driver left it out.** Added 2026-10-03, with the Creator's
/// ruling that NVIDIA cards render with OptiX (`Rendering` L23).
///
/// **Why.** Fact 47 saw `ubuntu-drivers install --gpgpu` on a Hyperstack image
/// leave `libnvoptix` out as well as `nvidia-smi`. With `OPTIX` named, every
/// cards job on such a machine ends before the render instruction
/// (`Rendering` L24b) — and that route is this one, so without this act the
/// refusal would meet every Ubuntu machine whose driver Smoozy installed.
///
/// **First the library is looked for, by the same lookup `Rendering` L24b
/// makes** — `smoozy_server_setup_machine::the_optix_library::THE_LOADERS_OWN_LOOKUP`,
/// carried here whole and held to that text by this module's tests, with the
/// loader's own publication quoted beside it. **Only where it is not found is
/// anything installed**: the shell's OR list, POSIX, *"If its exit status is
/// non-zero, command2 shall be executed"* and *"The exit status of an OR list
/// shall be the exit status of the last command that is executed in the
/// list"* (`https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html`,
/// read **2026-10-03**). So a machine that has the library answers `0` and is
/// not touched, and a refused install answers in apt's own words.
///
/// **Which package carries the file, from Ubuntu's own file index searched for
/// that exact file name**:
/// `https://packages.ubuntu.com/search?searchon=contents&keywords=libnvoptix.so.1&mode=exactfilename&suite=noble&arch=amd64`,
/// read **2026-10-03**: *"Found 1 results"*, `/usr/lib/x86_64-linux-gnu/libnvoptix.so.1`,
/// published by `libnvidia-gl-470`, `libnvidia-gl-470-server`,
/// `libnvidia-gl-535`, `libnvidia-gl-535-server`, `libnvidia-gl-550` and
/// `libnvidia-gl-550-server` — **the branch-numbered `libnvidia-gl-…` packages
/// and nothing else**, the same shape the fourth act's index search gave for
/// `nvidia-smi`, with no unversioned package and no `-open` variant. The same
/// search over `jammy` (22.04), read the same day, answers the same way:
/// `libnvidia-gl-418-server` up to `libnvidia-gl-510-server`. **And the
/// branches Ubuntu ships since carry it the same way**: the file list of
/// `libnvidia-gl-580-server` in `noble-updates`,
/// `https://packages.ubuntu.com/noble-updates/amd64/libnvidia-gl-580-server/filelist`,
/// read **2026-10-03**, lists `/usr/lib/x86_64-linux-gnu/libnvoptix.so.1`;
/// `https://packages.ubuntu.com/noble-updates/libnvidia-gl-595-server` names
/// *"Package: libnvidia-gl-595-server (595.91.07-0ubuntu0.24.04.1)"*; and
/// `https://packages.ubuntu.com/noble-updates/libnvidia-gl-580-server-open`
/// answers *"No such package."*
///
/// **The branch is read on the machine exactly as the fourth act reads it**,
/// from the driver package the install left, under E7's fifth carriage — the
/// same `dpkg-query`, the same selection and the same derivation, quoted at
/// [`THE_UBUNTU_DRIVER_SUPPORT_BINARIES_INSTALL`] and held identical to it by
/// this module's tests. `580-server` becomes `libnvidia-gl-580-server`. **Never
/// a fixed list of branch numbers**, which R1 item 5 forbids.
///
/// **Where no branch is read**, apt is asked for `libnvidia-gl-` and answers in
/// its own words that there is no such package, and the step ends there — the
/// fourth act's rule. **Where the library is still not found after the
/// install, the step does not end the setup**: the driver answers and a
/// processor job renders, and `Rendering` L24b says so in plain words when a
/// cards job reaches the machine. **Debian is not touched** (§13 item 27).
///
/// **S7b's bounded wait rides it**, and it is not refreshed for the fourth
/// act's two reasons: it is issued in the same step directly after the acts
/// whose refresh it runs against, and its read and its install are one
/// command.
pub const THE_UBUNTU_OPTIX_LIBRARY_INSTALL: AnElevatedStep = AnElevatedStep {
    what_it_does: "install the OptiX library where the driver left it out",
    // **Y1's class for this act.** R1 item 5's Ubuntu route, issued only as
    // part of it, like the four acts before it.
    what_it_does_to_the_machine: WhatAnActDoesToTheMachine::ADriverAct,
    plain: CommandString::constant(
        "sh -c 'LD_TRACE_LOADED_OBJECTS=1 LD_PRELOAD=libnvoptix.so.1 sh 2>/dev/null \
         | grep -F -q \"libnvoptix.so.1 => /\" || \
         apt-get -o DPkg::Lock::Timeout=120 install -y \
         \"libnvidia-gl-$(dpkg-query -W -f \"\\${db:Status-Abbrev}\\${Package}\\n\" \
         \"nvidia-headless-no-dkms-*\" | grep \"^.i\" | head -n 1 | \
         sed -e \"s/^.*-\\([0-9][0-9]*\\)/\\1/\" -e \"s/-open//\")\"'",
    ),
    elevated: CommandString::constant(
        "sudo -n sh -c 'LD_TRACE_LOADED_OBJECTS=1 LD_PRELOAD=libnvoptix.so.1 sh 2>/dev/null \
         | grep -F -q \"libnvoptix.so.1 => /\" || \
         apt-get -o DPkg::Lock::Timeout=120 install -y \
         \"libnvidia-gl-$(dpkg-query -W -f \"\\${db:Status-Abbrev}\\${Package}\\n\" \
         \"nvidia-headless-no-dkms-*\" | grep \"^.i\" | head -n 1 | \
         sed -e \"s/^.*-\\([0-9][0-9]*\\)/\\1/\" -e \"s/-open//\")\"'",
    ),
    elevated_reading_the_password_from_standard_input: CommandString::constant(
        "sudo -S sh -c 'LD_TRACE_LOADED_OBJECTS=1 LD_PRELOAD=libnvoptix.so.1 sh 2>/dev/null \
         | grep -F -q \"libnvoptix.so.1 => /\" || \
         apt-get -o DPkg::Lock::Timeout=120 install -y \
         \"libnvidia-gl-$(dpkg-query -W -f \"\\${db:Status-Abbrev}\\${Package}\\n\" \
         \"nvidia-headless-no-dkms-*\" | grep \"^.i\" | head -n 1 | \
         sed -e \"s/^.*-\\([0-9][0-9]*\\)/\\1/\" -e \"s/-open//\")\"'",
    ),
};

/// **R1 item 5's swap: a driver that answers below the OptiX bar of the
/// Blender the run installs gives way to Ubuntu's own 580 server driver.**
/// Added 2026-10-11, out of the Creator's rulings on OptiX and the driver;
/// designed in the third piece's account of that work (`Audits/Build/Step
/// 10c/`) and built in its second attempt.
///
/// **What the swap and the finishing of a stopped removal share: the trials,
/// the removal and the install**, once. `$d`, `$f` and `$b` are the act's own
/// readings. `Server Setup` R1 item 5, written 2026-11-03.
macro_rules! the_swaps_trials_removal_and_install {
    () => {
        "apt-get -o DPkg::Lock::Timeout=900 update && \
         apt-get -o DPkg::Lock::Timeout=900 -s install \"$d\" \"linux-headers-$(uname -r)\" $f && \
         for o in $b; do apt -o DPkg::Lock::Timeout=900 -s --purge remove -y \"*nvidia*$o*\" || exit $?; done && \
         for o in $b; do apt -o DPkg::Lock::Timeout=900 -o Dpkg::Options::=--force-overwrite --purge remove -y \"*nvidia*$o*\" || exit $?; done && \
         apt-get -o DPkg::Lock::Timeout=900 -o Dpkg::Options::=--force-overwrite install -y \
         --no-install-recommends \"$d\" \"linux-headers-$(uname -r)\" $f"
    };
}

/// **The text, once**, so its three forms cannot drift: [`THE_UBUNTU_DRIVER_SWAP`]
/// puts each prefix in front of it.
macro_rules! the_ubuntu_driver_swap {
    () => {
        concat!(
            "sh -c 'b=\"$(nvidia-smi --query-gpu=driver_version --format=csv,noheader | head -n 1 | cut -d . -f 1)\" && \
             case \"$b\" in [0-9][0-9][0-9]) ;; *) exit 3 ;; esac && \
             if grep -q \"Open Kernel Module\" /proc/driver/nvidia/version; then d=nvidia-driver-580-server-open; \
             elif grep -q \"Kernel Module\" /proc/driver/nvidia/version; then d=nvidia-driver-580-server; \
             else exit 4; fi && \
             f=\"\" && \
             if dpkg-query -W -f \"\\${db:Status-Abbrev}\\n\" \"nvidia-fabricmanager*\" 2>/dev/null | grep -q \"^.i\"; \
             then f=\"nvidia-fabricmanager-580 libnvidia-nscq-580\"; fi && ",
            the_swaps_trials_removal_and_install!(),
            "'"
        )
    };
}

/// **The kernel half of a driver left behind, as names**: `Server Setup` R1 item
/// 5's reading of 2026-11-03, once, because the reading and the act that
/// finishes a removal must mean the same thing. `db:Status-Abbrev` is three
/// characters, so `cut -c4-` leaves the name; its second letter *n* or *c*
/// is a package gone or gone with its settings left (`dpkg-query(1)`, quoted at
/// [`THE_UBUNTU_DRIVER_SUPPORT_BINARIES_INSTALL`]).
macro_rules! the_kernel_half_left_behind {
    () => {
        "dpkg-query -W -f \"\\${db:Status-Abbrev}\\${Package}\\n\" \"nvidia-dkms-[0-9]*\" \"nvidia-kernel-source-[0-9]*\" 2>/dev/null | grep -v \"^.[nc]\" | cut -c4-"
    };
}

/// **`Server Setup` R1 item 5's reading of what the package system holds of a
/// driver**, unelevated, where the check failed and no restart is owed. Added
/// 2026-11-03, out of the Sesterce live pass. Prints *a driver stands* where a
/// driver package is installed, and otherwise the names of the kernel half left
/// behind, which may be none. Read by
/// [`crate::readings::WhatTheDriverPackagesSay`].
pub const THE_DRIVER_PACKAGES_READING: CommandString = CommandString::constant(concat!(
    "sh -c 'if dpkg-query -W -f \"\\${db:Status-Abbrev}\\n\" \"nvidia-driver-[0-9]*\" \"nvidia-headless-no-dkms-[0-9]*\" 2>/dev/null | grep -q \"^.i\"; \
     then echo \"a driver stands\"; else ",
    the_kernel_half_left_behind!(),
    "; fi'"
));

/// **The text of the finishing act, once.** The branches and the flavour come
/// from the package names, because `nvidia-smi` and the loaded module may both
/// have gone. `$b` holds only three-digit words, by the `sed` that makes it.
macro_rules! the_ubuntu_driver_swaps_finish {
    () => {
        concat!(
            "sh -c 'b=\"$(",
            the_kernel_half_left_behind!(),
            " | sed -n \"s/^nvidia-[a-z-]*-\\([0-9][0-9][0-9]\\).*$/\\1/p\" | sort -u)\" && \
             [ -n \"$b\" ] || exit 3; \
             if ",
            the_kernel_half_left_behind!(),
            " | grep -q -e \"-open$\"; then d=nvidia-driver-580-server-open; else d=nvidia-driver-580-server; fi && \
             f=\"\" && \
             if dpkg-query -W -f \"\\${db:Status-Abbrev}\\n\" \"nvidia-fabricmanager*\" 2>/dev/null | grep -q \"^.i\"; \
             then f=\"nvidia-fabricmanager-580 libnvidia-nscq-580\"; fi && ",
            the_swaps_trials_removal_and_install!(),
            "'"
        )
    };
}

/// **`Server Setup` R1 item 5: a removal that stopped part-way, finished.** Added
/// 2026-11-03, out of the Sesterce live pass, where the swap's removal stopped on
/// a file two packages claim, and *Set up again* then took the install route over
/// a machine it could not mend.
///
/// **It is the swap with its first two readings replaced**, so a machine that no
/// longer answers `nvidia-smi` can be finished: the branches and the flavour are
/// read from the kernel half left behind, and the fabric manager as the swap
/// reads it. **Observed**, by hand, on that half-removed machine: the swap's own
/// removal with the overwrite option ended at exit 0, the install at exit 0, and
/// after a restart the driver answered 580.178.04.
///
/// **Where it fails, the step ends in its words and no restart goes out**:
/// [`crate::acts::HowTheDriverStepWent::what_went_wrong`], as for the swap.
pub const THE_UBUNTU_DRIVER_SWAPS_FINISH: AnElevatedStep = AnElevatedStep {
    what_it_does: "finish removing a driver a removal left part-way and install Ubuntu's own newer server driver",
    // **Y1's class for this act.** R1 item 5's driver route.
    what_it_does_to_the_machine: WhatAnActDoesToTheMachine::ADriverAct,
    plain: CommandString::constant(the_ubuntu_driver_swaps_finish!()),
    elevated: CommandString::constant(concat!("sudo -n ", the_ubuntu_driver_swaps_finish!())),
    elevated_reading_the_password_from_standard_input: CommandString::constant(concat!(
        "sudo -S ",
        the_ubuntu_driver_swaps_finish!()
    )),
};

/// **R1 item 5's swap, performed as one elevated act.** Added 2026-10-11.
///
/// **Where it is issued.** On Ubuntu, where the check answered and the driver's
/// version is below the bar of the Blender this run installs
/// ([`crate::ready_action::WhatTheDriverStepDoes::TheSwapUpToTheBar`]). Never
/// on a container, never on Debian, never where the driver does not answer.
///
/// **What it does, in order, and each part ends the act if it fails**, with
/// nothing after it run:
///
/// 1. **The old branch is read** from the driver's own version query — the
///    query `Servers` H11b reads the version with (**observed** on every
///    machine of the OptiX check) — its first line up to the first dot. **It
///    goes no further unless that is exactly three digits**, so nothing but a
///    branch number ever reaches the removal's pattern.
/// 2. **The flavour the machine already runs is read, and kept.** NVIDIA's own
///    README for 580.105.08,
///    `https://download.nvidia.com/XFree86/Linux-x86_64/580.105.08/README/kernel_open.html`,
///    read **2026-10-11**: the loaded flavour shows in
///    `/proc/driver/nvidia/version` — *"NVRM version: NVIDIA UNIX Open Kernel
///    Module for x86_64"* for the open modules and *"NVRM version: NVIDIA UNIX
///    x86_64 Kernel Module"* for the proprietary ones. **R1 item 5 forbids a
///    fixed flavour, because neither serves every card** (*"The open kernel
///    modules cannot support GPUs before Turing"*; *"Blackwell and later are
///    only supported by the open kernel modules"*, same README). **A card
///    whose driver answers already runs on the flavour that is read, so the
///    same flavour of the newer branch serves it**: the proprietary flavour
///    covers *"Maxwell, Pascal, Volta, Turing, and later GPUs until
///    Blackwell"*. Neither reading → no swap.
/// 3. **The fabric manager, where one stands.** The runner's ruling of
///    2026-10-11: an NVSwitch machine is never left without its fabric
///    manager. Where any `nvidia-fabricmanager…` package stands (read the way
///    the support binaries read a standing package: `dpkg-query`'s status, its
///    second letter *i*), the 580 branch's own goes on with the driver —
///    `nvidia-fabricmanager-580` and `libnvidia-nscq-580`, the pair Ubuntu
///    Server's *NVIDIA drivers installation* page names for a machine with
///    NVSwitch (`https://ubuntu.com/server/docs/how-to/graphics/install-nvidia-drivers/`,
///    read 2026-10-11: *"If your system comes with NVswitch hardware, then
///    you will want to install Fabric Manager"*, `nvidia-fabricmanager-535
///    libnvidia-nscq-535`, the fabric manager matching the `-server` driver).
///    **Both are in Ubuntu's own archive at the driver's own version on both
///    releases**, from packages.ubuntu.com read 2026-10-11:
///    `nvidia-fabricmanager-580` and `libnvidia-nscq-580` at
///    `580.178.04-0ubuntu0.22.04.1` (jammy-updates, jammy security) and
///    `580.178.04-0ubuntu0.24.04.1` (noble-updates, noble security), both in
///    *multiverse*; the fabric manager's one dependency is
///    `nvidia-kernel-common-580-server-580.178.04`, which both driver
///    flavours bring. **Where they cannot be had** — the archive has no such
///    package for the release, or *multiverse* is not enabled on the machine —
///    the trial run below fails and **nothing is removed**: the swap misses the
///    bar and the user is told.
/// 4. **The package lists are refreshed**, inside this act rather than by
///    [`issue`]'s own refresh, because that one waits 120 seconds for the
///    package lock and this act waits 900 (point 7).
/// 5. **A trial run of the whole install, and from 2026-11-03 of the removal,
///    comes before anything is removed**:
///    apt-get(8), apt 2.8.3, `https://manpages.ubuntu.com/manpages/noble/en/man8/apt-get.8.html`,
///    read 2026-10-11: `-s` *"No action; perform a simulation of events that
///    would occur based on the current system state but do not actually
///    change the system"*; and *"apt-get returns zero on normal operation,
///    decimal 100 on error"*. **So a machine whose running kernel has no
///    headers in the archive, or where a package is missing, keeps its old
///    driver untouched.** **Assumption, labelled**: the headers package is
///    `linux-headers-` followed by the running kernel's release. **The removal's
///    trial catches what apt's resolver refuses and cannot catch dpkg's refusal
///    of a file**: the same page, *"Simulated runs print out a series of lines,
///    each representing a dpkg operation"*, so dpkg never meets the file. Read
///    2026-11-03. **Assumption, labelled**: `apt` takes `-s`; where it does not,
///    the trial fails and nothing is removed.
/// 6. **The old branch is removed by Ubuntu's own command**, `apt --purge
///    remove '*nvidia*${DRIVER_BRANCH}*'` (Ubuntu Server, *NVIDIA drivers
///    installation*, *Uninstalling the NVIDIA drivers*, the address above,
///    read 2026-10-11), with `-y` and the lock wait. **`apt` taking `-y` and
///    two `-o` was an assumption and is observed**, the Sesterce pass's hand
///    repair of 2026-11-02. **From 2026-11-03 it carries
///    `-o Dpkg::Options::=--force-overwrite`, as point 7 does**: on an image
///    with NVIDIA's own archive, apt pulled that archive's `libnvidia-cfg1` in
///    during the removal, dpkg refused a file `libnvidia-cfg1-570` also claims,
///    and the act stopped with the driver half gone (`Server Setup` R1 item 5,
///    *The removal on an image that carries NVIDIA's own package archive*).
/// 7. **The new driver goes on**: `nvidia-driver-580-server-open` or
///    `nvidia-driver-580-server`, the flavour read in point 2, with the
///    running kernel's headers for DKMS (both depend on
///    `nvidia-dkms-580-server…`) and the fabric manager where point 3 set it.
///    **Ubuntu's own archive, at a version that meets every bar** (5.3's row,
///    `Rendering` L23a, is 580):
///    `580.178.04-0ubuntu0.22.04.1` and `580.178.04-0ubuntu0.24.04.1`
///    (packages.ubuntu.com, read 2026-10-11, for both flavours). Both depend on
///    `nvidia-utils-580-server` (the check's `nvidia-smi`) and
///    `libnvidia-gl-580-server` (OptiX's `libnvoptix.so.1`, R1 item 5's fifth
///    act's own index reading), so neither later act is needed. **Why `-server`
///    names**: the image the OptiX check met carries NVIDIA's own archive, the
///    package conflicts came from packages out of it, and that archive
///    publishes no `-server` names (the third piece's account). `--no-install-recommends`
///    (*"Do not consider recommended packages as a dependency for
///    installing"*, apt-get(8)) keeps recommended packages from that archive
///    out; `-o Dpkg::Options::=--force-overwrite` answers the observed failure,
///    one package's file claimed by another — dpkg(1), noble, *"Overwrite one
///    package's file with another's file"* (the third piece's account).
///
/// **Every package command waits up to 900 seconds for the package lock**:
/// on Hyperstack's R550 image the image's own automatic upgrades held it about
/// ten minutes (**observed**, `Making OptiX work — what we found.md`).
///
/// **Its two readings ride E7's fifth carriage**, as R1 item 5's fourth and
/// fifth acts' do: the branch and the kernel's release are read on the
/// machine, inside the command, and quoted as one word. **Nothing crosses from
/// Smoozy.**
///
/// **A refused swap is never what ends the run** while the driver still
/// answers: R1 item 5 says the run carries on, and the user is told the bar
/// was missed. **A swap that removed the old driver and could not put the new
/// one on** leaves no driver answering, and that is the item's existing
/// failure, *Could not be made ready*, in this act's own words.
pub const THE_UBUNTU_DRIVER_SWAP: AnElevatedStep = AnElevatedStep {
    what_it_does: "replace a driver below the OptiX bar with Ubuntu's own newer server driver",
    // **Y1's class for this act.** R1 item 5's driver route: it removes and
    // installs a driver.
    what_it_does_to_the_machine: WhatAnActDoesToTheMachine::ADriverAct,
    plain: CommandString::constant(the_ubuntu_driver_swap!()),
    elevated: CommandString::constant(concat!("sudo -n ", the_ubuntu_driver_swap!())),
    elevated_reading_the_password_from_standard_input: CommandString::constant(concat!(
        "sudo -S ",
        the_ubuntu_driver_swap!()
    )),
};

/// **M1b's Debian driver install**, the package that family's own detection
/// tool names.
pub const THE_DEBIAN_DRIVER_INSTALL: AnElevatedStep = AnElevatedStep {
    what_it_does: "install the package Debian's own detection tool named",
    // **Y1's class for this act.** R1 item 5's Debian route. **It is a package
    // install as well, and either word would do** — the driver word is carried
    // because the route is what the command belongs to.
    what_it_does_to_the_machine: WhatAnActDoesToTheMachine::ADriverAct,
    plain: CommandString::constant("apt-get -o DPkg::Lock::Timeout=120 install nvidia-driver"),
    elevated: CommandString::constant(
        "sudo -n apt-get -o DPkg::Lock::Timeout=120 install nvidia-driver",
    ),
    elevated_reading_the_password_from_standard_input: CommandString::constant(
        "sudo -S apt-get -o DPkg::Lock::Timeout=120 install nvidia-driver",
    ),
};

/// **The machine's package lists refreshed** — M1b's Debian components step,
/// second half, and **S7d's refresh before every package install of a setup
/// run**.
///
/// **Two callers.** M1b's components step issues it by name once the
/// components are enabled (the first half is not a command at all and is
/// [`crate::acts::enable_the_components`]'s). **And [`issue`] sends it in front
/// of every install in [`THE_INSTALLS_THE_LISTS_ARE_REFRESHED_BEFORE`]**, which
/// is S7d, added 2026-09-22: the third pass of step 10c met Hyperstack's plain
/// Ubuntu 24.04 image carrying lists from June 2025, and the install of
/// `ubuntu-drivers-common` fetched a version the archive no longer held —
/// *"404  Not Found"*, then apt's own *"Unable to fetch some archives, maybe run
/// apt-get update or try with --fix-missing?"*.
pub const THE_LIST_UPDATE: AnElevatedStep = AnElevatedStep {
    what_it_does: "update the package lists",
    // **Y1's class for this act.** M1b's Debian driver route names it as the
    // second half of its components step, and there it is a driver act. **It
    // installs nothing**; where [`issue`] sends it in front of an install, it is
    // part of that install's act and the caller has already set Y1's state for
    // that install's own class before anything goes out.
    what_it_does_to_the_machine: WhatAnActDoesToTheMachine::ADriverAct,
    plain: CommandString::constant("apt-get -o DPkg::Lock::Timeout=120 update"),
    elevated: CommandString::constant("sudo -n apt-get -o DPkg::Lock::Timeout=120 update"),
    elevated_reading_the_password_from_standard_input: CommandString::constant("sudo -S apt-get -o DPkg::Lock::Timeout=120 update"),
};

/// **S7d: every package install of a setup run, on either family, is preceded
/// by a refresh of the package lists** — and these are they.
///
/// **Why.** An install is resolved against the lists the machine holds, and a
/// rented image's lists are as old as the image. apt's own manual,
/// `apt-get(8)`: *"update is used to resynchronize the package index files from
/// their sources"*; and where a fetch fails apt's own source prints *"Unable to
/// fetch some archives, maybe run apt update or try with --fix-missing?"* —
/// both quoted at S7d with the addresses and the date read. **Observed**:
/// Hyperstack's plain Ubuntu 24.04 image, lists dated June 2025, the install
/// answered *404 Not Found* (step 10c's third pass, 2026-09-22).
///
/// **The class, every install on either route** (S7d lists it the same way):
///
/// - [`THE_TOOLING_INSTALL`] — R1 item 3, either family. **Refreshed.**
/// - [`THE_LIBRARIES_INSTALL_FOR_THE_FIRST_ENTRY`] — R1 item 4, either family.
///   **Refreshed.**
/// - [`THE_UBUNTU_DRIVER_TOOL_INSTALL`] — M1b's Ubuntu route. **Refreshed.**
/// - [`THE_DEBIAN_DRIVER_INSTALL`] — M1b's Debian route. **Refreshed**, and
///   where the components step has just enabled the components the lists are
///   then refreshed twice; M1b's own refresh is left where M1b puts it.
/// - [`THE_UBUNTU_DRIVER_INSTALL`] — not in this list: it is not an `apt-get`
///   command, and it is always issued in the same step directly after the tool
///   install above, whose refresh it runs against.
/// - [`THE_UBUNTU_DRIVER_SUPPORT_BINARIES_INSTALL`] — not in this list, for the
///   same reason and one more. **It is always issued in the same step directly
///   after the driver install**, which is directly after the tool install, whose
///   refresh it runs against — a fourth refresh inside one step would ask a paid
///   machine the same question three times over. **And it could not be refreshed
///   by this route anyway**: [`issue`] sends the refresh as its own command
///   before the step's, and this step's command derives its package name on the
///   machine from the driver package the act before it put there (R1 item 5 as
///   corrected on 2026-09-30), so the read and the install must stay one
///   command and one act.
/// - [`THE_UBUNTU_OPTIX_LIBRARY_INSTALL`] — not in this list, for the support
///   binaries' two reasons: it is issued in the same step directly after them,
///   and its lookup, its read and its install are one command. Added
///   2026-10-03.
/// - [`THE_REPAIRS_SECOND_COMMAND`] — not in this list: S7's repair, which no
///   route issues, and whether a refresh may go in front of the repair of an
///   interrupted transaction is not read.
/// - [`THE_MANAGER_ROUTE_REMOVAL`] — a removal, and not through `apt-get`.
///
/// **Held by [`tests::every_apt_install_is_refreshed_first_or_says_why_not`].**
pub const THE_INSTALLS_THE_LISTS_ARE_REFRESHED_BEFORE: [AnElevatedStep; 4] = [
    THE_TOOLING_INSTALL,
    THE_LIBRARIES_INSTALL_FOR_THE_FIRST_ENTRY,
    THE_UBUNTU_DRIVER_TOOL_INSTALL,
    THE_DEBIAN_DRIVER_INSTALL,
];

/// **M1b's Debian components step, the write itself.**
///
/// **The content crosses the file-transfer subsystem to a Smoozy-synthesised
/// staging token and the elevated command names the token and the fixed
/// destination** — both admissible classes, and it is exactly the shape E7
/// already declares for the container boot hook. **No component name and no
/// path of machine origin is in the string.**
pub const THE_COMPONENTS_WRITE: AnElevatedStep = AnElevatedStep {
    what_it_does: "put the rewritten sources file at its fixed destination",
    // **Y1's class for this act.** M1b's Debian driver route, its components
    // step. **Issued only as part of that route.**
    what_it_does_to_the_machine: WhatAnActDoesToTheMachine::ADriverAct,
    plain: CommandString::constant(
        "cp ~/.smoozy-carriage/debian-sources /etc/apt/sources.list.d/debian.sources",
    ),
    elevated: CommandString::constant(
        "sudo -n cp ~/.smoozy-carriage/debian-sources /etc/apt/sources.list.d/debian.sources",
    ),
    elevated_reading_the_password_from_standard_input: CommandString::constant(
        "sudo -S cp ~/.smoozy-carriage/debian-sources /etc/apt/sources.list.d/debian.sources",
    ),
};

/// **S7's repair operation, first command**, and nothing forced.
pub const THE_REPAIRS_FIRST_COMMAND: AnElevatedStep = AnElevatedStep {
    what_it_does: "configure what was unpacked and not configured",
    // **Y1's class for this act.** S7's repair, which finishes a package
    // transaction the machine itself reports interrupted. **It reaches the
    // package system to put packages on the machine**, which is Y1's first act
    // however the command is spelled.
    what_it_does_to_the_machine: WhatAnActDoesToTheMachine::AnElevatedPackageInstall,
    plain: crate::recovery::THE_REPAIR_OPERATION[0],
    elevated: CommandString::constant("sudo -n dpkg --configure -a"),
    elevated_reading_the_password_from_standard_input: CommandString::constant("sudo -S dpkg --configure -a"),
};

/// **S7's repair operation, second command.**
pub const THE_REPAIRS_SECOND_COMMAND: AnElevatedStep = AnElevatedStep {
    what_it_does: "fix what the interrupted transaction broke",
    // **Y1's class for this act.** S7's repair, second command, and the same
    // reason.
    what_it_does_to_the_machine: WhatAnActDoesToTheMachine::AnElevatedPackageInstall,
    plain: crate::recovery::THE_REPAIR_OPERATION[1],
    elevated: CommandString::constant(
        "sudo -n apt-get -o DPkg::Lock::Timeout=120 --fix-broken install",
    ),
    elevated_reading_the_password_from_standard_input: CommandString::constant(
        "sudo -S apt-get -o DPkg::Lock::Timeout=120 --fix-broken install",
    ),
};

/// **T1's restart**, issued over the connection Smoozy already holds, only on
/// the systemd kind, and only as part of a setup step that needs it.
pub const THE_RESTART: AnElevatedStep = AnElevatedStep {
    what_it_does: "restart the machine as part of the driver step",
    // **Y1's class for this act.** Y1's third act, and the one
    // `Rented-server runtime` P4 requires the state to be durable before.
    what_it_does_to_the_machine: WhatAnActDoesToTheMachine::ARestart,
    plain: crate::restart::THE_RESTART,
    elevated: CommandString::constant("sudo -n systemctl reboot"),
    elevated_reading_the_password_from_standard_input: CommandString::constant("sudo -S systemctl reboot"),
};

/// **B15's manager-route removal, under the one package name this module may
/// name** — the store's own, which B2 already reads the store by.
///
/// **A managed build under any other name has no admissible carriage for its
/// removal** and is reported as one this module cannot remove (§13 item 21).
pub const THE_MANAGER_ROUTE_REMOVAL: AnElevatedStep = AnElevatedStep {
    what_it_does: "remove the managed build through the machine's own software manager",
    // **Y1's class for this act.** **A removal is not an install**, and Y1's
    // additive class names *a Blender removal* by name. B14's warning is what a
    // removal costs live work, and Y1 says in as many words that the state would
    // not help with it.
    what_it_does_to_the_machine: WhatAnActDoesToTheMachine::NoneOfY1sThree,
    plain: CommandString::constant("snap remove blender"),
    elevated: CommandString::constant("sudo -n snap remove blender"),
    elevated_reading_the_password_from_standard_input: CommandString::constant("sudo -S snap remove blender"),
};

/// **B11's elevated search retry** — *"the same constant search command as the
/// unprivileged pass: fixed name, fixed root"*.
///
/// **It carried the same `-type f` the unprivileged pass did, and it came out of
/// both on 2026-09-04.** The restriction is not B10's and not B11's, and a
/// retry that narrowed where the first pass did not would not be the same
/// command at all. The reasoning and the cost are written once, at
/// [`crate::readings::THE_SEARCH`] — **and so is the `! -type d` both passes
/// carry from 2026-09-26**, which leaves directories out and nothing else, and
/// **the pruning of `/proc` and `/sys` from 2026-10-02** (B10). A test holds
/// the two passes to one search.
///
/// **B11 withdrew this retry on 2026-10-02** (the Creator's note 13 on 0.1.7):
/// the refusal count is no longer drawn, so no line offers the retry. The form
/// stays the same search, pruned the same way, so a caller of the retry
/// machinery in `acts.rs` cannot run a different one; nothing on a screen
/// reaches it.
pub const THE_ELEVATED_SEARCH: AnElevatedStep = AnElevatedStep {
    what_it_does: "run the whole-disk search again with elevation, after a refusal",
    // **Y1's class for this act.** A reading retried with elevation. **It
    // changes nothing**, and it is not one of the run's own steps at all.
    what_it_does_to_the_machine: WhatAnActDoesToTheMachine::NoneOfY1sThree,
    plain: crate::readings::THE_SEARCH,
    elevated: CommandString::constant(
        "sudo -n find / -path /proc -prune -o -path /sys -prune -o -name blender ! -type d -print",
    ),
    elevated_reading_the_password_from_standard_input: CommandString::constant(
        "sudo -S find / -path /proc -prune -o -path /sys -prune -o -name blender ! -type d -print",
    ),
};

// ──────────────── `Rented-server runtime` K10, K11 and K19 — the container's hook

/// **Where the hook's content is staged before the copy**, relative to the
/// login's own starting directory — E7's *"Smoozy-synthesised staging token"*,
/// beside the other staging tokens in the Smoozy-named directory.
pub const THE_CONTAINER_HOOKS_STAGING_TOKEN: &str = ".smoozy-carriage/container-hook";

/// **K11: the staged hook copied to K10's path.** E7's own declaration for this
/// step: *"the destination path and the mode change are constant, per K10's
/// fixed provider path; the hook's content crosses the subsystem to a
/// Smoozy-synthesised staging token, and the elevated command names the token and
/// the fixed destination"*. **The same shape as [`THE_COMPONENTS_WRITE`].**
///
/// **Documented** — GNU coreutils manual, *cp invocation*,
/// `https://www.gnu.org/software/coreutils/manual/html_node/cp-invocation.html`,
/// read 2026-09-21: *"If two file names are given, cp copies the first file to
/// the second"*; *"When copying to a symbolic link, cp follows the link only when
/// it refers to an existing regular file"* — which is the provider's own case,
/// its boot line linking the path to the file where one was not there; and
/// *"In the absence of this option, the permissions of existing destination files
/// are unchanged"*, which is why K19's two mode changes are separate commands
/// rather than assumed.
pub const THE_CONTAINER_HOOKS_COPY: AnElevatedStep = AnElevatedStep {
    what_it_does: "put the boot hook at the provider's own path",
    // **Y1's class for this act.** A file is written. **Not one of Y1's three**,
    // and it runs inside the placement, before any setup run exists to name.
    what_it_does_to_the_machine: WhatAnActDoesToTheMachine::NoneOfY1sThree,
    plain: CommandString::constant("cp ~/.smoozy-carriage/container-hook /root/onstart.sh"),
    elevated: CommandString::constant(
        "sudo -n cp ~/.smoozy-carriage/container-hook /root/onstart.sh",
    ),
    elevated_reading_the_password_from_standard_input: CommandString::constant(
        "sudo -S cp ~/.smoozy-carriage/container-hook /root/onstart.sh",
    ),
};

/// **K19, first half: the hook left readable by the login that reads it back.**
///
/// **Documented** — GNU coreutils manual, *chmod invocation*,
/// `https://www.gnu.org/software/coreutils/manual/html_node/chmod-invocation.html`,
/// read 2026-09-21: *"chmod changes the access permissions of the named files"*,
/// and *"for each symbolic link listed on the command line, chmod changes the
/// permissions of the pointed-to file"* — so the provider's link, where there is
/// one, is not what is changed.
pub const THE_CONTAINER_HOOK_MADE_READABLE: AnElevatedStep = AnElevatedStep {
    what_it_does: "leave the boot hook readable to the login that reads it back",
    // **Y1's class for this act.** A mode change. **Not one of Y1's three.**
    what_it_does_to_the_machine: WhatAnActDoesToTheMachine::NoneOfY1sThree,
    plain: CommandString::constant("chmod a+r /root/onstart.sh"),
    elevated: CommandString::constant("sudo -n chmod a+r /root/onstart.sh"),
    elevated_reading_the_password_from_standard_input: CommandString::constant(
        "sudo -S chmod a+r /root/onstart.sh",
    ),
};

/// **K19, second half, and it is the first of the two routes K19 names**:
/// *"Making the containing directory traversable is enough — a named file inside
/// it is then readable while listing the directory stays refused, so nothing
/// else in it is exposed."* **Search only**, never read: `a+x` on a directory
/// lets a name inside it be reached and does not let the directory be listed.
/// The same `chmod` publication as above.
pub const THE_CONTAINER_HOOKS_DIRECTORY_MADE_REACHABLE: AnElevatedStep = AnElevatedStep {
    what_it_does: "make the boot hook's directory reachable, without making it listable",
    // **Y1's class for this act.** A mode change. **Not one of Y1's three.**
    what_it_does_to_the_machine: WhatAnActDoesToTheMachine::NoneOfY1sThree,
    plain: CommandString::constant("chmod a+x /root"),
    elevated: CommandString::constant("sudo -n chmod a+x /root"),
    elevated_reading_the_password_from_standard_input: CommandString::constant(
        "sudo -S chmod a+x /root",
    ),
};

/// **The provider's own hook, read with elevation**, where the plain read over
/// the file-transfer subsystem is refused — K19's own observation, *"the
/// directory the hook sits in is mode 0700 root-owned"*. It is read so that what
/// the provider's file holds is kept rather than written over.
///
/// **Documented** — GNU coreutils manual, *cat invocation*,
/// `https://www.gnu.org/software/coreutils/manual/html_node/cat-invocation.html`,
/// read 2026-09-21: *"cat copies each file ('-' means standard input), or
/// standard input if none are given, to standard output."*
pub const THE_CONTAINER_HOOKS_READ: AnElevatedStep = AnElevatedStep {
    what_it_does: "read what the provider's own boot hook holds, where the login may not",
    // **Y1's class for this act.** A reading. **It changes nothing.**
    what_it_does_to_the_machine: WhatAnActDoesToTheMachine::NoneOfY1sThree,
    plain: CommandString::constant("cat /root/onstart.sh"),
    elevated: CommandString::constant("sudo -n cat /root/onstart.sh"),
    elevated_reading_the_password_from_standard_input: CommandString::constant(
        "sudo -S cat /root/onstart.sh",
    ),
};

/// **K11's and K19's root step, as the placement is handed it**, for how
/// elevation stands on this login.
///
/// - **Passwordless** (`Server Setup` M3's probe succeeded): every command has its
///   non-interactive elevated form.
/// - **Otherwise the elevated forms are absent**, and the placement's walk tries
///   the plain copy and, where that is refused, fails before the commit in words
///   that say a password would be needed. **A password is never held at this
///   point**: `Server Setup` E2's ask is made at a run step, and the placement
///   comes before the run (R2). **The standard-input forms above are written so
///   the set stays whole and paired**, and nothing sends them yet — which is
///   recorded at this crate's decisions record rather than papered over.
///
/// **A login that is the container's own root** needs none of the elevated
/// forms: the plain copy works, and the walk makes K19's two changes only where
/// the copy needed elevation.
pub fn the_root_step_on_a_container(
    how: HowElevationIsObtained<'_>,
) -> smoozy_server_protocol::placement::TheRootStepOnAContainer {
    use smoozy_server_protocol::placement::TheCommandAndItsElevatedForm;
    let passwordless = how == HowElevationIsObtained::Passwordless;
    let elevated = |step: AnElevatedStep| passwordless.then_some(step.elevated);
    smoozy_server_protocol::placement::TheRootStepOnAContainer {
        the_hooks_path: smoozy_server_protocol::placement::THE_PROVIDERS_HOOK_PATH.to_string(),
        the_staging_token_on_the_subsystem: THE_CONTAINER_HOOKS_STAGING_TOKEN.to_string(),
        the_copy: TheCommandAndItsElevatedForm {
            plain: THE_CONTAINER_HOOKS_COPY.plain,
            elevated: elevated(THE_CONTAINER_HOOKS_COPY),
        },
        the_hook_made_readable: elevated(THE_CONTAINER_HOOK_MADE_READABLE),
        the_directory_made_traversable: elevated(THE_CONTAINER_HOOKS_DIRECTORY_MADE_REACHABLE),
        the_hook_read: elevated(THE_CONTAINER_HOOKS_READ),
    }
}

/// **E7's enumeration, as the code performs it.** Every elevated command this
/// module issues is here, so a reader sees the whole set at once and a test can
/// hold each of them to the rule.
pub const THE_ELEVATED_STEPS: [AnElevatedStep; 23] = [
    THE_CREATION_OF_THE_ROOT,
    THE_CREATION_OF_THE_PACKAGE_AREAS_LOCATION,
    THE_HAND_OVER_OF_THE_ROOT,
    THE_HAND_OVER_OF_THE_PACKAGE_AREA,
    THE_TOOLING_INSTALL,
    THE_LIBRARIES_INSTALL_FOR_THE_FIRST_ENTRY,
    THE_UBUNTU_DRIVER_TOOL_INSTALL,
    THE_UBUNTU_DRIVER_INSTALL,
    THE_UBUNTU_DRIVER_SUPPORT_BINARIES_INSTALL,
    THE_UBUNTU_OPTIX_LIBRARY_INSTALL,
    THE_UBUNTU_DRIVER_SWAP,
    THE_UBUNTU_DRIVER_SWAPS_FINISH,
    THE_DEBIAN_DRIVER_INSTALL,
    THE_LIST_UPDATE,
    THE_COMPONENTS_WRITE,
    THE_REPAIRS_FIRST_COMMAND,
    THE_REPAIRS_SECOND_COMMAND,
    THE_RESTART,
    THE_MANAGER_ROUTE_REMOVAL,
    THE_CONTAINER_HOOKS_READ,
    THE_CONTAINER_HOOKS_COPY,
    THE_CONTAINER_HOOK_MADE_READABLE,
    THE_CONTAINER_HOOKS_DIRECTORY_MADE_REACHABLE,
    // The elevated search is a retry of an unprivileged reading rather than a
    // step of a run, so it is named at [`THE_ELEVATED_SEARCH`] and is not one of
    // the run's own steps.
];

/// **E7's rule, kept over the whole set at once**: the libraries install is the
/// only step whose command a reading selects, and it selects one of a fixed set.
pub fn the_libraries_install(
    what_to_install: &crate::ready_action::WhatTheLibrariesStepInstalls,
) -> Option<AnElevatedStep> {
    use crate::ready_action::WhatTheLibrariesStepInstalls as What;
    match what_to_install {
        // Nothing to do is not a step that fails; it is a step with no act.
        What::NothingWasMissing => None,
        // **The mapping does not name them, so no command is composed and none
        // is issued.** R1 item 4's own rule: never a partial install.
        What::TheMappingDoesNotNameThese(_) => None,
        What::ThesePackages(packages) => {
            let the_first_entry = crate::ready_action::THE_MAPPING[0].packages;
            // **The selection is one of a fixed set or it is none.** With one
            // entry in the mapping there is one admissible selection; a second
            // entry would need its own constant, which is what E7 requires and
            // what a merged command would break.
            if packages.as_slice() == the_first_entry {
                Some(THE_LIBRARIES_INSTALL_FOR_THE_FIRST_ENTRY)
            } else {
                None
            }
        }
    }
}

/// How elevation is obtained for one step.
///
/// **Three states rather than two, because *a password is owed* and *a password
/// is held* are different moments of E2's flow** — the offer is made at the
/// first and taken up at the second, and a step meets whichever the run is at.
#[derive(Debug, Clone, Copy)]
pub enum HowElevationIsObtained<'a> {
    /// **M3's probe succeeded**: the elevating command's non-interactive form
    /// runs the step and **no password is asked for, held or sent.**
    Passwordless,
    /// **A password is owed and none is held**, and E2's flow begins at the
    /// moment a step needs it — never before.
    APasswordIsOwedAndNoneIsHeld,
    /// **A password is held**, so E3's form runs and E4's validate goes first.
    ///
    /// **Who holds it across a connected period is not settled here.**
    /// `Security and licence` §2 item 3b's unstored hold owns that, and E5 says
    /// the lifetime is the connection's; what a step needs is a value for two
    /// deliveries — the validate and the operation — which is what this type
    /// gives.
    APasswordIsHeld(&'a Origin),
}

/// **Comparing two of these compares the state and never a secret.**
///
/// `Security and licence`'s own plaintext type has no equality on purpose —
/// §13 rule 4 — so two held passwords are the same here only when they are the
/// same hold, which is what a caller asking *is this the same state* means.
impl PartialEq for HowElevationIsObtained<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (HowElevationIsObtained::Passwordless, HowElevationIsObtained::Passwordless) => true,
            (
                HowElevationIsObtained::APasswordIsOwedAndNoneIsHeld,
                HowElevationIsObtained::APasswordIsOwedAndNoneIsHeld,
            ) => true,
            (
                HowElevationIsObtained::APasswordIsHeld(one),
                HowElevationIsObtained::APasswordIsHeld(other),
            ) => std::ptr::eq(*one, *other),
            _ => false,
        }
    }
}

impl Eq for HowElevationIsObtained<'_> {}

impl HowElevationIsObtained<'_> {
    /// **M3's two outcomes, read as this module's own question.**
    ///
    /// **The probe answers whether a password is owed and never whether one is
    /// held**, so the third state is not reachable from it: a held password
    /// arrives from the ask, not from the probe.
    pub fn from(probe: Probe) -> HowElevationIsObtained<'static> {
        match probe {
            Probe::Succeeded => HowElevationIsObtained::Passwordless,
            Probe::Failed => HowElevationIsObtained::APasswordIsOwedAndNoneIsHeld,
        }
    }
}

/// Why an elevated step did not run at all.
///
/// **Neither is a classification of the machine's own words** — both are this
/// application's knowledge of what it was doing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhyTheStepDidNotRun {
    /// **A password is owed for this step and none is held**, which is E4's
    /// fourth bullet: the moment the offer is made and the ask happens, at that
    /// moment and not before. **Nothing is sent** — a step that ran unelevated
    /// here would fail for a reason nobody could tell from a refusal.
    NoPasswordIsHeldYet,
    /// The request did not reach the machine.
    TheRequestDidNotReachTheMachine,
}

impl WhyTheStepDidNotRun {
    /// **What this application knows, in standard software words** (Rule 9),
    /// and nothing it does not (Rule 8). Added 2026-09-21: the only rendering
    /// of this until then was a debugging one, and it reached the screen.
    pub fn in_words(&self) -> &'static str {
        match self {
            WhyTheStepDidNotRun::NoPasswordIsHeldYet => {
                "Admin access needed and no password given. Nothing was sent."
            }
            WhyTheStepDidNotRun::TheRequestDidNotReachTheMachine => {
                "The request did not reach the server."
            }
        }
    }
}

/// What one elevated step said, with the machine's own words kept whole.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheElevatedStepSaid {
    /// Whether the step ended cleanly.
    pub it_ended_cleanly: bool,
    /// **What the step wrote to its standard output**, kept apart from the
    /// complaint stream and never merged with it. Most steps write nothing
    /// here; B11's elevated search is the one that answers on it.
    pub the_standard_output: Vec<u8>,
    /// **The machine's own words, verbatim, whole, streams kept apart** —
    /// `Security and licence` §13 rule 5 and `Rented-server runtime` M7 and M8's
    /// discipline, cited rather than restated: **never mined for a cause, never
    /// merged, never trimmed.**
    pub the_machines_own_words: String,
    /// **Which of E4's four bullets this was**, decided where the validate's own
    /// result is known and nowhere else.
    ///
    /// **It is recognised here and consumed by nobody in this crate.** The
    /// erase it can call for is `Security and licence` §5.1's — its order, its
    /// recorded debt and its sweep are that document's and are neither performed
    /// nor restated here.
    pub what_it_met: WhatTheStepMet,
}

/// **One elevated step, issued over the command channel.**
///
/// **Both forms are performed.** Both are Y7's *elevated steps are performed
/// over the SSH command channel by the machine's own elevating command*; what
/// differs is whether a password is carried, and where a password is carried
/// E4 puts the validate in front of the operation.
///
/// **E4's order, and what it separates.** The validate runs no command at all,
/// so a failed validate is never an operation's failure: it is classified as an
/// authentication result, the operation is not attempted, and the machine's own
/// words ride the answer. **Where the validate passes, the step follows and
/// carries the password again**, because the credential cache is not relied on
/// ([`crate::elevation::the_credential_cache_is_relied_on`]) and the validate's
/// own result is what classifies either way.
///
/// **E6, kept by there being no loop here.** One validate, one operation, and
/// no second attempt on either — rate limiting corrupts the diagnosis
/// (`Security and licence` §4).
///
/// **An elevated command's process is the connection's child**, so a connection
/// lost mid-step kills the step mid-flight and the boundary then honestly names
/// the last one passed — which is Y7's own sentence and Fact 49's hard-cut
/// shape.
///
/// **S7d: an install in [`THE_INSTALLS_THE_LISTS_ARE_REFRESHED_BEFORE`] is
/// preceded by [`THE_LIST_UPDATE`], in the same form.** Added 2026-09-22. It is
/// done here, the one place every elevated command leaves from, so no caller
/// can issue one of those installs without it. **A refresh that did not end
/// cleanly is the answer**, in its own words, and the install is not sent:
/// against lists that could not be refreshed it would be the same install that
/// failed on the third pass.
///
/// **Y7a: every command of the step leaves through `channel`**, so a channel
/// that reads its commands' two streams as they are printed — the desktop's,
/// which keeps them in a [`WhatTheRunPrinted`] — reads the whole step, the
/// refresh in front of an install included. Nothing here changes for it: what
/// comes back is the same whole answer. Added 2026-10-18.
pub async fn issue(
    channel: &mut (impl CommandChannel + CarriesAValueOnACommandsStandardInput),
    step: AnElevatedStep,
    how: HowElevationIsObtained<'_>,
) -> Result<WhatTheElevatedStepSaid, WhyTheStepDidNotRun> {
    if THE_INSTALLS_THE_LISTS_ARE_REFRESHED_BEFORE.contains(&step) {
        let refreshed = issue_one(channel, THE_LIST_UPDATE, how).await?;
        if !refreshed.it_ended_cleanly {
            return Ok(refreshed);
        }
    }
    issue_one(channel, step, how).await
}

/// **One elevated command, in E4's order, and nothing in front of it.**
async fn issue_one(
    channel: &mut (impl CommandChannel + CarriesAValueOnACommandsStandardInput),
    step: AnElevatedStep,
    how: HowElevationIsObtained<'_>,
) -> Result<WhatTheElevatedStepSaid, WhyTheStepDidNotRun> {
    match how {
        // **E4's fourth bullet: nothing is sent.** A step that ran unelevated
        // here would fail for a reason nobody could tell from a refusal.
        HowElevationIsObtained::APasswordIsOwedAndNoneIsHeld => {
            Err(WhyTheStepDidNotRun::NoPasswordIsHeldYet)
        }
        HowElevationIsObtained::Passwordless => match channel.run(step.elevated).await {
            Ok(result) => Ok(WhatTheElevatedStepSaid {
                it_ended_cleanly: result.ended_cleanly(),
                // **E4's third bullet, and it is reached rather than borrowed.**
                // Passwordless elevation holds no password and has no validate
                // to fail, so authentication succeeded either way — **and what
                // the step met is what the operation did**, which from
                // 2026-09-04 has a value for either outcome. Both forget
                // nothing.
                what_it_met: what_the_operation_met(result.ended_cleanly()),
                the_standard_output: result.output,
                the_machines_own_words: String::from_utf8_lossy(&result.complaint).into_owned(),
            }),
            Err(_) => Err(WhyTheStepDidNotRun::TheRequestDidNotReachTheMachine),
        },
        HowElevationIsObtained::APasswordIsHeld(password) => {
            let validate = channel
                .run_carrying_a_value_on_its_standard_input(
                    THE_VALIDATE_FORM,
                    password.for_a_destination(),
                )
                .await
                .map_err(|_| WhyTheStepDidNotRun::TheRequestDidNotReachTheMachine)?;

            if !validate.ended_cleanly() {
                let words = String::from_utf8_lossy(&validate.complaint).into_owned();
                return Ok(WhatTheElevatedStepSaid {
                    // **Nothing ran.** The validate runs no command, so the step
                    // did not end cleanly and did not end at all.
                    it_ended_cleanly: false,
                    the_standard_output: Vec::new(),
                    // **The reading of the captured text is `Security and
                    // licence` §13 rule 5's** and is made in that crate, not
                    // here: one exact constant is matched, or nothing is
                    // concluded.
                    what_it_met: what_a_failed_validate_means(true, false, false, &words),
                    the_machines_own_words: words,
                });
            }

            match channel
                .run_carrying_a_value_on_its_standard_input(
                    step.elevated_reading_the_password_from_standard_input,
                    password.for_a_destination(),
                )
                .await
            {
                Ok(result) => Ok(WhatTheElevatedStepSaid {
                    it_ended_cleanly: result.ended_cleanly(),
                    // **E4's third bullet.** The validate passed, so whatever
                    // the operation did is the operation's, and it forgets
                    // nothing either way.
                    what_it_met: what_the_operation_met(result.ended_cleanly()),
                    the_standard_output: result.output,
                    the_machines_own_words: String::from_utf8_lossy(&result.complaint).into_owned(),
                }),
                Err(_) => Err(WhyTheStepDidNotRun::TheRequestDidNotReachTheMachine),
            }
        }
    }
}

/// **E4: what the step met once the authentication is out of the way**, which
/// is the operation's own outcome and nothing else.
///
/// **Both values forget nothing**, and both existing readers give exactly the
/// answers they gave before this value existed — which is what makes adding it a
/// repair of the record rather than a change of behaviour.
fn what_the_operation_met(it_ended_cleanly: bool) -> WhatTheStepMet {
    if it_ended_cleanly {
        WhatTheStepMet::ASuccessfulOperationAfterASuccessfulValidate
    } else {
        WhatTheStepMet::AFailedOperationAfterASuccessfulValidate
    }
}

/// **E6, stated as the question a builder would otherwise answer wrongly.**
///
/// **No second route is opened, ever**: not the password as a command argument,
/// which Fact 60 forbids because an argument is readable by any other login on
/// the machine; not an environment value; and not a second elevating command.
/// **This held while E3's route was unavailable and it holds now that it is
/// there** — the route that landed is E3's own, the elevating command reading
/// the password from standard input, and no other was made on the way to it.
pub fn a_second_route_beside_the_elevating_command_is_opened(_it_would_help: bool) -> bool {
    false
}

// ── Y7a: what a run's elevated steps printed, read as it was printed ─────────

/// **Which of an elevated command's two streams some bytes came on.** Added
/// 2026-10-18, the Creator's thirteenth notes, note 7.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhichStream {
    /// The command's standard output — the ordinary stream.
    TheOrdinaryOutput,
    /// Its complaint output.
    TheComplaints,
}

/// **One step's words, both streams apart.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OneStepsWords {
    /// The step that printed them.
    pub step: crate::ready_action::Step,
    /// What it wrote to its standard output, every command of the step in the
    /// order they ran.
    pub the_ordinary_output: Vec<u8>,
    /// What it wrote to its complaint output, the same way.
    pub the_complaints: Vec<u8>,
    /// **Y7a item 5: some of what this step printed could not be brought back
    /// from the machine**, where a worker step's words would have passed the
    /// largest frame and its report or the run's state reading carried the
    /// mark in their place. **Never a part of either stream**: what the step
    /// printed is here whole or not at all. Added 2026-10-24, the Creator's
    /// ruling on the sixteenth notes, note 1.
    pub some_were_not_kept: bool,
}

/// **Y7a: the run's words, kept apart by stream and by step.** Added
/// 2026-10-18, Creator-stated, the thirteenth notes, note 7.
///
/// **Y7a: *"An elevated step's two streams are read by the desktop as they are
/// printed, and the desktop keeps the run's words, apart by stream and by
/// step, until the next run or until Smoozy closes."*** This is the keeping:
/// whoever holds one for a run hands it each piece as it is read, with the
/// step it belongs to and the stream it came on, and nothing else. **Where it
/// is held, and for how long, is the desktop's** — a new value for the next
/// run, and nothing written anywhere.
///
/// **Every command of a step goes through [`issue`] on the one channel** — the
/// lists refreshed before an install (S7d), the validate (E4), the operation —
/// so a channel that reads what it carries as printed reads the whole step.
/// **Nothing here reads a cause off the words** (`Security and licence` §13
/// rule 5): a piece is kept whole, in the order it came, under its stream.
///
/// **What it is not.** It is not S2a's record of a failed step's words, which
/// is written onto the machine exactly as before and is what *What the server
/// said* opens. ~~**A machine-side step adds nothing to it**, since that step's
/// words are the worker's own capture and reach the desktop only with a failed
/// ending (Y7a's own sentence).~~ **From 2026-10-24 every command a run issues
/// is kept here** (Y7a as changed, the Creator's ruling on the sixteenth notes,
/// note 1): the readings that compose the run, read as printed (item 1), and a
/// worker step's two streams, handed over once at the reading that shows the
/// step ended (item 2) — or, where they could not be brought back whole, the
/// step's mark that they were not ([`Self::its_words_were_not_kept`], item 5).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WhatTheRunPrinted {
    steps: Vec<OneStepsWords>,
}

impl WhatTheRunPrinted {
    /// **Some bytes a step printed**, kept under that step and that stream.
    ///
    /// **A step is the same entry for as long as it keeps printing**; a
    /// different step starts the next one. A step met a second time after
    /// another — a password asked for between two tries of the same step —
    /// starts an entry of its own, because the run printed in that order.
    pub fn printed(&mut self, step: crate::ready_action::Step, stream: WhichStream, bytes: &[u8]) {
        if bytes.is_empty() {
            return;
        }
        let this_one = self.the_entry_of(step);
        match stream {
            WhichStream::TheOrdinaryOutput => this_one.the_ordinary_output.extend_from_slice(bytes),
            WhichStream::TheComplaints => this_one.the_complaints.extend_from_slice(bytes),
        }
    }

    /// **Y7a item 5: some of what a step printed could not be brought back**,
    /// kept under that step as the mark and nothing else — no part of either
    /// stream is made up for it. Added 2026-10-24.
    pub fn its_words_were_not_kept(&mut self, step: crate::ready_action::Step) {
        self.the_entry_of(step).some_were_not_kept = true;
    }

    /// **Whether any step's words could not be brought back**, which is what
    /// the *Server output* panel says under the words it has.
    pub fn some_words_were_not_kept(&self) -> bool {
        self.steps.iter().any(|one| one.some_were_not_kept)
    }

    /// **The entry a step's words go into**: the last one where the step is
    /// still the one printing, and a new one where another step printed since.
    fn the_entry_of(&mut self, step: crate::ready_action::Step) -> &mut OneStepsWords {
        if self.steps.last().map(|last| last.step) != Some(step) {
            self.steps.push(OneStepsWords {
                step,
                the_ordinary_output: Vec::new(),
                the_complaints: Vec::new(),
                some_were_not_kept: false,
            });
        }
        self.steps.last_mut().expect("an entry was just made where none was")
    }

    /// **Each step's words, in the order the run printed them.**
    pub fn the_steps(&self) -> &[OneStepsWords] {
        &self.steps
    }

    /// **One stream, whole: every step's record of it in turn**, with nothing
    /// put between them and nothing taken out. A step that printed nothing on
    /// it adds nothing.
    pub fn the_whole_of(&self, stream: WhichStream) -> Vec<u8> {
        let mut whole = Vec::new();
        for one in &self.steps {
            whole.extend_from_slice(match stream {
                WhichStream::TheOrdinaryOutput => &one.the_ordinary_output,
                WhichStream::TheComplaints => &one.the_complaints,
            });
        }
        whole
    }
}

/// **A line a program rewrites in place, shown in its newest form, as a
/// terminal shows it** — `Desktop app and delivery` M13b's second narrowing,
/// as changed on 2026-10-24: *"a line a program rewrites in place is shown in
/// its newest form, here and in the whole record's panel, as a terminal shows
/// it, while the record keeps every byte as printed"* (`Desktop app and
/// delivery` M9). Added 2026-10-25,
/// the eighteenth notes, piece 2c.
///
/// **A carriage return sends the line back to its start and what follows
/// writes over it, character by character**, so a shorter rewrite leaves the
/// end of the longer one standing, the way a terminal leaves it. A carriage
/// return before a line feed changes nothing. **Nothing else is read**: no
/// other character is given a meaning, nothing is taken out, and the words
/// are never read for a cause (`Security and licence` §13 rule 5). **It is
/// how the words are drawn and never what is kept**: the record and *Copy*
/// keep every byte as printed.
pub fn as_a_terminal_shows_it(printed: &str) -> String {
    let mut shown = String::with_capacity(printed.len());
    let mut line: Vec<char> = Vec::new();
    let mut at = 0;
    for character in printed.chars() {
        match character {
            '\r' => at = 0,
            '\n' => {
                shown.extend(line.drain(..));
                shown.push('\n');
                at = 0;
            }
            other => {
                if at < line.len() {
                    line[at] = other;
                } else {
                    line.push(other);
                }
                at += 1;
            }
        }
    }
    shown.extend(line);
    shown
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::readings::tests::{block_on, complained, said, AMachine};

    /// **`Desktop app and delivery` M13b's rewritten line, shown as a
    /// terminal shows it**: a carriage
    /// return goes back to the line's start and what follows writes over it,
    /// a shorter rewrite leaving the longer one's end; a carriage return
    /// before a line feed changes nothing; a line still being rewritten shows
    /// its newest form; nothing else is touched. Piece 2c, 2026-10-25.
    #[test]
    fn eighteenth_a_rewritten_line_is_shown_in_its_newest_form() {
        assert_eq!(as_a_terminal_shows_it("10%\r20%\r30%\n"), "30%\n");
        assert_eq!(as_a_terminal_shows_it("abcdef\rxy\n"), "xycdef\n");
        assert_eq!(as_a_terminal_shows_it("done\r\nnext\r\n"), "done\nnext\n");
        assert_eq!(as_a_terminal_shows_it("one\nReading 12%\rReading 57%"), "one\nReading 57%");
        assert_eq!(as_a_terminal_shows_it("tabs\tand ünïcode\n"), "tabs\tand ünïcode\n");
        assert_eq!(as_a_terminal_shows_it(""), "");
    }

    #[test]
    fn every_elevated_form_is_its_plain_form_behind_the_prefix() {
        // **This is what keeps the three in step**, since a command string may
        // hold only a fixed literal and neither elevated form can be built from
        // the plain one at run time.
        for step in THE_ELEVATED_STEPS.iter().chain([&THE_ELEVATED_SEARCH]) {
            assert_eq!(
                step.elevated.as_str(),
                format!("{}{}", THE_NON_INTERACTIVE_PREFIX, step.plain.as_str()),
                "{} does not pair",
                step.what_it_does
            );
            assert_eq!(
                step.elevated_reading_the_password_from_standard_input
                    .as_str(),
                format!("{}{}", THE_STANDARD_INPUT_PREFIX, step.plain.as_str()),
                "{}'s standard-input form does not pair",
                step.what_it_does
            );
        }
    }

    /// **Y1's class, counted over the whole set rather than asserted one step
    /// at a time.**
    ///
    /// **A new elevated command fails this test until somebody classifies it**,
    /// which is the point: Y1 decides the class by the act, so every act has to
    /// carry a judgement and none may be added quietly.
    /// **`Rented-server runtime` K10's path is the one constant**, and every
    /// command of the container's root step names it — and the copy names the
    /// staging token the walk writes to. E7: *"the elevated command names the
    /// token and the fixed destination"*.
    #[test]
    fn the_container_hooks_commands_name_k10s_path_and_the_staging_token() {
        use smoozy_server_protocol::placement::THE_PROVIDERS_HOOK_PATH;
        for step in [
            THE_CONTAINER_HOOKS_COPY,
            THE_CONTAINER_HOOK_MADE_READABLE,
            THE_CONTAINER_HOOKS_READ,
        ] {
            assert!(
                step.plain.as_str().ends_with(&format!(" {THE_PROVIDERS_HOOK_PATH}")),
                "{} does not name K10's path",
                step.what_it_does
            );
        }
        assert_eq!(
            THE_CONTAINER_HOOKS_COPY.plain.as_str(),
            format!("cp ~/{THE_CONTAINER_HOOKS_STAGING_TOKEN} {THE_PROVIDERS_HOOK_PATH}")
        );
        let directory = THE_PROVIDERS_HOOK_PATH
            .rsplit_once('/')
            .map(|(directory, _)| directory)
            .expect("the path has a directory");
        assert_eq!(
            THE_CONTAINER_HOOKS_DIRECTORY_MADE_REACHABLE.plain.as_str(),
            format!("chmod a+x {directory}")
        );
    }

    /// **The root step is handed its elevated forms only where elevation needs
    /// no password**, and never a password form.
    #[test]
    fn the_root_step_carries_elevated_forms_only_where_no_password_is_needed() {
        let passwordless = the_root_step_on_a_container(HowElevationIsObtained::Passwordless);
        assert_eq!(passwordless.the_copy.plain, THE_CONTAINER_HOOKS_COPY.plain);
        assert_eq!(passwordless.the_copy.elevated, Some(THE_CONTAINER_HOOKS_COPY.elevated));
        assert_eq!(
            passwordless.the_hook_made_readable,
            Some(THE_CONTAINER_HOOK_MADE_READABLE.elevated)
        );
        assert_eq!(
            passwordless.the_directory_made_traversable,
            Some(THE_CONTAINER_HOOKS_DIRECTORY_MADE_REACHABLE.elevated)
        );
        assert_eq!(passwordless.the_hook_read, Some(THE_CONTAINER_HOOKS_READ.elevated));
        assert_eq!(passwordless.the_hooks_path, "/root/onstart.sh");

        let owed = the_root_step_on_a_container(HowElevationIsObtained::APasswordIsOwedAndNoneIsHeld);
        assert_eq!(owed.the_copy.plain, THE_CONTAINER_HOOKS_COPY.plain);
        assert_eq!(owed.the_copy.elevated, None);
        assert_eq!(owed.the_hook_made_readable, None);
        assert_eq!(owed.the_directory_made_traversable, None);
        assert_eq!(owed.the_hook_read, None);
    }

    #[test]
    fn every_elevated_act_carries_y1s_class_and_the_machine_changing_ones_are_these() {
        let all: Vec<&AnElevatedStep> = THE_ELEVATED_STEPS
            .iter()
            .chain([&THE_ELEVATED_SEARCH])
            .collect();
        // **Nineteen from 2026-09-21**, when the container hook's four were added
        // (`Rented-server runtime` K11 and K19), none of them one of Y1's three.
        // **Twenty from later the same day**, when R1 item 5's Ubuntu route
        // gained its first act, the detection tool made reachable — a driver
        // act, so one of Y1's three.
        // **Twenty-one from 2026-09-23**, when the same route gained its last
        // act, the driver support binaries the check is run from — the
        // publication's own additional-components install, and a driver act
        // like the two beside it.
        // **Twenty-two from 2026-10-03**, when the same route gained a fifth
        // act, the OptiX library put on where the driver left it out
        // (`Rendering` L23's ruling) — a driver act like the four before it.
        // **Twenty-three from 2026-10-11**, when R1 item 5 gained the swap up
        // to the OptiX bar — a driver act.
        // **Twenty-four from 2026-11-03**, when R1 item 5 gained the finishing
        // of a removal that stopped part-way — a driver act.
        assert_eq!(all.len(), 24, "an elevated act was added or removed");

        let changes: Vec<&str> = all
            .iter()
            .filter(|step| step.what_it_does_to_the_machine.is_machine_changing())
            .map(|step| step.what_it_does)
            .collect();
        assert_eq!(
            changes,
            vec![
                "install the one unpack tool, where absent",
                "install the packages the mapping's first entry names, all of them at once",
                "install the distribution's own driver detection tool",
                "install the driver through the distribution's own detection route",
                "install the driver support binaries the driver check is run from",
                "install the OptiX library where the driver left it out",
                "replace a driver below the OptiX bar with Ubuntu's own newer server driver",
                "finish removing a driver a removal left part-way and install Ubuntu's own newer server driver",
                "install the package Debian's own detection tool named",
                "update the package lists",
                "put the rewritten sources file at its fixed destination",
                "configure what was unpacked and not configured",
                "fix what the interrupted transaction broke",
                "restart the machine as part of the driver step",
            ],
            "Y1's three acts are an elevated package install, a driver act and a restart"
        );

        // **The other eleven are elevated and are none of Y1's three**, and the
        // two that matter are named here because getting either wrong is a
        // wrong class: a removal is not an install (Y1's additive class names a
        // Blender removal), and a search is a reading.
        assert!(!THE_MANAGER_ROUTE_REMOVAL
            .what_it_does_to_the_machine
            .is_machine_changing());
        assert!(!THE_ELEVATED_SEARCH
            .what_it_does_to_the_machine
            .is_machine_changing());
    }

    #[test]
    fn no_elevated_command_carries_a_value_of_user_provider_or_machine_origin() {
        // **E7's rule over the whole set, in both elevated forms.** Two
        // substitutions are admissible and no third is.
        //
        // - `Server protocol and transfers` 1.1's **fourth** carriage, which
        //   derives the connection's own login on the machine and carries no
        //   name from Smoozy;
        // - its **fifth**, added 2026-09-23 at `Server Setup` E7 for R1 item
        //   5's Ubuntu route: the driver branch, read on the machine inside the
        //   command and quoted there as one word. **Since 2026-09-30 (E7's
        //   fifth-carriage bullet as corrected that day) it is read from the
        //   driver package the install left, with `dpkg-query`, and no longer
        //   asked of the detection tool** — the tool read is still named in the
        //   constant string, as `Server protocol and transfers` §1.1's fifth
        //   route requires. **Nothing crosses from Smoozy either way**, which
        //   is the whole of what E7 is about.
        //
        // The only paths named are Smoozy's own carriage tokens and fixed
        // destinations.
        let the_fifth_carriage = "$(dpkg-query -W";
        // **R1 item 5's swap takes the same carriage for its two readings,
        // added 2026-10-11** (E7's fifth-carriage bullet as extended that day):
        // the old branch, read off the driver's own version query, and the
        // running kernel's release for its headers. **Both are this one act's**
        // and the test below says so.
        let the_swaps_readings = ["$(nvidia-smi --query-gpu=driver_version", "$(uname -r)"];
        for step in THE_ELEVATED_STEPS.iter().chain([&THE_ELEVATED_SEARCH]) {
            for (text, prefix) in [
                (step.elevated.as_str(), THE_NON_INTERACTIVE_PREFIX),
                (
                    step.elevated_reading_the_password_from_standard_input
                        .as_str(),
                    THE_STANDARD_INPUT_PREFIX,
                ),
            ] {
                assert!(text.starts_with(prefix));
                for substitution in text.match_indices("$(") {
                    let rest = &text[substitution.0..];
                    let the_swaps_own = (step == &THE_UBUNTU_DRIVER_SWAP || step == &THE_UBUNTU_DRIVER_SWAPS_FINISH)
                        && the_swaps_readings.iter().any(|reading| rest.starts_with(reading));
                    assert!(
                        rest.starts_with("$(id -u)")
                            || rest.starts_with("$(id -g)")
                            || rest.starts_with(the_fifth_carriage)
                            || the_swaps_own,
                        "{text} carries a substitution that is no declared carriage"
                    );
                }
            }
        }

        // **The fifth carriage is one act's and the test says which**, so a
        // second act taking it has to come back here and be weighed rather than
        // inherit it.
        let carrying: Vec<&str> = THE_ELEVATED_STEPS
            .iter()
            .filter(|step| step.elevated.as_str().contains(the_fifth_carriage))
            .map(|step| step.what_it_does)
            .collect();
        // **Widened on purpose on 2026-10-03 to R1 item 5's fifth act**, the
        // OptiX library's package for the same branch, which E7's fifth
        // carriage bullet now names in terms.
        assert_eq!(
            carrying,
            vec![
                "install the driver support binaries the driver check is run from",
                "install the OptiX library where the driver left it out",
                // **Widened on purpose on 2026-11-03**: R1 item 5's finishing of
                // a stopped removal, named at E7's fifth-carriage bullet.
                "finish removing a driver a removal left part-way and install Ubuntu's own newer server driver",
            ],
        );

        // **The derived value is quoted as one word wherever it is used.** That
        // is what makes a machine's answer an argument rather than more command
        // — the same protection the fourth carriage's own `"$(id -un)"` has.
        for form in [
            THE_UBUNTU_DRIVER_SUPPORT_BINARIES_INSTALL.plain.as_str(),
            THE_UBUNTU_DRIVER_SUPPORT_BINARIES_INSTALL.elevated.as_str(),
            THE_UBUNTU_DRIVER_SUPPORT_BINARIES_INSTALL
                .elevated_reading_the_password_from_standard_input
                .as_str(),
        ] {
            assert!(
                form.contains(&format!("\"nvidia-utils-{the_fifth_carriage}")),
                "the derived branch is not quoted as one word: {form}"
            );
            assert!(form.ends_with(")\"'"), "the quoting does not close: {form}");
        }
        for form in [
            THE_UBUNTU_OPTIX_LIBRARY_INSTALL.plain.as_str(),
            THE_UBUNTU_OPTIX_LIBRARY_INSTALL.elevated.as_str(),
            THE_UBUNTU_OPTIX_LIBRARY_INSTALL
                .elevated_reading_the_password_from_standard_input
                .as_str(),
        ] {
            assert!(
                form.contains(&format!("\"libnvidia-gl-{the_fifth_carriage}")),
                "the derived branch is not quoted as one word: {form}"
            );
            assert!(form.ends_with(")\"'"), "the quoting does not close: {form}");
        }
    }

    /// **R1 item 5's fifth act looks by the same lookup `Rendering` L24b
    /// makes, and reads the branch exactly as the fourth act does** — both
    /// kept here as text, so neither can drift from the one it must equal.
    #[test]
    fn the_optix_act_carries_rendering_s_own_lookup_and_the_fourth_act_s_own_read() {
        let lookup = smoozy_server_setup_machine::the_optix_library::THE_LOADERS_OWN_LOOKUP;
        let the_read = |form: &'static str| -> &'static str {
            let from = form.find("$(dpkg-query").expect("the read");
            &form[from..]
        };
        for (fifth, fourth, prefix) in [
            (
                THE_UBUNTU_OPTIX_LIBRARY_INSTALL.plain.as_str(),
                THE_UBUNTU_DRIVER_SUPPORT_BINARIES_INSTALL.plain.as_str(),
                "",
            ),
            (
                THE_UBUNTU_OPTIX_LIBRARY_INSTALL.elevated.as_str(),
                THE_UBUNTU_DRIVER_SUPPORT_BINARIES_INSTALL.elevated.as_str(),
                "sudo -n ",
            ),
            (
                THE_UBUNTU_OPTIX_LIBRARY_INSTALL
                    .elevated_reading_the_password_from_standard_input
                    .as_str(),
                THE_UBUNTU_DRIVER_SUPPORT_BINARIES_INSTALL
                    .elevated_reading_the_password_from_standard_input
                    .as_str(),
                "sudo -S ",
            ),
        ] {
            // **The lookup first, whole, and the install only after `||`.**
            assert!(
                fifth.starts_with(&format!("{prefix}sh -c '{lookup} || apt-get ")),
                "{fifth}"
            );
            // **S7b's bounded wait rides it like every other apt install.**
            assert!(fifth.contains("apt-get -o DPkg::Lock::Timeout=120 install -y "));
            assert_eq!(the_read(fifth), the_read(fourth));
        }
    }

    /// **The fifth act run in a real shell, over a stand-in loader listing and
    /// stand-in `dpkg-query` and `apt-get`**, so what it installs, and whether
    /// it installs anything, is the machine's own shell's answer.
    #[cfg(unix)]
    #[test]
    fn the_optix_act_installs_the_branch_s_library_only_where_the_loader_does_not_find_it() {
        use std::os::unix::fs::PermissionsExt;

        let here = std::env::temp_dir().join(format!(
            "smoozy-optix-act-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        std::fs::create_dir_all(&here).expect("a folder for the stand-ins");
        let stand_in = |name: &str, body: &str| {
            let path = here.join(name);
            std::fs::write(&path, format!("#!/bin/sh\n{body}\n")).expect("a stand-in");
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755))
                .expect("the stand-in made runnable");
        };
        // **The listed program**: what glibc's loader prints when it is asked
        // to trace one (`smoozy_server_setup_machine::the_optix_library`'s
        // head), and nothing at all where it is run without the loader's two
        // settings. **A shell function named `sh`, defined ahead of the act's
        // text, and never a program**: on Linux the two settings reach any
        // program `sh` names and the system's loader lists its interpreter
        // instead of running it, so a stand-in program never ran there and the
        // found case installed (hosted run `37188697561`). The function starts
        // no program and uses only the `[` and `printf` built into `dash` and
        // `bash`.
        let the_listed_program = "sh() {\n\
             [ -n \"$LD_TRACE_LOADED_OBJECTS\" ] && [ \"$LD_PRELOAD\" = libnvoptix.so.1 ] || exit 9\n\
             printf '%s' \"$LISTING\"\nprintf 'ERROR: ld.so: complaint\\n' >&2\n}\n";
        stand_in("dpkg-query", "cat \"$HERE/standing\"");
        stand_in(
            "apt-get",
            "for one in \"$@\"; do last=\"$one\"; done\nprintf '%s\\n' \"$last\"\nexit \"$(cat \"$HERE/apt\")\"",
        );

        let the_act_over = |listing: &str, standing: &str, apt: i32| -> (Option<i32>, String) {
            std::fs::write(here.join("standing"), standing).expect("the standing packages");
            std::fs::write(here.join("apt"), apt.to_string()).expect("apt's answer");
            let path = format!(
                "{}:{}",
                here.display(),
                std::env::var("PATH").unwrap_or_default()
            );
            // **The outer shell is the system's own**; only the `sh` the
            // lookup names is the stand-in, the function ahead of the text.
            // **The act's own text inside its `sh -c '…'`**: the `sh` the act
            // starts with is the machine's own, and here it would be the
            // stand-in, so the text is handed to the system's shell directly.
            let inside = THE_UBUNTU_OPTIX_LIBRARY_INSTALL
                .plain
                .as_str()
                .strip_prefix("sh -c '")
                .and_then(|rest| rest.strip_suffix('\''))
                .expect("the act is one shell line in single quotes");
            let ran = std::process::Command::new("/bin/sh")
                .arg("-c")
                .arg(format!("{the_listed_program}{inside}"))
                .env("PATH", path)
                .env("HERE", &here)
                .env("LISTING", listing)
                .output()
                .expect("the shell ran");
            (
                ran.status.code(),
                String::from_utf8_lossy(&ran.stdout).trim().to_string(),
            )
        };
        let found = "\tlinux-vdso.so.1 (0x00007ffd)\n\
                     \tlibnvoptix.so.1 => /usr/lib/x86_64-linux-gnu/libnvoptix.so.1 (0x00007f00)\n\
                     \tlibc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007f01)\n";
        let missing = "\tlinux-vdso.so.1 (0x00007ffd)\n\
                       \tlibc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007f01)\n";
        let standing = "ii nvidia-headless-no-dkms-580-server-open\n";

        // **Found: nothing is installed and the act ends clean.**
        assert_eq!(the_act_over(found, standing, 0), (Some(0), String::new()));
        // **Not found: the branch's own `libnvidia-gl-…` is installed.**
        assert_eq!(
            the_act_over(missing, standing, 0),
            (Some(0), "libnvidia-gl-580-server".to_string())
        );
        assert_eq!(
            the_act_over(missing, "ii nvidia-headless-no-dkms-595-server-open\n", 0).1,
            "libnvidia-gl-595-server"
        );
        // **A refused install ends the act in apt's own status**, which ends
        // the step.
        assert_eq!(
            the_act_over(missing, standing, 100),
            (Some(100), "libnvidia-gl-580-server".to_string())
        );
        // **No branch read composes no package**, and apt says so itself.
        assert_eq!(the_act_over(missing, "", 100).1, "libnvidia-gl-");

        let _ = std::fs::remove_dir_all(&here);
    }

    /// **R1 item 5's swap run in a real shell, over stand-ins for the driver's
    /// query, the loaded module's version file, `dpkg-query`, `apt-get`,
    /// `apt` and `uname`**, so which packages it removes and installs, and
    /// whether it removes anything at all, is the shell's own answer. Added
    /// 2026-10-11 — the fabric manager's two arms among them.
    #[cfg(unix)]
    #[test]
    fn the_swap_keeps_the_flavour_brings_the_fabric_manager_and_removes_nothing_it_cannot_replace() {
        use std::os::unix::fs::PermissionsExt;

        let here = std::env::temp_dir().join(format!(
            "smoozy-driver-swap-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        std::fs::create_dir_all(&here).expect("a folder for the stand-ins");
        let stand_in = |name: &str, body: &str| {
            let path = here.join(name);
            std::fs::write(&path, format!("#!/bin/sh\n{body}\n")).expect("a stand-in");
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755))
                .expect("the stand-in made runnable");
        };
        stand_in("nvidia-smi", "cat \"$HERE/version\"");
        stand_in(
            "dpkg-query",
            "[ -s \"$HERE/standing\" ] || exit 1\ncat \"$HERE/standing\"",
        );
        stand_in(
            "apt-get",
            "echo \"apt-get $*\" >> \"$HERE/log\"\n\
             case \" $* \" in *\" -s \"*) exit \"$(cat \"$HERE/trial\")\" ;; esac\nexit 0",
        );
        stand_in(
            "apt",
            "echo \"apt $*\" >> \"$HERE/log\"\n\
             case \" $* \" in *\" -s \"*) exit \"$(cat \"$HERE/removal_trial\")\" ;; esac\n\
             exit \"$(cat \"$HERE/removal\")\"",
        );
        stand_in("uname", "echo 6.14.0-37-generic");
        // **The loaded module's version file is the system's own path**, so it
        // is answered by a shell function ahead of the act's text, the way the
        // OptiX act's test answers `sh`; every other `grep` is the real one.
        let the_version_file = "grep() {\n\
             if [ \"$3\" = /proc/driver/nvidia/version ]; then command grep -q \"$2\" \"$HERE/module\"; \
             else command grep \"$@\"; fi\n}\n";

        struct Machine<'a> {
            version: &'a str,
            module: &'a str,
            standing: &'a str,
            trial: i32,
            removal_trial: i32,
            removal: i32,
        }
        let the_swap_on = |m: Machine| -> (Option<i32>, Vec<String>) {
            for (name, text) in [
                ("version", m.version.to_string()),
                ("module", m.module.to_string()),
                ("standing", m.standing.to_string()),
                ("trial", m.trial.to_string()),
                ("removal_trial", m.removal_trial.to_string()),
                ("removal", m.removal.to_string()),
                ("log", String::new()),
            ] {
                std::fs::write(here.join(name), text).expect("the stand-in's answer");
            }
            let path = format!(
                "{}:{}",
                here.display(),
                std::env::var("PATH").unwrap_or_default()
            );
            let inside = THE_UBUNTU_DRIVER_SWAP
                .plain
                .as_str()
                .strip_prefix("sh -c '")
                .and_then(|rest| rest.strip_suffix('\''))
                .expect("the act is one shell line in single quotes");
            let ran = std::process::Command::new("/bin/sh")
                .arg("-c")
                .arg(format!("{the_version_file}{inside}"))
                .env("PATH", path)
                .env("HERE", &here)
                .output()
                .expect("the shell ran");
            let log = std::fs::read_to_string(here.join("log")).unwrap_or_default();
            (ran.status.code(), log.lines().map(str::to_string).collect())
        };
        let open = "NVRM version: NVIDIA UNIX Open Kernel Module for x86_64  550.163.01  Release Build\n";
        let proprietary = "NVRM version: NVIDIA UNIX x86_64 Kernel Module  550.163.01  Release Build\n";
        let lock = "-o DPkg::Lock::Timeout=900";

        // **550 on the open modules, no fabric manager: refreshed, tried,
        // the old branch removed, the open 580 server driver put on.**
        let (status, log) = the_swap_on(Machine {
            version: "550.163.01\n550.163.01\n",
            module: open,
            standing: "",
            trial: 0,
            removal_trial: 0,
            removal: 0,
        });
        assert_eq!(status, Some(0));
        assert_eq!(
            log,
            vec![
                format!("apt-get {lock} update"),
                format!("apt-get {lock} -s install nvidia-driver-580-server-open linux-headers-6.14.0-37-generic"),
                // **`Server Setup` R1 item 5's removal, as written 2026-11-03**:
                // tried as well, and carrying the install's own option. The
                // Sesterce pass's image stopped here half done on a file two
                // packages claim.
                format!("apt {lock} -s --purge remove -y *nvidia*550*"),
                format!("apt {lock} -o Dpkg::Options::=--force-overwrite --purge remove -y *nvidia*550*"),
                format!(
                    "apt-get {lock} -o Dpkg::Options::=--force-overwrite install -y --no-install-recommends \
                     nvidia-driver-580-server-open linux-headers-6.14.0-37-generic"
                ),
            ]
        );

        // **A removal apt's resolver refuses leaves the old driver standing**:
        // tried, and nothing touched.
        let (status, log) = the_swap_on(Machine {
            version: "550.163.01\n",
            module: open,
            standing: "",
            trial: 0,
            removal_trial: 100,
            removal: 0,
        });
        assert_eq!(status, Some(100));
        assert_eq!(log.len(), 3, "refreshed and both tried, and nothing else: {log:?}");
        assert!(log.iter().all(|line| !line.contains("install -y") && !line.contains("force-overwrite")), "{log:?}");

        // **The proprietary flavour is kept where it is the one loaded.**
        let (status, log) = the_swap_on(Machine {
            version: "570.86.10\n",
            module: proprietary,
            standing: "",
            trial: 0,
            removal_trial: 0,
            removal: 0,
        });
        assert_eq!(status, Some(0));
        assert_eq!(log[3], format!("apt {lock} -o Dpkg::Options::=--force-overwrite --purge remove -y *nvidia*570*"));
        assert!(log[4].ends_with(" nvidia-driver-580-server linux-headers-6.14.0-37-generic"));

        // **A fabric manager stands, and the archive has the new one: it is
        // tried, and goes on with the driver.**
        let (status, log) = the_swap_on(Machine {
            version: "550.163.01\n",
            module: open,
            standing: "ii nvidia-fabricmanager-550\n",
            trial: 0,
            removal_trial: 0,
            removal: 0,
        });
        assert_eq!(status, Some(0));
        assert!(log[1].ends_with(" nvidia-fabricmanager-580 libnvidia-nscq-580"), "{log:?}");
        assert!(log[4].ends_with(" nvidia-fabricmanager-580 libnvidia-nscq-580"), "{log:?}");

        // **A fabric manager stands, and the archive has no new one for this
        // release: the trial fails and nothing is removed.**
        let (status, log) = the_swap_on(Machine {
            version: "550.163.01\n",
            module: open,
            standing: "ii nvidia-fabricmanager-550\n",
            trial: 100,
            removal_trial: 0,
            removal: 0,
        });
        assert_eq!(status, Some(100));
        assert_eq!(log.len(), 2, "refreshed and tried, and nothing else: {log:?}");
        assert!(log[1].contains("nvidia-fabricmanager-580"));

        // **A fabric manager removed earlier with its settings left (`rc`)
        // does not stand**, and none is put on.
        let (_, log) = the_swap_on(Machine {
            version: "550.163.01\n",
            module: open,
            standing: "rc nvidia-fabricmanager-550\n",
            trial: 0,
            removal_trial: 0,
            removal: 0,
        });
        assert!(!log[1].contains("fabricmanager"), "{log:?}");

        // **No headers for the running kernel, or any package missing: the
        // old driver is untouched.**
        let (status, log) = the_swap_on(Machine {
            version: "550.163.01\n",
            module: open,
            standing: "",
            trial: 100,
            removal_trial: 0,
            removal: 0,
        });
        assert_eq!(status, Some(100));
        assert!(log.iter().all(|line| !line.contains("remove") && !line.contains("install -y")));

        // **A removal that fails puts nothing on.**
        let (status, log) = the_swap_on(Machine {
            version: "550.163.01\n",
            module: open,
            standing: "",
            trial: 0,
            removal_trial: 0,
            removal: 100,
        });
        assert_eq!(status, Some(100));
        assert_eq!(log.len(), 4);

        // **A version that is not a three-digit branch, or no flavour read,
        // touches nothing at all** — not even the lists.
        for (version, module) in [("[N/A]\n", open), ("", open), ("1000.1\n", open), ("550.163.01\n", "")] {
            let (status, log) = the_swap_on(Machine {
                version,
                module,
                standing: "",
                trial: 0,
                removal_trial: 0,
                removal: 0,
            });
            assert_ne!(status, Some(0), "{version:?} {module:?}");
            assert!(log.is_empty(), "{version:?} {module:?}: {log:?}");
        }

        let _ = std::fs::remove_dir_all(&here);
    }

    /// **`Server Setup` R1 item 5: the reading and the finishing act, run by a
    /// real shell over stand-ins for the package tools**, on the packages the
    /// Sesterce pass's machine was left with.
    #[test]
    #[cfg(unix)]
    fn a_stopped_removal_is_read_from_the_packages_and_finished_with_no_driver_to_ask() {
        use std::os::unix::fs::PermissionsExt;
        let here = std::env::temp_dir().join(format!(
            "smoozy-driver-finish-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        std::fs::create_dir_all(&here).expect("a folder for the stand-ins");
        let stand_in = |name: &str, body: &str| {
            let path = here.join(name);
            std::fs::write(&path, format!("#!/bin/sh\n{body}\n")).expect("a stand-in");
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755))
                .expect("the stand-in made runnable");
        };
        // **`dpkg-query` answers only for the patterns it was given**, each a
        // shell pattern over the machine's list, printing what `-f` asked for.
        stand_in(
            "dpkg-query",
            "f=\"$3\"; shift 3; for p in \"$@\"; do while read -r s n; do case \"$n\" in $p) \
             case \"$f\" in *Package*) printf \"%-3s%s\\n\" \"$s\" \"$n\" ;; *) printf \"%-3s\\n\" \"$s\" ;; esac ;; esac; \
             done < \"$HERE/packages\"; done",
        );
        stand_in(
            "apt-get",
            "echo \"apt-get $*\" >> \"$HERE/log\"\ncase \" $* \" in *\" -s \"*) exit \"$(cat \"$HERE/trial\")\" ;; esac\nexit 0",
        );
        stand_in("apt", "echo \"apt $*\" >> \"$HERE/log\"\ncase \" $* \" in *\" -s \"*) exit 0 ;; esac\nexit \"$(cat \"$HERE/removal\")\"");
        stand_in("uname", "echo 6.8.0-90-generic");
        // **No `nvidia-smi` on this path at all**: the fault's own state.
        let run = |command: &str, packages: &str, trial: i32, removal: i32| -> (Option<i32>, String, Vec<String>) {
            std::fs::write(here.join("packages"), packages).expect("the packages");
            std::fs::write(here.join("trial"), trial.to_string()).expect("the trial");
            std::fs::write(here.join("removal"), removal.to_string()).expect("the removal");
            std::fs::write(here.join("log"), "").expect("the log");
            let inside = command
                .strip_prefix("sh -c '")
                .and_then(|rest| rest.strip_suffix('\''))
                .expect("one shell line in single quotes");
            let ran = std::process::Command::new("/bin/sh")
                .arg("-c")
                .arg(inside)
                .env("PATH", format!("{}:/usr/bin:/bin", here.display()))
                .env("HERE", &here)
                .output()
                .expect("the shell ran");
            let log = std::fs::read_to_string(here.join("log")).unwrap_or_default();
            (
                ran.status.code(),
                String::from_utf8_lossy(&ran.stdout).into_owned(),
                log.lines().map(str::to_string).collect(),
            )
        };
        let reading = THE_DRIVER_PACKAGES_READING.as_str();
        let finish = THE_UBUNTU_DRIVER_SWAPS_FINISH.plain.as_str();
        let says = |output: &str| crate::readings::what_the_driver_packages_say(output);
        use crate::readings::WhatTheDriverPackagesSay as Say;

        // **What the pass's machine held after the removal stopped**, from its
        // Server output and the account: the driver package gone, the kernel
        // half and the fabric manager standing.
        let the_pass = "ii nvidia-dkms-570-open\nii nvidia-kernel-source-570-open\nii nvidia-kernel-common-570\n\
                        ii nvidia-fabricmanager-570\nii libnvidia-cfg1-570\nrc libnvidia-compute-570\n";
        let (status, output, _) = run(reading, the_pass, 0, 0);
        assert_eq!(status, Some(0));
        assert_eq!(says(&output), Say::AKernelHalfIsLeftBehind, "{output:?}");
        let lock = "-o DPkg::Lock::Timeout=900";
        let (status, _, log) = run(finish, the_pass, 0, 0);
        assert_eq!(status, Some(0));
        assert_eq!(
            log,
            vec![
                format!("apt-get {lock} update"),
                format!("apt-get {lock} -s install nvidia-driver-580-server-open linux-headers-6.8.0-90-generic nvidia-fabricmanager-580 libnvidia-nscq-580"),
                format!("apt {lock} -s --purge remove -y *nvidia*570*"),
                format!("apt {lock} -o Dpkg::Options::=--force-overwrite --purge remove -y *nvidia*570*"),
                format!("apt-get {lock} -o Dpkg::Options::=--force-overwrite install -y --no-install-recommends nvidia-driver-580-server-open linux-headers-6.8.0-90-generic nvidia-fabricmanager-580 libnvidia-nscq-580"),
            ]
        );

        // **The proprietary flavour, read from the names.**
        let (_, _, log) = run(finish, "ii nvidia-dkms-550\n", 0, 0);
        assert!(log[1].contains(" nvidia-driver-580-server linux-headers"), "{log:?}");

        // **An install that failed after a whole removal reads the same way**:
        // the 580 kernel half left half-configured, and no driver package
        // installed, is finished under its own branch.
        let half = "iF nvidia-dkms-580-server-open\niU nvidia-driver-580-server-open\n";
        let (_, output, _) = run(reading, half, 0, 0);
        assert_eq!(says(&output), Say::AKernelHalfIsLeftBehind, "{output:?}");
        let (_, _, log) = run(finish, half, 0, 0);
        assert!(log[2].ends_with("*nvidia*580*"), "{log:?}");

        // **A trial or a removal that fails ends the act there, and no install
        // goes out.**
        let (status, _, log) = run(finish, the_pass, 100, 0);
        assert_eq!(status, Some(100));
        assert_eq!(log.len(), 2, "{log:?}");
        let (status, _, log) = run(finish, the_pass, 0, 100);
        assert_eq!(status, Some(100));
        assert!(log.iter().all(|line| !line.contains("install -y")), "{log:?}");

        // **Nothing left behind: the act touches nothing, not even the lists.**
        let (status, _, log) = run(finish, "rc nvidia-dkms-570-open\nii libnvidia-cfg1\n", 0, 0);
        assert_eq!(status, Some(3));
        assert!(log.is_empty(), "{log:?}");

        // **A driver package standing is the other answer**, on either route's
        // name, and a kernel half beside it does not change it.
        for stands in [
            "ii nvidia-driver-570-open\nii nvidia-dkms-570-open\n",
            "ii nvidia-headless-no-dkms-595-server-open\n",
        ] {
            let (_, output, _) = run(reading, stands, 0, 0);
            assert_eq!(says(&output), Say::ADriverPackageStands, "{stands}: {output:?}");
        }
        // **A package whose name starts the same and carries no branch is no
        // driver**, which is why the patterns name a digit.
        let (_, output, _) = run(reading, "ii nvidia-driver-assistant\nii libnvidia-container1\n", 0, 0);
        assert_eq!(says(&output), Say::NoDriverPackage, "{output:?}");
        let (_, output, _) = run(reading, "", 0, 0);
        assert_eq!(says(&output), Say::NoDriverPackage, "{output:?}");

        let _ = std::fs::remove_dir_all(&here);
    }

    /// **B11's retry is the same search**, and both leave directories out
    /// and keep links. Step 10c's sixth pass on Vast found the search offering
    /// a thumbnail cache folder named `blender` as a Blender that would not
    /// answer.
    #[test]
    fn the_elevated_search_is_the_plain_search_and_both_leave_directories_out() {
        let plain = crate::readings::THE_SEARCH.as_str();
        assert!(plain.contains("! -type d"), "{plain}");
        assert!(!plain.contains("-type f"), "a link to a build must stay a candidate");
        assert_eq!(
            THE_ELEVATED_SEARCH.elevated.as_str(),
            format!("sudo -n {plain}")
        );
        assert_eq!(
            THE_ELEVATED_SEARCH
                .elevated_reading_the_password_from_standard_input
                .as_str(),
            format!("sudo -S {plain}")
        );
    }

    /// **The support-binaries command, run in a real shell over stand-in
    /// `dpkg-query` and `apt-get`, so the package name apt is asked for is the
    /// one the machine's own shell would compose.** Changed 2026-09-30,
    /// eleventh-pass fix piece 10: the act reads the branch of the driver
    /// package the install left (`Server Setup` R1 item 5 as corrected that
    /// day) and no longer asks the recommended list.
    ///
    /// **Machine 1's case is the first one**: the second run's install left
    /// the 580 package standing and the 595 one removed with its configuration
    /// kept (`rc`), and the utilities must follow 580. With the old read they
    /// followed the list's 595.
    #[cfg(unix)]
    #[test]
    fn the_support_binaries_follow_the_driver_package_the_install_left() {
        use std::os::unix::fs::PermissionsExt;

        let form = THE_UBUNTU_DRIVER_SUPPORT_BINARIES_INSTALL.elevated.as_str();
        assert!(form.starts_with("sudo -n sh -c 'apt-get "));
        // **S7b's bounded wait rides it like every other apt install.**
        assert!(form.contains("-o DPkg::Lock::Timeout=120"));
        assert!(
            !form.contains("ubuntu-drivers list"),
            "the act still asks the recommended list, which is not what the install chose"
        );

        let here = std::env::temp_dir().join(format!(
            "smoozy-support-binaries-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        std::fs::create_dir_all(&here).expect("a folder for the stand-ins");
        let stand_in = |name: &str, body: &str| {
            let path = here.join(name);
            std::fs::write(&path, format!("#!/bin/sh\n{body}\n")).expect("a stand-in");
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755))
                .expect("the stand-in made runnable");
        };
        stand_in(
            "dpkg-query",
            "for one in \"$@\"; do printf '%s\\n' \"$one\" >> \"$HERE/asked\"; done\ncat \"$HERE/standing\"",
        );
        stand_in("apt-get", "for one in \"$@\"; do last=\"$one\"; done\nprintf '%s\\n' \"$last\"");

        let the_package_asked_for = |standing: &str| -> (String, Vec<String>) {
            std::fs::write(here.join("standing"), standing).expect("the standing packages");
            let _ = std::fs::remove_file(here.join("asked"));
            let path = format!(
                "{}:{}",
                here.display(),
                std::env::var("PATH").unwrap_or_default()
            );
            let ran = std::process::Command::new("sh")
                .arg("-c")
                .arg(THE_UBUNTU_DRIVER_SUPPORT_BINARIES_INSTALL.plain.as_str())
                .env("PATH", path)
                .env("HERE", &here)
                .output()
                .expect("the shell ran");
            let asked = std::fs::read_to_string(here.join("asked"))
                .unwrap_or_default()
                .lines()
                .map(str::to_string)
                .collect();
            (String::from_utf8_lossy(&ran.stdout).trim().to_string(), asked)
        };

        // **Machine 1, second run**: 595 removed with its configuration kept,
        // 580 standing.
        let (package, asked) = the_package_asked_for(
            "rc nvidia-headless-no-dkms-595-server-open\nii nvidia-headless-no-dkms-580-server-open\n",
        );
        assert_eq!(package, "nvidia-utils-580-server");
        // **What `dpkg-query` itself received**: the pattern whole, never
        // expanded by the shell, and the format with the shell's escapes gone.
        assert_eq!(
            asked,
            vec![
                "-W".to_string(),
                "-f".to_string(),
                "${db:Status-Abbrev}${Package}\\n".to_string(),
                "nvidia-headless-no-dkms-*".to_string(),
            ]
        );
        // **Every pass before the eleventh**: one branch, the one installed.
        assert_eq!(
            the_package_asked_for("ii nvidia-headless-no-dkms-595-server-open\n").0,
            "nvidia-utils-595-server"
        );
        // **A flavour with no `-server`** keeps its number alone.
        assert_eq!(
            the_package_asked_for("ii nvidia-headless-no-dkms-570\n").0,
            "nvidia-utils-570"
        );
        // **Nothing standing composes no package**, and apt answers in its own
        // words that there is no such package — the honest failure this act
        // ends on rather than a silent half-install.
        assert_eq!(the_package_asked_for("").0, "nvidia-utils-");
        assert_eq!(
            the_package_asked_for("rc nvidia-headless-no-dkms-595-server-open\n").0,
            "nvidia-utils-"
        );

        let _ = std::fs::remove_dir_all(&here);
    }

    /// **The command's package names are the entry's own, in the entry's own
    /// order, and nothing else is installed.**
    ///
    /// **The two are written out twice and the fourth pass is why this guard
    /// exists**: the entry and the command string are the same list kept in two
    /// places, and a name added to one and not the other is a step that either
    /// refuses a library it can place or installs a package no entry names.
    #[test]
    fn the_packages_this_step_names_are_the_entrys_own() {
        let entry = crate::ready_action::THE_MAPPING[0].packages;
        for form in [
            THE_LIBRARIES_INSTALL_FOR_THE_FIRST_ENTRY.plain.as_str(),
            THE_LIBRARIES_INSTALL_FOR_THE_FIRST_ENTRY.elevated.as_str(),
            THE_LIBRARIES_INSTALL_FOR_THE_FIRST_ENTRY
                .elevated_reading_the_password_from_standard_input
                .as_str(),
        ] {
            let named: Vec<&str> = form
                .split_whitespace()
                .skip_while(|word| *word != "-y")
                .skip(1)
                .collect();
            assert_eq!(
                named,
                entry.to_vec(),
                "the command's packages are not the entry's own: {form}"
            );
        }
    }

    #[test]
    fn the_libraries_install_selects_a_constant_and_never_composes_one() {
        use crate::ready_action::{
            what_to_install_for, WhatTheLibrariesStepInstalls, WHAT_A_BARE_CONTAINER_REPORTED,
        };
        let all_ten = what_to_install_for(&["libX11.so.6".to_string()]);
        assert!(matches!(
            all_ten,
            WhatTheLibrariesStepInstalls::ThesePackages(_)
        ));
        assert_eq!(
            the_libraries_install(&all_ten),
            Some(THE_LIBRARIES_INSTALL_FOR_THE_FIRST_ENTRY)
        );

        // **The whole list a bare container reports selects the same one
        // command**, which is the case the fourth pass on Vast met and which
        // nothing here drove before 2026-09-22.
        let the_bare_machines: Vec<String> = WHAT_A_BARE_CONTAINER_REPORTED
            .iter()
            .map(|library| library.to_string())
            .collect();
        let whole = what_to_install_for(&the_bare_machines);
        assert_eq!(
            the_libraries_install(&whole),
            Some(THE_LIBRARIES_INSTALL_FOR_THE_FIRST_ENTRY),
            "the nine a bare container reports do not select the one command: {whole:?}"
        );

        // **A library the mapping does not name installs nothing**, which is
        // R1 item 4's *never a partial install*.
        let unnamed = what_to_install_for(&["libsomething.so.9".to_string()]);
        assert_eq!(the_libraries_install(&unnamed), None);

        // Nothing missing is a step with no act rather than a step that failed.
        let nothing = what_to_install_for(&[]);
        assert_eq!(the_libraries_install(&nothing), None);
    }

    #[test]
    fn the_passwordless_form_runs_the_step_and_keeps_the_machines_own_words() {
        let mut machine = AMachine::answering(vec![
            (THE_LIST_UPDATE.elevated.as_str(), said("Reading package lists...", 0)),
            (
                THE_TOOLING_INSTALL.elevated.as_str(),
                said("Setting up xz-utils", 0),
            ),
        ]);
        let said_it = block_on(issue(
            &mut machine,
            THE_TOOLING_INSTALL,
            HowElevationIsObtained::Passwordless,
        ))
        .expect("the passwordless form runs");
        assert!(said_it.it_ended_cleanly);
        // **A step that ended cleanly met a successful operation**, which is the
        // value that landed 2026-09-04; before it, every successful elevated
        // command in the application was labelled a failed operation.
        assert_eq!(
            said_it.what_it_met,
            WhatTheStepMet::ASuccessfulOperationAfterASuccessfulValidate
        );
        // **Both readers answer exactly what they answered before**, which is
        // what makes this a repair of the record rather than a change.
        assert!(!said_it.what_it_met.erases_the_remembered_password());
        assert!(said_it.what_it_met.the_password_is_kept());
        // **No password was asked for, held or sent**, so nothing rode the
        // second carriage.
        assert!(machine.carried_on_standard_input.is_empty());
    }

    #[test]
    fn a_failed_passwordless_step_forgets_nothing() {
        // **E4's third bullet**: a failed operation after a successful validate
        // is a failed operation, and it forgets nothing. Passwordless elevation
        // has no validate to fail, so every failure here is that bullet's.
        let mut machine = AMachine::answering(vec![
            (THE_LIST_UPDATE.elevated.as_str(), said("", 0)),
            (
                THE_TOOLING_INSTALL.elevated.as_str(),
                complained("E: Unable to locate package", 100),
            ),
        ]);
        let said_it = block_on(issue(
            &mut machine,
            THE_TOOLING_INSTALL,
            HowElevationIsObtained::Passwordless,
        ))
        .expect("the command was issued");
        assert!(!said_it.it_ended_cleanly);
        assert_eq!(said_it.the_machines_own_words, "E: Unable to locate package");
        assert!(!said_it.what_it_met.erases_the_remembered_password());
        assert!(said_it.what_it_met.the_password_is_kept());
    }

    #[test]
    fn a_step_owing_a_password_with_none_held_sends_nothing() {
        let mut machine = AMachine::default();
        let outcome = block_on(issue(
            &mut machine,
            THE_RESTART,
            HowElevationIsObtained::APasswordIsOwedAndNoneIsHeld,
        ));
        assert_eq!(outcome, Err(WhyTheStepDidNotRun::NoPasswordIsHeldYet));
        // **Nothing was sent.** A step that has no elevation to obtain must not
        // reach the machine at all, or it would run unelevated and fail for a
        // reason nobody could tell from a refusal.
        assert!(machine.asked.is_empty());
        assert!(machine.carried_on_standard_input.is_empty());
    }

    #[test]
    fn the_probe_decides_which_form_a_step_takes() {
        assert_eq!(
            HowElevationIsObtained::from(Probe::Succeeded),
            HowElevationIsObtained::Passwordless
        );
        assert_eq!(
            HowElevationIsObtained::from(Probe::Failed),
            HowElevationIsObtained::APasswordIsOwedAndNoneIsHeld
        );
    }

    // ------------------------------------------- E3's route, now that it exists

    /// **E3 and E4 together, performed.** The validate goes first, the step
    /// follows, both carry the password on standard input, and **neither
    /// command string holds any part of it.**
    #[test]
    fn the_password_form_validates_first_and_then_runs_the_step() {
        let password = Origin::what_the_user_typed("hunter2");
        let mut machine = AMachine::answering(vec![
            (THE_VALIDATE_FORM.as_str(), said("", 0)),
            (
                THE_LIST_UPDATE
                    .elevated_reading_the_password_from_standard_input
                    .as_str(),
                said("Reading package lists...", 0),
            ),
            (
                THE_TOOLING_INSTALL
                    .elevated_reading_the_password_from_standard_input
                    .as_str(),
                said("Setting up xz-utils", 0),
            ),
        ]);
        let said_it = block_on(issue(
            &mut machine,
            THE_TOOLING_INSTALL,
            HowElevationIsObtained::APasswordIsHeld(&password),
        ))
        .expect("the password form runs");

        assert!(said_it.it_ended_cleanly);
        assert_eq!(
            said_it.what_it_met,
            WhatTheStepMet::ASuccessfulOperationAfterASuccessfulValidate
        );
        assert!(!said_it.what_it_met.erases_the_remembered_password());
        assert!(said_it.what_it_met.the_password_is_kept());

        // **The order is the clause.** Validate first, operation second — and
        // **S7d's refresh is an operation of its own in front of the install**,
        // validated the same way.
        assert_eq!(
            machine.asked,
            vec![
                THE_VALIDATE_FORM.as_str(),
                THE_LIST_UPDATE
                    .elevated_reading_the_password_from_standard_input
                    .as_str(),
                THE_VALIDATE_FORM.as_str(),
                THE_TOOLING_INSTALL
                    .elevated_reading_the_password_from_standard_input
                    .as_str(),
            ]
        );
        // **Twice per operation, because the credential cache is not relied
        // on** — E4's own limit, and
        // [`crate::elevation::the_credential_cache_is_relied_on`] says so.
        assert_eq!(machine.carried_on_standard_input.len(), 4);
        for (command, carried) in &machine.carried_on_standard_input {
            assert_eq!(carried, b"hunter2");
            assert!(
                !command.contains("hunter2"),
                "{command} carries the password in the command string"
            );
        }
        password.the_act_ended();
    }

    /// **E4's whole point: a failed validate is never an operation's failure.**
    /// The step is not attempted at all, and the wrong-password ending — matched
    /// as an exact constant by `Security and licence` §13 rule 5's exception —
    /// is what erases the remembered password.
    #[test]
    fn a_wrong_password_stops_before_the_step_and_erases_the_remembered_one() {
        let password = Origin::what_the_user_typed("wrong");
        let mut machine = AMachine::answering(vec![(
            THE_VALIDATE_FORM.as_str(),
            complained("sudo: 1 incorrect password attempt", 1),
        )]);
        let said_it = block_on(issue(
            &mut machine,
            THE_TOOLING_INSTALL,
            HowElevationIsObtained::APasswordIsHeld(&password),
        ))
        .expect("the validate was issued");

        assert!(!said_it.it_ended_cleanly);
        assert_eq!(said_it.what_it_met, WhatTheStepMet::AWrongPassword);
        assert!(said_it.what_it_met.erases_the_remembered_password());
        // **The operation was never attempted.** Only the validate was asked.
        assert_eq!(machine.asked, vec![THE_VALIDATE_FORM.as_str()]);
        assert_eq!(machine.carried_on_standard_input.len(), 1);
        password.the_act_ended();
    }

    /// **The refusal that names the host is never matched**, so nothing is
    /// established about the password and it is kept — `Security and licence`
    /// §4's invariant in its withholding direction, and W1's *Refused*.
    #[test]
    fn a_refusal_that_is_not_the_constant_keeps_the_password_and_runs_nothing() {
        let password = Origin::what_the_user_typed("right");
        let mut machine = AMachine::answering(vec![(
            THE_VALIDATE_FORM.as_str(),
            complained(
                "Sorry, user smoozynoadmin may not run sudo on a0e7a496ff47.",
                1,
            ),
        )]);
        let said_it = block_on(issue(
            &mut machine,
            THE_TOOLING_INSTALL,
            HowElevationIsObtained::APasswordIsHeld(&password),
        ))
        .expect("the validate was issued");

        assert_eq!(
            said_it.what_it_met,
            WhatTheStepMet::ARefusalNotEstablishedToBeAWrongPassword
        );
        assert!(said_it.what_it_met.the_password_is_kept());
        assert!(!said_it.what_it_met.erases_the_remembered_password());
        // **The machine's own words ride it, whole.**
        assert_eq!(
            said_it.the_machines_own_words,
            "Sorry, user smoozynoadmin may not run sudo on a0e7a496ff47."
        );
        assert_eq!(machine.asked, vec![THE_VALIDATE_FORM.as_str()]);
        password.the_act_ended();
    }

    /// **E6, swept over every step rather than over the one an earlier test
    /// happened to use**: the password rides the carriage and never a command
    /// string, whichever step is issued.
    ///
    /// Every step of the set is run with a password held, and what the machine
    /// was asked is checked against what it was given.
    #[test]
    fn no_step_of_the_whole_set_puts_the_password_in_a_command_string() {
        let password = Origin::what_the_user_typed("a-secret-nobody-may-see");
        for step in THE_ELEVATED_STEPS.iter().chain([&THE_ELEVATED_SEARCH]) {
            let mut machine = AMachine::answering(vec![
                (THE_VALIDATE_FORM.as_str(), said("", 0)),
                (
                    THE_LIST_UPDATE
                        .elevated_reading_the_password_from_standard_input
                        .as_str(),
                    said("", 0),
                ),
                (
                    step.elevated_reading_the_password_from_standard_input
                        .as_str(),
                    said("", 0),
                ),
            ]);
            block_on(issue(
                &mut machine,
                *step,
                HowElevationIsObtained::APasswordIsHeld(&password),
            ))
            .unwrap_or_else(|_| panic!("{} was issued", step.what_it_does));

            for asked in &machine.asked {
                assert!(
                    !asked.contains("a-secret-nobody-may-see"),
                    "{asked} carries the password in the command string"
                );
            }
            // **Twice per operation**, and S7d's refresh is an operation of its
            // own in front of the installs that carry it.
            let operations = if THE_INSTALLS_THE_LISTS_ARE_REFRESHED_BEFORE.contains(step) {
                2
            } else {
                1
            };
            assert_eq!(
                machine.carried_on_standard_input.len(),
                2 * operations,
                "{} did not carry the password on standard input twice per operation",
                step.what_it_does
            );
        }
        assert!(!a_second_route_beside_the_elevating_command_is_opened(true));
        password.the_act_ended();
    }

    /// **S7d, over the whole set rather than one install at a time.** Every
    /// elevated command that installs through `apt-get` is refreshed first, and
    /// the one that is not says why in its own list — the repair, which no
    /// route issues. **A new install fails this until somebody decides**, which
    /// is the point: the third pass's 404 was an install nobody had asked this
    /// question of.
    #[test]
    fn every_apt_install_is_refreshed_first_or_says_why_not() {
        // **The support-binaries install is the second deliberate exclusion,
        // and its reason is at [`THE_INSTALLS_THE_LISTS_ARE_REFRESHED_BEFORE`]**:
        // it is issued inside one act with the two installs in front of it,
        // whose refresh it runs against, and its own command cannot be split
        // from them because it derives its package name from the tool they put
        // on the machine.
        // **The OptiX library's install is the third, added 2026-10-03, for
        // the same reasons**: it is issued in the same act directly after the
        // support binaries, and its lookup, its read and its install are one
        // command.
        // **R1 item 5's swap is the fourth, added 2026-10-11**: it refreshes
        // the lists itself, inside its own command and ahead of its trial
        // run, under its own 900-second lock wait — `issue`'s refresh waits
        // 120, which the ten minutes observed on Hyperstack's R550 image
        // would outlast.
        let not_refreshed_on_purpose = [
            THE_REPAIRS_SECOND_COMMAND,
            THE_UBUNTU_DRIVER_SUPPORT_BINARIES_INSTALL,
            THE_UBUNTU_OPTIX_LIBRARY_INSTALL,
            THE_UBUNTU_DRIVER_SWAP,
            THE_UBUNTU_DRIVER_SWAPS_FINISH,
        ];
        for step in THE_ELEVATED_STEPS.iter().chain([&THE_ELEVATED_SEARCH]) {
            let plain = step.plain.as_str();
            // **An install is found by what the command holds and not by how it
            // is spelled.** This used to ask whether the string *began* with
            // `apt-get `, so an install wrapped in a shell would have slipped
            // the rule without anybody deciding it should.
            let an_apt_install = plain.contains("apt-get ") && plain.contains(" install");
            let refreshed = THE_INSTALLS_THE_LISTS_ARE_REFRESHED_BEFORE.contains(step);
            if step == &THE_UBUNTU_DRIVER_SWAP || step == &THE_UBUNTU_DRIVER_SWAPS_FINISH {
                let refresh = plain.find("apt-get -o DPkg::Lock::Timeout=900 update").expect("its own refresh");
                assert!(refresh < plain.find(" -s install").expect("its trial"), "refreshed before the trial");
            }
            if an_apt_install && !not_refreshed_on_purpose.contains(step) {
                assert!(refreshed, "{} is an install and is not refreshed first", step.what_it_does);
            }
            if refreshed {
                assert!(an_apt_install, "{} is refreshed first and is no install", step.what_it_does);
            }
        }
        // **The refresh itself is not refreshed first**, or `issue` would never
        // end.
        assert!(!THE_INSTALLS_THE_LISTS_ARE_REFRESHED_BEFORE.contains(&THE_LIST_UPDATE));
        assert!(THE_INSTALLS_THE_LISTS_ARE_REFRESHED_BEFORE.contains(&THE_UBUNTU_DRIVER_TOOL_INSTALL));
    }

    /// **S7d, performed: the refresh goes first, and a refresh that did not end
    /// cleanly is the answer — the install is not sent.**
    #[test]
    fn a_refresh_that_fails_is_the_answer_and_no_install_is_sent() {
        let mut machine = AMachine::answering(vec![
            (
                THE_LIST_UPDATE.elevated.as_str(),
                complained("E: Failed to fetch http://archive.example/dists/noble/InRelease", 100),
            ),
            (THE_UBUNTU_DRIVER_TOOL_INSTALL.elevated.as_str(), said("", 0)),
        ]);
        let said_it = block_on(issue(
            &mut machine,
            THE_UBUNTU_DRIVER_TOOL_INSTALL,
            HowElevationIsObtained::Passwordless,
        ))
        .expect("the refresh was issued");
        assert!(!said_it.it_ended_cleanly);
        assert!(said_it.the_machines_own_words.contains("Failed to fetch"));
        assert_eq!(machine.asked, vec![THE_LIST_UPDATE.elevated.as_str()]);

        // **And where it ends cleanly the install follows it, in that order.**
        let mut machine = AMachine::answering(vec![
            (THE_LIST_UPDATE.elevated.as_str(), said("", 0)),
            (THE_UBUNTU_DRIVER_TOOL_INSTALL.elevated.as_str(), said("", 0)),
        ]);
        block_on(issue(
            &mut machine,
            THE_UBUNTU_DRIVER_TOOL_INSTALL,
            HowElevationIsObtained::Passwordless,
        ))
        .expect("both were issued");
        assert_eq!(
            machine.asked,
            vec![
                THE_LIST_UPDATE.elevated.as_str(),
                THE_UBUNTU_DRIVER_TOOL_INSTALL.elevated.as_str(),
            ]
        );

        // **A step that is not an install goes out alone.**
        let mut machine = AMachine::answering(vec![(THE_RESTART.elevated.as_str(), said("", 0))]);
        block_on(issue(&mut machine, THE_RESTART, HowElevationIsObtained::Passwordless))
            .expect("the restart was issued");
        assert_eq!(machine.asked, vec![THE_RESTART.elevated.as_str()]);
    }

    /// **Y7a: the run's words are kept apart by stream and by step**, each
    /// piece whole and in the order it came, and each stream read back whole
    /// with nothing put between two steps. Added 2026-10-18, the Creator's
    /// thirteenth notes, note 7.
    #[test]
    fn y7a_the_runs_words_are_kept_apart_by_stream_and_by_step() {
        use crate::ready_action::Step;
        let mut printed = WhatTheRunPrinted::default();
        printed.printed(Step::Tooling, WhichStream::TheOrdinaryOutput, b"Reading package lists...\n");
        printed.printed(Step::Tooling, WhichStream::TheComplaints, b"debconf: delaying\n");
        printed.printed(Step::Tooling, WhichStream::TheOrdinaryOutput, b"Setting up zstd ...\n");
        printed.printed(Step::Driver, WhichStream::TheOrdinaryOutput, b"Unpacking nvidia-driver-595 ...\n");
        printed.printed(Step::Driver, WhichStream::TheComplaints, b"");

        let steps = printed.the_steps();
        assert_eq!(steps.len(), 2, "one entry per step: {steps:?}");
        assert_eq!(steps[0].step, Step::Tooling);
        assert_eq!(steps[0].the_ordinary_output, b"Reading package lists...\nSetting up zstd ...\n");
        assert_eq!(steps[0].the_complaints, b"debconf: delaying\n");
        assert_eq!(steps[1].step, Step::Driver);
        assert_eq!(steps[1].the_complaints, b"", "an empty piece keeps nothing");
        assert_eq!(
            printed.the_whole_of(WhichStream::TheOrdinaryOutput),
            b"Reading package lists...\nSetting up zstd ...\nUnpacking nvidia-driver-595 ...\n"
        );
        assert_eq!(printed.the_whole_of(WhichStream::TheComplaints), b"debconf: delaying\n");
    }

    /// **Y7a item 5: a step whose words could not be brought back keeps the
    /// mark under it, and no words are made up for it.** Added 2026-10-24,
    /// the Creator's ruling on the sixteenth notes, note 1.
    #[test]
    fn y7a_a_steps_mark_that_its_words_were_not_kept_stands_under_it_alone() {
        use crate::ready_action::Step;
        let mut printed = WhatTheRunPrinted::default();
        printed.printed(Step::Readings, WhichStream::TheOrdinaryOutput, b"NVIDIA-SMI 580\n");
        assert!(!printed.some_words_were_not_kept());
        printed.its_words_were_not_kept(Step::Libraries);
        printed.printed(Step::Done, WhichStream::TheOrdinaryOutput, b"Blender 5.2.2\n");
        let steps = printed.the_steps();
        assert_eq!(steps.len(), 3);
        assert!(steps[1].some_were_not_kept && steps[1].the_ordinary_output.is_empty());
        assert!(steps[1].the_complaints.is_empty());
        assert!(!steps[0].some_were_not_kept && !steps[2].some_were_not_kept);
        assert!(printed.some_words_were_not_kept());
        assert_eq!(
            printed.the_whole_of(WhichStream::TheOrdinaryOutput),
            b"NVIDIA-SMI 580\nBlender 5.2.2\n"
        );
    }
}
