//! **Making one release: compose, sign, verify, and only then write.**
//!
//! **Nothing here re-implements the release contract.** The manifest, the
//! key-set statement, the canonical bytes, the signature and the verification
//! are all `Server protocol and transfers`' own — B8, B9 and B10 — and this
//! file only supplies the values and holds the order.
//!
//! **The order is the whole of the safety, and it is this:** the secret is
//! proved to be the root this build carries before anything is signed; the two
//! documents are signed; **the result is verified through the desktop's own
//! `verify_before_download`**; and the folder is written last. **A release that
//! does not verify writes no file at all**, so there is nothing on disk for
//! anybody to publish by mistake.
//!
//! **From 2026-10-25 (step 11's D8) the release also writes the desktop update
//! file**, `desktop-update`, once the build carries the update master keys:
//! composed and signed by `crate::the_desktop_update` with the everyday key,
//! and believed through the desktop's own reading for every row before
//! anything is written. It is written after the server package's release file,
//! and a release that would carry a file the desktop refuses writes nothing.
//!
//! **Markers used below, each with the document it belongs to.** `Server
//! protocol and transfers` B8, B8a, B9, B10.

use smoozy_server_protocol::framing::DeclaredVersions;
use smoozy_server_protocol::release::composing::{
    compose_a_manifest, compose_an_artifact_entry, the_tuple_this_build_is_made_for,
    THE_FIXED_ENTRY_POINT_NAME,
};
use smoozy_server_protocol::release::trust::{AnAllowedKey, KeySetStatement, THE_KEY_SET_FORMAT};
use smoozy_server_protocol::release::verifying::{
    an_https_address, verify_before_download, AnOfferedRelease,
};
use smoozy_server_protocol::release::{
    a_signature_over, PublicKeyBytes, THE_KEY_SET_SIGNING_CONTEXT, THE_MANIFEST_SIGNING_CONTEXT,
};
use smoozy_server_protocol::transfers::integrity::Digest;

use crate::secret::{TheSigningSecret, TheUpdateSigningKey};
use crate::the_desktop_update::{
    compose_the_update_file, ARowsArtifact, WhatEveryRowIsOffered, WhyTheUpdateFileWasNotMade,
    THE_UPDATE_FILES_NAME,
};
use smoozy_desktop::updating::trust::{ACarriedStatement, TheCarriedSet};

/// **The identity the key-set statement gives the key that signs a manifest.**
///
/// **One key does both jobs today and that is a decision with a cost, written
/// down at `Audits/Build/Step 8a/Decisions record.md` rather than hidden here.**
/// B9's shape is an offline root that vouches for online manifest-signing keys,
/// and the project holds exactly one key — the root generated on 2026-09-06,
/// whose secret half is in the Mac's keychain and the Creator's password
/// manager. **So generation 1 of the key-set statement names that same key as
/// the one allowed manifest key.**
///
/// **What it costs:** a key-set statement can normally retire a compromised
/// online key, and it cannot retire this one, because retiring it would need a
/// signature from itself.
///
/// **And the other cost, which is the word *offline*.** While this key is the
/// one that signs a manifest, **it is not offline in B9's sense**: it is read out
/// of a keychain on the machine that builds Smoozy and used at every release, and
/// the word B9 uses buys more than the revocation machinery. **That machinery is
/// built, tested and inert** — the allowed set, the validity sequences and the
/// revocations all work, and there is nothing here they can revoke — so a reader
/// meeting them could conclude the product has a revocation route it does not
/// have.
///
/// **What removes both costs is one value and no code** — a second key, with its
/// own keychain entry — and [`compose_the_key_set_statement`] takes the allowed
/// key as an argument precisely so that the day it exists is a day nothing here
/// changes.
pub const THE_IDENTITY_OF_THE_KEY_THAT_SIGNS_A_MANIFEST: &str = "release root 2026-09-06";

/// **The name the one distributed file is written and published under.**
///
/// It carries the key-set statement, the root's signature over it, the
/// manifest, and the manifest signature — B9's *distributed with the manifest*,
/// as one thing that can be fetched with one address.
pub const THE_RELEASE_FILES_NAME: &str = "release";

/// The name the plain listing is written under.
pub const THE_LISTINGS_NAME: &str = "listing.txt";

/// One thing a release publishes.
pub struct AnArtifactToPublish {
    /// The name it is written and published under.
    pub name: String,
    /// Where its bytes are now.
    pub found_at: std::path::PathBuf,
    /// What it is, in the listing.
    pub what_it_is: String,
}

