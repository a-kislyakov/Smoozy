//! What of this document is not built here, named clause by clause rather
//! than silently absent.
//!
//! **`Preparation/Build/Plan.md` step 7 builds this module, and this is the
//! rules half.** Every clause of the plan has its judgement, its data shape or
//! its composition in this crate; what is left is the half that acts — a
//! reading driven through the supervisor, a folder opened, a transfer
//! arranged, a Blender's insides changed — and each piece of it is named
//! below with the clause it belongs to, so nothing is silently absent.
//!
//! **Nothing on the list is stubbed.** There is no always-yes answer, no empty
//! value standing in for one, and no default anywhere. Where a fact the plans
//! do not establish would otherwise have been written from memory, the
//! absence is a value at its own site — [`crate::remap::AssignmentAlone::NotEstablished`]
//! is the standing instance.
//!
//! **Every clause named below is checked against the plan** at the test,
//! which reads the plan rather than trusting this page.
//!
//! # What has landed since, and what has not — 2026-09-04
//!
//! **The acting half was built at the same step and most of this list is now
//! in [`crate::acting`], so the thirteen entries below are a map of the acts
//! rather than a list of absences.** Each entry is still true about *what the
//! act is* and *which clause it belongs to*; what has changed is that ten of
//! them have code. **This head says which, so nobody reads the list as a
//! backlog or as a claim that everything on it is done.**
//!
//! **Built:** §2's digest on the user's own computer ([`crate::adapter`]);
//! §3.1, §3.4 and the reading's own script ([`crate::acting::checking`],
//! [`crate::acting::reading_script`]); §4.2 and §4.3's enumeration and
//! stored-name reading ([`crate::adapter`]); §5.2 and §5.3's mirror
//! directories and arranged uploads ([`crate::acting::sending`]); §6.3's
//! application inside the running Blender ([`crate::acting::material`]);
//! §6.4's placement and the ordered unit ([`crate::acting::placing`]); §6.5's
//! production of a fixed copy ([`crate::acting::fixing_worker`]); §7.3's two
//! marks ([`crate::acting::marks`]).
//!
//! **Not built on 2026-09-04, and each is named where it is met rather than
//! only here.** Three of the four below are built since; the next heading says
//! which, and the first is still not.
//!
//! - **§2's Q3 digest of a machine file by the package's own code.** There is
//!   no route from the desktop into the package's endpoint and no worker of
//!   this kind on the far side of it — the program `smoozy-server` exists and
//!   refuses every worker kind by name — so
//!   [`crate::acting::machine::TheDigestWorker`]
//!   is the seam and what stands in it answers *did not complete* — which
//!   establishes nothing either way, so a row that wanted it is sent again
//!   rather than believed.
//! - **§6.7's M22c copy into the job folder.** A machine-side copy, the
//!   package's own by [`THE_ACTING_HALF`]'s own words, and M22c forbids the
//!   only route the desktop has — *"never a second crossing of the
//!   connection"*. What is here is the instruction
//!   ([`crate::acting::placing::TheCopyTheMachineOwes`]) and M22f's record.
//! - **§1 and §7.1's K1 tables, as a store.** The tables themselves are
//!   `Desktop app and delivery`'s record at its schema version 10; the keeper
//!   that reads and writes them lives in that crate beside `File Browser`'s,
//!   and is not this builder's to write.
//! - **§4.1's F1 searched-folder list, as a store.** The same keeper and the
//!   same absence, and it is a separate entry because **F1's list is not one
//!   of K1's five kinds**, so the entry above never covered it. `search` takes
//!   the list as an argument, which is faithful — who hands it in is the
//!   shell's wiring — and the table already exists as
//!   `dependency_searched_folder`. **Added 2026-09-05, out of round 1 of this
//!   module's code audit**: it was the one table whose absence nothing named,
//!   in a file whose whole promise is that every absence is named.
//!
//! # What is performed, and what still is not — 2026-09-29
//!
//! **Step 10c's dependency wave gave most of these acts a caller, so each entry
//! below now says what performs it.** The list stays a map of the acts, so it
//! still holds thirteen entries; none is struck, because each is still true
//! about what the act is and which clause it belongs to.
//!
//! **Performed:** §3.1 and §3.4's loop, by `Desktop app and delivery`'s
//! `src/checking_a_file.rs` through the machine's file-inspection worker —
//! all but the in-flight reading's cancel; §5.2 and §5.3's sends, by its
//! `src/the_found_files_sent.rs`; §6.4's composition and placing, by its
//! `src/submitting.rs`; §6.7's M22c copy, by the machine's render worker from
//! the instruction composed at submission; §7.3's marks, at a row send, at an
//! upload through `File Browser` and at a replace; §1 and §7.1's K1 store,
//! by its `src/record/dependency_records.rs`; and §4.1's F1 list, kept since
//! 2026-09-13 by its `src/surface/settings.rs`.
//!
//! **Still not built on 2026-09-29:** §2's Q3 digest of a machine file and
//! §6.5's fixing, both out of 0.1.7 on the build runner's ruling of
//! 2026-09-28; cancelling a reading already in flight; and §7.3's marks at
//! `File Browser`'s copy, move and delete, which were not built themselves.
//!
//! # What the 38's piece 7 performed — 2026-10-04
//!
//! **Every entry is performed now.** O2 was settled on 2026-10-03: the digest
//! and the fixing run as file-inspection workers. **§2's Q3** is the machine's
//! own program reading a file through (`crate::acting::machine`), which
//! `Desktop app and delivery`'s `src/the_found_files_sent.rs` asks where the
//! record does not establish a file (Q5 route 2, Q7's read-through). **§6.5**
//! is `crate::acting::fixing_worker` over the supervisor, produced at the end
//! of each check for every linked `.blend` this module placed and re-produced
//! where M16's triggers owe one, and tried again on its own (M16a).
//! **G15's cancel of the in-flight reading** is the check's own *Stop*,
//! `crate::acting::checking::drive_one_reading_until_stopped`. **§5.6's handed
//! unit** goes through the check its offer opens. **§7.3's marks at `File
//! Browser`'s copy, move and delete were found already written** — by
//! `Desktop app and delivery`'s `src/record/neighbouring_records.rs`
//! (`units_of_a_managing_act_are_marked`), since Files' owed acts pieces 3 and
//! 4 — so the line above saying they were not was stale.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `File Browser` X7a, N6, N7a, W1, W1a. `Desktop
//! app and delivery` F10a. `Server protocol and transfers` S26. `Job files
//! and outputs` R1, D16, D20.
//!
//! **`Rented-server runtime` A4 is not listed above and is written out at the
//! site**, because this plan defines an A4 of its own.

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
}

