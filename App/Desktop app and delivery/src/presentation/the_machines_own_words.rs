//! **The plain sentence, and the machine's own words behind it** — §16.3, M8 to
//! M13b, and `Preparation/Design/Whole app.md`'s *The failure shape* and its
//! panel.
//!
//! **Read the folder's head first.** This file draws inside the shape stated at
//! `src/presentation/mod.rs` and adds nothing to it.
//!
//! ## What this file is
//!
//! A render that ended badly has **two streams**, kept apart, unedited, and
//! read back with nobody connected. This file is the panel they are read in:
//! **one tab per stream, each under the name of the stream it came from**, and
//! **every tab says something a person can read**.
//!
//! ## The order, and it is M8
//!
//! **M8: the plain sentence is first and the raw record is behind it, never the
//! first thing in front of the user.** That is kept by where the panel is
//! reachable from rather than by anybody remembering it: [`ThePanel`] has no
//! route to a surface except through the one control
//! [`the_control_that_opens_it`] makes, and that control is an entry in the
//! failure shape's `the_raw_text`, which the shape draws **after** the
//! sentence. **There is no function here that hands a panel to anything without
//! a failure to hang it on.**
//!
//! ## Nothing here edits either stream, and it is held structurally
//!
//! **M9: not a redaction, not a filter, not a prefix, not a trim.** Three
//! things hold it and none of them is a rule somebody has to remember.
//!
//! 1. **The record is one string and there is no field for a part of one.** It
//!    arrives whole (`Rented-server runtime` M11: read back whole or not at all)
//!    and travels whole.
//! 2. **It leaves by the verbatim route** ([`Verbatim`], X9), which performs no
//!    substitution of any kind. X7 names the machine's own words first in its
//!    closed list and says the exemption from the typography point is
//!    **mandatory rather than optional**.
//! 3. **[`tests::nothing_here_edits_either_stream`] reads this file's own
//!    working half** and fails on every way a string is shortened or altered in
//!    Rust. The same sweep over the page half is in
//!    `src/surface/the_machines_own_words.rs`, because the page is where a last
//!    line would actually be lifted.
//!
//! ## M13b is already kept and must stay kept
//!
//! **`Preparation/Design/Mockup/Smoozy.html` draws what M13b forbids, twice** —
//! line 5514 puts `Fra:1 Mem:9.84G | Sample 96/2048` in a `.log .last` span
//! directly under the plain sentence, and line 5476 does the same with the
//! complaints stream's last line. M13b: *"No line of that stream is lifted out
//! of it. No excerpt, no last line, no highlight, no leading summary. The last
//! line is a sampling line, and setting it beside a plain sentence about a
//! failure asserts a cause by placement that no record carries."*
//!
//! **The clause wins** (Rule 10; `What Build inherits` item 5), and piece C made
//! the repair structural: `the_failure_shape::Drawn` has no field an excerpt
//! could travel in. **This file keeps that property**: the record is whole or it
//! is not here, and [`tests::no_line_is_lifted_out_of_a_stream`] fails on a
//! field or a function that would carry a part of one.
//!
//! ## The two the row exists to separate rest on two different records
//!
//! **M11**, and it is the one clause a reader will be tempted to break by
//! kindness. *Collected, and the stream was empty* rests on what Smoozy
//! observed passing its own loop; *never established* rests on the **absence**
//! of a capture record written before the worker started. **Neither is derived
//! from a sink.** So [`a_tab`] reads the recorded outcome and **never the text**
//! — an empty record on a *collected* outcome still says *collected*, because
//! repairing that here would be re-deriving an outcome from the sink, which is
//! the one thing M11 forbids.
//!
//! ## The five readings, and none of them is an empty panel
//!
//! **M10's four** — *collected*; *collected, and the stream was empty*;
//! *collection incomplete*; *never established* — **and M10a's fifth**, an
//! outcome not yet written, which is none of the four and never reads as
//! *collected*. Design states the same set from its own side and adds the rule
//! this file is built on: *"That panel never shows an empty box."*
//!
//! ## The words, and the two this piece ruled
//!
//! §24 leaves the words the user reads to Design, and Design settles four of the
//! five outright. **Two sentences the drawing carries are not drawn here, and
//! both are rulings of 2026-09-12 with their reasoning in the decisions
//! record.**
//!
//! - The drawing's *never established* reading adds *"The render ended before
//!   Smoozy could start reading, so there is nothing to show and nothing was
//!   lost."* **That outcome has three routes** — no capture record, a worker
//!   started before its sinks, and the marker never passing — and the sentence
//!   is true of one. **M6 forbids the presentation holding a state no record
//!   carries**, so what is drawn is the reading itself and no account of it.
//! - The drawing's fifth reading adds *"The server ran out of room to write it
//!   down."* Same ground: nothing handed to this file says **why** an outcome is
//!   unwritten, and M10a states the reading as the unwritten outcome and nothing
//!   more.
//!
//! ## What is deliberately not built
//!
//! **A tab may say how many lines its own stream holds** — Design's own *may*,
//! and *"a count is not"* reading the words for a cause. **It is not built**,
//! because counting the lines means taking the record apart, which is the one
//! habit this file holds structurally against, and because no supplier hands a
//! count over. Where a count is wanted it is the machine's own to report, beside
//! the outcome, the way `Rented-server runtime` M5's retained and discarded
//! counts already are.
//!
//! ## Markers of other documents
//!
//! **Markers used in this file that belong to another document are written out
//! with that document at the site, every one of them, and none is listed here.**
//! This plan defines an E-series, an F-series, an M-series, an S-series and a
//! V-series of its own, and so do `Rented-server runtime`, `Queue & History`
//! and `Server protocol and transfers`. A header list cannot tell two apart, so
//! a bare one would resolve to a real and wrong clause.

