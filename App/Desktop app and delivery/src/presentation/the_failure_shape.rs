//! **The one shape a failure is drawn in** — §16.1, §16.2, and
//! `Preparation/Design/Whole app.md`'s *The failure shape*.
//!
//! **Read the folder's head first.** This file is the shape itself; the head of
//! `src/presentation/mod.rs` is how a new drawing is added to it and what may
//! never be done inside it.
//!
//! ## The shape, in Design's own order
//!
//! *"A red dot, one plain sentence saying what did not happen, then — only
//! where Smoozy actually knows — a second sentence saying what to do about it,
//! then the controls: whatever act would try again, the raw text where there is
//! any, and a cross that clears it. Nothing else, in that order. The thing it
//! attaches to decides only how much room it has, never what it is made of."*
//!
//! **Two additions are permitted and only two.** The screen's own way past —
//! *"the act the user came to do, offered beside the retry"* — and **Report a
//! problem…**, after the raw text and before the cross, quiet. *"Those are the
//! only two additions anywhere."* [`Drawn::controls`] is built in that fixed
//! order and there is no third field a drawing could add one through.
//!
//! ## The three things that are not fields, and each is why
//!
//! - **The red dot.** Every failure has one, so it is not a value a drawing
//!   supplies and cannot be a value a drawing forgets. The page draws it.
//! - **The cross.** Design: *"only on a failure that describes something still
//!   true … A job's recorded ending is not a condition and carries no cross at
//!   all."* That is [`Kind`] and nothing else, so the cross is derived from the
//!   record's kind rather than passed in. **M5 and `State, identity and
//!   recovery` F4 are what it costs to get wrong**: a cross on an ending would
//!   offer to clear a fact.
//! - **Report a problem…** M33: *"The Report a problem… control is part of the
//!   one failure shape this plumbing draws, so a module that presents a failure
//!   through this section carries the handover by construction, present and
//!   future modules alike."* A drawing cannot switch it off and does not have
//!   to remember it on.
//!
//! ## The two sentences are two strings, and that is X10
//!
//! X2: *a piece of prose is one string as handed in*, and X10: *"where the
//! orphan rule must hold inside a multi-sentence block, the caller hands each
//! sentence as its own string."* **So the shape holds two sentence fields
//! rather than one paragraph**, and §22 item 28's narrowing does not reach a
//! failure drawn through this file.
//!
//! ## The sentence is composed before the point and drawn after it
//!
//! [`TheFailureShape::what_did_not_happen`] is a plain `String` — **the
//! composed sentence, before X7's two substitutions.** [`Drawn`] puts it
//! through [`Prose::new`] on the way to the screen, and
//! [`super::the_two_hooks`] captures the plain one for the diary. M32 is
//! explicit that the diary holds the words *"before X7's two substitutions"*
//! because *"the typography point is on the path to the screen and on no other
//! path, and the diary is not a screen"*. **The two orders are deliberate and
//! the types are what stops them being collapsed**: there is no `Prose` in this
//! struct to capture by mistake.
//!
//! ## Markers used in this file that belong to another document
//!
//! `State, identity and recovery` F1, F4, F6, F17. `Reporting a problem` X4.
//! **`Desktop app and delivery`'s own M-, X- and Q-markers are written bare.**

use crate::presentation::the_presentation_key::TheKeyAFailureIsDrawnOn;
use crate::presentation::the_two_hooks::TheFailureHandover;
use crate::record::failure_records::{Kind, Standing};
use crate::typography::{Prose, Verbatim};

/// **How much room the surface has** — Design: *"The thing it attaches to
/// decides only how much room it has, never what it is made of."*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Room {
    /// The whole shape.
    Expanded,
    /// **Collapsed onto a single line of a list row.** *"What survives is fixed
    /// in its order and in what may be added to it rather than fixed at a
    /// count. The red dot, then the plain sentence, then the cross where the
    /// failure describes something still true."* The second sentence, the raw
    /// text and reporting *"wait for the surface the row opens onto rather than
    /// being squeezed in — a row that grew to hold them would shift everything
    /// under it down the column."*
    ///
    /// **Design names exactly those three and the retry is not one of them**, so
    /// the act that would try again is drawn here as well. See
    /// [`TheFailureShape::drawn`] for the whole of why.
    ACollapsedRow,
}

