//! §3.1 and §3.2 — the immutable submission manifest.
//!
//! **M1: one manifest per job, written once and never rewritten.** It holds,
//! and holds nothing that changes.
//!
//! **M2: immutability is what `Rendering`'s Row 15 needs.** That row requires
//! that none of what a render must have can be something Smoozy applies by hand
//! each time a render begins, because a re-run happens with nobody connected.
//! **A re-run reads the manifest; it never recomposes one.** That is kept here
//! by the type: every field is set at [`SubmissionManifest::composed`] and
//! there is no method anywhere that changes one.
//!
//! **M1b: the card make's form, and why the field is here rather than read on
//! the machine.** Added to the plan 2026-09-20 out of a Build seam — `Rendering`
//! L29 said the manifest carried the cards and M1's list had no card field, so
//! the render worker had nothing to look the technology up from and **every
//! cards job ended at L23's loud failure before the render**.
//!
//! **M3: the manifest carries no observed value the machine still holds and no
//! state.** It is material. `State, identity and recovery` §3's D4 forbids a
//! durable copy of an observed machine state, and **two fields in M1 are
//! snapshots deliberately taken rather than copies of state**: the four-value
//! file identity, which is compared against, and the card make, which M1b
//! argues at length — taken once at submission, never believed past a refusal,
//! and wrong only into a loud ending with nothing written.
//!
//! **M4: the manifest and everything derived from it sit in the job record
//! directory, outside the job output folder**, for L7's three reasons.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` S13.
//! `Rendering` S5a. `Render Settings` Y9, Y12, Y16. `Telegram` V1.

use serde::{Deserialize, Serialize};
use smoozy_framing::source_reading::SourceReading;
use smoozy_state::identities::{Identity, IdentityKind};

/// **M1a's frame-range form: one of two and never a third.**
///
/// *"The user's parsed integers, or the explicit statement *the file's own*, the
/// second composed against the reading in force at the attempt"* (`Render
/// Settings` Y9).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FrameRange {
    /// Integers, parsed on the desktop before the manifest was composed.
    /// **Never text for the machine to parse.**
    TheUsersParsedIntegers(Vec<i64>),
    /// The explicit statement, composed against the reading in force at the
    /// attempt.
    TheFilesOwn,
}

/// **M1a's scene field: the name, and beside it that name's anchor state.**
///
/// *"Which is what lets a name-shaped entry be validated against the reading in
/// force before the render instruction"* (`Render Settings` Y12).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SceneField {
    pub name: String,
    pub anchor: AnchorState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnchorState {
    /// Anchored, **with the identity of the reading its bytes came from**.
    Anchored { the_reading_it_came_from: String },
    /// To-anchor, with its provenance.
    ToAnchor { provenance: String },
}

/// **M1a's build field: path beside version.**
///
/// *"`Server Setup` B13's matching rule at submission: **path beside
/// version**, because two builds of one version answer the same version when
/// asked, so a version alone cannot identify a build at launch."* **That
/// document's B13 and not this one's**, which is about when a preview pick is
/// made.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TheBlenderToRenderBy {
    /// The version `Render Settings` selected.
    pub version: String,
    /// **The path of the Blender build `Server Setup` picked**, beside the
    /// version above (`Server Setup` B13).
    pub build_path: String,
}

/// Material this document carries and does not interpret.
///
/// **Three of M1's fields are of this kind** — the settings payload `Render
/// Settings` composed, the reference to the remap material `Dependency Upload
/// & Remap` keeps in the asset area, and the per-job reporting settings
/// `Telegram` V1 fixed at submission. **Each is *carried and not interpreted
/// here***, and the type says so: nothing in this crate reads inside one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CarriedAndNotInterpretedHere(pub serde_json::Value);

