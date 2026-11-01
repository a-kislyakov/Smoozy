//! The ready action — `Server Setup` §2.1, §2.2 and §2.5.
//!
//! **The constraint** is Row 88, with Facts 47, 80, 27 and 48 all answered. The
//! action promises the outcome — the machine made able to render — reached by a
//! single action; **what that action is differs by machine kind**, and
//! pretending otherwise would be wrong on the container kind.
//!
//! **Nothing here performs a step.** What is here is the composition — which
//! steps a machine gets, in what order — the claim the action may make at the
//! end, and the closed list of things it never does.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Rented-server runtime` G1b, G26, G31, K2, K17, P4, P5, V6.
//! `Servers` H11, H12, H12b. `Server protocol and transfers` §5, §7.1, S22,
//! S24.
//! `Job files and outputs` L2, L4a, 1.1.

use smoozy_servers::hardware::{BusScan, Cards, DriverAnswer, DriverVersion, OptixBarRow};

use crate::kind::{DistributionFamily, MachineKind};

/// **The steps, the driver check and the libraries step's judgements, at the
/// paths they have always been reached at.** They live in
/// `smoozy-server-setup-machine`, the half of this plan the rented machine's
/// own program may take, because the setup worker performs the steps and the
/// run record names them. **What stays here is what reads the machine's kind
/// and family**, which are desktop readings. Split at step 10c piece 3.
pub use smoozy_server_setup_machine::ready_action::{
    what_to_install_for, MappingEntry, Step, WhatTheLibrariesStepInstalls, THE_DRIVER_CHECK,
    THE_MAPPING, WHAT_A_BARE_CONTAINER_REPORTED,
};

/// What a run was found to need, at run start. **Every field is a reading or a
/// user's own instruction, and none of them is remembered from a previous
/// sitting** (`Rented-server runtime` K2, Row 104).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheMachineNeeds {
    pub kind: MachineKind,
    pub family: DistributionFamily,
    /// Whether this run installs a Blender — the user having asked for one.
    pub this_run_fetches_a_blender: bool,
    /// **Item 4's subject.** Whether the machine already holds a Blender the
    /// search found. Where this is false and the run fetches none, **the
    /// libraries step has no subject and is not composed into the run at all.**
    pub the_machine_already_holds_a_blender: bool,
    /// **Item 3.** Whether the unpack tool is present, as
    /// [`crate::readings::probe_the_unpack_tool`] asked the machine's own
    /// package system — *"the step probes rather than assumes"*.
    ///
    /// **[`the_tooling_step_installs_the_tool`] is what reads it**, and it is
    /// the whole of what decides whether an elevated package install is issued
    /// for the tool at all.
    pub the_unpack_tool_is_present: bool,
    /// **Item 5's check, run first.** Whether the driver's own query answers.
    pub the_driver_answers: bool,
    /// **Item 5's other branch** — *"where a driver is already installed, the
    /// restart is tried before any install"*.
    ///
    /// **Two readings make it, and since 2026-09-30 R1 item 5 names the
    /// second**: [`crate::readings::WhatTheDriverCheckSaid::a_driver_is_already_installed`],
    /// read off the same check the field above is read off, **and the running
    /// kernel's module** ([`crate::readings::read_the_running_kernels_module`])
    /// — *"A driver counts as installed for this rule only where the running
    /// kernel has its module."* The module is asked only where the check ended
    /// at 9, so no other machine pays a round trip for it. The whole reasoning,
    /// with the publications behind both, is written at those two.
    ///
    /// **[`what_the_driver_step_does`] is what reads it**, and it is the whole
    /// of what decides whether an install is issued or a restart is tried
    /// first.
    pub a_driver_is_already_installed: bool,
    /// **Item 1.** Whether a card is present on the bus, as
    /// [`read_card_presence`] concluded it.
    pub cards: Cards,
    /// Whether Telegram is on, which is what puts item 8 in the run.
    pub telegram_is_on: bool,
    /// **R1 item 5's version reading, added 2026-10-11** (the Creator's
    /// rulings on OptiX and the driver): the driver's own version, read with
    /// `Servers` H11b's query (`smoozy_servers::readings::ask_the_drivers_versions`).
    ///
    /// **Asked only where it can change something**: the driver answered, the
    /// machine is the systemd kind, and [`WhatTheMachineNeeds::the_optix_bar`]
    /// is set. **Absent everywhere else**, and absent where the driver gave no
    /// version this reading can read — and an absent version is never read as
    /// below the bar.
    pub the_drivers_version: Option<DriverVersion>,
    /// **R1 item 5's bar: `Rendering` L23a's row for the newest Blender on the
    /// server**, from `Rendering` L23a's one table
    /// ([`the_bar_for_the_newest_blender_on_the_server`]) — every build B10's
    /// search found, and the build this run stages where its version is known
    /// before the step. *Widened 2026-10-11 by R1b: this was the bar of the
    /// Blender this run installs alone.* **Absent where no Blender is on the
    /// server and none is staged**, and then nothing is compared. A user's own
    /// archive (B7) is asked its version only once it is on the machine, so it
    /// is compared after publication, by R1b.
    pub the_optix_bar: Option<OptixBarRow>,
    /// **R1b's live set, read where the swap would be made**: whether anything
    /// on the server is rendering, waiting or held (`Queue & History` §17), or
    /// a transfer is under way between this desktop and the machine. Added
    /// 2026-10-12. **Where it is true the swap is not made in this run, and it
    /// waits** ([`what_the_driver_step_does`]). It is read by the driving at
    /// the driver step itself, and false everywhere else, so a run that would
    /// make no swap pays no reading for it.
    pub anything_is_live: bool,
}

/// **R1 item 5's bar, for the Blender a run installs**, as the version text the
/// run was asked for (`5.2.2`) read into its three numbers and looked up in
/// `Rendering` L23a's one table. **Never a copy of the table.**
pub fn the_bar_for_the_blender_this_run_installs(version: Option<&str>) -> Option<OptixBarRow> {
    the_bar_for_the_newest_blender_on_the_server(version, &[])
}

/// **R1 item 5's bar as R1b widened it: the bar of the newest Blender on the
/// server.** Added 2026-10-12.
///
/// **Its inputs are every build B10's search found, and the build this run
/// stages** where its version is known before the step (`5.2.2`, from the
/// list, B4). `Rendering` L23a's table rises with the version, so the newest
/// build's row is the highest bar any of them has. **A run that installs no
/// Blender compares against the Blenders already there**; where there are none
/// and none is staged, nothing comes back and nothing is compared.
pub fn the_bar_for_the_newest_blender_on_the_server(
    staged: Option<&str>,
    found: &[crate::blender::Version],
) -> Option<OptixBarRow> {
    let staged = staged.and_then(crate::blender::Version::parse);
    let newest = found.iter().copied().chain(staged).max()?;
    smoozy_servers::hardware::the_optix_bar_row((newest.major, newest.minor, newest.patch))
}

/// **R1b's comparison: whether a swap is owed on this server.** Added
/// 2026-10-12.
///
/// *"A swap is owed where the driver answers below the bar on a machine item 5
/// would swap on."* **The machine is the systemd kind**; its family is read by
/// the swap's own run, whose readings come first, and a Debian machine there
/// makes no swap and ends with the bar missed, which is what tells the user
/// (§13 item 28). **Nothing is owed where the driver gave no version, or where
/// no Blender gives a bar.**
///
/// **A missed swap stands until the next ready action** — the runner's ruling
/// of 2026-10-11, written into R1b: *"until then no comparison above concludes
/// that a swap is owed there"*. So where a miss is kept for the server, this
/// concludes nothing is owed. The ready action does not ask this: its item 5
/// is the retry.
pub fn a_swap_is_owed(
    kind: MachineKind,
    the_drivers_version: Option<&DriverVersion>,
    the_bar: Option<OptixBarRow>,
    a_missed_swap_stands: bool,
) -> bool {
    if a_missed_swap_stands || !matches!(kind, MachineKind::ASystemdMachine) {
        return false;
    }
    match (the_drivers_version, the_bar) {
        (Some(version), Some(bar)) => version.is_below(bar.driver_minimum),
        _ => false,
    }
}

