//! The readings the judgements stand on, and the shapes of the five
//! observations the render worker takes at its own copy loop — `Queue &
//! History` R1, R5, R6, R11–R13's judgements, H7's reading, P1a's figure,
//! E13's and E13a's observations, and §17's supply to `Rendering`.
//!
//! **R1: there is one reader, it rides the render worker's own copy loop, and
//! it produces readings, never state.** The worker already holds both of the
//! render's streams and copies them to the sinks itself; the reader is a tee
//! on that loop, observing bytes as they pass. **Nothing the reader produces
//! is ever a job state**: the ending comes from the operation record, the
//! group's validation and the reaped exit, and a stream that has stopped
//! moving is not an ending. **So this file holds shapes and folds, and no
//! function in it returns a [`crate::line::QueueState`] or an
//! [`crate::ending::Ending`].**
//!
//! **What is here and what is the acting half's.** The grammar per version
//! (R2), the tee itself (R1, R8), the reporting over the endpoint (R5) and
//! the parse of each kind of line (R7, R9, R10, R14, R15) are the acting
//! half's — `Rendering`'s worker performs the observations and this crate's
//! reader parses them. **The closed list of readings (R6), the largest-seen
//! fold (R11), the tiled-or-not condition read off the line (R12, R13) and
//! the five observation shapes are judgements and data, and they are here**,
//! because the bars in `smoozy-queue-history`'s `bars` consume them.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rented-server runtime` M3, M4, M28, W12, U2.
//! `Rendering` K13. `Download` H7a.

use std::time::SystemTime;

use smoozy_state::identities::Identity;
use smoozy_state::observed::Reading;

/// **R6's closed list: the readings the reader produces.** Nothing else is
/// read, apart from R6a's simulation pair before rendering: no percentage,
/// no card memory, no device name (that is a closing
/// report, E13) and no colour-substitution line (E13a). **A builder adding
/// anything to this list has found a fault rather than a case.**
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LiveReadings {
    /// `Queue & History` R6a: scene frames being simulated, never rendered work.
    pub simulation: Option<(i64, i64)>,
    /// **R9: the frame in hand** — read off the line in 4.5, counted off the
    /// announcements in 5.0 and later.
    pub frame_in_hand: Option<i64>,
    /// **R10: the view layer in hand** — off 4.5's progress line, and from
    /// 5.0 off the layer's own announcement (R10 as corrected 2026-10-04).
    /// Absent where nothing names it (4.5 under EEVEE, observed at the desk),
    /// and the entry then names the frame alone — the spec's own allowed
    /// fallback.
    pub view_layer: Option<String>,
    /// **R11: the largest sample count seen for the frame in hand, with the
    /// printed total beside it**, both shown as printed, never converted into
    /// a claim of completion.
    pub samples: Option<SamplePair>,
    /// **R12: the mark that sampling on this unit has ended.** On an untiled
    /// frame the trailing zero says the frame ended short of its count and does
    /// not say which cause; in 5.3, which prints no such zero, `Finished` on a
    /// frame short of its count says the same (R12's line of 2026-11-02). **On a
    /// frame with tiles `Finished` sets it at any count** (the same day's second
    /// line), every count there being a tile's. It fills the bars and says
    /// nothing else: never a picture on disk (E3), and never a claim about how
    /// many samples were taken (R11).
    pub stopped_short: bool,
    /// **R13: the tile figures where a tiled line carries them**, shown beside
    /// the samples and never combined with them into one figure.
    pub tiles: Option<TilePair>,
    /// **R15: Cycles' time estimate for the frame in hand, where a line
    /// carries one**, marked as Blender's and as rough, and never fed to a
    /// bar. Absent on EEVEE and Workbench, and on a line that carries none.
    pub blenders_own_estimate_for_the_frame: Option<String>,
    /// **R6's last: the count of frames announced or completed**, which is
    /// what the bars consume.
    pub frames_completed: u64,
}

/// A sample pair as the line printed it: count against total.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SamplePair {
    pub seen: u64,
    pub of: u64,
}

/// A tile pair as the line printed it: done against total.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TilePair {
    pub done: u64,
    pub of: u64,
}

/// **One progress line, as the reader's grammar read it** — the acting
/// half's parse, handed in as a value. **R12: the deciding condition is the
/// printed line, never a computed size: a line carrying a tile figure is a
/// tiled frame, and a line carrying none is not.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AProgressLine {
    pub frame: Option<i64>,
    pub view_layer: Option<String>,
    pub samples: Option<SamplePair>,
    pub tiles: Option<TilePair>,
    pub estimate: Option<String>,
    /// **R12's line of 2026-11-02: the line ends on the stage word `Finished`**,
    /// the end of the sampling on what it is about. **Never that a picture
    /// reached disk** (E3), which is why this is not one of R6's readings: it
    /// reaches nothing but the fold's mark.
    pub the_line_says_finished: bool,
}

