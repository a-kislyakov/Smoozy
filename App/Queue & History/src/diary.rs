//! This module's diary declaration — `Queue & History` §17's supply to
//! `Reporting a problem`.
//!
//! The rules are `Reporting a problem` **J2, J2a, J3, J4 and J4a**, taken as
//! they stand. **The list is fixed**, and an act absent from it is one this
//! document does not write rather than one nobody got to. Lines are written
//! through the diary service `Desktop app and delivery` §5.3 names; a module
//! declares or writes nothing itself (`Desktop app and delivery` A7).
//!
//! **Three acts, each a write of the desktop's own rather than a reading
//! come back**: untracked work taken over (A6); a machine's jobs stopped and
//! cleared (L13–L17, and L19's group-scoped form, which is one command and
//! therefore one line); and a deletion — one entry, a machine's history, or
//! the whole of history (X1, X6, X6a). **One told line**: a job's ending as
//! the machine recorded it — one line per attempt that ended.
//!
//! **Two subjects are left out, on a ground that holds**: a cancel (L1, L5)
//! and the drop of a waiting job whose Blender was removed (L22) are both of
//! a job that never rendered, so neither has an ending told line and neither
//! is named anywhere else in the diary; both are per job rather than per
//! act, and a line for a job with no other trace **is** the showing the
//! promise forbids — in a report, where the diary actually goes. **What the
//! omission costs is stated rather than hidden**: the diary is silent about
//! the two acts that remove a job with no other trace, and about the
//! erase's own machine-side units.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Reporting a problem` J2, J2a, J3, J4, J4a,
//! U7.
//!
//! **`Reporting a problem` R7, `Desktop app and delivery` A7, `Job files and
//! outputs` N3, and `State, identity and recovery` D5 and E1, are not listed
//! above and are written out at each site**, because this plan defines a
//! clause of each name and a bare one could mean either.

use std::time::SystemTime;

use smoozy_reporting::lines::{Line, LineKind};
use smoozy_state::identities::Identity;
use smoozy_state::observed::{Clock, Reading};

use crate::ending::Ending;
use crate::stop_and_clear::ThreeResults;

/// **The two kinds this module declares**, and no others.
pub const DECLARED: [LineKind; 2] = [LineKind::Act, LineKind::Told];

/// **The three acts.** Each fixed line form carries the act's name, the
/// moment on the desktop clock stated as that clock (`State, identity and
/// recovery` D5), and the declared values — and nothing free-composed at run
/// time (`Reporting a problem` J3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnAct {
    /// **Untracked work taken over** (A6): the server's label as N12's copy
    /// carries it, and the count of jobs the acceptance covered. **Not the
    /// adopted jobs' identities or labels** — one line per act, and each
    /// job's own entry is where it is met; nothing of the reading the offer
    /// was composed from; and no line for a decline, which writes nothing.
    UntrackedWorkTakenOver {
        the_servers_label: String,
        jobs_covered: usize,
    },
    /// **A machine's jobs stopped and cleared** (L13–L17, L19): the server's
    /// label and which of L17's three results was recorded. **Not the
    /// cleared jobs' identities or labels**, not the stop's resolved subject
    /// beyond its own operation identity, and no second line for the
    /// stopped job's ending, which is the told line.
    AMachinesJobsStoppedAndCleared {
        the_servers_label: String,
        result: ThreeResults,
    },
    /// **A deletion** (X1, X6, X6a): which of the three scopes, the count of
    /// entries the act covered, and on the per-machine scope the server's
    /// label. **No deleted job's identity, no deleted job's label, and
    /// nothing of the tombstones** — the line names no job and shows no job,
    /// which is what the deletion promise is about. Nor the erase's own
    /// units or outcomes.
    ADeletion {
        scope: DeletionScope,
        entries_covered: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeletionScope {
    OneEntry,
    AMachinesHistory { the_servers_label: String },
    TheWholeOfHistory,
}

/// **The identities an act line carries**: the run identity (`Reporting a
/// problem` U7) and the server row; the stop-and-clear's two operation
/// identities and, where the group-scoped form was the act, the group
/// identity (L18); the deletion's server row on the per-machine scope
/// alone.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Concerning {
    pub run: Identity,
    pub server_row: Option<Identity>,
    pub operations: Vec<Identity>,
    pub group: Option<Identity>,
}

