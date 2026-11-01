//! **§7 G1 to G5a performed — the card reading through the driver's own
//! per-card query and the disk figure at the root's mount, taken at the two
//! requests and carried on the health-readings route.**
//!
//! # Nothing of this crate's judgements turns on either figure
//!
//! **They are facts on the entry and neither may be narrowed to the render.**
//! No function in this crate reads a card figure or a disk figure to decide a
//! queue state, an ending, a bar or a drop. **G4 says it outright: nothing is
//! cleared, warned about or decided from the disk figure** (`State, identity
//! and recovery` §11.4), and a writability test is never a signal.
//!
//! # G1: card-level, per card, and the count is a count
//!
//! **The reading is taken by the supervisor through the driver's own per-card
//! query, unprivileged, on request.** It works unprivileged because the
//! driver's device nodes are readable by every login. **Card-level and never
//! process-level**: the two differ by driver context not attributed to the
//! process, and the two opposite endings a full card gives turn on who filled
//! the card, which a reading of the render alone would answer neither way.
//!
//! **The count is read as a count and never as a parsed total** — the per-card
//! query returns its value once per card, so a whole-output parse reads `22` on
//! a two-card box, and it passes every test ever written on a one-card
//! machine. [`smoozy_runtime::health_readings::how_many_cards`] is the count
//! and this module composes no second one.
//!
//! # G3: the host-inherited routes lie on a container and are not used
//!
//! **Card presence and readings come from the driver's own answer, selected by
//! the detected machine kind** — `Rented-server runtime` V6 entire, cited.
//! **A reading taken from a container is not a reading of the container**, and
//! the three forbidden routes are that crate's own closed list.
//!
//! # What is not established, and is left absent rather than written from
//! memory
//!
//! **The driver's per-card query field that carries a card's identity is on
//! record in no document of this project.** What is on record is the
//! *requirement* — a card's identity needs its UUID and its bus location,
//! because two cards of one model are otherwise indistinguishable — and the
//! spellings of three other fields. **So the identity field is an absence with
//! a name** ([`TheIdentityFieldsSpelling`]) rather than a guess, exactly as the
//! colour lines' printed text is at [`crate::ending::ThePrintedText`]. **A
//! spelling written from memory here would be invisible until a real rental.**
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rented-server runtime` V2, V5, V6. `Server
//! protocol and transfers` §9.1, F1, F3, F16. `State, identity and recovery`
//! §11.4. `Servers` H5a. `Desktop app and delivery` P10d.
//!
//! **`Servers` Q1, `State, identity and recovery` D5 and `Desktop app and
//! delivery` P9 are deliberately not on that list and are written out at every
//! site instead**, because this plan defines a clause of each of those names
//! too.

use smoozy_runtime::health_readings::{how_many_cards, HealthReading};
use smoozy_runtime::machine_kind::MachineKind;
use smoozy_framing::free_space::{FreeSpaceReading, WhyThereIsNoReading};
use smoozy_state::observed::Reading;

/// **The driver's per-card query fields this document asks for, in the
/// spellings the project has on record and no others.**
///
/// - `index` — **observed in a test**, `Servers` H5a: `nvidia-smi
///   --query-gpu=index` returned the rental's own two cards on a four-card
///   host, where the kernel's per-card directory returned four.
/// - `memory.used` and `memory.total` — **observed in a test**, on the rented
///   Linux machine, taken before and after a render with the figures moving on
///   the card the render was pinned to and the idle card reading zero at the
///   same instant.
///
/// **The format is asked for headerless and one row per card**, which is the
/// same shape `Rented-server runtime`'s own count already asks for.
pub const THE_PER_CARD_QUERY_FIELDS_ON_RECORD: [&str; 3] = ["index", "memory.used", "memory.total"];

/// **The field that carries a card's identity, and its spelling is on record
/// in no document.**
///
/// **What is on record is what identity has to be** — a card's UUID and its
/// bus location, because both cards of one two-card rental were the same model
/// and the model name alone does not separate them. **What is not on record is
/// which per-card query fields carry them.**
///
/// **One variant on purpose.** An absence a type can tell from a value, so a
/// reader cannot mistake a guess for a reading, and so nothing composes a query
/// string that was never run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheIdentityFieldsSpelling {
    /// On record in no document of this project.
    OnRecordInNoDocument,
}

