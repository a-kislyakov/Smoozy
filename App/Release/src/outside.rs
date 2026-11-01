//! **Checking a release from the outside: fetch what was published, by its
//! address, and verify it the way the desktop application will.**
//!
//! **A release nobody has fetched is a release nobody has tested**, which is
//! why the release runner's seventh step exists. What is checked here is the
//! two of B10's three verifications that do not need a rented machine:
//!
//! 1. **Before download** — the root-signed key-set statement, the manifest
//!    signature by an allowed key, the non-rollback rules, and the selection of
//!    an artifact on an exact tuple match.
//! 2. **Before upload** — the bytes came from an exact source the verified
//!    manifest names, and their byte length and digest are the selected
//!    artifact's.
//!
//! **The third is not checked and cannot be**: B10's at-rest read-back is taken
//! over a file-transfer connection to a rented machine, so it wants a rental.
//! **That is said plainly rather than passed over.**
//!
//! **Two things about how it checks, because both were once weaker than they
//! read.** **Every source the entry names is fetched and verified**, not the
//! first — a desktop may fetch from any of them, and a source nobody asks is a
//! source nobody tests. And **the floor it checks against is the release
//! runner's to type**: with none it checks as a copy of Smoozy that has never
//! accepted a release, which is not the ordinary user and is the easiest case to
//! be right about rather than the hardest. **Such a copy is still held to the
//! floor this build carries** (`Server protocol and transfers` B9's built-in
//! pair, raised off zero on 2026-09-21), and a typed floor only ever raises it.
//!
//! **The fetch runs `curl` and that is this crate's own, not the application's.**
//! `Server protocol and transfers` still has no fetcher that reaches the
//! internet, and this one does not become one: it lives in the release
//! machinery, which runs on the Creator's Mac and nowhere else, and it exists so
//! that no HTTP client has to be added to a workspace whose weight is watched.
//!
//! **The desktop update file is checked here too, from 2026-10-25** (step
//! 11's D8): fetched from the one fixed address every copy of Smoozy reads it
//! from, believed only through the desktop's own reading for every row, and
//! each row's artifact fetched from the same release and held to its length
//! and digest — [`check_the_published_update_file`].
//!
//! **Markers used below, each with the document it belongs to.** `Server
//! protocol and transfers` B8, B10; `Desktop app and delivery` E10a, E15, E16.

use smoozy_server_protocol::release::composing::{
    the_tuple_this_build_is_made_for, FetchesFromAnExactSourceTheManifestNamed, WhyTheFetchFailed,
};
use smoozy_server_protocol::release::trust::{
    TheBuiltInFloor, THE_RELEASE_ROOT_THIS_BUILD_CARRIES,
};
use smoozy_server_protocol::release::verifying::{
    verify_before_download_against, verify_what_was_downloaded, AnOfferedRelease,
};

use crate::the_desktop_update::{
    the_artifact_is_the_one_offered, what_every_row_is_offered, WhatEveryRowIsOffered,
    WhyTheUpdateFileWasNotMade,
};
use smoozy_desktop::updating::checking::{
    THE_MOST_THE_UPDATE_FILE_MAY_BE, WHERE_THE_UPDATE_FILE_IS_READ_FROM,
};
use smoozy_desktop::updating::trust::{ACarriedStatement, TheCarriedSet};

/// **A fetch that runs the `curl` the Mac already carries.**
///
/// **It is given the source whole and composes nothing** — B8's rule about the
/// artifact, applied to the address, and B8a's about the source.
///
/// **It follows a redirect, and that is a decision rather than a default.** The
/// first version of this refused one, on the reading that an exact source means
/// the bytes must come off the machine that address names. **That reading makes
/// the check unable to reach any published artifact at all**: a public
/// repository serves a release file by answering the address with a redirect to
/// its own object store, and so does every other hosting anybody would use.
///
/// **What B8's exactness is about is which address Smoozy asks, not which
/// machine finally answers.** Smoozy asks the one address the verified manifest
/// names and never composes another; what says the bytes are right is the byte
/// length and the digest in that same signed manifest, which is B10's second
/// verification and is made whatever route the bytes took. **The transport is
/// held to HTTPS across a redirect as well as at the first request**, so a
/// redirect cannot quietly move the fetch to a plain connection.
pub struct AFetchByRunningCurl {
    /// Every source it was asked for, in order.
    pub asked_for: Vec<String>,
}

impl AFetchByRunningCurl {
    pub fn new() -> AFetchByRunningCurl {
        AFetchByRunningCurl {
            asked_for: Vec::new(),
        }
    }
}