/// **Whether the driver's version, where one was read, is below the run's
/// bar, on a machine Smoozy manages.** Added 2026-10-11.
///
/// **False wherever any of it is missing** — no bar, no version, or a
/// container, whose driver is the host's and is told by `Servers` H11b's
/// statement on the card instead.
pub fn the_driver_is_below_the_bar(
    needs: &WhatTheMachineNeeds,
    version: Option<&DriverVersion>,
) -> bool {
    if !matches!(needs.kind, MachineKind::ASystemdMachine) {
        return false;
    }
    match (version, needs.the_optix_bar) {
        (Some(version), Some(bar)) => version.is_below(bar.driver_minimum),
        _ => false,
    }
}

/// **R1c: whether a scene-job is held for the driver at submission** — decided
/// on the desktop as its submission is composed, and nowhere else. Added
/// 2026-10-12, held-job piece A.
///
/// **All four stand, or nothing is held:**
///
/// - **its launch would name `OPTIX`** — a cards job on an NVIDIA machine
///   (`Rendering` L21, `Rendering` L23), read from the resolved device choice
///   the submission carries; a job whose device is not known at submission is
///   not held, and the caller passes false for it;
/// - **the driver is below `Rendering` L23a's bar for the job's own Blender**
///   — the version its pick answered, against this connection's driver
///   reading (`Servers` H11b), the comparison B12b's note on that row is drawn
///   from. A reading from an earlier connection is no reading here;
/// - **R1b's comparison concludes *a swap is owed* for this connected
///   period** — period-held and never stored, so where the desktop holds no
///   comparison, nothing is held;
/// - **`Servers` H11b's statement does not stand for this server** — a
///   container, or a machine whose swap missed the bar.
///
/// **So nothing is held on a container, on Debian, or on a server whose swap
/// missed the bar**, and such a job goes as `Rendering` §9 item 3 has it.
pub fn a_job_is_held_for_the_driver(
    its_launch_would_name_optix: bool,
    the_driver_on_this_connection: Option<&smoozy_servers::hardware::TheDriversOwnAnswer>,
    the_jobs_blender: Option<crate::blender::Version>,
    a_swap_is_owed_this_period: bool,
    a_missed_swap_stands: bool,
) -> bool {
    if !its_launch_would_name_optix || !a_swap_is_owed_this_period {
        return false;
    }
    let (Some(the_driver), Some(blender)) = (the_driver_on_this_connection, the_jobs_blender) else {
        return false;
    };
    if smoozy_servers::hardware::the_statement_is_said_on(
        the_driver.where_k1_placed_the_machine,
        a_missed_swap_stands,
    ) {
        return false;
    }
    the_driver_is_below_the_bar_of(&the_driver.version, blender)
}

/// **Whether a driver is below `Rendering` L23a's bar for one Blender** — the
/// row that Blender has reached, and nothing where it has none.
fn the_driver_is_below_the_bar_of(version: &DriverVersion, blender: crate::blender::Version) -> bool {
    match smoozy_servers::hardware::the_optix_bar_row((blender.major, blender.minor, blender.patch)) {
        Some(bar) => version.is_below(bar.driver_minimum),
        None => false,
    }
}

/// **What a hold of the driver's reason, found standing by a queue reading,
/// comes to** — R1c's *a standing hold of this reason is a moment for R1b's
/// comparison*. Added 2026-10-12, held-job piece A.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatAStandingHoldComesTo {
    /// **Nothing to read it against**: no driver reading of this connection.
    /// The hold stands.
    Nothing,
    /// **Below the job's bar, on a machine item 5 would swap on**: R1b's
    /// comparison concludes a swap is owed, and R1b's starting rule applies.
    ASwapIsOwed,
    /// **At or above the job's bar**: the swap's run ended with the driver
    /// there, on a connection the desktop did not see it end on. It is
    /// released, with no mark.
    ItIsReleased,
    /// **Below the bar, and `Servers` H11b's statement stands**: the swap's run
    /// missed it. It is released all the same, with `Queue & History` N23's
    /// mark written first.
    ItIsReleasedWithTheMark,
}

/// **R1c's third bullet, judged**: the driver of this connection against the
/// bar for the held job's own Blender, the version its record carries
/// (`Queue & History` N19). **No search is taken**, so B12's rule that the
/// search stays off the connection is untouched.
///
/// **The two releases are R1c's own, learned late.** R1c releases at the
/// ending of the run whose item 5 compared and acted, *"live, or, where it was
/// not connected, at the first reading of the last setup on the next
/// connection"*. The setup record does not keep whether item 5 acted, so the
/// later connection reads what that run left instead: a driver now at the bar,
/// or a miss kept on the row (`Servers` H11b). Either way the release comes,
/// as it would have at the ending. *This reading is the builder's, where the
/// clause left the choice; it is written into R1c's dated bullet.*
pub fn what_a_standing_hold_comes_to(
    the_driver_on_this_connection: Option<&smoozy_servers::hardware::TheDriversOwnAnswer>,
    a_missed_swap_stands: bool,
    the_jobs_blender: Option<crate::blender::Version>,
) -> WhatAStandingHoldComesTo {
    let Some(the_driver) = the_driver_on_this_connection else {
        return WhatAStandingHoldComesTo::Nothing;
    };
    let below = the_jobs_blender
        .is_some_and(|blender| the_driver_is_below_the_bar_of(&the_driver.version, blender));
    if !below {
        return WhatAStandingHoldComesTo::ItIsReleased;
    }
    if smoozy_servers::hardware::the_statement_is_said_on(
        the_driver.where_k1_placed_the_machine,
        a_missed_swap_stands,
    ) {
        return WhatAStandingHoldComesTo::ItIsReleasedWithTheMark;
    }
    let bar = the_jobs_blender.and_then(|blender| {
        smoozy_servers::hardware::the_optix_bar_row((blender.major, blender.minor, blender.patch))
    });
    let kind = match the_driver.where_k1_placed_the_machine {
        smoozy_servers::hardware::WhereK1PlacesIt::ItsOwnKernel => MachineKind::ASystemdMachine,
        smoozy_servers::hardware::WhereK1PlacesIt::AContainer => MachineKind::AContainer,
        smoozy_servers::hardware::WhereK1PlacesIt::NowhereItPlaces => MachineKind::TheTableDoesNotPlaceIt,
    };
    if a_swap_is_owed(kind, Some(&the_driver.version), bar, a_missed_swap_stands) {
        WhatAStandingHoldComesTo::ASwapIsOwed
    } else {
        WhatAStandingHoldComesTo::Nothing
    }
}

/// **R1, performed: the steps this machine gets, in R1's order.**
///
/// **R3 is applied here rather than beside it**: on a container the driver is
/// the host's — never installed, never repaired, never reloaded — and no
/// restart is issued, so neither step is composed at all.
pub fn compose_the_run(needs: &WhatTheMachineNeeds) -> Vec<Step> {
    let mut steps = Vec::new();
    // **Item 1 is always present**: the readings are what the rest is composed
    // from.
    steps.push(Step::Readings);

    if needs.this_run_fetches_a_blender {
        steps.push(Step::Fetch);
        // **Item 3 is the unpack**, so it is present whenever there is an
        // archive to unpack — installing the tool is the part that is
        // conditional, and that is `the_unpack_tool_is_present`'s business
        // rather than this step's presence.
        steps.push(Step::Tooling);
    }

    // **Item 4: the step has a subject only where there is a build on the
    // machine to read a library chain against.** Where this run fetches none
    // and the search found none, the action **declines the library half and
    // says so** ([`crate::statements`] W1) rather than composing a fetch nobody
    // asked for.
    if needs.this_run_fetches_a_blender || needs.the_machine_already_holds_a_blender {
        steps.push(Step::Libraries);
    }

    // **Item 5, systemd kind only** (R3, Fact 48).
    if the_driver_step_is_composed(needs) {
        steps.push(Step::Driver);
        // **Item 6 is composed with it**, and runs only where item 5's check
        // failed after its work. On Debian the restart is not optional
        // (M1b), so the step is present on that family whenever the driver
        // step is.
        steps.push(Step::Restart);
    }

    if needs.this_run_fetches_a_blender {
        steps.push(Step::BlenderPublication);
    }
    if needs.telegram_is_on {
        steps.push(Step::TelegramMaterial);
    }
    steps.push(Step::Done);
    steps
}

