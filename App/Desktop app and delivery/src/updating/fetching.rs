//! **The fetch: the artifact a verified manifest names, brought to this
//! computer and held to its length and digest** — E15, E16's digest and length,
//! E18, Z22 and R19. Built at step 11's Mac update piece. It runs after a check
//! that found a newer version, on the check's own sequence, and before anything
//! is unpacked or offered to the system's own check.
//!
//! **Where the artifact is read from — step 11's release piece's rule,
//! followed.** That piece writes every row's artifact into the same release as
//! the update file and reads each one back from **beside the update file**: the
//! update file's address with the artifact's own name in place of its last part
//! (`App/Release/src/outside.rs`, `where_an_artifact_it_names_is`). It asked
//! this piece to follow or overrule that; **this piece follows it**, so the
//! release checks exactly the address a copy reads, and there is one rule and
//! not two. [`where_the_artifact_is`] is that rule. **The name carries no
//! trust** — the length and digest the signed manifest names are what say the
//! bytes are right — but a name that is not a plain file name is refused rather
//! than put into an address, so no name can point the read anywhere but beside
//! the update file.
//!
//! **What the fetch is held to, in order, and each is a gate on the next**:
//!
//! 1. **Admitted** under the update fetch's class at the one desktop point
//!    (Z22, R19), asked again after each refusal for as long as it takes, as the
//!    check is.
//! 2. **Read** over the same one HTTPS client and saying the same two things
//!    about who is asking (E11) — [`super::checking::OverHttps`], the same seam
//!    — **with every byte drawn from the download budget** (Z22), and **no
//!    more read than the manifest's length and one byte beyond it**, so a body
//!    longer than named is found as one rather than read for ever.
//! 3. **The channel ended** before anything is checked (R5).
//! 4. **The length, then the digest**, against the signed manifest's entry
//!    (E16: *"the digest and length are checked against the signed manifest
//!    before platform verification"*) — the reading half's own check,
//!    [`super::verifying::the_artifact_is_the_one_named`], not a second one.
//! 5. **Written through the platform file adapter** into the update's staging
//!    folder, on the same filesystem as the bundle it will replace (E18), and
//!    re-read by path (F1, Z24).
//!
//! **A fetch that does not pass is an update that did not happen** (E15): no
//! partial artifact is offered on, nothing is retried into the same place, and
//! nothing skips a step.
//!
//! **No network in tests**: the read is the same seam the check reads through,
//! and the tests stand bytes there.
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this
//! crate's own plan, `Preparation/Architecture/Desktop app and delivery.md`.
//! Every other document's marker is written with that document at the site.

use std::path::{Path, PathBuf};
use std::time::Instant;

use smoozy_server_protocol::transfers::integrity::Digest;
use smoozy_server_protocol::transfers::moving::THE_ACCOUNTING_PERIOD;

use super::checking::{AdmitsTheUpdateFetch, ReadsTheUpdateFile, WhatTheRequestSays, WHERE_THE_UPDATE_FILE_IS_READ_FROM};
use super::manifest::AnEntry;
use super::verifying::{the_artifact_is_the_one_named, WhyTheArtifactIsNotTheOneNamed};
use crate::platform::files;

/// **Where an artifact the manifest names is read from** — beside the update
/// file, under the name the manifest gives it (step 11's release piece's rule,
/// followed). **Nothing where the name is not a plain file name**: letters,
/// digits, `.`, `-` and `_`, not beginning with a dot.
pub fn where_the_artifact_is(file_name: &str) -> Option<String> {
    let plain = !file_name.is_empty()
        && !file_name.starts_with('.')
        && file_name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-' | b'_'));
    if !plain {
        return None;
    }
    let (folder, _) = WHERE_THE_UPDATE_FILE_IS_READ_FROM.rsplit_once('/')?;
    Some(format!("{folder}/{file_name}"))
}

/// **Why a fetch did not pass.** Each is an update that did not happen (E15).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhyTheFetchDidNotPass {
    /// The manifest's name for the artifact is not a plain file name.
    ItsNameIsNotAPlainFileName,
    /// Nothing came back, in the network's own words.
    CouldNotBeReached(String),
    /// **The length or the digest is not the manifest's** (E16).
    ItIsNotTheArtifactNamed(WhyTheArtifactIsNotTheOneNamed),
    /// The bytes could not be written where they go, in the adapter's words.
    CouldNotBeStaged(String),
}

/// **A fetched artifact: its bytes on the disk, held to the manifest's entry.**
/// Only [`the_artifact_is_fetched`] makes one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AFetchedArtifact {
    path: PathBuf,
    entry: AnEntry,
}

