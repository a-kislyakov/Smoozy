//! B10 — verification precedes execution at both ends of the transfer, and
//! both ends are the desktop's.
//!
//! **Three verifications, in this order, and each one is a gate on the next.**
//!
//! 1. **Before download** — the root-signed key-set statement, its
//!    non-rollback rules, the manifest signature by an allowed non-revoked
//!    online key, and the manifest's canonical digest.
//!    [`verify_before_download`].
//! 2. **Before upload** — the bytes came from an exact source the verified
//!    manifest named, and their byte length and digest are the selected
//!    artifact's. [`verify_what_was_downloaded`].
//! 3. **After upload, at rest, from the desktop** — the staged bytes are read
//!    back over the file-transfer subsystem and their byte length and digest
//!    computed in Smoozy's own loop, **before the artifact is made executable
//!    and before any byte of it is invoked**. [`read_back_at_rest`] and
//!    [`compare_at_rest`], then [`make_the_artifact_executable`].
//!
//! **What the machine end is worth is stated rather than overstated.** The
//! read-back is served by the machine's own file-transfer server, so it detects
//! corruption in transfer and at rest and **defends against nothing the machine
//! itself does**. No verifier running on a machine can exceed that machine's
//! honesty. The anchors are B9's offline root and the connection §3
//! authenticates.
//!
//! **A failure leaves the machine as it was.** A signature, digest, tuple or
//! non-rollback failure never touches the active pointer or the boot
//! registration, and **version equality never overrides it** — there is no
//! route **through this file** that reaches [`make_the_artifact_executable`]
//! without the at-rest comparison having agreed, because the token that
//! function takes has no other producer, no copy and no clone.
//!
//! **That is narrower than it used to read, and the difference matters.** It
//! said there was *no route* at all, and there is one: the subsystem's own
//! permission-setting operation is public, and its own comment says outright
//! that *"the ordering is not this method's to keep"*. **What this file holds is
//! its own route and not every route**, and a reader deciding what the type
//! guarantees has to know which.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Rented-server runtime` G26, G27, G29.
//! `State, identity and recovery` J6.
//!
//! **How a clause marker is written in this crate, and it is a rule rather
//! than a habit.** A marker with no document beside it means a clause of this
//! crate's own plan, `Preparation/Architecture/Server protocol and
//! transfers.md`. Every other document's marker is written with that
//! document — at the site, or in the file's own header list above.

use crate::subsystem::client::Subsystem;
use crate::subsystem::SubsystemFailure;
use crate::transfers::integrity::{compare, Digest, Running, WhatTheComparisonSaid};

use super::manifest::{
    ArtifactEntry, ReleaseManifest, TargetTuple, WhySelectionFailed, WhyTheBytesWereNotRead,
};
use super::trust::{
    admit_a_key_set_statement_against, KeySetStatement, TheBuiltInFloor, TheTrustRecord,
    WhyTheStatementWasRefused,
};
use super::{a_signature_verifies, PublicKeyBytes, SignatureBytes, THE_MANIFEST_SIGNING_CONTEXT};

/// **What hosting serves, and what the desktop is handed before it trusts
/// anything.**
///
/// **B9's statement and its root signature are distributed with the manifest**
/// and retained across missed releases, so a desktop need not have observed an
/// overlap: the statement carries the whole allowed set rather than a change to
/// one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnOfferedRelease {
    pub the_key_set_statements_bytes: Vec<u8>,
    pub its_root_signature: SignatureBytes,
    pub the_manifests_bytes: Vec<u8>,
    pub its_online_signature: SignatureBytes,
}

/// The format of the one file the four fields above are distributed in.
pub const THE_OFFERED_RELEASE_FORMAT: &str = "smoozy-server-package-release-1";

impl AnOfferedRelease {
    /// **The four fields as one run of bytes, which is how hosting serves
    /// them.**
    ///
    /// **B9 requires the statement and its root signature to be distributed
    /// with the manifest**, and one file is the smallest way to do that: a
    /// fetch that had to name four addresses could be answered with three of
    /// one release and one of another, and every combination would then have to
    /// be reasoned about. **The container is Build's own and it is not signed
    /// and carries no trust** — each of the two documents inside it is signed on
    /// its own, over its own bytes and under its own context, exactly as it was
    /// before, and a container that had been tampered with fails those checks
    /// rather than a check of its own.
    ///
    /// **The two documents are carried as the bytes that were signed**, never
    /// re-encoded, which is the same rule the parsed manifest is already held
    /// to.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut out = super::Canonical::new();
        out.text(THE_OFFERED_RELEASE_FORMAT)
            .run(&self.the_key_set_statements_bytes)
            .run(&self.its_root_signature)
            .run(&self.the_manifests_bytes)
            .run(&self.its_online_signature);
        out.finish()
    }

    /// Read the container back. **Nothing is guessed at**: a wrong format, a
    /// signature that is not sixty-four bytes, a short read or a trailing byte
    /// all give nothing.
    pub fn read(bytes: &[u8]) -> Option<AnOfferedRelease> {
        let mut reading = super::Reading::of(bytes);
        if reading.text()? != THE_OFFERED_RELEASE_FORMAT {
            return None;
        }
        let the_key_set_statements_bytes = reading.run()?.to_vec();
        let its_root_signature: SignatureBytes = reading.run()?.try_into().ok()?;
        let the_manifests_bytes = reading.run()?.to_vec();
        let its_online_signature: SignatureBytes = reading.run()?.try_into().ok()?;
        reading.end()?;
        Some(AnOfferedRelease {
            the_key_set_statements_bytes,
            its_root_signature,
            the_manifests_bytes,
            its_online_signature,
        })
    }
}

/// **Why a release was refused before a single byte was downloaded.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhyTheReleaseWasRefused {
    /// B9's own admission failed.
    TheKeySetStatement(WhyTheStatementWasRefused),
    /// The manifest's bytes are not a manifest this build reads.
    TheManifest(WhyTheBytesWereNotRead),
    /// **No key the statement allows at this release sequence signed it.** That
    /// covers a forged signature, a key that is not in the set, a key whose
    /// validity has not begun, and a key revoked at or before this sequence —
    /// and it is one answer because the desktop learns the same thing from all
    /// four: nobody who may sign this manifest did.
    NoAllowedKeySignedTheManifest,
    /// **Its release sequence is below the floor in force**, which is the
    /// greater of what this build carries and what the desktop remembers.
    ItsReleaseSequenceIsBelowTheFloorInForce,
    /// A digest was stated for the manifest and its bytes do not have it. **On
    /// continuation this is the check that says the manifest is the one the
    /// package-phase record was written against.**
    ItIsNotTheManifestThatWasStated,
    /// Selection could not name one artifact.
    TheSelection(WhySelectionFailed),
    /// **The selected artifact names a source that is not an HTTPS address.**
    ///
    /// B8 requires an *"exact **HTTPS** source or sources"*, and until this the
    /// scheme was held only by the release tool — **which no shipped copy of
    /// Smoozy runs**, so half of one clause was kept by whoever composes and the
    /// other half by nobody who consumes. It is refused here, before a byte is
    /// downloaded.
    ItsSelectedArtifactNamesASourceThatIsNotAnHttpsAddress { the_source: String },
}

