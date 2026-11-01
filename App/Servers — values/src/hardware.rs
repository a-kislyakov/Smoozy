//! The hardware reading — `Servers` §8.
//!
//! **§8.2's own heading is *what the reading is made of, and every part of it
//! is a trap closed*.** Each clause below is a route a builder would otherwise
//! take, with the observation that shows it gives the wrong answer.
//!
//! **H14: the reading is taken over the pre-package surface, under the login
//! Smoozy holds, before anything is installed.** **H15: it is taken once per
//! primary connection**, after the not-Linux check and after the installation
//! identity is read, **and never on a schedule**. Nothing polls it and nothing
//! re-reads it to keep a screen fresh.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` F1, F2, F3,
//! F16. `Queue & History` G1, G4. `Rendering` L23, L23a. `Rented-server runtime`
//! V6.

use std::time::SystemTime;

use smoozy_state::identities::{InstallationCheck, MachineDerivedValue};
use smoozy_state::observed::{Clock, MachineClock, Reading};

/// **H1: a server's hardware has three states and the set is closed.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HardwareState {
    /// **No reading Smoozy may show is held against this row.** What is shown
    /// is **that there is nothing to show**, rather than an empty row. A server
    /// that has never been connected is in this state, and **so is one whose
    /// held reading has been disqualified**.
    NeverRead,
    /// A reading is held, and the *live* row's condition does not hold. The
    /// reading is shown **marked as what was last seen, with the moment it was
    /// taken**.
    LastSeen,
    /// **H1a: keyed on the connection the reading was taken on, never on the
    /// period alone.**
    ///
    /// As H1 stood, *live* read *"a reading was taken in the period that stands
    /// now"* — and S12's *a drop does not end a period* then kept **a destroyed
    /// machine's row in the *live* state**: its cards, its memory and its free
    /// disk stated as present facts, unqualified, beside an observed value
    /// flickering *reconnecting*.
    ///
    /// **The consolidation's suggested condition — *taken in the period that
    /// stands now and the primary connection is live* — was tested and not
    /// taken**: on H18's fourth branch it calls an earlier connection's reading
    /// *live* the moment the repair's connection comes up, which is exactly what
    /// H18 and H21 forbid.
    Live,
}

/// The two facts H1a keys the state on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhichConnectionHoldsIt {
    /// Whether the held reading was taken **on the connection that is live
    /// now** — not merely inside the period that stands now.
    pub taken_on_the_connection_that_is_live_now: bool,
}

/// H1's table, performed.
pub fn hardware_state(
    held: Option<&HardwareReading>,
    on: WhichConnectionHoldsIt,
    check: InstallationCheck,
) -> HardwareState {
    let Some(_) = held else {
        return HardwareState::NeverRead;
    };
    // **H18: the two branches that discard return the row to *never read*.**
    if check.what_becomes_of_machine_derived_values() == MachineDerivedValue::Disqualified {
        return HardwareState::NeverRead;
    }
    if on.taken_on_the_connection_that_is_live_now {
        HardwareState::Live
    } else {
        HardwareState::LastSeen
    }
}

/// **H4: there is one durable last-seen record per server row and a reading
/// replaces it whole.**
///
/// **Nothing merges a fresh reading into an old one, and no part of a record
/// outlives the reading it came from** — a half-updated record would show one
/// machine's cards beside another machine's disk.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HardwareReading {
    pub cards: Cards,
    pub machine_memory: MachineMemory,
    /// **Absent where the server records no folder to read the figure against,
    /// and the absence is a stated one rather than a substituted figure.**
    ///
    /// H10 is a **per-path** reading, and the path is the one `Job files and
    /// outputs` **L1** chooses once per installation at hand-over and records
    /// against the server row. **A machine no hand-over has reached records no
    /// such root**, so there is nothing to take the reading against and the
    /// figure is absent — which is the reading being honest rather than a gap.
    ///
    /// The route itself exists: a path is exactly the kind of value
    /// `Server protocol and transfers` §1.1 keeps out of a command string, so
    /// **F2's two routes are the only ones** — the `statvfs@openssh.com`
    /// extension, or §1.1's planted-link carriage — and the first is built and
    /// was read off a rented machine at two real locations on 2026-09-03.
    ///
    /// **Nothing is substituted for it.** A whole-machine figure and a figure
    /// derived from a filesystem list are what H10 and F16 forbid outright,
    /// and the container observation behind them is the reason: 51
    /// filesystems, one of them the host's own disk showing 20 GiB the rental
    /// may not use.
    pub free_space: Option<FreeSpace>,
    /// **H2: the moment and the clock travel with the reading and are never
    /// subtracted from anything.** The reading carries the moment it was taken
    /// **and the machine's own clock it was taken on**, and no duration is
    /// computed by subtracting it from the desktop's clock. **What binds it is
    /// `State, identity and recovery` D5** — that document's D5 and not this
    /// one's, which is about history records being keyed by the job identity.
    /// **The plan's own H2 writes it bare and the ambiguity is inherited rather
    /// than invented here.**
    pub the_machines_own_moment: Reading<()>,
    /// **H2a: the reading carries a second moment, and it is that one the *last
    /// seen* mark is drawn from.**
    ///
    /// H2 leaves the wording to Design and Design words it in three
    /// granularities — *last seen 11:04*, *last seen 3 Aug 18:12*, *last seen 28
    /// Jul* — **and choosing between them is a comparison against now, which
    /// H2's machine-clock moment cannot supply without the subtraction
    /// `State, identity and recovery` D5 forbids**.
    ///
    /// So ***last seen* is the desktop's moment, and the mark therefore means
    /// *when Smoozy last had a reading*** rather than *when the machine took
    /// one*. **The two are never subtracted from each other**, and neither is
    /// ever shown as a duration. **Where the desktop's clock has moved
    /// backwards**, a mark can read as in the future; **it is drawn as it stands
    /// and nothing is corrected**, because correcting it would be arithmetic
    /// across an undefended clock.
    pub the_desktops_own_moment: Reading<()>,
    /// **H17: every held reading carries the installation identity present on
    /// the machine when it was taken, or the explicit absence of one.** §2.3
    /// requires the carrying; **the explicit absence is what makes the rule work
    /// for the ordinary case of a machine Smoozy has not prepared**.
    pub read_under: Option<String>,
    /// **H11b: the driver's version and the CUDA version it reports**, part of
    /// this reading and replaced whole with it (H4). **Absent where the driver
    /// did not answer or gave no version** — never a reason the rest fails.
    pub the_driver: Option<TheDriversOwnAnswer>,
}

impl HardwareReading {
    /// **H11b's part, added to a reading taken by [`HardwareReading::taken`].**
    /// A reading made any other way — the machine program's own — carries none.
    pub fn with_the_driver(mut self, the_driver: Option<TheDriversOwnAnswer>) -> HardwareReading {
        self.the_driver = the_driver;
        self
    }

    /// Both moments, taken as the two clocks they belong to. **The second is
    /// written when the reading lands and is never computed from the first.**
    pub fn taken(
        cards: Cards,
        machine_memory: MachineMemory,
        free_space: Option<FreeSpace>,
        on_the_machines_clock: SystemTime,
        server: &str,
        received_here_at: SystemTime,
        read_under: Option<String>,
    ) -> HardwareReading {
        HardwareReading {
            cards,
            machine_memory,
            free_space,
            the_machines_own_moment: Reading::taken(
                (),
                on_the_machines_clock,
                Clock::AMachine(MachineClock(server.to_string())),
            ),
            the_desktops_own_moment: Reading::taken((), received_here_at, Clock::ThisComputer),
            read_under,
            the_driver: None,
        }
    }

    /// **H2 and H2a: the two moments are never subtracted from each other.**
    /// The spine's [`Reading::since`] already refuses it across clocks; this
    /// names the refusal where a builder would look for the arithmetic.
    pub fn how_long_between_the_two_moments(&self) -> Option<std::time::Duration> {
        self.the_desktops_own_moment
            .since(&self.the_machines_own_moment)
    }
}

