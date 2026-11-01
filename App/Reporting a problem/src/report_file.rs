//! The report file — `Reporting a problem` §5's head, parts and statements,
//! §7's name for it, and §11's own declaration of the act of writing one.
//!
//! **Pure rules again.** The caller hands over the diary as read back, what the
//! machine take brought (R3a's, over the ordinary connection, after the user
//! pressed *Save report*), the user's sentence and what the entry point
//! named. This composes the text and its name. The write — staged, published
//! under a name that never collides, re-checked by path — and the reveal are
//! the desktop's, under V6 to V8.
//!
//! S1: **a report is a plain-text file composed from the diary and from §6's
//! report-time material, and nothing else.** Its head carries the user's own
//! sentence, verbatim; where the report was opened from a failure, that
//! failure's naming line (X5); Smoozy's version and the operating system —
//! `Desktop app and delivery` E11's two values again, nothing more; the moment
//! of composition on the desktop clock, said to be that clock; and the
//! completeness statements of S5. **Every moment inside the body keeps the
//! clock its diary line carries.**
//!
//! S7: **never a quiet trim.** Every mechanism here that leaves anything out —
//! the middle drop, S4a's whole-part drops, the cap's eviction, an unreachable
//! machine, a read that did not complete — writes its statement into the file
//! itself, so the statement travels with the report wherever the user sends
//! it.

use std::time::{SystemTime, UNIX_EPOCH};

use smoozy_state::identities::Identity;
use smoozy_state::observed::Clock;

use crate::lines::{ActTests, ErasingActsAreNotDeclared, Line, LineKind};
use crate::report::{self, Slice, TheMiddleDrop, TheSlice, WhatTheMiddleDropCameTo, CEILING_BYTES};

/// Where the report was opened from, and what that entry point handed over.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenedFrom {
    /// X2: **the menu entry point hands the act nothing, and works with
    /// nothing.** It names no failure.
    TheMenu,
    /// X4 and X5: a failure's *Report a problem…*, naming the failure the user
    /// was looking at.
    AFailure(TheNamingLine),
    /// C3: *Report it…* after a crash. The last line and failure lines of the
    /// marked run supply the naming line where they exist.
    ACrash {
        the_naming_line: Option<TheNamingLine>,
    },
}

/// X5: **one line naming the failure — the presented sentence, the subject's
/// own label, the moment — drawn from the handover, never recomposed.** The
/// same three go into the file's head that go under the box's title, so the
/// Creator reads what the user was looking at when they pressed the control.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheNamingLine {
    /// The same string `Desktop app and delivery` M32 captured for the diary.
    pub the_presented_sentence: String,
    pub the_subjects_label: String,
    pub at: SystemTime,
    /// `State, identity and recovery` D5: the moment says its clock.
    pub clock: Clock,
}

/// What a report-time read came back with — one of S5's states for a part §6
/// takes. `DroppedForTheCeiling` is not here, because only the composition
/// below can decide it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Taken<T> {
    /// The part was taken whole (G3, G5: whole or not at all).
    Whole(T),
    /// *Absent*: the server was reached and there was nothing — a journal tail
    /// with no entries, no crash file. `why` is the take's own reason where it
    /// has one (G5a: a file that was not this attempt's is not taken, and the
    /// part reads absent with that reason stated).
    NothingThere { why: Option<String> },
    /// *Unreachable*: at the moment the report was cut, the server could not
    /// be reached, so the part was not taken.
    Unreachable,
    /// *Incomplete at the cut*: the read began and did not complete — the
    /// connection dropped during the take, or the request expired under
    /// `Server protocol and transfers` C19. What arrived, where anything did,
    /// is carried and named; **it is never written as included over a tail
    /// that is short, and never as absent over a note that exists.**
    BeganAndDidNotComplete { what_arrived: Option<T> },
}

impl<T> Taken<T> {
    fn content(&self) -> Option<&T> {
        match self {
            Taken::Whole(content) => Some(content),
            Taken::BeganAndDidNotComplete { what_arrived } => what_arrived.as_ref(),
            Taken::NothingThere { .. } | Taken::Unreachable => None,
        }
    }
}

/// G2: **the journal tail states, in the report, the sequence-number span it
/// holds and the journal's own oldest-retained number**, so the Creator reads
/// it as the window it is and never as a complete account. That is
/// `State, identity and recovery` E5's rule kept by statement where it cannot
/// be kept by cursor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheJournalTail {
    pub the_first_sequence_held: u64,
    pub the_last_sequence_held: u64,
    pub the_journals_oldest_retained: u64,
    /// The entries as read, whole. G4: nothing taken at report time is mined,
    /// trimmed of its own statements, or reworded.
    pub entries: String,
}

/// G5: Blender's own crash file, taken whole or not at all.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheCrashFile {
    /// **The file's own name carries the user's scene name** — the one
    /// user-derived string in the arrangement, and it travels with the take.
    pub name: String,
    pub text: String,
}

/// Which of a render's two captured streams an outcome is about.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheStream {
    StandardOutput,
    StandardError,
}

/// `Desktop app and delivery` M10's closed set of four, one per stream per
/// attempt. **Carried as recorded and never re-derived**, which is
/// `Desktop app and delivery` M11's rule honoured.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordedCollection {
    Collected,
    CollectedAndTheStreamWasEmpty,
    CollectionIncomplete,
    NeverEstablished,
}

impl RecordedCollection {
    /// S5: the recorded outcome, as the part's state. *Collected, and the
    /// stream was empty* is S5's own example of *absent*; *never established*
    /// is *never collected*, carried as such; and **collection incomplete is
    /// carried as its own statement and never collapses into printed
    /// nothing.**
    pub fn the_state(self) -> PartState {
        match self {
            RecordedCollection::Collected => PartState::Included,
            RecordedCollection::CollectedAndTheStreamWasEmpty => PartState::Absent,
            RecordedCollection::CollectionIncomplete => PartState::CollectionIncomplete,
            RecordedCollection::NeverEstablished => PartState::NeverCollected,
        }
    }

    /// The recorded outcome's own words, which is what the file carries.
    fn as_recorded(self) -> &'static str {
        match self {
            RecordedCollection::Collected => "collected",
            RecordedCollection::CollectedAndTheStreamWasEmpty => {
                "collected, and the stream was empty"
            }
            RecordedCollection::CollectionIncomplete => "collection incomplete",
            RecordedCollection::NeverEstablished => "never established",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ARecordedCollection {
    pub attempt: u32,
    pub stream: TheStream,
    pub outcome: RecordedCollection,
}

/// §6: what a reachable machine adds, for a report that names a job (G1).
/// Where the report names no job, there is none of this and no machine is
/// touched at all.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheMachineAdds {
    /// S5's *the machine's words among the diary's told lines*: the recorded
    /// collection of each captured stream. **The streams themselves are not
    /// taken** (G4) — they stay on the machine — so the file carries only how
    /// each collection was recorded.
    pub the_captured_output: Vec<ARecordedCollection>,
    pub the_journal_tail: Taken<TheJournalTail>,
    pub the_crash_file: Taken<TheCrashFile>,
    /// G3: **the note is taken whole, as the record it is.**
    pub the_per_job_note: Taken<String>,
}

/// Everything one report is composed from.
#[derive(Debug, Clone, Copy)]
pub struct WhatTheReportIsMadeFrom<'a> {
    /// V1: **the user's sentence goes into the file verbatim**, untouched by
    /// `Desktop app and delivery` X7's typography point, which is on the path
    /// to the screen and on no other path. No substitution, no correction, no
    /// trimming.
    pub the_users_sentence: &'a str,
    pub opened_from: &'a OpenedFrom,
    /// `Desktop app and delivery` E11's two values.
    pub smoozys_version: &'a str,
    pub the_system: &'a str,
    /// The moment of composition, on this computer's clock.
    pub made_at: SystemTime,
    /// The whole diary as the diary service read it back, oldest first.
    pub the_diary: &'a [Line],
    /// **Whether the diary service has ever evicted a segment** (R5). This is
    /// the one thing about the record its lines cannot say for themselves.
    pub the_cap_has_dropped_older_lines: bool,
    pub the_slice: &'a Slice,
    pub what_the_machine_adds: Option<&'a WhatTheMachineAdds>,
}

/// S5: the parts a report of its kind is made of.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Part {
    TheCapturedOutput { attempt: u32, stream: TheStream },
    TheJournalTail,
    ThePerJobNote,
    ThePreviousSession,
    TheCrashFile,
}

/// **S5's seven states**, one of which the file states for every part.
///
/// Six are S5's own bullets. The seventh is `Desktop app and delivery` M10's
/// *collection incomplete*, which S5 says is **carried as its own statement**
/// rather than folded into another. S6: **collapsing any two of them is the one
/// failure §5 exists to prevent**, because a reachable machine's empty stream
/// and an unreachable machine read identically without them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartState {
    Included,
    Absent,
    CollectionIncomplete,
    NeverCollected,
    Unreachable,
    IncompleteAtTheCut,
    /// S4a: the part was taken whole and did not fit, so it was dropped
    /// entire so the file could meet S3. **Dropped entire rather than
    /// trimmed.**
    DroppedForTheCeiling,
}

