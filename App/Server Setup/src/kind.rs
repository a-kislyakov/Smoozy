//! The machine kind and the distribution family — `Server Setup` §1.
//!
//! **The constraint** is Row 116 and Fact 27 on both weights. Two readings are
//! taken of every machine before a setup run is composed, they are taken
//! **before the first act that changes anything**, and a machine either pair
//! refuses is a machine no step of [`crate::ready_action`] is composed for.
//!
//! **Nothing here reaches a machine.** The two readings sit behind
//! [`ReadsTheMachineBeforeAnythingIsTouched`] and every judgement over them is
//! at the desk.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Rented-server runtime` K1, K2, K3, K4, K8, K11, K17, K19,
//! V6, G28. `Job files and outputs` L10. `Servers` H11, H12b, Q12. `Security
//! and licence` §13 rule 5.
//!
//! **`Server protocol and transfers` B8 is not in the list above and is written
//! out at every site**, because this plan defines a B8 of its own and a bare one
//! could mean either.

use smoozy_server_protocol::release::manifest::TargetTuple;
use smoozy_server_protocol::transport::CommandString;
use smoozy_servers::first_connection::NotLinuxOutcome;

/// **M1: the kind, as `Rented-server runtime` K1's table gives it.**
///
/// *"Every setup act below runs against the kind K1's table detected for the
/// current installation."* The detection itself is that document's and is not
/// re-invented — `systemd-detect-virt` plus the container marker file, selected
/// by that closed table, **re-taken at every placement and never carried
/// forward** (`Rented-server runtime` K2). `Servers` already performs the same
/// table for its own hardware reading, and the two must stay identical.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineKind {
    /// A systemd machine, virtual or bare-metal.
    ASystemdMachine,
    /// A container.
    AContainer,
    /// **The row `Rented-server runtime` K3 refuses.** *"A machine the table
    /// does not place is a machine no package is placed on and no setup runs
    /// on; the user is told the machine is not one Smoozy can set up."*
    TheTableDoesNotPlaceIt,
}

impl MachineKind {
    pub const ALL: [MachineKind; 3] = [
        MachineKind::ASystemdMachine,
        MachineKind::AContainer,
        MachineKind::TheTableDoesNotPlaceIt,
    ];

    /// **M1: a setup runs on a placed machine and on no other.**
    pub fn a_setup_may_run_on_it(self) -> bool {
        !matches!(self, MachineKind::TheTableDoesNotPlaceIt)
    }
}

/// **M1a: the pair of families this module prepares, and it is a fixed set
/// rather than a pattern.**
///
/// Creator-stated 2026-08-22 — `Preparation/Specs/Server Setup.md`, *Getting
/// the machine ready*. Every step below that installs, repairs or removes
/// anything is written in one family's terms, so the family is a reading of the
/// same standing as the kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistributionFamily {
    Ubuntu,
    Debian,
}

impl DistributionFamily {
    /// **The set is fixed at two.** *"The run is composed where `ID` is
    /// `ubuntu` or `debian`, and nowhere else."*
    pub const THE_SUPPORTED_PAIR: [DistributionFamily; 2] =
        [DistributionFamily::Ubuntu, DistributionFamily::Debian];

    /// The `ID` value each family answers with.
    pub fn id(self) -> &'static str {
        match self {
            DistributionFamily::Ubuntu => "ubuntu",
            DistributionFamily::Debian => "debian",
        }
    }
}

/// **M1a: which of the two files a reading came from, and the precedence is the
/// standard's own.**
///
/// **Documented** — `os-release(5)`, systemd 252.36, read 2026-08-22: *"The
/// file /etc/os-release takes precedence over /usr/lib/os-release. Applications
/// should check for the former, and exclusively use its data if it exists, and
/// only fall back to /usr/lib/os-release if it is missing."*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhichOsReleaseFile {
    /// Read first, and used exclusively where it exists.
    Etc,
    /// **Read only where the first is absent.**
    UsrLib,
}

impl WhichOsReleaseFile {
    /// The two, in the standard's own precedence order.
    pub const IN_PRECEDENCE_ORDER: [WhichOsReleaseFile; 2] =
        [WhichOsReleaseFile::Etc, WhichOsReleaseFile::UsrLib];

    pub fn path(self) -> &'static str {
        match self {
            WhichOsReleaseFile::Etc => "/etc/os-release",
            WhichOsReleaseFile::UsrLib => "/usr/lib/os-release",
        }
    }
}

/// **M1a's family reading, in its three states.**
///
/// **An absent answer is never read as Ubuntu.** *"Where neither file can be
/// read, or `ID` is absent, the family is **unknown** and the run is refused in
/// the same words as an unsupported one."*
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FamilyReading {
    /// One of the pair, by `ID` alone.
    Supported(DistributionFamily),
    /// The machine said which family it is and it is not one of the pair.
    /// **`ID_LIKE` is carried only to say why the machine was refused**, never
    /// to admit one.
    NotOneOfThePair {
        id: String,
        id_like: Option<String>,
    },
    /// **The machine would not say.** Neither file could be read, or `ID` was
    /// absent from what was read.
    ItWouldNotSay,
}

impl FamilyReading {
    /// **M1a: a run is composed on the supported pair and on nothing else.**
    pub fn a_setup_may_be_composed_for_it(&self) -> bool {
        matches!(self, FamilyReading::Supported(_))
    }

    pub fn family(&self) -> Option<DistributionFamily> {
        match self {
            FamilyReading::Supported(family) => Some(*family),
            _ => None,
        }
    }
}

/// **M1a: the file is parsed as the standard describes and never sourced as a
/// shell script.**
///
/// The standard permits sourcing and then says why it is not needed — *"beyond
/// mere variable assignments, no shell features are supported … allowing
/// applications to read the file without implementing a shell compatible
/// execution engine"*. Sourcing a machine-written file would also put a
/// machine-origin value into a command, which `Server protocol and transfers`
/// 1.1 forbids outright.
///
/// **So this reads assignments and nothing else**: a `#` comment line and a
/// blank line are skipped, a line with no `=` is skipped, and one layer of
/// matching single or double quotes is taken off the value. **No expansion of
/// any kind happens** — no `$`, no backticks, no command substitution, no
/// escape processing — because none of it is in the file's grammar.
pub fn read_an_os_release_file(text: &str, key: &str) -> Option<String> {
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((name, value)) = line.split_once('=') else {
            continue;
        };
        if name.trim() != key {
            continue;
        }
        let value = value.trim();
        let unquoted = value
            .strip_prefix('"')
            .and_then(|rest| rest.strip_suffix('"'))
            .or_else(|| {
                value
                    .strip_prefix('\'')
                    .and_then(|rest| rest.strip_suffix('\''))
            })
            .unwrap_or(value);
        return Some(unquoted.to_string());
    }
    None
}

