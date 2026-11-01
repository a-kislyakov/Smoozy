//! The render kind — §3.1 K1–K3 (the declarations), §3.2 K4–K7 (the
//! termination contract), §3.4 K12 (a dropped connection is not a stop) and
//! §3.5 K13 (the worker contract) — **every one a declaration decided at no
//! run time.**
//!
//! **K1: the render kind's declarations, against `Rented-server runtime`
//! A1, none decided at run time.** `Rented-server runtime` A1 takes five things: the resource
//! class — the cards **exclusive**, the processors and the machine's memory
//! **shared**, the cards exclusive whether or not the job uses them, because
//! a class decided per job is a class two callers decide differently;
//! priority first, above every other kind; cancellable, yes, at any moment;
//! atomic or partial, §7's table; and machine-level exclusion of its own
//! kind, **which this kind declares nothing for and `Rented-server runtime`
//! A1 reads as unlimited**.
//!
//! **K4: the termination is `SIGINT` — the interrupt — delivered to the
//! recorded render process: never the worker, never a shell, never anything
//! found by name, pattern or process search.** An interrupt aimed at the
//! instant the frame's own file appears finishes the write and leaves the
//! frame whole, while a polite terminate and an outright kill each leave a
//! short file at the frame's own final name (*Frames 13*) — a short file
//! sitting at exactly the finished frame's name is *present*, and presence
//! is what the already-there test reads, so a killed job leaves a frame
//! that is never rendered. **For a video the difference is total.** Rows 16
//! and 17 kept together.
//!
//! **K6: nothing matches an ending on its number** (`Rented-server runtime`
//! X13): a polite terminate is recorded as `-15` by one topic and `143` by
//! three others, and the classification reads the reaped status as a
//! recorded number and never as a name for a cause. Held by
//! [`a_cause_read_from_a_number`] having one answer.
//!
//! **Markers used in this file that belong to another document, each with
//! the document it belongs to.** `Rented-server runtime` X1, X6, X8, X9,
//! X11, X12, X13, W6, W12, M1, M3, M4, M28, R1. `Job files and outputs` C7,
//! C27, C34. `Queue & History` Q4, E6, E13, E13a, R1, R5, R6, H7, P1b.
//! `State, identity and recovery` D2. `Telegram` D1.
//!
//! **`Rented-server runtime` A1, W3, S1 and K18, `Queue & History` A9, P1a
//! and L13, and `Servers` K1 are not listed above and are written out at
//! each site**, because this plan defines a clause of each name.

use std::time::Duration;

/// One of `Rented-server runtime` A1's five things, in this kind's own terms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Declared {
    pub declaration: &'static str,
    pub value: &'static str,
}

/// **K1: the five declarations.**
pub const THE_DECLARATIONS: [Declared; 5] = [
    Declared {
        declaration: "resource class",
        value: "the cards exclusive, whether or not the job uses them; the processors and the machine's memory shared",
    },
    Declared {
        declaration: "priority",
        value: "first, above every other kind",
    },
    Declared {
        declaration: "cancellable",
        value: "yes, at any moment, by the termination contract",
    },
    Declared {
        declaration: "atomic or partial",
        value: "§7's table — one render attempt is one ordered unit",
    },
    Declared {
        declaration: "exclusion",
        value: "nothing declared, which `Rented-server runtime` A1 reads as unlimited — a machine may run as many renders at once as its cards admit, which the cards-exclusive class already decides",
    },
];

/// **K1 in `Rented-server runtime` A1's own vocabulary**: which resource is taken how, and the
/// rank. Compared against the runtime crate's own row for this kind in the
/// test below rather than asserted here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InA1sTerms {
    pub the_cards_exclusively: bool,
    pub the_processors_shared: bool,
    pub the_machines_memory_shared: bool,
    pub priority_first: bool,
    pub cancellable_at_any_moment: bool,
    pub excludes_its_own_kind: bool,
}

