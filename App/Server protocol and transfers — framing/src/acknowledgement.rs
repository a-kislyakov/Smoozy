//! §4.3 — what an acknowledgement is, and the three closed answers about an
//! operation identity.
//!
//! **What this is.** C7's whole term, and C8's three answers composed from the
//! records the machine holds. **Both ends speak it**, which is why it sits in
//! this half of the module: the supervisor composes an answer from its own
//! store, and the desktop reads one off a result frame.
//!
//! # C7, and why it needs a type rather than a sentence
//!
//! **C7: an acknowledgement says the request is recorded. It never says the
//! work is done.** *"This is the whole of the term, and conflating the two is
//! what Row 291 exists to prevent."*
//!
//! **The conflation is not a mistake somebody makes once; it is what a
//! `Result`-shaped answer invites.** A machine that answered *yes* to a
//! submitted request would be saying both things at once, and no reader could
//! tell which was meant. So [`Acknowledged`] carries the identity it is about
//! and nothing else, **and there is no variant of it, no field on it and no
//! function anywhere here that means *the work is done*** — the work's own
//! ending is a [`WhatBecameOfThisIdentity::ARecordedOutcome`], which is a
//! different answer to a different question asked later.
//!
//! **G2 is what makes that worth building rather than restating**: because an
//! acknowledgement never says the work is done, *it has started* and *it has
//! finished* are two separate correlated answers, and a protocol in which the
//! only answer is the finished one is one in which every noticeable moment is
//! silent.
//!
//! # C8, and what the machine has to hold for the set to be closed
//!
//! **C8: acknowledgement is atomic in the sense that there is no third state.**
//! Because C4 puts the durable record before the act, the machine's answer to
//! *what became of this identity* is one of exactly three things, and the set is
//! closed. **What closes it is the record**, which is
//! `State, identity and recovery` §5.1's and which the machine writes before it
//! acts — so this file composes the answer from that record and from nothing
//! else. **There is no route here to a folder listing, a file's presence or a
//! second reading.**
//!
//! **The middle answer is a requirement rather than an observation.** Fact 8 cut
//! the connection mid-submission 312 times: every trial answered *no record of
//! that identity* or *a recorded outcome*, and ***still in hand* was never once
//! returned* — *"the window in which a submission is accepted but has no
//! recorded outcome is narrower than a shell can aim at."* **So the set is still
//! closed and a machine returning anything else is a fault in
//! `Rented-server runtime` rather than a fourth state**; what the narrow window
//! costs is that the middle branch is untested, which is §12 item 10's ground.
//!
//! # What is deliberately absent
//!
//! **The request that carries the ask.** C9 resolves a lost acknowledgement by
//! asking the machine for the identity, and **that request is not built**. The
//! request set for an installed package version is declared by package
//! inspection (§1.2 item 3), **and package inspection — B5 — is built**, at
//! `smoozy_server_protocol::bootstrap::inspect`, whose
//! `AnInspection::the_declared_request_set` answers which requests a version
//! says it takes. *(Corrected 2026-09-07. This said B5 was still on
//! `smoozy_server_protocol::arriving_later`; that entry was struck on
//! 2026-09-06 when B5 landed, and what is left on it is B8a. The absence this
//! section is about is unchanged — only its reason was stale.)* What is here is
//! the answer's shape and how it is composed; **what is missing is the request
//! itself and the sending of it**, which is what
//! `smoozy_server_protocol::reconnection` says from its own side.
//! [`WHAT_CARRIES_THE_ASK_IS_NOT_BUILT`] says so where a reader meets it.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `State, identity and recovery` §5.1 and §12.
//! `Rented-server runtime` — the machine's own fault, where a fourth answer
//! would be one.

use smoozy_state::identities::Identity;
use smoozy_state::machine_operations::{
    Disposition, RecordedOutcome, WhatTheMachineHoldsUnderAnIdentity,
};

/// **C7. An acknowledgement says the request is recorded. It never says the work
/// is done.**
///
/// **There is one thing it can say**, and that is the whole of the term. It
/// carries the operation identity the request was framed with, because C2
/// matches a result to a request by that identity and by nothing else.
///
/// **What it deliberately cannot say.** There is no *done*, no *succeeded*, no
/// outcome and no field that could be read as one. A caller that wants to know
/// what became of the work asks the question C8 answers, which is a second
/// correlated answer and not this one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Acknowledged {
    /// **C1's operation identity**, chosen by the initiating side before the
    /// request was sent.
    pub the_request: Identity,
}

