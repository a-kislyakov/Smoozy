//! **§2 P1 step 8 and §3.5 K13 performed — both streams held and copied to
//! the supervisor's sinks, the bytes counted, the marker observed, and the
//! five observations taken as the bytes pass.**
//!
//! # This is the seam step 7 turns on
//!
//! `Queue & History` R1: *"there is one reader, it rides the render worker's
//! own copy loop, and it produces readings, never state… the reader is a tee
//! on that loop, observing bytes as they pass."* **So there is one loop and
//! one read of each stream**, and everything that wants to see the bytes
//! rides it: nothing opens a sink to read progress, nothing polls a folder,
//! and there is no second reader anywhere.
//!
//! **The split is: this module owns the loop and `Queue & History` owns the
//! grammar.** [`ObservesEachLineAsItPasses`] is the seam;
//! [`TheQueuesReaderAsATee`] is the one implementation that carries a line
//! into that crate's own parse and its readings back out. **Nothing here
//! knows a line's shape and nothing there opens a stream.**
//!
//! # What is reused rather than written again
//!
//! **The sinks and the write into them are `Rented-server runtime`'s** —
//! that document's M1 makes them the supervisor's before the worker starts
//! and its M3 gives the copy to the worker, and
//! [`smoozy_runtime::acting::capture::TheCopyIntoTheSinks`] is the write.
//! **The ceiling's arithmetic is that crate's**
//! [`smoozy_runtime::capture::CaptureCeiling`], **the count is Smoozy's own
//! and never a reading of a sink's size** (its M4), and **what the worker
//! does when the ceiling is reached or a sink refuses** is its
//! `what_the_worker_does` — read to end of stream, count what is discarded,
//! write none of it. Nothing of that is decided again here.
//!
//! # Five things the loop does that are this document's
//!
//! - **L7: both streams are held and both are captured, always.** In 4.5 the
//!   device line's route writes to the complaints stream, which also carries
//!   the failure text the user is shown, while from 5.0 it is on the
//!   ordinary output; and which stream a message lands on is not fixed even
//!   without the ask. **So neither stream is the one that matters and both
//!   are read**, on their own threads, so that one blocking cannot stall the
//!   other — a pipe left undrained can change or stop the child, which is
//!   the observed reason `Rented-server runtime` M5 drains rather than
//!   closing.
//! - **L27: what else 4.5's debugging switch puts on the complaints stream
//!   is not established, and nothing here edits a stream to hide it.** That
//!   stream is the one carrying the failure text the user is shown, so the
//!   raw record a user opens on a 4.5 failure may carry debug lines beside
//!   it. The streams are kept apart and unedited (`Rented-server runtime` M7,
//!   `Rented-server runtime` M8): the bytes reach the sinks as they came, and
//!   an observer is handed a line with nothing removed.
//! - **L28: the ask collides with the failure text only in 4.5 and only by
//!   sharing a stream, and the resolution is this capture's rather than the
//!   contract's.** Both streams are captured whole; **nothing is read from
//!   either stream to decide anything** (`Rented-server runtime` M20,
//!   `Rented-server runtime` M24), and which line is shown first is
//!   `Desktop app and delivery`'s.
//! - **Every byte read is observed, whether or not it was retained.** The
//!   ceiling and a refusing sink end *durable writes* for a stream; they do
//!   not end the reading, and they must not blind the reader. A full mount
//!   would otherwise stop the live progress figures as well as the record,
//!   which nothing anywhere asks for.
//! - **The observations are of bytes as they pass, never a mining of the
//!   sink afterwards** (K13, `Queue & History` E10). The loop keeps one
//!   partial line per stream and offers each complete line once.
//!
//! # The marker, and what the observation is worth
//!
//! `Rented-server runtime` M28's two observations — **whether the marker
//! passed the worker's own loop, and whether anything followed it** — are
//! taken here, on the ordinary stream alone: the complaints stream carries
//! no marker and that document adds none. **They are observations of
//! Smoozy's own act on bytes Smoozy is copying**, exactly as the count is,
//! and neither is a reading of a sink.
//!
//! **What the marker's absence is worth is K14's, not this module's**: a
//! launch made with the marker never observed is *failed* only where the
//! ordinary stream's collection outcome is *never established*, and where
//! that outcome is *collection incomplete* instead the limb does not fire —
//! a capture that stopped cannot support the inference.
//!
//! # The remap's per-pair lines, and why they are read here
//!
//! `Dependency Upload & Remap` M8 promises that the remap material *"reports
//! every pair in the marker, and fails nothing quietly"*, and the marker is
//! `Job files and outputs` D16's, **on the render's own captured stream**.
//! That stream is this loop's, so the comparison between the pairs the
//! material carried and the lines the marker actually carried belongs at this
//! loop and nowhere else. **The comparison itself is that crate's** —
//! `smoozy_dependencies_machine::material::what_the_marker_said_about_the_pairs`
//! — and [`TheRemapPairsAsTheyPass`] is the observer that gives it the lines.
//! **Nothing here parses a line and nothing here composes a pair.**
//!
//! **The material's own pairs are already the worker's**: §2 P1 step 2 reads
//! the remap material the manifest names (L30) and step 7 composes the launch
//! material from it, so the same attempt holds both halves of the comparison
//! by the time step 8 runs.
//!
//! **It judges nothing.** `Dependency Upload & Remap` M8's own words are that
//! whether a pair with no line is a failed render is this document's ending to
//! decide, and K14's failed limbs are a closed list that does not name it. So
//! the comparison is read and reported and **no ending is invented for it
//! here**.
//!
//! **Markers used in this file that belong to another document, each with
//! the document it belongs to.** `Rented-server runtime` M1, M3, M4, M5,
//! M28. `Queue & History` E10, E13, R1, R5, R6.
//!
//! **`Queue & History` P1a is deliberately not on that list and is written
//! out at every site instead**, because this plan defines a P1a of its own
//! and a bare one would resolve to a real and wrong clause.

use std::io::Read;
use std::sync::Mutex;

use smoozy_dependencies_machine::material::{
    what_the_marker_said_about_the_pairs, WhatTheMarkerSaidAboutThePairs, THE_PER_PAIR_HEAD,
};
use smoozy_dependencies_machine::remap::RemapMaterial;
use smoozy_queue_history_machine::acting::reader::{read_one_line, TheGrammar, WhatTheLineWas};
use smoozy_queue_history_machine::readings::{
    fold_a_line, AWorkersReport, LiveReadings, ObservedDeviceLine, WhichStream,
};
use smoozy_runtime::capture::{
    what_the_worker_does, CaptureCeiling, CopiesAStreamIntoItsSink, Stream,
    WhatTheWorkerDoesWithTheRestOfTheStream, WhatTheWorkerObserved,
};
use smoozy_state::observed::{Clock, Reading};

use crate::acting::material::THE_MARKER_HEAD;

/// **The seam: something that sees each line of the render's output as it
/// passes the copy loop.**
///
/// **One method and one shape**, because there is one loop: an observer is
/// handed the stream the line came from and the line itself, with no
/// trailing newline and with nothing removed. **It is handed the bytes and
/// never a file, a path or a sink**, so an observer that wanted a second
/// read of anything could not be written against this.
///
/// **An observer may not fail the render** (`Queue & History` R8): there is
/// no result to return, so there is no route from an observer back into the
/// loop's own conduct.
pub trait ObservesEachLineAsItPasses {
    fn a_line_passed(&mut self, stream: Stream, line: &[u8]);
}

/// **Two observers at once**, so the marker's own observation and the
/// queue's reader ride the same loop without either knowing about the other.
impl<A: ObservesEachLineAsItPasses, B: ObservesEachLineAsItPasses> ObservesEachLineAsItPasses
    for (A, B)
{
    fn a_line_passed(&mut self, stream: Stream, line: &[u8]) {
        self.0.a_line_passed(stream, line);
        self.1.a_line_passed(stream, line);
    }
}

/// **`Queue & History`'s reader, riding this loop as a tee** — R1's own
/// shape, and the whole of the seam between this crate and that one.
///
/// **It parses nothing itself**: every line goes to that crate's
/// `read_one_line` under the grammar the manifest's version chose, and what
/// comes back is folded by that crate's own `fold_a_line`.
///
/// **`Queue & History` R3: the reader consumes the ordinary stream only.** The complaints
/// stream is captured, retained and shown and is not parsed for progress. So
/// a complaints line reaches this observer and is dropped — **except the
/// device line**, which in 4.5 lands on the complaints stream by the only
/// route there is (L26, L7), and which is not a progress reading and is not
/// in `Queue & History` R6's closed list.
#[derive(Debug, Clone, PartialEq)]
pub struct TheQueuesReaderAsATee {
    grammar: TheGrammar,
    attempt: u32,
    readings: LiveReadings,
    the_attempts_frame_list: Vec<i64>,
    newest_frame_saved: Option<i64>,
    the_device_line: Option<ObservedDeviceLine>,
    frames_begun: u64,
    the_plan_total: Option<u64>,
    /// **`Queue & History` B1's renderlets**, which `Queue & History` B6's
    /// finished part is summed from at each report. Added 2026-10-03, piece
    /// 6b.
    the_plan: Option<Vec<smoozy_queue_history_machine::plan::Renderlet>>,
    /// **The view layers of the frame in hand whose renderlet has ended**,
    /// because the line named the next one. Piece 3. `Queue & History` B6's
    /// share of the frame in hand is read from these and from the readings
    /// themselves, which since piece 3b start over at each named layer
    /// (`fold_a_line`), so they are the unit in hand's own.
    the_layers_ended: Vec<String>,
    /// **The last finished figure sent**, which the next is never less than
    /// (`Queue & History` B6 as corrected 2026-10-04). A cell because the
    /// report is taken through a shared borrow under the observers' lock.
    the_last_figure_sent: std::cell::Cell<u64>,
    /// **The scene the job renders, by its name, where the job's reading
    /// knows it** — so the line naming a view layer is split after the known
    /// scene's name rather than at its first `, ` (`Queue & History` R10's
    /// limit, closed 2026-10-04, the Creator's second notes, piece 3c).
    the_scene: Option<String>,
}

