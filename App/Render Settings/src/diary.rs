//! This module's diary declaration — `Render Settings` §12's last part.
//!
//! The rules are `Reporting a problem` **J2, J2a, J3, J4 and J4a**, taken as
//! they stand. **The list below is fixed** — what is not on it is not written
//! — and nothing on it is a new act, a new reading, a new check or a new
//! store. Lines are written through the diary service `Desktop app and
//! delivery` §5.3 L14 names, whose hosting rule — a module declares or writes
//! nothing — is that document's A7.
//!
//! **The act, and there is one: a `.blend` read — Pre-load Settings**, in the
//! settled words the spec gives this module. It passes J2a's three tests: it
//! is Smoozy's own act, driven from the desktop as its own operation at the
//! user's own press (Q4); the spec names its completion as what the user sees
//! afterwards — the settings standing filled with the file's real values, and
//! the header stating when the file was read (Q18); and it runs at the user's
//! own rate — one line per Pre-load the user asked for, with Q5's rule that a
//! second asked for while one runs joins the running one.
//!
//! **Three things this document does that are not acts of its own**: the
//! render-start reading, a step inside the admitted render operation; the
//! payload's composition, a step inside submitting a job; and Q14 item 2's
//! comparison read, a reading that came back from a machine, taken for the
//! page's display alone. **This document writes no told lines, and the
//! absence is a decision**: what it consumes from a machine — the reading
//! itself, and the comparison read — comes back on the command path as the
//! answer to a command, so no per-machine sequence number is in hand for a
//! line to carry, and neither carries a moment on the machine's own clock.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Reporting a problem` J2a, J4a, N2, N3, U7.
//! `Desktop app and delivery` A7, L14, §16.
//!
//! **`Reporting a problem` J2, J3 and J4, and `State, identity and recovery`
//! D5, are not listed above and are written out at each site**, because this
//! plan defines a J2, a J3, a J4 and a D5 of its own and a bare one could
//! mean either.

use std::time::SystemTime;

use smoozy_reporting::lines::{Line, LineKind};
use smoozy_state::identities::Identity;
use smoozy_state::observed::Clock;

use crate::file_half::{version_text, BlenderIdentity};

/// **The one kind this module declares**, and no others.
pub const DECLARED: [LineKind; 1] = [LineKind::Act];

/// **The act: a `.blend` read.**
///
/// **The fixed line form**: the act's name; the moment on the desktop's
/// clock, stated as that clock (`State, identity and recovery` D5) — Q18's
/// own stamp at the storing, so the line invents no second moment; the
/// `.blend`'s recorded path; and the Blender the reading was taken by, its
/// build path and its answered version (Q11).
///
/// **What the line does not carry, and the first exclusion is the whole of
/// the boundary: nothing the reading returned.** No frame range, no sample
/// count, no engine, no device, no scene list, no view-layer name, no value
/// of any other setting — those are the `.blend`'s contents, which `Reporting
/// a problem` N2 bars from the diary outright, and its N3's acted-on rule
/// admits *the file the module read* and never what was inside it. Nor the
/// four-value file identity (Q13), which is a mechanism's own vocabulary. **And
/// never one line per what the read covers.**
///
/// **A read that gave nothing writes no act line, because nothing was read**
/// — Q7's no-reading, Q8's three endings, the read Smoozy itself ended, and
/// Q17's reading discarded on landing all store nothing. What the user met in
/// each case still reaches the diary as the sentence the page said, captured
/// verbatim by `Desktop app and delivery` §16's plumbing rather than written
/// here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ABlendRead {
    pub the_blends_recorded_path: String,
    pub taken_by: BlenderIdentity,
}

/// **The identities the line carries**: the run identity (`Reporting a
/// problem` U7); the server row's local identifier, with the user's own label
/// for the machine beside it and never in place of it; and the
/// file-inspection operation's own operation identity (W3). **There is no job
/// identity and no group identity** — a pre-load happens before any job of
/// that file exists.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Concerning {
    pub run: Identity,
    pub server_row: Identity,
    pub the_users_label_for_the_machine: String,
    pub operation: Identity,
}

