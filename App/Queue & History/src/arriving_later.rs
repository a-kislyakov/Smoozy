//! What of this document is not built here, named clause by clause rather
//! than silently absent — and the five observation shapes `Rendering`'s
//! worker fills, named exactly.
//!
//! **`Preparation/Build/Plan.md` step 7 builds this module.** Every clause of
//! the plan that is a judgement, a data shape, a composition or arithmetic has
//! its code in the files beside this one; what is left is the half that acts —
//! a durable store written, a command travelled, a stream teed, a machine
//! read, a derivative fetched — and each piece of it is named below with the
//! clause it belongs to, so nothing is silently absent.
//!
//! **EVERY ONE OF THE SIXTEEN HAS LANDED**, at [`crate::acting`], and
//! **[`WHAT_HAS_LANDED`] is the list, entry by entry, with the module that
//! performs each** — a test reads it against this crate's own sources, so a
//! module renamed or removed fails here rather than leaving a stale claim.
//! **A section may appear on that list more than once**, where two modules
//! perform different halves of it; §1 and §4 both do.
//!
//! **What each entry still leaves to somebody else is named in its own
//! `what` line**, and the two largest are unchanged: **the tee on the render
//! worker's copy loop is `Rendering`'s** (R1 makes this crate's reader a tee
//! on that loop rather than a second read), and **the preview worker's own
//! production is that worker's**. **Every machine-side act below is a function
//! the supervisor calls rather than a program that runs**: the program for the
//! rented machine is `smoozy-server`, at `App/Rented-server runtime —
//! program/`, and **since step 10c piece 5 it calls the machine-side acts of
//! §1 and §10** — the queue store's writes and reads, the job record and its
//! note in one act, the admission point's choice, the started and ended
//! records' state, the note's two writes that ride an attempt and its
//! outcome, and a released job's placement in the line — **and its render
//! worker runs this module's reader as the tee on `Rendering`'s copy loop.**
//! The other acts on the list — a cancel, a reorder, the clear, the previews,
//! the machine readings, the words — are still called by nothing it runs.
//! *(Not true of all of them since: the cancel and the clear are called from
//! step 10c's first stop-and-clear piece, and the reorder and the block move
//! from 2026-10-04, the 38's piece 8. Noted by that piece, which read no
//! further down the list.)*
//! *(Corrected 2026-09-19 by step 10c piece 5's second sitting: this said
//! nothing the program ran called any of them.)* *(Corrected
//! 2026-09-07: this said no such program existed anywhere in the workspace,
//! which stopped being true at step 8. Corrected again 2026-09-19: it said the
//! program depended on `smoozy-runtime`, `smoozy-framing` and `smoozy-state`
//! alone and so reached nothing here. At step 10c piece 3 the program took
//! `smoozy-queue-history-machine`, which holds this module's machine-side acts,
//! so it can reach them; it still reaches nothing of this crate's own.)*
//!
//! **What a reader of that list should not take it to mean, said plainly.**
//! Where a machine or the desktop's own local record is genuinely on the far
//! side, the act is behind a trait of this crate's, and **no such trait has an
//! implementation outside this crate's own tests** —
//! [`crate::acting::adoption::AsksAMachineForItsQueue`],
//! [`crate::acting::machine_readings::TakesTheEntrysReadings`],
//! [`crate::acting::words::ReadsTheSinkOverTheConnection`] and
//! [`crate::acting::submission::SendsTheSubmission`] on the machine side, and
//! [`crate::acting::catching_up::KeepsTheCatchUpHalf`],
//! [`crate::acting::submission::WritesTheProvisionalRecord`] and
//! [`crate::acting::adoption::WritesAdoptedJobs`] on the desk side.
//! *(Corrected 2026-10-03, the 38's piece 1: the list named
//! [`crate::acting::idle_recheck::ReadsTheMachineFresh`] and
//! [`crate::acting::idle_recheck::KeepsTheIdleMarks`] too, and both are
//! implemented now by the desktop, at its `src/servers/the_idle_recheck.rs`.
//! The rest of this paragraph was not re-checked then and may be as stale.)*
//! *(Corrected again 2026-10-03, the Creator's notes on 0.1.8, piece 6d:
//! [`crate::acting::words::ReadsTheSinkOverTheConnection`] is implemented by
//! the desktop, at its `src/servers/the_words_kept.rs`, which hands it to
//! [`crate::acting::words::fetch_the_words`]; and the machine's program now
//! calls `the_capture_records_as_fields` from the same file when it answers
//! the note reading.)* *(Corrected again 2026-10-04, the 38's piece 15:
//! [`crate::acting::catching_up::KeepsTheCatchUpHalf`] is implemented by the
//! desktop, at its `src/record/catch_up_notices.rs`, over the tables record
//! version 38 made, and its *Dismiss* goes through
//! [`crate::acting::catching_up::dismiss`]; the desktop's read forward writes
//! the notice record before it moves the cursor, through the same store.)*
//! *(Corrected again 2026-10-16, owed piece 10:
//! [`crate::acting::adoption::WritesAdoptedJobs`] is implemented by the
//! desktop, at its `src/servers/the_machines_work.rs`, which hands it to
//! [`crate::acting::adoption::accept`] when *Track it* is pressed, and writes
//! every adopted record in one transaction of its `src/record/job_records.rs`;
//! [`crate::acting::adoption::AsksAMachineForItsQueue`] has been implemented
//! in the same desktop file since step 10c piece 7.)* **That is
//! the same shape the rules half's [`crate::readings::Readings`] already has
//! and the same shape `Rendering`'s desk side has**: the connection is
//! `Servers`', the local record is `Desktop app and delivery`'s, and neither
//! is wired to a surface by any crate yet. **So every act below is built and
//! proved on values, and none of it has met a machine.**
//!
//! **Nothing on the list is stubbed.** There is no always-yes answer, no
//! empty value standing in for one, and no default anywhere. Where a fact
//! the plans do not establish would otherwise have been written from memory,
//! the absence is a value at its own site — the colour lines' printed text
//! at [`crate::ending::ThePrintedText`], the tile threshold that
//! [`crate::bars::the_grid_tile_count`] leaves no space for, and the two
//! uninhabited types [`crate::bars::ACountdownToTheEndOfAJob`] and
//! [`crate::idle::WhenAMachineWillFallQuiet`].
//!
//! **Every clause named below is checked against the plan** at the test,
//! which reads the plan rather than trusting this page.

