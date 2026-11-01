//! **The underway shape's own service** — §16.5 and §16.7, and the page half
//! is `web/the-underway-shape.js`.
//!
//! **This file is the surface and not the shape.** The shape is
//! `src/presentation/the_underway_shape.rs` and the repairing server's three
//! readings are `src/presentation/the_repairing_server.rs`; the rules every
//! drawing inside them keeps are at the head of `src/presentation/mod.rs`.
//! Read that before adding anything here.
//!
//! ## One command, three event kinds, and why the answer arrives as an event
//!
//! P6: *results reach the interface as events on one ordered queue, and the
//! interface holds no other route to a module.* **A command is the page asking;
//! an event is the application telling.** The one command below reads the
//! durable record and posts; the byte loop's own reports arrive later, on the
//! same queue, and the page replaces its drawing when they do.
//!
//! ## This is where the desktop's end of G4's transfer route lives
//!
//! **M19: the three reporting routes are `Server protocol and transfers` G4's
//! and this document reads them rather than inventing a fourth.**
//! [`TheTransfersReporter`] is the desktop's end of the **first** of the three:
//! it implements `smoozy_server_protocol`'s own
//! [`WatchesTheTransfer`] seam, takes the report the byte loop composes at its
//! accounting interval, and puts it on the queue. **Nothing is composed here
//! that the report does not carry** — the count is Smoozy's own at Smoozy's own
//! loop (`Server protocol and transfers` G5), and the two marks beside it are
//! `Server protocol and transfers` G4a and
//! `Server protocol and transfers` G4b, carried rather than re-derived.
//!
//! **The other two routes are read where they arrive** — the event journal for
//! an operation, and a reading's own result — and neither is this file's to
//! invent.
//!
//! ## Why a run-held reading may go on the queue, which is a ruling
//!
//! P7 says *a UI event never carries state that has not already been written
//! durably*, and two of M28's three readings are **run-held by the document
//! that publishes them** — `Servers` N10a says so in as many words. **The
//! ruling, taken 2026-09-09 under the Creator's standing delegation of
//! 2026-08-31 and written into the decisions record:** P7 binds a fact that has
//! a durable home and is about to be shown before it reaches it. Its own
//! reasoning is `State, identity and recovery` §9.4's — each unit's **outcome**
//! durable before it is presented — and the design it forbids is *"drew first
//! and recorded afterwards"*. **A reading declared run-held has no durable home
//! to be ahead of**, so posting it is not that design; P8 is the clause that
//! governs it, and P8 puts events in exactly that class.
//!
//! ## What the two commands may not do
//!
//! **Never take a reading here.** M1 binds the presentation folder and this
//! file feeds it: **a reading arrives already taken**, carried with its own
//! moment and the clock it was taken on (M3a). The one thing this file reads is
//! the local record, which is a record and not a source.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Servers` N10a.
//!
//! **Every other document's marker below is written out with that document at
//! the site.** This plan defines a C-series, a G-series, an R-series and an
//! S-series of its own and so does `Server protocol and transfers`, so a bare
//! one would resolve to a real and wrong clause.

use std::sync::Mutex;

use tauri::State;

use smoozy_server_protocol::transfers::moving::WatchesTheTransfer;
use smoozy_server_protocol::transfers::{Report, StartOverCause};
use smoozy_state::identities::Identity;

use crate::events::{Event, Said, Subject, TheOneQueue};
use crate::presentation::as_of_that_moment::AsOfThatMoment;
use crate::presentation::the_repairing_server::{
    a_repairing_server, TheThreeReadings, WhatTheColumnSays,
};
use crate::presentation::the_underway_shape::{
    underway, DrawnUnderway, HowFarAlong, TheProgressReading, WhatItAlreadyCarries,
    WhatTheChannelCanSay, WhatTheUserCanStillDo,
};
use crate::record::failure_records::DurableSubject;
use crate::scheduling::{
    BytesRemaining, TheDesktopSideAdmissionPoint, TheRemainingTime,
};
use crate::start::Started;
use crate::surface::the_failure_shape::a_subject;
use crate::typography::{Prose, Verbatim};
use smoozy_server_protocol::transfers::Direction;

/// **Something is underway, and the page draws its mark.**
///
/// **One event per thing and never a list in one event**, the same discipline
/// piece C's failure shape keeps: a list is a thing something downstream would
/// be tempted to summarise, and Design's rule is that one thing carries one
/// mark.
pub const SOMETHING_IS_UNDERWAY: &str = "underway:something-is-underway";

