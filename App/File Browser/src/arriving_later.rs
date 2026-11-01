//! What of this document is not built here — and from 2026-09-06 the answer is
//! one clause, L1b.
//!
//! **This page exists so that a gap is met by a reader of this crate rather
//! than discovered.** It follows the shape `Server protocol and transfers` and
//! `Job files and outputs` use in their own `arriving_later.rs`. **It has
//! carried four lists and every one of them was emptied by the work landing.**
//! [`WAITING_ON_THE_PATH_SEAM`] is the fifth, it holds one entry, and it is
//! written in the same shape the four before it used: the plan's section, the
//! clauses, what the clause is, why no code here keeps it, and what removes it.
//!
//! **The first was the acting half of step 5** — §3, §5, §6, §7 and §9,
//! sixty-six clauses — built at [`crate::request`], [`crate::uploading`],
//! [`crate::pool`], [`crate::managing`] and [`crate::continuation`], with their
//! real ends over the file-transfer subsystem at [`crate::wiring`].
//!
//! **The second was the readings that reached no machine.** It held five at the
//! start of 2026-09-04. **Four were wired that day**: [`crate::wiring`]'s
//! `TheReadingChannel` is the real end of the listing (L1), the already-exists
//! walk's machine side (X1), the version tag's prefix (L7) and the permission
//! walk's component reading (P3), all four over the file-transfer subsystem on
//! one channel value. **What that cost is written at the traits themselves** —
//! each of the four is asynchronous now, because a machine is at the other end
//! of a connection, so the three functions above them take their reader as a
//! type parameter rather than as `dyn`. **The fifth was P5's elevation reading,
//! and it got its end when `Server Setup` was built**, at that crate's
//! `elevation::WhatElevationWouldDo` — the assignment §12 makes, mapping that
//! document's own kind table onto P5's three rows rather than standing a
//! machine up.
//!
//! **The third and fourth were the two pieces `Preparation/Build/Plan.md` held
//! back from step 5 to step 7 — §8's whole warning layer, W1 to W10, and X7a's
//! two outward writes.** They were held back because they read and write
//! records `Dependency Upload & Remap`, `Queue & History` and `Render Settings`
//! own, and none of those crates existed. **All three landed on 2026-09-04 and
//! so did these**: §8 is [`crate::warnings`] and X7a is [`crate::publication`],
//! with their real ends over the neighbouring records in `Desktop app and
//! delivery`. **The two things this page said were waiting for them are joined
//! up rather than duplicated** — the diary line's job field
//! ([`crate::diary::WhatTheFileActLineNames::jobs_the_telling_named`]) and W8's
//! ordered pair in [`crate::declarations::THE_PARTITION_UNITS`].
//!
//! **Each list was removed rather than left standing empty**, and that is one
//! decision taken four times: a page that records an absence has to be read as
//! the whole truth about absences, and an empty list is a claim nobody can tell
//! apart from a list nobody kept. **The other side of that decision is this
//! one: an absence brings a list back.**
//!
//! **The fifth list stands because the plan gained L1b on 2026-09-06.** That
//! clause carries a ruling taken once for the whole workspace at
//! `Server protocol and transfers` P7 — a path is bytes on the machine side and
//! the receiving system's own path unit on the desktop side, and text nowhere
//! but where a name is shown. **Nothing in this crate keeps it**, and the three
//! text-taking reading traits and the crossing into the transfer record are
//! where it is not kept. The carrying out is scheduled and is not this page's,
//! and [`WAITING_ON_THE_PATH_SEAM`] names it and says what it moves.
//!
//! **What guards it is two tests and neither reads this page's prose.**
//!
//! - **Every clause the plan defines is either named by a working file of this
//!   crate or stands on [`WAITING_ON_THE_PATH_SEAM`] with what removes it**,
//!   and the naming site is found rather than inferred from a section number:
//!   until 2026-09-04 a clause counted as built when its section was in
//!   [`THE_SECTIONS_BUILT_HERE`], which is a tautology for a built section and
//!   would have passed with a working file deleted whole.
//! - **Every trait this crate declares is implemented somewhere in the
//!   application outside test code**, and it is real work: a reading added here
//!   and left unwired fails it, and so does a wiring taken out next door.

