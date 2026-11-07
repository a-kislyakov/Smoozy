//! **M8b: the folder a render opens its file from, so the file's libraries load
//! with the file.** Values only: what to make, and where. The making is the
//! machine's (`smoozy-rendering-machine`'s `acting::opening`), because this crate
//! reads no disk.
//!
//! **Why a folder and not the pairs alone.** A file's own start scripts run as
//! the file loads, before anything Smoozy gives Blender can run. Where the
//! libraries are missing at that moment and arrive afterwards by M8's reload, the
//! scripts inside them never run, a script in the top file finds nothing to act
//! on, and what a script did set is thrown away by the reload. The render finishes
//! at success with a different picture. So a library recorded relative to the file
//! is given a link at exactly that relative place, the file is opened through a
//! link beside it, and Blender loads the whole tree at load as it does on the
//! artist's computer. **The recorded text stays as the file holds it**, which a
//! script naming a library by that text also needs.
//!
//! **What cannot be helped here**: a library recorded as a whole location, which
//! has no relative place to stand at. It keeps M8's assignment and reload.

use crate::bytes::{folder_of, join, last_segment};
use crate::check::Kind;
use crate::remap::Pair;

/// **The name of each folder standing for a step above the file's own**, for a
/// library recorded through `..`. Nothing reads it; a script that looks up that far
/// sees this name rather than the real folder's.
const A_STEP_ABOVE: &[u8] = b"above";

/// **What to make, in the order it is made.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheOpeningFolder {
    /// The folder as a whole. A repeat attempt makes it afresh.
    pub root: Vec<u8>,
    /// **What the launch line opens** — a link to the job's file, under the file's
    /// own name, so everything that goes by the name reads as it did.
    pub the_file: Vec<u8>,
    /// Each link's place, and what it points at: the file, and each library.
    pub links: Vec<(Vec<u8>, Vec<u8>)>,
    /// **Each folder here with the real folder it stands for**, the file's own
    /// first and then each step above it. Whatever really stands in the real
    /// folder gets a link here too, so a location no pair names reads as it did
    /// from the file's real folder.
    pub stands_for: Vec<(Vec<u8>, Vec<u8>)>,
}

/// **The steps of a recorded location below `//`**, or nothing where it is not
/// relative to the file or names no file. `.` and doubled separators fall out, as
/// they do when Blender reads the path.
fn the_steps_of(match_text: &[u8]) -> Option<Vec<&[u8]>> {
    let below = match_text.strip_prefix(b"//")?;
    let steps: Vec<&[u8]> = below
        .split(|byte| *byte == b'/')
        .filter(|step| !step.is_empty() && *step != b".")
        .collect();
    match steps.last() {
        Some(last) if *last != b".." => Some(steps),
        _ => None,
    }
}