/// **G1's reading, per card.**
///
/// **Used against capacity as read** (G2): the two figures are the driver's
/// own and nothing here derives a third from them. **Free is not total and the
/// entry never conflates them** — 483 MiB of each observed card was already
/// held by the driver at idle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ACardsReading {
    /// The card's place in the driver's own ordering, which is the `index`
    /// field. **It is not the kernel's minor number and not a bus scan's
    /// position** — those are the routes G3 forbids on a container.
    pub index: u64,
    pub used_bytes: u64,
    /// **Capacity as the driver reports it.** The server list's own capacity is
    /// `Servers`', and the two are separate facts by the spec's own split.
    pub total_bytes: u64,
    /// **The fuller identity, absent because its query field's spelling is on
    /// record nowhere.**
    pub identity: TheIdentityFieldsSpelling,
}

/// **The machine, on the far side of one trait, because it genuinely is.**
///
/// **Both readings travel `Rented-server runtime` §12's health-readings
/// route** and are taken on request, never on a schedule (V2).
pub trait TakesTheEntrysReadings {
    /// **G1 and G3: the driver's own answer, one row per card, on the route
    /// the detected machine kind selects.** A tool that is not there, or that
    /// refuses, is a reading that could not be taken — **never an answer of no
    /// cards.**
    fn one_row_per_card(&self, kind: MachineKind) -> Result<Vec<ACardsReading>, String>;

    /// **G4: the available figure at the root's mount, read per path**, by
    /// `Server protocol and transfers` §9.1's mechanism. The path it was taken
    /// against rides the reading (that document's F1), so **nothing downstream
    /// can widen a per-path answer into a per-machine one** (its F16).
    fn the_available_figure_at(
        &self,
        the_root_of_the_job_area: &str,
    ) -> Result<FreeSpaceReading, WhyThereIsNoReading>;
}

/// **G5: both readings, carried on the health-readings route, each with its own
/// moment and clock, and nothing durable written from either.**
///
/// **The card count is the number of rows** and never a parsed total
/// (`Rented-server runtime` V6), which is what stops `22` on a two-card box.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheEntryShows {
    pub cards: HealthReading<Vec<ACardsReading>>,
    /// **Absent is absent.** A reading that could not be taken is stated as
    /// unavailable and never substituted for or read as zero.
    pub disk: HealthReading<Result<FreeSpaceReading, WhyThereIsNoReading>>,
}

impl WhatTheEntryShows {
    /// **The card count, counted rather than parsed.**
    pub fn how_many_cards(&self) -> usize {
        let rows: Vec<&str> = self.cards.reading.value().iter().map(|_| "").collect();
        how_many_cards(&rows)
    }
}

/// **G1, G3, G4 and G5, performed as one act at one of G5a's two requests.**
///
/// **Both readings are taken in the same act and each carries its own moment
/// and clock** (`State, identity and recovery` D5, `Rented-server runtime`
/// V5). **Nothing durable is written from either**, and this function has no
/// route to a store.
pub fn take_the_entrys_readings(
    machine: &dyn TakesTheEntrysReadings,
    kind: MachineKind,
    the_root_of_the_job_area: &str,
    at: std::time::SystemTime,
    on: smoozy_state::observed::Clock,
) -> Result<WhatTheEntryShows, String> {
    let cards = machine.one_row_per_card(kind)?;
    let disk = machine.the_available_figure_at(the_root_of_the_job_area);
    Ok(WhatTheEntryShows {
        cards: HealthReading {
            reading: Reading::taken(cards, at, on.clone()),
        },
        disk: HealthReading {
            reading: Reading::taken(disk, at, on),
        },
    })
}

/// **G5a's two requests, and the list is closed.**
///
/// **Each is a reading the user's own act already causes** — `State, identity
/// and recovery` §8.4's third moment in its own words. **No watcher, no timer
/// and no schedule is added**, so `Rented-server runtime` V2 stands unqualified
/// and `Desktop app and delivery` P9's *nothing on this side polls* is
/// untouched.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhenTheseReadingsAreTaken {
    /// **With `Servers` Q1 step 7's queue reading, on every primary
    /// connection, after the installation identity confirms** — one request
    /// per connection, carried on the same connection as the reading it rides.
    WithTheQueueReadingOnAPrimaryConnection,
    /// **At the user's own act of opening a job's entry**, once per opening.
    WhenTheUserOpensAJobsEntry,
}

impl WhenTheseReadingsAreTaken {
    /// **The two, and there are no others.**
    pub const THE_ONLY_TWO: [WhenTheseReadingsAreTaken; 2] = [
        WhenTheseReadingsAreTaken::WithTheQueueReadingOnAPrimaryConnection,
        WhenTheseReadingsAreTaken::WhenTheUserOpensAJobsEntry,
    ];

    /// **Neither recurs.** `Desktop app and delivery` P10d's closed set is
    /// what repeats inside a standing period and a health reading is excluded
    /// from it by name, **so an entry left open shows the figures it was given
    /// until one of the two acts above happens again. Nothing on the entry
    /// ticks and nothing re-reads to keep a figure fresh.**
    pub fn recurs_inside_a_standing_period(self) -> bool {
        false
    }
}