/// What the file states about one part.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Statement {
    Is(PartState),
    /// U7 and S5: **a previous run evicted by R5's cap is a previous run the
    /// report says it cannot include** — the cap's own statement, one of the
    /// two more S5 names of the same kind, standing where the part's state
    /// would.
    NotInTheRecordTheCapDroppedOlderLines,
}

/// The composed file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheReportFile {
    pub text: String,
    /// S4's middle drop, where one was made.
    pub the_middle_drop: Option<TheMiddleDrop>,
    /// S5: the statement made for every part, in the order the head gives
    /// them.
    pub statements: Vec<(Part, Statement)>,
    /// S5: where the cap has evicted lines a slice would have covered, the
    /// report states the moment its record begins and that older lines were
    /// dropped by the cap.
    pub says_the_cap_dropped_older_lines: bool,
    /// S3. **False only where the head alone is over the ceiling** — every
    /// diary line and every report-time part already gone. The head holds the
    /// user's own sentence, which V1 forbids trimming, so nothing is left to
    /// take.
    pub fits_the_ceiling: bool,
}

/// The report-time parts S4a may drop whole, **in its order: the journal tail,
/// then the crash file, then the per-job note.** The tail goes first because
/// it is a window of machine activity around the job and is the one part with
/// a fixed bound (G2); the crash file next, because it is unbounded and is one
/// ending's evidence; the note last, because it is the smallest and carries
/// the ending and the reason a render failed.
const S4A_ORDER: [Part; 3] = [
    Part::TheJournalTail,
    Part::TheCrashFile,
    Part::ThePerJobNote,
];

/// Compose the report.
///
/// **S4a, exactly.** S4's middle drop works on the slice's diary lines and on
/// nothing else, because the report-time parts are taken whole by rule and a
/// span inside one would trim it. Where every diary line has gone and the
/// file still exceeds S3, **then, and only then**, the first part in S4a's
/// order is dropped whole and states so — and the diary lines are cut again,
/// from nothing dropped, against the room that leaves, so a part dropped for
/// the ceiling never also costs diary lines the file had room for. Then the
/// next part, and so on.
pub fn compose(from: &WhatTheReportIsMadeFrom) -> TheReportFile {
    let slice = report::the_slice(from.the_diary, from.the_slice);
    let written: Vec<String> = slice
        .lines
        .iter()
        .map(|line| a_line_as_the_report_writes_it(line))
        .collect();
    let kinds: Vec<LineKind> = slice.lines.iter().map(|line| line.kind).collect();
    let sizes: Vec<u64> = written.iter().map(|line| line.len() as u64).collect();
    let says_the_cap_dropped_older_lines =
        from.the_cap_has_dropped_older_lines && slice.the_cap_may_have_taken_lines_from_it;

    let droppable: Vec<Part> = S4A_ORDER
        .into_iter()
        .filter(|part| {
            from.what_the_machine_adds.is_some_and(|adds| match part {
                Part::TheJournalTail => adds.the_journal_tail.content().is_some(),
                Part::TheCrashFile => adds.the_crash_file.content().is_some(),
                Part::ThePerJobNote => adds.the_per_job_note.content().is_some(),
                _ => false,
            })
        })
        .collect();

    for how_many in 0..=droppable.len() {
        let dropped_whole = &droppable[..how_many];
        let file = TheFile {
            from,
            slice: &slice,
            says_the_cap_dropped_older_lines,
            dropped_whole,
        };
        let around_the_lines =
            (file.head("").len() + DIARY_HEADING.len() + file.the_parts().len()) as u64;
        let came_to = report::drop_from_the_middle(&kinds, &sizes, |kept, drop| {
            around_the_lines
                + file.the_diary_line(drop).len() as u64
                + drop.map_or(0, |drop| {
                    the_line_at_the_cut(&file.the_cut_statement(drop)).len() as u64
                })
                + kept
                <= CEILING_BYTES
        });
        let drop = match came_to {
            WhatTheMiddleDropCameTo::NothingWasDropped => None,
            WhatTheMiddleDropCameTo::Dropped(drop) => Some(drop),
            WhatTheMiddleDropCameTo::NothingLeftToDrop(_) if how_many < droppable.len() => continue,
            // An empty slice has nothing to say it dropped.
            WhatTheMiddleDropCameTo::NothingLeftToDrop(drop) => {
                (drop.lines_dropped() > 0).then_some(drop)
            }
        };
        let text = file.whole(&written, drop.as_ref());
        return TheReportFile {
            fits_the_ceiling: text.len() as u64 <= CEILING_BYTES,
            text,
            statements: file.statements(),
            the_middle_drop: drop,
            says_the_cap_dropped_older_lines,
        };
    }
    unreachable!("the last pass returns whatever it came to")
}

/// One composition's fixed choices: which parts S4a has dropped whole.
struct TheFile<'a> {
    from: &'a WhatTheReportIsMadeFrom<'a>,
    slice: &'a TheSlice<'a>,
    says_the_cap_dropped_older_lines: bool,
    dropped_whole: &'a [Part],
}

const DIARY_HEADING: &str =
    "\n== App log ==\nEach time names its clock.\n";

const LEFT_OUT_WHOLE: &str = "omitted in full to keep this report under 5 MB";
const UNREACHABLE: &str = "unreachable: could not reach the server when this report was made";
const DID_NOT_FINISH: &str = "incomplete: the read did not finish";