/// **R1b: outside a ready action, the swap is a run of its own** — the
/// readings, item 5's swap, item 6's restart and *done*, and nothing else.
/// Added 2026-10-12.
///
/// **It is the ready action's own kind of run** (Y6b), composed of item 5 and
/// item 6 alone, so a machine program of any release takes it, and the card
/// reads it as the last setup. **Its driver step is check-first like every
/// other**: where the readings find the driver already at the bar, or a
/// container, nothing is done. **An install run stays additive** (Y1): the swap
/// follows it as this second run rather than joining it.
pub fn compose_the_swap_run(needs: &WhatTheMachineNeeds) -> Vec<Step> {
    let mut steps = vec![Step::Readings];
    if the_driver_step_is_composed(needs) {
        steps.push(Step::Driver);
        steps.push(Step::Restart);
    }
    steps.push(Step::Done);
    steps
}

/// **A Blender install from the Blender section: `Server Setup` Y6b's own kind
/// of run, and R1's Blender half with nothing of the machine's preparation in
/// it.** Added 2026-09-21.
///
/// **What it contains is R1's own steps, in R1's order, each present only where
/// its step is** — the readings, the fetch (B4), the tooling step (item 3: the
/// unpack, with the tool installed only where absent), the libraries step
/// (item 4, whose subject is the build this run fetches), the publication (item
/// 7) and *done* (item 9). **No driver step, no restart and no Telegram
/// material**: Y1 names a Blender install as an additive run where the tool is
/// present, and Y6b gives the machine's preparation and the Telegram placement
/// runs of their own. **So nothing here can restart the machine**, which is
/// `Design/Server Setup § A long step`'s own reason a Blender install warns only
/// where the unpack tool must be installed.
pub fn compose_a_blender_install() -> Vec<Step> {
    vec![
        Step::Readings,
        Step::Fetch,
        Step::Tooling,
        Step::Libraries,
        Step::BlenderPublication,
        Step::Done,
    ]
}

/// **The Telegram placement: [`crate::telegram`] G2's dedicated entry point as
/// Y6b's own kind of run.** Added 2026-10-04, the 38's piece 11.
///
/// **It places the material, sets the desired state to enabled, and does
/// nothing else** (G2): the readings, which every run is composed from and
/// whose *started* boundary the machine writes with the record; R1 item 8's
/// step; and *done* (item 9). **No fetch, no tooling, no libraries, no driver
/// and no restart**, so nothing here can restart the machine — which is the
/// whole reason the spec gives Telegram an action of its own.
pub fn compose_the_telegram_placement() -> Vec<Step> {
    vec![Step::Readings, Step::TelegramMaterial, Step::Done]
}

/// **A Blender removal: `Server Setup` B15's plain deletion as Y6b's own kind
/// of run.** Added 2026-10-10, the 38's piece 14c.
///
/// **The readings, which every run is composed from; the removal; and
/// *done*.** No fetch, no package step, no driver and no restart: the folder
/// deleted is Smoozy's own, under the Blender area, so nothing here is
/// elevated and nothing can restart the machine.
pub fn compose_a_blender_removal() -> Vec<Step> {
    vec![Step::Readings, Step::BlenderRemoval, Step::Done]
}

/// **R1 item 3's *where absent*, performed — and it is the one thing that
/// decides whether the tooling step reaches the package system.**
///
/// **The step is composed whenever there is an archive to unpack** and the
/// unpack is always performed; *"installing the tool is the part that is
/// conditional"*, and this is that condition.
///
/// **Y1's class for the run turns on this answer**, because the install is Y1's
/// own first example: *"a Blender install on a machine lacking the unpack tool
/// performs an elevated package install and is machine-changing, and one on a
/// machine that already has the tool performs an unpack and a rename and is
/// not."* **So a run that does not install here is additive on this account**,
/// and Y4a's mark for the step is *nothing to do* rather than *performed*.
pub fn the_tooling_step_installs_the_tool(needs: &WhatTheMachineNeeds) -> bool {
    !needs.the_unpack_tool_is_present
}

/// **R1 item 5 and R3: the driver step exists on the systemd kind and nowhere
/// else, and it is check-first.**
///
/// *"Where it answers, nothing is installed and nothing is touched."*
pub fn the_driver_step_is_composed(needs: &WhatTheMachineNeeds) -> bool {
    if !matches!(needs.kind, MachineKind::ASystemdMachine) {
        return false;
    }
    // **A card that is not established is not a card**, so nothing is installed
    // for one. The step is composed where a card is present on the bus and the
    // driver does not answer.
    if needs.the_driver_answers {
        // The check is still run — that is what makes it check-first — but the
        // step performs no act. It is composed so the boundary and its
        // *nothing to do* mark exist ([`crate::run`] Y4a).
        return true;
    }
    matches!(needs.cards, Cards::PresentAndUnreadable(_) | Cards::TheCards(_))
}

/// **R1 item 5's order, written out because getting it wrong costs a paid
/// machine an install it did not need.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheDriverStepDoes {
    /// **The check answered: nothing is installed and nothing is touched.**
    NothingTheCheckAnswered,
    /// **A driver is already installed and does not answer: the restart is
    /// tried before any install.** Both observed failure mechanisms — a card
    /// still bound to `nouveau`, and a module built for a kernel not yet booted
    /// — are fixed by the boot, and **a reinstall on a working-but-unbooted
    /// install silently becomes an upgrade the running kernel does not match**.
    TheRestartBeforeAnyInstall,
    /// **No driver is installed: the distribution's own detection route**, per
    /// family ([`crate::kind`] M1b), then the check again.
    TheDistributionsOwnDetectionRoute,
    /// **Nothing at all**: no card is present, or the kind closes the route.
    Nothing,
    /// **The check answered with a driver below the OptiX bar of the newest
    /// Blender on the server, on Ubuntu, with nothing live there (R1b): the
    /// old branch gives way to a newer one
    /// from Ubuntu's own archive** ([`crate::elevated::THE_UBUNTU_DRIVER_SWAP`]),
    /// then the check again. Added 2026-10-11, R1 item 5 as the Creator's
    /// rulings on OptiX and the driver extended it. **Debian gets no swap**:
    /// the family's route reached 535 there (§13 item 28), and a driver below
    /// the bar on that family is recorded as having missed it.
    TheSwapUpToTheBar,
}

/// **R1 item 5, performed — and it is the one place the clause's order is
/// decided.**
///
/// **It is reached from the running application** (`App/Desktop app and
/// delivery/src/setting_up.rs`, the driver step), which is what it was not until
/// 2026-09-20: it was called from four places and all four were this file's own
/// tests, while the driving kept the same clause its own way with no
/// already-installed branch at all. **Two implementations of one clause is the
/// fault this stage keeps paying for**, and this is now the only one.
///
/// **Both readings it takes are the run's own** — the check's answer and, where
/// it failed, its exit — so no question is asked of the machine here.
pub fn what_the_driver_step_does(needs: &WhatTheMachineNeeds) -> WhatTheDriverStepDoes {
    if !the_driver_step_is_composed(needs) {
        return WhatTheDriverStepDoes::Nothing;
    }
    if needs.the_driver_answers {
        // **R1 item 5, added 2026-10-11: a driver that answers below the bar
        // is brought up to it, where Smoozy has a route to do so.** Nothing
        // asks the user and nothing warns them. **R1b, 2026-10-12: where
        // anything is live the swap is not made in this run, and it waits.**
        if matches!(needs.family, DistributionFamily::Ubuntu)
            && !needs.anything_is_live
            && the_driver_is_below_the_bar(needs, needs.the_drivers_version.as_ref())
        {
            return WhatTheDriverStepDoes::TheSwapUpToTheBar;
        }
        return WhatTheDriverStepDoes::NothingTheCheckAnswered;
    }
    if needs.a_driver_is_already_installed {
        WhatTheDriverStepDoes::TheRestartBeforeAnyInstall
    } else {
        WhatTheDriverStepDoes::TheDistributionsOwnDetectionRoute
    }
}

