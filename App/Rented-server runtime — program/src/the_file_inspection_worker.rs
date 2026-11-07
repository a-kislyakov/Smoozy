//! **The file-inspection worker — `The shape`'s third class, for the kind
//! `Rented-server runtime` W1 calls *"one reading of a `.blend`"*.** Added
//! 2026-09-29, step 10c's dependency wave piece 2.
//!
//! # What it is
//!
//! **`Render Settings` Q4: a pre-load is *"one file-inspection operation driven
//! from the desktop through the supervisor's command path"*, admitted *"at the
//! machine's one admission point under"* that document's §10 — and
//! `Dependency Upload & Remap` G1: the check's reading is *"driven as a
//! file-inspection worker through the supervisor"*.** Both readings are this
//! kind (`Render Settings` W5, the runner's ruling of 2026-08-22), and both
//! arrive as one request, `smoozy_framing::requests`' *a reading of a
//! `.blend`*, whose own documentation says why one name serves both.
//!
//! **The supervisor's side of the start is §14's one unit and lives beside the
//! dispatch in `main.rs`**, the way the render and setup kinds' do; this file is
//! the child, what it is told and what it reports, and the per-kind field the
//! supervisor writes.
//!
//! # It starts Blender by the build's own path, never through a link
//!
//! **Until this piece the check's reading was a bare command through a link to
//! the Blender program** — the route `Dependency Upload & Remap` G1 and `Render
//! Settings` Q4 both rule out, and the one that crashed Blender at the desk:
//! started through a link, Blender 5.2.2 on the Creator's Mac could not find its
//! own scripts and ended at 139 before it read anything. **Here the build's path
//! is a protocol field** (`Security and licence` §10) and is handed to the kernel
//! as the program to run, the way `Rendering — machine`'s own reading does.
//!
//! - **The file's reading** is `Job files and outputs` R1's one invocation, with
//!   `Job files and outputs` R3's one script and **`Job files and outputs` R7's
//!   launch: the machine's own
//!   settings and no factory-startup switch** — `Rendering — machine`'s
//!   `the_words_of_a_reading`, and `Job files and outputs — machine`'s
//!   `the_reading_script` composed with **no job frames and no job scene**, so
//!   the script walks the file's own scene over its own span.
//! - **The check's reading** is `Dependency Upload & Remap`'s own line and
//!   script, with factory startup, from `smoozy_dependencies_machine::check_script`.
//! - **Both carry `Rendering` L8b's script before the file**, added
//!   2026-10-02 for `Job files and outputs` R18: its handler points every
//!   Replay fluid cache and every disk cache at a throwaway folder beside the
//!   reading's own script, removed when the run ends, so neither reading
//!   writes beside the user's `.blend`.
//!
//! # Two more operations of the kind — the 38's piece 7, 2026-10-04
//!
//! **`Dependency Upload & Remap` O2 settled the housing**: the contents reading
//! and the fixing run as file-inspection workers, each admitted under its own
//! row of that document's §9 table.
//!
//! - **The contents reading** is `Dependency Upload & Remap` Q3's digest of a
//!   file on the machine, computed here by Smoozy's own code over the byte range
//!   asked for. **No Blender is started**: its group is the worker alone (
//!   `Dependency Upload & Remap` O1), and its ending carries the digest rather than two streams.
//! - **The fixing** is that document's M14: the worker writes the fixing's
//!   script (`smoozy_dependencies_machine::fixing_script`) into the operation's
//!   folder and starts Blender by the build's own path with M14's words, the
//!   copy saved to the staging path the request names. **It publishes nothing**:
//!   the desktop reads the fixing's own report back and publishes by one rename.
//!
//! # `Render Settings` W2 and `Dependency Upload & Remap` O1, kept
//!
//! 1. **The interrupt goes to the Blender it recorded.** The worker reads the
//!    pair of the Blender it started off the kernel (`Rented-server runtime` X1)
//!    and tells the supervisor, which writes it into this kind's own field; a
//!    stop — `Server protocol and transfers` C21's request, naming the
//!    operation — is issued by the supervisor to that pair and to nothing else,
//!    through `Rendering — machine`'s own sequence. **Where the kernel publishes
//!    no record**, as on the desk's Mac, nothing is recorded and no stop can
//!    reach the Blender; the operation runs to its own end. The answer's *ended
//!    by Smoozy* is read off the record — its desired state, and whether the
//!    interrupt went out — rather than assumed.
//! 2. **It may be ended at any moment**: nothing durable is half-written by it.
//! 3. **The only cleanup owed is the outcome record**, which the supervisor
//!    writes from this worker's report (`Rented-server runtime` S1). The
//!    captured streams are retained *"under the ordinary rules"*.
//! 4. **It outlives the one Blender it starts**: it holds both streams to their
//!    end, reaps the process, and only then reports.
//!
//! # Where the document goes — the runner's ruling of 2026-09-28
//!
//! **Into the two sinks the supervisor made (`Rented-server runtime` M1), and
//! never into the answer.** One frame holds at most
//! `smoozy_framing::framing::THE_LARGEST_FRAME`, and a long animation's reading
//! passes it. The report says how many bytes each sink retained, the exit
//! status, and whether the reading's own start mark stands in either; the
//! supervisor writes that into the operation record; and the desktop, asking
//! the same request again, is told where the streams sit and reads them back
//! over the file session.
//!
//! **Markers used in this file that belong to another document are written
//! with their document at every site**, as the rest of this program does.

use std::ffi::OsStr;
use std::io::Read;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::process::ExitStatusExt;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde::{Deserialize, Serialize};
use smoozy_framing::framing::{Answered, Fields, Request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS};
use smoozy_framing::requests as asked;
use smoozy_job_files_machine::reading::{RecordedLocation, TheInvocation, TheVersionInHand};
use smoozy_runtime::acting::capture::TheCopyIntoTheSinks;
use smoozy_runtime::acting::endpoint::TheEndpointAsACaller;
use smoozy_runtime::acting::workers::{
    IT_WAS_ACCEPTED, IT_WAS_REFUSED_AS_STALE, THE_RUN_THE_WORKER_WAS_STARTED_UNDER,
    THE_WORKERS_REPORT_REQUEST, WHETHER_THE_REPORT_WAS_ACCEPTED,
};
use smoozy_runtime::capture::{CaptureCeiling, CaptureRecord, CopiesAStreamIntoItsSink, Stream};
use smoozy_runtime::execution_group::{RecordedPair, ValidatesTheRecordedPair};
use smoozy_state::identities::{Identity, IdentityKind};

use crate::the_render_worker::{
    THE_AVAILABLE_BYTES_FIELD, THE_COMPLAINTS_SINK_FIELD, THE_OPERATION_FIELD,
    THE_ORDINARY_SINK_FIELD, THE_WORKERS_OWN_PROCESS_FIELD, WHY_THERE_IS_NO_WORK_FIELD,
};

// ---------------------------------------------------------------------------
// The words the worker and the supervisor speak to each other
// ---------------------------------------------------------------------------

/// **The worker's first question: what was this file-inspection worker started
/// for.** Neither end is the desktop, so the name is this program's own — the
/// same standing the render worker's first question has. **A pure reading.**
pub const THE_FILE_INSPECTION_WORKERS_OWN_WORK_REQUEST: &str =
    "what the file-inspection worker was started for";

/// **Where the worker writes the reading's script**, inside the operation's own
/// folder beside its two sinks. Smoozy's own path, carrying no value of the
/// user's.
pub const THE_SCRIPT_FIELD: &str = "script";

