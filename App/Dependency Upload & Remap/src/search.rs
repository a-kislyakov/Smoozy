//! Finding the files — `Dependency Upload & Remap` §4.1 to §4.3, Rows 165,
//! 166 and 330: the folder list, the walk, the exact match and the near-miss.
//!
//! **Where the line falls between the two halves.** The plan's own word for
//! the walk is F8's: *"the walk enumerates; it never asks the disk whether two
//! names are the same."* An enumeration is a reading, and every reading is
//! behind [`Readings`] — so the walk's **rules** (the user's order, the whole
//! subtree, every match collected, a folder failing alone, a stop keeping
//! what it has) run here over that trait, and the **opening of folders** is
//! the acting half's implementation of it. The matching rules — F9's byte
//! equality, F10's fold, F14a's exactness by the normalized name — are
//! judgement and are here entire.
//!
//! **What breaks if this is got wrong, in the spec's own words:** *"Smoozy
//! would send the wrong file, point the reference at where that file landed,
//! and the render would find it and use it. A wrong picture, at full price,
//! with nothing missing and nothing to see."*
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Desktop app and delivery` C15, C16, F10a.
//! `State, identity and recovery` §9.4, D5.

use std::collections::HashMap;
use std::time::SystemTime;

use smoozy_state::lifetimes::Lifetime;
use smoozy_state::observed::{Clock, Reading};

use crate::bytes::{join, RecordedLocation, StoredLocation};
use crate::check::{classify, set_member_matches, Form, Kind, StoredSource};
use crate::readings::{EnumerationCasing, FolderListing, Readings, StoredNameAnswer};
use crate::record::{ArbitraryPick, HowResolved, Resolution, ResolvedTo};

/// **F1: the searched-folder list is one app-wide ordered list in the local
/// record, user-held.** It describes the user's own computer, so it is kept
/// once for the whole app and never per server; a new rented machine inherits
/// it with nothing re-added. Managing it is Design's; the order is the user's.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SearchedFolderList {
    pub folders: Vec<Vec<u8>>,
}

pub const THE_LIST_IS_USER_HELD: Lifetime = Lifetime::UserHeld;

/// **F2: a folder covers everything inside it, to any depth.**
pub const A_FOLDER_COVERS_ITS_WHOLE_SUBTREE: bool = true;

/// **F7: the search's outcome per listed folder** — searched whole, stopped
/// part-way, could not be read. **The verdict does not separate missing,
/// moved and refused** (F1a): a word that claims absence claims more than the
/// verdict holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FolderVerdict {
    SearchedWhole,
    StoppedPartWay,
    CouldNotBeRead,
}

/// **F1a: the list carries no mark of its own, and the *could not be read*
/// verdict is one search's or nothing.** A verdict is drawn where a search in
/// this run produced one, with nothing where none has, and it carries the
/// moment of the search it came from (`State, identity and recovery` D5),
/// because it is an observed reading and not a property of the row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PerFolderOutcome {
    pub folder: Vec<u8>,
    pub verdict: FolderVerdict,
    pub at: Reading<()>,
}

/// **F7 and F1a: the per-folder list is run-held by decision**, not an outcome
/// record under `State, identity and recovery` §9.4. A stopped search's report
/// does not survive the application run and the next search re-derives it by
/// walking again; a located answer is written at the moment it lands (K4) and
/// survives, and a *not found* row is the **absence** of a located answer
/// rather than a statement about the disk.
pub const THE_PER_FOLDER_OUTCOME_IS_RUN_HELD: Lifetime = Lifetime::RunHeld;

/// **F6 and F7: the search's own report** — whether it finished, each listed
/// folder's verdict, and every unreadable subfolder named.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SearchReport {
    pub finished: bool,
    pub per_folder: Vec<PerFolderOutcome>,
    pub unreadable_subfolders: Vec<Vec<u8>>,
    /// **F7: an entry that would not read is skipped and named beside the
    /// unreadable subfolders**, and the folder carries on being searched. The
    /// entry itself has no name to give — the listing failed before it had
    /// one — so what is named is the folder and how many of its entries would
    /// not read.
    pub folders_with_entries_that_would_not_read: Vec<(Vec<u8>, usize)>,
}

/// **F9 and F10's verdict on one enumerated name against the name a reference
/// wants**, decided under F14a's arrangement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CandidateVerdict {
    /// **F9: an equality of bytes, capitals included, whole and exact.**
    Exact,
    /// **F10: equal under case folding alone** — surfaced as *this is what
    /// was asked for, this is what is there, is it the same file*, and the
    /// user accepts it or does not. The fold decides only what is offered;
    /// it never decides what is found.
    AProposal,
    NotAMatch,
    /// **F14a's bound and §12 item 3's silence, removed at the one place it
    /// could mislead**: the enumeration's casing is not established on this
    /// filesystem and the stored-name reading refused the class, so the
    /// search **cannot separate an exact match from a case near-miss**. It
    /// reports that limit instead of a false exact match — no match, no
    /// proposal, and the row carries neither.
    TheSystemCannotSayWhichItIs,
}

/// **F9: a match is an equality of bytes.** No folding, no normalising and no
/// canonicalising sits anywhere in the match path.
pub fn is_an_exact_match(enumerated_stored_name: &[u8], wanted_name: &[u8]) -> bool {
    enumerated_stored_name == wanted_name
}

/// **F10's fold — Smoozy's own, chosen as Unicode full folding because that is
/// what the one measured disk does** (macOS folding predicted all fifteen
/// observed pairs where simple folding mispredicts three, so `ß` and `ss` are
/// one file — Fact 16).
///
/// **How it is computed here, and what that is worth.** The fold is the
/// upper-case mapping followed by the lower-case mapping, from the standard
/// library's own Unicode tables — which carries the full-folding cases the
/// one measured disk showed (`ß` reaches `ss`) without a folding table of this
/// crate's own. **A fold that deviates from some disk's own rule at an edge
/// code point costs a proposal, never a wrong file**, which is F10's own
/// sentence and the reason the exact table is not load-bearing. A name that
/// is not text folds to nothing, and two such names are compared on bytes
/// alone.
pub fn fold(name: &[u8]) -> Option<String> {
    let text = std::str::from_utf8(name).ok()?;
    Some(
        text.chars()
            .flat_map(char::to_uppercase)
            .flat_map(char::to_lowercase)
            .collect(),
    )
}

/// **F11: the proposal comparison is shaped so that one edit widens it to
/// normalisation-insensitive equality, and nothing is claimed for it now.**
/// This is that one place. A name that differs in normalisation rather than
/// case is a standing hazard the proposal does not cover (§12 item 10 — open
/// and cheap, owed to nobody, and not closed).
pub fn proposal_equality(one: &[u8], other: &[u8]) -> bool {
    match (fold(one), fold(other)) {
        (Some(a), Some(b)) => a == b,
        _ => one == other,
    }
}

/// **F14a's arrangement, on one candidate.** The walk enumerates as F8 says; a
/// candidate is taken by **fold-equality** against the wanted name, which is
/// insensitive to whatever casing the enumeration returned; and exactness is
/// decided by the enumerated bytes where the enumeration's casing is
/// established, and **only by the normalized name's last component** where it
/// is not.
///
/// **F8 claims nothing about casing, and that is why there is nothing here to
/// arbitrate.** F8 was narrowed on 2026-09-04 to what the walk actually owes
/// the clauses below — **candidates**, taken by fold-equality against the
/// wanted name, which does not care what casing the enumeration returned —
/// and its old general rule about every folding disk is struck in the
/// document. **What decides exactness is the stored-name reading of F12 and
/// F13, never the enumeration's own casing.** F14's denial is about a cited
/// route to the stored name on a given filesystem, which is a question about
/// a filesystem and not about enumeration, so the two are no longer opposite
/// readings of one thing. macOS is established by Fact 16 and a folding ext4
/// directory by the kernel's own documentation (F14a). Where the acting half
/// answers [`EnumerationCasing::IsTheStoredName`] — macOS, ext4, any
/// case-sensitive directory — F8's reading is what is built. Everywhere
/// else — a folding Linux filesystem that is not ext4, a Windows volume
/// outside NTFS and ReFS — the enumeration's casing is taken as not
/// established, exactness comes from the stored-name reading alone, and a
/// refusal is reported as the limit it is. Nobody has run either reading
/// (§12 item 21), and no space is left here for the result.
pub fn judge_candidate(
    wanted_name: &[u8],
    enumerated_name: &[u8],
    casing: EnumerationCasing,
    the_stored_name: impl FnOnce() -> StoredNameAnswer,
) -> CandidateVerdict {
    if !proposal_equality(wanted_name, enumerated_name) {
        return CandidateVerdict::NotAMatch;
    }
    match casing {
        EnumerationCasing::IsTheStoredName => {
            if is_an_exact_match(enumerated_name, wanted_name) {
                CandidateVerdict::Exact
            } else {
                CandidateVerdict::AProposal
            }
        }
        EnumerationCasing::NotEstablished => match the_stored_name() {
            StoredNameAnswer::TheStoredName(stored) => {
                if is_an_exact_match(stored.final_segment(), wanted_name) {
                    CandidateVerdict::Exact
                } else {
                    CandidateVerdict::AProposal
                }
            }
            StoredNameAnswer::TheClassIsRefused => CandidateVerdict::TheSystemCannotSayWhichItIs,
            StoredNameAnswer::NotThere => CandidateVerdict::NotAMatch,
        },
    }
}

/// **F13: the exact-case reading belongs in the platform file adapter**
/// (`Desktop app and delivery` C15's sending-end twin, its F10a) and this
/// document consumes a *stored-name reading* from it and does not build one.
/// A second implementation here would be the string-manipulation trap's
/// breeding ground. **F14: on Windows the stored name cannot be read by the
/// enumeration** — permanently unresolved with the rest of Windows — and
/// **F14a** gives NTFS and ReFS the normalized-name route (`[MS-FSCC]`
/// 2.4.35) and ext4 the kernel's own name-preserving citation
/// (`admin-guide/ext4`; `Desktop app and delivery` C16).
pub const THE_STORED_NAME_READING_IS_THE_ADAPTERS: bool = true;

/// **F15: Blender's exact-match strictness is Blender's own rule and not the
/// disk's — settled, and nothing here turns on it either way.** The repair
/// does not rescue a wrong spelling in either direction, which is the spec's
/// own closing of the question. Label: observed in a test (*Reading a file
/// 10*).
///
/// **M21, the first of its four reasons: Blender's own search matches capitals
/// exactly where a proposal is needed**, so it stands behind nothing here and
/// the search in this file is Smoozy's own entire. F10's fold turns a
/// case-only difference into a **proposal** rather than into a match, which is
/// the thing Blender's own search cannot do.
pub const NOTHING_HERE_TURNS_ON_BLENDERS_OWN_STRICTNESS: bool = true;

/// **F28: what shape of thing the search has to answer a reference with.**
/// F9 to F15 are written about a file's name; G6's folder-shaped kinds and
/// G24's sets need a directory, and the two need it for different reasons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WantedShape {
    /// A file, matched on its final segment (F9).
    AFile,
    /// **A folder-shaped reference** — G6's folder kinds — matched on the
    /// final *named* segment of its recorded location, any trailing separator
    /// dropped, against a directory entry's own name. F9's byte equality and
    /// F10's fold apply to it unchanged.
    AFolder,
    /// **A set row** (3.6, G22): the folder holding at least one entry
    /// matching the pattern. **The pattern's own characters are never a
    /// wanted name** — nothing on any disk is called `frame_####.png` — so
    /// this shape is never matched by a name at all.
    ASetsFolder,
}

