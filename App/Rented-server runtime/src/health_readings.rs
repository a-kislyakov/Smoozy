//! §12 — health readings, and two clauses that are not health readings and sit
//! here because the mechanism they use is here.
//!
//! **V1. *Health exchange* and *health readings* are two different things and are
//! not one word.** The health exchange is the package commit's verification step
//! (§2.4) and happens once per transaction. **Health readings are what the machine
//! reports about itself on request.**
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `State, identity and recovery` §1.3 rule 3, §3 D5,
//! §8.1, §8.3, §8.4, §8.6, §11.1. `Server protocol and transfers` F1, F2, F3,
//! F4, F5, F6a, F11a, F14, F15, F16, and its §9.1. `Servers` H9a, H11, H12, W41a.
//! `Telegram` §10 C1 and C2, and its §21 item 21. `Desktop app and delivery` §3.3
//! P10d and its §16.4. `Job files and outputs` D20, D21, L4a. `Queue & History`
//! §18 item 10.
//!
//! **A marker this plan also defines is never listed here and is written out
//! with its document at the site**, because a header list cannot tell the two
//! apart and a bare one there would resolve to a real and wrong clause.

use smoozy_framing::free_space::{FreeSpaceReading, WhyThereIsNoReading};
use smoozy_state::observed::Reading;

use crate::machine_kind::MachineKind;

/// **V2. Every health reading is taken on request and never on a schedule, with
/// one named exception.** Nothing here polls, nothing watches, and **no reading is
/// taken to keep a message fresh.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhenAReadingIsTaken {
    OnRequest,
    /// **The exception, and it has one beneficiary and no others: the `Telegram`
    /// role worker's status reading.** It is taken **by that worker on its own
    /// machine, about that machine, on that machine's own clock**, at the cadence
    /// `Telegram` §10 C2 states; **it is used for the status message and for the
    /// gather's reply and for nothing else**; and it is **never sent to the
    /// desktop and never written durably.**
    ///
    /// **This is an exception with a named beneficiary rather than a general
    /// loosening**, and V3's *no health check pings* is untouched by it.
    TheTelegramRoleWorkersStatusReadingOnItsOwnMachine,
}

/// **The desktop's live-reading sequence carries no health reading and neither
/// uses nor widens this exception.**
///
/// `Desktop app and delivery` §3.3 P10d's set is closed — the journal
/// read-forward, the queue reading, the commentary read — **and this section's
/// readings are named out of it**, so no health reading recurs for it and every
/// one stays on request.
pub const NO_HEALTH_READING_IS_IN_THE_DESKTOPS_LIVE_READING_SET: bool = true;

/// **V3. No health check pings.**
///
/// Fact 57: *"one provider answers no ping at all"* while its connection works
/// perfectly, **so any ping-based health check would report it dead.** The value
/// is `()` because the clause is an absence: nothing in this crate pings, and
/// there is no reading here a ping could answer.
pub const NO_HEALTH_CHECK_PINGS: () = ();

/// **V4. The observed value of the *supervisor unavailable* condition is one
/// thing: whether the supervisor answered a request on the command path.**
///
/// **It is not a reading of a process, not a reading of a file, and not a reading
/// of the connection.** `Servers` supplies the connection's own condition and
/// **the two are never inferred from one another.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupervisorUnavailable {
    ItAnsweredARequestOnTheCommandPath,
    ItDidNotAnswer,
}

/// **V5. Every health reading carries the moment it was taken and the clock it was
/// taken on, and it is period-held** (`State, identity and recovery` D5, §11.1).
///
/// **Nothing durable is written from a health reading, and no health reading is
/// ever journalled or written into a job record** — that half holds for the
/// readings this section is about, **everywhere and without exception**.
///
/// **And no promise rests on a health reading, with the single exception V2
/// names**: the status message's statement of what the machine is using rests on a
/// reading, and its statement of how long the machine has been up rests on one
/// **only on the machine kinds where `Telegram` C1 makes that statement** — C1
/// holds that half conditional on the detected kind (K1) and **states it as
/// unavailable on the kind where the figure is not the machine's own**, so nothing
/// rests on a reading there.
///
/// **What separates M4a's ceiling and V12's reading from a health reading is what
/// is being recorded.** A health reading is a live value with a lifetime — what
/// the machine is using now — **so writing one down puts a claim about the present
/// into a record that cannot be refreshed**, which is the fault this rule exists to
/// prevent. Those two are **figures about the moment they were taken, written once
/// into a record nothing rewrites**, and neither is refreshed, re-read or
/// withdrawn.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HealthReading<T> {
    /// The value with its moment and its clock, taken whole from `State, identity
    /// and recovery`.
    pub reading: Reading<T>,
}