/// Everything one release is made of.
pub struct WhatToRelease {
    pub release_sequence: u64,
    pub package_version: String,
    pub key_set_generation: u64,
    /// The rented machine's own program — **the one artifact the signed
    /// manifest names**.
    pub the_machine_program: AnArtifactToPublish,
    /// **The exact HTTPS sources the manifest names for it.**
    ///
    /// **B8a: nothing derives a source from a version, a name or a pattern.**
    /// These are values the release runner is given and passes in whole, and no
    /// line of this crate composes one.
    pub its_exact_sources: Vec<String>,
    /// The desktop application's artifacts. **They are published and listed and
    /// they are not in the server package's signed manifest** — see
    /// [`compose_the_manifest`]. Each row's is named in the desktop update file
    /// instead, through [`WhatToRelease::the_rows`].
    pub the_desktop_artifacts: Vec<AnArtifactToPublish>,
    /// **Which published desktop artifact each release-matrix row is offered**,
    /// as (row, the name it is published under). Read only when the desktop
    /// update file is written.
    pub the_rows: Vec<(String, String)>,
}

/// **Whether this release writes the desktop update file, and with what.**
pub enum TheDesktopUpdate<'a> {
    /// **The build carries no update master key**, so no copy of Smoozy could
    /// believe a file; none is written and the everyday key is not read.
    NotWritten,
    /// The build carries the keys: the file is written, signed by `key`, with
    /// the build's own statement after `earlier`.
    Written {
        carried: &'a TheCarriedSet,
        earlier: &'a [ACarriedStatement],
        key: &'a TheUpdateSigningKey,
    },
}

/// **Why a release was not made**, and none of these leaves a written file
/// behind.
#[derive(Debug)]
pub enum WhyTheReleaseWasNotMade {
    AnArtifactCouldNotBeRead { name: String, said: String },
    TheFolderIsNotEmptyOrCannotBeMade { said: String },
    /// **The release did not verify through the desktop's own path.** Nothing is
    /// written. This is the check the release runner's fifth step names, made
    /// here so that a failure cannot be published past.
    ItDidNotVerify { said: String },
    /// **The release verified and writing it failed part way through.**
    ///
    /// **Files may be on the disk and they are named rather than assumed away.**
    /// The machine program, each desktop artifact, the release file and the
    /// listing are written one after another, so a failure after some have
    /// landed leaves them there — and an ending that said *nothing was written*
    /// sent a release runner to run the same command again, be refused for the
    /// folder not being empty, and work out why.
    ///
    /// **Nothing is deleted.** Removing files after a failure is a second thing
    /// that can fail, and it would take the evidence with it.
    ItCouldNotBeWritten {
        said: String,
        and_the_folder_now_holds: Vec<String>,
    },
    /// No source was given for the machine program, and B8 requires at least
    /// one exact HTTPS source.
    TheMachineProgramHasNoSource,
    /// A source that is not an HTTPS address. **B8 says HTTPS and B8a says an
    /// unauthenticated fetch must be able to answer it.**
    ASourceIsNotAnHttpsAddress { said: String },
    /// **B8c: the machine's program asks the machine for a release of its own C
    /// library, so it will not run on every system the plans promise.**
    ///
    /// **This is the fault a real rental found and no desk check had.** Release
    /// 1638's artifact wanted release 2.39 of that library; Ubuntu 22.04 carries
    /// 2.35; the placed program died five times in one second and the setup
    /// could not finish. `Server Setup` M1a names Ubuntu and Debian and names no
    /// version of either, and a manifest has nowhere to write a floor — so an
    /// artifact with one cannot be published, whatever the floor is.
    TheMachineProgramAsksTheMachineForACLibrary {
        found: smoozy_server_protocol::release::composing::TheCLibraryReleasesTheArtifactAsksFor,
    },
    /// **The desktop update file was not made, or the desktop's own reading
    /// refused it.** Nothing is written.
    TheDesktopUpdateFileWasNotMade { why: WhyTheUpdateFileWasNotMade },
}

/// **Every ending, said in plain English**, because what this tool prints is
/// read by whoever is making the release.
impl std::fmt::Display for WhyTheReleaseWasNotMade {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WhyTheReleaseWasNotMade::AnArtifactCouldNotBeRead { name, said } => {
                write!(out, "{name} could not be read: {said}")
            }
            WhyTheReleaseWasNotMade::TheFolderIsNotEmptyOrCannotBeMade { said } => write!(
                out,
                "the folder to write the release into is not usable: {said}"
            ),
            WhyTheReleaseWasNotMade::ItDidNotVerify { said } => write!(
                out,
                "the release did not verify through the desktop's own path, so nothing was \
                 written and nothing can be published: {said}"
            ),
            WhyTheReleaseWasNotMade::ItCouldNotBeWritten { said, .. } => {
                write!(out, "the release could not be written: {said}")
            }
            WhyTheReleaseWasNotMade::TheMachineProgramHasNoSource => write!(
                out,
                "no source was given for the machine's own program, and a manifest must name at \
                 least one exact address the artifact will be served from"
            ),
            WhyTheReleaseWasNotMade::ASourceIsNotAnHttpsAddress { said } => write!(
                out,
                "{said} is not an https address, and a source that is not one cannot be named in \
                 a manifest"
            ),
            WhyTheReleaseWasNotMade::TheMachineProgramAsksTheMachineForACLibrary { found } => {
                write!(
                    out,
                    "the machine's own program was not built for every machine the plans promise: \
                     {found}. Build it for the target that carries its own C library — the \
                     workflow's own step does — and run this again"
                )
            }
            WhyTheReleaseWasNotMade::TheDesktopUpdateFileWasNotMade { why } => write!(
                out,
                "the desktop update file was not made, so nothing was written: {why}"
            ),
        }
    }
}

