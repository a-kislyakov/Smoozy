//! **The command surface: one file per service, and one registration list.**
//!
//! **READ THIS BEFORE ADDING A COMMAND. It is the layout rule for the seven
//! pieces of step 10a after piece A and for the eleven screens after them, and
//! it is written here rather than only in the decisions record because this is
//! the file somebody opens when they need it.**
//!
//! ## Why the folder exists
//!
//! Until 2026-09-08 every command the application exposed to its page was a
//! free function in `main.rs`, named in one `invoke_handler` list in the same
//! file. There were three of them and all three were step 1's. **Eight pieces
//! and then eleven screens all add commands**, and eight agents editing one
//! function list in one file that also holds the window construction is a merge
//! nobody wins. **So the surface is split: one file per service, and this file
//! holds nothing but the list.**
//!
//! ## The rule
//!
//! 1. **A service that the page calls gets its own file here**, named for the
//!    service and not for the screen — `chrome.rs`, `the_event_route.rs`, and
//!    whatever the pieces after add. Its commands, its request and response
//!    types, and the event kinds it posts all live in that one file.
//! 2. **[`the_registration_list`] is the only shared thing, and a piece adds
//!    exactly one line to it** — the module declaration above it and one entry
//!    in the list. That is a one-line change in a file whose other lines nobody
//!    else is touching, which merges.
//! 3. **Nothing else in this crate registers a command.** `main.rs` passes this
//!    list and holds none of its own; the test below is what says so.
//! 4. **Every command sits inside the privileged web-content boundary** —
//!    §2.8 H31, H33, H33a, H34 and H36, built at [`crate::web_content_boundary`].
//!    H34 in particular: *"Each exported method has a closed request and
//!    response type, validates lengths and enum values before touching an
//!    adapter, and returns only the values its owning module permits."* **No
//!    command hands the page a raw filesystem, process, shell, database, secret
//!    store or network object**, and none returns a bare `String` that is
//!    really prose — prose leaves by [`crate::typography::Prose`] and a value by
//!    [`crate::typography::Verbatim`], which is A6 and X9.
//! 5. **A command that returns a result to the caller is not how a result
//!    reaches a screen.** P6: results reach the interface as events on the one
//!    ordered queue, and *"the interface holds no other route to a module"*.
//!    **A command is the page asking for something; an event is the application
//!    telling it.** A piece that finds itself returning a module's outcome from
//!    a command has built the second route P6 forbids.
//!
//! ## The page's own file layout, which is the other half of the same rule
//!
//! It is stated at the head of `web/smoozy.js`, which is where somebody writing
//! page code will be. In short: **three files under `web/` are shared and each
//! takes a one-line addition** — `index.html`, `smoozy.js`'s import list and
//! `smoozy.css`'s `@import` list — **and everything else a piece draws is its
//! own `web/<service>.js` and `web/<service>.css`.**

pub mod a_server;
pub mod chrome;
pub mod files;
pub mod keys;
pub mod menus;
pub mod panes;
pub mod render_settings;
pub mod reporting;
pub mod said_once;
pub mod servers;
pub mod set_up;
pub mod settings;
pub mod telegram;
pub mod the_dependency_check;
pub mod the_event_route;
pub mod the_rail;
pub mod the_failure_shape;
pub mod the_history;
pub mod the_machines_own_words;
pub mod the_queue;
pub mod the_underway_shape;
pub mod transfers;

#[cfg(test)]
mod selecting_text;

