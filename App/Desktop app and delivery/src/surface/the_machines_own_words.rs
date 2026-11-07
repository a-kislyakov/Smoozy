//! **The machine's own words, and the disk-full carve-out, as a service** —
//! §16.3 and §16.4, and the page half is `web/the-machines-own-words.js`.
//!
//! **This file is the surface and not the shape.** The panel is
//! `src/presentation/the_machines_own_words.rs`, the carve-out is
//! `src/presentation/the_disk_full_carve_out.rs`, and the rules every drawing
//! inside them keeps are at the head of `src/presentation/mod.rs`. Read that
//! before adding anything here.
//!
//! ## One command, one event kind, and why the answer arrives as an event
//!
//! P6: *results reach the interface as events on one ordered queue, and the
//! interface holds no other route to a module.* **A command is the page asking;
//! an event is the application telling.**
//!
//! ## The two streams never share a key on the event, and that is M13a
//!
//! **Each stream has its own set of keys, named for the stream it came from**,
//! and there is no key either could occupy. So *"the ordinary stream's record
//! never occupies the place the complaints stream's record occupies"* is a
//! property of the transport rather than a rule the composer keeps: there is
//! nowhere to put it. **A merged panel is unavailable for the same reason** —
//! M13, and there is no key for a joined record anywhere below.
//!
//! ## What supplies this today, said plainly
//!
//! **Nothing.** The command reads what the local record holds about a job's
//! words, and the local record holds no job record: `Queue & History`'s own
//! records are that module's, and no step of `Preparation/Build/Plan.md` has put
//! them in this crate. **A subject Smoozy has recorded no words for is a subject
//! with no panel to draw**, and posting one anyway is exactly the fault M10a
//! names — so the command posts nothing for it, which is right rather than
//! hollow.
//!
//! **[`a_renders_words_are_posted`] is the composer** and it is what a caller
//! holding a render's ending calls. It is exercised by this file's tests and by
//! the temporary block the drawing was watched with, and it gains its first real
//! caller when the job record lands here. The same shape piece E's three
//! underway drawings are in, and for the same reason: **no worker of any kind
//! has ever been started on a rented machine**, so no real raw record exists
//! anywhere. That is steps 6, 7 and 8's owed milestones.
//!
//! ## This path composes a stand-in sentence, so it captures nothing
//!
//! **M32, as repaired on 2026-09-12:** *"the capture belongs to whatever
//! composes the sentence the user reads, and a path that composes a stand-in
//! sentence captures nothing."* [`THE_RENDER_DID_NOT_FINISH`] is this section's
//! plumbing speaking because no owning module has supplied words, so **this file
//! calls M32's hook not at all** and the record's one capture stays unspent for
//! the screen that later composes the sentence the user actually reads.
//! `src/surface/the_failure_shape.rs` is the other presentation path and does
//! the same thing through [`crate::presentation::the_two_hooks::WhoseSentenceItIs`];
//! **both call sites are named here because a repair that looked at only one of
//! them would be wrong.**
//!
//! ## Markers of other documents
//!
//! **Markers used in this file that belong to another document are written out
//! with that document at the site, every one of them, and none is listed here.**
//! This plan defines clauses of the same short names as
//! `State, identity and recovery`, `Queue & History` and
//! `Server protocol and transfers`, so a bare one would resolve to a real and
//! wrong clause.

use std::sync::Mutex;

use tauri::State;

use crate::events::{Event, Said, Subject, TheOneQueue};
use crate::presentation::the_disk_full_carve_out::{what_to_do_about_it, TheCarriedReading};
use crate::presentation::the_failure_shape::{Drawn, Room, TheFailureShape};
use crate::presentation::the_machines_own_words::{
    the_control_that_opens_it, ATab, Stream, ThePanel, BLENDER_OUTPUT,
};
use crate::presentation::the_presentation_key::TheKeyAFailureIsDrawnOn;
use crate::record::failure_records::{DurableSubject, Standing};
use crate::start::Started;
use crate::surface::the_failure_shape::a_subject;
use crate::typography::{Prose, Verbatim};

/// **A render's ending, with the machine's own words behind the plain
/// sentence.**
///
/// **One event per ending and never a list in one event**, the same discipline
/// piece C's failure shape keeps.
pub const A_RENDERS_WORDS_ARE_STANDING: &str = "presentation:a-renders-words-are-standing";

/// **The plumbing's own plain sentence for a render that ended badly.**
///
/// **This is a plumbing sentence and not a screen's.** §24 leaves the words the
/// user reads to Design and the eleven screens compose their own; what this says
/// is what a render's ending record on its own can say. **It names no cause** —
/// M13b, and M6 — and it joins no name into itself (X7).
pub const THE_RENDER_DID_NOT_FINISH: &str = "The render did not finish.";

/// **The panel's own words, and they are the drawing's rather than a stream's.**
///
/// **Every one of these was written into `web/the-machines-own-words.js` until
/// 2026-09-12**, where A6 and X1 say prose may not be: *"every string of prose
/// passes through one function before it reaches anything that draws it, and the
/// interface accepts prose from nowhere else"*, with the enforcement
/// **structural on both sides**. It was structural on one. Moved here at round 1
/// of the code audit of what is drawn inside the window (F-6), where
/// `src/surface/mod.rs`'s own page sweep is what now says so.
///
/// **The note is two sentences and travels as two strings** — X10, and §22 item
/// 28's narrowing, which a caller handing each sentence separately does not
/// meet.
pub(crate) const THE_PANELS_NOTE: &str = "Raw output, unchanged.";
/// The control that closes the panel.
pub(crate) const CLOSE: &str = "Close";
/// **The control that copies the open tab's words, exactly as kept** —
/// `Queue & History`, the Creator's third notes, note 13, as Keys has *Copy
/// public key*. Handed to Blender's output in Queue and History only, never to
/// *What the server said*.
pub(crate) const COPY: &str = "Copy";
/// **The two words a reading's moment is drawn between** — M3a's *as of that
/// moment*, on the clock it was taken on. **Two strings on two routes**: the
/// moment and the clock's name are values and travel beside these rather than
/// inside them (X7).
const AS_OF: &str = "as of";
/// The word before the clock's own name.
const ON_THE_CLOCK: &str = "on";
/// **The name the folder reading is drawn under.** The page used to write this
/// one itself.
const THE_FOLDER: &str = "the folder";
/// The word after a free-space figure.
const FREE: &str = "free";
/// The word after how much of a stream was kept.
const KEPT: &str = "kept";
/// The word after how much of it was dropped.
const DROPPED: &str = "dropped";

/// **The page asks what Smoozy has recorded about a subject's words.**
///
/// `subject_kind` is one of `State, identity and recovery` §8.7's closed list,
/// written; `subject` is the identity's own text. **H34: both are validated
/// before anything is touched**, through the one validator piece C built.
#[tauri::command]
pub fn the_machines_own_words_for(
    started: State<'_, Mutex<Started>>,
    queue: State<'_, std::sync::Arc<TheOneQueue>>,
    subject_kind: String,
    subject: String,
) {
    let Some(subject) = a_subject(&subject_kind, &subject) else {
        return;
    };
    let Ok(started) = started.lock() else {
        return;
    };
    the_renders_words_are_posted(&started, &queue, &subject);
}

/// Post one event for each render's ending recorded against the subject.
fn the_renders_words_are_posted(started: &Started, queue: &TheOneQueue, subject: &DurableSubject) {
    if started.record.is_none() {
        return;
    }
    for (record, key, panel, reading) in the_renders_endings_recorded_against(subject) {
        a_renders_words_are_posted(queue, &record, key, &panel, reading.as_ref());
    }
}

