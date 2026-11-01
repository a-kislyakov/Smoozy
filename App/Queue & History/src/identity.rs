//! Identity, folder, and label — `Queue & History` §2 Y1–Y9.
//!
//! **Constraint:** `State, identity and recovery` §2.1's seven rules and its
//! §2.2 register row — the job identity is issued by the desktop application
//! at submission, before the submission is sent, unique across every job
//! Smoozy has ever made, held in the local job record and the machine's
//! durable job record. Fact 29's byte limits and the receiving-end loss.
//!
//! **What is here is the judgement and the composition; the issuing is
//! `State, identity and recovery`'s own** — [`smoozy_state::identities`]
//! draws sixteen bytes from the operating system's cryptographic source and
//! nothing else, which is Y1 kept by that crate rather than a second issuer
//! here.
//!
use smoozy_job_files::names::{job_output_folder_name, job_record_directory_name};
use smoozy_state::identities::{Identity, IdentityKind};

/// **Y1: a job identity is 128 bits drawn from the operating system's
/// cryptographic random source at the moment of issue, and nothing else.** No
/// clock, no counter, no machine reading, no user input, no path. **The draw
/// is `State, identity and recovery`'s own issuer**, whose width is what
/// makes two jobs unable to collide however quickly one follows another and
/// across every machine and every rental.
pub fn issue_a_job_identity() -> Result<Identity, smoozy_state::identities::NoRandomness> {
    Identity::issue(IdentityKind::Job)
}

/// The width Y1 names, in bits — sixteen bytes, which is what the issuer
/// draws. Stated here so the test below holds this crate's clause to that
/// crate's constant rather than to a number remembered.
pub const IDENTITY_BITS: usize = smoozy_state::identities::IDENTITY_BYTES * 8;

/// **Y2: the textual form is fixed-length lowercase letters and digits,
/// nothing else.** No capitals, no spaces, no accents, no punctuation — each
/// exclusion an observed hazard: names differing only in capitals, and two
/// spellings of one accented name, collapsed to a single file on arrival at
/// the Mac with every fetch reporting success; one space in a path broke
/// three tools three different ways.
///
/// **This is a judgement over a rendering, never a parse**: it says whether
/// text has the form, and nothing reads meaning from it (Y9).
pub fn has_the_textual_form(text: &str) -> bool {
    text.len() == THE_TEXTUAL_FORMS_LENGTH
        && text
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
}

/// Y2's fixed length — two characters per byte of the draw.
pub const THE_TEXTUAL_FORMS_LENGTH: usize = smoozy_state::identities::IDENTITY_BYTES * 2;

/// **Y3: the identity is issued before the submission is sent, and that is
/// what makes a dropped submission answerable.** The provisional local job
/// record is written first (`State, identity and recovery` §7 step 2); a lost
/// acknowledgement is resolved by asking the machine for the identity, never
/// by re-issuing the work (its §12 rule 4).
pub const THE_IDENTITY_IS_ISSUED_BEFORE_THE_SUBMISSION_IS_SENT: bool = true;

/// **Y4: an adopted job keeps the identity the machine's record carries.**
/// One job never has two identities. Adoption reads the machine's text back
/// as the same identity rather than issuing a second one.
pub fn the_adopted_identity(the_machines_text: &str) -> Option<Identity> {
    Identity::read_back(IdentityKind::Job, the_machines_text)
}

/// **Y5: a re-run is the same job with the same identity and writes where its
/// first attempt did; a fresh attempt the user asks for is a new job with a
/// new identity and a new folder.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatAnAttemptIs {
    /// The machine's own re-run after an interruption: the same identity.
    AReRunOfTheSameJob,
    /// The user's own act — *submit again* or *carry on* — which is a new job.
    AFreshJobTheUserAskedFor,
}

