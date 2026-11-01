//! The four supported versions the contract is composed per, and L14's
//! version confirmation as a judgement.
//!
//! **The contract is composed per version** — L10's material names the
//! version it was composed for (`Job files and outputs` D7), the device
//! argument's spelling is declared per version (L24a), and the device-line
//! ask is the version's own switch (L26). So a version is a value of this
//! closed set, and a manifest naming a version outside it composes nothing:
//! there is no spelling table for a fifth version and no arm invents one.
//!
//! **A fifth, 5.3, from 2026-10-02**: the 5.3.0 Alpha Blender's build
//! service offers, and Smoozy installs, composed nothing until every name the
//! composition uses was read off it at the desk — on the Creator's own install,
//! `/Applications/Blender 5.3.0a.app`, which answered *5.3.0 Alpha*, build
//! hash `68609be8e23b`, built 2026-10-01 — and each came back the same as
//! 5.2.2's ([`crate::spellings::THE_5_3_LOOKUP`]). The pass before 0.1.9 on
//! Vast, new fault 1: every job on the alpha failed with *Smoozy cannot
//! compose a render for Blender 5.3.0*. **The type keeps its name**; it is the
//! closed set, and the count in the name is the one it was written with.
//!
//! **A sixth, 5.4, from 2026-10-18**: the 5.4.0 Alpha from Blender's build
//! service, downloaded for the reading and copied to `/Applications/Blender
//! 5.4a.app`, which answered *5.4.0 Alpha*, build hash `3f4216e1bad1`, built
//! 2026-10-18 — every name the composition, the launch material and the reading
//! script use was read off it at the desk beside the 5.3.0 Beta and each came
//! back the same ([`crate::spellings::THE_5_4_LOOKUP`]). The Creator's
//! thirteenth notes, note 8: a 5.2.2 file on the 5.4 alpha went straight to
//! failed. **A version past 5.4 still composes nothing**, and says so in the
//! artist's words ([`crate::composition::CannotComposeTheAttempt::in_words`]).
//!
//! **The four are the whole-app floor and ceiling** — `Render Settings` B7's
//! floor of 4.5 and ceiling of 5.2, the floor being `Server Setup`'s own
//! constant — and the boundary the research draws is at 5: *one name in
//! 4.5, another from 5.0* (§9 item 2), so 5.1 sits inside the from-5.0
//! branch everywhere this crate branches, and that inference is recorded
//! at [`TheFourVersions::is_inferred_across_the_boundary`].
//!
//! **Markers used in this file that belong to another document, each with
//! the document it belongs to.** `Job files and outputs` D7, R1. `Render
//! Settings` B7. `Server Setup` B2.

use smoozy_server_setup_machine::blender::{Version, THE_FLOOR};

/// The closed set. **A version outside it is not a case with a default; it
/// is a manifest this contract refuses to compose for.**
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TheFourVersions {
    V4_5,
    V5_0,
    V5_1,
    V5_2,
    V5_3,
    V5_4,
}

impl TheFourVersions {
    pub const ALL: [TheFourVersions; 6] = [
        TheFourVersions::V4_5,
        TheFourVersions::V5_0,
        TheFourVersions::V5_1,
        TheFourVersions::V5_2,
        TheFourVersions::V5_3,
        TheFourVersions::V5_4,
    ];

    /// The version the manifest names, as one of the four, keyed on the
    /// major and minor components — patch releases share a line (`Server
    /// Setup` B2's discipline: components, never strings).
    pub fn of(version: Version) -> Option<TheFourVersions> {
        match (version.major, version.minor) {
            (4, 5) => Some(TheFourVersions::V4_5),
            (5, 0) => Some(TheFourVersions::V5_0),
            (5, 1) => Some(TheFourVersions::V5_1),
            (5, 2) => Some(TheFourVersions::V5_2),
            (5, 3) => Some(TheFourVersions::V5_3),
            (5, 4) => Some(TheFourVersions::V5_4),
            _ => None,
        }
    }

    /// **§9 item 2: Blender 5.1 was not exercised for the per-version
    /// property names or the reading**, so every version-boundary branch in
    /// this crate is inferred for it from the research's own boundary at 5.
    /// Recorded as a property of the value rather than as a note somebody
    /// has to remember beside each branch.
    pub fn is_inferred_across_the_boundary(self) -> bool {
        matches!(self, TheFourVersions::V5_1)
    }

    /// Whether the version is from 5.0 — the one boundary the research
    /// draws, and the one every branch here turns on.
    pub fn is_from_five(self) -> bool {
        !matches!(self, TheFourVersions::V4_5)
    }

    /// The version written out, for a diary line or a material's stamp.
    pub fn as_text(self) -> &'static str {
        match self {
            TheFourVersions::V4_5 => "4.5",
            TheFourVersions::V5_0 => "5.0",
            TheFourVersions::V5_1 => "5.1",
            TheFourVersions::V5_2 => "5.2",
            TheFourVersions::V5_3 => "5.3",
            TheFourVersions::V5_4 => "5.4",
        }
    }
}

/// **The floor is `Server Setup`'s own constant, cited rather than copied.**
pub const THE_FLOOR_IS_SERVER_SETUPS: Version = THE_FLOOR;