/// **The one registration list.** A piece adds one line.
///
/// It is a function rather than the macro written into `main.rs`, so that the
/// list lives beside the files it names and `main.rs` holds the window and
/// nothing of the surface.
pub fn the_registration_list<R: tauri::Runtime>(
) -> impl Fn(tauri::ipc::Invoke<R>) -> bool + Send + Sync + 'static {
    tauri::generate_handler![
        a_server::the_servers_own_screen,
        chrome::chrome_text,
        chrome::system_name,
        chrome::linux_button_layout,
        files::the_files_tab,
        files::the_pick_is_dropped,
        files::the_folder_is_read,
        files::the_list_is_sorted_by,
        files::the_rows_are_put_in_order,
        files::the_upload_is_confirmed,
        files::the_already_exists_question_was_answered,
        files::the_files_the_check_placed_go_through_it,
        files::what_the_upload_came_to,
        files::the_transfers_on_this_server,
        files::an_upload_is_asked_for,
        files::files_were_dropped,
        files::what_is_being_dropped,
        files::the_delete_of,
        files::what_disturbing_costs,
        files::a_file_act_that_needs_a_connection,
        files::a_new_folder_is_made,
        files::a_rename_is_made,
        files::a_delete_is_made,
        files::a_cut_or_copy_is_taken,
        files::what_a_paste_costs,
        files::a_paste_is_made,
        files::a_folders_size_is_counted,
        files::the_delete_warning_was_switched_off,
        files::show_hidden_files_was_ticked,
        files::the_saved_with_version_is_read,
        files::a_picture_is_read,
        files::the_picture_is_let_go,
        files::the_words_for_how_many_are_chosen,
        keys::the_keys_screen,
        keys::the_types_the_dialog_may_offer,
        keys::a_key_is_made,
        keys::a_key_is_pointed_at,
        keys::a_public_half_is_copied,
        keys::what_taking_a_key_away_costs,
        keys::a_key_is_taken_away,
        keys::a_remembered_passphrase_is_cleared,
        keys::the_passphrase_box_was_answered,
        keys::the_passphrase_box_was_cancelled,
        menus::the_menu_bar,
        menus::a_menu_item_was_chosen,
        menus::the_held_update_is_installed_anyway,
        panes::the_pane_layout,
        panes::a_pane_drag_ended,
        panes::a_pane_width_was_reset,
        servers::the_all_servers_screen,
        servers::the_work_column_is_on_screen,
        servers::the_add_dialog,
        servers::the_edit_dialog,
        servers::the_removal_of,
        servers::the_add_is_checked,
        servers::the_pasted_line_is_read,
        servers::a_server_is_added,
        servers::an_edit_is_saved,
        servers::a_server_is_removed,
        servers::an_act_that_needs_a_connection,
        servers::the_key_a_dialog_holds,
        servers::the_keys_a_dialog_offers,
        servers::the_keys_a_dialog_offers_are_checked,
        servers::the_edit_is_checked,
        servers::a_remembered_password_is_forgotten,
        render_settings::the_render_settings_panel,
        render_settings::an_answer_was_given,
        render_settings::a_value_is_handed_back,
        render_settings::a_scene_is_ticked,
        render_settings::every_scene_is_ticked,
        render_settings::a_view_layer_is_pinned,
        render_settings::a_pin_is_handed_back,
        render_settings::the_file_was_pointed_at,
        render_settings::the_file_is_read,
        render_settings::the_read_failure_is_cleared,
        render_settings::the_missing_assets_warning_is_switched_off,
        render_settings::a_job_is_submitted,
        render_settings::the_version_list_is_asked_for,
        render_settings::the_page_was_left,
        render_settings::the_download_panel,
        render_settings::the_download_choices_are_made,
        render_settings::the_folder_is_asked_for_in_the_box,
        reporting::the_report_box_is_opened_from_a_failure,
        reporting::a_folder_is_chosen_for_the_report,
        reporting::the_report_is_saved,
        reporting::the_report_box_is_shut,
        set_up::the_set_up_tab,
        set_up::the_install_list,
        set_up::the_setup_under_way,
        set_up::the_servers_output,
        set_up::the_work_warning,
        set_up::what_removing_a_blender_costs,
        set_up::the_admin_access_box,
        set_up::a_set_up_act_that_needs_a_connection,
        set_up::the_admin_access_is_answered,
        settings::the_settings_screen,
        settings::a_search_folder_is_added,
        settings::a_search_folder_is_moved,
        settings::a_search_folder_is_removed,
        settings::the_download_folder_is_chosen,
        settings::a_speed_cap_is_set,
        settings::a_notification_switch_is_turned,
        settings::the_speeds_in_settings,
        telegram::telegram_is_switched,
        telegram::the_telegram_walkthrough_is_opened,
        telegram::another_username_is_offered,
        telegram::the_telegram_walkthrough_is_shut,
        telegram::the_pasted_token_is_checked,
        telegram::the_bot_is_kept_again,
        telegram::the_replacement_token_is_saved,
        telegram::the_reporting_default_is_set,
        telegram::the_default_minutes_are_typed,
        said_once::the_warnings_switched_off,
        said_once::every_switched_off_warning_is_restored,
        the_dependency_check::the_dependency_check_panel,
        the_dependency_check::the_check_is_started,
        the_dependency_check::an_act_on_the_check,
        the_event_route::the_page_is_ready,
        the_failure_shape::the_failures_standing_against,
        the_failure_shape::a_failure_was_cleared,
        the_history::the_history,
        the_machines_own_words::the_machines_own_words_for,
        the_queue::the_queue,
        the_queue::an_act_on_a_job,
        the_queue::the_untracked_work_is_ignored,
        the_queue::the_preview_picture,
        the_rail::the_rail,
        the_underway_shape::what_is_underway_against,
        transfers::the_transfers_screen, transfers::the_stop_box, transfers::the_format_box, transfers::an_act_on_transfers,
        transfers::a_whole_job_download_is_asked_for,
        transfers::a_file_download_is_asked_for,
        transfers::the_download_question_was_answered,
        transfers::a_file_is_sent_again,
        transfers::the_files_a_line_names,
        transfers::the_formats_are_chosen_for_a_job,
    ]
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    fn the_surface_folder() -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("src/surface")
    }

    fn this_file() -> &'static str {
        include_str!("mod.rs")
    }

    /// **Every file in this folder is in the registration list.**
    ///
    /// **This is the rule's teeth.** A piece that writes its own file here and
    /// forgets the one line it owes has built a command the page cannot call,
    /// and nothing about that is visible: the crate compiles, every test in
    /// that piece's own file passes, and the failure shows only when somebody
    /// clicks. This is the line that fails instead.
    #[test]
    fn every_file_of_the_surface_is_in_the_registration_list() {
        let list = this_file();
        for entry in fs::read_dir(the_surface_folder()).expect("the folder is readable") {
            let path = entry.expect("the entry is readable").path();
            if path.extension().and_then(|e| e.to_str()) != Some("rs") {
                continue;
            }
            let name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .expect("the file has a name");
            // Test-only page checks expose no application service.
            if name == "mod" || list.contains(&format!("#[cfg(test)]\nmod {name};")) {
                continue;
            }
            assert!(
                list.contains(&format!("pub mod {name};")),
                "src/surface/{name}.rs is not declared in src/surface/mod.rs"
            );
            assert!(
                list.contains(&format!("{name}::")),
                "src/surface/{name}.rs names no command in `the_registration_list`. \
                 **One file per service and one line in the list is the whole of \
                 the rule**, and a command that is not in the list is one the page \
                 cannot call, with nothing anywhere saying so"
            );
        }
    }

    /// **Every command in this folder is in the registration list.**
    ///
    /// The test above catches a file nobody registered; this one catches a
    /// second command added to a file that is already registered, which is the
    /// likelier mistake and the one that leaves the first test passing.
    #[test]
    fn every_command_of_the_surface_is_in_the_registration_list() {
        let list = this_file();
        for entry in fs::read_dir(the_surface_folder()).expect("the folder is readable") {
            let path = entry.expect("the entry is readable").path();
            if path.extension().and_then(|e| e.to_str()) != Some("rs") {
                continue;
            }
            let name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .expect("the file has a name")
                .to_string();
            if name == "mod" {
                continue;
            }
            let text = fs::read_to_string(&path).expect("the file is readable");
            for command in the_commands_in(&text) {
                assert!(
                    list.contains(&format!("{name}::{command}")),
                    "src/surface/{name}.rs exposes `{command}` and the registration \
                     list does not name it, so the page cannot call it"
                );
            }
        }
    }

    /// **A quoted mention of the attribute is not a command.**
    ///
    /// The reader above is a substring search over a file that also contains
    /// prose and tests about commands. **Without the line-of-its-own rule it
    /// names the next function after any mention**, which is how a test in
    /// `render_settings.rs` was reported as a command the registration list had
    /// forgotten.
    #[test]
    fn a_command_is_an_attribute_on_a_line_of_its_own_and_never_a_mention_of_one() {
        // Written this way so that no line of **this** file is the attribute
        // on a line of its own: the reader would then read the fixture as a
        // command of `mod.rs`.
        let attribute = "#[tauri::command]";
        let source = format!(
            "{attribute}\n\
             pub fn a_real_one(started: State<'_, Mutex<Started>>) {{}}\n\
             \n\
             #[cfg(test)]\n\
             mod tests {{\n\
             \x20   #[test]\n\
             \x20   fn a_test_that_reads_the_source() {{\n\
             \x20       for (at, _) in working.match_indices(\"{attribute}\") {{\n\
             \x20           let _ = at;\n\
             \x20       }}\n\
             \x20   }}\n\
             \x20   fn the_next_function_after_the_mention() {{}}\n\
             }}\n"
        );
        assert_eq!(the_commands_in(&source), vec!["a_real_one".to_string()]);
    }

    /// **`main.rs` registers this list and holds no command of its own.**
    ///
    /// Rule 3 of the layout above. **A command left in `main.rs` is the
    /// collision this whole folder exists to remove**, and it would work
    /// perfectly, which is why it needs a test rather than a habit.
    #[test]
    fn the_window_registers_this_list_and_nothing_else() {
        let window = include_str!("../main.rs");
        assert!(
            window.contains("surface::the_registration_list()"),
            "the window no longer registers the one command surface"
        );
        assert!(
            the_commands_in(window).is_empty(),
            "`main.rs` declares a command of its own. **Every command lives in \
             its own file under `src/surface/` and is named in the one \
             registration list**: eight pieces of step 10a and eleven screens \
             after them all add commands, and one list in the file that also \
             holds the window construction is the merge this folder was made to \
             avoid"
        );
    }

    /// **No command hands the page anything beneath the application-service
    /// surface** — §2.8 H34.
    ///
    /// H34's own words: *"No raw filesystem, process execution, shell,
    /// database, secret-store or unrestricted network object is bound into
    /// JavaScript."* The check is over the text of every file here, and it is
    /// narrow on purpose: it catches a command whose **signature** names one of
    /// those, which is the shape the mistake actually takes.
    #[test]
    fn no_command_binds_anything_beneath_the_service_surface() {
        for entry in fs::read_dir(the_surface_folder()).expect("the folder is readable") {
            let path = entry.expect("the entry is readable").path();
            if path.extension().and_then(|e| e.to_str()) != Some("rs") {
                continue;
            }
            if path.file_stem().and_then(|s| s.to_str()) == Some("mod") {
                continue;
            }
            let name = path.display().to_string();
            let text = fs::read_to_string(&path).expect("the file is readable");
            for at in the_command_attributes_in(&text) {
                let rest = &text[at..];
                let signature =
                    &rest[..rest.find(" {").unwrap_or(rest.len()).min(rest.len())];
                for beneath in [
                    "LocalRecord",
                    "Connection",
                    "DiaryService",
                    "PathBuf",
                    "&Path",
                    "File",
                    "Command",
                    "TcpStream",
                    "Session",
                    "Secret",
                ] {
                    assert!(
                        !signature.contains(beneath),
                        "{name}: a command's signature names {beneath}. **H34: the \
                         binding surface is the application-service surface and \
                         nothing beneath it** — no raw filesystem, process, shell, \
                         database, secret-store or network object is bound into the \
                         page"
                    );
                }
            }
        }
    }

    // ── The page's own half of the layout rule ──────────────────────────────
    //
    // The rule is stated at the head of `web/smoozy.js` and the tests that hold
    // it are here, beside the Rust half's, because they are one rule. **Every
    // one of them fails on a piece that wrote its own file and forgot the one
    // line it owed**, which is the mistake the rule exists to make impossible
    // to make quietly.

    fn the_page_folder() -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("web")
    }

    /// Every file in `web/`, as (name, text).
    fn the_page_files() -> Vec<(String, String)> {
        let mut out = Vec::new();
        for entry in fs::read_dir(the_page_folder()).expect("the page folder is readable") {
            let path = entry.expect("the entry is readable").path();
            if !path.is_file() {
                continue;
            }
            out.push((
                path.file_name()
                    .and_then(|s| s.to_str())
                    .expect("the file has a name")
                    .to_string(),
                fs::read_to_string(&path).expect("the file is readable"),
            ));
        }
        out.sort();
        out
    }

    /// **Every module in `web/` is in `smoozy.js`'s import list, and every
    /// stylesheet is in `smoozy.css`'s `@import` list.**
    ///
    /// **This is the page half of the rule's teeth.** A piece that writes
    /// `web/its-thing.js` and does not add the one import line has written a
    /// module the page never loads: the application builds, the window opens,
    /// nothing draws, and nothing anywhere says why. Same for a stylesheet,
    /// where the failure is quieter still — the thing draws, unstyled, and
    /// looks like a design mistake.
    #[test]
    fn every_page_file_is_named_in_the_one_list_that_loads_it() {
        let files = the_page_files();
        let entry = files
            .iter()
            .find(|(name, _)| name == "smoozy.js")
            .map(|(_, text)| text.clone())
            .expect("the page has its entry module");
        let sheet = files
            .iter()
            .find(|(name, _)| name == "smoozy.css")
            .map(|(_, text)| text.clone())
            .expect("the page has its one stylesheet link");

        for (name, _) in &files {
            if name.ends_with(".js") && name != "smoozy.js" {
                assert!(
                    entry.contains(&format!("\"./{name}\"")),
                    "web/{name} is in the folder and not in smoozy.js's import \
                     list, so nothing loads it. **A piece adds exactly one line \
                     to that list** — the rule is at the head of that file"
                );
            }
            if name.ends_with(".css") && name != "smoozy.css" {
                assert!(
                    sheet.contains(&format!("@import \"{name}\";")),
                    "web/{name} is in the folder and not in smoozy.css's @import \
                     list, so nothing loads it and whatever it styles draws \
                     unstyled. **A piece adds exactly one line to that list**"
                );
            }
        }
    }

    /// **`index.html` carries one script and one stylesheet link, and a piece
    /// adds nothing to it.**
    ///
    /// Rule 1 of the layout. It is also H33 held in the one place it can be
    /// broken without the build noticing: `script-src 'self'` and
    /// `style-src 'self'` mean **no inline script and no inline style**, and
    /// the mockup this application's screens are lifted from carries 87 inline
    /// `style=` attributes and would carry them straight in.
    #[test]
    fn the_document_carries_one_script_one_stylesheet_and_nothing_inline() {
        let document = fs::read_to_string(the_page_folder().join("index.html"))
            .expect("the document is readable");

        assert_eq!(
            document.matches("<script").count(),
            1,
            "the document has more than one script tag. **A piece adds nothing \
             to index.html**: it writes its own module and one line in \
             smoozy.js's import list"
        );
        assert!(
            document.contains("<script type=\"module\" src=\"smoozy.js\"></script>"),
            "the one script tag is no longer the module entry"
        );
        assert_eq!(
            document.matches("<link rel=\"stylesheet\"").count(),
            1,
            "the document has more than one stylesheet link. **Every other \
             stylesheet arrives through smoozy.css's @import list**"
        );
        assert!(
            !document.contains(" style=\""),
            "the document carries an inline style attribute. **H33: \
             style-src 'self'**, so an inline style is inert in the shipped \
             application — it does nothing, it reports nothing, and the thing \
             it was meant to style simply draws wrong"
        );
        assert!(
            !document.contains(" onclick=") && !document.contains(" onload="),
            "the document carries an event-handler attribute. **H32: values are \
             content, never markup**, and H33's script-src makes one inert"
        );
    }

    /// **H32, over every file of the page: values are content, never markup.**
    ///
    /// H32's own closed list. The mockup uses `innerHTML` in one place to cache
    /// a variant out of the DOM, so this is a real habit arriving with the lift
    /// rather than a hypothetical one.
    #[test]
    fn nothing_on_the_page_turns_a_value_into_markup() {
        for (name, text) in the_page_files() {
            if !name.ends_with(".js") {
                continue;
            }
            // Comment lines are read past. **Three of these files name the
            // habits below in order to forbid them**, and a check that could
            // not tell a prohibition from an instance would be one nobody could
            // write the prohibition beside.
            let text: String = text
                .lines()
                .filter(|line| {
                    let line = line.trim_start();
                    !(line.starts_with("//") || line.starts_with('*') || line.starts_with("/*"))
                })
                .collect::<Vec<_>>()
                .join("\n");
            for markup in [
                "innerHTML",
                "outerHTML",
                "insertAdjacentHTML",
                "document.write",
                "eval(",
                "new Function(",
            ] {
                assert!(
                    !text.contains(markup),
                    "web/{name} uses {markup}. **H32: filenames, paths, computer \
                     names, server names, captured streams, logs and every other \
                     value not compiled into the interface are inserted with \
                     text-node or value APIs** — never passed to markup \
                     compilation of any kind"
                );
            }
        }
    }

    // ── A6 and X1's page half ───────────────────────────────────────────────
    //
    // **Added 2026-09-12, out of round 1 of the code audit of what is drawn
    // inside the window (F-7).** A6 and X1 make the typography point *"the only
    // way prose reaches a screen"* and say the enforcement is **structural on
    // both sides**. It was structural on one: the Rust half carries prose in
    // [`crate::typography::Prose`] and values in
    // [`crate::typography::Verbatim`], and nothing at all read what a string on
    // the page was or which route it came by. **Both auditors of that round
    // named this as the cheapest repair in it**, and ten of its twenty findings
    // were page-half faults an instrument would have caught in a moment.

    /// One string literal found in a page file: where it starts, and its
    /// contents with every `${…}` substitution taken out.
    ///
    /// **The substitutions come out because what is being judged is the words
    /// the file writes itself.** A template holding two values and a space
    /// writes no words; one holding two values and *of* between them writes a
    /// word, and that word is prose the page composed.
    struct ALiteral {
        at: usize,
        words: String,
        /// Whether the literal begins or ends with a space, which is the shape
        /// a fragment being concatenated on to a value takes.
        joined_on: bool,
    }

    /// Every string literal in a piece of page source, comment lines already
    /// taken out.
    fn the_literals_in(text: &str) -> Vec<ALiteral> {
        let bytes = text.as_bytes();
        let mut out = Vec::new();
        let mut at = 0;
        while at < bytes.len() {
            let quote = bytes[at];
            if quote != b'"' && quote != b'\'' && quote != b'`' {
                at += 1;
                continue;
            }
            let opens = at;
            at += 1;
            let from = at;
            while at < bytes.len() && bytes[at] != quote {
                if bytes[at] == b'\\' {
                    at += 2;
                    continue;
                }
                at += 1;
            }
            // The slice is taken out of the text rather than built byte by
            // byte, so a word with an em dash or an ellipsis in it reads back
            // as itself when the fault is reported.
            let raw = text
                .get(from..at.min(text.len()))
                .unwrap_or_default()
                .to_string();
            at += 1;
            // Take the substitutions out.
            let mut words = String::new();
            let mut rest = raw.as_str();
            while let Some(begins) = rest.find("${") {
                words.push_str(&rest[..begins]);
                match rest[begins..].find('}') {
                    Some(ends) => rest = &rest[begins + ends + 1..],
                    None => {
                        rest = "";
                        break;
                    }
                }
            }
            words.push_str(rest);
            let joined_on = words.len() > words.trim().len() && !words.trim().is_empty();
            out.push(ALiteral {
                at: opens,
                words,
                joined_on,
            });
        }
        out
    }

    /// Whether a piece of text holds a run of two or more letters.
    fn holds_a_word(text: &str) -> bool {
        let mut run = 0;
        for letter in text.chars() {
            if letter.is_ascii_alphabetic() {
                run += 1;
                if run == 2 {
                    return true;
                }
            } else {
                run = 0;
            }
        }
        false
    }

    /// Whether a piece of text holds two words with a space between them.
    fn holds_two_words(text: &str) -> bool {
        text.split(' ').filter(|part| holds_a_word(part)).count() > 1
    }

    /// **A6 and X1: the interface accepts prose from nowhere else, and the page
    /// half is held here rather than by whoever writes a screen.**
    ///
    /// **Three legs, and each is a shape a written-in string actually takes.**
    ///
    /// 1. **A literal with letters in it reaching the text of an element or an
    ///    accessible name.** `textContent`, `createTextNode`, `aria-label`,
    ///    `title`, `placeholder`, `alt`.
    /// 2. **A literal holding two words with a space between them**, wherever
    ///    it is — which catches prose a helper composes and hands to a sink
    ///    somewhere else. **A class list is exempt and is the only exemption**:
    ///    `className` and `classList` carry the drawing's own class names, which
    ///    are not words a person reads.
    /// 3. **A literal with letters that begins or ends with a space**, which is
    ///    the shape a fragment being joined on to a value takes — *as of*, on
    ///    the end of a figure.
    ///
    /// **What it cannot see, said rather than glossed.** A word of one or two
    /// letters standing alone with no space around it and no sink beside it; a
    /// string built from characters; and anything in a file this folder does not
    /// read. **It is a text check over the page and it is written out so nobody
    /// mistakes it for a compiler's answer** — the same honesty
    /// [`crate::presentation`]'s own sweeps are written with.
    ///
    /// **The fix is never to write the string differently here.** It is to
    /// compose the sentence on the application side, each sentence its own
    /// string (X10), put it on the event as `Said::Prose`, and draw what
    /// arrives — with names and numbers beside the words on the verbatim route
    /// rather than inside them (X7).
    #[test]
    fn no_prose_is_written_into_the_page() {
        let sinks = [
            "textContent",
            "createTextNode(",
            "aria-label",
            "ariaLabel",
            ".title =",
            ".placeholder =",
            ".alt =",
        ];
        let mut faults = Vec::new();
        for (name, text) in the_page_files() {
            if !name.ends_with(".js") {
                continue;
            }
            let working: String = text
                .lines()
                .filter(|line| {
                    let line = line.trim_start();
                    !(line.starts_with("//") || line.starts_with('*') || line.starts_with("/*"))
                })
                .collect::<Vec<_>>()
                .join("\n");

            // Where a sink stands, and how far its statement runs.
            let mut spans = Vec::new();
            for sink in sinks {
                for (at, _) in working.match_indices(sink) {
                    let from = at + sink.len();
                    let to = working[from..]
                        .find(';')
                        .map(|found| from + found)
                        .unwrap_or(working.len());
                    // `setAttribute("aria-label", …)` names the attribute
                    // first; the name is not text a person reads.
                    let skip_the_first = sink == "aria-label";
                    spans.push((from, to, skip_the_first));
                }
            }

            for literal in the_literals_in(&working) {
                let exempt = working[..literal.at]
                    .rsplit(';')
                    .next()
                    .map(|statement| {
                        statement.contains("className") || statement.contains("classList")
                    })
                    .unwrap_or(false);

                if holds_a_word(&literal.words) {
                    let mut at_a_sink = false;
                    for (from, to, skip_the_first) in &spans {
                        if literal.at < *from || literal.at >= *to {
                            continue;
                        }
                        if *skip_the_first && literal.at < from + 2 {
                            continue;
                        }
                        at_a_sink = true;
                    }
                    if at_a_sink {
                        faults.push(format!(
                            "web/{name} writes {:?} straight on to the page. **A6 and \
                             X1: every string of prose passes through the typography \
                             point and the interface accepts prose from nowhere \
                             else**, and X1 says the enforcement is structural on \
                             both sides. Compose it on the application side and put \
                             it on the event",
                            literal.words
                        ));
                        continue;
                    }
                    if literal.joined_on {
                        faults.push(format!(
                            "web/{name} holds {:?}, which begins or ends with a space \
                             — the shape a piece of prose takes when it is joined on \
                             to a value. **X7: a name, a number, a path or a port \
                             never travels inside a sentence**; the words belong on \
                             the event beside the value",
                            literal.words
                        ));
                        continue;
                    }
                }
                if holds_two_words(&literal.words) && !exempt {
                    faults.push(format!(
                        "web/{name} holds {:?}, which is two words a person would \
                         read. **A6 and X1: prose reaches the interface through the \
                         typography point and by no other way.** Compose it on the \
                         application side; a class list is the one thing this leg \
                         exempts",
                        literal.words
                    ));
                }
            }
        }
        assert!(faults.is_empty(), "{faults:#?}");
    }

    /// **Where each real `#[tauri::command]` attribute starts in a piece of
    /// source.**
    ///
    /// **An attribute is a line of its own, and nothing else counts** — the
    /// same spelling inside a quoted string is a mention, not a declaration.
    /// Two tests here and one in `render_settings.rs` read the surface's own
    /// source for that spelling, so the text a file writes about commands is
    /// itself scanned, and a plain substring search reads a test's
    /// `match_indices("#[tauri::command]")` as a fourteenth command and then
    /// names whatever function follows it. That is what made
    /// `every_command_of_the_surface_is_in_the_registration_list` report
    /// `render_settings::the_two_dead_controls_are_dead_and_this_is_the_line_that_says_when_they_are_not`,
    /// which is a test and is not public to the page at all.
    fn the_command_attributes_in(text: &str) -> Vec<usize> {
        let mut out = Vec::new();
        let mut at = 0usize;
        for line in text.split_inclusive('\n') {
            if line.trim() == "#[tauri::command]" {
                out.push(at);
            }
            at += line.len();
        }
        out
    }

    /// The names of the `#[tauri::command]` functions in a piece of source.
    fn the_commands_in(text: &str) -> Vec<String> {
        let mut out = Vec::new();
        for at in the_command_attributes_in(text) {
            let rest = &text[at..];
            let Some(opens) = rest.find("fn ") else {
                continue;
            };
            let named = &rest[opens + 3..];
            let Some(closes) = named.find(['(', '<']) else {
                continue;
            };
            out.push(named[..closes].trim().to_string());
        }
        out
    }

    /// **Return in a box presses the box's own affirmative control, and never
    /// a red one** — `Preparation/Design/Whole app.md`, *Keyboard shortcuts*.
    ///
    /// **This runs the page code rather than reading it**: `web/the-return-key.js`
    /// is loaded under Node beside a stand-in for the little of a document it
    /// reads, and Return is pressed in a field, on a focused *Cancel*, on a
    /// tick box, in a box whose answer is red, and in a box with two action
    /// rows. **Before 2026-09-19 no box had the rule** — the file did not
    /// exist, so every case below fails; the Gemini catch-up of step 10a, Area
    /// 5 (F-4), and the build runner's sweep of every box. **Skipped, and says
    /// so, where no Node is installed.**
    #[test]
    fn return_in_a_box_presses_its_affirmative_and_never_a_red_one() {
        let folder = std::env::temp_dir().join(format!(
            "smoozy-return-key-harness-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&folder);
        fs::create_dir_all(&folder).expect("a scratch folder");
        fs::copy(
            the_page_folder().join("the-return-key.js"),
            folder.join("the-return-key.js"),
        )
        .expect("the page module is copied");
        fs::write(folder.join("harness.js"), super::THE_RETURN_KEY_HARNESS).expect("the harness is written");
        let ran = std::process::Command::new("node")
            .arg(folder.join("harness.js"))
            .current_dir(&folder)
            .output();
        let _ = fs::remove_dir_all(&folder);
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the Return key's page code was not run");
            return;
        };
        assert!(
            ran.status.success(),
            "the page code did not run: {}",
            String::from_utf8_lossy(&ran.stderr)
        );
        assert_eq!(
            String::from_utf8_lossy(&ran.stdout).trim(),
            r#"{"fieldPressesTheAffirmative":true,"aFocusedCancelDoesNotSwallowIt":true,"aTickBoxKeepsIt":true,"anAnsweredReturnIsLeftAlone":true,"anotherKeyIsNotTaken":true,"aRedAffirmativeTakesNoReturn":true,"theShownAffirmativeIsPressed":true,"noBoxNoReturn":true}"#
        );
    }

    /// **Every box in `web/` is a `.mbox`**, which is what the Return key's one
    /// listener keys on — so a box built another way would silently lack the
    /// rule. A file that builds a `modal` wrapper builds its box inside it as a
    /// `.mbox`; the one exception is the machine's own words, whose `modal` is a
    /// panel of raw text with no affirmative control.
    #[test]
    fn every_box_in_the_page_is_one_the_return_key_reaches() {
        for (name, text) in the_page_files() {
            if !name.ends_with(".js") || name == "the-machines-own-words.js" {
                continue;
            }
            let builds_a_box = text.contains("\"modal\"")
                || text.contains("\"modal top\"")
                || text.contains("\"modal\" })");
            if builds_a_box {
                assert!(
                    text.contains("mbox"),
                    "web/{name} builds a box that is not a .mbox, so Return does not \
                     reach its affirmative control"
                );
            }
        }
    }
}