/// **B8's *exact HTTPS source*, as the one test of it in this workspace.**
///
/// **It is deliberately the whole of the rule and no more.** What B8 asks is
/// that a source be an HTTPS address; which address is the Creator's, by B8a,
/// and nothing here derives, composes or corrects one.
pub fn an_https_address(source: &str) -> bool {
    source.starts_with("https://")
}

/// **A verified release, and everything downstream takes its values from
/// here.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AVerifiedRelease {
    pub manifest: ReleaseManifest,
    pub selected: ArtifactEntry,
    /// The digest of the manifest's canonical bytes — **the bytes that
    /// arrived**, never a re-encoding of the parsed value.
    pub manifest_digest: [u8; 32],
    /// Which allowed key's signature was accepted.
    pub signed_by: String,
    /// **What `State, identity and recovery` J6's server-package trust record
    /// becomes**, to be written durably before anything is downloaded.
    pub the_trust_record_to_keep: TheTrustRecord,
}

impl AVerifiedRelease {
    /// **The six values B1's verified release descriptor carries, with the
    /// requested package version.**
    ///
    /// `Rented-server runtime` G27 holds the descriptor itself and pins it by
    /// value in the package-phase record. **The two are not one type on
    /// purpose**: that crate takes the framing half of this module and none of
    /// this half, so a shared type would put this file's dependency tree back
    /// into the machine program. The field names below are that document's own,
    /// so the mapping is a transcription rather than a translation.
    pub fn the_release_descriptors_values(&self) -> TheDescriptorsValues {
        TheDescriptorsValues {
            manifest_format: self.manifest.manifest_format.clone(),
            verified_release_sequence: self.manifest.release_sequence,
            requested_package_version: self.manifest.package_version.clone(),
            manifest_digest: self.manifest_digest,
            artifact_identity: self.selected.artifact_identity.clone(),
            artifact_byte_length: self.selected.byte_length,
            artifact_digest: self.selected.digest,
        }
    }
}

/// The seven values `Rented-server runtime` G27's descriptor is made of.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheDescriptorsValues {
    pub manifest_format: String,
    pub verified_release_sequence: u64,
    pub requested_package_version: String,
    pub manifest_digest: [u8; 32],
    pub artifact_identity: String,
    pub artifact_byte_length: u64,
    pub artifact_digest: [u8; 32],
}

/// **B10's first verification, in the order the clause states it.**
///
/// `stated_manifest_digest` is the digest the caller already holds — the
/// package-phase record's, on a continuation. **Where there is none this is a
/// first placement and there is nothing to be equal to**; the digest is still
/// computed and carried, because it is what the record will hold.
pub fn verify_before_download(
    root: &PublicKeyBytes,
    remembered: Option<&TheTrustRecord>,
    offered: &AnOfferedRelease,
    detected: &TargetTuple,
    stated_manifest_digest: Option<[u8; 32]>,
) -> Result<AVerifiedRelease, WhyTheReleaseWasRefused> {
    verify_before_download_against(
        root,
        remembered,
        offered,
        detected,
        stated_manifest_digest,
        TheBuiltInFloor::this_build(),
    )
}

/// **The same verification, against a floor named by the caller.**
///
/// [`verify_before_download`] is this with the floor B9 has the build carry.
/// **What this is for is a caller that is not a desktop** — the release tool's
/// outside check, which has to be able to ask *would a desktop that has already
/// accepted generation N and sequence M refuse this?* A check that held no floor
/// could not ask it, and the ordinary user is exactly the case that holds one.
pub fn verify_before_download_against(
    root: &PublicKeyBytes,
    remembered: Option<&TheTrustRecord>,
    offered: &AnOfferedRelease,
    detected: &TargetTuple,
    stated_manifest_digest: Option<[u8; 32]>,
    floor: TheBuiltInFloor,
) -> Result<AVerifiedRelease, WhyTheReleaseWasRefused> {
    let admitted = admit_a_key_set_statement_against(
        root,
        remembered,
        &offered.the_key_set_statements_bytes,
        &offered.its_root_signature,
        floor,
    )
    .map_err(WhyTheReleaseWasRefused::TheKeySetStatement)?;

    let manifest = ReleaseManifest::read(&offered.the_manifests_bytes)
        .map_err(WhyTheReleaseWasRefused::TheManifest)?;

    // **There is always a floor** — B9's built-in pair is what a copy that has
    // never accepted a release is held to, and a remembered one only ever
    // raises it.
    if manifest.release_sequence < admitted.the_release_sequence_floor {
        return Err(WhyTheReleaseWasRefused::ItsReleaseSequenceIsBelowTheFloorInForce);
    }

    let signed_by = whichever_allowed_key_signed(
        &admitted.statement,
        manifest.release_sequence,
        &offered.the_manifests_bytes,
        &offered.its_online_signature,
    )
    .ok_or(WhyTheReleaseWasRefused::NoAllowedKeySignedTheManifest)?;

    let manifest_digest = Digest::of(&offered.the_manifests_bytes).bytes();
    if let Some(stated) = stated_manifest_digest {
        if stated != manifest_digest {
            return Err(WhyTheReleaseWasRefused::ItIsNotTheManifestThatWasStated);
        }
    }

    let selected = super::manifest::select(&manifest, detected)
        .map_err(WhyTheReleaseWasRefused::TheSelection)?
        .clone();

    // **B8's HTTPS half, held on the side that consumes a manifest** rather than
    // only on the side that composes one.
    if let Some(the_source) = selected
        .sources
        .iter()
        .find(|source| !an_https_address(source))
    {
        return Err(
            WhyTheReleaseWasRefused::ItsSelectedArtifactNamesASourceThatIsNotAnHttpsAddress {
                the_source: the_source.clone(),
            },
        );
    }

    let the_trust_record_to_keep = TheTrustRecord {
        greatest_accepted_release_sequence: manifest.release_sequence,
        greatest_accepted_key_set_generation: admitted.statement.generation,
        the_statement_accepted_at_that_generation: offered.the_key_set_statements_bytes.clone(),
        its_root_signature: offered.its_root_signature,
    };

    Ok(AVerifiedRelease {
        manifest,
        selected,
        manifest_digest,
        signed_by,
        the_trust_record_to_keep,
    })
}

