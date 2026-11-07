//! §8.1 and §8.2 — the main output, found by following the beauty pass, and the
//! two picks that are made once.
//!
//! **B1: the main output is found by following the beauty pass forward from
//! where the render enters the compositor.** The beauty pass is called
//! **Combined** in every version — in a written file and in a multi-layer
//! file's list of layers — and the render enters on a **Render Layers node**,
//! which names the scene and the view layer it reads, so the entry point can be
//! picked out. **The socket the beauty leaves that node on is called `Image`**,
//! which is a different name from the pass's own. **Nothing marks the picture
//! once it is travelling, in any version**, so the beauty is followed by wiring
//! and never recognised further along.
//!
//! **B1's socket claim was corrected in the plan on 2026-09-20 and this file
//! follows it.** The clause had said the pass is called *Combined* on the
//! socket it comes out of as well; **it is not**. Every Render Layers node in
//! 4.5.12, 5.0.1, 5.1.2 and 5.2.2 LTS was read at the desk, under Cycles and
//! under the built-in engine alike, and the socket the beauty leaves on is
//! called `Image`, with no socket on the node called `Combined` at all. **So
//! the walk matched nothing on any real file and every main-output walk came
//! back `NoEntry`** — B8's outcome for every job rather than for a failed one.
//! **The other two of B1's places are untouched**: the name inside a written
//! file and the name in a multi-layer file's list of layers, which is what B19
//! rests on.
//!
//! **B2: the walk reads back out of a saved file with no window at all,
//! complete, in every version**, which is what lets it run inside the reading of
//! §4 rather than needing a render. So the tree below arrives as a value and the
//! walk is performed at the desk exactly as it is performed on the machine.
//!
//! **B10: thirteen of the seventeen terminal tests behind the walk settle Linux
//! and are untested on macOS.** Every walk here runs on the rented machine,
//! which is Linux, so the untested half is not on the path this document takes.
//! Nothing below branches on an operating system.

use serde::{Deserialize, Serialize};

use crate::reading::{ArtistsText, TheVersionInHand, WhichBranch};

/// **B1: the beauty pass's name, in every version.**
///
/// *"Called **Combined** in every version — in a written file and in a
/// multi-layer file's list of layers."* **B19a: a picture is found by its name
/// and never by its position**, and this constant is that rule's one spelling.
///
/// **This is the name a file holds, and not the name of the socket it leaves
/// the entry node on**, which is [`THE_BEAUTYS_SOCKET_NAME`]. The two were one
/// constant until 2026-09-20 and that is what made the walk find nothing.
pub const THE_BEAUTY_PASSES_NAME: &[u8] = b"Combined";

/// **B1: the socket the beauty leaves the Render Layers node on.**
///
/// *"The socket the beauty leaves that node on is called `Image`, which is a
/// different name from the pass's own and is the name the walk starts at."*
///
/// Label: **observed in a test** — every Render Layers node in 4.5.12, 5.0.1,
/// 5.1.2 and 5.2.2 LTS, read at the desk under Cycles and under the built-in
/// engine alike, `Audits/Build/Step 10c/Piece 13 — what landed.md`, finding 1.
/// **Nothing here is a guess**: this is the name the four Blenders printed.
pub const THE_BEAUTYS_SOCKET_NAME: &[u8] = b"Image";

/// One socket of one node, by the node's place in the tree and the socket's own
/// place on that node.
///
/// **The socket's index is the wiring and never the picture's identity.** B19a
/// forbids picking a picture by position; this is a link's own end, which is the
/// only thing the tree holds positionally, and every picture below is still
/// found by its name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Socket {
    pub node: usize,
    pub index: usize,
}

/// One link between two sockets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Link {
    pub from: Socket,
    pub to: Socket,
}

/// **Which of a two-picture node's two inputs came out**, where that can be
/// told at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WhichInputCameOut {
    /// The node's **first** input, whichever socket the version puts it on.
    TheFirst,
    /// The other one.
    TheSecond,
}

/// **B3's second limit: which socket a splitting node's first input sits on
/// moves across the version boundary.**
///
/// *"A splitting node puts its first input on the right in 4.5 and on the left
/// in 5.0, 5.1 and 5.2, so which side carries the beauty is version-dependent
/// and the walk is composed per version like everything else here."*
///
/// **The two sides are the sockets in the order the file holds them**, and the
/// version decides which of the two is the *first* input. Nothing here reads a
/// side off a picture or off a name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WhichSide {
    Left,
    Right,
}

impl WhichSide {
    /// **The side the version puts the first input on.**
    pub fn the_first_input_of_a_splitting_node_in(version: TheVersionInHand) -> WhichSide {
        match version.branch() {
            WhichBranch::FourFive => WhichSide::Right,
            WhichBranch::FromFiveZero => WhichSide::Left,
        }
    }
}

/// What a node is, as far as the walk is concerned.
///
/// **The walk knows five shapes and one refusal, and the refusal is the point.**
/// B3: *"At any other kind, the walk stops."* A node this list does not name is
/// [`NodeKind::ANodeTheWalkCannotPass`], which is a **stated** outcome and never
/// a silent pass-through.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeKind {
    /// **B1's entry point.** It names the scene and the view layer it reads, and
    /// its outputs are named — the beauty leaves on the one called `Image`.
    RenderLayers {
        scene: ArtistsText,
        view_layer: ArtistsText,
        /// The output sockets' names, in the file's own order. **The walk finds
        /// [`THE_BEAUTYS_SOCKET_NAME`] among them by name** (B19a), and never
        /// by taking the first.
        outputs: Vec<ArtistsText>,
    },
    /// The tree's final output node — the **Composite** node in 4.5 and the
    /// **Group Output** node from 5.0. **Reaching it is what puts the regular
    /// output path among the candidates** (B4).
    TheFinalOutputNode,
    /// A file output node. Its inputs are named; the destination the beauty
    /// reaches is the input it arrived at, **or the whole node where it is set
    /// to gather its outputs into a single multi-layer file** (I7).
    AFileOutputNode {
        inputs: Vec<ArtistsText>,
        gathering_into_one_file: bool,
        /// **Whether the node is muted.** R4 item 2: *"A muted node contributes
        /// none"*, so the walk ends at a muted node and counts nothing (§8.1's
        /// paragraph after B9, added 2026-10-18). **Absent reads as not
        /// muted**, so a reading recorded before the field existed still parses
        /// and reads as it did. Added 2026-10-18, the Creator's twelfth notes,
        /// note 6: a file whose only output was a video was said to write frames
        /// from a muted File Output node.
        #[serde(default)]
        muted: bool,
    },
    /// **B3's first limit: a link cannot be walked across the boundary of a node
    /// group, so the walk steps into the group's own tree.**
    ANodeGroup { inner: TheCompositingTree },
    /// Inside a group's own tree: the picture arriving from outside, at the same
    /// socket index it entered the group on.
    TheGroupsOwnInput,
    /// Inside a group's own tree: the picture leaving for outside, at the same
    /// socket index it leaves the group on.
    TheGroupsOwnOutput,
    /// **B1a, from 5.3: a Group Input at the top of an effect's group**, where
    /// the render enters. Its outputs are named, and the beauty leaves on the
    /// one called [`THE_BEAUTYS_SOCKET_NAME`], found by name (B19a).
    TheRendersEntry { outputs: Vec<ArtistsText> },
    /// **B1a, from 5.3: one effect of a scene's stack, its name the node's**, and
    /// its group inside. The stack is read as a tree of these, in list order and
    /// with no links, so the reading's one tree field carries it.
    /// [`walk_the_beauty_through_the_effects`] is the walk made of them.
    AnEffect { inner: TheCompositingTree },
    /// A node that takes one picture and passes one on. **The walk goes
    /// through** — there is one input and one thing that can have come out.
    OnePictureInOnePictureOut,
    /// **B3's third and fourth limits.** Two pictures in, one out.
    ATwoPictureNode {
        /// **The third limit.** *"Where a two-picture node's factor sits between
        /// its ends, both inputs contribute and nothing says which contributed
        /// what. The walk stops."* **The factor is read rather than assumed**,
        /// which is why a freshly added mixing node starting at a different
        /// factor in 5.2 from the other three changes nothing here.
        the_factor_sits_between_its_ends: bool,
        /// **The fourth limit.** *"Which input came out can be told for five
        /// named kinds of two-picture node and for no others."*
        ///
        /// **The five are not named in this document and are not named here.**
        /// The reading establishes, for the node in hand, whether its kind is
        /// one of the five and which input came out; **absent means the kind is
        /// not one of the five and the walk stops**. A list composed here would
        /// be five spellings this project never established, and a wrong one
        /// would send the walk through a node it cannot see through.
        which_input_came_out: Option<WhichInputCameOut>,
    },
    /// Anything else. **The walk stops and names it** (B4's *blocked*).
    ANodeTheWalkCannotPass,
}

/// One node of a compositing tree.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Node {
    /// The node's own name as the file holds it. **The artist's own text, which
    /// crosses as bytes under R4a.**
    pub name: ArtistsText,
    pub kind: NodeKind,
}

/// A compositing tree — a group's own tree has the same shape, which is what
/// lets B3's first limit be handled by stepping into it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TheCompositingTree {
    pub nodes: Vec<Node>,
    pub links: Vec<Link>,
}

/// One destination the beauty reached, inside a file output node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ADestinationTheBeautyReached {
    /// The node's own name.
    pub node: ArtistsText,
    /// The output it arrived at, **or nothing where the node gathers its
    /// outputs into one file**, in which case the destination is the whole node
    /// (I7).
    pub output: Option<ArtistsText>,
    /// **R4 item 2 and B4, from 5.3: the effect whose group holds the node**, so
    /// the main output is the right one of two nodes sharing a name. Absent
    /// before 5.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub the_effect: Option<ArtistsText>,
}

/// **B4: the walk records one of three declared outcomes, and that is how
/// failure is detected.**
///
/// *"It is a stated result of the reading, never an empty answer inferred from a
/// missing one."*
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WhatTheWalkFound {
    /// **The walk arrived at one or more destinations, and at the final output
    /// node or not.** Those destinations are the main-output candidates; **the
    /// regular output path is among them exactly when the walk reached the final
    /// output node.**
    Reached {
        destinations: Vec<ADestinationTheBeautyReached>,
        it_reached_the_final_output_node: bool,
    },
    /// **The walk met a node it cannot pass, naming that node.** No main output.
    Blocked { the_node_it_could_not_pass: ArtistsText },
    /// **No Render Layers node for the job's scene and view layer, or the
    /// compositor does not run and B5 applies instead.** No main output, unless
    /// B5.
    NoEntry,
}

/// **B5: where the compositor does not run at all, the raw render lands at the
/// regular output path and no walk is needed** — the beauty reaches it by
/// definition, and that destination is the main output.
///
/// *"That is the *No file output nodes* and *nodes carrying only utility passes*
/// case in one rule."*
///
/// **B6: where a sequencer strip produces the output, the scene's own compositor
/// result never reaches the scene's own file**, so the walk does not deliver the
/// regular output path as a destination the beauty reaches. Observed as data out
/// of the reader: `sequencer_takes_over: true, compositor_runs: false`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WhatTheMainOutputIs {
    /// One or more destinations inside file output nodes. **B7: where the beauty
    /// reaches a destination inside a file output node, that destination wins
    /// over the regular output path**, whatever it is also writing. *"That is
    /// the Creator's decision about which output matters and it is applied, not
    /// re-derived."*
    ADestinationInsideAFileOutputNode,
    /// The regular output path — B5's case, or a walk that reached the final
    /// output node and no node destination.
    TheRegularOutputPath,
    /// **B8: no main output means *show outputs* opens the job's own folder and
    /// the preview falls to the last image and nothing better.** *"It is not a
    /// safety net; it is what every job whose walk did not get through gets,
    /// which is why B4 makes the outcome explicit."*
    ThereIsNone,
}

/// What the walk is asked about, beside the tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhereTheRenderEnters {
    /// The job's own scene.
    pub scene: ArtistsText,
    /// The view layer the beauty is followed from.
    pub view_layer: ArtistsText,
}

/// **B1 and B3, performed: the beauty followed forward by wiring.**
///
/// **Nothing marks the picture once it is travelling**, so every step below is a
/// link and never a recognition. **Each of B3's four limits is handled by the
/// walk stopping rather than guessing**, except the first, which steps into the
/// group's own tree.
///
/// **B2 is why this takes a tree rather than a machine**: the walk reads back out
/// of a saved file with no window at all, complete, in every version, so it runs
/// inside the reading and is exercised here with the tree as a value.
pub fn walk_the_beauty(
    tree: &TheCompositingTree,
    enters_at: &WhereTheRenderEnters,
    version: TheVersionInHand,
) -> WhatTheWalkFound {
    let Some(start) = the_beautys_own_socket(tree, enters_at) else {
        // **B4's *no entry*: no Render Layers node for the job's scene and view
        // layer.** It is a declared outcome and not an empty answer.
        return WhatTheWalkFound::NoEntry;
    };
    let mut found = Found::default();
    // **A tree can be wired in a ring and a walk that did not remember where it
    // had been would never end.** Nothing in the plan promises a tree without
    // one, and a walk that runs for ever inside a reading is a job that never
    // gets submitted.
    let mut already_walked = Vec::new();
    follow(tree, start, version, &mut found, &mut already_walked);
    found.into_outcome()
}

/// **Whether a tree as read is 5.3's stack of effects** rather than one tree.
pub fn it_is_a_stack_of_effects(tree: &TheCompositingTree) -> bool {
    tree.nodes.iter().any(|node| matches!(node.kind, NodeKind::AnEffect { .. }))
}

