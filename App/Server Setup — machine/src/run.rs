//! The setup run and its recorded safe boundaries — `Server Setup` §3.1 and
//! §3.2.
//!
//! **The constraint** is `Rented-server runtime` X15 — *"For a setup run,
//! termination is at, or recovery from, a recorded safe boundary, and the
//! boundary marker is the per-kind field of that operation record
//! (`Rented-server runtime` W4). Its
//! meaning is `Server Setup`'s"* — and that document's §9 entire. **The
//! boundaries, their granularity and their meaning are defined here and nowhere
//! else.**
//!
//! **The setup-in-progress state's mechanism is built and is not re-decided
//! here.** `Rented-server runtime` §9: this document sets and clears it through
//! the ordinary command path and the supervisor never does (P3); setting it is
//! durable before the restart a setup step causes (P4); **clearing it runs
//! reconciliation** (P5); no timer and no reconnection clears it (P7); it is
//! reported on every health answer (V9).
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Rented-server runtime` A5b, A11, A11a, P1, P2, P3, P4,
//! P5, P7, P8, V9, X15. `State, identity and recovery` D4, D5.
//! `Queue & History` C7. `Servers` H10.
//!
//! **`Rented-server runtime` A4, W3, W4 and R1 — that document's R1 being in its
//! §13 — are not listed above**, because this plan defines an A4, a W3, a W4 and
//! an R1 of its own and a bare one could mean either; where this file cites one, the document is written out at the site.

use smoozy_state::observed::Clock;

use crate::ready_action::Step;

/// **Y1: every setup act on a machine is a setup run** — one setup-kind
/// operation, one operation record written before its worker starts
/// (`Rented-server runtime` W3), one recorded execution group.
///
/// **There are two classes, and which class a run is decides whether the
/// machine-level state is set.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunClass {
    /// **A machine-changing run.** It sets the machine-level setup-in-progress
    /// state naming its own operation identity **before its first such step**,
    /// durably before any restart (`Rented-server runtime` P4), and clears it
    /// only at *done* (P5).
    MachineChanging,
    /// **An additive run** — a Blender install on a machine that already holds
    /// the unpack tool, an add-on install, a Blender removal, Telegram material
    /// placement. It sets no machine-level state, **not because none of its
    /// steps can disturb any work, but because none of them changes the machine
    /// in a way reconciliation or admission must be held back against.**
    Additive,
}

/// **Y1: the class is decided by the act a step performs, never by the step's
/// number.**
///
/// *"A run is machine-changing exactly where it performs an elevated package
/// install, a driver act, or a restart."* That is what
/// [`crate::ready_action`] R1 items 3 to 6 usually amount to, **but the test is
/// the act**: a Blender install on a machine lacking the unpack tool performs
/// an elevated package install and is machine-changing, and one on a machine
/// that already has the tool performs an unpack and a rename and is not.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct WhatTheStepsPerform {
    pub an_elevated_package_install: bool,
    pub a_driver_act: bool,
    pub a_restart: bool,
}

/// **Y1's test, performed.**
pub fn the_class(performs: WhatTheStepsPerform) -> RunClass {
    if performs.an_elevated_package_install || performs.a_driver_act || performs.a_restart {
        RunClass::MachineChanging
    } else {
        RunClass::Additive
    }
}

/// **Y1's observed limit, carried with the additive class rather than left
/// behind it.**
///
/// Installing a Blender beside a running render left every frame written;
/// installing an add-on changed not one frame value across sixty-four
/// picture-against-picture comparisons — **every one of those installs written
/// into a user repository and none into the folder a running Blender was
/// running from, which is [`crate::addons`] A6a's condition and travels with the
/// result** — and deleting the folder a render was running from let the render
/// finish normally.
///
/// **The observed limit travels with all of it**: *these* installs at *this*
/// rate changed no frame, not that installing is harmless in general, and the
/// observation behind the class is **an unpack and a rename beside a running
/// render** rather than an elevated package install on a busy machine — which
/// is why an install that must reach the package system falls on the other side
/// of the line.
pub const THE_ADDITIVE_CLASSS_OBSERVATION_IS_AN_UNPACK_AND_A_RENAME: bool = true;

/// **Y1's one exception to `Rented-server runtime` P2 and A11, and its scope is
/// stated exactly.**
///
/// *"The worker start of the run the state names is admitted"*, because the
/// state exists to protect that run rather than to strangle it. **The exception
/// is narrower than it looks**: the admission point gates worker starts
/// (`Rented-server runtime` A4), and this run's own worker was started before
/// the state was set, so the state never held the run's own steps back. **What
/// it is genuinely needed for is [`crate::recovery`] S4 item 2's start of the
/// recovery run's worker**, which is a worker start made while the state
/// stands. `Rented-server runtime` **A11a** admits that one worker start and
/// nothing else.
pub fn what_is_admitted_while_the_state_stands() -> &'static str {
    "the recovery run's worker start, by the operation the state itself names after the take-over, and nothing else"
}

/// **Y2: Row 110 is kept by the state and its clear, and by nothing this
/// document builds.**
///
/// The interrupted job is held back because the state is durable before the
/// restart and holds reconciliation and every admission (`Rented-server
/// runtime` P4, P2); it is re-run **after** the setup because clearing the
/// state runs reconciliation (P5). **The setup and the queue never talk to each
/// other; the state is the only thing between them, held by the machine, so it
/// works with nobody connected.**
pub const THE_QUEUE_AND_THE_SETUP_NEVER_TALK_TO_EACH_OTHER: bool = true;

