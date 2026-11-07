//! The readings taken over the command channel, and the acting half of
//! [`crate::hardware`] and of the not-Linux check.
//!
//! **Every judgement lives next door and none is retaken here.** What this
//! module does is ask the machine, turn each answer into the value the
//! judgement takes, and hand it over. The two are split so that the whole of
//! §8.2's table and Q13's three outcomes stay testable at the desk with no
//! machine in sight, which is what [`crate::hardware`] and
//! [`crate::first_connection`] already are.
//!
//! **Every command here is a constant** — `Server protocol and transfers` §1.1:
//! *"no user path, package path, option, identifier or other value of user,
//! provider or machine origin is ever interpolated into a command string"*. The
//! type enforces it; the constants below are the whole set this module sends.
//!
//! **Classifying on an exit status is admissible here, and saying why is worth
//! a paragraph.** `Server protocol and transfers` §1.3 forbids it *"where 255
//! is one of the possibilities"* — Fact 7's collapse, where a transport failure
//! and a remote command's own failure both arrive at 255 and only the
//! accompanying text separates them. **That collapse is a property of the
//! command-line client's own exit status**, and `Servers` K1a runs none: a
//! transport failure here arrives as a [`CommandFailure`] and never as a
//! status. So every status this module reads was produced by the machine's own
//! shell, and each reading turns on a status the plans themselves observed.
//! **No message is parsed for a cause anywhere in this module**, which is the
//! half of §1.3 that binds regardless.
//!
//! **H10's free space is here and it does not come over the command channel.**
//! It is a **per-path** reading, and a path is exactly the kind of value §1.1
//! keeps out of a command string; `Server protocol and transfers` F2's first
//! route is the file-transfer subsystem's own `statvfs@openssh.com` extension,
//! where the value crosses as a protocol field. [`TakesAFreeSpaceReading`] is
//! that seam. **Nothing substitutes a whole-machine figure for it**, which is
//! F16's prohibition and H10's.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` F2, F16.
//! `Job files and outputs` L1. `Rented-server runtime` V6.

use smoozy_server_protocol::transport::ssh::CommandFailure;
use smoozy_server_protocol::transport::{CommandResult, CommandString};

use crate::first_connection::NotLinuxOutcome;
use crate::hardware::{
    BusScan, Card, CardMake, CgroupAnswer, DriverAnswer, DriverVersion, MachineKind,
    TheDriversOwnAnswer, WhereK1PlacesIt,
};

/// One `exec` on the command channel, so every reading below is testable at the
/// desk against answers written by hand.
///
/// **It is a trait rather than the connection itself for the same reason
/// `Keys` put its readings behind one**: the judgements are the part worth
/// testing, and a judgement that can only be reached through a socket is a
/// judgement nobody tests.
pub trait CommandChannel {
    fn run(
        &mut self,
        command: CommandString,
    ) -> impl std::future::Future<Output = Result<CommandResult, CommandFailure>> + Send;
}

impl TakesAFreeSpaceReading for smoozy_server_protocol::transport::ssh::Connection {
    /// **F2 route 1, over a subsystem channel opened for the reading.**
    ///
    /// **The channel is opened and closed around the reading rather than held**
    /// — §1.1's long-lived reading channel is `File Browser`'s at step 5, and a
    /// hardware reading takes one figure. **`Server protocol and transfers` T5
    /// counts this session against the connection's own pool** like every
    /// other, and the caller admits it — **that document's T5 and not this
    /// one's**, which is about transfers going with a machine that turns out to
    /// be a different one.
    ///
    /// **H10's own rule for a path that does not exist yet** is kept by asking
    /// at the deepest existing ancestor and recording the path the reading was
    /// actually taken against.
    async fn free_space(&mut self, path: &str) -> Option<crate::hardware::FreeSpace> {
        let channel = self.open_a_subsystem_channel().await.ok()?;
        let mut subsystem =
            smoozy_server_protocol::subsystem::client::Subsystem::negotiate(channel)
                .await
                .ok()?;
        let reading = smoozy_server_protocol::free_space::read_it_at_the_deepest_ancestor_that_exists(
            &mut subsystem,
            path,
        )
        .await
        .ok()?;
        Some(crate::hardware::FreeSpace {
            available_bytes: reading.available_bytes,
            the_path_it_was_taken_against: reading.the_path_it_was_taken_against,
        })
    }
}

impl CommandChannel for smoozy_server_protocol::transport::ssh::Connection {
    fn run(
        &mut self,
        command: CommandString,
    ) -> impl std::future::Future<Output = Result<CommandResult, CommandFailure>> + Send {
        self.run_one_command(command)
    }
}

/// **H10's reading, and it is not a command.**
///
/// `Server protocol and transfers` F2 route 1 takes it as a protocol field on
/// the file-transfer subsystem; route 2's carriage is that document's and is
/// **strictly additional** (its §12 item 3). It is a trait for the same reason
/// [`CommandChannel`] is: the judgement is worth testing and a judgement that
/// can only be reached through a socket is a judgement nobody tests.
pub trait TakesAFreeSpaceReading {
    fn free_space(
        &mut self,
        path: &str,
    ) -> impl std::future::Future<Output = Option<crate::hardware::FreeSpace>> + Send;
}

// ------------------------------------------------------------ the constants

/// **Q12.** One reading asking the machine to name its own operating system.
const NAME_YOUR_OPERATING_SYSTEM: CommandString = CommandString::constant("uname -s");

/// **`Rented-server runtime` K1's first row, first limb** — the container
/// marker file, which the evidence names beside the tool: *"Tell them apart
/// with `systemd-detect-virt` and `/.dockerenv` before choosing"* (Fact 1).
const IS_THE_CONTAINER_MARKER_THERE: CommandString =
    CommandString::constant("test -e /.dockerenv");

/// **`Rented-server runtime` K1's first row, second limb.** The `--container`
/// option is the tool's own
/// discriminator and is documented — systemd's own `systemd-detect-virt.xml`,
/// read from the project's source on 2026-09-03: *"Only detects container
/// virtualization (i.e. shared kernel virtualization)."*, with *"If a
/// virtualization technology is detected, 0 is returned, a non-zero code
/// otherwise."*
///
/// **Using the option rather than matching the printed name is a build
/// decision.** `Rented-server runtime` K1 says the table's limb is
/// *"`systemd-detect-virt` names a
/// container technology"*, which as written asks a builder to keep a list of
/// technology names in step with a tool that adds them. The option answers the
/// same question from the tool itself. **It also disposes of that clause's own
/// near-miss** — a container inside a VMware virtual machine, where the bare
/// reading says `docker` and `-v` says `vmware` — because a reading limited to
/// container virtualization cannot report the outer one.
const ARE_YOU_A_CONTAINER: CommandString =
    CommandString::constant("systemd-detect-virt --container");

/// **`Rented-server runtime` K1's second row — whether systemd is the
/// machine's first process — read for `Servers` H9a's absence row alone.**
/// Added 2026-09-21 on the build runner's ruling.
///
/// **It is the reading `Server Setup` already takes for that row**, the first
/// process's command name — documented, `proc_pid_comm(5)`, read 2026-09-04
/// and quoted at that crate's `THE_FIRST_PROCESSS_NAME`: *"This file exposes
/// the process's comm value—that is, the command name associated with the
/// process."* `Server Setup` reads the path over the file-transfer subsystem;
/// **this module has only the command channel**, so it asks for the same file
/// by a constant command. The two place one machine alike.
const NAME_YOUR_FIRST_PROCESS: CommandString = CommandString::constant("cat /proc/1/comm");