impl Acknowledged {
    /// **The one thing an acknowledgement says.**
    pub const WHAT_IT_SAYS: &'static str = "the request is recorded";

    /// **C4 and C7 in one act, from the machine's side.** An acknowledgement
    /// exists only where the record was written first, so this takes the fact
    /// that it was rather than a bare identity — there is no way to compose one
    /// for a request the machine has not recorded.
    pub fn because_the_record_was_written(the_request: &Identity) -> Self {
        Acknowledged {
            the_request: the_request.clone(),
        }
    }
}

/// **C7's other half, which is an absence: nothing here can say the work is
/// done.**
///
/// [`Acknowledged`] has one field and no outcome; the only route to an outcome
/// anywhere in this file is [`WhatBecameOfThisIdentity::ARecordedOutcome`],
/// which answers a different question asked later. **The value is `()` because
/// the clause is an absence** — the same shape this module already uses for
/// `NOTHING_ABOUT_A_REQUESTS_FRAMING_IS_A_SECRET` — and what would break it is
/// a variant, a field or a function added here that meant *finished*.
pub const AN_ACKNOWLEDGEMENT_HAS_NO_WAY_TO_SAY_THE_WORK_IS_DONE: () = ();

/// **C8's three answers, and the set is closed.**
///
/// *"The machine's answer to what became of this identity is one of exactly
/// three things."*
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatBecameOfThisIdentity {
    /// **The operation ended, and the outcome is returned.**
    ARecordedOutcome(RecordedOutcome),
    /// **The identity is recorded and has no outcome yet.** It is returned as an
    /// answer, **carrying the latest reconciliation disposition and the attempt
    /// count**, and it is what a repeated request gets for an operation that is
    /// running, was resumed, or is on its second attempt.
    ///
    /// ***Still in hand* is an answer, not a silence** — which is why it carries
    /// what it carries rather than being an empty variant.
    StillInHand {
        latest_disposition: Option<Disposition>,
        attempt_count: u32,
    },
    /// **The request never arrived, or arrived and was refused before it was
    /// recorded. The work did not happen.**
    NoRecordOfThatIdentity,
}

/// **C8's answer, composed from what the machine holds under the identity and
/// from nothing else.**
///
/// **The set is closed by this function's own shape**: the three answers are the
/// three states a record can be in — closed, open, or absent — and there is no
/// input here that could produce a fourth.
///
/// **A command record answers the same three.** G4 gives it a recorded outcome
/// and no desired state, so an open one is *still in hand* carrying no
/// disposition and its first attempt, which is exactly what a command that
/// starts no worker has to say about itself.
pub fn what_became_of_this_identity(
    held: &WhatTheMachineHoldsUnderAnIdentity,
) -> WhatBecameOfThisIdentity {
    if let Some(outcome) = held.recorded_outcome() {
        return WhatBecameOfThisIdentity::ARecordedOutcome(outcome.clone());
    }
    match held {
        WhatTheMachineHoldsUnderAnIdentity::Nothing => {
            WhatBecameOfThisIdentity::NoRecordOfThatIdentity
        }
        _ => {
            let (latest_disposition, attempt_count) = match held.what_an_open_record_carries() {
                Some((disposition, attempts)) => (Some(disposition), attempts),
                None => (None, 1),
            };
            WhatBecameOfThisIdentity::StillInHand {
                latest_disposition,
                attempt_count,
            }
        }
    }
}

