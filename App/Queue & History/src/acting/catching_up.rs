//! **§11 C2, C3, C7 and C8, and §17 C1, performed — the notice record's
//! durable write before the cursor advances, the cursor's durable row, the
//! period-held reading's holding and dropping, and the six survivors'
//! stores.**
//!
//! # The order is the promise, and it is the only thing this module decides
//!
//! **C2: applying step 3 writes the notice's content into the local record as
//! part of the application, before the cursor advances past the events it
//! names.** *"The cursor moves when the work the event asked for is done, and
//! composing the account is that work."* So [`the_read_forward`] writes the
//! record and only then advances the cursor. **A crash between advancing and
//! showing loses the showing and not the record**, and the next run shows the
//! standing notice.
//!
//! **Reversing the two is the fault this order exists to prevent**: a cursor
//! advanced first and a crash before the write loses the account for good,
//! because the events it named will never be read again.
//!
//! # §17 C1's two passes are one mechanism
//!
//! **The read-forward that applies events is §7 step 3 on a connection and the
//! live-reading sequence's repetition inside a standing period**, and they are
//! one mechanism read at two moments. **Everything applied by either pass is
//! the catch-up account**, and [`the_read_forward`] is the one function both
//! call. **No new pass, no new store, no schedule and no watcher is added.**
//!
//! # The local record is reached through one trait
//!
//! [`KeepsTheCatchUpHalf`] is the split `App/Keys/` drew and every crate since
//! has kept: the judgements stay testable at the desk with the record's
//! answers arriving as values. **It is not a store of this document's own** —
//! the notice record and the idle-spoken mark sit in the local record, which
//! is `Desktop app and delivery` §5's, and the fact that
//! `State, identity and recovery` §1.1's inventory names neither is the
//! store-inventory seam this plan already records and does not repair.
//!
//! # The period-held reading is process memory and nothing else
//!
//! **C6 is kept by construction rather than by care**: [`ThePeriodHeldReadings`]
//! has no durable form, no serialization and no path. A period that ends drops
//! the record entirely, so **there is no durable cache anywhere that could be
//! shown hours stale.**
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `State, identity and recovery` §1.1, §7, §9.3.
//! `Desktop app and delivery` §5, T18, T19, P10d. `Servers` K3, K5, Q19.

use std::collections::{HashMap, HashSet};

use smoozy_state::identities::Identity;
use smoozy_state::observed::Reading;

use crate::catch_up::{compose_the_account, ANoticeEntry, AnAppliedEvent, EventCursor, NoticeRecord, ReadFrom};
use crate::readings::AJobAsTheMachineReadsIt;
use crate::two_records::{HowTheReadingIsDrawn, WhatSurvivesThePeriod};

/// **The two durable rows this document puts in the local record, reached
/// through one trait.**
///
/// **Every write below is the desktop's own, on the user's own computer**;
/// nothing here reaches a machine.
pub trait KeepsTheCatchUpHalf {
    /// **C2's record, singular per machine.**
    fn the_standing_notice(&self, server: &Identity) -> Option<NoticeRecord>;

    /// **C2: written before the cursor advances past the events it names.**
    fn write_the_notice(&mut self, record: &NoticeRecord) -> Result<(), String>;

    /// **C2's lifetime, and the user's own act.** Dismissal removes the
    /// record; a server row removed takes its record with it.
    fn remove_the_notice(&mut self, server: &Identity) -> Result<(), String>;

    /// Every standing, undismissed record at this moment — what C2a's one
    /// message is composed over at the draw.
    fn every_standing_notice(&self) -> Vec<NoticeRecord>;

    /// **C3's cursor, durable against the server row and the installation
    /// identity it was taken under.**
    fn the_cursor(&self, server: &Identity) -> Option<EventCursor>;

    /// **C3: never advanced on receipt.** This is called after the work the
    /// events asked for is done.
    fn advance_the_cursor(&mut self, server: &Identity, cursor: &EventCursor)
        -> Result<(), String>;
}

/// What one read-forward pass left.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatThePassLeft {
    /// Where the pass read from, which is C3's answer and never a clock.
    pub read_from: ReadFrom,
    /// **The account, where the pass composed or grew one.** `None` where
    /// nothing stood and nothing new was applied.
    pub the_account: Option<NoticeRecord>,
    /// The cursor as it stands after the pass.
    pub cursor: EventCursor,
}

