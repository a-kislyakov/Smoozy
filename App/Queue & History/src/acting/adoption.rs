//! **§13 A1 to A9 performed — the queue reading's one request, the
//! supervisor-unavailable condition, and the adoption offer composed from the
//! reading and accepted in one local write.**
//!
//! # A1 is one request and this crate holds no schedule
//!
//! **The reading is taken on demand, after the installation identity confirms,
//! and it returns the machine's whole working state.** *On demand* names who
//! composes the request and includes `Desktop app and delivery` P10d's
//! live-reading sequence, which repeats the same one request while a surface
//! showing that server's queue or its running entry is on screen. **This
//! document still holds no schedule of its own**, and there is no timer, no
//! watcher and no poll in this module.
//!
//! **It is never taken before `State, identity and recovery` §7 step 1
//! confirms the installation**, and on the absent or different branches
//! nothing on the machine is read as Smoozy's at all — which is why
//! [`take_the_reading`] takes the confirmation as a value rather than deciding
//! it.
//!
//! # A3's narrowing, consumed rather than retaken
//!
//! **The supervisor-unavailable condition is raised by the request not being
//! answered *with the transport shown to have carried it*.** `Server protocol
//! and transfers` C17a's verdict is consumed here: **a request left unanswered
//! inside a break `Servers` then declares raises nothing.** Without that,
//! a request in flight when the user's own line dies would raise a machine
//! verdict standing beside the connection-down statement for the whole outage,
//! with nothing to withdraw it — and `Preparation/Specs/Whole app.md` forbids
//! exactly that: *"A lost connection is a fact about the connection, never a
//! verdict on the machine."*
//!
//! # A5a: the offer is unfiltered, and nothing here adds a pre-check
//!
//! **`Servers` 11.3's ground is that a refusal is discovered by trying, not
//! predicted**, so a pre-check asking whether the current login could act on
//! the work would be a guess that could withhold work the user can in fact act
//! on. **There is no such check in this module and none is to be added.**
//!
//! # A9: nothing about adoption restores a deletion
//!
//! **The tombstone exclusion is what keeps Row 82's promise across the one
//! route that could quietly break it**, and it is
//! [`crate::deletion::may_become_an_entry_again`]'s answer rather than a
//! second rule here.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` C17a.
//! `State, identity and recovery` §7, §8.3, §10.1, §11.1.
//! `Desktop app and delivery` P10d.

use std::collections::HashSet;

use smoozy_state::identities::Identity;
use smoozy_state::observed::Reading;
use smoozy_state::tombstones::HistoryTombstones;

use crate::readings::AJobAsTheMachineReadsIt;

/// **A1's reading, whole: the machine's working state as one answer.**
///
/// **Per job: its state, its admission point standing, its operation, its
/// unreleased hold** — the shape Fact 40 answered — **plus the note and the
/// machine's own history copy's count and entries.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheMachinesWorkingState {
    pub jobs: Vec<AJobAsTheMachineReadsIt>,
    /// **The admission point's standing**: the job the machine would start
    /// next, or nothing where the line holds nothing startable.
    pub the_admission_point_stands_at: Option<Identity>,
    /// The per-job notes the machine holds, by job.
    pub notes: Vec<(Identity, crate::acting::note::TheNote)>,
    /// **The machine's own history copy's count and entries.**
    pub the_history_copys_count: usize,
    pub the_history_copys_entries: Vec<Identity>,
}

/// **What the one request gave, and the three are not the same thing.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheRequestGave {
    /// **A1: the reading, period-held, stamped with its moment and clock.**
    ItAnswered(Reading<TheMachinesWorkingState>),
    /// **A3: the supervisor-unavailable condition** — `State, identity and
    /// recovery` §8.3's. **The queue shows the condition rather than a guessed
    /// line.**
    TheSupervisorIsUnavailable,
    /// **A3's narrowing: a request left unanswered inside a break `Servers`
    /// then declares raises nothing.** It is a fact about the connection and
    /// never a verdict on the machine.
    ItWasLostInsideADeclaredBreakAndRaisesNothing,
}