use smoozy_queue_history::ending::CollectionOutcome;

use crate::presentation::the_failure_shape::AnAct;
use crate::typography::Verbatim;

/// **Which of the two streams a record came from** — M13, and M13a's *under the
/// name of the stream it came from*.
///
/// **The two are never merged** (M13): the complaints stream carries the failure
/// text and, in one version, a line the other version puts on the ordinary
/// stream, so anything mixing them loses which one a line came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stream {
    /// The render's own output.
    TheOrdinaryOutput,
    /// What the render complained about.
    TheComplaints,
}

impl Stream {
    /// **Both, and there is no third.** `Rented-server runtime` M3: the worker
    /// holds both of the render's streams and copies them to two sinks.
    pub const BOTH: [Stream; 2] = [Stream::TheOrdinaryOutput, Stream::TheComplaints];

    /// **The name of the stream this record came from** — M13a, and the tab's
    /// own word, which is Design's.
    pub fn the_name_of_the_stream(self) -> &'static str {
        match self {
            Stream::TheOrdinaryOutput => "Output",
            Stream::TheComplaints => "Errors",
        }
    }

    /// **M10's *collected, and the stream was empty*, worded to the stream it is
    /// about.** M10: *"the two wordings are one claim worded to the stream it is
    /// about"*, and *"it asserts nothing about the other stream"*.
    ///
    /// The complaints stream's wording is Design's own, ruled on 2026-09-02
    /// against the spec's figure, because a metaphor beside a tab named
    /// **Errors** is what Rule 9 forbids on a screen. **Reworded 2026-10-11,
    /// the Creator's eighth notes, note 6:** it read *The render reported no
    /// errors.*, which Blender 5 makes false by writing its errors to its
    /// ordinary output (M13's corrected premise). It now claims only that this
    /// stream is empty.
    fn the_render_printed_nothing_here(self) -> &'static str {
        match self {
            Stream::TheOrdinaryOutput => "The render printed no output.",
            Stream::TheComplaints => "Blender wrote nothing here.",
        }
    }
}