impl FetchesFromAnExactSourceTheManifestNamed for AFetchByRunningCurl {
    fn fetch(&mut self, source: &str) -> Result<Vec<u8>, WhyTheFetchFailed> {
        self.asked_for.push(source.to_owned());
        let ran = std::process::Command::new("curl")
            .arg("--silent")
            .arg("--show-error")
            .arg("--fail")
            .arg("--proto")
            .arg("=https")
            // HTTPS at the first request and at every redirect after it.
            .arg("--proto-redir")
            .arg("=https")
            .arg("--location")
            .arg("--max-redirs")
            .arg("5")
            .arg("--output")
            .arg("-")
            .arg(source)
            .output()
            .map_err(|failed| WhyTheFetchFailed::ItCouldNotBeReached {
                said: failed.to_string(),
            })?;
        if !ran.status.success() {
            return Err(WhyTheFetchFailed::ItCouldNotBeReached {
                said: String::from_utf8_lossy(&ran.stderr).trim().to_owned(),
            });
        }
        Ok(ran.stdout)
    }
}

/// **A floor the release runner types, so the check can run the refusal a real
/// desktop would.**
///
/// **The ordinary user is not a copy that has never seen a release.** They have
/// already accepted some generation and some sequence, and a release composed at
/// a generation or a sequence below one already shipped is accepted by a check
/// that holds no floor and refused by every such desktop. **That is the case the
/// check exists to catch and it could not see it.**
///
/// **Nothing derives this pair.** The release runner types it, the way both
/// release numbers are typed: a value read off the last release would be the
/// check agreeing with whatever was published rather than with what desktops
/// hold.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AsADesktopThatHasAccepted {
    pub key_set_generation: u64,
    pub release_sequence: u64,
}

/// **What a refused fetch said, in one sentence, for every reason the fetcher
/// has.**
///
/// **Widened 2026-09-07.** The fetcher that reaches the internet arrived that
/// day and `WhyTheFetchFailed` went from one reason to six with it; the two
/// places below had taken the one reason apart in a `let`, which stopped
/// compiling. **The match is total and carries no catch-all arm**, so a seventh
/// reason fails the build here rather than being folded into a sentence that
/// does not describe it.
///
/// `Server protocol and transfers` B8 and B10 are what the reasons are about —
/// the exact HTTPS source, and the manifest's own length as the ceiling.
fn what_the_fetch_said(why: WhyTheFetchFailed) -> String {
    match why {
        WhyTheFetchFailed::ItCouldNotBeReached { said } => said,
        WhyTheFetchFailed::TheSourceIsNotAnExactHttpsAddress { source } => {
            format!("that is not an exact HTTPS address: {source}")
        }
        WhyTheFetchFailed::TheSourceHasNoSuchThing => {
            "the source answered and there is nothing there".to_owned()
        }
        WhyTheFetchFailed::TheSourceAnsweredWith { status } => {
            format!("the source answered with {status}")
        }
        WhyTheFetchFailed::ItIsLargerThanTheCeiling { ceiling } => {
            format!("it offered more than the {ceiling} bytes it was held to")
        }
        WhyTheFetchFailed::TheResumeDidNotLineUp { said } => {
            format!("a partial answer did not start where it was asked to: {said}")
        }
    }
}

/// What the outside check found.
pub struct WhatTheOutsideCheckFound {
    pub release_sequence: u64,
    pub package_version: String,
    pub signed_by: String,
    /// **Every source the entry names, in the order the manifest names them.**
    /// All of them were fetched and all of them held to the signed length and
    /// digest.
    pub the_sources_the_artifact_was_fetched_from: Vec<String>,
    pub artifact_byte_length: u64,
}

/// **Why a published release did not check out.**
#[derive(Debug)]
pub enum WhyTheCheckFailed {
    ThisBuildCarriesNoReleaseRoot,
    TheReleaseFileCouldNotBeFetched { said: String },
    /// The bytes at that address are not one of this build's release files.
    ThoseBytesAreNotARelease,
    /// **B10's first verification refused it.** Whatever else was published,
    /// nothing downstream may act on it.
    ItDidNotVerify { said: String },
    TheArtifactCouldNotBeFetched { source: String, said: String },
    /// **B10's second verification refused the bytes**, from the source named.
    TheArtifactIsNotTheOneTheManifestNames { source: String, said: String },
    /// The selected entry names no source at all, so there is nothing to fetch.
    TheEntryNamesNoSource,
}

/// **Every ending, said in plain English**, for the same reason the making
/// half's are.
impl std::fmt::Display for WhyTheCheckFailed {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WhyTheCheckFailed::ThisBuildCarriesNoReleaseRoot => write!(
                out,
                "this build carries no release root, so it can verify nothing"
            ),
            WhyTheCheckFailed::TheReleaseFileCouldNotBeFetched { said } => write!(
                out,
                "the release file could not be fetched from that address: {said}"
            ),
            WhyTheCheckFailed::ThoseBytesAreNotARelease => write!(
                out,
                "something answered at that address and it is not a Smoozy release"
            ),
            WhyTheCheckFailed::ItDidNotVerify { said } => write!(
                out,
                "the published release does not verify against the release root this build \
                 carries: {said}"
            ),
            WhyTheCheckFailed::TheArtifactCouldNotBeFetched { source, said } => write!(
                out,
                "the release verified, and the artifact could not be fetched from {source}: {said}"
            ),
            WhyTheCheckFailed::TheArtifactIsNotTheOneTheManifestNames { source, said } => write!(
                out,
                "the bytes at {source} are not the artifact the manifest names: {said}"
            ),
            WhyTheCheckFailed::TheEntryNamesNoSource => write!(
                out,
                "the manifest's own entry for this build names no source, so there is nothing to \
                 fetch and nothing to check"
            ),
        }
    }
}