/// **M1a, performed: the family, from whichever of the two files answered.**
///
/// **The test is a fixed set rather than a pattern**, and `ID_LIKE` is read
/// only to say why a machine was refused. The Creator was asked on 2026-08-25
/// whether to widen it and ruled that the refusal stands: a machine carrying
/// `ID_LIKE=ubuntu` is Ubuntu underneath in every way this module touches, and
/// is refused all the same, because admitting a family on `ID_LIKE` alone is a
/// promise about machines nobody has run a setup on. **Widening is a Specs
/// change and stays the Creator's.**
///
/// **A second arm that admitted an unrecognised `ID` whose `ID_LIKE` named
/// `debian` was written on 2026-08-22 and taken out the same day, and it is not
/// written back here**: it assigned such a machine no family at all, so
/// [`M1b`](TheDriverRoute)'s per-family route, the repair sequence and the
/// package names would have had nothing to select on, and the machine would
/// have passed the gate only to meet the first step that differs between the
/// two families with no rule for it.
pub fn read_the_family(etc: Option<&str>, usr_lib: Option<&str>) -> FamilyReading {
    // The standard's own precedence: the first is used exclusively where it
    // exists, and the second is read only where it is missing.
    let text = match (etc, usr_lib) {
        (Some(text), _) => Some(text),
        (None, Some(text)) => Some(text),
        (None, None) => None,
    };
    let Some(text) = text else {
        return FamilyReading::ItWouldNotSay;
    };
    let Some(id) = read_an_os_release_file(text, "ID") else {
        return FamilyReading::ItWouldNotSay;
    };
    for family in DistributionFamily::THE_SUPPORTED_PAIR {
        if id == family.id() {
            return FamilyReading::Supported(family);
        }
    }
    FamilyReading::NotOneOfThePair {
        id,
        id_like: read_an_os_release_file(text, "ID_LIKE"),
    }
}

/// **M1b: Debian is not Ubuntu at the one step that matters, and each family
/// has its own route.**
///
/// Both were run end to end — the Ubuntu tool is Canonical's own and exists
/// nowhere else; the Debian route was run on a rented Debian 12 machine on
/// 2026-08-22, taking **149 seconds for the install plus one restart**.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheDriverRoute {
    pub family: DistributionFamily,
}

impl TheDriverRoute {
    pub fn for_family(family: DistributionFamily) -> TheDriverRoute {
        TheDriverRoute { family }
    }

    /// **M1b: the Debian route needs `contrib non-free non-free-firmware`
    /// enabled first**, because the stock image reaches `main` only and the
    /// driver is otherwise *not merely uninstalled but unreachable* —
    /// `apt-cache policy nvidia-driver` returned `Candidate: (none)`.
    pub fn the_components_must_be_enabled_first(self) -> bool {
        matches!(self.family, DistributionFamily::Debian)
    }

    /// **M1b: the Debian route needs a restart, unconditionally.**
    ///
    /// **The trap that makes it mandatory is in no manual**: the headers for the
    /// kernel a stale image booted may not be in the archive at all —
    /// `linux-headers-6.1.0-31-amd64` was absent — so the install pulls a newer
    /// kernel and the module is built against one the machine is not running.
    /// Until the boot the route *"looks broken and is not"*.
    pub fn the_restart_is_not_optional(self) -> bool {
        matches!(self.family, DistributionFamily::Debian)
    }

    /// **M1b and [`crate::ready_action`] R1 item 5: the steps of the route, in
    /// the order M1b gives them.**
    ///
    /// `Server protocol and transfers` 1.1 forbids interpolating any value of
    /// user, provider or machine origin into a command string, and every
    /// command below is fixed at build time — which is [`crate::elevation`]
    /// E7's third bullet. **One step of the Debian route carries no command
    /// string, and that is stated rather than filled in** — see
    /// [`DriverStep::the_command`].
    pub fn the_steps(self) -> &'static [DriverStep] {
        match self.family {
            DistributionFamily::Ubuntu => &UBUNTUS_DRIVER_ROUTE,
            DistributionFamily::Debian => &DEBIANS_DRIVER_ROUTE,
        }
    }
}

/// One step of a family's driver route.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DriverStep {
    /// What the step does, in M1b's own terms.
    pub what_it_does: &'static str,
    command: Option<CommandString>,
}

impl DriverStep {
    /// The constant command the step issues, where this document establishes
    /// one.
    ///
    /// **`None` is a stated absence and never a hole to be plugged with a
    /// remembered command.** One step has it: enabling `contrib non-free
    /// non-free-firmware` on Debian. M1b requires the components enabled and
    /// **names no command for it**, and the evidence behind M1b records the
    /// state it found — *"`/etc/apt/sources.list.d/debian.sources` reads
    /// `Components: main`"* — rather than the act that changed it. **Rule 2
    /// binds: a command not established by the plans or by the tool's own
    /// publication is not written from memory here.**
    ///
    /// **The reading was taken on 2026-09-04, and what it settles is that there
    /// is no command to write.** Debian's own publication names none: enabling
    /// a component is **an edit of the sources file's own `Components` field**,
    /// and the step is performed as that edit at
    /// [`crate::acts::enable_the_components`], which carries both citations.
    /// **So this `None` is no longer an absence waiting to be filled — it is the
    /// answer**: this step is not a command, and a builder who ever writes one
    /// here has invented it.
    ///
    /// **What broke while the step was unbuilt**, kept here because it is what
    /// the safe direction bought: on a Debian machine whose card had no working
    /// driver the route could not reach its install, the check-then-restart
    /// order then failed `nvidia-smi`, and the machine was reported **not
    /// ready** — [`crate::statements`] W4's refusal rather than a wrong render.
    pub fn the_command(self) -> Option<CommandString> {
        self.command
    }
}

