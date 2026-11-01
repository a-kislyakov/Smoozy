//! One job, two records, two lifetimes — `Queue & History` §11 C6–C11.
//!
//! **Constraint:** Rows 78 and 81. `State, identity and recovery` D4's three
//! exceptions and no fourth: the desktop keeps no durable copy of an
//! observed machine state the machine still holds, save the last-known queue
//! reading (period-held), the history record (Smoozy's own note, not a copy)
//! and the remembered server state (subject-held).
//!
//! **C10 is the line this file keeps: the two lists are two presentations of
//! two records with one identity, not two stores of one fact.** Every
//! function here composes and returns; none stores.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Servers` K3, K5, Q19, S11.

use smoozy_state::identities::Identity;
use smoozy_state::lifetimes::Lifetime;
use smoozy_state::observed::Reading;

use crate::ending::Ending;
use crate::line::QueueState;
use crate::readings::{AJobAsTheMachineReadsIt, WhatTheEntryHasLive};
use crate::record::LocalJobRecord;

/// **C6: the last-known queue reading is period-held** — process memory for
/// the connected period, never durable, gone when the period ends. So a
/// dropped-connection server shows last-known entries for the rest of the
/// run; a fresh launch knows nothing until it connects; and there is no
/// durable cache anywhere that could be shown hours stale — prevented by
/// construction rather than by care.
pub const THE_LAST_KNOWN_READINGS_LIFETIME: Lifetime = Lifetime::PeriodHeld;

/// **C7: a last-known entry carries the moment it was taken and says it is
/// not updating.** The stamp and the not-updating statement appear at the
/// moment `Servers` declares the break and tears the connection down (its
/// K3 and K5), and stand until a later connection's fresh reading — **and
/// the mark also reaches a reading held while a connected server's §7 has
/// not completed** (`Servers` Q19's fourth branch), because the mark is
/// about the reading's age, never about the connection's state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HowTheReadingIsDrawn {
    /// Live: the sequence that updates it is running.
    Live,
    /// The stamp, and the not-updating statement — wording Design's.
    LastKnown { taken_at: Reading<()> },
}

pub fn how_the_reading_is_drawn(
    the_reading: &Reading<()>,
    the_break_is_declared: bool,
    reconciliation_has_completed_on_this_connection: bool,
) -> HowTheReadingIsDrawn {
    if the_break_is_declared || !reconciliation_has_completed_on_this_connection {
        HowTheReadingIsDrawn::LastKnown {
            taken_at: the_reading.clone(),
        }
    } else {
        HowTheReadingIsDrawn::Live
    }
}

/// **C7's two readers.** `Servers`' removal warning reads this same
/// period-held record and is silent where Smoozy holds none; its edit
/// warning is a second reader and reads the durable job records instead, so
/// the two warnings differ in reach on purpose — the edit warning still
/// warns after a hand disconnect or a restart (`Servers` S11).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatAWarningReads {
    /// The removal warning: the period-held reading; nothing where none.
    ThePeriodHeldReading,
    /// The edit warning: the durable job records of C8 and C11.
    TheDurableJobRecords,
}

/// **C8: what survives the period is exactly the three exceptions and this
/// document's own notes** — each Smoozy's own record of Smoozy's own acts or
/// D4's named exceptions, none a copy of a machine state the machine still
/// holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatSurvivesThePeriod {
    /// Smoozy's own note, not a copy.
    TheHistoryRecord,
    /// `Servers`' Row 28, subject-held, owed there.
    TheRememberedServerState,
    TheTombstones,
    TheEventCursor,
    TheCatchUpNoticeRecord,
    TheIdleSpokenMark,
}

impl WhatSurvivesThePeriod {
    pub const ALL: [WhatSurvivesThePeriod; 6] = [
        WhatSurvivesThePeriod::TheHistoryRecord,
        WhatSurvivesThePeriod::TheRememberedServerState,
        WhatSurvivesThePeriod::TheTombstones,
        WhatSurvivesThePeriod::TheEventCursor,
        WhatSurvivesThePeriod::TheCatchUpNoticeRecord,
        WhatSurvivesThePeriod::TheIdleSpokenMark,
    ];

    /// Each is either Smoozy's own record of Smoozy's own act, or one of D4's
    /// three exceptions — and none is a copy of a live machine state.
    pub fn is_a_copy_of_a_machine_state_the_machine_still_holds(self) -> bool {
        false
    }

    pub fn lifetime(self) -> Lifetime {
        match self {
            WhatSurvivesThePeriod::TheHistoryRecord | WhatSurvivesThePeriod::TheTombstones => {
                Lifetime::UserHeld
            }
            WhatSurvivesThePeriod::TheRememberedServerState
            | WhatSurvivesThePeriod::TheEventCursor
            | WhatSurvivesThePeriod::TheCatchUpNoticeRecord
            | WhatSurvivesThePeriod::TheIdleSpokenMark => Lifetime::SubjectHeld,
        }
    }
}

/// **C9: a job is in history from the moment it starts rendering, and the
/// moment is the machine's, learned late.** The desktop learns the
/// render-start at step 3 or sees it live, and from that learning the local
/// job record is presented in history as well as in the queue. The started
/// moment shown is the note's, on the machine's clock, never the moment
/// Smoozy learned it.
pub fn is_in_history(the_render_start_has_been_learned: bool) -> bool {
    the_render_start_has_been_learned
}

