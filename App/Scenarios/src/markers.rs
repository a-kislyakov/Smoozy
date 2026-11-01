//! The marker guard: a clause marker written here resolves to the clause it
//! means, and never to a different document's clause of the same name.
//!
//! **This crate has no plan of its own**, so there is no bare marker that
//! could mean one: **every marker is written with the document it belongs
//! to**, at the site, or in that file's own header list of the markers in it
//! and the documents they belong to. The same short marker is defined by many
//! plans at once, so a bare one does not fail to resolve: **it resolves to a
//! real and wrong clause, which nobody reading it can tell from a right one.**
//!
//! This is the guard every crate of the workspace carries — the shape settled
//! on 2026-09-04 in `App/Server Setup/` and `App/Rented-server runtime/` —
//! with the one change a crate with no plan needs: the legs that ask what
//! *this* plan defines ask nothing, and a bare marker any plan defines is a
//! fault.
//!
//! **What is counted exactly, and why it is not the whole crate.** The files
//! of `src/` are counted exactly, as every crate's are. **The scenario tests
//! under `tests/` are read whole by the same walk but not counted**, because
//! each one has its own owner and two of them are written at once; one shared
//! number would put those two writers on the same line of this file, which is
//! the fight the crate was made to avoid. The walk recurses, so a test file in
//! a new folder is read all the same.

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::fs;
    use std::path::{Path, PathBuf};

    /// **Every file of `src/`, counted exactly.** See
    /// [`every_file_of_this_crate`] for why the number is here rather than a
    /// floor, and the head of this file for why `tests/` is not in it.
    const THE_SOURCE_FILE_COUNT: usize = 3;

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
    /// **The same reading is applied to every plan**, so a marker's document
    /// is decided one way rather than two.
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
        assert_eq!(
            every_rust_file_under(&root.join("src")).len(),
            THE_SOURCE_FILE_COUNT,
            "the files of src/ were not all found, or one was added or removed \
             without this count moving with it"
        );
        let read = every_rust_file_under(root);
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
    /// **A marker with nothing backticked before it is a bare one**, and in
    /// this crate a bare marker means no plan at all.
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
    /// 2026-09-03 having survived five gate runs. **Every earlier fix was a
    /// habit somebody had to remember at the moment the comment was typed.
    /// This one does not depend on anybody remembering.**
    ///
    /// Four things fail here, over every file of this crate.
    ///
    /// - **A bare marker that any plan defines**, unless the file's own header
    ///   list names it with its document. This crate has no plan, so a bare
    ///   marker can mean nothing of its own.
    /// - **A marker written with a document that does not define it.** That is
    ///   the wrongly-attributed case, and it reads exactly like a right one.
    /// - **A header list naming a marker the document it is filed under does
    ///   not define.** Without this the list is an unchecked whitelist.
    /// - **A header entry filed under no document at all**, which the list's
    ///   own heading forbids.
    ///
    /// **What it cannot see, said rather than glossed.**
    ///
    /// - **The scan is line-based and compiles nothing.** A marker inside a
    ///   name with an underscore in it is skipped, and so is one reached
    ///   through a link rather than written out.
    /// - **A capital followed by a digit is not taken as a marker's shape**,
    ///   because that matches a version number inside a derived path.
    /// - **A bare marker-shaped word that no plan defines anywhere is not a
    ///   fault**, because it is not a marker: `MD5` has that shape and is the
    ///   name of a digest.
    /// - **It decides what a document defines by one reading applied
    ///   everywhere**, so a clause stated in a shape [`the_clauses_defined_by`]
    ///   does not catch is invisible to it in both directions.
    #[test]
    fn no_marker_stands_bare_and_none_is_wrongly_attributed() {
        let dictionary = the_marker_dictionary();
        let plan_names: Vec<String> = dictionary.iter().map(|(name, _)| name.clone()).collect();
        let anybodys: HashSet<&String> = dictionary
            .iter()
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
                        if !in_the_header.contains(&marker) && anybodys.contains(&marker) {
                            faults.push(format!(
                                "{name}: {marker} stands bare, and this crate has no plan for \
                                 it to mean — it resolves to a real and wrong clause"
                            ));
                        }
                    }
                }
            }
        }
        assert!(faults.is_empty(), "{faults:#?}");
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
    /// This is the class a crate's own guard could not see. On 2026-09-04 two
    /// citations in one crate each paired a plan's name with two markers, and
    /// in both pairs the second was one that plan does not define. That
    /// crate's guard read each second marker as bare, and a bare marker of
    /// its own plan is what it allows. The eye, meanwhile, takes the name as
    /// governing both. **No example is written out here on purpose** — a real plan's
    /// name beside a marker it does not define is the very fault this leg
    /// reports, and every leg reads this file too.
    ///
    /// **A marker in such a run that the named document does not define is a
    /// fault whatever it means**, and the fix is always the same: write the
    /// document out at the site.
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