/// **The reading's ending, on `Rented-server runtime` W12's report**: one of
/// the two words below. Its presence is what makes a report a reading's.
pub const THE_READINGS_ENDING_FIELD: &str = "the reading's ending";
/// **The Blender ran and was reaped.** Whether what it printed is a reading is
/// the desktop's judgement (`Render Settings` Q7, `Dependency Upload & Remap`
/// G3) and never this program's.
pub const THE_READING_RAN_TO_ITS_END: &str = "the reading ran to its end";
/// **No Blender was started** — the script could not be composed or written, or
/// the build could not be run — and the machine's own words say which.
pub const THE_READING_WAS_NOT_STARTED: &str = "the reading was not started";
/// **The contents reading ended**, digested or not — no Blender is ever started
/// for it, so neither word above fits it.
pub const THE_CONTENTS_WERE_READ: &str = "the contents reading ran to its end";

/// **The contents reading and the fixing, on the worker's first answer** —
/// this program's own words for the two operations, since neither arrives as a
/// reading of a `.blend`.
pub const THE_CONTENTS_READINGS_WORD: &str = "the contents reading";
pub const THE_FIXINGS_WORD: &str = "the fixing";

/// **The worker tells its supervisor the pair of the Blender it started** —
/// `Rented-server runtime` X1, the pair `Dependency Upload & Remap` O1's
/// interrupt is aimed by. Neither end is the desktop, so the name is this
/// program's own, the way the render worker's own `the render process was
/// started` is. **Carries** the operation and [`THE_BLENDERS_PAIR_FIELD`].
pub const THE_BLENDER_OF_A_FILE_INSPECTION_WAS_STARTED_REQUEST: &str =
    "the Blender of a file inspection was started";
/// The pair, as `Rented-server runtime` writes one.
pub const THE_BLENDERS_PAIR_FIELD: &str = "the Blender's pair";

/// **The signal that ended the run, where one did**, in decimal. `Rented-server
/// runtime` M22's exit status is then absent, because none was reaped.
pub const THE_SIGNAL_FIELD: &str = "ended by signal";

/// **How long the worker waits for a supervisor to be holding the endpoint at
/// all**, when it asks what it was started for — the render worker's figure and
/// reason, taken rather than chosen again.
pub const HOW_LONG_THE_WORKER_LOOKS_FOR_ITS_SUPERVISOR: Duration =
    crate::the_render_worker::HOW_LONG_THE_WORKER_LOOKS_FOR_ITS_SUPERVISOR;

// ---------------------------------------------------------------------------
// Which reading, and what the worker is told
// ---------------------------------------------------------------------------

/// **Which operation of the kind** — the two readings
/// `smoozy_framing::requests` closes at two, and since the 38's piece 7 the
/// contents reading and the fixing beside them (`Dependency Upload & Remap`
/// O2). **The name stayed**, because the record's field is written under it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WhichReading {
    /// `Render Settings` Q4's read, `Job files and outputs` R1's document.
    TheFilesReading,
    /// `Dependency Upload & Remap` G1's reading.
    TheChecksReading,
    /// `Dependency Upload & Remap` Q3's digest of a machine file. No Blender.
    TheContentsReading,
    /// `Dependency Upload & Remap` M14's production of one fixed copy.
    TheFixing,
}

impl WhichReading {
    pub fn read_back(word: &str) -> Option<WhichReading> {
        match word {
            asked::THE_FILES_READING => Some(WhichReading::TheFilesReading),
            asked::THE_CHECKS_READING => Some(WhichReading::TheChecksReading),
            THE_CONTENTS_READINGS_WORD => Some(WhichReading::TheContentsReading),
            THE_FIXINGS_WORD => Some(WhichReading::TheFixing),
            _ => None,
        }
    }

    pub fn as_asked(self) -> &'static str {
        match self {
            WhichReading::TheFilesReading => asked::THE_FILES_READING,
            WhichReading::TheChecksReading => asked::THE_CHECKS_READING,
            WhichReading::TheContentsReading => THE_CONTENTS_READINGS_WORD,
            WhichReading::TheFixing => THE_FIXINGS_WORD,
        }
    }

    /// **The reading's own start mark** — `Render Settings` Q8's *"Smoozy's own
    /// frame"* — which differs between the two, deliberately
    /// (`Dependency Upload & Remap` G1: *"not the same act"*).
    ///
    /// **The file's reading's is the mark its script writes as its very first
    /// act** (`Render Settings` Q8, added 2026-10-02 for the Local View crash),
    /// never the document's opening mark, which is written only at the script's
    /// end — so Blender crashing partway through the script is *the read ran
    /// and did not finish*, not *this Blender could not open this file*. This
    /// program composes that script itself, so the mark is always in it.
    pub fn its_start_mark(self) -> &'static [u8] {
        match self {
            WhichReading::TheFilesReading => smoozy_job_files_machine::reading::THE_STARTED_MARK,
            WhichReading::TheChecksReading => {
                smoozy_dependencies_machine::check_script::THE_OPENING_SENTINEL
            }
            WhichReading::TheFixing => {
                smoozy_dependencies_machine::fixing_script::THE_FIXINGS_START_MARK.as_bytes()
            }
            // **No Blender, so no script and no mark.**
            WhichReading::TheContentsReading => b"",
        }
    }

    /// **Whether a captured stream shows this reading started.** The file's
    /// reading reads it the way the desktop does — its started mark, or the
    /// document's opening mark, which no script writes before it has started —
    /// so the two sides cannot judge one stream differently.
    pub fn started_in(self, held: &[u8]) -> bool {
        match self {
            WhichReading::TheFilesReading => {
                smoozy_job_files_machine::reading::the_reading_started_in(held)
            }
            WhichReading::TheChecksReading | WhichReading::TheFixing => {
                let mark = self.its_start_mark();
                held.windows(mark.len()).any(|window| window == mark)
            }
            WhichReading::TheContentsReading => false,
        }
    }
}

/// **The version the file's reading composes its script for** — `Job files and
/// outputs` R2 — read from `smoozy_framing::requests`' three-part version text.
/// **Only its first two parts are read**, which is all `TheVersionInHand`
/// carries.
pub fn the_version_in(text: &str) -> Option<TheVersionInHand> {
    let mut parts = text.split('.');
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next()?.parse().ok()?;
    Some(TheVersionInHand::of(major, minor))
}

/// **What the supervisor says this worker was started for.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheReadingsWork {
    pub operation: Identity,
    /// `Rented-server runtime` S2's run identifier, which the report carries
    /// (W12).
    pub run: Identity,
    pub which: WhichReading,
    /// The build's path, as the bytes it crossed as.
    pub build: Vec<u8>,
    /// The `.blend`'s path, as the bytes it crossed as.
    pub blend: Vec<u8>,
    /// The version text as the desktop sent it, where it sent one.
    pub version: Option<String>,
    pub script: String,
    /// `Rented-server runtime` M1's two sinks and M4a's ceiling.
    pub capture: CaptureRecord,
    /// The fixing's staging path; pairs also carry `Render Settings` Q4's
    /// library locations for the file's reading.
    pub staging: Option<Vec<u8>>,
    pub pairs: Option<Vec<u8>>,
    /// **The contents reading's range**: where it starts, and how many bytes,
    /// or to the end of the file.
    pub range: Option<(u64, Option<u64>)>,
}

/// **`Rented-server runtime` W4: the file-inspection kind's own field of the
/// operation record**, which is this kind's document's to shape.
///
/// **It names both sinks before the worker starts**, which is `Rented-server
/// runtime` M1's one requirement of it and what §14's unit checks. **The
/// ending is written once, from the worker's report**, with the record's
/// outcome in the same write.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TheFileInspectionKindsOwnField {
    pub which: WhichReading,
    pub build: Vec<u8>,
    pub blend: Vec<u8>,
    pub version: Option<String>,
    pub ordinary_sink: String,
    pub complaints_sink: String,
    pub script: String,
    pub ending: Option<TheReadingsEnding>,
    /// **The fixing's staging path and pairs, and the contents reading's
    /// range** — the 38's piece 7. Absent from every record written before it,
    /// which reads back as none.
    #[serde(default)]
    pub staging: Option<Vec<u8>>,
    #[serde(default)]
    pub pairs: Option<Vec<u8>>,
    #[serde(default)]
    pub range: Option<(u64, Option<u64>)>,
    /// **The pair of the Blender the worker started**, as `Rented-server
    /// runtime` writes one — what `Dependency Upload & Remap` O1's interrupt is
    /// aimed by. Written once, when the worker tells it.
    #[serde(default)]
    pub blender: Option<String>,
    /// **Every stop taken for this operation**, by its command identity, each
    /// closed when the operation's ending is written.
    #[serde(default)]
    pub stops: Vec<String>,
    /// **Whether the interrupt went out**, written before it is sent — the
    /// render's own order, so a supervisor that ends between the two never
    /// sends a second.
    #[serde(default)]
    pub the_interrupt_went_out: bool,
}

