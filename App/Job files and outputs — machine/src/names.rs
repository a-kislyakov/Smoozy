//! §2.2 to §2.5 — what a job folder is named for, and the length budget in
//! bytes checked before a path is used.

use serde::{Deserialize, Serialize};
use smoozy_state::identities::{Identity, IdentityKind};

/// **N5's two limits, in bytes.**
///
/// Observed on 2026-08-17: *"255 characters for a single name and 4095 for a
/// whole path"*, both refused **at open before any byte is sent**, both leaving
/// nothing behind, matching the `f_namemax = 255` the file-transfer extension
/// reports. The earlier six-machine batch measured the same limit **in bytes** —
/// *"255 **bytes** per name (85 CJK characters)"* (Fact 29).
///
/// **So the check counts bytes, never characters**, and a check written in
/// characters passes a name the machine will refuse. Every function below takes
/// its measure off `str::len`, which is bytes, and never off a character count.
pub const NAME_BUDGET_BYTES: usize = 255;

/// The whole-path half of N5's budget.
pub const PATH_BUDGET_BYTES: usize = 4095;

/// Why a name or a path is refused.
///
/// **N6: a refusal for length is never surfaced as the machine's own words.**
/// The refusal for both over-limit cases is `Bad message`, *"which reads as
/// protocol corruption rather than *name too long*"*. **The check is Smoozy's
/// own arithmetic against N5's budget**, so the cause is established before the
/// machine is asked — and this type carries no machine text at all, which is
/// what keeps `Security and licence` §13 rule 5 uncrossed by construction.
// **The reason travels with the map** (P12), which is written into the manifest
// and read back on the machine, so it is rendered and read like every other
// field of one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WhyThePathIsRefused {
    /// One segment is over [`NAME_BUDGET_BYTES`].
    ASingleNameIsOverTheBudget { it_is_this_many_bytes: usize },
    /// The whole path is over [`PATH_BUDGET_BYTES`].
    TheWholePathIsOverTheBudget { it_is_this_many_bytes: usize },
    /// A path with nothing in it names nothing.
    ItIsEmpty,
}

/// **N5, on one segment.**
pub fn a_name_fits(name: &str) -> Result<(), WhyThePathIsRefused> {
    if name.is_empty() {
        return Err(WhyThePathIsRefused::ItIsEmpty);
    }
    if name.len() > NAME_BUDGET_BYTES {
        return Err(WhyThePathIsRefused::ASingleNameIsOverTheBudget {
            it_is_this_many_bytes: name.len(),
        });
    }
    Ok(())
}

/// **N5, on a whole path: the path's own budget and every segment's.**
///
/// **N8: the budget is measured against the predicted file path and not against
/// the folder.** *"Checking the folder alone would pass a destination whose
/// files then fail at open."* So this is asked of whatever path is about to be
/// used, and the deepest predicted file path is what §7 hands it — which is
/// the one in `arriving_later`, in `smoozy-job-files`.
pub fn a_path_fits(path: &str) -> Result<(), WhyThePathIsRefused> {
    if path.is_empty() {
        return Err(WhyThePathIsRefused::ItIsEmpty);
    }
    if path.len() > PATH_BUDGET_BYTES {
        return Err(WhyThePathIsRefused::TheWholePathIsOverTheBudget {
            it_is_this_many_bytes: path.len(),
        });
    }
    for segment in path.split('/').filter(|segment| !segment.is_empty()) {
        a_name_fits(segment)?;
    }
    Ok(())
}

/// **N3: a job record directory's name is the job identity's textual form
/// alone.**
pub fn job_record_directory_name(job: &Identity) -> String {
    debug_assert_eq!(job.kind(), IdentityKind::Job);
    job.as_text()
}

/// What sits between the identity and the label in a job output folder's name.
///
/// **Build's own choice and the plan fixes none of it**, because *"nothing
/// anywhere parses a name back to recover a label"* (N3). One byte, so N7's
/// arithmetic is one subtraction.
const BETWEEN_THE_IDENTITY_AND_THE_LABEL: char = '-';

/// **How much of the identity a job output folder's name carries: its first
/// eight hex characters.** The Creator's twenty-second notes, note 3
/// (2026-10-29): the whole 32-character identity made names *"like a billion
/// characters long"*. Eight characters are four billion values, and a clash
/// only matters between two jobs with one label in one folder — where N9's
/// plain create-directory refuses the repeat rather than overwriting. The
/// folder is written once into the manifest and nothing parses it back, so the
/// job record directory (the whole identity) is untouched.
pub const THE_IDENTITY_CHARACTERS_IN_A_FOLDER_NAME: usize = 8;

