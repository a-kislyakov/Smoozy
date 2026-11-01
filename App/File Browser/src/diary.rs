//! This module's diary declaration — `File Browser` §11.1, B1 to B5.
//!
//! The rules are `Reporting a problem` **J2a's three tests, J3's line form and
//! J4a's told lines**, taken as they stand and not reinterpreted. **Lines are
//! written through the diary service `Desktop app and delivery` L14, at that
//! document's §5.3, names**, whose hosting rule — **A7 makes a module with no
//! declaration write
//! nothing, silently** — is that document's.
//!
//! **This subsection adds no act, no reading, no check and no store**: every act
//! declared below is one §7 or §3.1 already performs, and each line is written
//! at a durable write that already happens — `State, identity and recovery`
//! §9.4 item 2's write of the act's own outcome, the occasion U9a already
//! names — **so no second occasion, no sweep and no reading is created for the
//! diary.**
//!
//! **The two acts are declared here and both are performed.** The file act's
//! own five are §7's ([`crate::managing`]) and the request act's line is
//! composed off §3's record ([`crate::request::UploadRequest::how_it_ended`] and
//! [`crate::request::UploadRequest::the_four_counts`]). **The one field of the
//! file act's line that stood empty is filled** — the job identities §8's
//! telling makes durable, which arrived with §8 on 2026-09-04 and is taken from
//! that telling rather than composed again
//! ([`WhatTheFileActLineNames::with_the_telling`]). A declaration that exists
//! only in this file's tests is a module that writes nothing, so [`DECLARED`]
//! is what is handed to `Reporting a problem`'s registration at start-up.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Reporting a problem` J2a, J3, J4, J4a, J6, J8, §2.
//! `Desktop app and delivery` A7, §5.3, §16. `State, identity and recovery`
//! D5, D6, §9.4. `Server protocol and transfers` S26.
//!
//! **A marker this plan also defines is deliberately not listed above,
//! whichever document is meant, and is written out with its document at every
//! site instead** — the header list cannot tell the two apart, and a bare one
//! would resolve to a real and wrong clause. This plan defines an N2, an N3 and
//! an L14 of its own — names carried as bytes, the plain make-directory, and
//! the cache being a tree in process memory — so none of the three is listed
//! above.

use smoozy_reporting::lines::{ActTests, ErasingActsAreNotDeclared, LineKind, ToldTests};
use smoozy_state::identities::Identity;

/// **The one kind this module declares**, and no others.
///
/// **B5: this document writes no told lines**, so the array is one long and
/// the empty half is written out at [`WhyThereAreNoToldLines`] rather than left
/// silent — a missing declaration and an empty one read alike from the diary's
/// end.
pub const DECLARED: [LineKind; 1] = [LineKind::Act];

/// **B1: the list is two acts, and the act the spec's five give this module is
/// not one of them — *read a file* is not this document's, by the spec's own
/// sentence.**
///
/// `Preparation/Specs/File Browser.md` says it in terms: *"Choosing a `.blend`
/// from the browser is how a render begins … Choosing the file does **not**
/// read it — reading is a separate action, **Pre-load Settings**, that belongs
/// to Render Settings"*.
///
/// **The one file this document does read is the `.blend`'s opening bytes**
/// (L7–L10), **and that reading fails J2a's third test outright**: the version
/// tag is taken for **every `.blend` in a listing**, unasked, filling in behind
/// the four fields, so a line for it is one line per file — the shape the test
/// forbids in its own words — and there is no act the user asked for called
/// *read this file* for it to run at the rate of. **It fails the second test
/// too**: nothing of it stands afterwards, the tag being a live reading dropped
/// with the period.
///
/// **So it is not declared here, and no act is manufactured in its place.**
/// Both facts are stated rather than left, because a reader who takes the
/// spec's five as an assignment will otherwise look for it in this document and
/// find nothing.
pub const READING_A_FILE_IS_NOT_THIS_DOCUMENTS_ACT: bool = true;

