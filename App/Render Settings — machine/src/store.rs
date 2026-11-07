//! The override store — `Render Settings` §2.1 J1–J6, §2.3 J10's other
//! half, and §9.3 Y15–Y15a, what a history-opened page loads into it.
//!
//! **Constraint:** Rows 144 and 145 — *"That split is the whole rule: the
//! user's answers survive, the file's do not."* The two halves are the same
//! number on screen and different answers to the render; getting it wrong
//! sends the file's stale number as though the user had chosen it, at
//! success.
//!
//! **J2: the store is volatile — process memory for the life of the
//! application run — a member of `State, identity and recovery` §1.1's
//! volatile class.** *Closing Smoozy takes everything* is the spec's own
//! grant; nothing here is kept on disk, carried into another sitting, or
//! written into the local record, and the type carries no written form.
//!
//! **J4: nothing ever converts one kind into the other, and this is the
//! clause the whole page rests on.** A value the user has not touched has no
//! store entry — there is no materialise-on-focus, no copying a file value in
//! as a starting point. **Kept by the type**: no function in this file takes a
//! reading, so no path exists by which a reading's value could become an
//! entry. This is `State, identity and recovery` D2's discipline applied to
//! the page: the store is the desired half, the reading the observed half,
//! and the arrow between them points only through the user's own act.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rendering` A1, A1b, A1d, A4, S6. `Queue & History`
//! N10a. `File Browser` §12.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::device::DeviceChoice;
use crate::engine::OfferedEngine;
use crate::file_half::{BlenderIdentity, FileKey};
use crate::frames::{which_entry_composition_reads, FrameForm, WhichEntryCompositionReads};
use crate::names::{ExactName, NamedAnswer};
use crate::refusals::{clearing_the_engine_pick_against_the_standing_count, Refusal};

/// **J1: the display is composed from two objects, and neither is ever
/// written from the other.** For each setting the page shows the store's
/// entry where one exists (the user's, upright), else the standing reading's
/// value (the file's, italic), else the promise-form — *whatever the file
/// says*, not yet shown. Which face each takes is Design's; that they are two
/// objects is this type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shown<T> {
    TheUsers(T),
    TheFiles(T),
    PromiseForm,
}

pub fn shown<T>(the_users_entry: Option<T>, the_files_value: Option<T>) -> Shown<T> {
    match (the_users_entry, the_files_value) {
        (Some(users), _) => Shown::TheUsers(users),
        (None, Some(files)) => Shown::TheFiles(files),
        (None, None) => Shown::PromiseForm,
    }
}

/// **J3: the Simplify entries — the switch, and each of its three settings,
/// its own entry where the user answered it at `Rendering`'s failed-job
/// offer** (per-setting from 2026-08-29, Row 472 / that document's A1d).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SimplifyEntries {
    pub use_simplify: Option<bool>,
    pub subdivision_render: Option<u32>,
    pub child_particles_render: Option<f64>,
    pub texture_limit_render: Option<String>,
}

impl SimplifyEntries {
    pub fn is_empty(&self) -> bool {
        *self == SimplifyEntries::default()
    }
}

/// **A view-layer pin: per layer name with J7's provenance, on or off.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pin {
    pub layer: NamedAnswer,
    pub on: bool,
}

/// **J3's per-(server, file, scene) entries** — and, with no pins, the shape
/// of the pre-read answer set too.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SceneEntries {
    /// `Render Settings` Y1c: no answer means on.
    pub prepare_simulations: Option<bool>,
    pub frame_range: Option<FrameForm>,
    /// `Render Settings` F1: display only; never read for a number.
    pub frame_range_text: Option<String>,
    pub sample_count: Option<u64>,
    pub resolution_percentage: Option<u32>,
    /// One of the two offered.
    pub engine_pick: Option<OfferedEngine>,
    pub device: Option<DeviceChoice>,
    pub pins: Vec<Pin>,
    pub simplify: SimplifyEntries,
    /// **Y15b: where the range here came with a loaded setup, what stood here
    /// before the load.** `None` for a range the user typed, and for no range.
    /// It sits on the entries so that J3's re-key of the pre-read set carries
    /// it, and so that F1's accepted range and J5's cross end it by themselves.
    pub what_a_loaded_range_stands_on: Option<WhatStoodBeneath>,
}

/// **Y15b: the range and F1's display copy that stood where a load wrote**,
/// both empty where nothing did. What the page reads again once the loaded
/// range goes.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct WhatStoodBeneath {
    pub frame_range: Option<FrameForm>,
    pub frame_range_text: Option<String>,
}

/// One answer the user gave, in F1's parsed form or J11's accepted value —
/// **never text**, because the boxes refuse before anything reaches here.
#[derive(Debug, Clone, PartialEq)]
pub enum Answer {
    PrepareSimulations(bool),
    FrameRange(FrameForm),
    SampleCount(u64),
    ResolutionPercentage(u32),
    EnginePick(OfferedEngine),
    Device(DeviceChoice),
    SimplifySwitch(bool),
    SimplifySubdivision(u32),
    SimplifyChildParticles(f64),
    SimplifyTextureLimit(String),
}

/// **J5: the per-setting cross deletes that entry.** Which one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Setting {
    PrepareSimulations,
    FrameRange,
    SampleCount,
    ResolutionPercentage,
    EnginePick,
    Device,
    Pin(ExactName),
    SimplifySwitch,
    SimplifySubdivision,
    SimplifyChildParticles,
    SimplifyTextureLimit,
}

