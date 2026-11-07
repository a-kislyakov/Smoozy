//! §4 — one reading of the file, and everything reads it.
//!
//! **Five separate promises read the same file and must not disagree about what
//! they found**: the folders shown, the count, the main-output walk, the
//! preview's candidates, and the re-run's already-there test. **There is one
//! reading per job, recorded once, and every one of them reads that record.**
//!
//! **What is decided here and what acts.** The shapes below are the record —
//! R4's seven items and the fields R4a, I14a, I16a and I9a put on a
//! destination. Taking the reading is one invocation of a Blender on a machine
//! (R1), and [`TheInvocation`] is what that invocation names; **performing it
//! is nothing in this crate's.** *(Corrected 2026-09-07. This said performing
//! it is the work of `wiring`, in `smoozy-job-files`, over the command channel. That file says in its
//! own first lines that there is no command string in it at all — it is the
//! directories adapter and nothing else — and this crate's own
//! `arriving_later::WHAT_ANOTHER_DOCUMENT_CARRIES_OUT` in `smoozy-job-files` has always
//! routed R1's performing to `Rented-server runtime`'s worker start on the
//! file-inspection kind. The two records disagreed and this was the wrong
//! one.)* **Nothing anywhere starts that Blender today**: [`TheInvocation`] and
//! [`the_document_in`] have no caller outside this file's own tests. The two
//! sides hand back the same document (R3), which is what this file fixes.
//!
//! **The document has three payloads and only one of them is this document's.**
//! R1: *"all three payloads out of one read"*. [`TheOneDocument`] is that
//! shape, with the settings half and the dependencies half carried and not
//! interpreted here — those belong to `Render Settings` and to `Dependency
//! Upload & Remap`, and a second shape for either here would be a second
//! reading.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` S13. `Render
//! Settings` Q13, Q14, Q15, Q16.

use serde::{Deserialize, Serialize};
use smoozy_framing::source_reading::SourceReading;

use crate::manifest::CarriedAndNotInterpretedHere;

/// **R4a: a recorded location crosses as bytes, and no reading of one may
/// raise, substitute or skip.**
///
/// *"A location is exempt from the rule that a file's text is all written one
/// way — real disks hold names that are not."* **Both failures it prevents are
/// silent**: a reading that raises makes a file Blender opens perfectly well
/// *could not be read*, permanently; a reading that substitutes or skips keys
/// the wrong record, compares the wrong bytes and matches no pair at the remap,
/// so the asset is missing at a render that reports success.
///
/// **Held by the type.** There is no constructor here that takes a `String` and
/// no method that produces one, so nothing downstream can decode a location on
/// the way past.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RecordedLocation(Vec<u8>);

impl RecordedLocation {
    pub fn of(bytes: impl Into<Vec<u8>>) -> RecordedLocation {
        RecordedLocation(bytes.into())
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// The artist's own text — a scene's name, a node's name, an output's name, a
/// view layer's name.
///
/// **It crosses as bytes under R4a**, which I16a and B31a both state for the
/// fields they add. Every reader downstream compares and draws it as bytes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArtistsText(Vec<u8>);

impl ArtistsText {
    pub fn of(bytes: impl Into<Vec<u8>>) -> ArtistsText {
        ArtistsText(bytes.into())
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

/// **The version boundary every rule in this document branches on.**
///
/// D8 observed the boundary itself on disk: the property carrying a node's
/// folder is `base_path` in 4.5.9 and `directory` in 5.0.1 and 5.2.0. I2, I4,
/// B3 and B17 branch on the same boundary. **5.1 was exercised nowhere in the
/// batch and sits inside the from-5.0 branch** — §14 item 4, which states that
/// the risk is a version-specific difference nobody looked for and not a branch
/// nobody chose.
///
/// **Two things sit on top of the boundary and neither widens it.** D12 and
/// R4 item 1's Save Output switch arrive at 5.2, and R4 item 1's Use Nodes
/// switch decides nothing from 5.0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TheVersionInHand {
    pub major: u16,
    pub minor: u16,
}

/// The two sides of the boundary, named rather than compared at each site.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WhichBranch {
    /// Blender 4.5.
    FourFive,
    /// Blender 5.0 and everything after it.
    FromFiveZero,
}

impl TheVersionInHand {
    pub fn of(major: u16, minor: u16) -> TheVersionInHand {
        TheVersionInHand { major, minor }
    }

    /// **The boundary.** 4.5 on one side, 5.0 and after on the other.
    pub fn branch(self) -> WhichBranch {
        if self.major >= 5 {
            WhichBranch::FromFiveZero
        } else {
            WhichBranch::FourFive
        }
    }

    /// **D12's switch and R4 item 1's Save Output switch arrive here.**
    pub fn has_the_save_output_switch(self) -> bool {
        (self.major, self.minor) >= (5, 2)
    }

    /// **R4 item 1: from 5.3 a scene's compositor is a stack of effects**, each
    /// pointing at a node group with its own render switch (*The compositor 1*).
    /// 5.4's names were read the same at the desk
    /// (`Audits/Build/Blender 5.3/Piece 1, the desk readings — what landed.md`,
    /// reading 1), so it takes the same route. 4.5 to 5.2 stay as they were.
    pub fn holds_a_stack_of_effects(self) -> bool {
        (self.major, self.minor) >= (5, 3)
    }
}

/// **R4 item 1: the gating conditions, per scene.**
///
/// **R5: *does this file have file output nodes* is not asked anywhere.** What
/// is recorded per node is whether it **will run**, which is items 1 and 2
/// together — a file output node's outputs are recorded as destinations only
/// where the node will run ([`DestinationKind::AnOutputOfAFileOutputNode`]),
/// and there is no field here that answers the other question.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatingConditions {
    /// The Compositing switch. **Only this reaches a file output node** of the
    /// conditions below — a tree whose final output node is missing or muted
    /// still writes its nodes.
    pub compositing_switch: bool,
    /// Whether a compositing tree is present.
    pub a_compositing_tree_is_present: bool,
    /// Whether that tree holds a live final output node — the **Composite**
    /// node in 4.5 and the **Group Output** node from 5.0, which have no
    /// Composite node at all.
    pub a_live_final_output_node: bool,
    /// Whether that node is muted, **which stops the compositor's result
    /// reaching the file in 4.5 and makes no difference in the other three**.
    pub the_final_output_node_is_muted: bool,
    /// **4.5's old Use Nodes switch, which decides nothing from 5.0.** Absent
    /// where the version has none.
    pub four_fives_use_nodes_switch: Option<bool>,
    /// **From 5.2 alone, the Save Output switch** — D12's switch, which stops
    /// the scene's own frames being written even when a path is handed in on
    /// the command line, while a node in the same tree keeps writing.
    pub save_output_switch: Option<bool>,
    /// **The second thing that reaches a file output node, added 2026-09-01 out
    /// of the repaired Blender research**: a scene whose render goes out
    /// through its own sequencer holding a Scene strip writes nothing from its
    /// file output nodes at all, even though it is the scene being rendered
    /// (§14 item 13).
    pub the_render_goes_out_through_a_sequencer_holding_a_scene_strip: bool,
    /// **R4 item 1, from 5.3: the effects holding a group that are on for
    /// render, by name, as they stand at this frame.** Absent before 5.3, and
    /// from a reading taken before 2026-11-02.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub the_effects_on_for_render: Option<Vec<ArtistsText>>,
}

impl GatingConditions {
    /// **I8's second paragraph: a node in an effect that is off writes nothing.**
    /// A node with no effect named, which is every node before 5.3, is held by
    /// nothing here; nor is one read before the stack was.
    pub fn a_nodes_effect_is_on(&self, the_effect: Option<&ArtistsText>) -> bool {
        match (the_effect, &self.the_effects_on_for_render) {
            (Some(effect), Some(on)) => on.contains(effect),
            _ => true,
        }
    }

    /// **Whether the compositor runs at all**, which B5 reads and which decides
    /// whether the raw render lands at the regular output path with no walk
    /// needed.
    ///
    /// **It takes the version, because muting is version-dependent and this is
    /// the consumer.** R4 item 1 records muting as what it is — *"stops the
    /// compositor's result reaching the file in 4.5 and makes no difference in
    /// the other three"* — and until 2026-09-05 this function had no version to
    /// branch on and treated muting as fatal everywhere. On a 5.0, 5.1 or 5.2
    /// job whose artist muted the Group Output node the compositor does run and
    /// its result lands at the regular output path, and this came back false, so
    /// B5's shortcut named that path the main output **without ever walking**
    /// and B7 never got to hand the win to a file output node the beauty
    /// reached: *show outputs* could open a folder holding something other than
    /// the beauty, and B9's duplicate-picture warning was read off a walk that
    /// was never made.
    ///
    /// **The field stays as it is.** The reading records what the file says and
    /// the branch belongs here, which is how 4.5's Use Nodes switch is already
    /// handled one condition above.
    ///
    /// **B5 narrowed, 2026-10-18: a compositor with no Group Output still runs
    /// where a live File Output node is in it**, so *does not run at all* means
    /// neither a live final output node nor a live File Output node. Observed:
    /// `suzanne.blend` and a stand-in, each with two Render Layers nodes into
    /// two File Output nodes and no Group Output, wrote both node files (the
    /// plan's sources, entry 42b). Until that day this came back false for such
    /// a tree, so B5 made the regular output path the main output without a
    /// walk, and on the Creator's file the preview led with that path's fully
    /// transparent picture; now the walk runs and B7 hands the win to the node
    /// destination the beauty reaches. **Whether a live File Output node is in
    /// the tree is the caller's** ([`TheOutputsHalf::a_live_file_output_node_is_in_the_jobs_scene`]),
    /// read off the destinations the reading already keeps, so a reading
    /// recorded before that day is read the same way.
    pub fn the_compositor_runs(
        &self,
        version: TheVersionInHand,
        a_live_file_output_node_is_in_it: bool,
    ) -> bool {
        if !self.compositing_switch || !self.a_compositing_tree_is_present {
            return false;
        }
        // **4.5's old switch decides nothing from 5.0**, so it is read only
        // where the reading recorded one.
        if self.four_fives_use_nodes_switch == Some(false) {
            return false;
        }
        // **Muting stops the compositor's result reaching the file in 4.5 and
        // makes no difference in the other three** (R4 item 1).
        let the_final_output_node_counts = self.a_live_final_output_node
            && !(version.branch() == WhichBranch::FourFive && self.the_final_output_node_is_muted);
        the_final_output_node_counts || a_live_file_output_node_is_in_it
    }

    /// **Whether a file output node in this scene will write.** Two conditions
    /// reach one: the Compositing switch, and the sequencer's Scene strip.
    pub fn a_file_output_node_will_write(&self) -> bool {
        self.compositing_switch
            && self.a_compositing_tree_is_present
            && self.four_fives_use_nodes_switch != Some(false)
            && !self.the_render_goes_out_through_a_sequencer_holding_a_scene_strip
    }

    /// **D12: from 5.2 a switch inside the file stops the scene's own frames
    /// being written even when a path is handed in on the command line, while a
    /// node in the same tree keeps writing.** The count and the inventory are
    /// composed from what this says, so the redirect never promises a file that
    /// switch will prevent.
    pub fn the_scenes_own_frames_are_written(&self) -> bool {
        self.save_output_switch != Some(false)
    }
}

/// D20's added write places, as a closed set — four until 2026-10-02, five
/// since, when D20 gained a Replay fluid domain's cache folder.
///
/// **D21b: they are named settings, not found values.** Each enters the
/// destination set whether or not the walk read a value back from it, and the
/// redirect sets each one under the job's folder unconditionally. **This is the
/// opposite of D21's custom-property case**, where there is no setting to name
/// and nothing to assign to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnAddedWritePlace {
    TheBakeOutputPath,
    ThePointCache,
    TheGeometryNodesBakeFolder,
    TheSequencerProxyFolder,
    /// **D20's fifth added write place, added 2026-10-02: a fluid domain's
    /// cache folder, where the domain's cache type is Replay.** Replay is the
    /// only fluid cache type seen writing during a render, beside the `.blend`
    /// and outside the job's folder. **It is set as the file loads and never
    /// by the material** (D1a; `Rendering` L8b).
    TheReplayFluidCacheFolder,
}