impl<T> HealthReading<T> {
    /// **V5.** A health reading is never written durably. The type carries no
    /// route to a store, and **that absence is the clause.**
    pub const NOTHING_DURABLE_IS_WRITTEN_FROM_ONE: () = ();
}

/// **V6. Which mechanism a reading is taken by is chosen by the detected machine
/// kind (K1), and getting that wrong is silent.**
///
/// Three consequences bind, and each is a trap rather than a preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhereAProcessorOrMemoryReadingComesFrom {
    /// On a systemd machine the machine's own figures are the machine's own.
    TheMachinesOwn,
    /// **On a container the machine's own memory and processor readings are
    /// wrong** — one box said four processors and 15.57 GiB while the real
    /// allocations were 3.84 and 14.95 GiB, and a **20-core** slice on a second
    /// host reported **40** and the host's **64 GiB**. **The control group's
    /// figures are the ones to take.**
    ///
    /// **Both control-group generations are read and neither is assumed.** The
    /// reading is attempted at the newer pair **and** at the older three,
    /// whichever answers.
    TheControlGroupsWhicheverGenerationAnswers,
    /// **V6's middle outcome: the control group read the literal unlimited value,
    /// so nothing is capping the machine and its own figure is the right one.**
    ///
    /// V6 says it in terms — the control group's figures are the ones to take,
    /// *"falling back to the machine's own only where the cgroup reads the
    /// literal unlimited value"*. **The trigger is that literal value and nothing
    /// else**: an absent pair is [`Self::NotEstablished`]'s case and not this
    /// one, and reading the two as one would put the host's figure in front of a
    /// user on a slice, which is the eightfold overstatement the rule exists to
    /// close.
    ///
    /// **`Servers` H9a holds the same rule from the reading side and the two must
    /// stay identical**, which is why this member is written in the shape that
    /// document's own reading already has.
    TheMachinesOwnBecauseTheControlGroupReadsUnlimited,
    /// **Where neither answers, the allocation is not established and the
    /// machine's own figure is not substituted for it** — a stated absence,
    /// carried as such.
    ///
    /// **That case is observed rather than hypothetical**: on one box the older
    /// paths did not exist at all, so **a rule written on the older pair alone
    /// fell through to the host's answer while believing it had the rental's**,
    /// which is the eightfold error arriving by the route the rule was meant to
    /// close.
    NotEstablished,
}

/// **What one control-group path pair answered, and it is three things rather
/// than a yes or a no.**
///
/// **A yes-or-no reading cannot carry V6's middle outcome at all**, because the
/// literal unlimited value is an answer — the pair is there and says nothing is
/// capping the machine — and an absent pair is not. `Servers` H9a's own reading
/// carries the same three, and V6 says the two rules must stay identical; **this
/// is written alike rather than borrowed**, the clause being this document's.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatAControlGroupPairAnswered {
    /// A figure — **the rental's allocation.**
    AFigure,
    /// **The literal unlimited value, and that is the trigger and nothing
    /// else.** Nothing is capping the machine, so the machine's own figure is
    /// right.
    TheLiteralUnlimitedValue,
    /// The pair is not present, or is present and empty.
    NoRoute,
}

/// **V6's first consequence, and it has four answers rather than three.**
///
/// `Servers` H9a holds the same rule from the reading side and **the two must
/// stay identical.** The newer pair is read first and the older second,
/// **whichever answers**, and a pair with no route falls through to the next
/// rather than ending the reading.
pub fn where_a_processor_or_memory_reading_comes_from(
    kind: MachineKind,
    the_newer_control_group: WhatAControlGroupPairAnswered,
    the_older_control_group: WhatAControlGroupPairAnswered,
) -> WhereAProcessorOrMemoryReadingComesFrom {
    match kind {
        MachineKind::Systemd => WhereAProcessorOrMemoryReadingComesFrom::TheMachinesOwn,
        MachineKind::Container => {
            for answered in [the_newer_control_group, the_older_control_group] {
                match answered {
                    WhatAControlGroupPairAnswered::AFigure => {
                        return WhereAProcessorOrMemoryReadingComesFrom::TheControlGroupsWhicheverGenerationAnswers
                    }
                    WhatAControlGroupPairAnswered::TheLiteralUnlimitedValue => {
                        return WhereAProcessorOrMemoryReadingComesFrom::TheMachinesOwnBecauseTheControlGroupReadsUnlimited
                    }
                    WhatAControlGroupPairAnswered::NoRoute => continue,
                }
            }
            WhereAProcessorOrMemoryReadingComesFrom::NotEstablished
        }
    }
}