/// **What of the acting half has landed, and where.**
///
/// Each names an entry of [`THE_ACTING_HALF`] by the plan's own section, the
/// clauses of it that are now performed, and the module that performs them.
/// **Everything not named here stands as that list writes it.**
pub const WHAT_HAS_LANDED: [(&str, &str, &str); 17] = [
    (
        "§1",
        "Q1, Q3, Q6 and Q8 — the store, the record's publication, untracked \
         records, and the disk-full answer. The supervisor's own journalling \
         rides it and is the supervisor's.",
        "acting::queue_store",
    ),
    (
        "§1",
        "Q2's two queue-state writes — the started record's state written with \
         the render operation's identity in the same act the admission point \
         names it started, and the ended record's state written with the \
         recorded outcome's identity in the same act the note's outcome write \
         runs. Each names what is journalled and the journal is the \
         supervisor's. **Added 2026-09-05, out of a code round**: nothing \
         wrote either, so the admission point took the same job again, the \
         idle judgement read the line as holding startable work, and the \
         clear covered a record whose worker was running.",
        "acting::commands",
    ),
    (
        "§5",
        "R2, R7, R9, R10, R14 and R15 — the two grammars per version and the \
         parse of each kind of line. The tee itself and the bounded reporting \
         are `Rendering`'s copy loop's and stand.",
        "acting::reader",
    ),
    (
        "§9",
        "D1 to D13 — the disposition decided from durable records alone, the \
         attempt count advanced durably before the second attempt starts, the \
         re-run's explicit frame list taken from the reckoning, and the video \
         switch thrown before the render. D5, D8, D10 and D11 are grounds \
         rather than acts and are kept as rules and absences.",
        "acting::reconciliation",
    ),
    (
        "§10",
        "N1 to N9 — the per-job note as one durable record per job, created \
         with the job record and a null ending, each write riding the record \
         that makes its fact true, and N6's figure computed in the job's own \
         asked order. N9's recorded reading stays `Job files and outputs`' \
         store and is not copied in.",
        "acting::note",
    ),
    (
        "§4",
        "L1, L2, L6, L7, L10's queue side, L12, L13, L14, L21 and L22 — the \
         command records written durably before acting, their travel as one \
         named request, each rewrite written back in one serialized pass, the \
         record's removal and the material cleanup after it, the stop's \
         subject resolved and recorded at the clear's write, the mark riding \
         the removal's outcome, and the drop at the admission point.",
        "acting::commands",
    ),
    (
        "§3",
        "H2, H4 and H11 — the desk's answer composed as one command, the \
         serialized read-apply-write with H3's placement in the same act, the \
         release predicate supplied into the runtime's own evaluation, and \
         the drop applied by the same enumeration as a cancel.",
        "acting::held",
    ),
    (
        "§14",
        "X2 and X3 — the erase's eight enumerated units, its outcome record \
         opened before the first unit is attempted, an absent unit recorded \
         done, the tombstone's flag set only on all units done, and the \
         remainder composed from the record's own enumeration.",
        "acting::erase",
    ),
    (
        "§11",
        "C2, C3, C7 and C8 — the notice record written before the cursor \
         advances, the cursor's durable row against the installation it was \
         taken under, the period-held reading held in process memory and \
         dropped whole at the period's end, and the six survivors' stores.",
        "acting::catching_up",
    ),
    (
        "§17",
        "C1 — the read-forward both passes call, whose applied events the \
         account is composed from.",
        "acting::catching_up",
    ),
    (
        "§12",
        "W4, W5's write and W6 — the re-check scheduled from machine-clock \
         arithmetic, the claim re-taken from the machine before anything is \
         said, the mark written by the act of speaking alone, and the \
         telling composed for `Desktop app and delivery`'s own kind.",
        "acting::idle_recheck",
    ),
    (
        "§13",
        "A1 to A9 — the one request taken only after the installation \
         confirms, the supervisor-unavailable condition told apart from a \
         request lost inside a declared break, declined work said as a count, \
         the unfiltered offer less every tombstoned identity, and the \
         acceptance in one durable write.",
        "acting::adoption",
    ),
    (
        "§7",
        "G1 to G5a — the per-card query in the field spellings the project \
         has on record, the count counted rather than parsed, the disk figure \
         at the root's mount with the path it was taken against riding it, \
         both carried on the health-readings route with their own moment and \
         clock, and the two requests as a closed pair. The identity field's \
         own spelling is an absence rather than a guess.",
        "acting::machine_readings",
    ),
    (
        "§15",
        "P1, P1a's queueing, P1b's catch-up at the ending's write, P2's one \
         fetch in flight, P3 and P5's read-forward trigger, P4's fetch \
         arranged for the transfer contract's own loop, and P10a's heading. \
         The production itself is the preview worker's and the age ending is \
         `State, identity and recovery` §11's own.",
        "acting::previews",
    ),
    (
        "§8",
        "E4, E7, E8, E9, E13 and E13a — the four collection outcomes carried \
         one for one, the sinks taken whole from `Rented-server runtime`, the \
         live commentary read on request, the fetch covering every attempt \
         that carries a capture record and whole or not at all within one, \
         and the two observations recorded in the attempt's record and read \
         back for the fill-in.",
        "acting::words",
    ),
    (
        "§2",
        "Y3 — the provisional local record written before the submission is \
         sent, a refused write sending nothing, and a lost acknowledgement \
         answered by asking the machine for the identity rather than by \
         re-issuing the work.",
        "acting::submission",
    ),
    (
        "§4",
        "L10's queue side and L12's waiter — the stop's outcome read from the \
         recorded outcome alone, and a waiter with a deadline of its own \
         whose expiry says nothing about the stop. Sending the stop is \
         `Rendering`'s.",
        "acting::commands",
    ),
];

