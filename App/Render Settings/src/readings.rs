//! The readings the judgements stand on, behind one trait the acting half
//! implements — the split `App/Keys/` draws at its `usable::Readings`.
//!
//! **Three readings serve this document and none is a judgement of its own.**
//! The hardware reading is `Servers`' remembered one (D5); the version list is
//! `Server Setup`'s fresh search result (B1); and the four-value file
//! identity read at Q14 item 2 is a command-channel reading on the same
//! footing as a listing (Q15). **The recorded reading itself is not here**:
//! it arrives as the raw frame [`crate::endings::judge`] judges, because Q7's
//! whole-or-nothing boundary is drawn by that judgement and not by whoever
//! took the read.
//!
//! **The held hardware reading and the four-value read's values live in
//! `smoozy-render-settings-machine`**, the half of this plan the rented
//! machine's own program may take, and are re-exported below at the paths
//! they always had. The trait stays here, because the version list is `Server
//! Setup`'s search result and a machine half cannot take it. Split at step 10c
//! piece 3.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Servers` H1. `Server Setup` B11, B12, B13.

use smoozy_server_setup::search::TheSearchResult;
use smoozy_state::identities::Identity;

pub use smoozy_render_settings_machine::readings::{FourValuesNow, HeldHardware, HeldState};

/// The readings behind this document's judgements.
///
/// **Every one is a reading of something already held or already running,
/// and none starts a Blender**: the hardware reading is taken by `Servers`
/// once per primary connection; the version list is `Server Setup` B12's
/// search, run when this page asks for it, period-held and never durable,
/// with B11's honesty marks and B13's path-beside-version on every entry; and
/// the four-value read is a use of the standing connection at a user's act,
/// on the same footing as a listing.
pub trait Readings {
    /// `Servers`' remembered hardware reading for the server row, with its
    /// H1 state.
    fn the_hardware_reading(&self, server: &Identity) -> HeldHardware;

    /// **B1: the list is `Server Setup`'s fresh search result** — asked for
    /// when this page wants it. **`None` where there is no machine to ask it
    /// of** (a history-opened page with no machine yet, Y15), which is a
    /// different thing from a machine holding no Blender, which answers with
    /// an empty list.
    fn the_version_list(&self, server: &Identity) -> Option<TheSearchResult>;

    /// **Q14 item 2: the four values now standing for the file** (`Server
    /// protocol and transfers` S13's shape), or the channel's own inability
    /// to say.
    fn the_four_values_now(&self, server: &Identity, path: &str) -> FourValuesNow;
}
