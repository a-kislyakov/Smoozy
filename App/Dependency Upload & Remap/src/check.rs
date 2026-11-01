//! The check — `Dependency Upload & Remap` §3, Rows 159 to 164 and 171: the
//! judgements inside it. **The loop that drives it is not here.**
//!
//! **G14 draws the line this file keeps: the check is a desktop-driven loop of
//! single-file readings, not one long machine-side operation.** Each reading
//! is one operation with its own identity (G15), and what the loop does with
//! a reading — judge its ending (G3), take its kinds (G6), classify each
//! recorded form (G9), group references into rows (G10, G11), order them
//! (G11a), keep the walk finite (G13), read scene attribution (G20, G20a),
//! expand a set (G22), exclude the application's own entry (G27) and leave out
//! a library the Blender reading the file ships (G27a) — is
//! judgement over the document the reading returned, and is here. Issuing the
//! reading, admitting it, cancelling it, and carrying the tree down as files
//! arrive is the acting half's ([`crate::arriving_later`]).
//!
//! **G4: Blender 5.1 was not installed in the batch and is not spoken for.**
//! Every per-version claim is observed in 4.5.9, 5.0.1 and 5.2.0 and inferred
//! for 5.1 — §12 item 5, whose entry has no *what holds* limb, so the
//! document's own answer stands as written with that caution beside it.
//!
//! **G5: whether a read disturbs a running render is not answered**, and the
//! measurement is not coming (§12 item 16, permanently unresolved). Section
//! 9's declaration is shaped so that one edit tightens it — a property of the
//! shape and not a space kept for a measurement ([`crate::declarations`]).
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Render Settings` B2a, B5. `Job files and
//! outputs` R1, R4a, B4, C9, D21, I3, N15.

use std::cmp::Ordering;
use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use smoozy_render_settings::file_half::BlenderIdentity;
use smoozy_render_settings::version::{Resolution, WhyNone};

use crate::bytes::{segments, MachineLocation, RecordedLocation, StoredLocation, SEPARATOR};

/// **G2: the Blender driven is the one the job will render with — the version
/// selected in `Render Settings` for this file.** That module owns the
/// selection (its B2a: the store's override where one stands, else the
/// automatic match); this document consumes it. **A check is not run without
/// a named version.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NoCheckRuns {
    /// **G2's honest case, `Render Settings` B5's two met at the check
    /// instead of at submission**: no version can be resolved — the machine
    /// holds only below-floor builds and no pick, or no Blender at all — so
    /// no check runs and the user is told with the reason named. Wording
    /// Design's.
    NoVersionResolves(WhyNone),
}

pub fn the_version_to_read_with(resolution: &Resolution) -> Result<BlenderIdentity, NoCheckRuns> {
    match resolution {
        Resolution::TheUsersOverride(pick) => Ok(pick.clone()),
        Resolution::Automatic { build, .. } => Ok(build.as_identity()),
        Resolution::NoVersionResolves(why) => Err(NoCheckRuns::NoVersionResolves(*why)),
    }
}

/// **The kinds, at the path they have always been reached at.** The enum and
/// its judgements live in `smoozy-dependencies-machine`, because the remap
/// material's pairs carry a kind and the rented machine's own program reads
/// them. Split at step 10c piece 3.
pub use smoozy_dependencies_machine::check::Kind;

/// **The three kinds on G6's list the reading produces no reference for**, each
/// a named absence with what breaks rather than a silent drop.
///
/// **Every other kind on that list is reached**, and a test holds this list and
/// the script against [`Kind::ALL`] so that a kind can neither be dropped
/// quietly nor added without saying which of the two it is. The reasons are
/// written where the walk would otherwise have reached them, at
/// [`crate::acting::reading_script::THE_WALK_OVER_THE_NESTED_MARKS`]; in
/// short:
///
/// - **[`Kind::BrushIcon`] — no route is established at all.** Blender's own
///   reference carries no path-shaped property on a brush. **Breaks:** a custom
///   brush icon is unfound and undisclosed. **Holds:** G7's own honesty — the
///   check never claims the list is all of them.
/// - **[`Kind::PackedFileRecord`] — the bytes are inside the `.blend`.** A
///   packed picture ignores its file on disk (V8), and Blender's own walker
///   skips packed data by default. **Breaks:** the record's own text is never
///   shown. **Holds:** nothing is missing at the render, and no row asks the
///   user to find a file the render will not read.
/// - **[`Kind::LibraryWeakReference`] — read for G27 and never a row.** It is
///   a *"Read-only external reference"* and cannot be assigned. **Breaks:** its
///   recorded library is never offered as something to find or fix.
///   **Holds:** a row for it would carry an action that raises, which is the
///   spec's *a row that cannot be acted on is a control that does nothing* in
///   its worst form.
pub const THE_KINDS_NO_REFERENCE_IS_PRODUCED_FOR: [Kind; 3] = [
    Kind::BrushIcon,
    Kind::PackedFileRecord,
    Kind::LibraryWeakReference,
];

/// **G6: write-only destinations are not rows; they are the redirect's
/// (`Job files and outputs` Section 5).**
///
/// **The bake output path is not one of them.** G6 says, in as many words,
/// that it is a recorded write place that is also read, so it is a row where
/// a baked result exists to send, and its uploaded copy is placed at the
/// redirect's own job-folder target (M22a, Creator-stated 2026-08-22). It is
/// [`Kind::BakeFolder`] and nothing here. **G6 carried it on both lists until
/// 2026-09-05**, and this enum was built from the sentence that has gone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteOnlyDestination {
    RenderOutput,
    FileOutputNode,
    CollectionExporter,
}

impl WriteOnlyDestination {
    pub fn is_a_row(self) -> bool {
        false
    }
}

/// **G6: the map is `bpy.data.file_path_map` and the flat list is
/// `bpy.utils.blend_paths`, and the map is not optional** — where a linked
/// library points at a picture of its own, the map reaches through the
/// library and the flat list does not (Fact 128, *Dependencies 20*, observed
/// in a test). The enumeration is the walk over Blender's own path marks
/// joined with both, never any one alone, plus G8's node-tree walk.
pub const THE_MAP: &str = "bpy.data.file_path_map";
pub const THE_FLAT_LIST: &str = "bpy.utils.blend_paths";
pub const THE_FLAT_LIST_IS_NEVER_SUBSTITUTED_FOR_THE_MAP: bool = true;

/// **G7: a path held in a custom property is reached by no route this stage
/// has and is therefore not found at all** — not found and not disclosed,
/// exactly as `Job files and outputs` D21 states for the redirect. Whether
/// the kind list is complete is permanently unresolved (§12 item 13): an
/// unknown kind is unfound silently, and the check never claims the list is
/// all of them. **There is no [`Kind`] for it, so no pair is ever composed
/// for one (M7).**
///
/// **M20: the reach limits arrive symmetrically in the check and the remap** —
/// what cannot be found cannot be remapped. This constant is the check's half
/// and [`crate::remap::NO_PAIR_IS_COMPOSED_FOR_A_CUSTOM_PROPERTY`] is the
/// remap's, and the two are one limit written at both ends rather than one end
/// quietly covering for the other.
pub const A_CUSTOM_PROPERTY_PATH_IS_NOT_FOUND_AT_ALL: bool = true;

/// **G8's two settled constraints**: a `//` inside a **linked** node group's
/// import node is measured from the library file and never from the file
/// doing the linking, in every version; and packing never carries an imported
/// model into the `.blend`, so it is always an outside file.
pub const A_LINKED_NODE_GROUPS_RELATIVE_IMPORT_PATH_IS_MEASURED_FROM_THE_LIBRARY: bool = true;