impl SceneEntries {
    fn take(&mut self, answer: Answer) {
        // **A fraction with no rendering never becomes a store entry.** Y2
        // puts the payload into the manifest's carried-and-not-interpreted
        // field, whose type is a JSON value, and a non-finite fraction has no
        // rendering there — so holding one would kill the whole store at the
        // composition. The store declines to hold what the payload cannot
        // carry. **This is not a fourth page refusal and is not written as
        // one**: J11 says the page refuses in exactly three places, and
        // `Rendering` A1d owns what the failed-job offer takes and the range
        // it holds its boxes to. Nothing is said to the user.
        if let Answer::SimplifyChildParticles(fraction) = answer {
            if !fraction.is_finite() {
                return;
            }
        }
        match answer {
            Answer::PrepareSimulations(on) => self.prepare_simulations = Some(on),
            Answer::FrameRange(form) => {
                self.frame_range = Some(form);
                self.frame_range_text = None;
                self.what_a_loaded_range_stands_on = None;
            },
            Answer::SampleCount(count) => self.sample_count = Some(count),
            Answer::ResolutionPercentage(percentage) => {
                self.resolution_percentage = Some(percentage)
            }
            Answer::EnginePick(pick) => self.engine_pick = Some(pick),
            Answer::Device(choice) => self.device = Some(choice),
            Answer::SimplifySwitch(on) => self.simplify.use_simplify = Some(on),
            Answer::SimplifySubdivision(value) => self.simplify.subdivision_render = Some(value),
            Answer::SimplifyChildParticles(value) => {
                self.simplify.child_particles_render = Some(value)
            }
            Answer::SimplifyTextureLimit(value) => self.simplify.texture_limit_render = Some(value),
        }
    }

    /// **J5: clearing is deletion.** There is no *cleared* marker: an entry
    /// either exists or does not.
    fn delete(&mut self, setting: &Setting) {
        match setting {
            Setting::PrepareSimulations => self.prepare_simulations = None,
            Setting::FrameRange => {
                self.frame_range = None;
                self.frame_range_text = None;
                self.what_a_loaded_range_stands_on = None;
            },
            Setting::SampleCount => self.sample_count = None,
            Setting::ResolutionPercentage => self.resolution_percentage = None,
            Setting::EnginePick => self.engine_pick = None,
            Setting::Device => self.device = None,
            Setting::Pin(layer) => self
                .pins
                .retain(|pin| pin.layer.name.as_bytes() != layer.as_bytes()),
            Setting::SimplifySwitch => self.simplify.use_simplify = None,
            Setting::SimplifySubdivision => self.simplify.subdivision_render = None,
            Setting::SimplifyChildParticles => self.simplify.child_particles_render = None,
            Setting::SimplifyTextureLimit => self.simplify.texture_limit_render = None,
        }
    }

    /// **J11 item 3's second direction, guarded before the deletion happens.**
    /// Clearing the engine pick moves the engine in force from the picked
    /// engine to the file's own, and the range the box holds to follows the
    /// pick — so where a count above the no-pick ceiling stands, the cross
    /// does not take effect and the pick goes on standing.
    ///
    /// **No other route out of the store needs the guard.** The
    /// hand-everything-back control deletes the count in the same act as the
    /// pick, so it leaves no count behind; and J5's deletion is the only route
    /// out of the store there is.
    fn delete_or_refuse(&mut self, setting: &Setting) -> Result<(), Refusal> {
        if matches!(setting, Setting::EnginePick) {
            clearing_the_engine_pick_against_the_standing_count(self.sample_count)?;
        }
        self.delete(setting);
        Ok(())
    }

    /// **Y15b: what a load here would have to give back** — a record already
    /// standing, so a second load gives back what the user had and never the
    /// first load's range; else the range as it stands.
    fn what_stands_beneath_a_load(&self) -> WhatStoodBeneath {
        self.what_a_loaded_range_stands_on.clone().unwrap_or_else(|| WhatStoodBeneath {
            frame_range: self.frame_range.clone(),
            frame_range_text: self.frame_range_text.clone(),
        })
    }

    fn pin(&mut self, layer: NamedAnswer, on: bool) {
        self.pins
            .retain(|pin| pin.layer.name.as_bytes() != layer.name.as_bytes());
        self.pins.push(Pin { layer, on });
    }
}

/// **J3's per-(server, file) entries**, with the per-scene entries beneath
/// them.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PerFile {
    /// The scenes ticked, by name with J7's provenance.
    pub scenes_ticked: Vec<NamedAnswer>,
    /// The version pick override, where the user picked a build against the
    /// automatic match.
    pub version_pick: Option<BlenderIdentity>,
    /// **The group frame range — the one entry that stands only while more
    /// than one scene is ticked.** While it stands it shadows the per-scene
    /// frame-range entries of this file, which are neither read nor written.
    pub group_frame_range: Option<FrameForm>,
    /// F1's display copy of the accepted group range.
    pub group_frame_range_text: Option<String>,
    /// **The pre-read answer set** — the settings answerable with nothing
    /// read, while no scene is in hand. A scene tick and a view-layer pin
    /// each need a name only a reading or a kept setup supplies, so neither
    /// can exist in this set.
    pub pre_read: Option<SceneEntries>,
    /// Per scene, by name.
    pub per_scene: HashMap<ExactName, SceneEntries>,
}

/// Where an answer is given — a scene's own key, or the pre-read set of a
/// page with no scene in hand.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Where {
    AScene { file: FileKey, scene: ExactName },
    ThePreReadSet { file: FileKey },
}

/// **J5: what the hand-everything-back control deletes — the entries the
/// page is showing.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PageInHand {
    /// The ticked scene's own per-scene entries.
    OneScene(ExactName),
    /// The group frame range, where more than one scene is ticked.
    AGroup,
    /// The pre-read answer set, where no scene is in hand.
    NoSceneInHand,
}

/// **The group range stands only while more than one scene is ticked**, so an
/// answer to it while one or none is ticked has nowhere to go.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotAGroup;

/// **The override store.** Keyed three ways at once (J3): per server row, per
/// file — the file's path on that server, deliberately not its four-value
/// identity, because the spec has the user's answers survive the file being
/// replaced — and per scene, by name.
///
/// **J6: switching scenes, files and servers is a change of key, not a change
/// of contents.** Nothing is saved on leaving or restored on returning; the
/// entries were never keyed to the visit.
#[derive(Debug, Default)]
pub struct Store {
    files: HashMap<FileKey, PerFile>,
}

impl Store {
    pub fn new() -> Store {
        Store::default()
    }

    pub fn per_file(&self, file: &FileKey) -> Option<&PerFile> {
        self.files.get(file)
    }

    pub fn entries_for(&self, file: &FileKey, scene: &ExactName) -> Option<&SceneEntries> {
        self.files.get(file)?.per_scene.get(scene)
    }

    fn file(&mut self, file: FileKey) -> &mut PerFile {
        self.files.entry(file).or_default()
    }