/// **R1 item 1: card presence, read from the bus scan and the driver's own
/// answer and never from the card tool's absence** (Fact 46).
///
/// **The scan returning empty and the scan not running are two different
/// readings and are never collapsed**, because only the first concludes *no
/// card*: `lspci` was **absent from every container image** in the
/// multi-provider batch, and `pciutils` is not in those images — installing it
/// is a package operation this reading is taken before.
///
/// **`Servers` H11's table is what performs the join and it is not re-derived
/// here.** On a container the bus scan is the host's unfiltered view and lies,
/// which is that document's H12; a machine that is not a container and lacks
/// `lspci` too is its H12b.
pub fn read_card_presence(scan: BusScan, driver: DriverAnswer, kind: MachineKind) -> Cards {
    let servers_kind = match kind {
        MachineKind::AContainer => smoozy_servers::hardware::MachineKind::AContainer,
        // **A machine the table does not place takes no setup at all**, and no
        // reading of it is composed into one; it is passed through here as the
        // not-a-container row so this function is total.
        MachineKind::ASystemdMachine | MachineKind::TheTableDoesNotPlaceIt => {
            smoozy_servers::hardware::MachineKind::NotAContainer
        }
    };
    smoozy_servers::hardware::read_the_cards(scan, driver, servers_kind)
}

/// **R1 item 3: the one unpack tool Blender's documentation names.**
///
/// **Observed, 2026-08-22**: Blender ships as `.tar.xz` — the two archives
/// fetched were 377 902 364 and 384 441 228 bytes — and **`xz` was absent from
/// the image**, so the archive could not be unpacked until this package was
/// installed. **`tar` cannot be absent from an ordinary Ubuntu or Debian**,
/// because it is **Essential**, which Debian Policy §3.8 defines as *"the
/// minimal set of functionality that must be available and usable on the system
/// at all times"*; `xz-utils` carries no such tag and `tar`'s own page lists it
/// under *suggests* rather than *depends*.
pub const THE_UNPACK_TOOLS_PACKAGE: &str = "xz-utils";

/// **B7b's evidence for the same step, and it is what makes the step's
/// alternative loud rather than partial.**
///
/// On a deliberately bare rented Linux machine holding GNU tar 1.34 and no
/// `xz`, **all four versions' archives refused at exit code 2 with nothing
/// landing on disk** — tar could not run `xz` — and **naming the compression
/// outright made no difference**. One further package fixed it, taking the
/// machine from 253 installed packages to 254. Label: **observed in a test**,
/// one bare Linux machine, all four versions.
///
/// **What it settles**: a run that reaches the unpack on a machine without the
/// tool fails at that step in the machine's own words and **leaves nothing
/// behind**, so on this route there is no half-unpacked state for
/// [`crate::recovery`]'s re-run rule to meet.
pub const THE_ABSENT_TOOL_FAILURE_IS_LOUD_AND_TOTAL: bool = true;

/// **R1 item 4's cost, timed on the same box, and it is the opposite of what a
/// transfer-shaped estimate predicts.**
///
/// Download **5 s** for 4.5.12 and **4 s** for 5.2.0; unpack **62 s** and
/// **67 s** on four cores, single-threaded `xz`. **The unpack dominates the
/// download by an order of magnitude.** On a machine with more cores both
/// archives downloaded in parallel and both unpacked in **34 seconds in all**.
/// **So the cost of this step is the machine's processor rather than its
/// link**, and anything that shows the user a duration for it **must not budget
/// it as a transfer.**
pub const THE_UNPACK_IS_NOT_BUDGETED_AS_A_TRANSFER: bool = true;

/// **R1a: a machine asked cold what cards it has can answer that it has none,
/// and that answer is never read as a cardless machine.**
///
/// Cycles' own list of cards comes back **empty** in all four versions on a
/// factory-fresh Blender — with an untouched settings folder, and again started
/// at factory settings — and fills only once a refresh has been asked for.
/// **So a four-card machine and a cardless one give the same answer.** Label:
/// **observed in a test**.
///
/// **Nothing this document reads is that list**, which is why no step,
/// statement or ending moves: [`read_card_presence`] takes the bus scan and the
/// driver's own query, the driver check is `nvidia-smi`'s exit, and
/// [`crate::statements`] W4 reads a finished job's recorded device line.
pub const NOTHING_HERE_READS_BLENDERS_OWN_DEVICE_LIST: bool = true;

/// **R1a's rule, stated because the empty answer carries no mark
/// distinguishing it from a real one.**
///
/// Where anything reads Blender's own device list, **the refresh is asked for
/// first**, and **an unrefreshed empty list is never read as *this machine has
/// no cards*** — the only thing it establishes is that nothing filled it. The
/// conflict it sits against is `Server Setup` §13 item 25 and **it is not
/// resolved**.
pub fn an_unrefreshed_device_list_establishes_no_cards(_it_was_empty: bool) -> bool {
    false
}

/// **R2: on a bare machine the package placement precedes the ready action**,
/// because the setup-in-progress state is set through the ordinary command path
/// and there is no supervisor to take the command before the package exists.
///
/// **The order on a machine never prepared**, and each entry says who owns it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OnAMachineNeverPrepared {
    /// **[`crate::kind`] M1's kind and M1a's family read first, and nothing
    /// else if either refuses.** Added 2026-08-22 — the readings used to sit at
    /// the placement, which put them after the hand-over and made M1a's
    /// *nothing is touched* false.
    TheKindAndTheFamily,
    /// **The candidate set and the hand-over** ([`crate::placement`] R6, R7).
    TheCandidateSetAndTheHandOver,
    /// **The bootstrap-and-repair request placing the package** (`Server
    /// protocol and transfers` §5; `Rented-server runtime` §2). It acquires
    /// `Rented-server runtime` **G1b**'s one package-transaction claim before
    /// inspection or phase change — the desktop's own exclusive create, because
    /// on a machine never prepared **there is no Smoozy program on the machine
    /// to hold a kernel lock** — and accepts only G26 to G31's verified
    /// manifest entry and exact platform tuple.
    ThePlacement,
    /// **The setup run itself.**
    TheSetupRun,
}

impl OnAMachineNeverPrepared {
    /// **R2's order, entire.**
    pub const IN_ORDER: [OnAMachineNeverPrepared; 4] = [
        OnAMachineNeverPrepared::TheKindAndTheFamily,
        OnAMachineNeverPrepared::TheCandidateSetAndTheHandOver,
        OnAMachineNeverPrepared::ThePlacement,
        OnAMachineNeverPrepared::TheSetupRun,
    ];
}

/// **R2: a setup action that meets automatic repair or another application
/// instance already holding the package owner reports the in-progress owner and
/// phase and starts no second placement.**
pub fn a_second_placement_is_started(the_owner_is_held_by_somebody_else: bool) -> bool {
    !the_owner_is_held_by_somebody_else
}

/// **R3: on a container the action's shape is verification plus at most the
/// user-space half.**
///
/// The driver is the host's: never installed, never repaired, never reloaded,
/// and no restart is issued (Fact 48). **Where the login can install packages —
/// the container's root — steps 3 and 4 run; where it cannot, nothing elevated
/// runs at all** and the action is [`crate::statements`] W1's statements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatAContainerGets {
    VerificationAndTheUserSpaceHalf,
    VerificationAlone,
}

/// **R3, performed.**
pub fn what_a_container_gets(the_login_is_the_containers_root: bool) -> WhatAContainerGets {
    if the_login_is_the_containers_root {
        WhatAContainerGets::VerificationAndTheUserSpaceHalf
    } else {
        WhatAContainerGets::VerificationAlone
    }
}

/// **R3's reading rule, and R8's whole defence rests on it.**
///
/// **The action never reads a CUDA library's own version as evidence the
/// pairing works** — under a deliberate mismatch `cuDriverGetVersion` succeeded
/// and reported CUDA 9.1 while Blender rendered on the processor at exit 0
/// (Fact 48). **The driver's own answer is the only reading taken.**
pub fn a_user_space_library_version_is_evidence_the_pairing_works(_it_answered: bool) -> bool {
    false
}

