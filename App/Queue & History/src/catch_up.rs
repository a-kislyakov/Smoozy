//! The catch-up notice — `Queue & History` §11 C1–C5.
//!
//! **Constraint:** Row 77 — the catch-up notice, told once on the connection
//! that learns it. `State, identity and recovery` §7 step 3's cursor;
//! `Desktop app and delivery` T18's boundary — everything applied at step 3
//! is the catch-up notice's and posts no notification; its T22 — the notice
//! is a durable in-app record.
//!
//! **What is here is the composition**: from applied events to the notice's
//! content, the standing record it is added to, the message drawn over the
//! records, and the cursor's rule. **The read-forward, the application of
//! events and the durable write are the acting half's.** **No clock is
//! involved anywhere below** (C1a, C3): no function takes a moment.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Desktop app and delivery` T18, T19, T22.

use std::collections::HashSet;

use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_state::lifetimes::Lifetime;

use crate::ending::Ending;

/// **The events §7 step 3 applies, as far as the notice reads them** — and
/// C1's own list of what the notice names.
///
/// **Each carries the event's own per-machine sequence number** (`State,
/// identity and recovery` E1's), because C2's ordering — the notice before
/// the cursor — means a pass that is interrupted reads the same events
/// forward again, and reading them again is what must not duplicate them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnAppliedEvent {
    /// A job ended, with E2's ending — finished and when, failed, given up,
    /// stopped. A job that simply rendered to the end is named like any
    /// other, because it is the ordinary case.
    AJobEnded {
        job: Identity,
        ending: Ending,
        sequence: u64,
    },
    /// **Interrupted and re-run from a given frame.** `attempt` is the
    /// re-run's own attempt number, which names the attempt record its frame
    /// list sits in (`Queue & History` N1: *"the first frame of the re-run
    /// attempt's list"*). **`frame` is absent where that list had not been read
    /// when the event was applied** — the render worker writes it after the
    /// start is journalled — which is not a zero, and is filled in by
    /// [`NoticeRecord::the_re_runs_frame_is_known`] once it is read.
    InterruptedAndReRunFrom {
        job: Identity,
        attempt: u32,
        frame: Option<i64>,
        sequence: u64,
    },
    /// **A held job dropped, and why (H11)**, with the name it is told by —
    /// read before the drop took the local record, because after it **the
    /// notice is the only place the job is named** and nothing else is left
    /// that holds the name.
    AHeldJobWasDropped {
        job: Identity,
        name: String,
        why: String,
        sequence: u64,
    },
    /// A waiting job dropped for a Blender removed under it, named with the
    /// version it was submitted with (L22).
    AWaitingJobWasDroppedForARemovedBlender {
        job: Identity,
        a_bounded_rendering_of_the_label: String,
        the_version: String,
        sequence: u64,
    },
    /// An event the notice does not name — a render start, a derivative's
    /// production, a hold placed — applied and composed into nothing.
    SomethingTheNoticeDoesNotName,
}

/// One line of the notice's content, about one job this desktop tracks.
///
/// **Every arm carries the job identity and the event's own per-machine
/// sequence number**, and the pair is what recognises an entry the standing
/// record already holds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ANoticeEntry {
    Ended {
        job: Identity,
        ending: Ending,
        sequence: u64,
    },
    InterruptedAndReRunFrom {
        job: Identity,
        attempt: u32,
        frame: Option<i64>,
        sequence: u64,
    },
    HeldJobDropped {
        job: Identity,
        name: String,
        why: String,
        sequence: u64,
    },
    WaitingJobDroppedForARemovedBlender {
        job: Identity,
        a_bounded_rendering_of_the_label: String,
        the_version: String,
        sequence: u64,
    },
}

