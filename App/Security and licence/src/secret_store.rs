//! The contract every system's secret store implements — `Security and
//! licence` §3.0.
//!
//! S1: **five operations, and no others** — put, get, erase, probe, and
//! enumerate the subjects this application has stored. Every item is
//! identified by a kind and a subject. S3's six rules bind all three systems,
//! and the ones a builder can be held to in code are kept here rather than in
//! each adapter, so there is one of each rather than three.
//!
//! The adapters themselves are `Desktop app and delivery` §9's — this document
//! decides the store on each system and does not choose the implementation
//! (§1.4 B30).

use crate::held::{HeldSecret, StoredKind, Subject};
use crate::plaintext::Plaintext;
use std::time::Duration;

/// Why a get did not hand back a value — `Security and licence` §3.0 rule 4.
///
/// **A get that fails is never treated as *the secret is wrong*.** It is
/// classified into these three, and **only *not there* means the user is asked
/// again as though nothing was remembered.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GetFailure {
    /// The store says the secret is gone. This is the one class that asks the
    /// user to type it again.
    NotThere,
    /// The store refused this once — locked, or the user declined a prompt.
    LockedOrRefused,
    /// The store could not be reached, could not be classified, or did not
    /// answer inside the deadline.
    StoreUnavailable,
}

impl GetFailure {
    /// Rule 4's whole test. Only *not there* asks the user for the secret
    /// again; asking on either of the others would ask them to retype a secret
    /// the machine still holds.
    pub fn asks_the_user_again(self) -> bool {
        matches!(self, GetFailure::NotThere)
    }

    /// Rule 4's second half, *"stated here rather than left to the caller"*:
    /// the other two classes are **ordinary failures, not secret failures**.
    /// Smoozy reports that it could not reach what it remembered, on the thing
    /// that failed, and offers the action again. It does not ask the user for
    /// the secret, and it does not forget it.
    pub fn is_an_ordinary_failure(self) -> bool {
        !self.asks_the_user_again()
    }

    /// Whether meeting this failure erases what is remembered. It never does.
    /// Forgetting here would discard a secret that is present and readable at
    /// the next attempt; what does erase a secret is §5.2's list of events and
    /// nothing else.
    pub fn forgets_the_secret(self) -> bool {
        false
    }

    /// `Desktop app and delivery` S2: **an unmapped status is *store
    /// unavailable* and never *not there***, because an unknown status must
    /// never be read as permission to ask the user for a secret the store may
    /// still hold. Every adapter routes a status it does not recognise here.
    pub fn from_an_unmapped_status() -> Self {
        GetFailure::StoreUnavailable
    }

    /// Rule 5. A call that does not return inside its deadline is **store
    /// unavailable** — never *not there*, which would ask the user for a
    /// secret the store may still hold. This is a rule about how long Smoozy
    /// waits and not a claim about what any store does when it blocks.
    pub fn from_the_deadline_passing() -> Self {
        GetFailure::StoreUnavailable
    }
}

/// Why a put, an erase or an enumerate did not happen. The same two classes as
/// a failed get, less *not there*, which is not a failure of any of them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreFailure {
    LockedOrRefused,
    StoreUnavailable,
}

impl StoreFailure {
    pub fn from_an_unmapped_status() -> Self {
        StoreFailure::StoreUnavailable
    }

    /// Rule 5, for a put, an erase or an enumerate. **No store publishes a way
    /// to cancel a call in flight**, so the adapter that reports this is what
    /// keeps the report true: in `Desktop app and delivery`'s adapter a put
    /// that lands late is undone before any later call, and an erase that
    /// lands late leaves §5.1's mark standing for the sweep to confirm.
    pub fn from_the_deadline_passing() -> Self {
        StoreFailure::StoreUnavailable
    }
}

