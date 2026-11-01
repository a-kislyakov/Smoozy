//! **The system file picker** — `Desktop app and delivery` §18.4, K11 to K13.
//!
//! **One service for every module that wants one, reached through this adapter
//! and by no other route** (K11, A1). There is no second way to put a file
//! panel on the screen in this application, and
//! [`tests::the_page_is_granted_no_route_to_a_picker_of_its_own`] is what makes
//! that structural rather than remembered.
//!
//! **Row 307 is `File Browser`'s and is not answered here** (K12). Whether one
//! system dialog can take files and folders together is that module's question
//! and its answer is in that module's own code; what this document supplies is
//! the picker, and a caller asks for what it wants.
//!
//! **K13 is kept by the type a caller gets back.** Whatever a picker returns is
//! **a path from the user**, and it is judged like any other name before
//! anything is written (C20, W7). A path the user chose is not a path that is
//! safe to write into, and [`APathFromTheUser`] says so in the one place a
//! caller cannot walk past: it hands out a borrowed path under a name that
//! cannot be mistaken for a checked one. `Keys` already cites K13 for exactly
//! this, and `File Browser`'s own picker answer does too.
//!
//! ## What is behind it, and what that cost
//!
//! **The route is the shell project's own file-dialog plugin**, pinned in
//! `App/Cargo.toml` where the reasoning and the measured weight are written out.
//! It is not the shape [`crate::reveal`] takes — that one is this document's own
//! platform code, which is K20's instruction for the reveal and not for this —
//! and the manifest says why the two differ.
//!
//! **The page is granted none of the plugin's permissions**, so the plugin's own
//! commands are unreachable from JavaScript and this adapter is the only door.
//! That is K11 held by the capability file rather than by anybody's care.
//!
//! ## Files and folders together
//!
//! **On macOS, [`WhatMayBeChosen::FilesAndFolders`] opens the system's own
//! open panel with both switches on** (`File Browser` H3, observed 2026-09-04),
//! through [`crate::platform::the_open_panel`], which is this document's own
//! platform code because the plugin offers files or folders and never both.
//! *Built 2026-10-06, the Creator's second notes on the test copy, piece 9
//! (note 2): until then the route was owed and a Mac met the kind question
//! too, whose sentence was untrue there.*
//!
//! **On Windows and Linux it answers [`Picked::ThereIsNoPanelForThisAsk`]**,
//! because neither system's picker takes both (`File Browser` H1, H2), and
//! `File Browser` H4's fallback — the same Upload control asking which kind
//! first — is what those two meet. **It answered *the user chose nothing*
//! until 2026-09-12, and that was the fault F-11 named**: that is exactly what
//! a cancelled panel gives, so a caller could not tell a missing route from a
//! user who closed the panel. A caller has to handle the answer of its own.

use std::path::{Path, PathBuf};

use tauri::{AppHandle, Runtime};
use tauri_plugin_dialog::DialogExt;

use crate::typography::Prose;

/// **A path the user chose, and nothing has judged it** — K13.
///
/// It exists so that a caller cannot hold a picked path in the same shape as a
/// path Smoozy composed itself. **The name is the whole of the point**: every
/// site that takes one out is a site somebody has to have thought about the
/// judgement at.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct APathFromTheUser(PathBuf);

impl APathFromTheUser {
    /// **Exactly what the picker returned, unjudged.** §7's judgement is the
    /// caller's next act and this is its input, never its output.
    pub fn as_the_user_gave_it(&self) -> &Path {
        &self.0
    }
}

/// What a caller will accept.
///
/// **Files and folders together is Row 307's question and it is not answered
/// here** (K12): the variant exists because a caller may ask for it, and what
/// each system does with the ask is `File Browser`'s answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatMayBeChosen {
    Files,
    Folders,
    /// **macOS's own panel with both switches on; no panel elsewhere** — see
    /// this file's head. Never a files-only panel opened quietly, because a
    /// panel that silently dropped half of what was asked for is the failure a
    /// caller could not see.
    FilesAndFolders,
}

/// One, or as many as the user likes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HowMany {
    One,
    Any,
}