/// One outstanding reference and the shape its answer has to take. **The
/// search takes these rather than bare recorded locations**, because the
/// kind is what decides the shape and a recorded location does not carry it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Outstanding {
    /// The reference as the file records it: the key of everything kept for it.
    pub reference: RecordedLocation,
    /// **G9a: what is looked for**, the same bytes as [`Self::reference`] but
    /// for a template put in ([`crate::check::how_the_render_reads_it`]).
    pub the_location_read: RecordedLocation,
    pub shape: WantedShape,
    /// **G9 with DW0-3**: how Blender stores the thing, read off the reading
    /// ([`crate::check::ReadReference::stored_source`]). It has already
    /// decided [`Self::shape`], and [`search`] classifies with it too.
    pub stored: StoredSource,
}

impl Outstanding {
    /// **F28: the kind decides the shape, and a set's marks decide it over
    /// the kind** — G22's expansion is a set whatever kind carries it, and a
    /// physics point cache is on both G6's folder list and G24's expanding
    /// list.
    ///
    /// **DW0-3: the marks make a set only where Blender does not store the
    /// thing as one file**, so the stored source is taken here with the kind
    /// and never left for a caller to remember.
    pub fn of(reference: RecordedLocation, kind: Kind, stored: StoredSource) -> Outstanding {
        Outstanding::read_as(reference.clone(), reference, kind, stored)
    }

    /// **G9a: the same, where the render reads something other than the
    /// recorded bytes.** Shape and name both come from what is read.
    pub fn read_as(
        reference: RecordedLocation,
        the_location_read: RecordedLocation,
        kind: Kind,
        stored: StoredSource,
    ) -> Outstanding {
        let shape = if resolves_to_a_folder(&the_location_read, stored) {
            WantedShape::ASetsFolder
        } else if kind.names_a_folder() {
            WantedShape::AFolder
        } else {
            WantedShape::AFile
        };
        Outstanding {
            reference,
            the_location_read,
            shape,
            stored,
        }
    }

    /// A reference whose shape is a file — the ordinary case, and what a
    /// caller with no kind to hand has.
    pub fn a_file(reference: RecordedLocation) -> Outstanding {
        Outstanding {
            the_location_read: reference.clone(),
            reference,
            shape: WantedShape::AFile,
            stored: StoredSource::NotSaidToBeOneFile,
        }
    }

    /// **The name this reference wants matched** — a file's final segment
    /// (F9), or a folder's final *named* segment (F28). A set's folder is
    /// matched by its members and never by a name, so it has none.
    pub fn wanted_name(&self) -> Option<&[u8]> {
        match self.shape {
            WantedShape::AFile => Some(self.the_location_read.wanted_name()),
            WantedShape::AFolder => Some(self.the_location_read.final_named_segment()),
            WantedShape::ASetsFolder => None,
        }
    }

    /// **G22's pattern for a set row** — its recorded location's final
    /// segment, which carries the frame-number characters, the token or the
    /// view-name mark.
    pub fn the_sets_pattern(&self) -> Option<&[u8]> {
        match self.shape {
            WantedShape::ASetsFolder => Some(self.the_location_read.wanted_name()),
            _ => None,
        }
    }
}

/// One candidate the walk turned up for a reference: which listed folder it
/// sits under (by index in the user's order), its path as the walk composed
/// it from enumerated stored names, the verdict, and **what stands at that
/// path** (F28).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidate {
    pub folder_index: usize,
    pub path: Vec<u8>,
    pub verdict: CandidateVerdict,
    /// **F28: the shape this candidate answers**, taken from what the walk
    /// met rather than from what a caller said. A file candidate is only ever
    /// produced from a file entry and a folder candidate only ever from a
    /// directory entry, which is what keeps
    /// [`resolution_from_the_search`] from recording a file's path as a set
    /// row's folder.
    pub answers: WantedShape,
}

/// **The whole of one search**: every candidate for every outstanding
/// reference, and the report.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SearchOutcome {
    pub candidates: HashMap<RecordedLocation, Vec<Candidate>>,
    pub report: SearchReport,
}

/// **F27: why a recorded location could not be looked at**, named rather than
/// passed over — the shape A5 already uses for a mirrored location that will
/// not fit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyTheRecordedLocationWasNotLookedAt {
    /// Blender's own shorthand with nothing to measure it from — the holding
    /// file's source folder is not known (K5, F17).
    TheHoldingFilesSourceFolderIsNotKnown,
    /// **A location that is not absolute after being made whole.** Nothing
    /// here guesses a folder to measure it against.
    ItIsNotAbsolute,
}

/// **F27: the recorded location made whole** — Blender's own shorthand
/// resolved against the holding file's own source folder, a full location as
/// it stands. **F17's own computation is used rather than a second one**, so
/// there is one rule for making a recorded location whole in this crate.
pub fn the_whole_recorded_location(
    recorded: &RecordedLocation,
    the_holders_source_folder: Option<&[u8]>,
) -> Result<Vec<u8>, WhyTheRecordedLocationWasNotLookedAt> {
    if recorded.as_bytes().starts_with(b"//") && the_holders_source_folder.is_none() {
        return Err(WhyTheRecordedLocationWasNotLookedAt::TheHoldingFilesSourceFolderIsNotKnown);
    }
    let whole = crate::shift::the_full_old_location(recorded, the_holders_source_folder)
        .ok_or(WhyTheRecordedLocationWasNotLookedAt::ItIsNotAbsolute)?;
    if whole.starts_with(&[crate::bytes::SEPARATOR]) {
        Ok(whole)
    } else {
        Err(WhyTheRecordedLocationWasNotLookedAt::ItIsNotAbsolute)
    }
}

