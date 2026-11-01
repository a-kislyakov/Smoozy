//! The run-mark — `State, identity and recovery` §1.1 I2 and §3 D6a.
//!
//! D6: **no durable state depends on the desktop application shutting down
//! cleanly.** Every durable write happens at the moment the fact becomes true,
//! and nothing is written on exit. **One exemption is named, it falls on the
//! third sentence alone, and it is this file's.**
//!
//! D6a's width is the whole value of the clause: **one field, one reader, no
//! recovery path.** Nothing is written on exit — the *clean* value is written
//! when the user **asks** to quit, which is a fact becoming true. No recovery
//! path differs. **Nothing else may read the mark** — not reconciliation, not
//! a failure record, not a lifetime, not the update handover; a second reader
//! would make a second thing depend on how a run ended, which is what D6
//! exists to forbid. And **a second such field is a fault and not a
//! precedent**.

use crate::identities::Identity;

/// The pair the field holds: a state, and the run identity that wrote it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunMark {
    pub state: RunState,
    pub written_by: Identity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunState {
    /// Written durably at application start, before any feature module
    /// composes.
    Running,
    /// Written durably **at the moment the user asks to quit** — the ask, not
    /// the exit.
    Clean,
}

/// What a launch reads, and it is the field's one reader: `Reporting a
/// problem` §9's launch-time offer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheLaunchFound {
    /// The previous run ended after the user asked to quit.
    ThePreviousRunEndedCleanly,
    /// The previous run did not. The offer is made, against the run identity
    /// the mark names.
    ThePreviousRunDidNot { the_run: Identity },
    /// No previous run at all — a fresh installation, or one of D6a's other
    /// two causes of an absence. **The offer is not made**, which is correct
    /// in each of them: where the whole application-data location was cleared
    /// there is nothing left to cut a report out of, and where the local
    /// record alone went the diary may still hold the lines, so what is lost
    /// is the offer and not the evidence — the menu entry point takes no mark
    /// and works with nothing.
    NoPreviousRun,
}

/// Reads the field at launch. **This is the whole of what reads it.**
pub fn what_the_launch_found(mark: Option<RunMark>) -> WhatTheLaunchFound {
    match mark {
        None => WhatTheLaunchFound::NoPreviousRun,
        Some(RunMark {
            state: RunState::Clean,
            ..
        }) => WhatTheLaunchFound::ThePreviousRunEndedCleanly,
        Some(RunMark {
            state: RunState::Running,
            written_by,
        }) => WhatTheLaunchFound::ThePreviousRunDidNot {
            the_run: written_by,
        },
    }
}

/// The two writes, and there is no third.
///
/// Every write carries the identity of the run performing it, and the field is
/// overwritten durably at every application start before anything else reads
/// it — **so a stale value cannot survive a launch, and a wrong reading can
/// never name the wrong run**. That is what bounds a wrong reading to one
/// prompt in one direction or the other, rather than to a durable state that
/// disagrees with itself.
impl RunMark {
    /// Written durably at application start, after the launch has read what
    /// was there.
    pub fn this_run_is_running(run: Identity) -> Self {
        RunMark {
            state: RunState::Running,
            written_by: run,
        }
    }

    /// Written durably at the moment the user asks to quit.
    pub fn the_user_asked_to_quit(run: Identity) -> Self {
        RunMark {
            state: RunState::Clean,
            written_by: run,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identities::IdentityKind;

    fn a_run() -> Identity {
        Identity::issue(IdentityKind::Run).unwrap()
    }

    #[test]
    fn an_absent_mark_offers_nothing() {
        assert_eq!(
            what_the_launch_found(None),
            WhatTheLaunchFound::NoPreviousRun
        );
    }

    #[test]
    fn a_clean_mark_offers_nothing() {
        assert_eq!(
            what_the_launch_found(Some(RunMark::the_user_asked_to_quit(a_run()))),
            WhatTheLaunchFound::ThePreviousRunEndedCleanly
        );
    }

    #[test]
    fn a_running_mark_names_the_run_the_slice_is_cut_on() {
        let previous = a_run();
        assert_eq!(
            what_the_launch_found(Some(RunMark::this_run_is_running(previous.clone()))),
            WhatTheLaunchFound::ThePreviousRunDidNot { the_run: previous }
        );
    }

    #[test]
    fn every_write_carries_the_run_that_made_it() {
        let run = a_run();
        assert_eq!(RunMark::this_run_is_running(run.clone()).written_by, run);
        assert_eq!(RunMark::the_user_asked_to_quit(run.clone()).written_by, run);
    }
}