/// Y5, performed: whether the identity travels or a new one is issued.
pub fn the_identity_for(attempt: WhatAnAttemptIs, the_jobs: &Identity) -> IdentityTravels {
    match attempt {
        WhatAnAttemptIs::AReRunOfTheSameJob => IdentityTravels::TheSame(the_jobs.clone()),
        WhatAnAttemptIs::AFreshJobTheUserAskedFor => IdentityTravels::ANewOneIsIssued,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdentityTravels {
    TheSame(Identity),
    /// Issued by [`issue_a_job_identity`] at the new submission, with a new
    /// folder named for it.
    ANewOneIsIssued,
}

/// **Y6: where the job's folder sits is inherited, not chosen here** —
/// `Job files and outputs` chooses one root on one mount, once per
/// installation, and the job output folder is under its job area. A rental's
/// choice of disk is a quota rather than a second device, and nothing here
/// assumes otherwise.
pub const WHERE_THE_FOLDER_SITS_IS_INHERITED: bool = true;

/// **Y7: Fact 6's survival half is owed and this crate does not paper over
/// it.** Nothing here reads a folder's survival as assured, and a folder
/// found gone reads as the completeness statement's absent paths, never as a
/// silent success (§18 item 6, permanently unresolved).
pub const A_FOLDER_FOUND_GONE_READS_AS_ABSENT_PATHS_NEVER_AS_SUCCESS: bool = true;

/// **Y8: the entry always shows the full label, from the record, and the
/// folder name shown is the manifest's recorded path verbatim.**
///
/// The naming half is `Job files and outputs` N3–N4's and is not
/// re-decided; what was left to this document is what the user sees where a
/// label was substituted: **the label they typed, everywhere a label is
/// shown; the folder's real name, everywhere a path is shown.** No warning
/// and no marker is raised about the difference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheEntryShows {
    /// The label as typed, whole, from the record.
    pub the_label: Option<String>,
    /// The manifest's recorded job output folder, verbatim.
    pub the_folder_as_recorded: String,
}

/// Y8, performed. **The folder is the manifest's field and never a
/// re-rendering of the label**: this takes the recorded path as a value and
/// composes nothing from the label.
pub fn what_the_entry_shows(
    the_label_in_the_record: Option<&str>,
    the_manifests_job_output_folder: &str,
) -> WhatTheEntryShows {
    WhatTheEntryShows {
        the_label: the_label_in_the_record.map(str::to_string),
        the_folder_as_recorded: the_manifests_job_output_folder.to_string(),
    }
}

/// **The folder name is written once from the identity and the label**, by
/// `Job files and outputs` N3's own rule, at submission — the one place the
/// two meet. After this nothing here reads it (Y9).
pub fn the_folder_name_written_once(job: &Identity, label: Option<&str>) -> String {
    job_output_folder_name(job, label)
}

/// The job record directory's name, `Job files and outputs` N3's: the
/// identity's textual form alone, and it is that crate's own function.
pub fn the_record_directory_name(job: &Identity) -> String {
    job_record_directory_name(job)
}

/// **Y9: nothing anywhere parses a folder name back into an identity or a
/// label.** The identity travels in records, the label travels in the record
/// and the note, and the folder name is written once from them and read
/// never. **Kept by this crate having no function that takes a folder name
/// and returns anything** — the test below reads the crate's own sources for
/// one.
pub const NOTHING_PARSES_A_FOLDER_NAME: bool = true;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_job_identity_is_sixteen_random_bytes_and_two_never_match() {
        assert_eq!(IDENTITY_BITS, 128);
        let one = issue_a_job_identity().unwrap();
        let two = issue_a_job_identity().unwrap();
        assert_ne!(one, two);
        assert_eq!(one.kind(), IdentityKind::Job);
    }

    #[test]
    fn the_textual_form_is_fixed_length_lowercase_letters_and_digits() {
        let issued = issue_a_job_identity().unwrap();
        assert!(has_the_textual_form(&issued.as_text()));
        assert_eq!(issued.as_text().len(), THE_TEXTUAL_FORMS_LENGTH);
        // Each exclusion is an observed hazard.
        assert!(!has_the_textual_form(&issued.as_text().to_uppercase()));
        assert!(!has_the_textual_form(&format!("{} ", issued.as_text())));
        assert!(!has_the_textual_form("café"));
        assert!(!has_the_textual_form("ab-cd"));
        assert!(!has_the_textual_form(""));
    }

    #[test]
    fn an_adopted_job_keeps_the_identity_the_machine_carries() {
        let issued = issue_a_job_identity().unwrap();
        let adopted = the_adopted_identity(&issued.as_text()).unwrap();
        assert_eq!(issued, adopted);
        assert_eq!(the_adopted_identity("not an identity"), None);
    }

    #[test]
    fn a_re_run_keeps_the_identity_and_a_fresh_attempt_issues_a_new_one() {
        let job = issue_a_job_identity().unwrap();
        assert_eq!(
            the_identity_for(WhatAnAttemptIs::AReRunOfTheSameJob, &job),
            IdentityTravels::TheSame(job.clone())
        );
        assert_eq!(
            the_identity_for(WhatAnAttemptIs::AFreshJobTheUserAskedFor, &job),
            IdentityTravels::ANewOneIsIssued
        );
    }

    #[test]
    fn the_entry_shows_the_label_as_typed_and_the_folder_as_recorded() {
        let job = issue_a_job_identity().unwrap();
        let label = "kitchen/scene: the big one";
        let folder = format!(
            "/ephemeral/smoozy/jobs/{}",
            the_folder_name_written_once(&job, Some(label))
        );
        let shown = what_the_entry_shows(Some(label), &folder);
        // The label as typed, substitution and all left alone.
        assert_eq!(shown.the_label.as_deref(), Some(label));
        // The folder as recorded: the substituted name, verbatim.
        assert_eq!(shown.the_folder_as_recorded, folder);
        assert!(shown
            .the_folder_as_recorded
            .ends_with("kitchen_scene: the big one"));
        assert!(!shown.the_folder_as_recorded.contains("kitchen/scene"));
    }

    #[test]
    fn the_record_directory_is_the_identity_alone() {
        let job = issue_a_job_identity().unwrap();
        assert_eq!(the_record_directory_name(&job), job.as_text());
    }

    /// **Y9, held rather than remembered**: no function in this crate takes
    /// a folder name apart. The sources are read for the two operations a
    /// parse would need — splitting a name on the separator the folder name
    /// is composed with, or reading an identity back out of a folder name.
    ///
    /// **It reads the top level of both of this module's folders**, because six
    /// of the files it read here moved to `smoozy-queue-history-machine` on
    /// 2026-09-19 and a guard over this folder alone would have gone quiet over
    /// them.
    #[test]
    fn nothing_in_this_crate_parses_a_folder_name() {
        let here = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let entries = std::fs::read_dir(here.join("src"))
            .unwrap()
            .chain(std::fs::read_dir(here.join("../Queue & History — machine/src")).unwrap());
        for entry in entries {
            let path = entry.unwrap().path();
            if path.extension().and_then(|e| e.to_str()) != Some("rs") {
                continue;
            }
            let text = std::fs::read_to_string(&path).unwrap();
            for (number, line) in text.lines().enumerate() {
                let trimmed = line.trim_start();
                if trimmed.starts_with("//") || trimmed.starts_with("///") {
                    continue;
                }
                let names_a_folder = line.contains("folder");
                // The needles are built at run time so these lines are not
                // themselves a match.
                let split = format!("{}{}", "split_once(", "'-')");
                let splitn = format!("{}{}", "splitn(2, ", "'-')");
                let read = format!("{}{}", "read_back(IdentityKind::Job, ", "&folder");
                let takes_it_apart =
                    line.contains(&split) || line.contains(&splitn) || line.contains(&read);
                assert!(
                    !(names_a_folder && takes_it_apart),
                    "{}:{} parses a folder name: {line}",
                    path.display(),
                    number + 1
                );
            }
        }
        const _: () = assert!(NOTHING_PARSES_A_FOLDER_NAME);
    }
}
