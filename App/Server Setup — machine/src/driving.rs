//! The worker, the driving and the stop — `Server Setup` §3.3.
//!
//! **Y7's shape is the spec's own, not a limitation.** *"A connection lost
//! part-way through stops it, and nothing resumes on its own"* is
//! Creator-stated, and the machine cannot honour it alone: the machine **had
//! not learnt its connection ended within the five minutes it was watched**,
//! and by `sshd_config(5)`'s documented `TCPKeepAlive` default of *"yes"* and
//! the kernel's `tcp_keepalive_time` *"Default: 2hours"* it would not have for
//! about two hours (Fact 57). **Two hours is no use to a step-by-step run
//! either**, so the shape below is unmoved.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Rented-server runtime` A5a, A5b, A11, G21, P4, P5, P7,
//! X8, X12, X14. `Server protocol and transfers` C3, C20. `Servers` K3.

use std::time::Duration;

use crate::run::Boundary;

/// **Y7: which side performs a step, and the two are driven differently.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhoPerformsTheStep {
    /// **The setup worker**, commanded one step at a time: the fetch, the
    /// unpack, the `ldd` reads, the publication and the read-backs. **Each runs
    /// to its boundary and the worker then waits for the next command.**
    TheSetupWorker,
    /// **The SSH command channel**, by the machine's own elevating command with
    /// the password on standard input (`elevation` in `smoozy-server-setup` E3): the package
    /// installs, the driver work, the hand-over and the restart. **The
    /// desktop records the step's start against the run, and the boundary is
    /// written when the desktop's verification read passes.**
    ///
    /// **An elevated command's process is the connection's child**, so a
    /// connection lost mid-step kills the step mid-flight — the boundary then
    /// honestly names the last one passed, which is Fact 49's hard-cut shape.
    TheCommandChannelUnderElevation,
}

/// **Y7, performed: which side each step of a run is driven from.**
pub fn who_performs(step: crate::ready_action::Step) -> WhoPerformsTheStep {
    use crate::ready_action::Step;
    match step {
        // The readings, the fetch, the unpack, the publication and the final
        // verification are the worker's. **So is B15's plain deletion**: the
        // folder is Smoozy's own, under the Blender area, and needs no
        // elevation (the 38's piece 14c). **And B15's second route keeps the
        // step the worker's**: the manager's removal is elevated and goes out on
        // the command channel from the desk, as the tooling step's install does,
        // and the worker's half is reading that the build is gone, which writes
        // the boundary (step 10c's owed piece 6).
        Step::Readings
        | Step::Fetch
        | Step::BlenderPublication
        | Step::BlenderRemoval
        | Step::TelegramMaterial
        | Step::Done => WhoPerformsTheStep::TheSetupWorker,
        // The tooling step installs a package where the tool is absent, the
        // libraries step installs packages, the driver step installs or
        // repairs, and the restart is issued — all four are elevated.
        Step::Tooling | Step::Libraries | Step::Driver | Step::Restart => {
            WhoPerformsTheStep::TheCommandChannelUnderElevation
        }
    }
}

/// **Y7: the driving side refreshes the run at its own short cadence while the
/// run is open.**
///
/// **The refresh is not a step command**: it continues at its cadence for as
/// long as the run is open, **elevated steps included**, so the gap is never
/// measured across a step that sends the worker nothing — a driver install or a
/// long dependency chain runs for minutes without a worker command and **must
/// not end a run that is proceeding correctly**.
///
/// **The cadence is `Servers` K3's keepalive interval and is not a second
/// figure.** It is read from that crate rather than written again here, so a
/// change to the connection's own cadence moves this with it.
pub const THE_REFRESH_CADENCE: Duration = smoozy_servers_values::keepalive::KEEPALIVE_INTERVAL;

/// **Y7d: while words are moving, the refresh runs about once a second.**
/// Added 2026-10-25, the eighteenth notes, piece 2b (note 5: *"Server output
/// small 3-line window lags behind the actual output"*).
///
/// **It is the second cadence beside [`THE_REFRESH_CADENCE`] and never a
/// replacement for it**: the refresh is the run's state reading, so a step's
/// words and the Y7b and Y7c figures arrived up to fifteen seconds late while
/// it ran only at the keepalive interval. **[`THE_COMMAND_GAP_WINDOW`] is
/// measured between refreshes at either cadence**, so the faster one only
/// shortens gaps and never ends a run, and the window's value is unchanged.
///
/// **It is also how often the worker tells its count and prints its progress
/// lines** (Y7b, Y7c): *"no more often than once a second"*.
pub const THE_REFRESH_CADENCE_WHILE_WORDS_MOVE: Duration = Duration::from_secs(1);

