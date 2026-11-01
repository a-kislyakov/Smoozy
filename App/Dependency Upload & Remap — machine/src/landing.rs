//! The asset area — `Dependency Upload & Remap` §5: the area's own directory,
//! read off `Job files and outputs`' layout, and the three fixed segments
//! under it. **The landing rule, the budgets, the resume verdict and the
//! handed unit are not here**: they stay in `smoozy-dependencies`' `landing`,
//! which re-exports these at the paths they always had. These moved because
//! the remap material is kept under `remaps`.

use smoozy_job_files_machine::layout::{Area, TheLayout};

/// **A2: three fixed segments sit directly under the asset area, and their
/// names are constants of this document, deliberately short against the byte
/// budget.** Keeping the mirrors one segment down is what stops a user path's
/// own first segment ever colliding with a name this document needs for
/// itself.
pub const FILES: &str = "files";
pub const FIXED: &str = "fixed";
pub const REMAPS: &str = "remaps";

/// The asset area's own directory, from `Job files and outputs`' layout.
pub fn the_asset_area(layout: &TheLayout) -> String {
    layout.area(Area::Asset)
}
