//! The machine's own readings, taken — and V4's asking, which is not a reading
//! of the machine at all.
//!
//! **[`crate::health_readings`] holds the rules** — when a reading may be taken,
//! which mechanism the kind chooses, the three routes a container forbids, and
//! the two negatives behind the runaway rental. This module is the askings.
//!
//! # V4, and why it is an asking rather than a reading
//!
//! **V4's observed value is one thing: whether the supervisor answered a request
//! on the command path.** V4 is exact that it is **not** a reading of a process,
//! **not** a reading of a file and **not** a reading of the connection. **So it
//! is a framed request**, and the framing is
//! `Server protocol and transfers` §2's.
//!
//! **The connect probe this crate already has does not stand in for it, and that
//! refusal stands.** Z12's probe asks whether anything holds the endpoint, which
//! is a reading of the connection — the one thing V4 says the observed value is
//! not. Reading it as V4's value would make **a supervisor that holds its
//! endpoint and answers nothing look healthy**, which is the whole condition
//! `Desktop app and delivery`'s automatic repair trigger exists for. **So nothing
//! below touches the probe.** What [`TheMachinesOwnReadingsAndTheAsking`] does is
//! send one framed request over the endpoint and answer on whether a correlated
//! result came back.
//!
//! **Which request it sends is `Server protocol and transfers` §1.1's readiness**,
//! named in that clause's own list of what the entry point carries. **Nothing
//! about the machine is read out of the answer**: V4's value is that an answer
//! came, and the answer's fields are not looked at.
//!
//! **This is the machine-side leg of the command path.** A desktop request
//! reaches the machine over SSH, the entry point's adapter passes it to the
//! supervisor over the machine-local endpoint, and the result comes back; what
//! is asked here is the second half of that, on the machine, with no SSH under
//! it. `Servers` supplies the connection's own condition and **the two are never
//! inferred from one another.**
//!
//! # The card reading
//!
//! **V6's trap is the whole reason the shape is one line per card.** *"A
//! per-card query returns the value once per card, so a caller parsing the whole
//! output as an integer gets `22` on a two-card machine. It passes every test
//! ever written on a one-card machine."* The register's own conclusion beside
//! it: safe readings are the driver's own list, or **counting rows of any
//! per-card query**.
//!
//! **The command is the driver's own, and both halves of it are published.**
//! NVIDIA's `nvidia-smi` documentation, read 2026-09-04 at
//! `https://docs.nvidia.com/deploy/nvidia-smi/index.html`: of `--query-gpu`,
//! *"Information about GPU. Pass comma separated list of properties you want to
//! query."*; of the format, *"csv - comma separated values (MANDATORY);
//! noheader - skip first line with column headers"*. **The header is skipped at
//! the tool rather than dropped here**, because a reader that dropped a first
//! line would drop a card on a tool that stopped printing one.
//!
//! **What a two-card slice of a four-card host answers is the reason no other
//! route is taken.** Observed on two such hosts: the driver's own query returned
//! **2** — ours — while the kernel's per-card directory returned **4**, two of
//! the identifiers belonging to another tenant. **A count taken below the
//! driver's own query counts the host's cards, not the rental's, and the error
//! is quiet.**
//!
//! **What an empty answer means is not decided here.** `Servers` H12 owns it and
//! rules that on a container the hardware reading never claims *no card* and
//! says instead that the cards could not be established; Fact 46's cardless case
//! is permanently unresolved and no reading will settle it. **This module hands
//! back the rows and claims nothing about none of them.**
//!
//! # The running time
//!
//! `proc_uptime(5)`, read 2026-09-04 at
//! `https://man7.org/linux/man-pages/man5/proc_uptime.5.html`: *"This file
//! contains two numbers (values in seconds): the uptime of the system (including
//! time spent in suspend) and the amount of time spent in the idle process."*
//!
//! **On a container it is not taken at all**, and that is V13's own ruled
//! conduct rather than a gap: the figure there is the **host's** and would lie —
//! 8.2 days reported on a container created that morning — so the line says the
//! reading is unavailable rather than showing the host's. **No second route to
//! the figure is opened**, which is what the plan's own Section 16 item 31
//! records: this project's record holds one observation that would give a
//! container's own start, taken in passing for a different question, and it is
//! deliberately not adopted.

