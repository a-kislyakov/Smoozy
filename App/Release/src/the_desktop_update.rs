//! **The desktop update file, composed and signed by the release, and
//! believed only where the desktop's own reading believes it** — `Desktop app
//! and delivery` E10a, E15 and E19, step 11's D8 (2026-10-25).
//!
//! **What the file is.** E10a: *"E15's manifest with E19's key statements"*,
//! one more file in each release, under [`THE_UPDATE_FILES_NAME`]. Inside it:
//! every trust statement the master keys have signed, oldest first; the
//! manifest — the release sequence, the version, the everyday key's identifier,
//! and one entry per release-matrix row with that row's artifact, its length and
//! its digest; and the everyday key's signature over the manifest.
//!
//! **Nothing about the file's shape is written here.** The manifest, the trust
//! statement, the container, their canonical bytes, the verification and the
//! length-and-digest check are all the desktop's, built at step 11's D3 in
//! `App/Desktop app and delivery/src/updating/`, and this file only supplies the
//! values, holds the order and calls that code. **A file the desktop's reading
//! refuses for any row is never written**, so there is nothing on the disk to
//! publish by mistake.
//!
//! **What signs what.** The release signs one thing: the manifest, with the
//! everyday key read from the keychain (`crate::secret`). **The trust
//! statements it carries were signed once, by a master key, on the day each was
//! made**, and the release only carries them: the master keys never come near a
//! release (E19's *"never resides in the service, hosting or build runner"*).
//! The newest statement is the one the desktop build carries; every earlier one
//! is [`THE_EARLIER_TRUST_STATEMENTS`].
//!
//! **Until the build carries the two master keys, no file is written at all**
//! ([`this_build_can_be_updated`]). No copy of Smoozy could believe one, and a
//! release that refused to be made for want of a key nothing can use yet would
//! stop every server-package fix along with it.
//!
//! **How a clause marker is written in this crate.** A marker is written with
//! its document at the site: `Desktop app and delivery` E10a, not a bare E10a.

use smoozy_desktop::updating::manifest::{AnEntry, UpdateManifest, THE_MANIFEST_FORMAT, THE_ROWS};
use smoozy_desktop::updating::trust::{ACarriedStatement, TheCarriedSet};
use smoozy_desktop::updating::verifying::{
    the_artifact_is_the_one_named, verify_an_update_file_against, ASignedStatement, TheUpdateFile,
    WhyTheArtifactIsNotTheOneNamed, WhyTheUpdateFileWasRefused,
};
use smoozy_desktop::updating::THE_MANIFEST_SIGNING_CONTEXT;
pub use smoozy_desktop::updating::THE_UPDATE_FILES_NAME;
use smoozy_server_protocol::release::a_signature_over;
use smoozy_server_protocol::transfers::integrity::Digest;

use crate::secret::TheUpdateSigningKey;

/// **Every trust statement signed before the one this build carries, oldest
/// first — none, because no statement has been signed yet.**
///
/// `Desktop app and delivery` E19: each statement *"is retained on hosting
/// indefinitely, so a copy that missed the overlap can move directly from any
/// shipped set to the current one"*. The desktop's reading admits them in
/// trust-sequence order, so a copy holding an old statement walks every later
/// one; a compromise statement it never saw would leave it refusing the next.
/// **So every update file carries every statement**, these and then the build's
/// own.
///
/// **When the build's statement is replaced**, the one it replaces is moved
/// here, at the end, as its bytes and its master-key signature — exactly as the
/// desktop carried it. A test holds them below the build's statement and in
/// order.
pub const THE_EARLIER_TRUST_STATEMENTS: &[ACarriedStatement] = &[];

/// **Whether a desktop update file is written at all: only once the build
/// carries a master key.**
///
/// A build that carries none verifies no file (`Desktop app and delivery`
/// E19's *"does not verify and therefore does not update"*), so a file made for
/// it would be believed by no copy anywhere. **The release then writes none, and
/// says so**; it does not read the everyday key, which nothing could check yet.
/// **From the day the master keys are carried, every release writes one**, and
/// a missing everyday key stops the release. **The rule is the desktop's own**
/// (`Desktop app and delivery` E10b), so a build that never checks and a
/// release that writes no file are decided by the same line.
pub fn this_build_can_be_updated(carried: &TheCarriedSet) -> bool {
    carried.carries_a_recovery_key()
}

