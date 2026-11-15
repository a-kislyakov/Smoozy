//! The kinds a reference can be — `Dependency Upload & Remap` §3's list, with
//! the import-node path and the folder-shaped kinds. **Everything else of
//! the check is not here**: its judgements stay in `smoozy-dependencies`'
//! `check`, which re-exports [`Kind`] at the path it always had. The kind
//! moved because the remap material's pairs carry one.

use serde::{Deserialize, Serialize};

/// **G6: the kinds enumerated are the spec's list**, plus G8's import-node
/// path, read off the node itself in every version. **Write-only destinations
/// are not rows** — they are `WriteOnlyDestination`, which stayed in
/// `smoozy_dependencies::check`, and the redirect's.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Kind {
    Library,
    /// The weak reference kept after appending.
    LibraryWeakReference,
    Image,
    MovieClip,
    Sound,
    Font,
    Volume,
    CacheArchive,
    CacheLayer,
    Text,
    OslScript,
    IesProfile,
    /// **The spec names two modifiers individually** — *"the external
    /// displacement files of the Multires and Mesh Cache modifiers"* — and
    /// one variant covering both hid half the kind: the script's walk had no
    /// Mesh Cache branch, and the guard that holds every kind against the
    /// script passed on the Multires half. Split 2026-09-05, out of round 1
    /// of this module's code audit.
    MultiresExternalDisplacement,
    MeshCacheExternalDisplacement,
    PhysicsPointCache,
    FluidCacheFolder,
    GeometryNodeBakeFolder,
    /// The bake output path read as a place a baked result already sits —
    /// `Job files and outputs` D20's first write place, a row where a baked
    /// result exists to send (G6), and one of M19's seven inferred kinds.
    BakeFolder,
    OceanBakeFolder,
    DynamicPaintOutputFolder,
    SequencerStripFolder,
    SequencerStripFile,
    ProxyFolder,
    BrushIcon,
    /// The record kept with a packed file.
    PackedFileRecord,
    /// **G8: an import node's path, read off the node itself** — six kinds of
    /// import node in 4.5 to 5.2 and seven in 5.3, plus the sound route in 5.2
    /// and 5.3. In 4.5 that path takes no part in the file's own path
    /// housekeeping, so the node walk is the only route to it.
    ImportNodePath,
}

impl Kind {
    /// **Every kind, so a walk over them is not a list somebody keeps.**
    pub const ALL: [Kind; 26] = [
        Kind::Library,
        Kind::LibraryWeakReference,
        Kind::Image,
        Kind::MovieClip,
        Kind::Sound,
        Kind::Font,
        Kind::Volume,
        Kind::CacheArchive,
        Kind::CacheLayer,
        Kind::Text,
        Kind::OslScript,
        Kind::IesProfile,
        Kind::MultiresExternalDisplacement,
        Kind::MeshCacheExternalDisplacement,
        Kind::PhysicsPointCache,
        Kind::FluidCacheFolder,
        Kind::GeometryNodeBakeFolder,
        Kind::BakeFolder,
        Kind::OceanBakeFolder,
        Kind::DynamicPaintOutputFolder,
        Kind::SequencerStripFolder,
        Kind::SequencerStripFile,
        Kind::ProxyFolder,
        Kind::BrushIcon,
        Kind::PackedFileRecord,
        Kind::ImportNodePath,
    ];

    /// **G6: a recorded write place that is also read — a cache or a bake —
    /// is a row where a baked result exists to send**, per the spec's
    /// found-and-sent rule; where its uploaded copy is read from is M22a's.
    pub fn is_a_write_place_that_is_also_read(self) -> bool {
        matches!(
            self,
            Kind::PhysicsPointCache
                | Kind::FluidCacheFolder
                | Kind::GeometryNodeBakeFolder
                | Kind::BakeFolder
                | Kind::OceanBakeFolder
                | Kind::DynamicPaintOutputFolder
                | Kind::ProxyFolder
        )
    }

