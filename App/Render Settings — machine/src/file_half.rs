//! The file's half: the recorded reading as the desktop holds it, what
//! replaces it, what drops it, and what makes it stale — `Render Settings`
//! §1.2 Q6, Q17, Q18; §1.4 Q11–Q12; §1.5 Q13–Q15; §2.3 J10.
//!
//! **J2: the recorded reading on the desktop is a member of `State, identity
//! and recovery` §1.1's volatile class — a live reading taken from a machine
//! — and additionally period-held, dying with the connected period by J10.**
//! Nothing here is written to disk, and the type carries no written form.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Job files and outputs` R3, R10, R12, R12a,
//! R13, R16. `Server protocol and transfers` S13. `File Browser` X7a.
//! `Dependency Upload & Remap` V13. `Servers` H2, H2a. `Queue & History`
//! N16. `Desktop app and delivery` T19.

use std::collections::HashMap;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use smoozy_framing::source_reading::SourceReading;
use smoozy_server_setup_machine::blender::Version;
use smoozy_state::identities::Identity;
use smoozy_state::observed::{self, Clock};

use crate::reading::SettingsHalf;
use crate::readings::FourValuesNow;

/// One (server, file) — the key every recorded reading stands under. The file
/// is its path on that server.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FileKey {
    pub server: Identity,
    pub path: String,
}

/// **Q11: the identity of the Blender that took a reading — the build's path
/// and its answered version.** A version number does not identify a build
/// (`Server Setup` B13 by way of B3), which is why the path is beside it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlenderIdentity {
    pub path: String,
    pub version: Version,
}

/// A version written out, for a manifest field or a diary line. **A rendering
/// and never a comparison**: B2 compares components.
pub fn version_text(version: Version) -> String {
    format!("{}.{}.{}", version.major, version.minor, version.patch)
}

/// **Q11: every recorded reading carries the identity of the Blender that
/// took it beside the file's four-value identity and the moment.** The
/// reading and the version travel together, and `Job files and outputs` R13
/// already makes the version one of the staleness tests.
#[derive(Debug, Clone, PartialEq)]
pub struct RecordedReading {
    pub settings: SettingsHalf,
    pub taken_by: BlenderIdentity,
    /// R10's four values — size, inode, modification time and change time
    /// together — as `Server protocol and transfers` S13's one type.
    pub file_identity: SourceReading,
    /// **Q18: the moment the header states — the desktop's clock, stamped at
    /// the act of storing the reading, a field of the reading it describes.**
    /// Nothing rests on it: staleness is R13's rule entire, the noticing
    /// computes no interval, and Q17's discard tests the version and the four
    /// values and not a moment. The desktop's clock is itself undefended
    /// (`Desktop app and delivery` T19); what that costs here is a moment
    /// drawn wrong, never a decision taken wrong.
    pub stored_at: observed::Reading<()>,
}

/// **The four values as text** — a rendering of S13's type for a record that
/// has a written form, and never a second comparison shape.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FourValues {
    pub size: u64,
    pub inode: String,
    pub modification_time: i64,
    pub change_time: i64,
}

impl FourValues {
    pub fn of(reading: &SourceReading) -> FourValues {
        FourValues {
            size: reading.size,
            inode: reading.inode.clone(),
            modification_time: reading.modification_time,
            change_time: reading.change_time,
        }
    }
}

/// **A reading's identity, recorded beside an anchored name** (J7, Y12): the
/// file identity, the version, and the moment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadingIdentity {
    pub file: FourValues,
    pub build_path: String,
    pub version: String,
    pub stored_at: SystemTime,
}

impl ReadingIdentity {
    /// The text a manifest field carries for it.
    pub fn as_text(&self) -> String {
        serde_json::to_string(self).expect("an identity is composed of values that always render")
    }
}

impl RecordedReading {
    pub fn identity(&self) -> ReadingIdentity {
        ReadingIdentity {
            file: FourValues::of(&self.file_identity),
            build_path: self.taken_by.path.clone(),
            version: version_text(self.taken_by.version),
            stored_at: self.stored_at.taken_at(),
        }
    }
}