/// **It is not underway any more, so the mark goes.**
///
/// **This says nothing about how the work ended.** M21 and
/// `Server protocol and transfers` G9: a waiter that gave up says the waiter
/// gave up, and the work carries on being whatever the record and the journal
/// say it is. The outcome is §16.6's and is drawn by
/// `src/presentation/outcomes_presented.rs`.
pub const IT_IS_NO_LONGER_UNDERWAY: &str = "underway:it-is-no-longer-underway";

/// **A server Smoozy is repairing, with M28's three readings.**
pub const A_SERVER_IS_REPAIRING: &str = "underway:a-server-is-repairing";

/// **The words the page used to write itself, moved here 2026-09-12.**
///
/// A6 and X1 make the typography point *"the only way prose reaches a screen"*
/// and say the enforcement is **structural on both sides**; it was structural on
/// one, and `web/the-underway-shape.js` carried eight strings of prose written
/// straight on to the page. **One of them broke the em-dash rule by hand** — an
/// ordinary space before the dash in *starting again — …*, which is the one rule
/// X3 says *"cannot be held by remembering it"*, broken by somebody remembering
/// it. Found at round 1 of the code audit of what is drawn inside the window
/// (F-6); `src/surface/mod.rs`'s page sweep is what says so now.
///
/// **Each sentence is its own string** (X10), and every number and name stands
/// beside these on the verbatim route rather than inside them (X7).
///
/// **The start-over and its two causes say the event the way a Transfers row
/// says it** (`src/surface/transfers.rs`, *Could not resume — uploading from
/// the beginning.* and *File changed on the server — …*), so the user meets
/// one event in one set of words. Step 13's words sweep, 2026-10-28, and its
/// final words sweep the same day.
const STARTING_AGAIN: &str = "Restarting from the beginning.";
/// **`Server protocol and transfers` G4a's first cause.** It is set where the
/// partial file's prefix disagreed with its checkpoint, where the partial file
/// could not be carried on from at all, and where the source could not be
/// confirmed (`resume.rs`; `Server protocol and transfers` S12 step 5 and
/// `Server protocol and transfers` S13) — so the words say the resume
/// failed and never that a verification did, which only the first is.
const THE_PARTIAL_FILE_COULD_NOT_BE_RESUMED: &str = "Could not resume.";
/// **`Server protocol and transfers` G4a's second cause**: the changed source
/// of `Server protocol and transfers` S13, which starts over only on a
/// download — the file on the server is the one that changed.
const THE_FILE_CHANGED_ON_THE_SERVER: &str = "File changed on the server.";
/// The line drawn while a resume is checking what already went.
const CHECKING_WHAT_WAS_ALREADY_SENT: &str = "Verifying the partial file.";
/// **The countdown's two forms.** The seconds are a value the drawing counts
/// down and stand beside these words.
pub(crate) const TRYING_AGAIN_IN: &str = "retrying in";
/// P10c's hazard said rather than repaired: a suspended computer wakes the
/// reading long past due, and the honest line is that the attempt is due.
pub(crate) const TRYING_AGAIN_NOW: &str = "retrying now";
/// **M3a's *as of that moment***, with the moment beside it as a value.
const AS_OF: &str = "as of";
/// The word before the clock's own name.
const ON_THE_CLOCK: &str = "on";
/// The words a stale last-known reading is drawn under.
const LAST_SEEN: &str = "last seen";
/// The word between how much has gone and how much is expected.
const OF: &str = "of";

/// **The page asks what is underway against a subject.**
///
/// `subject_kind` is one of `State, identity and recovery` §8.7's closed list,
/// written; `subject` is the identity's own text. **H34: both are validated
/// before anything is touched**, through the one validator piece C built.
#[tauri::command]
pub fn what_is_underway_against(
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
    the_underway_things_are_posted(&started, &queue, &subject);
}

