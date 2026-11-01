//! The byte objects every comparison in this module is made over — G3a's
//! recorded location, F12's stored location, and the machine location Smoozy
//! composes — and the one rule they share: **bytes end to end, never text,
//! never tidied, never re-derived.**
//!
//! **G3a: the recorded location is one byte object with one origin.** What the
//! reading hands back for a reference is what the row's column carries, what
//! K3 keys the located answer on, what F9 compares against, what A4 mirrors
//! and what M4 carries as a pair's expected bytes. *"There is no second
//! reading, no re-composition from segments, and no tidying, folding or
//! normalising anywhere between the reading and the column."* The value is
//! bytes and is not required to be text — a location is exempt from the rule
//! that a file's text is all written one way.
//!
//! **F12: every resolution records the file's whole location as stored on
//! disk, read from the system and never rejoined from strings.** The type has
//! one constructor and it takes what the system returned.

use serde::{Deserialize, Serialize};

/// The separator byte a location splits at. Every path on the machine is
/// Linux's, and M4a's rule turns the other separator into this one before a
/// pair is matched.
pub const SEPARATOR: u8 = b'/';

/// **The bytes a location on the *user's own computer* separates its
/// components at**, which is not [`SEPARATOR`]: that one is the machine's, and
/// the machine is Linux whatever the user is sitting at.
///
/// **This is only ever read over a [`StoredLocation`]** — a value the local
/// system's own call returned. It is deliberately not read over a
/// [`RecordedLocation`], which is text a `.blend` carries: round 1's Verdict
/// dropped exactly that on the ground that a Windows-authored recorded
/// location is permanently unresolved (`Dependency Upload & Remap` A4, F14,
/// §12 item 3) and that splitting one on backslashes would leave a space for
/// an answer that is not coming. **Nothing here leaves that space.** The two
/// values have different origins and this rule reaches one of them.
///
/// **Windows: the backslash.** Microsoft's own *Naming Files, Paths, and
/// Namespaces*, `ms.date` 2024-08-28, read 2026-09-05 at
/// `https://learn.microsoft.com/en-us/windows/win32/fileio/naming-a-file`:
/// *"Use a backslash (\\) to separate the components of a path. The backslash
/// divides the file name from the path to it, and one directory name from
/// another directory name in a path. You cannot use a backslash in the name
/// for the actual file or directory because it is a reserved character that
/// separates the names into components."* And: *"The path to a specified file
/// consists of one or more components, separated by a special character (a
/// backslash)"*. The same page lists the forward slash among the characters a
/// name may not use, so on Windows both bytes are separators and neither can
/// sit inside a name.
///
/// **macOS and Linux: the forward slash alone.** A backslash is an ordinary
/// byte in a name on those two, so it is not a separator there — which is why
/// this is one rule per system rather than one rule for all three.
#[cfg(windows)]
const THE_LOCAL_SEPARATORS: &[u8] = &[b'\\', SEPARATOR];

#[cfg(not(windows))]
const THE_LOCAL_SEPARATORS: &[u8] = &[SEPARATOR];

/// **G3a: a recorded location, exactly as the reading carried it.** Bytes,
/// compared as bytes, drawn by whatever the screen does over them and never
/// replaced by that.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct RecordedLocation(Vec<u8>);