/// **R8: on a container the user-space driver libraries can be shadowed and a
/// broken pairing renders quietly at exit 0** — Fact 48's one dangerous gap.
///
/// **Nothing here inspects `LD_LIBRARY_PATH` or the symlinks**: `ldconfig`
/// silently reverts a swap and almost every package operation fires it, so a
/// reading of that state **would not stay true long enough to act on**. The
/// whole defence is R3's reading rule above plus [`crate::statements`] W4's
/// after-the-job evidence.
pub const NOTHING_INSPECTS_THE_LOADER_STATE: bool = true;

/// **R4: what *ready* claims, exactly, and it is cut to what was read.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhatReadyClaims {
    /// **Only where the machine held a Blender or this run fetched one**: that
    /// build's library chain resolves. **Where there was no Blender to read one
    /// against, the library half was not read and *ready* claims nothing about
    /// it** — the step had no subject, and the action says so rather than
    /// composing the claim out of a check that did not run.
    pub the_library_chain_resolves: Option<bool>,
    /// **Only where a card is present**: the driver answers.
    pub the_driver_answers: Option<bool>,
    /// **It does not claim the machine renders on its cards.** With the driver
    /// missing or wrong every version rendered on the processor and reported
    /// success, so a finished job is no proof and neither is a ready action.
    pub it_renders_on_its_cards: (),
}

/// **R4, performed from what the run actually read.**
pub fn what_ready_claims(needs: &WhatTheMachineNeeds, ldd_came_back_clean: bool) -> WhatReadyClaims {
    let there_was_a_build_to_read_against =
        needs.this_run_fetches_a_blender || needs.the_machine_already_holds_a_blender;
    WhatReadyClaims {
        the_library_chain_resolves: there_was_a_build_to_read_against.then_some(ldd_came_back_clean),
        the_driver_answers: match needs.cards {
            Cards::TheCards(_) | Cards::PresentAndUnreadable(_) => Some(needs.the_driver_answers),
            Cards::NoCard | Cards::CouldNotBeEstablished => None,
        },
        it_renders_on_its_cards: (),
    }
}

/// **R5: what the action never does.** A closed list, each entry with the
/// observation behind it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItNever {
    /// **Works from Blender's launch error** — one library per round; Fact 47.
    WorksFromBlendersLaunchError,
    /// **Relies on the driver's dependencies** — the metapackage trap, where a
    /// driver metapackage incidentally supplied six of nine missing libraries.
    ReliesOnTheDriversDependencies,
    /// **Restarts unconditionally** — Fact 80. Ubuntu's reboot-required flag
    /// was never set on the machine that needed a restart most.
    RestartsUnconditionally,
    /// **Reloads modules by hand** — the observed result oscillates between
    /// working and inconsistent, and **no reading taken while a machine is in
    /// that state is acted on here, card detection included**.
    ReloadsModulesByHand,
    /// **Trusts the reboot-required flag** — never set on the machine that
    /// needed it most.
    TrustsTheRebootRequiredFlag,
    /// **Repeats a driver install whose check passes** — the
    /// reinstall-becomes-upgrade observation is the exact shape of *running it
    /// again makes it worse*, and [`crate::recovery`] S4 is written so the
    /// re-run can never reach it.
    RepeatsADriverInstallWhoseCheckPasses,
}

impl ItNever {
    pub const ALL: [ItNever; 6] = [
        ItNever::WorksFromBlendersLaunchError,
        ItNever::ReliesOnTheDriversDependencies,
        ItNever::RestartsUnconditionally,
        ItNever::ReloadsModulesByHand,
        ItNever::TrustsTheRebootRequiredFlag,
        ItNever::RepeatsADriverInstallWhoseCheckPasses,
    ];
}

/// **R5, as the question a caller asks.** Every one of the six is refused, and
/// the function exists so that a builder reaching for one meets a value rather
/// than an absence.
pub fn the_action_may(_it: ItNever) -> bool {
    false
}

/// **Fact 80's restart cost, carried so nothing budgets a fixed wait.**
///
/// Restart cost varied **9 to 90 seconds** across eight measurements on three
/// machines, so **nothing here budgets a fixed wait** (`Rented-server runtime`
/// K17).
pub const NOTHING_BUDGETS_A_FIXED_RESTART_WAIT: bool = true;

