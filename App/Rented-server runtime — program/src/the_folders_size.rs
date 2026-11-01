//! **`Server protocol and transfers` §1.1's request *a folder's size is
//! counted on the machine*, answered by the SSH command adapter in its own
//! invocation.** Added 2026-10-30, the twenty-third notes, note 2 (`File
//! Browser`'s section 1.7).
//!
//! **What it does, and only this.** It walks everything below one folder,
//! once, and answers the total apparent size, how many files and folders it
//! met, and whether any folder below could not be read. **It writes nothing
//! anywhere**, so it is a pure reading.
//!
//! **The walk's rules are `File Browser` L26's.** A regular file adds its
//! apparent size — the size the list shows — and counts one file; a name
//! hard-linked twice counts twice, as two rows in the list would. A folder
//! counts one and is gone into. **A symbolic link below the folder is never
//! followed and adds nothing**, so the walk cannot loop. Anything else —
//! device, socket, pipe — adds nothing. **A mount of the system's in-memory
//! pseudo filesystems** (`/proc`, `/sys` and their kin) is not gone into,
//! because their sizes describe no disk; every other mount is. **A folder
//! below that cannot be opened or read is skipped and marks the count
//! incomplete.**
//!
//! **The folder asked about itself** is reached as the user named it: a link
//! there is followed, since the user asked for that folder. Nothing at the
//! path answers *nothing there*, something other than a folder answers *not a
//! folder*, and a folder that cannot be opened answers *not read* with the
//! machine's own words.
//!
//! **Why here and not in the supervisor** is the copy on the machine's reason
//! (`crate::the_copy_on_the_machine`): the supervisor serves its callers one
//! after another, and a walk of minutes there would hold every other request.
//! It reads no store the supervisor keeps.

use std::collections::HashSet;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

use smoozy_framing::framing::{
    Answered, Fields, Request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
};
use smoozy_framing::requests::{
    IT_IS_NOT_A_FOLDER, IT_WAS_COUNTED, IT_WAS_NOT_READ, NOTHING_IS_THERE, THE_ANSWER_IS_YES,
    THE_BYTES_COUNTED_FIELD, THE_COUNTED_FOLDER_FIELD, THE_COUNTS_OUTCOME_FIELD,
    THE_COUNT_IS_INCOMPLETE_FIELD, THE_COUNT_IS_WHOLE, THE_FILES_COUNTED_FIELD,
    THE_FOLDERS_COUNTED_FIELD, THE_MACHINES_OWN_WORDS_FIELD,
};

/// **The kinds of filesystem that describe no disk** — the system's own
/// in-memory views of processes, the kernel and its controls. A mount of one
/// of these below the counted folder is not gone into.
const THE_PSEUDO_FILESYSTEMS: [&str; 18] = [
    "proc", "sysfs", "cgroup", "cgroup2", "debugfs", "tracefs", "securityfs", "pstore", "bpf",
    "configfs", "fusectl", "devpts", "mqueue", "hugetlbfs", "binfmt_misc", "efivarfs",
    "selinuxfs", "autofs",
];

/// **What one walk found.**
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct WhatTheWalkCounted {
    pub bytes: u64,
    pub files: u64,
    pub folders: u64,
    pub incomplete: bool,
}

/// **Why a folder could not be counted at all.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhyNothingWasCounted {
    NothingThere,
    NotAFolder,
    NotRead(String),
}

