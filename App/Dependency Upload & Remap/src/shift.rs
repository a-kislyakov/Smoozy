//! The shift — `Dependency Upload & Remap` §4.4, Row 167: F16–F21, and the
//! record it depends on (1.2 K6).
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** *(none)*

use crate::bytes::{join, segments, RecordedLocation, StoredLocation, SEPARATOR};
use crate::check::set_member_matches;
use crate::readings::{FolderListing, Readings, StoredNameAnswer};
use crate::record::{LocatedAnswer, LocatedAnswerKey, ShiftKey};
use crate::search::{judge_candidate, CandidateVerdict, WantedShape};

/// **F16: a shift — the old root and the new root** — derived from one
/// hand-locate as the complement of the longest common suffix of whole
/// segments, compared in exact bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shift {
    pub old_root: Vec<u8>,
    pub new_root: Vec<u8>,
}

/// **F16's two refusals.** Locating then taught that one answer and nothing
/// more.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoShift {
    /// The located file's name differs from the reference's — the user
    /// pointed at a renamed file, which the spec allows — so there is no
    /// common suffix.
    TheNamesDiffer,
    /// **The common suffix is the reference's old location entire and the old
    /// root comes out empty** — the ordinary shape of a backup keeping the
    /// original tree under a new prefix. An empty old root is one every
    /// absolute reference sits under, so the shift would reach all of them
    /// and the bound would be vacuous. **The cost is stated rather than
    /// hidden**: such a user locates once per project root.
    TheOldRootWouldBeEmpty,
}

/// The byte spans of a path's whole segments, so a root can be cut from the
/// original bytes rather than rejoined.
fn segment_spans(path: &[u8]) -> Vec<(usize, usize)> {
    let mut spans = Vec::new();
    let mut start = None;
    for (at, byte) in path.iter().enumerate() {
        match (start, *byte == SEPARATOR) {
            (None, false) => start = Some(at),
            (Some(from), true) => {
                spans.push((from, at));
                start = None;
            }
            _ => {}
        }
    }
    if let Some(from) = start {
        spans.push((from, path.len()));
    }
    spans
}

/// **F16: the longest common suffix of whole segments, in exact bytes; what
/// is left is the shift.** Old location `work/project1/assets/textures/metallic.png`
/// and new location `Archive/project1/assets/textures/metallic.png` share
/// `project1/assets/textures/metallic.png`; the old root is `work` and the new
/// root is `Archive`.
pub fn derive(old_location: &[u8], new_location: &[u8]) -> Result<Shift, NoShift> {
    let old = segments(old_location);
    let new = segments(new_location);
    let common = old
        .iter()
        .rev()
        .zip(new.iter().rev())
        .take_while(|(a, b)| a == b)
        .count();
    if common == 0 {
        return Err(NoShift::TheNamesDiffer);
    }
    let old_root_segments = old.len() - common;
    if old_root_segments == 0 {
        return Err(NoShift::TheOldRootWouldBeEmpty);
    }
    let new_root_segments = new.len() - common;
    Ok(Shift {
        old_root: root_of(old_location, old_root_segments),
        new_root: root_of(new_location, new_root_segments),
    })
}

/// The first `count` segments of a path as the path's own bytes — a leading
/// separator kept, nothing rejoined.
fn root_of(path: &[u8], count: usize) -> Vec<u8> {
    let spans = segment_spans(path);
    match count {
        0 => {
            if path.starts_with(&[SEPARATOR]) {
                vec![SEPARATOR]
            } else {
                Vec::new()
            }
        }
        n => path[..spans[n - 1].1].to_vec(),
    }
}

impl Shift {
    /// **F16: a shift reaches exactly the references whose old location sits
    /// under the old root** — sibling folders at any depth included; nothing
    /// outside the old root.
    pub fn reaches(&self, old_location: &[u8]) -> bool {
        let root = segments(&self.old_root);
        let location = segments(old_location);
        location.len() > root.len()
            && location[..root.len()] == root[..]
            && old_location.starts_with(&[SEPARATOR]) == self.old_root.starts_with(&[SEPARATOR])
    }

