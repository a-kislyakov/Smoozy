//! §9 — the machine-level *setup in progress* state.
//!
//! **The shape**: *"`Rented-server runtime` owns a machine-level durable *setup
//! in progress* state that holds reconciliation back; Server Setup sets and
//! clears it through the same command path."* And `State, identity and recovery`
//! §6's Step 0, which is that state. **Row 110 is what needs it.**
//!
//! **P6. Ordering is what keeps Row 110's promise and it is stated as an ordering
//! rather than as a coordination.** The setup and the queue do not talk to each
//! other; **the state is the only thing between them, and it is held by the
//! machine rather than by Smoozy, so it works with nobody connected.** Smoozy is
//! not required to be running at any moment of it.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `State, identity and recovery` §6 Step 0. `Server
//! Setup` — its safe re-run, and Rows 106, 110, 112 and 114.

use smoozy_state::identities::Identity;

/// **P1. One machine-level durable state, in the package area, holding whether a
/// setup is in progress and the operation identity of the setup run that set
/// it.**
///
/// **The identity is what lets a later reading say which run it belongs to;
/// nothing infers the state from a setup worker being present or absent.**
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SetupInProgressState {
    #[serde(with = "crate::written_form::an_identity")]
    pub the_run_that_set_it: Identity,
}

/// **P2. While it is set, reconciliation does not run and nothing is admitted but
/// A11a's one exception.**
///
/// **Not the interrupted job, not a re-run, not a derivative, not a role worker's
/// work — the exclusion is at machine level.** The exception is the worker start
/// of the setup run the state names; **reconciliation is not excepted and still
/// does not run.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhileItIsSet {
    ReconciliationDoesNotRun,
    /// A11a's, and it is the only one.
    OnlyTheRunTheStateNamesIsAdmitted,
}

/// **P2.** Whether reconciliation may run.
pub fn reconciliation_may_run(state: Option<&SetupInProgressState>) -> bool {
    state.is_none()
}

/// **P3. `Server Setup` sets it and clears it through the ordinary command path,
/// and the supervisor never sets or clears it itself.**
///
/// **In particular no timer clears it, and the supervisor does not clear it
/// because a setup worker has gone**: a setup interrupted by a restart is
/// precisely the case where the state must survive with no worker behind it.
///
/// The trait is what `Server Setup` reaches through, and **it carries no clearing
/// route that does not name a run** — a bare *clear* would be exactly the act P3
/// forbids the supervisor.
pub trait SetsAndClearsTheState {
    /// **P4. Setting it is durable before the restart a setup step causes.** That
    /// is the whole of what Row 110 needs from this side: **the machine that comes
    /// back on its own comes back with the state still set**, so the interrupted
    /// job is not started before the setup finishes even though *"the machine
    /// re-runs without waiting for Smoozy or anyone else"*.
    fn set(&self, by_the_run: &Identity) -> Result<(), String>;

    /// **P5. Clearing it runs reconciliation.**
    ///
    /// Without that, a machine whose setup finished would start nothing until its
    /// next supervisor start, and Row 110's *"the interrupted job is re-run after
    /// it"* would not happen. **So the clear is two acts in one durable write: the
    /// state goes, and reconciliation runs immediately afterwards** — which is the
    /// second of the two moments S9 names.
    ///
    /// **The second act is owed rather than performed here, and the ordering is
    /// kept by the type**: this is the only route to a clear anywhere, it hands
    /// back [`TheStateIsGoneAndReconciliationIsOwed`], and
    /// [`crate::supervisor::the_moment_a_clear_reconciles_at`] is the only thing
    /// that takes one. **A bare clear does not exist**, which is what P3 forbids the
    /// supervisor and what a caller that simply forgot the second act would
    /// otherwise have.
    ///
    /// **It was a handed-in closure until 2026-09-07** and it is a returned value
    /// now, for one reason: reconciliation reads and writes the same supervisor
    /// that holds this store, so a closure the store carried could not reach it.
    fn clear(
        &self,
        by_the_run: &Identity,
    ) -> Result<TheStateIsGoneAndReconciliationIsOwed, String>;

    fn read(&self) -> Result<Option<SetupInProgressState>, String>;
}