    /// **An answer is a store entry — the store records the act of answering,
    /// not a difference against a reading** (J4). A value the user typed that
    /// happens to equal the file's is an ordinary entry.
    pub fn answer(&mut self, at: Where, answer: Answer) {
        match at {
            Where::AScene { file, scene } => self
                .file(file)
                .per_scene
                .entry(scene)
                .or_default()
                .take(answer),
            Where::ThePreReadSet { file } => self
                .file(file)
                .pre_read
                .get_or_insert_with(SceneEntries::default)
                .take(answer),
        }
    }

    fn entries_at(&mut self, at: Where) -> &mut SceneEntries {
        match at {
            Where::AScene { file, scene } => self.file(file).per_scene.entry(scene).or_default(),
            Where::ThePreReadSet { file } => self.file(file).pre_read.get_or_insert_with(SceneEntries::default),
        }
    }

    /// F1: an accepted range and its display copy are replaced together.
    /// **Y15b: a range the user typed is theirs**, whatever stood there.
    pub fn answer_the_typed_range(&mut self, at: Where, form: FrameForm, text: &str) {
        let entries = self.entries_at(at);
        entries.frame_range = Some(form);
        entries.frame_range_text = Some(text.trim().to_string());
        entries.what_a_loaded_range_stands_on = None;
    }

    /// **Y15b: `Rendering` A4's range, written over what the load brought**:
    /// the load's own record goes on standing under it.
    pub fn a_range_is_loaded(&mut self, at: Where, form: FrameForm) {
        let entries = self.entries_at(at);
        let beneath = entries.what_stands_beneath_a_load();
        entries.frame_range = Some(form);
        entries.frame_range_text = None;
        entries.what_a_loaded_range_stands_on = Some(beneath);
    }

    /// **Y15b, for a load that writes the pre-read set answer by answer**
    /// (`Rendering`'s load of an entry that named no scene): what stands in the
    /// set before it clears it.
    pub fn what_stands_beneath_a_load_of_the_pre_read_set(&self, file: &FileKey) -> WhatStoodBeneath {
        self.files
            .get(file)
            .and_then(|held| held.pre_read.as_ref())
            .map(SceneEntries::what_stands_beneath_a_load)
            .unwrap_or_default()
    }

    /// **Y15b: that load's record, written once its answers are in.** A load
    /// that left no range and took none away has nothing to record.
    pub fn the_pre_read_set_was_loaded(&mut self, file: FileKey, beneath: WhatStoodBeneath) {
        let there_was_one = beneath != WhatStoodBeneath::default();
        let Some(set) = self.file(file).pre_read.as_mut() else {
            return;
        };
        if there_was_one || set.frame_range.is_some() {
            set.what_a_loaded_range_stands_on = Some(beneath);
        }
    }

    /// **Y15b: the one job went, or the page was left.** Every entry of this
    /// (server, file) a load wrote a range into reads what stood there before.
    /// A range typed since has no record and is not touched.
    pub fn a_loaded_range_goes(&mut self, file: &FileKey) {
        let Some(held) = self.files.get_mut(file) else {
            return;
        };
        for entries in held.pre_read.iter_mut().chain(held.per_scene.values_mut()) {
            if let Some(beneath) = entries.what_a_loaded_range_stands_on.take() {
                entries.frame_range = beneath.frame_range;
                entries.frame_range_text = beneath.frame_range_text;
            }
        }
    }

    /// F1 and J3: the group display copy shadows the scene's copy too.
    pub fn answer_the_typed_group_range(&mut self, file: FileKey, form: FrameForm, text: &str) -> Result<(), NotAGroup> {
        self.answer_the_group_range(file.clone(), form)?;
        self.file(file).group_frame_range_text = Some(text.trim().to_string());
        Ok(())
    }

    /// **A pin needs a name only a reading or a kept setup supplies** (J7), so
    /// it is keyed by scene and never by the pre-read set.
    pub fn pin(&mut self, file: FileKey, scene: ExactName, layer: NamedAnswer, on: bool) {
        self.file(file)
            .per_scene
            .entry(scene)
            .or_default()
            .pin(layer, on);
    }

    /// **J5: the per-setting cross**, refused where J11 item 3's collision
    /// says the deletion must not happen. Every other cross always succeeds.
    pub fn clear_one(&mut self, at: Where, setting: &Setting) -> Result<(), Refusal> {
        match at {
            Where::AScene { file, scene } => match self.file(file).per_scene.get_mut(&scene) {
                Some(entries) => entries.delete_or_refuse(setting),
                None => Ok(()),
            },
            Where::ThePreReadSet { file } => match self.file(file).pre_read.as_mut() {
                Some(set) => set.delete_or_refuse(setting),
                None => Ok(()),
            },
        }
    }

    /// **J3: a scene ticked, by name with its provenance.**
    pub fn tick_a_scene(&mut self, file: FileKey, scene: NamedAnswer) {
        let held = self.file(file);
        if !held
            .scenes_ticked
            .iter()
            .any(|ticked| ticked.name.as_bytes() == scene.name.as_bytes())
        {
            held.scenes_ticked.push(scene);
        }
    }

    /// **Unticking back to one scene leaves the remaining scene's own entry
    /// exactly as it was**, since the group entry was never written into any
    /// scene's key — and the group entry, which stands only while more than
    /// one scene is ticked, goes with the second tick.
    pub fn untick_a_scene(&mut self, file: FileKey, scene: &ExactName) {
        let held = self.file(file);
        held.scenes_ticked
            .retain(|ticked| ticked.name.as_bytes() != scene.as_bytes());
        if held.scenes_ticked.len() <= 1 {
            held.group_frame_range = None;
            held.group_frame_range_text = None;
        }
    }

    /// **The group frame range, while more than one scene is ticked.**
    pub fn answer_the_group_range(
        &mut self,
        file: FileKey,
        form: FrameForm,
    ) -> Result<(), NotAGroup> {
        let held = self.file(file);
        if held.scenes_ticked.len() <= 1 {
            return Err(NotAGroup);
        }
        held.group_frame_range = Some(form);
        held.group_frame_range_text = None;
        Ok(())
    }

    pub fn pick_a_version(&mut self, file: FileKey, pick: BlenderIdentity) {
        self.file(file).version_pick = Some(pick);
    }

