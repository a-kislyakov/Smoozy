//! **The update file read in order, and the artifact checked against it** —
//! E15, E16's digest and length, E19, and `State, identity and recovery` J7.
//!
//! **One file, and what is in it.** E10a: the desktop's signed update file is
//! one more file in each release, *"E15's manifest with E19's key
//! statements"*. [`TheUpdateFile`] carries every trust statement the release
//! keeps (E19: retained on hosting indefinitely), the manifest's canonical
//! bytes, and one signature per signing key the manifest names. **The
//! container is Build's and carries no trust of its own**: each document in it
//! is signed over its own bytes under its own context, and a container that was
//! tampered with fails those checks rather than one of its own.
//!
//! **The order, and each step is a gate on the next.**
//!
//! 1. **The chain starts** from the newer of the build's own statement and the
//!    kept one, the kept one judged first by `State, identity and recovery`
//!    J7.
//! 2. **The file's statements are taken in trust-sequence order**, and each one
//!    above the trust in force must be admitted or the whole file is refused.
//!    Statements at or below it are history and are not acted on. **Taking them
//!    in order is what lets a revocation count**: a statement the other
//!    recovery key signed, revoking a stolen one, is admitted before any later
//!    statement the stolen key signed, and that one is then refused.
//! 3. **The manifest**: read, then its signatures — each signature by a key the
//!    statement in force allows at the manifest's release sequence must verify,
//!    and at least one must exist. A signature naming a key not in force counts
//!    for nothing. **The manifest never supplies a key** (E19).
//! 4. **The release-sequence floor** (E15): below it is refused.
//! 5. **The row**: the entry for the row this copy runs on, or none.
//!
//! **A verification that does not pass is an update that did not happen**
//! (E15). Every refusal below is one; none of them is retried here, and none of
//! them writes anything.
//!
//! **What is written, and when, is not this file's to decide.** A verified
//! update hands back the trust record to keep. E15 has the release sequence
//! stored *"before publication"*, which is the replacement's moment, so the
//! write is the caller's — through [`keep`].
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this
//! crate's own plan, `Preparation/Architecture/Desktop app and delivery.md`.
//! Every other document's marker is written with that document at the site.

use smoozy_server_protocol::release::{a_signature_verifies, Canonical, Reading, SignatureBytes};
use smoozy_server_protocol::transfers::integrity::Digest;

use super::manifest::{AnEntry, UpdateManifest, WhyTheBytesWereNotRead};
use super::trust::{
    trust_sequence_of, what_the_record_is_worth, TheCarriedSet, TheTrustInForce,
    TheUpdateTrustRecord, WhatTheRecordIsWorth, WhyTheStatementWasRefused,
};
use super::THE_MANIFEST_SIGNING_CONTEXT;
use crate::record::{update_trust, RecordFailure};

/// The format of the update file's container.
pub const THE_UPDATE_FILE_FORMAT: &str = "smoozy-desktop-update-file-1";

/// One trust statement as the file carries it: the bytes that were signed and
/// the recovery signature over them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ASignedStatement {
    pub bytes: Vec<u8>,
    pub its_recovery_signature: SignatureBytes,
}

/// **The update file, every part of it.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheUpdateFile {
    pub statements: Vec<ASignedStatement>,
    pub manifest: Vec<u8>,
    /// One per signing key the manifest names, in the order it names them.
    pub manifest_signatures: Vec<SignatureBytes>,
}

impl TheUpdateFile {
    /// The file as one run of bytes, which is how a release serves it. **The
    /// release side's function** and the tests'.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut out = Canonical::new();
        out.text(THE_UPDATE_FILE_FORMAT)
            .u32(self.statements.len() as u32);
        for statement in &self.statements {
            out.run(&statement.bytes)
                .run(&statement.its_recovery_signature);
        }
        out.run(&self.manifest)
            .u32(self.manifest_signatures.len() as u32);
        for signature in &self.manifest_signatures {
            out.run(signature);
        }
        out.finish()
    }

    /// Read the file back. A wrong format, a signature that is not sixty-four
    /// bytes, a short read or a trailing byte all give nothing.
    pub fn read(bytes: &[u8]) -> Option<TheUpdateFile> {
        let mut reading = Reading::of(bytes);
        if reading.text()? != THE_UPDATE_FILE_FORMAT {
            return None;
        }
        let how_many = reading.u32()?;
        let mut statements = Vec::new();
        for _ in 0..how_many {
            statements.push(ASignedStatement {
                bytes: reading.run()?.to_vec(),
                its_recovery_signature: reading.run()?.try_into().ok()?,
            });
        }
        let manifest = reading.run()?.to_vec();
        let how_many = reading.u32()?;
        let mut manifest_signatures = Vec::new();
        for _ in 0..how_many {
            manifest_signatures.push(reading.run()?.try_into().ok()?);
        }
        reading.end()?;
        Some(TheUpdateFile {
            statements,
            manifest,
            manifest_signatures,
        })
    }
}