/// **Ubuntu Server's own documented and recommended command**, *"which will
/// install the driver that is considered the best match for your hardware"*
/// (read 2026-08-20).
///
/// **Naming a flavour here is forbidden**, and this replaces a fixed rule that
/// named the proprietary one: NVIDIA's own README records that the proprietary
/// flavour serves up to Blackwell and the open modules serve Turing and later,
/// so **neither flavour serves every card** and any fixed rule fails at one end
/// of the range.
///
/// **Its first step makes the tool reachable — added 2026-09-21**, R1 item 5 as
/// corrected: `ubuntu-drivers` is in `ubuntu-drivers-common`, which a plain
/// Ubuntu image need not carry (Hyperstack's plain 24.04 did not). It is
/// [`crate::elevated::THE_UBUNTU_DRIVER_TOOL_INSTALL`]'s command.
const UBUNTUS_DRIVER_ROUTE: [DriverStep; 2] = [
    DriverStep {
        what_it_does: "install the distribution's own driver detection tool, which a plain \
                       image need not carry",
        command: Some(CommandString::constant(
            "apt-get -o DPkg::Lock::Timeout=120 install -y ubuntu-drivers-common",
        )),
    },
    DriverStep {
        what_it_does: "install the driver through the distribution's own detection route",
        command: Some(CommandString::constant("ubuntu-drivers install --gpgpu")),
    },
];

/// **Debian's own equivalent, run end to end on 2026-08-22.**
///
/// `nvidia-detect` is in `non-free` and **names the package to install**, which
/// is exactly what Canonical's tool is used for. `ubuntu-drivers-common` is
/// confirmed absent and unpackaged on Debian — `apt-cache policy` returned
/// nothing at all.
const DEBIANS_DRIVER_ROUTE: [DriverStep; 3] = [
    DriverStep {
        what_it_does: "enable `contrib non-free non-free-firmware` and update the lists, \
                       without which the driver is unreachable rather than merely uninstalled",
        command: None,
    },
    DriverStep {
        what_it_does: "ask Debian's own detection tool which package to install",
        command: Some(CommandString::constant("nvidia-detect")),
    },
    // **The bounded wait is [`crate::recovery`] S7b's and rides every package
    // step of a setup run**, rather than apt's own default of `0`, at which the
    // lock acquisition returns immediately.
    DriverStep {
        what_it_does: "install the package the detection tool named",
        command: Some(CommandString::constant(
            "apt-get -o DPkg::Lock::Timeout=120 install nvidia-driver",
        )),
    },
];

/// **M2: what each kind permits, taken from Fact 27 and Fact 48 rather than
/// probed by trial.**
///
/// **Nothing here turns on which kind is commoner.** Every rule in this
/// document is written per kind, off a reading taken at every placement and
/// never carried forward (`Rented-server runtime` K2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhatTheKindPermits {
    /// **Package placement and boot registration.** Unprivileged and whole on a
    /// systemd machine (`Rented-server runtime` K4 to K8), the boot-hook case
    /// not arising there; on a container the registration is the provider's
    /// hook and **installing it requires root** (`Rented-server runtime` K11),
    /// and the root step must leave it readable (`Rented-server runtime` K19).
    pub the_registration_step_needs_root: bool,
    /// **The hand-over of a chosen mount that arrived `root:root`** — an
    /// elevated step of this document's on either kind (`Job files and outputs`
    /// L10; Fact 6), and on a container only where the login can elevate at
    /// all.
    pub the_hand_over_is_an_elevated_step: bool,
    /// **The unpack tool and the shared libraries.** Elevated package installs
    /// on a systemd machine; on a container possible only where the login is
    /// the container's root, and **refused outright to a non-root login**
    /// (Fact 27).
    pub package_installs_need_the_containers_root: bool,
    /// **The driver install, repair and reload.** Elevated on a systemd
    /// machine, and **never** on a container — all four routes to the driver
    /// are closed, a restart is not a recovery action there, and a card that
    /// has stopped answering is terminal for that server (Fact 48).
    pub the_driver_may_be_touched: bool,
    /// **The machine restart.** `systemctl reboot` on a systemd machine
    /// (Fact 51); **never issued on a container**, where it would restart the
    /// container rather than the machine and repair nothing (Fact 48).
    pub a_restart_may_be_issued: bool,
}

/// **M2's table, performed.**
pub fn what_the_kind_permits(kind: MachineKind) -> Option<WhatTheKindPermits> {
    match kind {
        MachineKind::ASystemdMachine => Some(WhatTheKindPermits {
            the_registration_step_needs_root: false,
            the_hand_over_is_an_elevated_step: true,
            package_installs_need_the_containers_root: false,
            the_driver_may_be_touched: true,
            a_restart_may_be_issued: true,
        }),
        MachineKind::AContainer => Some(WhatTheKindPermits {
            the_registration_step_needs_root: true,
            the_hand_over_is_an_elevated_step: true,
            package_installs_need_the_containers_root: true,
            the_driver_may_be_touched: false,
            a_restart_may_be_issued: false,
        }),
        // **M1: a machine the table does not place has no row**, because no
        // setup runs on it at all.
        MachineKind::TheTableDoesNotPlaceIt => None,
    }
}

