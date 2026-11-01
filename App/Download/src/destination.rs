//! Destination trouble — `Download` §9, D1 to D9, Rows 218 and 219.
//!
//! **Constraint:** Row 218 — *a cause Smoozy genuinely knows*, said once — and
//! Row 219 — a cadence that never gives up.
//!
//! **The outcome classes are the adapter's and are consumed, never
//! re-derived.** `Desktop app and delivery` F1 to `Desktop app and delivery`
//! F9 own them, and that
//! document's crate cannot be reached from a feature module — it depends on
//! the feature modules and supplies their adapters. **So the shape is written
//! out once, here, as a value the adapter fills** ([`WhatTheAdapterSaid`]),
//! and **nothing in this file classifies an error number, a symbol or a
//! system's own words into it.** That is the same wall and the same answer
//! [`crate::fetching`] records for the name check.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Desktop app and delivery` F1, F2, F3, F4, F4a,
//! F5, F6, F7, F14, Q5, Q6, Q7, Q8, Q12, R17. `Server protocol and transfers`
//! S1, S28, T10, C16, C17. `State, identity and recovery` §1.3.
//!
//! **A marker this plan also defines is deliberately not listed above,
//! whichever document is meant, and is written out with its document at every
//! site instead** — the header list cannot tell the two apart, and a bare one
//! would resolve to a real and wrong clause. `Desktop app and delivery` Q1 and
//! Q2, `Servers` N3 and `Servers` N6 are the ones that arise here, and each is
//! written out wherever it stands. **So is the one marker two other documents
//! both define** — the withdrawal clause cited at [`the_condition_is_withdrawn`]
//! — because a header list filing it under one of them would forgive a bare
//! use meaning the other.

use std::time::Duration;

/// **The outcome classes `Desktop app and delivery` F1 to
/// `Desktop app and delivery` F9 declare, closed set, as the adapter hands
/// them on.**
///
/// **Nothing in this crate produces one from an error number or a symbol.**
/// The adapter maps by symbol per system and never by number, which is its
/// rule and not this file's.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheAdapterSaid {
    /// **The volume is full.** **No amount is consulted to reach it.**
    TheVolumeIsFull,
    /// **Writing is refused.**
    WritingIsRefused,
    /// **The destination is no longer there, and the adapter names neither
    /// cause behind it** — a removed directory and a detached volume are one
    /// value where the mount point went with the volume, and the separating
    /// test the findings once proposed is struck.
    TheDestinationIsNoLongerThere,
    /// **The write failed at the device, and the adapter names no cause behind
    /// it either** (`Desktop app and delivery` F4a, supplied 2026-08-23).
    ///
    /// **A class that is macOS-observed and is claimed for neither Windows nor
    /// Linux**, whose detached-volume values are unestablished — §15 items 1
    /// and 2, **both permanently unresolved rather than pending**.
    TheWriteFailedAtTheDevice,
    /// **A refusal the system reported in terms the adapter does not map.**
    /// It is never folded into one of the four above.
    Unclassified,
}

/// **D2: what Smoozy says for each class, and the narrowing is stated rather
/// than smoothed.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatIsSaid {
    /// **The promise holds whole**: the cause is named and it carries its
    /// remedy.
    TheCauseIsNamedWithItsRemedy,
    /// **For *the destination is gone* the genuinely known cause is that the
    /// destination is gone**, and Smoozy says that and says what covers both
    /// readings — make the destination exist again — **without claiming which
    /// of the two it was**, because a message naming the wrong cause is worse
    /// than one naming neither.
    TheDestinationIsGoneWithTheRemedyThatCoversBothReadings,
    /// **For the device failure the genuinely known cause is none**: Smoozy
    /// says the write failed at the device and names nothing behind it, **and
    /// no remedy is named there, because no cause is.**
    TheWriteFailedAtTheDeviceAndNothingBehindIt,
    /// **Nothing false is ever said.** An unmapped refusal is reported as what
    /// the system returned, with no class claimed for it.
    WhatTheSystemReturnedAndNoClassClaimedForIt,
}

