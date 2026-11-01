//! What of this document is not built here, named clause by clause rather than
//! silently absent.
//!
//! **`Preparation/Build/Plan.md` step 8 builds this module.** Every clause of
//! the plan that is a shape, a record, a judgement or a derivation has its code
//! in the files beside this one; **what is left is the half that acts** — a
//! walk performed over a connection, a fetch carried out as a transfer, a
//! durable store written, a machine read, a local write made, a folder opened
//! — and each piece of it is named below with the clause it belongs to, **so
//! nothing is silently absent.**
//!
//! **Nothing here is stubbed.** There is no always-yes answer, no empty value
//! standing in for one, and no default anywhere. **Where a machine or the
//! desktop's own local record is genuinely on the far side, the act is behind
//! a trait of this crate's.**
//!
//! **Three of those traits had no implementation anywhere outside this crate's
//! own tests until step 10c piece 10, and all three now have one.**
//! [`crate::arrangement::KeepsTheArrangement`] and
//! [`crate::request::KeepsTheRequest`] are answered by
//! `smoozy_desktop::record::download_records`, at record version 26, and
//! [`crate::fetching::TheReceivingSystemsRules`] by that crate's
//! `downloading::TheReceivingSystemsRulesHere`. **What was true when this page
//! was written — that every judgement here is built and proved on values, and
//! none of it has met a machine — is still true of the judgements; what changed
//! is that the acts now have callers.**
//!
//! # What has landed since, and what has not — corrected 2026-09-06
//!
//! **The acting half was built at the same step and most of this list is now
//! in [`crate::acting`], so the twelve entries below are a map of the acts
//! rather than a list of absences.** Each entry is still true about *what the
//! act is* and *which clause it belongs to*; what has changed is that most of
//! them have code. **This head says which, so nobody reads the list as a
//! backlog or as a claim that everything on it is done.**
//!
//! **Two entries were added at the close check of that same round, and they
//! are the other direction of the same fault.** §8.3's read of a destination
//! folder's listing and §8.4's creation of a landing folder are seams the
//! round itself made — [`crate::request::ReadsTheDestinationsListing`] and
//! [`crate::acting::enumerating::MakesTheLandingFolder`] — and each of their
//! doc comments points at this page while this page carried neither. **A
//! cross-reference to a register that does not hold the thing is a record
//! claiming a state the code is not in**, which is what the round's own
//! must-do 10 was raised about.
//!
//! **Two entries claimed more than they had, and were corrected at the code
//! round of 2026-09-06.** §5.1 said the byte loop was driven and nothing drove
//! it; §7 said E1's four steps were built in E1's order and step 3 — the
//! unit's ending taken back and written — had no code at all. **A record that
//! says a thing is done when it is not is the failure this project's board
//! names most often**, so the shape of the correction matters as much as the
//! correction: each entry now says which half is there and what removes the
//! rest.
//!
//! **Built, over a real transport.** §3's walk and its cadence
//! ([`crate::acting::walking`], [`crate::acting::wiring`]); §4 J2's and §6.1
//! N2's reads of the job's record directory
//! ([`crate::acting::record_directory`]); §7 E1's four steps
//! ([`crate::acting::running`]), step 3 included; §8.2's enumeration with its
//! per-branch refusals ([`crate::acting::enumerating`]).
//!
//! **Built, and driven from the desktop.** §5.1 P1 and P2 — the record
//! composed, the unit offered at `File Browser`'s own admission point, the
//! unit's ending taken back and written, and
//! [`crate::acting::fetching::carry_out_one_fetch`] called by
//! `smoozy_desktop::downloading`, which hands `Server protocol and transfers`'
//! own loop everything it takes. *(Corrected 2026-10-04, the 38's piece 9:
//! this paragraph said the byte loop was driven by nothing, which stopped
//! being true at step 10c piece 10.)*
//!
//! **Built as a sequence around a seam nothing fills here.** §8.5 M13a's
//! bounded reading and its one removal ([`crate::acting::orphans`]); §9.3 D7's
//! probe and its cadence ([`crate::acting::probing`]); §11 H7's opening
//! ([`crate::acting::revealing`]). **The far side of each is `Desktop app and
//! delivery`'s platform file adapter**, and **what is unbuilt in that crate is
//! C3, C10, C13, F6, F8, F10a's read of a stored name, and K20 to K24**;
//! **F1 to F5, F7, F9 and F10 are built there**, at that crate's
//! `platform/files.rs`. *(Corrected 2026-09-07: this said every one of those
//! clauses was unbuilt in that crate, which was never true of the adapter's own
//! write path.)* What is still missing for the seams above is the adapter in
//! that crate that implements this crate's traits over them, which a feature
//! module cannot write for itself, so a test double is the only thing that
//! fills one here.
//!
//! **Seams the round made, with no implementation anywhere.** §8.3 M6 and M8's
//! read of a destination folder's listing
//! ([`crate::request::ReadsTheDestinationsListing`]); §8.4 M9's creation of a
//! landing folder ([`crate::acting::enumerating::MakesTheLandingFolder`]).
//! **The far side of each is the same platform file adapter**: F10 is built
//! there and F10a's read of a name as the filesystem stores it is not, and
//! neither trait has an implementation outside this crate's own tests.
//!
//! **Not built, and each is named where it is met rather than only here.**
//!
//! - ~~**§2 R1 and R11's durable store, and §8.1 M1's.**~~ **BOTH BUILT at step
//!   10c piece 10**, at `smoozy_desktop::record::download_records`, record
//!   version 26. `State, identity and recovery` §1.1 puts every durable store
//!   in the one local record, that record is `Desktop app and delivery` §5's,
//!   and its keepers live in that crate beside `File Browser`'s —
//!   `smoozy_desktop::record::upload_requests` was the template and is the
//!   twin. What is here is still every write's moment and order
//!   ([`crate::acting::running`]).
//! - **§5.2 to §5.4's two adapter judgements**, and the three seams above that
//!   rest on the same wall. **The name check, the exclusive create and the
//!   write-failure classes are `Desktop app and delivery`'s and are consumed
//!   as values**; nothing in this crate produces one from a byte, a name, an
//!   error number or a symbol. *(Step 10c piece 10 built the **name check** —
//!   `Desktop app and delivery` C3, that document's C4, C5 and C7, per system — in that crate's
//!   `downloading`, so P4 has a real answer. **`Desktop app and delivery` C10's
//!   exclusive create and that document's C15 and C19 — saying what a taken
//!   name is taken by — are still unbuilt**, so
//!   P6's classification never runs from the running application and a stop
//!   lands at *left alone*, which is the safe direction.)*
//! - ~~**`Server protocol and transfers` S13's four-value reading**, which has
//!   no producer for a download's own source and none is invented here. It is a
//!   command-channel reading, and what program answers on an arbitrary rental
//!   is `File Browser` §13 item 9's unestablished ground — **permanently
//!   unresolved under the Creator's ruling of 2026-08-31**.~~ **CORRECTED
//!   2026-10-04, the 38's piece 9 (item 18): it was never permanently
//!   unresolved, and it is now read.** The same command-channel reading was
//!   taken on every rental already — at submission and by the dependency
//!   check, through `Render Settings`' `read_the_four_values` — so the stated
//!   reason was stale, and it is what kept a cut download starting over on no
//!   list. **The desktop now reads it for a manual download's file large
//!   enough to carry on from, when it is arranged and again before it carries
//!   on** (`smoozy_desktop::downloading`, through
//!   [`crate::acting::fetching::TheFourValues`]), and `Server protocol and
//!   transfers`' loop compares the two before a resume. **A reading that
//!   cannot be taken is S13's *could not be confirmed***: the file starts
//!   over, as before, and it is never read as *the file changed*. **What it
//!   still does not reach**: an auto-download's fetch, which is given up and
//!   fetched again rather than carried on (P3), and so **R7's newer-version
//!   telling, which compares the arrangement's own recorded readings, still
//!   never fires** — nothing is lost by that, since R4's predicted-path rule
//!   and P6's exclusive create both already refuse to write over the user's
//!   copy.
//! - **The connected period.** Nobody in this workspace opens, holds or
//!   reopens a reading channel for one, and `Server protocol and transfers`
//!   T5's own code says admission is the caller's. Every act here takes a
//!   channel it is handed and a connection condition it is told.
//! - ~~**The machine-side write of a recorded reading for a job submitted
//!   unread.**~~ **Written at step 10c piece 14 and read since the ninth-pass
//!   fix piece 1 (2026-09-28).** The render worker writes it into the job's
//!   record directory at render start, one file per attempt, and
//!   [`crate::acting::record_directory::read_the_recorded_reading_the_machine_wrote`]
//!   reads it. Until it lands, what stands is
//!   [`crate::narrowing::TheNarrowingMaterial::ItHasNotLanded`] — §15 item
//!   12's own *what holds*.
//!
//! **Two absences are values at their own site rather than gaps**, which is
//! the shape the sibling crates already use: the order that could not be read
//! is [`crate::vouching::TheOrder::ItCouldNotBeRead`], and the narrowing
//! material that has not landed is
//! [`crate::narrowing::TheNarrowingMaterial::ItHasNotLanded`]. **Neither is an
//! empty list**, because an empty list would read as a fact about the render.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` S1, S9, S12, S13,
//! S15, S28. `Desktop app and delivery` C3, C4, C5, C7, C10, C13, C15, C19,
//! F1, F5, F6, F7, F8, F9, F10, F10a, K20, K21, K22, K23, K24.
//! `Queue & History` C1.
//! `Render Settings` Q14.
//!
//! **A marker this plan also defines is deliberately not listed above,
//! whichever document is meant, and is written out with its document at every
//! site instead** — the header list cannot tell the two apart, and a bare one
//! would resolve to a real and wrong clause. `Job files and outputs` M8 is the
//! one that arises here.