/// **G20 and G20a: scene attribution per row — the set of every scene the
/// completed walk reached, or nothing.** *Established* means the walk
/// completed and this is every scene it reached; a walk that did not complete
/// says nothing about scenes rather than guessing, and the user sets the row
/// aside on their own memory of the file. **Stopping at the first scene would
/// make a wrong statement, not a short one.**
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Attribution {
    Established(Vec<String>),
    NotEstablished,
}

/// One reference as the reading's document carries it — a datablock in a
/// path-carrying collection, its kind, the setting that holds the location
/// (a property path, which M4 needs), the recorded location's bytes (G3a) and
/// its scene attribution (G19).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadReference {
    pub kind: Kind,
    pub datablock: String,
    pub setting: String,
    pub recorded: RecordedLocation,
    pub scenes: Attribution,
    /// **G9 with DW0-3: a picture's or a movie clip's source, as Blender
    /// stores it** — `Image.source` or `MovieClip.source`, the enumeration's
    /// own identifier carried as text (the quotations are at
    /// [`crate::acting::reading_script`]). **Nothing for every other kind.**
    /// A document that carries no such field reads as nothing — serde's own
    /// rule for an absent optional field — which is G9's rule as written.
    pub source: Option<String>,
}

impl ReadReference {
    /// **DW0-3's reading of [`Self::source`].**
    pub fn stored_source(&self) -> StoredSource {
        StoredSource::of(self.kind, self.source.as_deref())
    }
}

/// **G9 with DW0-3: a picture or a movie clip is a set only where Blender
/// stores it as one.** Added 2026-09-29, dependency wave piece 1.
///
/// **One file** is a picture Blender stores as `FILE` (*"Single Image --
/// Single image file."*) or `MOVIE` (*"Movie -- Movie file."*), and a movie
/// clip it stores as `MOVIE` (*"Movie File -- Movie file."*). For those,
/// frame-number characters, a tile token or a `%` in the name are part of the
/// name: *"the row is ordinary and is looked for at exactly the recorded
/// location."* **Everything else keeps G9's rule as written** — a picture
/// stored as a sequence or as tiles, a clip stored as a sequence, a value this
/// crate does not know, a reference that carries no source, and every other
/// kind G24 names. So the wide side, a set, is what an unknown gets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoredSource {
    OneFile,
    NotSaidToBeOneFile,
}

impl StoredSource {
    pub fn of(kind: Kind, source: Option<&str>) -> StoredSource {
        match (kind, source) {
            (Kind::Image, Some("FILE" | "MOVIE")) | (Kind::MovieClip, Some("MOVIE")) => {
                StoredSource::OneFile
            }
            _ => StoredSource::NotSaidToBeOneFile,
        }
    }
}

/// **G27's mark: a `library_weak_reference` on an ordinary datablock**, read
/// through `library_weak_reference.filepath` with `library_weak_reference.id_name`
/// beside it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeakReference {
    pub filepath: RecordedLocation,
    pub id_name: String,
}

/// **The document one reading returns between its sentinels** — `Job files
/// and outputs` R1's shape, whose carriage of a location's bytes unaltered is
/// that document's R4a (§12 item 18, closed).
///
/// **G27a is applied as the document is read**: a library the Blender reading
/// it ships never reaches the rest of Smoozy as a reference, so nothing
/// downstream can make a row of it ([`TheDocumentAsTheReadingWroteIt`]).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "TheDocumentAsTheReadingWroteIt")]
pub struct ReadingDocument {
    /// Every datablock in a path-carrying collection, with its recorded
    /// location.
    pub references: Vec<ReadReference>,
    /// The flat list, as G6 joins it in.
    pub flat_list: Vec<RecordedLocation>,
    /// Every weak reference, one per datablock (G27: more than one can
    /// exist).
    pub weak_references: Vec<WeakReference>,
    /// **G6b: the libraries where the reading observed a linked item that
    /// holds nothing** — a statement is produced only from something
    /// observed and never inferred from an absence, so a reading that cannot
    /// see a stand-in leaves this empty and says nothing.
    pub observed_stand_ins: Vec<RecordedLocation>,
}

/// What the invocation left behind — the acting half captures it, and
/// [`judge`] reads it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheReadingLeft {
    ASentinelFramedDocument(ReadingDocument),
    NoSentinelFramedDocument,
    AFailingStatus,
}

/// **G3: where that Blender cannot open the file at all, nothing is checked:
/// no rows, and Smoozy says the file could not be read and with which
/// version.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reading {
    Read(ReadingDocument),
    CouldNotBeRead { with: BlenderIdentity },
}

/// **G3: an empty answer is never inferred from a missing one** — a run that
/// returns no sentinel-framed document, or a failing status, is *could not be
/// read*, never an empty list. The same discipline as `Job files and outputs`
/// B4 and C9: a reading that did not complete is not a reading.
pub fn judge(left: WhatTheReadingLeft, with: &BlenderIdentity) -> Reading {
    match left {
        WhatTheReadingLeft::ASentinelFramedDocument(document) => Reading::Read(document),
        WhatTheReadingLeft::NoSentinelFramedDocument | WhatTheReadingLeft::AFailingStatus => {
            Reading::CouldNotBeRead { with: with.clone() }
        }
    }
}

/// **G6a: a linked file that is there while the named thing inside it is
/// gone is said as a statement, not as a row.** It names the library file,
/// which is the one thing always known, and **does not name the item**,
/// because Blender's signal never does. Not a row, not a warning, no action,
/// no suppression, no switch-off (V3a). It changes nothing about the check's
/// standing: V11's stop rule counts rows and this produces none.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkedItemStatement {
    pub library: RecordedLocation,
}

/// **G6b: produced only from something observed, never inferred from an
/// absence.** A run that read no stand-in is never read as proof there was
/// none, and the panel never claims the file is clean of this.
pub fn linked_item_statements(document: &ReadingDocument) -> Vec<LinkedItemStatement> {
    document
        .observed_stand_ins
        .iter()
        .map(|library| LinkedItemStatement {
            library: library.clone(),
        })
        .collect()
}

/// The marks that make a recorded location a set row (G9, 3.6).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetMarks {
    pub frame_number_characters: bool,
    pub udim: bool,
    pub uvtile: bool,
    pub view_name: bool,
}

/// **G9: forms that stand for something are enumerated as what they stand for
/// and never resolved as text.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Form {
    Ordinary,
    /// Frame-number characters, `<UDIM>` and `<UVTILE>` tokens, and `%` for a
    /// view name mark a set row (3.6).
    ASet(SetMarks),
    /// A `~` and an environment variable are stored exactly as written and
    /// never expanded anywhere, so they resolve to nothing and their rows read
    /// *not found* unless the user resolves them by hand. **The recorded
    /// location itself is still looked at for one** (F27), because that
    /// lookup asks the disk for the reference's exact recorded bytes and can
    /// never reach a file the reference does not mean.
    ATildeOrAnEnvironmentVariable,
}