/// **N3: a job output folder's name is the job identity's own textual form
/// and, where a label was given, a bounded rendering of that label.**
///
/// **The identity alone carries uniqueness**; nothing parses a name back to
/// recover a label; and **no scene name, no file name, no date and no counter
/// is any part of either**.
///
/// **Neither of the two hazards cited nearby forbids this composition.** `State,
/// identity and recovery` §2.1 rule 3 forbids deriving an **identity** from
/// anything the user typed, and an identity and a label in one folder name
/// derives nothing from the label. Fact 29's silent overwrite is a name made of
/// the label **alone** — with the identity in the name two same-label jobs get
/// distinct names, and N9's plain create-directory rule already refuses a
/// repeat.
pub fn job_output_folder_name(job: &Identity, label: Option<&str>) -> String {
    debug_assert_eq!(job.kind(), IdentityKind::Job);
    let mut identity = job.as_text();
    identity.truncate(THE_IDENTITY_CHARACTERS_IN_A_FOLDER_NAME);
    let Some(label) = label else {
        return identity;
    };
    // **N7's arithmetic: the label's bound in bytes is what the name budget
    // leaves after the identity, so a long label costs the label and never the
    // arithmetic.**
    let left = NAME_BUDGET_BYTES
        .saturating_sub(identity.len())
        .saturating_sub(BETWEEN_THE_IDENTITY_AND_THE_LABEL.len_utf8());
    let rendered = bounded_rendering_of(label, left);
    if rendered.is_empty() {
        return identity;
    }
    format!("{identity}{BETWEEN_THE_IDENTITY_AND_THE_LABEL}{rendered}")
}

/// **N3's *what cannot enter a name is substituted rather than refused*, and
/// the bound in bytes.**
///
/// **What cannot enter a name is two bytes and no more.** **N13** records what
/// a rented Linux machine actually refuses: *"a colon, an asterisk, a question
/// mark, a pipe, a semicolon, letters outside the Latin alphabet, an emoji, a
/// newline, a tab, a leading dash and a trailing space all reached disk
/// unaltered"*, and *"A rented Linux machine refuses nothing in a name but
/// length."* So the substitution covers the separator, which would make a
/// second segment rather than a name, and the zero byte, which ends a name in
/// the machine's own calls — **and nothing else, because substituting what the
/// machine accepts would be Smoozy inventing a rule the machine does not
/// have.**
///
/// **N15 is why a newline is left alone.** *"A newline in a file name splits a
/// listing across lines on both routes, so no reading here is line-oriented"* —
/// every reading takes the protocol's or the walk's own fields, so a newline in
/// a name costs nothing and needs no substitution.
///
/// **The truncation is at a character boundary**, never mid-character, so the
/// rendering is always text the user can read back.
fn bounded_rendering_of(label: &str, budget_bytes: usize) -> String {
    let substituted: String = label
        .chars()
        .map(|character| match character {
            '/' | '\0' => '_',
            other => other,
        })
        .collect();
    let trimmed = substituted.trim();
    if trimmed.len() <= budget_bytes {
        return trimmed.to_string();
    }
    let mut cut = budget_bytes;
    while cut > 0 && !trimmed.is_char_boundary(cut) {
        cut -= 1;
    }
    trimmed[..cut].trim_end().to_string()
}

/// **N4: a label is carried beside the identity, in the job's record, and is
/// *"never load-bearing for uniqueness anywhere"*.**
///
/// The full label stays authoritative in the job's record and on the entry,
/// which is where `Queue & History` shows it. **N3 renders it into one name and
/// nothing else composes with it.**
pub const THE_FULL_LABEL_IS_AUTHORITATIVE_IN_THE_RECORD_AND_NEVER_IN_A_NAME: bool = true;

/// **N12: Smoozy writes no output name.**
///
/// The redirect moves folders and leaves names alone, which is the spec's own
/// rule; what Blender then does with a name is Blender's, and it differs by
/// version. **Nothing in this crate composes the name of a file a render
/// writes**, and the redirect that moves the folders is §5's, which arrives at
/// step 7.
pub const SMOOZY_WRITES_NO_OUTPUT_NAME: bool = true;

/// **N15: two lines any reading of a folder must survive.**
///
/// A newline in a file name **splits a listing across lines** on both routes,
/// so no reading is line-oriented; and the file-transfer client's text output is
/// lossy where the protocol is not — *"no year or seconds for recent entries, no
/// timezone ever, symlink targets dropped, and non-Latin names escaped into
/// octal"*. **Every reading here takes the protocol's own fields and never a
/// rendered line**, which the enumeration in `Server protocol and transfers`
/// already keeps by returning names as bytes with no text-returning reader in
/// its wire layer at all.
pub const NO_READING_OF_A_FOLDER_IS_LINE_ORIENTED: bool = true;

/// **N14: the expected-output inventory is a statement about the machine and
/// never about the user's computer.**
///
/// Seventeen names written on the rental came back as **fourteen files** on the
/// Mac, three destroyed on arrival, **with every fetch reporting success**.
/// *"So frames rendered into distinct names can arrive as fewer files than were
/// made, with nothing said."* **What becomes of a name on arrival is `Desktop
/// app and delivery`'s receiving-filesystem collision judgement and
/// `Download`'s response to it.**
///
/// The inventory itself is §7's and is built at [`crate::inventory`]
/// *(corrected 2026-09-07: this said it arrives at step 7 and pointed at
/// `arriving_later` in `smoozy-job-files`, which step 7 landed and which has been empty
/// since)*; the statement is here because it is a property of this section's
/// names rather than of that section's count.
pub const THE_INVENTORY_IS_A_STATEMENT_ABOUT_THE_MACHINE: bool = true;

