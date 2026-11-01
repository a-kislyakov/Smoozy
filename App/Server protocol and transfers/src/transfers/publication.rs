//! §7.5 — publication is the last step, and it is one step.
//!
//! **S15: publication is a rename of the staging path onto the final path, and
//! it is the only moment the final name changes.** Nothing else in a transfer
//! touches the final name — not a create, not a truncate, not a touch, not a
//! permission change.
//!
//! **S16: on the machine, the operation is the file-transfer subsystem's
//! POSIX-rename extension, and the base operation will not do.** Observed:
//! `posix-rename@openssh.com` of a 3,145,728-byte staging file onto an existing
//! 1,200,000-byte final file returned status **0 `Success`**; the published
//! file was 3,145,728 bytes, its digest matched the source, **its inode had
//! changed**, and the staging name was gone. The base `SSH_FXP_RENAME` over an
//! existing target returned status **4 `Failure`**.
//!
//! **S19: what the Mac's own manual says about that operation, on the user's
//! own end.** Documented — `rename(2)`, macOS 26.5, dated June 3, 2021, read
//! 2026-08-17: *"The rename() system call guarantees that an instance of new
//! will always exist, even if the system should crash in the middle of the
//! operation."* That is a crash guarantee in the vendor's own words. **It says
//! nothing about the durability of the staged file's contents before the
//! rename**, which is S20's, and **nothing about Windows or about a desktop
//! Linux computer**, neither of which this project can reach.
//!
//! **How likely S17's degradation is at all: no machine this project can meet
//! takes it.** `posix-rename@openssh.com` has been announced since OpenSSH 4.9
//! — documented, that release's own announcement of 2008-03-31 — and the oldest
//! server on record here is 8.9p1. **S17 stays written**, because it costs
//! nothing and is the honest answer if a server ever does withhold the
//! extension; a builder should not treat it as an ordinary route to be
//! exercised.

use super::integrity::Digest;
use super::StableFileIdentity;

/// **S15a: every publication carries a caller-declared intent, and the intent
/// chooses the operation. Two intents and no third.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Intent {
    /// The caller has established that a file at the final name is to be
    /// replaced. The operation is S16's extension.
    ReplaceIntended,
    /// The caller requires that no file at the final name is overwritten, and
    /// the operation is the **base rename**, whose refusal of an existing
    /// target is exactly the backstop wanted.
    CreateIntended,
}

/// Which operation an intent chooses, and what a missing extension does to it.
///
/// **A create-intended publication never takes S17's route**, because removing
/// the final name first would destroy the very file the intent exists to refuse
/// to overwrite — and it would do so on exactly the machines where the
/// extension is absent, in exactly the two cases the backstop is for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    /// **S16.** The extension, over an existing target.
    ThePosixRenameExtension,
    /// **S15a and S17.** The base rename, in every case for a create-intended
    /// publication, announced extension or not.
    TheBaseRename,
    /// **S17's declared degradation, replace-intended only.** The staging file
    /// is created, **the final name is removed, and the base rename is
    /// performed**. No part-arrived bytes ever reach the final name and no
    /// hybrid is ever produced, so Row 281 is kept; what is lost is the
    /// replacement's indivisibility. **That window is recoverable and not a
    /// loss**, because the staged file is whole, S9's record names both paths,
    /// and S15's pre-state is written before the removal — so a connection that
    /// meets the final name absent knows Smoozy removed it and retries
    /// publication rather than reading the absence as ordinary.
    RemoveTheFinalNameThenTheBaseRename,
}

/// **The one place the intent and the announcement decide the operation.**
///
/// *"Nothing else is offered as a fallback, and in particular nothing writes to
/// the final name and hopes."*
pub fn which_operation(intent: Intent, the_extension_is_announced: bool) -> Operation {
    match (intent, the_extension_is_announced) {
        (Intent::CreateIntended, _) => Operation::TheBaseRename,
        (Intent::ReplaceIntended, true) => Operation::ThePosixRenameExtension,
        (Intent::ReplaceIntended, false) => Operation::RemoveTheFinalNameThenTheBaseRename,
    }
}

