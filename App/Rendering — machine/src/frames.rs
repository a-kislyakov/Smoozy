//! The frame list — L17 and L18: every job renders as an animation, the
//! frame arguments are the explicit list, and the list is composed from the
//! reckoning's own integers with no frame in it twice.
//!
//! **L17.** The render instruction is the animation form, never the
//! single-frame form, even for a job covering one frame — settled in `What a
//! job writes` and applied here. The ground is the naming split: started
//! from outside, the Python still call leaves the frame number out of the
//! name and every command-line render puts it in (*Writing 6*, *Writing 8*),
//! and a file output node adds its own split — the frame number every time
//! in 4.5, only on an animation from 5.0 (*Writing 6*) — **and the names on
//! disk are what the re-run's already-there test reads**, so the wrong form
//! corrupts the carry-on as well as the naming. The frame set is handed over
//! as an explicit list — available in every version, taken in the order
//! written, mixing single frames and stretches (`Queue & History` D6;
//! *Frames 7*) — and Blender's own skipping is never used (`Job files and
//! outputs` C2).
//!
//! **What the explicit list is, on the evidence, and why no separate
//! animation switch rides beside it.** *Frames 7* records the list's own
//! argument taken in the order written; *Writing 8*'s frame-number rules
//! were taken from *"a command-line frame render, a command-line animation
//! render and a Python animation render"* and hold for all three, so the
//! list's renders carry the frame number — which is what *the animation
//! form* means for the names on disk — and *Writing 6* carries 5.0's own
//! release note: *"Rendering through the command line always does an
//! animation render."* A whole-range switch handed in beside the list would
//! render the file's own range on top of it (*Frames 7*: *"`-s 1 -e 2 -a -f
//! 20` carried out both"*), paying for frames nobody asked for. So the
//! explicit list is the render instruction and ends the window.
//!
//! **Except for a video, since 2026-10-17 (L17a, the video piece).** A frame list
//! on a job whose destination is video-shaped wrote a finished video holding
//! one frame, in every version the video desk test ran. Such a job's list is still
//! composed here and still recorded, but the line hands Blender its first
//! and last frame and the animation instruction instead
//! (`crate::line::Argument::AnimationOverOneRun`).
//!
//! **L17a's step, since the same day (the video-range piece).** The first and last frame
//! alone keep the file's saved step, so a video job's run hands its own step
//! too, and a list evenly stepped is one run; only a list with gaps is refused.
//!
//! **L17b: a frame below zero is never handed to Blender as a bare negative
//! number** (added 2026-10-17, the video-range piece). The second desk test found every
//! signed number on the command line counted from the file's own range, never
//! taken as the frame named: `-f -3..0` on a file saved at −5 to 5 rendered
//! **nothing and exited 0**, `-f -3` rendered frame 3, and `-s -3` started the
//! run at −8. **So every frame below zero is written as an offset from the
//! file's saved start as the reading recorded it** (`+N`, N frames after it:
//! `-f +2..+5` rendered −3 to 0), and every frame at or above zero stays
//! absolute. [`AFrameWord`] is the one place a frame becomes a word. **The one
//! exception is the end of a video's run**, which a sign moves from the file's
//! own end and never from its start (that test's runs 26 and 27: `-e -2` gave 3
//! and `-e -5` gave 0 on a file ending at 5), so it is written from the end.
//! **A frame below zero outside the file's saved range is refused in words
//! rather than guessed at**: in that test only a run's start was ever moved past
//! the range, never a frame of a list, and one rule for both is the lean one.
//!
//! **L18.** A malformed frame list renders nothing at all and still reports
//! success (*Frames 7*; `Job files and outputs` C3), so the list is never
//! assembled from anything a user typed or a document parsed; and a frame
//! named twice is rendered twice (*Frames 7*), which on a paid machine is
//! money spent on a frame already had, so the composition emits each frame
//! once, in the asked order.
//!
//! **Markers used in this file that belong to another document, each with
//! the document it belongs to.** `Queue & History` D6. `Job files and
//! outputs` C2, C3.

