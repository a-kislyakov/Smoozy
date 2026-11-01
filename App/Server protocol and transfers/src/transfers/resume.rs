//! §7.4 — a resume never trusts a partial it has not verified.
//!
//! **S11: the standard tool's own documentation says why this clause exists, in
//! its own words.** Documented — `sftp(1)`, macOS 26.5, dated December 6, 2024,
//! read 2026-08-17. `reget` is *"Resume download of remote-path. Equivalent to
//! get with the -a flag set"*, and `get -a` is *"attempt to resume partial
//! transfers of existing files. Note that resumption assumes that any partial
//! copy of the local file matches the remote copy. If the remote file contents
//! differ from the partial local copy then the resultant file is likely to be
//! corrupt."* **So resume in both directions exists in the tool present at both
//! ends, and it is unverified by construction.** A corrupt file produced this
//! way carries no error and no marker — Row 281's exact failure arriving by the
//! route that looks safest.
//!
//! **S14: resume itself is exact, and it was observed in both directions.**
//! 119,424,000 bytes where 157,286,400 minus 37,862,400 was owed, and
//! 436,786,462 where 669,966,622 minus 233,180,160 was owed, with matching
//! checksums; and on a second provider an interrupted transfer resumed in
//! **12.85 seconds against a from-scratch control of 30.97**. *"The partial can
//! be held off the final name, which is the clause the spec leans on."* **The
//! two conditions that came with it are why S12 does not lean on a tool**: one
//! box's `rsync` accepted the resume flag and resumed nothing on the sending
//! direction, re-sending in full, and the partial directory is not
//! deterministic after a hard kill.
//!
//! **Everything in this file decides and nothing acts.** The digest of the
//! staged prefix is computed by [`super::moving`]; what is here is which
//! prefix to compute it over, and what to do with each answer.

use super::integrity::WhatTheComparisonSaid;
use super::{Checkpoint, SourceReading, StartOverCause, TransferRecord};

/// **S12 steps 1 and 2**: what a resume verifies, before it adds a byte.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatToVerify {
    /// **Step 1**, where no staging path exists. **There is nothing staged at
    /// all**, so the transfer starts from zero and **nothing started over**:
    /// this is the ordinary first attempt.
    ///
    /// **Separated from the value below on 2026-09-03.** The two were one, so a
    /// transfer already past its first attempt reported a start-over that did
    /// not happen, with a cause naming a prefix that never existed — and at the
    /// first attempt the checkpoints were not cleared, so the record could name
    /// a high-water mark the disk never reached.
    NothingBecauseThereIsNoStagingPathAtAll,
    /// **Step 2**, where there is staged material and **no checkpoint at or
    /// below its size**. There is a staged object and none of it can be carried
    /// on, which **is** a start-over: the attempt number is incremented and the
    /// old path is abandoned to the sweep.
    NothingBecauseTheStagedMaterialCannotBeCarriedOn,
    /// **Step 3.** Compute the digest of the staged prefix up to this
    /// checkpoint and compare it with the checkpoint's recorded digest.
    ThePrefixUpTo(Checkpoint),
    /// **S15's first bullet.** A record that says *publication pending* does
    /// not enter S12 on a resume; publication is retried instead. *"A staged
    /// file that is already complete with the final name absent is therefore
    /// finished rather than left where it sits."*
    NothingBecausePublicationIsPending,
}

/// What the staged size read from the machine, and the record, together say.
///
/// **S10: the staged size read from the machine is what the bytes actually
/// are; the record is what was accounted for. Where they differ, neither is
/// taken on trust.**
///
/// **S10a: which way the record is wrong depends on how the transfer ended, and
/// it was observed both ways.** On a 4 GiB transfer held near 384 KB/s:
///
/// | how it ended | the record said | the disk actually held | the record was |
/// |---|---|---|---|
/// | hard kill at 37 MB | 37,552,128 | 37,847,040 | **294,912 behind** |
/// | clean stop at 2.14 GB | 2,140,700,672 | 2,140,798,976 | **98,304 behind** |
/// | connection killed under a moving transfer, at 2.30 GB | 2,303,393,792 | 2,303,229,952 | **163,840 ahead** |
///
/// **A kill leaves the record behind the disk**, because bytes reached the far
/// end after the last record write. **A dropped connection leaves the record
/// ahead of the disk, because writes are pipelined** — the loop counts a chunk
/// as written when it hands it over, and the connection died before some of
/// those chunks landed.
///
/// **That is why this takes the smaller of the two and neither on its own.**
/// Had the resume trusted the record after the drop it would have written
/// **163,840 bytes past the end of the staged file**, left a hole, and found
/// out only from the end-to-end digest **after moving the remaining two
/// gigabytes**. *"The `min` is not caution; it is the difference between a
/// resume and two gigabytes of wasted transfer."*
pub fn what_to_verify(record: &TransferRecord, staged_size_on_disk: Option<u64>) -> WhatToVerify {
    if record.publication_pending.is_some() {
        return WhatToVerify::NothingBecausePublicationIsPending;
    }
    let Some(staged) = staged_size_on_disk else {
        // Step 1: where no staging path exists, the transfer starts from zero —
        // and nothing started over, because nothing was staged.
        return WhatToVerify::NothingBecauseThereIsNoStagingPathAtAll;
    };
    // The smaller of the two, and neither on its own.
    let trustworthy = staged.min(record.accounted_for());
    match record.highest_checkpoint_at_or_below(trustworthy) {
        Some(checkpoint) => WhatToVerify::ThePrefixUpTo(checkpoint.clone()),
        // Step 2: where there is none, the transfer starts from zero into a
        // fresh staging path. **There was staged material and none of it can be
        // carried on**, which is the start-over the value above is not.
        None => WhatToVerify::NothingBecauseTheStagedMaterialCannotBeCarriedOn,
    }
}

