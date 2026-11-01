//! All-or-nothing, and the three endings — `Render Settings` §1.3 Q7–Q10.
//!
//! **Constraint:** Row 141 — *"A read either succeeds whole or gives nothing…
//! Nothing is left half-filled."* The read is by construction many readings —
//! no single means reaches every place a file writes — so the whole-or-nothing
//! boundary has to be drawn by the mechanism, not assumed. It is drawn here,
//! over Smoozy's own frame and never over anything Blender printed.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Dependency Upload & Remap` G3. `Server protocol
//! and transfers` C16, C17. `Rented-server runtime` A6.

use crate::reading::SettingsHalf;

/// The channel's own answer for the command that carried the read — `Server
/// protocol and transfers`' ending, consumed here and never re-diagnosed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelOutcome {
    /// The command ran and the invocation's frame below is what came back.
    TheInvocationAnswered,
    /// The command never ran or never answered — a waiter's expired deadline
    /// says the waiter gave up and nothing else (`Server protocol and
    /// transfers` C16, C17).
    TheMachineCouldNotBeReached,
}

/// How the invocation exited, **as a status and never as what was printed**.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitStatus {
    Succeeded,
    Failed,
}

/// **What the invocation left, in Smoozy's own frame.** The start and end
/// sentinels are Smoozy's, the error map is the reader's own, and the exit
/// status is the process's. Nothing here is a line Blender printed.
#[derive(Debug, Clone, PartialEq)]
pub struct WhatTheInvocationLeft {
    pub channel: ChannelOutcome,
    /// **Whether Smoozy itself ended the invocation** — a stop-and-clear, a
    /// supervisor shutdown, or the kind's own cancellation, each its own act
    /// with a recorded outcome (W2, `Rented-server runtime` A6). Smoozy always
    /// knows which those are.
    pub smoozy_ended_it: bool,
    pub exit: ExitStatus,
    /// **Q8's start sentinel: the mark the reader writes as its very first
    /// act** (`Job files and outputs`' `THE_STARTED_MARK`, since 2026-10-02),
    /// never the document's opening mark, which comes only at the reader's end.
    pub start_sentinel_seen: bool,
    pub end_sentinel_seen: bool,
    /// The error map's emptiness, where a document arrived at all.
    pub error_map_is_empty: Option<bool>,
    /// The document between the sentinels, parsed, where it was complete.
    pub document: Option<SettingsHalf>,
}

/// **Q8: three endings are told apart, and each from Smoozy's own frame rather
/// than from anything Blender printed — and a read Smoozy itself ended is
/// none of the three.**
#[derive(Debug, Clone, PartialEq)]
pub enum Ending {
    /// **Q7: a reading is the complete sentinel-framed document with an empty
    /// error map**, and this is the only arm that carries one.
    AReading(SettingsHalf),
    /// **Ending 1.** The channel's own ending, `Server protocol and transfers`'
    /// entire.
    TheMachineCouldNotBeReached,
    /// **Ending 2.** The invocation completed, ended at a failing status, no
    /// start sentinel was ever emitted, and Smoozy did not end it — a file the
    /// Blender cannot open ends the run before any handed-in script runs.
    /// Observed against a corrupt file and an absent file, both versions. **The
    /// page says which version failed from Smoozy's own record of which binary
    /// it invoked**, never by mining Blender's message.
    TheFileCouldNotBeOpenedByThisVersion,
    /// **Ending 3.** The start sentinel is present and the document is short,
    /// or complete with a non-empty error map. The invocation's record is kept
    /// for diagnosis under the ordinary captured-streams rules and nothing of
    /// the document is consumed.
    TheReadRanAndSomethingInItFailed,
    /// **A read Smoozy ended itself is not diagnosed as any of the three.**
    /// Without this arm such a read matches ending 2 exactly and the page
    /// would tell the user their `.blend` cannot be opened by the Blender they
    /// picked, which is false. What the page says for it is one line: the read
    /// was stopped, nothing about the file is claimed, and it can be run
    /// again.
    SmoozyStoppedIt,
    /// **No reading, and a shape Q8's three endings do not name**: the
    /// invocation completed at a *succeeding* status with no start sentinel.
    /// Q8's second ending is established on two file shapes and no more, and
    /// whether any other shape emits no start sentinel is unposed — so this
    /// arm is an explicit absence rather than a label guessed at. What breaks
    /// if it is ever met is the ending's label alone, never the safety
    /// guarantee: nothing is stored (Q7), submitting stays offered (Q9).
    NotAReadingAndNoneOfTheThreeEndingsFits,
}

impl Ending {
    /// **Q7, Q9: nothing is stored, shown or consumed from any ending but a
    /// reading**, and a failed read changes nothing.
    pub fn is_a_reading(&self) -> bool {
        matches!(self, Ending::AReading(_))
    }
}