/// **M3: the elevation probe is the elevating command's non-interactive
/// form.**
///
/// **Documented** — `sudo(8)` 1.9.17p2, read 2026-08-19, and again at
/// **1.9.15p5**, the release the rental actually ran, read 2026-08-23, where
/// the option is given word for word: `-n, --non-interactive` *"Avoid prompting
/// the user for input of any kind. If a password is required for the command to
/// run, sudo will display an error message and exit."*
///
/// **The form is the non-interactive form of a command that does nothing, and
/// it is not the validate form. Changed 2026-09-04, out of a build sitting on
/// rented machines; M3 carries the repair and no auditor of that document has
/// seen it.**
///
/// **Observed in a controlled test** on two providers' standard images and two
/// distribution families — Hyperstack `Debian 12 Plain` and Hyperstack `Ubuntu
/// Server 22.04 LTS R570 CUDA 12.8`, each on the login its provider delivered,
/// **2026-09-04**: `sudo -n true` **exited 0 on both**, while `sudo -n -v`
/// **exited 1 with `sudo: a password is required` on both**. The cause was read
/// on the machines: each login is granted **both** `(ALL : ALL) ALL`, which asks
/// for a password, and `(ALL) NOPASSWD: ALL`, which does not, and validating
/// meets the first. **So the old form reported a password owed on machines that
/// needed none, and what followed was [`crate::elevation`] E2's password flow —
/// a user asked for a password that does not exist, on the ordinary case.**
///
/// **Why the validate form was the wrong question rather than a wrong answer**,
/// and this half is documented: `-v, --validate` *"Update the user's cached
/// credentials, authenticating the user if necessary… but does not run a
/// command"* asks whether the login can **authenticate**; `-n` on a command asks
/// whether **a password is required for that command to run**, which is what
/// this constant is for.
///
/// **The earlier observation stands and is not overwritten**: on 2026-08-22,
/// under a purpose-made non-root login in group `sudo` with Ubuntu's stock
/// sudoers, `sudo -n -v` returned `sudo: a password is required` at exit 1. That
/// login's policy and these two are different policies.
///
/// **What this constant does not claim.** A probe that succeeds says a password
/// was not required **for the probe's own command** and nothing further. A
/// policy granting some commands without a password and refusing others would
/// answer *passwordless* here and refuse a later step; the only sudoers policies
/// this project has read are the two blanket ones above, on two machines on one
/// day. **Nothing is built for that case**: [`crate::elevated::issue`] classifies
/// a refused step from that step's own result, and E2's timing rule puts the ask
/// at the refusal rather than at the probe. **What would settle it** is
/// `sudoers(5)` read as a publication on the per-command `NOPASSWD` tag, and one
/// rental carrying a per-command policy; neither has been done.
pub const THE_NON_INTERACTIVE_PROBE: CommandString = CommandString::constant("sudo -n true");

/// **M3's two outcomes, and there is no third.**
///
/// **The prompt's own text is never matched.** Ubuntu 25.10 and 26.04 install
/// `sudo-rs` by default — that project's own README states it — and its prompt
/// reads `[sudo: authenticate] <METHOD>:` where the classic tool's reads
/// `[sudo] password for <USERNAME>`. Nothing here breaks on that, because
/// [`crate::elevation`] E2 hands the password on standard input and matches no
/// prompt; it is recorded because anything at Build that matched on the prompt
/// text would break on Ubuntu 25.10 and later. **`sudo-rs` has never been run on
/// a rental.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Probe {
    /// **Passwordless elevation.**
    Succeeded,
    /// **A password is owed**, and [`crate::elevation`] E2's flow begins at the
    /// moment a step needs it — **never before**, which is the spec's own rule.
    Failed,
}

impl Probe {
    pub fn passwordless(self) -> bool {
        matches!(self, Probe::Succeeded)
    }
}

/// **M4: the login Smoozy holds is the connection's login and no other.**
///
/// Smoozy knows servers and SSH keys, never providers, so an account it has no
/// credential for — the cloud-init account holding passwordless root on one
/// observed image — **is not a route Smoozy has.** Where the connection's login
/// has no elevation route, the machine's preparation cannot be performed, which
/// is Row 116's own clause: *"a machine that refuses it may not be set up at
/// all."* **That is a narrowing the spec already carries, not a hole.**
pub const THE_ONLY_LOGIN_IS_THE_CONNECTIONS_OWN: bool = true;

/// **M4, stated as the question a caller asks rather than as a comment.**
///
/// A second account on the machine is never reached for, however much it would
/// help.
pub fn another_account_on_the_machine_is_a_route(_it_exists: bool) -> bool {
    false
}

/// **M1a's last bullet: both readings are taken before the hand-over rather
/// than at it.**
///
/// [`crate::ready_action`] R2's order puts the elevated `chown` of the chosen
/// mount and the areas' creation **before** the placement, so a family read at
/// the placement would be read on a machine already changed and *nothing is
/// touched* would be false in the one case it is written for. **It costs
/// nothing to move**: `/etc/os-release` is a plain read over the file-transfer
/// subsystem, needing no elevation, nothing placed and no hand-over, and the
/// kind detection is a command-channel reading of the same standing.
pub trait ReadsTheMachineBeforeAnythingIsTouched {
    /// `Rented-server runtime` K1's table, performed against this machine.
    fn the_kind(&mut self) -> MachineKind;
    /// One of the two files, or nothing where it could not be read.
    fn os_release(&mut self, which: WhichOsReleaseFile) -> Option<String>;
}

/// The pair of readings a run is composed from, taken together and before
/// anything is touched.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheOpeningReading {
    pub kind: MachineKind,
    pub family: FamilyReading,
}

impl TheOpeningReading {
    /// **M1 and M1a together: a run is composed only where both readings admit
    /// the machine.**
    pub fn a_run_may_be_composed(&self) -> bool {
        self.kind.a_setup_may_run_on_it() && self.family.a_setup_may_be_composed_for_it()
    }
}

/// **The opening reading, taken in one act before anything is touched.**
///
/// **Nothing is installed, nothing is touched, and the machine is left exactly
/// as found** wherever this refuses — so the refusal is a reading rather than a
/// part-finished run, and [`crate::recovery`]'s machinery never engages, because
/// no setup-in-progress state is ever set.
pub fn read_before_anything_is_touched(
    reader: &mut dyn ReadsTheMachineBeforeAnythingIsTouched,
) -> TheOpeningReading {
    let kind = reader.the_kind();
    let etc = reader.os_release(WhichOsReleaseFile::Etc);
    // **Read only where the first is absent**, which is the standard's own
    // precedence and not a fallback invented here.
    let usr_lib = match etc {
        Some(_) => None,
        None => reader.os_release(WhichOsReleaseFile::UsrLib),
    };
    TheOpeningReading {
        kind,
        family: read_the_family(etc.as_deref(), usr_lib.as_deref()),
    }
}

// -------------------------------------------------- the rental's own tuple

/// **The word a manifest's tuple carries for the Linux family.**
///
/// `Servers` Q12 asks the machine to name its own operating system and compares
/// the answer against the one value that means Linux; this is the word a
/// manifest writes that family as. **The two are different words for the same
/// family and neither is derived from the other** — the machine's is
/// `smoozy_servers::first_connection::THE_LINUX_VALUE` and this is
/// `Server protocol and transfers`'s. The test beside this code holds them
/// together by composing a tuple from a machine's own answers and comparing it
/// with what this build actually produced.
pub const THE_MANIFESTS_WORD_FOR_THE_LINUX_FAMILY: &str = "linux";