/// **M1's recorded-reading field, and its two states.**
///
/// *"The **recorded reading** and everything derived from it, where the file was
/// read before submitting, and **a statement that there is none** where it was
/// not."*
///
/// **R7's two bullets, and both are ordinary cases rather than one being a
/// gap.** Where the file was read before submitting, the desktop composes the
/// recorded reading and the map, and both go into the manifest at submission.
/// **Where it was not, the manifest says so and carries no map**, and the render
/// worker takes the reading after the load and before the render.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TheRecordedReading {
    /// **The statement M1 requires**, and it is a statement rather than an
    /// absent field. A reader that met a missing field could not tell *not
    /// read* from *the manifest is short*.
    ThereIsNone,
    /// The reading and everything derived from it (§4).
    ///
    /// **Boxed because it is much the larger of the two states**, and a
    /// manifest that is not read before submitting is an ordinary case rather
    /// than a rare one (R7's second bullet).
    ItWasReadBeforeSubmitting(Box<TheReadingAndEverythingDerivedFromIt>),
}

/// **R4 item 7: the reading, and the six things derived from it.**
///
/// *"The destination map, the collision record, the predicted names, the count,
/// the beauty walk's outcome and the two picks."* **The walk's outcome sits on
/// the reading itself** (B9), and the collision record, the predicted names and
/// the count are the inventory's own fields (I14), so the four values below hold
/// all seven items between them.
///
/// **R9: nothing anywhere recomputes a map that exists.** This is the object the
/// render applies, and *"the folders the user was shown and the folders the
/// render writes into are the same object, not two computations that have to
/// agree"* (R7).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TheReadingAndEverythingDerivedFromIt {
    pub reading: crate::reading::TheOutputsHalf,
    pub map: crate::mapping::TheMap,
    /// **I9a's withholding is a state of this field**, not an absence of it: a
    /// job whose scene writes more than one view separately carries the
    /// statement that the frame set is not predictable, with the reason named.
    pub inventory: crate::inventory::WhatTheInventoryIs,
    /// **B13: each pick is made once, at the moment the recorded reading is
    /// composed**, and B15 forbids recomputing either.
    pub picks: crate::main_output::ThePicks,
}

/// **M1b's card make field: one value, five states, and never a sixth.**
///
/// *"The make alone — never a copy of the hardware reading, never a card's
/// identity, its bus location or its memory, and never the technology
/// itself."* The technology is `Rendering` L23's table and is looked up on the
/// machine; **what crosses is the make**.
///
/// **Why this is on the manifest at all** is M1b: the make comes from the
/// hardware reading `Servers` holds, which is a desktop reading (`Servers`
/// H14, H15), so the render worker has no way to take one — and `Rendering`
/// L29 requires everything the launch needs to be durable on the machine
/// before the job exists.
///
/// **Markers used here that belong to another document, each with the
/// document it belongs to.** `Servers` H11, H12, H14, H15. `Rendering` L23,
/// L24, L29.
///
/// **Two are deliberately off that list and are written out at their sites
/// instead** — `Servers` H1 and `Servers` H7 — this plan defining an H1 and
/// an H7 of its own, so a bare one could mean either.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TheCardMakeAtSubmission {
    /// **The makes the reading's cards carried**, as the keyable values
    /// `Servers` H7 requires and never a wording to be parsed, **each
    /// distinct make once**.
    TheMakes(Vec<String>),
    /// The reading returned no card at all (`Servers` H11).
    NoCard,
    /// Cards are present and the driver could not read them, so no make is on
    /// the reading to key on (`Servers` H11).
    PresentAndUnreadable,
    /// The cards could not be established — a container, or a bus scan with
    /// no route (`Servers` H11, H12).
    CouldNotBeEstablished,
    /// **No hardware reading stood for this machine at submission** —
    /// `Servers` H1's *never read*, **an ordinary case rather than a gap**.
    NoReadingStood,
}