/// **Compose the key-set statement B9 requires, naming the keys allowed to sign
/// a manifest.**
///
/// The allowed key is an argument rather than a constant, so that the day a
/// separate manifest-signing key exists it is passed in and nothing here moves.
/// **Its validity starts at zero** — there is one key and it has been allowed
/// for every release there has ever been.
pub fn compose_the_key_set_statement(
    generation: u64,
    the_key_allowed_to_sign_a_manifest: PublicKeyBytes,
    its_identity: &str,
) -> KeySetStatement {
    KeySetStatement {
        statement_format: THE_KEY_SET_FORMAT.into(),
        generation,
        allowed: vec![AnAllowedKey {
            identity: its_identity.into(),
            public_key: the_key_allowed_to_sign_a_manifest,
            valid_from_release_sequence: 0,
        }],
        revocations: Vec::new(),
    }
}

/// **Compose the manifest, over the one artifact that belongs in it.**
///
/// **The desktop application's artifacts are deliberately not in here.** This
/// is B8's *server package* manifest: a desktop selects an artifact out of it
/// by the tuple `Server Setup` read off a rented machine, and an entry that is
/// not a rented machine's program has no business being selectable that way.
/// **The desktop application's own signed version manifest is `Desktop app and
/// delivery` §19's E15 and it is not built**, that document's signing,
/// packaging and update route being step 11's work; the desktop artifacts are
/// published as files and listed, with what that means said out loud in the
/// listing.
///
/// **The protocol versions come from the code and not from a hand.** A manifest
/// that disagreed with the build about which protocol version it speaks would
/// be a manifest nobody could have typed correctly twice.
pub fn compose_the_manifest(
    release_sequence: u64,
    package_version: &str,
    the_machine_programs_bytes: &[u8],
    its_exact_sources: &[String],
) -> smoozy_server_protocol::release::manifest::ReleaseManifest {
    let declared = DeclaredVersions::of_this_build();
    let artifact = compose_an_artifact_entry(
        // **The artifact identity is the tuple it is for**, which is stable
        // across releases and is what the package-phase record carries.
        "linux-x86_64-gnu",
        the_tuple_this_build_is_made_for(),
        the_machine_programs_bytes,
        THE_FIXED_ENTRY_POINT_NAME,
        its_exact_sources,
    );
    compose_a_manifest(
        release_sequence,
        package_version,
        declared.current().0,
        declared.the_single_immediately_previous().map(|it| it.0),
        vec![artifact],
    )
}

/// What was made, once it has verified.
pub struct TheReleaseThatWasMade {
    pub written_into: std::path::PathBuf,
    /// Every file in the folder, with what the listing says about it.
    pub files: Vec<(String, u64, String)>,
    pub the_manifests_digest: String,
    pub signed_by: String,
    /// **What the desktop's own reading found in the update file written**, or
    /// none where the build carries no master key and none was written.
    pub the_desktop_update: Option<WhatEveryRowIsOffered>,
}