    /// Clearing the version returns to the automatic match and never to a
    /// value out of the file (Y15).
    pub fn clear_the_version_pick(&mut self, file: FileKey) {
        self.file(file).version_pick = None;
    }

    /// **J5: the hand-everything-back control deletes the entries the page
    /// is showing, plus the version override** — the one entry that is not
    /// per scene and which the spec names in that control's own sentence.
    /// **Scenes the page is not showing keep their entries**, and it touches
    /// neither the scenes ticked nor anything in any reading.
    pub fn hand_everything_back(&mut self, file: FileKey, page: PageInHand) {
        let held = self.file(file);
        match page {
            PageInHand::OneScene(scene) => {
                held.per_scene.remove(&scene);
            }
            PageInHand::AGroup => {
                // `Render Settings` Y1c: reset the simulation choices shown for this group.
                for ticked in &held.scenes_ticked {
                    if let Some(entries) = held.per_scene.get_mut(&ticked.name) {
                        entries.prepare_simulations = None;
                    }
                }
                held.group_frame_range = None;
                held.group_frame_range_text = None;
            },
            PageInHand::NoSceneInHand => held.pre_read = None,
        }
        held.version_pick = None;
    }

    /// **J3's last sentence: when a reading first lands for that (server,
    /// file), the pre-read set's entries become the per-scene entries of the
    /// scene the page then stands in, in one act — re-keyed, never copied** —
    /// so the answer goes on standing in the box the user typed it into.
    /// From that moment ordinary per-scene keying governs, and the set does
    /// not re-form while the run holds any scene in hand.
    pub fn a_reading_first_landed(
        &mut self,
        file: FileKey,
        the_scene_the_page_stands_in: ExactName,
    ) {
        let held = self.file(file);
        if let Some(set) = held.pre_read.take() {
            // The set shadows nothing and holds no pins, so the re-key is the
            // move of the set itself; nothing merges over an entry that could
            // not exist yet under a key that did not.
            held.per_scene.insert(the_scene_the_page_stands_in, set);
        }
    }

    /// **F5: the answers a job's payload composition reads**, for the scene in
    /// hand — its own per-scene entries, with **the group entry of J3 in the
    /// range's place where it stands** (more than one scene ticked at
    /// submission); or, where no scene is in hand, the pre-read answer set.
    /// Where none of those stands there is no entry, and the job's frames are
    /// the file's own by F4.
    ///
    /// **Which of the three it is, is asked of F5's own function** rather than
    /// decided again here: [`which_entry_composition_reads`] is F5 written
    /// out, and the two halves drifted apart once already because it had no
    /// caller.
    ///
    /// **The group entry shadows the per-scene range while more than one scene
    /// is ticked, and the shadow is worded on the ticked count and not on the
    /// group entry existing** (J3): they are neither read nor written there.
    /// So with several scenes ticked and no group range typed, the range is
    /// absent and the file's own stands by F4 — never a range typed while one
    /// scene was ticked, which the page is not showing and nobody would be
    /// told about.
    pub fn the_answers_composition_reads(
        &self,
        file: &FileKey,
        scene_in_hand: Option<&ExactName>,
    ) -> SceneEntries {
        let Some(held) = self.files.get(file) else {
            return SceneEntries::default();
        };
        let the_scenes_own = || {
            scene_in_hand
                .and_then(|scene| held.per_scene.get(scene))
                .cloned()
                .unwrap_or_default()
        };
        match which_entry_composition_reads(held.scenes_ticked.len(), scene_in_hand.is_some()) {
            WhichEntryCompositionReads::TheGroupEntry => {
                let mut answers = the_scenes_own();
                answers.frame_range = held.group_frame_range.clone();
                answers.frame_range_text = held.group_frame_range_text.clone();
                answers
            }
            WhichEntryCompositionReads::TheScenesOwn => the_scenes_own(),
            WhichEntryCompositionReads::ThePreReadAnswerSet => {
                held.pre_read.clone().unwrap_or_default()
            }
        }
    }
}

/// **Y15: what a history-opened page loads into the store is the kept setup's
/// entries that were the user's own answers, and nothing else.** `Rendering`
/// A1 fills the page from the history entry's kept setup, and that setup is a
/// payload — so it carries entries that were never anybody's answer, the
/// resolved version pick above all, which B4 puts on **every** payload.
#[derive(Debug, Clone, PartialEq)]
pub struct KeptSetup {
    /// The kept scene name — J7's second naming act — under which the
    /// scene-keyed entries are written.
    pub scene: NamedAnswer,
    pub entries: SceneEntries,
    pub version: KeptVersion,
}

/// **The override-or-automatic distinction on the entry**, which `Queue &
/// History` N10a keeps as a field of the record, written at submission from
/// which side supplied the value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeptVersion {
    /// The entry records the pick as the user's own override, and it is
    /// loaded.
    TheUsersOverride(BlenderIdentity),
    /// B4's resolved-always field: **not loaded** — the automatic match runs
    /// afresh for the machine in hand (B2), which keeps a pick made for a
    /// machine that no longer exists from arriving as an answer the user
    /// never gave.
    ResolvedAutomatically,
}

/// **Y15a: the kept setup's Simplify entries are loaded only where the page
/// draws `Rendering`'s offer**, which that document's A1b makes the history
/// entry the page was opened from having ended *failed*. Where it is not
/// drawn they are not written into the store and not held by the page; with
/// no store entry, Y1's Simplify row is composed for no such job.
pub fn what_the_load_carries(kept: &KeptSetup, the_offer_is_drawn: bool) -> SceneEntries {
    let mut entries = kept.entries.clone();
    if !the_offer_is_drawn {
        entries.simplify = SimplifyEntries::default();
    }
    entries
}

