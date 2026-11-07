//! The ending, and the words — `Queue & History` §8 E1–E3, E5, E6, E10,
//! E11, E12, E12a, E13's and E13a's entry side, and E14.
//!
//! **Constraint:** Rows 67, 68 and 69. `Rented-server runtime` §11 owns the
//! captured streams, the exit and the closed set of four collection
//! outcomes; its M20–M22 own the ending's ground. Nothing here re-derives
//! any of it; what is decided is what the entry does with it.
//!
//! **This crate owns the [`Ending`] type and E2's mapping; `Rendering`
//! supplies the classification value into it** — the runner's ruling for
//! step 7, because E2 maps from recorded outcomes and that document is
//! built after this one. So [`TheRenderKindsOwnEnding`] is the value that
//! document's worker classifies, bounded by the two rules E2 fixes for it.
//!
//! **E12a is the sharpest trap in this step and it is held by the type.**
//! [`DevicesThatDidTheWork`] is filled from the recorded device line and from
//! nothing else, has a third value that is a state rather than a default, and
//! [`the_device_field`] takes no ask as an argument — so nothing here can
//! infer a device from what was asked for.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rented-server runtime` M20, M21, M22, M23, M24,
//! M13.

use smoozy_render_settings_machine::device::DeviceChoice;

use crate::readings::ObservedDeviceLine;

/// **E2: the entry's four endings map from recorded outcomes and from
/// nothing else.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ending {
    Finished,
    Failed,
    Stopped,
    GivenUp,
}

impl Ending {
    /// **§17's told line: the ending in E2's own four words.**
    pub const ALL: [Ending; 4] = [
        Ending::Finished,
        Ending::Failed,
        Ending::Stopped,
        Ending::GivenUp,
    ];
}

/// **The render kind's own ending, whose classification is `Rendering`'s to
/// define** — with two rules already fixed for it here that that document
/// reads as constraints: **an ending with no status is failed**
/// (`Rented-server runtime` M21), and **a successful exit status is never
/// read as *everything was written*** (`Rented-server runtime` M22) — twenty
/// separate endings report success, six of them ending at 0 with an empty
/// output folder — and the exit number never names a cause, 137, 134, 135
/// and 139 each arriving from unrelated events (M23).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheRenderKindsOwnEnding {
    /// The render ended and `Rendering` classifies it finished. **A
    /// statement about the render's ending, never about the outputs** (E3).
    Finished,
    /// The render ended and `Rendering` classifies it failed.
    Failed,
    /// **M21: a group that is gone with no reaped status is an ending with no
    /// status, and an ending with no status is failed.** A value of its own
    /// so the mapping keeps the rule rather than trusting the caller to.
    NoStatusWasReaped,
}

/// **The recorded outcome an ending is mapped from.** *Left stopped* is
/// written only by the stop path (L11); *given up* by §6's disposition
/// (D12); the render kind's own ending by its worker.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordedOutcome {
    LeftStopped,
    GivenUp,
    TheRenderKindsOwn(TheRenderKindsOwnEnding),
}

/// **E2, performed.** Nothing but a recorded outcome reaches it: no stream,
/// no exit number, no timing (E1).
pub fn the_ending(outcome: RecordedOutcome) -> Ending {
    match outcome {
        RecordedOutcome::LeftStopped => Ending::Stopped,
        RecordedOutcome::GivenUp => Ending::GivenUp,
        RecordedOutcome::TheRenderKindsOwn(TheRenderKindsOwnEnding::Finished) => Ending::Finished,
        RecordedOutcome::TheRenderKindsOwn(TheRenderKindsOwnEnding::Failed)
        | RecordedOutcome::TheRenderKindsOwn(TheRenderKindsOwnEnding::NoStatusWasReaped) => {
            Ending::Failed
        }
    }
}