/// **One stream's record, as the caller hands it over.**
///
/// **Every field is supplied and none is derived here** — M23's discipline
/// applied to §16.3: the presentation reads records and composes no outcome of
/// its own.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AStreamsRecord {
    /// Which stream. **M13a: its record is presented under this name and never
    /// occupies the other's place.**
    pub stream: Stream,
    /// **`Rented-server runtime` M13's closed set of four, one per stream per
    /// attempt**, carried here through `Queue & History` E4 rather than
    /// restated — a second copy of a closed set is a second thing to keep true.
    ///
    /// **`None` is M10a**: an outcome not yet written, which is none of the four
    /// and **never presents as *collected***. It is the one state most easily
    /// lost by giving this field a default.
    pub outcome: Option<CollectionOutcome>,
    /// **The retained bytes, read back whole** (`Rented-server runtime` M11: a
    /// partial read is not a reading) **and unedited** (M9). Empty where the
    /// reading says there is nothing to read, and never drawn in that case.
    pub as_read_back: String,
    /// `Rented-server runtime` M5's retained count, where the machine reported
    /// one.
    pub kept: Option<u64>,
    /// `Rented-server runtime` M5's discarded count, where the machine reported
    /// one.
    pub dropped: Option<u64>,
    /// `Rented-server runtime` M5's first loss reason, recorded once by the
    /// worker. **The machine's own words, so a value and never prose** (X7, X9).
    pub why_the_rest_went: Option<String>,
}

/// **What one tab says** — M10's four and M10a's fifth.
///
/// **None of the five is an empty panel**, which is Design's own reason for
/// there being five sentences rather than a blank: *"That panel never shows an
/// empty box."*
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatATabSays {
    /// *collected* — **the record is the whole of what was printed on that
    /// stream**, and it is there to open.
    TheRecord {
        /// **Whole, unedited, by the verbatim route.** There is no second field
        /// here and no function that answers a part of this one — M9 and M13b.
        the_whole_of_it: Verbatim,
    },
    /// *collected, and the stream was empty* — **a recorded fact rather than an
    /// inference from an empty file** (M11).
    TheStreamWasEmpty,
    /// *collection incomplete* — **the retained bytes are a verbatim prefix and
    /// nothing says they are all of it.** `Rented-server runtime` M26: the raw
    /// record is still offered for an incomplete stream, marked as incomplete,
    /// and this **never collapses into *collected and empty***.
    OnlyPartWasRecorded {
        the_prefix: Verbatim,
        kept: Option<Verbatim>,
        dropped: Option<Verbatim>,
        why_the_rest_went: Option<Verbatim>,
    },
    /// *never established* — **nothing of the render's own printing was
    /// established**, which rests on the absence of a capture record and never
    /// on an empty sink (M11).
    NothingWasRecorded,
    /// **M10a's fifth, and it is none of the four.** Never *collected*, which
    /// would claim a record is there to open when none is; never *never
    /// established*, which is a different claim resting on a different record.
    TheOutcomeHasNotBeenRecordedYet,
}

/// **One tab of the panel.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ATab {
    /// **M13a.** The stream this record came from, and the tab's name is taken
    /// from it rather than passed in — **so the ordinary stream's record cannot
    /// be put under the complaints stream's name.**
    pub the_stream_it_came_from: Stream,
    pub says: WhatATabSays,
}

