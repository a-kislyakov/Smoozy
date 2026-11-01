//! What of this document is not built here, named clause by clause rather
//! than silently absent — **and, since the acting half landed, what of it now
//! is.**
//!
//! **`Preparation/Build/Plan.md` step 7 builds this module.** The rules half
//! came first — every judgement, data shape and composition — and
//! [`crate::acting`] followed it: the real end of every reading the judgements
//! stand on, and the driving of the one act this document has.
//!
//! **So this page is two lists.** [`THE_ACTS_NOW_BUILT_HERE`] names what the
//! rules half left and where each piece landed.
//! [`THE_ACTS_STILL_OUTSIDE_THIS_CRATE`] names what is still not this crate's,
//! each with the document that owns it, so nothing is silently absent.
//!
//! **Nothing on either list is stubbed.** There is no always-yes answer, no
//! empty value standing in for one, and no default anywhere. Where a fact the
//! plans do not establish would otherwise have been written from memory, the
//! absence is a value at its own site — the five spellings at
//! [`crate::spellings`] are the standing instance.
//!
//! **Every clause named below is checked against the plan** at the test, which
//! reads the plan rather than trusting this page.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rented-server runtime` A4, A6. `Rendering` L14,
//! L21, P1. `Job files and outputs` R1, R3. `Server Setup` B11, B12. `File
//! Browser` X7a. `Servers` H14, H15.

/// One act of this document that the rules half does not perform.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnActLeftToTheActingHalf {
    /// The plan's own top-level section.
    pub section: &'static str,
    /// Every clause of that section this covers.
    pub clauses: &'static [&'static str],
    /// The act.
    pub what: &'static str,
    /// Why it is not in the rules half, and what of the clause *is* here.
    pub why: &'static str,
}

/// One act that has since been built, and where.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnActNowBuilt {
    /// The plan's own top-level section.
    pub section: &'static str,
    /// Every clause of that section this covers.
    pub clauses: &'static [&'static str],
    /// The act.
    pub what: &'static str,
    /// Where it landed, and what of it is still on the far side of a seam.
    pub where_it_landed: &'static str,
}

/// **What the rules half left and the acting half now does.**
pub const THE_ACTS_NOW_BUILT_HERE: [AnActNowBuilt; 5] = [
    AnActNowBuilt {
        section: "§1.2",
        clauses: &["Q4", "Q5"],
        what: "the pre-load itself: one file-inspection operation driven from the desktop through the supervisor's command path, admitted at the machine's one admission point under §10's declarations; and one read per (server, file) at a time, a second Pre-load joining the running one",
        where_it_landed: "`crate::acting::pre_load`. **The invocation itself is still a seam** — it is `Job files and outputs` R1's, one invocation of the Blender the manifest names, headless, one JSON document between sentinels, and that document's §4 arrives at this same step; admission on the far side is `Rented-server runtime` A4's. What is driven is `crate::acting::pre_load::DrivesOneFileInspection`, and Q5's join is `crate::acting::pre_load::ReadsInFlight`",
    },
    AnActNowBuilt {
        section: "§1.2",
        clauses: &["Q17"],
        what: "carrying the version pick and the four-value file identity with the read while it is in flight",
        where_it_landed: "`crate::acting::pre_load`, which composes what the read is started under and asks the page what stands at the landing; the judgement over the pair is `crate::file_half::FileHalf::a_reading_landed`'s and is not repeated there",
    },
    AnActNowBuilt {
        section: "§1.5",
        clauses: &["Q14"],
        what: "item 2's fresh read of the four values over the command channel at the page's open",
        where_it_landed: "`crate::acting::four_values`, the `.blend`'s path riding the planted-link carriage rather than the command string. **Item 1's drop at a publication is still another module's act** and is on the other list",
    },
    AnActNowBuilt {
        section: "§7.1",
        clauses: &["B1"],
        what: "asking `Server Setup` for the fresh search result at the moment `Server Setup` B12 names",
        where_it_landed: "`crate::acting::versions`, composed out of `Server Setup`'s own search, asking run and answer, with `Server Setup` B11's honesty marks travelling with it. **`Rendering` L14's confirmation of the binary at the manifest's path is that document's own step** and is on the other list",
    },
    AnActNowBuilt {
        section: "§8.2",
        clauses: &["D5"],
        what: "taking the hardware reading — `Servers` H14 and H15's, once per primary connection",
        where_it_landed: "`crate::acting::hardware`, which reaches no machine at all: the reading is `Servers`' remembered one, read off the server row with the state `Servers` H1's own table puts it in",
    },
];