/// **The name the GNU C library gives itself when asked.**
///
/// **Documented** — `confstr(3)`, Linux man-pages through `manpages-dev`
/// 6.03-2, dated 2023-02-05, read 2026-09-07 at
/// `https://manpages.debian.org/bookworm/manpages-dev/confstr.3.en.html`: the
/// GNU C library version variable is *"A string which identifies the GNU C
/// library version on this system (e.g., "glibc 2.3.4")"*, and it exists on the
/// *"GNU C library only"*.
///
/// **The first field of that answer is taken as the library's name, and that is
/// a narrowing rather than a reading.** The publication gives the shape as an
/// example rather than as a specification, so a machine whose answer begins
/// with anything else is refused rather than admitted. **The safe direction is
/// the one that refuses**: a refusal places nothing.
pub const THE_GNU_C_LIBRARYS_OWN_NAME: &str = "glibc";

/// **The word a manifest's tuple carries for the GNU C library's ABI.**
pub const THE_MANIFESTS_WORD_FOR_THE_GNU_C_LIBRARY: &str = "gnu";

/// **The answer that is not an answer**, which the asking utility writes for a
/// variable that is valid and has no value on this machine.
///
/// **Documented** — `getconf`, POSIX.1-2017, read 2026-09-07 at
/// `https://pubs.opengroup.org/onlinepubs/9699919799/utilities/getconf.html`:
/// *"If the specified variable is valid, but is undefined on the system,
/// getconf shall write using the following format: "undefined\n""*. It is
/// written at exit 0, so **a machine that answers this has ended cleanly and
/// still named no library.**
pub const THE_ANSWER_THAT_NAMES_NOTHING: &str = "undefined";

/// **Why no tuple could be read**, which is the *would not say* state.
///
/// **An unreadable answer refuses and is never defaulted.** That is the
/// discipline M1a already holds to for the family — an absent answer is never
/// read as Ubuntu — applied to the tuple: an absent answer is never read as the
/// tuple this build happens to have been made for, which is the one reading
/// that would make the exact match unable to catch anything.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhyNoTupleWasRead {
    /// The machine named an operating system and it was not Linux.
    TheMachineIsNotALinuxMachine,
    /// It named nothing anybody could read — no answer, a refused command, an
    /// unreadable one, all one outcome.
    ItWouldNotNameItsOperatingSystem,
    /// The machine-hardware-name reading did not answer.
    ItWouldNotNameItsProcessorArchitecture,
    /// The C library reading did not answer, or answered
    /// [`THE_ANSWER_THAT_NAMES_NOTHING`].
    ItWouldNotNameItsCLibrary,
    /// It named a C library and it is not the one this build's ABI half is
    /// written for. **The name is kept whole**, so the refusal can say what the
    /// machine actually said.
    ItNamedADifferentCLibrary { what_it_named: String },
}

/// **`Server protocol and transfers` B8's detected rental tuple: the operating-
/// system family, the processor architecture, and the ABI.**
///
/// That clause names this module as the supplier in as many words — *"`Server
/// Setup` supplies the detected rental tuple; the desktop selects an artifact
/// only on an exact manifest match"* — and `Rented-server runtime` G28 says the
/// same from the machine's side. **Until this landed nothing in Smoozy read a
/// machine's processor architecture or its C library**, so every selection was
/// made against the constant naming what this build produced and could not fail.
///
/// **The three states are supported, said something else, and would not say, and
/// only the last is this module's.** A tuple read whole is handed to the
/// selection, and the first two are that selection's own two answers —
/// `Server protocol and transfers` B8's *"no matching entry is an unsupported
/// machine reported before any package byte is executed"*. **Would not say is
/// this one's**, and it is [`WhyNoTupleWasRead`]: no tuple is composed, so
/// nothing is selected and nothing is placed.
///
/// **No fallback of any kind is reachable from here** — no closest processor, no
/// closest ABI, no generic Linux entry, no capability fallback, no package-name
/// guess and no first entry — because this composes a value and makes no
/// comparison at all. The only comparison anywhere is whole-value equality, in
/// the crate that owns the manifest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheRentalsTuple {
    /// **Every half answered.** This is a value put to the selection and **not a
    /// claim that the machine is supported**, which is the selection's answer
    /// and nothing else's.
    ItSaid(TargetTuple),
    /// **A half would not say, or named something with no place in a tuple.**
    ItWouldNotSay(WhyNoTupleWasRead),
}

impl TheRentalsTuple {
    /// The tuple where one was read, and nothing otherwise.
    pub fn as_read(&self) -> Option<&TargetTuple> {
        match self {
            TheRentalsTuple::ItSaid(tuple) => Some(tuple),
            TheRentalsTuple::ItWouldNotSay(_) => None,
        }
    }
}

/// **`Server protocol and transfers` B8's tuple, composed from what the machine
/// said and from nothing else.**
///
/// **Nothing here is inferred.** Each half is either a value the machine named
/// or an absence, and one absence is the whole reading's absence. The
/// operating-system half is `Servers` Q12's own reading, taken as a value; the
/// other two are the machine's own words, trimmed and otherwise untouched.
///
/// **The C library's release is read and then used for nothing, deliberately.**
/// The answer names the library and its release together, and only the library
/// reaches the tuple: a binary built against one release of that library is not
/// thereby known to run against an older one, and nobody on this project has
/// read that library's own publication on the question. **So there is no
/// generation arithmetic and no close enough**, which is what
/// `Server protocol and transfers` B8 forbids anyway.
pub fn the_tuple_the_machine_named(
    what_it_named_its_operating_system: NotLinuxOutcome,
    what_it_named_its_processor_architecture: Option<&str>,
    what_it_named_its_c_library: Option<&str>,
) -> TheRentalsTuple {
    let family = match what_it_named_its_operating_system {
        NotLinuxOutcome::ItNamedLinux => THE_MANIFESTS_WORD_FOR_THE_LINUX_FAMILY,
        NotLinuxOutcome::ItNamedSomethingElse => {
            return TheRentalsTuple::ItWouldNotSay(WhyNoTupleWasRead::TheMachineIsNotALinuxMachine)
        }
        NotLinuxOutcome::NothingSmoozyCouldRead => {
            return TheRentalsTuple::ItWouldNotSay(
                WhyNoTupleWasRead::ItWouldNotNameItsOperatingSystem,
            )
        }
    };

    let architecture = match what_it_named_its_processor_architecture.map(str::trim) {
        Some(named) if !named.is_empty() => named,
        _ => {
            return TheRentalsTuple::ItWouldNotSay(
                WhyNoTupleWasRead::ItWouldNotNameItsProcessorArchitecture,
            )
        }
    };

    let named_library = match what_it_named_its_c_library.map(str::trim) {
        Some(named) if !named.is_empty() && named != THE_ANSWER_THAT_NAMES_NOTHING => named,
        _ => return TheRentalsTuple::ItWouldNotSay(WhyNoTupleWasRead::ItWouldNotNameItsCLibrary),
    };
    // **The first field is the library's own name and the rest is its release**,
    // which is only ever read to say what a refused machine carries.
    let its_name = named_library.split_whitespace().next().unwrap_or_default();
    if its_name != THE_GNU_C_LIBRARYS_OWN_NAME {
        return TheRentalsTuple::ItWouldNotSay(WhyNoTupleWasRead::ItNamedADifferentCLibrary {
            what_it_named: its_name.to_string(),
        });
    }

    TheRentalsTuple::ItSaid(TargetTuple {
        operating_system_family: family.to_string(),
        processor_architecture: architecture.to_string(),
        abi_and_capabilities: THE_MANIFESTS_WORD_FOR_THE_GNU_C_LIBRARY.to_string(),
    })
}

