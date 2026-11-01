//! **The folder-reveal service** — `Desktop app and delivery` §18.6, K20 to
//! K24.
//!
//! **Two acts and no third**: reveal a named file selected in its folder, and
//! open a folder, in the system's own file manager.
//!
//! **It is platform code of this document's own, beside the shell**, the same
//! shape the title bar takes, and that is K20 rather than a preference. The
//! chosen shell reaches none of it: its one browser-opening call **refuses the
//! `file` scheme by name** and **returns nothing at all on all three
//! platforms**, so even were the scheme permitted the application could not
//! learn that the open failed — which is precisely what it must be able to
//! learn.
//!
//! **K22 is kept by the return type and by there being one of it.** Every route
//! below answers, [`Revealed::ItCouldNotBeOpened`] exists rather than silence,
//! and **no function here returns nothing on any system** — so a builder cannot
//! half-use this service. The service does nothing else: no retry, no failure
//! record of its own, no surface. What a caller does with a refusal is that
//! caller's document's.
//!
//! **What is observed and what is documentation alone, said here rather than
//! left to be assumed** (§22 item 53, and items 2 and 3):
//!
//! - **macOS is observed.** K21 records that a plain binary reached the Finder
//!   and got answers back, with no bundle, no entitlements and no Developer ID.
//!   [`tests::the_mac_reveals_a_real_file_and_a_real_folder`] is the run this
//!   piece added, and what it saw is written at it.
//! - **Windows and Linux are documentation alone**, from Microsoft's and
//!   freedesktop's own pages. No Windows desktop edition has ever been reached
//!   and no Linux desktop computer has ever been run, and the Creator's ruling
//!   of 2026-08-31 means neither is coming. **No space is left in this file for
//!   an answer that is not coming.**

use std::path::{Path, PathBuf};

/// **K22: the outcome, and there are two of them.**
///
/// *Could not be opened* is a thing that exists rather than silence, and it
/// carries **no cause modelled behind it** — K22's own words. The routes below
/// each learn a different kind of negative from their own system, and composing
/// one vocabulary out of three unread ones would be this file inventing a
/// classification nobody read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Revealed {
    /// The system's file manager took the request and answered.
    ItOpened,
    /// It did not, and nothing more is claimed about why.
    ItCouldNotBeOpened,
}

impl Revealed {
    /// So a caller reads the outcome rather than matching on it where all it
    /// wants is the one bit. `Reporting a problem` V8 withholds a sentence on a
    /// refusal and `Download` H8 presents a place Smoozy cannot open.
    pub fn it_opened(self) -> bool {
        matches!(self, Revealed::ItOpened)
    }
}

/// **The first act: a named file, selected in its folder.**
///
/// **The path is resolved before it is asked for** — see
/// [`the_path_the_system_is_asked_for`], which is macOS's constraint and is
/// applied everywhere so that one act does not mean two things.
pub fn a_file_in_its_folder(path: &Path) -> Revealed {
    let Some(path) = the_path_the_system_is_asked_for(path) else {
        return Revealed::ItCouldNotBeOpened;
    };
    per_system::a_file_in_its_folder(&path)
}

/// **The second act: a folder, opened.**
pub fn a_folder(path: &Path) -> Revealed {
    let Some(path) = the_path_the_system_is_asked_for(path) else {
        return Revealed::ItCouldNotBeOpened;
    };
    per_system::a_folder(&path)
}

/// **Symbolic links are resolved before the system is asked** — K21's macOS
/// constraint, which is the one that keeps that route honest: Apple's own page
/// says the call *"does not follow symlinks when selecting the file"* and
/// selects the symlink instead of its target, **so the wrong thing is
/// highlighted unless the path is resolved first.**
///
/// **It is applied on all three rather than on macOS alone**, and that is a
/// decision rather than an oversight: the service performs one act, and an act
/// that pointed at the link on one system and at its target on another would be
/// two acts wearing one name. K24 is what makes it safe to say so — *telling
/// two paths to be one folder is not this section's question*, and this
/// resolves rather than compares.
///
/// A path that cannot be resolved at all — nothing is there — is a refusal
/// before any system is asked, which is K22's outcome and not a silence.
fn the_path_the_system_is_asked_for(path: &Path) -> Option<PathBuf> {
    std::fs::canonicalize(path).ok()
}

// ── macOS ───────────────────────────────────────────────────────────────────

