//! The acting half, clause by clause, with where each act landed — the
//! unresolved entries this crate met and what it built for each; the plan's
//! own contradictions recorded at the site; and the two runs a rented
//! machine could settle.
//!
//! **`Preparation/Build/Plan.md` step 7 builds this module.** Every clause
//! of the plan that is a judgement, a data shape, a composition or a
//! declaration has its code in the files beside this one; **the half that
//! acts — a process started, a stream copied, a signal sent, a machine read,
//! a store written — is [`crate::acting`]**, and each piece of it is named
//! below with the clause it belongs to and the module it landed in.
//!
//! **THE ACTING HALF LANDED ON 2026-09-04**, in the same Build step. This
//! file was written when it had not, and its list is now the map from a
//! clause to the module that performs it rather than a list of absences.
//! **[`WHERE_EACH_ACT_LANDED`] is that map**, and a test reads it against
//! this crate's own sources rather than trusting this page.
//!
//! **Nothing on the list is stubbed.** There is no always-yes answer, no
//! empty value standing in for one, and no default anywhere. Where a fact
//! the plans do not establish would otherwise have been written from
//! memory, the absence is a value at its own site —
//! [`crate::spellings::Spelling::NotOnRecord`]'s shape, the *undetermined*
//! cause that is K14a's default, and the browser-opening case that
//! [`crate::again::WhereTheBrowserOpens::NoDocumentNamesTheOpeningMachine`]
//! names rather than invents.
//!
//! **Every clause named below is checked against the plan** at the test,
//! which reads the plan rather than trusting this page.

/// **Where each of the ten acts landed**, in the order [`THE_ACTING_HALF`]
/// names them. **Read against this crate's own sources at the test**, so a
/// module renamed or removed fails here rather than leaving a stale claim.
pub const WHERE_EACH_ACT_LANDED: [&str; 10] = [
    "acting::launch",
    "acting::material and acting::launch",
    "acting::copy_loop",
    "acting::pipeline",
    "acting::stopping and acting::copy_loop",
    "acting::submitting",
    "acting::page_readings",
    "acting::page_readings",
    "acting::page_readings",
    "acting::pipeline and acting::stopping",
];

/// **What is still not here, and it is one thing rather than a list.**
///
/// **SUPERSEDED 2026-09-19 by step 10c piece 5's second sitting: the program
/// starts a worker of this document's kind.** `smoozy-server` runs the line,
/// starts a render worker for each job in turn and for the second attempt
/// *re-run once* owes, and the worker walks §2's nine steps through
/// `smoozy-rendering-machine`'s own driver with the steps answered on the
/// machine, copies both streams, reaps, walks and reports; the supervisor
/// writes the recorded outcome. ~~**What is absent now is narrower and it is
/// one thing**: §2 P1 step 4's render-start reading is not taken, because the
/// script that has a Blender print `Job files and outputs` R1's one document is
/// on record nowhere in the workspace — so every attempt ends at step 4 and no
/// render is launched.~~ The paragraph that
/// follows is what this said before, kept because it is what the code was
/// built to.
///
/// **STRUCK 2026-09-23, out of step 10c's fifth pass on Vast.** Piece 13 wrote
/// the script and piece 14 took the reading into the driver on 2026-09-20, and
/// **the pass watched the reading taken on a real rented machine** — the
/// version ask, the reading run and a recorded reading written beside them, by
/// a headless Blender on the machine. **The sentence went on being printed at
/// every start of the machine's program for three days**, and it is the first
/// thing anybody reading that machine sees. **Nothing of this document's
/// acting half is absent now**, which is what the constant below says; what is
/// still unproved is a different sentence and it is said there.
///
/// **Nothing starts a worker of this document's kind.** *(Corrected
/// 2026-09-07. This said — as four agents had recorded — that there was no
/// program for the rented machine anywhere in this workspace. That stopped
/// being true at step 8: the program is `smoozy-server`, at
/// `App/Rented-server runtime — program/`.)* **What is still true is that the
/// program starts no worker**: it refuses every one of the seven kinds by
/// name, and it cannot reach this crate. *(Corrected 2026-09-19: this said the
/// program depended on `smoozy-runtime`, `smoozy-framing` and `smoozy-state`
/// and on nothing else. Since step 10c piece 3 it also takes
/// `smoozy-rendering-machine`, where the render worker's acts now live, but not
/// this crate.)* So every
/// machine-side act below is **a library the supervisor would call**, exactly
/// as `Rented-server runtime`'s own worker start already is. **Nothing here
/// invents a program**, and no file of this crate has a `main`.
pub const THE_ONE_THING_STILL_ABSENT: &str =
    "nothing of this document's acting half is absent: the program for the rented machine \
     starts a render worker, walks the nine steps, takes the render-start reading at step 4 \
     and launches. What is unproved rather than absent is that a render has ever finished on \
     a rented machine end to end, which no test can settle and only a pass can";

