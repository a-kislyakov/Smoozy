//! What this module says about a machine — `Server Setup` §10.
//!
//! **The constraint** is Rows 89, 295 and 308 — **all three about what is said
//! rather than what is done.**
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Queue & History` §17, E12, N12, R12. `Rented-server
//! runtime` G21, G22. `Rendering` L24a, L26.
//!
//! **`Rented-server runtime` R1, in its §13, and `Job files and outputs` M1 are
//! not listed above**, because this plan defines an R1 and an M1 of its own and a
//! bare one could mean either; where this file cites one, the document is written out at the site.


use crate::kind::{FamilyReading, MachineKind};
use crate::ready_action::WhatReadyClaims;
use crate::search::LiveState;

/// **W1: the ready action ends in one of five statements, and they are
/// distinguishable by construction, not by wording.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheEnding {
    /// **Ready** — [`crate::ready_action`] R4's claim, no more.
    Ready(WhatReadyClaims),
    /// **Nothing to prepare** — every check that had a subject passed before
    /// any act was needed. **Said plainly and not presented as a fault** — the
    /// option had nothing to do, which is not the same as failing.
    ///
    /// **Where the machine held no Blender and the run fetched none, the
    /// library half was not read at all, and the statement says so rather than
    /// claiming a check that never ran.**
    NothingToPrepare {
        the_library_half_was_read: bool,
    },
    /// **Not a kind Smoozy prepares** — [`crate::kind`] M1a's family reading
    /// placed the machine outside Ubuntu and Debian, or could not place it at
    /// all.
    ///
    /// **It comes before every other ending because it is reached before any
    /// step is composed**: nothing was installed, nothing was touched, no
    /// setup-in-progress state was ever set, **and the machine is left exactly
    /// as found**. **It is not *Refused* and not *Could not be made ready***,
    /// and the difference is real rather than a shade of wording — those two
    /// describe a machine that was worked on and would not take it, **while this
    /// one was never worked on.**
    NotAKindSmoozyPrepares(FamilyReading),
    /// **Refused** — a step was refused for lack of permission and the user
    /// declined elevation, or the machine offers no route ([`crate::kind`] M4),
    /// **or a failed validate whose text is not the wrong-password constant,
    /// which is not established to be a wrong password and so keeps the
    /// password**. Named with the step that was refused.
    Refused { the_step_that_was_refused: String },
    /// **Could not be made ready** — the action ran and **a step of it failed
    /// and could not be got past**, the card never answering being one case of
    /// it rather than the whole of it (the terminal container fault; the Secure
    /// Boot case), **named with the machine's own words behind it**, which
    /// [`crate::recovery`] S2a already records on the machine.
    ///
    /// **W1 was widened on 2026-09-04, out of this module's round-1 code
    /// audit.** It read *the action ran and the card never answered*, which left
    /// a step that failed for any other reason with no ending at all: libraries
    /// the fixed mapping does not name, a tooling install refused for a full
    /// disk, a package system reporting itself busy, a fetch failing its digest,
    /// a Debian components step that did not take. **A machine whose library
    /// chain does not resolve was told *Ready*.**
    ///
    /// **No sixth ending was added**: a step that failed is a machine that was
    /// worked on and would not take it, which is what this ending already means.
    CouldNotBeMadeReady { the_machines_own_words: String },
}

/// **What the run reached, as one value**, so the ending is decided from a
/// reading rather than from eight arguments a caller could put in the wrong
/// order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheRunReached {
    pub kind: MachineKind,
    pub family: FamilyReading,
    /// The step that was refused for lack of permission, where one was.
    pub a_step_was_refused_for_permission: Option<String>,
    /// The machine's own words, where any were printed.
    pub the_machines_own_words: Option<String>,
    pub claims: WhatReadyClaims,
    pub the_library_half_was_read: bool,
    pub anything_was_installed: bool,
    /// **W1, widened 2026-09-04: a step of the run failed and could not be got
    /// past**, for a reason that is neither a permission refusal nor the card
    /// not answering.
    pub a_step_failed_and_could_not_be_got_past: bool,
}