/// **Fetch a published release by its address and verify it.**
///
/// **Every source the entry names is fetched and verified, not the first.** An
/// entry may name several, a desktop may fetch from any of them, and a second
/// source serving the wrong bytes used to pass this check untouched.
///
/// **The floor is the release runner's to type.** With none, this checks as a
/// copy of Smoozy that has never seen a release — which is not the ordinary
/// user and is the easiest case to be right about, not the hardest. See
/// [`AsADesktopThatHasAccepted`].
pub fn check_a_published_release(
    the_release_files_address: &str,
    fetch: &mut impl FetchesFromAnExactSourceTheManifestNamed,
    as_a_desktop_that_has_accepted: Option<AsADesktopThatHasAccepted>,
) -> Result<WhatTheOutsideCheckFound, WhyTheCheckFailed> {
    let Some(root) = THE_RELEASE_ROOT_THIS_BUILD_CARRIES else {
        return Err(WhyTheCheckFailed::ThisBuildCarriesNoReleaseRoot);
    };
    check_a_published_release_against(&root, the_release_files_address, fetch, as_a_desktop_that_has_accepted)
}

/// **The same check, against a root named by the caller.**
///
/// **The real root's secret half is the Creator's**, so no test in this crate
/// can compose a release the real root signed. This is the seam that lets the
/// check itself be run end to end against a release signed by a key written down
/// in a test — and [`check_a_published_release`] is it with the root this build
/// carries.
pub fn check_a_published_release_against(
    root: &smoozy_server_protocol::release::PublicKeyBytes,
    the_release_files_address: &str,
    fetch: &mut impl FetchesFromAnExactSourceTheManifestNamed,
    as_a_desktop_that_has_accepted: Option<AsADesktopThatHasAccepted>,
) -> Result<WhatTheOutsideCheckFound, WhyTheCheckFailed> {
    let bytes = fetch
        .fetch(the_release_files_address)
        .map_err(|why| WhyTheCheckFailed::TheReleaseFileCouldNotBeFetched {
            said: what_the_fetch_said(why),
        })?;
    let offered =
        AnOfferedRelease::read(&bytes).ok_or(WhyTheCheckFailed::ThoseBytesAreNotARelease)?;

    // **A typed floor raises the build's own and never lowers it** — `Server
    // protocol and transfers` B9's *the greater of the built-in value and the
    // remembered one, never the smaller*, because a desktop of this build that
    // has accepted a release holds both. *(Added 2026-09-21, when the build's
    // pair was raised off zero: until then the typed pair replacing it and the
    // typed pair raising it were the same thing, and from then a floor typed
    // below the build's would have passed a release every such desktop refuses.)*
    let this_build = TheBuiltInFloor::this_build();
    let floor = match as_a_desktop_that_has_accepted {
        None => this_build,
        Some(accepted) => TheBuiltInFloor {
            greatest_key_set_generation: accepted
                .key_set_generation
                .max(this_build.greatest_key_set_generation),
            greatest_release_sequence: accepted
                .release_sequence
                .max(this_build.greatest_release_sequence),
        },
    };
    let verified = verify_before_download_against(
        root,
        None,
        &offered,
        &the_tuple_this_build_is_made_for(),
        None,
        floor,
    )
    .map_err(|why| WhyTheCheckFailed::ItDidNotVerify {
        said: format!("{why:?}"),
    })?;

    // **Every address is taken out of the verified manifest**, which is the
    // whole of B10's *downloads only from an exact source named by that
    // verified manifest*. **All of them, because a desktop may fetch from any
    // one of them**, and a source serving the wrong bytes is only found by
    // asking it.
    if verified.selected.sources.is_empty() {
        return Err(WhyTheCheckFailed::TheEntryNamesNoSource);
    }
    for source in &verified.selected.sources {
        let artifact = fetch.fetch(source).map_err(|why| {
            WhyTheCheckFailed::TheArtifactCouldNotBeFetched {
                source: source.clone(),
                said: what_the_fetch_said(why),
            }
        })?;
        verify_what_was_downloaded(&verified.selected, source, &artifact).map_err(|why| {
            WhyTheCheckFailed::TheArtifactIsNotTheOneTheManifestNames {
                source: source.clone(),
                said: format!("{why:?}"),
            }
        })?;
    }

    Ok(WhatTheOutsideCheckFound {
        release_sequence: verified.manifest.release_sequence,
        package_version: verified.manifest.package_version,
        signed_by: verified.signed_by,
        the_sources_the_artifact_was_fetched_from: verified.selected.sources.clone(),
        artifact_byte_length: verified.selected.byte_length,
    })
}