impl ANoticeEntry {
    /// **What recognises an entry: the job identity every arm already carries,
    /// plus the event's own per-machine sequence number.**
    ///
    /// The same shape [`crate::record::LocalJobRecord::the_read_fills_what_it_lacks`]
    /// already uses for the per-attempt marks it merges.
    pub fn recognised_by(&self) -> (&Identity, u64) {
        match self {
            ANoticeEntry::Ended { job, sequence, .. }
            | ANoticeEntry::InterruptedAndReRunFrom { job, sequence, .. }
            | ANoticeEntry::HeldJobDropped { job, sequence, .. }
            | ANoticeEntry::WaitingJobDroppedForARemovedBlender { job, sequence, .. } => {
                (job, *sequence)
            }
        }
    }
}

/// **C2: the notice is a durable record of Smoozy's own composition, written
/// before it is shown** — singular per machine, standing until the user's
/// own dismissal. **Its lifetime class is subject-held on the server row**:
/// it is about one machine's journal and it ends with the row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoticeRecord {
    pub server: Identity,
    pub entries: Vec<ANoticeEntry>,
    /// **C4: a gap is stated, never smoothed** — where the journal's oldest
    /// retained sequence number has passed the cursor, the notice says the
    /// account is incomplete rather than presenting what arrived as the whole
    /// story.
    pub the_account_is_incomplete: bool,
}

impl NoticeRecord {
    pub const LIFETIME: Lifetime = Lifetime::SubjectHeld;

    /// **The re-runs this record names whose frame has not been read yet** —
    /// each job with the attempt whose record holds its list.
    pub fn the_re_runs_whose_frame_is_not_known(&self) -> Vec<(Identity, u32)> {
        self.entries
            .iter()
            .filter_map(|entry| match entry {
                ANoticeEntry::InterruptedAndReRunFrom {
                    job,
                    attempt,
                    frame: None,
                    ..
                } => Some((job.clone(), *attempt)),
                _ => None,
            })
            .collect()
    }

    /// **A re-run's frame, read later than its event** — `Queue & History` N1:
    /// the first frame of that attempt's own list. **Only an entry that has no
    /// frame yet takes one**, so a frame once said is never replaced. Answers
    /// whether anything changed.
    pub fn the_re_runs_frame_is_known(&mut self, of: &Identity, at_attempt: u32, first: i64) -> bool {
        let mut changed = false;
        for entry in &mut self.entries {
            if let ANoticeEntry::InterruptedAndReRunFrom {
                job,
                attempt,
                frame: frame @ None,
                ..
            } = entry
            {
                if job == of && *attempt == at_attempt {
                    *frame = Some(first);
                    changed = true;
                }
            }
        }
        changed
    }
}

