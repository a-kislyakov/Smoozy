//! The frame range, and the tidy — `Render Settings` §3 F1–F5.
//!
//! **Constraint:** Row 146. Frame lists are taken in the order written, may
//! mix single frames and stretches, and repeat a frame named twice; **a
//! malformed frame list renders nothing at all and the run reports success**
//! (*Frames 7*). `Rendering` L18: the list is composed from integers and
//! never from text, and no frame appears in it twice.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rendering` L18, P1. `Job files and outputs`
//! §10, M1a.

use serde::{Deserialize, Serialize};

/// **F1: the parsed integer form is the only source for numbers.** The
/// accepted text is kept separately for display only. A first-last-step triple is held as three integers; a written
/// list is held as the tidied list — repeats dropped, ascending order — of
/// integers. **Blender is never handed the user's text.**
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FrameForm {
    Triple {
        first: i64,
        last: i64,
        step: i64,
    },
    /// Tidied: ascending, no repeats.
    List(Vec<i64>),
}

/// **What cannot be parsed as a range never enters the store — the spec's own
/// four, and J11 item 2's fifth**, refused in place, the setting staying on
/// its last good value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameRefusal {
    /// A last frame before the first — in a run alone, or inside a run of the
    /// list.
    ALastFrameBeforeTheFirst,
    /// **The spec's *step of zero*, and its negative neighbour with it.** F2's
    /// count *mirrors the documented step rule — the step counts forward from
    /// the first frame*, which is arithmetic defined for a forward step only;
    /// a step below one has no count under that rule and is refused under the
    /// spec's own item rather than given one of its own.
    AStepBelowOne,
    AnEmptyList,
    AStrayCharacter,
    /// **J11 item 2's fifth item, added 2026-09-05: a range or list covering
    /// more than one million frames cannot be composed and is refused in
    /// place.** The bound is Smoozy's own and states nothing about Blender —
    /// J12's line for it states the bound alone.
    MoreFramesThanCanBeComposed,
}

/// **J11 item 2's bound.** One million frames is over eleven hours of finished
/// film at twenty-four a second, so no real job reaches it, and a million
/// whole numbers is eight megabytes. The store is process memory for the life
/// of the run (J2), so an unbounded set would be built at the moment the text
/// is accepted and would take every setting typed this sitting with it.
pub const THE_MOST_FRAMES_A_SET_MAY_COVER: u64 = 1_000_000;

/// One entry of the field: a single frame, or a run with its own step.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AnEntry {
    One(i64),
    ARun { first: i64, last: i64, step: i64 },
}

/// **F1: the whole frame field, parsed once, at the moment the text is
/// accepted.** One field, not two — the Design document draws a single box
/// and this is its one entry point. It reads the four drawn forms:
///
/// - a plain run, `1-240`;
/// - a run with a step, `1-120x2`;
/// - a list, `1, 3, 7`;
/// - and a mix, `1-10, 22-25`.
///
/// **A run alone is [`FrameForm::Triple`], with or without a step** — a plain
/// run is a step of one — because F1 holds a first-last-step run as three
/// integers. **A list or a mix is [`FrameForm::List`]**, tidied once: repeats
/// dropped, ascending order, a run inside it expanded under the documented
/// step rule. A frame may be zero or negative (V11), so a run's two ends are
/// each a signed whole number and the run's dash is the one between them.
pub fn parse_the_frame_field(text: &str) -> Result<FrameForm, FrameRefusal> {
    if text.trim().is_empty() {
        return Err(FrameRefusal::AnEmptyList);
    }
    let mut entries = Vec::new();
    for entry in text.split(',') {
        let entry = entry.trim();
        if entry.is_empty() {
            // `1,,3` or a trailing comma: a stray character rather than an
            // empty list, which is text with nothing in it at all.
            return Err(FrameRefusal::AStrayCharacter);
        }
        entries.push(one_entry(entry)?);
    }

    // **J11 item 2's fifth item: the set is counted in arithmetic before
    // anything is built.** What is counted here is the size of the thing that
    // would be built — every run's own count added up — and not F2's count,
    // which is taken after the tidy. The two differ only where entries name
    // the same frame twice, and counting after the tidy would mean building
    // the very set this bound exists to stop.
    let mut how_many: u64 = 0;
    for entry in &entries {
        how_many = how_many.saturating_add(match entry {
            AnEntry::One(_) => 1,
            AnEntry::ARun { first, last, step } => how_many_a_run_covers(*first, *last, *step),
        });
        if how_many > THE_MOST_FRAMES_A_SET_MAY_COVER {
            return Err(FrameRefusal::MoreFramesThanCanBeComposed);
        }
    }

    if let [AnEntry::ARun { first, last, step }] = entries[..] {
        return Ok(FrameForm::Triple { first, last, step });
    }
    let mut frames = Vec::with_capacity(how_many as usize);
    for entry in &entries {
        match entry {
            AnEntry::One(frame) => frames.push(*frame),
            AnEntry::ARun { first, last, step } => {
                frames.extend((*first..=*last).step_by(*step as usize))
            }
        }
    }
    frames.sort_unstable();
    frames.dedup();
    Ok(FrameForm::List(frames))
}