/// One act of this document, and where it landed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnActLeftToTheActingHalf {
    pub section: &'static str,
    pub clauses: &'static [&'static str],
    pub what: &'static str,
    pub why: &'static str,
}

/// **The acts, section by section**, paired by position with
/// [`WHERE_EACH_ACT_LANDED`].
pub const THE_ACTING_HALF: [AnActLeftToTheActingHalf; 10] = [
    AnActLeftToTheActingHalf {
        section: "§1.2",
        clauses: &["L4", "L5", "L6", "L7"],
        what: "the render worker started by the supervisor against the operation record, the worker replacing itself with Blender so the recorded render process is Blender's own, the detachment that lets the render outlive the connection and the supervisor, and both streams held and copied to the sinks",
        why: "**a process started and held on the machine.** What is here is the line it is started with (`crate::line`) and the contract the worker keeps (`crate::kind`)",
    },
    AnActLeftToTheActingHalf {
        section: "§1.4 and §1.5",
        clauses: &["L11", "L12", "L14"],
        what: "rendering the typed material into the text the render's Blender runs, placing it, reading the placement back under the render's login, and asking the binary to name its own version",
        why: "**a write, a read-back and an invocation on the machine.** The material's contents and order are `crate::material`; the judgement over the version's answer is `crate::versions::confirm_the_version`",
    },
    AnActLeftToTheActingHalf {
        section: "§1.8",
        clauses: &["L26", "L27", "L28"],
        what: "observing the device line in passing at the copy loop on the version's own stream, and recording it in the attempt record",
        why: "**an observation of bytes as they pass.** The ask's switch per version is `crate::spellings::DEVICE_LINE_ASK`; what the line is evidence of is `Queue & History` E12's gate, cited",
    },
    AnActLeftToTheActingHalf {
        section: "§2",
        clauses: &["P1", "P1a", "P2", "P3"],
        what: "the pipeline performed — admission, the remap material read, the four-value check, the reading, the hold placed with a supervisor-issued identity, the walk, the launch, the copy loop, the ending step and its report",
        why: "**every step is an act of the render worker on the machine.** The order is `crate::pipeline::THE_NINE_STEPS`, P1a's judgement is `crate::pipeline::validate_the_names`, step 6's reading rule is `crate::pipeline::the_previous_attempts_frame_list`, and step 7's composition is `crate::composition::compose_the_attempt`",
    },
    AnActLeftToTheActingHalf {
        section: "§3.2 to §3.5",
        clauses: &["K4", "K5", "K7", "K8", "K9", "K10", "K11", "K13"],
        what: "sending the interrupt through the machine's own call to the recorded pair, the seven steps performed with their durable writes, the waiter's clock and its expiry, the ending step's reap and walk, and the copy loop's observations reported to the supervisor",
        why: "**signals, clocks, stores and streams.** The contract, the sequence, the deadline's value and the judgements over what the waiter holds are `crate::kind` and `crate::stop`",
    },
    AnActLeftToTheActingHalf {
        section: "§4.1 to §4.4",
        clauses: &["S1", "S3", "S4", "S4a", "S4b", "S9", "S10", "S11", "S12", "S13", "S14", "S15", "S16", "S17"],
        what: "the provisional record's write, the remap material placed and the material published through the transfer contract, the job record written by the supervisor, the serialisation waited on, the cleanup debt taken at the next connection, the machine asked for a command's identity after a drop, the hold record placed and the release command travelled",
        why: "**writes, transfers, commands and readings.** The gate, the identities, the unit's enumeration, the serialisation's judgement, the debt's composition, the manifest, the reporting field, the split and the hold instruction are `crate::submission`; the resolution and the telling are `crate::after_a_drop`",
    },
    AnActLeftToTheActingHalf {
        section: "§5",
        clauses: &["W1"],
        what: "reading the check standing out of the local record at the moment of submitting",
        why: "**a read of the local record.** The evaluation over what the record says is `crate::warning::evaluate`",
    },
    AnActLeftToTheActingHalf {
        section: "§6.1 to §6.4",
        clauses: &["A1", "A1a", "A4", "A5", "A6", "A7", "A7a", "A8", "A9", "A10", "A11"],
        what: "opening the page from the kept setup, the browser opened on a machine, the carry-on reading taken from the machine's attempt record and its folder, the stat and the walk over the command channel, the file-attribute read of the `.blend`'s path",
        why: "**readings of a machine and the page's opening.** The fresh act's identities, the offer's condition, the non-load, the ranges, the browser's opening rule, the carry-on arithmetic and the three-way judgement over what a reading gave are `crate::again`. **The carry-on reading is `crate::acting::page_readings::the_carry_ons_figure`, over the two readings A4 names on that module's own trait, and its arithmetic is `crate::again::the_last_frame_already_there_on_the_machine`. This entry claimed that reading had landed from 2026-09-04 until 2026-09-05 while nothing in this crate walked a folder or read an attempt record**, so the page could open only on the figure `Queue & History` kept, under A9's sentence saying the machine could not be checked — the names are written out here so the claim is checkable rather than taken on trust. **The page's opening is the desktop's, since the 38's piece 2 (2026-10-03)**: `App/Desktop app and delivery/src/a_new_job_from_an_old_one.rs` loads the kept setup through `crate::again::the_kept_setup_from`, gives `the_carry_ons_figure` its two readings over the server's own point, and takes A10's read as one attribute read on the file-transfer route rather than the command channel's stat — until then nothing opened the page from an entry at all",
    },
    AnActLeftToTheActingHalf {
        section: "§8",
        clauses: &["S5a"],
        what: "reading the app-wide reporting item out of the local record's settings at composition",
        why: "**a read of the local record.** The field's composition from the two values is `crate::submission::the_reporting_field`",
    },
    AnActLeftToTheActingHalf {
        section: "§3.6",
        clauses: &["K14", "K14a"],
        what: "writing the recorded outcome and its cause into the operation record and journalling it",
        why: "**the supervisor's durable write.** The classification and the cause are `crate::endings::classify`; the value into `Queue & History`'s mapping is `crate::endings::as_the_render_kinds_own_ending`",
    },
];

