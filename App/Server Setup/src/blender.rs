//! The Blender lifecycle's one desktop-side composition — `Server Setup` §5.1,
//! §5.2 and §5.3 — and, from 2026-10-16, Y9a's one judgement over the installs
//! still waiting behind a chain of Blender installs, which only the desktop
//! holds.
//!
//! **Everything else in this module is in `smoozy-server-setup-machine`**, the
//! half of this plan the rented machine's own program may take, and is
//! re-exported here at the paths it has always been reached at, so
//! `crate::blender::Version`, `THE_FLOOR` and the rest resolve exactly as they
//! did. **What stays is the Blender area's path**, because it is composed from
//! `Job files and outputs`' layout and this crate's own placement, and that
//! half would take the machine half into a crate it must not reach. The split
//! was made at step 10c piece 3.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Job files and outputs` N1, L6.

pub use smoozy_server_setup_machine::blender::*;

/// **B9: the area's paths are composed here, and that was a recorded seam
/// rather than a silence.**
///
/// `Job files and outputs` N1 gave every machine path except the staging path to
/// that document, and its 1.2 closed the root's areas at four. **A Blender
/// install is neither package state nor job material nor asset, and this
/// document places it** — so the layout chosen here is a sibling under the root,
/// **widening that document's enumeration from four areas to five**.
///
/// **Closed 2026-08-22 at the Architecture seam close, both ways** — N1 names
/// this document's Blender-area paths beside the staging path, and 1.2 L6 names
/// five areas with the Blender area among them. **The Blender area is one of the
/// four that sit directly under the root** and is untouched by `Job files and
/// outputs` L4a's placing of
/// the package area on a mount of its own.
pub fn the_blender_area(layout: &smoozy_job_files::layout::TheLayout) -> String {
    crate::placement::the_blender_area(layout)
}

/// **Y9a item 2: whether the installs kept for a server are ones an earlier
/// run of Smoozy left by closing first**, and so are named on the Set up card.
/// Added 2026-10-16, step 10c's owed piece 4.
///
/// **Y9a item 1 is what makes this the whole test**: the desktop writes the
/// list at the press, rewrites it as each next install begins, and empties it
/// when the chain ends, however it ends. So a list standing while this run of
/// the desktop drives no setup run on the server can only be one a run that
/// exited first left behind. **While a run is driven, never**: the list is that
/// run's own and it is still going. **Nothing is begun from it** (Y9a item 4) —
/// this says only whether to name them.
pub fn a_closed_run_left_them(kept: &[String], a_run_is_driven_here: bool) -> bool {
    !kept.is_empty() && !a_run_is_driven_here
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn installs_kept_are_a_closed_runs_only_while_no_run_is_driven_here() {
        let kept = vec!["5.2.2".to_string(), "4.5.14".to_string()];
        assert!(a_closed_run_left_them(&kept, false));
        assert!(!a_closed_run_left_them(&kept, true), "the list is the run's own while it goes");
        assert!(!a_closed_run_left_them(&[], false), "nothing kept is nothing to name");
    }

    #[test]
    fn the_blender_area_is_composed_from_the_layout_and_not_from_a_name() {
        use smoozy_job_files::layout::{MachineRoot, TheLayout, WhereThePackageAreaSits};
        let layout = TheLayout::of(
            MachineRoot::chosen("/home/smoozy/smoozy").expect("a root"),
            WhereThePackageAreaSits::UnderTheRoot,
        );
        assert_eq!(the_blender_area(&layout), "/home/smoozy/smoozy/blender");
    }
}
