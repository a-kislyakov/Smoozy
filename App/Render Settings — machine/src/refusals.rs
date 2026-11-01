//! What the page refuses, and what a refusal states — `Render Settings` §2.4
//! J11–J12.
//!
//! **Constraint:** Row 437. The design sentence — *"A number Smoozy will not
//! take turns the field red and says the range, and says what Blender would
//! otherwise have done silently"* — is stated for **every** number the page
//! refuses.
//!
//! **J11: the page refuses in exactly three places, and every refusal is F1's
//! shape**: the typed value never becomes a store entry, the entry that stood
//! goes on standing, nothing unaccepted is ever computed from (Q2a), and
//! nothing unaccepted ever composes into a payload (F5, Y1). The three are the
//! resolution percentage box, the frame field, and the samples box. Simplify's
//! three numbers are not answered on this page — `Rendering` A1d gives the
//! offer's boxes their ranges — so these three are the page's whole set.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rendering` A1d, L8a.

use std::ops::RangeInclusive;

use crate::engine::OfferedEngine;
use crate::frames::FrameRefusal;

/// **Y1, J11 item 1: whole numbers from 1 to 32767 and nothing else** — the
/// one range the setting has (*Settings 14*), settled 2026-08-10.
pub const PERCENTAGE_RANGE: RangeInclusive<u32> = 1..=32767;

/// **J11 item 3: Cycles' final-render count takes 1 to 16,777,216** (*Settings
/// 18*, read out of each running Blender, identical in all four versions on
/// both machines).
pub const CYCLES_COUNT_RANGE: RangeInclusive<u64> = 1..=16_777_216;

/// **J11 item 3: EEVEE's takes 1 to 2,147,483,647** (*Settings 18*).
pub const EEVEE_COUNT_RANGE: RangeInclusive<u64> = 1..=2_147_483_647;

/// **J11 item 3: the range is the chosen engine's own, and it is not one line
/// for this page.** **The engine the range is taken from is the user's pick,
/// and never a reading's engine**: taking a range from a reading would be J4's
/// forbidden arrow. **Where they have not picked, the box holds to the
/// narrower of the two** — inside both engines' ranges, so a count accepted
/// with no pick standing is in range whatever engine the job turns out to
/// run. What it costs is a refusal of counts above sixteen million on a job
/// that would have run under EEVEE, and the remedy is the pick, on the page
/// the user is already standing on.
pub fn the_count_range(the_users_pick: Option<OfferedEngine>) -> RangeInclusive<u64> {
    match the_users_pick {
        Some(OfferedEngine::Eevee) => EEVEE_COUNT_RANGE,
        Some(OfferedEngine::Cycles) | None => CYCLES_COUNT_RANGE,
    }
}

/// Every refusal the page makes, told apart because J12 states them apart.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Refusal {
    /// A whole number outside 1 to 32767.
    PercentageOutOfRange {
        typed: i128,
    },
    /// A fraction, text, or a number too large for a machine whole number —
    /// **three refusals rather than one** in the plan's words, told apart from
    /// the range by what Blender does with each (J12).
    PercentageNotAWholeNumber,
    TheFrameField(FrameRefusal),
    SamplesNotAWholeNumber,
    /// A whole number outside the range the engine in force declares.
    SamplesOutOfRange {
        typed: i128,
        range: RangeInclusive<u64>,
    },
    /// **J11 item 3's fourth bullet: an act moving the engine in force while a
    /// count above the resulting engine's ceiling stands is refused in place**
    /// — nothing becomes an entry, the entry that stood goes on standing, and
    /// nothing the user answered is withdrawn or corrected behind them.
    /// **The collision has two directions and this one variant serves both**
    /// (corrected 2026-09-05): changing the pick, and clearing it. Clearing or
    /// retyping the count is what lets either act through.
    EnginePickWhileACountAboveItsCeilingStands {
        standing_count: u64,
        the_picks_range: RangeInclusive<u64>,
    },
}

/// **J11 item 1: the resolution percentage box.**
pub fn percentage_box(text: &str) -> Result<u32, Refusal> {
    let typed = a_whole_number(text).ok_or(Refusal::PercentageNotAWholeNumber)?;
    match u32::try_from(typed) {
        Ok(value) if PERCENTAGE_RANGE.contains(&value) => Ok(value),
        _ => Err(Refusal::PercentageOutOfRange { typed }),
    }
}

