//! A report is a slice — `Reporting a problem` §5, the half that works on the
//! diary's own lines.
//!
//! **Pure rules over lines already read back.** Nothing here opens the diary,
//! reads a file or touches a machine. The diary service reads the record back
//! (R4: it is the one writer and the one reader) and hands the lines here in
//! the record's own order, oldest first.
//!
//! What is here: S2's three slices, cut on run identities and job identities,
//! with U7's reading of *the one before*; S3's ceiling; and S4's middle drop,
//! which never drops a line that states something until nothing else is left.
//! The file the slice goes into — its head, the report-time parts, S4a's whole
//! drops and S5's statements — is [`crate::report_file`]'s.
//!
//! N1 binds all of it: **the cut performs no filtering of any kind.** It
//! slices, trims and states, and every byte it may include was admissible the
//! moment it was written. Nothing here reads a line's text to decide anything.

use std::collections::HashSet;
use std::ops::Range;

use smoozy_state::identities::Identity;

use crate::lines::{Line, LineKind};

/// S3: **the report ceiling is 5 MB, fixed rather than read**, and it is this
/// plan's own constant.
///
/// The spec gives the diary 20 MB and the report no figure; *too large* is a
/// receiving provider's limit and not Smoozy's to know, so no reading and no
/// outside claim decides this. **It is a quarter of the diary's cap**, so a
/// report is always strictly smaller than the record it is cut from — which is
/// why it is written as the cap divided rather than as a second number that
/// could drift from it. No free-space reading and no mail-provider belief
/// raises or lowers it. The runner's to overrule, exactly as the cap is
/// (§12 item 9).
pub const CEILING_BYTES: u64 = crate::diary::CAP_BYTES / 4;

/// S2: which slice a report is cut on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Slice {
    /// **A failed job**: every line carrying that job identity, plus the whole
    /// of every run in which such a line occurs — *that job's own acts and the
    /// session around them*.
    AFailedJob { job: Identity },
    /// **A crash**: every line carrying the run identity the run-mark named
    /// (C3) — *the session that died*, findable after the run is gone because
    /// the identity is durable in the mark and on the lines.
    ACrash { run: Identity },
    /// **Nothing in particular**: the current run's lines and the previous
    /// run's, the previous being read off the record's own order (U7). The
    /// menu's slice (X2), and a failure's whose subject is not a job (X4).
    NothingInParticular { this_run: Identity },
}

/// One slice, cut.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheSlice<'a> {
    /// The slice's lines, **in the record's own order**. Every run in it is
    /// whole: each slicing rule takes runs entire.
    pub lines: Vec<&'a Line>,
    /// For the third slice only: the run before this one, where the record
    /// holds one. `None` for the other two slices.
    pub the_run_before: Option<Identity>,
    /// **Whether the cap may have taken lines this slice would have covered.**
    ///
    /// Read off the record's own shape and nothing else, three ways: the slice
    /// is empty; or it reaches the record's oldest line, so whatever came
    /// before is gone; or one of its runs does not open with its session line,
    /// which U7 says every run does. On its own this does not say the cap
    /// dropped anything — a fresh record looks the same — so the statement S5
    /// asks for is made only where the diary service also says lines were
    /// evicted ([`crate::report_file`]).
    pub the_cap_may_have_taken_lines_from_it: bool,
}

