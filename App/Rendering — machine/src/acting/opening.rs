//! **`Dependency Upload & Remap` M8b, on the machine's own disk: the folder a render
//! opens its file from.** What to make is that document's and is values
//! ([`TheOpeningFolder`]); this makes it.
//!
//! **Links, and nothing here is ever written through.** Nothing Smoozy runs follows
//! a link when it removes a job's records: Rust's own folder removal takes the link
//! and leaves what it points at.

use smoozy_dependencies_machine::opening::TheOpeningFolder;

/// **Makes the folder afresh.** A repeat attempt has its own number and so its own
/// folder; a folder already standing is from a machine restart inside this attempt.
pub fn make_the_opening_folder(folder: &TheOpeningFolder) -> Result<(), String> {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;
    use std::path::Path;

    let at = |bytes: &[u8]| Path::new(OsStr::from_bytes(bytes)).to_path_buf();
    let root = at(&folder.root);
    if std::fs::symlink_metadata(&root).is_ok() {
        std::fs::remove_dir_all(&root).map_err(|why| format!("{}: {why}", root.display()))?;
    }
    std::fs::create_dir_all(&root).map_err(|why| format!("{}: {why}", root.display()))?;
    for (place, to) in &folder.links {
        let place = at(place);
        if let Some(parent) = place.parent() {
            std::fs::create_dir_all(parent).map_err(|why| format!("{}: {why}", parent.display()))?;
        }
        std::os::unix::fs::symlink(at(to), &place).map_err(|why| format!("{}: {why}", place.display()))?;
    }
    for (here, real) in &folder.stands_for {
        let here = at(here);
        std::fs::create_dir_all(&here).map_err(|why| format!("{}: {why}", here.display()))?;
        what_stands_there_is_linked_here(&at(real), &here);
    }
    Ok(())
}

/// **Whatever stands in the real folder gets a link here**, going down only where a
/// library's folder already stands in the way, so a location no pair names reads as
/// it did from the file's real folder. **A folder that cannot be read costs only
/// what it held**: the render reads it or does not, as it would have.
fn what_stands_there_is_linked_here(real: &std::path::Path, here: &std::path::Path) {
    let Ok(entries) = std::fs::read_dir(real) else {
        return;
    };
    for entry in entries.flatten() {
        let there = entry.path();
        let linked = here.join(entry.file_name());
        match std::fs::symlink_metadata(&linked) {
            Ok(standing) if standing.is_dir() => {
                if there.is_dir() {
                    what_stands_there_is_linked_here(&there, &linked);
                }
            }
            Ok(_) => {}
            Err(_) => {
                let _ = std::os::unix::fs::symlink(&there, &linked);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_dependencies_machine::bytes::{MachineLocation, RecordedLocation};
    use smoozy_dependencies_machine::check::Kind;
    use smoozy_dependencies_machine::opening::the_folder_the_file_opens_from;
    use smoozy_dependencies_machine::remap::{compose_pair, NewLocation};
    use std::os::unix::ffi::OsStrExt;
    use std::path::{Path, PathBuf};

    struct Scratch(PathBuf);
    impl Drop for Scratch {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.0).ok();
        }
    }
    fn a_scratch(name: &str) -> Scratch {
        let at = std::env::temp_dir().join(format!("smoozy-opening-{name}-{}", std::process::id()));
        std::fs::remove_dir_all(&at).ok();
        std::fs::create_dir_all(&at).unwrap();
        Scratch(at)
    }
    fn write(at: &Path, text: &str) {
        std::fs::create_dir_all(at.parent().unwrap()).unwrap();
        std::fs::write(at, text).unwrap();
    }
    fn a_library(recorded: &str, fixed: &Path) -> smoozy_dependencies_machine::remap::Pair {
        compose_pair(
            Kind::Library,
            "filepath",
            &RecordedLocation::as_the_reading_carried_it(recorded.as_bytes()),
            NewLocation::AFixedCopy(MachineLocation::composed(fixed.as_os_str().as_bytes().to_vec())),
        )
        .unwrap()
    }

    /// **Through the folder**: the file, a library at its recorded place, what stood
    /// beside the file, and what stood in a folder a library also needs.
    #[test]
    fn through_the_folder_each_recorded_place_reads_what_it_should() {
        let scratch = a_scratch("reads");
        let root = &scratch.0;
        write(&root.join("scenes/shot.blend"), "the file");
        write(&root.join("scenes/lib/rig.blend"), "the user's copy");
        write(&root.join("scenes/lib/beside_the_rig.png"), "a picture in the library's folder");
        write(&root.join("scenes/tex/wood.png"), "a picture beside the file");
        write(&root.join("scenes/settings.json"), "a script's file");
        write(&root.join("elsewhere.png"), "a step above the file");
        write(&root.join("fixed/rig.blend"), "the fixed copy");
        write(&root.join("fixed/set.blend"), "the fixed set");

        let opening = root.join("opening-1");
        let folder = the_folder_the_file_opens_from(
            &[
                a_library("//lib/rig.blend", &root.join("fixed/rig.blend")),
                a_library("//../set.blend", &root.join("fixed/set.blend")),
            ],
            root.join("scenes/shot.blend").as_os_str().as_bytes(),
            opening.as_os_str().as_bytes(),
        )
        .unwrap();
        make_the_opening_folder(&folder).unwrap();

        let opened = Path::new(std::ffi::OsStr::from_bytes(&folder.the_file)).to_path_buf();
        let from_the_file = |relative: &str| {
            std::fs::read_to_string(opened.parent().unwrap().join(relative)).unwrap()
        };
        assert_eq!(std::fs::read_to_string(&opened).unwrap(), "the file");
        assert_eq!(from_the_file("lib/rig.blend"), "the fixed copy");
        assert_eq!(from_the_file("../set.blend"), "the fixed set");
        assert_eq!(from_the_file("lib/beside_the_rig.png"), "a picture in the library's folder");
        assert_eq!(from_the_file("tex/wood.png"), "a picture beside the file");
        assert_eq!(from_the_file("settings.json"), "a script's file");
        assert_eq!(from_the_file("../elsewhere.png"), "a step above the file");

        // **A second making stands the same, and taking it away takes only links.**
        make_the_opening_folder(&folder).unwrap();
        assert_eq!(from_the_file("lib/rig.blend"), "the fixed copy");
        std::fs::remove_dir_all(&opening).unwrap();
        assert_eq!(std::fs::read_to_string(root.join("scenes/shot.blend")).unwrap(), "the file");
        assert_eq!(std::fs::read_to_string(root.join("scenes/lib/rig.blend")).unwrap(), "the user's copy");
        assert_eq!(std::fs::read_to_string(root.join("fixed/rig.blend")).unwrap(), "the fixed copy");
    }

    #[test]
    fn a_folder_that_cannot_be_made_says_where() {
        let scratch = a_scratch("refused");
        let in_the_way = scratch.0.join("a file");
        write(&in_the_way, "not a folder");
        let folder = the_folder_the_file_opens_from(
            &[a_library("//rig.blend", &scratch.0.join("rig.blend"))],
            scratch.0.join("shot.blend").as_os_str().as_bytes(),
            in_the_way.join("opening").as_os_str().as_bytes(),
        )
        .unwrap();
        let why = make_the_opening_folder(&folder).unwrap_err();
        assert!(why.contains("opening"), "{why}");
    }
}