/// **What one control in the shape is.** In the shape's fixed order and never
/// in the caller's.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum WhatItIs {
    /// *"whatever act would try again"*.
    TryAgain,
    /// **The first permitted addition** — *"Where a failure sits in a box the
    /// user is standing in front of, the screen that owns the box may add its
    /// own way past — the act the user came to do, offered beside the retry"*.
    TheScreensOwnWayPast,
    /// *"the raw text where there is any"*, and **the control names whose text
    /// it is** — *Blender output*, *What the server said*.
    TheRawText,
    /// **The second and last permitted addition**, quiet, after the raw text
    /// and before the cross.
    ReportAProblem,
    /// *"a cross that clears it"* — on a condition and never on an ending.
    TheCross,
}

/// One act a drawing offers. **The label and the act are two strings on two
/// routes** — the label is prose and goes through the typography point, the act
/// is a token the page routes on and Smoozy never alters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnAct {
    /// The label, **as composed and before the point's substitutions**.
    pub label: String,
    /// What the page calls back with. A value, not prose.
    pub act: String,
}

/// One control as the page receives it.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AControl {
    pub what_it_is: WhatItIs,
    /// Prose, through the point (X1).
    pub label: Prose,
    /// A value, by the verbatim route (X9). Empty for the cross, which is the
    /// shape's own act rather than a drawing's.
    pub act: Verbatim,
}

/// **What a drawing composes, and the only thing this folder will draw.**
///
/// A drawing fills this and hands it over. It cannot add a part, reorder the
/// parts, or leave out a part the shape supplies — which is what makes *a
/// failure has one shape wherever it attaches* a property of the code rather
/// than of eleven screens agreeing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheFailureShape {
    /// **Q1's key**, which is what said-once and the notification replacement
    /// both turn on — widened by one member for the one ending whose subject
    /// has no identity of its own. See [`TheKeyAFailureIsDrawnOn`].
    pub key: TheKeyAFailureIsDrawnOn,
    /// **The plain sentence saying what did not happen.** Composed, before the
    /// point's substitutions.
    pub what_did_not_happen: String,
    /// **The second sentence, only where Smoozy actually knows** — Design's own
    /// *"only where"*. Composed, before the point's substitutions.
    pub what_to_do_about_it: Option<String>,
    /// The act that would try again, where there is one. **Where the act is a
    /// control the surface already carries, the failure does not repeat it**
    /// (Design), and that is this being `None`.
    pub the_act_that_would_try_again: Option<AnAct>,
    /// The screen's own way past — the first permitted addition.
    pub the_screens_own_way_past: Option<AnAct>,
    /// *"the raw text where there is any"*. **One control per record the user
    /// can open, and the label names whose words they are** — *Blender output*,
    /// *What the server said*.
    ///
    /// **Corrected by piece D, 2026-09-12. This read *one per stream*, and
    /// Design says the opposite**: *"One panel serves every one of them … A
    /// render has two streams and they are kept apart, so it has two tabs;
    /// anything with one stream has none."* So a render supplies **one** control
    /// and the panel behind it gives its two streams a tab each, which is where
    /// M13a's *under the name of the stream it came from* is kept — at the tabs,
    /// in `src/presentation/the_machines_own_words.rs`. Two controls would be
    /// two panels, which is the drawing Design refused.
    pub the_raw_text: Vec<AnAct>,
    /// **Every name, path, host, port or machine word the drawing shows beside
    /// the sentence** — each by the verbatim route and never joined into the
    /// sentence. See the folder's head: joining one into the sentence breaks X7
    /// and no test anywhere would notice.
    pub the_values_beside_it: Vec<(&'static str, String)>,
}

