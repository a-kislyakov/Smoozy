//! The store version on each rented machine — §1.2's table, §1.3 rule 7, and
//! §6 step 0a.
//!
//! **Every store in the machine's state area carries one version, held in its
//! own record**, and a newer program upgrades the stores forward, one step at a
//! time, the first time it reads them. The package record and the installation
//! identity are outside it: package inspection reads both across every release,
//! so their shape is the protocol's.
//!
//! **This file is the rule and nothing else.** It opens no file: the record's
//! read and write are the supervisor's (§1.3 rule 1), and the order the upgrade
//! takes against the package transaction — never before *verified* — is
//! `Rented-server runtime` G15a's, performed by that crate and by the program
//! the machine runs.
//!
//! # Why a first version, and why this build keeps it
//!
//! **A state area with no store version record is at the first version**, which
//! is rule 7's last paragraph: every program placed before the rule wrote that
//! shape, and a program that creates the stores writes the record before its
//! first write to any of them. **It is the fourth absence rule 5 reads on
//! purpose**, on the same footing as the other three.
//!
//! **This build keeps the first version and carries no step.** Between release
//! 0.1.6, the last placed before this rule, and this build, one machine store's
//! written form grew: the per-job note gained two optional fields, the reason a
//! render failed and the device line, and a note written without them reads
//! with both absent. A record that reads the same with or without a step needs
//! no step, so there is nothing to upgrade and the version does not move.
//! **The version moves in the same change that adds the first step**, and
//! [`THE_STORE_VERSION_THIS_BUILD_KEEPS`] is where it moves.

/// **One number saying which shape every store in the machine's state area is
/// in** (§1.2's table).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StoreVersion(pub u32);

/// **The version a state area holding no record is at** (§1.3 rule 7).
pub const THE_FIRST_STORE_VERSION: StoreVersion = StoreVersion(1);

/// **The version this build's program writes and reads.** See the file's head
/// for why it is still the first.
pub const THE_STORE_VERSION_THIS_BUILD_KEEPS: StoreVersion = THE_FIRST_STORE_VERSION;

/// **What the stores are at**, read from the record — an absent record being
/// the first version and nothing else.
pub fn the_stores_are_at(recorded: Option<StoreVersion>) -> StoreVersion {
    recorded.unwrap_or(THE_FIRST_STORE_VERSION)
}

/// **The stores against the program reading them**, which is the whole of what
/// §6 step 0a and `Rented-server runtime` G15a branch on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheStoresAgainstThisProgram {
    /// The ordinary case: nothing to upgrade.
    AtItsOwnVersion,
    /// **Older than the program**: upgraded step by step, and only once the
    /// placement is *verified*.
    Older { at: StoreVersion, own: StoreVersion },
    /// **Newer than the program**: it opens none of them and says so (§1.3
    /// rule 7's fourth bullet), answering only the health exchange.
    Newer { at: StoreVersion, own: StoreVersion },
}

/// The stores the record says, against the program's own version.
pub fn the_stores_against(
    recorded: Option<StoreVersion>,
    own: StoreVersion,
) -> TheStoresAgainstThisProgram {
    let at = the_stores_are_at(recorded);
    match at.cmp(&own) {
        std::cmp::Ordering::Equal => TheStoresAgainstThisProgram::AtItsOwnVersion,
        std::cmp::Ordering::Less => TheStoresAgainstThisProgram::Older { at, own },
        std::cmp::Ordering::Greater => TheStoresAgainstThisProgram::Newer { at, own },
    }
}

/// **One step of the upgrade: the stores from [`AStoreStep::from`] to the
/// version after it.**
///
/// **It changes a record's shape and never its meaning, and never removes** a
/// job record, a hold, a history copy, a per-job note or a result not yet
/// acknowledged. **It is safe to run again over its own partly finished
/// result**, because the version moves only after the step is durable, so an
/// interrupted upgrade runs the same step again.
pub trait AStoreStep {
    /// The version this step takes the stores from.
    fn from(&self) -> StoreVersion;
    /// The step, performed durably.
    fn run(&self) -> Result<(), String>;
}

/// The store version record, read and written by the one writer.
pub trait KeepsTheStoreVersion {
    /// The record, or nothing where none was ever written.
    fn read(&self) -> Result<Option<StoreVersion>, String>;
    /// The record, written durably (§1.3 rule 6).
    fn write(&self, version: StoreVersion) -> Result<(), String>;
}

/// **What an upgrade came to.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheUpgradeDid {
    /// Each step run, by the version it started from, in order.
    pub steps_run: Vec<StoreVersion>,
    /// Where the stores are now.
    pub now_at: StoreVersion,
}

/// **§1.3 rule 7's upgrade: forward only, one step at a time, in order**, the
/// version written durably after each step and never before it.
///
/// **It refuses rather than guesses** where the record is newer than the
/// program, and where a version between the two has no step: a program that
/// cannot take the stores the whole way leaves them where they are.
pub fn upgrade_step_by_step(
    keeps: &dyn KeepsTheStoreVersion,
    own: StoreVersion,
    steps: &[&dyn AStoreStep],
) -> Result<WhatTheUpgradeDid, String> {
    let mut at = the_stores_are_at(keeps.read()?);
    if at > own {
        return Err(format!(
            "the stores are at version {} and this program keeps version {}, so it opens none \
             of them",
            at.0, own.0
        ));
    }
    let mut steps_run = Vec::new();
    while at < own {
        let step = steps
            .iter()
            .find(|step| step.from() == at)
            .ok_or_else(|| {
                format!(
                    "the stores are at version {} and this program carries no step from it, so \
                     they are left there",
                    at.0
                )
            })?;
        step.run()
            .map_err(|why| format!("the step from store version {} did not finish: {why}", at.0))?;
        let next = StoreVersion(at.0 + 1);
        keeps.write(next)?;
        steps_run.push(at);
        at = next;
    }
    Ok(WhatTheUpgradeDid {
        steps_run,
        now_at: at,
    })
}