/// **D2, as one total function over D1's classes.**
pub fn what_is_said(said: WhatTheAdapterSaid) -> WhatIsSaid {
    match said {
        WhatTheAdapterSaid::TheVolumeIsFull | WhatTheAdapterSaid::WritingIsRefused => {
            WhatIsSaid::TheCauseIsNamedWithItsRemedy
        }
        WhatTheAdapterSaid::TheDestinationIsNoLongerThere => {
            WhatIsSaid::TheDestinationIsGoneWithTheRemedyThatCoversBothReadings
        }
        WhatTheAdapterSaid::TheWriteFailedAtTheDevice => {
            WhatIsSaid::TheWriteFailedAtTheDeviceAndNothingBehindIt
        }
        WhatTheAdapterSaid::Unclassified => WhatIsSaid::WhatTheSystemReturnedAndNoClassClaimedForIt,
    }
}

/// **D1's two rules about the write sequence, consumed as the adapter states
/// them and restated nowhere.**
///
/// **The write sequence's outcome is the first error any call returned**
/// (`Desktop app and delivery` F2), so the failure is reported and never read
/// as a clean finish; **and the destination is re-checked by path after
/// closing, because a deleted directory reports success at every call**
/// (`Desktop app and delivery` F1).
pub const THE_FIRST_ERROR_IS_THE_OUTCOME_AND_THE_PATH_IS_RE_CHECKED_AFTER_CLOSING: bool = true;

/// **D2's one undetected case, carried rather than claimed away.**
///
/// **A detached volume whose mount point survived** — where the frame lands on
/// the underlying volume with **no error anywhere**, and which is therefore no
/// class of D1's — **is not detected, not defended and not claimed.** It is
/// §15 item 3's, carried with `Desktop app and delivery` F14.
///
/// **What breaks is more than one write**: the write succeeds with no error
/// anywhere, so the account records *fetched*, and the never-fetched-again
/// rule makes that permanent. The file is on the underlying volume, invisible
/// under the mount point the moment the drive is reattached. **What holds is
/// the file itself, whole, on the wrong disk — and nothing else.**
pub const THE_MOUNT_POINT_SURVIVED_DETACH_IS_NOT_DETECTED_AND_IS_NOT_CLAIMED: bool = true;

/// **D3: no amount decides anything.**
///
/// A full volume never reads zero — 1,068 KB on the genuinely full one — **so
/// no threshold exists here** (`Desktop app and delivery` F5's rule taken
/// whole), and **only a write attempt establishes that a refusal is over**
/// (`Desktop app and delivery` F6).
pub const NO_AMOUNT_DECIDES_ANYTHING_AND_ONLY_A_WRITE_ESTABLISHES_RECOVERY: bool = true;

/// **D4: one condition, keyed on the refusal's kind and the destination
/// folder's own path in one normalised form.**
///
/// So a pool of transfers failing at the same moment for the same reason
/// resolves to **one** condition identity, and `Desktop app and delivery` Q5
/// makes *said once* fall out of the key rather than being enforced.
/// **Unit outcomes are never suppressed** (`Desktop app and delivery` Q6): the
/// files that could not land are counted and named; **what is said once is the
/// reason.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheConditionKey {
    pub refusal: WhatTheAdapterSaid,
    /// **The destination folder's own path in one normalised form.** **What
    /// normalises a desktop path on each of the three systems is the platform
    /// adapter's and is taken from `Desktop app and delivery` as supplied, not
    /// decided here** — this field holds what that adapter returned.
    pub the_destination_folder_normalised: Vec<u8>,
}