/// Read the transfers arranged against a subject and post one underway mark
/// each.
///
/// **M18: the mark comes from the local write and not from an answer.** A
/// transfer's record is durable **before the first byte is written**
/// (`Server protocol and transfers` S8), so a transfer that is recorded and has
/// not ended is a thing the user asked for that Smoozy has not finished — which
/// is what *underway* means and is true from the instant of the click.
///
/// **How far along is `ItCannotSay` until the byte loop reports, and that is a
/// finding rather than a shortcut.** The durable record's own high-water offset
/// is a reading and **`Server protocol and transfers`' checkpoint carries no
/// moment** — an offset and a digest and nothing else — so a figure recovered
/// from the record after a restart cannot be presented under M3a without this
/// side inventing a moment for it. **What holds is Design's own sentence**:
/// *"a shape that never becomes a figure is the app saying it cannot … That
/// shape is itself the admission and no words are owed anywhere for it."* The
/// figure appears the moment [`TheTransfersReporter`] posts one, which carries
/// its own moment on this computer's clock.
pub fn the_underway_things_are_posted(
    started: &Started,
    queue: &TheOneQueue,
    subject: &DurableSubject,
) {
    let Some(record) = started.record.as_ref() else {
        return;
    };
    let Some(against) = subject.identity() else {
        return;
    };
    let Ok(transfers) = crate::record::transfer_records::all(record.connection()) else {
        return;
    };

    // **P8, and it is told rather than assumed**: the records were just read
    // out of the durable store against this subject, so the subject exists.
    // **When the module that arranges a transfer arrives it calls
    // `the_subject_exists` at creation and this stops being the first caller
    // rather than becoming a second answer** — telling the queue twice about
    // one subject is one entry in a set.
    queue.the_subject_exists(against.clone());

    for transfer in transfers {
        // **The subject may be the server row the transfers were arranged
        // against, or one transfer arrangement of `State, identity and
        // recovery` §8.7's list.** Both are on that closed list and both are
        // asked about here.
        let mine = transfer.server == *against || transfer.transfer == *against;
        if !mine || transfer.ending.is_some() {
            continue;
        }
        queue.the_subject_exists(transfer.transfer.clone());
        let Some(drawn) = underway(
            // **Design: on the row, where the work belongs to one row of a list
            // and the user will come back to it.** A transfer is that document's
            // own example.
            WhatTheUserCanStillDo::EverythingButThatOneRow,
            WhatItAlreadyCarries::Nothing,
            &transfer.transfer.as_text(),
            HowFarAlong::ItCannotSay,
            WhatTheChannelCanSay::ItIsMoving,
        ) else {
            continue;
        };
        let subject_of_the_event = Subject::Durable(transfer.transfer.clone());
        let whole = transfer.total_size_expected;
        let _ = queue.read_then_post::<DrawnUnderway, ()>(
            || Ok(drawn.clone()),
            |drawn| {
                the_underway_event(subject_of_the_event.clone(), drawn)
                    // **The whole is durable before the first byte**
                    // (`Server protocol and transfers` S8), so it exists to be
                    // read from the first moment. It is a value on the verbatim
                    // route (X9).
                    .saying("theWhole", Said::Value(Verbatim::new(whole.to_string())))
            },
        );
    }
}

/// One underway mark as an event. **Every string is on one of the two routes
/// and the two are the complete set** (A6, X9).
fn the_underway_event(about: Subject, drawn: &DrawnUnderway) -> Event {
    let mut event = Event::about(about, SOMETHING_IS_UNDERWAY)
        .saying("mark", Said::Value(Verbatim::new(drawn.mark)))
        .saying(
            "whatItIsOn",
            Said::Value(drawn.what_it_is_on.clone()),
        )
        .saying(
            "theChannel",
            Said::Value(Verbatim::new(match drawn.the_channel {
                WhatTheChannelCanSay::ItIsMoving => "moving",
                // **M21: a stalled channel is a stalled channel and never a
                // broken connection.** The word the page draws says only that.
                WhatTheChannelCanSay::ItIsStalled => "stalled",
            })),
        );
    if let HowFarAlong::Of(reading) = &drawn.how_far_along {
        event = event
            .saying("staged", Said::Value(reading.staged.value.clone()))
            .saying("expected", Said::Value(reading.expected.value.clone()))
            .saying(
                "takenAt",
                Said::Value(Verbatim::new(
                    reading
                        .staged
                        .taken_at
                        .map(|at| at.to_string())
                        .unwrap_or_default(),
                )),
            )
            .saying(
                "onTheClock",
                Said::Value(Verbatim::new(reading.staged.on_the_clock.clone())),
            )
            // **The words the figures are drawn between, each on its own
            // route** — the figures are values and never travel inside a
            // sentence (X7).
            .saying("theOfWord", Said::Prose(Prose::new(OF)))
            .saying("theReadingAsOf", Said::Prose(Prose::new(AS_OF)))
            .saying(
                "theReadingOnTheClock",
                Said::Prose(Prose::new(ON_THE_CLOCK)),
            );
    }
    event
}

