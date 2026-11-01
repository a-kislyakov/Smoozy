//! **The transaction's durable records, written and read through the
//! subsystem** — the phase record, the installation identity, the
//! boot-registration record and the active-package pointer.
//!
//! # Why a write here is three operations and not one
//!
//! `Rented-server runtime` G3 writes every phase **durably** before the act it
//! names, and what *durably* means is `State, identity and recovery` §1.3 rule
//! 6's — the bytes on the device before the name that publishes them exists.
//! The machine's own half performs that with `fsync` and a rename on its own
//! filesystem. **This side has neither**, so it composes the same four steps out
//! of §7's own publication contract: the bytes to a staging name, S20's
//! `fsync@openssh.com` against the open handle, and S16's publication over the
//! final name.
//!
//! **The operation the publication takes is not decided here.** It is
//! [`crate::transfers::publication::which_operation`]'s, on a
//! [`crate::transfers::publication::Intent`] of *replace* — because a record is
//! replaced at every phase — and S17's declared degradation is what happens
//! where the extension is not announced. **Nothing writes to the final name and
//! hopes.**
//!
//! **S20 is scoped the same way and takes the same shape.** The clause is
//! *"where the server offers it"*, so a server that does not announce the flush
//! is a server the ordering does not apply on —
//! [`whether_the_durability_ordering_applies`] is that scoping asked once, and
//! [`WhetherTheDurabilityOrderingApplied`] is its answer, declared and carried
//! back to the caller rather than skipped quietly.
//!
//! # What is not read as state
//!
//! `State, identity and recovery` §1.3 rule 5 binds every read below: **nothing
//! reads a file's presence, its name or its size as state.** The pointer is read
//! as the link's own value, not as a path that resolves; a record is read as
//! content; and an absent record is the honest never-held answer rather than a
//! zero.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rented-server runtime` G10, G11, G16, G17, G31a,
//! K15, K16, K16a, K16b, Z2, Z3. `State, identity and recovery` D1.
//!
//! **A marker this plan also defines is never listed there and is written out
//! with its document at the site**, because a header list cannot tell the two
//! apart and a bare one there would resolve to a real and wrong clause.
//! `Rented-server runtime` G1 to G9 are of that kind here.

use smoozy_runtime::package::phase::PhaseRecord;
use smoozy_runtime::registration::BootRegistrationRecord;
use smoozy_runtime::the_program::{records, the_active_package_pointer_at};
use smoozy_runtime::written_form;
use smoozy_state::identities::Identity;
use tokio::io::{AsyncRead, AsyncWrite};

use crate::bootstrap::{the_record_in, WhatARecordHeld, THE_MOST_ONE_OF_THESE_RECORDS_MAY_HOLD};
use crate::subsystem::client::Subsystem;
use crate::subsystem::wire::extension;
use crate::subsystem::{StatusCode, SubsystemFailure};
use crate::transfers::publication::{which_operation, Intent, Operation};

/// One of `Rented-server runtime`'s own records, at its path under a package
/// area.
///
/// **The composition is that crate's** —
/// [`smoozy_runtime::the_program::the_record_at`] — and this is the same call
/// under a name this module can reach for. **No path is synthesised here**,
/// which is the same rule C15 states for the installation identity.
pub fn the_record_at(package_area: &str, record: &str) -> String {
    smoozy_runtime::the_program::the_record_at(package_area, record)
}

/// **What a staging name for a record looks like on this side.**
///
/// **It is drawn from §1.1's class-2 alphabet** — lower-case letters, digits,
/// hyphens and the path separator — so that a path composed here is syntax on
/// no route, and it sits beside the final name so the publication's rename
/// stays inside one directory.
pub fn the_staging_name_for(final_name: &str) -> String {
    format!("{final_name}-being-written")
}