/// **H11: four card states exist, not two, and the discrimination is by the bus
/// scan wherever there is one.**
///
/// Observed (Fact 46): **a card present with no driver makes the driver's tool
/// simply absent, byte-identical to what a cardless machine gives**; a driver
/// with no card gives `No devices were found`, exit 6. *"Anything concluding
/// "this machine has no GPU" from `nvidia-smi` failing will misclassify a
/// perfectly good rented A6000 as cardless — and on two of these three
/// providers that is not a corner case, it is what they shipped."*
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cards {
    /// The cards, in full.
    TheCards(Vec<Card>),
    /// **Cards are present and cannot be read**, naming which of the two it
    /// was.
    PresentAndUnreadable(WhyUnreadable),
    /// **No card.** The one conclusion that needs the scan to have answered
    /// *empty*.
    NoCard,
    /// **The cards could not be established**, naming that the scan is
    /// unavailable.
    CouldNotBeEstablished,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyUnreadable {
    TheDriverIsAbsent,
    TheDriverAnsweredWithNone,
}

/// What the bus scan said.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BusScan {
    Populated,
    Empty,
    /// **H11's fifth row, added 2026-08-22, and without it the table has no
    /// input on a whole machine kind rather than a lying one.** `lspci` is
    /// **absent on every Vast container in the batch — A, E and F** — and
    /// present on the Hyperstack virtual machine. `pciutils` is not in the
    /// image, and installing it is a package operation the pre-package reading
    /// is taken before. **So on that kind the scan does not return `empty`; it
    /// does not run.** **Those two must never be collapsed**, because `empty` is
    /// the one row that concludes *no card*.
    NoRoute,
}

/// What the driver's own query said.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DriverAnswer {
    Answered(Vec<Card>),
    Absent,
    NoDevicesWereFound,
}

/// The machine kind, **settled first** — `Rented-server runtime` V6's detection,
/// cited.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineKind {
    /// **H12: on a container the bus scan is the host's unfiltered view, so the
    /// *empty* row is not available there.** Observed — the scan *"listed the
    /// host's own storage controller, its audio device and its network card,
    /// none of which belong to the container"*, and *"in a view with genuinely
    /// no cards, `/proc/driver/nvidia` and the PCI sysfs scan lie"*, both
    /// inherited from the host kernel. **On a container the reading never claims
    /// *no card*; it says the cards could not be established.**
    ///
    /// **H12a: the reason widens and the conclusion is strengthened.** On the
    /// container H12 was written from, the scan **lied**. On the three
    /// containers of the multi-provider batch the scan **does not exist**. **Two
    /// different failures, one outcome.**
    AContainer,
    /// **H12b: one case is uncovered by both clauses and is named rather than
    /// left silent** — a machine that is **not** a container and has **no**
    /// `lspci` either. Nothing in the batch was such a box, so it is unobserved
    /// rather than dismissed. **H11's fifth row is what meets it.**
    NotAContainer,
}

/// **H11's table, performed, with H12's kind gate in front of the *empty* row.**
///
/// **The fifth row is keyed on the driver's answer as well as on the scan, and
/// that is a correction of 2026-08-23.** As written it read *anything* in the
/// driver column, **so a container on which the driver's own query answers
/// honestly with the rental's cards was still ruled *the cards could not be
/// established*** — discarding a good reading on precisely the boxes the row was
/// added for. §16 item 2 records that `nvidia-smi` was honest on both of those
/// hosts. **The scan is only ever a discriminator where the driver does not
/// answer.**
///
/// **What does not change is the prohibition either way**: a missing scan never
/// concludes *no card*, and a missing driver never concludes it either.
pub fn read_the_cards(scan: BusScan, driver: DriverAnswer, kind: MachineKind) -> Cards {
    // **A driver that answers is the reading, whatever the scan's state.**
    if let DriverAnswer::Answered(cards) = driver {
        return Cards::TheCards(cards);
    }
    let why = match driver {
        DriverAnswer::Absent => WhyUnreadable::TheDriverIsAbsent,
        DriverAnswer::NoDevicesWereFound => WhyUnreadable::TheDriverAnsweredWithNone,
        DriverAnswer::Answered(_) => unreachable!("answered is handled above"),
    };
    match scan {
        BusScan::Populated => Cards::PresentAndUnreadable(why),
        // **H12: on a container the *empty* row is not available.**
        BusScan::Empty => match kind {
            MachineKind::AContainer => Cards::CouldNotBeEstablished,
            MachineKind::NotAContainer => Cards::NoCard,
        },
        BusScan::NoRoute => Cards::CouldNotBeEstablished,
    }
}

/// **H6: each card carries an identity that survives two identical cards, and a
/// make that is a keyable value.**
///
/// Observed — both cards on the two-card box were the same model, so *"the UUID
/// and bus location are what tell them apart"*. **Identity comes from the
/// driver's own query, presence from the bus scan.** **Vindicated on a rented
/// multi-card slice, 2026-08-22** — the identifier and the bus location are what
/// named the two cards that were ours on a four-card host, and that pair
/// *"turns out to be the only pair that survives this"*.
///
/// **A card's index, UUID and bus location also survive a restart unchanged**,
/// observed across a reboot on a virtual machine — **on one card, which is the
/// observation's own scope**. **Whether an ordering is stable across a restart
/// is unposed**, which costs nothing because **nothing here is keyed on the
/// index**.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Card {
    pub unique_identifier: String,
    pub bus_location: String,
    /// **H7: the make is a value `Rendering` L23's table can be keyed on, never
    /// a wording to be parsed.** That is `Rendering` §8's requirement by name,
    /// because the card technology named at launch is looked up from it.
    ///
    /// **A reading returning no card at all and a make with no row in that
    /// table are one case**, and that case is `Rendering`'s to answer — **no
    /// submission is withheld anywhere** and the attempt ends on the machine
    /// before the render instruction with the reason recorded.
    pub make: CardMake,
    /// **H8: per-card memory has exactly one route and where that route is
    /// absent the count and the identity survive and the memory does not.**
    ///
    /// Observed — the kernel's own per-card information *"gives model, UUID and
    /// bus location and no memory figure at all"*. **A reading with cards and no
    /// per-card memory is a complete reading with a stated absence and never a
    /// failed one.**
    pub total_memory_bytes: Option<u64>,
}

/// **The make every card the driver's own query names carries** — the key
/// `Rendering` L23's table has its NVIDIA row under, declared once and read by
/// that table (`Rendering — machine`'s `technology.rs`) rather than spelled a
/// second time there.
///
/// **Why the make is this value and not anything the tool prints.** The
/// reading asks one tool for its cards, `nvidia-smi`, and scans the bus for one
/// vendor, `0x10de` (H11, H13) — both NVIDIA's own, which is what the spec's
/// *"the card maker's own tool"* names. **So the make is established by the
/// route the reading took, and no card this reading names can be of another
/// make.** Nothing is read out of the card's name to get it. A second maker's
/// route, when one is written, brings its own key with it.
///
/// **What it cost before 2026-09-26, found on a real rented machine.** The
/// make was the tool's own name for the card — *NVIDIA GeForce GTX 1650* —
/// while the table was keyed *NVIDIA*, so on step 10c's sixth pass on Vast
/// every job set to use the cards ended before the render with *Smoozy does
/// not know how to render on a NVIDIA GeForce GTX 1650*. The desktop's test
/// pinned the full name and the machine's tests pinned *NVIDIA*: one fact,
/// two spellings, each half green against its own idea of the value.
pub const NVIDIA: &str = "NVIDIA";