/// **V6's second consequence: on a container the card readings inherited from the
/// host lie, and the newer case is quieter than the old one.**
///
/// The kernel's own driver directory and the bus scan **are inherited from the
/// host** and reported two devices in a container that could touch neither. **So
/// card presence is read from the driver's own answer and never from either of
/// those on a container.**
///
/// **The observed case used to be a container holding none of the host's cards,
/// where the reading is obviously wrong. It is now also a container holding *some*
/// of them, where it is not** — two hosts of four cards each, rented as two-card
/// slices, on which the driver's own query returned **2** and the host-visible
/// directory returned **4**, two of the identifiers belonging to another tenant.
/// **A count taken there is two too many and looks entirely reasonable.**
///
/// **The device-node route is forbidden too**: four nodes appeared for one
/// two-card rental and two for the other, and on the second the kernel's minor
/// numbers for our own cards were **2** and **0**, unrelated to the query's index.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ACardReadingRouteThisDocumentForbidsOnAContainer {
    TheKernelsOwnDriverDirectory,
    TheBusScan,
    TheDeviceNodes,
}

impl ACardReadingRouteThisDocumentForbidsOnAContainer {
    pub const ALL: [ACardReadingRouteThisDocumentForbidsOnAContainer; 3] = [
        ACardReadingRouteThisDocumentForbidsOnAContainer::TheKernelsOwnDriverDirectory,
        ACardReadingRouteThisDocumentForbidsOnAContainer::TheBusScan,
        ACardReadingRouteThisDocumentForbidsOnAContainer::TheDeviceNodes,
    ];
}

/// **V6's third consequence: the count is read as a count and never as a parsed
/// total.**
///
/// **A per-card query returns the value once per card**, so a caller parsing the
/// whole output as an integer gets `22` on a two-card machine. **It passes every
/// test ever written on a one-card machine.**
pub fn how_many_cards(one_line_per_card: &[&str]) -> usize {
    one_line_per_card.len()
}

/// **V7. Free space is a choice of mount and never a reading of the machine.**
///
/// `State, identity and recovery` §1.3 rule 3 and `Server protocol and transfers`
/// §9.1 own the mechanism. **What this document adds is the mount it names for its
/// own state**: the state area's mount is recorded at placement and **every
/// free-space claim about state names that mount.**
///
/// **The only other free-space reading this document takes is V12's**, which is
/// about a job's own output folder rather than about state and names that path's
/// mount instead.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhichFreeSpaceReadingThisDocumentTakes {
    /// M4a's ceiling is derived through this one.
    TheStateAreasRecordedMount,
    /// V12's.
    ThatJobsOwnOutputFolder,
}

/// **V12. A render's recorded outcome carries a free-space reading against that
/// job's own output folder, taken in the same act that writes the outcome.**
///
/// `Server protocol and transfers` F6a states the disk-full carve-out's route (b)
/// from the desktop's side and **leaves the machine-side act here**. **It is
/// neither of V1's two things, and neither V2 nor V5 reaches it**: it is taken
/// once, in one act, and **nothing schedules it, refreshes it or re-reads it.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheRendersFreeSpaceReading {
    /// **What is read**: F1's per-path reading against **that job's own output
    /// folder and nothing wider** — that document's F3 available figure, **never
    /// its F4 subtraction, never its F5 writability test, and never widened into a
    /// statement about the machine** (its F16).
    ///
    /// **One reading covers the job output folder and every write place the
    /// redirect brings under it** (`Job files and outputs` P7, D20), which is what
    /// makes a single per-path reading the right one; **it says nothing about the
    /// one residue the redirect does not reach** (that document's D21) and **is
    /// never widened to stand for it.**
    ///
    /// **Where it cannot be taken**: the job output folder gone, the mount absent,
    /// the call refusing — **none of it holds up the outcome**, which is decided
    /// and written whatever the reading does. **An outcome carrying no reading is
    /// not an outcome carrying a zero**, exactly as S13's outcome carrying no
    /// completeness statement is not one carrying an empty statement: the absence
    /// is recorded explicitly, **route (b) does not fire on it**, nothing is
    /// re-read afterwards to supply one, and **nothing infers a full disk from
    /// it.**
    pub reading: Result<FreeSpaceReading, WhyThereIsNoReading>,
    /// **What it carries**: the moment it was taken, on the machine's own clock,
    /// **and it is immutable with the outcome it rides.** It is a statement about a
    /// past moment rather than about the present.
    pub taken_at: std::time::SystemTime,
}