/// The name the first process carries where the machine was booted with
/// systemd — the same value `Server Setup` compares against.
const SYSTEMDS_OWN_NAME: &str = "systemd";

/// **H11 and H13.** The bus scan, by **vendor id and never by device class** —
/// *"so vendor id `0x10de` is the discriminator, never device class"*, because
/// counting display devices *"reports two cards on a one-card machine"*.
/// Observed: it *"returns an empty result with exit 0 for an absent vendor and
/// a populated line for a present one"*.
const SCAN_THE_BUS_FOR_THE_CARD_VENDOR: CommandString =
    CommandString::constant("lspci -d 10de:");

/// **H5, H5a and H6.** A per-card query, so the count is the number of rows —
/// never the count query, which *"returns the value once per card"* and gives
/// `22` on a two-card box to anyone parsing it whole. The four fields are
/// H6's: the unique identifier, the bus location, the make as a value, and the
/// total memory.
const ASK_THE_DRIVER_FOR_THE_CARDS: CommandString = CommandString::constant(
    "nvidia-smi --query-gpu=uuid,pci.bus_id,name,memory.total --format=csv,noheader",
);

/// **H11b: the driver's version.** One row per card, every row the same driver.
///
/// **Source — observed output recorded in this project.** This exact query was
/// run on every machine of the OptiX check
/// (`Audits/Build/Step 10c/Making OptiX work — what we found.md`, *What Smoozy
/// should do*, *On the reading*), and the same field asked beside others gave
/// `595.71.05` on a rented RTX A6000
/// (`Audits/Architecture/Six machines/findings-hyperstack.md`). The two format
/// flags are NVIDIA's own, cited where the card query is cited
/// (`Rented-server runtime`'s `acting/readings.rs`: *"csv - comma separated
/// values (MANDATORY); noheader - skip first line with column headers"*).
const ASK_THE_DRIVER_FOR_ITS_VERSION: CommandString =
    CommandString::constant("nvidia-smi --query-gpu=driver_version --format=csv,noheader");

/// **H11b: the CUDA version the driver reports**, from the driver's own summary.
///
/// **Source — observed output recorded in this project.** The summary's top
/// line carries it beside the driver's version:
/// `| NVIDIA-SMI 580.105.08   Driver Version: 580.105.08   CUDA Version: 13.0 |`
/// (`Preparation/Architecture/The rented machine.md`), and the same line read
/// `CUDA Version: 13.0` on a virtual machine
/// (`Audits/Architecture/Batch E/Findings — the virtual machine.md`) and
/// `CUDA Version: 13.2` on a rented RTX A6000
/// (`Audits/Architecture/Six machines/findings-hyperstack.md`). **No per-card
/// query field for it is cited anywhere in this project**, so none is used.
const ASK_THE_DRIVER_FOR_ITS_SUMMARY: CommandString = CommandString::constant("nvidia-smi");

/// **H11b's OptiX-library bullet: the machine's loader asked for the OptiX
/// library**, by `Rendering` L24b's own line — the one the render worker asks
/// before a job naming `OPTIX` is launched, with the loader's and `grep`'s
/// publications quoted at `smoozy_server_setup_machine::the_optix_library`.
/// `0` found, `1` not found, anything else no answer. Added 2026-11-02.
const LOOK_FOR_THE_OPTIX_LIBRARY: CommandString = CommandString::constant(
    smoozy_server_setup_machine::the_optix_library::THE_LOADERS_OWN_LOOKUP,
);

/// The label the summary puts in front of the CUDA version, as recorded above.
const THE_CUDA_VERSIONS_LABEL: &str = "CUDA Version:";

/// **H9a's newer pair.** Read, not assumed.
const THE_NEWER_MEMORY_LIMIT: CommandString =
    CommandString::constant("cat /sys/fs/cgroup/memory.max");

/// **H9a's older pair.** Read, not assumed.
const THE_OLDER_MEMORY_LIMIT: CommandString =
    CommandString::constant("cat /sys/fs/cgroup/memory/memory.limit_in_bytes");

/// **H9's `/proc` figure**, which is the whole machine and is used only where
/// the cgroup read the literal `max`.
const THE_MACHINES_OWN_MEMORY: CommandString = CommandString::constant("cat /proc/meminfo");

/// **H2's second half: the machine's own clock, read from the machine.**
///
/// H2 makes every hardware reading carry *"the moment it was taken **and the
/// machine's own clock it was taken on**"*, so the moment has to come from the
/// machine rather than from the desktop — which is the whole point of
/// `State, identity and recovery` D5's rule that readings on two clocks are
/// never subtracted.
///
/// **Seconds rather than anything finer, and that is a build decision.** The
/// finest granularity anything draws from either moment is a minute (H2a's
/// three wordings), and the nanosecond form of this command is one shell's
/// extension rather than a portable reading. **Nothing computes a duration from
/// it**, so precision buys nothing here.
const THE_MACHINES_OWN_CLOCK: CommandString = CommandString::constant("date +%s");

/// Every command this module sends, so a reader can see the whole set at once
/// and a test can assert that none of them carries a value.
pub const EVERY_COMMAND_THIS_MODULE_SENDS: [CommandString; 13] = [
    THE_MACHINES_OWN_CLOCK,
    NAME_YOUR_OPERATING_SYSTEM,
    IS_THE_CONTAINER_MARKER_THERE,
    ARE_YOU_A_CONTAINER,
    NAME_YOUR_FIRST_PROCESS,
    SCAN_THE_BUS_FOR_THE_CARD_VENDOR,
    ASK_THE_DRIVER_FOR_THE_CARDS,
    ASK_THE_DRIVER_FOR_ITS_VERSION,
    ASK_THE_DRIVER_FOR_ITS_SUMMARY,
    LOOK_FOR_THE_OPTIX_LIBRARY,
    THE_NEWER_MEMORY_LIMIT,
    THE_OLDER_MEMORY_LIMIT,
    THE_MACHINES_OWN_MEMORY,
];

// -------------------------------------------------------------- the readings

/// **Q1 step 4, and Q13's three outcomes.**
///
/// The value is compared against the one value that means Linux. **A machine
/// that answered nothing Smoozy could read claims nothing about what the
/// machine is** — no answer, a refused command, an unreadable one, all one
/// outcome, because Smoozy cannot tell the three apart and never claims to.
pub async fn read_the_operating_system(
    channel: &mut impl CommandChannel,
) -> NotLinuxOutcome {
    let Ok(result) = channel.run(NAME_YOUR_OPERATING_SYSTEM).await else {
        return crate::first_connection::read_the_operating_system(None);
    };
    if !result.ended_cleanly() {
        return crate::first_connection::read_the_operating_system(None);
    }
    match result.output_as_text().map(str::trim) {
        Some(named) if !named.is_empty() => {
            crate::first_connection::read_the_operating_system(Some(named))
        }
        _ => crate::first_connection::read_the_operating_system(None),
    }
}

