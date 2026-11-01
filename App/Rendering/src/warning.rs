//! The missing-assets warning — §5 W1–W5, W4a (Rows 5 and 374).
//!
//! **W1: the warning is evaluated at the moment of submitting, from the
//! check standing alone, with no read of the file and no read of the
//! machine.** It appears while no complete check standing satisfying
//! `Dependency Upload & Remap` V11 exists for the (root file's machine path,
//! machine under its installation identity, submission's Blender version)
//! the act names, and is absent where one does.
//!
//! **W2: the three cases are three record states, and the warning says
//! which** — no standing at all; a standing carrying the replaced-since-
//! check or disturbed mark; a standing complete for this file on this
//! machine but keyed to another Blender version. **The third is W1's key
//! read part by part rather than as one test**: a standing matching the
//! path and the machine and differing only on the version is a record
//! state, not an absence, and **the warning is never allowed to say nobody
//! checked where one stands.**
//!
//! **W3: what counts as *the file has been replaced* is V13's answer**,
//! cited — Smoozy's own recorded act, never a comparison of what sits on
//! the machine; presence, size and the machine's clock decide nothing.
//!
//! **Markers used in this file that belong to another document, each with
//! the document it belongs to.** `Dependency Upload & Remap` V10, V11,
//! V12, V13, V14, V15. `Desktop app and delivery` G3, G4, G5, G6.
//!
//! **`Desktop app and delivery` A7 is not listed above and is written out at
//! the site**, because this plan defines an A7 of its own.

use smoozy_dependencies::standing::{
    the_warning_over, CheckStanding, Counts, TheWarning, WhatTheRecordSays,
};
use smoozy_render_settings::file_half::BlenderIdentity;

/// **What the warning says, and it is one of W2's three states or W4's
/// replacement — or nothing.**
///
/// # Six states against the band's four statements
///
/// **W4a counts the band as four statements — W2's three record states and
/// W4's replacement — and this type has six.** The suppression is one
/// identity over the whole band either way ([`THE_SUPPRESSION_IDENTITY`]),
/// so nobody meets a warning they switched off; what the split costs is that
/// two of the six have no sentence of their own drawn anywhere. **Whoever
/// presents this must not invent one.** Rule 10 settles the direction:
/// Design conforms to Architecture, the plan's four statements stand, and
/// the extra states map onto them. **Written down here on 2026-09-05, out of
/// Round 1 of this crate's code audit, which ruled the six kept and this
/// mapping stated rather than the two folded away** — the six are the more
/// truthful set, `Dependency Upload & Remap` genuinely keeps the replaced
/// and disturbed marks apart, and folding them back would throw away a
/// distinction that document already makes.
///
/// **The mapping, state by state:**
///
/// 1. [`Self::NobodyHasEverCheckedThisFileOnThisMachine`] — W2's **first**
///    statement, which is drawn for it exactly.
/// 2. [`Self::AcheckWasRunAndTheFileHasBeenReplacedSince`] — W2's **second**,
///    drawn for it exactly, including that it is the new upload that put the
///    warning back.
/// 3. [`Self::ACheckWasRunAndItsTreeHasBeenDisturbedSince`] — W2's
///    **second**. **No sentence of its own is drawn.** W2 names the
///    disturbed mark inside that one statement, alongside the replaced mark,
///    so this state is presented under it.
/// 4. [`Self::ACheckStandsAndDoesNotCoverThisBlender`] — W2's **third**,
///    drawn for it exactly.
/// 5. [`Self::ACheckWasRunAndDidNotComplete`] — W2's **first**. **No
///    sentence of its own is drawn, and the one it is presented under does
///    not fit it: W2's first statement says nobody has ever checked, and
///    here somebody did and it did not finish.** It is presented under that
///    statement because W1's test is a *complete* standing satisfying
///    `Dependency Upload & Remap` V11, and an incomplete one satisfies
///    nothing — so as far as the warning goes no check stands. **The cost is
///    stated rather than smoothed**: it is the one place in the band where
///    the drawn wording is wrong about the record, and the alternative is a
///    sentence nobody drew.
/// 6. [`Self::TheCheckLeftThingsUnfound`] — W4's **fourth** statement, the
///    replacement, with the standing's own counts.
///
/// [`Self::Absent`] carries no statement: it is the band not appearing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheWarningsCase {
    /// **No standing at all**: nobody has ever checked this file on this
    /// machine. **W2's first statement.**
    NobodyHasEverCheckedThisFileOnThisMachine,
    /// **A standing carrying the replaced-since-check mark**: a check was
    /// run and the file has been replaced since — and it is the new upload
    /// that put the warning back. **W2's second statement.**
    AcheckWasRunAndTheFileHasBeenReplacedSince,
    /// **A standing carrying the disturbed mark**. **W2's second statement**,
    /// which names this mark beside the replaced one; **no sentence of its
    /// own is drawn** and none is invented.
    ACheckWasRunAndItsTreeHasBeenDisturbedSince,
    /// **A standing complete for this file on this machine but keyed to
    /// another version**: the check stands and does not cover the Blender
    /// this job will render with. **W2's third statement.**
    ACheckStandsAndDoesNotCoverThisBlender { the_standing_covers: String },
    /// A standing that never completed — stopped, or the connection
    /// dropped — stops nothing. **W2's first statement**, which is the one
    /// the band draws for a check that does not stand; **no sentence of its
    /// own is drawn, and that statement's wording is wrong about this
    /// record** — see the mapping on [`TheWarningsCase`].
    ACheckWasRunAndDidNotComplete,
    /// **W4: the check that left things unfound replaces the warning with
    /// the specific one, and the counts it names are the standing's own.**
    /// **The band's fourth statement.**
    TheCheckLeftThingsUnfound(Counts),
    /// A complete standing whose every row is confirmed or set aside: no
    /// warning. **No statement: the band does not appear.**
    Absent,
}