/// **S15's durable pre-state, written before the final name is touched.**
///
/// The transfer record is put into **publication pending**, naming both the
/// staging path and the final path and recording the complete staged object's
/// stable file identity, byte length and SHA-256 digest.
///
/// **Where the supplier cannot answer, the record says the identity is
/// unavailable and publication still proceeds on the byte length and digest** —
/// content evidence, which rests on no name — **and every branch below that
/// needed the identity refuses to claim success rather than guessing.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicationPreState {
    pub intent: Intent,
    pub staging_path: String,
    pub final_path: String,
    pub staged_identity: StableFileIdentity,
    pub staged_byte_length: u64,
    pub staged_digest: Digest,
}

/// What can be read back of the final object at recovery, or after the rename.
///
/// **Every field is an option because a route may not answer.** The byte length
/// always answers on the file-transfer subsystem; the identity and the at-rest
/// digest answer where their command-channel or platform routes do.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WhatTheFinalObjectReadsAs {
    pub exists: bool,
    pub byte_length: Option<u64>,
    pub identity: Option<String>,
    pub at_rest_digest: Option<Digest>,
}

/// **S15's second bullet**, and it never converts absence into success.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryVerdict {
    /// **Only an exact match with the staged-object evidence durably recorded
    /// before rename proves that the expected object crossed** and permits this
    /// outcome.
    ThePublicationHappened,
    /// **A missing or different final object records *staging removed outside
    /// publication* and starts a fresh transfer from zero.**
    StartAFreshTransferFromZero,
    /// **Where the proof cannot be composed at all** — a recorded value with no
    /// route to read its counterpart now — recovery never records `published`
    /// and starts a fresh transfer from zero, **the mismatch outcome rather
    /// than a new one**.
    TheProofCouldNotBeComposedSoStartAFresh,
    /// **A transfer-aware user deletion has already recorded *staging removed
    /// by user* before removing the path**, so recovery takes that explicit
    /// branch without inspecting the final name.
    TheUserRemovedTheStaging,
}

/// **S15's recovery, for a publication whose staging path no longer exists.**
///
/// *"This is what distinguishes a crash after rename from a user deletion or an
/// unrelated cleanup."*
pub fn judge_the_recovery(
    pre_state: &PublicationPreState,
    the_user_had_recorded_a_removal: bool,
    final_object: &WhatTheFinalObjectReadsAs,
) -> RecoveryVerdict {
    if the_user_had_recorded_a_removal {
        return RecoveryVerdict::TheUserRemovedTheStaging;
    }
    if !final_object.exists {
        return RecoveryVerdict::StartAFreshTransferFromZero;
    }

    // The byte length always answers on the subsystem, so its absence is a
    // proof that cannot be composed rather than a mismatch.
    let Some(byte_length) = final_object.byte_length else {
        return RecoveryVerdict::TheProofCouldNotBeComposedSoStartAFresh;
    };
    if byte_length != pre_state.staged_byte_length {
        return RecoveryVerdict::StartAFreshTransferFromZero;
    }

    match &pre_state.staged_identity {
        // Where the identity was recorded, it is one of the values the
        // pre-state recorded and it must be compared.
        StableFileIdentity::The(recorded) => match &final_object.identity {
            Some(now) if now == recorded => {}
            Some(_) => return RecoveryVerdict::StartAFreshTransferFromZero,
            None => return RecoveryVerdict::TheProofCouldNotBeComposedSoStartAFresh,
        },
        // **Where the identity was recorded unavailable, the byte length and
        // SHA-256 digest are the recorded evidence and both must be read and
        // match, which is content proof resting on no name.**
        StableFileIdentity::TheSupplierCouldNotAnswer => match &final_object.at_rest_digest {
            Some(digest) if *digest == pre_state.staged_digest => {}
            Some(_) => return RecoveryVerdict::StartAFreshTransferFromZero,
            None => return RecoveryVerdict::TheProofCouldNotBeComposedSoStartAFresh,
        },
    }

    // Where the at-rest digest answers as well, it is compared too — every
    // value the pre-state recorded, against every counterpart there is a route
    // to read.
    if let Some(digest) = &final_object.at_rest_digest {
        if *digest != pre_state.staged_digest {
            return RecoveryVerdict::StartAFreshTransferFromZero;
        }
    }
    RecoveryVerdict::ThePublicationHappened
}