pub const IN_A1S_TERMS: InA1sTerms = InA1sTerms {
    the_cards_exclusively: true,
    the_processors_shared: true,
    the_machines_memory_shared: true,
    priority_first: true,
    cancellable_at_any_moment: true,
    excludes_its_own_kind: false,
};

/// **K2: the render worker outlives every member of its group — X11's
/// fourth thing, declared outright.** The worker remains until its external
/// processes have exited, or is terminated only after them. That is what
/// makes X1's leader-only recording and X9's leader-gone-means-group-gone
/// sound for this kind, what keeps the capture alive to the render's last
/// byte, and what lets the ending step run after a stop.
pub const THE_WORKER_OUTLIVES_EVERY_MEMBER_OF_ITS_GROUP: bool = true;

/// **K3: Fact 2 is answered yes on both halves on both machine kinds,
/// against a real render, and the row that would have been the second
/// escalation is spent.** The interrupt reached Blender itself and never a
/// shell, under all three signals, on both boxes. **The question is not
/// open and this says so rather than leaving the row reading as though it
/// were.**
pub const FACT_2_IS_ANSWERED_AND_THE_SECOND_ESCALATION_IS_SPENT: bool = true;

/// **K4: the termination, and its target.** The signal is named as the
/// interrupt; sending it is the acting half's, through the machine's own
/// call, to the recorded render process's identity paired with its
/// kernel-set start time (X1, X12).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheTermination {
    /// The signal's name. **The interrupt and never the kill**: K4's whole
    /// ground is what each ending leaves at the frame's own file.
    pub signal: &'static str,
    /// The recorded render process — Blender's own, which L5's `exec`
    /// makes the worker's child — and nothing found later.
    pub target: &'static str,
}

pub const THE_TERMINATION: TheTermination = TheTermination {
    signal: "SIGINT",
    target: "the recorded render process, by the recorded execution-group pair; never the worker, never a shell, never anything found by name, pattern or process search",
};

/// **X11's four named things, answered for this kind** (K2, K4, K7).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheTerminationContract {
    pub which_member_receives_the_termination: &'static str,
    pub at_any_moment: bool,
    pub the_cleanup_owed: &'static str,
    pub the_worker_outlives_every_member: bool,
}

pub const THE_TERMINATION_CONTRACT: TheTerminationContract = TheTerminationContract {
    which_member_receives_the_termination: THE_TERMINATION.target,
    at_any_moment: true,
    // **K7: none beyond the ending step.** The render writes straight to
    // final names and grows in place — no staging to sweep and no
    // publication to unwind — and what a stop leaves on disk is the
    // frames, which are the user's.
    the_cleanup_owed: "none beyond the ending step — the worker reaps, takes the ending step, reports, and exits; the sinks and records are retained",
    the_worker_outlives_every_member: THE_WORKER_OUTLIVES_EVERY_MEMBER_OF_ITS_GROUP,
};

impl TheTerminationContract {
    pub fn names_all_four_of_x11s_things(&self) -> bool {
        !self.which_member_receives_the_termination.is_empty()
            && self.at_any_moment
            && !self.the_cleanup_owed.is_empty()
            && self.the_worker_outlives_every_member
    }
}

/// **K5: the stop takes the seven-step sequence and nothing shorter**
/// (`Rented-server runtime` X8; `State, identity and recovery` §5.3), with
/// the one boundary — the issuance — cited rather than re-decided: a worker
/// that ended on its own **before step 4's termination was issued** records
/// its own ending, and an ending past the issuance is recorded *left
/// stopped*.
pub const THE_SEVEN_STEPS: [&str; 7] = [
    "the stop's own command record, first",
    "the desired state *should be stopped* durable before the group is looked at",
    "the pair validated — a failed validation ends the sequence with no signal sent",
    "the interrupt to the recorded render process",
    "nothing inferred from delivery",
    "*left stopped* recorded when the group has ended",
    "a worker that ended on its own before step 4's issuance records its own ending; an ending past the issuance is *left stopped*",
];

