//! This module's diary declaration — `Dependency Upload & Remap` §11's last
//! part: two acts, and no told lines.
//!
//! The rules are `Reporting a problem` **J2a, J3, J4 and J4a**, taken as they
//! stand. **Nothing below is new conduct** — each act named is one this
//! document already performs, and the list is fixed: an act failing any of
//! the three tests is not declared, and none is manufactured to fill the
//! entry.
//!
//! **What was considered and is not declared, each on the test it fails**: a
//! hand-over from `File Browser` (a line per unit is a line per file — the
//! third test — and there is no completion of a set for the second to read);
//! a row resolved, a guess rejected, a row set aside, an act over a selection
//! (decisions **inside** a sitting — the second test — and per row, the
//! third); the shift (what it produces is rows in the sitting the user is
//! already looking at — the second); producing a fixed copy and placing a
//! job's remap material (steps inside something else, per linked file and per
//! job — the second and the third); removing an asset or a job's material
//! (`File Browser`'s or `Queue & History`'s act, not this document's).
//!
//! **This document writes no told lines, and the absence is declared so it is
//! a decision rather than a silence.** Every machine ending and reading it
//! consumes arrives per file, per row or per member — the check reading's
//! return, the four-value read and the digest, a transfer's publication, the
//! fixing's ending. A transfer's ending is J4a's own example of a told line,
//! and it is refused here on its rate rather than on its merits.
//!
//! **Where the boundary with `Server protocol and transfers` falls**: what is
//! declared here is the act at the rate the user asked for it — the send,
//! whatever it covers — and what that document owns is the transfer
//! underneath, per file and per resume, at nobody's rate.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Reporting a problem` J2a, J3, J4, J4a, J6, N2,
//! U7. `State, identity and recovery` D5.

use std::time::SystemTime;

use smoozy_reporting::lines::{Line, LineKind};
use smoozy_state::identities::Identity;
use smoozy_state::observed::Clock;

use crate::standing::Completion;

/// **The one kind this module declares**, and no others.
pub const DECLARED: [LineKind; 1] = [LineKind::Act];

/// **The identities a line carries**: the run identity (`Reporting a problem`
/// U7), and the server row and the installation identity the act ran under
/// (A9, K7; G2, V1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Concerning {
    pub run: Identity,
    pub server_row: Identity,
    pub installation: Identity,
}

/// **The first act: *sent a batch*** — Section 10's *upload set after a
/// check* performed once, for the one press that names a number of files.
///
/// **Why it passes the three tests.** It is Smoozy's own act, arranged by the
/// desktop when the user asks for it; its completion is what the user has
/// afterwards — every row's own state on the panel, read from the placement
/// records publication wrote (Q5, K7), and the files standing in the asset
/// area (A4); and it runs at the user's own rate — one line for the send,
/// whatever number of files that send covers.
///
/// **The fixed line form**: the act's name; the moment on the desktop clock,
/// stated as that clock (`State, identity and recovery` D5); the file the
/// check was run on, as the user's own location for it; and three figures —
/// how many rows the send covered, how many were sent, and how many ended
/// *could not be sent* (A5, A11). **What the line never carries**: a row, a
/// file name or a member of a set row one by one — a batch is many files, and
/// a line per file is exactly what the third test refuses (G23); no content
/// identity, digest, transfer identity or staging path; and nothing read out
/// of any file of the user's (`Reporting a problem` N2). **A row re-sent on
/// its own writes one line of this same act** (A11, F26); a retry, a resume
/// or a re-connection **inside** one transfer writes none (A12).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SentABatch {
    pub the_files_own_location: String,
    pub rows_covered: usize,
    pub sent: usize,
    pub could_not_be_sent: usize,
}

impl SentABatch {
    pub const NAME: &'static str = "sent a batch";

    pub fn line(&self, concerning: Concerning, at: SystemTime) -> Line {
        Line {
            run: concerning.run.clone(),
            kind: LineKind::Act,
            at,
            clock: Clock::ThisComputer,
            text: [
                SentABatch::NAME,
                &self.the_files_own_location,
                &self.rows_covered.to_string(),
                &self.sent.to_string(),
                &self.could_not_be_sent.to_string(),
            ]
            .join("\t"),
            concerning: vec![
                concerning.run,
                concerning.server_row,
                concerning.installation,
            ],
        }
    }
}