/// **D4: the key is composed from the folder and not its volume, because only
/// one of the classes is a property of a volume.**
///
/// *The volume is full* is; *writing is refused* is a folder's permissions,
/// *the destination is gone* is a folder deleted, and the device class names
/// no cause at all — **so a volume-keyed condition would be wider than the
/// fact for three of D1's four classes.** Two arrangements into two folders on
/// one disk would then share one identity: the hold would reach an arrangement
/// whose own folder is perfectly writable, and the withdrawal would fire on a
/// successful write into a healthy folder while the sick one never recovered,
/// **raising a fresh identity every minute for a trouble that never
/// resolved.**
///
/// `Desktop app and delivery` Q7's guarantee that *"a different subject or a
/// different kind is a different condition identity by construction and never
/// resolves to the same one"* **is what the folder key delivers and the volume
/// key breaks**, and `Desktop app and delivery` Q2 warns in its own words that
/// a suppression drawn too wide silences a second, unrelated failure.
///
/// **A folder that cannot be resolved keys on the path as recorded in the
/// arrangement**, which is always available and never absent. **That is the
/// destination-gone case**: a folder that is gone has no volume to read, and
/// where the mount point went with the volume there is no mount point either
/// — so a key that needed either component would leave a builder unable to
/// raise the condition at all, and the user would meet the failure per file,
/// which Row 218 forbids.
pub fn the_condition_key(
    refusal: WhatTheAdapterSaid,
    the_destination_folder_normalised: Vec<u8>,
) -> TheConditionKey {
    TheConditionKey {
        refusal,
        the_destination_folder_normalised,
    }
}

/// **D5: withdrawal is the next real write to that place succeeding, and
/// nothing else** — not a recovered free-space reading.
///
/// **That is one signal where the transfer contract's own withdrawal takes
/// two — `Server protocol and transfers` F11 — and the divergence is declared
/// with its reason rather than left quiet.**
/// That document's withdrawal takes both a non-zero free-space reading **and**
/// a real write to that place succeeding. **On this side the free-space half
/// is forbidden** — `Desktop app and delivery` F5's rule, and the 1,068-KB
/// reading on a genuinely full volume, mean a free-space figure never reaches
/// zero and so establishes nothing — **so the withdrawal takes the write
/// alone.**
///
/// **A withdrawn record is spent; a destination that works and later fails
/// again raises a fresh identity against the same key and is said once again**
/// (`Desktop app and delivery` Q12), which is exactly the spec's *fresh piece
/// of trouble*. **The destination's state across attempts is the condition
/// record and its withdrawal, and nothing else is tracked.**
pub fn the_condition_is_withdrawn(a_real_write_to_that_place_succeeded: bool) -> bool {
    a_real_write_to_that_place_succeeded
}

/// **D5's other half, held as a value: a free-space reading never withdraws a
/// condition here.**
pub const A_FREE_SPACE_READING_NEVER_WITHDRAWS_A_CONDITION_ON_THIS_SIDE: bool = true;

/// **D6: while the condition stands, the arrangement that raised it arranges
/// no transfers at all — that arrangement, and no other.**
///
/// Staging sits inside the destination folder (`Server protocol and transfers`
/// S1, S28), **so an unwritable destination cannot even stage**, and a fetch
/// arranged against it would fail at the first byte. So the arrangement holds
/// its owed units, **stops offering work to admission** — it occupies no slot
/// at any admission point and cannot hammer the mechanism
/// `Server protocol and transfers` T10 and `Desktop app and delivery` R17
/// braced against — **and keeps only its watch and its probe.**
///
/// **An arrangement whose own destination folder is writable is untouched by
/// another's condition**, which is what the folder key buys and what a volume
/// key would have taken away.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheArrangementDoes {
    /// It offers work to admission as usual.
    ItOffersWorkToAdmission,
    /// **It arranges nothing, occupies no slot, and keeps only its watch and
    /// its probe.**
    ItHoldsItsOwedUnitsAndKeepsOnlyItsWatchAndItsProbe,
}