#[cfg(target_os = "macos")]
mod per_system {
    use super::Revealed;
    use objc2_app_kit::NSWorkspace;
    use objc2_foundation::NSString;
    use std::path::Path;

    /// **K21, macOS: `selectFile:inFileViewerRootedAtPath:`, which returns a
    /// status.**
    ///
    /// **The no-outcome variant `activateFileViewerSelectingURLs:` is
    /// refused**, and the refusal is K21's rather than this file's: its
    /// declared return is nothing, so there is no outcome that could count as
    /// failure, and failure has to reach the caller.
    /// [`super::tests::the_no_outcome_variant_is_never_named`] is what holds
    /// that, because the call is one line away and looks like the simpler one.
    pub(super) fn a_file_in_its_folder(path: &Path) -> Revealed {
        let root = path.parent().unwrap_or(path);
        answer(
            NSWorkspace::sharedWorkspace().selectFile_inFileViewerRootedAtPath(
                Some(&NSString::from_str(&path.to_string_lossy())),
                &NSString::from_str(&root.to_string_lossy()),
            ),
        )
    }

    /// **Folder-open is the same call with no file** — K21.
    pub(super) fn a_folder(path: &Path) -> Revealed {
        answer(
            NSWorkspace::sharedWorkspace()
                .selectFile_inFileViewerRootedAtPath(None, &NSString::from_str(&path.to_string_lossy())),
        )
    }

    fn answer(said_yes: bool) -> Revealed {
        if said_yes {
            Revealed::ItOpened
        } else {
            Revealed::ItCouldNotBeOpened
        }
    }
}

// ── Windows ─────────────────────────────────────────────────────────────────

#[cfg(target_os = "windows")]
mod per_system {
    use super::Revealed;
    use std::path::Path;
    use windows_sys::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_APARTMENTTHREADED};
    use windows_sys::Win32::UI::Shell::{
        ILCreateFromPathW, ILFree, SHOpenFolderAndSelectItems, ShellExecuteW,
    };
    use windows_sys::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;

    /// **K21, Windows: `SHOpenFolderAndSelectItems`, with the component system
    /// initialised first.**
    ///
    /// **Its own page requires that rather than suggesting it** — *"Not doing
    /// so causes … to fail."* — so the initialisation is part of the call and
    /// not a tidiness. It is undone on the way out whether the call succeeded or
    /// not, because this thread is the shell's and does not belong to us.
    ///
    /// **The `explorer.exe /select` command line is folklore and is not used**
    /// (K21). Microsoft publishes it in no reference documentation, and a
    /// launched process would return no status — which is the same fault K20
    /// refuses the shell's own browser-opening call for.
    /// [`super::tests::the_windows_folklore_is_never_named`] holds it.
    pub(super) fn a_file_in_its_folder(path: &Path) -> Revealed {
        let wide = wide(path);
        // Safety: every argument is a pointer this function owns for the
        // whole call. The item list is freed on both paths below.
        unsafe {
            let started = CoInitializeEx(std::ptr::null(), COINIT_APARTMENTTHREADED as u32);
            let item = ILCreateFromPathW(wide.as_ptr());
            let outcome = if item.is_null() {
                Revealed::ItCouldNotBeOpened
            } else {
                // **Zero items and a null array is what is passed, and nothing
                // read establishes what the call makes of it.** This said *the
                // documented way* until 2026-09-12, and **no page saying so is
                // recorded anywhere in `Preparation/`** — K21 names this call
                // and its one published requirement, which is the component
                // system being initialised first, and says nothing at all about
                // what is passed to it. The word was a claim about a vendor's
                // documentation with no citation, in the one file family nobody
                // at this desk can run, which is exactly the shape K21 exists to
                // prevent (F-13).
                //
                // **What is known is what the outcome says**: the call answers,
                // and the answer reaches the caller either way, which is K22.
                // **The call is not replaced**, nobody having checked a
                // replacement either, and §22 item 70 carries the rest.
                let hresult = SHOpenFolderAndSelectItems(item, 0, std::ptr::null(), 0);
                ILFree(item);
                if hresult >= 0 {
                    Revealed::ItOpened
                } else {
                    Revealed::ItCouldNotBeOpened
                }
            };
            if started >= 0 {
                CoUninitialize();
            }
            outcome
        }
    }

    /// **K21, Windows: folder-open is `ShellExecute` on the folder path, with
    /// its documented greater-than-32 success contract.**
    pub(super) fn a_folder(path: &Path) -> Revealed {
        let wide = wide(path);
        // Safety: the two wide strings outlive the call, and the window handle
        // is deliberately null — this is not opened as a child of Smoozy's
        // window.
        let answered = unsafe {
            ShellExecuteW(
                std::ptr::null_mut(),
                std::ptr::null(),
                wide.as_ptr(),
                std::ptr::null(),
                std::ptr::null(),
                SW_SHOWNORMAL,
            )
        };
        // The published contract is the returned value being **greater than
        // 32**, and it is a handle-shaped value rather than a status, which is
        // why the comparison is written out rather than hidden behind a helper.
        if answered as isize > 32 {
            Revealed::ItOpened
        } else {
            Revealed::ItCouldNotBeOpened
        }
    }

    fn wide(path: &Path) -> Vec<u16> {
        use std::os::windows::ffi::OsStrExt;
        path.as_os_str().encode_wide().chain(Some(0)).collect()
    }
}