/// **Where one attempt's artifact is staged — and it stays there once
/// committed, because the pointer comes to name it.**
///
/// **S4 is the rule and this is S4 performed**: *"The staging path is
/// synthesised from the transfer identity … and never from the file's own
/// name"*, and *"This document owns that path"*. The identity is the attempt's,
/// which is B1's idempotency key. **S4's attempt number adds nothing here**:
/// a placement never gives one identity a second path — `Rented-server
/// runtime` G17's only restart under the same identity is a fresh replacement
/// *after* sweeping this same path — so the identity alone names one object.
///
/// **Why this is not [`the_staging_name_for`] of anything, and it is the
/// whole of step 10c piece 12's first fault.** That rule gives a record's
/// staging name from its final name, and the artifact was composed as
/// `the_staging_name_for(<the fixed entry-point name>)` — which is exactly the
/// pointer's own staging name, since the pointer carries the fixed
/// entry-point name. **So the pointer's switch removed the staged program as
/// its first act** and left the pointer naming a file that no longer existed,
/// on every machine. Two different things, one rule, one path.
///
/// **The name keeps §1.1's class-2 alphabet** — lower-case letters, digits and
/// hyphens, the identity being lower-case hexadecimal — and **it can be no
/// other name in the package area**: every record there, the pointer, and
/// every staging name [`the_staging_name_for`] gives, carries a letter no
/// hexadecimal identity holds after this prefix. The test beside it holds that
/// against every record `Rented-server runtime` names.
pub fn the_staging_location_for(
    package_area: &str,
    attempt: &smoozy_state::identities::Identity,
) -> String {
    format!(
        "{}/{THE_STAGED_PACKAGES_PREFIX}{}",
        crate::placement::the_package_area_as_text(package_area.as_bytes()),
        attempt.as_text()
    )
}

/// The fixed part of every staged package's name. See
/// [`the_staging_location_for`].
pub const THE_STAGED_PACKAGES_PREFIX: &str = "staged-package-";

/// **Whether S20's durability ordering applied to a write, and there are two
/// answers rather than one.**
///
/// **This is the shape S17's own degradation already takes next door** — a
/// named branch of a decision function, declared with what it costs, rather
/// than a step that silently does not happen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhetherTheDurabilityOrderingApplied {
    /// **S20 as written.** The server announced `fsync@openssh.com`, so the
    /// staged bytes were made durable before the name that publishes them
    /// existed.
    TheStagedBytesWereMadeDurableFirst,
    /// **S20's own scoping met.** The server announced no flush, so the write
    /// happened without the ordering. **What holds without it is §12 item 7's**:
    /// the staging file is whole, both paths are named, and the removal in
    /// S17's degradation is a recorded act rather than a silent one — so what a
    /// power cut at the wrong instant costs is a record that has to be written
    /// again, and Smoozy knows which records were written this way.
    TheServerOffersNoFlushSoTheOrderingDidNotApply,
}

/// **S20's own scoping, asked once: *"where the server offers it"*.**
///
/// **A server that does not announce the flush is not a server to refuse.** The
/// clause is scoped in its own words, S17's compound case says in as many words
/// what happens on a machine announcing neither extension — the remove-then-rename
/// route is taken *"while S20's durability ordering does not apply"* — and §12
/// item 7 writes out what holds without it. **The same crate already reads it
/// this way for transfers**, at [`crate::transfers::moving`], which asks the
/// announcement and skips the flush where it is absent.
///
/// **What refusing instead would cost**, since this file refused until a code
/// round said otherwise: the very first act of putting Smoozy on such a machine
/// fails, every time, with no route left — a machine the plan serves turned into
/// one Smoozy will not touch.
pub fn whether_the_durability_ordering_applies(
    the_flush_is_announced: bool,
) -> WhetherTheDurabilityOrderingApplied {
    if the_flush_is_announced {
        WhetherTheDurabilityOrderingApplied::TheStagedBytesWereMadeDurableFirst
    } else {
        WhetherTheDurabilityOrderingApplied::TheServerOffersNoFlushSoTheOrderingDidNotApply
    }
}

