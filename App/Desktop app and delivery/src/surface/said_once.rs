//! **Said once, and the one control that restores every suppressed warning** —
//! `Desktop app and delivery` §14 and §15. The page half is `web/said-once.js`.
//!
//! **This file is the surface and not the mechanism.** §14's gate is
//! [`crate::presentation::said_once`], §15's store is
//! [`crate::record::suppressed_warnings`], and the declaration route both lean
//! on is [`crate::presentation::declarations`]. Read those three heads before
//! adding anything here.
//!
//! ## Why two halves of one plan are one service
//!
//! §14 is *how many times Smoozy says one thing*; §15 is *the one control that
//! undoes every time the user said don't say it again*. **They meet in the same
//! place on the same screen** — Design puts the four notification switches and
//! the restore control in one section of Settings, *"because they are the same
//! family: things Smoozy says to the user"* — and they meet in the same rule:
//! a suppression is the user's own and nothing here ages one, caps one or
//! sweeps one away.
//!
//! ## What the page gets, and it is one route
//!
//! P6: results reach the interface as events on the one ordered queue, and the
//! interface holds no other route to a module. **A command is the page asking;
//! an event is the application telling.** So neither command below answers with
//! anything: [`the_warnings_switched_off`] asks and the count arrives as an
//! event, [`every_switched_off_warning_is_restored`] presses the control and
//! the new count arrives as the same event. **One fact, one kind, one drawing
//! path** — a count returned from one call and posted from the other would be
//! two answers to one question.
//!
//! **The count is a reading and never a stored number.** G4 forbids a count in
//! the row, and a count kept beside the table would be the fact in two places.
//!
//! ## What is not built here, and it is named rather than left to be found
//!
//! - **[`a_warning_was_switched_off`] has no caller.** It is the writer a
//!   warning's own box calls when the user ticks *Don't warn me about this
//!   again*, and **no screen in Smoozy draws a warning yet** — `File Browser`'s
//!   delete box and `Rendering`'s missing-assets band are both later screens.
//!   It is built now because §15's control has to have something to restore and
//!   because the declaration check has one home rather than one per caller.
//! - **[`a_condition_was_raised_is_told_to_the_operating_system`] has no caller
//!   either**, for the same shape of reason: **nothing in Smoozy raises a
//!   condition record.** `crate::record::failure_records`'
//!   `a_condition_was_raised` is reached by no working code in this crate —
//!   `Servers` supplies the observed value under `State, identity and recovery`
//!   §8.5 and that wiring is a later step's. What is here is the composed route
//!   the raiser calls, with Q13 built into it so a raiser cannot get it wrong.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** None: every marker of another document below is
//! written out with that document at the site.
//!
//! **`State, identity and recovery`'s own markers are deliberately not listed
//! above and are written out with that document at every site instead** — this
//! plan defines clauses of the same short names, and a header list cannot tell
//! the two apart.

use std::sync::Mutex;

use tauri::State;

use crate::events::{Event, Said, Subject, TheOneQueue};
use crate::notifications::{kinds, picture, APost, HowTheRecordEnded, TheNotificationService};
use crate::presentation::said_once::TheThingsSaidOnce;
use crate::presentation::the_presentation_key::ThePresentationKey;
use crate::record::suppressed_warnings;
use crate::start::Started;
use crate::typography::{Prose, Verbatim};
use smoozy_state::identities::Identity;

/// **The kind the page listens for**, declared here beside the service that
/// posts it and copied into `web/said-once.js`. Nothing holds the two together,
/// so copy carefully.
///
/// **One kind for both commands**, because both answer the same question: how
/// many warnings the user has switched off, and what the control says about
/// them.
pub const THE_WARNINGS_SWITCHED_OFF: &str = "settings:the-warnings-switched-off";

/// **The section's own heading**, Design's word for word.
const THE_HEADING: &str = "Switched-off warnings";

/// **The word beside the count.** It is its own string because the count is a
/// value and this is prose: X7 forbids the typography point touching a value,
/// and joining the two before the point is the trap the whole of step 10a is
/// written against. The page puts them side by side and never into one string.
const THE_COUNTS_WORD: &str = "switched off";