impl AnAct {
    pub fn name(&self) -> &'static str {
        match self {
            AnAct::UntrackedWorkTakenOver { .. } => "untracked work taken over",
            AnAct::AMachinesJobsStoppedAndCleared { .. } => "jobs stopped and cleared",
            AnAct::ADeletion { .. } => "history deleted",
        }
    }

    fn text(&self) -> String {
        match self {
            AnAct::UntrackedWorkTakenOver {
                the_servers_label,
                jobs_covered,
            } => [self.name(), the_servers_label, &jobs_covered.to_string()].join("\t"),
            AnAct::AMachinesJobsStoppedAndCleared {
                the_servers_label,
                result,
            } => {
                let result = match result {
                    ThreeResults::CouldNotBeginAtAll => "could not begin at all",
                    ThreeResults::ClearedAndStopped => "cleared and stopped",
                    ThreeResults::ClearedAndTheStopFailedOrExpiredItsWaiter => {
                        "cleared and the stop failed or expired its waiter"
                    }
                };
                [self.name(), the_servers_label, result].join("\t")
            }
            AnAct::ADeletion {
                scope,
                entries_covered,
            } => {
                let (scope, label) = match scope {
                    DeletionScope::OneEntry => ("one entry", ""),
                    DeletionScope::AMachinesHistory { the_servers_label } => {
                        ("a machine's history", the_servers_label.as_str())
                    }
                    DeletionScope::TheWholeOfHistory => ("the whole of history", ""),
                };
                [self.name(), scope, &entries_covered.to_string(), label].join("\t")
            }
        }
    }

    /// The line, ready for the diary service, at the moment of the act on
    /// the desktop's clock.
    pub fn line(&self, concerning: &Concerning, at: SystemTime) -> Line {
        let mut about = vec![concerning.run.clone()];
        about.extend(concerning.server_row.iter().cloned());
        about.extend(concerning.operations.iter().cloned());
        about.extend(concerning.group.iter().cloned());
        Line {
            run: concerning.run.clone(),
            kind: LineKind::Act,
            at,
            clock: Clock::ThisComputer,
            text: self.text(),
            concerning: about,
        }
    }
}

/// **The one told line (`Reporting a problem` J4a): a job's ending as the
/// machine recorded it** — one line per attempt that ended, never one per
/// journal event, per frame or per reading. **The fixed form**: the ending
/// in E2's own four words, and a bounded rendering of the job's label (`Job
/// files and outputs` N3).
///
/// **What it does not carry**: anything either stream said; the completeness
/// statement and the last-frame figure; the four collection outcomes; the
/// device line (E12); **the job's device ask (E12a)**, which is a field of
/// the entry written at submission and never an act or an ending; and no
/// told line for a reading of any kind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AJobsEnding {
    pub ending: Ending,
    pub a_bounded_rendering_of_the_label: String,
    /// On a re-run, the attempt number (E11).
    pub attempt: Option<u32>,
}

/// **The four identities a told line carries** (`Reporting a problem` J4):
/// the machine's own moment on the machine's clock, the per-machine journal
/// sequence number the outcome arrived at (`State, identity and recovery`
/// E1), the installation identity it was read under, and the server row —
/// with the job identity beside them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsTheMachineRecordedIt {
    pub run: Identity,
    pub the_machines_moment: Reading<()>,
    pub journal_sequence_number: u64,
    pub installation: Identity,
    pub server_row: Identity,
    pub job: Identity,
}