/// **Every render's ending this crate's store holds against a subject, which is
/// none.**
///
/// `Queue & History`'s job record is that module's and no step of
/// `Preparation/Build/Plan.md` has put it in this crate; the tables here are
/// piece C's failure records, the outcome record, the server rows, the key rows
/// and the transfer records, and **none of them carries a capture record's
/// collection outcomes.** The fetch of the retained bytes themselves is
/// `Rented-server runtime` §13's and reaches nothing here either.
///
/// **Nothing is posted for a subject Smoozy has recorded no words for**, and
/// that is right rather than a gap papered over. Posting a panel reading *not
/// recorded yet* would be worse than posting
/// nothing: M10a's fifth reading is a real state of a real record, and using it
/// for *Smoozy holds no record at all* would make the one sentence that
/// separates them mean nothing.
fn the_renders_endings_recorded_against(
    subject: &DurableSubject,
) -> Vec<(Standing, TheKeyAFailureIsDrawnOn, ThePanel, Option<TheCarriedReading>)> {
    let _ = subject;
    Vec::new()
}

/// **A render's ending, composed and posted** — the plain sentence first, the
/// machine's own words behind it, and the carve-out's second sentence where the
/// carried reading fires it.
///
/// **M8 is kept by where each part goes.** The sentence is the shape's
/// `what_did_not_happen`; the words are behind the one control the shape puts in
/// its `the_raw_text`, after the sentence; and the page opens the panel only
/// when that control is pressed. **There is no route by which the record is the
/// first thing in front of the user.**
///
/// **M14 is kept by the two being composed apart.** The diagnosis fills
/// `what_to_do_about_it` and the words fill the panel, and neither function can
/// see the other, so *alongside and never instead of* has no route to fail.
pub fn a_renders_words_are_posted(
    queue: &TheOneQueue,
    record: &Standing,
    key: TheKeyAFailureIsDrawnOn,
    panel: &ThePanel,
    the_reading: Option<&TheCarriedReading>,
) {
    // **P8, and it is told rather than assumed**: a render's ending record is
    // handed in already read back, exactly as piece C's are. **When the module
    // that keeps a job record arrives it calls `the_subject_exists` at creation
    // and this stops being a caller rather than becoming a second answer** —
    // telling the queue twice about one subject is one entry in a set.
    let about = match record.subject.identity() {
        Some(identity) => {
            queue.the_subject_exists(identity.clone());
            Subject::Durable(identity.clone())
        }
        None => Subject::TheRun,
    };

    // **The composed sentence, before X7's substitutions.** The shape puts
    // it through the typography point on the way to the screen and
    // `src/presentation/the_two_hooks.rs` captures the plain one for the diary.
    let sentence = THE_RENDER_DID_NOT_FINISH.to_string();
    let shape = TheFailureShape {
        key,
        what_did_not_happen: sentence.clone(),
        // **§16.4 M14: Design's *only where Smoozy actually knows*.** The
        // plumbing never knows; the carve-out does, and only where the carried
        // reading says there was no room.
        what_to_do_about_it: what_to_do_about_it(the_reading),
        // The act that would try again is the entry's own *Render it again*
        // group, which Design says the failure does not repeat.
        the_act_that_would_try_again: None,
        the_screens_own_way_past: None,
        // **One control, and the panel behind it gives the two streams a tab
        // each** — Design. M13a's *under the name of the stream it came from* is
        // kept by the tabs, which take their names from their own streams.
        the_raw_text: vec![the_control_that_opens_it(
            BLENDER_OUTPUT,
            &record.identity_or_mark,
        )],
        the_values_beside_it: Vec::new(),
    };
    let Some(drawn) = shape.presented(record, None, Room::Expanded) else {
        return;
    };
    let drawn_panel = panel.clone();
    let carried = the_reading.cloned();
    let _ = queue.read_then_post::<Drawn, ()>(
        || Ok(drawn.clone()),
        |drawn| {
            the_event_for(
                about.clone(),
                drawn,
                &sentence,
                &drawn_panel,
                carried.as_ref(),
            )
        },
    );
}

/// One render's ending as an event. **Every string is on one of the two routes
/// and the two are the complete set** (A6, X9).
fn the_event_for(
    about: Subject,
    drawn: &Drawn,
    before_the_point: &str,
    panel: &ThePanel,
    the_reading: Option<&TheCarriedReading>,
) -> Event {
    let mut event = Event::about(about, A_RENDERS_WORDS_ARE_STANDING)
        // Prose, through the point.
        .saying("sentence", Said::Prose(drawn.what_did_not_happen.clone()))
        // Values, by the verbatim route.
        .saying("kind", Said::Value(Verbatim::new(drawn.kind)))
        .saying("subject", Said::Value(Verbatim::new(drawn.subject.clone())))
        .saying("subjectKind", Said::Value(Verbatim::new(drawn.subject_kind)))
        .saying(
            "identityOrMark",
            Said::Value(Verbatim::new(drawn.identity_or_mark.clone())),
        )
        .saying("key", Said::Value(Verbatim::new(drawn.key.clone())))
        // **M33's handover carries the sentence captured before the point**, so
        // it goes by the verbatim route: X7's list includes *any string that is
        // about to be written anywhere*, and this one is written into a report.
        .saying(
            "theSentenceForTheReport",
            Said::Value(Verbatim::new(before_the_point)),
        );
    // **§16.4 M14: the diagnosis, alongside the machine's own words.**
    if let Some(what_to_do) = &drawn.what_to_do_about_it {
        event = event.saying("whatToDoAboutIt", Said::Prose(what_to_do.clone()));
    }
    // **The one control, and the label names whose words they are.** The label
    // is prose and the token is a value — two strings on two routes.
    for control in &drawn.controls {
        if control.what_it_is == crate::presentation::the_failure_shape::WhatItIs::TheRawText {
            event = event
                .saying("theRawTextLabel", Said::Prose(control.label.clone()))
                .saying("theRawTextAct", Said::Value(control.act.clone()));
        }
    }
    // **The whole control list, in the shape's fixed order** — F-13, and the
    // same composition piece C's own service makes. The page draws the list it
    // is handed rather than composing the order a second time.
    event = crate::surface::the_failure_shape::the_controls_on_the_event(event, &drawn.controls);
    // **The panel's own words, each its own string** (X10, A6, X1).
    event = event
        .saying("thePanelNote", Said::Prose(Prose::new(THE_PANELS_NOTE)))
        .saying("theCloseControl", Said::Prose(Prose::new(CLOSE)))
        .saying("theKeptWord", Said::Prose(Prose::new(KEPT)))
        .saying("theDroppedWord", Said::Prose(Prose::new(DROPPED)));
    for tab in &panel.tabs {
        event = a_tabs_own_keys(event, tab);
    }
    if let Some(reading) = the_reading {
        let presented = reading.presented();
        event = event
            // **M3a: the figure, its moment and the clock it was taken on.** The
            // page draws the clock's name — the server's own — beside the figure
            // and puts the moment on its own line.
            .saying("theFreeSpace", Said::Value(presented.value.clone()))
            .saying(
                "theFreeSpaceTakenAt",
                Said::Value(Verbatim::new(
                    presented
                        .taken_at
                        .map(|at| at.to_string())
                        .unwrap_or_default(),
                )),
            )
            .saying(
                "theFreeSpaceOnTheClock",
                Said::Value(Verbatim::new(presented.on_the_clock.clone())),
            )
            // **M17 and `Server protocol and transfers` F1: the exact folder the
            // reading was taken against**, so nothing widens it into a statement
            // about the machine.
            .saying(
                "theFolderTheReadingWasTakenAgainst",
                Said::Value(reading.the_folder_it_was_taken_against()),
            )
            // **The words the reading is drawn with, each on its own route.**
            // The moment, the clock's name and the figure are values and stand
            // beside these rather than inside them (X7, M3a).
            .saying("theFolderLabel", Said::Prose(Prose::new(THE_FOLDER)))
            .saying("theReadingAsOf", Said::Prose(Prose::new(AS_OF)))
            .saying(
                "theReadingOnTheClock",
                Said::Prose(Prose::new(ON_THE_CLOCK)),
            )
            .saying("theFreeSpaceWord", Said::Prose(Prose::new(FREE)));
    }
    event
}