/// One unresolved entry of §9 met by this half, and what was built.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnEntryMet {
    pub item: u8,
    pub what_was_built: &'static str,
}

/// **The unresolved entries this half met, and what was built for each** —
/// the *what holds* where the entry has one, the document's own answer
/// with the caution beside the code where it has none, and nothing for a
/// routing note.
pub const THE_ENTRIES_MET: [AnEntryMet; 21] = [
    AnEntryMet { item: 1, what_was_built: "answered: the read-back compares the raw text handed in and sits after the file by construction (`crate::line`, `crate::material`)" },
    AnEntryMet { item: 2, what_was_built: "5.1 is inferred across the boundary at `crate::versions::TheFourVersions::is_inferred_across_the_boundary`; no redirect setting's name is composed here" },
    AnEntryMet { item: 3, what_was_built: "L23's three rows with each row's filling labelled, and the threshold's half left no space (`crate::technology::THE_TABLE`)" },
    AnEntryMet { item: 4, what_was_built: "the request comes from the launch, which is the side of the line every observed refusal sits on; nothing tells a mismatched driver from a purged one (`crate::technology::TheLoudRefusal`)" },
    AnEntryMet { item: 5, what_was_built: "no stream is edited; 4.5's ask is the debugging switch on the complaints stream, recorded at `crate::spellings::DEVICE_LINE_ASK`" },
    AnEntryMet { item: 6, what_was_built: "nothing claims an EEVEE or Workbench job names a device; the line's reading is `Queue & History` E12's gate, cited" },
    AnEntryMet { item: 7, what_was_built: "120 seconds, both measurement sets recorded beside it, neither chosen between (`crate::stop`)" },
    AnEntryMet { item: 9, what_was_built: "no *breaks* limb; nothing built — the disk-full cost is `Rented-server runtime`'s and nothing of this crate's writes a store" },
    AnEntryMet { item: 10, what_was_built: "the material takes a per-kind reload by `Dependency Upload & Remap` M9c's one edit; the five kinds' silent consequence is recorded at that crate's site and not guarded against here" },
    AnEntryMet { item: 11, what_was_built: "Simplify's four render-half pairs composed exactly where the payload carries them; the launch route for two of the three stays inferred and is not claimed" },
    AnEntryMet { item: 12, what_was_built: "the unnamed-scene branch composes no scene argument and the manifest states the branch; nothing is built on what an interface-saved file records" },
    AnEntryMet { item: 13, what_was_built: "no limbs; the document's own answer stands — nothing in this crate is conditional on a system, and Windows is recorded as untried" },
    AnEntryMet { item: 14, what_was_built: "no *what holds*; closed — the field is `Job files and outputs` M1's and `Rendering` S5a composes it (`crate::submission::the_reporting_field`)" },
    AnEntryMet { item: 15, what_was_built: "no limbs; every machine figure here is carried as the plan's own and nothing reads one as a description of rented machines" },
    AnEntryMet { item: 16, what_was_built: "Fact 105 permanently unresolved: the confirmation is its own judgement on every attempt and the told line is written only where a machine route carried the value (`crate::versions`, `crate::diary`)" },
    AnEntryMet { item: 19, what_was_built: "L8a's switch at the head by construction; *Reports 12* records it working before the script argument and doing nothing after; the position was never varied and one run at Build would settle it" },
    AnEntryMet { item: 20, what_was_built: "nothing infers from a device list; the technology named at launch overrides both preference sets (`crate::technology`)" },
    AnEntryMet { item: 23, what_was_built: "the two boxes' refusals state the range alone (`crate::again`)" },
    AnEntryMet { item: 24, what_was_built: "the opening machine where more than one is connected and none browsed is a named absence, not an invented rule (`crate::again::WhereTheBrowserOpens`)" },
    AnEntryMet { item: 25, what_was_built: "L10a's name check after every settings pair, on the name and never the value; untried in any version and one run at Build would settle it" },
    AnEntryMet { item: 28, what_was_built: "the ask sits at the head with the other process options and never after the bare double dash, on L8a's footing rather than on an observation; no position is claimed from evidence, and the one run at Build that settles item 19 settles this in the same run (`crate::line`)" },
];