/// **M1a: the family is re-read exactly as the kind is** — at every placement,
/// never carried forward (`Rented-server runtime` K2). A recycled or re-imaged
/// machine under a new installation identity has **no remembered family**.
pub const NOTHING_ABOUT_THE_MACHINE_IS_CARRIED_FORWARD: bool = true;

/// **M1a: everything else about a refused machine is untouched.**
///
/// *"It connects, it is browsed, its Blenders are found by [`crate::search`]
/// B10's search, and a job runs on it if it already carries what a render
/// needs."* **This module declines; the app does not.**
pub const THE_APP_DOES_NOT_DECLINE_WITH_THIS_MODULE: bool = true;

#[cfg(test)]
mod tests {
    use super::*;

    struct AMachine {
        kind: MachineKind,
        etc: Option<String>,
        usr_lib: Option<String>,
        etc_reads: usize,
        usr_lib_reads: usize,
    }

    impl ReadsTheMachineBeforeAnythingIsTouched for AMachine {
        fn the_kind(&mut self) -> MachineKind {
            self.kind
        }

        fn os_release(&mut self, which: WhichOsReleaseFile) -> Option<String> {
            match which {
                WhichOsReleaseFile::Etc => {
                    self.etc_reads += 1;
                    self.etc.clone()
                }
                WhichOsReleaseFile::UsrLib => {
                    self.usr_lib_reads += 1;
                    self.usr_lib.clone()
                }
            }
        }
    }

    fn a_machine(kind: MachineKind, etc: Option<&str>, usr_lib: Option<&str>) -> AMachine {
        AMachine {
            kind,
            etc: etc.map(str::to_string),
            usr_lib: usr_lib.map(str::to_string),
            etc_reads: 0,
            usr_lib_reads: 0,
        }
    }

    #[test]
    fn the_supported_pair_is_two_and_nothing_else_is_admitted() {
        assert_eq!(DistributionFamily::THE_SUPPORTED_PAIR.len(), 2);
        assert_eq!(
            read_the_family(Some("ID=ubuntu\n"), None),
            FamilyReading::Supported(DistributionFamily::Ubuntu)
        );
        assert_eq!(
            read_the_family(Some("ID=debian\n"), None),
            FamilyReading::Supported(DistributionFamily::Debian)
        );
    }

    #[test]
    fn id_like_says_why_a_machine_was_refused_and_never_admits_one() {
        // M1a, and the Creator's ruling of 2026-08-25: a machine carrying
        // `ID_LIKE=ubuntu` is Ubuntu underneath in every way this module
        // touches, and is refused all the same.
        let reading = read_the_family(Some("ID=linuxmint\nID_LIKE=ubuntu\n"), None);
        assert_eq!(
            reading,
            FamilyReading::NotOneOfThePair {
                id: "linuxmint".to_string(),
                id_like: Some("ubuntu".to_string()),
            }
        );
        assert!(!reading.a_setup_may_be_composed_for_it());
        // The second arm taken out on 2026-08-22 would have admitted this one.
        assert!(read_the_family(Some("ID=pop\nID_LIKE=\"ubuntu debian\"\n"), None)
            .family()
            .is_none());
    }

    #[test]
    fn a_machine_that_will_not_say_is_refused_and_never_read_as_ubuntu() {
        assert_eq!(read_the_family(None, None), FamilyReading::ItWouldNotSay);
        assert_eq!(
            read_the_family(Some("PRETTY_NAME=\"Something\"\n"), None),
            FamilyReading::ItWouldNotSay
        );
        assert!(!FamilyReading::ItWouldNotSay.a_setup_may_be_composed_for_it());
    }

    #[test]
    fn the_first_file_is_used_exclusively_where_it_exists() {
        // The standard's own precedence, and the second file is not even read.
        let mut machine = a_machine(
            MachineKind::ASystemdMachine,
            Some("ID=ubuntu\n"),
            Some("ID=debian\n"),
        );
        let reading = read_before_anything_is_touched(&mut machine);
        assert_eq!(
            reading.family,
            FamilyReading::Supported(DistributionFamily::Ubuntu)
        );
        assert_eq!(machine.etc_reads, 1);
        assert_eq!(machine.usr_lib_reads, 0);
    }

    #[test]
    fn the_second_file_is_read_only_where_the_first_is_missing() {
        let mut machine = a_machine(MachineKind::ASystemdMachine, None, Some("ID=debian\n"));
        let reading = read_before_anything_is_touched(&mut machine);
        assert_eq!(
            reading.family,
            FamilyReading::Supported(DistributionFamily::Debian)
        );
        assert_eq!(machine.usr_lib_reads, 1);
    }

    #[test]
    fn the_file_is_parsed_as_assignments_and_never_sourced() {
        // Quotes come off; comments and lines with no `=` are skipped; and
        // nothing is expanded, because expansion is not in the file's grammar.
        let text = "# a comment\n\nNAME=\"Ubuntu\"\nID=ubuntu\nnonsense\nID_LIKE='debian'\n";
        assert_eq!(
            read_an_os_release_file(text, "ID"),
            Some("ubuntu".to_string())
        );
        assert_eq!(
            read_an_os_release_file(text, "ID_LIKE"),
            Some("debian".to_string())
        );
        assert_eq!(read_an_os_release_file(text, "VERSION_ID"), None);
        // A value that looks like a shell expansion is carried as its own text
        // and nothing runs.
        assert_eq!(
            read_an_os_release_file("ID=$(whoami)\n", "ID"),
            Some("$(whoami)".to_string())
        );
    }