/// **Y7d: how many readings in a row that bring nothing new send the refresh
/// back to [`THE_REFRESH_CADENCE`].** The plan's own figure: *"goes back to
/// `THE_REFRESH_CADENCE` after five readings in a row bring nothing new"*.
pub const HOW_MANY_QUIET_READINGS_END_THE_QUICK_CADENCE: u32 = 5;

/// **Y7d, held for one run on the driving side: which of the two cadences the
/// next refresh waits.**
///
/// **A run opens at the quick cadence**, so the first readings' words show at
/// once. **A reading brings something new** where it carried new words or a
/// moved Y7b or Y7c count, or where a desktop-run command's stream delivered
/// bytes within the last second — the caller says which, since only it holds
/// the reading. **Five quiet readings in a row** put the refresh back at
/// [`THE_REFRESH_CADENCE`]; the next new thing brings the quick cadence back.
/// Nothing is written anywhere for it, and a machine program older than this
/// answers the quicker refresh as it answers any refresh.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TheRefreshCadence {
    quiet_in_a_row: u32,
}

impl TheRefreshCadence {
    /// **The cadence a run opens at: the quick one.**
    pub fn as_a_run_opens() -> TheRefreshCadence {
        TheRefreshCadence { quiet_in_a_row: 0 }
    }

    /// **A reading was taken**, and it brought something new or it did not.
    pub fn a_reading_brought(&mut self, something_new: bool) {
        self.quiet_in_a_row = if something_new { 0 } else { self.quiet_in_a_row.saturating_add(1) };
    }

    /// **How long the next refresh waits.**
    pub fn until_the_next_reading(&self) -> Duration {
        if self.quiet_in_a_row >= HOW_MANY_QUIET_READINGS_END_THE_QUICK_CADENCE {
            THE_REFRESH_CADENCE
        } else {
            THE_REFRESH_CADENCE_WHILE_WORDS_MOVE
        }
    }
}

/// **Y7: the command-gap window, measured *between refreshes* and never against
/// any machine-return cost.**
///
/// **The value is Build's and this is it: ninety seconds.** `Server Setup` §13
/// item 7 says the mechanism is *"one number in one place"* and that the window
/// **has never been run** — it wants a machine and about two hours of it, and
/// the Creator ruled on 2026-08-31 that no further machine is rented to answer
/// a question. **So this is a decision taken with the failure directions in
/// hand rather than a measurement, and it is written where a later measurement
/// would replace it.**
///
/// **Why ninety.** The window is measured between refreshes, and the refreshes
/// continue through elevated steps, so what it has to survive is a run of
/// missed refreshes rather than a slow step. `Servers`' own break declaration
/// lands between **45 and 60 seconds** after the connection stops answering
/// (its `EARLIEST_DECLARATION` and `LATEST_DECLARATION`), so **a window shorter
/// than 60 seconds would end runs on connections `Servers` has not yet called
/// broken** — the run would die before the thing that killed it was even
/// declared. Ninety is six refreshes and comfortably past the latest
/// declaration.
///
/// **What each direction costs, in §13 item 7's own terms:** a window too short
/// *"ends a run the user was still driving between slow steps — costing a
/// recovery run, never harm"*; too long *"delays the exclusion's release"*.
/// Neither direction damages a machine, and the exclusion this delays is
/// [`one_run_at_a_time`]'s, whose cost is a wait.
pub const THE_COMMAND_GAP_WINDOW: Duration = Duration::from_secs(90);

/// **Y7: what a worker whose refresh gap exceeds its window does.**
///
/// *"Ends the run at its current boundary with the kind's own ending **ended at
/// boundary N, not completed**, journalled."* That is what turns a dead
/// connection into a stopped run rather than a worker waiting forever, holding
/// [`one_run_at_a_time`]'s exclusion and blocking every package commit
/// (`Rented-server runtime` G21).
pub fn the_window_has_elapsed(since_the_last_refresh: Duration) -> bool {
    since_the_last_refresh > THE_COMMAND_GAP_WINDOW
}