/// **P5's second act, owed and not yet done.**
///
/// **The function that takes it is named in one place and named in code**, so a
/// rename cannot leave a note pointing at something that is no longer there —
/// which is what had happened to this note by 2026-09-12, when it still sent a
/// reader to `reconcile_after_the_clear`, a name nothing has carried since the
/// clear became a returned value. The name is
/// [`crate::supervisor::the_moment_a_clear_reconciles_at`] and this module's own
/// tests name it as a value, so the compiler answers for it.
///
/// **The state is gone durably by the time this exists**, which is the ordering
/// P5 states: a reconciliation that ran over a state whose removal had not
/// reached the device would be the one act P2 holds back arriving early.
///
/// **Its field is private to this module and it is made by
/// [`SetsAndClearsTheState::clear`] alone**, so the clear and the reconciliation
/// cannot be separated by anybody forgetting the second one. It is the same shape
/// `Queue & History` uses for the outstanding acts on its own line, and it has
/// the same honest limit: a caller may drop it, which the compiler warns about
/// and which no caller in this workspace does.
#[must_use = "P5 makes the clear two acts: the state goes, and reconciliation runs \
              immediately afterwards. Hand this to the one function that takes this \
              value, which is reconciliation's own clear moment."]
#[derive(Debug)]
pub struct TheStateIsGoneAndReconciliationIsOwed(());

impl TheStateIsGoneAndReconciliationIsOwed {
    /// **The one constructor, and it is the store's.** It is visible inside this
    /// crate because the store that performs the clear is in this crate's acting
    /// half; nothing outside can make one, so nothing outside can reach
    /// reconciliation's clear moment without having cleared the state.
    pub(crate) fn the_state_is_gone() -> Self {
        TheStateIsGoneAndReconciliationIsOwed(())
    }
}

/// **P7. Nothing clears the state on the strength of time passing or of Smoozy
/// reconnecting.**
///
/// **A machine whose setup was abandoned holds the state until a setup run clears
/// it** — either by completing, or by `Server Setup`'s recovery from its recorded
/// safe boundary on a re-run.
///
/// **The state is reported on every health answer** (§12 V9), so the first thing a
/// reconnecting Smoozy learns is that a setup is unfinished, and **`Server Setup`
/// owns what the user is offered.** §17 records the check on that.
pub const NOTHING_CLEARS_IT_ON_TIME_OR_ON_A_RECONNECTION: () = ();

/// **P8. A setup run that finds the state already set and naming a different run
/// does not clear it. It reports the state and its identity, and what happens next
/// is `Server Setup`'s safe re-run.**
///
/// **§16 item 27 records what this clause cannot reach and it is left as the seam
/// check wrote it rather than re-derived here.** P8's subject is *a setup run
/// that finds the state already set and naming a different run*, **and under P2
/// and A11a no such run is ever admitted while the state stands** — so the clause
/// states a report and gives it to something that cannot run.
///
/// **A5b now gives the report an actor for the admission moment**: the admission
/// point refuses the submission and names the operation identity the state holds.
/// **A builder implementing P8 literally writes a report path inside a setup
/// worker that is never started**, and the refusal A5b states is the only one that
/// ever fires. **Nothing downstream reads the difference**, since `Server Setup`'s
/// safe re-run is reached through its own recovery command rather than through P8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatASetupRunFindingTheStateDoes {
    /// It is the run the state names: it may proceed (A11a), and it is the one
    /// that will clear it.
    ItIsTheRunTheStateNames,
    /// **P8's own sentence.** It reports the state and its identity and does not
    /// clear it.
    ReportsTheStateAndItsIdentityAndDoesNotClearIt(Identity),
}

/// **P8.**
pub fn what_a_setup_run_finding_the_state_does(
    state: &SetupInProgressState,
    this_run: &Identity,
) -> WhatASetupRunFindingTheStateDoes {
    if &state.the_run_that_set_it == this_run {
        WhatASetupRunFindingTheStateDoes::ItIsTheRunTheStateNames
    } else {
        WhatASetupRunFindingTheStateDoes::ReportsTheStateAndItsIdentityAndDoesNotClearIt(
            state.the_run_that_set_it.clone(),
        )
    }
}

