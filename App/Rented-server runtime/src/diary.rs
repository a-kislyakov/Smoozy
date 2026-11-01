//! This module's diary declaration — `Rented-server runtime` §15's entry *To
//! `Reporting a problem`*, **and it is a declaration of none.**
//!
//! **`Reporting a problem` J2 requires every feature document to declare in itself
//! the acts it writes to the diary, and its J4a the machine endings and readings
//! it writes as told lines. This document writes no act lines and no told lines,
//! and that is a decision rather than a silence.**
//!
//! **It was also a silence until the declaration was written, and that is recorded
//! rather than tidied away.** `Reporting a problem` §12 item 7 was carried by
//! thirteen implementers and closed saying *every feature document now holds a
//! declaration*. **This document held none**, so under `Desktop app and delivery`
//! A7 — a module with no declaration writes nothing, *"a hosting fault and not a
//! default"* — **the largest machine-side document in the stage was silently absent
//! from every bug report, and nothing failed to say so.**
//!
//! **The outcome the fault produced and the outcome declared here are the same**,
//! which is exactly why nothing caught it: **a hosting fault and a declaration of
//! none are indistinguishable from the outside.** That is the reason the
//! declaration is written out rather than left implied — and it is why this file
//! exists at all, with a test under it, rather than being an absence somebody would
//! have to notice.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Reporting a problem` J2, J2a, J4, J4a, J6, and its
//! §12 item 7. `Desktop app and delivery` §5.3 L14, and its hosting rule.
//! `Dependency Upload & Remap` and `Server protocol and transfers` — the batch
//! send they reached independently. `Server Setup`, `Servers`, `Rendering`,
//! `Queue & History` — the four documents that declare the acts this one's work
//! sits under.
//!
//! **A marker this plan also defines is never listed here and is written out
//! with its document at the site**, because a header list cannot tell the two
//! apart and a bare one there would resolve to a real and wrong clause.

use smoozy_reporting::lines::LineKind;

/// **The kinds this module declares, and it declares none.**
///
/// **`Desktop app and delivery` A7 makes a module with no declaration write
/// nothing, silently**, so **an empty declaration and no declaration produce the
/// same behaviour** — which is why this is written out with its reasons rather than
/// left absent.
pub const DECLARED: [LineKind; 0] = [];

/// **Why no act line — J2's *whose spec promises it*, and this document has no
/// spec of its own.**
///
/// **No act here is one the user could name, because no spec promises this
/// document's work.** J2 gives each act *"to the module whose spec promises it"*,
/// and **every row this document answers is cross-filed here from `Whole app` or
/// from the documents it supplies.**
///
/// **What a user could name at each of its acts is promised elsewhere and declared
/// there** — and each is named here so a reader can follow the act to the module
/// that does write the line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhereTheActIsDeclaredInstead {
    /// The act, in the words a user could name it by.
    pub the_act: &'static str,
    /// The document whose spec promises it and which declares the line.
    pub declared_by: &'static str,
}

/// **Four acts, four other documents.**
///
/// **A line here would be a second line for an act already declared once**, which
/// is the duplication J2's *whose spec promises it* exists to prevent, and which
/// `Dependency Upload & Remap` and `Server protocol and transfers` reached
/// independently for the batch send.
pub const THE_ACTS_ARE_DECLARED_ELSEWHERE: [WhereTheActIsDeclaredInstead; 4] = [
    WhereTheActIsDeclaredInstead {
        the_act: "a machine becoming ready",
        declared_by: "Server Setup",
    },
    WhereTheActIsDeclaredInstead {
        the_act: "connecting",
        declared_by: "Servers",
    },
    WhereTheActIsDeclaredInstead {
        the_act: "starting a job",
        declared_by: "Rendering",
    },
    WhereTheActIsDeclaredInstead {
        the_act: "what became of that job",
        declared_by: "Queue & History",
    },
];

/// **Why no act line — J2a's third test, and it refuses this document by a wide
/// margin.**
///
/// **This document's unit is one operation, one worker lifecycle or one package
/// phase, and one act the user asked for is many of them**: a single setup is a
/// placement, a boot registration, a health exchange and a commit; a single render
/// is an admission, a worker, a recorded execution group, two captured streams and
/// an ending. **A line at this document's own unit is several lines per act, which
/// is the rate that test excludes by name.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HowManyOfThisDocumentsUnitsOneUserActIs {
    pub the_users_act: &'static str,
    /// The units of this document that one such act passes through.
    pub units: &'static [&'static str],
}

/// **The two worked examples the plan gives, kept as figures rather than as a
/// claim.**
pub const ONE_ACT_IS_MANY_UNITS: [HowManyOfThisDocumentsUnitsOneUserActIs; 2] = [
    HowManyOfThisDocumentsUnitsOneUserActIs {
        the_users_act: "a single setup",
        units: &[
            "a placement",
            "a boot registration",
            "a health exchange",
            "a commit",
        ],
    },
    HowManyOfThisDocumentsUnitsOneUserActIs {
        the_users_act: "a single render",
        units: &[
            "an admission",
            "a worker",
            "a recorded execution group",
            "two captured streams",
            "an ending",
        ],
    },
];

/// **The one case worth stating rather than passing over, because it is the closest
/// thing here to an act with a completion: the package transaction.**
///
/// **It fails test two**: no spec names a package commit, a rollback or a
/// replacement as something the user is told about or can see afterwards — **what a
/// spec names is the machine being ready, and `Server Setup` owns that.**
///
/// **The phases are durable in the package-phase record** (§2) **and a report
/// reaches them there, which is where a builder looks, and not through a diary
/// line.**
pub const THE_PACKAGE_TRANSACTION_FAILS_J2AS_SECOND_TEST: bool = true;