/// **M1, in full.**
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubmissionManifest {
    /// The **job identity**, and the group identity where the job came from an
    /// all-scenes submission.
    pub job: String,
    pub group: Option<String>,
    /// The **Blender version** the job is to be rendered by, as `Render
    /// Settings` selected it, and **the path of the build** beside it.
    pub blender: TheBlenderToRenderBy,
    /// The **scene** the job is, with M1a's anchor state.
    pub scene: SceneField,
    /// The **frame range** the job covers, in M1a's one of two forms.
    pub frame_range: FrameRange,
    /// The **settings payload** `Render Settings` composed.
    pub settings_payload: CarriedAndNotInterpretedHere,
    /// The reference to the **remap material** `Dependency Upload & Remap`
    /// keeps in the asset area.
    pub remap_material: CarriedAndNotInterpretedHere,
    /// The **per-job reporting settings** fixed at submission (`Telegram` V1).
    ///
    /// **`Rendering` S5a composes its value and `Render Settings` Y16 supplies
    /// the control**; the manifest is the vehicle because it is the one per-job
    /// material written once and never rewritten, **and a job submitted without
    /// it sits on an immutable record that can never be repaired**.
    pub reporting_settings: CarriedAndNotInterpretedHere,
    /// The **`.blend`'s path on the machine**.
    pub blend_path_on_the_machine: String,
    /// Its **four-value file identity** at submission — size, inode,
    /// modification time and change time together (R10).
    ///
    /// **R11: this is the same mechanism another closed document already
    /// uses**, so it is that document's own type rather than a second one.
    /// *"One shape rather than two."*
    #[serde(with = "the_four_values")]
    pub file_identity_at_submission: SourceReading,
    /// The **recorded reading**, or the statement that there is none.
    pub recorded_reading: TheRecordedReading,
    /// **The card make the hardware reading `Servers` holds gave for the
    /// machine at submission** (M1b; `Rendering` L23, L29).
    ///
    /// **M3 is argued for this field rather than assumed**: it is a snapshot
    /// deliberately taken, like the four values above, and a stale or wrong
    /// make cannot produce a wrong render — only `Rendering` L23's loud ending
    /// before the render instruction, or L24's refusal at launch with nothing
    /// written.
    pub card_make: TheCardMakeAtSubmission,
    /// The **job output folder's path** — or, from 2026-10-04, **the
    /// statement that it is chosen at render start** (§6.6 P21's second
    /// case), with the server's chosen fallback folder beside it (P23).
    ///
    /// **Where it names a path, the path is the one the placement makes.** It
    /// is taken from the same [`crate::publication::AJobsPaths`] the placement
    /// creates the folder from ([`WhereTheJobOutputFolderIs::named_by`]), so
    /// the folder the manifest names and the folder that is actually made are
    /// **one value** rather than two that have to be checked against each
    /// other. *(Made a property of the type 2026-09-03 at the code round: they
    /// were composed separately and nothing anywhere compared them.)*
    ///
    /// **What a disagreement costs.** M2 has the render worker read the
    /// manifest and point the redirect at the folder it names; Blender makes a
    /// folder that is not there without a word (D22, N11); `Download` watches
    /// the folder Smoozy made, which stays empty; the completeness statement
    /// reports every expected file absent; and the carry-on re-renders the
    /// whole range. **The render worked and the user pays twice.**
    pub job_output_folder: WhereTheJobOutputFolderIs,
}

/// **M1's job output folder field, in one of two forms and never a third.**
///
/// **Written as it always was where it names a path** — a plain string, the
/// form every manifest before §6.6 carries — so a re-run of an older job reads
/// its own recorded folder and nothing already on a server moves. **The render
/// start statement is an object**, so the two forms can never be read as each
/// other.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WhereTheJobOutputFolderIs {
    /// **The path, named at submission.** Every manifest before 2026-10-04,
    /// in the job area of L6, and every manifest a desktop chooses for (P21's
    /// first case — which no route reaches while the desktop writes no
    /// recorded reading into a manifest: `Job files and outputs` R7, the
    /// runner's ruling of 2026-09-29).
    NamedAtSubmission(String),
    /// **P21's second case: chosen at render start**, by the render worker at
    /// `Rendering` P1 step 4.
    ChosenAtRenderStart {
        /// **N3's name for the job output folder**, composed on the desktop,
        /// where the label is: the identity and a bounded rendering of the
        /// label (`crate::names::job_output_folder_name`). The worker joins it
        /// to the place it chooses. Build's own field: the machine holds no
        /// label, and a name composed there would lose it.
        the_folders_name: String,
        /// **P23: the server row's chosen fallback folder as it stood at
        /// submission**, or nothing where none was set.
        the_servers_fallback_folder_at_submission: Option<String>,
    },
}

impl WhereTheJobOutputFolderIs {
    /// **The folder the placement makes, named** — the one value M1 holds.
    pub fn named_by(paths: &crate::publication::AJobsPaths) -> WhereTheJobOutputFolderIs {
        WhereTheJobOutputFolderIs::NamedAtSubmission(paths.job_output_folder.clone())
    }