impl AProgressLine {
    /// **R12, R13: tiled or not, decided by the line alone.** 2048×2048 at
    /// the default tile size printed no tile figure; 2049×2049 printed four.
    pub fn is_a_tiled_frame(&self) -> bool {
        self.tiles.is_some()
    }
}

/// **R11's fold: the largest seen for the frame in hand, never the latest.**
/// A frame that stops short prints one line reading zero of the total just
/// before it ends, so the latest reading is wrong at exactly the moment a
/// frame ends early. **On a tiled frame the sample number restarts inside
/// each tile against the scene's own undivided maximum (R13), so what is
/// read there is that tile's count and it is shown beside the tile figure as
/// the pair the line printed — never carried across tiles, never presented as
/// the frame's count.**
///
/// **R12: on an untiled frame the trailing zero is the stopped-short mark; on
/// a tiled frame no line carrying a tile figure produces it, and `Finished`
/// does** (R12's line of 2026-11-02, B3b). A new frame in hand starts the fold
/// over.
///
/// **So does a new view layer on the same frame** — B3's *"it fills and
/// finishes once per renderlet"*, and B3b's *the next unit beginning*. Added
/// 2026-10-04, the Creator's second notes, piece 3b: the fold had started
/// over on a new frame alone, so on a frame of two layers the second layer's
/// opening `Sample 0/1024` met the first layer's `1024/1024` and read as the
/// stopped-short mark, and the top bar sat full for the whole second layer
/// (observed at the desk in 5.2.2 and 4.5.12). **Only a named layer is a
/// boundary** — never a falling count (B3a).
pub fn fold_a_line(held: &LiveReadings, line: &AProgressLine) -> LiveReadings {
    let a_new_frame = line.frame.is_some() && line.frame != held.frame_in_hand;
    let a_new_layer = line.view_layer.is_some() && line.view_layer != held.view_layer;
    let mut next = if a_new_frame || a_new_layer {
        LiveReadings {
            frame_in_hand: line.frame.or(held.frame_in_hand),
            frames_completed: held.frames_completed,
            ..LiveReadings::default()
        }
    } else {
        held.clone()
    };
    next.simulation = None;
    if line.view_layer.is_some() {
        next.view_layer = line.view_layer.clone();
    }
    next.blenders_own_estimate_for_the_frame = line.estimate.clone();
    if line.is_a_tiled_frame() {
        // The tile's own pair, as printed; no frame-wide fold, and no line with
        // a tile figure sets the mark.
        next.tiles = line.tiles;
        next.samples = line.samples;
        next.stopped_short = false;
        return next;
    }
    if let Some(printed) = line.samples {
        match next.samples {
            Some(largest) if largest.seen >= printed.seen => {
                // The trailing zero on an untiled frame: the stopped-short
                // mark, and the largest-seen reading stands.
                if printed.seen == 0 && largest.seen > 0 {
                    next.stopped_short = true;
                }
            }
            _ => next.samples = Some(printed),
        }
    }
    // **R12's line of 2026-11-02, for B3b: 5.3 prints no closing zero**, so a
    // frame cut short goes `Sample 160/4096`, then `Finished`. The one rule that
    // reads every version is the largest count seen against its maximum, read
    // here as the sampling ends; older versions' zero has set the mark by now
    // and a frame at its full count sets nothing. **The `Finished` line carries
    // no tile figure, so a tiled frame is known by the figure it already
    // printed. There every count is a tile's, so a count short of its maximum
    // says nothing, and `Finished` is the mark at any count** (`Queue & History`
    // R12's second line of 2026-11-02: every real 4K frame on a card has tiles,
    // and both bars sat at `3/4` through denoising and saving, in every version).
    if line.the_line_says_finished
        && (next.tiles.is_some() || next.samples.is_some_and(|largest| largest.seen < largest.of))
    {
        next.stopped_short = true;
    }
    next
}

/// **R5: readings travel worker → supervisor → desktop, and are
/// period-held.** The worker reports its current readings to the supervisor
/// at its own bounded interval; every reading carries the moment it was
/// taken and the clock it was taken on (`State, identity and recovery` D5);
/// on the desktop it is period-held and nothing durable is written from it.
/// Across a supervisor restart the reporting re-establishes on the worker's
/// own next interval by `Rented-server runtime` W12's route, so a resumed
/// render's entry carries no live figures for the gap — absent readings,
/// which R4 already makes an ordinary state of the entry, never a stale one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AWorkersReport {
    pub attempt: u32,
    pub readings: Reading<LiveReadings>,
    /// **P1a's figure: the newest frame of the attempt's list whose save has
    /// been announced** — frames are worked through ascending, one at a time,
    /// in the order given, so the figure marks the list's completed prefix.
    /// `None` where no save has been announced yet, and where the stream
    /// prints nothing at all (R4's two cases) no report ever arrives.
    pub newest_frame_saved: Option<i64>,
    /// **B1's plan total for the job**, worked out once by the render worker
    /// from the reading it took at `Rendering` P1 step 4, and carried in every
    /// report. `None` where no total could be worked out — no reading stood,
    /// or the engine in force counts no samples. Added 2026-10-03, the
    /// Creator's notes on 0.1.8, piece 6a.
    pub the_plan_total: Option<u64>,
    /// **B6's finished part: the plan units of the frames finished so far**,
    /// worked out by the worker by the same arithmetic as the total
    /// ([`crate::plan::the_units_finished`]), so the second bar is this over
    /// [`AWorkersReport::the_plan_total`] and both come from the machine.
    /// `None` wherever the total is. Added 2026-10-03, the Creator's notes on
    /// 0.1.8, piece 6b.
    pub the_units_finished: Option<u64>,
}

