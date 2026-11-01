//! **Anything that isn't instant shows that it is underway** — §16.5 M18 to
//! M22, and `Preparation/Design/Whole app.md`'s *Anything that isn't instant
//! shows that it's underway*.
//!
//! **Read the folder's head first.** This file draws inside the shape stated
//! there and adds nothing to it.
//!
//! ## The three shapes, and what decides which
//!
//! Design: *"There are three shapes, and what the user can still do decides
//! which. That is the whole rule, and it is meant to be applied without
//! judgement."* On the control, on the row, over the pane — and **where two
//! look possible, the narrowest that is true wins**, because a shape that
//! covers more than it has to takes away work the user could have been doing.
//! [`WhatTheUserCanStillDo`] is that rule performed rather than remembered:
//! the caller says what is still usable and [`the_shape`] answers, and there is
//! no route by which a caller supplies a shape of its own.
//!
//! ## The two rules that are easy to lose, and both are Design's own
//!
//! - **The control keeps its place and its width, loses its label to a turning
//!   mark, and is not dimmed.** Added at Design on 2026-09-02, *"after the
//!   drawing turned out to have two of these, one faded and one not"*. **Not
//!   being pressable is carried by the control not answering**; a fade on top
//!   of that says *unavailable* twice, and on a filled button it washes the
//!   turning mark out to almost nothing. The place and the width are kept by
//!   the label staying in the markup, and the page's own stylesheet is where
//!   both halves are held — `web/the-underway-shape.css`.
//! - **One thing never carries two underway marks at once.** *"A server Smoozy
//!   is repairing already says so on its row, so its Connect control keeps its
//!   amber — which is state rather than an underway mark — and what it does not
//!   additionally do is lose its label or spin."* That is a different rule from
//!   the narrowest-true one: the first settles **which** shape, this one stops
//!   a single thing wearing **two**. It is kept structurally by
//!   [`TheUnderwayMark`] being one value rather than a set, and by
//!   [`the_mark_for`] answering nothing at all for a thing that already carries
//!   its own state.
//!
//! ## Where the mark comes from, and it is not a response
//!
//! **M18: the indicator can appear at the instant of the click, and that is a
//! property of the protocol rather than of the screen.** A request is recorded
//! locally before it is sent (`Server protocol and transfers` C3) and
//! acknowledged as recorded before its work completes
//! (`Server protocol and transfers` C7), **so the presentation shows *underway*
//! from the local write, before anything has left the computer.**
//!
//! [`WhatSaysItIsUnderway`] is that written down as a closed set whose **first**
//! member is the local write, and there is no member meaning *a response came
//! back*. A drawing that waited for one would have nothing to draw at the
//! moment of the click, which is the whole of what Row 291 is about.
//!
//! ## The three reporting routes are read and not invented
//!
//! **M19: the three reporting routes are `Server protocol and transfers` G4's
//! and this document reads them rather than inventing a fourth.** They are
//! built, at `smoozy_server_protocol::transfers::Report` for the transfer and
//! at that crate's framing half for the other two. [`HowItReports`] is the same
//! three and [`tests::the_reporting_routes_are_three_and_they_are_g4s`] is what
//! says there is no fourth.
//!
//! ## What may never be done here
//!
//! - **Never derive progress from the far end's view of a file.**
//!   M20, and `Server protocol and transfers` G5. [`HowFarAlong`] has no field
//!   a remote size could travel in, and the count that fills it is Smoozy's own
//!   at Smoozy's own loop.
//! - **Never present an expired deadline as the work ending, and never present
//!   a stalled channel as a broken connection.** M21, and
//!   `Server protocol and transfers` G9 and G8. Both are kept by there being no
//!   value in this file that says either thing — see
//!   [`WhatTheWaiterCanSay`] and [`WhatTheChannelCanSay`].
//! - **Never detect a hang, and never make one undetectable later.** M22. The
//!   first half is an absence; **the second half binds what is built**, and it
//!   is why every reading here is an [`super::as_of_that_moment::AsOfThatMoment`]
//!   carrying its own moment: a stalled channel is told from a moving one by
//!   its last progress moment and by nothing else.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** None: every marker of another document below is
//! written out with that document at the site.
//!
//! **That is deliberate rather than lazy.** This plan defines a C-series, a
//! G-series, an R-series and an S-series of its own, and
//! `Server protocol and transfers` — which supplies nearly every clause this
//! file reads — defines all four as well. A header list cannot tell two clauses
//! of the same short name apart, so a bare one would resolve to a real and
//! wrong clause.