/// **Why the durable half of the acceptance could not be carried out.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhyTheReleaseWasNotAccepted {
    /// The release itself was refused, and nothing durable moved.
    TheRelease(WhyTheReleaseWasRefused),
    /// The store said something. **Nothing is downloaded on a store that will
    /// not answer**, because the non-rollback floor is what makes a verified
    /// manifest worth acting on.
    TheStore { said: String },
}

/// **The whole desktop-side acceptance: read what is remembered, verify, and
/// write what was accepted — B9's *durably remembers* performed rather than
/// described.**
///
/// **The record is written before anything is downloaded**, which is the order
/// B9's non-rollback rule needs: a manifest that verified and was then not
/// remembered would let the same copy accept an older one afterwards.
///
/// **A record `State, identity and recovery` J7 discards is discarded here**,
/// so the next attempt starts from the floor this build carries rather than
/// from a number nothing stands behind.
pub fn accept_a_release(
    root: &PublicKeyBytes,
    store: &mut impl super::trust::RemembersTheServerPackageTrustRecord,
    offered: &AnOfferedRelease,
    detected: &TargetTuple,
    stated_manifest_digest: Option<[u8; 32]>,
) -> Result<AVerifiedRelease, WhyTheReleaseWasNotAccepted> {
    let remembered = store
        .read_it()
        .map_err(|said| WhyTheReleaseWasNotAccepted::TheStore { said })?;

    if remembered.is_some()
        && super::trust::what_the_record_is_worth(root, remembered.as_ref())
            == super::trust::WhatTheRecordIsWorth::ItIsDiscardedAndTheFloorIsGone
    {
        store
            .discard_it()
            .map_err(|said| WhyTheReleaseWasNotAccepted::TheStore { said })?;
    }

    let verified = verify_before_download(
        root,
        remembered.as_ref(),
        offered,
        detected,
        stated_manifest_digest,
    )
    .map_err(WhyTheReleaseWasNotAccepted::TheRelease)?;

    store
        .write_it(&verified.the_trust_record_to_keep)
        .map_err(|said| WhyTheReleaseWasNotAccepted::TheStore { said })?;
    Ok(verified)
}

/// Which of the keys the statement allows at this sequence signed these bytes.
///
/// **Every candidate is tried rather than one being named beside the
/// signature.** A key identity written outside the signed bytes would be an
/// unsigned hint about which key to check, and the desktop would be reading
/// something the root never vouched for.
fn whichever_allowed_key_signed(
    statement: &KeySetStatement,
    release_sequence: u64,
    manifest_bytes: &[u8],
    signature: &SignatureBytes,
) -> Option<String> {
    statement
        .keys_that_may_authorize(release_sequence)
        .into_iter()
        .find(|key| {
            a_signature_verifies(
                &key.public_key,
                THE_MANIFEST_SIGNING_CONTEXT,
                manifest_bytes,
                signature,
            )
        })
        .map(|key| key.identity.clone())
}

/// **Why downloaded bytes are not the artifact.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhyTheDownloadWasRefused {
    /// **The bytes came from somewhere the verified manifest does not name.**
    ThatSourceIsNotOneTheManifestNamed,
    /// **The address they came from is not an HTTPS address**, which B8 requires
    /// of every source. **It is checked here as well as before the download**
    /// because this is the side that has the bytes in its hand.
    ThatSourceIsNotAnHttpsAddress,
    /// The length is not the manifest's.
    ItsByteLengthIsNotTheOneTheManifestCarries { expected: u64, measured: u64 },
    /// The digest is not the manifest's.
    ItsDigestIsNotTheOneTheManifestCarries,
}

/// **B10's second verification: the byte length and the digest, before
/// upload.**
///
/// The source is checked first and it is a whole-value comparison against the
/// entry's own list. **A download from an address that is not in that list is
/// refused whatever its bytes turn out to be**, because the exactness is the
/// point: bytes that happen to match are bytes somebody else could have
/// arranged to match.
pub fn verify_what_was_downloaded(
    selected: &ArtifactEntry,
    the_source_it_came_from: &str,
    bytes: &[u8],
) -> Result<(), WhyTheDownloadWasRefused> {
    if !selected
        .sources
        .iter()
        .any(|named| named == the_source_it_came_from)
    {
        return Err(WhyTheDownloadWasRefused::ThatSourceIsNotOneTheManifestNamed);
    }
    if !an_https_address(the_source_it_came_from) {
        return Err(WhyTheDownloadWasRefused::ThatSourceIsNotAnHttpsAddress);
    }
    if bytes.len() as u64 != selected.byte_length {
        return Err(
            WhyTheDownloadWasRefused::ItsByteLengthIsNotTheOneTheManifestCarries {
                expected: selected.byte_length,
                measured: bytes.len() as u64,
            },
        );
    }
    if compare(
        &Digest::from_bytes(selected.digest),
        &Digest::of(bytes),
    ) == WhatTheComparisonSaid::TheyDisagree
    {
        return Err(WhyTheDownloadWasRefused::ItsDigestIsNotTheOneTheManifestCarries);
    }
    Ok(())
}

/// What the read-back measured.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheReadingAtRest {
    pub byte_length: u64,
    pub digest: [u8; 32],
}

/// **What the read-back found, and there are two answers rather than one.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheReadBackFound {
    /// The staged bytes, read to their end inside the length the manifest names.
    ItRead(TheReadingAtRest),
    /// **There are more bytes at rest than the manifest names**, and the read
    /// stopped at the moment that became true rather than reading on to find out
    /// how many.
    ///
    /// **It is a refusal in its own right.** A6 says an authenticated connection
    /// is no evidence about the machine at the other end of it, and this reads
    /// from exactly such a machine; a machine that answers every read with a full
    /// chunk and never ends would otherwise hold the desktop for ever, computing
    /// a digest it would never compare. **The same happens by accident on a file
    /// that grows under the read.**
    ThereAreMoreBytesThanTheManifestNames { the_manifest_names: u64 },
}

/// **Whether one chunk may be taken, which is arithmetic and not a policy.**
///
/// **The addition saturates** so that a length the far end chose cannot wrap
/// past the ceiling instead of exceeding it.
fn a_chunk_may_be_taken(so_far: u64, how_many: usize, the_manifest_names: u64) -> bool {
    so_far.saturating_add(how_many as u64) <= the_manifest_names
}