impl Store {
    /// **Y15, under the key — where the entry's machine still stands.** The
    /// key is the entry's own, and the kept setup's entries are written into
    /// the store under it at the open, the scene-keyed ones under the kept
    /// scene name. The kept scene is ticked with its kept-setup provenance,
    /// so the job the page re-submits carries that job's scene name (Y10).
    pub fn load_a_kept_setup(&mut self, file: FileKey, kept: KeptSetup, the_offer_is_drawn: bool) {
        let mut entries = what_the_load_carries(&kept, the_offer_is_drawn);
        let held = self.file(file);
        // **Y15b: what stood under this load, kept with what it writes**, a
        // load that brings no range included: it takes a typed one away too.
        let beneath = held
            .per_scene
            .get(&kept.scene.name)
            .map(SceneEntries::what_stands_beneath_a_load)
            .unwrap_or_default();
        entries.what_a_loaded_range_stands_on =
            (entries.frame_range.is_some() || beneath != WhatStoodBeneath::default()).then_some(beneath);
        held.per_scene.insert(kept.scene.name.clone(), entries);
        held.scenes_ticked = vec![kept.scene];
        held.version_pick = match kept.version {
            KeptVersion::TheUsersOverride(pick) => Some(pick),
            KeptVersion::ResolvedAutomatically => None,
        };
    }

    /// **Y15's no-key-yet route: the loaded setup is held by the page,
    /// outside the store, and written into the store in one act at the moment
    /// the user's pick lands and the key first exists.** `File Browser` §12's
    /// *a file at a new path starts clean* is a statement about prior entries
    /// and never a wipe of this write.
    pub fn write_the_held_setup_at_the_pick(&mut self, file: FileKey, held: HeldSetup) {
        self.load_a_kept_setup(file, held.working_set, held.the_offer_is_drawn);
    }
}

/// **The held setup of a page with no (server row, file path) yet — the
/// page's working set rather than a frozen copy of the entry's.** A fresh
/// answer given while the page is keyless joins the held setup and rides the
/// same one write; the per-setting cross and the hand-everything-back
/// control act on the set in hand; and the one write at the pick carries the
/// set as it stands at the pick.
#[derive(Debug, Clone, PartialEq)]
pub struct HeldSetup {
    working_set: KeptSetup,
    the_offer_is_drawn: bool,
}

impl HeldSetup {
    pub fn from_the_entry(kept: KeptSetup, the_offer_is_drawn: bool) -> HeldSetup {
        let mut working_set = kept;
        working_set.entries = what_the_load_carries(&working_set, the_offer_is_drawn);
        HeldSetup {
            working_set,
            the_offer_is_drawn,
        }
    }

    pub fn working_set(&self) -> &KeptSetup {
        &self.working_set
    }

    /// An answer the page accepts replaces the held entry for that setting or
    /// stands beside the others as a new one.
    pub fn answer(&mut self, answer: Answer) {
        self.working_set.entries.take(answer);
    }

    pub fn clear_one(&mut self, setting: &Setting) -> Result<(), Refusal> {
        self.working_set.entries.delete_or_refuse(setting)
    }