/// **What one reading came to, as the worker reports it and the record keeps
/// it.**
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TheReadingsEnding {
    /// Whether a Blender was started at all.
    pub a_blender_ran: bool,
    /// **The exit status, verbatim** (`Rented-server runtime` M22); absent where
    /// none was reaped.
    pub exit_status: Option<i32>,
    /// The signal that ended the run, where one did.
    pub signal: Option<i32>,
    /// **How many bytes each sink retained** — what the desktop reads back to.
    pub ordinary_retained: u64,
    pub complaints_retained: u64,
    /// **Bytes the ceiling or a refused write cost**, which make a stream a
    /// prefix; the desktop's own judgement then finds no closing mark.
    pub discarded: u64,
    pub start_mark_seen: bool,
    /// **Written by the supervisor, never reported**: whether the record's
    /// desired state said *should be stopped* and the run ended by a signal —
    /// `Render Settings` Q8's *a read Smoozy ended itself*.
    pub ended_by_smoozy: bool,
    /// The machine's own words, where a reading was not started or lost bytes.
    pub words: Option<String>,
    /// **What the contents reading came to** — nothing for the other three.
    #[serde(default)]
    pub digest: Option<TheDigest>,
}

/// **What one contents reading came to** (`Dependency Upload & Remap` Q3).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TheDigest {
    /// **The range read through and digested**: the sixty-four hexadecimal
    /// digits of `Server protocol and transfers` S23's function, and how many
    /// bytes they cover.
    Digested { digest: String, bytes: u64 },
    /// **The system said no such file**, and only that.
    NothingThere,
    /// **Anything else**, with the machine's words beside it — *a digest that
    /// did not complete is nothing* (`Dependency Upload & Remap` §9).
    NotDigested,
}

impl TheFileInspectionKindsOwnField {
    pub fn as_written(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("the field renders")
    }

    pub fn read_back(bytes: &[u8]) -> Option<TheFileInspectionKindsOwnField> {
        serde_json::from_slice(bytes).ok()
    }

    /// **`Server protocol and transfers` C8's answer for a record of this kind,
    /// and the fields the runner's ruling of 2026-09-28 names** — where the
    /// streams sit, how long each is, the exit status, whether the start mark
    /// was seen and whether Smoozy ended the run. **Still in hand** while the
    /// record carries no ending.
    pub fn as_it_stands(&self) -> Fields {
        let Some(ending) = &self.ending else {
            return Fields::none().and(asked::WHAT_BECAME_OF_IT_FIELD, asked::IT_IS_STILL_IN_HAND);
        };
        let yes_or_no = |it: bool| {
            if it {
                asked::THE_ANSWER_IS_YES
            } else {
                asked::THE_ANSWER_IS_NO
            }
        };
        let mut fields = Fields::none()
            .and(
                asked::WHAT_BECAME_OF_IT_FIELD,
                asked::IT_HAS_A_RECORDED_OUTCOME,
            )
            .and(asked::THE_WHICH_READING_FIELD, self.which.as_asked())
            .and(
                asked::THE_START_MARK_SEEN_FIELD,
                yes_or_no(ending.start_mark_seen),
            )
            .and(
                asked::THE_RUN_WAS_ENDED_BY_SMOOZY_FIELD,
                yes_or_no(ending.ended_by_smoozy),
            );
        // **A reading whose Blender never ran names no stream**, so a reader
        // cannot mistake two empty sinks for a run that printed nothing.
        if ending.a_blender_ran {
            fields = fields
                .and(
                    asked::THE_ORDINARY_STREAM_FIELD,
                    self.ordinary_sink.as_str(),
                )
                .and(
                    asked::THE_ORDINARY_STREAM_LENGTH_FIELD,
                    ending.ordinary_retained.to_string(),
                )
                .and(
                    asked::THE_COMPLAINTS_STREAM_FIELD,
                    self.complaints_sink.as_str(),
                )
                .and(
                    asked::THE_COMPLAINTS_STREAM_LENGTH_FIELD,
                    ending.complaints_retained.to_string(),
                );
        }
        if let Some(status) = ending.exit_status {
            fields = fields.and(asked::THE_EXIT_STATUS_FIELD, status.to_string());
        }
        if let Some(digest) = &ending.digest {
            fields = the_digest_as_fields(digest, fields);
        }
        if let Some(words) = &ending.words {
            fields = fields.and(asked::THE_MACHINES_OWN_WORDS_FIELD, words.as_str());
        }
        fields
    }
}

/// **The contents reading's answer** — the same three fields on the report and
/// on the desktop's answer, so the two cannot say it differently.
pub fn the_digest_as_fields(digest: &TheDigest, fields: Fields) -> Fields {
    match digest {
        TheDigest::Digested { digest, bytes } => fields
            .and(asked::THE_DIGESTS_OUTCOME_FIELD, asked::IT_WAS_DIGESTED)
            .and(asked::THE_DIGEST_FIELD, digest.as_str())
            .and(asked::THE_BYTES_DIGESTED_FIELD, bytes.to_string()),
        TheDigest::NothingThere => fields.and(asked::THE_DIGESTS_OUTCOME_FIELD, asked::NOTHING_IS_THERE),
        TheDigest::NotDigested => fields.and(asked::THE_DIGESTS_OUTCOME_FIELD, asked::IT_WAS_NOT_DIGESTED),
    }
}

/// The contents reading's answer, read back.
pub fn the_digest_in(fields: &Fields) -> Option<TheDigest> {
    match fields.text(asked::THE_DIGESTS_OUTCOME_FIELD)? {
        asked::IT_WAS_DIGESTED => Some(TheDigest::Digested {
            digest: fields.text(asked::THE_DIGEST_FIELD)?.to_string(),
            bytes: fields.text(asked::THE_BYTES_DIGESTED_FIELD)?.parse().ok()?,
        }),
        asked::NOTHING_IS_THERE => Some(TheDigest::NothingThere),
        asked::IT_WAS_NOT_DIGESTED => Some(TheDigest::NotDigested),
        _ => None,
    }
}

/// **The work, as the supervisor answers the worker's first question.**
pub fn the_work_as_fields(work: &TheReadingsWork, available_bytes: u64) -> Fields {
    let mut fields = Fields::none()
        .and(THE_OPERATION_FIELD, work.operation.as_text())
        .and(THE_RUN_THE_WORKER_WAS_STARTED_UNDER, work.run.as_text())
        .and(asked::THE_WHICH_READING_FIELD, work.which.as_asked())
        .and(asked::THE_BUILD_FIELD, work.build.clone())
        .and(asked::THE_BLEND_FIELD, work.blend.clone())
        .and(THE_SCRIPT_FIELD, work.script.as_str())
        .and(THE_ORDINARY_SINK_FIELD, work.capture.ordinary_sink.as_str())
        .and(
            THE_COMPLAINTS_SINK_FIELD,
            work.capture.complaints_sink.as_str(),
        )
        .and(THE_AVAILABLE_BYTES_FIELD, available_bytes.to_string());
    if let Some(version) = &work.version {
        fields = fields.and(asked::THE_BLENDER_VERSION_FIELD, version.as_str());
    }
    if let Some(staging) = &work.staging {
        fields = fields.and(asked::THE_FIXINGS_STAGING_FIELD, staging.clone());
    }
    if let Some(pairs) = &work.pairs {
        fields = fields.and(if work.which == WhichReading::TheFilesReading {
            asked::THE_READINGS_LIBRARY_PAIRS_FIELD
        } else { asked::THE_FIXINGS_PAIRS_FIELD }, pairs.clone());
    }
    if let Some((from, length)) = work.range {
        fields = fields.and(asked::THE_RANGE_STARTS_AT_FIELD, from.to_string());
        if let Some(length) = length {
            fields = fields.and(asked::THE_RANGE_LENGTH_FIELD, length.to_string());
        }
    }
    fields
}