/// **B1a: the walk through a scene's effects, as one chain.**
///
/// Several effects run in list order, each taking the one before's result, and
/// an effect off for render is left out (*The compositor 1*), so
/// `the_effects_on` is what stands at the job's first frame, which is the frame
/// B1 enters at.
///
/// In each effect a Render Layers node reading the walk's view layer is an
/// entry as ever. **A top-level Group Input is an entry in the first effect on,
/// and in a later one only where the beauty reached the Group Output of the one
/// before.** An effect with no Group Output stops carrying the beauty (B3): what
/// it hands on is not established. The walk reached the final output node only
/// where the beauty reaches the Group Output of the last effect on.
///
/// **Caution (I8, `Frames 9`, the runner's ruling).** The entry is taken for
/// whichever view layer the walk is on, though it carried the first view
/// layer's picture where it was seen. Which layer it carries where the first is
/// off at a frame is not established: a render with only the second layer on
/// crashed in 5.3 and in 5.2.2 on the same Mac.
pub fn walk_the_beauty_through_the_effects(
    stack: &TheCompositingTree,
    the_effects_on: &[ArtistsText],
    enters_at: &WhereTheRenderEnters,
    version: TheVersionInHand,
) -> WhatTheWalkFound {
    let mut found = Found::default();
    let mut the_group_input_carries_the_beauty = true;
    for node in &stack.nodes {
        let NodeKind::AnEffect { inner } = &node.kind else {
            continue;
        };
        if !the_effects_on.contains(&node.name) {
            continue;
        }
        let mut here = Found::default();
        let mut walked = Vec::new();
        if let Some(start) = the_beautys_own_socket(inner, enters_at) {
            follow(inner, start, version, &mut here, &mut walked);
        }
        if the_group_input_carries_the_beauty {
            for (at, one) in inner.nodes.iter().enumerate() {
                let NodeKind::TheRendersEntry { outputs } = &one.kind else {
                    continue;
                };
                // **B19a: by name and never by position.**
                if let Some(index) =
                    outputs.iter().position(|name| name.as_bytes() == THE_BEAUTYS_SOCKET_NAME)
                {
                    follow(inner, Socket { node: at, index }, version, &mut here, &mut walked);
                }
            }
        }
        the_group_input_carries_the_beauty = here.reached_the_final_output_node;
        found.reached_the_final_output_node = here.reached_the_final_output_node;
        for mut reached in here.destinations {
            reached.the_effect = Some(node.name.clone());
            if !found.destinations.contains(&reached) {
                found.destinations.push(reached);
            }
        }
        if let Some(named) = here.could_not_pass {
            found.could_not_pass.get_or_insert(named);
        }
    }
    found.into_outcome()
}

/// What the walk has collected so far.
#[derive(Debug, Default)]
struct Found {
    destinations: Vec<ADestinationTheBeautyReached>,
    reached_the_final_output_node: bool,
    /// **The first node the walk could not pass**, which B4's *blocked* names.
    could_not_pass: Option<ArtistsText>,
    /// **Which of the group's own output sockets the beauty arrived at**, filled
    /// only by a walk inside a group's tree and read by the caller stepping back
    /// out. Added 2026-09-05, out of a code round.
    the_groups_own_output_sockets_the_beauty_arrived_at: Vec<usize>,
}

impl Found {
    fn into_outcome(self) -> WhatTheWalkFound {
        if !self.destinations.is_empty() || self.reached_the_final_output_node {
            return WhatTheWalkFound::Reached {
                destinations: self.destinations,
                it_reached_the_final_output_node: self.reached_the_final_output_node,
            };
        }
        match self.could_not_pass {
            Some(node) => WhatTheWalkFound::Blocked {
                the_node_it_could_not_pass: node,
            },
            None => WhatTheWalkFound::NoEntry,
        }
    }
}

/// **B1: the socket the beauty comes out of**, found by the name `Image` on
/// the Render Layers node that names the job's scene and view layer.
///
/// **This matched by `Combined` until 2026-09-20**, which is the pass's name in
/// a file and not the socket's, so it matched nothing on any real file and
/// every walk came back `NoEntry`.
fn the_beautys_own_socket(
    tree: &TheCompositingTree,
    enters_at: &WhereTheRenderEnters,
) -> Option<Socket> {
    for (at, node) in tree.nodes.iter().enumerate() {
        let NodeKind::RenderLayers {
            scene,
            view_layer,
            outputs,
        } = &node.kind
        else {
            continue;
        };
        if scene.as_bytes() != enters_at.scene.as_bytes()
            || view_layer.as_bytes() != enters_at.view_layer.as_bytes()
        {
            continue;
        }
        // **B19a: by name and never by position.**
        let index = outputs
            .iter()
            .position(|name| name.as_bytes() == THE_BEAUTYS_SOCKET_NAME)?;
        return Some(Socket { node: at, index });
    }
    None
}

/// One step of the walk: from an output socket, along every link leaving it.
fn follow(
    tree: &TheCompositingTree,
    from: Socket,
    version: TheVersionInHand,
    found: &mut Found,
    already_walked: &mut Vec<(usize, usize)>,
) {
    if already_walked.contains(&(from.node, from.index)) {
        return;
    }
    already_walked.push((from.node, from.index));
    for link in tree.links.iter().filter(|link| link.from == from) {
        arrive(tree, link.to, version, found, already_walked);
    }
}

/// The beauty arrives at one input socket of one node.
fn arrive(
    tree: &TheCompositingTree,
    at: Socket,
    version: TheVersionInHand,
    found: &mut Found,
    already_walked: &mut Vec<(usize, usize)>,
) {
    let Some(node) = tree.nodes.get(at.node) else {
        return;
    };
    match &node.kind {
        NodeKind::TheFinalOutputNode => {
            found.reached_the_final_output_node = true;
        }
        NodeKind::AFileOutputNode {
            inputs,
            gathering_into_one_file,
            muted,
        } => {
            // **A muted node is no destination: the walk ends here and counts
            // nothing** (R4 item 2; §8.1 after B9). Before 2026-10-18 it was
            // counted, so a video-only file was told it also wrote frames, and
            // B9's warning and B5's main output could count a place nothing is
            // written to.
            if *muted {
                return;
            }
            // **I7: one destination for the whole node where it gathers its
            // outputs into a single multi-layer file, otherwise the output the
            // beauty arrived at.**
            let output = if *gathering_into_one_file {
                None
            } else {
                inputs.get(at.index).cloned()
            };
            let destination = ADestinationTheBeautyReached {
                the_effect: None,
                node: node.name.clone(),
                output,
            };
            if !found.destinations.contains(&destination) {
                found.destinations.push(destination);
            }
        }
        NodeKind::ANodeGroup { inner } => {
            // **B3's first limit: a link cannot be walked across the boundary of
            // a node group, so the walk steps into the group's own tree**, at
            // the group's own input, on the socket it entered on.
            let Some(inside) = inner
                .nodes
                .iter()
                .position(|node| matches!(node.kind, NodeKind::TheGroupsOwnInput))
            else {
                // A group with no input inside it carries the picture nowhere.
                // **Stated as blocked rather than passed through.**
                remember_it_could_not_pass(found, node);
                return;
            };
            let mut inner_found = Found::default();
            let mut inner_walked = Vec::new();
            follow(
                inner,
                Socket {
                    node: inside,
                    index: at.index,
                },
                version,
                &mut inner_found,
                &mut inner_walked,
            );
            // **What the group's own tree found is the walk's**, and where the
            // picture leaves through the group's own output the walk comes back
            // out at the matching socket of the group node itself.
            found.destinations.extend(inner_found.destinations);
            found.reached_the_final_output_node |= inner_found.reached_the_final_output_node;
            if let Some(named) = inner_found.could_not_pass {
                found.could_not_pass.get_or_insert(named);
            }
            // **Only the sockets the beauty actually arrived at.** Corrected
            // 2026-09-05, out of a code round: the walk used to list *every*
            // link arriving at the group's own output node, whatever fed them,
            // and carry on outside as though the beauty came out on each — so a
            // group taking two pictures, the beauty ending at a file output
            // node inside and the group's own output fed by a depth pass, came
            // back *reached, and it reached the final output node*, and the
            // regular output path joined the main-output candidates on the
            // strength of a picture that is not the beauty. B9 reads the
            // duplicate-picture warning off this walk and nothing else, so the
            // user was told before spending money that the same picture is
            // written to more than one place when it is not.
            //
            // **Where the beauty reached none of them the group carries the
            // picture nowhere and the walk stops here**, which is what B3 asks
            // of every limit.
            for leaving in inner_found.the_groups_own_output_sockets_the_beauty_arrived_at {
                follow(
                    tree,
                    Socket {
                        node: at.node,
                        index: leaving,
                    },
                    version,
                    found,
                    already_walked,
                );
            }
        }
        NodeKind::TheGroupsOwnOutput => {
            // **Which of the group's own output sockets the beauty arrived at**,
            // which is the one thing that says where it leaves the group. The
            // caller steps back out on these and on no others.
            if !found
                .the_groups_own_output_sockets_the_beauty_arrived_at
                .contains(&at.index)
            {
                found
                    .the_groups_own_output_sockets_the_beauty_arrived_at
                    .push(at.index);
            }
        }
        NodeKind::OnePictureInOnePictureOut => {
            // One picture in and one out: nothing has to be told apart.
            follow(
                tree,
                Socket {
                    node: at.node,
                    index: 0,
                },
                version,
                found,
                already_walked,
            );
        }
        NodeKind::ATwoPictureNode {
            the_factor_sits_between_its_ends,
            which_input_came_out,
        } => {
            // **B3's third limit, and it is checked first.** Where the factor
            // sits between its ends both inputs contribute and nothing says
            // which contributed what.
            if *the_factor_sits_between_its_ends {
                remember_it_could_not_pass(found, node);
                return;
            }
            // **B3's fourth limit.** Where the kind is not one of the five, the
            // reading records nothing and the walk stops.
            let Some(came_out) = which_input_came_out else {
                remember_it_could_not_pass(found, node);
                return;
            };
            // **B3's second limit: which socket the first input sits on moves
            // with the version.** The beauty arrived at one socket; whether that
            // socket is the *first* input is the version's answer, and only then
            // can it be compared with which input came out.
            let arrived_at_the_first_input = the_side_the_beauty_arrived_on(at.index)
                == WhichSide::the_first_input_of_a_splitting_node_in(version);
            let it_is_the_one_that_came_out = match came_out {
                WhichInputCameOut::TheFirst => arrived_at_the_first_input,
                WhichInputCameOut::TheSecond => !arrived_at_the_first_input,
            };
            if !it_is_the_one_that_came_out {
                // The beauty went into the input that did not come out. **The
                // walk stops rather than guessing that the result still carries
                // it.**
                remember_it_could_not_pass(found, node);
                return;
            }
            follow(
                tree,
                Socket {
                    node: at.node,
                    index: 0,
                },
                version,
                found,
                already_walked,
            );
        }
        NodeKind::RenderLayers { .. }
        | NodeKind::TheGroupsOwnInput
        | NodeKind::TheRendersEntry { .. }
        | NodeKind::AnEffect { .. } => {
            // Neither takes a picture in. A link arriving at one is not
            // something the walk can carry on from.
            remember_it_could_not_pass(found, node);
        }
        NodeKind::ANodeTheWalkCannotPass => remember_it_could_not_pass(found, node),
    }
}

/// **B4's *blocked* names the node**, and the first one met is the one named.
fn remember_it_could_not_pass(found: &mut Found, node: &Node) {
    found.could_not_pass.get_or_insert_with(|| node.name.clone());
}

/// Which of the two sockets of a two-picture node the beauty arrived on.
///
/// **The file holds two input sockets in its own order and socket 0 is the left
/// one.** Which of the two is the node's *first* input is the version's answer
/// and never the file's, which is what B3's second limit says.
fn the_side_the_beauty_arrived_on(index: usize) -> WhichSide {
    if index == 0 {
        WhichSide::Left
    } else {
        WhichSide::Right
    }
}

/// **B4, B5, B6 and B7 read together: what the main output is.**
///
/// **B7 is applied and not re-derived**: a destination inside a file output node
/// wins over the regular output path, whatever it is also writing. **B5** is
/// what a job with no compositor gets. **B6** is why a sequencer-driven job does
/// not get the regular output path off the walk.
///
/// **B5 and B4's table both hold only where the scene's own frames are
/// written**, which is D12's switch — the one §7 has always read and §8 never
/// did. The plan was repaired on 2026-09-05, out of a code round, and this
/// follows it: **where the switch is off the regular output path is no candidate
/// at all**, so a job whose beauty reaches no file output node falls to B8's *no
/// main output*. `What a job writes` already promised that outcome in terms —
/// a job can have no main output *"as well as in Blender 5.2 with the regular
/// output path switched off"*, *"an ordinary, working setup, not a fault"* — so
/// this is the plan catching up rather than a choice about the product. **On
/// such a job *show outputs* opens the job's own folder rather than one that
/// stays empty for the job's life**, and B9's warning stops counting a place
/// nothing is written to.
pub fn what_the_main_output_is(
    the_compositor_runs: bool,
    the_scenes_own_frames_are_written: bool,
    a_sequencer_strip_produces_the_output: bool,
    walk: &WhatTheWalkFound,
) -> WhatTheMainOutputIs {
    if a_sequencer_strip_produces_the_output {
        // **B6.** The scene's own compositor result never reaches the scene's
        // own file, so nothing here delivers the regular output path.
        if let WhatTheWalkFound::Reached { destinations, .. } = walk {
            if !destinations.is_empty() {
                return WhatTheMainOutputIs::ADestinationInsideAFileOutputNode;
            }
        }
        return WhatTheMainOutputIs::ThereIsNone;
    }
    if !the_compositor_runs {
        // **B5.** The raw render lands at the regular output path and no walk is
        // needed — the beauty reaches it by definition, **where the scene's own
        // frames are written at all**.
        return if the_scenes_own_frames_are_written {
            WhatTheMainOutputIs::TheRegularOutputPath
        } else {
            WhatTheMainOutputIs::ThereIsNone
        };
    }
    match walk {
        WhatTheWalkFound::Reached {
            destinations,
            it_reached_the_final_output_node,
        } => {
            if !destinations.is_empty() {
                WhatTheMainOutputIs::ADestinationInsideAFileOutputNode
            } else if *it_reached_the_final_output_node && the_scenes_own_frames_are_written {
                // **B4's table**: the regular output path is among the
                // candidates exactly when the walk reached the final output node
                // and the scene's own frames are written.
                WhatTheMainOutputIs::TheRegularOutputPath
            } else {
                WhatTheMainOutputIs::ThereIsNone
            }
        }
        WhatTheWalkFound::Blocked { .. } | WhatTheWalkFound::NoEntry => {
            WhatTheMainOutputIs::ThereIsNone
        }
    }
}

/// **The places the beauty is written as images**, as B9's walk found them —
/// so the duplicate-picture line can name them (the runner's ruling of
/// 2026-10-12, *The duplicate-picture warning names what it found*;
/// `Design/Render Settings § Warnings`).
///
/// **The regular output path is a place only where it writes images**: a video
/// there is not a pass at all (B9's dated paragraph of 2026-10-12, the
/// Creator's eighth notes, note 4).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThePlacesTheBeautyIsWritten {
    /// **How many File Output nodes** the beauty reached, each counted once
    /// however many of its inputs it arrives at.
    pub file_output_nodes: usize,
    /// **How many of their inputs** it arrives at — a node gathering into one
    /// file is one. Never fewer than [`Self::file_output_nodes`].
    pub their_inputs: usize,
    /// **The regular output path**, where the walk reached the final output
    /// node and that path writes images.
    pub the_output_path: bool,
}

impl ThePlacesTheBeautyIsWritten {
    /// How many places, all told.
    pub fn how_many(&self) -> usize {
        self.their_inputs + usize::from(self.the_output_path)
    }
}