/// **The five, as one list.** D20's set is closed and this is it.
pub const EVERY_ADDED_WRITE_PLACE: [AnAddedWritePlace; 5] = [
    AnAddedWritePlace::TheBakeOutputPath,
    AnAddedWritePlace::ThePointCache,
    AnAddedWritePlace::TheGeometryNodesBakeFolder,
    AnAddedWritePlace::TheSequencerProxyFolder,
    AnAddedWritePlace::TheReplayFluidCacheFolder,
];

/// **D21d: which of the four is established, and it is one.**
///
/// Fact 116 is answered for the physics point cache alone, at the four-value
/// condition D21b-i states. **For the other three nothing was established** and
/// the attempts rather than any conclusion are what is recorded: each
/// assignment read back as made, and the fixture produced no bake, no proxy and
/// no image bake, so no assignment was ever tested against a real write.
/// **D21b-ii is why the read-backs settle nothing.**
///
/// **It is permanently unresolved** (§14 item 22), so this is a fixed property
/// of the place and never a value that could be filled in later.
pub fn the_assignment_is_established_for(place: AnAddedWritePlace) -> bool {
    matches!(place, AnAddedWritePlace::ThePointCache)
}

/// **What kind of destination this is.**
///
/// **R4 item 2 names two kinds apart inside the destination set because two of
/// §9.1's conditions read only the first.** [`Destination::is_one_the_render_asked_for`]
/// is that separation, and H1a and I14 both read it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DestinationKind {
    /// The regular output path where the render writes.
    TheRegularOutputPath,
    /// One connected output of a file output node that will run.
    AnOutputOfAFileOutputNode {
        node: ArtistsText,
        /// **The output's own piece of path**, which I2's recipe joins after the
        /// folder.
        output: ArtistsText,
        /// **The node's own base file name, which I2 puts between the folder and
        /// the output's name from 5.0.** *"4.5 joins the node's folder, the
        /// output's own piece of path, the frame number and the ending; 5.0, 5.1
        /// and 5.2 put the node's own base file name between the folder and the
        /// output's name."* **Absent in 4.5**, which has no such value — an
        /// absence and not an empty name.
        the_nodes_base_file_name: Option<ArtistsText>,
        /// **The node's folder, its base file name and this output's own name
        /// with their curly-brace expressions put in** (I2's *A curly-brace
        /// expression in a node's names*, added 2026-10-18). **Absent from a
        /// reading taken before that date**, which is taken as its raw text.
        #[serde(default)]
        replaced: Option<TheNodesTextReplaced>,
        /// **R4 item 2, from 5.3: the effect whose group holds the node**, since
        /// two effects' groups can hold nodes of one name. Absent before 5.3.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        the_effect: Option<ArtistsText>,
    },
    /// **One destination for the whole node** where it is set to gather its
    /// outputs into a single multi-layer file (I7).
    AWholeFileOutputNodeGatheringIntoOneFile {
        node: ArtistsText,
        /// As above, and absent in 4.5 for the same reason.
        the_nodes_base_file_name: Option<ArtistsText>,
        /// As above, with no output's own name: the node writes one file.
        #[serde(default)]
        replaced: Option<TheNodesTextReplaced>,
        /// As above.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        the_effect: Option<ArtistsText>,
    },
    /// One of D20's four (D21b).
    AnAddedWritePlace(AnAddedWritePlace),
}

/// **A node's text with Blender's curly-brace expressions put in** —
/// `Architecture/Job files and outputs` I2, *A curly-brace expression in a
/// node's names*, added 2026-10-18 (the Creator's thirteenth notes, notes 5, 9
/// and 10).
///
/// *Dependencies 5*: an expression is *"substituted at the moment the path is
/// used so that the braces stay in the field, with a format specifier allowed
/// after a colon and a literal brace written doubled"*. **Blender replaces each
/// one before it writes, so I2's recipe joins the replaced text and never the
/// raw text.** Observed: base file name `{blend_name} - suzy` with the output
/// `Image` wrote `suzanne - suzyImage0001.png` from `suzanne.blend` (the plan's
/// sources, entry 42b).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WithItsBracesReplaced {
    /// **Every expression in it put in** — `{blend_name}` and a doubled brace,
    /// the only two the plan replaces. Text holding no brace is here as it is.
    Replaced(ArtistsText),
    /// **It holds an expression Smoozy does not replace**, named whole: any
    /// other variable, a format specifier, a brace that does not close — or,
    /// in an output's own name, any brace at all.
    Unresolved { the_expression: ArtistsText },
}

impl WithItsBracesReplaced {
    /// The text Blender writes, where every expression was replaced.
    pub fn the_text(&self) -> Option<&ArtistsText> {
        match self {
            WithItsBracesReplaced::Replaced(text) => Some(text),
            WithItsBracesReplaced::Unresolved { .. } => None,
        }
    }

    /// The expression left unresolved, where one was.
    pub fn the_expression(&self) -> Option<&ArtistsText> {
        match self {
            WithItsBracesReplaced::Replaced(_) => None,
            WithItsBracesReplaced::Unresolved { the_expression } => Some(the_expression),
        }
    }
}

/// **What the reading records beside a node destination's raw text** (I2's
/// *How Smoozy gets the replaced text*): its folder, its base file name and its
/// output's own name, each replaced or unresolved.
///
/// **4.5's gathering node holds its name front inside its folder property**
/// (D8), so its folder here is that whole property replaced and its front is
/// read off it ([`crate::mapping::the_name_front_of`]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TheNodesTextReplaced {
    /// The node's folder property, whole (*A node's folder is read the same
    /// way*).
    pub the_folder: WithItsBracesReplaced,
    /// Absent where the node has no base file name, which is 4.5.
    pub the_base_file_name: Option<WithItsBracesReplaced>,
    /// Absent for a node gathering into one file, which names no output.
    pub the_outputs_own_name: Option<WithItsBracesReplaced>,
}

impl TheNodesTextReplaced {
    /// **The first expression left unresolved**, folder first — where there
    /// is one, the output owes no inventory entry (*What an unresolved name
    /// does*).
    pub fn the_unresolved_expression(&self) -> Option<&ArtistsText> {
        self.the_folder
            .the_expression()
            .or_else(|| self.the_base_file_name.as_ref().and_then(WithItsBracesReplaced::the_expression))
            .or_else(|| self.the_outputs_own_name.as_ref().and_then(WithItsBracesReplaced::the_expression))
    }
}

/// **The one variable the reading replaces**, by its name inside the braces.
pub const THE_ONE_VARIABLE_REPLACED: &[u8] = b"blend_name";

/// **The ending `{blend_name}` leaves off** — observed as the file's name
/// without its `.blend` ending (entry 42b).
pub const THE_ENDING_BLEND_NAME_LEAVES_OFF: &[u8] = b".blend";

/// **`{blend_name}`'s value, from the open file's own path**: its name after
/// the last separator, without its `.blend` ending. **Nothing where the name
/// does not end so** — what the variable gives then is not recorded, and
/// `{blend_name}` is left unresolved rather than guessed.
pub fn the_blend_name_of(the_open_files_path: &[u8]) -> Option<Vec<u8>> {
    let name = match the_open_files_path.iter().rposition(|byte| *byte == b'/') {
        Some(at) => &the_open_files_path[at + 1..],
        None => the_open_files_path,
    };
    name.strip_suffix(THE_ENDING_BLEND_NAME_LEAVES_OFF)
        .filter(|stem| !stem.is_empty())
        .map(<[u8]>::to_vec)
}

/// **The regular output path's location, its folder read by the rule below**
/// and the front of the name left as recorded (*The regular output path's
/// folder is read the same way*, added 2026-10-18). The folder is the part
/// before the last separator, split as everything else splits that setting
/// ([`crate::mapping::split_the_regular_output_path`]): `//../Render/{blend_name}/x_`
/// is read `//../Render/standin/x_` from `standin.blend`; a folder holding any
/// other expression is unresolved, with it named.
pub fn the_regular_output_paths_location_read(
    location: &[u8],
    the_blend_name: Option<&[u8]>,
    version: TheVersionInHand,
) -> WithItsBracesReplaced {
    let (folder, _) = crate::mapping::split_the_regular_output_path(location);
    match a_folder_read(folder, the_blend_name, version) {
        WithItsBracesReplaced::Replaced(replaced) => {
            let mut out = replaced.as_bytes().to_vec();
            // What follows the folder — its separator and the front of the
            // name — exactly as recorded.
            out.extend_from_slice(&location[folder.len()..]);
            WithItsBracesReplaced::Replaced(ArtistsText::of(out))
        }
        unresolved => unresolved,
    }
}

/// **The rule, for a node's folder and base file name**: `{blend_name}`
/// becomes `the_blend_name`, `{{` becomes `{` and `}}` becomes `}`; anything
/// else in braces, or a brace that does not close, leaves the text
/// **unresolved** with that expression named. **No call that claims to work
/// out templates is made** — the plan forbids it — so the only value is the
/// one the reading read itself.
pub fn with_its_braces_replaced(text: &[u8], the_blend_name: Option<&[u8]>) -> WithItsBracesReplaced {
    let unresolved = |expression: &[u8]| WithItsBracesReplaced::Unresolved {
        the_expression: ArtistsText::of(expression.to_vec()),
    };
    let mut out = Vec::with_capacity(text.len());
    let mut at = 0;
    while at < text.len() {
        match (text[at], text.get(at + 1)) {
            (b'{', Some(b'{')) => {
                out.push(b'{');
                at += 2;
            }
            (b'}', Some(b'}')) => {
                out.push(b'}');
                at += 2;
            }
            (b'{', _) => {
                let Some(length) = text[at + 1..].iter().position(|byte| *byte == b'}') else {
                    return unresolved(&text[at..]);
                };
                let whole = &text[at..at + length + 2];
                match the_blend_name {
                    Some(name) if &text[at + 1..at + 1 + length] == THE_ONE_VARIABLE_REPLACED => {
                        out.extend_from_slice(name)
                    }
                    _ => return unresolved(whole),
                }
                at += length + 2;
            }
            (b'}', _) => return unresolved(b"}"),
            (byte, _) => {
                out.push(byte);
                at += 1;
            }
        }
    }
    WithItsBracesReplaced::Replaced(ArtistsText::of(out))
}

/// **The rule, for a folder: `Job files and outputs` P7b, added 2026-11-02.**
/// [`with_its_braces_replaced`] first. What that leaves unresolved stays so,
/// and is handed to Blender as written, only where Blender works it out and it
/// lands under the job's folder: a folder that opens with the blend-file
/// shorthand or a separator, whose every expression is one this version offers
/// ([`a_variable_blender_works_out`]). **Every other folder is replaced**, by
/// [`a_folder_with_its_expressions_named`], so Blender is handed no expression:
/// `{project_root}` would print *Invalid path template* and the node write
/// nothing; `{blend_dir}` is a whole path and would put the file's own folder
/// inside the job's; and a folder that opens with an expression is no path the
/// rule can measure.
pub fn a_folder_read(
    text: &[u8],
    the_blend_name: Option<&[u8]>,
    version: TheVersionInHand,
) -> WithItsBracesReplaced {
    match with_its_braces_replaced(text, the_blend_name) {
        WithItsBracesReplaced::Unresolved { .. } if !blender_works_out_all_of(text, version) => {
            WithItsBracesReplaced::Replaced(ArtistsText::of(a_folder_with_its_expressions_named(text)))
        }
        read => read,
    }
}

/// **The variables each version offers and works out in a node's folder**:
/// *Dependencies 5*'s lists, and each observed at the desk on 2026-11-02 (4.5.12,
/// 5.2.3, 5.3.0 Beta, 5.4.0 Alpha; `Audits/Build/Blender 5.3/Piece 5, what the
/// desk tests found — what landed.md`). **`{blend_dir}` and `{blend_dir_lib}` are
/// worked out too and left off on purpose**: each is a whole path, so under the
/// job's folder it repeats the file's folder there. The research's other
/// expression, `{project_root}`, is refused where no project is active, and no
/// project travels to a rented machine.
fn a_variable_blender_works_out(name: &[u8], version: TheVersionInHand) -> bool {
    const IN_EVERY_VERSION: [&[u8]; 4] = [b"blend_name", b"fps", b"resolution_x", b"resolution_y"];
    const FROM_FIVE_ZERO: [&[u8]; 4] = [b"blend_name_lib", b"scene_name", b"camera_name", b"node_name"];
    IN_EVERY_VERSION.contains(&name)
        || (version.branch() != WhichBranch::FourFive && FROM_FIVE_ZERO.contains(&name))
}