    /// **F28: the kinds whose recorded location names a folder rather than a
    /// file.** G6's folder-shaped kinds — a physics point cache, a fluid
    /// cache folder, a geometry-node bake folder, an ocean bake folder, a
    /// dynamic-paint output folder, a sequencer strip folder and a proxy
    /// folder — with the bake output path beside them, G6 making that a row
    /// on the Creator's ruling of 2026-08-22 (M22a).
    ///
    /// **A reference of one of these is matched on the final *named* segment
    /// of its recorded location** against a directory entry's own name
    /// ([`crate::bytes::RecordedLocation::final_named_segment`]), under F9's
    /// byte equality and F10's fold exactly as a file is. Everything else on
    /// G6's list names a file.
    pub fn names_a_folder(self) -> bool {
        matches!(
            self,
            Kind::PhysicsPointCache
                | Kind::FluidCacheFolder
                | Kind::GeometryNodeBakeFolder
                | Kind::BakeFolder
                | Kind::OceanBakeFolder
                | Kind::DynamicPaintOutputFolder
                | Kind::SequencerStripFolder
                | Kind::ProxyFolder
        )
    }
}

// ------------------------------------------------- a picture stored as a sequence
//
// **`Dependency Upload & Remap`, *Colour spaces and image sequences*, with G9 and
// G22.** Every rule here is observed, in 4.5, 5.0, 5.1, 5.2 and 5.3 on the
// Creator's Mac, in `Audits/Blender Research/Colour spaces and image
// sequences/Hand tests — 5.3.md` (tests 4 to 6), and written into
// `Preparation/Blender Research/Findings/5.3/Colour spaces and image
// sequences.md`, questions 3 to 5. Where a case was not tested, nothing is worked
// out and the stored name stands for it, as it did before.

/// **One node showing a sequence, as the reading carries it**: an Image Texture
/// node's image user, or a compositor Image node's own settings — `frame_start`,
/// `frame_offset`, `frame_duration`, `use_cyclic`, the names test 5 set in all
/// five versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ASequencesUser {
    pub start: i64,
    pub offset: i64,
    pub frames: i64,
    pub cyclic: bool,
}

/// **Every node the reading found showing a picture stored as `SEQUENCE`**, and
/// whether any other node names it. A user outside a node tree is not seen.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TheSequencesUsers {
    pub users: Vec<ASequencesUser>,
    #[serde(default)]
    pub other_users: bool,
}

/// **Where the frame number sits in a name: its last run of digits**, as
/// (where it starts, how long it is). Test 4: `s_0001_v2.png` read `s_0001_v1.png`
/// at frame 1, `shot2_0001.png` read `shot2_0002.png` at frame 2.
///
/// **Nothing for a name the tests did not reach**, which stays as it was: no
/// digits at all (Blender reads `plain1.png`, test 4), a set's own marks (`#`,
/// a tile token, `%`; Blender reads `b_####1.png` for a `#` name), and digits in
/// the ending, which no test named.
fn the_frame_numbers_place(name: &[u8]) -> Option<(usize, usize)> {
    if name.contains(&b'#') || name.contains(&b'%') || name.contains(&b'<') {
        return None;
    }
    let stem = match name.iter().rposition(|byte| *byte == b'.') {
        Some(at) if at > 0 => {
            if name[at..].iter().any(u8::is_ascii_digit) {
                return None;
            }
            &name[..at]
        }
        _ => name,
    };
    let end = stem.iter().rposition(u8::is_ascii_digit)? + 1;
    let start = stem[..end].iter().rposition(|byte| !byte.is_ascii_digit()).map_or(0, |at| at + 1);
    Some((start, end - start))
}

/// **G22's pattern for a picture stored as a sequence**: its name with the last
/// run of digits written as that many `#`. G22 matches a run of `#` against a run
/// of at least that many digits, which is test 4's rule: the digit count is kept
/// as the least and a number grows past it (`o_998.png` read `o_1000.png`,
/// `b_0001.png` read `b_10000.png`, `c_0001.png` did not read `c_10.png`).
pub fn the_sequences_pattern(name: &[u8]) -> Option<Vec<u8>> {
    let (start, length) = the_frame_numbers_place(name)?;
    Some([&name[..start], &vec![b'#'; length][..], &name[start + length..]].concat())
}

/// **The file a number reads**: the last run of digits replaced by the number,
/// written with at least that many digits, and a number below nought read as
/// nought (test 4's renders, test 6's `filepath_from_user` at each
/// `frame_current`, -1 reading `q_0000.png`).
pub fn the_file_a_number_reads(name: &[u8], number: i64) -> Option<Vec<u8>> {
    let (start, length) = the_frame_numbers_place(name)?;
    let digits = format!("{:0length$}", number.max(0));
    Some([&name[..start], digits.as_bytes(), &name[start + length..]].concat())
}

