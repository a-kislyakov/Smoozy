//! **The failure shape's own service** — §16, and the page half is
//! `web/the-failure-shape.js`.
//!
//! **This file is the surface and not the shape.** The shape, and every rule a
//! drawing inside it keeps, is at the head of `src/presentation/mod.rs`; read
//! that before adding anything here.
//!
//! ## Two commands, and why the answer arrives as an event
//!
//! P6: *results reach the interface as events on one ordered queue, and the
//! interface holds no other route to a module.* **A command is the page asking;
//! an event is the application telling.** So neither command below returns a
//! failure: each reads the record and posts, through
//! [`crate::events::TheOneQueue::read_then_post`] and
//! [`crate::events::TheOneQueue::write_then_post`], which are P7's two doors —
//! the durable act runs first and a failed one posts nothing.
//!
//! ## What the queue is told about the subject, and why it is told here
//!
//! P8 has the queue refuse an event about a durable subject it has not been
//! told about, and piece A's record says *whoever creates a durable subject
//! calls `the_subject_exists`*. **None of those creators is built.** What is
//! built is this: **a subject read back out of the local record is a subject
//! that exists**, and reading it is how this side knows. So the presentation
//! tells the queue at the moment it has just read the subject's records, which
//! is a fact rather than an assumption, and tells it the subject is gone at
//! [`crate::record::failure_records::FailureRecords::the_subject_was_removed`]'s
//! own call site. **When the owning modules arrive they call
//! `the_subject_exists` at creation and this stops being the first caller
//! rather than becoming a second answer** — telling the queue twice about one
//! subject is one entry in a set.
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
use crate::presentation::the_failure_shape::{
    AControl, AnAct, Drawn, Room, TheFailureShape, WhatItIs,
};
use crate::presentation::the_presentation_key::{TheKeyAFailureIsDrawnOn, ThePresentationKey};
use crate::presentation::the_two_hooks::WhoseSentenceItIs;
use crate::record::failure_records::{DurableSubject, FailureRecords, Kind, Standing};
use crate::start::Started;
use crate::typography::Verbatim;
use smoozy_state::identities::{Identity, IdentityKind};

/// **The kind the page listens for, one event per standing failure.**
///
/// Declared here, beside the service that posts it, and copied into
/// `web/the-failure-shape.js`. Nothing holds the two together and nothing can
/// without reading every page file for string literals, so copy carefully.
///
/// **One event per failure and never a list in one event.** M7a: two conditions
/// standing at once are presented as two, *"each … exactly as it would be were
/// it the only one standing"*, and a list in one event is a thing something
/// downstream would be tempted to summarise.
pub const A_FAILURE_IS_STANDING: &str = "presentation:a-failure-is-standing";

/// **The record ended, so the drawing goes.** Posted at the user's own clearing
/// gesture (`State, identity and recovery` F25's second ending) and at a
/// withdrawal (its third).
pub const A_FAILURE_WAS_CLEARED: &str = "presentation:a-failure-was-cleared";

/// **The clearing did not land, so the drawing stays and the user is told.**
///
/// **L8a: *"that screen shows the failure rather than an emptied version of
/// itself"*.** A refused delete used to post nothing at all — the cross did not
/// move, no failure was drawn anywhere, and what the user saw was a control that
/// did nothing twice. Added 2026-09-12 at round 1 of the code audit of what is
/// drawn inside the window (F-10).
pub const A_FAILURE_WAS_NOT_CLEARED: &str = "presentation:a-failure-was-not-cleared";

/// **The plumbing's own sentence for a clearing that did not land**, before the
/// typography point.
///
/// **The words are Design's and are owed** (§24), the same as
/// [`the_sentence_for`]'s. What is fixed here is that the press says something:
/// it names no cause, and it does not say the record is gone.
const THE_CLEARING_DID_NOT_LAND: &str = "Could not clear.";

/// **What is said where the failure records themselves will not read** — L8a,
/// and it is the opposite of drawing an empty screen.
///
/// **The words are Design's and are owed** (§24), the same as the two above.
/// What is fixed here is that a read which did not answer says so instead of
/// saying there is nothing wrong.
const THE_FAILURES_COULD_NOT_BE_READ: &str = "Could not read errors.";

/// **The mark a read that did not answer is drawn under.** It is not a subject
/// outcome and no store holds it; it exists because the shape takes one.
const THE_READ_DID_NOT_ANSWER: &str = "the failure records could not be read";

/// **The page asks what is standing against a subject.**
///
/// `subject_kind` is one of §8.7's closed list, written; `subject` is the
/// identity's own text. **H34: both are validated before anything is touched**
/// — a subject kind that is none of the list, or an identity that will not read
/// back, answers nothing rather than reaching the record.
#[tauri::command]
pub fn the_failures_standing_against(
    started: State<'_, Mutex<Started>>,
    queue: State<'_, std::sync::Arc<TheOneQueue>>,
    subject_kind: String,
    subject: String,
) {
    let Some(subject) = a_subject(&subject_kind, &subject) else {
        return;
    };
    let Ok(mut started) = started.lock() else {
        return;
    };
    the_failures_are_posted(&mut started, &queue, &subject);
}

/// **The user's own clearing gesture** — `State, identity and recovery` §8.8
/// `State, identity and recovery` F25's second ending, and M5: *"Clearing
/// takes the mark and never the
/// fact, and it reaches a condition record alone."* **A request to clear an
/// ending is refused** in [`the_user_cleared_one`] before anything is touched.
///
/// **The write runs first and the post follows it** (P7), so a clearing that did
/// not land takes nothing off the screen and the mark is still there when the
/// page next asks. **There is nothing in this path that could touch a subject
/// outcome**: the record holds no copy of one, so *the clearing gesture takes
/// the mark and never the fact* is true by there being no other field to reach.
///
/// **`kind` is the record's own kind, written** — `"condition"` or `"ending"`,
/// the same word [`crate::record::failure_records::Kind::as_text`] puts on the
/// standing event and the page draws the cross beside. **H34: it is validated
/// here like the other two** — a word that is neither is not a record's kind and
/// is refused rather than guessed at, and `"ending"` is read and then refused,
/// an ending not being the user's to clear.
///
/// **It is asked for because the key is composed from it.** `State, identity and
/// recovery` F2's own rule is that *every failure record carries its kind and
/// the presentation reads it rather than guessing*; the key a drawing is filed
/// under is the condition's identity for one kind and the subject's for the
/// other, so a clearing that does not carry the kind cannot name the drawing it
/// is about. Added 2026-09-12 at the close of round 1 (C-1).
#[tauri::command]
pub fn a_failure_was_cleared(
    started: State<'_, Mutex<Started>>,
    queue: State<'_, std::sync::Arc<TheOneQueue>>,
    kind: String,
    subject_kind: String,
    subject: String,
    identity_or_mark: String,
) {
    let Some(kind) = Kind::from_text(&kind) else {
        return;
    };
    let Some(subject) = a_subject(&subject_kind, &subject) else {
        return;
    };
    let Ok(started) = started.lock() else {
        return;
    };
    the_user_cleared_one(&started, &queue, kind, &subject, &identity_or_mark);
}

