//! **§4.3 performed — the machine asked about each provisional record after
//! a drop, and the telling composed from what it answered.**
//!
//! # The ask is mandatory and the guess is forbidden
//!
//! **A connection dropping mid-act leaves each scene-job in one of three
//! states, and Smoozy resolves them by asking the machine, never by guessing
//! and never by re-issuing.** Fact 8 is why: the window this machinery covers
//! is about three milliseconds wide, ***still in hand* was never once
//! returned in the 230 usable trials**, **and cutting a connection does not
//! guarantee the request never landed.** So the narrowness of the window is
//! not a reason to skip the ask — it is the reason the ask is the only thing
//! that can answer.
//!
//! **The identity asked about is the submission command's own operation
//! identity, issued at the desk before the request was sent — never the
//! job's.** That ordering is what let 230 cut connections produce zero
//! disagreements between the machine's answer and the queue.
//!
//! # Nothing is re-issued and nothing retries on its own
//!
//! **A lost acknowledgement is resolved by identity and the work is never
//! re-issued under a fresh one** (`State, identity and recovery` §12 rule 4).
//! **S11: submitting the rest is the user's next act** — a fresh act with a
//! new group and new job identities — so there is no retry loop anywhere in
//! this file and nothing is held in reserve.
//!
//! # S12, kept by the shape of the ask
//!
//! **The reading of the machine costs nothing it does not already have**: the
//! ask is `Server protocol and transfers` C8's answer from the durable
//! records that document's C4 wrote. **It needs no listing of any folder and
//! reads no file's presence as state**, which is kept here by
//! [`AsksTheMachineAboutOneCommand`] having one method that takes an identity
//! and answers one of three values.
//!
//! # A record that stays provisional stays provisional
//!
//! ***Still in hand* is an answer**, and the machine having answered is
//! exactly what makes it different from the connection not coming back. The
//! record stays provisional and is asked again next connection; **it is never
//! resolved by waiting, by a second ask in the same run, or by a deadline.**
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` C4, C8. `State,
//! identity and recovery` D5.

use smoozy_state::identities::Identity;

use crate::after_a_drop::{
    resolve, the_telling, ASceneJobAfterTheDrop, Resolution, TheAsk, TheTelling,
    WhatTheMachineAnswered,
};

/// Asking the machine about one command's identity, behind a trait.
///
/// **One method, one identity, three answers.** There is no route from here
/// to a folder listing, a file's presence or a second reading, which is S12
/// kept by the shape rather than by a rule.
///
/// **`None` is the connection not coming back**, which is not one of C8's
/// three answers: the machine answered nothing at all, and the scene-job is
/// among the unaccounted rather than resolved.
pub trait AsksTheMachineAboutOneCommand {
    fn what_the_machine_says(
        &mut self,
        ask: &TheAsk,
    ) -> impl std::future::Future<Output = Option<WhatTheMachineAnswered>> + Send;
}

/// One provisional record, as the desktop holds it at the reconnection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AProvisionalRecord {
    /// The scene the user named, for the telling.
    pub scene: String,
    /// The job identity, which is what the answer resolves.
    pub job: Identity,
    /// **The submission command's own operation identity, issued at the desk
    /// before anything was sent** — the identity the ask is about.
    pub the_submission_commands_operation_identity: Identity,
    /// **Whether this scene-job's material units had completed** when the
    /// drop caught it, which is what decides whether S4b's cleanup debt is
    /// owed for it once the machine answers *no record of that identity*.
    pub its_material_units_had_completed: bool,
}

/// What the reconnection's resolution produced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheReconnectionResolved {
    /// Each provisional record with what it resolved to, in the order it was
    /// asked.
    pub per_scene_job: Vec<(AProvisionalRecord, Option<Resolution>)>,
}

impl WhatTheReconnectionResolved {
    /// **S10, composed from the resolutions and from nothing else.** Nothing
    /// is presented as known that only the machine can say.
    pub fn the_telling(&self) -> TheTelling {
        let scene_jobs: Vec<ASceneJobAfterTheDrop> = self
            .per_scene_job
            .iter()
            .map(|(record, resolution)| ASceneJobAfterTheDrop {
                scene: record.scene.clone(),
                resolution: resolution.clone(),
            })
            .collect();
        the_telling(&scene_jobs)
    }