/// **§17 C1 and §11 C2 and C3, performed as one pass.**
///
/// **The events are applied by the caller** — applying them is `State,
/// identity and recovery` §7 step 3's and `Desktop app and delivery` P10d's,
/// and both already exist and already move the cursor. What this function
/// does is the composition over what they applied, the durable write of the
/// account, and the cursor's advance, **in that order**.
///
/// **A new installation reads from the beginning of its own journal**, never
/// from a cursor taken under a dead one — [`crate::catch_up::where_to_read_from`]
/// answers it and this only obeys.
///
/// **T18: the account posts no notification.** Nothing here fires one, and
/// the per-ending notifications the desktop raises are `Desktop app and
/// delivery`'s, untouched.
pub fn the_read_forward(
    keeper: &mut dyn KeepsTheCatchUpHalf,
    server: &Identity,
    the_installation_now: &Identity,
    applied: &[AnAppliedEvent],
    tracked: &HashSet<Identity>,
    the_sequence_number_the_pass_reached: u64,
    the_journals_oldest_retained_has_passed_the_cursor: bool,
) -> Result<WhatThePassLeft, String> {
    let standing = keeper.the_cursor(server);
    let read_from = crate::catch_up::where_to_read_from(standing.as_ref(), the_installation_now);
    let account = compose_the_account(
        keeper.the_standing_notice(server),
        server,
        applied,
        tracked,
        the_journals_oldest_retained_has_passed_the_cursor,
    );
    // **C2: the record is written first.** A crash here loses the showing and
    // not the record.
    if let Some(record) = &account {
        keeper.write_the_notice(record)?;
    }
    let cursor = EventCursor {
        taken_under: the_installation_now.clone(),
        past_sequence_number: the_sequence_number_the_pass_reached,
    };
    keeper.advance_the_cursor(server, &cursor)?;
    Ok(WhatThePassLeft {
        read_from,
        the_account: account,
        cursor,
    })
}

/// **C2's dismissal, and C2a's two shapes of it.**
///
/// **Dismissing the message dismisses every record behind it, each on its own
/// terms; dismissing one group dismisses that record alone.** Nothing spanning
/// servers is stored, so there is no second lifetime to end.
///
/// *(Since 2026-10-12 the desktop calls neither shape: the message as one
/// dismissable whole is gone with the banner, and the two acts that end a
/// record are [`history_was_opened`] and [`the_dropped_line_is_dismissed`].)*
pub fn dismiss(
    keeper: &mut dyn KeepsTheCatchUpHalf,
    what: crate::catch_up::Dismissal,
) -> Result<(), String> {
    let standing = keeper.every_standing_notice();
    let kept = crate::catch_up::dismiss(standing.clone(), what);
    for record in standing {
        if !kept.iter().any(|left| left.server == record.server) {
            keeper.remove_the_notice(&record.server)?;
        }
    }
    Ok(())
}

/// **Whether a line is about a job dropped before it rendered** — H11's held
/// job, or L22's waiting job whose Blender was removed. Such a job has no
/// History entry, so its line is the only place it is ever named.
fn it_names_a_dropped_job(entry: &ANoticeEntry) -> bool {
    matches!(
        entry,
        ANoticeEntry::HeldJobDropped { .. } | ANoticeEntry::WaitingJobDroppedForARemovedBlender { .. }
    )
}

/// **C2a's dismissal as changed 2026-10-12, Creator-stated, the Creator's
/// ninth notes (2026-10-11), note 3: opening History takes every ending off
/// every standing record.** A record left naming nothing is dismissed; one
/// still naming a dropped job stands, holding that line alone, until the line
/// is dismissed ([`the_dropped_line_is_dismissed`]).
///
/// **A re-run goes with the endings.** It is not an ending, and C1's account
/// no longer tells it anywhere: the job's own entry carries the mark. A
/// re-run line left behind would keep a record standing that nothing on
/// screen names, which is the reading this function takes of *a record left
/// naming nothing*. So would C4's gap statement on a record with no line, and
/// it goes with the record.
///
/// **This is what makes the count's *Since you last opened History* true**:
/// the count is composed at draw time from the endings the standing records
/// hold, and after this none holds one.
pub fn history_was_opened(keeper: &mut dyn KeepsTheCatchUpHalf) -> Result<(), String> {
    for mut record in keeper.every_standing_notice() {
        let before = record.entries.len();
        record.entries.retain(it_names_a_dropped_job);
        if record.entries.is_empty() {
            keeper.remove_the_notice(&record.server)?;
        } else if record.entries.len() != before {
            keeper.write_the_notice(&record)?;
        }
    }
    Ok(())
}