    /// The path, where the manifest names one.
    pub fn the_path_it_names(&self) -> Option<&str> {
        match self {
            WhereTheJobOutputFolderIs::NamedAtSubmission(path) => Some(path),
            WhereTheJobOutputFolderIs::ChosenAtRenderStart { .. } => None,
        }
    }
}

/// **R10's four values, written and read back.**
///
/// The type is `Server protocol and transfers`' own, because **R11 requires one
/// shape rather than two** — *"§7.4 S13 records all four values when a download
/// is arranged and compares all four before a resume"*. That crate does not
/// render its own types, so the written form is composed here. **This is a
/// rendering and not a second shape**: nothing compares against it, and the
/// comparison is still the one type's own.
pub mod the_four_values {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use smoozy_framing::source_reading::SourceReading;

    #[derive(Serialize, Deserialize)]
    struct AsWritten {
        size: u64,
        inode: String,
        modification_time: i64,
        change_time: i64,
    }

    pub fn serialize<S: Serializer>(reading: &SourceReading, out: S) -> Result<S::Ok, S::Error> {
        AsWritten {
            size: reading.size,
            inode: reading.inode.clone(),
            modification_time: reading.modification_time,
            change_time: reading.change_time,
        }
        .serialize(out)
    }

    pub fn deserialize<'a, D: Deserializer<'a>>(input: D) -> Result<SourceReading, D::Error> {
        let written = AsWritten::deserialize(input)?;
        Ok(SourceReading {
            size: written.size,
            inode: written.inode,
            modification_time: written.modification_time,
            change_time: written.change_time,
        })
    }
}

/// The file the manifest is written as, inside the job record directory.
///
/// **Build's own choice of name and form.** The plan fixes neither. **JSON is
/// the form this document already uses on a machine** — R1's reading comes back
/// as *"one JSON document between sentinels"* — so the manifest takes the same
/// shape rather than a second one.
pub const MANIFEST_FILE_NAME: &str = "manifest.json";

/// **The file the recorded reading is written as, inside the job record
/// directory, where the render worker took the reading at render start.**
///
/// **Build's own choice of name and form**, as the manifest's is: `State,
/// identity and recovery` §1.2 names what the directory holds and fixes no file
/// name, and JSON is the form every other record on the machine takes. **One
/// per attempt**, because R13 keeps each attempt's own reading and map rather
/// than overwriting the last.
///
/// **Here rather than in the machine's program, beside the manifest's name,
/// because both ends read it.** The render worker writes it (`Rendering` P1
/// step 4) and the desktop reads it back (`Download` N2). Moved here from the
/// program at step 10c's ninth-pass fix piece 1, so the one name has one home;
/// the machine's behaviour did not change.
pub fn the_recorded_readings_file_name(attempt: u32) -> String {
    format!("recorded-reading-{attempt}.json")
}

impl SubmissionManifest {
    /// **Composed once.** There is no method that changes a field afterwards,
    /// which is M1's *written once and never rewritten* and M2's *a re-run
    /// reads the manifest; it never recomposes one*, kept by the type.
    ///
    /// **The job output folder is M1's one field in one of two forms**: named
    /// by the paths the placement creates it from
    /// ([`WhereTheJobOutputFolderIs::named_by`]), so the manifest cannot name a
    /// folder other than the one that is made, or chosen at render start.
    #[allow(clippy::too_many_arguments)]
    pub fn composed(
        job: &Identity,
        group: Option<&Identity>,
        blender: TheBlenderToRenderBy,
        scene: SceneField,
        frame_range: FrameRange,
        settings_payload: CarriedAndNotInterpretedHere,
        remap_material: CarriedAndNotInterpretedHere,
        reporting_settings: CarriedAndNotInterpretedHere,
        blend_path_on_the_machine: String,
        file_identity_at_submission: SourceReading,
        recorded_reading: TheRecordedReading,
        card_make: TheCardMakeAtSubmission,
        job_output_folder: WhereTheJobOutputFolderIs,
    ) -> SubmissionManifest {
        debug_assert_eq!(job.kind(), IdentityKind::Job);
        // **M1's group field is the group identity, so the guard asks for the
        // kind that is right rather than ruling one wrong kind out.** A
        // transfer, an operation, a server or an installation identity used to
        // pass — and the manifest is immutable, so a wrong value is on the
        // machine for the life of the job and the all-scenes grouping is wrong
        // wherever it is read.
        debug_assert!(group.is_none_or(|group| group.kind() == IdentityKind::Group));
        SubmissionManifest {
            job: job.as_text(),
            group: group.map(Identity::as_text),
            blender,
            scene,
            frame_range,
            settings_payload,
            remap_material,
            reporting_settings,
            blend_path_on_the_machine,
            file_identity_at_submission,
            recorded_reading,
            card_make,
            job_output_folder,
        }
    }