use std::path::PathBuf;
use std::time::Duration;

use smoozy_framing::framing::{
    Request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, THE_READINESS_REQUEST,
};
use smoozy_state::identities::{Identity, IdentityKind};

use crate::acting::endpoint::TheEndpointAsACaller;
use crate::acting::machine::run;
use crate::health_readings::{
    the_running_time, SupervisorUnavailable, TakesHealthReadings, TheRunningTime,
};
use crate::machine_kind::MachineKind;

/// The driver's own per-card query, as the two published halves compose it.
pub const ASK_THE_DRIVER_FOR_ONE_LINE_PER_CARD: [&str; 2] =
    ["--query-gpu=index", "--format=csv,noheader"];

/// Where the kernel publishes the machine's own running time.
pub const WHERE_THE_RUNNING_TIME_IS_PUBLISHED: &str = "uptime";

/// **The driver's own per-card query for each card's memory, used and
/// whole** — the `Telegram` role worker's status reading (`Rented-server
/// runtime` V2's one named exception; `Telegram` C1). The query's two
/// published halves are the ones above. **The two properties and what the tool
/// prints for them are observed, not published here**: `The rented machine`
/// records the same query answering one line per card, `5989 MiB, 24564 MiB`,
/// under the header `memory.used [MiB], memory.total [MiB]`, on a rented
/// card. Added 2026-10-16, the Telegram worker's piece 6.
pub const ASK_THE_DRIVER_FOR_EACH_CARDS_MEMORY: [&str; 2] =
    ["--query-gpu=memory.used,memory.total", "--format=csv,noheader"];

/// **One card's memory, used and whole, in mebibytes**, as the driver's own
/// query prints it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OneCardsMemory {
    pub used_mib: f64,
    pub total_mib: f64,
}

/// **One line of [`ASK_THE_DRIVER_FOR_EACH_CARDS_MEMORY`]'s answer, read** —
/// two values, each a number and the unit the observed answer printed, `MiB`.
/// **A line in any other shape is nothing**, and never a nought: a reading
/// that cannot be read is said to be unavailable (`Telegram` C1).
pub fn one_cards_memory_in(line: &str) -> Option<OneCardsMemory> {
    let mut values = line.split(',').map(|value| {
        value
            .trim()
            .strip_suffix("MiB")
            .and_then(|number| number.trim().parse::<f64>().ok())
            .filter(|number| number.is_finite() && *number >= 0.0)
    });
    let (Some(Some(used_mib)), Some(Some(total_mib)), None) = (values.next(), values.next(), values.next()) else {
        return None;
    };
    Some(OneCardsMemory { used_mib, total_mib })
}

/// The machine's own readings that need no request framing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheMachinesOwnReadings {
    /// The kind K1's table placed this machine as, read at this placement.
    ///
    /// **V6: which mechanism a reading is taken by is chosen by the detected
    /// kind, and getting that wrong is silent.**
    pub kind: MachineKind,
    /// Where the kernel publishes its own figures. `/proc` on a rented machine.
    pub where_the_kernel_publishes: PathBuf,
    /// The driver's own tool, by name.
    pub the_card_tool: String,
    /// **How long the driver's own tool is given to answer, carried from the
    /// caller** (K17). A tool that never returns would otherwise hold the
    /// reading for ever.
    pub how_long_the_card_tool_is_given: Duration,
}

impl TheMachinesOwnReadings {
    /// The rented machine, of the kind this placement read.
    pub fn on(kind: MachineKind, how_long_the_card_tool_is_given: Duration) -> Self {
        TheMachinesOwnReadings {
            kind,
            where_the_kernel_publishes: PathBuf::from("/proc"),
            the_card_tool: "nvidia-smi".to_string(),
            how_long_the_card_tool_is_given,
        }
    }