/// **P6's ordering, stated as a value rather than as a coordination.**
///
/// **Smoozy is not required to be running at any moment of it.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheOrdering {
    /// The state is set, durably (P4).
    TheStateIsSetDurably,
    /// The setup step causes a restart; the machine comes back with the state
    /// still set.
    TheMachineComesBackWithTheStateStillSet,
    /// The setup run clears it, and the clear runs reconciliation (P5).
    TheClearRunsReconciliation,
    /// Only then is the interrupted job re-run.
    TheInterruptedJobIsReRun,
}

impl TheOrdering {
    /// Row 110's promise, in order.
    pub const IN_ORDER: [TheOrdering; 4] = [
        TheOrdering::TheStateIsSetDurably,
        TheOrdering::TheMachineComesBackWithTheStateStillSet,
        TheOrdering::TheClearRunsReconciliation,
        TheOrdering::TheInterruptedJobIsReRun,
    ];
}

#[cfg(test)]
mod tests {
    use smoozy_state::identities::IdentityKind;

    use super::*;

    fn an_identity() -> Identity {
        Identity::issue(IdentityKind::Operation).expect("randomness")
    }

    /// **The one function that takes P5's second act, named as a value so a
    /// rename fails here rather than leaving a note pointing at nothing.**
    ///
    /// This is not an assertion about behaviour and does not pretend to be: the
    /// value is the function itself, and the crate stops compiling if the name
    /// moves. The note above [`TheStateIsGoneAndReconciliationIsOwed`] had been
    /// pointing at a name nothing carried since the clear stopped taking a
    /// closure, and nothing anywhere said so.
    #[test]
    fn the_second_act_is_the_function_the_note_above_names() {
        let second_act: fn(
            TheStateIsGoneAndReconciliationIsOwed,
        ) -> crate::supervisor::ReconciliationMoment =
            crate::supervisor::the_moment_a_clear_reconciles_at;
        let _ = second_act;
    }

    #[test]
    fn reconciliation_does_not_run_while_the_state_is_set() {
        let state = SetupInProgressState {
            the_run_that_set_it: an_identity(),
        };
        assert!(!reconciliation_may_run(Some(&state)));
        assert!(reconciliation_may_run(None));
    }

    #[test]
    fn a_setup_run_that_did_not_set_the_state_reports_it_and_does_not_clear_it() {
        let state = SetupInProgressState {
            the_run_that_set_it: an_identity(),
        };
        let another = an_identity();
        assert_eq!(
            what_a_setup_run_finding_the_state_does(&state, &another),
            WhatASetupRunFindingTheStateDoes::ReportsTheStateAndItsIdentityAndDoesNotClearIt(
                state.the_run_that_set_it.clone()
            )
        );
    }

    #[test]
    fn the_run_the_state_names_is_the_one_that_may_proceed() {
        let run = an_identity();
        let state = SetupInProgressState {
            the_run_that_set_it: run.clone(),
        };
        assert_eq!(
            what_a_setup_run_finding_the_state_does(&state, &run),
            WhatASetupRunFindingTheStateDoes::ItIsTheRunTheStateNames
        );
    }

    #[test]
    fn the_ordering_puts_the_clear_before_the_re_run() {
        let order = TheOrdering::IN_ORDER;
        let clear = order
            .iter()
            .position(|step| *step == TheOrdering::TheClearRunsReconciliation)
            .expect("the clear is a step");
        let re_run = order
            .iter()
            .position(|step| *step == TheOrdering::TheInterruptedJobIsReRun)
            .expect("the re-run is a step");
        assert!(clear < re_run);
        assert_eq!(order[0], TheOrdering::TheStateIsSetDurably);
    }

    #[test]
    fn while_it_is_set_the_only_admission_is_the_run_it_names() {
        // The two members are the whole of what P2 says, and both are named.
        let both = [
            WhileItIsSet::ReconciliationDoesNotRun,
            WhileItIsSet::OnlyTheRunTheStateNamesIsAdmitted,
        ];
        // **A third thing P2 says fails the build here**, the match being total
        // and carrying no default arm — where the length of a list written out
        // inside the test could never have been false.
        for (at, says) in both.iter().enumerate() {
            let where_the_list_puts_it = match says {
                WhileItIsSet::ReconciliationDoesNotRun => 0,
                WhileItIsSet::OnlyTheRunTheStateNamesIsAdmitted => 1,
            };
            assert_eq!(at, where_the_list_puts_it, "{says:?} is not where the list puts it");
        }
    }
}