impl AFetchedArtifact {
    /// Where the fetched bytes are.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// The manifest's entry they were held to.
    pub fn entry(&self) -> &AnEntry {
        &self.entry
    }
}

/// **The fetch, whole**, into `the_staging_folder`, which must already be
/// there. See the head of this file for each step and the order.
pub fn the_artifact_is_fetched(
    file: &dyn ReadsTheUpdateFile,
    admission: &dyn AdmitsTheUpdateFetch,
    entry: &AnEntry,
    says: &WhatTheRequestSays,
    the_staging_folder: &Path,
) -> Result<AFetchedArtifact, WhyTheFetchDidNotPass> {
    let Some(from) = where_the_artifact_is(&entry.file_name) else {
        return Err(WhyTheFetchDidNotPass::ItsNameIsNotAPlainFileName);
    };
    the_artifact_is_fetched_from(file, admission, entry, says, the_staging_folder, &from)
}

/// Same admitted, budgeted and verified fetch, from a named release.
/// Linux uses this only for the exact running release's rollback package.
pub(crate) fn the_artifact_is_fetched_from(
    file: &dyn ReadsTheUpdateFile,
    admission: &dyn AdmitsTheUpdateFetch,
    entry: &AnEntry,
    says: &WhatTheRequestSays,
    the_staging_folder: &Path,
    from: &str,
) -> Result<AFetchedArtifact, WhyTheFetchDidNotPass> {
        if where_the_artifact_is(&entry.file_name).is_none() {
        return Err(WhyTheFetchDidNotPass::ItsNameIsNotAPlainFileName);
    }
    let bytes = bytes_are_fetched(
        file,
        admission,
        from, says, entry.byte_length.saturating_add(1),
    )?;
    the_artifact_is_the_one_named(entry, bytes.len() as u64, &Digest::of(&bytes))
        .map_err(WhyTheFetchDidNotPass::ItIsNotTheArtifactNamed)?;
    let path = the_staging_folder.join(&entry.file_name);
    let written = files::write_whole_file(&path, &bytes, files::Durability::ToTheDevice);
    if !written.wrote_the_file() {
        return Err(WhyTheFetchDidNotPass::CouldNotBeStaged(match written.first_error {
            Some(failure) => failure.reported,
            None => format!("{:?}", written.recheck),
        }));
    }
    Ok(AFetchedArtifact {
        path,
        entry: entry.clone(),
    })
}

