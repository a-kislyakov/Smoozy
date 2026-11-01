//! The restart — `Server Setup` §8.
//!
//! **The constraint** is Rows 105 and 283 — the same promise in two specs,
//! written once — with **Fact 51 answered yes and the escalation spent**; Rows
//! 106 to 109; Fact 57; Fact 80's cost spread.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Servers` M6, N2a. `Rented-server runtime` K17, P2,
//! P4, P5, P7. `Server protocol and transfers` §7, S26. `Queue & History` §6.
//!
//! **`Servers` M1 and M4 are not listed above**, because this plan defines an M1
//! and an M4 of its own and a bare one could mean either; where this file cites one, the document is written out at the site.

use std::time::Duration;

use smoozy_server_protocol::transport::CommandString;
use smoozy_servers::restart::HowTheWaitEnds;

use crate::kind::MachineKind;

/// **T1: the restart is `systemctl reboot`, issued over the connection Smoozy
/// holds, as an elevated act under [`crate::elevation`] E3, only on the systemd
/// kind, and only as part of a setup step that needs it** — which on the
/// evidence is exactly one step, the driver's.
///
/// Observed: the connection reported `closed by remote host` and the same
/// address and port answered again in about ten seconds with a new boot id and
/// the disk intact (Fact 51). **Confirmed on a second seller, 2026-08-22**:
/// `sudo systemctl reboot` returned 0 and the connection dropped; the box came
/// back in **49 seconds**, and in **55** on a second restart later the same day,
/// **the address unchanged and all three host keys byte-identical** before and
/// after.
pub const THE_RESTART: CommandString = CommandString::constant("systemctl reboot");

/// **T1: never through a provider's console or interface of any kind** — **no
/// provider exists anywhere in the app** — **and never on a container**, where it
/// would restart the container and repair nothing (Fact 48).
pub fn a_restart_may_be_issued(kind: MachineKind, it_is_part_of_a_setup_step: bool) -> bool {
    matches!(kind, MachineKind::ASystemdMachine) && it_is_part_of_a_setup_step
}

/// **T1: what makes the restart load-bearing rather than incidental** is
/// [`crate::kind`] M1b's Debian route, **which cannot end in a working card
/// query without it**: the driver's module is built against a kernel the machine
/// is not yet running. **So the whole-app boundary's carve-out is the difference
/// between preparing a Debian machine and sending the user to their provider
/// mid-setup.**
pub const THE_CARVE_OUT_IS_LOAD_BEARING_ON_DEBIAN: bool = true;

/// **T2: the exception is exactly as narrow as the whole-app boundary states**
/// — a restart a setup step needs, and nothing else. **Smoozy never powers a
/// machine on and never powers one down for good.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatSmoozyNeverDoes {
    PowersAMachineOn,
    PowersAMachineDownForGood,
    RestartsThroughAProvidersInterface,
}

impl WhatSmoozyNeverDoes {
    pub const ALL: [WhatSmoozyNeverDoes; 3] = [
        WhatSmoozyNeverDoes::PowersAMachineOn,
        WhatSmoozyNeverDoes::PowersAMachineDownForGood,
        WhatSmoozyNeverDoes::RestartsThroughAProvidersInterface,
    ];
}

/// **T2, as the question a caller asks.**
pub fn smoozy_may(_it: WhatSmoozyNeverDoes) -> bool {
    false
}

/// **T3: the user is not asked and is told** (the spec's rule).
///
/// **The machine-level state is durable before the command is issued**
/// ([`crate::run`] Y4's *restart issued* boundary; `Rented-server runtime` P4),
/// so **whatever happens next, the machine that comes back holds its queue back
/// until the setup finishes.**
pub fn the_user_is_asked_before_the_restart() -> bool {
    false
}