/// **F27: what the recorded location itself answered.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheRecordedLocationsOwnAnswer {
    /// Found exactly — K4's *how it was resolved* takes its first value,
    /// `HowResolved::TheRecordedLocationItself`.
    Resolved(Resolution),
    /// **F10's proposal, carried exactly as the search carries one.**
    AProposal(Vec<u8>),
    /// Nothing is at those bytes. The reference goes on to the search
    /// unchanged.
    NothingIsThere,
    /// F14a's refusal — the volume cannot separate an exact match from a case
    /// near-miss, and says so rather than answering wrongly.
    TheSystemCannotSay,
    /// Refused, with the reason named. The reference goes on to the search
    /// exactly as it was.
    Refused(WhyTheRecordedLocationWasNotLookedAt),
}

/// **F27: the first place looked is the recorded location itself, and it is
/// looked at before any search.** The spec's own rule is that Smoozy *"looks
/// in two places, in order. First, where the file says the thing is."*
///
/// **Taken once per outstanding reference, whatever form G9 gives it — the
/// tilde and environment-variable forms included.** This lookup asks the disk
/// for the reference's exact recorded bytes, so it can never reach a file the
/// reference does not mean: G9 stores those two forms exactly as written and
/// never expands them, so the literal bytes are what Blender itself would
/// look for. Where nothing is at them the lookup answers nothing and the row
/// is unchanged; where a real file is named `cost$.png` the row resolves
/// instead of sending the user hunting. **That is why this is not
/// [`Form::is_searched`]'s question**: the search matches a final segment
/// against every name on the disk and could find a file the reference does
/// not mean, and this cannot.
///
/// **Exactness is F12's stored-name reading taken at that place**, exactly as
/// the search takes it — equal bytes is F9's match, fold-equal and not equal
/// is F10's proposal, per reference.
pub fn the_recorded_locations_own_answer(
    recorded: &RecordedLocation,
    shape: WantedShape,
    the_holders_source_folder: Option<&[u8]>,
    readings: &dyn Readings,
) -> TheRecordedLocationsOwnAnswer {
    let whole = match the_whole_recorded_location(recorded, the_holders_source_folder) {
        Ok(whole) => whole,
        Err(why) => return TheRecordedLocationsOwnAnswer::Refused(why),
    };
    match crate::shift::look_for(&whole, shape, readings) {
        crate::shift::ShiftOutcome::Resolved(stored) => {
            TheRecordedLocationsOwnAnswer::Resolved(Resolution {
                to: match shape {
                    WantedShape::AFile | WantedShape::AFolder => ResolvedTo::AWholeLocation(stored),
                    WantedShape::ASetsFolder => ResolvedTo::AFolderForASetRow(stored),
                },
                how: HowResolved::TheRecordedLocationItself,
            })
        }
        crate::shift::ShiftOutcome::AProposal(path) => {
            TheRecordedLocationsOwnAnswer::AProposal(path)
        }
        crate::shift::ShiftOutcome::Unchanged => TheRecordedLocationsOwnAnswer::NothingIsThere,
        crate::shift::ShiftOutcome::TheSystemCannotSay => {
            TheRecordedLocationsOwnAnswer::TheSystemCannotSay
        }
    }
}

/// **F3: the search walks every listed folder, in the user's order, each
/// subtree to completion, and collects every match for every outstanding
/// reference — it never returns the first thing the disk offers.** Two
/// promises force the full walk: the first folder holding a match wins,
/// which cannot be known without finishing enough of the walk to know no
/// earlier folder holds one; and the user is told every place a name matched.
///
/// **F6: stopped at any point, it keeps what it has** — the report says it did
/// not finish, so a *not found* left by a stopped search is not read as the
/// disk's answer. **F7: a folder that cannot be read fails alone.**
///
/// **G9: a reference of a form that is not searched is not on the list**, and
/// the caller has already classified it; this walk asks [`classify`] again
/// rather than trusting the caller.
///
/// **F28: the walk answers with a folder as well as with a file.** A
/// directory entry is a candidate for a folder-shaped reference — matched on
/// its own name, not on its contents — and for a set row where at least one
/// entry inside it matches the pattern; the walk goes on descending into it
/// either way (F3).
pub fn search(
    list: &SearchedFolderList,
    outstanding: &[Outstanding],
    readings: &dyn Readings,
    stop: &mut dyn FnMut() -> bool,
    now: SystemTime,
) -> SearchOutcome {
    let wanted: Vec<&Outstanding> = outstanding
        .iter()
        .filter(|reference| classify(&reference.the_location_read, reference.stored).is_searched())
        .collect();
    let mut outcome = SearchOutcome::default();
    let mut stopped = false;

    for (index, folder) in list.folders.iter().enumerate() {
        if stopped {
            break;
        }
        let verdict = match readings.enumerate_folder(folder) {
            FolderListing::CouldNotBeRead => FolderVerdict::CouldNotBeRead,
            FolderListing::Entries {
                entries,
                that_would_not_read,
            } => {
                if that_would_not_read > 0 {
                    outcome
                        .report
                        .folders_with_entries_that_would_not_read
                        .push((folder.clone(), that_would_not_read));
                }
                let mut walk = Walk {
                    readings,
                    wanted: &wanted,
                    folder_index: index,
                    outcome: &mut outcome,
                    stop,
                    stopped: false,
                };
                walk.through(folder, entries);
                if walk.stopped {
                    stopped = true;
                    FolderVerdict::StoppedPartWay
                } else {
                    FolderVerdict::SearchedWhole
                }
            }
        };
        outcome.report.per_folder.push(PerFolderOutcome {
            folder: folder.clone(),
            verdict,
            at: Reading::taken((), now, Clock::ThisComputer),
        });
    }
    outcome.report.finished = !stopped;
    outcome
}

struct Walk<'a> {
    readings: &'a dyn Readings,
    wanted: &'a [&'a Outstanding],
    folder_index: usize,
    outcome: &'a mut SearchOutcome,
    stop: &'a mut dyn FnMut() -> bool,
    stopped: bool,
}