/// The stand-in document the Return key's page code is run against.
#[cfg(test)]
const THE_RETURN_KEY_HARNESS: &str = r#"
// A stand-in for the little of a document the module reads.
class Element {
  constructor(tag, classes = [], attrs = {}) {
    this.tagName = tag;
    this.classes = new Set(classes);
    this.attrs = attrs;
    this.children = [];
    this.parent = null;
    this.hidden = false;
    this.disabled = false;
    this.isContentEditable = false;
    this.clicks = 0;
  }
  get classList() {
    return { contains: (one) => this.classes.has(one) };
  }
  getAttribute(name) {
    return name in this.attrs ? this.attrs[name] : null;
  }
  append(...kids) {
    for (const kid of kids) {
      kid.parent = this;
      this.children.push(kid);
    }
    return this;
  }
  get isConnected() {
    let at = this;
    while (at.parent) at = at.parent;
    return at === body;
  }
  closest(selector) {
    if (selector !== "[hidden]") throw new Error("unexpected " + selector);
    for (let at = this; at; at = at.parent) if (at.hidden) return at;
    return null;
  }
  getClientRects() {
    return this.isConnected && !this.closest("[hidden]") ? [1] : [];
  }
  contains(other) {
    for (let at = other; at; at = at.parent) if (at === this) return true;
    return false;
  }
  all() {
    return [this, ...this.children.flatMap((kid) => kid.all())];
  }
  querySelectorAll(selector) {
    const everything = this.all().slice(1);
    if (selector === ".mbox") return everything.filter((one) => one.classes.has("mbox"));
    if (selector === "button.btn") {
      return everything.filter((one) => one.tagName === "BUTTON" && one.classes.has("btn"));
    }
    throw new Error("unexpected " + selector);
  }
  click() {
    this.clicks += 1;
  }
}
globalThis.Element = Element;
const body = new Element("BODY");
const listeners = [];
globalThis.document = {
  addEventListener(kind, listener) {
    if (kind === "keydown") listeners.push(listener);
  },
  querySelectorAll: (selector) => body.querySelectorAll(selector),
};
require("./the-return-key.js");

function press(target, key = "Enter", already = false) {
  const event = {
    key,
    target,
    defaultPrevented: already,
    isComposing: false,
    shiftKey: false,
    altKey: false,
    ctrlKey: false,
    metaKey: false,
    preventDefault() {
      this.defaultPrevented = true;
    },
  };
  for (const listener of listeners) listener(event);
  return event.defaultPrevented;
}

function aBox(...controls) {
  const modal = new Element("DIV", ["modal"]);
  const box = new Element("DIV", ["mbox"]);
  const field = new Element("INPUT", ["fld"], { type: "text" });
  const tick = new Element("SPAN", ["box"], { role: "checkbox" });
  const row = new Element("DIV", ["mact"]);
  row.append(...controls);
  box.append(field, tick, row);
  modal.append(box);
  body.append(modal);
  return { modal, box, field, tick };
}

const out = {};
// A box with Cancel and Connect: Return in the field presses Connect.
const cancel = new Element("BUTTON", ["btn", "quiet"]);
const connect = new Element("BUTTON", ["btn"]);
const one = aBox(cancel, connect);
out.fieldPressesTheAffirmative = press(one.field) && connect.clicks === 1 && cancel.clicks === 0;
// A focused Cancel does not swallow it.
out.aFocusedCancelDoesNotSwallowIt = press(cancel) && connect.clicks === 2 && cancel.clicks === 0;
// A tick box keeps its own use of the key.
out.aTickBoxKeepsIt = !press(one.tick) && connect.clicks === 2;
// A field that answered Return itself is left alone.
out.anAnsweredReturnIsLeftAlone = press(one.field, "Enter", true) && connect.clicks === 2;
// Another key is not this listener's.
out.anotherKeyIsNotTaken = !press(one.field, "Escape") && connect.clicks === 2;
one.modal.hidden = true;
// A box whose affirmative is red takes no Return, even on the red control.
const keep = new Element("BUTTON", ["btn", "quiet"]);
const remove = new Element("BUTTON", ["btn", "danger"]);
const red = aBox(keep, remove);
out.aRedAffirmativeTakesNoReturn =
  press(red.field) && press(remove) && remove.clicks === 0 && keep.clicks === 0;
red.modal.hidden = true;
// Of two action rows, the one shown is the one pressed.
const go = new Element("BUTTON", ["btn"]);
go.hidden = true;
const done = new Element("BUTTON", ["btn"]);
const two = aBox(go, done);
out.theShownAffirmativeIsPressed = press(two.field) && done.clicks === 1 && go.clicks === 0;
two.modal.hidden = true;
// With no box open, Return is left to the page.
out.noBoxNoReturn = !press(new Element("INPUT", [], { type: "text" }));
console.log(JSON.stringify(out));
"#;

/// **A value is drawn in the order its bytes run** — `Desktop app and delivery`
/// X9, its paragraph *Drawn in the order its bytes run*, which
/// `web/the-value-drawn.js` keeps on the page. Added at step 10c, dependency
/// wave piece 10a.
///
/// **What the page did before** is what the tests below fail on: a value was
/// one run of text, so `/root/uploads/القوام/فراء.tif` read as
/// `/root/uploads/فراء/القوام.tif` on this Mac's engine — the two Arabic names
/// and the `/` between them laid out as one right-to-left run. **How an engine
/// lays the result out is not something Node can see**, so these hold the
/// shape that gets it right — every segment its own left-to-right isolate,
/// the separators outside them, the whole one isolate — and hold that the text,
/// read back in order, is the value byte for byte. The engine itself was looked
/// at in a throwaway copy (`Audits/Build/Step 10c/Dependency wave piece 10a —
/// what landed.md`).
///
/// **Skipped, and says so, where no Node is installed** — the shape the Return
/// key's harness above uses.
#[cfg(test)]
mod the_order_a_value_runs {
    use std::fs;
    use std::path::Path;

    /// The three values the dependency wave's map names for this piece: two
    /// paths holding Arabic, and a name holding Cyrillic, Chinese and Arabic
    /// that must come out as it was.
    const THE_BATTLE_VALUES: [&str; 3] = [
        "/root/uploads/القوام/فراء.tif",
        "//../القوام/فراء 2 (نسخة).png",
        "сцена льва — 狮子 أسد.blend",
    ];