/// **There is no curly-brace form here, and its absence is the rule rather
/// than an omission.** G9 allows a curly-brace expression only in the render
/// output path and the File Output node, and both are
/// [`WriteOnlyDestination`]s that produce no reference — so no reference this
/// function ever classifies can be one, and no pair can name one
/// ([`crate::remap::NO_PAIR_IS_COMPOSED_FOR_A_CURLY_BRACE_EXPRESSION`] is
/// M7's other half kept the same way).
///
/// **This used to be a text test — any `{` anywhere and any `}` anywhere —
/// and it cost a folder honestly named `{hero}` its whole row**: the search
/// passed it by, so it read *not found* while the file sat where the `.blend`
/// said, and no pair was ever composed for it, so the reference was never
/// re-pointed even after the user located it by hand and it was sent. Removed
/// 2026-09-05, out of round 1 of this module's code audit; M7 is kept, by
/// construction rather than by a test over the bytes.
///
/// **DW0-3: the stored source comes first among the marks.** A picture or a
/// movie clip Blender stores as one file is [`Form::Ordinary`] whatever its
/// name holds — a still picture named `лев's база #2.jpg` sat exactly where
/// its `.blend` said and was *not found*, because this function looked for a
/// folder of frames instead. **The tilde and environment-variable test still
/// comes before it**, because that is about how the location is written and
/// not about what it stands for.
pub fn classify(location: &RecordedLocation, stored: StoredSource) -> Form {
    let bytes = location.as_bytes();
    if bytes.starts_with(b"~") || bytes.contains(&b'$') {
        return Form::ATildeOrAnEnvironmentVariable;
    }
    if stored == StoredSource::OneFile {
        return Form::Ordinary;
    }
    let name = location.wanted_name();
    let marks = SetMarks {
        frame_number_characters: name.contains(&b'#'),
        udim: contains(name, b"<UDIM>"),
        uvtile: contains(name, b"<UVTILE>"),
        view_name: name.contains(&b'%'),
    };
    if marks.frame_number_characters || marks.udim || marks.uvtile || marks.view_name {
        Form::ASet(marks)
    } else {
        Form::Ordinary
    }
}

impl Form {
    /// Whether the search looks for a reference of this form at all. A `~` or
    /// an environment variable resolves to nothing unless by hand, and
    /// searching for its final segment would find a file it does not mean.
    /// **F27's lookup at the recorded location itself is not the search and
    /// is taken for every form**, this one included.
    pub fn is_searched(&self) -> bool {
        matches!(self, Form::Ordinary | Form::ASet(_))
    }
}

fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    haystack
        .windows(needle.len())
        .any(|window| window == needle)
}

/// **G22: the expansion is a listing of the resolved folder with members
/// matched against the pattern's own form** — a run of frame-number
/// characters matches a run of digits at least that many long (the padding is
/// a floor, not a ceiling — the same recipe fact `Job files and outputs` I3
/// records); a `<UDIM>` or `<UVTILE>` token matches a tile number in that
/// token's own place; a `%` matches a view name. **The matching rule is
/// Smoozy's own decision and errs wide**: a member Blender would not have
/// counted costs bandwidth; a member Smoozy misses goes to the substitute
/// colour on its own. How Blender itself decides membership is uncovered by
/// the research and is deliberately not modelled.
pub fn set_member_matches(pattern: &[u8], candidate: &[u8]) -> bool {
    matches_from(&tokens_of(pattern), candidate)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Literal(u8),
    /// At least this many digits.
    Digits(usize),
    /// A view name: at least one byte that is not a separator.
    ViewName,
}

fn tokens_of(pattern: &[u8]) -> Vec<Token> {
    let mut out = Vec::new();
    let mut at = 0;
    while at < pattern.len() {
        let rest = &pattern[at..];
        if rest.starts_with(b"<UDIM>") {
            out.push(Token::Digits(1));
            at += b"<UDIM>".len();
        } else if rest.starts_with(b"<UVTILE>") {
            out.push(Token::Digits(1));
            at += b"<UVTILE>".len();
        } else if rest[0] == b'#' {
            let run = rest.iter().take_while(|byte| **byte == b'#').count();
            out.push(Token::Digits(run));
            at += run;
        } else if rest[0] == b'%' {
            out.push(Token::ViewName);
            at += 1;
        } else {
            out.push(Token::Literal(rest[0]));
            at += 1;
        }
    }
    out
}

fn matches_from(tokens: &[Token], candidate: &[u8]) -> bool {
    match tokens.first() {
        None => candidate.is_empty(),
        Some(Token::Literal(byte)) => {
            candidate.first() == Some(byte) && matches_from(&tokens[1..], &candidate[1..])
        }
        Some(Token::Digits(at_least)) => {
            let run = candidate
                .iter()
                .take_while(|byte| byte.is_ascii_digit())
                .count();
            (*at_least..=run)
                .any(|taken| taken > 0 && matches_from(&tokens[1..], &candidate[taken..]))
        }
        Some(Token::ViewName) => {
            let run = candidate
                .iter()
                .take_while(|byte| **byte != SEPARATOR)
                .count();
            (1..=run).any(|taken| matches_from(&tokens[1..], &candidate[taken..]))
        }
    }
}

/// **G21: a set row is expanded by Smoozy's own rule in every version, and
/// 5.2's expansion is not leaned on.** One rule rather than two.
pub const SMOOZY_EXPANDS_A_SET_IN_EVERY_VERSION: bool = true;

/// **G24: the kinds that expand this way.** Render output is a write place and
/// not gathered.
pub const THE_KINDS_THAT_EXPAND: [Kind; 6] = [
    Kind::Image,
    Kind::Volume,
    Kind::CacheArchive,
    Kind::CacheLayer,
    Kind::SequencerStripFolder,
    Kind::PhysicsPointCache,
];

/// **G23: the whole set is one row and goes up as one thing** — its size and
/// share cover the whole set, pointing the row at a folder points the whole
/// row, each member crosses as its own transfer, and a retry re-sends only
/// members that did not publish (Section 10).
pub const THE_WHOLE_SET_IS_ONE_ROW: bool = true;

/// **G10: a row stands for the thing pointed at; a located-answer record
/// stands for one reference.** What a row was grouped by.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GroupedBy {
    /// **G11, before resolution: by the tidied resolved-location text** —
    /// Blender's own one-library rule, where two recordings differing only in
    /// how the path is written are one library because Blender tidies the
    /// writing before it decides, and two recordings reaching one file by two
    /// names on disk are separate because the tidying stops at the text.
    TidiedText(Vec<u8>),
    /// **After resolution: by the resolved file** — the same stored whole
    /// location (F12), and never a name match alone.
    TheResolvedFile(StoredLocation),
}

/// One row of the panel: the references it stands for, and what grouped them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    pub references: Vec<RecordedLocation>,
    pub grouped_by: GroupedBy,
}

impl Row {
    /// The row says that more than one reference led to it (G10).
    pub fn more_than_one_reference_led_to_it(&self) -> bool {
        self.references.len() > 1
    }

    /// **G11a: a many-reference row's key is the least of its references'
    /// recorded locations under the ordering** — deterministic, derived from
    /// values the row already holds, needing nothing kept.
    pub fn ordering_key(&self) -> &RecordedLocation {
        self.references
            .iter()
            .min_by(|one, other| compare_recorded(one, other))
            .expect("a row stands for at least one reference")
    }
}

/// **G11's tidying for the grouping text — Smoozy's own, over the text and
/// stopping at the text**: the blend-file shorthand resolved against the
/// holding file's own folder, separators as one byte, `.` and `..` steps
/// collapsed. **No folding, no lookup of the disk, no symlink resolved** —
/// two names on disk stay two texts.
pub fn tidied(holder_folder: &[u8], recorded: &RecordedLocation) -> Vec<u8> {
    let mut text: Vec<u8> = recorded
        .as_bytes()
        .iter()
        .map(|byte| if *byte == b'\\' { SEPARATOR } else { *byte })
        .collect();
    if let Some(rest) = text.strip_prefix(b"//") {
        text = crate::bytes::join(holder_folder, rest);
    }
    let absolute = text.starts_with(&[SEPARATOR]);
    let mut out: Vec<&[u8]> = Vec::new();
    for segment in segments(&text) {
        match segment {
            b"." => {}
            b".." => {
                out.pop();
            }
            other => out.push(other),
        }
    }
    let mut joined = Vec::new();
    if absolute {
        joined.push(SEPARATOR);
    }
    for (index, segment) in out.iter().enumerate() {
        if index > 0 {
            joined.push(SEPARATOR);
        }
        joined.extend_from_slice(segment);
    }
    joined
}