/// **H7: a keyable value, never a wording.** The key is what `Rendering` L23's
/// table is looked up by; **the driver's own name for the card rides beside it
/// and is never keyed on** — it is what the server list shows as *which they
/// are* (the `Servers` spec's *"Its GPUs — how many, which they are"*), and it
/// is prose, not a value.
///
/// **They are two fields rather than one string because they were one string**,
/// and the one string was keyed on in one place and shown in another until a
/// real machine found the two uses disagreed. See [`NVIDIA`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CardMake {
    key: String,
    the_drivers_name_for_the_card: String,
}

impl CardMake {
    /// **A card the driver's own query named** — `nvidia-smi`, NVIDIA's own
    /// tool, so the make is [`NVIDIA`] by the route and the text is the
    /// driver's name for the card, kept whole to be shown.
    ///
    /// **Nothing is parsed out of the text.** A text that happens to begin
    /// *NVIDIA* and one that does not give the same make, because the make
    /// never came from the text.
    pub fn from_the_drivers_own_query(the_drivers_name_for_the_card: &str) -> CardMake {
        CardMake {
            key: NVIDIA.to_string(),
            the_drivers_name_for_the_card: the_drivers_name_for_the_card.to_string(),
        }
    }

    /// **A make established by some other route**, with that route's own name
    /// for the card. No reading takes such a route today — the only query this
    /// module sends is NVIDIA's — and `Rendering` L23's AMD and Intel rows are
    /// filled on documentation alone; this is how a test, or that route when it
    /// is written, puts one of those makes on a card.
    pub fn of_a_make(key: &str, the_drivers_name_for_the_card: &str) -> CardMake {
        CardMake {
            key: key.to_string(),
            the_drivers_name_for_the_card: the_drivers_name_for_the_card.to_string(),
        }
    }

    /// **The key `Rendering` L23's table is looked up by**, and the only part
    /// of this value that may be.
    pub fn as_key(&self) -> &str {
        &self.key
    }

    /// **The driver's own name for the card, to be shown and never keyed
    /// on** — *NVIDIA GeForce GTX 1650*. It is what a person reads as *which
    /// card this is*.
    pub fn the_drivers_name_for_the_card(&self) -> &str {
        &self.the_drivers_name_for_the_card
    }
}

/// **H5: the card count is read as a count and never as a parsed total.**
///
/// `Queue & History` G1 carries the trap and Fact 46 is the observation: the
/// driver's own count query **returns the value once per card**, so *"A caller
/// reading the first line is right by luck; one parsing the whole output as an
/// integer gets `22`"* on a two-card box, and *"It passes every test ever
/// written on a one-card machine."*
///
/// **The count is the number of rows of a per-card query, and nothing else is
/// read as a count.**
///
/// **H5a: and on a slice of a multi-card host, that rule is the only one that
/// counts the cards the rental actually holds.** Two different hosts, each
/// holding **four** cards, each rented as a **two-card** slice: `nvidia-smi -L`
/// and `nvidia-smi --query-gpu=index` returned **2** on both — ours.
/// **`/proc/driver/nvidia/gpus` returned 4 on both**, the extra two UUIDs
/// belonging to another tenant. **So a count taken below the driver's own query
/// counts the host's cards, not the rental's**, and the error is quiet: the
/// reading is not obviously wrong, it is two too many. **This project's own
/// earlier probe used `ls /proc/driver/nvidia/gpus | wc -l` and agreed only
/// because that box had one card and one tenant.**
///
/// **Reading the kernel's per-card directory as a count is forbidden**, and so
/// is any arithmetic — an allotment, a memory total, a price-per-card — built on
/// it.
pub fn card_count(cards: &Cards) -> Option<usize> {
    match cards {
        Cards::TheCards(rows) => Some(rows.len()),
        _ => None,
    }
}

/// **H13: counting display devices is forbidden outright.** Observed — counting
/// the kernel's display-device entries *"reports two cards on a one-card
/// machine"*, because an emulated console adapter is a display device and not a
/// render card, *"so vendor id `0x10de` is the discriminator, never device
/// class"*.
///
/// **H5b: identifying or keying a card by its device node is forbidden too, and
/// the numbering is not the query's.** On the same two boxes, `/dev/nvidia*`
/// exposed **four** nodes on one two-card rental and **two** on the other, and
/// on the second the kernel's `Device Minor` values for **our own** cards were
/// **2** and **0** — **no relationship to `nvidia-smi`'s index 0 and 1**. So
/// **node *n* is not index *n*, the node count is not a card count, and neither
/// is stable between hosts.** **The only card identity is H6's.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForbiddenCardRoute {
    CountingDisplayDevices,
    TheKernelsPerCardDirectoryAsACount,
    ADeviceNodeAsAnIdentity,
    TheDeviceNodeCountAsACardCount,
}

impl ForbiddenCardRoute {
    pub const ALL: [ForbiddenCardRoute; 4] = [
        ForbiddenCardRoute::CountingDisplayDevices,
        ForbiddenCardRoute::TheKernelsPerCardDirectoryAsACount,
        ForbiddenCardRoute::ADeviceNodeAsAnIdentity,
        ForbiddenCardRoute::TheDeviceNodeCountAsACardCount,
    ];
}

/// **H7a: nothing here reads a card's live memory use**, and one reading of it
/// is on the record **with a limit that matters more than the number**.
///
/// Used memory read three times at ten-second intervals on two two-card slices
/// with nothing running: **steady at 1 MiB on both cards of one box and 2 MiB on
/// both of the other**, with the compute-application list empty on both. **The
/// limit is what makes this weak**: the cards the driver's own query shows are
/// the two we hold, so **a neighbouring tenant's work on the host's other two
/// cards could not have appeared in the reading whether it existed or not**.
/// **Whether a card can ever be shared under us was not posed and this test
/// could not pose it.** Recorded so nobody reads the steady figures as evidence
/// of exclusivity.
pub const NOTHING_HERE_READS_LIVE_CARD_MEMORY: bool = true;

/// **H9 and H9a: the machine's own memory reading has two shapes and three
/// outcomes, not one shape and two.**
///
/// **H9: the reading is the container's allocation where there is one.**
/// Observed — a container reported 4 processors and 15.57 GiB against real
/// allocations of **3.84 processors and 14.95 GiB**, and *"On a 1-of-8 rental,
/// `/proc/meminfo` would overstate usable memory eightfold."*
///
/// **H9a's correction, and as it stood the rule fell through to nothing on a
/// host that is not on the older kind.** On a two-card slice of a four-card
/// host, **the older pair answered nothing** — *"`cpu.cfs_quota_us` and
/// `memory.limit_in_bytes` both empty — the host is on cgroup v2 and the v1
/// paths do not exist."* The rental was sold as a **20-core** slice; `nproc`
/// reported **40** and `free` reported the host's **64 GiB**. **So H9 as written
/// had no input, and the fallback's stated trigger — the literal `max` — never
/// fired, because there was no file to read it from.**
///
/// **Both path pairs are named, and the older is not assumed.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineMemory {
    /// A figure — **the rental's allocation**.
    TheAllocation(u64),
    /// **The cgroup read the literal `max`, so the `/proc` figure is the whole
    /// machine and is right, because nothing is capping it.** **And, from
    /// 2026-09-21 on the build runner's ruling, no route at all on a machine with its own kernel**
    /// ([`WhereK1PlacesIt::ItsOwnKernel`]), for the same reason.
    TheWholeMachine(u64),
    /// **The third row, and it is the correction.** No route at all — neither
    /// pair present or both empty — **on a container, or on a machine K1 does
    /// not place** (keyed on the kind 2026-09-21 on the build runner's ruling; see
    /// [`WhereK1PlacesIt`]). **The allocation could not be established**,
    /// carried as a stated absence, **and the `/proc` figure is *not*
    /// substituted for it**: the `/proc` figure on a slice is the host's, and a
    /// reader that reaches it because the cgroup was silent gets the host's
    /// answer while believing it got the rental's — **the eightfold
    /// overstatement H9 was written to prevent, arriving by a route H9 did not
    /// cover.**
    ///
    /// **And the third row's own footing is stated rather than borrowed.** **No
    /// box on record has been read at both path pairs and answered at neither.**
    /// The batch read the older pair on a host running the newer kind and found
    /// it absent; **it did not read the newer pair**. **The third row is
    /// therefore this document's own construction against a case nothing has
    /// posed**, kept because it errs in the one safe direction. Label: **this
    /// document's rule**, not an observation.
    CouldNotBeEstablished,
}