/// **L14: before the line is composed, the render worker confirms that the
/// Blender it is about to launch is the version the manifest names, and
/// where it is not, the attempt fails before the render instruction and no
/// launch happens** (`Job files and outputs` D7).
///
/// **The confirmation is never a read of a path, a folder name or an
/// installation record** — a directory's presence is evidence of nothing —
/// and the thing confirmed is the binary about to be launched. The ask
/// itself is the acting half's: the binary is asked to name its own version
/// through `Job files and outputs` R1's reading machinery where a reading
/// runs in this attempt, and by one short invocation asking for the version
/// alone where none does. **What is here is the judgement over the answer.**
///
/// **§9 item 16 — Fact 105 — is permanently unresolved and this function
/// leaves no space for it**: whether the reading's payloads carry the
/// running Blender's own version identifier is not established, so the
/// confirmation is written as its own invocation on every attempt, and
/// where a reading happens to carry the version the acting half hands it in
/// here exactly as it would hand in the short invocation's answer. The cost
/// is one short headless invocation per attempt, priced and declined at
/// §10.
pub fn confirm_the_version(
    the_manifest_names: Version,
    the_binary_answered: Option<Version>,
) -> Result<Version, VersionRefused> {
    match the_binary_answered {
        None => Err(VersionRefused::TheBinaryDidNotAnswer { the_manifest_names }),
        Some(answered) if answered == the_manifest_names => Ok(answered),
        Some(answered) => Err(VersionRefused::ADifferentVersionAnswered {
            the_manifest_names,
            answered,
        }),
    }
}

/// L14's refusal, with the reason recorded verbatim for K14's failed limb.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VersionRefused {
    /// The binary answered nothing — the confirmation cannot be made and
    /// the attempt fails; a silence is never read as a match.
    TheBinaryDidNotAnswer { the_manifest_names: Version },
    /// The binary named itself and the name is not the manifest's.
    ADifferentVersionAnswered {
        the_manifest_names: Version,
        answered: Version,
    },
}

/// **A version as the tests write one, for both folders of this module** —
/// `smoozy-rendering`'s warning tests build from it too. Behind the `testing`
/// switch for that crate's tests, as `material::testing` is. Moved
/// here from this file's tests on 2026-09-19, unchanged.
#[cfg(any(test, feature = "testing"))]
pub mod testing {
    use super::*;

    pub fn a_version(major: u32, minor: u32, patch: u32) -> Version {
        Version {
            major,
            minor,
            patch,
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    pub(crate) use super::testing::*;

    #[test]
    fn the_four_are_keyed_on_major_and_minor_and_a_fifth_is_not_a_case() {
        assert_eq!(
            TheFourVersions::of(a_version(4, 5, 12)),
            Some(TheFourVersions::V4_5)
        );
        assert_eq!(
            TheFourVersions::of(a_version(5, 0, 1)),
            Some(TheFourVersions::V5_0)
        );
        assert_eq!(
            TheFourVersions::of(a_version(5, 1, 2)),
            Some(TheFourVersions::V5_1)
        );
        assert_eq!(
            TheFourVersions::of(a_version(5, 2, 0)),
            Some(TheFourVersions::V5_2)
        );
        // **5.3 since 2026-10-02**, read off the 5.3.0 Alpha at the desk.
        assert_eq!(
            TheFourVersions::of(a_version(5, 3, 0)),
            Some(TheFourVersions::V5_3)
        );
        // **5.4 since 2026-10-18**, read off the 5.4.0 Alpha at the desk.
        assert_eq!(
            TheFourVersions::of(a_version(5, 4, 0)),
            Some(TheFourVersions::V5_4)
        );
        assert_eq!(TheFourVersions::of(a_version(4, 4, 0)), None);
        assert_eq!(TheFourVersions::of(a_version(5, 5, 0)), None);
        assert_eq!(TheFourVersions::of(a_version(6, 0, 0)), None);
    }

    #[test]
    fn the_boundary_is_at_five_and_five_one_is_the_inferred_one() {
        assert!(!TheFourVersions::V4_5.is_from_five());
        for version in [
            TheFourVersions::V5_0,
            TheFourVersions::V5_1,
            TheFourVersions::V5_2,
            TheFourVersions::V5_3,
            TheFourVersions::V5_4,
        ] {
            assert!(version.is_from_five());
        }
        assert!(!TheFourVersions::V5_3.is_inferred_across_the_boundary());
        assert!(!TheFourVersions::V5_4.is_inferred_across_the_boundary());
        assert!(TheFourVersions::V5_1.is_inferred_across_the_boundary());
        assert!(!TheFourVersions::V5_2.is_inferred_across_the_boundary());
    }

    #[test]
    fn the_floor_is_server_setups_own() {
        assert_eq!(THE_FLOOR_IS_SERVER_SETUPS, a_version(4, 5, 0));
        assert_eq!(
            TheFourVersions::of(THE_FLOOR_IS_SERVER_SETUPS),
            Some(TheFourVersions::V4_5)
        );
    }

    #[test]
    fn the_version_is_confirmed_on_all_three_components_and_a_silence_is_never_a_match() {
        let named = a_version(4, 5, 12);
        assert_eq!(confirm_the_version(named, Some(named)), Ok(named));
        // A patch apart is a different binary from the one the manifest
        // names (`Server Setup` B13: path beside version — and the version
        // itself compared whole).
        assert_eq!(
            confirm_the_version(named, Some(a_version(4, 5, 9))),
            Err(VersionRefused::ADifferentVersionAnswered {
                the_manifest_names: named,
                answered: a_version(4, 5, 9)
            })
        );
        assert_eq!(
            confirm_the_version(named, None),
            Err(VersionRefused::TheBinaryDidNotAnswer {
                the_manifest_names: named
            })
        );
    }
}