/// **The desktop's end of `Server protocol and transfers` G4's first route.**
///
/// The byte loop composes a report at its own accounting interval — **bounded
/// above in time as well as in bytes**, so the gap between reports is bounded —
/// and hands it here. **This puts it on the queue and composes nothing.**
///
/// **It is run-held and writes nothing durable** (P8). The transfer's own
/// durable record is written by the loop's record seam and not by this.
pub struct TheTransfersReporter {
    queue: std::sync::Arc<TheOneQueue>,
    transfer: Identity,
    /// The moment the run began, on this computer's clock, so a report's
    /// moment is a moment rather than an elapsed time —
    /// `State, identity and recovery` D5 forbids arithmetic across two clocks
    /// and an interval composed here would already have done it.
    taken_on: std::time::SystemTime,
}

impl TheTransfersReporter {
    /// **The transfer's identity is the subject and the queue is told it
    /// exists.** The record is durable before the first byte
    /// (`Server protocol and transfers` S8), so it does.
    pub fn about(queue: std::sync::Arc<TheOneQueue>, transfer: Identity) -> TheTransfersReporter {
        queue.the_subject_exists(transfer.clone());
        TheTransfersReporter {
            queue,
            transfer,
            taken_on: std::time::SystemTime::now(),
        }
    }

    /// The moment a report is taken, on this computer's clock.
    ///
    /// **This is the one place in this piece that reads a clock**, and it reads
    /// the wall clock rather than taking a measurement: M3a wants the moment a
    /// reading was taken, and the page draws it. It is not a timer, nothing
    /// repeats, and nothing here polls (P9, P10).
    fn now(&self) -> AsOfThatMoment {
        AsOfThatMoment::from(&smoozy_state::observed::Reading::taken(
            String::new(),
            std::time::SystemTime::now().max(self.taken_on),
            smoozy_state::observed::Clock::ThisComputer,
        ))
    }
}

impl WatchesTheTransfer for TheTransfersReporter {
    /// **G4's count, put on the queue with its own moment.**
    ///
    /// **M20 and `Server protocol and transfers` G5: nothing here derives
    /// progress from the far end's view of the file.** The two figures are the
    /// report's own.
    ///
    /// **`Server protocol and transfers` G4a's mark rides with the count**, so
    /// a count that has fallen to zero is never left to stand as the whole
    /// report: *"nothing infers a start-over from the count falling, exactly as
    /// nothing infers an ending from a rate falling."*
    ///
    /// **And `Server protocol and transfers` G4b's mark rides with it too**, so
    /// an unmoving count on a resume is never the whole report and
    /// `Server protocol and transfers` G7's stall test does not read a
    /// verifying channel as stalled.
    fn report(&mut self, report: &Report) {
        let moment = self.now();
        let staged = AsOfThatMoment {
            value: Verbatim::new(report.bytes_staged.to_string()),
            taken_at: moment.taken_at,
            on_the_clock: moment.on_the_clock.clone(),
        };
        let expected = AsOfThatMoment {
            value: Verbatim::new(report.bytes_expected.to_string()),
            taken_at: moment.taken_at,
            on_the_clock: moment.on_the_clock.clone(),
        };
        let Some(drawn) = underway(
            WhatTheUserCanStillDo::EverythingButThatOneRow,
            WhatItAlreadyCarries::Nothing,
            &self.transfer.as_text(),
            HowFarAlong::Of(TheProgressReading { staged, expected }),
            // **M22: nothing here detects a hang.** A report is a report and
            // says the channel is moving; the stall test is
            // `Server protocol and transfers` G7's and its inactivity deadline
            // is that document's C19, both of which live in the byte loop and
            // reach this side as their own statement. **What this side does not
            // do is make a hang undetectable later**: the moment above is
            // carried with every count, which is the whole of what G7's test
            // reads.
            WhatTheChannelCanSay::ItIsMoving,
        ) else {
            return;
        };
        let about = Subject::Durable(self.transfer.clone());
        let started_over = report.started_over;
        let verifying = report.verifying_a_prefix_of;
        let _ = self.queue.read_then_post::<DrawnUnderway, ()>(
            || Ok(drawn.clone()),
            |drawn| {
                let mut event = the_underway_event(about.clone(), drawn);
                if let Some(cause) = started_over {
                    // **`Server protocol and transfers` G4a: the count falling
                    // to zero is never left to stand as the whole report.**
                    //
                    // **Both of these are prose and used to travel as values.**
                    // A cause written for a person is a sentence, and the
                    // verbatim route is reserved for data Smoozy must not alter
                    // (X9) — so neither the orphan rule nor the em-dash rule
                    // reached them, and the page then wrote the *starting
                    // again — * half itself.
                    //
                    // ~~**THE WORDING OF THE TWO CAUSES IS OWED TO DESIGN**~~ —
                    // the plumbing's own *The staged prefix did not verify.* and
                    // *The source changed under the download.* were replaced
                    // at step 13's words sweep (2026-10-28) by the Transfers
                    // row's own words for the same event; no Design document
                    // quotes the underway line.
                    event = event
                        .saying("startingAgain", Said::Prose(Prose::new(STARTING_AGAIN)))
                        .saying(
                            "startedOver",
                            Said::Prose(Prose::new(match cause {
                                StartOverCause::TheStagedPrefixDidNotVerify => {
                                    THE_PARTIAL_FILE_COULD_NOT_BE_RESUMED
                                }
                                StartOverCause::TheSourceChangedUnderADownload => {
                                    THE_FILE_CHANGED_ON_THE_SERVER
                                }
                            })),
                        );
                }
                if let Some(prefix) = verifying {
                    // **`Server protocol and transfers` G4b: an unmoving count
                    // is never the whole report on a resume.** The figure is a
                    // value and the line beside it is prose.
                    event = event
                        .saying(
                            "verifyingAPrefixOf",
                            Said::Value(Verbatim::new(prefix.to_string())),
                        )
                        .saying(
                            "theCheckingLine",
                            Said::Prose(Prose::new(CHECKING_WHAT_WAS_ALREADY_SENT)),
                        );
                }
                event
            },
        );
    }
}