/// **D6, as one judgement over the standing conditions.**
///
/// The condition is matched by the arrangement's **own** destination folder,
/// so another arrangement's trouble never reaches this one.
pub fn what_the_arrangement_does(
    the_arrangements_destination_normalised: &[u8],
    the_standing_conditions: &[TheConditionKey],
) -> WhatTheArrangementDoes {
    let its_own = the_standing_conditions.iter().any(|key| {
        key.the_destination_folder_normalised == the_arrangements_destination_normalised
    });
    if its_own {
        WhatTheArrangementDoes::ItHoldsItsOwedUnitsAndKeepsOnlyItsWatchAndItsProbe
    } else {
        WhatTheArrangementDoes::ItOffersWorkToAdmission
    }
}

/// **D6's last sentence: a manual request's units that failed on the condition
/// stay failed with the one condition said once; re-asking is the user's
/// act.**
pub const A_MANUAL_REQUESTS_FAILED_UNITS_STAY_FAILED_AND_RE_ASKING_IS_THE_USERS_ACT: bool = true;

/// **D7's pause: 60 seconds, measured from the end of the previous probe, for
/// as long as the condition stands, indefinitely.**
///
/// **60 seconds, from the end**, aligning with the one settled neighbouring
/// figure, `Servers` N6's standing pause — and **measured from the end because
/// a probe against an unresponsive network volume takes time to fail**, which
/// is `Servers` N3's load-bearing shape.
pub const THE_PROBES_PAUSE: Duration = Duration::from_secs(60);

/// **D7: the probe, in the order its two acts run.**
///
/// **The creation first, where the directory is absent**, because nothing may
/// be assumed to have made it. No clause of `Server protocol and transfers`
/// §7 names an actor or a moment for creating a staging directory, and D6
/// forbids the only act this document names as making one. **Two permitted
/// starting states reach the same dead end without this**: a user switches
/// auto-download on against a destination that is already full or unwritable,
/// so no transfer was ever arranged and no directory was ever made; or the
/// destination folder is deleted or its drive detached, taking the staging
/// directory inside it with it. **In either state a probe that only wrote into
/// a directory that was not there would fail for a reason unrelated to the
/// condition**, the withdrawal would never be met, and the arrangement would
/// probe at one byte a minute for as long as the app runs, with no ceiling, no
/// growing pause, no terminal state and no recovery.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheProbesStep {
    /// **The arrangement's own staging directory created inside the
    /// destination folder where it is absent.**
    CreateTheStagingDirectoryWhereItIsAbsent,
    /// **One real write of at least one byte into it**, because on a full
    /// filesystem the writability test returns writable and creating an empty
    /// file succeeds — `State, identity and recovery` §1.3 rule 4's rule,
    /// applied on the user's side.
    WriteAtLeastOneByteIntoIt,
    /// **Both removed at once where the probe itself created the directory,
    /// and the byte alone where the directory already stood.**
    ///
    /// **D7 gained that clause on 2026-09-06**, out of the code round on this
    /// section's acting half. The *both* was written for the directory the
    /// probe creates where none stands; a directory that was **already there**
    /// may hold the staged material of a transfer interrupted before the
    /// condition was raised — the material `Server protocol and transfers`
    /// S26's third row keeps with its checkpoints and M10 quotes as the whole
    /// of Row 209's carry-on. A probe running a minute after a 3.9 GB transfer
    /// was interrupted by a full volume would otherwise ask the adapter to
    /// remove the bytes the resume carries on from.
    RemoveWhatTheProbeMade,
}

/// **D7's steps, in order.**
pub const THE_PROBE: [TheProbesStep; 3] = [
    TheProbesStep::CreateTheStagingDirectoryWhereItIsAbsent,
    TheProbesStep::WriteAtLeastOneByteIntoIt,
    TheProbesStep::RemoveWhatTheProbeMade,
];