/// **Which number a user reads at a scene frame**, test 5 frame by frame in a
/// material and in the compositor alike: where Frames is 0, nought (`k3`, `k3c`);
/// otherwise p = frame − Start + 1, wrapped into 1 to Frames with Cyclic (nought
/// reading Frames) and held between nought and Frames without it, then Offset
/// added (`k1`, `k2`, `k4`, `k6` to `k9`).
///
/// **Nothing where the tests did not reach**: Frames 0 with an Offset, Cyclic
/// with a negative Offset, and Frames below nought.
pub fn the_number_a_frame_reads(user: &ASequencesUser, frame: i64) -> Option<i64> {
    if user.frames == 0 {
        return (user.offset == 0).then_some(0);
    }
    if user.frames < 0 || (user.cyclic && user.offset < 0) {
        return None;
    }
    let mut at = frame.saturating_sub(user.start).saturating_add(1);
    if user.cyclic {
        at = at.rem_euclid(user.frames);
        if at == 0 {
            at = user.frames;
        }
    } else {
        at = at.clamp(0, user.frames);
    }
    Some(at.saturating_add(user.offset).max(0))
}

/// **The files a sequence needs over the job's frames**: every user's number at
/// every frame, as its file; **and the stored name itself** where another node
/// names the picture, where a user's number is not worked out, or where no user
/// was seen — what the check did before for each of those. Nothing for a name the
/// rule does not reach.
pub fn the_files_a_sequence_needs(
    stored_name: &[u8],
    users: &TheSequencesUsers,
    frames: &[i64],
) -> Option<std::collections::BTreeSet<Vec<u8>>> {
    the_frame_numbers_place(stored_name)?;
    let mut numbers = std::collections::BTreeSet::new();
    let mut the_stored_name_too = users.other_users || users.users.is_empty();
    for user in &users.users {
        for frame in frames {
            match the_number_a_frame_reads(user, *frame) {
                Some(number) => {
                    numbers.insert(number);
                }
                None => the_stored_name_too = true,
            }
        }
    }
    let mut out: std::collections::BTreeSet<Vec<u8>> = numbers
        .into_iter()
        .filter_map(|number| the_file_a_number_reads(stored_name, number))
        .collect();
    if the_stored_name_too {
        out.insert(stored_name.to_vec());
    }
    Some(out)
}

// ------------------------------------------- a colour name the Blender lacks

/// Which of the three names Blender said it does not have.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum WhichColourName {
    ColourSpace,
    ViewTransform,
    Display,
}

/// **One name the Blender that read the file does not have**, as it said it.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AColourNameNotFound {
    pub which: WhichColourName,
    pub name: String,
}

/// **The colour names Blender said, as it opened the file, that its
/// configuration does not have** — test 3, in all five versions, and test 2's
/// project in 5.3; read off every stream given, each name once, in the order
/// said. The lines, 5.x then 4.5:
///
/// - `image colorspace "<name>" not found, will use default instead.` (both);
/// - `scene view "<name>" not found, setting to "<default>".` and 4.5's
///   `scene view "<name>" not found, setting default "<default>".`, with 5.3's
///   `scene output view …`;
/// - `Display "<name>" used by scene not found, setting to "<default>".` and
///   4.5's `display "<name>" used by scene not found, setting to default
///   ("<default>").`, with 5.3's `… used by scene output …`.
pub fn the_colour_names_not_found(streams: &[&[u8]]) -> Vec<AColourNameNotFound> {
    let mut out: Vec<AColourNameNotFound> = Vec::new();
    for stream in streams {
        for line in String::from_utf8_lossy(stream).lines() {
            let Some(one) = a_colour_name_in(line) else {
                continue;
            };
            if !out.contains(&one) {
                out.push(one);
            }
        }
    }
    out
}

fn a_colour_name_in(line: &str) -> Option<AColourNameNotFound> {
    let between = |after: &str, before: &str| -> Option<String> {
        let from = line.find(after)? + after.len();
        let to = from + line[from..].rfind(before)?;
        let name = &line[from..to];
        (!name.is_empty()).then(|| name.to_string())
    };
    if let Some(name) = between("image colorspace \"", "\" not found, will use default") {
        return Some(AColourNameNotFound { which: WhichColourName::ColourSpace, name });
    }
    if line.contains("scene view \"") || line.contains("scene output view \"") {
        if let Some(name) = between("view \"", "\" not found, setting") {
            return Some(AColourNameNotFound { which: WhichColourName::ViewTransform, name });
        }
    }
    for opening in ["Display \"", "display \""] {
        if let Some(name) = between(opening, "\" used by scene") {
            return Some(AColourNameNotFound { which: WhichColourName::Display, name });
        }
    }
    None
}