    /// **V6.** One line per card, counted rather than parsed as a total.
    ///
    /// **A tool that is not there, or that refuses, is a reading that could not
    /// be taken and never an answer of no cards.** Fact 46's cardless case is
    /// permanently unresolved and no reading settles it, so a refusal that
    /// answered *none* would be manufacturing exactly the distinction the
    /// register says is not posed.
    pub fn one_line_per_card(&self) -> Result<Vec<String>, String> {
        let said = run(
            &self.the_card_tool,
            &ASK_THE_DRIVER_FOR_ONE_LINE_PER_CARD,
            &[],
            self.how_long_the_card_tool_is_given,
        )?;
        if !said.ended_at_zero {
            return Err(format!(
                "the driver's own query did not answer: {}",
                said.as_text()
            ));
        }
        Ok(said
            .ordinary
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(str::to_string)
            .collect())
    }

    /// **Each card's memory, one line per card** — V6's shape, counted as
    /// lines and never parsed as a total. **Taken for the `Telegram` role
    /// worker's status message alone** (V2's one named exception), on its own
    /// clock; nothing else asks for it. A tool that is not there, that
    /// refuses, or that prints a line this cannot read is a reading that
    /// could not be taken, never an answer of no memory. Added 2026-10-16, the
    /// Telegram worker's piece 6.
    pub fn each_cards_memory(&self) -> Result<Vec<OneCardsMemory>, String> {
        let said = run(
            &self.the_card_tool,
            &ASK_THE_DRIVER_FOR_EACH_CARDS_MEMORY,
            &[],
            self.how_long_the_card_tool_is_given,
        )?;
        if !said.ended_at_zero {
            return Err(format!("the driver's own query did not answer: {}", said.as_text()));
        }
        said.ordinary
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(|line| one_cards_memory_in(line).ok_or_else(|| format!("a card's line does not read: {line}")))
            .collect()
    }

    /// **V13's second half**, and it is conditional on the machine kind.
    ///
    /// `None` is the stated absence on the kind where the figure would not be
    /// the machine's own. **It is a stated absence and never a zero**, and
    /// nothing here carries a held figure forward as current.
    pub fn the_machines_own_running_time(&self) -> Result<Option<Duration>, String> {
        match the_running_time(self.kind) {
            TheRunningTime::Unavailable => Ok(None),
            TheRunningTime::TheMachinesOwn => {
                let at = self
                    .where_the_kernel_publishes
                    .join(WHERE_THE_RUNNING_TIME_IS_PUBLISHED);
                let published = std::fs::read_to_string(&at)
                    .map_err(|why| format!("{}: {why}", at.display()))?;
                the_seconds_in(&published)
                    .map(Some)
                    .ok_or_else(|| format!("{}: the figure does not read back", at.display()))
            }
        }
    }
}

/// **The whole of [`crate::health_readings::TakesHealthReadings`]: V4's asking,
/// V6's card reading and V13's running time.**
///
/// **V2 binds all three: every one of them is taken on request.** There is no
/// schedule here, no watcher and no subscription, and nothing is taken to keep a
/// message fresh.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheMachinesOwnReadingsAndTheAsking {
    /// V6's and V13's, which are the machine's own.
    pub readings: TheMachinesOwnReadings,
    /// V4's, which is not a reading of the machine but a request to its
    /// supervisor over the machine-local endpoint at Z2's recorded path.
    pub endpoint: TheEndpointAsACaller,
}

impl TheMachinesOwnReadingsAndTheAsking {
    pub fn on(
        kind: MachineKind,
        recorded_endpoint_path: &str,
        how_long_a_result_may_take: Duration,
        how_long_the_card_tool_is_given: Duration,
    ) -> Self {
        TheMachinesOwnReadingsAndTheAsking {
            readings: TheMachinesOwnReadings::on(kind, how_long_the_card_tool_is_given),
            endpoint: TheEndpointAsACaller::at(recorded_endpoint_path, how_long_a_result_may_take),
        }
    }
}