/// **Items 17, 18, 21, 22 and 26 are closed in the plan and are built as
/// closed**: `Job files and outputs` C18a's rule at `crate::pipeline`, `Job
/// files and outputs` D9a's route rule at `crate::material`, `Render
/// Settings` Y15a's condition at `crate::again`, and item 26's
/// withdrawal at `crate::submission::the_reporting_field`. **Item 8 is
/// permanently unresolved on its housekeeping half and nothing here reads
/// presence as state.** **Item 27 is S3a's own residue and is
/// `crate::submission::TheGate::NothingToJudgeAgainstSoTheAttemptDecides`.**
pub const THE_CLOSED_ENTRIES_ARE_BUILT_AS_CLOSED: [u8; 7] = [8, 17, 18, 21, 22, 26, 27];

/// **Two contradictions in the plan, recorded at their sites and reported
/// to the runner as faults to route — neither repaired here.**
///
/// **BOTH WERE TAKEN BY THE PLANS, and this list is kept rather than
/// deleted because it is what the code was built to.** Checked on
/// 2026-09-04 by the builder of this crate's acting half, who was told two
/// of three reported faults were fixed and one was not, and found all three
/// fixed:
///
/// - **The first is repaired at `Rendering` §8**, which now reads
///   *"Corrected 2026-09-04 … The reconciliation half of `Queue & History`
///   H7 is answered and its §18 item 17 records the close"*. **The code was already built to
///   K13, so nothing moves.**
/// - **The second is repaired in both places.** `Rendering` S5a carries its
///   own correction of 2026-08-31, and `Telegram` V1a carries the same one
///   in place — *"`Telegram` V2 names no such set … the set itself was never
///   in doubt and nothing moves."* **The code was already built to what
///   `Telegram` V1a requires, so nothing moves here either.**
/// - **The third, L8's list ordering the device-line ask past the bare
///   double dash, is struck in the plan**, which is recorded at
///   [`THE_ASKS_POSITION_IN_L8S_LIST_IS_REPORTED_AS_A_FAULT`].
pub const THE_TWO_CONTRADICTIONS: [&str; 2] = [
    "K13 says the reconciliation-written half of `Queue & History` H7 was owed by closed documents and is now answered at `Rented-server runtime` S11 to S13; §8's paragraph to `Queue & History` still says the reconciliation half of `Queue & History` H7 remains the seam its §18 item 17 records. Built to K13 — `crate::kind::THE_RECONCILIATION_HALF_OF_H7_IS_ANSWERED_AT_RENTED_SERVER_RUNTIME_S11_TO_S13`",
    "S5a records that `Telegram` V1a cites *the closed set `Telegram` V2 names* and that `Telegram` V2 names no such set. Built to what `Telegram` V1a requires — the field on every manifest — at `crate::submission::the_reporting_field`; the mis-citation is `Telegram`'s to repair",
];