/// **B9's places, read off the walk and off nothing else.**
pub fn the_places_the_beauty_is_written(
    walk: &WhatTheWalkFound,
    the_output_path_writes_a_video: bool,
) -> ThePlacesTheBeautyIsWritten {
    match walk {
        WhatTheWalkFound::Reached {
            destinations,
            it_reached_the_final_output_node,
        } => {
            let mut nodes: Vec<&ArtistsText> = Vec::new();
            for reached in destinations {
                if !nodes.contains(&&reached.node) {
                    nodes.push(&reached.node);
                }
            }
            ThePlacesTheBeautyIsWritten {
                file_output_nodes: nodes.len(),
                their_inputs: destinations.len(),
                the_output_path: *it_reached_the_final_output_node
                    && !the_output_path_writes_a_video,
            }
        }
        _ => ThePlacesTheBeautyIsWritten::default(),
    }
}

/// **B9: the duplicate-picture warning is read off this same walk and off
/// nothing else**, so Smoozy warns only where the beauty really does reach more
/// than one destination — **and only destinations writing images count**: a
/// video at the regular output path is not the same picture written again
/// (B9's dated paragraph of 2026-10-12).
pub fn the_beauty_reaches_more_than_one_destination(
    walk: &WhatTheWalkFound,
    the_output_path_writes_a_video: bool,
) -> bool {
    the_places_the_beauty_is_written(walk, the_output_path_writes_a_video).how_many() > 1
}

/// **Frames and a video, told as two outputs** — B9's dated paragraph of
/// 2026-10-12: the walk reached the final output node, that path writes a
/// video, and at least one File Output node carries the beauty as frames.
pub fn the_render_reaches_frames_and_a_video(
    walk: &WhatTheWalkFound,
    the_output_path_writes_a_video: bool,
) -> bool {
    match walk {
        WhatTheWalkFound::Reached {
            destinations,
            it_reached_the_final_output_node,
        } => *it_reached_the_final_output_node
            && the_output_path_writes_a_video
            && !destinations.is_empty(),
        _ => false,
    }
}

/// **B12: the split between working and viewing formats is applied and not built
/// for.**
///
/// *"Anything that is not PNG or JPEG is a working format; where only those two
/// are present PNG is the working one; where only one format is present it is
/// both."* **That is the Creator's ruling of 2026-08-13 in the spec, and Row 297
/// closed there**, so nothing here re-decides it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatClass {
    Viewing,
    Working,
    /// **Where only one format is present it is both.**
    Both,
}

/// The two spellings the ruling names. **They are the ruling's own words** and
/// are compared as bytes, like every other name in this crate.
pub const PNG: &[u8] = b"PNG";
/// The other one the ruling names.
pub const JPEG: &[u8] = b"JPEG";

/// **B12, performed over the formats a job's destinations carry.**
///
/// The class of one format is decided against the whole set present, because the
/// ruling's second and third sentences are both about what else is there.
pub fn the_class_of(one: &[u8], every_format_present: &[Vec<u8>]) -> FormatClass {
    let distinct: Vec<&Vec<u8>> = {
        let mut out: Vec<&Vec<u8>> = Vec::new();
        for format in every_format_present {
            if !out.iter().any(|already| already.as_slice() == format.as_slice()) {
                out.push(format);
            }
        }
        out
    };
    if distinct.len() <= 1 {
        return FormatClass::Both;
    }
    let only_the_two = distinct
        .iter()
        .all(|format| format.as_slice() == PNG || format.as_slice() == JPEG);
    if only_the_two {
        // **Where only those two are present PNG is the working one.**
        return if one == PNG {
            FormatClass::Working
        } else {
            FormatClass::Viewing
        };
    }
    if one == PNG || one == JPEG {
        FormatClass::Viewing
    } else {
        FormatClass::Working
    }
}

/// **B11 and B13: the two picks, made once, at the moment the recorded reading
/// is composed, and written into the job record on the machine as part of that
/// reading.**
///
/// **B15: nothing anywhere recomputes a pick.** *"A pick recomputed at each
/// opening satisfies every other sentence in the spec's section and breaks this
/// one, and nothing would show it but a user noticing the preview changed."*
///
/// **Held by the type.** There is no method here that takes a job and produces a
/// pick: a pick is composed once by [`ThePicks::made_once`] and after that it is
/// read. **B14: the desktop takes both into the job's history record the first
/// time it learns them, and after that the history record is authoritative.**
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThePicks {
    /// Which candidate is the main output **where two or more are left in the
    /// same format class**. Absent where the job has no main output — B15's
    /// *"where the desktop never learned a pick and the machine is gone, the job
    /// has no main output"* and falls to B8.
    pub the_main_output: Option<ADestinationTheBeautyReached>,
    /// Which file the preview reads **where two are left after every other
    /// rule**, as its predicted path under the job output folder.
    pub the_preview_source: Option<Vec<u8>>,
}

impl ThePicks {
    /// **Made once, at the moment the recorded reading is composed** (B13).
    ///
    /// **Arbitrary is allowed and unstable is not** (B11): where two or more
    /// candidates are left the pick is arbitrary, and this takes the first the
    /// reading holds, which is the file's own order. What matters is that it is
    /// taken once.
    pub fn made_once(
        the_main_output: Option<ADestinationTheBeautyReached>,
        the_preview_source: Option<Vec<u8>>,
    ) -> ThePicks {
        ThePicks {
            the_main_output,
            the_preview_source,
        }
    }
}

/// **B15, said where it can be found.**
///
/// A pick is composed by [`ThePicks::made_once`] and by nothing else in this
/// crate, and no function anywhere takes a job and derives one a second time.
pub const NOTHING_ANYWHERE_RECOMPUTES_A_PICK: bool = true;