/// One act of this document that the rules half does not perform.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnActLeftToTheActingHalf {
    /// The plan's own section.
    pub section: &'static str,
    /// Every clause of that section this covers.
    pub clauses: &'static [&'static str],
    /// The act.
    pub what: &'static str,
    /// Why it is not in the rules half, and what of the clause *is* here.
    pub why: &'static str,
    /// **What became of it at the acting half, 2026-09-05.**
    ///
    /// **Where the act was built, this names the module that performs it.
    /// Where it was not, it names what removes it** — the module, the crate or
    /// the document that has to write the far side — so no entry on this list
    /// is a name with nothing behind it either way.
    pub what_became_of_it: &'static str,
}

/// **The acts, section by section.**
pub const THE_ACTING_HALF: [AnActLeftToTheActingHalf; 12] = [
    AnActLeftToTheActingHalf {
        section: "§2",
        clauses: &["R1", "R11"],
        what: "the arrangement record's durable store — the fields, the reckoning and the per-entry four-value readings, in the local record `Desktop app and delivery` §5 builds",
        why: "**a durable store is that document's to build**, and `State, identity and recovery` §1.1's inventory already names transfer arrangements there. What is here is the record's whole shape, its lifetime, its survivals and every judgement over it (`crate::arrangement`), behind `crate::arrangement::KeepsTheArrangement`",
        what_became_of_it: "**BUILT at step 10c piece 10**, at `smoozy_desktop::record::download_records::DownloadArrangements`, over record version 26's three tables — the arrangement, its narrowing and its reckoning. It is the keeper this entry named: in that crate, beside `smoozy_desktop::record::upload_requests`, which is `File Browser`'s twin of this exact shape. Every write's moment and order was already here, at `crate::acting::running`",
    },
    AnActLeftToTheActingHalf {
        section: "§3",
        clauses: &["W1", "W4"],
        what: "the walk itself — one directory reading per folder over the file-transfer subsystem's reading route, descending through the whole tree, batched at the protocol's own reply size, and the pause between walks",
        why: "**a reading over a connection this crate does not open.** What is here is the cadence and its aggregate bound, the window that decides whether a walk runs at all, the walk entry's shape, and the change detection over two complete walks (`crate::watch`)",
        what_became_of_it: "**BUILT** at `crate::acting::walking`, over the real subsystem route at `crate::acting::wiring`",
    },
    AnActLeftToTheActingHalf {
        section: "§4",
        clauses: &["J2"],
        what: "the desktop-initiated read of the job's record directory for the attempt record's ordered frame list — `Job files and outputs` M8's record, read over the connection on the route that document supplies",
        why: "**a reading of a machine.** What is here is the order's shape, the refusal to infer one from a range, and both mechanisms' judgements over it (`crate::vouching`); the read's absence is `crate::vouching::TheOrder::ItCouldNotBeRead`, which is a value and not an empty order",
        what_became_of_it: "**BUILT** at `crate::acting::record_directory::read_the_attempts_order`",
    },
    AnActLeftToTheActingHalf {
        section: "§5.1",
        clauses: &["P1", "P2"],
        what: "carrying out one file's transfer under `Server protocol and transfers` §7 — the staging, the record written before the first byte under S9, the byte loop, the integrity split, the publication under S15 — and offering each unit to the per-server admission point",
        why: "**the transfer is that document's, unmodified, and its crate performs it.** What is here is the layer above: which transfers exist, what class each carries, the order within a request or an arrangement, and what a failed unit becomes (`crate::fetching`, `crate::classes`)",
        what_became_of_it: "**BUILT, and driven since step 10c piece 10.** The record is composed at `crate::acting::running::arrange_the_fetch_for` through `crate::acting::fetching::arrange_one_fetch`, the unit is offered at that document's own admission point, the unit's ending is taken back and written at `crate::acting::running::a_fetch_ended`, and `crate::acting::fetching::carry_out_one_fetch` is called by `smoozy_desktop::downloading`, which hands `Server protocol and transfers`' own loop the five things it takes — the channel, the local-computer surface, the shared budget, the record keeper and the watcher — and, since the 38's piece 9, S13's four values read before a resume. *(Corrected 2026-10-04: this entry said the byte loop was driven by nothing and the call had no caller, which stopped being true at piece 10 and went on being said.)*",
    },
    AnActLeftToTheActingHalf {
        section: "§5.2 to §5.4",
        clauses: &["P4", "P5", "P6"],
        what: "the adapter's own two judgements — `Desktop app and delivery` C3's name check per component against the receiving volume, and C10's exclusive create of the final name with C13's reservation completed or removed on the next run",
        why: "**the adapter's crate depends on the feature modules and supplies their adapters, so a feature module cannot reach into it** — the wall `Dependency Upload & Remap` F13 already met. What is here is the outcome's shape as a value the adapter fills (`crate::fetching::TheNameCheckSaid`), the classification of every way the create can fail, and the seam `crate::fetching::TheReceivingSystemsRules`",
        what_became_of_it: "**NOT BUILT, and it is unbuilt in the adapter's own crate too.** What removes it is `Desktop app and delivery` C3, C10 and C13 written there. The sequence around it was built: the check runs at the moment a file first becomes fetchable, at `crate::acting::running`",
    },
    AnActLeftToTheActingHalf {
        section: "§6.1",
        clauses: &["N2"],
        what: "the desktop-initiated read of the job's record directory for the recorded reading and the inventory, over the connection, at or after render-start",
        why: "**a reading of a machine.** What is here is the format list drawn off that reading, and the state where it has not landed (`crate::narrowing::TheNarrowingMaterial::ItHasNotLanded`), which is a value and not an empty list",
        what_became_of_it: "**BUILT** at `crate::acting::record_directory::read_the_narrowing_material`, for a job read before submitting, and at `crate::acting::record_directory::read_the_recorded_reading_the_machine_wrote` for a job submitted unread — the file the render worker writes at render start, one per attempt (step 10c's ninth-pass fix piece 1, 2026-09-28)",
    },
    AnActLeftToTheActingHalf {
        section: "§7",
        clauses: &["E1"],
        what: "running the ending walk — the same reading as §3's, taken once when the ending arrives — and draining the fetch queue to terminal states",
        why: "**the walk and the transfers are acts.** What is here is the whole verdict the walk feeds: what becomes fetchable, what becomes never coming, what carries the could-not-be-vouched-for mark, and when the arrangement is done (`crate::ending`)",
        what_became_of_it: "**BUILT** at `crate::acting::running`, as E1's four steps in E1's order. **Step 3 was the half this entry used to claim and did not have**: nothing took a unit out of the queue, read its ending and wrote the result back, so no arrangement could ever be done, no destination condition could ever be raised and no diary line was ever written. It is `crate::acting::running::a_fetch_ended` — `Server protocol and transfers` S26's ending read, P6's classification of a publication refusal applied, the entry's state and transfer identity written, the slot given back, D4's condition raised from a write-failure class, and §12's act composed at the done transition",
    },
    AnActLeftToTheActingHalf {
        section: "§8.1 and §8.2",
        clauses: &["M1", "M2"],
        what: "the manual request record's durable store, and the enumeration's own remote walk — one directory reading per folder, descending from what the user picked, over the same subsystem route",
        why: "**a durable store is `Desktop app and delivery` §5's and the walk is a reading over a connection.** What is here is the record's shape, the entry's fields, the enumeration's completeness rule, the pre-flight's condition and the whole in files and bytes (`crate::request`), behind `crate::request::KeepsTheRequest`",
        what_became_of_it: "**BUILT at step 10c piece 10.** The enumeration was already at `crate::acting::enumerating`, with its per-branch refusals and the component walk; the store is now `smoozy_desktop::record::download_records::DownloadRequests`, which writes both halves of M1's one record — the shared §9.4 outcome record and version 26's own three tables — joined by §9.4's own ordering",
    },
    AnActLeftToTheActingHalf {
        section: "§8.3",
        clauses: &["M6", "M8"],
        what: "reading what is standing in each destination folder — the whole paths, with names as stored on disk — which is what M6's already-exists question and M8's counted suffix are both asked against",
        why: "**a read of the user's own disk is the platform file adapter's**, the same wall every other local act in this crate meets. What is here is the comparison itself, byte for byte with no folding modelled, the staging and reservation subtraction (R6), the four answers applied to the enumeration once, and the composition of the kept-both name (`crate::request`), behind `crate::request::ReadsTheDestinationsListing`",
        what_became_of_it: "**NOT BUILT.** The seam was added at the code round of 2026-09-06 and has no implementation anywhere in this workspace. What removes it is `Desktop app and delivery` F10a's read of a name as the filesystem stores it, on F10's one route, written in that crate. What was built beside it is the whole judgement over a listing it is handed",
    },
    AnActLeftToTheActingHalf {
        section: "§8.4",
        clauses: &["M9"],
        what: "creating each landing folder a picked folder puts on the user's disk, through the platform file adapter, with a folder that already stands merging rather than colliding",
        why: "**a local write is the platform file adapter's** and a feature module cannot reach into that crate. What is here is which folders the enumeration says are owed, and each outcome handed back with its own folder so a failure is never read as a clean finish (`crate::acting::enumerating`), behind `crate::acting::enumerating::MakesTheLandingFolder`",
        what_became_of_it: "**NOT BUILT.** The seam was added at the code round of 2026-09-06 and has no implementation anywhere in this workspace. **`Desktop app and delivery` F10's local write is built** — that crate's `platform/files.rs` is the one route to the filesystem and its `ensure_directory` is the call M9 wants — so what removes this is the adapter in that crate that implements this trait over it, which a feature module cannot write for itself. *(Corrected 2026-09-07: this said F10's local write was what removes it, as though F10 were unbuilt.)* What was built beside it is the sequence at `crate::acting::enumerating::make_the_landing_folders`",
    },
    AnActLeftToTheActingHalf {
        section: "§8.5",
        clauses: &["M13a"],
        what: "enumerating the reserved staging directory inside each named destination folder, and removing one named unowned entry — `Desktop app and delivery` F10 and F10a are the one route for both",
        why: "**a local enumeration and an explicit deletion are the adapter's.** What is here is which bounded directories may be asked, the classification of an entry as unowned, and the bound that a removal never follows an entry outside the reserved directory (`crate::request`)",
        what_became_of_it: "**BUILT as a sequence** at `crate::acting::orphans`. The two calls it makes are `Desktop app and delivery` F10, which is built in that crate, and F10a's read of a name as the filesystem stores it, which is not — and no adapter in that crate implements this crate's seam over either, so the seam has no producer here. *(Corrected 2026-09-07: this said both were unbuilt in that crate.)*",
    },
    AnActLeftToTheActingHalf {
        section: "§9.3 and §11",
        clauses: &["D7", "H7"],
        what: "the probe's own two writes on the user's disk, and the folder-reveal call — `Desktop app and delivery` K20 to K24's service, per system, whose outcome reaches the caller by its K22",
        why: "**both are local acts through the platform adapter.** What is here is the probe's shape, its ordering, its cadence and what its outcome means (`crate::destination`), and the set of landing places with what the control is handed (`crate::landing`), which are composed from records already kept and are correct whether or not anything opens",
        what_became_of_it: "**BUILT as two sequences** at `crate::acting::probing` and `crate::acting::revealing`. Their far sides are `Desktop app and delivery` F1 to F9 and K20 to K24. **F1 to F5, F7 and F9 are built in that crate**, at its `platform/files.rs`; **F6, F8 and the whole of K20 to K24 are named by nothing there**, and no adapter in that crate implements this crate's two seams, so neither seam has a producer here. *(Corrected 2026-09-07: this said F1 to F9 were unbuilt in that crate.)*",
    },
];