impl AWorkersReport {
    /// **The report as named fields** — the worker's report to the supervisor
    /// and the queue reading's part for a rendering job carry the same names
    /// (`smoozy_framing::requests`), composed here and read back by
    /// [`AWorkersReport::read_back`], so neither end spells them.
    ///
    /// **A figure the stream never printed is left out, never sent as a
    /// zero** (R4). The moment, its clock, the attempt and the count of frames
    /// are always there.
    pub fn as_pairs(&self) -> Vec<(&'static str, String)> {
        use smoozy_framing::requests as asked;
        use smoozy_state::observed::{Clock, MachineClock};
        let seconds = self
            .readings
            .taken_at()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|since| since.as_secs())
            .unwrap_or(0);
        let clock = match self.readings.clock() {
            Clock::AMachine(MachineClock(name)) => name.clone(),
            Clock::ThisComputer => THIS_COMPUTER.to_string(),
        };
        let read = self.readings.value();
        let mut pairs = vec![
            (asked::THE_ATTEMPT_FIELD, self.attempt.to_string()),
            (asked::THE_FIGURES_TAKEN_AT_FIELD, seconds.to_string()),
            (asked::THE_FIGURES_CLOCK_FIELD, clock),
            (asked::THE_FRAMES_COUNTED_FIELD, read.frames_completed.to_string()),
        ];
        // `Queue & History` R6a: these optional fields use the same report
        // on both sides; older reports simply carry no simulation reading.
        if let Some((frame, to)) = read.simulation {
            pairs.push((THE_SIMULATION_FRAME, frame.to_string()));
            pairs.push((THE_SIMULATION_END, to.to_string()));
        }
        if let Some(frame) = read.frame_in_hand {
            pairs.push((asked::THE_FRAME_IN_HAND_FIELD, frame.to_string()));
        }
        if let Some(layer) = &read.view_layer {
            pairs.push((asked::THE_VIEW_LAYER_IN_HAND_FIELD, layer.clone()));
        }
        if let Some(samples) = read.samples {
            pairs.push((asked::THE_SAMPLES_SEEN_FIELD, samples.seen.to_string()));
            pairs.push((asked::THE_SAMPLES_PRINTED_TOTAL_FIELD, samples.of.to_string()));
        }
        if read.stopped_short {
            pairs.push((asked::THE_FRAME_STOPPED_SHORT_FIELD, asked::THE_ANSWER_IS_YES.to_string()));
        }
        if let Some(tiles) = read.tiles {
            pairs.push((asked::THE_TILES_DONE_FIELD, tiles.done.to_string()));
            pairs.push((asked::THE_TILES_TOTAL_FIELD, tiles.of.to_string()));
        }
        if let Some(estimate) = &read.blenders_own_estimate_for_the_frame {
            pairs.push((asked::BLENDERS_OWN_ESTIMATE_FIELD, estimate.clone()));
        }
        if let Some(frame) = self.newest_frame_saved {
            pairs.push((asked::THE_NEWEST_FRAME_SAVED_FIELD, frame.to_string()));
        }
        if let Some(total) = self.the_plan_total {
            pairs.push((asked::THE_PLAN_TOTAL_FIELD, total.to_string()));
        }
        if let Some(finished) = self.the_units_finished {
            pairs.push((asked::THE_UNITS_FINISHED_FIELD, finished.to_string()));
        }
        pairs
    }

    /// **The report read back from named fields**, wherever they ride — `text`
    /// answers a field's name with its value, so the supervisor reads a
    /// worker's report and the desktop reads a queue reading's item with the
    /// same function.
    ///
    /// **`None` where the moment, its clock, the attempt or the count of
    /// frames is missing or does not read**: a report without them is no
    /// report, and an entry reads without figures (R4) rather than with a
    /// half-read set. A figure that is there and does not read is left out.
    pub fn read_back<'a>(text: impl Fn(&str) -> Option<&'a str>) -> Option<AWorkersReport> {
        use smoozy_framing::requests as asked;
        use smoozy_state::observed::{Clock, MachineClock};
        let number = |name: &str| text(name).and_then(|value| value.parse::<u64>().ok());
        let signed = |name: &str| text(name).and_then(|value| value.parse::<i64>().ok());
        let attempt: u32 = text(asked::THE_ATTEMPT_FIELD)?.parse().ok()?;
        let seconds = number(asked::THE_FIGURES_TAKEN_AT_FIELD)?;
        let clock = match text(asked::THE_FIGURES_CLOCK_FIELD)? {
            THIS_COMPUTER => Clock::ThisComputer,
            name => Clock::AMachine(MachineClock(name.to_string())),
        };
        let pair = |one: &str, other: &str| Some((number(one)?, number(other)?));
        let read = LiveReadings {
            simulation: signed(THE_SIMULATION_FRAME).zip(signed(THE_SIMULATION_END))
                .filter(|(frame, to)| frame <= to),
            frame_in_hand: signed(asked::THE_FRAME_IN_HAND_FIELD),
            view_layer: text(asked::THE_VIEW_LAYER_IN_HAND_FIELD).map(str::to_string),
            samples: pair(asked::THE_SAMPLES_SEEN_FIELD, asked::THE_SAMPLES_PRINTED_TOTAL_FIELD)
                .map(|(seen, of)| SamplePair { seen, of }),
            stopped_short: text(asked::THE_FRAME_STOPPED_SHORT_FIELD) == Some(asked::THE_ANSWER_IS_YES),
            tiles: pair(asked::THE_TILES_DONE_FIELD, asked::THE_TILES_TOTAL_FIELD)
                .map(|(done, of)| TilePair { done, of }),
            blenders_own_estimate_for_the_frame: text(asked::BLENDERS_OWN_ESTIMATE_FIELD)
                .map(str::to_string),
            frames_completed: number(asked::THE_FRAMES_COUNTED_FIELD)?,
        };
        Some(AWorkersReport {
            attempt,
            readings: Reading::taken(
                read,
                SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(seconds),
                clock,
            ),
            newest_frame_saved: signed(asked::THE_NEWEST_FRAME_SAVED_FIELD),
            the_plan_total: number(asked::THE_PLAN_TOTAL_FIELD),
            the_units_finished: number(asked::THE_UNITS_FINISHED_FIELD),
        })
    }
}