/// What a cgroup path pair answered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CgroupAnswer {
    AFigure(u64),
    /// **The fallback's trigger is the literal `max` and nothing else** —
    /// observed on the reproduction, *"the high-water reading answers the
    /// literal `max`, so a reader taking that instead falls back to the wrong
    /// figure."*
    TheLiteralMax,
    /// The pair is not present, or is present and empty.
    NoRoute,
}

/// **Where `Rented-server runtime` K1's table places the machine, as far as
/// H9a's absence row turns on it.** Added 2026-09-21 on the build runner's ruling.
///
/// On a plain virtual machine neither control-group memory file exists at the
/// root of the tree — observed on a rental that day — so the absence row, written
/// for a container slice, caught every virtual machine and bare-metal server and
/// told the user their memory could not be read. **H9a now splits that row on
/// this value and on nothing else.**
///
/// **Why three and not [`MachineKind`]'s two.** *Not a container* includes the
/// machine K1 refuses — neither container reading, and systemd not the first
/// process — and nothing says that machine runs its own kernel. **Only K1's
/// second row does**, so only it takes the machine's own figure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhereK1PlacesIt {
    /// K1's first row: the marker file, or the tool's container-only form.
    /// **That form asks whether the kernel is shared**, so a systemd-booted
    /// system container of any technology the tool names lands here and keeps
    /// the absence. One it does not name cannot be seen by any reading —
    /// `Rented-server runtime` §16 item 33.
    AContainer,
    /// K1's second row: neither container reading, **and** systemd is the
    /// machine's first process. A virtual machine or bare metal.
    ItsOwnKernel,
    /// K1's third row, which that document refuses. **Never read as its own
    /// kernel.**
    NowhereItPlaces,
}

impl WhereK1PlacesIt {
    /// K1's table, from the container reading [`MachineKind`] already carries
    /// and the first-process reading beside it.
    pub fn from_the_two_readings(kind: MachineKind, systemd_is_the_first_process: bool) -> Self {
        match (kind, systemd_is_the_first_process) {
            (MachineKind::AContainer, _) => WhereK1PlacesIt::AContainer,
            (MachineKind::NotAContainer, true) => WhereK1PlacesIt::ItsOwnKernel,
            (MachineKind::NotAContainer, false) => WhereK1PlacesIt::NowhereItPlaces,
        }
    }
}

/// H9a's three rows, performed. **The reading is attempted at the newer paths —
/// `cpu.max` and `memory.max` — and at the older — `cpu.cfs_quota_us` with
/// `cpu.cfs_period_us`, and `memory.limit_in_bytes`. Whichever answers is the
/// reading.**
///
/// **The `/proc` figure is *a figure* or *nothing came back*, and never a zero
/// standing for both.** Where the cgroup read the literal `max` and no figure
/// came back — the request did not reach the machine, the machine answered
/// badly, or it named its units in a way this reading does not know — the
/// answer is the third row: **the memory could not be established**. A zero
/// passed through as though it were a fact would put *no memory at all* in
/// front of a user choosing a machine to render on, and Rule 8 makes Smoozy
/// saying something it does not know a fault whatever caused it. **It is the
/// same shape the machine's own clock reading next door already takes**, for
/// the same reason.
///
/// **The absence row is split by [`WhereK1PlacesIt`] — the build runner's ruling, 2026-09-21.** Where
/// neither pair answers, a machine K1 places at its second row takes the
/// `/proc` figure as the whole machine, because it runs its own kernel and
/// nothing caps it; **a container, and a machine K1 does not place, keep the
/// stated absence** exactly as before. The figure and the literal `max` rows do
/// not look at the kind at all.
pub fn read_the_machines_memory(
    placed: WhereK1PlacesIt,
    newer_paths: CgroupAnswer,
    older_paths: CgroupAnswer,
    proc_figure: Option<u64>,
) -> MachineMemory {
    for answer in [newer_paths, older_paths] {
        match answer {
            CgroupAnswer::AFigure(figure) => return MachineMemory::TheAllocation(figure),
            CgroupAnswer::TheLiteralMax => {
                return match proc_figure {
                    Some(figure) => MachineMemory::TheWholeMachine(figure),
                    None => MachineMemory::CouldNotBeEstablished,
                }
            }
            CgroupAnswer::NoRoute => continue,
        }
    }
    // **The absence row, keyed on the kind — the build runner's ruling, 2026-09-21.** Only a machine
    // K1 places at its second row runs its own kernel, so only there is the
    // `/proc` figure the machine's own when nothing caps it.
    match (placed, proc_figure) {
        (WhereK1PlacesIt::ItsOwnKernel, Some(figure)) => MachineMemory::TheWholeMachine(figure),
        _ => MachineMemory::CouldNotBeEstablished,
    }
}

/// **H10: free space is a choice of mount and the reading names the path it was
/// taken against.**
///
/// Observed — a plain reading on one container returned **51 filesystems, 48 of
/// them memory-backed**, and of the three that were not, one was *"the host's
/// own 32 GiB disk bind-mounted in to deliver the driver files, showing 20 GiB
/// free that the rental may not use"*.
///
/// So the reading is `Server protocol and transfers` F1 to F3's **per-path**
/// reading, taking the **available** figure and **never the free figure**, and
/// it is taken against **the path a job's folders sit under** — the same path
/// `Queue & History` G4 reads, **so the server list and a queue entry never
/// disagree about one machine**.
///
/// **Where that path does not exist yet, the reading is taken against its
/// deepest existing ancestor and the record names the path it was actually taken
/// against.**
///
/// **A figure derived from a filesystem list is forbidden**, and so is widening
/// a per-path answer into a per-machine one.
///
/// **H3: free disk is explicitly allowed to be wrong.** A job left running
/// writes files, so a held figure is only what was true when the reading was
/// taken. **The remembered figure is never shown as current**: the same
/// last-seen mark covers it. Label: **Creator-stated** (2026-08-13).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreeSpace {
    /// **The available figure, never the free figure.**
    pub available_bytes: u64,
    /// **The path it was actually taken against** — the job root, or its
    /// deepest existing ancestor.
    pub the_path_it_was_taken_against: String,
}

/// **H18: what each of §2.3's four branches does to a held reading, and the
/// fresh reading is taken on three of them.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhatTheBranchDoes {
    pub the_held_reading: MachineDerivedValue,
    pub a_fresh_reading_is_taken: bool,
}

pub fn what_the_branch_does(check: InstallationCheck) -> WhatTheBranchDoes {
    WhatTheBranchDoes {
        the_held_reading: check.what_becomes_of_machine_derived_values(),
        // **H21: H19's argument does not reach the fourth branch, and nothing
        // is taken there.** §2.3's requirement on that branch is not that the
        // held value go unused but that it be **unchanged**, and H4's *a
        // reading replaces it whole* is exactly what a fresh reading would do
        // to it. A reading taken there would also carry an **unconfirmed**
        // identity and would be shown as **live** under H1 — the thing §2.3
        // forbids in terms.
        a_fresh_reading_is_taken: !matches!(
            check,
            InstallationCheck::PresentAndTheSupervisorDidNotAnswer
        ),
    }
}

/// **H19: the fresh reading is taken on the three branches above that take one,
/// including the two that stop §7's desktop reconciliation at step 1.**
///
/// That step's stop governs reading **Smoozy's own state on the machine** — it
/// names the queue, the note, the journal and the history copy — **and a
/// hardware reading is none of those**. It is a reading of the machine in front
/// of Smoozy rather than a use of state Smoozy holds, **which is exactly what a
/// newly added, never-prepared server needs.**
pub const A_HARDWARE_READING_IS_NOT_SMOOZYS_OWN_STATE: bool = true;