/// **What the three reading traits take, and it was met at the wiring and left
/// where it was.** Each takes its path as text, while the subsystem's own calls
/// take bytes and this crate carries names as bytes everywhere it owns the value
/// (N2, L1a). **That is one seam for the whole workspace to decide once**, and it
/// is parked at `Preparation/Groundwork/Open Threads.md` rather than settled
/// here.
///
/// **What the wiring of 2026-09-04 does with it, said plainly.**
/// [`crate::wiring`] hands the text on as its bytes, which loses nothing in that
/// direction; **what stands is that a path whose bytes are not valid text
/// cannot be named to any of the three at all**, so such a folder cannot be
/// listed, walked or version-tagged. That is the same cost the upload side
/// already carries and it is the reason the seam is routed rather than fixed
/// three traits at a time.
///
/// **The routed question was answered on 2026-09-06, and the answer is not here
/// either.** `Server protocol and transfers` P7 rules it for the whole
/// workspace and this crate's plan cites that ruling at L1b. **The code has not
/// moved**, so everything above still describes what this crate does today;
/// what changed is that the seam is settled rather than parked, and the entry in
/// [`WAITING_ON_THE_PATH_SEAM`] holds the carrying out.
pub const THE_READING_TRAITS_TAKE_PATHS_AS_TEXT_AND_THAT_IS_A_ROUTED_QUESTION: bool = true;

/// One clause of this plan that no working file of this crate names, with what
/// removes it.
///
/// **This is the shape the four earlier lists used**, with the step field
/// dropped: what removes this one is not a step of `Preparation/Build/Plan.md`
/// but a piece of scheduled work with its own work list, and naming a step would
/// send whoever builds that step looking for something that is not filed there.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotNamedByWorkingCode {
    /// The plan's own top-level section.
    pub section: &'static str,
    /// Every clause of that section this covers.
    pub clauses: &'static [&'static str],
    /// What the clause is.
    pub what: &'static str,
    /// Why no working file of this crate names it.
    pub why: &'static str,
    /// **What removes it** — the work that makes the clause true here, named
    /// exactly enough that a reader can go and find it.
    pub what_removes_it: &'static str,
}

/// **The one clause waiting on the path seam.**
///
/// **Added 2026-09-06, the day the plan gained L1b.** The clause was added and
/// nothing in this crate accounted for it, which is what the forward-leg test
/// said; this entry is the account.
pub const WAITING_ON_THE_PATH_SEAM: [NotNamedByWorkingCode; 1] = [NotNamedByWorkingCode {
    section: "§1",
    clauses: &["L1b"],
    what: "the rule for what a machine-side name's bytes become when they cross to the transfer surface. **L1b states no mechanism of its own**: it cites `Server protocol and transfers` P7 — bytes on the machine side, the receiving system's own path unit on the desktop side, text only where a name is shown — names `Desktop app and delivery` C3's name check as the single place one becomes the other, and says outright that this document restates none of it and holds no second version of it. What it binds here is that this crate's paths cross as bytes",
    why: "**this crate does the opposite today, in two places, and naming L1b at either would record a promise as kept where the code contradicts it.** The three reading traits take their path as text — `ReadsADirectory` at [`crate::listing`], `CanListAComponent` at [`crate::permission`] and `ReadsAPrefix` at [`crate::blend_version`] — and [`crate::wiring`] hands the text on as its bytes, so a folder whose name is not valid text cannot be listed, walked or version-tagged at all. And the crossing into the transfer record at [`crate::request`] turns the source and landing bytes into text with `String::from_utf8` and refuses the name where it is not text, which is the pre-check at [`crate::uploading::check_the_name_can_be_carried`]. **L1a, N2 and X1 are the same rule seen from this crate's own side and they are kept**, which is why the absence is L1b alone rather than a section",
    what_removes_it: "**the carrying out of the ruling, whose work list is `Audits/Build/Seam decisions/Paths across the transfer seam — the work list.md`.** That list's `File Browser` section is what moves here: the three reading traits take bytes, which deletes the three conversions in [`crate::wiring`]; their consumers move with them — the permission walk and its refused-at value, and the version tag's read and its path field; and the crossing at [`crate::request`] hands the bytes to the transfer record as bytes, which deletes both `String::from_utf8` calls, the pre-check at [`crate::uploading`] and the not-valid-text failure they raise, and turns the comparison back the other way into a byte comparison. **It cannot land here first**: that list's own order puts the transfer record and the local-computer surface in `Server protocol and transfers` first and the durable store in `Desktop app and delivery` second, because the record's path fields are what this crate hands to. **When it lands, L1b is named at the crossing**, and the two markers the work list says to write in the code are `Server protocol and transfers` P7 and, where a name is shown, L1a",
}];