impl TheFile<'_> {
    fn whole(&self, written: &[String], drop: Option<&TheMiddleDrop>) -> String {
        let mut text = self.head(&self.the_diary_line(drop));
        text.push_str(DIARY_HEADING);
        for (at, line) in written.iter().enumerate() {
            if let Some(drop) = drop {
                if at == drop.the_point_of_the_cut() {
                    text.push_str(&the_line_at_the_cut(&self.the_cut_statement(drop)));
                }
                if drop.dropped[at] {
                    continue;
                }
            }
            text.push_str(line);
        }
        if let Some(drop) = drop {
            if drop.the_point_of_the_cut() >= written.len() {
                text.push_str(&the_line_at_the_cut(&self.the_cut_statement(drop)));
            }
        }
        text.push_str(&self.the_parts());
        text
    }

    /// S1's head. `the_diary_line` is placed verbatim and nothing else in the
    /// head turns on it, so its length can be added to the rest's.
    fn head(&self, the_diary_line: &str) -> String {
        let from = self.from;
        let mut head =
            String::from("Smoozy problem report\n\nUser's description:\n");
        head.push_str(from.the_users_sentence);
        head.push_str("\n\n");
        match from.opened_from {
            OpenedFrom::TheMenu => head.push_str("Opened from: the Smoozy menu.\n"),
            OpenedFrom::AFailure(naming) => {
                head.push_str(&format!("Opened from: {}\n", the_naming_line(naming)));
            }
            OpenedFrom::ACrash {
                the_naming_line: naming,
            } => {
                head.push_str(
                    "Opened after a crash: the previous session ended unexpectedly.\n",
                );
                if let Some(naming) = naming {
                    head.push_str(&format!("Last recorded: {}\n", the_naming_line(naming)));
                }
            }
        }
        head.push_str(&format!("Smoozy version: {}\n", from.smoozys_version));
        head.push_str(&format!("System: {}\n", from.the_system));
        head.push_str(&format!(
            "Created: {}\n",
            the_moment_on_its_clock(from.made_at, &Clock::ThisComputer)
        ));
        head.push_str(&match from.the_slice {
            Slice::AFailedJob { job } => {
                format!(
                    "Scope: job {}, and every session it appears in.\n",
                    job.as_text()
                )
            }
            Slice::ACrash { run } => {
                format!(
                    "Scope: the crashed session, run {}.\n",
                    run.as_text()
                )
            }
            Slice::NothingInParticular { .. } => {
                "Scope: this session and the previous one.\n".to_string()
            }
        });
        head.push_str("\nContents:\n");
        head.push_str(the_diary_line);
        if self.says_the_cap_dropped_older_lines {
            head.push_str(&format!("- {}\n", self.the_cap_statement()));
        }
        for (part, statement) in self.statements() {
            head.push_str(&format!(
                "- {}: {}\n",
                the_parts_name(part),
                self.the_statement_in_words(part, statement)
            ));
        }
        if from.what_the_machine_adds.is_some() {
            head.push_str(
                "Captured output stays on the server; this report lists only how each \
                 stream was collected.\n",
            );
        }
        head
    }

    /// The head's line about the diary: what the slice's record covers, and
    /// the cut statement repeated (S4: *S1's head repeats it*).
    fn the_diary_line(&self, drop: Option<&TheMiddleDrop>) -> String {
        let lines = &self.slice.lines;
        match (lines.first(), lines.last(), drop) {
            (None, _, _) | (_, None, _) => "- App log: no lines.\n".to_string(),
            (Some(first), Some(last), None) => format!(
                "- App log: all {} lines, from {} to {}.\n",
                lines.len(),
                the_moment_on_its_clock(first.at, &first.clock),
                the_moment_on_its_clock(last.at, &last.clock),
            ),
            (Some(_), Some(_), Some(drop)) => format!(
                "- App log: {} of {} lines. {}\n",
                lines.len() - drop.lines_dropped(),
                lines.len(),
                self.the_cut_statement(drop)
            ),
        }
    }

    /// S4: **how many lines were dropped and the span they covered, by first
    /// and last retained moments.** The statement travels inside the file —
    /// the spec's one absolute, *never a quiet trim*.
    fn the_cut_statement(&self, drop: &TheMiddleDrop) -> String {
        let (before, after) = drop.the_retained_lines_either_side();
        let before = before.map_or_else(
            || "the start".to_string(),
            |at| the_moment_on_its_clock(self.slice.lines[at].at, &self.slice.lines[at].clock),
        );
        let after = after.map_or_else(
            || "the end".to_string(),
            |at| the_moment_on_its_clock(self.slice.lines[at].at, &self.slice.lines[at].clock),
        );
        if drop.statements_too {
            format!(
                "All {} routine lines and {} error and status lines between {before} and {after} \
                 were omitted to keep this report under 5 MB.",
                drop.ordinary_lines_dropped, drop.statement_lines_dropped,
            )
        } else {
            format!(
                "{} routine lines between {before} and {after} were omitted to keep this report \
                 under 5 MB. All error and status lines were kept.",
                drop.ordinary_lines_dropped,
            )
        }
    }

    /// S5's cap statement: the moment the record begins, and that older lines
    /// were dropped by the cap.
    fn the_cap_statement(&self) -> String {
        match self.from.the_diary.first() {
            Some(oldest) => format!(
                "The app log begins at {}; older lines were dropped by its 20 MB limit.",
                the_moment_on_its_clock(oldest.at, &oldest.clock)
            ),
            None => {
                "The app log is empty; older lines were dropped by its 20 MB limit.".to_string()
            }
        }
    }

    /// S5: one statement per part a report of its kind is made of.
    fn statements(&self) -> Vec<(Part, Statement)> {
        let mut out = Vec::new();
        if let Slice::NothingInParticular { .. } = self.from.the_slice {
            out.push((
                Part::ThePreviousSession,
                match (
                    &self.slice.the_run_before,
                    self.says_the_cap_dropped_older_lines,
                ) {
                    (Some(_), _) => Statement::Is(PartState::Included),
                    (None, true) => Statement::NotInTheRecordTheCapDroppedOlderLines,
                    (None, false) => Statement::Is(PartState::Absent),
                },
            ));
        }
        if let Some(adds) = self.from.what_the_machine_adds {
            if adds.the_captured_output.is_empty() {
                // `Desktop app and delivery` M11: *never established* rests on
                // the absence of a capture record, and no attempt left one.
                out.push((
                    Part::TheCapturedOutput {
                        attempt: 0,
                        stream: TheStream::StandardOutput,
                    },
                    Statement::Is(PartState::NeverCollected),
                ));
            }
            for recorded in &adds.the_captured_output {
                out.push((
                    Part::TheCapturedOutput {
                        attempt: recorded.attempt,
                        stream: recorded.stream,
                    },
                    Statement::Is(recorded.outcome.the_state()),
                ));
            }
            for part in [
                Part::TheJournalTail,
                Part::ThePerJobNote,
                Part::TheCrashFile,
            ] {
                out.push((part, Statement::Is(self.the_state_of(part, adds))));
            }
        }
        out
    }

    fn the_state_of(&self, part: Part, adds: &WhatTheMachineAdds) -> PartState {
        if self.dropped_whole.contains(&part) {
            return PartState::DroppedForTheCeiling;
        }
        match part {
            Part::TheJournalTail => the_state_of_a_take(&adds.the_journal_tail),
            Part::ThePerJobNote => the_state_of_a_take(&adds.the_per_job_note),
            Part::TheCrashFile => the_state_of_a_take(&adds.the_crash_file),
            Part::TheCapturedOutput { .. } | Part::ThePreviousSession => {
                unreachable!("these are stated from the record, not from the take")
            }
        }
    }

    fn the_statement_in_words(&self, part: Part, statement: Statement) -> String {
        let state = match statement {
            Statement::NotInTheRecordTheCapDroppedOlderLines => {
                return "not in the app log; older lines were dropped by its 20 MB limit".to_string();
            }
            Statement::Is(state) => state,
        };
        let adds = self.from.what_the_machine_adds;
        match part {
            Part::ThePreviousSession => match state {
                PartState::Included => "included".to_string(),
                _ => "absent: no previous session in the app log".to_string(),
            },
            Part::TheCapturedOutput { attempt, .. } => {
                let recorded = adds
                    .into_iter()
                    .flat_map(|adds| adds.the_captured_output.iter())
                    .find(|recorded| {
                        Part::TheCapturedOutput {
                            attempt: recorded.attempt,
                            stream: recorded.stream,
                        } == part
                    });
                match recorded {
                    // Carried as recorded, in the recorded outcome's own words.
                    Some(recorded) => recorded.outcome.as_recorded().to_string(),
                    None => {
                        debug_assert_eq!(attempt, 0);
                        "never established: no attempt recorded a collection".to_string()
                    }
                }
            }
            Part::TheJournalTail => {
                let tail = adds.map(|adds| &adds.the_journal_tail);
                let span = |tail: &TheJournalTail| {
                    format!(
                        "entries {} to {}; the log's oldest retained entry is {}",
                        tail.the_first_sequence_held,
                        tail.the_last_sequence_held,
                        tail.the_journals_oldest_retained
                    )
                };
                self.in_words(state, tail, span, "the server had no entries")
            }
            Part::ThePerJobNote => {
                let note = adds.map(|adds| &adds.the_per_job_note);
                self.in_words(
                    state,
                    note,
                    |_| String::new(),
                    "the server has no job record for this job",
                )
            }
            Part::TheCrashFile => {
                let file = adds.map(|adds| &adds.the_crash_file);
                // G5's third bound: **the absence of a crash file is never
                // evidence about the ending**, and a report that has none says
                // so rather than implying anything.
                let words = self.in_words(
                    state,
                    file,
                    |file| file.name.clone(),
                    "no crash file",
                );
                if state == PartState::Absent {
                    format!("{words}. This does not show how the render ended")
                } else {
                    words
                }
            }
        }
    }

    fn in_words<T>(
        &self,
        state: PartState,
        taken: Option<&Taken<T>>,
        what_it_holds: impl Fn(&T) -> String,
        nothing_there: &str,
    ) -> String {
        let holds = |content: &T| {
            let said = what_it_holds(content);
            if said.is_empty() {
                String::new()
            } else {
                format!(", {said}")
            }
        };
        match (state, taken) {
            (PartState::Included, Some(Taken::Whole(content))) => {
                format!("included{}", holds(content))
            }
            (PartState::Absent, Some(Taken::NothingThere { why })) => match why {
                Some(why) => format!("absent: {nothing_there} ({why})"),
                None => format!("absent: {nothing_there}"),
            },
            (PartState::Unreachable, _) => UNREACHABLE.to_string(),
            (
                PartState::IncompleteAtTheCut,
                Some(Taken::BeganAndDidNotComplete { what_arrived }),
            ) => match what_arrived {
                Some(content) => {
                    format!("{DID_NOT_FINISH}; what arrived is below{}", holds(content))
                }
                None => format!("{DID_NOT_FINISH}; nothing arrived"),
            },
            (PartState::DroppedForTheCeiling, Some(Taken::BeganAndDidNotComplete { .. })) => {
                format!("{LEFT_OUT_WHOLE}. Only part of it had been received")
            }
            (PartState::DroppedForTheCeiling, _) => LEFT_OUT_WHOLE.to_string(),
            _ => unreachable!("a state is only ever read off the take it describes"),
        }
    }

    /// The report-time parts, each whole under its own heading, in the order
    /// the head lists them. A part dropped for the ceiling, absent or
    /// unreachable has no section: its statement in the head is all there is.
    fn the_parts(&self) -> String {
        let mut text = String::new();
        let Some(adds) = self.from.what_the_machine_adds else {
            return text;
        };
        let unfinished = |incomplete: bool| {
            if incomplete {
                " (incomplete: the read did not finish)"
            } else {
                ""
            }
        };
        if !self.dropped_whole.contains(&Part::TheJournalTail) {
            if let Some(tail) = adds.the_journal_tail.content() {
                text.push_str(&format!(
                    "\n== Server event log, entries {} to {}{} ==\nTimes use the server's clock. \
                     The log's oldest retained entry is {}.\n",
                    tail.the_first_sequence_held,
                    tail.the_last_sequence_held,
                    unfinished(!matches!(adds.the_journal_tail, Taken::Whole(_))),
                    tail.the_journals_oldest_retained,
                ));
                text.push_str(&ended(&tail.entries));
            }
        }
        if !self.dropped_whole.contains(&Part::ThePerJobNote) {
            if let Some(note) = adds.the_per_job_note.content() {
                text.push_str(&format!(
                    "\n== Server job record{} ==\nTimes use the server's clock.\n",
                    unfinished(!matches!(adds.the_per_job_note, Taken::Whole(_))),
                ));
                text.push_str(&ended(note));
            }
        }
        if !self.dropped_whole.contains(&Part::TheCrashFile) {
            if let Some(file) = adds.the_crash_file.content() {
                text.push_str(&format!(
                    "\n== Blender's crash file, {}{} ==\n",
                    file.name,
                    unfinished(!matches!(adds.the_crash_file, Taken::Whole(_))),
                ));
                text.push_str(&ended(&file.text));
            }
        }
        text
    }
}