/// Whether this folder is one P7b hands on as written.
fn blender_works_out_all_of(text: &[u8], version: TheVersionInHand) -> bool {
    if !crate::mapping::it_opens_with_a_place(text) {
        return false;
    }
    let mut at = 0;
    while at < text.len() {
        match (text[at], text.get(at + 1)) {
            (b'{', Some(b'{')) | (b'}', Some(b'}')) => at += 2,
            (b'{', _) => {
                let Some(length) = text[at + 1..].iter().position(|byte| *byte == b'}') else {
                    return false;
                };
                if !a_variable_blender_works_out(&text[at + 1..at + 1 + length], version) {
                    return false;
                }
                at += length + 2;
            }
            (b'}', _) => return false,
            _ => at += 1,
        }
    }
    true
}

/// **P7b's name for a folder that held an expression**: each one stands as its
/// own word, so the user can tell it from the folder they typed. `{project_root}/out`
/// reads `project_root/out`, `//{fps:03}` reads `//fps_03`. A stray brace goes,
/// and whatever is not a letter, a digit, `_` or `-` inside an expression
/// becomes `_`, so no expression can make a step or a separator.
pub fn a_folder_with_its_expressions_named(text: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(text.len());
    let mut at = 0;
    while at < text.len() {
        match (text[at], text.get(at + 1)) {
            (b'{', Some(b'{')) | (b'}', Some(b'}')) => at += 2,
            (b'{', _) => match text[at + 1..].iter().position(|byte| *byte == b'}') {
                Some(length) => {
                    out.extend_from_slice(&a_word_for(&text[at + 1..at + 1 + length]));
                    at += length + 2;
                }
                None => at += 1,
            },
            (b'}', _) => at += 1,
            (byte, _) => {
                out.push(byte);
                at += 1;
            }
        }
    }
    out
}

/// One expression's own word; *expression* where nothing of it is left.
fn a_word_for(inside: &[u8]) -> Vec<u8> {
    let word: Vec<u8> = inside
        .iter()
        .map(|byte| {
            if byte.is_ascii_alphanumeric() || *byte == b'-' || *byte >= 0x80 {
                *byte
            } else {
                b'_'
            }
        })
        .collect();
    let start = word.iter().position(|byte| *byte != b'_').unwrap_or(word.len());
    let end = word.iter().rposition(|byte| *byte != b'_').map_or(start, |at| at + 1);
    if start == end {
        b"expression".to_vec()
    } else {
        word[start..end].to_vec()
    }
}

/// **The rule, for an output's own name**: whether that name takes templates
/// is not established (§14 item 31), so **a name holding any brace at all is
/// unresolved whole**, with its first expression named.
pub fn an_outputs_own_name_read(name: &[u8]) -> WithItsBracesReplaced {
    let Some(at) = name.iter().position(|byte| *byte == b'{' || *byte == b'}') else {
        return WithItsBracesReplaced::Replaced(ArtistsText::of(name.to_vec()));
    };
    let doubled = name.get(at + 1) == Some(&name[at]);
    let ends = if doubled {
        at + 2
    } else if name[at] == b'{' {
        name[at + 1..]
            .iter()
            .position(|byte| *byte == b'}')
            .map_or(name.len(), |length| at + length + 2)
    } else {
        at + 1
    };
    WithItsBracesReplaced::Unresolved {
        the_expression: ArtistsText::of(name[at..ends].to_vec()),
    }
}

/// **A literal brace in a value Blender will read for expressions is written
/// doubled**, so it lands as the one brace the inventory predicts — the
/// redirect's value for a node folder the reading replaced
/// ([`crate::redirect::the_value_a_nodes_folder_property_takes`]).
pub fn with_its_braces_doubled(text: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(text.len());
    for byte in text {
        out.push(*byte);
        if *byte == b'{' || *byte == b'}' {
            out.push(*byte);
        }
    }
    out
}

/// **I14a: the format is a field of the destination, one per destination, per
/// version-correct evaluation, taken by the same reading.**
///
/// *"No format is copied onto an inventory entry: an entry already names the
/// destination aiming at it, and a second copy would be a second mapping for
/// `Download` N5's to disagree with."*
///
/// **It is the file's own value and never a name this crate composes** (N12), so
/// it is carried as the artist's text with the ending it writes beside it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Format {
    /// The format as the file records it.
    pub as_the_file_records_it: ArtistsText,
    /// The ending it writes, where the artist has not switched the ending off
    /// (I3). **Absent is a state and not an empty string**: a destination whose
    /// ending is switched off writes no ending at all, and that is exactly the
    /// shape §14 item 20 is about.
    pub the_ending_it_writes: Option<ArtistsText>,
    /// **I19: whether this destination is video-shaped.** No version's node can
    /// write video, so only the regular output path can carry a true here.
    pub is_video_shaped: bool,
}

/// **Which scene a destination came from.**
///
/// R4 item 3: a scene reached through a **Render Layers node** contributes
/// nothing — its compositor does no work and its own nodes stay silent,
/// whatever its own compositing switch says. A scene reached through a **strip
/// in the video sequencer** contributes its own destinations, read against
/// **its own** gating conditions rather than the job's. **A reading that stops
/// at the job's own scene is short in five places at once.**
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WhichScene {
    TheJobsOwnScene,
    ReachedThroughASequencerStrip {
        scene: ArtistsText,
        strip: ArtistsText,
    },
}

/// **I16a: each destination carries where it comes from, by name.**
///
/// *"The path alone is the same string twice, and the naming is what sends the
/// artist to the right two places in their own file."* For the regular output
/// path it is the route I14 already records; for a file output node's
/// destination it is **the node's own name as the file holds it**.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WhereItComesFrom {
    TheScenesOutputPath,
    AFileOutputNodeNamed(ArtistsText),
    AnAddedWritePlaceNamed(AnAddedWritePlace),
}

impl AnAddedWritePlace {
    /// **What the place is, in standard software words** (Rule 9) — beside the
    /// type, so a refusal that names one is a sentence. Added 2026-09-23, step
    /// 10c's fifth pass on Vast, fault 5.
    pub fn in_words(self) -> &'static str {
        match self {
            AnAddedWritePlace::TheBakeOutputPath => "the bake output folder",
            AnAddedWritePlace::ThePointCache => "the physics cache folder",
            AnAddedWritePlace::TheGeometryNodesBakeFolder => "the geometry nodes bake folder",
            AnAddedWritePlace::TheSequencerProxyFolder => "the sequencer proxy folder",
            AnAddedWritePlace::TheReplayFluidCacheFolder => "the Replay fluid cache folder",
        }
    }
}

impl WhereItComesFrom {
    /// **Which part of the file a destination comes from, in standard software
    /// words** (Rule 9). **The node's own name is the artist's text and is
    /// carried whole**, never parsed.
    pub fn in_words(&self) -> String {
        match self {
            WhereItComesFrom::TheScenesOutputPath => "the scene's own output path".to_string(),
            WhereItComesFrom::AFileOutputNodeNamed(name) => {
                format!(
                    "the File Output node {}",
                    String::from_utf8_lossy(name.as_bytes())
                )
            }
            WhereItComesFrom::AnAddedWritePlaceNamed(place) => place.in_words().to_string(),
        }
    }
}

/// One destination in R4 item 2's set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Destination {
    pub kind: DestinationKind,
    /// **Where it writes, as the file holds it** — R4a's bytes.
    ///
    /// **Absent for one of D20's four the walk read no value back from**, which
    /// D21b says is an ordinary case rather than a gap: the setting is named
    /// whether or not a value was found, and the redirect assigns it under the
    /// job's folder either way.
    pub recorded_location: Option<RecordedLocation>,
    /// I14a's field. **Absent for an added write place**, which writes files
    /// this stage has no recipe to name and no method to count (I14).
    pub format: Option<Format>,
    pub origin: WhereItComesFrom,
    pub scene: WhichScene,
    /// **Which view layers feed this destination**, where any do.
    ///
    /// **I8: a view layer that does not render at a frame writes nothing at that
    /// frame, so a node input fed only by it owes no file.** That is the count's
    /// own settled clause and it is why R4 item 5 evaluates per frame. **An
    /// empty list is a destination no view layer feeds** — the added write places
    /// are the standing case — and it is never read as *fed by none of them*.
    pub the_view_layers_that_feed_it: Vec<ArtistsText>,
    /// **The regular output path's location with its folder's curly-brace
    /// expressions put in** — `Job files and outputs` I2's *The regular output
    /// path's folder is read the same way*, added 2026-10-18 (the thirteenth
    /// notes, piece P2). *Replaced* holds the whole location: the folder with
    /// `{blend_name}` and a doubled brace put in, then the front of the name
    /// exactly as recorded, since the name is asked of Blender (I1) and Smoozy
    /// writes none (N12). *Unresolved* names the expression the folder holds
    /// that Smoozy does not replace.
    ///
    /// **Absent for every other destination**, whose replaced text is its
    /// kind's own ([`DestinationKind`]'s `replaced`), **and from a reading taken
    /// before that date**, which is taken as its raw text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub the_location_with_its_folder_replaced: Option<WithItsBracesReplaced>,
}

impl Destination {
    /// **R4 item 2's first kind: the render's own output destinations.**
    ///
    /// H1a reads this and I14 reads it: an added write place is in the map, in
    /// the folders shown and under the job's folder, and **in no count**.
    pub fn is_one_the_render_asked_for(&self) -> bool {
        !matches!(self.kind, DestinationKind::AnAddedWritePlace(_))
    }

    /// The added write place this destination is, where it is one.
    pub fn as_an_added_write_place(&self) -> Option<AnAddedWritePlace> {
        match self.kind {
            DestinationKind::AnAddedWritePlace(place) => Some(place),
            _ => None,
        }
    }

    /// **The effect whose group holds this node** (R4 item 2); nothing for any
    /// other destination, or for any node before 5.3.
    pub fn the_effect(&self) -> Option<&ArtistsText> {
        match &self.kind {
            DestinationKind::AnOutputOfAFileOutputNode { the_effect, .. }
            | DestinationKind::AWholeFileOutputNodeGatheringIntoOneFile { the_effect, .. } => {
                the_effect.as_ref()
            }
            _ => None,
        }
    }

    /// **A node destination's text as the reading replaced it**; nothing for
    /// any other destination, or for a reading taken before 2026-10-18.
    pub fn the_replaced_text(&self) -> Option<&TheNodesTextReplaced> {
        match &self.kind {
            DestinationKind::AnOutputOfAFileOutputNode { replaced, .. }
            | DestinationKind::AWholeFileOutputNodeGatheringIntoOneFile { replaced, .. } => {
                replaced.as_ref()
            }
            _ => None,
        }
    }

    /// **The location Blender writes into**: a node's folder with its
    /// expressions put in, where the reading replaced them all; the recorded
    /// location otherwise — **an unresolved folder is handed on unchanged**
    /// (*A node's folder is read the same way*). The map, the name front and
    /// the folder chosen for the job all read this.
    ///
    /// **The regular output path's is its location with its folder replaced**,
    /// where the reading replaced it (*The regular output path's folder is
    /// read the same way*); its recorded location otherwise.
    pub fn the_location_blender_writes_into(&self) -> Option<&[u8]> {
        if let Some(WithItsBracesReplaced::Replaced(location)) = &self.the_location_with_its_folder_replaced {
            return Some(location.as_bytes());
        }
        match self.the_replaced_text().map(|replaced| &replaced.the_folder) {
            Some(WithItsBracesReplaced::Replaced(folder)) => Some(folder.as_bytes()),
            _ => self.recorded_location.as_ref().map(RecordedLocation::as_bytes),
        }
    }