/// **One row's artifact, as the release has read it.**
pub struct ARowsArtifact<'a> {
    /// The release-matrix row (`Desktop app and delivery` E4b), as the desktop
    /// spells it in `THE_ROWS`.
    pub row: &'a str,
    /// The name it is published under in the same release.
    pub file_name: &'a str,
    /// Its bytes, exactly as they are written into the release.
    pub bytes: &'a [u8],
}

/// **Why a desktop update file was not made, or not believed.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhyTheUpdateFileWasNotMade {
    /// A row that is not one of the release matrix's.
    NotARowOfTheMatrix { row: String },
    /// One row given twice, so which artifact it is offered would be a choice.
    ARowIsGivenTwice { row: String },
    /// **A row of the matrix with no artifact.** Every copy on that row would be
    /// told it is up to date while a newer version exists for the others.
    ARowIsNotGiven { row: String },
    /// A row names a file this release does not publish.
    ARowNamesNoFileOfThisRelease { row: String, name: String },
    /// **The build carries master keys and no trust statement**, so no
    /// everyday key is in force and nothing can be signed for it.
    ThisBuildCarriesNoTrustStatement,
    /// **The desktop's own reading refused the file for this row.** Whatever
    /// the reason, no copy on that row would update from it.
    ItIsRefused { row: String, why: WhyTheUpdateFileWasRefused },
    /// **The desktop's own length-and-digest check refused an artifact**: the
    /// bytes are not the ones the manifest names for that row.
    AnArtifactIsNotTheOneNamed {
        row: String,
        why: WhyTheArtifactIsNotTheOneNamed,
    },
}

impl std::fmt::Display for WhyTheUpdateFileWasNotMade {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WhyTheUpdateFileWasNotMade::NotARowOfTheMatrix { row } => write!(
                out,
                "{row} is not a row of the release matrix; the rows are {}",
                THE_ROWS.join(", ")
            ),
            WhyTheUpdateFileWasNotMade::ARowIsGivenTwice { row } => {
                write!(out, "the row {row} is given twice")
            }
            WhyTheUpdateFileWasNotMade::ARowIsNotGiven { row } => write!(
                out,
                "the row {row} has no artifact, so every copy of Smoozy on it would be told it \
                 is up to date while a newer version exists"
            ),
            WhyTheUpdateFileWasNotMade::ARowNamesNoFileOfThisRelease { row, name } => write!(
                out,
                "the row {row} names {name}, and this release publishes no file of that name \
                 (give it with --also)"
            ),
            WhyTheUpdateFileWasNotMade::ThisBuildCarriesNoTrustStatement => write!(
                out,
                "this build carries the update master keys and no trust statement naming an \
                 everyday key, so nothing can be signed for it"
            ),
            WhyTheUpdateFileWasNotMade::ItIsRefused { row, why } => write!(
                out,
                "the desktop's own reading refuses the update file for the row {row}: {why:?}"
            ),
            WhyTheUpdateFileWasNotMade::AnArtifactIsNotTheOneNamed { row, why } => write!(
                out,
                "the artifact for the row {row} is not the one the update file names: {why:?}"
            ),
        }
    }
}