/// **What the control's line reads where nothing is switched off.**
///
/// Design, settled 2026-08-23: *"Where none is switched off, the control is
/// there and reads *none switched off* rather than going away — a control that
/// appears only when it has work is a control nobody knows exists."*
const WHEN_NONE_ARE_SWITCHED_OFF: &str = "None switched off";

/// The control itself.
const THE_CONTROL: &str = "Restore all";

/// **What the control covers**, and it is two sentences handed over as two
/// strings — X10: the orphan rule binds the last two words of *a piece of
/// prose*, and a paragraph handed over whole leaves every interior sentence
/// ending unbound.
const WHAT_THE_CONTROL_COVERS: &str = "Restores every warning you switched off.";

/// The second sentence. Design: *"says there is no way back one at a time … the
/// spec makes it all at once; saying so stops a user hunting for a list that
/// does not exist."*
const THERE_IS_NO_WAY_BACK_ONE_AT_A_TIME: &str = "They cannot be restored one at a time.";

/// **What the line reads where the store would not answer** — L8a, and it is
/// the opposite of drawing a nought.
///
/// **The words are Design's and are owed** (§24). What is fixed here is that the
/// line says the number is unread rather than saying there is none.
const THE_COUNT_WOULD_NOT_READ: &str = "Could not read how many are switched off.";

/// **What is said when the press did not land.**
///
/// **The rule this round works to: no press of a control in this Subject may end
/// in nothing being said.** The user presses *Restore all* with nine
/// warnings switched off, the delete is refused, and what they used to see was a
/// control that did nothing — twice, because pressing it again did nothing
/// again. **The words are Design's and are owed.**
const THE_RESTORE_DID_NOT_LAND: &str = "Could not restore the warnings.";

/// **The page asks what the control will do.**
#[tauri::command]
pub fn the_warnings_switched_off(
    started: State<'_, Mutex<Started>>,
    queue: State<'_, std::sync::Arc<TheOneQueue>>,
) {
    let Ok(started) = started.lock() else {
        return;
    };
    the_count_is_posted(&started, &queue);
}

/// **The control was pressed.**
///
/// **G7: one delete over the table, and it walks no modules.** **G8: its
/// partition unit is the whole table**, so it restores all of them or leaves
/// the table as it was.
///
/// **The write runs first and the post follows it** (P7) — a restore that did
/// not land leaves the count on screen where it was, and the page will read the
/// same number when it next asks. That is the clause and not an oversight.
#[tauri::command]
pub fn every_switched_off_warning_is_restored(
    started: State<'_, Mutex<Started>>,
    queue: State<'_, std::sync::Arc<TheOneQueue>>,
) {
    let Ok(started) = started.lock() else {
        return;
    };
    the_control_was_pressed(&started, &queue);
}

/// The control's act, with the spine handed in — **so the whole of it can be
/// driven without the shell's own managed state**, which the command cannot be.
/// The same split piece B and piece C both make, and for the same reason.
pub fn the_control_was_pressed(started: &Started, queue: &TheOneQueue) {
    let Some(record) = started.record.as_ref() else {
        return;
    };
    let connection = record.connection();
    let _ = queue.write_then_post::<WhatTheControlSays, std::convert::Infallible>(
        // **The refusal is what the durable act answers with**, rather than an
        // error that leaves the queue with nothing to say. P7's door runs the
        // durable act first and composes from what it returned; a refusal *is*
        // what it returned.
        || {
            // **The count is read either way, and a refused press does not take
            // the number off the screen.** Until the close of round 1 a refusal
            // posted the refusal sentence and no count at all, so the page took
            // its unread branch and drew an empty line: a user with nine
            // warnings switched off pressed the control, was refused, and the
            // line that read *9 switched off* went blank. **L8a: that screen
            // shows the failure rather than an emptied version of itself** — and
            // the table is exactly as it was, so the number is still true.
            // Found at the close check of round 1 (C-2).
            let did_not_land =
                suppressed_warnings::every_switched_off_warning_is_restored(connection).is_err();
            Ok(WhatTheControlSays {
                the_count: the_count(connection),
                the_press_did_not_land: did_not_land,
            })
        },
        the_control_says,
    );
}

