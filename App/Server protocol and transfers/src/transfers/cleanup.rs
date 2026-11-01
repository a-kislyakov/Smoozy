//! §7.8 — cleanup, and what the sweep may and may not remove.
//!
//! **S28: abandoned staging is removed by a record-driven sweep in each
//! direction.** A sweep begins from a live or ended transfer record in an
//! explicit removable state and **derives the exact staging path from that
//! record** — its transfer identity, the attempt number the path was
//! synthesised under, and its recorded installation identity.
//!
//! **It removes the object only when the path, synthesised name and stable file
//! identity all match the record.** A path whose record is gone is an **orphan
//! and is not automatically removed**; a path found by enumerating a staging
//! directory is **never ownership evidence**. A path with a different file
//! identity is left alone and reported as an ownership mismatch.
//!
//! **S29: nothing about cleanup reads a file's presence, directory membership
//! or size as ownership or state.** A missing record proves nothing about
//! material at a path.
//!
//! **S30: cleanup is never inferred from a machine being reachable again.**
//! Material a recycle destroyed is gone, and the record still says *interrupted*
//! until C14's branch disqualifies it.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `File Browser` X12.

use super::{Ending, StableFileIdentity, StagingPath, StoppedReason, TransferRecord};

/// **S28's seven removable states**, and *interrupted* is not one of them.
///
/// **S28a: an *interrupted* record is never swept**, because *interrupted*
/// means the work may be carried on and its material is kept for exactly that
/// — correctly, because a sweep that removed it would remove the leftover Row
/// 197 promises the user.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemovableState {
    Stopped,
    /// **Separated out of *interrupted* on 2026-08-31.**
    Failed,
    /// **Separated out of *interrupted* on 2026-08-31.**
    Stalled,
    /// **Was on this list already with no way to derive the superseded path**,
    /// which S4's attempt number now supplies — **and with no identity to match
    /// the derived path against**, which S9's per-attempt identity now
    /// supplies. Without the second the first reached the removing branch and
    /// could not pass it.
    SupersededByAStartOver,
    StagingRemovedByUser,
    AbandonedByItsRequest,
    /// **C14's disqualification.** Staged material at a path this document
    /// remembers is not Smoozy's material, whatever its size.
    InstallationReplaced,
}

/// Whether a record's ending puts it on the sweep's list, and under which
/// state.
///
/// **The consequence of *interrupted* not being here is a requirement on
/// whoever decides not to carry it on** (S28a): the module that abandons an
/// *interrupted* transfer's staging **first writes that transfer record durably
/// to *stopped*, with the reason *abandoned by its request*, and only then
/// arranges the replacement unit**. Without it *"an ordinary **Skip existing**
/// leaves every leftover partial on the rental for the life of the
/// installation"*. **Nothing infers the abandonment from the absence of a
/// take-up.**
pub fn removable_state_of(ending: Option<&Ending>) -> Option<RemovableState> {
    match ending? {
        Ending::Published => None,
        Ending::Interrupted => None,
        Ending::Failed => Some(RemovableState::Failed),
        Ending::Stalled => Some(RemovableState::Stalled),
        Ending::Stopped(StoppedReason::StagingRemovedByUser) => {
            Some(RemovableState::StagingRemovedByUser)
        }
        Ending::Stopped(StoppedReason::AbandonedByItsRequest) => {
            Some(RemovableState::AbandonedByItsRequest)
        }
        Ending::Stopped(StoppedReason::TheSourceChangedUnderIt) => Some(RemovableState::Stopped),
    }
}

/// One thing the sweep intends to remove: a path it derived, and the identity
/// it must match before anything is removed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivedFromTheRecord {
    pub path: StagingPath,
    pub state: RemovableState,
    pub expected_identity: Option<StableFileIdentity>,
}

/// **The sweep's whole derivation.** It begins from records and never from a
/// directory.
///
/// A record's superseded attempt numbers each derive a path too, which is what
/// S4's attempt number was added to make possible — **and each derived path is
/// handed the stable file identity the record kept for that attempt** (S9), so
/// the removing branch below has the value it needs on both sides.
pub fn what_the_sweep_derives(records: &[TransferRecord]) -> Vec<DerivedFromTheRecord> {
    let mut derived = Vec::new();
    for record in records {
        let live_state = removable_state_of(record.ending.as_ref());
        for (path, expected_identity) in
            record.every_staging_path_with_the_identity_recorded_for_it()
        {
            let is_the_live_attempt = path == record.staging_path;
            let state = if is_the_live_attempt {
                match live_state {
                    Some(state) => state,
                    None => continue,
                }
            } else {
                RemovableState::SupersededByAStartOver
            };
            derived.push(DerivedFromTheRecord {
                path,
                state,
                expected_identity,
            });
        }
    }
    derived
}