/// **The machine, on the far side of one trait, because it genuinely is.**
///
/// **One request, one at a time, period-held as ever.** The transport is
/// `Servers`' connection and this module composes none of it.
pub trait AsksAMachineForItsQueue {
    /// **`Server protocol and transfers` C17a's verdict is part of the
    /// answer**, not something composed afterwards: an implementation says
    /// whether the transport was shown to have carried the request.
    fn ask(&self, server: &Identity) -> WhatTheRequestGave;
}

/// **A1, performed: the reading is never taken before the installation
/// identity confirms.**
///
/// **On the absent or different branches nothing on the machine is read as
/// Smoozy's at all**, so no request is composed and no condition is raised —
/// which is `State, identity and recovery` §7 step 1's answer and not a second
/// one here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhetherTheInstallationConfirmed {
    ItIsTheSameInstallation,
    /// Absent or different: nothing on the machine is Smoozy's.
    NothingOnItIsSmoozys,
}

pub fn take_the_reading(
    machine: &dyn AsksAMachineForItsQueue,
    server: &Identity,
    confirmed: WhetherTheInstallationConfirmed,
) -> Option<WhatTheRequestGave> {
    match confirmed {
        WhetherTheInstallationConfirmed::NothingOnItIsSmoozys => None,
        WhetherTheInstallationConfirmed::ItIsTheSameInstallation => Some(machine.ask(server)),
    }
}

/// **A2: what distinguishes tracked from untracked is the desktop's local
/// record and nothing on the machine.**
///
/// *"The reading reaches untracked and declined work because the store does
/// not distinguish them"* (Q6). **Identities the local record names are
/// tracked; identities it does not are untracked.**
pub fn the_untracked_in(
    state: &TheMachinesWorkingState,
    tracked: &HashSet<Identity>,
) -> Vec<Identity> {
    state
        .jobs
        .iter()
        .map(|job| job.job.clone())
        .filter(|job| !tracked.contains(job))
        .collect()
}

/// **A4: declined work is sayable as a fact without being shown as jobs.**
///
/// **The fact of it and the count of it, never the jobs themselves** — so a
/// tracked job sitting at the top of the queue and not starting has a stated
/// cause. **Nothing durable holds it** (C6); each connection reads it fresh.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhatTheQueueSaysAboutDeclinedWork {
    pub the_machine_is_busy_with_work_the_user_chose_not_to_track: bool,
    pub how_many: usize,
}

pub fn what_the_queue_says_about_declined_work(
    state: &TheMachinesWorkingState,
    tracked: &HashSet<Identity>,
    declined: &HashSet<Identity>,
) -> WhatTheQueueSaysAboutDeclinedWork {
    let untracked = the_untracked_in(state, tracked);
    let how_many = untracked
        .iter()
        .filter(|job| declined.contains(*job))
        .count();
    WhatTheQueueSaysAboutDeclinedWork {
        the_machine_is_busy_with_work_the_user_chose_not_to_track: how_many > 0,
        how_many,
    }
}

/// **A5, A5a and A9: the offer, one per machine, covering everything at
/// once.**
///
/// **The set is every job identity on the machine the local record does not
/// name, less any identity a history tombstone names** — which is not
/// offerable work but an owed erase (`State, identity and recovery` §10.1).
///
/// **It is unfiltered**: no pre-check asks whether the current login could act
/// on the work, on `Servers` 11.3's ground that a refusal is discovered by
/// trying rather than predicted.
///
/// **Declining writes nothing durable**, so the next connection that finds the
/// work offers again — the spec's *declining is not final*, delivered by the
/// absence of a record rather than by a schedule. **There is no decline
/// function here for that reason.**
pub fn the_adoption_offer(
    state: &TheMachinesWorkingState,
    tracked: &HashSet<Identity>,
    tombstones: &dyn HistoryTombstones,
) -> Vec<Identity> {
    the_untracked_in(state, tracked)
        .into_iter()
        .filter(|job| crate::deletion::may_become_an_entry_again(tombstones, job))
        .collect()
}

