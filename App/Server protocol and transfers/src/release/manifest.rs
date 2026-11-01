//! B8 — what a signed release manifest carries, and the exact match an
//! artifact is selected on.
//!
//! **A server package is selected from a signed release manifest, not from a
//! filename or a version endpoint.** Everything below is B8's own field list,
//! written out, plus the reading and writing of the canonical bytes those
//! fields are carried in.
//!
//! **The set of tuples the product supports is not decided here and not
//! Build's.** B8 says outright that it is the Creator-ruling boundary outside
//! this document. What this file holds is the shape of a tuple and the rule
//! that a match is exact; which tuples a manifest names is whoever composes
//! the manifest's, and the one this build makes is
//! [`composing::THE_TUPLE_THIS_BUILD_IS_MADE_FOR`](super::composing::THE_TUPLE_THIS_BUILD_IS_MADE_FOR).
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Server Setup` M1a. `State, identity and
//! recovery` §1.3 rule 6.
//!
//! **How a clause marker is written in this crate, and it is a rule rather
//! than a habit.** A marker with no document beside it means a clause of this
//! crate's own plan, `Preparation/Architecture/Server protocol and
//! transfers.md`. Every other document's marker is written with that
//! document — at the site, or in the file's own header list above.

use super::{Canonical, Reading};

/// The manifest format this build composes and reads.
///
/// **B8's first field.** A manifest naming anything else is not read: the
/// fields after the format are only meaningful under a format that names them.
pub const THE_MANIFEST_FORMAT: &str = "smoozy-server-package-manifest-1";

/// **B8's target tuple: the operating-system family, the processor
/// architecture, and the ABI and capabilities.**
///
/// **All three are compared whole and none of them is parsed.** A tuple is a
/// value `Server Setup` reads off a rented machine and a value a manifest
/// carries, and the only question ever asked of the pair is whether they are
/// the same value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetTuple {
    /// `Server Setup` M1a limits the supported families to Ubuntu and Debian,
    /// and `State, identity and recovery` §1.3 rule 6 makes every rented
    /// machine a Linux machine.
    pub operating_system_family: String,
    pub processor_architecture: String,
    /// The C library generation and whatever else a build has to match.
    pub abi_and_capabilities: String,
}

/// **B8's per-artifact entry, every field of it.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactEntry {
    /// **Stable.** It names this artifact across releases and is what the
    /// package-phase record carries under B10.
    pub artifact_identity: String,
    pub tuple: TargetTuple,
    pub byte_length: u64,
    /// SHA-256, as S23 names it, so one reading serves both purposes.
    pub digest: [u8; 32],
    /// **The fixed entry-point name** — the constant, installation-owned
    /// command string §1.1 sends after hand-over.
    pub entry_point_name: String,
    /// **The exact HTTPS source or sources.** A download from anywhere else is
    /// not a download of this artifact, whatever its bytes turn out to be.
    pub sources: Vec<String>,
}

/// **B8's manifest, every field of it.**
///
/// The parsed form of a run of canonical bytes. **Nothing composes these bytes
/// from this value at verification time** — the bytes are what arrived and what
/// was signed, and this is what they say.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseManifest {
    pub manifest_format: String,
    /// **Monotonically increasing**, and the value B9's non-rollback memory is
    /// kept against.
    pub release_sequence: u64,
    pub package_version: String,
    pub protocol_version_current: u32,
    /// **The one immediately previous supported version where one exists**, and
    /// absent rather than zero where none does.
    pub protocol_version_previous_supported: Option<u32>,
    pub artifacts: Vec<ArtifactEntry>,
}

/// **Why a manifest's bytes were not read at all.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyTheBytesWereNotRead {
    /// A field ran off the end, a length did not fit, or bytes were left over.
    TheyAreNotAManifest,
    /// The bytes read, and the format they name is not one this build knows.
    ItIsNotAFormatThisBuildKnows,
}

impl ReleaseManifest {
    /// Read a manifest from the exact bytes that were signed.
    pub fn read(bytes: &[u8]) -> Result<ReleaseManifest, WhyTheBytesWereNotRead> {
        let manifest =
            read_fields(bytes).ok_or(WhyTheBytesWereNotRead::TheyAreNotAManifest)?;
        if manifest.manifest_format != THE_MANIFEST_FORMAT {
            return Err(WhyTheBytesWereNotRead::ItIsNotAFormatThisBuildKnows);
        }
        Ok(manifest)
    }