/// What the sweep does about one derived path once it has read what is there.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheSweepDoes {
    /// The path, the synthesised name and the stable file identity all match.
    Remove,
    /// **A path with a different file identity is left alone and reported as an
    /// ownership mismatch.**
    LeaveItAndReportAnOwnershipMismatch,
    /// **S28's automatic removal requires the recorded identity and does not
    /// run without it** — §12 item 15's stated break: on a machine or platform
    /// that cannot answer, *"the automatic half of cleanup"* is what breaks,
    /// and abandoned staging accumulates until the user removes it by hand
    /// through the visible orphan route. **The ownership guarantee itself —
    /// nothing is removed or published on the strength of a name alone — does
    /// not break.**
    LeaveItBecauseTheIdentityWasNeverRecorded,
    /// Nothing is there. **A path that is not there is not a fault**; the
    /// material was already gone.
    NothingIsThere,
}

pub fn what_the_sweep_does(
    derived: &DerivedFromTheRecord,
    identity_now: Option<&StableFileIdentity>,
) -> WhatTheSweepDoes {
    let Some(now) = identity_now else {
        return WhatTheSweepDoes::NothingIsThere;
    };
    match (&derived.expected_identity, now) {
        (Some(StableFileIdentity::The(recorded)), StableFileIdentity::The(now)) => {
            if recorded == now {
                WhatTheSweepDoes::Remove
            } else {
                WhatTheSweepDoes::LeaveItAndReportAnOwnershipMismatch
            }
        }
        // **An attempt whose supplier could not answer, and one whose staging
        // object was never created**, are the two cases with nothing to match
        // against — for a superseded attempt exactly as for the live one, since
        // S9 keeps the identity beside every attempt number the record retains.
        _ => WhatTheSweepDoes::LeaveItBecauseTheIdentityWasNeverRecorded,
    }
}

/// **S28's orphan rule, stated as a value so nothing forgets it.**
///
/// *"A path whose record is gone is an orphan and is not automatically removed;
/// a path found by enumerating a staging directory is never ownership
/// evidence."* Orphans **remain visible for the user's explicit removal** —
/// through `File Browser` X12 on the machine and through `Download` on the
/// user's own computer — **and the sweep does not claim them.**
pub const A_PATH_FOUND_BY_ENUMERATING_IS_NEVER_OWNERSHIP_EVIDENCE: bool = true;

/// **S27: a stop that could not be carried out is an owed cleanup and not a
/// forgotten one.**
///
/// The pattern is not invented here: it is the recorded debt `Security and
/// licence` §5.1 uses for a secret and `State, identity and recovery` §10.1
/// uses for a machine-side history copy, applied a third time so a builder
/// meets one shape rather than three. **The debt is cleared only when the
/// removal is confirmed.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhetherTheDebtClears {
    /// The removal was confirmed.
    Cleared,
    /// It was not, so the debt stands into the next connection.
    ItStands,
}

pub fn whether_the_debt_clears(the_removal_was_confirmed: bool) -> WhetherTheDebtClears {
    if the_removal_was_confirmed {
        WhetherTheDebtClears::Cleared
    } else {
        WhetherTheDebtClears::ItStands
    }
}

#[cfg(test)]
mod tests {
    use super::super::integrity::Digest;
    use super::super::{Checkpoint, Direction};
    use super::*;
    use smoozy_state::identities::{Identity, IdentityKind};

    fn a_record() -> TransferRecord {
        let issue = |kind| Identity::issue(kind).expect("random bytes");
        TransferRecord::arranged(
            issue(IdentityKind::Transfer),
            Direction::Up,
            issue(IdentityKind::LocalServer),
            Some(issue(IdentityKind::Installation)),
            "/local/scene.blend".to_string(),
            "/jobs/scene.blend".to_string(),
            "/jobs/.smoozy-staging",
            1024,
            issue(IdentityKind::Run),
        )
    }

    #[test]
    fn an_interrupted_record_is_never_swept() {
        // S28a. A sweep that removed it would remove the leftover Row 197
        // promises the user.
        assert_eq!(removable_state_of(Some(&Ending::Interrupted)), None);
        assert_eq!(removable_state_of(Some(&Ending::Published)), None);
        assert_eq!(removable_state_of(None), None);
    }

    #[test]
    fn the_two_endings_separated_out_of_interrupted_are_both_on_the_list() {
        // Made reachable 2026-08-31, with *failed* and *stalled*.
        assert_eq!(
            removable_state_of(Some(&Ending::Failed)),
            Some(RemovableState::Failed)
        );
        assert_eq!(
            removable_state_of(Some(&Ending::Stalled)),
            Some(RemovableState::Stalled)
        );
    }

    #[test]
    fn abandoning_an_interrupted_transfer_puts_it_on_the_list_only_by_writing_stopped() {
        // S28a's requirement on whoever decides not to carry it on. Before the
        // write there is nothing for the sweep; after it there is.
        let mut record = a_record();
        record.ending = Some(Ending::Interrupted);
        assert!(what_the_sweep_derives(std::slice::from_ref(&record)).is_empty());
        record.ending = Some(Ending::Stopped(StoppedReason::AbandonedByItsRequest));
        let derived = what_the_sweep_derives(std::slice::from_ref(&record));
        assert_eq!(derived.len(), 1);
        assert_eq!(derived[0].state, RemovableState::AbandonedByItsRequest);
    }