/// **W1: the family ending outranks all four, and by construction rather than
/// by rule** — it is decided from a reading taken before the sequence is
/// composed, **so no step has run to refuse anything or to leave a card
/// unanswered.**
///
/// **The precedence between the last two, because one machine can satisfy
/// both.** Fact 48 makes a container's non-answering card terminal for that
/// server and Fact 27 makes nothing open at all from a non-root login, **so a
/// container can be refused and unmakeable at once.**
///
/// **Where the machine kind closes the driver route by construction and the card
/// does not answer, the ending is *Could not be made ready*, whatever else was
/// refused** — [`crate::ready_action`] R3 settles that a container's driver
/// routes are closed and are never probed by trial, **so no elevation the user
/// could grant would change the answer**, and putting an elevation offer in
/// front of them would be an ask no step needs, which [`crate::elevation`] E2
/// forbids. **Where the card answers and only a permission refusal stopped the
/// action, the ending is *Refused*** and the elevation offer stands.
pub fn which_ending(reached: WhatTheRunReached) -> TheEnding {
    let WhatTheRunReached {
        kind,
        family,
        a_step_was_refused_for_permission,
        the_machines_own_words,
        claims,
        the_library_half_was_read,
        anything_was_installed,
        a_step_failed_and_could_not_be_got_past,
    } = reached;
    // **The family ending outranks all four.**
    if !family.a_setup_may_be_composed_for_it() || !kind.a_setup_may_run_on_it() {
        return TheEnding::NotAKindSmoozyPrepares(family);
    }

    let the_card_never_answered = matches!(claims.the_driver_answers, Some(false));
    let the_kind_closes_the_driver_route = matches!(kind, MachineKind::AContainer);

    if the_card_never_answered && the_kind_closes_the_driver_route {
        return TheEnding::CouldNotBeMadeReady {
            the_machines_own_words: the_machines_own_words.unwrap_or_default(),
        };
    }
    if let Some(step) = a_step_was_refused_for_permission {
        return TheEnding::Refused {
            the_step_that_was_refused: step,
        };
    }
    if the_card_never_answered {
        return TheEnding::CouldNotBeMadeReady {
            the_machines_own_words: the_machines_own_words.unwrap_or_default(),
        };
    }
    // **W1, widened: a step that failed and could not be got past is a machine
    // that was worked on and would not take it**, which is this ending's own
    // meaning. **The library chain is one of those steps and is read here** —
    // *Ready* is R4's claim and R4's claim is that the build's chain resolves,
    // so a run whose chain did not resolve may not be told *Ready*, and *Nothing
    // to prepare* requires in W1's own words that *"its library chain
    // resolves"*.
    let the_library_chain_did_not_resolve = matches!(claims.the_library_chain_resolves, Some(false));
    if a_step_failed_and_could_not_be_got_past || the_library_chain_did_not_resolve {
        return TheEnding::CouldNotBeMadeReady {
            the_machines_own_words: the_machines_own_words.unwrap_or_default(),
        };
    }

    // **W1: *Nothing to prepare* is every check that had a subject passing
    // before any act was needed** — so the two are told apart by whether the run
    // installed anything, and by nothing else. The endings that describe a
    // machine that was worked on and would not take it are already returned
    // above, so a run that reaches here having installed nothing is one that had
    // nothing to do.
    //
    // **The driver claim cannot be false here and the chain claim cannot be
    // false here**, both having been returned above — which is how *Nothing to
    // prepare*'s own two requirements are kept by construction rather than by a
    // second guard.
    //
    // **The card reading reaches this decision only through `claims`**, which is
    // where [`crate::ready_action`] R4 already folds it in: a machine whose cards
    // could not be established makes no driver claim at all, and *ready* would
    // then claim a check that did not run.
    if !anything_was_installed {
        return TheEnding::NothingToPrepare {
            the_library_half_was_read,
        };
    }
    TheEnding::Ready(claims)
}

/// **W2: what every one of those statements withholds, on the spec's own
/// bound — whether the machine renders on its cards.**
///
/// **An ordinary job will not settle it** — with the driver missing or wrong,
/// every version rendered on the processor and reported success — so the
/// statement is: **whether the cards work will be known once a job has run on
/// them, and until then nobody can say.**
pub fn what_every_statement_withholds() -> &'static str {
    "whether the cards work will be known once a job has run on them, and until then nobody can say"
}