/// **A durable write through the surface: the bytes, then S20's flush, then
/// S16's publication.**
///
/// `State, identity and recovery` §1.3 rule 6 is what *durably* means and this
/// is that rule performed over §1.1's second mechanism. **The flush comes before
/// the rename and not after it**: a rename that publishes unflushed bytes
/// publishes a name whose contents may not be on the device.
///
/// **Where `fsync@openssh.com` is not announced the write still happens and the
/// answer says the ordering did not apply.** §1.2's *"a capability that is not
/// announced is never attempted and never guessed at"* is kept by asking the
/// announcement rather than by refusing the write: the flush is not attempted,
/// and nothing pretends it ran. **What that leaves is §12 item 7's declared
/// residual and not a silent one.**
pub async fn publish_durably<S>(
    subsystem: &mut Subsystem<S>,
    final_name: &str,
    bytes: &[u8],
) -> Result<WhetherTheDurabilityOrderingApplied, SubsystemFailure>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    let staging = the_staging_name_for(final_name);

    // A staging name left by an interrupted attempt is replaced rather than
    // read: nothing anywhere reads one, and every record it could hold is
    // re-composed from values the caller already has.
    match subsystem.remove(staging.as_bytes()).await {
        Ok(()) => {}
        Err(why) if why.is_the_path_not_being_there() => {}
        Err(why) => return Err(why),
    }

    let handle = subsystem.create_exclusively(staging.as_bytes()).await?;
    let mut at = 0u64;
    let chunk = subsystem.announcement().limits.write_chunk();
    for piece in bytes.chunks(chunk) {
        subsystem.send_a_write(&handle, at, piece).await?;
        at += piece.len() as u64;
    }
    subsystem.settle_the_writes().await?;
    let ordering = whether_the_durability_ordering_applies(
        subsystem.announcement().announces(extension::FSYNC),
    );
    let flushed = match ordering {
        WhetherTheDurabilityOrderingApplied::TheStagedBytesWereMadeDurableFirst => {
            subsystem.make_durable(&handle).await
        }
        WhetherTheDurabilityOrderingApplied::TheServerOffersNoFlushSoTheOrderingDidNotApply => {
            Ok(())
        }
    };
    subsystem.close(&handle).await?;
    flushed?;

    let announced = subsystem.announcement().announces(extension::POSIX_RENAME);
    match which_operation(Intent::ReplaceIntended, announced) {
        Operation::ThePosixRenameExtension => {
            subsystem
                .posix_rename(staging.as_bytes(), final_name.as_bytes())
                .await?
        }
        // **S17's declared degradation**, and it is declared rather than
        // improvised: the staging file is whole, the final name is removed, and
        // the base rename follows. No part-arrived bytes ever reach the final
        // name.
        Operation::RemoveTheFinalNameThenTheBaseRename => {
            match subsystem.remove(final_name.as_bytes()).await {
                Ok(()) => {}
                Err(why) if why.is_the_path_not_being_there() => {}
                Err(why) => return Err(why),
            }
            subsystem
                .rename(staging.as_bytes(), final_name.as_bytes())
                .await?
        }
        // A record is always replace-intended, so this arm is unreachable from
        // here; it is written out rather than defaulted because the operation
        // set is the publication contract's and not this file's to narrow.
        Operation::TheBaseRename => {
            subsystem
                .rename(staging.as_bytes(), final_name.as_bytes())
                .await?
        }
    }
    Ok(ordering)
}

/// **`Rented-server runtime` G1's phase record, written durably.**
///
/// **`Rented-server runtime` G3's ordering is the caller's and this is only the
/// write** — the same split the machine's own half takes.
pub async fn write_the_phase_record<S>(
    subsystem: &mut Subsystem<S>,
    package_area: &str,
    record: &PhaseRecord,
) -> Result<WhetherTheDurabilityOrderingApplied, SubsystemFailure>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    let bytes =
        written_form::write(record).map_err(|_| SubsystemFailure::TheAnswerCouldNotBeRead)?;
    publish_durably(
        subsystem,
        &the_record_at(package_area, records::THE_PHASE_RECORD),
        &bytes,
    )
    .await
}