/// One act of this document that the rules half does not perform.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnActLeftToTheActingHalf {
    /// The plan's own top-level section.
    pub section: &'static str,
    /// Every clause of that section this covers.
    pub clauses: &'static [&'static str],
    /// The act.
    pub what: &'static str,
    /// Why it is not in the rules half, and what of the clause *is* here.
    pub why: &'static str,
}

/// **The acts, section by section.**
pub const THE_ACTING_HALF: [AnActLeftToTheActingHalf; 16] = [
    AnActLeftToTheActingHalf {
        section: "§1",
        clauses: &["Q1", "Q2", "Q3", "Q6", "Q8"],
        what: "the machine-local queue as one durable store in the package area, written only by the supervisor; the order of publication — material first, then the job record and the note in one act, then the event; untracked records in the same store; the disk-full answer; and Q2's two queue-state writes, each in the act that makes its fact true and each journalled",
        why: "**a durable store on the machine and the supervisor's own writes.** What is here is the record's shape (Q2), the line's order rule (Q4), the held record's absence of a key (Q5) and the key's rule (Q7) — `crate::line`",
    },
    AnActLeftToTheActingHalf {
        section: "§3",
        clauses: &["H2", "H4", "H11"],
        what: "the desk's answer as a command over the connection, the machine's serialization of two answers, and the drop event's journalling and application",
        why: "**`Rented-server runtime` §8's mechanism and the command path.** The predicate, its moment and what the entry says are `crate::waiting`",
    },
    AnActLeftToTheActingHalf {
        section: "§4",
        clauses: &["L1", "L2", "L6", "L7", "L13", "L14", "L21", "L22"],
        what: "the command records written durably before acting, their travel, the supervisor's one serialized write of each rewrite, the removal of a record and the material cleanup that follows, the mark's write riding the removal's outcome, and the drop at the admission point performed",
        why: "**each is the single writer's durable act.** Every rewrite is computed here as a pure function over the records as they stand — `crate::line`, `crate::acts`, `crate::stop_and_clear` — and written there",
    },
    AnActLeftToTheActingHalf {
        section: "§5",
        clauses: &["R1", "R2", "R3", "R4", "R5", "R7", "R8", "R9", "R10", "R14", "R15"],
        what: "the tee on the render worker's copy loop, the two grammars per version, the parse of each kind of line, the bounded reporting to the supervisor and its re-establishment across a restart",
        why: "**the reader rides a process's stream on the machine.** The closed list of readings, the largest-seen fold, the tiled-or-not condition and the report's shape are `crate::readings`",
    },
    AnActLeftToTheActingHalf {
        section: "§7",
        clauses: &["G1", "G2", "G3", "G4", "G5", "G5a"],
        what: "the card reading through the driver's own per-card query and the disk figure at the root's mount, taken at the two requests and carried on the health-readings route",
        why: "**readings of a machine, `Rented-server runtime` §12's route.** Nothing of this crate's turns on them",
    },
    AnActLeftToTheActingHalf {
        section: "§8",
        clauses: &["E4", "E7", "E8", "E9", "E13", "E13a"],
        what: "the capture into the sinks, the live commentary read over the connection, the fetch of both streams into the history record at reconciliation, and the two observations at the copy loop performed",
        why: "**the sinks and the loop are the worker's; the fetch is a transfer.** The ending's mapping, the device field and what the entry says about a colour line are `crate::ending`; the observation shapes are `crate::readings`",
    },
    AnActLeftToTheActingHalf {
        section: "§9",
        clauses: &["D1", "D2", "D3", "D4", "D5", "D6", "D7", "D8", "D9", "D10", "D11", "D12", "D13"],
        what: "reconciliation on the machine after it comes back — the re-run from durable state, the explicit frame list from the reckoning, the video switch, the attempt count advanced before the second attempt",
        why: "**`State, identity and recovery` §6 taken entire by `Rented-server runtime` §3.3, and `Job files and outputs` §10's reckoning.** What this crate keeps is the ending each disposition maps to (`crate::ending`) and the bar a re-run picks up (`crate::bars`)",
    },
    AnActLeftToTheActingHalf {
        section: "§10",
        clauses: &["N1", "N2", "N3", "N4", "N5", "N6", "N7", "N8", "N9"],
        what: "the per-job note on the machine — its creation with the record, its writes in the same act as each fact, the last-frame figure computed at the ending, and the recorded reading's journey to the entry by §7 step 4",
        why: "**a durable store on the machine written by the supervisor, and the fill-in.** The local record's fields, the merge rule and every draw are `crate::record`; N4's one duration is `crate::record::TheFourMoments`",
    },
    AnActLeftToTheActingHalf {
        section: "§11",
        clauses: &["C2", "C3", "C7", "C8"],
        what: "the notice record's durable write before the cursor advances, the cursor's durable row, the period-held reading's holding and dropping, and the six survivors' stores",
        why: "**writes to the local record and process memory.** The composition, the cursor's rule, the mark's moment and the two presentations are `crate::catch_up` and `crate::two_records`",
    },
    AnActLeftToTheActingHalf {
        section: "§12",
        clauses: &["W4", "W5", "W6"],
        what: "the desktop's timer that schedules the re-check, the fresh reading re-taken from the machine before speaking, the mark's write, and the telling through `Desktop app and delivery`'s notification kind",
        why: "**a timer, a reading and a write.** The idle judgement, the quiet moment's selection, the overdue arithmetic, the mark's shape and its one use are `crate::idle`",
    },
    AnActLeftToTheActingHalf {
        section: "§13",
        clauses: &["A1", "A2", "A3", "A4", "A5", "A5a", "A6", "A7", "A8", "A9"],
        what: "the queue reading's one request, the supervisor-unavailable condition, the adoption offer composed from the reading and accepted in one local write",
        why: "**a reading of a machine and §7 step 6.** The reading's shape is `crate::readings::AJobAsTheMachineReadsIt`; the tombstone exclusion is `crate::deletion::may_become_an_entry_again`",
    },
    AnActLeftToTheActingHalf {
        section: "§14",
        clauses: &["X2", "X3"],
        what: "the owed machine-side erase — its enumerated units removed at the next connection, its outcome record in the local record, the tombstone's flag set on all units done",
        why: "**a machine-reaching action with a `State, identity and recovery` §9.4 record.** The local act, what a disconnected deletion leaves, the refusal and the clearing acts are `crate::deletion`",
    },
    AnActLeftToTheActingHalf {
        section: "§15",
        clauses: &["P1", "P1b", "P2", "P3", "P4", "P4a", "P5", "P7", "P8", "P10", "P10a"],
        what: "the preview worker's production per finished frame, the ending's catch-up write, the live preview's one fetch in flight, the journalled trigger and the read-forward, the fetch through the transfer contract and the age ending's sweep, and the heading over the picture",
        why: "**production on the machine and transfers to the desktop.** P1a's queueing arithmetic, P6's class and P9's per-version name are `crate::previews`; the age ending's constant and sweep are `State, identity and recovery` §11's own",
    },
    AnActLeftToTheActingHalf {
        section: "§2",
        clauses: &["Y3"],
        what: "the provisional local record's write before the submission is sent, and the ask-the-machine resolution of a dropped submission",
        why: "**`State, identity and recovery` §7 step 2's write and `Rendering`'s matching.** The identity's issue, its form and the folder-and-label rules are `crate::identity`",
    },
    AnActLeftToTheActingHalf {
        section: "§4",
        clauses: &["L10", "L12"],
        what: "sending the stop — `Rendering`'s — and each waiter's own deadline",
        why: "**`State, identity and recovery` §5.3's sequence and `Server protocol and transfers`' per-waiter deadlines.** The stop's queue side and what the entry reads meanwhile are `crate::acts`",
    },
    AnActLeftToTheActingHalf {
        section: "§17",
        clauses: &["C1"],
        what: "the read-forward that applies events — §7 step 3 on a connection and the live-reading sequence's repetition inside a standing period — whose applied events the account is composed from",
        why: "**two passes that already exist and already move the cursor.** The composition over what they applied is `crate::catch_up::compose_the_account`",
    },
];

