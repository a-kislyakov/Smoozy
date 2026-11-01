//! **The presentation key** — `Desktop app and delivery` §14 Q1 and Q10, and
//! `State, identity and recovery` §8.9 F28, which supplies the identity Q1 is
//! written on.
//!
//! **This file is piece G's writing and piece C's file, and it arrived here on
//! 2026-09-09 unchanged.** It was written into `src/notifications/` on
//! 2026-09-09 because piece G needed the key before `src/presentation/`
//! existed, and its own head said it would move here when piece C landed. It
//! did, with not a character of the body altered, and the module line in
//! `src/notifications/mod.rs` is gone.
//!
//! **It is the key piece C presents failures on, piece F suppresses on, and
//! piece G replaces notification posts on.** Nothing in it is about
//! notifications and nothing in it may become so.
//!
//! **[`ThePresentationKey::as_text`] is a written form the operating system
//! now holds.** Piece G keys every post it makes on it, and macOS was observed
//! taking a post down again by that exact string. **Changing what `as_text`
//! produces breaks replace-and-take-down on posts already delivered**, which
//! is a failure nothing in this application can see: the old post simply stays
//! on the user's screen and a second one appears beside it.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `State, identity and recovery` F28.

use smoozy_state::identities::{Identity, IdentityKind};

/// **What one presented thing is keyed on** — §14 Q1: *"the presentation key
/// is the condition identity, everywhere a failure is presented"*, with the
/// other three of the four notification kinds keyed on the durable subject
/// their fact is about.
///
/// **Q1 is about failures and this enum is wider, deliberately.** Q10 keys a
/// **post** on the identity, and §13's four kinds include three that are not
/// failures at all — so a key that only knew conditions would leave three
/// kinds with nothing to be replaced on, and T24's replace-by-identity is the
/// mechanism a repeating fact is said once with on every one of them.
///
/// **Nothing here is composed from words.** `State, identity and recovery`
/// F28: two failures share a reason when they share a **condition identity**,
/// *"not when they share their words, and not when they share the unit that
/// met them"* — so the key holds identities and nothing else, and a change to
/// what a sentence says can never change what is replaced.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ThePresentationKey {
    /// **A live condition, by its condition identity** — §14 Q1. The identity
    /// is `State, identity and recovery` §8.3's, issued by whatever raised
    /// the condition and looked up through `State, identity and recovery` F11's
    /// condition key, which stays
    /// inside that section and never reaches here.
    ACondition(Identity),
    /// **One job's own ending**, by the job identity.
    AJobsEnding(Identity),
    /// **A group of scene-jobs finishing**, by the group identity §2.2 issues
    /// when an all-scenes submission is composed — §13 T11.
    AGroupsFinishing(Identity),
    /// **A machine that has had nothing to do**, by the server row's identity
    /// — §13 T14.
    AMachineIdle(Identity),
}

impl ThePresentationKey {
    /// **The written form the operating system replaces on** — §13 T24.
    ///
    /// **The kind is part of it and that is not decoration.** F28's own
    /// example is that *a disk-full and a permission refusal on the same
    /// mount are two, because the kind differs*; the same reasoning applies
    /// here from the other side, where one server row can carry both a
    /// connection condition and an idle reading at once and they are two
    /// facts rather than one replacing the other.
    pub fn as_text(&self) -> String {
        match self {
            ThePresentationKey::ACondition(identity) => {
                format!("condition:{}", identity.as_text())
            }
            ThePresentationKey::AJobsEnding(identity) => format!("job:{}", identity.as_text()),
            ThePresentationKey::AGroupsFinishing(identity) => {
                format!("group:{}", identity.as_text())
            }
            ThePresentationKey::AMachineIdle(identity) => format!("idle:{}", identity.as_text()),
        }
    }

    /// **The durable subject the key is about**, which is what an event about
    /// it is posted against and what P8 discards it for when the subject is
    /// gone.
    pub fn the_subject(&self) -> &Identity {
        match self {
            ThePresentationKey::ACondition(identity)
            | ThePresentationKey::AJobsEnding(identity)
            | ThePresentationKey::AGroupsFinishing(identity)
            | ThePresentationKey::AMachineIdle(identity) => identity,
        }
    }