/// **W3: Row 295 — the warning is a property of the step, the table is closed,
/// and silence where a step cannot disturb is enforced by the table rather than
/// by judgment.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WarnableStep {
    /// **Warns.** The live jobs named from `Queue & History` §17's live set; a
    /// running render is taken down by the restart — **observed**, two real
    /// machine restarts with a render in flight (Fact 55) — and re-run after the
    /// setup finishes; a transfer is interrupted and carried on.
    AReadyActionContainingDriverWorkOrARestart,
    /// **No warning.** Observed harmless, every frame written — **and the
    /// observation is of an unpack and a rename beside a running render, which
    /// is exactly what this row covers.**
    ABlenderInstallOnAMachineThatAlreadyHoldsTheUnpackTool,
    /// **Warns.** The step performs an elevated package install, **which the
    /// *no* row's observation never covered**; the run is machine-changing.
    ABlenderInstallWhereTheUnpackToolMustBeInstalled,
    /// **No warning.** Observed harmless, frames identical picture against
    /// picture; **the rate limit of the observation carried, and its
    /// write-location condition** — every install observed went into a user
    /// repository and none into the folder a running Blender was running from
    /// ([`crate::addons`] A6a).
    AnAddOnInstall,
    /// **Warns.** [`crate::search`] B14's warning.
    ABlenderRemovalALiveJobsBuildMatches,
    /// **No warning.** Restarts nothing but the role worker.
    TelegramMaterialOrADesiredStateChange,
    /// **No warning of this document's.** The whole commit is blocked while live
    /// or held work depends on the runtime and **the caller is told it is
    /// deferred** (`Rented-server runtime` G21, `Rented-server runtime` G22, and
    /// the deferred result is `Server protocol and transfers` B6's).
    PackagePlacementRepairOrReplacement,
}

impl WarnableStep {
    /// **W3's table, closed at seven rows.**
    pub const THE_TABLE: [WarnableStep; 7] = [
        WarnableStep::AReadyActionContainingDriverWorkOrARestart,
        WarnableStep::ABlenderInstallOnAMachineThatAlreadyHoldsTheUnpackTool,
        WarnableStep::ABlenderInstallWhereTheUnpackToolMustBeInstalled,
        WarnableStep::AnAddOnInstall,
        WarnableStep::ABlenderRemovalALiveJobsBuildMatches,
        WarnableStep::TelegramMaterialOrADesiredStateChange,
        WarnableStep::PackagePlacementRepairOrReplacement,
    ];

    /// **W3, performed. Warning where nothing can be disturbed is what trains
    /// the user to click through the one that matters, so the *no* rows are as
    /// binding as the *yes* rows.**
    pub fn warns(self) -> bool {
        matches!(
            self,
            WarnableStep::AReadyActionContainingDriverWorkOrARestart
                | WarnableStep::ABlenderInstallWhereTheUnpackToolMustBeInstalled
                | WarnableStep::ABlenderRemovalALiveJobsBuildMatches
        )
    }
}

/// **W3: a step whose effect is settled later slots into the table by the same
/// test — could it disturb work — rather than changing the rule.**
pub fn the_test_for_a_new_row() -> &'static str {
    "could it disturb work"
}

/// **W3a: the reading behind the three lines is over every finished job the
/// machine still holds, not over one.**
///
/// **The reading is on the machine and never on the desktop's history.** A
/// history entry carries the server's **label** as a copy taken at submission
/// (`Queue & History` N12) and no server identity, **so the desktop cannot
/// answer *on this machine* at all.** The machine's own finished-job records
/// can, they are already there, and the installation-identity gate has already
/// established that it is the same machine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheThreeQuestionsAnswered {
    /// Does any finished job record exist.
    pub any_finished_job_record_exists: bool,
    /// Which is the most recent, **by the machine's own clock** — the only
    /// clock involved, so this crosses no clocks.
    pub the_most_recent: Option<String>,
    /// **Does any record whose recorded outcome is *finished* carry a recorded
    /// device line**, with the most recent that does.
    ///
    /// **Corrected 2026-08-31 with [`what_the_device_line_says`]**: the third
    /// question used to be asked of the records at large, **and a line printed
    /// by a run that did not finish would have been selected by it.**
    pub the_most_recent_finished_record_carrying_a_device_line: Option<String>,
}