/// One entry of the field: a single frame, or a run `a-b` with an optional
/// `x` and a step, where either end may be negative.
fn one_entry(entry: &str) -> Result<AnEntry, FrameRefusal> {
    let (first, rest) = a_signed_number_at_the_front(entry)?;
    if rest.trim().is_empty() {
        return Ok(AnEntry::One(first));
    }
    // A run: the separator, then the second end, then an optional step.
    let Some(after_dash) = rest.trim_start().strip_prefix('-') else {
        return Err(FrameRefusal::AStrayCharacter);
    };
    let (last, rest) = a_signed_number_at_the_front(after_dash.trim_start())?;
    let rest = rest.trim();
    let step = match rest.strip_prefix('x') {
        Some(digits) => whole_number(digits)?,
        None if rest.is_empty() => 1,
        None => return Err(FrameRefusal::AStrayCharacter),
    };
    if last < first {
        return Err(FrameRefusal::ALastFrameBeforeTheFirst);
    }
    if step < 1 {
        return Err(FrameRefusal::AStepBelowOne);
    }
    Ok(AnEntry::ARun { first, last, step })
}

/// A signed whole number at the front of the text, and whatever follows it.
fn a_signed_number_at_the_front(text: &str) -> Result<(i64, &str), FrameRefusal> {
    let bytes = text.as_bytes();
    let mut at = 0;
    if at < bytes.len() && bytes[at] == b'-' {
        at += 1;
    }
    let digits_from = at;
    while at < bytes.len() && bytes[at].is_ascii_digit() {
        at += 1;
    }
    if at == digits_from {
        return Err(FrameRefusal::AStrayCharacter);
    }
    Ok((whole_number(&text[..at])?, &text[at..]))
}

/// **F2's arithmetic for one run** — the step counts forward from the first
/// frame and the last renders only where the step lands on it (*Frames 8*).
/// **The subtraction is done in a wider type than the frames themselves**: a
/// first frame far below zero and a last far above it differ by more than a
/// frame number can hold, and a run is reachable from typed text.
fn how_many_a_run_covers(first: i64, last: i64, step: i64) -> u64 {
    let how_many = ((last as i128 - first as i128) / step as i128) + 1;
    // The widest run of all — every frame number there is — is one more than
    // a `u64` holds, so the answer saturates rather than wrapping to nothing.
    u64::try_from(how_many).unwrap_or(u64::MAX)
}

fn whole_number(text: &str) -> Result<i64, FrameRefusal> {
    let text = text.trim();
    let digits = text.strip_prefix('-').unwrap_or(text);
    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(FrameRefusal::AStrayCharacter);
    }
    text.parse().map_err(|_| FrameRefusal::AStrayCharacter)
}

impl FrameForm {
    /// **F2: the tidied list and the counted list are the same object.** A
    /// triple's count is arithmetic on the three integers, mirroring the
    /// documented step rule — the step counts forward from the first frame and
    /// the last renders only where the step lands on it (*Frames 8*).
    pub fn count(&self) -> u64 {
        match self {
            FrameForm::Triple { first, last, step } => how_many_a_run_covers(*first, *last, *step),
            FrameForm::List(frames) => frames.len() as u64,
        }
    }

    /// **V11: the numerical minimum of the frame set** — a triple's first
    /// integer, a tidied list's first element. Lowest and first are the same
    /// value in the store, and that is F1's doing.
    pub fn lowest(&self) -> Option<i64> {
        match self {
            FrameForm::Triple { first, .. } => Some(*first),
            FrameForm::List(frames) => frames.first().copied(),
        }
    }

    /// **F3: the explicit frame list the reckoning composes from the parsed
    /// form** (`Job files and outputs` §10, `Rendering` P1 step 6, L18) —
    /// each frame once, ascending, the step counting forward from the first.
    pub fn explicit_list(&self) -> Vec<i64> {
        match self {
            FrameForm::Triple { first, last, step } => {
                (*first..=*last).step_by(*step as usize).collect()
            }
            FrameForm::List(frames) => frames.clone(),
        }
    }