/// **The five observations `Rendering` K13's worker takes at its own copy
/// loop, and the shape this crate defines for each** — named exactly, so
/// the fourth builder finds them by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnObservationShape {
    pub clauses: &'static [&'static str],
    pub what: &'static str,
    pub the_shape: &'static str,
}

pub const THE_FIVE_OBSERVATION_SHAPES: [AnObservationShape; 5] = [
    AnObservationShape {
        clauses: &["R1", "R5", "R6"],
        what: "the live readings and their bounded reporting",
        the_shape: "crate::readings::AWorkersReport, carrying crate::readings::LiveReadings folded by crate::readings::fold_a_line",
    },
    AnObservationShape {
        clauses: &["H7"],
        what: "the ending-step completeness reading carried with the outcome",
        the_shape: "crate::waiting::CompletenessReading",
    },
    AnObservationShape {
        clauses: &["P1a"],
        what: "the frame's save announcement, with the newest-saved figure reported on the same bounded reporting",
        the_shape: "crate::readings::AWorkersReport::newest_frame_saved, and crate::readings::AnObservationAtTheCopyLoop::TheNewestFrameSaved",
    },
    AnObservationShape {
        clauses: &["E13"],
        what: "the device line observed in passing on the version's own stream",
        the_shape: "crate::readings::ObservedDeviceLine, judged by crate::ending::the_device_field",
    },
    AnObservationShape {
        clauses: &["E13a"],
        what: "the colour-substitution line observed in passing on either stream",
        the_shape: "crate::readings::ObservedColourSubstitutionLine, judged by crate::ending::what_the_entry_says_about_colour",
    },
];