/// One reference as the grouping sees it: its recorded location, the folder
/// of the file holding it, and where it has resolved to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReferenceToGroup {
    /// The reference's kind — which G6's exception for the bake output path
    /// (DW0-2) is read against.
    pub kind: Kind,
    pub recorded: RecordedLocation,
    pub holder_folder: Vec<u8>,
    pub resolved: Option<StoredLocation>,
}

/// **G6 with DW0-2: the bake output path never sends the `.blend`'s own
/// folder.** Added 2026-09-29, dependency wave piece 1. *"Except where the
/// bake output path names the folder that holds the `.blend`, or any folder
/// above it … A bake output path that resolves there produces no row."*
///
/// **Every new scene carries `//` there**, so every file met this: the check
/// found the `.blend`'s own folder, called it found, and would have sent the
/// whole of it, the file being checked included — and a file in the downloads
/// folder would have proposed sending that folder.
///
/// **Two ways of judging, as the clause gives them.**
///
/// - **Relative (`//…`): on the recorded text alone**, no disk asked — *"a
///   location that never descends below the `.blend`'s own folder."* The text
///   after `//` is walked segment by segment, `.` ignored and `..` taking back
///   a named segment where there is one; what is left names the `.blend`'s
///   own folder or one above it exactly where no named segment is left. So
///   `//`, `//..` and `//bake/..` are the folder or above, and `//bake/` and
///   `//../bake/` are not (the second is beside it, not above it). **Split at
///   the machine's separator alone**, like every recorded location in this
///   crate: a backslash-written one is the permanently unresolved Windows case
///   ([`crate::bytes`]'s own rule), and is a row as before.
/// - **Written in full: on the resolved location**, against the folder that
///   holds the `.blend` — **`holder_folder`, which is the source folder
///   `crate::search` takes as `the_holders_source_folder`** where the
///   holding file's origin is known. It is that folder or above where the
///   resolved folder is the holder's folder or one of the folders it sits in,
///   walked up by [`StoredLocation::folder`] so the user's own computer's
///   separators decide. **Not resolved, and it is a row as before**: an
///   unfound full location is not known to be the `.blend`'s own folder.
///
/// **Any other kind is never touched here.**
pub fn the_bake_output_path_makes_no_row(reference: &ReferenceToGroup) -> bool {
    if reference.kind != Kind::BakeFolder {
        return false;
    }
    let bytes = reference.recorded.as_bytes();
    if let Some(relative) = bytes.strip_prefix(b"//") {
        let mut named = 0usize;
        for segment in segments(relative) {
            match segment {
                b"." => {}
                b".." => named = named.saturating_sub(1),
                _ => named += 1,
            }
        }
        return named == 0;
    }
    let Some(resolved) = &reference.resolved else {
        return false;
    };
    let whole = resolved.as_bytes();
    let wanted = match crate::bytes::without_trailing_separators(whole) {
        // The root is all separators, and walking up ends at its first byte.
        [] if !whole.is_empty() => &whole[..1],
        [] => return false,
        trimmed => trimmed,
    };
    let mut here = StoredLocation::as_the_system_returned_it(
        crate::bytes::without_trailing_separators(&reference.holder_folder),
    );
    loop {
        if here.as_bytes() == wanted {
            return true;
        }
        let up = here.folder();
        if up.is_empty() || up == here.as_bytes() {
            return false;
        }
        here = StoredLocation::as_the_system_returned_it(&up);
    }
}

/// **G10 and G11: references are grouped into one row where they resolve to
/// the same file** — before resolution by their tidied text, after it by the
/// resolved file. Order within the result is G11a's.
///
/// **G6 with DW0-2 is decided here, where the row is composed**: a bake output
/// path naming the `.blend`'s own folder or one above it
/// ([`the_bake_output_path_makes_no_row`]) produces no row.
pub fn group_into_rows(references: &[ReferenceToGroup]) -> Vec<Row> {
    let mut rows: Vec<Row> = Vec::new();
    for reference in references {
        if the_bake_output_path_makes_no_row(reference) {
            continue;
        }
        let key = match &reference.resolved {
            Some(file) => GroupedBy::TheResolvedFile(file.clone()),
            None => GroupedBy::TidiedText(tidied(&reference.holder_folder, &reference.recorded)),
        };
        match rows.iter_mut().find(|row| row.grouped_by == key) {
            Some(row) => row.references.push(reference.recorded.clone()),
            None => rows.push(Row {
                references: vec![reference.recorded.clone()],
                grouped_by: key,
            }),
        }
    }
    rows.sort_by(|one, other| compare_recorded(one.ordering_key(), other.ordering_key()));
    rows
}

/// **G11a: the within-group ordering — the recorded location's own bytes,
/// compared whole segment by whole segment, a location standing at its final
/// segment before one still descending.** The value ordered is G3a's byte
/// object and never the drawable form; the key is the location as recorded,
/// never as resolved and never tidied; the comparison is over exact bytes with
/// no folding. It is a run-held comparison made at drawing and derives
/// nothing.
pub fn compare_recorded(one: &RecordedLocation, other: &RecordedLocation) -> Ordering {
    compare_paths(one.as_bytes(), other.as_bytes())
}

pub fn compare_paths(one: &[u8], other: &[u8]) -> Ordering {
    let left = segments(one);
    let right = segments(other);
    let mut at = 0;
    loop {
        match (left.get(at), right.get(at)) {
            (None, None) => return Ordering::Equal,
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (Some(a), Some(b)) => {
                let a_stands = at + 1 == left.len();
                let b_stands = at + 1 == right.len();
                match (a_stands, b_stands) {
                    (true, false) => return Ordering::Less,
                    (false, true) => return Ordering::Greater,
                    _ => match a.cmp(b) {
                        Ordering::Equal => at += 1,
                        decided => return decided,
                    },
                }
            }
        }
    }
}

/// **G13: finiteness is a visited set, not a depth limit.** A file already
/// read in this sitting is not walked again, keyed by its resolved machine
/// location; a chain that comes back round stops there. No depth cap is
/// imposed — a chain is recorded to at least nineteen libraries deep.
#[derive(Debug, Default)]
pub struct Visited {
    read: HashSet<MachineLocation>,
}

impl Visited {
    pub fn new() -> Visited {
        Visited::default()
    }

    /// Whether this is the first time the sitting meets the file. **Nothing
    /// about depth is asked.**
    pub fn first_time(&mut self, file: &MachineLocation) -> bool {
        self.read.insert(file.clone())
    }
}

/// **G12: capitals decide the library case differently at the two ends and
/// the answer leans on neither** — the mirror and the remap write exact
/// stored bytes (4.3, 5.2), so neither end's folding is ever relied on.
pub const NEITHER_ENDS_FOLDING_IS_RELIED_ON: bool = true;

/// **G16 and G17: the sitting is durable as it happens, and nothing carries on
/// by itself when the connection comes back** — a fresh check starts from
/// what is already known: the record supplies every located answer by K3's
/// keying and the reading re-reads every file (K11).
pub const A_FRESH_CHECK_RE_READS_EVERY_FILE_AND_RE_DECIDES_NOTHING: bool = true;

/// **G18: a check sitting is partial across files; one reading is atomic.**
/// Declared in Section 10 ([`crate::units`]).
pub const ONE_READING_IS_ATOMIC: bool = true;

/// **G19: scene attribution is computed inside the same reading, by walking
/// the record of what uses what until a scene appears** — there is no single
/// call that does it for anything but an object, in any version.
pub const ATTRIBUTION_IS_A_WALK_INSIDE_THE_SAME_READING: bool = true;