/// **J11 item 3: the samples box** — a value that is not a whole number, and a
/// whole number outside the range the engine in force declares.
///
/// **Why it is imposed at the box, and why nothing below the box could have
/// caught it**: a count outside the range is corrected to the nearest limit
/// at assignment, with no error, no warning and no line of any kind, and the
/// render then goes ahead and is paid for at the corrected number. Nothing
/// downstream sees it, so entry is the only place it can be caught.
pub fn samples_box(text: &str, the_users_pick: Option<OfferedEngine>) -> Result<u64, Refusal> {
    let typed = a_whole_number(text).ok_or(Refusal::SamplesNotAWholeNumber)?;
    let range = the_count_range(the_users_pick);
    match u64::try_from(typed) {
        Ok(value) if range.contains(&value) => Ok(value),
        _ => Err(Refusal::SamplesOutOfRange { typed, range }),
    }
}

/// **J11 item 3: the engine control is held to the standing count by the same
/// test.** This is the first of the collision's two directions: changing the
/// pick from EEVEE to Cycles while a count above Cycles' ceiling stands.
pub fn engine_pick_against_the_standing_count(
    pick: OfferedEngine,
    standing_count: Option<u64>,
) -> Result<OfferedEngine, Refusal> {
    the_engine_in_force_against_the_standing_count(Some(pick), standing_count)?;
    Ok(pick)
}

/// **J11 item 3's second direction, corrected 2026-09-05: clearing the pick
/// moves the engine in force too.** It goes from the picked engine to the
/// file's own, and the range the box holds to follows the pick, so clearing
/// an EEVEE pick would drop the range in force to the no-pick ceiling with a
/// count above it left standing behind — which the clause's *a store entry
/// once accepted is never re-tested* then guarantees nothing catches.
/// **So the clear is refused in place, on the act**: the cross does not take
/// effect, the pick goes on standing, and the line says what is out of range.
/// Clearing or retyping the count is what lets the clear through.
///
/// It takes the standing count alone, because the range it tests against is
/// the no-pick one by [`the_count_range`]'s own rule.
pub fn clearing_the_engine_pick_against_the_standing_count(
    standing_count: Option<u64>,
) -> Result<(), Refusal> {
    the_engine_in_force_against_the_standing_count(None, standing_count)
}

/// The one test both directions are: the range the engine in force would
/// declare after the act, against the count standing before it.
fn the_engine_in_force_against_the_standing_count(
    the_pick_after_the_act: Option<OfferedEngine>,
    standing_count: Option<u64>,
) -> Result<(), Refusal> {
    let range = the_count_range(the_pick_after_the_act);
    match standing_count {
        Some(count) if !range.contains(&count) => {
            Err(Refusal::EnginePickWhileACountAboveItsCeilingStands {
                standing_count: count,
                the_picks_range: range,
            })
        }
        _ => Ok(()),
    }
}

/// **A store entry once accepted is never re-tested and never dropped**, here
/// or anywhere: J5's deletion is the user's own act and there is no other
/// route out of the store. Kept by there being no function in this crate that
/// takes an accepted value back through a box.
pub const AN_ACCEPTED_ENTRY_IS_NEVER_RE_TESTED: () = ();

/// The whole-number gate every box shares: digits with an optional sign, and
/// nothing else — a decimal, a word, text that looks like a number, or nothing
/// at all is not a whole number.
fn a_whole_number(text: &str) -> Option<i128> {
    let text = text.trim();
    let digits = text.strip_prefix('-').unwrap_or(text);
    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    text.parse().ok()
}