/// The reading, posted. **P7's read door**: the number is taken out of the
/// durable store and then posted, never composed from anything held on this
/// side.
pub fn the_count_is_posted(started: &Started, queue: &TheOneQueue) {
    let Some(record) = started.record.as_ref() else {
        return;
    };
    let connection = record.connection();
    let _ = queue.read_then_post::<WhatTheControlSays, std::convert::Infallible>(
        || {
            Ok(WhatTheControlSays {
                the_count: the_count(connection),
                // The page asked; nothing was pressed.
                the_press_did_not_land: false,
            })
        },
        the_control_says,
    );
}

/// **What the control's own line says**, which is a count or the admission that
/// there is none to draw.
///
/// **L8a: *"there is no state in which a list is drawn short, a count reads
/// zero, or a used by nothing line appears because a read failed"*, and *"that
/// screen shows the failure rather than an emptied version of itself"*.** Until
/// 2026-09-12 a failed read was drawn as **nought**, which the page turned into
/// *None switched off* and a control greyed out — the positive statement that
/// the user has nothing suppressed, at the one moment they are looking to find
/// out whether the restore they just pressed worked. Found at round 1 of the
/// code audit of what is drawn inside the window (F-10).
/// **The two lines are two facts and the section draws both**, which is why
/// this is a pair and not a choice of three.
///
/// **A refused press used to be the third of three**, so it carried no count at
/// all and the count line went blank underneath it — the user lost the number
/// they were reading at the moment they were told the press failed. **L8a again,
/// by the other route**: *that screen shows the failure rather than an emptied
/// version of itself*. Found at the close check of round 1 (C-2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhatTheControlSays {
    /// **The count line, and it is always drawn.**
    pub the_count: TheCountsLine,
    /// **Whether a press was refused**, which adds a line and takes none away.
    /// False wherever the page merely asked.
    pub the_press_did_not_land: bool,
}

/// What the count line reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheCountsLine {
    /// How many are switched off, read out of the store.
    ThisMany(u32),
    /// **The store would not answer**, so the line says so rather than saying
    /// nought.
    ItWouldNotRead,
}

fn the_count(connection: &rusqlite::Connection) -> TheCountsLine {
    match suppressed_warnings::how_many_are_switched_off(connection) {
        Ok(how_many) => TheCountsLine::ThisMany(how_many),
        Err(_) => TheCountsLine::ItWouldNotRead,
    }
}

/// **What the control says**, in the two routes and never in one string.
///
/// A6 and X9: prose through the typography point, everything else by the
/// verbatim route, and **the two are the complete set**. The count goes by the
/// verbatim route because it is a reading rather than a sentence — and because
/// a number joined into prose before the point is exactly the join X7 forbids,
/// whatever the number happens to be.
fn the_control_says(says: &WhatTheControlSays) -> Event {
    let mut event = Event::about(Subject::TheRun, THE_WARNINGS_SWITCHED_OFF);
    match says.the_count {
        TheCountsLine::ThisMany(how_many) => {
            event = event.saying("count", Said::Value(Verbatim::new(how_many.to_string())));
        }
        // **No count is posted at all**, so the page has nothing to draw as
        // nought, and the control's own line says it is unread instead.
        TheCountsLine::ItWouldNotRead => {
            event = event.saying(
                "theCountWouldNotRead",
                Said::Prose(Prose::new(THE_COUNT_WOULD_NOT_READ)),
            );
        }
    }
    // **A second line beside the count and never instead of it** (C-2).
    if says.the_press_did_not_land {
        event = event.saying(
            "theRestoreDidNotLand",
            Said::Prose(Prose::new(THE_RESTORE_DID_NOT_LAND)),
        );
    }
    event
        .saying("theCountsWord", Said::Prose(Prose::new(THE_COUNTS_WORD)))
        .saying(
            "whenNoneAreSwitchedOff",
            Said::Prose(Prose::new(WHEN_NONE_ARE_SWITCHED_OFF)),
        )
        .saying("heading", Said::Prose(Prose::new(THE_HEADING)))
        .saying("theControl", Said::Prose(Prose::new(THE_CONTROL)))
        .saying(
            "whatItCovers",
            Said::Prose(Prose::new(WHAT_THE_CONTROL_COVERS)),
        )
        .saying(
            "andNoWayBackOneAtATime",
            Said::Prose(Prose::new(THERE_IS_NO_WAY_BACK_ONE_AT_A_TIME)),
        )
}

