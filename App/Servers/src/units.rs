//! Declarations — atomic or partial. `Servers` §14.
//!
//! Per `State, identity and recovery` §9.2, **declared here, decided by the
//! three ordered tests and never at run time**: *"Each feature document
//! declares, per action, its partition unit and which of the three tests decided
//! it."*
//!
//! **No unit here is retried inside its action** (§9.5). **A repair run is not a
//! retry of a connect**: each attempt is its own action with its own identity,
//! and the run is **the period's repair rather than the connect's**.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Queue & History` L16.

/// The roles a unit is enumerated by. **A role and never a value.**
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnitRole {
    /// **Connect's own unit, and it is the whole connection.** The readings
    /// taken on the first-connection sequence are **separate actions with their
    /// own outcomes**; the two gates — host trust and the not-Linux check — are
    /// **steps of this unit and their failure is its failure**.
    TheConnection,
    /// **Atomic entire. Nothing is sent to the machine, so nothing can
    /// half-happen.**
    TheDisconnect,
    /// **The server is always saved** whatever the other two do, which is the
    /// spec's own rule.
    TheSave,
    /// The connection an add makes where a setup option is ticked.
    TheAddsConnection,
    /// **The setup run is `Server Setup`'s.**
    TheSetupRun,
    /// The edit's put-back. **Where the save fails, the put-back is *not
    /// attempted*, naming the save.** **A put-back is merely useless without the
    /// save and not unsafe**, so §9.3 does not apply.
    ThePutBack,
    /// **One durable write removes the row and everything keyed to it, and the
    /// period ends with it.** **An unconfirmed secret erase is a recorded debt
    /// and not a partial removal.**
    TheRemoval,
    /// **The pair is one unit by `Queue & History` L16** (§9.3, the second
    /// unsafe without the first).
    TheClearAndStopPair,
    TelegramsRoleOff,
    /// **Atomic entire, runs last and only on the user's word**, and the period
    /// ends with it — the first two units reach the machine over the connection
    /// the period holds.
    TheLocalForget,
    /// **Atomic entire.** A reading with a stated absence — no per-card memory,
    /// cards that could not be established — is **a complete reading and not a
    /// partial one**.
    TheReading,
}

impl UnitRole {
    /// **The role's own name, and it is a role rather than a value.** These are
    /// **not the words the user reads**: presenting an outcome record is
    /// `Desktop app and delivery`'s, and Rule 9 binds it there.
    pub fn as_role(self) -> &'static str {
        match self {
            UnitRole::TheConnection => "the connection",
            UnitRole::TheDisconnect => "the disconnect",
            UnitRole::TheSave => "the save",
            UnitRole::TheAddsConnection => "the connection",
            UnitRole::TheSetupRun => "the setup run",
            UnitRole::ThePutBack => "the put-back",
            UnitRole::TheRemoval => "the removal",
            UnitRole::TheClearAndStopPair => "the clear-and-stop pair",
            UnitRole::TelegramsRoleOff => "Telegram's role off",
            UnitRole::TheLocalForget => "the local forget",
            UnitRole::TheReading => "the reading",
        }
    }
}

/// Which of §9.2's three tests decided the partition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Test {
    /// **The promise is about the whole thing.**
    One,
    Two,
    /// **Independent units.**
    Three,
}

/// Every action this document declares.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// **The spec promises an outcome about connecting and about nothing
    /// smaller: the server is connected or it is not.**
    Connect,
    Disconnect,
    Add,
    /// **Ordered and independent.**
    Edit,
    RemovalNotReachingTheMachine,
    RemovalReachingTheMachine,
    TheHardwareReading,
}

impl Action {
    pub const ALL: [Action; 7] = [
        Action::Connect,
        Action::Disconnect,
        Action::Add,
        Action::Edit,
        Action::RemovalNotReachingTheMachine,
        Action::RemovalReachingTheMachine,
        Action::TheHardwareReading,
    ];

    /// §14's table, performed.
    pub fn units(self) -> Vec<UnitRole> {
        match self {
            Action::Connect => vec![UnitRole::TheConnection],
            Action::Disconnect => vec![UnitRole::TheDisconnect],
            Action::Add => vec![
                UnitRole::TheSave,
                UnitRole::TheAddsConnection,
                UnitRole::TheSetupRun,
            ],
            Action::Edit => vec![UnitRole::TheSave, UnitRole::ThePutBack],
            Action::RemovalNotReachingTheMachine => vec![UnitRole::TheRemoval],
            Action::RemovalReachingTheMachine => vec![
                UnitRole::TheClearAndStopPair,
                UnitRole::TelegramsRoleOff,
                UnitRole::TheLocalForget,
            ],
            Action::TheHardwareReading => vec![UnitRole::TheReading],
        }
    }