/// **G25 and G27: the recorded location pointing into the Blender application
/// the file was saved with is not a row, and the recognition mark is the
/// weak reference — a presence test rather than a string test.** The path
/// appears in the flat list and **no datablock in any path-carrying collection
/// holds it**; a user's own reference always holds one. Never a match against
/// a folder name, an application path or a version string. Observed in 4.5.12
/// and 5.2.0 (Fact 100).
pub fn is_the_applications_own_recorded_location(
    weak: &WeakReference,
    document: &ReadingDocument,
) -> bool {
    let in_the_flat_list = document.flat_list.contains(&weak.filepath);
    let held_by_a_datablock = document
        .references
        .iter()
        .any(|reference| reference.recorded == weak.filepath);
    in_the_flat_list && !held_by_a_datablock
}

/// **G27: the exclusion removes every occurrence rather than one** — two were
/// present in both versions, one per datablock, both naming the same file, so
/// a per-datablock reader sees two. What is excluded does not count against
/// the check leaving nothing unfound (G25).
pub fn the_applications_own_entries(document: &ReadingDocument) -> Vec<WeakReference> {
    document
        .weak_references
        .iter()
        .filter(|weak| is_the_applications_own_recorded_location(weak, document))
        .cloned()
        .collect()
}

/// **The document as the reading wrote it**, which carries one list the rest
/// of Smoozy never sees: G27a's answer, the recorded locations of the
/// libraries the Blender reading the file holds at the same place under its
/// own `datafiles/assets/`. **A document from before the reading answered
/// that reads as an empty list**, so every library in it stays a row.
#[derive(Deserialize)]
struct TheDocumentAsTheReadingWroteIt {
    references: Vec<ReadReference>,
    flat_list: Vec<RecordedLocation>,
    weak_references: Vec<WeakReference>,
    observed_stand_ins: Vec<RecordedLocation>,
    #[serde(default)]
    shipped_with_this_blender: Vec<RecordedLocation>,
}

impl From<TheDocumentAsTheReadingWroteIt> for ReadingDocument {
    fn from(wrote: TheDocumentAsTheReadingWroteIt) -> ReadingDocument {
        let TheDocumentAsTheReadingWroteIt {
            references,
            flat_list,
            weak_references,
            observed_stand_ins,
            shipped_with_this_blender,
        } = wrote;
        ReadingDocument {
            references: references
                .into_iter()
                .filter(|reference| {
                    !is_a_file_the_blender_reading_it_ships(reference, &shipped_with_this_blender)
                })
                .collect(),
            flat_list,
            weak_references,
            observed_stand_ins,
        }
    }
}

/// **G27a: a linked file Blender ships is not a row either** — G25's case,
/// not counted against the check leaving nothing unfound, and **the user sees
/// nothing**. Creator-stated, 2026-10-04: *"this one just comes packed inside
/// Blender itself, there's no need to flag it."*
///
/// **Both halves, and either alone is not enough.** The reference is a
/// **library** whose recorded location runs through a `datafiles/assets/`
/// folder ([`runs_through_an_assets_folder`]), **and** the reading on the
/// server answered that its own Blender holds a file at the same remainder
/// under its own `datafiles/assets/` — a presence test, so like G27 it is
/// never a list of names, an application path or a version string matched on
/// its own. **A user's own library of the same name anywhere else is a row**,
/// and so is the same location where the server's Blender holds no such file.
///
/// **What it does not establish** is whether the Blender rendering the job
/// then uses its own copy: §12 item 27, permanently unresolved.
pub fn is_a_file_the_blender_reading_it_ships(
    reference: &ReadReference,
    shipped_with_this_blender: &[RecordedLocation],
) -> bool {
    reference.kind == Kind::Library
        && runs_through_an_assets_folder(&reference.recorded)
        && shipped_with_this_blender.contains(&reference.recorded)
}

/// **G27a's first half, on the recorded location alone**: its last
/// `datafiles` step followed by `assets` leaves a remainder of at least one
/// step, none of them empty, `.` or `..`. **Both separators are read**,
/// because a file saved on Windows records its own; this is a recognition and
/// never a resolution, so the crate's one-separator rule for resolving is not
/// what it leans on. The reading's script walks the same steps the same way.
pub fn runs_through_an_assets_folder(recorded: &RecordedLocation) -> bool {
    let steps: Vec<&[u8]> = recorded
        .as_bytes()
        .split(|byte| *byte == b'/' || *byte == b'\\')
        .collect();
    let Some(at) = (0..steps.len().saturating_sub(1))
        .rev()
        .find(|at| steps[*at] == b"datafiles" && steps[*at + 1] == b"assets")
    else {
        return false;
    };
    let rest = &steps[at + 2..];
    !rest.is_empty()
        && rest
            .iter()
            .all(|step| !matches!(*step, b"" | b"." | b".."))
}

/// **G26: what the evidence gives recognition to work with** — the entry sits
/// inside the very lists this reading takes, in every version, and Blender's
/// own missing-file accounting excludes it. Blender's own report is disproved
/// as a route: it printed `No missing files` for a path that would exist on
/// no other machine (G27).
///
/// **M21: Blender's own search and repair stand behind nothing here, for the
/// four recorded reasons** — it matches capitals exactly where a proposal is
/// needed; it takes the largest file on disk and says nothing about there
/// having been a choice; its wider form silently repoints references that were
/// not broken; and the form it writes a repaired location back in is not
/// established (§12 item 25, permanently unresolved). **This constant is that
/// clause held at the check end**: no route anywhere in this crate consumes
/// Blender's own report or its repair, and the standing instruction M21 leaves
/// is that Smoozy writes the location it means and reads it back rather than
/// assuming its form.
pub const BLENDERS_OWN_MISSING_FILE_REPORT_STANDS_BEHIND_NOTHING: bool = true;

#[cfg(test)]
pub mod tests {
    use super::*;
    use smoozy_render_settings::version::{HowItMatched, PickedBuild};
    use smoozy_server_setup::blender::Version;

    pub fn a_blender() -> BlenderIdentity {
        BlenderIdentity {
            path: "/root/blender/blender-4.5.12/blender".to_string(),
            version: Version {
                major: 4,
                minor: 5,
                patch: 12,
            },
        }
    }

    fn recorded(bytes: &[u8]) -> RecordedLocation {
        RecordedLocation::as_the_reading_carried_it(bytes)
    }

    fn a_reference(kind: Kind, datablock: &str, recorded_bytes: &[u8]) -> ReadReference {
        ReadReference {
            kind,
            datablock: datablock.to_string(),
            setting: "filepath".to_string(),
            recorded: recorded(recorded_bytes),
            scenes: Attribution::NotEstablished,
            source: None,
        }
    }

    #[test]
    fn the_version_is_render_settings_resolution_and_no_version_means_no_check() {
        let picked = Resolution::Automatic {
            build: PickedBuild {
                path: a_blender().path,
                answered: a_blender().version,
            },
            how: HowItMatched::TheExactLine,
        };
        assert_eq!(the_version_to_read_with(&picked), Ok(a_blender()));
        assert_eq!(
            the_version_to_read_with(&Resolution::TheUsersOverride(a_blender())),
            Ok(a_blender())
        );
        assert_eq!(
            the_version_to_read_with(&Resolution::NoVersionResolves(WhyNone::TheListIsEmpty)),
            Err(NoCheckRuns::NoVersionResolves(WhyNone::TheListIsEmpty))
        );
    }

    #[test]
    fn no_document_and_a_failing_status_are_could_not_be_read_and_an_empty_document_is_a_reading() {
        let with = a_blender();
        assert_eq!(
            judge(WhatTheReadingLeft::NoSentinelFramedDocument, &with),
            Reading::CouldNotBeRead { with: with.clone() }
        );
        assert_eq!(
            judge(WhatTheReadingLeft::AFailingStatus, &with),
            Reading::CouldNotBeRead { with: with.clone() }
        );
        let empty = ReadingDocument {
            references: vec![],
            flat_list: vec![],
            weak_references: vec![],
            observed_stand_ins: vec![],
        };
        assert_eq!(
            judge(
                WhatTheReadingLeft::ASentinelFramedDocument(empty.clone()),
                &with
            ),
            Reading::Read(empty)
        );
    }