    /// The canonical bytes of this manifest.
    ///
    /// **This is the release side's function and never the verifier's.** A
    /// verifier that re-composed the bytes it was about to check a signature
    /// over would be checking a signature over its own encoding rather than
    /// over what arrived.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut out = Canonical::new();
        out.text(&self.manifest_format)
            .u64(self.release_sequence)
            .text(&self.package_version)
            .u32(self.protocol_version_current)
            .whether(self.protocol_version_previous_supported.is_some());
        if let Some(previous) = self.protocol_version_previous_supported {
            out.u32(previous);
        }
        out.u32(self.artifacts.len() as u32);
        for artifact in &self.artifacts {
            out.text(&artifact.artifact_identity)
                .text(&artifact.tuple.operating_system_family)
                .text(&artifact.tuple.processor_architecture)
                .text(&artifact.tuple.abi_and_capabilities)
                .u64(artifact.byte_length)
                .digest(&artifact.digest)
                .text(&artifact.entry_point_name)
                .u32(artifact.sources.len() as u32);
            for source in &artifact.sources {
                out.text(source);
            }
        }
        out.finish()
    }
}

fn read_fields(bytes: &[u8]) -> Option<ReleaseManifest> {
    let mut reading = Reading::of(bytes);
    let manifest_format = reading.text()?;
    let release_sequence = reading.u64()?;
    let package_version = reading.text()?;
    let protocol_version_current = reading.u32()?;
    let protocol_version_previous_supported = if reading.whether()? {
        Some(reading.u32()?)
    } else {
        None
    };
    let how_many = reading.u32()?;
    let mut artifacts = Vec::new();
    for _ in 0..how_many {
        artifacts.push(ArtifactEntry {
            artifact_identity: reading.text()?,
            tuple: TargetTuple {
                operating_system_family: reading.text()?,
                processor_architecture: reading.text()?,
                abi_and_capabilities: reading.text()?,
            },
            byte_length: reading.u64()?,
            digest: reading.digest()?,
            entry_point_name: reading.text()?,
            sources: {
                let sources = reading.u32()?;
                let mut out = Vec::new();
                for _ in 0..sources {
                    out.push(reading.text()?);
                }
                out
            },
        });
    }
    reading.end()?;
    Some(ReleaseManifest {
        manifest_format,
        release_sequence,
        package_version,
        protocol_version_current,
        protocol_version_previous_supported,
        artifacts,
    })
}

/// **What selection answered.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhySelectionFailed {
    /// **No matching entry is an unsupported machine, reported before any
    /// package byte is executed.** It is not a failure of the manifest and not
    /// a fault of the machine; it is the product's own boundary meeting a
    /// machine outside it.
    ThisMachineIsNotSupported,
    /// Two entries claim the same tuple. **Which one a manifest meant cannot be
    /// answered, so neither is taken.** B8 gives no rule for this because a
    /// manifest is composed rather than found; refusing it is Build's, and the
    /// alternative — taking the first — is one of the six selections B8's
    /// neighbours refuse by name.
    TheManifestNamesThatTupleTwice,
}

/// **B8's selection: an artifact is selected only on an exact manifest
/// match.**
///
/// `Server Setup` supplies the detected rental tuple. **No closest
/// architecture, no closest ABI, no capability fallback, no generic-Linux
/// entry, no package-name guess and no first entry** — and none of those is
/// refused by a check here, because none of them is reachable: the only
/// comparison this function makes is whole-value equality, and a tuple that is
/// not equal is not a candidate.
pub fn select<'a>(
    manifest: &'a ReleaseManifest,
    detected: &TargetTuple,
) -> Result<&'a ArtifactEntry, WhySelectionFailed> {
    let mut matching = manifest
        .artifacts
        .iter()
        .filter(|artifact| &artifact.tuple == detected);
    let Some(only) = matching.next() else {
        return Err(WhySelectionFailed::ThisMachineIsNotSupported);
    };
    if matching.next().is_some() {
        return Err(WhySelectionFailed::TheManifestNamesThatTupleTwice);
    }
    Ok(only)
}

#[cfg(test)]
mod tests {
    use super::*;

    pub(super) fn a_tuple() -> TargetTuple {
        TargetTuple {
            operating_system_family: "linux".into(),
            processor_architecture: "x86_64".into(),
            abi_and_capabilities: "gnu".into(),
        }
    }