/// **The second act: *checked a file's dependencies*** — one check sitting,
/// from the reading of the file the user pointed at to the sitting's end.
///
/// **Why it passes the three tests.** The act is the desktop's own loop (G14,
/// G15); its completion is what the user is told about afterwards — the rows
/// and the figures beneath them (G21–G23), and the check standing that stops
/// `Rendering`'s warning (V10, V11); and it runs at the user's own rate —
/// **one line per check the user asked for, never one per file read in the
/// tree** (a chain is recorded to at least nineteen libraries deep, G13), and
/// never one per row and never one per pass.
///
/// **The fixed line form**: the act's name; the moment on the desktop clock;
/// the file the check was run on, as the user's own location for it; the
/// Blender version the readings used — **`Render Settings`' selection as G2
/// consumes it and never a reading of the machine**; three figures — files
/// read, rows standing, and how many of those rows still need sending; and
/// how the sitting ended, as one of three declared values (G15, G16).
///
/// **Where no check runs at all — no version can be resolved, or the named
/// Blender cannot open the file — no act line is written**, because nothing
/// was checked (G2, G5). What the user meets there is a failure, and a
/// failure line is `Reporting a problem` J6's, captured verbatim at
/// presentation by the plumbing rather than declared here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckedAFilesDependencies {
    pub the_files_own_location: String,
    pub the_version_the_readings_used: String,
    pub files_read: usize,
    pub rows_standing: usize,
    pub rows_still_needing_sending: usize,
    pub ended: Completion,
}

impl CheckedAFilesDependencies {
    pub const NAME: &'static str = "checked a file's dependencies";

    fn ending_word(ended: Completion) -> &'static str {
        match ended {
            Completion::Completed => "completed",
            Completion::StoppedByTheUser => "stopped by the user",
            Completion::TheConnectionDropped => "the connection dropped",
        }
    }

    pub fn line(&self, concerning: Concerning, at: SystemTime) -> Line {
        Line {
            run: concerning.run.clone(),
            kind: LineKind::Act,
            at,
            clock: Clock::ThisComputer,
            text: [
                CheckedAFilesDependencies::NAME,
                &self.the_files_own_location,
                &self.the_version_the_readings_used,
                &self.files_read.to_string(),
                &self.rows_standing.to_string(),
                &self.rows_still_needing_sending.to_string(),
                CheckedAFilesDependencies::ending_word(self.ended),
            ]
            .join("\t"),
            concerning: vec![
                concerning.run,
                concerning.server_row,
                concerning.installation,
            ],
        }
    }
}

/// **Told lines: none**, and the empty half is written out rather than left
/// silent, because a missing declaration and an empty one read alike from
/// the diary's end.
pub const NO_TOLD_LINES: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_reporting::lines::{ActTests, Declarations, ErasingActsAreNotDeclared, ToldTests};
    use smoozy_state::identities::IdentityKind;

    fn concerning() -> Concerning {
        Concerning {
            run: Identity::issue(IdentityKind::Run).expect("randomness"),
            server_row: Identity::issue(IdentityKind::LocalServer).expect("randomness"),
            installation: Identity::issue(IdentityKind::Installation).expect("randomness"),
        }
    }

    #[test]
    fn this_module_declares_act_lines_and_no_told_lines() {
        let mut declarations: Declarations<&str> = Declarations::new();
        declarations.module_declares("Dependency Upload & Remap", DECLARED.to_vec());
        assert!(declarations.may_write(&"Dependency Upload & Remap", LineKind::Act));
        assert!(!declarations.may_write(&"Dependency Upload & Remap", LineKind::Told));
    }

    #[test]
    fn both_acts_pass_the_three_tests_and_a_per_file_ending_is_not_a_told_line() {
        let tests = ActTests {
            smoozys_own: true,
            the_spec_names_its_completion_as_something_the_user_sees: true,
            at_the_users_own_rate: true,
        };
        let bar = ErasingActsAreNotDeclared {
            the_subjects_spec_promises_no_trace_afterwards: false,
        };
        assert!(tests.is_diary_worthy(bar));
        let told = ToldTests {
            the_user_could_name_it: true,
            not_per_event_frame_derivative_file_or_byte: false,
        };
        assert!(!told.may_be_declared());
    }

    #[test]
    fn the_sent_a_batch_line_carries_the_three_figures_and_no_file_name_of_a_row() {
        let concerning = concerning();
        let line = SentABatch {
            the_files_own_location: "/Users/a/proj/scene.blend".to_string(),
            rows_covered: 12,
            sent: 11,
            could_not_be_sent: 1,
        }
        .line(concerning.clone(), SystemTime::UNIX_EPOCH);
        assert_eq!(line.kind, LineKind::Act);
        assert_eq!(line.clock, Clock::ThisComputer);
        assert_eq!(
            line.text,
            "sent a batch\t/Users/a/proj/scene.blend\t12\t11\t1"
        );
        assert_eq!(
            line.concerning,
            vec![
                concerning.run.clone(),
                concerning.server_row,
                concerning.installation
            ]
        );
        assert_eq!(line.run, concerning.run);
    }

    #[test]
    fn the_checked_line_names_the_version_the_figures_and_one_of_three_endings() {
        let line = CheckedAFilesDependencies {
            the_files_own_location: "/Users/a/proj/scene.blend".to_string(),
            the_version_the_readings_used: "4.5.12".to_string(),
            files_read: 3,
            rows_standing: 20,
            rows_still_needing_sending: 4,
            ended: Completion::TheConnectionDropped,
        }
        .line(concerning(), SystemTime::UNIX_EPOCH);
        assert_eq!(
            line.text,
            "checked a file's dependencies\t/Users/a/proj/scene.blend\t4.5.12\t3\t20\t4\tthe connection dropped"
        );
    }
}
