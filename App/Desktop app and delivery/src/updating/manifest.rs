//! **E15's signed manifest, every field it names, and the entry for one row.**
//!
//! E15: *"The signed manifest carries a monotonically increasing unsigned
//! 64-bit `release_sequence`, the human version, the exact artifact digest,
//! platform/package row, artifact length, and signing-key identifier."* Each
//! of those is a field below. **Two things are Build's own**, and each is said
//! where it is:
//!
//! - **one entry per row**, each with its own digest and length, because a
//!   release carries an artifact for every row of E4b and one manifest names
//!   them all, which is how step 11's release piece writes it;
//! - **the artifact's file name in the same release**, so the fetch is told
//!   by the signed document which file to take rather than composing a name.
//!   The name carries no trust: the length and the digest are what is checked.
//!
//! **The signing-key identifier is a list of one or two.** E19: *"A normal
//! transition manifest is signed by the current and next online keys"*, so a
//! manifest names every key whose signature travels with it, in the order the
//! signatures are carried. **The names are inside the signed bytes**, so which
//! key a signature claims to be is itself signed.
//!
//! **The distributed form is the signed form.** The manifest is carried as the
//! canonical bytes that were signed, and [`UpdateManifest::read`] derives the
//! value from those bytes. Nothing re-encodes a manifest before a signature is
//! checked over it.
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this
//! crate's own plan, `Preparation/Architecture/Desktop app and delivery.md`.
//! Every other document's marker is written with that document at the site.

use smoozy_server_protocol::release::{Canonical, Reading};

/// The manifest format this build composes and reads. A manifest naming
/// anything else is not read.
pub const THE_MANIFEST_FORMAT: &str = "smoozy-desktop-update-manifest-1";

/// **E4b's five rows, as a manifest names them.** The names are Build's; the
/// rows are E4b's. One spelling, here, for the release that writes them and
/// the copy that looks for its own.
pub const THE_MAC_ROW: &str = "macos-aarch64";
pub const THE_WINDOWS_ROW: &str = "windows-x86_64";
pub const THE_UBUNTU_24_04_ROW: &str = "ubuntu-24.04-amd64";
pub const THE_UBUNTU_26_04_ROW: &str = "ubuntu-26.04-amd64";
pub const THE_DEBIAN_13_ROW: &str = "debian-13-amd64";

/// Every row E4b rules, in its table's order.
pub const THE_ROWS: [&str; 5] = [
    THE_MAC_ROW,
    THE_WINDOWS_ROW,
    THE_UBUNTU_24_04_ROW,
    THE_UBUNTU_26_04_ROW,
    THE_DEBIAN_13_ROW,
];

/// **One row's artifact.** E15's row, length and digest, and the file it is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnEntry {
    /// The platform/package row, compared whole and never parsed.
    pub row: String,
    /// The artifact's file name in the same release. **Not trust**: see the
    /// file's head.
    pub file_name: String,
    pub byte_length: u64,
    /// SHA-256 of the artifact's bytes, the thirty-two bytes themselves.
    pub digest: [u8; 32],
}

/// **E15's manifest, every field of it.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateManifest {
    pub manifest_format: String,
    /// **Monotonically increasing, unsigned, 64 bits.** The value the stored
    /// high-water is kept against.
    pub release_sequence: u64,
    /// The human version, for reading and for nothing a decision rests on.
    pub version: String,
    /// The signing-key identifiers, one per signature carried with it.
    pub signing_keys: Vec<String>,
    pub entries: Vec<AnEntry>,
}

/// **Why a manifest's bytes were not read at all.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyTheBytesWereNotRead {
    /// A field ran off the end, a length did not fit, or bytes were left over.
    TheyAreNotAManifest,
    /// The bytes read, and the format they name is not one this build knows.
    ItIsNotAFormatThisBuildKnows,
    /// **It names one row twice, names no signing key, or names one twice.**
    /// Two entries for one row would make selection a choice, and a key named
    /// twice would let one signature be counted as two.
    ItIsAmbiguous,
}

impl UpdateManifest {
    /// Read a manifest from the exact bytes that were signed.
    pub fn read(bytes: &[u8]) -> Result<UpdateManifest, WhyTheBytesWereNotRead> {
        let manifest = read_fields(bytes).ok_or(WhyTheBytesWereNotRead::TheyAreNotAManifest)?;
        if manifest.manifest_format != THE_MANIFEST_FORMAT {
            return Err(WhyTheBytesWereNotRead::ItIsNotAFormatThisBuildKnows);
        }
        let mut rows = std::collections::HashSet::new();
        let mut keys = std::collections::HashSet::new();
        if manifest.signing_keys.is_empty()
            || !manifest
                .entries
                .iter()
                .all(|entry| rows.insert(entry.row.as_str()))
            || !manifest
                .signing_keys
                .iter()
                .all(|key| keys.insert(key.as_str()))
        {
            return Err(WhyTheBytesWereNotRead::ItIsAmbiguous);
        }
        Ok(manifest)
    }

    /// The canonical bytes of this manifest. **The release side's function**
    /// (step 11's release piece) and the tests'; a verifier never re-composes what it is
    /// about to check a signature over.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut out = Canonical::new();
        out.text(&self.manifest_format)
            .u64(self.release_sequence)
            .text(&self.version)
            .u32(self.signing_keys.len() as u32);
        for key in &self.signing_keys {
            out.text(key);
        }
        out.u32(self.entries.len() as u32);
        for entry in &self.entries {
            out.text(&entry.row)
                .text(&entry.file_name)
                .u64(entry.byte_length)
                .digest(&entry.digest);
        }
        out.finish()
    }

    /// **The entry for the row this copy runs on, or none.** Whole-value
    /// equality and nothing else; a manifest with no entry for a row offers that
    /// row nothing, which is not a failure of the manifest.
    pub fn the_entry_for(&self, row: &str) -> Option<&AnEntry> {
        self.entries.iter().find(|entry| entry.row == row)
    }
}