/// How a moment on the desktop's own clock is named, where one is ever
/// written — the same words the per-job note's moments use.
const THIS_COMPUTER: &str = "this computer";
const THE_SIMULATION_FRAME: &str = "simulation frame";
const THE_SIMULATION_END: &str = "simulation end";

/// **A rendering job's two sinks, as the queue reading names them** — E8's
/// *"the desktop reads the sink's current bytes over the connection, on
/// request"*, with the path on the machine and the bytes standing in it when
/// the answer was composed, so the desktop never builds a machine path itself.
/// Added 2026-10-03, the Creator's notes on 0.1.8, piece 6a.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheSinksOfARenderingJob {
    pub ordinary: ASink,
    pub complaints: ASink,
}

/// One sink: where it is on the machine, and its length. **The length is
/// absent where the machine could not read it**, which is never a length of
/// nought.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ASink {
    pub path: String,
    pub length: Option<u64>,
}

impl TheSinksOfARenderingJob {
    /// The two sinks as named fields, under the names a reading's own
    /// recorded outcome already gives the same two facts.
    pub fn as_pairs(&self) -> Vec<(&'static str, String)> {
        use smoozy_framing::requests as asked;
        let mut pairs = Vec::new();
        for (sink, path, length) in [
            (&self.ordinary, asked::THE_ORDINARY_STREAM_FIELD, asked::THE_ORDINARY_STREAM_LENGTH_FIELD),
            (
                &self.complaints,
                asked::THE_COMPLAINTS_STREAM_FIELD,
                asked::THE_COMPLAINTS_STREAM_LENGTH_FIELD,
            ),
        ] {
            pairs.push((path, sink.path.clone()));
            if let Some(bytes) = sink.length {
                pairs.push((length, bytes.to_string()));
            }
        }
        pairs
    }

    /// The two sinks read back, or `None` where either path is missing.
    pub fn read_back<'a>(text: impl Fn(&str) -> Option<&'a str>) -> Option<TheSinksOfARenderingJob> {
        use smoozy_framing::requests as asked;
        let sink = |path: &str, length: &str| {
            Some(ASink {
                path: text(path)?.to_string(),
                length: text(length).and_then(|bytes| bytes.parse().ok()),
            })
        };
        Some(TheSinksOfARenderingJob {
            ordinary: sink(asked::THE_ORDINARY_STREAM_FIELD, asked::THE_ORDINARY_STREAM_LENGTH_FIELD)?,
            complaints: sink(
                asked::THE_COMPLAINTS_STREAM_FIELD,
                asked::THE_COMPLAINTS_STREAM_LENGTH_FIELD,
            )?,
        })
    }
}