/// **One entry per row of the matrix, in the matrix's order**, each measured
/// from the bytes that are published.
pub fn the_entries(artifacts: &[ARowsArtifact<'_>]) -> Result<Vec<AnEntry>, WhyTheUpdateFileWasNotMade> {
    for (at, artifact) in artifacts.iter().enumerate() {
        if !THE_ROWS.contains(&artifact.row) {
            return Err(WhyTheUpdateFileWasNotMade::NotARowOfTheMatrix {
                row: artifact.row.to_owned(),
            });
        }
        if artifacts[..at].iter().any(|before| before.row == artifact.row) {
            return Err(WhyTheUpdateFileWasNotMade::ARowIsGivenTwice {
                row: artifact.row.to_owned(),
            });
        }
    }
    THE_ROWS
        .iter()
        .map(|row| {
            let artifact = artifacts
                .iter()
                .find(|artifact| artifact.row == *row)
                .ok_or_else(|| WhyTheUpdateFileWasNotMade::ARowIsNotGiven {
                    row: (*row).to_owned(),
                })?;
            Ok(AnEntry {
                row: (*row).to_owned(),
                file_name: artifact.file_name.to_owned(),
                byte_length: artifact.bytes.len() as u64,
                digest: Digest::of(artifact.bytes).bytes(),
            })
        })
        .collect()
}

/// **Every statement the file carries: the earlier ones, then the build's
/// own.**
pub fn the_statements_it_carries(
    carried: &TheCarriedSet,
    earlier: &[ACarriedStatement],
) -> Result<Vec<ASignedStatement>, WhyTheUpdateFileWasNotMade> {
    let Some((bytes, signature)) = &carried.statement else {
        return Err(WhyTheUpdateFileWasNotMade::ThisBuildCarriesNoTrustStatement);
    };
    let mut out: Vec<ASignedStatement> = earlier
        .iter()
        .map(|statement| ASignedStatement {
            bytes: statement.bytes.to_vec(),
            its_recovery_signature: statement.its_recovery_signature,
        })
        .collect();
    out.push(ASignedStatement {
        bytes: bytes.clone(),
        its_recovery_signature: *signature,
    });
    Ok(out)
}

/// **Compose the manifest and sign it with the everyday key**, judging
/// nothing. [`compose_the_update_file`] is this and then the desktop's reading.
pub fn sign_the_update_file(
    statements: Vec<ASignedStatement>,
    release_sequence: u64,
    version: &str,
    entries: Vec<AnEntry>,
    key: &TheUpdateSigningKey,
) -> Vec<u8> {
    let manifest = UpdateManifest {
        manifest_format: THE_MANIFEST_FORMAT.into(),
        release_sequence,
        version: version.to_owned(),
        signing_keys: vec![key.identifier().to_owned()],
        entries,
    }
    .canonical_bytes();
    let signature = a_signature_over(key.secret().bytes(), THE_MANIFEST_SIGNING_CONTEXT, &manifest);
    TheUpdateFile {
        statements,
        manifest,
        manifest_signatures: vec![signature],
    }
    .canonical_bytes()
}

/// **What the desktop's own reading makes of a file, row by row.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatEveryRowIsOffered {
    pub release_sequence: u64,
    pub version: String,
    /// The everyday keys whose signatures the reading accepted.
    pub signed_by: Vec<String>,
    /// One per row of the matrix, in the matrix's order.
    pub entries: Vec<AnEntry>,
}

/// **The file read the way a copy of Smoozy on each row reads it**: through
/// `smoozy_desktop::updating::verifying::verify_an_update_file_against`, as a
/// copy that has accepted nothing yet and is held to the build's own floor.
/// **Every row must be offered an artifact.**
///
/// **It is read from every starting point a copy can have**: this build's own
/// statement, and each earlier statement as an older build carries it. A copy
/// of this build passes over the earlier statements as history, so reading the
/// file only as this build would never look at them — and they are in the file
/// for the older copies alone.
pub fn what_every_row_is_offered(
    carried: &TheCarriedSet,
    earlier: &[ACarriedStatement],
    file: &[u8],
) -> Result<WhatEveryRowIsOffered, WhyTheUpdateFileWasNotMade> {
    let mut older_builds: Vec<TheCarriedSet> = earlier
        .iter()
        .map(|statement| TheCarriedSet {
            statement: Some((statement.bytes.to_vec(), statement.its_recovery_signature)),
            ..carried.clone()
        })
        .collect();
    older_builds.push(carried.clone());
    let mut entries = Vec::new();
    let mut first = None;
    for row in THE_ROWS {
        let mut verified = None;
        for starting_from in &older_builds {
            verified = Some(
                verify_an_update_file_against(starting_from, None, file, row).map_err(|why| {
                    WhyTheUpdateFileWasNotMade::ItIsRefused {
                        row: row.to_owned(),
                        why,
                    }
                })?,
            );
        }
        // **The last reading is this build's own**, and its entry is the one kept.
        let verified = verified.expect("this build is always a starting point");
        let entry = verified.the_entry_for_this_row.clone().ok_or_else(|| {
            WhyTheUpdateFileWasNotMade::ARowIsNotGiven {
                row: row.to_owned(),
            }
        })?;
        entries.push(entry);
        first.get_or_insert(verified);
    }
    let first = first.expect("the matrix has rows");
    Ok(WhatEveryRowIsOffered {
        release_sequence: first.manifest.release_sequence,
        version: first.manifest.version,
        signed_by: first.signed_by,
        entries,
    })
}