/// **Q13: what counts as changed is the four-value file identity and nothing
/// weaker.** Size and modification time alone are refused on the observed
/// ground: a killed transfer leaves a piece of the file under the final name
/// with no marker of any kind, and a hand-restored modification time still
/// moves the change time (Fact 21). The comparison is the type's own equality
/// over all four, which is one identity mechanism for the whole project —
/// `Server protocol and transfers` S13, `Job files and outputs` R10 and this
/// clause are one shape.
pub fn the_file_changed(reading: &RecordedReading, now: &SourceReading) -> bool {
    reading.file_identity != *now
}

/// **`Job files and outputs` R13's rule, used here at Q13 and Y8: a recorded
/// reading is stale when, and only when, the four values differ or the
/// version differs.**
pub fn is_stale(
    reading: &RecordedReading,
    now: &SourceReading,
    version_in_force: &BlenderIdentity,
) -> bool {
    the_file_changed(reading, now) || reading.taken_by != *version_in_force
}

/// **Q17: a read carries what it was started under** — the version pick and
/// the four-value file identity — so that on landing Q6 stores the reading
/// only where both still stand.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartedUnder {
    pub version_pick: BlenderIdentity,
    pub file_identity: SourceReading,
}

/// What became of a reading that landed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Landed {
    /// **Q6: the file's half replaced whole** — never merged, never patched.
    Stored,
    /// **Q17: the version is not the page's current pick**, so the arriving
    /// reading is discarded and nothing is written.
    DiscardedTheVersionMoved,
    /// **Q17: the four values are not the ones now standing** for that
    /// (server, file), so the arriving reading is discarded and nothing is
    /// written.
    DiscardedTheFileMoved,
}

/// **J10: the three events that drop the file's half, all by the same single
/// act.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatDroppedIt {
    /// The connected period ends.
    ThePeriodEnded,
    /// **Q14 item 1**: any Smoozy publication onto that path — the publishing
    /// module performs the drop, at the publication, with no comparison
    /// (`File Browser` X7a — its uploads, its copies and its same-filesystem
    /// moves alike; `Dependency Upload & Remap`'s own publications likewise,
    /// that document's V13 mark being written by the same publication).
    /// **Widened 2026-09-05 from an upload alone**: a copy and a
    /// same-filesystem move publish over the same path with the same Replace
    /// answer, and a reading describes the file it read.
    ASmoozyPublicationOntoThePath,
    /// **Q14 item 2**: the comparison found the four values differ.
    TheFourValuesDiffer,
    /// **Q12**: the user changed the Blender version on this page.
    TheVersionChanged,
}

/// **What Q14 item 2's comparison found** at the page's open.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Noticed {
    Unchanged,
    /// The file's half is dropped and the page says the file changed.
    Changed,
    /// The channel could not answer on this rental: the comparison is not run
    /// and nothing is claimed.
    NotRun,
    /// No reading stands for the (server, file), so there is nothing to
    /// compare against.
    NothingStands,
}

/// **The file's half, as the desktop holds it: one reading per (server,
/// file), replaced whole and dropped whole.** Nothing anywhere holds a read
/// that outlives its version (Q12), because this store keys one reading per
/// (server, file) and Q6 replaces it whole, so there is no second copy to go
/// stale.
#[derive(Debug, Default)]
pub struct FileHalf {
    standing: HashMap<FileKey, RecordedReading>,
}

impl FileHalf {
    pub fn new() -> FileHalf {
        FileHalf::default()
    }

    pub fn standing(&self, key: &FileKey) -> Option<&RecordedReading> {
        self.standing.get(key)
    }

    /// **Q6, Q17 and Q18 as one act.** A reading that lands is stored only
    /// where the version pick it was started under is still the page's
    /// current pick and the four values it was started under are the ones now
    /// standing; otherwise it is discarded and nothing is written, the page
    /// left exactly where it was. Where it is stored, it replaces the file's
    /// half whole and is stamped with the desktop's moment at this act.
    ///
    /// **The override store is untouched by construction**: it is a different
    /// object and no code path here can reach it (J4).
    #[allow(clippy::too_many_arguments)]
    pub fn a_reading_landed(
        &mut self,
        key: FileKey,
        started_under: &StartedUnder,
        settings: SettingsHalf,
        the_pages_current_pick: Option<&BlenderIdentity>,
        the_four_values_now: &SourceReading,
        stored_at: SystemTime,
    ) -> Landed {
        if the_pages_current_pick != Some(&started_under.version_pick) {
            return Landed::DiscardedTheVersionMoved;
        }
        if started_under.file_identity != *the_four_values_now {
            return Landed::DiscardedTheFileMoved;
        }
        self.standing.insert(
            key,
            RecordedReading {
                settings,
                taken_by: started_under.version_pick.clone(),
                file_identity: started_under.file_identity.clone(),
                stored_at: observed::Reading::taken((), stored_at, Clock::ThisComputer),
            },
        );
        Landed::Stored
    }