/// **B14: the history record is where a pick lives after the desktop learns
/// it.**
///
/// *"That is step 4 of `State, identity and recovery` §7 — filling in what the
/// local record does not hold from the machine's own notes — and it needs no
/// mechanism of its own."* **The history record is user-held**, so the pick
/// lives as long as the entry and survives the machine being destroyed, which is
/// the ordinary end of a job.
pub const THE_DESKTOP_LEARNS_A_PICK_BY_THE_FILL_IN_AND_BY_NO_MECHANISM_OF_ITS_OWN: bool = true;

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) fn text(of: &str) -> ArtistsText {
        ArtistsText::of(of.as_bytes().to_vec())
    }

    pub(crate) fn enters_at() -> WhereTheRenderEnters {
        WhereTheRenderEnters {
            scene: text("Scene"),
            view_layer: text("ViewLayer"),
        }
    }

    fn render_layers() -> Node {
        Node {
            name: text("Render Layers"),
            kind: NodeKind::RenderLayers {
                scene: text("Scene"),
                view_layer: text("ViewLayer"),
                // **The beauty's socket is not first here**, so a walk reading
                // by position takes the wrong picture. **A real node puts
                // `Image` first** — the four Blenders all do — and this order
                // is the test's own, to make the position reading fail.
                outputs: vec![text("Depth"), text("Image"), text("DiffCol")],
            },
        }
    }

    fn from_the_beautys_socket(to: Socket) -> Link {
        Link {
            from: Socket { node: 0, index: 1 },
            to,
        }
    }

    /// **B1's socket, by the name the four Blenders print.**
    ///
    /// **It looked for `Combined` until 2026-09-20** — the pass's name in a
    /// file, which is not the socket's — and so matched nothing on any real
    /// file. This test passed on the whole time, because every tree it was
    /// given carried the name the code was looking for.
    #[test]
    fn the_beauty_leaves_on_the_socket_named_image_and_never_on_the_first_one() {
        // B1 and B19a: found by its name and never by its position.
        let tree = TheCompositingTree {
            nodes: vec![
                render_layers(),
                Node {
                    name: text("Composite"),
                    kind: NodeKind::TheFinalOutputNode,
                },
            ],
            links: vec![from_the_beautys_socket(Socket { node: 1, index: 0 })],
        };
        assert_eq!(
            walk_the_beauty(&tree, &enters_at(), TheVersionInHand::of(5, 2)),
            WhatTheWalkFound::Reached {
                destinations: Vec::new(),
                it_reached_the_final_output_node: true,
            }
        );

        // And with the socket names moved about, the walk follows the name.
        let mut moved = tree.clone();
        moved.nodes[0].kind = NodeKind::RenderLayers {
            scene: text("Scene"),
            view_layer: text("ViewLayer"),
            outputs: vec![text("Image"), text("Depth")],
        };
        moved.links = vec![Link {
            from: Socket { node: 0, index: 0 },
            to: Socket { node: 1, index: 0 },
        }];
        assert!(matches!(
            walk_the_beauty(&moved, &enters_at(), TheVersionInHand::of(5, 2)),
            WhatTheWalkFound::Reached { .. }
        ));
    }

    #[test]
    fn no_render_layers_node_for_the_jobs_scene_is_the_declared_no_entry_outcome() {
        // B4: a stated result, never an empty answer inferred from a missing
        // one.
        let tree = TheCompositingTree {
            nodes: vec![Node {
                name: text("Render Layers"),
                kind: NodeKind::RenderLayers {
                    scene: text("Another"),
                    view_layer: text("ViewLayer"),
                    outputs: vec![text("Image")],
                },
            }],
            links: Vec::new(),
        };
        assert_eq!(
            walk_the_beauty(&tree, &enters_at(), TheVersionInHand::of(4, 5)),
            WhatTheWalkFound::NoEntry
        );
    }

    #[test]
    fn a_node_the_walk_cannot_pass_is_blocked_and_names_the_node() {
        // B3's fourth limit and B4's blocked outcome.
        let tree = TheCompositingTree {
            nodes: vec![
                render_layers(),
                Node {
                    name: text("Glare"),
                    kind: NodeKind::ANodeTheWalkCannotPass,
                },
            ],
            links: vec![from_the_beautys_socket(Socket { node: 1, index: 0 })],
        };
        assert_eq!(
            walk_the_beauty(&tree, &enters_at(), TheVersionInHand::of(5, 2)),
            WhatTheWalkFound::Blocked {
                the_node_it_could_not_pass: text("Glare"),
            }
        );
    }

    #[test]
    fn a_factor_between_its_ends_stops_the_walk_however_the_kind_is_known() {
        // B3's third limit: both inputs contribute and nothing says which
        // contributed what. It is checked before the fourth limit, so a node
        // whose kind is one of the five still stops.
        let tree = TheCompositingTree {
            nodes: vec![
                render_layers(),
                Node {
                    name: text("Mix"),
                    kind: NodeKind::ATwoPictureNode {
                        the_factor_sits_between_its_ends: true,
                        which_input_came_out: Some(WhichInputCameOut::TheFirst),
                    },
                },
                Node {
                    name: text("Composite"),
                    kind: NodeKind::TheFinalOutputNode,
                },
            ],
            links: vec![
                from_the_beautys_socket(Socket { node: 1, index: 0 }),
                Link {
                    from: Socket { node: 1, index: 0 },
                    to: Socket { node: 2, index: 0 },
                },
            ],
        };
        assert_eq!(
            walk_the_beauty(&tree, &enters_at(), TheVersionInHand::of(5, 2)),
            WhatTheWalkFound::Blocked {
                the_node_it_could_not_pass: text("Mix"),
            }
        );
    }

    #[test]
    fn a_two_picture_node_of_no_known_kind_stops_the_walk() {
        // B3's fourth limit. **The five kinds are not named in this document and
        // are not named in the code**: absent means the walk stops.
        let tree = TheCompositingTree {
            nodes: vec![
                render_layers(),
                Node {
                    name: text("Some two-picture node"),
                    kind: NodeKind::ATwoPictureNode {
                        the_factor_sits_between_its_ends: false,
                        which_input_came_out: None,
                    },
                },
            ],
            links: vec![from_the_beautys_socket(Socket { node: 1, index: 0 })],
        };
        assert!(matches!(
            walk_the_beauty(&tree, &enters_at(), TheVersionInHand::of(5, 2)),
            WhatTheWalkFound::Blocked { .. }
        ));
    }

    #[test]
    fn which_side_carries_the_first_input_moves_across_the_version_boundary() {
        // B3's second limit. The same file, the same wiring, two versions, two
        // answers — which is the whole reason the walk is composed per version.
        let tree = |arrives_at: usize| TheCompositingTree {
            nodes: vec![
                render_layers(),
                Node {
                    name: text("Split"),
                    kind: NodeKind::ATwoPictureNode {
                        the_factor_sits_between_its_ends: false,
                        which_input_came_out: Some(WhichInputCameOut::TheFirst),
                    },
                },
                Node {
                    name: text("Composite"),
                    kind: NodeKind::TheFinalOutputNode,
                },
            ],
            links: vec![
                from_the_beautys_socket(Socket {
                    node: 1,
                    index: arrives_at,
                }),
                Link {
                    from: Socket { node: 1, index: 0 },
                    to: Socket { node: 2, index: 0 },
                },
            ],
        };
        assert_eq!(
            WhichSide::the_first_input_of_a_splitting_node_in(TheVersionInHand::of(4, 5)),
            WhichSide::Right
        );
        assert_eq!(
            WhichSide::the_first_input_of_a_splitting_node_in(TheVersionInHand::of(5, 0)),
            WhichSide::Left
        );
        // Arriving on the left socket: the first input from 5.0, not in 4.5.
        assert!(matches!(
            walk_the_beauty(&tree(0), &enters_at(), TheVersionInHand::of(5, 0)),
            WhatTheWalkFound::Reached { .. }
        ));
        assert!(matches!(
            walk_the_beauty(&tree(0), &enters_at(), TheVersionInHand::of(4, 5)),
            WhatTheWalkFound::Blocked { .. }
        ));
    }

    #[test]
    fn the_walk_steps_into_a_node_groups_own_tree() {
        // B3's first limit: a link cannot be walked across the boundary, so the
        // walk steps in. Without this the group is a wall and every grouped
        // tree reports no main output.
        let inner = TheCompositingTree {
            nodes: vec![
                Node {
                    name: text("Group Input"),
                    kind: NodeKind::TheGroupsOwnInput,
                },
                Node {
                    name: text("Inner file output"),
                    kind: NodeKind::AFileOutputNode {
                        inputs: vec![text("Beauty")],
                        gathering_into_one_file: false,
                        muted: false,
                    },
                },
            ],
            links: vec![Link {
                from: Socket { node: 0, index: 0 },
                to: Socket { node: 1, index: 0 },
            }],
        };
        let tree = TheCompositingTree {
            nodes: vec![
                render_layers(),
                Node {
                    name: text("Group"),
                    kind: NodeKind::ANodeGroup { inner },
                },
            ],
            links: vec![from_the_beautys_socket(Socket { node: 1, index: 0 })],
        };
        assert_eq!(
            walk_the_beauty(&tree, &enters_at(), TheVersionInHand::of(5, 2)),
            WhatTheWalkFound::Reached {
                destinations: vec![ADestinationTheBeautyReached {
                    the_effect: None,
                    node: text("Inner file output"),
                    output: Some(text("Beauty")),
                }],
                it_reached_the_final_output_node: false,
            }
        );
    }

    #[test]
    fn the_walk_leaves_a_group_only_on_the_sockets_the_beauty_arrived_at() {
        // The walk used to list every link arriving at the group's own output
        // node, whatever fed them, and step back out on each. Here the beauty
        // goes in on socket 0 and ends at a file output node inside; the
        // group's own output is fed by socket 1 — a depth pass. The old walk
        // came back *and it reached the final output node*, so the regular
        // output path joined the main-output candidates on the strength of a
        // picture that is not the beauty, and B9's warning told the user before
        // they spent money that the same picture is written to more than one
        // place when it is not.
        let inner = TheCompositingTree {
            nodes: vec![
                Node {
                    name: text("Group Input"),
                    kind: NodeKind::TheGroupsOwnInput,
                },
                Node {
                    name: text("Inner file output"),
                    kind: NodeKind::AFileOutputNode {
                        inputs: vec![text("Beauty")],
                        gathering_into_one_file: false,
                        muted: false,
                    },
                },
                Node {
                    name: text("Group Output"),
                    kind: NodeKind::TheGroupsOwnOutput,
                },
            ],
            links: vec![
                // The beauty, in on socket 0, to the file output node.
                Link {
                    from: Socket { node: 0, index: 0 },
                    to: Socket { node: 1, index: 0 },
                },
                // The depth pass, in on socket 1, out through the group.
                Link {
                    from: Socket { node: 0, index: 1 },
                    to: Socket { node: 2, index: 0 },
                },
            ],
        };
        let tree = TheCompositingTree {
            nodes: vec![
                render_layers(),
                Node {
                    name: text("Group"),
                    kind: NodeKind::ANodeGroup { inner },
                },
                Node {
                    name: text("Composite"),
                    kind: NodeKind::TheFinalOutputNode,
                },
            ],
            links: vec![
                from_the_beautys_socket(Socket { node: 1, index: 0 }),
                Link {
                    from: Socket { node: 1, index: 0 },
                    to: Socket { node: 2, index: 0 },
                },
            ],
        };
        assert_eq!(
            walk_the_beauty(&tree, &enters_at(), TheVersionInHand::of(5, 2)),
            WhatTheWalkFound::Reached {
                destinations: vec![ADestinationTheBeautyReached {
                    the_effect: None,
                    node: text("Inner file output"),
                    output: Some(text("Beauty")),
                }],
                it_reached_the_final_output_node: false,
            }
        );
    }

    #[test]
    fn the_walk_steps_back_out_where_the_beauty_did_reach_the_groups_own_output() {
        // The other half of the same repair: where the beauty does leave on the
        // group's own output socket, the walk carries on outside exactly there.
        let inner = TheCompositingTree {
            nodes: vec![
                Node {
                    name: text("Group Input"),
                    kind: NodeKind::TheGroupsOwnInput,
                },
                Node {
                    name: text("Group Output"),
                    kind: NodeKind::TheGroupsOwnOutput,
                },
            ],
            links: vec![Link {
                from: Socket { node: 0, index: 0 },
                to: Socket { node: 1, index: 0 },
            }],
        };
        let tree = TheCompositingTree {
            nodes: vec![
                render_layers(),
                Node {
                    name: text("Group"),
                    kind: NodeKind::ANodeGroup { inner },
                },
                Node {
                    name: text("Composite"),
                    kind: NodeKind::TheFinalOutputNode,
                },
            ],
            links: vec![
                from_the_beautys_socket(Socket { node: 1, index: 0 }),
                Link {
                    from: Socket { node: 1, index: 0 },
                    to: Socket { node: 2, index: 0 },
                },
            ],
        };
        assert_eq!(
            walk_the_beauty(&tree, &enters_at(), TheVersionInHand::of(5, 2)),
            WhatTheWalkFound::Reached {
                destinations: Vec::new(),
                it_reached_the_final_output_node: true,
            }
        );
    }

    #[test]
    fn a_beauty_blocked_inside_a_group_reports_blocked_and_not_reached() {
        // The third consequence of the same fault, and the same repair closes
        // it. The beauty meets a node the walk cannot pass inside the group,
        // while another link reaches the group's own output. Stepping out on
        // every socket carried the walk on to the final output node and
        // `Found::into_outcome` reported *reached* before it ever looked at
        // `could_not_pass`, where B4's table says *blocked*.
        let inner = TheCompositingTree {
            nodes: vec![
                Node {
                    name: text("Group Input"),
                    kind: NodeKind::TheGroupsOwnInput,
                },
                Node {
                    name: text("Impassable"),
                    kind: NodeKind::ANodeTheWalkCannotPass,
                },
                Node {
                    name: text("Group Output"),
                    kind: NodeKind::TheGroupsOwnOutput,
                },
            ],
            links: vec![
                Link {
                    from: Socket { node: 0, index: 0 },
                    to: Socket { node: 1, index: 0 },
                },
                Link {
                    from: Socket { node: 0, index: 1 },
                    to: Socket { node: 2, index: 0 },
                },
            ],
        };
        let tree = TheCompositingTree {
            nodes: vec![
                render_layers(),
                Node {
                    name: text("Group"),
                    kind: NodeKind::ANodeGroup { inner },
                },
                Node {
                    name: text("Composite"),
                    kind: NodeKind::TheFinalOutputNode,
                },
            ],
            links: vec![
                from_the_beautys_socket(Socket { node: 1, index: 0 }),
                Link {
                    from: Socket { node: 1, index: 0 },
                    to: Socket { node: 2, index: 0 },
                },
            ],
        };
        assert_eq!(
            walk_the_beauty(&tree, &enters_at(), TheVersionInHand::of(5, 2)),
            WhatTheWalkFound::Blocked {
                the_node_it_could_not_pass: text("Impassable"),
            }
        );
    }

    #[test]
    fn a_gathering_node_is_one_destination_for_the_whole_node() {
        // I7: one file for the whole node, whatever the number of outputs.
        let tree = TheCompositingTree {
            nodes: vec![
                render_layers(),
                Node {
                    name: text("Gathered"),
                    kind: NodeKind::AFileOutputNode {
                        inputs: vec![text("Beauty"), text("Depth")],
                        gathering_into_one_file: true,
                        muted: false,
                    },
                },
            ],
            links: vec![from_the_beautys_socket(Socket { node: 1, index: 0 })],
        };
        assert_eq!(
            walk_the_beauty(&tree, &enters_at(), TheVersionInHand::of(5, 2)),
            WhatTheWalkFound::Reached {
                destinations: vec![ADestinationTheBeautyReached {
                    the_effect: None,
                    node: text("Gathered"),
                    output: None,
                }],
                it_reached_the_final_output_node: false,
            }
        );
    }

    #[test]
    fn b7_makes_a_node_destination_win_over_the_regular_output_path() {
        let walk = WhatTheWalkFound::Reached {
            destinations: vec![ADestinationTheBeautyReached {
                the_effect: None,
                node: text("Beauty"),
                output: Some(text("Image")),
            }],
            it_reached_the_final_output_node: true,
        };
        assert_eq!(
            what_the_main_output_is(true, true, false, &walk),
            WhatTheMainOutputIs::ADestinationInsideAFileOutputNode
        );
        // And with no node destination the final output node delivers the
        // regular output path.
        let only_the_final = WhatTheWalkFound::Reached {
            destinations: Vec::new(),
            it_reached_the_final_output_node: true,
        };
        assert_eq!(
            what_the_main_output_is(true, true, false, &only_the_final),
            WhatTheMainOutputIs::TheRegularOutputPath
        );
    }

    #[test]
    fn b5_gives_a_job_with_no_compositor_the_regular_output_path_with_no_walk() {
        assert_eq!(
            what_the_main_output_is(false, true, false, &WhatTheWalkFound::NoEntry),
            WhatTheMainOutputIs::TheRegularOutputPath
        );
    }

    #[test]
    fn the_regular_output_path_is_no_candidate_where_the_scenes_own_frames_are_not_written() {
        // D12's switch, which §7 has always read and §8 never did. A 5.2 job
        // with the regular output path switched off writes nothing there for
        // its whole life, and both routes to it used to name it the main
        // output: B5's shortcut for a job with no compositor, and B4's table
        // for a walk that reached the final output node. `What a job writes`
        // already promised such a job no main output at all, and called the
        // setup ordinary rather than a fault.
        assert_eq!(
            what_the_main_output_is(false, false, false, &WhatTheWalkFound::NoEntry),
            WhatTheMainOutputIs::ThereIsNone
        );
        let only_the_final = WhatTheWalkFound::Reached {
            destinations: Vec::new(),
            it_reached_the_final_output_node: true,
        };
        assert_eq!(
            what_the_main_output_is(true, false, false, &only_the_final),
            WhatTheMainOutputIs::ThereIsNone
        );
        // A file output node the beauty reached still wins, by B7.
        let a_node = WhatTheWalkFound::Reached {
            destinations: vec![ADestinationTheBeautyReached {
                the_effect: None,
                node: text("Beauty"),
                output: Some(text("Image")),
            }],
            it_reached_the_final_output_node: true,
        };
        assert_eq!(
            what_the_main_output_is(true, false, false, &a_node),
            WhatTheMainOutputIs::ADestinationInsideAFileOutputNode
        );
    }

    #[test]
    fn b6_keeps_the_regular_output_path_out_where_a_sequencer_strip_produces_the_output() {
        // The scene's own compositor result never reaches the scene's own file.
        let reached_the_final = WhatTheWalkFound::Reached {
            destinations: Vec::new(),
            it_reached_the_final_output_node: true,
        };
        assert_eq!(
            what_the_main_output_is(false, true, true, &reached_the_final),
            WhatTheMainOutputIs::ThereIsNone
        );
    }

    #[test]
    fn b8_is_what_a_blocked_walk_gets() {
        assert_eq!(
            what_the_main_output_is(
                true,
                true,
                false,
                &WhatTheWalkFound::Blocked {
                    the_node_it_could_not_pass: text("Glare"),
                }
            ),
            WhatTheMainOutputIs::ThereIsNone
        );
    }

    #[test]
    fn the_duplicate_picture_warning_is_read_off_the_walk_and_off_nothing_else() {
        // B9: Smoozy warns only where the beauty really does reach more than one
        // destination.
        let one = WhatTheWalkFound::Reached {
            destinations: vec![ADestinationTheBeautyReached {
                the_effect: None,
                node: text("Beauty"),
                output: Some(text("Image")),
            }],
            it_reached_the_final_output_node: false,
        };
        assert!(!the_beauty_reaches_more_than_one_destination(&one, false));
        let two = WhatTheWalkFound::Reached {
            destinations: vec![ADestinationTheBeautyReached {
                the_effect: None,
                node: text("Beauty"),
                output: Some(text("Image")),
            }],
            it_reached_the_final_output_node: true,
        };
        assert!(the_beauty_reaches_more_than_one_destination(&two, false));
    }

    /// **The Creator's twelfth notes, note 6, as a tree**: Render Layers feeds
    /// a Reroute on to the final output node, and also a File Output node,
    /// muted or not; the regular output path writes a video.
    fn the_video_file_with_a_file_output_node(muted: bool) -> TheCompositingTree {
        TheCompositingTree {
            nodes: vec![
                render_layers(),
                Node {
                    name: text("Reroute"),
                    kind: NodeKind::OnePictureInOnePictureOut,
                },
                Node {
                    name: text("Group Output"),
                    kind: NodeKind::TheFinalOutputNode,
                },
                Node {
                    name: text("File Output"),
                    kind: NodeKind::AFileOutputNode {
                        inputs: vec![text("Image")],
                        gathering_into_one_file: false,
                        muted,
                    },
                },
            ],
            links: vec![
                from_the_beautys_socket(Socket { node: 1, index: 0 }),
                Link {
                    from: Socket { node: 1, index: 0 },
                    to: Socket { node: 2, index: 0 },
                },
                from_the_beautys_socket(Socket { node: 3, index: 0 }),
            ],
        }
    }

    /// **R4 item 2: a muted node contributes none.** The walk ends at it and
    /// counts nothing, so a file whose only output is a video is not told it
    /// also writes frames, B9's warning is not raised, and the main output is
    /// the regular output path rather than a folder nothing is written to.
    #[test]
    fn a_muted_file_output_node_is_no_destination_of_the_walk() {
        let walk = walk_the_beauty(
            &the_video_file_with_a_file_output_node(true),
            &enters_at(),
            TheVersionInHand::of(5, 1),
        );
        assert_eq!(
            walk,
            WhatTheWalkFound::Reached {
                destinations: Vec::new(),
                it_reached_the_final_output_node: true,
            }
        );
        // The frames-and-a-video line, read with a video at the output path.
        assert!(!the_render_reaches_frames_and_a_video(&walk, true));
        // B9's duplicate-picture warning, with images at the output path too.
        assert!(!the_beauty_reaches_more_than_one_destination(&walk, false));
        assert_eq!(
            the_places_the_beauty_is_written(&walk, false),
            ThePlacesTheBeautyIsWritten {
                file_output_nodes: 0,
                their_inputs: 0,
                the_output_path: true,
            }
        );
        // B5/B7: the main output is not a destination inside the muted node.
        assert_eq!(
            what_the_main_output_is(true, true, false, &walk),
            WhatTheMainOutputIs::TheRegularOutputPath
        );

        // **A muted node that is the beauty's only way out reaches nothing**,
        // and the walk says no destination rather than naming the node.
        let mut alone = the_video_file_with_a_file_output_node(true);
        alone.links.retain(|link| link.to.node == 3);
        let walk = walk_the_beauty(&alone, &enters_at(), TheVersionInHand::of(5, 1));
        assert!(!matches!(walk, WhatTheWalkFound::Reached { .. }), "{walk:?}");
        assert_eq!(
            what_the_main_output_is(true, true, false, &walk),
            WhatTheMainOutputIs::ThereIsNone
        );

        // **The same inside a node group**: the group's tree is walked by the
        // same step, so a muted node there counts nothing either.
        let inner = TheCompositingTree {
            nodes: vec![
                Node {
                    name: text("Group Input"),
                    kind: NodeKind::TheGroupsOwnInput,
                },
                Node {
                    name: text("Inner file output"),
                    kind: NodeKind::AFileOutputNode {
                        inputs: vec![text("Beauty")],
                        gathering_into_one_file: true,
                        muted: true,
                    },
                },
            ],
            links: vec![Link {
                from: Socket { node: 0, index: 0 },
                to: Socket { node: 1, index: 0 },
            }],
        };
        let grouped = TheCompositingTree {
            nodes: vec![
                render_layers(),
                Node {
                    name: text("Group"),
                    kind: NodeKind::ANodeGroup { inner },
                },
            ],
            links: vec![from_the_beautys_socket(Socket { node: 1, index: 0 })],
        };
        assert!(!matches!(
            walk_the_beauty(&grouped, &enters_at(), TheVersionInHand::of(5, 2)),
            WhatTheWalkFound::Reached { .. }
        ));
    }

    /// **The same tree with the node not muted reads as it always did**: one
    /// destination, frames and a video told as two outputs, and B9 raised
    /// where the output path writes images.
    #[test]
    fn an_unmuted_file_output_node_is_a_destination_as_before() {
        let walk = walk_the_beauty(
            &the_video_file_with_a_file_output_node(false),
            &enters_at(),
            TheVersionInHand::of(5, 1),
        );
        assert_eq!(
            walk,
            WhatTheWalkFound::Reached {
                destinations: vec![ADestinationTheBeautyReached {
                    the_effect: None,
                    node: text("File Output"),
                    output: Some(text("Image")),
                }],
                it_reached_the_final_output_node: true,
            }
        );
        assert!(the_render_reaches_frames_and_a_video(&walk, true));
        assert!(!the_beauty_reaches_more_than_one_destination(&walk, true));
        assert!(the_beauty_reaches_more_than_one_destination(&walk, false));
        assert_eq!(
            what_the_main_output_is(true, true, false, &walk),
            WhatTheMainOutputIs::ADestinationInsideAFileOutputNode
        );
    }

    #[test]
    fn b12s_three_sentences_each_decide_a_different_set() {
        let png = PNG.to_vec();
        let jpeg = JPEG.to_vec();
        let exr = b"OPEN_EXR".to_vec();
        // Only one format present: it is both.
        assert_eq!(the_class_of(PNG, std::slice::from_ref(&png)), FormatClass::Both);
        assert_eq!(the_class_of(&exr, std::slice::from_ref(&exr)), FormatClass::Both);
        // Only PNG and JPEG present: PNG is the working one.
        assert_eq!(
            the_class_of(PNG, &[png.clone(), jpeg.clone()]),
            FormatClass::Working
        );
        assert_eq!(
            the_class_of(JPEG, &[png.clone(), jpeg.clone()]),
            FormatClass::Viewing
        );
        // Anything else present: anything that is not PNG or JPEG is working.
        assert_eq!(
            the_class_of(&exr, &[png.clone(), exr.clone()]),
            FormatClass::Working
        );
        assert_eq!(the_class_of(PNG, &[png, exr]), FormatClass::Viewing);
    }

    #[test]
    fn a_ring_in_the_tree_does_not_make_the_walk_run_for_ever() {
        // A reading that never returns is a job that is never submitted, and
        // nothing in the plan promises a tree without a ring in it.
        let tree = TheCompositingTree {
            nodes: vec![
                render_layers(),
                Node {
                    name: text("One"),
                    kind: NodeKind::OnePictureInOnePictureOut,
                },
                Node {
                    name: text("Two"),
                    kind: NodeKind::OnePictureInOnePictureOut,
                },
            ],
            links: vec![
                from_the_beautys_socket(Socket { node: 1, index: 0 }),
                Link {
                    from: Socket { node: 1, index: 0 },
                    to: Socket { node: 2, index: 0 },
                },
                Link {
                    from: Socket { node: 2, index: 0 },
                    to: Socket { node: 1, index: 0 },
                },
            ],
        };
        assert_eq!(
            walk_the_beauty(&tree, &enters_at(), TheVersionInHand::of(5, 2)),
            WhatTheWalkFound::NoEntry
        );
    }

    // -----------------------------------------------------------------------
    // B1a: the stack of effects, from 5.3
    // -----------------------------------------------------------------------

    fn the_entry() -> Node {
        Node {
            name: text("Group Input"),
            kind: NodeKind::TheRendersEntry { outputs: vec![text("Image")] },
        }
    }

    fn the_end() -> Node {
        Node { name: text("Group Output"), kind: NodeKind::TheFinalOutputNode }
    }

    fn a_file_output(named: &str) -> Node {
        Node {
            name: text(named),
            kind: NodeKind::AFileOutputNode {
                inputs: vec![text("Image")],
                gathering_into_one_file: false,
                muted: false,
            },
        }
    }

    fn a_link(from: usize, to: usize) -> Link {
        Link { from: Socket { node: from, index: 0 }, to: Socket { node: to, index: 0 } }
    }

    fn an_effect(named: &str, nodes: Vec<Node>, links: Vec<Link>) -> Node {
        Node { name: text(named), kind: NodeKind::AnEffect { inner: TheCompositingTree { nodes, links } } }
    }

    fn a_stack(effects: Vec<Node>) -> TheCompositingTree {
        TheCompositingTree { nodes: effects, links: Vec::new() }
    }

    fn walked(stack: &TheCompositingTree, on: &[&str]) -> WhatTheWalkFound {
        let on: Vec<ArtistsText> = on.iter().map(|one| text(one)).collect();
        walk_the_beauty_through_the_effects(stack, &on, &enters_at(), TheVersionInHand::of(5, 3))
    }

    /// **The tree 5.3's New button makes**: no Render Layers node, and still a
    /// walk that reaches the final output node.
    #[test]
    fn the_tree_the_new_button_makes_reaches_the_final_output_node() {
        let stack = a_stack(vec![an_effect("Scene Effect", vec![the_entry(), the_end()], vec![a_link(0, 1)])]);
        assert!(it_is_a_stack_of_effects(&stack));
        assert_eq!(
            walked(&stack, &["Scene Effect"]),
            WhatTheWalkFound::Reached { destinations: Vec::new(), it_reached_the_final_output_node: true }
        );
    }

    /// **A node fed from that Group Input is reached, and named by its effect**,
    /// so the same node name in another effect is another destination.
    #[test]
    fn a_node_fed_from_the_group_input_is_reached_and_names_its_effect() {
        let stack = a_stack(vec![
            an_effect("One", vec![the_entry(), a_file_output("File Output")], vec![a_link(0, 1)]),
            an_effect("Two", vec![the_entry(), a_file_output("File Output")], vec![a_link(0, 1)]),
        ]);
        // **The first effect has no Group Output, so the second's Group Input
        // carries no beauty** (B3), and only the first's node is the beauty's.
        assert_eq!(
            walked(&stack, &["One", "Two"]),
            WhatTheWalkFound::Reached {
                destinations: vec![ADestinationTheBeautyReached {
                    the_effect: Some(text("One")),
                    node: text("File Output"),
                    output: Some(text("Image")),
                }],
                it_reached_the_final_output_node: false,
            }
        );
    }

    /// **The Compositor 1: list order, each taking the one before's result.** The
    /// walk reaches the final output node only through the last effect on, and
    /// an effect off is left out of the chain.
    #[test]
    fn the_chain_carries_the_beauty_through_the_effects_on_and_leaves_out_one_off() {
        let passes = |named: &str| an_effect(named, vec![the_entry(), the_end()], vec![a_link(0, 1)]);
        let takes_it = |named: &str| {
            an_effect(named, vec![the_entry(), a_file_output("Later")], vec![a_link(0, 1)])
        };
        let stack = a_stack(vec![passes("First"), takes_it("Middle"), passes("Last")]);
        // All three on: the middle has no Group Output, so the chain stops there.
        let all = walked(&stack, &["First", "Middle", "Last"]);
        assert!(matches!(
            &all,
            WhatTheWalkFound::Reached { destinations, it_reached_the_final_output_node: false } if destinations.len() == 1
        ));
        // The middle off: it is left out, and the beauty reaches the end.
        assert_eq!(
            walked(&stack, &["First", "Last"]),
            WhatTheWalkFound::Reached { destinations: Vec::new(), it_reached_the_final_output_node: true }
        );
        // Every effect off is no entry, and B5 takes it from there.
        assert_eq!(walked(&stack, &[]), WhatTheWalkFound::NoEntry);
    }

    /// **A later effect's Render Layers node is an entry as ever**, whatever the
    /// one before did, and the older walk still knows no stack.
    #[test]
    fn a_later_effects_render_layers_node_is_an_entry_as_ever() {
        let first = an_effect("First", vec![a_file_output("Alone")], Vec::new());
        let the_beautys_place = match &render_layers().kind {
            NodeKind::RenderLayers { outputs, .. } => {
                outputs.iter().position(|name| name.as_bytes() == THE_BEAUTYS_SOCKET_NAME).expect("named")
            }
            _ => unreachable!(),
        };
        let second = an_effect(
            "Second",
            vec![render_layers(), the_end()],
            vec![Link { from: Socket { node: 0, index: the_beautys_place }, to: Socket { node: 1, index: 0 } }],
        );
        let stack = a_stack(vec![first, second]);
        assert_eq!(
            walked(&stack, &["First", "Second"]),
            WhatTheWalkFound::Reached { destinations: Vec::new(), it_reached_the_final_output_node: true }
        );
        assert!(!it_is_a_stack_of_effects(&TheCompositingTree {
            nodes: vec![render_layers()],
            links: Vec::new()
        }));
    }

    /// **A node the walk cannot pass in an effect stops the chain and is named.**
    #[test]
    fn a_node_the_walk_cannot_pass_in_an_effect_blocks_the_chain() {
        let glare = Node { name: text("Glare"), kind: NodeKind::ANodeTheWalkCannotPass };
        let stack = a_stack(vec![
            an_effect("First", vec![the_entry(), glare, the_end()], vec![a_link(0, 1), a_link(1, 2)]),
            an_effect("Second", vec![the_entry(), the_end()], vec![a_link(0, 1)]),
        ]);
        assert_eq!(
            walked(&stack, &["First", "Second"]),
            WhatTheWalkFound::Blocked { the_node_it_could_not_pass: text("Glare") }
        );
    }

    /// **A Group Input inside a nested group stays B3's first limit.**
    #[test]
    fn a_group_input_inside_a_nested_group_is_no_entry_of_its_own() {
        let inner = TheCompositingTree {
            nodes: vec![
                Node { name: text("Inner input"), kind: NodeKind::TheGroupsOwnInput },
                a_file_output("Inside"),
            ],
            links: vec![a_link(0, 1)],
        };
        let group = Node { name: text("Group"), kind: NodeKind::ANodeGroup { inner } };
        let not_fed = a_stack(vec![an_effect("One", vec![group.clone()], Vec::new())]);
        assert_eq!(walked(&not_fed, &["One"]), WhatTheWalkFound::NoEntry);
        let fed = a_stack(vec![an_effect("One", vec![the_entry(), group], vec![a_link(0, 1)])]);
        assert!(matches!(
            walked(&fed, &["One"]),
            WhatTheWalkFound::Reached { destinations, .. }
                if destinations.len() == 1 && destinations[0].the_effect == Some(text("One"))
        ));
    }
}