    /// **Y9: the manifest's frame-range field, the user's parsed integers**
    /// (`Job files and outputs` M1a's first form).
    pub fn as_the_manifests_field(&self) -> smoozy_job_files_machine::manifest::FrameRange {
        smoozy_job_files_machine::manifest::FrameRange::TheUsersParsedIntegers(self.explicit_list())
    }
}

/// **F2's group count**: with more than one scene ticked the count is the
/// group's — the group entry's count once for every ticked scene where a range
/// was typed, and each ticked scene's own range counted and added up where
/// none was, read per scene from the reading in force.
pub fn the_group_count(group_entry: Option<&FrameForm>, ticked_scenes_own_counts: &[u64]) -> u64 {
    match group_entry {
        Some(entry) => entry.count() * ticked_scenes_own_counts.len() as u64,
        None => ticked_scenes_own_counts.iter().sum(),
    }
}

/// **F4: an untouched range is no entry at all**, and the job's frames are
/// then the file's own range, resolved at each attempt from the reading in
/// force (Y9's second form). Kept by [`Option`] on every entry and by
/// [`crate::payload::Payload::the_manifests_frame_range`].
pub const AN_UNTOUCHED_RANGE_IS_NO_ENTRY: () = ();

/// **F5: which entry a job's payload composition reads, stated once.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhichEntryCompositionReads {
    /// More than one scene was ticked at submission: the group entry of J3,
    /// imposed on every ticked scene alike without ever being written into a
    /// scene's key.
    TheGroupEntry,
    /// One scene in hand: that scene's own per-scene entry.
    TheScenesOwn,
    /// No scene in hand — the job carries no scene name: J3's pre-read answer
    /// set, and the same composition serves that set's other entries.
    ThePreReadAnswerSet,
}