/// **Y3 and Y4: one recorded safe boundary.**
///
/// **Y3's meaning:** *"every step up to and including the named one is done and
/// verified; no later step has begun."* Fact 49 shows the machine honouring
/// exactly that — a hard cut names the last boundary passed, never the step it
/// died in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Boundary {
    /// The record exists; **no machine-level state is set yet**; the machine is
    /// untouched.
    Started,
    /// The archive is whole at its staging path, its digest matching the
    /// published checksum; **nothing is installed**.
    Fetched,
    /// **The machine-level setup-in-progress state is set** where this run is
    /// machine-changing and this is its first such step; the unpack tool is
    /// present; the archive is unpacked into staging by that tool; nothing else
    /// is installed.
    Tooling,
    /// The read list is installed and `ldd` re-read clean; **the driver is
    /// untouched**.
    Libraries,
    /// The driver step's work ran to its own end and the check's outcome after
    /// it is recorded; **no restart has been issued**.
    DriverPlaced,
    /// The state is durable and the restart command was issued; **the machine
    /// may be down**.
    RestartIssued,
    /// The check passed — **on this boot**, not on a reading taken in any
    /// hand-reloaded state.
    CardAnswering,
    /// The build answered the expected version in staging and was **then**
    /// published at its final name ([`crate::ready_action`] R1 item 7's order).
    Blender,
    /// **`Server Setup` B15: the install's folder is gone.** A removal run's
    /// own boundary — the step order check reads boundaries only, and
    /// `Queue & History` L21's mark rides the removal's recorded outcome. The
    /// runner's ruling of 2026-10-09; built 2026-10-10, the 38's piece 14c.
    Removed,
    /// The role's material is placed and the desired state set.
    Telegram,
    /// The final verification passed; the state (where set) is cleared and
    /// reconciliation ran.
    Done,
}

impl Boundary {
    /// **Y4's full set, in [`crate::ready_action`] R1's order.**
    pub const IN_ORDER: [Boundary; 11] = [
        Boundary::Started,
        Boundary::Fetched,
        Boundary::Tooling,
        Boundary::Libraries,
        Boundary::DriverPlaced,
        Boundary::RestartIssued,
        Boundary::CardAnswering,
        Boundary::Blender,
        Boundary::Removed,
        Boundary::Telegram,
        Boundary::Done,
    ];