/// **R4: absence of readings is never an ending.** Where nothing is printed —
/// a route that prints nothing until the frame is saved, or a Workbench
/// stream under an adopted job (R14) — every reading is absent and the entry
/// reads without them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheEntryHasLive {
    /// **What a rendering job's part of the queue reading carries** — R5's
    /// figures, the worker's newest report as the supervisor holds it, and
    /// E8's two sinks; **either may be absent on its own** (a worker that has
    /// not reported yet still has its sinks). Read back by
    /// [`AWorkersReport::read_back`] and [`TheSinksOfARenderingJob::read_back`],
    /// so the desktop spells no name. Carried whole since 2026-10-03, the
    /// Creator's notes on 0.1.8, piece 6b; until then it held the readings
    /// alone and nothing ever filled it.
    Readings {
        figures: Option<AWorkersReport>,
        sinks: Option<TheSinksOfARenderingJob>,
    },
    /// **An ended job's part while its ending is owed to a full store** —
    /// `Rented-server runtime` S14 and `Rented-server runtime` S14a: the outcome the supervisor
    /// classified, marked owed, and `Rented-server runtime` V12's reading,
    /// carried beside the job's *ended* until the landing. **It is the
    /// supervisor's reading of what it holds, never the copy loop's**, and
    /// never evidence that a record landed. Added 2026-10-25, step 12's SC4
    /// follow-up.
    ItsEndingIsOwed(crate::acting::note::TheEndingOwed),
    /// An ordinary state of the entry, never a stale one and never a fault.
    NoReadings,
}

impl WhatTheEntryHasLive {
    /// **A rendering job's part, read back from the queue reading's item** —
    /// `text` answers a field's name with its value. **Nothing where the item
    /// carries neither**, which R4 makes an ordinary entry.
    pub fn read_back<'a>(text: impl Fn(&str) -> Option<&'a str> + Copy) -> WhatTheEntryHasLive {
        match (AWorkersReport::read_back(text), TheSinksOfARenderingJob::read_back(text)) {
            (None, None) => WhatTheEntryHasLive::NoReadings,
            (figures, sinks) => WhatTheEntryHasLive::Readings { figures, sinks },
        }
    }

    /// R5's figures, where the machine carried them.
    pub fn the_figures(&self) -> Option<&AWorkersReport> {
        match self {
            WhatTheEntryHasLive::Readings { figures, .. } => figures.as_ref(),
            WhatTheEntryHasLive::ItsEndingIsOwed(_) | WhatTheEntryHasLive::NoReadings => None,
        }
    }

    /// E8's two sinks, where the machine named them.
    pub fn the_sinks(&self) -> Option<&TheSinksOfARenderingJob> {
        match self {
            WhatTheEntryHasLive::Readings { sinks, .. } => sinks.as_ref(),
            WhatTheEntryHasLive::ItsEndingIsOwed(_) | WhatTheEntryHasLive::NoReadings => None,
        }
    }

    /// **The ending owed to a full store**, where the reading carried one
    /// (`Rented-server runtime` S14a).
    pub fn the_ending_owed(&self) -> Option<&crate::acting::note::TheEndingOwed> {
        match self {
            WhatTheEntryHasLive::ItsEndingIsOwed(owed) => Some(owed),
            _ => None,
        }
    }
}

/// **§17's five observations the render worker takes at its own copy loop
/// and carries — the shapes `Rendering` K13's worker fills.** Each is an
/// observation of bytes in passing, never a mining of the sink afterwards
/// (E10), and each is recorded where the plan says: the live readings and
/// the newest-saved figure on R5's bounded reporting, the other three in the
/// attempt's record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnObservationAtTheCopyLoop {
    /// **R1, R5–R6**: the live readings, reported on the bounded interval.
    TheLiveReadings(AWorkersReport),
    /// **H7**: the ending-step completeness reading, carried with the outcome
    /// — every outcome the worker reports carries one, a stopped render
    /// included.
    TheEndingStepsCompletenessReading(crate::waiting::CompletenessReading),
    /// **P1a**: the frame's save announcement, observed in passing, with the
    /// newest-saved figure reported on the same bounded reporting. A frame
    /// announced is a frame begun; only its save ends the write, and the line
    /// proves the write ended and never that the file is whole.
    TheNewestFrameSaved { frame: i64 },
    /// **E13**: the device line, observed on the version's own stream and
    /// recorded in the attempt record.
    TheDeviceLine(ObservedDeviceLine),
    /// **E13a**: the colour-substitution line, observed on both streams with
    /// no stream asserted, recorded like the device line's.
    TheColourSubstitutionLine(ObservedColourSubstitutionLine),
}

/// **E13's record of the device line**: the line as printed, on which stream
/// the version's route prints it, and the attempt it was observed on.
/// **What it means for the entry is [`crate::ending`]'s, and it turns on
/// the recorded outcome and never on the line's presence** (E12).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservedDeviceLine {
    pub attempt: u32,
    pub the_line_as_printed: String,
    pub on_stream: WhichStream,
}

/// **E13a's record of the colour-substitution line**: which of the three
/// substance-kinds it matched, the line as printed, and the stream it was
/// seen on — **which is recorded, never asserted**, since which stream
/// carries it was never established (§18 item 27).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservedColourSubstitutionLine {
    pub attempt: u32,
    pub kind: crate::ending::ColourSubstitutionKind,
    pub the_line_as_printed: String,
    pub on_stream: WhichStream,
}

