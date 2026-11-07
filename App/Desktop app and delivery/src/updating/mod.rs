//! **The desktop's own update file, read and trusted** — E15, E16's digest and
//! length, E19 and E10a, with `State, identity and recovery` J6's first row
//! and `State, identity and recovery` J7 and `State, identity and recovery`
//! J10 beneath them. Built at step 11's update-reading piece, which is the
//! reading half: what the file is, what it takes for it to be believed, and
//! what is kept afterwards. **Nothing in these first three fetches, stages,
//! replaces or launches anything**; the check and the replacement are the files
//! below them.
//!
//! **The check is the fourth file, from step 11's check piece** — [`checking`]:
//! the read of the update file at the start, at the one repeating interval and
//! at the user's press, admitted under the update fetch's class, and About's
//! reading. It reads the file through the three below and fetches no artifact.
//!
//! **The Mac route is four more, from step 11's Mac update piece** — what
//! happens after a check finds a newer version on a Mac:
//!
//! - [`fetching`] — the artifact the manifest names, fetched from beside the
//!   update file and held to its length and digest (E16).
//! - [`the_hold`] — Z31's reading: publication waits while any job has no
//!   ending or any transfer is under way.
//! - [`the_mac`] — the staged bundle offered to the system's own check, the
//!   directory-level replace with its named backup, and the rollback (Z5 to Z9,
//!   Z21, Z24). **No route skips the check.**
//! - [`the_probe`] — the new build started and watched for its token, and what
//!   a launch does with an uncleared probe record (Z21, Z21a, P2).
//!
//! **The Windows route is two more, from step 11's Windows update piece** — side by side, never
//! in place:
//!
//! - [`the_windows_side_by_side`] — the new build fetched into a new per-user
//!   folder, held to its length and digest and to the system's own signature
//!   check, the Start-menu shortcut pointed at it, and the relaunch helper
//!   started (Z13 to Z15a, Z21, Z24).
//! - [`the_relaunch_helper`] — that helper: this same program, started by the
//!   old process to start the probe, watch it, and roll back if it fails (Z18
//!   to Z20, P3's second helper).
//!
//! **Three files.**
//!
//! - [`manifest`] — E15's signed manifest, every field it names, and the one
//!   entry for the row this copy runs on.
//! - [`trust`] — E19's trust set: the recovery keys a build carries, the
//!   recovery-signed statements that move the online keys, their monotonic
//!   trust sequence, and `State, identity and recovery` J7's judgement of what
//!   a kept record is worth.
//! - [`verifying`] — the whole file read in order: statements, then the
//!   manifest, then the row; and after a fetch, the artifact's length and
//!   digest against the manifest (E16).
//!
//! **The Creator's two recovery keys are in this build from 2026-11-01** (E19,
//! two from 2026-10-24), with the statement naming the first everyday key:
//! [`trust::THE_RECOVERY_KEYS_THIS_BUILD_CARRIES`] and
//! [`trust::THE_STATEMENT_THIS_BUILD_CARRIES`]. Their secret halves are the
//! Creator's alone. **A build from before that day carries none and verifies
//! nothing**, which is E19's own *"a build that cannot form a chain from its
//! embedded recovery public key does not verify and therefore does not
//! update"* — 0.2.0 is one.
//!
//! **The signature itself is not written here.** It is the one the server
//! package's release already uses — Ed25519 over a context and the canonical
//! bytes, the strict check — and it is called from there:
//! `smoozy_server_protocol::release::a_signature_verifies`, with that crate's
//! canonical writer and reader. **What is this module's own is the three
//! contexts below**, which are not the server package's. That is half of
//! `State, identity and recovery` J10 kept by construction: a signature the
//! server-package release root made over any document cannot be a signature
//! over one of these, whichever key is tried. The other half is that no
//! function here is ever handed that root — the trust set is
//! [`trust::TheCarriedSet`] and nothing else.
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this
//! crate's own plan, `Preparation/Architecture/Desktop app and delivery.md`.
//! Every other document's marker is written with that document at the site.

pub mod checking;
pub mod fetching;
pub mod manifest;
pub mod the_hold;
pub mod the_linux_package;
pub mod the_mac;
pub mod the_probe;
pub mod the_relaunch_helper;
pub mod the_windows_side_by_side;
pub mod trust;
pub mod verifying;

/// **The context every trust statement's recovery signature covers.**
///
/// It is not the server package's key-set context, and the test below says so.
pub const THE_STATEMENT_SIGNING_CONTEXT: &[u8] = b"smoozy.desktop-update.trust-statement.1\0";

/// **The context every manifest signature covers**, and it is deliberately not
/// [`THE_STATEMENT_SIGNING_CONTEXT`]: a signature over one kind of document is
/// never a signature over the other.
pub const THE_MANIFEST_SIGNING_CONTEXT: &[u8] = b"smoozy.desktop-update.manifest.1\0";

/// **The context a recovery key's one public proof is signed under**, and it
/// is neither of the two above, so a proof kept in this repository is inert:
/// no document here verifies under it. See
/// [`trust::ACarriedRecoveryKey::proof`].
pub const THE_RECOVERY_PROOF_CONTEXT: &[u8] = b"smoozy.desktop-update.recovery-proof.1\0";

/// **The update file's name in each release** — E10a leaves the name to Build.
///
/// **It is never the server package's release file**, which is called
/// `release` in the same repository (`App/Release/src/making.rs`). The two
/// share a release page and share no trust.
pub const THE_UPDATE_FILES_NAME: &str = "desktop-update";

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_server_protocol::release as server_package;

    /// **`State, identity and recovery` J10, by construction: none of the
    /// three contexts here is either of the server package's.** A signature the
    /// server-package release root made can never verify as one of these
    /// documents, whatever key it is tried under.
    #[test]
    fn no_context_here_is_the_server_packages() {
        let ours = [
            THE_STATEMENT_SIGNING_CONTEXT,
            THE_MANIFEST_SIGNING_CONTEXT,
            THE_RECOVERY_PROOF_CONTEXT,
        ];
        let theirs = [
            server_package::THE_KEY_SET_SIGNING_CONTEXT,
            server_package::THE_MANIFEST_SIGNING_CONTEXT,
            server_package::trust::THE_ROOT_PROOF_CONTEXT,
        ];
        for one in ours {
            for other in theirs {
                assert_ne!(one, other);
            }
        }
        assert_ne!(THE_STATEMENT_SIGNING_CONTEXT, THE_MANIFEST_SIGNING_CONTEXT);
        assert_ne!(THE_STATEMENT_SIGNING_CONTEXT, THE_RECOVERY_PROOF_CONTEXT);
        assert_ne!(THE_MANIFEST_SIGNING_CONTEXT, THE_RECOVERY_PROOF_CONTEXT);
    }
}