/// **K5: the two callers, and the phone's stop is the same command.** The
/// user from the queue, and `Servers` on a stop-and-clear removal; the
/// phone's stop is the same command submitted by the owning machine's role
/// worker (`Telegram` D1). The paired stop-and-clear's target resolution is
/// `Queue & History` L13's.
pub const THE_CALLERS: [&str; 2] = [
    "the user, from the queue",
    "`Servers`, on a stop-and-clear removal — the phone's stop arriving as the same command by the machine's own role worker",
];

/// **K6: what reading a number for a cause gives — nothing.** There is one
/// answer and it is not a cause.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoCauseIsReadFromANumber {
    /// The number is recorded verbatim; it names nothing.
    TheNumberIsRecordedAndNamesNothing,
}

pub fn a_cause_read_from_a_number(_reaped: i32) -> NoCauseIsReadFromANumber {
    NoCauseIsReadFromANumber::TheNumberIsRecordedAndNamesNothing
}

/// **K12: no clause anywhere in this document fires for a lost connection,
/// and the promise is kept by the launch rather than by a rule.** The stop
/// exists only as a command with its own operation identity (K5); a
/// connection's end submits no command. **This is Row 19, and its cost when
/// got wrong is the user paying for frames a hang-up killed.**
pub const NO_CLAUSE_FIRES_FOR_A_LOST_CONNECTION: bool = true;

/// One obligation of the worker contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Obligation {
    pub what: &'static str,
    /// The shape `Queue & History` defines for it, named exactly as that
    /// crate's `arriving_later::THE_FIVE_OBSERVATION_SHAPES` names it.
    pub the_shape: &'static str,
    pub clauses: &'static str,
}

/// **K13: the render worker's obligations at its own copy loop, taken from
/// `Queue & History` §17 and carried as the kind's contract.** Six
/// obligations; the five observations are that crate's five shapes, filled
/// by this worker.
pub const THE_WORKER_CONTRACT: [Obligation; 6] = [
    Obligation {
        what: "the capture — both streams held and copied to the sinks, bytes counted at the loop, the marker and what follows it observed as the bytes pass",
        the_shape: "`Rented-server runtime` M3, M4, M28's capture",
        clauses: "`Rented-server runtime` M3, M4, M28",
    },
    Obligation {
        what: "the live readings — the reader rides the loop as a tee, and the worker reports its current readings to the supervisor at its own bounded interval, re-established across a supervisor restart by W12's route",
        the_shape: "smoozy_queue_history::readings::AWorkersReport carrying readings::LiveReadings",
        clauses: "`Queue & History` R1, R5, R6",
    },
    Obligation {
        what: "the ending-step completeness reading — every outcome the worker reports carries the completeness statement taken as a step of that ending, a stopped render included",
        the_shape: "smoozy_queue_history::waiting::CompletenessReading",
        clauses: "`Queue & History` H7; `Job files and outputs` C7, C27",
    },
    Obligation {
        what: "the frame's save announcement — observed in passing on the version's own grammar, with the newest frame of the attempt's list whose save has been announced reported on the live readings' own bounded reporting",
        the_shape: "smoozy_queue_history::readings::AWorkersReport::newest_frame_saved",
        clauses: "`Queue & History` P1a",
    },
    Obligation {
        what: "the device line — observed in passing on the version's own stream, recorded in the attempt record",
        the_shape: "smoozy_queue_history::readings::ObservedDeviceLine",
        clauses: "L26; `Queue & History` E13",
    },
    Obligation {
        what: "the colour-substitution line — observed in passing on both streams with no stream asserted, recorded like the device line's; no ask and no switch produces it",
        the_shape: "smoozy_queue_history::readings::ObservedColourSubstitutionLine",
        clauses: "`Queue & History` E13a",
    },
];