/// **The `Ending` type and the classification value `Rendering` supplies
/// into it**, named for the same reader.
pub const THE_ENDING_TYPE: &str = "crate::ending::Ending";
pub const THE_VALUE_RENDERING_SUPPLIES: &str = "crate::ending::TheRenderKindsOwnEnding";

/// **Two things belong to somebody else and this crate builds no part of
/// either.** W1a refuses the mockup's *idle in 12 min* and sends it back to
/// Design to be parked — recorded at `crate::idle::WhenAMachineWillFallQuiet`,
/// with nothing built and no number supplied. §18 item 22 — N21's
/// disturbed-file mark, where `File Browser` W8's write does not name the
/// field — is routed to `File Browser` §8.4 and lands inside step 5's
/// held-back warning layer; the field, its readings and its lifetime are
/// `crate::record`'s, and the write that lands it is that layer's.
pub const TWO_THINGS_BELONG_TO_SOMEBODY_ELSE: [&str; 2] = [
    "W1a — the drawn value goes back to Design to be parked",
    "§18 item 22 — the mark's write rides `File Browser` W8's telling, owed at `File Browser` §8.4",
];

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use super::*;

    fn the_plan() -> String {
        fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../Preparation/Architecture/Queue & History.md"),
        )
        .expect("the plan is readable")
    }

    fn opens_a_clause(plan: &str, clause: &str) -> bool {
        plan.lines()
            .any(|line| line.trim_start().starts_with(&format!("**{clause}.")))
    }

    /// **Every clause named on the list is one the plan defines**, read at
    /// the test rather than copied here.
    #[test]
    fn every_clause_left_to_the_acting_half_is_a_clause_the_plan_defines() {
        let plan = the_plan();
        for act in THE_ACTING_HALF {
            for clause in act.clauses {
                assert!(
                    opens_a_clause(&plan, clause),
                    "{clause} is named at {} and the plan defines no such clause",
                    act.section
                );
            }
        }
        for shape in THE_FIVE_OBSERVATION_SHAPES {
            for clause in shape.clauses {
                assert!(opens_a_clause(&plan, clause), "{clause}");
            }
        }
    }

    /// **The five observation shapes name types this crate defines.**
    #[test]
    fn every_observation_shape_names_a_type_of_this_crate() {
        let sources: String = crate::markers::tests::every_source_of_this_crate()
            .into_iter()
            .map(|(_, text)| text)
            .collect();
        for shape in THE_FIVE_OBSERVATION_SHAPES {
            for name in shape
                .the_shape
                .split(|c: char| !c.is_alphanumeric() && c != '_' && c != ':')
            {
                if let Some(last) = name.rsplit("::").next() {
                    if last.chars().next().is_some_and(char::is_uppercase) {
                        assert!(
                            sources.contains(&format!(" {last}")),
                            "{last} is not defined"
                        );
                    }
                }
            }
        }
        assert!(sources.contains("pub enum Ending"));
        assert!(sources.contains("pub enum TheRenderKindsOwnEnding"));
        assert_eq!(THE_ENDING_TYPE, "crate::ending::Ending");
        assert_eq!(
            THE_VALUE_RENDERING_SUPPLIES,
            "crate::ending::TheRenderKindsOwnEnding"
        );
    }

    /// **What has landed names sections the plan has and modules this crate
    /// has**, read off both rather than trusted.
    #[test]
    fn what_has_landed_names_real_sections_and_real_modules() {
        let plan = the_plan();
        // **Both folders' acting halves since 2026-09-19**, when the machine's
        // acts moved to `smoozy-queue-history-machine`; they are re-exported
        // here at the paths the list names.
        let here = Path::new(env!("CARGO_MANIFEST_DIR"));
        let modules: Vec<String> = fs::read_dir(here.join("src/acting"))
            .expect("the acting half is there")
            .chain(
                fs::read_dir(here.join("../Queue & History — machine/src/acting"))
                    .expect("the machine's acting half is there"),
            )
            .map(|entry| {
                entry
                    .expect("the entry is readable")
                    .path()
                    .file_stem()
                    .and_then(|name| name.to_str())
                    .expect("the file has a name")
                    .to_string()
            })
            .collect();
        for (section, what, module) in WHAT_HAS_LANDED {
            assert!(
                THE_ACTING_HALF.iter().any(|act| act.section == section),
                "{section} is not a section the acting half names"
            );
            assert!(!what.is_empty());
            let module = module.trim_start_matches("acting::");
            assert!(
                modules.iter().any(|have| have == module),
                "{module} is named and this crate has no such module"
            );
        }
        // Every clause named as landed is one the plan defines.
        for clause in [
            "Q1", "Q2", "Q3", "Q6", "Q8", "R2", "R7", "R9", "R10", "R14", "R15",
        ] {
            assert!(opens_a_clause(&plan, clause), "{clause}");
        }
    }

    #[test]
    fn nothing_on_the_list_supplies_a_threshold_a_countdown_or_an_idle_forecast() {
        for act in THE_ACTING_HALF {
            assert!(!act.what.contains("threshold"), "{}", act.section);
            assert!(!act.what.contains("countdown"), "{}", act.section);
        }
        assert_eq!(TWO_THINGS_BELONG_TO_SOMEBODY_ELSE.len(), 2);
    }
}