/// The work, read back by the worker.
pub fn the_work_in(answered: &Answered) -> Result<TheReadingsWork, String> {
    let fields = &answered.fields;
    if let Some(why) = fields.text(WHY_THERE_IS_NO_WORK_FIELD) {
        return Err(format!(
            "smoozy-server: the file-inspection worker has no work: {why}"
        ));
    }
    let text = |name: &str| {
        fields
            .text(name)
            .ok_or_else(|| format!("smoozy-server: the file-inspection worker was not told {name}"))
    };
    let bytes = |name: &str| {
        fields
            .value(name)
            .map(<[u8]>::to_vec)
            .ok_or_else(|| format!("smoozy-server: the file-inspection worker was not told {name}"))
    };
    let operation = Identity::read_back(IdentityKind::Operation, text(THE_OPERATION_FIELD)?)
        .ok_or("smoozy-server: the operation identity did not read back")?;
    let run = Identity::read_back(
        IdentityKind::Run,
        text(THE_RUN_THE_WORKER_WAS_STARTED_UNDER)?,
    )
    .ok_or("smoozy-server: the run identifier did not read back")?;
    let which = WhichReading::read_back(text(asked::THE_WHICH_READING_FIELD)?)
        .ok_or("smoozy-server: the reading asked for is neither of the two")?;
    let available: u64 = text(THE_AVAILABLE_BYTES_FIELD)?
        .parse()
        .map_err(|_| "smoozy-server: the available figure did not read back".to_string())?;
    Ok(TheReadingsWork {
        capture: CaptureRecord {
            job: operation.clone(),
            attempt: 1,
            ordinary_sink: text(THE_ORDINARY_SINK_FIELD)?.to_string(),
            complaints_sink: text(THE_COMPLAINTS_SINK_FIELD)?.to_string(),
            ceiling: CaptureCeiling::from(available),
        },
        operation,
        run,
        which,
        build: bytes(asked::THE_BUILD_FIELD)?,
        blend: bytes(asked::THE_BLEND_FIELD)?,
        version: fields
            .text(asked::THE_BLENDER_VERSION_FIELD)
            .map(str::to_string),
        script: text(THE_SCRIPT_FIELD)?.to_string(),
        staging: fields.value(asked::THE_FIXINGS_STAGING_FIELD).map(<[u8]>::to_vec),
        pairs: fields.value(if which == WhichReading::TheFilesReading {
            asked::THE_READINGS_LIBRARY_PAIRS_FIELD
        } else { asked::THE_FIXINGS_PAIRS_FIELD }).map(<[u8]>::to_vec),
        range: the_range_in(fields),
    })
}

/// **The contents reading's range, as a request or the work carries it** —
/// nothing where no start is named, and nothing where a number does not read.
pub fn the_range_in(fields: &Fields) -> Option<(u64, Option<u64>)> {
    let from = fields.text(asked::THE_RANGE_STARTS_AT_FIELD)?.parse().ok()?;
    let length = match fields.text(asked::THE_RANGE_LENGTH_FIELD) {
        Some(text) => Some(text.parse().ok()?),
        None => None,
    };
    Some((from, length))
}

/// **The ending, on `Rented-server runtime` W12's report.**
pub fn the_ending_as_fields(ending: &TheReadingsEnding, fields: Fields) -> Fields {
    let mut fields = fields
        .and(
            THE_READINGS_ENDING_FIELD,
            if ending.digest.is_some() {
                THE_CONTENTS_WERE_READ
            } else if ending.a_blender_ran {
                THE_READING_RAN_TO_ITS_END
            } else {
                THE_READING_WAS_NOT_STARTED
            },
        )
        .and(
            asked::THE_ORDINARY_STREAM_LENGTH_FIELD,
            ending.ordinary_retained.to_string(),
        )
        .and(
            asked::THE_COMPLAINTS_STREAM_LENGTH_FIELD,
            ending.complaints_retained.to_string(),
        )
        .and(
            asked::THE_START_MARK_SEEN_FIELD,
            if ending.start_mark_seen {
                asked::THE_ANSWER_IS_YES
            } else {
                asked::THE_ANSWER_IS_NO
            },
        );
    if let Some(status) = ending.exit_status {
        fields = fields.and(asked::THE_EXIT_STATUS_FIELD, status.to_string());
    }
    if let Some(signal) = ending.signal {
        fields = fields.and(THE_SIGNAL_FIELD, signal.to_string());
    }
    if let Some(digest) = &ending.digest {
        fields = the_digest_as_fields(digest, fields);
    }
    if let Some(words) = &ending.words {
        fields = fields.and(asked::THE_MACHINES_OWN_WORDS_FIELD, words.as_str());
    }
    fields
}

/// **The ending a report carries, where it is a reading's**, read back by the
/// supervisor. `None` where the report carries no reading's ending.
pub fn the_ending_in(fields: &Fields) -> Option<TheReadingsEnding> {
    let (a_blender_ran, digest) = match fields.text(THE_READINGS_ENDING_FIELD)? {
        THE_READING_RAN_TO_ITS_END => (true, None),
        THE_READING_WAS_NOT_STARTED => (false, None),
        THE_CONTENTS_WERE_READ => (false, Some(the_digest_in(fields).unwrap_or(TheDigest::NotDigested))),
        _ => return None,
    };
    let status = |name: &str| fields.text(name).and_then(|text| text.parse::<i32>().ok());
    let length = |name: &str| fields.text(name).and_then(|text| text.parse::<u64>().ok());
    Some(TheReadingsEnding {
        a_blender_ran,
        exit_status: status(asked::THE_EXIT_STATUS_FIELD),
        signal: status(THE_SIGNAL_FIELD),
        ordinary_retained: length(asked::THE_ORDINARY_STREAM_LENGTH_FIELD).unwrap_or(0),
        complaints_retained: length(asked::THE_COMPLAINTS_STREAM_LENGTH_FIELD).unwrap_or(0),
        discarded: 0,
        start_mark_seen: fields.text(asked::THE_START_MARK_SEEN_FIELD)
            == Some(asked::THE_ANSWER_IS_YES),
        ended_by_smoozy: false,
        words: fields
            .text(asked::THE_MACHINES_OWN_WORDS_FIELD)
            .map(str::to_string),
        digest,
    })
}

// ---------------------------------------------------------------------------
// The worker, whole
// ---------------------------------------------------------------------------