/// **`Rented-server runtime` K1's table, for the one question this module
/// asks of it: container or not.**
///
/// The marker file is asked first because it is the limb that answers on a
/// container with no service manager at all, where the tool is simply absent
/// and would otherwise read as *not a container*. **That ordering is
/// `Rented-server runtime` K1's own**
/// — *"by `systemd-detect-virt` and by the presence of the container marker
/// file, in that order"* is a statement about which readings are taken, and the
/// table joins them with **or**, so either limb answering *container* settles
/// it.
///
/// **This performs the same table `Rented-server runtime` performs, and the two
/// must stay identical** — the shape H9a and V6 already use for the cgroup
/// rule. **That module's own reading has landed**: its table is
/// `smoozy_runtime::machine_kind` and the two readings it takes are
/// `smoozy_runtime::acting::machine_kind`. *(Corrected 2026-09-07: this said
/// the reading lands at step 6. It landed at step 8a, and nothing holds the two
/// copies together — a reader changing one has to change the other by hand.)*
pub async fn read_the_machine_kind(channel: &mut impl CommandChannel) -> MachineKind {
    if let Ok(marker) = channel.run(IS_THE_CONTAINER_MARKER_THERE).await {
        if marker.ended_cleanly() {
            return MachineKind::AContainer;
        }
    }
    if let Ok(tool) = channel.run(ARE_YOU_A_CONTAINER).await {
        if tool.ended_cleanly() {
            return MachineKind::AContainer;
        }
    }
    MachineKind::NotAContainer
}

/// **`Rented-server runtime` K1's whole table, for H9a's absence row alone.**
/// Added 2026-09-21 on the build runner's ruling.
///
/// The container limbs are [`read_the_machine_kind`]'s and are not asked
/// again; **the first process is asked only where they found no container**,
/// because a container keeps the absence whatever its first process is.
/// **An unreadable answer is never read as systemd** — it lands on K1's third
/// row, which keeps the absence — the same discipline `Server Setup` holds.
pub async fn where_k1_places_it(
    channel: &mut impl CommandChannel,
    kind: MachineKind,
) -> WhereK1PlacesIt {
    let systemd_is_the_first_process = match kind {
        MachineKind::AContainer => false,
        MachineKind::NotAContainer => match channel.run(NAME_YOUR_FIRST_PROCESS).await {
            Ok(result) if result.ended_cleanly() => result
                .output_as_text()
                .is_some_and(|name| name.trim() == SYSTEMDS_OWN_NAME),
            _ => false,
        },
    };
    WhereK1PlacesIt::from_the_two_readings(kind, systemd_is_the_first_process)
}

/// **H11's scan, in its three states.**
///
/// **The three are never collapsed**, because `empty` is the one state that
/// concludes *no card*, and a scan that did not run is not an empty one. On
/// every container of the multi-provider batch `lspci` was absent from the
/// image altogether, so *did not run* is the ordinary state on a whole machine
/// kind rather than a corner.
pub async fn scan_the_bus(channel: &mut impl CommandChannel) -> BusScan {
    let Ok(result) = channel.run(SCAN_THE_BUS_FOR_THE_CARD_VENDOR).await else {
        return BusScan::NoRoute;
    };
    // **Exit 0 is the only answering state the observation records.** Anything
    // else is the scan not having answered, which is H11's fifth row and never
    // its fourth.
    if !result.ended_cleanly() {
        return BusScan::NoRoute;
    }
    match result.output_as_text() {
        Some(text) if text.trim().is_empty() => BusScan::Empty,
        Some(_) => BusScan::Populated,
        None => BusScan::NoRoute,
    }
}

/// **Observed exit status for a driver with no card** — `No devices were
/// found`, exit 6 (Fact 46).
const NO_DEVICES_WERE_FOUND: u32 = 6;

/// **H11's driver column, in its three states, and H5's count rule kept by
/// construction.**
///
/// The count is never read from this query's own output as a number: what
/// comes back is one row per card, and the count is however many rows there
/// are. **Nothing in this module reads a count any other way.**
pub async fn ask_the_driver(channel: &mut impl CommandChannel) -> DriverAnswer {
    let Ok(result) = channel.run(ASK_THE_DRIVER_FOR_THE_CARDS).await else {
        return DriverAnswer::Absent;
    };
    if result.status == Some(NO_DEVICES_WERE_FOUND) {
        return DriverAnswer::NoDevicesWereFound;
    }
    if !result.ended_cleanly() {
        return DriverAnswer::Absent;
    }
    let Some(text) = result.output_as_text() else {
        return DriverAnswer::Absent;
    };
    let cards: Vec<Card> = text.lines().filter_map(one_card).collect();
    if cards.is_empty() {
        // A clean exit with no rows is a driver that answered nothing. It is
        // **not** read as *no card*: H11 forbids that conclusion from anything
        // but the scan's own `empty`.
        return DriverAnswer::Absent;
    }
    DriverAnswer::Answered(cards)
}

/// **H11b: the driver's version and the CUDA version it reports, asked only
/// where the driver already answered with its cards.**
///
/// **Neither can fail the reading.** A query that does not answer, or answers
/// with nothing this can read, leaves its value absent; the cards already
/// read are untouched, because they were read before this is asked and this
/// returns nothing about them.
pub async fn ask_the_drivers_versions(
    channel: &mut impl CommandChannel,
) -> (Option<DriverVersion>, Option<String>) {
    let version = match channel.run(ASK_THE_DRIVER_FOR_ITS_VERSION).await {
        Ok(result) if result.ended_cleanly() => result
            .output_as_text()
            .and_then(|text| text.lines().find(|line| !line.trim().is_empty()).map(str::to_string))
            .and_then(|line| DriverVersion::parse(&line)),
        _ => None,
    };
    let cuda_version = match channel.run(ASK_THE_DRIVER_FOR_ITS_SUMMARY).await {
        Ok(result) if result.ended_cleanly() => result
            .output_as_text()
            .and_then(|text| the_cuda_version_in(&text)),
        _ => None,
    };
    (version, cuda_version)
}

/// The figure after the summary's `CUDA Version:` label, up to the next space
/// or column rule. **It must look like a version** — dot-separated numbers — or
/// it is absent.
fn the_cuda_version_in(summary: &str) -> Option<String> {
    let (_, after) = summary.split_once(THE_CUDA_VERSIONS_LABEL)?;
    let figure = after
        .trim_start()
        .split(|c: char| c.is_whitespace() || c == '|')
        .next()?;
    DriverVersion::parse(figure)?;
    Some(figure.to_string())
}