// ------------------------------------------------- the desktop update file

/// **The fixed address every copy of Smoozy reads its update file from** —
/// `Desktop app and delivery` E10a, the desktop's own constant, not a second
/// spelling of it.
pub const WHERE_THE_DESKTOP_READS_ITS_UPDATE_FILE: &str = WHERE_THE_UPDATE_FILE_IS_READ_FROM;

/// **Where an artifact the update file names is published: beside the update
/// file, in the same release, under the name the file gives it.** Build's rule,
/// step 11's D8. The release writes every row's artifact into the same release
/// as the update file, so the address is the update file's with its own name
/// in place of the last part. **The name carries no trust**: what says the
/// bytes are right is the length and digest the signed manifest names.
pub fn where_an_artifact_it_names_is(the_update_files_address: &str, file_name: &str) -> String {
    match the_update_files_address.rsplit_once('/') {
        Some((folder, _)) => format!("{folder}/{file_name}"),
        None => file_name.to_owned(),
    }
}

/// What the check of the published update file found.
pub struct WhatTheUpdateCheckFound {
    pub offered: WhatEveryRowIsOffered,
    /// **Every artifact address that was fetched and held to its row's length
    /// and digest**, once each.
    pub fetched: Vec<String>,
}

/// **Why the published update file did not check out.**
#[derive(Debug)]
pub enum WhyTheUpdateCheckFailed {
    TheFileCouldNotBeFetched { said: String },
    /// **Larger than a copy of Smoozy reads**, so no copy would read it at all.
    ItIsLargerThanACopyReads { length: u64 },
    /// The desktop's own reading refused it for a row, or a row is offered
    /// nothing.
    ItIsNotBelieved { why: WhyTheUpdateFileWasNotMade },
    AnArtifactCouldNotBeFetched { address: String, said: String },
    /// **The bytes published for a row are not the ones the file names.**
    AnArtifactIsNotTheOneNamed { address: String, why: WhyTheUpdateFileWasNotMade },
}

impl std::fmt::Display for WhyTheUpdateCheckFailed {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WhyTheUpdateCheckFailed::TheFileCouldNotBeFetched { said } => write!(
                out,
                "the desktop update file could not be fetched from {WHERE_THE_DESKTOP_READS_ITS_UPDATE_FILE}: {said}"
            ),
            WhyTheUpdateCheckFailed::ItIsLargerThanACopyReads { length } => write!(
                out,
                "the desktop update file is {length} bytes, more than the \
                 {THE_MOST_THE_UPDATE_FILE_MAY_BE} a copy of Smoozy reads, so no copy would read it"
            ),
            WhyTheUpdateCheckFailed::ItIsNotBelieved { why } => write!(
                out,
                "the published desktop update file would update no copy of Smoozy: {why}"
            ),
            WhyTheUpdateCheckFailed::AnArtifactCouldNotBeFetched { address, said } => write!(
                out,
                "the desktop update file is believed, and an artifact it names could not be \
                 fetched from {address}: {said}"
            ),
            WhyTheUpdateCheckFailed::AnArtifactIsNotTheOneNamed { address, why } => {
                write!(out, "the bytes at {address} are not the artifact the update file names: {why}")
            }
        }
    }
}