/// **H20: a container recycle is the case this exists for.** *"nothing in the
/// connection distinguishes a recycled machine from a rebooted one"* — same
/// address, same port, back in 90 seconds — **so a remembered reading shown
/// against a recycled machine would be a reading of a different machine**. The
/// disqualification is the one act that covers it, and **it is a single act
/// because the carrying rule is a single rule**.
pub const THE_RECYCLE_IS_WHAT_THE_DISQUALIFICATION_IS_FOR: bool = true;

/// **H21a: that row's *none is taken* is about the hardware reading and about
/// nothing else.** Q19 re-takes step 5's **identity** read while the fourth
/// branch stands, and takes no reading of this section's. **A fresh hardware
/// reading is taken when that read settles on one of the other three
/// branches**, under H18's own row for whichever it is.
pub const THE_FOURTH_BRANCHS_NONE_IS_TAKEN_IS_THIS_READING_ALONE: bool = true;

/// **H16: no Blender is in it, and that is a decision rather than an
/// oversight.** Four reasons, and the fourth is the strongest **because it
/// defeats the reading on a machine that has cards**.
///
/// 1. **The promise covers any connected server, including one with no Blender
///    on it** — the ordinary case at the moment a server is added.
/// 2. **Card memory has never been seen except through Blender's window, never
///    with more than one card**, so the route answers less than the driver's own
///    query does.
/// 3. From `--factory-startup` on a rented multi-card box, in both Blender
///    versions, `compute_device_type` arrives as **`NONE`** while **every
///    individual device in the list arrives with `use = True`**. **So Blender's
///    own device list read as it comes is not a statement about what the machine
///    has, nor about what Blender will use.**
/// 4. **The repaired Blender research records Cycles' own card list coming back
///    empty on a factory-fresh Blender in all four versions**, filling only once
///    a refresh has been asked for. Label: **observed in a test**. **So a machine
///    asked cold gives the same answer as a cardless one, and this document's
///    whole job at H11 is telling those two apart** — the route would have fed
///    the one row of that table that concludes *no card*.
///
/// **Where the third reason and the fourth disagree, they are not reconciled
/// here**: §16 item 24 carries it, and **no observation will settle it** under
/// the ruling of 2026-08-31.
pub fn a_blender_is_asked_for_the_hardware_reading() -> bool {
    false
}

/// A Blender version as its three numbers — major, minor, patch.
///
/// **It is three numbers and never text**, so 5.10 sorts after 5.9. `Server
/// Setup`'s own `blender::Version` gives them as `(v.major, v.minor, v.patch)`;
/// this crate cannot take that type, because that crate takes this one.
pub type BlenderNumbers = (u32, u32, u32);

/// One row of `Rendering` **L23a**'s table: from this Blender version on, the
/// NVIDIA driver must be at least this for OptiX.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OptixBarRow {
    /// The earliest Blender version the row belongs to.
    pub from_blender: BlenderNumbers,
    /// Blender's own stated OptiX driver minimum (Linux), as the driver's
    /// version numbers.
    pub driver_minimum: &'static [u32],
}

/// **`Rendering` L23a: Blender's own stated OptiX driver minimum per Blender
/// version, and this table is it.** All three rows are **documented** —
/// `Preparation/Architecture/Sources/Rendering.sources.md`, every page read
/// 2026-10-11. **The bar is what Blender's own pages state, never what happened
/// to work on a machine** — 570 rendered on 5.2.2 in a test, and is below 5.2.1's
/// stated 575.
///
/// **This is the only copy.** The desktop reads it through `smoozy-servers`,
/// which re-exports this file, and the machine program reads it from this
/// crate. **A new Blender's bar is one more row here.**
///
/// Rows run oldest first, and a Blender's bar is the last row it has reached —
/// so *4.5 to 5.1* is the first row up to 5.2.0, and *5.2.1 and later* is the
/// last row onward.
pub const THE_OPTIX_DRIVER_BAR: [OptixBarRow; 3] = [
    OptixBarRow {
        from_blender: (4, 5, 0),
        driver_minimum: &[535],
    },
    OptixBarRow {
        from_blender: (5, 2, 0),
        driver_minimum: &[570],
    },
    OptixBarRow {
        from_blender: (5, 2, 1),
        driver_minimum: &[575],
    },
];

/// **The row a Blender version is read against.**
///
/// **A Blender later than the table's last row is read against the last row**
/// (L23a, a decision): Blender may raise its bar later, and until a row is
/// added Smoozy's bar for that version is too low — a loud failure, never a
/// render read as finished.
///
/// **A Blender earlier than the first row has no bar here**, and none is made
/// up: L23a's table starts at 4.5, which is also the whole app's floor
/// (`Server Setup — machine`'s `THE_FLOOR`), so nothing Smoozy installs is
/// earlier.
pub fn the_optix_bar_row(blender: BlenderNumbers) -> Option<OptixBarRow> {
    THE_OPTIX_DRIVER_BAR
        .iter()
        .rev()
        .find(|row| row.from_blender <= blender)
        .copied()
}

/// **The driver's version, as the driver wrote it and as its numbers.**
///
/// **Compared number by number, never as text** (L23a): as text, 1000 would sort
/// below 575. A missing number counts as zero, so `575` and `575.00` are the
/// same version and both meet a bar of 575.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DriverVersion {
    numbers: Vec<u32>,
    as_the_driver_wrote_it: String,
}

impl DriverVersion {
    /// Dot-separated whole numbers, or nothing. **A version that is not that
    /// shape is no version**, never a guess at one.
    pub fn parse(text: &str) -> Option<DriverVersion> {
        let text = text.trim();
        let numbers = text
            .split('.')
            .map(|part| {
                if part.is_empty() || !part.bytes().all(|b| b.is_ascii_digit()) {
                    None
                } else {
                    part.parse::<u32>().ok()
                }
            })
            .collect::<Option<Vec<u32>>>()?;
        Some(DriverVersion {
            numbers,
            as_the_driver_wrote_it: text.to_string(),
        })
    }

    /// The driver's own words, for a screen — `550.163.01`, leading zero kept.
    pub fn as_the_driver_wrote_it(&self) -> &str {
        &self.as_the_driver_wrote_it
    }

    /// **Number by number**, a missing number counting as zero.
    pub fn is_below(&self, minimum: &[u32]) -> bool {
        let longest = self.numbers.len().max(minimum.len());
        for at in 0..longest {
            let ours = self.numbers.get(at).copied().unwrap_or(0);
            let theirs = minimum.get(at).copied().unwrap_or(0);
            if ours != theirs {
                return ours < theirs;
            }
        }
        false
    }
}

/// **`Servers` H11b: where the driver answers, the reading also carries the
/// driver's version and the CUDA version the driver reports** — and the machine
/// kind, because the statement needs the kind at the first connection and it
/// is *"the same reading by K1's own route, taken with this one — not a second
/// mechanism"*.
///
/// **Absent from the reading where the driver did not answer, or answered with
/// no version this reading can read.** That absence never touches the cards: a
/// reading whose driver gave its cards and not its version is a working card
/// reading with this part missing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheDriversOwnAnswer {
    pub version: DriverVersion,
    /// The CUDA version the driver reports, in the driver's own words — shown,
    /// never compared. **Absent where the driver's summary did not give it**,
    /// and the version above still stands.
    pub cuda_version: Option<String>,
    /// `Rented-server runtime` K1, as the desk's reading placed the machine
    /// when this reading was taken.
    pub where_k1_placed_the_machine: WhereK1PlacesIt,
}