/// **E1: the ending is decided by the render having ended, and the entry's
/// state comes from records alone.** The reverse also holds: a stream that
/// has gone quiet is not an ending, and the entry stays *rendering* while
/// the group validates — a run that sits forever is a live run the user can
/// stop, and **no timer of Smoozy's ends a render**. The limit is on
/// waiters, each with its own deadline, never on the run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheEntrysStateComesFrom {
    /// The operation record, the execution group's validation and the reaped
    /// exit status.
    TheRecords,
}

/// E1's rule: a quiet stream, a missing reading and a run of any length are
/// not endings.
pub fn is_an_ending(the_stream_has_gone_quiet: bool, no_reading_for_a_long_time: bool) -> bool {
    let _ = (the_stream_has_gone_quiet, no_reading_for_a_long_time);
    false
}

/// **E5: some endings never arrive at all, and the entry is honest about
/// that shape too.** Two observed 4.5 cases print a message and then sit for
/// ever — and an engine named on the launch line whose add-on is absent does
/// not exit at all on 4.5 — so they read as live work, because they are live
/// processes, and the stop is the user's route. This crate manufactures no
/// ending for them, and the idle machinery never fires while a render
/// operation runs (W1).
pub const NO_ENDING_IS_MANUFACTURED_FOR_A_RUN_THAT_NEVER_ENDS: bool = true;

/// **E6: a dropped connection is not a stop, and no clause of this section
/// fires for one.** Nothing here reads a connection's end as an event about
/// a job.
pub const A_DROPPED_CONNECTION_IS_NOT_AN_EVENT_ABOUT_A_JOB: bool = true;

/// **E3: *finished* is a statement about the render's ending, never about the
/// outputs, and the completeness statement stands beside it.** The entry
/// never converts exit 0 into *everything was written* and never converts a
/// short count into a failure the render did not report. **One render prints
/// `Sample 600/600`, then `Finished`, then `Cancelled`, exits 1 and leaves no
/// file.** The two are shown as the two facts they are. **A count that found
/// nothing at all of what the job owed is `Rendering` K14b's *failed*, decided on
/// the machine at the ending (limited 2026-11-02)**: the entry maps from that
/// outcome as from any other, so nothing here converts anything.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TwoFactsBesideEachOther {
    pub the_ending: Ending,
    /// `Job files and outputs` C27's statement, named per path (`Job files and outputs` C28) — held
    /// here as the count that came up short, where it did, and never merged
    /// into the ending.
    pub the_completeness_statement: Option<crate::waiting::CompletenessReading>,
}

impl TwoFactsBesideEachOther {
    /// Neither fact moves the other.
    pub fn the_ending_is_unchanged_by_the_count(&self) -> bool {
        true
    }
}

/// **E10: nothing reads the words.** Not sorted, not grouped, not mined for
/// a cause — no version hands out a failure in a form a program could act
/// on, and four wordings exist for one failure (`Rented-server runtime` M24).
/// The three things observed *at the loop* rather than read from the sink
/// are E13's device line, E13a's colour-substitution line and P1a's
/// frame-save announcement, each an observation of bytes in passing.
pub const NOTHING_READS_THE_WORDS: bool = true;

/// **E11: a re-run's words are the new attempt's own** — two sinks and two
/// outcomes per attempt, never merged; the entry reaches each attempt's
/// record, and history keeps what it fetched per attempt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnAttemptsWords {
    pub attempt: u32,
    /// The per-stream collection outcomes, `Rented-server runtime` M13's
    /// closed set of four, carried per stream per attempt (E4) and presented
    /// each as its own sentence — an empty panel is never what the user
    /// meets.
    pub ordinary_output: CollectionOutcome,
    pub complaints: CollectionOutcome,
}

/// **E4: which silence it is comes from the collection outcome, not from the
/// sink.** *The render printed nothing* and *nothing of what the render
/// printed reached Smoozy* are the same on a naive entry and are not the
/// same event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectionOutcome {
    Collected,
    CollectedAndTheStreamWasEmpty,
    CollectionIncomplete,
    NeverEstablished,
}