/// **C10: the two presentations, composed from the two records at the
/// draw.** The queue entry is the period-held reading of the machine's live
/// state; the history entry is the local record. While both hold the job,
/// the machine is where the live half is read from and Smoozy's record is
/// what survives the machine.
#[derive(Debug, Clone, PartialEq)]
pub struct TwoPresentations<'a> {
    pub queue_row: Option<AQueueRow<'a>>,
    pub history_row: Option<AHistoryRow<'a>>,
}

/// The queue row: the reading's live half beside the record's own copies.
#[derive(Debug, Clone, PartialEq)]
pub struct AQueueRow<'a> {
    pub job: &'a Identity,
    pub state: &'a QueueState,
    pub live: &'a WhatTheEntryHasLive,
    pub record: &'a LocalJobRecord,
}

/// The history row: the local record, with the ending Smoozy learned.
#[derive(Debug, Clone, PartialEq)]
pub struct AHistoryRow<'a> {
    pub record: &'a LocalJobRecord,
    pub ending: HistoryEnding,
}

/// **C11: when the job ends it leaves the queue and the history entry takes
/// the ending Smoozy learns** — from watching, or from the note on a later
/// connection — and an entry whose machine never came back keeps the job
/// with no ending Smoozy ever learned, shown as such, for good.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistoryEnding {
    Learned(Ending),
    NoEndingSmoozyEverLearned,
}

pub fn the_two_presentations<'a>(
    record: &'a LocalJobRecord,
    the_reading: Option<&'a AJobAsTheMachineReadsIt>,
    the_render_start_has_been_learned: bool,
    the_ending_learned: Option<Ending>,
) -> TwoPresentations<'a> {
    let queue_row = the_reading
        .filter(|reading| !matches!(reading.state, QueueState::Ended { .. }))
        .map(|reading| AQueueRow {
            job: &reading.job,
            state: &reading.state,
            live: &reading.live,
            record,
        });
    let history_row = is_in_history(the_render_start_has_been_learned).then_some(AHistoryRow {
        record,
        ending: match the_ending_learned {
            Some(ending) => HistoryEnding::Learned(ending),
            None => HistoryEnding::NoEndingSmoozyEverLearned,
        },
    });
    TwoPresentations {
        queue_row,
        history_row,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::record::tests::{a_record, a_server};
    use smoozy_state::observed::{Clock, MachineClock};
    use std::time::SystemTime;

    fn a_machine_reading() -> Reading<()> {
        Reading::taken(
            (),
            SystemTime::UNIX_EPOCH,
            Clock::AMachine(MachineClock("server-7".to_string())),
        )
    }

    #[test]
    fn the_last_known_reading_is_period_held_and_nothing_durable() {
        assert_eq!(THE_LAST_KNOWN_READINGS_LIFETIME, Lifetime::PeriodHeld);
    }

    #[test]
    fn the_mark_appears_at_the_declared_break_and_on_an_unreconciled_connection() {
        let reading = a_machine_reading();
        assert_eq!(
            how_the_reading_is_drawn(&reading, false, true),
            HowTheReadingIsDrawn::Live
        );
        assert_eq!(
            how_the_reading_is_drawn(&reading, true, true),
            HowTheReadingIsDrawn::LastKnown {
                taken_at: reading.clone()
            }
        );
        assert_eq!(
            how_the_reading_is_drawn(&reading, false, false),
            HowTheReadingIsDrawn::LastKnown {
                taken_at: reading.clone()
            }
        );
    }

    #[test]
    fn what_survives_the_period_is_six_things_and_none_is_a_copy_of_a_live_machine_state() {
        assert_eq!(WhatSurvivesThePeriod::ALL.len(), 6);
        for thing in WhatSurvivesThePeriod::ALL {
            assert!(!thing.is_a_copy_of_a_machine_state_the_machine_still_holds());
            assert_ne!(thing.lifetime(), Lifetime::PeriodHeld);
        }
    }

    #[test]
    fn a_job_is_in_both_lists_from_the_render_start_and_only_in_history_after_the_ending() {
        let server = a_server();
        let record = a_record(&server);
        let live = AJobAsTheMachineReadsIt {
            job: record.job.clone(),
            state: QueueState::Rendering {
                operation: crate::line::testing::an_operation(),
            },
            live: WhatTheEntryHasLive::NoReadings,
            the_hold: None,
        };
        let both = the_two_presentations(&record, Some(&live), true, None);
        assert!(both.queue_row.is_some());
        assert!(both.history_row.is_some());
        // Waiting: in the queue, not yet in history.
        let waiting = AJobAsTheMachineReadsIt {
            state: QueueState::Waiting,
            ..live.clone()
        };
        let queue_only = the_two_presentations(&record, Some(&waiting), false, None);
        assert!(queue_only.queue_row.is_some());
        assert!(queue_only.history_row.is_none());
        // Ended: leaves the queue, history takes the ending.
        let ended = AJobAsTheMachineReadsIt {
            state: QueueState::Ended {
                outcome: crate::line::testing::an_operation(),
            },
            ..live
        };
        let history_only =
            the_two_presentations(&record, Some(&ended), true, Some(Ending::Finished));
        assert!(history_only.queue_row.is_none());
        assert_eq!(
            history_only.history_row.unwrap().ending,
            HistoryEnding::Learned(Ending::Finished)
        );
    }

    #[test]
    fn a_machine_that_never_came_back_leaves_no_ending_smoozy_ever_learned_for_good() {
        let server = a_server();
        let record = a_record(&server);
        let gone = the_two_presentations(&record, None, true, None);
        assert!(gone.queue_row.is_none());
        assert_eq!(
            gone.history_row.unwrap().ending,
            HistoryEnding::NoEndingSmoozyEverLearned
        );
    }
}