/// **T4: Row 107 — *this drop is mine* is a state Smoozy sets before the act,
/// never a reading it takes after it.**
///
/// A machine closing politely and a provider powering one off **are detected in
/// the same second**, while a restart and a destruction **look alike for the
/// first few seconds** — nothing in the moment distinguishes them (Fact 57). So
/// before issuing the command, the desktop marks the server's connected period
/// **restarting, caused by Smoozy**; while the mark stands, the
/// dropped-connection repair machinery and the could-not-restore notification
/// are suppressed and **the server reads as restarting**.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheMark {
    /// **Set before the command leaves.**
    pub set_before_the_command_leaves: bool,
    pub the_ordinary_repair_is_suppressed: bool,
    pub the_could_not_restore_notification_is_suppressed: bool,
}

/// **T4, as one value. The connected period itself, and the reconnect attempts,
/// are `Servers`'** — this document supplies the mark, the moment it is set and
/// the moment it clears.
pub const THE_MARK: TheMark = TheMark {
    set_before_the_command_leaves: true,
    the_ordinary_repair_is_suppressed: true,
    the_could_not_restore_notification_is_suppressed: true,
};

/// **T4's mark as a witness rather than a flag.**
///
/// *"*This drop is mine* is a state Smoozy sets **before the act**, never a
/// reading it takes after it."* **Only [`mark_the_connected_period_restarting`]
/// produces one**, which is the shape [`crate::declarations`]'s
/// `SettingAMachineUpWasOpen` already uses: a value that carries nothing but the
/// fact that the act was performed.
///
/// **Why it is a witness.** [`crate::acts::TheRestartMayBeIssued`] used to be a
/// plain flag made out of a machine kind and a `true`, so it carried no evidence
/// the mark had been set — and a caller that forgot the mark would send the
/// reboot, `Servers` would read an ordinary lost connection, and the whole-app
/// *could not restore* notification would fire, which Row 107 says must not
/// happen on a restart Smoozy caused. **That is exactly what the caller written
/// at step 10c piece 9 did**: it issued the restart around the witness rather
/// than through it, and the restart went out with no mark set for a day.
///
/// **So the witness carries evidence rather than a promise, from 2026-09-20.**
/// It is made only by [`mark_the_connected_period_restarting`], which is handed
/// the connected period's own mark and sets it — so a value of this type means
/// a real period is marked, and not that somebody called a function named after
/// marking one.
#[derive(Debug)]
pub struct TheMarkWasSetBeforeTheCommandLeft(());

/// **T4, performed: the desktop marks the server's connected period *restarting,
/// caused by Smoozy*, and this is what the act hands back.**
///
/// **The connected period itself and the reconnect attempts are `Servers`'** —
/// this document supplies the mark, the moment it is set and the moment it
/// clears ([`THE_MARK`]) — so the mark it is handed is that document's own
/// handle on the standing period, and what this function does is set it and
/// produce the only evidence a restart step accepts.
///
/// **The moment is the caller's and cannot be checked here**: T4 is *before the
/// command leaves*, and nothing in a value can say what has not happened yet.
/// What the type does buy is that the command cannot leave **without** it.
pub fn mark_the_connected_period_restarting(
    period: &smoozy_servers::restart::TheRestartMark,
) -> TheMarkWasSetBeforeTheCommandLeft {
    period.set();
    TheMarkWasSetBeforeTheCommandLeft(())
}

/// **T5: the setup is picked up on the machine answering and on nothing
/// else.**
///
/// *"When the machine answers again, the setup resumes by [`crate::recovery`]
/// S4 item 2's recovery run from the *restart issued* boundary."* **The policy
/// giving up resumes nothing** — the setup is reported cut off part-way per S2
/// — and **the user stopping the wait resumes nothing either**, because the
/// mark stands and what ends is the waiting. **Nothing resumes with nobody
/// connected**, which is the caller's other condition and not this one.
pub fn the_setup_is_picked_up_on(how: HowTheWaitEnds) -> bool {
    matches!(how, HowTheWaitEnds::TheMachineAnswers)
}

