//! §10 — nothing is carried out with nothing to report until it is done.
//!
//! **What this is.** The floor §10.1 puts on every request this document
//! defines, and the two clauses of §10.3 that say what a waiter's own deadline
//! does and does not change. **Both ends speak it**, which is why it sits in
//! this half of the module: the machine owes the acknowledgement half, and the
//! desktop owes the local record half and the reading of what comes back.
//!
//! # Why a floor is a thing to build rather than a thing to say
//!
//! **G1 is written as a requirement and not as a wish** — *"A request with none
//! of the three is a fault in this section rather than a case it does not
//! cover"* — and Row 291 is what forbids designing the alternative. So the three
//! routes are a value a request carries, and [`the_floor_is_met`] is the test
//! that decides whether a request keeps the floor. **A request that keeps none
//! of the three is a fault**, and it is named as one rather than answered.
//!
//! **G2 is what makes the floor more than a restatement.** Because C7's
//! acknowledgement says the request is recorded and never that the work is done,
//! *it has started* and *it has finished* are **two separate correlated
//! answers** rather than one. **A protocol in which the only answer is the
//! finished one is a protocol in which every noticeable moment is silent.**
//!
//! # What is deliberately absent
//!
//! **The indicator.** Row 291's own drawing is Build's, where it sits is
//! Design's, and `Desktop app and delivery` presents it. **Nothing here draws
//! anything**; what is owed here is that no piece of work in this document's
//! scope is carried out in a way that has nothing to report until it is done.
//!
//! **G4's three routes are named and not performed here.** The transfer's own
//! count is `crate`'s neighbour half, the event journal is
//! `State, identity and recovery` §4's — **its rules at `smoozy_state::journal`
//! and its store at `smoozy_runtime::acting::journal`, which a supervisor makes
//! at every start** *(corrected 2026-09-07: this said the journal was unbuilt,
//! which stopped being true when it landed)* — and a reading's own
//! result is the reading's. **This file says which route a request is held to
//! and never carries one.**
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `State, identity and recovery` §4.
//! `Desktop app and delivery` — the presenting.

/// **G4's three routes, and each kind of work in this document's scope uses one
/// of them.**
///
/// **This names the route rather than carrying it.** What a transfer counts,
/// what the journal holds and what a reading answers are each settled where
/// they are performed; what §10 owes is that every request is held to one of the
/// three and that none is held to nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhichRouteItsProgressTakes {
    /// **A transfer** — bytes staged against bytes expected, counted at the byte
    /// loop's own accounting interval and **never derived from asking the far
    /// end for a file's size** (G5). The interval is bounded above by R6's fixed
    /// short period, in time and not only in bytes, so the gap between reports
    /// is bounded.
    ItsOwnByteCount,
    /// **A request the machine took as an operation** — the event journal
    /// (`State, identity and recovery` §4), read forward from the caller's own
    /// durable cursor.
    TheEventJournal,
    /// **A reading** — a listing, an attribute read, a free-space reading, a
    /// hardware reading, package inspection, a digest over a byte range: its own
    /// result, and the fact that it was submitted. **A reading that cannot
    /// report progress inside itself is bounded by its waiter's deadline
    /// instead**, which is G3.
    ItsOwnResultAndItsWaitersDeadline,
}

/// **What one request has, of the three things G1's floor asks for.**
///
/// The three are not alternatives a caller picks between: **a request has all
/// three in the ordinary case** — recorded locally before it is sent,
/// acknowledged as recorded before its work completes, and its progress on one
/// of G4's routes. What the floor forbids is a request with **none** of them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhatARequestReportsBeforeItsResult {
    /// **C3.** The caller wrote its pending-command record durably before the
    /// request left. *"A request that has been sent and has no local record is a
    /// request whose answer cannot be found."*
    pub it_was_recorded_locally_before_it_was_sent: bool,
    /// **C7.** The machine acknowledged it as recorded before its work
    /// completed. **This is the one that makes the floor more than a
    /// restatement** — see G2 at the head of this file.
    pub it_was_acknowledged_as_recorded: bool,
    /// **G4.** Which of the three routes its work's own progress reaches the
    /// caller by, or none.
    pub its_progress_route: Option<WhichRouteItsProgressTakes>,
}