/// The clearing itself, with the spine handed in — **so the whole of it can be
/// driven without the shell's own managed state**, which the command cannot be:
/// that state has no constructor outside a running application. Piece B's
/// [`crate::surface::panes::a_drag_ended_on`] is the same split for the same
/// reason.
pub fn the_user_cleared_one(
    started: &Started,
    queue: &TheOneQueue,
    kind: Kind,
    subject: &DurableSubject,
    identity_or_mark: &str,
) {
    // **An ending is not the user's to clear, so a request to clear one is
    // refused before anything is touched** — the same silent H34 refusal made
    // below for a request whose values are not a record's.
    // `State, identity and recovery` F25 gives the user's clearing to a
    // condition record alone, and M5 says an ending's mark carries no clearing gesture: Whole app clears a
    // failure *"only where it describes something still true"*. The page draws
    // no cross on an ending, and this command can be reached without the page's
    // list, which is why it is refused here as well as never offered there.
    // Until 2026-09-19 this cleared the record on the plans' earlier reading.
    // The Gemini catch-up of step 10a, Area 4 (F-5, with U-1).
    if kind == Kind::Ending {
        return;
    }
    let Some(record) = started.record.as_ref() else {
        return;
    };
    // **The key is composed before anything is touched, and a request whose key
    // will not compose is refused rather than posted with an empty one.**
    //
    // **Until the close of round 1 this composed the key a second way** — it
    // read `identity_or_mark` back as a condition identity and nothing else — so
    // a **recorded ending**, whose mark is the owning module's words rather than
    // an identity, got no key at all and the call site wrote an empty string on
    // to the event. The page keys its drawings on that string (F-5), so the
    // press cleared the record in the store and **the drawing never left the
    // screen**, and the *did not land* line could not be drawn on an ending
    // either. **One composer now serves both** — [`the_key_a_record_is_drawn_on`]
    // — which is what stops the two ever disagreeing again. Found at the close
    // check of round 1 of the code audit of what is drawn inside the window
    // (C-1). **Since 2026-09-19 an ending never reaches this line**, being
    // refused above; the one composer still keys a standing ending's drawing,
    // so the press and the drawing still cannot disagree for a condition.
    //
    // **Refusing here says nothing to the user, and that is H34 rather than
    // F-10's silence.** A record whose key will not compose was never drawn:
    // [`a_shape`] refuses it and the page is handed no failure, so no cross
    // exists to press. What reaches this refusal is a request whose values are
    // not a record's — the same refusal [`a_subject`] already makes.
    let Some(key) = the_key_a_record_is_drawn_on(kind, subject, identity_or_mark) else {
        return;
    };
    let key = key.as_text();
    let about = match subject.identity() {
        Some(identity) => Subject::Durable(identity.clone()),
        None => Subject::TheRun,
    };
    let mut records = FailureRecords::over(record.connection());
    let _ = queue.write_then_post::<WhatThePressDid, std::convert::Infallible>(
        // **The refusal is what the durable act answers with, rather than an
        // error that leaves the queue with nothing to say.** P7's door runs the
        // durable act first and composes the event from what it returned; a
        // refusal **is** what it returned, so saying so keeps the door exactly
        // as it is.
        || {
            Ok(match records.the_user_cleared_it(subject, identity_or_mark) {
                Ok(()) => WhatThePressDid::ItLanded,
                Err(refused) => WhatThePressDid::ItDidNot(format!("{refused:?}")),
            })
        },
        |did| {
            let event = match did {
                WhatThePressDid::ItLanded => Event::about(about.clone(), A_FAILURE_WAS_CLEARED),
                // **F-10's rule: no press of a control in this Subject may end
                // in nothing being said.** A refused delete used to post
                // nothing at all — the cross did not move, no failure was
                // drawn, and the user pressed it again to the same silence.
                WhatThePressDid::ItDidNot(_) => {
                    Event::about(about.clone(), A_FAILURE_WAS_NOT_CLEARED).saying(
                        "sentence",
                        Said::Prose(crate::typography::Prose::new(THE_CLEARING_DID_NOT_LAND)),
                    )
                }
            };
            event
                .saying("subject", Said::Value(Verbatim::new(subject.as_text())))
                .saying(
                    "subjectKind",
                    Said::Value(Verbatim::new(subject.kind_as_text())),
                )
                .saying(
                    "identityOrMark",
                    Said::Value(Verbatim::new(identity_or_mark.to_string())),
                )
                // **The key, which is the one thing that cannot collide** — the
                // page keys its drawings on it (F-5), so the clearing has to
                // name it or the wrong drawing leaves the screen.
                .saying("key", Said::Value(Verbatim::new(key.clone())))
        },
    );
}

/// What came of a press. **Not an error**: the act ran, and this is what it did.
enum WhatThePressDid {
    ItLanded,
    ItDidNot(String),
}

/// Read what stands against the subject, draw each on its own, and post one
/// event each.
///
/// **M7a: each is drawn exactly as it would be were it the only one standing.**
///
/// **M32's hook fires here**, at the first presentation of each record, and its
/// answer never reaches the queue: the diary is not a surface.
pub fn the_failures_are_posted(
    started: &mut Started,
    queue: &TheOneQueue,
    subject: &DurableSubject,
) {
    let Some(record) = started.record.as_ref() else {
        return;
    };
    let read = FailureRecords::over(record.connection()).standing_against(subject);

    // **P8, and it is told rather than assumed**: the subject was reached in the
    // durable store, so it exists. **A read that failed reached it too** — the
    // engine answered about that subject's own rows — and the queue has to have
    // been told before the failure below can be posted about it.
    if let Some(identity) = subject.identity() {
        queue.the_subject_exists(identity.clone());
    }
    let about = match subject.identity() {
        Some(identity) => Subject::Durable(identity.clone()),
        // Legacy subjects without an identity use the run’s presentation.
        None => Subject::TheRun,
    };

    let standing = match read {
        Ok(standing) => standing,
        // **L8a, and this is the whole of why the store fault below it went
        // unseen for a day.** This used to `return`: the read failed, nothing
        // was drawn, nothing was logged and nothing anywhere said so. On the
        // Creator's own record every one of these reads was failing, so **no
        // failure of any kind reached a screen in the whole application** and
        // the app looked exactly like an app with nothing wrong with it.
        //
        // **L8a is the plan's answer and it is built rather than invented**:
        // *"that screen shows the failure rather than an emptied version of
        // itself"*, and *"there is no state in which a list is drawn short …
        // because a read failed"*. An empty list of standing failures **is**
        // that screen emptied.
        //
        // **What is posted is a failure and not a record**, and the difference
        // is worth holding: nothing is written, nothing is keyed on a stored
        // identity, and the drawing goes when the page next asks and the read
        // answers. See [`the_read_did_not_answer`].
        Err(_) => {
            let did_not_answer = the_read_did_not_answer(subject);
            let sentence = THE_FAILURES_COULD_NOT_BE_READ.to_string();
            // **Keyed as a read that did not answer about this subject, and
            // never as a job's ending.** Until 2026-09-19 it went through the
            // record's own key composer as an ending, which wrapped whatever
            // subject was asked about — a server row on the Servers screen —
            // as `job:<identity>`. When that composer stopped inventing a key
            // for an ending against a server row (the Gemini catch-up of step
            // 10a, Area 4, F-2), this drawing would have lost its key and L8a's
            // failure would have left the screen; it has a key of its own
            // instead.
            let shape = the_shape_on(
                TheKeyAFailureIsDrawnOn::AReadThatDidNotAnswer(
                    the_read_did_not_answer_about(subject),
                ),
                &sentence,
            );
            if let Some(drawn) = shape.presented(&did_not_answer, None, Room::Expanded) {
                let _ = queue.read_then_post::<Drawn, ()>(
                    || Ok(drawn.clone()),
                    |drawn| the_event_for(about.clone(), drawn, &sentence),
                );
            }
            return;
        }
    };

    for record in &standing {
        // **The composed sentence, before X7's substitutions.** The screen
        // takes it again through the point.
        let sentence = the_sentence_for(record);

        // **A record whose key cannot be composed is skipped and nothing is
        // invented to key it with** — see [`the_key_for`].
        let Some(shape) = a_shape(record, &sentence) else {
            continue;
        };
        let Some(drawn) = shape.presented(record, None, Room::Expanded) else {
            continue;
        };

        // **M32's hook, after the presentation has been decided and not
        // before.** Until 2026-09-12 it ran first, so a record found not to
        // stand still handed the diary a sentence nobody met. Found at round 1
        // of the code audit of what is drawn inside the window (F-4).
        //
        // **What it hands over is a stand-in and the capture is therefore not
        // spent** — M32: *"the capture belongs to whatever composes the sentence
        // the user reads, and a path that composes a stand-in sentence captures
        // nothing"*. [`the_sentence_for`] is the plumbing's own words and not a
        // screen's, so the record's one capture stays unspent for the screen
        // that later composes the sentence the user actually reads.
        {
            let Some(local) = started.record.as_ref() else {
                return;
            };
            let mut records = FailureRecords::over(local.connection());
            crate::presentation::the_two_hooks::the_diary_is_handed_the_sentence(
                started.diary.as_ref(),
                &mut records,
                started.run.as_ref(),
                record,
                whose_sentence_it_is(record, &sentence),
            );
        }
        let _ = queue.read_then_post::<Drawn, ()>(
            || Ok(drawn.clone()),
            |drawn| the_event_for(about.clone(), drawn, &sentence),
        );
    }
}