impl TheQueuesReaderAsATee {
    /// The reader for one attempt, under the version's own grammar.
    ///
    /// **The frame list is the attempt record's** (`Job files and outputs`
    /// M8), which `Queue & History` R9 says the reader has: *"a reader that knows the
    /// attempt's frame list (it does: the attempt record holds it) knows
    /// which real frame that is."*
    pub fn for_the_attempt(
        major_version: u32,
        attempt: u32,
        the_attempts_frame_list: &[i64],
    ) -> Self {
        TheQueuesReaderAsATee {
            grammar: TheGrammar::for_a_version(major_version),
            attempt,
            readings: LiveReadings::default(),
            the_attempts_frame_list: the_attempts_frame_list.to_vec(),
            newest_frame_saved: None,
            the_device_line: None,
            frames_begun: 0,
            the_plan_total: None,
            the_plan: None,
            the_layers_ended: Vec::new(),
            the_last_figure_sent: std::cell::Cell::new(0),
            the_scene: None,
        }
    }

    /// **The scene's name, as the job's reading knows it**, so the reader
    /// takes it off the front of the line naming a view layer. Nothing, or an
    /// empty name, leaves the split at the first `, `. Added 2026-10-04, the
    /// Creator's second notes, piece 3c.
    pub fn knowing_the_scene(mut self, the_scene: Option<String>) -> Self {
        self.the_scene = the_scene.filter(|name| !name.is_empty());
        self
    }

    /// **`Queue & History` B1's plan, carried in every report as its total and
    /// as `Queue & History` B6's finished part** — worked out once by the
    /// worker from the reading it took at §2 P1 step 4, before the launch, and
    /// never by the reader. Added 2026-10-03, the Creator's notes on 0.1.8,
    /// piece 6a, as the total; the plan itself since piece 6b.
    pub fn carrying_the_plan(
        mut self,
        plan: Option<Vec<smoozy_queue_history_machine::plan::Renderlet>>,
    ) -> Self {
        self.the_plan_total = plan
            .as_deref()
            .map(smoozy_queue_history_machine::plan::the_plan_total);
        self.the_plan = plan;
        self
    }

    /// **`Queue & History` R5's report, as the worker sends it to the supervisor at its own
    /// bounded interval.**
    ///
    /// The interval is the worker's own (K13), and `Queue & History` R5 rules
    /// it at two seconds: [`THE_WORKERS_REPORTING_INTERVAL`]. What this offers
    /// is the current readings whenever the worker's interval comes round.
    pub fn the_report(&self, at: std::time::SystemTime, on: Clock) -> AWorkersReport {
        AWorkersReport {
            attempt: self.attempt,
            // **`State, identity and recovery` D5: every reading carries the
            // moment it was taken and the clock it was taken on**, and the
            // clock here is the machine's own — the worker takes it, not the
            // desk.
            readings: Reading::taken(self.readings.clone(), at, on),
            newest_frame_saved: self.newest_frame_saved,
            the_plan_total: self.the_plan_total,
            // **`Queue & History` B6's finished part, from the same
            // renderlets** — the frames this attempt does not ask for, its
            // own frames up to the newest saved (piece 6b), and since
            // 2026-10-04 the frame in hand's share, never less than the last
            // figure sent (the Creator's second notes, piece 3).
            the_units_finished: self.the_plan.as_deref().map(|plan| {
                let figure = smoozy_queue_history_machine::plan::the_units_finished(
                    plan,
                    &self.the_attempts_frame_list,
                    self.newest_frame_saved,
                    self.readings.frame_in_hand.map(|frame| {
                        smoozy_queue_history_machine::plan::TheFrameInHand {
                            frame,
                            the_layers_ended: &self.the_layers_ended,
                            the_layer_in_hand: self.readings.view_layer.as_deref(),
                            the_unit_in_hand: &self.readings,
                        }
                    }),
                    self.the_last_figure_sent.get(),
                );
                self.the_last_figure_sent.set(figure);
                figure
            }),
        }
    }

    /// **The readings take one more line** — `Queue & History` R11's own fold,
    /// which starts over at a new frame and at a new named view layer
    /// (`Queue & History` B3b's next unit beginning). Before it, the ended
    /// layers move with it: a new frame clears them, and a line naming a
    /// different view layer of the same frame ends the one before. **Only a
    /// named layer is a boundary**: nothing here reads one off a falling count
    /// (`Queue & History` B3a). Piece 3; one fold since piece 3b.
    fn the_readings_take(&mut self, line: &smoozy_queue_history_machine::readings::AProgressLine) {
        let held = &self.readings;
        if line.frame.is_some() && line.frame != held.frame_in_hand {
            self.the_layers_ended.clear();
        } else if let (Some(named), Some(in_hand)) = (&line.view_layer, &held.view_layer) {
            if named != in_hand && !self.the_layers_ended.contains(in_hand) {
                self.the_layers_ended.push(in_hand.clone());
            }
        }
        self.readings = fold_a_line(&self.readings, line);
    }

    /// **`Queue & History` E13's record, where the line was observed.** `None` where it was
    /// not, which is the entry's *the job's devices are not known*.
    pub fn the_device_line(&self) -> Option<&ObservedDeviceLine> {
        self.the_device_line.as_ref()
    }

    /// The readings as they stand, for a caller that wants them without
    /// composing a report.
    pub fn readings(&self) -> &LiveReadings {
        &self.readings
    }
}

impl ObservesEachLineAsItPasses for TheQueuesReaderAsATee {
    fn a_line_passed(&mut self, stream: Stream, line: &[u8]) {
        // **A line that is not text yields nothing** — `Queue & History` R3's rule met one step
        // earlier, and never a lossy reading that would put a wrong figure on
        // the entry.
        let Ok(text) = std::str::from_utf8(line) else {
            return;
        };
        match read_one_line(self.grammar, self.the_scene.as_deref(), text) {
            // **L26 and `Queue & History` E13: observed on the version's own stream** — the
            // complaints stream in 4.5, the ordinary output from 5.0 — so
            // this arm is the one thing the reader takes off either stream.
            // **The first observed line stands**; a second is not a second
            // job's.
            WhatTheLineWas::TheDeviceLine { as_printed } => {
                self.the_device_line.get_or_insert(ObservedDeviceLine {
                    attempt: self.attempt,
                    the_line_as_printed: as_printed,
                    on_stream: match stream {
                        Stream::Ordinary => WhichStream::TheOrdinaryOutput,
                        Stream::Complaints => WhichStream::TheComplaints,
                    },
                });
            }
            other if stream == Stream::Complaints => {
                // **`Queue & History` R3: the complaints stream is not parsed for progress.**
                let _ = other;
            }
            WhatTheLineWas::Simulation { frames } => {
                // `Queue & History` R6a: report simulation without advancing
                // any rendered-frame count or sample figure.
                self.readings.simulation = frames;
            }
            WhatTheLineWas::AProgressLine(read) => {
                self.the_readings_take(&read);
                // **`Queue & History` R6's last reading: the count of frames announced or
                // completed.** In 4.5 nothing announces a frame, so the
                // progress line's own frame is the only marker there is
                // (`Queue & History` R9)
                // and the count follows it.
                if self.grammar == TheGrammar::PlainLinesNamingTheViewLayer {
                    self.readings.frames_completed = self.frames_seen();
                }
            }
            WhatTheLineWas::AFrameHasBegun { frame } => {
                // **`Queue & History` R9, from 5.0: the current frame is the last
                // announcement.** The announcement is not a count of frames —
                // it prints again on a step of zero and once before a frame
                // that is then skipped — so what is counted is the frames of
                // the attempt's own list that have been named.
                let announced = smoozy_queue_history_machine::readings::AProgressLine {
                    frame: Some(frame),
                    view_layer: None,
                    samples: None,
                    tiles: None,
                    estimate: None,
                    the_line_says_finished: false,
                };
                self.the_readings_take(&announced);
                self.frames_begun = self.frames_begun.saturating_add(1);
                self.readings.frames_completed = self.frames_seen();
            }
            WhatTheLineWas::AViewLayerHasBegun { view_layer } => {
                // **`Queue & History` R10 as corrected 2026-10-04, from 5.0:
                // the view layer is announced at each layer's start**, so the
                // renderlet in hand is named and the readings start over for
                // it, as 4.5's lines always made them. Piece 3b.
                self.the_readings_take(&smoozy_queue_history_machine::readings::AProgressLine {
                    frame: None,
                    view_layer: Some(view_layer),
                    samples: None,
                    tiles: None,
                    estimate: None,
                    the_line_says_finished: false,
                });
            }
            WhatTheLineWas::AFrameWasSaved { .. } => {
                // **`Queue & History` P1a's figure: the newest frame of the attempt's list
                // whose save has been announced.** Frames are worked through
                // in the order given, so the figure marks the list's
                // completed prefix; the line names a path and the frame in
                // hand is what says which frame it was.
                if let Some(frame) = self.readings.frame_in_hand {
                    if self.the_attempts_frame_list.contains(&frame) {
                        self.newest_frame_saved = Some(frame);
                    }
                }
            }
            WhatTheLineWas::TheFrameRange { .. } | WhatTheLineWas::NothingReadable => {}
        }
    }
}