    /// **The records that stay provisional and are asked again next
    /// connection** — *still in hand*, and the ones the connection never came
    /// back for.
    pub fn still_provisional(&self) -> Vec<&AProvisionalRecord> {
        self.per_scene_job
            .iter()
            .filter(|(_, resolution)| {
                matches!(
                    resolution,
                    None | Some(Resolution::StillProvisionalAskAgainNextConnection)
                )
            })
            .map(|(record, _)| record)
            .collect()
    }

    /// **S4b: the cleanup debt is taken where the machine answered *no record
    /// of that identity* for a scene-job whose material units had
    /// completed.**
    ///
    /// **Nothing is owed for any other resolution**, and nothing is owed for a
    /// record whose material never published: keeping a material set standing
    /// for ever under a dead identity on a machine the user pays for is the
    /// shape `Queue & History` X3 exists to remove, and inventing a removal
    /// for material that was never placed would be the opposite fault.
    pub fn the_cleanup_debt_is_owed_for(&self) -> Vec<&AProvisionalRecord> {
        self.per_scene_job
            .iter()
            .filter(|(record, resolution)| {
                record.its_material_units_had_completed
                    && matches!(resolution, Some(Resolution::RemovedNotSubmitted))
            })
            .map(|(record, _)| record)
            .collect()
    }
}