/// **B2: the first act — an upload request ended.**
///
/// It passes all three of J2a's tests: **it is Smoozy's own**, arranged,
/// admitted and published by the desktop, with nothing in it composed from what
/// a machine reported; **the spec names its completion outright** — *"Stopping
/// an upload stops the whole request: files already finished stay on the
/// server"* — and U1 makes the request's own ending a derived field the transfer
/// window reads afterwards, with the files themselves standing in a folder; and
/// **it runs at the user's rate**, one line per request the user confirmed,
/// three hundred textures being three hundred transfers and not three hundred
/// lines.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnUploadRequestEnded {
    /// The moment, on the desktop clock, stated as that clock (D5). The clock
    /// itself is [`smoozy_state::observed::Clock::ThisComputer`] and rides the
    /// line rather than this form.
    pub at: std::time::SystemTime,
    /// **The server row's own label**, carried beside the identity and never in
    /// place of it.
    pub server_row_label: String,
    /// **The destination folder, as the machine path the user was browsing**
    /// (U1).
    pub destination_folder: String,
    /// **One of two.** The third ending writes no line — see
    /// [`WhyTheThirdEndingWritesNoLine`].
    pub ending: HowARequestEnded,
    /// **Four counts over the enumeration** — units to send, done, skipped
    /// existing, handed over (U11, U12, U14).
    pub counts: TheFourCounts,
}

/// The two endings that write a line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HowARequestEnded {
    Completed,
    StoppedByTheUser,
}

/// U1's third ending, and why it writes nothing.
///
/// **A request *ended by the run ending* has no write to hang a line on** — D6
/// writes nothing on exit, which is U9a's own reasoning for the moment field —
/// **so what records that run's end is `Reporting a problem` J8's last line,
/// and nothing is promised here beside it.** Said rather than left to be
/// discovered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhyTheThirdEndingWritesNoLine;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TheFourCounts {
    pub to_send: usize,
    pub done: usize,
    pub skipped_existing: usize,
    pub handed_over: usize,
}

/// **The identities B2's line carries**, and no others.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheUploadLineNames {
    /// The run identity of the run that wrote it (`Reporting a problem` §2).
    pub run: Identity,
    /// **The request's own operation identity**, the `State, identity and
    /// recovery` §9.4 record's.
    pub request: Identity,
    pub server_row: Identity,
    pub installation: Option<Identity>,
}

/// **B3: the second act — a file act on the machine, completed.**
///
/// §7's five — delete, rename, move, copy, new folder — **one line per act, and
/// the five share one form.** All three tests hold: each is a request Smoozy
/// itself issues, **the machine's refusal being the answer to the act rather
/// than the act**; the spec's *Managing files on the server* names each, and
/// what each leaves stands in the folder afterwards — permanently, for a
/// delete; and **it is one line per act the user asked for**, a delete of fifty
/// entries being one act over fifty units and never fifty lines.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AFileActCompleted {
    /// **From the closed set below.**
    pub act: FileAct,
    pub at: std::time::SystemTime,
    pub server_row_label: String,
    /// **The path the act was about** — the entry's own path where the act
    /// names one thing, and the folder with the count of entries where it names
    /// several.
    pub about: WhatTheActWasAbout,
    /// **The destination path as well, for a move or a copy**, and absent for
    /// the other three. **An absence is written as one.**
    pub destination: Option<String>,
    /// **Whether the act ran ordinary or elevated** (P6, P8).
    pub ran: HowItRan,
    /// **The counts of units done, refused and not attempted** (P9, §9.4's
    /// derivation, never a second stored field).
    pub counts: TheThreeCounts,
}

/// **B3's closed set of five.** A sixth is a hole in §7 rather than a case this
/// form does not cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileAct {
    Delete,
    Rename,
    Move,
    Copy,
    NewFolder,
}

impl FileAct {
    pub const ALL: [FileAct; 5] = [
        FileAct::Delete,
        FileAct::Rename,
        FileAct::Move,
        FileAct::Copy,
        FileAct::NewFolder,
    ];

    /// Whether the line carries a destination path — **a move or a copy, and
    /// neither of the other three.**
    pub fn carries_a_destination(self) -> bool {
        matches!(self, FileAct::Move | FileAct::Copy)
    }

    /// **The act's name on the line**, from B3's closed set as §11.1 spells it.
    pub fn name(self) -> &'static str {
        match self {
            FileAct::Delete => "delete",
            FileAct::Rename => "rename",
            FileAct::Move => "move",
            FileAct::Copy => "copy",
            FileAct::NewFolder => "new folder",
        }
    }
}

