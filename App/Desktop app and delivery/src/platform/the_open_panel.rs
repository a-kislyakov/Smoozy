//! **macOS's own open panel with both switches on** — `File Browser` H3, and
//! the route `src/picker.rs` named as owed for files and folders together.
//!
//! *Added 2026-10-06, the Creator's second notes on the test copy, piece 9
//! (note 2).* `File Browser` H3 records one macOS panel taking files and
//! folders in one pick, observed in a controlled test on 2026-09-04. The
//! dialog plugin behind [`crate::picker`] offers files or folders and never
//! both, so this is the document's own platform code, the same shape
//! [`crate::reveal`] takes for its macOS third.
//!
//! **No new outside package.** `objc2-app-kit` and `objc2-foundation` are
//! already this crate's macOS dependencies, at the workspace's pins, with the
//! open panel among their default switches.
//!
//! **The panel is AppKit's and is drawn on the main thread**, so it is put
//! there through the shell's own hand-off, and the asking thread waits for the
//! answer. A caller is never on the main thread (`src/picker.rs`'s
//! `no_plain_command_reaches_the_blocking_ask`); if one ever is, the panel is
//! put up where it stands rather than waiting on itself.

use std::path::{Path, PathBuf};
use std::sync::mpsc;

use objc2::MainThreadMarker;
use objc2_app_kit::{NSModalResponseOK, NSOpenPanel};
use objc2_foundation::{NSString, NSURL};
use tauri::{AppHandle, Runtime};

/// **The panel could not be put up**: the main thread did not take it, or it
/// never answered. The picker says so as *no panel*, never as a user who
/// chose nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoPanel;

/// **Put the panel up with files and folders both choosable, and wait.**
///
/// `None` inside is a user who closed it. Every path is exactly what the panel
/// returned; judging it is the caller's next act (K13).
pub fn files_and_folders_are_chosen<R: Runtime>(
    app: &AppHandle<R>,
    many: bool,
    starting_at: Option<&Path>,
    title: &str,
) -> Result<Option<Vec<PathBuf>>, NoPanel> {
    let title = title.to_owned();
    let starting_at = starting_at.map(Path::to_path_buf);
    let put_up = move |on_main: MainThreadMarker| {
        let panel = NSOpenPanel::openPanel(on_main);
        panel.setCanChooseFiles(true);
        panel.setCanChooseDirectories(true);
        panel.setAllowsMultipleSelection(many);
        // **The message, as the plugin's own panels carry their title**: a
        // macOS open panel shows no title bar of its own.
        panel.setMessage(Some(&NSString::from_str(&title)));
        if let Some(place) = starting_at.as_ref().and_then(NSURL::from_file_path) {
            panel.setDirectoryURL(Some(&place));
        }
        if panel.runModal() != NSModalResponseOK {
            return None;
        }
        Some(
            panel
                .URLs()
                .iter()
                .filter_map(|url| url.to_file_path())
                .collect::<Vec<PathBuf>>(),
        )
    };
    if let Some(on_main) = MainThreadMarker::new() {
        return Ok(put_up(on_main));
    }
    let (answer, answered) = mpsc::channel();
    app.run_on_main_thread(move || {
        if let Some(on_main) = MainThreadMarker::new() {
            let _ = answer.send(put_up(on_main));
        }
    })
    .map_err(|_| NoPanel)?;
    answered.recv().map_err(|_| NoPanel)
}