/// **H11b's judgement, as a value with no screen.** What the card, the warning
/// and the Blender table's note (the fourth piece of this work) are drawn from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheBarSays {
    /// **The statement H11b owes**, or `None` where nothing is said. It is owed
    /// only where the driver is below a bar in L23a's table **and Smoozy cannot
    /// change the driver** — see [`the_statement_is_said_on`].
    pub the_statement: Option<TheDriverStatement>,
    /// **`Server Setup` B12b**: each Blender on the server whose bar the driver
    /// is below. **Whatever the machine kind** — the note is on the Blender, and
    /// a machine Smoozy can manage that still has an old driver still has one.
    pub blenders_below_their_bar: Vec<BlenderNumbers>,
}

/// What H11b's statement names. The words are Design's (the fourth piece of this work).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheDriverStatement {
    pub the_drivers_version: String,
    pub cuda_version: Option<String>,
    /// **The earliest Blender version the bar belongs to, *and later*.** Where
    /// the driver is below more than one bar, it is the earliest such bar's,
    /// because *and later* then covers every Blender the driver falls short of.
    pub from_blender: BlenderNumbers,
}

/// **H11b: *"It is said only where Smoozy cannot change the driver."***
///
/// **A container**, by K1's kind (`Server Setup` M2). **On any other machine
/// nothing is said**, because setup brings the driver up silently.
///
/// **The second case H11b names — a machine where a setup's swap did not reach
/// the bar (`Server Setup` R1 item 5) — joins here as the second input.
/// Joined 2026-10-11, the third piece of this work.** Where setup could not
/// bring the driver up to the bar, a machine K1 places as its own kernel is
/// told as a container is. The desk learns it from the setup run's own ending
/// (`Desktop app and delivery`'s `setting_up`, *the driver missed the OptiX
/// bar*); **keeping it in the desk's stored record is the fourth piece's**.
pub fn the_statement_is_said_on(placed: WhereK1PlacesIt, setup_could_not_update_it: bool) -> bool {
    match placed {
        WhereK1PlacesIt::AContainer => true,
        WhereK1PlacesIt::ItsOwnKernel => setup_could_not_update_it,
        // K1's refused row: no setup runs there, so no swap is attempted, and
        // H11b names only the container. Told by K3's own words instead.
        WhereK1PlacesIt::NowhereItPlaces => false,
    }
}

