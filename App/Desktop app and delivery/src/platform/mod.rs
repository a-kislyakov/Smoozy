//! The platform adapters — `Desktop app and delivery` §6 and §9.
//!
//! **A1 closes every route to a service Section 21 does not list**, so what is
//! public here is the whole of what a module may reach the operating system
//! through, and there is no second route to any of it.
//!
//! - [`locations`] — where Smoozy's own places are (L3, L3a, L3b, F11).
//! - [`files`] — every local write, deletion and publication (§6).
//! - [`collision`] — the receiving-filesystem collision judgement's taken half
//!   and two limits of its name half (§7).
//! - [`downloads_folder`] — where a download is proposed to land (§8).
//! - [`secrets`] — the secret store on each system (§9).
//! - [`key_names`] — the existing name readings for `Keys` G11.
//! - [`the_open_panel`] — macOS's own open panel with files and folders both
//!   choosable (`File Browser` H3), which [`crate::picker`] reaches for that
//!   one ask on that one system.

pub mod collision;
pub mod downloads_folder;
pub mod files;
pub mod key_names;
pub mod locations;
pub mod secrets;
#[cfg(target_os = "macos")]
pub mod the_open_panel;

/// L3a's sweep and `Security and licence` §7's, run once at application start
/// **before any module uses either area**.
///
/// It is the area's own act rather than the writing module's, which is L3a's
/// reason: *discarded at once* does not cover a process that exits first, so
/// an interrupted run leaves three real private key files with no owner until
/// some later run.
///
/// It returns what it could not clear rather than stopping, and a caller that
/// meets trouble here has an area that is not empty rather than an application
/// that cannot start.
///
/// **It also leaves both areas standing, and that was missing until 2026-09-13
/// at step 10a piece 4, where it was found by running the application.** L3a
/// makes these areas swept at start; **nothing anywhere created one**, and
/// [`files::clear_directory`] treats a missing area as an empty one and moves
/// on. So on a computer that had never had one, `Keys` T2's probe wrote three
/// throwaway key pairs into a folder that was not there, all three writes
/// failed, and the make-key dialog offered **no type at all** — a control the
/// user could not use, with nothing anywhere saying why. **A caller creating it
/// for itself would put that duty on every caller**; the area's own act is
/// where it belongs, which is the same reason the sweep is here rather than in
/// the writing module.
pub fn sweep_the_throwaway_areas() -> Vec<files::Failure> {
    let mut trouble = Vec::new();
    for location in locations::SWEPT_AT_START {
        match locations::resolve(location) {
            Ok(path) => {
                trouble.extend(files::clear_directory(&path));
                if let Err(failure) = files::ensure_directory(&path) {
                    trouble.push(failure);
                }
            }
            Err(not_named) => trouble.push(files::Failure {
                refusal: files::Refusal::Unclassified,
                reported: not_named.reported,
            }),
        }
    }
    trouble
}
