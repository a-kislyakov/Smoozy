//! The three paths one job has on a machine — the half of `publication.rs`
//! the rented machine's own program may take.
//!
//! **Nothing here places anything.** The placement, the attempt record's write
//! and every other act on the file-transfer subsystem stay in
//! `smoozy-job-files`, which re-exports this type at
//! `publication::AJobsPaths`, the path it has always been reached at. The
//! paths sit here because the manifest is composed from them and the machine
//! reads the manifest. Split at step 10c piece 3.
//!
//! **The staging prefix is public here where it was crate-wide there**, because
//! the placement next door still spells its staged names with it and a crate
//! cannot lend a private item to another.

use crate::layout::{Area, MachineRoot, TheLayout, WhereThePackageAreaSits};
use crate::manifest::MANIFEST_FILE_NAME;
use crate::names::{self, WhyThePathIsRefused};

/// The three paths one job has on a machine, composed once and carried.
///
/// **N1: every path Smoozy composes on a rented machine is composed by this
/// document's rules.** These are composed here and nowhere else.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AJobsPaths {
    /// **The job output folder** — the render's, and nothing else's (L7).
    pub job_output_folder: String,
    /// **The job record directory** — the manifest, the attempt records, and
    /// everything else derived from the manifest (M4, L9).
    pub job_record_directory: String,
    /// **The staging directory M5 publishes from.** It sits in the job-record
    /// area beside the final name, which is `Server protocol and transfers`
    /// S1's same-filesystem requirement met by placement rather than by a
    /// check.
    pub staging_directory: String,
}

/// **What sits between the staging directory and the job identity in its
/// name.** Build's own choice; a job identity is never reused, so the staging
/// name is unique for the same reason the record directory's is.
///
/// **It is the crate's one spelling of those four letters.** Made reachable
/// beyond this file on 2026-09-05, so §8.4's derivative staging marks itself the
/// same way rather than writing them a second time.
pub const THE_STAGING_PREFIX: &str = "staging-";

/// **`Server Setup`'s staging folder for its runs, inside its Blender area**: the
/// one composition, because the supervisor makes it and `Server Setup` B10's
/// search must leave it out. Moved here 2026-11-03.
pub fn the_setup_runs_staging_folder(blender_area: &str) -> String {
    format!("{blender_area}/{THE_STAGING_PREFIX}setup")
}

/// **The job record directory alone, from the root and the job identity** —
/// N3: it is named by the identity's textual form alone, so a reader holding
/// the root and no label can compose it.
///
/// **That reader is the supervisor**, which writes this path into
/// `Queue & History` Q2's job record as its reference to the directory. Added at
/// step 10c piece 5; [`AJobsPaths::composed`] composes its own through here, so
/// the directory has one composition.
///
/// **The package area's placement does not enter it**: the job-record area is
/// one of the four L6 puts directly under the root whatever L4a decided, so the
/// layout below is asked for that area and for nothing else.
pub fn the_job_record_directory(
    root: &MachineRoot,
    job: &smoozy_state::identities::Identity,
) -> String {
    let layout = TheLayout::of(root.clone(), WhereThePackageAreaSits::UnderTheRoot);
    format!(
        "{}/{}",
        layout.area(Area::JobRecord),
        names::job_record_directory_name(job)
    )
}

impl AJobsPaths {
    /// **Composed from the layout, the job identity and the label — and from
    /// nothing else.**
    ///
    /// **N5's budget is checked before any of them is used** (N8's *the check is
    /// made before any path is used*), so a path that the machine would refuse
    /// at open is refused here, by Smoozy's own arithmetic, with the cause
    /// established before the machine is asked (N6).
    pub fn composed(
        layout: &TheLayout,
        job: &smoozy_state::identities::Identity,
        label: Option<&str>,
    ) -> Result<AJobsPaths, WhyThePathIsRefused> {
        let record_name = names::job_record_directory_name(job);
        let paths = AJobsPaths {
            job_output_folder: format!(
                "{}/{}",
                layout.area(Area::Job),
                names::job_output_folder_name(job, label)
            ),
            job_record_directory: the_job_record_directory(layout.root(), job),
            staging_directory: format!(
                "{}/{THE_STAGING_PREFIX}{record_name}",
                layout.area(Area::JobRecord)
            ),
        };
        names::a_path_fits(&paths.job_output_folder)?;
        names::a_path_fits(&paths.job_record_directory)?;
        names::a_path_fits(&paths.staging_directory)?;
        // **N8: the budget is measured against the predicted file path and not
        // against the folder** — *"checking the folder alone would pass a
        // destination whose files then fail at open."* The manifest is the one
        // file this composition already knows the name of, and the path it is
        // opened at is the staged one, which is the longer of the two by the
        // staging prefix; the published path is that path with the prefix
        // taken off, so a staged path that fits carries a published one that
        // fits. **The attempt records' two paths are checked where their name
        // is known**, at [`write_an_attempt_record`], because the name carries
        // the attempt's number.
        names::a_path_fits(&paths.the_manifest_where_it_is_written())?;
        Ok(paths)
    }

    /// **Where the manifest is written**, which is inside the staging directory
    /// and not inside the published one — M5 publishes the whole directory by
    /// one rename.
    pub fn the_manifest_where_it_is_written(&self) -> String {
        format!("{}/{MANIFEST_FILE_NAME}", self.staging_directory)
    }
}