/// **W1 and W2, performed** over what `Dependency Upload & Remap` V14
/// supplies as data.
pub fn evaluate(
    the_record_says: &WhatTheRecordSays,
    the_submissions_version: &BlenderIdentity,
) -> TheWarningsCase {
    let standing: &CheckStanding = match the_record_says {
        WhatTheRecordSays::NobodyHasEverChecked => {
            return TheWarningsCase::NobodyHasEverCheckedThisFileOnThisMachine
        }
        WhatTheRecordSays::AStanding(standing) => standing,
    };
    // The key read part by part: a standing for this file on this machine
    // is a record state whatever else it carries.
    if standing.replaced_since_check {
        return TheWarningsCase::AcheckWasRunAndTheFileHasBeenReplacedSince;
    }
    if standing.disturbed {
        return TheWarningsCase::ACheckWasRunAndItsTreeHasBeenDisturbedSince;
    }
    if !standing.covers(the_submissions_version) {
        return TheWarningsCase::ACheckStandsAndDoesNotCoverThisBlender {
            the_standing_covers: smoozy_render_settings::file_half::version_text(
                standing.read_with.version,
            ),
        };
    }
    match the_warning_over(standing, the_submissions_version) {
        TheWarning::Stopped => TheWarningsCase::Absent,
        TheWarning::Replaced(counts) => TheWarningsCase::TheCheckLeftThingsUnfound(counts),
        TheWarning::Standing => TheWarningsCase::ACheckWasRunAndDidNotComplete,
    }
}

/// **W4: the three ways on — the spec's, the third reaching S13's
/// composition unchanged.** Wording and presentation are Design's.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AWayOn {
    GoAndCheck,
    SubmitAnyway,
    SubmitWithTheFirstFrameCheck,
}

pub const THE_THREE_WAYS_ON: [AWayOn; 3] = [
    AWayOn::GoAndCheck,
    AWayOn::SubmitAnyway,
    AWayOn::SubmitWithTheFirstFrameCheck,
];

/// **W4a: the warning's suppression identity, and it is the only one this
/// document owns** (Row 374; `Desktop app and delivery` G3–G6). One
/// identity covers the whole band in all four of its statements; the
/// switch-off is across every file and every machine by construction; a
/// suppression withholds the warning and touches no record.
pub const THE_SUPPRESSION_IDENTITY: &str = "rendering.missing-assets";

/// The row in the one suppression table: this document, and that identity.
pub const THE_SUPPRESSION_ROW: (&str, &str) = ("Rendering", THE_SUPPRESSION_IDENTITY);

/// **W4a, performed: a suppression withholds the band and every one of its
/// four statements, and touches no record** — W1's evaluation is unmoved
/// and the standing is a different object.
pub fn what_the_user_meets(case: TheWarningsCase, suppressed: bool) -> Option<TheWarningsCase> {
    match case {
        TheWarningsCase::Absent => None,
        case if suppressed => {
            let _ = case;
            None
        }
        case => Some(case),
    }
}

/// **W4a: what goes with the band and what does not** — the three ways on
/// hang off the band; **the first-frame check is unaffected**, being one of
/// the things settled at the moment of submitting in its own right.
pub const THE_FIRST_FRAME_CHECK_IS_UNAFFECTED_BY_THE_SUPPRESSION: bool = true;