/// The sections this crate builds, by their top-level number.
///
/// **It is a reader's map and not the accounting.** What says nothing has
/// fallen between this page and the code is the test below, which goes and
/// finds the naming site of every clause; a section number standing in for that
/// would pass whether or not a line of code existed.
///
/// **All eleven**, since 2026-09-04, when §8 and X7a landed.
pub const THE_SECTIONS_BUILT_HERE: [&str; 11] =
    ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11"];

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use super::*;

    const THE_PLAN: &str = include_str!("../../../Preparation/Architecture/File Browser.md");

    /// Every clause the plan defines, as (top-level section number, marker), in
    /// the plan's own order.
    fn the_clauses_the_plan_defines() -> Vec<(String, String)> {
        let mut heading = "";
        let mut out = Vec::new();
        for line in THE_PLAN.lines() {
            if let Some(rest) = line.strip_prefix("### ") {
                heading = rest;
            } else if let Some(rest) = line.strip_prefix("## ") {
                heading = rest;
            }
            let number = heading
                .split_whitespace()
                .next()
                .unwrap_or_default()
                .trim_end_matches('.');
            let top = number.split('.').next().unwrap_or_default();
            if top.is_empty() || !top.chars().all(|c| c.is_ascii_digit()) {
                continue;
            }
            let body = line
                .trim_start()
                .trim_start_matches("- ")
                .trim_start_matches("* ");
            let Some(rest) = body.strip_prefix("**") else {
                continue;
            };
            let Some(marker) = rest.split('.').next() else {
                continue;
            };
            let looks_like_a_marker = marker.starts_with(char::is_uppercase)
                && marker.chars().any(|c| c.is_ascii_digit())
                && marker
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '-');
            if looks_like_a_marker {
                out.push((top.to_string(), marker.to_string()));
            }
        }
        out
    }

    /// **The fifteen clauses step 5 held back are each named by working code,
    /// and this says so by marker rather than by section.**
    ///
    /// §8's fourteen and X7a were this page's whole list until 2026-09-04. The
    /// list is gone because they are built; **this test is what stops that
    /// removal being a claim nobody checked.**
    #[test]
    fn the_fifteen_clauses_held_back_to_step_seven_are_each_named_by_working_code() {
        let named = the_clauses_working_code_names();
        let held_back = [
            "W1", "W1a", "W1b", "W2", "W3", "W4", "W5", "W5a", "W5b", "W6", "W7", "W8", "W9",
            "W10", "X7a",
        ];
        assert_eq!(held_back.len(), 15);
        for clause in held_back {
            assert!(
                named.iter().any(|(found, _)| found == clause),
                "{clause} was held back to step 7 and no working file of this crate names it"
            );
        }
    }

    /// The other plans, by name, so that a marker written with one of them
    /// beside it is not read as a clause of this one.
    fn the_other_plans() -> Vec<String> {
        let plans = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../Preparation/Architecture");
        let mut out = Vec::new();
        for entry in fs::read_dir(&plans).expect("the Architecture folder is readable") {
            let path = entry.expect("the entry is readable").path();
            if path.extension().and_then(|e| e.to_str()) != Some("md") {
                continue;
            }
            let name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .expect("the plan has a name")
                .to_string();
            if name != "File Browser" {
                out.push(name);
            }
        }
        assert!(
            out.len() > 15,
            "the other plans were not found, so nothing below would be excluded"
        );
        out
    }

    /// Every working file of this crate, as (file name, its text with the
    /// header list of other documents' markers taken out).
    ///
    /// **[`arriving_later`](self), `lib.rs` and `markers.rs` are not working
    /// files** — this page is the record of what is *not* built, `lib.rs` is the
    /// crate's front matter, and `markers.rs` is the marker guard, which keeps
    /// no clause and whose prose names markers only to say how one is written.
    /// A clause named only in one of those is a clause nothing keeps, so none of
    /// them may stand in for code. **Leaving `markers.rs` in would have made
    /// this test weaker rather than stronger**: a clause named nowhere but in
    /// the guard's own account of itself would have counted as built.
    fn the_working_files() -> Vec<(String, String)> {
        let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut out = Vec::new();
        for entry in fs::read_dir(&src).expect("the source folder is readable") {
            let path = entry.expect("the entry is readable").path();
            if path.extension().and_then(|e| e.to_str()) != Some("rs") {
                continue;
            }
            let name = path
                .file_name()
                .and_then(|s| s.to_str())
                .expect("the file has a name")
                .to_string();
            if name == "arriving_later.rs" || name == "lib.rs" || name == "markers.rs" {
                continue;
            }
            let text = fs::read_to_string(&path).expect("the file is readable");
            out.push((name, without_the_header_list(&text)));
        }
        // Twenty from 2026-10-31, when `picture.rs` brought L29 (the
        // Creator's seventeenth notes, note 11).
        assert_eq!(
            out.len(),
            20,
            "the working files of this crate were not all found"
        );
        out
    }

    /// The file's text as one run, with the comment markers taken off so a
    /// citation that wrapped over a line break reads as one sentence, and with
    /// the header list of other documents' markers removed — that list names
    /// other documents' clauses and never keeps one.
    fn without_the_header_list(text: &str) -> String {
        let mut out = String::new();
        let mut in_the_header_list = false;
        for line in text.lines() {
            let line = line.trim();
            if line.contains("Markers used") {
                in_the_header_list = true;
            }
            if in_the_header_list {
                if line.starts_with("//!") {
                    continue;
                }
                in_the_header_list = false;
            }
            let line = line
                .strip_prefix("///")
                .or_else(|| line.strip_prefix("//!"))
                .or_else(|| line.strip_prefix("//"))
                .unwrap_or(line);
            out.push_str(line);
            out.push(' ');
        }
        out
    }

    /// Whether the marker at `at` is written with no document beside it —
    /// which, by the rule this crate states in `lib.rs`, is what makes it a
    /// clause of this crate's own plan.
    fn no_document_stands_beside_it(text: &str, at: usize, other_plans: &[String]) -> bool {
        let before = text[..at]
            .trim_end()
            .trim_end_matches(|c: char| "*([{\"'".contains(c))
            .trim_end();
        let Some(inner) = before.strip_suffix('`') else {
            return true;
        };
        match inner.rfind('`') {
            Some(opens) => !other_plans.iter().any(|plan| plan == &inner[opens + 1..]),
            None => true,
        }
    }

    /// Every clause of this plan that working code names, found rather than
    /// assumed.
    fn the_clauses_working_code_names() -> Vec<(String, String)> {
        let other_plans = the_other_plans();
        let mut out = Vec::new();
        for (name, text) in the_working_files() {
            let bytes = text.as_bytes();
            let mut at = 0;
            while at < bytes.len() {
                let starts_a_word = bytes[at].is_ascii_uppercase()
                    && (at == 0
                        || !(bytes[at - 1].is_ascii_alphanumeric() || bytes[at - 1] == b'_'));
                if !starts_a_word {
                    at += 1;
                    continue;
                }
                let mut end = at;
                while end < bytes.len() && bytes[end].is_ascii_alphanumeric() {
                    end += 1;
                }
                if !(end < bytes.len() && bytes[end] == b'_')
                    && no_document_stands_beside_it(&text, at, &other_plans)
                {
                    out.push((text[at..end].to_string(), name.clone()));
                }
                at = end.max(at + 1);
            }
        }
        out
    }

    #[test]
    fn working_code_names_every_clause_the_plan_defines() {
        // This is the forward leg of the gate, run over this one document at
        // the desk: a clause no working file names is a promise nobody built.
        // **The naming site is found, not inferred from the section number** —
        // inferring it would pass for every clause of a built section whether
        // or not any code existed, and this test would survive a working file
        // being deleted whole.
        //
        // **A clause on `WAITING_ON_THE_PATH_SEAM` is accounted for by that
        // entry instead**, which is the shape this test carried until
        // 2026-09-04, when the last list emptied and the second limb went with
        // it. The limb is back because the plan gained a clause nothing here
        // keeps. It is not an escape: the entry is read against the plan by the
        // test below, an entry with no *what removes it* fails there, and a
        // clause that is neither built nor recorded still fails here.
        let recorded: Vec<&str> = WAITING_ON_THE_PATH_SEAM
            .iter()
            .flat_map(|part| part.clauses.iter().copied())
            .collect();
        let named = the_clauses_working_code_names();

        let mut unaccounted = Vec::new();
        for (section, marker) in the_clauses_the_plan_defines() {
            if recorded.contains(&marker.as_str()) {
                continue;
            }
            if named.iter().any(|(found, _)| found == &marker) {
                continue;
            }
            unaccounted.push(format!("§{section} {marker}"));
        }
        assert!(
            unaccounted.is_empty(),
            "these clauses are named by no working file of this crate and are not \
             recorded as waiting: {unaccounted:?}"
        );
    }

    #[test]
    fn every_recorded_absence_is_a_clause_of_the_section_it_is_filed_under() {
        // The markers on this page are written as text rather than in comments,
        // so no sweep of the comments reaches them. This is what reaches them:
        // the plan is asked, clause by clause, and the section an entry files a
        // clause under has to be the section the plan defines it in. Each entry
        // is also made to say what the clause is, why nothing here names it, and
        // what removes it — an absence with no way out of it is a gap wearing a
        // record's clothes.
        let defined = the_clauses_the_plan_defines();
        let named = the_clauses_working_code_names();
        let mut checked = 0;
        for part in WAITING_ON_THE_PATH_SEAM {
            assert!(!part.clauses.is_empty());
            assert!(!part.what.is_empty());
            assert!(!part.why.is_empty());
            assert!(!part.what_removes_it.is_empty());
            let filed_under = part.section.trim_start_matches('§');
            for clause in part.clauses {
                let sits_in = defined
                    .iter()
                    .find(|(_, marker)| marker == clause)
                    .map(|(section, _)| section.as_str())
                    .unwrap_or_else(|| panic!("the plan defines no {clause}"));
                assert_eq!(
                    sits_in, filed_under,
                    "{clause} is filed under {} and the plan defines it in §{sits_in}",
                    part.section
                );
                // A recorded absence that working code also names would let the
                // record and the code say different things with nothing to
                // catch it.
                assert!(
                    !named.iter().any(|(found, _)| found == clause),
                    "{clause} is recorded as waiting and a working file names it"
                );
                checked += 1;
            }
        }
        assert_eq!(checked, 1);
    }

    #[test]
    fn the_plans_clause_count_has_not_moved_under_this_page() {
        // 136 when this was written; 137 from 2026-09-04, when §8.4 gained W11;
        // 138 from 2026-09-06, when §1 gained L1b and the path seam's ruling
        // reached this document; 143 from 2026-10-02, when §1.6 gained L19 to
        // L23 for *Show hidden files* (Row 520); 148 from 2026-10-30, when §1.7
        // gained L24 to L28 for a folder's size; 149 from 2026-10-31, when §1.8
        // gained L29 for a picture shown in Files. A change to the plan that adds
        // a clause fails here, which is what sends whoever made it to the list
        // above.
        assert_eq!(the_clauses_the_plan_defines().len(), 149);
    }

    #[test]
    fn the_readers_map_holds_every_section_the_plan_has() {
        // **All eleven**, and the map is checked against the plan's own
        // headings rather than against itself.
        let mut sections: Vec<String> = the_clauses_the_plan_defines()
            .into_iter()
            .map(|(section, _)| section)
            .collect();
        sections.sort();
        sections.dedup();
        let mut built: Vec<String> = THE_SECTIONS_BUILT_HERE
            .iter()
            .map(|section| (*section).to_string())
            .collect();
        built.sort();
        assert_eq!(sections, built);
    }

    /// **Every reading this crate declares is implemented somewhere in the
    /// application outside test code.**
    ///
    /// **This is the half of the old test that had a subject.** Until
    /// 2026-09-04 this page also carried a list of readings that reached no
    /// machine, and the test's other half checked that each name on it really
    /// was implemented nowhere. **That list is gone** — its last entry was P5's
    /// elevation reading, and `Server Setup` implements it — so that half had
    /// nothing left to read and went with it. **What is left is not a
    /// weaker question but the opposite one**, and it is the one that fails when
    /// a seam is opened and left hanging.
    ///
    /// **The claim is about the whole application, so the test reads the whole
    /// application.** Until 2026-09-04 it read this crate's own source folder
    /// and nothing else, and carried a hand-kept list of two names to stop it
    /// failing over the traits answered next door. That list read nothing and
    /// proved nothing, and a `File Browser` seam answered in another crate is
    /// the ordinary way one gets its end rather than a corner — **four of this
    /// crate's twelve traits are answered in `Desktop app and delivery`, one in
    /// `Server Setup`, and a sixth has a second end in `Download` today**.
    /// *(Corrected 2026-09-07: this read two of ten and one in `Server Setup`.
    /// The figures had moved with the work; the test below never read them, so
    /// nothing failed when they went stale, which is the whole reason a count
    /// in prose beside a counting test is worth re-reading.)* The hand-kept
    /// list is gone because the reading replaces it.
    ///
    /// **What it reads**: every `.rs` file under the `src` tree of every member
    /// named by the workspace manifest at `App/Cargo.toml`, this crate
    /// included. The member list is read rather than written here, so a member
    /// added later is read without anybody remembering to add it, and every
    /// member is asserted to have contributed at least one file — a walk that
    /// silently found nothing fails here rather than passing narrowly.
    ///
    /// **What it cannot see, said rather than glossed.**
    ///
    /// - **It reads text and does not compile anything.** An implementation
    ///   written by a macro, or one whose trait is named through an alias or a
    ///   renaming import, is not found. A trait of some other crate that happens
    ///   to share one of these names would be found and counted as an end for
    ///   this crate's, which nothing here can tell apart.
    /// - **The two lines have to be one line.** A declaration is read only where
    ///   `impl` and the trait it implements sit on the same line, so an
    ///   implementation wrapped across two lines is invisible and this test
    ///   passes without it. That has happened once already, in a first draft
    ///   next door, and a test below is what holds the reader to it.
    /// - **`#[cfg(test)]` is taken as beginning test code and running to the end
    ///   of the file**, which is where every file in this workspace puts it. A
    ///   real implementation written after such a line would be read as test
    ///   code.
    /// - **A crate that is not a workspace member is not read**, and that is
    ///   deliberate: what is not built is not part of the application. So a
    ///   reading whose only end sat outside the workspace would fail here, which
    ///   is right — the application cannot reach it either.
    #[test]
    fn the_readings_this_crate_declares_all_have_a_real_end() {
        let mut declared = Vec::new();
        for (_, text) in the_working_files_as_written() {
            for line in text.lines() {
                if let Some(rest) = line.trim_start().strip_prefix("pub trait ") {
                    let name = rest.split_whitespace().next().unwrap_or_default();
                    declared.push(name.to_string());
                }
            }
        }
        assert!(
            declared.len() >= 10,
            "the traits of this crate were not found"
        );

        let implemented = what_the_application_implements();

        for trait_name in &declared {
            assert!(
                implemented.iter().any(|(held, _)| held == trait_name),
                "{trait_name} is a reading this crate declares and nothing in the \
                 application implements it, so no caller can reach a machine through it"
            );
        }
    }

    /// **The reading above is line-based, and this is what says so out loud.**
    ///
    /// [`what_the_application_implements`](self::tests::what_the_application_implements)
    /// only sees an implementation whose `impl` and whose trait name share one
    /// line. **Wrapping them over two lines makes the implementation invisible
    /// to it**, which was checked by doing exactly that on 2026-09-04. That is
    /// the one way this mechanism fails quietly: a record of readings with no
    /// real end stays green while an implementation it denies sits in the tree
    /// wrapped. So the wrapped form is written out here and the reader is held to
    /// missing it — the day that stops being true, this test fails and whoever
    /// changed the reading is told what else changed with it.
    #[test]
    fn the_reading_misses_a_declaration_wrapped_over_two_lines() {
        let one_line = "impl Something for AThing {";
        let wrapped = "impl Something\n    for AThing\n{";
        assert_eq!(what_this_text_implements(one_line), vec!["Something"]);
        assert!(
            what_this_text_implements(wrapped).is_empty(),
            "the reading now sees a wrapped declaration, so the account of what it \
             cannot see is out of date and must be rewritten"
        );
    }

    /// Every trait implemented in one piece of text, by the same rule
    /// [`what_the_application_implements`](self::tests::what_the_application_implements)
    /// uses — **it is that rule, called by both, so the test above cannot drift
    /// away from the thing it describes.**
    fn what_this_text_implements(text: &str) -> Vec<String> {
        let mut out = Vec::new();
        let mut in_tests = false;
        for line in text.lines() {
            let line = line.trim_start();
            if line.starts_with("#[cfg(test)]") {
                in_tests = true;
            }
            if in_tests {
                continue;
            }
            if !line.starts_with("impl") {
                continue;
            }
            let Some(at) = line.find(" for ") else {
                continue;
            };
            if let Some(name) = line[..at].split_whitespace().last() {
                out.push(
                    name.split('<')
                        .next()
                        .unwrap_or_default()
                        .rsplit("::")
                        .next()
                        .unwrap_or_default()
                        .to_string(),
                );
            }
        }
        out
    }

    /// The members of the workspace, **read from `App/Cargo.toml`** rather than
    /// kept here, so a member added later is read without anybody remembering.
    ///
    /// **Every comment is taken off before the list is split, and that is a
    /// repair rather than a tidy.** The member list carries long `#` notes
    /// saying why a member exists, and those notes have commas in them.
    /// Splitting the raw text on a comma glued the tail of a note onto the
    /// member after it, and an entry beginning `#` rather than `"` was dropped
    /// — silently, with the floor below still passing. **Two members were being
    /// skipped this way**, so the sweep that claims to read every member was not
    /// reading them. Found on 2026-09-06 in the same reading at
    /// `App/Rented-server runtime/src/arriving_later.rs`, where a third note had
    /// made the same bug drop that crate's own member and the floor caught it.
    fn the_members_of_the_workspace() -> Vec<String> {
        let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../Cargo.toml");
        let text = fs::read_to_string(&manifest).expect("the workspace manifest is readable");
        let after = text
            .split_once("members = [")
            .expect("the workspace manifest names its members")
            .1;
        let list = after.split_once(']').expect("the member list is closed").0;
        let list: String = list
            .lines()
            .map(|line| line.split('#').next().unwrap_or(""))
            .collect::<Vec<_>>()
            .join("\n");
        let members: Vec<String> = list
            .split(',')
            .filter_map(|entry| {
                let entry = entry.trim();
                entry
                    .strip_prefix('"')
                    .and_then(|rest| rest.strip_suffix('"'))
                    .map(str::to_string)
            })
            .collect();
        assert!(
            members.len() > 5 && members.iter().any(|member| member == "File Browser"),
            "the workspace members were not read, so nothing below would be searched"
        );
        members
    }

    /// Every trait the application implements outside test code, with the file
    /// it is implemented in — **read across every workspace member**, which is
    /// the reach the record above claims.
    fn what_the_application_implements() -> Vec<(String, String)> {
        let app = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        let mut out = Vec::new();
        for member in the_members_of_the_workspace() {
            let src = app.join(&member).join("src");
            let files = every_rust_file_under(&src);
            assert!(
                !files.is_empty(),
                "{member} is a workspace member and none of its source was read"
            );
            for (where_it_is, text) in files {
                for name in what_this_text_implements(&text) {
                    out.push((name, where_it_is.clone()));
                }
            }
        }
        out
    }

    /// Every `.rs` file under a folder, however deep, as (path, text).
    fn every_rust_file_under(folder: &Path) -> Vec<(String, String)> {
        let mut out = Vec::new();
        let Ok(entries) = fs::read_dir(folder) else {
            return out;
        };
        for entry in entries {
            let path = entry.expect("the entry is readable").path();
            if path.is_dir() {
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

    /// The working files as they are written, comments and all — the check above
    /// reads `pub trait` lines, which
    /// [`the_working_files`](self::tests::the_working_files) leaves alone but
    /// whose text it rewrites.
    fn the_working_files_as_written() -> Vec<(String, String)> {
        let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut out = Vec::new();
        for entry in fs::read_dir(&src).expect("the source folder is readable") {
            let path = entry.expect("the entry is readable").path();
            if path.extension().and_then(|e| e.to_str()) != Some("rs") {
                continue;
            }
            let name = path
                .file_name()
                .and_then(|s| s.to_str())
                .expect("the file has a name")
                .to_string();
            out.push((name, fs::read_to_string(&path).expect("readable")));
        }
        out
    }

    #[test]
    fn x7a_is_built_in_its_own_file_and_not_among_the_warning_layers() {
        // It sits in §4.2 rather than in §8, and it is built in its own file
        // for that reason: a reader of §4 meets it there.
        let named = the_clauses_working_code_names();
        let where_it_is: Vec<&str> = named
            .iter()
            .filter(|(marker, _)| marker == "X7a")
            .map(|(_, file)| file.as_str())
            .collect();
        assert!(
            where_it_is.contains(&"publication.rs"),
            "X7a is named in {where_it_is:?} and not in publication.rs"
        );
    }
}