use crate::presentation::as_of_that_moment::AsOfThatMoment;

/// **What the user can still do, which is the whole of what decides the shape.**
///
/// Design: *"There are three shapes, and what the user can still do decides
/// which. That is the whole rule, and it is meant to be applied without
/// judgement."*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheUserCanStillDo {
    /// *"the control the user pressed is the thing that is working and
    /// everything else on the screen still works"* — *Connect*, *Read again*,
    /// *Check dependencies…*.
    EverythingElseOnTheScreen,
    /// *"the work belongs to one row of a list and the user will come back to
    /// it"* — a transfer, a queue entry, a setup step.
    EverythingButThatOneRow,
    /// *"nothing on that pane can be used until the work finishes"* — a file
    /// listing being read, a dependency search running. **Nothing else on the
    /// screen is covered.**
    NothingOnThatPane,
}

/// **The three shapes, and there is no fourth.**
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum TheUnderwayMark {
    /// **On the control.** *"The control keeps its place and its width, loses
    /// its label to a turning mark, and cannot be pressed until it finishes …
    /// It is not dimmed."*
    OnTheControl,
    /// **On the row.** *"The row says how far along in its own status
    /// column."*
    OnTheRow,
    /// **Over the pane.**
    OverThePane,
}

impl TheUnderwayMark {
    /// The written form the page draws on. **A value and never prose** — X9.
    pub fn as_text(self) -> &'static str {
        match self {
            TheUnderwayMark::OnTheControl => "on-the-control",
            TheUnderwayMark::OnTheRow => "on-the-row",
            TheUnderwayMark::OverThePane => "over-the-pane",
        }
    }
}

/// **Design's rule performed: the narrowest that is true wins.**
///
/// *"Where two look possible, the narrowest that is true wins, because a shape
/// that covers more than it has to takes away work the user could have been
/// doing."* The narrowing is in the order of [`WhatTheUserCanStillDo`]'s own
/// three values, so there is nothing here to get wrong and no judgement to
/// exercise.
pub fn the_shape(what_is_still_usable: WhatTheUserCanStillDo) -> TheUnderwayMark {
    match what_is_still_usable {
        WhatTheUserCanStillDo::EverythingElseOnTheScreen => TheUnderwayMark::OnTheControl,
        WhatTheUserCanStillDo::EverythingButThatOneRow => TheUnderwayMark::OnTheRow,
        WhatTheUserCanStillDo::NothingOnThatPane => TheUnderwayMark::OverThePane,
    }
}

/// **What the thing already carries, which is the other half of the rule.**
///
/// Design: *"One thing never carries two of them. A server Smoozy is repairing
/// already says so on its row, so its Connect control keeps its amber — which
/// is state rather than an underway mark — and what it does not additionally do
/// is lose its label or spin."*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatItAlreadyCarries {
    /// The ordinary case: nothing, so the mark is the thing's first.
    Nothing,
    /// **Its own state**, which is not an underway mark and is not replaced by
    /// one. The repairing server's amber *Reconnecting…* control is the
    /// standing instance and is §16.7's.
    ItsOwnState,
    /// **An underway mark already**, from work that started earlier.
    AnUnderwayMark,
}

/// **The mark for one thing, or nothing at all.**
///
/// **`None` is an answer and not a gap.** A thing that already carries its own
/// state, or a mark from earlier work, takes no second one — which is how *one
/// thing never carries two of them* is a property of what this function can
/// return rather than a rule somebody applies at eleven screens.
pub fn the_mark_for(
    what_is_still_usable: WhatTheUserCanStillDo,
    already: WhatItAlreadyCarries,
) -> Option<TheUnderwayMark> {
    match already {
        WhatItAlreadyCarries::Nothing => Some(the_shape(what_is_still_usable)),
        WhatItAlreadyCarries::ItsOwnState | WhatItAlreadyCarries::AnUnderwayMark => None,
    }
}