/// One drawn failure as an event. **Every string is on one of the two routes
/// and the two are the complete set** (A6, X9).
fn the_event_for(about: Subject, drawn: &Drawn, before_the_point: &str) -> Event {
    let mut event = Event::about(about, A_FAILURE_IS_STANDING)
        // Prose, through the point.
        .saying("sentence", Said::Prose(drawn.what_did_not_happen.clone()))
        // Values, by the verbatim route.
        .saying("kind", Said::Value(Verbatim::new(drawn.kind)))
        .saying("subject", Said::Value(Verbatim::new(drawn.subject.clone())))
        .saying(
            "subjectKind",
            Said::Value(Verbatim::new(drawn.subject_kind)),
        )
        .saying(
            "identityOrMark",
            Said::Value(Verbatim::new(drawn.identity_or_mark.clone())),
        )
        .saying("key", Said::Value(Verbatim::new(drawn.key.clone())))
        // **M33's handover, on the event.** The sentence in it is the one
        // captured **before** the point, which is why it cannot be composed
        // from `sentence` above — and why it goes by the verbatim route: X7's
        // list includes *any string that is about to be written anywhere*, and
        // this one is written into a report.
        .saying(
            "theSentenceForTheReport",
            Said::Value(Verbatim::new(before_the_point)),
        );
    if let Some(what_to_do) = &drawn.what_to_do_about_it {
        event = event.saying("whatToDoAboutIt", Said::Prose(what_to_do.clone()));
    }
    if let Some(handover) = &drawn.the_report_handover {
        event = event.saying(
            "at",
            Said::Value(Verbatim::new(
                handover
                    .at
                    .map(|at| at.to_string())
                    .unwrap_or_else(String::new),
            )),
        );
    }
    // **Every value the drawing puts beside the sentence, each on the verbatim
    // route and each under its own name** — never joined into the sentence,
    // which is X7's trap and the folder head's rule 4.
    for (name, value) in &drawn.the_values_beside_it {
        event = event.saying(name, Said::Value(value.clone()));
    }
    the_controls_on_the_event(event, &drawn.controls)
}

/// **The keys one control travels on.** Three each — what it is, its label, its
/// act — and as many sets as the shape can put in a list.
///
/// **There are eight because the shape's own order has five kinds and the raw
/// text is a list**; a drawing offering more controls than this has left
/// Design's shape behind, and the sweep below says so rather than dropping
/// them quietly.
const THE_CONTROLS_OWN_KEYS: [(&str, &str, &str); 8] = [
    ("control0Is", "control0Label", "control0Act"),
    ("control1Is", "control1Label", "control1Act"),
    ("control2Is", "control2Label", "control2Act"),
    ("control3Is", "control3Label", "control3Act"),
    ("control4Is", "control4Label", "control4Act"),
    ("control5Is", "control5Label", "control5Act"),
    ("control6Is", "control6Label", "control6Act"),
    ("control7Is", "control7Label", "control7Act"),
];

/// **The controls, in the shape's fixed order, on the event.**
///
/// **Design: *"A failure has one shape wherever it attaches."*** The application
/// composes the controls in Design's order and a test asserts that order — and
/// until 2026-09-12 the event then **dropped them**, so the page composed the
/// order a second time out of what it was given and supplied the shape's own two
/// labels itself. The two orders agreed; nothing held them together, and the day
/// somebody edited one list the test would still have passed. **This is the same
/// fact kept in two places, which the head of `src/presentation/mod.rs` names as
/// the fault this Build stage keeps meeting.** Found at round 1 of the code
/// audit of what is drawn inside the window (F-13).
///
/// **The label is prose and the act is a value** — two strings on two routes, so
/// the labels of the shape's own controls (*Report a problem…*, *Dismiss*)
/// reach the page through the typography point rather than being written into a
/// page file (A6, X1, F-6).
pub fn the_controls_on_the_event(mut event: Event, controls: &[AControl]) -> Event {
    for (control, (is, label, act)) in controls.iter().zip(THE_CONTROLS_OWN_KEYS) {
        event = event
            .saying(is, Said::Value(Verbatim::new(the_token_for(control.what_it_is))))
            .saying(label, Said::Prose(control.label.clone()))
            .saying(act, Said::Value(control.act.clone()));
    }
    event.saying(
        "controlCount",
        Said::Value(Verbatim::new(
            controls.len().min(THE_CONTROLS_OWN_KEYS.len()).to_string(),
        )),
    )
}

/// **What a control is, as a token the page routes on** — a value and never
/// prose. The page uses it to pick the class and the act's own handler, and
/// never to decide the order: the order is the one the list arrives in.
fn the_token_for(what_it_is: WhatItIs) -> &'static str {
    match what_it_is {
        WhatItIs::TryAgain => "try-again",
        WhatItIs::TheScreensOwnWayPast => "way-past",
        WhatItIs::TheRawText => "raw-text",
        WhatItIs::ReportAProblem => "report",
        WhatItIs::TheCross => "cross",
    }
}