impl ATab {
    /// **M13a: the name of the stream it came from**, derived from the stream
    /// itself.
    pub fn the_name_of_the_stream(&self) -> &'static str {
        self.the_stream_it_came_from.the_name_of_the_stream()
    }

    /// **The sentences this tab says, each as its own string.**
    ///
    /// X2: *a piece of prose is one string as handed in*; X10: where the orphan
    /// rule must hold inside a multi-sentence block, **the caller hands each
    /// sentence as its own string**. So this answers a list and never a
    /// paragraph, and the caller puts each one through the typography point on
    /// its own.
    ///
    /// **A *collected* tab says no sentence and shows the record**, which is the
    /// one member of the five that is not a sentence — and it is not an empty
    /// panel either, because there is a record in it.
    pub fn the_sentences(&self) -> Vec<&'static str> {
        match &self.says {
            WhatATabSays::TheRecord { .. } => Vec::new(),
            // **One sentence, about this stream alone** — M10 as narrowed
            // 2026-10-11: *there is nothing further to read* spoke for the
            // other stream, which may hold Blender's errors.
            WhatATabSays::TheStreamWasEmpty => vec![self
                .the_stream_it_came_from
                .the_render_printed_nothing_here()],
            WhatATabSays::OnlyPartWasRecorded { .. } => vec![ONLY_PART_WAS_RECORDED],
            WhatATabSays::NothingWasRecorded => vec![NOTHING_WAS_RECORDED],
            WhatATabSays::TheOutcomeHasNotBeenRecordedYet => vec![
                THE_OUTCOME_HAS_NOT_BEEN_RECORDED_YET,
                SMOOZY_DOES_NOT_KNOW_YET,
            ],
        }
    }

    /// **Which of the five this tab is, as a token the page routes on.**
    ///
    /// A value and not prose: the page picks a shape with it and never draws
    /// it. **The five are the closed set** — M10's four and M10a's fifth — and a
    /// sixth would have to be added here, where the compiler asks for it.
    pub fn as_token(&self) -> &'static str {
        match self.says {
            WhatATabSays::TheRecord { .. } => "the-record",
            WhatATabSays::TheStreamWasEmpty => "the-stream-was-empty",
            WhatATabSays::OnlyPartWasRecorded { .. } => "only-part-was-recorded",
            WhatATabSays::NothingWasRecorded => "nothing-was-recorded",
            WhatATabSays::TheOutcomeHasNotBeenRecordedYet => "not-recorded-yet",
        }
    }

    /// **The record, where this tab has one to show.** Whole, and there is no
    /// other way to reach it.
    pub fn the_record(&self) -> Option<&Verbatim> {
        match &self.says {
            WhatATabSays::TheRecord { the_whole_of_it } => Some(the_whole_of_it),
            WhatATabSays::OnlyPartWasRecorded { the_prefix, .. } => Some(the_prefix),
            _ => None,
        }
    }
}

/// Design's own words for *collection incomplete*. **How much was kept and why
/// the rest went stand beside it as values** rather than being joined into it —
/// X7, and the folder head's rule 4.
pub const ONLY_PART_WAS_RECORDED: &str = "Partly recorded.";

/// Design's own words for *never established*.
pub const NOTHING_WAS_RECORDED: &str = "No output was recorded.";

/// Design's own words for M10a's fifth reading.
pub const THE_OUTCOME_HAS_NOT_BEEN_RECORDED_YET: &str =
    "Not recorded yet.";

/// The second sentence of M10a's reading. **It says what is not known and never
/// why**, which is the ruling at this file's head.
pub const SMOOZY_DOES_NOT_KNOW_YET: &str = "Contents unknown.";

/// **The control that opens the panel, and the label names whose words they
/// are** — Design: *"The raw text is always one control away, and the control
/// names whose text it is — Blender output, What the server said."*
pub const BLENDER_OUTPUT: &str = "Blender output";

/// **M10 and M10a, performed: the recorded outcome decides what the tab says,
/// and the text never does.**
///
/// **The outcome is read and nothing is re-derived** (M11). A `None` outcome is
/// M10a's fifth reading and is never read as any of the four.
pub fn a_tab(record: &AStreamsRecord) -> ATab {
    let says = match record.outcome {
        // **M10a first, so that no default can reach the four.**
        None => WhatATabSays::TheOutcomeHasNotBeenRecordedYet,
        Some(CollectionOutcome::Collected) => WhatATabSays::TheRecord {
            the_whole_of_it: Verbatim::new(record.as_read_back.clone()),
        },
        Some(CollectionOutcome::CollectedAndTheStreamWasEmpty) => WhatATabSays::TheStreamWasEmpty,
        Some(CollectionOutcome::CollectionIncomplete) => WhatATabSays::OnlyPartWasRecorded {
            the_prefix: Verbatim::new(record.as_read_back.clone()),
            kept: a_count(record.kept),
            dropped: a_count(record.dropped),
            why_the_rest_went: record
                .why_the_rest_went
                .as_ref()
                .map(|reason| Verbatim::new(reason.clone())),
        },
        Some(CollectionOutcome::NeverEstablished) => WhatATabSays::NothingWasRecorded,
    };
    ATab {
        the_stream_it_came_from: record.stream,
        says,
    }
}

/// A count as the page receives it. **A number is a value and takes the verbatim
/// route** (X9); the page writes it in the reader's own units, because that is a
/// drawing rather than a fact.
fn a_count(count: Option<u64>) -> Option<Verbatim> {
    count.map(|count| Verbatim::new(count.to_string()))
}