/// **G5a's last bullet: where no connection stands, no reading is taken and
/// the entry shows none.**
///
/// **Absence, which R4 already makes an ordinary state of the entry rather
/// than a fault**, and which N17 already covers for an entry whose machine is
/// gone.
pub const WHERE_NO_CONNECTION_STANDS_THE_ENTRY_SHOWS_NO_FIGURES: bool = true;

/// **G4, stated as a value: nothing is cleared, warned about or decided from
/// the disk figure.**
///
/// `State, identity and recovery` §11.4 — nothing on a machine is cleared on
/// the strength of a free-space reading alone — and the promise's reach is
/// `File Browser`'s reading, which this document does not exceed.
pub const NOTHING_IS_DECIDED_FROM_THE_DISK_FIGURE: bool = true;

/// **G5's own remainder, recorded rather than rescued.**
///
/// Fact 43's remainder — two renders on two cards, one render spanning both,
/// cross-login attribution — **bites nothing here, because the figure shown is
/// card-level and attribution is never claimed.** **No per-process or
/// per-render narrowing is built**, which is the whole of what the remainder
/// would be needed for.
pub const ATTRIBUTION_IS_NEVER_CLAIMED: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_framing::free_space::WhichRoute;
    use smoozy_state::observed::{Clock, MachineClock};
    use std::time::SystemTime;

    struct AMachineInHand {
        cards: Result<Vec<ACardsReading>, String>,
        disk: Result<FreeSpaceReading, WhyThereIsNoReading>,
        asked_for: std::cell::RefCell<Vec<String>>,
    }

    impl TakesTheEntrysReadings for AMachineInHand {
        fn one_row_per_card(&self, _kind: MachineKind) -> Result<Vec<ACardsReading>, String> {
            self.cards.clone()
        }

        fn the_available_figure_at(
            &self,
            the_root_of_the_job_area: &str,
        ) -> Result<FreeSpaceReading, WhyThereIsNoReading> {
            self.asked_for
                .borrow_mut()
                .push(the_root_of_the_job_area.to_string());
            self.disk.clone()
        }
    }

    fn a_card(index: u64, used: u64, total: u64) -> ACardsReading {
        ACardsReading {
            index,
            used_bytes: used,
            total_bytes: total,
            identity: TheIdentityFieldsSpelling::OnRecordInNoDocument,
        }
    }

    fn a_clock() -> Clock {
        Clock::AMachine(MachineClock("server-1".to_string()))
    }

    fn a_machine(cards: Vec<ACardsReading>) -> AMachineInHand {
        AMachineInHand {
            cards: Ok(cards),
            disk: Ok(FreeSpaceReading {
                available_bytes: 900,
                the_path_it_was_taken_against: "/ephemeral/smoozy".to_string(),
                route: WhichRoute::TheStatvfsExtension,
            }),
            asked_for: std::cell::RefCell::new(Vec::new()),
        }
    }

    /// **G1 and G2: the two figures are the driver's own, used against
    /// capacity, and the idle card reads zero at the same instant.**
    #[test]
    fn the_reading_is_per_card_and_used_against_capacity_as_read() {
        let machine = a_machine(vec![
            a_card(0, 2_937_282_560, 25_757_220_864),
            a_card(1, 0, 25_757_220_864),
        ]);
        let shown = take_the_entrys_readings(
            &machine,
            MachineKind::Systemd,
            "/ephemeral/smoozy",
            SystemTime::UNIX_EPOCH,
            a_clock(),
        )
        .expect("taken");
        let cards = shown.cards.reading.value();
        assert_eq!(cards.len(), 2);
        assert_eq!(cards[1].used_bytes, 0, "the idle card reads zero");
        assert_eq!(cards[0].total_bytes, cards[1].total_bytes);
        assert!(matches!(shown.cards.reading.clock(), Clock::AMachine(_)));
    }

    /// **G1: the count is a count.** Two cards read two, which is what a
    /// whole-output parse would have read as `22`.
    #[test]
    fn two_cards_count_two_and_never_twenty_two() {
        let machine = a_machine(vec![a_card(0, 1, 2), a_card(1, 1, 2)]);
        let shown = take_the_entrys_readings(
            &machine,
            MachineKind::Container,
            "/ephemeral/smoozy",
            SystemTime::UNIX_EPOCH,
            a_clock(),
        )
        .expect("taken");
        assert_eq!(shown.how_many_cards(), 2);
    }

    /// **G4: the figure is taken at the root of the job area, per path, and
    /// the path it was taken against rides the reading.**
    #[test]
    fn the_disk_figure_is_taken_at_the_root_of_the_job_area_and_names_its_path() {
        let machine = a_machine(Vec::new());
        let shown = take_the_entrys_readings(
            &machine,
            MachineKind::Systemd,
            "/ephemeral/smoozy",
            SystemTime::UNIX_EPOCH,
            a_clock(),
        )
        .expect("taken");
        assert_eq!(machine.asked_for.borrow().as_slice(), ["/ephemeral/smoozy"]);
        let reading = shown.disk.reading.value().as_ref().expect("read");
        assert_eq!(reading.the_path_it_was_taken_against, "/ephemeral/smoozy");
    }

    /// **A reading that could not be taken is stated as unavailable and never
    /// read as zero.**
    #[test]
    fn a_disk_figure_that_could_not_be_taken_is_absent_and_never_a_zero() {
        let machine = AMachineInHand {
            cards: Ok(Vec::new()),
            disk: Err(WhyThereIsNoReading::OnlyTheCarriageIsLeftAndItsProgramIsUnestablished),
            asked_for: std::cell::RefCell::new(Vec::new()),
        };
        let shown = take_the_entrys_readings(
            &machine,
            MachineKind::Systemd,
            "/ephemeral/smoozy",
            SystemTime::UNIX_EPOCH,
            a_clock(),
        )
        .expect("taken");
        assert!(shown.disk.reading.value().is_err());
        const _: () = assert!(NOTHING_IS_DECIDED_FROM_THE_DISK_FIGURE);
    }

    /// **G1: a driver's tool that refuses is a reading that could not be
    /// taken, never an answer of no cards.**
    #[test]
    fn a_refused_card_query_is_a_reading_that_could_not_be_taken() {
        let machine = AMachineInHand {
            cards: Err("the driver's own query did not answer".to_string()),
            disk: Err(WhyThereIsNoReading::OnlyTheCarriageIsLeftAndItsProgramIsUnestablished),
            asked_for: std::cell::RefCell::new(Vec::new()),
        };
        assert!(take_the_entrys_readings(
            &machine,
            MachineKind::Systemd,
            "/ephemeral/smoozy",
            SystemTime::UNIX_EPOCH,
            a_clock(),
        )
        .is_err());
    }

    /// **G5a: the two requests are the only two, and neither recurs.**
    #[test]
    fn the_two_requests_are_the_only_two_and_neither_recurs() {
        assert_eq!(WhenTheseReadingsAreTaken::THE_ONLY_TWO.len(), 2);
        for when in WhenTheseReadingsAreTaken::THE_ONLY_TWO {
            assert!(!when.recurs_inside_a_standing_period());
        }
        const _: () = assert!(WHERE_NO_CONNECTION_STANDS_THE_ENTRY_SHOWS_NO_FIGURES);
        const _: () = assert!(ATTRIBUTION_IS_NEVER_CLAIMED);
    }

    /// **The identity field's spelling is an absence and nothing composes a
    /// query string that was never run.**
    #[test]
    fn the_identity_fields_spelling_is_absent_and_no_field_is_written_from_memory() {
        assert_eq!(
            TheIdentityFieldsSpelling::OnRecordInNoDocument,
            a_card(0, 1, 2).identity
        );
        assert_eq!(
            THE_PER_CARD_QUERY_FIELDS_ON_RECORD,
            ["index", "memory.used", "memory.total"]
        );
        let source = include_str!("machine_readings.rs");
        let working_half = source
            .split("#[cfg(test)]")
            .next()
            .expect("the file has a working half");
        let code: String = working_half
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join(" ");
        for guessed in ["uuid", "bus_id", "gpu_name", "utilization"] {
            assert!(
                !code.contains(guessed),
                "{guessed} is a query field spelling on record in no document"
            );
        }
    }

    /// **G3: nothing here reads the host-inherited routes**, and the three
    /// forbidden ones are `Rented-server runtime`'s closed list.
    #[test]
    fn nothing_here_reads_a_route_forbidden_on_a_container() {
        let source = include_str!("machine_readings.rs");
        let working_half = source
            .split("#[cfg(test)]")
            .next()
            .expect("the file has a working half");
        let code: String = working_half
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join(" ");
        for forbidden in ["/proc/driver", "lspci", "/dev/nvidia"] {
            assert!(
                !code.contains(forbidden),
                "{forbidden} is a forbidden route"
            );
        }
        assert_eq!(
            smoozy_runtime::health_readings::ACardReadingRouteThisDocumentForbidsOnAContainer::ALL
                .len(),
            3
        );
    }
}