    /// **The identity kind each variant is built on**, so that a key composed
    /// from the wrong sort of identity is a thing a test can catch rather than
    /// a wrong replacement nobody sees. `State, identity and recovery` §2.2
    /// says which issuer issues each.
    pub fn the_identity_kind_it_is_built_on(&self) -> IdentityKind {
        match self {
            ThePresentationKey::ACondition(_) => IdentityKind::Condition,
            ThePresentationKey::AJobsEnding(_) => IdentityKind::Job,
            ThePresentationKey::AGroupsFinishing(_) => IdentityKind::Group,
            ThePresentationKey::AMachineIdle(_) => IdentityKind::LocalServer,
        }
    }

    /// Whether the key is built on an identity of the kind it names.
    pub fn is_well_formed(&self) -> bool {
        self.the_subject().kind() == self.the_identity_kind_it_is_built_on()
    }
}

/// **What one drawn failure is keyed on** — §14 Q1's key, widened by three
/// members for the things §13's notification kinds do not cover: the licence
/// record's ending, an outcome record's ending, and L8a's read that did not
/// answer (the last two added 2026-09-19, the Gemini catch-up of step 10a,
/// Area 4, F-2).
///
/// **Why this exists beside [`ThePresentationKey`] rather than inside it.**
/// §13's four kinds are each about a durable subject that **has** an identity,
/// and [`ThePresentationKey::the_subject`] answers it — which is what the
/// notification side reads to tell the queue what a post is about. **§16
/// presents one thing that list does not cover**: an ending recorded against a
/// historical subject with no identity of its own: the licence record. A member inside
/// [`ThePresentationKey`] would leave `the_subject` with nothing to answer, and
/// the honest answer there is that the four kinds it serves all have one.
///
/// **Q1 is untouched.** It fixes what a key is *for* — one presentation per
/// condition identity, so that *said once* falls out rather than being enforced
/// — and says nothing about the list of things a key can be about. That list is
/// this code's own supply, and this is it. Added 2026-09-12 out of round 1 of
/// the code audit of what is drawn inside the window (F-11), where the code had
/// been **minting a job identity** for a licence record's ending and keying a
/// licence failure as a job's.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TheKeyAFailureIsDrawnOn {
    /// One of §13's four, on the identity it is about.
    OnAnIdentity(ThePresentationKey),
    /// **The licence record's own ending.** There is one licence record per
    /// installation in the historical schema, so there is one legacy key —
    /// carrying no identity, rather than one invented to fill the space.
    TheLicenceRecordsEnding,
    /// **An ending recorded against a multi-part action's outcome record**, by
    /// that record's own operation identity. M4 names the outcome record as one
    /// of the places an ending is recorded, and §13's notification kinds do not
    /// cover it — the licence record's reason for a member of its own, so it
    /// sits here beside that one and not in [`ThePresentationKey`].
    ///
    /// **It is not a job's ending.** Until 2026-09-19 it was keyed
    /// [`ThePresentationKey::AJobsEnding`], written `job:<identity>`, over an
    /// identity that is not a job's. Found by the Gemini catch-up of step 10a,
    /// Area 4 (F-2) — the sibling of F-11's licence case that repair did not
    /// reach.
    AnOutcomesEnding(Identity),
    /// **A read of the failure records that did not answer**, about the
    /// subject it was asked about — L8a's drawing, which is a presentation and
    /// not a record. It carries the subject's sort and written form, and no
    /// identity is read out of it. Until 2026-09-19 it was keyed as a job's
    /// ending over whatever subject was asked about — a server row included —
    /// which is F-2's fault by another route (the Gemini catch-up of step 10a,
    /// Area 4).
    AReadThatDidNotAnswer(String),
}

/// The licence record's ending, written. **Not an identity and nothing may read
/// it as one**, which is the same care
/// [`crate::record::failure_records`] takes with the subject's own written form.
const THE_LICENCE_RECORDS_ENDING: &str = "licence-record-ending";

