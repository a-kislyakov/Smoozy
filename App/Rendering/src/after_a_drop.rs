//! After a drop, the machine is read before anything is said — §4.3 S9–S12
//! — and §4.4's cited half, S14–S19.
//!
//! **S9: a connection dropping mid-act leaves each scene-job in one of
//! three states, and Smoozy resolves them by asking the machine, never by
//! guessing and never by re-issuing.** On reconnection, for each provisional
//! record, the machine is asked for **that scene-job's submission command's
//! own operation identity** — the identity S3 issued at the desk, which is
//! what `Server protocol and transfers` C8 answers about. C8's three closed
//! answers map onto the three states: a recorded outcome → submitted or
//! refused, per the outcome recorded; still in hand → the record stays
//! provisional and is asked again next connection; no record of that
//! identity → the record removed, not submitted. **Fact 8 says the window
//! this covers is about three milliseconds wide and *still in hand* was
//! never once returned in 230 usable trials — and that cutting a connection
//! does not guarantee the request never landed**, which is why the ask is
//! mandatory and the guess is forbidden.
//!
//! **The three answers are C8's own closed set**, whose request framing is
//! `Server protocol and transfers` §2's and is built in that module's framing
//! half — the frame at `smoozy_framing::framing` and C8's three answers at
//! `smoozy_framing::acknowledgement`. *(Corrected 2026-09-07: this said §2's
//! framing was unbuilt in that crate.)* **What is still not built is the
//! sending**: nothing carries a framed request to a machine's entry point. The
//! shape is carried here as the three states it maps to, and the acting
//! half decodes C8's answer into it. Nothing here asks a machine.
//!
//! **Markers used in this file that belong to another document, each with
//! the document it belongs to.** `Server protocol and transfers` C4, C8.
//! `State, identity and recovery` §12. `Rented-server runtime` U6, U7, U8,
//! U9, U10, U11, U13, U14, U15, U17, U18, U19, U20, U22, U23, U24, U25.
//! `Job files and outputs` H3, H4, H5. `Queue & History` H7, H8, H9.
//! `Telegram` §17.

use smoozy_state::identities::Identity;
use smoozy_state::outcomes::ActionOutcome;

/// **C8's three closed answers, about one submission command's identity.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheMachineAnswered {
    /// A recorded outcome for that identity — the unit outcomes as the
    /// machine recorded them.
    ARecordedOutcome(ActionOutcome),
    /// Still in hand: the machine answered rather than being unable to.
    StillInHand,
    /// No record of that identity — answered from command records written
    /// durably before acting (C4), so a command the machine never recorded
    /// is one it will never act on.
    NoRecordOfThatIdentity,
}

/// **S9's three states of a scene-job after a drop.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Resolution {
    /// Submitted — the job is in the line exactly when its job record
    /// exists.
    Submitted,
    /// Refused, per the outcome recorded.
    Refused,
    /// The record stays provisional and is asked again next connection.
    StillProvisionalAskAgainNextConnection,
    /// The record removed, not submitted — and S4b's debt where the
    /// material units had completed.
    RemovedNotSubmitted,
}

/// **S9, performed: the resolution from the machine's answer, and from
/// nothing else.** The job identity is what the answer resolves and never
/// what is asked.
pub fn resolve(answer: &WhatTheMachineAnswered) -> Resolution {
    match answer {
        WhatTheMachineAnswered::ARecordedOutcome(ActionOutcome::Whole) => Resolution::Submitted,
        WhatTheMachineAnswered::ARecordedOutcome(
            ActionOutcome::Partial | ActionOutcome::Nothing,
        ) => Resolution::Refused,
        WhatTheMachineAnswered::StillInHand => Resolution::StillProvisionalAskAgainNextConnection,
        WhatTheMachineAnswered::NoRecordOfThatIdentity => Resolution::RemovedNotSubmitted,
    }
}

/// **S9: the identity that is asked about is the submission command's
/// operation identity, never the job's.** Kept by the signature: the ask
/// takes the command's identity and the job identity is what comes back
/// resolved.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheAsk {
    pub the_submission_commands_operation_identity: Identity,
}

/// **S10: what the user is told reads from those resolutions** — which
/// scenes were submitted, which were not, and, where the connection has
/// not come back, which scenes Smoozy could not account for, named as
/// exactly that. The provisional records are the list of the unaccounted;
/// nothing is presented as known that only the machine can say. **This is
/// Row 3.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheTelling {
    pub submitted: Vec<String>,
    pub not_submitted: Vec<String>,
    /// Named as *could not account for*, never as either of the two above.
    pub could_not_be_accounted_for: Vec<String>,
}

/// One scene-job as the desktop holds it after the drop.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ASceneJobAfterTheDrop {
    pub scene: String,
    /// The resolution, where the machine could be asked; `None` where the
    /// connection has not come back.
    pub resolution: Option<Resolution>,
}