/// **S12 steps 4 and 5**: what happens after the comparison.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResumePlan {
    /// **Step 4.** Carry on from that checkpoint and discard whatever sat above
    /// it. **Bytes between the checkpoint and the staged size are re-sent**,
    /// which bounds the waste at one checkpoint interval.
    CarryOnFrom(u64),
    /// **Step 1's own outcome, and it is not a start-over.** Nothing was
    /// staged, so the attempt number does not move, no cause is reported, and
    /// G4a's mark is not set. **The checkpoints are cleared all the same**: a
    /// record carrying a checkpoint with no staged file behind it would name a
    /// high-water mark the disk never reached, and S12 step 2 would take it at
    /// the next resume.
    StartFromZeroWithNothingStaged,
    /// **Step 5.** The attempt number is incremented and the transfer starts
    /// over from zero into the staging path that number synthesises; the
    /// superseded attempt's path is abandoned and its number retained so the
    /// sweep can reach it. **It is never repaired in place.**
    StartOverFromZero(StartOverCause),
    /// The record was already complete and awaiting publication.
    RetryThePublication,
}

/// Steps 4 and 5, decided.
pub fn after_verifying(what: &WhatToVerify, said: WhatTheComparisonSaid) -> ResumePlan {
    match what {
        WhatToVerify::NothingBecausePublicationIsPending => ResumePlan::RetryThePublication,
        WhatToVerify::NothingBecauseThereIsNoStagingPathAtAll => {
            ResumePlan::StartFromZeroWithNothingStaged
        }
        WhatToVerify::NothingBecauseTheStagedMaterialCannotBeCarriedOn => {
            ResumePlan::StartOverFromZero(StartOverCause::TheStagedPrefixDidNotVerify)
        }
        WhatToVerify::ThePrefixUpTo(checkpoint) => match said {
            WhatTheComparisonSaid::TheyAgree => ResumePlan::CarryOnFrom(checkpoint.offset),
            WhatTheComparisonSaid::TheyDisagree => {
                ResumePlan::StartOverFromZero(StartOverCause::TheStagedPrefixDidNotVerify)
            }
        },
    }
}

/// **S13: resuming also requires that the source has not changed under the
/// transfer, and the two directions are answered by different means.**
///
/// **The two ways the upload gate can fail have two different outcomes**, and
/// this is the correction of 2026-08-31 — the clause used to say that any
/// unconfirmed source starts the transfer over, which is not what `File
/// Browser` answers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheSourceSaid {
    /// The four values match the four recorded.
    ItHasNotChanged,
    /// **The source has changed.** On an upload the transfer does not start
    /// over: **it stops and is reported**, the record says *stopped* with its
    /// reason, the staged material is removed, and the user is told the file
    /// changed while it was being sent and offered the new version. *"A silent
    /// restart is the wrong answer here rather than a lesser one."* On a
    /// download it is G4a's second start-over cause.
    ItChanged,
    /// **Merely unconfirmed without having changed** — the reading could not be
    /// taken at all. *"Where it is unavailable the resume simply does not
    /// happen and the transfer starts over"*, with no claim made about the
    /// user's file.
    ItCouldNotBeConfirmed,
}

/// What the four-value reading says, where one can be taken at all.
pub fn compare_the_source(
    recorded: Option<&SourceReading>,
    read_now: Option<&SourceReading>,
) -> WhatTheSourceSaid {
    match (recorded, read_now) {
        (Some(recorded), Some(now)) if recorded.is_the_same_file_as(now) => {
            WhatTheSourceSaid::ItHasNotChanged
        }
        (Some(_), Some(_)) => WhatTheSourceSaid::ItChanged,
        _ => WhatTheSourceSaid::ItCouldNotBeConfirmed,
    }
}

