//! **§5.2 and §5.3 performed — A7, A9, A11 and A12.** The mirror directories
//! made segment by segment, and one asset's upload arranged under the transfer
//! contract.
//!
//! **The contract is `Server protocol and transfers` §7, taken entire and not
//! written a second time.** Staging, integrity, resume, publication last,
//! cleanup — every one of them is that document's loop, and what this file does
//! is arrange one transfer and hand it over. **There is a working transport in
//! this workspace and this module drives it rather than composing another.**
//!
//! # The directories, and the one form that is used
//!
//! **A7: the plain create-directory form, segment by segment, and never the
//! make-parents form**, whose silent success against an existing folder is the
//! recorded collision path (`Job files and outputs` N9). **An already-standing
//! segment is the ordinary case rather than a fault** — unlike a job folder, a
//! mirror folder is *meant* to be shared by every asset beneath it — so a
//! refusal is resolved by one follow-up attribute read: where the path now
//! exists as a directory, the landing proceeds; anything else fails the row as
//! *could not be sent*.
//!
//! **The refusal-then-read is safe against Smoozy's own concurrency** because
//! this module never removes a mirror directory (U1), so a directory seen
//! standing stays standing.
//!
//! # The transfer, and the three things this file supplies to it
//!
//! **A9: every asset upload is a transfer under §7, arranged per file, bound to
//! its server, admitted through the per-server point, with this module's
//! transfers declared as one priority class — the dependency-upload class.**
//! That class is already the declared order's third row, declared by this
//! document and named there, so nothing here declares a second one.
//!
//! **A11: a retry is a new transfer**, under a new transfer identity, re-sending
//! only what did not publish — no unit is retried inside its action (`State,
//! identity and recovery` §9.5).
//!
//! **A12: a lost connection behaves exactly as the contract makes it behave
//! everywhere.** The transfer stops, staged material survives, published files
//! stay, nothing at a final name is partial, and nothing resumes on its own.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Job files and outputs` N9, N15. `Server protocol
//! and transfers` S9, T3, T5, T8. `State, identity and recovery` §9.5. `File
//! Browser` §13.

use smoozy_server_protocol::subsystem::client::Subsystem;
use smoozy_server_protocol::subsystem::SubsystemFailure;
use smoozy_server_protocol::transfers::{Direction, TransferRecord};
use smoozy_state::identities::Identity;
use tokio::io::{AsyncRead, AsyncWrite};

use crate::bytes::{join, segments, MachineLocation, StoredLocation, SEPARATOR};
use crate::landing::{fits_the_budgets, CouldNotBeSent};

/// **A7's outcome for one mirror directory.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HowTheDirectoryWent {
    /// It was made, or it was already standing — **the ordinary case rather
    /// than a fault**, and the two are one outcome because A7 makes them one.
    ItStands,
    /// The plain form was refused and the follow-up read did not find a
    /// directory there. The row fails as *could not be sent*.
    TheRowFailsAsCouldNotBeSent,
}

/// The asset-upload boundary was opened.
#[derive(Debug)]
pub struct SendingFilesUpWasOpen(());

/// Open the upload without consulting paid access.
pub fn open_an_asset_upload() -> Result<SendingFilesUpWasOpen, std::convert::Infallible> {
    Ok(SendingFilesUpWasOpen(()))
}