/// **R9: Secure Boot, and the machine Smoozy cannot make ready.**
///
/// Fact 81 is unposable — **none of the seven machines this project rented had
/// Secure Boot enforcing** — and its failure mode is named as a clause of Row
/// 88's answer rather than as a reason to leave the row open. On such a rental
/// the driver step's module signing could fail outright or demand a human
/// enrolling a key at the machine's console, **which Smoozy cannot do over the
/// connection it holds**. Label: **unresolved**; `Server Setup` §13 item 1
/// carries what breaks.
///
/// **The ending is [`crate::statements`] W1's *could not be made ready*, with
/// the machine's own words behind it** — never a claim about Secure Boot, which
/// nothing here reads.
pub fn secure_boot_is_read_to_decide_the_ending(_anything: ()) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn a_bare_systemd_machine() -> WhatTheMachineNeeds {
        WhatTheMachineNeeds {
            kind: MachineKind::ASystemdMachine,
            family: DistributionFamily::Ubuntu,
            this_run_fetches_a_blender: true,
            the_machine_already_holds_a_blender: false,
            the_unpack_tool_is_present: false,
            the_driver_answers: false,
            a_driver_is_already_installed: true,
            cards: Cards::PresentAndUnreadable(
                smoozy_servers::hardware::WhyUnreadable::TheDriverIsAbsent,
            ),
            telegram_is_on: false,
            the_drivers_version: None,
            the_optix_bar: None,
            anything_is_live: false,
        }
    }

    #[test]
    fn the_steps_are_composed_in_r1s_order_and_never_in_another() {
        let steps = compose_the_run(&a_bare_systemd_machine());
        let mut sorted = steps.clone();
        sorted.sort();
        assert_eq!(steps, sorted);
        assert_eq!(steps.first(), Some(&Step::Readings));
        assert_eq!(steps.last(), Some(&Step::Done));
    }

    #[test]
    fn a_run_with_no_blender_anywhere_has_no_libraries_step_at_all() {
        // R1 item 4 and Y4a's third bullet: the step is not composed into the
        // run, so it has no boundary and gains no per-step outcome. Writing one
        // would record a check that never ran.
        let mut needs = a_bare_systemd_machine();
        needs.this_run_fetches_a_blender = false;
        needs.the_machine_already_holds_a_blender = false;
        let steps = compose_the_run(&needs);
        assert!(!steps.contains(&Step::Libraries));
        assert!(!steps.contains(&Step::Fetch));
        assert!(!steps.contains(&Step::BlenderPublication));

        // And *ready* then claims nothing about the library half.
        let claims = what_ready_claims(&needs, true);
        assert_eq!(claims.the_library_chain_resolves, None);
    }

    #[test]
    fn the_library_half_is_re_reached_the_moment_a_blender_arrives() {
        let mut needs = a_bare_systemd_machine();
        needs.this_run_fetches_a_blender = false;
        needs.the_machine_already_holds_a_blender = true;
        assert!(compose_the_run(&needs).contains(&Step::Libraries));
    }

    #[test]
    fn a_blender_install_is_r1s_blender_half_and_nothing_of_the_machines_preparation() {
        let steps = compose_a_blender_install();
        for never in [Step::Driver, Step::Restart, Step::TelegramMaterial] {
            assert!(!steps.contains(&never), "{never:?} is in a Blender install");
        }
        // **R1's order is kept**: every step of it the install holds sits where
        // the ready action's own composition puts it.
        let mut needs = a_bare_systemd_machine();
        needs.this_run_fetches_a_blender = true;
        needs.telegram_is_on = false;
        let ready: Vec<Step> = compose_the_run(&needs)
            .into_iter()
            .filter(|step| steps.contains(step))
            .collect();
        assert_eq!(ready, steps);
    }

    /// **G2: the Telegram placement places the material and does nothing
    /// else** — no step that could restart the machine or touch anything but
    /// Telegram's piece, in R1's own order. The 38's piece 11.
    #[test]
    fn the_telegram_placement_is_item_8_and_nothing_else() {
        let steps = compose_the_telegram_placement();
        assert_eq!(steps, vec![Step::Readings, Step::TelegramMaterial, Step::Done]);
        let mut needs = a_bare_systemd_machine();
        needs.telegram_is_on = true;
        let ready: Vec<Step> = compose_the_run(&needs)
            .into_iter()
            .filter(|step| steps.contains(step))
            .collect();
        assert_eq!(ready, steps, "R1's order is kept");
    }

    /// **B15: a removal is the readings, the removal and *done*, and never
    /// part of the ready action.** The 38's piece 14c.
    #[test]
    fn a_blender_removal_is_its_own_run_and_never_part_of_the_ready_action() {
        assert_eq!(
            compose_a_blender_removal(),
            vec![Step::Readings, Step::BlenderRemoval, Step::Done]
        );
        let mut needs = a_bare_systemd_machine();
        needs.telegram_is_on = true;
        needs.this_run_fetches_a_blender = true;
        assert!(!compose_the_run(&needs).contains(&Step::BlenderRemoval));
        assert!(!compose_a_blender_install().contains(&Step::BlenderRemoval));
    }

    #[test]
    fn a_container_never_gets_a_driver_step_or_a_restart() {
        // R3 and Fact 48: all four routes closed, and a restart there would
        // restart the container and repair nothing.
        let mut needs = a_bare_systemd_machine();
        needs.kind = MachineKind::AContainer;
        let steps = compose_the_run(&needs);
        assert!(!steps.contains(&Step::Driver));
        assert!(!steps.contains(&Step::Restart));
        assert_eq!(
            what_the_driver_step_does(&needs),
            WhatTheDriverStepDoes::Nothing
        );
    }

    #[test]
    fn a_driver_that_answers_is_never_touched() {
        // R1 item 5's check-first order, and R5's sixth negative.
        let mut needs = a_bare_systemd_machine();
        needs.the_driver_answers = true;
        assert_eq!(
            what_the_driver_step_does(&needs),
            WhatTheDriverStepDoes::NothingTheCheckAnswered
        );
        assert!(!the_action_may(ItNever::RepeatsADriverInstallWhoseCheckPasses));
    }

    /// **R1 item 5 as extended 2026-10-11: a driver that answers below the
    /// OptiX bar of the Blender this run installs is swapped, on Ubuntu, and
    /// on nothing else.**
    #[test]
    fn a_driver_below_the_bar_of_the_blender_this_run_installs_is_swapped_on_ubuntu_alone() {
        let v = |text: &str| DriverVersion::parse(text);
        let mut needs = a_bare_systemd_machine();
        needs.the_driver_answers = true;
        needs.the_drivers_version = v("550.163.01");
        needs.the_optix_bar = the_bar_for_the_blender_this_run_installs(Some("5.2.2"));
        assert_eq!(
            what_the_driver_step_does(&needs),
            WhatTheDriverStepDoes::TheSwapUpToTheBar
        );

        // **Already at the bar: nothing is touched.**
        let mut at = needs.clone();
        at.the_drivers_version = v("595.91.07");
        assert_eq!(
            what_the_driver_step_does(&at),
            WhatTheDriverStepDoes::NothingTheCheckAnswered
        );
        // **A run installing only 5.1 reads 535's row**, which 550 meets.
        let mut older = needs.clone();
        older.the_optix_bar = the_bar_for_the_blender_this_run_installs(Some("5.1.2"));
        assert_eq!(
            what_the_driver_step_does(&older),
            WhatTheDriverStepDoes::NothingTheCheckAnswered
        );
        // **A container is never touched** (R3), whatever its driver reads.
        let mut container = needs.clone();
        container.kind = MachineKind::AContainer;
        assert_eq!(
            what_the_driver_step_does(&container),
            WhatTheDriverStepDoes::Nothing
        );
        assert!(!the_driver_is_below_the_bar(&container, container.the_drivers_version.as_ref()));
        // **Debian gets no swap** (§13 item 28); the driver is below the bar
        // and the run records it as missed.
        let mut debian = needs.clone();
        debian.family = DistributionFamily::Debian;
        assert_eq!(
            what_the_driver_step_does(&debian),
            WhatTheDriverStepDoes::NothingTheCheckAnswered
        );
        assert!(the_driver_is_below_the_bar(&debian, debian.the_drivers_version.as_ref()));
        // **No version read, or no Blender installed, compares nothing.**
        let mut unread = needs.clone();
        unread.the_drivers_version = None;
        assert_eq!(
            what_the_driver_step_does(&unread),
            WhatTheDriverStepDoes::NothingTheCheckAnswered
        );
        let mut no_blender = needs.clone();
        no_blender.the_optix_bar = the_bar_for_the_blender_this_run_installs(None);
        assert_eq!(
            what_the_driver_step_does(&no_blender),
            WhatTheDriverStepDoes::NothingTheCheckAnswered
        );
    }

    /// **R1b: the bar is the newest Blender on the server's, whether the run
    /// installs one or not.** Added 2026-10-12.
    #[test]
    fn the_bar_is_taken_over_every_blender_found_and_the_one_staged() {
        let v = |text: &str| crate::blender::Version::parse(text).expect("a version");
        let bar = |staged: Option<&str>, found: &[crate::blender::Version]| {
            the_bar_for_the_newest_blender_on_the_server(staged, found).map(|row| row.driver_minimum)
        };
        // **No install, a 5.2.2 already there beside a 4.5**: 5.2.2's row.
        assert_eq!(bar(None, &[v("4.5.3"), v("5.2.2")]), Some(&[575][..]));
        // **The order the search found them in plays no part.**
        assert_eq!(bar(None, &[v("5.2.2"), v("4.5.3")]), Some(&[575][..]));
        // **An older Blender installed beside a newer one keeps the newer bar.**
        assert_eq!(bar(Some("4.5.3"), &[v("5.2.2")]), Some(&[575][..]));
        // **A newer one staged raises it.**
        assert_eq!(bar(Some("5.2.2"), &[v("4.5.3")]), Some(&[575][..]));
        // **Nothing found and nothing staged compares nothing.**
        assert_eq!(bar(None, &[]), None);

        // **And the driver step reads it with no install at all**: a ready
        // action on a 550 machine holding 5.2.2 swaps.
        let mut needs = a_bare_systemd_machine();
        needs.this_run_fetches_a_blender = false;
        needs.the_driver_answers = true;
        needs.the_drivers_version = DriverVersion::parse("550.163.01");
        needs.the_optix_bar = the_bar_for_the_newest_blender_on_the_server(None, &[v("5.2.2")]);
        assert_eq!(
            what_the_driver_step_does(&needs),
            WhatTheDriverStepDoes::TheSwapUpToTheBar
        );
    }

    /// **R1b: where anything is live the swap is not made in this run.** The
    /// check still answered, so nothing else is done. Added 2026-10-12.
    #[test]
    fn the_swap_is_left_out_while_anything_is_live() {
        let mut needs = a_bare_systemd_machine();
        needs.the_driver_answers = true;
        needs.the_drivers_version = DriverVersion::parse("550.163.01");
        needs.the_optix_bar = the_bar_for_the_blender_this_run_installs(Some("5.2.2"));
        needs.anything_is_live = true;
        assert_eq!(
            what_the_driver_step_does(&needs),
            WhatTheDriverStepDoes::NothingTheCheckAnswered
        );
        needs.anything_is_live = false;
        assert_eq!(
            what_the_driver_step_does(&needs),
            WhatTheDriverStepDoes::TheSwapUpToTheBar
        );
    }

    /// **R1b's own run: the readings, the driver, the restart and *done***,
    /// in R1's order, and on a container the readings and *done* alone.
    /// Added 2026-10-12.
    #[test]
    fn the_swap_run_is_items_5_and_6_and_nothing_else() {
        let mut needs = a_bare_systemd_machine();
        needs.the_driver_answers = true;
        needs.telegram_is_on = true;
        assert_eq!(
            compose_the_swap_run(&needs),
            vec![Step::Readings, Step::Driver, Step::Restart, Step::Done]
        );
        let ready: Vec<Step> = compose_the_run(&needs)
            .into_iter()
            .filter(|step| compose_the_swap_run(&needs).contains(step))
            .collect();
        assert_eq!(ready, compose_the_swap_run(&needs), "R1's order is kept");
        needs.kind = MachineKind::AContainer;
        assert_eq!(compose_the_swap_run(&needs), vec![Step::Readings, Step::Done]);
    }

    /// **R1b's comparison**: owed on a systemd machine whose driver answers
    /// below the newest Blender's bar, and nowhere else — and never while a
    /// missed swap stands (the runner's ruling of 2026-10-11). Added
    /// 2026-10-12.
    /// **R1c at submission: all four stand, or nothing is held** — each of
    /// the four failing once sends the job plain. Held-job piece A.
    #[test]
    fn a_job_is_held_for_the_driver_only_where_all_four_stand() {
        use smoozy_servers::hardware::{TheDriversOwnAnswer, WhereK1PlacesIt};
        let v = |text: &str| crate::blender::Version::parse(text);
        let a_driver = |version: &str, placed| TheDriversOwnAnswer {
            version: DriverVersion::parse(version).expect("a version"),
            cuda_version: None,
            where_k1_placed_the_machine: placed,
        };
        let old = a_driver("550.163.01", WhereK1PlacesIt::ItsOwnKernel);
        assert!(a_job_is_held_for_the_driver(true, Some(&old), v("5.2.2"), true, false));
        // 1. Its launch would not name OptiX (the processor, another make, or
        //    a device not known at submission).
        assert!(!a_job_is_held_for_the_driver(false, Some(&old), v("5.2.2"), true, false));
        // 2. The driver meets the job's own bar — 550 is above 4.5's 535 — or
        //    there is no driver reading of this connection.
        assert!(!a_job_is_held_for_the_driver(true, Some(&old), v("4.5.3"), true, false));
        assert!(!a_job_is_held_for_the_driver(
            true,
            Some(&a_driver("580.178.04", WhereK1PlacesIt::ItsOwnKernel)),
            v("5.2.2"),
            true,
            false
        ));
        assert!(!a_job_is_held_for_the_driver(true, None, v("5.2.2"), true, false));
        // 3. No swap owed for this period.
        assert!(!a_job_is_held_for_the_driver(true, Some(&old), v("5.2.2"), false, false));
        // 4. `Servers` H11b's statement stands: a container, or a missed swap.
        assert!(!a_job_is_held_for_the_driver(
            true,
            Some(&a_driver("550.163.01", WhereK1PlacesIt::AContainer)),
            v("5.2.2"),
            true,
            false
        ));
        assert!(!a_job_is_held_for_the_driver(true, Some(&old), v("5.2.2"), true, true));
    }

    /// **R1c's standing hold, read on a later connection**: owed below the bar
    /// on a machine item 5 swaps on; released at the bar; released with the
    /// mark where the statement stands; nothing with no driver reading.
    #[test]
    fn a_standing_hold_for_the_driver_is_compared_against_its_own_blender() {
        use smoozy_servers::hardware::{TheDriversOwnAnswer, WhereK1PlacesIt};
        let v = |text: &str| crate::blender::Version::parse(text);
        let a_driver = |version: &str, placed| TheDriversOwnAnswer {
            version: DriverVersion::parse(version).expect("a version"),
            cuda_version: None,
            where_k1_placed_the_machine: placed,
        };
        let old = a_driver("550.163.01", WhereK1PlacesIt::ItsOwnKernel);
        let new = a_driver("580.178.04", WhereK1PlacesIt::ItsOwnKernel);
        assert_eq!(
            what_a_standing_hold_comes_to(Some(&old), false, v("5.2.2")),
            WhatAStandingHoldComesTo::ASwapIsOwed
        );
        assert_eq!(
            what_a_standing_hold_comes_to(Some(&new), false, v("5.2.2")),
            WhatAStandingHoldComesTo::ItIsReleased
        );
        assert_eq!(
            what_a_standing_hold_comes_to(Some(&old), true, v("5.2.2")),
            WhatAStandingHoldComesTo::ItIsReleasedWithTheMark
        );
        assert_eq!(
            what_a_standing_hold_comes_to(None, false, v("5.2.2")),
            WhatAStandingHoldComesTo::Nothing
        );
    }

    #[test]
    fn a_swap_is_owed_only_below_the_bar_on_a_machine_item_5_swaps_on() {
        let v = |text: &str| crate::blender::Version::parse(text).expect("a version");
        let five_two = the_bar_for_the_newest_blender_on_the_server(None, &[v("5.2.2")]);
        let old = DriverVersion::parse("550.163.01");
        let new = DriverVersion::parse("580.178.04");
        assert!(a_swap_is_owed(MachineKind::ASystemdMachine, old.as_ref(), five_two, false));
        assert!(!a_swap_is_owed(MachineKind::ASystemdMachine, new.as_ref(), five_two, false));
        assert!(!a_swap_is_owed(MachineKind::AContainer, old.as_ref(), five_two, false));
        assert!(!a_swap_is_owed(MachineKind::TheTableDoesNotPlaceIt, old.as_ref(), five_two, false));
        assert!(!a_swap_is_owed(MachineKind::ASystemdMachine, None, five_two, false));
        assert!(!a_swap_is_owed(MachineKind::ASystemdMachine, old.as_ref(), None, false));
        // **A missed swap stands until the next ready action.**
        assert!(!a_swap_is_owed(MachineKind::ASystemdMachine, old.as_ref(), five_two, true));
        // **A 4.5 alone reads 535's row, which 550 meets.**
        let four_five = the_bar_for_the_newest_blender_on_the_server(None, &[v("4.5.3")]);
        assert!(!a_swap_is_owed(MachineKind::ASystemdMachine, old.as_ref(), four_five, false));
    }

    /// **The bar is `Rendering` L23a's one table, read for the version the run was asked
    /// for**; a version that does not read compares nothing.
    #[test]
    fn the_bar_is_the_row_of_the_blender_the_run_installs() {
        let bar = |text: &str| {
            the_bar_for_the_blender_this_run_installs(Some(text)).map(|row| row.driver_minimum)
        };
        assert_eq!(bar("5.2.2"), Some(&[575][..]));
        assert_eq!(bar("5.2.0"), Some(&[570][..]));
        assert_eq!(bar("4.5.3"), Some(&[535][..]));
        assert_eq!(bar("not a version"), None);
        assert_eq!(the_bar_for_the_blender_this_run_installs(None), None);
    }

    #[test]
    fn an_installed_driver_that_does_not_answer_gets_the_restart_before_any_install() {
        // The reinstall-becomes-upgrade observation: both observed failure
        // mechanisms are fixed by the boot.
        let mut needs = a_bare_systemd_machine();
        assert_eq!(
            what_the_driver_step_does(&needs),
            WhatTheDriverStepDoes::TheRestartBeforeAnyInstall
        );
        needs.a_driver_is_already_installed = false;
        assert_eq!(
            what_the_driver_step_does(&needs),
            WhatTheDriverStepDoes::TheDistributionsOwnDetectionRoute
        );
    }

    #[test]
    fn the_already_installed_branch_is_the_checks_published_exit_and_the_running_kernels_module() {
        // R1 item 5's *where a driver is already installed*, read off the check
        // R1 item 5 runs first — and, **from 2026-09-30, only where the running
        // kernel has the driver's module** (R1 item 5 as corrected that day).
        use crate::readings::{
            TheRunningKernelsModule, WhatTheDriverCheckSaid,
            THE_EXIT_THAT_MEANS_A_DRIVER_IS_INSTALLED_AND_NOT_LOADED,
        };
        let said = |exit: Option<u32>| WhatTheDriverCheckSaid::ItDidNot {
            the_exit: exit,
            the_machines_own_words: String::new(),
            what_it_wrote_to_standard_output: String::new(),
        };
        let nine = said(Some(THE_EXIT_THAT_MEANS_A_DRIVER_IS_INSTALLED_AND_NOT_LOADED));
        assert!(nine.a_driver_is_already_installed(TheRunningKernelsModule::IsThere));
        // **Machine 1 of step 10c's eleventh pass**: the utilities stood, the
        // tool ended at 9, and no kernel had a module. **That is not an
        // installed driver**, and the restart cannot mend it.
        assert!(!nine.a_driver_is_already_installed(TheRunningKernelsModule::IsNotThere));
        // **Every other published value, and the one that is no value at all,
        // read as *no driver installed*** whatever the module — the tool itself
        // is what a driver install puts on the machine.
        for other in [0, 2, 3, 4, 6, 8, 10, 12, 13, 14, 15, 127, 255] {
            assert!(
                !said(Some(other)).a_driver_is_already_installed(TheRunningKernelsModule::IsThere),
                "exit {other} does not say a driver is installed"
            );
        }
        assert!(!said(None).a_driver_is_already_installed(TheRunningKernelsModule::IsThere));
        assert!(!WhatTheDriverCheckSaid::ItAnswered
            .a_driver_is_already_installed(TheRunningKernelsModule::IsThere));
    }

    /// **R1 item 5's two branches, on the two machines the eleventh pass tells
    /// apart.** A driver whose module the running kernel holds takes the
    /// restart before any install; a driver whose module is absent takes the
    /// family's install route, *"the same route a bare machine takes"*. Added
    /// 2026-09-30, eleventh-pass fix piece 10.
    #[test]
    fn a_driver_with_no_module_for_the_running_kernel_takes_the_install_route() {
        use crate::readings::{
            TheRunningKernelsModule, WhatTheDriverCheckSaid,
            THE_EXIT_THAT_MEANS_A_DRIVER_IS_INSTALLED_AND_NOT_LOADED,
        };
        let nine = WhatTheDriverCheckSaid::ItDidNot {
            the_exit: Some(THE_EXIT_THAT_MEANS_A_DRIVER_IS_INSTALLED_AND_NOT_LOADED),
            the_machines_own_words: String::new(),
            what_it_wrote_to_standard_output: String::new(),
        };
        let mut needs = a_bare_systemd_machine();
        needs.a_driver_is_already_installed =
            nine.a_driver_is_already_installed(TheRunningKernelsModule::IsThere);
        assert_eq!(
            what_the_driver_step_does(&needs),
            WhatTheDriverStepDoes::TheRestartBeforeAnyInstall
        );
        needs.a_driver_is_already_installed =
            nine.a_driver_is_already_installed(TheRunningKernelsModule::IsNotThere);
        assert_eq!(
            what_the_driver_step_does(&needs),
            WhatTheDriverStepDoes::TheDistributionsOwnDetectionRoute
        );
    }

    #[test]
    fn a_scan_that_did_not_run_never_concludes_no_card() {
        // R1 item 1 and `Servers` H11's fifth row. `lspci` was absent from
        // every container image in the batch, so *did not run* is the ordinary
        // state on a whole machine kind rather than a corner.
        assert_eq!(
            read_card_presence(
                BusScan::NoRoute,
                DriverAnswer::Absent,
                MachineKind::ASystemdMachine
            ),
            Cards::CouldNotBeEstablished
        );
        assert_eq!(
            read_card_presence(
                BusScan::Empty,
                DriverAnswer::Absent,
                MachineKind::ASystemdMachine
            ),
            Cards::NoCard
        );
        // On a container the *empty* row is not available at all.
        assert_eq!(
            read_card_presence(BusScan::Empty, DriverAnswer::Absent, MachineKind::AContainer),
            Cards::CouldNotBeEstablished
        );
    }

    #[test]
    fn the_readings_come_before_the_hand_over_and_the_hand_over_before_the_placement() {
        // R2's order, and M1a's *nothing is touched* is false without it.
        assert_eq!(
            OnAMachineNeverPrepared::IN_ORDER[0],
            OnAMachineNeverPrepared::TheKindAndTheFamily
        );
        assert_eq!(
            OnAMachineNeverPrepared::IN_ORDER[1],
            OnAMachineNeverPrepared::TheCandidateSetAndTheHandOver
        );
        // **The index this test used to skip**, which is the placement — so the
        // test named for *the hand-over before the placement* never asserted it.
        assert_eq!(
            OnAMachineNeverPrepared::IN_ORDER[2],
            OnAMachineNeverPrepared::ThePlacement
        );
        assert_eq!(
            OnAMachineNeverPrepared::IN_ORDER[3],
            OnAMachineNeverPrepared::TheSetupRun
        );
        assert!(!a_second_placement_is_started(true));
        assert!(a_second_placement_is_started(false));
    }

    #[test]
    fn ready_never_claims_the_machine_renders_on_its_cards() {
        // R4 and W2's bound: with the driver missing or wrong every version
        // rendered on the processor and reported success.
        let needs = a_bare_systemd_machine();
        let claims = what_ready_claims(&needs, true);
        assert_eq!(claims.it_renders_on_its_cards, ());
        assert_eq!(claims.the_driver_answers, Some(false));
    }

    #[test]
    fn the_driver_half_of_the_claim_is_absent_where_no_card_is_present() {
        let mut needs = a_bare_systemd_machine();
        needs.cards = Cards::NoCard;
        assert_eq!(what_ready_claims(&needs, true).the_driver_answers, None);
        needs.cards = Cards::CouldNotBeEstablished;
        assert_eq!(what_ready_claims(&needs, true).the_driver_answers, None);
    }

    #[test]
    fn every_one_of_r5s_six_negatives_is_refused() {
        assert_eq!(ItNever::ALL.len(), 6);
        for it in ItNever::ALL {
            assert!(!the_action_may(it));
        }
    }

    #[test]
    fn a_container_with_a_non_root_login_gets_verification_alone() {
        assert_eq!(
            what_a_container_gets(false),
            WhatAContainerGets::VerificationAlone
        );
        assert_eq!(
            what_a_container_gets(true),
            WhatAContainerGets::VerificationAndTheUserSpaceHalf
        );
        // R3's reading rule, and R8's whole defence.
        assert!(!a_user_space_library_version_is_evidence_the_pairing_works(
            true
        ));
        const { assert!(NOTHING_INSPECTS_THE_LOADER_STATE) };
    }

    #[test]
    fn an_unrefreshed_device_list_is_never_read_as_a_cardless_machine() {
        // R1a. A four-card machine and a cardless one give the same answer.
        assert!(!an_unrefreshed_device_list_establishes_no_cards(true));
        const { assert!(NOTHING_HERE_READS_BLENDERS_OWN_DEVICE_LIST) };
    }

    #[test]
    fn secure_boot_is_never_read_to_decide_an_ending() {
        // R9: the ending is W1's *could not be made ready*, with the machine's
        // own words behind it, and nothing here reads the firmware.
        assert!(!secure_boot_is_read_to_decide_the_ending(()));
    }

    #[test]
    fn the_tooling_steps_install_is_issued_only_where_the_tool_is_absent() {
        // **R1 item 3's *where absent*, and Y1's own pair of examples.** The
        // step itself is in the run either way — there is an archive to unpack
        // — and the install is what the probe decides.
        let mut needs = a_bare_systemd_machine();
        assert!(compose_the_run(&needs).contains(&Step::Tooling));
        assert!(the_tooling_step_installs_the_tool(&needs));

        needs.the_unpack_tool_is_present = true;
        assert!(
            compose_the_run(&needs).contains(&Step::Tooling),
            "the archive still has to be unpacked"
        );
        assert!(!the_tooling_step_installs_the_tool(&needs));
    }

    #[test]
    fn the_unpack_tool_is_the_one_blenders_documentation_names() {
        assert_eq!(THE_UNPACK_TOOLS_PACKAGE, "xz-utils");
        const { assert!(THE_ABSENT_TOOL_FAILURE_IS_LOUD_AND_TOTAL) };
        const { assert!(THE_UNPACK_IS_NOT_BUDGETED_AS_A_TRANSFER) };
        const { assert!(NOTHING_BUDGETS_A_FIXED_RESTART_WAIT) };
    }

    #[test]
    fn the_ubuntu_route_is_reached_through_the_family_and_not_named_here() {
        // M1b: the driver route is per family and this module selects rather
        // than composes it.
        let needs = a_bare_systemd_machine();
        let route = crate::kind::TheDriverRoute::for_family(needs.family);
        assert!(!route.the_restart_is_not_optional());
        assert_eq!(THE_DRIVER_CHECK.as_str(), "nvidia-smi");
    }
}