// =============================================================================
// §8.3 — the preview's candidate list.
// =============================================================================

/// **B17's two named passes, and they are named apart by version.**
///
/// *"The **diffuse colour pass**, which is `DiffCol` in 4.5 and `Diffuse Color`
/// from 5.0 — so the rung needs to know which Blender wrote the file, which the
/// manifest names."*
pub const THE_DIFFUSE_COLOUR_PASS_IN_FOUR_FIVE: &[u8] = b"DiffCol";
/// The same pass, from 5.0.
pub const THE_DIFFUSE_COLOUR_PASS_FROM_FIVE_ZERO: &[u8] = b"Diffuse Color";

/// The diffuse colour pass's name in the version that wrote the file.
pub fn the_diffuse_colour_passes_name(version: TheVersionInHand) -> &'static [u8] {
    match version.branch() {
        WhichBranch::FourFive => THE_DIFFUSE_COLOUR_PASS_IN_FOUR_FIVE,
        WhichBranch::FromFiveZero => THE_DIFFUSE_COLOUR_PASS_FROM_FIVE_ZERO,
    }
}

/// **B17's three rungs, in order.**
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rung {
    /// **The beauty pass.**
    TheBeautyPass,
    /// **Failing that, the diffuse colour pass.**
    TheDiffuseColourPass,
    /// **Failing that, the latest image, whatever pass it turns out to be.**
    TheLatestImageWhateverItIs,
}

/// One file the job has written, as the candidate list sees it.
///
/// **B16: the candidate list is the expected-output inventory filtered to the
/// files that are present, and it is not the main output's folder.** *"The
/// preview reads across everything the job wrote."*
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct APresentFile {
    /// The file's predicted path, which is the inventory entry's own.
    pub path: Vec<u8>,
    /// The frame it belongs to, in the frame order the inventory holds. Absent
    /// for a video destination's entry, which is not a candidate at any rung.
    pub frame: Option<i64>,
    /// **What the file's picture is**, taken off the destination that aims at
    /// it: the beauty where the walk reached that destination, otherwise the
    /// output's own name, which is what B19 chooses a picture inside a
    /// multi-layer file by.
    pub the_picture_it_holds: Option<ArtistsText>,
    /// The destination's format, as the reading records it, and its class
    /// against the whole set present (B12, B18).
    pub format: Vec<u8>,
    /// **B20's first exclusion: an empty file is never a candidate.**
    pub bytes: u64,
    /// Whether the destination aiming at it is video-shaped (B20's separate
    /// exclusion).
    pub is_video_shaped: bool,
    /// Whether it sits in the main output's own folder (B18's tie-break).
    pub it_sits_in_the_main_outputs_folder: bool,
}

/// What the candidate list is asked, beside the files.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheRungsAreToldAboutTheJob {
    /// The version the manifest names, which B17's second rung needs to know.
    pub version: TheVersionInHand,
    /// **B20's second exclusion**: where the render was interrupted, *"the last
    /// frame asked for that is present is not a candidate either, for the same
    /// reason C17 re-renders it"*. Absent where no render was interrupted.
    pub the_last_frame_asked_for_that_is_present: Option<i64>,
}

/// **B16 to B21, performed: the preview's candidates in the order the rungs put
/// them.**
///
/// **B20: a file Smoozy cannot treat as finished is not a candidate at any rung,
/// and the exclusion is exactly the two it can see.** *"The third case is not
/// excludable and is not pretended to be: a file left by a write that ran out of
/// room carries the name the finished frame would have carried and Smoozy does
/// not tell it from a finished one."* **That is the spec's own standing and C15
/// states the same limit from the other side, so the two do not disagree.**
///
/// **B21: where the job has written no image at all there is no preview**, which
/// covers a job that writes nothing and a running job before its first frame
/// lands — so an empty answer here is that case and never a failure.
/// One candidate: the file, **with the rung and the format class it was ranked
/// by**.
///
/// **The two are computed once, here, and carried out with the file.** Added
/// 2026-09-05, out of a code round. The format class depends on the whole set of
/// formats present (B12), so computing it a second time somewhere else is a
/// second place for the two to disagree — and the tie-break below did exactly
/// that, filtering by the exact format name where the sort had ranked by class.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ACandidate {
    pub file: APresentFile,
    /// Which rung of B17 it fell on.
    pub rung: Rung,
    /// Its format's class against the whole set present (B12).
    pub format_class: FormatClass,
}