/// **What is still not this crate's, each with the document that owns it.**
pub const THE_ACTS_STILL_OUTSIDE_THIS_CRATE: [AnActLeftToTheActingHalf; 5] = [
    AnActLeftToTheActingHalf {
        section: "§1.3",
        clauses: &["Q8", "Q9", "Q10"],
        what: "capturing the invocation's exit status, the two sentinels and the error map into `crate::endings::WhatTheInvocationLeft`, and keeping the invocation's record for diagnosis under the ordinary captured-streams rules",
        why: "**the capture is the worker's and the ending's diagnosis is here.** `crate::endings::judge` tells the three endings apart from Smoozy's own frame and never from anything Blender printed, and `crate::acting::pre_load` runs it over whatever the seam hands back; the page's sentence for each is Design's",
    },
    AnActLeftToTheActingHalf {
        section: "§1.5",
        clauses: &["Q14"],
        what: "item 1's drop at a publication, performed by the publishing module (`File Browser` X7a)",
        why: "**another module's act.** It lands on `crate::file_half`'s own drop, which takes it whole with no comparison",
    },
    AnActLeftToTheActingHalf {
        section: "§4.2 and §4.3",
        clauses: &["V4", "V5", "V6", "V7", "V9"],
        what: "applying a pin pair inside the running Blender — resolving the layer by its bytes, severing the switch's animation by the version's own route, assigning the pinned value — and the launch material that carries it",
        why: "**all of it happens inside a Blender on the machine, in `Rendering`'s launch material.** What is here is the pair's shape and order (`crate::view_layers::THE_PIN_PAIR`), the switch's name (`crate::spellings::VIEW_LAYER_USE`), the route per version (`crate::view_layers::the_route_to_the_curve`), and the requirement that an unapplied pin ends the attempt",
    },
    AnActLeftToTheActingHalf {
        section: "§7.1",
        clauses: &["B6"],
        what: "`Rendering` L14's confirmation that the binary at the manifest's path answers with the manifest's version",
        why: "**`Rendering`'s own step of the attempt.** What is here is the rule it guards — a pick made against a stale list costs a failed attempt, never a wrong render — and the list ask itself, which is now `crate::acting::versions`",
    },
    AnActLeftToTheActingHalf {
        section: "§8",
        clauses: &["D2"],
        what: "the cards side: the device argument after the bare double dash, looked up from the card make",
        why: "**`Rendering` L21's, cited whole and composed there.** What is here is the resolved choice L21 reads and the processor side's pair",
    },
];

/// **The three things this document does that are not acts of its own, each
/// a step inside something another document owns**: the render-start
/// reading, inside the admitted render operation (`Job files and outputs`
/// R3, `Rendering` P1 step 4); composing the submission payload, inside
/// submitting a job; and the version-list ask, `Server Setup`'s own search.
/// None is a worker of this crate's and none is on either list above.
pub const THE_STEPS_INSIDE_OTHER_DOCUMENTS_ACTS: [&str; 3] = [
    "the render-start reading",
    "composing the submission payload",
    "the version-list ask",
];

/// **W2's cancellation, which is a clause rather than a gap.** The kind is
/// terminable at any moment and nothing durable is half-written at any point,
/// so ending a read anywhere loses a reading and corrupts nothing
/// (`Rented-server runtime` A6). On the desktop side that is a dropped future,
/// and `crate::acting::pre_load` gives up the read's claim on its (server,
/// file) whether the read finished or was dropped.
pub const THE_READ_IS_TERMINABLE_AT_ANY_MOMENT: bool = true;

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use super::*;

    fn the_plan() -> String {
        fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../Preparation/Architecture/Render Settings.md"),
        )
        .expect("the plan is readable")
    }

    fn the_plan_defines(plan: &str, clause: &str) -> bool {
        plan.lines()
            .any(|line| line.trim_start().starts_with(&format!("**{clause}.")))
    }

    /// **Every clause named on either list is one the plan defines**, read at
    /// the test rather than copied here: a clause opens a line with the marker
    /// in bold.
    #[test]
    fn every_clause_named_on_either_list_is_a_clause_the_plan_defines() {
        let plan = the_plan();
        for act in THE_ACTS_STILL_OUTSIDE_THIS_CRATE {
            for clause in act.clauses {
                assert!(
                    the_plan_defines(&plan, clause),
                    "{clause} is named at {} and the plan defines no such clause",
                    act.section
                );
            }
        }
        for act in THE_ACTS_NOW_BUILT_HERE {
            for clause in act.clauses {
                assert!(
                    the_plan_defines(&plan, clause),
                    "{clause} is named at {} and the plan defines no such clause",
                    act.section
                );
            }
        }
    }

    /// **Nothing moved off the first list without landing on the second.** The
    /// rules half named eight acts over sixteen clauses; the two lists together
    /// must still name every one of those clauses, or an act has gone quiet
    /// rather than been built.
    #[test]
    fn the_two_lists_together_still_name_every_clause_the_rules_half_left() {
        let left_by_the_rules_half = [
            "Q4", "Q5", "Q8", "Q9", "Q10", "Q17", "Q14", "V4", "V5", "V6", "V7", "V9", "B1", "B6",
            "D2", "D5",
        ];
        for clause in left_by_the_rules_half {
            let named = THE_ACTS_NOW_BUILT_HERE
                .iter()
                .any(|act| act.clauses.contains(&clause))
                || THE_ACTS_STILL_OUTSIDE_THIS_CRATE
                    .iter()
                    .any(|act| act.clauses.contains(&clause));
            assert!(named, "{clause} is on neither list");
        }
    }

    #[test]
    fn nothing_on_either_list_is_the_lookup_item_29_hands_to_build() {
        // Item 29 is work rather than an act of the acting half's, and it is
        // met at `crate::spellings` rather than filed here.
        for act in THE_ACTS_STILL_OUTSIDE_THIS_CRATE {
            assert!(!act.what.contains("spelling"), "{}", act.section);
        }
        for act in THE_ACTS_NOW_BUILT_HERE {
            assert!(!act.what.contains("spelling"), "{}", act.section);
        }
    }
}