/// **The mark goes.** The work is no longer underway, and this says nothing
/// about how it ended — M21.
pub fn it_is_no_longer_underway(queue: &TheOneQueue, transfer: &Identity) {
    let _ = queue.read_then_post::<(), ()>(
        || Ok(()),
        |()| {
            Event::about(
                Subject::Durable(transfer.clone()),
                IT_IS_NO_LONGER_UNDERWAY,
            )
            .saying(
                "whatItIsOn",
                Said::Value(Verbatim::new(transfer.as_text())),
            )
        },
    );
}

/// **A repairing server's three readings, posted** — M28.
///
/// **All three arrive already taken**, each carrying its own moment and the
/// clock it was taken on (M3a, M1). Nothing is read here and nothing is
/// composed from two of them (M31).
///
/// **The moment the next attempt is due goes across as a moment and never as a
/// remaining time.** M30: the countdown is a drawing that ticks against a
/// published moment, and a remaining time computed here would be this side
/// running the clock down — which is the timer P10b forbids.
pub fn a_server_is_repairing(
    queue: &TheOneQueue,
    server_row: &Identity,
    readings: TheThreeReadings,
    a_repair_is_under_way: bool,
) {
    queue.the_subject_exists(server_row.clone());
    let drawn = a_repairing_server(&server_row.as_text(), readings, a_repair_is_under_way);
    let about = Subject::Durable(server_row.clone());
    let _ = queue.read_then_post::<(), ()>(
        || Ok(()),
        |()| {
            let mut event = Event::about(about.clone(), A_SERVER_IS_REPAIRING)
                .saying(
                    "serverRow",
                    Said::Value(drawn.server_row.clone()),
                )
                // **`Servers` K6's value travels as supply and the page does
                // not draw it as the column's word** — M29.
                .saying(
                    "theConnectionCondition",
                    Said::Value(drawn.readings.the_connection_condition.value.clone()),
                );
            if let Some(due) = &drawn.readings.the_moment_the_next_attempt_is_due {
                // **M30: the countdown ticks against this moment and nothing
                // else.** The words it is drawn with are prose and come from
                // here; the seconds it counts down are the drawing's own
                // arithmetic against the moment.
                event = event
                    .saying("theNextAttemptIsDueAt", Said::Value(due.value.clone()))
                    .saying("theTryingAgainIn", Said::Prose(Prose::new(TRYING_AGAIN_IN)))
                    .saying(
                        "theTryingAgainNow",
                        Said::Prose(Prose::new(TRYING_AGAIN_NOW)),
                    );
            }
            if let Some(work) = &drawn.readings.the_last_known_work_reading {
                event = event
                    .saying("theLastKnownWork", Said::Value(work.value.clone()))
                    .saying(
                        "theLastKnownWorkWasTakenAt",
                        Said::Value(Verbatim::new(
                            work.taken_at.map(|at| at.to_string()).unwrap_or_default(),
                        )),
                    )
                    .saying(
                        "theLastKnownWorkClock",
                        Said::Value(Verbatim::new(work.on_the_clock.clone())),
                    )
                    // **M3a, and the reading is stale by construction, which
                    // saying so is the requirement.** The words are prose; the
                    // moment and the clock's name stand beside them.
                    .saying("theLastSeen", Said::Prose(Prose::new(LAST_SEEN)))
                    .saying(
                        "theReadingOnTheClock",
                        Said::Prose(Prose::new(ON_THE_CLOCK)),
                    );
            }
            // **M31: both contents where both are true, each on its own.**
            // Which occupies the one slot Design drew is Design's, and the page
            // is where that choice is made.
            for (at, content) in drawn.the_column_is_owed.iter().enumerate() {
                let name: &'static str = match at {
                    0 => "theColumnMaySayFirst",
                    1 => "theColumnMaySaySecond",
                    _ => continue,
                };
                // **The word is prose and the job's name is a value, and the
                // two travel on two routes** — X7 and the folder head's rule 4.
                event = event.saying(name, Said::Prose(Prose::new(content.the_word())));
                if let WhatTheColumnSays::Rendering(job) = content {
                    event = event.saying("theJobsName", Said::Value(job.clone()));
                }
            }
            event
        },
    );
}