/// **L18: the list as the line carries it — integers, each once, in the
/// asked order.** The one constructor takes integers and nothing else, so
/// text never reaches it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExplicitFrameList {
    frames: Vec<i64>,
}

/// Why no list was composed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoList {
    /// **S13: a job with no frame to name is not composed at all** — the
    /// second job of a one-frame range is the case, and L8 item 8 has
    /// nothing to name for it.
    NothingToName,
}

impl NoList {
    /// **Why there is no frame list, in standard software words** (Rule 9).
    /// Added 2026-09-23, step 10c's fifth pass on Vast, fault 5.
    pub fn in_words(self) -> &'static str {
        match self {
            NoList::NothingToName => "no frames left to render",
        }
    }
}

impl ExplicitFrameList {
    /// **From the reckoning's own integers** (P1 step 6, `Job files and
    /// outputs` §10). A frame named twice is emitted once, at its first
    /// place in the asked order.
    pub fn from_the_reckonings_integers(asked: &[i64]) -> Result<ExplicitFrameList, NoList> {
        let mut frames: Vec<i64> = Vec::with_capacity(asked.len());
        for frame in asked {
            if !frames.contains(frame) {
                frames.push(*frame);
            }
        }
        if frames.is_empty() {
            return Err(NoList::NothingToName);
        }
        Ok(ExplicitFrameList { frames })
    }

    /// The frames, in the asked order, each once. **This is what the attempt
    /// record carries** (`Job files and outputs` M8 — the list in the order
    /// it was written).
    pub fn in_the_asked_order(&self) -> &[i64] {
        &self.frames
    }

    /// **The argument's text: a comma list mixing single frames and
    /// stretches** (*Frames 7*: `-f 1..3,10..12`), a stretch being three or
    /// more consecutive ascending frames. The order is the asked order;
    /// inside a stretch the frames are ascending by construction, which is
    /// the order a stretch renders in (*Frames 10*).
    ///
    /// **L17b: every frame below zero is written as its offset from the
    /// file's saved start** ([`AFrameWord`]), and **a stretch never crosses
    /// from below zero to zero**: it is split there, so each stretch is
    /// written in one of the two forms the second desk test ran (`+2..+5`, and
    /// the absolute `1..10`), never a mix of them. **A frame below zero
    /// outside the file's saved range has no word**, and the list is refused.
    pub fn as_argument_text(&self, from: TheFilesSavedRange) -> Result<String, ABareNegativeFrame> {
        let word = |frame: i64| AFrameWord::in_a_list(frame, from).map(AFrameWord::as_text);
        let mut parts: Vec<String> = Vec::new();
        let mut at = 0;
        while at < self.frames.len() {
            let start = self.frames[at];
            let mut end = at;
            while end + 1 < self.frames.len()
                && self.frames[end + 1] == self.frames[end] + 1
                && (self.frames[end + 1] < 0) == (start < 0)
            {
                end += 1;
            }
            if end - at >= 2 {
                parts.push(format!("{}..{}", word(start)?, word(self.frames[end])?));
            } else {
                for frame in &self.frames[at..=end] {
                    parts.push(word(*frame)?);
                }
            }
            at = end + 1;
        }
        Ok(parts.join(","))
    }

    /// **L17a's step: whether the list is one evenly stepped ascending run**,
    /// and its step where it is — the gap between its first two frames, and 1
    /// for a list of one frame. `None` for a list with gaps, which one video
    /// cannot hold, or out of order.
    pub fn as_one_stepped_run(&self) -> Option<OneSteppedRun> {
        let (first, last) = (*self.frames.first()?, *self.frames.last()?);
        let step = match self.frames.get(1) {
            Some(second) => second - first,
            None => 1,
        };
        let even = step >= 1 && self.frames.windows(2).all(|pair| pair[1] - pair[0] == step);
        even.then_some(OneSteppedRun { first, last, step })
    }