/// **E12: the device line is taken only from a job whose recorded outcome is
/// *finished*, and the entry says *not known* everywhere else.** After a
/// stop, 5.0 and 5.1 print the whole closing block, 5.2 prints a cancelled
/// line and no block, and 4.5 prints nothing; and 5.2's own fix list calls
/// that block on exit fake, with wrong numbers. **So a device line observed
/// on a run that did not finish is not evidence, and it is never read.**
///
/// **The third value is a state rather than a default**: it is what the
/// entry says, in the spec's own words, wherever the line was not observed
/// on a finished job — a stopped run that printed a block included.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DevicesThatDidTheWork {
    /// The line, observed on a job whose recorded outcome is *finished*,
    /// stated plainly and as printed.
    Known { the_line_as_printed: String },
    /// Not known — every other ending, and a finished job whose line was not
    /// observed. **A state, never a default.**
    NotKnown,
}

/// **E12, performed — and E12a's bar, kept by the signature: there is no
/// argument for the ask.** A builder that fills this field from the ask has
/// broken E12, and a reader that infers the device from the ask has made the
/// mistake `Rendering` L21a exists to forbid. The only inputs are the
/// recorded outcome and the observed line.
pub fn the_device_field(
    outcome: RecordedOutcome,
    observed: Option<&ObservedDeviceLine>,
) -> DevicesThatDidTheWork {
    the_device_field_from_the_line_as_printed(
        outcome,
        observed.map(|line| line.the_line_as_printed.as_str()),
    )
}

/// **E12, performed on the line as the note's ending carries it** — the text
/// alone, which is all `Queue & History` §10's paragraph after N3 sends off
/// the machine: the attempt it passed on and the stream it passed on stay in
/// the attempt's own record. **The rule is this one function's**, and
/// [`the_device_field`] is it applied to an observation; E12a's bar holds here
/// too, because there is still no argument for the ask.
///
/// Added at step 10c's fix piece 2, when the desktop first had a line to judge.
pub fn the_device_field_from_the_line_as_printed(
    outcome: RecordedOutcome,
    the_line_as_printed: Option<&str>,
) -> DevicesThatDidTheWork {
    match (the_ending(outcome), the_line_as_printed) {
        (Ending::Finished, Some(line)) if !line.is_empty() => DevicesThatDidTheWork::Known {
            the_line_as_printed: line.to_string(),
        },
        _ => DevicesThatDidTheWork::NotKnown,
    }
}

/// **E12a: what was *asked for* is a different fact from what did the work,
/// Smoozy holds it for every job on every ending, and the entry states it
/// whatever the ending was.** The resolved device choice is `Render
/// Settings` D1's payload entry, composed at submission; two copies already
/// exist — in the manifest and in the local job record — and neither is
/// derived from a reading. **The entry states the two as two and never
/// merges them**: where the line is not permitted, the entry says the
/// devices are not known **beside** the ask rather than instead of it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheDevicesOnTheEntry {
    /// What the job was asked to render on, in words that name it as the ask.
    pub asked_for: DeviceChoice,
    /// What did the work, only where E12 permits.
    pub did_the_work: DevicesThatDidTheWork,
}

/// The two, composed from their two sources. **The ask never fills the other
/// field**: `did_the_work` comes from [`the_device_field`], whose signature
/// has no ask in it.
pub fn the_devices_on_the_entry(
    asked_for: DeviceChoice,
    outcome: RecordedOutcome,
    observed: Option<&ObservedDeviceLine>,
) -> TheDevicesOnTheEntry {
    TheDevicesOnTheEntry {
        asked_for,
        did_the_work: the_device_field(outcome, observed),
    }
}

/// **E12a's third bullet, stated so nobody promotes it**: a job asked for
/// the cards can render on the processor and say nothing about it.
pub const THE_ASK_IS_NOT_EVIDENCE_OF_WHAT_RAN: bool = true;