/// **F5, performed** over the page's state at submission.
pub fn which_entry_composition_reads(
    scenes_ticked: usize,
    a_scene_is_in_hand: bool,
) -> WhichEntryCompositionReads {
    if scenes_ticked > 1 {
        WhichEntryCompositionReads::TheGroupEntry
    } else if a_scene_is_in_hand {
        WhichEntryCompositionReads::TheScenesOwn
    } else {
        WhichEntryCompositionReads::ThePreReadAnswerSet
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_one_field_reads_all_four_drawn_forms() {
        // A plain run is a run with a step of one, and both are the triple.
        assert_eq!(
            parse_the_frame_field("1-240"),
            Ok(FrameForm::Triple {
                first: 1,
                last: 240,
                step: 1
            })
        );
        // The Design document's own drawn value.
        assert_eq!(
            parse_the_frame_field("1-120x2"),
            Ok(FrameForm::Triple {
                first: 1,
                last: 120,
                step: 2
            })
        );
        assert_eq!(
            parse_the_frame_field(" 10 - 50 x 2 "),
            Ok(FrameForm::Triple {
                first: 10,
                last: 50,
                step: 2
            })
        );
        // A list, and a mix of single frames and runs.
        assert_eq!(
            parse_the_frame_field("1, 3, 7"),
            Ok(FrameForm::List(vec![1, 3, 7]))
        );
        assert_eq!(
            parse_the_frame_field("1-10, 22-25"),
            Ok(FrameForm::List(vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 22, 23, 24, 25
            ]))
        );
        // A run inside a mix is expanded under the documented step rule.
        assert_eq!(
            parse_the_frame_field("1-9x4, 100"),
            Ok(FrameForm::List(vec![1, 5, 9, 100]))
        );
        // A lone frame is a list of one, not a run.
        assert_eq!(parse_the_frame_field("7"), Ok(FrameForm::List(vec![7])));
    }

    #[test]
    fn a_list_is_tidied_once_repeats_dropped_and_ascending() {
        // The spec's own example: `40, 22-25` covers 22, 23, 24, 25 and 40.
        assert_eq!(
            parse_the_frame_field("40, 22-25"),
            Ok(FrameForm::List(vec![22, 23, 24, 25, 40]))
        );
        // A frame named twice, and two overlapping runs, are the same frames
        // named more than once.
        assert_eq!(
            parse_the_frame_field("3, 1-3, 2-4, 3"),
            Ok(FrameForm::List(vec![1, 2, 3, 4]))
        );
        // Negative frames, in a run and alone.
        assert_eq!(
            parse_the_frame_field("-5--3, -1"),
            Ok(FrameForm::List(vec![-5, -4, -3, -1]))
        );
    }

    #[test]
    fn the_specs_four_refusals_hold_for_every_form_of_the_field() {
        assert_eq!(parse_the_frame_field(""), Err(FrameRefusal::AnEmptyList));
        assert_eq!(parse_the_frame_field("   "), Err(FrameRefusal::AnEmptyList));
        assert_eq!(
            parse_the_frame_field("10-5"),
            Err(FrameRefusal::ALastFrameBeforeTheFirst)
        );
        assert_eq!(
            parse_the_frame_field("50-10, 1"),
            Err(FrameRefusal::ALastFrameBeforeTheFirst)
        );
        // A step below one is the spec's own *step of zero* item, not a new
        // refusal.
        assert_eq!(
            parse_the_frame_field("1-10x0"),
            Err(FrameRefusal::AStepBelowOne)
        );
        assert_eq!(
            parse_the_frame_field("1-10x-1"),
            Err(FrameRefusal::AStepBelowOne)
        );
        for stray in [
            "1, x", "1,,3", "1-2-3", "1.5", "1-1O", "1-10x", "1-10y2", "x2", "1-10 2",
        ] {
            assert_eq!(
                parse_the_frame_field(stray),
                Err(FrameRefusal::AStrayCharacter),
                "{stray}"
            );
        }
    }

    #[test]
    fn a_set_too_large_to_compose_is_refused_before_anything_is_built() {
        // J11 item 2's fifth item. A million frames exactly is accepted; one
        // more is not, in a run, in a list, and across a mix.
        assert!(parse_the_frame_field("1-1000000").is_ok());
        assert_eq!(
            parse_the_frame_field("1-1000001"),
            Err(FrameRefusal::MoreFramesThanCanBeComposed)
        );
        assert_eq!(
            parse_the_frame_field("1-10000000000"),
            Err(FrameRefusal::MoreFramesThanCanBeComposed)
        );
        assert_eq!(
            parse_the_frame_field("1-600000, 700000-1400000"),
            Err(FrameRefusal::MoreFramesThanCanBeComposed)
        );
        // A step keeps a wide run inside the bound.
        assert!(parse_the_frame_field("1-10000000x100").is_ok());
        // The widening: a first frame far below zero and a last far above it
        // differ by more than a frame number can hold, and this is reachable
        // from typed text now that a run is the triple.
        assert_eq!(
            parse_the_frame_field("-9223372036854775808-9223372036854775807"),
            Err(FrameRefusal::MoreFramesThanCanBeComposed)
        );
        assert_eq!(
            FrameForm::Triple {
                first: i64::MIN,
                last: i64::MAX,
                step: 1
            }
            .count(),
            u64::MAX
        );
    }

    #[test]
    fn the_count_mirrors_the_documented_step_rule_and_the_list_counts_itself() {
        // 10–50 step 2 covers 10, 12, … 50: 21 frames. Step 3 stops at 49: 14.
        assert_eq!(parse_the_frame_field("10-50x2").unwrap().count(), 21);
        assert_eq!(parse_the_frame_field("10-50x3").unwrap().count(), 14);
        assert_eq!(
            parse_the_frame_field("10-50x3")
                .unwrap()
                .explicit_list()
                .last(),
            Some(&49)
        );
        assert_eq!(parse_the_frame_field("40, 22-25").unwrap().count(), 5);
        // A run of one frame is an ordinary run, and a negative frame is
        // allowed.
        assert_eq!(parse_the_frame_field("-5--5").unwrap().count(), 1);
    }

    #[test]
    fn the_lowest_frame_is_the_first_integer_wherever_it_was_written() {
        assert_eq!(
            parse_the_frame_field("40, 22-25").unwrap().lowest(),
            Some(22)
        );
        assert_eq!(parse_the_frame_field("-3-3").unwrap().lowest(), Some(-3));
    }

    #[test]
    fn the_manifests_field_is_the_explicit_integers_and_never_text() {
        use smoozy_job_files_machine::manifest::FrameRange;
        assert_eq!(
            parse_the_frame_field("1-5x2")
                .unwrap()
                .as_the_manifests_field(),
            FrameRange::TheUsersParsedIntegers(vec![1, 3, 5])
        );
    }

    #[test]
    fn the_group_count_is_the_typed_range_once_per_scene_or_the_scenes_own_added_up() {
        let group = parse_the_frame_field("1-10").unwrap();
        assert_eq!(the_group_count(Some(&group), &[100, 200, 300]), 30);
        assert_eq!(the_group_count(None, &[100, 200, 300]), 600);
    }

    #[test]
    fn which_entry_the_composition_reads_is_decided_by_the_ticked_count_and_the_scene_in_hand() {
        assert_eq!(
            which_entry_composition_reads(3, true),
            WhichEntryCompositionReads::TheGroupEntry
        );
        assert_eq!(
            which_entry_composition_reads(1, true),
            WhichEntryCompositionReads::TheScenesOwn
        );
        assert_eq!(
            which_entry_composition_reads(0, false),
            WhichEntryCompositionReads::ThePreReadAnswerSet
        );
    }
}