/// **The acts, section by section.**
pub const THE_ACTING_HALF: [AnActLeftToTheActingHalf; 13] = [
    AnActLeftToTheActingHalf {
        section: "§2",
        clauses: &["Q2", "Q3"],
        what: "computing a digest — on the user's computer by the application reading the file through, and on the machine by the package's own code as a per-operation worker",
        why: "**both are readings behind `crate::readings::Readings`.** What is here is Q1's content identity, Q5's two routes, Q7's confirmation, Q8's trigger and Q9's lookup, all judged over those readings. **The machine's half is performed since 2026-10-04** (the 38's piece 7) by the machine's own program as a file-inspection operation (`crate::acting::machine::the_machine_copys_contents`)",
    },
    AnActLeftToTheActingHalf {
        section: "§3.1 and §3.4",
        clauses: &["G1", "G14", "G15", "G16", "G17"],
        what: "the check's loop: one invocation of the named Blender per file, headless, driven as a file-inspection worker through the supervisor and admitted as `Rented-server runtime` A4 argues; issuing the next reading as files arrive; cancelling the in-flight one; writing each finding to the record at the moment it lands",
        why: "**a process on a machine, and the loop that drives it.** What is here is G2's version consumption, G3's ending judgement, the document shape (`Job files and outputs` R1's), G13's visited set, and every judgement over what a reading returned (`crate::check`). **Performed since 2026-09-29**: `Desktop app and delivery`'s `src/checking_a_file.rs` drives the loop, one request per `.blend` to the machine's own file-inspection worker, and writes each reading's findings in one save as it lands (step 10c's dependency wave pieces 2 and 5). **Cancelling the in-flight reading since 2026-10-04** (the 38's piece 7): the check's *Stop* sends `Server protocol and transfers` C21's stop naming the reading, and the machine interrupts its recorded Blender",
    },
    AnActLeftToTheActingHalf {
        section: "§3.2 and §3.5",
        clauses: &["G8", "G19"],
        what: "the reading's own script: the walk over Blender's own path marks joined with the map and the flat list, the node-tree walk into every node group for the import nodes, and the walk over the record of what uses what for scene attribution",
        why: "**runs inside the Blender the reading drives.** What is here is what the script returns (`crate::check::ReadingDocument`), the routes it must take (`crate::check::THE_MAP`, `THE_FLAT_LIST`), and G20a's rule for what attribution means",
    },
    AnActLeftToTheActingHalf {
        section: "§4.2 and §4.3",
        clauses: &["F8", "F12", "F13"],
        what: "enumerating a folder on the user's own computer, and the stored-name reading — `Desktop app and delivery`'s platform file adapter (its F10a), on macOS by F12's call set, on NTFS and ReFS by F14a's normalized name, on ext4 by the enumeration",
        why: "**a reading of the user's disk.** The walk's rules, F9's byte equality, F10's fold and F14a's exactness are `crate::search`'s, run over `crate::readings::Readings`",
    },
    AnActLeftToTheActingHalf {
        section: "§5.2 and §5.3",
        clauses: &["A7", "A8", "A11", "A12"],
        what: "creating the mirror directories with the plain form segment by segment, arranging each asset's transfer under the contract in the dependency-upload class, the retry as a new transfer, and the ending under `Server protocol and transfers` S26",
        why: "**machine acts over the file-transfer subsystem.** What is here is A4's mirror, A3's address, A5's byte checks, A9's class, A10's contents test and A16's refusal (`crate::landing`). **Performed since 2026-09-29**: `Desktop app and delivery`'s `src/the_found_files_sent.rs` is the caller of `crate::acting::sending`, sending each found file to its mirror and writing its placement record as it lands (step 10c's dependency wave piece 6)",
    },
    AnActLeftToTheActingHalf {
        section: "§5.6",
        clauses: &["A17", "A18"],
        what: "the handed unit's three acts — the send to A4's mirror, G1's reading of the published `.blend`, M14's fixing — and returning the transfer request's operation identity as the act opens",
        why: "**the send, the reading and the fixing are all acts of the acting half's.** What is here is A15's four fields, A16's one refusal, and A18's destination rule (`crate::landing::the_destination_of`). **Performed since 2026-10-04** (the 38's piece 7): `Desktop app and delivery`'s `src/surface/files.rs` hands each unit over (`File Browser` U12) and opens the check of the `.blend` whose tree reached it, whose own send, reading and fixing carry it",
    },
    AnActLeftToTheActingHalf {
        section: "§6.3",
        clauses: &["M8", "M9"],
        what: "applying each pair inside the running Blender by the per-setting route, reloading a library after its assignment, and writing every pair's outcome into `Job files and outputs` D16's marker on the render's own captured stream",
        why: "**happens inside a Blender on the machine, in `Rendering`'s launch material.** What is here is M4a's match text, the matching rule, M9d's field and the outcome per pair (`crate::remap`)",
    },
    AnActLeftToTheActingHalf {
        section: "§6.4",
        clauses: &["M10", "M11"],
        what: "placing the material through the transfer contract before the submission's own material, as one ordered unit with the submission",
        why: "**a transfer and a step of submission.** What is here is the material's document, its location and its manifest reference (`crate::remap::RemapMaterial`), the composition from the record (`crate::remap::the_answers_to_compose` and `crate::remap::compose_the_pairs`, every pair's kind and setting from K4's last limb), and the ordered unit's declaration (`crate::units`). **Both acts are performed**: `Desktop app and delivery`'s `src/submitting.rs` composes the material from the record alone at submission and places it first through `crate::acting::placing` (step 10c's dependency wave pieces 7 and 7b, 2026-09-29)",
    },
    AnActLeftToTheActingHalf {
        section: "§6.5",
        clauses: &["M14", "M16", "M16a"],
        what: "producing or replacing a fixed copy: open the machine file headless, rewrite by the per-setting route, save the result to a staging path as a copy with the save-time relative-remapping off, digest, publish by one rename, rewrite K8 at publication; and the retry",
        why: "**drives a Blender on the machine.** What is here is A3's address, the three triggers as record comparisons, and M16a's stable reasons (`crate::fixing`). **Performed since 2026-10-04** (the 38's piece 7): `crate::acting::fixing_worker` asks the machine's file-inspection worker for the copy and publishes it by one rename; `Desktop app and delivery`'s `src/the_found_files_sent.rs` (`a_library_is_fixed`) writes K8 at publication, at each check's end and on *Try again*",
    },
    AnActLeftToTheActingHalf {
        section: "§6.7",
        clauses: &["M22c"],
        what: "copying the uploaded bake from the asset area to the job-folder location D20 assigns, on the machine, before the render starts, and reading its four values the moment it was made",
        why: "**a machine-side copy.** What is here is M22f's placed-copy record and the written-over test (`crate::remap::PlacedCopy`), and what a submission owes (`crate::acting::placing::compose_the_copies_owed`). **Performed since 2026-09-29**: the instruction is the machine half's `placed_copies::TheCopyTheMachineOwes`, riding the remap material; `Desktop app and delivery`'s `src/submitting.rs` composes it at submission, and the machine's render worker makes the copy before the render starts (step 10c's dependency wave pieces 8 and 7). The four values each placed file is read at are written on the machine, and nothing on the desktop reads them yet",
    },
    AnActLeftToTheActingHalf {
        section: "§7.3",
        clauses: &["V13", "V13a"],
        what: "writing the marks at the publishing act — `File Browser` X7a for its own publications, an upload's, a copy's and a same-filesystem move's, and this module's own publication handling for a row send — into the standing store `Desktop app and delivery` §5 builds",
        why: "**the write is the publishing act's.** What is here is the standing, the two marks as its own operations, and which roots a disturbing act reaches (`crate::standing`). **Performed since 2026-09-29 for a row send, an upload through `File Browser` and a replace**: `Desktop app and delivery`'s `src/the_found_files_sent.rs` and `src/record/neighbouring_records.rs` write the marks into the store its `src/record/dependency_records.rs` keeps, the walk taking `File Browser`'s own answer for where a file came from first (step 10c's dependency wave pieces 6, 7 and 7b). **`File Browser`'s copy, move and delete mark too** — found on 2026-10-04 (the 38's piece 7) already written by `src/record/neighbouring_records.rs`'s `units_of_a_managing_act_are_marked`, since Files' owed acts pieces 3 and 4",
    },
    AnActLeftToTheActingHalf {
        section: "§4.1",
        clauses: &["F1"],
        what: "the searched-folder list's durable store — one app-wide ordered list, user-held, in the same database `Desktop app and delivery` §5 builds and the same keeper the K1 entry below names",
        why: "**a durable store is that document's to build, and F1's list is not one of K1's five kinds**, so the entry below does not cover it. What is here is the list's shape and the walk over it in the user's order (`crate::search::SearchedFolderList`, `crate::search::search`), which take the list as an argument — who hands it in is the shell's wiring. **No question of this shape is added to `crate::record::TheRecord`**: nothing in this crate asks the record for the list, and inventing one for a store nobody has built would be building past what is written down. **The store is built, and was before this entry said otherwise**: `Desktop app and delivery`'s `src/surface/settings.rs` has kept the list since 2026-09-13 (step 10a's piece 11), and its `src/record/dependency_records.rs` names that keeper rather than a second one (step 10c's dependency wave piece 4). Still no question of this shape is asked of `crate::record::TheRecord`; the check's driver hands the list in",
    },
    AnActLeftToTheActingHalf {
        section: "§1 and §7.1",
        clauses: &["K1", "V1"],
        what: "the local record's tables for the five kinds and the per-job fields, in the database `Desktop app and delivery` §5 builds — the store behind `crate::record::TheRecord`, which `File Browser`'s W1 and W1a read",
        why: "**a durable store is that document's to build.** What is here is every record's shape, its keying, its lifetime, and every question this module asks of it. **Built since 2026-09-29**: `Desktop app and delivery`'s `src/record/dependency_records.rs` reads and writes every kind over the tables, one save per reading, and hands this crate a snapshot of one machine's record as `crate::record::InMemoryRecord` (step 10c's dependency wave piece 4)",
    },
];