impl Walk<'_> {
    fn through(&mut self, folder: &[u8], entries: Vec<crate::readings::EnumeratedEntry>) {
        let casing = self.readings.enumeration_casing_of(folder);
        for entry in entries {
            if (self.stop)() {
                self.stopped = true;
                return;
            }
            let path = join(folder, &entry.name);
            if entry.is_a_folder {
                let below = match self.readings.enumerate_folder(&path) {
                    FolderListing::Entries {
                        entries,
                        that_would_not_read,
                    } => {
                        // F7: an entry that would not read is named and the
                        // rest of the folder is searched.
                        if that_would_not_read > 0 {
                            self.outcome
                                .report
                                .folders_with_entries_that_would_not_read
                                .push((path.clone(), that_would_not_read));
                        }
                        Some(entries)
                    }
                    // F7: an unreadable subfolder is skipped, named, and the
                    // walk carries on.
                    FolderListing::CouldNotBeRead => {
                        self.outcome.report.unreadable_subfolders.push(path.clone());
                        None
                    }
                };
                // **F28: a directory entry is a candidate for a folder-shaped
                // reference on its own name**, whether or not it can be
                // listed — the reference wants the folder, not its contents.
                self.judge_a_directorys_own_name(&path, &entry.name, casing);
                // **F28: and for a set row where at least one entry inside it
                // matches the pattern** (G22), which needs the listing.
                if let Some(below) = &below {
                    self.judge_a_directory_as_a_sets_folder(&path, below);
                }
                if let Some(below) = below {
                    self.through(&path, below);
                    if self.stopped {
                        return;
                    }
                }
                continue;
            }
            let readings = self.readings;
            for reference in self.wanted {
                if reference.shape != WantedShape::AFile {
                    continue;
                }
                let Some(wanted_name) = reference.wanted_name() else {
                    continue;
                };
                let verdict = judge_candidate(wanted_name, &entry.name, casing, || {
                    readings.the_stored_name_reading(&path)
                });
                self.keep(reference, &path, verdict, WantedShape::AFile);
            }
        }
    }

    /// **F28: a folder-shaped reference is matched on the final *named*
    /// segment of its recorded location against a directory entry's own
    /// name**, under F9's byte equality and F10's fold exactly as a file is.
    fn judge_a_directorys_own_name(&mut self, path: &[u8], name: &[u8], casing: EnumerationCasing) {
        let readings = self.readings;
        for reference in self.wanted {
            if reference.shape != WantedShape::AFolder {
                continue;
            }
            let Some(wanted_name) = reference.wanted_name() else {
                continue;
            };
            if wanted_name.is_empty() {
                continue;
            }
            let verdict = judge_candidate(wanted_name, name, casing, || {
                readings.the_stored_name_reading(path)
            });
            self.keep(reference, path, verdict, WantedShape::AFolder);
        }
    }

    /// **F28 and G22: a set row's answer is the folder holding at least one
    /// entry matching the pattern.** The pattern is never a wanted name, so
    /// there is no fold and no proposal here — the folder either holds a
    /// member or it does not.
    fn judge_a_directory_as_a_sets_folder(
        &mut self,
        path: &[u8],
        below: &[crate::readings::EnumeratedEntry],
    ) {
        for reference in self.wanted {
            let Some(pattern) = reference.the_sets_pattern() else {
                continue;
            };
            let holds_a_member = below
                .iter()
                .any(|entry| !entry.is_a_folder && set_member_matches(pattern, &entry.name));
            if holds_a_member {
                self.keep(
                    reference,
                    path,
                    CandidateVerdict::Exact,
                    WantedShape::ASetsFolder,
                );
            }
        }
    }

    fn keep(
        &mut self,
        reference: &Outstanding,
        path: &[u8],
        verdict: CandidateVerdict,
        answers: WantedShape,
    ) {
        if verdict == CandidateVerdict::NotAMatch {
            return;
        }
        self.outcome
            .candidates
            .entry(reference.reference.clone())
            .or_default()
            .push(Candidate {
                folder_index: self.folder_index,
                path: path.to_vec(),
                verdict,
                answers,
            });
    }
}

/// **F4 and F5: which candidate answers a reference.** The first folder in
/// the user's order holding an exact match supplies the answer; where that
/// folder's subtree holds several, the pick is arbitrary and the answer says
/// so — the pick taken, the candidates it was taken among, and that it was a
/// coin toss rather than a judgment. **This is the thing Blender's own search
/// is recorded never to do** (F5, *Reading a file 10*).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheSearchsAnswer {
    pub folder_index: usize,
    pub folder: Vec<u8>,
    pub pick: Vec<u8>,
    /// The other exact candidates in the winning folder, where there were
    /// any — the pick was arbitrary among them and the row shows it.
    pub arbitrary_among: Vec<Vec<u8>>,
    /// **F28: what the pick is** — a file, a folder-shaped reference's own
    /// folder, or a set row's folder. Taken from the candidate the walk
    /// produced, never from a caller.
    pub answers: WantedShape,
}

pub fn the_answer_for(
    list: &SearchedFolderList,
    candidates: &[Candidate],
) -> Option<TheSearchsAnswer> {
    let winning = candidates
        .iter()
        .filter(|candidate| candidate.verdict == CandidateVerdict::Exact)
        .map(|candidate| candidate.folder_index)
        .min()?;
    let exact: Vec<&Candidate> = candidates
        .iter()
        .filter(|candidate| {
            candidate.verdict == CandidateVerdict::Exact && candidate.folder_index == winning
        })
        .collect();
    let pick = exact[0].path.clone();
    let answers = exact[0].answers;
    let arbitrary_among = if exact.len() > 1 {
        exact
            .iter()
            .map(|candidate| candidate.path.clone())
            .collect()
    } else {
        Vec::new()
    };
    Some(TheSearchsAnswer {
        folder_index: winning,
        folder: list.folders[winning].clone(),
        pick,
        arbitrary_among,
        answers,
    })
}

/// **F10: the proposals for a reference the search did not resolve**, and
/// **K4's rejected guess is not put forward again by itself.** Every place
/// a name near-missed is offered (F3's second promise).
pub fn the_proposals_for(
    candidates: &[Candidate],
    rejected_guess: Option<&StoredLocation>,
) -> Vec<Vec<u8>> {
    candidates
        .iter()
        .filter(|candidate| candidate.verdict == CandidateVerdict::AProposal)
        .map(|candidate| candidate.path.clone())
        .filter(|path| {
            rejected_guess
                .map(|guess| guess.as_bytes() != path.as_slice())
                .unwrap_or(true)
        })
        .collect()
}

/// **Whether the search met the limit on this reference** — some folder
/// could not separate an exact match from a near-miss, and the row says so
/// rather than carrying a false match.
pub fn the_limit_was_met(candidates: &[Candidate]) -> bool {
    candidates
        .iter()
        .any(|candidate| candidate.verdict == CandidateVerdict::TheSystemCannotSayWhichItIs)
}

/// Why a resolution could not be recorded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolutionFailed {
    /// **F23: a candidate that moved between the search and the act fails
    /// that row alone.**
    TheCandidateIsNoLongerThere,
    /// **F12 has no route on this volume** — the class is refused (F14a;
    /// §12 item 3) — so the whole stored location cannot be recorded and the
    /// mirror's stored-spelling guarantee is gone. The row fails with the
    /// reason named rather than recording a spelling nothing established.
    TheStoredSpellingCannotBeRead,
}

/// **F12: every resolution records the file's whole location as stored on
/// disk, read from the system and never rejoined from strings.** Taken per
/// row at the moment that row is resolved (F23).
pub fn the_stored_location_of(
    path: &[u8],
    readings: &dyn Readings,
) -> Result<StoredLocation, ResolutionFailed> {
    match readings.the_stored_name_reading(path) {
        StoredNameAnswer::TheStoredName(stored) => Ok(stored),
        StoredNameAnswer::NotThere => Err(ResolutionFailed::TheCandidateIsNoLongerThere),
        StoredNameAnswer::TheClassIsRefused => Err(ResolutionFailed::TheStoredSpellingCannotBeRead),
    }
}

/// **K4's resolution for a search's answer**, recorded with how it was
/// resolved — the search, from which folder, and whether the pick was
/// arbitrary and among which candidates.
/// **What the answer is recorded as comes from the candidate the walk found,
/// not from a flag a caller passed.** Before 2026-09-05 this took a
/// `a_set_row: bool` and wrapped `answer.pick` — a file off the walk — as
/// `ResolvedTo::AFolderForASetRow` whenever the caller said so.
pub fn resolution_from_the_search(
    answer: &TheSearchsAnswer,
    readings: &dyn Readings,
) -> Result<Resolution, ResolutionFailed> {
    let stored = the_stored_location_of(&answer.pick, readings)?;
    let arbitrary = if answer.arbitrary_among.is_empty() {
        None
    } else {
        Some(ArbitraryPick {
            taken: stored.clone(),
            among: answer
                .arbitrary_among
                .iter()
                .map(|path| StoredLocation::as_the_system_returned_it(path))
                .collect(),
        })
    };
    Ok(Resolution {
        to: match answer.answers {
            // K4: *"a whole location on the user's computer, as stored on
            // disk"* — a file, or the folder a folder-shaped reference names.
            WantedShape::AFile | WantedShape::AFolder => ResolvedTo::AWholeLocation(stored),
            // K4: *"or a folder for a set row"* (3.6).
            WantedShape::ASetsFolder => ResolvedTo::AFolderForASetRow(stored),
        },
        how: HowResolved::TheSearch {
            from_folder: answer.folder.clone(),
            arbitrary,
        },
    })
}

/// **F10 and F24: a proposal accepted is the user's own hand**, recorded per
/// row with F12's stored-name reading taken for that row.
pub fn resolution_from_an_accepted_proposal(
    proposal: &[u8],
    readings: &dyn Readings,
) -> Result<Resolution, ResolutionFailed> {
    Ok(Resolution {
        to: ResolvedTo::AWholeLocation(the_stored_location_of(proposal, readings)?),
        how: HowResolved::TheUsersOwnHand,
    })
}