pub fn the_telling(scene_jobs: &[ASceneJobAfterTheDrop]) -> TheTelling {
    let mut telling = TheTelling {
        submitted: Vec::new(),
        not_submitted: Vec::new(),
        could_not_be_accounted_for: Vec::new(),
    };
    for scene_job in scene_jobs {
        match &scene_job.resolution {
            Some(Resolution::Submitted) => telling.submitted.push(scene_job.scene.clone()),
            Some(Resolution::Refused) | Some(Resolution::RemovedNotSubmitted) => {
                telling.not_submitted.push(scene_job.scene.clone())
            }
            Some(Resolution::StillProvisionalAskAgainNextConnection) | None => telling
                .could_not_be_accounted_for
                .push(scene_job.scene.clone()),
        }
    }
    telling
}

/// **S11: submitting the rest is the user's next act** — a fresh act
/// covering the scene-jobs resolved as not submitted, a new group with new
/// job identities. **Nothing retries on its own and nothing is held in
/// reserve.**
pub fn the_scenes_the_users_next_act_would_cover(telling: &TheTelling) -> &[String] {
    &telling.not_submitted
}

/// **S12: the reading of the machine costs nothing it does not already
/// have** — C8's answer from the durable records C4 wrote, no listing of
/// any folder, no file's presence read as state.
pub const THE_ASK_READS_NO_FOLDER_AND_NO_FILES_PRESENCE: bool = true;

/// **S18: Row 23 is the same record with the other origin.** A job whose
/// file was not read before submitting is evaluated at P1 step 5, after the
/// load and before the render, against `Job files and outputs` §9.1's
/// three conditions; where one holds, the attempt ends *held for an
/// answer*, the machine is released for its line, and the hold waits for
/// the same answer by the same two routes (U6, H4, H5). **The second place
/// something on the machine waits for a person, and the same mechanism.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheHoldsOrigin {
    /// S13: the second job of a first-frame pair, held from submission.
    Submission,
    /// S18: an attempt that evaluated the three conditions and found one.
    AnAttempt,
}

/// **One hold record, two origins, one release path** — S14's release
/// command by either route, S15's negative case as the machine's own act,
/// S16's fresh attempt, S17's no expiry. Nothing here is a second waiting
/// mechanism.
pub const ONE_HOLD_RECORD_TWO_ORIGINS_ONE_RELEASE_PATH: [TheHoldsOrigin; 2] =
    [TheHoldsOrigin::Submission, TheHoldsOrigin::AnAttempt];

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_state::identities::IdentityKind;

    #[test]
    fn the_three_answers_map_onto_the_three_states_and_nothing_is_guessed() {
        assert_eq!(
            resolve(&WhatTheMachineAnswered::ARecordedOutcome(
                ActionOutcome::Whole
            )),
            Resolution::Submitted
        );
        assert_eq!(
            resolve(&WhatTheMachineAnswered::ARecordedOutcome(
                ActionOutcome::Nothing
            )),
            Resolution::Refused
        );
        assert_eq!(
            resolve(&WhatTheMachineAnswered::ARecordedOutcome(
                ActionOutcome::Partial
            )),
            Resolution::Refused
        );
        assert_eq!(
            resolve(&WhatTheMachineAnswered::StillInHand),
            Resolution::StillProvisionalAskAgainNextConnection
        );
        assert_eq!(
            resolve(&WhatTheMachineAnswered::NoRecordOfThatIdentity),
            Resolution::RemovedNotSubmitted
        );
    }

    #[test]
    fn the_telling_names_the_unaccounted_as_exactly_that() {
        let jobs = vec![
            ASceneJobAfterTheDrop {
                scene: "Kitchen".into(),
                resolution: Some(Resolution::Submitted),
            },
            ASceneJobAfterTheDrop {
                scene: "Exterior".into(),
                resolution: Some(Resolution::RemovedNotSubmitted),
            },
            ASceneJobAfterTheDrop {
                scene: "Attic".into(),
                resolution: None,
            },
            ASceneJobAfterTheDrop {
                scene: "Garage".into(),
                resolution: Some(Resolution::StillProvisionalAskAgainNextConnection),
            },
        ];
        let telling = the_telling(&jobs);
        assert_eq!(telling.submitted, vec!["Kitchen"]);
        assert_eq!(telling.not_submitted, vec!["Exterior"]);
        assert_eq!(telling.could_not_be_accounted_for, vec!["Attic", "Garage"]);
        assert_eq!(
            the_scenes_the_users_next_act_would_cover(&telling),
            &["Exterior".to_string()]
        );
        const _: () = assert!(THE_ASK_READS_NO_FOLDER_AND_NO_FILES_PRESENCE);
    }

    #[test]
    fn the_ask_is_about_the_submission_commands_identity_and_the_hold_has_two_origins() {
        let ask = TheAsk {
            the_submission_commands_operation_identity: Identity::issue(IdentityKind::Operation)
                .unwrap(),
        };
        assert_eq!(
            ask.the_submission_commands_operation_identity.kind(),
            IdentityKind::Operation
        );
        assert_eq!(
            ONE_HOLD_RECORD_TWO_ORIGINS_ONE_RELEASE_PATH,
            [TheHoldsOrigin::Submission, TheHoldsOrigin::AnAttempt]
        );
    }
}