/// **The sentence, composed here and before the typography point.**
///
/// **This is a plumbing sentence and not a screen's, and the words are owed to
/// Design.** §24 leaves the words the user reads to Design, and the eleven
/// screens compose their own; what this says is what a failure record on its
/// own can say, which is **what the record carries and nothing more**. It names
/// no cause (M6, M13b) and it joins no name into itself (X7).
///
/// **The condition sentence used to end *"and Smoozy is still watching it"* and
/// that was untrue of this application.** M7 says in as many words that
/// *"nothing here adds a watcher"* and P9 that nothing on this side polls: the
/// record stands until the user clears it or until something the user did
/// happens to report a new value. **A user who read that Smoozy was watching
/// would wait, and nothing would ever change, because nothing was ever going to
/// change it.** Corrected 2026-09-12 at round 1 of the code audit of what is
/// drawn inside the window (F-2, F-3).
///
/// **What is left is the shortest standard pair, *Error.* and *Did not
/// finish.*, and that is honest rather than finished** (step 13's words sweep,
/// 2026-10-28; until then *Something is not working.* and *Something did not
/// finish.*). It is as much as a record with no owning module's words behind
/// it carries. **The words a person should read here are Design's
/// and are owed**, the same way the window buttons' names are marked owed at the
/// head of `src/surface/chrome.rs`; what this file fixes is that the sentence
/// says what the record carries and nothing more.
///
/// **One condition has Design's own words and takes them**: `State, identity
/// and recovery` §8.3's supervisor-unavailable condition, whose words
/// `Desktop app and delivery` V1 left to Design and `Design/Servers § A row`
/// gives — *Smoozy isn't responding.* Added 2026-09-27, step 10c's eighth-pass
/// fix piece 6, out of the seventh pass on RunPod, where a server after a
/// restart read *Something is not working.* See [`whose_sentence_it_is`].
fn the_sentence_for(record: &Standing) -> String {
    if crate::servers::published::is_the_supervisors_condition(record) {
        return crate::surface::servers::words::SMOOZY_ISNT_RESPONDING.to_string();
    }
    match record.kind {
        Kind::Condition => "Error.".to_string(),
        Kind::Ending => "Did not finish.".to_string(),
    }
}

/// **Whose sentence [`the_sentence_for`] composed** — Design's own, where the
/// condition has words of Design's, and the plumbing's stand-in everywhere
/// else. **M32: the capture belongs to whatever composes the sentence the user
/// reads**, so only Design's own words spend it.
fn whose_sentence_it_is<'a>(record: &Standing, sentence: &'a str) -> WhoseSentenceItIs<'a> {
    if crate::servers::published::is_the_supervisors_condition(record) {
        WhoseSentenceItIs::TheComposersOwn(sentence)
    } else {
        WhoseSentenceItIs::AStandInThePlumbingComposed(sentence)
    }
}

/// **A read of the failure records that did not answer, put in the shape a
/// failure is drawn in** — L8a.
///
/// **It is a presentation and not a record, and three things follow.** Nothing
/// is written to the store, which is the thing that just failed. It is an
/// **ending** — the read either answered or it did not, and that has already
/// happened — so the shape gives it no cross, which is right: there is nothing
/// stored to clear. And it holds over no value, so nothing withdraws it except
/// the next ask, when a read that answers replaces the drawing on the same key.
///
/// **The diary is handed nothing for it, and that is M32 rather than an
/// oversight**: *"a path that composes a stand-in sentence captures nothing"*,
/// and the mark that would stop a second line is a column of a row this read
/// could not reach. So a failed read is drawn every time it is asked and
/// written to the diary never. **That half of the silence is still open** and
/// is recorded in `Audits/Build/Step 10a/The failure nobody could see — what it
/// was.md`.
fn the_read_did_not_answer(subject: &DurableSubject) -> Standing {
    Standing {
        kind: Kind::Ending,
        subject: subject.clone(),
        identity_or_mark: THE_READ_DID_NOT_ANSWER.to_string(),
        the_values_it_was_raised_over: Vec::new(),
        the_diary_holds_the_sentence: false,
    }
}

/// The shape for one record. **Nothing here adds a part the shape does not
/// have**, and every drawing after this one fills the same struct.
///
/// **Answers nothing where the record cannot be keyed**, which is
/// [`the_key_for`]'s refusal carried out rather than papered over.
fn a_shape(record: &Standing, sentence: &str) -> Option<TheFailureShape> {
    Some(the_shape_on(the_key_for(record)?, sentence))
}

/// **What a read that did not answer is keyed about**: the subject's sort and
/// its written form, so a failed read about a server row and one about a
/// history record of the same rendering are two drawings.
fn the_read_did_not_answer_about(subject: &DurableSubject) -> String {
    format!(
        "{}:{}",
        subject.kind_as_text().replace(' ', "-"),
        subject.as_text().replace(' ', "-")
    )
}

/// The shape for one record on a key already composed.
fn the_shape_on(key: TheKeyAFailureIsDrawnOn, sentence: &str) -> TheFailureShape {
    TheFailureShape {
        key,
        what_did_not_happen: sentence.to_string(),
        // **Only where Smoozy actually knows** — Design. The plumbing on its own
        // never knows, so it says nothing; the carve-out that does is §16.4's
        // and is piece D's.
        what_to_do_about_it: None,
        the_act_that_would_try_again: None,
        the_screens_own_way_past: None,
        the_raw_text: Vec::new(),
        // **Nothing beside it.** Until 2026-09-27 the durable subject's raw
        // identity stood here as a value, and the pane at the foot of the
        // window printed it — *the subject c0ac6314…* on the seventh pass on
        // RunPod — which is no word a person reads (Rule 9) and says nothing
        // the sentence does not. **The subject still travels on the event** for
        // the report's handover (M33), on its own keys, and is drawn nowhere.
        // Step 10c's eighth-pass fix piece 6.
        the_values_beside_it: Vec::new(),
    }
}

/// **Q1's key.** A condition is keyed on its condition identity; an ending on
/// the subject it is about.
///
/// **It refuses rather than invents, and both refusals were fabrications until
/// 2026-09-12** — round 1 of the code audit of what is drawn inside the window
/// (F-11).
///
/// - **A condition whose stored identity will not read back answers nothing.**
///   It used to mint a **brand new** condition identity and use it as the key,
///   so the same standing record got a different key at every read — and that
///   key is what *said once* counts on, what the operating system replaces and
///   takes a post down on, and what the page is handed. Nothing reported any of
///   it; the old post simply stayed on the user's screen. **Answering nothing
///   and skipping the record is the same refusal
///   [`a_subject`] already makes of a request's own values.**
/// - **An ending about a subject with no identity of its own is keyed on the
///   member that exists for it.** It used to mint a **job** identity, so a
///   licence failure was keyed as a job’s ending despite having no identity. [`crate::record::failure_records::DurableSubject::identity`]
///   answers `None` there for that reason and says so in its own note.
///
/// **Nothing in this folder issues an identity**, which is the rule the
/// presentation folder's own sweep already holds in its neighbouring file.
fn the_key_for(record: &Standing) -> Option<TheKeyAFailureIsDrawnOn> {
    the_key_a_record_is_drawn_on(record.kind, &record.subject, &record.identity_or_mark)
}