    /// **Whether the regular output path's folder holds an expression Smoozy
    /// does not replace** — its entries are then unresolved (*The regular
    /// output path's folder is read the same way*). False for every other
    /// destination and for a reading taken before 2026-10-18.
    pub fn its_folder_is_unresolved(&self) -> bool {
        matches!(
            self.the_location_with_its_folder_replaced,
            Some(WithItsBracesReplaced::Unresolved { .. })
        )
    }

    /// **The reading's replacement, put in beside the raw text** of a node
    /// destination, and of the regular output path's folder, from
    /// `{blend_name}`'s value (I2's *How Smoozy gets the replaced text*, and
    /// *The regular output path's folder is read the same way*). An added
    /// write place is left as it is. **A folder is read by P7b's rule**
    /// ([`a_folder_read`]), and a name by the plain one: Smoozy writes no name
    /// (N12).
    pub fn put_in_the_replaced_text(&mut self, the_blend_name: Option<&[u8]>, version: TheVersionInHand) {
        if self.kind == DestinationKind::TheRegularOutputPath {
            self.the_location_with_its_folder_replaced = self
                .recorded_location
                .as_ref()
                .map(|location| the_regular_output_paths_location_read(location.as_bytes(), the_blend_name, version));
            return;
        }
        let folder = self
            .recorded_location
            .as_ref()
            .map(|location| location.as_bytes().to_vec())
            .unwrap_or_default();
        let replaced_of = |base: &Option<ArtistsText>, output: Option<&ArtistsText>| TheNodesTextReplaced {
            the_folder: a_folder_read(&folder, the_blend_name, version),
            the_base_file_name: base
                .as_ref()
                .map(|base| with_its_braces_replaced(base.as_bytes(), the_blend_name)),
            the_outputs_own_name: output.map(|output| an_outputs_own_name_read(output.as_bytes())),
        };
        match &mut self.kind {
            DestinationKind::AnOutputOfAFileOutputNode {
                output,
                the_nodes_base_file_name,
                replaced,
                ..
            } => *replaced = Some(replaced_of(the_nodes_base_file_name, Some(output))),
            DestinationKind::AWholeFileOutputNodeGatheringIntoOneFile {
                the_nodes_base_file_name,
                replaced,
                ..
            } => *replaced = Some(replaced_of(the_nodes_base_file_name, None)),
            _ => {}
        }
    }
}

/// **R4 item 5: the per-frame set.**
///
/// Every gating condition and every view layer's renderability is evaluated **at
/// every frame the job covers, not once for the job**. **That is also what
/// covers 4.5's one way of gating a node frame by frame** — the old Use Nodes
/// switch taking a keyframe — so the count is worked out per frame in all four
/// versions rather than being firm from 5.0 and known-wrong in 4.5.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AtOneFrame {
    pub frame: i64,
    /// Which view layers render at that frame.
    pub view_layers_that_render: Vec<ArtistsText>,
    /// The gating conditions as they stand at that frame.
    pub gating: GatingConditions,
    /// **I1's first bullet: for the regular output path, the name is asked of
    /// the Blender that will render**, at every frame the job covers.
    ///
    /// *"Asked where a frame will land, the file's answer matches disk for a
    /// frame render and for an animation, templates and frame-number padding and
    /// all."* **And the asking doubles as a check**: a broken template raises the
    /// same complaint before any render begins that stops the render at a
    /// failing status, so a job that could never write is stopped before it costs
    /// anything — which is why an unanswerable frame is an absence here rather
    /// than a name this crate composed.
    ///
    /// It is the whole path Blender answered, under the artist's own folder; the
    /// inventory keeps the part after the last separator and joins it to the
    /// mapped folder.
    pub the_regular_output_paths_answered_path: Option<RecordedLocation>,
}

/// **One view layer the job pins** — R4 item 5's sentence of 2026-10-04, the
/// Creator's note 14 on the test copy. The job's own choice, carried beside the
/// reading and never in it: **the recorded reading stays what the file says**,
/// so a re-run that reads the file again applies the same pins again.
///
/// It is `Render Settings` V4's pin, as bytes: that document's own entry is a
/// `Render Settings` type this crate cannot reach, so the side that holds the
/// payload hands its pins over in this shape.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AViewLayerTheJobPins {
    pub layer: ArtistsText,
    pub on: bool,
}

/// **The one rule: a pinned layer renders at every frame the job covers or at
/// none, as its pin says** (R4 item 5, the rule `Render Settings` V4 makes
/// true at the render). The pin's answer where the job pins the layer; nothing
/// where the file's own switch stands.
///
/// **Every reader of *which layers render* asks this and nothing else** — the
/// per-frame set below, B1's entry ([`the_layer_the_walk_enters_at`]) and
/// `Queue & History` B1's plan total on the machine — so the count, the folders,
/// the walk and the plan cannot disagree about a layer.
pub fn what_the_jobs_pin_says(pins: &[AViewLayerTheJobPins], layer: &[u8]) -> Option<bool> {
    pins.iter()
        .find(|pin| pin.layer.as_bytes() == layer)
        .map(|pin| pin.on)
}

/// **R4 item 5, with the job's pins applied**: the per-frame set the map, the
/// inventory, the holds and the six things are composed from. A layer pinned
/// off leaves every frame; a layer pinned on joins every frame it is not
/// already in. Everything else at each frame is the file's own.
///
/// **No order of the scene's layers is handed in here**, so a layer pinned
/// on that the file left off goes at the end of the frame's set — the answer
/// before 2026-10-17. A caller that holds the scene's own list asks
/// [`the_per_frame_set_with_the_jobs_pins_in_the_scenes_order`], which puts it
/// in its place; the layers in the set are the same either way.
pub fn the_per_frame_set_with_the_jobs_pins(
    per_frame: &[AtOneFrame],
    pins: &[AViewLayerTheJobPins],
) -> Vec<AtOneFrame> {
    the_per_frame_set_with_the_jobs_pins_in_the_scenes_order(per_frame, pins, std::iter::empty())
}

/// **The same rule, with a layer pinned on put at its place in the scene's
/// own order** — `the_scenes_layers_in_order` as Blender lists them, the
/// order every frame's set already keeps (observed in five Blenders, owed
/// piece 8 of step 10c). Added 2026-10-17, piece P8a of step 10c: the layer
/// had been added at the end, so the preview's dropdown, which lists a
/// frame's layers by their place in its set (`Queue & History` P1, P4),
/// listed it last.
///
/// **It goes before the first layer of the frame's set that comes after it
/// in the scene's order**, so no other layer moves. A layer the list does not
/// name goes at the end, as with no list at all.
pub fn the_per_frame_set_with_the_jobs_pins_in_the_scenes_order<'a>(
    per_frame: &[AtOneFrame],
    pins: &[AViewLayerTheJobPins],
    the_scenes_layers_in_order: impl IntoIterator<Item = &'a [u8]>,
) -> Vec<AtOneFrame> {
    let order: Vec<&[u8]> = the_scenes_layers_in_order.into_iter().collect();
    let place = |layer: &[u8]| order.iter().position(|one| *one == layer);
    per_frame
        .iter()
        .map(|at| {
            let mut at = at.clone();
            at.view_layers_that_render
                .retain(|layer| what_the_jobs_pin_says(pins, layer.as_bytes()) != Some(false));
            for pin in pins {
                if what_the_jobs_pin_says(pins, pin.layer.as_bytes()) == Some(true)
                    && !at.view_layers_that_render.contains(&pin.layer)
                {
                    let its_place = place(pin.layer.as_bytes())
                        .and_then(|its| {
                            at.view_layers_that_render
                                .iter()
                                .position(|other| place(other.as_bytes()).is_some_and(|theirs| theirs > its))
                        })
                        .unwrap_or(at.view_layers_that_render.len());
                    at.view_layers_that_render.insert(its_place, pin.layer.clone());
                }
            }
            at
        })
        .collect()
}

/// **B1's entry: the first of the scene's view layers, in the scene's own
/// order, that renders at the job's first frame with the job's pins applied** —
/// never a layer the job switched off. Added 2026-10-04, out of the Creator's
/// note 14 on the test copy: the build had taken the job's first pinned layer
/// whatever its pin.
///
/// The job's first frame is the lowest the per-frame set covers. **Nothing
/// where no layer of the scene renders there**, and the caller then enters at
/// the scene's first layer as a render with no pin would.
pub fn the_layer_the_walk_enters_at<'a>(
    the_scenes_layers_in_order: impl IntoIterator<Item = &'a [u8]>,
    per_frame: &[AtOneFrame],
    pins: &[AViewLayerTheJobPins],
) -> Option<ArtistsText> {
    let first = per_frame.iter().min_by_key(|at| at.frame);
    the_scenes_layers_in_order
        .into_iter()
        .find(|layer| {
            what_the_jobs_pin_says(pins, layer).unwrap_or_else(|| {
                first.is_some_and(|at| {
                    at.view_layers_that_render
                        .iter()
                        .any(|rendering| rendering.as_bytes() == *layer)
                })
            })
        })
        .map(|layer| ArtistsText::of(layer.to_vec()))
}

/// **I9a: views are not view layers, they do multiply the files, and they
/// change the names.**
///
/// *"With two views switched on, no file with that name is written at all — the
/// files are `img_0001_L.png` and `img_0001_R.png`."* **So the reading
/// establishes how many views the job's scene writes, and it is one of I6's six
/// questions rather than an afterthought.**
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HowManyViewsTheSceneWrites {
    /// **The case every rule here is written for**, and nothing changes for it.
    One,
    /// More than one view, written to one file each.
    MoreThanOneWrittenSeparately { how_many: u32 },
    /// More than one view, told to put them all in one file — every version
    /// writes one.
    MoreThanOneGatheredIntoOneFile,
}

impl HowManyViewsTheSceneWrites {
    /// **Whether the expected-output inventory may be composed for this job at
    /// all.**
    ///
    /// I9a: *"Where the reading finds the scene writing more than one view, the
    /// expected-output inventory is not composed for that job."* **This
    /// document composes no per-view names, deliberately** — the suffix form was
    /// observed for stills on one rented Linux machine and on no Mac, and for a
    /// stereo movie the name differs by version, and a recipe built from that
    /// would be assembled out of several questions, which this stage does not
    /// do.
    ///
    /// **The gathered case is one file and the names are the ordinary ones**, so
    /// it is not withheld.
    pub fn the_names_are_predictable(self) -> bool {
        !matches!(self, HowManyViewsTheSceneWrites::MoreThanOneWrittenSeparately { .. })
    }
}

/// **R4: one recorded reading, and it is the only thing anything downstream
/// reads.**
///
/// Items 1 to 6 are the reading itself. **Item 7 — the destination map, the
/// collision record, the predicted names, the count, the beauty walk's outcome
/// and the two picks — is derived**, and each is composed by the module that
/// owns it: [`crate::mapping`], [`crate::inventory`] and [`crate::main_output`].
/// They are carried on [`crate::attempts::TheMapInForce`] and on the job's
/// record rather than recomputed (R9).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TheOutputsHalf {
    /// **The version the reading was taken by** (R2). Four separate things move
    /// across the version boundary and each changes the answer rather than its
    /// wording.
    pub taken_by: TheVersionInHand,
    /// **Item 1**, for the job's own scene.
    pub gating: GatingConditions,
    /// **Item 2**, entire — the render's own output destinations and every
    /// further write place item 6's walk reaches.
    pub destinations: Vec<Destination>,
    /// **Item 3: every scene a sequencer strip reaches, read the same way.**
    /// A scene reached through a Render Layers node is not here, because it
    /// contributes nothing.
    pub scenes_reached_through_a_sequencer_strip: Vec<SceneReachedThroughAStrip>,
    /// **Item 4: whether the job's compositing tree is shared with another
    /// scene**, which arises from 5.0 on and never in 4.5, and which scene is
    /// named inside it.
    pub the_tree_is_shared_with: Option<ArtistsText>,
    /// **Item 5.**
    pub per_frame: Vec<AtOneFrame>,
    /// **Item 6's locations**, taken by a walk over Blender's own marks on its
    /// settings — **never the file's own list of outside files** (D13).
    pub recorded_locations: Vec<RecordedLocation>,
    /// **I9a's question, one of I6's six.**
    pub views: HowManyViewsTheSceneWrites,
    /// **B9's duplicate-picture warning is read off the walk and off nothing
    /// else**, so the walk's outcome is a field of the reading
    /// ([`crate::main_output`]).
    pub the_beauty_walk: crate::main_output::WhatTheWalkFound,
    /// **B25: the job's own view transform, read out of the `.blend` in this
    /// reading and carried in the manifest with it** — the colour treatment a
    /// derivative of a working-format picture is made in
    /// ([`crate::derivatives`]). Read off the job's scene as its own text
    /// (R4a) by the name [`crate::reading_script::THE_VIEW_TRANSFORMS_NAME`]
    /// cites. Added 2026-10-03, the 38's piece 3.
    ///
    /// **Absent from a reading taken before that date**, which reads as not
    /// read: B25's treatment cannot then be chosen, so no derivative of a
    /// working-format picture is made for that job, and nothing guesses one.
    #[serde(default)]
    pub the_view_transform: Option<ArtistsText>,
}