/// **The request, answered.** A request with no folder is refused by name
/// rather than answered on a path made up here.
pub fn answered(request: &Request, package_version: &str) -> Answered {
    let empty = Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, package_version);
    if !smoozy_framing::framing::DeclaredVersions::of_this_build().covers(request.protocol_version) {
        return empty.saying_it_does_not_speak_that_protocol_version(request.protocol_version);
    }
    let Some(folder) = request.fields.value(THE_COUNTED_FOLDER_FIELD) else {
        return empty.saying_it_does_not_answer_that_request(&request.name);
    };
    let folder = Path::new(std::ffi::OsStr::from_bytes(folder));
    empty.carrying(match the_folder_is_counted(folder, &the_pseudo_filesystems_devices()) {
        Ok(counted) => Fields::none()
            .and(THE_COUNTS_OUTCOME_FIELD, IT_WAS_COUNTED)
            .and(THE_BYTES_COUNTED_FIELD, counted.bytes.to_string())
            .and(THE_FILES_COUNTED_FIELD, counted.files.to_string())
            .and(THE_FOLDERS_COUNTED_FIELD, counted.folders.to_string())
            .and(
                THE_COUNT_IS_INCOMPLETE_FIELD,
                if counted.incomplete { THE_ANSWER_IS_YES } else { THE_COUNT_IS_WHOLE },
            ),
        Err(WhyNothingWasCounted::NothingThere) => {
            Fields::none().and(THE_COUNTS_OUTCOME_FIELD, NOTHING_IS_THERE)
        }
        Err(WhyNothingWasCounted::NotAFolder) => {
            Fields::none().and(THE_COUNTS_OUTCOME_FIELD, IT_IS_NOT_A_FOLDER)
        }
        Err(WhyNothingWasCounted::NotRead(words)) => Fields::none()
            .and(THE_COUNTS_OUTCOME_FIELD, IT_WAS_NOT_READ)
            .and(THE_MACHINES_OWN_WORDS_FIELD, words),
    })
}

/// **The devices of every pseudo filesystem mounted on this machine**, read
/// off the system's own mount table. A machine with no such table (not
/// Linux) has none to skip.
fn the_pseudo_filesystems_devices() -> HashSet<u64> {
    let Ok(table) = std::fs::read_to_string("/proc/self/mounts") else {
        return HashSet::new();
    };
    table
        .lines()
        .filter_map(|line| {
            let mut words = line.split(' ');
            let _source = words.next()?;
            let at = words.next()?;
            let kind = words.next()?;
            THE_PSEUDO_FILESYSTEMS
                .contains(&kind)
                .then(|| the_mount_tables_path(at))
        })
        .filter_map(|at| std::fs::metadata(at).ok().map(|seen| seen.dev()))
        .collect()
}

/// **A path as the mount table writes it**, where a space, a tab, a newline
/// and a backslash are written as a backslash and three octal digits.
fn the_mount_tables_path(written: &str) -> PathBuf {
    let bytes = written.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut at = 0;
    while at < bytes.len() {
        let octal = bytes.get(at + 1..at + 4).filter(|digits| {
            bytes[at] == b'\\' && digits.iter().all(|digit| (b'0'..=b'7').contains(digit))
        });
        match octal {
            Some(digits) => {
                out.push(digits.iter().fold(0u8, |sum, digit| sum.wrapping_mul(8) + (digit - b'0')));
                at += 4;
            }
            None => {
                out.push(bytes[at]);
                at += 1;
            }
        }
    }
    PathBuf::from(std::ffi::OsStr::from_bytes(&out))
}