/// **The phase record, read as content.**
///
/// **The three answers are not two.** An absent record is a machine that has
/// never held a package; a record that is there and does not read back is
/// C14's *present and different* branch and is never folded into absence.
/// `Rented-server runtime` G17's retry table is answered from this and from the
/// pointer, and from nothing else.
pub async fn read_the_phase_record<S>(
    subsystem: &mut Subsystem<S>,
    package_area: &str,
) -> Result<WhatARecordHeld<PhaseRecord>, SubsystemFailure>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    read_a_record(
        subsystem,
        &the_record_at(package_area, records::THE_PHASE_RECORD),
        written_form::read::<PhaseRecord>,
    )
    .await
}

/// **`Rented-server runtime` K16's record, written durably.**
///
/// **It holds the registration's content by value** and nothing derives it from
/// what is on the machine at the moment it is read. `Rented-server runtime` K16a
/// makes it the non-root placement's on both machine kinds, which is this
/// performer on every bare machine.
pub async fn write_the_boot_registration_record<S>(
    subsystem: &mut Subsystem<S>,
    package_area: &str,
    record: &BootRegistrationRecord,
) -> Result<WhetherTheDurabilityOrderingApplied, SubsystemFailure>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    let bytes =
        written_form::write(record).map_err(|_| SubsystemFailure::TheAnswerCouldNotBeRead)?;
    publish_durably(
        subsystem,
        &the_record_at(package_area, records::THE_BOOT_REGISTRATION_RECORD),
        &bytes,
    )
    .await
}

/// **`Rented-server runtime` K16's record, read back.**
pub async fn read_the_boot_registration_record<S>(
    subsystem: &mut Subsystem<S>,
    package_area: &str,
) -> Result<WhatARecordHeld<BootRegistrationRecord>, SubsystemFailure>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    read_a_record(
        subsystem,
        &the_record_at(package_area, records::THE_BOOT_REGISTRATION_RECORD),
        written_form::read::<BootRegistrationRecord>,
    )
    .await
}

/// **`Rented-server runtime` G10: the installation identity, written and made
/// durable before the pointer switch, not after it and not beside it.**
///
/// `State, identity and recovery` §13 item 2 records the window this closes from
/// one side: an interrupted first commit could leave a committed package with no
/// installation identity, which that document's §7 step 1 would read as absent
/// and act on by discarding what the local record holds. **With this ordering a
/// machine that has a pointer always has an identity.**
///
/// **The record's name is C15's own** — the same one
/// [`crate::reconnection::read_the_installation_identity`] reads — so the two
/// ends of this record are one record rather than two agreeing habits.
pub async fn make_the_installation_identity_durable<S>(
    subsystem: &mut Subsystem<S>,
    package_area: &str,
    installation: &Identity,
) -> Result<WhetherTheDurabilityOrderingApplied, SubsystemFailure>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    publish_durably(
        subsystem,
        &the_record_at(package_area, records::THE_INSTALLATION_IDENTITY),
        installation.as_text().as_bytes(),
    )
    .await
}