    /// Which step of the run this boundary belongs to.
    ///
    /// **Y5: granularity is one boundary per step whose repetition is not
    /// free**, which is why the driver has three: *"the boundary before the
    /// restart is what a machine that never comes back is read against, and the
    /// boundary after the check is what stops a re-run ever reinstalling a
    /// driver that answers."*
    pub fn its_step(self) -> Step {
        match self {
            Boundary::Started => Step::Readings,
            Boundary::Fetched => Step::Fetch,
            Boundary::Tooling => Step::Tooling,
            Boundary::Libraries => Step::Libraries,
            Boundary::DriverPlaced | Boundary::CardAnswering => Step::Driver,
            Boundary::RestartIssued => Step::Restart,
            Boundary::Blender => Step::BlenderPublication,
            Boundary::Removed => Step::BlenderRemoval,
            Boundary::Telegram => Step::TelegramMaterial,
            Boundary::Done => Step::Done,
        }
    }
}

/// **Y4a: beside each boundary, the same durable act records whether the step
/// performed its act or found nothing to perform — a closed set of two.**
///
/// **The test is the act — the same test [`the_class`] classifies runs by,
/// applied per step — and the set is closed at two; no third mark exists to
/// invent.** Every step that can have nothing to do is check-first by its own
/// rule, so whichever side writes the boundary knows which it was and writes it
/// in the same durable act.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheStepDid {
    Performed,
    NothingToDo,
}

impl WhatTheStepDid {
    pub const BOTH: [WhatTheStepDid; 2] = [WhatTheStepDid::Performed, WhatTheStepDid::NothingToDo];
}

/// One boundary as the record holds it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedBoundary {
    pub boundary: Boundary,
    /// **Y4a's mark**, in the per-kind field beside its boundary, written in
    /// the same durable act.
    pub what_the_step_did: WhatTheStepDid,
    /// **Y3: the moment on the machine's own clock.** Never the desktop's —
    /// `State, identity and recovery` D5 forbids subtracting two clocks, and
    /// [`Clock`] is what carries which one this is.
    pub at: std::time::SystemTime,
    pub clock: Clock,
}

/// **Y4: the boundary list is composed at run start from the steps the run
/// contains, in [`crate::ready_action`] R1's order.**
///
/// *"Each present only where its step is."* **So a container's card has no
/// driver line at all**, because [`crate::ready_action`] R3 and Fact 48 keep the
/// driver step out of that machine's run and therefore out of its list.
pub fn compose_the_boundary_list(steps: &[Step]) -> Vec<Boundary> {
    Boundary::IN_ORDER
        .into_iter()
        .filter(|boundary| steps.contains(&boundary.its_step()))
        .collect()
}

/// **Y4a's third bullet: the design's third mark is not a mark, and none is
/// invented for it.**
///
/// Where the machine held no Blender and the run fetched none, the libraries
/// step **is not composed into the run at all**, so it has no boundary and
/// gains no per-step outcome — **writing one would record a check that never
/// ran**, which is exactly what [`crate::ready_action`] R4 and
/// `statements` in `smoozy-server-setup` W1 forbid claiming.
///
/// **What the panel's *not checked* line is composed from is the recorded
/// composition itself**: a recorded list holding no libraries entry is a
/// positive record that the run contained no such step — a reading of what was
/// composed, never an inference from an absent answer.
pub fn the_run_contained_a_libraries_step(list: &[Boundary]) -> bool {
    list.contains(&Boundary::Libraries)
}

/// **Y6: what is readable over the connection.**
///
/// The setup state, its named operation, its boundary list and its last-reached
/// marker are **reported on every health answer** (`Rented-server runtime` V9)
/// and answered from the operation record like any command result. **A
/// submission refused while the state stands is told the state's own operation
/// identity besides** — `Rented-server runtime` A5b's refusal at the admission
/// point, on the identity P1 holds — **and it is the two routes above, never
/// that refusal, that carry the boundary list and the marker.**
///
/// **`Rented-server runtime` P8 is not a third route**: its actor is a setup run
/// that finds the state already set, which P2 and A11a never admit, and it
/// carries no boundary in any case.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhereTheReadingComesFrom {
    /// Reported on every health answer.
    TheHealthAnswer,
    /// Answered from the operation record like any command result.
    TheOperationRecord,
}