/// **The one composer of the key a drawing is filed under, and there is exactly
/// one.**
///
/// It takes the three parts of the record the key is made of — the kind, the
/// subject and the mark — and nothing else, so **every path that needs the key
/// composes it the same way from the same facts.** [`the_key_for`] hands it a
/// standing record; [`the_user_cleared_one`] hands it a press's own three
/// values.
///
/// **There were two composers until the close of round 1 and they disagreed.**
/// The clearing's own read `identity_or_mark` back as a condition identity and
/// nothing else, so a recorded ending composed no key and the event carried an
/// empty string. The close check named the wider shape — *two functions
/// composing the same key from different inputs with nothing holding them
/// together* — and this is that shape closed rather than the one site patched
/// (C-1).
fn the_key_a_record_is_drawn_on(
    kind: Kind,
    subject: &DurableSubject,
    identity_or_mark: &str,
) -> Option<TheKeyAFailureIsDrawnOn> {
    match kind {
        Kind::Condition => Identity::read_back(IdentityKind::Condition, identity_or_mark)
            .map(ThePresentationKey::ACondition)
            .map(TheKeyAFailureIsDrawnOn::OnAnIdentity),
        // **An ending is keyed by the sort of subject it is recorded against,
        // and only the three M4 and `State, identity and recovery` F25 name.**
        // Until 2026-09-19 any subject with an identity was wrapped as a job's
        // ending, so an outcome record's ending was written `job:<identity>`
        // over an identity that is no job's. **A server row, a key row or a
        // transfer arrangement has no ending named against it, so none is
        // invented**: both callers already read `None` as *never drawn*. The
        // Gemini catch-up of step 10a, Area 4 (F-2), on F-11's own rule.
        Kind::Ending => match subject {
            DurableSubject::JobRecord(identity) | DurableSubject::HistoryRecord(identity) => {
                Some(TheKeyAFailureIsDrawnOn::OnAnIdentity(
                    ThePresentationKey::AJobsEnding(identity.clone()),
                ))
            }
            DurableSubject::OutcomeRecord(identity) => {
                Some(TheKeyAFailureIsDrawnOn::AnOutcomesEnding(identity.clone()))
            }
            DurableSubject::LicenceRecord => Some(TheKeyAFailureIsDrawnOn::TheLicenceRecordsEnding),
            DurableSubject::ServerRow(_)
            | DurableSubject::KeyRow(_)
            | DurableSubject::TransferArrangement(_) => None,
        },
    }
}

/// **H34: the request's values are validated before an adapter is touched.**
///
/// A subject kind that is none of §8.7's closed list, or an identity that will
/// not read back, is not a subject — and is answered with nothing rather than
/// with a guess.
pub fn a_subject(subject_kind: &str, subject: &str) -> Option<DurableSubject> {
    let read = |kind: IdentityKind| Identity::read_back(kind, subject);
    match subject_kind {
        "server row" => read(IdentityKind::LocalServer).map(DurableSubject::ServerRow),
        "key row" => read(IdentityKind::LocalKey).map(DurableSubject::KeyRow),
        "transfer arrangement" => {
            read(IdentityKind::Transfer).map(DurableSubject::TransferArrangement)
        }
        "job record" => read(IdentityKind::Job).map(DurableSubject::JobRecord),
        "history record" => read(IdentityKind::Job).map(DurableSubject::HistoryRecord),
        "outcome record" => read(IdentityKind::Operation).map(DurableSubject::OutcomeRecord),
        "licence record" => Some(DurableSubject::LicenceRecord),
        _ => None,
    }
}

/// An act, for a drawing that has one to offer. Here so the type is reachable
/// from a drawing's own file without reaching past the shape.
pub type TheActOffered = AnAct;

/// One drawn failure, for a caller that wants the list rather than the event.
pub type OneDrawnFailure = Drawn;

#[cfg(test)]
mod tests {
    use super::*;

    fn a_condition_record() -> Standing {
        Standing {
            kind: Kind::Condition,
            subject: DurableSubject::ServerRow(
                Identity::issue(IdentityKind::LocalServer).expect("the desk has randomness"),
            ),
            identity_or_mark: Identity::issue(IdentityKind::Condition)
                .expect("the desk has randomness")
                .as_text(),
            the_values_it_was_raised_over: vec!["disconnected".to_string()],
            the_diary_holds_the_sentence: false,
        }
    }

    /// **M7 and P9: the sentence says what the record carries and nothing
    /// more.**
    ///
    /// *"Nothing here adds a watcher to close it"*, and nothing on this side
    /// polls — so a sentence saying Smoozy is still watching is one the
    /// application cannot keep. A user would wait for a change nothing was ever
    /// going to make.
    #[test]
    fn the_plumbings_sentence_claims_no_watcher_and_no_cause() {
        for record in [a_condition_record()] {
            let sentence = the_sentence_for(&record);
            let said = sentence.to_lowercase();
            for claiming in ["watch", "still", "poll", "check", "monitor", "because"] {
                assert!(
                    !said.contains(claiming),
                    "the plumbing's sentence says {claiming}: {sentence:?}. **M7: \
                     nothing here adds a watcher to close it**, P9: nothing on \
                     this side polls, and M6 and M13b: it names no cause"
                );
            }
        }
    }

    /// **F-11: a condition whose stored identity will not read back is skipped,
    /// and nothing is minted to key it with.**
    ///
    /// A freshly issued identity would give the same standing record a different
    /// key at every read — and the key is what *said once* counts on, what the
    /// operating system replaces and takes a post down on, and what the page is
    /// handed.
    #[test]
    fn a_condition_whose_identity_will_not_read_back_is_refused_rather_than_reissued() {
        let mut record = a_condition_record();
        record.identity_or_mark = "not an identity".to_string();
        assert!(the_key_for(&record).is_none());
        assert!(a_shape(&record, "Error.").is_none());
    }

    /// **F-11: the licence record's ending is keyed on its own member and never
    /// on a minted job identity** — retained legacy subject handling.
    #[test]
    fn a_licence_records_ending_is_not_keyed_as_a_jobs() {
        let record = Standing {
            kind: Kind::Ending,
            subject: DurableSubject::LicenceRecord,
            identity_or_mark: "the licence could not be checked".to_string(),
            the_values_it_was_raised_over: Vec::new(),
            the_diary_holds_the_sentence: false,
        };
        assert_eq!(
            the_key_for(&record),
            Some(TheKeyAFailureIsDrawnOn::TheLicenceRecordsEnding)
        );
        // And it is stable: the same record read twice gives the same key,
        // which is what a minted identity could never be.
        assert_eq!(the_key_for(&record), the_key_for(&record));
    }