    /// J5's hand-everything-back, performed on the set in hand: the entries
    /// and the version override go; the kept scene name stays, because the
    /// control touches the scenes ticked nowhere.
    pub fn hand_everything_back(&mut self) {
        self.working_set.entries = SceneEntries::default();
        self.working_set.version = KeptVersion::ResolvedAutomatically;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_half::tests::{a_blender, a_key, a_server};
    use crate::frames::parse_the_frame_field;
    use crate::names::Provenance;

    fn named(name: &str) -> NamedAnswer {
        NamedAnswer {
            name: ExactName::as_the_reading_produced_it(name),
            provenance: Provenance::AKeptSetup {
                the_old_job: "job-1".to_string(),
            },
        }
    }

    fn kitchen() -> ExactName {
        ExactName::as_the_reading_produced_it("Kitchen")
    }

    fn at_kitchen(file: &FileKey) -> Where {
        Where::AScene {
            file: file.clone(),
            scene: kitchen(),
        }
    }

    /// **Y15b: a loaded range belongs to one job.** A load over nothing, and
    /// `Rendering` A4's range over it, read the file's own again once it goes;
    /// over a typed range, that range and its text come back. **Undone (the
    /// range left where the load wrote it), this fails.**
    #[test]
    fn a_loaded_range_goes_and_the_page_reads_what_stood_before() {
        let file = a_key(&a_server());
        let setup = |frames: Option<FrameForm>| KeptSetup {
            scene: named("Kitchen"),
            entries: SceneEntries { frame_range: frames, sample_count: Some(64), ..SceneEntries::default() },
            version: KeptVersion::ResolvedAutomatically,
        };
        let range = |first, last| FrameForm::Triple { first, last, step: 1 };

        // **Over nothing: a *Resume*, the kept range then A4's.**
        let mut store = Store::new();
        store.load_a_kept_setup(file.clone(), setup(Some(range(1, 40))), false);
        store.a_range_is_loaded(at_kitchen(&file), range(11, 40));
        assert_eq!(store.entries_for(&file, &kitchen()).unwrap().frame_range, Some(range(11, 40)));
        store.a_loaded_range_goes(&file);
        let after = store.entries_for(&file, &kitchen()).unwrap();
        assert_eq!(after.frame_range, None, "the file's own range stands again");
        assert_eq!(after.what_a_loaded_range_stands_on, None);
        assert_eq!(after.sample_count, Some(64), "every other answer stays as Y15 has it");

        // **Over a typed range, twice over**: what comes back is what the user had.
        let mut store = Store::new();
        store.answer_the_typed_range(at_kitchen(&file), range(5, 9), "5-9");
        store.load_a_kept_setup(file.clone(), setup(Some(range(1, 40))), false);
        store.load_a_kept_setup(file.clone(), setup(Some(range(2, 30))), false);
        store.a_range_is_loaded(at_kitchen(&file), range(11, 30));
        store.a_loaded_range_goes(&file);
        let after = store.entries_for(&file, &kitchen()).unwrap();
        assert_eq!(after.frame_range, Some(range(5, 9)));
        assert_eq!(after.frame_range_text.as_deref(), Some("5-9"));

        // **A load that brings no range took the typed one away, and gives it back.**
        let mut store = Store::new();
        store.answer_the_typed_range(at_kitchen(&file), range(5, 9), "5-9");
        store.load_a_kept_setup(file.clone(), setup(None), false);
        assert_eq!(store.entries_for(&file, &kitchen()).unwrap().frame_range, None);
        store.a_loaded_range_goes(&file);
        assert_eq!(store.entries_for(&file, &kitchen()).unwrap().frame_range, Some(range(5, 9)));
    }

    /// **Y15b: a range the user typed stays, and a cross stays cleared.** Each
    /// of F1's two writes and J5's cross ends the record. **Undone, the load's
    /// going takes the typed range with it, and this fails.**
    #[test]
    fn a_range_typed_after_a_load_stays_when_the_load_goes() {
        let file = a_key(&a_server());
        let setup = KeptSetup {
            scene: named("Kitchen"),
            entries: SceneEntries {
                frame_range: Some(FrameForm::Triple { first: 1, last: 40, step: 1 }),
                ..SceneEntries::default()
            },
            version: KeptVersion::ResolvedAutomatically,
        };
        let typed = FrameForm::Triple { first: 3, last: 4, step: 1 };

        let mut store = Store::new();
        store.load_a_kept_setup(file.clone(), setup.clone(), false);
        store.answer_the_typed_range(at_kitchen(&file), typed.clone(), "3-4");
        store.a_loaded_range_goes(&file);
        let after = store.entries_for(&file, &kitchen()).unwrap();
        assert_eq!(after.frame_range, Some(typed.clone()));
        assert_eq!(after.frame_range_text.as_deref(), Some("3-4"));

        let mut store = Store::new();
        store.load_a_kept_setup(file.clone(), setup.clone(), false);
        store.answer(at_kitchen(&file), Answer::FrameRange(typed.clone()));
        store.a_loaded_range_goes(&file);
        assert_eq!(store.entries_for(&file, &kitchen()).unwrap().frame_range, Some(typed));

        let mut store = Store::new();
        store.answer_the_typed_range(at_kitchen(&file), FrameForm::Triple { first: 5, last: 9, step: 1 }, "5-9");
        store.load_a_kept_setup(file.clone(), setup, false);
        store.clear_one(at_kitchen(&file), &Setting::FrameRange).unwrap();
        store.a_loaded_range_goes(&file);
        assert_eq!(store.entries_for(&file, &kitchen()).unwrap().frame_range, None, "the cross was the user's");
    }

    /// **Y15b: the record travels with J3's re-key**, so an entry that named no
    /// scene gives its range back after a read has landed as well as before.
    #[test]
    fn a_loaded_range_in_the_pre_read_set_goes_after_the_rekey_too() {
        let file = a_key(&a_server());
        let the_set = Where::ThePreReadSet { file: file.clone() };
        let range = FrameForm::Triple { first: 11, last: 40, step: 1 };
        let mut store = Store::new();
        let beneath = store.what_stands_beneath_a_load_of_the_pre_read_set(&file);
        store.answer(the_set.clone(), Answer::FrameRange(range.clone()));
        store.the_pre_read_set_was_loaded(file.clone(), beneath);
        store.a_reading_first_landed(file.clone(), kitchen());
        assert_eq!(store.entries_for(&file, &kitchen()).unwrap().frame_range, Some(range.clone()));
        store.a_loaded_range_goes(&file);
        assert_eq!(store.entries_for(&file, &kitchen()).unwrap().frame_range, None);

        let mut store = Store::new();
        let beneath = store.what_stands_beneath_a_load_of_the_pre_read_set(&file);
        store.answer(the_set, Answer::FrameRange(range));
        store.the_pre_read_set_was_loaded(file.clone(), beneath);
        store.a_loaded_range_goes(&file);
        assert_eq!(store.per_file(&file).unwrap().pre_read.as_ref().unwrap().frame_range, None);
    }

    #[test]
    fn the_display_shows_the_users_entry_else_the_files_value_else_the_promise_form() {
        assert_eq!(shown(Some(50), Some(100)), Shown::TheUsers(50));
        assert_eq!(shown(None, Some(100)), Shown::TheFiles(100));
        assert_eq!(shown::<u32>(None, None), Shown::PromiseForm);
    }

    #[test]
    fn an_untouched_setting_has_no_entry_and_a_typed_one_is_an_entry_whatever_the_file_says() {
        let server = a_server();
        let file = a_key(&server);
        let mut store = Store::new();
        assert!(store.entries_for(&file, &kitchen()).is_none());
        store.answer(at_kitchen(&file), Answer::ResolutionPercentage(100));
        assert_eq!(
            store
                .entries_for(&file, &kitchen())
                .unwrap()
                .resolution_percentage,
            Some(100)
        );
        assert_eq!(
            store.entries_for(&file, &kitchen()).unwrap().sample_count,
            None
        );
    }

    #[test]
    fn clearing_is_deletion_and_there_is_no_cleared_marker() {
        let server = a_server();
        let file = a_key(&server);
        let mut store = Store::new();
        store.answer(at_kitchen(&file), Answer::SampleCount(128));
        assert_eq!(
            store.clear_one(at_kitchen(&file), &Setting::SampleCount),
            Ok(())
        );
        assert_eq!(
            store.entries_for(&file, &kitchen()).unwrap().sample_count,
            None
        );
        // A pin's cross deletes that pin alone.
        store.pin(file.clone(), kitchen(), named("A"), true);
        store.pin(file.clone(), kitchen(), named("B"), false);
        let _ = store.clear_one(
            at_kitchen(&file),
            &Setting::Pin(ExactName::as_the_reading_produced_it("A")),
        );
        let pins = &store.entries_for(&file, &kitchen()).unwrap().pins;
        assert_eq!(pins.len(), 1);
        assert_eq!(pins[0].layer.name.as_text(), "B");
    }

    #[test]
    fn a_pin_re_answered_replaces_the_pin_for_that_layer_rather_than_adding_a_second() {
        let server = a_server();
        let file = a_key(&server);
        let mut store = Store::new();
        store.pin(file.clone(), kitchen(), named("A"), true);
        store.pin(file.clone(), kitchen(), named("A"), false);
        let pins = &store.entries_for(&file, &kitchen()).unwrap().pins;
        assert_eq!(pins.len(), 1);
        assert!(!pins[0].on);
    }

    #[test]
    fn hand_everything_back_clears_the_page_in_hand_and_the_version_override_and_nothing_else() {
        let server = a_server();
        let file = a_key(&server);
        let mut store = Store::new();
        store.answer(at_kitchen(&file), Answer::SampleCount(128));
        store.answer(
            Where::AScene {
                file: file.clone(),
                scene: ExactName::as_the_reading_produced_it("Exterior"),
            },
            Answer::SampleCount(64),
        );
        store.tick_a_scene(file.clone(), named("Kitchen"));
        store.pick_a_version(file.clone(), a_blender());

        store.hand_everything_back(file.clone(), PageInHand::OneScene(kitchen()));
        let held = store.per_file(&file).unwrap();
        assert!(!held.per_scene.contains_key(&kitchen()));
        // Scenes the page is not showing keep their entries.
        assert_eq!(
            held.per_scene
                .get(&ExactName::as_the_reading_produced_it("Exterior"))
                .unwrap()
                .sample_count,
            Some(64)
        );
        // The version override goes; the scenes ticked do not.
        assert_eq!(held.version_pick, None);
        assert_eq!(held.scenes_ticked.len(), 1);
    }

    #[test]
    fn the_group_range_stands_only_while_more_than_one_scene_is_ticked_and_shadows_the_scenes_own()
    {
        let server = a_server();
        let file = a_key(&server);
        let mut store = Store::new();
        let own = parse_the_frame_field("100-200").unwrap();
        let group = parse_the_frame_field("1-10").unwrap();
        store.answer(at_kitchen(&file), Answer::FrameRange(own.clone()));
        store.tick_a_scene(file.clone(), named("Kitchen"));
        assert_eq!(
            store.answer_the_group_range(file.clone(), group.clone()),
            Err(NotAGroup)
        );
        store.tick_a_scene(file.clone(), named("Exterior"));
        assert_eq!(
            store.answer_the_group_range(file.clone(), group.clone()),
            Ok(())
        );

        // Composition reads the group entry in the range's place.
        let answers = store.the_answers_composition_reads(&file, Some(&kitchen()));
        assert_eq!(answers.frame_range, Some(group));
        // And the scene's own entry was never written over.
        assert_eq!(
            store.entries_for(&file, &kitchen()).unwrap().frame_range,
            Some(own.clone())
        );

        // Unticking back to one scene: the group entry goes, the scene's own
        // stands exactly as it was.
        store.untick_a_scene(
            file.clone(),
            &ExactName::as_the_reading_produced_it("Exterior"),
        );
        assert_eq!(store.per_file(&file).unwrap().group_frame_range, None);
        assert_eq!(
            store
                .the_answers_composition_reads(&file, Some(&kitchen()))
                .frame_range,
            Some(own)
        );
    }

    #[test]
    fn a_second_scene_ticked_after_a_range_was_typed_composes_no_range_at_all() {
        // J3: the group entry shadows the per-scene entries **while more than
        // one scene is ticked**, whether or not a group range was ever typed.
        // The page shows the file's own range in that state, so a range typed
        // while one scene was ticked must not ride the submission behind it.
        let server = a_server();
        let file = a_key(&server);
        let mut store = Store::new();
        let own = parse_the_frame_field("1-1000").unwrap();
        store.answer(at_kitchen(&file), Answer::FrameRange(own.clone()));
        store.tick_a_scene(file.clone(), named("Kitchen"));
        store.tick_a_scene(file.clone(), named("Exterior"));

        let answers = store.the_answers_composition_reads(&file, Some(&kitchen()));
        assert_eq!(answers.frame_range, None);
        // The scene's own entry is not touched, only shadowed — unticking
        // brings it straight back.
        assert_eq!(
            store.entries_for(&file, &kitchen()).unwrap().frame_range,
            Some(own.clone())
        );
        store.untick_a_scene(
            file.clone(),
            &ExactName::as_the_reading_produced_it("Exterior"),
        );
        assert_eq!(
            store
                .the_answers_composition_reads(&file, Some(&kitchen()))
                .frame_range,
            Some(own)
        );
    }

    #[test]
    fn clearing_the_engine_pick_is_refused_while_a_count_above_the_no_pick_ceiling_stands() {
        let server = a_server();
        let file = a_key(&server);
        let mut store = Store::new();
        // An EEVEE pick let this count in.
        store.answer(at_kitchen(&file), Answer::SampleCount(20_000_000));
        store.answer(at_kitchen(&file), Answer::EnginePick(OfferedEngine::Eevee));

        assert!(store
            .clear_one(at_kitchen(&file), &Setting::EnginePick)
            .is_err());
        // The deletion did not happen: the pick goes on standing.
        assert_eq!(
            store.entries_for(&file, &kitchen()).unwrap().engine_pick,
            Some(OfferedEngine::Eevee)
        );

        // Retyping the count is what lets the clear through.
        store.answer(at_kitchen(&file), Answer::SampleCount(128));
        assert_eq!(
            store.clear_one(at_kitchen(&file), &Setting::EnginePick),
            Ok(())
        );
        assert_eq!(
            store.entries_for(&file, &kitchen()).unwrap().engine_pick,
            None
        );

        // The same guard on the pre-read set, and on the held setup of a
        // keyless page.
        let mut store = Store::new();
        let at_the_set = Where::ThePreReadSet { file: file.clone() };
        store.answer(at_the_set.clone(), Answer::SampleCount(20_000_000));
        store.answer(at_the_set.clone(), Answer::EnginePick(OfferedEngine::Eevee));
        assert!(store.clear_one(at_the_set, &Setting::EnginePick).is_err());

        let mut held = HeldSetup::from_the_entry(
            KeptSetup {
                scene: named("Kitchen"),
                entries: SceneEntries::default(),
                version: KeptVersion::ResolvedAutomatically,
            },
            false,
        );
        held.answer(Answer::SampleCount(20_000_000));
        held.answer(Answer::EnginePick(OfferedEngine::Eevee));
        assert!(held.clear_one(&Setting::EnginePick).is_err());

        // Hand-everything-back needs no guard: it deletes the count in the
        // same act, so no count is left standing behind the cleared pick.
        held.hand_everything_back();
        assert_eq!(held.working_set().entries, SceneEntries::default());
    }

    #[test]
    fn a_simplify_fraction_the_payload_could_not_carry_never_becomes_an_entry() {
        let server = a_server();
        let file = a_key(&server);
        let mut store = Store::new();
        for cannot_be_carried in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            store.answer(
                at_kitchen(&file),
                Answer::SimplifyChildParticles(cannot_be_carried),
            );
        }
        assert_eq!(
            store
                .entries_for(&file, &kitchen())
                .unwrap()
                .simplify
                .child_particles_render,
            None
        );
        // An ordinary fraction is an ordinary entry.
        store.answer(at_kitchen(&file), Answer::SimplifyChildParticles(0.5));
        assert_eq!(
            store
                .entries_for(&file, &kitchen())
                .unwrap()
                .simplify
                .child_particles_render,
            Some(0.5)
        );
        // And a non-finite one after it does not overwrite it either.
        store.answer(at_kitchen(&file), Answer::SimplifyChildParticles(f64::NAN));
        assert_eq!(
            store
                .entries_for(&file, &kitchen())
                .unwrap()
                .simplify
                .child_particles_render,
            Some(0.5)
        );
    }