    /// **L17: the last frame in the asked order** — the frame `Job files and
    /// outputs` C17's rule and `Queue & History` N6's figure are both keyed
    /// on, which is why the order is the record.
    pub fn the_last_asked_for(&self) -> i64 {
        *self.frames.last().expect("a list is never empty")
    }

    pub fn len(&self) -> usize {
        self.frames.len()
    }

    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }
}

/// **L17a: one evenly stepped ascending run** — what a video job's frames
/// must be, and all its render instruction carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OneSteppedRun {
    pub first: i64,
    pub last: i64,
    /// At least 1.
    pub step: i64,
}

/// **L17b: the job's scene's saved first and last frame, as the reading in
/// force recorded them** — what every signed frame on the command line counts
/// from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheFilesSavedRange {
    pub start: i64,
    pub end: i64,
}

/// **A frame below zero outside the file's saved range** — it has no word
/// L17b can write, so the line is refused rather than handed a bare negative.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ABareNegativeFrame {
    pub frame: i64,
    pub from: TheFilesSavedRange,
}

/// **L17b: one frame, as the command line is handed it.** The one place a
/// frame becomes a word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AFrameWord {
    /// A frame at or above zero, written as itself.
    Absolute(i64),
    /// A frame below zero, written `+N`: N frames after the file's saved
    /// start (*`-f +2`* on a file starting at −5 rendered −3; *`-s +2`* gave a
    /// start of −3).
    AfterTheFilesStart(i64),
    /// **The end of a video's run below zero**, written from the file's saved
    /// end, which is what a sign on the end argument moves: `-N` is N frames
    /// before it, `+0` the end itself (*`-e -5`* on a file ending at 5 gave 0).
    BeforeTheFilesEnd(i64),
}

impl AFrameWord {
    /// **A frame of a list, or the start of a video's run**: below zero, its
    /// offset from the file's saved start.
    pub fn in_a_list(frame: i64, from: TheFilesSavedRange) -> Result<AFrameWord, ABareNegativeFrame> {
        if frame >= 0 {
            return Ok(AFrameWord::Absolute(frame));
        }
        if frame < from.start || frame > from.end {
            return Err(ABareNegativeFrame { frame, from });
        }
        Ok(AFrameWord::AfterTheFilesStart(frame - from.start))
    }

    /// **The end of a video's run**: below zero, how far before the file's
    /// saved end it falls.
    pub fn at_the_end(frame: i64, from: TheFilesSavedRange) -> Result<AFrameWord, ABareNegativeFrame> {
        if frame >= 0 {
            return Ok(AFrameWord::Absolute(frame));
        }
        if frame < from.start || frame > from.end {
            return Err(ABareNegativeFrame { frame, from });
        }
        Ok(AFrameWord::BeforeTheFilesEnd(from.end - frame))
    }

    /// The word itself. **A signed word always carries its sign**, `+0`
    /// included, because a bare `0` is the absolute frame 0.
    pub fn as_text(self) -> String {
        match self {
            AFrameWord::Absolute(frame) => frame.to_string(),
            AFrameWord::AfterTheFilesStart(after) => format!("+{after}"),
            AFrameWord::BeforeTheFilesEnd(0) => "+0".to_string(),
            AFrameWord::BeforeTheFilesEnd(before) => format!("-{before}"),
        }
    }
}

/// **L17: the render instruction is the animation form, never the
/// single-frame form, even for a job covering one frame.** Kept by there
/// being one list type and one argument for it, and no still-image route
/// anywhere in this crate.
pub const EVERY_JOB_RENDERS_AS_AN_ANIMATION: bool = true;

/// **L17, `Job files and outputs` C2: Blender's own skipping is never
/// used.** Its test is bare existence at one path, an empty file counts as
/// done, and it does not reach file output nodes. What decides which frames
/// an attempt renders is the reckoning's list and the skip switch is imposed
/// off in the material ([`crate::material`]).
pub const BLENDERS_OWN_SKIPPING_IS_NEVER_USED: bool = true;

