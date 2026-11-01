//! The values two of the readings arrive in — the held hardware reading and
//! the four-value comparison read. **The trait the acting half implements is
//! not here**: it stays in `smoozy-render-settings`' `readings`, because one
//! of its three readings is `Server Setup`'s search result, which a machine
//! half cannot take. That module re-exports these at the paths they always had.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Servers` H1, H2, H11, H12. `Server protocol and
//! transfers` S13. `File Browser` §13.

use smoozy_framing::source_reading::SourceReading;
use smoozy_servers_values::hardware::{HardwareReading, HardwareState};

/// **D5's second half: the hardware reading, which is `Servers`' remembered
/// one** — no Blender in it, so this page's version list owes it nothing —
/// with the state it stands in under `Servers` H1.
///
/// The reading is absent exactly where the state is *never read*, and the
/// type says so rather than leaving the pair to agree by care.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HeldHardware {
    /// `Servers` H1's *never read*: no reading Smoozy may show is held.
    NeverRead,
    /// A reading is held, in H1's *live* or *last seen* state, carrying
    /// H11–H12's card state and H2's moments.
    Held {
        state: HeldState,
        reading: HardwareReading,
    },
}

/// The two states a held reading can be in — `Servers` H1's *live* and *last
/// seen*, keyed on the connection the reading was taken on (`Servers` H1a)
/// and never on the period.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeldState {
    Live,
    LastSeen,
}

impl HeldHardware {
    /// Composed from `Servers`' own two values, so the pairing cannot be
    /// wrong by construction: a *never read* state carries no reading, and a
    /// reading carries a state that is not *never read*.
    pub fn from_servers(state: HardwareState, reading: Option<HardwareReading>) -> HeldHardware {
        match (state, reading) {
            (HardwareState::Live, Some(reading)) => HeldHardware::Held {
                state: HeldState::Live,
                reading,
            },
            (HardwareState::LastSeen, Some(reading)) => HeldHardware::Held {
                state: HeldState::LastSeen,
                reading,
            },
            _ => HeldHardware::NeverRead,
        }
    }
}

/// **Q14 item 2's comparison read**: the four values read fresh, or the
/// channel unable to answer on this rental — the file-transfer surface's
/// revision 3 attribute structure carries no inode and no change time, and
/// where the command channel cannot answer, **the comparison is not run and
/// nothing is claimed** (`File Browser` §13 item 9).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FourValuesNow {
    Answered(SourceReading),
    TheChannelCannotAnswer,
}