    /// **J10: the recorded reading for that (server, file) is discarded
    /// whole** — every scene's values at once, the announced add-on case
    /// included (Q12) — and the store of the user's answers is not touched.
    /// The page then shows the store's entries alone over promise-form.
    pub fn drop_the_files_half(&mut self, key: &FileKey, _why: WhatDroppedIt) {
        self.standing.remove(key);
    }

    /// **J10's first event, for every file of that server at once.**
    pub fn the_period_ended(&mut self, server: &Identity) {
        self.standing.retain(|key, _| key.server != *server);
    }

    /// **Q14 item 2: opening the page onto a (server, file) a recorded reading
    /// stands for runs Q13's comparison**, and where the four values differ
    /// the file's half is dropped and the page says the file changed. **Q15:
    /// this is a use of the standing connection at a user's act — not a timer,
    /// not a poll, not admitted at the arbitration point, nothing durable.**
    /// A change landing between two of Q14's moments is not noticed until the
    /// next one; what guarantees the render is `Job files and outputs` R12's
    /// fresh check at every attempt start, and this page-side comparison sits
    /// outside R12 by that document's R12a (Q16).
    pub fn noticing_at_the_open(&mut self, key: &FileKey, now: FourValuesNow) -> Noticed {
        let Some(reading) = self.standing.get(key) else {
            return Noticed::NothingStands;
        };
        match now {
            FourValuesNow::TheChannelCannotAnswer => Noticed::NotRun,
            FourValuesNow::Answered(values) => {
                if the_file_changed(reading, &values) {
                    self.drop_the_files_half(key, WhatDroppedIt::TheFourValuesDiffer);
                    Noticed::Changed
                } else {
                    Noticed::Unchanged
                }
            }
        }
    }
}

/// **The readings this plan's tests are built from, in one place for both
/// folders.** `smoozy-render-settings` keeps tests over what stayed there — the
/// acting half, the diary, the endings and the version pick — and they build
/// their readings from these. A test-only module cannot be reached from another
/// crate, so this one is compiled for this crate's own tests and, behind the
/// `testing` switch, for that crate's tests, which turn it on as a development
/// dependency. **Nothing outside a test ever turns it on**, and the program a
/// rented machine runs does not. One copy rather than two, so the two folders'
/// tests cannot come to disagree about what a reading looks like.
#[cfg(any(test, feature = "testing"))]
pub mod testing {
    use super::*;
    use crate::names::ExactName;
    use crate::reading::*;
    use smoozy_state::identities::IdentityKind;
    use std::time::Duration;

    pub fn a_server() -> Identity {
        Identity::issue(IdentityKind::LocalServer).expect("randomness")
    }

    pub fn a_key(server: &Identity) -> FileKey {
        FileKey {
            server: server.clone(),
            path: "/ephemeral/smoozy/assets/kitchen.blend".to_string(),
        }
    }

    pub fn four_values(size: u64) -> SourceReading {
        SourceReading {
            size,
            inode: "912".to_string(),
            modification_time: 1_756_800_000,
            change_time: 1_756_800_001,
        }
    }

    pub fn a_version(major: u32, minor: u32, patch: u32) -> Version {
        Version {
            major,
            minor,
            patch,
        }
    }

    pub fn a_blender() -> BlenderIdentity {
        BlenderIdentity {
            path: "/ephemeral/smoozy/blender/blender-4.5.12/blender".to_string(),
            version: a_version(4, 5, 12),
        }
    }

    pub fn a_layer(name: &str, on: bool) -> ViewLayerReading {
        ViewLayerReading {
            name: ExactName::as_the_reading_produced_it(name),
            renders_at: (1..=10).map(|frame| (frame, on)).collect(),
            animation: SwitchAnimation::None { value: on },
            own_sample_count: 0,
            overrides_the_scenes_count: false,
        }
    }