/// **D7: what the creation call did, which is not the same question as whether
/// the directory is there now.**
///
/// **The removal turns on this and on nothing else.** A call that could only
/// say *the directory is there* leaves the probe unable to tell the directory
/// it made from one that was already standing, and the safe answer then is to
/// remove neither or both — one of which strands the condition and the other
/// of which throws away an interrupted transfer's staged bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheCreationDid {
    /// **This probe made the directory**, so this probe removes it.
    ItMadeTheDirectory,
    /// **The directory was already standing.** It is left standing, and what
    /// is inside it is none of the probe's business.
    ItAlreadyStood,
}

/// **D7: the fewest bytes a real write may be.**
pub const THE_PROBES_WRITE_IS_AT_LEAST_THIS_MANY_BYTES: usize = 1;

/// **D7: the probe exercises the whole path the next transfer needs — the
/// destination folder's existence, its writability, and the volume's room —
/// and cannot pass on a state that cannot stage.**
///
/// **What it makes is the directory the next transfer would have made**, in
/// the place `Server protocol and transfers` S28 puts it, **so the probe adds
/// no artefact class the contract has not already granted and touches nothing
/// of the user's.**
///
/// **The cost is one directory and one byte of local disk per minute and
/// nothing on any wire or any rented machine** — a cadence paid indefinitely,
/// priced so that paying it indefinitely is nothing.
pub const THE_PROBE_CANNOT_PASS_ON_A_STATE_THAT_CANNOT_STAGE: bool = true;

/// **D7: each probe carries a waiter's deadline; expiry changes no durable
/// state and reads as *still unwritable*** (`Server protocol and transfers`
/// C16, C17).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheProbeSaid {
    /// **D8: on a probe that succeeds, the condition is withdrawn**, the
    /// arrangement re-arranges everything written while it could not, **and
    /// the catch-up is the ordinary mechanism** — the reckoning still names
    /// the owed units, the walk kept running, and nothing was lost in the
    /// meantime because the files are on the machine and the machine was never
    /// asked to hold anything for this.
    ItSucceededSoTheConditionIsWithdrawn,
    /// The probe failed, or its deadline expired. **Still unwritable, and no
    /// durable state moved.**
    StillUnwritable,
}

/// **D7 and D8, as one reading over the probe's own outcome.**
pub fn what_the_probe_said(
    the_directory_was_made_or_already_stood: bool,
    the_real_write_succeeded: bool,
) -> WhatTheProbeSaid {
    if the_directory_was_made_or_already_stood && the_real_write_succeeded {
        WhatTheProbeSaid::ItSucceededSoTheConditionIsWithdrawn
    } else {
        WhatTheProbeSaid::StillUnwritable
    }
}