impl TheQueuesReaderAsATee {
    /// How many of the attempt's own frames have been reached, by the frame
    /// in hand's place in the list. **Never a count of announcements**,
    /// which the research says outright is not a count of frames.
    fn frames_seen(&self) -> u64 {
        match self.readings.frame_in_hand {
            Some(frame) => self
                .the_attempts_frame_list
                .iter()
                .position(|asked| *asked == frame)
                .map(|at| at as u64)
                .unwrap_or(0),
            None => 0,
        }
    }
}

/// **`Rented-server runtime` M28's two observations, taken on the ordinary
/// stream alone.**
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TheMarkersObservation {
    passed: bool,
    something_followed_it: bool,
}

impl TheMarkersObservation {
    /// Whether the marker passed the worker's own loop, on the ordinary
    /// stream. `None` on the complaints stream, which has no marker.
    pub fn passed(&self, stream: Stream) -> Option<bool> {
        (stream == Stream::Ordinary).then_some(self.passed)
    }

    /// Whether anything followed it, on the ordinary stream.
    pub fn something_followed_it(&self, stream: Stream) -> Option<bool> {
        (stream == Stream::Ordinary).then_some(self.something_followed_it)
    }
}

impl ObservesEachLineAsItPasses for TheMarkersObservation {
    fn a_line_passed(&mut self, stream: Stream, line: &[u8]) {
        if stream != Stream::Ordinary {
            return;
        }
        if self.passed {
            self.something_followed_it = true;
            return;
        }
        // **Nothing here compares against the marker's length.** The marker
        // names the job identity and each destination it set, so its length
        // varies by job; what is observed is its own head passing.
        if window(line, THE_MARKER_HEAD.as_bytes()) {
            self.passed = true;
        }
    }
}

fn window(haystack: &[u8], needle: &[u8]) -> bool {
    needle.len() <= haystack.len() && haystack.windows(needle.len()).any(|at| at == needle)
}

/// **The remap's per-pair lines, kept as they pass this loop.**
///
/// `Dependency Upload & Remap` M8 writes one line per pair onto
/// `Job files and outputs` D16's marker, on the render's own ordinary output.
/// **This observer keeps those lines and nothing else** — it reads none of
/// them, and it is the same shape of act as [`TheMarkersObservation`]: an
/// observation of bytes as they pass, never a mining of a sink afterwards
/// (K13, `Queue & History` E10).
///
/// **The ordinary stream alone.** The material writes the marker there and
/// nowhere else, so a line carrying the head on the complaints stream is not
/// the material's and is not taken for it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TheRemapPairsAsTheyPass {
    lines: Vec<Vec<u8>>,
}

impl TheRemapPairsAsTheyPass {
    /// The kept lines as one run, in the order they passed — what the reader
    /// on the far side of the comparison takes.
    ///
    /// **A run rather than a count**, because the reading is that crate's and
    /// this one holds no shape of a line.
    fn as_the_stream_carried_them(&self) -> Vec<u8> {
        let mut out = Vec::new();
        for line in &self.lines {
            out.extend_from_slice(line);
            out.push(b'\n');
        }
        out
    }

    /// How many lines carrying the head passed. **Not a count of pairs**: a
    /// line naming no pair of the material and a line the reader cannot follow
    /// both land here, and telling those apart is the comparison's.
    pub fn how_many_lines_passed(&self) -> usize {
        self.lines.len()
    }
}

impl ObservesEachLineAsItPasses for TheRemapPairsAsTheyPass {
    fn a_line_passed(&mut self, stream: Stream, line: &[u8]) {
        if stream != Stream::Ordinary {
            return;
        }
        if window(line, THE_PER_PAIR_HEAD.as_bytes()) {
            self.lines.push(line.to_vec());
        }
    }
}

/// **`Dependency Upload & Remap` M8 held end to end: the pairs the material
/// carried against the lines the marker carried.**
///
/// That clause's promise is *"reports every pair in the marker, and fails
/// nothing quietly"*, and the comparison that checks it is that crate's own
/// `what_the_marker_said_about_the_pairs`. **Its caller is whoever holds the
/// render's own captured stream, and that is this loop** — the marker rides
/// `Job files and outputs` D16 rather than a channel of that module's, so the
/// stream is this document's.
///
/// **The material is the one §2 P1 step 2 read** (L30), carried through step 7
/// into the launch material, so the pairs compared are the pairs applied and
/// never a fresh composition.
///
/// **It judges nothing and ends no attempt.** What a pair with no line is
/// worth is this document's ending to decide, and K14's failed limbs are a
/// closed list that does not name it — so nothing here fails an attempt, and
/// no limb is added to that list.
pub fn what_the_marker_said_about_the_remap_pairs(
    observed: &TheRemapPairsAsTheyPass,
    remap: &RemapMaterial,
) -> WhatTheMarkerSaidAboutThePairs {
    what_the_marker_said_about_the_pairs(&remap.pairs, &observed.as_the_stream_carried_them())
}

/// **The loop, held by the worker for one attempt.**
///
/// The ceiling and the observers are shared by both streams — the two draw
/// from one counter (`Rented-server runtime` M4a) and one reader sees one
/// render — so each is behind a lock and each lock is held for one write or
/// one line at a time.
pub struct TheWorkersCopyLoop<'a, C, O> {
    /// `Rented-server runtime` M3's write, taken whole.
    pub copy: &'a C,
    /// `Rented-server runtime` M4a's ceiling, recorded by value before the
    /// worker started.
    pub ceiling: Mutex<CaptureCeiling>,
    /// Everything that rides the loop.
    pub observers: Mutex<O>,
}

/// How much is read at once. **A buffer size and nothing more**: no
/// judgement turns on it, the count is of what the write retained, and a
/// line is assembled across reads.
const HOW_MUCH_IS_READ_AT_ONCE: usize = 8 * 1024;

/// **The worker's reporting interval: two seconds.** K13 leaves the interval
/// to the worker and `Queue & History` R5 rules it — *"The worker's interval
/// is two seconds"* — on the build runner's ruling of 2026-10-02 (the map of
/// the Creator's notes on 0.1.8, hole 9).
pub const THE_WORKERS_REPORTING_INTERVAL: std::time::Duration = std::time::Duration::from_secs(2);

impl<'a, C, O> TheWorkersCopyLoop<'a, C, O>
where
    C: CopiesAStreamIntoItsSink + Sync,
    O: ObservesEachLineAsItPasses + Send,
{
    pub fn holding(copy: &'a C, ceiling: CaptureCeiling, observers: O) -> Self {
        TheWorkersCopyLoop {
            copy,
            ceiling: Mutex::new(ceiling),
            observers: Mutex::new(observers),
        }
    }

    /// **Both streams, held and copied to the sinks at the worker's own
    /// loop** (L7, `Rented-server runtime` M3).
    ///
    /// **Each stream is read on its own thread**, because a pipe left
    /// undrained can change or stop the child: reading one to its end before
    /// touching the other would stall a render whose other stream filled its
    /// pipe.
    pub fn run(
        &self,
        ordinary: impl Read + Send,
        complaints: impl Read + Send,
    ) -> (WhatTheWorkerObserved, WhatTheWorkerObserved) {
        std::thread::scope(|threads| {
            let one = threads.spawn(|| self.one_stream(Stream::Ordinary, ordinary));
            let other = threads.spawn(|| self.one_stream(Stream::Complaints, complaints));
            (
                one.join().expect("the ordinary stream's copy"),
                other.join().expect("the complaints stream's copy"),
            )
        })
    }

    /// **Both streams, as [`TheWorkersCopyLoop::run`], with the observers'
    /// readings taken and sent at every `interval` while the streams run** —
    /// `Queue & History` R5's report at the worker's own bounded interval.
    /// Added 2026-10-03, the Creator's notes on 0.1.8, piece 6a.
    ///
    /// **`take` runs under the observers' lock and `send` outside it**, so a
    /// slow or absent supervisor holds up the next report and never the copy:
    /// `Queue & History` R8, the reader never slows the render by more than its
    /// own loop. **Nothing sent can fail the render**: `send` returns nothing,
    /// a report that does not land is simply followed by the next one, and
    /// there is no route back into the loop.
    ///
    /// **The reporting ends with the streams.** A send under way when they end
    /// is let finish, so it is bounded by the sender's own exchange bound and
    /// nothing else.
    pub fn run_reporting_every<T>(
        &self,
        interval: std::time::Duration,
        ordinary: impl Read + Send,
        complaints: impl Read + Send,
        take: impl Fn(&O) -> T + Sync,
        send: impl Fn(T) + Sync,
    ) -> (WhatTheWorkerObserved, WhatTheWorkerObserved) {
        let (the_streams_run, the_streams_ended) = std::sync::mpsc::channel::<()>();
        let (take, send) = (&take, &send);
        std::thread::scope(|threads| {
            let reporter = threads.spawn(move || loop {
                match the_streams_ended.recv_timeout(interval) {
                    Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                        // **A poisoned lock means a reader panicked**, and a
                        // reader may not fail the render; its report is not
                        // sent rather than made up.
                        let taken = match self.observers.lock() {
                            Ok(observers) => take(&observers),
                            Err(_) => continue,
                        };
                        send(taken);
                    }
                    _ => return,
                }
            });
            let observed = self.run(ordinary, complaints);
            drop(the_streams_run);
            reporter.join().ok();
            observed
        })
    }

    /// One stream, read to its end whatever the writes do.
    pub fn one_stream(&self, stream: Stream, mut from: impl Read) -> WhatTheWorkerObserved {
        let mut retained = 0u64;
        let mut discarded = 0u64;
        let mut first_loss_reason: Option<String> = None;
        let mut partial: Vec<u8> = Vec::new();
        let mut buffer = vec![0u8; HOW_MUCH_IS_READ_AT_ONCE];
        let mut end_of_stream = false;

        loop {
            let how_many = match from.read(&mut buffer) {
                Ok(0) => {
                    end_of_stream = true;
                    break;
                }
                Ok(how_many) => how_many,
                Err(why) if why.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(_) => break,
            };
            let bytes = &buffer[..how_many];

            // **Observed whether or not it is retained.** The ceiling and a
            // refusing sink end durable writes for this stream; they do not
            // end the reading, and a reader blinded by a full disk is a cost
            // nothing asks for.
            self.offer_the_lines_in(stream, bytes, &mut partial);

            let ceiling_now = *self.ceiling.lock().expect("the ceiling is not poisoned");
            match what_the_worker_does(ceiling_now, first_loss_reason.is_some()) {
                WhatTheWorkerDoesWithTheRestOfTheStream::DrainsAndDiscards => {
                    discarded += how_many as u64;
                    if first_loss_reason.is_none() {
                        first_loss_reason =
                            Some("the shared capture ceiling was reached".to_string());
                    }
                }
                WhatTheWorkerDoesWithTheRestOfTheStream::RetainsIt => {
                    let may_write = ceiling_now.may_offer(how_many as u64);
                    match self.copy.write(stream, bytes, may_write) {
                        Ok(took) => {
                            retained += took;
                            self.ceiling
                                .lock()
                                .expect("the ceiling is not poisoned")
                                .retained(took);
                            let left_over = how_many as u64 - took;
                            if left_over > 0 {
                                discarded += left_over;
                                first_loss_reason.get_or_insert_with(|| {
                                    "the shared capture ceiling was reached".to_string()
                                });
                            }
                        }
                        Err(why) => {
                            // **M5's first reason, recorded once**, and the
                            // machine's own words carried rather than
                            // summarised.
                            discarded += how_many as u64;
                            first_loss_reason.get_or_insert(why);
                        }
                    }
                }
            }
        }

        // A last line with no newline after it still passed the loop.
        if !partial.is_empty() {
            let line = std::mem::take(&mut partial);
            self.observers
                .lock()
                .expect("the observers are not poisoned")
                .a_line_passed(stream, &line);
        }

        let marker = self
            .observers
            .lock()
            .expect("the observers are not poisoned");
        let _ = &*marker;
        drop(marker);

        WhatTheWorkerObserved {
            stream,
            retained,
            discarded,
            first_loss_reason,
            end_of_stream,
            // **M28's two are the marker observer's**, and the worker reads
            // them off it after the loop rather than the loop guessing.
            the_marker_passed: None,
            something_followed_the_marker: None,
        }
    }

    fn offer_the_lines_in(&self, stream: Stream, bytes: &[u8], partial: &mut Vec<u8>) {
        let mut observers = self
            .observers
            .lock()
            .expect("the observers are not poisoned");
        for byte in bytes {
            if *byte == b'\n' {
                let line = std::mem::take(partial);
                // A line ending in a carriage return is one line, not a line
                // with a stray byte on the end.
                let line = match line.last() {
                    Some(b'\r') => line[..line.len() - 1].to_vec(),
                    _ => line,
                };
                observers.a_line_passed(stream, &line);
            } else {
                partial.push(*byte);
            }
        }
    }
}