impl ABlendRead {
    pub const NAME: &'static str = "blend read";

    fn text(&self, label: &str) -> String {
        [
            ABlendRead::NAME,
            &self.the_blends_recorded_path,
            &self.taken_by.path,
            &version_text(self.taken_by.version),
            label,
        ]
        .join("\t")
    }

    /// The line, ready for the diary service. **The moment is Q18's stamp**,
    /// on the desktop's clock, stated as that clock.
    pub fn line(&self, concerning: Concerning, stored_at: SystemTime) -> Line {
        Line {
            run: concerning.run.clone(),
            kind: LineKind::Act,
            at: stored_at,
            clock: Clock::ThisComputer,
            text: self.text(&concerning.the_users_label_for_the_machine),
            concerning: vec![concerning.run, concerning.server_row, concerning.operation],
        }
    }
}

/// **Told lines: none**, and the empty half is written out rather than left
/// silent, because a missing declaration and an empty one read alike from
/// the diary's end. Where `Reporting a problem` J4a's identities are ever
/// widened to admit a reading that arrives on the command path, this reading
/// is the one that comes with them, and that is the runner's to see.
pub const NO_TOLD_LINES: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_half::testing::{a_blender, a_moment};
    use smoozy_reporting::lines::{ActTests, Declarations, ErasingActsAreNotDeclared, ToldTests};
    use smoozy_state::identities::IdentityKind;

    fn concerning() -> Concerning {
        Concerning {
            run: Identity::issue(IdentityKind::Run).expect("randomness"),
            server_row: Identity::issue(IdentityKind::LocalServer).expect("randomness"),
            the_users_label_for_the_machine: "the big box".to_string(),
            operation: Identity::issue(IdentityKind::Operation).expect("randomness"),
        }
    }

    fn a_read() -> ABlendRead {
        ABlendRead {
            the_blends_recorded_path: "/ephemeral/smoozy/assets/kitchen.blend".to_string(),
            taken_by: a_blender(),
        }
    }

    #[test]
    fn this_module_declares_act_lines_and_no_told_lines() {
        let mut declarations: Declarations<&str> = Declarations::new();
        declarations.module_declares("Render Settings", DECLARED.to_vec());
        assert!(declarations.may_write(&"Render Settings", LineKind::Act));
        assert!(!declarations.may_write(&"Render Settings", LineKind::Told));
    }

    #[test]
    fn the_one_act_passes_all_three_tests_and_no_told_line_is_declarable() {
        let tests = ActTests {
            smoozys_own: true,
            the_spec_names_its_completion_as_something_the_user_sees: true,
            at_the_users_own_rate: true,
        };
        let bar = ErasingActsAreNotDeclared {
            the_subjects_spec_promises_no_trace_afterwards: false,
        };
        assert!(tests.is_diary_worthy(bar));
        // The comparison read and the recorded reading come back as the
        // answer to a command, with no machine moment and no sequence number,
        // so neither is a thing the user could name as a machine's own
        // ending.
        let told = ToldTests {
            the_user_could_name_it: false,
            not_per_event_frame_derivative_file_or_byte: true,
        };
        assert!(!told.may_be_declared());
    }

    #[test]
    fn the_line_carries_the_path_the_blender_and_the_label_beside_the_identities_and_nothing_the_reading_returned(
    ) {
        let concerning = concerning();
        let line = a_read().line(concerning.clone(), a_moment());
        assert_eq!(line.kind, LineKind::Act);
        assert_eq!(line.clock, Clock::ThisComputer);
        assert_eq!(line.at, a_moment());
        assert_eq!(
            line.text,
            "blend read\t/ephemeral/smoozy/assets/kitchen.blend\t/ephemeral/smoozy/blender/blender-4.5.12/blender\t4.5.12\tthe big box"
        );
        assert_eq!(
            line.concerning,
            vec![
                concerning.run.clone(),
                concerning.server_row,
                concerning.operation
            ]
        );
        assert_eq!(line.run, concerning.run);
        // The label is beside the identifier, never in place of it: the
        // identifier is in `concerning`, the label in the text.
        assert!(!line.text.contains(&line.run.as_text()));
    }
}
