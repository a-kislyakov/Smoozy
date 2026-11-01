//! The per-job reporting choice, which is carried here and is not a payload
//! entry — `Render Settings` §9.5 Y16.
//!
//! **Y16: the page carries the job's reporting choice on the submit strip,
//! and hands it to the manifest's own field rather than into the payload.**
//! It is not a payload entry, and the ground is Y1's own: the payload is a
//! statement of overrides every entry of which must reach a Blender inside
//! the launch window (Y3); a reporting choice reaches no render. `Telegram`
//! V1a puts it on the manifest **beside the settings payload and never inside
//! it**, and `Job files and outputs` M1 carries the field. **What composes the
//! field is `Rendering` S5a, and this file is the source S5a reads** — nothing
//! here composes the manifest field.
//!
//! **This page decides nothing about the choice itself.** The members, their
//! order and their words are Design's (`Preparation/Design/Settings.md`,
//! *What a job reports*: five radios from 2026-10-31, steps of one decision,
//! the app-wide value the default, this page settling it per job); that the value is
//! fixed at submission and travels with it is `Telegram` V1's; the field and
//! its form are `Job files and outputs` M1 and M1a's.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rendering` S5a. `Job files and outputs` M1,
//! M1a. `Telegram` V1a, `Telegram` V1d.

use serde::{Deserialize, Serialize};

/// **The closed set of five, held as the identifier and never the
/// wording** (`Rendering` S5a). The words are Design's: everything, the
/// pictures without the running commentary, or only what needs an answer —
/// with one floor under all three, that finished, failed and waiting-on-you
/// are always sent.
///
/// **The fourth, from 2026-10-19, is `APictureEveryFewMinutes`, and it alone
/// carries a value**: `minutes`, a whole number from 1 to 60
/// ([`THE_FEWEST_MINUTES`] to [`THE_MOST_MINUTES`]). With serde's own form it
/// is written `{"APictureEveryFewMinutes":{"minutes":5}}` beside the other
/// three's bare names (`Telegram` V1a). It sends what `Everything` sends, the
/// frame pictures paced on the machine (`Telegram` I2a). The Creator's
/// want of 2026-10-19: *"a choice that the user can manually type in the
/// number of minutes is the way to do it, not some presets."*
///
/// **The app-wide value holds any of the five, minutes included, from
/// 2026-10-31** (`Telegram` V1a as amended, `Telegram` V1d): a choice made
/// from the phone with `/reporting` becomes it, and that choice can be *Off*
/// or a picture every few minutes, so `Design/Settings` draws five radios.
/// *(Until that day the app-wide value was one of three, `THE_APP_WIDE_SET`,
/// and the fourth and fifth were per job only.)*
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReportingChoice {
    Everything,
    ThePicturesWithoutTheRunningCommentary,
    APictureEveryFewMinutes { minutes: u8 },
    OnlyWhatNeedsAnAnswer,
    /// No automatic report (`Telegram` V1b).
    Nothing,
}

/// **The fewest minutes between pictures**, and the most — `Specs/Telegram`,
/// *Fewer pictures, on a clock*: *"whole minutes, from 1 to 60"*.
pub const THE_FEWEST_MINUTES: u8 = 1;
pub const THE_MOST_MINUTES: u8 = 60;
/// **The minutes a job starts from**, until the user types another: *"5 until
/// the user types another"*.
pub const THE_MINUTES_TO_BEGIN_WITH: u8 = 5;

impl ReportingChoice {
    /// **The choice in the manifest's own form, as JSON text** — what the
    /// manifest's reporting field holds, what `Telegram` V1c's request carries
    /// and what the desk's record keeps (version 55). The twenty-first notes,
    /// note 7.
    pub fn in_the_manifests_form(self) -> String {
        serde_json::to_string(&self).expect("a reporting choice is a value that always renders")
    }

    /// **That form read back**, or nothing where it is not one of the five.
    pub fn from_the_manifests_form(text: &str) -> Option<ReportingChoice> {
        serde_json::from_str(text).ok()
    }

    /// In Design's own order (`Design/Render Settings § Submitting`, *A fourth
    /// choice, per job only*): the new row third, *"because it sends fewer
    /// pictures than the two above it and more than the one below"*. The
    /// fourth carries the minutes a job starts from.
    pub const ALL: [ReportingChoice; 5] = [
        ReportingChoice::Everything,
        ReportingChoice::ThePicturesWithoutTheRunningCommentary,
        ReportingChoice::APictureEveryFewMinutes {
            minutes: THE_MINUTES_TO_BEGIN_WITH,
        },
        ReportingChoice::OnlyWhatNeedsAnAnswer,
        ReportingChoice::Nothing,
    ];
}

/// **The minutes the user typed, taken or refused** — `Specs/Telegram`, *Fewer
/// pictures, on a clock*: *"An entry that is empty, is not a whole number, or
/// is outside 1 to 60 is not taken: Smoozy says the range, and the job keeps
/// the minutes that stood before it, as every typed value on that page does"*
/// (`Render Settings` F1). Spaces around the figure are not an entry of their
/// own and are let go; nothing else is tidied.
pub fn the_minutes_typed(text: &str) -> Result<u8, TheMinutesAreWholeFrom1To60> {
    let text = text.trim();
    if text.is_empty() || !text.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(TheMinutesAreWholeFrom1To60);
    }
    match text.parse::<u64>() {
        Ok(minutes) if (u64::from(THE_FEWEST_MINUTES)..=u64::from(THE_MOST_MINUTES)).contains(&minutes) => {
            Ok(minutes as u8)
        }
        _ => Err(TheMinutesAreWholeFrom1To60),
    }
}