pub fn the_candidates(
    present: &[APresentFile],
    told: &WhatTheRungsAreToldAboutTheJob,
) -> Vec<ACandidate> {
    let every_format: Vec<Vec<u8>> = {
        let mut out: Vec<Vec<u8>> = Vec::new();
        for file in present {
            if !out.contains(&file.format) {
                out.push(file.format.clone());
            }
        }
        out
    };
    let mut kept: Vec<(Rung, bool, i64, ACandidate)> = Vec::new();
    for file in present {
        // **B20's first exclusion**, and C13's rule from the preview's side:
        // Smoozy refusing to count nothing as something.
        if file.bytes == 0 {
            continue;
        }
        // **B20's separate exclusion.** The rungs read an image, a video file is
        // not one, and B26 has no route for it.
        if file.is_video_shaped {
            continue;
        }
        // **B20's second exclusion.**
        if told.the_last_frame_asked_for_that_is_present.is_some()
            && file.frame == told.the_last_frame_asked_for_that_is_present
        {
            continue;
        }
        let rung = the_rung_of(file, told.version);
        // **B18: within one rank the viewing format is preferred, wherever in
        // the job's folder it sits.**
        let format_class = the_class_of(&file.format, &every_format);
        let viewing = !matches!(format_class, FormatClass::Working);
        kept.push((
            rung,
            viewing,
            file.frame.unwrap_or(i64::MIN),
            ACandidate {
                file: file.clone(),
                rung,
                format_class,
            },
        ));
    }
    // The rungs in order; inside a rung the viewing format first; then the
    // latest frame, which is B17's third rung's own word and is what a running
    // job's preview follows (B31).
    kept.sort_by(|one, other| {
        one.0
            .cmp(&other.0)
            .then(other.1.cmp(&one.1))
            .then(other.2.cmp(&one.2))
    });
    kept.into_iter().map(|(_, _, _, candidate)| candidate).collect()
}

/// Which rung a present file falls on.
///
/// **B19: inside a multi-layer file the picture is chosen by name, which is the
/// same reading as choosing the file.** The beauty is `Combined` under the name
/// of the view layer it came from; a picture written there by a file output node
/// takes the name of the output that fed it. **B19a: by name and never by
/// position**, which is why this compares names and nothing else.
fn the_rung_of(file: &APresentFile, version: TheVersionInHand) -> Rung {
    let Some(picture) = file.the_picture_it_holds.as_ref() else {
        return Rung::TheLatestImageWhateverItIs;
    };
    if picture.as_bytes() == THE_BEAUTY_PASSES_NAME {
        return Rung::TheBeautyPass;
    }
    if picture.as_bytes() == the_diffuse_colour_passes_name(version) {
        return Rung::TheDiffuseColourPass;
    }
    Rung::TheLatestImageWhateverItIs
}

/// **B18's tie-break, and B15's pick behind it.**
///
/// *"Where two files are left after that, the one in the main output's folder is
/// read, which is the walk of §8.1 being read a second time rather than a second
/// walk. Where the job has no main output, or neither file sits there, B13's pick
/// decides."*
/// **The tie set is the same three things the sort ranked by: rung, format class
/// and frame.** Corrected 2026-09-05, out of a code round, where the filter read
/// frame and the **exact format name** and nothing else — wrong in both
/// directions at once.
///
/// - **Too wide.** With no rung in the filter, a same-frame same-format file at
///   a **worse** rung was swept into the tie and could win on sitting in the
///   main output's folder: frame 10 with the beauty as a PNG in `beauty/` and
///   the diffuse colour pass as a PNG in `main/` returned the diffuse colour
///   pass, **permanently**, because B15 makes the pick once and keeps it. The
///   spec is blunt: *"The pass comes first, the format second."*
/// - **Too narrow.** With the exact format name rather than the class, two files
///   of one rung and one frame in different viewing formats never met in the tie
///   set at all — where OpenEXR is present both PNG and JPEG are viewing
///   formats — so the tie came back at one, the main-output-folder check was
///   never reached, and **B18's tiebreak silently did not run**.
///
/// **It is filtered by rung and not by the picture's name**: two different
/// utility passes both fall on the lowest rung, and B18's own word is *"within
/// one rank"*, which is what [`Rung`] is.
pub fn which_file_the_preview_reads<'a>(
    candidates: &'a [ACandidate],
    picks: &ThePicks,
) -> Option<&'a APresentFile> {
    let first = candidates.first()?;
    // Everything of the same standing as the first candidate — same rung, same
    // format class, same frame — is what is left to break a tie between.
    let level: Vec<&ACandidate> = candidates
        .iter()
        .filter(|candidate| {
            candidate.rung == first.rung
                && candidate.format_class == first.format_class
                && candidate.file.frame == first.file.frame
        })
        .collect();
    if level.len() < 2 {
        return Some(&first.file);
    }
    if let Some(in_the_main_outputs_folder) = level
        .iter()
        .find(|candidate| candidate.file.it_sits_in_the_main_outputs_folder)
    {
        return Some(&in_the_main_outputs_folder.file);
    }
    // **B13's pick decides, and it is read rather than made again** (B15).
    if let Some(picked) = picks.the_preview_source.as_ref() {
        if let Some(candidate) = level.iter().find(|candidate| candidate.file.path == *picked) {
            return Some(&candidate.file);
        }
    }
    Some(&first.file)
}

/// **B16's second half: the same list is where the moving picture and the
/// contact sheet get their source.**
///
/// *"Their source is the job's **frame-shaped candidates at the destination the
/// rungs of B17 and B18 chose**, in the frame order the inventory holds; a job
/// with no frame-shaped candidate has neither."* **Producing them is the media
/// worker's** under `Rented-server runtime`, and how many of that source a
/// picture draws on is `Telegram`'s and Design's; **this document names the
/// source and nothing further.**
pub fn the_source_the_media_worker_reads<'a>(
    candidates: &'a [ACandidate],
    picks: &ThePicks,
) -> Vec<&'a APresentFile> {
    let Some(chosen) = which_file_the_preview_reads(candidates, picks) else {
        return Vec::new();
    };
    let mut out: Vec<&APresentFile> = candidates
        .iter()
        .map(|candidate| &candidate.file)
        .filter(|file| {
            file.frame.is_some()
                && file.the_picture_it_holds == chosen.the_picture_it_holds
                && the_folder_of(&file.path) == the_folder_of(&chosen.path)
        })
        .collect();
    // **In the frame order the inventory holds.**
    out.sort_by_key(|file| file.frame);
    out
}

fn the_folder_of(path: &[u8]) -> &[u8] {
    match path.iter().rposition(|byte| *byte == b'/') {
        Some(at) => &path[..at],
        None => &[],
    }
}

// =============================================================================
// §8.4 — the derivatives: one set, made once, on the machine.
// =============================================================================

/// **B22: the still copies are produced once on the rented machine and both
/// readers read the same set.**
///
/// `Queue & History` fetches and holds copies and produces none; `Telegram` reads
/// the same derivatives. **The moving picture and the contact sheet are not in
/// that set and are not defined here** — `The shape` gives them to the media
/// worker, and what this document owns of them is the source file named at
/// [`the_source_the_media_worker_reads`].
pub const THE_STILL_COPIES_ARE_ONE_SET_AND_BOTH_READERS_READ_IT: bool = true;

/// **B23: a derivative is produced on the machine, by the worker driving the
/// Blender the manifest names, headless and needing no connection, and by
/// nothing else, because there is nothing else.**
///
/// **The desktop produces none.** Observed on 2026-08-17: *"outside Blender the
/// machine carries no image tool of any kind, and the system Python carries no
/// imaging library"*; installing one plans six further packages and nothing was
/// installed.
pub const THE_DESKTOP_PRODUCES_NO_DERIVATIVE: bool = true;

/// **B24: what this document declares for the derivative worker.**
///
/// *"Arbitration between a preview or media worker and a render worker is
/// `Rented-server runtime`'s, which owns scheduling and exclusion for every
/// worker class that can contend; what this document declares for it is that a
/// derivative is low priority, cancellable, and never blocks a render."*
///
/// **The declaration is made against that document's own mechanism and is not
/// restated as a second one here** — `Rented-server runtime` A2 already carries
/// it, value for value, as the preview kind's.
///
/// **B24a is the fifth declaration and it is *nothing declared*, which
/// `Rented-server runtime` A1 reads as unlimited.** §12 makes a job's
/// derivatives **partial across derivatives, atomic within one**, so no promise
/// here wants the machine held to one derivative at a time, and a declaration of
/// one would serialise per-frame work against that. **What keeps derivatives out
/// of a render's way is the class and not a count.**
pub const THIS_DOCUMENT_DECLARES_NO_MACHINE_LEVEL_EXCLUSION_FOR_THE_DERIVATIVE_KIND: bool = true;

/// **B26: the route is chosen by the source file's kind, and the split is forced
/// by what was observed rather than preferred.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhichRouteMakesTheCopy {
    /// **A single-picture working format** — single-layer OpenEXR, Radiance HDR
    /// — *"is loaded, scaled and saved by Blender's own image handling, which
    /// behaved identically in all three versions"*.
    BlendersOwnImageHandling,
    /// **A multi-layer file takes the image library bundled inside Blender's own
    /// Python, pointed at Blender's own colour configuration.** *"Loading a
    /// multilayer OpenEXR as an image gives no pixels in all three versions and
    /// the scaling call then silently succeeds and changes nothing, while the
    /// saving call writes at full size."* **The bundled library is the one route
    /// observed to work for multilayer in all three.**
    TheBundledImageLibrary,
}

/// **B26, performed.**
pub fn which_route(the_source_is_multi_layer: bool) -> WhichRouteMakesTheCopy {
    if the_source_is_multi_layer {
        WhichRouteMakesTheCopy::TheBundledImageLibrary
    } else {
        WhichRouteMakesTheCopy::BlendersOwnImageHandling
    }
}

/// **B27: the bundled library is never pointed at anything but Blender's own
/// colour configuration.**
///
/// Observed: *"pointed at nothing it fails or guesses, and the obvious guess a
/// general tool makes is plausible and is **not** what the job was made under;
/// only the route through Blender's own configuration reproduces it."*
///
/// **B25 is what it is pointed at with**: the colour treatment is the job's own
/// view transform, read out of the `.blend` in the reading of §4 and carried in
/// the manifest. *"The file records no treatment, so something has to choose, and
/// the choice is the whole difficulty."* Observed on the same frame under three
/// choices: the job's own transform blew out **none** of the picture, a plain
/// choice blew out **23.7 per cent**, and a copy with no treatment chosen reads
/// as shading gone and surfaces flat clipped colour.
pub const THE_BUNDLED_LIBRARY_IS_ONLY_EVER_POINTED_AT_BLENDERS_OWN_CONFIGURATION: bool = true;

/// **B26a's three caps, and two of the three are published.**
///
/// *"The photo's width and height must not exceed 10000 in total. Width and
/// height ratio must be at most 20."* (`Telegram` I3, reading
/// `https://core.telegram.org/bots/api` on **2026-08-20**.)
pub const THE_WIDTH_AND_HEIGHT_MUST_NOT_EXCEED_IN_TOTAL: u32 = 10_000;
/// The published ratio cap.
pub const THE_WIDEST_RATIO: u32 = 20;
/// **The third, this project's own observed cap** — the same page's *"The photo
/// must be at most 10 MB in size"*, reconciled as ten mebibytes.
pub const THE_LARGEST_A_DERIVATIVE_MAY_BE_IN_BYTES: u64 = 10_485_760;

/// What size a derivative is produced at.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ATargetSize {
    pub width: u32,
    pub height: u32,
    /// **B26a's last sentence.** *"Where the source cannot be brought under the
    /// caps at all — a ratio beyond 20 is the one shape scaling cannot fix — the
    /// derivative is produced as small as it can be made and the fact is
    /// recorded with it, so a consumer refuses knowingly rather than being
    /// surprised."*
    pub it_could_not_be_brought_under_every_cap: bool,
}

/// **B26a, performed: the derivative's scaling has a target rather than only a
/// direction, and the target is the tightest cap any consumer places on it.**
///
/// **This matters because a full-size derivative is demonstrated rather than
/// hypothetical**: on the multi-layer route B26 names, the scaling call silently
/// succeeds and changes nothing while the saving call writes at full size — *"so
/// without a stated target an 8K multi-layer job produces a derivative that no
/// consumer can take, for every frame."*
///
/// **The byte cap is a requirement on the production and not a cap this
/// arithmetic can meet**: nothing here knows what a picture will weigh once it is
/// written, so the produced file is measured against
/// [`THE_LARGEST_A_DERIVATIVE_MAY_BE_IN_BYTES`] after it is written and the
/// production is what has to come under it
/// ([`the_size_to_write_it_at_next`]).
pub fn the_target_size(width: u32, height: u32) -> ATargetSize {
    if width == 0 || height == 0 {
        return ATargetSize {
            width,
            height,
            it_could_not_be_brought_under_every_cap: true,
        };
    }
    let long = width.max(height);
    let short = width.min(height);
    // **A ratio beyond 20 is the one shape scaling cannot fix**, because scaling
    // keeps the ratio.
    let beyond_the_ratio = long > short.saturating_mul(THE_WIDEST_RATIO);
    let total = u64::from(width) + u64::from(height);
    if total <= u64::from(THE_WIDTH_AND_HEIGHT_MUST_NOT_EXCEED_IN_TOTAL) {
        return ATargetSize {
            width,
            height,
            it_could_not_be_brought_under_every_cap: beyond_the_ratio,
        };
    }
    // Scale both sides by the same amount, so the ratio is untouched and the
    // total lands under the cap. The floor is one pixel a side: a derivative of
    // no pixels is not a picture.
    let cap = u64::from(THE_WIDTH_AND_HEIGHT_MUST_NOT_EXCEED_IN_TOTAL);
    let scaled_width = ((u64::from(width) * cap) / total).max(1) as u32;
    let scaled_height = ((u64::from(height) * cap) / total).max(1) as u32;
    ATargetSize {
        width: scaled_width,
        height: scaled_height,
        it_could_not_be_brought_under_every_cap: beyond_the_ratio,
    }
}

/// **B26a, *how the production comes under the byte cap*: how many times one copy
/// is written at most** — its first write and two smaller. A Build figure: the
/// first smaller size lands under the cap on the frames measured, and the last is
/// for a picture whose weight does not follow its area.
pub const THE_MOST_WRITES_OF_ONE_COPY: u32 = 3;