/// S2 and U7: cut a slice out of the record.
///
/// `record` is the whole diary as read back, oldest first.
pub fn the_slice<'a>(record: &'a [Line], slice: &Slice) -> TheSlice<'a> {
    let (runs, the_run_before): (HashSet<Identity>, Option<Identity>) = match slice {
        Slice::AFailedJob { job } => (
            record
                .iter()
                .filter(|line| line.concerning.contains(job))
                .map(|line| line.run.clone())
                .collect(),
            None,
        ),
        Slice::ACrash { run } => ([run.clone()].into_iter().collect(), None),
        Slice::NothingInParticular { this_run } => {
            let before = the_run_before(record, this_run);
            let runs = [Some(this_run.clone()), before.clone()]
                .into_iter()
                .flatten()
                .collect();
            (runs, before)
        }
    };

    let positions: Vec<usize> = (0..record.len())
        .filter(|&at| runs.contains(&record[at].run))
        .collect();

    let reaches_the_oldest_line = match slice {
        // The run before is read off whatever precedes this run's session
        // line. Where nothing precedes it, the run before — if there was one —
        // is not in the record any more.
        Slice::NothingInParticular { .. } => {
            positions.first() == Some(&0) || the_run_before.is_none()
        }
        _ => positions.first() == Some(&0),
    };
    let a_run_does_not_open_with_its_session_line = runs.iter().any(|run| {
        record
            .iter()
            .find(|line| &line.run == run)
            .is_some_and(|first| first.kind != LineKind::Session)
    });

    TheSlice {
        lines: positions.iter().map(|&at| &record[at]).collect(),
        the_run_before,
        the_cap_may_have_taken_lines_from_it: positions.is_empty()
            || reaches_the_oldest_line
            || a_run_does_not_open_with_its_session_line,
    }
}

/// U7: ***the one before* is the run whose lines immediately precede the
/// current run's session line in the record's own order.**
///
/// Where the current run's session line is not in the record — a refused
/// write loses the entry and nothing else — its first line stands in for it,
/// and where the current run has no line at all, the end of the record does:
/// that is where its session line would have been. The nearest earlier line of
/// another run names the run before.
pub fn the_run_before(record: &[Line], this_run: &Identity) -> Option<Identity> {
    let opens_at = record
        .iter()
        .position(|line| &line.run == this_run && line.kind == LineKind::Session)
        .or_else(|| record.iter().position(|line| &line.run == this_run))
        .unwrap_or(record.len());
    record[..opens_at]
        .iter()
        .rev()
        .find(|line| &line.run != this_run)
        .map(|line| line.run.clone())
}

/// S4: **lines that state something are never dropped** — session lines,
/// failure lines, assumption lines, gap lines and last lines. Act and told
/// lines are the ordinary ones the span widens across.
///
/// *Where the least is happening* is answered by position and kind, never by
/// reading content for meaning: the diary is dense where trouble is because
/// failures, assumptions and gaps are statement lines, and statements are
/// never dropped.
pub fn states_something(kind: LineKind) -> bool {
    match kind {
        LineKind::Session
        | LineKind::Failure
        | LineKind::Assumption
        | LineKind::Gap
        | LineKind::Last => true,
        LineKind::Act | LineKind::Told => false,
    }
}

/// S4's one contiguous span, dropped from the middle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheMiddleDrop {
    /// The span, in positions of the slice. It grew outward from the slice's
    /// midpoint.
    pub span: Range<usize>,
    /// **S4's last resort**: every ordinary line had gone and the statements
    /// alone still did not fit, so the span took statements too — and the cut
    /// statement says so.
    pub statements_too: bool,
    /// Per position of the slice: whether that line was dropped.
    pub dropped: Vec<bool>,
    /// How many act and told lines were dropped.
    pub ordinary_lines_dropped: usize,
    /// How many lines that state something were dropped. Zero unless
    /// [`Self::statements_too`].
    pub statement_lines_dropped: usize,
}

impl TheMiddleDrop {
    pub fn lines_dropped(&self) -> usize {
        self.ordinary_lines_dropped + self.statement_lines_dropped
    }

    /// **The span by its first and last retained moments**: the kept line just
    /// before the span and the kept line just after it, as positions of the
    /// slice. `None` on a side where the span reached the slice's edge.
    pub fn the_retained_lines_either_side(&self) -> (Option<usize>, Option<usize>) {
        let before = self.span.start.checked_sub(1);
        let after = (self.span.end < self.dropped.len()).then_some(self.span.end);
        (before, after)
    }

    /// Where the cut statement is written: at the point of the cut, which is
    /// the span's start. Statement lines S4 kept from inside the span follow
    /// it in their own order.
    pub fn the_point_of_the_cut(&self) -> usize {
        self.span.start
    }
}