    /// **`Telegram` Y1's submission, which the machine makes for itself: this
    /// manifest copied under a new job identity.** Added 2026-10-16, the
    /// Telegram worker's piece 5.
    ///
    /// **The original is not touched** (M1, M2): the copy is a second
    /// manifest, written once into the new job's own record directory, and
    /// the failed job's stays as it was written. `State, identity and
    /// recovery` Y7a has the supervisor issue `job` before this is called.
    ///
    /// **What changes, and nothing else does**:
    ///
    /// - **the job identity**, to `job`;
    /// - **the job output folder**, because a fresh attempt the user asks for
    ///   is a new job with a new identity **and a new folder** (`Queue &
    ///   History` Y5; `State, identity and recovery` §5.4). N3's name for it
    ///   is composed from `job` and `label`, the failed job's label from its
    ///   note: where the original named a path, the new folder sits beside the
    ///   old one, in the same parent; where it was to be chosen at render
    ///   start, it still is, under the new name and with the same fallback
    ///   folder (P21, P23). N5's budget is checked on the new path;
    /// - **the group identity, which is left out.** The copy comes from one
    ///   tap on one job and not from an all-scenes submission, and the group
    ///   it came from has already spoken once its last member ended
    ///   (`Telegram` G1), so a member joining afterwards would make it speak
    ///   again.
    ///
    /// Everything that decides the render — the Blender, its build, the
    /// scene, the frame range, the settings, the remap material, the file and
    /// its identity, the reading, the card make, the reporting settings — is
    /// copied as it was, which is the spec's *exactly the same job again,
    /// unchanged*.
    pub fn copied_under(
        &self,
        job: &Identity,
        label: Option<&str>,
    ) -> Result<SubmissionManifest, crate::names::WhyThePathIsRefused> {
        debug_assert_eq!(job.kind(), IdentityKind::Job);
        let the_folders_name = crate::names::job_output_folder_name(job, label);
        crate::names::a_name_fits(&the_folders_name)?;
        let job_output_folder = match &self.job_output_folder {
            WhereTheJobOutputFolderIs::NamedAtSubmission(path) => {
                let beside = match path.trim_end_matches('/').rsplit_once('/') {
                    Some(("", _)) => "/".to_string(),
                    Some((parent, _)) => format!("{parent}/"),
                    None => String::new(),
                };
                let new = format!("{beside}{the_folders_name}");
                crate::names::a_path_fits(&new)?;
                WhereTheJobOutputFolderIs::NamedAtSubmission(new)
            }
            WhereTheJobOutputFolderIs::ChosenAtRenderStart {
                the_servers_fallback_folder_at_submission,
                ..
            } => WhereTheJobOutputFolderIs::ChosenAtRenderStart {
                the_folders_name,
                the_servers_fallback_folder_at_submission:
                    the_servers_fallback_folder_at_submission.clone(),
            },
        };
        Ok(SubmissionManifest {
            job: job.as_text(),
            group: None,
            job_output_folder,
            ..self.clone()
        })
    }

    /// The bytes that go on the machine.
    pub fn as_written(&self) -> Vec<u8> {
        serde_json::to_vec_pretty(self)
            .expect("a manifest is composed of values that always render")
    }