impl AFileActCompleted {
    /// **B3's fixed form, filled**, one tab between fields: the act's name, the
    /// server row's own label, the path the act was about, the destination —
    /// *no destination* written as that absence for the three acts that carry
    /// none — how it ran, and the three counts. The moment rides the line on
    /// the desktop clock (`State, identity and recovery` D5). **Nothing else
    /// is on it**: no entry one by one, no listing, nothing read out of a file
    /// ([`NeverOnALine`]). Built at step 10c's Files' owed acts piece 3, when
    /// the first of §7's acts was performed.
    pub fn line(&self, names: &WhatTheFileActLineNames) -> smoozy_reporting::lines::Line {
        let about = match &self.about {
            WhatTheActWasAbout::OneThing(path) => path.clone(),
            WhatTheActWasAbout::AFolderAndACount { folder, entries } => {
                format!("{folder}, {entries} entries")
            }
        };
        let destination = self
            .destination
            .clone()
            .unwrap_or_else(|| "no destination".to_string());
        let ran = match self.ran {
            HowItRan::Ordinary => "ordinary",
            HowItRan::Elevated => "elevated",
        };
        let counts = format!(
            "{} done, {} refused, {} not attempted",
            self.counts.done, self.counts.refused, self.counts.not_attempted
        );
        let mut concerning = vec![names.run.clone(), names.act.clone(), names.server_row.clone()];
        concerning.extend(names.installation.iter().cloned());
        concerning.extend(names.jobs_the_telling_named.iter().cloned());
        smoozy_reporting::lines::Line {
            run: names.run.clone(),
            kind: LineKind::Act,
            at: self.at,
            clock: smoozy_state::observed::Clock::ThisComputer,
            text: [
                self.act.name(),
                &self.server_row_label,
                &about,
                &destination,
                ran,
                &counts,
            ]
            .join("\t"),
            concerning,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheActWasAbout {
    /// The entry's own path, where the act names one thing.
    OneThing(String),
    /// The folder and the count, where it names several. **Never the entries
    /// one by one** — that is a line per file by another route.
    AFolderAndACount { folder: String, entries: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HowItRan {
    Ordinary,
    /// **The elevated retry adds no line**, exactly as it adds no partition
    /// unit: elevation changes the route, never the unit, so it is a value on
    /// the act's own line and not an act of its own.
    Elevated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TheThreeCounts {
    pub done: usize,
    pub refused: usize,
    pub not_attempted: usize,
}

/// **The identities B3's line carries**, and no others.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheFileActLineNames {
    pub run: Identity,
    /// The act's operation identity (§9.4).
    pub act: Identity,
    pub server_row: Identity,
    pub installation: Option<Identity>,
    /// **Where the act was pushed past the live-job warning, the affected job
    /// identities the telling already made durable in that same record** (W8).
    ///
    /// **Carried because W8's whole ground is the gap in time between the
    /// warning and the failure hours later, and a report is read across exactly
    /// that gap.** The jobs are taken from the telling itself
    /// ([`crate::warnings::TheTellingThatStandsFirst`]) and are composed
    /// nowhere else; **an act that was not pushed past the live-job warning
    /// names none**, and the empty list is that absence written as one.
    pub jobs_the_telling_named: Vec<Identity>,
}

impl WhatTheFileActLineNames {
    /// **The line's job field, taken from W8's telling.**
    ///
    /// The telling is the durable write that stood before the act's first
    /// machine effect, so the line names exactly the jobs that write already
    /// names — **one fact with one home**, never a second enumeration composed
    /// at the diary.
    pub fn with_the_telling(
        mut self,
        telling: &crate::warnings::TheTellingThatStandsFirst,
    ) -> WhatTheFileActLineNames {
        self.jobs_the_telling_named = telling.the_jobs.clone();
        self
    }
}

/// **What no line of this module's ever carries.**
///
/// - **The enumeration** — not one entry, not one source path on the user's
///   computer, not one landing path, because a line naming them is a line per
///   file by another route and `Reporting a problem` N3 admits the act's own
///   subject rather than an inventory.
/// - **The collision answer and the per-file dispositions**, which are the
///   record's and not the act's outcome.
/// - **No second line for anything inside the act** — not a transfer, not a
///   carry-on across a repair, not a resume, not a retry, not a byte.
/// - **The entries of a multi-entry act one by one**, and **the folder's
///   listing**, which no line ever holds (`Reporting a problem` N3).
/// - **Nothing read out of any file** — no `.blend` prefix byte and no version
///   tag reaches a line, which is `Reporting a problem` N2 by construction here,
///   since L7's reading is not an act.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NeverOnALine;

/// **B4: what else was tested against the three and is not declared, each on
/// the test it failed.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotDeclared {
    pub what: &'static str,
    /// Which of J2a's three tests it failed.
    pub failed_test: u8,
    pub why: &'static str,
}

/// The list, with the test each one failed.
pub const TESTED_AND_NOT_DECLARED: [NotDeclared; 6] = [
    NotDeclared {
        what: "opening a folder (L1)",
        failed_test: 2,
        why: "the spec's own words for a listing are that it is what Smoozy saw when it last looked — a live surface that goes stale and is dropped with the period (L14, L16) — so nothing of a navigation stands afterwards on a screen, in a row, in history or in a folder. It fails the first as well, its content being a reading that came back from the machine",
    },
    NotDeclared {
        what: "the already-exists walk (X1)",
        failed_test: 2,
        why: "it is a step inside the upload request, and §11's table already declares it a reading, complete or not asked",
    },
    NotDeclared {
        what: "the free-space figure (L11-L13)",
        failed_test: 1,
        why: "a reading the machine returned",
    },
    NotDeclared {
        what: "the permission walk's answers (P1-P4)",
        failed_test: 1,
        why: "readings the machine returned",
    },
    NotDeclared {
        what: "a single transfer's ending, a resume, a retry and a per-file publication",
        failed_test: 3,
        why: "one line per file, per retry or per byte inside one act — the shape the third test forbids in its own words. A transfer Smoozy stops because the source changed (K8, K9) fails it for the same reason, and what the user meets there is a failure sentence, which is `Reporting a problem` J6's kind, captured at presentation by `Desktop app and delivery` §16's plumbing and never composed here",
    },
    NotDeclared {
        what: "putting something on the pasteboard (M11-M18)",
        failed_test: 2,
        why: "nothing on the machine changes until the paste, and the paste is B3's act",
    },
];

/// **`Reporting a problem` N3's illustration is not a declaration**, and it is
/// named here so a reader does not take it for one.
///
/// *"A line says read a listing at this folder"* is that clause's example of the
/// acted-on boundary — **the line names the folder and never the listing** —
/// and **no module owes the line it draws.**
pub const N3S_ILLUSTRATION_IS_NOT_A_DECLARATION: bool = true;

/// **B5: this document writes no told lines, and the absence is declared so it
/// is a decision rather than a silence.**
///
/// J4a asks which of a machine's endings and readings each module writes as told
/// lines, **and this document consumes none that qualify.** The four things it
/// reads off a machine — the listing (L1), the free-space figure (L11), the
/// `.blend` prefix (L7) and the permission answers (P1) — **are none of them an
/// ending or a reading the user could name, and each runs per folder, per file
/// or per click rather than at the user's rate.**
///
/// **None of them arrives as an event on the machine's journal either**, so
/// J4's required per-machine sequence number does not exist for any of them,
/// and J4a's own sentence that the read-forward is not itself a source of told
/// lines leaves nothing here to be one. **The two readings J4 names by hand —
/// the machine's Blender version and its Linux family — are `Rendering`'s and
/// `Server Setup`'s and are never taken here.** A transfer's own ending is
/// `Server protocol and transfers` S26's record and is per file, which is the
/// same bound again.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhyThereAreNoToldLines;

impl WhyThereAreNoToldLines {
    /// The four machine readings, each against J4a's two tests. **Not one of
    /// them may be declared**, and this is the test rather than the claim.
    pub const THE_FOUR_MACHINE_READINGS: [(&'static str, ToldTests); 4] = [
        (
            "the listing (L1)",
            ToldTests {
                the_user_could_name_it: false,
                not_per_event_frame_derivative_file_or_byte: false,
            },
        ),
        (
            "the free-space figure (L11)",
            ToldTests {
                the_user_could_name_it: false,
                not_per_event_frame_derivative_file_or_byte: false,
            },
        ),
        (
            "the `.blend` prefix (L7)",
            ToldTests {
                the_user_could_name_it: false,
                not_per_event_frame_derivative_file_or_byte: false,
            },
        ),
        (
            "the permission answers (P1)",
            ToldTests {
                the_user_could_name_it: false,
                not_per_event_frame_derivative_file_or_byte: false,
            },
        ),
    ];
}

/// **Neither declared line composes prose at run time and neither carries a
/// mechanism's vocabulary** — no transfer identity, no staging path, no
/// protocol status, no `errno`, no rendered machine text (J3). **Each is the
/// declared form filled with the declared values, and nothing else.**
pub const NEITHER_LINE_COMPOSES_PROSE_AT_RUN_TIME: bool = true;

/// J2a's three tests as this module answers them for its two declared acts.
/// **Held as values so the declaration is checkable rather than asserted.**
pub fn the_tests_for(act: DeclaredAct) -> (ActTests, ErasingActsAreNotDeclared) {
    let tests = match act {
        DeclaredAct::AnUploadRequestEnded | DeclaredAct::AFileActCompleted => ActTests {
            smoozys_own: true,
            the_spec_names_its_completion_as_something_the_user_sees: true,
            at_the_users_own_rate: true,
        },
    };
    (
        tests,
        ErasingActsAreNotDeclared {
            // **Neither act's subject promises that nothing afterwards shows it
            // existed.** An upload's files stand in a folder and a delete's
            // absence stands in the listing; the erasing acts J2a's bar is
            // about — a cancel, a dropped job, a deleted entry or history —
            // are `Queue & History`'s and none of them is here.
            the_subjects_spec_promises_no_trace_afterwards: false,
        },
    )
}

/// The two acts this module declares.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclaredAct {
    AnUploadRequestEnded,
    AFileActCompleted,
}

impl DeclaredAct {
    pub const ALL: [DeclaredAct; 2] = [
        DeclaredAct::AnUploadRequestEnded,
        DeclaredAct::AFileActCompleted,
    ];
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_reporting::lines::Declarations;

    #[test]
    fn the_module_declares_act_lines_and_nothing_else() {
        assert_eq!(DECLARED, [LineKind::Act]);

        let mut declarations: Declarations<&str> = Declarations::new();
        declarations.module_declares("File Browser", DECLARED.to_vec());
        assert!(declarations.may_write(&"File Browser", LineKind::Act));
        for kind in LineKind::ALL {
            if kind != LineKind::Act {
                assert!(
                    !declarations.may_write(&"File Browser", kind),
                    "{kind:?} was writable on a declaration that does not carry it"
                );
            }
        }
    }

    /// **B3's line, filled**: the act's name from the closed set, the label,
    /// what it was about, the destination's absence written as one, how it
    /// ran and the three counts — and the identities, the telling's jobs
    /// among them.
    #[test]
    fn a_file_acts_line_is_the_fixed_form_and_carries_its_identities() {
        use smoozy_state::identities::IdentityKind;
        let issue = |kind| Identity::issue(kind).expect("randomness");
        let names = WhatTheFileActLineNames {
            run: issue(IdentityKind::Run),
            act: issue(IdentityKind::Operation),
            server_row: issue(IdentityKind::LocalServer),
            installation: Some(issue(IdentityKind::Installation)),
            jobs_the_telling_named: vec![issue(IdentityKind::Job)],
        };
        let rename = AFileActCompleted {
            act: FileAct::Rename,
            at: std::time::SystemTime::UNIX_EPOCH,
            server_row_label: "the big box".to_string(),
            about: WhatTheActWasAbout::OneThing("/home/ubuntu/kitchen/wood.png".to_string()),
            destination: None,
            ran: HowItRan::Ordinary,
            counts: TheThreeCounts { done: 1, refused: 0, not_attempted: 0 },
        };
        let line = rename.line(&names);
        assert_eq!(line.kind, LineKind::Act);
        assert_eq!(
            line.text,
            "rename\tthe big box\t/home/ubuntu/kitchen/wood.png\tno destination\tordinary\t1 done, 0 refused, 0 not attempted"
        );
        assert_eq!(line.concerning.len(), 5);
        assert!(line.concerning.contains(&names.jobs_the_telling_named[0]));
        let names: Vec<&str> = FileAct::ALL.iter().map(|act| act.name()).collect();
        assert_eq!(names, ["delete", "rename", "move", "copy", "new folder"]);
        let many = AFileActCompleted {
            act: FileAct::Delete,
            about: WhatTheActWasAbout::AFolderAndACount { folder: "/home/ubuntu/cache".to_string(), entries: 40 },
            counts: TheThreeCounts { done: 38, refused: 2, not_attempted: 0 },
            ..rename
        };
        assert!(many.line(&WhatTheFileActLineNames {
            run: issue(IdentityKind::Run),
            act: issue(IdentityKind::Operation),
            server_row: issue(IdentityKind::LocalServer),
            installation: None,
            jobs_the_telling_named: Vec::new(),
        })
        .text
        .starts_with("delete\tthe big box\t/home/ubuntu/cache, 40 entries\tno destination\tordinary\t38 done, 2 refused"));
    }

    #[test]
    fn a_module_that_declared_nothing_writes_nothing() {
        // `Desktop app and delivery` A7's hosting rule, which is the whole
        // reason this file exists.
        let declarations: Declarations<&str> = Declarations::new();
        for kind in LineKind::ALL {
            assert!(!declarations.may_write(&"File Browser", kind));
        }
    }

    #[test]
    fn both_declared_acts_pass_all_three_of_j2as_tests_and_the_bar() {
        for act in DeclaredAct::ALL {
            let (tests, bar) = the_tests_for(act);
            assert!(
                tests.is_diary_worthy(bar),
                "{act:?} is declared and fails a test"
            );
        }
        assert_eq!(DeclaredAct::ALL.len(), 2);
    }

    #[test]
    fn no_machine_reading_of_this_documents_may_be_a_told_line() {
        for (what, tests) in WhyThereAreNoToldLines::THE_FOUR_MACHINE_READINGS {
            assert!(
                !tests.may_be_declared(),
                "{what} passed J4a's tests and is not declared"
            );
        }
    }

    #[test]
    fn the_five_file_acts_are_closed_and_only_two_of_them_carry_a_destination() {
        assert_eq!(FileAct::ALL.len(), 5);
        let with_destination: Vec<FileAct> = FileAct::ALL
            .into_iter()
            .filter(|act| act.carries_a_destination())
            .collect();
        assert_eq!(with_destination, vec![FileAct::Move, FileAct::Copy]);
    }

    #[test]
    fn what_was_tested_and_not_declared_names_the_test_each_one_failed() {
        assert_eq!(TESTED_AND_NOT_DECLARED.len(), 6);
        for entry in TESTED_AND_NOT_DECLARED {
            assert!(
                (1..=3).contains(&entry.failed_test),
                "{} names no test of J2a's three",
                entry.what
            );
            assert!(!entry.why.is_empty());
        }
        const { assert!(READING_A_FILE_IS_NOT_THIS_DOCUMENTS_ACT) };
        const { assert!(N3S_ILLUSTRATION_IS_NOT_A_DECLARATION) };
        const { assert!(NEITHER_LINE_COMPOSES_PROSE_AT_RUN_TIME) };
    }

    #[test]
    fn a_multi_entry_act_is_one_line_over_a_folder_and_a_count() {
        // The third test in code: fifty entries is one act, so the line's
        // subject is the folder and the number rather than fifty paths.
        let about = WhatTheActWasAbout::AFolderAndACount {
            folder: "/jobs".to_string(),
            entries: 50,
        };
        assert!(matches!(
            about,
            WhatTheActWasAbout::AFolderAndACount { entries: 50, .. }
        ));
    }

    /// **The line's job field is the telling's own list and is composed
    /// nowhere else** (W8), and an act that was pushed past no warning names
    /// none.
    #[test]
    fn the_file_act_lines_job_field_is_taken_from_the_telling_and_never_composed_again() {
        use crate::warnings::{TheAct, TheTellingThatStandsFirst};
        use smoozy_state::identities::IdentityKind;

        let names = WhatTheFileActLineNames {
            run: Identity::issue(IdentityKind::Run).expect("randomness"),
            act: Identity::issue(IdentityKind::Operation).expect("randomness"),
            server_row: Identity::issue(IdentityKind::LocalServer).expect("randomness"),
            installation: None,
            jobs_the_telling_named: Vec::new(),
        };
        assert!(names.jobs_the_telling_named.is_empty());

        let one = Identity::issue(IdentityKind::Job).expect("randomness");
        let other = Identity::issue(IdentityKind::Job).expect("randomness");
        let telling = TheTellingThatStandsFirst {
            the_jobs: vec![one.clone(), other.clone()],
            the_act_intended: TheAct::ADelete,
        };
        let named = names.with_the_telling(&telling);
        assert_eq!(named.jobs_the_telling_named, vec![one, other]);
    }
}