/// **What says a thing is underway — M18's closed set, in the order the answers
/// arrive.**
///
/// **The first member is the local write and there is no member meaning *the
/// far end answered*.** M18: *"A request is recorded locally before it is sent
/// and acknowledged as recorded before its work completes, so *it has started*
/// and *it has finished* are two separate correlated answers … the presentation
/// shows *underway* from the local write, before anything has left the
/// computer."*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatSaysItIsUnderway {
    /// **`Server protocol and transfers` C3: the request is recorded locally
    /// before it is sent.** This is the earliest of the three and it is the one
    /// the indicator appears on.
    TheRequestWasRecordedLocally,
    /// **`Server protocol and transfers` C7: acknowledged as recorded before
    /// its work completes.** An acknowledgement says the request is recorded
    /// and never that the work is done.
    TheMachineHasItRecorded,
    /// The work's own report, by one of [`HowItReports`]' three routes.
    ItHasReported,
}

impl WhatSaysItIsUnderway {
    /// **M18: shown from the local write.** Every member says *underway*, and
    /// the point of the set is that the **first** one already does.
    pub fn shows_underway(self) -> bool {
        true
    }
}

/// **M19's three reporting routes — `Server protocol and transfers` G4's, read
/// rather than invented.**
///
/// *"Three routes, and each kind of work in this document's scope uses one of
/// them."* **A fourth is a fault in that section rather than a case it does not
/// cover**, so this set is closed here too.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HowItReports {
    /// **A transfer** — bytes staged against bytes expected, counted at the
    /// byte loop's own accounting interval, **bounded above in time as well as
    /// in bytes** so the gap between reports is bounded.
    /// `smoozy_server_protocol::transfers::Report` is the built shape.
    ATransfer,
    /// **A request the machine took as an operation** — the event journal,
    /// read forward from the caller's own durable cursor.
    AnOperationInTheJournal,
    /// **A reading** — a listing, an attribute read, a free-space reading, a
    /// hardware reading, package inspection, a digest over a byte range: its
    /// own result and the fact that it was submitted. *"A reading that cannot
    /// report progress inside itself is bounded by its waiter's deadline
    /// instead."*
    AReading,
}

impl HowItReports {
    /// **The whole set, and it is three.** Named here so a drawing reads the
    /// routes rather than composing one.
    pub const THE_THREE: [HowItReports; 3] = [
        HowItReports::ATransfer,
        HowItReports::AnOperationInTheJournal,
        HowItReports::AReading,
    ];
}

/// **How far along, where the thing can say.**
///
/// Design: *"Anything that shows it is underway says how far along where it
/// can, and a shape that never becomes a figure is the app saying it cannot.
/// That shape is itself the admission and no words are owed anywhere for it."*
/// So **[`HowFarAlong::ItCannotSay`] is an answer**, it needs no sentence
/// beside it, and what the rule forbids is showing nothing at all while work is
/// happening.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum HowFarAlong {
    /// *Connect* is the standing instance: nothing could say how far along it
    /// is.
    ItCannotSay,
    /// **Smoozy's own count at Smoozy's own loop** — never the far end's view
    /// of a file.
    Of(TheProgressReading),
}

/// **One progress reading, carried with its own moment.**
///
/// **M20 and `Server protocol and transfers` G5: progress is never derived from
/// the far end's own view of the file.** There is no field here a remote size
/// could travel in: what fills these two is
/// `smoozy_server_protocol::transfers::Report`'s own count, which the byte loop
/// adds to as it writes.
///
/// **M22's second half is why the moment is not optional.** *"Nothing here
/// detects a hang and nothing here makes one undetectable later … a stalled
/// channel is distinguishable from a moving one by its last progress
/// moment."* A reading drawn without one would take that away, quietly, and
/// nothing would report it — which is also M3a's general rule.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheProgressReading {
    /// Bytes staged, as a value on the verbatim route (X9), **with the moment
    /// the count was taken** (M3a).
    pub staged: AsOfThatMoment,
    /// Bytes expected. Durable before the first byte
    /// (`Server protocol and transfers` S8), so a whole exists to be read from
    /// the first moment.
    pub expected: AsOfThatMoment,
}