/// **Q7 and Q8, performed.** The first ending is distinguishable from the
/// other two because it is a different document's ending arriving on a
/// different surface; the second from the third by the start sentinel, which
/// is Smoozy's own frame. **Q10: the connection dropping mid-read is ending 1
/// or ending 3 by the same tests and needs no case of its own.**
pub fn judge(left: WhatTheInvocationLeft) -> Ending {
    if left.channel == ChannelOutcome::TheMachineCouldNotBeReached {
        return Ending::TheMachineCouldNotBeReached;
    }
    // Q7: complete, and the error map empty — and only then a reading. This is
    // `Dependency Upload & Remap` G3's rule applied to the same kind.
    if left.start_sentinel_seen && left.end_sentinel_seen && left.error_map_is_empty == Some(true) {
        if let Some(document) = left.document {
            return Ending::AReading(document);
        }
    }
    if left.smoozy_ended_it {
        return Ending::SmoozyStoppedIt;
    }
    if left.start_sentinel_seen {
        return Ending::TheReadRanAndSomethingInItFailed;
    }
    match left.exit {
        ExitStatus::Failed => Ending::TheFileCouldNotBeOpenedByThisVersion,
        ExitStatus::Succeeded => Ending::NotAReadingAndNoneOfTheThreeEndingsFits,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_half::testing::a_settings_half;

    fn a_frame() -> WhatTheInvocationLeft {
        WhatTheInvocationLeft {
            channel: ChannelOutcome::TheInvocationAnswered,
            smoozy_ended_it: false,
            exit: ExitStatus::Succeeded,
            start_sentinel_seen: true,
            end_sentinel_seen: true,
            error_map_is_empty: Some(true),
            document: Some(a_settings_half()),
        }
    }

    #[test]
    fn a_reading_is_the_complete_framed_document_with_an_empty_error_map() {
        assert!(judge(a_frame()).is_a_reading());
    }

    #[test]
    fn a_document_whose_end_sentinel_never_arrived_is_no_reading() {
        let short = WhatTheInvocationLeft {
            end_sentinel_seen: false,
            document: None,
            ..a_frame()
        };
        assert_eq!(judge(short), Ending::TheReadRanAndSomethingInItFailed);
    }

    #[test]
    fn a_complete_document_with_a_non_empty_error_map_is_no_reading() {
        let errored = WhatTheInvocationLeft {
            error_map_is_empty: Some(false),
            ..a_frame()
        };
        assert_eq!(judge(errored), Ending::TheReadRanAndSomethingInItFailed);
    }

    #[test]
    fn the_channels_own_ending_is_ending_one_and_is_never_re_diagnosed() {
        let unreachable = WhatTheInvocationLeft {
            channel: ChannelOutcome::TheMachineCouldNotBeReached,
            exit: ExitStatus::Failed,
            start_sentinel_seen: false,
            end_sentinel_seen: false,
            error_map_is_empty: None,
            document: None,
            ..a_frame()
        };
        assert_eq!(judge(unreachable), Ending::TheMachineCouldNotBeReached);
    }

    #[test]
    fn a_failing_status_with_no_start_sentinel_is_the_file_this_version_cannot_open() {
        let refused = WhatTheInvocationLeft {
            exit: ExitStatus::Failed,
            start_sentinel_seen: false,
            end_sentinel_seen: false,
            error_map_is_empty: None,
            document: None,
            ..a_frame()
        };
        assert_eq!(judge(refused), Ending::TheFileCouldNotBeOpenedByThisVersion);
    }

    #[test]
    fn a_read_smoozy_ended_itself_is_none_of_the_three() {
        // Exactly ending 2's frame, but Smoozy's own act — and it must not read
        // as the file being unopenable.
        let stopped = WhatTheInvocationLeft {
            smoozy_ended_it: true,
            exit: ExitStatus::Failed,
            start_sentinel_seen: false,
            end_sentinel_seen: false,
            error_map_is_empty: None,
            document: None,
            ..a_frame()
        };
        assert_eq!(judge(stopped), Ending::SmoozyStoppedIt);
        // And the same where the reader had started.
        let stopped_late = WhatTheInvocationLeft {
            smoozy_ended_it: true,
            end_sentinel_seen: false,
            document: None,
            ..a_frame()
        };
        assert_eq!(judge(stopped_late), Ending::SmoozyStoppedIt);
    }

    #[test]
    fn a_succeeding_status_with_no_sentinel_is_an_explicit_absence_and_not_a_guess() {
        let odd = WhatTheInvocationLeft {
            start_sentinel_seen: false,
            end_sentinel_seen: false,
            error_map_is_empty: None,
            document: None,
            ..a_frame()
        };
        let ending = judge(odd);
        assert_eq!(ending, Ending::NotAReadingAndNoneOfTheThreeEndingsFits);
        assert!(!ending.is_a_reading());
    }
}