/// **One fault in the plan's own list, found by building it, and reported
/// rather than patched around**: L8 ordered the device-line ask as item 10
/// after the device argument at item 9, which sits after the bare double
/// dash — where L8's own third edge says Blender reads nothing of its own.
/// It was built at the head with the other process options
/// (`crate::line`). **The plan took the repair on 2026-09-04**: item 10 is
/// struck, the ask's place is stated below the list at the head, and what
/// is left of it is §9 item 28. The code is unchanged by the repair.
pub const THE_ASKS_POSITION_IN_L8S_LIST_IS_REPORTED_AS_A_FAULT: bool = true;

/// **A3 reopens a spine question and routes it**; nothing is built for it
/// (`crate::again::A3_ROUTES_A_SPINE_QUESTION_AND_NOTHING_IS_BUILT_FOR_IT`).
pub const A3_IS_A_ROUTING_NOTE: bool = true;

/// **Two runs a rented machine could answer later**, named for the runner
/// and built around now: §9 item 19 (where L8a's switch sits) and item 25
/// (L10a's name check, never run in any version).
pub const THE_TWO_RUNS_A_RENTED_MACHINE_COULD_SETTLE: [u8; 2] = [19, 25];

/// One clause of the plan that nobody has built, and what removes it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AClauseStillOwed {
    pub clause: &'static str,
    /// What it asks for.
    pub what: &'static str,
    /// Why it is not built, and what breaks while it stands.
    pub why: &'static str,
    /// **What removes it** — the piece that builds it, named where it is
    /// written down.
    pub what_removes_it: &'static str,
}