/// One row of the per-card query — H6's four fields.
///
/// **A row that is not four fields is not a card**, and is dropped rather than
/// half-read. **A memory figure whose unit the tool did not name is an
/// absence** (H8's shape), never a number this module chose a unit for.
///
/// **The third field is the driver's own name for the card and is not the
/// make.** H7's make is a key `Rendering` L23's table is looked up by, and it
/// is established by the route — this query is NVIDIA's own tool, so every
/// card it names is keyed [`crate::hardware::NVIDIA`]. The name rides beside the
/// key to be shown. **Until 2026-09-26 the name was put where the key goes**,
/// and on step 10c's sixth pass on Vast every job set to use the cards ended
/// before the render on an NVIDIA machine because the table had no row for
/// *NVIDIA GeForce GTX 1650*.
fn one_card(row: &str) -> Option<Card> {
    let mut fields = row.split(',').map(str::trim);
    let unique_identifier = fields.next()?.to_string();
    let bus_location = fields.next()?.to_string();
    let the_drivers_name_for_the_card = fields.next()?;
    let memory = fields.next()?;
    if unique_identifier.is_empty()
        || bus_location.is_empty()
        || the_drivers_name_for_the_card.is_empty()
    {
        return None;
    }
    Some(Card {
        unique_identifier,
        bus_location,
        make: CardMake::from_the_drivers_own_query(the_drivers_name_for_the_card),
        total_memory_bytes: mebibytes(memory),
    })
}

/// A figure the tool wrote with its own unit beside it.
///
/// **The unit is read rather than assumed.** A value with no unit, or with one
/// this reading does not know, is `None` — which H8 already makes a complete
/// reading with a stated absence rather than a failed one.
fn mebibytes(value: &str) -> Option<u64> {
    let (figure, unit) = value.trim().split_once(' ')?;
    if unit != "MiB" {
        return None;
    }
    figure.parse::<u64>().ok()?.checked_mul(1024 * 1024)
}

/// **H9a's cgroup reading, at whichever pair is asked.**
///
/// **The fallback's trigger is the literal `max` and nothing else.** A pair
/// that is absent, or present and empty, is no route — and no route at both
/// pairs is a stated absence rather than the machine's own figure.
pub async fn read_a_cgroup_pair(
    channel: &mut impl CommandChannel,
    which: WhichCgroupPair,
) -> CgroupAnswer {
    let command = match which {
        WhichCgroupPair::TheNewer => THE_NEWER_MEMORY_LIMIT,
        WhichCgroupPair::TheOlder => THE_OLDER_MEMORY_LIMIT,
    };
    let Ok(result) = channel.run(command).await else {
        return CgroupAnswer::NoRoute;
    };
    if !result.ended_cleanly() {
        return CgroupAnswer::NoRoute;
    }
    match result.output_as_text().map(str::trim) {
        Some("max") => CgroupAnswer::TheLiteralMax,
        Some(text) => match text.parse::<u64>() {
            Ok(figure) => CgroupAnswer::AFigure(figure),
            Err(_) => CgroupAnswer::NoRoute,
        },
        None => CgroupAnswer::NoRoute,
    }
}

/// Which of H9a's two path pairs a reading is being taken at. **Both are read
/// and neither is assumed.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhichCgroupPair {
    /// `memory.max`, with `cpu.max` beside it.
    TheNewer,
    /// `memory.limit_in_bytes`, with `cpu.cfs_quota_us` and `cpu.cfs_period_us`
    /// beside it.
    TheOlder,
}

/// **The machine's own moment**, taken as a value.
///
/// **A machine that will not name its own clock has no moment**, and the
/// caller does not substitute the desktop's — which is the one thing
/// `State, identity and recovery` D5 makes meaningless. H4's *a reading
/// replaces it whole* then has nothing to replace the held reading with, and
/// that is the correct outcome rather than a reading with a borrowed moment in
/// it.
///
/// **A machine that names a moment this computer cannot hold has no moment
/// either, and it used to abort the application instead. Corrected 2026-09-07**,
/// found beside the same fault at `Access`'s `clock::moment_from_milliseconds`
/// after hosted run `34091883056`.
///
/// The seconds come off a rented machine over the network as text, and any
/// number a 64-bit count holds parses — a machine whose clock is set absurdly
/// far ahead, or one whose `date` answers in some unit other than seconds.
/// **On Windows a moment is a `FILETIME`, 100-nanosecond intervals since 1601 in
/// a signed 64-bit number, and it runs out around the year 30828**; adding more
/// than that panicked, and the panic was in a desktop crate on a value the
/// desktop does not choose. **This is the same class as the `Access` fault and
/// it would have failed on Windows the same day.**
///
/// **The answer here is the one this function already gives**, and no clamp is
/// wanted: a machine naming a moment this computer cannot hold has not named a
/// moment, so nothing came back — which is what both callers already handle.
pub async fn read_the_machines_own_clock(
    channel: &mut impl CommandChannel,
) -> Option<std::time::SystemTime> {
    let result = channel.run(THE_MACHINES_OWN_CLOCK).await.ok()?;
    if !result.ended_cleanly() {
        return None;
    }
    let seconds: u64 = result.output_as_text()?.trim().parse().ok()?;
    std::time::UNIX_EPOCH.checked_add(std::time::Duration::from_secs(seconds))
}

/// **H9's `/proc` figure**, read only to be handed to
/// [`crate::hardware::read_the_machines_memory`], which uses it where the
/// cgroup said the literal `max`, and — from 2026-09-21, on the build runner's ruling — where no cgroup
/// route answers on a machine with its own kernel. Both are a machine nothing
/// caps, where the whole-machine figure is the right one.
///
/// **A machine that will not name its own memory answers *nothing came back*,
/// never a zero.** The request may not reach the machine, the machine may
/// answer badly, or it may name its units in a way this reading does not know;
/// **all three are one absence and none of them is a figure.** A zero handed on
/// as though it were a fact reaches the server list as a machine with no memory
/// at all, and the user reads that and picks another machine — which is Smoozy
/// saying something it does not know, and Rule 8 makes that a fault whatever
/// caused it. **This is the shape [`read_the_machines_own_clock`] next door
/// already uses**, for the same reason.
pub async fn read_the_machines_own_memory(channel: &mut impl CommandChannel) -> Option<u64> {
    let result = channel.run(THE_MACHINES_OWN_MEMORY).await.ok()?;
    if !result.ended_cleanly() {
        return None;
    }
    let text = result.output_as_text()?;
    // The file names its own unit on the line, so the unit is read rather than
    // assumed — the same rule the per-card figure follows.
    for line in text.lines() {
        let Some(rest) = line.strip_prefix("MemTotal:") else {
            continue;
        };
        let mut fields = rest.split_whitespace();
        let (Some(figure), Some(unit)) = (fields.next(), fields.next()) else {
            return None;
        };
        if unit != "kB" {
            return None;
        }
        return figure
            .parse::<u64>()
            .ok()
            .and_then(|figure| figure.checked_mul(1024));
    }
    None
}