/// **K13's third bullet, and a contradiction in the plan recorded at the
/// site with both sentences quoted, built to the later and more specific
/// one.** K13 says: *"The reconciliation-written half of H7 was owed by
/// closed documents and is now answered — `Rented-server runtime` S11 to
/// S13, filed by Row 310 on 2026-08-21, which closes `Queue & History` §18
/// item 17."* §8's paragraph to `Queue & History` still says: *"The
/// reconciliation half of H7 remains the seam its §18 item 17 records."*
/// Both stand in the file. **This crate builds to K13** — the worker's half
/// is the observation and the report, and the reconciliation-written half
/// is `Rented-server runtime` S11–S13's, already answered — and the
/// contradiction is reported to the runner as a plan fault to route, not
/// repaired here.
pub const THE_RECONCILIATION_HALF_OF_H7_IS_ANSWERED_AT_RENTED_SERVER_RUNTIME_S11_TO_S13: bool =
    true;

/// **K9's neighbour, stated here because K13's reporting rides it**: the
/// worker's report is re-attempted against the endpoint's recorded path
/// until accepted, carrying the run identifier it was started under, so it
/// survives a supervisor restart (`Rented-server runtime` M6, W12).
pub const THE_REPORT_IS_RE_ATTEMPTED_UNTIL_ACCEPTED: bool = true;