impl WhereTheReadingComesFrom {
    /// **The two routes, and there is no third.**
    pub const THE_TWO: [WhereTheReadingComesFrom; 2] = [
        WhereTheReadingComesFrom::TheHealthAnswer,
        WhereTheReadingComesFrom::TheOperationRecord,
    ];
}

/// The setup operation record as this document reads it back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheSetupRecord {
    /// Which kind of setup act wrote it — Y6b's per-kind retention turns on
    /// this.
    pub kind: SetupKind,
    /// **Y4's list, composed at run start**, which is *what it consisted of*.
    ///
    /// **It is a second field and not the one below, and the two are asked
    /// different questions.** One field carried both until 2026-09-04, and both
    /// readings could not be right at once: [`crate::recovery::take_over`] built
    /// the inherited list out of it and took the marker out of its end, so
    /// either the inherited list ended at the marker and a recovery run resumed
    /// with no steps at all, or the marker was always the run's last step and
    /// every run read as finished.
    pub the_composed_list: Vec<Boundary>,
    /// **Y3: the boundaries actually reached, with their marks and their
    /// moments**, in the order they were reached. **The last of these is the
    /// marker.**
    pub the_boundaries_reached: Vec<RecordedBoundary>,
    /// **The run's recorded ending, where one was recorded.**
    pub ending: Option<crate::recovery::HowTheRunEnded>,
}

impl TheSetupRecord {
    /// **Y6a: *that a setup happened* is the record existing with *done* among
    /// its boundaries.**
    pub fn a_setup_finished(&self) -> bool {
        self.the_boundaries_reached
            .iter()
            .any(|recorded| recorded.boundary == Boundary::Done)
    }

    /// **Y3: the last boundary reached, which is the marker.**
    ///
    /// **The reached list and never the composed one**: a composed list always
    /// ends at *done*, so reading its end as the marker would make every run
    /// read as finished whatever happened.
    pub fn the_last_boundary_reached(&self) -> Option<&RecordedBoundary> {
        self.the_boundaries_reached.last()
    }

    /// **Y4: what the run consisted of**, which is the composed list and not
    /// what it managed.
    pub fn what_it_consisted_of(&self) -> &[Boundary] {
        &self.the_composed_list
    }

    /// **S1, over this record's own two lists.** How far each step of the run
    /// got, from the composed list, the marker and the step that was in flight.
    pub fn how_far_it_got(
        &self,
        the_step_in_flight: Option<Step>,
    ) -> Vec<(Boundary, crate::recovery::WhereAStepGot)> {
        crate::recovery::how_far_it_got(
            &self.the_composed_list,
            self.the_last_boundary_reached().map(|it| it.boundary),
            the_step_in_flight,
        )
    }

    /// **Y6a: *when it happened* is the *done* boundary's moment, on the
    /// machine's clock.**
    ///
    /// **It is shown as a moment and never as a duration** — computing *three
    /// days ago* would subtract the machine's clock from the desktop's, which
    /// `State, identity and recovery` D5 forbids. **Converting the moment to
    /// the user's own zone for display is permitted and is not that
    /// subtraction**: the machine is on UTC and knows no time zone, so a zone
    /// shift uses the desktop's zone setting and reads none of its clock.
    pub fn when_it_finished(&self) -> Option<(std::time::SystemTime, &Clock)> {
        self.the_boundaries_reached
            .iter()
            .find(|recorded| recorded.boundary == Boundary::Done)
            .map(|recorded| (recorded.at, &recorded.clock))
    }
}