impl TheRendersFreeSpaceReading {
    /// **Only the render kind takes it**, and no other kind's ending is given one.
    pub const ONLY_THE_RENDER_KIND_TAKES_IT: bool = true;

    /// **What it needs**: no announced extension and no program on the machine
    /// beyond the supervisor itself. **After hand-over the supervisor is Smoozy's
    /// own code and takes the filesystem call directly**, so this reading is **not
    /// one of the two routes `Server protocol and transfers` F2 negotiates**, and
    /// neither of those routes going unannounced withdraws it.
    pub const IT_NEEDS_NO_ANNOUNCED_EXTENSION_AND_NO_OTHER_PROGRAM: bool = true;

    /// **Whether route (b) may fire on this outcome.** **It does not fire on a
    /// missing reading.**
    pub fn route_b_may_fire(&self) -> bool {
        self.reading.is_ok()
    }
}

/// **V12's *what it rides*, and the correction that goes with it.**
///
/// It rides **the write that records the outcome**, so it needs no durability the
/// ending does not already need and reaches a reader by the route the ending does.
/// **On a mount the render has filled, that write is refused and both it and this
/// reading ride the pending outcome under S14** — the ending is late rather than
/// guaranteed, and **S14's hold-and-re-attempt is the whole of what saves the
/// case.**
///
/// **M4a's control-state reserve is not what leaves room for it**: M4a says in
/// terms that the reserve protects the state mount from Smoozy's own stream
/// capture and *"claims no protection from the user's outputs or another process
/// filling that mount"*, `Job files and outputs` L4a says the separation is
/// placement and not protection, and that document's §14 item 21 states what breaks
/// on a machine with one writable mount. **A builder who took the opposite at face
/// value had no reason to build S14's pending queue at all.**
pub const THE_RESERVE_IS_NOT_WHAT_LEAVES_ROOM_FOR_THE_OUTCOMES_OWN_WRITE: bool = true;

/// **V12's last bullet: where the outcome's own write is refused for want of room,
/// the reading is the first attempt's and rides the pending outcome unchanged.**
///
/// **Re-taken at the landing it would read the freed mount and silence the
/// diagnosis** — the fault the repair closed, arriving again by that window.
pub const THE_READING_IS_THE_FIRST_ATTEMPTS_AND_IS_NOT_RE_TAKEN_AT_THE_LANDING: bool = true;

/// **What V12 is not.**
///
/// **It is never a supplier of the live disk-full condition**, which keeps `State,
/// identity and recovery` §8.3's own two current suppliers; **it is not V4's
/// observed value**; and **it never becomes a statement about why the render
/// ended.** What the user meets is `Desktop app and delivery` §16.4 M15's
/// presentation of Row 287 and **nothing new from here.**
pub const V12_SUPPLIES_NOTHING_TO_THE_LIVE_DISK_FULL_CONDITION: () = ();

/// **V8. A writability test is not a health reading and is not a signal.**
///
/// On a full filesystem **the writability test returns writable and creating an
/// empty file succeeds; only writing at least one byte fails** (`State, identity
/// and recovery` §1.3 rule 4).
pub const A_WRITABILITY_TEST_IS_NOT_A_READING: () = ();

/// **V9. The machine-level *setup in progress* state and its setup run's identity
/// are reported with every health answer** (P7), **so the state is learned on the
/// first request rather than discovered by a machine doing nothing.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HealthAnswerCarries {
    pub setup_in_progress: Option<crate::setup_state::SetupInProgressState>,
}

/// A health answer is an observation, never authority to alter desktop security.
pub const NOTHING_IN_A_HEALTH_ANSWER_IS_AN_ENTITLEMENT_STATEMENT: () = ();