/// **C1 and C1a, performed: the account is composed from the applied events,
/// not from the connection, so it is composed the same way whether or not a
/// period broke.**
///
/// **It is idempotent, and it has to be**: an entry is added only where the
/// standing record does not already hold it, recognised by the job identity
/// and the event's own per-machine sequence number. C2's ordering — the notice
/// written before the cursor advances — means a pass interrupted between the
/// two reads the same events forward again on the next connection, and the
/// ordering is right and does not move: it exists so the events *can* be read
/// again. Everything applied by either pass — §7 step 3 on a
/// connection, or the read-forward inside a standing period — is the
/// catch-up account. **Events applied while an undismissed record stands are
/// composed into that record; a fresh record is composed only where none
/// stands.** Only jobs this desktop tracks are named: a job the local record
/// does not name is told to nobody (L22, A2).
///
/// Returns the record to write, or `None` where nothing stands and nothing
/// new was applied — which is C2's *a notice only of what is new, or none*.
pub fn compose_the_account(
    standing: Option<NoticeRecord>,
    server: &Identity,
    applied: &[AnAppliedEvent],
    tracked: &HashSet<Identity>,
    the_journals_oldest_retained_has_passed_the_cursor: bool,
) -> Option<NoticeRecord> {
    debug_assert_eq!(server.kind(), IdentityKind::LocalServer);
    let mut record = standing.unwrap_or(NoticeRecord {
        server: server.clone(),
        entries: Vec::new(),
        the_account_is_incomplete: false,
    });
    let before = record.entries.len();
    for event in applied {
        let entry = match event {
            AnAppliedEvent::AJobEnded {
                job,
                ending,
                sequence,
            } if tracked.contains(job) => ANoticeEntry::Ended {
                job: job.clone(),
                ending: *ending,
                sequence: *sequence,
            },
            AnAppliedEvent::InterruptedAndReRunFrom {
                job,
                attempt,
                frame,
                sequence,
            } if tracked.contains(job) => ANoticeEntry::InterruptedAndReRunFrom {
                job: job.clone(),
                attempt: *attempt,
                frame: *frame,
                sequence: *sequence,
            },
            AnAppliedEvent::AHeldJobWasDropped {
                job,
                name,
                why,
                sequence,
            } if tracked.contains(job) => ANoticeEntry::HeldJobDropped {
                job: job.clone(),
                name: name.clone(),
                why: why.clone(),
                sequence: *sequence,
            },
            AnAppliedEvent::AWaitingJobWasDroppedForARemovedBlender {
                job,
                a_bounded_rendering_of_the_label,
                the_version,
                sequence,
            } if tracked.contains(job) => ANoticeEntry::WaitingJobDroppedForARemovedBlender {
                job: job.clone(),
                a_bounded_rendering_of_the_label: a_bounded_rendering_of_the_label.clone(),
                the_version: the_version.clone(),
                sequence: *sequence,
            },
            _ => continue,
        };
        // **An entry the standing record already holds is not added again.**
        // C2 puts the notice's write before the cursor's advance precisely so
        // the events can be read again after an interruption, and reading them
        // again is what must stop duplicating them.
        if record
            .entries
            .iter()
            .any(|held| held.recognised_by() == entry.recognised_by())
        {
            continue;
        }
        record.entries.push(entry);
    }
    if the_journals_oldest_retained_has_passed_the_cursor {
        record.the_account_is_incomplete = true;
    }
    let nothing_new =
        record.entries.len() == before && !the_journals_oldest_retained_has_passed_the_cursor;
    if record.entries.is_empty() && !record.the_account_is_incomplete {
        return None;
    }
    if nothing_new && before == 0 {
        return None;
    }
    Some(record)
}

/// **T18's boundary, from this side**: the account posts no notification.
pub const THE_ACCOUNT_POSTS_NO_NOTIFICATION: bool = true;

/// **C2a: one message spanning several servers is a presentation over the
/// standing records and never a record of its own.** The message is every
/// standing, undismissed notice record at the moment it is drawn, grouped by
/// server — one group per record; the heading's count is composed at draw
/// time from the same set and stored nowhere.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheMessage<'a> {
    pub groups: Vec<&'a NoticeRecord>,
    /// Jobs across records.
    pub jobs: usize,
    /// Servers, as records.
    pub servers: usize,
}

pub fn the_message(standing: &[NoticeRecord]) -> TheMessage<'_> {
    let groups: Vec<&NoticeRecord> = standing.iter().collect();
    TheMessage {
        jobs: groups.iter().map(|record| record.entries.len()).sum(),
        servers: groups.len(),
        groups,
    }
}