impl TheOutputsHalf {
    /// **Whether a live File Output node is in the job's own scene's tree** —
    /// B5 narrowed's second half (2026-10-18), read by
    /// [`GatingConditions::the_compositor_runs`].
    ///
    /// **Read off the destinations, which already say it**: R4 item 2 records
    /// a File Output node's outputs only where the node will run — the script
    /// passes over a muted node, and
    /// [`crate::reading_script::the_destinations_that_will_be_written`] keeps
    /// them only where [`GatingConditions::a_file_output_node_will_write`]
    /// says so. So no second fact is recorded for it, and a reading recorded
    /// before that day answers the same way.
    pub fn a_live_file_output_node_is_in_the_jobs_scene(&self) -> bool {
        self.destinations.iter().any(|destination| {
            destination.scene == WhichScene::TheJobsOwnScene
                && matches!(
                    destination.kind,
                    DestinationKind::AnOutputOfAFileOutputNode { .. }
                        | DestinationKind::AWholeFileOutputNodeGatheringIntoOneFile { .. }
                )
        })
    }

    /// **Whether the job's compositor runs at all** (B5, narrowed 2026-10-18):
    /// its gating conditions, with whether a live File Output node is in it.
    pub fn the_compositor_runs(&self) -> bool {
        self.gating
            .the_compositor_runs(self.taken_by, self.a_live_file_output_node_is_in_the_jobs_scene())
    }
}

/// One scene a sequencer strip reaches, with its own gating conditions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SceneReachedThroughAStrip {
    pub scene: ArtistsText,
    pub strip: ArtistsText,
    /// **Read against its own gating conditions rather than the job's.**
    pub gating: GatingConditions,
}

/// **R1's one document, with all three payloads out of one read.**
///
/// *"Returned as one JSON document between sentinels because Blender's own
/// chatter shares that stream."* **The settings half and the dependencies half
/// are carried and not interpreted here**: they are `Render Settings`' and
/// `Dependency Upload & Remap`'s, and a second shape for either here would be
/// the second reading R9 forbids.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TheOneDocument {
    pub outputs: TheOutputsHalf,
    pub settings: CarriedAndNotInterpretedHere,
    pub dependencies: CarriedAndNotInterpretedHere,
    /// **D21b-i, added 2026-10-02: every physics disk cache the file marks
    /// baked whose files are on the machine where the file records them**,
    /// which the render worker copies to its job-folder value before the
    /// render, as a sent bake is (`Dependency Upload & Remap` M22c).
    ///
    /// **Read only on the machine that renders** — the files are looked for
    /// on that machine's own disk — and absent from a reading written before
    /// that date, which reads as none.
    #[serde(default)]
    pub the_baked_caches_on_the_machine: Vec<ABakedCacheOnTheMachine>,
    /// **R4 item 8, added 2026-10-02 for `Rendering` L10b (the Creator's
    /// decision A): whether the job's scene holds a simulation that is not
    /// baked** — a geometry-nodes simulation zone, a point cache not baked on
    /// any carrier the walk reaches (the rigid-body world's included), or a
    /// Replay fluid domain. **Every name it is read by was read off all four
    /// Blenders** ([`THE_SIMULATION_LOOKUP`]).
    ///
    /// **A geometry-nodes simulation zone always counts**, baked or not: no
    /// name in any of the four says whether one is baked
    /// ([`THE_SIMULATION_LOOKUP`]), and stepping a baked one rendered the same
    /// frame as not stepping it (observed at the desk, 4.5.12 and 5.2.2), so a
    /// baked zone costs its stepping time and nothing else.
    ///
    /// **A recorded reading that carries no item 8 — taken before it existed —
    /// reads as no.**
    #[serde(default)]
    pub the_scene_holds_a_simulation_not_baked: bool,
    /// **The scene this reading read, by its own name** — carried whole from
    /// [`crate::reading_script::ThePrintedReading::the_scene_it_read`]. Absent
    /// from a reading taken before 2026-10-03, which reads as not named.
    #[serde(default)]
    pub the_scene_it_read: Option<ArtistsText>,
}

/// **Where R4 item 8's names come from: read off 4.5.12, 5.0.1, 5.1.2 and
/// 5.2.2 at the desk on 2026-10-02, the same in all four** — a modifier's
/// `type` `NODES`, its `bakes` and each bake's `node`, whose `bl_idname` is
/// `GeometryNodeSimulationOutput` for a simulation zone (`GeometryNodeBake`
/// for a bake node, which is not a simulation); `PointCache.is_baked`;
/// `Scene.rigidbody_world` and its `point_cache`; `Scene.objects`;
/// `ClothModifier.point_cache`, `SoftBodyModifier.point_cache`,
/// `DynamicPaintModifier.canvas_settings`, its `canvas_surfaces` and each
/// one's `point_cache`; `Object.particle_systems`, each one's `point_cache`,
/// `settings` and `use_hair_dynamics`, and `ParticleSettings.type` `HAIR`;
/// and the Replay names of [`crate::redirect::THE_FLUID_CACHE_LOOKUP`].
/// **`NodesModifierBake` carries no property saying whether it is baked** in
/// any of the four (every one of its properties was listed).
pub const THE_SIMULATION_LOOKUP: &str = "the simulation lookup, 2026-10-02 \
     (Audits/Build/Step 10c/Decision A — what landed.md)";

/// **One physics disk cache the file marks baked, whose files are on the
/// machine** (`Job files and outputs` D21b-i, added 2026-10-02).
///
/// A cache marked baked whose files are not where its setting then looks gives
/// the object as it starts, unmoving, on every frame. So a baked cache left
/// beside the `.blend` while the redirect points it at the job's folder is
/// never read unless its files are copied there.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ABakedCacheOnTheMachine {
    /// **What the cache's own setting records**, which is how its job-folder
    /// value is found in the map (R4 item 2): absent where it records none.
    pub recorded_location: Option<RecordedLocation>,
    /// **Where its files sit on this machine**, as bytes: its recorded folder
    /// where *external* is on, else the default folder beside the `.blend`
    /// (D21c).
    pub its_files_sit_in: RecordedLocation,
}

/// **The sentinels R1 names.**
///
/// Blender's own chatter shares the stream the document comes back on, so the
/// document is found between two marks rather than by taking the whole stream.
/// **Build's own choice of text**, which the plan fixes nowhere.
pub const THE_OPENING_SENTINEL: &[u8] = b"<<<SMOOZY-READING>>>";
/// The closing mark, and the same is true of it.
pub const THE_CLOSING_SENTINEL: &[u8] = b"<<<END-SMOOZY-READING>>>";

/// **`Render Settings` Q8's start sentinel: the mark the reading script writes
/// as its very first act**, before it reads anything. Added 2026-10-02, the
/// Local View crash.
///
/// **Until then the start sentinel was [`THE_OPENING_SENTINEL`], written only
/// at the script's last line with the whole document**, so anything that went
/// wrong inside the script — a crash of Blender while the walk read the file, a
/// Python error, the composed-for guard — had the same shape as a file the
/// Blender could not open, and the page told the user to pick another Blender.
/// **With this mark first, a file Blender cannot open still writes nothing**
/// (the handed-in script never runs; observed at the desk, 2026-10-02, a
/// corrupt, a cut-short and an absent file, all four Blenders), and a failure
/// inside the script is `Render Settings` Q8's third ending.
///
/// **It holds neither of the document's two marks inside it**, so a stream
/// carrying it is never mistaken for one carrying the document.
pub const THE_STARTED_MARK: &[u8] = b"<<<SMOOZY-READING-STARTED>>>";

/// **Whether a reading's streams show the script started**: its started mark,
/// or — from a script composed before that mark existed — the opening mark,
/// which no script writes before it has started.
pub fn the_reading_started_in(stream: &[u8]) -> bool {
    find(stream, THE_STARTED_MARK).is_some() || find(stream, THE_OPENING_SENTINEL).is_some()
}

/// Why a reading's stream carried no document.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyTheDocumentWasNotRead {
    /// Neither mark, or only one of the two.
    TheSentinelsWereNotBothThere,
    /// The marks were there and what sat between them was not a document.
    WhatSatBetweenTheMarksIsNotADocument,
}

/// **The document, taken out from between the sentinels.**
///
/// **Nothing is guessed from a stream that does not carry both marks.** C9's
/// discipline is the same one from the other end — *"a reading that did not
/// complete is not a reading"* — and a stream cut short before the closing mark
/// reads as no document here rather than as an empty one.
pub fn the_document_in(stream: &[u8]) -> Result<TheOneDocument, WhyTheDocumentWasNotRead> {
    let opens = find(stream, THE_OPENING_SENTINEL)
        .ok_or(WhyTheDocumentWasNotRead::TheSentinelsWereNotBothThere)?;
    let from = opens + THE_OPENING_SENTINEL.len();
    let closes = find(&stream[from..], THE_CLOSING_SENTINEL)
        .ok_or(WhyTheDocumentWasNotRead::TheSentinelsWereNotBothThere)?;
    serde_json::from_slice(&stream[from..from + closes])
        .map_err(|_| WhyTheDocumentWasNotRead::WhatSatBetweenTheMarksIsNotADocument)
}

fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

/// **R3: where the reading runs is decided by whether the file has been read
/// before submitting, and by nothing else.**
///
/// **The invocation and the payload are the same on both sides**, which is why
/// there is one [`TheInvocation`] and one [`TheOneDocument`] rather than a pair
/// of each.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhereTheReadingRuns {
    /// At pre-load, from the desktop over the connection.
    FromTheDesktopOverTheConnection,
    /// At render start, on the machine as a step of the render worker before
    /// the render is launched.
    OnTheMachineAsAStepOfTheRenderWorker,
}

/// **R1: what the one invocation names.**
///
/// One invocation of **the Blender the manifest names**, headless, over the
/// command channel. **No command string is composed here**: every value below
/// crosses as a field, which is `Server protocol and transfers` §1.1's rule and
/// the same discipline `acting` in `smoozy-job-files` already keeps.
///
/// **The cost is not a reason to skip it** (R16). About 3.2 to 3.8 seconds end
/// to end including a fresh connection, the reading itself 0.09 to 0.12 seconds,
/// 15 to 26 KB back — **and those figures are an upper bound taken on a
/// container with three testers sharing four cores**.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheInvocation {
    /// **The Blender the manifest names**, by the path of the build — a version
    /// alone cannot identify a build.
    pub the_blender_build_path: String,
    /// The `.blend` to read, as it sits on the machine.
    pub the_blend_path_on_the_machine: RecordedLocation,
    /// The version the material and the recipes are composed for (R2). The
    /// reading is taken **by the version that will render**.
    pub the_version_it_will_render_by: TheVersionInHand,
    /// The frames the job covers, which item 5 is evaluated at.
    pub the_frames_the_job_covers: Vec<i64>,
    /// **R4c: whether this is the render-start reading of a first-frame
    /// check** — the job's payload carries `Render Settings` Y1b's entry. The
    /// script then reads, and the render writes, the check's frame as a still
    /// picture where the scene's only output is a video. False at pre-load
    /// and for every other job. Added 2026-10-27.
    pub the_check_is_written_as_a_picture: bool,
    /// **P18a: whether this is the render-start reading of a first-frame
    /// check that writes into its range job's folder** — the job's payload
    /// names the range as *writes into* (`Rendering` S13). The script then
    /// puts `check_` at the start of every name the render will write before
    /// it reads anything, so every destination, the map and the inventory
    /// read marked names. False at pre-load and for every other job. Added
    /// 2026-10-31, the twenty-first notes, note 6.
    pub the_check_is_marked: bool,
}