    #[test]
    fn forms_are_classified_and_only_ordinary_and_set_forms_are_searched() {
        assert_eq!(
            classify(&recorded(b"//tex/x.png"), StoredSource::NotSaidToBeOneFile),
            Form::Ordinary
        );
        assert_eq!(
            classify(
                &recorded(b"//tex/tile.<UDIM>.png"),
                StoredSource::NotSaidToBeOneFile
            ),
            Form::ASet(SetMarks {
                frame_number_characters: false,
                udim: true,
                uvtile: false,
                view_name: false,
            })
        );
        assert_eq!(
            classify(
                &recorded(b"//seq/frame_####.png"),
                StoredSource::NotSaidToBeOneFile
            ),
            Form::ASet(SetMarks {
                frame_number_characters: true,
                udim: false,
                uvtile: false,
                view_name: false,
            })
        );
        assert_eq!(
            classify(&recorded(b"~/tex/x.png"), StoredSource::NotSaidToBeOneFile),
            Form::ATildeOrAnEnvironmentVariable
        );
        assert_eq!(
            classify(&recorded(b"$TEX/x.png"), StoredSource::NotSaidToBeOneFile),
            Form::ATildeOrAnEnvironmentVariable
        );
        assert!(
            classify(&recorded(b"//tex/x.png"), StoredSource::NotSaidToBeOneFile).is_searched()
        );
        assert!(
            !classify(&recorded(b"~/tex/x.png"), StoredSource::NotSaidToBeOneFile).is_searched()
        );
    }

    #[test]
    fn a_folder_named_with_curly_braces_is_an_ordinary_location_and_is_searched() {
        // G9 allows a curly-brace expression only in the render output path
        // and the File Output node, both write-only destinations that produce
        // no reference — so a `{` in a reference is a character in a folder's
        // name, on a Mac and on the rented Linux machine alike.
        assert_eq!(
            classify(
                &recorded(b"//{hero}/tex/x.png"),
                StoredSource::NotSaidToBeOneFile
            ),
            Form::Ordinary
        );
        assert_eq!(
            classify(
                &recorded(b"/Users/a/{hero}/x.png"),
                StoredSource::NotSaidToBeOneFile
            ),
            Form::Ordinary
        );
        assert!(classify(
            &recorded(b"//{hero}/tex/x.png"),
            StoredSource::NotSaidToBeOneFile
        )
        .is_searched());
        // And a set inside one is still a set.
        assert_eq!(
            classify(
                &recorded(b"//{hero}/seq/frame_####.png"),
                StoredSource::NotSaidToBeOneFile
            ),
            Form::ASet(SetMarks {
                frame_number_characters: true,
                udim: false,
                uvtile: false,
                view_name: false,
            })
        );
    }

    #[test]
    fn the_folder_shaped_kinds_are_g6s_and_the_bake_output_path_is_one_of_them() {
        for kind in [
            Kind::PhysicsPointCache,
            Kind::FluidCacheFolder,
            Kind::GeometryNodeBakeFolder,
            Kind::BakeFolder,
            Kind::OceanBakeFolder,
            Kind::DynamicPaintOutputFolder,
            Kind::SequencerStripFolder,
            Kind::ProxyFolder,
        ] {
            assert!(kind.names_a_folder(), "{kind:?}");
        }
        for kind in [
            Kind::Image,
            Kind::Library,
            Kind::Sound,
            Kind::SequencerStripFile,
            Kind::CacheArchive,
            Kind::ImportNodePath,
        ] {
            assert!(!kind.names_a_folder(), "{kind:?}");
        }
    }

    #[test]
    fn a_set_member_matches_by_the_patterns_own_form_and_the_padding_is_a_floor() {
        assert!(set_member_matches(b"frame_####.png", b"frame_0001.png"));
        assert!(set_member_matches(b"frame_####.png", b"frame_10001.png"));
        assert!(!set_member_matches(b"frame_####.png", b"frame_001.png"));
        assert!(!set_member_matches(b"frame_####.png", b"frame_0001.jpg"));
        assert!(set_member_matches(b"tile.<UDIM>.png", b"tile.1001.png"));
        assert!(set_member_matches(b"tile.<UVTILE>.png", b"tile.7.png"));
        assert!(!set_member_matches(b"tile.<UDIM>.png", b"tile.png"));
        assert!(set_member_matches(b"shot_%.png", b"shot_left.png"));
        assert!(!set_member_matches(b"shot_%.png", b"shot_.png"));
        assert!(!set_member_matches(b"shot_%.png", b"shot_a/b.png"));
    }

    #[test]
    fn references_group_by_tidied_text_before_resolution_and_by_the_file_after() {
        let holder = b"/Users/a/proj".to_vec();
        let before = vec![
            ReferenceToGroup {
                kind: Kind::Image,
                recorded: recorded(b"//tex/x.png"),
                holder_folder: holder.clone(),
                resolved: None,
            },
            ReferenceToGroup {
                kind: Kind::Image,
                recorded: recorded(b"/Users/a/proj/tex/x.png"),
                holder_folder: holder.clone(),
                resolved: None,
            },
            ReferenceToGroup {
                kind: Kind::Image,
                recorded: recorded(b"//tex/../tex/x.png"),
                holder_folder: holder.clone(),
                resolved: None,
            },
            // A different spelling on disk is a different text: the tidying
            // stops at the text.
            ReferenceToGroup {
                kind: Kind::Image,
                recorded: recorded(b"//tex/X.png"),
                holder_folder: holder.clone(),
                resolved: None,
            },
        ];
        let rows = group_into_rows(&before);
        assert_eq!(rows.len(), 2);
        let big = rows
            .iter()
            .find(|row| row.references.len() == 3)
            .expect("three grouped");
        assert!(big.more_than_one_reference_led_to_it());
        assert_eq!(
            big.grouped_by,
            GroupedBy::TidiedText(b"/Users/a/proj/tex/x.png".to_vec())
        );

        // After resolution, by the resolved file — a name match alone never
        // groups.
        let file = StoredLocation::as_the_system_returned_it(b"/Volumes/T/tex/x.png");
        let other = StoredLocation::as_the_system_returned_it(b"/Volumes/U/tex/x.png");
        let after = vec![
            ReferenceToGroup {
                kind: Kind::Image,
                recorded: recorded(b"//tex/x.png"),
                holder_folder: holder.clone(),
                resolved: Some(file.clone()),
            },
            ReferenceToGroup {
                kind: Kind::Image,
                recorded: recorded(b"//old/x.png"),
                holder_folder: holder.clone(),
                resolved: Some(file),
            },
            ReferenceToGroup {
                kind: Kind::Image,
                recorded: recorded(b"//other/x.png"),
                holder_folder: holder,
                resolved: Some(other),
            },
        ];
        let rows = group_into_rows(&after);
        assert_eq!(rows.len(), 2);
        assert_eq!(
            rows.iter().filter(|row| row.references.len() == 2).count(),
            1
        );
    }

    #[test]
    fn the_ordering_keeps_a_folders_own_files_before_its_subfolders_blocks() {
        let mut rows = [
            recorded(b"//tex/wood/plank.png"),
            recorded(b"//tex/zebra.png"),
            recorded(b"//tex/apple.png"),
            recorded(b"//tex/brick/red.png"),
            recorded(b"//models/chair.blend"),
        ];
        rows.sort_by(compare_recorded);
        let bytes: Vec<&[u8]> = rows.iter().map(|row| row.as_bytes()).collect();
        assert_eq!(
            bytes,
            vec![
                &b"//models/chair.blend"[..],
                b"//tex/apple.png",
                b"//tex/zebra.png",
                b"//tex/brick/red.png",
                b"//tex/wood/plank.png",
            ]
        );
        // A value with no separator is one segment and compares whole.
        assert_eq!(compare_paths(b"x.png", b"x.png"), Ordering::Equal);
        assert_eq!(compare_paths(b"a.png", b"b/c.png"), Ordering::Less);
    }