// ── Linux ───────────────────────────────────────────────────────────────────

#[cfg(target_os = "linux")]
mod per_system {
    use super::Revealed;
    use std::path::Path;

    /// **K21, Linux: the portal's `OpenURI.OpenDirectory`**, published and
    /// versioned, **whose fallback is the route's own published behaviour
    /// rather than this application's error handling**: it is implemented over
    /// `org.freedesktop.FileManager1.ShowItems()` and falls back to the plain
    /// open on the directory where no file manager owns that name.
    ///
    /// **It returns a handle and answers on a response signal, so failure is
    /// learnable in the ordinary way** — which is what is waited for here. A
    /// computer with no portal at all answers with a bus error, which is the
    /// same refusal by a different door; **nothing here assumes a file manager
    /// is present**, and the portal's own fallback is what makes that
    /// survivable.
    pub(super) fn a_file_in_its_folder(path: &Path) -> Revealed {
        let Ok(file) = std::fs::File::open(path) else {
            return Revealed::ItCouldNotBeOpened;
        };
        let Ok(waiting) = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
        else {
            return Revealed::ItCouldNotBeOpened;
        };
        waiting.block_on(async {
            match ashpd::desktop::open_uri::OpenDirectoryRequest::default()
                .send(&file)
                .await
            {
                Ok(request) => match request.response() {
                    Ok(()) => Revealed::ItOpened,
                    Err(_) => Revealed::ItCouldNotBeOpened,
                },
                Err(_) => Revealed::ItCouldNotBeOpened,
            }
        })
    }

    /// **K21, Linux: folder-open falls to `xdg-open`**, which ships in a
    /// separate project and is declared as a package dependency in E4's matrix,
    /// **or the call fails** — a machine without it answers, rather than going
    /// quiet, because a process that will not start is an error this code sees.
    pub(super) fn a_folder(path: &Path) -> Revealed {
        match std::process::Command::new("xdg-open").arg(path).status() {
            Ok(status) if status.success() => Revealed::ItOpened,
            _ => Revealed::ItCouldNotBeOpened,
        }
    }
}

// ── Anywhere else ───────────────────────────────────────────────────────────

/// **There is no fourth system in the release matrix**, and this arm exists so
/// that the crate compiles wherever somebody points a compiler at it rather
/// than failing in a way that reads like a fault in the reveal. **It is not a
/// route and claims nothing**: it answers K22's refusal, which is the honest
/// answer for a system this service was never written for.
#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
mod per_system {
    use super::Revealed;
    use std::path::Path;

    pub(super) fn a_file_in_its_folder(_path: &Path) -> Revealed {
        Revealed::ItCouldNotBeOpened
    }