impl TheInvocation {
    /// **Headless, no window and no display server.** Held as a value rather
    /// than left to whoever writes the call, so the one thing R1 fixes about
    /// the invocation cannot be dropped by the acting half.
    pub const HEADLESS_WITH_NO_WINDOW_AND_NO_DISPLAY_SERVER: bool = true;
}

/// **R17: whether a read disturbs a running render is NOT answered.**
///
/// Five reads taken during a 21-frame render all succeeded with an empty error
/// map and the render finished all 21 frames and all 105 files, but the run
/// totalled 155.0 s against a control's 79.8 s while the reads spanned only the
/// first 40 s, and three testers shared four cores — *"the two cannot be
/// separated"*. **What stands is that a read against a running render completed
/// correctly every time; the magnitude is untested.**
///
/// **Scheduling and exclusion between the read worker and a render worker are
/// `Rented-server runtime`'s**, which owns arbitration for every worker class
/// that can contend (§14 item 11). **Nothing here schedules, delays or refuses a
/// read**, which would be this document inventing an arbitration it does not
/// own.
pub const NOTHING_HERE_SCHEDULES_A_READ_AGAINST_A_RUNNING_RENDER: bool = true;

/// **R10: a live job holds a snapshot, not a mutable file — the four-value file
/// identity, size, inode, modification time and change time together.**
///
/// **R11: this is not a reading of state and it is the same mechanism another
/// closed document already uses.** `Server protocol and transfers` §7.4 S13
/// records all four values when a download is arranged and compares all four
/// before a resume. **One shape rather than two**, so the type is that
/// document's own.
pub type TheFourValueFileIdentity = SourceReading;

/// **R12: a fresh check is required at exactly two moments and at no others.**
///
/// *"At the start of every render attempt, before the map is applied; and when
/// the reading is taken. **Not** on a reconnection, **not** on a listing,
/// **not** on an elapsed period, and **not** because a transfer touched the
/// folder."*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheTwoMoments {
    AtTheStartOfEveryRenderAttemptBeforeTheMapIsApplied,
    WhenTheReadingIsTaken,
}

/// The two, as one list. **There is no third.**
pub const THE_TWO_MOMENTS: [TheTwoMoments; 2] = [
    TheTwoMoments::AtTheStartOfEveryRenderAttemptBeforeTheMapIsApplied,
    TheTwoMoments::WhenTheReadingIsTaken,
];

/// **R12a: R12 is scoped to the job pipeline's use of the recorded reading, and
/// a display-only comparison is outside it.**
///
/// *At exactly two moments and at no others* fixes when a **fresh check is
/// required of the pipeline** — what a manifest, a map, an inventory or an
/// attempt may rest on. A comparison taken for a page's own display, touching
/// no manifest, triggering nothing in the pipeline and writing nothing durable,
/// is neither required by R12 nor forbidden by it: `Render Settings` Q13 to Q16
/// is that case and it stands. **What R12 still forbids absolutely is a
/// pipeline moment taking its freshness from such a comparison.**
pub const NO_PIPELINE_MOMENT_TAKES_ITS_FRESHNESS_FROM_A_DISPLAY_READING: bool = true;

/// **R13: why a recorded reading is stale.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyTheReadingIsStale {
    /// The four values differ.
    TheFileChanged,
    /// The Blender version it was taken by is not the version the manifest
    /// names. **R2 requires the reading to be taken by the version that will
    /// render and `Render Settings` permits that version to be changed
    /// afterwards, which no file value moves**, so the version is one of the
    /// staleness tests rather than an assumption around them.
    ItWasTakenByAnotherVersion,
}

/// **R13, performed: a recorded reading is stale when, and only when, the four
/// values differ, or the version it was taken by is not the version the
/// manifest names.**
///
/// **The four values are compared together and never one at a time.** Fact 21:
/// a file *"can be told not to have changed from size, inode, modification time
/// and change time together"*, a save-by-rename changes the inode, and *"a
/// hand-restored modification time still moves the change time, which cannot be
/// set back"*.
pub fn is_stale(
    taken_by: TheVersionInHand,
    at_the_reading: &TheFourValueFileIdentity,
    the_version_the_manifest_names: TheVersionInHand,
    now: &TheFourValueFileIdentity,
) -> Option<WhyTheReadingIsStale> {
    if the_four_values_differ(at_the_reading, now) {
        return Some(WhyTheReadingIsStale::TheFileChanged);
    }
    if taken_by != the_version_the_manifest_names {
        return Some(WhyTheReadingIsStale::ItWasTakenByAnotherVersion);
    }
    None
}

/// The four values, compared together.
pub fn the_four_values_differ(
    one: &TheFourValueFileIdentity,
    other: &TheFourValueFileIdentity,
) -> bool {
    one.size != other.size
        || one.inode != other.inode
        || one.modification_time != other.modification_time
        || one.change_time != other.change_time
}

/// **R7: the destination map is computed once per job, on whichever side ran the
/// reading, and the render applies it rather than deriving one.**
///
/// *"The folders the user was shown and the folders the render writes into are
/// **the same object**, not two computations that have to agree."*
///
/// **H5's two triggers read this same fact, and [`crate::holds`] re-exports this
/// type rather than defining a second.** It defined one until 2026-09-05, with
/// one variant spelled differently, so a caller holding one had to translate
/// before asking the other.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WhichSideComposedTheMap {
    /// The file was read before submitting: the desktop composed the reading
    /// and the map, and both go into the manifest at submission. **The render
    /// applies the map. Nothing is recomputed.**
    TheDesktopAtSubmission,
    /// It was not: the manifest says so and carries no map. The render worker
    /// takes the reading after the load and before the render, composes the
    /// map, and writes it into the attempt record and the job record.
    TheRenderWorkerAtRenderStart,
}

/// **R8: that is consistent with `Rendering`'s Row 7 rather than against it.**
///
/// *"The lever is always the machine's; the arithmetic is done once."* The
/// redirect is applied on the machine at render start in every case, and what is
/// applied is a map computed on whichever side had the reading.
pub const THE_LEVER_IS_ALWAYS_THE_MACHINES_AND_THE_ARITHMETIC_IS_DONE_ONCE: bool = true;

/// **R9: nothing anywhere recomputes a map that exists.**
///
/// *"A builder that finds a second evaluation of the shared-run rule has found
/// the fault Row 122 exists to prevent — the user checks where their work is
/// going, agrees, and it goes somewhere else."*
///
/// **Held where it can be checked**: [`crate::mapping::compose_the_map`] is the
/// one function in this crate that runs the shared-run rule, and a test reads
/// this crate's own sources to see that nothing else does.
pub const NOTHING_RECOMPUTES_A_MAP_THAT_EXISTS: bool = true;

/// **R15: which files a live job depends on is the manifest's `.blend` and the
/// resolved set `Dependency Upload & Remap` keeps.**
///
/// This document supplies the first, with its identity and its staleness rule;
/// that module owns the second under its own rows, and `File Browser`'s
/// live-job warning reads both.
pub const THIS_DOCUMENT_SUPPLIES_THE_BLEND_AND_NOT_THE_RESOLVED_SET: bool = true;