/// **The panel behind the control** — one tab per stream, apart, never merged.
///
/// **A render has two and anything with one stream has one**, which is Design's
/// *"A render has two streams and they are kept apart, so it has two tabs;
/// anything with one stream has none."* The tab strip is the page's and is drawn
/// only where there is more than one tab.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThePanel {
    /// **One per stream.** M13: the two are presented apart and are never merged
    /// into one panel.
    pub tabs: Vec<ATab>,
}

/// **The panel, composed from the records of the streams there are.**
///
/// **The caller hands one record per stream and this composes one tab each.**
/// Nothing is merged, nothing is ordered by anything but the order the streams
/// were handed in, and **nothing is weighted** — M13b's *"any ordering or
/// weighting of the words"* is not supplied by this document and is not invented
/// here.
pub fn the_panel(records: &[AStreamsRecord]) -> ThePanel {
    ThePanel {
        tabs: records.iter().map(a_tab).collect(),
    }
}

/// **The one control that opens the panel**, for the failure shape's
/// `the_raw_text`.
///
/// `what_it_opens` is the token the page routes on — a value, not prose — and
/// the label is prose the shape puts through the typography point.
///
/// **One control and not one per stream.** Design: the control names whose words
/// they are and *"the panel it opens gives a render's two streams a tab each"*.
/// M13a's *under the name of the stream it came from* is kept by the tabs, which
/// take their names from their own streams.
pub fn the_control_that_opens_it(label: &str, what_it_opens: &str) -> AnAct {
    AnAct {
        label: label.to_string(),
        act: what_it_opens.to_string(),
    }
}

/// **M12, held by there being nothing here to hold.**
///
/// *"No presentation says a job succeeded because its status was 0."* Twenty
/// endings report success and **six of them end at 0 with an empty output
/// folder**; three of this stage's own observations from outside Blender — a
/// full disk, a purged driver and a mismatched driver — each finished at 0 as
/// well. **A completeness gate reading only the status would call every one of
/// those six a finished job with its frames written.**
///
/// So **no exit status reaches this folder at all**: nothing here takes one,
/// stores one or reads one, and the word *succeeded* is therefore not reachable
/// from a status code because there is no status code to reach it from. The
/// status is carried verbatim by `Rented-server runtime` M22 and is handed on;
/// what a job's completeness is read off is `Job files and outputs` §10's.
/// `src/presentation/mod.rs`'s own folder sweep is what says the word stays out.
pub const NOTHING_HERE_READS_THE_NUMBER_A_RENDER_ENDED_WITH: () = ();

#[cfg(test)]
mod tests {
    use super::*;