/// A recorded location's form decides whether a resolution is a folder for a
/// set row (K4, 3.6). **Read by [`Outstanding::of`]**, which is how the shape
/// reaches the walk.
pub fn resolves_to_a_folder(reference: &RecordedLocation, stored: StoredSource) -> bool {
    matches!(classify(reference, stored), Form::ASet(_))
}

/// **G22: the expansion is a listing of the resolved folder with members
/// matched against the pattern's own form.** Once F28's search has answered a
/// set row with its folder, these are the row's members on the user's own
/// computer — the size and share G23 shows for the whole set, the files each
/// crossing as its own transfer, and the local half
/// [`crate::contents::set_row_membership`] compares against the mirrored
/// folder's own listing (Q5).
///
/// **The matching rule errs wide, which is G22's own decision**: a member
/// Blender would not have counted costs bandwidth; a member Smoozy misses
/// goes to the substitute colour on its own. `None` where the folder cannot
/// be read at all, which is not an empty set.
pub fn the_sets_members_in(
    folder: &StoredLocation,
    pattern_name: &[u8],
    readings: &dyn Readings,
) -> Option<Vec<Vec<u8>>> {
    let FolderListing::Entries { entries, .. } = readings.enumerate_folder(folder.as_bytes())
    else {
        return None;
    };
    Some(
        entries
            .into_iter()
            .filter(|entry| !entry.is_a_folder && set_member_matches(pattern_name, &entry.name))
            .map(|entry| entry.name)
            .collect(),
    )
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::bytes::MachineLocation;
    use crate::contents::ContentIdentity;
    use crate::readings::{EnumeratedEntry, FourValuesNow, MachineContents};
    use crate::record::MachineKey;
    use smoozy_server_protocol::transfers::integrity::Digest;
    use std::collections::HashSet;

    /// A disk written by hand: folders, their entries, and what the
    /// stored-name reading answers for each path.
    #[derive(Default)]
    pub struct Disk {
        pub folders: HashMap<Vec<u8>, Vec<EnumeratedEntry>>,
        pub unreadable: HashSet<Vec<u8>>,
        /// How many entries of a folder the listing itself could not read
        /// (F7).
        pub entries_that_would_not_read: HashMap<Vec<u8>, usize>,
        pub casing: HashMap<Vec<u8>, EnumerationCasing>,
        pub stored_names: HashMap<Vec<u8>, StoredNameAnswer>,
    }

    impl Disk {
        pub fn folder(
            &mut self,
            folder: &[u8],
            files: &[&[u8]],
            subfolders: &[&[u8]],
        ) -> &mut Disk {
            let mut entries: Vec<EnumeratedEntry> = files
                .iter()
                .map(|name| EnumeratedEntry {
                    name: name.to_vec(),
                    is_a_folder: false,
                })
                .collect();
            entries.extend(subfolders.iter().map(|name| EnumeratedEntry {
                name: name.to_vec(),
                is_a_folder: true,
            }));
            self.folders.insert(folder.to_vec(), entries);
            self
        }
    }

    impl Readings for Disk {
        fn the_local_files_content_identity(
            &self,
            _file: &StoredLocation,
        ) -> Option<ContentIdentity> {
            None
        }
        fn the_four_values_now(
            &self,
            _machine: &MachineKey,
            _at: &MachineLocation,
        ) -> FourValuesNow {
            FourValuesNow::TheChannelCannotAnswer
        }
        fn the_machine_copys_contents(
            &self,
            _machine: &MachineKey,
            _at: &MachineLocation,
        ) -> MachineContents {
            MachineContents::DidNotComplete
        }
        fn enumerate_folder(&self, folder: &[u8]) -> FolderListing {
            if self.unreadable.contains(folder) {
                return FolderListing::CouldNotBeRead;
            }
            match self.folders.get(folder) {
                Some(entries) => FolderListing::Entries {
                    entries: entries.clone(),
                    that_would_not_read: self
                        .entries_that_would_not_read
                        .get(folder)
                        .copied()
                        .unwrap_or(0),
                },
                None => FolderListing::CouldNotBeRead,
            }
        }
        fn enumeration_casing_of(&self, folder: &[u8]) -> EnumerationCasing {
            self.casing
                .get(folder)
                .copied()
                .unwrap_or(EnumerationCasing::IsTheStoredName)
        }
        fn the_stored_name_reading(&self, path: &[u8]) -> StoredNameAnswer {
            self.stored_names.get(path).cloned().unwrap_or_else(|| {
                StoredNameAnswer::TheStoredName(StoredLocation::as_the_system_returned_it(path))
            })
        }
        fn the_sources_prefix_digest(
            &self,
            _source: &StoredLocation,
            _up_to: u64,
        ) -> Option<Digest> {
            None
        }
    }

    fn recorded(bytes: &[u8]) -> RecordedLocation {
        RecordedLocation::as_the_reading_carried_it(bytes)
    }

    /// An outstanding reference wanting a file — the ordinary case.
    pub fn a_file(bytes: &[u8]) -> Outstanding {
        Outstanding::a_file(recorded(bytes))
    }

    /// An outstanding reference of a kind whose recorded location names a
    /// folder (F28).
    pub fn a_folder(bytes: &[u8]) -> Outstanding {
        Outstanding::of(
            recorded(bytes),
            Kind::FluidCacheFolder,
            StoredSource::NotSaidToBeOneFile,
        )
    }

    /// A set row — G22's pattern in its final segment (F28).
    pub fn a_set(bytes: &[u8]) -> Outstanding {
        Outstanding::of(
            recorded(bytes),
            Kind::Image,
            StoredSource::NotSaidToBeOneFile,
        )
    }

    fn never_stop() -> impl FnMut() -> bool {
        || false
    }

    #[test]
    fn a_match_is_bytes_and_a_case_difference_is_a_proposal_never_a_match() {
        assert!(is_an_exact_match(b"metallic.png", b"metallic.png"));
        assert!(!is_an_exact_match(b"METallic.png", b"metallic.png"));
        assert_eq!(
            judge_candidate(
                b"metallic.png",
                b"metallic.png",
                EnumerationCasing::IsTheStoredName,
                || unreachable!()
            ),
            CandidateVerdict::Exact
        );
        assert_eq!(
            judge_candidate(
                b"metallic.png",
                b"METallic.png",
                EnumerationCasing::IsTheStoredName,
                || unreachable!()
            ),
            CandidateVerdict::AProposal
        );
        assert_eq!(
            judge_candidate(
                b"metallic.png",
                b"wood.png",
                EnumerationCasing::IsTheStoredName,
                || unreachable!()
            ),
            CandidateVerdict::NotAMatch
        );
    }

    #[test]
    fn the_fold_is_full_so_eszett_and_ss_are_one_name_and_only_a_proposal() {
        assert!(proposal_equality(
            "straße.png".as_bytes(),
            "STRASSE.png".as_bytes()
        ));
        assert_eq!(
            judge_candidate(
                "straße.png".as_bytes(),
                "STRASSE.png".as_bytes(),
                EnumerationCasing::IsTheStoredName,
                || unreachable!()
            ),
            CandidateVerdict::AProposal
        );
        // Bytes that are not text compare as bytes.
        let raw = [0xff, 0xfe, b'.', b'p', b'n', b'g'];
        assert!(proposal_equality(&raw, &raw));
        assert!(!proposal_equality(&raw, b"x.png"));
    }

    #[test]
    fn where_the_enumerations_casing_is_not_established_the_normalized_name_decides_and_a_refusal_is_the_limit(
    ) {
        let stored = || {
            StoredNameAnswer::TheStoredName(StoredLocation::as_the_system_returned_it(
                b"C:/tex/metallic.png",
            ))
        };
        assert_eq!(
            judge_candidate(
                b"metallic.png",
                b"METALLIC.PNG",
                EnumerationCasing::NotEstablished,
                stored
            ),
            CandidateVerdict::Exact
        );
        let differs = || {
            StoredNameAnswer::TheStoredName(StoredLocation::as_the_system_returned_it(
                b"C:/tex/Metallic.png",
            ))
        };
        assert_eq!(
            judge_candidate(
                b"metallic.png",
                b"metallic.png",
                EnumerationCasing::NotEstablished,
                differs
            ),
            CandidateVerdict::AProposal
        );
        assert_eq!(
            judge_candidate(
                b"metallic.png",
                b"metallic.png",
                EnumerationCasing::NotEstablished,
                || StoredNameAnswer::TheClassIsRefused
            ),
            CandidateVerdict::TheSystemCannotSayWhichItIs
        );
        // And the reading is not taken at all for a name that does not fold
        // equal.
        assert_eq!(
            judge_candidate(
                b"metallic.png",
                b"wood.png",
                EnumerationCasing::NotEstablished,
                || unreachable!()
            ),
            CandidateVerdict::NotAMatch
        );
    }

    #[test]
    fn the_walk_takes_the_users_order_walks_every_subtree_and_collects_every_match() {
        let mut disk = Disk::default();
        disk.folder(b"/Archive", &[b"notes.txt"], &[b"deep"])
            .folder(b"/Archive/deep", &[b"metallic.png"], &[])
            .folder(b"/Work", &[b"metallic.png", b"METallic.png"], &[b"sub"])
            .folder(b"/Work/sub", &[b"metallic.png"], &[]);
        let list = SearchedFolderList {
            folders: vec![b"/Archive".to_vec(), b"/Work".to_vec()],
        };
        let wanted = [a_file(b"//tex/metallic.png")];
        let outcome = search(
            &list,
            &wanted,
            &disk,
            &mut never_stop(),
            SystemTime::UNIX_EPOCH,
        );
        assert!(outcome.report.finished);
        assert!(outcome
            .report
            .per_folder
            .iter()
            .all(|folder| folder.verdict == FolderVerdict::SearchedWhole));
        let candidates = &outcome.candidates[&wanted[0].reference];
        // Every place the name matched is collected — four, one a proposal.
        assert_eq!(candidates.len(), 4);
        let answer = the_answer_for(&list, candidates).expect("an answer");
        // F4: the first folder in the user's order wins, whatever the disk
        // offered first.
        assert_eq!(answer.folder, b"/Archive");
        assert_eq!(answer.pick, b"/Archive/deep/metallic.png");
        assert!(answer.arbitrary_among.is_empty());
    }

    #[test]
    fn several_matches_in_the_winning_folder_are_an_arbitrary_pick_that_says_so() {
        let mut disk = Disk::default();
        disk.folder(b"/Work", &[b"metallic.png"], &[b"sub"]).folder(
            b"/Work/sub",
            &[b"metallic.png"],
            &[],
        );
        let list = SearchedFolderList {
            folders: vec![b"/Work".to_vec()],
        };
        let wanted = [a_file(b"//tex/metallic.png")];
        let outcome = search(
            &list,
            &wanted,
            &disk,
            &mut never_stop(),
            SystemTime::UNIX_EPOCH,
        );
        let answer =
            the_answer_for(&list, &outcome.candidates[&wanted[0].reference]).expect("an answer");
        assert_eq!(answer.arbitrary_among.len(), 2);
        let resolution = resolution_from_the_search(&answer, &disk).expect("resolves");
        match resolution.how {
            HowResolved::TheSearch {
                from_folder,
                arbitrary,
            } => {
                assert_eq!(from_folder, b"/Work");
                assert_eq!(arbitrary.expect("arbitrary").among.len(), 2);
            }
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn a_folder_that_cannot_be_read_fails_alone_and_an_unreadable_subfolder_is_named() {
        let mut disk = Disk::default();
        disk.folder(b"/Work", &[b"metallic.png"], &[b"locked"]);
        disk.unreadable.insert(b"/Work/locked".to_vec());
        let list = SearchedFolderList {
            folders: vec![b"/Missing".to_vec(), b"/Work".to_vec()],
        };
        let wanted = [a_file(b"//tex/metallic.png")];
        let outcome = search(
            &list,
            &wanted,
            &disk,
            &mut never_stop(),
            SystemTime::UNIX_EPOCH,
        );
        assert!(outcome.report.finished);
        assert_eq!(
            outcome.report.per_folder[0].verdict,
            FolderVerdict::CouldNotBeRead
        );
        assert_eq!(
            outcome.report.per_folder[1].verdict,
            FolderVerdict::SearchedWhole
        );
        assert_eq!(
            outcome.report.unreadable_subfolders,
            vec![b"/Work/locked".to_vec()]
        );
        assert!(the_answer_for(&list, &outcome.candidates[&wanted[0].reference]).is_some());
    }

    /// **F7: one entry that will not read does not make the folder
    /// unreadable.**
    ///
    /// **This is the test that would have caught it**: a single failing entry
    /// threw away every entry gathered so far and answered *could not be
    /// read*, so where that folder was the first in the user's order its whole
    /// subtree went unwalked and the folders below it supplied the answers —
    /// which quietly reverses the one thing F4's order decides.
    #[test]
    fn one_entry_that_will_not_read_is_named_and_the_rest_of_the_folder_is_still_searched() {
        let mut disk = Disk::default();
        disk.folder(b"/Archive", &[b"metallic.png"], &[b"deep"])
            .folder(b"/Archive/deep", &[b"wood.png"], &[]);
        disk.folder(b"/Work", &[b"metallic.png"], &[]);
        disk.entries_that_would_not_read
            .insert(b"/Archive".to_vec(), 1);
        let list = SearchedFolderList {
            folders: vec![b"/Archive".to_vec(), b"/Work".to_vec()],
        };
        let wanted = [a_file(b"//tex/metallic.png")];
        let outcome = search(
            &list,
            &wanted,
            &disk,
            &mut never_stop(),
            SystemTime::UNIX_EPOCH,
        );
        // The folder was searched whole, not called unreadable.
        assert_eq!(
            outcome.report.per_folder[0].verdict,
            FolderVerdict::SearchedWhole
        );
        // The failure is named beside the unreadable subfolders.
        assert_eq!(
            outcome.report.folders_with_entries_that_would_not_read,
            vec![(b"/Archive".to_vec(), 1)]
        );
        // And F4's order still decides: the first listed folder supplies the
        // answer, which is the thing that was quietly reversed.
        let answer =
            the_answer_for(&list, &outcome.candidates[&wanted[0].reference]).expect("an answer");
        assert_eq!(answer.folder, b"/Archive");
        assert_eq!(answer.pick, b"/Archive/metallic.png");
    }

    #[test]
    fn a_stopped_search_keeps_what_it_has_and_says_it_did_not_finish() {
        let mut disk = Disk::default();
        disk.folder(b"/A", &[b"metallic.png"], &[])
            .folder(b"/B", &[b"wood.png"], &[]);
        let list = SearchedFolderList {
            folders: vec![b"/A".to_vec(), b"/B".to_vec()],
        };
        let wanted = [a_file(b"//metallic.png"), a_file(b"//wood.png")];
        let mut asked = 0;
        let mut stop = || {
            asked += 1;
            asked > 1
        };
        let outcome = search(&list, &wanted, &disk, &mut stop, SystemTime::UNIX_EPOCH);
        assert!(!outcome.report.finished);
        assert!(outcome.candidates.contains_key(&wanted[0].reference));
        assert!(!outcome.candidates.contains_key(&wanted[1].reference));
        assert_eq!(outcome.report.per_folder.len(), 2);
        assert_eq!(
            outcome.report.per_folder[1].verdict,
            FolderVerdict::StoppedPartWay
        );
    }

    #[test]
    fn proposals_are_offered_only_where_nothing_matched_and_a_rejected_guess_is_not_put_forward_again(
    ) {
        let mut disk = Disk::default();
        disk.folder(b"/Work", &[b"METallic.png"], &[]);
        let list = SearchedFolderList {
            folders: vec![b"/Work".to_vec()],
        };
        let wanted = [a_file(b"//tex/metallic.png")];
        let outcome = search(
            &list,
            &wanted,
            &disk,
            &mut never_stop(),
            SystemTime::UNIX_EPOCH,
        );
        let candidates = &outcome.candidates[&wanted[0].reference];
        assert!(the_answer_for(&list, candidates).is_none());
        assert_eq!(
            the_proposals_for(candidates, None),
            vec![b"/Work/METallic.png".to_vec()]
        );
        let rejected = StoredLocation::as_the_system_returned_it(b"/Work/METallic.png");
        assert!(the_proposals_for(candidates, Some(&rejected)).is_empty());
    }

    #[test]
    fn on_a_volume_that_refuses_the_class_the_search_reports_the_limit_and_finds_nothing() {
        let mut disk = Disk::default();
        disk.folder(b"/mnt/fat", &[b"metallic.png"], &[]);
        disk.casing
            .insert(b"/mnt/fat".to_vec(), EnumerationCasing::NotEstablished);
        disk.stored_names.insert(
            b"/mnt/fat/metallic.png".to_vec(),
            StoredNameAnswer::TheClassIsRefused,
        );
        let list = SearchedFolderList {
            folders: vec![b"/mnt/fat".to_vec()],
        };
        let wanted = [a_file(b"//tex/metallic.png")];
        let outcome = search(
            &list,
            &wanted,
            &disk,
            &mut never_stop(),
            SystemTime::UNIX_EPOCH,
        );
        let candidates = &outcome.candidates[&wanted[0].reference];
        assert!(the_answer_for(&list, candidates).is_none());
        assert!(the_proposals_for(candidates, None).is_empty());
        assert!(the_limit_was_met(candidates));
    }

    #[test]
    fn forms_that_are_not_searched_are_left_out_of_the_walk() {
        let mut disk = Disk::default();
        disk.folder(b"/Work", &[b"x.png"], &[]);
        let list = SearchedFolderList {
            folders: vec![b"/Work".to_vec()],
        };
        let wanted = [a_file(b"~/x.png"), a_file(b"$TEX/x.png")];
        let outcome = search(
            &list,
            &wanted,
            &disk,
            &mut never_stop(),
            SystemTime::UNIX_EPOCH,
        );
        assert!(outcome.candidates.is_empty());
    }

    #[test]
    fn the_stored_location_is_the_systems_answer_and_a_moved_candidate_fails_alone() {
        let mut disk = Disk::default();
        disk.stored_names.insert(
            b"/Work/metallic.png".to_vec(),
            StoredNameAnswer::TheStoredName(StoredLocation::as_the_system_returned_it(
                b"/Volumes/Work/metallic.png",
            )),
        );
        disk.stored_names
            .insert(b"/Work/gone.png".to_vec(), StoredNameAnswer::NotThere);
        assert_eq!(
            the_stored_location_of(b"/Work/metallic.png", &disk)
                .expect("stored")
                .as_bytes(),
            b"/Volumes/Work/metallic.png"
        );
        assert_eq!(
            the_stored_location_of(b"/Work/gone.png", &disk),
            Err(ResolutionFailed::TheCandidateIsNoLongerThere)
        );
    }

    #[test]
    fn a_fluid_cache_folder_is_found_written_with_and_without_a_trailing_separator() {
        let mut disk = Disk::default();
        disk.folder(b"/Work", &[], &[b"cache"])
            .folder(b"/Work/cache", &[], &[b"fluid"])
            .folder(
                b"/Work/cache/fluid",
                &[b"config.uni", b"data_0001.uni"],
                &[],
            );
        let list = SearchedFolderList {
            folders: vec![b"/Work".to_vec()],
        };
        let wanted = [a_folder(b"//cache/fluid/"), a_folder(b"//cache/fluid")];
        let outcome = search(
            &list,
            &wanted,
            &disk,
            &mut never_stop(),
            SystemTime::UNIX_EPOCH,
        );
        for reference in &wanted {
            let candidates = &outcome.candidates[&reference.reference];
            let answer = the_answer_for(&list, candidates).expect("an answer");
            assert_eq!(answer.pick, b"/Work/cache/fluid");
            assert_eq!(answer.answers, WantedShape::AFolder);
            let resolution = resolution_from_the_search(&answer, &disk).expect("resolves");
            assert_eq!(
                resolution.to,
                ResolvedTo::AWholeLocation(StoredLocation::as_the_system_returned_it(
                    b"/Work/cache/fluid"
                ))
            );
        }
        // The walk still descends into it: a file inside the folder is found
        // as a file for a reference that wants one.
        let also = [a_file(b"//cache/fluid/config.uni")];
        let outcome = search(
            &list,
            &also,
            &disk,
            &mut never_stop(),
            SystemTime::UNIX_EPOCH,
        );
        assert_eq!(
            the_answer_for(&list, &outcome.candidates[&also[0].reference])
                .expect("an answer")
                .pick,
            b"/Work/cache/fluid/config.uni"
        );
    }

    #[test]
    fn a_capitals_only_difference_in_a_folder_name_is_a_proposal_and_never_a_match() {
        let mut disk = Disk::default();
        disk.folder(b"/Work", &[], &[b"Fluid"])
            .folder(b"/Work/Fluid", &[], &[]);
        let list = SearchedFolderList {
            folders: vec![b"/Work".to_vec()],
        };
        let wanted = [a_folder(b"//cache/fluid/")];
        let outcome = search(
            &list,
            &wanted,
            &disk,
            &mut never_stop(),
            SystemTime::UNIX_EPOCH,
        );
        let candidates = &outcome.candidates[&wanted[0].reference];
        assert!(the_answer_for(&list, candidates).is_none());
        assert_eq!(
            the_proposals_for(candidates, None),
            vec![b"/Work/Fluid".to_vec()]
        );
    }

    #[test]
    fn a_frame_sequence_resolves_to_the_folder_holding_its_members_and_a_stranger_is_ignored() {
        let mut disk = Disk::default();
        disk.folder(b"/Work", &[], &[b"seq", b"empty"])
            .folder(
                b"/Work/seq",
                &[
                    b"frame_0001.png",
                    b"frame_0002.png",
                    b"frame_0003.png",
                    b"notes.txt",
                ],
                &[],
            )
            .folder(b"/Work/empty", &[b"notes.txt"], &[]);
        let list = SearchedFolderList {
            folders: vec![b"/Work".to_vec()],
        };
        let wanted = [a_set(b"//seq/frame_####.png")];
        let outcome = search(
            &list,
            &wanted,
            &disk,
            &mut never_stop(),
            SystemTime::UNIX_EPOCH,
        );
        let candidates = &outcome.candidates[&wanted[0].reference];
        // Only the folder that holds a member is a candidate.
        assert_eq!(candidates.len(), 1);
        let answer = the_answer_for(&list, candidates).expect("an answer");
        assert_eq!(answer.pick, b"/Work/seq");
        assert_eq!(answer.answers, WantedShape::ASetsFolder);
        let resolution = resolution_from_the_search(&answer, &disk).expect("resolves");
        assert_eq!(
            resolution.to,
            ResolvedTo::AFolderForASetRow(StoredLocation::as_the_system_returned_it(b"/Work/seq"))
        );
        // G22's expansion over the folder the search answered with: three
        // members and the stranger left out.
        let folder = StoredLocation::as_the_system_returned_it(b"/Work/seq");
        let members = the_sets_members_in(&folder, b"frame_####.png", &disk).expect("a listing");
        assert_eq!(
            members,
            vec![
                b"frame_0001.png".to_vec(),
                b"frame_0002.png".to_vec(),
                b"frame_0003.png".to_vec()
            ]
        );
    }

    #[test]
    fn a_set_rows_answer_can_never_be_a_file_off_the_walk() {
        // The pattern's own characters are never a wanted name, so a file
        // called `frame_####.png` on disk is not the set's answer — and
        // nothing can hand a file's path to the resolution as a folder.
        let mut disk = Disk::default();
        disk.folder(b"/Work", &[b"frame_####.png"], &[]);
        let list = SearchedFolderList {
            folders: vec![b"/Work".to_vec()],
        };
        let wanted = [a_set(b"//seq/frame_####.png")];
        let outcome = search(
            &list,
            &wanted,
            &disk,
            &mut never_stop(),
            SystemTime::UNIX_EPOCH,
        );
        assert!(outcome.candidates.is_empty());
    }

    #[test]
    fn the_recorded_location_itself_is_looked_at_first_and_records_that_it_resolved_there() {
        let mut disk = Disk::default();
        disk.folder(b"/Users/a/proj/tex", &[b"metallic.png"], &[]);
        let recorded_shorthand = recorded(b"//tex/metallic.png");
        assert_eq!(
            the_recorded_locations_own_answer(
                &recorded_shorthand,
                WantedShape::AFile,
                Some(b"/Users/a/proj"),
                &disk
            ),
            TheRecordedLocationsOwnAnswer::Resolved(Resolution {
                to: ResolvedTo::AWholeLocation(StoredLocation::as_the_system_returned_it(
                    b"/Users/a/proj/tex/metallic.png"
                )),
                how: HowResolved::TheRecordedLocationItself,
            })
        );
        // A full location as it stands needs no source folder.
        assert!(matches!(
            the_recorded_locations_own_answer(
                &recorded(b"/Users/a/proj/tex/metallic.png"),
                WantedShape::AFile,
                None,
                &disk
            ),
            TheRecordedLocationsOwnAnswer::Resolved(_)
        ));
        // Nothing there leaves the reference for the search.
        assert_eq!(
            the_recorded_locations_own_answer(
                &recorded(b"/Users/a/proj/tex/gone.png"),
                WantedShape::AFile,
                None,
                &disk
            ),
            TheRecordedLocationsOwnAnswer::NothingIsThere
        );
    }

    #[test]
    fn the_recorded_location_lookup_refuses_what_is_not_absolute_and_names_the_reason() {
        let disk = Disk::default();
        assert_eq!(
            the_recorded_locations_own_answer(
                &recorded(b"//tex/x.png"),
                WantedShape::AFile,
                None,
                &disk
            ),
            TheRecordedLocationsOwnAnswer::Refused(
                WhyTheRecordedLocationWasNotLookedAt::TheHoldingFilesSourceFolderIsNotKnown
            )
        );
        assert_eq!(
            the_recorded_locations_own_answer(
                &recorded(b"tex/x.png"),
                WantedShape::AFile,
                None,
                &disk
            ),
            TheRecordedLocationsOwnAnswer::Refused(
                WhyTheRecordedLocationWasNotLookedAt::ItIsNotAbsolute
            )
        );
        // A shorthand measured against a folder that is itself not absolute
        // is refused too, rather than looked for somewhere unnamed.
        assert_eq!(
            the_recorded_locations_own_answer(
                &recorded(b"//tex/x.png"),
                WantedShape::AFile,
                Some(b"proj"),
                &disk
            ),
            TheRecordedLocationsOwnAnswer::Refused(
                WhyTheRecordedLocationWasNotLookedAt::ItIsNotAbsolute
            )
        );
    }

    #[test]
    fn the_recorded_location_is_looked_at_for_the_tilde_and_dollar_forms_too() {
        // The lookup asks the disk for the reference's exact recorded bytes,
        // so a real file named with a `$` resolves instead of sending the
        // user hunting — while the search still leaves both forms alone.
        let mut disk = Disk::default();
        disk.folder(b"/Users/a/proj", &[b"cost$.png", b"~notes.png"], &[]);
        assert!(matches!(
            the_recorded_locations_own_answer(
                &recorded(b"/Users/a/proj/cost$.png"),
                WantedShape::AFile,
                None,
                &disk
            ),
            TheRecordedLocationsOwnAnswer::Resolved(_)
        ));
        assert!(matches!(
            the_recorded_locations_own_answer(
                &recorded(b"//~notes.png"),
                WantedShape::AFile,
                Some(b"/Users/a/proj"),
                &disk
            ),
            TheRecordedLocationsOwnAnswer::Resolved(_)
        ));
        assert!(!classify(
            &recorded(b"/Users/a/proj/cost$.png"),
            StoredSource::NotSaidToBeOneFile
        )
        .is_searched());
    }

    #[test]
    fn a_folder_shaped_reference_is_looked_at_by_the_recorded_location_too() {
        let mut disk = Disk::default();
        disk.folder(b"/Users/a/proj/cache", &[], &[b"fluid"])
            .folder(b"/Users/a/proj/cache/fluid", &[b"config.uni"], &[]);
        assert_eq!(
            the_recorded_locations_own_answer(
                &recorded(b"//cache/fluid/"),
                WantedShape::AFolder,
                Some(b"/Users/a/proj"),
                &disk
            ),
            TheRecordedLocationsOwnAnswer::Resolved(Resolution {
                to: ResolvedTo::AWholeLocation(StoredLocation::as_the_system_returned_it(
                    b"/Users/a/proj/cache/fluid"
                )),
                how: HowResolved::TheRecordedLocationItself,
            })
        );
        // A set row's own folder, on G22's membership rather than a name.
        let mut disk = Disk::default();
        disk.folder(b"/Users/a/proj/seq", &[b"frame_0001.png"], &[]);
        assert_eq!(
            the_recorded_locations_own_answer(
                &recorded(b"//seq/frame_####.png"),
                WantedShape::ASetsFolder,
                Some(b"/Users/a/proj"),
                &disk
            ),
            TheRecordedLocationsOwnAnswer::Resolved(Resolution {
                to: ResolvedTo::AFolderForASetRow(StoredLocation::as_the_system_returned_it(
                    b"/Users/a/proj/seq"
                )),
                how: HowResolved::TheRecordedLocationItself,
            })
        );
    }

    #[test]
    fn the_shape_comes_from_the_kind_and_a_sets_marks_win_over_it() {
        assert_eq!(
            Outstanding::of(
                recorded(b"//tex/x.png"),
                Kind::Image,
                StoredSource::NotSaidToBeOneFile
            )
            .shape,
            WantedShape::AFile
        );
        assert_eq!(
            Outstanding::of(
                recorded(b"//cache/fluid/"),
                Kind::FluidCacheFolder,
                StoredSource::NotSaidToBeOneFile
            )
            .shape,
            WantedShape::AFolder
        );
        assert_eq!(
            Outstanding::of(
                recorded(b"//seq/frame_####.png"),
                Kind::Image,
                StoredSource::NotSaidToBeOneFile
            )
            .shape,
            WantedShape::ASetsFolder
        );
        // A physics point cache is on G6's folder list and on G24's expanding
        // list; the marks decide.
        assert_eq!(
            Outstanding::of(
                recorded(b"//cache/blendcache"),
                Kind::PhysicsPointCache,
                StoredSource::NotSaidToBeOneFile
            )
            .shape,
            WantedShape::AFolder
        );
        assert_eq!(
            Outstanding::of(
                recorded(b"//cache/blendcache/x_######.bphys"),
                Kind::PhysicsPointCache,
                StoredSource::NotSaidToBeOneFile
            )
            .shape,
            WantedShape::ASetsFolder
        );
        assert_eq!(
            Outstanding::of(
                recorded(b"//cache/fluid/"),
                Kind::FluidCacheFolder,
                StoredSource::NotSaidToBeOneFile
            )
            .wanted_name(),
            Some(&b"fluid"[..])
        );
        assert_eq!(
            Outstanding::of(
                recorded(b"//seq/frame_####.png"),
                Kind::Image,
                StoredSource::NotSaidToBeOneFile
            )
            .the_sets_pattern(),
            Some(&b"frame_####.png"[..])
        );
    }

    #[test]
    fn a_set_reference_resolves_to_a_folder() {
        assert!(resolves_to_a_folder(
            &recorded(b"//tex/tile.<UDIM>.png"),
            StoredSource::NotSaidToBeOneFile
        ));
        assert!(!resolves_to_a_folder(
            &recorded(b"//tex/tile.png"),
            StoredSource::NotSaidToBeOneFile
        ));
    }

    /// **DW0-3 carried to the search's own shapes**: a still picture whose
    /// name holds a `#` is wanted as a file by its whole name, and F27's look
    /// at the recorded location finds it where the `.blend` says — the case
    /// that was *not found* on the splash scenes' battle project.
    #[test]
    fn a_picture_stored_as_one_file_is_wanted_as_a_file_and_found_where_the_file_says() {
        let still = "//../Текстуры/лев's база #2.jpg".as_bytes();
        let wanted = Outstanding::of(recorded(still), Kind::Image, StoredSource::OneFile);
        assert_eq!(wanted.shape, WantedShape::AFile);
        assert_eq!(wanted.wanted_name(), Some("лев's база #2.jpg".as_bytes()));
        assert!(!resolves_to_a_folder(
            &recorded(still),
            StoredSource::OneFile
        ));
        // The same name not said to be one file is still a set.
        assert_eq!(
            Outstanding::of(
                recorded(still),
                Kind::Image,
                StoredSource::NotSaidToBeOneFile
            )
            .shape,
            WantedShape::ASetsFolder
        );

        let mut disk = Disk::default();
        disk.folder(
            "/p/scene/Текстуры".as_bytes(),
            &["лев's база #2.jpg".as_bytes()],
            &[],
        );
        let here = "//Текстуры/лев's база #2.jpg".as_bytes();
        let answer = the_recorded_locations_own_answer(
            &recorded(here),
            Outstanding::of(recorded(here), Kind::Image, StoredSource::OneFile).shape,
            Some(b"/p/scene"),
            &disk,
        );
        assert!(
            matches!(answer, TheRecordedLocationsOwnAnswer::Resolved(_)),
            "{answer:?}"
        );
    }
}