/// **V11. A machine's clock is its own and readings taken on different clocks are
/// never subtracted** (`State, identity and recovery` D5).
///
/// **A container has no clock of its own** — the reading is the host kernel's and
/// **nothing inside can discipline it** (§16 item 10). A fifth reading of that, on
/// a second seller: **no time daemon of any kind present**, the control tool
/// unusable because there is no service manager, and **the clock nonetheless
/// correct to the second**. **The new detail is the second half rather than the
/// first: nothing inside can correct it either**, so a container whose host clock
/// is wrong stays wrong for the whole rental and no step of this document can act
/// on it.
pub const READINGS_ON_DIFFERENT_CLOCKS_ARE_NEVER_SUBTRACTED: () = ();

/// **V13. Nothing this document places on a machine can end that machine's rental
/// or cap its life, and Smoozy builds nothing that tries.**
///
/// **This is the architecture half of the runaway rental.** It is here because
/// both of its grounds are properties of the machine and of what runs on it;
/// **the half the user meets is not this document's** and is named at §18.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyNothingHereCanEndARental {
    /// **The provider publishes no way to cap a rental's life, and this was read
    /// rather than assumed.** The create-instance reference lists the request body
    /// in full and **carries no duration field at all**; a duration exists only as
    /// a read-only number on the instance. The provider's own answer to *what does
    /// lifetime mean*: *"This rental end date is locked when you rent and cannot be
    /// changed by the host."* **A cap was asked for on creation and again on the
    /// running instance; both reported success and changed nothing**, which is what
    /// an ignored unknown field looks like.
    TheProviderPublishesNoWayToCapIt,
    /// **A watchdog inside a container cannot end the container either, and that
    /// is a kernel rule rather than a provider's choice.** Only signals the
    /// namespace's first process has established a handler for can be sent to it
    /// from inside, and the forcible signal **can never be caught, so no handler
    /// for it can ever exist**; the forcible route runs from an ancestor namespace,
    /// which is the provider's side. **Every worker class of §5, and the supervisor
    /// itself, is inside that namespace.** The observed attempt is recorded: a
    /// watchdog written to kill the first process at a chosen hour **did not end
    /// the box.**
    NothingInsideAContainerCanEndIt,
}

impl WhyNothingHereCanEndARental {
    pub const BOTH: [WhyNothingHereCanEndARental; 2] = [
        WhyNothingHereCanEndARental::TheProviderPublishesNoWayToCapIt,
        WhyNothingHereCanEndARental::NothingInsideAContainerCanEndIt,
    ];
}

/// **V13's third and fourth bullets, and they are a ruling rather than an
/// omission.**
///
/// **So the only thing that ends the charge is a person destroying the rental by
/// hand.** This document places nothing on a machine that could do it, **and no
/// clause here is to be read as licence to add one.**
///
/// **No automatic destruction is built.** A rescue that destroyed a machine would
/// destroy the work on it, **and that is the worse of the two failures.** There is
/// no timer here that reaches a provider, no condition that ends a rental, and **no
/// *give up and destroy* branch anywhere in §2, §3 or §5.**
pub const NO_AUTOMATIC_DESTRUCTION_IS_BUILT: () = ();

/// **V13's fifth bullet: the running time is a reading, and it is conditional on
/// the machine kind.**
///
/// **On a systemd rental the uptime reading is the machine's own. On a container it
/// is the host's and would lie** — 8.2 days reported on a container created that
/// morning — **so it is not a figure about the rental at all**, and the ruled
/// conduct stands unchanged: **where the reading would not be the machine's own,
/// the line says it is unavailable rather than showing the host's.** §16 item 31
/// carries what that costs, and **this document opens no second route to the
/// figure.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheRunningTime {
    /// The machine's own.
    TheMachinesOwn,
    /// **Unavailable**, said rather than shown as the host's.
    Unavailable,
}

/// **V13.** Which the machine kind gives.
pub fn the_running_time(kind: MachineKind) -> TheRunningTime {
    match kind {
        MachineKind::Systemd => TheRunningTime::TheMachinesOwn,
        MachineKind::Container => TheRunningTime::Unavailable,
    }
}