/// **Make one release: compose, sign, verify, write.**
pub fn make_a_release(
    what: &WhatToRelease,
    secret: &TheSigningSecret,
    root: &PublicKeyBytes,
    update: TheDesktopUpdate<'_>,
    into: &std::path::Path,
) -> Result<TheReleaseThatWasMade, WhyTheReleaseWasNotMade> {
    if what.its_exact_sources.is_empty() {
        return Err(WhyTheReleaseWasNotMade::TheMachineProgramHasNoSource);
    }
    for source in &what.its_exact_sources {
        // **B8's HTTPS test lives in the module that owns the clause**, and this
        // calls it rather than keeping a second copy: a rule with two homes is a
        // rule one of whose homes goes stale.
        if !an_https_address(source) {
            return Err(WhyTheReleaseWasNotMade::ASourceIsNotAnHttpsAddress {
                said: source.clone(),
            });
        }
    }

    let machine_program = read_it(&what.the_machine_program)?;

    // **B8c's check, over the bytes that are about to be signed, and before
    // anything else is done with them.** The rule and the reading live in the
    // module that owns B8, for the reason the HTTPS test above gives: a rule
    // with two homes is a rule one of whose homes goes stale. What it is worth
    // and what it cannot see are written at the function itself.
    //
    // **It is here rather than in the workflow alone** because the workflow is
    // not what publishes: a release is composed at the desk out of whatever
    // artifact the release runner hands it, downloaded from a run or lying in a
    // folder, and this is the last place anything reads those bytes before a
    // signature goes over them.
    smoozy_server_protocol::release::composing::the_artifact_needs_no_c_library_from_the_machine(
        &machine_program,
    )
    .map_err(
        |found| WhyTheReleaseWasNotMade::TheMachineProgramAsksTheMachineForACLibrary { found },
    )?;

    let mut desktop = Vec::new();
    for artifact in &what.the_desktop_artifacts {
        desktop.push((artifact, read_it(artifact)?));
    }

    // ---- compose and sign

    let statement = compose_the_key_set_statement(
        what.key_set_generation,
        // **The key that signs a manifest is the root, today.** See the
        // identity constant above for what that costs and what removes it.
        *root,
        THE_IDENTITY_OF_THE_KEY_THAT_SIGNS_A_MANIFEST,
    );
    let statement_bytes = statement.canonical_bytes();
    let its_root_signature = a_signature_over(
        secret.bytes(),
        THE_KEY_SET_SIGNING_CONTEXT,
        &statement_bytes,
    );

    let manifest = compose_the_manifest(
        what.release_sequence,
        &what.package_version,
        &machine_program,
        &what.its_exact_sources,
    );
    let manifest_bytes = manifest.canonical_bytes();
    let its_online_signature =
        a_signature_over(secret.bytes(), THE_MANIFEST_SIGNING_CONTEXT, &manifest_bytes);

    let offered = AnOfferedRelease {
        the_key_set_statements_bytes: statement_bytes,
        its_root_signature,
        the_manifests_bytes: manifest_bytes,
        its_online_signature,
    };

    // ---- verify, before anything is written

    let verified = verify_before_download(
        root,
        None,
        &offered,
        &the_tuple_this_build_is_made_for(),
        None,
    )
    .map_err(|why| WhyTheReleaseWasNotMade::ItDidNotVerify {
        said: format!("{why:?}"),
    })?;

    // ---- the desktop update file, composed and believed the desktop's way

    let the_desktop_update = match update {
        TheDesktopUpdate::NotWritten => None,
        TheDesktopUpdate::Written {
            carried,
            earlier,
            key,
        } => {
            let not_made = |why| WhyTheReleaseWasNotMade::TheDesktopUpdateFileWasNotMade { why };
            let mut artifacts = Vec::new();
            for (row, name) in &what.the_rows {
                let Some((artifact, bytes)) =
                    desktop.iter().find(|(artifact, _)| artifact.name == *name)
                else {
                    return Err(not_made(
                        WhyTheUpdateFileWasNotMade::ARowNamesNoFileOfThisRelease {
                            row: row.clone(),
                            name: name.clone(),
                        },
                    ));
                };
                artifacts.push(ARowsArtifact {
                    row,
                    file_name: &artifact.name,
                    bytes,
                });
            }
            Some(
                compose_the_update_file(
                    carried,
                    earlier,
                    what.release_sequence,
                    &what.package_version,
                    &artifacts,
                    key,
                )
                .map_err(not_made)?,
            )
        }
    };

    // ---- write

    std::fs::create_dir_all(into).map_err(|failed| {
        WhyTheReleaseWasNotMade::TheFolderIsNotEmptyOrCannotBeMade {
            said: failed.to_string(),
        }
    })?;
    let already = std::fs::read_dir(into)
        .map_err(|failed| WhyTheReleaseWasNotMade::TheFolderIsNotEmptyOrCannotBeMade {
            said: failed.to_string(),
        })?
        .count();
    if already > 0 {
        return Err(WhyTheReleaseWasNotMade::TheFolderIsNotEmptyOrCannotBeMade {
            said: format!("{} already holds something", into.display()),
        });
    }

    let mut files = Vec::new();
    let mut write = |name: &str, bytes: &[u8], what_it_is: &str| -> Result<(), WhyTheReleaseWasNotMade> {
        std::fs::write(into.join(name), bytes).map_err(|failed| {
            WhyTheReleaseWasNotMade::ItCouldNotBeWritten {
                said: format!("{name}: {failed}"),
                and_the_folder_now_holds: what_the_folder_holds(into),
            }
        })?;
        files.push((
            format!("{name}  ({what_it_is})"),
            bytes.len() as u64,
            Digest::of(bytes).as_text(),
        ));
        Ok(())
    };

    write(
        &what.the_machine_program.name,
        &machine_program,
        &what.the_machine_program.what_it_is,
    )?;
    for (artifact, bytes) in &desktop {
        write(&artifact.name, bytes, &artifact.what_it_is)?;
    }
    // About and notices, wave 4: the server package stays unchanged; the
    // release itself carries the full GPL and that program's target notices.
    write("COPYING", include_bytes!("../../../COPYING"), "GNU GPL version 3")?;
    write(
        "THIRD-PARTY-LICENSES-server.txt",
        include_bytes!("../generated-notices/THIRD-PARTY-LICENSES-server.txt"),
        "third-party notices for the Linux-musl server program",
    )?;
    let the_release_file = offered.canonical_bytes();
    write(
        THE_RELEASE_FILES_NAME,
        &the_release_file,
        "the signed manifest, the root-signed key-set statement, and both signatures",
    )?;
    if let Some((file, _)) = &the_desktop_update {
        write(
            THE_UPDATE_FILES_NAME,
            file,
            "the desktop update file: the manifest naming each row's artifact, signed by the \
             everyday key, and every trust statement the master keys have signed",
        )?;
    }

    let listing = the_listing(
        what,
        &verified.signed_by,
        &files,
        the_desktop_update.as_ref().map(|(_, offered)| offered),
    );
    std::fs::write(into.join(THE_LISTINGS_NAME), listing.as_bytes()).map_err(|failed| {
        WhyTheReleaseWasNotMade::ItCouldNotBeWritten {
            said: format!("{THE_LISTINGS_NAME}: {failed}"),
            and_the_folder_now_holds: what_the_folder_holds(into),
        }
    })?;

    Ok(TheReleaseThatWasMade {
        written_into: into.to_path_buf(),
        files,
        the_manifests_digest: Digest::from_bytes(verified.manifest_digest).as_text(),
        signed_by: verified.signed_by,
        the_desktop_update: the_desktop_update.map(|(_, offered)| offered),
    })
}

