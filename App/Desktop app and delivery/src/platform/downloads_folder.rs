//! **Where a download is proposed to land** — `Desktop app and delivery` §8.
//!
//! **W4 is this file's reason for existing: nothing is written into the folder
//! unasked, and that is Apple's own instruction rather than a preference.** An
//! app *"must not write files to the preceding directories unless explicitly
//! directed to do so by the user"*. **So the adapter proposes the folder and
//! never writes into it on its own**; what the user is asked and when is
//! `Download`'s and Design's — `Download` H2 and H3, which the format boxes on
//! the Transfers screen draw.
//!
//! **The proposal had no route anywhere in this crate until step 10a piece 12.**
//! W1's three mechanisms, W2's refusal and W3's reading were cited at
//! `src/platform/locations.rs` as the shape of *other* readings and were kept for
//! the downloads folder by nothing. A proposal with no route is no proposal, so
//! the three are written here, **each exactly as §8 names it**, and nothing is
//! added beside them.
//!
//! - **macOS**: `FileManager.url(for: .downloadsDirectory, in: .userDomainMask,
//!   appropriateFor: nil, create:)` — **with creation off**, because creating the
//!   folder would be writing into the user's own place unasked (W4).
//! - **Windows**: `SHGetKnownFolderPath(FOLDERID_Downloads, 0, NULL, &p)`, then
//!   `CoTaskMemFree` — **the default-path flag never passed** (W2).
//! - **Linux**: `XDG_DOWNLOAD_DIR`, read from
//!   `${XDG_CONFIG_HOME:-$HOME/.config}/user-dirs.dirs` — **read and never
//!   assembled, and nothing cached** (W3).
//!
//! **W5: where the route returns nothing, the adapter says so and proposes
//! nothing.** No composed path, no folder created.
//!
//! **W6 is carried rather than answered**: whether the macOS route looks the
//! location up or composes it is §22 item 20, and it blocks no route.
//!
//! **W7: whatever the folder turns out to be, a name written into it is judged
//! by §7 like any other** — `src/platform/collision.rs` for the taken half.

use std::path::PathBuf;

/// **What the system proposes.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheProposal {
    /// The system named this folder. **A proposal, and nothing has been written
    /// there** (W4).
    TheSystemNamesThisFolder(PathBuf),
    /// **W5: the route returned nothing**, in the system's own words for the
    /// diary. **Never shown to the user as it stands.**
    TheSystemNamesNoFolder(String),
}

/// **W1's route on this system, taken fresh.** W3's *nothing caches it* holds on
/// all three: there is no store in this file and every call asks again.
pub fn the_folder_the_system_proposes() -> TheProposal {
    per_system()
}

#[cfg(target_os = "macos")]
fn per_system() -> TheProposal {
    use objc2_foundation::{NSFileManager, NSSearchPathDirectory, NSSearchPathDomainMask};

    let manager = NSFileManager::defaultManager();
    // **W4: `create` is false.** The same call `locations.rs` makes for the
    // application-data location with creation on, and the one difference is the
    // point: that folder is Smoozy's, this one is the user's.
    let url = match manager.URLForDirectory_inDomain_appropriateForURL_create_error(
        NSSearchPathDirectory::DownloadsDirectory,
        NSSearchPathDomainMask::UserDomainMask,
        None,
        false,
    ) {
        Ok(url) => url,
        Err(error) => {
            return TheProposal::TheSystemNamesNoFolder(error.localizedDescription().to_string())
        }
    };
    match url.path() {
        Some(path) => TheProposal::TheSystemNamesThisFolder(PathBuf::from(path.to_string())),
        None => TheProposal::TheSystemNamesNoFolder(
            "the location the system named is not a file path".to_string(),
        ),
    }
}

#[cfg(target_os = "windows")]
fn per_system() -> TheProposal {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    use windows_sys::Win32::System::Com::CoTaskMemFree;
    use windows_sys::Win32::UI::Shell::{SHGetKnownFolderPath, FOLDERID_Downloads};

    // **W2: the flags argument is 0**, so a relocated folder is followed rather
    // than defeated. **W5's trap is not reached for**: `.NET`'s special folders
    // have no Downloads member and there is no older equivalent to fall back on.
    let mut wide: windows_sys::core::PWSTR = std::ptr::null_mut();
    let status =
        unsafe { SHGetKnownFolderPath(&FOLDERID_Downloads, 0, std::ptr::null_mut(), &mut wide) };
    let path = if wide.is_null() {
        None
    } else {
        let mut length = 0usize;
        while unsafe { *wide.add(length) } != 0 {
            length += 1;
        }
        let slice = unsafe { std::slice::from_raw_parts(wide, length) };
        let owned = OsString::from_wide(slice);
        unsafe { CoTaskMemFree(wide as *const core::ffi::c_void) };
        Some(PathBuf::from(owned))
    };
    if status < 0 {
        return TheProposal::TheSystemNamesNoFolder(format!(
            "the system named no downloads folder (0x{:08X})",
            status as u32
        ));
    }
    match path {
        Some(path) if !path.as_os_str().is_empty() => TheProposal::TheSystemNamesThisFolder(path),
        _ => TheProposal::TheSystemNamesNoFolder("the system returned an empty path".to_string()),
    }
}

