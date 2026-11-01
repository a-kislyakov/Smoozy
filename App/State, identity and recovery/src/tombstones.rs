//! Deletion tombstones — `State, identity and recovery` §10.
//!
//! H1: **a tombstone exists where a later reading would otherwise undo a
//! deletion. There are two kinds and the list is closed.** The second kind is
//! the secret-erase mark, which is `Security and licence` §5.1's and lives in
//! that module ([`smoozy_security_and_licence::erasure`]); it is a tombstone in
//! everything but name and is cited here so both members of the class are
//! visible in one place.

use crate::identities::Identity;
use crate::observed::{Clock, Reading};

/// The history tombstone — §10.1.
///
/// H2: it holds **the job identity, the moment of the deletion, and whether
/// the machine-side copy has been removed yet**, and nothing else.
///
/// H3: **it is keyed by the job identity and by nothing else, deliberately.** A
/// tombstone keyed to a server row would be lost if the user removed that
/// server and added the same machine again, and the promise is about that
/// machine rather than about a row in a list. Job identities are never reused,
/// so the key is sufficient on its own and the tombstone is one identity in
/// size.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryTombstone {
    pub job: Identity,
    pub deleted_at: Reading<()>,
    pub machine_side_copy: MachineSideCopy,
}

/// H5: the tombstone **is also the owed erase for the machine's copy**. While
/// the copy has not been removed, the next connection to a machine holding
/// that job identity removes it and marks the tombstone. That is the same
/// recorded-debt pattern `Security and licence` §5.1 already uses for a
/// secret, applied a second time — one shape rather than two.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineSideCopy {
    /// The machine was not connected at the deletion, or the removal has not
    /// completed. The debt stands.
    StillOwed,
    /// The machine reported the copy gone.
    Removed,
}

impl HistoryTombstone {
    /// The deletion is recorded on the user's own computer's clock, which is
    /// D5's presentation use and nothing a promise rests on: nothing subtracts
    /// this moment from anything.
    pub fn for_a_deleted_entry(job: Identity, deleted_at: std::time::SystemTime) -> Self {
        HistoryTombstone {
            job,
            deleted_at: Reading::taken((), deleted_at, Clock::ThisComputer),
            machine_side_copy: MachineSideCopy::StillOwed,
        }
    }

    /// H4: **it refuses step 4 of desktop reconciliation unconditionally**,
    /// whatever the machine still holds, and whether or not the machine-side
    /// copy was ever removed.
    pub fn refuses_filling_in(&self) -> bool {
        true
    }
}

/// The tombstones the local record holds, and the one question anything asks
/// of them.
///
/// H6: **a tombstone is kept for the life of the local record** and is cleared
/// by nothing. R1 states the consequence that belongs to the lifetimes
/// section: no deletion of a history record and no wipe of a machine's history
/// removes a tombstone, so a tombstone outlives every entry it names.
pub trait HistoryTombstones {
    fn tombstone_for(&self, job: &Identity) -> Option<&HistoryTombstone>;

    /// Desktop reconciliation step 4's whole test. A job the record does not
    /// know is filled in from the machine — **unless a tombstone names it**,
    /// and then never.
    fn may_be_filled_in(&self, job: &Identity) -> bool {
        match self.tombstone_for(job) {
            Some(tombstone) => !tombstone.refuses_filling_in(),
            None => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identities::IdentityKind;
    use std::time::SystemTime;

    struct Record(Vec<HistoryTombstone>);

    impl HistoryTombstones for Record {
        fn tombstone_for(&self, job: &Identity) -> Option<&HistoryTombstone> {
            self.0.iter().find(|tombstone| &tombstone.job == job)
        }
    }

    fn a_job() -> Identity {
        Identity::issue(IdentityKind::Job).unwrap()
    }

    #[test]
    fn a_deleted_entry_is_never_filled_back_in() {
        let job = a_job();
        let record = Record(vec![HistoryTombstone::for_a_deleted_entry(
            job.clone(),
            SystemTime::now(),
        )]);
        assert!(!record.may_be_filled_in(&job));
    }

    #[test]
    fn the_machine_copy_having_gone_does_not_lift_the_refusal() {
        let job = a_job();
        let mut tombstone = HistoryTombstone::for_a_deleted_entry(job.clone(), SystemTime::now());
        tombstone.machine_side_copy = MachineSideCopy::Removed;
        let record = Record(vec![tombstone]);
        assert!(!record.may_be_filled_in(&job));
    }

    #[test]
    fn a_job_with_no_tombstone_is_filled_in_as_usual() {
        let record = Record(Vec::new());
        assert!(record.may_be_filled_in(&a_job()));
    }

    #[test]
    fn a_fresh_tombstone_still_owes_the_machine_side_erase() {
        let tombstone = HistoryTombstone::for_a_deleted_entry(a_job(), SystemTime::now());
        assert_eq!(tombstone.machine_side_copy, MachineSideCopy::StillOwed);
    }
}