    /// This file's working half, with its comments read past.
    ///
    /// **The sweeps below read this rather than the whole file.** Every
    /// prohibition here is written down beside the code that keeps it, and a
    /// check that could not tell a prohibition from an instance would be one
    /// nobody could write the prohibition beside — the same reason piece A's
    /// page checks and piece C's folder sweeps read past comments.
    fn the_working_half() -> String {
        let source = include_str!("the_machines_own_words.rs");
        let working = source
            .split("#[cfg(test)]")
            .next()
            .expect("the file has a working half");
        working
            .lines()
            .filter(|line| {
                let line = line.trim_start();
                !(line.starts_with("//") || line.starts_with('*') || line.starts_with("/*"))
            })
            .collect::<Vec<_>>()
            .join("\n")
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

    /// **M9: nothing here edits either stream — not a redaction, not a filter,
    /// not a prefix, not a trim.**
    ///
    /// **This is M9 held structurally rather than by a rule somebody must
    /// remember**, which is what this piece's brief asked for. The list is every
    /// way a string is shortened or altered in Rust, and the record is the only
    /// string this file holds.
    #[test]
    fn nothing_here_edits_either_stream() {
        let working = the_working_half();
        for editing in [
            ".trim(",
            ".trim_start(",
            ".trim_end(",
            ".replace(",
            ".replacen(",
            ".truncate(",
            ".lines(",
            ".split(",
            ".splitn(",
            ".chars(",
            ".bytes(",
            ".strip_prefix(",
            ".strip_suffix(",
            ".to_lowercase(",
            ".to_uppercase(",
            ".retain(",
            ".drain(",
            ".find(",
            ".rfind(",
            ".last(",
        ] {
            assert!(
                !working.contains(editing),
                "src/presentation/the_machines_own_words.rs names {editing}. **M9: \
                 nothing here edits either stream — not a redaction, not a filter, \
                 not a prefix, not a trim**, and X7 makes the raw record's \
                 exemption from the typography point mandatory rather than \
                 optional. The record arrives whole and travels whole"
            );
        }
    }

    /// **M13b: no line of the ordinary stream is lifted out of it.**
    ///
    /// *"No excerpt, no last line, no highlight, no leading summary. The last
    /// line is a sampling line, and setting it beside a plain sentence about a
    /// failure asserts a cause by placement that no record carries."*
    ///
    /// **The mockup does exactly that, twice**, at lines 5476 and 5514. The
    /// clause wins, and what keeps it is that there is nowhere here to put one.
    #[test]
    fn no_line_is_lifted_out_of_a_stream() {
        let working = the_working_half();
        for lifted in [
            "last_line",
            "excerpt",
            "the_last",
            "first_line",
            "summary",
            "highlight",
            "tail",
            "snippet",
        ] {
            assert!(
                !working.contains(lifted),
                "src/presentation/the_machines_own_words.rs has a {lifted}. **M13b: \
                 no line of that stream is lifted out of it — no excerpt, no last \
                 line, no highlight, no leading summary.** The record is whole or \
                 it is not here"
            );
        }
    }

    /// **M10 and Design: an empty panel is never what the user meets.**
    ///
    /// Every one of the five readings gives the user either a record or at least
    /// one sentence, and the check walks the five rather than a list kept here.
    #[test]
    fn every_one_of_the_five_readings_gives_the_user_something_to_read() {
        for stream in Stream::BOTH {
            let five = [
                a_tab(&a_record(stream, Some(CollectionOutcome::Collected), "words")),
                a_tab(&a_record(
                    stream,
                    Some(CollectionOutcome::CollectedAndTheStreamWasEmpty),
                    "",
                )),
                a_tab(&a_record(
                    stream,
                    Some(CollectionOutcome::CollectionIncomplete),
                    "some of the words",
                )),
                a_tab(&a_record(
                    stream,
                    Some(CollectionOutcome::NeverEstablished),
                    "",
                )),
                a_tab(&a_record(stream, None, "")),
            ];
            for tab in five {
                let has_a_record = tab
                    .the_record()
                    .map(|record| !record.as_str().is_empty())
                    .unwrap_or(false);
                assert!(
                    has_a_record || !tab.the_sentences().is_empty(),
                    "a tab on the {} stream draws an empty box. **M10: an empty \
                     panel is never what the user meets**, and Design: *that panel \
                     never shows an empty box*",
                    tab.the_name_of_the_stream()
                );
                for sentence in tab.the_sentences() {
                    assert!(!sentence.is_empty());
                }
            }
        }
    }

    /// **M10a: an outcome not yet written is none of the four and never presents
    /// as *collected*.**
    ///
    /// **This is the one most easily lost by giving an `Option` a default**, so
    /// it is asserted from both sides: the reading is its own, and it is not any
    /// of the four.
    #[test]
    fn an_outcome_not_yet_written_is_none_of_the_four() {
        let tab = a_tab(&a_record(Stream::TheOrdinaryOutput, None, "words"));
        assert_eq!(tab.says, WhatATabSays::TheOutcomeHasNotBeenRecordedYet);
        assert!(tab.the_record().is_none());
        for one_of_the_four in [
            CollectionOutcome::Collected,
            CollectionOutcome::CollectedAndTheStreamWasEmpty,
            CollectionOutcome::CollectionIncomplete,
            CollectionOutcome::NeverEstablished,
        ] {
            assert_ne!(
                tab.says,
                a_tab(&a_record(Stream::TheOrdinaryOutput, Some(one_of_the_four), "words")).says,
                "an unwritten outcome reads as one of `Rented-server runtime` M13's \
                 four. **M10a: it is none of them**"
            );
        }
    }

    /// **M11: the two the row exists to separate rest on two different records,
    /// and the text is not one of them.**
    ///
    /// A *collected* outcome with an empty record still says *collected*; a
    /// *never established* outcome with bytes in hand still says nothing was
    /// recorded. **Repairing either here would be re-deriving an outcome from
    /// the sink.**
    #[test]
    fn the_outcome_decides_what_a_tab_says_and_the_text_never_does() {
        let collected_but_empty = a_tab(&a_record(
            Stream::TheComplaints,
            Some(CollectionOutcome::Collected),
            "",
        ));
        assert!(matches!(
            collected_but_empty.says,
            WhatATabSays::TheRecord { .. }
        ));

        let never_established_with_bytes = a_tab(&a_record(
            Stream::TheComplaints,
            Some(CollectionOutcome::NeverEstablished),
            "something a sink happens to hold",
        ));
        assert_eq!(
            never_established_with_bytes.says,
            WhatATabSays::NothingWasRecorded
        );
    }

    /// **M11 again, from the wording's side.** *Collected and empty* and
    /// *nothing was recorded* are two claims and never one, which is the whole
    /// reason Row 286 exists.
    #[test]
    fn an_empty_stream_and_nothing_recorded_are_two_different_sentences() {
        let empty = a_tab(&a_record(
            Stream::TheComplaints,
            Some(CollectionOutcome::CollectedAndTheStreamWasEmpty),
            "",
        ));
        let never = a_tab(&a_record(
            Stream::TheComplaints,
            Some(CollectionOutcome::NeverEstablished),
            "",
        ));
        assert_ne!(empty.the_sentences(), never.the_sentences());
    }

    /// **`Rented-server runtime` M26: *collection incomplete* never collapses
    /// into *collected and empty***, and the raw record is still offered for it,
    /// marked as incomplete.
    #[test]
    fn an_incomplete_collection_still_offers_its_prefix_and_says_it_is_a_prefix() {
        let mut record = a_record(
            Stream::TheOrdinaryOutput,
            Some(CollectionOutcome::CollectionIncomplete),
            "the first part of it",
        );
        record.kept = Some(2_100_000);
        record.dropped = Some(41_800_000);
        record.why_the_rest_went = Some("No space left on device".to_string());
        let tab = a_tab(&record);
        assert_eq!(tab.the_sentences(), vec![ONLY_PART_WAS_RECORDED]);
        assert_eq!(
            tab.the_record().expect("the prefix is offered").as_str(),
            "the first part of it"
        );
        let WhatATabSays::OnlyPartWasRecorded {
            kept,
            dropped,
            why_the_rest_went,
            ..
        } = &tab.says
        else {
            panic!("an incomplete collection says so")
        };
        assert_eq!(kept.as_ref().expect("a count").as_str(), "2100000");
        assert_eq!(dropped.as_ref().expect("a count").as_str(), "41800000");
        // **The machine's own reason, by the verbatim route** — X7's first
        // entry, and Smoozy does not alter what the machine said.
        assert_eq!(
            why_the_rest_went.as_ref().expect("a reason").as_str(),
            "No space left on device"
        );
    }

    /// **M13 and M13a: two streams, two tabs, each under the name of the stream
    /// it came from, and the ordinary stream's record never occupies the
    /// complaints stream's place.**
    ///
    /// The case is M13a's own: on the ending the spec names, the complaints
    /// stream is *collected, and the stream was empty* and the ordinary stream
    /// is *collected*.
    #[test]
    fn the_two_streams_take_two_outcomes_each_under_its_own_name() {
        let panel = the_panel(&[
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
        ]);
        assert_eq!(panel.tabs.len(), 2);
        assert_eq!(panel.tabs[0].the_name_of_the_stream(), "Output");
        assert_eq!(panel.tabs[1].the_name_of_the_stream(), "Errors");
        assert!(panel.tabs[0].the_record().is_some());
        assert!(panel.tabs[1].the_record().is_none());
        assert_eq!(panel.tabs[1].the_sentences(), vec!["Blender wrote nothing here."]);
    }

    /// **M13a: a tab's name comes from its own stream and cannot be passed
    /// in.** There is no route by which one stream's record is drawn under the
    /// other's name.
    #[test]
    fn a_tabs_name_is_its_own_streams_and_is_never_supplied() {
        for stream in Stream::BOTH {
            let tab = a_tab(&a_record(stream, Some(CollectionOutcome::Collected), "x"));
            assert_eq!(
                tab.the_name_of_the_stream(),
                stream.the_name_of_the_stream()
            );
        }
        assert_ne!(
            Stream::TheOrdinaryOutput.the_name_of_the_stream(),
            Stream::TheComplaints.the_name_of_the_stream()
        );
    }

    /// **M10: the empty-stream claim is worded to the stream it is about and
    /// asserts nothing about the other.**
    #[test]
    fn the_empty_sentence_is_worded_to_its_own_stream() {
        let output = a_tab(&a_record(
            Stream::TheOrdinaryOutput,
            Some(CollectionOutcome::CollectedAndTheStreamWasEmpty),
            "",
        ));
        let errors = a_tab(&a_record(
            Stream::TheComplaints,
            Some(CollectionOutcome::CollectedAndTheStreamWasEmpty),
            "",
        ));
        assert_ne!(output.the_sentences()[0], errors.the_sentences()[0]);
        assert_eq!(errors.the_sentences()[0], "Blender wrote nothing here.");
    }

    /// **The Creator's eighth notes, note 6 (2026-10-11): an empty tab claims
    /// only what is true of its own stream.** Blender 5 writes its errors to
    /// its ordinary output, so an empty *Errors* tab says nothing about whether
    /// the render reported errors, and neither tab says there is nothing
    /// further to read while the other may hold words.
    #[test]
    fn an_empty_tab_claims_nothing_about_the_other_stream() {
        let errors = a_tab(&a_record(
            Stream::TheComplaints,
            Some(CollectionOutcome::CollectedAndTheStreamWasEmpty),
            "",
        ));
        let output = a_tab(&a_record(
            Stream::TheOrdinaryOutput,
            Some(CollectionOutcome::CollectedAndTheStreamWasEmpty),
            "",
        ));
        assert_eq!(errors.the_sentences(), vec!["Blender wrote nothing here."]);
        assert_eq!(output.the_sentences(), vec!["The render printed no output."]);
        for tab in [&errors, &output] {
            for sentence in tab.the_sentences() {
                let sentence = sentence.to_lowercase();
                assert!(!sentence.contains("no errors"), "{sentence}");
                assert!(!sentence.contains("nothing further"), "{sentence}");
            }
        }
        assert!(!errors.the_sentences().iter().any(|s| s.to_lowercase().contains("output")));
        assert!(!output.the_sentences().iter().any(|s| s.to_lowercase().contains("error")));
    }

    /// **X2 and X10: the sentences are separate strings and are never one
    /// paragraph.**
    ///
    /// A paragraph handed over whole leaves every interior sentence ending
    /// unbound, which is §22 item 28's narrowing. This file hands each sentence
    /// on its own, so the narrowing does not reach a panel drawn here.
    #[test]
    fn each_sentence_is_its_own_string() {
        let tab = a_tab(&a_record(Stream::TheComplaints, None, ""));
        let sentences = tab.the_sentences();
        assert_eq!(sentences.len(), 2);
        for sentence in sentences {
            assert_eq!(
                sentence.matches(". ").count(),
                0,
                "two sentences were handed over as one string"
            );
        }
    }

    /// **The control names whose words they are, and there is one of it.**
    #[test]
    fn one_control_opens_the_panel_and_the_label_names_whose_words_they_are() {
        let control = the_control_that_opens_it(BLENDER_OUTPUT, "words.job-7.attempt-1");
        assert_eq!(control.label, "Blender output");
        assert_eq!(control.act, "words.job-7.attempt-1");
    }

    /// **M12: no exit status reaches this folder**, so the word *succeeded* is
    /// not reachable from a status code. The folder-wide half of this is
    /// `src/presentation/mod.rs`'s own sweep.
    #[test]
    fn no_exit_status_is_taken_anywhere_in_this_file() {
        let working = the_working_half();
        for reaching in ["exit_status", "status_code", "exit_code", "i32"] {
            assert!(
                !working.contains(reaching),
                "src/presentation/the_machines_own_words.rs names {reaching}. \
                 **M12: no presentation says a job succeeded because its status \
                 was 0** — twenty endings report success and six of them end at 0 \
                 with an empty output folder, so the status does not come here at \
                 all"
            );
        }
    }
}