/// **V13's last bullet: what this clause supplies to whatever surface tells the
/// user, and what it refuses to supply.**
///
/// **The statement — that a rented machine runs and bills until it is destroyed by
/// hand — is true on every machine kind, needs no reading, and must never be made
/// conditional on one**; the machine that cannot report its running time is exactly
/// the machine the statement matters most on.
///
/// **This document supplies the two negatives and the kind-conditional reading and
/// nothing else. Which surface says it, when, and in what words is not decided
/// here** (§18). **Who makes the request is `Servers` W41a's** — one request per
/// primary connection, on the confirmed branch only. **V2 is untouched by it**: a
/// reading taken once when a connection is made is taken on request. **Where the
/// request is not made or is not answered, no figure stands and none is inferred**
/// — the stated absence, never a zero and never a held figure carried forward as
/// current.
pub const THE_STATEMENT_IS_TRUE_ON_EVERY_KIND_AND_NEEDS_NO_READING: bool = true;

/// The readings themselves, behind one trait.
///
/// **Every one of them is on request** (V2), so the trait has no schedule, no
/// watcher and no subscription — **and that shape is the clause.**
pub trait TakesHealthReadings {
    /// **V4.** Whether the supervisor answered a request on the command path.
    fn supervisor_answered(&self) -> Result<SupervisorUnavailable, String>;
    /// **V6.** One line per card, counted rather than parsed as a total.
    fn one_line_per_card(&self) -> Result<Vec<String>, String>;
    /// **V13's second half.** `None` is the stated absence on the kind where the
    /// figure would not be the machine's own.
    fn the_machines_own_running_time(&self) -> Result<Option<std::time::Duration>, String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_containers_processor_and_memory_readings_come_from_the_control_group() {
        assert_eq!(
            where_a_processor_or_memory_reading_comes_from(
                MachineKind::Container,
                WhatAControlGroupPairAnswered::AFigure,
                WhatAControlGroupPairAnswered::NoRoute
            ),
            WhereAProcessorOrMemoryReadingComesFrom::TheControlGroupsWhicheverGenerationAnswers
        );
        assert_eq!(
            where_a_processor_or_memory_reading_comes_from(
                MachineKind::Container,
                WhatAControlGroupPairAnswered::NoRoute,
                WhatAControlGroupPairAnswered::AFigure
            ),
            WhereAProcessorOrMemoryReadingComesFrom::TheControlGroupsWhicheverGenerationAnswers
        );
    }

    /// **V6's middle outcome.** A control group that reads the literal unlimited
    /// value is not capping the machine, so the machine's own figure is the
    /// right one — and that is a different answer from *not established*, where
    /// the machine's own figure must not be substituted at all.
    #[test]
    fn a_control_group_reading_the_literal_unlimited_value_takes_the_machines_own_figure() {
        assert_eq!(
            where_a_processor_or_memory_reading_comes_from(
                MachineKind::Container,
                WhatAControlGroupPairAnswered::TheLiteralUnlimitedValue,
                WhatAControlGroupPairAnswered::NoRoute
            ),
            WhereAProcessorOrMemoryReadingComesFrom::TheMachinesOwnBecauseTheControlGroupReadsUnlimited
        );
        // The older pair answers it where the newer has no route at all.
        assert_eq!(
            where_a_processor_or_memory_reading_comes_from(
                MachineKind::Container,
                WhatAControlGroupPairAnswered::NoRoute,
                WhatAControlGroupPairAnswered::TheLiteralUnlimitedValue
            ),
            WhereAProcessorOrMemoryReadingComesFrom::TheMachinesOwnBecauseTheControlGroupReadsUnlimited
        );
        // **A figure is still a figure**: an unlimited answer further down does
        // not displace an allocation read higher up.
        assert_eq!(
            where_a_processor_or_memory_reading_comes_from(
                MachineKind::Container,
                WhatAControlGroupPairAnswered::AFigure,
                WhatAControlGroupPairAnswered::TheLiteralUnlimitedValue
            ),
            WhereAProcessorOrMemoryReadingComesFrom::TheControlGroupsWhicheverGenerationAnswers
        );
        // **And it is not the same answer as *not established*.**
        assert_ne!(
            where_a_processor_or_memory_reading_comes_from(
                MachineKind::Container,
                WhatAControlGroupPairAnswered::TheLiteralUnlimitedValue,
                WhatAControlGroupPairAnswered::NoRoute
            ),
            WhereAProcessorOrMemoryReadingComesFrom::NotEstablished
        );
    }