/// **What a caller asks for.** A closed request, which is H34's shape.
#[derive(Debug, Clone)]
pub struct Asking {
    pub what: WhatMayBeChosen,
    pub how_many: HowMany,
    /// Where the panel opens. **A place Smoozy already holds** — never
    /// somewhere composed from a name the user typed.
    pub starting_at: Option<PathBuf>,
    /// The panel's own title. **Prose, through the typography point** — X5:
    /// *the system file picker's own chrome* is drawn by no engine and passes
    /// the point anyway, which is exactly the surface X5 names and which piece A
    /// recorded as owed because it did not exist.
    pub title: Prose,
}

/// **What came back.**
///
/// A user who closed the panel chose nothing, and that is an ordinary answer
/// rather than a failure — there is no refusal to model, because a picker that
/// could not be shown and a user who changed their mind are one thing to every
/// caller Smoozy has.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Picked {
    TheUserChose(Vec<APathFromTheUser>),
    TheUserChoseNothing,
    /// **No panel was opened at all, because this ask has no route.** Added
    /// 2026-09-12, step 10a round 1 must-do F-11.
    ///
    /// **It used to answer [`Picked::TheUserChoseNothing`]**, which is exactly
    /// what a cancelled panel gives — so a caller could not tell an unbuilt
    /// route from a user who closed the panel, and no panel appeared at all.
    /// The day somebody asks for files and folders together they meet a question
    /// they must answer rather than a control that does nothing.
    ///
    /// **The unbuilt route is a record and not a defect**, and the record is at
    /// the head of this file with the condition that removes it named. What this
    /// variant repairs is the answer, not the absence.
    ThereIsNoPanelForThisAsk(WhatMayBeChosen),
}

/// **The same ask, from a command: the panel is put up on a blocking worker
/// and the command waits for it without holding any thread.**
///
/// **This is the route a command takes**, and it has to be taken from an
/// `async` command. The shell's own page — *Calling Rust from the Frontend*,
/// `https://v2.tauri.app/develop/calling-rust/`, read 2026-09-27, under *Async
/// Commands* — says **"Commands without the _async_ keyword are executed on the
/// main thread unless defined with _#\[tauri::command(async)\]_."** The
/// dialog plugin pinned in `App/Cargo.toml` says of every `blocking_pick_…`
/// **"This is a blocking operation, and should *NOT* be used when running on
/// the main thread."** (`tauri-plugin-dialog` 2.7.3, `src/lib.rs`, *Blocking
/// APIs*). **So a plain command reaching [`ask`] holds the drawing thread for
/// as long as the panel is up, and the panel needs that thread to draw**: the
/// window froze under the Creator's pointer on the seventh pass on Hyperstack,
/// at *Somewhere else…*. Until 2026-09-27 four commands did that.
/// [`tests::no_plain_command_reaches_the_blocking_ask`] is what keeps it from
/// coming back.
///
/// **A panel whose worker did not come back answers as no panel** — the one
/// answer that is neither a choice nor a user who closed it.
pub async fn asked<R: Runtime>(app: &AppHandle<R>, asking: Asking) -> Picked {
    let app = app.clone();
    let what = asking.what;
    tauri::async_runtime::spawn_blocking(move || ask(&app, &asking))
        .await
        .unwrap_or(Picked::ThereIsNoPanelForThisAsk(what))
}

/// **Put the system's own picker up and wait for the answer.**
///
/// It blocks the calling thread until the user has acted, and it must therefore
/// not be called on the interface thread — §3.2. **A command reaches it through
/// [`asked`]**, or from a thread of its own; see there for the shell's own
/// sentence on where a plain command runs.
pub fn ask<R: Runtime>(app: &AppHandle<R>, asking: &Asking) -> Picked {
    // **Both kinds together never reach the plugin**, which has no panel for
    // them: a Mac's own panel takes them, and elsewhere the ask is answered as
    // itself rather than as a user who chose nothing — F-11.
    if asking.what == WhatMayBeChosen::FilesAndFolders {
        return both_kinds(app, asking);
    }

    let mut panel = app.dialog().file().set_title(asking.title.as_str());
    if let Some(place) = asking.starting_at.as_ref() {
        panel = panel.set_directory(place);
    }

    let chose = match (asking.what, asking.how_many) {
        (WhatMayBeChosen::Files, HowMany::One) => panel.blocking_pick_file().map(|one| vec![one]),
        (WhatMayBeChosen::Files, HowMany::Any) => panel.blocking_pick_files(),
        (WhatMayBeChosen::Folders, HowMany::One) => {
            panel.blocking_pick_folder().map(|one| vec![one])
        }
        (WhatMayBeChosen::Folders, HowMany::Any) => panel.blocking_pick_folders(),
        // **Answered above, before any panel was built.** The arm is kept so
        // that adding a fourth kind has to come here rather than compiling into
        // silence.
        (WhatMayBeChosen::FilesAndFolders, _) => return both_kinds(app, asking),
    };

    // **A picker on these three systems answers with a path.** The plugin's
    // own answer can also be an opaque handle, which is a phone's shape and not
    // one of the three systems in the release matrix; where it is one, there
    // is no path from the user and there is nothing to judge, so it is dropped
    // rather than turned into a name.
    the_answer_from(chose.map(|paths| {
        paths
            .into_iter()
            .filter_map(|path| path.into_path().ok())
            .collect()
    }))
}