/// **E13a: the three lines on record, in substance, for 4.5 to 5.2 — their
/// exact printed text is on record for none of them** (§18 item 27,
/// permanently unresolved). 5.3's lines are quoted, from a beta, and do not fit
/// these three; a reader for them is parked in Open Threads, on the build
/// runner's ruling of 2026-11-02.
/// **So this holds the substance as a kind and no text at all**: an absence
/// a type can tell from a value, exactly as the five spellings in `Render
/// Settings` are held. A line shape written from a plausible memory would be
/// mistaken for an observed one, and where a match built on it missed, the
/// telling would not fire — silently, because absence of the line and
/// absence of a substitution present the same way.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColourSubstitutionKind {
    /// The scene view not found, and a default set.
    TheSceneViewNotFoundAndADefaultSet,
    /// The image colour space not found, and a default to be used in its
    /// place.
    TheImageColourSpaceNotFoundAndADefaultUsed,
    /// A warning that the current value matches no entry in the list.
    TheCurrentValueMatchesNoEntryInTheList,
}

impl ColourSubstitutionKind {
    pub const ALL_THREE: [ColourSubstitutionKind; 3] = [
        ColourSubstitutionKind::TheSceneViewNotFoundAndADefaultSet,
        ColourSubstitutionKind::TheImageColourSpaceNotFoundAndADefaultUsed,
        ColourSubstitutionKind::TheCurrentValueMatchesNoEntryInTheList,
    ];

    /// **No printed text of the three is held here, in any version.** Not a
    /// string that is empty — an absence the type states, and the one thing
    /// this returns.
    pub fn the_printed_text(self) -> ThePrintedText {
        ThePrintedText::NotHeldHere
    }
}

/// The printed text of a colour-substitution line, which this crate does not
/// hold: 4.5 to 5.2's is on record nowhere, and 5.3's, from a beta, is parked
/// (E13a). **There is no other variant**, and that is the point: nothing
/// composes a match from this, and the acting half's grammar cannot be
/// written from memory through it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThePrintedText {
    NotHeldHere,
}

/// **E13a: the entry states two facts and infers none** — that the machine
/// did not hold what the file named, and that Blender rendered with its own
/// in its place. **It is never an ending, never a failure and never a warning
/// about the picture.** Where no such line was observed the entry says
/// nothing at all, and **absence is never evidence that no substitution
/// happened**.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheEntrySaysAboutColour {
    /// The two facts, from the observation.
    TheMachineDidNotHaveWhatTheFileNamedAndBlenderRenderedWithItsOwn {
        kind: ColourSubstitutionKind,
    },
    /// Nothing at all — and no claim that no substitution happened.
    NothingAndNoClaim,
}

pub fn what_the_entry_says_about_colour(
    observed: Option<&crate::readings::ObservedColourSubstitutionLine>,
) -> WhatTheEntrySaysAboutColour {
    match observed {
        Some(line) => {
            WhatTheEntrySaysAboutColour::TheMachineDidNotHaveWhatTheFileNamedAndBlenderRenderedWithItsOwn {
                kind: line.kind,
            }
        }
        None => WhatTheEntrySaysAboutColour::NothingAndNoClaim,
    }
}