/// S5: a take's state, as the take reported it.
fn the_state_of_a_take<T>(taken: &Taken<T>) -> PartState {
    match taken {
        Taken::Whole(_) => PartState::Included,
        Taken::NothingThere { .. } => PartState::Absent,
        Taken::Unreachable => PartState::Unreachable,
        Taken::BeganAndDidNotComplete { .. } => PartState::IncompleteAtTheCut,
    }
}

/// A part's text as taken, whole. A missing last newline is added after it
/// rather than inside it, so the next heading starts on its own line and not
/// one byte of the part is changed.
fn ended(text: &str) -> String {
    if text.is_empty() || text.ends_with('\n') {
        text.to_string()
    } else {
        format!("{text}\n")
    }
}

fn the_line_at_the_cut(statement: &str) -> String {
    format!("--- {statement} ---\n")
}

fn the_parts_name(part: Part) -> String {
    match part {
        Part::ThePreviousSession => "Previous session".to_string(),
        Part::TheCapturedOutput { attempt: 0, .. } => "Captured output".to_string(),
        Part::TheCapturedOutput { attempt, stream } => format!(
            "Captured output, attempt {attempt}, {}",
            match stream {
                TheStream::StandardOutput => "standard output",
                TheStream::StandardError => "standard error",
            }
        ),
        Part::TheJournalTail => "Server event log".to_string(),
        Part::ThePerJobNote => "Server job record".to_string(),
        Part::TheCrashFile => "Blender's crash file".to_string(),
    }
}

fn the_naming_line(naming: &TheNamingLine) -> String {
    format!(
        "{} \u{b7} {} \u{b7} {}",
        naming.the_presented_sentence,
        naming.the_subjects_label,
        the_moment_on_its_clock(naming.at, &naming.clock)
    )
}

/// One diary line as the report writes it: the moment and the clock it is on,
/// the kind, the run, what it concerns, and the declared text. **Nothing in it
/// is reworded** — the text is the line's own, and a failure line's is the
/// sentence the user met (J6).
pub fn a_line_as_the_report_writes_it(line: &Line) -> String {
    let mut written = format!(
        "{} [{}] {:?} {:?}",
        the_moment_in_utc(line.at),
        match &line.clock {
            Clock::ThisComputer => "this computer".to_string(),
            Clock::AMachine(machine) => format!("server {}", machine.0),
        },
        line.kind,
        line.run,
    );
    for identity in &line.concerning {
        written.push_str(&format!(" {identity:?}"));
    }
    written.push_str(": ");
    written.push_str(&line.text);
    written.push('\n');
    written
}

/// A moment and the clock it is on, in words — `State, identity and recovery`
/// D5: no moment is written without its clock.
pub fn the_moment_on_its_clock(at: SystemTime, clock: &Clock) -> String {
    match clock {
        Clock::ThisComputer => format!("{} on this computer's clock", the_moment_in_utc(at)),
        Clock::AMachine(machine) => {
            format!("{} on server {}'s clock", the_moment_in_utc(at), machine.0)
        }
    }
}

/// A moment as a complete UTC date and time. **UTC, and said**, because a
/// report is read on another computer in another zone, and a moment without
/// its zone could mean any of them.
pub fn the_moment_in_utc(at: SystemTime) -> String {
    let civil = Civil::of(seconds_since_the_epoch(at));
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02} UTC",
        civil.year, civil.month, civil.day, civil.hour, civil.minute, civil.second
    )
}

fn seconds_since_the_epoch(at: SystemTime) -> i64 {
    match at.duration_since(UNIX_EPOCH) {
        Ok(since) => since.as_secs() as i64,
        Err(before) => -(before.duration().as_secs() as i64),
    }
}

/// A civil date and time from seconds since the epoch, by the proleptic
/// Gregorian arithmetic.
struct Civil {
    year: i64,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
}

impl Civil {
    fn of(seconds: i64) -> Civil {
        let (days, of_day) = (seconds.div_euclid(86_400), seconds.rem_euclid(86_400));
        let z = days + 719_468;
        let era = z.div_euclid(146_097);
        let doe = z.rem_euclid(146_097);
        let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let day = doy - (153 * mp + 2) / 5 + 1;
        let month = if mp < 10 { mp + 3 } else { mp - 9 };
        Civil {
            year: yoe + era * 400 + i64::from(month <= 2),
            month: month as u32,
            day: day as u32,
            hour: (of_day / 3600) as u32,
            minute: (of_day % 3600 / 60) as u32,
            second: (of_day % 60) as u32,
        }
    }
}

/// V3: **the name is Smoozy's own composition: a fixed stem, a moment, and the
/// plain-text extension.** The drawn form is `smoozy-report-25-aug-14-32.txt`:
/// the day of the month, the month's three-letter name in lower case, and the
/// hour and minute on a 24-hour clock, joined by hyphens — the day written
/// without a leading zero, as the box's naming line writes *8 Aug*.
///
/// **The moment is the desktop clock's at the save, and that is a presentation
/// use** — `State, identity and recovery` D5's own allowance — because
/// nothing rests on the name. So it is this computer's local time, as the user
/// reads the clock on their screen: the caller hands over the moment and the
/// zone's offset from UTC at that moment, in seconds east.
///
/// `the_try` is V5's: **a taken name is never overwritten and never prompted
/// over**, so on *taken* the name is composed again with a non-colliding
/// suffix and judged again. The first try has no suffix; the second is
/// `-2`, the third `-3`. A second report in the same minute lands beside the
/// first, whole, under its own name.
pub fn the_file_name(at: SystemTime, utc_offset_seconds: i32, the_try: u32) -> String {
    const MONTHS: [&str; 12] = [
        "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec",
    ];
    let civil = Civil::of(seconds_since_the_epoch(at) + i64::from(utc_offset_seconds));
    let suffix = if the_try > 1 {
        format!("-{the_try}")
    } else {
        String::new()
    };
    format!(
        "smoozy-report-{}-{}-{:02}-{:02}{suffix}.txt",
        civil.day,
        MONTHS[(civil.month - 1) as usize],
        civil.hour,
        civil.minute
    )
}

/// §11's own declaration: **the act, a report written.** It belongs to no
/// feature module — its two entry points are `Desktop app and delivery`'s menu
/// item and failure plumbing — so the owning document declares it, and that
/// is this one.
///
/// **It passes J2a's three tests**: it is Smoozy's own act, the spec names the
/// written file as what the user gets, and it happens once per press of *Save
/// the report*.
pub const A_REPORT_WRITTEN: ActTests = ActTests {
    smoozys_own: true,
    the_spec_names_its_completion_as_something_the_user_sees: true,
    at_the_users_own_rate: true,
};

/// And J2a's bar does not reach it: nothing in the spec promises that a report
/// leaves no trace. **V9: the act of writing one is an act line in the diary.**
pub const A_REPORT_WRITTEN_IS_NO_ERASURE: ErasingActsAreNotDeclared = ErasingActsAreNotDeclared {
    the_subjects_spec_promises_no_trace_afterwards: false,
};