impl AJobsEnding {
    fn the_word(&self) -> &'static str {
        match self.ending {
            Ending::Finished => "finished",
            Ending::Failed => "failed",
            Ending::Stopped => "stopped",
            Ending::GivenUp => "given up",
        }
    }

    /// The told line. **Never on one clock**: its moment is the machine's
    /// own, on the machine's clock, as `State, identity and recovery` D5
    /// requires of a reading that came back from a machine.
    pub fn line(&self, as_recorded: &AsTheMachineRecordedIt) -> Line {
        let attempt = self
            .attempt
            .map(|attempt| attempt.to_string())
            .unwrap_or_default();
        Line {
            run: as_recorded.run.clone(),
            kind: LineKind::Told,
            at: as_recorded.the_machines_moment.taken_at(),
            clock: as_recorded.the_machines_moment.clock().clone(),
            text: [
                "job ended",
                self.the_word(),
                &self.a_bounded_rendering_of_the_label,
                &as_recorded.journal_sequence_number.to_string(),
                &attempt,
            ]
            .join("\t"),
            concerning: vec![
                as_recorded.run.clone(),
                as_recorded.installation.clone(),
                as_recorded.server_row.clone(),
                as_recorded.job.clone(),
            ],
        }
    }
}

/// **What else was tested, and the test each fails** — named so the next
/// reader is not left to re-derive them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotDeclaredHere {
    /// A reorder and a group block move (L6, L7) — the second test: nothing
    /// of Smoozy's own stands afterwards but the machine's own line redrawn.
    AReorderOrABlockMove,
    /// A stop of one rendering job (L10) — `Rendering`'s to send; what this
    /// document holds is the recorded outcome, which is the told line.
    AStopOfOneRenderingJob,
    /// Answering a job held on the user (H4) — the hold mechanism's and the
    /// tap's; the completion is the job starting, which is `Rendering`'s.
    AnsweringAHeldJob,
    /// The preview set's production and its fetch (P1, P4) — the third test,
    /// twice over.
    ThePreviewSetsProductionAndFetch,
    /// The queue reading and the live commentary read (A1, E8), the history
    /// copy, the note and the manifest at the fill-in (A1, N1, A7), and the
    /// idle reading and the catch-up notice (W1, C1) — the first test: each
    /// is a reading or a composition, not Smoozy's own act; and no journal
    /// event becomes a told line by itself.
    AReadingOrAComposition,
    /// *Show outputs*, *show a job's downloaded files* and the auto-download
    /// switch (L1a) — not this document's acts at all.
    AnotherDocumentsControlOnTheEntry,
    /// **A cancel, and the removed-Blender drop** — left out on the ground
    /// the module header states: a job that never rendered, per job rather
    /// than per act, with no other trace.
    ACancelOrARemovedBlenderDrop,
}