/// **B10's read-back: the staged artifact's bytes, read back over the
/// file-transfer subsystem, with the digest computed in Smoozy's own loop.**
///
/// **Nothing on the machine is asked for a digest.** Before hand-over there is
/// no Smoozy program on the machine to ask, and after it there is still no
/// machine-side trust anchor — S22 is the clause, and this is that clause's
/// pre-hand-over route.
///
/// **The read has a ceiling and it is the manifest's own byte length.** There is
/// no deadline, timeout or duration anywhere in the subsystem client beneath
/// this, so nothing else bounds it: the loop would otherwise stop only when the
/// far end said end-of-file. **More bytes than the manifest names is a refusal
/// rather than something to read on and find out about**, and the comparison
/// this feeds already fails on a wrong length — what the ceiling does is make
/// sure that comparison is reached.
pub async fn read_back_at_rest<S>(
    subsystem: &mut Subsystem<S>,
    staged_path: &[u8],
    the_byte_length_the_manifest_names: u64,
    read_at_a_time: usize,
) -> Result<WhatTheReadBackFound, SubsystemFailure>
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send,
{
    let handle = subsystem.open_to_read(staged_path).await?;
    let mut running = Running::new();
    let mut at = 0u64;
    let mut there_are_more = false;
    loop {
        let read = subsystem.read_at(&handle, at, read_at_a_time).await?;
        if read.is_empty() {
            break;
        }
        if !a_chunk_may_be_taken(at, read.len(), the_byte_length_the_manifest_names) {
            there_are_more = true;
            break;
        }
        at += read.len() as u64;
        running.take(&read);
    }
    subsystem.close(&handle).await?;
    if there_are_more {
        return Ok(WhatTheReadBackFound::ThereAreMoreBytesThanTheManifestNames {
            the_manifest_names: the_byte_length_the_manifest_names,
        });
    }
    Ok(WhatTheReadBackFound::ItRead(TheReadingAtRest {
        byte_length: running.bytes_taken(),
        digest: running.so_far().bytes(),
    }))
}

/// **The at-rest comparison agreed, about one artifact at one path.**
///
/// **It is the only thing [`make_the_artifact_executable`] accepts**, it can be
/// made nowhere else in this workspace, and **it is consumed once**: there is no
/// copy and no clone, so a caller that has spent one does not still hold it.
///
/// **It names what it was earned by.** The staged path it was taken over and the
/// artifact digest it agreed on are inside it, and
/// [`make_the_artifact_executable`] checks both against what it was handed —
/// **so an agreement earned by comparing artifact A cannot be spent on artifact
/// B.** There is no production caller placing two artifacts today; the type
/// exists to prevent exactly that, and a value that carried no field said
/// nothing on the day there is one.
#[derive(Debug, PartialEq, Eq)]
pub struct TheAtRestComparisonAgreed {
    the_staged_path: Vec<u8>,
    the_artifact_digest: [u8; 32],
}

/// **B10's third verification, against the values B1's descriptor carries** —
/// which is `Rented-server runtime` G29's comparison, performed by the desktop
/// through the surface where no verified runtime is alive.
///
/// **The staged path is taken in so that the agreement can name it**, which is
/// what stops one agreement being spent on a different artifact.
pub fn compare_at_rest(
    descriptor: &TheDescriptorsValues,
    staged_path: &[u8],
    reading: TheReadingAtRest,
) -> Option<TheAtRestComparisonAgreed> {
    let agrees = reading.byte_length == descriptor.artifact_byte_length
        && compare(
            &Digest::from_bytes(descriptor.artifact_digest),
            &Digest::from_bytes(reading.digest),
        ) == WhatTheComparisonSaid::TheyAgree;
    agrees.then(|| TheAtRestComparisonAgreed {
        the_staged_path: staged_path.to_vec(),
        the_artifact_digest: descriptor.artifact_digest,
    })
}

/// **Whether an agreement is this artifact's at this path**, which is the whole
/// of the check [`make_the_artifact_executable`] makes before it sets anything.
fn the_agreement_is_about(
    agreed: &TheAtRestComparisonAgreed,
    staged_path: &[u8],
    the_artifact_digest: &[u8; 32],
) -> bool {
    agreed.the_staged_path == staged_path && &agreed.the_artifact_digest == the_artifact_digest
}

/// **Why a staged artifact was not made executable.**
#[derive(Debug)]
pub enum WhyTheArtifactWasNotMadeExecutable {
    /// **The agreement was earned over a different path or a different
    /// artifact**, so it is not this one's to spend. **Nothing was set.**
    ThatAgreementWasNotAboutThisArtifact,
    /// The subsystem refused the operation.
    TheSubsystemRefusedIt(SubsystemFailure),
}

/// **The mode a placed artifact is given**, and it is the owner's alone.
///
/// `Job files and outputs` L13 puts the supervisor and every worker it starts
/// under one login, so nothing else on the machine needs to run this file.
pub const THE_MODE_A_PLACED_ARTIFACT_IS_GIVEN: u32 = 0o700;

/// **Make the staged artifact executable — the subsystem's own attribute-setting
/// operation, and the last step before it can be invoked.**
///
/// It takes [`TheAtRestComparisonAgreed`] rather than a boolean, so the
/// ordering B10 requires is held by the compiler and not by whoever writes the
/// caller. **And it checks that the agreement is this artifact's**: the path and
/// the digest it was earned over must be the ones handed in here.
pub async fn make_the_artifact_executable<S>(
    subsystem: &mut Subsystem<S>,
    staged_path: &[u8],
    the_artifact_digest: &[u8; 32],
    the_comparison_agreed: TheAtRestComparisonAgreed,
) -> Result<(), WhyTheArtifactWasNotMadeExecutable>
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send,
{
    if !the_agreement_is_about(&the_comparison_agreed, staged_path, the_artifact_digest) {
        return Err(WhyTheArtifactWasNotMadeExecutable::ThatAgreementWasNotAboutThisArtifact);
    }
    subsystem
        .set_permissions(staged_path, THE_MODE_A_PLACED_ARTIFACT_IS_GIVEN)
        .await
        .map_err(WhyTheArtifactWasNotMadeExecutable::TheSubsystemRefusedIt)
}