/// **`Rented-server runtime` Z2 and Z3, performed: the endpoint's path
/// recorded in the package area at placement, and its containing directory
/// made for the login alone.**
///
/// **Nothing did this before step 10c piece 12's first repair**, and the
/// machine's own program refuses to start a supervisor where the record is
/// absent — so a program this placement put on a machine could not have
/// started even where the pointer named it. **No test met it because every
/// test that ran the real program wrote the record by hand.**
///
/// **A record that already holds a path is kept, whatever path it holds.**
/// Z2's path is *"fixed for the installation, does not vary with the package
/// version"*, and that fixity is what puts two supervisors of two versions on
/// one endpoint where S3's interlock can act; rewriting it on a replacement
/// would put the new supervisor on a second endpoint beside a live first one.
/// The record is read as content, never as presence.
///
/// **Z3's directory is made with the login's permission alone, and an
/// existing one is given that permission again** — a directory another step or
/// a provider image widened is narrowed back rather than trusted. A thing at
/// that name that is not a directory is not made into one: the failure is the
/// subsystem's own, and it stops the placement before the commit.
///
/// `None` is *the record already stood*; otherwise whether S20's ordering held
/// for the one write this made.
pub async fn record_the_endpoints_path<S>(
    subsystem: &mut Subsystem<S>,
    package_area: &str,
) -> Result<Option<WhetherTheDurabilityOrderingApplied>, SubsystemFailure>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    let record = the_record_at(package_area, records::THE_ENDPOINTS_RECORDED_PATH);
    if let Some(standing) = read_a_records_bytes(subsystem, &record).await? {
        if !String::from_utf8_lossy(&standing).trim().is_empty() {
            return Ok(None);
        }
    }

    let directory = the_record_at(
        package_area,
        smoozy_runtime::the_program::THE_ENDPOINTS_OWN_DIRECTORY,
    );
    const THE_LOGIN_ALONE: u32 = 0o700;
    const WHAT_KIND_OF_THING: u32 = 0o170_000;
    const A_DIRECTORY: u32 = 0o040_000;
    if let Err(why) = subsystem.make_directory(directory.as_bytes()).await {
        // The subsystem's version 3 names no *already exists*, so the make's
        // refusal is asked about rather than read: a directory standing there
        // is Z3's directory, and anything else is the refusal it was.
        let stands = subsystem.lstat(directory.as_bytes()).await;
        let is_a_directory = matches!(
            stands,
            Ok(ref found) if found
                .permissions
                .is_some_and(|mode| mode & WHAT_KIND_OF_THING == A_DIRECTORY)
        );
        if !is_a_directory {
            return Err(why);
        }
    }
    subsystem
        .set_permissions(directory.as_bytes(), THE_LOGIN_ALONE)
        .await?;

    publish_durably(
        subsystem,
        &record,
        smoozy_runtime::the_program::the_endpoints_path_under(package_area).as_bytes(),
    )
    .await
    .map(Some)
}

/// **`Rented-server runtime` G11: the active-package pointer's target, read as a
/// value.**
///
/// **The link is read rather than followed**, so what comes back is the name the
/// pointer holds and never a statement that something is there — which is the
/// whole of *nothing about the commit is inferred from a directory or a file
/// being there*. A resolving read would answer with the staging directory
/// instead, and `Rented-server runtime` G31a's rule about which recorded
/// descriptor is the active one would then read the wrong one.
///
/// **A pointer that is not there is a machine with no active package**, which is
/// the never-held answer and not a failure.
pub async fn the_active_pointers_value<S>(
    subsystem: &mut Subsystem<S>,
    package_area: &str,
) -> Result<Option<String>, SubsystemFailure>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    match subsystem
        .read_link(the_active_package_pointer_at(package_area).as_bytes())
        .await
    {
        Ok(target) => Ok(Some(String::from_utf8_lossy(&target).into_owned())),
        Err(SubsystemFailure::TheServerRefused(StatusCode::NoSuchFile)) => Ok(None),
        Err(why) => Err(why),
    }
}