/// **M28's two observations written onto the worker's own record**, taken
/// from the marker observer after the loop has ended.
///
/// **They are the ordinary stream's alone**: the complaints stream carries no
/// marker and `Rented-server runtime` M16 adds none.
pub fn with_the_markers_observation(
    mut observed: WhatTheWorkerObserved,
    marker: &TheMarkersObservation,
) -> WhatTheWorkerObserved {
    observed.the_marker_passed = marker.passed(observed.stream);
    observed.something_followed_the_marker = marker.something_followed_it(observed.stream);
    observed
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_runtime::capture::CaptureRecord;
    use smoozy_state::identities::{Identity, IdentityKind};

    /// A ceiling with room in it. **`Rented-server runtime` M4a's ceiling is
    /// the available reading less that document's own control-state
    /// reserve**, so a reading below the reserve is a spent ceiling — which
    /// is a real state and not the one these tests are about.
    fn a_ceiling_with_room() -> CaptureCeiling {
        CaptureCeiling::from(smoozy_runtime::capture::THE_CONTROL_STATE_RESERVE + 1024 * 1024)
    }

    fn a_capture_record(root: &std::path::Path) -> CaptureRecord {
        CaptureRecord {
            job: Identity::issue(IdentityKind::Job).expect("randomness"),
            attempt: 1,
            ordinary_sink: root.join("ordinary").display().to_string(),
            complaints_sink: root.join("complaints").display().to_string(),
            ceiling: a_ceiling_with_room(),
        }
    }

    fn a_root(named: &str) -> std::path::PathBuf {
        let root =
            std::env::temp_dir().join(format!("smoozy-copy-loop-{}-{named}", std::process::id()));
        std::fs::remove_dir_all(&root).ok();
        std::fs::create_dir_all(&root).expect("the place is made");
        root
    }

    /// **The loop is run for real against the two sinks `Rented-server
    /// runtime` makes**, with both streams held and both copied.
    #[test]
    fn both_streams_are_copied_to_their_own_sinks_and_the_counts_are_smoozys_own() {
        use smoozy_runtime::acting::capture::{TheCopyIntoTheSinks, TheSinksOnTheMachine};
        use smoozy_runtime::capture::MakesTheSinks;
        let root = a_root("both");
        let record = a_capture_record(&root);
        TheSinksOnTheMachine.make(&record).expect("the sinks");
        let copy = TheCopyIntoTheSinks::opening(&record).expect("opened");
        let loop_ = TheWorkersCopyLoop::holding(
            &copy,
            record.ceiling,
            (TheMarkersObservation::default(), ACountOfLines::default()),
        );
        let (ordinary, complaints) = loop_.run(
            &b"Fra:1 | Sample 1/16\nSMOOZY-MATERIAL {\"opened\":\"j\"}\nafter\n"[..],
            &b"a complaint\n"[..],
        );
        assert!(ordinary.end_of_stream && complaints.end_of_stream);
        assert_eq!(ordinary.retained, 57);
        assert_eq!(complaints.retained, 12);
        assert_eq!(ordinary.discarded, 0);
        assert_eq!(
            std::fs::read_to_string(&record.complaints_sink).unwrap(),
            "a complaint\n",
            "nothing merges them"
        );
        std::fs::remove_dir_all(&root).ok();
    }

    /// **M28's two, observed as the bytes pass** — and the marker's absence
    /// told apart from its presence with nothing after it.
    #[test]
    fn the_marker_and_what_followed_it_are_observed_on_the_ordinary_stream_alone() {
        for (stream_text, passed, followed) in [
            ("nothing of the kind\n", false, false),
            ("SMOOZY-MATERIAL {\"opened\":\"j\"}\n", true, false),
            (
                "SMOOZY-MATERIAL {\"opened\":\"j\"}\nFra:1 | Sample 1/16\n",
                true,
                true,
            ),
        ] {
            let root = a_root("marker");
            let record = a_capture_record(&root);
            use smoozy_runtime::acting::capture::{TheCopyIntoTheSinks, TheSinksOnTheMachine};
            use smoozy_runtime::capture::MakesTheSinks;
            TheSinksOnTheMachine.make(&record).expect("the sinks");
            let copy = TheCopyIntoTheSinks::opening(&record).expect("opened");
            let loop_ = TheWorkersCopyLoop::holding(
                &copy,
                record.ceiling,
                TheMarkersObservation::default(),
            );
            let ordinary = loop_.one_stream(Stream::Ordinary, stream_text.as_bytes());
            let marker = *loop_.observers.lock().unwrap();
            let ordinary = with_the_markers_observation(ordinary, &marker);
            assert_eq!(ordinary.the_marker_passed, Some(passed), "{stream_text}");
            assert_eq!(
                ordinary.something_followed_the_marker,
                Some(followed),
                "{stream_text}"
            );
            // The complaints stream has no marker and takes neither
            // observation.
            let on_the_complaints = with_the_markers_observation(
                WhatTheWorkerObserved {
                    stream: Stream::Complaints,
                    retained: 0,
                    discarded: 0,
                    first_loss_reason: None,
                    end_of_stream: true,
                    the_marker_passed: None,
                    something_followed_the_marker: None,
                },
                &marker,
            );
            assert_eq!(on_the_complaints.the_marker_passed, None);
            std::fs::remove_dir_all(&root).ok();
        }
    }

    /// **The queue's reader rides the loop and reads the render's own
    /// lines** — the seam, run end to end from bytes to readings.
    #[test]
    fn the_queues_reader_tees_the_loop_and_produces_readings_from_the_bytes() {
        let root = a_root("tee");
        let record = a_capture_record(&root);
        use smoozy_runtime::acting::capture::{TheCopyIntoTheSinks, TheSinksOnTheMachine};
        use smoozy_runtime::capture::MakesTheSinks;
        TheSinksOnTheMachine.make(&record).expect("the sinks");
        let copy = TheCopyIntoTheSinks::opening(&record).expect("opened");
        let loop_ = TheWorkersCopyLoop::holding(
            &copy,
            record.ceiling,
            TheQueuesReaderAsATee::for_the_attempt(5, 1, &[1, 2, 3]),
        );
        let stream = "00:00.405  render           | Rendering animation (frames 1..3)\n\
             00:00.405  render           | Rendering frame 1\n\
             00:00.430  render           | Fra: 1 | Remaining: 00:00.05 | Mem: 2M | Sample 1/16\n\
             00:00.500  render           | Fra: 1 | Mem: 2M | Sample 16/16\n\
             00:00.935  render           | Saved: '/out/frame_0001.png'\n\
             00:00.940  render           | Rendering frame 2\n";
        let observed = loop_.one_stream(Stream::Ordinary, stream.as_bytes());
        assert!(observed.end_of_stream);
        let reader = loop_.observers.lock().unwrap();
        assert_eq!(reader.readings().frame_in_hand, Some(2));
        assert_eq!(
            reader.newest_frame_saved,
            Some(1),
            "P1a's figure, off the save announcement"
        );
        assert_eq!(reader.readings().frames_completed, 1);
        std::fs::remove_dir_all(&root).ok();
    }

    /// **Piece 3c: the reader takes the known scene's name off the front of the
    /// line naming a view layer**, so a scene named `Shot, 01` names the layer
    /// `Back, plate` (the line 5.2.2 printed at the desk). **What it bites
    /// on**: a reader that drops the scene's name, which names the layer
    /// `01, Back, plate`.
    #[test]
    fn the_reader_knowing_the_scene_names_the_layer_after_it() {
        let line = b"00:00.409  render           | Start rendering: Shot, 01, Back, plate";
        let mut reader = TheQueuesReaderAsATee::for_the_attempt(5, 1, &[1])
            .knowing_the_scene(Some("Shot, 01".to_string()));
        reader.a_line_passed(Stream::Ordinary, line);
        assert_eq!(reader.readings().view_layer.as_deref(), Some("Back, plate"));
        let mut unknown = TheQueuesReaderAsATee::for_the_attempt(5, 1, &[1]).knowing_the_scene(Some(String::new()));
        unknown.a_line_passed(Stream::Ordinary, line);
        assert_eq!(unknown.readings().view_layer.as_deref(), Some("01, Back, plate"), "an empty name is not known");
    }

    /// **The device line is observed on either stream** — 4.5's route puts
    /// it on the complaints stream and there is no other, which is why L7
    /// captures both, always.
    #[test]
    fn the_device_line_is_taken_off_the_complaints_stream_in_four_five() {
        let mut reader = TheQueuesReaderAsATee::for_the_attempt(4, 1, &[1]);
        reader.a_line_passed(
            Stream::Complaints,
            b"Path tracing on: a processor, its thread count [CPU]",
        );
        let observed = reader.the_device_line().expect("observed");
        assert_eq!(observed.on_stream, WhichStream::TheComplaints);
        assert_eq!(observed.attempt, 1);
    }

    /// **`Queue & History` R3: the complaints stream is not parsed for
    /// progress.** A progress
    /// line that arrived there moves no reading.
    #[test]
    fn a_progress_line_on_the_complaints_stream_moves_no_reading() {
        let mut reader = TheQueuesReaderAsATee::for_the_attempt(4, 1, &[1]);
        reader.a_line_passed(
            Stream::Complaints,
            b"Fra:1 Mem:9.77M (Peak 9.77M) | Time:00:00.02 | Mem:1.48M, Peak:1.48M | \
              Scene, ViewLayer | Sample 1/16",
        );
        assert_eq!(reader.readings(), &LiveReadings::default());
    }

    /// **A full sink does not blind the reader**, which is the one thing the
    /// order of the loop's two acts decides. The fault is introduced by a
    /// write that refuses everything, and the readings still move.
    #[test]
    fn a_refusing_sink_ends_the_durable_writes_and_never_the_observation() {
        struct RefusesEverything;
        impl CopiesAStreamIntoItsSink for RefusesEverything {
            fn write(&self, _: Stream, _: &[u8], _: u64) -> Result<u64, String> {
                Err("No space left on device".to_string())
            }
        }
        let refuses = RefusesEverything;
        let loop_ = TheWorkersCopyLoop::holding(
            &refuses,
            a_ceiling_with_room(),
            TheQueuesReaderAsATee::for_the_attempt(4, 1, &[7]),
        );
        let observed = loop_.one_stream(
            Stream::Ordinary,
            &b"Fra:7 Mem:9.77M (Peak 9.77M) | Time:00:00.02 | Mem:1.48M, Peak:1.48M | \
               Scene, ViewLayer | Sample 3/16\n"[..],
        );
        assert_eq!(observed.retained, 0);
        assert!(observed.discarded > 0);
        assert_eq!(
            observed.first_loss_reason.as_deref(),
            Some("No space left on device"),
            "the machine's own words, recorded once"
        );
        let reader = loop_.observers.lock().unwrap();
        assert_eq!(reader.readings().frame_in_hand, Some(7));
    }

    /// A line split across two reads is one line, and a last line with no
    /// newline still passes the loop.
    #[test]
    fn a_line_split_across_reads_is_one_line_and_a_last_line_needs_no_newline() {
        struct InTwoPieces(Vec<Vec<u8>>);
        impl Read for InTwoPieces {
            fn read(&mut self, into: &mut [u8]) -> std::io::Result<usize> {
                if self.0.is_empty() {
                    return Ok(0);
                }
                let piece = self.0.remove(0);
                into[..piece.len()].copy_from_slice(&piece);
                Ok(piece.len())
            }
        }
        struct KeepsThem(Vec<String>);
        impl ObservesEachLineAsItPasses for KeepsThem {
            fn a_line_passed(&mut self, _: Stream, line: &[u8]) {
                self.0.push(String::from_utf8_lossy(line).to_string());
            }
        }
        struct TakesEverything;
        impl CopiesAStreamIntoItsSink for TakesEverything {
            fn write(&self, _: Stream, bytes: &[u8], may: u64) -> Result<u64, String> {
                Ok((bytes.len() as u64).min(may))
            }
        }
        let takes = TakesEverything;
        let loop_ =
            TheWorkersCopyLoop::holding(&takes, a_ceiling_with_room(), KeepsThem(Vec::new()));
        loop_.one_stream(
            Stream::Ordinary,
            InTwoPieces(vec![b"one li".to_vec(), b"ne\r\ntrailing".to_vec()]),
        );
        let kept = loop_.observers.lock().unwrap();
        assert_eq!(kept.0, vec!["one line".to_string(), "trailing".to_string()]);
    }

    /// One pair's line exactly as `Dependency Upload & Remap` M8's fragment
    /// writes it, so these tests hold the real shape rather than a rehearsal
    /// of it.
    fn the_marker_line_for(pair: &smoozy_dependencies_machine::remap::Pair) -> String {
        use smoozy_dependencies_machine::material::AsTheMarkerCarriesIt;
        format!(
            "{}{}",
            THE_PER_PAIR_HEAD,
            serde_json::to_string(&AsTheMarkerCarriesIt {
                kind: serde_json::to_string(&pair.kind)
                    .expect("a kind renders")
                    .trim_matches('"')
                    .to_string(),
                setting: pair.setting.clone(),
                match_text: pair.match_text.clone(),
                applied: true,
                data_blocks: 1,
                assignment_alone: smoozy_dependencies_machine::remap::assignment_alone_is(pair.kind),
                reloaded: smoozy_dependencies_machine::remap::takes_a_reload_after_assignment(pair.kind),
                values_found: Vec::new(),
            })
            .expect("the line renders")
        )
    }

    /// **`Dependency Upload & Remap` M8, held at the loop that owns the
    /// stream: every pair the material carried is found in the marker.**
    ///
    /// The comparison existed in that crate and had no caller anywhere, so
    /// *reports every pair* was a sentence. **This runs the real loop**, with
    /// the pair observer riding it beside the marker's own, and reads the
    /// answer off the observer afterwards.
    #[test]
    fn every_remap_pair_of_the_material_is_found_in_the_marker_at_the_loop() {
        use smoozy_runtime::acting::capture::{TheCopyIntoTheSinks, TheSinksOnTheMachine};
        use smoozy_runtime::capture::MakesTheSinks;
        let root = a_root("pairs");
        let record = a_capture_record(&root);
        TheSinksOnTheMachine.make(&record).expect("the sinks");
        let copy = TheCopyIntoTheSinks::opening(&record).expect("opened");

        let job = crate::material::tests::a_job();
        let remap = crate::material::tests::a_remap(&job, true);
        let loop_ = TheWorkersCopyLoop::holding(
            &copy,
            record.ceiling,
            (
                TheMarkersObservation::default(),
                TheRemapPairsAsTheyPass::default(),
            ),
        );
        let stream = format!(
            "SMOOZY-MATERIAL {{\"opened\":\"j\"}}\n{}\n{}\nFra:1 Mem:12M\nsaved\n",
            the_marker_line_for(&remap.pairs[0]),
            the_marker_line_for(&remap.pairs[1]),
        );
        loop_.one_stream(Stream::Ordinary, stream.as_bytes());

        let observers = loop_.observers.lock().expect("the observers");
        let said = what_the_marker_said_about_the_remap_pairs(&observers.1, &remap);
        assert!(said.every_pair_was_reported());
        assert_eq!(said.per_pair.len(), remap.pairs.len());
        assert!(said.per_pair.iter().all(|(_, line)| line.is_some()));
        assert_eq!(said.pairs_the_marker_never_mentioned, 0);
        drop(observers);
        std::fs::remove_dir_all(&root).ok();
    }

    /// **A pair the marker never mentioned is named rather than lost**, which
    /// is the whole of what the comparison is for: without it a pair that
    /// produced no line at all looked exactly like a stream nobody had read.
    ///
    /// **The library pair is the one dropped here on purpose.** It is the kind
    /// whose remap is the difference between a render reading the artist's own
    /// machine and reading the machine it is on.
    #[test]
    fn a_remap_pair_the_marker_never_mentioned_is_counted_and_named() {
        use smoozy_runtime::acting::capture::{TheCopyIntoTheSinks, TheSinksOnTheMachine};
        use smoozy_runtime::capture::MakesTheSinks;
        let root = a_root("pairs-short");
        let record = a_capture_record(&root);
        TheSinksOnTheMachine.make(&record).expect("the sinks");
        let copy = TheCopyIntoTheSinks::opening(&record).expect("opened");

        let job = crate::material::tests::a_job();
        let remap = crate::material::tests::a_remap(&job, true);
        let loop_ =
            TheWorkersCopyLoop::holding(&copy, record.ceiling, TheRemapPairsAsTheyPass::default());
        let stream = format!(
            "{}\nFra:1 Mem:12M\nsaved\n",
            the_marker_line_for(&remap.pairs[0])
        );
        loop_.one_stream(Stream::Ordinary, stream.as_bytes());

        let observed = loop_.observers.lock().expect("the observers");
        assert_eq!(observed.how_many_lines_passed(), 1);
        let said = what_the_marker_said_about_the_remap_pairs(&observed, &remap);
        assert!(!said.every_pair_was_reported());
        assert_eq!(said.pairs_the_marker_never_mentioned, 1);
        assert!(said.per_pair[1].1.is_none());
        assert_eq!(
            said.per_pair[1].0.kind,
            smoozy_dependencies_machine::check::Kind::Library
        );
        drop(observed);
        std::fs::remove_dir_all(&root).ok();
    }

    /// **The material writes the marker on the ordinary stream and nowhere
    /// else**, so a line carrying the head on the complaints stream is not
    /// taken for one of its pairs — and a run with the lines only there reads
    /// as every pair unreported rather than as every pair fine.
    #[test]
    fn a_per_pair_line_on_the_complaints_stream_is_not_taken_for_the_materials() {
        let job = crate::material::tests::a_job();
        let remap = crate::material::tests::a_remap(&job, true);
        let mut observed = TheRemapPairsAsTheyPass::default();
        for pair in &remap.pairs {
            observed.a_line_passed(Stream::Complaints, the_marker_line_for(pair).as_bytes());
        }
        assert_eq!(observed.how_many_lines_passed(), 0);
        let said = what_the_marker_said_about_the_remap_pairs(&observed, &remap);
        assert_eq!(said.pairs_the_marker_never_mentioned, remap.pairs.len());
        assert!(!said.every_pair_was_reported());
    }

    /// **A stream that arrives a line at a time, with a pause before each** —
    /// a render's own printing is spread across the run, in bursts at each
    /// frame (`Queue & History` R4, observed in a test).
    struct AStreamThatArrivesSlowly {
        lines: std::collections::VecDeque<&'static str>,
        pause: std::time::Duration,
    }

    impl Read for AStreamThatArrivesSlowly {
        fn read(&mut self, into: &mut [u8]) -> std::io::Result<usize> {
            let Some(line) = self.lines.pop_front() else {
                return Ok(0);
            };
            std::thread::sleep(self.pause);
            into[..line.len()].copy_from_slice(line.as_bytes());
            Ok(line.len())
        }
    }

    /// **`Queue & History` R5's report goes out at the interval while the
    /// streams run, carrying the readings as they then stood, and stops with
    /// the streams** — the Creator's notes on 0.1.8, piece 6a.
    ///
    /// **What it bites on**: a loop that reports nothing (the state before
    /// this piece, where `the_report` had no caller), one that reports only
    /// at the end, and one that goes on sending after the render has ended.
    #[test]
    fn the_report_goes_out_while_the_streams_run_and_stops_when_they_end() {
        let loop_ = TheWorkersCopyLoop::holding(
            &ASinkThatTakesEverything,
            a_ceiling_with_room(),
            TheQueuesReaderAsATee::for_the_attempt(5, 1, &[1, 2, 3]).carrying_the_plan(Some(
                (1..=3)
                    .map(|frame| smoozy_queue_history_machine::plan::Renderlet {
                        frame,
                        view_layer: "ViewLayer".to_string(),
                        planned_samples: 16,
                        tiles: 1,
                    })
                    .collect(),
            )),
        );
        let sent: Mutex<Vec<AWorkersReport>> = Mutex::new(Vec::new());
        let stream = AStreamThatArrivesSlowly {
            lines: [
                "00:00.405  render           | Rendering frame 1\n",
                "00:00.430  render           | Fra: 1 | Remaining: 00:00.05 | Mem: 2M | Sample 1/16\n",
                "00:00.935  render           | Saved: '/out/frame_0001.png'\n",
                "00:00.940  render           | Rendering frame 2\n",
                "00:00.960  render           | Fra: 2 | Mem: 2M | Sample 16/16\n",
                "00:01.935  render           | Saved: '/out/frame_0002.png'\n",
                "00:01.940  render           | Rendering frame 3\n",
            ]
            .into_iter()
            .collect(),
            pause: std::time::Duration::from_millis(60),
        };
        let clock = Clock::AMachine(smoozy_state::observed::MachineClock("installation-1".into()));
        let (ordinary, _) = loop_.run_reporting_every(
            std::time::Duration::from_millis(25),
            stream,
            &b""[..],
            |tee: &TheQueuesReaderAsATee| tee.the_report(std::time::SystemTime::now(), clock.clone()),
            |report| sent.lock().unwrap().push(report),
        );
        assert!(ordinary.end_of_stream);
        let sent_by_the_end = sent.lock().unwrap().len();
        std::thread::sleep(std::time::Duration::from_millis(100));
        let sent = sent.into_inner().unwrap();
        assert_eq!(sent.len(), sent_by_the_end, "nothing is sent once the streams have ended");
        assert!(sent.len() >= 5, "a report at every interval while the streams ran: {}", sent.len());
        let frames: Vec<Option<i64>> = sent.iter().map(|report| report.readings.value().frame_in_hand).collect();
        assert!(frames.contains(&Some(1)) && frames.contains(&Some(2)), "{frames:?}");
        assert!(
            sent.iter().all(|report| report.the_plan_total == Some(48)),
            "`Queue & History` B1's total rides every report"
        );
        assert!(
            sent.iter().any(|report| report.newest_frame_saved == Some(1)),
            "`Queue & History` P1a's figure rides it too"
        );
        // **`Queue & History` B6's finished part rides beside the total and
        // follows the saves** (piece 6b) **and the frame in hand** (the
        // Creator's second notes, piece 3): frame 1 at 1 of 16 is 1 unit
        // before any save, 16 once it is saved, 32 once frame 2 reads 16 of 16.
        let finished: Vec<Option<u64>> = sent.iter().map(|report| report.the_units_finished).collect();
        assert!(
            finished.contains(&Some(1)) && finished.contains(&Some(16)) && finished.contains(&Some(32)),
            "{finished:?}"
        );
        assert!(
            sent.iter().all(|report| report.the_units_finished
                >= Some(16 * report.newest_frame_saved.unwrap_or(0) as u64)),
            "never less than the saved frames' own units: {finished:?}"
        );
        assert!(finished.windows(2).all(|two| two[0] <= two[1]), "never backwards: {finished:?}");
    }

    /// **The Creator's second notes, piece 3 (note 13): the second bar moves
    /// inside a frame**, off the lines a real Blender printed at the desk for
    /// one frame of two view layers at 1,024 samples each (shortened to the
    /// lines the reader reads). **What it bites on**: the figure of
    /// 2026-10-03, which read nought until the save; a figure read off the
    /// frame's fold alone, which the second layer's opening `Sample 0/1024`
    /// marks stopped short and so reads finished half-way; and a figure that
    /// falls when that layer's count starts again.
    ///
    /// **Piece 3b: and the first bar's readings start over at each layer**, in
    /// both grammars — 5.0 and later by the layer's own announcement — so the
    /// second layer's opening zero is that layer's first reading, never the
    /// stopped-short mark that filled the top bar for the whole layer.
    #[test]
    fn a_frame_cut_short_in_5_3_ends_its_unit_on_finished_and_the_next_layer_starts_over() {
        let two_layers = ["First", "Second"].map(|layer| smoozy_queue_history_machine::plan::Renderlet {
            frame: 1,
            view_layer: layer.to_string(),
            planned_samples: 4_096,
            tiles: 1,
        });
        let at = std::time::SystemTime::UNIX_EPOCH;
        let mut reader = TheQueuesReaderAsATee::for_the_attempt(5, 1, &[1])
            .carrying_the_plan(Some(two_layers.to_vec()));
        for line in [
            "00:00.406  render           | Rendering frame 1",
            "00:00.407  render           | Start rendering: Scene, First",
            "00:02.000  render           | Fra: 1 | Mem: 5M | Sample 160/4096",
        ] {
            reader.a_line_passed(Stream::Ordinary, line.as_bytes());
        }
        assert!(!reader.readings().stopped_short, "still sampling");
        assert_eq!(
            reader.the_report(at, Clock::ThisComputer).the_units_finished,
            Some(160),
            "the second bar's share is the count so far"
        );
        reader.a_line_passed(
            Stream::Ordinary,
            "00:02.500  render           | Fra: 1 | Mem: 5M | Finished".as_bytes(),
        );
        assert!(reader.readings().stopped_short, "`Queue & History` B3b: the unit has ended");
        assert_eq!(
            reader.readings().samples.map(|pair| (pair.seen, pair.of)),
            Some((160, 4_096)),
            "the figures stay as printed"
        );
        assert_eq!(reader.the_report(at, Clock::ThisComputer).the_units_finished, Some(4_096));
        reader.a_line_passed(
            Stream::Ordinary,
            "00:02.600  render           | Start rendering: Scene, Second".as_bytes(),
        );
        assert!(!reader.readings().stopped_short, "the next unit starts over");
        reader.a_line_passed(
            Stream::Ordinary,
            "00:03.000  render           | Fra: 1 | Mem: 5M | Sample 32/4096".as_bytes(),
        );
        assert_eq!(
            reader.the_report(at, Clock::ThisComputer).the_units_finished,
            Some(4_096 + 32),
            "the first layer stays whole"
        );
    }

    /// **`Queue & History` R12's line of 2026-11-02, through the worker's own
    /// reader: `Finished` takes a tiled frame's share of the second bar to its
    /// end, once per layer, and no further.** Both layers at 4 tiles and 4,096
    /// samples are 32,768 units. **What it bites on**: at `3/4 tiles` the share
    /// sat at three quarters of the layer through denoising and saving, the
    /// live pass's *Whole job 75%*.
    #[test]
    fn a_tiled_frame_ends_its_unit_on_finished_and_the_second_bar_never_passes_the_frame() {
        let two_layers = ["First", "Second"].map(|layer| smoozy_queue_history_machine::plan::Renderlet {
            frame: 1,
            view_layer: layer.to_string(),
            planned_samples: 4_096,
            tiles: 4,
        });
        let at = std::time::SystemTime::UNIX_EPOCH;
        let mut reader = TheQueuesReaderAsATee::for_the_attempt(5, 1, &[1])
            .carrying_the_plan(Some(two_layers.to_vec()));
        for line in [
            "00:00.406  render           | Rendering frame 1",
            "00:00.407  render           | Start rendering: Scene, First",
            "00:39.309  render           | Fra: 1 | Mem: 574M | Rendered 3/4 Tiles, Sample 48/4096",
        ] {
            reader.a_line_passed(Stream::Ordinary, line.as_bytes());
        }
        assert_eq!(
            reader.the_report(at, Clock::ThisComputer).the_units_finished,
            Some(3 * 4_096 + 48),
            "three tiles and the tile in hand"
        );
        reader.a_line_passed(Stream::Ordinary, "00:39.430  render           | Fra: 1 | Mem: 574M | Finished".as_bytes());
        assert!(reader.readings().stopped_short);
        assert_eq!(reader.the_report(at, Clock::ThisComputer).the_units_finished, Some(4 * 4_096), "the layer is whole");
        assert_eq!(reader.readings().tiles.map(|pair| (pair.done, pair.of)), Some((3, 4)), "the figures stay as printed");
        reader.a_line_passed(Stream::Ordinary, "00:39.454  render           | Fra: 1 | Mem: 496M | Reading full buffer from disk".as_bytes());
        assert_eq!(reader.the_report(at, Clock::ThisComputer).the_units_finished, Some(4 * 4_096), "denoising moves nothing");
        reader.a_line_passed(Stream::Ordinary, "00:39.600  render           | Start rendering: Scene, Second".as_bytes());
        assert!(!reader.readings().stopped_short, "the next unit starts over");
        reader.a_line_passed(Stream::Ordinary, "00:40.000  render           | Fra: 1 | Mem: 574M | Rendered 0/4 Tiles, Sample 16/4096".as_bytes());
        assert_eq!(reader.the_report(at, Clock::ThisComputer).the_units_finished, Some(4 * 4_096 + 16), "the first layer stays whole");
        reader.a_line_passed(Stream::Ordinary, "00:50.000  render           | Fra: 1 | Mem: 574M | Rendered 3/4 Tiles, Sample 4096/4096".as_bytes());
        reader.a_line_passed(Stream::Ordinary, "00:50.100  render           | Fra: 1 | Mem: 574M | Finished".as_bytes());
        assert_eq!(reader.the_report(at, Clock::ThisComputer).the_units_finished, Some(8 * 4_096), "both whole, and never past the frame");
    }

    #[test]
    fn one_frame_of_two_layers_moves_the_second_bar_as_each_layer_renders() {
        let two_layers = |frame: i64| {
            ["First", "Second"].map(|layer| smoozy_queue_history_machine::plan::Renderlet {
                frame,
                view_layer: layer.to_string(),
                planned_samples: 1_024,
                tiles: 1,
            })
        };
        let at = std::time::SystemTime::UNIX_EPOCH;
        let figure = |reader: &TheQueuesReaderAsATee| {
            reader
                .the_report(at, Clock::ThisComputer)
                .the_units_finished
                .expect("the plan is carried")
        };

        // **4.5**: the line names the layer, so each renderlet is the unit.
        let mut reader = TheQueuesReaderAsATee::for_the_attempt(4, 1, &[1])
            .carrying_the_plan(Some(two_layers(1).to_vec()));
        let mut read = Vec::new();
        for line in [
            "Fra:1 Mem:8.30M (Peak 9.10M) | Time:00:00.02 | Mem:0.09M, Peak:0.09M | Scene, First | Sample 0/1024",
            "Fra:1 Mem:16.29M (Peak 16.29M) | Time:00:00.05 | Remaining:00:24.09 | Mem:8.00M, Peak:8.00M | Scene, First | Sample 512/1024",
            "Fra:1 Mem:32.12M (Peak 47.94M) | Time:00:27.01 | Mem:8.00M, Peak:8.00M | Scene, First | Sample 1024/1024",
            "Fra:1 Mem:24.13M (Peak 47.94M) | Time:00:27.02 | Mem:0.09M, Peak:0.09M | Scene, Second | Sample 0/1024",
            "Fra:1 Mem:32.12M (Peak 47.94M) | Time:00:27.04 | Remaining:00:22.38 | Mem:8.00M, Peak:8.00M | Scene, Second | Sample 512/1024",
            "Saved: '/out/f_0001.png'",
        ] {
            reader.a_line_passed(Stream::Ordinary, line.as_bytes());
            read.push(figure(&reader));
            if line.contains("Second | Sample 0/1024") {
                let top = reader.readings();
                assert_eq!(top.view_layer.as_deref(), Some("Second"), "4.5");
                assert_eq!(top.samples.map(|pair| (pair.seen, pair.of)), Some((0, 1_024)), "4.5");
                assert!(!top.stopped_short, "4.5: the opening zero marks nothing");
            }
        }
        assert_eq!(read, vec![0, 512, 1_024, 1_024, 1_536, 2_048], "4.5");

        // **5.2**: each layer is announced, so each renderlet is the unit as
        // in 4.5, and the second layer's opening zero starts that layer.
        let mut reader = TheQueuesReaderAsATee::for_the_attempt(5, 1, &[1])
            .carrying_the_plan(Some(two_layers(1).to_vec()));
        let mut read = Vec::new();
        for line in [
            "00:00.406  render           | Rendering frame 1",
            "00:00.407  render           | Start rendering: Scene, First",
            "00:00.429  render           | Fra: 1 | Mem: 1M | Sample 0/1024",
            "00:00.452  render           | Fra: 1 | Remaining: 00:24.40 | Mem: 9M | Sample 1/1024",
            "00:26.288  render           | Fra: 1 | Mem: 9M | Sample 1024/1024",
            "00:26.288  render           | Start rendering: Scene, Second",
            "00:26.290  render           | Fra: 1 | Mem: 1M | Sample 0/1024",
            "00:26.312  render           | Fra: 1 | Remaining: 00:22.42 | Mem: 9M | Sample 1/1024",
            "00:52.082  render           | Fra: 1 | Mem: 9M | Sample 1024/1024",
            "00:52.174  render           | Saved: '/out/f_0001.png'",
        ] {
            reader.a_line_passed(Stream::Ordinary, line.as_bytes());
            read.push(figure(&reader));
            if line.starts_with("00:26.290") {
                let top = reader.readings();
                assert_eq!(top.view_layer.as_deref(), Some("Second"), "5.2");
                assert_eq!(top.samples.map(|pair| (pair.seen, pair.of)), Some((0, 1_024)), "5.2");
                assert!(!top.stopped_short, "5.2: the opening zero marks nothing");
            }
        }
        assert_eq!(read, vec![0, 0, 0, 1, 1_024, 1_024, 1_024, 1_025, 2_048, 2_048], "5.2");

        // **The reader keeps the last figure it sent**: a frame that moves on
        // with no save announced never drops the bar back to nought. **Since
        // 2026-10-17** (the Creator's eleventh notes, note 1: `Queue & History`
        // B6, a frame of the list counts as finished once a later frame of it
        // has begun) frame 1 counts whole the moment frame 2 begins; until
        // then the figure held at frame 1's share, 512.
        let mut reader = TheQueuesReaderAsATee::for_the_attempt(5, 1, &[1, 2])
            .carrying_the_plan(Some([two_layers(1), two_layers(2)].concat()));
        let mut read = Vec::new();
        for line in [
            "00:00.406  render           | Rendering frame 1",
            "00:00.452  render           | Fra: 1 | Mem: 9M | Sample 512/1024",
            "00:26.288  render           | Rendering frame 2",
        ] {
            reader.a_line_passed(Stream::Ordinary, line.as_bytes());
            read.push(figure(&reader));
        }
        assert_eq!(read, vec![0, 512, 2_048], "frame 1 ended when frame 2 began");
    }

    /// **The Creator's second notes, piece 3 (note 13), at the desk.** A real
    /// Blender renders a one-frame file with two view layers through this
    /// loop, with `Queue & History`'s reader riding it and carrying the file's
    /// own plan — one frame, two renderlets of `SMOOZY_DESK_SAMPLES` each —
    /// and every report that leaves it is printed with the second bar it makes.
    ///
    /// **Why this loop and not the whole program**: macOS publishes no
    /// per-process record, so the program undoes every render it starts at
    /// this desk (`Rented-server runtime` X1, found by the 0.1.8 notes' piece
    /// 6a). The reports the program carries are this loop's, built by this
    /// reader, so this is the figure the desktop would draw.
    #[test]
    #[ignore = "run by hand at the desk with a real Blender and a real .blend"]
    fn a_real_blender_renders_one_frame_of_two_layers_and_each_report_is_printed() {
        use smoozy_runtime::acting::capture::{TheCopyIntoTheSinks, TheSinksOnTheMachine};
        use smoozy_runtime::capture::MakesTheSinks;
        let blender = std::env::var("SMOOZY_DESK_BLENDER").expect("SMOOZY_DESK_BLENDER");
        let blend = std::env::var("SMOOZY_DESK_BLEND").expect("SMOOZY_DESK_BLEND");
        let major: u32 = std::env::var("SMOOZY_DESK_MAJOR")
            .expect("SMOOZY_DESK_MAJOR")
            .parse()
            .expect("a number");
        let samples: u64 = std::env::var("SMOOZY_DESK_SAMPLES")
            .expect("SMOOZY_DESK_SAMPLES")
            .parse()
            .expect("a number");
        let root = a_root("a-real-two-layer-frame");
        let record = a_capture_record(&root);
        TheSinksOnTheMachine.make(&record).expect("the sinks");
        let copy = TheCopyIntoTheSinks::opening(&record).expect("opened");
        let plan: Vec<smoozy_queue_history_machine::plan::Renderlet> = ["First", "Second"]
            .into_iter()
            .map(|layer| smoozy_queue_history_machine::plan::Renderlet {
                frame: 1,
                view_layer: layer.to_string(),
                planned_samples: samples,
                // `Queue & History` B2's grid count for the file's size and tile size, where
                // the file is tiled (piece 3b: the Creator's 4K shape).
                tiles: std::env::var("SMOOZY_DESK_TILES").ok().and_then(|tiles| tiles.parse().ok()).unwrap_or(1),
            })
            .collect();
        let began = std::time::Instant::now();
        let loop_ = TheWorkersCopyLoop::holding(
            &copy,
            record.ceiling,
            (
                TheQueuesReaderAsATee::for_the_attempt(major, 1, &[1])
                    .carrying_the_plan(Some(plan))
                    // The scene's name, where one is given (piece 3c).
                    .knowing_the_scene(std::env::var("SMOOZY_DESK_SCENE").ok()),
                EachCountAsItPasses(began),
            ),
        );
        // **A script run before the render, where one is named** — at the
        // desk, the one that turns this Mac's graphics card on, which a
        // factory start leaves off (piece 3b, on the Creator's word that the
        // proof renders run on the card where Blender offers it).
        let before: Vec<String> = std::env::var("SMOOZY_DESK_BEFORE")
            .map(|script| vec!["--python".to_string(), script])
            .unwrap_or_default();
        let mut child = std::process::Command::new(&blender)
            .args(["--factory-startup", "-b", &blend])
            .args(&before)
            .arg("-o")
            .arg(root.join("out/f_"))
            .arg("-a")
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("the real Blender starts");
        let ordinary = child.stdout.take().expect("piped");
        let complaints = child.stderr.take().expect("piped");
        // **Stopped once enough has been seen, where a limit is named** — the
        // Creator's word, piece 3b.
        if let Some(seconds) = std::env::var("SMOOZY_DESK_STOP_AFTER").ok().and_then(|s| s.parse::<u64>().ok()) {
            let pid = child.id().to_string();
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_secs(seconds));
                std::process::Command::new("kill").arg(&pid).status().ok();
            });
        }
        let clock = Clock::AMachine(smoozy_state::observed::MachineClock("installation-1".into()));
        loop_.run_reporting_every(
            THE_WORKERS_REPORTING_INTERVAL,
            ordinary,
            complaints,
            |both: &(TheQueuesReaderAsATee, EachCountAsItPasses)| {
                both.0.the_report(std::time::SystemTime::now(), clock.clone())
            },
            |report| {
                let read = report.readings.value();
                // **The first bar as the desktop draws it** (`smoozy-queue-history`'s
                // `the_two_bars_from_what_the_machine_carries`, which this crate
                // cannot take): against the total printed beside the count, `Queue & History` B4's
                // arithmetic on a tiled line, full on the stopped-short mark.
                // Piece 3c.
                let first = if read.stopped_short {
                    Some(100)
                } else {
                    match (read.tiles, read.samples) {
                        (Some(tiles), samples) if tiles.of > 0 => {
                            let (done, of) = smoozy_queue_history_machine::plan::the_part_of_a_tiled_renderlet(
                                tiles,
                                samples,
                                samples.map(|pair| pair.of).unwrap_or(0),
                            );
                            Some(done * 100 / of.max(1))
                        }
                        (None, Some(pair)) if pair.of > 0 => Some(pair.seen.min(pair.of) * 100 / pair.of),
                        _ => None,
                    }
                };
                println!(
                    "{:>6.1}s REPORT frame {:?}, layer {:?}, samples {:?}, tiles {:?}, stopped short {}, \
                     newest saved {:?}, plan total {:?}, finished {:?}, first bar {:?}%, second bar {:?}%",
                    began.elapsed().as_secs_f64(),
                    read.frame_in_hand,
                    read.view_layer,
                    read.samples.map(|pair| (pair.seen, pair.of)),
                    read.tiles.map(|pair| (pair.done, pair.of)),
                    read.stopped_short,
                    report.newest_frame_saved,
                    report.the_plan_total,
                    report.the_units_finished,
                    first,
                    report
                        .the_units_finished
                        .zip(report.the_plan_total)
                        .map(|(done, total)| done * 100 / total.max(1)),
                );
            },
        );
        let status = child.wait().expect("Blender ends");
        println!("Blender ended {status:?} after {:?}", began.elapsed());
        if let Ok(keep) = std::env::var("SMOOZY_DESK_KEEP_THE_STREAM") {
            std::fs::copy(&record.ordinary_sink, keep).expect("the stream is kept");
        }
        std::fs::remove_dir_all(&root).ok();
    }

    /// **Every line carrying a count, a tile figure or a layer's start,
    /// printed with the moment it passed** — how often each Blender prints a
    /// count is what the Creator asked to see (piece 3b).
    struct EachCountAsItPasses(std::time::Instant);

    impl ObservesEachLineAsItPasses for EachCountAsItPasses {
        fn a_line_passed(&mut self, stream: Stream, line: &[u8]) {
            let text = String::from_utf8_lossy(line);
            if ["Sample ", "Tiles", "Start rendering", "Rendering ", "Saved:", "SMOOZY DESK CARD", "Path tracing on"]
                .iter()
                .any(|head| text.contains(head))
            {
                println!("{:>6.1}s LINE {stream:?} {}", self.0.elapsed().as_secs_f64(), text.trim_end());
            }
        }
    }

    /// A sink that takes every byte offered and keeps none of them.
    struct ASinkThatTakesEverything;

    impl CopiesAStreamIntoItsSink for ASinkThatTakesEverything {
        fn write(&self, _: Stream, bytes: &[u8], _: u64) -> Result<u64, String> {
            Ok(bytes.len() as u64)
        }
    }

    #[derive(Debug, Default)]
    struct ACountOfLines(usize);

    impl ObservesEachLineAsItPasses for ACountOfLines {
        fn a_line_passed(&mut self, _: Stream, _: &[u8]) {
            self.0 += 1;
        }
    }
}


