//! The marker guard: a clause marker written here resolves to the clause it
//! means, and never to a different document's clause of the same name.
//!
//! **A marker with no document beside it means a clause of this crate's own
//! plan**, `Preparation/Architecture/Desktop app and delivery.md`. **Every other document's
//! marker is written with that document**, at the site, or in that file's own
//! header list of the markers in it that belong elsewhere. The same short
//! marker is defined by many plans at once, so a bare one does not fail to
//! resolve: **it resolves to a real and wrong clause, which nobody reading it
//! can tell from a right one.**
//!
//! **Where a marker is a clause of this crate's own plan *and* of the document
//! meant, the header list is not enough and the document is written out at the
//! site**, because a bare use could then mean either.
//!
//! This is the shape settled on 2026-09-04 in `App/Server Setup/` and
//! `App/Rented-server runtime/`, carried here unchanged but for the two
//! readings named at the test below.
//!
//! **One more test lives here from 2026-09-06, and it is about the plan rather
//! than about attribution**:
//! [`tests::the_plans_clause_count_has_not_moved_under_this_crate`]. It is here
//! because this is the only file in this crate that reads the plan at all, and
//! its own reason is written at it.

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::fs;
    use std::path::{Path, PathBuf};

    /// This crate's own plan.
    const THIS_PLANS_NAME: &str = "Desktop app and delivery";

    /// **Every file of this crate, counted exactly.** See
    /// [`every_file_of_this_crate`] for why the number is here rather than a
    /// floor.
    ///
    /// **116 to 117 on 2026-09-28**: step 10c's transfer wave piece 1 wrote
    /// its desk harness as `tests/huge_transfers_at_the_desk.rs`, kept in the
    /// tree this time, and the walk reads `tests/`. Moved by piece 8, which
    /// holds this file for the wave, so the two builders do not both move it.
    ///
    /// **117 to 118 on 2026-09-29**: step 10c's dependency wave piece 3 added
    /// `src/reading_the_file.rs`, the driver of *Read the file*.
    ///
    /// **118 to 119 on 2026-09-29**: step 10c's dependency wave piece 4 added
    /// `src/record/dependency_records.rs`, the keeper of `Dependency Upload &
    /// Remap`'s record.
    ///
    /// **119 to 121 on 2026-09-29**: step 10c's dependency wave piece 5 added
    /// `src/checking_a_file.rs`, the driver of *Check dependencies…*, and its
    /// desk harness `tests/the_check_at_the_desk.rs`.
    ///
    /// **121 to 122 on 2026-09-29**: step 10c's S2 added
    /// `src/servers/stop_and_clear.rs`, the one sender of the stop-and-clear
    /// pair.
    ///
    /// **122 to 123 on 2026-09-29**: step 10c's dependency wave piece 6 added
    /// `src/the_found_files_sent.rs`, the sending of the check's found files.
    /// S2 landed its file first and moved the count once; this is the second.
    ///
    /// **123 to 124 on 2026-09-29**: step 10c's S3 added
    /// `src/servers/the_queues_own_acts.rs`, the queue's own stop, cancel and
    /// group cancel. Dependency piece 7 was in the tree beside it; if it adds
    /// a file too, whichever lands second moves this once more.
    ///
    /// **124 to 125 on 2026-10-02**: step 10c's check-before-0.1.7 fix piece 1
    /// added `tests/a_line_cut_silently_under_a_real_connection.rs`, a real
    /// connection through OpenSSH's own server with a line that goes silent.
    ///
    /// **125 to 126 on 2026-10-02**: step 10c's Files' owed acts piece 3 added
    /// `src/managing_files.rs`, `File Browser` §7's acts done on the machine.
    ///
    /// **126 to 127 on 2026-10-02**: step 10c's Files' owed acts piece 4 added
    /// `src/record/file_acts.rs`, `File Browser` §11's record for a delete, a
    /// move and a copy.
    ///
    /// **127 to 130 on 2026-10-03**: the 38's piece 1 added
    /// `src/record/idle_marks.rs` (`Queue & History` W5's mark),
    /// `src/servers/the_endings_posted.rs` and `src/servers/the_idle_recheck.rs`.
    ///
    /// **130 to 132 on 2026-10-03**: the Creator's notes on 0.1.8, piece 6d,
    /// added `src/record/kept_words.rs` (`Queue & History` E9's kept words)
    /// and `src/servers/the_words_kept.rs` (their fetch).
    ///
    /// **132 to 135 on 2026-10-03**: the 38's piece 5 added
    /// `src/the_licence_check.rs` (the historical paid-check sequence),
    /// `src/the_licence_check/at_the_desk.rs` (its tests against the built
    /// licence service on loopback) and `src/the_address_opener.rs` (K25).
    ///
    /// **135 to 136 on 2026-10-03**: the 38's piece 2 added
    /// `src/a_new_job_from_an_old_one.rs` (`Rendering` §6's route from an
    /// entry to the page).
    ///
    /// **136 to 138 on 2026-10-03**: the 38's piece 3, desktop half, added
    /// `src/the_previews.rs` (`Queue & History` §15's fetch, holding and age
    /// sweep) and `src/record/previews.rs` (record version 37's store).
    ///
    /// **138 to 139 on 2026-10-03**: the 38's piece 6 added
    /// `tests/the_set_up_card_read_from_the_machine.rs` (`Server Setup` Y6a
    /// and `Server Setup` Y6b at the desk, through the machine's own program).
    ///
    /// **139 to 140 on 2026-10-03**: the Creator's notes on the test copy,
    /// piece 1, added `tests/a_wrong_key_is_said_where_the_user_is.rs`
    /// (`Servers` S10 item 5 met by a real OpenSSH server refusing the key, and
    /// the answer naming its server).
    ///
    /// **140 to 141 on 2026-10-04**: the 38's piece 15 added
    /// `src/record/catch_up_notices.rs` (`Queue & History` C2's notice record,
    /// record version 38's store).
    ///
    /// **141 to 142 on 2026-10-06**: the Creator's second notes on the test
    /// copy, piece 9, added `src/platform/the_open_panel.rs` (`File Browser`
    /// H3's macOS panel taking files and folders together).
    ///
    /// **142 to 143 on 2026-10-12**: the OptiX and driver work's piece that
    /// builds `Server Setup` R1b added `src/servers/the_driver_swap.rs` (the
    /// driver following every Blender, and the swap waiting for the server's
    /// work to end).
    ///
    /// **143 to 144 on 2026-10-16**: the Telegram worker's piece 5 added
    /// `tests/a_job_the_machine_made_is_found_on_the_next_connection.rs`
    /// (`State, identity and recovery` Y7a's adoption of a job the machine
    /// made on a phone's *try it again*, at the desk).
    ///
    /// **144 to 145 on 2026-10-16**: step 10c's owed piece 4 added
    /// `src/record/installs_not_begun.rs` (`Server Setup` Y9a's list of the
    /// installs still waiting on a server, in the local record).
    ///
    /// **145 to 146 on 2026-10-19**: the live pass's faults, piece D, added
    /// `tests/a_submission_on_a_line_that_goes_silent.rs` (`Queue & History`
    /// C10's not-submitted row, proved over a line gone silent at the desk).
    /// **146 to 148 on 2026-10-19**: the dead-line repairs added
    /// `src/record/submission_material.rs` and `src/submission_cleanup.rs`.
    /// **148 to 150 on 2026-10-19**: the dead-line desk coverage added
    /// `tests/a_dead_line_under_the_first_half_of_the_acts.rs` and
    /// `tests/the_server_acts_on_a_line_that_goes_silent.rs`, one for each
    /// half of the acts.
    /// **152 to 153 on 2026-10-24**: step 11's R1 added
    /// `src/the_one_instance.rs`, P2's one running copy.
    /// **153 to 154 on 2026-10-24**: step 11's R3a added
    /// `src/the_report_take.rs`, what a reachable machine adds to a report.
    /// **154 to 155 on 2026-10-24**: step 11's R4 added
    /// `src/surface/reporting.rs`, the report box, its write and its reveal.
    /// **155 to 156 on 2026-10-24**: step 11's R5 added
    /// `src/the_last_line.rs`, the raw last line.
    /// **156 to 161 on 2026-10-24**: step 11's update-reading piece added
    /// `src/updating/mod.rs`, `src/updating/manifest.rs`,
    /// `src/updating/trust.rs`, `src/updating/verifying.rs` and
    /// `src/record/update_trust.rs`.
    /// **161 to 162 on 2026-10-24**: step 11's check piece added
    /// `src/updating/checking.rs`, the update check and its one sequence.
    /// **162 to 163 on 2026-10-25**: the eighteenth notes, note 14, added
    /// `tests/files_list_again_once_the_connection_is_back.rs`, Files after
    /// reconnecting timed at the desk.
    /// **163 to 167 on 2026-10-25**: step 11's Mac update piece added
    /// `src/updating/fetching.rs`, `src/updating/the_hold.rs`,
    /// `src/updating/the_mac.rs` and `src/updating/the_probe.rs`.
    /// **167 to 168 on 2026-10-25**: the eighteenth notes, note 2, added
    /// `src/surface/selecting_text.rs`, the page's selection checks.
    /// **168 to 169 on 2026-10-25**: step 11's Linux update piece added
    /// `src/updating/the_linux_package.rs`.
    /// **169 to 171 on 2026-10-26**: step 11's Windows update piece added
    /// `src/updating/the_windows_side_by_side.rs`, the Windows update route,
    /// and `src/updating/the_relaunch_helper.rs`, its relaunch helper.
    /// 171 to 166: free-and-open-source wave 2 removed surface/access.rs,
    /// the_licence_check.rs, its at_the_desk.rs, record/licence_record.rs
    /// and platform/identity.rs. Added platform/key_names.rs to preserve
    /// Keys' existing default name; the result is 171 - 5 + 1 = 167.
    /// 167 to 168 on 2026-10-27: the free-software drawing’s browser check
    /// adds `tests/the_free_drawing.rs`; all source and test files are counted.
    const THE_FILE_COUNT: usize = 168;

    /// Whether a word has a clause marker's shape.
    ///
    /// **One or two letters, one or two digits, and at most two trailing
    /// lower-case letters.**
    ///
    /// **It is deliberately tighter than a capital followed by a digit**, which
    /// matches a version number inside a derived path — `Blender5` is the
    /// standing instance — and would put the attribution leg on prose rather
    /// than on a marker.
    fn looks_like_a_marker(word: &str) -> bool {
        let bytes = word.as_bytes();
        if bytes.is_empty() || !bytes[0].is_ascii_uppercase() {
            return false;
        }
        let mut at = 1;
        if at < bytes.len() && bytes[at].is_ascii_alphabetic() {
            at += 1;
        }
        let digits_from = at;
        while at < bytes.len() && bytes[at].is_ascii_digit() {
            at += 1;
        }
        if !(1..=2).contains(&(at - digits_from)) {
            return false;
        }
        let mut tail = 0;
        while at < bytes.len() && bytes[at].is_ascii_lowercase() && tail < 2 {
            at += 1;
            tail += 1;
        }
        at == bytes.len()
    }

    /// Every clause a plan defines, in the plan's own order.
    ///
    /// **The same reading is applied to this plan and to every other**, so a
    /// marker's document is decided one way rather than two.
    ///
    /// **It admits three shapes that a stricter reading misses, and each was
    /// found by a correct citation being called a wrong one.**
    ///
    /// - **A clause is not required to be followed by a full stop.** One plan
    ///   opens a clause with the marker, a space and a dash, and a reading that
    ///   demanded the full stop could not see it at all.
    /// - **A clause may be a row of a table.** `Servers` states its two
    ///   period-opening acts and its three connection acts that way, and a
    ///   reading that only took bold runs at the start of a line called this
    ///   crate's neighbours' correct bare citations of them faults.
    /// - **A clause may sit under a heading that carries no number.**
    ///   `State, identity and recovery` states the four rules that bind it
    ///   before anything is decided under such a heading.
    ///
    /// **A clause a checker cannot see is a clause nothing guards**, and it is
    /// worse than that: the same blindness turns a correct citation of it into
    /// a reported wrong attribution.
    fn the_clauses_defined_by(plan: &str) -> Vec<String> {
        let mut under_a_heading = false;
        let mut out = Vec::new();
        for line in plan.lines() {
            if line.starts_with("## ") || line.starts_with("### ") {
                under_a_heading = true;
            }
            if !under_a_heading {
                continue;
            }
            let mut body = line.trim();
            for opener in ["- ", "* ", "| "] {
                if let Some(rest) = body.strip_prefix(opener) {
                    body = rest;
                    break;
                }
            }
            let Some(rest) = body.strip_prefix("**") else {
                continue;
            };
            let marker: String = rest.chars().take_while(char::is_ascii_alphanumeric).collect();
            // The marker is followed by its own punctuation, a space, or the
            // rest of the bold run; a bolded sentence opening with an ordinary
            // word is not one.
            let after = rest[marker.len()..].chars().next().unwrap_or(' ');
            if looks_like_a_marker(&marker) && matches!(after, '.' | ' ' | ',' | ':' | '*' | '—') {
                out.push(marker);
            }
        }
        out
    }

    fn the_architecture_folder() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../Preparation/Architecture")
    }

    /// Every plan, by name, with the clauses it defines. **Read from the folder
    /// rather than kept here**, so a plan added later is read without anybody
    /// remembering.
    fn the_marker_dictionary() -> Vec<(String, HashSet<String>)> {
        let mut out = Vec::new();
        for entry in fs::read_dir(the_architecture_folder()).expect("the folder is readable") {
            let path = entry.expect("the entry is readable").path();
            if path.extension().and_then(|e| e.to_str()) != Some("md") {
                continue;
            }
            let name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .expect("the plan has a name")
                .to_string();
            let text = fs::read_to_string(&path).expect("the plan is readable");
            out.push((name, the_clauses_defined_by(&text).into_iter().collect()));
        }
        assert!(
            out.len() > 40,
            "the Architecture documents were not found, so the dictionary would be empty"
        );
        assert!(
            out.iter().any(|(name, _)| name == THIS_PLANS_NAME),
            "this crate's own plan was not found in the dictionary"
        );
        out
    }

    /// Every `.rs` file under a folder, **however deep**, as (path, text).
    ///
    /// A directory named `target` is not walked: it holds no source of this
    /// crate's, only what a build put there.
    fn every_rust_file_under(folder: &Path) -> Vec<(String, String)> {
        let mut out = Vec::new();
        let Ok(entries) = fs::read_dir(folder) else {
            return out;
        };
        for entry in entries {
            let path = entry.expect("the entry is readable").path();
            if path.is_dir() {
                if path.file_name().and_then(|s| s.to_str()) == Some("target") {
                    continue;
                }
                out.extend(every_rust_file_under(&path));
            } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                out.push((
                    path.display().to_string(),
                    fs::read_to_string(&path).expect("the file is readable"),
                ));
            }
        }
        out
    }

    /// Every file of this crate, as (name, body, header list).
    ///
    /// **Nothing is left out** — the front matter, the build script and the
    /// integration tests are read the same as the working files, because a
    /// marker written bare in any of them resolves to the wrong clause just as
    /// readily.
    ///
    /// **The walk recurses and the count is asserted exactly, and both halves
    /// of that are the point.** `Rented-server runtime` grew two sub-folders
    /// during the very step that settled this test. Under a walk that read one
    /// level with a floor rather than an exact count, every file in a new
    /// folder would have escaped the check in silence with the floor still
    /// passing. **A floor cannot fail on a file that was never read, and cannot
    /// notice one that has gone.**
    fn every_file_of_this_crate() -> Vec<(String, String, String)> {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let read = every_rust_file_under(root);
        assert_eq!(
            read.len(),
            THE_FILE_COUNT,
            "this crate's files were not all found, or one was added or removed \
             without this count moving with it"
        );
        let mut out = Vec::new();
        for (path, text) in read {
            let name = Path::new(&path)
                .file_name()
                .and_then(|s| s.to_str())
                .expect("the file has a name")
                .to_string();
            let (body, header) = split_off_the_header_list(&text);
            out.push((name, body, header));
        }
        out.extend(every_page_file_of_this_crate());
        out
    }

    /// **Every file of the page**, as (name, body, header list).
    ///
    /// **Added 2026-09-12, out of round 1 of the code audit of what is drawn
    /// inside the window (F-8).** This guard walked `.rs` files only, so **a
    /// marker written into a page file was checked by nobody**. Two sites in
    /// `web/the-failure-shape.js` wrote `X4` bare, meaning `Reporting a
    /// problem` X4 — and this crate's own plan defines an X4 of its own, about
    /// character substitution, so each resolved to a real and wrong clause. The
    /// neighbouring application file gets it right and says in its own head
    /// exactly why it must.
    ///
    /// **It costs nothing and it covers the eleven screens that come after**,
    /// every one of which will carry comments of the same kind. The reading is
    /// the same one applied to the Rust files: a page comment is stripped of its
    /// `//` the same way a doc comment is stripped of its `///`.
    ///
    /// **The count is not asserted here and that is deliberate.** `web/` is one
    /// flat folder whose files arrive one per piece, and the exact-count
    /// discipline [`every_file_of_this_crate`] needs is for a walk that recurses
    /// — the failure that count exists to catch is a sub-folder nobody reads,
    /// and there are none here.
    fn every_page_file_of_this_crate() -> Vec<(String, String, String)> {
        let folder = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        let mut out = Vec::new();
        for entry in fs::read_dir(folder).expect("the page folder is readable") {
            let path = entry.expect("the entry is readable").path();
            let extension = path.extension().and_then(|e| e.to_str());
            if !matches!(extension, Some("js") | Some("css") | Some("html")) {
                continue;
            }
            let name = path
                .file_name()
                .and_then(|s| s.to_str())
                .expect("the file has a name")
                .to_string();
            let text = fs::read_to_string(&path).expect("the file is readable");
            let (body, header) = split_off_the_header_list(&text);
            out.push((format!("web/{name}"), body, header));
        }
        out.sort();
        out
    }

    /// The file's text as one run with the comment markers taken off, split
    /// into the body and **the header list of markers that belong to another
    /// document** — that list names other documents' clauses and never keeps
    /// one.
    ///
    /// **The list ends at the first empty comment line**, so a file may follow
    /// it with a note saying which markers it deliberately does not list and
    /// why. A note inside the list would put those markers straight back into
    /// it.
    ///
    /// **Every line of both halves is trimmed before it is joined, and joined
    /// with exactly one space.** A document's name broken across two comment
    /// lines otherwise reads with a double space in it, matches no plan, and
    /// turns a correct citation into an unattributed one.
    fn split_off_the_header_list(text: &str) -> (String, String) {
        let mut body = String::new();
        let mut header = String::new();
        let mut in_the_header_list = false;
        for line in text.lines() {
            let line = line.trim();
            if line.contains("Markers used") {
                in_the_header_list = true;
            }
            if in_the_header_list {
                match line.strip_prefix("//!").or_else(|| line.strip_prefix("///")) {
                    Some(rest) if !rest.trim().is_empty() => {
                        header.push_str(rest.trim());
                        header.push(' ');
                        continue;
                    }
                    _ => in_the_header_list = false,
                }
            }
            let line = line
                .strip_prefix("///")
                .or_else(|| line.strip_prefix("//!"))
                .or_else(|| line.strip_prefix("//"))
                .unwrap_or(line);
            body.push_str(line.trim());
            body.push(' ');
        }
        (body, header)
    }

    /// The header list read **as segments rather than as one flat list**: each
    /// entry is a marker with the document whose name last opened a segment.
    ///
    /// **A header list is written as a plan's name in backticks followed by
    /// that plan's markers, then the next plan's name and its markers**, so a
    /// backticked plan name opens a segment and every marker after it belongs
    /// to that plan until the next name. **Reading the list flat is what let a
    /// wrongly attributed marker through**: the flat list forgave any bare
    /// marker in the body that appeared anywhere in it, whatever document it
    /// was filed under, and nothing ever asked whether that document defines
    /// it.
    ///
    /// A marker standing before any plan name carries `None` and is a fault of
    /// its own — the list's own heading says each marker is named with the
    /// document it belongs to.
    fn the_header_list_by_document(
        header: &str,
        plan_names: &[String],
    ) -> Vec<(Option<String>, String)> {
        let mut out = Vec::new();
        let mut current: Option<String> = None;
        let bytes = header.as_bytes();
        let mut at = 0;
        while at < bytes.len() {
            if bytes[at] == b'`' {
                let Some(closes) = header[at + 1..].find('`').map(|found| at + 1 + found) else {
                    break;
                };
                let named = &header[at + 1..closes];
                if let Some(plan) = plan_names.iter().find(|plan| plan.as_str() == named) {
                    current = Some(plan.clone());
                }
                at = closes + 1;
                continue;
            }
            if bytes[at].is_ascii_alphanumeric() {
                let mut end = at;
                while end < bytes.len() && bytes[end].is_ascii_alphanumeric() {
                    end += 1;
                }
                let word = &header[at..end];
                if looks_like_a_marker(word) {
                    out.push((current.clone(), word.to_string()));
                }
                at = end;
                continue;
            }
            at += 1;
        }
        out
    }

    /// The document written immediately before the marker at `at`, in
    /// backticks, where that is one of the plans.
    ///
    /// **By the rule this file states, a marker with nothing backticked before
    /// it means a clause of this crate's own plan.**
    fn the_document_standing_beside(
        text: &str,
        at: usize,
        plan_names: &[String],
    ) -> Option<String> {
        let before = text[..at]
            .trim_end()
            .trim_end_matches(|c: char| "*([{\"'".contains(c))
            .trim_end();
        let inner = before.strip_suffix('`')?;
        let opens = inner.rfind('`')?;
        let named = &inner[opens + 1..];
        plan_names
            .iter()
            .find(|plan| plan.as_str() == named)
            .cloned()
    }

    /// Every marker-shaped word in `text`, with the document written beside it
    /// where one is, as (marker, that document).
    fn the_markers_in(text: &str, plan_names: &[String]) -> Vec<(String, Option<String>)> {
        let bytes = text.as_bytes();
        let mut out = Vec::new();
        let mut at = 0;
        while at < bytes.len() {
            let starts_a_word = bytes[at].is_ascii_uppercase()
                && (at == 0 || !(bytes[at - 1].is_ascii_alphanumeric() || bytes[at - 1] == b'_'));
            if !starts_a_word {
                at += 1;
                continue;
            }
            let mut end = at;
            while end < bytes.len() && bytes[end].is_ascii_alphanumeric() {
                end += 1;
            }
            let word = &text[at..end];
            let followed_by_underscore = end < bytes.len() && bytes[end] == b'_';
            if looks_like_a_marker(word) && !followed_by_underscore {
                out.push((
                    word.to_string(),
                    the_document_standing_beside(text, at, plan_names),
                ));
            }
            at = end.max(at + 1);
        }
        out
    }

    /// **The marker rule, held rather than remembered.**
    ///
    /// About a hundred and fifty bare markers were swept out of nine crates on
    /// 2026-09-03 having survived five gate runs, and fifteen more were found
    /// on the day the first two of these tests were written. **Every earlier
    /// fix was a habit somebody had to remember at the moment the comment was
    /// typed. This is the first one that does not depend on anybody
    /// remembering.**
    ///
    /// Five things fail here, over every file of this crate.
    ///
    /// - **A bare marker belonging to another document.** A bare one is allowed
    ///   only where this plan defines it, or where the file's own header list
    ///   names it.
    /// - **A marker written with a document that does not define it.** That is
    ///   the wrongly-attributed case, and it reads exactly like a right one.
    /// - **A header list naming a marker this plan also defines.** The header
    ///   list is not enough for such a marker, because a bare use could then
    ///   mean either document and nothing could tell; the document is written
    ///   out at the site instead.
    /// - **A header list naming a marker the document it is filed under does
    ///   not define.** Without this the list is an unchecked whitelist: every
    ///   marker in it flattened into one set, and any bare marker in the body
    ///   that appeared there forgiven, whatever document it was filed under.
    /// - **A header entry filed under no document at all**, which the list's
    ///   own heading forbids.
    ///
    /// **What it cannot see, said rather than glossed, because a test that
    /// claims more than it does is worse than a narrow one.**
    ///
    /// - **The scan is line-based and compiles nothing.** A marker inside a
    ///   name with an underscore in it is skipped, and so is one reached
    ///   through a link rather than written out.
    /// - **A capital followed by a digit is not taken as a marker's shape**,
    ///   because that matches a version number inside a derived path and puts
    ///   the attribution leg on prose. [`looks_like_a_marker`] is tighter.
    /// - **A plan's name broken across two comment lines reads with a double
    ///   space** unless every line is trimmed before it is joined — and a name
    ///   with a double space in it matches no plan, so a correct citation is
    ///   called unattributed. [`split_off_the_header_list`] trims both halves.
    /// - **A bare marker-shaped word that no plan defines anywhere is not a
    ///   fault**, because it is not a marker: `MD5` has that shape and is the
    ///   name of a digest. **This leg fires on the case the class is about** —
    ///   a bare marker that resolves to a real clause of the wrong document —
    ///   and not on a stale or invented marker that resolves to nothing.
    /// - **It cannot tell a bare marker that means this plan's clause from one
    ///   that means a colliding clause of a document named earlier in the same
    ///   sentence**, which is why the third leg above forbids the header list
    ///   from standing in for writing the document out.
    /// - **It decides what a document defines by one reading applied
    ///   everywhere**, so a clause stated in a shape [`the_clauses_defined_by`]
    ///   does not catch is invisible to it in both directions.
    #[test]
    fn no_marker_stands_bare_that_is_not_this_plans_and_none_is_wrongly_attributed() {
        let dictionary = the_marker_dictionary();
        let plan_names: Vec<String> = dictionary.iter().map(|(name, _)| name.clone()).collect();
        let ours: &HashSet<String> = &dictionary
            .iter()
            .find(|(name, _)| name == THIS_PLANS_NAME)
            .expect("this crate's plan is in the dictionary")
            .1;
        let anybody_elses: HashSet<&String> = dictionary
            .iter()
            .filter(|(name, _)| name != THIS_PLANS_NAME)
            .flat_map(|(_, markers)| markers.iter())
            .collect();

        let mut faults = Vec::new();
        for (name, body, header) in every_file_of_this_crate() {
            let filed = the_header_list_by_document(&header, &plan_names);
            let in_the_header: HashSet<String> =
                filed.iter().map(|(_, marker)| marker.clone()).collect();
            for (document, marker) in &filed {
                match document {
                    None => faults.push(format!(
                        "{name}: the header list names {marker} under no document — the list \
                         names each marker with the document it belongs to"
                    )),
                    Some(document) => {
                        let defines = dictionary
                            .iter()
                            .find(|(plan, _)| plan == document)
                            .map(|(_, markers)| markers.contains(marker))
                            .unwrap_or(false);
                        if !defines {
                            faults.push(format!(
                                "{name}: the header list files {marker} under `{document}`, \
                                 which does not define it"
                            ));
                        }
                    }
                }
            }
            for ambiguous in in_the_header.intersection(ours) {
                faults.push(format!(
                    "{name}: the header list names {ambiguous}, which this plan also defines \
                     — write the document out at the site instead"
                ));
            }
            for (marker, beside) in the_markers_in(&body, &plan_names) {
                match beside {
                    Some(document) => {
                        let defines = dictionary
                            .iter()
                            .find(|(plan, _)| *plan == document)
                            .map(|(_, markers)| markers.contains(&marker))
                            .unwrap_or(false);
                        if !defines {
                            faults.push(format!(
                                "{name}: {marker} is written with `{document}`, which does \
                                 not define it"
                            ));
                        }
                    }
                    None => {
                        if !ours.contains(&marker)
                            && !in_the_header.contains(&marker)
                            && anybody_elses.contains(&marker)
                        {
                            faults.push(format!(
                                "{name}: {marker} stands bare and belongs to another document \
                                 — it resolves to a real and wrong clause"
                            ));
                        }
                    }
                }
            }
        }
        assert!(faults.is_empty(), "{faults:#?}");
    }

    /// **The plan gained a clause and this crate said nothing, so this counts
    /// the clauses. Written 2026-09-06.**
    ///
    /// **What happened.** A ruling taken that day added F10b to this crate's
    /// plan and P7 to `Server protocol and transfers`'s, both citing a seam
    /// decision whose carrying out is scheduled separately. `File Browser`'s
    /// clause of the same ruling was met by that crate the same hour, because
    /// its own `arriving_later.rs` carries a **forward-leg test** — it reads
    /// the plan, finds the naming site of every clause, and fails where a
    /// clause is neither named by working code nor recorded as waiting. **This
    /// crate had nothing of the kind and neither did the other**, so both
    /// absences were found by a search a day later.
    ///
    /// **Whether this crate should carry that test was ruled on 2026-09-06
    /// under the Creator's standing delegation of 2026-08-31, and the answer is
    /// no.** The reason is the state of this crate against its own plan. **The
    /// plan defines 394 clauses and 279 of them are named by no working file
    /// here** — not because they are unbuilt in error, but because this crate's
    /// plan is the largest in the project and covers the installer, the update
    /// mechanism, the presentation surface and the whole main window, none of
    /// which any build step has reached. The forward leg requires a written
    /// account of every one, each with what removes it. **Written today that is
    /// 279 entries, most of them a restatement of which step
    /// `Preparation/Build/Plan.md` already says builds them, and this project
    /// has measured what such a record is worth**: the Architecture stage's
    /// closing gates found 108 faults across twenty documents and every single
    /// one was a record that had stopped being true. **A page nobody can keep
    /// true is worse than no page**, because a reader cannot tell it from one
    /// that is.
    ///
    /// **What this test does instead, said narrowly so nobody reports a pass it
    /// did not earn.** It counts. It fails on the day the plan gains or loses a
    /// clause, and sends whoever changed it to look for what the change owes
    /// this crate — which is the exact failure above and the whole of what that
    /// day cost. **It cannot say which clause moved**, and **it cannot see a
    /// clause that has been unbuilt and unrecorded all along**; that is the
    /// build gate's forward leg, run by a fresh agent over the module, which
    /// `Preparation/Build/Method.md` already gives it.
    ///
    /// **What would make the full leg right here is the crate being built**,
    /// and the honest moment to write one is when the last of those steps
    /// lands. Until then this is what stands.
    ///
    /// **The number is [`the_clauses_defined_by`]'s reading and no other's.**
    /// A different reader over the same plan gives a different figure and
    /// neither is wrong; what this needs is one reading that moves when the
    /// plan moves, so comparing this number with anybody else's is a mistake.
    #[test]
    fn the_plans_clause_count_has_not_moved_under_this_crate() {
        let dictionary = the_marker_dictionary();
        let ours = &dictionary
            .iter()
            .find(|(name, _)| name == THIS_PLANS_NAME)
            .expect("this crate's plan is in the dictionary")
            .1;
        // **Moved from 394 to 399 on 2026-09-06, and the five are not one
        // sitting's.** The test did what it is for — it failed, and what it
        // sent somebody to look for was real — so the five are named here
        // rather than absorbed into a number, and so is which of them anybody
        // has acted on.
        //
        // The historical machine-identity reader and its limitation were
        // removed by the free-software plan tidy on 2026-10-27.
        //
        // **The other four were checked against the code on 2026-09-06, later
        // the same day, and the line that said they were unchecked is gone.**
        // They are the re-readings against the selected shell that this plan's
        // own sentences promised. Where each one landed:
        //
        // - **H31a** — built. Its two refusals are closures the window
        //   construction installs rather than a configuration entry, and a
        //   build that installs neither keeps neither. They are at
        //   [`crate::web_content_boundary`], with the test that says they are
        //   installed and the table that says the predicate admits the
        //   packaged origin alone.
        // - **H10a** and **H16a** — the three legs of them that settle a
        //   standing property of the shipped configuration or of the
        //   workspace's pin are named at `the_selected_shell.rs`.
        // - **E4a** — owed at the first Linux release and not before, because
        //   no Linux package is produced yet. It stands at
        //   [`crate::arriving_later`] with what removes it.
        //
        // **The count itself was derived rather than read off this test's own
        // failure, on 2026-09-06.** The plan carries **403** definition-shaped
        // lines and **399** distinct markers; the four repeats are H4, P10, R14
        // and Z22, each stated twice — H4 by its own closure line sitting above
        // the clause it closes, the other three by a later bullet restating the
        // clause it belongs to. **A set is the right reading**, because the
        // unit is the clause and not the sentence, which is why this compares
        // against the set's size. 399 is what that derivation gives and it is
        // the figure the test already carried.
        //
        // **Moved from 399 to 400 on 2026-09-18, and this test did its job at
        // the cost of a red tree.** Step 10b piece 6 added **L12a** — a
        // migration never empties the local job record — to §5.2, on the build
        // runner's ruling, and did not move this number with it. **The commit
        // that carried it says the desk was green and it was not**, this test
        // having failed from the moment the clause was written. **The clause is
        // built**: `src/record/schema.rs`'s
        // `TABLES_A_MIGRATION_NEVER_EMPTIES` names the job record, which is
        // what that piece added it for. Found by step 10b piece 8, which
        // touched neither the plan nor that table.
        //
        // **Moved from 400 to 408 on 2026-09-28, once, for the whole of step
        // 10c's transfer wave**, by that wave's piece 8 and by no other piece
        // — the wave's map named one mover so the tree went red once and green
        // once. **The eight are two rulings of the Creator's, both of that
        // day.** Row 518, that Smoozy does not update itself while jobs run,
        // added **Z31** and **Z32**; Row 519, that servers follow the app's
        // release, added **V11** to **V16**. Where each landed:
        //
        // - **V11, V12, V14, and V13's *too old* and *too new*** — built by
        //   piece 8: `src/servers/the_machines_work.rs`'s step 9
        //   (`the_step_nine`, and V12's re-try at a queue reading),
        //   `src/servers/installation.rs`'s inspection and the fourth branch
        //   that no longer reads another protocol as Smoozy not responding,
        //   `src/servers/published.rs`'s five conditions, and
        //   `src/setting_up.rs`'s own-release address, refusal of a newer
        //   machine and the update's placement.
        // - **V13's row words and V15** — the next piece's (the wave's piece
        //   9), which draws the five lines and the *Update now…* box from the
        //   conditions piece 8 raises.
        // - **V16** is what was weighed and not taken, and has no code.
        // - **Z31 and Z32** are step 11's, the application's own updater —
        //   built on the Mac at step 11's Mac update piece (2026-10-25):
        //   `src/updating/the_hold.rs` and `src/updating/the_mac.rs`, with
        //   About's *Install now*.
        //
        // **Moved from 408 to 409 on 2026-10-03, on the build runner's word**,
        // by the builder of `Audits/Build/Step 10c/Brief — after the Vast
        // fixes, the whole desk, four rulings and the slow send.md`. A plan
        // pass that day added **K25** — one service opens the licence's *buy*
        // and *recover* addresses in the user's own browser, and no others
        // (§18.7). **No code names K25 when this number moves**: the count
        // follows the plan, and the clause is owed to whichever piece builds
        // the address-opening service.
        //
        // **Moved from 409 to 411 on 2026-10-24**, by the build runner, after
        // step 11's plan pass (`Audits/Build/Step 11/` — its first plan pass and the
        // drawing, 2026-10-24) gave this plan **E10a** — where the
        // desktop finds its signed update file — and **E4b**, the release
        // matrix's rows. **Both owed**: E4b to step 11's artifacts piece, E10a
        // to its update-reading and update-publishing pieces.
        // D7b's dated corrections add one distinct definition-shaped marker
        // on 2026-10-26. Recounted independently: 411 before, 412 after,
        // D7b alone added and no marker removed. Its Windows update rule is
        // implemented by the separate D7b builder; the count remains exact.
        // Wave 5 removes three machine-identity definitions from the plan.
        // Read all direct plan definitions with the_clauses_defined_by, then
        // deduplicate as the dictionary does: 412 − 3 = 409.
        // **Moved from 409 to 410 on 2026-11-01**, by the build runner, after
        // the Windows signing plan pass gave this plan **E7a** (the
        // certificate is SignPath Foundation's). **Owed** to step 11, once
        // SignPath accepts the project.
        assert_eq!(ours.len(), 410);
    }

    /// **A document's name governs the marker immediately after it and no
    /// further**, so every later marker in the same run is read as that
    /// document's and resolves to the wrong clause.
    ///
    /// Returns each run of markers that follows a backticked plan name, as
    /// (file, plan, markers in the run after the first).
    ///
    /// **A run is markers joined by nothing but separators** — a space, a
    /// comma, `and`, `or`, a dash, `to`. Anything else ends it, so a marker
    /// in the next sentence is not swept up and a bolded run that closes
    /// after the first marker is not read past.
    fn the_runs_after_a_document_name(
        text: &str,
        plan_names: &[String],
    ) -> Vec<(String, Vec<String>)> {
        let bytes = text.as_bytes();
        let mut out = Vec::new();
        let mut at = 0;
        while at < bytes.len() {
            if bytes[at] != b'`' {
                at += 1;
                continue;
            }
            let Some(closes) = text[at + 1..].find('`').map(|found| at + 1 + found) else {
                break;
            };
            let named = text[at + 1..closes].to_string();
            at = closes + 1;
            if !plan_names.contains(&named) {
                continue;
            }
            // The run: marker, separator, marker, separator, … Anything that is
            // neither ends it.
            let mut run: Vec<String> = Vec::new();
            let mut rest = &text[at..];
            loop {
                let rest_trimmed = rest.trim_start_matches(' ');
                let word: String = rest_trimmed
                    .chars()
                    .take_while(char::is_ascii_alphanumeric)
                    .collect();
                if word.is_empty() || !looks_like_a_marker(&word) {
                    break;
                }
                let mut after = &rest_trimmed[word.len()..];
                // A marker inside a name with an underscore in it is not a
                // citation, and the outer guard already skips one.
                if after.starts_with('_') {
                    break;
                }
                run.push(word);
                // A marker's own possessive is part of the citation.
                after = after.strip_prefix("'s").unwrap_or(after);
                // Exactly the separators a list of citations is written with.
                // Anything else — a full stop, a bold run closing, a word —
                // ends the run.
                let stepped = after
                    .strip_prefix(',')
                    .unwrap_or(after)
                    .trim_start_matches(' ');
                let stepped = ["and ", "or ", "to ", "– ", "- "]
                    .iter()
                    .find_map(|joiner| stepped.strip_prefix(joiner))
                    .unwrap_or(stepped);
                let separated = stepped.len() < after.len();
                if !separated {
                    break;
                }
                rest = stepped;
            }
            if run.len() > 1 {
                out.push((named, run[1..].to_vec()));
            }
        }
        out
    }

    /// **The neighbour rule, held rather than remembered: a document's name
    /// governs the marker immediately after it and lends itself to no
    /// other.**
    ///
    /// This is the class the guard above cannot see, named in its own list of
    /// limits: *"it cannot tell a bare marker that means this plan's clause
    /// from one that means a colliding clause of a document named earlier in
    /// the same sentence"*. On 2026-09-04 two citations in one crate each
    /// paired a plan's name with two markers, and in both pairs the second
    /// was one that plan does not define. The guard above forgave both: it
    /// read each second marker as bare, and a bare marker this plan defines
    /// is what it allows. The eye, meanwhile, takes the name as governing
    /// both. **No example is written out here on purpose** — a real plan's
    /// name beside a marker it does not define is the very fault this leg
    /// reports, and every leg reads this file too.
    ///
    /// **A marker in such a run that the named document does not define is a
    /// fault whatever it means**, and the fix is always the same: write the
    /// document out at the site, this plan's own name included.
    ///
    /// **What it cannot see.** A run that the named document *does* define
    /// every marker of is passed, which is right; and a marker separated from
    /// the run by anything but a plain separator is not in the run at all, so
    /// a citation spread across a clause is not reached.
    #[test]
    fn a_document_name_governs_only_the_run_it_opens_and_lends_itself_to_none() {
        let dictionary = the_marker_dictionary();
        let plan_names: Vec<String> = dictionary.iter().map(|(name, _)| name.clone()).collect();

        let mut faults = Vec::new();
        for (name, body, header) in every_file_of_this_crate() {
            for text in [&body, &header] {
                for (document, run) in the_runs_after_a_document_name(text, &plan_names) {
                    let defines = dictionary
                        .iter()
                        .find(|(plan, _)| *plan == document)
                        .map(|(_, markers)| markers.clone())
                        .unwrap_or_default();
                    for marker in run {
                        if !defines.contains(&marker) {
                            faults.push(format!(
                                "{name}: {marker} stands in a run opened by `{document}`, \
                                 which does not define it — the name governs only the marker \
                                 after it, so write this one's document out"
                            ));
                        }
                    }
                }
            }
        }
        assert!(faults.is_empty(), "{faults:#?}");
    }
}