/// What a probe establishes — `Security and licence` §3.3 S24.
///
/// **Whether a round trip worked, and nothing beyond that.** Whether a failed
/// round trip was no service reachable on the bus, or a service reached that
/// refused or locked, is what Fact 38's unresolved 13 leaves open, and §14
/// item 7 records that as permanently unresolved: no Linux computer exists on
/// this project and none is coming. **A failed probe is never quietly
/// recovered as *nowhere to remember*** — what it drives is §3.3 S25's
/// conservative reading and nothing else.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProbeOutcome {
    RoundTripWorked,
    RoundTripFailed,
}

/// The subject the probe's throwaway item is written under — §3.3 S24. It is
/// reserved: no local record row ever issues it, so a probe item can never
/// collide with a real one.
pub const PROBE_SUBJECT: &str = "probe";

/// The reserved probe item, made here so every adapter writes the same one.
pub fn probe_item() -> HeldSecret {
    HeldSecret::new(
        StoredKind::StoreProbe,
        Subject::issued_by_the_local_record(PROBE_SUBJECT),
    )
}

/// The retired item's service name, kept only so an upgrade can erase it.
/// It is not a stored kind and cannot be used for a put or get.
pub const LEGACY_LICENCE_KEY_NAME: &str = "Smoozy.licence-key";

/// The five operations, and no others — `Security and licence` §3.0 S1, and
/// `Desktop app and delivery` S1, *"and adds no sixth"*.
///
/// **Two rules are in the shape of this trait rather than in its prose.**
/// Rule 5 — no store call is waited on without a deadline — is why every
/// operation takes one. And rule 6 — a value that came out of the store goes
/// to the thing that consumes it and to nothing that draws — is why `get`
/// hands back a [`Plaintext`], which has no route to a screen, while
/// [`SecretStore::enumerate`] hands back subjects and no values at all.
///
/// **Rule 3 is the caller's:** every call blocks, so every call runs off the
/// thread that draws the interface (`Desktop app and delivery` P4, P5). That
/// is not expressible here and is enforced where the adapter is hosted.
pub trait SecretStore {
    /// Writes the value against the item, replacing whatever was there.
    fn put(
        &self,
        item: &HeldSecret,
        value: Plaintext,
        deadline: Duration,
    ) -> Result<(), StoreFailure>;

    /// Reads the value. `Err` is classified by rule 4 and by nothing else.
    fn get(&self, item: &HeldSecret, deadline: Duration) -> Result<Plaintext, GetFailure>;

    /// Removes the item. `Ok` means the store reported the item gone —
    /// confirmed where the system confirms it and inferred nowhere
    /// (`Desktop app and delivery` S10). **An erase that could not unlock is
    /// reported as an erase that did not happen** (§3.3 S31), which is an
    /// `Err` and not an `Ok`.
    fn erase(&self, item: &HeldSecret, deadline: Duration) -> Result<(), StoreFailure>;

    /// Erasure-only compatibility for the retired item. A store without this
    /// route leaves the migration debt standing; it never confirms an erase.
    fn erase_legacy_licence_key(&self, _subject: &Subject, _deadline: Duration) -> Result<(), StoreFailure> {
        Err(StoreFailure::StoreUnavailable)
    }

    /// Answers *is there somewhere to remember at all*. It is a Linux member
    /// of the contract (§3.0 S2) — the only system where it has a mechanism —
    /// and on the other two it answers [`ProbeOutcome::RoundTripWorked`]
    /// without reaching the store, because nothing there waits on it.
    fn probe(&self, deadline: Duration) -> ProbeOutcome;

    /// The subjects this application has stored.
    ///
    /// **The narrowing must not be lost: each system enumerates by a name
    /// filter, not by application** (§3.0 S2) — `SecItemCopyMatching` with the
    /// match limit set to all on macOS, `CredEnumerate` with a name-prefix
    /// filter on Windows, a search on this application's own attribute set on
    /// Linux. That is what the prefix scheme gives, and a naming convention is
    /// not an operating-system boundary.
    fn enumerate(&self, deadline: Duration) -> Result<Vec<HeldSecret>, StoreFailure>;
}