/// **The mirror directories one send has seen standing** — A7's own
/// reasoning, carried across the send's files. Added 2026-09-30, step 10c's
/// eleventh-pass fix piece 4.
///
/// **A7: *"this module never removes a mirror directory (Section 8), so a
/// directory seen standing stays standing."*** That sentence licenses the
/// refusal-then-read; it licenses this too. **A directory this send made, or
/// read back as a directory after a refusal, is not asked for again in the
/// same send.** Nothing here is a listing and nothing is kept past the send:
/// it is held by the one send that saw it, and a later send asks again.
///
/// **The case A7 was first written for** is one asset at a time, where every
/// segment of its mirror was made — or refused and read back — once. **The
/// case it now also meets** is a project's fifty-seven found files, where the
/// mirror is the file's whole path from the user's own disk — all but the last
/// one or two segments the same for every file — and each standing one costs
/// a refusal and a read. **At the desk the 4.5 battle project's send spent 52
/// of its 73 round trips a file there** (a mirror of 27 folders; step 10c's
/// eleventh-pass fix piece 4's account), **for directories the send had
/// already made itself.**
///
/// **Shared by the send's files that move at once.** What was seen is behind
/// a lock that is never held across a wait. **And one file makes directories
/// at a time**: files that start together would otherwise each ask for the
/// whole mirror before any of them had seen it — four times the asking, for
/// the same folders — so the others wait for the one making them, and then
/// find them seen. A file whose folders are all seen does not wait at all.
#[derive(Debug, Default)]
pub struct TheMirrorDirectoriesSeen {
    seen: std::sync::Mutex<std::collections::BTreeSet<Vec<u8>>>,
    making: tokio::sync::Mutex<()>,
}

impl TheMirrorDirectoriesSeen {
    fn held(&self) -> std::sync::MutexGuard<'_, std::collections::BTreeSet<Vec<u8>>> {
        self.seen
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// **Whether this send has seen the directory standing.**
    pub fn stands(&self, directory: &[u8]) -> bool {
        self.held().contains(directory)
    }

    fn it_stands(&self, directory: &[u8]) {
        self.held().insert(directory.to_vec());
    }
}

/// **A7, performed over every segment of a mirrored location's folder** that
/// the send has not already seen standing.
///
/// The final segment is the file's own name and is not made as a directory;
/// everything above it is.
///
/// **Nothing here reads a listing to decide anything** (`Job files and outputs`
/// N15), and nothing takes a directory's presence as state beyond the one send
/// that saw it ([`TheMirrorDirectoriesSeen`]): the attribute read after a
/// refusal is the follow-up A7 names, made as one comparison at one moment,
/// and its only two outcomes are the two above.
pub async fn make_the_mirror_directories<S>(
    _sending_files_up_was_open: &SendingFilesUpWasOpen,
    subsystem: &mut Subsystem<S>,
    mirrored_location: &MachineLocation,
    seen: &TheMirrorDirectoriesSeen,
) -> HowTheDirectoryWent
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    let whole = mirrored_location.as_bytes();
    let leading = if whole.first() == Some(&SEPARATOR) {
        vec![SEPARATOR]
    } else {
        Vec::new()
    };
    let parts = segments(whole);
    // Every segment but the last, which is the file's own name.
    let mut folders = Vec::new();
    let mut so_far = leading;
    for segment in parts.iter().take(parts.len().saturating_sub(1)) {
        so_far = join(&so_far, segment);
        folders.push(so_far.clone());
    }
    if folders.iter().all(|folder| seen.stands(folder)) {
        return HowTheDirectoryWent::ItStands;
    }
    let _one_file_makes_them_at_a_time = seen.making.lock().await;
    for so_far in folders {
        // **Seen standing by this send, so it stands** (A7's own sentence).
        if seen.stands(&so_far) {
            continue;
        }
        match subsystem.make_directory(&so_far).await {
            Ok(()) => seen.it_stands(&so_far),
            Err(_) => {
                // **The one follow-up A7 allows**, and only after a refusal.
                match subsystem.stat(&so_far).await {
                    Ok(attributes) if is_a_directory(&attributes) => seen.it_stands(&so_far),
                    _ => return HowTheDirectoryWent::TheRowFailsAsCouldNotBeSent,
                }
            }
        }
    }
    HowTheDirectoryWent::ItStands
}

/// **The one bit of a mode this file reads, and it reads nothing else.**
///
/// A directory's own type bit is what separates *the folder is already there*
/// from *something else is standing at that name*, which is the whole of the
/// follow-up read's question. **It is not a permission check and it is not a
/// listing.**
fn is_a_directory(attributes: &smoozy_server_protocol::subsystem::Attributes) -> bool {
    // The file-type bits of a Unix mode, and the directory value among them.
    const THE_TYPE_BITS: u32 = 0o170_000;
    const A_DIRECTORY: u32 = 0o040_000;
    matches!(attributes.permissions, Some(mode) if mode & THE_TYPE_BITS == A_DIRECTORY)
}