/// **H11b's judgement**: given the driver's answer and the Blender versions on
/// the server, whether the statement is owed and which Blenders sit below
/// their bar.
///
/// **It stops nothing** (H11b): a value to be shown, never a refusal.
///
/// **`setup_could_not_update_it`** is [`the_statement_is_said_on`]'s second
/// input, passed through: true where a setup on this machine ended with its
/// driver still below the bar (`Server Setup` R1 item 5).
pub fn what_the_bar_says(
    the_driver: &TheDriversOwnAnswer,
    blenders_on_the_server: &[BlenderNumbers],
    setup_could_not_update_it: bool,
) -> WhatTheBarSays {
    let the_earliest_bar_it_misses = THE_OPTIX_DRIVER_BAR
        .iter()
        .find(|row| the_driver.version.is_below(row.driver_minimum));
    let the_statement = match the_earliest_bar_it_misses {
        Some(row)
            if the_statement_is_said_on(
                the_driver.where_k1_placed_the_machine,
                setup_could_not_update_it,
            ) =>
        {
            Some(TheDriverStatement {
                the_drivers_version: the_driver.version.as_the_driver_wrote_it().to_string(),
                cuda_version: the_driver.cuda_version.clone(),
                from_blender: row.from_blender,
            })
        }
        _ => None,
    };
    let blenders_below_their_bar = blenders_on_the_server
        .iter()
        .copied()
        .filter(|blender| match the_optix_bar_row(*blender) {
            Some(row) => the_driver.version.is_below(row.driver_minimum),
            None => false,
        })
        .collect();
    WhatTheBarSays {
        the_statement,
        blenders_below_their_bar,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn a_card(id: &str) -> Card {
        Card {
            unique_identifier: id.into(),
            bus_location: format!("0000:{id}:00.0"),
            make: CardMake::from_the_drivers_own_query("NVIDIA GeForce RTX 4090"),
            total_memory_bytes: Some(25_757_220_864),
        }
    }

    fn a_reading() -> HardwareReading {
        HardwareReading::taken(
            Cards::TheCards(vec![a_card("01"), a_card("02")]),
            MachineMemory::TheAllocation(16_051_527_680),
            Some(FreeSpace {
                available_bytes: 20_000_000_000,
                the_path_it_was_taken_against: "/opt/smoozy/jobs".into(),
            }),
            SystemTime::UNIX_EPOCH + Duration::from_secs(1_000),
            "srv-1",
            SystemTime::UNIX_EPOCH + Duration::from_secs(2_000),
            Some("inst-1".into()),
        )
    }

    #[test]
    fn the_count_is_the_number_of_rows_and_never_a_parsed_total() {
        // The driver's count query returns the value once per card, so a
        // caller parsing the whole output on a two-card box gets 22.
        let cards = Cards::TheCards(vec![a_card("01"), a_card("02")]);
        assert_eq!(card_count(&cards), Some(2));
    }

    #[test]
    fn no_count_is_offered_where_the_cards_could_not_be_read() {
        assert_eq!(card_count(&Cards::CouldNotBeEstablished), None);
        assert_eq!(
            card_count(&Cards::PresentAndUnreadable(WhyUnreadable::TheDriverIsAbsent)),
            None
        );
        assert_eq!(card_count(&Cards::NoCard), None);
    }

    #[test]
    fn a_populated_scan_with_no_driver_says_present_and_unreadable_and_never_no_card() {
        // The misclassification two of three providers shipped.
        assert_eq!(
            read_the_cards(
                BusScan::Populated,
                DriverAnswer::Absent,
                MachineKind::NotAContainer
            ),
            Cards::PresentAndUnreadable(WhyUnreadable::TheDriverIsAbsent)
        );
        assert_eq!(
            read_the_cards(
                BusScan::Populated,
                DriverAnswer::NoDevicesWereFound,
                MachineKind::NotAContainer
            ),
            Cards::PresentAndUnreadable(WhyUnreadable::TheDriverAnsweredWithNone)
        );
    }

    #[test]
    fn a_missing_scan_never_concludes_no_card() {
        // H11's fifth row: no route is not `empty`, and collapsing the two
        // would reach the one row that concludes *no card*.
        assert_eq!(
            read_the_cards(
                BusScan::NoRoute,
                DriverAnswer::Absent,
                MachineKind::NotAContainer
            ),
            Cards::CouldNotBeEstablished
        );
    }

    #[test]
    fn a_driver_that_answers_is_the_reading_whatever_the_scan_did() {
        // The 2026-08-23 correction: the fifth row is keyed on the driver too,
        // or a container whose driver answers honestly has its good reading
        // discarded.
        let honest = read_the_cards(
            BusScan::NoRoute,
            DriverAnswer::Answered(vec![a_card("01"), a_card("02")]),
            MachineKind::AContainer,
        );
        assert_eq!(card_count(&honest), Some(2));
    }

    #[test]
    fn a_container_never_claims_no_card_even_on_an_empty_scan() {
        // H12: on a container the scan is the host's unfiltered view.
        assert_eq!(
            read_the_cards(BusScan::Empty, DriverAnswer::Absent, MachineKind::AContainer),
            Cards::CouldNotBeEstablished
        );
        assert_eq!(
            read_the_cards(
                BusScan::Empty,
                DriverAnswer::Absent,
                MachineKind::NotAContainer
            ),
            Cards::NoCard
        );
    }

    #[test]
    fn the_cgroups_silence_never_substitutes_the_proc_figure() {
        // H9a's third row. The `/proc` figure on a slice is the host's, and
        // reaching it because the cgroup was silent is the eightfold
        // overstatement arriving by a route H9 did not cover.
        assert_eq!(
            read_the_machines_memory(
                WhereK1PlacesIt::AContainer,
                CgroupAnswer::NoRoute,
                CgroupAnswer::NoRoute,
                Some(68_719_476_736)
            ),
            MachineMemory::CouldNotBeEstablished
        );
        // **And a machine K1 does not place is not taken to run its own
        // kernel**: the absence stands there too.
        assert_eq!(
            read_the_machines_memory(
                WhereK1PlacesIt::NowhereItPlaces,
                CgroupAnswer::NoRoute,
                CgroupAnswer::NoRoute,
                Some(68_719_476_736)
            ),
            MachineMemory::CouldNotBeEstablished
        );
    }

    /// **The ruled case, 2026-09-21: a machine with its own kernel and no
    /// control-group route is not capped, so its own figure is its memory.** On
    /// a plain virtual machine neither file exists at the root of the tree, and
    /// the row written for a container slice said the memory could not be read.
    #[test]
    fn a_machine_with_its_own_kernel_and_no_cgroup_route_is_its_own_figure() {
        assert_eq!(
            read_the_machines_memory(
                WhereK1PlacesIt::ItsOwnKernel,
                CgroupAnswer::NoRoute,
                CgroupAnswer::NoRoute,
                Some(68_719_476_736)
            ),
            MachineMemory::TheWholeMachine(68_719_476_736)
        );
        // **A figure that did not come back is still no figure**, on this kind
        // as on every other.
        assert_eq!(
            read_the_machines_memory(
                WhereK1PlacesIt::ItsOwnKernel,
                CgroupAnswer::NoRoute,
                CgroupAnswer::NoRoute,
                None
            ),
            MachineMemory::CouldNotBeEstablished
        );
    }

    /// **The kind reaches the absence row and nothing else.** A real cap on a
    /// machine with its own kernel is still the allocation, and the literal
    /// `max` on a container is still the whole machine.
    #[test]
    fn a_real_cgroup_figure_is_the_allocation_whatever_the_kind() {
        for placed in [
            WhereK1PlacesIt::AContainer,
            WhereK1PlacesIt::ItsOwnKernel,
            WhereK1PlacesIt::NowhereItPlaces,
        ] {
            assert_eq!(
                read_the_machines_memory(
                    placed,
                    CgroupAnswer::NoRoute,
                    CgroupAnswer::AFigure(16_051_527_680),
                    Some(68_719_476_736)
                ),
                MachineMemory::TheAllocation(16_051_527_680)
            );
            assert_eq!(
                read_the_machines_memory(
                    placed,
                    CgroupAnswer::TheLiteralMax,
                    CgroupAnswer::NoRoute,
                    Some(68_719_476_736)
                ),
                MachineMemory::TheWholeMachine(68_719_476_736)
            );
        }
    }

    #[test]
    fn k1s_table_places_the_two_readings() {
        assert_eq!(
            WhereK1PlacesIt::from_the_two_readings(MachineKind::AContainer, true),
            WhereK1PlacesIt::AContainer
        );
        assert_eq!(
            WhereK1PlacesIt::from_the_two_readings(MachineKind::AContainer, false),
            WhereK1PlacesIt::AContainer
        );
        assert_eq!(
            WhereK1PlacesIt::from_the_two_readings(MachineKind::NotAContainer, true),
            WhereK1PlacesIt::ItsOwnKernel
        );
        assert_eq!(
            WhereK1PlacesIt::from_the_two_readings(MachineKind::NotAContainer, false),
            WhereK1PlacesIt::NowhereItPlaces
        );
    }

    #[test]
    fn the_newer_paths_are_tried_and_the_older_are_not_assumed() {
        assert_eq!(
            read_the_machines_memory(
                WhereK1PlacesIt::AContainer,
                CgroupAnswer::AFigure(21_474_836_480),
                CgroupAnswer::NoRoute,
                Some(68_719_476_736)
            ),
            MachineMemory::TheAllocation(21_474_836_480)
        );
        assert_eq!(
            read_the_machines_memory(
                WhereK1PlacesIt::AContainer,
                CgroupAnswer::NoRoute,
                CgroupAnswer::AFigure(16_051_527_680),
                Some(68_719_476_736)
            ),
            MachineMemory::TheAllocation(16_051_527_680)
        );
    }

    #[test]
    fn the_literal_max_is_the_only_trigger_for_the_proc_figure() {
        assert_eq!(
            read_the_machines_memory(
                WhereK1PlacesIt::AContainer,
                CgroupAnswer::TheLiteralMax,
                CgroupAnswer::NoRoute,
                Some(68_719_476_736)
            ),
            MachineMemory::TheWholeMachine(68_719_476_736)
        );
    }

    #[test]
    fn an_uncapped_machine_with_no_figure_could_not_be_established_and_is_never_a_zero() {
        // H9a's third row, reached by the other route. **The figure answers *a
        // figure* or *nothing came back***, and a zero passed through as a fact
        // would show the user a machine with no memory at all.
        assert_eq!(
            read_the_machines_memory(
                WhereK1PlacesIt::AContainer,
                CgroupAnswer::TheLiteralMax,
                CgroupAnswer::NoRoute,
                None,
            ),
            MachineMemory::CouldNotBeEstablished
        );
        assert_eq!(
            read_the_machines_memory(
                WhereK1PlacesIt::AContainer,
                CgroupAnswer::NoRoute,
                CgroupAnswer::TheLiteralMax,
                None,
            ),
            MachineMemory::CouldNotBeEstablished
        );
        // A machine that really has no memory is not a case, and a figure of
        // zero is never invented for one: the cap's own figure still stands.
        assert_eq!(
            read_the_machines_memory(
                WhereK1PlacesIt::AContainer,
                CgroupAnswer::AFigure(0),
                CgroupAnswer::NoRoute,
                None,
            ),
            MachineMemory::TheAllocation(0)
        );
    }

    #[test]
    fn live_is_keyed_on_the_connection_and_not_on_the_period() {
        let held = a_reading();
        // A period standing over a declared break: the reading is last seen.
        assert_eq!(
            hardware_state(
                Some(&held),
                WhichConnectionHoldsIt {
                    taken_on_the_connection_that_is_live_now: false
                },
                InstallationCheck::PresentAndEqualConfirmed
            ),
            HardwareState::LastSeen
        );
        assert_eq!(
            hardware_state(
                Some(&held),
                WhichConnectionHoldsIt {
                    taken_on_the_connection_that_is_live_now: true
                },
                InstallationCheck::PresentAndEqualConfirmed
            ),
            HardwareState::Live
        );
    }

    #[test]
    fn a_disqualified_reading_returns_the_row_to_never_read() {
        let held = a_reading();
        for check in [
            InstallationCheck::Absent,
            InstallationCheck::PresentAndDifferent,
        ] {
            assert_eq!(
                hardware_state(
                    Some(&held),
                    WhichConnectionHoldsIt {
                        taken_on_the_connection_that_is_live_now: true
                    },
                    check
                ),
                HardwareState::NeverRead
            );
        }
    }

    #[test]
    fn the_fourth_branch_discards_nothing_and_takes_no_fresh_reading() {
        let branch = what_the_branch_does(InstallationCheck::PresentAndTheSupervisorDidNotAnswer);
        assert_eq!(branch.the_held_reading, MachineDerivedValue::StandsUnchanged);
        assert!(!branch.a_fresh_reading_is_taken);
    }

    #[test]
    fn a_fresh_reading_is_taken_on_the_other_three_branches() {
        for check in [
            InstallationCheck::PresentAndEqualConfirmed,
            InstallationCheck::Absent,
            InstallationCheck::PresentAndDifferent,
        ] {
            assert!(what_the_branch_does(check).a_fresh_reading_is_taken);
        }
    }

    #[test]
    fn the_two_moments_are_never_subtracted_from_each_other() {
        // `State, identity and recovery` D5 kept by the spine's own type: two
        // clocks give no duration.
        assert_eq!(a_reading().how_long_between_the_two_moments(), None);
    }

    #[test]
    fn a_reading_with_a_stated_absence_is_complete_and_not_failed() {
        // H8: cards and no per-card memory.
        let mut card = a_card("01");
        card.total_memory_bytes = None;
        let cards = Cards::TheCards(vec![card]);
        assert_eq!(card_count(&cards), Some(1));
    }

    #[test]
    fn the_free_space_reading_names_the_path_it_was_taken_against() {
        let held = a_reading();
        assert_eq!(
            held.free_space
                .as_ref()
                .expect("the reading in this test carries one")
                .the_path_it_was_taken_against,
            "/opt/smoozy/jobs"
        );
    }

    #[test]
    fn no_blender_is_in_the_reading() {
        assert!(!a_blender_is_asked_for_the_hardware_reading());
    }

    #[test]
    fn four_card_routes_are_forbidden_outright() {
        assert_eq!(ForbiddenCardRoute::ALL.len(), 4);
    }

    /// **H7: the make is the key, and the driver's own name for the card is
    /// beside it and never in it.** Step 10c's sixth pass on Vast: the make
    /// was *NVIDIA GeForce GTX 1650*, the table was keyed *NVIDIA*, and every
    /// cards job on every NVIDIA machine ended before the render.
    #[test]
    fn a_cards_make_is_the_key_and_never_the_name_the_driver_gives_it() {
        let make = CardMake::from_the_drivers_own_query("NVIDIA GeForce GTX 1650");
        assert_eq!(make.as_key(), NVIDIA);
        assert_eq!(make.the_drivers_name_for_the_card(), "NVIDIA GeForce GTX 1650");
        // **Nothing is read out of the text**: a name that does not begin with
        // the maker's own word is still that maker's card, because the route is.
        let bare = CardMake::from_the_drivers_own_query("Tesla T4");
        assert_eq!(bare.as_key(), NVIDIA);
        // Two models of one make are one key.
        assert_eq!(
            make.as_key(),
            CardMake::from_the_drivers_own_query("NVIDIA RTX A6000").as_key()
        );
    }

    // ---- `Rendering` L23a and `Servers` H11b: the OptiX driver bar ----

    fn the_bar_for(blender: BlenderNumbers) -> Vec<u32> {
        the_optix_bar_row(blender)
            .expect("every Blender from 4.5 on has a bar")
            .driver_minimum
            .to_vec()
    }

    /// **L23a's three rows and its last-row rule**, one version at each edge.
    #[test]
    fn each_blender_version_is_read_against_its_own_row_of_the_bar() {
        assert_eq!(the_bar_for((4, 5, 0)), vec![535], "4.5: the first row");
        assert_eq!(the_bar_for((5, 1, 2)), vec![535], "5.1: still the first row");
        assert_eq!(the_bar_for((5, 2, 0)), vec![570], "5.2.0: its own row");
        assert_eq!(the_bar_for((5, 2, 1)), vec![575], "5.2.1: the last row");
        assert_eq!(the_bar_for((5, 2, 2)), vec![575], "5.2.2: and later");
        assert_eq!(
            the_bar_for((6, 1, 0)),
            vec![575],
            "a version past the last row is read against the last row"
        );
        assert_eq!(the_optix_bar_row((4, 4, 0)), None, "no row below 4.5 is made up");
    }

    /// **Number by number, never as text.** As text, `1000` sorts below `575`
    /// and `575.10` below `575.9`.
    #[test]
    fn the_drivers_version_is_compared_number_by_number() {
        let v = |t: &str| DriverVersion::parse(t).expect("a version");
        assert!(v("550.163.01").is_below(&[575]));
        assert!(!v("575").is_below(&[575]), "at the bar is not below it");
        assert!(!v("575.00").is_below(&[575]), "a missing number is zero");
        assert!(!v("1000.1").is_below(&[575]), "never compared as text");
        assert!(v("575.9").is_below(&[575, 10]));
        assert_eq!(v("550.163.01").as_the_driver_wrote_it(), "550.163.01");
        assert_eq!(DriverVersion::parse("[N/A]"), None);
        assert_eq!(DriverVersion::parse(""), None);
        assert_eq!(DriverVersion::parse("550..1"), None);
    }

    fn a_driver(version: &str, placed: WhereK1PlacesIt) -> TheDriversOwnAnswer {
        TheDriversOwnAnswer {
            version: DriverVersion::parse(version).expect("a version"),
            cuda_version: Some("12.4".into()),
            where_k1_placed_the_machine: placed,
        }
    }

    const ON_THE_SERVER: [BlenderNumbers; 3] = [(4, 5, 3), (5, 1, 2), (5, 2, 2)];

    /// **H11b: below the bar on a container, the statement is owed**, naming
    /// the earliest Blender version whose bar the driver misses.
    #[test]
    fn a_container_below_the_bar_is_told() {
        let said = what_the_bar_says(
            &a_driver("550.163.01", WhereK1PlacesIt::AContainer),
            &ON_THE_SERVER,
            false,
        );
        assert_eq!(
            said.the_statement,
            Some(TheDriverStatement {
                the_drivers_version: "550.163.01".into(),
                cuda_version: Some("12.4".into()),
                from_blender: (5, 2, 0),
            })
        );
        assert_eq!(said.blenders_below_their_bar, vec![(5, 2, 2)]);
    }

    /// **At the bar, nothing is said and no Blender is below it.**
    #[test]
    fn a_container_at_the_bar_is_told_nothing() {
        let said = what_the_bar_says(
            &a_driver("575.00", WhereK1PlacesIt::AContainer),
            &ON_THE_SERVER,
            false,
        );
        assert_eq!(said.the_statement, None);
        assert!(said.blenders_below_their_bar.is_empty());
    }

    /// **On a machine Smoozy can manage, nothing is said** — setup brings the
    /// driver up silently — **but the Blender table's note still stands.**
    #[test]
    fn a_systemd_machine_below_the_bar_is_told_nothing_but_its_blenders_are_noted() {
        let said = what_the_bar_says(
            &a_driver("550.163.01", WhereK1PlacesIt::ItsOwnKernel),
            &ON_THE_SERVER,
            false,
        );
        assert_eq!(said.the_statement, None);
        assert_eq!(said.blenders_below_their_bar, vec![(5, 2, 2)]);
    }

    /// **A driver below every row names the first row's Blender, and every
    /// Blender on the server is noted.**
    #[test]
    fn a_driver_below_every_bar_names_the_first_rows_blender() {
        let said = what_the_bar_says(
            &a_driver("525.60", WhereK1PlacesIt::AContainer),
            &ON_THE_SERVER,
            false,
        );
        assert_eq!(said.the_statement.map(|s| s.from_blender), Some((4, 5, 0)));
        assert_eq!(said.blenders_below_their_bar, ON_THE_SERVER.to_vec());
    }

    /// **H11b's second case, joined 2026-10-11 (`Server Setup` R1 item 5): a
    /// machine Smoozy manages whose setup could not bring the driver up to the
    /// bar is told as a container is.** At the bar, nothing is said whatever
    /// the setup came to; and a machine K1 does not place is never told.
    #[test]
    fn a_machine_whose_setup_missed_the_bar_is_told() {
        let said = what_the_bar_says(
            &a_driver("550.163.01", WhereK1PlacesIt::ItsOwnKernel),
            &ON_THE_SERVER,
            true,
        );
        assert_eq!(said.the_statement.map(|s| s.from_blender), Some((5, 2, 0)));
        assert_eq!(said.blenders_below_their_bar, vec![(5, 2, 2)]);
        let at_the_bar = what_the_bar_says(
            &a_driver("580.178.04", WhereK1PlacesIt::ItsOwnKernel),
            &ON_THE_SERVER,
            true,
        );
        assert_eq!(at_the_bar.the_statement, None);
        assert!(!the_statement_is_said_on(WhereK1PlacesIt::NowhereItPlaces, true));
        assert!(the_statement_is_said_on(WhereK1PlacesIt::AContainer, false));
        assert!(!the_statement_is_said_on(WhereK1PlacesIt::ItsOwnKernel, false));
    }
}