    /// Runs `harness` under Node in a scratch folder holding the named page
    /// files and the stand-ins, and gives back what it printed — or `None`
    /// where no Node is installed.
    fn ran(tag: &str, pages: &[&str], stand_ins: &[(&str, &str)], harness: &str) -> Option<String> {
        let folder = std::env::temp_dir().join(format!(
            "smoozy-value-order-{tag}-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&folder);
        fs::create_dir_all(&folder).expect("a scratch folder");
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for page in pages {
            fs::copy(web.join(page), folder.join(page)).expect("the page is copied");
        }
        for (name, text) in stand_ins {
            fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        fs::write(folder.join("package.json"), r#"{ "type": "module" }"#).expect("the package note");
        fs::write(folder.join("the-values.json"), serde_json::to_string(&THE_BATTLE_VALUES).unwrap())
            .expect("the values are written");
        fs::write(folder.join("harness.js"), harness).expect("the harness is written");
        let out = std::process::Command::new("node")
            .arg(folder.join("harness.js"))
            .current_dir(&folder)
            .output();
        let _ = fs::remove_dir_all(&folder);
        let Ok(out) = out else {
            eprintln!("SKIPPED: no Node on this computer, so the page code was not run");
            return None;
        };
        assert!(
            out.status.success(),
            "the page code did not run: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        Some(String::from_utf8(out.stdout).expect("the harness prints text").trim().to_string())
    }

    /// What each battle value must come out as: the outer element, then each
    /// piece in order — a segment as `[bdi, ltr, text]`, a separator as its
    /// text alone.
    fn the_shapes_wanted() -> serde_json::Value {
        serde_json::json!([
            {
                "whole": ["BDI", "ltr"],
                "pieces": ["/", ["BDI", "ltr", "root"], "/", ["BDI", "ltr", "uploads"], "/",
                           ["BDI", "ltr", "القوام"], "/", ["BDI", "ltr", "فراء.tif"]],
                "readBack": THE_BATTLE_VALUES[0],
                "sameBytes": true
            },
            {
                "whole": ["BDI", "ltr"],
                "pieces": ["/", "/", ["BDI", "ltr", ".."], "/", ["BDI", "ltr", "القوام"], "/",
                           ["BDI", "ltr", "فراء 2 (نسخة).png"]],
                "readBack": THE_BATTLE_VALUES[1],
                "sameBytes": true
            },
            {
                "whole": ["BDI", "ltr"],
                "pieces": [],
                "readBack": THE_BATTLE_VALUES[2],
                "sameBytes": true
            }
        ])
    }

    /// The stand-in document every harness here draws into: elements with a
    /// tag, a `dir`, children and text, and text nodes. **`textContent` reads
    /// the element's own text and then its children's, in order**, which is
    /// what a person selecting the value off the screen reads.
    const THE_STAND_IN_DOCUMENT: &str = r#"
class TextPiece {
  constructor(text) { this.data = String(text); this.children = []; }
  get textContent() { return this.data; }
}
class Piece {
  constructor(tag) {
    this.tagName = tag.toUpperCase(); this.children = []; this.own = ""; this.dir = "";
    this.className = ""; this.dataset = {}; this.hidden = false; this.type = "";
    const names = new Set();
    this.classList = { add: (...more) => more.forEach((one) => names.add(one)), contains: (one) => names.has(one) };
  }
  set textContent(value) { this.children = []; this.own = String(value); }
  get textContent() { return this.own + this.children.map((one) => one.textContent).join(""); }
  appendChild(child) { this.children.push(child); return child; }
  setAttribute() {}
  addEventListener() {}
}
globalThis.document = {
  createElement: (tag) => new Piece(tag),
  createTextNode: (text) => new TextPiece(text),
};
globalThis.window = { __TAURI__: { core: { invoke: () => Promise.resolve(null) } } };
const fs = await import("node:fs");
const values = JSON.parse(fs.readFileSync("./the-values.json", "utf8"));
function theShapeOf(drawn, value) {
  const pieces = drawn.children.map((one) =>
    one instanceof TextPiece ? one.data : [one.tagName, one.dir, one.textContent]);
  const readBack = drawn.textContent;
  return {
    whole: [drawn.tagName, drawn.dir],
    pieces,
    readBack,
    sameBytes: Buffer.from(readBack, "utf8").equals(Buffer.from(value, "utf8")),
  };
}
"#;

    /// **The one drawing, on the three battle values.** Each path comes out as
    /// its segments in order, every one a left-to-right isolate with the
    /// separators between them; the name comes out as one isolate holding it
    /// whole; and each reads back as the value, byte for byte. **An empty
    /// value leaves its holder empty**, as `textContent = ""` did.
    #[test]
    fn a_path_is_drawn_segment_by_segment_and_reads_back_byte_for_byte() {
        let harness = format!(
            "{THE_STAND_IN_DOCUMENT}\n{}",
            r#"
const { theValueDrawn, theValueIsDrawnInto } = await import("./the-value-drawn.js");
const shapes = values.map((value) => theShapeOf(theValueDrawn(value), value));
const holder = document.createElement("span");
holder.textContent = "before";
theValueIsDrawnInto(holder, "");
const emptyLeavesItEmpty = holder.children.length === 0 && holder.textContent === "";
theValueIsDrawnInto(holder, values[0]);
const oneValueOneIsolate = holder.children.length === 1 && holder.textContent === values[0];
console.log(JSON.stringify({ shapes, emptyLeavesItEmpty, oneValueOneIsolate }));
"#
        );
        let Some(printed) = ran("drawn", &["the-value-drawn.js"], &[], &harness) else {
            return;
        };
        let got: serde_json::Value = serde_json::from_str(&printed).expect("the harness prints one answer");
        assert_eq!(got["shapes"], the_shapes_wanted(), "the drawing of the three battle values");
        assert_eq!(got["emptyLeavesItEmpty"], true, "an empty value left something in its holder");
        assert_eq!(got["oneValueOneIsolate"], true, "a value drawn into a holder is not one isolate");
    }

    /// **The one failure shape draws a value it names the same way** — a path
    /// a failure sentence stands beside is the first case the ruling names
    /// (`web/the-failure-shape.js`). The real page is run, beside the real
    /// drawing, with a stand-in for the event route.
    #[test]
    fn the_failure_shape_draws_the_path_it_names_in_the_order_its_bytes_run() {
        let harness = format!(
            "{THE_STAND_IN_DOCUMENT}\n{}",
            r#"
const { theFailureShape, EXPANDED } = await import("./the-failure-shape.js");
const shapes = values.map((value) => {
  const shape = theFailureShape({ sentence: "", values: [["", value]], controls: [], room: EXPANDED });
  const list = shape.children.find((one) => one.tagName === "DL");
  const said = list.children.find((one) => one.tagName === "DD");
  return said.children.length === 1 ? theShapeOf(said.children[0], value) : { drawnAs: said.textContent };
});
console.log(JSON.stringify({ shapes }));
"#
        );
        let Some(printed) = ran(
            "failure",
            &["the-value-drawn.js", "the-failure-shape.js"],
            &[("event-route.js", "export function whenTold() {}")],
            &harness,
        ) else {
            return;
        };
        let got: serde_json::Value = serde_json::from_str(&printed).expect("the harness prints one answer");
        assert_eq!(got["shapes"], the_shapes_wanted(), "the failure shape's drawing of a path it names");
    }

    // ════════════════════════════════════════════════════════════════════════
    // Step 10c's dependency wave piece 10b: the last three pages
    // ════════════════════════════════════════════════════════════════════════

    /// **The same drawing on the last three pages** — Render Settings, the
    /// dependency check and Transfers — each run for real under Node, beside
    /// the real drawing and the real failure shape, and **each handed the
    /// answer the application itself composes**, with the battle values put
    /// where a person would meet them. Every other page a page imports is a
    /// stand-in that draws nothing.
    mod on_the_last_three_pages {
        use super::{ran, the_shapes_wanted, THE_BATTLE_VALUES};

        use std::path::PathBuf;

        use crate::surface::files::Words as APiecewiseSentence;
        use crate::surface::servers::Words;
        use crate::typography::{Prose, Verbatim};

        /// **A path the way a `.blend` saved on Windows records it** — the
        /// check's *what the file records* is where it is met (10a's account).
        const A_WINDOWS_PATH: &str = "C:\\Users\\artist\\القوام\\فراء.tif";

        /// The three battle values and the Windows path, in that order.
        fn the_values() -> Vec<&'static str> {
            let mut values = THE_BATTLE_VALUES.to_vec();
            values.push(A_WINDOWS_PATH);
            values
        }

        /// The battle values' shapes, and the Windows path's: cut at each `\`.
        fn the_shapes() -> serde_json::Value {
            let mut shapes = the_shapes_wanted();
            shapes.as_array_mut().unwrap().push(serde_json::json!({
                "whole": ["BDI", "ltr"],
                "pieces": [["BDI", "ltr", "C:"], "\\", ["BDI", "ltr", "Users"], "\\",
                           ["BDI", "ltr", "artist"], "\\", ["BDI", "ltr", "القوام"], "\\",
                           ["BDI", "ltr", "فراء.tif"]],
                "readBack": A_WINDOWS_PATH,
                "sameBytes": true
            }));
            shapes
        }

        /// **A stand-in document with enough of the real one for a whole page
        /// to draw into**: parents, classes, attributes, removal and
        /// replacement. `textContent` reads an element's own text and then its
        /// children's, in order — what a person selecting it reads.
        const THE_DOCUMENT: &str = r#"
class TextPiece {
  constructor(text) { this.data = String(text); this.parentNode = null; }
  get textContent() { return this.data; }
  set textContent(value) { this.data = String(value); }
  remove() { if (this.parentNode) this.parentNode.removeChild(this); }
}
class Piece {
  constructor(tag) {
    this.tagName = tag.toUpperCase(); this.children = []; this.own = ""; this.dir = "";
    this.dataset = {}; this.hidden = false; this.type = ""; this.disabled = false;
    this.attributes = {}; this.parentNode = null; this.value = ""; this.open = false;
    const names = new Set();
    this.names = names;
    this.classList = {
      add: (...more) => more.forEach((one) => names.add(one)),
      remove: (...more) => more.forEach((one) => names.delete(one)),
      toggle: (one, on) => { const want = on === undefined ? !names.has(one) : Boolean(on);
        if (want) names.add(one); else names.delete(one); return want; },
      contains: (one) => names.has(one),
    };
  }
  set className(value) { this.names.clear(); String(value).split(/\s+/).filter(Boolean).forEach((one) => this.names.add(one)); }
  get className() { return [...this.names].join(" "); }
  set textContent(value) { this.children.forEach((one) => { one.parentNode = null; }); this.children = []; this.own = String(value); }
  get textContent() { return this.own + this.children.map((one) => one.textContent).join(""); }
  appendChild(child) { if (child.parentNode) child.parentNode.removeChild(child); child.parentNode = this; this.children.push(child); return child; }
  append(...more) { more.forEach((one) => this.appendChild(one)); }
  removeChild(child) { const at = this.children.indexOf(child); if (at >= 0) this.children.splice(at, 1); child.parentNode = null; return child; }
  replaceChildren(...more) { this.textContent = ""; this.append(...more); }
  remove() { if (this.parentNode) this.parentNode.removeChild(this); }
  after(node) { const parent = this.parentNode; if (!parent) return; if (node.parentNode) node.parentNode.removeChild(node);
    node.parentNode = parent; parent.children.splice(parent.children.indexOf(this) + 1, 0, node); }
  contains(node) { for (let at = node; at; at = at.parentNode) if (at === this) return true; return false; }
  get parentElement() { return this.parentNode; }
  get firstChild() { return this.children[0] || null; }
  get childElementCount() { return this.children.filter((one) => one instanceof Piece).length; }
  setAttribute(name, value) { this.attributes[name] = String(value); }
  getAttribute(name) { return name in this.attributes ? this.attributes[name] : null; }
  addEventListener() {}
  focus() { document.activeElement = this; }
  closest() { return null; }
  querySelector() { return null; }
  querySelectorAll() { return []; }
}
globalThis.document = {
  createElement: (tag) => new Piece(tag),
  createTextNode: (text) => new TextPiece(text),
  activeElement: null,
  addEventListener() {},
  querySelector() { return null; },
  body: new Piece("body"),
};
globalThis.CSS = { escape: (text) => text };
const fs = await import("node:fs");
const values = JSON.parse(fs.readFileSync("./the-four-values.json", "utf8"));
const answers = JSON.parse(fs.readFileSync("./the-answers.json", "utf8"));
let answering = 0;
globalThis.window = {
  addEventListener() {},
  __TAURI__: { core: { invoke: (command) => Promise.resolve(
    answers[answering] && command in answers[answering] ? answers[answering][command] : null) } },
};
const settle = () => new Promise((go) => setImmediate(go));
function everyPiece(root, found = []) {
  if (!(root instanceof Piece)) return found;
  found.push(root);
  root.children.forEach((one) => everyPiece(one, found));
  return found;
}
function theShapeOf(drawn, value) {
  const pieces = drawn.children.map((one) =>
    one instanceof TextPiece ? one.data : [one.tagName, one.dir, one.textContent]);
  const readBack = drawn.textContent;
  return {
    whole: [drawn.tagName, drawn.dir],
    pieces,
    readBack,
    sameBytes: Buffer.from(readBack, "utf8").equals(Buffer.from(value, "utf8")),
  };
}
/// The value where a site holds it: the outermost isolate reading as the
/// value, or — where there is none — what the site reads as.
function theValueIn(site, value) {
  if (!site) return { noSite: true };
  const isolate = everyPiece(site).find((one) => one.tagName === "BDI" && one.textContent === value);
  return isolate ? theShapeOf(isolate, value) : { drawnAs: site.textContent };
}
const withClass = (root, name) => everyPiece(root).find((one) => one.classList.contains(name));
const withBox = (root, box) => everyPiece(root).find((one) => one.dataset.box === box);
"#;

        /// The stand-in modules every page here imports and nothing here
        /// looks at.
        fn the_stand_ins(answers: &serde_json::Value) -> Vec<(&'static str, String)> {
            let values = serde_json::to_string(&the_values()).unwrap();
            vec![
                ("the-four-values.json", values),
                ("the-answers.json", answers.to_string()),
                ("event-route.js", "export function whenTold() {}".into()),
                (
                    "a-server.js",
                    "export const RENDER_SETTINGS = 'set'; export const THE_DEPENDENCY_CHECK = 'chk'; \
                     export const SET_UP = 'setupview'; export const FILES = 'files'; export const panels = []; \
                     export function aPanelIsRegistered(panel) { panels.push(panel); } \
                     export function thePanelIsOpenedOver() {} export function theStripReads() {} \
                     export function theCurrentTabIs() {} export function thePanelCollapsesAsItsChevronDoes() {}"
                        .into(),
                ),
                (
                    "files.js",
                    "export function theBrowserIsOpenedAt() {} export function theSavedWithVersionReadFor() { return ''; }"
                        .into(),
                ),
                (
                    "the-rail.js",
                    "export const A_SERVER = 'machineview'; export const TRANSFERS = 'transfersview'; \
                     export const THE_QUEUE = 'queueview'; \
                     export const screens = []; export function aScreenIsRegistered(screen) { screens.push(screen); } \
                     export function theCurrentPlaceIs() {} export function theRowsFigureIs() {}"
                        .into(),
                ),
                (
                    "the-underway-shape.js",
                    "export function theMarkOnARow() { return document.createElement('span'); } \
                     export function theCountdown() { return document.createElement('span'); } \
                     export function theMarkOnAControl() {}"
                        .into(),
                ),
                ("the-machines-own-words.js", "export function theMachinesOwnWordsFor() {}".into()),
                (
                    "the-typed-text.js",
                    "export function theTypingIsTakenFrom() { return null; } export function theTypingIsPutBackInto() {}"
                        .into(),
                ),
                (
                    "settings.js",
                    "export const TRANSFERS_SECTION = 's-tr'; export const SEARCH_FOLDERS_SECTION = 's-fo'; \
                     export function theSettingsSectionIsOpened() {}"
                        .into(),
                ),
                ("the-history.js", "export function theHistoryEntryIsOpened() {}".into()),
                // **The one writer of a moment**, which the render settings
                // page imports for its band (the 38's piece 2).
                ("servers.js", "export function theMomentReads(seconds) { return String(seconds); }".into()),
                // **The other two of the three**, standing in wherever one of
                // them is imported by the page under test.
                (
                    "the-dependency-check.js",
                    "export async function theCheckIsOpenedOn() {} export async function theCheckIsShownOn() {}".into(),
                ),
                (
                    "transfers.js",
                    "export function aTransferIsAskedFor() {} export async function theFormatBoxIsOpenedForAFile() {}".into(),
                ),
            ]
        }

        fn run(tag: &str, page: &str, answers: serde_json::Value, harness: &str, stand_ins_but: &[&str]) -> Option<serde_json::Value> {
            let stand_ins = the_stand_ins(&answers);
            let stand_ins: Vec<(&str, &str)> = stand_ins
                .iter()
                .filter(|(name, _)| !stand_ins_but.contains(name))
                .map(|(name, text)| (*name, text.as_str()))
                .collect();
            let harness = format!("{THE_DOCUMENT}\n{harness}");
            let printed = ran(tag, &["the-value-drawn.js", "the-failure-shape.js", page], &stand_ins, &harness)?;
            Some(serde_json::from_str(&printed).expect("the harness prints one answer"))
        }

        /// The shapes one site came out as, one per value, against the ones
        /// wanted.
        fn each_site_holds_the_value_in_order(got: &serde_json::Value, sites: &[&str], page: &str) {
            for site in sites {
                assert_eq!(
                    got[site],
                    the_shapes(),
                    "{page}: {site} does not read in the order its bytes run"
                );
            }
        }

        fn words_of(value: &str) -> Words {
            Words {
                lead: Prose::new(""),
                values: vec![Verbatim::new(value.to_string())],
                rest: None,
                the_rest_joins: false,
            }
        }

        /// **Render Settings: the file's name in the title, where a job
        /// writes, a colliding pair naming the path it lands at, and a
        /// scene's own name** — each drawn from the panel
        /// `render_settings::the_panel` composes.
        #[test]
        fn render_settings_draws_every_path_and_name_in_the_order_its_bytes_run() {
            use crate::surface::render_settings::{
                the_panel, ABox, AWriteRow, TheOutputWarning, TheWrites, WhatTheRunHolds,
            };
            let answers: Vec<serde_json::Value> = the_values()
                .into_iter()
                .map(|value| {
                    let mut panel = the_panel(&WhatTheRunHolds::default(), "server", "/root/uploads/scene.blend");
                    panel.title.values = vec![Verbatim::new(value.to_string())];
                    panel.the_output_group_is_drawn = true;
                    panel.what_it_will_write = Some(TheWrites {
                        rows: vec![AWriteRow {
                            place: Verbatim::new(value.to_string()),
                            the_top: None,
                            how_many: Verbatim::new("250".to_string()),
                            format: Some(Verbatim::new("PNG".to_string())),
                        }],
                        total: words_of("250"),
                        by_format: Vec::new(),
                    });
                    // **The line above the list names a folder on the
                    // server** — the Creator's second notes, piece 2.
                    panel.the_folder_line = Some(vec![words_of(value)]);
                    panel.the_output_warnings = vec![TheOutputWarning {
                        which: Verbatim::new("collide".to_string()),
                        lead: Prose::new(""),
                        rest: Prose::new(""),
                        pairs: vec![vec![vec![words_of(value)], vec![words_of("Scene")]]],
                    }];
                    panel.the_scene_group_is_drawn = true;
                    panel.the_scenes = vec![ABox { name: Verbatim::new(value.to_string()), ticked: false, tag: None }];
                    serde_json::json!({ "the_render_settings_panel": panel })
                })
                .collect();
            let harness = r#"
const { panels } = await import("./a-server.js");
const { theSettingsAreOpenedOn } = await import("./render-settings.js");
const got = { title: [], writes: [], pair: [], scene: [], folderLine: [] };
for (let at = 0; at < values.length; at += 1) {
  answering = at;
  const server = `server-${at}`;
  await theSettingsAreOpenedOn(server, [1], "");
  const head = document.createElement("div");
  const body = document.createElement("div");
  panels[0].itIsDrawn({ head, body }, server);
  await settle(); await settle();
  const value = values[at];
  got.title.push(theValueIn(everyPiece(head).find((one) => one.tagName === "H2"), value));
  got.writes.push(theValueIn(withClass(body, "writes"), value));
  got.pair.push(theValueIn(withClass(body, "owpair"), value));
  got.scene.push(theValueIn(withBox(body, `scene-${value}`), value));
  got.folderLine.push(theValueIn(withBox(body, "the-jobs-folder"), value));
}
console.log(JSON.stringify(got));
"#;
            let Some(got) = run(
                "render-settings",
                "render-settings.js",
                serde_json::Value::Array(answers),
                harness,
                &["render-settings.js"],
            ) else {
                return;
            };
            each_site_holds_the_value_in_order(
                &got,
                &["title", "writes", "pair", "scene", "folderLine"],
                "web/render-settings.js",
            );
        }

        /// **The dependency check: a row's name, its one location, and the
        /// location a replace would write over** — drawn from the panel
        /// `the_dependency_check::the_panel` composes, with a row the way a
        /// reading lands one. *(Until 2026-10-18, the dependency check's fresh
        /// look, the row's two paths: what the file records and what Smoozy
        /// found.)*
        #[test]
        fn the_dependency_check_draws_every_path_in_the_order_its_bytes_run() {
            use crate::surface::the_dependency_check::{
                the_panel, AGroup, ARow, TheOverwrite, TheTable, WhatTheRunHolds,
            };
            let answers: Vec<serde_json::Value> = the_values()
                .into_iter()
                .map(|value| {
                    let mut panel = the_panel(&WhatTheRunHolds::default(), "server", "/root/uploads/scene.blend", None);
                    let row = ARow {
                        at: Verbatim::new("0".to_string()),
                        name: Verbatim::new(value.to_string()),
                        a_linked_blend: false,
                        kind: APiecewiseSentence::plain(""),
                        location: APiecewiseSentence::plain("").and(value),
                        the_file_says: Verbatim::new(value.to_string()),
                        notes: Vec::new(),
                        size: Some(Verbatim::new("2 MB".to_string())),
                        action: None,
                        more: Prose::new(""),
                        more_items: Vec::new(),
                        chosen: false,
                        choosable: true,
                    };
                    panel.the_table = Some(TheTable {
                        groups: vec![AGroup {
                            which: Verbatim::new("found".to_string()),
                            heading: Prose::new(""),
                            colour: Verbatim::new(String::new()),
                            how_many: APiecewiseSentence::plain("").and("1"),
                            folded: false,
                            rows: vec![row],
                        }],
                        adds_a_row_with: Verbatim::new("meta".to_string()),
                    });
                    panel.the_overwrite = Some(TheOverwrite {
                        title: Prose::new(""),
                        what_was_read: Vec::new(),
                        location: Verbatim::new(value.to_string()),
                        live_jobs: None,
                        the_folder: None,
                        stop: Prose::new(""),
                        replace: Prose::new(""),
                    });
                    serde_json::json!({ "the_dependency_check_panel": panel })
                })
                .collect();
            let harness = r#"
const { panels } = await import("./a-server.js");
await import("./the-dependency-check.js");
const got = { name: [], location: [], replacing: [] };
for (let at = 0; at < values.length; at += 1) {
  answering = at;
  const sheet = document.createElement("div");
  const head = sheet.appendChild(document.createElement("div"));
  const body = sheet.appendChild(document.createElement("div"));
  panels[0].itIsDrawn({ head, body }, `server-${at}`);
  await settle(); await settle();
  const value = values[at];
  got.name.push(theValueIn(withClass(body, "nm"), value));
  got.location.push(theValueIn(withClass(body, "loc"), value));
  got.replacing.push(theValueIn(withClass(sheet, "clist"), value));
}
console.log(JSON.stringify(got));
"#;
            let Some(got) = run(
                "the-dependency-check",
                "the-dependency-check.js",
                serde_json::Value::Array(answers),
                harness,
                &["the-dependency-check.js"],
            ) else {
                return;
            };
            each_site_holds_the_value_in_order(&got, &["name", "location", "replacing"], "web/the-dependency-check.js");
        }

        /// **Transfers: a file's row, the folder an upload goes into, and the
        /// format box's folders** — drawn from the screen
        /// `transfers::the_screen_from` composes and the box
        /// `transfers::the_format_box_from` composes.
        #[test]
        fn transfers_draws_every_path_in_the_order_its_bytes_run() {
            use crate::platform::downloads_folder::TheProposal;
            use crate::surface::transfers::{
                the_format_box_from, the_screen_from, AFileKnown, AFormatKnown, ARequestKnown, AServerKnown,
                TheFormatAnswer, TheKind, WhatHappenedToTheFile, WhatIsKnown, WhatItIsOf, WhatTheBoxKnows,
                WhereTheRequestStands,
            };
            use smoozy_server_protocol::transfers::Report;
            let answers: Vec<serde_json::Value> = the_values()
                .into_iter()
                .map(|value| {
                    let request = ARequestKnown {
                        start: None,
                        id: "one".into(),
                        kind: TheKind::Upload,
                        server: "row".into(),
                        of: WhatItIsOf::AFolder { folder: value.to_string() },
                        files_done: 0,
                        files_in_the_whole: 1,
                        bytes: Some((10, 100)),
                        seconds_left: None,
                        destination: PathBuf::from(value),
                        narrowed_to: Vec::new(),
                        reconnecting: None,
                        waits_for_the_connection: false,
                        its_period_ended_it: false,
                        trouble: None,
                        left_alone: 0,
                        cannot_land: 0,
                        may_be_incomplete: 0,
                        split_between: None,
                        the_job_ended: false,
                        the_job_ended_short: false,
                        the_arrangement_is_done: false,
                        the_whole_was_read: false,
                        nothing_is_moving: false,
                        stands: WhereTheRequestStands::Running,
                        files: vec![AFileKnown {
                            path: value.to_string(),
                            record_ending: None,
                            report: Report {
                                bytes_staged: 10,
                                bytes_expected: 100,
                                started_over: None,
                                verifying_a_prefix_of: None,
                            },
                            happened: WhatHappenedToTheFile::Moving,
                        }],
                    };
                    let known = WhatIsKnown {
                        servers: vec![AServerKnown { row: "row".into(), name: "RunPod 4090".into(), badge: None }],
                        requests: vec![request],
                        ..WhatIsKnown::default()
                    };
                    let a_format = |name: &str| AFormatKnown {
                        name: name.into(),
                        where_in_the_job: value.to_string(),
                        at_the_top: false,
                        files: 3,
                        chosen: true,
                    };
                    let the_box = the_format_box_from(
                        &WhatTheBoxKnows {
                            formats: Some(vec![a_format("EXR"), a_format("PNG")]),
                            files_in_the_job: 6,
                            the_standing_folder: Some(value.as_bytes().to_vec()),
                            the_formats_chosen: None,
                            the_folder_chosen_for_this_job: None,
                        },
                        &TheProposal::TheSystemNamesNoFolder(String::new()),
                    );
                    serde_json::json!({
                        "the_transfers_screen": the_screen_from(&known, None),
                        "the_format_box": TheFormatAnswer::TheBox(the_box),
                    })
                })
                .collect();
            let harness = r#"
const { screens } = await import("./the-rail.js");
const { theFormatBoxIsOpenedFor } = await import("./transfers.js");
const got = { file: [], request: [], formatsFolder: [], whereItGoes: [] };
for (let at = 0; at < values.length; at += 1) {
  answering = at;
  const host = document.createElement("div");
  screens[0].itIsDrawn(host);
  screens[0].itIsCurrentNow();
  await settle(); await settle();
  const value = values[at];
  got.file.push(theValueIn(withClass(host, "tn"), value));
  got.request.push(theValueIn(withClass(host, "rqn"), value));
  document.body.replaceChildren();
  await theFormatBoxIsOpenedFor("job");
  await settle();
  got.formatsFolder.push(theValueIn(withClass(document.body, "fmw"), value));
  got.whereItGoes.push(theValueIn(withClass(withClass(document.body, "transfers-folders"), "fmn"), value));
}
console.log(JSON.stringify(got));
"#;
            let Some(got) = run(
                "transfers",
                "transfers.js",
                serde_json::Value::Array(answers),
                harness,
                &["transfers.js"],
            ) else {
                return;
            };
            each_site_holds_the_value_in_order(
                &got,
                &["file", "request", "formatsFolder", "whereItGoes"],
                "web/transfers.js",
            );
        }
    }
}

/// **Where the user had scrolled is not lost to a redraw either** —
/// `Design/Whole app § The window`: *"a list drawn again with the same things
/// in it keeps its place, and a list that gains or loses rows keeps the row
/// the user was looking at in view."* Step 10c's eleventh pass (*Smaller* 6): a
/// Files list receiving an upload jumped to its top at every file, because
/// every upload that ended read the browsed folder again (`File Browser` L12)
/// and the rows went while it was read. Added 2026-09-30, step 10c's
/// eleventh-pass fix piece 7.
///
/// **This runs `web/files.js` under Node** beside a stand-in document that
/// lays its list out — rows of one height, a view of ten, and an offset that
/// falls back inside what is there whenever the list is emptied, which is what
/// the Mac's own engine did in the page harness. The answers are the ones the
/// application composes. **Skipped, and says so, where no Node is installed.**
#[cfg(test)]
mod where_the_user_had_scrolled {
    use std::fs;
    use std::path::Path;

    use smoozy_file_browser::listing::{ApparentSize, EntryName, Kind, LastChanged, ListedEntry};
    use smoozy_servers::condition::ObservedValue;

    use crate::surface::files::{a_folder_being_read, the_tab, the_tab_after_the_reading, WhatTheReadingFound};

    fn a_frame(name: &str) -> ListedEntry {
        ListedEntry {
            name: EntryName::from_the_machine(name.as_bytes().to_vec()),
            size: ApparentSize::of(665_000),
            last_changed: Some(LastChanged::at_epoch_seconds(1_754_483_000)),
            kind: Kind::File,
        }
    }

    #[test]
    fn a_folder_read_again_keeps_the_row_the_user_was_looking_at() {
        let server = "a".repeat(32);
        let folder = b"/home/ubuntu/as-they-come";
        let frames = |more: &[&str]| {
            let mut entries: Vec<ListedEntry> = (0..120).map(|at| a_frame(&format!("frame_{at:04}.png"))).collect();
            entries.extend(more.iter().map(|name| a_frame(name)));
            entries
        };
        let the_rows = |more: &[&str]| {
            the_tab_after_the_reading(
                Default::default(),
                &server,
                folder,
                None,
                WhatTheReadingFound::TheFolder { entries: frames(more), free: None },
                Some(ObservedValue::Connected),
                false,
            )
        };
        let mut being_read =
            the_tab(&server, folder, None, a_folder_being_read(folder), Some(ObservedValue::Connected), None, false);
        being_read.a_reading_follows = true;
        let answers = serde_json::json!({
            "being_read": being_read,
            // The first reading, then the same folder with the same rows, then
            // a new file landing above where the user was.
            "readings": [the_rows(&[]), the_rows(&[]), the_rows(&["a_new_upload.png"])],
        });

        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        let scratch = std::env::temp_dir().join(format!("smoozy-scrolled-harness-{}", std::process::id()));
        let _ = fs::remove_dir_all(&scratch);
        fs::create_dir_all(&scratch).expect("a scratch folder");
        fs::copy(web.join("files.js"), scratch.join("files.js")).expect("the page module is copied");
        for (name, text) in THE_STAND_INS {
            fs::write(scratch.join(name), text).expect("a stand-in is written");
        }
        fs::write(scratch.join("package.json"), r#"{ "type": "module" }"#).expect("the package note");
        fs::write(scratch.join("answers.json"), answers.to_string()).expect("the answers are written");
        fs::write(scratch.join("harness.js"), THE_HARNESS).expect("the harness is written");
        let ran = std::process::Command::new("node")
            .arg(scratch.join("harness.js"))
            .current_dir(&scratch)
            .output();
        let _ = fs::remove_dir_all(&scratch);
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the Files page code was not run");
            return;
        };
        assert!(ran.status.success(), "the page code did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value =
            serde_json::from_str(String::from_utf8_lossy(&ran.stdout).trim()).expect("the harness prints one answer");
        let wanted = serde_json::json!({
            "lookingAt": "frame_0040.png",
            "whileItIsRead": { "lookingAt": "frame_0040.png", "covered": true },
            "theSameRows": { "lookingAt": "frame_0040.png", "from": 0 },
            "aRowAbove": { "lookingAt": "frame_0040.png", "from": 0 },
            "anotherFolderStartsAtItsTop": true
        });
        assert_eq!(got, wanted, "the Files list lost where the user had scrolled");
    }

    /// The modules `web/files.js` imports, standing in; none is looked at.
    const THE_STAND_INS: [(&str, &str); 7] = [
        (
            "event-route.js",
            "export function whenTold(kind, handler) { (globalThis.told[kind] ||= []).push(handler); }",
        ),
        (
            "a-server.js",
            "export const FILES = 'files'; export function aTabIsRegistered(tab) { globalThis.tabs.push(tab); } \
             export function aPanelIsOpenOver() { return false; }",
        ),
        ("the-dependency-check.js", "export async function theCheckIsOpenedOn() {}"),
        ("transfers.js", "export function aTransferIsAskedFor() {} export function theTransfersAreShown() {}"),
        ("render-settings.js", "export async function theSettingsAreOpenedOn() {}"),
        (
            "the-underway-shape.js",
            "export function theMarkOverAPane() { const cover = document.createElement('div'); \
             const turning = document.createElement('i'); turning.className = 'spin'; cover.appendChild(turning); return cover; }",
        ),
        (
            "the-value-drawn.js",
            "export function theValueIsDrawnInto(node, value) { node.textContent = value; } \
             export function theValueDrawn(value) { const node = document.createElement('span'); node.textContent = value; return node; }",
        ),
    ];

    /// **A stand-in document that lays the list out.** Every row is 24 high
    /// and the list shows ten; the cover sits over the list and takes no
    /// height. **Emptying the list lets its offset fall back inside what is
    /// left**, as the engine does when it lays the list out again.
    const THE_HARNESS: &str = r#"
const ROW = 24, VIEW = 240;
class TextPiece {
  constructor(text) { this.data = String(text); this.parentNode = null; }
  get textContent() { return this.data; }
  remove() { if (this.parentNode) this.parentNode.removeChild(this); }
}
class Piece {
  constructor(tag) {
    this.tagName = tag.toUpperCase(); this.children = []; this.own = ""; this.parentNode = null;
    this.dataset = {}; this.attributes = {}; this.hidden = false; this.type = ""; this.title = ""; this.top = 0;
    const names = new Set(); this.names = names;
    this.classList = {
      add: (...more) => more.forEach((one) => names.add(one)),
      remove: (...more) => more.forEach((one) => names.delete(one)),
      toggle: (one, on) => { const want = on === undefined ? !names.has(one) : Boolean(on);
        if (want) names.add(one); else names.delete(one); return want; },
      contains: (one) => names.has(one),
    };
  }
  set className(value) { this.names.clear(); String(value).split(/\s+/).filter(Boolean).forEach((one) => this.names.add(one)); }
  get className() { return [...this.names].join(" "); }
  set textContent(value) {
    this.children.forEach((one) => { one.parentNode = null; }); this.children = []; this.own = String(value);
    this.scrollTop = this.scrollTop;
  }
  get textContent() { return this.own + this.children.map((one) => one.textContent).join(""); }
  get firstChild() { return this.children[0] || null; }
  get offsetHeight() { return this.names.has("fwait") ? 0 : ROW; }
  get offsetTop() {
    if (!this.parentNode) return 0;
    let at = 0;
    for (const one of this.parentNode.children) { if (one === this) return at; if (one instanceof Piece) at += one.offsetHeight; }
    return at;
  }
  get clientHeight() { return this.names.has("flist") ? VIEW : 0; }
  get scrollHeight() { return this.children.reduce((sum, one) => sum + (one instanceof Piece ? one.offsetHeight : 0), 0); }
  get scrollTop() { return this.top; }
  set scrollTop(value) { this.top = Math.max(0, Math.min(Number(value) || 0, Math.max(0, this.scrollHeight - this.clientHeight))); }
  appendChild(child) { if (child.parentNode) child.parentNode.removeChild(child); child.parentNode = this; this.children.push(child); return child; }
  insertBefore(child, before) { if (child.parentNode) child.parentNode.removeChild(child); child.parentNode = this;
    const at = this.children.indexOf(before); this.children.splice(at < 0 ? this.children.length : at, 0, child); return child; }
  removeChild(child) { const at = this.children.indexOf(child); if (at >= 0) this.children.splice(at, 1); child.parentNode = null; return child; }
  replaceWith(other) { const parent = this.parentNode; if (!parent) return; const at = parent.children.indexOf(this);
    if (other.parentNode) other.parentNode.removeChild(other); parent.children.splice(at, 1, other); other.parentNode = parent; this.parentNode = null; }
  remove() { if (this.parentNode) this.parentNode.removeChild(this); }
  setAttribute(name, value) { this.attributes[name] = String(value); }
  getAttribute(name) { return name in this.attributes ? this.attributes[name] : null; }
  addEventListener() {}
  focus() {}
  contains(node) { for (let at = node; at; at = at.parentNode) if (at === this) return true; return false; }
  everyPiece(found = []) { for (const one of this.children) if (one instanceof Piece) { found.push(one); one.everyPiece(found); } return found; }
  querySelector(selector) { return this.everyPiece().find((one) => selector.startsWith(".") && one.names.has(selector.slice(1))) || null; }
  querySelectorAll(selector) { return this.everyPiece().filter((one) => selector.startsWith(".") && one.names.has(selector.slice(1))); }
}
globalThis.told = {};
globalThis.tabs = [];
globalThis.document = {
  createElement: (tag) => new Piece(tag),
  createElementNS: (space, tag) => new Piece(tag),
  createTextNode: (text) => new TextPiece(text),
  addEventListener() {},
  activeElement: null,
  body: new Piece("body"),
};
const fs = await import("node:fs");
const answers = JSON.parse(fs.readFileSync("./answers.json", "utf8"));
let readings = 0;
let theReadingLands = null;
globalThis.window = {
  addEventListener() {},
  __TAURI__: { core: { invoke: (command) => {
    if (command === "the_files_tab") return Promise.resolve(answers.being_read);
    if (command === "the_folder_is_read") return new Promise((go) => { theReadingLands = () => go(answers.readings[readings++]); });
    if (command === "what_the_upload_came_to") return Promise.resolve({ pieces: [{ said: "Uploaded", value: null }] });
    return Promise.resolve(null);
  } } },
};
const settle = async () => { for (let at = 0; at < 4; at += 1) await new Promise((go) => setImmediate(go)); };
await import("./files.js");
const host = new Piece("div");
host.className = "files";
document.body.appendChild(host);
const tab = globalThis.tabs[0];
tab.itIsDrawn(host);
tab.itIsShowing("a".repeat(32));
await settle();
theReadingLands(); await settle();
const theList = () => host.querySelector(".flist");
/// The row at the top of the view, and how far into the view it starts.
const where = () => {
  const list = theList();
  const row = list.children.find((one) => one.tagName === "BUTTON" && one.offsetTop + one.offsetHeight > list.scrollTop);
  return { lookingAt: row ? row.querySelector(".fname").textContent : null, from: row ? row.offsetTop - list.scrollTop : null };
};
const anUploadEnded = async () => {
  for (const handler of globalThis.told["files:the-answer"] || []) handler({ said: { what: "the-upload-ended", server: "a".repeat(32), request: "r" } });
  await settle();
};
// The user scrolls to the row forty-one rows down (the header is the first).
theList().scrollTop = ROW * 41;
const out = { lookingAt: where().lookingAt };
await anUploadEnded();
out.whileItIsRead = { lookingAt: where().lookingAt, covered: Boolean(theList().querySelector(".fwait")) };
theReadingLands(); await settle();
out.theSameRows = where();
await anUploadEnded();
theReadingLands(); await settle();
out.aRowAbove = where();
// Another folder is opened by its row's own click in the application; here,
// asked for by the crumbs' way — a folder drawn for the first time starts at
// its top.
answers.being_read.folder.bytes = [47];
await anUploadEnded();
out.anotherFolderStartsAtItsTop = theList().scrollTop === 0;
console.log(JSON.stringify(out));
"#;
}

/// **A redraw keeps where the user had scrolled, on the three pages that draw
/// themselves anew on every answer** — `Design/Whole app § The window`: *"a
/// list drawn again with the same things in it keeps its place"*; and **Render
/// stays where it is when it is pressed**, `§ The window` again: *"Nothing
/// already drawn moves when something appears or disappears … A line that can
/// go keeps its space when it goes, or sits below the controls."* The
/// Creator's second notes on the test copy, notes 7, 8 and 9, piece 5: the
/// cross handing a View Layer back threw Render Settings to its top, Render
/// jumped when pressed, and a job's entry in Queue went to its top at every
/// reading. Added 2026-10-04.
///
/// **This runs `web/render-settings.js`, `web/the-queue.js`,
/// `web/the-history.js` and `web/the-scrolled-place.js` under Node** beside a
/// stand-in document that lays out the parts that scroll, tells the page a
/// scroll at the next frame, tells `MutationObserver`s what a drawing changed
/// once it has run, and lays out a part only where nothing it sits in is
/// hidden — what the page harness saw the engine do. Render Settings is drawn
/// from the panel the application composes. **Skipped, and says so, where no
/// Node is installed.**
#[cfg(test)]
mod a_redraw_keeps_its_place {
    use std::fs;
    use std::path::Path;

    use crate::surface::render_settings::tests::{a_layer, a_run_that_read, a_run_with_a_reading, a_scene, a_server, a_version};
    use crate::surface::render_settings::the_panel;

    /// The four page files run as they are; everything they import stands in.
    const THE_PAGES: [&str; 4] = ["render-settings.js", "the-queue.js", "the-history.js", "the-scrolled-place.js"];

    #[test]
    fn a_redraw_keeps_the_place_and_render_keeps_its_own() {
        // One scene of nine View Layers, read; the page as it stands after
        // *Read the file* landed.
        let mut scene = a_scene("Kitchen");
        scene.view_layers = (0..9).map(|at| a_layer(&format!("Layer {at}"), at % 3 != 0, at == 2)).collect();
        let (mut held, file) = a_run_that_read(vec![scene], a_version(5, 1, 0));
        held.version_lists = a_run_with_a_reading().version_lists;
        let _ = crate::surface::render_settings::the_answer_is_taken(
            &mut held, &a_server(), file, crate::surface::render_settings::WhichSetting::DownloadAsTheyRender, "on",
        );
        let answers = serde_json::json!({ "panel": the_panel(&held, &a_server(), file) });

        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        let scratch = std::env::temp_dir().join(format!("smoozy-a-redraw-keeps-its-place-{}", std::process::id()));
        let _ = fs::remove_dir_all(&scratch);
        fs::create_dir_all(&scratch).expect("a scratch folder");
        for page in THE_PAGES {
            fs::copy(web.join(page), scratch.join(page)).expect("the page is copied");
        }
        for (name, text) in THE_STAND_INS {
            fs::write(scratch.join(name), text).expect("a stand-in is written");
        }
        fs::write(scratch.join("package.json"), r#"{ "type": "module" }"#).expect("the package note");
        fs::write(scratch.join("answers.json"), answers.to_string()).expect("the answers are written");
        fs::write(scratch.join("document.mjs"), THE_DOCUMENT).expect("the document is written");
        fs::write(scratch.join("harness.mjs"), THE_HARNESS).expect("the harness is written");
        let ran = std::process::Command::new("node")
            .arg(scratch.join("harness.mjs"))
            .current_dir(&scratch)
            .output();
        let _ = fs::remove_dir_all(&scratch);
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the pages' redraws were not run");
            return;
        };
        assert!(ran.status.success(), "the page code did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value =
            serde_json::from_str(String::from_utf8_lossy(&ran.stdout).trim()).expect("the harness prints one answer");
        // **The foot holds the same parts in the same order, drawn empty and
        // drawn full**: what the submission says goes onto the foot's second
        // line, under Render, and nothing joins Render's own place. **Render
        // is drawn before the second line**, so the keyboard reaches Render
        // before *Show in Queue* — piece 5's owed tidy, done by the Creator's
        // second notes, piece 4, and kept by the two-line foot of their tenth
        // notes, note 8. The third line is there throughout, because this
        // answer names no folder for the frames, and it does not change.
        let the_foot = serde_json::json!(["field", "so sfl1", "btn", "so sfl2", "sfl3"]);
        let wanted = serde_json::json!({
            // Note 7.
            "theCrossHandedBack": { "at": 400, "drawnAnew": true, "theCrossIsGone": true },
            // Note 8.
            "drawnEmpty": { "parts": the_foot, "says": "" },
            "submitting": { "parts": the_foot, "says": "Submitting." },
            "submitted": { "parts": the_foot, "says": "\u{2713}Submitted to Vast 4090. Show in Queue", "atTheColumns": 400 },
            "refused": { "parts": the_foot, "says": "Not submitted. Type the frames or tick a scene." },
            // Drawn while the panel is collapsed, and put back when it is up.
            "collapsed": { "whileDown": 0, "backUp": 250 },
            // A scroll told only after its part was drawn again is not the place.
            "aScrollToldAfterItsPartWent": 250,
            // Another file, and the first again, start at their top.
            "anotherFile": 0,
            "theFirstFileAgain": 0,
            // Note 9.
            "theQueueReadAgain": { "entry": 400, "list": 600 },
            "anotherJob": { "entry": 0, "list": 600 },
            "theFirstJobAgain": { "entry": 0 },
            "theSameJobOnBothScreens": { "queue": 400, "history": 100, "historysList": 500 },
            "anotherHistoryEntry": { "entry": 0, "list": 500 },
            "theQueueAway": { "whileAway": 0, "shownAgain": 400 }
        });
        assert_eq!(got, wanted, "a redraw lost where the user had scrolled, or moved Render");
    }

    /// **What the submission says sits under Render, on the foot's second
    /// line, and takes no line of its own** — `Design/Render Settings §
    /// Submitting`, from the Creator's tenth notes, note 8: the submitted line
    /// sits under Render, right-aligned in Render's column. **Render does not
    /// move when it appears**, which is what the Creator's second notes, note
    /// 8, asked (Render jumped when pressed): the second line is always drawn,
    /// holding *Render one frame first*, and the line under Render shares it.
    /// Until 2026-10-17 this held a line of its own across the whole foot,
    /// kept one text-line tall drawn empty (`.sfoot > .rs-line`); the two-line
    /// foot replaces it. What Node cannot see — where the line lands — is
    /// measured in a real browser by
    /// `the_foot_is_two_lines_aligned_to_the_top_as_drawn` in
    /// `src/surface/render_settings.rs`.
    #[test]
    fn the_line_under_render_shares_the_second_line_and_moves_nothing() {
        let read = |name: &str| {
            fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("web").join(name))
                .expect("the stylesheet is readable")
        };
        let rule = |sheet: &str, selector: &str| -> String {
            let at = sheet
                .find(&format!("\n{selector} {{"))
                .unwrap_or_else(|| panic!("no rule for {selector}"));
            sheet[at..].split('}').next().unwrap_or_default().to_string()
        };
        let page = read("render-settings.css");
        assert!(!page.contains(".rs-line"), "the old line across the foot is still styled");
        let answer = rule(&page, ".sfoot .sfl2 > [data-box=\"submit-answer\"]");
        for declaration in ["margin-left: auto;", "padding-left: 0;"] {
            assert!(answer.contains(declaration), "what the submission answered lost `{declaration}`: {answer}");
        }
        let drawing = read("the-drawing.css");
        let sent = rule(&drawing, ".rs-sent");
        for declaration in ["margin-left: auto;", "white-space: nowrap;"] {
            assert!(sent.contains(declaration), "the submitted line lost `{declaration}`: {sent}");
        }
        // The second line runs to Render's right edge, under Render.
        assert!(
            rule(&drawing, ".sfoot").contains(r#"grid-template-areas: "nlabel nfield l1 go" ". nhint l2 l2" ". . l3 l3";"#),
            "the foot's grid no longer runs line 2 under Render"
        );
    }

    /// What the pages import, standing in; none is looked at.
    const THE_STAND_INS: [(&str, &str); 13] = [
        ("a-server.js", r#"export const RENDER_SETTINGS = "set"; export const SET_UP = "setupview"; export const FILES = "files"; export const panels = []; export function aPanelIsRegistered(panel) { panels.push(panel); } export function thePanelIsOpenedOver() {} export function theStripReads() {} export function theCurrentTabIs() {} export function thePanelCollapsesAsItsChevronDoes() {}"#),
        ("files.js", r#"export function theBrowserIsOpenedAt() {} export function theSavedWithVersionReadFor() { return ""; }"#),
        ("event-route.js", r#"export function whenTold(kind, handler) { (globalThis.told[kind] ||= []).push(handler); }"#),
        ("panes.js", r#"export function theSeamsAreLive() {}"#),
        ("servers.js", r#"export function theMomentReads(seconds) { return String(seconds); }"#),
        ("the-dependency-check.js", r#"export async function theCheckIsOpenedOn() {} export async function theCheckIsShownOn() {}"#),
        ("the-failure-shape.js", r#"export const EXPANDED = "e"; export function theControlsFrom() { return []; } export function theFailureShape() { return document.createElement("div"); }"#),
        ("the-machines-own-words.js", r#"export function theMachinesOwnWordsFor() {} export function theNewestWordsStayInView() {} export function theWordsPanel() { const panel = document.createElement("div"); panel.hidden = true; return panel; }"#),
        ("the-rail.js", r#"export const THE_QUEUE = "queueview"; export const THE_HISTORY = "historyview"; export function aScreenIsRegistered(screen) { globalThis.screens[screen.place] = screen; } export function theRowsFigureIs() {} export function theCurrentPlace() { return THE_HISTORY; } export function theCurrentPlaceIs() {}"#),
        ("the-typed-text.js", r#"export function theTypingIsTakenFrom() { return null; } export function theTypingIsPutBackInto() {}"#),
        ("the-underway-shape.js", r#"export function theMarkOnAControl(control, label) { control.textContent = label; return control; } export function theMarkOnARow() { return document.createElement("span"); }"#),
        ("the-value-drawn.js", r#"export function theValueIsDrawnInto(node, value) { node.textContent = value; } export function theValueDrawn(value) { const node = document.createElement("span"); node.textContent = value; return node; }"#),
        ("transfers.js", r#"export function aTransferIsAskedFor() {} export async function theFormatBoxIsOpenedFor() {} export async function theFormatBoxIsOpenedForAFile() {}"#),
    ];

    /// **A stand-in document that lays out the parts that scroll.**
    const THE_DOCUMENT: &str = r#"// **A stand-in document that lays out the parts that scroll** — the least of
// one the three pages and `the-scrolled-place.js` need. A part is laid out when
// it is in the document and nothing it sits in is hidden. A part with anything
// in it scrolls 700 pixels (1,000 high in a view of 300); an emptied part falls
// back to its top, as the engine's does. A scroll is told to the page at the
// next frame, as the engine tells it; so is a part being laid out
// (`ResizeObserver`). What the page draws is told to `MutationObserver`s once
// its drawing has run, before anything else.
const observers = new Set();
const sizeWatchers = new Set();
let records = [];
let told = false;
const scrolledSinceTheFrame = new Set();
const documentListeners = [];

function aChange(addedNodes, removedNodes) {
  if (!observers.size) return;
  records.push({ addedNodes, removedNodes });
  if (told) return;
  told = true;
  queueMicrotask(() => {
    told = false;
    const these = records;
    records = [];
    for (const observer of observers) observer.callback(these, observer);
  });
}

class TextPiece {
  constructor(text) { this.data = String(text); this.parentNode = null; this.nodeType = 3; }
  get textContent() { return this.data; }
  remove() { if (this.parentNode) this.parentNode.removeChild(this); }
  get isConnected() { return Boolean(this.parentNode && this.parentNode.isConnected); }
}

function aTest(selector) {
  const ors = selector.split(",").map((one) => one.trim());
  const tests = ors.map((one) => {
    const found = /^([a-zA-Z0-9]*)((?:\.[\w-]+)*)(?:\[([\w-]+)(?:=(?:"([^"]*)"|([^\]"]*)))?\])?$/.exec(one);
    if (!found) throw new Error("a selector the stand-in does not read: " + one);
    const [, tag, classes, attribute, quoted, bare] = found;
    const names = classes ? classes.split(".").filter(Boolean) : [];
    const value = quoted !== undefined ? quoted : bare;
    return (element) => {
      if (!(element instanceof Piece)) return false;
      if (tag && element.tagName !== tag.toUpperCase()) return false;
      if (!names.every((name) => element.classList.contains(name))) return false;
      if (!attribute) return true;
      const held = element.getAttribute(attribute);
      if (held === null) return false;
      return value === undefined || held === value;
    };
  });
  return (element) => tests.some((test) => test(element));
}

const camel = (name) => name.replace(/-([a-z])/g, (_, letter) => letter.toUpperCase());

class Piece {
  constructor(tag) {
    this.tagName = String(tag).toUpperCase();
    this.nodeType = 1;
    this.children = [];
    this.own = "";
    this.parentNode = null;
    this.dataset = {};
    this.attributes = {};
    this.style = {};
    this.listeners = {};
    this.hidden = false;
    this.value = "";
    this.top = 0;
    const names = new Set();
    this.names = names;
    this.classList = {
      add: (...more) => more.forEach((one) => names.add(one)),
      remove: (...more) => more.forEach((one) => names.delete(one)),
      toggle: (one, on) => { const want = on === undefined ? !names.has(one) : Boolean(on); if (want) names.add(one); else names.delete(one); return want; },
      contains: (one) => names.has(one),
    };
  }
  set className(value) { this.names.clear(); String(value).split(/\s+/).filter(Boolean).forEach((one) => this.names.add(one)); }
  get className() { return [...this.names].join(" "); }
  get isConnected() { let at = this; while (at.parentNode) at = at.parentNode; return at === document.documentElement; }
  get laidOut() { if (!this.isConnected) return false; for (let at = this; at; at = at.parentNode) if (at.hidden) return false; return true; }
  getClientRects() { return this.laidOut ? [{}] : []; }
  get clientHeight() { return this.laidOut ? 300 : 0; }
  get scrollHeight() { return this.laidOut ? (this.children.length ? 1000 : 300) : 0; }
  get scrollTop() { return this.laidOut ? this.top : 0; }
  set scrollTop(value) {
    if (!this.laidOut) return;
    const top = Math.max(0, Math.min(Number(value) || 0, this.scrollHeight - this.clientHeight));
    if (top === this.top) return;
    this.top = top;
    scrolledSinceTheFrame.add(this);
  }
  emptied() {
    if (this.top && this.laidOut) { this.top = 0; scrolledSinceTheFrame.add(this); }
  }
  set textContent(value) {
    const gone = this.children;
    gone.forEach((one) => { one.parentNode = null; });
    this.children = [];
    this.own = String(value);
    if (this.isConnected && gone.length) aChange([], gone);
    this.emptied();
  }
  get textContent() { return this.own + this.children.map((one) => one.textContent).join(""); }
  appendChild(child) { return this.insertBefore(child, null); }
  append(...more) { more.forEach((one) => this.appendChild(typeof one === "string" ? new TextPiece(one) : one)); }
  insertBefore(child, before) {
    if (child.parentNode) child.parentNode.removeChild(child);
    child.parentNode = this;
    const at = before ? this.children.indexOf(before) : -1;
    if (at >= 0) this.children.splice(at, 0, child); else this.children.push(child);
    if (this.isConnected) aChange([child], []);
    return child;
  }
  removeChild(child) {
    const at = this.children.indexOf(child);
    if (at < 0) return child;
    const connected = this.isConnected;
    this.children.splice(at, 1);
    child.parentNode = null;
    if (connected) aChange([], [child]);
    if (!this.children.length) this.emptied();
    return child;
  }
  replaceChildren(...more) { this.textContent = ""; this.append(...more); }
  remove() { if (this.parentNode) this.parentNode.removeChild(this); }
  after(node) { const parent = this.parentNode; if (!parent) return; parent.insertBefore(node, parent.children[parent.children.indexOf(this) + 1] || null); }
  replaceWith(other) { const parent = this.parentNode; if (!parent) return; parent.insertBefore(other, this); parent.removeChild(this); }
  contains(node) { for (let at = node; at; at = at.parentNode) if (at === this) return true; return false; }
  get parentElement() { return this.parentNode; }
  get firstChild() { return this.children[0] || null; }
  get childElementCount() { return this.children.filter((one) => one instanceof Piece).length; }
  setAttribute(name, value) {
    if (name.startsWith("data-")) this.dataset[camel(name.slice(5))] = String(value);
    else if (name === "class") this.className = value;
    else this.attributes[name] = String(value);
  }
  getAttribute(name) {
    if (name.startsWith("data-")) { const held = this.dataset[camel(name.slice(5))]; return held === undefined ? null : String(held); }
    if (name === "class") return this.className;
    return name in this.attributes ? this.attributes[name] : null;
  }
  removeAttribute(name) { if (name.startsWith("data-")) delete this.dataset[camel(name.slice(5))]; else delete this.attributes[name]; }
  addEventListener(kind, handler) { (this.listeners[kind] ||= []).push(handler); }
  removeEventListener() {}
  dispatchEvent() { return true; }
  press() { for (const handler of this.listeners.click || []) handler({ preventDefault() {}, target: this }); }
  click() { this.press(); }
  focus() { document.activeElement = this; }
  blur() {}
  getBoundingClientRect() { return { left: 0, top: 0, right: 0, bottom: 0, width: 0, height: 0 }; }
  matches(selector) { return aTest(selector)(this); }
  closest(selector) { const test = aTest(selector); for (let at = this; at; at = at.parentNode) if (test(at)) return at; return null; }
  querySelectorAll(selector) { const test = aTest(selector); return everyPiece(this).slice(1).filter(test); }
  querySelector(selector) { return this.querySelectorAll(selector)[0] || null; }
}

export function everyPiece(root, found = []) {
  if (!(root instanceof Piece)) return found;
  found.push(root);
  root.children.forEach((one) => everyPiece(one, found));
  return found;
}

/// **A frame**: the scrolls since the last are told to the page, then the
/// parts newly laid out to whoever watches their size.
export function aFrame() {
  const scrolled = [...scrolledSinceTheFrame];
  scrolledSinceTheFrame.clear();
  for (const target of scrolled) {
    for (const [kind, handler, capture] of documentListeners) if (kind === "scroll" && capture) handler({ target });
  }
  for (const watcher of sizeWatchers) {
    const sized = [...watcher.watched].filter((one) => one.laidOut !== watcher.reported.get(one));
    sized.forEach((one) => watcher.reported.set(one, one.laidOut));
    if (sized.length) watcher.callback(sized.map((target) => ({ target })), watcher);
  }
}

/// **The user scrolls a part**, and a frame passes.
export function theUserScrolls(part, top) {
  part.scrollTop = top;
  aFrame();
}

const root = new Piece("html");
const body = new Piece("body");
globalThis.document = {
  documentElement: root,
  body,
  activeElement: null,
  createElement: (tag) => new Piece(tag),
  createElementNS: (space, tag) => new Piece(tag),
  createTextNode: (text) => new TextPiece(text),
  addEventListener(kind, handler, capture) { documentListeners.push([kind, handler, Boolean(capture)]); },
  removeEventListener() {},
  querySelector: (selector) => root.querySelector(selector),
  querySelectorAll: (selector) => root.querySelectorAll(selector),
};
root.appendChild(body);
globalThis.CSS = { escape: (text) => String(text) };
globalThis.MutationObserver = class {
  constructor(callback) { this.callback = callback; }
  observe() { observers.add(this); }
  disconnect() { observers.delete(this); }
};
globalThis.ResizeObserver = class {
  constructor(callback) { this.callback = callback; this.watched = new Set(); this.reported = new Map(); sizeWatchers.add(this); }
  observe(part) { this.watched.add(part); this.reported.set(part, false); }
  unobserve(part) { this.watched.delete(part); this.reported.delete(part); }
  disconnect() { this.watched.clear(); }
};
"#;

    /// **Render Settings, the queue and History, drawn and drawn again**, with
    /// the place the user had scrolled to read after each.
    const THE_HARNESS: &str = r#"// Render Settings, the queue and History, each drawn from what the application
// says and drawn again; the place the user had scrolled to read after each.
import { everyPiece, aFrame, theUserScrolls } from "./document.mjs";
import { readFileSync } from "node:fs";
const answers = JSON.parse(readFileSync("./answers.json", "utf8"));
globalThis.told = {};
globalThis.screens = {};
const settle = async () => { for (let at = 0; at < 10; at += 1) await new Promise((go) => setImmediate(go)); };
const tell = async (kind, said) => { for (const handler of globalThis.told[kind] || []) await handler({ said }); await settle(); };
const withBox = (root, box) => everyPiece(root).find((one) => one.dataset.box === box) || null;
const words = (piece) => (piece ? piece.textContent.replace(/\s+/g, " ").trim() : null);

const SERVER = answers.panel.server;
const asIs = (file) => {
  const one = structuredClone(answers.panel);
  if (file) { one.file = file; one.file_bytes = Array.from(Buffer.from(file)); }
  return one;
};
// **The third View Layer answered by the user**, as the application draws a
// box it holds a pin for: the user's, with the cross that hands it back.
const withAPin = () => {
  const one = asIs();
  const layer = one.the_view_layers[2];
  layer.whose = "the-users";
  layer.ticked = !layer.ticked;
  layer.hand_back_to = { lead: "the file's", values: [], rest: null, the_rest_joins: false, the_rest_binds: false };
  return one;
};
let current = withAPin();
const AN_ACT = "render-settings:an-act";
globalThis.window = { addEventListener() {}, __TAURI__: { core: { invoke(command) {
  if (command === "the_render_settings_panel") return Promise.resolve(structuredClone(current));
  if (command === "the_pane_layout") return Promise.resolve({ seams: [] });
  if (command === "a_pin_is_handed_back") {
    current = asIs();
    setImmediate(() => tell(AN_ACT, { server: SERVER, file: current.file, theBox: "view-layer", partCount: "0" }));
  }
  return Promise.resolve(null);
} } } };

// As `smoozy.js` loads it: before anything draws.
await import("./the-scrolled-place.js");
const { panels } = await import("./a-server.js");
const { theSettingsAreOpenedOn } = await import("./render-settings.js");
await import("./the-queue.js");
await import("./the-history.js");
const out = {};

// ── Render Settings ──
const wrap = document.createElement("div");
const head = document.createElement("div");
const body = document.createElement("div");
wrap.append(head, body);
document.body.appendChild(wrap);
panels[0].itIsDrawn({ head, body }, SERVER);
await theSettingsAreOpenedOn(SERVER, asIs().file_bytes, "");
await settle();
const columns = () => body.querySelector(".cols");
theUserScrolls(columns(), 400);
const before = columns();
// Note 7: the cross beside a View Layer the user answered.
withBox(body, "layer-cross-Layer 2").press();
await settle();
out.theCrossHandedBack = { at: columns().scrollTop, drawnAnew: columns() !== before, theCrossIsGone: !withBox(body, "layer-cross-Layer 2") };

// Note 8: Render pressed — *Submitting.*, then the confirmation; and a
// refusal. What the foot holds, in order, and what its own line says.
const foot = () => body.querySelector(".sfoot");
const theFootsParts = () => foot().children.map((one) => one.className);
// What the line under Render says: a submission that went, or what the
// submission answered, both on the foot's second line.
const theLine = () => {
  const second = foot().children.find((one) => one.classList.contains("sfl2"));
  return { textContent: second.children.filter((one) => one.dataset.box === "submitted" || one.dataset.box === "submit-answer").map((one) => one.textContent).join(" ") };
};
const SAID = { server: SERVER, file: current.file };
out.drawnEmpty = { parts: theFootsParts(), says: words(theLine()) };
await tell(AN_ACT, { ...SAID, theBox: "submit", partCount: "1", lead0: "Submitting.", plainParts: "1" });
out.submitting = { parts: theFootsParts(), says: words(theLine()) };
await tell(AN_ACT, { ...SAID, theBox: "submit", partCount: "1", lead0: "Submitted to", value00: "Vast 4090", rest0: ".",
  joins0: "yes", plainParts: "1", jobToShow: "job-1", showInQueue: "Show in Queue" });
out.submitted = { parts: theFootsParts(), says: words(theLine()), atTheColumns: columns().scrollTop };
await tell(AN_ACT, { ...SAID, theBox: "submit", partCount: "2", lead0: "Not submitted.", lead1: "Type the frames or tick a scene." });
out.refused = { parts: theFootsParts(), says: words(theLine()) };

// Collapsed, an answer arrives, and the panel comes back.
theUserScrolls(columns(), 250);
wrap.hidden = true;
await tell(AN_ACT, { ...SAID, theBox: "version-list", partCount: "0" });
out.collapsed = { whileDown: columns().scrollTop };
wrap.hidden = false;
aFrame();
out.collapsed.backUp = columns().scrollTop;

// A scroll the engine tells only after its part was drawn again while the
// panel was down: the part it tells about is gone, and reads as at its top,
// which is not where the user was.
columns().scrollTop = 450;
wrap.hidden = true;
await tell(AN_ACT, { ...SAID, theBox: "version-list", partCount: "0" });
aFrame();
wrap.hidden = false;
aFrame();
out.aScrollToldAfterItsPartWent = columns().scrollTop;

// Another file is a fresh page; the first again is too.
current = asIs("/ephemeral/smoozy/assets/hallway.blend");
await theSettingsAreOpenedOn(SERVER, current.file_bytes, "");
await settle();
out.anotherFile = columns().scrollTop;
current = asIs();
await theSettingsAreOpenedOn(SERVER, current.file_bytes, "");
await settle();
out.theFirstFileAgain = columns().scrollTop;

// ── The queue and History ──
function anEntry(job, more) {
  const said = {
    entryJob: job, entryName: "Kitchen " + job, entryServer: "Vast 4090", entryBlend: "/root/kitchen.blend", entryScene: "Scene",
    entryStatusCount: "1", entryStatus0Lead: "Rendering frame 12 of 240", factCount: "3", submittedWith: "Submitted with",
    fact0Name: "Frames", fact0Value: "1-240", fact1Name: "Samples", fact1Value: "1024", fact2Name: "Engine", fact2Value: "Cycles",
  };
  return Object.assign(said, more || {});
}
function aQueue(job, more) {
  const said = { theHeading: "Queue", lineCount: "1", line0Server: "s1", line0Name: "Vast 4090", rowCount: "4" };
  ["j3", "j7", "j8", "j9"].forEach((one, at) => Object.assign(said, { ["row" + at + "Job"]: one, ["row" + at + "Line"]: "0", ["row" + at + "Name"]: "Kitchen " + one }));
  return Object.assign(said, anEntry(job), more || {});
}
function aHistory(job) {
  const said = { hrowCount: "3", hrowChosen: job };
  ["j3", "h1", "h2"].forEach((one, at) => Object.assign(said, { ["hrow" + at + "Job"]: one, ["hrow" + at + "Name"]: "Kitchen " + one }));
  return Object.assign(said, anEntry(job));
}
const QUEUE = "queue:the-queue";
const HISTORY = "history:the-history";
const queue = document.createElement("div");
const history = document.createElement("div");
document.body.append(queue, history);
globalThis.screens.queueview.itIsDrawn(queue);
globalThis.screens.historyview.itIsDrawn(history);
const entry = (screen) => screen.querySelector(".qdbody");
const list = (screen) => screen.querySelector(".qscroll");

await tell(QUEUE, aQueue("j3"));
theUserScrolls(entry(queue), 400);
theUserScrolls(list(queue), 600);
// Note 9: the live reading, again and again.
await tell(QUEUE, aQueue("j3", { entryStatus0Lead: "Rendering frame 13 of 240" }));
await tell(QUEUE, aQueue("j3", { entryStatus0Lead: "Rendering frame 14 of 240" }));
out.theQueueReadAgain = { entry: entry(queue).scrollTop, list: list(queue).scrollTop };
await tell(QUEUE, aQueue("j7"));
out.anotherJob = { entry: entry(queue).scrollTop, list: list(queue).scrollTop };
await tell(QUEUE, aQueue("j3"));
out.theFirstJobAgain = { entry: entry(queue).scrollTop };

// The same job's entry on both screens keeps a place on each.
await tell(HISTORY, aHistory("j3"));
theUserScrolls(entry(queue), 400);
theUserScrolls(entry(history), 100);
theUserScrolls(list(history), 500);
await tell(QUEUE, aQueue("j3"));
await tell(HISTORY, aHistory("j3"));
out.theSameJobOnBothScreens = { queue: entry(queue).scrollTop, history: entry(history).scrollTop, historysList: list(history).scrollTop };
await tell(HISTORY, aHistory("h1"));
out.anotherHistoryEntry = { entry: entry(history).scrollTop, list: list(history).scrollTop };

// The queue away while it is drawn again, then shown.
queue.hidden = true;
await tell(QUEUE, aQueue("j3", { entryStatus0Lead: "Rendering frame 15 of 240" }));
out.theQueueAway = { whileAway: entry(queue).scrollTop };
queue.hidden = false;
aFrame();
out.theQueueAway.shownAgain = entry(queue).scrollTop;

console.log(JSON.stringify(out));
"#;
}

/// **Escape closes the thing on top, one layer at a time** —
/// `Preparation/Design/Whole app.md`, *Keyboard shortcuts*; the Creator's
/// seventh notes on the test copy, note 8.
///
/// **This runs the page code rather than reading it**: `web/the-escape-key.js`
/// is loaded under Node beside a stand-in for the little of a document it
/// reads, with stand-ins for the boxes that did not answer Escape before
/// 2026-10-10 — a Queue box, a History box, a Keys box, the Blender output
/// panel — each built in the shape its own file builds it, which the second
/// test pins to those files. **Skipped, and says so, where no Node is
/// installed.**
#[cfg(test)]
mod the_escape_key {
    use std::fs;

    fn the_page_folder() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web")
    }

    #[test]
    fn escape_presses_the_step_back_control_of_the_overlay_on_top_and_nothing_more() {
        let folder = std::env::temp_dir().join(format!(
            "smoozy-escape-key-harness-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&folder);
        fs::create_dir_all(&folder).expect("a scratch folder");
        let copied = fs::copy(
            the_page_folder().join("the-escape-key.js"),
            folder.join("the-escape-key.js"),
        );
        fs::write(folder.join("harness.js"), THE_ESCAPE_KEY_HARNESS).expect("the harness is written");
        let ran = std::process::Command::new("node")
            .arg(folder.join("harness.js"))
            .current_dir(&folder)
            .output();
        let _ = fs::remove_dir_all(&folder);
        copied.expect("web/the-escape-key.js is there to be run");
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the Escape key's page code was not run");
            return;
        };
        assert!(
            ran.status.success(),
            "the page code did not run: {}",
            String::from_utf8_lossy(&ran.stderr)
        );
        assert_eq!(
            String::from_utf8_lossy(&ran.stdout).trim(),
            r#"{"aQueueBoxCloses":true,"aHistoryBoxClosesAndNeverByItsRedControl":true,"aKeysBoxCloses":true,"theOutputPanelCloses":true,"oneLayerAtATime":true,"aBoxThatAnswersItselfKeepsIt":true,"aStoppedPressIsLeftAlone":true,"anOpenMenuOwnsThePress":true,"nothingOpenNothingDone":true,"aBoxWithNoStepBackStays":true,"anActUnderwayIsLeftToFinish":true,"anotherKeyIsNotTaken":true}"#
        );
    }

    /// **The stand-ins above are the real shapes.** Each box that did not
    /// answer Escape is a `.modal` whose step-back control is a `.btn.quiet`
    /// that closes it; if a file stops building it that way, the listener no
    /// longer reaches it, and this says which. And the page loads the listener.
    #[test]
    fn the_boxes_that_lacked_escape_are_built_the_way_the_listener_finds_them() {
        let read = |name: &str| fs::read_to_string(the_page_folder().join(name)).expect("a page file");
        let queue = read("the-queue.js");
        assert!(queue.contains("modal.className = \"modal\";"), "the Queue and History box is a .modal");
        assert!(
            queue.contains("aButton(said.boxKeep, \"btn\", \"quiet\");")
                && queue.contains("keep.addEventListener(\"click\", () => modal.remove());"),
            "the Queue and History box's step-back control is a .btn.quiet that closes it"
        );
        let keys = read("keys.js");
        assert!(keys.contains("root.className = \"modal\";"), "every Keys box is a .modal");
        assert!(keys.contains("cancel.className = \"btn quiet\";"), "a Keys box steps back by a .btn.quiet");
        let words = read("the-machines-own-words.js");
        assert!(words.contains("panel.className = \"modal top words-panel\";"), "the output panel is a .modal.top");
        assert!(
            words.contains("close.className = \"btn quiet\";") && words.contains("panel.hidden = true;"),
            "the output panel's Close is a .btn.quiet that hides it"
        );
        assert!(read("smoozy.js").contains("import \"./the-escape-key.js\";"), "the page loads the Escape key");
    }

    /// The stand-in document the Escape key's page code is run against.
    const THE_ESCAPE_KEY_HARNESS: &str = r#"
class Element {
  constructor(tag, classes = []) {
    this.tagName = tag;
    this.classes = new Set(classes);
    this.children = [];
    this.parent = null;
    this.hidden = false;
    this.disabled = false;
    this.clicks = 0;
    this.onclick = null;
  }
  get classList() {
    return { contains: (one) => this.classes.has(one) };
  }
  append(...kids) {
    for (const kid of kids) {
      kid.parent = this;
      this.children.push(kid);
    }
    return this;
  }
  remove() {
    if (!this.parent) return;
    this.parent.children = this.parent.children.filter((one) => one !== this);
    this.parent = null;
  }
  get isConnected() {
    let at = this;
    while (at.parent) at = at.parent;
    return at === body;
  }
  closest(selector) {
    if (selector !== "[hidden]") throw new Error("unexpected " + selector);
    for (let at = this; at; at = at.parent) if (at.hidden) return at;
    return null;
  }
  getClientRects() {
    return this.isConnected && !this.closest("[hidden]") ? [1] : [];
  }
  all() {
    return [this, ...this.children.flatMap((kid) => kid.all())];
  }
  querySelectorAll(selector) {
    const everything = this.all().slice(1);
    if (selector === ".modal") return everything.filter((one) => one.classes.has("modal"));
    if (selector === ".ctx") return everything.filter((one) => one.classes.has("ctx"));
    if (selector === "button.btn") {
      return everything.filter((one) => one.tagName === "BUTTON" && one.classes.has("btn"));
    }
    throw new Error("unexpected " + selector);
  }
  click() {
    this.clicks += 1;
    if (this.onclick) this.onclick();
  }
}
const body = new Element("BODY");
const capturing = [];
const bubbling = [];
globalThis.window = {
  addEventListener(kind, listener, capture) {
    if (kind === "keydown") (capture ? capturing : bubbling).push(listener);
  },
};
globalThis.document = { querySelectorAll: (selector) => body.querySelectorAll(selector) };
require("./the-escape-key.js");

// **The page's own listeners**, which hear the key between the window's
// capture and the window's bubble, as on an element or the document.
let own = [];
function press(key = "Escape") {
  const event = {
    key,
    defaultPrevented: false,
    stopped: false,
    isComposing: false,
    shiftKey: false,
    altKey: false,
    ctrlKey: false,
    metaKey: false,
    preventDefault() {
      this.defaultPrevented = true;
    },
    stopPropagation() {
      this.stopped = true;
    },
  };
  for (const listener of capturing) listener(event);
  for (const listener of own) if (!event.stopped) listener(event);
  if (!event.stopped) for (const listener of bubbling) listener(event);
  return event.defaultPrevented;
}
function button(...classes) {
  return new Element("BUTTON", ["btn", ...classes]);
}

// `web/the-queue.js`, `theBoxIsDrawn`: Queue and History alike.
function aQueueBox(red = false) {
  const modal = new Element("DIV", ["modal"]);
  const box = new Element("DIV", ["mbox"]);
  const row = new Element("DIV", ["mrow"]);
  const keep = button("quiet");
  keep.onclick = () => modal.remove();
  const go = red ? button("danger") : button();
  row.append(new Element("SPAN", ["grow"]), keep, go);
  box.append(row);
  modal.append(box);
  body.append(modal);
  return { modal, keep, go };
}
// `web/keys.js`, `aBox` and its Cancel.
function aKeysBox() {
  const root = new Element("DIV", ["modal"]);
  const box = new Element("DIV", ["mbox"]);
  const acts = new Element("DIV", ["mact"]);
  const cancel = button("quiet");
  cancel.onclick = () => (root.hidden = true);
  const go = button();
  acts.append(cancel, go);
  box.append(acts);
  root.append(box);
  body.append(root);
  return { root, cancel, go };
}
// `web/the-machines-own-words.js`, `theWordsPanel` and its Close.
function theOutputPanel() {
  const panel = new Element("DIV", ["modal", "top", "words-panel"]);
  const box = new Element("DIV", ["lbox"]);
  const tabs = new Element("DIV", ["ltabs"]);
  tabs.append(new Element("BUTTON", ["ltab"]), new Element("BUTTON", ["ltab"]));
  const foot = new Element("DIV", ["lfoot"]);
  const close = button("quiet");
  close.onclick = () => (panel.hidden = true);
  foot.append(close);
  box.append(tabs, foot);
  panel.append(box);
  body.append(panel);
  return { panel, close };
}
function clear() {
  body.children = [];
  own = [];
}

const out = {};

let queue = aQueueBox();
out.aQueueBoxCloses = press() && queue.keep.clicks === 1 && queue.go.clicks === 0 && !queue.modal.isConnected;
clear();

let history = aQueueBox(true);
out.aHistoryBoxClosesAndNeverByItsRedControl =
  press() && history.keep.clicks === 1 && history.go.clicks === 0 && !history.modal.isConnected;
clear();

const keys = aKeysBox();
out.aKeysBoxCloses = press() && keys.cancel.clicks === 1 && keys.go.clicks === 0 && keys.root.hidden;
clear();

let panel = theOutputPanel();
out.theOutputPanelCloses = press() && panel.close.clicks === 1 && panel.panel.hidden;
clear();

// The output panel open over a History box: the panel first, then the box.
const under = aQueueBox();
panel = theOutputPanel();
body.children.reverse();
press();
const first = panel.panel.hidden && under.modal.isConnected && under.keep.clicks === 0;
press();
out.oneLayerAtATime = first && !under.modal.isConnected && panel.close.clicks === 1;
clear();

// A box that answers Escape itself and says so (`web/servers.js`, the dependency
// check), open over another box: it closes, and the box under it stays.
const beneath = aQueueBox();
let itsOwn = aKeysBox();
let heard = 0;
own.push((event) => {
  if (event.key !== "Escape" || event.defaultPrevented) return;
  heard += 1;
  event.preventDefault();
  itsOwn.root.hidden = true;
});
out.aBoxThatAnswersItselfKeepsIt =
  press() && heard === 1 && itsOwn.cancel.clicks === 0 && beneath.modal.isConnected && beneath.keep.clicks === 0;
clear();

// A box that answers it and stops it on the way up (`web/telegram.js`, `web/settings.js`).
itsOwn = aKeysBox();
heard = 0;
own.push((event) => {
  heard += 1;
  event.stopPropagation();
});
out.aStoppedPressIsLeftAlone = !press() && heard === 1 && itsOwn.cancel.clicks === 0;
clear();

// A menu that closes on Escape without saying so (`web/the-queue.js`, the row
// menu; the app menu), open over a box: the menu goes, the box stays.
queue = aQueueBox();
const menu = new Element("DIV", ["ctx"]);
body.append(menu);
own.push((event) => {
  if (event.key === "Escape") menu.remove();
});
press();
const menuGone = !menu.isConnected && queue.modal.isConnected && queue.keep.clicks === 0;
press();
out.anOpenMenuOwnsThePress = menuGone && !queue.modal.isConnected;
clear();

out.nothingOpenNothingDone = !press();
const hiddenBox = aKeysBox();
hiddenBox.root.hidden = true;
out.nothingOpenNothingDone = out.nothingOpenNothingDone && !press() && hiddenBox.cancel.clicks === 0;
clear();

// A box with no step-back control: only a disabled Cancel and a Done.
const bare = aKeysBox();
bare.cancel.disabled = true;
out.aBoxWithNoStepBackStays = !press() && bare.cancel.clicks === 0 && bare.go.clicks === 0 && !bare.root.hidden;
clear();

const busy = aKeysBox();
busy.go.classes.add("uway");
out.anActUnderwayIsLeftToFinish = !press() && busy.cancel.clicks === 0 && !busy.root.hidden;
clear();

queue = aQueueBox();
out.anotherKeyIsNotTaken = !press("Enter") && !press("Tab") && queue.keep.clicks === 0;
clear();

console.log(JSON.stringify(out));
"#;
}