/// **`Rented-server runtime` G7's deciding write, performed: the single rename
/// of the pointer.**
///
/// *"This arrangement has one write: the boot registration is written to start
/// whatever the active-package pointer names, rather than to name a package
/// version."* So the registration's content does not change when the version
/// does, and **the commit is the single rename of the pointer**, which is atomic
/// by `rename(2)`'s own words.
///
/// **The staged link is made under a name of its own and renamed over the
/// pointer**, because a link cannot be repointed in place and a
/// remove-then-make would be the two writes `Rented-server runtime` G7 exists to
/// remove. **The rename is S16's extension where it is announced**, which is the
/// one operation that replaces an existing name in a single step; the base
/// rename refuses an existing target, observed.
///
/// **`Rented-server runtime` G14's restore is this same call in the other
/// direction**, and it has no separate function for that reason.
pub async fn rename_the_pointer_to<S>(
    subsystem: &mut Subsystem<S>,
    package_area: &str,
    target: &str,
) -> Result<(), SubsystemFailure>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    let pointer = the_active_package_pointer_at(package_area);
    let staging = the_staging_name_for(&pointer);

    match subsystem.remove(staging.as_bytes()).await {
        Ok(()) => {}
        Err(why) if why.is_the_path_not_being_there() => {}
        Err(why) => return Err(why),
    }
    subsystem
        .plant_a_link(target.as_bytes(), staging.as_bytes())
        .await?;

    let announced = subsystem.announcement().announces(extension::POSIX_RENAME);
    match which_operation(Intent::ReplaceIntended, announced) {
        Operation::ThePosixRenameExtension => {
            subsystem
                .posix_rename(staging.as_bytes(), pointer.as_bytes())
                .await
        }
        Operation::RemoveTheFinalNameThenTheBaseRename | Operation::TheBaseRename => {
            match subsystem.remove(pointer.as_bytes()).await {
                Ok(()) => {}
                Err(why) if why.is_the_path_not_being_there() => {}
                Err(why) => return Err(why),
            }
            subsystem
                .rename(staging.as_bytes(), pointer.as_bytes())
                .await
        }
    }
}

/// **The bytes at a path, read as content, with absence separated from
/// refusal.**
///
/// **This is what K15's read-back is made of**, and it returns bytes rather than
/// a parsed record because a registration's content is whatever Smoozy wrote
/// there and is compared byte for byte. `None` is the path not being there,
/// which the subsystem separates from a refusal in one request by two different
/// status codes.
pub async fn read_a_records_bytes<S>(
    subsystem: &mut Subsystem<S>,
    at: &str,
) -> Result<Option<Vec<u8>>, SubsystemFailure>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    let handle = match subsystem.open_to_read(at.as_bytes()).await {
        Ok(handle) => handle,
        Err(SubsystemFailure::TheServerRefused(StatusCode::NoSuchFile)) => return Ok(None),
        Err(why) => return Err(why),
    };
    let read = subsystem
        .read_at(&handle, 0, THE_MOST_ONE_OF_THESE_RECORDS_MAY_HOLD)
        .await;
    subsystem.close(&handle).await?;
    Ok(Some(read?))
}

