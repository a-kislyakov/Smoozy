//! **Test folders and files in the system's temporary directory, taken away
//! when the test program ends.** Test-only. Step 10c, the three small owed
//! fixes, item 3, 2026-10-24.
//!
//! The board found about a hundred entries left in the temporary directory by
//! two runs. Most tests here make a folder for a record of their own and hand
//! the record back, so the folder has to outlive the helper that made it, and
//! nothing ever removed it. **The helper here makes the folder and remembers
//! it, and when the test program ends every one it made is removed.** Nothing
//! is removed while a test could still be using it, because the removal waits
//! for the whole program, and a test that drops its record and opens it again
//! finds it still there.
//!
//! A program killed outright removes nothing; the next run starts each folder
//! afresh, as every helper here already did.

use std::path::{Path, PathBuf};
use std::sync::{Mutex, Once};

/// Every folder or file made through here, removed at the end.
static MADE: Mutex<Vec<PathBuf>> = Mutex::new(Vec::new());

static AT_THE_END: Once = Once::new();

extern "C" {
    /// The C library's own hook for the end of the program, on all three
    /// systems. The test harness ends by returning from its main or by
    /// calling exit, and both run it.
    fn atexit(callback: extern "C" fn()) -> std::os::raw::c_int;
}

/// **Runs once, as the program ends.** It never waits on a test still holding
/// the list, and it cannot panic.
extern "C" fn everything_made_is_removed() {
    let Ok(mut made) = MADE.try_lock() else { return };
    for path in std::mem::take(&mut *made) {
        if path.is_dir() {
            let _ = std::fs::remove_dir_all(&path);
        } else {
            let _ = std::fs::remove_file(&path);
            // A record's own companions, where the engine left any.
            for companion in ["-wal", "-shm", "-journal"] {
                let mut beside = path.clone().into_os_string();
                beside.push(companion);
                let _ = std::fs::remove_file(beside);
            }
        }
    }
}

/// **This folder or file goes when the test program ends.**
fn goes_at_the_end(path: &Path) {
    AT_THE_END.call_once(|| {
        // SAFETY: `atexit` takes a plain function with no arguments, and the
        // one given touches only this module's own list.
        unsafe {
            atexit(everything_made_is_removed);
        }
    });
    if let Ok(mut made) = MADE.lock() {
        made.push(path.to_path_buf());
    }
}

/// **A fresh, empty folder of this name in the system's temporary
/// directory**, removed when the test program ends.
pub(crate) fn a_folder(named: impl AsRef<str>) -> PathBuf {
    let folder = std::env::temp_dir().join(named.as_ref());
    let _ = std::fs::remove_dir_all(&folder);
    std::fs::create_dir_all(&folder).expect("a scratch folder in the temporary directory");
    goes_at_the_end(&folder);
    folder
}

/// **A path of this name in the system's temporary directory, with nothing
/// there yet**, for a record opened straight at it. The file and the record's
/// own companions are removed when the test program ends.
pub(crate) fn a_file(named: impl AsRef<str>) -> PathBuf {
    let file = std::env::temp_dir().join(named.as_ref());
    let _ = std::fs::remove_file(&file);
    goes_at_the_end(&file);
    file
}