/// **The file-inspection worker's own run**: ask, read, report.
///
/// **A worker that gets no answer does nothing**, and says so naming both
/// documents `Rented-server runtime` W1 gives this kind to; its operation
/// record is reconciliation's.
pub fn the_file_inspection_worker(recorded_endpoint_path: &str) -> Result<(), String> {
    let supervisor = TheEndpointAsACaller::at(
        recorded_endpoint_path,
        crate::the_render_worker::HOW_LONG_ONE_EXCHANGE_MAY_TAKE,
    );
    let asked_for_work = Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        a_fresh_identity()?,
        THE_FILE_INSPECTION_WORKERS_OWN_WORK_REQUEST,
    )
    .carrying(Fields::none().and(
        THE_WORKERS_OWN_PROCESS_FIELD,
        std::process::id().to_string(),
    ));
    let answered = supervisor
        .ask_waiting_for_a_supervisor(
            &asked_for_work,
            HOW_LONG_THE_WORKER_LOOKS_FOR_ITS_SUPERVISOR,
        )
        .map_err(|why| format!("smoozy-server: the file-inspection worker could not ask: {why}"))?
        .ok_or_else(|| {
            "smoozy-server: the file-inspection worker (`Render Settings` and `Dependency Upload \
             & Remap`'s kind) asked what it was started for and no supervisor answered, so it \
             does nothing; its operation record is reconciliation's"
                .to_string()
        })?;
    let work = the_work_in(&answered)?;
    let ending = one_reading_telling(&work, &|pair| {
        if let Err(why) = the_blender_is_told(&supervisor, &work, pair) {
            eprintln!(
                "smoozy-server: the file-inspection worker's Blender was not recorded, so no stop \
                 can reach it: {why}"
            );
        }
    });
    report_the_ending(&supervisor, &work, &ending)
}

/// **`Rented-server runtime` X1 for this kind: the Blender's pair, told to the
/// supervisor**, which writes it into the kind's own field.
fn the_blender_is_told(
    supervisor: &TheEndpointAsACaller,
    work: &TheReadingsWork,
    pair: RecordedPair,
) -> Result<(), String> {
    let request = Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        a_fresh_identity()?,
        THE_BLENDER_OF_A_FILE_INSPECTION_WAS_STARTED_REQUEST,
    )
    .carrying(
        Fields::none()
            .and(THE_OPERATION_FIELD, work.operation.as_text())
            .and(THE_BLENDERS_PAIR_FIELD, pair.as_this_document_records_it()),
    );
    match supervisor.ask(&request)? {
        Some(answered)
            if answered.fields.text(crate::the_render_worker::WHETHER_IT_WAS_RECORDED_FIELD)
                == Some(crate::the_render_worker::IT_WAS_RECORDED) =>
        {
            Ok(())
        }
        Some(answered) => Err(answered
            .fields
            .text(asked::THE_MACHINES_OWN_WORDS_FIELD)
            .unwrap_or("the supervisor did not record it")
            .to_string()),
        None => Err("the supervisor did not answer".to_string()),
    }
}

fn a_fresh_identity() -> Result<Identity, String> {
    Identity::issue(IdentityKind::Operation)
        .map_err(|_| "smoozy-server: the system would not supply random bytes".to_string())
}

/// **`Rented-server runtime` W12: re-attempted until it is accepted**, at the
/// render worker's pace. A supervisor that refuses it as stale has answered.
fn report_the_ending(
    supervisor: &TheEndpointAsACaller,
    work: &TheReadingsWork,
    ending: &TheReadingsEnding,
) -> Result<(), String> {
    let request = Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        work.operation.clone(),
        THE_WORKERS_REPORT_REQUEST,
    )
    .carrying(the_ending_as_fields(
        ending,
        Fields::none().and(THE_RUN_THE_WORKER_WAS_STARTED_UNDER, work.run.as_text()),
    ));
    loop {
        if let Ok(Some(answered)) = supervisor.ask(&request) {
            match answered.fields.text(WHETHER_THE_REPORT_WAS_ACCEPTED) {
                Some(IT_WAS_ACCEPTED) => return Ok(()),
                Some(IT_WAS_REFUSED_AS_STALE) => {
                    return Err(
                        "smoozy-server: the file-inspection worker's ending was refused as stale"
                            .to_string(),
                    )
                }
                _ => {
                    if let Some(why) = answered.fields.text(asked::THE_MACHINES_OWN_WORDS_FIELD) {
                        eprintln!(
                            "smoozy-server: the file-inspection worker's ending was not written \
                             yet: {why}"
                        );
                    }
                }
            }
        }
        std::thread::sleep(crate::the_render_worker::BETWEEN_TWO_ATTEMPTS_AT_THE_REPORT);
    }
}

/// `Render Settings` Q4: links only, removed on success and every early return.
struct TheReadingsOpeningFolder(Option<std::path::PathBuf>);
impl Drop for TheReadingsOpeningFolder {
    fn drop(&mut self) {
        if let Some(root) = &self.0 { let _ = std::fs::remove_dir_all(root); }
    }
}

/// **One reading, from the script to the ending the worker reports**, with
/// nobody told its Blender — the tests' own shape.
#[cfg(test)]
pub fn one_reading(work: &TheReadingsWork) -> TheReadingsEnding {
    one_reading_telling(work, &|_| {})
}

