//! A job that ends short delivers what it wrote, and what *which frames* is
//! — §3.7 K17–K23 (Row 310).
//!
//! **K17: only one half of the promise is this document's.** *Stay on the
//! machine* is the ending's own leaving — the render writes straight to
//! final names (K7); *can be brought down* is the entry's own routes,
//! offered whatever the ending was (`Queue & History` Y8, N10; `Download`
//! Q3); the endings themselves are `Queue & History`'s and the wording is
//! Design's. **What is decided here is what *which frames were rendered* is
//! composed from, and when it can be said.**
//!
//! **K18: it is the completeness statement's per-frame reckoning, and it is
//! not the entry's last-frame figure.** The set is derived from `Job files
//! and outputs` C27's four sets by C12 and C13 — **a frame is in *rendered*
//! when every file the inventory expects for it is present and none of them
//! is empty, and in *not rendered* otherwise** — and the two sets together
//! cover the job's whole asked range. `Queue & History` N6's figure is a
//! **boundary** in the job's asked order and a different shape of answer;
//! **the figure may not stand in for the set**, so nobody substitutes the
//! cheaper one. Held here by [`TheSet`] taking the four sets and nothing
//! taking a figure.
//!
//! **K19: composed once, on the machine, at the ending, never recomputed on
//! the desktop.** K20: the telling is a thing the desktop does when it
//! learns of the ending. K21: C15's limit — a file short but not empty
//! counts as there — and C29's — a write that ran out of room counts as
//! present — ride with the set and are not softened. **K22: an outcome
//! carrying no statement states no set, and that is different from an empty
//! one** — absent evidence is never read as evidence of absence.
//!
//! **Markers used in this file that belong to another document, each with
//! the document it belongs to.** `Job files and outputs` C12, C13, C15,
//! C27, C28, C29. `Queue & History` N6, N7, N10, Y8, H8. `Download` Q3.
//!
//! **`Rented-server runtime` S11 and S13 are not listed above and are written
//! out at each site**, because this plan defines an S11 and an S13 of its own.

use std::collections::BTreeMap;

/// One expected file of the inventory, keyed to the frame it belongs to
/// (`Job files and outputs` C27's *files expected*, one per expected file).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpectedFile {
    pub frame: i64,
    pub path: String,
}

/// **C27's four sets, as the walk produced them.** The files expected, the
/// files present, the files absent, and the files present at zero bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheFourSets {
    pub expected: Vec<ExpectedFile>,
    pub present: Vec<String>,
    pub absent: Vec<String>,
    pub present_at_zero_bytes: Vec<String>,
}

/// **K18's set: the frames rendered and the frames not, together covering
/// the whole asked range, in the asked order.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheSet {
    pub rendered: Vec<i64>,
    pub not_rendered: Vec<i64>,
}

impl TheSet {
    /// **K18, performed, by C12 and C13**: a frame is rendered when every
    /// file expected for it is present and none is empty. A frame with no
    /// expected file at all is not rendered — nothing vouches for it.
    pub fn from_the_four_sets(sets: &TheFourSets, asked_order: &[i64]) -> TheSet {
        let mut expected_per_frame: BTreeMap<i64, Vec<&str>> = BTreeMap::new();
        for file in &sets.expected {
            expected_per_frame
                .entry(file.frame)
                .or_default()
                .push(&file.path);
        }
        let mut rendered = Vec::new();
        let mut not_rendered = Vec::new();
        for frame in asked_order {
            let counts = expected_per_frame.get(frame).is_some_and(|files| {
                files.iter().all(|path| {
                    sets.present.iter().any(|p| p == path)
                        && !sets.present_at_zero_bytes.iter().any(|p| p == path)
                        && !sets.absent.iter().any(|p| p == path)
                })
            });
            if counts {
                rendered.push(*frame);
            } else {
                not_rendered.push(*frame);
            }
        }
        TheSet {
            rendered,
            not_rendered,
        }
    }

    /// The two sets together cover the whole asked range.
    pub fn covers(&self, asked_order: &[i64]) -> bool {
        self.rendered.len() + self.not_rendered.len() == asked_order.len()
            && asked_order
                .iter()
                .all(|frame| self.rendered.contains(frame) || self.not_rendered.contains(frame))
    }
}

/// **K22: what an outcome carries — a statement, or no statement at all,
/// and the second is not an empty set.** `Rented-server runtime` S13's case — the walk could not be
/// taken — and N7's — no ending was ever learned — both end with the entry
/// saying which frames it cannot state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheOutcomeCarries {
    TheSet(TheSet),
    /// No statement: the walk could not be taken, or no ending was learned.
    /// **The entry states no set, and never reports nothing rendered.**
    NoStatement,
}

impl WhatTheOutcomeCarries {
    /// **K20, K22: what the telling may say** — the set where one is
    /// carried, and which frames it cannot state where none is.
    pub fn what_the_telling_says(&self, asked_order: &[i64]) -> TheTelling {
        match self {
            WhatTheOutcomeCarries::TheSet(set) => TheTelling::TheseFrames(set.clone()),
            WhatTheOutcomeCarries::NoStatement => TheTelling::CannotState {
                frames: asked_order.to_vec(),
            },
        }
    }
}

