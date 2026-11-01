//! The half that acts: the real end of every reading this document's
//! judgements stand on, and the driving of the one act it has.
//!
//! # What this is
//!
//! The rest of this crate is judgement — a rule, a table, a total function
//! over a reading somebody else took. **Nothing there opens a socket, starts a
//! process or runs a command.** This module is where all of that happens, and
//! it is the only place in the crate that does. It is the shape `App/Servers/`
//! settled at its own `readings` and `acting` pair and `App/Server Setup/`
//! carried, taken here unchanged.
//!
//! # The three readings, and why the trait's methods take no machine
//!
//! [`crate::readings::Readings`] is synchronous, so a real end cannot reach a
//! machine from inside it. **The acting half takes each answer at the moment
//! the page needs it and [`TheMachineThisPageIsOn`] carries the answers into
//! the judgement** — the shape `Server Setup`'s own `TheMachineAtARefusal`
//! already uses, and for the same reason.
//!
//! - **D5's hardware reading** is [`hardware`], and it reaches no machine at
//!   all: it is `Servers`' **remembered** one, read off the server row.
//! - **B1's version list** is [`versions`], and it is `Server Setup`'s own
//!   search driven at the moment `Server Setup` B12 names. Nothing here
//!   re-runs, caches durably or supplements it.
//! - **Q14 item 2's four-value read** is [`four_values`], over the command
//!   channel with the `.blend`'s path riding the planted-link carriage.
//!
//! # The one act
//!
//! [`pre_load`] drives it: B5a's rule that no read runs while no version
//! resolves, Q5's one read per (server, file) with a second Pre-load joining
//! the running one, Q17's carrying of what the read was started under, and
//! Q8's judgement over the frame the invocation left.
//!
//! # What is not here, and each is a clause rather than an omission
//!
//! **The invocation itself is not this crate's.** `Job files and outputs` R1
//! owns it — one invocation of the Blender the manifest names, headless, one
//! JSON document between sentinels — and **that document's own §4 is built, at
//! `smoozy_job_files::reading`**, which is what the invocation names and what
//! the one document holds. *(Corrected 2026-09-07: this said §4 was recorded as
//! arriving at step 7 in that crate's `arriving_later`. Step 7 landed it and
//! that page has been empty since.)* **What is still not built is the
//! performing** — nothing in this workspace starts a Blender for it — **so the
//! invocation is a seam here** ([`pre_load::DrivesOneFileInspection`]), exactly
//! as `Servers` makes
//! what opens a connection a seam, and admission at the machine's one
//! admission point is `Rented-server runtime` A4's on the far side of it.
//!
//! **The reader that composes the document is not written here either**, and
//! [`pre_load`] says what it must emit rather than emitting it.
//!
//! **The pin pairs, the device argument and the version confirmation are
//! `Rendering`'s**, composed in that document's launch material and already
//! built there. [`crate::arriving_later`] carries the list.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Job files and outputs` R1. `Rented-server
//! runtime` A4. `Servers` H1.

pub mod four_values;
pub mod hardware;
pub mod pre_load;
pub mod versions;

use smoozy_server_setup::search::TheSearchResult;
use smoozy_state::identities::Identity;

use crate::readings::{FourValuesNow, HeldHardware, Readings};

/// **The three answers, taken at the moment the page needs them, carried into
/// the judgement.**
///
/// **Every fallback below is the value that claims nothing**, which is the
/// whole reason the type is shaped this way: asked about a server or a file
/// this value was not taken for, it answers with the arm that makes no claim
/// rather than with a held answer that belongs to something else.
#[derive(Debug, Clone, PartialEq)]
pub struct TheMachineThisPageIsOn {
    /// The server row the three answers were taken for.
    pub server: Identity,
    /// The `.blend`'s path on that server the four-value read was taken
    /// against.
    pub path: String,
    /// **D5's second half** — `Servers`' remembered reading with its `Servers`
    /// H1 state.
    pub hardware: HeldHardware,
    /// **B1's list**, or nothing where there was no machine to ask it of.
    pub version_list: Option<TheSearchResult>,
    /// **Q14 item 2's comparison read.**
    pub four_values: FourValuesNow,
}

impl Readings for TheMachineThisPageIsOn {
    fn the_hardware_reading(&self, server: &Identity) -> HeldHardware {
        if *server != self.server {
            // `Servers` H1's *never read* is the absence of a reading Smoozy
            // may show — which is exactly what is true of a row these answers
            // were not taken for.
            return HeldHardware::NeverRead;
        }
        self.hardware.clone()
    }

    fn the_version_list(&self, server: &Identity) -> Option<TheSearchResult> {
        if *server != self.server {
            return None;
        }
        self.version_list.clone()
    }

    fn the_four_values_now(&self, server: &Identity, path: &str) -> FourValuesNow {
        if *server != self.server || path != self.path {
            // Nothing is claimed, which is what Q14 item 2 already does where
            // the channel cannot answer.
            return FourValuesNow::TheChannelCannotAnswer;
        }
        self.four_values.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_half::testing::{a_server, four_values};

    fn answers(server: &Identity) -> TheMachineThisPageIsOn {
        TheMachineThisPageIsOn {
            server: server.clone(),
            path: "/ephemeral/smoozy/assets/kitchen.blend".to_string(),
            hardware: HeldHardware::NeverRead,
            version_list: None,
            four_values: FourValuesNow::Answered(four_values(4096)),
        }
    }

    #[test]
    fn an_answer_taken_for_one_server_is_never_handed_back_for_another() {
        let taken_for = a_server();
        let somebody_else = a_server();
        let held = answers(&taken_for);
        assert_eq!(
            held.the_four_values_now(&taken_for, "/ephemeral/smoozy/assets/kitchen.blend"),
            FourValuesNow::Answered(four_values(4096))
        );
        assert_eq!(
            held.the_four_values_now(&somebody_else, "/ephemeral/smoozy/assets/kitchen.blend"),
            FourValuesNow::TheChannelCannotAnswer
        );
        assert_eq!(
            held.the_hardware_reading(&somebody_else),
            HeldHardware::NeverRead
        );
        assert!(held.the_version_list(&somebody_else).is_none());
    }

    #[test]
    fn a_four_value_answer_taken_against_one_file_is_never_handed_back_for_another() {
        // The silent-wrong-answer this arm exists to stop: the comparison
        // would then run one file's values against another file's reading.
        let server = a_server();
        let held = answers(&server);
        assert_eq!(
            held.the_four_values_now(&server, "/ephemeral/smoozy/assets/other.blend"),
            FourValuesNow::TheChannelCannotAnswer
        );
    }
}