/// **§8's whole reading, taken over the command channel** — H18's *a fresh
/// reading is taken* performed.
///
/// **Every judgement in it is [`crate::hardware`]'s and none is retaken here.**
/// What this does is ask the machine in the order the judgements need — the
/// machine kind first, because H12 makes the *empty* row unavailable on a
/// container — and hand the answers over.
///
/// **The reading is one act and lands whole** (H4): nothing merges a fresh
/// reading into an old one, and **no part of a record outlives the reading it
/// came from**, so a machine that would not name its own clock produces no
/// reading at all rather than one with a borrowed moment in it.
///
/// **H10's free space is taken where there is a path to take it against, and
/// is an absence where there is not.** The path is *"the path a job's folders
/// sit under"*, which `Job files and outputs` **L1** records at hand-over —
/// *"the root is chosen once per installation and never re-chosen … recorded
/// against the server row on the desktop side as a machine-derived value"*. **A
/// machine that has never been set up has no such root**, so the figure is
/// absent on it, and that is the reading being honest rather than a gap.
/// **Nothing is substituted for it** (H10, F16).
pub async fn take_the_hardware_reading(
    channel: &mut (impl CommandChannel + TakesAFreeSpaceReading),
    server: &str,
    read_under: Option<String>,
    the_recorded_root: Option<&str>,
    received_here_at: std::time::SystemTime,
) -> Option<crate::hardware::HardwareReading> {
    let kind = read_the_machine_kind(channel).await;
    let scan = scan_the_bus(channel).await;
    let driver = ask_the_driver(channel).await;
    // **H11b: asked only where the driver answered**, and after the cards are
    // in hand, so nothing it does can touch them.
    let the_drivers_versions = match driver {
        DriverAnswer::Answered(_) => Some(ask_the_drivers_versions(channel).await),
        _ => None,
    };
    // **H11b's OptiX-library bullet, added 2026-11-02: asked only where the
    // driver answered, after the cards are in hand.** Only the loader's *not
    // found* is carried; a lookup that gave no answer says nothing.
    let the_optix_library_is_missing = match driver {
        DriverAnswer::Answered(_) => the_loader_does_not_find_the_optix_library(channel).await,
        _ => false,
    };
    let cards = crate::hardware::read_the_cards(scan, driver, kind);

    let newer = read_a_cgroup_pair(channel, WhichCgroupPair::TheNewer).await;
    let older = read_a_cgroup_pair(channel, WhichCgroupPair::TheOlder).await;
    // **H9's `/proc` figure is read once and used only where nothing caps the
    // machine** — the cgroup said the literal `max`, or, from 2026-09-21, no cgroup route
    // answered on a machine K1 places as its own kernel. **The kind is the one
    // taken first above**, and the first process is asked only beside it.
    let placed = where_k1_places_it(channel, kind).await;
    let machine_memory = crate::hardware::read_the_machines_memory(
        placed,
        newer,
        older,
        read_the_machines_own_memory(channel).await,
    );

    let free_space = match the_recorded_root {
        Some(root) => channel.free_space(root).await,
        None => None,
    };

    let on_the_machines_clock = read_the_machines_own_clock(channel).await?;
    Some(crate::hardware::HardwareReading::taken(
        cards,
        machine_memory,
        free_space,
        on_the_machines_clock,
        server,
        received_here_at,
        read_under,
    )
    .with_the_driver(match the_drivers_versions {
        Some((Some(version), cuda_version)) => Some(TheDriversOwnAnswer {
            version,
            cuda_version,
            where_k1_placed_the_machine: placed,
        }),
        _ => None,
    })
    .with_the_optix_library_missing(the_optix_library_is_missing))
}