pub const EVERY_CANDIDATE_NOT_DECLARED: [NotDeclaredHere; 7] = [
    NotDeclaredHere::AReorderOrABlockMove,
    NotDeclaredHere::AStopOfOneRenderingJob,
    NotDeclaredHere::AnsweringAHeldJob,
    NotDeclaredHere::ThePreviewSetsProductionAndFetch,
    NotDeclaredHere::AReadingOrAComposition,
    NotDeclaredHere::AnotherDocumentsControlOnTheEntry,
    NotDeclaredHere::ACancelOrARemovedBlenderDrop,
];

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_reporting::lines::{ActTests, Declarations, ErasingActsAreNotDeclared, ToldTests};
    use smoozy_state::identities::IdentityKind;
    use smoozy_state::observed::MachineClock;

    fn concerning() -> Concerning {
        Concerning {
            run: Identity::issue(IdentityKind::Run).unwrap(),
            server_row: Some(Identity::issue(IdentityKind::LocalServer).unwrap()),
            operations: Vec::new(),
            group: None,
        }
    }

    #[test]
    fn this_module_declares_act_and_told_lines_and_no_other_kind() {
        let mut declarations: Declarations<&str> = Declarations::new();
        declarations.module_declares("Queue & History", DECLARED.to_vec());
        assert!(declarations.may_write(&"Queue & History", LineKind::Act));
        assert!(declarations.may_write(&"Queue & History", LineKind::Told));
        assert!(!declarations.may_write(&"Queue & History", LineKind::Failure));
    }

    #[test]
    fn the_three_acts_pass_the_three_tests_and_the_deletion_is_in_because_omitting_it_bought_nothing(
    ) {
        let tests = ActTests {
            smoozys_own: true,
            the_spec_names_its_completion_as_something_the_user_sees: true,
            at_the_users_own_rate: true,
        };
        // The deletion's told line was already in the diary when the machine
        // recorded the ending, and the deletion neither reaches it nor
        // could; the line as declared carries no job, so the bar does not
        // catch it.
        let bar = ErasingActsAreNotDeclared {
            the_subjects_spec_promises_no_trace_afterwards: false,
        };
        assert!(tests.is_diary_worthy(bar));
        // A cancel and a removed-Blender drop: the bar catches them.
        let erasing = ErasingActsAreNotDeclared {
            the_subjects_spec_promises_no_trace_afterwards: true,
        };
        assert!(!tests.is_diary_worthy(erasing));
        assert_eq!(EVERY_CANDIDATE_NOT_DECLARED.len(), 7);
    }

    #[test]
    fn the_told_line_is_declarable_and_is_one_per_attempt_that_ended() {
        let told = ToldTests {
            the_user_could_name_it: true,
            not_per_event_frame_derivative_file_or_byte: true,
        };
        assert!(told.may_be_declared());
    }

    #[test]
    fn an_act_line_carries_its_declared_values_and_no_jobs_identity_or_label() {
        let concerning = concerning();
        let at = SystemTime::UNIX_EPOCH;
        let adopted = AnAct::UntrackedWorkTakenOver {
            the_servers_label: "the big box".to_string(),
            jobs_covered: 3,
        }
        .line(&concerning, at);
        assert_eq!(adopted.kind, LineKind::Act);
        assert_eq!(adopted.clock, Clock::ThisComputer);
        assert_eq!(adopted.text, "untracked work taken over\tthe big box\t3");
        assert_eq!(adopted.concerning.len(), 2);

        let cleared = AnAct::AMachinesJobsStoppedAndCleared {
            the_servers_label: "the big box".to_string(),
            result: ThreeResults::ClearedAndStopped,
        }
        .line(&concerning, at);
        assert_eq!(
            cleared.text,
            "jobs stopped and cleared\tthe big box\tcleared and stopped"
        );

        let deleted = AnAct::ADeletion {
            scope: DeletionScope::TheWholeOfHistory,
            entries_covered: 12,
        }
        .line(
            &Concerning {
                server_row: None,
                ..concerning.clone()
            },
            at,
        );
        assert_eq!(deleted.text, "history deleted\tthe whole of history\t12\t");
        assert_eq!(deleted.concerning, vec![concerning.run.clone()]);
        // No job identity anywhere in an act line.
        for line in [&adopted, &cleared, &deleted] {
            assert!(line
                .concerning
                .iter()
                .all(|identity| identity.kind() != IdentityKind::Job));
        }
    }

    #[test]
    fn the_told_line_carries_the_ending_in_four_words_on_the_machines_clock_with_the_four_identities(
    ) {
        let as_recorded = AsTheMachineRecordedIt {
            run: Identity::issue(IdentityKind::Run).unwrap(),
            the_machines_moment: Reading::taken(
                (),
                SystemTime::UNIX_EPOCH,
                Clock::AMachine(MachineClock("server-7".to_string())),
            ),
            journal_sequence_number: 812,
            installation: Identity::issue(IdentityKind::Installation).unwrap(),
            server_row: Identity::issue(IdentityKind::LocalServer).unwrap(),
            job: Identity::issue(IdentityKind::Job).unwrap(),
        };
        let line = AJobsEnding {
            ending: Ending::GivenUp,
            a_bounded_rendering_of_the_label: "Kitchen".to_string(),
            attempt: Some(2),
        }
        .line(&as_recorded);
        assert_eq!(line.kind, LineKind::Told);
        assert_eq!(
            line.clock,
            Clock::AMachine(MachineClock("server-7".to_string()))
        );
        assert_eq!(line.text, "job ended\tgiven up\tKitchen\t812\t2");
        assert_eq!(line.concerning.len(), 4);
        assert!(line.concerning.contains(&as_recorded.job));
    }
}