impl TakesHealthReadings for TheMachinesOwnReadingsAndTheAsking {
    /// **V4.** One framed request, and the value is whether a correlated result
    /// came back.
    ///
    /// **Nothing in the result is read.** V4's value is one thing — that the
    /// supervisor answered — and reading a field out of the answer would make it
    /// two things, the second of which V5 would then have to carry a moment for.
    ///
    /// **`Err` is not *it did not answer*.** A system that would not let this
    /// process reach the path at all is a refusal this reading met rather than a
    /// verdict on the supervisor, and V4's value is never composed out of one.
    fn supervisor_answered(&self) -> Result<SupervisorUnavailable, String> {
        // `Server protocol and transfers` C1: chosen by the initiating side
        // before the request is sent. This is a pure reading, so
        // `Server protocol and transfers` C6 gives it an identity for
        // correlation and reporting rather than for idempotency.
        let identity = Identity::issue(IdentityKind::Operation)
            .map_err(|_| "the system would not supply random bytes".to_string())?;
        let request = Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            identity,
            THE_READINESS_REQUEST,
        );
        match self.endpoint.ask(&request)? {
            Some(_) => Ok(SupervisorUnavailable::ItAnsweredARequestOnTheCommandPath),
            None => Ok(SupervisorUnavailable::ItDidNotAnswer),
        }
    }

    /// **V6.** One line per card, counted rather than parsed as a total.
    fn one_line_per_card(&self) -> Result<Vec<String>, String> {
        self.readings.one_line_per_card()
    }

    /// **V13's second half.** `None` is the stated absence on the kind where the
    /// figure would not be the machine's own.
    fn the_machines_own_running_time(&self) -> Result<Option<Duration>, String> {
        self.readings.the_machines_own_running_time()
    }
}

/// The first of the kernel's two numbers, in seconds.
///
/// **The second is the idle figure and nothing here reads it.**
pub fn the_seconds_in(published: &str) -> Option<Duration> {
    let seconds: f64 = published.split_whitespace().next()?.parse().ok()?;
    if !seconds.is_finite() || seconds < 0.0 {
        return None;
    }
    Some(Duration::from_secs_f64(seconds))
}

/// **§12's own request and the three names its answer carries, by name.**
///
/// **V2 is what makes it a request and not a subscription**: every health
/// reading is taken on request and never on a schedule, with one named
/// beneficiary that is not this — the `Telegram` role worker's status reading,
/// taken on its own machine about its own machine and never sent to the desktop.
///
/// **The names are Build's own and the plan fixes none of them**, exactly as it
/// fixes none of this plan's record names in the package area. What the plan
/// fixes is the readings, which are V6's and V13's.
///
/// **They are declared in `smoozy-framing` and re-exported here at the paths
/// they have always had. Moved 2026-09-21.** `Servers` W41a makes the desktop
/// the caller — one request per primary connection, at its first-connection
/// step 7 — and this crate's acting half is built for the machine alone, so a
/// desktop that must build for Windows could never name what it has to send.
/// **`Desktop app and delivery`'s own step-7 file had recorded exactly that as
/// the reason `Servers` W41a's request is not made.** It is the same move and the same
/// reason as [`crate::acting::stores::THE_SETUP_IN_PROGRESS_IS_SET_REQUEST`]'s,
/// and nothing on this side moved with it.
///
/// **V9's field is absent where no setup is in progress, V13's where the
/// reading would not be the machine's own**, and the reasons are written at the
/// constants themselves.
pub use smoozy_framing::requests::{
    THE_CARD_COUNT_FIELD, THE_HEALTH_READINGS_REQUEST, THE_RUNNING_TIME_FIELD,
    THE_SETUP_IN_PROGRESS_FIELD,
};

/// **§12's readings, put into one framed result** — V6's count and V13's
/// running time, and nothing else.
///
/// **V5: every health reading carries the moment it was taken and the clock it
/// was taken on, and nothing durable is written from one.** The moment is the
/// result's own arrival at a caller that asked for it: **nothing here writes
/// one down**, and no clause asks this answer to carry a moment the caller did
/// not itself produce by asking.
pub fn the_health_readings_as_fields(
    how_many_cards: usize,
    the_machines_own_running_time: Option<Duration>,
) -> smoozy_framing::framing::Fields {
    let fields = smoozy_framing::framing::Fields::none()
        .and(THE_CARD_COUNT_FIELD, how_many_cards.to_string());
    match the_machines_own_running_time {
        Some(running) => fields.and(THE_RUNNING_TIME_FIELD, running.as_secs().to_string()),
        None => fields,
    }
}