/// **An artifact's bytes against the entry a row is offered**, through the
/// desktop's own `the_artifact_is_the_one_named` — the length first, then the
/// digest (`Desktop app and delivery` E16).
pub fn the_artifact_is_the_one_offered(
    entry: &AnEntry,
    bytes: &[u8],
) -> Result<(), WhyTheUpdateFileWasNotMade> {
    the_artifact_is_the_one_named(entry, bytes.len() as u64, &Digest::of(bytes)).map_err(|why| {
        WhyTheUpdateFileWasNotMade::AnArtifactIsNotTheOneNamed {
            row: entry.row.clone(),
            why,
        }
    })
}

/// **Compose, sign, and believe it the desktop's way before handing it back.**
///
/// The order: one entry per row, measured; every statement; the manifest
/// signed by the everyday key; **the whole file read by the desktop's own code
/// for every row**; and each row's artifact checked against the entry that
/// reading offers it. **Anything refused hands back no file.**
pub fn compose_the_update_file(
    carried: &TheCarriedSet,
    earlier: &[ACarriedStatement],
    release_sequence: u64,
    version: &str,
    artifacts: &[ARowsArtifact<'_>],
    key: &TheUpdateSigningKey,
) -> Result<(Vec<u8>, WhatEveryRowIsOffered), WhyTheUpdateFileWasNotMade> {
    let entries = the_entries(artifacts)?;
    let statements = the_statements_it_carries(carried, earlier)?;
    let file = sign_the_update_file(statements, release_sequence, version, entries, key);
    let offered = what_every_row_is_offered(carried, earlier, &file)?;
    for entry in &offered.entries {
        let artifact = artifacts
            .iter()
            .find(|artifact| artifact.row == entry.row)
            .expect("every row was given");
        the_artifact_is_the_one_offered(entry, artifact.bytes)?;
    }
    Ok((file, offered))
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use smoozy_desktop::updating::trust::{
        ARecoveryKey, AnOnlineKey, TrustStatement, WhichStatement, THE_RECOVERY_KEYS_THIS_BUILD_CARRIES,
        THE_STATEMENT_FORMAT, THE_STATEMENT_THIS_BUILD_CARRIES,
    };
    use smoozy_desktop::updating::verifying::TheUpdateFile;
    use smoozy_desktop::updating::THE_STATEMENT_SIGNING_CONTEXT;
    use smoozy_server_protocol::release::composing::test_only_keys::{
        a_key_pair_from_a_written_down_seed, AKeyPair,
    };
    use smoozy_server_protocol::release::SignatureBytes;

    // **Throwaway keys, made here from seeds written down here**, and so public
    // the moment they are written.
    pub(crate) fn master_a() -> AKeyPair {
        a_key_pair_from_a_written_down_seed(*b"release-side master key A, tests")
    }
    pub(crate) fn master_b() -> AKeyPair {
        a_key_pair_from_a_written_down_seed(*b"release-side master key B, tests")
    }
    pub(crate) const EVERYDAY_SEED: [u8; 32] = *b"release-side everyday key, tests";
    pub(crate) fn everyday() -> AKeyPair {
        a_key_pair_from_a_written_down_seed(EVERYDAY_SEED)
    }
    pub(crate) fn the_everyday_key() -> TheUpdateSigningKey {
        crate::secret::update_key_for_tests_only(EVERYDAY_SEED, "everyday-1")
    }

    pub(crate) fn a_statement(trust_sequence: u64, which: WhichStatement) -> TrustStatement {
        TrustStatement {
            statement_format: THE_STATEMENT_FORMAT.into(),
            trust_sequence,
            which,
            online_keys: vec![AnOnlineKey {
                identifier: "everyday-1".into(),
                public_key: everyday().public,
                first_release_sequence: 1,
            }],
            revoked_online_keys: Vec::new(),
            revoked_recovery_keys: Vec::new(),
        }
    }

    pub(crate) fn signed(by: &AKeyPair, statement: &TrustStatement) -> (Vec<u8>, SignatureBytes) {
        let bytes = statement.canonical_bytes();
        let signature = a_signature_over(&by.secret, THE_STATEMENT_SIGNING_CONTEXT, &bytes);
        (bytes, signature)
    }

    /// **A build as it will be on the day the Creator's keys are written in**:
    /// two master keys, and a statement master key A signed putting the
    /// everyday key in force.
    pub(crate) fn a_build_carrying_the_keys() -> TheCarriedSet {
        TheCarriedSet {
            recovery_keys: vec![
                ARecoveryKey {
                    identifier: "master-a".into(),
                    public_key: master_a().public,
                },
                ARecoveryKey {
                    identifier: "master-b".into(),
                    public_key: master_b().public,
                },
            ],
            statement: Some(signed(&master_a(), &a_statement(1, WhichStatement::Transition))),
            greatest_release_sequence_published: 0,
        }
    }

    /// One artifact per row, each its own bytes.
    pub(crate) fn every_rows_bytes() -> Vec<(&'static str, String, Vec<u8>)> {
        THE_ROWS
            .iter()
            .map(|row| {
                (
                    *row,
                    format!("Smoozy-{row}.artifact"),
                    format!("the artifact for {row}").into_bytes(),
                )
            })
            .collect()
    }

    fn as_artifacts<'a>(rows: &'a [(&'static str, String, Vec<u8>)]) -> Vec<ARowsArtifact<'a>> {
        rows.iter()
            .map(|(row, name, bytes)| ARowsArtifact {
                row,
                file_name: name,
                bytes,
            })
            .collect()
    }

    // ------------------------------------------- the two halves agree

    /// **The release composes a file and the desktop's own reading believes it
    /// for every row**, with every row's length and digest the published
    /// artifact's own.
    #[test]
    fn a_composed_file_is_believed_by_the_desktops_own_reading_on_every_row() {
        let carried = a_build_carrying_the_keys();
        let rows = every_rows_bytes();
        let (file, offered) = compose_the_update_file(
            &carried,
            THE_EARLIER_TRUST_STATEMENTS,
            1700,
            "0.2.0",
            &as_artifacts(&rows),
            &the_everyday_key(),
        )
        .expect("the file is composed");
        assert_eq!((offered.release_sequence, offered.version.as_str()), (1700, "0.2.0"));
        assert_eq!(offered.signed_by, ["everyday-1"]);

        // **Read again here, straight through the desktop's code**, not through
        // what compose handed back.
        for (row, name, bytes) in &rows {
            let verified = verify_an_update_file_against(&carried, None, &file, row)
                .unwrap_or_else(|why| panic!("{row} is refused: {why:?}"));
            assert_eq!(verified.manifest.release_sequence, 1700);
            assert_eq!(verified.manifest.version, "0.2.0");
            assert_eq!(verified.signed_by, ["everyday-1"]);
            let entry = verified.the_entry_for_this_row.expect("every row is offered");
            assert_eq!(&entry.row, row);
            assert_eq!(&entry.file_name, name);
            assert_eq!(entry.byte_length, bytes.len() as u64);
            assert_eq!(entry.digest, Digest::of(bytes).bytes());
            assert_eq!(
                the_artifact_is_the_one_named(&entry, bytes.len() as u64, &Digest::of(bytes)),
                Ok(())
            );
        }

        // The file carries the build's statement, and one manifest signature.
        let read = TheUpdateFile::read(&file).expect("it is an update file");
        assert_eq!(read.statements.len(), 1);
        assert_eq!(read.statements[0].bytes, carried.statement.as_ref().unwrap().0);
        assert_eq!(read.manifest_signatures.len(), 1);
        let manifest = UpdateManifest::read(&read.manifest).unwrap();
        assert_eq!(manifest.entries.len(), THE_ROWS.len());
    }

    /// **The kept record a believing copy writes stands on its own**, and the
    /// next release, one sequence higher, is believed by that copy too.
    #[test]
    fn a_copy_that_kept_one_release_believes_the_next() {
        let carried = a_build_carrying_the_keys();
        let rows = every_rows_bytes();
        let compose = |sequence| {
            compose_the_update_file(
                &carried,
                THE_EARLIER_TRUST_STATEMENTS,
                sequence,
                "0.2.0",
                &as_artifacts(&rows),
                &the_everyday_key(),
            )
            .unwrap()
            .0
        };
        let first = compose(1700);
        let kept = verify_an_update_file_against(&carried, None, &first, THE_ROWS[0])
            .unwrap()
            .the_trust_record_to_keep;
        let next = compose(1701);
        assert!(verify_an_update_file_against(&carried, Some(&kept), &next, THE_ROWS[0]).is_ok());
        // And the one before it is refused by that copy.
        let older = compose(1699);
        assert_eq!(
            verify_an_update_file_against(&carried, Some(&kept), &older, THE_ROWS[0]).map(|_| ()),
            Err(WhyTheUpdateFileWasRefused::ItsReleaseSequenceIsBelowTheFloorInForce)
        );
    }

    // ------------------------------------------- each refusal, both halves

    /// **A wrong everyday key**: one the build's statement does not name, signing
    /// under the identifier it does. The desktop refuses the file, and so does
    /// compose, which hands back nothing.
    #[test]
    fn a_wrong_everyday_key_is_refused_by_both_halves() {
        let carried = a_build_carrying_the_keys();
        let rows = every_rows_bytes();
        let wrong = crate::secret::update_key_for_tests_only(
            *b"an everyday key nobody vouched f",
            "everyday-1",
        );
        let statements = the_statements_it_carries(&carried, THE_EARLIER_TRUST_STATEMENTS).unwrap();
        let entries = the_entries(&as_artifacts(&rows)).unwrap();
        let file = sign_the_update_file(statements, 1700, "0.2.0", entries, &wrong);
        for row in THE_ROWS {
            assert_eq!(
                verify_an_update_file_against(&carried, None, &file, row).map(|_| ()),
                Err(WhyTheUpdateFileWasRefused::ASignatureByAKeyInForceDidNotVerify {
                    identifier: "everyday-1".into()
                })
            );
        }
        assert!(matches!(
            compose_the_update_file(
                &carried,
                THE_EARLIER_TRUST_STATEMENTS,
                1700,
                "0.2.0",
                &as_artifacts(&rows),
                &wrong
            ),
            Err(WhyTheUpdateFileWasNotMade::ItIsRefused {
                why: WhyTheUpdateFileWasRefused::ASignatureByAKeyInForceDidNotVerify { .. },
                ..
            })
        ));
        // **The right key under an identifier the statement does not give it**
        // is a manifest no key in force signed.
        let renamed = crate::secret::update_key_for_tests_only(EVERYDAY_SEED, "everyday-9");
        assert!(matches!(
            compose_the_update_file(
                &carried,
                THE_EARLIER_TRUST_STATEMENTS,
                1700,
                "0.2.0",
                &as_artifacts(&rows),
                &renamed
            ),
            Err(WhyTheUpdateFileWasNotMade::ItIsRefused {
                why: WhyTheUpdateFileWasRefused::NoKeyInForceSignedTheManifest,
                ..
            })
        ));
    }

    /// **A statement under a key the build does not carry**: the build's
    /// statement signed by a stranger rather than a master key. The desktop
    /// refuses the whole file, and compose hands back nothing.
    #[test]
    fn a_statement_under_a_key_the_build_does_not_carry_is_refused_by_both_halves() {
        let stranger = a_key_pair_from_a_written_down_seed(*b"a master key no build ever carri");
        let carried = TheCarriedSet {
            statement: Some(signed(&stranger, &a_statement(1, WhichStatement::Transition))),
            ..a_build_carrying_the_keys()
        };
        let rows = every_rows_bytes();
        let statements = the_statements_it_carries(&carried, THE_EARLIER_TRUST_STATEMENTS).unwrap();
        let entries = the_entries(&as_artifacts(&rows)).unwrap();
        let file = sign_the_update_file(statements, 1700, "0.2.0", entries, &the_everyday_key());
        use smoozy_desktop::updating::trust::WhyTheStatementWasRefused;
        assert_eq!(
            verify_an_update_file_against(&carried, None, &file, THE_ROWS[0]).map(|_| ()),
            Err(WhyTheUpdateFileWasRefused::AStatement(
                WhyTheStatementWasRefused::NoRecoveryKeyInForceSignedIt
            ))
        );
        assert!(matches!(
            compose_the_update_file(
                &carried,
                THE_EARLIER_TRUST_STATEMENTS,
                1700,
                "0.2.0",
                &as_artifacts(&rows),
                &the_everyday_key()
            ),
            Err(WhyTheUpdateFileWasNotMade::ItIsRefused {
                why: WhyTheUpdateFileWasRefused::AStatement(
                    WhyTheStatementWasRefused::NoRecoveryKeyInForceSignedIt
                ),
                ..
            })
        ));
        // **And an earlier statement under a stranger refuses the file too**,
        // however good the build's own: an older copy carrying it would believe
        // nothing. A copy of this build passes over it as history, which is why
        // the file is read from every starting point.
        let (bytes, its_recovery_signature) =
            signed(&stranger, &a_statement(0, WhichStatement::Transition));
        let leaked: &'static [u8] = Box::leak(bytes.into_boxed_slice());
        let earlier = [ACarriedStatement {
            bytes: leaked,
            its_recovery_signature,
        }];
        assert!(matches!(
            compose_the_update_file(
                &TheCarriedSet {
                    statement: None,
                    ..a_build_carrying_the_keys()
                },
                &earlier,
                1700,
                "0.2.0",
                &as_artifacts(&rows),
                &the_everyday_key()
            ),
            Err(WhyTheUpdateFileWasNotMade::ThisBuildCarriesNoTrustStatement)
        ));
        assert!(matches!(
            compose_the_update_file(
                &a_build_carrying_the_keys(),
                &earlier,
                1700,
                "0.2.0",
                &as_artifacts(&rows),
                &the_everyday_key()
            ),
            Err(WhyTheUpdateFileWasNotMade::ItIsRefused {
                why: WhyTheUpdateFileWasRefused::AStatement(_),
                ..
            })
        ));
    }

    /// **A length or a digest that is not its artifact's**: the bytes a copy
    /// fetches are not the ones the file names. The desktop's own check refuses
    /// each, and so does the release's, through the same function.
    #[test]
    fn a_length_or_digest_that_is_not_its_artifacts_is_refused_by_both_halves() {
        let carried = a_build_carrying_the_keys();
        let rows = every_rows_bytes();
        let (_, offered) = compose_the_update_file(
            &carried,
            THE_EARLIER_TRUST_STATEMENTS,
            1700,
            "0.2.0",
            &as_artifacts(&rows),
            &the_everyday_key(),
        )
        .unwrap();
        for (entry, (_, _, bytes)) in offered.entries.iter().zip(&rows) {
            let mut longer = bytes.clone();
            longer.push(b'!');
            assert_eq!(
                the_artifact_is_the_one_named(entry, longer.len() as u64, &Digest::of(&longer)),
                Err(WhyTheArtifactIsNotTheOneNamed::ItsLengthIsNotTheManifests {
                    named: bytes.len() as u64,
                    found: longer.len() as u64
                })
            );
            assert!(matches!(
                the_artifact_is_the_one_offered(entry, &longer),
                Err(WhyTheUpdateFileWasNotMade::AnArtifactIsNotTheOneNamed {
                    why: WhyTheArtifactIsNotTheOneNamed::ItsLengthIsNotTheManifests { .. },
                    ..
                })
            ));
            let mut changed = bytes.clone();
            changed[0] ^= 1;
            assert_eq!(
                the_artifact_is_the_one_named(entry, changed.len() as u64, &Digest::of(&changed)),
                Err(WhyTheArtifactIsNotTheOneNamed::ItsDigestIsNotTheManifests)
            );
            assert!(matches!(
                the_artifact_is_the_one_offered(entry, &changed),
                Err(WhyTheUpdateFileWasNotMade::AnArtifactIsNotTheOneNamed {
                    why: WhyTheArtifactIsNotTheOneNamed::ItsDigestIsNotTheManifests,
                    ..
                })
            ));
            assert_eq!(the_artifact_is_the_one_offered(entry, bytes), Ok(()));
        }
    }

    // ------------------------------------------- the rows

    #[test]
    fn every_row_of_the_matrix_once_and_no_other() {
        let rows = every_rows_bytes();
        let all = as_artifacts(&rows);
        assert_eq!(the_entries(&all).unwrap().len(), THE_ROWS.len());

        let missing = &all[1..];
        assert_eq!(
            the_entries(missing).map(|_| ()),
            Err(WhyTheUpdateFileWasNotMade::ARowIsNotGiven {
                row: THE_ROWS[0].into()
            })
        );
        let mut twice = as_artifacts(&rows);
        twice.push(ARowsArtifact {
            row: THE_ROWS[2],
            file_name: "again",
            bytes: b"again",
        });
        assert_eq!(
            the_entries(&twice).map(|_| ()),
            Err(WhyTheUpdateFileWasNotMade::ARowIsGivenTwice {
                row: THE_ROWS[2].into()
            })
        );
        let mut unknown = as_artifacts(&rows);
        unknown.push(ARowsArtifact {
            row: "macos-x86_64",
            file_name: "intel",
            bytes: b"intel",
        });
        assert_eq!(
            the_entries(&unknown).map(|_| ()),
            Err(WhyTheUpdateFileWasNotMade::NotARowOfTheMatrix {
                row: "macos-x86_64".into()
            })
        );
        // **A file the release itself signed with a row left out is refused by
        // the check's reading**, for that row.
        let carried = a_build_carrying_the_keys();
        let mut four = the_entries(&all).unwrap();
        four.remove(3);
        let statements = the_statements_it_carries(&carried, THE_EARLIER_TRUST_STATEMENTS).unwrap();
        let file = sign_the_update_file(statements, 1700, "0.2.0", four, &the_everyday_key());
        assert_eq!(
            what_every_row_is_offered(&carried, THE_EARLIER_TRUST_STATEMENTS, &file),
            Err(WhyTheUpdateFileWasNotMade::ARowIsNotGiven {
                row: THE_ROWS[3].into()
            })
        );
    }

    // ------------------------------------------- what this build carries

    /// **From 2026-11-01 this build carries the Creator's two master keys, so
    /// every release writes an update file**; a build carrying none writes
    /// none.
    #[test]
    fn the_release_writes_an_update_file_exactly_when_the_build_carries_master_keys() {
        assert_eq!(THE_RECOVERY_KEYS_THIS_BUILD_CARRIES.len(), 2);
        assert!(this_build_can_be_updated(&TheCarriedSet::this_build()));
        assert!(this_build_can_be_updated(&a_build_carrying_the_keys()));
        assert!(!this_build_can_be_updated(&TheCarriedSet {
            recovery_keys: Vec::new(),
            statement: None,
            greatest_release_sequence_published: 0,
        }));
    }

    /// **The earlier statements are below the build's own, in order, and
    /// believed under the master keys the build carries.** None today.
    #[test]
    fn the_earlier_statements_come_before_the_builds_own_and_in_order() {
        let mut last = None;
        for statement in THE_EARLIER_TRUST_STATEMENTS {
            let read = TrustStatement::read(statement.bytes).expect("a statement");
            if let Some(before) = last {
                assert!(read.trust_sequence > before);
            }
            last = Some(read.trust_sequence);
        }
        if let Some(earliest_newest) = last {
            let builds = THE_STATEMENT_THIS_BUILD_CARRIES
                .expect("earlier statements are only kept beside the build's own");
            assert!(
                TrustStatement::read(builds.bytes).unwrap().trust_sequence > earliest_newest
            );
        }
    }
}