impl WhatARequestReportsBeforeItsResult {
    /// **G1's floor: every request has at least one report before its result.**
    ///
    /// **This is the floor rather than a wish**, and a request with none of the
    /// three does not fail this test quietly — [`the_floor_is_met`] returns the
    /// fault so a caller has something to say rather than a boolean to ignore.
    pub fn how_many_of_the_three(&self) -> usize {
        usize::from(self.it_was_recorded_locally_before_it_was_sent)
            + usize::from(self.it_was_acknowledged_as_recorded)
            + usize::from(self.its_progress_route.is_some())
    }
}

/// **What G1 answers about one request.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhetherTheFloorIsMet {
    /// At least one of the three, which is the floor.
    ItIsMet,
    /// **None of the three. G1 calls this a fault in §10 rather than a case §10
    /// does not cover**, so it is an answer with a name and not a `false`.
    ItIsAFaultInSectionTen,
}

/// **G1.**
pub fn the_floor_is_met(request: &WhatARequestReportsBeforeItsResult) -> WhetherTheFloorIsMet {
    if request.how_many_of_the_three() == 0 {
        return WhetherTheFloorIsMet::ItIsAFaultInSectionTen;
    }
    WhetherTheFloorIsMet::ItIsMet
}

/// **G2. C7 is what makes G1 more than a restatement.**
///
/// *It has started* and *it has finished* are **two separate correlated
/// answers** rather than one, and the separation is C7's: an acknowledgement
/// says the request is recorded and never that the work is done, so a caller
/// that has one still has a second answer coming.
///
/// **What this rules out, in code rather than in prose**: a request whose only
/// report is its own result keeps none of the three, because a result is not one
/// of them. [`the_floor_is_met`] calls that a fault, which is the protocol
/// G2 forbids designing being refused rather than described.
pub fn a_request_whose_only_answer_is_the_finished_one() -> WhatARequestReportsBeforeItsResult {
    WhatARequestReportsBeforeItsResult {
        it_was_recorded_locally_before_it_was_sent: false,
        it_was_acknowledged_as_recorded: false,
        its_progress_route: None,
    }
}

/// **G3. Nothing waits silently for a state transition.**
///
/// **Every waiter has its own deadline and expiry is reported** (C16, C17). So
/// **the longest a caller can go with nothing to say is its own deadline**, and
/// that is a reportable event rather than a hang.
///
/// **The deadline is the waiter's own and is not carried here.** C16 makes it
/// volatile and per-waiter — two waiters on one operation have two deadlines,
/// either of which may expire while the other does not — so what this file holds
/// is the rule that an expiry is an event, never a figure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatAWaitersExpiryIs {
    /// **A reportable event**, which is the whole of G3's answer to a hang.
    AReportableEvent,
}

/// **G3.**
pub fn what_a_waiters_expiry_is() -> WhatAWaitersExpiryIs {
    WhatAWaitersExpiryIs::AReportableEvent
}

/// **G9. A waiter's expired deadline never changes what is reported about the
/// work.**
///
/// C17. **A waiter that gave up says the waiter gave up**, and the transfer or
/// operation carries on being whatever the record and the journal say it is.
///
/// **The rule is kept by what this function takes rather than by what it
/// does**: it takes what the record and the journal say, and there is no
/// parameter for a waiter's expiry — so there is no route through it by which an
/// expiry could reach the answer. A caller that wanted to fold one in would have
/// to change the shape, which is the point.
pub fn what_is_reported_about_the_work(
    what_the_record_and_the_journal_say: &str,
) -> String {
    what_the_record_and_the_journal_say.to_string()
}

/// **G9's other half, said plainly: an expiry is a statement about the
/// waiter.**
///
/// C17 already gives it: the expiry *"does not write an outcome, does not write
/// a desired state, does not cancel, and does not count as an attempt"*. **What
/// G9 adds is the reporting side of the same rule** — the work goes on being
/// reported as whatever it is.
pub const A_WAITER_THAT_GAVE_UP_SAYS_THE_WAITER_GAVE_UP: &str =
    "a waiter's expiry is a statement about the waiter; the work carries on being whatever the \
     record and the journal say it is";

#[cfg(test)]
mod tests {
    use super::*;

    fn an_ordinary_request() -> WhatARequestReportsBeforeItsResult {
        WhatARequestReportsBeforeItsResult {
            it_was_recorded_locally_before_it_was_sent: true,
            it_was_acknowledged_as_recorded: true,
            its_progress_route: Some(WhichRouteItsProgressTakes::TheEventJournal),
        }
    }