/// **The user's *Dismiss* on one dropped job's line takes that one name off
/// the record that holds it** — C2a as changed 2026-10-12. The record stands
/// while it still counts an ending or names another dropped job, and is
/// dismissed once it does neither. A job named on no standing record changes
/// nothing.
pub fn the_dropped_line_is_dismissed(
    keeper: &mut dyn KeepsTheCatchUpHalf,
    job: &Identity,
) -> Result<(), String> {
    for mut record in keeper.every_standing_notice() {
        let before = record.entries.len();
        record
            .entries
            .retain(|entry| !(it_names_a_dropped_job(entry) && entry.recognised_by().0 == job));
        if record.entries.len() == before {
            continue;
        }
        let it_still_names_something = record
            .entries
            .iter()
            .any(|entry| it_names_a_dropped_job(entry) || matches!(entry, ANoticeEntry::Ended { .. }));
        if it_still_names_something {
            keeper.write_the_notice(&record)?;
        } else {
            keeper.remove_the_notice(&record.server)?;
        }
    }
    Ok(())
}

/// **C2's lifetime, performed: a server row removed takes its record with
/// it**, and the message is one group shorter the next time it is drawn.
/// **Nothing is left naming a row that is gone.**
pub fn the_row_was_removed(
    keeper: &mut dyn KeepsTheCatchUpHalf,
    server: &Identity,
) -> Result<(), String> {
    keeper.remove_the_notice(server)
}

/// **C6 and C7's holder: the last-known queue readings, in process memory for
/// the connected period and nowhere else.**
///
/// **There is no durable form of this type and no path in it.** A fresh launch
/// holds none; a period that ends drops the record entirely.
#[derive(Debug, Default)]
pub struct ThePeriodHeldReadings {
    per_server: HashMap<String, AReadingHeldForThePeriod>,
}

/// One machine's last-known reading, with what C7 marks it with.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AReadingHeldForThePeriod {
    pub reading: Reading<Vec<AJobAsTheMachineReadsIt>>,
    /// **C7's trigger: the moment `Servers` declares the break and tears the
    /// connection down** (its K3 and K5) — never the period ending, which
    /// drops the record entirely, and never the first failed run of attempts.
    pub the_break_is_declared: bool,
    /// **`Servers` Q19's fourth branch**: a connection holding a reading taken
    /// on an earlier connection, with §7 not yet complete. **The mark is about
    /// the reading's age, never about the connection's state.**
    pub reconciliation_has_completed_on_this_connection: bool,
}

impl ThePeriodHeldReadings {
    /// **A fresh reading, taken by `Servers` Q1 step 7, which starts a fresh
    /// sequence and clears C7's mark.**
    pub fn a_fresh_reading_was_taken(
        &mut self,
        server: &Identity,
        reading: Reading<Vec<AJobAsTheMachineReadsIt>>,
        reconciliation_has_completed_on_this_connection: bool,
    ) {
        self.per_server.insert(
            server.as_text(),
            AReadingHeldForThePeriod {
                reading,
                the_break_is_declared: false,
                reconciliation_has_completed_on_this_connection,
            },
        );
    }

    /// **C7: the stamp and the not-updating statement appear at the moment the
    /// break is declared.**
    pub fn the_break_was_declared(&mut self, server: &Identity) {
        if let Some(held) = self.per_server.get_mut(&server.as_text()) {
            held.the_break_is_declared = true;
        }
    }

    /// **§7 completing on this connection clears Q19's half of the mark.**
    pub fn reconciliation_completed(&mut self, server: &Identity) {
        if let Some(held) = self.per_server.get_mut(&server.as_text()) {
            held.reconciliation_has_completed_on_this_connection = true;
        }
    }

    /// **C6: the period ending drops the record entirely.** Not stamped, not
    /// kept, not written anywhere.
    pub fn the_period_ended(&mut self, server: &Identity) {
        self.per_server.remove(&server.as_text());
    }