    fn an_artifact(identity: &str, tuple: TargetTuple) -> ArtifactEntry {
        ArtifactEntry {
            artifact_identity: identity.into(),
            tuple,
            byte_length: 4096,
            digest: [7; 32],
            entry_point_name: "smoozy-server".into(),
            sources: vec!["https://example.invalid/a".into()],
        }
    }

    fn a_manifest() -> ReleaseManifest {
        ReleaseManifest {
            manifest_format: THE_MANIFEST_FORMAT.into(),
            release_sequence: 12,
            package_version: "0.1.0".into(),
            protocol_version_current: 2,
            protocol_version_previous_supported: Some(1),
            artifacts: vec![an_artifact("linux-x86_64-gnu", a_tuple())],
        }
    }

    #[test]
    fn a_manifest_reads_back_as_what_was_composed() {
        let manifest = a_manifest();
        let bytes = manifest.canonical_bytes();
        assert_eq!(ReleaseManifest::read(&bytes).unwrap(), manifest);
    }

    #[test]
    fn no_previous_protocol_version_is_absent_rather_than_zero() {
        let mut manifest = a_manifest();
        manifest.protocol_version_previous_supported = None;
        let bytes = manifest.canonical_bytes();
        let read = ReleaseManifest::read(&bytes).unwrap();
        assert!(read.protocol_version_previous_supported.is_none());
        // And a manifest naming version 0 as its previous one is a different
        // document from a manifest naming none.
        let mut names_zero = manifest.clone();
        names_zero.protocol_version_previous_supported = Some(0);
        assert_ne!(names_zero.canonical_bytes(), bytes);
    }

    #[test]
    fn a_format_this_build_does_not_know_is_refused_rather_than_read() {
        let mut manifest = a_manifest();
        manifest.manifest_format = "something-else".into();
        assert_eq!(
            ReleaseManifest::read(&manifest.canonical_bytes()),
            Err(WhyTheBytesWereNotRead::ItIsNotAFormatThisBuildKnows)
        );
    }

    #[test]
    fn bytes_that_are_not_a_manifest_are_refused_rather_than_half_read() {
        assert_eq!(
            ReleaseManifest::read(&[]),
            Err(WhyTheBytesWereNotRead::TheyAreNotAManifest)
        );
        let bytes = a_manifest().canonical_bytes();
        assert_eq!(
            ReleaseManifest::read(&bytes[..bytes.len() - 1]),
            Err(WhyTheBytesWereNotRead::TheyAreNotAManifest)
        );
        let mut with_a_tail = bytes.clone();
        with_a_tail.push(0);
        assert_eq!(
            ReleaseManifest::read(&with_a_tail),
            Err(WhyTheBytesWereNotRead::TheyAreNotAManifest)
        );
    }

    #[test]
    fn selection_is_exact_and_a_near_miss_is_an_unsupported_machine() {
        let manifest = a_manifest();
        assert_eq!(
            select(&manifest, &a_tuple()).unwrap().artifact_identity,
            "linux-x86_64-gnu"
        );
        for wrong in [
            TargetTuple {
                processor_architecture: "x86-64".into(),
                ..a_tuple()
            },
            TargetTuple {
                abi_and_capabilities: "musl".into(),
                ..a_tuple()
            },
            TargetTuple {
                operating_system_family: "Linux".into(),
                ..a_tuple()
            },
        ] {
            assert_eq!(
                select(&manifest, &wrong),
                Err(WhySelectionFailed::ThisMachineIsNotSupported),
                "{wrong:?} was matched"
            );
        }
    }

    #[test]
    fn a_manifest_naming_one_tuple_twice_takes_neither_entry() {
        let mut manifest = a_manifest();
        manifest
            .artifacts
            .push(an_artifact("a second one", a_tuple()));
        assert_eq!(
            select(&manifest, &a_tuple()),
            Err(WhySelectionFailed::TheManifestNamesThatTupleTwice)
        );
    }

    #[test]
    fn a_manifest_with_no_artifacts_is_a_manifest_and_matches_nothing() {
        let mut manifest = a_manifest();
        manifest.artifacts.clear();
        let bytes = manifest.canonical_bytes();
        assert_eq!(ReleaseManifest::read(&bytes).unwrap(), manifest);
        assert_eq!(
            select(&manifest, &a_tuple()),
            Err(WhySelectionFailed::ThisMachineIsNotSupported)
        );
    }
}