/// **C2a: dismissing the message dismisses every record behind it, each on
/// its own terms; dismissing one group dismisses that record alone.** Both
/// are the user's own act on Smoozy's own record. Returns the records that
/// go.
pub fn dismiss(standing: Vec<NoticeRecord>, what: Dismissal) -> Vec<NoticeRecord> {
    match what {
        Dismissal::TheWholeMessage => Vec::new(),
        Dismissal::OneGroup(server) => standing
            .into_iter()
            .filter(|record| record.server != server)
            .collect(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Dismissal {
    TheWholeMessage,
    OneGroup(Identity),
}

/// **C2a: a server removed while its group stands takes its record with it**,
/// per C2's own lifetime, and the message is one group shorter the next
/// time it is drawn.
pub fn the_row_was_removed(standing: Vec<NoticeRecord>, server: &Identity) -> Vec<NoticeRecord> {
    standing
        .into_iter()
        .filter(|record| &record.server != server)
        .collect()
}

/// **C3: the point-since is the event cursor and nothing else.** Durable on
/// the desktop against the server row and the installation identity it was
/// taken under; never advanced on receipt; **a new installation reads from
/// the beginning of its own journal, never from a cursor taken under a dead
/// one.** No clock is involved, deliberately — `Desktop app and delivery`
/// T19's rule, kept.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventCursor {
    pub taken_under: Identity,
    pub past_sequence_number: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadFrom {
    /// The sequence number after the cursor's.
    AfterTheCursor(u64),
    /// From the beginning of the installation's own journal.
    TheBeginningOfItsOwnJournal,
}

pub fn where_to_read_from(
    cursor: Option<&EventCursor>,
    the_installation_now: &Identity,
) -> ReadFrom {
    match cursor {
        Some(cursor) if &cursor.taken_under == the_installation_now => {
            ReadFrom::AfterTheCursor(cursor.past_sequence_number + 1)
        }
        _ => ReadFrom::TheBeginningOfItsOwnJournal,
    }
}

/// **C4's test**: the journal's oldest retained sequence number has passed
/// the cursor — the next number the desktop would read is no longer held.
pub fn the_oldest_retained_has_passed(cursor_past: u64, oldest_retained: u64) -> bool {
    oldest_retained > cursor_past + 1
}

/// **C5: where the machine is gone before the next connection, there is no
/// notice, and the entries say what they know** — the tracked jobs that had
/// started rendering read *no ending Smoozy ever learned*; nothing is
/// invented to fill the silence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhereTheMachineIsGone {
    NoNotice,
}

pub fn where_the_machine_is_gone() -> WhereTheMachineIsGone {
    WhereTheMachineIsGone::NoNotice
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::line::testing::a_job;
    use crate::record::tests::a_server;

    fn tracked(jobs: &[&Identity]) -> HashSet<Identity> {
        jobs.iter().map(|job| (*job).clone()).collect()
    }

    #[test]
    fn the_account_names_what_the_events_say_about_tracked_jobs_and_nothing_about_untracked_ones() {
        let server = a_server();
        let mine = a_job();
        let theirs = a_job();
        let applied = vec![
            AnAppliedEvent::AJobEnded {
                job: mine.clone(),
                ending: Ending::Finished,
                sequence: 11,
            },
            AnAppliedEvent::AJobEnded {
                job: theirs.clone(),
                ending: Ending::Failed,
                sequence: 12,
            },
            AnAppliedEvent::SomethingTheNoticeDoesNotName,
            AnAppliedEvent::AWaitingJobWasDroppedForARemovedBlender {
                job: mine.clone(),
                a_bounded_rendering_of_the_label: "Kitchen".to_string(),
                the_version: "4.5.12".to_string(),
                sequence: 13,
            },
        ];
        let record =
            compose_the_account(None, &server, &applied, &tracked(&[&mine]), false).unwrap();
        assert_eq!(record.entries.len(), 2);
        assert!(
            matches!(&record.entries[0], ANoticeEntry::Ended { job, ending: Ending::Finished, sequence: 11 } if job == &mine)
        );
        assert!(!record.the_account_is_incomplete);
        assert_eq!(NoticeRecord::LIFETIME, Lifetime::SubjectHeld);
    }

    #[test]
    fn events_applied_while_a_record_stands_are_composed_into_it_and_a_fresh_one_only_where_none_stands(
    ) {
        let server = a_server();
        let one = a_job();
        let two = a_job();
        let first = compose_the_account(
            None,
            &server,
            &[AnAppliedEvent::AJobEnded {
                job: one.clone(),
                ending: Ending::Finished,
                sequence: 1,
            }],
            &tracked(&[&one, &two]),
            false,
        )
        .unwrap();
        // The overnight case: a second pass inside the same standing period.
        let grown = compose_the_account(
            Some(first),
            &server,
            &[AnAppliedEvent::AJobEnded {
                job: two.clone(),
                ending: Ending::GivenUp,
                sequence: 2,
            }],
            &tracked(&[&one, &two]),
            false,
        )
        .unwrap();
        assert_eq!(
            grown.entries.len(),
            2,
            "one account of two endings, not two accounts of one"
        );
    }

    /// **C2's ordering is right and does not move: the notice is written
    /// before the cursor advances precisely so the events can be read again,
    /// and reading them again is what must stop duplicating them.**
    ///
    /// **The fault this catches is the shape a crash between the two leaves**:
    /// the account was written, the cursor was not, and the next connection
    /// reads the same events forward. Every event here is applied twice, and
    /// the second pass adds nothing.
    #[test]
    fn an_event_read_forward_twice_is_composed_into_the_account_once() {
        let server = a_server();
        let one = a_job();
        let two = a_job();
        let applied = vec![
            AnAppliedEvent::AJobEnded {
                job: one.clone(),
                ending: Ending::Finished,
                sequence: 7,
            },
            AnAppliedEvent::AHeldJobWasDropped {
                job: two.clone(),
                name: "Kitchen — final v4".to_string(),
                why: "the dependency was cancelled".to_string(),
                sequence: 8,
            },
        ];
        let who = tracked(&[&one, &two]);
        let first = compose_the_account(None, &server, &applied, &who, false).unwrap();
        assert_eq!(first.entries.len(), 2);
        let again = compose_the_account(Some(first.clone()), &server, &applied, &who, false);
        assert_eq!(
            again,
            Some(first.clone()),
            "the same events read again add nothing"
        );
        // **A later event of the same job is still new**, because the
        // sequence number is half of what recognises an entry.
        let later = compose_the_account(
            Some(first),
            &server,
            &[AnAppliedEvent::AJobEnded {
                job: one.clone(),
                ending: Ending::Finished,
                sequence: 9,
            }],
            &who,
            false,
        )
        .unwrap();
        assert_eq!(later.entries.len(), 3);
    }

    #[test]
    fn nothing_new_and_nothing_standing_composes_no_notice() {
        let server = a_server();
        assert_eq!(
            compose_the_account(
                None,
                &server,
                &[AnAppliedEvent::SomethingTheNoticeDoesNotName],
                &HashSet::new(),
                false
            ),
            None
        );
        // But a standing record with nothing new still stands.
        let standing = NoticeRecord {
            server: server.clone(),
            entries: vec![ANoticeEntry::HeldJobDropped {
                job: a_job(),
                name: "Kitchen — final v4".to_string(),
                why: "the dependency was cancelled".to_string(),
                sequence: 4,
            }],
            the_account_is_incomplete: false,
        };
        assert_eq!(
            compose_the_account(Some(standing.clone()), &server, &[], &HashSet::new(), false),
            Some(standing)
        );
    }

    #[test]
    fn a_gap_is_stated_and_never_smoothed() {
        let server = a_server();
        let record = compose_the_account(None, &server, &[], &HashSet::new(), true).unwrap();
        assert!(record.the_account_is_incomplete);
        assert!(the_oldest_retained_has_passed(10, 12));
        assert!(!the_oldest_retained_has_passed(10, 11));
        assert!(!the_oldest_retained_has_passed(10, 3));
    }

    #[test]
    fn the_message_is_every_standing_record_grouped_by_server_and_counted_at_the_draw() {
        let a = a_server();
        let b = a_server();
        let standing = vec![
            NoticeRecord {
                server: a.clone(),
                entries: vec![
                    ANoticeEntry::Ended {
                        job: a_job(),
                        ending: Ending::Finished,
                        sequence: 1,
                    },
                    ANoticeEntry::Ended {
                        job: a_job(),
                        ending: Ending::Stopped,
                        sequence: 2,
                    },
                ],
                the_account_is_incomplete: false,
            },
            NoticeRecord {
                server: b.clone(),
                entries: vec![ANoticeEntry::InterruptedAndReRunFrom {
                    job: a_job(),
                    attempt: 2,
                    frame: Some(61),
                    sequence: 3,
                }],
                the_account_is_incomplete: false,
            },
        ];
        let message = the_message(&standing);
        assert_eq!(message.servers, 2);
        assert_eq!(message.jobs, 3);
        // Dismissing one group leaves the other; dismissing the message
        // takes every record.
        let one_less = dismiss(standing.clone(), Dismissal::OneGroup(a.clone()));
        assert_eq!(one_less.len(), 1);
        assert_eq!(one_less[0].server, b);
        assert!(dismiss(standing.clone(), Dismissal::TheWholeMessage).is_empty());
        // A removed row takes its record with it.
        assert_eq!(the_row_was_removed(standing, &b).len(), 1);
    }

    #[test]
    fn the_cursor_is_the_point_since_and_a_new_installation_reads_from_its_own_beginning() {
        let installation = Identity::issue(IdentityKind::Installation).unwrap();
        let cursor = EventCursor {
            taken_under: installation.clone(),
            past_sequence_number: 41,
        };
        assert_eq!(
            where_to_read_from(Some(&cursor), &installation),
            ReadFrom::AfterTheCursor(42)
        );
        let reinstalled = Identity::issue(IdentityKind::Installation).unwrap();
        assert_eq!(
            where_to_read_from(Some(&cursor), &reinstalled),
            ReadFrom::TheBeginningOfItsOwnJournal
        );
        assert_eq!(
            where_to_read_from(None, &installation),
            ReadFrom::TheBeginningOfItsOwnJournal
        );
    }

    /// **N1's frame, read after its event: only a re-run with no frame yet
    /// takes one, and only for its own attempt.** The 38's piece 15.
    #[test]
    fn a_re_runs_frame_read_later_fills_only_the_entry_that_lacks_it() {
        let server = a_server();
        let clay = a_job();
        let other = a_job();
        let mut record = compose_the_account(
            None,
            &server,
            &[
                AnAppliedEvent::InterruptedAndReRunFrom {
                    job: clay.clone(),
                    attempt: 2,
                    frame: None,
                    sequence: 5,
                },
                AnAppliedEvent::InterruptedAndReRunFrom {
                    job: other.clone(),
                    attempt: 2,
                    frame: Some(12),
                    sequence: 6,
                },
            ],
            &tracked(&[&clay, &other]),
            false,
        )
        .unwrap();
        assert_eq!(record.the_re_runs_whose_frame_is_not_known(), vec![(clay.clone(), 2)]);
        assert!(!record.the_re_runs_frame_is_known(&clay, 3, 61), "another attempt's list");
        assert!(record.the_re_runs_frame_is_known(&clay, 2, 61));
        assert!(!record.the_re_runs_frame_is_known(&other, 2, 99), "a frame once said is kept");
        assert!(record.the_re_runs_whose_frame_is_not_known().is_empty());
        assert!(matches!(
            &record.entries[0],
            ANoticeEntry::InterruptedAndReRunFrom { frame: Some(61), .. }
        ));
        assert!(matches!(
            &record.entries[1],
            ANoticeEntry::InterruptedAndReRunFrom { frame: Some(12), .. }
        ));
    }

    #[test]
    fn where_the_machine_is_gone_there_is_no_notice() {
        assert_eq!(where_the_machine_is_gone(), WhereTheMachineIsGone::NoNotice);
    }
}