/// **Files and folders together, on a Mac: its own panel with both switches
/// on** — `File Browser` H3. A panel that could not be put up answers as no
/// panel, never as a user who chose nothing.
#[cfg(target_os = "macos")]
fn both_kinds<R: Runtime>(app: &AppHandle<R>, asking: &Asking) -> Picked {
    match crate::platform::the_open_panel::files_and_folders_are_chosen(
        app,
        asking.how_many == HowMany::Any,
        asking.starting_at.as_deref(),
        asking.title.as_str(),
    ) {
        Ok(chose) => the_answer_from(chose),
        Err(_) => Picked::ThereIsNoPanelForThisAsk(asking.what),
    }
}

/// **Files and folders together, on Windows and Linux: no panel takes both**
/// (`File Browser` H1, H2), so the ask is answered as itself — F-11.
#[cfg(not(target_os = "macos"))]
fn both_kinds<R: Runtime>(_app: &AppHandle<R>, asking: &Asking) -> Picked {
    Picked::ThereIsNoPanelForThisAsk(asking.what)
}

/// What a panel gave back, as the one answer every caller reads.
pub(crate) fn the_answer_from(chose: Option<Vec<PathBuf>>) -> Picked {
    let paths: Vec<APathFromTheUser> = chose
        .unwrap_or_default()
        .into_iter()
        .map(APathFromTheUser)
        .collect();
    if paths.is_empty() {
        Picked::TheUserChoseNothing
    } else {
        Picked::TheUserChose(paths)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **K11, held by the capability file rather than by anybody's care.**
    ///
    /// *One service for every module, reached through this document's adapter
    /// and by no other route.* A plugin's commands are reachable from the page
    /// only where a capability grants them, and this application grants the
    /// dialog plugin's none — nor the filesystem plugin's, which arrives
    /// beneath it. **A single line added to that file would open a second route
    /// to a picker and nothing else in the application would change**, which is
    /// why this is a test.
    #[test]
    fn the_page_is_granted_no_route_to_a_picker_of_its_own() {
        let folder = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("capabilities");
        let mut read = 0;
        for entry in std::fs::read_dir(&folder).expect("the capabilities folder is readable") {
            let path = entry.expect("the entry is readable").path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            read += 1;
            let text = std::fs::read_to_string(&path).expect("the capability is readable");
            for granted in ["dialog:", "fs:"] {
                assert!(
                    !text.contains(granted),
                    "{}: the page is granted `{granted}` permissions. **K11: the \
                     system file picker is reached through this document's \
                     adapter and by no other route**, and H34 binds the page to \
                     the application-service surface and nothing beneath it — a \
                     filesystem or dialog plugin reachable from JavaScript is \
                     both of those broken at once",
                    path.display()
                );
            }
        }
        assert!(read > 0, "no capability file was read, so nothing was checked");
    }

    /// **K13's name is what a caller meets, and it holds nothing else.**
    ///
    /// A picked path comes out under one name and that name says it is
    /// unjudged. This is the line that fails if somebody adds a second reader
    /// that hands the path out as an ordinary one.
    #[test]
    fn a_picked_path_comes_out_under_the_one_name_that_says_it_is_unjudged() {
        let picked = APathFromTheUser(PathBuf::from("/somewhere/the user/chose.blend"));
        assert_eq!(
            picked.as_the_user_gave_it(),
            Path::new("/somewhere/the user/chose.blend")
        );

        let (working, _) = include_str!("picker.rs")
            .split_once("#[cfg(test)]")
            .expect("the tests are the last thing in this file");
        let readers: Vec<&str> = working
            .lines()
            .map(str::trim)
            .filter(|line| line.starts_with("pub fn ") && line.contains("&Path"))
            .collect();
        assert_eq!(
            readers,
            vec!["pub fn as_the_user_gave_it(&self) -> &Path {"],
            "a second way out of `APathFromTheUser` was added. **K13: whatever a \
             picker returns is a path from the user and is judged like any other \
             name before anything is written** — the one name is what makes \
             every site that takes one out a site somebody thought about it at"
        );
    }

    /// **Files and folders together: a Mac's own panel with both switches on,
    /// and no panel on the other two** — `File Browser` H3 and H4, the
    /// Creator's second notes on the test copy, piece 9 (note 2).
    ///
    /// Read from the source, because a panel needs a screen and a person. **Two
    /// things must never happen.** A files-only panel opening where both were
    /// asked for, which silently drops half of the ask. And the ask answering
    /// *the user chose nothing* where no panel was put up, which is what a
    /// cancelled panel gives (F-11).
    #[test]
    fn both_kinds_open_the_macs_own_panel_and_no_panel_elsewhere() {
        let (working, _) = include_str!("picker.rs")
            .split_once("#[cfg(test)]")
            .expect("the tests are the last thing in this file");
        let the_one_after = |opening: &str| -> String {
            let at = working.find(opening).unwrap_or_else(|| panic!("`{opening}` is not in the file"));
            let rest = &working[at..];
            rest[..rest.find("\n}\n").expect("the function ends")].to_string()
        };
        let on_a_mac = the_one_after("#[cfg(target_os = \"macos\")]\nfn both_kinds");
        assert!(
            on_a_mac.contains("the_open_panel::files_and_folders_are_chosen("),
            "on a Mac, files and folders together no longer reach the Mac's own panel:\n{on_a_mac}"
        );
        assert!(
            on_a_mac.contains("Err(_) => Picked::ThereIsNoPanelForThisAsk(asking.what)"),
            "a Mac panel that could not be put up no longer answers as no panel:\n{on_a_mac}"
        );
        let elsewhere = the_one_after("#[cfg(not(target_os = \"macos\"))]\nfn both_kinds");
        assert!(
            elsewhere.contains("Picked::ThereIsNoPanelForThisAsk(asking.what)") && !elsewhere.contains("dialog"),
            "on Windows and Linux, files and folders together no longer answer as no panel:\n{elsewhere}"
        );
        let ask = the_one_after("pub fn ask<R: Runtime>");
        let routed = ask.find("return both_kinds(app, asking);").expect("both kinds are routed");
        assert!(
            routed < ask.find("app.dialog()").expect("the plugin's panel"),
            "both kinds reach the plugin's panel, which takes files or folders and never both"
        );
        let panel = include_str!("platform/the_open_panel.rs");
        for switch in ["setCanChooseFiles(true)", "setCanChooseDirectories(true)"] {
            assert!(panel.contains(switch), "the Mac's panel lost `{switch}`");
        }

        // **A closed panel is nothing chosen; a choice is the paths as given.**
        assert_eq!(the_answer_from(None), Picked::TheUserChoseNothing);
        assert_eq!(the_answer_from(Some(Vec::new())), Picked::TheUserChoseNothing);
        assert_eq!(
            the_answer_from(Some(vec![PathBuf::from("/a/textures"), PathBuf::from("/a/one.png")])),
            Picked::TheUserChose(vec![
                APathFromTheUser(PathBuf::from("/a/textures")),
                APathFromTheUser(PathBuf::from("/a/one.png")),
            ])
        );

        // **A caller has to handle no panel**, which is what the variant is
        // for: a match over `Picked` naming only the two old answers does not
        // compile.
        match Picked::ThereIsNoPanelForThisAsk(WhatMayBeChosen::FilesAndFolders) {
            Picked::TheUserChose(_) | Picked::TheUserChoseNothing => {
                panic!("no panel answers as one of the two ordinary answers")
            }
            Picked::ThereIsNoPanelForThisAsk(what) => {
                assert_eq!(what, WhatMayBeChosen::FilesAndFolders)
            }
        }
    }

    /// **Every command that reaches the blocking ask is off the drawing
    /// thread** — `Desktop app and delivery` §3.2 and K11, and the shell's own
    /// sentence cited at [`asked`]: *"Commands without the async keyword are
    /// executed on the main thread unless defined with
    /// #\[tauri::command(async)\]."*
    ///
    /// **Why this is a reading of the source rather than a run.** The freeze
    /// needs a real panel and a person, because a folder panel ignores a
    /// synthetic click; four commands carried it for weeks with every test
    /// green, and the seventh pass on Hyperstack found it only when the Creator
    /// reached for the panel. So this reads every file under `src/`, finds each
    /// `#[tauri::command]` function, and follows its body — and the bodies of
    /// the same file's functions it calls — to `picker::ask(`. **What runs
    /// inside `spawn_blocking(…)` or `thread::spawn(…)` is not the command's
    /// thread** and is not followed. **A command that reaches it must be
    /// `async` or carry `(async)`**; one that does not fails here by name.
    #[test]
    fn no_plain_command_reaches_the_blocking_ask() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut plain_and_reaching = Vec::new();
        let mut commands_that_put_a_panel_up = Vec::new();
        for (name, text) in rust_files_under(&root) {
            let functions = the_functions_in(&text);
            let on_the_commands_thread = the_ones_that_reach(&functions, &["picker::ask("], |f| {
                &f.on_its_own_thread
            });
            let at_all = the_ones_that_reach(&functions, &["picker::ask(", "picker::asked("], |f| {
                &f.whole
            });
            for function in functions.iter().filter(|function| function.is_a_command) {
                if at_all.contains(&function.name) {
                    commands_that_put_a_panel_up.push(function.name.clone());
                }
                if on_the_commands_thread.contains(&function.name)
                    && !function.runs_off_the_drawing_thread
                {
                    plain_and_reaching.push(format!("{name}: {}", function.name));
                }
            }
        }
        assert!(
            plain_and_reaching.is_empty(),
            "these plain commands put the system's panel up on the drawing \
             thread, which freezes the whole window while it is up: {plain_and_reaching:?}. \
             Make each `async` and ask through `crate::picker::asked`"
        );
        // **The reading found what it is about**: Settings' two, Render
        // Settings' *Somewhere else…*, Files' *Upload…*, the manual download's
        // *…to* (through `the_destination_chosen`, which runs the panel in
        // `spawn_blocking`) and Keys' *Choose a key…* (on a thread of its own)
        // — and only the first four reach the ask on the command's own thread.
        for known in [
            "a_search_folder_is_added",
            "the_download_folder_is_chosen",
            "the_download_choices_are_made",
            "an_upload_is_asked_for",
        ] {
            assert!(
                commands_that_put_a_panel_up.iter().any(|found| found == known),
                "the reading did not find `{known}` putting a panel up, so it is not \
                 reading what it is about: {commands_that_put_a_panel_up:?}"
            );
        }
    }

    struct AFunction {
        name: String,
        is_a_command: bool,
        runs_off_the_drawing_thread: bool,
        /// The body, with every `spawn_blocking(…)` and `thread::spawn(…)`
        /// taken out.
        on_its_own_thread: String,
        /// The body whole.
        whole: String,
    }

    fn rust_files_under(folder: &std::path::Path) -> Vec<(String, String)> {
        let mut found = Vec::new();
        for entry in std::fs::read_dir(folder).expect("the folder reads").flatten() {
            let path = entry.path();
            if path.is_dir() {
                found.extend(rust_files_under(&path));
            } else if path.extension().is_some_and(|e| e == "rs") {
                let text = std::fs::read_to_string(&path).expect("the file reads");
                found.push((path.display().to_string(), text));
            }
        }
        found
    }

    /// **Each function, by the shape this crate is written in**: a line
    /// holding `fn name`, its body ending at the first line that is its own
    /// indentation and a closing brace. The tests at the foot of a file are
    /// left out, being no command's — the module, and not a test-only helper
    /// standing among the working code.
    fn the_functions_in(text: &str) -> Vec<AFunction> {
        let working = text.split("#[cfg(test)]\nmod tests").next().unwrap_or(text);
        let lines: Vec<&str> = working.lines().collect();
        let mut functions = Vec::new();
        for (at, line) in lines.iter().enumerate() {
            let trimmed = line.trim_start();
            let indent = &line[..line.len() - trimmed.len()];
            let signature = trimmed
                .trim_start_matches("pub(crate) ")
                .trim_start_matches("pub ");
            let is_async = signature.starts_with("async fn ");
            let Some(rest) = signature
                .strip_prefix("async fn ")
                .or_else(|| signature.strip_prefix("fn "))
            else {
                continue;
            };
            let name: String = rest
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .collect();
            let mut attributes = Vec::new();
            let mut back = at;
            while back > 0 && lines[back - 1].trim_start().starts_with("#[") {
                back -= 1;
                attributes.push(lines[back].trim());
            }
            let command = attributes
                .iter()
                .find(|attribute| attribute.starts_with("#[tauri::command"));
            let closing = format!("{indent}}}");
            let body: Vec<&str> = lines[at..]
                .iter()
                .skip(1)
                .take_while(|body_line| **body_line != closing)
                .copied()
                .collect();
            functions.push(AFunction {
                name,
                is_a_command: command.is_some(),
                runs_off_the_drawing_thread: is_async
                    || command.is_some_and(|attribute| attribute.contains("async")),
                on_its_own_thread: without_what_is_spawned(&body.join("\n")),
                whole: body.join("\n"),
            });
        }
        functions
    }

    /// The body with each `spawn_blocking(…)` and `thread::spawn(…)` cut out,
    /// its brackets matched.
    fn without_what_is_spawned(body: &str) -> String {
        let mut kept = String::new();
        let mut rest = body;
        loop {
            let next = ["spawn_blocking(", "thread::spawn("]
                .iter()
                .filter_map(|opening| rest.find(opening).map(|at| (at, opening.len())))
                .min();
            let Some((at, opening)) = next else {
                kept.push_str(rest);
                return kept;
            };
            kept.push_str(&rest[..at]);
            let mut depth = 1;
            let mut end = rest.len();
            for (offset, character) in rest[at + opening..].char_indices() {
                match character {
                    '(' => depth += 1,
                    ')' => {
                        depth -= 1;
                        if depth == 0 {
                            end = at + opening + offset + 1;
                            break;
                        }
                    }
                    _ => {}
                }
            }
            rest = &rest[end..];
        }
    }

    /// The names of the functions whose `body` reaches one of `calls`,
    /// directly or through the same file's functions.
    fn the_ones_that_reach(
        functions: &[AFunction],
        calls: &[&str],
        body: fn(&AFunction) -> &String,
    ) -> Vec<String> {
        let mut reaching: Vec<String> = functions
            .iter()
            .filter(|function| calls.iter().any(|call| body(function).contains(call)))
            .map(|function| function.name.clone())
            .collect();
        loop {
            let more: Vec<String> = functions
                .iter()
                .filter(|function| !reaching.contains(&function.name))
                .filter(|function| {
                    let text = body(function);
                    reaching.iter().any(|name| {
                        text.match_indices(&format!("{name}(")).any(|(at, _)| {
                            !text[..at].ends_with(|c: char| c.is_alphanumeric() || c == '_')
                        })
                    })
                })
                .map(|function| function.name.clone())
                .collect();
            if more.is_empty() {
                return reaching;
            }
            reaching.extend(more);
        }
    }

    /// **X5, which is why the title is `Prose`.**
    ///
    /// *A great deal of what the user reads is drawn by no engine at all* —
    /// the system file picker's own chrome among the three X5 names. A field
    /// that took a `String` would compile, would draw, and would have skipped
    /// the typography point.
    #[test]
    fn the_panels_own_title_comes_by_the_prose_route() {
        let asking = Asking {
            what: WhatMayBeChosen::Files,
            how_many: HowMany::One,
            starting_at: None,
            title: Prose::new("Choose a Blender file"),
        };
        let _: &Prose = &asking.title;
        assert_eq!(asking.title.as_str(), "Choose a Blender\u{00A0}file");
    }
}