/// **The user switched a warning off** — §15.2, and the one write there is.
///
/// **The declaration is checked here and nowhere else** (G5, and §4 A7's
/// hosting rule). A module that has not declared the identity answers *no*, and
/// **the suppression is refused rather than written**: A7's *a declaration
/// missing is a module that cannot be hosted, not a default this document
/// supplies*.
///
/// **G3 is what makes the check worth having.** It binds documents not yet
/// written: every dismissible warning in every module, present and future,
/// records its suppression here. **A module that reached the table without a
/// declaration would be a row nothing could ever tie back to a module**, and
/// G4's *the module and the warning identity* would be false of it.
///
/// The answer is whether the suppression went down.
pub fn a_warning_was_switched_off(started: &Started, module: &str, warning_identity: &str) -> bool {
    if !started
        .what_modules_declare
        .the_warning_is_declared(module, warning_identity)
    {
        return false;
    }
    let Some(record) = started.record.as_ref() else {
        return false;
    };
    suppressed_warnings::the_user_switched_it_off(record.connection(), warning_identity).is_ok()
}

/// **Whether this warning is switched off** — what a warning's own box asks
/// before it draws.
///
/// **No declaration means no**, which is the same hosting rule from the reading
/// side: an undeclared identity is not a warning this application hosts, so it
/// is not one the user can have switched off.
///
/// **A read that failed is its own answer and never *no*.** Until 2026-09-12 a
/// store that would not answer was drawn as *not switched off* — the kinder
/// direction, and the same substitution L8a forbids by name: *"there is no state
/// in which a list is drawn short, a count reads zero, or a used by nothing line
/// appears because a read failed."* Found at round 1 of the code audit of what
/// is drawn inside the window (F-10). **A box asking this carries the failure
/// rather than drawing itself as though the answer were known.**
pub fn the_warning_is_switched_off(
    started: &Started,
    module: &str,
    warning_identity: &str,
) -> WhetherItIsSwitchedOff {
    if !started
        .what_modules_declare
        .the_warning_is_declared(module, warning_identity)
    {
        return WhetherItIsSwitchedOff::No;
    }
    let Some(record) = started.record.as_ref() else {
        return WhetherItIsSwitchedOff::TheReadFailed;
    };
    match suppressed_warnings::it_is_switched_off(record.connection(), warning_identity) {
        Ok(true) => WhetherItIsSwitchedOff::Yes,
        Ok(false) => WhetherItIsSwitchedOff::No,
        Err(_) => WhetherItIsSwitchedOff::TheReadFailed,
    }
}

/// The three answers, and **the third is not one of the other two**.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhetherItIsSwitchedOff {
    Yes,
    No,
    /// **The store would not answer.** L8a: what a surface does with this is
    /// show the failure, never draw the kinder of the two answers.
    TheReadFailed,
}