/// One failure as the page receives it. **Composed here and never on the
/// page.**
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Drawn {
    /// Q1's key, written. What said-once counts on and what a notification is
    /// replaced on.
    pub key: String,
    /// **M4: the record's kind, read rather than guessed.** *condition* or
    /// *ending*, and the page draws the distinction rather than deriving it.
    pub kind: &'static str,
    /// §8.7's durable subject, and which of that list it is.
    pub subject: String,
    pub subject_kind: &'static str,
    /// The condition identity or the subject outcome mark.
    pub identity_or_mark: String,
    /// The plain sentence, through the point.
    pub what_did_not_happen: Prose,
    /// The second sentence, through the point, where there is one.
    pub what_to_do_about_it: Option<Prose>,
    /// The controls, **in the shape's fixed order**.
    pub controls: Vec<AControl>,
    /// Values shown beside the sentence, by the verbatim route.
    pub the_values_beside_it: Vec<(&'static str, Verbatim)>,
    /// **M33's closed handover, present on every expanded presentation and on
    /// no collapsed row** — which is exactly where the control is drawn.
    pub the_report_handover: Option<TheFailureHandover>,
}

impl TheFailureShape {
    /// **§16.2 M4, M5 and `State, identity and recovery` F6: present it while,
    /// and only while, the current observed value is one of the values it was
    /// raised over.**
    ///
    /// `observed_now` is what §8.3's **named supplier** last reported.
    ///
    /// - **A value outside the set answers `None`** — the record is withdrawn
    ///   *by that change alone*, and there is no gesture, no sweep and no second
    ///   mechanism anywhere in this folder.
    /// - **No value at all leaves the record standing**, which is M7 and
    ///   `State, identity and recovery` F17's declared narrowing carried rather
    ///   than hidden: *"Where nothing has refreshed a condition, the record
    ///   stands until the user clears it"*, and **nothing here adds a watcher to
    ///   close it**.
    /// - **An ending record is not withdrawn by a value at all**, because it is
    ///   a statement about something that has already happened.
    ///
    /// **Nothing here holds a second copy of the value** (M6). `observed_now`
    /// is borrowed for the length of this call and is not kept.
    pub fn presented(
        self,
        record: &Standing,
        observed_now: Option<&str>,
        room: Room,
    ) -> Option<Drawn> {
        if let Some(observed_now) = observed_now {
            if !record.still_holds_at(observed_now) {
                return None;
            }
        }
        Some(self.drawn(record, room))
    }

    /// The drawing itself, once the record has been found to stand.
    fn drawn(self, record: &Standing, room: Room) -> Drawn {
        let mut controls = Vec::new();

        // **The act that would try again is drawn on a collapsed row too, and
        // that is Design read rather than assumed.** Design names **exactly
        // three** things a single line has no room for — the second sentence,
        // the raw text and reporting — and the retry is not among them; it says
        // the collapsed shape is *"fixed in its order and in what may be added
        // to it rather than fixed at a count"*. Its rule about the retry is
        // **conditional**: *"where the act that would try again is a control the
        // row already carries, the failure does not repeat it"*, which only
        // means anything where the failure carries one and the row does not.
        // **That condition is expressed by the caller supplying nothing**, which
        // is what [`TheFailureShape::the_act_that_would_try_again`]'s own note
        // already says the field is for.
        //
        // **Until 2026-09-12 this sat inside the expanded branch**, so a failure
        // on a row carrying no retry of its own showed a dot, a sentence and a
        // cross, and the only thing the user could do about it was make the
        // message go away — with `Preparation/Specs/Whole app.md`'s *"Whatever
        // the user can do about it stays available"* not kept on that surface.
        // Found at round 1 of the code audit of what is drawn inside the window
        // (F-12).
        if let Some(act) = &self.the_act_that_would_try_again {
            controls.push(a_control(WhatItIs::TryAgain, act));
        }

        if room == Room::Expanded {
            if let Some(act) = &self.the_screens_own_way_past {
                controls.push(a_control(WhatItIs::TheScreensOwnWayPast, act));
            }
            for act in &self.the_raw_text {
                controls.push(a_control(WhatItIs::TheRawText, act));
            }
            // **The second and last addition, supplied by the shape and never
            // by a drawing** — M33. Quiet and last before the cross: Design's
            // *"a loud control here would be actively wrong"*, because a
            // connection dropping when the user shut their laptop is the
            // commonest failure in the app and is nobody's bug.
            controls.push(AControl {
                what_it_is: WhatItIs::ReportAProblem,
                label: Prose::new(REPORT_A_PROBLEM),
                act: Verbatim::new(""),
            });
        }

        // **The cross, on a condition and never on an ending** — Design, and
        // M5: clearing takes the mark and never the fact.
        if record.kind == Kind::Condition {
            controls.push(AControl {
                what_it_is: WhatItIs::TheCross,
                label: Prose::new(CLEAR_THIS),
                act: Verbatim::new(""),
            });
        }

        Drawn {
            key: self.key.as_text(),
            kind: record.kind.as_text(),
            subject: record.subject.as_text(),
            subject_kind: record.subject.kind_as_text(),
            identity_or_mark: record.identity_or_mark.clone(),
            what_did_not_happen: Prose::new(self.what_did_not_happen.clone()),
            what_to_do_about_it: match room {
                // *"The second sentence … is what a line has no room for."*
                Room::ACollapsedRow => None,
                Room::Expanded => self.what_to_do_about_it.clone().map(Prose::new),
            },
            controls,
            the_values_beside_it: self
                .the_values_beside_it
                .iter()
                .map(|(name, value)| (*name, Verbatim::new(value.clone())))
                .collect(),
            the_report_handover: match room {
                Room::ACollapsedRow => None,
                Room::Expanded => Some(TheFailureHandover::of(record, &self.what_did_not_happen)),
            },
        }
    }
}

/// The report control's label. **One string, in one place**, because it is the
/// shape's own control rather than a drawing's.
pub const REPORT_A_PROBLEM: &str = "Report a problem…";

/// The clearing gesture's label, for whatever needs to name it. **The gesture
/// itself is Design's** by that concern's own marker, and this document
/// presents it; what is fixed here is that it ends the record and nothing else.
pub const CLEAR_THIS: &str = "Dismiss";

fn a_control(what_it_is: WhatItIs, act: &AnAct) -> AControl {
    AControl {
        what_it_is,
        // Prose, through the point.
        label: Prose::new(act.label.clone()),
        // A value, by the verbatim route. **Two strings on two routes**, which
        // is the trap the folder's head names.
        act: Verbatim::new(act.act.clone()),
    }
}

/// **M7a: two conditions standing at the same moment are presented as two,
/// whatever set them off.**
///
/// This function exists to be the only way a set of standing records reaches a
/// surface, so that *nothing composes a third statement about a shared cause*
/// is a property of there being nowhere to put one. **Each record is drawn
/// exactly as it would be were it the only one standing**, and the answer is a
/// list rather than anything derived from the list.
///
/// *"Each is its own record against its own durable subject, each is shown and
/// withdrawn on its own kind's columns under M4, and neither is hidden, held
/// back or reworded because the other stands."*
pub fn each_on_its_own(drawings: Vec<Drawn>) -> Vec<Drawn> {
    drawings
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::presentation::the_presentation_key::ThePresentationKey;
    use crate::record::failure_records::DurableSubject;
    use smoozy_state::identities::{Identity, IdentityKind};

    fn an_identity(kind: IdentityKind) -> Identity {
        Identity::issue(kind).expect("the desk has randomness")
    }

    fn a_condition_record() -> Standing {
        let condition = an_identity(IdentityKind::Condition);
        Standing {
            kind: Kind::Condition,
            subject: DurableSubject::ServerRow(an_identity(IdentityKind::LocalServer)),
            identity_or_mark: condition.as_text(),
            the_values_it_was_raised_over: vec![
                "disconnected".to_string(),
                "reconnecting".to_string(),
            ],
            the_diary_holds_the_sentence: false,
        }
    }

    fn an_ending_record() -> Standing {
        Standing {
            kind: Kind::Ending,
            subject: DurableSubject::HistoryRecord(an_identity(IdentityKind::Job)),
            identity_or_mark: "the render did not finish".to_string(),
            the_values_it_was_raised_over: Vec::new(),
            the_diary_holds_the_sentence: false,
        }
    }

    fn a_shape(record: &Standing) -> TheFailureShape {
        TheFailureShape {
            key: TheKeyAFailureIsDrawnOn::OnAnIdentity(match record.kind {
                Kind::Condition => ThePresentationKey::ACondition(an_identity(
                    IdentityKind::Condition,
                )),
                Kind::Ending => ThePresentationKey::AJobsEnding(an_identity(IdentityKind::Job)),
            }),
            what_did_not_happen: "Smoozy could not reach the server.".to_string(),
            what_to_do_about_it: None,
            the_act_that_would_try_again: Some(AnAct {
                label: "Retry".to_string(),
                act: "servers.connect".to_string(),
            }),
            the_screens_own_way_past: None,
            the_raw_text: Vec::new(),
            the_values_beside_it: Vec::new(),
        }
    }

    /// **M5 and `State, identity and recovery` F6: the value moving outside the
    /// set withdraws it, and by that change alone.**
    #[test]
    fn a_condition_is_shown_while_and_only_while_its_value_is_in_the_set() {
        let record = a_condition_record();
        assert!(a_shape(&record)
            .presented(&record, Some("reconnecting"), Room::Expanded)
            .is_some());
        assert!(a_shape(&record)
            .presented(&record, Some("disconnected"), Room::Expanded)
            .is_some());
        assert!(a_shape(&record)
            .presented(&record, Some("connected"), Room::Expanded)
            .is_none());
    }

    /// **M7 and `State, identity and recovery` F17: the narrowing is carried
    /// and not hidden.** Where nothing has refreshed a condition, the record
    /// stands until the user clears it, and nothing here adds a watcher to
    /// close it.
    #[test]
    fn a_condition_nothing_has_refreshed_stands() {
        let record = a_condition_record();
        assert!(a_shape(&record)
            .presented(&record, None, Room::Expanded)
            .is_some());
    }

    /// **Design, and M5: the cross is only on a failure that describes
    /// something still true.** A job's recorded ending carries no cross at all.
    #[test]
    fn an_ending_carries_no_cross_and_a_condition_does() {
        let condition = a_condition_record();
        let drawn = a_shape(&condition)
            .presented(&condition, Some("disconnected"), Room::Expanded)
            .unwrap();
        assert!(drawn
            .controls
            .iter()
            .any(|control| control.what_it_is == WhatItIs::TheCross));

        let ending = an_ending_record();
        let drawn = a_shape(&ending)
            .presented(&ending, None, Room::Expanded)
            .unwrap();
        assert!(drawn
            .controls
            .iter()
            .all(|control| control.what_it_is != WhatItIs::TheCross));
    }

    /// **Design's order, and the two permitted additions in their places.**
    #[test]
    fn the_controls_are_in_the_shapes_order_and_a_drawing_cannot_reorder_them() {
        let record = a_condition_record();
        let mut shape = a_shape(&record);
        shape.the_screens_own_way_past = Some(AnAct {
            label: "Remove anyway".to_string(),
            act: "servers.remove".to_string(),
        });
        shape.the_raw_text = vec![
            AnAct {
                label: "Blender output".to_string(),
                act: "words.ordinary".to_string(),
            },
            AnAct {
                label: "What the server said".to_string(),
                act: "words.complaints".to_string(),
            },
        ];
        let drawn = shape
            .presented(&record, Some("disconnected"), Room::Expanded)
            .unwrap();
        let order: Vec<WhatItIs> = drawn
            .controls
            .iter()
            .map(|control| control.what_it_is)
            .collect();
        assert_eq!(
            order,
            vec![
                WhatItIs::TryAgain,
                WhatItIs::TheScreensOwnWayPast,
                WhatItIs::TheRawText,
                WhatItIs::TheRawText,
                WhatItIs::ReportAProblem,
                WhatItIs::TheCross,
            ]
        );
    }

    /// **Design's collapsed form**: dot, plain sentence, and the cross where the
    /// failure describes something still true. **The second sentence, the raw
    /// text and reporting wait for the surface the row opens onto.**
    #[test]
    fn a_collapsed_row_carries_the_cross_and_none_of_the_three_it_has_no_room_for() {
        let record = a_condition_record();
        let mut shape = a_shape(&record);
        shape.what_to_do_about_it = Some("Check the machine is still rented.".to_string());
        shape.the_raw_text = vec![AnAct {
            label: "What the server said".to_string(),
            act: "words".to_string(),
        }];
        let drawn = shape
            .presented(&record, Some("disconnected"), Room::ACollapsedRow)
            .unwrap();

        assert!(drawn.what_to_do_about_it.is_none());
        assert_eq!(
            drawn
                .controls
                .iter()
                .map(|control| control.what_it_is)
                .collect::<Vec<_>>(),
            vec![WhatItIs::TryAgain, WhatItIs::TheCross]
        );
        // **M33: the handover exists wherever the control is drawn**, and the
        // collapsed row does not carry the control.
        assert!(drawn.the_report_handover.is_none());
    }

    /// **The retry is drawn on a collapsed row, and a caller supplying nothing
    /// is how Design's *the row already carries it* is expressed.**
    ///
    /// Design names exactly three things a line has no room for and the retry is
    /// not one of them. **A failure on a row that carries no retry of its own
    /// otherwise offers the user nothing but making the message go away**, which
    /// is `Preparation/Specs/Whole app.md`'s *"Whatever the user can do about it
    /// stays available"* not kept on that surface.
    #[test]
    fn a_collapsed_row_carries_the_retry_and_carries_none_where_the_caller_offers_none() {
        let record = a_condition_record();

        let drawn = a_shape(&record)
            .presented(&record, None, Room::ACollapsedRow)
            .unwrap();
        assert_eq!(
            drawn
                .controls
                .iter()
                .map(|control| control.what_it_is)
                .collect::<Vec<_>>(),
            vec![WhatItIs::TryAgain, WhatItIs::TheCross]
        );

        // **The row already carries the act**, so the caller offers none and the
        // failure does not repeat it.
        let mut shape = a_shape(&record);
        shape.the_act_that_would_try_again = None;
        let drawn = shape
            .presented(&record, None, Room::ACollapsedRow)
            .unwrap();
        assert_eq!(
            drawn
                .controls
                .iter()
                .map(|control| control.what_it_is)
                .collect::<Vec<_>>(),
            vec![WhatItIs::TheCross]
        );
    }

    /// **M33: every expanded failure presentation supplies the handover, by
    /// construction.** A drawing has no field it could leave out.
    #[test]
    fn every_expanded_presentation_carries_the_report_control_and_the_handover() {
        for record in [a_condition_record(), an_ending_record()] {
            let drawn = a_shape(&record)
                .presented(&record, None, Room::Expanded)
                .unwrap();
            assert!(drawn
                .controls
                .iter()
                .any(|control| control.what_it_is == WhatItIs::ReportAProblem));
            assert!(drawn.the_report_handover.is_some());
        }
    }

    /// **M4: the record carries its kind and the presentation reads it.** The
    /// drawn form says which, so nothing downstream guesses.
    #[test]
    fn the_drawn_form_says_which_kind_the_record_is() {
        let condition = a_condition_record();
        assert_eq!(
            a_shape(&condition)
                .presented(&condition, None, Room::Expanded)
                .unwrap()
                .kind,
            "condition"
        );
        let ending = an_ending_record();
        assert_eq!(
            a_shape(&ending)
                .presented(&ending, None, Room::Expanded)
                .unwrap()
                .kind,
            "ending"
        );
    }

    /// **X10: the two sentences are two strings**, so the orphan rule holds at
    /// the end of each rather than only at the end of the block.
    #[test]
    fn each_sentence_is_bound_on_its_own() {
        let record = a_condition_record();
        let mut shape = a_shape(&record);
        shape.what_did_not_happen = "Smoozy could not reach the server.".to_string();
        shape.what_to_do_about_it = Some("Check the machine is still rented.".to_string());
        let drawn = shape
            .presented(&record, None, Room::Expanded)
            .unwrap();
        assert!(drawn.what_did_not_happen.as_str().contains('\u{00A0}'));
        assert!(drawn
            .what_to_do_about_it
            .as_ref()
            .unwrap()
            .as_str()
            .contains('\u{00A0}'));
    }

    /// **X7: a name takes the verbatim route and never the point's.** The
    /// sentence and the name are two strings that never meet.
    #[test]
    fn a_name_beside_the_sentence_is_never_touched_by_the_point() {
        let record = a_condition_record();
        let mut shape = a_shape(&record);
        // A path carrying both of the point's triggers.
        shape.the_values_beside_it =
            vec![("the folder", "/Volumes/Work — renders/final one".to_string())];
        let drawn = shape
            .presented(&record, None, Room::Expanded)
            .unwrap();
        assert_eq!(
            drawn.the_values_beside_it[0].1.as_str(),
            "/Volumes/Work — renders/final one"
        );
    }

    /// **M7a: two standing conditions are two drawings and nothing composes a
    /// third.**
    #[test]
    fn two_standing_conditions_are_drawn_as_two() {
        let one = a_condition_record();
        let other = a_condition_record();
        let drawings = each_on_its_own(vec![
            a_shape(&one).presented(&one, None, Room::Expanded).unwrap(),
            a_shape(&other)
                .presented(&other, None, Room::Expanded)
                .unwrap(),
        ]);
        assert_eq!(drawings.len(), 2);
        assert_ne!(drawings[0].identity_or_mark, drawings[1].identity_or_mark);
    }
}