/// **R14: what a re-run does for the user was never this stage's, and it is
/// settled at Specs.**
///
/// The re-run goes ahead, and **what was settled is the telling** — the job's
/// entry says two things: that the file changed between attempts, and **what
/// became of the frames that were already finished**, which is reused where the
/// arrangement did not move and rendered again where it did. It is said, not
/// asked, and **the telling is `Queue & History`'s**; this document composes
/// none of it.
///
/// **Corrected 2026-09-05, out of a code round.** This note said flatly that the
/// frames were rendered again rather than reused — the sentence R14's own
/// correction of 2026-09-04 withdrew, because a file edited in a way that moves
/// no destination yields an identical map and C12 finds the earlier attempt's
/// frames at exactly the paths it predicts. **The behaviour was already right**:
/// [`crate::reckoning::any_frame_an_earlier_attempt_wrote_was_kept`] is C12a and
/// implements the corrected reading. Only this note was stale.
///
/// **What this document owns is the reading the already-there test is made
/// against**: the map in force for the attempt being started, and no other
/// (§10.2 C7). **R13 keeps the previous attempt's map**, so one job's folder
/// holding two arrangements of subfolders is a recorded fact with each
/// arrangement named by the attempt that made it.
pub const THE_ALREADY_THERE_TEST_READS_THE_MAP_IN_FORCE_FOR_THIS_ATTEMPT: bool = true;

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    #[test]
    fn the_walk_enters_at_a_layer_that_renders_when_the_first_pin_is_off() {
        // **B1 as corrected 2026-10-04.** The Creator's job pinned *Box AO* off
        // first; the walk had entered there. It enters at the first of the
        // scene's layers, in the scene's order, that renders at the job's
        // first frame with the pins applied.
        let text = |of: &str| ArtistsText::of(of.as_bytes().to_vec());
        let at = |frame: i64, layers: &[&str]| AtOneFrame {
            frame,
            view_layers_that_render: layers.iter().map(|layer| text(layer)).collect(),
            gating: plain_gating(),
            the_regular_output_paths_answered_path: None,
        };
        let scene: Vec<&[u8]> = vec![b"Box AO", b"Box", b"Mattress"];
        let per_frame = vec![at(2, &["Mattress"]), at(1, &["Box AO", "Box", "Mattress"])];
        let pin = |layer: &str, on: bool| AViewLayerTheJobPins { layer: text(layer), on };

        let pins = vec![pin("Box AO", false), pin("Mattress AO", false)];
        assert_eq!(
            the_layer_the_walk_enters_at(scene.clone(), &per_frame, &pins),
            Some(text("Box"))
        );
        // No pin: the file's own first frame decides, frame 1 being the lowest.
        assert_eq!(the_layer_the_walk_enters_at(scene.clone(), &per_frame, &[]), Some(text("Box AO")));
        // A layer pinned on renders at the first frame whatever the file says.
        let off_at_one = vec![at(1, &["Mattress"])];
        assert_eq!(
            the_layer_the_walk_enters_at(scene.clone(), &off_at_one, &[pin("Box", true)]),
            Some(text("Box"))
        );
        // Nothing renders: nothing, and the caller enters as an unpinned render.
        assert_eq!(the_layer_the_walk_enters_at(scene, &[at(1, &[])], &[]), None);
    }

    #[test]
    fn the_per_frame_set_takes_each_pin_at_every_frame_and_keeps_the_rest() {
        let text = |of: &str| ArtistsText::of(of.as_bytes().to_vec());
        let at = |frame: i64, layers: &[&str]| AtOneFrame {
            frame,
            view_layers_that_render: layers.iter().map(|layer| text(layer)).collect(),
            gating: plain_gating(),
            the_regular_output_paths_answered_path: None,
        };
        let pins = vec![
            AViewLayerTheJobPins { layer: text("AO"), on: false },
            AViewLayerTheJobPins { layer: text("Fog"), on: true },
        ];
        let set = the_per_frame_set_with_the_jobs_pins(&[at(1, &["Box", "AO"]), at(2, &["Fog"])], &pins);
        assert_eq!(set[0].view_layers_that_render, vec![text("Box"), text("Fog")]);
        assert_eq!(set[1].view_layers_that_render, vec![text("Fog")]);
    }

    /// **Piece P8a of step 10c: a layer the job pins on that the file
    /// switched off takes its place in the scene's own order**, not the end
    /// of the frame's set, so the preview's dropdown lists it where Blender
    /// does (`Queue & History` P1, P4). Five layers made out of alphabetical
    /// order, the third switched off and pinned on: it is third at every
    /// frame, and nothing else moves — a frame where a later layer is keyed
    /// off, a first layer pinned on, a layer pinned off beside it, and a
    /// layer the file already renders, pinned on, all keep the file's order.
    #[test]
    fn p8a_a_layer_pinned_on_takes_its_place_in_the_scenes_order() {
        let text = |of: &str| ArtistsText::of(of.as_bytes().to_vec());
        let texts = |layers: &[&str]| layers.iter().map(|layer| text(layer)).collect::<Vec<_>>();
        let at = |frame: i64, layers: &[&str]| AtOneFrame {
            frame,
            view_layers_that_render: texts(layers),
            gating: plain_gating(),
            the_regular_output_paths_answered_path: None,
        };
        let pin = |layer: &str, on: bool| AViewLayerTheJobPins { layer: text(layer), on };
        let scene: Vec<&[u8]> = vec![b"Mid", b"Aardvark", b"Kitchen", b"Alpha", b"Beta"];
        // *Kitchen* is switched off in the file; *Alpha* is keyed off at frame 3.
        let the_files_own = vec![
            at(1, &["Mid", "Aardvark", "Alpha", "Beta"]),
            at(2, &["Mid", "Aardvark", "Alpha", "Beta"]),
            at(3, &["Mid", "Aardvark", "Beta"]),
        ];
        let set = |pins: &[AViewLayerTheJobPins]| {
            the_per_frame_set_with_the_jobs_pins_in_the_scenes_order(&the_files_own, pins, scene.clone())
                .into_iter()
                .map(|at| (at.frame, at.view_layers_that_render))
                .collect::<Vec<_>>()
        };
        assert_eq!(
            set(&[pin("Kitchen", true)]),
            vec![
                (1, texts(&["Mid", "Aardvark", "Kitchen", "Alpha", "Beta"])),
                (2, texts(&["Mid", "Aardvark", "Kitchen", "Alpha", "Beta"])),
                (3, texts(&["Mid", "Aardvark", "Kitchen", "Beta"])),
            ],
            "the third in third place at every frame, nothing else moved"
        );
        assert_eq!(
            set(&[pin("Aardvark", false), pin("Kitchen", true), pin("Beta", true)]),
            vec![
                (1, texts(&["Mid", "Kitchen", "Alpha", "Beta"])),
                (2, texts(&["Mid", "Kitchen", "Alpha", "Beta"])),
                (3, texts(&["Mid", "Kitchen", "Beta"])),
            ],
            "beside a layer pinned off, and with one already rendering pinned on"
        );
        let first_off = vec![at(1, &["Aardvark", "Alpha"])];
        assert_eq!(
            the_per_frame_set_with_the_jobs_pins_in_the_scenes_order(
                &first_off,
                &[pin("Kitchen", true), pin("Mid", true)],
                scene.clone()
            )[0]
            .view_layers_that_render,
            texts(&["Mid", "Aardvark", "Kitchen", "Alpha"]),
            "the scene's first layer pinned on goes first; two pinned on each take their own place"
        );
        // **No order to hand**: the old reading's answer, at the end.
        assert_eq!(
            the_per_frame_set_with_the_jobs_pins(&the_files_own, &[pin("Kitchen", true)])[0].view_layers_that_render,
            texts(&["Mid", "Aardvark", "Alpha", "Beta", "Kitchen"]),
        );
    }

    pub(crate) fn a_location(text: &str) -> RecordedLocation {
        RecordedLocation::of(text.as_bytes().to_vec())
    }

    pub(crate) fn plain_gating() -> GatingConditions {
        GatingConditions {
            the_effects_on_for_render: None,
            compositing_switch: true,
            a_compositing_tree_is_present: true,
            a_live_final_output_node: true,
            the_final_output_node_is_muted: false,
            four_fives_use_nodes_switch: None,
            save_output_switch: None,
            the_render_goes_out_through_a_sequencer_holding_a_scene_strip: false,
        }
    }

    pub(crate) fn a_destination(
        kind: DestinationKind,
        at: &str,
        origin: WhereItComesFrom,
    ) -> Destination {
        Destination {
            kind,
            recorded_location: Some(a_location(at)),
            format: Some(Format {
                as_the_file_records_it: ArtistsText::of(b"PNG".to_vec()),
                the_ending_it_writes: Some(ArtistsText::of(b".png".to_vec())),
                is_video_shaped: false,
            }),
            origin,
            scene: WhichScene::TheJobsOwnScene,
            the_view_layers_that_feed_it: Vec::new(),
            the_location_with_its_folder_replaced: None,
        }
    }

    /// **I2's rule** (added 2026-10-18): `{blend_name}` is put in from the open
    /// file's own name and `{{`/`}}` become one brace; any other expression,
    /// one with a format specifier, or a brace that does not close leaves the
    /// text unresolved with that expression named. **An output's own name
    /// holding any brace at all is unresolved.**
    #[test]
    fn only_blend_name_and_a_doubled_brace_are_replaced_and_everything_else_is_named_unresolved() {
        let replaced = |text: &str| WithItsBracesReplaced::Replaced(ArtistsText::of(text.as_bytes().to_vec()));
        let unresolved = |expression: &str| WithItsBracesReplaced::Unresolved {
            the_expression: ArtistsText::of(expression.as_bytes().to_vec()),
        };
        let name = Some(&b"suzanne"[..]);
        let read = |text: &str| with_its_braces_replaced(text.as_bytes(), name);
        assert_eq!(read("{blend_name} - suzy"), replaced("suzanne - suzy"));
        assert_eq!(read("//../Render/{blend_name}/x{blend_name}"), replaced("//../Render/suzanne/xsuzanne"));
        assert_eq!(read("{{x}}"), replaced("{x}"));
        assert_eq!(read("{{blend_name}}"), replaced("{blend_name}"), "a doubled brace is a literal one");
        assert_eq!(read("Image"), replaced("Image"));
        assert_eq!(read(""), replaced(""));
        assert_eq!(read("{fps} - a"), unresolved("{fps}"));
        assert_eq!(read("{blend_name} {resolution_x}"), unresolved("{resolution_x}"));
        assert_eq!(read("{blend_name:>10}"), unresolved("{blend_name:>10}"), "a format specifier");
        assert_eq!(read("{blend_dir}"), unresolved("{blend_dir}"));
        assert_eq!(read("{project_root}"), unresolved("{project_root}"));
        assert_eq!(read("a {blend_name"), unresolved("{blend_name"), "a brace that does not close");
        assert_eq!(read("a } b"), unresolved("}"));
        assert_eq!(
            with_its_braces_replaced(b"{blend_name} - a", None),
            unresolved("{blend_name}"),
            "no value read, nothing guessed"
        );

        assert_eq!(an_outputs_own_name_read(b"Image"), replaced("Image"));
        assert_eq!(an_outputs_own_name_read(b"{blend_name}Image"), unresolved("{blend_name}"));
        assert_eq!(an_outputs_own_name_read(b"x{{y}}"), unresolved("{{"));
        assert_eq!(an_outputs_own_name_read(b"x}"), unresolved("}"));

        assert_eq!(the_blend_name_of(b"/home/ubuntu/rendering/suzanne.blend"), Some(b"suzanne".to_vec()));
        assert_eq!(the_blend_name_of(b"suzanne - v2.blend"), Some(b"suzanne - v2".to_vec()));
        assert_eq!(the_blend_name_of(b"/home/ubuntu/suzanne.blend1"), None);
        assert_eq!(the_blend_name_of(b"/home/ubuntu/.blend"), None);

        assert_eq!(with_its_braces_doubled(b"/job/{x}/"), b"/job/{{x}}/".to_vec());
        assert_eq!(with_its_braces_doubled(b"/job/out/"), b"/job/out/".to_vec());
    }

    /// **P7b, each row observed at the desk (5.2.3, 5.3.0 Beta, 5.4.0 Alpha).**
    /// An expression Blender refuses, or a whole path, or a folder opening with
    /// no place, is replaced by its own word; what Blender works out in a place
    /// the rule can measure is handed on, in the versions that offer it.
    #[test]
    fn p7b_a_folder_is_replaced_unless_blender_works_it_out_under_the_jobs_folder() {
        let five = TheVersionInHand::of(5, 2);
        let four = TheVersionInHand::of(4, 5);
        let read = |text: &str, version| a_folder_read(text.as_bytes(), Some(b"job"), version);
        let replaced = |text: &str| WithItsBracesReplaced::Replaced(ArtistsText::of(text.as_bytes().to_vec()));
        let kept = |expression: &str| WithItsBracesReplaced::Unresolved {
            the_expression: ArtistsText::of(expression.as_bytes().to_vec()),
        };
        // Refused by Blender: replaced, whatever opens the folder.
        assert_eq!(read("{project_root}/out", five), replaced("project_root/out"));
        assert_eq!(read("//renders/{project_root}/out", five), replaced("//renders/project_root/out"));
        assert_eq!(read("//{fps:03}", five), replaced("//fps_03"));
        assert_eq!(read("//{unknown}/out", five), replaced("//unknown/out"));
        assert_eq!(read("//{unclosed/out", five), replaced("//unclosed/out"));
        assert_eq!(read("//}x/out", five), replaced("//x/out"));
        // A whole path, worked out and still replaced.
        assert_eq!(read("{blend_dir}/out", five), replaced("blend_dir/out"));
        assert_eq!(read("//{blend_dir}/out", five), replaced("//blend_dir/out"));
        assert_eq!(read("//{blend_dir_lib}", five), replaced("//blend_dir_lib"));
        // Opens with no place: replaced though Blender would work it out.
        assert_eq!(read("{scene_name}/out", five), replaced("scene_name/out"));
        assert_eq!(read("{node_name}/out", five), replaced("node_name/out"));
        // Worked out in a place the rule can measure: as it was.
        assert_eq!(read("//{scene_name}/out", five), kept("{scene_name}"));
        assert_eq!(read("/mnt/{fps}", five), kept("{fps}"));
        assert_eq!(read("//{resolution_x}x{resolution_y}", five), kept("{resolution_x}"));
        // 4.5 offers four, so the rest are refused there.
        assert_eq!(read("//{fps}", four), kept("{fps}"));
        assert_eq!(read("//{scene_name}/out", four), replaced("//scene_name/out"));
        assert_eq!(read("//{node_name}", four), replaced("//node_name"));
        // The plain rule still comes first.
        assert_eq!(read("//{blend_name}/out", five), replaced("//job/out"));
        assert_eq!(read("{blend_name}/out", five), replaced("job/out"));
        assert_eq!(read("{{lit}}/out", five), replaced("{lit}/out"));
        // Nothing an expression holds can make a step or a separator.
        assert_eq!(a_folder_with_its_expressions_named(b"{a/b}/{..}/{}"), b"a_b/expression/expression".to_vec());
        // The regular output path's folder takes the same rule, its front as
        // recorded.
        assert_eq!(
            the_regular_output_paths_location_read(b"{blend_dir}/r/f_{fps}", Some(b"job"), five),
            replaced("blend_dir/r/f_{fps}")
        );
        assert_eq!(the_regular_output_paths_location_read(b"//{scene_name}/f_", Some(b"job"), five), kept("{scene_name}"));
    }

    #[test]
    fn a_recorded_location_has_no_route_to_text_and_survives_bytes_no_reader_could_decode() {
        // R4a. A name a disk holds that is not text at all must cross intact:
        // a reading that substituted or dropped it would key the wrong record
        // and match no pair at the remap, with the render reporting success.
        let hostile = vec![0xffu8, 0xfe, b'/', b'o', b'u', b't'];
        let location = RecordedLocation::of(hostile.clone());
        assert_eq!(location.as_bytes(), &hostile[..]);
        let written = serde_json::to_vec(&location).unwrap();
        let again: RecordedLocation = serde_json::from_slice(&written).unwrap();
        assert_eq!(again.as_bytes(), &hostile[..]);
    }

    #[test]
    fn only_the_compositing_switch_and_the_scene_strip_reach_a_file_output_node() {
        // R4 item 1. A tree whose final output node is missing or muted still
        // writes its nodes; a scene strip stops them.
        let mut gating = plain_gating();
        gating.a_live_final_output_node = false;
        assert!(gating.a_file_output_node_will_write());
        assert!(!gating.the_compositor_runs(TheVersionInHand::of(5, 2), false));

        let mut with_a_strip = plain_gating();
        with_a_strip.the_render_goes_out_through_a_sequencer_holding_a_scene_strip = true;
        assert!(!with_a_strip.a_file_output_node_will_write());
    }

    #[test]
    fn a_muted_final_output_node_stops_the_compositor_in_four_five_alone() {
        // R4 item 1: muting *"stops the compositor's result reaching the file in
        // 4.5 and makes no difference in the other three"*. The consumer had no
        // version and treated it as fatal everywhere, so a 5.0, 5.1 or 5.2 job
        // whose artist muted the Group Output node had its main output come back
        // as the regular output path off B5's shortcut, with no walk taken at
        // all — B7 never got to hand the win to a file output node the beauty
        // reached, *show outputs* could open a folder holding something other
        // than the beauty, and B9's warning was read off a walk nobody made.
        let mut muted = plain_gating();
        muted.the_final_output_node_is_muted = true;
        assert!(!muted.the_compositor_runs(TheVersionInHand::of(4, 5), false));
        for version in [
            TheVersionInHand::of(5, 0),
            TheVersionInHand::of(5, 1),
            TheVersionInHand::of(5, 2),
        ] {
            assert!(muted.the_compositor_runs(version, false), "{version:?}");
        }
        // And the field itself is unchanged: the reading records what the file
        // says and the branch is here.
        assert!(muted.the_final_output_node_is_muted);
    }

    /// **B5 narrowed, 2026-10-18: a compositor with no Group Output still
    /// runs where a live File Output node is in it**, in every version; with
    /// neither, it does not run at all, as before. A live File Output node
    /// does not make it run where the Compositing switch, the tree or 4.5's
    /// Use Nodes switch says it does not. In 4.5 a muted Composite node beside
    /// a live File Output node is the same case as a missing one.
    #[test]
    fn a_compositor_with_no_final_output_node_runs_where_a_live_file_output_node_is_in_it() {
        let mut no_final = plain_gating();
        no_final.a_live_final_output_node = false;
        for version in [
            TheVersionInHand::of(4, 5),
            TheVersionInHand::of(5, 0),
            TheVersionInHand::of(5, 1),
            TheVersionInHand::of(5, 2),
            TheVersionInHand::of(5, 3),
        ] {
            assert!(no_final.the_compositor_runs(version, true), "{version:?}");
            assert!(!no_final.the_compositor_runs(version, false), "{version:?}: neither");
        }
        let mut muted_in_four_five = plain_gating();
        muted_in_four_five.the_final_output_node_is_muted = true;
        assert!(muted_in_four_five.the_compositor_runs(TheVersionInHand::of(4, 5), true));

        let mut switched_off = no_final.clone();
        switched_off.compositing_switch = false;
        assert!(!switched_off.the_compositor_runs(TheVersionInHand::of(5, 3), true));
        let mut no_tree = no_final.clone();
        no_tree.a_compositing_tree_is_present = false;
        assert!(!no_tree.the_compositor_runs(TheVersionInHand::of(5, 3), true));
        let mut use_nodes_off = no_final;
        use_nodes_off.four_fives_use_nodes_switch = Some(false);
        assert!(!use_nodes_off.the_compositor_runs(TheVersionInHand::of(4, 5), true));
    }

    #[test]
    fn four_fives_use_nodes_switch_decides_nothing_from_five_zero() {
        // R4 item 1. The reading records the switch only where the version has
        // one, so a from-5.0 reading cannot be gated by it at all.
        let mut four_five = plain_gating();
        four_five.four_fives_use_nodes_switch = Some(false);
        assert!(!four_five.the_compositor_runs(TheVersionInHand::of(4, 5), true));
        assert!(!four_five.a_file_output_node_will_write());

        let from_five_zero = plain_gating();
        assert_eq!(from_five_zero.four_fives_use_nodes_switch, None);
        assert!(from_five_zero.the_compositor_runs(TheVersionInHand::of(5, 2), false));
    }

    #[test]
    fn the_save_output_switch_stops_the_scenes_own_frames_and_not_a_node() {
        // D12. From 5.2 the switch stops the scene's own frames being written
        // even when a path is handed in on the command line, while a node in
        // the same tree keeps writing.
        let mut gating = plain_gating();
        gating.save_output_switch = Some(false);
        assert!(!gating.the_scenes_own_frames_are_written());
        assert!(gating.a_file_output_node_will_write());
        assert!(TheVersionInHand::of(5, 2).has_the_save_output_switch());
        assert!(!TheVersionInHand::of(5, 1).has_the_save_output_switch());
        assert!(!TheVersionInHand::of(4, 5).has_the_save_output_switch());
    }

    #[test]
    fn the_version_boundary_puts_five_one_on_the_from_five_zero_side() {
        // §14 item 4: 5.1 was exercised nowhere and the research settles the
        // boundary itself, so it sits inside the from-5.0 branch.
        assert_eq!(TheVersionInHand::of(4, 5).branch(), WhichBranch::FourFive);
        for minor in [0u16, 1, 2] {
            assert_eq!(
                TheVersionInHand::of(5, minor).branch(),
                WhichBranch::FromFiveZero
            );
        }
    }

    #[test]
    fn r13_is_stale_on_the_four_values_and_on_the_version_and_on_nothing_else() {
        let at_the_reading = SourceReading {
            size: 4096,
            inode: "912".to_string(),
            modification_time: 1_756_800_000,
            change_time: 1_756_800_001,
        };
        let four_five = TheVersionInHand::of(4, 5);
        assert_eq!(
            is_stale(four_five, &at_the_reading, four_five, &at_the_reading),
            None
        );

        // A hand-restored modification time still moves the change time.
        let mut restored = at_the_reading.clone();
        restored.change_time += 1;
        assert_eq!(
            is_stale(four_five, &at_the_reading, four_five, &restored),
            Some(WhyTheReadingIsStale::TheFileChanged)
        );

        // A save-by-rename changes the inode and nothing else need move.
        let mut renamed = at_the_reading.clone();
        renamed.inode = "913".to_string();
        assert_eq!(
            is_stale(four_five, &at_the_reading, four_five, &renamed),
            Some(WhyTheReadingIsStale::TheFileChanged)
        );

        // And the version is a staleness test in its own right.
        assert_eq!(
            is_stale(
                four_five,
                &at_the_reading,
                TheVersionInHand::of(5, 2),
                &at_the_reading
            ),
            Some(WhyTheReadingIsStale::ItWasTakenByAnotherVersion)
        );
    }

    #[test]
    fn the_document_is_taken_from_between_the_sentinels_and_never_from_the_whole_stream() {
        // R1: Blender's own chatter shares that stream.
        let document = TheOneDocument {
            the_baked_caches_on_the_machine: Vec::new(),
            the_scene_holds_a_simulation_not_baked: false,
            the_scene_it_read: None,
            outputs: TheOutputsHalf {
                taken_by: TheVersionInHand::of(4, 5),
                gating: plain_gating(),
                destinations: Vec::new(),
                scenes_reached_through_a_sequencer_strip: Vec::new(),
                the_tree_is_shared_with: None,
                per_frame: Vec::new(),
                recorded_locations: Vec::new(),
                views: HowManyViewsTheSceneWrites::One,
                the_beauty_walk: crate::main_output::WhatTheWalkFound::NoEntry,
                the_view_transform: None,
            },
            settings: CarriedAndNotInterpretedHere(serde_json::json!({})),
            dependencies: CarriedAndNotInterpretedHere(serde_json::json!({})),
        };
        let mut stream = b"Blender 4.5.9\nRead prefs\n".to_vec();
        stream.extend_from_slice(THE_OPENING_SENTINEL);
        stream.extend_from_slice(&serde_json::to_vec(&document).unwrap());
        stream.extend_from_slice(THE_CLOSING_SENTINEL);
        stream.extend_from_slice(b"\nBlender quit\n");
        assert_eq!(the_document_in(&stream).unwrap(), document);
    }

    /// **`Render Settings` Q8, the Local View crash: the started mark is told
    /// apart from both of the document's marks**, and a stream carrying it
    /// alone — a reading that crashed after its first act — is a started
    /// reading with no document.
    #[test]
    fn the_started_mark_holds_neither_document_mark_and_alone_is_a_start_with_no_document() {
        let holds = |mark: &[u8], inside: &[u8]| find(mark, inside).is_some();
        assert!(!holds(THE_STARTED_MARK, THE_OPENING_SENTINEL));
        assert!(!holds(THE_STARTED_MARK, THE_CLOSING_SENTINEL));
        assert!(!holds(THE_OPENING_SENTINEL, THE_STARTED_MARK));

        let mut crashed = THE_STARTED_MARK.to_vec();
        crashed.extend_from_slice(b"\nWriting: /tmp/the-file.crash.txt\n");
        assert!(the_reading_started_in(&crashed));
        assert_eq!(
            the_document_in(&crashed),
            Err(WhyTheDocumentWasNotRead::TheSentinelsWereNotBothThere)
        );

        // **A file the Blender could not open writes no mark at all.**
        assert!(!the_reading_started_in(
            b"Error: Failed to read blend file '/a.blend': Missing DNA block\n"
        ));
        // **A script composed before the started mark existed** still shows
        // its start by the opening mark.
        assert!(the_reading_started_in(THE_OPENING_SENTINEL));
    }

    #[test]
    fn a_stream_cut_short_before_the_closing_mark_is_no_document_rather_than_an_empty_one() {
        // C9's discipline from the other end: partial output is delivered
        // before a connection dies, and a truncated answer read as an empty
        // one is the whole hazard.
        let mut stream = b"chatter".to_vec();
        stream.extend_from_slice(THE_OPENING_SENTINEL);
        stream.extend_from_slice(br#"{"outputs":"#);
        assert_eq!(
            the_document_in(&stream),
            Err(WhyTheDocumentWasNotRead::TheSentinelsWereNotBothThere)
        );
    }

    #[test]
    fn an_added_write_place_is_not_one_the_render_asked_for() {
        // R4 item 2 names the two kinds apart because H1a and I14 read only
        // the first.
        let regular = a_destination(
            DestinationKind::TheRegularOutputPath,
            "/home/a/out/f_",
            WhereItComesFrom::TheScenesOutputPath,
        );
        assert!(regular.is_one_the_render_asked_for());
        let cache = Destination {
            kind: DestinationKind::AnAddedWritePlace(AnAddedWritePlace::ThePointCache),
            recorded_location: None,
            format: None,
            origin: WhereItComesFrom::AnAddedWritePlaceNamed(AnAddedWritePlace::ThePointCache),
            scene: WhichScene::TheJobsOwnScene,
            the_view_layers_that_feed_it: Vec::new(),
            the_location_with_its_folder_replaced: None,
        };
        assert!(!cache.is_one_the_render_asked_for());
        assert_eq!(
            cache.as_an_added_write_place(),
            Some(AnAddedWritePlace::ThePointCache)
        );
    }

    #[test]
    fn only_the_point_cache_has_an_established_assignment() {
        // D21d and §14 item 22, permanently unresolved for the other three.
        for place in EVERY_ADDED_WRITE_PLACE {
            assert_eq!(
                the_assignment_is_established_for(place),
                place == AnAddedWritePlace::ThePointCache
            );
        }
    }

    #[test]
    fn a_two_view_scene_withholds_the_names_and_a_gathered_pair_does_not() {
        // I9a. The withholding is only for the shape whose names are never
        // written; a job told to put both views in one file writes the
        // ordinary name.
        assert!(HowManyViewsTheSceneWrites::One.the_names_are_predictable());
        assert!(HowManyViewsTheSceneWrites::MoreThanOneGatheredIntoOneFile.the_names_are_predictable());
        assert!(
            !HowManyViewsTheSceneWrites::MoreThanOneWrittenSeparately { how_many: 2 }
                .the_names_are_predictable()
        );
    }

    #[test]
    fn there_are_two_moments_and_no_third() {
        // R12.
        assert_eq!(THE_TWO_MOMENTS.len(), 2);
    }
}
