//! The previews' judgements — `Queue & History` §15 P1a's queueing rule, P6's
//! transfer class, P9's per-version second choice.
//!
//! **Constraint:** Rows 84 and 85. `Job files and outputs` §8 settles the
//! derivative set — made once, on the machine, both readers reading the
//! same set — the candidate list, the rungs, the two picks made once and
//! never recomputed. **The production, the fetch, the journalled trigger,
//! the age ending and the heading over the picture are acts and readings
//! and are the acting half's** (`smoozy-queue-history`'s `arriving_later`); what is here
//! decides nothing at run time or is arithmetic over records already held.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Job files and outputs` B17, B18, B19, B20,
//! B24. `Server protocol and transfers` T8, T9, T10. `File Browser` O4, O5.

/// **P1a's queueing rule, as arithmetic over records Smoozy wrote.** On a
/// report the supervisor queues preview work, in frame order, for every
/// frame of the completed prefix whose chosen candidate has no published
/// derivative and no production standing — queued or running — under the
/// candidate rules' own exclusions (`Job files and outputs` B20). A
/// production that failed leaves its frame owed and queued again at the
/// next report; never retried inside its own action, never on a timer, and
/// never on a free-space reading.
pub fn the_frames_owed_a_preview(
    the_attempts_list_in_order: &[i64],
    newest_frame_saved: Option<i64>,
    published: &[i64],
    standing_queued_or_running: &[i64],
    excluded_by_the_candidate_rules: &[i64],
) -> Vec<i64> {
    let Some(newest) = newest_frame_saved else {
        return Vec::new();
    };
    let Some(end) = the_attempts_list_in_order
        .iter()
        .position(|frame| *frame == newest)
    else {
        return Vec::new();
    };
    the_attempts_list_in_order[..=end]
        .iter()
        .copied()
        .filter(|frame| {
            !published.contains(frame)
                && !standing_queued_or_running.contains(frame)
                && !excluded_by_the_candidate_rules.contains(frame)
        })
        .collect()
}

/// **P3: what the event a derivative's production is journalled under says**
/// — the frame, the operation whose record holds the production, and P10a's
/// view layer name where `Job files and outputs` B31a recorded one, as the
/// bytes it is (`Job files and outputs` R4a). Added 2026-10-03, the 38's piece 3.
///
/// **One composition and one reading back, for both sides**: the machine
/// composes it with [`ThePublishedPreview::as_says`] and the desktop reads it
/// with [`ThePublishedPreview::read_back`], so neither spells a key. **Where
/// the name would carry the event past `State, identity and recovery` E6a's
/// budget, the event names the record instead** — the operation — and the
/// name is read from there, never cut short.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ThePublishedPreview {
    pub frame: i64,
    /// The preview operation whose record holds the derivative's own record.
    pub operation: String,
    /// P10a's name, where the derivative's record carries one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub view_layer: Option<Vec<u8>>,
    /// **Whether this is the frame's main copy** (P10) — the one the live
    /// fetch, the finished-job notification and `Telegram`'s *send a frame*
    /// read, and the only one they ever read. Added 2026-10-10, the Creator's
    /// seventh notes, note 4. **Written only where false**, so the main copy's
    /// words are what they were, and an event from before reads as the main
    /// copy — every frame then had one.
    #[serde(default = "an_event_from_before_names_the_main_copy", skip_serializing_if = "it_is_true")]
    pub main: bool,
    /// **Its place in the frame's set** (P4), which names the file it was
    /// published at (`Job files and outputs`' `where_this_copy_is_written_in`).
    /// Written only where it is not nought.
    #[serde(default, skip_serializing_if = "it_is_nought")]
    pub place: u32,
    /// **Where its picture came from, for a copy that names no View Layer in
    /// a frame of more than one** — the regular output path, or the File
    /// Output node by name, as `Job files and outputs` B31a records it. **Written
    /// only where present** (P3, added 2026-10-17, the Creator's tenth notes,
    /// note 2), so every other copy's words are what they were, and an event
    /// from before reads with none.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<smoozy_job_files_machine::main_output::WhereThePictureCameFrom>,
}

fn an_event_from_before_names_the_main_copy() -> bool {
    true
}

fn it_is_true(main: &bool) -> bool {
    *main
}

fn it_is_nought(place: &u32) -> bool {
    *place == 0
}

/// **The part of `State, identity and recovery` E6a's 512-byte budget the
/// event's words may take**, leaving
/// the rest for its identity, sequence, moment, kind and subject.
pub const THE_MOST_THE_WORDS_MAY_TAKE: usize = 300;

