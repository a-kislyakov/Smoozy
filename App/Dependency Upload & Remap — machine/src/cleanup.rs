//! What goes with a job's machine-side record — `Dependency Upload & Remap`
//! §8. **The rest of §8 is not here**: it stays in `smoozy-dependencies`'
//! `cleanup`, which re-exports this at the path it always had.

use smoozy_job_files_machine::layout::TheLayout;
use smoozy_state::identities::Identity;

use crate::bytes::MachineLocation;
use crate::remap::RemapMaterial;

/// **U2: a job's remap material is cleared when the job's machine-side record
/// goes, and by the same acts.** `Job files and outputs` Section 11 clears a
/// job record directory when the job's record is gone or the machine is
/// destroyed; the remap material at `remaps/<job identity>` follows the same
/// rule, and the act that removes the job's machine-side record removes it —
/// supplied to `Queue & History`, since deletion of jobs is that module's.
pub fn what_goes_with_the_jobs_machine_side_record(
    layout: &TheLayout,
    job: &Identity,
) -> MachineLocation {
    RemapMaterial::location_in(layout, job)
}