/// **What carries the ask is not built, said here rather than left to be
/// discovered.**
///
/// C9 resolves a lost acknowledgement by asking the machine for the identity,
/// and **no request of that name exists anywhere**: §1.2 item 3 makes the
/// request set for a package version a thing package inspection declares.
/// **Package inspection — B5 — is built**, at
/// `smoozy_server_protocol::bootstrap::inspect`, so what is missing is the
/// request itself and the sending of it rather than the declaration route.
/// *(Corrected 2026-09-07: this said package inspection was unbuilt, which
/// stopped being true on 2026-09-06.)* **What is built here is the answer's
/// shape and its composition from the record**, so that whoever builds the
/// request composes it from one description rather than two.
pub const WHAT_CARRIES_THE_ASK_IS_NOT_BUILT: &str =
    "C9's ask has no request name anywhere. The request set is declared by package inspection, \
     which is B5's and is built at smoozy_server_protocol::bootstrap::inspect — so what is \
     missing is the request itself and the sending of it, not the declaration. This file holds \
     the answer and its composition and nothing that puts it on a wire";

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_state::identities::IdentityKind;
    use smoozy_state::machine_operations::{
        CommandRecord, OperationRecord, WhichOperationItActsOn,
    };

    fn an_identity(kind: IdentityKind) -> Identity {
        Identity::issue(kind).expect("the desk has randomness")
    }

    fn a_record() -> OperationRecord {
        OperationRecord::opened(
            an_identity(IdentityKind::Operation),
            an_identity(IdentityKind::Job),
            "render",
        )
    }

    /// **C7: the whole of what an acknowledgement says is that the request is
    /// recorded.**
    #[test]
    fn an_acknowledgement_says_the_request_is_recorded_and_carries_the_identity() {
        let identity = an_identity(IdentityKind::Operation);
        let acknowledged = Acknowledged::because_the_record_was_written(&identity);
        assert_eq!(acknowledged.the_request, identity);
        assert_eq!(Acknowledged::WHAT_IT_SAYS, "the request is recorded");
    }

    /// **C8: a closed record answers with its recorded outcome.**
    #[test]
    fn a_closed_record_answers_with_the_outcome_it_recorded() {
        let mut record = a_record();
        record
            .close(RecordedOutcome::TheKindsOwn("the render finished".into()), None)
            .unwrap();
        assert_eq!(
            what_became_of_this_identity(&WhatTheMachineHoldsUnderAnIdentity::AnOperationRecord(
                record
            )),
            WhatBecameOfThisIdentity::ARecordedOutcome(RecordedOutcome::TheKindsOwn(
                "the render finished".into()
            ))
        );
    }

    /// ***Still in hand* is an answer, not a silence**, and it carries the
    /// latest disposition and the attempt count — which is what a repeated
    /// request gets for an operation that was resumed or is on its second
    /// attempt.
    #[test]
    fn an_open_record_answers_still_in_hand_carrying_its_disposition_and_attempt_count() {
        let mut record = a_record();
        record
            .a_run_took_this_branch(Disposition::ReRunOnce, an_identity(IdentityKind::Run))
            .unwrap();
        assert_eq!(
            what_became_of_this_identity(&WhatTheMachineHoldsUnderAnIdentity::AnOperationRecord(
                record
            )),
            WhatBecameOfThisIdentity::StillInHand {
                latest_disposition: Some(Disposition::ReRunOnce),
                attempt_count: 2,
            }
        );
    }

    /// **A record no run has taken a branch for is still in hand and says so**,
    /// carrying no disposition rather than a made-up one.
    #[test]
    fn a_record_no_run_has_reconciled_is_still_in_hand_and_carries_no_disposition() {
        assert_eq!(
            what_became_of_this_identity(&WhatTheMachineHoldsUnderAnIdentity::AnOperationRecord(
                a_record()
            )),
            WhatBecameOfThisIdentity::StillInHand {
                latest_disposition: None,
                attempt_count: 1,
            }
        );
    }

    /// **The work did not happen**, and that is the answer to an identity the
    /// machine never recorded.
    #[test]
    fn an_identity_the_machine_never_recorded_answers_that_the_work_did_not_happen() {
        assert_eq!(
            what_became_of_this_identity(&WhatTheMachineHoldsUnderAnIdentity::Nothing),
            WhatBecameOfThisIdentity::NoRecordOfThatIdentity
        );
    }

    /// **A command record answers the same three**, an open one as *still in
    /// hand* with no disposition, a closed one with its outcome.
    #[test]
    fn a_command_record_answers_the_same_three_and_never_a_fourth() {
        let mut command = CommandRecord::submitted(
            an_identity(IdentityKind::Operation),
            WhichOperationItActsOn::TheCallerNamedIt(an_identity(IdentityKind::Operation)),
        );
        assert_eq!(
            what_became_of_this_identity(&WhatTheMachineHoldsUnderAnIdentity::ACommandRecord(
                command.clone()
            )),
            WhatBecameOfThisIdentity::StillInHand {
                latest_disposition: None,
                attempt_count: 1,
            }
        );
        command
            .close(RecordedOutcome::TheKindsOwn("nothing was rendering".into()))
            .unwrap();
        assert!(matches!(
            what_became_of_this_identity(&WhatTheMachineHoldsUnderAnIdentity::ACommandRecord(
                command
            )),
            WhatBecameOfThisIdentity::ARecordedOutcome(_)
        ));
    }

    /// **The absence is named rather than glossed.**
    #[test]
    fn what_carries_the_ask_says_which_clause_owes_it() {
        assert!(WHAT_CARRIES_THE_ASK_IS_NOT_BUILT.contains("package inspection"));
        assert!(WHAT_CARRIES_THE_ASK_IS_NOT_BUILT.contains("B5"));
    }
}