/// **Y8: the user's stop is the durable desired state, honoured at a boundary
/// and never by a signal.**
///
/// The stop command writes *should be stopped* durably first (`Rented-server
/// runtime` X8 step 2); the worker checks it at each boundary and ends there.
/// **No signal is ever sent to the worker or its children**, because a setup
/// step's children include the package system mid-transaction, and **killing
/// that is how a machine gets into the state Row 114 exists to get it out of.**
pub fn a_signal_is_sent_to_the_worker_or_its_children(_stopping: bool) -> bool {
    false
}

/// **Y8a: stopping is one control offered for the whole of a run's life, and it
/// has two outcomes — decided by whether the machine can take the durable
/// write, never by the user choosing between two acts.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheStopDoes {
    /// **Where the machine answers** — a machine-side step in flight, an
    /// elevated step in flight, or the worker waiting at a boundary for its
    /// next command. *Should be stopped* is written durably first, the worker
    /// ends at its next boundary, and for an elevated step **the driving side
    /// stops issuing and closes the step's channel** so the run ends at the
    /// last boundary passed.
    TheDurableWriteAndAnEndingAtTheNextBoundary,
    /// **Where the machine is down, which is the restart wait
    /// (`restart` in `smoozy-server-setup` T4) and nothing else.** The durable write cannot be
    /// made and **no request leaves at all**, so the stop reaches the desktop's
    /// waiting alone. **That is the design's *cannot be called back* as a
    /// property of the machine's unreachability rather than a rule written to
    /// enforce it.**
    TheDesktopsWaitingAlone,
}

/// **Y8a, performed.**
pub fn what_the_stop_does(the_machine_answers: bool) -> WhatTheStopDoes {
    if the_machine_answers {
        WhatTheStopDoes::TheDurableWriteAndAnEndingAtTheNextBoundary
    } else {
        WhatTheStopDoes::TheDesktopsWaitingAlone
    }
}

/// **Y8a: a stop made during the restart wait is not held for delivery when the
/// machine answers.**
///
/// No request leaves, so there is **no pending-command record to resolve**
/// (`Server protocol and transfers` C3, and C20's rule that a sent request
/// cannot be un-sent), and `restart` in `smoozy-server-setup` T6 stands unchanged: the mark
/// stands, the machine-level state stands, `Servers`' reconnect policy for that
/// marked period goes on running, and the setup is picked up on return by T5's
/// recovery run.
///
/// **The earliest end available to the user after a restart is that recovery
/// run's next boundary.**
pub fn a_stop_during_the_restart_wait_is_held_for_delivery(_stopped: bool) -> bool {
    false
}

/// **Y8b: what a stop leaves, said for the state and the interrupted job rather
/// than for the machine alone.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhatAStopLeaves {
    /// **The machine-level state stands.** [`crate::recovery`] S6's clearing
    /// set is closed at two — a run reaching *done*, and a recovery run the
    /// user asked to abandon the machine-changing work — **and a stop is
    /// neither.**
    pub the_machine_level_state_stands: bool,
    /// **The interrupted job stays held back**, because [`crate::run`] Y2 holds
    /// it on that standing state and only the clear runs reconciliation
    /// (`Rented-server runtime` P5). **So a stopped machine-changing run reaches
    /// the same place `restart` in `smoozy-server-setup` T6a names for the user walking away,
    /// by a control rather than by neglect.**
    pub the_interrupted_job_stays_held_back: bool,
}

/// **Y8b, performed. An additive run sets no machine-level state
/// ([`crate::run`] Y1), so stopping one holds nothing back at all**: no
/// admission changes and no job waits on it.
pub fn what_a_stop_leaves(class: crate::run::RunClass) -> WhatAStopLeaves {
    let machine_changing = matches!(class, crate::run::RunClass::MachineChanging);
    WhatAStopLeaves {
        the_machine_level_state_stands: machine_changing,
        the_interrupted_job_stays_held_back: machine_changing,
    }
}

/// **Y8: a restart already begun cannot be called back.**
///
/// Stopping then only stops the waiting, **the machine still restarts**, and
/// the standing state holds the machine safe until the user picks the setup up
/// again — the spec's own sentence, kept by `Rented-server runtime` P4 and P7.
/// **Stopping leaves exactly what a lost connection leaves.**
pub fn a_restart_already_begun_can_be_called_back(_stopped: bool) -> bool {
    false
}