    #[test]
    fn a_machine_the_table_does_not_place_has_no_row_and_takes_no_setup() {
        assert!(what_the_kind_permits(MachineKind::TheTableDoesNotPlaceIt).is_none());
        assert!(!MachineKind::TheTableDoesNotPlaceIt.a_setup_may_run_on_it());
        let mut machine = a_machine(
            MachineKind::TheTableDoesNotPlaceIt,
            Some("ID=ubuntu\n"),
            None,
        );
        assert!(!read_before_anything_is_touched(&mut machine).a_run_may_be_composed());
    }

    #[test]
    fn a_containers_driver_is_never_touched_and_no_restart_is_ever_issued_there() {
        // Fact 48: all four routes closed, a restart is not a recovery action,
        // and a card that has stopped answering is terminal for that server.
        let container = what_the_kind_permits(MachineKind::AContainer).expect("a row");
        assert!(!container.the_driver_may_be_touched);
        assert!(!container.a_restart_may_be_issued);
        assert!(container.the_registration_step_needs_root);

        let systemd = what_the_kind_permits(MachineKind::ASystemdMachine).expect("a row");
        assert!(systemd.the_driver_may_be_touched);
        assert!(systemd.a_restart_may_be_issued);
        assert!(!systemd.the_registration_step_needs_root);
    }

    #[test]
    fn the_debian_route_enables_its_components_first_and_restarts_unconditionally() {
        let debian = TheDriverRoute::for_family(DistributionFamily::Debian);
        assert!(debian.the_components_must_be_enabled_first());
        assert!(debian.the_restart_is_not_optional());

        let ubuntu = TheDriverRoute::for_family(DistributionFamily::Ubuntu);
        assert!(!ubuntu.the_components_must_be_enabled_first());
        assert!(!ubuntu.the_restart_is_not_optional());
        // **Two from 2026-09-21**: the tool made reachable, then the tool.
        assert_eq!(ubuntu.the_steps().len(), 2);
        assert_eq!(
            ubuntu.the_steps()[0].the_command(),
            Some(crate::elevated::THE_UBUNTU_DRIVER_TOOL_INSTALL.plain)
        );
    }

    #[test]
    fn every_command_of_either_route_is_a_constant_carrying_no_reading() {
        // `Server protocol and transfers` 1.1, and E7's third bullet: every
        // byte of these is fixed at build time.
        for family in DistributionFamily::THE_SUPPORTED_PAIR {
            for step in TheDriverRoute::for_family(family).the_steps() {
                assert!(!step.what_it_does.is_empty());
                let Some(command) = step.the_command() else {
                    continue;
                };
                assert!(!command.as_str().is_empty());
                assert!(!command.as_str().contains('$'));
                assert!(!command.as_str().contains('`'));
            }
        }
        assert_eq!(THE_NON_INTERACTIVE_PROBE.as_str(), "sudo -n true");
    }

    /// **M3: the probe asks whether a command runs without a password, and never
    /// whether the login can authenticate.**
    ///
    /// **This is the test the fault of 2026-09-04 did not have.** The probe was
    /// `sudo -n -v`, the validate form, which on two providers' standard images
    /// answered *a password is owed* on machines where `sudo -n true` exited 0 —
    /// so Smoozy would have opened E2's password flow on the ordinary case. The
    /// old assertion only pinned the string, so the wrong question passed it.
    #[test]
    fn the_probe_runs_a_command_rather_than_validating() {
        let probe = THE_NON_INTERACTIVE_PROBE.as_str();
        // It is the elevating command's non-interactive form, which is M3's
        // own words and the half that is documented.
        assert!(probe.starts_with(crate::elevated::THE_NON_INTERACTIVE_PREFIX));
        let asked = &probe[crate::elevated::THE_NON_INTERACTIVE_PREFIX.len()..];
        // **A command, not a switch.** The validate form runs no command at
        // all, so it cannot answer whether one needs a password.
        assert!(!asked.is_empty());
        assert!(!asked.starts_with('-'));
        assert!(!probe.contains(" -v"));
        assert!(!probe.contains(" --validate"));
        // **And it is never the form E4 uses to authenticate a held password**,
        // which is a different question asked at a different moment.
        assert_ne!(probe, crate::elevation::THE_VALIDATE_FORM.as_str());
        // **Every byte fixed at build time** — `Server protocol and transfers`
        // 1.1, like every other command this crate sends.
        assert!(!probe.contains('$'));
        assert!(!probe.contains('`'));
    }

    #[test]
    fn the_one_step_with_no_established_command_says_so_rather_than_carrying_a_guess() {
        // Rule 2. M1b requires the components enabled and names no command,
        // and the evidence behind it records the state it found rather than
        // the act that changed it. The absence is a value here and is recorded
        // at `crate::arriving_later` with what breaks.
        let debian = TheDriverRoute::for_family(DistributionFamily::Debian);
        let without = debian
            .the_steps()
            .iter()
            .filter(|step| step.the_command().is_none())
            .count();
        assert_eq!(without, 1);
        assert!(debian.the_steps()[0].the_command().is_none());
    }

    #[test]
    fn no_second_account_on_the_machine_is_ever_a_route() {
        // M4. The cloud-init account holding passwordless root on one observed
        // image is not a route Smoozy has, however much it would help.
        assert!(!another_account_on_the_machine_is_a_route(true));
        const { assert!(THE_ONLY_LOGIN_IS_THE_CONNECTIONS_OWN) };
    }

    #[test]
    fn a_probe_that_fails_says_a_password_is_owed_and_nothing_more() {
        assert!(Probe::Succeeded.passwordless());
        assert!(!Probe::Failed.passwordless());
    }