    /// The eightfold error's own route: neither generation answers, and the
    /// machine's own figure is **not** substituted.
    #[test]
    fn neither_control_group_answering_is_a_stated_absence_and_not_the_hosts_figure() {
        assert_eq!(
            where_a_processor_or_memory_reading_comes_from(
                MachineKind::Container,
                WhatAControlGroupPairAnswered::NoRoute,
                WhatAControlGroupPairAnswered::NoRoute
            ),
            WhereAProcessorOrMemoryReadingComesFrom::NotEstablished
        );
    }

    #[test]
    fn a_systemd_machines_own_figures_are_its_own() {
        for newer in [
            WhatAControlGroupPairAnswered::AFigure,
            WhatAControlGroupPairAnswered::TheLiteralUnlimitedValue,
            WhatAControlGroupPairAnswered::NoRoute,
        ] {
            assert_eq!(
                where_a_processor_or_memory_reading_comes_from(
                    MachineKind::Systemd,
                    newer,
                    WhatAControlGroupPairAnswered::NoRoute
                ),
                WhereAProcessorOrMemoryReadingComesFrom::TheMachinesOwn
            );
        }
    }

    /// V6's third: `22` on a two-card machine is what parsing the whole output
    /// as an integer gives, and it passes every one-card test.
    #[test]
    fn the_card_count_is_a_count_of_lines_and_never_a_parsed_total() {
        assert_eq!(how_many_cards(&["2", "2"]), 2);
        assert_eq!(how_many_cards(&["1"]), 1);
        assert_eq!(how_many_cards(&[]), 0);
    }

    #[test]
    fn three_card_reading_routes_are_forbidden_on_a_container() {
        // **The set is closed by the compiler rather than by a count.** The
        // match is total and carries no default arm, so a route added to the
        // enum has none and the crate stops compiling.
        for (at, route) in ACardReadingRouteThisDocumentForbidsOnAContainer::ALL.iter().enumerate() {
            let where_the_list_puts_it = match route {
                ACardReadingRouteThisDocumentForbidsOnAContainer::TheKernelsOwnDriverDirectory => 0,
                ACardReadingRouteThisDocumentForbidsOnAContainer::TheBusScan => 1,
                ACardReadingRouteThisDocumentForbidsOnAContainer::TheDeviceNodes => 2,
            };
            assert_eq!(at, where_the_list_puts_it, "{route:?} is not where the list puts it");
        }
    }

    #[test]
    fn a_containers_running_time_is_unavailable_rather_than_the_hosts() {
        assert_eq!(the_running_time(MachineKind::Container), TheRunningTime::Unavailable);
        assert_eq!(the_running_time(MachineKind::Systemd), TheRunningTime::TheMachinesOwn);
    }

    #[test]
    fn route_b_does_not_fire_on_a_missing_reading() {
        let absent = TheRendersFreeSpaceReading {
            reading: Err(WhyThereIsNoReading::OnlyTheCarriageIsLeftAndItsProgramIsUnestablished),
            taken_at: std::time::SystemTime::UNIX_EPOCH,
        };
        assert!(!absent.route_b_may_fire());
        let taken = TheRendersFreeSpaceReading {
            reading: Ok(FreeSpaceReading {
                available_bytes: 0,
                the_path_it_was_taken_against: "/smoozy/jobs/1/output".into(),
                route: smoozy_framing::free_space::WhichRoute::TheStatvfsExtension,
            }),
            taken_at: std::time::SystemTime::UNIX_EPOCH,
        };
        assert!(taken.route_b_may_fire());
    }

    /// An outcome carrying no reading is not one carrying a zero — the two are
    /// different values and nothing collapses them.
    #[test]
    fn an_absent_reading_is_not_a_zero() {
        let zero = TheRendersFreeSpaceReading {
            reading: Ok(FreeSpaceReading {
                available_bytes: 0,
                the_path_it_was_taken_against: "/smoozy/jobs/1/output".into(),
                route: smoozy_framing::free_space::WhichRoute::TheStatvfsExtension,
            }),
            taken_at: std::time::SystemTime::UNIX_EPOCH,
        };
        let absent = TheRendersFreeSpaceReading {
            reading: Err(WhyThereIsNoReading::OnlyTheCarriageIsLeftAndItsProgramIsUnestablished),
            taken_at: std::time::SystemTime::UNIX_EPOCH,
        };
        assert_ne!(zero, absent);
    }