#[cfg(test)]
mod tests {
    use std::cell::{Cell, RefCell};

    use super::*;

    struct ARecord(RefCell<Option<StoreVersion>>);

    impl KeepsTheStoreVersion for ARecord {
        fn read(&self) -> Result<Option<StoreVersion>, String> {
            Ok(*self.0.borrow())
        }
        fn write(&self, version: StoreVersion) -> Result<(), String> {
            *self.0.borrow_mut() = Some(version);
            Ok(())
        }
    }

    /// A step that fails the first `fails_first` times it is run.
    struct AStep {
        from: u32,
        runs: Cell<u32>,
        fails_first: u32,
    }

    impl AStoreStep for AStep {
        fn from(&self) -> StoreVersion {
            StoreVersion(self.from)
        }
        fn run(&self) -> Result<(), String> {
            self.runs.set(self.runs.get() + 1);
            if self.runs.get() <= self.fails_first {
                Err("the machine went down mid-step".into())
            } else {
                Ok(())
            }
        }
    }

    fn a_step(from: u32, fails_first: u32) -> AStep {
        AStep {
            from,
            runs: Cell::new(0),
            fails_first,
        }
    }

    #[test]
    fn an_absent_record_is_the_first_version_and_nothing_else() {
        assert_eq!(the_stores_are_at(None), THE_FIRST_STORE_VERSION);
        assert_eq!(
            the_stores_against(None, THE_FIRST_STORE_VERSION),
            TheStoresAgainstThisProgram::AtItsOwnVersion
        );
        assert_eq!(
            the_stores_against(None, StoreVersion(2)),
            TheStoresAgainstThisProgram::Older {
                at: StoreVersion(1),
                own: StoreVersion(2)
            }
        );
        assert_eq!(
            the_stores_against(Some(StoreVersion(3)), StoreVersion(2)),
            TheStoresAgainstThisProgram::Newer {
                at: StoreVersion(3),
                own: StoreVersion(2)
            }
        );
    }

    #[test]
    fn the_stores_go_forward_one_step_at_a_time_in_order() {
        let record = ARecord(RefCell::new(None));
        let (two, one) = (a_step(2, 0), a_step(1, 0));
        let did = upgrade_step_by_step(&record, StoreVersion(3), &[&two, &one])
            .expect("the upgrade runs");
        assert_eq!(did.steps_run, vec![StoreVersion(1), StoreVersion(2)]);
        assert_eq!(did.now_at, StoreVersion(3));
        assert_eq!(*record.0.borrow(), Some(StoreVersion(3)));
        assert_eq!((one.runs.get(), two.runs.get()), (1, 1));
    }

    /// **An interrupted step leaves the version where the last durable step
    /// put it, and the next upgrade runs that step again.**
    #[test]
    fn an_interrupted_step_leaves_the_version_and_runs_again_cleanly() {
        let record = ARecord(RefCell::new(Some(StoreVersion(1))));
        let (one, two) = (a_step(1, 0), a_step(2, 1));
        let first = upgrade_step_by_step(&record, StoreVersion(3), &[&one, &two]);
        assert!(first.is_err());
        assert_eq!(*record.0.borrow(), Some(StoreVersion(2)));
        let second = upgrade_step_by_step(&record, StoreVersion(3), &[&one, &two])
            .expect("the step runs again");
        assert_eq!(second.steps_run, vec![StoreVersion(2)]);
        assert_eq!(*record.0.borrow(), Some(StoreVersion(3)));
        assert_eq!((one.runs.get(), two.runs.get()), (1, 2));
    }

    #[test]
    fn stores_newer_than_the_program_are_left_alone() {
        let record = ARecord(RefCell::new(Some(StoreVersion(4))));
        let one = a_step(1, 0);
        assert!(upgrade_step_by_step(&record, StoreVersion(2), &[&one]).is_err());
        assert_eq!(*record.0.borrow(), Some(StoreVersion(4)));
        assert_eq!(one.runs.get(), 0);
    }

    #[test]
    fn a_gap_with_no_step_is_refused_rather_than_skipped() {
        let record = ARecord(RefCell::new(None));
        let two = a_step(2, 0);
        assert!(upgrade_step_by_step(&record, StoreVersion(3), &[&two]).is_err());
        assert_eq!(*record.0.borrow(), None);
        assert_eq!(two.runs.get(), 0);
    }

    #[test]
    fn this_build_keeps_the_first_version_and_an_equal_record_runs_nothing() {
        assert_eq!(THE_STORE_VERSION_THIS_BUILD_KEEPS, THE_FIRST_STORE_VERSION);
        let record = ARecord(RefCell::new(None));
        let did = upgrade_step_by_step(&record, THE_STORE_VERSION_THIS_BUILD_KEEPS, &[])
            .expect("nothing to do");
        assert!(did.steps_run.is_empty());
        assert_eq!(*record.0.borrow(), None);
    }
}