/// **The keys one stream's record travels on, and no other stream's.**
///
/// **M13a, held by the transport.** Each stream has its own set and there is no
/// key both could occupy, so the ordinary stream's record cannot be put where
/// the complaints stream's record goes — not by a mistake and not by a later
/// edit that looked harmless.
struct TheStreamsOwnKeys {
    tab: &'static str,
    says: &'static str,
    said: &'static str,
    said_next: &'static str,
    record: &'static str,
    kept: &'static str,
    dropped: &'static str,
    why_the_rest_went: &'static str,
}

/// The ordinary stream's keys.
const THE_ORDINARY_STREAMS_KEYS: TheStreamsOwnKeys = TheStreamsOwnKeys {
    tab: "theOutputTab",
    says: "theOutputSays",
    said: "theOutputSaid",
    said_next: "theOutputSaidNext",
    record: "theOutputRecord",
    kept: "theOutputKept",
    dropped: "theOutputDropped",
    why_the_rest_went: "theOutputWhyTheRestWent",
};

/// The complaints stream's keys.
const THE_COMPLAINTS_STREAMS_KEYS: TheStreamsOwnKeys = TheStreamsOwnKeys {
    tab: "theErrorsTab",
    says: "theErrorsSays",
    said: "theErrorsSaid",
    said_next: "theErrorsSaidNext",
    record: "theErrorsRecord",
    kept: "theErrorsKept",
    dropped: "theErrorsDropped",
    why_the_rest_went: "theErrorsWhyTheRestWent",
};

fn the_keys_for(stream: Stream) -> TheStreamsOwnKeys {
    match stream {
        Stream::TheOrdinaryOutput => THE_ORDINARY_STREAMS_KEYS,
        Stream::TheComplaints => THE_COMPLAINTS_STREAMS_KEYS,
    }
}

/// One tab, under its own stream's keys.
///
/// **The sentences go on two keys and are never joined** — X2 and X10: a piece
/// of prose is one string as handed in, and the caller hands each sentence as
/// its own string. **The record goes by the verbatim route, whole** (M9, X7).
fn a_tabs_own_keys(event: Event, tab: &ATab) -> Event {
    a_tabs_own_keys_named(event, tab, &|key| key)
}