/// **The walk itself**, by `File Browser` L26's rules, kept on a list rather
/// than by recursion so a deep tree cannot exhaust the stack.
pub fn the_folder_is_counted(
    folder: &Path,
    pseudo: &HashSet<u64>,
) -> Result<WhatTheWalkCounted, WhyNothingWasCounted> {
    let top = match std::fs::metadata(folder) {
        Ok(top) => top,
        Err(why) if why.kind() == std::io::ErrorKind::NotFound => {
            return Err(WhyNothingWasCounted::NothingThere)
        }
        Err(why) => {
            return Err(WhyNothingWasCounted::NotRead(format!("{}: {why}", folder.display())))
        }
    };
    if !top.is_dir() {
        return Err(WhyNothingWasCounted::NotAFolder);
    }
    let first = std::fs::read_dir(folder)
        .map_err(|why| WhyNothingWasCounted::NotRead(format!("{}: {why}", folder.display())))?;
    let mut counted = WhatTheWalkCounted::default();
    let mut waiting = vec![first];
    while let Some(entries) = waiting.pop() {
        for entry in entries {
            let Ok(entry) = entry else {
                counted.incomplete = true;
                continue;
            };
            // **Neither of these follows a link**: both read the entry itself.
            let Ok(kind) = entry.file_type() else {
                counted.incomplete = true;
                continue;
            };
            if kind.is_symlink() {
                continue;
            }
            if kind.is_file() {
                match entry.metadata() {
                    Ok(seen) => {
                        counted.files += 1;
                        counted.bytes = counted.bytes.saturating_add(seen.len());
                    }
                    Err(_) => counted.incomplete = true,
                }
                continue;
            }
            if !kind.is_dir() {
                continue;
            }
            counted.folders += 1;
            let path = entry.path();
            if let Ok(seen) = std::fs::symlink_metadata(&path) {
                if pseudo.contains(&seen.dev()) {
                    continue;
                }
            }
            match std::fs::read_dir(&path) {
                Ok(below) => waiting.push(below),
                Err(_) => counted.incomplete = true,
            }
        }
    }
    Ok(counted)
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_state::identities::{Identity, IdentityKind};

    struct Scratch(PathBuf);

    impl Scratch {
        fn new(named: &str) -> Scratch {
            let at = std::env::temp_dir().join(format!(
                "smoozy-size-{named}-{}",
                &Identity::issue(IdentityKind::Operation).expect("randomness").as_text()[..12]
            ));
            std::fs::create_dir_all(&at).expect("made");
            Scratch(at)
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            use std::os::unix::fs::PermissionsExt;
            // A folder made unreadable by a test is opened again so it can go.
            for entry in walk_all(&self.0) {
                let _ = std::fs::set_permissions(&entry, std::fs::Permissions::from_mode(0o755));
            }
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn walk_all(at: &Path) -> Vec<PathBuf> {
        let mut found = vec![at.to_path_buf()];
        if let Ok(entries) = std::fs::read_dir(at) {
            for entry in entries.flatten() {
                if entry.file_type().map(|kind| kind.is_dir()).unwrap_or(false) {
                    found.extend(walk_all(&entry.path()));
                }
            }
        }
        found
    }

    fn a_request(folder: &Path) -> Request {
        Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            Identity::issue(IdentityKind::Operation).expect("randomness"),
            smoozy_framing::requests::THE_FOLDERS_SIZE_IS_COUNTED_REQUEST,
        )
        .carrying(Fields::none().and(THE_COUNTED_FOLDER_FIELD, folder.as_os_str().as_bytes()))
    }

    /// **Every file in every folder below, to any depth, is counted** — and
    /// the answer carries the four figures.
    #[test]
    fn twenty_third_2_the_walk_counts_nested_folders() {
        let scratch = Scratch::new("nested");
        let deep = scratch.0.join("a/b/c");
        std::fs::create_dir_all(&deep).expect("made");
        std::fs::write(scratch.0.join("top.bin"), vec![0u8; 100]).expect("written");
        std::fs::write(scratch.0.join("a/one.bin"), vec![0u8; 20]).expect("written");
        std::fs::write(deep.join("deep.bin"), vec![0u8; 3]).expect("written");
        std::fs::create_dir_all(scratch.0.join("empty")).expect("made");
        let answered = answered(&a_request(&scratch.0), "0.1.8");
        let fields = &answered.fields;
        assert_eq!(fields.text(THE_COUNTS_OUTCOME_FIELD), Some(IT_WAS_COUNTED), "{answered:?}");
        assert_eq!(fields.text(THE_BYTES_COUNTED_FIELD), Some("123"));
        assert_eq!(fields.text(THE_FILES_COUNTED_FIELD), Some("3"));
        assert_eq!(fields.text(THE_FOLDERS_COUNTED_FIELD), Some("4"));
        assert_eq!(fields.text(THE_COUNT_IS_INCOMPLETE_FIELD), Some(THE_COUNT_IS_WHOLE));
    }

    /// **A link below is never followed and adds nothing** — not to a file,
    /// and not to a folder, even one that holds the walk's own top.
    #[test]
    fn twenty_third_2_the_walk_skips_links() {
        let scratch = Scratch::new("links");
        let inside = scratch.0.join("inside");
        std::fs::create_dir_all(&inside).expect("made");
        std::fs::write(inside.join("real.bin"), vec![0u8; 10]).expect("written");
        std::os::unix::fs::symlink(inside.join("real.bin"), inside.join("to-the-file")).expect("linked");
        std::os::unix::fs::symlink(&scratch.0, inside.join("to-the-top")).expect("linked");
        let counted = the_folder_is_counted(&scratch.0, &HashSet::new()).expect("counted");
        assert_eq!(
            counted,
            WhatTheWalkCounted { bytes: 10, files: 1, folders: 1, incomplete: false }
        );
    }

    /// **A folder below that cannot be read is skipped and marks the count
    /// incomplete**, and what could be read is still counted.
    #[test]
    fn twenty_third_2_an_unreadable_folder_marks_the_count_incomplete() {
        use std::os::unix::fs::PermissionsExt;
        let scratch = Scratch::new("shut");
        let shut = scratch.0.join("shut");
        std::fs::create_dir_all(&shut).expect("made");
        std::fs::write(shut.join("hidden.bin"), vec![0u8; 50]).expect("written");
        std::fs::write(scratch.0.join("seen.bin"), vec![0u8; 7]).expect("written");
        std::fs::set_permissions(&shut, std::fs::Permissions::from_mode(0o000)).expect("shut");
        if std::fs::read_dir(&shut).is_ok() {
            // Run with powers that read anything (as root): nothing to prove here.
            return;
        }
        let answered = answered(&a_request(&scratch.0), "0.1.8");
        let fields = &answered.fields;
        assert_eq!(fields.text(THE_COUNTS_OUTCOME_FIELD), Some(IT_WAS_COUNTED));
        assert_eq!(fields.text(THE_BYTES_COUNTED_FIELD), Some("7"));
        assert_eq!(fields.text(THE_FILES_COUNTED_FIELD), Some("1"));
        assert_eq!(fields.text(THE_FOLDERS_COUNTED_FIELD), Some("1"));
        assert_eq!(fields.text(THE_COUNT_IS_INCOMPLETE_FIELD), Some(THE_ANSWER_IS_YES));
    }

    /// **Nothing there, not a folder, and a request with no folder** each
    /// answer as they should.
    #[test]
    fn twenty_third_2_nothing_there_and_not_a_folder_are_told_apart() {
        let scratch = Scratch::new("apart");
        let file = scratch.0.join("file.bin");
        std::fs::write(&file, b"x").expect("written");
        let gone = answered(&a_request(&scratch.0.join("gone")), "0.1.8");
        assert_eq!(gone.fields.text(THE_COUNTS_OUTCOME_FIELD), Some(NOTHING_IS_THERE));
        let a_file = answered(&a_request(&file), "0.1.8");
        assert_eq!(a_file.fields.text(THE_COUNTS_OUTCOME_FIELD), Some(IT_IS_NOT_A_FOLDER));
        let bare = Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            Identity::issue(IdentityKind::Operation).expect("randomness"),
            smoozy_framing::requests::THE_FOLDERS_SIZE_IS_COUNTED_REQUEST,
        );
        assert!(answered(&bare, "0.1.8").the_request_name_it_says_it_does_not_answer().is_some());
    }

    /// **A mount of a pseudo filesystem is not gone into** — shown here by
    /// naming the device of a folder below as one.
    #[test]
    fn twenty_third_2_a_pseudo_filesystem_is_not_descended() {
        let scratch = Scratch::new("pseudo");
        let mounted = scratch.0.join("proc");
        std::fs::create_dir_all(&mounted).expect("made");
        std::fs::write(mounted.join("kcore"), vec![0u8; 99]).expect("written");
        let device = std::fs::metadata(&mounted).expect("seen").dev();
        let counted = the_folder_is_counted(&scratch.0, &HashSet::from([device])).expect("counted");
        // The top shares the device, but the top is always read: only folders below are skipped.
        assert_eq!(counted, WhatTheWalkCounted { bytes: 0, files: 0, folders: 1, incomplete: false });
        assert_eq!(the_mount_tables_path("/mnt/my\\040disk"), PathBuf::from("/mnt/my disk"));
    }
}