/// **S15's third bullet: the successful rename is checked before its outcome is
/// written.**
///
/// *"A readable value that does not match is a failed publication, not a
/// published file; a value with no route to read it back does not fail the
/// publication — the pre-state already carries the staged object's verified
/// byte length and digest, and the rename is one step — and the outcome records
/// which values were compared."*
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatWasCompared {
    pub byte_length: bool,
    pub identity: bool,
    pub at_rest_digest: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RenameCheck {
    ThePublicationStands(WhatWasCompared),
    /// A readable value did not match.
    ThePublicationFailed,
}

pub fn check_the_rename(
    pre_state: &PublicationPreState,
    final_object: &WhatTheFinalObjectReadsAs,
) -> RenameCheck {
    let mut compared = WhatWasCompared {
        byte_length: false,
        identity: false,
        at_rest_digest: false,
    };
    if let Some(length) = final_object.byte_length {
        if length != pre_state.staged_byte_length {
            return RenameCheck::ThePublicationFailed;
        }
        compared.byte_length = true;
    }
    if let (StableFileIdentity::The(recorded), Some(now)) =
        (&pre_state.staged_identity, &final_object.identity)
    {
        if recorded != now {
            return RenameCheck::ThePublicationFailed;
        }
        compared.identity = true;
    }
    if let Some(digest) = &final_object.at_rest_digest {
        if *digest != pre_state.staged_digest {
            return RenameCheck::ThePublicationFailed;
        }
        compared.at_rest_digest = true;
    }
    RenameCheck::ThePublicationStands(compared)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn a_pre_state(identity: StableFileIdentity) -> PublicationPreState {
        PublicationPreState {
            intent: Intent::ReplaceIntended,
            staging_path: "/jobs/.smoozy-staging/abc-1.smoozy-part".to_string(),
            final_path: "/jobs/scene.blend".to_string(),
            staged_identity: identity,
            staged_byte_length: 3_145_728,
            staged_digest: Digest::of(b"the staged bytes"),
        }
    }

    #[test]
    fn a_create_intended_publication_never_removes_the_final_name() {
        // S17's scoping, added because the degradation would destroy the very
        // file the intent exists to refuse to overwrite — on exactly the
        // machines where the extension is absent.
        assert_eq!(
            which_operation(Intent::CreateIntended, false),
            Operation::TheBaseRename
        );
        assert_eq!(
            which_operation(Intent::CreateIntended, true),
            Operation::TheBaseRename
        );
    }

    #[test]
    fn a_replace_intended_publication_takes_the_extension_where_it_is_announced() {
        assert_eq!(
            which_operation(Intent::ReplaceIntended, true),
            Operation::ThePosixRenameExtension
        );
        assert_eq!(
            which_operation(Intent::ReplaceIntended, false),
            Operation::RemoveTheFinalNameThenTheBaseRename
        );
    }

    #[test]
    fn an_absent_final_name_never_becomes_a_success() {
        // S15's second bullet, and the one thing it says outright: *"it never
        // converts absence into success."*
        let pre_state = a_pre_state(StableFileIdentity::The("inode-9".to_string()));
        assert_eq!(
            judge_the_recovery(&pre_state, false, &WhatTheFinalObjectReadsAs::default()),
            RecoveryVerdict::StartAFreshTransferFromZero
        );
    }

    #[test]
    fn content_proof_stands_where_the_identity_was_never_available() {
        // §12 item 15's own rule: publication proceeds on the recorded byte
        // length and digest, which rest on no name.
        let pre_state = a_pre_state(StableFileIdentity::TheSupplierCouldNotAnswer);
        let final_object = WhatTheFinalObjectReadsAs {
            exists: true,
            byte_length: Some(3_145_728),
            identity: None,
            at_rest_digest: Some(Digest::of(b"the staged bytes")),
        };
        assert_eq!(
            judge_the_recovery(&pre_state, false, &final_object),
            RecoveryVerdict::ThePublicationHappened
        );
    }

    #[test]
    fn a_proof_that_cannot_be_composed_starts_afresh_rather_than_claiming_success() {
        // The identity was unavailable and the digest cannot be read now, so
        // there is nothing to prove the object crossed.
        let pre_state = a_pre_state(StableFileIdentity::TheSupplierCouldNotAnswer);
        let final_object = WhatTheFinalObjectReadsAs {
            exists: true,
            byte_length: Some(3_145_728),
            identity: None,
            at_rest_digest: None,
        };
        assert_eq!(
            judge_the_recovery(&pre_state, false, &final_object),
            RecoveryVerdict::TheProofCouldNotBeComposedSoStartAFresh
        );
    }

    #[test]
    fn windows_own_documented_failure_state_ends_in_a_restart() {
        // §12 item 2: `ReplaceFileW`'s documented failure state — the replaced
        // file gone and the replacement under its original name — is met by
        // this branch as a mismatch, so it ends in a restart rather than in a
        // claimed publication.
        let pre_state = a_pre_state(StableFileIdentity::TheSupplierCouldNotAnswer);
        let the_replaced_file_is_gone = WhatTheFinalObjectReadsAs {
            exists: false,
            ..WhatTheFinalObjectReadsAs::default()
        };
        assert_eq!(
            judge_the_recovery(&pre_state, false, &the_replaced_file_is_gone),
            RecoveryVerdict::StartAFreshTransferFromZero
        );
    }

    #[test]
    fn a_user_deletion_takes_its_own_branch_without_inspecting_the_final_name() {
        let pre_state = a_pre_state(StableFileIdentity::TheSupplierCouldNotAnswer);
        assert_eq!(
            judge_the_recovery(&pre_state, true, &WhatTheFinalObjectReadsAs::default()),
            RecoveryVerdict::TheUserRemovedTheStaging
        );
    }

    #[test]
    fn the_rename_check_records_which_values_were_compared() {
        // S15's third bullet: a value with no route to read it back does not
        // fail the publication, and the outcome records which values were
        // compared rather than implying all three.
        let pre_state = a_pre_state(StableFileIdentity::TheSupplierCouldNotAnswer);
        let only_the_length = WhatTheFinalObjectReadsAs {
            exists: true,
            byte_length: Some(3_145_728),
            identity: None,
            at_rest_digest: None,
        };
        assert_eq!(
            check_the_rename(&pre_state, &only_the_length),
            RenameCheck::ThePublicationStands(WhatWasCompared {
                byte_length: true,
                identity: false,
                at_rest_digest: false,
            })
        );
    }

    #[test]
    fn a_readable_value_that_does_not_match_fails_the_publication() {
        let pre_state = a_pre_state(StableFileIdentity::The("inode-9".to_string()));
        let a_different_object = WhatTheFinalObjectReadsAs {
            exists: true,
            byte_length: Some(3_145_728),
            identity: Some("inode-11".to_string()),
            at_rest_digest: None,
        };
        assert_eq!(
            check_the_rename(&pre_state, &a_different_object),
            RenameCheck::ThePublicationFailed
        );
    }
}