impl ThePublishedPreview {
    /// The event's words. **A name that would carry them past the budget is
    /// left to the record** — a view layer's, or a File Output node's in the
    /// source; the source's kind is never what makes them too long, so an
    /// output path is always said.
    pub fn as_says(&self) -> String {
        let whole = serde_json::to_string(self).expect("the words always render");
        if whole.len() <= THE_MOST_THE_WORDS_MAY_TAKE {
            return whole;
        }
        use smoozy_job_files_machine::main_output::WhereThePictureCameFrom;
        serde_json::to_string(&ThePublishedPreview {
            view_layer: None,
            source: self
                .source
                .clone()
                .filter(|source| matches!(source, WhereThePictureCameFrom::TheOutputPath)),
            ..self.clone()
        })
        .expect("the words always render")
    }

    /// The event's words, read back; nothing where they are not this shape.
    pub fn read_back(says: &str) -> Option<ThePublishedPreview> {
        serde_json::from_str(says).ok()
    }
}

/// **P6: the preview fetch declares its transfer class: below every
/// user-asked class and below auto-download, never starved.** Preview
/// traffic is the one transfer nobody asked for by name, so it ranks last —
/// behind manual downloads, uploads and auto-download alike — and `Server
/// protocol and transfers` T10's no-starvation guarantee keeps *catching up
/// the way an auto-download does* true on a busy line. **Lowest is what this
/// fixes, at any count**: the ordinal is not written, because `File Browser`
/// O4 exists to let later documents declare classes above it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThePreviewFetchsClass {
    pub below_every_user_asked_class: bool,
    pub below_auto_download: bool,
    pub never_starved: bool,
}

pub const THE_PREVIEW_FETCHS_CLASS: ThePreviewFetchsClass = ThePreviewFetchsClass {
    below_every_user_asked_class: true,
    below_auto_download: true,
    never_starved: true,
};

/// **P9: the rungs are `Job files and outputs` B17–B19's, and the second
/// choice is written per version off the manifest.** The beauty pass first
/// — *Combined*, one name in every version; the diffuse colour pass second,
/// whose name parts at 5.0; the latest image last, where the promise gives
/// way rather than breaking.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rung {
    TheBeautyPass,
    TheDiffuseColourPass,
    TheLatestImage,
}

impl Rung {
    pub const IN_ORDER: [Rung; 3] = [
        Rung::TheBeautyPass,
        Rung::TheDiffuseColourPass,
        Rung::TheLatestImage,
    ];

    /// **The pass name the file carries, per the manifest's Blender
    /// version's release line.** Label: observed in a test, at the plan's
    /// P9 — `DiffCol` in 4.5, `Diffuse Color` from 5.0. `None` for the last
    /// rung, which names no pass.
    pub fn the_pass_name(self, release_line: (u32, u32)) -> Option<&'static str> {
        match self {
            Rung::TheBeautyPass => Some("Combined"),
            Rung::TheDiffuseColourPass => {
                if release_line < (5, 0) {
                    Some("DiffCol")
                } else {
                    Some("Diffuse Color")
                }
            }
            Rung::TheLatestImage => None,
        }
    }
}

/// **P9's exclusions, cited**: an empty file is never a candidate; the last
/// frame asked for that is present is not a candidate on an interrupted job;
/// a video destination is never a candidate (`Job files and outputs` B20).
/// **§18 item 9: the diffuse colour pass's purpose was never established and
/// is permanently unresolved** — the second rung's value as a texture check
/// is what breaks, the first choice carries the promise alone, and the rung
/// order stands as the spec's own ruling.
pub const THE_SECOND_RUNGS_PURPOSE_IS_PERMANENTLY_UNRESOLVED: bool = true;