/// **And no told lines.**
///
/// **J4a's declaration is of the machine endings and readings a document
/// *consumes*, and every one taken here is taken *for a consumer* and handed to it
/// by §15's own entries.** **Where one reaches the diary it is written by the module
/// whose act already read it**, which is J4's own pattern. **This document is where
/// they are captured, never where they are told.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TakenHereAndToldElsewhere {
    pub what_is_taken: &'static str,
    /// Which of §15's entries hands it on.
    pub handed_to: &'static str,
}

/// **The four readings and endings this document takes, each for a consumer.**
pub const EVERY_READING_IS_TAKEN_FOR_A_CONSUMER: [TakenHereAndToldElsewhere; 4] = [
    TakenHereAndToldElsewhere {
        what_is_taken: "the exit status",
        handed_to: "Desktop app and delivery",
    },
    TakenHereAndToldElsewhere {
        what_is_taken: "the explicit collection outcome",
        handed_to: "Desktop app and delivery",
    },
    TakenHereAndToldElsewhere {
        // **The wording carries the section on purpose.** §12's request is now
        // named in `smoozy-framing`, whose guard holds every request name to one
        // spelling in the workspace — and a line of prose that happens to be
        // that name reads to that guard as a second spelling. Reworded
        // 2026-09-21 when the name moved; the reading this names is unchanged.
        what_is_taken: "§12's health readings",
        handed_to: "Servers",
    },
    TakenHereAndToldElsewhere {
        what_is_taken: "the free-space reading a render's recorded outcome carries",
        handed_to: "Desktop app and delivery",
    },
];

/// **What the declaration costs, said rather than left to be found.**
///
/// **A report covering a machine that misbehaved holds the owning module's act line
/// and whatever failure line was captured at the presentation of the failure**
/// (`Reporting a problem` J6), **and nothing between them about which phase, which
/// worker or which admission it was.** On a machine-side fault that never reached a
/// screen, **the diary says nothing at all** and the two captured streams and the
/// package-phase record are the whole of what a reader has.
///
/// **That is the trade J2a's third test makes everywhere, and this is the document
/// where the count it refuses would be largest.**
pub const WHAT_THE_DECLARATION_COSTS: &str = "a report covering a machine that \
misbehaved holds the owning module's act line and whatever failure line was \
captured at the presentation of the failure, and nothing between them about which \
phase, which worker or which admission it was; on a machine-side fault that never \
reached a screen the diary says nothing at all, and the two captured streams and \
the package-phase record are the whole of what a reader has";

/// **Lines are written through the diary service `Desktop app and delivery` §5.3
/// L14 names, and this module hands it none.**
///
/// The function exists so that the declaration is a thing code consults rather than
/// a sentence in a comment: **anything asking whether this module may write a line
/// of a given kind is told no, for every kind.**
pub fn may_write(_kind: LineKind) -> bool {
    DECLARED.contains(&_kind)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_module_declares_no_kind_at_all() {
        assert!(DECLARED.is_empty());
        for kind in LineKind::ALL {
            assert!(!may_write(kind), "{kind:?} is not declared and must not be written");
        }
    }

    /// The declaration and the hosting fault produce the same behaviour, which is
    /// why the reasons are written out. This test is the thing that makes the
    /// difference visible: a hosting fault has no reasons beside it.
    #[test]
    fn every_act_of_this_document_is_declared_by_the_module_whose_spec_promises_it() {
        // **The length of an array whose type fixes it cannot be false**, so
        // what stands here instead is that no two entries name the same act —
        // which can be, and which a copied entry makes false.
        let named: std::collections::HashSet<&str> = THE_ACTS_ARE_DECLARED_ELSEWHERE
            .iter()
            .map(|act| act.the_act)
            .collect();
        assert_eq!(named.len(), THE_ACTS_ARE_DECLARED_ELSEWHERE.len());
        for act in THE_ACTS_ARE_DECLARED_ELSEWHERE {
            assert!(!act.the_act.is_empty());
            assert!(!act.declared_by.is_empty());
            assert_ne!(act.declared_by, "Rented-server runtime");
        }
    }

    #[test]
    fn one_user_act_is_several_of_this_documents_units_which_is_what_the_third_test_refuses() {
        let named: std::collections::HashSet<&str> = ONE_ACT_IS_MANY_UNITS
            .iter()
            .map(|example| example.the_users_act)
            .collect();
        assert_eq!(named.len(), ONE_ACT_IS_MANY_UNITS.len());
        for example in ONE_ACT_IS_MANY_UNITS {
            assert!(
                example.units.len() > 1,
                "{} would be one line per act and the third test would not refuse it",
                example.the_users_act
            );
        }
    }

    #[test]
    fn every_reading_taken_here_is_taken_for_a_consumer_and_handed_on() {
        let named: std::collections::HashSet<&str> = EVERY_READING_IS_TAKEN_FOR_A_CONSUMER
            .iter()
            .map(|taken| taken.what_is_taken)
            .collect();
        assert_eq!(named.len(), EVERY_READING_IS_TAKEN_FOR_A_CONSUMER.len());
        for taken in EVERY_READING_IS_TAKEN_FOR_A_CONSUMER {
            assert_ne!(taken.handed_to, "Rented-server runtime");
        }
    }

    #[test]
    fn the_cost_is_written_out_rather_than_left_to_be_found() {
        assert!(WHAT_THE_DECLARATION_COSTS.contains("the diary says nothing at all"));
    }
}