/// **B26a, *how the production comes under the byte cap*: the size a copy over the
/// cap is written at next**, from the size it was written at and what it weighed.
/// A PNG's weight follows its area and only roughly, so each side is scaled by the
/// square root of nine tenths of the cap over the weight, and the ratio is kept.
/// **Nothing where the copy is not over the cap**, or can be made no smaller.
pub fn the_size_to_write_it_at_next(written_at: (u32, u32), it_weighed: u64) -> Option<ATargetSize> {
    let (width, height) = written_at;
    if it_weighed <= THE_LARGEST_A_DERIVATIVE_MAY_BE_IN_BYTES || width == 0 || height == 0 {
        return None;
    }
    let scale = (0.9 * THE_LARGEST_A_DERIVATIVE_MAY_BE_IN_BYTES as f64 / it_weighed as f64).sqrt();
    let scaled = |side: u32| ((f64::from(side) * scale) as u32).max(1);
    let next = (scaled(width), scaled(height));
    (next != written_at).then(|| the_target_size(next.0, next.1))
}

/// **B28: the derivative is written in a viewing format, and that is what removes
/// a question this document cannot answer.**
///
/// *"The fetched copy is opened on the user's own computer, and Windows cannot be
/// posed at all today. A viewing-format picture is the one thing all three
/// systems open without Smoozy carrying an image library, so nothing about
/// reading a working format ever crosses to the user's end."*
pub const A_DERIVATIVE_IS_WRITTEN_IN_A_VIEWING_FORMAT: &[u8] = PNG;

/// **B30 and B29: where a derivative sits and how it is published.**
///
/// **B30: derivatives sit in the job record directory and never in the job output
/// folder** (L7, L8), *"so no derivative is ever counted by the inventory,
/// watched by `Download`, or brought home as an output"*.
///
/// **B29: a derivative is produced into a staging path and published by a
/// rename**, under `Server protocol and transfers` §7.5. *"Blender writes straight
/// to the final name and grows in place, so a derivative produced directly at its
/// final name can be fetched half-written and shown to the user as the render."*
/// **This is the one unit in this document that publishes over an existing
/// name** — M6 removes the need for the extension from the job-material unit
/// entirely, because a job identity is never reused — so `Server protocol and
/// transfers` S17's declared degradation applies here and nowhere else in this
/// document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhereADerivativeIsWritten {
    /// **Inside the job record directory, under a name marking it as staged**,
    /// which is `Server protocol and transfers` S1's same-filesystem
    /// requirement met by placement rather than by a check.
    ///
    /// **It used to be inside `AJobsPaths::staging_directory`, and that is the
    /// directory the job's own publication renames away.** Corrected 2026-09-05,
    /// out of a code round: `publication.rs` renames the staging directory onto
    /// the job record directory's path at submission — its own test asserts
    /// *"the rename is the cleanup and the staging name must be gone"* — and
    /// every derivative is made long afterwards. So the staged path sat inside a
    /// directory that no longer exists and that nothing in the crate re-creates:
    /// **either every derivative failed and a finished job had no preview, or
    /// whoever wired the preview worker up reached for a make-parents create,
    /// which N9 forbids outright as *"the silent collision path"*.**
    pub staged_at: String,
    /// Its final name inside the job record directory.
    pub published_at: String,
}

/// **B29 and B30, composed from the job's own paths and from nothing else.**
///
/// **The name is Build's own**, as every name this document synthesises is (N3),
/// and it carries the frame so that one job's set of stills is one file per
/// frame in one directory.
pub fn where_a_derivative_is_written(
    paths: &crate::publication::AJobsPaths,
    frame: i64,
) -> WhereADerivativeIsWritten {
    where_a_derivative_is_written_in(&paths.job_record_directory, frame)
}

/// **The same composition, from the job record directory alone** — which is
/// all of the job's paths it reads, and all the preview worker holds, the
/// supervisor writing that directory into `Queue & History` Q2's job record.
/// Added 2026-10-03, the 38's piece 3; [`where_a_derivative_is_written`] is
/// this, so the name has one composition.
pub fn where_a_derivative_is_written_in(
    job_record_directory: &str,
    frame: i64,
) -> WhereADerivativeIsWritten {
    let name = format!("preview-{frame}.png");
    // **Both sit in the job record directory**, which exists for the life of the
    // job — so there is no directory to create, and the rename stays on one
    // filesystem. The staged name is marked with `publication.rs`'s own prefix
    // rather than four letters written a second time.
    WhereADerivativeIsWritten {
        staged_at: format!(
            "{job_record_directory}/{}{name}",
            crate::publication::THE_STAGING_PREFIX
        ),
        published_at: format!("{job_record_directory}/{name}"),
    }
}

/// **Where one other View Layer's copy of a frame is written** — `Queue &
/// History` P1 and P4's per-layer copy, beside the frame's main copy (which
/// keeps [`where_a_derivative_is_written_in`]'s name, so every reader of the
/// main copy is untouched). **The name carries the layer's place in the frame's
/// set and never its name**, which is the artist's own text (R4a). Added
/// 2026-10-10, the Creator's seventh notes, note 4.
pub fn where_a_layers_copy_is_written_in(
    job_record_directory: &str,
    frame: i64,
    place: u32,
) -> WhereADerivativeIsWritten {
    let name = format!("preview-{frame}-layer-{place}.png");
    WhereADerivativeIsWritten {
        staged_at: format!(
            "{job_record_directory}/{}{name}",
            crate::publication::THE_STAGING_PREFIX
        ),
        published_at: format!("{job_record_directory}/{name}"),
    }
}

/// **Where a copy is written, by its record**: the main copy at the frame's
/// own name, every other layer's at its place (P4).
pub fn where_this_copy_is_written_in(
    job_record_directory: &str,
    frame: i64,
    it_is_the_main_copy: bool,
    place: u32,
) -> WhereADerivativeIsWritten {
    if it_is_the_main_copy {
        where_a_derivative_is_written_in(job_record_directory, frame)
    } else {
        where_a_layers_copy_is_written_in(job_record_directory, frame, place)
    }
}

/// **B31a: a derivative's record carries the name of the view layer its picture
/// came from, where its choosing establishes one, and only where the file holds
/// more than one view layer.**
///
/// - **The name is established exactly where the picture's own choosing names a
///   view layer.** Inside a multi-layer file the beauty is chosen as `Combined`
///   under the name of the view layer it came from (B19), and that name is the
///   record's; **a picture a file output node fed is chosen by the name of the
///   output that fed it, which names no view layer, and no name is recorded** —
///   except where exactly one view layer feeds that destination, whose name is
///   then recorded (changed 2026-10-12 for the Creator's ninth notes, note 2;
///   made by `derivatives::the_derivatives_record`).
///   ~~For the regular output path's ordinary image the name is the file's
///   **first** view layer, the only layer such a file holds (I9).~~ **From
///   2026-10-17 that image names no view layer where the file holds more than
///   one**: it holds the first layer's picture or whichever the compositor was
///   set to read, so no layer's name can honestly be given, and the record
///   names where it came from instead ([`ADerivativesRecord::where_it_came_from`];
///   B31a's dated note on its first bullet, the Creator's tenth notes, note 2).
/// - **Where the file holds one view layer no name is recorded** — there is
///   nothing to tell apart.
/// - **Absence is a recorded state of the derivative's record, never a gap.**
/// - A view layer name is the artist's own text and is carried as bytes under
///   R4a's discipline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ADerivativesRecord {
    /// The one named source file the derivative is made of (B31).
    pub the_source_file: Vec<u8>,
    pub frame: i64,
    /// **The view layer's name, where one is established** — and a recorded
    /// absence otherwise.
    pub the_view_layer_its_picture_came_from: Option<ArtistsText>,
    /// **Whether this copy is the frame's main copy** — B26 step 1's pick,
    /// the one everything outside the preview's layer choice reads (`Queue &
    /// History` P10). Added 2026-10-10, the Creator's seventh notes, note 4
    /// (B31a's new bullet). **A record written before then is the frame's only
    /// copy, so it reads as the main one.**
    #[serde(default = "a_copy_recorded_before_the_layers_is_the_main_one")]
    pub it_is_the_main_copy: bool,
    /// **Its place in the frame's set** — the index of its view layer in the
    /// frame's layers read with the job's choices (`Queue & History` P1, P4),
    /// so the desktop orders a frame's layers without reading anything else.
    /// Nought where the frame has one layer, and on every record written
    /// before 2026-10-10.
    #[serde(default)]
    pub its_place_in_the_frames_set: u32,
    /// **Where its picture came from, for a copy that names no View Layer in
    /// a frame of more than one** — the regular output path, or the File
    /// Output node by name (B31a's last bullet, added 2026-10-17, the
    /// Creator's tenth notes, note 2), so a frame's main copy is never
    /// unnamed beside named ones. **Absent everywhere else**: on a named copy,
    /// on a frame of one layer, and on every record written before that day,
    /// which reads as it did.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub where_it_came_from: Option<WhereThePictureCameFrom>,
}

fn a_copy_recorded_before_the_layers_is_the_main_one() -> bool {
    true
}

/// **Which of the job's write places a picture was written by** — the regular
/// output path, or a File Output node, by the node's own name. B31a's last
/// bullet, added 2026-10-17, the Creator's tenth notes, note 2. **The node's
/// name is the artist's own text and is carried as bytes under R4a's
/// discipline**, like a view layer's.
///
/// **One written form, for the record, the published event (`Queue & History`
/// P3) and the desktop's kept copy alike**: `"output_path"`, or
/// `{"file_output_node":[…the name's bytes…]}`. The desktop words it — *Output
/// path*, *File Output node* with the name as a value — and nothing here does.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WhereThePictureCameFrom {
    #[serde(rename = "output_path")]
    TheOutputPath,
    #[serde(rename = "file_output_node")]
    AFileOutputNode(ArtistsText),
}

impl WhereThePictureCameFrom {
    /// **Its written form**, as [`WhereThePictureCameFrom`] says.
    pub fn as_written(&self) -> String {
        serde_json::to_string(self).expect("the source always renders")
    }

    /// **Its written form, read back**; nothing where the words are not this
    /// shape.
    pub fn read_back(words: &str) -> Option<WhereThePictureCameFrom> {
        serde_json::from_str(words).ok()
    }
}

/// **B31a, performed.**
///
/// `the_file_holds_more_than_one_view_layer` is read off the reading's own
/// evaluation, which already enumerates every view layer per frame (R4 item 5,
/// I6), and is not a second reading at the draw.
///
/// **A name is given only where the picture is one layer's by name**: inside
/// a multi-layer file, the `<view layer>.Combined` the copy is made of (B19,
/// B26 step 1). **The regular output path's ordinary image names none** where
/// the file holds more than one view layer — it holds the first layer's
/// picture or whichever the compositor was set to read (`Specs/What a job
/// writes`, *A frame with several active view layers*, observed) — and its
/// record names its source instead (B31a's dated note on its first bullet,
/// 2026-10-17, the Creator's tenth notes, note 2). *Until that day it was
/// named by the file's first view layer.*
pub fn the_view_layer_a_derivatives_record_carries(
    the_file_holds_more_than_one_view_layer: bool,
    it_is_a_multi_layer_file: bool,
    the_view_layer_the_beauty_is_under: Option<&ArtistsText>,
    it_came_from_a_file_output_node: bool,
) -> Option<ArtistsText> {
    if !the_file_holds_more_than_one_view_layer {
        return None;
    }
    if it_came_from_a_file_output_node {
        // Chosen by the name of the output that fed it, which names no view
        // layer. A node fed by exactly one view layer is named by that layer
        // in `derivatives::the_derivatives_record`, before this is asked.
        return None;
    }
    if it_is_a_multi_layer_file {
        return the_view_layer_the_beauty_is_under.cloned();
    }
    None
}

/// **B31: what a derivative is made of is one named source file**, chosen by
/// §8.3.
///
/// *"A running job's preview follows the newest candidate the rungs allow; when
/// that is refreshed and how it is shown is `Queue & History`'s."*
pub const A_DERIVATIVE_IS_MADE_OF_ONE_NAMED_SOURCE_FILE: bool = true;

#[cfg(test)]
mod the_previews_and_the_derivatives {
    use super::tests::text;
    use super::*;

    fn a_file(path: &str, frame: i64, picture: Option<&str>, format: &str, bytes: u64) -> APresentFile {
        APresentFile {
            path: path.as_bytes().to_vec(),
            frame: Some(frame),
            the_picture_it_holds: picture.map(text),
            format: format.as_bytes().to_vec(),
            bytes,
            is_video_shaped: false,
            it_sits_in_the_main_outputs_folder: false,
        }
    }

    fn told(version: TheVersionInHand) -> WhatTheRungsAreToldAboutTheJob {
        WhatTheRungsAreToldAboutTheJob {
            version,
            the_last_frame_asked_for_that_is_present: None,
        }
    }

    #[test]
    fn the_rungs_are_taken_in_order_and_the_beauty_wins_over_a_later_diffuse_frame() {
        // B17: the beauty pass; failing that the diffuse colour pass; failing
        // that the latest image, whatever pass it turns out to be. A rule that
        // simply took the newest file would take the diffuse frame here.
        let present = vec![
            a_file("/j/out/diff0009.png", 9, Some("Diffuse Color"), "PNG", 10),
            a_file("/j/out/beauty0002.png", 2, Some("Combined"), "PNG", 10),
            a_file("/j/out/other0011.png", 11, Some("Mist"), "PNG", 10),
        ];
        let ordered = the_candidates(&present, &told(TheVersionInHand::of(5, 2)));
        assert_eq!(ordered[0].file.path, b"/j/out/beauty0002.png".to_vec());
        assert_eq!(ordered[1].file.path, b"/j/out/diff0009.png".to_vec());
    }

    #[test]
    fn the_diffuse_pass_is_named_apart_by_version_and_the_wrong_name_falls_to_the_last_rung() {
        // B17: `DiffCol` in 4.5 and `Diffuse Color` from 5.0, so the rung needs
        // to know which Blender wrote the file.
        let in_four_five = [a_file("/j/out/d0001.png", 1, Some("DiffCol"), "PNG", 10)];
        assert_eq!(
            the_rung_of(&in_four_five[0], TheVersionInHand::of(4, 5)),
            Rung::TheDiffuseColourPass
        );
        assert_eq!(
            the_rung_of(&in_four_five[0], TheVersionInHand::of(5, 2)),
            Rung::TheLatestImageWhateverItIs
        );
    }

    #[test]
    fn an_empty_file_is_never_a_candidate_at_any_rung() {
        // B20's first exclusion, and C13's rule from the preview's side.
        let present = vec![
            a_file("/j/out/beauty0009.png", 9, Some("Combined"), "PNG", 0),
            a_file("/j/out/beauty0002.png", 2, Some("Combined"), "PNG", 10),
        ];
        let ordered = the_candidates(&present, &told(TheVersionInHand::of(5, 2)));
        assert_eq!(ordered.len(), 1);
        assert_eq!(ordered[0].file.frame, Some(2));
    }