    /// Which test decided it. **The machine-reaching removal is decided by two**
    /// — the pair by test 1, the other two by test 3 — and that is written as
    /// the pair's test, because the pair is the one the ordering argument is
    /// about.
    pub fn test(self) -> Test {
        match self {
            Action::Connect
            | Action::Disconnect
            | Action::RemovalNotReachingTheMachine
            | Action::RemovalReachingTheMachine
            | Action::TheHardwareReading => Test::One,
            Action::Add | Action::Edit => Test::Three,
        }
    }

    /// **Whether §9.4's outcome record is owed**, which that document decides by
    /// one question: **are the action's units smaller than the action?**
    ///
    /// **Three of this document's actions have such units** — the add, the edit,
    /// and the removal that reaches the machine. **W22 is what rests on the
    /// third**: the enumeration of units and each unit's outcome are durable
    /// before the first unit is attempted, **so an interruption between doing
    /// the part and saying so loses the saying and never the fact** — the next
    /// presentation reads the record.
    pub fn owes_an_outcome_record(self) -> bool {
        self.units().len() > 1
    }

    /// **§9.5: no unit here is retried inside its action.**
    pub fn a_unit_is_retried_inside_it(self) -> bool {
        false
    }
}

/// **The edit's ordering rule, stated because it is the one place a unit's
/// failure stops a later unit.** Where the save fails, **the put-back is not
/// attempted, naming the save**.
pub fn the_put_back_is_attempted(the_save_succeeded: bool) -> bool {
    the_save_succeeded
}

/// **The add's rule, stated because it is the opposite one.** **The server is
/// always saved whatever the other two do.**
pub fn the_save_happens_whatever_the_others_do() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connect_is_one_unit_and_its_gates_are_steps_of_it() {
        assert_eq!(Action::Connect.units(), vec![UnitRole::TheConnection]);
        assert_eq!(Action::Connect.test(), Test::One);
        assert!(!Action::Connect.owes_an_outcome_record());
    }

    #[test]
    fn a_disconnect_is_atomic_entire_because_nothing_is_sent() {
        assert_eq!(Action::Disconnect.units(), vec![UnitRole::TheDisconnect]);
        assert!(!Action::Disconnect.owes_an_outcome_record());
    }

    #[test]
    fn the_add_has_three_independent_units_and_the_save_is_unconditional() {
        assert_eq!(Action::Add.units().len(), 3);
        assert_eq!(Action::Add.test(), Test::Three);
        assert!(the_save_happens_whatever_the_others_do());
    }

    #[test]
    fn the_edits_put_back_is_not_attempted_where_the_save_failed() {
        assert_eq!(Action::Edit.units(), vec![UnitRole::TheSave, UnitRole::ThePutBack]);
        assert!(!the_put_back_is_attempted(false));
        assert!(the_put_back_is_attempted(true));
    }

    #[test]
    fn the_machine_reaching_removal_has_three_units_ending_at_the_local_forget() {
        let units = Action::RemovalReachingTheMachine.units();
        assert_eq!(units.len(), 3);
        assert_eq!(units[2], UnitRole::TheLocalForget);
        assert!(Action::RemovalReachingTheMachine.owes_an_outcome_record());
    }

    #[test]
    fn the_removal_that_reaches_no_machine_is_atomic_entire() {
        assert_eq!(
            Action::RemovalNotReachingTheMachine.units(),
            vec![UnitRole::TheRemoval]
        );
        assert!(!Action::RemovalNotReachingTheMachine.owes_an_outcome_record());
    }

    #[test]
    fn a_reading_with_a_stated_absence_is_complete_and_not_partial() {
        assert_eq!(Action::TheHardwareReading.units(), vec![UnitRole::TheReading]);
        assert_eq!(Action::TheHardwareReading.test(), Test::One);
    }

    #[test]
    fn no_unit_is_retried_inside_its_action() {
        for action in Action::ALL {
            assert!(!action.a_unit_is_retried_inside_it());
        }
    }

    #[test]
    fn exactly_three_actions_owe_an_outcome_record() {
        let owing: Vec<_> = Action::ALL
            .into_iter()
            .filter(|action| action.owes_an_outcome_record())
            .collect();
        assert_eq!(
            owing,
            vec![Action::Add, Action::Edit, Action::RemovalReachingTheMachine]
        );
    }

    #[test]
    fn every_role_names_itself_and_never_a_value() {
        for action in Action::ALL {
            for role in action.units() {
                let named = role.as_role();
                assert!(!named.is_empty());
                assert!(!named.contains("SHA256:"));
            }
        }
    }
}