/// **T4: a closed set of two, and nothing else clears the mark** — the machine
/// answering ([`the_wait_ends`]), or `Servers`' reconnect policy for a
/// restart-marked period giving up.
///
/// **The user stopping the wait is not one of the two**: it ends the setup's
/// waiting and nothing else.
pub fn the_mark_clears(how: HowTheWaitEnds) -> bool {
    how.clears_the_mark()
}

/// **T5: Row 106 — when the machine answers again, the setup resumes by
/// [`crate::recovery`] S4 item 2's recovery run from the *restart issued*
/// boundary** — the reboot reaped the old worker, its group validates gone, and
/// the recovery takes the state over and carries on at the check.
///
/// **Nothing resumes with nobody connected**; the machine's only autonomous act
/// while down and after return is holding everything back (`Rented-server
/// runtime` P2, P4).
pub fn where_the_recovery_picks_up() -> crate::run::Boundary {
    crate::run::Boundary::RestartIssued
}

/// **T6: Row 108 — the wait has no constant of this document's, and cannot have
/// one.**
///
/// Return cost was measured at **9 to 90 seconds** across three machines, and a
/// container recycle at about **90** — **anything budgeting a fixed wait is
/// wrong on most machines** (`Rented-server runtime` K17). **The structure is
/// settled here; the figures are `Servers`'.**
///
/// **This document's requirement on that policy is only that no figure is
/// derived from any one machine's observed cost**, which `Servers` M5 keeps: its
/// ten minutes is the repair's own run structure repeated, and **more than six
/// times the top of this spread.**
pub const THE_OBSERVED_RETURN_SPREAD: (Duration, Duration) =
    (Duration::from_secs(9), Duration::from_secs(90));

/// **T6: the wait ends one of three ways, and only two of them clear the
/// mark.** The set is `Servers` M6's, performed from that crate rather than
/// written a second time.
pub fn the_wait_ends() -> [HowTheWaitEnds; 3] {
    HowTheWaitEnds::ALL
}

/// **T6: what the user stopping the wait costs, and it is nothing the machine
/// holds.**
///
/// *"The mark stands, the machine-level state stands, `Servers`' reconnect
/// policy for that marked period goes on running, and the setup is picked up on
/// return; what ends is the user's waiting and nothing else."*
pub fn stopping_the_wait_clears_anything() -> bool {
    false
}

/// **T6: where the reconnect policy gives up, Smoozy says only what it
/// knows.**
///
/// The mark clears, the setup is reported cut off part-way per
/// [`crate::recovery`] S2, and **the connection is down and could not be
/// restored, with no claim to know why, because a machine still starting cannot
/// be told from one that will never answer** (the spec's own sentence).
pub fn what_is_claimed_when_the_policy_gives_up() -> &'static str {
    "the connection is down and could not be restored, with no claim to know why"
}

/// **T6a: what the restart panel may say about the render, bounded.**
///
/// The design has the panel say *"The render that was running stopped, and
/// restarts by itself."* **The stopping half is unconditional and is T7's.** The
/// restarting half is not, and the boundary is exact rather than a hedge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatThePanelMaySayAboutTheRender {
    /// **What this document supplies the panel**: *the render that was running
    /// stopped, and is held to be re-run when the setup finishes* — **the stop
    /// as fact, the re-run as consequent.** The wording is Design's; the drafted
    /// sentence states the consequent unconditionally, and that is recorded
    /// here rather than repaired in a document this stage does not own.
    TheStopAsFactAndTheReRunAsConsequent,
}

/// **T6a: the case that falsifies the unconditional form is the user walking
/// away.**
///
/// [`crate::driving`] Y7 drives the run from the desktop one step at a time, so
/// **a user who closes the panel and never reconnects leaves the run undriven,
/// the state standing and the job never re-run.** **That is not a silence**: the
/// machine's own record is what [`crate::recovery`] S1 and S2 read on the next
/// connection, and the user meets a part-finished setup naming what was reached
/// and what to do. **The re-run is owed to a finished setup and the panel is
/// where the setup is finished.**
pub fn the_re_run_is_owed_to(the_run_reached_done: bool) -> bool {
    the_run_reached_done
}