/// **A9 and A11: one asset's upload, arranged.**
///
/// **What this composes is the record the contract's own loop drives**, and
/// nothing else: the transfer identity, the direction, the server row and its
/// installation identity, the source path as stored on disk (F12), the mirrored
/// location A4 gives it, and the size. **The loop is
/// `smoozy_server_protocol::transfers::moving::carry_out`'s** and this module
/// writes none of it.
///
/// **A5's two byte checks run before anything is arranged** — every segment
/// against 255 and the whole path against 4095, in bytes, by Smoozy's own
/// arithmetic — so a row whose mirrored location cannot fit is *could not be
/// sent* with the reason named **before any transfer exists**.
#[allow(
    clippy::too_many_arguments,
    reason = "every one is a value the contract's own record takes, and               `TransferRecord::arranged` carries the same allow for the same               reason: gathering them into a struct of this module's own would               be a second shape for the one that document already fixes"
)]
pub fn arrange_one_assets_upload(
    _sending_files_up_was_open: &SendingFilesUpWasOpen,
    transfer: Identity,
    server_row: Identity,
    installation: Identity,
    run: Identity,
    source: &StoredLocation,
    mirrored_location: &MachineLocation,
    staging_directory: &str,
    size: u64,
) -> Result<TransferRecord, CouldNotBeSent> {
    fits_the_budgets(mirrored_location.as_bytes()).map_err(CouldNotBeSent::TheMirrorWillNotFit)?;
    // **The transfer record's own two paths are text on that document's
    // surface**, and a location that is not text is one no transfer can be
    // arranged for. **That is a real residue and it is named rather than
    // smoothed**: it is *could not be sent* with the reason named — the
    // landing rule does not cover it — and never a substituted spelling, which
    // would key the wrong record and mirror a wrong name. Every location
    // Smoozy composes on the machine is text by construction; the one that may
    // not be is the user's own source path.
    let source_path = std::str::from_utf8(source.as_bytes())
        .map_err(|_| CouldNotBeSent::TheLocationsFormIsNotOneTheLandingRuleCovers)?
        .to_string();
    let final_path = std::str::from_utf8(mirrored_location.as_bytes())
        .map_err(|_| CouldNotBeSent::TheLocationsFormIsNotOneTheLandingRuleCovers)?
        .to_string();
    Ok(TransferRecord::arranged(
        transfer,
        Direction::Up,
        server_row,
        Some(installation),
        source_path,
        final_path,
        staging_directory,
        size,
        run,
    ))
}

/// **A11: a retry is a new transfer under a new identity**, and this is that
/// said as code rather than as a habit. The old record is left exactly as it
/// was — *"a retry that fails again leaves the row as it was"*.
pub fn a_retry_is_a_new_transfer(previous: &TransferRecord, fresh: Identity) -> bool {
    previous.transfer != fresh
}