impl RecordedLocation {
    /// The one constructor: the bytes the reading's document carried for the
    /// reference. Nothing decodes, substitutes or drops a byte.
    pub fn as_the_reading_carried_it(bytes: &[u8]) -> RecordedLocation {
        RecordedLocation(bytes.to_vec())
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// **The name the reference wants — its final segment** — which is what
    /// F9 compares an enumerated stored name against. **For a file.** A
    /// folder-shaped reference wants [`RecordedLocation::final_named_segment`]
    /// instead (F28), and this one is left exactly as it was for files.
    pub fn wanted_name(&self) -> &[u8] {
        last_segment(&self.0)
    }

    /// **F28: the final *named* segment, any trailing separator dropped** —
    /// which is what a folder-shaped reference wants matched against a
    /// directory entry's own name. A location written `//cache/fluid/` wants
    /// `fluid`; under [`RecordedLocation::wanted_name`] it would want the
    /// empty slice and match nothing on any disk.
    pub fn final_named_segment(&self) -> &[u8] {
        last_named_segment(&self.0)
    }
}

/// **F12: a location on the user's own computer as stored on disk**, read
/// from the system in one call and never rejoined from strings. The stored
/// name is what the mirror of 5.2 carries and what the located answer holds,
/// so the file lands on the machine spelled as it really is.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct StoredLocation(Vec<u8>);

impl StoredLocation {
    /// The one constructor: the bytes the system's own stored-name reading
    /// returned (F12's call set on macOS; F14a's normalized name on NTFS and
    /// ReFS; the ordinary enumeration in a folding ext4 directory). **Nothing
    /// here composes one out of a folder and a name it typed.**
    pub fn as_the_system_returned_it(bytes: &[u8]) -> StoredLocation {
        StoredLocation(bytes.to_vec())
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// **F14a: exactness is decided by the stored-name reading's last
    /// component compared against the wanted bytes** — *"exactness is decided
    /// only by the normalized name's last component"* — and this is where that
    /// last component is taken. **The value is a whole location on the user's
    /// own computer (F12), so it splits at that computer's own separators**
    /// ([`THE_LOCAL_SEPARATORS`]) and not at the machine's.
    ///
    /// **Until 2026-09-05 it split at the machine's separator on every
    /// system**, so on Windows — where the normalized name comes back
    /// backslash-separated — the whole location was compared against the
    /// wanted name. Nothing was ever exact: a file asked for by its own
    /// spelling came back a proposal, which is F9's match failing on every
    /// reference on that system.
    pub fn final_segment(&self) -> &[u8] {
        last_local_segment(&self.0)
    }

    /// The folder the location sits in — everything before its final
    /// segment. Used by F17 to resolve a relative location inside a file whose
    /// source folder is known, **and by G6's bake-output test (DW0-2), which
    /// compares**: a bake output path written in full is judged by walking up
    /// from the `.blend`'s own folder with this and comparing each folder with
    /// the resolved one (`smoozy-dependencies`' `check::the_bake_output_path_makes_no_row`).
    /// *(Corrected 2026-09-29, step 10c's dependency wave piece 2: this said
    /// *"by nothing that compares"*, which stopped being true at piece 1.)*
    /// **Split at the user's own computer's separators**, for the same reason
    /// [`StoredLocation::final_segment`] is.
    pub fn folder(&self) -> Vec<u8> {
        local_folder_of(&self.0)
    }
}

/// A location on the rented machine — a mirror, a fixed copy, a job's remap
/// material, or a file already there by another route. Composed by this
/// module under Section 5's rules or read back from a record it wrote;
/// **never derived from a listing of the machine.**
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct MachineLocation(Vec<u8>);

impl MachineLocation {
    pub fn composed(bytes: Vec<u8>) -> MachineLocation {
        MachineLocation(bytes)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

/// The whole segments of a path, in order, empty ones dropped — the unit F16's
/// suffix rule and G11a's ordering both compare. **Bytes throughout.**
pub fn segments(path: &[u8]) -> Vec<&[u8]> {
    path.split(|byte| *byte == SEPARATOR)
        .filter(|segment| !segment.is_empty())
        .collect()
}

/// The final segment, or the whole value where it carries no separator —
/// *"a location carrying no separator byte is one segment and compares
/// whole"* (G11a).
pub fn last_segment(path: &[u8]) -> &[u8] {
    match path.iter().rposition(|byte| *byte == SEPARATOR) {
        Some(at) => &path[at + 1..],
        None => path,
    }
}

/// **F28: the final segment of a path read as a folder's name** — every
/// trailing separator dropped first, so `//cache/fluid/` and `//cache/fluid`
/// name the same folder. **Only for a folder-shaped reference**;
/// [`last_segment`] is what a file's name is and is untouched.
pub fn last_named_segment(path: &[u8]) -> &[u8] {
    last_segment(without_trailing_separators(path))
}

/// A path with every trailing separator dropped. The root is all separators
/// and comes back empty, which names no folder and matches nothing.
pub fn without_trailing_separators(path: &[u8]) -> &[u8] {
    let mut end = path.len();
    while end > 0 && path[end - 1] == SEPARATOR {
        end -= 1;
    }
    &path[..end]
}

/// **The final component of a location on the user's own computer**, split at
/// that computer's own separators ([`THE_LOCAL_SEPARATORS`]), or the whole
/// value where it carries none. **Only for a [`StoredLocation`]**;
/// [`last_segment`] is what every other value in this module splits by and it
/// is untouched.
fn last_local_segment(path: &[u8]) -> &[u8] {
    match path
        .iter()
        .rposition(|byte| THE_LOCAL_SEPARATORS.contains(byte))
    {
        Some(at) => &path[at + 1..],
        None => path,
    }
}

/// Everything before that final component. The separator kept for a location
/// standing at the top is the one that was found, never a substituted byte.
fn local_folder_of(path: &[u8]) -> Vec<u8> {
    match path
        .iter()
        .rposition(|byte| THE_LOCAL_SEPARATORS.contains(byte))
    {
        Some(0) => vec![path[0]],
        Some(at) => path[..at].to_vec(),
        None => Vec::new(),
    }
}

/// Everything before the final segment, without the trailing separator.
pub fn folder_of(path: &[u8]) -> Vec<u8> {
    match path.iter().rposition(|byte| *byte == SEPARATOR) {
        Some(0) => vec![SEPARATOR],
        Some(at) => path[..at].to_vec(),
        None => Vec::new(),
    }
}

/// Joins a folder and a name with one separator, adding none where the folder
/// already ends in one. A composition, never a comparison.
pub fn join(folder: &[u8], name: &[u8]) -> Vec<u8> {
    let mut out = folder.to_vec();
    if !out.is_empty() && !out.ends_with(&[SEPARATOR]) {
        out.push(SEPARATOR);
    }
    out.extend_from_slice(name);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn segments_drop_empties_and_keep_order() {
        assert_eq!(
            segments(b"/Users/alice//tex/x.png"),
            vec![&b"Users"[..], b"alice", b"tex", b"x.png"]
        );
        assert_eq!(segments(b"x.png"), vec![&b"x.png"[..]]);
    }

    #[test]
    fn the_wanted_name_is_the_final_segment_and_a_bare_name_is_itself() {
        assert_eq!(
            RecordedLocation::as_the_reading_carried_it(b"//tex/METallic.png").wanted_name(),
            b"METallic.png"
        );
        assert_eq!(
            RecordedLocation::as_the_reading_carried_it(b"metallic.png").wanted_name(),
            b"metallic.png"
        );
    }

    #[test]
    fn a_folder_shaped_reference_wants_its_final_named_segment_with_a_trailing_separator_dropped() {
        let with = RecordedLocation::as_the_reading_carried_it(b"//cache/fluid/");
        let without = RecordedLocation::as_the_reading_carried_it(b"//cache/fluid");
        assert_eq!(with.final_named_segment(), b"fluid");
        assert_eq!(without.final_named_segment(), b"fluid");
        // This is the whole of the fault: the file reading answers nothing
        // for the trailing-separator form.
        assert_eq!(with.wanted_name(), b"");
        assert_eq!(last_named_segment(b"/"), b"");
        assert_eq!(last_named_segment(b"fluid//"), b"fluid");
        assert_eq!(without_trailing_separators(b"/a/b//"), b"/a/b");
    }

    #[test]
    fn a_recorded_location_keeps_bytes_that_are_not_text() {
        let raw = [b'/', 0xff, 0xfe, b'/', b'x', b'.', b'p', b'n', b'g'];
        let location = RecordedLocation::as_the_reading_carried_it(&raw);
        assert_eq!(location.as_bytes(), &raw);
        assert_eq!(location.wanted_name(), b"x.png");
    }

    /// **F14a's last component, taken off a whole location the local system
    /// returned.** The two systems split differently and each is asserted on
    /// itself, because a backslash is a name's own byte on one and a separator
    /// on the other.
    #[test]
    fn a_stored_locations_last_component_is_taken_at_this_computers_own_separator() {
        let unix = StoredLocation::as_the_system_returned_it(b"/tmp/scratch/METallic.png");
        assert_eq!(unix.final_segment(), b"METallic.png");
        assert_eq!(unix.folder(), b"/tmp/scratch");
        assert_eq!(
            StoredLocation::as_the_system_returned_it(b"/x.png").folder(),
            b"/"
        );
        assert_eq!(
            StoredLocation::as_the_system_returned_it(b"METallic.png").final_segment(),
            b"METallic.png"
        );
    }

    /// **The Windows half of the same rule, and this is the test the hosted
    /// run of 2026-09-05 would have needed.** A normalized name comes back
    /// backslash-separated, so its last component is what follows the last
    /// backslash — not the whole location, which is what was compared before.
    #[cfg(windows)]
    #[test]
    fn a_windows_stored_locations_last_component_is_taken_at_the_backslash() {
        let stored = StoredLocation::as_the_system_returned_it(
            br"\Users\runneradmin\AppData\Local\Temp\smoozy\METallic.png",
        );
        assert_eq!(stored.final_segment(), b"METallic.png");
        assert_eq!(
            stored.folder(),
            br"\Users\runneradmin\AppData\Local\Temp\smoozy"
        );
        assert_eq!(
            StoredLocation::as_the_system_returned_it(br"\x.png").folder(),
            br"\"
        );
    }

    /// **On the two unix systems a backslash is a name's own byte and is not
    /// a separator**, which is the half the Windows rule must not cost.
    #[cfg(not(windows))]
    #[test]
    fn a_backslash_stays_inside_a_name_on_this_computer() {
        let stored = StoredLocation::as_the_system_returned_it(br"/tmp/odd\name.png");
        assert_eq!(stored.final_segment(), br"odd\name.png");
        assert_eq!(stored.folder(), b"/tmp");
    }

    /// **A recorded location is untouched by any of this**, whatever system
    /// this runs on: round 1's Verdict dropped the backslash split for that
    /// value, and a Windows-authored recorded location stays permanently
    /// unresolved (§12 item 3).
    #[test]
    fn a_recorded_location_never_splits_at_a_backslash() {
        assert_eq!(
            RecordedLocation::as_the_reading_carried_it(br"C:\tex\METallic.png").wanted_name(),
            br"C:\tex\METallic.png"
        );
    }

    #[test]
    fn folder_and_join_are_compositions_over_bytes() {
        assert_eq!(folder_of(b"/Users/alice/x.png"), b"/Users/alice");
        assert_eq!(folder_of(b"/x.png"), b"/");
        assert_eq!(folder_of(b"x.png"), b"");
        assert_eq!(join(b"/Users/alice", b"x.png"), b"/Users/alice/x.png");
        assert_eq!(join(b"/", b"x.png"), b"/x.png");
    }
}