/// **W3a: which of the three lines the panel shows, and nothing is enumerated
/// to the desktop.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhichLine {
    /// **No finished job record** — *nothing has run here yet*.
    NothingHasRunHereYet,
    /// **At least one *finished* record carrying a line** —
    /// [`what_the_device_line_says`]'s conclusion, drawn from the most recent
    /// that carries one.
    TheDeviceLinesConclusion,
    /// **Records exist and not one of them qualifies** — *jobs have run here and
    /// none of them said*. **A machine whose only device lines were printed by
    /// runs that did not finish reads as this third line**, because no such line
    /// is ever selected. **A machine that has only ever rendered EEVEE has
    /// finished jobs and nothing to read, and this line says so rather than
    /// reading as the first.**
    JobsHaveRunHereAndNoneOfThemSaid,
}

/// **W3a, performed.**
pub fn which_line(answered: &WhatTheThreeQuestionsAnswered) -> WhichLine {
    if !answered.any_finished_job_record_exists {
        return WhichLine::NothingHasRunHereYet;
    }
    match answered.the_most_recent_finished_record_carrying_a_device_line {
        Some(_) => WhichLine::TheDeviceLinesConclusion,
        None => WhichLine::JobsHaveRunHereAndNoneOfThemSaid,
    }
}

/// **W3a: this clause widens the *set of jobs asked*, not what may be concluded
/// from any of them**, and the outcome gate below is not loosened by it.
pub const THE_SET_IS_WIDENED_AND_NOT_WHAT_MAY_BE_CONCLUDED: bool = true;

/// **W4: Row 308 — this module reads one thing from a job whose recorded
/// outcome is *finished* — the recorded device line — and concludes inside the
/// spec's exact bounds. Corrected 2026-08-31, out of the repaired Blender
/// research.**
///
/// The line is Cycles' own closing report, observed in passing on the version's
/// own stream by the render worker and recorded in the attempt record
/// (`Rendering` L26).
///
/// **This clause read that *a stopped render never reaches it*, and that
/// sentence was the whole ground for trusting a line found on the stream. It is
/// false.** After a stop, **5.0 and 5.1 print the whole closing block, 5.2
/// prints a cancelled line and no block, and 4.5 prints nothing**; and **5.2's
/// own published fix list calls that block on exit fake, with wrong numbers.**
/// Label: **observed in a test**. **So a device line observed on a run that did
/// not finish is not evidence of anything.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordedOutcome {
    Finished,
    /// **Every other ending, a stopped run that printed a block included.**
    AnythingElse,
}

/// What the manifest asked for, which is the other half of the three
/// conditions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheManifestAskedFor {
    TheCards,
    TheProcessor,
}

/// What the recorded device line names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheLineNames {
    TheProcessorAlone,
    /// **Anything but the processor alone** — the cards, as every card run on
    /// record names them. See [`what_the_line_names`] for why the two are
    /// told apart from one side only.
    TheCards,
}

/// **The device token a processor-only line closes with** — the bracketed
/// device list at the end of Cycles' `Path tracing on:` line. Label:
/// **observed in a test**. *What actually ran* question 1 (the comparison):
/// the line *"carries the model, and for a processor the thread count, and
/// for a card the route taken to it and its place on the bus"*, in all four
/// versions; its 5.1 findings (*What actually ran — second pass*) transcribe
/// the processor runs, Linux and Mac, as `… (CPU) (24 threads) [CPU]` and
/// `… (CPU) (8 threads) [CPU]`, and the card runs as `… [CUDA_…]`,
/// `… [CUDA_…_OptiX]` and `… [METAL_…]`; and `Rendering`'s sources file, in its entry on the 5.3
/// Alpha read beside 5.2.2, records `Path tracing on: … [CPU]` off both at the
/// desk.
pub const A_PROCESSOR_ONLY_LINE_CLOSES_WITH: &str = "[CPU]";