/// **A condition record was raised, and the operating system is told once** —
/// §14 Q10 and Q13.
///
/// **The gate is inside the route rather than beside it**, so a raiser cannot
/// post the second one by forgetting to ask. Q13: *"a later failure resolving
/// to a standing identity needs no post at all … the repeat's correct treatment
/// on the notification surface is silence."*
///
/// **Why silence rather than a replacement.** Q10's replace-by-identity carries
/// the posts that are made, and it is documented on all three systems — but
/// whether replacing on a used identity behaves is unestablished on every one
/// of them (Section 22 item 27). **Posting every time and leaning on replace
/// would put a hundred notifications on the screen of any user whose system
/// turns out not to replace**, and nothing in Smoozy would report it. Built
/// this way the answer holds however that closes.
///
/// **T13: the post is made when the condition is raised and not when a request
/// happens to fail**, which is why this takes the condition identity the raise
/// answered with and nothing about the request that met it.
pub fn a_condition_was_raised_is_told_to_the_operating_system(
    said_once: &TheThingsSaidOnce,
    notifications: &TheNotificationService,
    condition: &Identity,
    the_subjects_own_name: Verbatim,
) {
    let what = kinds::what_a_raised_connection_condition_posts(condition);
    if !said_once.a_post_is_owed(&what.key) {
        return;
    }
    notifications.post(APost::from(
        what,
        the_subjects_own_name,
        picture::the_picture_for(std::env::consts::OS, None),
    ));
}