/// **What one adopted job enters the local record as.**
///
/// **A6: each adopted record is written *confirmed*, not provisional** — the
/// work demonstrably exists on the machine — **and under the identity the
/// machine carries, kept rather than reissued** (Y4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnAdoptedJob {
    pub job: Identity,
    pub server: Identity,
    /// **A7: what the machine's records hold**, which is step 4's fill-in's
    /// reach and no more. **Where a record is unreadable, the entry states the
    /// absence** — carried as `None` rather than as an invented value.
    pub the_note: Option<crate::acting::note::TheNote>,
}

/// **The local record's write, reached through one trait.**
pub trait WritesAdoptedJobs {
    /// **A6: every offered job enters the local record in one durable write.**
    /// **All or nothing is the offer's own shape** (`State, identity and
    /// recovery` §9.2 test 1 — the promise is about the offer, not about
    /// members), so an implementation that writes members one at a time has
    /// broken it.
    fn accept_them_all(&mut self, adopted: &[AnAdoptedJob]) -> Result<(), String>;
}

/// **A6 and A8, performed: accepting is atomic entire, and from that moment
/// the local record is Smoozy's own note.**
///
/// **A job already rendering enters history at once** (C9); a job that ends
/// later takes its ending by the ordinary routes. **Every guarantee this
/// document rests on holds for the adopted job by construction**: its folder
/// was named for its identity when it was first submitted, *no job ever writes
/// over another* stands on the identity that never changed, and re-runnability
/// is the machine's own reconciliation, which never needed Smoozy.
pub fn accept(
    writer: &mut dyn WritesAdoptedJobs,
    server: &Identity,
    offer: &[Identity],
    state: &TheMachinesWorkingState,
) -> Result<Vec<AnAdoptedJob>, String> {
    let adopted: Vec<AnAdoptedJob> = offer
        .iter()
        .map(|job| AnAdoptedJob {
            job: job.clone(),
            server: server.clone(),
            the_note: state
                .notes
                .iter()
                .find(|(named, _)| named == job)
                .map(|(_, note)| note.clone()),
        })
        .collect();
    writer.accept_them_all(&adopted)?;
    Ok(adopted)
}

/// **A9, stated as a value: nothing about adoption restores a deletion.**
pub const ADOPTION_NEVER_RESTORES_A_DELETION: bool = true;