/// What the source gate does to a resume, per direction.
///
/// **This is the whole of S13's corrected supply**, and the asymmetry is the
/// point: a changed source stops an upload and starts a download over.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheSourceGateDoes {
    /// Go on to S12's prefix verification.
    TheResumeMayProceed,
    /// **A start-over and not an ending**: it is none of S26's five, the
    /// transfer is not failed, and the carry-on of Rows 199 and 209 is not
    /// broken.
    StartOver(StartOverCause),
    /// **S26's second row, with its reason.** The upload direction alone.
    StopAndReportThatTheSourceChanged,
}

pub fn what_the_source_gate_does(
    direction: super::Direction,
    said: WhatTheSourceSaid,
) -> WhatTheSourceGateDoes {
    match (direction, said) {
        (_, WhatTheSourceSaid::ItHasNotChanged) => WhatTheSourceGateDoes::TheResumeMayProceed,
        (super::Direction::Up, WhatTheSourceSaid::ItChanged) => {
            WhatTheSourceGateDoes::StopAndReportThatTheSourceChanged
        }
        (super::Direction::Down, WhatTheSourceSaid::ItChanged) => {
            WhatTheSourceGateDoes::StartOver(StartOverCause::TheSourceChangedUnderADownload)
        }
        (_, WhatTheSourceSaid::ItCouldNotBeConfirmed) => {
            WhatTheSourceGateDoes::StartOver(StartOverCause::TheStagedPrefixDidNotVerify)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::integrity::Digest;
    use super::super::publication::{Intent, PublicationPreState};
    use super::super::{Direction, StableFileIdentity, TransferRecord};
    use super::*;
    use smoozy_state::identities::{Identity, IdentityKind};

    fn a_record_with_checkpoints(offsets: &[u64]) -> TransferRecord {
        let issue = |kind| Identity::issue(kind).expect("random bytes");
        let mut record = TransferRecord::arranged(
            issue(IdentityKind::Transfer),
            Direction::Up,
            issue(IdentityKind::LocalServer),
            Some(issue(IdentityKind::Installation)),
            "/local/scene.blend".to_string(),
            "/jobs/scene.blend".to_string(),
            "/jobs/.smoozy-staging",
            4_294_967_296,
            issue(IdentityKind::Run),
        );
        for offset in offsets {
            record.checkpoints.push(Checkpoint {
                offset: *offset,
                digest_of_everything_up_to_it: Digest::of(&offset.to_be_bytes()),
            });
        }
        record
    }

    #[test]
    fn a_record_ahead_of_the_disk_is_not_trusted_and_the_hole_is_not_written() {
        // S10a's third row, which is the one that costs two gigabytes. The
        // record says 2,303,393,792 and the disk holds 2,303,229,952; the
        // checkpoint taken must be one the disk actually reaches.
        let record = a_record_with_checkpoints(&[2_303_000_000, 2_303_393_792]);
        let what = what_to_verify(&record, Some(2_303_229_952));
        assert_eq!(
            what,
            WhatToVerify::ThePrefixUpTo(record.checkpoints[0].clone())
        );
    }

    #[test]
    fn a_record_behind_the_disk_takes_the_record_and_re_sends_the_difference() {
        // S10a's first row: a kill leaves the record behind the disk, because
        // bytes reached the far end after the last record write. The waste is
        // bounded at one checkpoint interval, which is step 4's own bound.
        let record = a_record_with_checkpoints(&[37_552_128]);
        let what = what_to_verify(&record, Some(37_847_040));
        assert_eq!(
            what,
            WhatToVerify::ThePrefixUpTo(record.checkpoints[0].clone())
        );
    }

    #[test]
    fn no_staging_path_at_all_starts_from_zero_and_is_not_a_start_over() {
        // The first half of the separation of 2026-09-03. Nothing is staged, so
        // nothing started over: a transfer already past its first attempt used
        // to report a start-over that did not happen, with a cause naming a
        // prefix that never existed.
        let record = a_record_with_checkpoints(&[1_000]);
        let what = what_to_verify(&record, None);
        assert_eq!(what, WhatToVerify::NothingBecauseThereIsNoStagingPathAtAll);
        assert_eq!(
            after_verifying(&what, WhatTheComparisonSaid::TheyAgree),
            ResumePlan::StartFromZeroWithNothingStaged
        );
    }

    #[test]
    fn staged_material_that_cannot_be_carried_on_is_a_start_over() {
        // The second half: step 2, where there is no checkpoint at or below the
        // staged size. There **is** a staged object and none of it can be used,
        // so the attempt number moves and the old path goes to the sweep.
        let record = a_record_with_checkpoints(&[1_000_000]);
        let what = what_to_verify(&record, Some(4_096));
        assert_eq!(
            what,
            WhatToVerify::NothingBecauseTheStagedMaterialCannotBeCarriedOn
        );
        assert_eq!(
            after_verifying(&what, WhatTheComparisonSaid::TheyAgree),
            ResumePlan::StartOverFromZero(StartOverCause::TheStagedPrefixDidNotVerify)
        );
    }

    #[test]
    fn the_two_ways_a_transfer_starts_from_zero_are_two_values() {
        // The fault in one sentence: one value for both meant a first attempt
        // and an abandoned prefix were the same answer, and only one of them is
        // a start-over.
        let record = a_record_with_checkpoints(&[1_000_000]);
        assert_ne!(
            what_to_verify(&record, None),
            what_to_verify(&record, Some(4_096))
        );
    }

    #[test]
    fn a_publication_pending_record_does_not_enter_the_resume_at_all() {
        // S15's first bullet. A staged file that is already complete with the
        // final name absent is finished rather than left where it sits.
        let mut record = a_record_with_checkpoints(&[4_294_967_296]);
        record.publication_pending = Some(PublicationPreState {
            intent: Intent::ReplaceIntended,
            staging_path: record.staging_path.as_text(),
            final_path: record.final_path.clone(),
            staged_identity: StableFileIdentity::TheSupplierCouldNotAnswer,
            staged_byte_length: 4_294_967_296,
            staged_digest: Digest::of(b"the whole file"),
        });
        assert_eq!(
            what_to_verify(&record, Some(4_294_967_296)),
            WhatToVerify::NothingBecausePublicationIsPending
        );
        assert_eq!(
            after_verifying(
                &WhatToVerify::NothingBecausePublicationIsPending,
                WhatTheComparisonSaid::TheyAgree
            ),
            ResumePlan::RetryThePublication
        );
    }

    #[test]
    fn a_prefix_that_does_not_verify_starts_over_rather_than_being_repaired() {
        // S12 step 5. The alternative is the one OpenSSH warns about in its own
        // words — a resumed file that *"is likely to be corrupt"* with no error.
        let record = a_record_with_checkpoints(&[1_048_576]);
        let what = what_to_verify(&record, Some(1_048_576));
        assert_eq!(
            after_verifying(&what, WhatTheComparisonSaid::TheyDisagree),
            ResumePlan::StartOverFromZero(StartOverCause::TheStagedPrefixDidNotVerify)
        );
        assert_eq!(
            after_verifying(&what, WhatTheComparisonSaid::TheyAgree),
            ResumePlan::CarryOnFrom(1_048_576)
        );
    }

    #[test]
    fn a_changed_source_stops_an_upload_and_starts_a_download_over() {
        // S13's corrected supply, and the asymmetry is the whole of it. A
        // silent restart on the upload side re-sends a whole file over the
        // user's own line for a save the user made and says nothing about it.
        assert_eq!(
            what_the_source_gate_does(Direction::Up, WhatTheSourceSaid::ItChanged),
            WhatTheSourceGateDoes::StopAndReportThatTheSourceChanged
        );
        assert_eq!(
            what_the_source_gate_does(Direction::Down, WhatTheSourceSaid::ItChanged),
            WhatTheSourceGateDoes::StartOver(StartOverCause::TheSourceChangedUnderADownload)
        );
    }

    #[test]
    fn a_source_that_cannot_be_confirmed_starts_over_and_claims_nothing() {
        // *"with no claim made about the user's file"* — which is why this is
        // the prefix cause and not the changed-source one.
        assert_eq!(
            what_the_source_gate_does(Direction::Up, WhatTheSourceSaid::ItCouldNotBeConfirmed),
            WhatTheSourceGateDoes::StartOver(StartOverCause::TheStagedPrefixDidNotVerify)
        );
    }

    #[test]
    fn a_reading_that_could_not_be_taken_is_unconfirmed_rather_than_unchanged() {
        let recorded = SourceReading {
            size: 1,
            inode: "1".to_string(),
            modification_time: 1,
            change_time: 1,
        };
        assert_eq!(
            compare_the_source(Some(&recorded), None),
            WhatTheSourceSaid::ItCouldNotBeConfirmed
        );
        assert_eq!(
            compare_the_source(None, Some(&recorded)),
            WhatTheSourceSaid::ItCouldNotBeConfirmed
        );
        assert_eq!(
            compare_the_source(Some(&recorded), Some(&recorded)),
            WhatTheSourceSaid::ItHasNotChanged
        );
    }
}