/// **The plan's clauses that are owed, carried here so a reader of this
/// crate meets the gap rather than discovering it** — the shape given to
/// `Server protocol and transfers` B8b, on that crate's own
/// `arriving_later.rs`, the day its plan gained it. Added 2026-10-17.
///
/// **Nothing in either folder of this module builds an entry here**, and a
/// test says so: the day a working file names one of these clauses, the test
/// goes red and the entry comes off as built.
///
/// **Empty since 2026-10-17, later the same day.** L17a was its one entry and
/// is built: the eleventh notes' video piece hands the map's video shape to the
/// line in `Rendering — machine`, and a job with a video-shaped destination is
/// started as its first frame, its last and the animation instruction
/// (`Argument::AnimationOverOneRun` in that folder's `line.rs`), once the Blender fact
/// was settled by the build runner's desk test of that day. The entry read:
/// *a job with any video-shaped destination is the one exception to L17's
/// explicit list … in whatever form writes the video whole*, owed *until a
/// Blender fact is in*, removed by *the build piece for the Creator's eleventh
/// notes, note 1*.
pub const THE_CLAUSES_STILL_OWED: [AClauseStillOwed; 0] = [];

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use super::*;

    fn the_plan() -> String {
        fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../Preparation/Architecture/Rendering.md"),
        )
        .expect("the plan is readable")
    }

    fn opens_a_clause(plan: &str, clause: &str) -> bool {
        plan.lines()
            .any(|line| line.trim_start().starts_with(&format!("**{clause}.")))
    }

    #[test]
    fn every_clause_left_to_the_acting_half_is_a_clause_the_plan_defines() {
        let plan = the_plan();
        for act in THE_ACTING_HALF {
            for clause in act.clauses {
                assert!(
                    opens_a_clause(&plan, clause),
                    "{clause} is named at {} and the plan defines no such clause",
                    act.section
                );
            }
        }
    }

    /// **Every entry met is an entry §9 numbers**, read off the plan.
    #[test]
    fn every_entry_met_is_numbered_in_section_nine() {
        let plan = the_plan();
        let section = plan
            .split("## 9. Unresolved")
            .nth(1)
            .and_then(|rest| rest.split("## 10. Moments").next())
            .expect("§9 is in the plan");
        let numbered: Vec<u8> = section
            .lines()
            .filter_map(|line| {
                let trimmed = line.trim_start();
                let digits: String = trimmed.chars().take_while(char::is_ascii_digit).collect();
                (!digits.is_empty() && trimmed[digits.len()..].starts_with(". "))
                    .then(|| digits.parse().ok())
                    .flatten()
            })
            .collect();
        for entry in THE_ENTRIES_MET {
            assert!(
                numbered.contains(&entry.item),
                "item {} is not in §9",
                entry.item
            );
            assert!(!entry.what_was_built.is_empty());
        }
        for item in THE_CLOSED_ENTRIES_ARE_BUILT_AS_CLOSED {
            assert!(numbered.contains(&item), "item {item} is not in §9");
        }
        assert_eq!(numbered.len(), 29, "§9 has twenty-nine entries");
    }

    /// **Every act names a module this crate really has**, read off the
    /// sources rather than trusted.
    #[test]
    fn every_act_landed_in_a_module_this_crate_defines() {
        assert_eq!(WHERE_EACH_ACT_LANDED.len(), THE_ACTING_HALF.len());
        // **Both folders' acting halves since 2026-09-19**, when the render
        // worker's acts moved to `smoozy-rendering-machine`; they are
        // re-exported here at the paths the list names.
        let here = Path::new(env!("CARGO_MANIFEST_DIR"));
        let modules: Vec<String> = fs::read_dir(here.join("src/acting"))
            .expect("the acting half is there")
            .chain(
                fs::read_dir(here.join("../Rendering — machine/src/acting"))
                    .expect("the render worker's acting half is there"),
            )
            .map(|entry| {
                entry
                    .expect("the entry is readable")
                    .path()
                    .file_stem()
                    .and_then(|name| name.to_str())
                    .expect("the file has a name")
                    .to_string()
            })
            .collect();
        for landed in WHERE_EACH_ACT_LANDED {
            for named in landed.split(" and ") {
                let module = named
                    .trim()
                    .trim_start_matches("acting::")
                    .split("'s")
                    .next()
                    .expect("a name");
                assert!(
                    modules.iter().any(|have| have == module),
                    "{module} is named and this crate has no such module"
                );
            }
        }
        assert!(!THE_ONE_THING_STILL_ABSENT.is_empty());
    }

    /// **No file of this crate has a `main`.**
    ///
    /// ~~The one thing still absent is a program for the rented machine, and
    /// this is what says none was invented here.~~ **Struck 2026-09-23: the
    /// program has existed since step 8** and is `smoozy-server`, which the
    /// paragraph on [`THE_ONE_THING_STILL_ABSENT`] already corrected on
    /// 2026-09-07 while this sentence went on saying the older thing. **What
    /// the test is for is unchanged**: this crate is a library the supervisor
    /// calls, and a `main` here would be a second program.
    #[test]
    fn no_file_of_this_crate_carries_a_program_of_its_own() {
        for (path, text) in crate::markers::tests::every_source_of_this_crate() {
            for line in text.lines() {
                let trimmed = line.trim_start();
                assert!(
                    !trimmed.starts_with(&["fn ", "main("].concat()),
                    "{path}: {line}"
                );
            }
        }
    }

    /// **Every name this record cites is one this crate really defines**,
    /// read off the sources rather than trusted.
    ///
    /// **Added 2026-09-05 out of Round 1 of this crate's code audit, whose
    /// fifth must-do was a claim on this page that was not true.** The
    /// §6.1–6.4 entry listed the carry-on reading among the acts that
    /// landed, under a header saying nothing on the list is stubbed, while
    /// nothing in this crate walked a job output folder or read an attempt
    /// record. **The guard that stood then read the module names alone**, and
    /// a module of that name existed, so it passed over the claim entirely.
    ///
    /// **What this one does instead**: every `crate::…` path written into an
    /// act's *why* has to name something these sources define, so the record
    /// cites the code that answers it rather than the folder it sits in — and
    /// a rename or a removal fails here rather than leaving a claim standing
    /// over nothing.
    ///
    /// **What it still cannot see, said rather than glossed**: it reads
    /// definitions and never calls, so a name that exists and nothing reaches
    /// passes. That is the round's seventh must-do rather than this one, and
    /// no test here claims to hold it.
    #[test]
    fn every_name_this_record_cites_is_one_this_crate_defines() {
        let sources = crate::markers::tests::every_source_of_this_crate();
        let all: String = sources
            .iter()
            .map(|(_, text)| text.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        let mut cited = 0;
        for act in THE_ACTING_HALF {
            for piece in act.why.split('`') {
                let Some(path) = piece.strip_prefix("crate::") else {
                    continue;
                };
                let named = path
                    .rsplit("::")
                    .next()
                    .expect("a path has a last segment")
                    .trim_end_matches(|c: char| !(c.is_ascii_alphanumeric() || c == '_'));
                if named.is_empty() {
                    continue;
                }
                cited += 1;
                let defined = ["fn ", "struct ", "enum ", "const ", "type ", "trait ", "mod "]
                    .iter()
                    .any(|opener| all.contains(&format!("{opener}{named}")))
                    || sources.iter().any(|(file, _)| {
                        file.ends_with(&format!("{named}.rs"))
                    });
                assert!(
                    defined,
                    "{} cites `crate::{path}` and this crate defines no `{named}`",
                    act.section
                );
            }
        }
        assert!(cited >= 20, "only {cited} names were read off the record");
    }

    /// **Every owed clause is one the plan defines, and no working file of
    /// this module names it** — so the guard counting the plan's clauses
    /// counts it, and nothing claims it is built. When a piece builds one,
    /// its working file names the clause, this goes red, and the entry comes
    /// off [`THE_CLAUSES_STILL_OWED`]. Added 2026-10-17, for L17a.
    #[test]
    fn every_owed_clause_is_the_plans_and_no_working_file_claims_it() {
        let plan = the_plan();
        let sources = crate::markers::tests::every_source_of_this_crate();
        for owed in THE_CLAUSES_STILL_OWED {
            assert!(opens_a_clause(&plan, owed.clause), "{} is owed and the plan defines no such clause", owed.clause);
            assert!(!owed.what.is_empty() && !owed.why.is_empty() && !owed.what_removes_it.is_empty());
            for (path, text) in &sources {
                if path.ends_with("arriving_later.rs") && path.contains("/Rendering/src/")
                    || path.ends_with("markers.rs") && path.contains("/Rendering/src/")
                {
                    continue;
                }
                let names_it = text.match_indices(owed.clause).any(|(at, _)| {
                    let before = text[..at].chars().next_back();
                    let after = text[at + owed.clause.len()..].chars().next();
                    !before.is_some_and(|c| c.is_ascii_alphanumeric() || c == '_')
                        && !after.is_some_and(|c| c.is_ascii_alphanumeric() || c == '_')
                });
                assert!(
                    !names_it,
                    "{path} names {}, which this page carries as owed — if it is built, take it off \
                     THE_CLAUSES_STILL_OWED and move the count in `markers`",
                    owed.clause
                );
            }
        }
    }

    #[test]
    fn nothing_on_the_list_is_stubbed_and_the_records_are_named() {
        for act in THE_ACTING_HALF {
            assert!(!act.what.contains("stub"), "{}", act.section);
            assert!(!act.why.is_empty());
        }
        assert_eq!(THE_TWO_CONTRADICTIONS.len(), 2);
        assert_eq!(THE_TWO_RUNS_A_RENTED_MACHINE_COULD_SETTLE, [19, 25]);
        const _: () = assert!(THE_ASKS_POSITION_IN_L8S_LIST_IS_REPORTED_AS_A_FAULT);
        const _: () = assert!(A3_IS_A_ROUTING_NOTE);
    }
}