    /// The candidate location for a reference the shift reaches — old root
    /// swapped for new.
    pub fn shifted(&self, old_location: &[u8]) -> Option<Vec<u8>> {
        if !self.reaches(old_location) {
            return None;
        }
        let root = segments(&self.old_root).len();
        let spans = segment_spans(old_location);
        let remainder = &old_location[spans[root].0..];
        Some(join(&self.new_root, remainder))
    }
}

/// **F17: a shift needs an old location written out in full** — it applies
/// where the recorded location is full, or where the holding file's placement
/// record names where its upload came from, the relative location resolved
/// against that source folder. A file with no known origin (K5) contributes
/// nothing to a shift.
pub fn the_full_old_location(
    recorded: &RecordedLocation,
    the_holders_source_folder: Option<&[u8]>,
) -> Option<Vec<u8>> {
    let bytes = recorded.as_bytes();
    if let Some(relative) = bytes.strip_prefix(b"//") {
        return the_holders_source_folder.map(|folder| join(folder, relative));
    }
    if bytes.starts_with(&[SEPARATOR]) {
        return Some(bytes.to_vec());
    }
    None
}

/// **F18: what looking at a shifted location found.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShiftOutcome {
    /// Found exactly — the row resolves, marked as from this shift (F19).
    Resolved(StoredLocation),
    /// Found as a case near-miss — offered as a proposal exactly as the
    /// search offers one.
    AProposal(Vec<u8>),
    /// Found not at all — the row stays as it was.
    Unchanged,
    /// The volume could not say (F14a's refusal) — the row stays as it was
    /// and carries the limit.
    TheSystemCannotSay,
}

/// **F18: a shift resolves rows by looking, not by assuming.** Each candidate
/// is checked for a file at the shifted location over enumerated stored bytes
/// like everything else, with F12's stored-name reading taken where it is
/// found exactly.
pub fn apply_to_one(
    shift: &Shift,
    old_location: &[u8],
    shape: WantedShape,
    readings: &dyn Readings,
) -> ShiftOutcome {
    let Some(candidate) = shift.shifted(old_location) else {
        return ShiftOutcome::Unchanged;
    };
    look_for(&candidate, shape, readings)
}

/// **F18's look, shared with F26's locate-at-a-folder and F27's lookup at the
/// recorded location itself**: enumerate the named place's own parent and
/// judge each entry against the wanted name.
///
/// **F28: a folder-shaped row resolves at a directory.** Which entries are
/// looked at, and what is compared, is the shape's:
///
/// - **A file** — the entries that are not folders, matched on the
///   candidate's final segment.
/// - **A folder-shaped reference** — the entries that are folders, matched on
///   the candidate's final *named* segment with any trailing separator
///   dropped. F9's byte equality and F10's fold apply unchanged.
/// - **A set row** — the folder the pattern sits in, resolved where it holds
///   at least one member matching the pattern (G22). There is no name to fold
///   and so no proposal.
pub fn look_for(candidate: &[u8], shape: WantedShape, readings: &dyn Readings) -> ShiftOutcome {
    if shape == WantedShape::ASetsFolder {
        return look_for_a_sets_folder(candidate, readings);
    }
    let named = match shape {
        WantedShape::AFolder => crate::bytes::without_trailing_separators(candidate),
        _ => candidate,
    };
    let folder = crate::bytes::folder_of(named);
    let wanted = crate::bytes::last_segment(named);
    if wanted.is_empty() {
        return ShiftOutcome::Unchanged;
    }
    let want_a_folder = shape == WantedShape::AFolder;
    let FolderListing::Entries { entries, .. } = readings.enumerate_folder(&folder) else {
        return ShiftOutcome::Unchanged;
    };
    let casing = readings.enumeration_casing_of(&folder);
    let mut proposal = None;
    let mut limit = false;
    for entry in entries
        .iter()
        .filter(|entry| entry.is_a_folder == want_a_folder)
    {
        let path = join(&folder, &entry.name);
        match judge_candidate(wanted, &entry.name, casing, || {
            readings.the_stored_name_reading(&path)
        }) {
            CandidateVerdict::Exact => {
                return match readings.the_stored_name_reading(&path) {
                    StoredNameAnswer::TheStoredName(stored) => ShiftOutcome::Resolved(stored),
                    StoredNameAnswer::TheClassIsRefused => ShiftOutcome::TheSystemCannotSay,
                    StoredNameAnswer::NotThere => ShiftOutcome::Unchanged,
                };
            }
            CandidateVerdict::AProposal => proposal = proposal.or(Some(path)),
            CandidateVerdict::TheSystemCannotSayWhichItIs => limit = true,
            CandidateVerdict::NotAMatch => {}
        }
    }
    match (proposal, limit) {
        (Some(path), _) => ShiftOutcome::AProposal(path),
        (None, true) => ShiftOutcome::TheSystemCannotSay,
        (None, false) => ShiftOutcome::Unchanged,
    }
}