/// **H11b's OptiX-library bullet: whether the machine's loader does not find
/// the OptiX library.** `Rendering` L24b's own line, read by its own reading
/// of the status — so *not found* here is exactly what would end a cards job
/// there. **A lookup the machine did not answer, or answered with anything but
/// `0` or `1`, is not *not found*.** Added 2026-11-02.
async fn the_loader_does_not_find_the_optix_library(channel: &mut impl CommandChannel) -> bool {
    use smoozy_server_setup_machine::the_optix_library::{what_the_status_says, WhatTheLoaderFound};
    match channel.run(LOOK_FOR_THE_OPTIX_LIBRARY).await {
        Ok(result) => {
            what_the_status_says(result.status.map(|status| status as i32))
                == WhatTheLoaderFound::NoLibrary
        }
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hardware::{read_the_cards, Cards, WhyUnreadable};
    use std::collections::HashMap;

    /// A machine that answers whatever it was told to answer, so every reading
    /// above is exercised without a socket.
    #[derive(Default)]
    struct Answers(HashMap<&'static str, CommandResult>);

    impl Answers {
        fn to(mut self, command: CommandString, status: u32, output: &str) -> Answers {
            self.0.insert(
                command.as_str(),
                CommandResult {
                    output: output.as_bytes().to_vec(),
                    complaint: Vec::new(),
                    status: Some(status),
                },
            );
            self
        }
    }

    impl CommandChannel for Answers {
        async fn run(
            &mut self,
            command: CommandString,
        ) -> Result<CommandResult, CommandFailure> {
            self.0
                .get(command.as_str())
                .cloned()
                // A command with no answer arranged is a command the machine
                // did not answer, which is a real state of every reading here.
                .ok_or(CommandFailure::TheRequestDidNotReachTheMachine)
        }
    }

    /// **No free-space reading is taken by these tests**: they pass no recorded
    /// root, so the sequence never asks, and nothing is invented here.
    impl TakesAFreeSpaceReading for Answers {
        async fn free_space(&mut self, _path: &str) -> Option<crate::hardware::FreeSpace> {
            None
        }
    }

    #[tokio::test]
    async fn the_machine_naming_linux_carries_the_sequence_on() {
        let mut machine = Answers::default().to(NAME_YOUR_OPERATING_SYSTEM, 0, "Linux\n");
        assert_eq!(
            read_the_operating_system(&mut machine).await,
            NotLinuxOutcome::ItNamedLinux
        );
    }

    #[tokio::test]
    async fn a_machine_naming_something_else_is_said_plainly() {
        let mut machine = Answers::default().to(NAME_YOUR_OPERATING_SYSTEM, 0, "Darwin\n");
        let outcome = read_the_operating_system(&mut machine).await;
        assert_eq!(outcome, NotLinuxOutcome::ItNamedSomethingElse);
        assert!(outcome.claims_what_the_machine_is());
    }

    #[tokio::test]
    async fn a_machine_that_answered_nothing_claims_nothing() {
        // Q13's third outcome, and the three causes it cannot tell apart: no
        // answer, a refused command, an unreadable one.
        for machine in [
            Answers::default(),
            Answers::default().to(NAME_YOUR_OPERATING_SYSTEM, 127, ""),
            Answers::default().to(NAME_YOUR_OPERATING_SYSTEM, 0, "   \n"),
        ] {
            let mut machine = machine;
            let outcome = read_the_operating_system(&mut machine).await;
            assert_eq!(outcome, NotLinuxOutcome::NothingSmoozyCouldRead);
            assert!(!outcome.claims_what_the_machine_is());
        }
    }

    #[tokio::test]
    async fn the_marker_file_answers_on_a_container_with_no_service_manager() {
        // The case the ordering exists for: `systemd-detect-virt` is absent,
        // so the tool limb reads *not a container* and the marker settles it.
        let mut machine = Answers::default().to(IS_THE_CONTAINER_MARKER_THERE, 0, "");
        assert_eq!(
            read_the_machine_kind(&mut machine).await,
            MachineKind::AContainer
        );
    }

    #[tokio::test]
    async fn the_tool_answers_on_a_container_with_no_marker_file() {
        let mut machine = Answers::default()
            .to(IS_THE_CONTAINER_MARKER_THERE, 1, "")
            .to(ARE_YOU_A_CONTAINER, 0, "docker\n");
        assert_eq!(
            read_the_machine_kind(&mut machine).await,
            MachineKind::AContainer
        );
    }

    #[tokio::test]
    async fn a_machine_that_is_neither_reads_as_not_a_container() {
        let mut machine = Answers::default()
            .to(IS_THE_CONTAINER_MARKER_THERE, 1, "")
            .to(ARE_YOU_A_CONTAINER, 1, "none\n");
        assert_eq!(
            read_the_machine_kind(&mut machine).await,
            MachineKind::NotAContainer
        );
    }

    #[tokio::test]
    async fn an_empty_scan_and_a_scan_that_did_not_run_are_two_readings() {
        // The one distinction H11's fifth row exists for. Collapsing them
        // concludes *no card* on a container image with no `pciutils`.
        let mut answered = Answers::default().to(SCAN_THE_BUS_FOR_THE_CARD_VENDOR, 0, "\n");
        assert_eq!(scan_the_bus(&mut answered).await, BusScan::Empty);

        let mut absent = Answers::default().to(SCAN_THE_BUS_FOR_THE_CARD_VENDOR, 127, "");
        assert_eq!(scan_the_bus(&mut absent).await, BusScan::NoRoute);

        let mut populated = Answers::default().to(
            SCAN_THE_BUS_FOR_THE_CARD_VENDOR,
            0,
            "01:00.0 VGA compatible controller: NVIDIA Corporation Device [10de:2684]\n",
        );
        assert_eq!(scan_the_bus(&mut populated).await, BusScan::Populated);
    }

    #[tokio::test]
    async fn a_missing_scan_never_reaches_the_no_card_row() {
        // §16 item 2's *what holds*, driven from the reading side rather than
        // asserted on the judgement's own inputs.
        let mut machine = Answers::default()
            .to(SCAN_THE_BUS_FOR_THE_CARD_VENDOR, 127, "")
            .to(IS_THE_CONTAINER_MARKER_THERE, 1, "")
            .to(ARE_YOU_A_CONTAINER, 1, "none\n");
        let scan = scan_the_bus(&mut machine).await;
        let driver = ask_the_driver(&mut machine).await;
        let kind = read_the_machine_kind(&mut machine).await;
        assert_eq!(
            read_the_cards(scan, driver, kind),
            Cards::CouldNotBeEstablished
        );
    }

    #[tokio::test]
    async fn the_count_is_the_rows_and_two_identical_cards_are_told_apart() {
        // H5's trap and H6's answer to it, on the two-card box's own shape.
        let mut machine = Answers::default().to(
            ASK_THE_DRIVER_FOR_THE_CARDS,
            0,
            "GPU-aaaa, 00000000:01:00.0, NVIDIA GeForce RTX 4090, 24564 MiB\n\
             GPU-bbbb, 00000000:02:00.0, NVIDIA GeForce RTX 4090, 24564 MiB\n",
        );
        let DriverAnswer::Answered(cards) = ask_the_driver(&mut machine).await else {
            panic!("the driver answered");
        };
        assert_eq!(cards.len(), 2);
        assert_ne!(cards[0].unique_identifier, cards[1].unique_identifier);
        assert_ne!(cards[0].bus_location, cards[1].bus_location);
        assert_eq!(cards[0].make, cards[1].make, "both cards are the same model");
        assert_eq!(cards[0].total_memory_bytes, Some(24564 * 1024 * 1024));
    }

    #[tokio::test]
    async fn a_memory_figure_with_no_unit_named_is_an_absence() {
        // H8's shape. The alternative is choosing a unit for the tool, which
        // is inventing a number.
        let mut machine = Answers::default().to(
            ASK_THE_DRIVER_FOR_THE_CARDS,
            0,
            "GPU-aaaa, 00000000:01:00.0, NVIDIA A6000, 49140\n",
        );
        let DriverAnswer::Answered(cards) = ask_the_driver(&mut machine).await else {
            panic!("the driver answered");
        };
        assert_eq!(cards[0].total_memory_bytes, None);
        // **H7: the make is the key the table is looked up by, and the
        // driver's own name for the card rides beside it** — never the one in
        // place of the other.
        assert_eq!(cards[0].make.as_key(), crate::hardware::NVIDIA);
        assert_eq!(cards[0].make.the_drivers_name_for_the_card(), "NVIDIA A6000");
    }

    #[tokio::test]
    async fn the_driver_answering_with_none_is_not_the_driver_being_absent() {
        // H11's two middle rows, which take the same conclusion by different
        // routes and must still be named apart.
        let mut none = Answers::default().to(
            ASK_THE_DRIVER_FOR_THE_CARDS,
            NO_DEVICES_WERE_FOUND,
            "No devices were found\n",
        );
        assert_eq!(
            ask_the_driver(&mut none).await,
            DriverAnswer::NoDevicesWereFound
        );

        let mut absent = Answers::default();
        assert_eq!(ask_the_driver(&mut absent).await, DriverAnswer::Absent);

        let scan = BusScan::Populated;
        assert_eq!(
            read_the_cards(scan, DriverAnswer::NoDevicesWereFound, MachineKind::AContainer),
            Cards::PresentAndUnreadable(WhyUnreadable::TheDriverAnsweredWithNone)
        );
        assert_eq!(
            read_the_cards(scan, DriverAnswer::Absent, MachineKind::AContainer),
            Cards::PresentAndUnreadable(WhyUnreadable::TheDriverIsAbsent)
        );
    }

    #[tokio::test]
    async fn the_literal_max_is_the_only_thing_that_falls_back() {
        let mut literal = Answers::default().to(THE_NEWER_MEMORY_LIMIT, 0, "max\n");
        assert_eq!(
            read_a_cgroup_pair(&mut literal, WhichCgroupPair::TheNewer).await,
            CgroupAnswer::TheLiteralMax
        );

        let mut figure = Answers::default().to(THE_NEWER_MEMORY_LIMIT, 0, "16050000000\n");
        assert_eq!(
            read_a_cgroup_pair(&mut figure, WhichCgroupPair::TheNewer).await,
            CgroupAnswer::AFigure(16_050_000_000)
        );

        // The observed case: the v1 paths do not exist on a v2 host, and an
        // empty answer is no route rather than a figure of zero.
        let mut empty = Answers::default().to(THE_OLDER_MEMORY_LIMIT, 0, "\n");
        assert_eq!(
            read_a_cgroup_pair(&mut empty, WhichCgroupPair::TheOlder).await,
            CgroupAnswer::NoRoute
        );

        let mut absent = Answers::default();
        assert_eq!(
            read_a_cgroup_pair(&mut absent, WhichCgroupPair::TheOlder).await,
            CgroupAnswer::NoRoute
        );
    }

    #[tokio::test]
    async fn the_machines_own_memory_reads_the_unit_the_file_names() {
        let mut machine = Answers::default().to(
            THE_MACHINES_OWN_MEMORY,
            0,
            "MemTotal:       16321364 kB\nMemFree:         1234 kB\n",
        );
        assert_eq!(
            read_the_machines_own_memory(&mut machine).await,
            Some(16_321_364 * 1024)
        );

        let mut odd = Answers::default().to(THE_MACHINES_OWN_MEMORY, 0, "MemTotal: 16 GB\n");
        assert_eq!(read_the_machines_own_memory(&mut odd).await, None);
    }

    #[tokio::test]
    async fn a_machine_that_will_not_name_its_memory_answers_nothing_and_never_a_zero() {
        // The three causes are one absence, and none of them is a figure: the
        // request did not reach the machine, the machine answered badly, and
        // the unit was not one this reading knows.
        let mut unreached = Answers::default();
        assert_eq!(read_the_machines_own_memory(&mut unreached).await, None);

        let mut refused = Answers::default().to(THE_MACHINES_OWN_MEMORY, 1, "");
        assert_eq!(read_the_machines_own_memory(&mut refused).await, None);

        let mut nonsense = Answers::default().to(THE_MACHINES_OWN_MEMORY, 0, "MemFree: 12 kB\n");
        assert_eq!(read_the_machines_own_memory(&mut nonsense).await, None);
    }

    #[tokio::test]
    async fn an_uncapped_machine_that_would_not_name_its_memory_says_so_rather_than_none_at_all() {
        // H9a's third row reached from the reading side. Read as a zero, the
        // server list would show a machine with no memory and the user would
        // pick a different one to render on.
        let mut machine = Answers::default().to(THE_NEWER_MEMORY_LIMIT, 0, "max\n");
        let newer = read_a_cgroup_pair(&mut machine, WhichCgroupPair::TheNewer).await;
        let older = read_a_cgroup_pair(&mut machine, WhichCgroupPair::TheOlder).await;
        let own = read_the_machines_own_memory(&mut machine).await;
        assert_eq!(own, None);
        assert_eq!(
            crate::hardware::read_the_machines_memory(
                WhereK1PlacesIt::ItsOwnKernel,
                newer,
                older,
                own
            ),
            crate::hardware::MachineMemory::CouldNotBeEstablished
        );
    }

    /// A machine answering the whole of §8's reading as a plain virtual machine
    /// does: no container, systemd first, and — the case the ruling was made on —
    /// **neither control-group memory file**, observed at the root of the tree
    /// on a rental on 2026-09-21.
    fn a_virtual_machine_with_no_cgroup_file() -> Answers {
        Answers::default()
            .to(IS_THE_CONTAINER_MARKER_THERE, 1, "")
            .to(ARE_YOU_A_CONTAINER, 1, "")
            .to(NAME_YOUR_FIRST_PROCESS, 0, "systemd\n")
            .to(THE_NEWER_MEMORY_LIMIT, 1, "")
            .to(THE_OLDER_MEMORY_LIMIT, 1, "")
            .to(THE_MACHINES_OWN_MEMORY, 0, "MemTotal:       65805484 kB\n")
            .to(THE_MACHINES_OWN_CLOCK, 0, "1770000000\n")
    }

    async fn the_memory_it_reads(mut machine: Answers) -> crate::hardware::MachineMemory {
        take_the_hardware_reading(&mut machine, "server", None, None, std::time::UNIX_EPOCH)
            .await
            .expect("the machine named its clock")
            .machine_memory
    }

    /// **The ruled case: a machine with its own kernel and no control-group route shows its
    /// own figure**, taken through the whole reading rather than the judgement
    /// alone, so the kind that reaches the judgement is the one read first.
    #[tokio::test]
    async fn a_machine_with_its_own_kernel_and_no_cgroup_route_reads_its_own_memory() {
        assert_eq!(
            the_memory_it_reads(a_virtual_machine_with_no_cgroup_file()).await,
            crate::hardware::MachineMemory::TheWholeMachine(65_805_484 * 1024)
        );
    }

    /// **The container keeps the absence** — the host's figure is what `/proc`
    /// holds there, and a systemd first process does not change that.
    #[tokio::test]
    async fn a_container_with_no_cgroup_route_still_could_not_be_established() {
        for container in [
            a_virtual_machine_with_no_cgroup_file().to(IS_THE_CONTAINER_MARKER_THERE, 0, ""),
            a_virtual_machine_with_no_cgroup_file().to(ARE_YOU_A_CONTAINER, 0, "lxc\n"),
        ] {
            assert_eq!(
                the_memory_it_reads(container).await,
                crate::hardware::MachineMemory::CouldNotBeEstablished
            );
        }
    }

    /// **A machine K1 does not place is not taken to run its own kernel**: a
    /// first process that is not systemd, or one that would not be named.
    #[tokio::test]
    async fn a_machine_whose_first_process_is_not_systemd_still_could_not_be_established() {
        for machine in [
            a_virtual_machine_with_no_cgroup_file().to(NAME_YOUR_FIRST_PROCESS, 0, "bash\n"),
            a_virtual_machine_with_no_cgroup_file().to(NAME_YOUR_FIRST_PROCESS, 1, ""),
        ] {
            assert_eq!(
                the_memory_it_reads(machine).await,
                crate::hardware::MachineMemory::CouldNotBeEstablished
            );
        }
    }

    /// **A real cap is still the allocation on a machine with its own kernel.**
    #[tokio::test]
    async fn a_machine_with_its_own_kernel_and_a_real_cgroup_figure_is_the_allocation() {
        let capped =
            a_virtual_machine_with_no_cgroup_file().to(THE_NEWER_MEMORY_LIMIT, 0, "16051527680\n");
        assert_eq!(
            the_memory_it_reads(capped).await,
            crate::hardware::MachineMemory::TheAllocation(16_051_527_680)
        );
    }

    /// **A machine naming a moment this computer cannot hold has named no
    /// moment, and it used to abort the application.** Found 2026-09-07 beside
    /// the `Access` fault hosted run `34091883056` came back red on Windows
    /// with.
    ///
    /// The seconds arrive as text off a rented machine and any 64-bit count
    /// parses. **Windows keeps a moment as intervals since 1601 in a signed
    /// 64-bit number and runs out around the year 30828**, so the values below
    /// crashed there; this desk and Linux keep 64-bit seconds and run out much
    /// later, which is why only the widest of them shows the fault here.
    #[tokio::test]
    async fn a_machine_naming_a_moment_this_computer_cannot_hold_names_no_moment() {
        // The ordinary case still reads.
        let mut ordinary = Answers::default().to(THE_MACHINES_OWN_CLOCK, 0, "1757250000\n");
        assert_eq!(
            read_the_machines_own_clock(&mut ordinary).await,
            Some(std::time::UNIX_EPOCH + std::time::Duration::from_secs(1_757_250_000))
        );

        // A machine whose `date` answered in some unit that is not seconds, and
        // one whose clock is set as far ahead as the reading can carry. **The
        // first of these is inside what this desk holds and outside what
        // Windows holds** — only a hosted Windows run can show that one — and
        // the second is outside both, which is what this desk can prove.
        for absurd in ["1757250000000000000", "18446744073709551615"] {
            let mut machine = Answers::default().to(THE_MACHINES_OWN_CLOCK, 0, absurd);
            let read = read_the_machines_own_clock(&mut machine).await;
            match std::time::UNIX_EPOCH
                .checked_add(std::time::Duration::from_secs(absurd.parse().unwrap()))
            {
                // A system wide enough to hold it: it is a moment, and the
                // caller decides what a machine with a clock like that means.
                Some(held) => assert_eq!(read, Some(held)),
                // A system that is not: nothing came back, and nothing panicked.
                None => assert_eq!(read, None),
            }
        }
    }

    #[test]
    fn no_command_this_module_sends_carries_a_value() {
        // §1.1, asserted over the whole set rather than at each call. The type
        // is what enforces it; this is the reader's list.
        for command in EVERY_COMMAND_THIS_MODULE_SENDS {
            let text = command.as_str();
            assert!(!text.contains('$'), "{text} interpolates");
            assert!(!text.contains('`'), "{text} substitutes");
        }
    }

    // ---- H11b: the driver's version and the CUDA version it reports ----

    /// The driver's summary as recorded on a rented RTX 4090
    /// (`Preparation/Architecture/The rented machine.md`), its version swapped
    /// for the one observed failing OptiX on 5.2.2.
    const A_SUMMARY: &str = "\
+-----------------------------------------------------------------------------------------+\n\
| NVIDIA-SMI 550.163.01             Driver Version: 550.163.01     CUDA Version: 12.4     |\n\
|-----------------------------------------+------------------------+----------------------+\n";

    fn a_virtual_machine_whose_driver_answers() -> Answers {
        a_virtual_machine_with_no_cgroup_file().to(
            ASK_THE_DRIVER_FOR_THE_CARDS,
            0,
            "GPU-aaaa, 00000000:01:00.0, NVIDIA RTX A6000, 49140 MiB\n",
        )
    }

    async fn the_reading_of(mut machine: Answers) -> crate::hardware::HardwareReading {
        take_the_hardware_reading(&mut machine, "server", None, None, std::time::UNIX_EPOCH)
            .await
            .expect("the machine named its clock")
    }

    /// **Both values read**, one row per card, and the kind taken with them.
    #[tokio::test]
    async fn a_driver_that_answers_gives_its_version_and_its_cuda_version() {
        let machine = a_virtual_machine_whose_driver_answers()
            .to(ASK_THE_DRIVER_FOR_ITS_VERSION, 0, "550.163.01\n550.163.01\n")
            .to(ASK_THE_DRIVER_FOR_ITS_SUMMARY, 0, A_SUMMARY);
        let the_driver = the_reading_of(machine)
            .await
            .the_driver
            .expect("the driver answered with both");
        assert_eq!(the_driver.version.as_the_driver_wrote_it(), "550.163.01");
        assert_eq!(the_driver.cuda_version.as_deref(), Some("12.4"));
        assert_eq!(the_driver.where_k1_placed_the_machine, WhereK1PlacesIt::ItsOwnKernel);
    }

    /// **Both missing**: answers this cannot read are absences, never guesses.
    #[tokio::test]
    async fn answers_that_carry_no_version_leave_both_absent() {
        let mut machine = a_virtual_machine_whose_driver_answers()
            .to(ASK_THE_DRIVER_FOR_ITS_VERSION, 0, "[N/A]\n")
            .to(ASK_THE_DRIVER_FOR_ITS_SUMMARY, 0, "| NVIDIA-SMI 550.163.01 |\n");
        assert_eq!(ask_the_drivers_versions(&mut machine).await, (None, None));
        assert_eq!(the_reading_of(machine).await.the_driver, None);
    }

    /// **The card reading still holds** where both version queries fail
    /// outright — and the CUDA version alone failing keeps the version.
    #[tokio::test]
    async fn a_driver_that_will_not_give_its_versions_still_gives_its_cards() {
        let reading = the_reading_of(a_virtual_machine_whose_driver_answers()).await;
        let Cards::TheCards(cards) = &reading.cards else {
            panic!("the cards were read: {:?}", reading.cards);
        };
        assert_eq!(cards.len(), 1);
        assert_eq!(reading.the_driver, None);

        let machine = a_virtual_machine_whose_driver_answers()
            .to(ASK_THE_DRIVER_FOR_ITS_VERSION, 0, "575.57.08\n");
        let the_driver = the_reading_of(machine).await.the_driver.expect("the version stands");
        assert_eq!(the_driver.version.as_the_driver_wrote_it(), "575.57.08");
        assert_eq!(the_driver.cuda_version, None);
    }

    /// **Asked only where the driver answered** (H11b's *"where the driver
    /// answers"*): a machine with no driver is not asked for its version.
    #[tokio::test]
    async fn a_driver_that_did_not_answer_is_not_asked_its_version() {
        let machine = a_virtual_machine_with_no_cgroup_file()
            .to(ASK_THE_DRIVER_FOR_ITS_VERSION, 0, "550.163.01\n")
            .to(ASK_THE_DRIVER_FOR_ITS_SUMMARY, 0, A_SUMMARY);
        assert_eq!(the_reading_of(machine).await.the_driver, None);
    }
    // ---- H11b's OptiX-library bullet (2026-11-02) ----

    /// **The loader's answer is carried only where it says *not found***:
    /// `1` is *not found*, `0` is found, and anything else — or a lookup the
    /// machine did not answer — carries nothing. The rented RTX 4090 of
    /// 2026-11-01 ended setup ready with driver 580.178.04 and no
    /// `libnvoptix.so.1`, and the first three cards jobs failed at once.
    #[tokio::test]
    async fn a_driver_whose_loader_does_not_find_the_optix_library_is_carried_as_missing() {
        let answering = || {
            a_virtual_machine_whose_driver_answers()
                .to(ASK_THE_DRIVER_FOR_ITS_VERSION, 0, "580.178.04\n")
                .to(ASK_THE_DRIVER_FOR_ITS_SUMMARY, 0, A_SUMMARY)
        };
        let missing = the_reading_of(answering().to(LOOK_FOR_THE_OPTIX_LIBRARY, 1, "")).await;
        assert!(missing.the_optix_library_is_missing);
        assert!(missing.the_driver.is_some(), "the driver's part still stands");

        let found = the_reading_of(answering().to(LOOK_FOR_THE_OPTIX_LIBRARY, 0, "")).await;
        assert!(!found.the_optix_library_is_missing);
        let no_answer = the_reading_of(answering().to(LOOK_FOR_THE_OPTIX_LIBRARY, 2, "")).await;
        assert!(!no_answer.the_optix_library_is_missing);
        let not_answered = the_reading_of(answering()).await;
        assert!(!not_answered.the_optix_library_is_missing);
    }

    /// **Asked only where the driver answered**, like the two values: a
    /// machine whose driver did not answer is never told it lacks the library.
    #[tokio::test]
    async fn a_driver_that_did_not_answer_is_not_asked_for_the_optix_library() {
        let machine = a_virtual_machine_with_no_cgroup_file().to(LOOK_FOR_THE_OPTIX_LIBRARY, 1, "");
        assert!(!the_reading_of(machine).await.the_optix_library_is_missing);
    }

    /// **The lookup is `Rendering` L24b's own line, shared rather than
    /// copied**, so the reading and the render worker cannot ask two
    /// different questions of one machine.
    #[test]
    fn the_optix_lookup_is_the_render_workers_own_line() {
        assert_eq!(
            LOOK_FOR_THE_OPTIX_LIBRARY.as_str(),
            smoozy_server_setup_machine::the_optix_library::THE_LOADERS_OWN_LOOKUP
        );
        assert!(EVERY_COMMAND_THIS_MODULE_SENDS.contains(&LOOK_FOR_THE_OPTIX_LIBRARY));
    }
}
