//! **One repeating failure is said once** — `Desktop app and delivery` §14 Q1
//! to Q13.
//!
//! **Read the head of `src/presentation/mod.rs` first.** This file is inside
//! the shape rule and keeps every part of it.
//!
//! # What is actually built here, and what was already true
//!
//! **Most of §14 is not a mechanism at all: it falls out of the key.** Q1 puts
//! the presentation key on the **condition identity**, and
//! `State, identity and recovery` F28 — the key's own supplier — says two
//! failures share a reason when they share that identity, *"not when they share
//! their words, and not when they share the unit that met them"*. So:
//!
//! - **Q4: there is already only one record.** `State, identity and recovery`
//!   §8.3 composes the condition key, looks it up, and where the key already
//!   has an identity **that identity is used and no second record is raised**.
//!   The store keeps it rather than the caller:
//!   [`crate::record::failure_records`] carries a unique index on the condition
//!   key. **A hundred files refused by one full mount are one record before
//!   anything here is asked.**
//! - **Q2: the floor is §8.9's and this file moves it in neither direction.**
//!   Nothing here composes a key, narrows one or widens one; the only thing it
//!   does with a key is compare it with keys it has already seen.
//!   [`tests::nothing_here_composes_a_key`] is what says so.
//! - **Q5: the key already normalises the mount.** A pool of channels refused
//!   on one full mount resolves to one condition key because the key carries
//!   the mount rather than the path — and the normalising is the raiser's,
//!   never this side's.
//! - **Q9: one entry per identity in the interface**, which the page keeps by
//!   drawing entries against the identity the record carries.
//!   [`tests::the_pages_entry_key_and_the_presentation_key_are_one_identity`]
//!   holds the two together so they cannot drift apart.
//! - **Q6: unit outcomes are never suppressed.** *"What is said once is the
//!   reason, not the tally."* There is nothing in this file an outcome record
//!   or a unit could reach, which is the whole of how that is kept.
//! - **Q3: this is condition records' and not endings'.** An ending names a
//!   subject outcome that happened once; there is one per subject and nothing
//!   to repeat. [`TheThingsSaidOnce::a_post_is_owed`] says so in as many words
//!   rather than widening past the row's own example.
//!
//! **What is left, and it is the one thing: Q13.** *"A later failure resolving
//! to a standing identity needs no post at all … the repeat's correct treatment
//! on the notification surface is silence."* Q4 has already refused the second
//! record and Q9 already has the entry on screen — so the operating system must
//! be told **on the first and never again**, and that is what
//! [`TheThingsSaidOnce`] is.
//!
//! **Why that matters more than it reads.** Q10's replace-by-identity is
//! unestablished on every system (Section 22 item 27), and Q13 is what makes
//! §14 hold whichever way that closes: a repeat posts nothing at all, so there
//! is nothing for a system that will not replace to add a second copy of.
//! **Built the other way — post every time and lean on replace — a full mount
//! refusing a hundred files would put a hundred notifications on the user's
//! screen on any system where replace turns out not to work**, and nothing in
//! Smoozy would report it.
//!
//! # Run-held, and that is `State, identity and recovery` §11.1's class
//!
//! What this holds is *has the operating system been told about this key in
//! this run* — process memory for the life of the run and written nowhere,
//! which is that document's run-held class. **Nothing durable is written from
//! it and nothing branches on it but the post**, so its total loss costs one
//! repeated notification and nothing else.
//!
//! **Q12 rides on the key rather than on this store.** A withdrawn record is
//! spent and a condition that returns is a **fresh identity** against the same
//! key — a different presentation key, so it is said once again whatever this
//! file remembers. [`TheThingsSaidOnce::the_record_ended`] forgets the old one
//! anyway, because a set that only grows is a set that grows.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `State, identity and recovery` F28.
//!
//! **`State, identity and recovery`'s other markers are written out with that
//! document at every site below** — this plan defines clauses of the same short
//! names, and a header list cannot tell the two apart.

use std::collections::HashSet;
use std::sync::Mutex;

use super::the_presentation_key::ThePresentationKey;

/// **What the operating system has already been told about in this run** —
/// §14 Q13.
///
/// One per run, held on [`crate::start::Started`] beside the record and the
/// diary, because that is the one thing every command reaches and because it is
/// run-held.
#[derive(Debug, Default)]
pub struct TheThingsSaidOnce {
    /// **Keys and nothing else.** `State, identity and recovery` F28: the key
    /// holds identities, so a change to what a sentence says can never change
    /// what is or is not said again.
    already_said: Mutex<HashSet<ThePresentationKey>>,
}

impl TheThingsSaidOnce {
    pub fn new() -> Self {
        TheThingsSaidOnce::default()
    }

    /// **Whether the operating system is owed a post for this key** — Q10 and
    /// Q13.
    ///
    /// **True the first time in this run and false afterwards.** A later
    /// failure resolving to a standing identity is the same key, so it answers
    /// *no* and nothing is posted, which is Q13's silence built rather than
    /// remembered.
    ///
    /// **Q3: an ending is not this machinery's.** A record of kind *ending*
    /// names a subject outcome that happened once — there is one per subject
    /// and there is nothing to repeat — so a key that is not a condition's is
    /// answered *yes* and is not remembered here. Widening this to endings
    /// would be Q6's fault by a different door: a group's finishing and a job's
    /// ending are separate facts the user is owed each time they happen.
    ///
    /// **Asking is telling.** There is deliberately no *has it been said*
    /// reader beside this: two calls where one would do is how a caller ends up
    /// asking and then not posting, and the register would then say a thing was
    /// said that never was.
    pub fn a_post_is_owed(&self, key: &ThePresentationKey) -> bool {
        if !matches!(key, ThePresentationKey::ACondition(_)) {
            return true;
        }
        let Ok(mut already_said) = self.already_said.lock() else {
            // A poisoned lock is not a reason to go quiet about a failure.
            // **Rule 8's line: Smoozy telling the truth about what happened is
            // never past it**, and the cost of the other answer is a repeat
            // notification where the cost of this one is silence about a real
            // failure.
            return true;
        };
        already_said.insert(key.clone())
    }