/// The two streams the worker copies (`Rented-server runtime` M3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhichStream {
    TheOrdinaryOutput,
    TheComplaints,
}

/// **A1's queue reading, per job, as the acting half supplies it**: the
/// machine's whole working state, period-held, stamped with moment and
/// clock. **It carries no group field (L20) and no carry-on field (N20)**;
/// those are the local record's copies, joined at the draw by job identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AJobAsTheMachineReadsIt {
    pub job: Identity,
    pub state: crate::line::QueueState,
    pub live: WhatTheEntryHasLive,
    /// **A1's *its unreleased hold*, for a held job: why it is held and what
    /// it waits behind** (H1, H6), read off the machine's hold record.
    /// **`None` for a job that is not held, and for every job in a reading
    /// from a program older than step 10c's eleventh-pass fix piece 8b**,
    /// which carried the hold identity alone. Added 2026-09-30.
    pub the_hold: Option<WhatTheHoldSays>,
}

/// **What a held job's hold says, as the queue reading carries it** — H1's
/// *"the entry's state derives from the hold record"* and H6's *"the hold
/// record names the dependency; the entry states it"*.
///
/// **Written for one case and now meeting two.** A1's reading was first
/// written (step 10c piece 7) with the hold identity alone, for a desktop
/// that had just sent the held job and knew why. **It now also meets a
/// desktop opened after the submission**, which knows only what the reading
/// says. The record and the world parted company at the first restart of
/// Smoozy with a held rest standing: the entry read *Waiting on you* and
/// offered neither answer. Step 10c's eleventh-pass fix piece 8b.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheHoldSays {
    /// **The reason, in the placing document's own words** —
    /// `Rented-server runtime` U2 carries it and never interprets it; the
    /// placing document (`Rendering`, for the first-frame check) reads it.
    pub reason: String,
    /// **The job it is held behind**, where the hold names one (H6).
    pub held_behind: Option<Identity>,
}

/// **The readings behind this crate's judgements, behind one trait the
/// acting half implements** — the split `App/Keys/` draws at its
/// `usable::Readings` and every crate since has kept.
///
/// **Every one is a reading of something already held or already running,
/// and none reaches a machine from here**: the queue reading is A1's one
/// request, on demand, after the installation identity confirms; the server
/// row lookup is a read of the local record at the draw (N17); the landing
/// places are `Download`'s composition, consumed whole (N18).
pub trait Readings {
    /// **A1, C6: the period-held queue reading for a server**, or `None`
    /// where Smoozy holds none — a fresh launch knows nothing until it
    /// connects, and a period that ended dropped the record entirely.
    fn the_queue_reading(&self, server: &Identity)
        -> Option<Reading<Vec<AJobAsTheMachineReadsIt>>>;

    /// **N17: whether a server row stands for the entry's copied local
    /// server identifier**, looked up at the moment the entry is drawn. A
    /// dangling identifier reads as nothing.
    fn a_server_row_stands(&self, local_server: &Identity) -> bool;

    /// **N18: the set of landing places for a job's downloaded files**,
    /// composed by `Download` when the control is used and never cached; and
    /// whether any manual entry under the job's output folder published, for
    /// its H7a's two readings of an empty set.
    fn the_landing_places(&self, job: &Identity) -> LandingPlaces;

    /// The desktop's own clock, for the one figure the entry takes from it
    /// (N16) and for a tombstone's deletion moment (X1).
    fn now_on_this_computer(&self) -> SystemTime;
}