    pub fn a_scene(name: &str) -> SceneReading {
        SceneReading {
            name: ExactName::as_the_reading_produced_it(name),
            frame_start: 1,
            frame_end: 10,
            frame_step: 1,
            frame_count: 10,
            full_size: (1920, 1080),
            resolution_percentage: 100,
            size_at_the_files_percentage: (1920, 1080),
            engine_as_read: ExactName::as_the_reading_produced_it("CYCLES"),
            sample_counts: PerEngineCounts {
                cycles: 64,
                eevee: 64,
            },
            device: DeviceSetting::TheCards,
            early_stop: EarlyStop {
                adaptive_sampling_is_on: true,
                threshold: 0.01,
                minimum: 0,
            },
            time_limit: 0.0,
            layer_samples_rule: LayerSamplesRule::Use,
            saved_with: "4.5.12".to_string(),
            view_layers: vec![a_layer("ViewLayer", true), a_layer("Detail", false)],
            pulls_in: Vec::new(),
            simplify: SimplifyReadBack {
                use_simplify: false,
                subdivision_render: 6,
                child_particles_render: 1.0,
                texture_limit_render: "OFF".to_string(),
            },
        }
    }

    pub fn a_settings_half() -> SettingsHalf {
        SettingsHalf {
            scenes: vec![a_scene("Kitchen"), a_scene("Exterior")],
            offered_engine_identifiers: OfferedEngineIdentifiers {
                cycles: Some(ExactName::as_the_reading_produced_it("CYCLES")),
                eevee: Some(ExactName::as_the_reading_produced_it("BLENDER_EEVEE_NEXT")),
            },
            announced_add_on_case: None,
        }
    }