/// **The condition record ended, so the post comes down and the key is spent**
/// — T24a and §14 Q12.
///
/// The three endings are `State, identity and recovery` §8.8's and this reaches
/// all of them: the subject going, the user clearing it, and withdrawal. **A
/// condition that returns is a fresh identity against the same key** and is
/// said once again, which is Q12 and is why nothing here has to decide when a
/// suppression of a post expires.
pub fn the_condition_record_ended(
    said_once: &TheThingsSaidOnce,
    notifications: &TheNotificationService,
    condition: &Identity,
    how: HowTheRecordEnded,
) {
    let key = ThePresentationKey::ACondition(condition.clone());
    said_once.the_record_ended(&key);
    notifications.the_condition_record_ended(&key, how);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::presentation::declarations::{Declared, TheDeclarations};

    /// A reading, with nothing pressed.
    fn a_count_of(how_many: u32) -> WhatTheControlSays {
        WhatTheControlSays {
            the_count: TheCountsLine::ThisMany(how_many),
            the_press_did_not_land: false,
        }
    }

    /// **The count and the word are two strings on two routes**, and the number
    /// never travels through the typography point.
    ///
    /// **The event carries no accessor for what it says** — nothing on this
    /// side reads an event back, which is P6 working — so the assertion is over
    /// its written form, composed from the same two constructors the event
    /// uses so that neither escaping nor spelling can drift between them.
    #[test]
    fn the_count_is_a_value_and_the_word_beside_it_is_prose() {
        let event = the_control_says(&a_count_of(3));
        assert_eq!(event.kind(), THE_WARNINGS_SWITCHED_OFF);
        let said = format!("{event:?}");
        assert!(
            said.contains(&format!("{:?}", Verbatim::new("3"))),
            "the count left by a route other than the verbatim one: {said}"
        );
        assert!(
            said.contains(&format!("{:?}", Prose::new(THE_COUNTS_WORD))),
            "the word beside the count is not prose: {said}"
        );
        // **And the two are never one string.** A number joined into a
        // sentence before the point is the join X7 forbids, whatever the
        // number is.
        assert!(!said.contains("3 switched"), "{said}");
    }

    /// **Design's own line where nothing is switched off**, and the control is
    /// still there.
    ///
    /// *"A control that appears only when it has work is a control nobody knows
    /// exists."*
    #[test]
    fn the_control_says_none_rather_than_going_away() {
        let event = the_control_says(&a_count_of(0));
        let said = format!("{event:?}");
        assert!(said.contains(&format!("{:?}", Prose::new(WHEN_NONE_ARE_SWITCHED_OFF))));
        assert!(said.contains(&format!("{:?}", Prose::new(THE_CONTROL))));
        // **Both have already been through the typography point** (X1), so the
        // last pair of each is bound before it ever reaches the page.
        assert_eq!(
            Prose::new(WHEN_NONE_ARE_SWITCHED_OFF).as_str(),
            "None switched\u{a0}off"
        );
    }

    /// **C-2: a refused press adds a line and takes none away.**
    ///
    /// The user with nine warnings switched off presses *Restore all*, the
    /// delete is refused, and **the line that read *9 switched off* must still
    /// read it**. Until the close of round 1 the refusal carried no count at
    /// all, the page took its unread branch, and that branch drew
    /// `theCountWouldNotRead` — which was not on the event either, so the line
    /// went blank. **L8a: that screen shows the failure rather than an emptied
    /// version of itself.**
    #[test]
    fn a_refused_press_leaves_the_count_the_user_was_reading_standing() {
        let event = the_control_says(&WhatTheControlSays {
            the_count: TheCountsLine::ThisMany(9),
            the_press_did_not_land: true,
        });
        let said = format!("{event:?}");
        assert!(
            said.contains(&format!("{:?}", Prose::new(THE_RESTORE_DID_NOT_LAND))),
            "the refusal is not said: {said}"
        );
        assert!(
            said.contains(&format!("{:?}", Verbatim::new("9"))),
            "the count went off the event with the refusal, so the page draws \
             no number at the one moment the user is looking to find out \
             whether the press worked: {said}"
        );
        // **And the page's own unread branch is not entered**: it turns on the
        // absence of a count, so a count that is there keeps the number drawn.
        assert!(
            !said.contains(&format!("{:?}", Prose::new(THE_COUNT_WOULD_NOT_READ))),
            "a refused press was drawn as a count that would not read: {said}"
        );
    }

    /// **C-2's other half: a refusal and an unreadable count are two facts, and
    /// a press that met both says both.**
    #[test]
    fn a_refused_press_over_a_count_that_will_not_read_says_both() {
        let event = the_control_says(&WhatTheControlSays {
            the_count: TheCountsLine::ItWouldNotRead,
            the_press_did_not_land: true,
        });
        let said = format!("{event:?}");
        assert!(said.contains(&format!("{:?}", Prose::new(THE_RESTORE_DID_NOT_LAND))));
        assert!(said.contains(&format!("{:?}", Prose::new(THE_COUNT_WOULD_NOT_READ))));
    }

    /// **The page draws the refusal beside the count line rather than in it.**
    ///
    /// **Nothing in this project reads the page half for behaviour**, so this
    /// reads it as text: the count line turns on `parts.count`, and the refusal
    /// is its own paragraph appended to the section. A page that drew the
    /// refusal *as* the count line would empty the number again from the other
    /// side.
    #[test]
    fn the_page_draws_the_refusal_beside_the_count_and_not_in_it() {
        let page = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("web")
                .join("said-once.js"),
        )
        .expect("the page module is readable");
        let label = page
            .split_once("const label")
            .expect("the page draws the count line")
            .1;
        let label = &label[..label.find("row.appendChild(label)").expect("the line is put on")];
        assert!(
            !label.contains("theRestoreDidNotLand"),
            "web/said-once.js draws the refusal inside the count line. **The \
             refusal is a line beside the count and never instead of it** — the \
             table is as it was, so the number is still true (C-2)"
        );
    }

    /// **X10: each sentence of the covering line is its own string**, so the
    /// orphan rule binds the last two words of each rather than of the pair.
    #[test]
    fn the_covering_line_is_two_sentences_and_two_strings() {
        assert!(WHAT_THE_CONTROL_COVERS.ends_with('.'));
        assert!(THERE_IS_NO_WAY_BACK_ONE_AT_A_TIME.ends_with('.'));
        let bound = Prose::new(THERE_IS_NO_WAY_BACK_ONE_AT_A_TIME);
        assert!(
            bound.as_str().contains('\u{a0}'),
            "the last pair of the second sentence is not bound, which is what \
             handing it separately is for"
        );
    }

    /// **A platform that writes down what it was asked to do**, so §14 Q10 and
    /// Q13 can be driven through the **real** notification service rather than
    /// through a copy of it. The same shape piece G's own tests use, and the
    /// reason is that clause's: a gate somebody could forget to ask is not a
    /// mechanism.
    #[derive(Default)]
    struct WhatTheSystemWasAskedToDo {
        posted: std::sync::Mutex<Vec<String>>,
        taken_down: std::sync::Mutex<Vec<String>>,
    }

    impl crate::notifications::ThePlatform for std::sync::Arc<WhatTheSystemWasAskedToDo> {
        fn the_machinery(&self) -> crate::notifications::TheMachinery {
            crate::notifications::TheMachinery::ItIsInPlace
        }
        fn the_permission(&self) -> crate::notifications::permission::WhatTheSystemSays {
            crate::notifications::permission::WhatTheSystemSays::ItIsGranted
        }
        fn ask_for_permission_now(&self) {}
        fn post(&self, post: &APost) {
            if let Ok(mut posted) = self.posted.lock() {
                posted.push(post.key.as_text());
            }
        }
        fn take_down(&self, key: &ThePresentationKey) {
            if let Ok(mut taken_down) = self.taken_down.lock() {
                taken_down.push(key.as_text());
            }
        }
    }

    /// **Q10 and Q13, driven through the real notification service: a hundred
    /// failures on one condition identity reach the operating system once, and
    /// the repeats reach it not at all.**
    ///
    /// **This is Row 288's own example** — every file in a job failing to land
    /// in a folder that is full — and it is asked of the whole composed route
    /// rather than of the gate alone, because the fault this is written against
    /// is a raiser that posts without asking.
    ///
    /// **Q12 is the second half of it**: the record ends, the post comes down,
    /// and the condition that returns is a fresh identity said once again.
    #[test]
    fn a_hundred_failures_on_one_condition_reach_the_operating_system_once() {
        let system = std::sync::Arc::new(WhatTheSystemWasAskedToDo::default());
        let service = TheNotificationService::on(Box::new(system.clone()));
        // The record has answered with every kind at its default (T8).
        service.the_switches_are(Some(
            crate::record::settings::TheSwitches::nobody_has_turned_one(),
        ));
        let said_once = TheThingsSaidOnce::new();
        let condition = Identity::issue(smoozy_state::identities::IdentityKind::Condition)
            .expect("the desk has randomness");

        for _ in 0..100 {
            a_condition_was_raised_is_told_to_the_operating_system(
                &said_once,
                &service,
                &condition,
                Verbatim::new("Old 4080"),
            );
        }
        assert_eq!(
            system.posted.lock().expect("nothing poisoned it").len(),
            1,
            "**Q13: a later failure resolving to a standing identity needs no \
             post at all.** Posting every time and leaning on replace would put \
             a hundred notifications on the screen of any user whose system \
             turns out not to replace — Section 22 item 27 says that is \
             unestablished on every one of them"
        );

        // **Q12 and T24a**: the record's ending takes the post down and spends
        // the key.
        the_condition_record_ended(
            &said_once,
            &service,
            &condition,
            HowTheRecordEnded::TheUserClearedIt,
        );
        assert_eq!(
            system.taken_down.lock().expect("nothing poisoned it").len(),
            1
        );

        // A condition that returns is a fresh identity against the same key,
        // and is said once again.
        let again = Identity::issue(smoozy_state::identities::IdentityKind::Condition)
            .expect("the desk has randomness");
        a_condition_was_raised_is_told_to_the_operating_system(
            &said_once,
            &service,
            &again,
            Verbatim::new("Old 4080"),
        );
        assert_eq!(system.posted.lock().expect("nothing poisoned it").len(), 2);
    }

    /// **§4 A7: a module with no declaration cannot switch a warning off.**
    ///
    /// **This is G3's teeth.** A module that reached the table without a
    /// declaration would leave a row nothing could tie back to a module, and
    /// G4's *the module and the warning identity* would be false of it.
    #[test]
    fn an_undeclared_warning_cannot_be_switched_off() {
        let mut register = TheDeclarations::new();
        register.module_declares(
            "File Browser",
            Declared {
                dismissible_warnings: vec!["file-browser.delete"],
                ..Declared::default()
            },
        );
        assert!(register.the_warning_is_declared("File Browser", "file-browser.delete"));
        assert!(!register.the_warning_is_declared("File Browser", "file-browser.rename"));
        assert!(!register.the_warning_is_declared("Servers", "servers.disconnected-removal"));
    }
}