/// One durable record, read over the subsystem and read back through
/// `Rented-server runtime`'s own written form.
///
/// **The reader is handed in rather than reached for through a trait bound**,
/// which is the shape [`crate::bootstrap`] already uses for the same four
/// records.
async fn read_a_record<T, S>(
    subsystem: &mut Subsystem<S>,
    at: &str,
    read_it_back: fn(&[u8]) -> Result<T, written_form::TheRecordCouldNotBeRead>,
) -> Result<WhatARecordHeld<T>, SubsystemFailure>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    let handle = match subsystem.open_to_read(at.as_bytes()).await {
        Ok(handle) => handle,
        Err(SubsystemFailure::TheServerRefused(StatusCode::NoSuchFile)) => {
            return Ok(WhatARecordHeld::Nothing)
        }
        Err(why) => return Err(why),
    };
    let read = subsystem
        .read_at(&handle, 0, THE_MOST_ONE_OF_THESE_RECORDS_MAY_HOLD)
        .await;
    subsystem.close(&handle).await?;
    let bytes = read?;
    Ok(the_record_in(&bytes, read_it_back))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_records_staging_name_sits_beside_it_and_is_drawn_from_class_twos_alphabet() {
        let staging = the_staging_name_for("/opt/smoozy/package/package-phase");
        assert!(staging.starts_with("/opt/smoozy/package/"));
        assert!(staging
            .rsplit('/')
            .next()
            .expect("it has a last component")
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte == b'-'));
    }

    /// **Step 10c piece 12's first fault, held shut: the artifact's staging
    /// location is no other name in the package area** — not the pointer, not
    /// the pointer's own staging name (which it was), not any record and not
    /// any record's staging name.
    #[test]
    fn a_staged_package_can_be_no_other_name_in_the_package_area() {
        use smoozy_runtime::the_program::THE_FIXED_ENTRY_POINT_NAME;
        use smoozy_state::identities::{Identity, IdentityKind};

        let area = "/home/someone/smoozy/package";
        let attempt = Identity::issue(IdentityKind::Operation).expect("randomness");
        let staged = the_staging_location_for(area, &attempt);

        let mut every_other_name = vec![
            THE_FIXED_ENTRY_POINT_NAME,
            records::THE_ENDPOINTS_RECORDED_PATH,
            records::THE_PHASE_RECORD,
            records::THE_BOOT_REGISTRATION_RECORD,
            records::THE_INSTALLATION_IDENTITY,
            records::THE_PACKAGE_TRANSACTION_CLAIM,
            records::THE_AREA_THE_HOLD_RECORDS_SIT_IN,
            records::THE_SETUP_IN_PROGRESS_STATE,
            records::THE_TELEGRAM_ROLE_STATE,
            smoozy_runtime::the_program::THE_ENDPOINTS_OWN_DIRECTORY,
            // `Job files and outputs` L1's record of the root, which
            // `Server Setup` R7b writes into the same area.
            "root",
        ]
        .into_iter()
        .map(|name| the_record_at(area, name))
        .collect::<Vec<_>>();
        let their_staging_names = every_other_name
            .iter()
            .map(|name| the_staging_name_for(name))
            .collect::<Vec<_>>();
        every_other_name.extend(their_staging_names);

        for other in &every_other_name {
            assert_ne!(&staged, other, "the staged package would be {other}");
        }
        // **The composition that deleted the program on the rental**, named so
        // that nobody brings it back.
        assert_ne!(
            staged,
            the_staging_name_for(&the_active_package_pointer_at(area)),
            "the staged package is the pointer's own staging name again"
        );
        // Beside the pointer, in §1.1's class-2 alphabet.
        let (directory, name) = staged.rsplit_once('/').expect("it has a directory");
        assert_eq!(directory, area);
        assert!(name
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-'));
        // One attempt, one path; two attempts, two paths.
        assert_eq!(staged, the_staging_location_for(&format!("{area}/"), &attempt));
        assert_ne!(
            staged,
            the_staging_location_for(
                area,
                &Identity::issue(IdentityKind::Operation).expect("randomness")
            )
        );
    }

    #[test]
    fn a_record_is_reached_by_the_one_name_that_record_has() {
        assert_eq!(
            the_record_at("/opt/smoozy/package", records::THE_PHASE_RECORD),
            "/opt/smoozy/package/package-phase"
        );
    }

    /// **A record is replaced at every phase, so its publication is
    /// replace-intended** — and that is what makes S16's extension the
    /// operation where it is announced and S17's declared degradation the one
    /// where it is not.
    /// **S20's scoping, both ways round — and the second arm is the one a code
    /// round had to put back.**
    ///
    /// A server that does not announce the flush is served without the ordering
    /// and the answer says so. **The refusal that stood here instead made the
    /// placement's very first act fail on such a machine**, with no route left,
    /// which is not what the clause says.
    #[test]
    fn a_server_offering_no_flush_is_served_and_the_answer_says_the_ordering_did_not_apply() {
        assert_eq!(
            whether_the_durability_ordering_applies(true),
            WhetherTheDurabilityOrderingApplied::TheStagedBytesWereMadeDurableFirst
        );
        assert_eq!(
            whether_the_durability_ordering_applies(false),
            WhetherTheDurabilityOrderingApplied::TheServerOffersNoFlushSoTheOrderingDidNotApply
        );
    }

    #[test]
    fn a_records_publication_is_replace_intended_on_both_routes() {
        assert_eq!(
            which_operation(Intent::ReplaceIntended, true),
            Operation::ThePosixRenameExtension
        );
        assert_eq!(
            which_operation(Intent::ReplaceIntended, false),
            Operation::RemoveTheFinalNameThenTheBaseRename
        );
    }
}