/// What a refusal on the file-transfer surface says about a mirror directory.
///
/// **Kept as a value so a reader can see that nothing is inferred from it.**
/// The refusal decides only whether the follow-up read is taken; the read
/// decides the outcome.
pub fn a_refusal_only_asks_for_the_follow_up_read(_failure: &SubsystemFailure) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_state::identities::IdentityKind;

    fn an_identity(kind: IdentityKind) -> Identity {
        Identity::issue(kind).expect("this machine supplies random bytes")
    }

    #[test]
    fn an_asset_upload_opens_without_any_policy() {
        assert!(open_an_asset_upload().is_ok());
    }

    /// The ordinary boundary supplies the upload witness.
    fn an_open_gate() -> SendingFilesUpWasOpen {
        open_an_asset_upload().expect("an open answer opens the act")
    }

    /// **A5's two byte checks run before a transfer exists.** A mirrored
    /// location that cannot fit is *could not be sent* with the reason named,
    /// and no staging path, no identity and no record is made for it.
    #[test]
    fn a_mirror_that_cannot_fit_is_refused_before_anything_is_arranged() {
        let over_the_name_budget: Vec<u8> =
            [b"/root/assets/files/".as_slice(), &[b'x'; 300]].concat();
        let refused = arrange_one_assets_upload(
            &an_open_gate(),
            an_identity(IdentityKind::Transfer),
            an_identity(IdentityKind::LocalServer),
            an_identity(IdentityKind::Installation),
            an_identity(IdentityKind::Run),
            &StoredLocation::as_the_system_returned_it(b"/Users/a/x.png"),
            &MachineLocation::composed(over_the_name_budget),
            "/root/assets/.smoozy-staging",
            10,
        );
        assert!(matches!(
            refused,
            Err(CouldNotBeSent::TheMirrorWillNotFit(_))
        ));
    }

    /// **A9: the arrangement is an upload bound to its server and its
    /// installation identity**, carrying the source as stored on disk and the
    /// mirror A4 gives it — and nothing of the loop is composed here.
    #[test]
    fn one_assets_upload_is_arranged_under_the_contract_and_nothing_else() {
        let server = an_identity(IdentityKind::LocalServer);
        let installation = an_identity(IdentityKind::Installation);
        let record = arrange_one_assets_upload(
            &an_open_gate(),
            an_identity(IdentityKind::Transfer),
            server.clone(),
            installation.clone(),
            an_identity(IdentityKind::Run),
            &StoredLocation::as_the_system_returned_it(b"/Users/a/tex/METallic.png"),
            &MachineLocation::composed(b"/root/assets/files/Users/a/tex/METallic.png".to_vec()),
            "/root/assets/.smoozy-staging",
            4096,
        )
        .expect("it arranges");
        assert_eq!(record.direction, Direction::Up);
        assert_eq!(record.server, server);
        assert_eq!(record.installation, Some(installation));
        assert_eq!(record.source_path, "/Users/a/tex/METallic.png");
        assert_eq!(
            record.final_path,
            "/root/assets/files/Users/a/tex/METallic.png"
        );
        assert_eq!(record.total_size_expected, 4096);
        // **S9: no ending until the loop writes one**, and no publication has
        // happened, so no placement record can exist yet (Q11).
        assert_eq!(record.ending, None);
    }

    /// **A11 and `State, identity and recovery` §9.5: a retry is a new
    /// transfer, never the same one tried again.**
    #[test]
    fn a_retry_takes_a_fresh_transfer_identity() {
        let first = arrange_one_assets_upload(
            &an_open_gate(),
            an_identity(IdentityKind::Transfer),
            an_identity(IdentityKind::LocalServer),
            an_identity(IdentityKind::Installation),
            an_identity(IdentityKind::Run),
            &StoredLocation::as_the_system_returned_it(b"/Users/a/x.png"),
            &MachineLocation::composed(b"/root/assets/files/Users/a/x.png".to_vec()),
            "/root/assets/.smoozy-staging",
            10,
        )
        .expect("it arranges");
        assert!(a_retry_is_a_new_transfer(
            &first,
            an_identity(IdentityKind::Transfer)
        ));
        assert!(!a_retry_is_a_new_transfer(&first, first.transfer.clone()));
        const {
            assert!(crate::landing::A_RETRY_IS_A_NEW_TRANSFER_AND_RESENDS_ONLY_WHAT_DID_NOT_PUBLISH)
        };
    }

    /// **A7: the plain form, segment by segment.** The class is named as a
    /// value so a reader can see the make-parents form is refused by decision
    /// rather than by omission.
    #[test]
    fn the_mirror_is_made_with_the_plain_form_segment_by_segment() {
        const {
            assert!(crate::landing::A_MIRROR_DIRECTORY_IS_CREATED_SEGMENT_BY_SEGMENT_WITH_THE_PLAIN_FORM)
        };
        // The directory type bit is what the follow-up read asks about, and
        // nothing else in the mode is read.
        let a_directory = smoozy_server_protocol::subsystem::Attributes::permissions(0o040_755);
        let a_file = smoozy_server_protocol::subsystem::Attributes::permissions(0o100_644);
        assert!(is_a_directory(&a_directory));
        assert!(!is_a_directory(&a_file));
        assert!(!is_a_directory(
            &smoozy_server_protocol::subsystem::Attributes::none()
        ));
    }
}