/// **What is in the folder, by name, read rather than remembered.**
///
/// **It is read off the disk instead of being taken from what this code thinks
/// it wrote**, because a write that failed part way through may have left a file
/// this code does not count as written. **A folder that cannot be read gives an
/// empty list**, which the ending then says nothing about rather than claiming
/// the folder is empty.
fn what_the_folder_holds(into: &std::path::Path) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(into) else {
        return Vec::new();
    };
    let mut names: Vec<String> = entries
        .flatten()
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
}

fn read_it(artifact: &AnArtifactToPublish) -> Result<Vec<u8>, WhyTheReleaseWasNotMade> {
    std::fs::read(&artifact.found_at).map_err(|failed| {
        WhyTheReleaseWasNotMade::AnArtifactCouldNotBeRead {
            name: artifact.name.clone(),
            said: failed.to_string(),
        }
    })
}

/// **The plain listing, which carries no trust and says so.**
///
/// It exists so that a person looking at a published release can see what is in
/// it and check a file they downloaded by hand. **Nothing in Smoozy reads it**,
/// and it is not a manifest: the machine program's real length and digest are
/// in the signed manifest inside the release file, and the desktop artifacts
/// have no signed manifest at all yet.
fn the_listing(
    what: &WhatToRelease,
    signed_by: &str,
    files: &[(String, u64, String)],
    the_desktop_update: Option<&WhatEveryRowIsOffered>,
) -> String {
    let mut out = String::new();
    out.push_str("Smoozy release\n\n");
    out.push_str(&format!("version:          {}\n", what.package_version));
    out.push_str(&format!("release sequence: {}\n", what.release_sequence));
    out.push_str(&format!("manifest signed by: {signed_by}\n\n"));
    out.push_str("Files\n\n");
    for (name, length, digest) in files {
        out.push_str(&format!("  {name}\n    bytes: {length}\n    sha-256: {digest}\n"));
    }
    out.push_str(
        "\nThis listing is unsigned; use it to check a manual download. Smoozy trusts only the \
         signed manifest in `release`, verified against the release root in every build.\n",
    );
    match the_desktop_update {
        Some(offered) => {
            out.push_str(&format!(
                "\nDesktop update files by row, from `{THE_UPDATE_FILES_NAME}` (signed by {}):\n",
                offered.signed_by.join(" and ")
            ));
            for entry in &offered.entries {
                out.push_str(&format!("  {}: {}\n", entry.row, entry.file_name));
            }
        }
        None => out.push_str(
            "\nThis release carries no desktop update file: its build has no update master key.\n",
        ),
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_server_protocol::release::composing::test_only_keys;

    fn a_folder(named: &str) -> std::path::PathBuf {
        let at = std::env::temp_dir().join(format!(
            "smoozy-release-{named}-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let _ = std::fs::remove_dir_all(&at);
        at
    }

    fn a_key() -> test_only_keys::AKeyPair {
        test_only_keys::a_key_pair_from_a_written_down_seed(*b"a test release root, written dow")
    }

    fn an_artifact(at: &std::path::Path, name: &str, bytes: &[u8]) -> AnArtifactToPublish {
        std::fs::create_dir_all(at).unwrap();
        let found_at = at.join(name);
        std::fs::write(&found_at, bytes).unwrap();
        AnArtifactToPublish {
            name: name.into(),
            found_at,
            what_it_is: "for the sake of the argument".into(),
        }
    }

    /// **The release after the one this build knows about**: one past the
    /// release sequence it carries as its floor, at the generation it carries.
    /// *(Sequence 7 at generation 1 until 2026-09-21, which verified only while
    /// `Server protocol and transfers` B9's built-in pair was zero — the release
    /// is verified the desktop's way before anything is written, and with the
    /// pair at the first published release's numbers, 7 is below it.)*
    const THE_NEXT_RELEASE_SEQUENCE: u64 =
        smoozy_server_protocol::release::trust::THE_GREATEST_RELEASE_SEQUENCE_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE
            + 1;

    fn what_to_release(from: &std::path::Path) -> WhatToRelease {
        WhatToRelease {
            release_sequence: THE_NEXT_RELEASE_SEQUENCE,
            package_version: "0.1.0".into(),
            key_set_generation:
                smoozy_server_protocol::release::trust::THE_GREATEST_KEY_SET_GENERATION_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE,
            the_machine_program: an_artifact(from, "smoozy-server", b"a machine program"),
            its_exact_sources: vec![
                "https://example.invalid/releases/download/the-next-release/smoozy-server".into(),
            ],
            the_desktop_artifacts: vec![an_artifact(from, "Smoozy-macos.zip", b"a bundle")],
            the_rows: Vec::new(),
        }
    }

    /// **The whole act, end to end**, with a key that is written down in this
    /// file rather than the real one.
    #[test]
    fn a_release_is_made_and_what_it_wrote_verifies_the_way_the_desktop_will() {
        let from = a_folder("from");
        let into = a_folder("into");
        let key = a_key();
        let secret = crate::secret::for_tests_only(key.secret);
        let what = what_to_release(&from);

        let made = make_a_release(&what, &secret, &key.public, TheDesktopUpdate::NotWritten, &into).unwrap();
        assert_eq!(made.signed_by, THE_IDENTITY_OF_THE_KEY_THAT_SIGNS_A_MANIFEST);

        // Every artifact is in the folder, and the release file with them.
        assert!(into.join("smoozy-server").exists());
        assert!(into.join("Smoozy-macos.zip").exists());
        assert!(into.join(THE_LISTINGS_NAME).exists());
        assert_eq!(std::fs::read(into.join("COPYING")).unwrap(), include_bytes!("../../../COPYING"));
        let notices = std::fs::read_to_string(into.join("THIRD-PARTY-LICENSES-server.txt")).unwrap();
        assert!(notices.contains("smoozy-server — x86_64-unknown-linux-musl"));
        assert!(!notices.contains("smoozy-desktop —"));

        // **The check is the desktop's own path over the bytes that were
        // written**, not over the values that were composed.
        let written = std::fs::read(into.join(THE_RELEASE_FILES_NAME)).unwrap();
        let offered = AnOfferedRelease::read(&written).unwrap();
        let verified = verify_before_download(
            &key.public,
            None,
            &offered,
            &the_tuple_this_build_is_made_for(),
            None,
        )
        .unwrap();
        assert_eq!(verified.manifest.release_sequence, THE_NEXT_RELEASE_SEQUENCE);
        assert_eq!(verified.manifest.package_version, "0.1.0");
        assert_eq!(verified.selected.entry_point_name, THE_FIXED_ENTRY_POINT_NAME);
        assert_eq!(verified.selected.sources, what.its_exact_sources);
        // The length and the digest are the artifact's own, measured.
        let bytes = std::fs::read(into.join("smoozy-server")).unwrap();
        assert_eq!(verified.selected.byte_length, bytes.len() as u64);
        assert_eq!(verified.selected.digest, Digest::of(&bytes).bytes());

        // The desktop artifact is published and is not in the signed manifest.
        assert_eq!(verified.manifest.artifacts.len(), 1);

        let _ = std::fs::remove_dir_all(&from);
        let _ = std::fs::remove_dir_all(&into);
    }

    #[test]
    fn the_protocol_versions_come_from_the_build_rather_than_from_a_hand() {
        let declared = DeclaredVersions::of_this_build();
        let manifest = compose_the_manifest(1, "0.1.0", b"bytes", &["https://a".into()]);
        assert_eq!(manifest.protocol_version_current, declared.current().0);
        assert_eq!(
            manifest.protocol_version_previous_supported,
            declared.the_single_immediately_previous().map(|it| it.0)
        );
    }

    #[test]
    fn a_source_that_is_not_an_https_address_stops_the_release() {
        let from = a_folder("plain-http-from");
        let into = a_folder("plain-http-into");
        let key = a_key();
        let secret = crate::secret::for_tests_only(key.secret);
        let mut what = what_to_release(&from);
        what.its_exact_sources = vec!["http://example.invalid/smoozy-server".into()];
        assert!(matches!(
            make_a_release(&what, &secret, &key.public, TheDesktopUpdate::NotWritten, &into),
            Err(WhyTheReleaseWasNotMade::ASourceIsNotAnHttpsAddress { .. })
        ));
        assert!(!into.exists(), "a refused release wrote a folder");
        let _ = std::fs::remove_dir_all(&from);
    }

    /// **B8c: a machine program that asks the machine for a release of its own
    /// C library is refused, and nothing is written.**
    ///
    /// This is the fault the fifth pass on Sesterce met on a real rented
    /// machine — release 1638's artifact wanting release 2.39 on a machine
    /// carrying 2.35 — caught here, at the desk, before a signature goes over
    /// the bytes.
    #[test]
    fn a_machine_program_that_needs_the_machines_c_library_stops_the_release() {
        let from = a_folder("wants-a-c-library-from");
        let into = a_folder("wants-a-c-library-into");
        let key = a_key();
        let secret = crate::secret::for_tests_only(key.secret);
        let mut what = what_to_release(&from);
        what.the_machine_program = an_artifact(
            &from,
            "smoozy-server",
            b"a machine program built against GLIBC_2.39 and nothing older",
        );
        let why = match make_a_release(&what, &secret, &key.public, TheDesktopUpdate::NotWritten, &into) {
            Err(why) => why,
            Ok(_) => panic!("an artifact carrying the mark was published"),
        };
        match &why {
            WhyTheReleaseWasNotMade::TheMachineProgramAsksTheMachineForACLibrary { found } => {
                assert_eq!(found.asked_for, vec!["GLIBC_2.39".to_owned()]);
            }
            other => panic!("the release was refused for the wrong reason: {other:?}"),
        }
        // **The sentence a release runner reads names the release.**
        assert!(why.to_string().contains("GLIBC_2.39"));
        assert!(!into.exists(), "a refused release wrote a folder");
        let _ = std::fs::remove_dir_all(&from);
    }

    #[test]
    fn a_release_that_would_not_verify_writes_nothing() {
        let from = a_folder("wrong-key-from");
        let into = a_folder("wrong-key-into");
        let key = a_key();
        let stranger =
            test_only_keys::a_key_pair_from_a_written_down_seed(*b"a stranger, also written down___");
        let secret = crate::secret::for_tests_only(stranger.secret);
        let what = what_to_release(&from);
        // Signed by the stranger, offered against the real root.
        assert!(matches!(
            make_a_release(&what, &secret, &key.public, TheDesktopUpdate::NotWritten, &into),
            Err(WhyTheReleaseWasNotMade::ItDidNotVerify { .. })
        ));
        assert!(!into.exists(), "a release that did not verify wrote a folder");
        let _ = std::fs::remove_dir_all(&from);
    }

    #[test]
    fn a_folder_that_already_holds_something_is_refused() {
        let from = a_folder("busy-from");
        let into = a_folder("busy-into");
        std::fs::create_dir_all(&into).unwrap();
        std::fs::write(into.join("something"), b"already here").unwrap();
        let key = a_key();
        let secret = crate::secret::for_tests_only(key.secret);
        let what = what_to_release(&from);
        assert!(matches!(
            make_a_release(&what, &secret, &key.public, TheDesktopUpdate::NotWritten, &into),
            Err(WhyTheReleaseWasNotMade::TheFolderIsNotEmptyOrCannotBeMade { .. })
        ));
        let _ = std::fs::remove_dir_all(&from);
        let _ = std::fs::remove_dir_all(&into);
    }

    #[test]
    fn the_key_set_statement_allows_the_key_it_was_given_and_nothing_else() {
        let key = a_key();
        let statement = compose_the_key_set_statement(1, key.public, "the one key");
        assert_eq!(statement.generation, 1);
        assert_eq!(statement.allowed.len(), 1);
        assert_eq!(statement.allowed[0].public_key, key.public);
        assert!(statement.revocations.is_empty());
        // It may authorize from the very first release.
        assert_eq!(statement.keys_that_may_authorize(0).len(), 1);
    }

    // ------------------------------------------- the desktop update file

    use crate::the_desktop_update::tests::{a_build_carrying_the_keys, the_everyday_key};
    use crate::the_desktop_update::THE_EARLIER_TRUST_STATEMENTS;
    use smoozy_desktop::updating::manifest::THE_ROWS;
    use smoozy_desktop::updating::verifying::verify_an_update_file_against;

    /// One desktop artifact per row, each published under its own name.
    fn every_row_published(what: &mut WhatToRelease, from: &std::path::Path) {
        what.the_desktop_artifacts.clear();
        what.the_rows.clear();
        for row in THE_ROWS {
            let name = format!("Smoozy-{row}.artifact");
            what.the_desktop_artifacts.push(an_artifact(
                from,
                &name,
                format!("the desktop for {row}").as_bytes(),
            ));
            what.the_rows.push((row.to_owned(), name));
        }
    }

    /// **The whole act with the update file**: it is written into the release
    /// under its own name, and the desktop's own reading believes the written
    /// bytes for every row, each entry the written artifact's own.
    #[test]
    fn a_release_writes_the_desktop_update_file_and_the_desktop_believes_what_was_written() {
        let from = a_folder("update-from");
        let into = a_folder("update-into");
        let key = a_key();
        let secret = crate::secret::for_tests_only(key.secret);
        let mut what = what_to_release(&from);
        every_row_published(&mut what, &from);
        let carried = a_build_carrying_the_keys();
        let everyday = the_everyday_key();

        let made = make_a_release(
            &what,
            &secret,
            &key.public,
            TheDesktopUpdate::Written {
                carried: &carried,
                earlier: THE_EARLIER_TRUST_STATEMENTS,
                key: &everyday,
            },
            &into,
        )
        .unwrap();
        let offered = made.the_desktop_update.expect("the file was written");
        assert_eq!(offered.release_sequence, THE_NEXT_RELEASE_SEQUENCE);

        let written = std::fs::read(into.join(THE_UPDATE_FILES_NAME)).unwrap();
        for (row, name) in &what.the_rows {
            let verified = verify_an_update_file_against(&carried, None, &written, row).unwrap();
            assert_eq!(verified.manifest.version, "0.1.0");
            let entry = verified.the_entry_for_this_row.unwrap();
            assert_eq!(&entry.file_name, name);
            let bytes = std::fs::read(into.join(name)).unwrap();
            assert_eq!(entry.byte_length, bytes.len() as u64);
            assert_eq!(entry.digest, Digest::of(&bytes).bytes());
        }
        // **The server package's release is untouched by it** and still
        // verifies its own way.
        let release = std::fs::read(into.join(THE_RELEASE_FILES_NAME)).unwrap();
        assert!(verify_before_download(
            &key.public,
            None,
            &AnOfferedRelease::read(&release).unwrap(),
            &the_tuple_this_build_is_made_for(),
            None,
        )
        .is_ok());
        let listing = std::fs::read_to_string(into.join(THE_LISTINGS_NAME)).unwrap();
        assert!(listing.contains(THE_UPDATE_FILES_NAME));
        assert!(listing.contains("everyday-1"));

        let _ = std::fs::remove_dir_all(&from);
        let _ = std::fs::remove_dir_all(&into);
    }

    /// **A build with no master key writes no update file**, and the listing
    /// says so.
    #[test]
    fn a_build_with_no_master_key_writes_no_update_file_and_says_so() {
        let from = a_folder("no-update-from");
        let into = a_folder("no-update-into");
        let key = a_key();
        let secret = crate::secret::for_tests_only(key.secret);
        let what = what_to_release(&from);
        let made = make_a_release(&what, &secret, &key.public, TheDesktopUpdate::NotWritten, &into)
            .unwrap();
        assert!(made.the_desktop_update.is_none());
        assert!(!into.join(THE_UPDATE_FILES_NAME).exists());
        let listing = std::fs::read_to_string(into.join(THE_LISTINGS_NAME)).unwrap();
        assert!(listing.contains("carries no desktop update file"));
        let _ = std::fs::remove_dir_all(&from);
        let _ = std::fs::remove_dir_all(&into);
    }

    /// **An update file the desktop would refuse stops the whole release**, and
    /// nothing at all is written: a wrong everyday key, a row left out, and a
    /// row naming a file the release does not publish.
    #[test]
    fn an_update_file_the_desktop_would_refuse_writes_nothing() {
        let from = a_folder("refused-update-from");
        let into = a_folder("refused-update-into");
        let key = a_key();
        let secret = crate::secret::for_tests_only(key.secret);
        let carried = a_build_carrying_the_keys();
        let mut what = what_to_release(&from);
        every_row_published(&mut what, &from);

        let wrong = crate::secret::update_key_for_tests_only(
            *b"not the everyday key the build n",
            "everyday-1",
        );
        let mut missing = what_to_release(&from);
        every_row_published(&mut missing, &from);
        missing.the_rows.pop();
        let mut unpublished = what_to_release(&from);
        every_row_published(&mut unpublished, &from);
        unpublished.the_rows[0].1 = "a file nobody gave".into();

        let everyday = the_everyday_key();
        for (what, signing) in [(&what, &wrong), (&missing, &everyday), (&unpublished, &everyday)] {
            let why = match make_a_release(
                what,
                &secret,
                &key.public,
                TheDesktopUpdate::Written {
                    carried: &carried,
                    earlier: THE_EARLIER_TRUST_STATEMENTS,
                    key: signing,
                },
                &into,
            ) {
                Err(why) => why,
                Ok(_) => panic!("a release carrying a refused update file was made"),
            };
            assert!(
                matches!(why, WhyTheReleaseWasNotMade::TheDesktopUpdateFileWasNotMade { .. }),
                "{why:?}"
            );
            assert!(why.to_string().contains("nothing was written"));
            assert!(!into.exists(), "a refused release wrote a folder");
        }
        let _ = std::fs::remove_dir_all(&from);
    }
}