/// **J12: what each refusal states is established per setting, and where
/// nothing is established the line states the range alone.** Every fact
/// below is Blender Research's, cited at the question the map names for it;
/// none is composed from another setting's evidence. **The exact wording the
/// user reads is Design's; what may be claimed is this.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    /// **Blender refuses nothing.** The value is corrected the instant it is
    /// assigned — below the range stores the lower limit, above it the upper —
    /// with no error, no warning and no line of any kind, and the render then
    /// goes ahead and is paid for at the corrected value. The percentage's is
    /// *Settings 15*'s (both machines, all four versions, with a control);
    /// the count's is *Settings 18*'s, the same way.
    CorrectedSilentlyAtAssignmentAndPaidFor { lower: u64, upper: u64 },
    /// **Refused, not rounded, with an error naming the setting** — and had the
    /// box taken it, the refusal would land inside the launch material, where
    /// `Rendering` L8a's exit-status switch makes it an attempt ending at a
    /// failure status with nothing written: **a job that fails, not a job that
    /// quietly renders at another value.**
    RefusedWithAnErrorNamingTheSettingSoTheAttemptWouldFail,
    /// **The frame field, the list forms**: Blender handed a list it cannot
    /// read **renders nothing at all and still exits 0** (*Frames 7*). A run
    /// reporting success having rendered nothing is the whole of what
    /// refusing in place buys.
    RendersNothingAtAllAndStillExitsZero,
    /// **A step of 0 is accepted, reads back as 0, and writes the first frame
    /// once** (*Frames 8*, posed as a scene setting and as a command-line
    /// argument, never through an explicit list — the bound travels with the
    /// statement).
    AStepOfZeroWritesTheFirstFrameOnce,
    /// **An end below the start renders nothing at all and exits 0** (*Frames
    /// 8*, the same bound).
    AnEndBelowTheStartRendersNothingAndExitsZero,
    /// Where nothing is established for the number itself, the line states
    /// the range alone.
    TheRangeAlone { lower: u64, upper: u64 },
}

/// **The largest figure this plan states for these settings** — EEVEE's
/// declared count ceiling, which is [`EEVEE_COUNT_RANGE`]'s own end.
///
/// **J12 draws the boundary between what Blender corrects in silence and what
/// it refuses loudly at *what a machine whole number can hold*, and nowhere
/// says what that width is.** Above the largest figure this plan states,
/// nothing here establishes which side of that boundary a number falls on, so
/// the statement is the plan's own rule for anything unestablished: the range
/// alone. **The real width stays permanently unresolved, and what breaks is
/// stated rather than scheduled** — between the real width and this boundary,
/// Smoozy cannot say whether Blender would have corrected the number in
/// silence or refused it loudly, so it says the range and tells the user
/// nothing about the outcome; Row 437's design sentence wants both halves and
/// gets one.
fn the_largest_figure_this_plan_states() -> i128 {
    *EEVEE_COUNT_RANGE.end() as i128
}

/// **J12 for a number the box would not hold.** At or below the largest figure
/// this plan states, the silent correction is established and is what the line
/// says; above it, nothing is, and the line states the range alone.
fn what_a_number_out_of_range_states(typed: i128, range: &RangeInclusive<u64>) -> Statement {
    let (lower, upper) = (*range.start(), *range.end());
    if typed <= the_largest_figure_this_plan_states() {
        Statement::CorrectedSilentlyAtAssignmentAndPaidFor { lower, upper }
    } else {
        Statement::TheRangeAlone { lower, upper }
    }
}

/// **J12, performed.**
pub fn what_the_refusal_states(refusal: &Refusal) -> Statement {
    match refusal {
        Refusal::PercentageOutOfRange { typed } => {
            what_a_number_out_of_range_states(*typed, &percentage_range_as_counts())
        }
        Refusal::PercentageNotAWholeNumber | Refusal::SamplesNotAWholeNumber => {
            Statement::RefusedWithAnErrorNamingTheSettingSoTheAttemptWouldFail
        }
        Refusal::TheFrameField(FrameRefusal::AStepBelowOne) => {
            Statement::AStepOfZeroWritesTheFirstFrameOnce
        }
        Refusal::TheFrameField(FrameRefusal::ALastFrameBeforeTheFirst) => {
            Statement::AnEndBelowTheStartRendersNothingAndExitsZero
        }
        Refusal::TheFrameField(FrameRefusal::AnEmptyList | FrameRefusal::AStrayCharacter) => {
            Statement::RendersNothingAtAllAndStillExitsZero
        }
        // **J12's new bullet, added 2026-09-05 with J11 item 2's fifth item.**
        // The bound is Smoozy's own and the range never leaves the desktop, so
        // nothing is established about what Blender would do with a set that
        // size and the line states the bound alone.
        Refusal::TheFrameField(FrameRefusal::MoreFramesThanCanBeComposed) => {
            Statement::TheRangeAlone {
                lower: 1,
                upper: crate::frames::THE_MOST_FRAMES_A_SET_MAY_COVER,
            }
        }
        Refusal::SamplesOutOfRange { typed, range } => {
            what_a_number_out_of_range_states(*typed, range)
        }
        // The standing count is an accepted store entry, so it is inside one
        // of the two engines' declared ranges and can never be above the
        // largest figure this plan states; the boundary is applied here all
        // the same, so no route to a statement skips it.
        Refusal::EnginePickWhileACountAboveItsCeilingStands {
            standing_count,
            the_picks_range,
        } => what_a_number_out_of_range_states(*standing_count as i128, the_picks_range),
    }
}