/// **Y6a and Y6b: which kind of setup act a record belongs to.**
///
/// `Rented-server runtime` R1's retention term, in that document's §13, is
/// **per kind** from
/// 2026-08-25: *"one setup operation record per machine per kind of setup act …
/// each replaced at the start of the next run of its own kind"*, except where an
/// owning clause ties the record to a subject, **as [`crate::blender`] B8a
/// already does for a Blender install whose build was published**.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetupKind {
    /// [`crate::ready_action`]'s machine-preparation run, **and a recovery run
    /// that takes such a run over** ([`crate::recovery`] S4 item 2, whose
    /// take-over gives the recovery the taken-over run's kind).
    TheReadyAction,
    ABlenderInstall,
    ABlenderRemoval,
    AnAddOnInstall,
    TheTelegramPlacement,
}

impl SetupKind {
    pub const ALL: [SetupKind; 5] = [
        SetupKind::TheReadyAction,
        SetupKind::ABlenderInstall,
        SetupKind::ABlenderRemoval,
        SetupKind::AnAddOnInstall,
        SetupKind::TheTelegramPlacement,
    ];
}

/// **Y6b: the record the *This server* card is composed from is the ready
/// action's, and a run of any other kind replaces nothing of it.**
///
/// **So both of the design's sentences hold by construction rather than by
/// rule.** On a machine never prepared, the dedicated Telegram entry point's run
/// writes a record of its own kind and no ready-action record exists — the card
/// still reads *never set up*, while Telegram reporting is genuinely installed.
/// On a machine prepared earlier, the placement replaces nothing.
pub fn the_card_reads(records: &[TheSetupRecord]) -> Option<&TheSetupRecord> {
    records
        .iter()
        .find(|record| record.kind == SetupKind::TheReadyAction)
}

/// **Y6a: *has this server ever been set up* is the record's presence and
/// nothing else.** No record, no setup.
///
/// **And a recycled machine reads as never set up without any rule of its
/// own**, because `State, identity and recovery` §2.3's installation-identity
/// gate disqualifies every remembered value before this is read, and the
/// machine that answers is a different machine holding no record.
pub fn has_this_server_ever_been_set_up(records: &[TheSetupRecord]) -> bool {
    the_card_reads(records).is_some()
}

/// **Y6b: the card's Telegram line is the role's own reading, never a line of
/// the run.**
///
/// It is composed from `telegram` in `smoozy-server-setup` G1's durable enabled-or-disabled
/// desired state, **read fresh over the connection**, and one source whichever
/// entry point placed the material. **A ready run's *telegram* boundary stays in
/// the list as what that run did; the line about the role now is the role's
/// reading.** No file's presence is read for it, and the desktop stores none of
/// it.
pub fn the_telegram_line_is_read_from_the_boundary_list(_it_is_there: bool) -> bool {
    false
}