/// **D9: nothing switches the arrangement off.**
///
/// Not the condition, not its repetition, not the suppression —
/// `Desktop app and delivery` Q8's own sentence — **and there is no attempt
/// ceiling, no growing pause and no terminal state.** The only early ends are
/// a removed server, a cancelled job and the user's own switch.
///
/// **Server-side trouble has no cadence here at all**: the arrangement waits
/// on the connection condition and `Servers`' standing repair is the cadence.
pub const NOTHING_SWITCHES_THE_ARRANGEMENT_OFF_AND_THERE_IS_NO_ATTEMPT_CEILING: bool = true;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_two_causes_smoozy_genuinely_knows_carry_a_remedy() {
        assert_eq!(
            what_is_said(WhatTheAdapterSaid::TheVolumeIsFull),
            WhatIsSaid::TheCauseIsNamedWithItsRemedy
        );
        assert_eq!(
            what_is_said(WhatTheAdapterSaid::WritingIsRefused),
            WhatIsSaid::TheCauseIsNamedWithItsRemedy
        );
        assert_eq!(
            what_is_said(WhatTheAdapterSaid::TheDestinationIsNoLongerThere),
            WhatIsSaid::TheDestinationIsGoneWithTheRemedyThatCoversBothReadings
        );
        assert_eq!(
            what_is_said(WhatTheAdapterSaid::TheWriteFailedAtTheDevice),
            WhatIsSaid::TheWriteFailedAtTheDeviceAndNothingBehindIt
        );
        assert_eq!(
            what_is_said(WhatTheAdapterSaid::Unclassified),
            WhatIsSaid::WhatTheSystemReturnedAndNoClassClaimedForIt
        );
    }

    #[test]
    fn a_pool_of_transfers_failing_at_once_for_one_reason_resolves_to_one_identity() {
        let one = the_condition_key(
            WhatTheAdapterSaid::TheVolumeIsFull,
            b"/Users/a/Downloads".to_vec(),
        );
        let other = the_condition_key(
            WhatTheAdapterSaid::TheVolumeIsFull,
            b"/Users/a/Downloads".to_vec(),
        );
        assert_eq!(one, other);
    }

    #[test]
    fn a_different_kind_or_a_different_folder_is_a_different_identity() {
        let full = the_condition_key(
            WhatTheAdapterSaid::TheVolumeIsFull,
            b"/Users/a/Downloads".to_vec(),
        );
        assert_ne!(
            full,
            the_condition_key(
                WhatTheAdapterSaid::WritingIsRefused,
                b"/Users/a/Downloads".to_vec()
            )
        );
        // Two folders on one disk: the whole reason the key is the folder's.
        assert_ne!(
            full,
            the_condition_key(
                WhatTheAdapterSaid::TheVolumeIsFull,
                b"/Users/a/Downloads/night".to_vec()
            )
        );
    }

    #[test]
    fn an_arrangement_whose_own_folder_is_writable_is_untouched_by_anothers_condition() {
        let standing = vec![the_condition_key(
            WhatTheAdapterSaid::TheVolumeIsFull,
            b"/Users/a/Downloads".to_vec(),
        )];
        assert_eq!(
            what_the_arrangement_does(b"/Users/a/Downloads", &standing),
            WhatTheArrangementDoes::ItHoldsItsOwedUnitsAndKeepsOnlyItsWatchAndItsProbe
        );
        assert_eq!(
            what_the_arrangement_does(b"/Volumes/work", &standing),
            WhatTheArrangementDoes::ItOffersWorkToAdmission
        );
    }

    #[test]
    fn the_condition_is_withdrawn_by_a_real_write_and_by_nothing_else() {
        assert!(the_condition_is_withdrawn(true));
        assert!(!the_condition_is_withdrawn(false));
    }

    #[test]
    fn the_probe_makes_the_directory_first_and_cannot_pass_where_it_could_not_be_made() {
        assert_eq!(
            THE_PROBE[0],
            TheProbesStep::CreateTheStagingDirectoryWhereItIsAbsent
        );
        assert_eq!(THE_PROBE[1], TheProbesStep::WriteAtLeastOneByteIntoIt);
        assert_eq!(THE_PROBE[2], TheProbesStep::RemoveWhatTheProbeMade);
        // The dead end the ordering exists to close: the folder is gone, so
        // the directory cannot be made, and a probe that only wrote would fail
        // for a reason unrelated to the condition.
        assert_eq!(
            what_the_probe_said(false, false),
            WhatTheProbeSaid::StillUnwritable
        );
        assert_eq!(
            what_the_probe_said(true, false),
            WhatTheProbeSaid::StillUnwritable
        );
        assert_eq!(
            what_the_probe_said(true, true),
            WhatTheProbeSaid::ItSucceededSoTheConditionIsWithdrawn
        );
    }

    #[test]
    fn the_probes_pause_is_a_minute_and_its_write_is_a_real_one() {
        assert_eq!(THE_PROBES_PAUSE, Duration::from_secs(60));
        assert_eq!(THE_PROBES_WRITE_IS_AT_LEAST_THIS_MANY_BYTES, 1);
    }
}