    #[test]
    fn a_many_reference_rows_key_is_the_least_of_its_references() {
        let row = Row {
            references: vec![recorded(b"//tex/z.png"), recorded(b"//tex/a.png")],
            grouped_by: GroupedBy::TidiedText(vec![]),
        };
        assert_eq!(row.ordering_key().as_bytes(), b"//tex/a.png");
    }

    #[test]
    fn the_visited_set_stops_a_chain_that_comes_back_round_and_caps_no_depth() {
        let mut visited = Visited::new();
        let lib = MachineLocation::composed(b"/root/assets/files/lib.blend".to_vec());
        assert!(visited.first_time(&lib));
        assert!(!visited.first_time(&lib));
        for depth in 0..40 {
            assert!(visited.first_time(&MachineLocation::composed(
                format!("/root/assets/files/l{depth}.blend").into_bytes()
            )));
        }
    }

    #[test]
    fn the_applications_own_entry_is_recognised_by_presence_and_every_occurrence_is_excluded() {
        let brushes = recorded(b"/Applications/Blender.app/Contents/Resources/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend");
        let users = recorded(b"//lib/chairs.blend");
        let document = ReadingDocument {
            references: vec![a_reference(Kind::Library, "LIchairs", users.as_bytes())],
            flat_list: vec![brushes.clone(), brushes.clone(), users.clone()],
            weak_references: vec![
                WeakReference {
                    filepath: brushes.clone(),
                    id_name: "MADots Stroke".to_string(),
                },
                WeakReference {
                    filepath: brushes.clone(),
                    id_name: "MAMaterial".to_string(),
                },
                // A weak reference whose path a datablock also holds is the
                // user's own and is never excluded.
                WeakReference {
                    filepath: users.clone(),
                    id_name: "OBChair".to_string(),
                },
            ],
            observed_stand_ins: vec![],
        };
        let excluded = the_applications_own_entries(&document);
        assert_eq!(excluded.len(), 2);
        assert!(excluded.iter().all(|weak| weak.filepath == brushes));
    }