#[cfg(test)]
mod tests {
    use super::*;

    fn a_job() -> Identity {
        Identity::issue(IdentityKind::Job).unwrap()
    }

    #[test]
    fn n5_counts_bytes_and_not_characters() {
        // *"255 bytes per name (85 CJK characters)"*. A name of 85 CJK
        // characters is exactly the budget; 86 is over it, and a check written
        // in characters would pass 255 of them.
        let eighty_five: String = "々".repeat(85);
        assert_eq!(eighty_five.chars().count(), 85);
        assert_eq!(eighty_five.len(), 255);
        assert!(a_name_fits(&eighty_five).is_ok());

        let eighty_six: String = "々".repeat(86);
        assert_eq!(eighty_six.chars().count(), 86);
        assert!(matches!(
            a_name_fits(&eighty_six),
            Err(WhyThePathIsRefused::ASingleNameIsOverTheBudget { .. })
        ));
    }

    #[test]
    fn the_whole_path_budget_and_the_per_name_budget_are_two_checks() {
        // A path made of segments that each fit, and does not.
        let segment = "a".repeat(200);
        let long: String = (0..30)
            .map(|_| format!("/{segment}"))
            .collect::<Vec<_>>()
            .join("");
        assert!(a_name_fits(&segment).is_ok());
        assert!(matches!(
            a_path_fits(&long),
            Err(WhyThePathIsRefused::TheWholePathIsOverTheBudget { .. })
        ));
    }

    #[test]
    fn n7s_arithmetic_costs_the_label_and_never_the_budget() {
        let job = a_job();
        let name = job_output_folder_name(&job, Some(&"x".repeat(1000)));
        assert!(a_name_fits(&name).is_ok());
        assert_eq!(name.len(), NAME_BUDGET_BYTES);
        assert!(name.starts_with(&job.as_text()[..THE_IDENTITY_CHARACTERS_IN_A_FOLDER_NAME]));
    }

    #[test]
    fn a_folder_name_carries_a_short_identity_and_the_label() {
        // The Creator's twenty-second notes, note 3.
        let job = a_job();
        let name = job_output_folder_name(&job, Some("Kitchen"));
        assert_eq!(name, format!("{}-Kitchen", &job.as_text()[..8]));
        assert_eq!(job_output_folder_name(&job, None).len(), 8);
    }

    #[test]
    fn a_bounded_rendering_never_cuts_a_character_in_half() {
        let job = a_job();
        // Every character here is three bytes, so an arithmetic cut lands
        // mid-character unless the boundary is respected.
        let label: String = "々".repeat(400);
        let name = job_output_folder_name(&job, Some(&label));
        assert!(a_name_fits(&name).is_ok());
        // It is still text — a cut through a character would not be.
        assert!(std::str::from_utf8(name.as_bytes()).is_ok());
        assert!(name.len() <= NAME_BUDGET_BYTES);
    }

    #[test]
    fn a_separator_in_a_label_is_substituted_rather_than_making_a_second_segment() {
        let job = a_job();
        let name = job_output_folder_name(&job, Some("kitchen/scene"));
        assert!(!name.contains('/'), "a label made a second segment: {name}");
        assert!(name.ends_with("kitchen_scene"));
    }

    #[test]
    fn n13s_own_list_of_what_the_machine_accepts_is_left_alone() {
        // *"A rented Linux machine refuses nothing in a name but length."*
        // Substituting what the machine accepts would be Smoozy inventing a
        // rule the machine does not have.
        let job = a_job();
        let name = job_output_folder_name(&job, Some("a:b*c?d|e;f\tg é 🙂"));
        for character in [':', '*', '?', '|', ';', '\t', 'é', '🙂'] {
            assert!(
                name.contains(character),
                "{character:?} was substituted and N13 says the machine takes it"
            );
        }
    }

    #[test]
    fn two_jobs_with_one_label_get_two_names() {
        // Fact 29's silent overwrite is a name made of the label **alone**.
        let one = job_output_folder_name(&a_job(), Some("Kitchen"));
        let other = job_output_folder_name(&a_job(), Some("Kitchen"));
        assert_ne!(one, other);
    }

    #[test]
    fn a_job_record_directory_is_the_identity_alone() {
        let job = a_job();
        assert_eq!(job_record_directory_name(&job), job.as_text());
        // And no label reaches it, however long a label the job carries.
        assert!(!job_record_directory_name(&job).contains('-'));
    }

    #[test]
    fn a_label_that_renders_to_nothing_leaves_the_identity_alone() {
        let job = a_job();
        assert_eq!(
            job_output_folder_name(&job, Some("   ")),
            job.as_text()[..THE_IDENTITY_CHARACTERS_IN_A_FOLDER_NAME],
            "a name must never end in a bare separator"
        );
    }
}