impl TheKeyAFailureIsDrawnOn {
    /// **The written form**, which is what said-once counts on, what the
    /// operating system replaces on, and what the page is handed.
    pub fn as_text(&self) -> String {
        match self {
            TheKeyAFailureIsDrawnOn::OnAnIdentity(key) => key.as_text(),
            TheKeyAFailureIsDrawnOn::TheLicenceRecordsEnding => {
                THE_LICENCE_RECORDS_ENDING.to_string()
            }
            TheKeyAFailureIsDrawnOn::AnOutcomesEnding(identity) => {
                format!("outcome:{}", identity.as_text())
            }
            TheKeyAFailureIsDrawnOn::AReadThatDidNotAnswer(about) => format!("unread:{about}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn an_identity(kind: IdentityKind) -> Identity {
        Identity::issue(kind).expect("the desk has randomness")
    }

    /// **Q10: one post per identity.** Two keys of the same variant over the
    /// same identity are the same key, so the second post replaces the first
    /// rather than being added to it.
    #[test]
    fn the_same_identity_gives_the_same_key() {
        let condition = an_identity(IdentityKind::Condition);
        assert_eq!(
            ThePresentationKey::ACondition(condition.clone()).as_text(),
            ThePresentationKey::ACondition(condition).as_text()
        );
    }

    /// **`State, identity and recovery` F28's *the kind differs* rule, applied
    /// to the one subject that can carry two facts at once.**
    ///
    /// A server row is the subject of both the connection kind and the idle
    /// kind. Were the key the bare identity, an idle post would take the place
    /// of a standing *could not be restored* on the same row — which is the
    /// harm §22 item 61 names, arrived at from inside Smoozy rather than from
    /// a system that will not take a post down.
    #[test]
    fn one_server_row_carrying_two_facts_gives_two_keys() {
        let server = an_identity(IdentityKind::LocalServer);
        let condition = an_identity(IdentityKind::Condition);
        assert_ne!(
            ThePresentationKey::AMachineIdle(server).as_text(),
            ThePresentationKey::ACondition(condition).as_text()
        );
    }

    /// **Nothing in the key is composed from words** — F28.
    ///
    /// The text is the variant's own name and an identity's hexadecimal
    /// rendering, and there is nowhere in it a sentence could reach.
    #[test]
    fn the_key_holds_no_words() {
        let key = ThePresentationKey::AJobsEnding(an_identity(IdentityKind::Job));
        let text = key.as_text();
        let (before, after) = text.split_once(':').expect("the key names its kind");
        assert_eq!(before, "job");
        assert_eq!(after.len(), 32);
        assert!(after.chars().all(|c| c.is_ascii_hexdigit() && !c.is_uppercase()));
    }

    /// **The licence record's ending is keyed on nothing and never on a minted
    /// identity** — retained historical subject handling.
    #[test]
    fn the_licence_records_ending_is_keyed_without_an_identity() {
        let key = TheKeyAFailureIsDrawnOn::TheLicenceRecordsEnding;
        assert_eq!(key.as_text(), "licence-record-ending");
        // And it is no job's ending, however many jobs there are.
        let job = TheKeyAFailureIsDrawnOn::OnAnIdentity(ThePresentationKey::AJobsEnding(
            an_identity(IdentityKind::Job),
        ));
        assert_ne!(key.as_text(), job.as_text());
    }

    /// **A key about an identity is written exactly as it was before**, so the
    /// written form the operating system already holds does not move.
    #[test]
    fn wrapping_a_key_does_not_change_what_it_is_written_as() {
        let condition = an_identity(IdentityKind::Condition);
        let key = ThePresentationKey::ACondition(condition);
        assert_eq!(
            TheKeyAFailureIsDrawnOn::OnAnIdentity(key.clone()).as_text(),
            key.as_text()
        );
    }

    /// **F-2 of the Gemini catch-up, Area 4: an outcome record's ending is
    /// written as an outcome's**, never as a job's, over the operation identity
    /// it is built on.
    #[test]
    fn an_outcomes_ending_is_written_as_an_outcomes_and_no_jobs() {
        let operation = an_identity(IdentityKind::Operation);
        let key = TheKeyAFailureIsDrawnOn::AnOutcomesEnding(operation.clone());
        assert_eq!(key.as_text(), format!("outcome:{}", operation.as_text()));
        assert_ne!(
            key.as_text(),
            ThePresentationKey::AJobsEnding(operation).as_text()
        );
    }

    /// A key built on the wrong sort of identity is catchable.
    #[test]
    fn a_key_says_which_sort_of_identity_it_is_built_on() {
        assert!(ThePresentationKey::AGroupsFinishing(an_identity(IdentityKind::Group))
            .is_well_formed());
        assert!(!ThePresentationKey::AGroupsFinishing(an_identity(IdentityKind::Job))
            .is_well_formed());
    }
}