/// **M8b: the folder for these pairs**, under `root`, or nothing where no library
/// pair has a relative place — the file then opens where it sits, as it always has.
///
/// **Only library pairs.** Every other pair is assigned after the load as before
/// (M9), which is proved for the image kind by render.
pub fn the_folder_the_file_opens_from(
    pairs: &[Pair],
    the_files_path: &[u8],
    root: &[u8],
) -> Option<TheOpeningFolder> {
    let libraries: Vec<(Vec<&[u8]>, &[u8])> = pairs
        .iter()
        .filter(|pair| pair.kind == Kind::Library)
        .filter_map(|pair| Some((the_steps_of(pair.match_text.as_bytes())?, pair.new_location.as_bytes())))
        .collect();
    if libraries.is_empty() {
        return None;
    }

    // **How far up any library reaches**, so a `..` never leaves the folder.
    let mut how_far_up = 0usize;
    for (steps, _) in &libraries {
        let mut depth = 0isize;
        for step in steps {
            if *step == b".." {
                depth -= 1;
                how_far_up = how_far_up.max(depth.unsigned_abs());
            } else {
                depth += 1;
            }
        }
    }
    let mut the_files_own_folder = root.to_vec();
    for _ in 0..how_far_up {
        the_files_own_folder = join(&the_files_own_folder, A_STEP_ABOVE);
    }

    let the_file = join(&the_files_own_folder, last_segment(the_files_path));
    let mut links = vec![(the_file.clone(), the_files_path.to_vec())];
    for (steps, to) in &libraries {
        let mut place: Vec<Vec<u8>> = vec![A_STEP_ABOVE.to_vec(); how_far_up];
        for step in steps {
            if *step == b".." {
                place.pop();
            } else {
                place.push(step.to_vec());
            }
        }
        let mut at = root.to_vec();
        for step in &place {
            at = join(&at, step);
        }
        // **One place can hold one thing.** Two recorded locations that come to
        // one place, or a library standing where another's folder has to go, leave
        // the later one to M8's assignment and reload.
        let a_place_taken = links.iter().any(|(taken, _)| {
            *taken == at
                || taken.strip_prefix(at.as_slice()).is_some_and(|rest| rest.first() == Some(&b'/'))
                || at.strip_prefix(taken.as_slice()).is_some_and(|rest| rest.first() == Some(&b'/'))
        });
        if !a_place_taken {
            links.push((at, to.to_vec()));
        }
    }

    let mut stands_for = Vec::new();
    let mut here = the_files_own_folder;
    let mut real = folder_of(the_files_path);
    for _ in 0..=how_far_up {
        stands_for.push((here.clone(), real.clone()));
        here = folder_of(&here);
        real = folder_of(&real);
    }

    Some(TheOpeningFolder {
        root: root.to_vec(),
        the_file,
        links,
        stands_for,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bytes::{MachineLocation, RecordedLocation};
    use crate::remap::{compose_pair, NewLocation};

    fn a_library(recorded: &[u8], fixed: &[u8]) -> Pair {
        compose_pair(
            Kind::Library,
            "filepath",
            &RecordedLocation::as_the_reading_carried_it(recorded),
            NewLocation::AFixedCopy(MachineLocation::composed(fixed.to_vec())),
        )
        .unwrap()
    }

    fn an_image(recorded: &[u8]) -> Pair {
        compose_pair(
            Kind::Image,
            "filepath",
            &RecordedLocation::as_the_reading_carried_it(recorded),
            NewLocation::AMirror(MachineLocation::composed(b"/m/a.png".to_vec())),
        )
        .unwrap()
    }

    #[test]
    fn a_library_is_linked_at_its_recorded_place_beside_a_link_to_the_file() {
        let folder = the_folder_the_file_opens_from(
            &[
                a_library(b"//lib/rig.blend", b"/fixed/lib/rig.blend"),
                a_library(b"//set.blend", b"/fixed/set.blend"),
            ],
            b"/home/u/scenes/shot.blend",
            b"/r/opening-1",
        )
        .unwrap();
        assert_eq!(folder.the_file, b"/r/opening-1/shot.blend");
        assert_eq!(
            folder.links,
            vec![
                (b"/r/opening-1/shot.blend".to_vec(), b"/home/u/scenes/shot.blend".to_vec()),
                (b"/r/opening-1/lib/rig.blend".to_vec(), b"/fixed/lib/rig.blend".to_vec()),
                (b"/r/opening-1/set.blend".to_vec(), b"/fixed/set.blend".to_vec()),
            ]
        );
        assert_eq!(
            folder.stands_for,
            vec![(b"/r/opening-1".to_vec(), b"/home/u/scenes".to_vec())]
        );
    }

    /// **A `..` never leaves the folder**: the file sits as many steps down as the
    /// farthest library reaches up.
    #[test]
    fn a_library_above_the_file_stays_inside_the_folder() {
        let folder = the_folder_the_file_opens_from(
            &[
                a_library(b"//../../shared/a.blend", b"/fixed/a.blend"),
                a_library(b"//../b.blend", b"/fixed/b.blend"),
                a_library(b"//c/../d.blend", b"/fixed/d.blend"),
            ],
            b"/home/u/scenes/shot.blend",
            b"/r/o",
        )
        .unwrap();
        assert_eq!(folder.the_file, b"/r/o/above/above/shot.blend");
        let places: Vec<&[u8]> = folder.links.iter().map(|(at, _)| at.as_slice()).collect();
        assert_eq!(
            places,
            vec![
                &b"/r/o/above/above/shot.blend"[..],
                b"/r/o/shared/a.blend",
                b"/r/o/above/b.blend",
                b"/r/o/above/above/d.blend",
            ]
        );
        assert_eq!(
            folder.stands_for,
            vec![
                (b"/r/o/above/above".to_vec(), b"/home/u/scenes".to_vec()),
                (b"/r/o/above".to_vec(), b"/home/u".to_vec()),
                (b"/r/o".to_vec(), b"/home".to_vec()),
            ]
        );
    }

    /// **Nothing to do where nothing has a relative place**: no library pair, or
    /// every one recorded as a whole location. The file opens where it sits.
    #[test]
    fn no_relative_library_means_no_folder() {
        assert_eq!(the_folder_the_file_opens_from(&[], b"/s/a.blend", b"/r/o"), None);
        assert_eq!(
            the_folder_the_file_opens_from(&[an_image(b"//a.png")], b"/s/a.blend", b"/r/o"),
            None
        );
        assert_eq!(
            the_folder_the_file_opens_from(
                &[a_library(b"/Users/a/lib.blend", b"/fixed/lib.blend"), a_library(b"C:/lib.blend", b"/fixed/c.blend")],
                b"/s/a.blend",
                b"/r/o"
            ),
            None
        );
    }

    /// **M4a's rule reaches here**: the pair's text has forward separators where
    /// the file was saved on Windows, so the place is the one Blender reads.
    #[test]
    fn a_location_saved_on_windows_has_one_place() {
        let folder = the_folder_the_file_opens_from(
            &[a_library(b"//lib\\rig.blend", b"/fixed/rig.blend")],
            b"/s/a.blend",
            b"/r/o",
        )
        .unwrap();
        assert_eq!(folder.links[1].0, b"/r/o/lib/rig.blend");
    }

    /// **One place holds one thing**, and the library left out keeps M8's route.
    #[test]
    fn two_things_at_one_place_keep_the_first() {
        let folder = the_folder_the_file_opens_from(
            &[
                a_library(b"//a.blend", b"/fixed/1.blend"),
                a_library(b"//./a.blend", b"/fixed/2.blend"),
                a_library(b"//a.blend/b.blend", b"/fixed/3.blend"),
                a_library(b"//shot.blend", b"/fixed/4.blend"),
            ],
            b"/s/shot.blend",
            b"/r/o",
        )
        .unwrap();
        assert_eq!(folder.links.len(), 2);
        assert_eq!(folder.links[1], (b"/r/o/a.blend".to_vec(), b"/fixed/1.blend".to_vec()));
    }
}