/// The percentage's range, as the count-shaped range a statement carries.
fn percentage_range_as_counts() -> RangeInclusive<u64> {
    *PERCENTAGE_RANGE.start() as u64..=*PERCENTAGE_RANGE.end() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_percentage_box_takes_whole_numbers_from_one_to_32767_and_nothing_else() {
        assert_eq!(percentage_box("50"), Ok(50));
        assert_eq!(percentage_box(" 32767 "), Ok(32767));
        assert_eq!(
            percentage_box("0"),
            Err(Refusal::PercentageOutOfRange { typed: 0 })
        );
        assert_eq!(
            percentage_box("32768"),
            Err(Refusal::PercentageOutOfRange { typed: 32768 })
        );
        assert_eq!(
            percentage_box("-5"),
            Err(Refusal::PercentageOutOfRange { typed: -5 })
        );
        for not_whole in ["12.5", "half", "", "1e3", "12%"] {
            assert_eq!(
                percentage_box(not_whole),
                Err(Refusal::PercentageNotAWholeNumber)
            );
        }
    }

    #[test]
    fn the_samples_box_holds_to_the_picked_engines_range_and_to_the_narrower_where_none_is_picked()
    {
        assert_eq!(samples_box("128", Some(OfferedEngine::Cycles)), Ok(128));
        assert_eq!(
            samples_box("16777217", Some(OfferedEngine::Cycles)),
            Err(Refusal::SamplesOutOfRange {
                typed: 16_777_217,
                range: CYCLES_COUNT_RANGE
            })
        );
        assert_eq!(
            samples_box("16777217", Some(OfferedEngine::Eevee)),
            Ok(16_777_217)
        );
        assert_eq!(
            samples_box("2147483648", Some(OfferedEngine::Eevee)),
            Err(Refusal::SamplesOutOfRange {
                typed: 2_147_483_648,
                range: EEVEE_COUNT_RANGE
            })
        );
        // No pick: the narrower ceiling, and never a reading's engine.
        assert_eq!(the_count_range(None), CYCLES_COUNT_RANGE);
        assert!(samples_box("16777217", None).is_err());
        assert_eq!(
            samples_box("0", None),
            Err(Refusal::SamplesOutOfRange {
                typed: 0,
                range: CYCLES_COUNT_RANGE
            })
        );
        for not_whole in ["1.5", "many", "", "1e3"] {
            assert_eq!(
                samples_box(not_whole, None),
                Err(Refusal::SamplesNotAWholeNumber)
            );
        }
    }

    #[test]
    fn the_engine_pick_is_refused_while_a_count_above_its_ceiling_stands() {
        let count = 20_000_000;
        assert_eq!(
            engine_pick_against_the_standing_count(OfferedEngine::Cycles, Some(count)),
            Err(Refusal::EnginePickWhileACountAboveItsCeilingStands {
                standing_count: count,
                the_picks_range: CYCLES_COUNT_RANGE
            })
        );
        assert_eq!(
            engine_pick_against_the_standing_count(OfferedEngine::Eevee, Some(count)),
            Ok(OfferedEngine::Eevee)
        );
        assert_eq!(
            engine_pick_against_the_standing_count(OfferedEngine::Cycles, None),
            Ok(OfferedEngine::Cycles)
        );
    }

    #[test]
    fn clearing_the_engine_pick_is_refused_while_a_count_above_the_no_pick_ceiling_stands() {
        // An EEVEE pick let this count in; clearing it would drop the range in
        // force to the no-pick ceiling with the count left standing behind.
        let count = 20_000_000;
        assert_eq!(
            clearing_the_engine_pick_against_the_standing_count(Some(count)),
            Err(Refusal::EnginePickWhileACountAboveItsCeilingStands {
                standing_count: count,
                the_picks_range: CYCLES_COUNT_RANGE
            })
        );
        // Retyping the count is what lets the clear through, and so is having
        // typed none.
        assert_eq!(
            clearing_the_engine_pick_against_the_standing_count(Some(128)),
            Ok(())
        );
        assert_eq!(
            clearing_the_engine_pick_against_the_standing_count(None),
            Ok(())
        );
        // The same collision, so the same refusal: no second one was invented.
        assert_eq!(
            clearing_the_engine_pick_against_the_standing_count(Some(count)),
            engine_pick_against_the_standing_count(OfferedEngine::Cycles, Some(count)).map(|_| ())
        );
    }

    #[test]
    fn above_the_largest_figure_this_plan_states_the_line_states_the_range_alone() {
        // At the largest stated figure the silent correction is established.
        assert_eq!(
            what_the_refusal_states(&Refusal::SamplesOutOfRange {
                typed: 2_147_483_647,
                range: CYCLES_COUNT_RANGE
            }),
            Statement::CorrectedSilentlyAtAssignmentAndPaidFor {
                lower: 1,
                upper: 16_777_216
            }
        );
        // One past it nothing is, and the line says the range and no more —
        // never that Blender would have corrected it in silence and charged
        // for the render.
        assert_eq!(
            what_the_refusal_states(&Refusal::SamplesOutOfRange {
                typed: 2_147_483_648,
                range: CYCLES_COUNT_RANGE
            }),
            Statement::TheRangeAlone {
                lower: 1,
                upper: 16_777_216
            }
        );
        assert_eq!(
            what_the_refusal_states(&Refusal::PercentageOutOfRange {
                typed: i128::from(u64::MAX)
            }),
            Statement::TheRangeAlone {
                lower: 1,
                upper: 32767
            }
        );
        // Below the range is established in both directions and unaffected.
        assert_eq!(
            what_the_refusal_states(&Refusal::PercentageOutOfRange { typed: -5 }),
            Statement::CorrectedSilentlyAtAssignmentAndPaidFor {
                lower: 1,
                upper: 32767
            }
        );
    }

    #[test]
    fn a_frame_set_too_large_to_compose_states_the_bound_alone() {
        assert_eq!(
            what_the_refusal_states(&Refusal::TheFrameField(
                FrameRefusal::MoreFramesThanCanBeComposed
            )),
            Statement::TheRangeAlone {
                lower: 1,
                upper: 1_000_000
            }
        );
    }

    #[test]
    fn each_refusal_states_what_is_established_for_its_own_setting() {
        assert_eq!(
            what_the_refusal_states(&Refusal::PercentageOutOfRange { typed: 0 }),
            Statement::CorrectedSilentlyAtAssignmentAndPaidFor {
                lower: 1,
                upper: 32767
            }
        );
        assert_eq!(
            what_the_refusal_states(&Refusal::PercentageNotAWholeNumber),
            Statement::RefusedWithAnErrorNamingTheSettingSoTheAttemptWouldFail
        );
        assert_eq!(
            what_the_refusal_states(&Refusal::TheFrameField(FrameRefusal::AStepBelowOne)),
            Statement::AStepOfZeroWritesTheFirstFrameOnce
        );
        assert_eq!(
            what_the_refusal_states(&Refusal::TheFrameField(
                FrameRefusal::ALastFrameBeforeTheFirst
            )),
            Statement::AnEndBelowTheStartRendersNothingAndExitsZero
        );
        assert_eq!(
            what_the_refusal_states(&Refusal::TheFrameField(FrameRefusal::AStrayCharacter)),
            Statement::RendersNothingAtAllAndStillExitsZero
        );
        assert_eq!(
            what_the_refusal_states(&Refusal::SamplesOutOfRange {
                typed: 0,
                range: EEVEE_COUNT_RANGE
            }),
            Statement::CorrectedSilentlyAtAssignmentAndPaidFor {
                lower: 1,
                upper: 2_147_483_647
            }
        );
        assert_eq!(
            what_the_refusal_states(&Refusal::SamplesNotAWholeNumber),
            Statement::RefusedWithAnErrorNamingTheSettingSoTheAttemptWouldFail
        );
    }
}