/// **The same, telling `started` the pair of the Blender it started**, where
/// the kernel publishes one — the route `Dependency Upload & Remap` O1's
/// interrupt is aimed by. **The contents reading starts no Blender** and is
/// read here, by this program's own code.
pub fn one_reading_telling(
    work: &TheReadingsWork,
    tell_the_pair: &dyn Fn(RecordedPair),
) -> TheReadingsEnding {
    if work.which == WhichReading::TheContentsReading {
        return one_contents_reading(work);
    }
    let not_started = |words: String| TheReadingsEnding {
        a_blender_ran: false,
        exit_status: None,
        signal: None,
        ordinary_retained: 0,
        complaints_retained: 0,
        discarded: 0,
        start_mark_seen: false,
        ended_by_smoozy: false,
        words: Some(words),
        digest: None,
    };
    // `Render Settings` Q4; `Dependency Upload & Remap` M8b: reuse the
    // job's folder construction, scoped to this file-inspection operation.
    let mut opening = TheReadingsOpeningFolder(None);
    let mut opened_work = work.clone();
    if work.which == WhichReading::TheFilesReading {
        if let Some(bytes) = &work.pairs {
            let pairs: Vec<smoozy_dependencies_machine::remap::Pair> = match serde_json::from_slice(bytes) {
                Ok(pairs) => pairs,
                Err(why) => return not_started(format!("the reading's library pairs did not read back: {why}")),
            };
            let root = std::path::Path::new(&work.script).parent().unwrap_or(std::path::Path::new("."))
                .join("opening");
            if let Some(folder) = smoozy_dependencies_machine::opening::the_folder_the_file_opens_from(
                &pairs, &work.blend, root.as_os_str().as_bytes()) {
                opening.0 = Some(root);
                if let Err(why) = smoozy_rendering_machine::acting::opening::make_the_opening_folder(&folder) {
                    return not_started(format!("the folder the file opens from could not be made: {why}"));
                }
                opened_work.blend = folder.the_file;
            }
        }
    }
    let work = &opened_work;
    // **`Job files and outputs` R18: every read of the user's file carries
    // `Rendering` L8b's script before the file**, whose handler points every
    // Replay fluid cache and every disk cache at a throwaway folder beside the
    // reading's own script, removed when the run ends.
    // **Not the fixing's**: it saves what the running Blender holds, and the
    // handler's throwaway places would be saved into the fixed copy
    // (`smoozy_dependencies_machine::fixing_script`'s head).
    let (load_time, throwaway) = if work.which == WhichReading::TheFixing {
        (String::new(), None)
    } else {
        match smoozy_rendering_machine::acting::reading::write_the_reads_load_time_script(&work.script) {
            Ok((load_time, throwaway)) => (load_time, Some(throwaway)),
            Err(why) => return not_started(why),
        }
    };
    let (script, words) = match the_script_and_the_words(work, &load_time) {
        Ok(both) => both,
        Err(why) => return not_started(why),
    };
    if let Err(why) = std::fs::write(&work.script, script) {
        return not_started(format!("{}: {why}", work.script));
    }
    let sinks = match TheCopyIntoTheSinks::opening(&work.capture) {
        Ok(sinks) => Arc::new(sinks),
        Err(why) => return not_started(format!("the two sinks could not be opened: {why}")),
    };
    let mut started = match Command::new(OsStr::from_bytes(&work.build))
        .args(words.iter().map(|word| OsStr::from_bytes(word)))
        // **Held by nothing a connection owns**, the same as the render's own
        // start: standard input closed rather than inherited.
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(started) => started,
        Err(why) => {
            return not_started(format!(
                "the reading could not be started: {} could not be run: {why}",
                String::from_utf8_lossy(&work.build)
            ))
        }
    };
    // **`Rented-server runtime` X1: the pair, read off the kernel the moment the
    // Blender started**, and told. **Where the kernel publishes nothing** the
    // Blender runs unrecorded and no stop reaches it — a reading or a fixing is
    // short, and undoing the start would end work the user asked for.
    if let Ok(Some(pair)) =
        smoozy_runtime::acting::execution_group::ThePairAsTheKernelReportsIt::the_rented_machine()
            .what_is_at(started.id())
    {
        tell_the_pair(pair);
    }

    // **`Render Settings` W2 item 4: both streams held to their end**, each
    // copied into its sink under `Rented-server runtime` M4a's one ceiling.
    let ceiling = Arc::new(Mutex::new(work.capture.ceiling));
    let copying = |stream: Stream, from: Box<dyn Read + Send>| {
        let sinks = Arc::clone(&sinks);
        let ceiling = Arc::clone(&ceiling);
        std::thread::spawn(move || copy_to_its_end(stream, from, sinks.as_ref(), &ceiling))
    };
    let ordinary = started
        .stdout
        .take()
        .map(|out| copying(Stream::Ordinary, Box::new(out)));
    let complaints = started
        .stderr
        .take()
        .map(|err| copying(Stream::Complaints, Box::new(err)));
    let copied = |one: Option<std::thread::JoinHandle<Copied>>| {
        one.and_then(|handle| handle.join().ok())
            .unwrap_or_default()
    };
    let ordinary = copied(ordinary);
    let complaints = copied(complaints);
    // **Reaped, so the reading leaves no process behind** — and only after
    // both streams reached their end.
    let ended = started.wait().ok();
    // **`Job files and outputs` R18's *removed after*.**
    if let Some(throwaway) = &throwaway {
        smoozy_rendering_machine::acting::reading::remove_the_throwaway_folder(throwaway);
    }

    let start_mark_seen = [&work.capture.ordinary_sink, &work.capture.complaints_sink]
        .iter()
        .any(|sink| {
            std::fs::read(sink)
                .map(|held| work.which.started_in(&held))
                .unwrap_or(false)
        });
    let losses: Vec<String> = [&ordinary, &complaints]
        .iter()
        .filter_map(|copied| copied.first_loss.clone())
        .collect();
    let discarded = ordinary.discarded + complaints.discarded;
    TheReadingsEnding {
        a_blender_ran: true,
        exit_status: ended.and_then(|status| status.code()),
        signal: ended.and_then(|status| status.signal()),
        ordinary_retained: ordinary.retained,
        complaints_retained: complaints.retained,
        discarded,
        start_mark_seen,
        ended_by_smoozy: false,
        words: (discarded > 0 || !losses.is_empty()).then(|| {
            format!(
                "{discarded} bytes of the reading's streams were not kept{}",
                if losses.is_empty() {
                    String::new()
                } else {
                    format!(": {}", losses.join("; "))
                }
            )
        }),
        digest: None,
    }
}

/// **`Dependency Upload & Remap` Q3's contents reading: the range read through
/// and digested by this program's own code** — `Server protocol and transfers`
/// S23's function, SHA-256, the same function the desktop's own digest is, so
/// the two compare as one. ***Nothing there* is the system's own *no such
/// file* and nothing else**: `Dependency Upload & Remap` Q7 turns it into *not
/// found*, so a refusal or a short read is *not digested*, with its words.
pub fn one_contents_reading(work: &TheReadingsWork) -> TheReadingsEnding {
    use sha2::Digest as _;
    use std::io::{Seek, SeekFrom};
    let ended = |digest: TheDigest, words: Option<String>| TheReadingsEnding {
        a_blender_ran: false,
        exit_status: None,
        signal: None,
        ordinary_retained: 0,
        complaints_retained: 0,
        discarded: 0,
        start_mark_seen: false,
        ended_by_smoozy: false,
        words,
        digest: Some(digest),
    };
    let (from, length) = work.range.unwrap_or((0, None));
    let path = OsStr::from_bytes(&work.blend);
    let mut file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(why) if why.kind() == std::io::ErrorKind::NotFound => {
            return ended(TheDigest::NothingThere, None)
        }
        Err(why) => {
            return ended(
                TheDigest::NotDigested,
                Some(format!("{}: {why}", String::from_utf8_lossy(&work.blend))),
            )
        }
    };
    if let Err(why) = file.seek(SeekFrom::Start(from)) {
        return ended(TheDigest::NotDigested, Some(format!("the range could not be reached: {why}")));
    }
    let mut running = sha2::Sha256::new();
    let mut buffer = vec![0u8; 1024 * 1024];
    let mut read_so_far = 0u64;
    loop {
        let wanted = match length {
            Some(length) => ((length - read_so_far) as usize).min(buffer.len()),
            None => buffer.len(),
        };
        if wanted == 0 {
            break;
        }
        match file.read(&mut buffer[..wanted]) {
            Ok(0) => break,
            Ok(read) => {
                running.update(&buffer[..read]);
                read_so_far += read as u64;
            }
            Err(why) if why.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(why) => {
                return ended(TheDigest::NotDigested, Some(format!("the file could not be read: {why}")))
            }
        }
    }
    // **A range that runs past the end is not the range asked for.**
    if let Some(length) = length {
        if read_so_far != length {
            return ended(
                TheDigest::NotDigested,
                Some(format!("the file ended {read_so_far} bytes into a range of {length}")),
            );
        }
    }
    let digest: String = running
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    ended(TheDigest::Digested { digest, bytes: read_so_far }, None)
}

/// **The script and the words after the build's path, for the reading asked
/// for.**
fn the_script_and_the_words(
    work: &TheReadingsWork,
    the_load_time_scripts_path: &str,
) -> Result<(String, Vec<Vec<u8>>), String> {
    match work.which {
        WhichReading::TheFilesReading => {
            let version = work
                .version
                .as_deref()
                .and_then(the_version_in)
                .ok_or("the file's reading names no version its script can be composed for")?;
            // **The render's own reading takes the `.blend`'s path as text**, and
            // this one is the same act, so a path that is not text is refused
            // in words rather than decoded.
            let blend = std::str::from_utf8(&work.blend).map_err(|_| {
                "the .blend's path is not text, and the file's reading takes it as text"
            })?;
            let invocation = TheInvocation {
                the_blender_build_path: String::from_utf8_lossy(&work.build).to_string(),
                the_blend_path_on_the_machine: RecordedLocation::of(work.blend.clone()),
                the_version_it_will_render_by: version,
                // **No job frames**: the script walks the scene's own span.
                the_frames_the_job_covers: Vec::new(),
                the_check_is_written_as_a_picture: false,
                the_check_is_marked: false,
            };
            // **No job scene**: the file's own scene is read, and beside it
            // every other scene's outputs half (`Job files and outputs` R4b).
            let script =
                smoozy_job_files_machine::reading_script::the_pre_load_reading_script(&invocation)
                    .map_err(|missing| format!("a spelling is not on record: {}", missing.what))?;
            let words = smoozy_rendering_machine::acting::reading::the_words_of_a_reading(
                blend,
                the_load_time_scripts_path,
                &work.script,
            )
            .map_err(|missing| format!("a spelling is not on record: {}", missing.what))?;
            Ok((script, words.into_iter().map(String::into_bytes).collect()))
        }
        WhichReading::TheChecksReading => Ok((
            smoozy_dependencies_machine::check_script::the_check_readings_script(),
            smoozy_dependencies_machine::check_script::the_words_of_the_check_reading(
                &work.blend,
                the_load_time_scripts_path.as_bytes(),
                work.script.as_bytes(),
            ),
        )),
        // **`Dependency Upload & Remap` M14**: the pairs as the request carried
        // them, the copy saved to its staging path.
        WhichReading::TheFixing => {
            let staging = work
                .staging
                .as_deref()
                .ok_or("the fixing names no staging path")?;
            let pairs: Vec<smoozy_dependencies_machine::remap::Pair> = serde_json::from_slice(
                work.pairs.as_deref().ok_or("the fixing names no pairs")?,
            )
            .map_err(|why| format!("the fixing's pairs did not read back: {why}"))?;
            Ok((
                smoozy_dependencies_machine::fixing_script::the_fixing_python(&pairs, staging),
                smoozy_dependencies_machine::fixing_script::the_words_of_the_fixing(
                    &work.blend,
                    work.script.as_bytes(),
                ),
            ))
        }
        WhichReading::TheContentsReading => {
            Err("the contents reading starts no Blender".to_string())
        }
    }
}