/// **S9, performed: every provisional record asked about, in order.**
///
/// **The ask is made for every provisional record and the walk does not stop
/// at the first unanswered one**: a scene-job the connection could not answer
/// for is among the unaccounted, and stopping would leave the ones after it
/// unaccounted too when the machine could have said.
pub async fn resolve_the_provisional_records(
    asks: &mut impl AsksTheMachineAboutOneCommand,
    records: Vec<AProvisionalRecord>,
) -> WhatTheReconnectionResolved {
    let mut per_scene_job = Vec::with_capacity(records.len());
    for record in records {
        let ask = TheAsk {
            the_submission_commands_operation_identity: record
                .the_submission_commands_operation_identity
                .clone(),
        };
        let resolution = asks.what_the_machine_says(&ask).await.as_ref().map(resolve);
        per_scene_job.push((record, resolution));
    }
    WhatTheReconnectionResolved { per_scene_job }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::testing::a_job;
    use smoozy_state::identities::{Identity, IdentityKind};
    use smoozy_state::outcomes::ActionOutcome;

    fn a_record(scene: &str, its_material_units_had_completed: bool) -> AProvisionalRecord {
        AProvisionalRecord {
            scene: scene.to_string(),
            job: a_job(),
            the_submission_commands_operation_identity: Identity::issue(IdentityKind::Condition)
                .expect("randomness"),
            its_material_units_had_completed,
        }
    }

    /// A machine that answers one thing per scene, in the order they are
    /// asked.
    struct Answers(Vec<Option<WhatTheMachineAnswered>>);
    impl AsksTheMachineAboutOneCommand for Answers {
        async fn what_the_machine_says(&mut self, _ask: &TheAsk) -> Option<WhatTheMachineAnswered> {
            if self.0.is_empty() {
                return None;
            }
            self.0.remove(0)
        }
    }

    fn run<F: std::future::Future>(mut future: F) -> F::Output {
        use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
        fn no_op(_: *const ()) {}
        fn clone(data: *const ()) -> RawWaker {
            RawWaker::new(data, &VTABLE)
        }
        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, no_op, no_op, no_op);
        let waker = unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) };
        let mut context = Context::from_waker(&waker);
        let mut future = unsafe { std::pin::Pin::new_unchecked(&mut future) };
        loop {
            match future.as_mut().poll(&mut context) {
                Poll::Ready(value) => return value,
                Poll::Pending => std::thread::yield_now(),
            }
        }
    }

    /// **C8's three answers map onto S9's three states, and a connection
    /// that never came back is a fourth thing entirely** — the unaccounted,
    /// named as exactly that.
    #[test]
    fn the_three_answers_resolve_and_a_silence_is_the_unaccounted() {
        let mut asks = Answers(vec![
            Some(WhatTheMachineAnswered::ARecordedOutcome(
                ActionOutcome::Whole,
            )),
            Some(WhatTheMachineAnswered::NoRecordOfThatIdentity),
            Some(WhatTheMachineAnswered::StillInHand),
            None,
        ]);
        let resolved = run(resolve_the_provisional_records(
            &mut asks,
            vec![
                a_record("Kitchen", false),
                a_record("Exterior", false),
                a_record("Bathroom", false),
                a_record("Hall", false),
            ],
        ));
        let telling = resolved.the_telling();
        assert_eq!(telling.submitted, vec!["Kitchen".to_string()]);
        assert_eq!(telling.not_submitted, vec!["Exterior".to_string()]);
        assert_eq!(
            telling.could_not_be_accounted_for,
            vec!["Bathroom".to_string(), "Hall".to_string()],
            "*still in hand* and a silence are both unaccounted, and neither is presented as known"
        );
        assert_eq!(resolved.still_provisional().len(), 2);
    }

    /// **The walk asks about every record and does not stop at the first the
    /// connection could not answer for.** The fault is introduced by putting
    /// the silence first: stopping there would leave the scenes after it
    /// unaccounted when the machine could have said.
    #[test]
    fn a_record_the_connection_could_not_answer_for_does_not_stop_the_rest() {
        let mut asks = Answers(vec![
            None,
            Some(WhatTheMachineAnswered::ARecordedOutcome(
                ActionOutcome::Whole,
            )),
        ]);
        let resolved = run(resolve_the_provisional_records(
            &mut asks,
            vec![a_record("Hall", false), a_record("Kitchen", false)],
        ));
        let telling = resolved.the_telling();
        assert_eq!(telling.submitted, vec!["Kitchen".to_string()]);
        assert_eq!(telling.could_not_be_accounted_for, vec!["Hall".to_string()]);
    }

    /// **S4b: the debt is owed for *no record of that identity* on a
    /// scene-job whose material units had completed, and for nothing
    /// else.**
    #[test]
    fn the_cleanup_debt_is_owed_only_where_the_material_published_and_no_record_stands() {
        let mut asks = Answers(vec![
            Some(WhatTheMachineAnswered::NoRecordOfThatIdentity),
            Some(WhatTheMachineAnswered::NoRecordOfThatIdentity),
            Some(WhatTheMachineAnswered::StillInHand),
        ]);
        let resolved = run(resolve_the_provisional_records(
            &mut asks,
            vec![
                // Its material published: the debt is owed.
                a_record("Kitchen", true),
                // Its material never published: nothing to remove.
                a_record("Exterior", false),
                // Still in hand: the record stays provisional, and a debt
                // here would remove material a job may still be made of.
                a_record("Bathroom", true),
            ],
        ));
        let owed = resolved.the_cleanup_debt_is_owed_for();
        assert_eq!(owed.len(), 1);
        assert_eq!(owed[0].scene, "Kitchen");
    }

    /// **S11: submitting the rest is the user's next act**, over the
    /// scene-jobs resolved as not submitted and over no others.
    #[test]
    fn the_users_next_act_covers_the_not_submitted_and_never_the_unaccounted() {
        let mut asks = Answers(vec![
            Some(WhatTheMachineAnswered::ARecordedOutcome(
                ActionOutcome::Nothing,
            )),
            Some(WhatTheMachineAnswered::StillInHand),
        ]);
        let resolved = run(resolve_the_provisional_records(
            &mut asks,
            vec![a_record("Exterior", false), a_record("Bathroom", false)],
        ));
        let telling = resolved.the_telling();
        assert_eq!(
            crate::after_a_drop::the_scenes_the_users_next_act_would_cover(&telling),
            &["Exterior".to_string()]
        );
        const _: () = assert!(crate::after_a_drop::THE_ASK_READS_NO_FOLDER_AND_NO_FILES_PRESENCE);
    }

    /// **The identity asked about is the submission command's and never the
    /// job's**, read off the ask this module composes.
    #[test]
    fn the_ask_carries_the_submission_commands_identity_and_not_the_jobs() {
        struct KeepsTheAsk(Option<Identity>);
        impl AsksTheMachineAboutOneCommand for KeepsTheAsk {
            async fn what_the_machine_says(
                &mut self,
                ask: &TheAsk,
            ) -> Option<WhatTheMachineAnswered> {
                self.0 = Some(ask.the_submission_commands_operation_identity.clone());
                Some(WhatTheMachineAnswered::StillInHand)
            }
        }
        let record = a_record("Kitchen", false);
        let its_command = record.the_submission_commands_operation_identity.clone();
        let its_job = record.job.clone();
        let mut asks = KeepsTheAsk(None);
        let _ = run(resolve_the_provisional_records(&mut asks, vec![record]));
        assert_eq!(asks.0.as_ref(), Some(&its_command));
        assert_ne!(asks.0.as_ref(), Some(&its_job));
    }
}