#[cfg(test)]
mod tests {
    use super::*;

    /// A saved range no frame of these tests reaches past.
    const ANYWHERE: TheFilesSavedRange = TheFilesSavedRange { start: -1000, end: 1000 };

    #[test]
    fn the_list_keeps_the_asked_order_and_names_each_frame_once() {
        // The spec's own typed example, `40, 22-25`, rendered in the order
        // written and never sorted.
        let list = ExplicitFrameList::from_the_reckonings_integers(&[40, 22, 23, 24, 25]).unwrap();
        assert_eq!(list.in_the_asked_order(), &[40, 22, 23, 24, 25]);
        assert_eq!(list.as_argument_text(ANYWHERE).unwrap(), "40,22..25");
        assert_eq!(list.the_last_asked_for(), 25);
    }

    /// **The fourth of the four faults this step exists against, from the
    /// list's side**: a frame named twice would be rendered twice and paid
    /// for twice.
    #[test]
    fn a_frame_named_twice_is_emitted_once_at_its_first_place() {
        let list = ExplicitFrameList::from_the_reckonings_integers(&[5, 1, 9, 5, 1]).unwrap();
        assert_eq!(list.in_the_asked_order(), &[5, 1, 9]);
        assert_eq!(list.as_argument_text(ANYWHERE).unwrap(), "5,1,9");
    }

    #[test]
    fn a_stretch_is_three_or_more_consecutive_frames_and_a_pair_is_two_singles() {
        let list = ExplicitFrameList::from_the_reckonings_integers(&[1, 2, 10, 11, 12, 3]).unwrap();
        assert_eq!(list.as_argument_text(ANYWHERE).unwrap(), "1,2,10..12,3");
        let one = ExplicitFrameList::from_the_reckonings_integers(&[7]).unwrap();
        assert_eq!(one.as_argument_text(ANYWHERE).unwrap(), "7");
        assert_eq!(one.len(), 1);
        assert!(!one.is_empty());
    }

    /// **L17b: a frame below zero is never a bare negative number.** On a file
    /// saved at −5 to 5, −3 to 0 is the stretch below zero written from the
    /// saved start, split from frame 0, which stays itself — and not the
    /// `-3..0` this list was handed until the video-range piece, which rendered nothing and
    /// reported success. The file's own −5 is `+0`, never a bare 0.
    #[test]
    fn a_frame_below_zero_is_written_after_the_files_saved_start_and_never_bare() {
        let saved = TheFilesSavedRange { start: -5, end: 5 };
        let list = |asked: &[i64]| ExplicitFrameList::from_the_reckonings_integers(asked).unwrap();
        assert_eq!(list(&[-3, -2, -1, 0]).as_argument_text(saved).unwrap(), "+2..+4,0");
        assert_eq!(
            list(&(-5..=5).collect::<Vec<i64>>()).as_argument_text(saved).unwrap(),
            "+0..+4,0..5"
        );
        assert_eq!(list(&[-3]).as_argument_text(saved).unwrap(), "+2");
        assert_eq!(list(&[-1, 0, 1]).as_argument_text(saved).unwrap(), "+4,0,1");
        assert_eq!(list(&[4, -4, -2]).as_argument_text(saved).unwrap(), "4,+1,+3");
        for asked in [vec![-3, -2, -1, 0], (-5..=5).collect(), vec![-1, 0, 1], vec![4, -4, -2]] {
            let text = list(&asked).as_argument_text(saved).unwrap();
            assert!(
                !text.split([',', '.']).any(|word| word.starts_with('-')),
                "a bare negative in {text}"
            );
        }
        // Frames at or above zero are written exactly as before, whatever the
        // file's range.
        assert_eq!(list(&[0, 1, 2, 3]).as_argument_text(saved).unwrap(), "0..3");
    }