    /// **G1: the ordinary case has all three, and the floor is one.**
    #[test]
    fn a_request_with_all_three_meets_the_floor_and_so_does_one_with_any_one_of_them() {
        assert_eq!(an_ordinary_request().how_many_of_the_three(), 3);
        assert_eq!(the_floor_is_met(&an_ordinary_request()), WhetherTheFloorIsMet::ItIsMet);

        for one in [
            WhatARequestReportsBeforeItsResult {
                it_was_recorded_locally_before_it_was_sent: true,
                it_was_acknowledged_as_recorded: false,
                its_progress_route: None,
            },
            WhatARequestReportsBeforeItsResult {
                it_was_recorded_locally_before_it_was_sent: false,
                it_was_acknowledged_as_recorded: true,
                its_progress_route: None,
            },
            WhatARequestReportsBeforeItsResult {
                it_was_recorded_locally_before_it_was_sent: false,
                it_was_acknowledged_as_recorded: false,
                its_progress_route: Some(WhichRouteItsProgressTakes::ItsOwnByteCount),
            },
        ] {
            assert_eq!(one.how_many_of_the_three(), 1);
            assert_eq!(the_floor_is_met(&one), WhetherTheFloorIsMet::ItIsMet);
        }
    }

    /// **G1 and G2 together: a request whose only answer is the finished one
    /// keeps none of the three, and that is a fault rather than a case.**
    #[test]
    fn a_request_whose_only_answer_is_the_finished_one_is_a_fault_in_section_ten() {
        let silent = a_request_whose_only_answer_is_the_finished_one();
        assert_eq!(silent.how_many_of_the_three(), 0);
        assert_eq!(
            the_floor_is_met(&silent),
            WhetherTheFloorIsMet::ItIsAFaultInSectionTen
        );
    }

    /// **G2: C7's half on its own is enough for the floor**, which is what makes
    /// the acknowledgement more than a restatement of the result.
    #[test]
    fn the_acknowledgement_alone_meets_the_floor() {
        let mut only_the_acknowledgement = a_request_whose_only_answer_is_the_finished_one();
        only_the_acknowledgement.it_was_acknowledged_as_recorded = true;
        assert_eq!(
            the_floor_is_met(&only_the_acknowledgement),
            WhetherTheFloorIsMet::ItIsMet
        );
    }

    /// **G3: an expiry is a reportable event and never a hang**, and the set is
    /// one member because there is nothing else it can be.
    #[test]
    fn a_waiters_expiry_is_a_reportable_event() {
        assert_eq!(what_a_waiters_expiry_is(), WhatAWaitersExpiryIs::AReportableEvent);
    }

    /// **G9: what is reported about the work is what the record and the journal
    /// say, and an expiry cannot reach it.**
    ///
    /// **The assertion is about the shape**: there is no argument for an expiry,
    /// so two callers — one whose waiter gave up and one whose waiter did not —
    /// get the same answer for the same work, which is exactly what G9 requires.
    #[test]
    fn a_waiter_that_gave_up_changes_nothing_about_what_is_reported() {
        let what_the_records_say = "the second attempt is running";
        let after_a_waiter_gave_up = what_is_reported_about_the_work(what_the_records_say);
        let with_a_waiter_still_waiting = what_is_reported_about_the_work(what_the_records_say);
        assert_eq!(after_a_waiter_gave_up, with_a_waiter_still_waiting);
        assert_eq!(after_a_waiter_gave_up, what_the_records_say);
        assert!(A_WAITER_THAT_GAVE_UP_SAYS_THE_WAITER_GAVE_UP.contains("about the waiter"));
    }

    /// **G4's three routes are named**, and a request is held to one of them
    /// rather than to nothing.
    #[test]
    fn each_of_the_three_routes_meets_the_floor_on_its_own() {
        for route in [
            WhichRouteItsProgressTakes::ItsOwnByteCount,
            WhichRouteItsProgressTakes::TheEventJournal,
            WhichRouteItsProgressTakes::ItsOwnResultAndItsWaitersDeadline,
        ] {
            let mut request = a_request_whose_only_answer_is_the_finished_one();
            request.its_progress_route = Some(route);
            assert_eq!(the_floor_is_met(&request), WhetherTheFloorIsMet::ItIsMet);
        }
    }
}