/// **`Server Setup` W4's third condition, read off the line: does it name the
/// processor alone?** Added 2026-10-03, the 38's piece 6, when the card first
/// had a line to read.
///
/// **Told apart from one side only, and that is the safe side.** A line whose
/// device list is exactly [`A_PROCESSOR_ONLY_LINE_CLOSES_WITH`] names the
/// processor alone; **every other line is read as not the processor alone**,
/// and so can never fire W4's setup fault — which offers a step that can
/// restart the machine. **What this cannot see, said plainly**: 4.5's own
/// processor line was not transcribed in any finding read here, so a 4.5
/// processor line shaped otherwise reads as not the processor alone, and the
/// card states the line as printed rather than calling a fault. A line naming
/// the processor *and* a card (a list with more than the one token) is not the
/// processor alone either.
pub fn what_the_line_names(the_line_as_printed: &str) -> WhatTheLineNames {
    let the_device_list = the_line_as_printed
        .trim_end()
        .rfind('[')
        .map(|at| &the_line_as_printed.trim_end()[at..]);
    if the_device_list == Some(A_PROCESSOR_ONLY_LINE_CLOSES_WITH) {
        WhatTheLineNames::TheProcessorAlone
    } else {
        WhatTheLineNames::TheCards
    }
}

/// **W4's conclusion, and the closed set of what may be said.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatW4Concludes {
    /// **Where all three hold — the job's recorded outcome is *finished*, its
    /// manifest's device ask was the cards, and its recorded line names the
    /// processor alone** — this is a **setup fault**, said so, with the one
    /// thing Smoozy knows to do — run the ready action again — offered under
    /// [`WarnableStep`]'s warning, since it can restart the machine.
    ASetupFault,
    /// **The job's devices are not known.** For every ending but *finished*,
    /// **a stopped run that printed a block included**; and **the line's absence
    /// concludes nothing** — a job whose line was not observed is a job whose
    /// devices are not known, full stop. **EEVEE and Workbench establish
    /// nothing.**
    TheJobsDevicesAreNotKnown,
    /// **Where the job asked for the processor: nothing is said about the
    /// machine.**
    NothingIsSaidAboutTheMachine,
    /// The line names the cards and the ask was the cards: the job ran as
    /// asked, and no fault is declared.
    ItRanAsAsked,
}

/// **W4, performed. The reading is gated on the job's recorded outcome and
/// never on the line's presence.**
///
/// The device line is read **only** from a job whose recorded outcome is
/// *finished* — `Queue & History` E12, repaired the same day on the same
/// evidence, is the rule and this clause states no second one.
pub fn what_the_device_line_says(
    outcome: RecordedOutcome,
    asked_for: WhatTheManifestAskedFor,
    line: Option<WhatTheLineNames>,
) -> WhatW4Concludes {
    if !matches!(outcome, RecordedOutcome::Finished) {
        return WhatW4Concludes::TheJobsDevicesAreNotKnown;
    }
    if matches!(asked_for, WhatTheManifestAskedFor::TheProcessor) {
        return WhatW4Concludes::NothingIsSaidAboutTheMachine;
    }
    match line {
        Some(WhatTheLineNames::TheProcessorAlone) => WhatW4Concludes::ASetupFault,
        Some(WhatTheLineNames::TheCards) => WhatW4Concludes::ItRanAsAsked,
        None => WhatW4Concludes::TheJobsDevicesAreNotKnown,
    }
}

/// **W4's one dependency that reaches outside this module, named 2026-09-01 out
/// of the repaired Blender research.**
///
/// Those same three conditions are **also produced by Smoozy's own launch line
/// getting the device argument's name wrong**: after the bare double dash an
/// argument Cycles does not know is **swallowed without a word and the render
/// goes ahead at exit 0 on the processor** (`Rendering` **L24a**). **This clause
/// cannot tell the two apart from anything it reads, and it would then blame the
/// machine for Smoozy's own line and restart it.**
///
/// **What keeps that from happening is `Rendering` L24a's rule that the
/// argument's spelling is a constant of that contract at one declaration point,
/// never composed per job** — so a wrong spelling is **one Build-time fault,
/// caught once**, rather than a condition this module could meet in the field.
/// **This clause depends on that rule and does not restate it.**
pub const W4_DEPENDS_ON_THE_DEVICE_ARGUMENTS_SPELLING_BEING_A_CONSTANT: bool = true;