/// The telling's shape; the words are Design's.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheTelling {
    TheseFrames(TheSet),
    /// *Which frames it cannot state*, rather than *nothing rendered*.
    CannotState {
        frames: Vec<i64>,
    },
}

/// **K19 as a rule**: composed once, on the machine, at the ending; the
/// desktop may not walk the folder itself.
pub const THE_SET_IS_COMPOSED_ONCE_ON_THE_MACHINE_AT_THE_ENDING: bool = true;

/// **K21: two limits ride with the set** — C15's short-but-not-empty file
/// counts, C29's out-of-room write counts as present — and nothing here
/// claims the set is a statement about picture quality.
pub const THE_SET_SAYS_NOTHING_ABOUT_PICTURE_QUALITY: bool = true;

/// **K18: the last-frame figure may not stand in for the set.** Kept by
/// there being no function from a figure to a [`TheSet`]; stated so it can
/// be cited.
pub const THE_FIGURE_MAY_NOT_STAND_IN_FOR_THE_SET: bool = true;

/// **K23: checked against Method's three escalations and declined** — the
/// spec states the telling and this document composes it.
pub const K23_CHECKED_AND_DECLINED: bool = true;

#[cfg(test)]
mod tests {
    use super::*;

    fn expected(frame: i64, path: &str) -> ExpectedFile {
        ExpectedFile {
            frame,
            path: path.to_string(),
        }
    }

    #[test]
    fn a_frame_is_rendered_when_every_expected_file_is_present_and_none_is_empty() {
        let sets = TheFourSets {
            expected: vec![
                expected(1, "frame_0001.png"),
                expected(1, "passes/frame_0001.exr"),
                expected(2, "frame_0002.png"),
                expected(2, "passes/frame_0002.exr"),
                expected(3, "frame_0003.png"),
                expected(3, "passes/frame_0003.exr"),
            ],
            present: vec![
                "frame_0001.png".into(),
                "passes/frame_0001.exr".into(),
                "frame_0002.png".into(),
                "frame_0003.png".into(),
                "passes/frame_0003.exr".into(),
            ],
            absent: vec!["passes/frame_0002.exr".into()],
            present_at_zero_bytes: vec!["frame_0003.png".into()],
        };
        let set = TheSet::from_the_four_sets(&sets, &[1, 2, 3]);
        assert_eq!(set.rendered, vec![1]);
        assert_eq!(set.not_rendered, vec![2, 3]);
        assert!(set.covers(&[1, 2, 3]));
    }

    /// **The fourth named fault from the set's side: a zero-byte file is
    /// never a rendered frame** — six placeholders at zero bytes were six
    /// frames the folder said were there and were not.
    #[test]
    fn a_zero_byte_file_never_counts_wherever_in_the_range_it_sits() {
        let sets = TheFourSets {
            expected: (12..=17)
                .map(|frame| expected(frame, &format!("f_{frame:04}.png")))
                .collect(),
            present: (12..=17).map(|frame| format!("f_{frame:04}.png")).collect(),
            absent: Vec::new(),
            present_at_zero_bytes: (12..=17).map(|frame| format!("f_{frame:04}.png")).collect(),
        };
        let set = TheSet::from_the_four_sets(&sets, &(12..=17).collect::<Vec<_>>());
        assert!(set.rendered.is_empty());
        assert_eq!(set.not_rendered, (12..=17).collect::<Vec<_>>());
    }

    #[test]
    fn a_frame_with_no_expected_file_is_not_rendered_and_the_asked_order_is_kept() {
        let sets = TheFourSets {
            expected: vec![expected(22, "f_0022.png"), expected(40, "f_0040.png")],
            present: vec!["f_0040.png".into(), "f_0022.png".into()],
            absent: Vec::new(),
            present_at_zero_bytes: Vec::new(),
        };
        let set = TheSet::from_the_four_sets(&sets, &[40, 22, 23]);
        assert_eq!(set.rendered, vec![40, 22]);
        assert_eq!(set.not_rendered, vec![23]);
        assert!(set.covers(&[40, 22, 23]));
    }

    /// **K22: no statement is not an empty set.**
    #[test]
    fn an_outcome_carrying_no_statement_says_which_frames_it_cannot_state_and_never_nothing_rendered(
    ) {
        let telling = WhatTheOutcomeCarries::NoStatement.what_the_telling_says(&[1, 2, 3]);
        assert_eq!(
            telling,
            TheTelling::CannotState {
                frames: vec![1, 2, 3]
            }
        );
        let empty = WhatTheOutcomeCarries::TheSet(TheSet {
            rendered: Vec::new(),
            not_rendered: vec![1, 2, 3],
        })
        .what_the_telling_says(&[1, 2, 3]);
        assert_ne!(empty, telling);
        assert!(matches!(empty, TheTelling::TheseFrames(_)));
        const _: () = assert!(THE_SET_IS_COMPOSED_ONCE_ON_THE_MACHINE_AT_THE_ENDING);
        const _: () = assert!(THE_SET_SAYS_NOTHING_ABOUT_PICTURE_QUALITY);
        const _: () = assert!(THE_FIGURE_MAY_NOT_STAND_IN_FOR_THE_SET);
        const _: () = assert!(K23_CHECKED_AND_DECLINED);
    }
}