fn read_fields(bytes: &[u8]) -> Option<UpdateManifest> {
    let mut reading = Reading::of(bytes);
    let manifest_format = reading.text()?;
    let release_sequence = reading.u64()?;
    let version = reading.text()?;
    let how_many = reading.u32()?;
    let mut signing_keys = Vec::new();
    for _ in 0..how_many {
        signing_keys.push(reading.text()?);
    }
    let how_many = reading.u32()?;
    let mut entries = Vec::new();
    for _ in 0..how_many {
        entries.push(AnEntry {
            row: reading.text()?,
            file_name: reading.text()?,
            byte_length: reading.u64()?,
            digest: reading.digest()?,
        });
    }
    reading.end()?;
    Some(UpdateManifest {
        manifest_format,
        release_sequence,
        version,
        signing_keys,
        entries,
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) fn a_manifest(release_sequence: u64, signing_keys: &[&str]) -> UpdateManifest {
        UpdateManifest {
            manifest_format: THE_MANIFEST_FORMAT.into(),
            release_sequence,
            version: format!("0.2.{release_sequence}"),
            signing_keys: signing_keys.iter().map(|key| key.to_string()).collect(),
            entries: vec![
                AnEntry {
                    row: THE_MAC_ROW.into(),
                    file_name: "Smoozy.app.tar".into(),
                    byte_length: 7,
                    digest: smoozy_server_protocol::transfers::integrity::Digest::of(b"the mac")
                        .bytes(),
                },
                AnEntry {
                    row: THE_DEBIAN_13_ROW.into(),
                    file_name: "smoozy_amd64.deb".into(),
                    byte_length: 10,
                    digest: smoozy_server_protocol::transfers::integrity::Digest::of(b"the debian")
                        .bytes(),
                },
            ],
        }
    }

    #[test]
    fn every_field_e15_names_reads_back_as_it_was_composed() {
        let manifest = a_manifest(41, &["online-1", "online-2"]);
        let read = UpdateManifest::read(&manifest.canonical_bytes()).unwrap();
        assert_eq!(read, manifest);
        assert_eq!(read.release_sequence, 41);
        assert_eq!(read.version, "0.2.41");
        assert_eq!(read.signing_keys, ["online-1", "online-2"]);
        let mac = read.the_entry_for(THE_MAC_ROW).unwrap();
        assert_eq!(
            (mac.byte_length, mac.file_name.as_str()),
            (7, "Smoozy.app.tar")
        );
        // **Unsigned 64 bits, the whole range.**
        let highest = a_manifest(u64::MAX, &["online-1"]);
        assert_eq!(
            UpdateManifest::read(&highest.canonical_bytes())
                .unwrap()
                .release_sequence,
            u64::MAX
        );
    }

    #[test]
    fn a_row_with_no_entry_is_offered_nothing_and_selection_is_exact() {
        let manifest = a_manifest(1, &["online-1"]);
        assert!(manifest.the_entry_for(THE_WINDOWS_ROW).is_none());
        assert!(manifest.the_entry_for("macos-aarch6").is_none());
        assert!(manifest.the_entry_for("MACOS-AARCH64").is_none());
        assert_eq!(
            manifest
                .the_entry_for(THE_DEBIAN_13_ROW)
                .unwrap()
                .byte_length,
            10
        );
    }

    #[test]
    fn a_manifest_that_is_not_one_or_names_another_format_is_not_read() {
        let manifest = a_manifest(1, &["online-1"]);
        let mut bytes = manifest.canonical_bytes();
        bytes.push(0);
        assert_eq!(
            UpdateManifest::read(&bytes),
            Err(WhyTheBytesWereNotRead::TheyAreNotAManifest)
        );
        let other = UpdateManifest {
            manifest_format: "smoozy-server-package-manifest-1".into(),
            ..manifest
        };
        assert_eq!(
            UpdateManifest::read(&other.canonical_bytes()),
            Err(WhyTheBytesWereNotRead::ItIsNotAFormatThisBuildKnows)
        );
    }

    #[test]
    fn a_manifest_that_makes_selection_or_counting_a_choice_is_not_read() {
        let mut twice = a_manifest(1, &["online-1"]);
        twice.entries.push(twice.entries[0].clone());
        assert_eq!(
            UpdateManifest::read(&twice.canonical_bytes()),
            Err(WhyTheBytesWereNotRead::ItIsAmbiguous)
        );
        let no_key = a_manifest(1, &[]);
        assert_eq!(
            UpdateManifest::read(&no_key.canonical_bytes()),
            Err(WhyTheBytesWereNotRead::ItIsAmbiguous)
        );
        let one_key_twice = a_manifest(1, &["online-1", "online-1"]);
        assert_eq!(
            UpdateManifest::read(&one_key_twice.canonical_bytes()),
            Err(WhyTheBytesWereNotRead::ItIsAmbiguous)
        );
    }

    #[test]
    fn the_five_rows_are_distinct() {
        let distinct: std::collections::HashSet<_> = THE_ROWS.iter().collect();
        assert_eq!(distinct.len(), 5);
    }
}