#[cfg(test)]
mod a_sequence_and_a_colour_name {
    use super::*;

    fn user(start: i64, offset: i64, frames: i64, cyclic: bool) -> ASequencesUser {
        ASequencesUser { start, offset, frames, cyclic }
    }

    fn numbers(one: ASequencesUser, frames: std::ops::RangeInclusive<i64>) -> Vec<Option<i64>> {
        frames.map(|frame| the_number_a_frame_reads(&one, frame)).collect()
    }

    /// **Test 4: the frame number takes the last group of digits, keeps its
    /// count as the least, and grows past it.**
    #[test]
    fn the_file_a_number_reads_is_the_last_run_of_digits_replaced() {
        let at = |name: &[u8], number| the_file_a_number_reads(name, number).map(|one| String::from_utf8(one).unwrap());
        assert_eq!(at(b"b_0001.png", 12).as_deref(), Some("b_0012.png"));
        assert_eq!(at(b"b_0001.png", 10000).as_deref(), Some("b_10000.png"));
        assert_eq!(at(b"o_998.png", 1000).as_deref(), Some("o_1000.png"));
        assert_eq!(at(b"o_998.png", 1).as_deref(), Some("o_001.png"));
        assert_eq!(at(b"a_1.png", 10).as_deref(), Some("a_10.png"));
        assert_eq!(at(b"s_0001_v2.png", 1).as_deref(), Some("s_0001_v1.png"));
        assert_eq!(at(b"shot2_0001.png", 2).as_deref(), Some("shot2_0002.png"));
        assert_eq!(at(b"0001_plate.png", 3).as_deref(), Some("0003_plate.png"));
        assert_eq!(at(b"frame.0001.png", 13).as_deref(), Some("frame.0013.png"));
        // Below nought reads as nought (test 6's `frame_current` -1).
        assert_eq!(at(b"q_0001.png", -2).as_deref(), Some("q_0000.png"));
        // Stays as before: no digits, a `#`, digits in the ending.
        assert_eq!(at(b"plain.png", 1), None);
        assert_eq!(at(b"b_####.png", 1), None);
        assert_eq!(at(b"x_0001.jp2", 1), None);
    }

    #[test]
    fn a_sequences_pattern_writes_its_last_run_of_digits_as_that_many_marks() {
        let pattern = |name: &[u8]| the_sequences_pattern(name).map(|one| String::from_utf8(one).unwrap());
        assert_eq!(pattern(b"b_0001.png").as_deref(), Some("b_####.png"));
        assert_eq!(pattern(b"a_1.png").as_deref(), Some("a_#.png"));
        assert_eq!(pattern(b"s_0001_v2.png").as_deref(), Some("s_0001_v#.png"));
        assert_eq!(pattern(b"plain.png"), None);
        assert_eq!(pattern(b"b_####.png"), None);
        assert_eq!(pattern(b"tile.<UDIM>.png"), None);
        assert_eq!(pattern(b"view_%_0001.png"), None);
        assert_eq!(pattern(b"x_0001.jp2"), None);
    }

    /// **Test 5, frame by frame**, `k1` to `k8` (material and compositor
    /// alike).
    #[test]
    fn the_number_each_frame_reads_is_test_fives() {
        let s = |list: &[i64]| list.iter().map(|one| Some(*one)).collect::<Vec<_>>();
        // k1: Start 5, Offset 2, Frames 6.
        assert_eq!(numbers(user(5, 2, 6, false), 1..=15), s(&[2, 2, 2, 2, 3, 4, 5, 6, 7, 8, 8, 8, 8, 8, 8]));
        // k2: the same, Cyclic.
        assert_eq!(numbers(user(5, 2, 6, true), 1..=15), s(&[5, 6, 7, 8, 3, 4, 5, 6, 7, 8, 3, 4, 5, 6, 7]));
        // k3 and k3c: Frames 0.
        assert_eq!(numbers(user(5, 0, 0, false), 1..=15), s(&[0; 15]));
        assert_eq!(numbers(user(5, 0, 0, true), 1..=15), s(&[0; 15]));
        // k4: Offset -2.
        assert_eq!(numbers(user(5, -2, 6, false), 1..=15), s(&[0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 4, 4, 4, 4, 4]));
        // k6, k7, k8: Start 1.
        assert_eq!(numbers(user(1, 0, 6, false), 1..=8), s(&[1, 2, 3, 4, 5, 6, 6, 6]));
        // k9's right half: Offset 5.
        assert_eq!(numbers(user(1, 5, 20, false), 1..=3), s(&[6, 7, 8]));
        // Not tested, so not worked out: Frames 0 with an Offset, Cyclic with
        // a negative Offset.
        assert_eq!(the_number_a_frame_reads(&user(5, 2, 0, false), 1), None);
        assert_eq!(the_number_a_frame_reads(&user(5, -2, 6, true), 1), None);
    }