/// **V9's field, added to a result that already carries something.**
///
/// It is written this way rather than as a fifth thing inside G12's answer
/// because **G12 asks for four things and takes nothing else as evidence**: the
/// four are that clause's and this is §12's, riding beside them. `Server
/// protocol and transfers` P5 is what makes that safe in both directions — a
/// reader that does not know a field ignores it rather than refusing the result.
pub fn also_carrying_the_setup_state(
    fields: smoozy_framing::framing::Fields,
    standing: Option<&crate::setup_state::SetupInProgressState>,
) -> smoozy_framing::framing::Fields {
    match standing {
        Some(state) => fields.and(
            THE_SETUP_IN_PROGRESS_FIELD,
            state.the_run_that_set_it.as_text(),
        ),
        None => fields,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::health_readings::how_many_cards;

    fn an_area() -> PathBuf {
        let area = std::env::temp_dir().join(format!(
            "smoozy-runtime-readings-{}-{}",
            std::process::id(),
            {
                static NEXT: std::sync::atomic::AtomicUsize =
                    std::sync::atomic::AtomicUsize::new(0);
                NEXT.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
            }
        ));
        std::fs::create_dir_all(&area).expect("the area is made");
        area
    }

    /// A driver's tool that answers as the real one does.
    fn a_driver_that_answers(at: &std::path::Path, prints: &str, ends_at: i32) {
        use std::io::Write;
        use std::os::unix::fs::PermissionsExt;
        let mut file = std::fs::File::create(at).expect("the stand-in is written");
        write!(
            file,
            "#!/bin/sh\ncat <<'THEANSWER'\n{prints}\nTHEANSWER\nexit {ends_at}\n"
        )
        .expect("the stand-in is written");
        drop(file);
        let mut permission = std::fs::metadata(at).unwrap().permissions();
        permission.set_mode(0o700);
        std::fs::set_permissions(at, permission).expect("the stand-in may be run");
    }

    fn readings(area: &std::path::Path, kind: MachineKind) -> TheMachinesOwnReadings {
        TheMachinesOwnReadings {
            kind,
            where_the_kernel_publishes: area.to_path_buf(),
            the_card_tool: area.join("the driver").display().to_string(),
            how_long_the_card_tool_is_given: Duration::from_secs(30),
        }
    }

    /// **Each card's memory, one line per card, read as the observed answer
    /// prints it** — the `Telegram` status reading. Two cards give two lines
    /// and never one total; a line in another shape, or a tool that refuses,
    /// is a reading that could not be taken.
    #[test]
    fn each_cards_memory_is_one_line_per_card_and_nothing_else_reads() {
        let area = an_area();
        let reading = readings(&area, MachineKind::Container);
        a_driver_that_answers(&area.join("the driver"), "5989 MiB, 24564 MiB\n1 MiB, 24564 MiB", 0);
        assert_eq!(
            reading.each_cards_memory().expect("it reads"),
            vec![
                OneCardsMemory { used_mib: 5989.0, total_mib: 24564.0 },
                OneCardsMemory { used_mib: 1.0, total_mib: 24564.0 }
            ]
        );
        a_driver_that_answers(&area.join("the driver"), "5989, 24564", 0);
        assert!(reading.each_cards_memory().is_err(), "a line with no unit is not read as one");
        a_driver_that_answers(&area.join("the driver"), "NVIDIA-SMI has failed", 9);
        assert!(reading.each_cards_memory().is_err(), "a refusal is never no memory");
        assert_eq!(one_cards_memory_in("5989 MiB, 24564 MiB, 3 MiB"), None);
        std::fs::remove_dir_all(&area).ok();
    }

    /// **The trap, run through the real reading.** A two-card machine answers
    /// two rows, and the count is the number of rows.
    /// **V4, run over a real endpoint.** The supervisor answered a request, so
    /// the observed value is that it answered.
    #[test]
    fn the_supervisor_answering_a_framed_request_is_what_v4_reads() {
        use crate::acting::endpoint::as_a_supervisor;
        use smoozy_framing::framing::Answered;
        let path = as_a_supervisor::a_path();
        let supervisor = as_a_supervisor::answering(&path, 1, |request| {
            assert_eq!(request.name, THE_READINESS_REQUEST);
            Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "2026.09.04")
        });
        let asking = TheMachinesOwnReadingsAndTheAsking::on(
            MachineKind::Systemd,
            &path,
            Duration::from_millis(2000),
            Duration::from_secs(30),
        );
        assert_eq!(
            asking.supervisor_answered().unwrap(),
            SupervisorUnavailable::ItAnsweredARequestOnTheCommandPath
        );
        supervisor.join().unwrap();
        std::fs::remove_file(&path).ok();
    }

    /// **The refusal that must not be undone, made a test.**
    ///
    /// A supervisor **holding its endpoint and answering nothing** is exactly the
    /// machine `Desktop app and delivery`'s automatic repair trigger exists for.
    /// **Z12's connect probe reads it as accepted** — it is a reading of the
    /// connection — and V4 reads it as *did not answer*, because V4's value is
    /// whether a request was answered. **The two answers disagree here and that
    /// disagreement is the clause**: had the probe been taken as V4's value, this
    /// machine would have read as healthy.
    #[test]
    fn a_supervisor_holding_its_endpoint_and_answering_nothing_has_not_answered() {
        use crate::acting::endpoint::{as_a_supervisor, TheEndpointOnTheMachine};
        use crate::endpoint::{BindAttempt, ConnectProbe, DrivesTheEndpoint};
        let path = as_a_supervisor::a_path();
        let holder = TheEndpointOnTheMachine::at(&path);
        assert_eq!(holder.bind_at(&path).unwrap(), BindAttempt::Bound);
        // It holds the endpoint and never accepts a caller.

        let asking = TheMachinesOwnReadingsAndTheAsking::on(
            MachineKind::Systemd,
            &path,
            Duration::from_millis(150),
            Duration::from_secs(30),
        );
        // **Bounded, so that a wait with no bound fails rather than hangs.**
        let answered =
            as_a_supervisor::within(Duration::from_secs(3), move || asking.supervisor_answered());
        assert_eq!(answered.unwrap(), SupervisorUnavailable::ItDidNotAnswer);
        assert_eq!(
            holder.probe_as_a_caller(&path).unwrap(),
            ConnectProbe::Accepted,
            "the probe would have called this machine healthy"
        );
        holder.remove_its_own_object().unwrap();
    }

    /// **Nothing holding the endpoint is *it did not answer*** — and it is not a
    /// reading of a file: no object is looked for and none is read.
    #[test]
    fn nothing_holding_the_endpoint_is_that_the_supervisor_did_not_answer() {
        use crate::acting::endpoint::as_a_supervisor;
        let path = as_a_supervisor::a_path();
        let asking = TheMachinesOwnReadingsAndTheAsking::on(
            MachineKind::Systemd,
            &path,
            Duration::from_millis(150),
            Duration::from_secs(30),
        );
        let answered =
            as_a_supervisor::within(Duration::from_secs(3), move || asking.supervisor_answered());
        assert_eq!(answered.unwrap(), SupervisorUnavailable::ItDidNotAnswer);
    }

    /// **The whole trait through one value**: V4's asking beside V6's card
    /// reading and V13's running time, which is what the trait is.
    #[test]
    fn the_two_machine_readings_are_the_same_two_through_the_whole_trait() {
        use crate::acting::endpoint::as_a_supervisor;
        let area = an_area();
        a_driver_that_answers(&area.join("the driver"), "0\n1", 0);
        std::fs::write(area.join(WHERE_THE_RUNNING_TIME_IS_PUBLISHED), "10.5 1.0\n").unwrap();
        let asking = TheMachinesOwnReadingsAndTheAsking {
            readings: readings(&area, MachineKind::Systemd),
            endpoint: TheEndpointAsACaller::at(
                &as_a_supervisor::a_path(),
                Duration::from_millis(50),
            ),
        };
        assert_eq!(asking.one_line_per_card().unwrap().len(), 2);
        assert_eq!(
            asking.the_machines_own_running_time().unwrap(),
            Some(Duration::from_secs_f64(10.5))
        );
        std::fs::remove_dir_all(&area).ok();
    }

    #[test]
    fn a_two_card_machine_answers_two_rows_and_the_count_is_two() {
        let area = an_area();
        a_driver_that_answers(&area.join("the driver"), "0\n1", 0);
        let rows = readings(&area, MachineKind::Systemd).one_line_per_card().unwrap();
        assert_eq!(rows, vec!["0", "1"]);
        let rows: Vec<&str> = rows.iter().map(String::as_str).collect();
        assert_eq!(how_many_cards(&rows), 2);
        std::fs::remove_dir_all(&area).ok();
    }

    /// **The trap itself, named so nobody re-introduces it.** The count query
    /// answers the value once per card, and a caller parsing the whole output
    /// as one number reads `22` on a two-card machine. **The reading here
    /// counts rows**, so the same output reads as two.
    #[test]
    fn a_count_query_would_have_read_twenty_two_and_this_reading_reads_two() {
        let area = an_area();
        a_driver_that_answers(&area.join("the driver"), "2\n2", 0);
        let rows = readings(&area, MachineKind::Systemd).one_line_per_card().unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(
            rows.concat().parse::<u32>().unwrap(),
            22,
            "this is what a caller parsing the whole output would have got"
        );
        std::fs::remove_dir_all(&area).ok();
    }

    #[test]
    fn a_one_card_machine_answers_one_row() {
        let area = an_area();
        a_driver_that_answers(&area.join("the driver"), "0", 0);
        assert_eq!(
            readings(&area, MachineKind::Systemd).one_line_per_card().unwrap().len(),
            1
        );
        std::fs::remove_dir_all(&area).ok();
    }

    /// **A tool that is not there is a reading that could not be taken**, and
    /// never an answer of no cards.
    #[test]
    fn a_driver_that_is_not_there_is_a_refusal_and_never_an_answer_of_none() {
        let area = an_area();
        let why = readings(&area, MachineKind::Systemd)
            .one_line_per_card()
            .expect_err("there is no such tool");
        assert!(why.contains("could not be run"));
        std::fs::remove_dir_all(&area).ok();
    }

    #[test]
    fn a_driver_that_refuses_is_a_refusal_and_never_an_answer_of_none() {
        let area = an_area();
        a_driver_that_answers(&area.join("the driver"), "", 9);
        let why = readings(&area, MachineKind::Systemd)
            .one_line_per_card()
            .expect_err("it refused");
        assert!(why.contains("did not answer"));
        std::fs::remove_dir_all(&area).ok();
    }

    #[test]
    fn the_running_time_on_a_systemd_machine_is_the_machines_own() {
        let area = an_area();
        std::fs::write(area.join(WHERE_THE_RUNNING_TIME_IS_PUBLISHED), "350735.47 234388.90\n")
            .unwrap();
        let taken = readings(&area, MachineKind::Systemd)
            .the_machines_own_running_time()
            .unwrap()
            .expect("it is the machine's own");
        assert_eq!(taken.as_secs(), 350_735);
        std::fs::remove_dir_all(&area).ok();
    }

    /// **On a container the figure is the host's and would lie, so it is not
    /// taken at all** — the stated absence, never a zero.
    #[test]
    fn the_running_time_on_a_container_is_not_taken_and_is_a_stated_absence() {
        let area = an_area();
        // The kernel publishes a figure. It is the host's, and it is 8.2 days
        // on a container created that morning.
        std::fs::write(
            area.join(WHERE_THE_RUNNING_TIME_IS_PUBLISHED),
            "708480.00 1.00\n",
        )
        .unwrap();
        assert_eq!(
            readings(&area, MachineKind::Container)
                .the_machines_own_running_time()
                .unwrap(),
            None
        );
        std::fs::remove_dir_all(&area).ok();
    }

    #[test]
    fn a_figure_that_does_not_read_back_is_said_rather_than_read_as_a_zero() {
        let area = an_area();
        std::fs::write(area.join(WHERE_THE_RUNNING_TIME_IS_PUBLISHED), "not a number\n").unwrap();
        let why = readings(&area, MachineKind::Systemd)
            .the_machines_own_running_time()
            .expect_err("it does not read back");
        assert!(why.contains("does not read back"));
        std::fs::remove_dir_all(&area).ok();
    }

    #[test]
    fn the_second_number_is_the_idle_figure_and_is_not_read() {
        assert_eq!(the_seconds_in("10.5 99999.0"), Some(Duration::from_secs_f64(10.5)));
        assert_eq!(the_seconds_in(""), None);
        assert_eq!(the_seconds_in("-1 2"), None);
    }
}