/// **What a verification failure leaves behind, written out so nobody has to
/// infer it.**
///
/// B10: *a signature, digest, tuple or non-rollback failure leaves the active
/// pointer and boot registration untouched and returns a correlated
/// verification failure; version equality never overrides it.* **`Rented-server
/// runtime` G26's *failed before commit* is the machine-side name for the same
/// ending.** Nothing in this file writes a pointer or a registration, which is
/// how the first half is kept; the second half is kept by there being no
/// comparison of versions anywhere in it.
pub const WHAT_A_REFUSAL_LEAVES_UNTOUCHED: [&str; 2] =
    ["the active-package pointer", "the boot registration"];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::release::composing::{self, test_only_keys};
    use crate::release::trust::{
        AnAllowedKey, ARevocation, KeySetStatement,
        THE_GREATEST_KEY_SET_GENERATION_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE,
        THE_GREATEST_RELEASE_SEQUENCE_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE, THE_KEY_SET_FORMAT,
    };
    use crate::release::{a_signature_over, THE_KEY_SET_SIGNING_CONTEXT};

    /// **A release sequence `n` past the floor this build carries.**
    ///
    /// **Every release these tests offer is one a desktop built today could
    /// genuinely meet**, which is a release at or above the build's own floor —
    /// so what a test here refuses on is the thing the test is about, and never
    /// the build's floor standing in for it by accident. *(Until 2026-09-21
    /// these were bare numbers — 11, 12, 13, 19, 20 — which read as ordinary
    /// releases only while `Server protocol and transfers` B9's built-in pair
    /// was zero. With the pair at the first published release's numbers every
    /// one of them was below it.)* The build's floor itself is held by
    /// `a_desktop_built_today_refuses_what_is_below_the_first_published_release`.
    fn past_the_builds_floor(n: u64) -> u64 {
        THE_GREATEST_RELEASE_SEQUENCE_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE + n
    }

    /// The key-set generation these tests' statement carries: two past the
    /// build's own, for the same reason.
    const THE_STATEMENTS_GENERATION: u64 =
        THE_GREATEST_KEY_SET_GENERATION_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE + 2;

    fn root() -> test_only_keys::AKeyPair {
        test_only_keys::a_key_pair_from_a_written_down_seed(*b"the release root, written down__")
    }

    fn online() -> test_only_keys::AKeyPair {
        test_only_keys::a_key_pair_from_a_written_down_seed(*b"the online key, also written dow")
    }

    fn the_artifacts_bytes() -> Vec<u8> {
        b"a machine program, for the sake of the argument".to_vec()
    }

    const THE_SOURCE: &str = "https://example.invalid/smoozy-server-0.1.0";

    fn a_statement() -> KeySetStatement {
        KeySetStatement {
            statement_format: THE_KEY_SET_FORMAT.into(),
            generation: THE_STATEMENTS_GENERATION,
            allowed: vec![AnAllowedKey {
                identity: "online one".into(),
                public_key: online().public,
                valid_from_release_sequence: 1,
            }],
            revocations: Vec::new(),
        }
    }

    fn an_offered_release(statement: &KeySetStatement, release_sequence: u64) -> AnOfferedRelease {
        let artifact = composing::compose_an_artifact_entry(
            "linux-x86_64-gnu",
            composing::the_tuple_this_build_is_made_for(),
            &the_artifacts_bytes(),
            composing::THE_FIXED_ENTRY_POINT_NAME,
            &[THE_SOURCE.into()],
        );
        let manifest =
            composing::compose_a_manifest(release_sequence, "0.1.0", 2, Some(1), vec![artifact]);
        let statement_bytes = statement.canonical_bytes();
        let manifest_bytes = manifest.canonical_bytes();
        AnOfferedRelease {
            its_root_signature: a_signature_over(
                &root().secret,
                THE_KEY_SET_SIGNING_CONTEXT,
                &statement_bytes,
            ),
            the_key_set_statements_bytes: statement_bytes,
            its_online_signature: a_signature_over(
                &online().secret,
                THE_MANIFEST_SIGNING_CONTEXT,
                &manifest_bytes,
            ),
            the_manifests_bytes: manifest_bytes,
        }
    }

    /// **The debt the first release left, closed and held: a copy of Smoozy
    /// built today, with no trust record at all, refuses what is below the
    /// first published release.** Added 2026-09-21.
    ///
    /// Smoozy 0.1.0 was published at release sequence 1609 with a key-set
    /// statement of generation 1. **A fresh install is exactly the case B9's
    /// built-in pair exists for** — nothing remembered, so the only floor is the
    /// one the build carries — and until this date that pair was zero, so a
    /// fresh install would have admitted a genuine but older release, or a
    /// statement of a generation the root had since replaced. **The numbers
    /// below are the published release's own and are written out rather than
    /// read from the constants**, because a test that read them would go on
    /// passing if somebody put zero back. They stay true when a later release
    /// raises the pair again, a floor only ever going up.
    #[test]
    fn a_desktop_built_today_refuses_what_is_below_the_first_published_release() {
        use crate::release::trust::WhyTheStatementWasRefused;
        const THE_FIRST_PUBLISHED_RELEASE_SEQUENCE: u64 = 1609;
        const ITS_KEY_SET_GENERATION: u64 = 1;

        let at_generation = |generation: u64| KeySetStatement {
            generation,
            ..a_statement()
        };
        let this_machine = composing::the_tuple_this_build_is_made_for();

        // A manifest one below the first published release, under a statement
        // the build would otherwise admit: refused, with nothing remembered.
        let older = an_offered_release(
            &at_generation(ITS_KEY_SET_GENERATION),
            THE_FIRST_PUBLISHED_RELEASE_SEQUENCE - 1,
        );
        assert_eq!(
            verify_before_download(&root().public, None, &older, &this_machine, None),
            Err(WhyTheReleaseWasRefused::ItsReleaseSequenceIsBelowTheFloorInForce)
        );

        // A statement one generation below the published one, carrying a
        // manifest far above every floor: refused on the statement, before the
        // manifest is read.
        let replaced = an_offered_release(
            &at_generation(ITS_KEY_SET_GENERATION - 1),
            past_the_builds_floor(100),
        );
        assert_eq!(
            verify_before_download(&root().public, None, &replaced, &this_machine, None),
            Err(WhyTheReleaseWasRefused::TheKeySetStatement(
                WhyTheStatementWasRefused::ItsGenerationIsBelowTheFloorInForce
            ))
        );

        // **The same two through the whole acceptance, from an empty store** —
        // the real fresh install — and neither leaves anything remembered.
        let mut nothing_remembered = AStoreThatRemembers {
            held: None,
            discards: 0,
        };
        for refused in [&older, &replaced] {
            assert!(accept_a_release(
                &root().public,
                &mut nothing_remembered,
                refused,
                &this_machine,
                None
            )
            .is_err());
        }
        assert!(nothing_remembered.held.is_none());

        // **And it refuses only what is below**: at the floor this build
        // carries, on both numbers, the same fresh install admits.
        let at_the_floor = an_offered_release(
            &at_generation(THE_GREATEST_KEY_SET_GENERATION_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE),
            THE_GREATEST_RELEASE_SEQUENCE_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE,
        );
        assert!(accept_a_release(
            &root().public,
            &mut nothing_remembered,
            &at_the_floor,
            &this_machine,
            None
        )
        .is_ok());
    }

    #[test]
    fn a_whole_release_verifies_and_carries_the_descriptors_values() {
        let offered = an_offered_release(&a_statement(), past_the_builds_floor(12));
        let verified = verify_before_download(
            &root().public,
            None,
            &offered,
            &composing::the_tuple_this_build_is_made_for(),
            None,
        )
        .unwrap();
        assert_eq!(verified.signed_by, "online one");
        let descriptor = verified.the_release_descriptors_values();
        assert_eq!(descriptor.verified_release_sequence, past_the_builds_floor(12));
        assert_eq!(descriptor.requested_package_version, "0.1.0");
        assert_eq!(
            descriptor.artifact_byte_length,
            the_artifacts_bytes().len() as u64
        );
        assert_eq!(
            descriptor.manifest_digest,
            Digest::of(&offered.the_manifests_bytes).bytes()
        );
        // The trust record to keep carries the statement that admitted it.
        assert_eq!(
            verified
                .the_trust_record_to_keep
                .greatest_accepted_key_set_generation,
            THE_STATEMENTS_GENERATION
        );
        assert_eq!(
            verified
                .the_trust_record_to_keep
                .greatest_accepted_release_sequence,
            past_the_builds_floor(12)
        );
    }

    #[test]
    fn a_manifest_signed_by_a_key_the_root_never_allowed_is_refused() {
        let stranger =
            test_only_keys::a_key_pair_from_a_written_down_seed(*b"a stranger's key, written down__");
        let mut offered = an_offered_release(&a_statement(), past_the_builds_floor(12));
        offered.its_online_signature = a_signature_over(
            &stranger.secret,
            THE_MANIFEST_SIGNING_CONTEXT,
            &offered.the_manifests_bytes,
        );
        assert_eq!(
            verify_before_download(
                &root().public,
                None,
                &offered,
                &composing::the_tuple_this_build_is_made_for(),
                None
            ),
            Err(WhyTheReleaseWasRefused::NoAllowedKeySignedTheManifest)
        );
    }

    #[test]
    fn a_revoked_key_cannot_authorize_a_manifest_at_or_after_its_revocation() {
        let revoked = KeySetStatement {
            revocations: vec![ARevocation {
                identity: "online one".into(),
                first_disallowed_release_sequence: past_the_builds_floor(12),
            }],
            ..a_statement()
        };
        // At the revocation sequence, refused.
        let offered = an_offered_release(&revoked, past_the_builds_floor(12));
        assert_eq!(
            verify_before_download(
                &root().public,
                None,
                &offered,
                &composing::the_tuple_this_build_is_made_for(),
                None
            ),
            Err(WhyTheReleaseWasRefused::NoAllowedKeySignedTheManifest)
        );
        // Below it, the same key still signs.
        let offered = an_offered_release(&revoked, past_the_builds_floor(11));
        assert!(verify_before_download(
            &root().public,
            None,
            &offered,
            &composing::the_tuple_this_build_is_made_for(),
            None
        )
        .is_ok());
    }

    #[test]
    fn a_genuine_older_manifest_cannot_downgrade_a_copy() {
        let statement = a_statement();
        let newer = an_offered_release(&statement, past_the_builds_floor(20));
        let verified = verify_before_download(
            &root().public,
            None,
            &newer,
            &composing::the_tuple_this_build_is_made_for(),
            None,
        )
        .unwrap();
        let remembered = verified.the_trust_record_to_keep;

        let older = an_offered_release(&statement, past_the_builds_floor(19));
        assert_eq!(
            verify_before_download(
                &root().public,
                Some(&remembered),
                &older,
                &composing::the_tuple_this_build_is_made_for(),
                None
            ),
            Err(WhyTheReleaseWasRefused::ItsReleaseSequenceIsBelowTheFloorInForce)
        );
        // The same one again is not a rollback.
        let same = an_offered_release(&statement, past_the_builds_floor(20));
        assert!(verify_before_download(
            &root().public,
            Some(&remembered),
            &same,
            &composing::the_tuple_this_build_is_made_for(),
            None
        )
        .is_ok());
    }

    #[test]
    fn one_byte_of_the_manifest_changed_is_a_manifest_nobody_signed() {
        let mut offered = an_offered_release(&a_statement(), past_the_builds_floor(12));
        let last = offered.the_manifests_bytes.len() - 1;
        offered.the_manifests_bytes[last] ^= 1;
        assert!(matches!(
            verify_before_download(
                &root().public,
                None,
                &offered,
                &composing::the_tuple_this_build_is_made_for(),
                None
            ),
            Err(WhyTheReleaseWasRefused::NoAllowedKeySignedTheManifest)
                | Err(WhyTheReleaseWasRefused::TheManifest(_))
        ));
    }

    #[test]
    fn a_continuation_refuses_a_manifest_that_is_not_the_one_the_record_names() {
        let offered = an_offered_release(&a_statement(), past_the_builds_floor(12));
        assert_eq!(
            verify_before_download(
                &root().public,
                None,
                &offered,
                &composing::the_tuple_this_build_is_made_for(),
                Some([0; 32])
            ),
            Err(WhyTheReleaseWasRefused::ItIsNotTheManifestThatWasStated)
        );
    }

    #[test]
    fn a_machine_outside_the_manifests_tuples_is_reported_before_any_byte_is_downloaded() {
        let offered = an_offered_release(&a_statement(), past_the_builds_floor(12));
        let elsewhere = TargetTuple {
            operating_system_family: "linux".into(),
            processor_architecture: "aarch64".into(),
            abi_and_capabilities: "gnu".into(),
        };
        assert_eq!(
            verify_before_download(&root().public, None, &offered, &elsewhere, None),
            Err(WhyTheReleaseWasRefused::TheSelection(
                WhySelectionFailed::ThisMachineIsNotSupported
            ))
        );
    }

    #[test]
    fn a_download_is_held_to_the_source_the_length_and_the_digest() {
        let offered = an_offered_release(&a_statement(), past_the_builds_floor(12));
        let verified = verify_before_download(
            &root().public,
            None,
            &offered,
            &composing::the_tuple_this_build_is_made_for(),
            None,
        )
        .unwrap();
        let bytes = the_artifacts_bytes();
        assert!(verify_what_was_downloaded(&verified.selected, THE_SOURCE, &bytes).is_ok());
        assert_eq!(
            verify_what_was_downloaded(
                &verified.selected,
                "https://example.invalid/somewhere-else",
                &bytes
            ),
            Err(WhyTheDownloadWasRefused::ThatSourceIsNotOneTheManifestNamed)
        );
        let mut short = bytes.clone();
        short.pop();
        assert!(matches!(
            verify_what_was_downloaded(&verified.selected, THE_SOURCE, &short),
            Err(WhyTheDownloadWasRefused::ItsByteLengthIsNotTheOneTheManifestCarries { .. })
        ));
        let mut changed = bytes.clone();
        changed[0] ^= 1;
        assert_eq!(
            verify_what_was_downloaded(&verified.selected, THE_SOURCE, &changed),
            Err(WhyTheDownloadWasRefused::ItsDigestIsNotTheOneTheManifestCarries)
        );
    }

    /// **B8's HTTPS half is held on the verifying side, at both moments.**
    ///
    /// **The scheme used to be checked only in the release tool, which no
    /// shipped copy of Smoozy runs**, so half of one clause was kept by whoever
    /// composes and the other half by nobody who consumes. What it costs is
    /// small — the bytes are still held to the signed length and digest, so a
    /// plain-address fetch could only be watched or blocked — and the rule now
    /// lives where the clause aims it.
    #[test]
    fn a_source_that_is_not_an_https_address_is_refused_on_the_verifying_side() {
        assert!(an_https_address("https://example.invalid/a"));
        assert!(!an_https_address("http://example.invalid/a"));
        assert!(!an_https_address("HTTPS://example.invalid/a"));
        assert!(!an_https_address("ftp://example.invalid/a"));
        assert!(!an_https_address("example.invalid/a"));

        // Before the download: a manifest whose selected artifact names one.
        let plain = "http://example.invalid/smoozy-server";
        let statement = a_statement();
        let artifact = composing::compose_an_artifact_entry(
            "linux-x86_64-gnu",
            composing::the_tuple_this_build_is_made_for(),
            &the_artifacts_bytes(),
            composing::THE_FIXED_ENTRY_POINT_NAME,
            &[THE_SOURCE.into(), plain.into()],
        );
        let manifest = composing::compose_a_manifest(
            past_the_builds_floor(12),
            "0.1.0",
            2,
            Some(1),
            vec![artifact],
        );
        let statement_bytes = statement.canonical_bytes();
        let manifest_bytes = manifest.canonical_bytes();
        let offered = AnOfferedRelease {
            its_root_signature: a_signature_over(
                &root().secret,
                THE_KEY_SET_SIGNING_CONTEXT,
                &statement_bytes,
            ),
            the_key_set_statements_bytes: statement_bytes,
            its_online_signature: a_signature_over(
                &online().secret,
                THE_MANIFEST_SIGNING_CONTEXT,
                &manifest_bytes,
            ),
            the_manifests_bytes: manifest_bytes,
        };
        assert_eq!(
            verify_before_download(
                &root().public,
                None,
                &offered,
                &composing::the_tuple_this_build_is_made_for(),
                None,
            ),
            Err(
                WhyTheReleaseWasRefused::ItsSelectedArtifactNamesASourceThatIsNotAnHttpsAddress {
                    the_source: plain.to_string()
                }
            )
        );

        // And after it: bytes that came from a plain address the manifest does
        // name are refused on the scheme rather than on the address.
        let entry = composing::compose_an_artifact_entry(
            "linux-x86_64-gnu",
            composing::the_tuple_this_build_is_made_for(),
            &the_artifacts_bytes(),
            composing::THE_FIXED_ENTRY_POINT_NAME,
            &[plain.into()],
        );
        assert_eq!(
            verify_what_was_downloaded(&entry, plain, &the_artifacts_bytes()),
            Err(WhyTheDownloadWasRefused::ThatSourceIsNotAnHttpsAddress)
        );
    }

    /// A store that remembers, for the acceptance tests.
    struct AStoreThatRemembers {
        held: Option<TheTrustRecord>,
        discards: usize,
    }

    impl super::super::trust::RemembersTheServerPackageTrustRecord for AStoreThatRemembers {
        fn read_it(&self) -> Result<Option<TheTrustRecord>, String> {
            Ok(self.held.clone())
        }
        fn write_it(&mut self, record: &TheTrustRecord) -> Result<(), String> {
            self.held = Some(record.clone());
            Ok(())
        }
        fn discard_it(&mut self) -> Result<(), String> {
            self.held = None;
            self.discards += 1;
            Ok(())
        }
    }

    #[test]
    fn accepting_a_release_remembers_it_and_the_next_older_one_is_refused() {
        let mut store = AStoreThatRemembers {
            held: None,
            discards: 0,
        };
        let statement = a_statement();
        let newer = an_offered_release(&statement, past_the_builds_floor(20));
        accept_a_release(
            &root().public,
            &mut store,
            &newer,
            &composing::the_tuple_this_build_is_made_for(),
            None,
        )
        .unwrap();
        assert_eq!(
            store.held.as_ref().unwrap().greatest_accepted_release_sequence,
            past_the_builds_floor(20)
        );

        let older = an_offered_release(&statement, past_the_builds_floor(19));
        assert_eq!(
            accept_a_release(
                &root().public,
                &mut store,
                &older,
                &composing::the_tuple_this_build_is_made_for(),
                None
            ),
            Err(WhyTheReleaseWasNotAccepted::TheRelease(
                WhyTheReleaseWasRefused::ItsReleaseSequenceIsBelowTheFloorInForce
            ))
        );
        // The refusal moved nothing durable.
        assert_eq!(
            store.held.as_ref().unwrap().greatest_accepted_release_sequence,
            past_the_builds_floor(20)
        );
        assert_eq!(store.discards, 0);
    }

    #[test]
    fn a_remembered_record_that_will_not_verify_is_discarded_and_stops_refusing() {
        let statement = a_statement();
        let newer = an_offered_release(&statement, past_the_builds_floor(20));
        let mut damaged = {
            let mut store = AStoreThatRemembers {
                held: None,
                discards: 0,
            };
            accept_a_release(
                &root().public,
                &mut store,
                &newer,
                &composing::the_tuple_this_build_is_made_for(),
                None,
            )
            .unwrap();
            store
        };
        damaged.held.as_mut().unwrap().its_root_signature[0] ^= 1;

        // An older but genuine release is now accepted on its own merits, and
        // the damaged record was discarded rather than obeyed.
        let older = an_offered_release(&statement, past_the_builds_floor(19));
        accept_a_release(
            &root().public,
            &mut damaged,
            &older,
            &composing::the_tuple_this_build_is_made_for(),
            None,
        )
        .unwrap();
        assert_eq!(damaged.discards, 1);
        assert_eq!(
            damaged.held.as_ref().unwrap().greatest_accepted_release_sequence,
            past_the_builds_floor(19)
        );
    }

    /// **The read-back's ceiling, held as the arithmetic it is.** A chunk that
    /// would take the running total past the length the manifest names is not
    /// taken, and the addition saturates so a length the far end chose cannot
    /// wrap past the ceiling instead of exceeding it.
    #[test]
    fn a_chunk_that_would_pass_the_length_the_manifest_names_is_not_taken() {
        // Exactly reaching the length is fine; one byte past it is not.
        assert!(a_chunk_may_be_taken(0, 100, 100));
        assert!(a_chunk_may_be_taken(60, 40, 100));
        assert!(!a_chunk_may_be_taken(60, 41, 100));
        assert!(!a_chunk_may_be_taken(100, 1, 100));
        // A file that grows under the read is the same case.
        assert!(!a_chunk_may_be_taken(100, 32_768, 100));
        // And the running total never wraps.
        assert!(!a_chunk_may_be_taken(u64::MAX, 1, 100));
    }

    #[test]
    fn the_at_rest_comparison_needs_both_the_length_and_the_digest() {
        let offered = an_offered_release(&a_statement(), past_the_builds_floor(12));
        let verified = verify_before_download(
            &root().public,
            None,
            &offered,
            &composing::the_tuple_this_build_is_made_for(),
            None,
        )
        .unwrap();
        let descriptor = verified.the_release_descriptors_values();
        let right = TheReadingAtRest {
            byte_length: descriptor.artifact_byte_length,
            digest: descriptor.artifact_digest,
        };
        let staged = b"/ephemeral/smoozy/staging/artifact".as_slice();
        assert!(compare_at_rest(&descriptor, staged, right).is_some());
        assert!(compare_at_rest(
            &descriptor,
            staged,
            TheReadingAtRest {
                byte_length: right.byte_length - 1,
                ..right
            }
        )
        .is_none());
        assert!(compare_at_rest(
            &descriptor,
            staged,
            TheReadingAtRest {
                digest: [0; 32],
                ..right
            }
        )
        .is_none());
    }

    /// **An agreement names the artifact and the path it was earned over**, so
    /// one earned by comparing artifact A cannot be spent on artifact B.
    ///
    /// **There is no production caller placing two artifacts today**, and that
    /// is exactly why this is worth holding: the day there is one, the type
    /// that exists to prevent this has to say something.
    #[test]
    fn an_agreement_earned_over_one_artifact_is_not_this_ones_to_spend() {
        let offered = an_offered_release(&a_statement(), past_the_builds_floor(12));
        let verified = verify_before_download(
            &root().public,
            None,
            &offered,
            &composing::the_tuple_this_build_is_made_for(),
            None,
        )
        .unwrap();
        let descriptor = verified.the_release_descriptors_values();
        let reading = TheReadingAtRest {
            byte_length: descriptor.artifact_byte_length,
            digest: descriptor.artifact_digest,
        };
        let agreed = compare_at_rest(&descriptor, b"/staging/a", reading).unwrap();
        // The path it was earned over.
        assert!(the_agreement_is_about(&agreed, b"/staging/a", &descriptor.artifact_digest));
        // A second artifact at a second path, and the same artifact at the
        // wrong path.
        assert!(!the_agreement_is_about(&agreed, b"/staging/b", &descriptor.artifact_digest));
        assert!(!the_agreement_is_about(&agreed, b"/staging/a", &[0; 32]));
    }

    #[test]
    fn the_one_distributed_file_reads_back_as_the_four_fields_and_still_verifies() {
        let offered = an_offered_release(&a_statement(), past_the_builds_floor(12));
        let bytes = offered.canonical_bytes();
        let read = AnOfferedRelease::read(&bytes).unwrap();
        assert_eq!(read, offered);
        // **The container carries no trust of its own**, so what says it is
        // sound is the same verification the desktop makes.
        assert!(verify_before_download(
            &root().public,
            None,
            &read,
            &composing::the_tuple_this_build_is_made_for(),
            None,
        )
        .is_ok());
    }

    #[test]
    fn a_container_that_is_not_one_gives_nothing_rather_than_a_guess() {
        let offered = an_offered_release(&a_statement(), past_the_builds_floor(12));
        let bytes = offered.canonical_bytes();
        assert!(AnOfferedRelease::read(&[]).is_none());
        assert!(AnOfferedRelease::read(&bytes[..bytes.len() - 1]).is_none());
        let mut with_a_tail = bytes.clone();
        with_a_tail.push(0);
        assert!(AnOfferedRelease::read(&with_a_tail).is_none());

        // A signature that is not sixty-four bytes.
        let mut short = super::super::Canonical::new();
        short
            .text(THE_OFFERED_RELEASE_FORMAT)
            .run(&offered.the_key_set_statements_bytes)
            .run(&offered.its_root_signature[..63])
            .run(&offered.the_manifests_bytes)
            .run(&offered.its_online_signature);
        assert!(AnOfferedRelease::read(&short.finish()).is_none());

        // A format this build does not know.
        let mut wrong = super::super::Canonical::new();
        wrong
            .text("some-other-container")
            .run(&offered.the_key_set_statements_bytes)
            .run(&offered.its_root_signature)
            .run(&offered.the_manifests_bytes)
            .run(&offered.its_online_signature);
        assert!(AnOfferedRelease::read(&wrong.finish()).is_none());
    }

    #[test]
    fn a_document_swapped_inside_the_container_is_caught_by_the_signatures() {
        let offered = an_offered_release(&a_statement(), past_the_builds_floor(12));
        let another = an_offered_release(&a_statement(), past_the_builds_floor(13));
        let mixed = AnOfferedRelease {
            the_manifests_bytes: another.the_manifests_bytes,
            ..offered.clone()
        };
        let bytes = mixed.canonical_bytes();
        let read = AnOfferedRelease::read(&bytes).unwrap();
        assert_eq!(
            verify_before_download(
                &root().public,
                None,
                &read,
                &composing::the_tuple_this_build_is_made_for(),
                None,
            ),
            Err(WhyTheReleaseWasRefused::NoAllowedKeySignedTheManifest)
        );
    }
}