/// **N18: what the computer-side shortcut is handed** — a set of landing
/// places, never a folder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LandingPlaces {
    /// In the order each was first written to; a place that no longer exists
    /// kept in the set and stated as one Smoozy cannot open.
    pub places: Vec<ALandingPlace>,
    /// Whether manual entries under the job's output folder published, which
    /// decides which of the two empty-set readings the entry states.
    pub manual_entries_under_the_jobs_output_folder_published: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ALandingPlace {
    pub path: String,
    pub still_exists: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn a_line(frame: i64, samples: Option<(u64, u64)>, tiles: Option<(u64, u64)>) -> AProgressLine {
        AProgressLine {
            frame: Some(frame),
            view_layer: None,
            samples: samples.map(|(seen, of)| SamplePair { seen, of }),
            tiles: tiles.map(|(done, of)| TilePair { done, of }),
            estimate: None,
            the_line_says_finished: false,
        }
    }

    #[test]
    fn the_largest_seen_stands_and_the_trailing_zero_marks_an_untiled_frame_stopped_short() {
        let mut held = LiveReadings::default();
        for seen in [16u64, 64, 200] {
            held = fold_a_line(&held, &a_line(7, Some((seen, 500)), None));
        }
        assert_eq!(held.samples, Some(SamplePair { seen: 200, of: 500 }));
        assert!(!held.stopped_short);
        // The line reading zero of the total just before the frame ends.
        held = fold_a_line(&held, &a_line(7, Some((0, 500)), None));
        assert_eq!(
            held.samples,
            Some(SamplePair { seen: 200, of: 500 }),
            "never the latest"
        );
        assert!(held.stopped_short);
    }

    #[test]
    fn a_tiled_line_yields_the_tiles_pair_as_printed_and_no_stopped_short_mark() {
        let mut held = LiveReadings::default();
        held = fold_a_line(&held, &a_line(1, Some((16, 16)), Some((2, 4))));
        held = fold_a_line(&held, &a_line(1, Some((0, 16)), Some((3, 4))));
        assert_eq!(held.tiles, Some(TilePair { done: 3, of: 4 }));
        assert_eq!(
            held.samples,
            Some(SamplePair { seen: 0, of: 16 }),
            "the tile's own pair"
        );
        assert!(
            !held.stopped_short,
            "up to 5.2 a finished tiled frame ends on a trailing zero; 5.3's ends on `Sample 16/16` (R12)"
        );
    }

    #[test]
    fn a_new_frame_in_hand_starts_the_fold_over() {
        let mut held = LiveReadings::default();
        held = fold_a_line(&held, &a_line(1, Some((400, 500)), None));
        held = fold_a_line(&held, &a_line(2, Some((10, 500)), None));
        assert_eq!(held.frame_in_hand, Some(2));
        assert_eq!(held.samples, Some(SamplePair { seen: 10, of: 500 }));
        assert!(!held.stopped_short);
    }

    /// **Piece 3b: a new view layer on the same frame starts the fold over**,
    /// so the second layer's opening zero is that layer's first reading and
    /// never the stopped-short mark — the lines 5.2.2 printed at the desk,
    /// with the layer from its announcement. **What it bites on**: the fold of
    /// before, which read the second layer as `1024/1024`, stopped short.
    #[test]
    fn a_new_view_layer_on_the_same_frame_starts_the_fold_over_and_its_opening_zero_marks_nothing() {
        let named = |layer: &str| AProgressLine {
            frame: None,
            view_layer: Some(layer.to_string()),
            samples: None,
            tiles: None,
            estimate: None,
            the_line_says_finished: false,
        };
        let mut held = LiveReadings {
            frames_completed: 3,
            ..LiveReadings::default()
        };
        held = fold_a_line(&held, &a_line(1, None, None));
        held = fold_a_line(&held, &named("First"));
        for seen in [0u64, 1, 1024] {
            held = fold_a_line(&held, &a_line(1, Some((seen, 1024)), None));
        }
        assert_eq!(held.samples, Some(SamplePair { seen: 1024, of: 1024 }));
        held = fold_a_line(&held, &named("Second"));
        held = fold_a_line(&held, &a_line(1, Some((0, 1024)), None));
        assert_eq!(held.view_layer.as_deref(), Some("Second"));
        assert_eq!(held.frame_in_hand, Some(1), "the frame is kept");
        assert_eq!(held.frames_completed, 3, "the count of frames is kept");
        assert_eq!(held.samples, Some(SamplePair { seen: 0, of: 1024 }));
        assert!(!held.stopped_short, "the opening zero marks nothing");
        // The same layer named again is no boundary, and the mark still works
        // inside a layer.
        held = fold_a_line(&held, &a_line(1, Some((300, 1024)), None));
        held = fold_a_line(&held, &named("Second"));
        held = fold_a_line(&held, &a_line(1, Some((0, 1024)), None));
        assert_eq!(held.samples, Some(SamplePair { seen: 300, of: 1024 }));
        assert!(held.stopped_short);
        // 4.5's own lines name the layer on every progress line.
        let mut four = LiveReadings::default();
        let mut on = |layer: &str, seen: u64| {
            let mut line = a_line(1, Some((seen, 1024)), None);
            line.view_layer = Some(layer.to_string());
            four = fold_a_line(&four, &line);
        };
        on("First", 1024);
        on("Second", 0);
        assert_eq!(four.samples, Some(SamplePair { seen: 0, of: 1024 }));
        assert!(!four.stopped_short);
    }

    #[test]
    fn the_estimate_is_the_lines_own_and_never_held_over_from_an_earlier_line() {
        let mut held = LiveReadings::default();
        let mut with = a_line(3, Some((8, 64)), None);
        with.estimate = Some("00:00.05".to_string());
        held = fold_a_line(&held, &with);
        assert_eq!(
            held.blenders_own_estimate_for_the_frame.as_deref(),
            Some("00:00.05")
        );
        held = fold_a_line(&held, &a_line(3, Some((64, 64)), None));
        assert_eq!(held.blenders_own_estimate_for_the_frame, None);
    }

    #[test]
    fn a_line_carrying_no_reading_moves_nothing() {
        let held = fold_a_line(
            &LiveReadings::default(),
            &AProgressLine {
                frame: None,
                view_layer: None,
                samples: None,
                tiles: None,
                estimate: None,
                the_line_says_finished: false,
            },
        );
        assert_eq!(held, LiveReadings::default());
    }

    /// A report as the worker sends it, with every figure set.
    fn a_whole_report() -> AWorkersReport {
        use smoozy_state::observed::{Clock, MachineClock};
        AWorkersReport {
            attempt: 2,
            readings: Reading::taken(
                LiveReadings {
                    simulation: None,
                    frame_in_hand: Some(-3),
                    view_layer: Some("Back plate".to_string()),
                    samples: Some(SamplePair { seen: 812, of: 2048 }),
                    stopped_short: true,
                    tiles: Some(TilePair { done: 3, of: 4 }),
                    blenders_own_estimate_for_the_frame: Some("00:12.58".to_string()),
                    frames_completed: 7,
                },
                SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1_790_000_000),
                Clock::AMachine(MachineClock("installation-1".to_string())),
            ),
            newest_frame_saved: Some(-4),
            the_plan_total: Some(4_096_000),
            the_units_finished: Some(1_024_000),
        }
    }

    fn the_pairs_read_by_name<'p>(
        pairs: &'p [(&'static str, String)],
    ) -> impl Fn(&str) -> Option<&'p str> + 'p {
        move |name| pairs.iter().find(|(at, _)| *at == name).map(|(_, value)| value.as_str())
    }

    /// **R5's report reads back as it was sent**, every figure with it — the
    /// one composition both legs ride.
    #[test]
    fn a_report_reads_back_whole_from_its_own_fields() {
        let report = a_whole_report();
        let pairs = report.as_pairs();
        assert_eq!(AWorkersReport::read_back(the_pairs_read_by_name(&pairs)), Some(report));
    }

    /// **R4: a figure the stream never printed is left out, never sent as a
    /// zero**, and reads back as absent.
    #[test]
    fn a_figure_never_printed_is_absent_on_both_sides_and_never_a_zero() {
        let mut report = a_whole_report();
        report.readings = Reading::taken(
            LiveReadings::default(),
            report.readings.taken_at(),
            report.readings.clock().clone(),
        );
        report.newest_frame_saved = None;
        report.the_plan_total = None;
        report.the_units_finished = None;
        let pairs = report.as_pairs();
        let names: Vec<&str> = pairs.iter().map(|(name, _)| *name).collect();
        use smoozy_framing::requests as asked;
        assert_eq!(
            names,
            [
                asked::THE_ATTEMPT_FIELD,
                asked::THE_FIGURES_TAKEN_AT_FIELD,
                asked::THE_FIGURES_CLOCK_FIELD,
                asked::THE_FRAMES_COUNTED_FIELD,
            ]
        );
        assert_eq!(AWorkersReport::read_back(the_pairs_read_by_name(&pairs)), Some(report));
    }

    /// **A report missing its moment is no report**, and an entry then reads
    /// without figures rather than with figures from nowhere.
    #[test]
    fn a_report_without_its_moment_does_not_read_back() {
        use smoozy_framing::requests as asked;
        let pairs: Vec<(&'static str, String)> = a_whole_report()
            .as_pairs()
            .into_iter()
            .filter(|(name, _)| *name != asked::THE_FIGURES_TAKEN_AT_FIELD)
            .collect();
        assert_eq!(AWorkersReport::read_back(the_pairs_read_by_name(&pairs)), None);
    }

    /// **E8's two sinks read back as named**, and a length the machine could
    /// not read stays absent rather than nought.
    #[test]
    fn a_rendering_jobs_sinks_read_back_with_their_lengths() {
        let sinks = TheSinksOfARenderingJob {
            ordinary: ASink {
                path: "/root/smoozy/streams/job/1/ordinary".to_string(),
                length: Some(38_184),
            },
            complaints: ASink {
                path: "/root/smoozy/streams/job/1/complaints".to_string(),
                length: None,
            },
        };
        let pairs = sinks.as_pairs();
        assert_eq!(
            TheSinksOfARenderingJob::read_back(the_pairs_read_by_name(&pairs)),
            Some(sinks)
        );
    }
}

#[cfg(test)]
mod wait_count_tests {
    use super::*;
    #[test]
    fn wait_count_survives_the_same_report_the_queue_reads() {
        let report = AWorkersReport {
            attempt: 1,
            readings: Reading::taken(LiveReadings::default(), SystemTime::UNIX_EPOCH,
                smoozy_state::observed::Clock::ThisComputer),
            newest_frame_saved: None, the_plan_total: None, the_units_finished: None,
        };
        let mut sent = report.as_pairs();
        sent.extend([("simulation frame", "-2".into()), ("simulation end", "430".into())]);
        let read = AWorkersReport::read_back(|name| sent.iter().find(|(key, _)| *key == name).map(|(_, value)| value.as_str())).unwrap();
        assert!(read.as_pairs().contains(&("simulation frame", "-2".into())));
        assert!(read.as_pairs().contains(&("simulation end", "430".into())));
        assert_eq!(read.readings.value().frames_completed, 0);
        assert_eq!(read.readings.value().frame_in_hand, None);
    }
}