    pub(super) fn a_folder(_path: &Path) -> Revealed {
        Revealed::ItCouldNotBeOpened
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **K22: there is no variant that returns nothing.**
    ///
    /// Both acts answer, and a path that is not there is a refusal rather than
    /// a silence — which is the case a caller most easily forgets, because it
    /// never reaches an operating system at all.
    #[test]
    fn a_path_that_is_not_there_is_a_refusal_rather_than_a_silence() {
        let nowhere = std::env::temp_dir().join("smoozy-no-such-thing-at-all");
        assert_eq!(a_file_in_its_folder(&nowhere), Revealed::ItCouldNotBeOpened);
        assert_eq!(a_folder(&nowhere), Revealed::ItCouldNotBeOpened);
    }

    /// **K21's macOS constraint, and it is the one the whole route turns on.**
    ///
    /// Apple's own page says the call does not follow symbolic links and
    /// selects the link rather than its target, **so an unresolved path
    /// highlights the wrong thing**. This is the line that fails if somebody
    /// takes the resolution out because it looked like a tidy-up.
    #[test]
    fn a_symbolic_link_is_resolved_before_the_system_is_asked() {
        let area = std::env::temp_dir().join(format!(
            "smoozy-reveal-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|since| since.as_nanos())
                .unwrap_or(0)
        ));
        std::fs::create_dir_all(area.join("real")).expect("the area is made");
        let target = area.join("real/frame 001.exr");
        std::fs::write(&target, b"").expect("the file is written");
        let link = area.join("pointing at it");

        #[cfg(unix)]
        std::os::unix::fs::symlink(&target, &link).expect("the link is made");
        #[cfg(not(unix))]
        std::fs::write(&link, b"").expect("stood in for the link");

        let asked_for = the_path_the_system_is_asked_for(&link).expect("it resolves");
        #[cfg(unix)]
        assert_eq!(
            asked_for,
            std::fs::canonicalize(&target).expect("the target resolves"),
            "the path handed to the system is still the link. **K21: the call \
             does not follow symbolic links and would select the link instead \
             of its target**, so the wrong thing is highlighted"
        );
        #[cfg(not(unix))]
        let _ = asked_for;

        let _ = std::fs::remove_dir_all(&area);
    }

    /// **K21 refuses `activateFileViewerSelectingURLs:` by name**, because its
    /// declared return is nothing and failure has to reach the caller.
    ///
    /// **It is one line away from the call this file makes and reads like the
    /// simpler one**, which is exactly why the refusal is held by a test rather
    /// than by a comment somebody would delete along with the code.
    #[test]
    fn the_no_outcome_variant_is_never_named() {
        let (working, _) = include_str!("reveal.rs")
            .split_once("#[cfg(test)]")
            .expect("the tests are the last thing in this file");
        assert_eq!(
            working.matches("activateFileViewerSelectingURLs").count(),
            1,
            "the macOS route names `activateFileViewerSelectingURLs` somewhere \
             other than the comment that refuses it. **K21: its declared return \
             is nothing, so there is no outcome that could count as failure**, \
             and K22 requires the outcome to reach the caller"
        );
    }

    /// **K21: the `explorer.exe /select` command line is folklore and is not
    /// used.**
    ///
    /// Microsoft publishes it in no reference documentation and a launched
    /// process would return no status. **It is the first thing anybody reaches
    /// for on Windows**, and nobody at this desk can see the Windows build, so
    /// the refusal is a test rather than a habit.
    #[test]
    fn the_windows_folklore_is_never_named() {
        let (working, _) = include_str!("reveal.rs")
            .split_once("#[cfg(test)]")
            .expect("the tests are the last thing in this file");
        // **Comment lines are read past**, because the Windows route names the
        // folklore in order to refuse it and a check that could not tell a
        // prohibition from an instance would be one nobody could write the
        // prohibition beside. That is the same rule the page's own H32 check
        // uses.
        let working: String = working
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            !working.contains("explorer.exe"),
            "the Windows route reaches for `explorer.exe /select`. **K21: it is \
             folklore, Microsoft publishes it in no reference documentation, \
             and a launched process would return no status** — which is the \
             same fault K20 refuses the shell's own call for"
        );
    }

    /// **The two acts and no third** — K20.
    ///
    /// The service is a named service of §21 and A1 admits no other route to
    /// either act, so a third public act here would be a third act nobody
    /// declared.
    #[test]
    fn there_are_two_acts_and_no_third() {
        let (working, _) = include_str!("reveal.rs")
            .split_once("#[cfg(test)]")
            .expect("the tests are the last thing in this file");
        let acts: Vec<&str> = working
            .lines()
            .filter(|line| line.starts_with("pub fn "))
            .collect();
        assert_eq!(
            acts,
            vec![
                "pub fn a_file_in_its_folder(path: &Path) -> Revealed {",
                "pub fn a_folder(path: &Path) -> Revealed {",
            ],
            "the folder-reveal service performs an act K20 does not name. \
             **Two acts and no third**, and A1 admits no other route to either"
        );
    }