/// **Why an update file was refused.** Each one is an update that did not
/// happen (E15).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhyTheUpdateFileWasRefused {
    /// The bytes are not an update file of a format this build knows.
    ItIsNotAnUpdateFile,
    /// A statement's trust sequence could not be read, so the statements
    /// cannot be put in order.
    AStatementCannotBeRead,
    /// **Two statements carry one trust sequence.** Which one is the trust set
    /// would be a choice.
    TwoStatementsShareATrustSequence,
    /// A statement above the trust in force was not admitted.
    AStatement(WhyTheStatementWasRefused),
    /// The manifest's bytes are not a manifest this build reads.
    TheManifest(WhyTheBytesWereNotRead),
    /// **The manifest names a different number of signing keys than it
    /// carries signatures**, so no signature can be paired with its key.
    ItsSignaturesAreNotTheKeysItNames,
    /// **A key in force named a signature and the signature does not verify.**
    /// A forged or damaged signature, or a manifest changed after signing.
    ASignatureByAKeyInForceDidNotVerify { identifier: String },
    /// **No key in force at this release sequence signed it** — no statement
    /// was ever admitted, every key it names is unknown, revoked or not yet
    /// active, or the build carries no recovery key at all.
    NoKeyInForceSignedTheManifest,
    /// **Its release sequence is below the floor in force** (E15).
    ItsReleaseSequenceIsBelowTheFloorInForce,
}

/// **A verified update file, and everything after it takes its values from
/// here.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AVerifiedUpdate {
    pub manifest: UpdateManifest,
    /// **The entry for the row this copy runs on, or none** — a release with
    /// nothing for this system is a check that completed and found nothing.
    pub the_entry_for_this_row: Option<AnEntry>,
    /// The keys whose signatures were accepted.
    pub signed_by: Vec<String>,
    /// **What `State, identity and recovery` J6's desktop-update trust record
    /// becomes**, for the caller to write at the moment E15 names.
    pub the_trust_record_to_keep: TheUpdateTrustRecord,
}

/// **The whole reading, against what this build carries.**
pub fn verify_an_update_file(
    remembered: Option<&TheUpdateTrustRecord>,
    file: &[u8],
    row: &str,
) -> Result<AVerifiedUpdate, WhyTheUpdateFileWasRefused> {
    verify_an_update_file_against(&TheCarriedSet::this_build(), remembered, file, row)
}