/// **What is left to another module rather than to this one's acting half.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItBelongsToSomebodyElse {
    pub what: &'static str,
    pub whose: &'static str,
}

/// **Three things this document consumes and never performs**, named so that
/// their absence here is not read as an omission.
pub const THREE_THINGS_BELONG_TO_SOMEBODY_ELSE: [ItBelongsToSomebodyElse; 3] = [
    ItBelongsToSomebodyElse {
        what: "the job's ending, one of four, live or handed over once per connection at `Queue & History` C1's catch-up",
        whose: "`Queue & History`",
    },
    ItBelongsToSomebodyElse {
        what: "the connection condition an arrangement's window and its waiting reading turn on, and the standing repair that is the whole cadence for server-side trouble",
        whose: "`Servers`",
    },
    ItBelongsToSomebodyElse {
        what: "the auto-download toggle and the narrowing control on the job's entry, and the placement of every telling this document supplies",
        whose: "`Queue & History` and Design",
    },
];

/// **Nothing of this document waits on a Blender fact that is coming**, and
/// two of its own unresolved entries are permanently unresolved rather than
/// pending: the macOS half of the within-frame write ordering (§15 item 17)
/// and Row 214's format the version cannot deliver (§13). **No space is left
/// in this crate or in its record for either answer.**
pub const NO_SPACE_IS_LEFT_FOR_AN_ANSWER_THAT_IS_NOT_COMING: bool = true;

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use super::*;

    fn the_plan() -> String {
        fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../Preparation/Architecture/Download.md"),
        )
        .expect("the plan is readable")
    }

    /// **Every clause named on the list is one the plan defines**, read at the
    /// test rather than copied here: a clause opens a line with the marker in
    /// bold followed by its own full stop or a colon.
    #[test]
    fn every_clause_left_to_the_acting_half_is_a_clause_the_plan_defines() {
        let plan = the_plan();
        for act in THE_ACTING_HALF {
            for clause in act.clauses {
                let opens_a_clause = plan.lines().any(|line| {
                    let line = line.trim_start();
                    line.starts_with(&format!("**{clause}."))
                        || line.starts_with(&format!("**{clause}:"))
                        || line.starts_with(&format!("**{clause} "))
                });
                assert!(
                    opens_a_clause,
                    "{clause} is named on the acting half's list and the plan defines no such clause"
                );
            }
        }
    }

    /// **The list covers every section that has an act in it**, so a section
    /// dropped from the list fails here rather than going quiet.
    #[test]
    fn the_list_names_twelve_acts_across_the_plans_own_sections() {
        assert_eq!(THE_ACTING_HALF.len(), 12);
        let sections: Vec<&str> = THE_ACTING_HALF.iter().map(|act| act.section).collect();
        for expected in [
            "§2",
            "§3",
            "§4",
            "§5.1",
            "§5.2 to §5.4",
            "§6.1",
            "§7",
            "§8.1 and §8.2",
            "§8.3",
            "§8.4",
            "§8.5",
            "§9.3 and §11",
        ] {
            assert!(
                sections.contains(&expected),
                "{expected} is not on the list"
            );
        }
    }

    /// **Every entry says what is left and why**, so no entry is a name with
    /// nothing behind it.
    #[test]
    fn every_entry_names_the_act_and_says_what_of_the_clause_is_here() {
        for act in THE_ACTING_HALF {
            assert!(!act.clauses.is_empty(), "{} names no clause", act.section);
            assert!(act.what.len() > 40, "{} says too little", act.section);
            assert!(act.why.len() > 40, "{} gives no reason", act.section);
            // **Every entry says what became of it**, and an entry that was
            // not built says what removes it. A page that only said what was
            // missing would go stale the day the acting half landed, which is
            // exactly the failure this project keeps catching in its own
            // records.
            assert!(
                act.what_became_of_it.len() > 40,
                "{} does not say what became of it",
                act.section
            );
            assert!(
                act.what_became_of_it.contains("BUILT"),
                "{} does not say whether it was built",
                act.section
            );
        }
    }
}