    pub fn a_moment() -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_secs(1_756_900_000)
    }

    pub fn a_recorded_reading() -> RecordedReading {
        RecordedReading {
            settings: a_settings_half(),
            taken_by: a_blender(),
            file_identity: four_values(4096),
            stored_at: observed::Reading::taken((), a_moment(), Clock::ThisComputer),
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    pub(crate) use super::testing::*;

    fn started_under() -> StartedUnder {
        StartedUnder {
            version_pick: a_blender(),
            file_identity: four_values(4096),
        }
    }

    #[test]
    fn a_reading_that_lands_under_the_standing_pick_and_file_is_stored_whole_and_stamped() {
        let server = a_server();
        let mut half = FileHalf::new();
        let landed = half.a_reading_landed(
            a_key(&server),
            &started_under(),
            a_settings_half(),
            Some(&a_blender()),
            &four_values(4096),
            a_moment(),
        );
        assert_eq!(landed, Landed::Stored);
        let standing = half.standing(&a_key(&server)).expect("stored");
        assert_eq!(standing.stored_at.taken_at(), a_moment());
        assert_eq!(standing.stored_at.clock(), &Clock::ThisComputer);
        assert_eq!(standing.taken_by, a_blender());
    }

    #[test]
    fn a_reading_landing_after_the_version_moved_is_discarded_and_nothing_is_written() {
        let server = a_server();
        let mut half = FileHalf::new();
        let other = BlenderIdentity {
            path: "/opt/blender-5.2/blender".to_string(),
            version: a_version(5, 2, 1),
        };
        let landed = half.a_reading_landed(
            a_key(&server),
            &started_under(),
            a_settings_half(),
            Some(&other),
            &four_values(4096),
            a_moment(),
        );
        assert_eq!(landed, Landed::DiscardedTheVersionMoved);
        assert!(half.standing(&a_key(&server)).is_none());

        // A pick cleared under the read is no current pick, and the read is
        // discarded the same way (B5a's last bullet).
        let landed = half.a_reading_landed(
            a_key(&server),
            &started_under(),
            a_settings_half(),
            None,
            &four_values(4096),
            a_moment(),
        );
        assert_eq!(landed, Landed::DiscardedTheVersionMoved);
    }

    #[test]
    fn a_reading_landing_after_the_file_moved_is_discarded_and_nothing_is_written() {
        let server = a_server();
        let mut half = FileHalf::new();
        let landed = half.a_reading_landed(
            a_key(&server),
            &started_under(),
            a_settings_half(),
            Some(&a_blender()),
            &four_values(8192),
            a_moment(),
        );
        assert_eq!(landed, Landed::DiscardedTheFileMoved);
        assert!(half.standing(&a_key(&server)).is_none());
    }

    #[test]
    fn a_landing_replaces_the_files_half_whole_and_never_merges() {
        let server = a_server();
        let mut half = FileHalf::new();
        half.a_reading_landed(
            a_key(&server),
            &started_under(),
            a_settings_half(),
            Some(&a_blender()),
            &four_values(4096),
            a_moment(),
        );
        let mut fewer = a_settings_half();
        fewer.scenes.truncate(1);
        half.a_reading_landed(
            a_key(&server),
            &started_under(),
            fewer.clone(),
            Some(&a_blender()),
            &four_values(4096),
            a_moment(),
        );
        assert_eq!(half.standing(&a_key(&server)).unwrap().settings, fewer);
    }

    #[test]
    fn the_four_values_and_nothing_weaker_decide_changed() {
        let reading = a_recorded_reading();
        assert!(!the_file_changed(&reading, &four_values(4096)));
        // Same size and modification time, a moved change time: changed.
        let mut restored = four_values(4096);
        restored.change_time += 1;
        assert!(the_file_changed(&reading, &restored));
        assert!(the_file_changed(&reading, &four_values(4097)));
    }

    #[test]
    fn a_reading_is_stale_on_the_four_values_or_the_version_and_on_nothing_else() {
        let reading = a_recorded_reading();
        assert!(!is_stale(&reading, &four_values(4096), &a_blender()));
        assert!(is_stale(&reading, &four_values(1), &a_blender()));
        let other = BlenderIdentity {
            path: "/opt/blender-5.2/blender".to_string(),
            version: a_version(5, 2, 1),
        };
        assert!(is_stale(&reading, &four_values(4096), &other));
        // Two builds of one version are two identities (B3, `Server Setup`
        // B13).
        let same_version_other_path = BlenderIdentity {
            path: "/opt/other/blender".to_string(),
            version: a_version(4, 5, 12),
        };
        assert!(is_stale(
            &reading,
            &four_values(4096),
            &same_version_other_path
        ));
    }

    #[test]
    fn the_three_drops_take_the_files_half_whole_and_the_periods_end_takes_every_file_of_that_server(
    ) {
        let server = a_server();
        let other_server = a_server();
        let mut half = FileHalf::new();
        for key in [a_key(&server), a_key(&other_server)] {
            half.a_reading_landed(
                key,
                &started_under(),
                a_settings_half(),
                Some(&a_blender()),
                &four_values(4096),
                a_moment(),
            );
        }
        half.drop_the_files_half(&a_key(&server), WhatDroppedIt::TheVersionChanged);
        assert!(half.standing(&a_key(&server)).is_none());
        assert!(half.standing(&a_key(&other_server)).is_some());

        half.the_period_ended(&other_server);
        assert!(half.standing(&a_key(&other_server)).is_none());
    }

    #[test]
    fn the_noticing_at_the_open_drops_on_a_difference_and_claims_nothing_where_the_channel_cannot_answer(
    ) {
        let server = a_server();
        let mut half = FileHalf::new();
        assert_eq!(
            half.noticing_at_the_open(&a_key(&server), FourValuesNow::Answered(four_values(1))),
            Noticed::NothingStands
        );
        half.a_reading_landed(
            a_key(&server),
            &started_under(),
            a_settings_half(),
            Some(&a_blender()),
            &four_values(4096),
            a_moment(),
        );
        assert_eq!(
            half.noticing_at_the_open(&a_key(&server), FourValuesNow::TheChannelCannotAnswer),
            Noticed::NotRun
        );
        assert!(half.standing(&a_key(&server)).is_some());
        assert_eq!(
            half.noticing_at_the_open(&a_key(&server), FourValuesNow::Answered(four_values(4096))),
            Noticed::Unchanged
        );
        assert_eq!(
            half.noticing_at_the_open(&a_key(&server), FourValuesNow::Answered(four_values(1))),
            Noticed::Changed
        );
        assert!(half.standing(&a_key(&server)).is_none());
    }

    #[test]
    fn a_readings_identity_carries_the_file_the_build_and_the_moment() {
        let identity = a_recorded_reading().identity();
        assert_eq!(identity.file, FourValues::of(&four_values(4096)));
        assert_eq!(identity.version, "4.5.12");
        assert_eq!(identity.build_path, a_blender().path);
        assert_eq!(identity.stored_at, a_moment());
        assert!(identity.as_text().contains("4.5.12"));
    }
}