    #[test]
    fn the_last_frame_asked_for_that_is_present_is_not_a_candidate_either() {
        // B20's second exclusion, for the same reason C17 re-renders it: it is
        // the one frame in the range that can hold a file which is present, is
        // not empty, and is not whole.
        let present = vec![
            a_file("/j/out/beauty0009.png", 9, Some("Combined"), "PNG", 10),
            a_file("/j/out/beauty0008.png", 8, Some("Combined"), "PNG", 10),
        ];
        let ordered = the_candidates(
            &present,
            &WhatTheRungsAreToldAboutTheJob {
                version: TheVersionInHand::of(5, 2),
                the_last_frame_asked_for_that_is_present: Some(9),
            },
        );
        assert_eq!(ordered.len(), 1);
        assert_eq!(ordered[0].file.frame, Some(8));
    }

    #[test]
    fn a_video_shaped_destination_is_not_a_preview_candidate_at_any_rung() {
        // The rungs read an image, a video file is not one, and B26 has no route
        // for it.
        let mut video = a_file("/j/out/whole.mp4", 1, None, "FFMPEG", 4096);
        video.is_video_shaped = true;
        assert!(the_candidates(&[video], &told(TheVersionInHand::of(5, 2))).is_empty());
    }

    #[test]
    fn a_job_that_has_written_no_image_has_no_preview() {
        // B21: a job that writes nothing, and a running job before its first
        // frame lands.
        assert!(the_candidates(&[], &told(TheVersionInHand::of(5, 2))).is_empty());
    }

    #[test]
    fn within_one_rank_the_viewing_format_is_preferred_wherever_it_sits() {
        // B18, over B12's classification of the set present.
        let present = vec![
            a_file("/j/out/beauty0002.exr", 2, Some("Combined"), "OPEN_EXR", 10),
            a_file("/j/passes/beauty0002.png", 2, Some("Combined"), "PNG", 10),
        ];
        let ordered = the_candidates(&present, &told(TheVersionInHand::of(5, 2)));
        assert_eq!(ordered[0].file.format, b"PNG".to_vec());
    }

    #[test]
    fn where_two_are_left_the_one_in_the_main_outputs_folder_is_read_and_then_the_pick() {
        // B18's tie-break, and B15's pick behind it — read rather than made
        // again.
        let mut inside = a_file("/j/main/beauty0002.png", 2, Some("Combined"), "PNG", 10);
        inside.it_sits_in_the_main_outputs_folder = true;
        let outside = a_file("/j/other/beauty0002.png", 2, Some("Combined"), "PNG", 10);
        let candidates = the_candidates(
            &[outside.clone(), inside.clone()],
            &told(TheVersionInHand::of(5, 2)),
        );
        let picks = ThePicks::made_once(None, None);
        assert_eq!(
            which_file_the_preview_reads(&candidates, &picks).unwrap().path,
            inside.path
        );

        // With neither in the main output's folder, B13's pick decides.
        let another = a_file("/j/third/beauty0002.png", 2, Some("Combined"), "PNG", 10);
        let neither = the_candidates(
            &[outside.clone(), another.clone()],
            &told(TheVersionInHand::of(5, 2)),
        );
        let picked = ThePicks::made_once(None, Some(another.path.clone()));
        assert_eq!(
            which_file_the_preview_reads(&neither, &picked).unwrap().path,
            another.path
        );
    }

    #[test]
    fn a_worse_rung_is_never_swept_into_the_tie_and_cannot_win_on_sitting_in_the_folder() {
        // The tie set used to be filtered by frame and exact format name and by
        // nothing else, so a same-frame same-format file at a worse rung joined
        // the tie and won on sitting in the main output's folder — and B15
        // makes the pick once and keeps it, so the job's preview was the
        // diffuse colour pass for good. The spec is blunt: *"The pass comes
        // first, the format second."*
        let beauty = a_file("/j/beauty/beauty0010.png", 10, Some("Combined"), "PNG", 10);
        let mut diffuse = a_file("/j/main/diff0010.png", 10, Some("Diffuse Color"), "PNG", 10);
        diffuse.it_sits_in_the_main_outputs_folder = true;
        let candidates = the_candidates(
            &[beauty.clone(), diffuse],
            &told(TheVersionInHand::of(5, 2)),
        );
        assert_eq!(
            which_file_the_preview_reads(&candidates, &ThePicks::made_once(None, None))
                .unwrap()
                .path,
            beauty.path
        );
    }

    #[test]
    fn two_viewing_formats_of_one_rung_and_frame_meet_in_the_tie_set() {
        // The other half of the same line. With OpenEXR present both PNG and
        // JPEG are viewing formats (B12), so these two are of one rung, one
        // frame and one format class — but the filter read the exact format
        // name, so they never met, the tie came back at one, and B18's
        // main-output-folder tiebreak silently did not run.
        let exr = a_file("/j/w/beauty0010.exr", 10, Some("Combined"), "OPEN_EXR", 10);
        let png = a_file("/j/other/beauty0010.png", 10, Some("Combined"), "PNG", 10);
        let mut jpeg = a_file("/j/main/beauty0010.jpg", 10, Some("Combined"), "JPEG", 10);
        jpeg.it_sits_in_the_main_outputs_folder = true;
        let candidates = the_candidates(
            &[exr, png, jpeg.clone()],
            &told(TheVersionInHand::of(5, 2)),
        );
        assert_eq!(
            which_file_the_preview_reads(&candidates, &ThePicks::made_once(None, None))
                .unwrap()
                .path,
            jpeg.path
        );
    }

    #[test]
    fn the_media_workers_source_is_the_frame_shaped_candidates_in_frame_order() {
        // B16's second half. `Telegram` already makes which file each picture
        // comes from this document's; producing them is the media worker's.
        let present = vec![
            a_file("/j/out/beauty0003.png", 3, Some("Combined"), "PNG", 10),
            a_file("/j/out/beauty0001.png", 1, Some("Combined"), "PNG", 10),
            a_file("/j/out/beauty0002.png", 2, Some("Combined"), "PNG", 10),
            a_file("/j/elsewhere/mist0002.png", 2, Some("Mist"), "PNG", 10),
        ];
        let candidates = the_candidates(&present, &told(TheVersionInHand::of(5, 2)));
        let source = the_source_the_media_worker_reads(&candidates, &ThePicks::made_once(None, None));
        assert_eq!(
            source.iter().map(|file| file.frame).collect::<Vec<_>>(),
            vec![Some(1), Some(2), Some(3)]
        );
    }

    #[test]
    fn b26a_brings_a_full_size_multi_layer_frame_under_the_published_caps() {
        // **A full-size derivative is demonstrated rather than hypothetical**:
        // on the multi-layer route the scaling call silently succeeds and
        // changes nothing while the saving call writes at full size, so without
        // a stated target an 8K job produces a derivative no consumer can take,
        // for every frame.
        let target = the_target_size(7680, 4320);
        assert!(
            u64::from(target.width) + u64::from(target.height)
                <= u64::from(THE_WIDTH_AND_HEIGHT_MUST_NOT_EXCEED_IN_TOTAL)
        );
        assert!(!target.it_could_not_be_brought_under_every_cap);
        // The ratio is untouched, which is what scaling can and cannot do.
        let before = 7680_f64 / 4320_f64;
        let after = f64::from(target.width) / f64::from(target.height);
        assert!((before - after).abs() < 0.01);
    }

    #[test]
    fn a_ratio_beyond_twenty_is_recorded_rather_than_pretended_away() {
        // B26a's last sentence: the one shape scaling cannot fix.
        let target = the_target_size(9000, 100);
        assert!(target.it_could_not_be_brought_under_every_cap);
        // It is still brought under the cap it can be brought under.
        assert!(
            u64::from(target.width) + u64::from(target.height)
                <= u64::from(THE_WIDTH_AND_HEIGHT_MUST_NOT_EXCEED_IN_TOTAL)
        );
    }

    #[test]
    fn a_picture_already_under_the_caps_is_left_where_it_is() {
        let target = the_target_size(1920, 1080);
        assert_eq!((target.width, target.height), (1920, 1080));
    }

    /// **B26a, *how the production comes under the byte cap*.** A 4K frame of
    /// noise as an eight-bit PNG weighs its pixels, three bytes each.
    #[test]
    fn b26a_a_copy_over_the_byte_cap_is_sized_from_how_far_over_it_was() {
        let cap = THE_LARGEST_A_DERIVATIVE_MAY_BE_IN_BYTES;
        let weighed = 3840 * 2160 * 3;
        let next = the_size_to_write_it_at_next((3840, 2160), weighed).expect("it is over");
        assert!(next.width < 3840 && next.height < 2160, "{next:?}");
        // **The ratio is kept**, to the pixel the rounding costs.
        let ratio = f64::from(next.width) / f64::from(next.height);
        assert!((ratio - 3840.0 / 2160.0).abs() < 0.01, "{next:?}");
        // **A weight that follows the area lands under the cap, and not far under it.**
        let then = u64::from(next.width) * u64::from(next.height) * 3;
        assert!(then <= cap, "{then}");
        assert!(then > cap * 8 / 10, "no smaller than it had to be: {then}");
        // The further over, the smaller.
        let further = the_size_to_write_it_at_next((3840, 2160), weighed * 4).expect("it is over");
        assert!(further.width < next.width, "{further:?}");
        assert!(!next.it_could_not_be_brought_under_every_cap);
    }

    #[test]
    fn b26a_a_copy_at_or_under_the_byte_cap_has_no_next_size() {
        let cap = THE_LARGEST_A_DERIVATIVE_MAY_BE_IN_BYTES;
        assert_eq!(the_size_to_write_it_at_next((3840, 2160), cap), None);
        assert_eq!(the_size_to_write_it_at_next((3840, 2160), 1), None);
        let just_over = the_size_to_write_it_at_next((3840, 2160), cap + 1).expect("over");
        assert!(just_over.width < 3840, "{just_over:?}");
    }

    #[test]
    fn b26a_a_copy_that_can_be_made_no_smaller_has_no_next_size() {
        let cap = THE_LARGEST_A_DERIVATIVE_MAY_BE_IN_BYTES;
        assert_eq!(the_size_to_write_it_at_next((1, 1), cap * 2), None);
        assert_eq!(the_size_to_write_it_at_next((0, 100), cap * 2), None);
        // A side is never scaled down to nothing.
        let thin = the_size_to_write_it_at_next((9000, 1), cap * 100).expect("the long side can shrink");
        assert_eq!(thin.height, 1);
        assert!(thin.width < 9000);
        // A ratio past twenty stays on the record.
        assert!(thin.it_could_not_be_brought_under_every_cap);
    }

    #[test]
    fn the_multi_layer_route_is_the_bundled_library_and_the_other_is_blenders_own() {
        // B26: the split is forced by what was observed rather than preferred.
        assert_eq!(which_route(true), WhichRouteMakesTheCopy::TheBundledImageLibrary);
        assert_eq!(
            which_route(false),
            WhichRouteMakesTheCopy::BlendersOwnImageHandling
        );
    }

    #[test]
    fn a_derivative_is_staged_and_published_and_never_written_at_its_final_name() {
        // B29: Blender writes straight to the final name and grows in place, so
        // a derivative produced directly at its final name can be fetched
        // half-written and shown to the user as the render. B30 keeps it out of
        // the job output folder.
        let layout = crate::layout::TheLayout::of(
            crate::layout::MachineRoot::chosen("/ephemeral/smoozy").unwrap(),
            crate::layout::WhereThePackageAreaSits::UnderTheRoot,
        );
        let job = smoozy_state::identities::Identity::issue(
            smoozy_state::identities::IdentityKind::Job,
        )
        .unwrap();
        let paths = crate::publication::AJobsPaths::composed(&layout, &job, None).unwrap();
        let written = where_a_derivative_is_written(&paths, 147);
        assert!(written.staged_at.starts_with(&paths.job_record_directory));
        assert!(written.published_at.starts_with(&paths.job_record_directory));
        assert_ne!(written.staged_at, written.published_at);
        assert!(!written.published_at.starts_with(&paths.job_output_folder));

        // **The one the four above would not have caught, and the fault was
        // real**: the staged path must sit under a directory the job's own
        // publication does not rename away. `place_a_jobs_material` renames the
        // staging directory onto the job record directory's path at submission
        // — its own test asserts *"the rename is the cleanup and the staging
        // name must be gone"* — and every derivative is made long after that,
        // so a derivative staged inside it was staged into a directory that no
        // longer exists and that nothing in this crate re-creates.
        assert!(
            !written.staged_at.starts_with(&paths.staging_directory),
            "the publication renames the staging directory away and nothing re-creates it"
        );
    }

    #[test]
    fn b31a_records_a_view_layer_name_only_where_the_choosing_establishes_one() {
        let reflections = text("Reflections");
        let first = text("ViewLayer");
        // One view layer: nothing to tell apart, so nothing is recorded.
        assert_eq!(
            the_view_layer_a_derivatives_record_carries(false, true, Some(&reflections), false),
            None
        );
        // The beauty inside a multi-layer file carries the layer it came from.
        assert_eq!(
            the_view_layer_a_derivatives_record_carries(true, true, Some(&reflections), false),
            Some(reflections.clone())
        );
        // A picture a file output node fed is chosen by the output's name, which
        // names no view layer.
        assert_eq!(
            the_view_layer_a_derivatives_record_carries(true, true, Some(&reflections), true),
            None
        );
        // **The regular output path's ordinary image names no view layer** in
        // a file of several (the Creator's tenth notes, note 2): until
        // 2026-10-17 it was named by the file's first.
        assert_eq!(
            the_view_layer_a_derivatives_record_carries(true, false, Some(&first), false),
            None
        );
    }

    /// **B31a's source, written once for the record, the event and the
    /// desktop**: the output path and a node by name each read back as
    /// themselves, a node's name as its bytes; words of another shape read as
    /// nothing; and a record written before 2026-10-17 reads with none, and
    /// is written again without the field. The Creator's tenth notes, note 2.
    #[test]
    fn the_source_of_a_picture_is_written_once_and_read_back() {
        let node = WhereThePictureCameFrom::AFileOutputNode(text("Beauty"));
        assert_eq!(WhereThePictureCameFrom::TheOutputPath.as_written(), "\"output_path\"");
        assert_eq!(node.as_written(), r#"{"file_output_node":[66,101,97,117,116,121]}"#);
        for one in [WhereThePictureCameFrom::TheOutputPath, node.clone()] {
            assert_eq!(WhereThePictureCameFrom::read_back(&one.as_written()), Some(one));
        }
        assert_eq!(WhereThePictureCameFrom::read_back("\"a_sequencer\""), None);
        let old: ADerivativesRecord =
            serde_json::from_str(r#"{"the_source_file":[47],"frame":3,"the_view_layer_its_picture_came_from":null}"#)
                .expect("an older record reads");
        assert_eq!(old.where_it_came_from, None);
        assert!(!serde_json::to_string(&old).unwrap().contains("where_it_came_from"));
        let sourced = ADerivativesRecord { where_it_came_from: Some(node), ..old };
        let again: ADerivativesRecord = serde_json::from_str(&serde_json::to_string(&sourced).unwrap()).unwrap();
        assert_eq!(again, sourced);
    }

}