    #[test]
    fn the_pre_read_set_holds_an_answer_before_its_key_exists_and_is_re_keyed_when_a_reading_lands()
    {
        let server = a_server();
        let file = a_key(&server);
        let mut store = Store::new();
        let range = parse_the_frame_field("1-5").unwrap();
        store.answer(
            Where::ThePreReadSet { file: file.clone() },
            Answer::FrameRange(range.clone()),
        );
        store.answer(
            Where::ThePreReadSet { file: file.clone() },
            Answer::Device(DeviceChoice::TheProcessor),
        );
        // Composition for a job with no scene in hand reads the set.
        let answers = store.the_answers_composition_reads(&file, None);
        assert_eq!(answers.frame_range, Some(range.clone()));
        assert_eq!(answers.device, Some(DeviceChoice::TheProcessor));

        store.a_reading_first_landed(file.clone(), kitchen());
        let held = store.per_file(&file).unwrap();
        assert!(held.pre_read.is_none());
        assert_eq!(
            held.per_scene.get(&kitchen()).unwrap().frame_range,
            Some(range)
        );
        // Re-keyed, never copied: the set is gone and the answer stands once.
        assert!(store
            .the_answers_composition_reads(&file, None)
            .frame_range
            .is_none());
    }

    #[test]
    fn a_history_opened_page_loads_the_users_own_answers_and_never_a_resolved_version() {
        let server = a_server();
        let file = a_key(&server);
        let mut store = Store::new();
        let mut entries = SceneEntries {
            sample_count: Some(256),
            ..Default::default()
        };
        entries.simplify.use_simplify = Some(true);
        let kept = KeptSetup {
            scene: named("Kitchen"),
            entries,
            version: KeptVersion::ResolvedAutomatically,
        };
        store.load_a_kept_setup(file.clone(), kept.clone(), true);
        let held = store.per_file(&file).unwrap();
        assert_eq!(held.version_pick, None);
        assert_eq!(held.scenes_ticked, vec![named("Kitchen")]);
        assert_eq!(
            held.per_scene.get(&kitchen()).unwrap().sample_count,
            Some(256)
        );
        assert_eq!(
            held.per_scene
                .get(&kitchen())
                .unwrap()
                .simplify
                .use_simplify,
            Some(true)
        );

        // The user's own override is loaded.
        let mut store = Store::new();
        store.load_a_kept_setup(
            file.clone(),
            KeptSetup {
                version: KeptVersion::TheUsersOverride(a_blender()),
                ..kept
            },
            true,
        );
        assert_eq!(
            store.per_file(&file).unwrap().version_pick,
            Some(a_blender())
        );
    }