    /// What a re-run reads (M2).
    pub fn read_back(bytes: &[u8]) -> Option<SubmissionManifest> {
        serde_json::from_slice(bytes).ok()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::layout::{MachineRoot, TheLayout, WhereThePackageAreaSits};
    use crate::publication::AJobsPaths;

    /// The layout every manifest below is composed against.
    pub(crate) fn a_layout() -> TheLayout {
        TheLayout::of(
            MachineRoot::chosen("/ephemeral/smoozy").unwrap(),
            WhereThePackageAreaSits::UnderTheRoot,
        )
    }

    /// **One label, and the manifest's folder is the paths' folder.** This
    /// helper used to compose a folder of its own carrying the label
    /// `Kitchen`, while the placement's own test composed its paths with no
    /// label at all — and the two disagreed with nothing anywhere noticing.
    pub(crate) fn a_manifest(job: &Identity, paths: &AJobsPaths) -> SubmissionManifest {
        SubmissionManifest::composed(
            job,
            None,
            TheBlenderToRenderBy {
                version: "4.5.9".to_string(),
                build_path: "/ephemeral/smoozy/blender/blender-4.5.9/blender".to_string(),
            },
            SceneField {
                name: "Scene".to_string(),
                anchor: AnchorState::Anchored {
                    the_reading_it_came_from: "reading-1".to_string(),
                },
            },
            FrameRange::TheUsersParsedIntegers(vec![1, 2, 3]),
            CarriedAndNotInterpretedHere(serde_json::json!({"samples": 128})),
            CarriedAndNotInterpretedHere(serde_json::json!({"remap": "asset-set-1"})),
            CarriedAndNotInterpretedHere(serde_json::json!({"reporting": "everything"})),
            "/ephemeral/smoozy/assets/kitchen.blend".to_string(),
            SourceReading {
                size: 4096,
                inode: "912".to_string(),
                modification_time: 1_756_800_000,
                change_time: 1_756_800_001,
            },
            TheRecordedReading::ThereIsNone,
            TheCardMakeAtSubmission::TheMakes(vec!["NVIDIA".to_string()]),
            WhereTheJobOutputFolderIs::named_by(paths),
        )
    }

    /// A job and the paths composed for it, which is the only way a manifest is
    /// composed at all now.
    pub(crate) fn a_job_and_its_paths() -> (Identity, AJobsPaths) {
        let job = Identity::issue(IdentityKind::Job).unwrap();
        let paths = AJobsPaths::composed(&a_layout(), &job, Some("Kitchen")).unwrap();
        (job, paths)
    }

    #[test]
    fn a_manifest_is_written_once_and_reads_back_as_itself() {
        let (job, paths) = a_job_and_its_paths();
        let manifest = a_manifest(&job, &paths);
        let again = SubmissionManifest::read_back(&manifest.as_written()).unwrap();
        assert_eq!(manifest, again, "a re-run must read what submission wrote");
    }

    #[test]
    fn the_job_output_folder_is_the_paths_own_and_cannot_be_given_a_second_value() {
        // **M1, M2 and M7 together.** The render worker reads the manifest and
        // points the redirect at the folder it names, so a manifest naming a
        // folder other than the one M7 creates sends the render somewhere
        // Blender makes silently, and every promise about the folder Smoozy
        // made then reads as an empty render.
        let (job, paths) = a_job_and_its_paths();
        assert_eq!(
            a_manifest(&job, &paths)
                .job_output_folder
                .the_path_it_names(),
            Some(paths.job_output_folder.as_str())
        );
    }

    /// **§6.6 P21: a manifest from before 2026-10-04 names its folder as a
    /// plain string and still reads back as naming it**, so a re-run of an
    /// older job keeps its recorded folder; **the render-start statement is a
    /// form of its own** and carries N3's name and the server's chosen
    /// fallback folder (P23).
    #[test]
    fn an_older_manifests_folder_reads_back_and_the_render_start_statement_is_its_own_form() {
        let (job, paths) = a_job_and_its_paths();
        let mut older: serde_json::Value =
            serde_json::from_slice(&a_manifest(&job, &paths).as_written()).unwrap();
        assert!(
            older["job_output_folder"].is_string(),
            "written as it always was"
        );
        older["job_output_folder"] = serde_json::Value::from("/ephemeral/smoozy/jobs/older");
        let read = SubmissionManifest::read_back(&serde_json::to_vec(&older).unwrap()).unwrap();
        assert_eq!(
            read.job_output_folder.the_path_it_names(),
            Some("/ephemeral/smoozy/jobs/older")
        );

        for fallback in [None, Some("/mnt/renders".to_string())] {
            let at_render_start = WhereTheJobOutputFolderIs::ChosenAtRenderStart {
                the_folders_name: "job-Kitchen".to_string(),
                the_servers_fallback_folder_at_submission: fallback.clone(),
            };
            let written = serde_json::to_vec(&at_render_start).unwrap();
            let again: WhereTheJobOutputFolderIs = serde_json::from_slice(&written).unwrap();
            assert_eq!(again, at_render_start);
            assert_eq!(again.the_path_it_names(), None);
        }
    }

    #[test]
    fn the_recorded_readings_absence_is_a_statement_and_not_a_missing_field() {
        let (job, paths) = a_job_and_its_paths();
        let written = String::from_utf8(a_manifest(&job, &paths).as_written()).unwrap();
        assert!(
            written.contains("ThereIsNone"),
            "M1 asks for a statement that there is none, not a silence: {written}"
        );
    }

    #[test]
    fn the_frame_range_takes_one_of_two_forms_and_never_a_third() {
        // M1a. A round trip of each, so neither can be composed as text the
        // machine would have to parse.
        for range in [
            FrameRange::TheUsersParsedIntegers(vec![7, 8, 9]),
            FrameRange::TheFilesOwn,
        ] {
            let written = serde_json::to_vec(&range).unwrap();
            let again: FrameRange = serde_json::from_slice(&written).unwrap();
            assert_eq!(range, again);
        }
    }

    #[test]
    fn the_build_path_travels_beside_the_version() {
        // `Server Setup` B13: two builds of one version answer the same
        // version when asked, so a version alone cannot identify a build at
        // launch.
        let (job, paths) = a_job_and_its_paths();
        let manifest = a_manifest(&job, &paths);
        assert!(!manifest.blender.build_path.is_empty());
        assert!(!manifest.blender.version.is_empty());
    }

    #[test]
    fn the_reporting_field_is_on_the_manifest_whether_or_not_telegram_is_on() {
        // `Rendering` S5a: it must be on every job's manifest, the manifest is
        // written once and never rewritten, **so a job submitted without it can
        // never be repaired**. The field is not an `Option`, which is what
        // makes composing a manifest without one impossible.
        let (job, paths) = a_job_and_its_paths();
        let manifest = a_manifest(&job, &paths);
        let _: &CarriedAndNotInterpretedHere = &manifest.reporting_settings;
    }

    /// **F-11's guard, and it stays a builder's guard.** Turning it into a hard
    /// check would stop the shipped application dead in front of the user where
    /// today it does not, which no clause requires — so it is a debug assertion
    /// and the test asks for it only where debug assertions are on.
    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn the_group_field_must_be_a_group_identity_and_not_merely_not_a_run() {
        // A transfer identity used to pass this guard, and the manifest is
        // immutable, so the all-scenes grouping would be wrong for the life of
        // the job wherever it is read.
        let (job, paths) = a_job_and_its_paths();
        let not_a_group = Identity::issue(IdentityKind::Transfer).unwrap();
        let _ = SubmissionManifest::composed(
            &job,
            Some(&not_a_group),
            TheBlenderToRenderBy {
                version: "4.5.9".to_string(),
                build_path: "/ephemeral/smoozy/blender/blender-4.5.9/blender".to_string(),
            },
            SceneField {
                name: "Scene".to_string(),
                anchor: AnchorState::ToAnchor {
                    provenance: "the user typed it".to_string(),
                },
            },
            FrameRange::TheFilesOwn,
            CarriedAndNotInterpretedHere(serde_json::json!({})),
            CarriedAndNotInterpretedHere(serde_json::json!({})),
            CarriedAndNotInterpretedHere(serde_json::json!({})),
            "/ephemeral/smoozy/assets/kitchen.blend".to_string(),
            SourceReading {
                size: 4096,
                inode: "912".to_string(),
                modification_time: 1_756_800_000,
                change_time: 1_756_800_001,
            },
            TheRecordedReading::ThereIsNone,
            TheCardMakeAtSubmission::TheMakes(vec!["NVIDIA".to_string()]),
            WhereTheJobOutputFolderIs::named_by(&paths),
        );
    }

    #[cfg(debug_assertions)]
    #[test]
    fn a_group_identity_passes_the_guard() {
        let (job, paths) = a_job_and_its_paths();
        let group = Identity::issue(IdentityKind::Group).unwrap();
        let manifest = SubmissionManifest::composed(
            &job,
            Some(&group),
            TheBlenderToRenderBy {
                version: "4.5.9".to_string(),
                build_path: "/ephemeral/smoozy/blender/blender-4.5.9/blender".to_string(),
            },
            SceneField {
                name: "Scene".to_string(),
                anchor: AnchorState::ToAnchor {
                    provenance: "the user typed it".to_string(),
                },
            },
            FrameRange::TheFilesOwn,
            CarriedAndNotInterpretedHere(serde_json::json!({})),
            CarriedAndNotInterpretedHere(serde_json::json!({})),
            CarriedAndNotInterpretedHere(serde_json::json!({})),
            "/ephemeral/smoozy/assets/kitchen.blend".to_string(),
            SourceReading {
                size: 4096,
                inode: "912".to_string(),
                modification_time: 1_756_800_000,
                change_time: 1_756_800_001,
            },
            TheRecordedReading::ThereIsNone,
            TheCardMakeAtSubmission::TheMakes(vec!["NVIDIA".to_string()]),
            WhereTheJobOutputFolderIs::named_by(&paths),
        );
        assert_eq!(manifest.group, Some(group.as_text()));
    }

    /// **`Telegram` Y1: the copy under a new identity** changes the job, the
    /// folder and the group and nothing else; the original is untouched; and
    /// the folder is new in both of P21's forms, beside the old one where a
    /// path was named.
    #[test]
    fn a_copy_under_a_new_identity_changes_the_job_the_folder_and_the_group_and_nothing_else() {
        let (job, paths) = a_job_and_its_paths();
        let mut original = a_manifest(&job, &paths);
        original.group = Some(Identity::issue(IdentityKind::Group).unwrap().as_text());
        let before = original.as_written();
        let new = Identity::issue(IdentityKind::Job).unwrap();

        let copy = original
            .copied_under(&new, Some("Kitchen"))
            .expect("it fits");
        assert_eq!(
            original.as_written(),
            before,
            "the original is not touched (M2)"
        );
        assert_eq!(copy.job, new.as_text());
        assert_eq!(copy.group, None);
        let new_paths = AJobsPaths::composed(&a_layout(), &new, Some("Kitchen")).unwrap();
        assert_eq!(
            copy.job_output_folder.the_path_it_names(),
            Some(new_paths.job_output_folder.as_str())
        );
        assert_ne!(
            copy.job_output_folder, original.job_output_folder,
            "a new folder"
        );
        let unchanged = SubmissionManifest {
            job: original.job.clone(),
            group: original.group.clone(),
            job_output_folder: original.job_output_folder.clone(),
            ..copy.clone()
        };
        assert_eq!(unchanged, original, "everything else is copied as it was");
        assert_eq!(
            SubmissionManifest::read_back(&copy.as_written()),
            Some(copy.clone())
        );

        let mut chosen_later = original.clone();
        chosen_later.job_output_folder = WhereTheJobOutputFolderIs::ChosenAtRenderStart {
            the_folders_name: crate::names::job_output_folder_name(&job, Some("Kitchen")),
            the_servers_fallback_folder_at_submission: Some("/data/renders".to_string()),
        };
        assert_eq!(
            chosen_later
                .copied_under(&new, Some("Kitchen"))
                .expect("it fits")
                .job_output_folder,
            WhereTheJobOutputFolderIs::ChosenAtRenderStart {
                the_folders_name: crate::names::job_output_folder_name(&new, Some("Kitchen")),
                the_servers_fallback_folder_at_submission: Some("/data/renders".to_string()),
            }
        );
        assert_eq!(
            original
                .copied_under(&new, None)
                .expect("it fits")
                .job_output_folder
                .the_path_it_names(),
            Some(
                format!(
                    "/ephemeral/smoozy/jobs/{}",
                    &new.as_text()[..crate::names::THE_IDENTITY_CHARACTERS_IN_A_FOLDER_NAME]
                )
                .as_str()
            ),
            "no label, the identity's short form alone"
        );
    }
}