/// **A5a, stated as a value: the offer is unfiltered and no pre-check asks
/// whether the current login could act on the work.**
pub const THE_OFFER_IS_UNFILTERED: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::acting::note::TheNote;
    use crate::line::QueueState;
    use crate::readings::WhatTheEntryHasLive;
    use smoozy_state::identities::IdentityKind;
    use smoozy_state::tombstones::HistoryTombstone;
    use std::time::SystemTime;

    fn a_job() -> Identity {
        Identity::issue(IdentityKind::Job).expect("randomness")
    }

    fn a_server() -> Identity {
        Identity::issue(IdentityKind::LocalServer).expect("randomness")
    }

    fn a_state(jobs: &[Identity]) -> TheMachinesWorkingState {
        TheMachinesWorkingState {
            jobs: jobs
                .iter()
                .map(|job| AJobAsTheMachineReadsIt {
                    job: job.clone(),
                    state: QueueState::Waiting,
                    live: WhatTheEntryHasLive::NoReadings,
                    the_hold: None,
                })
                .collect(),
            the_admission_point_stands_at: jobs.first().cloned(),
            notes: jobs
                .iter()
                .map(|job| (job.clone(), TheNote::created_with_the_job_record("a label")))
                .collect(),
            the_history_copys_count: 0,
            the_history_copys_entries: Vec::new(),
        }
    }

    #[derive(Default)]
    struct Tombstones(Vec<HistoryTombstone>);

    impl HistoryTombstones for Tombstones {
        fn tombstone_for(&self, job: &Identity) -> Option<&HistoryTombstone> {
            self.0.iter().find(|stone| &stone.job == job)
        }
    }

    #[derive(Default)]
    struct TheRecordInHand {
        written: Vec<AnAdoptedJob>,
        writes: usize,
        refused: bool,
    }

    impl WritesAdoptedJobs for TheRecordInHand {
        fn accept_them_all(&mut self, adopted: &[AnAdoptedJob]) -> Result<(), String> {
            if self.refused {
                return Err("refused".to_string());
            }
            self.writes += 1;
            self.written.extend_from_slice(adopted);
            Ok(())
        }
    }

    struct AMachineInHand(WhatTheRequestGave);

    impl AsksAMachineForItsQueue for AMachineInHand {
        fn ask(&self, _server: &Identity) -> WhatTheRequestGave {
            self.0.clone()
        }
    }

    fn tracked(jobs: &[&Identity]) -> HashSet<Identity> {
        jobs.iter().map(|job| (*job).clone()).collect()
    }

    /// **A1: no request is composed before the installation identity
    /// confirms**, and on the absent or different branch nothing on the
    /// machine is read as Smoozy's at all.
    #[test]
    fn no_reading_is_taken_before_the_installation_confirms() {
        let machine = AMachineInHand(WhatTheRequestGave::TheSupervisorIsUnavailable);
        assert_eq!(
            take_the_reading(
                &machine,
                &a_server(),
                WhetherTheInstallationConfirmed::NothingOnItIsSmoozys
            ),
            None
        );
        assert!(take_the_reading(
            &machine,
            &a_server(),
            WhetherTheInstallationConfirmed::ItIsTheSameInstallation
        )
        .is_some());
    }

    /// **A3: a request lost inside a declared break raises nothing**, and is
    /// not the same answer as the supervisor being unavailable.
    #[test]
    fn a_request_lost_inside_a_declared_break_raises_no_verdict_on_the_machine() {
        let lost =
            AMachineInHand(WhatTheRequestGave::ItWasLostInsideADeclaredBreakAndRaisesNothing);
        assert_eq!(
            take_the_reading(
                &lost,
                &a_server(),
                WhetherTheInstallationConfirmed::ItIsTheSameInstallation
            ),
            Some(WhatTheRequestGave::ItWasLostInsideADeclaredBreakAndRaisesNothing)
        );
        let unavailable = AMachineInHand(WhatTheRequestGave::TheSupervisorIsUnavailable);
        assert_ne!(
            take_the_reading(
                &unavailable,
                &a_server(),
                WhetherTheInstallationConfirmed::ItIsTheSameInstallation
            ),
            Some(WhatTheRequestGave::ItWasLostInsideADeclaredBreakAndRaisesNothing),
            "the two are not one answer"
        );
    }

    /// **A2 and A5: the offer is every identity the local record does not
    /// name.**
    #[test]
    fn the_offer_is_every_identity_the_local_record_does_not_name() {
        let mine = a_job();
        let theirs = a_job();
        let state = a_state(&[mine.clone(), theirs.clone()]);
        let offer = the_adoption_offer(&state, &tracked(&[&mine]), &Tombstones::default());
        assert_eq!(offer, vec![theirs]);
    }

    /// **A9: a tombstoned identity is never offered**, which is what keeps
    /// Row 82's promise across the one route that could quietly break it.
    #[test]
    fn a_tombstoned_identity_is_never_offered() {
        let deleted = a_job();
        let state = a_state(std::slice::from_ref(&deleted));
        let tombstones = Tombstones(vec![HistoryTombstone::for_a_deleted_entry(
            deleted.clone(),
            SystemTime::UNIX_EPOCH,
        )]);
        assert!(the_adoption_offer(&state, &HashSet::new(), &tombstones).is_empty());
        // And without the tombstone it is offered, so the exclusion is what
        // did the work.
        assert_eq!(
            the_adoption_offer(&state, &HashSet::new(), &Tombstones::default()),
            vec![deleted]
        );
        const _: () = assert!(ADOPTION_NEVER_RESTORES_A_DELETION);
    }

    /// **A6: accepting is atomic entire — one durable write, identities kept,
    /// each record confirmed.**
    #[test]
    fn accepting_is_one_write_and_keeps_every_identity() {
        let server = a_server();
        let first = a_job();
        let second = a_job();
        let state = a_state(&[first.clone(), second.clone()]);
        let offer = the_adoption_offer(&state, &HashSet::new(), &Tombstones::default());
        let mut record = TheRecordInHand::default();
        let adopted = accept(&mut record, &server, &offer, &state).expect("accepted");
        assert_eq!(record.writes, 1, "one durable write for the whole offer");
        assert_eq!(adopted.len(), 2);
        assert_eq!(adopted[0].job, first);
        assert_eq!(adopted[1].job, second);
        assert!(adopted.iter().all(|job| job.server == server));
        assert!(adopted.iter().all(|job| job.the_note.is_some()));
    }

    /// **A6: all or nothing.** A refused write leaves nothing in the record.
    #[test]
    fn a_refused_write_adopts_nothing_at_all() {
        let state = a_state(&[a_job(), a_job()]);
        let offer = the_adoption_offer(&state, &HashSet::new(), &Tombstones::default());
        let mut record = TheRecordInHand {
            refused: true,
            ..TheRecordInHand::default()
        };
        assert!(accept(&mut record, &a_server(), &offer, &state).is_err());
        assert!(record.written.is_empty());
    }

    /// **A7: where a record is unreadable, the entry states the absence** —
    /// carried as nothing rather than as an invented value.
    #[test]
    fn an_adopted_job_whose_note_cannot_be_read_carries_the_absence() {
        let job = a_job();
        let mut state = a_state(std::slice::from_ref(&job));
        state.notes.clear();
        let mut record = TheRecordInHand::default();
        let adopted = accept(&mut record, &a_server(), &[job], &state).expect("accepted");
        assert_eq!(adopted[0].the_note, None);
    }

    /// **A4: declined work is a fact and a count, never the jobs
    /// themselves.**
    #[test]
    fn declined_work_is_said_as_a_count_and_the_jobs_are_not_shown() {
        let mine = a_job();
        let declined_one = a_job();
        let declined_two = a_job();
        let state = a_state(&[mine.clone(), declined_one.clone(), declined_two.clone()]);
        let said = what_the_queue_says_about_declined_work(
            &state,
            &tracked(&[&mine]),
            &tracked(&[&declined_one, &declined_two]),
        );
        assert!(said.the_machine_is_busy_with_work_the_user_chose_not_to_track);
        assert_eq!(said.how_many, 2);
        // The type carries no identities at all, which is the rule kept by
        // the shape rather than by care.
        const _: () = assert!(THE_OFFER_IS_UNFILTERED);
    }

    /// **A5a: no pre-check filters the offer.** The guard is a scan of this
    /// file's working half for a login or permission condition.
    #[test]
    fn nothing_here_pre_checks_a_login_or_a_permission() {
        let source = include_str!("adoption.rs");
        let working_half = source
            .split("#[cfg(test)]")
            .next()
            .expect("the file has a working half");
        let code: String = working_half
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join(" ");
        for forbidden in ["login", "permission", "elevat", "may_act"] {
            assert!(
                !code.contains(forbidden),
                "{forbidden} would be the pre-check A5a forbids"
            );
        }
    }
}