    #[test]
    fn the_files_a_sequence_needs_are_every_users_over_every_frame() {
        let names = |found: Option<std::collections::BTreeSet<Vec<u8>>>| {
            found.map(|set| set.into_iter().map(|one| String::from_utf8(one).unwrap()).collect::<Vec<_>>())
        };
        let users = TheSequencesUsers { users: vec![user(1, 0, 6, false), user(1, 5, 20, false)], other_users: false };
        assert_eq!(
            names(the_files_a_sequence_needs(b"q_0005.png", &users, &[1, 2])).unwrap(),
            vec!["q_0001.png", "q_0002.png", "q_0006.png", "q_0007.png"]
        );
        // Another node names it: the stored name stands for it too.
        let other = TheSequencesUsers { users: vec![user(1, 0, 6, false)], other_users: true };
        assert_eq!(
            names(the_files_a_sequence_needs(b"q_0005.png", &other, &[1])).unwrap(),
            vec!["q_0001.png", "q_0005.png"]
        );
        // No user seen, or one not worked out: the stored name.
        let none = TheSequencesUsers { users: vec![], other_users: false };
        assert_eq!(names(the_files_a_sequence_needs(b"q_0005.png", &none, &[1])).unwrap(), vec!["q_0005.png"]);
        let untested = TheSequencesUsers { users: vec![user(1, 3, 0, false)], other_users: false };
        assert_eq!(names(the_files_a_sequence_needs(b"q_0005.png", &untested, &[1])).unwrap(), vec!["q_0005.png"]);
        // A name the rule does not reach: nothing.
        assert_eq!(names(the_files_a_sequence_needs(b"plain.png", &users, &[1])), None);
    }

    /// **Test 3's lines, as 5.3 and 4.5 printed them.**
    #[test]
    fn the_colour_names_blender_said_it_does_not_have_are_read_off_both_streams() {
        let ordinary = b"Read blend: x\n00:00.368  color_management | WARNING image colorspace \"Smoozy Orphan\" not found, will use default instead.\n<<<SMOOZY-DEPENDENCIES>>>{}<<<END-SMOOZY-DEPENDENCIES>>>\n";
        let complaints = concat!(
            "00:00.338  color_management | WARNING scene view \"Smoozy View\" not found, setting to \"Standard\".\n",
            "00:00.357  color_management | WARNING Display \"Smoozy Display\" used by scene not found, setting to \"sRGB\".\n",
            "Color management: image colorspace \"Smoozy Orphan\" not found, will use default instead.\n",
            "Color management: scene view \"Old View\" not found, setting default \"Standard\".\n",
            "Color management: display \"Old Display\" used by scene not found, setting to default (\"sRGB\").\n",
            "00:00.343  color_management | WARNING scene output view \"Standard\" not found, setting to \"Un-tone-mapped\".\n",
            "00:00.343  color_management | WARNING Display \"sRGB\" used by scene output not found, setting to \"sRGB - Display\".\n",
            "00:00.401  cycles           | WARNING Colorspace  not found, using scene linear instead\n",
        );
        let said = the_colour_names_not_found(&[ordinary.as_slice(), complaints.as_bytes()]);
        let said: Vec<(WhichColourName, &str)> = said.iter().map(|one| (one.which, one.name.as_str())).collect();
        assert_eq!(
            said,
            vec![
                (WhichColourName::ColourSpace, "Smoozy Orphan"),
                (WhichColourName::ViewTransform, "Smoozy View"),
                (WhichColourName::Display, "Smoozy Display"),
                (WhichColourName::ViewTransform, "Old View"),
                (WhichColourName::Display, "Old Display"),
                (WhichColourName::ViewTransform, "Standard"),
                (WhichColourName::Display, "sRGB"),
            ]
        );
        assert!(the_colour_names_not_found(&[b"Blender quit\n".as_slice()]).is_empty());
    }
}