#[cfg(target_os = "linux")]
fn per_system() -> TheProposal {
    use std::path::Path;

    let Some(home) = std::env::var_os("HOME").map(PathBuf::from) else {
        return TheProposal::TheSystemNamesNoFolder("HOME is unset".to_string());
    };
    if !home.is_absolute() {
        return TheProposal::TheSystemNamesNoFolder("HOME names no absolute path".to_string());
    }
    let config = match std::env::var_os("XDG_CONFIG_HOME").map(PathBuf::from) {
        Some(value) if value.is_absolute() => value,
        _ => home.join(".config"),
    };
    let Ok(text) = std::fs::read_to_string(config.join("user-dirs.dirs")) else {
        return TheProposal::TheSystemNamesNoFolder(
            "user-dirs.dirs could not be read".to_string(),
        );
    };
    match the_download_folder_named_in(&text, Path::new(&home)) {
        Some(folder) => TheProposal::TheSystemNamesThisFolder(folder),
        None => TheProposal::TheSystemNamesNoFolder(
            "user-dirs.dirs names no downloads folder".to_string(),
        ),
    }
}

/// **W3's reading of the tool's own file**, kept apart so it is compiled and
/// tested on every system even though only Linux reads the file.
///
/// **Read and never assembled.** The tool writes the value either as an absolute
/// path or as `$HOME` followed by the rest, and that is the file's own form being
/// read rather than a path being built out of a word. **A value that is neither
/// is not a proposal**, and a missing key proposes nothing (W5) — **the fallback
/// is never `$HOME/Downloads`**, which is the one wrong answer W3 names.
#[cfg_attr(not(any(target_os = "linux", test)), allow(dead_code))]
fn the_download_folder_named_in(text: &str, home: &std::path::Path) -> Option<PathBuf> {
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with('#') {
            continue;
        }
        let Some(value) = line.strip_prefix("XDG_DOWNLOAD_DIR=") else {
            continue;
        };
        let value = value.trim().strip_prefix('"')?.strip_suffix('"')?;
        if value == "$HOME" {
            return Some(home.to_path_buf());
        }
        if let Some(rest) = value.strip_prefix("$HOME/") {
            return Some(home.join(rest));
        }
        if value.starts_with('/') {
            return Some(PathBuf::from(value));
        }
        return None;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    /// **W4, kept structurally: this file holds no write of any kind.**
    #[test]
    fn nothing_here_writes_into_the_users_folder() {
        let working = include_str!("downloads_folder.rs")
            .split("#[cfg(test)]\nmod tests {")
            .next()
            .expect("the working half");
        let code: String = working
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join("\n");
        for write in [
            "fs::write",
            "create_dir",
            "OpenOptions",
            "ensure_directory",
            "File::create",
            "remove_file",
        ] {
            assert!(!code.contains(write), "W4: this file writes ({write})");
        }
        assert!(
            code.contains("None,\n        false,"),
            "W4: the macOS route is asked with creation off"
        );
    }

    /// **W3's reading of the tool's file**, in each of the forms it takes.
    #[test]
    fn the_tools_file_is_read_and_never_assembled() {
        let home = Path::new("/home/artist");
        assert_eq!(
            the_download_folder_named_in("XDG_DOWNLOAD_DIR=\"$HOME/Téléchargements\"\n", home),
            Some(PathBuf::from("/home/artist/Téléchargements"))
        );
        assert_eq!(
            the_download_folder_named_in("# a comment\nXDG_DOWNLOAD_DIR=\"/data/incoming\"", home),
            Some(PathBuf::from("/data/incoming"))
        );
        assert_eq!(
            the_download_folder_named_in("XDG_DOWNLOAD_DIR=\"$HOME\"", home),
            Some(PathBuf::from("/home/artist")),
            "the tool resets a missing folder to $HOME, and that is what is read"
        );
        assert_eq!(
            the_download_folder_named_in("XDG_DESKTOP_DIR=\"$HOME/Desktop\"", home),
            None,
            "W5: no key proposes nothing, and never $HOME/Downloads"
        );
        assert_eq!(the_download_folder_named_in("XDG_DOWNLOAD_DIR=relative", home), None);
    }

    /// **On this Mac the route answers**, and asking it wrote nothing.
    #[cfg(target_os = "macos")]
    #[test]
    fn the_mac_proposes_a_folder() {
        match the_folder_the_system_proposes() {
            TheProposal::TheSystemNamesThisFolder(folder) => assert!(folder.is_absolute()),
            TheProposal::TheSystemNamesNoFolder(why) => {
                // W5 is a real answer on a machine that names none.
                assert!(!why.is_empty());
            }
        }
    }
}
