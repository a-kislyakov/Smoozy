//! **The one shape a failure is drawn in, and the rules every drawing inside it
//! keeps** — `Desktop app and delivery` §16.
//!
//! **READ THIS BEFORE ADDING A DRAWING. It is the shape rule for pieces D, E
//! and F of step 10a and for the eleven screens after them, and it is written
//! here rather than only in the decisions record because this is the file
//! somebody opens when they need it.** Piece A did the same at the head of
//! `src/surface/mod.rs` and it worked; this is that habit kept.
//!
//! # Why the folder exists
//!
//! Until 2026-09-09 every module in Smoozy raised failures and outcomes and
//! **nothing anywhere put one in front of a person.** Design states the shape
//! once — *"A failure has one shape wherever it attaches, and this is where it
//! is stated"* — and the mockup then draws that shape **eleven times, in five
//! modules, with five different class prefixes** (`qfail`, `svfail`, `tffail`,
//! `rdfail`, `cstate .cbad`), because a mockup is one static file with no
//! components in it. **A code base that copied the mockup would have five
//! failure shapes and Design's sentence would be false the day the second one
//! drifted.** So there is one shape, it is [`the_failure_shape`], and a drawing
//! fills it rather than building one.
//!
//! # The rule
//!
//! 1. **A drawing composes a [`the_failure_shape::TheFailureShape`] and hands
//!    it over. It never composes a failure of its own.** The struct has a field
//!    for every part a drawing supplies and no field for any part it does not:
//!    the red dot, the cross and *Report a problem…* are the shape's and cannot
//!    be added, removed or reordered from outside.
//! 2. **A new drawing gets its own file in this folder**, named for what it
//!    draws rather than for the screen it appears on, with **one `pub mod` line
//!    in the list below**. That is a one-line change in a file whose other lines
//!    nobody else is touching, which merges. `src/surface/` and `web/` follow
//!    piece A's rule unchanged.
//! 3. **The two sentences are two strings and are never joined.** X2: *a piece
//!    of prose is one string as handed in*; X10: where the orphan rule must hold
//!    inside a multi-sentence block, **the caller hands each sentence as its own
//!    string**. The shape has two fields for that reason.
//! 4. **A name is not a sentence, and joining the two is the trap of this whole
//!    step.** X7 forbids the typography point touching a job's name, a server's
//!    name, a path, a host, a port or anything the user can copy. **A drawing
//!    that writes `format!("{} could not be reached", server_name)` and hands
//!    the result to the point has broken X7, and no test anywhere would notice**
//!    — the string is prose-shaped and the substitution lands silently inside a
//!    name. Piece G carries two strings on two routes for exactly this reason
//!    and so does this folder: the sentence goes in
//!    `what_did_not_happen`, the name goes in `the_values_beside_it`, and
//!    [`tests::no_drawing_builds_prose_out_of_a_value`] fails on the join.
//! 5. **Every reading carries its own moment and the clock it was taken on**,
//!    and is presented *as of that moment* — M3a. [`as_of_that_moment`] is the
//!    only constructor, and it takes the moment. A figure that sits on screen
//!    beside an honest state with nothing saying it is frozen is the exact
//!    failure that clause was written for.
//! 6. **Cite the clause.** Every piece of code here that keeps a clause names
//!    it by document and marker. `M1` is defined in eight documents.
//!
//! # What must never be done in this folder
//!
//! - **Never open a connection, take a reading, or reach a machine.** M1:
//!   *"Presentation reads records and never sources … Nothing on a presentation
//!   path opens a connection, takes a reading, or reaches out to a machine"*,
//!   which is `State, identity and recovery` §8.4's rule and P9's.
//!   **[`tests::nothing_on_a_presentation_path_can_reach_a_machine`] is how that
//!   is held**: it reads every file in this folder and fails on any of the ways
//!   into a process, a socket or a connection. It is a text check and it is
//!   deliberate — the honest alternative would be a crate boundary, and this
//!   folder shares a crate with the adapters by §5's own design.
//! - **Never hold a second copy of a condition's current value.** M6, and
//!   `State, identity and recovery` F5's *no second copy of the current value
//!   and no independent notion of whether the condition still holds*. The
//!   observed value is borrowed for the length of one call in
//!   [`the_failure_shape::TheFailureShape::presented`] and is not kept
//!   anywhere. **This is the fault this whole stage keeps meeting — a fact kept
//!   in two places — and it has cost an agent's whole run once and two days of
//!   red builds twice.**
//! - **Never add a watcher, a sweep, a poll or a timer.** M7 carries the
//!   narrowing rather than closing it: *"Where nothing has refreshed a
//!   condition, the record stands until the user clears it … nothing here adds
//!   a watcher to close it."* P9 and P10 bind the page side too.
//! - **Never compose a third statement out of two records.** M7a: two
//!   conditions standing at once are presented as two, whatever set them off,
//!   and **what is refused is a statement about a cause the two share** — no
//!   supplier reports one and no column withdraws one, so asserting it would be
//!   the presentation holding a state no record carries. M31 is the same rule
//!   about two readings.
//! - **Never present a job as succeeded because its exit status was 0.** M12,
//!   and it is piece D's to keep at the drawing: twenty endings report success
//!   and six of them end at 0 with an empty output folder.
//! - **Never lift a line out of the ordinary stream, and never put anything
//!   composed from it where the reason is said.** M13b, and it is the sharpest
//!   thing this piece found: **the mockup does exactly that, twice.** See
//!   *What the drawing does that a clause forbids* below.
//! - **Never say a failure is finished before its outcome is durable.** M3 and
//!   P7. The queue's two doors already make the ordering unavailable rather
//!   than discouraged — [`crate::events::TheOneQueue::write_then_post`].
//!
//! # What the drawing does that a clause forbids, recorded here because Rule 10
//! closed the direction
//!
//! **`Preparation/Design/Mockup/Smoozy.html` line 5514 draws a job's failure
//! with the last line of Blender's ordinary output set beside the plain
//! sentence** — `Fra:1 Mem:9.84G | Sample 96/2048`, in a `.log .last` span,
//! directly under *"This render reported no errors, and nothing in Blender's
//! own output says why it ended."* **M13b forbids that in as many words**: *"No
//! line of that stream is lifted out of it. No excerpt, no last line, no
//! highlight, no leading summary. **The last line is a sampling line**, and
//! setting it beside a plain sentence about a failure asserts a cause **by
//! placement** that no record carries."* Line 5476 does the same with the
//! complaints stream's last line, which M13b's second bullet reaches by a
//! different route — *nothing composed from that stream reaches the surface
//! where the reason is said*.
//!
//! **The clause wins** (Rule 10; `What Build inherits` item 5). **So the shape
//! has no field an excerpt could travel in**: `the_raw_text` is a list of
//! **controls that open the record**, each named for the stream it came from
//! (M13a), and there is nowhere in [`the_failure_shape::Drawn`] to put a line
//! of a stream. **Piece D inherits that and should not add one.**
//!
//! # Where each part is
//!
//! - [`the_presentation_key`] — Q1's key, which is what said-once, the
//!   notification replacement and this shape all turn on. **Piece G wrote it
//!   and parked it in `src/notifications/`; it moved here unchanged.**
//! - [`the_failure_shape`] — the shape itself: M4 to M7a, Design's order, the
//!   two permitted additions, the collapsed form.
//! - [`outcomes_presented`] — M23 to M27, read off the outcome record.
//! - [`the_two_hooks`] — M32's diary capture and M33's report handover.
//! - [`as_of_that_moment`] — M3a, and it binds every piece after this one.
//! - [`declarations`] — §4 A7's route, for the two of the five that are this
//!   section's.
//! - [`the_underway_shape`] — §16.5 M18 to M22, and Design's three shapes.
//! - [`the_repairing_server`] — §16.7 M28 to M31a, the three readings, and the
//!   moment the countdown on the page ticks against.
//! - [`the_machines_own_words`] — §16.3 M8 to M13b, the two streams kept apart,
//!   and the five readings a tab can carry.
//! - [`the_disk_full_carve_out`] — §16.4 M14 to M17, the one diagnosis Smoozy
//!   names on a failed render, and the reading that arrives carried.
//!
//! **The record side is not here.** `State, identity and recovery` §8's store
//! is [`crate::record::failure_records`], because A2 gives no module a durable
//! store of its own and §5 builds the one there is. **Read that file's head
//! before you assume the table should not exist** — piece C's brief was told a
//! reading that says it should not, and §8 says the opposite in five places.
//!
//! # How a clause marker is written in this crate
//!
//! A marker with no document beside it means a clause of this crate's own plan,
//! `Preparation/Architecture/Desktop app and delivery.md`. Every other
//! document's marker is written with that document — at the site, or in that
//! file's own header list.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `State, identity and recovery` §8.4.
//!
//! **`State, identity and recovery`'s own markers are deliberately not listed
//! above and are written out with that document at every site instead** — this
//! plan defines clauses of the same short names, and a header list cannot tell
//! the two apart, so a bare one would resolve to a real and wrong clause.