/// **E14: Rows 68 and 69 are answered together deliberately**, because the
/// device line is bytes on the same captured streams: one capture, one
/// loop, one fill-in, no second channel.
pub const ONE_CAPTURE_ONE_LOOP_ONE_FILL_IN_NO_SECOND_CHANNEL: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::readings::WhichStream;

    fn a_device_line() -> ObservedDeviceLine {
        ObservedDeviceLine {
            attempt: 1,
            the_line_as_printed: "Device: NVIDIA RTX 4090 (CUDA)".to_string(),
            on_stream: WhichStream::TheOrdinaryOutput,
        }
    }

    #[test]
    fn the_four_endings_map_from_recorded_outcomes_and_from_nothing_else() {
        assert_eq!(the_ending(RecordedOutcome::LeftStopped), Ending::Stopped);
        assert_eq!(the_ending(RecordedOutcome::GivenUp), Ending::GivenUp);
        assert_eq!(
            the_ending(RecordedOutcome::TheRenderKindsOwn(
                TheRenderKindsOwnEnding::Finished
            )),
            Ending::Finished
        );
        assert_eq!(
            the_ending(RecordedOutcome::TheRenderKindsOwn(
                TheRenderKindsOwnEnding::Failed
            )),
            Ending::Failed
        );
    }

    #[test]
    fn an_ending_with_no_status_is_failed() {
        assert_eq!(
            the_ending(RecordedOutcome::TheRenderKindsOwn(
                TheRenderKindsOwnEnding::NoStatusWasReaped
            )),
            Ending::Failed
        );
    }

    #[test]
    fn a_quiet_stream_and_a_long_run_are_not_endings() {
        assert!(!is_an_ending(true, true));
        assert!(!is_an_ending(true, false));
        assert!(!is_an_ending(false, true));
    }

    #[test]
    fn the_device_field_is_known_only_from_a_line_on_a_finished_job() {
        let line = a_device_line();
        assert_eq!(
            the_device_field(
                RecordedOutcome::TheRenderKindsOwn(TheRenderKindsOwnEnding::Finished),
                Some(&line)
            ),
            DevicesThatDidTheWork::Known {
                the_line_as_printed: line.the_line_as_printed.clone()
            }
        );
        // A stopped run that printed a block: not evidence, never read.
        assert_eq!(
            the_device_field(RecordedOutcome::LeftStopped, Some(&line)),
            DevicesThatDidTheWork::NotKnown
        );
        assert_eq!(
            the_device_field(RecordedOutcome::GivenUp, Some(&line)),
            DevicesThatDidTheWork::NotKnown
        );
        assert_eq!(
            the_device_field(
                RecordedOutcome::TheRenderKindsOwn(TheRenderKindsOwnEnding::Failed),
                Some(&line)
            ),
            DevicesThatDidTheWork::NotKnown
        );
        // A finished job whose line was not observed.
        assert_eq!(
            the_device_field(
                RecordedOutcome::TheRenderKindsOwn(TheRenderKindsOwnEnding::Finished),
                None
            ),
            DevicesThatDidTheWork::NotKnown
        );
    }

    /// **E12 on the line as the note's ending carries it** — the text alone,
    /// judged by the same rule: known only on a finished ending, and never
    /// from an empty line (`Queue & History` §10, the paragraph after N3).
    #[test]
    fn the_line_the_note_carries_is_read_only_on_a_finished_ending() {
        let line = "Device: NVIDIA RTX 4090 (CUDA)";
        let finished = RecordedOutcome::TheRenderKindsOwn(TheRenderKindsOwnEnding::Finished);
        assert_eq!(
            the_device_field_from_the_line_as_printed(finished, Some(line)),
            DevicesThatDidTheWork::Known {
                the_line_as_printed: line.to_string()
            }
        );
        for not_finished in [
            RecordedOutcome::LeftStopped,
            RecordedOutcome::GivenUp,
            RecordedOutcome::TheRenderKindsOwn(TheRenderKindsOwnEnding::Failed),
            RecordedOutcome::TheRenderKindsOwn(TheRenderKindsOwnEnding::NoStatusWasReaped),
        ] {
            assert_eq!(
                the_device_field_from_the_line_as_printed(not_finished, Some(line)),
                DevicesThatDidTheWork::NotKnown,
                "{not_finished:?}"
            );
        }
        assert_eq!(
            the_device_field_from_the_line_as_printed(finished, Some("")),
            DevicesThatDidTheWork::NotKnown
        );
        assert_eq!(
            the_device_field_from_the_line_as_printed(finished, None),
            DevicesThatDidTheWork::NotKnown
        );
    }

    /// **E12a's trap, held.** A job asked for the cards, failed for want of
    /// memory: the entry states the ask and says the devices are not known,
    /// and the ask never becomes the device.
    #[test]
    fn the_ask_never_fills_the_device_field_on_any_ending() {
        for outcome in [
            RecordedOutcome::LeftStopped,
            RecordedOutcome::GivenUp,
            RecordedOutcome::TheRenderKindsOwn(TheRenderKindsOwnEnding::Failed),
            RecordedOutcome::TheRenderKindsOwn(TheRenderKindsOwnEnding::Finished),
        ] {
            let entry = the_devices_on_the_entry(DeviceChoice::TheCards, outcome, None);
            assert_eq!(entry.asked_for, DeviceChoice::TheCards);
            assert_eq!(entry.did_the_work, DevicesThatDidTheWork::NotKnown);
        }
        // And with a line on a finished job, the field is the line — which
        // may name the processor while the ask named the cards.
        let processor_line = ObservedDeviceLine {
            the_line_as_printed: "Device: CPU".to_string(),
            ..a_device_line()
        };
        let entry = the_devices_on_the_entry(
            DeviceChoice::TheCards,
            RecordedOutcome::TheRenderKindsOwn(TheRenderKindsOwnEnding::Finished),
            Some(&processor_line),
        );
        assert_eq!(entry.asked_for, DeviceChoice::TheCards);
        assert_eq!(
            entry.did_the_work,
            DevicesThatDidTheWork::Known {
                the_line_as_printed: "Device: CPU".to_string()
            }
        );
    }

    /// **The signature is the guard**: the sources are read for a device
    /// field composed from a `DeviceChoice`, which is the one thing E12a
    /// forbids.
    #[test]
    fn no_line_of_this_crate_composes_the_device_field_from_the_ask() {
        for (path, text) in crate::markers::tests::every_source_of_this_crate() {
            for line in text.lines() {
                let trimmed = line.trim_start();
                if trimmed.starts_with("//") {
                    continue;
                }
                let ask = format!("{}{}", "DeviceChoice", "::");
                let known = format!("{}{}", "DevicesThatDidTheWork::", "Known");
                assert!(
                    !(line.contains(&ask) && line.contains(&known)),
                    "{path}: {line}"
                );
            }
        }
    }

    #[test]
    fn the_three_colour_lines_have_a_kind_each_and_no_text_held_here() {
        assert_eq!(ColourSubstitutionKind::ALL_THREE.len(), 3);
        for kind in ColourSubstitutionKind::ALL_THREE {
            assert_eq!(kind.the_printed_text(), ThePrintedText::NotHeldHere);
        }
    }

    #[test]
    fn the_entry_states_the_two_colour_facts_from_an_observation_and_nothing_from_none() {
        let observed = crate::readings::ObservedColourSubstitutionLine {
            attempt: 1,
            kind: ColourSubstitutionKind::TheSceneViewNotFoundAndADefaultSet,
            the_line_as_printed: "…".to_string(),
            on_stream: WhichStream::TheComplaints,
        };
        assert_eq!(
            what_the_entry_says_about_colour(Some(&observed)),
            WhatTheEntrySaysAboutColour::TheMachineDidNotHaveWhatTheFileNamedAndBlenderRenderedWithItsOwn {
                kind: ColourSubstitutionKind::TheSceneViewNotFoundAndADefaultSet
            }
        );
        assert_eq!(
            what_the_entry_says_about_colour(None),
            WhatTheEntrySaysAboutColour::NothingAndNoClaim
        );
    }

    #[test]
    fn the_ending_and_the_count_are_two_facts_and_neither_moves_the_other() {
        let facts = TwoFactsBesideEachOther {
            the_ending: Ending::Finished,
            the_completeness_statement: Some(crate::waiting::CompletenessReading {
                frames_already_there: Vec::new(),
                the_shape: crate::waiting::TheInventorysShape::Frames,
            }),
        };
        assert_eq!(
            facts.the_ending,
            Ending::Finished,
            "exit 0 with nothing written is still finished"
        );
        assert!(facts.the_ending_is_unchanged_by_the_count());
    }
}