    /// The reading Smoozy holds for a machine, or nothing.
    pub fn held_for(&self, server: &Identity) -> Option<&AReadingHeldForThePeriod> {
        self.per_server.get(&server.as_text())
    }

    /// **C7, performed over what is held** — the judgement is
    /// [`crate::two_records::how_the_reading_is_drawn`]'s and is not made
    /// here.
    pub fn how_it_is_drawn(&self, server: &Identity) -> Option<HowTheReadingIsDrawn> {
        let held = self.held_for(server)?;
        let moment = Reading::taken((), held.reading.taken_at(), held.reading.clock().clone());
        Some(crate::two_records::how_the_reading_is_drawn(
            &moment,
            held.the_break_is_declared,
            held.reconciliation_has_completed_on_this_connection,
        ))
    }
}

/// **C8's six, with the store each sits in.**
///
/// **Each is Smoozy's own record of Smoozy's own acts, or one of `State,
/// identity and recovery` §3 D4's three named exceptions**, and **none is a
/// copy of a machine state the machine still holds.**
pub fn the_store_each_survivor_sits_in(survivor: WhatSurvivesThePeriod) -> &'static str {
    match survivor {
        WhatSurvivesThePeriod::TheHistoryRecord => "the local record's job records",
        WhatSurvivesThePeriod::TheRememberedServerState => "the local record's server rows",
        WhatSurvivesThePeriod::TheTombstones => "the local record's history tombstones",
        WhatSurvivesThePeriod::TheEventCursor => "the local record's event cursor rows",
        WhatSurvivesThePeriod::TheCatchUpNoticeRecord => "the local record's notice rows",
        WhatSurvivesThePeriod::TheIdleSpokenMark => "the local record's idle-spoken marks",
    }
}