    /// **L17b: a frame below zero outside the file's saved range has no word**,
    /// so the list is refused rather than handed a guess.
    #[test]
    fn a_frame_below_zero_outside_the_files_saved_range_is_refused() {
        let saved = TheFilesSavedRange { start: 1, end: 250 };
        let list = ExplicitFrameList::from_the_reckonings_integers(&[-2, -1, 0, 1]).unwrap();
        assert_eq!(
            list.as_argument_text(saved),
            Err(ABareNegativeFrame { frame: -2, from: saved })
        );
        let past_the_end = TheFilesSavedRange { start: -10, end: -5 };
        assert_eq!(
            ExplicitFrameList::from_the_reckonings_integers(&[-3]).unwrap().as_argument_text(past_the_end),
            Err(ABareNegativeFrame { frame: -3, from: past_the_end })
        );
    }

    /// **The end of a video's run is written from the file's saved end**, which
    /// is what a sign on the end argument moves; the start, like a list, from
    /// the saved start.
    #[test]
    fn a_runs_end_below_zero_is_written_before_the_files_saved_end() {
        let saved = TheFilesSavedRange { start: -5, end: 5 };
        assert_eq!(AFrameWord::at_the_end(-2, saved).map(AFrameWord::as_text), Ok("-7".to_string()));
        assert_eq!(AFrameWord::at_the_end(0, saved).map(AFrameWord::as_text), Ok("0".to_string()));
        assert_eq!(AFrameWord::in_a_list(-4, saved).map(AFrameWord::as_text), Ok("+1".to_string()));
        let ends_below = TheFilesSavedRange { start: -5, end: -1 };
        assert_eq!(AFrameWord::at_the_end(-1, ends_below).map(AFrameWord::as_text), Ok("+0".to_string()));
        assert_eq!(AFrameWord::in_a_list(-5, ends_below).map(AFrameWord::as_text), Ok("+0".to_string()));
        assert!(AFrameWord::at_the_end(-6, ends_below).is_err());
    }

    /// **L17a's step: an evenly stepped ascending list is one run with its
    /// step; a list with gaps, or out of order, is not.**
    #[test]
    fn an_evenly_stepped_list_is_one_run_and_a_list_with_gaps_is_not() {
        let run = |asked: &[i64]| ExplicitFrameList::from_the_reckonings_integers(asked).unwrap().as_one_stepped_run();
        assert_eq!(run(&(1..=90).collect::<Vec<i64>>()), Some(OneSteppedRun { first: 1, last: 90, step: 1 }));
        assert_eq!(run(&[1, 3, 5, 7, 9]), Some(OneSteppedRun { first: 1, last: 9, step: 2 }));
        assert_eq!(run(&[7]), Some(OneSteppedRun { first: 7, last: 7, step: 1 }));
        assert_eq!(run(&[-3, -2, -1, 0]), Some(OneSteppedRun { first: -3, last: 0, step: 1 }));
        for gapped in [vec![1, 2, 3, 20, 21], vec![1, 3, 4], vec![3, 2, 1], vec![40, 22, 23, 24, 25]] {
            assert_eq!(run(&gapped), None, "{gapped:?}");
        }
    }

    #[test]
    fn an_empty_list_is_not_composed_because_there_is_nothing_to_name() {
        assert_eq!(
            ExplicitFrameList::from_the_reckonings_integers(&[]),
            Err(NoList::NothingToName)
        );
    }

    /// **The list never carries a whole-range switch beside it**: the text
    /// is frames and separators and nothing else.
    #[test]
    fn the_argument_text_is_frames_and_separators_and_nothing_else() {
        let list = ExplicitFrameList::from_the_reckonings_integers(&[1, 2, 3, 4, 8]).unwrap();
        let text = list.as_argument_text(ANYWHERE).unwrap();
        assert!(text
            .chars()
            .all(|c| c.is_ascii_digit() || c == ',' || c == '.' || c == '-' || c == '+'));
        const _: () = assert!(EVERY_JOB_RENDERS_AS_AN_ANIMATION);
        const _: () = assert!(BLENDERS_OWN_SKIPPING_IS_NEVER_USED);
    }
}