/// **The bounded interval the live readings are reported at is the worker's
/// own** and nothing here names a number for it — `Queue & History` R5's
/// *its own bounded interval* is the whole of the rule, and a figure
/// written here would be invented.
pub const THE_REPORTING_INTERVAL_IS_THE_WORKERS_OWN_AND_BOUNDED: Option<Duration> = None;

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_runtime::arbitration::{
        the_two_declarations_already_given, Cancellability, How, Resource,
    };
    use smoozy_runtime::workers::WorkerKind;

    #[test]
    fn the_five_declarations_are_k1s() {
        let names: Vec<&str> = THE_DECLARATIONS.iter().map(|row| row.declaration).collect();
        assert_eq!(
            names,
            vec![
                "resource class",
                "priority",
                "cancellable",
                "atomic or partial",
                "exclusion"
            ]
        );
        assert!(THE_DECLARATIONS[0].value.contains("cards exclusive"));
        assert!(THE_DECLARATIONS[1].value.starts_with("first"));
        assert!(THE_DECLARATIONS[2].value.starts_with("yes, at any moment"));
        assert!(THE_DECLARATIONS[4].value.contains("unlimited"));
    }

    /// **K1 read against the runtime crate's own row for the render kind**
    /// — that crate's A2 already carries the render's card class as
    /// exclusive, and the two must agree value for value.
    #[test]
    fn the_runtime_names_this_document_as_the_owner_and_its_own_row_agrees_with_k1() {
        assert_eq!(WorkerKind::Render.declarations_owned_by(), &["Rendering"]);
        let theirs = the_two_declarations_already_given(WorkerKind::Render).unwrap();
        let takes = |resource: Resource| {
            theirs
                .resources
                .iter()
                .find(|(r, _)| *r == resource)
                .map(|(_, how)| *how)
        };
        assert_eq!(takes(Resource::Cards), Some(How::Exclusively));
        assert_eq!(takes(Resource::Processors), Some(How::Shared));
        assert_eq!(takes(Resource::MachineMemory), Some(How::Shared));
        assert_eq!(theirs.priority, 0);
        assert_eq!(theirs.cancellability, Cancellability::AtAnyMoment);
        assert!(!theirs.excludes_its_own_kind);
        assert_eq!(
            IN_A1S_TERMS,
            InA1sTerms {
                the_cards_exclusively: takes(Resource::Cards) == Some(How::Exclusively),
                the_processors_shared: takes(Resource::Processors) == Some(How::Shared),
                the_machines_memory_shared: takes(Resource::MachineMemory) == Some(How::Shared),
                priority_first: theirs.priority == 0,
                cancellable_at_any_moment: theirs.cancellability == Cancellability::AtAnyMoment,
                excludes_its_own_kind: theirs.excludes_its_own_kind,
            }
        );
    }

    #[test]
    fn the_termination_is_the_interrupt_to_the_recorded_render_process_and_never_the_kill() {
        assert_eq!(THE_TERMINATION.signal, "SIGINT");
        assert!(THE_TERMINATION.target.contains("recorded render process"));
        assert!(THE_TERMINATION.target.contains("never the worker"));
        assert!(THE_TERMINATION_CONTRACT.names_all_four_of_x11s_things());
        assert!(THE_TERMINATION_CONTRACT
            .the_cleanup_owed
            .starts_with("none beyond the ending step"));
        const _: () = assert!(THE_WORKER_OUTLIVES_EVERY_MEMBER_OF_ITS_GROUP);
        const _: () = assert!(FACT_2_IS_ANSWERED_AND_THE_SECOND_ESCALATION_IS_SPENT);
    }

    #[test]
    fn the_stop_is_seven_steps_and_the_boundary_is_the_issuance() {
        assert_eq!(THE_SEVEN_STEPS.len(), 7);
        assert!(THE_SEVEN_STEPS[0].contains("command record"));
        assert!(THE_SEVEN_STEPS[1].contains("durable before"));
        assert!(THE_SEVEN_STEPS[2].contains("no signal sent"));
        assert!(THE_SEVEN_STEPS[3].contains("interrupt"));
        assert!(THE_SEVEN_STEPS[6].contains("issuance"));
        assert_eq!(THE_CALLERS.len(), 2);
    }

    /// **K6, held**: every number gives the one answer, and it is not a
    /// cause — 143 and −15 for the same terminate, 137, 134, 135 and 139
    /// from unrelated events.
    #[test]
    fn no_number_names_a_cause() {
        for reaped in [0, 1, 2, 63, 134, 135, 137, 139, 143, -9, -15] {
            assert_eq!(
                a_cause_read_from_a_number(reaped),
                NoCauseIsReadFromANumber::TheNumberIsRecordedAndNamesNothing
            );
        }
        const _: () = assert!(NO_CLAUSE_FIRES_FOR_A_LOST_CONNECTION);
    }

    /// **K13's five observations name the five shapes `Queue & History`
    /// defines, read off that crate's own list rather than remembered.**
    #[test]
    fn the_worker_contracts_five_observations_are_queue_and_historys_five_shapes() {
        use smoozy_queue_history::arriving_later::THE_FIVE_OBSERVATION_SHAPES;
        assert_eq!(THE_WORKER_CONTRACT.len(), 6);
        let observations: Vec<&Obligation> = THE_WORKER_CONTRACT.iter().skip(1).collect();
        assert_eq!(observations.len(), THE_FIVE_OBSERVATION_SHAPES.len());
        for (ours, theirs) in observations.iter().zip(THE_FIVE_OBSERVATION_SHAPES.iter()) {
            let their_type = theirs
                .the_shape
                .split(|c: char| !c.is_alphanumeric() && c != '_' && c != ':')
                .find(|word| word.starts_with("crate::"))
                .unwrap()
                .trim_start_matches("crate::");
            assert!(
                ours.the_shape.contains(their_type),
                "{} does not name {their_type}",
                ours.what
            );
            for clause in theirs.clauses {
                assert!(
                    ours.clauses.contains(clause),
                    "{} does not cite {clause}",
                    ours.what
                );
            }
        }
        const _: () =
            assert!(THE_RECONCILIATION_HALF_OF_H7_IS_ANSWERED_AT_RENTED_SERVER_RUNTIME_S11_TO_S13);
        const _: () = assert!(THE_REPORT_IS_RE_ATTEMPTED_UNTIL_ACCEPTED);
        assert_eq!(THE_REPORTING_INTERVAL_IS_THE_WORKERS_OWN_AND_BOUNDED, None);
    }
}