    /// **K24, and it is the one thing §18.6 says was never checked even on the
    /// Mac.** Run it by hand and watch what the Finder does:
    ///
    /// ```text
    /// cargo test -p smoozy-desktop the_mac_reveals -- --ignored --nocapture
    /// ```
    ///
    /// **What was seen when this piece ran it on the Creator's own Mac,
    /// 2026-09-08** — written here rather than in a report, because this is
    /// where the next person asking the question will be standing.
    ///
    /// **Both calls answered true. The Finder became the frontmost application
    /// — read back from `lsappinfo front`, which needs no permission — and a
    /// Finder window came forward showing the scratch folder with
    /// `frame 001.exr` as the one highlighted row**, its own status line reading
    /// *1 of 1 selected*. The screen was photographed and the row read off it.
    /// **The second act reused the same window rather than opening a second
    /// one, and the selection stayed.**
    ///
    /// **So K24's question — whether a reveal actually brings a window forward
    /// with the right row highlighted — is answered yes on macOS, and it is
    /// observed rather than documented.** §22 item 53 said it had never been
    /// checked even on the Mac; it has now. **It stays unanswered on Windows
    /// and Linux, where every claim is documentation alone and no computer is
    /// coming, and no space is left here for an answer that is not coming.**
    ///
    /// It is ignored by default because it puts a window on the screen of
    /// whoever runs the suite, which is a thing no test should do unasked.
    ///
    /// **The folder carries the process id**, the way every other scratch name
    /// in this crate does. It used to be the fixed name `smoozy-k24`, which is
    /// the fault repaired across this crate on 2026-09-18: two runs at once on
    /// one computer share a fixed name and delete each other's files.
    ///
    /// **It is the one scratch folder here that is not removed at the end**,
    /// and deliberately: the whole point is a Finder window somebody looks at,
    /// and deleting the folder empties that window before they can. The last
    /// line says where it is so whoever ran it can remove it.
    #[test]
    #[ignore = "puts a Finder window on the screen; run it by hand and watch"]
    fn the_mac_reveals_a_real_file_and_a_real_folder() {
        let area = std::env::temp_dir().join(format!("smoozy-k24-{}", std::process::id()));
        std::fs::create_dir_all(&area).expect("the area is made");
        let file = area.join("frame 001.exr");
        std::fs::write(&file, b"not really a frame").expect("the file is written");

        println!("revealing {}", file.display());
        assert_eq!(a_file_in_its_folder(&file), Revealed::ItOpened);
        println!("opening {}", area.display());
        assert_eq!(a_folder(&area), Revealed::ItOpened);
        println!("remove {} when you have finished looking", area.display());
    }

    /// **K21's paragraph of 2026-10-03: whether the first act, handed a
    /// folder, selects it.** Apple's page speaks only of a file, so on macOS it
    /// is a Rule 2 fact *observed in a test* or nothing. Run it by hand and
    /// watch:
    ///
    /// ```text
    /// cargo test -p smoozy-desktop the_mac_selects_a_real_folder -- --ignored --nocapture
    /// ```
    ///
    /// It stands in for a job's folder landed in a downloads folder: a folder
    /// holding a frame, beside a second folder, so a selection shows which.
    /// **What was seen is written at `Audits/Build/Step 10c/The Creator's
    /// notes on the test copy, piece 4 — what landed.md`**, section 4.
    ///
    /// Ignored by default for the reason the test above gives, and like it,
    /// it leaves its folder for whoever is looking.
    #[test]
    #[ignore = "puts a Finder window on the screen; run it by hand and watch"]
    fn the_mac_selects_a_real_folder_in_its_parent() {
        let area = std::env::temp_dir().join(format!("smoozy-k21-folder-{}", std::process::id()));
        let the_jobs_folder = area.join("abc night");
        std::fs::create_dir_all(&the_jobs_folder).expect("the folder is made");
        std::fs::create_dir_all(area.join("another job")).expect("the second folder is made");
        std::fs::write(the_jobs_folder.join("frame 001.exr"), b"not really a frame")
            .expect("the file is written");

        println!("revealing {}", the_jobs_folder.display());
        assert_eq!(a_file_in_its_folder(&the_jobs_folder), Revealed::ItOpened);
        println!("remove {} when you have finished looking", area.display());
    }
}