    /// **C-1: the clearing composes the very key the drawing is filed under,
    /// for every kind of record — and a recorded ending is the one that was
    /// broken.**
    ///
    /// The page keys each drawing on the key the standing event carried (F-5)
    /// and removes the one the clearing event names. **Until the close of round
    /// 1 the clearing composed its key a second way** — reading the mark back as
    /// a condition identity and nothing else — so for an ending, whose mark is
    /// the owning module's own words, it composed nothing and the call site
    /// wrote an empty string. The record was cleared in the store and **the
    /// drawing stayed on the screen for the life of the page**, and the *did not
    /// land* line could not reach an ending either.
    ///
    /// **There is one composer now**, so this test is what holds the press and
    /// the drawing together rather than an assertion about one of them.
    #[test]
    fn the_clearing_composes_the_key_the_drawing_is_filed_under() {
        let a_jobs_ending = Standing {
            kind: Kind::Ending,
            subject: DurableSubject::JobRecord(
                Identity::issue(IdentityKind::Job).expect("the desk has randomness"),
            ),
            // **The owning module's own words**, which is what an ending's mark
            // is and what no identity will ever read back out of.
            identity_or_mark: "the render did not finish".to_string(),
            the_values_it_was_raised_over: Vec::new(),
            the_diary_holds_the_sentence: false,
        };
        let a_licence_ending = Standing {
            kind: Kind::Ending,
            subject: DurableSubject::LicenceRecord,
            identity_or_mark: "the licence could not be checked".to_string(),
            the_values_it_was_raised_over: Vec::new(),
            the_diary_holds_the_sentence: false,
        };
        for record in [a_condition_record(), a_jobs_ending, a_licence_ending] {
            let drawn = the_key_for(&record).expect("the drawing is filed under a key");
            let pressed = the_key_a_record_is_drawn_on(
                record.kind,
                &record.subject,
                &record.identity_or_mark,
            )
            .expect(
                "the press composes a key. **An ending's mark is the owning \
                 module's words and never an identity**, so a composer that \
                 reads it back as one answers nothing and the page is handed \
                 an empty key it can match no drawing against",
            );
            assert_eq!(
                drawn.as_text(),
                pressed.as_text(),
                "the drawing and the press name two different keys, so the \
                 cross clears the record and the drawing stays on the screen"
            );
            assert!(
                !pressed.as_text().is_empty(),
                "the press names an empty key: {:?}",
                record.kind
            );
        }
    }

    /// **F-2 of the Gemini catch-up, Area 4: an ending is keyed by the sort of
    /// subject it is recorded against**, and only the three M4 names get one.
    ///
    /// Before 2026-09-19 every subject with an identity was keyed as a job's
    /// ending: the outcome record's check below failed on `job:`, and the three
    /// refusals failed on a key where there should be none.
    #[test]
    fn an_ending_is_keyed_by_its_subjects_sort_and_refused_where_none_is_named() {
        let key = |subject: DurableSubject| {
            the_key_a_record_is_drawn_on(Kind::Ending, &subject, "the owning module's words")
        };
        let job = Identity::issue(IdentityKind::Job).expect("the desk has randomness");
        for subject in [
            DurableSubject::JobRecord(job.clone()),
            DurableSubject::HistoryRecord(job.clone()),
        ] {
            assert_eq!(
                key(subject),
                Some(TheKeyAFailureIsDrawnOn::OnAnIdentity(
                    ThePresentationKey::AJobsEnding(job.clone())
                ))
            );
        }

        let operation = Identity::issue(IdentityKind::Operation).expect("the desk has randomness");
        let outcomes = key(DurableSubject::OutcomeRecord(operation.clone()))
            .expect("M4 names the outcome record as a place an ending is recorded");
        assert_eq!(outcomes, TheKeyAFailureIsDrawnOn::AnOutcomesEnding(operation.clone()));
        assert!(
            outcomes.as_text().starts_with("outcome:"),
            "an outcome record's ending is written as a job's: {}",
            outcomes.as_text()
        );

        for (subject, named) in [
            (
                DurableSubject::ServerRow(
                    Identity::issue(IdentityKind::LocalServer).expect("the desk has randomness"),
                ),
                "a server row",
            ),
            (
                DurableSubject::KeyRow(
                    Identity::issue(IdentityKind::LocalKey).expect("the desk has randomness"),
                ),
                "a key row",
            ),
            (
                DurableSubject::TransferArrangement(
                    Identity::issue(IdentityKind::Transfer).expect("the desk has randomness"),
                ),
                "a transfer arrangement",
            ),
        ] {
            assert!(
                key(subject).is_none(),
                "an ending against {named} was given a key. M4 and \
                 `State, identity and recovery` F25 name no ending against it, so \
                 none is invented"
            );
        }
    }

    /// **F-5 of the Gemini catch-up, Area 4: a request to clear an ending
    /// leaves the record standing and posts nothing.**
    ///
    /// `State, identity and recovery` F25 gives the user's clearing to a
    /// condition record alone. Before 2026-09-19 this request cleared the
    /// record and posted *cleared*, so the record was gone and the check on
    /// what stands failed.
    #[test]
    fn a_request_to_clear_an_ending_leaves_it_standing_and_says_nothing() {
        use crate::events::{Delivers, Delivery};
        use crate::record::LocalRecord;
        use std::sync::{Arc, Mutex as StdMutex};

        struct WhatThePageGot(Arc<StdMutex<Vec<Delivery>>>);
        impl Delivers for WhatThePageGot {
            fn deliver(&self, delivery: &Delivery) {
                self.0.lock().unwrap().push(delivery.clone());
            }
        }

        let mut path = std::env::temp_dir();
        path.push(format!(
            "smoozy-no-ending-clear-{}-{}.sqlite3",
            std::process::id(),
            line!()
        ));
        let _ = std::fs::remove_file(&path);
        let record = LocalRecord::open_at(&path).expect("the desk can make a record");
        let history = DurableSubject::HistoryRecord(
            Identity::issue(IdentityKind::Job).expect("the desk has randomness"),
        );
        FailureRecords::over(record.connection())
            .an_ending_was_recorded(&history, "the render did not finish")
            .expect("the ending is recorded");

        let queue = TheOneQueue::new();
        let got = Arc::new(StdMutex::new(Vec::new()));
        queue.the_page_is_ready(Box::new(WhatThePageGot(got.clone())));
        if let Some(identity) = history.identity() {
            queue.the_subject_exists(identity.clone());
        }

        let started = a_started_holding(record);
        the_user_cleared_one(
            &started,
            &queue,
            Kind::Ending,
            &history,
            "the render did not finish",
        );

        let standing = FailureRecords::over(
            started.record.as_ref().expect("the record is held").connection(),
        )
        .standing_against(&history)
        .expect("the record reads");
        assert_eq!(
            standing.len(),
            1,
            "a request to clear an ending cleared it. An ending is not the \
             user's to clear (`State, identity and recovery` F25, M5)"
        );
        assert!(
            got.lock().unwrap().is_empty(),
            "a refused request to clear an ending posted something"
        );

        drop(started);
        let _ = std::fs::remove_file(&path);
    }