/// **The same reading, against a carried set named by the caller** — the
/// tests, and the release's own check, which asks what a copy carrying a
/// given set would make of a file.
pub fn verify_an_update_file_against(
    carried: &TheCarriedSet,
    remembered: Option<&TheUpdateTrustRecord>,
    file: &[u8],
    row: &str,
) -> Result<AVerifiedUpdate, WhyTheUpdateFileWasRefused> {
    let file = TheUpdateFile::read(file).ok_or(WhyTheUpdateFileWasRefused::ItIsNotAnUpdateFile)?;

    let mut in_force = TheTrustInForce::at_the_start(carried, remembered);
    let mut ordered = Vec::with_capacity(file.statements.len());
    for statement in &file.statements {
        let at = trust_sequence_of(&statement.bytes)
            .ok_or(WhyTheUpdateFileWasRefused::AStatementCannotBeRead)?;
        ordered.push((at, statement));
    }
    ordered.sort_by_key(|(at, _)| *at);
    if ordered.windows(2).any(|pair| pair[0].0 == pair[1].0) {
        return Err(WhyTheUpdateFileWasRefused::TwoStatementsShareATrustSequence);
    }
    for (at, statement) in ordered {
        if in_force.statement.is_some() && at <= in_force.trust_sequence() {
            continue;
        }
        in_force = in_force
            .admit(&statement.bytes, &statement.its_recovery_signature)
            .map_err(WhyTheUpdateFileWasRefused::AStatement)?;
    }

    let manifest =
        UpdateManifest::read(&file.manifest).map_err(WhyTheUpdateFileWasRefused::TheManifest)?;
    if manifest.signing_keys.len() != file.manifest_signatures.len() {
        return Err(WhyTheUpdateFileWasRefused::ItsSignaturesAreNotTheKeysItNames);
    }
    let Some((statement, statement_bytes, statement_signature)) = &in_force.statement else {
        return Err(WhyTheUpdateFileWasRefused::NoKeyInForceSignedTheManifest);
    };
    let mut signed_by = Vec::new();
    for (identifier, signature) in manifest.signing_keys.iter().zip(&file.manifest_signatures) {
        let Some(key) = statement.the_key_that_may_sign(identifier, manifest.release_sequence)
        else {
            continue;
        };
        if !a_signature_verifies(
            &key.public_key,
            THE_MANIFEST_SIGNING_CONTEXT,
            &file.manifest,
            signature,
        ) {
            return Err(
                WhyTheUpdateFileWasRefused::ASignatureByAKeyInForceDidNotVerify {
                    identifier: identifier.clone(),
                },
            );
        }
        signed_by.push(identifier.clone());
    }
    if signed_by.is_empty() {
        return Err(WhyTheUpdateFileWasRefused::NoKeyInForceSignedTheManifest);
    }
    if manifest.release_sequence < in_force.the_release_sequence_floor {
        return Err(WhyTheUpdateFileWasRefused::ItsReleaseSequenceIsBelowTheFloorInForce);
    }

    let the_trust_record_to_keep = TheUpdateTrustRecord {
        greatest_accepted_release_sequence: manifest.release_sequence,
        greatest_accepted_trust_sequence: statement.trust_sequence,
        the_statement_accepted_at_that_trust_sequence: statement_bytes.clone(),
        its_recovery_signature: *statement_signature,
    };
    Ok(AVerifiedUpdate {
        the_entry_for_this_row: manifest.the_entry_for(row).cloned(),
        manifest,
        signed_by,
        the_trust_record_to_keep,
    })
}

/// **Why fetched bytes are not the artifact the manifest names.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhyTheArtifactIsNotTheOneNamed {
    ItsLengthIsNotTheManifests { named: u64, found: u64 },
    ItsDigestIsNotTheManifests,
}

/// **E16: the digest and length checked against the signed manifest, before
/// any platform verification.** The length first, because it is free.
///
/// **On Linux this is also the artifact's signature** (E16's *"the manifest
/// and artifact signatures against E19's versioned key set"*): the manifest's
/// signature covers this digest and this length, so an artifact that matches
/// them is the one an online key in force signed for.
pub fn the_artifact_is_the_one_named(
    entry: &AnEntry,
    byte_length: u64,
    digest: &Digest,
) -> Result<(), WhyTheArtifactIsNotTheOneNamed> {
    if byte_length != entry.byte_length {
        return Err(WhyTheArtifactIsNotTheOneNamed::ItsLengthIsNotTheManifests {
            named: entry.byte_length,
            found: byte_length,
        });
    }
    if digest.bytes() != entry.digest {
        return Err(WhyTheArtifactIsNotTheOneNamed::ItsDigestIsNotTheManifests);
    }
    Ok(())
}

/// **The kept trust record as it stands — `State, identity and recovery` J7
/// performed against the record.** A record whose statement will not verify is
/// discarded in the record, whole, and nothing is handed back; one that stands
/// is handed back for [`verify_an_update_file`].
pub fn the_kept_record_as_it_stands(
    connection: &rusqlite::Connection,
    carried: &TheCarriedSet,
) -> Result<Option<TheUpdateTrustRecord>, RecordFailure> {
    let kept = update_trust::read_the_trust_record(connection)?;
    if kept.is_none() {
        return Ok(None);
    }
    match what_the_record_is_worth(carried, kept.as_ref()) {
        WhatTheRecordIsWorth::ItsNumbersStand { .. } => Ok(kept),
        WhatTheRecordIsWorth::ItIsDiscarded => {
            update_trust::discard_the_trust_record(connection)?;
            Ok(None)
        }
    }
}