/// **G22 and F28: a set row resolves at the folder the pattern sits in, where
/// that folder holds at least one member matching the pattern.**
fn look_for_a_sets_folder(candidate: &[u8], readings: &dyn Readings) -> ShiftOutcome {
    let folder = crate::bytes::folder_of(candidate);
    let pattern = crate::bytes::last_segment(candidate);
    let FolderListing::Entries { entries, .. } = readings.enumerate_folder(&folder) else {
        return ShiftOutcome::Unchanged;
    };
    let holds_a_member = entries
        .iter()
        .any(|entry| !entry.is_a_folder && set_member_matches(pattern, &entry.name));
    if !holds_a_member {
        return ShiftOutcome::Unchanged;
    }
    match readings.the_stored_name_reading(&folder) {
        StoredNameAnswer::TheStoredName(stored) => ShiftOutcome::Resolved(stored),
        StoredNameAnswer::TheClassIsRefused => ShiftOutcome::TheSystemCannotSay,
        StoredNameAnswer::NotThere => ShiftOutcome::Unchanged,
    }
}

/// **F19: every row a shift resolved names the shift record (K6), and the
/// shift is shown as the move it is** — the old location and the new one.
pub fn shown_as_the_move_it_is(shift: &Shift, old_location: &[u8]) -> Option<(Vec<u8>, Vec<u8>)> {
    shift
        .shifted(old_location)
        .map(|new_location| (old_location.to_vec(), new_location))
}

/// **F20: one action returns every row the shift produced to *not found*.**
/// The undo enumerates by the shift record's key, which every answer it
/// produced carries; a shift application stopped part-way leaves only answers
/// carrying that key, so the undo is complete whether or not the application
/// ran to its end.
pub fn the_undo_enumerates(answers: &[LocatedAnswer], shift: ShiftKey) -> Vec<LocatedAnswerKey> {
    answers
        .iter()
        .filter(|answer| answer.was_produced_by(shift))
        .map(|answer| answer.key.clone())
        .collect()
}