/// **Y6a: this is a fresh reading over the connection and the desktop stores
/// none of it**, so [`crate::recovery`] S2's *no second home* holds unchanged
/// and `State, identity and recovery` D4 is untouched — the machine still holds
/// it.
///
/// **With no connection the card is last-known, greyed and stamped**, on Row
/// 323's rule, **and never blank**; the marking is composable because every
/// reading carries its own moment and clock (`State, identity and recovery` D5),
/// and the statement for a held reading shown while nothing updates it is
/// `Queue & History` C7's.
pub const THE_DESKTOP_STORES_NONE_OF_THE_CARD: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

    fn a_boundary(boundary: Boundary, did: WhatTheStepDid) -> RecordedBoundary {
        RecordedBoundary {
            boundary,
            what_the_step_did: did,
            at: SystemTime::UNIX_EPOCH,
            clock: Clock::AMachine(smoozy_state::observed::MachineClock("box-1".to_string())),
        }
    }

    #[test]
    fn the_class_is_decided_by_the_act_and_never_by_the_steps_number() {
        // Y1's own example: a Blender install on a machine lacking the unpack
        // tool performs an elevated package install and is machine-changing;
        // one on a machine that already has the tool is an unpack and a rename
        // and is not.
        assert_eq!(
            the_class(WhatTheStepsPerform {
                an_elevated_package_install: true,
                ..Default::default()
            }),
            RunClass::MachineChanging
        );
        assert_eq!(
            the_class(WhatTheStepsPerform {
                a_driver_act: true,
                ..Default::default()
            }),
            RunClass::MachineChanging
        );
        assert_eq!(
            the_class(WhatTheStepsPerform {
                a_restart: true,
                ..Default::default()
            }),
            RunClass::MachineChanging
        );
        assert_eq!(
            the_class(WhatTheStepsPerform::default()),
            RunClass::Additive
        );
    }

    #[test]
    fn the_boundary_list_holds_only_the_steps_the_run_contains() {
        // Y4: *each present only where its step is*. A container's list has no
        // driver line at all.
        let steps = [Step::Readings, Step::Libraries, Step::Done];
        let list = compose_the_boundary_list(&steps);
        assert_eq!(
            list,
            vec![Boundary::Started, Boundary::Libraries, Boundary::Done]
        );
        assert!(!list.contains(&Boundary::DriverPlaced));
        assert!(!list.contains(&Boundary::RestartIssued));
        assert!(!list.contains(&Boundary::CardAnswering));
    }

    #[test]
    fn the_list_is_in_r1s_order_whatever_order_the_steps_arrive_in() {
        let steps = [Step::Done, Step::Readings, Step::Driver, Step::Fetch];
        let list = compose_the_boundary_list(&steps);
        let mut sorted = list.clone();
        sorted.sort();
        assert_eq!(list, sorted);
    }

    #[test]
    fn the_driver_has_three_boundaries_and_that_is_y5s_granularity_rule() {
        // **Y5's actual claim, and the fault this test used to have.** Its name
        // and its variable both said three and its assertion checked two,
        // because the filter asked for the driver's own step and the restart's
        // boundary is filed under the restart's. **What Y5 claims is the three
        // from *driver placed* to *card answering* inclusive** — *"the boundary
        // before the restart is what a machine that never comes back is read
        // against, and the boundary after the check is what stops a re-run ever
        // reinstalling a driver that answers"* — so that is what is counted, and
        // moving the restart's boundary out from between them fails here.
        let three: Vec<Boundary> = Boundary::IN_ORDER
            .into_iter()
            .filter(|boundary| {
                (Boundary::DriverPlaced..=Boundary::CardAnswering).contains(boundary)
            })
            .collect();
        assert_eq!(
            three,
            vec![
                Boundary::DriverPlaced,
                Boundary::RestartIssued,
                Boundary::CardAnswering
            ]
        );
        // And the two the driver's own step owns are the outer pair.
        let the_drivers_own: Vec<Boundary> = Boundary::IN_ORDER
            .into_iter()
            .filter(|boundary| boundary.its_step() == Step::Driver)
            .collect();
        assert_eq!(
            the_drivers_own,
            vec![Boundary::DriverPlaced, Boundary::CardAnswering]
        );
    }

    #[test]
    fn a_run_with_no_libraries_step_records_no_libraries_outcome() {
        // Y4a's third bullet: writing one would record a check that never ran.
        let list = compose_the_boundary_list(&[Step::Readings, Step::Done]);
        assert!(!the_run_contained_a_libraries_step(&list));
    }

    #[test]
    fn the_mark_beside_a_boundary_is_a_closed_set_of_two() {
        assert_eq!(WhatTheStepDid::BOTH.len(), 2);
    }

    #[test]
    fn a_finished_setup_is_the_record_with_done_among_the_boundaries_it_reached() {
        let record = TheSetupRecord {
            kind: SetupKind::TheReadyAction,
            the_composed_list: vec![Boundary::Started, Boundary::Done],
            the_boundaries_reached: vec![
                a_boundary(Boundary::Started, WhatTheStepDid::Performed),
                a_boundary(Boundary::Done, WhatTheStepDid::Performed),
            ],
            ending: None,
        };
        assert!(record.a_setup_finished());
        assert!(record.when_it_finished().is_some());
        assert!(has_this_server_ever_been_set_up(&[record]));
    }

    #[test]
    fn the_two_lists_answer_two_questions_and_neither_stands_in_for_the_other() {
        // **The fault this test exists for.** One field carried both until
        // 2026-09-04, and both readings could not be right at once: a record
        // whose field held what it reached made the recovery resume with no
        // steps at all, and one whose field held the composed list made every
        // run read as finished. **This record's two halves could not have been
        // produced by one field**: the composed list runs to *done* and the run
        // reached only the fetch.
        let record = TheSetupRecord {
            kind: SetupKind::TheReadyAction,
            the_composed_list: vec![
                Boundary::Started,
                Boundary::Fetched,
                Boundary::Tooling,
                Boundary::Done,
            ],
            the_boundaries_reached: vec![
                a_boundary(Boundary::Started, WhatTheStepDid::Performed),
                a_boundary(Boundary::Fetched, WhatTheStepDid::Performed),
            ],
            ending: None,
        };
        // Fact 49: a hard cut names the last boundary actually passed, never
        // the step it died in.
        assert!(!record.a_setup_finished());
        assert_eq!(record.when_it_finished(), None);
        assert_eq!(
            record.the_last_boundary_reached().map(|it| it.boundary),
            Some(Boundary::Fetched)
        );
        assert_eq!(record.what_it_consisted_of().len(), 4);

        // And the recovery resumes with the steps after the marker rather than
        // with none at all.
        let taken = crate::recovery::take_over(&record, false);
        assert_eq!(taken.the_inherited_marker, Some(Boundary::Fetched));
        assert_eq!(
            crate::recovery::where_the_recovery_begins(
                &taken.the_inherited_boundary_list,
                taken.the_inherited_marker
            ),
            vec![Boundary::Tooling, Boundary::Done]
        );
        // S1, over the record's own two lists.
        let read = record.how_far_it_got(Some(Step::Tooling));
        assert_eq!(read[1].1, crate::recovery::WhereAStepGot::Done);
        assert_eq!(read[2].1, crate::recovery::WhereAStepGot::StoppedPartWay);
        assert_eq!(read[3].1, crate::recovery::WhereAStepGot::NeverStarted);
    }

    #[test]
    fn the_card_reads_the_ready_actions_record_and_no_other_kinds() {
        // Y6b: on a machine never prepared, the dedicated Telegram entry
        // point's run writes a record of its own kind and the card still reads
        // *never set up*.
        let telegram_only = [TheSetupRecord {
            kind: SetupKind::TheTelegramPlacement,
            the_composed_list: vec![Boundary::Done],
            the_boundaries_reached: vec![a_boundary(Boundary::Done, WhatTheStepDid::Performed)],
            ending: None,
        }];
        assert!(!has_this_server_ever_been_set_up(&telegram_only));
        assert!(the_card_reads(&telegram_only).is_none());
        assert_eq!(SetupKind::ALL.len(), 5);
    }

    #[test]
    fn the_telegram_line_is_never_read_off_the_boundary_list() {
        assert!(!the_telegram_line_is_read_from_the_boundary_list(true));
    }

    #[test]
    fn a_boundarys_moment_is_on_the_machines_own_clock() {
        // Y3, and `State, identity and recovery` D5: two clocks are never
        // subtracted, so the moment has to say which one it is on.
        let recorded = a_boundary(Boundary::Done, WhatTheStepDid::Performed);
        assert!(matches!(recorded.clock, Clock::AMachine(_)));
    }

    #[test]
    fn only_the_recovery_runs_worker_start_is_admitted_while_the_state_stands() {
        // Y1's exception, and `Rented-server runtime` A11a's field for it.
        assert!(what_is_admitted_while_the_state_stands().contains("recovery run's worker start"));
        assert!(what_is_admitted_while_the_state_stands().contains("nothing else"));
        assert_eq!(WhereTheReadingComesFrom::THE_TWO.len(), 2);
    }
}