pub mod as_of_that_moment;
pub mod declarations;
pub mod outcomes_presented;
pub mod said_once;
pub mod the_disk_full_carve_out;
pub mod the_failure_shape;
pub mod the_machines_own_words;
pub mod the_presentation_key;
pub mod the_repairing_server;
pub mod the_two_hooks;
pub mod the_underway_shape;

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};

    fn this_folder() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("src/presentation")
    }

    /// Every file in this folder, as (name, text).
    fn the_files() -> Vec<(String, String)> {
        let mut out = Vec::new();
        for entry in fs::read_dir(this_folder()).expect("the folder is readable") {
            let path = entry.expect("the entry is readable").path();
            if path.extension().and_then(|e| e.to_str()) != Some("rs") {
                continue;
            }
            out.push((
                path.file_name()
                    .and_then(|s| s.to_str())
                    .expect("the file has a name")
                    .to_string(),
                fs::read_to_string(&path).expect("the file is readable"),
            ));
        }
        out.sort();
        out
    }

    /// Every file in this folder **except this one**, as (name, text).
    ///
    /// **`mod.rs` is left out of the prohibition sweeps below on purpose**, and
    /// the reason is rule 2 of the layout: **a drawing gets its own file** and
    /// this file holds the module list and the prohibitions themselves. A sweep
    /// that read this file would fail on its own forbidden-words list, and the
    /// only way to write the list would be to stop stating it — which is the
    /// same trap piece A's page checks meet and solve by reading past comments.
    fn the_drawings() -> Vec<(String, String)> {
        the_files()
            .into_iter()
            .filter(|(name, _)| name != "mod.rs")
            .collect()
    }

    /// The file's text with its comment lines read past. **Every prohibition in
    /// this folder is written down beside the code that keeps it**, and a check
    /// that could not tell a prohibition from an instance would be one nobody
    /// could write the prohibition beside — the same reason piece A's page
    /// checks read past comments.
    fn past_the_comments(text: &str) -> String {
        text.lines()
            .filter(|line| {
                let line = line.trim_start();
                !(line.starts_with("//") || line.starts_with('*') || line.starts_with("/*"))
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// **Every file in this folder is in the module list above.**
    ///
    /// A drawing written into this folder and left out of the list is a drawing
    /// nothing compiles: the crate builds, every test that file holds is never
    /// run, and nothing anywhere says why. This is the line that fails instead.
    #[test]
    fn every_file_of_the_folder_is_in_the_module_list() {
        let list = include_str!("mod.rs");
        for (name, _) in the_files() {
            if name == "mod.rs" {
                continue;
            }
            let module = name.trim_end_matches(".rs");
            assert!(
                list.contains(&format!("pub mod {module};")),
                "src/presentation/{name} is not declared in src/presentation/mod.rs. \
                 **A drawing adds exactly one line to that list** — the rule is at \
                 the head of this file"
            );
        }
    }

    /// **§16.1 M1: nothing on a presentation path opens a connection, takes a
    /// reading, or reaches out to a machine.**
    ///
    /// **This is M1 held structurally rather than currently**, which is what
    /// piece C's brief asked for. The check is over the text of every file here
    /// and its list is the ways into a process, a socket, a connection or a
    /// transfer — the four things a presentation could reach a machine by.
    ///
    /// **Why a text check and not a crate boundary.** The honest structural
    /// answer would be a crate of its own with no dependency that can reach a
    /// machine. This folder is inside `Desktop app and delivery`'s crate because
    /// §5, §6 and §9 put the record, the adapters and the services there, and
    /// splitting the crate to hold one rule would be a change to the shape of
    /// the whole application for one clause. **So the boundary is this test, and
    /// it is written out so nobody mistakes it for a compiler's answer.**
    #[test]
    fn nothing_on_a_presentation_path_can_reach_a_machine() {
        for (name, text) in the_drawings() {
            let text = past_the_comments(&text);
            for reaching in [
                "std::process",
                "std::net",
                "Command::new",
                "TcpStream",
                "UdpSocket",
                "russh",
                "ssh2",
                "reqwest",
                "crate::transfers",
                "crate::scheduling",
                "smoozy_server_protocol",
                "smoozy_servers",
            ] {
                assert!(
                    !text.contains(reaching),
                    "src/presentation/{name} names {reaching}. **M1: presentation \
                     reads records and never sources, and nothing on a \
                     presentation path opens a connection, takes a reading or \
                     reaches out to a machine** — `State, identity and recovery` \
                     §8.4's rule and P9's. A reading arrives here already taken, \
                     carried with its own moment (M3a)"
                );
            }
        }
    }

    /// **X7 and X8: a name is not prose, and joining one into a sentence breaks
    /// X7 with no test noticing.**
    ///
    /// The failure is silent and it is the trap of this whole step: a job's
    /// name, a server's name or a path joined into a sentence before the
    /// typography point gets a non-breaking space substituted **inside the
    /// name**, and the string that reaches the screen is a value Smoozy quietly
    /// changed. Piece G carries two strings on two routes for this reason.
    ///
    /// **What this catches is the join**, which is the shape the mistake takes:
    /// a formatted string handed straight to the point.
    #[test]
    fn no_drawing_builds_prose_out_of_a_value() {
        for (name, text) in the_drawings() {
            let text = past_the_comments(&text);
            assert!(
                !text.contains("Prose::new(format!"),
                "src/presentation/{name} builds a piece of prose with `format!`. \
                 **X7: the point never touches a name, a path, a host or a port, \
                 and X8 says a builder meeting an ambiguous case treats the \
                 string as data.** A name joined into a sentence before the point \
                 is a name with a non-breaking space substituted inside it, and \
                 nothing anywhere reports it. **Two strings on two routes**: the \
                 sentence in `what_did_not_happen`, the name in \
                 `the_values_beside_it`"
            );
        }
    }

    /// **M6 and `State, identity and recovery` F5: nothing here holds a second
    /// copy of a condition's current value.**
    ///
    /// The observed value is borrowed for the length of one call and never
    /// stored, so this catches the one shape a copy would take — a field named
    /// for the current value.
    #[test]
    fn nothing_here_holds_a_conditions_current_value() {
        for (name, text) in the_drawings() {
            let text = past_the_comments(&text);
            for held in ["current_value", "observed_value:", "last_observed"] {
                assert!(
                    !text.contains(held),
                    "src/presentation/{name} holds a field named {held}. **M6: \
                     nothing here holds a second copy of a condition's current \
                     value** — the record is a derivation, and presenting a copy \
                     is what would let the mark and the world drift apart, which \
                     is the harm `Preparation/Specs/Whole app.md` names: *the app \
                     being wrong on screen about its own state*"
                );
            }
        }
    }

    /// **M7, P9 and P10: nothing here watches, sweeps, polls or ticks.**
    ///
    /// `State, identity and recovery` F26 rules out *no age, no cap, no sweep,
    /// no schedule, and nothing that reaches out to a machine to look*, and M7
    /// says outright that the narrowing is carried rather than closed by adding
    /// a watcher.
    #[test]
    fn nothing_here_watches_or_ticks() {
        for (name, text) in the_drawings() {
            let text = past_the_comments(&text);
            for ticking in [
                "thread::spawn",
                "sleep(",
                "Instant::now",
                "set_interval",
                "tokio::spawn",
            ] {
                assert!(
                    !text.contains(ticking),
                    "src/presentation/{name} names {ticking}. **M7: nothing here \
                     adds a watcher**, and P9 and P10 leave this application one \
                     timer, which is the update check"
                );
            }
        }
    }

    /// **M12: no presentation says a job succeeded because its status was 0.**
    ///
    /// **Twenty endings report success and six of them end at 0 with an empty
    /// output folder**, and three of this stage's own observations from outside
    /// Blender — a full disk, a purged driver and a mismatched driver — each
    /// finished at 0 as well. *"A completeness gate reading only the status
    /// would call every one of those six a finished job with its frames
    /// written."*
    ///
    /// **The word is held out of the folder rather than out of one drawing**,
    /// and so is the status it would come from: nothing here takes an exit
    /// status, so there is nothing for the word to be reachable from. Added by
    /// piece D on 2026-09-12, beside the four sweeps piece C left.
    ///
    /// **This one reads each file's working half rather than the whole file**,
    /// which the four above do not need to. The reason is the same one
    /// [`past_the_comments`] exists for: a test that asserts the word is absent
    /// has to write the word down, and a sweep that could not tell a
    /// prohibition from an instance would be one nobody could write the
    /// prohibition beside.
    #[test]
    fn nothing_here_says_a_job_succeeded() {
        for (name, text) in the_drawings() {
            let working = text
                .split("#[cfg(test)]")
                .next()
                .expect("the file has a working half")
                .to_string();
            let text = past_the_comments(&working).to_lowercase();
            for claiming in ["succeed", "success", "exit_status", "exit_code"] {
                assert!(
                    !text.contains(claiming),
                    "src/presentation/{name} names {claiming}. **M12: no \
                     presentation says a job succeeded because its status was \
                     0** — twenty endings report success and six of them end at \
                     0 with an empty output folder, so the word must not be \
                     reachable from a status code in this folder"
                );
            }
        }
    }

    /// **Every file names the section it keeps.**
    ///
    /// `crate::markers` already holds the harder half — that no marker here
    /// stands bare that belongs to another document. What this adds is the
    /// weaker and earlier one: a file in this folder that names no section at
    /// all is a drawing nobody can trace back to a promise, and the counted
    /// gate's reverse leg is what would otherwise find it at the end of the
    /// step rather than at the moment it was written.
    #[test]
    fn every_file_names_the_section_it_keeps() {
        for (name, text) in the_files() {
            assert!(
                text.contains('§'),
                "src/presentation/{name} names no section of any document. **A \
                 marker alone is unrecoverable — `M1` is defined in eight \
                 documents** — so a file here says which document and which \
                 section it is keeping"
            );
        }
    }
}