/// **M20's second half: a batch reports both levels.**
///
/// `Server protocol and transfers` G6: *"The per-transfer count of G4 and the
/// enumeration of S9 together give the detail and the whole, which is the
/// primitive Rows 190 and 299 are written against."* **Both are supplied and
/// neither is composed from the other** — a batch drawn from the per-transfer
/// counts alone would have to guess the whole, and one drawn from the
/// enumeration alone would have nothing to say about the transfer in front of
/// the user.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ABatchsProgress {
    /// The detail: one reading per transfer, **each on its own**.
    pub each_transfer: Vec<TheProgressReading>,
    /// The whole: the enumeration's own count.
    pub the_enumeration: TheProgressReading,
}

/// **M21's first half: a waiter's expired deadline is the waiter giving up.**
///
/// `Server protocol and transfers` G9: *"A waiter that gave up says the waiter
/// gave up, and the transfer or operation carries on being whatever the record
/// and the journal say it is."* **There is no member of this enumeration
/// meaning the work ended**, which is what stops the collapse rather than a
/// rule asking somebody not to make it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum WhatTheWaiterCanSay {
    /// The waiter's own deadline expired — `Server protocol and transfers` C17.
    TheWaiterGaveUp,
}

/// **M21's second half: a stalled channel is a stalled channel.**
///
/// `Server protocol and transfers` G8: *"A stalled channel is not a broken
/// connection and neither is inferred from the other."* The reason is measured:
/// a caller's own line pulled went undetected for **21 minutes 45 seconds** with
/// keepalives off, while a machine closing politely and a provider's own stop
/// were both announced in the same second.
///
/// **The connection's own condition comes from `Servers` and never from here.**
/// There is no member of this enumeration about a connection, and no function in
/// this file takes one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum WhatTheChannelCanSay {
    /// It is moving.
    ItIsMoving,
    /// It has not moved since its last progress moment — and that is all it
    /// says.
    ItIsStalled,
}

/// **One thing that is underway, as the page receives it.**
///
/// **Composed here and never on the page**, the same discipline
/// [`super::the_failure_shape::Drawn`] keeps.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct DrawnUnderway {
    /// Which of the three shapes — [`TheUnderwayMark::as_text`].
    pub mark: &'static str,
    /// **What the thing is, by the verbatim route** (X9): a control's act
    /// token, a row's identity, a pane's name. Never joined into a sentence,
    /// which is the folder head's rule 4.
    pub what_it_is_on: crate::typography::Verbatim,
    /// How far along, where it can say.
    pub how_far_along: HowFarAlong,
    /// Whether the channel behind it is moving. **Never a statement about a
    /// connection** — M21.
    pub the_channel: WhatTheChannelCanSay,
}