/// **The same keys, each made by `named`** — so a screen that carries more
/// than one panel's worth of tabs on one event (History's kept words, one
/// pair per attempt) puts each attempt's under its own names, **and every
/// stream still travels under its own stream's key set** (M13a, held by the
/// transport). The Creator's notes on 0.1.8, piece 6d.
fn a_tabs_own_keys_named(
    event: Event,
    tab: &ATab,
    named: &dyn Fn(&'static str) -> &'static str,
) -> Event {
    let keys = the_keys_for(tab.the_stream_it_came_from);
    let mut event = event
        // The tab's name is prose the user reads.
        .saying(named(keys.tab), Said::Prose(Prose::new(tab.the_name_of_the_stream())))
        // Which of the five readings, as a token the page routes on.
        .saying(named(keys.says), Said::Value(Verbatim::new(tab.as_token())));
    let sentences = tab.the_sentences();
    if let Some(said) = sentences.first() {
        event = event.saying(named(keys.said), Said::Prose(Prose::new(*said)));
    }
    if let Some(said) = sentences.get(1) {
        event = event.saying(named(keys.said_next), Said::Prose(Prose::new(*said)));
    }
    if let Some(record) = tab.the_record() {
        event = event.saying(named(keys.record), Said::Value(record.clone()));
    }
    if let crate::presentation::the_machines_own_words::WhatATabSays::OnlyPartWasRecorded {
        kept,
        dropped,
        why_the_rest_went,
        ..
    } = &tab.says
    {
        if let Some(kept) = kept {
            event = event.saying(named(keys.kept), Said::Value(kept.clone()));
        }
        if let Some(dropped) = dropped {
            event = event.saying(named(keys.dropped), Said::Value(dropped.clone()));
        }
        if let Some(why) = why_the_rest_went {
            event = event.saying(named(keys.why_the_rest_went), Said::Value(why.clone()));
        }
    }
    event
}

/// **The word a tab's attempt is named by**, where a panel holds more than one
/// attempt's streams — `Queue & History` E11: *the entry reaches each
/// attempt's record*. Rule 9's own word.
const ATTEMPT: &str = "Attempt";

/// **The kept words on another screen's event** — History's *Blender output*,
/// `Queue & History` E9: every attempt's two streams in the one panel, each
/// tab one of M10's four or M10a's fifth and never an empty box. **The same
/// panel words and the same stream key sets as [`A_RENDERS_WORDS_ARE_STANDING`]
/// carries**, each attempt's under `words<n>` before the stream's own key, so
/// the page builds the panel with the one builder,
/// `web/the-machines-own-words.js`'s `theWordsPanel`. The Creator's notes on
/// 0.1.8, piece 6d.
///
/// **No failure shape and no plain sentence ride with it**: the entry it is
/// opened from already carries those, and the panel is behind its control
/// (M8).
pub(crate) fn the_kept_words_are_said(
    event: Event,
    job: &str,
    server: &str,
    attempts: &[(u32, ThePanel)],
) -> Event {
    use crate::surface::settings::{a_kept_name, a_name};
    let mut event = event
        .saying("wordsTitle", Said::Prose(Prose::new(BLENDER_OUTPUT)))
        .saying("wordsJob", Said::Value(Verbatim::new(job)))
        .saying("wordsServer", Said::Value(Verbatim::new(server)))
        .saying("wordsNote", Said::Prose(Prose::new(THE_PANELS_NOTE)))
        .saying("wordsClose", Said::Prose(Prose::new(CLOSE)))
        .saying("wordsCopy", Said::Prose(Prose::new(COPY)))
        .saying("wordsKeptWord", Said::Prose(Prose::new(KEPT)))
        .saying("wordsDroppedWord", Said::Prose(Prose::new(DROPPED)))
        .saying("wordsAttemptWord", Said::Prose(Prose::new(ATTEMPT)))
        .saying(
            "wordsAttempts",
            Said::Value(Verbatim::new(attempts.len().to_string())),
        );
    for (at, (attempt, panel)) in attempts.iter().enumerate() {
        event = event.saying(
            a_name("words", at, "Attempt"),
            Said::Value(Verbatim::new(attempt.to_string())),
        );
        let prefix = format!("words{at}");
        for tab in &panel.tabs {
            event = a_tabs_own_keys_named(event, tab, &|key| a_kept_name(format!("{prefix}{key}")));
        }
    }
    event
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::presentation::the_machines_own_words::{the_panel, AStreamsRecord};
    use crate::record::failure_records::Kind;
    use smoozy_queue_history::ending::CollectionOutcome;
    use smoozy_state::identities::{Identity, IdentityKind};
    use smoozy_state::observed::MachineClock;
    use std::path::Path;
    use std::time::{Duration, SystemTime};

    /// The page half, with its comment lines read past.
    ///
    /// **The sweeps below read the page as well as the application**, because
    /// the page is where a line would actually be lifted out of a stream: the
    /// drawing does it there and nowhere else.
    fn the_page_half() -> String {
        let text = std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("web/the-machines-own-words.js"),
        )
        .expect("the page half is readable");
        text.lines()
            .filter(|line| {
                let line = line.trim_start();
                !(line.starts_with("//") || line.starts_with('*') || line.starts_with("/*"))
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn an_ending_record() -> Standing {
        Standing {
            kind: Kind::Ending,
            subject: DurableSubject::HistoryRecord(
                Identity::issue(IdentityKind::Job).expect("the desk has randomness"),
            ),
            identity_or_mark: "the render did not finish".to_string(),
            the_values_it_was_raised_over: Vec::new(),
            the_diary_holds_the_sentence: false,
        }
    }

    fn a_key() -> TheKeyAFailureIsDrawnOn {
        TheKeyAFailureIsDrawnOn::OnAnIdentity(
            crate::presentation::the_presentation_key::ThePresentationKey::AJobsEnding(
                Identity::issue(IdentityKind::Job).expect("the desk has randomness"),
            ),
        )
    }

    fn a_record(stream: Stream, outcome: Option<CollectionOutcome>, text: &str) -> AStreamsRecord {
        AStreamsRecord {
            stream,
            outcome,
            as_read_back: text.to_string(),
            kept: None,
            dropped: None,
            why_the_rest_went: None,
        }
    }

    /// The panel M13a's own case makes: the complaints stream empty, the
    /// ordinary stream collected.
    fn the_quiet_ending() -> ThePanel {
        the_panel(&[
            a_record(
                Stream::TheOrdinaryOutput,
                Some(CollectionOutcome::Collected),
                "Fra:1 Mem:9.84G | Sample 96/2048",
            ),
            a_record(
                Stream::TheComplaints,
                Some(CollectionOutcome::CollectedAndTheStreamWasEmpty),
                "",
            ),
        ])
    }

    fn a_full_folder() -> TheCarriedReading {
        TheCarriedReading::of(
            0,
            SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000),
            MachineClock("RunPod 4090".to_string()),
            "/home/artist/render/kitchen-final/beauty",
        )
    }

    /// A place for deliveries to land, so every test below drives the real
    /// queue rather than a copy of its logic — the shape `src/events.rs`'s own
    /// tests already use.
    struct WhatThePageGot(std::sync::Arc<Mutex<Vec<crate::events::Delivery>>>);

    impl crate::events::Delivers for WhatThePageGot {
        fn deliver(&self, delivery: &crate::events::Delivery) {
            self.0
                .lock()
                .expect("the desk is not poisoned")
                .push(delivery.clone());
        }
    }

    /// Post one render's ending through the real queue and answer the delivery.
    fn the_event_for_a_panel(
        panel: &ThePanel,
        the_reading: Option<&TheCarriedReading>,
    ) -> crate::events::Delivery {
        let queue = TheOneQueue::new();
        let got = std::sync::Arc::new(Mutex::new(Vec::new()));
        queue.the_page_is_ready(Box::new(WhatThePageGot(got.clone())));
        a_renders_words_are_posted(&queue, &an_ending_record(), a_key(), panel, the_reading);
        let held = got.lock().expect("the desk is not poisoned");
        held.last().expect("one event was posted").clone()
    }

    fn the_event(the_reading: Option<&TheCarriedReading>) -> crate::events::Delivery {
        the_event_for_a_panel(&the_quiet_ending(), the_reading)
    }

    /// **M13a, held by the transport: the two streams have two sets of keys and
    /// there is no key either could occupy.**
    ///
    /// The keys are read out of the two constants rather than out of a list kept
    /// here, so a key added to one and copied to the other fails here.
    #[test]
    fn the_two_streams_share_no_key_on_the_event() {
        let ordinary = THE_ORDINARY_STREAMS_KEYS;
        let complaints = THE_COMPLAINTS_STREAMS_KEYS;
        let mine = [
            ordinary.tab,
            ordinary.says,
            ordinary.said,
            ordinary.said_next,
            ordinary.record,
            ordinary.kept,
            ordinary.dropped,
            ordinary.why_the_rest_went,
        ];
        let theirs = [
            complaints.tab,
            complaints.says,
            complaints.said,
            complaints.said_next,
            complaints.record,
            complaints.kept,
            complaints.dropped,
            complaints.why_the_rest_went,
        ];
        for key in mine {
            assert!(
                !theirs.contains(&key),
                "{key} is a key both streams travel on. **M13a: the ordinary \
                 stream's record never occupies the place the complaints \
                 stream's record occupies** — there is nowhere to put it"
            );
        }
    }

    /// **M13 and M13a on the wire: each stream's record under its own name, and
    /// nothing merged.**
    #[test]
    fn each_streams_record_travels_under_its_own_streams_name() {
        let delivery = the_event(None);
        assert_eq!(
            delivery.said.get("theOutputRecord").expect("the record"),
            &Said::Value(Verbatim::new("Fra:1 Mem:9.84G | Sample 96/2048"))
        );
        // **The complaints stream was empty, so it has no record and says so.**
        assert!(delivery.said.get("theErrorsRecord").is_none());
        assert_eq!(
            delivery.said.get("theErrorsSaid").expect("the sentence"),
            &Said::Prose(Prose::new("Blender wrote nothing here."))
        );
    }

    /// **M9: the record travels whole and unedited, by the verbatim route.**
    #[test]
    fn the_record_travels_whole_and_by_the_verbatim_route() {
        let panel = the_panel(&[a_record(
            Stream::TheOrdinaryOutput,
            Some(CollectionOutcome::Collected),
            "one — two\nthree ",
        )]);
        let delivery = the_event_for_a_panel(&panel, None);
        // The trailing space and the em dash are both the point's triggers, and
        // neither is substituted: X7's first entry is the machine's own words.
        assert_eq!(
            delivery.said.get("theOutputRecord").expect("the record"),
            &Said::Value(Verbatim::new("one — two\nthree "))
        );
    }

    /// **M8: the plain sentence is on the event and the record is behind a
    /// control.**
    #[test]
    fn the_sentence_is_first_and_the_record_is_behind_a_control() {
        let delivery = the_event(None);
        assert_eq!(
            delivery.said.get("sentence").expect("the sentence"),
            &Said::Prose(Prose::new(THE_RENDER_DID_NOT_FINISH))
        );
        assert_eq!(
            delivery.said.get("theRawTextLabel").expect("the control"),
            &Said::Prose(Prose::new(BLENDER_OUTPUT))
        );
    }

    /// **§16.4 M14: the diagnosis rides alongside the machine's words, and the
    /// words are still there.**
    #[test]
    fn the_diagnosis_stands_beside_the_words_and_never_instead_of_them() {
        let delivery = the_event(Some(&a_full_folder()));
        assert!(delivery.said.get("whatToDoAboutIt").is_some());
        assert!(delivery.said.get("theRawTextLabel").is_some());
        assert!(delivery.said.get("theOutputRecord").is_some());
        assert!(delivery.said.get("theErrorsSaid").is_some());
    }

    /// **M15a and M3a: the reading arrives carried, with its moment and the
    /// server's own name, and the folder it was taken against.**
    #[test]
    fn the_carried_reading_travels_with_its_moment_its_clock_and_its_folder() {
        let delivery = the_event(Some(&a_full_folder()));
        assert_eq!(
            delivery.said.get("theFreeSpace").expect("the figure"),
            &Said::Value(Verbatim::new("0"))
        );
        assert_eq!(
            delivery.said.get("theFreeSpaceTakenAt").expect("the moment"),
            &Said::Value(Verbatim::new("1700000000"))
        );
        assert_eq!(
            delivery
                .said
                .get("theFreeSpaceOnTheClock")
                .expect("the clock"),
            &Said::Value(Verbatim::new("RunPod 4090"))
        );
        assert_eq!(
            delivery
                .said
                .get("theFolderTheReadingWasTakenAgainst")
                .expect("the folder"),
            &Said::Value(Verbatim::new("/home/artist/render/kitchen-final/beauty"))
        );
    }

    /// **M15a: a recorded outcome carrying no reading is not one carrying a
    /// zero.** No reading means no diagnosis and no figure anywhere on the
    /// event.
    #[test]
    fn no_reading_puts_no_diagnosis_and_no_figure_on_the_event() {
        let delivery = the_event(None);
        assert!(delivery.said.get("whatToDoAboutIt").is_none());
        assert!(delivery.said.get("theFreeSpace").is_none());
        assert!(delivery.said.get("theFreeSpaceTakenAt").is_none());
    }

    /// **M9 and M13b, on the page: nothing there takes a record apart.**
    ///
    /// **This is the sweep that matters most of the two**, because the drawing
    /// lifts a last line on the page and nowhere else — `.log .last` at mockup
    /// lines 5476 and 5514.
    #[test]
    fn nothing_on_the_page_takes_a_record_apart() {
        let page = the_page_half();
        for editing in [
            ".slice(",
            ".substring(",
            ".substr(",
            ".split(",
            ".trim(",
            ".trimEnd(",
            ".replace(",
            ".replaceAll(",
            ".charAt(",
            ".lastIndexOf(",
            ".indexOf(",
            ".pop(",
            ".shift(",
            "lastLine",
            "excerpt",
            "summary",
            "highlight",
            "\"last\"",
            "'last'",
        ] {
            assert!(
                !page.contains(editing),
                "web/the-machines-own-words.js names {editing}. **M9: nothing here \
                 edits either stream**, and **M13b: no line of that stream is \
                 lifted out of it — no excerpt, no last line, no highlight, no \
                 leading summary.** The drawing does exactly that at its lines \
                 5476 and 5514, and the clause wins"
            );
        }
    }

    /// **The disk-full reading says what it measures** — the Gemini catch-up
    /// of step 10a, Area 4 (F-4).
    ///
    /// The application puts *free* on the event as `theFreeSpaceWord`. Before
    /// 2026-09-19 nothing on the page read it, so the reading was a bare
    /// figure. It is drawn as its own node, never joined into the figure (X7).
    #[test]
    fn the_page_draws_the_free_space_word_beside_the_figure() {
        let page = the_page_half();
        assert!(
            page.contains("whatItMeasures: said.theFreeSpaceWord"),
            "web/the-machines-own-words.js does not hand the reading the word \
             *free*, so the disk-full reading is a number with no word saying \
             what it measures"
        );
        assert!(
            page.contains("measures.textContent = words.whatItMeasures;"),
            "the word is not drawn as a node of its own"
        );
        for joined in ["+ words.whatItMeasures", "words.whatItMeasures +", "${words.whatItMeasures}"] {
            assert!(
                !page.contains(joined),
                "the word is joined into the figure's string ({joined}), which X7 \
                 forbids"
            );
        }
    }

    /// **M12, on the page: the word *succeeded* is not there either.**
    ///
    /// Twenty endings report success and six of them end at 0 with an empty
    /// output folder, so no drawing anywhere reads a status as a claim about
    /// what was written.
    #[test]
    fn nothing_on_the_page_says_a_job_succeeded() {
        let page = the_page_half().to_lowercase();
        for claiming in ["succe", "exitstatus", "exit_status", "statuscode"] {
            assert!(
                !page.contains(claiming),
                "web/the-machines-own-words.js names {claiming}. **M12: no \
                 presentation says a job succeeded because its status was 0**"
            );
        }
    }

    /// **M8, on the page: the panel is made hidden and only the control opens
    /// it.**
    ///
    /// *"The plain sentence is first and the raw record is behind it, never the
    /// first thing in front of the user."* A panel drawn open would put the
    /// machine's words in front of the sentence, which is the one ordering M8
    /// forbids.
    #[test]
    fn the_panel_is_made_hidden_and_the_control_is_what_opens_it() {
        let page = the_page_half();
        assert!(
            page.contains("panel.hidden = true"),
            "the panel is not made hidden. **M8: the plain sentence is first and \
             the raw record is behind it, never the first thing in front of the \
             user**"
        );
        assert!(
            page.contains("panel.hidden = false"),
            "nothing on the page opens the panel, so the record is unreachable"
        );
    }
    /// **The words a page test hands the panel**: Blender's two streams, the
    /// first holding a line far longer than the panel and every character a
    /// careless copy would change — a tab, a carriage return, trailing spaces,
    /// an em dash, a line with nothing on it.
    fn the_words_the_harness_hands_over() -> (String, String, String) {
        let output = format!(
            "Fra:1 Mem:9.84G | Sample 96/2048\r\n{}\tend — of the line   \n\nlast",
            "Blender quit while reading /home/artist/render/kitchen-final/".repeat(80)
        );
        let errors = "Error: Not freed memory blocks: 2\n  total unfreed memory 0.000 MB \n".to_string();
        let added = "\nFra:2 Mem:9.91G | Sample 97/2048 — added after opening".to_string();
        (output, errors, added)
    }

    /// **Run the panel itself under Node, with a stand-in clipboard**, and
    /// answer what it printed. `None` where no Node is installed — the shape
    /// `src/surface/keys.rs`'s own harness uses.
    fn the_panel_is_run(harness: &str) -> Option<serde_json::Value> {
        let folder = std::env::temp_dir().join(format!(
            "smoozy-the-machines-own-words-harness-{}-{}",
            std::process::id(),
            harness.len()
        ));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        std::fs::copy(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("web/the-machines-own-words.js"),
            folder.join("the-machines-own-words.js"),
        )
        .expect("the page is copied");
        let (output, errors, added) = the_words_the_harness_hands_over();
        std::fs::write(
            folder.join("words.json"),
            serde_json::json!({ "output": output, "errors": errors, "added": added }).to_string(),
        )
        .expect("the words are written");
        for (name, text) in THE_STAND_INS {
            std::fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        std::fs::write(folder.join("harness.js"), format!("{THE_FAKE_PAGE}\n{harness}"))
            .expect("the harness is written");
        let ran = std::process::Command::new("node")
            .arg(folder.join("harness.js"))
            .current_dir(&folder)
            .output();
        let _ = std::fs::remove_dir_all(&folder);
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the panel's page was not run");
            return None;
        };
        assert!(
            ran.status.success(),
            "the page did not run: {}",
            String::from_utf8_lossy(&ran.stderr)
        );
        Some(
            serde_json::from_slice(&ran.stdout).expect("the harness prints one JSON answer"),
        )
    }

    /// **Blender's output carries *Copy*, and pressing it hands the open tab's
    /// words to the clipboard byte for byte** — `Queue & History`, the
    /// Creator's third notes, note 13.
    ///
    /// Pressed on the first tab, which holds a line far longer than the panel;
    /// then on the second tab, chosen after the panel opened; then on the first
    /// again after Queue's live panel added words to it; then once more where
    /// the engine refuses the first way of copying, so the second way is the
    /// one that has to carry the words.
    #[test]
    fn copy_hands_the_open_tabs_words_over_exactly_as_kept() {
        let Some(said) = the_panel_is_run(
            r#"
const panel = theWordsPanel({
  title: "Blender output", subtitle: "Kitchen", copy: "Copy", close: "Close",
  tabs: [
    { name: "Output", says: "collected", record: words.output },
    { name: "Errors", says: "collected", record: words.errors },
  ],
});
const copy = findAll(panel, (one) => one.tag === "button" && one.classList.contains("words-copy"));
const tabs = findAll(panel, (one) => one.classList.contains("ltab"));
const press = async () => { copy[0].click(); await settle(); };
await press();
tabs[1].click();
await press();
tabs[0].click();
const printed = findAll(panel, (one) => one.tag === "pre")[0];
printed.appendChild(document.createTextNode(words.added));
await press();
theEngineRefuses = true;
await press();
console.log(JSON.stringify({
  copies: copy.length,
  taken: clipboard,
  takenTheOtherWay: takenTheOtherWay,
}));
"#,
        ) else {
            return;
        };
        let (output, errors, added) = the_words_the_harness_hands_over();
        assert_eq!(said["copies"], 1, "the panel carries no *Copy*: {said}");
        let taken: Vec<String> = serde_json::from_value(said["taken"].clone()).expect("a list");
        assert_eq!(
            taken,
            vec![output.clone(), errors, format!("{output}{added}")],
            "the clipboard was not handed the open tab's words exactly as kept"
        );
        let other: Vec<String> =
            serde_json::from_value(said["takenTheOtherWay"].clone()).expect("a list");
        assert_eq!(
            other,
            vec![format!("{output}{added}")],
            "where the engine refuses the first way, the second way does not carry the words"
        );
    }

    /// A caller with no Copy word gets no icon. Queue and History supply it
    /// for printed server output; other screens retain their own contracts.
    #[test]
    fn server_panels_follow_their_own_copy_contract() {
        let Some(said) = the_panel_is_run(
            r#"
const panel = theWordsPanel({
  title: "What the server said", close: "Close",
  tabs: [{ name: "What the server said", record: words.errors }],
});
const copy = findAll(panel, (one) => one.tag === "button" && one.classList.contains("words-copy"));
const icons = findAll(panel, (one) => one.tag === "svg" || one.classList.contains("ib"));
const frame = findAll(panel, (one) => one.classList.contains("lwords"));
console.log(JSON.stringify({
  copies: copy.length,
  icons: icons.length,
  frames: frame.length,
  keepsRoom: frame.length === 1 && frame[0].classList.contains("withcopy"),
}));
"#,
        ) else {
            return;
        };
        assert_eq!(said["copies"], 0, "*What the server said* carries a *Copy*");
        assert_eq!(said["icons"], 0, "*What the server said* carries an icon");
        assert_eq!(said["frames"], 1, "*What the server said* has no frame round its words");
        assert_eq!(
            said["keepsRoom"], false,
            "*What the server said* keeps room in its words for an icon it does not carry"
        );

        // Queue and History offer Copy for printed words on either output.
        // Other screens retain their own panel contracts.
        let read = |name: &str| {
            std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("web").join(name))
                .expect("a page is readable")
        };
        assert!(
            read("the-queue.js").contains("copy: said.copy,"),
            "Queue's Blender output does not hand the panel its *Copy*"
        );
        // Each call that builds a panel, from its opening to its close.
        let the_calls_in = |page: &str| -> Vec<String> {
            page.match_indices("theWordsPanel({")
                .map(|(at, _)| {
                    let rest = &page[at..];
                    rest[..rest.find("});").expect("the call closes")].to_string()
                })
                .collect()
        };
        let history = the_calls_in(&read("the-history.js"));
        // **History's kept words open from `web/the-queue.js`**, where the
        // panel moved when a first-frame pair's box began to open a check's
        // words on either screen (the nineteenth notes, note 7).
        let blender = the_calls_in(&read("the-queue.js"))
            .into_iter()
            .find(|call| call.contains("said.wordsTitle"))
            .expect("History's Blender output panel");
        assert!(
            blender.contains("copy: said.wordsCopy,"),
            "History's Blender output does not hand the panel its *Copy*"
        );
        let server = history.iter().find(|call| call.contains("said.failTheServersWordsTitle")).expect("History's server output panel");
        assert!(server.contains("copy: said.failWordsCopy,"), "Queue & History's output contract includes printed server words");
        let mut the_servers_words = Vec::new();
        the_servers_words.extend(the_calls_in(&read("servers.js")));
        // **A setup run's *Server output* is not *What the server said*, and it
        // hands the panel its *Copy*** — `Design/Server Setup § A long step`,
        // the Creator's thirteenth notes, note 7 (2026-10-18): *"3 lines with
        // an option to open the whole thing and copy to clipboard."*
        let (server_output, set_up): (Vec<String>, Vec<String>) = the_calls_in(&read("set-up.js"))
            .into_iter()
            .partition(|call| call.contains("title: output.title,"));
        assert_eq!(server_output.len(), 1, "Set up's *Server output* panel was not found");
        assert!(
            server_output[0].contains("copy: output.copy,"),
            "*Server output* does not hand the panel its *Copy*"
        );
        the_servers_words.extend(set_up);
        assert!(the_servers_words.len() >= 3, "the panels behind *What the server said* were not found");
        for call in the_servers_words {
            assert!(
                !call.contains("copy:"),
                "*What the server said* is handed a *Copy*: {call}"
            );
        }
    }

    #[test]
    fn seventeenth_piece_2_again_copy_follows_only_the_selected_tabs_printed_words() {
        let Some(said) = the_panel_is_run(r#"
const panel = theWordsPanel({title:"Output", copy:"Copy", tabs:[
  {name:"Output", record:"", said:"Nothing printed"}, {name:"Errors",record:"Failure"}
]});
const frame = panel.querySelector(".lwords");
const copy = frame.querySelector(".words-copy");
const tabs = panel.querySelectorAll(".ltab");
const answer = {empty:copy.hidden, noRoom:!frame.classList.contains("withcopy")};
tabs[1].click();answer.printed=!copy.hidden;
tabs[0].click();answer.emptyAgain=copy.hidden;
const body = panel.querySelector(".words-body");
body.querySelector("pre.lbody").textContent="First printed line";
theNewestWordsStayInView(body);answer.arrival=!copy.hidden;
console.log(JSON.stringify(answer));
"#) else { return; };
        assert_eq!(said, serde_json::json!({"empty":true,"noRoom":true,"printed":true,"emptyAgain":true,"arrival":true}));
    }

    /// **The header carries no *Copy*; the words carry the copy icon, named
    /// *Copy*** — `Queue & History`, the Creator's fourth notes, note 2: *"it
    /// should be a copy icon right in the window, not up top as a text
    /// button."*
    ///
    /// The icon is the Files toolbar's own, so its shape is held against the
    /// drawing's `<symbol id="i-copy">` itself: the same box, the same weight,
    /// the same rectangle and the same path. Its name, on hover and to a screen
    /// reader, is the word the application hands over, and the words keep room
    /// for it.
    #[test]
    fn the_header_carries_no_copy_and_the_words_carry_the_copy_icon() {
        let Some(said) = the_panel_is_run(
            r#"
const panel = theWordsPanel({
  title: "Blender output", subtitle: "Kitchen", copy: "Copy", close: "Close",
  tabs: [
    { name: "Output", says: "collected", record: words.output },
    { name: "Errors", says: "collected", record: words.errors },
  ],
});
const head = findAll(panel, (one) => one.classList.contains("lhead"))[0];
const frame = findAll(panel, (one) => one.classList.contains("lwords"));
const copy = findAll(panel, (one) => one.tag === "button" && one.classList.contains("words-copy"));
const icon = copy.length ? copy[0] : null;
const svg = icon ? icon.children.find((one) => one.tag === "svg") : null;
const shapes = svg ? svg.children.map((one) => ({ tag: one.tag, attributes: one.attributes })) : [];
const hover = icon ? icon.children.filter((one) => one.classList && one.classList.contains("ibl")) : [];
console.log(JSON.stringify({
  headButtons: findAll(head, (one) => one.tag === "button").length,
  headWords: head.textContent,
  copies: copy.length,
  inTheFrame: frame.length === 1 && icon !== null && icon.parentNode === frame[0],
  keepsRoom: frame.length === 1 && frame[0].classList.contains("withcopy"),
  anIconButton: icon !== null && icon.classList.contains("ib") && icon.classList.contains("lcopy"),
  space: svg ? svg.space : null,
  svg: svg ? svg.attributes : null,
  shapes,
  accessibleName: icon ? icon.getAttribute("aria-label") : null,
  hoverTitle: icon ? icon.title : null,
  hoverName: hover.map((one) => one.textContent),
  wordsOnTheIcon: icon ? icon.text : null,
}));
"#,
        ) else {
            return;
        };
        assert_eq!(said["headButtons"], 0, "the header still carries a control: {said}");
        assert!(
            !said["headWords"].as_str().unwrap_or_default().contains(COPY),
            "the header still carries *Copy*: {said}"
        );
        assert_eq!(said["copies"], 1, "the words carry no copy icon: {said}");
        assert_eq!(said["inTheFrame"], true, "the copy icon is not in the words' frame: {said}");
        assert_eq!(said["keepsRoom"], true, "the words keep no room for the icon: {said}");
        assert_eq!(said["anIconButton"], true, "the copy control is not drawn as an icon: {said}");
        assert_eq!(said["accessibleName"], COPY, "the icon's accessible name is not *Copy*");
        assert_eq!(said["hoverTitle"], COPY, "the icon's hover name is not *Copy*");
        assert_eq!(said["hoverName"], serde_json::json!([COPY]), "the icon carries no *Copy* on hover");
        assert_eq!(said["wordsOnTheIcon"], "", "the icon carries its word as text beside the shape");

        // **The shape is the drawing's own** — read out of the mockup's
        // `<symbol id="i-copy">` rather than written down twice here.
        assert_eq!(said["space"], "http://www.w3.org/2000/svg", "the icon is not drawn as a shape");
        let mockup = std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../Preparation/Design/Mockup/Smoozy.html"),
        )
        .expect("the drawing is readable");
        let symbol = &mockup[mockup.find("<symbol id=\"i-copy\"").expect("the drawing's copy icon")..];
        let symbol = &symbol[..symbol.find("</symbol>").expect("the symbol closes")];
        let attribute = |within: &str, name: &str| -> String {
            let from = within.find(&format!(" {name}=\"")).unwrap_or_else(|| panic!("no {name} in {within}"))
                + name.len()
                + 3;
            within[from..from + within[from..].find('"').expect("the value closes")].to_string()
        };
        for name in ["viewBox", "fill", "stroke", "stroke-width", "stroke-linejoin"] {
            assert_eq!(
                said["svg"][name], attribute(symbol, name),
                "the icon's {name} is not the drawing's: {said}"
            );
        }
        let rect = &symbol[symbol.find("<rect").expect("the drawing's rectangle")..];
        let path = &symbol[symbol.find("<path").expect("the drawing's path")..];
        let mut drawn = serde_json::json!({});
        for name in ["x", "y", "width", "height", "rx"] {
            drawn[name] = serde_json::Value::String(attribute(rect, name));
        }
        assert_eq!(
            said["shapes"],
            serde_json::json!([
                { "tag": "rect", "attributes": drawn },
                { "tag": "path", "attributes": { "d": attribute(path, "d") } },
            ]),
            "the icon's shape is not the drawing's copy icon"
        );
    }

    /// **The copy icon stays in place while the words scroll** — `Queue &
    /// History`. Each tab's body is what scrolls; the icon sits in the frame
    /// round the bodies, never inside one, and the frame itself does not
    /// scroll. Held on the page, where the icon is put, and in the two
    /// stylesheets, where the scrolling and the placing are said — the placing
    /// in the bottom-right corner of the words' box (the Creator's fifth notes,
    /// note 6, piece W), at a place the page measures (seventh notes, note 7).
    #[test]
    fn the_copy_icon_stays_in_place_while_the_words_scroll() {
        let Some(said) = the_panel_is_run(
            r#"
const panel = theWordsPanel({
  title: "Blender output", subtitle: "Kitchen", copy: "Copy", close: "Close",
  tabs: [
    { name: "Output", says: "collected", record: words.output },
    { name: "Errors", says: "collected", record: words.errors },
  ],
});
const frame = findAll(panel, (one) => one.classList.contains("lwords"))[0];
const bodies = findAll(panel, (one) => one.classList.contains("words-body"));
const icon = findAll(panel, (one) => one.tag === "button" && one.classList.contains("words-copy"))[0];
let insideABody = false;
for (let at = icon && icon.parentNode; at; at = at.parentNode) {
  if (at.classList && at.classList.contains("words-body")) insideABody = true;
}
console.log(JSON.stringify({
  insideABody,
  bodiesInTheFrame: bodies.length === 2 && bodies.every((one) => one.parentNode === frame),
}));
"#,
        ) else {
            return;
        };
        assert_eq!(said["insideABody"], false, "the copy icon scrolls with the words: {said}");
        assert_eq!(said["bodiesInTheFrame"], true, "the words are not inside the icon's frame: {said}");

        // The rule a stylesheet gives one selector, from its opening brace to
        // its close.
        let read = |name: &str| {
            std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("web").join(name))
                .expect("a stylesheet is readable")
        };
        let the_rule = |sheet: &str, selector: &str| -> String {
            let at = sheet
                .find(&format!("\n{selector} {{"))
                .unwrap_or_else(|| panic!("no rule for {selector}"));
            let rest = &sheet[at..];
            rest[..rest.find('}').expect("the rule closes")].to_string()
        };
        let drawing = read("the-drawing.css");
        let frame = the_rule(&drawing, ".lwords");
        assert!(frame.contains("position: relative"), "the icon's frame places nothing: {frame}");
        assert!(!frame.contains("overflow"), "the icon's frame scrolls: {frame}");
        let icon = the_rule(&drawing, ".lwords .lcopy");
        assert!(icon.contains("position: absolute"), "the copy icon is not held to the frame's corner");

        // **Where in the corner is measured on the drawn page**, the seventh
        // notes, note 7; the rule takes the measured figures, and its own are
        // only where the icon sits unmeasured. The measuring is held by
        // `the_copy_icon_is_equally_clear_of_the_inner_box`.
        assert!(!icon.contains(" top:"), "the copy icon is back at the top: {icon}");
        assert!(
            icon.contains("bottom: var(--lcopy-bottom") && icon.contains("right: var(--lcopy-right"),
            "the copy icon does not take its measured place: {icon}"
        );
        assert!(
            the_rule(&read("the-machines-own-words.css"), ".words-body").contains("overflow: auto"),
            "the words do not scroll inside the frame"
        );
    }

    /// **The copy icon is as far from the inner box's right as from its
    /// bottom**, counted from the box's visible inside: inside its border and
    /// clear of any scroll bar — the Creator's seventh notes, note 7: *"Just
    /// add an equal margin from the right and the bottom. But not from the
    /// pop-up window itself — from the inner window."* Measured on a page a
    /// real browser draws with the two stylesheets and the panel's own script,
    /// for a short record and for one long enough to scroll, at the end and
    /// scrolled back to the top. A computer with no Chrome skips it, as one
    /// with no Node skips the page runs above.
    #[test]
    fn the_copy_icon_is_equally_clear_of_the_inner_box() {
        let chrome = [
            "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
            "/Applications/Chromium.app/Contents/MacOS/Chromium",
            "/usr/bin/google-chrome",
            "/usr/bin/chromium",
            "/usr/bin/chromium-browser",
            "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
        ]
        .into_iter()
        .map(std::path::PathBuf::from)
        .find(|one| one.exists());
        let Some(chrome) = chrome else {
            eprintln!("SKIPPED: no Chrome on this computer, so the copy icon's place was not measured");
            return;
        };
        let folder = std::env::temp_dir().join(format!(
            "smoozy-the-copy-icons-place-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for name in ["the-machines-own-words.js", "the-drawing.css", "the-machines-own-words.css"] {
            std::fs::copy(web.join(name), folder.join(name)).expect("the page is copied");
        }
        for (name, text) in THE_STAND_INS {
            std::fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        std::fs::write(folder.join("index.html"), THE_MEASURED_PAGE).expect("the page is written");
        let dumped = folder.join("dumped.html");
        let mut chrome = std::process::Command::new(chrome)
            .args([
                "--headless",
                "--disable-gpu",
                "--no-first-run",
                "--allow-file-access-from-files",
                "--virtual-time-budget=5000",
                "--window-size=1200,900",
                "--dump-dom",
            ])
            .arg(format!("--user-data-dir={}", folder.join("profile").display()))
            .arg(format!("file://{}", folder.join("index.html").display()))
            .stdout(std::fs::File::create(&dumped).expect("the dump is written"))
            .stderr(std::process::Stdio::null())
            .spawn()
            .expect("Chrome starts");
        // A fixed deadline, never an open wait: a browser that has not drawn
        // the page in a minute is stopped.
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(60);
        while chrome.try_wait().expect("Chrome is watched").is_none() {
            if std::time::Instant::now() > deadline {
                let _ = chrome.kill();
                let _ = chrome.wait();
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        let page = std::fs::read_to_string(&dumped).unwrap_or_default();
        let _ = std::fs::remove_dir_all(&folder);
        let said = page
            .split("<pre id=\"out\">")
            .nth(1)
            .and_then(|rest| rest.split("</pre>").next())
            .filter(|said| !said.is_empty())
            .unwrap_or_else(|| panic!("the page drew no measurement: {page}"));
        let said: serde_json::Value = serde_json::from_str(said).expect("one JSON answer");
        assert_eq!(said["long"]["scrolls"], true, "the long record does not scroll: {said}");
        for record in ["short", "long"] {
            for when in ["atTheEnd", "scrolledUp"] {
                let gap = &said[record][when];
                let right = gap["right"].as_f64().expect("a gap on the right");
                let bottom = gap["bottom"].as_f64().expect("a gap at the bottom");
                assert!(
                    (right - bottom).abs() <= 0.5 && right > 0.0,
                    "the {record} record, {when}: the copy icon is {right}px from the box's \
                     visible inside on the right and {bottom}px at the bottom: {said}"
                );
            }
        }
    }

    /// **A page that draws the panel and measures the icon's two gaps** to the
    /// shown record's visible inside — inside its border, inside its scroll
    /// bar, and inside whatever scrolls it — and writes them into `#out`.
    const THE_MEASURED_PAGE: &str = r#"<!doctype html><html><head><meta charset="utf-8">
<link rel="stylesheet" href="the-drawing.css"><link rel="stylesheet" href="the-machines-own-words.css">
</head><body><pre id="out"></pre>
<script type="module">
import { theWordsPanel, theNewestWordsStayInView } from "./the-machines-own-words.js";
const drawn = () => new Promise((done) => setTimeout(done, 0));
const gaps = (panel) => {
  const frame = panel.querySelector(".lwords");
  const icon = frame.querySelector(".lcopy").getBoundingClientRect();
  const shown = panel.querySelector("pre.lbody");
  const box = shown.getBoundingClientRect();
  let right = box.left + shown.clientLeft + shown.clientWidth;
  let bottom = box.top + shown.clientTop + shown.clientHeight;
  for (let up = shown.parentElement; up !== frame; up = up.parentElement) {
    const clip = up.getBoundingClientRect();
    right = Math.min(right, clip.left + up.clientLeft + up.clientWidth);
    bottom = Math.min(bottom, clip.top + up.clientTop + up.clientHeight);
  }
  return { right: Math.round((right - icon.right) * 10) / 10, bottom: Math.round((bottom - icon.bottom) * 10) / 10 };
};
const run = async (record) => {
  const panel = theWordsPanel({ title: "Blender output", subtitle: "Kitchen", copy: "Copy", close: "Close",
    tabs: [{ name: "Output", says: "collected", record }] });
  panel.hidden = false;
  document.body.appendChild(panel);
  const body = panel.querySelector(".words-body");
  body.scrollTop = body.scrollHeight;
  body.dispatchEvent(new Event("scroll"));
  await drawn();
  const atTheEnd = gaps(panel);
  const scrolls = body.scrollHeight > body.clientHeight;
  body.scrollTop = 0;
  body.dispatchEvent(new Event("scroll"));
  await drawn();
  const scrolledUp = gaps(panel);
  panel.remove();
  return { atTheEnd, scrolledUp, scrolls };
};
const line = (at) => "Fra:" + at + " Mem:9.84G | Time:01:12.44 | Scene, Interior | Path Tracing Sample " + at + "/2048";
const short = await run([1, 2, 3].map(line).join("\n"));
const long = await run(Array.from({ length: 400 }, (_, at) => line(at)).join("\n"));
document.getElementById("out").textContent = JSON.stringify({ short, long });
</script></body></html>"#;

    /// **History's kept words carry the word *Copy*** for the page to hand on.
    #[test]
    fn historys_kept_words_carry_copy() {
        let event = the_kept_words_are_said(
            Event::about(Subject::TheRun, "a kind"),
            "a job",
            "a server",
            &[(1, the_quiet_ending())],
        );
        // The event's own fields are the queue's; its printed form names each
        // field with what it carries.
        assert!(
            format!("{event:?}").contains(&format!("\"wordsCopy\": {:?}", Said::Prose(Prose::new(COPY)))),
            "History's kept words carry no *Copy*: {event:?}"
        );
    }

    /// **Queue's live Blender output carries the word *Copy*** for the page to
    /// hand on — `the_running_words_said` in `src/surface/the_queue.rs`.
    #[test]
    fn queues_running_words_carry_copy() {
        let event = crate::surface::the_queue::the_running_words_said(
            &crate::servers::the_machines_work::TheWordsThatArrived {
                job: "a job".to_string(),
                name: "Kitchen".to_string(),
                first: true,
                new: [String::new(), String::new()],
                any_yet: [false, false],
            },
        );
        assert!(
            format!("{event:?}").contains(&format!("\"copy\": {:?}", Said::Prose(Prose::new(COPY)))),
            "Queue's running words carry no *Copy*: {event:?}"
        );
    }

    /// The stand-ins the panel's page is loaded beside.
    const THE_STAND_INS: [(&str, &str); 3] = [
        ("package.json", r#"{ "type": "module" }"#),
        ("event-route.js", "export function whenTold() {}"),
        (
            "the-failure-shape.js",
            r#"export const EXPANDED = "expanded";
export function theFailureShape() { return document.createElement("div"); }
export function theControlsFrom() { return []; }"#,
        ),
    ];

    /// **A page just big enough for the panel**, with a stand-in clipboard
    /// that records what it is handed and can be told to refuse.
    const THE_FAKE_PAGE: &str = r#"
function anElement(tag) {
  const names = new Set();
  const listeners = new Map();
  const element = {
    tag, children: [], parentNode: null, dataset: {}, hidden: false, text: "",
    value: "", type: "", title: "", attributes: {},
    classList: {
      add: (...more) => more.forEach((one) => names.add(one)),
      contains: (one) => names.has(one),
      toggle: (one, on) => { if (on) names.add(one); else names.delete(one); },
    },
    set className(value) { names.clear(); String(value).split(" ").filter(Boolean).forEach((one) => names.add(one)); },
    get className() { return Array.from(names).join(" "); },
    set textContent(value) { element.children = []; element.text = String(value); },
    get textContent() { return element.text + element.children.map((one) => one.textContent).join(""); },
    appendChild(child) { child.parentNode = element; element.children.push(child); return child; },
    remove() { if (element.parentNode) element.parentNode.children = element.parentNode.children.filter((one) => one !== element); },
    setAttribute(name, value) { element.attributes[name] = String(value); },
    getAttribute(name) { return name in element.attributes ? element.attributes[name] : null; },
    select() {},
    addEventListener(kind, handler) {
      const held = listeners.get(kind) || [];
      held.push(handler);
      listeners.set(kind, held);
    },
    click() { for (const one of listeners.get("click") || []) one({ target: element }); },
    closest(selector) {
      const [wanted, named] = selector.split(".");
      for (let at = element; at; at = at.parentNode) {
        if ((!wanted || at.tag === wanted) && (!named || at.classList.contains(named))) return at;
      }
      return null;
    },
    querySelector(selector) {
      const [wanted, named] = selector.split(".");
      return findAll(element, (one) => one !== element && (!wanted || one.tag === wanted) && (!named || one.classList.contains(named)))[0] || null;
    },
    querySelectorAll(selector) {
      const named = selector.split(".")[1];
      return findAll(element, (one) => one !== element && one.classList.contains(named));
    },
  };
  return element;
}
function findAll(element, fits, found = []) {
  if (element && element.classList && fits(element)) found.push(element);
  for (const child of (element && element.children) || []) findAll(child, fits, found);
  return found;
}
globalThis.document = {
  createElement: anElement,
  createElementNS: (space, tag) => Object.assign(anElement(tag), { space }),
  createTextNode: (text) => ({ textContent: text, children: [] }),
  body: anElement("body"),
  execCommand(command) {
    if (command !== "copy") return false;
    const held = findAll(document.body, (one) => one.tag === "textarea")[0];
    takenTheOtherWay.push(held.value);
    return true;
  },
};
const clipboard = [];
const takenTheOtherWay = [];
let theEngineRefuses = false;
Object.defineProperty(globalThis, "navigator", {
  configurable: true,
  value: {
    clipboard: {
      writeText(words) {
        if (theEngineRefuses) return Promise.reject(new Error("refused"));
        clipboard.push(words);
        return Promise.resolve();
      },
    },
  },
});
const settle = () => new Promise((done) => setImmediate(done));
const words = JSON.parse((await import("node:fs")).readFileSync("words.json", "utf8"));
const { theWordsPanel, theNewestWordsStayInView } = await import("./the-machines-own-words.js");
"#;
}