/// What one stream's copy came to.
#[derive(Debug, Clone, Default)]
pub(crate) struct Copied {
    pub(crate) retained: u64,
    pub(crate) discarded: u64,
    pub(crate) first_loss: Option<String>,
}

/// **One stream, copied into its sink until it ends.** Bytes the ceiling or a
/// refused write cannot keep are read and discarded, so the Blender is never
/// held up by a full pipe.
pub(crate) fn copy_to_its_end(
    stream: Stream,
    from: Box<dyn Read + Send>,
    sinks: &dyn CopiesAStreamIntoItsSink,
    ceiling: &Mutex<CaptureCeiling>,
) -> Copied {
    copy_to_its_end_watching(stream, from, sinks, ceiling, &mut |_| {})
}

/// **The same copy, with every chunk shown to a watcher as it passes**, before
/// the sink sees it — so a worker that has to answer something its Blender
/// prints (the preview worker's size, `Job files and outputs` B26a) reads it
/// at its own loop rather than out of the sink afterwards. Added 2026-10-03,
/// the 38's piece 3; the copy itself is unchanged.
pub(crate) fn copy_to_its_end_watching(
    stream: Stream,
    mut from: Box<dyn Read + Send>,
    sinks: &dyn CopiesAStreamIntoItsSink,
    ceiling: &Mutex<CaptureCeiling>,
    watch: &mut dyn FnMut(&[u8]),
) -> Copied {
    let mut copied = Copied::default();
    let mut buffer = vec![0u8; 64 * 1024];
    loop {
        let read = match from.read(&mut buffer) {
            Ok(0) => return copied,
            Ok(read) => {
                watch(&buffer[..read]);
                read
            }
            Err(why) if why.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(why) => {
                copied
                    .first_loss
                    .get_or_insert(format!("the stream could not be read: {why}"));
                return copied;
            }
        };
        let may = {
            let ceiling = ceiling.lock().expect("the ceiling is not poisoned");
            ceiling.may_offer(read as u64)
        };
        let kept = if copied.first_loss.is_some() {
            0
        } else {
            match sinks.write(stream, &buffer[..read], may) {
                Ok(kept) => kept,
                Err(why) => {
                    copied.first_loss = Some(why);
                    0
                }
            }
        };
        ceiling
            .lock()
            .expect("the ceiling is not poisoned")
            .retained(kept);
        copied.retained += kept;
        copied.discarded += read as u64 - kept;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **The runner's ruling of 2026-09-28, held as a fact about the frame**: a
    /// document one byte past one frame's ceiling could never ride inside an
    /// answer. The frame refuses it, so an answer carrying it would never be
    /// written — which is why the answer carries where the document is.
    #[test]
    fn a_document_past_one_frames_ceiling_could_never_ride_inside_an_answer() {
        let request = Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            Identity::issue(IdentityKind::Operation).expect("randomness"),
            asked::THE_READING_OF_A_BLEND_REQUEST,
        );
        let document = vec![b'x'; smoozy_framing::framing::THE_LARGEST_FRAME + 1];
        let answered = Answered::to(&request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "0.1.7")
            .carrying(Fields::none().and("document", document));
        let frame = smoozy_framing::framing::write_result_frame(&answered);
        assert!(
            smoozy_framing::framing::read_one_result(&mut frame.as_slice()).is_err(),
            "a frame past the ceiling was read"
        );
    }

    /// **The field names both sinks, which `Rented-server runtime` M1 and
    /// §14's unit require**, and reads back whole.
    #[test]
    fn the_kinds_own_field_names_both_sinks_and_reads_back_whole() {
        let field = TheFileInspectionKindsOwnField {
            which: WhichReading::TheChecksReading,
            build: b"/root/blender/5.2.2/blender".to_vec(),
            blend: vec![b'/', 0xff, b'.', b'b'],
            version: Some("5.2.2".to_string()),
            ordinary_sink: "/pkg/captured-streams/op/ordinary".to_string(),
            complaints_sink: "/pkg/captured-streams/op/complaints".to_string(),
            script: "/pkg/captured-streams/op/reading.py".to_string(),
            ending: None,
            staging: None,
            pairs: None,
            range: None,
            blender: None,
            stops: Vec::new(),
            the_interrupt_went_out: false,
        };
        let written = field.as_written();
        let text = String::from_utf8_lossy(&written);
        assert!(text.contains(&field.ordinary_sink) && text.contains(&field.complaints_sink));
        assert_eq!(
            TheFileInspectionKindsOwnField::read_back(&written),
            Some(field.clone())
        );
        assert_eq!(
            field.as_it_stands().text(asked::WHAT_BECAME_OF_IT_FIELD),
            Some(asked::IT_IS_STILL_IN_HAND)
        );
    }

    /// **A reading whose Blender never ran names no stream**, and one that ran
    /// names both with their lengths.
    #[test]
    fn a_reading_names_its_streams_only_where_a_blender_ran() {
        let mut field = TheFileInspectionKindsOwnField {
            which: WhichReading::TheFilesReading,
            build: b"/b".to_vec(),
            blend: b"/a.blend".to_vec(),
            version: Some("4.5.12".to_string()),
            ordinary_sink: "/o".to_string(),
            complaints_sink: "/c".to_string(),
            script: "/s.py".to_string(),
            ending: Some(TheReadingsEnding {
                a_blender_ran: false,
                exit_status: None,
                signal: None,
                ordinary_retained: 0,
                complaints_retained: 0,
                discarded: 0,
                start_mark_seen: false,
                ended_by_smoozy: false,
                words: Some("/b could not be run".to_string()),
                digest: None,
            }),
            staging: None,
            pairs: None,
            range: None,
            blender: None,
            stops: Vec::new(),
            the_interrupt_went_out: false,
        };
        let answer = field.as_it_stands();
        assert_eq!(
            answer.text(asked::WHAT_BECAME_OF_IT_FIELD),
            Some(asked::IT_HAS_A_RECORDED_OUTCOME)
        );
        assert_eq!(answer.text(asked::THE_ORDINARY_STREAM_FIELD), None);
        assert_eq!(answer.text(asked::THE_EXIT_STATUS_FIELD), None);
        assert_eq!(
            answer.text(asked::THE_MACHINES_OWN_WORDS_FIELD),
            Some("/b could not be run")
        );

        let ending = field.ending.as_mut().expect("an ending");
        ending.a_blender_ran = true;
        ending.exit_status = Some(0);
        ending.ordinary_retained = 1_200_000;
        ending.start_mark_seen = true;
        let answer = field.as_it_stands();
        assert_eq!(answer.text(asked::THE_ORDINARY_STREAM_FIELD), Some("/o"));
        assert_eq!(
            answer.text(asked::THE_ORDINARY_STREAM_LENGTH_FIELD),
            Some("1200000")
        );
        assert_eq!(answer.text(asked::THE_EXIT_STATUS_FIELD), Some("0"));
        assert_eq!(
            answer.text(asked::THE_START_MARK_SEEN_FIELD),
            Some(asked::THE_ANSWER_IS_YES)
        );
    }

    /// **`Render Settings` Q8, the Local View crash: a reading whose Blender
    /// wrote the started mark and then died by a signal is one that started**
    /// — the third ending, not the second — and one that wrote nothing and
    /// ended at a failing status is not. A stand-in stands where the build's
    /// path goes; **nothing here says what a real Blender prints.**
    #[test]
    fn a_reading_that_crashed_after_its_first_act_is_a_start_and_one_that_wrote_nothing_is_not() {
        use std::os::unix::fs::PermissionsExt;
        let root = std::env::temp_dir().join(format!(
            "smoozy-a-reading-that-crashed-{}",
            Identity::issue(IdentityKind::Operation)
                .expect("randomness")
                .as_text()
        ));
        std::fs::create_dir_all(&root).expect("the folder");
        let reading_with = |stand_in: &str, named: &str| {
            let build = root.join(named);
            std::fs::write(&build, stand_in).expect("the stand-in");
            std::fs::set_permissions(&build, std::fs::Permissions::from_mode(0o755))
                .expect("runnable");
            let at = |name: &str| root.join(format!("{named}-{name}")).display().to_string();
            // **`Rented-server runtime` M1: the supervisor makes both sinks
            // before the worker starts**, so the stand-in's are made here.
            for sink in ["ordinary", "complaints"] {
                std::fs::write(at(sink), b"").expect("the sink");
            }
            one_reading(&TheReadingsWork {
                operation: Identity::issue(IdentityKind::Operation).expect("randomness"),
                run: Identity::issue(IdentityKind::Operation).expect("randomness"),
                which: WhichReading::TheFilesReading,
                build: build.display().to_string().into_bytes(),
                blend: b"/a.blend".to_vec(),
                version: Some("5.1.2".to_string()),
                script: at("reading.py"),
                capture: CaptureRecord {
                    job: Identity::issue(IdentityKind::Job).expect("randomness"),
                    attempt: 1,
                    ordinary_sink: at("ordinary"),
                    complaints_sink: at("complaints"),
                    ceiling: CaptureCeiling::from(u64::MAX),
                },
                staging: None,
                pairs: None,
                range: None,
            })
        };
        let started = std::str::from_utf8(smoozy_job_files_machine::reading::THE_STARTED_MARK)
            .expect("the mark is text");

        let crashed = reading_with(
            &format!("#!/bin/sh\nprintf '%s' '{started}'\nkill -SEGV $$\n"),
            "crashed",
        );
        assert!(crashed.a_blender_ran, "{:?}", crashed.words);
        assert_eq!(crashed.signal, Some(11));
        assert_eq!(crashed.exit_status, None);
        assert!(crashed.start_mark_seen, "the script started, so the read ran");

        let refused = reading_with(
            "#!/bin/sh\necho \"Error: Failed to read blend file '/a.blend': Missing DNA block\" >&2\nexit 1\n",
            "refused",
        );
        assert_eq!(refused.exit_status, Some(1));
        assert!(!refused.start_mark_seen, "a file Blender could not open runs no script");

        // **The script the reading wrote carries the mark as its first act.**
        let script = std::fs::read_to_string(root.join("crashed-reading.py")).expect("written");
        assert!(script.starts_with(&format!("import sys\nsys.stdout.write(\"{started}\")")));
        let _ = std::fs::remove_dir_all(&root);
    }

    // `Render Settings` Q4: the page opens the same library folder as the job.
    #[test]
    fn the_pages_reading_opens_its_libraries_and_cleans_up_even_on_failure() {
        use std::os::unix::fs::PermissionsExt;
        use smoozy_dependencies_machine::{bytes,check::Kind,remap};
        let root=std::env::temp_dir().join(format!("smoozy-page-links-{}",a_fresh_identity().unwrap().as_text()));
        std::fs::create_dir_all(root.join("files")).unwrap();
        std::fs::write(root.join("files/top.blend"),b"top").unwrap();
        std::fs::write(root.join("library.blend"),b"linked simulation").unwrap();
        let pair=remap::compose_pair(Kind::Library,"filepath",
            &bytes::RecordedLocation::as_the_reading_carried_it(b"//lib/rig.blend"),
            remap::NewLocation::AFixedCopy(bytes::MachineLocation::composed(root.join("library.blend").as_os_str().as_bytes().to_vec()))).unwrap();
        let build=root.join("reader");
        std::fs::write(&build,r#"#!/usr/bin/env python3
import sys,pathlib
path=next(pathlib.Path(x) for x in sys.argv[1:] if x.endswith('.blend'))
assert path.read_bytes()==b'top'
assert (path.parent/'lib/rig.blend').read_bytes()==b'linked simulation'
print('linked simulation seen')
"#).unwrap();
        std::fs::set_permissions(&build,std::fs::Permissions::from_mode(0o755)).unwrap();
        let work=TheReadingsWork {
            operation:a_fresh_identity().unwrap(),run:a_fresh_identity().unwrap(),
            which:WhichReading::TheFilesReading,build:build.as_os_str().as_bytes().to_vec(),
            blend:root.join("files/top.blend").as_os_str().as_bytes().to_vec(),version:Some("5.3.0".into()),
            script:root.join("reading.py").display().to_string(),
            capture:CaptureRecord {job:a_fresh_identity().unwrap(),attempt:1,
                ordinary_sink:root.join("ordinary").display().to_string(),
                complaints_sink:root.join("complaints").display().to_string(),ceiling:CaptureCeiling::from(u64::MAX)},
            staging:None,pairs:Some(serde_json::to_vec(&vec![pair]).unwrap()),range:None,
        };
        for sink in [&work.capture.ordinary_sink,&work.capture.complaints_sink] {std::fs::write(sink,b"").unwrap();}
        let ended=one_reading(&work);
        assert_eq!(ended.exit_status,Some(0),"{:?}: {}",ended.words,std::fs::read_to_string(&work.capture.complaints_sink).unwrap());
        assert!(std::fs::read_to_string(&work.capture.ordinary_sink).unwrap().contains("linked simulation seen"));
        assert!(!root.join("opening").exists(),"reading folder removed");
        let mut refused=work.clone();refused.build=b"/no-such-blender".to_vec();
        assert!(!one_reading(&refused).a_blender_ran);
        assert!(!root.join("opening").exists(),"folder removed after failed spawn");
        assert_eq!(std::fs::read(root.join("library.blend")).unwrap(),b"linked simulation");
        std::fs::remove_dir_all(root).unwrap();
    }

    /// **The report reads back into the ending it was made from.**
    #[test]
    fn the_ending_crosses_the_report_whole() {
        let ending = TheReadingsEnding {
            a_blender_ran: true,
            exit_status: None,
            signal: Some(2),
            ordinary_retained: 12,
            complaints_retained: 34,
            discarded: 0,
            start_mark_seen: true,
            ended_by_smoozy: false,
            words: None,
            digest: None,
        };
        assert_eq!(
            the_ending_in(&the_ending_as_fields(&ending, Fields::none())),
            Some(ending)
        );
        assert_eq!(
            the_ending_in(&Fields::none()),
            None,
            "a render's report is not a reading's"
        );
    }

    /// **The version text reads as its first two parts**, and nothing else
    /// reads as a version.
    #[test]
    fn the_version_is_read_from_its_first_two_parts() {
        assert_eq!(the_version_in("5.2.2"), Some(TheVersionInHand::of(5, 2)));
        assert_eq!(the_version_in("4.5"), Some(TheVersionInHand::of(4, 5)));
        assert_eq!(the_version_in("five"), None);
        assert_eq!(the_version_in(""), None);
    }
}
