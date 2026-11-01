//! **What a job is called, wherever Smoozy names one — one rule, kept here
//! because both ends read it.**
//!
//! `Design/Queue & History § A rendering entry`, Creator-stated 2026-08-10:
//! *"Where the job carries no label of its own, the name is the file."* The
//! label the user typed, whole (Y8); where there is none, the name of the
//! `.blend` the job renders — the last part of its path on the machine. **The
//! desktop reads that path off the local record** (N11) and **the machine reads
//! it off the job's manifest** (`Job files and outputs` M1), and the two are the
//! same path, so a screen and a phone call one job the same thing.
//!
//! **Why it is here and not on the desktop, where it was first written.**
//! Vast-pass fix piece 2 wrote it in the desktop's queue screen, and the phone
//! needs it too (`Telegram` R7a): the phone's words are composed on the rented
//! machine, whose program may not take the desktop. This crate is the half of
//! the module that owns the label which both programs already take, so **the
//! rule moved here once and both call it** — the desktop through
//! `smoozy-queue-history`'s re-export, the machine through
//! [`crate::acting::note::TheNote::what_the_job_is_called`]. Moved
//! 2026-10-01, Vast-pass fix piece 3.
//!
//! **Nothing here is composed from the job's identity**: the job's folder is
//! the one place the identity names it (`Job files and outputs` N3).

/// **The rule.** The label whole, where there is one; else the file's name;
/// else nothing. **A label of only spaces is no label**, because it would
/// read as blank.
pub fn what_a_job_is_called(label: Option<&str>, blend_path_on_the_machine: Option<&str>) -> String {
    match label.filter(|label| !label.trim().is_empty()) {
        Some(label) => label.to_string(),
        None => blend_path_on_the_machine.map(the_files_name).unwrap_or_default(),
    }
}

/// **The last part of a path on the machine** — the file's name, as the
/// Render Settings page shows the file it was opened on. A path ending in a
/// separator, which no `.blend` path does, is given back whole rather than as
/// nothing.
pub fn the_files_name(path: &str) -> String {
    path.rsplit('/')
        .next()
        .filter(|name| !name.is_empty())
        .unwrap_or(path)
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_label_is_used_whole_and_the_file_is_not_added_to_it() {
        assert_eq!(
            what_a_job_is_called(Some("Kitchen \u{2014} final"), Some("/r/assets/kitchen.blend")),
            "Kitchen \u{2014} final"
        );
    }

    #[test]
    fn no_label_is_named_by_the_file() {
        assert_eq!(what_a_job_is_called(None, Some("/pv-huge/blend/pv-anim.blend")), "pv-anim.blend");
        assert_eq!(what_a_job_is_called(Some(""), Some("/pv-huge/blend/pv-anim.blend")), "pv-anim.blend");
        assert_eq!(what_a_job_is_called(Some("  \t"), Some("/pv-huge/blend/pv-anim.blend")), "pv-anim.blend");
    }

    #[test]
    fn with_neither_there_is_nothing_and_nothing_is_made_up() {
        assert_eq!(what_a_job_is_called(None, None), "");
        assert_eq!(what_a_job_is_called(Some(""), None), "");
    }

    #[test]
    fn the_files_name_is_the_last_part_of_the_path() {
        assert_eq!(the_files_name("/a/b/scene.blend"), "scene.blend");
        assert_eq!(the_files_name("scene.blend"), "scene.blend");
        assert_eq!(the_files_name("/a/b/"), "/a/b/");
    }
}