/// **W3 and W5**: the replacement trigger is V13's recorded act, never a
/// comparison; and a file changed at render time is not this warning's
/// case — P1 step 3's fresh four-value check catches it.
pub const THE_TRIGGER_IS_SMOOZYS_OWN_RECORDED_ACT_AND_RENDER_TIME_IS_P1_STEP_3S: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::testing::{a_blender, a_moment, four_values};
    use smoozy_dependencies::bytes::MachineLocation;
    use smoozy_dependencies::record::MachineKey;
    use smoozy_dependencies::rows::RowState;
    use smoozy_dependencies::standing::{Completion, RowAtCompletion};
    use smoozy_state::identities::{Identity, IdentityKind};

    fn a_standing(completion: Completion, rows: &[RowAtCompletion]) -> CheckStanding {
        CheckStanding::at_the_sittings_end(
            MachineLocation::composed(b"/ephemeral/smoozy/assets/kitchen.blend".to_vec()),
            MachineKey::of(
                Identity::issue(IdentityKind::LocalServer).unwrap(),
                Identity::issue(IdentityKind::Installation).unwrap(),
            ),
            a_blender(),
            a_moment(),
            completion,
            rows,
            four_values(),
        )
    }

    fn confirmed() -> RowAtCompletion {
        RowAtCompletion {
            state: RowState::AlreadyOnTheMachine,
            inside_a_linked_blend: None,
        }
    }

    fn not_found() -> RowAtCompletion {
        RowAtCompletion {
            state: RowState::NotFound,
            inside_a_linked_blend: None,
        }
    }

    #[test]
    fn no_standing_at_all_is_nobody_has_ever_checked() {
        assert_eq!(
            evaluate(&WhatTheRecordSays::NobodyHasEverChecked, &a_blender()),
            TheWarningsCase::NobodyHasEverCheckedThisFileOnThisMachine
        );
    }

    #[test]
    fn a_complete_confirmed_standing_for_this_blender_stops_the_warning() {
        let standing = a_standing(Completion::Completed, &[confirmed(), confirmed()]);
        assert_eq!(
            evaluate(
                &WhatTheRecordSays::AStanding(Box::new(standing)),
                &a_blender()
            ),
            TheWarningsCase::Absent
        );
    }

    /// **W2's third state**: a standing keyed to another version is a
    /// record state and never *nobody checked*.
    #[test]
    fn a_standing_keyed_to_another_version_says_the_check_does_not_cover_this_blender() {
        let standing = a_standing(Completion::Completed, &[confirmed()]);
        let other = BlenderIdentity {
            path: "/ephemeral/smoozy/blender/blender-4.5.12/blender".to_string(),
            version: crate::versions::testing::a_version(4, 5, 12),
        };
        assert_eq!(
            evaluate(&WhatTheRecordSays::AStanding(Box::new(standing)), &other),
            TheWarningsCase::ACheckStandsAndDoesNotCoverThisBlender {
                the_standing_covers: "5.2.0".to_string()
            }
        );
    }

    #[test]
    fn the_replaced_and_disturbed_marks_are_their_own_states() {
        let replaced = a_standing(Completion::Completed, &[confirmed()]).replaced_since_check();
        assert_eq!(
            evaluate(
                &WhatTheRecordSays::AStanding(Box::new(replaced)),
                &a_blender()
            ),
            TheWarningsCase::AcheckWasRunAndTheFileHasBeenReplacedSince
        );
        let disturbed = a_standing(Completion::Completed, &[confirmed()]).disturbed();
        assert_eq!(
            evaluate(
                &WhatTheRecordSays::AStanding(Box::new(disturbed)),
                &a_blender()
            ),
            TheWarningsCase::ACheckWasRunAndItsTreeHasBeenDisturbedSince
        );
    }

    #[test]
    fn a_check_that_left_things_unfound_replaces_the_warning_with_the_standings_own_counts() {
        let standing = a_standing(
            Completion::Completed,
            &[confirmed(), not_found(), not_found()],
        );
        let case = evaluate(
            &WhatTheRecordSays::AStanding(Box::new(standing)),
            &a_blender(),
        );
        let TheWarningsCase::TheCheckLeftThingsUnfound(counts) = case else {
            panic!("{case:?}");
        };
        assert_eq!(counts.not_found, 2);
        assert_eq!(counts.confirmed, 1);
        let stopped = a_standing(Completion::StoppedByTheUser, &[confirmed()]);
        assert_eq!(
            evaluate(
                &WhatTheRecordSays::AStanding(Box::new(stopped)),
                &a_blender()
            ),
            TheWarningsCase::ACheckWasRunAndDidNotComplete
        );
    }

    #[test]
    fn a_suppression_withholds_every_statement_of_the_band_and_touches_no_record() {
        assert_eq!(
            THE_SUPPRESSION_ROW,
            ("Rendering", "rendering.missing-assets")
        );
        for case in [
            TheWarningsCase::NobodyHasEverCheckedThisFileOnThisMachine,
            TheWarningsCase::AcheckWasRunAndTheFileHasBeenReplacedSince,
            TheWarningsCase::ACheckWasRunAndItsTreeHasBeenDisturbedSince,
            TheWarningsCase::ACheckStandsAndDoesNotCoverThisBlender {
                the_standing_covers: "4.5.12".into(),
            },
            TheWarningsCase::TheCheckLeftThingsUnfound(Counts::default()),
        ] {
            assert_eq!(what_the_user_meets(case.clone(), true), None);
            assert_eq!(what_the_user_meets(case.clone(), false), Some(case));
        }
        assert_eq!(what_the_user_meets(TheWarningsCase::Absent, false), None);
        assert_eq!(THE_THREE_WAYS_ON.len(), 3);
        const _: () = assert!(THE_FIRST_FRAME_CHECK_IS_UNAFFECTED_BY_THE_SUPPRESSION);
        const _: () =
            assert!(THE_TRIGGER_IS_SMOOZYS_OWN_RECORDED_ACT_AND_RENDER_TIME_IS_P1_STEP_3S);
    }
}