/// **Keep what was accepted**, replacing the record whole. The numbers and the
/// statement that carried them are written together and never one without the
/// other (`State, identity and recovery` J6).
pub fn keep(
    connection: &rusqlite::Connection,
    record: &TheUpdateTrustRecord,
) -> Result<(), RecordFailure> {
    update_trust::write_the_trust_record(connection, record)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::updating::manifest::tests::a_manifest;
    use crate::updating::manifest::{THE_DEBIAN_13_ROW, THE_MAC_ROW, THE_WINDOWS_ROW};
    use crate::updating::trust::tests::{
        a_build_with_both_recovery_keys, a_statement, an_online_key, online_1, online_2,
        recovery_a, recovery_b, signed,
    };
    use crate::updating::trust::{TrustStatement, WhichStatement};
    use smoozy_server_protocol::release::a_signature_over;
    use smoozy_server_protocol::release::composing::test_only_keys::{
        a_key_pair_from_a_written_down_seed, AKeyPair,
    };

    fn a_signed_statement(by: &AKeyPair, statement: &TrustStatement) -> ASignedStatement {
        let (bytes, its_recovery_signature) = signed(by, statement);
        ASignedStatement {
            bytes,
            its_recovery_signature,
        }
    }

    fn the_first() -> TrustStatement {
        a_statement(
            1,
            WhichStatement::Transition,
            vec![an_online_key("online-1", &online_1())],
        )
    }

    fn the_overlap() -> TrustStatement {
        a_statement(
            2,
            WhichStatement::Transition,
            vec![
                an_online_key("online-1", &online_1()),
                an_online_key("online-2", &online_2()),
            ],
        )
    }

    /// A file whose manifest, at `release_sequence`, is signed by each named
    /// key in turn.
    fn a_file(
        statements: Vec<ASignedStatement>,
        release_sequence: u64,
        signers: &[(&str, AKeyPair)],
    ) -> Vec<u8> {
        let names: Vec<&str> = signers.iter().map(|(name, _)| *name).collect();
        let manifest = a_manifest(release_sequence, &names).canonical_bytes();
        let manifest_signatures = signers
            .iter()
            .map(|(_, key)| a_signature_over(&key.secret, THE_MANIFEST_SIGNING_CONTEXT, &manifest))
            .collect();
        TheUpdateFile {
            statements,
            manifest,
            manifest_signatures,
        }
        .canonical_bytes()
    }

    fn verify(
        remembered: Option<&TheUpdateTrustRecord>,
        file: &[u8],
    ) -> Result<AVerifiedUpdate, WhyTheUpdateFileWasRefused> {
        verify_an_update_file_against(
            &a_build_with_both_recovery_keys(),
            remembered,
            file,
            THE_MAC_ROW,
        )
    }

    #[test]
    fn the_file_reads_back_as_it_was_composed_and_refuses_a_tail() {
        let file = TheUpdateFile {
            statements: vec![a_signed_statement(&recovery_a(), &the_first())],
            manifest: a_manifest(3, &["online-1"]).canonical_bytes(),
            manifest_signatures: vec![[7; 64]],
        };
        let bytes = file.canonical_bytes();
        assert_eq!(TheUpdateFile::read(&bytes).unwrap(), file);
        let mut tail = bytes;
        tail.push(0);
        assert!(TheUpdateFile::read(&tail).is_none());
    }

    #[test]
    fn a_genuine_file_is_accepted_with_the_entry_for_this_row_and_the_record_to_keep() {
        let file = a_file(
            vec![a_signed_statement(&recovery_a(), &the_first())],
            12,
            &[("online-1", online_1())],
        );
        let verified = verify(None, &file).unwrap();
        assert_eq!(verified.manifest.release_sequence, 12);
        assert_eq!(verified.signed_by, ["online-1"]);
        assert_eq!(verified.the_entry_for_this_row.unwrap().row, THE_MAC_ROW);
        let kept = verified.the_trust_record_to_keep;
        assert_eq!(
            (
                kept.greatest_accepted_release_sequence,
                kept.greatest_accepted_trust_sequence
            ),
            (12, 1)
        );
        // **The record it hands back stands on its own** under `State, identity and recovery` J7's judgement.
        assert!(matches!(
            what_the_record_is_worth(&a_build_with_both_recovery_keys(), Some(&kept)),
            WhatTheRecordIsWorth::ItsNumbersStand { .. }
        ));
        // A row with no entry: the check completed and found nothing for it.
        let none = verify_an_update_file_against(
            &a_build_with_both_recovery_keys(),
            None,
            &file,
            THE_WINDOWS_ROW,
        )
        .unwrap();
        assert!(none.the_entry_for_this_row.is_none());
        assert_eq!(
            verify_an_update_file_against(
                &a_build_with_both_recovery_keys(),
                None,
                &file,
                THE_DEBIAN_13_ROW
            )
            .unwrap()
            .the_entry_for_this_row
            .unwrap()
            .byte_length,
            10
        );
    }

    #[test]
    fn either_recovery_key_alone_is_enough_for_a_whole_file() {
        for signer in [recovery_a(), recovery_b()] {
            let file = a_file(
                vec![a_signed_statement(&signer, &the_first())],
                1,
                &[("online-1", online_1())],
            );
            assert!(verify(None, &file).is_ok());
        }
    }

    #[test]
    fn a_bad_manifest_signature_is_refused() {
        let statements = vec![a_signed_statement(&recovery_a(), &the_first())];
        let manifest = a_manifest(5, &["online-1"]).canonical_bytes();
        let mut signature =
            a_signature_over(&online_1().secret, THE_MANIFEST_SIGNING_CONTEXT, &manifest);
        signature[10] ^= 1;
        let file = TheUpdateFile {
            statements: statements.clone(),
            manifest: manifest.clone(),
            manifest_signatures: vec![signature],
        };
        assert_eq!(
            verify(None, &file.canonical_bytes()),
            Err(
                WhyTheUpdateFileWasRefused::ASignatureByAKeyInForceDidNotVerify {
                    identifier: "online-1".into()
                }
            )
        );
        // A manifest changed after it was signed.
        let good = a_signature_over(&online_1().secret, THE_MANIFEST_SIGNING_CONTEXT, &manifest);
        let changed = a_manifest(6, &["online-1"]).canonical_bytes();
        let file = TheUpdateFile {
            statements,
            manifest: changed,
            manifest_signatures: vec![good],
        };
        assert!(matches!(
            verify(None, &file.canonical_bytes()),
            Err(WhyTheUpdateFileWasRefused::ASignatureByAKeyInForceDidNotVerify { .. })
        ));
    }

    #[test]
    fn a_manifest_signed_by_no_key_in_force_is_refused_and_supplies_no_key_of_its_own() {
        // Signed by a key the statement does not name, under that key's own
        // claimed identifier.
        let stranger = a_key_pair_from_a_written_down_seed(*b"an online key nobody ever named.");
        let file = a_file(
            vec![a_signed_statement(&recovery_a(), &the_first())],
            5,
            &[("online-9", stranger)],
        );
        assert_eq!(
            verify(None, &file),
            Err(WhyTheUpdateFileWasRefused::NoKeyInForceSignedTheManifest)
        );
        // **Under a name the statement does give, it is a bad signature.**
        let file = a_file(
            vec![a_signed_statement(&recovery_a(), &the_first())],
            5,
            &[("online-1", stranger)],
        );
        assert!(matches!(
            verify(None, &file),
            Err(WhyTheUpdateFileWasRefused::ASignatureByAKeyInForceDidNotVerify { .. })
        ));
        // No statement at all: nothing is in force.
        let file = a_file(Vec::new(), 5, &[("online-1", online_1())]);
        assert_eq!(
            verify(None, &file),
            Err(WhyTheUpdateFileWasRefused::NoKeyInForceSignedTheManifest)
        );
    }

    #[test]
    fn a_statement_under_the_wrong_root_refuses_the_whole_file() {
        let somebody = a_key_pair_from_a_written_down_seed(*b"a root this build does not carry");
        let file = a_file(
            vec![a_signed_statement(&somebody, &the_first())],
            5,
            &[("online-1", online_1())],
        );
        assert_eq!(
            verify(None, &file),
            Err(WhyTheUpdateFileWasRefused::AStatement(
                WhyTheStatementWasRefused::NoRecoveryKeyInForceSignedIt
            ))
        );
    }

    /// **`State, identity and recovery` J10: the server package's release, or
    /// a statement signed as a server-package document, is no update file.**
    #[test]
    fn the_server_packages_root_and_release_are_refused_for_an_update() {
        use smoozy_server_protocol::release as server_package;
        // The server package's own container, offered whole.
        let theirs = server_package::verifying::AnOfferedRelease {
            the_key_set_statements_bytes: b"a key-set statement".to_vec(),
            its_root_signature: [1; 64],
            the_manifests_bytes: b"a manifest".to_vec(),
            its_online_signature: [2; 64],
        };
        assert_eq!(
            verify(None, &theirs.canonical_bytes()),
            Err(WhyTheUpdateFileWasRefused::ItIsNotAnUpdateFile)
        );
        // An update statement signed by a recovery key under the server
        // package's key-set context: not a signature here.
        let bytes = the_first().canonical_bytes();
        let as_a_server_document = a_signature_over(
            &recovery_a().secret,
            server_package::THE_KEY_SET_SIGNING_CONTEXT,
            &bytes,
        );
        let file = a_file(
            vec![ASignedStatement {
                bytes,
                its_recovery_signature: as_a_server_document,
            }],
            5,
            &[("online-1", online_1())],
        );
        assert_eq!(
            verify(None, &file),
            Err(WhyTheUpdateFileWasRefused::AStatement(
                WhyTheStatementWasRefused::NoRecoveryKeyInForceSignedIt
            ))
        );
        // A manifest signed under the server package's manifest context.
        let manifest = a_manifest(5, &["online-1"]).canonical_bytes();
        let file = TheUpdateFile {
            statements: vec![a_signed_statement(&recovery_a(), &the_first())],
            manifest_signatures: vec![a_signature_over(
                &online_1().secret,
                server_package::THE_MANIFEST_SIGNING_CONTEXT,
                &manifest,
            )],
            manifest,
        };
        assert!(matches!(
            verify(None, &file.canonical_bytes()),
            Err(WhyTheUpdateFileWasRefused::ASignatureByAKeyInForceDidNotVerify { .. })
        ));
    }

    #[test]
    fn a_release_sequence_going_backwards_is_refused() {
        let accepted = verify(
            None,
            &a_file(
                vec![a_signed_statement(&recovery_a(), &the_first())],
                30,
                &[("online-1", online_1())],
            ),
        )
        .unwrap()
        .the_trust_record_to_keep;
        let older = a_file(
            vec![a_signed_statement(&recovery_a(), &the_first())],
            29,
            &[("online-1", online_1())],
        );
        assert_eq!(
            verify(Some(&accepted), &older),
            Err(WhyTheUpdateFileWasRefused::ItsReleaseSequenceIsBelowTheFloorInForce)
        );
        // The same release again is not going backwards.
        let same = a_file(
            vec![a_signed_statement(&recovery_a(), &the_first())],
            30,
            &[("online-1", online_1())],
        );
        assert!(verify(Some(&accepted), &same).is_ok());
        // **A copy that remembers nothing is still held to the build's floor.**
        let build = TheCarriedSet {
            greatest_release_sequence_published: 30,
            ..a_build_with_both_recovery_keys()
        };
        assert_eq!(
            verify_an_update_file_against(&build, None, &older, THE_MAC_ROW),
            Err(WhyTheUpdateFileWasRefused::ItsReleaseSequenceIsBelowTheFloorInForce)
        );
        // **A damaged record does not hold the floor**, `State, identity and recovery` J7's discard: the
        // older genuine file is then judged on its own merits.
        let mut damaged = accepted.clone();
        damaged.its_recovery_signature[0] ^= 1;
        assert!(verify(Some(&damaged), &older).is_ok());
    }

    #[test]
    fn a_trust_sequence_going_backwards_moves_nothing() {
        // Kept: statement 5, naming online-1 alone.
        let five = a_statement(
            5,
            WhichStatement::Transition,
            vec![an_online_key("online-1", &online_1())],
        );
        let kept = verify(
            None,
            &a_file(
                vec![a_signed_statement(&recovery_a(), &five)],
                10,
                &[("online-1", online_1())],
            ),
        )
        .unwrap()
        .the_trust_record_to_keep;
        // Offered: an older, well-signed statement naming online-2, and a
        // manifest only online-2 signed. **The older statement is not acted on.**
        let three = a_statement(
            3,
            WhichStatement::Transition,
            vec![an_online_key("online-2", &online_2())],
        );
        let file = a_file(
            vec![a_signed_statement(&recovery_b(), &three)],
            11,
            &[("online-2", online_2())],
        );
        assert_eq!(
            verify(Some(&kept), &file),
            Err(WhyTheUpdateFileWasRefused::NoKeyInForceSignedTheManifest)
        );
        // Without the kept record the same file is genuine.
        assert!(verify(None, &file).is_ok());
    }

    #[test]
    fn a_copy_that_missed_the_overlap_moves_straight_to_the_newest_set() {
        let kept = verify(
            None,
            &a_file(
                vec![a_signed_statement(&recovery_a(), &the_first())],
                10,
                &[("online-1", online_1())],
            ),
        )
        .unwrap()
        .the_trust_record_to_keep;
        let newest = a_statement(
            3,
            WhichStatement::Transition,
            vec![an_online_key("online-2", &online_2())],
        );
        let file = a_file(
            vec![
                a_signed_statement(&recovery_a(), &the_first()),
                a_signed_statement(&recovery_b(), &newest),
                a_signed_statement(&recovery_a(), &the_overlap()),
            ],
            20,
            &[("online-2", online_2())],
        );
        let verified = verify(Some(&kept), &file).unwrap();
        assert_eq!(
            verified
                .the_trust_record_to_keep
                .greatest_accepted_trust_sequence,
            3
        );
        // **During the overlap a manifest carries both signatures**, and a copy
        // on the old set and a copy on the new one both accept it.
        let both = a_file(
            vec![
                a_signed_statement(&recovery_a(), &the_first()),
                a_signed_statement(&recovery_a(), &the_overlap()),
            ],
            15,
            &[("online-1", online_1()), ("online-2", online_2())],
        );
        let verified = verify(Some(&kept), &both).unwrap();
        assert_eq!(verified.signed_by, ["online-1", "online-2"]);
    }

    #[test]
    fn a_compromised_keys_signature_counts_for_nothing_after_its_statement() {
        let compromise = TrustStatement {
            revoked_online_keys: vec!["online-1".into()],
            ..a_statement(
                2,
                WhichStatement::Compromise,
                vec![an_online_key("online-2", &online_2())],
            )
        };
        let file = a_file(
            vec![
                a_signed_statement(&recovery_a(), &the_first()),
                a_signed_statement(&recovery_b(), &compromise),
            ],
            40,
            &[("online-1", online_1())],
        );
        assert_eq!(
            verify(None, &file),
            Err(WhyTheUpdateFileWasRefused::NoKeyInForceSignedTheManifest)
        );
        // The replacement's signature is good.
        let file = a_file(
            vec![
                a_signed_statement(&recovery_a(), &the_first()),
                a_signed_statement(&recovery_b(), &compromise),
            ],
            40,
            &[("online-2", online_2())],
        );
        assert!(verify(None, &file).is_ok());
    }

    /// **The statements are taken in order, so a revocation counts even where
    /// the file puts the thief's statement first.**
    #[test]
    fn a_stolen_recovery_keys_later_statement_is_refused_once_the_other_has_revoked_it() {
        let b_revokes_a = TrustStatement {
            revoked_recovery_keys: vec!["recovery-a".into()],
            ..a_statement(
                2,
                WhichStatement::Compromise,
                vec![an_online_key("online-1", &online_1())],
            )
        };
        let thief = TrustStatement {
            revoked_recovery_keys: vec!["recovery-a".into(), "recovery-b".into()],
            ..a_statement(
                3,
                WhichStatement::Compromise,
                vec![an_online_key("online-9", &online_2())],
            )
        };
        let file = a_file(
            vec![
                a_signed_statement(&recovery_a(), &thief),
                a_signed_statement(&recovery_b(), &b_revokes_a),
            ],
            9,
            &[("online-9", online_2())],
        );
        assert_eq!(
            verify(None, &file),
            Err(WhyTheUpdateFileWasRefused::AStatement(
                WhyTheStatementWasRefused::NoRecoveryKeyInForceSignedIt
            ))
        );
    }

    #[test]
    fn a_file_that_leaves_the_trust_set_a_choice_is_refused() {
        let other_one = a_statement(
            1,
            WhichStatement::Transition,
            vec![an_online_key("online-2", &online_2())],
        );
        let file = a_file(
            vec![
                a_signed_statement(&recovery_a(), &the_first()),
                a_signed_statement(&recovery_b(), &other_one),
            ],
            5,
            &[("online-1", online_1())],
        );
        assert_eq!(
            verify(None, &file),
            Err(WhyTheUpdateFileWasRefused::TwoStatementsShareATrustSequence)
        );
        // A signature with no key named for it.
        let manifest = a_manifest(5, &["online-1"]).canonical_bytes();
        let signature =
            a_signature_over(&online_1().secret, THE_MANIFEST_SIGNING_CONTEXT, &manifest);
        let file = TheUpdateFile {
            statements: vec![a_signed_statement(&recovery_a(), &the_first())],
            manifest,
            manifest_signatures: vec![signature, signature],
        };
        assert_eq!(
            verify(None, &file.canonical_bytes()),
            Err(WhyTheUpdateFileWasRefused::ItsSignaturesAreNotTheKeysItNames)
        );
    }

    /// **This build accepts no update file signed under keys it does not
    /// carry, however well signed.** From 2026-11-01 it carries the Creator's
    /// two master keys and its own statement at trust sequence 1, so a file
    /// under the test keys — here a newer statement, at 2 — is refused because
    /// no master key this build carries signed it.
    #[test]
    fn this_build_accepts_no_update_file_signed_under_keys_it_does_not_carry() {
        let file = a_file(
            vec![a_signed_statement(&recovery_a(), &the_overlap())],
            5,
            &[("online-1", online_1()), ("online-2", online_2())],
        );
        assert!(
            verify(None, &file).is_ok(),
            "the file is genuine under the test keys"
        );
        assert_eq!(
            verify_an_update_file(None, &file, THE_MAC_ROW),
            Err(WhyTheUpdateFileWasRefused::AStatement(
                WhyTheStatementWasRefused::NoRecoveryKeyInForceSignedIt
            ))
        );
    }

    #[test]
    fn the_artifacts_length_and_digest_are_the_manifests_or_it_is_not_the_artifact() {
        let entry = a_manifest(1, &["online-1"])
            .the_entry_for(THE_MAC_ROW)
            .unwrap()
            .clone();
        assert_eq!(
            the_artifact_is_the_one_named(&entry, 7, &Digest::of(b"the mac")),
            Ok(())
        );
        assert_eq!(
            the_artifact_is_the_one_named(&entry, 8, &Digest::of(b"the mac")),
            Err(WhyTheArtifactIsNotTheOneNamed::ItsLengthIsNotTheManifests { named: 7, found: 8 })
        );
        assert_eq!(
            the_artifact_is_the_one_named(&entry, 7, &Digest::of(b"the maC")),
            Err(WhyTheArtifactIsNotTheOneNamed::ItsDigestIsNotTheManifests)
        );
    }

    fn a_record_on_disk(name: &str) -> (crate::record::LocalRecord, std::path::PathBuf) {
        let folder = crate::scratch_folders::a_folder(format!(
            "smoozy-update-trust-{name}-{}",
            std::process::id()
        ));
        let path = folder.join("local-record.sqlite3");
        (crate::record::LocalRecord::open_at(&path).unwrap(), folder)
    }

    /// **`State, identity and recovery` J7 through the record itself**: what
    /// was kept stands, and a damaged one is discarded in the record, whole.
    #[test]
    fn a_kept_record_stands_in_the_record_and_a_damaged_one_is_discarded_there() {
        let (record, folder) = a_record_on_disk("j7");
        let connection = record.connection();
        let build = a_build_with_both_recovery_keys();
        assert_eq!(
            the_kept_record_as_it_stands(connection, &build).unwrap(),
            None
        );
        let accepted = verify(
            None,
            &a_file(
                vec![a_signed_statement(&recovery_b(), &the_first())],
                30,
                &[("online-1", online_1())],
            ),
        )
        .unwrap()
        .the_trust_record_to_keep;
        keep(connection, &accepted).unwrap();
        assert_eq!(
            the_kept_record_as_it_stands(connection, &build).unwrap(),
            Some(accepted.clone())
        );
        connection
            .execute(
                "UPDATE update_trust SET greatest_accepted_trust_sequence = 900",
                [],
            )
            .unwrap();
        assert_eq!(
            the_kept_record_as_it_stands(connection, &build).unwrap(),
            None
        );
        assert_eq!(
            update_trust::read_the_trust_record(connection).unwrap(),
            None,
            "the record is gone, not lowered"
        );
        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }
}