#[cfg(test)]
mod wait_count_tests {
    use super::*;
    #[test]
    fn wait_count_reaches_the_report_without_counting_a_rendered_frame() {
        for major in [4, 5] {
            let mut reader = TheQueuesReaderAsATee::for_the_attempt(major, 1, &[-2, 1]);
            reader.a_line_passed(Stream::Ordinary, b"SMOOZY-SIMULATION {\"frame\":-2,\"to\":1}");
            assert_eq!(reader.readings().simulation, Some((-2, 1)));
            assert_eq!(reader.readings().frame_in_hand, None);
            assert_eq!(reader.readings().frames_completed, 0);
            let report = reader.the_report(std::time::SystemTime::UNIX_EPOCH, Clock::ThisComputer);
            let pairs = report.as_pairs();
            let read = AWorkersReport::read_back(|key| pairs.iter().find(|(name, _)| *name == key).map(|(_, value)| value.as_str())).unwrap();
            assert_eq!(read.readings.value().simulation, Some((-2, 1)));
            for bad in [b"SMOOZY-SIMULATION {\"frame\":99,\"to\":1}".as_slice(), b"SMOOZY-SIMULATION broken"] {
                reader.a_line_passed(Stream::Ordinary, bad);
                assert_eq!(reader.readings().simulation, Some((-2, 1)));
            }
            reader.a_line_passed(Stream::Complaints, b"SMOOZY-SIMULATION {\"finished\":true}");
            assert_eq!(reader.readings().simulation, Some((-2, 1)));
            reader.a_line_passed(Stream::Ordinary, b"SMOOZY-SIMULATION {\"finished\":true}");
            assert_eq!(reader.readings().simulation, None);
            reader.a_line_passed(Stream::Ordinary, b"SMOOZY-SIMULATION {\"frame\":0,\"to\":1}");
            let render = if major == 4 { "Fra:1 | Sample 1/16" } else { "00:01.000 render | Fra: 1 | Sample 1/16" };
            reader.a_line_passed(Stream::Ordinary, render.as_bytes());
            assert_eq!(reader.readings().simulation, None);
            assert_eq!(reader.readings().frame_in_hand, Some(1));
        }
    }
}