/// **C8, stated as a value: every survivor sits in the local record, and this
/// document opens no store outside `State, identity and recovery` §1.1's
/// inventory.**
///
/// **Two of the six are record kinds §1.1's inventory does not name** — the
/// catch-up notice record and the idle-spoken mark — and **that is the
/// store-inventory seam five documents have found, recorded and never repaired
/// here.** Nothing in this module widens that inventory.
pub const EVERY_SURVIVOR_SITS_IN_THE_LOCAL_RECORD: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ending::Ending;
    use smoozy_state::identities::IdentityKind;
    use smoozy_state::lifetimes::Lifetime;
    use smoozy_state::observed::{Clock, MachineClock};
    use std::time::SystemTime;

    fn a_server() -> Identity {
        Identity::issue(IdentityKind::LocalServer).expect("randomness")
    }

    fn an_installation() -> Identity {
        Identity::issue(IdentityKind::Installation).expect("randomness")
    }

    fn a_job() -> Identity {
        Identity::issue(IdentityKind::Job).expect("randomness")
    }

    /// A local record in hand, which records the order its writes arrived in.
    #[derive(Default)]
    struct TheRecordInHand {
        notices: HashMap<String, NoticeRecord>,
        cursors: HashMap<String, EventCursor>,
        in_the_order_they_landed: Vec<&'static str>,
        the_notice_write_is_refused: bool,
    }

    impl KeepsTheCatchUpHalf for TheRecordInHand {
        fn the_standing_notice(&self, server: &Identity) -> Option<NoticeRecord> {
            self.notices.get(&server.as_text()).cloned()
        }

        fn write_the_notice(&mut self, record: &NoticeRecord) -> Result<(), String> {
            if self.the_notice_write_is_refused {
                return Err("No space left on device (os error 28)".to_string());
            }
            self.in_the_order_they_landed.push("the notice");
            self.notices.insert(record.server.as_text(), record.clone());
            Ok(())
        }

        fn remove_the_notice(&mut self, server: &Identity) -> Result<(), String> {
            self.notices.remove(&server.as_text());
            Ok(())
        }

        fn every_standing_notice(&self) -> Vec<NoticeRecord> {
            self.notices.values().cloned().collect()
        }

        fn the_cursor(&self, server: &Identity) -> Option<EventCursor> {
            self.cursors.get(&server.as_text()).cloned()
        }

        fn advance_the_cursor(
            &mut self,
            server: &Identity,
            cursor: &EventCursor,
        ) -> Result<(), String> {
            self.in_the_order_they_landed.push("the cursor");
            self.cursors.insert(server.as_text(), cursor.clone());
            Ok(())
        }
    }

    fn tracked(jobs: &[&Identity]) -> HashSet<Identity> {
        jobs.iter().map(|job| (*job).clone()).collect()
    }

    /// **C2: the notice record is written before the cursor advances past the
    /// events it names.**
    #[test]
    fn the_notice_is_written_before_the_cursor_advances() {
        let mut keeper = TheRecordInHand::default();
        let server = a_server();
        let installation = an_installation();
        let job = a_job();
        let left = the_read_forward(
            &mut keeper,
            &server,
            &installation,
            &[AnAppliedEvent::AJobEnded {
                job: job.clone(),
                ending: Ending::Finished,
                sequence: 1,
            }],
            &tracked(&[&job]),
            17,
            false,
        )
        .expect("applied");
        assert_eq!(
            keeper.in_the_order_they_landed,
            vec!["the notice", "the cursor"]
        );
        assert_eq!(left.cursor.past_sequence_number, 17);
        assert_eq!(left.the_account.expect("composed").entries.len(), 1);
    }

    /// **The order is the promise: a refused notice write leaves the cursor
    /// where it stood**, so the events it named are read again.
    #[test]
    fn a_refused_notice_write_leaves_the_cursor_where_it_stood() {
        let mut keeper = TheRecordInHand {
            the_notice_write_is_refused: true,
            ..TheRecordInHand::default()
        };
        let server = a_server();
        let installation = an_installation();
        let job = a_job();
        let refused = the_read_forward(
            &mut keeper,
            &server,
            &installation,
            &[AnAppliedEvent::AJobEnded {
                job: job.clone(),
                ending: Ending::Failed,
                sequence: 1,
            }],
            &tracked(&[&job]),
            9,
            false,
        );
        assert!(refused.is_err());
        assert_eq!(keeper.the_cursor(&server), None, "nothing advanced");
    }

    /// **C1a: events applied while an undismissed record stands are composed
    /// into that record**, so the overnight case is one account of three
    /// endings rather than three accounts of one.
    #[test]
    fn a_second_pass_grows_the_standing_record_rather_than_raising_a_second() {
        let mut keeper = TheRecordInHand::default();
        let server = a_server();
        let installation = an_installation();
        let first = a_job();
        let second = a_job();
        the_read_forward(
            &mut keeper,
            &server,
            &installation,
            &[AnAppliedEvent::AJobEnded {
                job: first.clone(),
                ending: Ending::Finished,
                sequence: 1,
            }],
            &tracked(&[&first, &second]),
            1,
            false,
        )
        .expect("applied");
        the_read_forward(
            &mut keeper,
            &server,
            &installation,
            &[AnAppliedEvent::AJobEnded {
                job: second.clone(),
                ending: Ending::Failed,
                sequence: 1,
            }],
            &tracked(&[&first, &second]),
            2,
            false,
        )
        .expect("applied");
        assert_eq!(keeper.every_standing_notice().len(), 1, "one record");
        assert_eq!(
            keeper
                .the_standing_notice(&server)
                .expect("there")
                .entries
                .len(),
            2
        );
    }

    /// **C3: a new installation reads from the beginning of its own journal,
    /// never from a cursor taken under a dead one.**
    #[test]
    fn a_new_installation_reads_from_the_beginning_of_its_own_journal() {
        let mut keeper = TheRecordInHand::default();
        let server = a_server();
        let first_installation = an_installation();
        the_read_forward(
            &mut keeper,
            &server,
            &first_installation,
            &[],
            &HashSet::new(),
            40,
            false,
        )
        .expect("applied");
        let second_installation = an_installation();
        let left = the_read_forward(
            &mut keeper,
            &server,
            &second_installation,
            &[],
            &HashSet::new(),
            0,
            false,
        )
        .expect("applied");
        assert_eq!(left.read_from, ReadFrom::TheBeginningOfItsOwnJournal);
        // And under the same installation it reads on from the cursor.
        let left = the_read_forward(
            &mut keeper,
            &server,
            &second_installation,
            &[],
            &HashSet::new(),
            5,
            false,
        )
        .expect("applied");
        assert_eq!(left.read_from, ReadFrom::AfterTheCursor(1));
    }

    /// **C2's lifetime: a server row removed takes its record with it, and
    /// nothing is left naming a row that is gone.**
    #[test]
    fn a_removed_row_takes_its_notice_with_it() {
        let mut keeper = TheRecordInHand::default();
        let server = a_server();
        let job = a_job();
        the_read_forward(
            &mut keeper,
            &server,
            &an_installation(),
            &[AnAppliedEvent::AJobEnded {
                job: job.clone(),
                ending: Ending::Finished,
                sequence: 1,
            }],
            &tracked(&[&job]),
            1,
            false,
        )
        .expect("applied");
        the_row_was_removed(&mut keeper, &server).expect("removed");
        assert!(keeper.every_standing_notice().is_empty());
    }

    /// **C2a as changed 2026-10-12, the Creator's ninth notes, note 3:
    /// opening History takes every ending (and every re-run) off every
    /// standing record; a record left naming nothing is dismissed, and one
    /// naming a dropped job stands with that line alone until it is
    /// dismissed, which takes that one name off it.** A record that gains an
    /// ending after History was opened keeps it, and still stands after its
    /// dropped line goes; once it names nothing it is dismissed.
    ///
    /// **What it bites on**: an ending left on a record after History is
    /// opened (the count would not clear), a dropped job's line taken by
    /// opening History, a record left standing with nothing in it, a
    /// dropped line's dismissal taking the whole record or another dropped
    /// job's line, and a record kept once it names nothing.
    #[test]
    fn opening_history_takes_the_endings_and_a_dropped_line_goes_only_by_its_own_dismiss() {
        let mut keeper = TheRecordInHand::default();
        let (one, two, three) = (a_server(), a_server(), a_server());
        let (finished, failed, re_run, kitchen, attic) = (a_job(), a_job(), a_job(), a_job(), a_job());
        let ended = |job: &Identity, ending, sequence| ANoticeEntry::Ended { job: job.clone(), ending, sequence };
        let held_dropped = |job: &Identity, name: &str, sequence| ANoticeEntry::HeldJobDropped {
            job: job.clone(),
            name: name.to_string(),
            why: "the hold's release outcome was written: dropped".to_string(),
            sequence,
        };
        // One server with two endings, a re-run and a dropped job; one with
        // endings alone; one with a job dropped for a removed Blender and the
        // gap stated.
        for record in [
            NoticeRecord {
                server: one.clone(),
                entries: vec![
                    ended(&finished, Ending::Finished, 1),
                    ANoticeEntry::InterruptedAndReRunFrom { job: re_run.clone(), attempt: 2, frame: Some(61), sequence: 2 },
                    held_dropped(&kitchen, "Kitchen — final v4", 3),
                    ended(&failed, Ending::Failed, 4),
                ],
                the_account_is_incomplete: false,
            },
            NoticeRecord {
                server: two.clone(),
                entries: vec![ended(&a_job(), Ending::Stopped, 1), ended(&a_job(), Ending::GivenUp, 2)],
                the_account_is_incomplete: false,
            },
            NoticeRecord {
                server: three.clone(),
                entries: vec![ANoticeEntry::WaitingJobDroppedForARemovedBlender {
                    job: attic.clone(),
                    a_bounded_rendering_of_the_label: "Attic".to_string(),
                    the_version: "4.5.3".to_string(),
                    sequence: 7,
                }],
                the_account_is_incomplete: true,
            },
        ] {
            keeper.write_the_notice(&record).expect("written");
        }

        history_was_opened(&mut keeper).expect("opened");
        assert_eq!(
            keeper.the_standing_notice(&one).map(|record| record.entries),
            Some(vec![held_dropped(&kitchen, "Kitchen — final v4", 3)]),
            "the endings and the re-run stay, or the dropped line went"
        );
        assert_eq!(keeper.the_standing_notice(&two), None, "a record naming nothing still stands");
        let attics = keeper.the_standing_notice(&three).expect("a dropped job's record went with History");
        assert_eq!(attics.entries.len(), 1);
        assert!(attics.the_account_is_incomplete);

        // **An ending learned after History was opened** joins the record
        // that still stands for its dropped job.
        let later = a_job();
        let mut grown = keeper.the_standing_notice(&one).expect("standing");
        grown.entries.push(ended(&later, Ending::Finished, 9));
        keeper.write_the_notice(&grown).expect("written");

        // **Dismissing one dropped line takes that name alone**: the Attic
        // line is untouched, and server one's record stands for its ending.
        the_dropped_line_is_dismissed(&mut keeper, &kitchen).expect("dismissed");
        assert_eq!(
            keeper.the_standing_notice(&one).map(|record| record.entries),
            Some(vec![ended(&later, Ending::Finished, 9)])
        );
        assert!(keeper.the_standing_notice(&three).is_some(), "another dropped job's line went");
        // A job no record names changes nothing.
        the_dropped_line_is_dismissed(&mut keeper, &finished).expect("nothing to take");
        assert_eq!(keeper.every_standing_notice().len(), 2);

        // **Left naming nothing, a record is dismissed** — by its last
        // dropped line, or by History taking its last ending.
        the_dropped_line_is_dismissed(&mut keeper, &attic).expect("dismissed");
        assert_eq!(keeper.the_standing_notice(&three), None);
        history_was_opened(&mut keeper).expect("opened");
        assert!(keeper.every_standing_notice().is_empty());
    }

    fn a_reading(at: u64) -> Reading<Vec<AJobAsTheMachineReadsIt>> {
        Reading::taken(
            Vec::new(),
            SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(at),
            Clock::AMachine(MachineClock("server-1".to_string())),
        )
    }

    /// **C7: the mark appears at the declared break, and a fresh reading
    /// clears it.**
    #[test]
    fn the_reading_is_live_until_the_break_is_declared_and_fresh_again_after() {
        let mut held = ThePeriodHeldReadings::default();
        let server = a_server();
        assert_eq!(
            held.how_it_is_drawn(&server),
            None,
            "a fresh launch holds none"
        );
        held.a_fresh_reading_was_taken(&server, a_reading(10), true);
        assert_eq!(
            held.how_it_is_drawn(&server),
            Some(HowTheReadingIsDrawn::Live)
        );
        held.the_break_was_declared(&server);
        assert!(matches!(
            held.how_it_is_drawn(&server),
            Some(HowTheReadingIsDrawn::LastKnown { .. })
        ));
        held.a_fresh_reading_was_taken(&server, a_reading(20), true);
        assert_eq!(
            held.how_it_is_drawn(&server),
            Some(HowTheReadingIsDrawn::Live)
        );
    }

    /// **`Servers` Q19's fourth branch: a reading held while §7 has not
    /// completed carries the mark too**, because the mark is about the
    /// reading's age.
    #[test]
    fn a_reading_held_on_an_unreconciled_connection_carries_the_mark() {
        let mut held = ThePeriodHeldReadings::default();
        let server = a_server();
        held.a_fresh_reading_was_taken(&server, a_reading(10), false);
        assert!(matches!(
            held.how_it_is_drawn(&server),
            Some(HowTheReadingIsDrawn::LastKnown { .. })
        ));
        held.reconciliation_completed(&server);
        assert_eq!(
            held.how_it_is_drawn(&server),
            Some(HowTheReadingIsDrawn::Live)
        );
    }

    /// **C6: the period ending drops the record entirely, so there is no
    /// durable cache anywhere that could be shown hours stale.**
    #[test]
    fn the_period_ending_drops_the_reading_rather_than_stamping_it() {
        let mut held = ThePeriodHeldReadings::default();
        let server = a_server();
        held.a_fresh_reading_was_taken(&server, a_reading(10), true);
        held.the_period_ended(&server);
        assert_eq!(held.held_for(&server), None);
        assert_eq!(held.how_it_is_drawn(&server), None);
        assert_eq!(
            crate::two_records::THE_LAST_KNOWN_READINGS_LIFETIME,
            Lifetime::PeriodHeld
        );
    }

    /// **C8: all six survivors sit in the local record, and none is a copy of
    /// a machine state the machine still holds.**
    #[test]
    fn every_survivor_sits_in_the_local_record_and_none_is_a_copy() {
        for survivor in WhatSurvivesThePeriod::ALL {
            let store = the_store_each_survivor_sits_in(survivor);
            assert!(
                store.starts_with("the local record"),
                "{survivor:?} sits at {store}"
            );
            assert!(!survivor.is_a_copy_of_a_machine_state_the_machine_still_holds());
            assert_ne!(survivor.lifetime(), Lifetime::PeriodHeld);
        }
        const _: () = assert!(EVERY_SURVIVOR_SITS_IN_THE_LOCAL_RECORD);
    }
}