    #[test]
    fn simplify_is_loaded_only_where_the_offer_is_drawn() {
        let server = a_server();
        let file = a_key(&server);
        let mut store = Store::new();
        let mut entries = SceneEntries::default();
        entries.simplify.subdivision_render = Some(2);
        let kept = KeptSetup {
            scene: named("Kitchen"),
            entries,
            version: KeptVersion::ResolvedAutomatically,
        };
        store.load_a_kept_setup(file.clone(), kept, false);
        assert!(store
            .entries_for(&file, &kitchen())
            .unwrap()
            .simplify
            .is_empty());
    }

    #[test]
    fn a_keyless_page_holds_the_setup_takes_fresh_answers_and_writes_all_of_it_at_the_pick() {
        let kept = KeptSetup {
            scene: named("Kitchen"),
            entries: SceneEntries {
                sample_count: Some(256),
                ..Default::default()
            },
            version: KeptVersion::TheUsersOverride(a_blender()),
        };
        let mut held = HeldSetup::from_the_entry(kept, false);
        held.answer(Answer::ResolutionPercentage(50));
        assert_eq!(held.clear_one(&Setting::SampleCount), Ok(()));

        let server = a_server();
        let file = a_key(&server);
        let mut store = Store::new();
        store.write_the_held_setup_at_the_pick(file.clone(), held);
        let entries = store.entries_for(&file, &kitchen()).unwrap();
        assert_eq!(entries.resolution_percentage, Some(50));
        assert_eq!(entries.sample_count, None);
        assert_eq!(
            store.per_file(&file).unwrap().version_pick,
            Some(a_blender())
        );

        // Hand-everything-back on the set in hand clears the entries and the
        // override and keeps the scene name.
        let mut held = HeldSetup::from_the_entry(
            KeptSetup {
                scene: named("Kitchen"),
                entries: SceneEntries {
                    sample_count: Some(256),
                    ..Default::default()
                },
                version: KeptVersion::TheUsersOverride(a_blender()),
            },
            true,
        );
        held.hand_everything_back();
        assert_eq!(held.working_set().entries, SceneEntries::default());
        assert_eq!(
            held.working_set().version,
            KeptVersion::ResolvedAutomatically
        );
        assert_eq!(held.working_set().scene, named("Kitchen"));
    }

    #[test]
    fn switching_keys_changes_nothing_in_any_entry() {
        let server = a_server();
        let file = a_key(&server);
        let other = FileKey {
            server: server.clone(),
            path: "/ephemeral/smoozy/assets/other.blend".to_string(),
        };
        let mut store = Store::new();
        store.answer(at_kitchen(&file), Answer::SampleCount(128));
        store.answer(
            Where::AScene {
                file: other.clone(),
                scene: kitchen(),
            },
            Answer::SampleCount(1),
        );
        assert_eq!(
            store.entries_for(&file, &kitchen()).unwrap().sample_count,
            Some(128)
        );
        assert_eq!(
            store.entries_for(&other, &kitchen()).unwrap().sample_count,
            Some(1)
        );
    }
}