/// **The K5 entry: a check started from `File Browser` N7a's *Check
/// dependencies…* on a file its N6 lookup finds nothing for** starts the
/// check in K5's reduced form. The start is that document's control and the
/// acting half's loop; the reduced keying is `crate::record::HoldingFile`.
pub const THE_K5_START_IS_FILE_BROWSERS_CONTROL: bool = true;

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use super::*;

    fn the_plan() -> String {
        fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../Preparation/Architecture/Dependency Upload & Remap.md"),
        )
        .expect("the plan is readable")
    }

    /// **Every clause named on the list is one the plan defines**, read at the
    /// test rather than copied here: a clause opens a line with the marker in
    /// bold followed by its own full stop.
    #[test]
    fn every_clause_left_to_the_acting_half_is_a_clause_the_plan_defines() {
        let plan = the_plan();
        for act in THE_ACTING_HALF {
            for clause in act.clauses {
                let opens_a_clause = plan
                    .lines()
                    .any(|line| line.trim_start().starts_with(&format!("**{clause}.")));
                assert!(
                    opens_a_clause,
                    "{clause} is named at {} and the plan defines no such clause",
                    act.section
                );
            }
        }
    }

    #[test]
    fn nothing_on_the_list_is_one_of_the_five_kinds_answer_or_the_windows_reading() {
        // Both are permanently unresolved; neither is an act of the acting
        // half's and neither is filed here as one.
        for act in THE_ACTING_HALF {
            assert!(!act.what.contains("cache file"), "{}", act.section);
            assert!(!act.what.contains("Windows machine"), "{}", act.section);
        }
    }
}