    /// **The record ended, so the key is spent** — Q12, and
    /// `State, identity and recovery` §8.8's three endings.
    ///
    /// A condition that returns is a fresh identity against the same key, so it
    /// is a different presentation key and is said once again whether or not
    /// this is called. **What this is for is that the set does not only grow**
    /// across a long run.
    pub fn the_record_ended(&self, key: &ThePresentationKey) {
        if let Ok(mut already_said) = self.already_said.lock() {
            already_said.remove(key);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_state::identities::{Identity, IdentityKind};

    fn a_condition() -> ThePresentationKey {
        ThePresentationKey::ACondition(
            Identity::issue(IdentityKind::Condition).expect("the desk has randomness"),
        )
    }

    /// **Q13, and it is the sentence the whole of §14 rests on: a hundred
    /// failures on one condition identity are one post.**
    ///
    /// This is Row 288's own example — every file in a job failing to land in a
    /// folder that is full — asked of the thing that answers it.
    #[test]
    fn a_hundred_failures_on_one_condition_identity_are_one_post() {
        let said_once = TheThingsSaidOnce::new();
        let full_mount = a_condition();
        let mut posts = 0;
        for _ in 0..100 {
            if said_once.a_post_is_owed(&full_mount) {
                posts += 1;
            }
        }
        assert_eq!(
            posts, 1,
            "**Q13: a later failure resolving to a standing identity needs no \
             post at all.** A hundred messages buries the one that matters"
        );
    }

    /// **Q7: a second, unrelated failure is never suppressed.**
    ///
    /// *"A different subject or a different kind is a different condition
    /// identity by construction and never resolves to the same one."* Getting
    /// this wrong in the quiet direction breaks the promise that Smoozy always
    /// tells the truth about what happened.
    #[test]
    fn a_second_unrelated_failure_is_said() {
        let said_once = TheThingsSaidOnce::new();
        assert!(said_once.a_post_is_owed(&a_condition()));
        assert!(said_once.a_post_is_owed(&a_condition()));
    }

    /// **Q12: a withdrawn record is spent, and a condition that returns is a
    /// fresh identity said once again.**
    ///
    /// The second key is a fresh identity because that is what §8.3 issues when
    /// the key has no standing record — which is the whole reason nothing here
    /// has to decide when a suppression expires.
    #[test]
    fn a_condition_that_returns_is_said_once_again() {
        let said_once = TheThingsSaidOnce::new();
        let first = a_condition();
        assert!(said_once.a_post_is_owed(&first));
        said_once.the_record_ended(&first);
        assert!(said_once.a_post_is_owed(&a_condition()));
    }

    /// **Q3: this machinery is condition records' and not endings'.**
    ///
    /// A job's ending, a group's finishing and a machine idle are not
    /// suppressed here and are not remembered here — each is a fact that
    /// happened once, and Q6's *what is said once is the reason, not the tally*
    /// is the same rule from the other side.
    #[test]
    fn an_ending_is_not_this_machinerys() {
        let said_once = TheThingsSaidOnce::new();
        let job = ThePresentationKey::AJobsEnding(
            Identity::issue(IdentityKind::Job).expect("the desk has randomness"),
        );
        assert!(said_once.a_post_is_owed(&job));
        assert!(said_once.a_post_is_owed(&job));
    }

    /// **Q9 and Q1 are one identity, and this is what stops them drifting.**
    ///
    /// The page draws one entry per record against the identity the record
    /// carries; §14 keys the post on the presentation key. **For a condition
    /// those are the same identity**, and the day they stop being the same is
    /// the day one repeating failure is one entry on screen and two posts in
    /// the operating system, with nothing anywhere saying so.
    #[test]
    fn the_pages_entry_key_and_the_presentation_key_are_one_identity() {
        let identity = Identity::issue(IdentityKind::Condition).expect("the desk has randomness");
        let key = ThePresentationKey::ACondition(identity.clone());
        assert_eq!(key.as_text(), format!("condition:{}", identity.as_text()));
    }

    /// **Q2 and Q5: the floor is `State, identity and recovery` §8.9's and this
    /// file moves it in neither direction.**
    ///
    /// Held by reading this file's own working half. **A key composed here
    /// would be a second answer to the question that section already answers**,
    /// and a suppression drawn too wide silences a second, unrelated failure —
    /// which is the direction Q2 says is not cosmetic.
    #[test]
    fn nothing_here_composes_a_key() {
        let source = include_str!("said_once.rs");
        let working = source.split("#[cfg(test)]").next().unwrap_or_default();
        let working: String = working
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join("\n");
        for composing in [
            "ConditionKey",
            "Identity::issue",
            "as_text",
            "format!",
            "mount",
        ] {
            assert!(
                !working.contains(composing),
                "src/presentation/said_once.rs names {composing}. **Q2: the \
                 floor is `State, identity and recovery` §8.9's and this \
                 section moves it in neither direction**, and **Q5: the key \
                 already normalises the mount.** What this file does with a key \
                 is compare it with keys it has seen"
            );
        }
    }
}