/// **Fetch the published desktop update file from its fixed address and check
/// it the way every copy of Smoozy will**: believed through the desktop's own
/// reading for every row, from every starting point a copy can have, and each
/// row's artifact fetched from beside it and held to its length and digest.
pub fn check_the_published_update_file(
    carried: &TheCarriedSet,
    earlier: &[ACarriedStatement],
    fetch: &mut impl FetchesFromAnExactSourceTheManifestNamed,
) -> Result<WhatTheUpdateCheckFound, WhyTheUpdateCheckFailed> {
    let address = WHERE_THE_DESKTOP_READS_ITS_UPDATE_FILE;
    let file = fetch
        .fetch(address)
        .map_err(|why| WhyTheUpdateCheckFailed::TheFileCouldNotBeFetched {
            said: what_the_fetch_said(why),
        })?;
    if file.len() as u64 > THE_MOST_THE_UPDATE_FILE_MAY_BE {
        return Err(WhyTheUpdateCheckFailed::ItIsLargerThanACopyReads {
            length: file.len() as u64,
        });
    }
    let offered = what_every_row_is_offered(carried, earlier, &file)
        .map_err(|why| WhyTheUpdateCheckFailed::ItIsNotBelieved { why })?;
    let mut fetched: Vec<String> = Vec::new();
    for entry in &offered.entries {
        let at = where_an_artifact_it_names_is(address, &entry.file_name);
        if fetched.contains(&at) {
            continue;
        }
        let bytes = fetch
            .fetch(&at)
            .map_err(|why| WhyTheUpdateCheckFailed::AnArtifactCouldNotBeFetched {
                address: at.clone(),
                said: what_the_fetch_said(why),
            })?;
        the_artifact_is_the_one_offered(entry, &bytes).map_err(|why| {
            WhyTheUpdateCheckFailed::AnArtifactIsNotTheOneNamed {
                address: at.clone(),
                why,
            }
        })?;
        fetched.push(at);
    }
    Ok(WhatTheUpdateCheckFound { offered, fetched })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A fetch that answers from a table written down in the test, so the whole
    /// check can be run without anything being published.
    struct AFetchFromATableWrittenDownHere {
        table: Vec<(String, Vec<u8>)>,
        asked_for: Vec<String>,
    }

    impl FetchesFromAnExactSourceTheManifestNamed for AFetchFromATableWrittenDownHere {
        fn fetch(&mut self, source: &str) -> Result<Vec<u8>, WhyTheFetchFailed> {
            self.asked_for.push(source.to_owned());
            self.table
                .iter()
                .find(|(named, _)| named == source)
                .map(|(_, bytes)| bytes.clone())
                .ok_or(WhyTheFetchFailed::ItCouldNotBeReached {
                    said: format!("nothing at {source}"),
                })
        }
    }

    /// **The check is run against a release made by this crate's own making
    /// half**, which is as close to the real act as anything can get without
    /// publishing.
    fn a_release_at(where_the_release_file_is: &str, where_the_artifact_is: &str) -> (Vec<u8>, Vec<u8>) {
        use smoozy_server_protocol::release::composing::{
            compose_a_manifest, compose_an_artifact_entry, THE_FIXED_ENTRY_POINT_NAME,
        };
        use smoozy_server_protocol::release::{
            a_signature_over, THE_KEY_SET_SIGNING_CONTEXT, THE_MANIFEST_SIGNING_CONTEXT,
        };

        let _ = where_the_release_file_is;
        // **The real root, whose secret half nobody here holds**, cannot sign a
        // test release — so this one is signed under a key written down here and
        // the test below checks the refusal rather than the acceptance.
        let key = smoozy_server_protocol::release::composing::test_only_keys::a_key_pair_from_a_written_down_seed(
            *b"an outside check's own key, writ",
        );
        let artifact_bytes = b"a machine program".to_vec();
        let manifest = compose_a_manifest(
            1,
            "0.1.0",
            1,
            None,
            vec![compose_an_artifact_entry(
                "linux-x86_64-gnu",
                the_tuple_this_build_is_made_for(),
                &artifact_bytes,
                THE_FIXED_ENTRY_POINT_NAME,
                &[where_the_artifact_is.into()],
            )],
        );
        let statement = crate::making::compose_the_key_set_statement(1, key.public, "a test key");
        let statement_bytes = statement.canonical_bytes();
        let manifest_bytes = manifest.canonical_bytes();
        let offered = AnOfferedRelease {
            its_root_signature: a_signature_over(
                &key.secret,
                THE_KEY_SET_SIGNING_CONTEXT,
                &statement_bytes,
            ),
            the_key_set_statements_bytes: statement_bytes,
            its_online_signature: a_signature_over(
                &key.secret,
                THE_MANIFEST_SIGNING_CONTEXT,
                &manifest_bytes,
            ),
            the_manifests_bytes: manifest_bytes,
        };
        (offered.canonical_bytes(), artifact_bytes)
    }

    #[test]
    fn a_release_signed_by_a_key_this_build_does_not_carry_is_refused_from_the_outside() {
        let release_at = "https://example.invalid/releases/download/release-1/release";
        let artifact_at = "https://example.invalid/releases/download/release-1/smoozy-server";
        let (release, artifact) = a_release_at(release_at, artifact_at);
        let mut fetch = AFetchFromATableWrittenDownHere {
            table: vec![
                (release_at.into(), release),
                (artifact_at.into(), artifact),
            ],
            asked_for: Vec::new(),
        };
        // **The root this build carries did not sign it**, so the check refuses
        // it before it reaches the artifact.
        assert!(matches!(
            check_a_published_release(release_at, &mut fetch, None),
            Err(WhyTheCheckFailed::ItDidNotVerify { .. })
        ));
        assert_eq!(fetch.asked_for, vec![release_at.to_owned()]);
    }

    #[test]
    fn bytes_that_are_not_a_release_are_refused_rather_than_half_read() {
        let release_at = "https://example.invalid/release";
        let mut fetch = AFetchFromATableWrittenDownHere {
            table: vec![(release_at.into(), b"not a release at all".to_vec())],
            asked_for: Vec::new(),
        };
        assert!(matches!(
            check_a_published_release(release_at, &mut fetch, None),
            Err(WhyTheCheckFailed::ThoseBytesAreNotARelease)
        ));
    }

    /// The key those releases are signed under, so the check can be run whole
    /// rather than only its refusals. **The real root is the Creator's.**
    fn a_key() -> smoozy_server_protocol::release::composing::test_only_keys::AKeyPair {
        smoozy_server_protocol::release::composing::test_only_keys::a_key_pair_from_a_written_down_seed(
            *b"an outside check's own key, writ",
        )
    }

    /// One release, at the sequence and generation named, over the sources
    /// named.
    fn a_release_over(
        release_sequence: u64,
        key_set_generation: u64,
        sources: &[String],
    ) -> (Vec<u8>, Vec<u8>) {
        use smoozy_server_protocol::release::composing::{
            compose_a_manifest, compose_an_artifact_entry, THE_FIXED_ENTRY_POINT_NAME,
        };
        use smoozy_server_protocol::release::{
            a_signature_over, THE_KEY_SET_SIGNING_CONTEXT, THE_MANIFEST_SIGNING_CONTEXT,
        };

        let key = a_key();
        let artifact_bytes = b"a machine program".to_vec();
        let manifest = compose_a_manifest(
            release_sequence,
            "0.1.0",
            1,
            None,
            vec![compose_an_artifact_entry(
                "linux-x86_64-gnu",
                the_tuple_this_build_is_made_for(),
                &artifact_bytes,
                THE_FIXED_ENTRY_POINT_NAME,
                sources,
            )],
        );
        let statement =
            crate::making::compose_the_key_set_statement(key_set_generation, key.public, "a test key");
        let statement_bytes = statement.canonical_bytes();
        let manifest_bytes = manifest.canonical_bytes();
        let offered = AnOfferedRelease {
            its_root_signature: a_signature_over(
                &key.secret,
                THE_KEY_SET_SIGNING_CONTEXT,
                &statement_bytes,
            ),
            the_key_set_statements_bytes: statement_bytes,
            its_online_signature: a_signature_over(
                &key.secret,
                THE_MANIFEST_SIGNING_CONTEXT,
                &manifest_bytes,
            ),
            the_manifests_bytes: manifest_bytes,
        };
        (offered.canonical_bytes(), artifact_bytes)
    }

    /// **Every source the entry names is fetched, not the first.**
    ///
    /// **A second source serving the wrong bytes used to pass this check
    /// untouched**, because only the first was ever asked. A desktop may fetch
    /// from any of them, so a source nobody asked is a source nobody tested.
    #[test]
    fn a_second_source_serving_the_wrong_bytes_is_caught() {
        let release_at = "https://example.invalid/release";
        let first = "https://example.invalid/one/smoozy-server";
        let second = "https://example.invalid/two/smoozy-server";
        // Above the floor this build carries, so what is checked is the
        // sources. *(Sequence 1 and generation 1 until 2026-09-21, which sat
        // below the build's floor once that was raised off zero.)*
        let (release, artifact) = a_release_over(
            THE_BUILDS_SEQUENCE + 1,
            THE_BUILDS_GENERATION,
            &[first.into(), second.into()],
        );

        // Both sources serve the artifact: the check passes and says it asked
        // for both.
        let mut fetch = AFetchFromATableWrittenDownHere {
            table: vec![
                (release_at.into(), release.clone()),
                (first.into(), artifact.clone()),
                (second.into(), artifact.clone()),
            ],
            asked_for: Vec::new(),
        };
        let found = check_a_published_release_against(&a_key().public, release_at, &mut fetch, None)
            .expect("both sources serve the artifact");
        assert_eq!(
            found.the_sources_the_artifact_was_fetched_from,
            vec![first.to_owned(), second.to_owned()]
        );
        assert_eq!(
            fetch.asked_for,
            vec![release_at.to_owned(), first.to_owned(), second.to_owned()]
        );

        // The second serves something else: the check refuses and names it.
        let mut wrong = artifact.clone();
        wrong[0] ^= 1;
        let mut fetch = AFetchFromATableWrittenDownHere {
            table: vec![
                (release_at.into(), release),
                (first.into(), artifact),
                (second.into(), wrong),
            ],
            asked_for: Vec::new(),
        };
        match check_a_published_release_against(&a_key().public, release_at, &mut fetch, None) {
            Err(WhyTheCheckFailed::TheArtifactIsNotTheOneTheManifestNames { source, .. }) => {
                assert_eq!(source, second);
            }
            other => panic!("a second source serving the wrong bytes passed: {other:?}", other = other.map(|_| "it passed")),
        }
    }

    /// The floor this build carries, as the tests below build on it.
    const THE_BUILDS_SEQUENCE: u64 =
        smoozy_server_protocol::release::trust::THE_GREATEST_RELEASE_SEQUENCE_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE;
    const THE_BUILDS_GENERATION: u64 =
        smoozy_server_protocol::release::trust::THE_GREATEST_KEY_SET_GENERATION_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE;

    /// **The check can run the refusal a real desktop would**, which is the
    /// case it could not see before: the ordinary user holds a floor.
    ///
    /// *(Until 2026-09-21 the release here was sequence 5 at generation 2 and
    /// the typed floors were written as bare numbers just above it. That read
    /// as "a copy that has never accepted a release passes it" only while the
    /// build's own floor was zero; every number is now written from the build's
    /// floor, so the typed floor is still what refuses.)*
    #[test]
    fn a_release_below_a_typed_floor_is_refused_the_way_a_real_desktop_refuses_it() {
        let release_at = "https://example.invalid/release";
        let source = "https://example.invalid/smoozy-server";
        let (release, artifact) = a_release_over(
            THE_BUILDS_SEQUENCE + 5,
            THE_BUILDS_GENERATION + 1,
            &[source.into()],
        );
        let table = vec![
            (release_at.to_owned(), release),
            (source.to_owned(), artifact),
        ];

        // With no floor typed, it passes — which is a copy that has never
        // accepted a release, held only to this build's floor, and is not the
        // ordinary user.
        let mut fetch = AFetchFromATableWrittenDownHere {
            table: table.clone(),
            asked_for: Vec::new(),
        };
        assert!(check_a_published_release_against(&a_key().public, release_at, &mut fetch, None).is_ok());

        // As a desktop that has accepted a later release sequence, it is
        // refused.
        let mut fetch = AFetchFromATableWrittenDownHere {
            table: table.clone(),
            asked_for: Vec::new(),
        };
        assert!(matches!(
            check_a_published_release_against(
                &a_key().public,
                release_at,
                &mut fetch,
                Some(AsADesktopThatHasAccepted {
                    key_set_generation: THE_BUILDS_GENERATION + 1,
                    release_sequence: THE_BUILDS_SEQUENCE + 6,
                })
            ),
            Err(WhyTheCheckFailed::ItDidNotVerify { .. })
        ));
        // Nothing was fetched past the release file: the refusal came first.
        assert_eq!(fetch.asked_for, vec![release_at.to_owned()]);

        // And as a desktop that has accepted a later key-set generation.
        let mut fetch = AFetchFromATableWrittenDownHere {
            table,
            asked_for: Vec::new(),
        };
        assert!(matches!(
            check_a_published_release_against(
                &a_key().public,
                release_at,
                &mut fetch,
                Some(AsADesktopThatHasAccepted {
                    key_set_generation: THE_BUILDS_GENERATION + 2,
                    release_sequence: 1,
                })
            ),
            Err(WhyTheCheckFailed::ItDidNotVerify { .. })
        ));
    }

    /// **A floor typed below the one this build carries does not lower it**,
    /// because no desktop of this build holds a lower one. Added 2026-09-21,
    /// the day the build's pair was raised off zero.
    #[test]
    fn a_typed_floor_below_the_builds_own_does_not_lower_it() {
        let release_at = "https://example.invalid/release";
        let source = "https://example.invalid/smoozy-server";
        let (release, artifact) = a_release_over(
            THE_BUILDS_SEQUENCE - 1,
            THE_BUILDS_GENERATION,
            &[source.into()],
        );
        let mut fetch = AFetchFromATableWrittenDownHere {
            table: vec![
                (release_at.to_owned(), release),
                (source.to_owned(), artifact),
            ],
            asked_for: Vec::new(),
        };
        assert!(matches!(
            check_a_published_release_against(
                &a_key().public,
                release_at,
                &mut fetch,
                Some(AsADesktopThatHasAccepted {
                    key_set_generation: 0,
                    release_sequence: 0,
                })
            ),
            Err(WhyTheCheckFailed::ItDidNotVerify { .. })
        ));
    }

    #[test]
    fn an_address_that_answers_nothing_is_said_plainly() {
        let mut fetch = AFetchFromATableWrittenDownHere {
            table: Vec::new(),
            asked_for: Vec::new(),
        };
        assert!(matches!(
            check_a_published_release("https://example.invalid/nothing", &mut fetch, None),
            Err(WhyTheCheckFailed::TheReleaseFileCouldNotBeFetched { .. })
        ));
    }

    // ------------------------------------------- the desktop update file

    use crate::the_desktop_update::tests::{a_build_carrying_the_keys, every_rows_bytes, the_everyday_key};
    use crate::the_desktop_update::{compose_the_update_file, ARowsArtifact, THE_EARLIER_TRUST_STATEMENTS};

    /// A published release as a table: the update file at its fixed address
    /// and every row's artifact beside it.
    fn a_published_update(carried: &TheCarriedSet) -> Vec<(String, Vec<u8>)> {
        let rows = every_rows_bytes();
        let artifacts: Vec<ARowsArtifact<'_>> = rows
            .iter()
            .map(|(row, name, bytes)| ARowsArtifact {
                row,
                file_name: name,
                bytes,
            })
            .collect();
        let (file, _) = compose_the_update_file(
            carried,
            THE_EARLIER_TRUST_STATEMENTS,
            1700,
            "0.2.0",
            &artifacts,
            &the_everyday_key(),
        )
        .unwrap();
        let mut table = vec![(WHERE_THE_DESKTOP_READS_ITS_UPDATE_FILE.to_owned(), file)];
        for (_, name, bytes) in &rows {
            table.push((
                where_an_artifact_it_names_is(WHERE_THE_DESKTOP_READS_ITS_UPDATE_FILE, name),
                bytes.clone(),
            ));
        }
        table
    }

    #[test]
    fn an_artifact_is_looked_for_beside_the_update_file() {
        assert_eq!(
            where_an_artifact_it_names_is(WHERE_THE_DESKTOP_READS_ITS_UPDATE_FILE, "Smoozy-macos.zip"),
            "https://github.com/a-kislyakov/Smoozy-releases/releases/latest/download/Smoozy-macos.zip"
        );
        assert!(WHERE_THE_DESKTOP_READS_ITS_UPDATE_FILE
            .ends_with(&format!("/{}", crate::the_desktop_update::THE_UPDATE_FILES_NAME)));
    }

    /// **A published update file the release composed checks out the way a
    /// copy will read it**, and every row's artifact is fetched and held.
    #[test]
    fn a_published_update_file_checks_out_through_the_desktops_own_reading() {
        let carried = a_build_carrying_the_keys();
        let mut fetch = AFetchFromATableWrittenDownHere {
            table: a_published_update(&carried),
            asked_for: Vec::new(),
        };
        let found = check_the_published_update_file(&carried, THE_EARLIER_TRUST_STATEMENTS, &mut fetch)
            .expect("it checks out");
        assert_eq!(found.offered.release_sequence, 1700);
        assert_eq!(found.offered.signed_by, ["everyday-1"]);
        assert_eq!(found.fetched.len(), every_rows_bytes().len());
        assert_eq!(fetch.asked_for[0], WHERE_THE_DESKTOP_READS_ITS_UPDATE_FILE);
        assert_eq!(fetch.asked_for.len(), 1 + every_rows_bytes().len());
    }

    /// **A published artifact that is not the one named is caught**, by its
    /// length and by its digest, and the address is named.
    #[test]
    fn a_published_artifact_that_is_not_the_one_named_is_caught() {
        let carried = a_build_carrying_the_keys();
        let changes: [fn(&mut Vec<u8>); 2] = [|bytes| bytes.push(0), |bytes| bytes[0] ^= 1];
        for change in changes {
            let mut table = a_published_update(&carried);
            change(&mut table[2].1);
            let wrong = table[2].0.clone();
            let mut fetch = AFetchFromATableWrittenDownHere {
                table,
                asked_for: Vec::new(),
            };
            match check_the_published_update_file(&carried, THE_EARLIER_TRUST_STATEMENTS, &mut fetch) {
                Err(WhyTheUpdateCheckFailed::AnArtifactIsNotTheOneNamed { address, .. }) => {
                    assert_eq!(address, wrong);
                }
                other => panic!("a wrong artifact passed: {:?}", other.map(|_| "it passed")),
            }
        }
    }

    /// **A file this build would not believe is refused**, whatever is beside
    /// it — here, one signed for master keys this build does not carry.
    #[test]
    fn a_published_update_file_this_build_would_not_believe_is_refused() {
        let carried = a_build_carrying_the_keys();
        let mut fetch = AFetchFromATableWrittenDownHere {
            table: a_published_update(&carried),
            asked_for: Vec::new(),
        };
        // **This very build**, which carries no master key.
        assert!(matches!(
            check_the_published_update_file(
                &TheCarriedSet::this_build(),
                THE_EARLIER_TRUST_STATEMENTS,
                &mut fetch
            ),
            Err(WhyTheUpdateCheckFailed::ItIsNotBelieved { .. })
        ));
        assert_eq!(fetch.asked_for, vec![WHERE_THE_DESKTOP_READS_ITS_UPDATE_FILE.to_owned()]);

        let mut fetch = AFetchFromATableWrittenDownHere {
            table: vec![(
                WHERE_THE_DESKTOP_READS_ITS_UPDATE_FILE.to_owned(),
                vec![0; THE_MOST_THE_UPDATE_FILE_MAY_BE as usize + 1],
            )],
            asked_for: Vec::new(),
        };
        assert!(matches!(
            check_the_published_update_file(&carried, THE_EARLIER_TRUST_STATEMENTS, &mut fetch),
            Err(WhyTheUpdateCheckFailed::ItIsLargerThanACopyReads { .. })
        ));

        let mut fetch = AFetchFromATableWrittenDownHere {
            table: Vec::new(),
            asked_for: Vec::new(),
        };
        assert!(matches!(
            check_the_published_update_file(&carried, THE_EARLIER_TRUST_STATEMENTS, &mut fetch),
            Err(WhyTheUpdateCheckFailed::TheFileCouldNotBeFetched { .. })
        ));
    }
}