    /// **A machine's three answers become one tuple, and nothing else does.**
    #[test]
    fn the_answers_of_an_ordinary_rented_box_become_a_tuple() {
        let tuple = the_tuple_the_machine_named(
            NotLinuxOutcome::ItNamedLinux,
            Some("x86_64\n"),
            Some("glibc 2.35\n"),
        );
        assert_eq!(
            tuple.as_read(),
            Some(&TargetTuple {
                operating_system_family: "linux".to_string(),
                processor_architecture: "x86_64".to_string(),
                abi_and_capabilities: "gnu".to_string(),
            })
        );
    }

    /// **The C library's release reaches nothing.**
    ///
    /// A binary built against one release of that library is not thereby known
    /// to run against an older one, and nobody on this project has read that
    /// library's own publication on the question — so the release is read and
    /// then used for nothing. **Two machines a decade apart compose the same
    /// tuple**, and whether either is supported is the manifest's answer rather
    /// than this reading's.
    #[test]
    fn two_releases_of_the_same_library_compose_the_same_tuple() {
        let old = the_tuple_the_machine_named(
            NotLinuxOutcome::ItNamedLinux,
            Some("x86_64"),
            Some("glibc 2.17"),
        );
        let new = the_tuple_the_machine_named(
            NotLinuxOutcome::ItNamedLinux,
            Some("x86_64"),
            Some("glibc 2.39"),
        );
        assert_eq!(old, new);
        assert!(old.as_read().is_some());
    }

    /// **The processor's name is carried as the machine's own word and is never
    /// spelled again.**
    ///
    /// A machine that names a processor no manifest of this build carries
    /// composes a tuple all the same, and the refusal is the selection's — which
    /// is what keeps the closest-processor fallback `Server protocol and
    /// transfers` B8 forbids out of reach here.
    #[test]
    fn another_processor_is_carried_whole_rather_than_translated() {
        let tuple = the_tuple_the_machine_named(
            NotLinuxOutcome::ItNamedLinux,
            Some("aarch64"),
            Some("glibc 2.39"),
        );
        assert_eq!(
            tuple
                .as_read()
                .map(|read| read.processor_architecture.as_str()),
            Some("aarch64")
        );
    }

    /// **An unreadable answer refuses rather than defaulting**, half by half.
    #[test]
    fn a_half_that_would_not_say_composes_no_tuple() {
        assert_eq!(
            the_tuple_the_machine_named(
                NotLinuxOutcome::NothingSmoozyCouldRead,
                Some("x86_64"),
                Some("glibc 2.35"),
            ),
            TheRentalsTuple::ItWouldNotSay(WhyNoTupleWasRead::ItWouldNotNameItsOperatingSystem)
        );
        assert_eq!(
            the_tuple_the_machine_named(
                NotLinuxOutcome::ItNamedSomethingElse,
                Some("x86_64"),
                Some("glibc 2.35"),
            ),
            TheRentalsTuple::ItWouldNotSay(WhyNoTupleWasRead::TheMachineIsNotALinuxMachine)
        );
        assert_eq!(
            the_tuple_the_machine_named(NotLinuxOutcome::ItNamedLinux, None, Some("glibc 2.35")),
            TheRentalsTuple::ItWouldNotSay(
                WhyNoTupleWasRead::ItWouldNotNameItsProcessorArchitecture
            )
        );
        assert_eq!(
            the_tuple_the_machine_named(NotLinuxOutcome::ItNamedLinux, Some("  \n"), None),
            TheRentalsTuple::ItWouldNotSay(
                WhyNoTupleWasRead::ItWouldNotNameItsProcessorArchitecture
            )
        );
        assert_eq!(
            the_tuple_the_machine_named(NotLinuxOutcome::ItNamedLinux, Some("x86_64"), None),
            TheRentalsTuple::ItWouldNotSay(WhyNoTupleWasRead::ItWouldNotNameItsCLibrary)
        );
    }

    /// **The asking utility's own word for a variable with no value here is not
    /// a library**, and it arrives at exit 0 like any answer.
    #[test]
    fn the_answer_that_names_nothing_is_never_read_as_a_library() {
        assert_eq!(
            the_tuple_the_machine_named(
                NotLinuxOutcome::ItNamedLinux,
                Some("x86_64"),
                Some(THE_ANSWER_THAT_NAMES_NOTHING),
            ),
            TheRentalsTuple::ItWouldNotSay(WhyNoTupleWasRead::ItWouldNotNameItsCLibrary)
        );
    }

    /// **A machine carrying another C library is never read as carrying this
    /// one**, and what it named is kept so the refusal can say it.
    #[test]
    fn a_different_c_library_is_refused_and_its_name_is_kept() {
        assert_eq!(
            the_tuple_the_machine_named(
                NotLinuxOutcome::ItNamedLinux,
                Some("x86_64"),
                Some("musl 1.2.4"),
            ),
            TheRentalsTuple::ItWouldNotSay(WhyNoTupleWasRead::ItNamedADifferentCLibrary {
                what_it_named: "musl".to_string(),
            })
        );
    }

    #[test]
    fn every_package_command_carries_the_bounded_wait_rather_than_the_default() {
        // Moved here from `recovery.rs` on 2026-09-19, when that file left for
        // the machine half of this module: it reads this file's own driver
        // route, which stays on the desktop side.
        use crate::recovery::{THE_LOCK_TIMEOUT_OPTION, THE_LOCK_TIMEOUT_SECONDS, THE_REPAIR_OPERATION};
        // S7b: apt's own default is 0 and the lock acquisition returns
        // immediately at that value.
        assert!(THE_LOCK_TIMEOUT_OPTION.contains(&THE_LOCK_TIMEOUT_SECONDS.to_string()));
        assert!(THE_REPAIR_OPERATION[1]
            .as_str()
            .contains(THE_LOCK_TIMEOUT_OPTION));
        // **The count is here because the selection is a text match, and a text
        // match that stops matching leaves a test that cannot fail.** Added
        // 2026-09-21: the loop picks the package commands out of the route by
        // the opening of their own text, so a route whose package command were
        // spelled any other way would be skipped in silence and this test would
        // go on passing over nothing.
        let mut checked = 0;
        for step in crate::kind::TheDriverRoute::for_family(crate::kind::DistributionFamily::Debian)
            .the_steps()
        {
            let Some(command) = step.the_command() else {
                continue;
            };
            if command.as_str().starts_with("apt-get") {
                assert!(command.as_str().contains(THE_LOCK_TIMEOUT_OPTION));
                checked += 1;
            }
        }
        assert!(checked > 0, "no package command of the route was reached at all");
    }
}