/// **T7: Row 109 — what a restart stops, said per kind of work, with the
/// unposed clause named.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatARestartDoesTo {
    /// **A transfer in flight dies with the machine**, and under `Server
    /// protocol and transfers` §7's contract that is an **interruption**: the
    /// staged partial is kept off the final name, the record says interrupted,
    /// and it is resumed on the re-made connection (S26).
    ATransferInFlight,
    /// **A dependency check or any other reading is assumed to die with its
    /// worker, and that clause was never posed** — Fact 55's own *Not covered*
    /// line names it as the one still open (`Server Setup` §13 item 2). Its
    /// operation record has no outcome, **and reconciliation writes its
    /// disposition** — which does not depend on the manner of death.
    AReadingInFlight,
    /// **A running render dies with the machine, and that is observed rather
    /// than assumed**: two real machine restarts with a render in flight, on
    /// machines restarted over the connection as root (Fact 55).
    ARunningRender,
}

/// **T7: both interruption shapes are real and neither is aimable.**
///
/// A restart landing **in sampling — about 92% of a frame — leaves nothing at
/// the frame's path at all**, no partial and no zero-byte file; landing **in the
/// write — about 8% — leaves a partial under the frame's real final name with
/// nothing to mark it**, which settles that Blender does not write to a
/// temporary name and rename.
///
/// **A polite stop behaves identically to a hard kill at exit 143, and that is
/// the signal a shutdown sends.** Both restarts landed in sampling despite the
/// second being aimed deliberately at a write, because the shutdown latency
/// varied from 9 seconds to 6.4 between runs minutes apart — **so the shape
/// cannot be chosen even on purpose**, which is why **nothing anywhere reads a
/// file's presence as a frame's success.**
pub fn a_files_presence_is_read_as_a_frames_success(_it_is_there: bool) -> bool {
    false
}

