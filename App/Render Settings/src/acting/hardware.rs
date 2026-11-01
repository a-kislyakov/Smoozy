//! **D5's second half, taken** — `Render Settings` §8.2 D5, D6.
//!
//! **This reading reaches no machine, and that is the clause rather than a
//! convenience.** D5: *"the hardware reading, which is `Servers`' remembered
//! one — no Blender in it, so this page's version list owes it nothing."* So
//! the real end of [`crate::readings::Readings::the_hardware_reading`] is a
//! read of the server row, and the whole of the work is composing the row's
//! held reading with the state `Servers` H1 puts it in.
//!
//! **Nothing here re-takes a reading, and nothing here decides a state.**
//! `Servers` H4 makes a fresh reading replace the row's whole, and `Servers`
//! H1's table is that document's own function, called rather than performed
//! again — a second copy of a closed set is a set that drifts.
//!
//! **D6's two axes come together here and nowhere else in this crate**: the
//! card state the reading carries (`Servers` H11, H12) and the reading's own
//! state (`Servers` H1). [`crate::device::the_warning`] is the judgement over
//! the pair and is untouched by this file.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Servers` H1, H1a, H4, H11, H12, H15, H18, H21.
//!
//! **`State, identity and recovery` Y10 is deliberately not on that list and
//! is written out at both of its sites**, because this plan defines a Y10 of
//! its own and a bare one would resolve to a real and wrong clause.

use smoozy_servers::hardware::{hardware_state, WhichConnectionHoldsIt};
use smoozy_servers::store::ServerRow;
use smoozy_state::identities::InstallationCheck;

use crate::readings::HeldHardware;

/// **D5, performed: the remembered reading with the state it stands in.**
///
/// **The state is keyed on the connection the held reading was taken on,
/// never on the period** (`Servers` H1a), which is what D6's own correction of
/// 2026-08-22 turns on: a reading held over a dead connection inside a
/// standing period stood in the *live* column under the superseded keying, and
/// this warning then showed a destroyed machine's cards unqualified. **So the
/// caller hands in whether the held reading was taken on the connection that
/// is live now, and not whether a period stands** — `Servers` H18's fourth
/// branch is the case that separates the two (`Servers` H15, H21).
///
/// **The installation check is carried rather than assumed** (`State,
/// identity and recovery` Y10): a machine-derived value read under an
/// installation identity that is now absent or different is disqualified, and
/// `Servers` H1's own table returns the row to *never read* for it. Nothing
/// here decides that; it is passed straight through.
pub fn the_hardware_reading(
    row: &ServerRow,
    taken_on_the_connection_that_is_live_now: bool,
    check: InstallationCheck,
) -> HeldHardware {
    let held = row.remembered_hardware_reading.as_ref();
    let state = hardware_state(
        held,
        WhichConnectionHoldsIt {
            taken_on_the_connection_that_is_live_now,
        },
        check,
    );
    HeldHardware::from_servers(state, held.cloned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::readings::HeldState;
    use smoozy_servers::hardware::{Cards, HardwareReading, MachineMemory};
    use smoozy_servers::identity::{Host, KeyReference, Port, Username};
    use std::time::{Duration, SystemTime};

    fn a_reading() -> HardwareReading {
        HardwareReading::taken(
            Cards::CouldNotBeEstablished,
            MachineMemory::CouldNotBeEstablished,
            None,
            SystemTime::UNIX_EPOCH + Duration::from_secs(1_756_800_000),
            "row-1",
            SystemTime::UNIX_EPOCH + Duration::from_secs(1_756_900_000),
            None,
        )
    }

    fn a_row(held: Option<HardwareReading>) -> ServerRow {
        let mut row = ServerRow::saved(
            "row-1",
            Host::what_the_user_typed("machine.example").expect("a host"),
            Port::what_the_user_typed(22).expect("a port"),
            Username::what_the_user_typed("smoozy").expect("a username"),
            KeyReference {
                local_key_identifier: "key-1".to_string(),
                fingerprint: "SHA256:aaaa".to_string(),
            },
        );
        row.remembered_hardware_reading = held;
        row
    }

    #[test]
    fn a_row_with_no_remembered_reading_is_never_read_and_carries_no_reading() {
        assert_eq!(
            the_hardware_reading(
                &a_row(None),
                true,
                InstallationCheck::PresentAndEqualConfirmed
            ),
            HeldHardware::NeverRead
        );
    }

    #[test]
    fn a_reading_taken_on_the_connection_that_is_live_now_is_live_and_any_other_is_last_seen() {
        // **D6's own correction of 2026-08-22, driven from the reading side.**
        // Keyed on the period rather than the connection, the second case here
        // reads *live* and the warning states a destroyed machine's cards in
        // the present tense.
        let row = a_row(Some(a_reading()));
        let live = the_hardware_reading(&row, true, InstallationCheck::PresentAndEqualConfirmed);
        assert!(matches!(
            live,
            HeldHardware::Held {
                state: HeldState::Live,
                ..
            }
        ));
        let last_seen =
            the_hardware_reading(&row, false, InstallationCheck::PresentAndEqualConfirmed);
        assert!(matches!(
            last_seen,
            HeldHardware::Held {
                state: HeldState::LastSeen,
                ..
            }
        ));
    }

    #[test]
    fn a_disqualified_installation_returns_the_row_to_never_read_however_live_the_connection() {
        // `Servers` H18's two discarding branches, which are `State, identity
        // and recovery` Y10's rule and are not re-decided here.
        for check in [
            InstallationCheck::Absent,
            InstallationCheck::PresentAndDifferent,
        ] {
            assert_eq!(
                the_hardware_reading(&a_row(Some(a_reading())), true, check),
                HeldHardware::NeverRead,
                "{check:?}"
            );
        }
        // And the one branch that leaves it standing.
        assert!(matches!(
            the_hardware_reading(
                &a_row(Some(a_reading())),
                true,
                InstallationCheck::PresentAndTheSupervisorDidNotAnswer
            ),
            HeldHardware::Held { .. }
        ));
    }

    #[test]
    fn the_reading_handed_on_is_the_rows_own_and_is_never_composed_here() {
        let row = a_row(Some(a_reading()));
        let HeldHardware::Held { reading, .. } =
            the_hardware_reading(&row, true, InstallationCheck::PresentAndEqualConfirmed)
        else {
            panic!("a reading is held");
        };
        assert_eq!(reading, a_reading());
    }
}