/// **F21: a project spread over several roots takes one locate per root, and
/// shifts do not chain** — a later locate derives its own shift from its own
/// pair, never from an earlier shift's output. Kept by construction:
/// [`derive`] takes the two locations of one hand-locate and nothing else.
pub const SHIFTS_DO_NOT_CHAIN: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::record::{HoldingFile, HowResolved, Resolution, ResolvedTo};
    use crate::search::tests::Disk;

    #[test]
    fn the_shift_is_the_complement_of_the_longest_common_suffix() {
        let shift = derive(
            b"work/project1/assets/textures/metallic.png",
            b"Archive/project1/assets/textures/metallic.png",
        )
        .expect("a shift");
        assert_eq!(shift.old_root, b"work");
        assert_eq!(shift.new_root, b"Archive");

        let absolute = derive(
            b"/Users/a/work/proj/tex/x.png",
            b"/Volumes/Old/proj/tex/x.png",
        )
        .expect("a shift");
        assert_eq!(absolute.old_root, b"/Users/a/work");
        assert_eq!(absolute.new_root, b"/Volumes/Old");
    }

    #[test]
    fn a_renamed_file_and_an_empty_old_root_derive_no_shift() {
        assert_eq!(
            derive(
                b"/work/proj/tex/metallic.png",
                b"/Archive/proj/tex/metal.png"
            ),
            Err(NoShift::TheNamesDiffer)
        );
        assert_eq!(
            derive(
                b"/Users/alice/proj/tex/x.png",
                b"/Volumes/Backup/Users/alice/proj/tex/x.png"
            ),
            Err(NoShift::TheOldRootWouldBeEmpty)
        );
    }

    #[test]
    fn a_shift_reaches_siblings_at_any_depth_under_the_old_root_and_nothing_outside_it() {
        let shift = derive(b"/work/proj/tex/x.png", b"/Archive/proj/tex/x.png").expect("a shift");
        assert!(shift.reaches(b"/work/proj/models/deep/chair.blend"));
        assert!(shift.reaches(b"/work/other.png"));
        assert!(!shift.reaches(b"/works/proj/tex/x.png"));
        assert!(!shift.reaches(b"/other/proj/tex/x.png"));
        assert!(!shift.reaches(b"/work"));
        assert_eq!(
            shift
                .shifted(b"/work/proj/models/chair.blend")
                .expect("shifted"),
            b"/Archive/proj/models/chair.blend"
        );
        assert_eq!(
            shown_as_the_move_it_is(&shift, b"/work/a.png"),
            Some((b"/work/a.png".to_vec(), b"/Archive/a.png".to_vec()))
        );
    }

    #[test]
    fn a_relative_location_needs_the_holders_source_folder_and_k5_contributes_nothing() {
        let recorded = RecordedLocation::as_the_reading_carried_it(b"//tex/x.png");
        assert_eq!(
            the_full_old_location(&recorded, Some(b"/Users/a/proj")),
            Some(b"/Users/a/proj/tex/x.png".to_vec())
        );
        assert_eq!(the_full_old_location(&recorded, None), None);
        let full = RecordedLocation::as_the_reading_carried_it(b"/Users/a/proj/tex/x.png");
        assert_eq!(
            the_full_old_location(&full, None),
            Some(b"/Users/a/proj/tex/x.png".to_vec())
        );
        let bare = RecordedLocation::as_the_reading_carried_it(b"x.png");
        assert_eq!(the_full_old_location(&bare, Some(b"/Users/a")), None);
    }

    #[test]
    fn a_shift_resolves_by_looking_and_a_near_miss_is_a_proposal() {
        let mut disk = Disk::default();
        disk.folder(b"/Archive/proj/tex", &[b"x.png", b"WOOD.png"], &[]);
        let shift = derive(b"/work/proj/tex/x.png", b"/Archive/proj/tex/x.png").expect("a shift");
        assert_eq!(
            apply_to_one(&shift, b"/work/proj/tex/x.png", WantedShape::AFile, &disk),
            ShiftOutcome::Resolved(StoredLocation::as_the_system_returned_it(
                b"/Archive/proj/tex/x.png"
            ))
        );
        assert_eq!(
            apply_to_one(
                &shift,
                b"/work/proj/tex/wood.png",
                WantedShape::AFile,
                &disk
            ),
            ShiftOutcome::AProposal(b"/Archive/proj/tex/WOOD.png".to_vec())
        );
        assert_eq!(
            apply_to_one(
                &shift,
                b"/work/proj/tex/gone.png",
                WantedShape::AFile,
                &disk
            ),
            ShiftOutcome::Unchanged
        );
        assert_eq!(
            apply_to_one(&shift, b"/elsewhere/x.png", WantedShape::AFile, &disk),
            ShiftOutcome::Unchanged
        );
    }

    #[test]
    fn a_folder_shaped_row_and_a_set_row_resolve_at_a_directory_after_a_shift() {
        let mut disk = Disk::default();
        disk.folder(b"/Archive/proj/cache", &[], &[b"fluid"])
            .folder(b"/Archive/proj/cache/fluid", &[b"config.uni"], &[])
            .folder(
                b"/Archive/proj/seq",
                &[b"frame_0001.png", b"notes.txt"],
                &[],
            );
        let shift = derive(b"/work/proj/tex/x.png", b"/Archive/proj/tex/x.png").expect("a shift");

        // A fluid cache folder, written with a trailing separator.
        assert_eq!(
            apply_to_one(
                &shift,
                b"/work/proj/cache/fluid/",
                WantedShape::AFolder,
                &disk
            ),
            ShiftOutcome::Resolved(StoredLocation::as_the_system_returned_it(
                b"/Archive/proj/cache/fluid"
            ))
        );
        // The same row read as a file finds nothing, which is the fault this
        // shape exists to remove.
        assert_eq!(
            apply_to_one(
                &shift,
                b"/work/proj/cache/fluid/",
                WantedShape::AFile,
                &disk
            ),
            ShiftOutcome::Unchanged
        );
        // A set row resolves at the folder holding a member.
        assert_eq!(
            apply_to_one(
                &shift,
                b"/work/proj/seq/frame_####.png",
                WantedShape::ASetsFolder,
                &disk
            ),
            ShiftOutcome::Resolved(StoredLocation::as_the_system_returned_it(
                b"/Archive/proj/seq"
            ))
        );
        // A folder with no member of the set is not the set's answer.
        assert_eq!(
            apply_to_one(
                &shift,
                b"/work/proj/cache/fluid/frame_####.png",
                WantedShape::ASetsFolder,
                &disk
            ),
            ShiftOutcome::Unchanged
        );
        // A folder-shaped row differing only in capitals is a proposal.
        let mut folded = Disk::default();
        folded
            .folder(b"/Archive/proj/cache", &[], &[b"Fluid"])
            .folder(b"/Archive/proj/cache/Fluid", &[], &[]);
        assert_eq!(
            apply_to_one(
                &shift,
                b"/work/proj/cache/fluid",
                WantedShape::AFolder,
                &folded
            ),
            ShiftOutcome::AProposal(b"/Archive/proj/cache/Fluid".to_vec())
        );
    }

    #[test]
    fn the_undo_enumerates_by_the_shift_key_and_nothing_else() {
        let key = |reference: &[u8]| LocatedAnswerKey {
            holding_file: HoldingFile::OnTheUsersComputer(
                StoredLocation::as_the_system_returned_it(b"/a.blend"),
            ),
            reference: RecordedLocation::as_the_reading_carried_it(reference),
        };
        let by = |how: HowResolved, reference: &[u8]| LocatedAnswer {
            key: key(reference),
            resolution: Some(Resolution {
                to: ResolvedTo::AWholeLocation(StoredLocation::as_the_system_returned_it(b"/x")),
                how,
            }),
            set_aside: false,
            rejected_guess: None,
            read_as: Vec::new(),
        };
        let answers = vec![
            by(HowResolved::AShift(ShiftKey(1)), b"/work/a.png"),
            by(HowResolved::AShift(ShiftKey(2)), b"/work/b.png"),
            by(HowResolved::TheUsersOwnHand, b"/work/c.png"),
            by(HowResolved::AShift(ShiftKey(1)), b"/work/d.png"),
        ];
        let undone = the_undo_enumerates(&answers, ShiftKey(1));
        assert_eq!(undone, vec![key(b"/work/a.png"), key(b"/work/d.png")]);
    }
}