    #[test]
    fn a_superseded_attempts_path_is_derivable_and_is_swept_while_the_live_one_is_not() {
        // S4's attempt number, which is what made *superseded by a start-over*
        // reachable at all: it was on the list with no way to derive the path.
        let mut record = a_record();
        record.checkpoints.push(Checkpoint {
            offset: 512,
            digest_of_everything_up_to_it: Digest::of(b"half"),
        });
        record.start_over();
        // The live attempt is still running, so it has no ending.
        let derived = what_the_sweep_derives(std::slice::from_ref(&record));
        assert_eq!(derived.len(), 1);
        assert_eq!(derived[0].state, RemovableState::SupersededByAStartOver);
        assert_ne!(derived[0].path, record.staging_path);
    }

    #[test]
    fn a_superseded_path_is_removed_on_the_identity_recorded_for_its_own_attempt() {
        // S9 and S28 together. The identity the superseded attempt was recorded
        // with is what the sweep matches the object at that path against —
        // before it was carried, every superseded path arrived with none and
        // the only removing branch could not run, so the material stayed where
        // it was for good.
        let mut record = a_record();
        record.staging_identity = Some(StableFileIdentity::The("volume-1:file-7".to_string()));
        let superseded_path = record.staging_path.clone();
        record.start_over();

        let derived = what_the_sweep_derives(std::slice::from_ref(&record));
        let for_the_superseded = derived
            .iter()
            .find(|one| one.path == superseded_path)
            .expect("the superseded path is derived");
        assert_eq!(
            what_the_sweep_does(
                for_the_superseded,
                Some(&StableFileIdentity::The("volume-1:file-7".to_string()))
            ),
            WhatTheSweepDoes::Remove
        );
        // And the ownership guarantee is untouched: a different object standing
        // at that path is left alone and reported.
        assert_eq!(
            what_the_sweep_does(
                for_the_superseded,
                Some(&StableFileIdentity::The("volume-1:file-9".to_string()))
            ),
            WhatTheSweepDoes::LeaveItAndReportAnOwnershipMismatch
        );
    }

    #[test]
    fn a_superseded_path_with_no_recorded_identity_is_still_not_removed() {
        // §12 item 15's break, which does not close: where the attempt's
        // supplier could not answer there is nothing to match, and the object
        // stays as a visible orphan rather than being removed on its name.
        let mut record = a_record();
        record.staging_identity = Some(StableFileIdentity::TheSupplierCouldNotAnswer);
        let superseded_path = record.staging_path.clone();
        record.start_over();

        let derived = what_the_sweep_derives(std::slice::from_ref(&record));
        let for_the_superseded = derived
            .iter()
            .find(|one| one.path == superseded_path)
            .expect("the superseded path is derived");
        assert_eq!(
            what_the_sweep_does(
                for_the_superseded,
                Some(&StableFileIdentity::The("volume-1:file-7".to_string()))
            ),
            WhatTheSweepDoes::LeaveItBecauseTheIdentityWasNeverRecorded
        );
    }

    #[test]
    fn nothing_is_removed_on_a_name_alone() {
        // §12 item 15's break, and what does not break with it. Where the
        // identity was never recorded the object stays as a visible orphan.
        let derived = DerivedFromTheRecord {
            path: StagingPath::synthesise(
                "/jobs/.smoozy-staging",
                &Identity::issue(IdentityKind::Transfer).expect("random bytes"),
                1,
            ),
            state: RemovableState::Failed,
            expected_identity: Some(StableFileIdentity::TheSupplierCouldNotAnswer),
        };
        assert_eq!(
            what_the_sweep_does(
                &derived,
                Some(&StableFileIdentity::The("inode-3".to_string()))
            ),
            WhatTheSweepDoes::LeaveItBecauseTheIdentityWasNeverRecorded
        );
    }

    #[test]
    fn a_different_identity_at_the_path_is_left_alone_and_reported() {
        let derived = DerivedFromTheRecord {
            path: StagingPath::synthesise(
                "/jobs/.smoozy-staging",
                &Identity::issue(IdentityKind::Transfer).expect("random bytes"),
                1,
            ),
            state: RemovableState::Failed,
            expected_identity: Some(StableFileIdentity::The("inode-3".to_string())),
        };
        assert_eq!(
            what_the_sweep_does(
                &derived,
                Some(&StableFileIdentity::The("inode-9".to_string()))
            ),
            WhatTheSweepDoes::LeaveItAndReportAnOwnershipMismatch
        );
        assert_eq!(
            what_the_sweep_does(
                &derived,
                Some(&StableFileIdentity::The("inode-3".to_string()))
            ),
            WhatTheSweepDoes::Remove
        );
        assert_eq!(
            what_the_sweep_does(&derived, None),
            WhatTheSweepDoes::NothingIsThere
        );
    }

    #[test]
    fn a_debt_clears_only_on_a_confirmed_removal() {
        // S27. *"The debt is cleared only when the removal is confirmed."*
        assert_eq!(whether_the_debt_clears(true), WhetherTheDebtClears::Cleared);
        assert_eq!(
            whether_the_debt_clears(false),
            WhetherTheDebtClears::ItStands
        );
    }
}