/// What S4 came to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheMiddleDropCameTo {
    /// The slice fitted whole, and nothing was dropped.
    NothingWasDropped,
    /// It fitted once this was dropped.
    Dropped(TheMiddleDrop),
    /// **Every line of the slice is gone and it still does not fit.** S4a then
    /// drops the report-time parts whole, and the diary lines are cut again
    /// against the room that leaves ([`crate::report_file`]).
    NothingLeftToDrop(TheMiddleDrop),
}

/// S4: one contiguous span dropped from the middle, whole lines only.
///
/// `sizes` and `kinds` are the slice's lines, position by position. `fits` is
/// asked whether the file fits with this many bytes of the slice's lines kept
/// and this drop made — the caller knows what else the file holds and what the
/// cut statement costs. **It is asked first with nothing dropped**, and then
/// after every line the span drops, so the span stops at the first width at
/// which the remainder fits.
///
/// The mechanism, exactly as S4 states it:
///
/// - **The span grows outward from the slice's midpoint until the remainder
///   fits** — the head and tail of the slice always survive while anything
///   else can go, which is where a story's beginning and its ending live. It
///   takes the position after the span, then the one before, and so on; on a
///   side that has reached the slice's edge it takes the other side only.
/// - **Lines that state something are excluded from the span**, which widens
///   across the act and told lines around them.
/// - **Only where the statements alone still do not fit does the span take
///   statements too.** By then every ordinary line has gone, so the span is
///   grown again from the midpoint over the statements.
pub fn drop_from_the_middle(
    kinds: &[LineKind],
    sizes: &[u64],
    mut fits: impl FnMut(u64, Option<&TheMiddleDrop>) -> bool,
) -> WhatTheMiddleDropCameTo {
    assert_eq!(kinds.len(), sizes.len(), "one size per line of the slice");
    let all: u64 = sizes.iter().sum();
    if fits(all, None) {
        return WhatTheMiddleDropCameTo::NothingWasDropped;
    }

    let count = kinds.len();
    let mut drop = TheMiddleDrop {
        span: count / 2..count / 2,
        statements_too: false,
        dropped: vec![false; count],
        ordinary_lines_dropped: 0,
        statement_lines_dropped: 0,
    };
    let mut kept = all;

    for statements_too in [false, true] {
        drop.statements_too = statements_too;
        drop.span = count / 2..count / 2;
        while drop.span.start > 0 || drop.span.end < count {
            // After the span first, then before it, so the middle stays the
            // middle; a side at the edge gives way to the other.
            let take_after = drop.span.end < count
                && (drop.span.start == 0
                    || drop.span.end - count / 2 <= count / 2 - drop.span.start);
            let at = if take_after {
                drop.span.end += 1;
                drop.span.end - 1
            } else {
                drop.span.start -= 1;
                drop.span.start
            };
            let droppable = statements_too || !states_something(kinds[at]);
            if !droppable || drop.dropped[at] {
                continue;
            }
            drop.dropped[at] = true;
            kept -= sizes[at];
            if states_something(kinds[at]) {
                drop.statement_lines_dropped += 1;
            } else {
                drop.ordinary_lines_dropped += 1;
            }
            if fits(kept, Some(&drop)) {
                return WhatTheMiddleDropCameTo::Dropped(drop);
            }
        }
        // A slice with no lines to drop in this pass reaches here at once; one
        // that dropped every line it could and still does not fit goes on to
        // the statements, and then to S4a.
    }
    drop.span = 0..count;
    WhatTheMiddleDropCameTo::NothingLeftToDrop(drop)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use smoozy_state::identities::IdentityKind;
    use smoozy_state::observed::Clock;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    pub(crate) fn an_identity(kind: IdentityKind) -> Identity {
        Identity::issue(kind).expect("the system supplies random bytes")
    }

    pub(crate) fn a_line(
        run: &Identity,
        kind: LineKind,
        second: u64,
        concerning: &[&Identity],
    ) -> Line {
        Line {
            run: run.clone(),
            kind,
            at: UNIX_EPOCH + Duration::from_secs(1_756_130_000 + second),
            clock: Clock::ThisComputer,
            text: format!("{kind:?} line at {second}"),
            concerning: concerning
                .iter()
                .map(|identity| (*identity).clone())
                .collect(),
        }
    }

    /// Three runs, the middle one touching a job, the last touching it again.
    fn a_record() -> (Vec<Line>, [Identity; 3], Identity) {
        let runs = [
            an_identity(IdentityKind::Run),
            an_identity(IdentityKind::Run),
            an_identity(IdentityKind::Run),
        ];
        let job = an_identity(IdentityKind::Job);
        let other_job = an_identity(IdentityKind::Job);
        let record = vec![
            a_line(&runs[0], LineKind::Session, 0, &[]),
            a_line(&runs[0], LineKind::Act, 1, &[&other_job]),
            a_line(&runs[1], LineKind::Session, 10, &[]),
            a_line(&runs[1], LineKind::Act, 11, &[&job]),
            a_line(&runs[1], LineKind::Told, 12, &[&job]),
            a_line(&runs[1], LineKind::Last, 13, &[]),
            a_line(&runs[2], LineKind::Session, 20, &[]),
            a_line(&runs[2], LineKind::Act, 21, &[]),
            a_line(&runs[2], LineKind::Failure, 22, &[&job]),
        ];
        (record, runs, job)
    }

    fn seconds_of(slice: &TheSlice) -> Vec<u64> {
        slice
            .lines
            .iter()
            .map(|line| line.at.duration_since(UNIX_EPOCH).unwrap().as_secs() - 1_756_130_000)
            .collect()
    }

    #[test]
    fn the_ceiling_is_five_megabytes_a_quarter_of_the_cap() {
        assert_eq!(CEILING_BYTES, 5_242_880);
        assert_eq!(CEILING_BYTES * 4, crate::diary::CAP_BYTES);
    }

    #[test]
    fn a_failed_job_slice_is_every_run_the_job_appears_in_whole() {
        let (record, _, job) = a_record();
        let slice = the_slice(&record, &Slice::AFailedJob { job });
        // Runs two and three whole, including their lines that never name the
        // job; run one not at all.
        assert_eq!(seconds_of(&slice), vec![10, 11, 12, 13, 20, 21, 22]);
        assert_eq!(slice.the_run_before, None);
        assert!(!slice.the_cap_may_have_taken_lines_from_it);
    }

    #[test]
    fn a_crash_slice_is_every_line_of_the_marked_run_and_nothing_else() {
        let (record, runs, _) = a_record();
        let slice = the_slice(
            &record,
            &Slice::ACrash {
                run: runs[1].clone(),
            },
        );
        assert_eq!(seconds_of(&slice), vec![10, 11, 12, 13]);
    }

    #[test]
    fn nothing_in_particular_is_this_run_and_the_one_before() {
        let (record, runs, _) = a_record();
        let slice = the_slice(
            &record,
            &Slice::NothingInParticular {
                this_run: runs[2].clone(),
            },
        );
        assert_eq!(seconds_of(&slice), vec![10, 11, 12, 13, 20, 21, 22]);
        assert_eq!(slice.the_run_before, Some(runs[1].clone()));
        assert!(!slice.the_cap_may_have_taken_lines_from_it);
    }

    #[test]
    fn the_one_before_is_read_off_the_line_before_this_runs_session_line() {
        let (mut record, runs, _) = a_record();
        // A line of the current run written before its session line could not
        // happen in order; what U7 reads is what precedes the session line.
        let this_run = an_identity(IdentityKind::Run);
        record.push(a_line(&this_run, LineKind::Session, 30, &[]));
        assert_eq!(the_run_before(&record, &this_run), Some(runs[2].clone()));
    }

    #[test]
    fn a_run_whose_session_line_was_never_written_still_finds_the_one_before() {
        let (mut record, runs, _) = a_record();
        let this_run = an_identity(IdentityKind::Run);
        record.push(a_line(&this_run, LineKind::Act, 30, &[]));
        assert_eq!(the_run_before(&record, &this_run), Some(runs[2].clone()));
        // And a run with no line at all reads the end of the record.
        let silent = an_identity(IdentityKind::Run);
        assert_eq!(the_run_before(&record, &silent), Some(this_run));
    }

    #[test]
    fn a_first_run_has_no_run_before_and_says_the_cap_may_have_reached_it() {
        let run = an_identity(IdentityKind::Run);
        let record = vec![a_line(&run, LineKind::Session, 0, &[])];
        let slice = the_slice(&record, &Slice::NothingInParticular { this_run: run });
        assert_eq!(slice.the_run_before, None);
        assert!(slice.the_cap_may_have_taken_lines_from_it);
    }

    #[test]
    fn a_run_cut_into_by_the_cap_is_seen_by_its_missing_session_line() {
        let (record, runs, _) = a_record();
        // The oldest segment went with run one and run two's session line.
        let evicted = &record[3..];
        let slice = the_slice(
            evicted,
            &Slice::ACrash {
                run: runs[1].clone(),
            },
        );
        assert_eq!(seconds_of(&slice), vec![11, 12, 13]);
        assert!(slice.the_cap_may_have_taken_lines_from_it);
        // And run three, whole, is not.
        let slice = the_slice(
            evicted,
            &Slice::ACrash {
                run: runs[2].clone(),
            },
        );
        assert!(!slice.the_cap_may_have_taken_lines_from_it);
    }

    #[test]
    fn a_crash_slice_with_no_lines_left_says_the_cap_may_have_taken_them() {
        let (record, _, _) = a_record();
        let gone = an_identity(IdentityKind::Run);
        let slice = the_slice(&record, &Slice::ACrash { run: gone });
        assert!(slice.lines.is_empty());
        assert!(slice.the_cap_may_have_taken_lines_from_it);
    }

    #[test]
    fn the_statement_kinds_are_the_five_s4_names() {
        let statements: Vec<LineKind> = LineKind::ALL
            .into_iter()
            .filter(|kind| states_something(*kind))
            .collect();
        assert_eq!(
            statements,
            vec![
                LineKind::Session,
                LineKind::Failure,
                LineKind::Assumption,
                LineKind::Gap,
                LineKind::Last
            ]
        );
    }

    /// A drop against a plain byte budget, with the cut statement costing
    /// nothing, so the arithmetic is the span's alone.
    fn against(budget: u64, kinds: &[LineKind], sizes: &[u64]) -> WhatTheMiddleDropCameTo {
        drop_from_the_middle(kinds, sizes, |kept, _| kept <= budget)
    }

    #[test]
    fn a_slice_that_fits_is_not_cut() {
        let kinds = [LineKind::Act; 4];
        assert_eq!(
            against(40, &kinds, &[10; 4]),
            WhatTheMiddleDropCameTo::NothingWasDropped
        );
    }

    #[test]
    fn the_span_grows_from_the_middle_and_keeps_the_head_and_the_tail() {
        let kinds = [LineKind::Act; 9];
        let WhatTheMiddleDropCameTo::Dropped(drop) = against(60, &kinds, &[10; 9]) else {
            panic!("it fits once three lines go");
        };
        assert_eq!(drop.span, 3..6);
        assert_eq!(
            drop.dropped,
            vec![false, false, false, true, true, true, false, false, false]
        );
        assert_eq!(drop.ordinary_lines_dropped, 3);
        assert_eq!(drop.the_retained_lines_either_side(), (Some(2), Some(6)));
    }

    #[test]
    fn a_statement_line_in_the_middle_is_never_dropped() {
        let mut kinds = [LineKind::Told; 9];
        kinds[4] = LineKind::Failure;
        let WhatTheMiddleDropCameTo::Dropped(drop) = against(60, &kinds, &[10; 9]) else {
            panic!("it fits once three ordinary lines go");
        };
        assert!(!drop.dropped[4], "the failure line stays");
        assert_eq!(drop.ordinary_lines_dropped, 3);
        assert_eq!(drop.statement_lines_dropped, 0);
        assert!(!drop.statements_too);
        // The span widened across it rather than stopping at it.
        assert!(drop.span.contains(&4));
    }

    #[test]
    fn every_kind_that_states_something_survives_while_an_ordinary_line_remains() {
        let mut kinds = Vec::new();
        for kind in LineKind::ALL {
            kinds.push(kind);
            kinds.push(LineKind::Act);
        }
        let sizes = vec![10; kinds.len()];
        // Room for the five statement lines and two ordinary ones.
        let WhatTheMiddleDropCameTo::Dropped(drop) = against(70, &kinds, &sizes) else {
            panic!("it fits with the statements kept");
        };
        for (at, kind) in kinds.iter().enumerate() {
            if states_something(*kind) {
                assert!(!drop.dropped[at], "{kind:?} was dropped");
            }
        }
    }

    #[test]
    fn statements_are_taken_only_when_they_alone_do_not_fit() {
        let kinds = [
            LineKind::Session,
            LineKind::Act,
            LineKind::Failure,
            LineKind::Told,
            LineKind::Gap,
            LineKind::Act,
            LineKind::Last,
        ];
        // Four statements of ten bytes each, and room for three.
        let WhatTheMiddleDropCameTo::Dropped(drop) = against(30, &kinds, &[10; 7]) else {
            panic!("it fits once one statement goes");
        };
        assert!(drop.statements_too);
        assert_eq!(drop.ordinary_lines_dropped, 3);
        assert_eq!(drop.statement_lines_dropped, 1);
        // From the middle: the statement nearest the midpoint goes — the span
        // takes the side after it first, finds that line already gone, and
        // takes the failure line before it — and the session line at the head,
        // the gap line and the last line at the tail stay.
        assert!(drop.dropped[2]);
        assert!(!drop.dropped[0] && !drop.dropped[4] && !drop.dropped[6]);
    }

    #[test]
    fn a_slice_that_cannot_fit_drops_everything_and_says_so() {
        let kinds = [LineKind::Session, LineKind::Act, LineKind::Last];
        let WhatTheMiddleDropCameTo::NothingLeftToDrop(drop) =
            drop_from_the_middle(&kinds, &[10; 3], |_, _| false)
        else {
            panic!("nothing fits");
        };
        assert_eq!(drop.dropped, vec![true; 3]);
        assert_eq!(drop.span, 0..3);
        assert!(drop.statements_too);
    }

    #[test]
    fn an_empty_slice_that_does_not_fit_has_nothing_to_drop() {
        assert!(matches!(
            drop_from_the_middle(&[], &[], |_, _| false),
            WhatTheMiddleDropCameTo::NothingLeftToDrop(_)
        ));
    }

    #[test]
    fn the_drop_is_asked_about_after_every_line_and_stops_at_the_first_fit() {
        let kinds = [LineKind::Act; 6];
        let mut asked = Vec::new();
        let _ = drop_from_the_middle(&kinds, &[10; 6], |kept, drop| {
            asked.push((kept, drop.map(TheMiddleDrop::lines_dropped)));
            kept <= 40
        });
        assert_eq!(asked, vec![(60, None), (50, Some(1)), (40, Some(2))]);
    }

    #[test]
    fn a_large_slice_is_cut_in_reasonable_time() {
        // A whole diary's worth of lines: the cap over a hundred bytes each.
        let count = (crate::diary::CAP_BYTES / 100) as usize;
        let kinds: Vec<LineKind> = (0..count)
            .map(|at| {
                if at % 50 == 0 {
                    LineKind::Failure
                } else {
                    LineKind::Act
                }
            })
            .collect();
        let sizes = vec![100; count];
        let started = SystemTime::now();
        let WhatTheMiddleDropCameTo::Dropped(drop) = against(CEILING_BYTES, &kinds, &sizes) else {
            panic!("it fits once the middle goes");
        };
        assert!(started.elapsed().unwrap() < Duration::from_secs(10));
        assert_eq!(drop.statement_lines_dropped, 0);
        assert!(!drop.dropped[0] && !drop.dropped[count - 1]);
    }
}