/// **The one refusal the minutes have**: the range, said alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheMinutesAreWholeFrom1To60;

/// **What the submit strip offers.** *"It is a quiet line on the submit strip
/// under the download choices — the word `Telegram` and one of the three
/// choices — rather than a question, because a default is carried and
/// submitting normally asks nothing. Where Telegram is switched off it is
/// not there at all."*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheSubmitStripLine {
    /// The line, carrying the app-wide value as its default.
    Offered { default: ReportingChoice },
    /// Telegram is switched off: the line is not there at all.
    NotThere,
}

pub fn the_submit_strip_line(
    telegram_is_switched_on: bool,
    the_app_wide_value: ReportingChoice,
) -> TheSubmitStripLine {
    if telegram_is_switched_on {
        TheSubmitStripLine::Offered {
            default: the_app_wide_value,
        }
    } else {
        TheSubmitStripLine::NotThere
    }
}

/// **Where the control is not drawn, the app-wide value stands and the field
/// is still written.** `Telegram` V1a requires the field present on every
/// job's manifest whether or not Telegram is switched on anywhere, so a
/// submission that offered no choice hands over the app-wide item's value and
/// never an absence. **The page never omits the hand-over, only the control.**
/// **The value is read once, at composition, and never afterwards**: it is
/// not part of any reading, not anchored, not in Y1's entry list, and no
/// re-submission carries it forward from an earlier job's manifest.
pub fn the_hand_over(
    the_users_per_job_answer: Option<ReportingChoice>,
    the_app_wide_value: ReportingChoice,
) -> ReportingChoice {
    the_users_per_job_answer.unwrap_or(the_app_wide_value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_line_is_offered_with_the_app_wide_default_and_is_not_there_where_telegram_is_off() {
        assert_eq!(
            the_submit_strip_line(true, ReportingChoice::OnlyWhatNeedsAnAnswer),
            TheSubmitStripLine::Offered {
                default: ReportingChoice::OnlyWhatNeedsAnAnswer
            }
        );
        assert_eq!(
            the_submit_strip_line(false, ReportingChoice::Everything),
            TheSubmitStripLine::NotThere
        );
    }

    #[test]
    fn the_hand_over_is_the_per_job_answer_where_given_and_the_app_wide_value_otherwise_and_never_an_absence(
    ) {
        assert_eq!(
            the_hand_over(
                Some(ReportingChoice::Everything),
                ReportingChoice::OnlyWhatNeedsAnAnswer
            ),
            ReportingChoice::Everything
        );
        assert_eq!(
            the_hand_over(
                None,
                ReportingChoice::ThePicturesWithoutTheRunningCommentary
            ),
            ReportingChoice::ThePicturesWithoutTheRunningCommentary
        );
    }

    /// **The set is five from 2026-10-28, in Design's order**, Off last.
    /// *(Until that day `the_set_is_three_and_in_designs_order`; until
    /// 2026-10-31 it also held the app-wide value to three, which `Telegram`
    /// V1d ended.)*
    #[test]
    fn the_set_is_five_and_in_designs_order() {
        assert_eq!(
            ReportingChoice::ALL,
            [
                ReportingChoice::Everything,
                ReportingChoice::ThePicturesWithoutTheRunningCommentary,
                ReportingChoice::APictureEveryFewMinutes { minutes: 5 },
                ReportingChoice::OnlyWhatNeedsAnAnswer,
                ReportingChoice::Nothing,
            ]
        );
    }

    /// **`Telegram` V1d: the app-wide value may be any of the five, minutes
    /// included**, and a job with no answer of its own carries it whole.
    #[test]
    fn the_app_wide_value_may_be_any_of_the_five_and_a_job_with_no_answer_carries_it_whole() {
        for the_app_wide_value in ReportingChoice::ALL
            .into_iter()
            .chain([ReportingChoice::APictureEveryFewMinutes { minutes: 25 }])
        {
            assert_eq!(the_hand_over(None, the_app_wide_value), the_app_wide_value);
            assert_eq!(
                the_submit_strip_line(true, the_app_wide_value),
                TheSubmitStripLine::Offered { default: the_app_wide_value }
            );
        }
    }

    /// **`Telegram` V1a's form**: the fourth is the one member that carries a
    /// value, held as `{"APictureEveryFewMinutes":{"minutes":5}}`, beside the
    /// other three's bare names, and it reads back.
    #[test]
    fn the_fourth_is_held_with_its_minutes_beside_the_others_bare_names() {
        let written = serde_json::to_value(ReportingChoice::APictureEveryFewMinutes { minutes: 5 }).unwrap();
        assert_eq!(written, serde_json::json!({"APictureEveryFewMinutes": {"minutes": 5}}));
        assert_eq!(
            serde_json::from_value::<ReportingChoice>(written).unwrap(),
            ReportingChoice::APictureEveryFewMinutes { minutes: 5 }
        );
        assert_eq!(
            serde_json::to_value(ReportingChoice::Everything).unwrap(),
            serde_json::json!("Everything")
        );
    }

    /// **The minutes: whole, from 1 to 60**; everything else is refused with
    /// the range, so the minutes that stood go on standing.
    #[test]
    fn the_minutes_are_whole_from_1_to_60_and_anything_else_is_refused() {
        for (typed, taken) in [("1", 1), ("5", 5), ("60", 60), (" 12 ", 12), ("007", 7)] {
            assert_eq!(the_minutes_typed(typed), Ok(taken), "{typed:?}");
        }
        for typed in ["", " ", "0", "61", "-1", "2.5", "five", "1e1", "+3", "99999999999999999999"] {
            assert_eq!(the_minutes_typed(typed), Err(TheMinutesAreWholeFrom1To60), "{typed:?}");
        }
    }
}