/// **`Desktop app and delivery` R15a and R15b's readings, taken at the
/// accounting point and handed on already taken.**
///
/// **This is the caller both clauses were waiting for**, and it is here rather
/// than in `src/presentation/` because M1 forbids a presentation path taking a
/// reading: *"a reading arrives here already taken"*. This function is where it
/// is taken.
///
/// - **R15a's achieved rate**, per direction, over
///   `Server protocol and transfers` R12a's window.
/// - **R15b's remaining-time estimate**, over this request's own channels and
///   no others, with its three no-estimate states.
///
/// **Both come back as readings with their own moment** (M3a). The moment on
/// the accounting point's own clock is monotonic and cannot be drawn, so the
/// moment carried to the surface is this computer's wall clock at the instant
/// the reading was taken — `State, identity and recovery` D5 is untouched
/// because nothing subtracts across the two: the window's own arithmetic stays
/// entirely on the monotonic clock inside `src/scheduling.rs`, and what crosses
/// here is a figure and a moment rather than an interval.
pub fn the_rate_and_the_remaining_time(
    point: &TheDesktopSideAdmissionPoint,
    direction: Direction,
    the_requests_channels: &[u64],
    bytes_remaining: BytesRemaining,
    the_request_has_ended: bool,
    now: std::time::Instant,
) -> (Option<AsOfThatMoment>, TheRemainingTime) {
    let taken_at = std::time::SystemTime::now();
    let rate = point
        .the_reading_at_this_accounting_interval(direction, now)
        .map(|reading| {
            AsOfThatMoment::from(&smoozy_state::observed::Reading::taken(
                reading.value().0.to_string(),
                taken_at,
                reading.the_clock_it_was_taken_on(),
            ))
        });
    let remaining = point.the_remaining_time(
        direction,
        the_requests_channels,
        bytes_remaining,
        the_request_has_ended,
        now,
    );
    (rate, remaining)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::presentation::the_repairing_server::TheThreeReadings;
    use smoozy_state::identities::IdentityKind;
    use smoozy_state::observed::{Clock, Reading};
    use std::sync::Arc;
    use std::time::{Duration, SystemTime};

    fn an_identity(kind: IdentityKind) -> Identity {
        Identity::issue(kind).expect("the desk has randomness")
    }

    fn a_reading(value: &str, at: u64) -> AsOfThatMoment {
        AsOfThatMoment::from(&Reading::taken(
            value.to_string(),
            SystemTime::UNIX_EPOCH + Duration::from_secs(at),
            Clock::ThisComputer,
        ))
    }

    /// Everything one post said, by name.
    fn what_was_said(delivery: &crate::events::Delivery) -> std::collections::BTreeMap<String, String> {
        delivery
            .said
            .iter()
            .map(|(name, said)| {
                let text = match said {
                    Said::Prose(prose) => prose.as_str().to_string(),
                    Said::Value(value) => value.as_str().to_string(),
                };
                ((*name).to_string(), text)
            })
            .collect()
    }

    struct Collects(Arc<Mutex<Vec<crate::events::Delivery>>>);
    impl crate::events::Delivers for Collects {
        fn deliver(&self, delivery: &crate::events::Delivery) {
            self.0
                .lock()
                .expect("the collector")
                .push(delivery.clone());
        }
    }

    fn a_queue() -> (Arc<TheOneQueue>, Arc<Mutex<Vec<crate::events::Delivery>>>) {
        let queue = Arc::new(TheOneQueue::new());
        let held = Arc::new(Mutex::new(Vec::new()));
        queue.the_page_is_ready(Box::new(Collects(Arc::clone(&held))));
        (queue, held)
    }

    /// **M28: the three readings reach the page, each with its own moment.**
    #[test]
    fn a_repairing_server_posts_its_three_readings() {
        let (queue, held) = a_queue();
        let row = an_identity(IdentityKind::LocalServer);
        a_server_is_repairing(
            &queue,
            &row,
            TheThreeReadings {
                the_connection_condition: a_reading("reconnecting", 1_700_000_100),
                the_moment_the_next_attempt_is_due: Some(a_reading("1700000112", 1_700_000_100)),
                the_last_known_work_reading: Some(a_reading("Kitchen — final", 1_700_000_040)),
            },
            true,
        );
        let posted = held.lock().expect("the collector");
        assert_eq!(posted.len(), 1);
        let said = what_was_said(&posted[0]);
        assert_eq!(said["theConnectionCondition"], "reconnecting");
        assert_eq!(said["theNextAttemptIsDueAt"], "1700000112");
        assert_eq!(said["theLastKnownWork"], "Kitchen — final");
        assert_eq!(said["theLastKnownWorkWasTakenAt"], "1700000040");
        assert_eq!(said["theLastKnownWorkClock"], "this computer");
    }

    /// **M31: both of the column's contents are supplied and the page chooses
    /// which occupies the slot.**
    #[test]
    fn both_of_the_columns_contents_go_across() {
        let (queue, held) = a_queue();
        a_server_is_repairing(
            &queue,
            &an_identity(IdentityKind::LocalServer),
            TheThreeReadings {
                the_connection_condition: a_reading("disconnected", 1_700_000_100),
                the_moment_the_next_attempt_is_due: None,
                the_last_known_work_reading: Some(a_reading("Kitchen — final", 1_700_000_040)),
            },
            true,
        );
        let posted = held.lock().expect("the collector");
        let said = what_was_said(&posted[0]);
        assert_eq!(said["theColumnMaySayFirst"], "Rendering");
        assert_eq!(said["theColumnMaySaySecond"], "Reconnecting…");
        // **X7: the job's name travels beside the word and never inside it.**
        assert_eq!(said["theJobsName"], "Kitchen — final");
    }

    /// **M30: where the moment is absent the line has no countdown**, and the
    /// event simply carries none. `Servers` N10a: absent is a value.
    #[test]
    fn an_attempt_in_flight_sends_no_moment_at_all() {
        let (queue, held) = a_queue();
        a_server_is_repairing(
            &queue,
            &an_identity(IdentityKind::LocalServer),
            TheThreeReadings {
                the_connection_condition: a_reading("reconnecting", 1_700_000_100),
                the_moment_the_next_attempt_is_due: None,
                the_last_known_work_reading: None,
            },
            true,
        );
        let posted = held.lock().expect("the collector");
        let said = what_was_said(&posted[0]);
        assert!(!said.contains_key("theNextAttemptIsDueAt"));
        assert_eq!(said["theColumnMaySayFirst"], "Reconnecting…");
    }

    /// **M19 and `Server protocol and transfers` G4: the transfer's report
    /// reaches the page as it is, with its two marks beside the count.**
    #[test]
    fn a_transfers_report_reaches_the_page_with_its_own_moment() {
        let (queue, held) = a_queue();
        let transfer = an_identity(IdentityKind::Transfer);
        let mut reporter = TheTransfersReporter::about(Arc::clone(&queue), transfer);
        reporter.report(&Report {
            bytes_staged: 1_400_000_000,
            bytes_expected: 4_200_000_000,
            started_over: None,
            verifying_a_prefix_of: None,
        });
        let posted = held.lock().expect("the collector");
        assert_eq!(posted.len(), 1);
        let said = what_was_said(&posted[0]);
        assert_eq!(said["staged"], "1400000000");
        assert_eq!(said["expected"], "4200000000");
        assert_eq!(said["onTheClock"], "this computer");
        assert!(!said["takenAt"].is_empty());
        assert_eq!(said["mark"], "on-the-row");
    }

    /// **`Server protocol and transfers` G4a: the count falling to zero is
    /// never left to stand as the whole report**, so the mark travels with it.
    #[test]
    fn a_start_over_carries_its_mark_and_never_only_a_count_of_zero() {
        let (queue, held) = a_queue();
        let transfer = an_identity(IdentityKind::Transfer);
        let mut reporter = TheTransfersReporter::about(Arc::clone(&queue), transfer);
        reporter.report(&Report {
            bytes_staged: 0,
            bytes_expected: 4_200_000_000,
            started_over: Some(StartOverCause::TheStagedPrefixDidNotVerify),
            verifying_a_prefix_of: None,
        });
        let posted = held.lock().expect("the collector");
        let said = what_was_said(&posted[0]);
        assert_eq!(said["staged"], "0");
        // **The line and its cause are two sentences on two strings, both
        // prose.** They travelled as values until 2026-09-12, so neither the
        // orphan rule nor the em-dash rule reached them and the page wrote the
        // *starting again — * half itself (F-6). The non-breaking space is the
        // typography point having been through them.
        assert_eq!(said["startingAgain"], "Restarting from the\u{a0}beginning.");
        assert_eq!(
            said["startedOver"],
            "Could not\u{a0}resume."
        );
    }

    /// **`Server protocol and transfers` G4b: a transfer verifying a staged
    /// prefix says so, and its unmoving count is never the whole report.**
    #[test]
    fn a_verifying_transfer_says_so_beside_its_unmoving_count() {
        let (queue, held) = a_queue();
        let transfer = an_identity(IdentityKind::Transfer);
        let mut reporter = TheTransfersReporter::about(Arc::clone(&queue), transfer);
        reporter.report(&Report {
            bytes_staged: 4_000_000_000,
            bytes_expected: 4_294_967_296,
            started_over: None,
            verifying_a_prefix_of: Some(4_000_000_000),
        });
        let posted = held.lock().expect("the collector");
        let said = what_was_said(&posted[0]);
        assert_eq!(said["verifyingAPrefixOf"], "4000000000");
        assert_eq!(
            said["theCheckingLine"],
            "Verifying the partial\u{a0}file."
        );
    }

    /// **M21: the mark going says nothing about how the work ended.**
    #[test]
    fn the_mark_going_says_nothing_about_the_outcome() {
        let (queue, held) = a_queue();
        let transfer = an_identity(IdentityKind::Transfer);
        queue.the_subject_exists(transfer.clone());
        it_is_no_longer_underway(&queue, &transfer);
        let posted = held.lock().expect("the collector");
        let said = what_was_said(&posted[0]);
        for outcome in ["failed", "succeeded", "ended", "gave up"] {
            for value in said.values() {
                assert!(
                    !value.contains(outcome),
                    "the mark going said {outcome}. **M21 and \
                     `Server protocol and transfers` G9: a waiter that gave up \
                     says the waiter gave up, and the work carries on being \
                     whatever the record and the journal say it is**"
                );
            }
        }
    }

    /// **The repairing server's Connect control carries a word whichever of
    /// the two the event carries** — the Gemini catch-up of step 10a, Area 4
    /// (F-3).
    ///
    /// With no last-known work the event sends one content only, as
    /// `theColumnMaySayFirst`. Before 2026-09-19 the control read only the
    /// second and was drawn as an amber dot with no label, where the drawing
    /// reads *Reconnecting…*. Nothing drives the page in a test, so this reads
    /// it as text, which is what this crate's other page checks do.
    #[test]
    fn the_repairing_servers_connect_control_falls_back_to_the_first_word() {
        let page = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("web")
                .join("the-underway-shape.js"),
        )
        .expect("the page module is readable");
        let control = page
            .split_once("const connect = document.createElement(\"button\");")
            .expect("the repairing server draws a Connect control")
            .1;
        let control = &control[..control.find("acts.appendChild(connect)").expect("it is placed")];
        assert!(
            control.contains("parts.theColumnMaySaySecond || parts.theColumnMaySayFirst"),
            "the Connect control reads only the second word, so a repairing \
             server with no last-known work draws it with no label"
        );
    }
}