/// **W3: the live set the warning is composed from, per `Queue & History`
/// §17.**
pub fn is_live(state: LiveState) -> bool {
    matches!(
        state,
        LiveState::Waiting | LiveState::Held | LiveState::Rendering
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kind::DistributionFamily;
    use crate::ready_action::WhatReadyClaims;

    fn claims(driver_answers: Option<bool>, library: Option<bool>) -> WhatReadyClaims {
        WhatReadyClaims {
            the_library_chain_resolves: library,
            the_driver_answers: driver_answers,
            it_renders_on_its_cards: (),
        }
    }

    /// **W4's third condition read off the line, from the processor's side
    /// only**: the transcribed processor lines name the processor alone; the
    /// transcribed card lines, a line naming both, and a line shaped otherwise
    /// do not — so none of them can fire the setup fault. The 38's piece 6.
    #[test]
    fn only_a_line_closing_on_the_processor_alone_names_the_processor_alone() {
        for processor in [
            "Path tracing on: AMD EPYC 7B12 64-Core Processor (CPU) (24 threads) [CPU]",
            "Path tracing on: Apple M1 Pro (CPU) (8 threads) [CPU]",
            "Path tracing on: AMD EPYC 7B13 64-Core Processor (CPU) (61 threads) [CPU]  ",
        ] {
            assert_eq!(what_the_line_names(processor), WhatTheLineNames::TheProcessorAlone, "{processor}");
        }
        for not_alone in [
            "Path tracing on: NVIDIA GeForce RTX 4080 SUPER (OptiX) [CUDA_NVIDIA GeForce RTX 4080 SUPER_0000:00:07_OptiX]",
            "Path tracing on: NVIDIA GeForce RTX 4080 SUPER (CUDA) [CUDA_NVIDIA GeForce RTX 4080 SUPER_0000:00:07]",
            "Path tracing on: Apple M1 Pro (GPU - 14 cores) (Metal) (display) [METAL_Apple M1 Pro (GPU - 14 cores)]",
            "Path tracing on: a card and a processor [CUDA_a card, CPU]",
            "Path tracing on: a processor, its thread count",
            "",
        ] {
            assert_eq!(what_the_line_names(not_alone), WhatTheLineNames::TheCards, "{not_alone}");
        }
    }

    #[test]
    fn the_family_ending_outranks_every_other_and_is_reached_before_any_step() {
        let ending = which_ending(WhatTheRunReached {
            kind: MachineKind::ASystemdMachine,
            family: FamilyReading::ItWouldNotSay,
            a_step_was_refused_for_permission: Some("the driver install".to_string()),
            the_machines_own_words: Some("words".to_string()),
            claims: claims(Some(false), None),
            the_library_half_was_read: false,
            anything_was_installed: true,
            a_step_failed_and_could_not_be_got_past: false,
        });
        assert!(matches!(ending, TheEnding::NotAKindSmoozyPrepares(_)));
    }

    #[test]
    fn a_container_that_is_both_refused_and_unmakeable_ends_could_not_be_made_ready() {
        // W1's precedence: no elevation the user could grant would change the
        // answer, and an offer in front of them would be an ask no step needs.
        let ending = which_ending(WhatTheRunReached {
            kind: MachineKind::AContainer,
            family: FamilyReading::Supported(DistributionFamily::Ubuntu),
            a_step_was_refused_for_permission: Some("the libraries install".to_string()),
            the_machines_own_words: Some("permission denied".to_string()),
            claims: claims(Some(false), Some(false)),
            the_library_half_was_read: true,
            anything_was_installed: true,
            a_step_failed_and_could_not_be_got_past: false,
        });
        assert!(matches!(ending, TheEnding::CouldNotBeMadeReady { .. }));
    }

    #[test]
    fn a_systemd_machine_whose_card_answers_and_whose_step_was_refused_ends_refused() {
        let ending = which_ending(WhatTheRunReached {
            kind: MachineKind::ASystemdMachine,
            family: FamilyReading::Supported(DistributionFamily::Debian),
            a_step_was_refused_for_permission: Some("the libraries install".to_string()),
            the_machines_own_words: None,
            claims: claims(Some(true), Some(false)),
            the_library_half_was_read: true,
            anything_was_installed: true,
            a_step_failed_and_could_not_be_got_past: false,
        });
        assert_eq!(
            ending,
            TheEnding::Refused {
                the_step_that_was_refused: "the libraries install".to_string()
            }
        );
    }

    #[test]
    fn nothing_to_prepare_says_when_the_library_half_was_not_read() {
        // W1 and R4: the statement says so rather than claiming a check that
        // never ran.
        let ending = which_ending(WhatTheRunReached {
            kind: MachineKind::ASystemdMachine,
            family: FamilyReading::Supported(DistributionFamily::Ubuntu),
            a_step_was_refused_for_permission: None,
            the_machines_own_words: None,
            claims: claims(Some(true), None),
            the_library_half_was_read: false,
            anything_was_installed: false,
            a_step_failed_and_could_not_be_got_past: false,
        });
        assert_eq!(
            ending,
            TheEnding::NothingToPrepare {
                the_library_half_was_read: false
            }
        );
    }

    #[test]
    fn a_machine_whose_cards_could_not_be_established_and_took_no_act_had_nothing_to_prepare() {
        // The card reading reaches the ending only through the claim, and a
        // machine with no driver claim at all is not a machine that was worked
        // on and would not take it.
        let ending = which_ending(WhatTheRunReached {
            kind: MachineKind::AContainer,
            family: FamilyReading::Supported(DistributionFamily::Ubuntu),
            a_step_was_refused_for_permission: None,
            the_machines_own_words: None,
            claims: claims(None, Some(true)),
            the_library_half_was_read: true,
            anything_was_installed: false,
            a_step_failed_and_could_not_be_got_past: false,
        });
        assert_eq!(
            ending,
            TheEnding::NothingToPrepare {
                the_library_half_was_read: true
            }
        );
    }

    #[test]
    fn ready_is_reached_only_where_the_run_actually_installed_something() {
        let ending = which_ending(WhatTheRunReached {
            kind: MachineKind::ASystemdMachine,
            family: FamilyReading::Supported(DistributionFamily::Ubuntu),
            a_step_was_refused_for_permission: None,
            the_machines_own_words: None,
            claims: claims(Some(true), Some(true)),
            the_library_half_was_read: true,
            anything_was_installed: true,
            a_step_failed_and_could_not_be_got_past: false,
        });
        assert!(matches!(ending, TheEnding::Ready(_)));
    }

    #[test]
    fn a_machine_whose_library_chain_does_not_resolve_is_never_told_ready_or_nothing_to_prepare() {
        // **The fault this test exists for.** `which_ending` never read the
        // chain claim until 2026-09-04: the value was set, asserted once in a
        // test, and consulted by no judgement — so a machine whose chain did not
        // resolve was told *Ready* where the run installed something and
        // *Nothing to prepare* where it did not, and R4's claim rode out inside
        // the answer.
        for anything_was_installed in [true, false] {
            let ending = which_ending(WhatTheRunReached {
                kind: MachineKind::ASystemdMachine,
                family: FamilyReading::Supported(DistributionFamily::Ubuntu),
                a_step_was_refused_for_permission: None,
                the_machines_own_words: Some("libnowhere.so.9 not found".to_string()),
                claims: claims(Some(true), Some(false)),
                the_library_half_was_read: true,
                anything_was_installed,
                a_step_failed_and_could_not_be_got_past: false,
            });
            assert_eq!(
                ending,
                TheEnding::CouldNotBeMadeReady {
                    the_machines_own_words: "libnowhere.so.9 not found".to_string()
                }
            );
        }
    }

    #[test]
    fn a_step_that_failed_for_neither_a_refusal_nor_a_silent_card_could_not_be_made_ready() {
        // W1, widened 2026-09-04: the package system reporting itself busy, a
        // fetch failing its digest, a components step that did not take — none
        // is a permission refusal and none is the card not answering, and every
        // one of them had no ending at all before.
        let ending = which_ending(WhatTheRunReached {
            kind: MachineKind::ASystemdMachine,
            family: FamilyReading::Supported(DistributionFamily::Debian),
            a_step_was_refused_for_permission: None,
            the_machines_own_words: Some("E: dpkg was interrupted".to_string()),
            claims: claims(Some(true), Some(true)),
            the_library_half_was_read: true,
            anything_was_installed: true,
            a_step_failed_and_could_not_be_got_past: true,
        });
        assert_eq!(
            ending,
            TheEnding::CouldNotBeMadeReady {
                the_machines_own_words: "E: dpkg was interrupted".to_string()
            }
        );
    }

    #[test]
    fn every_statement_withholds_whether_the_machine_renders_on_its_cards() {
        assert!(what_every_statement_withholds().contains("until then nobody can say"));
    }

    #[test]
    fn the_warning_table_is_closed_at_seven_and_three_of_them_warn() {
        assert_eq!(WarnableStep::THE_TABLE.len(), 7);
        assert_eq!(
            WarnableStep::THE_TABLE.iter().filter(|it| it.warns()).count(),
            3
        );
        // The two Blender-install rows fall on opposite sides, which is the
        // whole point of splitting them.
        assert!(!WarnableStep::ABlenderInstallOnAMachineThatAlreadyHoldsTheUnpackTool.warns());
        assert!(WarnableStep::ABlenderInstallWhereTheUnpackToolMustBeInstalled.warns());
        assert_eq!(the_test_for_a_new_row(), "could it disturb work");
    }

    #[test]
    fn a_device_line_from_a_run_that_did_not_finish_concludes_nothing() {
        // W4, corrected 2026-08-31: 5.0 and 5.1 print the whole closing block
        // after a stop, and 5.2's own fix list calls that block on exit fake.
        assert_eq!(
            what_the_device_line_says(
                RecordedOutcome::AnythingElse,
                WhatTheManifestAskedFor::TheCards,
                Some(WhatTheLineNames::TheProcessorAlone),
            ),
            WhatW4Concludes::TheJobsDevicesAreNotKnown
        );
    }

    #[test]
    fn a_finished_cards_job_whose_line_names_the_processor_alone_is_a_setup_fault() {
        assert_eq!(
            what_the_device_line_says(
                RecordedOutcome::Finished,
                WhatTheManifestAskedFor::TheCards,
                Some(WhatTheLineNames::TheProcessorAlone),
            ),
            WhatW4Concludes::ASetupFault
        );
        const { assert!(W4_DEPENDS_ON_THE_DEVICE_ARGUMENTS_SPELLING_BEING_A_CONSTANT) };
    }

    #[test]
    fn the_lines_absence_concludes_nothing_and_a_processor_job_says_nothing_about_the_machine() {
        assert_eq!(
            what_the_device_line_says(
                RecordedOutcome::Finished,
                WhatTheManifestAskedFor::TheCards,
                None,
            ),
            WhatW4Concludes::TheJobsDevicesAreNotKnown
        );
        assert_eq!(
            what_the_device_line_says(
                RecordedOutcome::Finished,
                WhatTheManifestAskedFor::TheProcessor,
                Some(WhatTheLineNames::TheProcessorAlone),
            ),
            WhatW4Concludes::NothingIsSaidAboutTheMachine
        );
    }

    #[test]
    fn a_machine_whose_only_lines_came_from_unfinished_runs_reads_as_the_third_line() {
        // W3a: no such line is ever selected, so the panel says jobs have run
        // here and none of them said.
        let answered = WhatTheThreeQuestionsAnswered {
            any_finished_job_record_exists: true,
            the_most_recent: Some("job-9".to_string()),
            the_most_recent_finished_record_carrying_a_device_line: None,
        };
        assert_eq!(which_line(&answered), WhichLine::JobsHaveRunHereAndNoneOfThemSaid);

        let empty = WhatTheThreeQuestionsAnswered {
            any_finished_job_record_exists: false,
            the_most_recent: None,
            the_most_recent_finished_record_carrying_a_device_line: None,
        };
        assert_eq!(which_line(&empty), WhichLine::NothingHasRunHereYet);
        const { assert!(THE_SET_IS_WIDENED_AND_NOT_WHAT_MAY_BE_CONCLUDED) };
    }

    #[test]
    fn the_live_set_is_the_three_states_queue_and_history_names() {
        for state in [LiveState::Waiting, LiveState::Held, LiveState::Rendering] {
            assert!(is_live(state));
        }
    }
}