/// **The one way a drawing composes an underway mark.**
///
/// A caller says what the user can still do, what the thing already carries,
/// and how far along it is. **It does not choose a shape**, and there is no
/// parameter through which it could.
///
/// **`None` where the thing already carries something** — Design's *one thing
/// never carries two of them*.
pub fn underway(
    what_is_still_usable: WhatTheUserCanStillDo,
    already: WhatItAlreadyCarries,
    what_it_is_on: &str,
    how_far_along: HowFarAlong,
    the_channel: WhatTheChannelCanSay,
) -> Option<DrawnUnderway> {
    let mark = the_mark_for(what_is_still_usable, already)?;
    Some(DrawnUnderway {
        mark: mark.as_text(),
        what_it_is_on: crate::typography::Verbatim::new(what_it_is_on.to_string()),
        how_far_along,
        the_channel,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::typography::Verbatim;
    use smoozy_state::observed::{Clock, Reading};
    use std::time::{Duration, SystemTime};

    /// This file's working half — everything before the tests.
    ///
    /// **The sweeps below read it rather than the whole file**, for the same
    /// reason the folder's own checks read past comment lines: a check that
    /// could not tell a prohibition from an instance would be one nobody could
    /// write the prohibition beside, and the forbidden words are written out
    /// here in order to forbid them.
    fn the_working_half() -> &'static str {
        include_str!("the_underway_shape.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("the file has a working half")
    }

    fn a_reading(value: &str, at: u64) -> AsOfThatMoment {
        AsOfThatMoment::from(&Reading::taken(
            value.to_string(),
            SystemTime::UNIX_EPOCH + Duration::from_secs(at),
            Clock::ThisComputer,
        ))
    }

    /// **M18: the indicator can appear at the instant of the click**, because
    /// the earliest thing that says *underway* is the local write and not an
    /// answer from anywhere.
    #[test]
    fn underway_is_shown_from_the_local_write() {
        assert!(WhatSaysItIsUnderway::TheRequestWasRecordedLocally.shows_underway());
    }

    /// **M18, from the other side: there is no member of the set meaning *the
    /// far end answered*.**
    ///
    /// The set is read out of the source rather than out of a list kept here,
    /// so a member added later is met by this test rather than by nobody.
    #[test]
    fn nothing_in_the_set_waits_for_a_response() {
        let source = the_working_half();
        let opens = source
            .find("pub enum WhatSaysItIsUnderway {")
            .expect("the set is an enum");
        let body = &source[opens..opens + source[opens..].find('}').expect("it closes")];
        for waiting in ["Response", "Answered", "TheWorkFinished", "Completed"] {
            assert!(
                !body.contains(waiting),
                "`WhatSaysItIsUnderway` names {waiting}. **M18: the presentation \
                 shows underway from the local write, before anything has left \
                 the computer** — a member meaning the far end answered would \
                 put the indicator back after the round trip, which is exactly \
                 what Row 291 exists to forbid"
            );
        }
    }

    /// **M19: the three reporting routes are `Server protocol and transfers`
    /// G4's and no fourth is invented.**
    #[test]
    fn the_reporting_routes_are_three_and_they_are_g4s() {
        assert_eq!(HowItReports::THE_THREE.len(), 3);
        // The three are distinct, so the constant is not three copies of one.
        for route in HowItReports::THE_THREE {
            assert_eq!(
                HowItReports::THE_THREE
                    .iter()
                    .filter(|other| **other == route)
                    .count(),
                1
            );
        }
        // **And the enumeration itself declares three and not four.** The
        // members are read out of the source, so a fourth added later fails
        // here rather than passing quietly with the constant left behind.
        let source = the_working_half();
        let opens = source
            .find("pub enum HowItReports {")
            .expect("the routes are an enum");
        let body = &source[opens..opens + source[opens..].find("\n}").expect("it closes")];
        let members = body
            .lines()
            .filter(|line| line.trim_start().starts_with("ATransfer")
                || line.trim_start().starts_with("AnOperationInTheJournal")
                || line.trim_start().starts_with("AReading"))
            .count();
        assert_eq!(
            members, 3,
            "**M19: the three reporting routes are \
             `Server protocol and transfers` G4's and this document reads them \
             rather than inventing a fourth.** A fourth route is a fault in that \
             section rather than a case it does not cover"
        );
    }

    /// **Design's narrowing rule, applied without judgement.**
    #[test]
    fn the_narrowest_shape_that_is_true_wins() {
        assert_eq!(
            the_shape(WhatTheUserCanStillDo::EverythingElseOnTheScreen),
            TheUnderwayMark::OnTheControl
        );
        assert_eq!(
            the_shape(WhatTheUserCanStillDo::EverythingButThatOneRow),
            TheUnderwayMark::OnTheRow
        );
        assert_eq!(
            the_shape(WhatTheUserCanStillDo::NothingOnThatPane),
            TheUnderwayMark::OverThePane
        );
    }

    /// **Design: one thing never carries two underway marks at once.** The
    /// repairing server's Connect control is the standing instance — it keeps
    /// its amber, which is state, and does not additionally lose its label or
    /// spin.
    #[test]
    fn a_thing_that_already_carries_its_own_state_takes_no_mark() {
        assert!(the_mark_for(
            WhatTheUserCanStillDo::EverythingElseOnTheScreen,
            WhatItAlreadyCarries::ItsOwnState
        )
        .is_none());
        assert!(the_mark_for(
            WhatTheUserCanStillDo::EverythingElseOnTheScreen,
            WhatItAlreadyCarries::AnUnderwayMark
        )
        .is_none());
        assert!(the_mark_for(
            WhatTheUserCanStillDo::EverythingElseOnTheScreen,
            WhatItAlreadyCarries::Nothing
        )
        .is_some());
    }

    /// **Design: a shape that never becomes a figure is the app saying it
    /// cannot**, and that shape is itself the admission — so *underway* with no
    /// figure is still drawn and owes no words.
    #[test]
    fn a_thing_that_cannot_say_how_far_along_is_still_drawn() {
        let drawn = underway(
            WhatTheUserCanStillDo::EverythingElseOnTheScreen,
            WhatItAlreadyCarries::Nothing,
            "servers.connect",
            HowFarAlong::ItCannotSay,
            WhatTheChannelCanSay::ItIsMoving,
        )
        .expect("nothing else is carried");
        assert_eq!(drawn.mark, "on-the-control");
        assert_eq!(drawn.how_far_along, HowFarAlong::ItCannotSay);
    }

    /// **M20 and `Server protocol and transfers` G5: progress is Smoozy's own
    /// count and there is nowhere here a far end's view of a file could go.**
    #[test]
    fn nothing_in_a_progress_reading_could_come_from_the_far_end() {
        let source = the_working_half();
        let opens = source
            .find("pub struct TheProgressReading {")
            .expect("the reading is a struct");
        let body = &source[opens..opens + source[opens..].find('}').expect("it closes")];
        for remote in ["remote", "far_end", "the_files_size", "asked_the_server"] {
            assert!(
                !body.contains(remote),
                "`TheProgressReading` names {remote}. \
                 **`Server protocol and transfers` G5: progress is never derived \
                 from the far end's own view of the file** — a staged file's \
                 size is not a statement of progress, and a zero-byte one is \
                 indistinguishable between still being written, the disk \
                 filled, and the render wrote nothing"
            );
        }
    }

    /// **M20's second half: a batch reports both levels and neither is composed
    /// from the other.**
    #[test]
    fn a_batch_reports_the_detail_and_the_whole() {
        let batch = ABatchsProgress {
            each_transfer: vec![
                TheProgressReading {
                    staged: a_reading("1400000000", 10),
                    expected: a_reading("4200000000", 10),
                },
                TheProgressReading {
                    staged: a_reading("0", 10),
                    expected: a_reading("900000000", 10),
                },
            ],
            the_enumeration: TheProgressReading {
                staged: a_reading("1", 10),
                expected: a_reading("2", 10),
            },
        };
        assert_eq!(batch.each_transfer.len(), 2);
        assert_eq!(batch.the_enumeration.expected.value.as_str(), "2");
    }

    /// **M21: an expired deadline is the waiter giving up and never the work
    /// ending.** The enumeration has one member and it says only that.
    #[test]
    fn an_expired_deadline_says_only_that_the_waiter_gave_up() {
        let source = the_working_half();
        let opens = source
            .find("pub enum WhatTheWaiterCanSay {")
            .expect("it is an enum");
        let body = &source[opens..opens + source[opens..].find('}').expect("it closes")];
        for ending in ["TheWorkEnded", "TheWorkFailed", "Ended", "Failed"] {
            assert!(
                !body.contains(ending),
                "`WhatTheWaiterCanSay` names {ending}. \
                 **`Server protocol and transfers` G9: a waiter that gave up \
                 says the waiter gave up, and the transfer or operation carries \
                 on being whatever the record and the journal say it is**"
            );
        }
        assert_eq!(WhatTheWaiterCanSay::TheWaiterGaveUp as u8, 0);
    }

    /// **M21: a stalled channel is a stalled channel and never a broken
    /// connection.** Nothing in this file names a connection at all, which is
    /// what makes the inference unavailable rather than discouraged.
    #[test]
    fn a_stalled_channel_is_never_a_broken_connection() {
        let source = the_working_half();
        let opens = source
            .find("pub enum WhatTheChannelCanSay {")
            .expect("it is an enum");
        let body = &source[opens..opens + source[opens..].find('}').expect("it closes")];
        for connection in ["Connection", "Disconnected", "TheLineWent"] {
            assert!(
                !body.contains(connection),
                "`WhatTheChannelCanSay` names {connection}. \
                 **`Server protocol and transfers` G8: a stalled channel is not \
                 a broken connection and neither is inferred from the other** — \
                 a caller's own line pulled went undetected for 21 minutes 45 \
                 seconds with keepalives off. The connection's observed value \
                 comes from `Servers`"
            );
        }
    }

    /// **M22's second half, which is the one that binds what is built: nothing
    /// here makes a hang undetectable later.**
    ///
    /// Every progress reading carries its own moment, so a stalled channel is
    /// told from a moving one by its last progress moment — and there is no
    /// constructor that leaves the moment out.
    #[test]
    fn every_progress_reading_carries_its_last_progress_moment() {
        let reading = TheProgressReading {
            staged: a_reading("17233920", 1_700_000_000),
            expected: a_reading("4294967296", 1_700_000_000),
        };
        assert_eq!(reading.staged.taken_at, Some(1_700_000_000));
        assert_eq!(reading.staged.on_the_clock, "this computer");
    }

    /// **X9: what the mark is on is a value and takes the verbatim route.** A
    /// control's act token, a row's identity or a pane's name is never prose and
    /// is never joined into a sentence.
    #[test]
    fn what_the_mark_is_on_is_never_touched_by_the_typography_point() {
        let drawn = underway(
            WhatTheUserCanStillDo::EverythingButThatOneRow,
            WhatItAlreadyCarries::Nothing,
            "/Volumes/Work — renders/final one",
            HowFarAlong::ItCannotSay,
            WhatTheChannelCanSay::ItIsMoving,
        )
        .expect("nothing else is carried");
        assert_eq!(
            drawn.what_it_is_on,
            Verbatim::new("/Volumes/Work — renders/final one")
        );
    }

    /// **The running setup step's mark stands clear of the step's last
    /// letter, by at least the drawing's own gap** — `.stm`'s left margin, the
    /// gap the drawing puts between a step's words and what follows them on its
    /// line. Step 10c's tenth pass on Hyperstack saw the mark touch the letter
    /// (*driver◌*); the page harness measured no gap before and 8 pixels
    /// after. Step 10c's tenth-pass fix piece 6.
    #[test]
    fn a_running_steps_mark_stands_clear_of_its_last_letter() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let pixels_after = |sheet: &str, rule: &str| -> f64 {
            let at = sheet.find(rule).unwrap_or_else(|| panic!("{rule} is not in the sheet"));
            let body = &sheet[at..at + sheet[at..].find('}').expect("it closes")];
            let at = body.find("margin-left:").unwrap_or_else(|| panic!("{rule} sets no left margin"));
            let value = body[at + "margin-left:".len()..].trim_start();
            value[..value.find("px").expect("in pixels")].trim().parse().expect("a number")
        };
        let drawing = std::fs::read_to_string(root.join("web/the-drawing.css")).expect("the drawing is readable");
        let shape = std::fs::read_to_string(root.join("web/the-underway-shape.css")).expect("the shape is readable");
        let the_drawings = pixels_after(&drawing, "\n.stm {");
        let the_steps = pixels_after(&shape, "\n.steps li > .underway-row {");
        assert!(the_steps >= the_drawings, "the mark stands {the_steps}px clear, the drawing {the_drawings}px");
    }
}