/// Shared read for the signed own-release file as well as artifact bodies.
pub(crate) fn bytes_are_fetched(
    file: &dyn ReadsTheUpdateFile,
    admission: &dyn AdmitsTheUpdateFetch,
    from: &str,
    says: &WhatTheRequestSays,
    most: u64,
) -> Result<Vec<u8>, WhyTheFetchDidNotPass> {
    let mut draw = loop {
        if let Some(draw) = admission.admit_the_update_fetch() {
            break draw;
        }
        // A delay inside a running fetch (P10a), one accounting period.
        std::thread::sleep(THE_ACCOUNTING_PERIOD);
    };
    let read = file.read(from, says, most, &mut draw);
    // **R5: the share returns at once**, before anything is checked.
    let _ = draw.the_channel_ended(Instant::now());
    read.map_err(WhyTheFetchDidNotPass::CouldNotBeReached)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::downloading::TheRunsTransfers;
    use crate::updating::manifest::THE_MAC_ROW;
    use smoozy_server_protocol::transfers::moving::DrawsFromTheSharedBudget;
    use std::sync::Mutex;

    /// **The artifact at the seam**: what it serves, and what it was asked.
    pub(crate) struct AnArtifactAtTheSeam {
        pub serves: Option<Vec<u8>>,
        pub asked: Mutex<Vec<(String, u64)>>,
    }

    impl AnArtifactAtTheSeam {
        pub(crate) fn serving(serves: Option<Vec<u8>>) -> AnArtifactAtTheSeam {
            AnArtifactAtTheSeam {
                serves,
                asked: Mutex::new(Vec::new()),
            }
        }
    }

    impl ReadsTheUpdateFile for AnArtifactAtTheSeam {
        fn read(
            &self,
            from: &str,
            _says: &WhatTheRequestSays,
            the_most: u64,
            draw: &mut dyn DrawsFromTheSharedBudget,
        ) -> Result<Vec<u8>, String> {
            self.asked.lock().unwrap().push((from.to_string(), the_most));
            let Some(bytes) = self.serves.clone() else {
                return Err("nothing answered".into());
            };
            let mut reading: &[u8] = &bytes;
            crate::updating::checking::read_drawing_from_the_budget(&mut reading, the_most, draw)
        }
    }

    pub(crate) fn an_entry_for(bytes: &[u8]) -> AnEntry {
        AnEntry {
            row: THE_MAC_ROW.to_string(),
            file_name: "Smoozy-macos.zip".to_string(),
            byte_length: bytes.len() as u64,
            digest: Digest::of(bytes).bytes(),
        }
    }

    fn the_mac_asks() -> WhatTheRequestSays {
        WhatTheRequestSays {
            version: "0.1.8".into(),
            system: THE_MAC_ROW.into(),
        }
    }

    /// **The artifact is read from beside the update file**, the rule step
    /// 11's release piece checks every row by — and a name that is not a plain
    /// file name is put into no address.
    #[test]
    fn the_artifact_is_read_from_beside_the_update_file() {
        assert_eq!(
            where_the_artifact_is("Smoozy-macos.zip").as_deref(),
            Some("https://github.com/a-kislyakov/Smoozy-releases/releases/latest/download/Smoozy-macos.zip")
        );
        for refused in ["", ".hidden", "../release", "a/b.zip", "a b.zip", "a?b", "a%2Fb", "a#b"] {
            assert_eq!(where_the_artifact_is(refused), None, "{refused:?}");
        }
    }

    /// **A fetched file passes only at the manifest's length and digest**, is
    /// written into the staging folder whole, and reads no more than one byte
    /// past the length named; every byte is drawn from the budget under the
    /// update fetch's class.
    #[test]
    fn a_fetched_artifact_is_held_to_its_length_and_digest() {
        let folder = crate::scratch_folders::a_folder(format!("smoozy-fetch-{}", std::process::id()));
        let transfers = TheRunsTransfers::for_this_run();
        let bytes = b"the complete Mac download".to_vec();
        let entry = an_entry_for(&bytes);

        let seam = AnArtifactAtTheSeam::serving(Some(bytes.clone()));
        let fetched = the_artifact_is_fetched(&seam, &transfers, &entry, &the_mac_asks(), &folder).expect("it passes");
        assert_eq!(std::fs::read(fetched.path()).unwrap(), bytes);
        assert_eq!(fetched.path(), folder.join("Smoozy-macos.zip"));
        assert_eq!(fetched.entry(), &entry);
        assert_eq!(
            seam.asked.lock().unwrap().clone(),
            vec![(where_the_artifact_is("Smoozy-macos.zip").unwrap(), bytes.len() as u64 + 1)]
        );
        assert_eq!(transfers.grants_made(smoozy_server_protocol::transfers::Direction::Down), 1);

        // **Refused for size**: one byte more than named, and one fewer.
        let _ = std::fs::remove_file(folder.join("Smoozy-macos.zip"));
        for wrong in [[bytes.clone(), b"!".to_vec()].concat(), bytes[..bytes.len() - 1].to_vec()] {
            let seam = AnArtifactAtTheSeam::serving(Some(wrong));
            assert!(matches!(
                the_artifact_is_fetched(&seam, &transfers, &entry, &the_mac_asks(), &folder),
                Err(WhyTheFetchDidNotPass::ItIsNotTheArtifactNamed(
                    WhyTheArtifactIsNotTheOneNamed::ItsLengthIsNotTheManifests { .. }
                ))
            ));
        }
        // **Refused for its fingerprint**: the right length, one byte changed.
        let mut changed = bytes.clone();
        changed[0] ^= 1;
        let seam = AnArtifactAtTheSeam::serving(Some(changed));
        assert_eq!(
            the_artifact_is_fetched(&seam, &transfers, &entry, &the_mac_asks(), &folder),
            Err(WhyTheFetchDidNotPass::ItIsNotTheArtifactNamed(
                WhyTheArtifactIsNotTheOneNamed::ItsDigestIsNotTheManifests
            ))
        );
        // Nothing reached, and a name that is no file name.
        let seam = AnArtifactAtTheSeam::serving(None);
        assert!(matches!(
            the_artifact_is_fetched(&seam, &transfers, &entry, &the_mac_asks(), &folder),
            Err(WhyTheFetchDidNotPass::CouldNotBeReached(_))
        ));
        let odd = AnEntry {
            file_name: "../release".into(),
            ..entry.clone()
        };
        assert_eq!(
            the_artifact_is_fetched(&seam, &transfers, &odd, &the_mac_asks(), &folder),
            Err(WhyTheFetchDidNotPass::ItsNameIsNotAPlainFileName)
        );
        assert!(
            !folder.join("Smoozy-macos.zip").exists(),
            "a refused fetch left its bytes where a passed one goes"
        );
        let _ = std::fs::remove_dir_all(&folder);
    }
}