    /// A document as the reading's script writes it, one library in it, with
    /// G27a's answer beside it or not.
    fn a_written_document(library: &[u8], kind: &str, shipped: Option<&[u8]>) -> String {
        let bytes = |one: &[u8]| serde_json::to_string(one).expect("it renders");
        let shipped = match shipped {
            Some(one) => format!(r#", "shipped_with_this_blender": [{}]"#, bytes(one)),
            None => String::new(),
        };
        format!(
            r#"{{"references": [{{"kind": "{kind}", "datablock": "a", "setting": "filepath", "recorded": {}, "scenes": "NotEstablished"}}], "flat_list": [], "weak_references": [], "observed_stand_ins": []{shipped}}}"#,
            bytes(library)
        )
    }

    fn the_rows_of(written: &str) -> Vec<Row> {
        let document: ReadingDocument = serde_json::from_str(written).expect("the document reads");
        let references: Vec<ReferenceToGroup> = document
            .references
            .iter()
            .map(|one| ReferenceToGroup {
                kind: one.kind,
                recorded: one.recorded.clone(),
                holder_folder: b"/Volumes/Work/Project/Scenes".to_vec(),
                resolved: None,
            })
            .collect();
        group_into_rows(&references)
    }

    const THE_ESSENTIALS_AS_THE_CREATORS_FILE_RECORDS_IT: &[u8] =
        b"//../../../Applications/Blender.app/Contents/Resources/5.1/datafiles/assets/nodes/geometry_nodes_essentials.blend";

    /// **G27a, the map's first test**: the Creator's own case, a library
    /// recorded into the Mac's Blender, where the server's Blender holds
    /// `nodes/geometry_nodes_essentials.blend` under its own — not a row.
    #[test]
    fn a_library_the_servers_blender_ships_is_not_a_row() {
        let written = a_written_document(
            THE_ESSENTIALS_AS_THE_CREATORS_FILE_RECORDS_IT,
            "Library",
            Some(THE_ESSENTIALS_AS_THE_CREATORS_FILE_RECORDS_IT),
        );
        assert!(the_rows_of(&written).is_empty());
        // **Written on Windows**, the same.
        let windows: &[u8] = b"C:\\Program Files\\Blender Foundation\\Blender 5.1\\5.1\\datafiles\\assets\\nodes\\geometry_nodes_essentials.blend";
        assert!(the_rows_of(&a_written_document(windows, "Library", Some(windows))).is_empty());
    }

    /// **G27a, the map's second test**: the same location where the server's
    /// Blender holds no such file is a row — and so is every library in a
    /// document from before the reading answered.
    #[test]
    fn the_same_library_where_the_servers_blender_holds_none_is_a_row() {
        let written = a_written_document(
            THE_ESSENTIALS_AS_THE_CREATORS_FILE_RECORDS_IT,
            "Library",
            None,
        );
        assert_eq!(the_rows_of(&written).len(), 1);
    }

    /// **G27a, the map's third test**: a user's own library of the same name
    /// anywhere else is a row, even where a reading named it — the reading
    /// answers presence, and the location must run through the folder too.
    /// **And only a library**: a picture there is a row as before.
    #[test]
    fn a_users_own_library_of_the_same_name_is_a_row() {
        let users: &[u8] = b"//lib/geometry_nodes_essentials.blend";
        assert_eq!(the_rows_of(&a_written_document(users, "Library", Some(users))).len(), 1);
        let picture: &[u8] = b"//../Blender/5.1/datafiles/assets/nodes/x.png";
        assert_eq!(the_rows_of(&a_written_document(picture, "Image", Some(picture))).len(), 1);
    }

    /// **G27a's first half walks steps, not text**: the last `datafiles` then
    /// `assets` with something beneath, never an empty, `.` or `..` step.
    #[test]
    fn running_through_an_assets_folder_is_a_walk_over_steps() {
        let runs = |bytes: &[u8]| runs_through_an_assets_folder(&recorded(bytes));
        assert!(runs(THE_ESSENTIALS_AS_THE_CREATORS_FILE_RECORDS_IT));
        assert!(runs(b"/opt/blender/5.2/datafiles/assets/nodes/a.blend"));
        assert!(runs(b"D:\\b\\datafiles\\assets\\nodes\\a.blend"));
        assert!(!runs(b"/opt/blender/5.2/datafiles/assets/"));
        assert!(!runs(b"/opt/blender/5.2/datafiles/assets/../../x.blend"));
        assert!(!runs(b"/opt/blender/5.2/datafiles/assets//x.blend"));
        assert!(!runs(b"/opt/mydatafiles/assets/x.blend"));
        assert!(!runs(b"/opt/datafiles/assetsx/x.blend"));
        assert!(!runs(b"//lib/geometry_nodes_essentials.blend"));
    }

    #[test]
    fn the_mark_is_never_a_string_test_on_the_path() {
        // A path that looks like an application's is a user's own reference
        // where a datablock holds it.
        let looks_like_the_app = recorded(b"/Applications/Blender.app/x.blend");
        let document = ReadingDocument {
            references: vec![a_reference(
                Kind::Library,
                "LIx",
                looks_like_the_app.as_bytes(),
            )],
            flat_list: vec![looks_like_the_app.clone()],
            weak_references: vec![WeakReference {
                filepath: looks_like_the_app,
                id_name: "MAx".to_string(),
            }],
            observed_stand_ins: vec![],
        };
        assert!(the_applications_own_entries(&document).is_empty());
    }

    #[test]
    fn a_linked_item_statement_names_the_library_and_comes_only_from_an_observation() {
        let library = recorded(b"//lib/chairs.blend");
        let mut document = ReadingDocument {
            references: vec![],
            flat_list: vec![],
            weak_references: vec![],
            observed_stand_ins: vec![],
        };
        assert!(linked_item_statements(&document).is_empty());
        document.observed_stand_ins.push(library.clone());
        assert_eq!(
            linked_item_statements(&document),
            vec![LinkedItemStatement { library }]
        );
    }

    #[test]
    fn write_places_that_are_also_read_are_rows_and_write_only_destinations_are_not() {
        assert!(Kind::PhysicsPointCache.is_a_write_place_that_is_also_read());
        assert!(!Kind::Image.is_a_write_place_that_is_also_read());
        assert!(!WriteOnlyDestination::RenderOutput.is_a_row());
        assert!(!WriteOnlyDestination::FileOutputNode.is_a_row());
        assert!(!WriteOnlyDestination::CollectionExporter.is_a_row());
        // G6: the bake output path is a row, and is a kind rather than a
        // write-only destination.
        assert!(Kind::BakeFolder.is_a_write_place_that_is_also_read());
        assert!(Kind::ALL.contains(&Kind::BakeFolder));
    }

    /// **DW0-3, the fault it was written for**: a still picture whose name
    /// holds a `#` sat exactly where its `.blend` said and was *not found*,
    /// because it was looked for as a folder of frames. Stored as one file,
    /// it is an ordinary row; stored as a sequence or as tiles, the marks
    /// still make a set.
    #[test]
    fn a_picture_stored_as_one_file_is_ordinary_whatever_its_name_holds() {
        let still = recorded("//../Текстуры/лев's база #2.jpg".as_bytes());
        let one_file = StoredSource::of(Kind::Image, Some("FILE"));
        assert_eq!(one_file, StoredSource::OneFile);
        assert_eq!(classify(&still, one_file), Form::Ordinary);
        assert_eq!(
            classify(&recorded(b"//tex/tile.<UDIM>.png"), one_file),
            Form::Ordinary
        );
        assert_eq!(
            classify(
                &recorded(b"//clips/take_%.mov"),
                StoredSource::of(Kind::MovieClip, Some("MOVIE"))
            ),
            Form::Ordinary
        );
        // Stored as a set, or not said to be one file: G9 as written.
        for (kind, source) in [
            (Kind::Image, Some("SEQUENCE")),
            (Kind::Image, Some("TILED")),
            (Kind::Image, None),
            (Kind::Image, Some("SOMETHING_NEW")),
            (Kind::MovieClip, Some("SEQUENCE")),
            (Kind::Volume, Some("FILE")),
            (Kind::PhysicsPointCache, None),
        ] {
            let stored = StoredSource::of(kind, source);
            assert_eq!(
                stored,
                StoredSource::NotSaidToBeOneFile,
                "{kind:?} {source:?}"
            );
            assert!(
                matches!(classify(&still, stored), Form::ASet(_)),
                "{kind:?} {source:?}"
            );
        }
        // The tilde and environment-variable forms come first, stored as one
        // file or not.
        assert_eq!(
            classify(&recorded(b"~/tex/x #1.png"), one_file),
            Form::ATildeOrAnEnvironmentVariable
        );
    }

    /// **The reading carries the source for a picture, and a document from
    /// before it did reads as no source** — G9's rule as written.
    #[test]
    fn the_reading_carries_a_pictures_source_and_its_absence_reads_as_none() {
        let with: ReadReference = serde_json::from_str(
            r#"{"kind": "Image", "datablock": "a", "setting": "filepath", "recorded": [47, 47, 35], "scenes": "NotEstablished", "source": "FILE"}"#,
        )
        .expect("a reference with a source reads");
        assert_eq!(with.stored_source(), StoredSource::OneFile);
        let without: ReadReference = serde_json::from_str(
            r#"{"kind": "Image", "datablock": "a", "setting": "filepath", "recorded": [47, 47, 35], "scenes": "NotEstablished"}"#,
        )
        .expect("a reference with no source reads");
        assert_eq!(without.source, None);
        assert_eq!(without.stored_source(), StoredSource::NotSaidToBeOneFile);
    }

    fn a_bake_output(
        recorded_bytes: &[u8],
        holder: &[u8],
        resolved: Option<&[u8]>,
    ) -> ReferenceToGroup {
        ReferenceToGroup {
            kind: Kind::BakeFolder,
            recorded: recorded(recorded_bytes),
            holder_folder: holder.to_vec(),
            resolved: resolved.map(StoredLocation::as_the_system_returned_it),
        }
    }

    /// **DW0-2, the fault it was written for**: every new scene carries `//`
    /// as its bake output path, and the check found the `.blend`'s own folder
    /// and would have sent the whole of it.
    #[test]
    fn a_relative_bake_output_path_at_or_above_the_blends_folder_makes_no_row() {
        let holder = b"/Users/a/proj/shots";
        for at_or_above in [
            &b"//"[..],
            b"//.",
            b"//./",
            b"//..",
            b"//../..",
            b"//bake/..",
            b"//bake/../../",
        ] {
            assert!(
                the_bake_output_path_makes_no_row(&a_bake_output(at_or_above, holder, None)),
                "{}",
                String::from_utf8_lossy(at_or_above)
            );
        }
        for below_or_beside in [&b"//bake/"[..], b"//bake", b"//../bake/", b"//./bake/x/.."] {
            assert!(
                !the_bake_output_path_makes_no_row(&a_bake_output(below_or_beside, holder, None)),
                "{}",
                String::from_utf8_lossy(below_or_beside)
            );
        }
    }

    #[test]
    fn a_full_bake_output_path_is_judged_on_where_it_resolved() {
        let holder = b"/Users/a/proj/shots";
        // The folder itself, and every folder above it.
        for resolved in [
            &b"/Users/a/proj/shots"[..],
            b"/Users/a/proj/shots/",
            b"/Users/a/proj",
            b"/Users",
            b"/",
        ] {
            assert!(
                the_bake_output_path_makes_no_row(&a_bake_output(
                    b"/whatever/it/says/",
                    holder,
                    Some(resolved)
                )),
                "{}",
                String::from_utf8_lossy(resolved)
            );
        }
        // Below it, beside it, a name that only starts the same, or not
        // resolved at all: a row as before.
        for resolved in [
            &b"/Users/a/proj/shots/bake"[..],
            b"/Users/a/proj/other",
            b"/Users/a/pro",
        ] {
            assert!(!the_bake_output_path_makes_no_row(&a_bake_output(
                b"/whatever/",
                holder,
                Some(resolved)
            )));
        }
        assert!(!the_bake_output_path_makes_no_row(&a_bake_output(
            b"/Users/a/proj/shots/",
            holder,
            None
        )));
    }

    #[test]
    fn only_the_bake_output_path_is_ever_dropped_and_the_row_is_dropped_where_rows_are_composed() {
        let holder = b"/Users/a/proj".to_vec();
        let mut references = vec![a_bake_output(b"//", &holder, Some(b"/Users/a/proj"))];
        // Every other folder-shaped kind at the same place is still a row.
        for kind in [
            Kind::GeometryNodeBakeFolder,
            Kind::FluidCacheFolder,
            Kind::PhysicsPointCache,
        ] {
            references.push(ReferenceToGroup {
                kind,
                recorded: recorded(b"//"),
                holder_folder: holder.clone(),
                resolved: Some(StoredLocation::as_the_system_returned_it(b"/Users/a/proj")),
            });
        }
        references.push(a_bake_output(b"//bake/", &holder, None));
        let rows = group_into_rows(&references);
        assert_eq!(rows.len(), 2, "{rows:#?}");
        assert!(rows.iter().any(|row| row.references.len() == 3));
        assert!(rows
            .iter()
            .any(|row| row.references == vec![recorded(b"//bake/")]));
    }
}