    /// **C-1, the page's half: the press hands the kind back.**
    ///
    /// **Nothing in this project reads the page half for behaviour** (F-7's own
    /// reason for existing), so this reads it as text — which is what the
    /// neighbouring piece's own page checks do. Without the kind the application
    /// cannot compose the key, nor refuse a request to clear an ending (F-5 of
    /// the Gemini catch-up, Area 4).
    #[test]
    fn the_press_hands_the_record_kind_back_to_the_application() {
        let page = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("web")
                .join("the-failure-shape.js"),
        )
        .expect("the page module is readable");
        let asking = page
            .split_once("a_failure_was_cleared")
            .expect("the page asks for a clearing")
            .1;
        let asking = &asking[..asking.find("});").expect("the ask is closed")];
        for named in ["kind:", "subjectKind:", "subject:", "identityOrMark:"] {
            assert!(
                asking.contains(named),
                "web/the-failure-shape.js does not hand {named} back with the \
                 press. **The application composes the key the answering event \
                 carries out of the kind, the subject and the mark**, and a \
                 press missing one of them names a drawing that is not there"
            );
        }
    }

    /// **C-1, the other half: the kind arrives with the press and is validated
    /// rather than guessed at.**
    ///
    /// `State, identity and recovery` F2: *every failure record carries its kind
    /// and the presentation reads it rather than guessing.* A word that is
    /// neither of the two is not a kind, and H34 refuses it here the way
    /// [`a_subject`] refuses a subject kind that is none of §8.7's list.
    #[test]
    fn a_press_carrying_no_readable_kind_is_refused() {
        assert_eq!(Kind::from_text("condition"), Some(Kind::Condition));
        assert_eq!(Kind::from_text("ending"), Some(Kind::Ending));
        for neither in ["", "Condition", "failure", "endings"] {
            assert!(
                Kind::from_text(neither).is_none(),
                "{neither:?} was read as a record's kind"
            );
        }
    }

    /// **F-13: the controls travel on the event, in the shape's fixed order.**
    ///
    /// The application composes the order and the page draws the list it is
    /// handed. **The two shape-owned labels reach the page as prose** through
    /// the typography point, which is what stops the page writing them itself
    /// (A6, X1).
    #[test]
    fn the_control_list_travels_on_the_event_in_the_shapes_order() {
        let record = a_condition_record();
        let shape = TheFailureShape {
            key: TheKeyAFailureIsDrawnOn::OnAnIdentity(ThePresentationKey::ACondition(
                Identity::read_back(IdentityKind::Condition, &record.identity_or_mark)
                    .expect("the record's own identity"),
            )),
            what_did_not_happen: "Error.".to_string(),
            what_to_do_about_it: None,
            the_act_that_would_try_again: Some(AnAct {
                label: "Retry".to_string(),
                act: "servers.connect".to_string(),
            }),
            the_screens_own_way_past: None,
            the_raw_text: Vec::new(),
            the_values_beside_it: Vec::new(),
        };
        let drawn = shape
            .presented(&record, None, Room::Expanded)
            .expect("it stands");
        let event = the_event_for(Subject::TheRun, &drawn, "Error.");
        let said = format!("{event:?}");

        assert!(said.contains("controlCount"));
        // Design's order, written on to the event: the retry, then the report
        // control, then the cross.
        for (name, token) in [
            ("control0Is", "try-again"),
            ("control1Is", "report"),
            ("control2Is", "cross"),
        ] {
            assert!(said.contains(name), "{name} is not on the event: {said}");
            assert!(
                said.contains(token),
                "{token} is not on the event: {said}. **Design: a failure has one \
                 shape wherever it attaches**, and the order is the \
                 application's"
            );
        }
        // **The shape's own labels are prose and reach the page through the
        // point**, which is what stops a page file writing them itself.
        assert!(said.contains(&format!(
            "{:?}",
            crate::typography::Prose::new(
                crate::presentation::the_failure_shape::REPORT_A_PROBLEM
            )
        )));
        assert!(said.contains(&format!(
            "{:?}",
            crate::typography::Prose::new(crate::presentation::the_failure_shape::CLEAR_THIS)
        )));
    }

    /// **H34: a subject kind that is none of §8.7's closed list is not a
    /// subject.**
    #[test]
    fn a_subject_kind_the_list_does_not_name_is_refused() {
        let identity = Identity::issue(IdentityKind::LocalServer)
            .expect("the desk has randomness")
            .as_text();
        assert!(a_subject("the whole machine", &identity).is_none());
        assert!(a_subject("", &identity).is_none());
    }

    /// **An identity that will not read back is not an identity.** The same
    /// refusal `crate::record::key_rows` makes, and it is what stops a page
    /// value reaching the record as a subject.
    #[test]
    fn an_identity_that_will_not_read_back_is_refused() {
        assert!(a_subject("server row", "not an identity").is_none());
        assert!(a_subject("server row", "").is_none());
    }

    /// The historical licence subject has no identity of its own.
    #[test]
    fn the_licence_record_is_a_subject_without_an_identity() {
        let subject = a_subject("licence record", "").expect("it is on the list");
        assert_eq!(subject, DurableSubject::LicenceRecord);
        assert!(subject.identity().is_none());
    }

    /// Every one of §8.7's seven durable subjects is reachable from the page,
    /// and nothing else is.
    #[test]
    fn every_one_of_the_closed_list_is_reachable_and_nothing_else_is() {
        for (kind, identity_kind) in [
            ("server row", IdentityKind::LocalServer),
            ("key row", IdentityKind::LocalKey),
            ("transfer arrangement", IdentityKind::Transfer),
            ("job record", IdentityKind::Job),
            ("history record", IdentityKind::Job),
            ("outcome record", IdentityKind::Operation),
        ] {
            let identity = Identity::issue(identity_kind)
                .expect("the desk has randomness")
                .as_text();
            assert!(
                a_subject(kind, &identity).is_some(),
                "{kind} is on `State, identity and recovery` §8.7's closed list \
                 and the page cannot name it"
            );
        }
        assert!(a_subject("licence record", "").is_some());
    }
    /// **L8a, proved over the fault it was written out of: a read of the
    /// failure records that will not answer puts a failure on the screen rather
    /// than nothing at all.**
    ///
    /// The record is put back to the shape the Creator's own record was
    /// carrying — `failure_record_raised_over` with three columns and a
    /// condition record standing against a server — so `standing_against`
    /// answers *no such column*, which is exactly what it was answering on that
    /// computer. **Before 2026-09-13 this posted nothing and wrote nothing**,
    /// and the page drew an application with nothing wrong with it.
    #[test]
    fn a_read_that_did_not_answer_puts_a_failure_on_the_screen() {
        use crate::events::{Delivers, Delivery};
        use crate::record::LocalRecord;
        use std::sync::{Arc, Mutex as StdMutex};

        struct WhatThePageGot(Arc<StdMutex<Vec<Delivery>>>);
        impl Delivers for WhatThePageGot {
            fn deliver(&self, delivery: &Delivery) {
                self.0.lock().unwrap().push(delivery.clone());
            }
        }

        let mut path = std::env::temp_dir();
        path.push(format!(
            "smoozy-l8a-{}-{}.sqlite3",
            std::process::id(),
            line!()
        ));
        let _ = std::fs::remove_file(&path);
        let record = LocalRecord::open_at(&path).expect("the desk can make a record");
        let server = Identity::issue(IdentityKind::LocalServer).expect("the desk has randomness");
        let condition = Identity::issue(IdentityKind::Condition).expect("the desk has randomness");
        record
            .connection()
            .execute(
                "INSERT INTO failure_record
                    (kind, durable_subject, durable_subject_kind, identity_or_mark,
                     condition_key, the_diary_holds_the_sentence)
                 VALUES ('condition', ?1, 'server row', ?2, 'a-key', 0)",
                rusqlite::params![server.as_text(), condition.as_text()],
            )
            .expect("the row goes in");
        record
            .connection()
            .execute_batch(
                "DROP TABLE failure_record_raised_over;
                 CREATE TABLE failure_record_raised_over (
                     durable_subject TEXT NOT NULL,
                     identity_or_mark TEXT NOT NULL,
                     value TEXT NOT NULL,
                     PRIMARY KEY (durable_subject, identity_or_mark, value))",
            )
            .expect("the old shape goes back");

        let subject = DurableSubject::ServerRow(server.clone());
        assert!(
            FailureRecords::over(record.connection())
                .standing_against(&subject)
                .is_err(),
            "the read answered, so this test is not standing on the fault"
        );

        let queue = TheOneQueue::new();
        let got = Arc::new(StdMutex::new(Vec::new()));
        queue.the_page_is_ready(Box::new(WhatThePageGot(got.clone())));

        let mut started = a_started_holding(record);
        the_failures_are_posted(&mut started, &queue, &subject);

        let delivered = got.lock().unwrap().clone();
        assert_eq!(
            delivered.len(),
            1,
            "a read that did not answer said nothing to the page"
        );
        assert_eq!(delivered[0].kind, A_FAILURE_IS_STANDING);
        assert_eq!(delivered[0].subject, server.as_text());
        let Some(Said::Prose(sentence)) = delivered[0].said.get("sentence") else {
            panic!("the failure carried no sentence");
        };
        assert!(
            sentence.as_str().to_lowercase().contains("could not read"),
            "the sentence does not say the read did not answer: {:?}",
            sentence.as_str()
        );
        // **Keyed as a read that did not answer about this server row, and
        // never as a job's ending** (the Gemini catch-up of step 10a, Area 4,
        // F-2): before 2026-09-19 this key read `job:<the server's identity>`.
        assert_eq!(
            delivered[0].said.get("key"),
            Some(&Said::Value(Verbatim::new(format!(
                "unread:server-row:{}",
                server.as_text()
            ))))
        );
        // **An ending, so the shape gives it no cross** — there is nothing
        // stored to clear, and offering a control that could only fail is
        // F-10's silence by another route.
        assert_eq!(
            delivered[0].said.get("kind"),
            Some(&Said::Value(Verbatim::new("ending")))
        );
        let controls: Vec<String> = (0..8)
            .filter_map(|at| delivered[0].said.get(THE_CONTROLS_OWN_KEYS[at].0))
            .map(|said| format!("{said:?}"))
            .collect();
        assert!(
            !controls.iter().any(|control| control.contains("cross")),
            "a read that did not answer offered a cross: {controls:?}"
        );

        drop(started);
        let _ = std::fs::remove_file(&path);
    }

    /// **`State, identity and recovery` §8.3's supervisor-unavailable condition
    /// is drawn in Design's words, *Smoozy isn't responding.*, and nothing
    /// beside the sentence prints the server's raw identity** — the seventh
    /// pass on RunPod read *Something is not working.* on the row and *the
    /// subject c0ac6314…* at the foot of the window. **The connection
    /// condition keeps the plumbing's own sentence**, having no words of
    /// Design's. Step 10c's eighth-pass fix piece 6.
    #[test]
    fn smoozy_not_answering_is_said_in_designs_words_and_no_raw_identity_is_drawn() {
        use crate::events::{Delivers, Delivery};
        use crate::record::failure_records::ConditionKey;
        use crate::record::LocalRecord;
        use crate::typography::Prose;
        use std::sync::{Arc, Mutex as StdMutex};

        struct WhatThePageGot(Arc<StdMutex<Vec<Delivery>>>);
        impl Delivers for WhatThePageGot {
            fn deliver(&self, delivery: &Delivery) {
                self.0.lock().unwrap().push(delivery.clone());
            }
        }

        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-v1-words-{}-{}.sqlite3", std::process::id(), line!()));
        let _ = std::fs::remove_file(&path);
        let record = LocalRecord::open_at(&path).expect("the desk can make a record");
        let server = Identity::issue(IdentityKind::LocalServer).expect("the desk has randomness");
        let subject = DurableSubject::ServerRow(server.clone());
        {
            let mut records = FailureRecords::over(record.connection());
            records
                .a_condition_was_raised(
                    &ConditionKey::over(
                        &subject,
                        crate::servers::published::THE_SUPERVISORS_KIND,
                        None,
                    ),
                    &subject,
                    &["the supervisor not answering"],
                    true,
                )
                .expect("the supervisor's condition is raised");
            records
                .a_condition_was_raised(
                    &ConditionKey::over(&subject, crate::servers::published::THE_KIND, None),
                    &subject,
                    &["disconnected", "reconnecting"],
                    true,
                )
                .expect("the connection condition is raised");
        }

        let queue = TheOneQueue::new();
        let got = Arc::new(StdMutex::new(Vec::new()));
        queue.the_page_is_ready(Box::new(WhatThePageGot(got.clone())));
        let mut started = a_started_holding(record);
        the_failures_are_posted(&mut started, &queue, &subject);

        let delivered = got.lock().unwrap().clone();
        assert_eq!(delivered.len(), 2, "one drawing per standing condition");
        let sentences: Vec<Prose> = delivered
            .iter()
            .filter_map(|one| match one.said.get("sentence") {
                Some(Said::Prose(sentence)) => Some(sentence.clone()),
                _ => None,
            })
            .collect();
        assert_eq!(
            sentences,
            vec![
                Prose::new(crate::surface::servers::words::SMOOZY_ISNT_RESPONDING),
                Prose::new("Error."),
            ]
        );
        // **Only the subject's own key carries the identity**, for the
        // report's handover; no value beside the sentence does, so the pane at
        // the foot of the window has nothing raw to print.
        for one in &delivered {
            for (name, said) in &one.said {
                if *name == "subject" {
                    continue;
                }
                let text = match said {
                    Said::Prose(prose) => prose.as_str().to_string(),
                    Said::Value(value) => value.as_str().to_string(),
                };
                assert!(
                    !text.contains(&server.as_text()),
                    "{name} carries the server's raw identity: {text}"
                );
            }
        }

        drop(started);
        let _ = std::fs::remove_file(&path);
    }

    /// A start report holding nothing but the record, which is all this file's
    /// own route reads. **The application has no other constructor for one**,
    /// so it is written here rather than reached for.
    fn a_started_holding(record: crate::record::LocalRecord) -> Started {
        Started {
            the_sweep_left_behind: Vec::new(),
            record: Some(record),
            record_trouble: None,
            diary: None,
            declarations: smoozy_reporting::lines::Declarations::new(),
            what_modules_declare: crate::presentation::declarations::TheDeclarations::default(),
            said_once: crate::presentation::said_once::TheThingsSaidOnce::new(),
            run: None,
            the_previous_run: smoozy_state::run_mark::WhatTheLaunchFound::NoPreviousRun,

        }
    }
}