    #[test]
    fn the_two_negatives_behind_the_runaway_rental_are_both_named() {
        // **The pair is closed by the compiler rather than by a count.**
        for (at, why) in WhyNothingHereCanEndARental::BOTH.iter().enumerate() {
            let where_the_list_puts_it = match why {
                WhyNothingHereCanEndARental::TheProviderPublishesNoWayToCapIt => 0,
                WhyNothingHereCanEndARental::NothingInsideAContainerCanEndIt => 1,
            };
            assert_eq!(at, where_the_list_puts_it, "{why:?} is not where the list puts it");
        }
    }

    #[test]
    fn the_health_answer_carries_the_setup_state_where_one_stands() {
        let carries = HealthAnswerCarries {
            setup_in_progress: None,
        };
        assert!(carries.setup_in_progress.is_none());
    }

    /// **V2's rule and its one exception, exercised rather than compared.**
    ///
    /// The assertion this replaced compared two members of the enum that were
    /// never the same value, so it held by construction and checked the named
    /// behaviour nowhere. What is checked here is the rule itself: **every
    /// reading this document takes is on request, except the one whose
    /// beneficiary is named** — and the match is total, so a way of taking a
    /// reading added to the enum stops the crate compiling.
    #[test]
    fn a_reading_is_taken_on_request_and_the_one_exception_has_a_named_beneficiary() {
        for taken in [
            WhenAReadingIsTaken::OnRequest,
            WhenAReadingIsTaken::TheTelegramRoleWorkersStatusReadingOnItsOwnMachine,
        ] {
            let whose_it_is: Option<&str> = match taken {
                WhenAReadingIsTaken::OnRequest => None,
                WhenAReadingIsTaken::TheTelegramRoleWorkersStatusReadingOnItsOwnMachine => {
                    Some("the `Telegram` role worker's status reading")
                }
            };
            match taken {
                WhenAReadingIsTaken::OnRequest => assert!(
                    whose_it_is.is_none(),
                    "the rule has no beneficiary; it is what every reading takes"
                ),
                WhenAReadingIsTaken::TheTelegramRoleWorkersStatusReadingOnItsOwnMachine => {
                    assert!(
                        whose_it_is.is_some_and(|whose| !whose.is_empty()),
                        "the one exception names its beneficiary"
                    )
                }
            }
        }
    }

    /// **The two readings are named apart by what each is taken against**, which
    /// is the thing that matters: V7's is the state area's recorded mount and
    /// V12's is that job's own output folder, and a reading taken against one
    /// standing for the other is the fault the division exists to stop. The
    /// assertion this replaced compared two members of the enum that were never
    /// the same value.
    #[test]
    fn the_two_free_space_readings_this_document_takes_are_named_apart() {
        for reading in [
            WhichFreeSpaceReadingThisDocumentTakes::TheStateAreasRecordedMount,
            WhichFreeSpaceReadingThisDocumentTakes::ThatJobsOwnOutputFolder,
        ] {
            let taken_against = match reading {
                WhichFreeSpaceReadingThisDocumentTakes::TheStateAreasRecordedMount => {
                    "the state area's recorded mount"
                }
                WhichFreeSpaceReadingThisDocumentTakes::ThatJobsOwnOutputFolder => {
                    "that job's own output folder"
                }
            };
            assert!(!taken_against.is_empty());
            assert_ne!(
                taken_against,
                match reading {
                    WhichFreeSpaceReadingThisDocumentTakes::TheStateAreasRecordedMount =>
                        "that job's own output folder",
                    WhichFreeSpaceReadingThisDocumentTakes::ThatJobsOwnOutputFolder =>
                        "the state area's recorded mount",
                },
                "neither reading is taken against what the other is taken against"
            );
        }
    }

    /// **V4's value is one thing: whether the supervisor answered a request on
    /// the command path.** The assertion this replaced compared two members of
    /// the enum that were never the same value; what is checked here is that the
    /// value is read off that one fact and off nothing else.
    #[test]
    fn the_supervisor_unavailable_value_is_one_thing_and_has_two_answers() {
        for it_answered_a_request in [true, false] {
            let value = if it_answered_a_request {
                SupervisorUnavailable::ItAnsweredARequestOnTheCommandPath
            } else {
                SupervisorUnavailable::ItDidNotAnswer
            };
            let what_it_says = match value {
                SupervisorUnavailable::ItAnsweredARequestOnTheCommandPath => true,
                SupervisorUnavailable::ItDidNotAnswer => false,
            };
            assert_eq!(what_it_says, it_answered_a_request);
        }
    }
}