/// The name every adapter builds its item names from — `Security and licence`
/// §3.2 S17, which takes Microsoft's own instruction that *"the name be
/// prefixed by the name of the company implementing the service"*, and §3.1
/// S11, whose enumerate is bounded by *"the service-name prefix Smoozy already
/// adopts"*.
///
/// The name carries the kind and the opaque subject and nothing else.
pub const ITEM_NAME_PREFIX: &str = "Smoozy";

/// Composes one item's name. One function so the three adapters cannot drift
/// from one another, and so [`SecretStore::enumerate`]'s filter and
/// [`SecretStore::put`]'s name are built the same way.
pub fn item_name(item: &HeldSecret) -> String {
    format!(
        "{ITEM_NAME_PREFIX}.{}.{}",
        item.kind.as_str(),
        item.subject.as_str()
    )
}

/// Reads a name back into the pair that made it, for enumerate. An unrecognised
/// name is not this application's item and is skipped rather than guessed at.
pub fn item_from_name(name: &str) -> Option<HeldSecret> {
    let rest = name.strip_prefix(ITEM_NAME_PREFIX)?.strip_prefix('.')?;
    let (kind, subject) = rest.split_once('.')?;
    let kind = StoredKind::ALL.into_iter().find(|k| k.as_str() == kind)?;
    Some(HeldSecret::new(
        kind,
        Subject::issued_by_the_local_record(subject),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn an_item() -> HeldSecret {
        HeldSecret::new(
            StoredKind::MachinePassword,
            Subject::issued_by_the_local_record("server-7"),
        )
    }

    #[test]
    fn only_not_there_asks_the_user_again() {
        assert!(GetFailure::NotThere.asks_the_user_again());
        assert!(!GetFailure::LockedOrRefused.asks_the_user_again());
        assert!(!GetFailure::StoreUnavailable.asks_the_user_again());
    }

    #[test]
    fn no_failed_get_ever_forgets_the_secret() {
        for failure in [
            GetFailure::NotThere,
            GetFailure::LockedOrRefused,
            GetFailure::StoreUnavailable,
        ] {
            assert!(!failure.forgets_the_secret());
        }
    }

    #[test]
    fn an_unmapped_status_is_store_unavailable_and_never_not_there() {
        assert_eq!(
            GetFailure::from_an_unmapped_status(),
            GetFailure::StoreUnavailable
        );
        assert!(!GetFailure::from_an_unmapped_status().asks_the_user_again());
    }

    #[test]
    fn a_passed_deadline_is_store_unavailable_and_never_not_there() {
        assert_eq!(
            GetFailure::from_the_deadline_passing(),
            GetFailure::StoreUnavailable
        );
        assert!(!GetFailure::from_the_deadline_passing().asks_the_user_again());
    }

    #[test]
    fn an_item_name_carries_the_kind_and_the_subject_and_nothing_else() {
        assert_eq!(item_name(&an_item()), "Smoozy.machine-password.server-7");
    }

    #[test]
    fn a_name_reads_back_into_the_pair_that_made_it() {
        let item = an_item();
        assert_eq!(item_from_name(&item_name(&item)), Some(item));
    }

    #[test]
    fn a_name_that_is_not_ours_is_skipped_rather_than_guessed_at() {
        assert_eq!(item_from_name("git:https://example.com"), None);
        assert_eq!(item_from_name("Smoozy.not-a-kind.server-7"), None);
        assert_eq!(item_from_name("Smoozy"), None);
    }

    #[test]
    fn the_probe_item_uses_the_reserved_subject() {
        assert_eq!(probe_item().subject.as_str(), PROBE_SUBJECT);
        assert_eq!(probe_item().kind, StoredKind::StoreProbe);
        assert_eq!(item_from_name(&item_name(&probe_item())), Some(probe_item()));
    }
}