/// **P10: inside a multi-layer file the preview reads the Combined of the
/// file's first view layer** — the pick is `Job files and outputs` B19's,
/// made once; Design may restyle its presentation, not its pick.
pub const THE_MULTI_LAYER_PICK_IS_THE_FIRST_VIEW_LAYERS_COMBINED: bool = true;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_frames_owed_are_the_completed_prefix_less_what_is_published_standing_or_excluded() {
        let list = [1, 2, 3, 4, 5];
        assert_eq!(
            the_frames_owed_a_preview(&list, Some(4), &[1], &[3], &[]),
            vec![2, 4]
        );
        // An excluded frame is owed nothing; a frame past the prefix is not
        // yet owed.
        assert_eq!(
            the_frames_owed_a_preview(&list, Some(3), &[], &[], &[3]),
            vec![1, 2]
        );
        assert!(the_frames_owed_a_preview(&list, None, &[], &[], &[]).is_empty());
        assert!(the_frames_owed_a_preview(&list, Some(9), &[], &[], &[]).is_empty());
    }

    /// **P3's words, read back whole by the one reader**; a name that would
    /// carry them past the budget is left to the record, never cut short.
    #[test]
    fn the_published_previews_words_read_back_and_a_long_name_is_left_to_the_record() {
        let named = ThePublishedPreview {
            frame: 147,
            operation: "b1c2".to_string(),
            view_layer: Some(b"Reflections".to_vec()),
            main: true,
            place: 0,
            source: None,
        };
        let says = named.as_says();
        assert!(says.contains("\"frame\":147"), "{says}");
        // **The main copy's words are what they were** (the Creator's seventh
        // notes, note 4), and an event from before reads as the main copy.
        assert!(!says.contains("main") && !says.contains("place"), "{says}");
        let another = ThePublishedPreview { main: false, place: 2, ..named.clone() };
        assert_eq!(ThePublishedPreview::read_back(&another.as_says()), Some(another.clone()));
        assert_eq!(ThePublishedPreview::read_back(&says), Some(named.clone()));
        let none = ThePublishedPreview { view_layer: None, ..named.clone() };
        assert!(!none.as_says().contains("view_layer"), "an absent name is absent");
        let long = ThePublishedPreview {
            view_layer: Some(vec![0xff; 200]),
            ..named
        };
        let says = long.as_says();
        assert!(says.len() <= THE_MOST_THE_WORDS_MAY_TAKE);
        assert_eq!(ThePublishedPreview::read_back(&says).and_then(|read| read.view_layer), None);
        assert_eq!(ThePublishedPreview::read_back("frame 1"), None);
    }

    /// **P3: the source rides the event only where present** (the Creator's
    /// tenth notes, note 2) — the output path, or a File Output node with its
    /// name, read back whole; every other copy's words are what they were,
    /// and words from before read with none. **A node's name that would carry
    /// the event past the budget is left to the record**, never cut short.
    #[test]
    fn tenth_notes_2_the_source_rides_the_event_only_where_present() {
        use smoozy_job_files_machine::main_output::WhereThePictureCameFrom;
        use smoozy_job_files_machine::reading::ArtistsText;
        let unnamed = ThePublishedPreview {
            frame: 147,
            operation: "b1c2".to_string(),
            view_layer: None,
            main: true,
            place: 0,
            source: Some(WhereThePictureCameFrom::TheOutputPath),
        };
        let says = unnamed.as_says();
        assert!(says.contains(r#""source":"output_path""#), "{says}");
        assert_eq!(ThePublishedPreview::read_back(&says), Some(unnamed.clone()));
        let node = ThePublishedPreview {
            source: Some(WhereThePictureCameFrom::AFileOutputNode(ArtistsText::of(b"Beauty".to_vec()))),
            ..unnamed.clone()
        };
        assert_eq!(ThePublishedPreview::read_back(&node.as_says()), Some(node.clone()));
        let none = ThePublishedPreview { source: None, ..unnamed.clone() };
        assert!(!none.as_says().contains("source"), "absent where absent");
        assert_eq!(
            ThePublishedPreview::read_back(r#"{"frame":3,"operation":"x"}"#).and_then(|read| read.source),
            None,
            "words from before carry none"
        );
        let long = ThePublishedPreview {
            source: Some(WhereThePictureCameFrom::AFileOutputNode(ArtistsText::of(vec![0xff; 200]))),
            ..unnamed
        };
        let says = long.as_says();
        assert!(says.len() <= THE_MOST_THE_WORDS_MAY_TAKE);
        assert_eq!(ThePublishedPreview::read_back(&says).and_then(|read| read.source), None);
    }

    #[test]
    fn a_failed_production_is_owed_again_at_the_next_report() {
        // Nothing published for frame 2, nothing standing: owed again.
        assert_eq!(
            the_frames_owed_a_preview(&[1, 2], Some(2), &[1], &[], &[]),
            vec![2]
        );
    }

    #[test]
    fn the_preview_fetch_ranks_below_everything_asked_for_and_is_never_starved() {
        const _: () = assert!(THE_PREVIEW_FETCHS_CLASS.below_every_user_asked_class);
        const _: () = assert!(THE_PREVIEW_FETCHS_CLASS.below_auto_download);
        const _: () = assert!(THE_PREVIEW_FETCHS_CLASS.never_starved);
    }

    #[test]
    fn the_second_choices_name_parts_at_five_and_the_first_is_one_name_everywhere() {
        assert_eq!(Rung::TheBeautyPass.the_pass_name((4, 5)), Some("Combined"));
        assert_eq!(Rung::TheBeautyPass.the_pass_name((5, 2)), Some("Combined"));
        assert_eq!(
            Rung::TheDiffuseColourPass.the_pass_name((4, 5)),
            Some("DiffCol")
        );
        assert_eq!(
            Rung::TheDiffuseColourPass.the_pass_name((5, 0)),
            Some("Diffuse Color")
        );
        assert_eq!(
            Rung::TheDiffuseColourPass.the_pass_name((5, 2)),
            Some("Diffuse Color")
        );
        assert_eq!(Rung::TheLatestImage.the_pass_name((5, 0)), None);
        assert_eq!(Rung::IN_ORDER[0], Rung::TheBeautyPass);
    }
}