/// **Y9: one setup run at a time per machine, enforced at the supervisor's
/// admission point, and declared in `declarations` in `smoozy-server-setup`.**
///
/// *"A second setup submission while one run is open is refused with the open
/// run reported"* — **Smoozy's own rule, per machine and never per app**;
/// several machines run at once.
///
/// **The rule that acts on the declaration, and the refusal itself, live where
/// the admission point does.** `Rented-server runtime` **A5a** performs the
/// fifth declaration as a numbered admission rule, and **A5b** makes an
/// operation A5a or A11 holds back **refused at the admission point rather than
/// queued**, the refusal naming the operation standing in its way. **So this
/// clause's *refused with the open run reported* is read out of the document
/// that owns the admission point rather than promised from this end.**
///
/// **The refusal is narrow by A5b's own terms**, reaching A5a's exclusion and
/// A11's state and nothing else: an operation waiting on a held resource still
/// waits, so nothing here touches the render kind's queueing.
pub fn one_run_at_a_time(a_setup_run_is_already_open_on_this_machine: bool) -> WhetherItIsAdmitted {
    if a_setup_run_is_already_open_on_this_machine {
        WhetherItIsAdmitted::RefusedNamingTheOpenRun
    } else {
        WhetherItIsAdmitted::Admitted
    }
}

/// What the admission point does with a second setup submission.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhetherItIsAdmitted {
    Admitted,
    /// **Refused rather than queued**, naming the operation standing in its
    /// way. Before `Rented-server runtime` A5a and A5b, every outcome that
    /// document named for an operation it would not admit was a **wait** — so a
    /// second submission would have queued unrefused and unreported.
    RefusedNamingTheOpenRun,
}

/// **Y9: the exclusion is per machine and never per app.**
pub const SEVERAL_MACHINES_RUN_AT_ONCE: bool = true;

/// **Y9: the one refusal this rule cannot prevent, and it is not read as a
/// broken machine.**
///
/// *"What it cannot serialise is a Blender the user started"* — with one
/// Blender holding the add-on store lock, an install fired from another refuses
/// in under a second at a failing status naming the session that holds it.
/// That is [`crate::addons`] A8's and **cannot be prevented from here**.
pub const THE_ADD_ON_STORE_LOCK_IS_NOT_SERIALISED_BY_THIS_RULE: bool = true;