/// §11: the *report written* line, in its fixed form.
///
/// - **The fixed line form:** the act's name; the moment on the desktop
///   clock, stated as that clock; the folder the user chose (N3 admits it
///   deliberately); and the file's composed name (V3).
/// - **The identities it carries:** the run identity of the run that wrote it
///   (U7); and, where the report was opened from a failure entry point, the
///   condition identity or subject outcome mark and the durable subject the
///   handover carried (X4). **The menu entry point carries none of the second
///   group**, and their absence is the line's own reading rather than a gap.
/// - **What the line does not carry:** the user's own sentence, which belongs
///   in the report and not in the record the report was cut from; the slice's
///   shape, its size, or what it could not include, all of which the file
///   states about itself; and no second line for the reveal, which V8 rules is
///   the pointing rather than the act.
///
/// **This plan writes no told lines**, declared at §11 so the absence is a
/// decision: G1's take is report-time material and goes into the file rather
/// than into the diary.
pub fn the_report_written_line(
    run: &Identity,
    at: SystemTime,
    the_folder: &str,
    the_file_name: &str,
    from_the_failure: &[Identity],
) -> Line {
    Line {
        run: run.clone(),
        kind: LineKind::Act,
        at,
        clock: Clock::ThisComputer,
        text: format!("Report written: {the_file_name} in {the_folder}"),
        concerning: from_the_failure.to_vec(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::report::tests::{a_line, an_identity};
    use smoozy_state::identities::IdentityKind;
    use smoozy_state::observed::MachineClock;
    use std::time::Duration;

    const MADE_AT_SECONDS: u64 = 1_756_132_320; // 2025-08-25 14:32:00 UTC

    fn made_at() -> SystemTime {
        UNIX_EPOCH + Duration::from_secs(MADE_AT_SECONDS)
    }

    fn a_naming_line() -> TheNamingLine {
        TheNamingLine {
            the_presented_sentence: "Kitchen \u{2014} final did not finish".to_string(),
            the_subjects_label: "RunPod 4090".to_string(),
            at: UNIX_EPOCH + Duration::from_secs(1_754_694_840),
            clock: Clock::ThisComputer,
        }
    }

    fn a_tail() -> TheJournalTail {
        TheJournalTail {
            the_first_sequence_held: 4100,
            the_last_sequence_held: 4611,
            the_journals_oldest_retained: 7,
            entries: "4100 frame 12 finished\n4611 job ended\n".to_string(),
        }
    }

    fn a_crash_file() -> TheCrashFile {
        TheCrashFile {
            name: "heavy.crash.txt".to_string(),
            text: "# Blender 4.5.3\nbacktrace\n".to_string(),
        }
    }

    fn everything_taken() -> WhatTheMachineAdds {
        WhatTheMachineAdds {
            the_captured_output: vec![
                ARecordedCollection {
                    attempt: 1,
                    stream: TheStream::StandardOutput,
                    outcome: RecordedCollection::Collected,
                },
                ARecordedCollection {
                    attempt: 1,
                    stream: TheStream::StandardError,
                    outcome: RecordedCollection::CollectedAndTheStreamWasEmpty,
                },
            ],
            the_journal_tail: Taken::Whole(a_tail()),
            the_crash_file: Taken::Whole(a_crash_file()),
            the_per_job_note: Taken::Whole("ended: failed\nreason: out of memory".to_string()),
        }
    }

    /// Two runs, the second the current one, the job in the first.
    fn a_diary() -> (Vec<Line>, Identity, Identity, Identity) {
        let before = an_identity(IdentityKind::Run);
        let now = an_identity(IdentityKind::Run);
        let job = an_identity(IdentityKind::Job);
        let mut told = a_line(&before, LineKind::Told, 3, &[&job]);
        told.clock = Clock::AMachine(MachineClock("server-1".to_string()));
        let diary = vec![
            a_line(&before, LineKind::Session, 0, &[]),
            a_line(&before, LineKind::Act, 1, &[&job]),
            a_line(&before, LineKind::Failure, 2, &[&job]),
            told,
            a_line(&now, LineKind::Session, 10, &[]),
            a_line(&now, LineKind::Act, 11, &[]),
        ];
        (diary, before, now, job)
    }

    fn a_report<'a>(
        diary: &'a [Line],
        slice: &'a Slice,
        opened_from: &'a OpenedFrom,
        adds: Option<&'a WhatTheMachineAdds>,
    ) -> WhatTheReportIsMadeFrom<'a> {
        WhatTheReportIsMadeFrom {
            the_users_sentence:
                "The frames aren\u{2019}t on the drive but Smoozy says they arrived.",
            opened_from,
            smoozys_version: "2.4.1",
            the_system: "macOS",
            made_at: made_at(),
            the_diary: diary,
            the_cap_has_dropped_older_lines: false,
            the_slice: slice,
            what_the_machine_adds: adds,
        }
    }

    fn the_head(text: &str) -> &str {
        &text[..text
            .find("\n== App log ==")
            .expect("the log section is there")]
    }

    #[test]
    fn the_head_carries_what_s1_lists_and_nothing_else_about_the_user() {
        let (diary, _, now, _) = a_diary();
        let slice = Slice::NothingInParticular { this_run: now };
        let report = compose(&a_report(&diary, &slice, &OpenedFrom::TheMenu, None));
        let head = the_head(&report.text);
        assert!(head.starts_with("Smoozy problem report\n"));
        assert!(
            head.contains("The frames aren\u{2019}t on the drive but Smoozy says they arrived.\n")
        );
        assert!(head.contains("Smoozy version: 2.4.1\n"));
        assert!(head.contains("System: macOS\n"));
        assert!(head.contains("Created: 2025-08-25 14:32:00 UTC on this computer's clock\n"));
        assert!(head.contains("Opened from: the Smoozy menu.\n"));
        assert!(head.contains("Scope: this session and the previous one.\n"));
        assert!(head.contains("\nContents:\n"));
    }

    #[test]
    fn the_users_sentence_goes_in_verbatim_whatever_it_holds() {
        let (diary, _, now, _) = a_diary();
        let slice = Slice::NothingInParticular { this_run: now };
        let opened = OpenedFrom::TheMenu;
        let mut from = a_report(&diary, &slice, &opened, None);
        // Straight quotes the typography point would curl, a double space, a
        // tab and a line break: none of it is touched.
        let sentence = "It said \"done\" -- but  the\tfolder's empty...\nTwice.";
        from.the_users_sentence = sentence;
        let report = compose(&from);
        assert!(report
            .text
            .contains(&format!("User's description:\n{sentence}\n")));
    }

    #[test]
    fn a_report_opened_from_a_failure_names_it_in_the_head() {
        let (diary, _, now, _) = a_diary();
        let slice = Slice::NothingInParticular { this_run: now };
        let opened = OpenedFrom::AFailure(a_naming_line());
        let report = compose(&a_report(&diary, &slice, &opened, None));
        assert!(the_head(&report.text).contains(
            "Opened from: Kitchen \u{2014} final did not finish \u{b7} RunPod 4090 \u{b7} \
             2025-08-08 23:14:00 UTC on this computer's clock\n"
        ));
        assert!(!report.text.contains("Smoozy menu"));
    }

    #[test]
    fn a_crash_report_says_so_and_carries_its_naming_line_where_there_is_one() {
        let (diary, before, _, _) = a_diary();
        let slice = Slice::ACrash {
            run: before.clone(),
        };
        let opened = OpenedFrom::ACrash {
            the_naming_line: Some(a_naming_line()),
        };
        let report = compose(&a_report(&diary, &slice, &opened, None));
        let head = the_head(&report.text);
        assert!(head
            .contains("Opened after a crash: the previous session ended unexpectedly.\n"));
        assert!(head.contains("Last recorded: Kitchen"));
        assert!(head.contains(&format!("run {}", before.as_text())));
        // The crash slice is the dead run's lines and no other run's.
        assert!(report.text.contains(&format!("{before:?}")));
        assert_eq!(report.text.matches("] Session ").count(), 1);

        let opened = OpenedFrom::ACrash {
            the_naming_line: None,
        };
        let report = compose(&a_report(&diary, &slice, &opened, None));
        assert!(!report.text.contains("Last recorded:"));
    }

    #[test]
    fn every_moment_in_the_body_keeps_its_own_clock() {
        let (diary, _, _, job) = a_diary();
        let slice = Slice::AFailedJob { job };
        let opened = OpenedFrom::AFailure(a_naming_line());
        let report = compose(&a_report(&diary, &slice, &opened, None));
        assert!(report
            .text
            .contains("2025-08-25 13:53:23 UTC [server server-1] Told "));
        assert!(report
            .text
            .contains("2025-08-25 13:53:20 UTC [this computer] Session "));
    }

    #[test]
    fn the_report_holds_the_diary_lines_as_written_and_reworded_nowhere() {
        let (diary, _, now, _) = a_diary();
        let slice = Slice::NothingInParticular { this_run: now };
        let report = compose(&a_report(&diary, &slice, &OpenedFrom::TheMenu, None));
        for line in &diary {
            assert!(report.text.contains(&a_line_as_the_report_writes_it(line)));
        }
        assert!(the_head(&report.text).contains("- App log: all 6 lines, from "));
        assert_eq!(report.the_middle_drop, None);
        assert!(report.fits_the_ceiling);
    }

    #[test]
    fn the_previous_session_is_stated_three_ways() {
        let (diary, _, now, _) = a_diary();
        let slice = Slice::NothingInParticular {
            this_run: now.clone(),
        };
        let report = compose(&a_report(&diary, &slice, &OpenedFrom::TheMenu, None));
        assert_eq!(
            report.statements,
            vec![(Part::ThePreviousSession, Statement::Is(PartState::Included))]
        );
        assert!(report
            .text
            .contains("- Previous session: included\n"));

        // A first run on a fresh record: absent, and no claim about the cap.
        let only = &diary[4..];
        let report = compose(&a_report(only, &slice, &OpenedFrom::TheMenu, None));
        assert_eq!(
            report.statements,
            vec![(Part::ThePreviousSession, Statement::Is(PartState::Absent))]
        );
        assert!(report
            .text
            .contains("absent: no previous session in the app log"));
        assert!(!report.says_the_cap_dropped_older_lines);

        // The same record after the cap evicted the run before: the cap's own
        // statement, with the moment the record begins.
        let opened = OpenedFrom::TheMenu;
        let mut from = a_report(only, &slice, &opened, None);
        from.the_cap_has_dropped_older_lines = true;
        let report = compose(&from);
        assert_eq!(
            report.statements,
            vec![(
                Part::ThePreviousSession,
                Statement::NotInTheRecordTheCapDroppedOlderLines
            )]
        );
        assert!(report.says_the_cap_dropped_older_lines);
        assert!(report.text.contains(
            "- The app log begins at 2025-08-25 13:53:30 UTC on this computer's clock; older lines were \
             dropped by its 20 MB limit.\n"
        ));
        assert!(report
            .text
            .contains("- Previous session: not in the app log; older lines were dropped"));
    }

    #[test]
    fn the_cap_is_not_blamed_for_a_slice_it_did_not_reach() {
        let (diary, _, now, _) = a_diary();
        let slice = Slice::ACrash { run: now };
        let opened = OpenedFrom::ACrash {
            the_naming_line: None,
        };
        let mut from = a_report(&diary, &slice, &opened, None);
        from.the_cap_has_dropped_older_lines = true;
        let report = compose(&from);
        assert!(!report.says_the_cap_dropped_older_lines);
        assert!(!report.text.contains("20 MB"));
    }

    fn the_statements_of(adds: &WhatTheMachineAdds) -> (TheReportFile, Vec<(Part, Statement)>) {
        let (diary, _, _, job) = a_diary();
        let slice = Slice::AFailedJob { job };
        let opened = OpenedFrom::AFailure(a_naming_line());
        let report = compose(&a_report(&diary, &slice, &opened, Some(adds)));
        let statements = report.statements.clone();
        (report, statements)
    }

    fn the_line_for<'a>(report: &'a TheReportFile, name: &str) -> &'a str {
        the_head(&report.text)
            .lines()
            .find(|line| line.starts_with(&format!("- {name}: ")))
            .unwrap_or_else(|| panic!("no line for {name}"))
    }

    #[test]
    fn every_part_taken_whole_is_included_and_its_section_is_there_unchanged() {
        let adds = everything_taken();
        let (report, statements) = the_statements_of(&adds);
        assert_eq!(
            statements,
            vec![
                (
                    Part::TheCapturedOutput {
                        attempt: 1,
                        stream: TheStream::StandardOutput
                    },
                    Statement::Is(PartState::Included)
                ),
                (
                    Part::TheCapturedOutput {
                        attempt: 1,
                        stream: TheStream::StandardError
                    },
                    Statement::Is(PartState::Absent)
                ),
                (Part::TheJournalTail, Statement::Is(PartState::Included)),
                (Part::ThePerJobNote, Statement::Is(PartState::Included)),
                (Part::TheCrashFile, Statement::Is(PartState::Included)),
            ]
        );
        assert_eq!(
            the_line_for(&report, "Server event log"),
            "- Server event log: included, entries 4100 to 4611; the log's oldest retained entry is 7"
        );
        assert_eq!(
            the_line_for(&report, "Blender's crash file"),
            "- Blender's crash file: included, heavy.crash.txt"
        );
        assert!(report
            .text
            .contains("== Server event log, entries 4100 to 4611 ==\n"));
        assert!(report.text.contains(&a_tail().entries));
        // The note had no last newline; its text is kept byte for byte.
        assert!(report
            .text
            .contains("ended: failed\nreason: out of memory\n"));
        assert!(report
            .text
            .contains("== Blender's crash file, heavy.crash.txt ==\n# Blender 4.5.3\nbacktrace\n"));
        assert!(report
            .text
            .contains("Times use the server's clock."));
    }

    #[test]
    fn the_captured_output_is_carried_in_its_recorded_words() {
        let mut adds = everything_taken();
        adds.the_captured_output = [
            RecordedCollection::Collected,
            RecordedCollection::CollectedAndTheStreamWasEmpty,
            RecordedCollection::CollectionIncomplete,
            RecordedCollection::NeverEstablished,
        ]
        .into_iter()
        .enumerate()
        .map(|(at, outcome)| ARecordedCollection {
            attempt: at as u32 + 1,
            stream: TheStream::StandardError,
            outcome,
        })
        .collect();
        let (report, statements) = the_statements_of(&adds);
        let states: Vec<Statement> = statements
            .iter()
            .take(4)
            .map(|(_, statement)| *statement)
            .collect();
        assert_eq!(
            states,
            vec![
                Statement::Is(PartState::Included),
                Statement::Is(PartState::Absent),
                Statement::Is(PartState::CollectionIncomplete),
                Statement::Is(PartState::NeverCollected),
            ]
        );
        for (attempt, words) in [
            (1, "collected"),
            (2, "collected, and the stream was empty"),
            (3, "collection incomplete"),
            (4, "never established"),
        ] {
            assert_eq!(
                the_line_for(
                    &report,
                    &format!("Captured output, attempt {attempt}, standard error")
                ),
                format!("- Captured output, attempt {attempt}, standard error: {words}")
            );
        }
        assert!(report
            .text
            .contains("Captured output stays on the server"));
    }

    #[test]
    fn a_job_with_no_capture_record_reads_never_established() {
        let mut adds = everything_taken();
        adds.the_captured_output.clear();
        let (report, statements) = the_statements_of(&adds);
        assert_eq!(statements[0].1, Statement::Is(PartState::NeverCollected));
        assert_eq!(
            the_line_for(&report, "Captured output"),
            "- Captured output: never established: no attempt recorded a collection"
        );
    }

    #[test]
    fn an_unreachable_server_is_stated_per_part_and_never_as_absent() {
        let adds = WhatTheMachineAdds {
            the_captured_output: everything_taken().the_captured_output,
            the_journal_tail: Taken::Unreachable,
            the_crash_file: Taken::Unreachable,
            the_per_job_note: Taken::Unreachable,
        };
        let (report, statements) = the_statements_of(&adds);
        for part in [
            Part::TheJournalTail,
            Part::ThePerJobNote,
            Part::TheCrashFile,
        ] {
            assert!(statements.contains(&(part, Statement::Is(PartState::Unreachable))));
        }
        for name in [
            "Server event log",
            "Server job record",
            "Blender's crash file",
        ] {
            assert_eq!(
                the_line_for(&report, name),
                format!("- {name}: {UNREACHABLE}")
            );
        }
        assert!(!report.text.contains("== Server "));
        assert!(!report.text.contains("absent"));
    }

    #[test]
    fn nothing_there_is_absent_and_a_missing_crash_file_claims_nothing_about_the_ending() {
        let adds = WhatTheMachineAdds {
            the_captured_output: everything_taken().the_captured_output,
            the_journal_tail: Taken::NothingThere { why: None },
            the_crash_file: Taken::NothingThere {
                why: Some("the file there was written before this attempt started".to_string()),
            },
            the_per_job_note: Taken::NothingThere { why: None },
        };
        let (report, statements) = the_statements_of(&adds);
        for part in [
            Part::TheJournalTail,
            Part::ThePerJobNote,
            Part::TheCrashFile,
        ] {
            assert!(statements.contains(&(part, Statement::Is(PartState::Absent))));
        }
        assert_eq!(
            the_line_for(&report, "Server event log"),
            "- Server event log: absent: the server had no entries"
        );
        assert_eq!(
            the_line_for(&report, "Blender's crash file"),
            "- Blender's crash file: absent: no crash file (the file there was written before \
             this attempt started). This does not show how the render ended"
        );
    }

    #[test]
    fn a_read_that_did_not_finish_is_incomplete_and_names_what_arrived() {
        let mut short = a_tail();
        short.the_last_sequence_held = 4200;
        let adds = WhatTheMachineAdds {
            the_captured_output: everything_taken().the_captured_output,
            the_journal_tail: Taken::BeganAndDidNotComplete {
                what_arrived: Some(short),
            },
            the_crash_file: Taken::Whole(a_crash_file()),
            the_per_job_note: Taken::BeganAndDidNotComplete { what_arrived: None },
        };
        let (report, statements) = the_statements_of(&adds);
        assert!(statements.contains(&(
            Part::TheJournalTail,
            Statement::Is(PartState::IncompleteAtTheCut)
        )));
        assert!(statements.contains(&(
            Part::ThePerJobNote,
            Statement::Is(PartState::IncompleteAtTheCut)
        )));
        assert_eq!(
            the_line_for(&report, "Server event log"),
            format!("- Server event log: {DID_NOT_FINISH}; what arrived is below, entries 4100 to 4200; the log's oldest retained entry is 7")
        );
        assert_eq!(
            the_line_for(&report, "Server job record"),
            format!("- Server job record: {DID_NOT_FINISH}; nothing arrived")
        );
        assert!(report.text.contains("== Server event log, entries 4100 to 4200 (incomplete: the read did not finish) =="));
        assert!(!report.text.contains("== Server job record"));
    }

    #[test]
    fn the_seven_states_are_seven_different_sentences() {
        // S6: collapsing any two of them is the failure §5 exists to prevent.
        let mut words: Vec<String> = Vec::new();
        let adds_for = |tail: Taken<TheJournalTail>| WhatTheMachineAdds {
            the_captured_output: vec![],
            the_journal_tail: tail,
            the_crash_file: Taken::NothingThere { why: None },
            the_per_job_note: Taken::NothingThere { why: None },
        };
        for tail in [
            Taken::Whole(a_tail()),
            Taken::NothingThere { why: None },
            Taken::Unreachable,
            Taken::BeganAndDidNotComplete { what_arrived: None },
        ] {
            let adds = adds_for(tail);
            let (report, _) = the_statements_of(&adds);
            words.push(the_line_for(&report, "Server event log").to_string());
        }
        for outcome in [
            RecordedCollection::CollectionIncomplete,
            RecordedCollection::NeverEstablished,
        ] {
            let mut adds = adds_for(Taken::Unreachable);
            adds.the_captured_output = vec![ARecordedCollection {
                attempt: 1,
                stream: TheStream::StandardOutput,
                outcome,
            }];
            let (report, _) = the_statements_of(&adds);
            words.push(
                the_line_for(&report, "Captured output, attempt 1, standard output").to_string(),
            );
        }
        words.push(LEFT_OUT_WHOLE.to_string());
        let after_the_name: Vec<&str> = words
            .iter()
            .map(|line| {
                line.split_once(": ")
                    .map_or(line.as_str(), |(_, said)| said)
            })
            .collect();
        let distinct: std::collections::HashSet<&&str> = after_the_name.iter().collect();
        assert_eq!(distinct.len(), 7, "{after_the_name:#?}");
    }

    /// A diary large enough that the ceiling bites: one long run around the
    /// job, statement lines sprinkled through it.
    fn a_large_diary(lines: usize) -> (Vec<Line>, Identity) {
        let run = an_identity(IdentityKind::Run);
        let job = an_identity(IdentityKind::Job);
        let mut diary = vec![a_line(&run, LineKind::Session, 0, &[])];
        for at in 1..lines {
            let kind = if at % 1000 == 0 {
                LineKind::Failure
            } else {
                LineKind::Act
            };
            let mut line = a_line(&run, kind, at as u64, &[&job]);
            line.text = format!("{kind:?} line {at} {}", "x".repeat(80));
            diary.push(line);
        }
        diary.push(a_line(&run, LineKind::Last, lines as u64, &[]));
        (diary, job)
    }

    #[test]
    fn a_slice_over_the_ceiling_is_cut_from_the_middle_and_the_file_fits() {
        let (diary, job) = a_large_diary(80_000);
        let slice = Slice::AFailedJob { job };
        let opened = OpenedFrom::AFailure(a_naming_line());
        let adds = everything_taken();
        let report = compose(&a_report(&diary, &slice, &opened, Some(&adds)));
        assert!(report.fits_the_ceiling);
        assert!(report.text.len() as u64 <= CEILING_BYTES);
        // And not by much: the span stopped at the first width that fitted.
        assert!(report.text.len() as u64 > CEILING_BYTES - 1000);
        let drop = report
            .the_middle_drop
            .clone()
            .expect("the middle was dropped");
        assert!(!drop.statements_too);
        // Every statement line survived, and so did the head and tail.
        for line in diary
            .iter()
            .filter(|line| report::states_something(line.kind))
        {
            assert!(
                report.text.contains(&a_line_as_the_report_writes_it(line)),
                "{:?}",
                line.kind
            );
        }
        assert!(report
            .text
            .contains(&a_line_as_the_report_writes_it(&diary[1])));
        assert!(report
            .text
            .contains(&a_line_as_the_report_writes_it(&diary[diary.len() - 2])));
        // The cut is stated at the point of the cut, and repeated in the head.
        let cut = format!(
            "{} routine lines between ",
            drop.ordinary_lines_dropped
        );
        assert_eq!(report.text.matches(&cut).count(), 2);
        assert!(the_head(&report.text).contains(&format!(
            "- App log: {} of {} lines. {cut}",
            diary.len() - drop.ordinary_lines_dropped,
            diary.len()
        )));
        assert!(report.text.contains(&format!("\n--- {cut}")));
        // Nothing report-time was touched.
        assert!(report
            .statements
            .iter()
            .all(|(_, statement)| *statement != Statement::Is(PartState::DroppedForTheCeiling)));
        assert!(report.text.contains(&a_tail().entries));
    }

    #[test]
    fn the_cut_names_the_retained_moments_either_side() {
        let (diary, job) = a_large_diary(80_000);
        let slice = Slice::AFailedJob { job };
        let opened = OpenedFrom::AFailure(a_naming_line());
        let report = compose(&a_report(&diary, &slice, &opened, None));
        let drop = report.the_middle_drop.clone().unwrap();
        let (before, after) = drop.the_retained_lines_either_side();
        let moment = |at: usize| the_moment_on_its_clock(diary[at].at, &diary[at].clock);
        assert!(report.text.contains(&format!(
            "between {} and {} were omitted to keep this report under 5 MB. All error and status lines \
             were kept.",
            moment(before.unwrap()),
            moment(after.unwrap())
        )));
    }

    #[test]
    fn statements_are_cut_only_once_every_ordinary_line_is_gone_and_no_part_goes_while_a_line_could() {
        // Statement lines alone well over the ceiling.
        let run = an_identity(IdentityKind::Run);
        let job = an_identity(IdentityKind::Job);
        let mut diary = Vec::new();
        for at in 0..60_000u64 {
            let kind = if at % 2 == 0 {
                LineKind::Failure
            } else {
                LineKind::Act
            };
            let mut line = a_line(&run, kind, at, &[&job]);
            line.text = format!("{kind:?} {at} {}", "y".repeat(100));
            diary.push(line);
        }
        let slice = Slice::AFailedJob { job };
        let opened = OpenedFrom::AFailure(a_naming_line());
        let adds = everything_taken();
        let report = compose(&a_report(&diary, &slice, &opened, Some(&adds)));
        assert!(report.fits_the_ceiling);
        let drop = report.the_middle_drop.clone().unwrap();
        assert!(drop.statements_too);
        assert_eq!(drop.ordinary_lines_dropped, 30_000);
        assert!(drop.statement_lines_dropped > 0 && drop.statement_lines_dropped < 30_000);
        assert!(report.text.contains(&format!(
            "All 30000 routine lines and {} error and status lines between ",
            drop.statement_lines_dropped
        )));
        // The diary lines were not exhausted, so no part was dropped.
        assert!(report
            .statements
            .iter()
            .all(|(_, statement)| *statement != Statement::Is(PartState::DroppedForTheCeiling)));
    }

    #[test]
    fn report_time_parts_go_whole_in_s4as_order_only_once_the_diary_lines_are_exhausted() {
        // These identities deliberately contain the crash content's old
        // three-letter search: dropping that part must still keep the diary.
        let run = Identity::read_back(IdentityKind::Run, "cccccccccccccccccccccccccccccccc").unwrap();
        let job = Identity::read_back(IdentityKind::Job, "11111111111111111111111111111111").unwrap();
        let diary = vec![
            a_line(&run, LineKind::Session, 0, &[]),
            a_line(&run, LineKind::Act, 1, &[&job]),
            a_line(&run, LineKind::Failure, 2, &[&job]),
        ];
        let slice = Slice::AFailedJob { job };
        let opened = OpenedFrom::AFailure(a_naming_line());
        let third = CEILING_BYTES as usize / 3;
        let big = |byte: char| byte.to_string().repeat(third);
        let tail = TheJournalTail {
            entries: big('t'),
            ..a_tail()
        };
        let crash = TheCrashFile {
            name: "heavy.crash.txt".to_string(),
            text: big('c'),
        };

        // Tail, crash file and note, each a third of the ceiling: the three
        // together cannot fit beside the head, so the tail goes first — and
        // the diary lines come back, because the room is there again.
        let adds = WhatTheMachineAdds {
            the_captured_output: vec![],
            the_journal_tail: Taken::Whole(tail.clone()),
            the_crash_file: Taken::Whole(crash.clone()),
            the_per_job_note: Taken::Whole(big('n')),
        };
        let report = compose(&a_report(&diary, &slice, &opened, Some(&adds)));
        assert!(report.fits_the_ceiling);
        assert!(report.statements.contains(&(
            Part::TheJournalTail,
            Statement::Is(PartState::DroppedForTheCeiling)
        )));
        assert!(report
            .statements
            .contains(&(Part::TheCrashFile, Statement::Is(PartState::Included))));
        assert!(report
            .statements
            .contains(&(Part::ThePerJobNote, Statement::Is(PartState::Included))));
        assert_eq!(
            the_line_for(&report, "Server event log"),
            format!("- Server event log: {LEFT_OUT_WHOLE}")
        );
        assert!(!report.text.contains(&tail.entries));
        assert!(report.text.contains(&crash.text));
        assert_eq!(
            report.the_middle_drop, None,
            "the diary lines fit again once the tail went"
        );

        // Twice as large: the tail and then the crash file go, the note stays.
        let two_thirds = CEILING_BYTES as usize * 2 / 3;
        let adds = WhatTheMachineAdds {
            the_captured_output: vec![],
            the_journal_tail: Taken::Whole(TheJournalTail {
                entries: "t".repeat(two_thirds),
                ..a_tail()
            }),
            the_crash_file: Taken::Whole(TheCrashFile {
                text: "c".repeat(two_thirds),
                ..crash.clone()
            }),
            the_per_job_note: Taken::Whole("n".repeat(two_thirds)),
        };
        let report = compose(&a_report(&diary, &slice, &opened, Some(&adds)));
        assert!(report.fits_the_ceiling);
        assert!(report.statements.contains(&(
            Part::TheJournalTail,
            Statement::Is(PartState::DroppedForTheCeiling)
        )));
        assert!(report.statements.contains(&(
            Part::TheCrashFile,
            Statement::Is(PartState::DroppedForTheCeiling)
        )));
        assert!(report
            .statements
            .contains(&(Part::ThePerJobNote, Statement::Is(PartState::Included))));
        // A part is included entire or dropped entire: never a piece of one.
        assert!(report.text.contains(&"n".repeat(two_thirds)));
        // Match the content at its line start, not inside a diary identity.
        assert!(!report.text.contains("\nccc"));
        assert!(!report.text.contains("\nttt"));
        assert!(!report.text.contains("\n== Blender's crash file, "));
        assert!(!report.text.contains("\n== Server event log, "));
        assert!(report.text.contains(&a_line_as_the_report_writes_it(&diary[1])));
        assert_eq!(report.the_middle_drop, None);

        // A note larger than the ceiling on its own goes too, last.
        let adds = WhatTheMachineAdds {
            the_captured_output: vec![],
            the_journal_tail: Taken::NothingThere { why: None },
            the_crash_file: Taken::Unreachable,
            the_per_job_note: Taken::Whole("n".repeat(CEILING_BYTES as usize + 1)),
        };
        let report = compose(&a_report(&diary, &slice, &opened, Some(&adds)));
        assert!(report.fits_the_ceiling);
        assert!(report.statements.contains(&(
            Part::ThePerJobNote,
            Statement::Is(PartState::DroppedForTheCeiling)
        )));
        // The states of the parts that had nothing to drop are left alone.
        assert!(report
            .statements
            .contains(&(Part::TheJournalTail, Statement::Is(PartState::Absent))));
        assert!(report
            .statements
            .contains(&(Part::TheCrashFile, Statement::Is(PartState::Unreachable))));
    }

    #[test]
    fn a_part_that_arrived_in_part_and_is_dropped_says_both() {
        let (diary, _, _, job) = a_diary();
        let slice = Slice::AFailedJob { job };
        let opened = OpenedFrom::AFailure(a_naming_line());
        let adds = WhatTheMachineAdds {
            the_captured_output: vec![],
            the_journal_tail: Taken::BeganAndDidNotComplete {
                what_arrived: Some(TheJournalTail {
                    entries: "t".repeat(CEILING_BYTES as usize),
                    ..a_tail()
                }),
            },
            the_crash_file: Taken::NothingThere { why: None },
            the_per_job_note: Taken::NothingThere { why: None },
        };
        let report = compose(&a_report(&diary, &slice, &opened, Some(&adds)));
        assert_eq!(
            the_line_for(&report, "Server event log"),
            format!("- Server event log: {LEFT_OUT_WHOLE}. Only part of it had been received")
        );
    }

    #[test]
    fn a_sentence_larger_than_the_ceiling_is_kept_whole_and_the_file_says_it_does_not_fit() {
        let (diary, _, now, _) = a_diary();
        let slice = Slice::NothingInParticular { this_run: now };
        let opened = OpenedFrom::TheMenu;
        let sentence = "s".repeat(CEILING_BYTES as usize + 10);
        let mut from = a_report(&diary, &slice, &opened, None);
        from.the_users_sentence = &sentence;
        let report = compose(&from);
        assert!(!report.fits_the_ceiling);
        assert!(report.text.contains(&sentence));
        assert_eq!(
            report
                .the_middle_drop
                .as_ref()
                .map(TheMiddleDrop::lines_dropped),
            Some(diary.len())
        );
    }

    #[test]
    fn the_file_name_is_the_drawn_form() {
        // 2025-08-25 14:32 UTC, read on a computer at UTC.
        assert_eq!(
            the_file_name(made_at(), 0, 1),
            "smoozy-report-25-aug-14-32.txt"
        );
        // The same moment read in a zone two hours east is that zone's clock.
        assert_eq!(
            the_file_name(made_at(), 7200, 1),
            "smoozy-report-25-aug-16-32.txt"
        );
        // And west across midnight, into the day before.
        assert_eq!(
            the_file_name(made_at(), -15 * 3600, 1),
            "smoozy-report-24-aug-23-32.txt"
        );
        // A single-figure day has no leading zero; hours and minutes do.
        let early = UNIX_EPOCH + Duration::from_secs(1_754_614_800); // 2025-08-08 01:00 UTC
        assert_eq!(the_file_name(early, 0, 1), "smoozy-report-8-aug-01-00.txt");
    }

    #[test]
    fn a_taken_name_is_composed_again_with_a_suffix_that_does_not_collide() {
        assert_eq!(
            the_file_name(made_at(), 0, 2),
            "smoozy-report-25-aug-14-32-2.txt"
        );
        assert_eq!(
            the_file_name(made_at(), 0, 3),
            "smoozy-report-25-aug-14-32-3.txt"
        );
        let names: std::collections::HashSet<String> =
            (1..50).map(|n| the_file_name(made_at(), 0, n)).collect();
        assert_eq!(names.len(), 49);
    }

    #[test]
    fn every_month_has_its_three_letters() {
        let months: Vec<String> = (0..12)
            .map(|month| {
                // The fifteenth of each month of 2025.
                let days_to_the_fifteenth =
                    [14, 45, 73, 104, 134, 165, 195, 226, 257, 287, 318, 348][month];
                let at = UNIX_EPOCH
                    + Duration::from_secs(1_735_689_600 + days_to_the_fifteenth * 86_400);
                the_file_name(at, 0, 1)
            })
            .collect();
        for (name, month) in months.iter().zip([
            "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec",
        ]) {
            assert_eq!(name, &format!("smoozy-report-15-{month}-00-00.txt"));
        }
    }

    #[test]
    fn the_moment_in_utc_is_a_complete_date_and_time() {
        assert_eq!(the_moment_in_utc(made_at()), "2025-08-25 14:32:00 UTC");
        assert_eq!(the_moment_in_utc(UNIX_EPOCH), "1970-01-01 00:00:00 UTC");
        // A leap day.
        assert_eq!(
            the_moment_in_utc(UNIX_EPOCH + Duration::from_secs(1_709_208_000)),
            "2024-02-29 12:00:00 UTC"
        );
    }

    #[test]
    fn a_report_written_is_declarable_and_its_line_is_the_declared_form() {
        assert!(A_REPORT_WRITTEN.is_diary_worthy(A_REPORT_WRITTEN_IS_NO_ERASURE));
        let run = an_identity(IdentityKind::Run);
        let name = the_file_name(made_at(), 0, 1);
        let from_the_menu =
            the_report_written_line(&run, made_at(), "/Users/artist/Downloads", &name, &[]);
        assert_eq!(from_the_menu.kind, LineKind::Act);
        assert_eq!(from_the_menu.clock, Clock::ThisComputer);
        assert_eq!(from_the_menu.run, run);
        assert_eq!(from_the_menu.at, made_at());
        assert_eq!(
            from_the_menu.text,
            "Report written: smoozy-report-25-aug-14-32.txt in /Users/artist/Downloads"
        );
        assert!(from_the_menu.concerning.is_empty());

        let condition = an_identity(IdentityKind::Condition);
        let subject = an_identity(IdentityKind::Job);
        let from_a_failure = the_report_written_line(
            &run,
            made_at(),
            "/Users/artist/Downloads",
            &name,
            &[condition.clone(), subject.clone()],
        );
        assert_eq!(from_a_failure.concerning, vec![condition, subject]);
        // The user's sentence is the report's and never the record's.
        assert!(!from_a_failure.text.contains("frames"));
    }
}