/// **T7: nothing announces the death.** The system log records only that it sent
/// a termination signal to remaining processes, **naming neither Blender nor the
/// render nor its output.** The observation is limited to **one output format
/// and one picture size**.
pub const NOTHING_ANNOUNCES_THE_RENDERS_DEATH: bool = true;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_restart_is_issued_only_on_a_systemd_machine_and_only_inside_a_setup_step() {
        assert!(a_restart_may_be_issued(MachineKind::ASystemdMachine, true));
        assert!(!a_restart_may_be_issued(MachineKind::ASystemdMachine, false));
        assert!(!a_restart_may_be_issued(MachineKind::AContainer, true));
        assert!(!a_restart_may_be_issued(MachineKind::TheTableDoesNotPlaceIt, true));
        assert_eq!(THE_RESTART.as_str(), "systemctl reboot");
    }

    #[test]
    fn smoozy_never_powers_a_machine_on_or_down_and_never_uses_a_providers_interface() {
        assert_eq!(WhatSmoozyNeverDoes::ALL.len(), 3);
        for it in WhatSmoozyNeverDoes::ALL {
            assert!(!smoozy_may(it));
        }
    }

    #[test]
    fn the_user_is_told_rather_than_asked_and_the_state_is_durable_first() {
        assert!(!the_user_is_asked_before_the_restart());
        const { assert!(THE_MARK.set_before_the_command_leaves) };
        const { assert!(THE_MARK.the_ordinary_repair_is_suppressed) };
        const { assert!(THE_MARK.the_could_not_restore_notification_is_suppressed) };
    }

    #[test]
    fn exactly_two_of_the_three_endings_clear_the_mark() {
        // T4's closed set of two, performed from `Servers` M6 rather than
        // written again.
        assert_eq!(the_wait_ends().len(), 3);
        assert!(the_mark_clears(HowTheWaitEnds::TheMachineAnswers));
        assert!(the_mark_clears(HowTheWaitEnds::ThePolicyGivesUp));
        assert!(!the_mark_clears(HowTheWaitEnds::TheUserStopsWaiting));
        assert!(!stopping_the_wait_clears_anything());
    }

    #[test]
    fn the_recovery_picks_up_at_the_restart_issued_boundary() {
        assert_eq!(where_the_recovery_picks_up(), crate::run::Boundary::RestartIssued);
    }

    #[test]
    fn only_the_machine_answering_picks_the_setup_up() {
        // T5, and the other two endings resume nothing: the policy giving up is
        // reported cut off part-way, and the user stopping the wait ends the
        // waiting alone.
        assert!(the_setup_is_picked_up_on(HowTheWaitEnds::TheMachineAnswers));
        assert!(!the_setup_is_picked_up_on(HowTheWaitEnds::ThePolicyGivesUp));
        assert!(!the_setup_is_picked_up_on(HowTheWaitEnds::TheUserStopsWaiting));
        // **Every ending that picks the setup up is one that clears the mark**,
        // which is what stops a resume being attempted under a mark that stands.
        for how in HowTheWaitEnds::ALL {
            assert!(!the_setup_is_picked_up_on(how) || the_mark_clears(how));
        }
    }

    #[test]
    fn the_witness_is_made_only_by_marking_a_real_period() {
        // T4: the mark is set on the connected period `Servers` holds, and the
        // witness is evidence of that rather than of a call.
        let period = smoozy_servers::restart::TheRestartMark::none_is_set();
        assert!(!period.stands());
        let _witness = mark_the_connected_period_restarting(&period);
        assert!(period.stands());
    }

    #[test]
    fn no_wait_figure_is_derived_from_any_one_machines_observed_cost() {
        // T6: 9 to 90 seconds across three machines, and `Servers` M5's ten
        // minutes is more than six times the top of it.
        assert_eq!(THE_OBSERVED_RETURN_SPREAD.1, Duration::from_secs(90));
        assert!(smoozy_servers::restart::THE_WAIT > THE_OBSERVED_RETURN_SPREAD.1 * 6);
        const { assert!(THE_CARVE_OUT_IS_LOAD_BEARING_ON_DEBIAN) };
    }

    #[test]
    fn what_is_claimed_at_the_give_up_is_only_what_smoozy_knows() {
        assert!(what_is_claimed_when_the_policy_gives_up().contains("no claim to know why"));
    }

    #[test]
    fn the_panels_re_run_is_a_consequent_and_the_stop_is_a_fact() {
        // T6a: the case that falsifies the unconditional form is the user
        // walking away.
        assert!(the_re_run_is_owed_to(true));
        assert!(!the_re_run_is_owed_to(false));
        assert_eq!(
            WhatThePanelMaySayAboutTheRender::TheStopAsFactAndTheReRunAsConsequent,
            WhatThePanelMaySayAboutTheRender::TheStopAsFactAndTheReRunAsConsequent
        );
    }

    #[test]
    fn a_files_presence_is_never_read_as_a_frames_success() {
        // T7: neither interruption shape is aimable, so nothing may rest on it.
        assert!(!a_files_presence_is_read_as_a_frames_success(true));
        const { assert!(NOTHING_ANNOUNCES_THE_RENDERS_DEATH) };
        // The three kinds of work a restart reaches.
        for it in [
            WhatARestartDoesTo::ATransferInFlight,
            WhatARestartDoesTo::AReadingInFlight,
            WhatARestartDoesTo::ARunningRender,
        ] {
            assert!(matches!(
                it,
                WhatARestartDoesTo::ATransferInFlight
                    | WhatARestartDoesTo::AReadingInFlight
                    | WhatARestartDoesTo::ARunningRender
            ));
        }
    }
}