/// **Y8's kind of ending, and [`crate::run`]'s boundary is what it names.**
pub fn ended_at_boundary_not_completed(at: Boundary) -> String {
    format!("ended at {at:?}, not completed")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ready_action::Step;
    use crate::run::RunClass;

    #[test]
    fn the_elevated_steps_are_the_four_that_reach_the_package_system_or_the_boot() {
        for step in [Step::Tooling, Step::Libraries, Step::Driver, Step::Restart] {
            assert_eq!(
                who_performs(step),
                WhoPerformsTheStep::TheCommandChannelUnderElevation
            );
        }
        for step in [
            Step::Readings,
            Step::Fetch,
            Step::BlenderPublication,
            Step::BlenderRemoval,
            Step::TelegramMaterial,
            Step::Done,
        ] {
            assert_eq!(who_performs(step), WhoPerformsTheStep::TheSetupWorker);
        }
    }

    #[test]
    fn the_window_is_longer_than_the_latest_moment_servers_declares_a_break() {
        // Y7 and §13 item 7. A window shorter than that would end runs on
        // connections `Servers` has not yet called broken.
        assert!(THE_COMMAND_GAP_WINDOW > smoozy_servers_values::keepalive::LATEST_DECLARATION);
        assert!(THE_COMMAND_GAP_WINDOW > smoozy_servers_values::keepalive::EARLIEST_DECLARATION);
        // Six refreshes.
        assert_eq!(
            THE_COMMAND_GAP_WINDOW.as_secs(),
            6 * THE_REFRESH_CADENCE.as_secs()
        );
    }

    #[test]
    fn the_window_is_not_budgeted_against_any_steps_own_length() {
        // The refresh continues at its cadence through elevated steps, so a
        // driver install of 149 seconds does not end the run. This test is what
        // says that is deliberate: the window is shorter than that install and
        // that is correct.
        assert!(THE_COMMAND_GAP_WINDOW < Duration::from_secs(149));
        assert!(the_window_has_elapsed(Duration::from_secs(91)));
        assert!(!the_window_has_elapsed(Duration::from_secs(89)));
    }

    /// **Y7d: a run opens at the quick cadence, stays there while readings
    /// bring something new, goes back to the keepalive cadence after five quiet
    /// readings in a row, and comes back at the next new thing.** The gap
    /// window is measured between refreshes at either cadence, so the quick one
    /// is well inside it. Added 2026-10-25, the eighteenth notes, piece 2b.
    #[test]
    fn y7d_the_refresh_is_quick_while_words_move_and_goes_back_after_five_quiet_readings() {
        let mut cadence = TheRefreshCadence::as_a_run_opens();
        assert_eq!(cadence.until_the_next_reading(), Duration::from_secs(1), "a run opens quick");
        cadence.a_reading_brought(true);
        assert_eq!(cadence.until_the_next_reading(), THE_REFRESH_CADENCE_WHILE_WORDS_MOVE);
        for quiet in 1..HOW_MANY_QUIET_READINGS_END_THE_QUICK_CADENCE {
            cadence.a_reading_brought(false);
            assert_eq!(
                cadence.until_the_next_reading(),
                THE_REFRESH_CADENCE_WHILE_WORDS_MOVE,
                "back to the keepalive cadence after only {quiet} quiet readings"
            );
        }
        cadence.a_reading_brought(false);
        assert_eq!(cadence.until_the_next_reading(), THE_REFRESH_CADENCE, "five quiet readings");
        cadence.a_reading_brought(false);
        assert_eq!(cadence.until_the_next_reading(), THE_REFRESH_CADENCE);
        cadence.a_reading_brought(true);
        assert_eq!(cadence.until_the_next_reading(), THE_REFRESH_CADENCE_WHILE_WORDS_MOVE, "words moved again");
        assert!(!the_window_has_elapsed(THE_REFRESH_CADENCE_WHILE_WORDS_MOVE));
        assert!(!the_window_has_elapsed(THE_REFRESH_CADENCE));
    }

    #[test]
    fn no_signal_is_ever_sent_to_a_setup_worker_or_its_children() {
        // A setup step's children include the package system mid-transaction.
        assert!(!a_signal_is_sent_to_the_worker_or_its_children(true));
    }

    #[test]
    fn the_stops_two_outcomes_turn_on_the_machine_and_not_on_the_user() {
        assert_eq!(
            what_the_stop_does(true),
            WhatTheStopDoes::TheDurableWriteAndAnEndingAtTheNextBoundary
        );
        assert_eq!(
            what_the_stop_does(false),
            WhatTheStopDoes::TheDesktopsWaitingAlone
        );
        assert!(!a_restart_already_begun_can_be_called_back(true));
        assert!(!a_stop_during_the_restart_wait_is_held_for_delivery(true));
    }

    #[test]
    fn stopping_a_machine_changing_run_leaves_the_state_and_the_job_standing() {
        let left = what_a_stop_leaves(RunClass::MachineChanging);
        assert!(left.the_machine_level_state_stands);
        assert!(left.the_interrupted_job_stays_held_back);
    }

    #[test]
    fn stopping_an_additive_run_holds_nothing_back_at_all() {
        let left = what_a_stop_leaves(RunClass::Additive);
        assert!(!left.the_machine_level_state_stands);
        assert!(!left.the_interrupted_job_stays_held_back);
    }

    #[test]
    fn a_second_setup_on_one_machine_is_refused_and_the_open_run_is_named() {
        assert_eq!(
            one_run_at_a_time(true),
            WhetherItIsAdmitted::RefusedNamingTheOpenRun
        );
        assert_eq!(one_run_at_a_time(false), WhetherItIsAdmitted::Admitted);
        const { assert!(SEVERAL_MACHINES_RUN_AT_ONCE) };
        const { assert!(THE_ADD_ON_STORE_LOCK_IS_NOT_SERIALISED_BY_THIS_RULE) };
    }

    #[test]
    fn the_ending_names_the_boundary_it_stopped_at() {
        assert!(ended_at_boundary_not_completed(Boundary::Fetched).contains("Fetched"));
        assert!(ended_at_boundary_not_completed(Boundary::Fetched).contains("not completed"));
    }
}
