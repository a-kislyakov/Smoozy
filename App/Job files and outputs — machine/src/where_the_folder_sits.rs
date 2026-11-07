//! §6.6 — where the job output folder sits: P18 to P23.
//!
//! **P18: the job output folder sits inside the shared run of the render's own
//! output destinations, resolved on the machine, and is named by N3.** For the
//! Creator's file — regular output `//../Render/Animation`, nodes at
//! `//../Render/PNG` and `//../Render/EXR`, the `.blend` in
//! `/home/ubuntu/rendering` — that is `/home/ubuntu/Render/<N3 name>/`.
//!
//! **P19 and P23: the fallback, in two cases, and the fallback's place.** A
//! run from another computer, or a folder Smoozy cannot use, takes the
//! server's chosen fallback folder where the user set one and it passes the
//! same conditions, and the job area of L6 otherwise. The case is recorded
//! with the path, and so is whether a chosen fallback folder was passed over
//! (P23 as reversed later on 2026-10-04).
//!
//! **One choice, one function.** [`the_folder_is_chosen_and_made`] is the
//! choice the render worker makes at `Rendering` P1 step 4 (P21's second
//! case), and [`where_it_would_sit`] is the same function asked with nothing
//! made, which is what Render Settings says before submitting. **A second
//! arrangement of these tests would be the fact kept in two places**, and it
//! would show as the page saying one folder and the files landing in another.
//!
//! **This crate makes no folder.** The making is a [`MakesFolders`] the caller
//! hands in — the machine's own filesystem in the render worker — so every
//! test here runs at the desk with the machine's answers as values.
//!
//! Added 2026-10-04, the Creator's second notes on the test copy, piece 4
//! (note 18's want, and their want of the same evening to choose the fallback
//! folder per server).

use serde::{Deserialize, Serialize};

use crate::layout::{TheLayout, WhereThePackageAreaSits, SHARED_TEMPORARY_AREAS};
use crate::mapping::{the_renders_own_folders_resolved, the_shared_run, Anchor};
use crate::names;
use crate::reading::{Destination, TheVersionInHand};

/// **P19's two cases, and there is no third.**
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WhyTheFilesOwnPlaceWasNotUsed {
    /// **Case 1, another computer's**: a render destination in P4's foreign
    /// form, a run of `/` alone, or a run that would need a folder made
    /// directly under the machine's own root (a Mac's `/Users/…`).
    ItIsAnotherComputers,
    /// **Case 2, a folder Smoozy cannot use**: under the root, the package
    /// area's location or a shared temporary area; a refused make; or a
    /// destination's deepest predicted path over N8's budget there.
    ItIsAFolderSmoozyCannotUse,
}

/// **Where the job output folder went, and why** — what is recorded with the
/// path (M1, P21, P23).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WhereItIs {
    /// P18: inside the run the render's own destinations share.
    WhereTheFilePoints,
    /// P19 and P23: the fallback's place.
    TheFallback {
        /// P19's case. **Absent for a job with no render destination at all**,
        /// which takes the fallback's place *"without a reason recorded"*
        /// (P18's last sentence).
        why: Option<WhyTheFilesOwnPlaceWasNotUsed>,
        /// **P23's reversal**: a fallback folder the user chose for this server
        /// could not be used, so the job went to Smoozy's own job area. Which
        /// of P23's conditions failed is not recorded.
        the_servers_fallback_folder_was_passed_over: bool,
    },
}

/// **The job output folder's path, with where it is** — chosen once per job
/// and never again (P21).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WhereTheJobOutputFolderSits {
    pub path: String,
    pub where_it_is: WhereItIs,
}

/// **The file the render worker writes the choice into, inside the job record
/// directory, where it chose at render start** (P21's second case: *"writes
/// the path and its case into the job record directory"*).
///
/// **Build's own name and form**, as the manifest's are: JSON, written once
/// and never rewritten. **Here and not in the machine's program because both
/// ends read it**: a later attempt reads it so it never chooses again, and the
/// desktop reads it with the recorded reading (`Queue & History` N9) and
/// `Download` reads it for the folder it walks.
pub const THE_RECORD_OF_WHERE_THE_FOLDER_SITS: &str = "job-output-folder.json";

impl WhereTheJobOutputFolderSits {
    /// **The folder the job output folder sits in** — the run, the chosen
    /// fallback folder, or the job area — which is what Render Settings names.
    pub fn the_folder_it_sits_in(&self) -> &str {
        match self.path.rfind('/') {
            Some(0) => "/",
            Some(at) => &self.path[..at],
            None => "",
        }
    }

    pub fn as_written(&self) -> Vec<u8> {
        serde_json::to_vec_pretty(self).expect("a folder's place is made of values that always render")
    }

    pub fn read_back(bytes: &[u8]) -> Option<WhereTheJobOutputFolderSits> {
        serde_json::from_slice(bytes).ok()
    }
}

/// **The run P18 places the job output folder in**, before anything is tested
/// against it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheRun {
    /// **No render destination at all** (P18's last sentence): no run, and the
    /// fallback's place with no reason recorded.
    ThereIsNone,
    /// **P19 case 1, known from the paths alone**: a destination in P4's
    /// foreign form, or a run of `/` alone.
    ItIsAnotherComputers,
    /// The run, as whole names under `/`.
    It(Vec<Vec<u8>>),
}

/// **P18's run, worked out over the render's own output destinations** by the
/// one resolution and the one shared-run rule the map uses
/// ([`the_renders_own_folders_resolved`], [`the_shared_run`]).
pub fn the_run_of(
    destinations: &[Destination],
    the_folder_the_blend_sits_in: &[u8],
    version: TheVersionInHand,
) -> TheRun {
    let resolved = the_renders_own_folders_resolved(destinations, the_folder_the_blend_sits_in, version);
    if resolved.is_empty() {
        return TheRun::ThereIsNone;
    }
    if resolved.iter().any(|folder| folder.anchor == Anchor::ItsOwnTree) {
        return TheRun::ItIsAnotherComputers;
    }
    let mut run = the_shared_run(&resolved);
    // **P7b: the job's folder is made by name, and Blender would read a brace
    // in it for an expression**, so the run stops before any folder holding
    // one. That folder is carried to Blender under the job's folder as P7 says.
    if let Some(at) = run.iter().position(|name| name.iter().any(|byte| *byte == b'{' || *byte == b'}')) {
        run.truncate(at);
    }
    if run.is_empty() {
        return TheRun::ItIsAnotherComputers;
    }
    TheRun::It(run)
}

/// **What stands at a path on the machine**, as P20's stat finds it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatStands {
    AFolder,
    Nothing,
    /// A file, or an answer that could not be read: either way not a folder
    /// the job's can be made in, and P20 takes it as a refusal.
    SomethingElse,
}

/// **The machine's two acts P20 needs**: a stat, and N9's plain make — never
/// the make-parents form, which is the silent collision path.
pub trait MakesFolders {
    fn what_stands_at(&mut self, path: &str) -> WhatStands;
    fn make_one_folder(&mut self, path: &str) -> Result<(), ()>;
}

/// **What the choice is made from.** Every field is the caller's own fact.
pub struct WhatTheChoiceIsMadeFrom<'a> {
    pub the_run: &'a TheRun,
    pub layout: &'a TheLayout,
    /// **N3's name**, composed where the label is known — on the desktop at
    /// submission — and carried here.
    pub the_job_output_folders_name: &'a str,
    /// **P23: the server's chosen fallback folder as it stood at submission**,
    /// or nothing where none was set.
    pub the_servers_fallback_folder: Option<&'a str>,
}

/// **N10's fault: the job area refused a fresh job output folder.** A job
/// identity is never reused, so this is a fault and not a case, and the
/// attempt fails.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheJobAreaRefusedTheFolder;

/// Why one place could not take the job output folder.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WhyNotHere {
    ItNeedsAFolderDirectlyUnderTheRoot,
    SmoozyCannotUseIt,
}

impl WhyNotHere {
    fn as_the_case(self) -> WhyTheFilesOwnPlaceWasNotUsed {
        match self {
            WhyNotHere::ItNeedsAFolderDirectlyUnderTheRoot => WhyTheFilesOwnPlaceWasNotUsed::ItIsAnotherComputers,
            WhyNotHere::SmoozyCannotUseIt => WhyTheFilesOwnPlaceWasNotUsed::ItIsAFolderSmoozyCannotUse,
        }
    }
}

/// **P18 to P23, performed: the place chosen and the job output folder made
/// there, or the next place tried.**
///
/// The order, and each step is a clause:
///
/// 1. **Where the file points** (P18): the run, if it passes P19 — not
///    another computer's, not under the root, the package area's location or a
///    shared temporary area, every destination fitting N8's budget there
///    (`fits`), and its missing folders and then the job output folder made by
///    P20, one segment at a time in the plain form.
/// 2. **The server's chosen fallback folder** (P23), where one was set, by the
///    same conditions. A folder already made on the way to a place that then
///    fails is left: it is where the file, or the user, pointed.
/// 3. **The job area** (L6), with N9's plain make of the job output folder
///    alone, the area existing already. **A refusal there is N10's fault.**
///
/// **`fits` is asked with each candidate job output folder** and answers
/// whether every destination's deepest predicted path fits N8's budget with
/// it as the prefix; the caller composes the map to say. It is not asked of
/// the job area, where P12 case 1 holds the job as it always did.
pub fn the_folder_is_chosen_and_made(
    from: &WhatTheChoiceIsMadeFrom<'_>,
    fits: &mut dyn FnMut(&str) -> bool,
    maker: &mut dyn MakesFolders,
) -> Result<WhereTheJobOutputFolderSits, TheJobAreaRefusedTheFolder> {
    let name = from.the_job_output_folders_name;
    let why = match from.the_run {
        TheRun::ThereIsNone => None,
        TheRun::ItIsAnotherComputers => Some(WhyTheFilesOwnPlaceWasNotUsed::ItIsAnotherComputers),
        TheRun::It(segments) => match the_run_as_a_path(segments) {
            None => Some(WhyTheFilesOwnPlaceWasNotUsed::ItIsAFolderSmoozyCannotUse),
            Some(inside) => match it_is_made_in(&inside, from, fits, maker) {
                Ok(path) => {
                    return Ok(WhereTheJobOutputFolderSits {
                        path,
                        where_it_is: WhereItIs::WhereTheFilePoints,
                    })
                }
                Err(not_here) => Some(not_here.as_the_case()),
            },
        },
    };

    let mut passed_over = false;
    if let Some(chosen) = the_chosen_folder(from.the_servers_fallback_folder) {
        match chosen.and_then(|inside| it_is_made_in(&inside, from, fits, maker).map_err(|_| ())) {
            Ok(path) => {
                return Ok(WhereTheJobOutputFolderSits {
                    path,
                    where_it_is: WhereItIs::TheFallback {
                        why,
                        the_servers_fallback_folder_was_passed_over: false,
                    },
                })
            }
            Err(()) => passed_over = true,
        }
    }

    let path = in_the_job_area(from.layout, name);
    the_job_output_folder_is_made(maker, &path).map_err(|()| TheJobAreaRefusedTheFolder)?;
    Ok(WhereTheJobOutputFolderSits {
        path,
        where_it_is: WhereItIs::TheFallback {
            why,
            the_servers_fallback_folder_was_passed_over: passed_over,
        },
    })
}

/// **The same choice, asked with nothing made** — what Render Settings says
/// before submitting (`Design/Render Settings § What it will write`). Every
/// make is taken to land, so **only what is known before submitting rules a
/// place out**: another computer's paths, Smoozy's own areas and the temporary
/// ones, a relative fallback folder, the budget where `fits` measures one, and
/// **`the_machines_top`, the folders the reading found directly under the
/// machine's root** (`Job files and outputs` P21's note of 2026-11-02), which is
/// P20's first stat answered by the machine. **A folder below the top is taken
/// to stand**, since one missing there is simply made. A refused make, and a
/// folder made or removed at the top since the read, are found at render start
/// and said on the entry. **With no list, every folder is taken to stand**, and
/// the caller knows it cannot claim what that gave.
pub fn where_it_would_sit(
    from: &WhatTheChoiceIsMadeFrom<'_>,
    the_machines_top: Option<&[String]>,
    fits: &mut dyn FnMut(&str) -> bool,
) -> WhereTheJobOutputFolderSits {
    the_folder_is_chosen_and_made(from, fits, &mut NothingIsMadeAtTheDesk { the_machines_top })
        .unwrap_or_else(|TheJobAreaRefusedTheFolder| unreachable!("nothing is refused at the desk"))
}

/// **The job output folder in force for a job**: the one the manifest names,
/// or, where the manifest says it is chosen at render start, the one the
/// render worker recorded — and nothing until it has.
pub fn the_job_output_folder_in_force(
    named_by_the_manifest: Option<&str>,
    recorded_at_render_start: Option<&WhereTheJobOutputFolderSits>,
) -> Option<String> {
    named_by_the_manifest
        .map(str::to_string)
        .or_else(|| recorded_at_render_start.map(|sits| sits.path.clone()))
}

/// The answers [`where_it_would_sit`] asks with.
struct NothingIsMadeAtTheDesk<'a> {
    the_machines_top: Option<&'a [String]>,
}

impl MakesFolders for NothingIsMadeAtTheDesk<'_> {
    fn what_stands_at(&mut self, path: &str) -> WhatStands {
        let at_the_top = path.strip_prefix('/').filter(|name| !name.contains('/'));
        match (self.the_machines_top, at_the_top) {
            (Some(top), Some(name)) if !top.iter().any(|one| one == name) => WhatStands::Nothing,
            _ => WhatStands::AFolder,
        }
    }
    fn make_one_folder(&mut self, _path: &str) -> Result<(), ()> {
        Ok(())
    }
}

/// The run's whole names as one absolute path, or nothing where a name is not
/// text — a folder Smoozy cannot name is one it cannot use (P19 case 2).
fn the_run_as_a_path(segments: &[Vec<u8>]) -> Option<String> {
    let mut out = String::new();
    for segment in segments {
        out.push('/');
        out.push_str(std::str::from_utf8(segment).ok()?);
    }
    Some(out)
}

/// **The server's chosen fallback folder, tidied** — `None` where none is set,
/// `Some(Err)` where what is held cannot be a folder at all: not absolute, or
/// holding a `.` or `..` step a comparison of text would walk round (L3's own
/// reason for refusing one in a root). The Edit dialog refuses a relative path
/// where it is typed (`Servers` D3), so this guards only a value that reached
/// the row by some other route (P23's last paragraph).
fn the_chosen_folder(held: Option<&str>) -> Option<Result<String, ()>> {
    let typed = held?.trim();
    if typed.is_empty() {
        return None;
    }
    if !typed.starts_with('/') {
        return Some(Err(()));
    }
    let tidied = typed.trim_end_matches('/');
    if tidied.split('/').any(|segment| segment == "." || segment == "..") {
        return Some(Err(()));
    }
    Some(Ok(tidied.to_string()))
}

/// **One place tried**: P19's tests that need no machine, then N8's budget,
/// then P20's making.
fn it_is_made_in(
    inside: &str,
    from: &WhatTheChoiceIsMadeFrom<'_>,
    fits: &mut dyn FnMut(&str) -> bool,
    maker: &mut dyn MakesFolders,
) -> Result<String, WhyNotHere> {
    let segments: Vec<&str> = inside.split('/').filter(|segment| !segment.is_empty()).collect();
    // **A folder directly under `/` is the job output folder's own place
    // only where the run is `/` alone**, which is P19 case 1.
    if segments.is_empty() {
        return Err(WhyNotHere::ItNeedsAFolderDirectlyUnderTheRoot);
    }
    if smoozy_cannot_use(inside, from.layout) {
        return Err(WhyNotHere::SmoozyCannotUseIt);
    }
    let path = format!("{inside}/{}", from.the_job_output_folders_name);
    if names::a_path_fits(&path).is_err() || !fits(&path) {
        return Err(WhyNotHere::SmoozyCannotUseIt);
    }
    // **P20: each missing folder, found missing by a stat, made one segment at
    // a time**; a folder missing directly under `/` is P19 case 1.
    let mut prefix = String::new();
    let mut missing = false;
    for (at, segment) in segments.iter().enumerate() {
        prefix.push('/');
        prefix.push_str(segment);
        if !missing {
            match maker.what_stands_at(&prefix) {
                WhatStands::AFolder => continue,
                WhatStands::Nothing if at == 0 => return Err(WhyNotHere::ItNeedsAFolderDirectlyUnderTheRoot),
                WhatStands::Nothing => missing = true,
                WhatStands::SomethingElse => return Err(WhyNotHere::SmoozyCannotUseIt),
            }
        }
        maker.make_one_folder(&prefix).map_err(|()| WhyNotHere::SmoozyCannotUseIt)?;
    }
    the_job_output_folder_is_made(maker, &path).map_err(|()| WhyNotHere::SmoozyCannotUseIt)?;
    Ok(path)
}

/// **N9's plain make of the job output folder itself — or, where a folder
/// already stands at its path, that folder taken. Added 2026-10-31, P18a.** A
/// first-frame check writes into its range job's folder and renders first, so
/// it makes the folder; the range job then finds it standing. **The name holds
/// the range job's identity, which is never reused** (N10), so a folder
/// standing there was made for this job — by its check, or by an earlier
/// attempt of its own that stopped before it recorded the choice. **The stat
/// is asked only after the plain make was refused**, so a fresh folder is made
/// exactly as before, and anything standing there that is not a folder is
/// still a refusal.
fn the_job_output_folder_is_made(maker: &mut dyn MakesFolders, path: &str) -> Result<(), ()> {
    match maker.make_one_folder(path) {
        Ok(()) => Ok(()),
        Err(()) if maker.what_stands_at(path) == WhatStands::AFolder => Ok(()),
        Err(()) => Err(()),
    }
}

/// **P19 case 2's places nothing of a job may sit under**: the root of L1, the
/// package area's location of L4a (L7, `Security and licence` §10), and a
/// shared temporary area (L3).
fn smoozy_cannot_use(inside: &str, layout: &TheLayout) -> bool {
    let under = |owner: &str| inside == owner || inside.starts_with(&format!("{owner}/"));
    if under(layout.root().as_path()) {
        return true;
    }
    if let WhereThePackageAreaSits::ApartAt(apart) = layout.where_the_package_area_sits() {
        if under(apart.as_path()) {
            return true;
        }
    }
    SHARED_TEMPORARY_AREAS.iter().any(|area| under(area))
}

/// The job output folder in the job area of L6, as every job had it before
/// §6.6.
fn in_the_job_area(layout: &TheLayout, name: &str) -> String {
    format!("{}/{name}", layout.area(crate::layout::Area::Job))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::MachineRoot;
    use crate::reading::tests::a_destination;
    use crate::reading::{AnAddedWritePlace, ArtistsText, DestinationKind, WhereItComesFrom};
    use std::collections::BTreeSet;

    fn a_layout() -> TheLayout {
        TheLayout::of(
            MachineRoot::chosen("/ephemeral/smoozy").unwrap(),
            WhereThePackageAreaSits::UnderTheRoot,
        )
    }

    const NAME: &str = "job-7-Kitchen";

    /// A machine whose folders are a set, which refuses a make where told to.
    #[derive(Default)]
    struct AMachine {
        folders: BTreeSet<String>,
        refuse: Option<String>,
        made: Vec<String>,
    }

    impl AMachine {
        fn with(folders: &[&str]) -> AMachine {
            AMachine {
                folders: folders.iter().map(|one| one.to_string()).collect(),
                ..AMachine::default()
            }
        }
    }

    impl MakesFolders for AMachine {
        fn what_stands_at(&mut self, path: &str) -> WhatStands {
            if self.folders.contains(path) {
                WhatStands::AFolder
            } else {
                WhatStands::Nothing
            }
        }
        fn make_one_folder(&mut self, path: &str) -> Result<(), ()> {
            if self.refuse.as_deref() == Some(path) || !self.folders.insert(path.to_string()) {
                return Err(());
            }
            self.made.push(path.to_string());
            Ok(())
        }
    }

    fn a_run(path: &str) -> TheRun {
        TheRun::It(
            path.split('/')
                .filter(|one| !one.is_empty())
                .map(|one| one.as_bytes().to_vec())
                .collect(),
        )
    }

    fn chosen(run: &TheRun, fallback: Option<&str>, machine: &mut AMachine) -> WhereTheJobOutputFolderSits {
        let layout = a_layout();
        the_folder_is_chosen_and_made(
            &WhatTheChoiceIsMadeFrom {
                the_run: run,
                layout: &layout,
                the_job_output_folders_name: NAME,
                the_servers_fallback_folder: fallback,
            },
            &mut |_| true,
            machine,
        )
        .expect("a folder")
    }

    fn fell_back(why: Option<WhyTheFilesOwnPlaceWasNotUsed>, passed_over: bool) -> WhereItIs {
        WhereItIs::TheFallback {
            why,
            the_servers_fallback_folder_was_passed_over: passed_over,
        }
    }

    const THE_JOB_AREA: &str = "/ephemeral/smoozy/jobs";

    /// **P7b: a node's folder opening with an expression is no other computer's**
    /// (P19 case 1), which is what sent such a job to the fallback; and the job's
    /// folder is made by name, so its run stops before a folder holding a brace.
    #[test]
    fn p7b_an_expression_sends_no_job_to_the_fallback_and_makes_no_folder_with_a_brace() {
        use crate::reading::{ArtistsText, Destination, DestinationKind, RecordedLocation, WhereItComesFrom, WhichScene};
        let read = |kind: DestinationKind, origin: WhereItComesFrom, at: &str| {
            let mut one = Destination {
                kind,
                recorded_location: Some(RecordedLocation::of(at.as_bytes().to_vec())),
                format: None,
                origin,
                scene: WhichScene::TheJobsOwnScene,
                the_view_layers_that_feed_it: Vec::new(),
                the_location_with_its_folder_replaced: None,
            };
            one.put_in_the_replaced_text(Some(b"job"), TheVersionInHand::of(5, 2));
            one
        };
        let regular = |at: &str| read(DestinationKind::TheRegularOutputPath, WhereItComesFrom::TheScenesOutputPath, at);
        let a_node = |at: &str| {
            let name = ArtistsText::of(b"A".to_vec());
            read(
                DestinationKind::AnOutputOfAFileOutputNode {
                    node: name.clone(),
                    output: ArtistsText::of(b"Image".to_vec()),
                    the_nodes_base_file_name: None,
                    replaced: None,
                    the_effect: None,
                },
                WhereItComesFrom::AFileOutputNodeNamed(name),
                at,
            )
        };
        let version = TheVersionInHand::of(5, 2);
        let the_blends = b"/home/ubuntu/rendering";
        let run = |at: &str| TheRun::It(at.split('/').filter(|one| !one.is_empty()).map(|one| one.as_bytes().to_vec()).collect());
        assert_eq!(
            the_run_of(&[regular("//render/f_"), a_node("{project_root}/out")], the_blends, version),
            run("/home/ubuntu/rendering/render")
        );
        assert_eq!(
            the_run_of(&[regular("//{scene_name}/a/f_"), a_node("//{scene_name}/a/b")], the_blends, version),
            run("/home/ubuntu/rendering")
        );
        // **A path that truly is another computer's still is.**
        assert_eq!(
            the_run_of(&[regular("//render/f_"), a_node("C:/out")], the_blends, version),
            TheRun::ItIsAnotherComputers
        );
    }

    /// **P18a: a first-frame check and its range job share one folder.** The
    /// check's manifest carries the range job's name, so the check, rendering
    /// first, makes the folder; the range job then finds it standing and takes
    /// it, where the file points or at the fallback alike. **Something that is
    /// not a folder at that path is still a refusal.** The twenty-first notes,
    /// note 6.
    #[test]
    fn twenty_first_6_the_check_and_its_range_job_share_one_folder() {
        let run = a_run("/home/ubuntu/Render");
        let mut machine = AMachine::with(&["/home", "/home/ubuntu", "/home/ubuntu/Render"]);
        let the_checks = chosen(&run, None, &mut machine);
        let the_ranges = chosen(&run, None, &mut machine);
        assert_eq!(the_checks.path, format!("/home/ubuntu/Render/{NAME}"));
        assert_eq!(the_ranges, the_checks);
        assert_eq!(the_ranges.where_it_is, WhereItIs::WhereTheFilePoints);
        assert_eq!(machine.made, vec![format!("/home/ubuntu/Render/{NAME}")], "made once");

        // The same at the job area, where another computer's paths send both.
        let mut machine = AMachine::with(&[THE_JOB_AREA]);
        let the_checks = chosen(&TheRun::ItIsAnotherComputers, None, &mut machine);
        let the_ranges = chosen(&TheRun::ItIsAnotherComputers, None, &mut machine);
        assert_eq!(the_ranges, the_checks);
        assert_eq!(the_ranges.path, format!("{THE_JOB_AREA}/{NAME}"));

        // A refusal with nothing standing there is still N10's fault.
        let mut machine = AMachine::with(&[THE_JOB_AREA]);
        machine.refuse = Some(format!("{THE_JOB_AREA}/{NAME}"));
        let layout = a_layout();
        let refused = the_folder_is_chosen_and_made(
            &WhatTheChoiceIsMadeFrom {
                the_run: &TheRun::ItIsAnotherComputers,
                layout: &layout,
                the_job_output_folders_name: NAME,
                the_servers_fallback_folder: None,
            },
            &mut |_| true,
            &mut machine,
        );
        assert_eq!(refused, Err(TheJobAreaRefusedTheFolder));
    }

    #[test]
    fn the_creators_file_puts_the_jobs_folder_where_its_outputs_point_and_makes_what_is_missing() {
        // P18: `//../Render/PNG`, `//../Render/EXR` and `//../Render/Animation`
        // from `/home/ubuntu/rendering` share `/home/ubuntu/Render`, which is
        // not there yet: it is made, then the job's folder in it (P20).
        let blend = b"/home/ubuntu/rendering";
        let a_node = |node: &str, output: &str, at: &str| {
            a_destination(
                DestinationKind::AnOutputOfAFileOutputNode {
                    the_effect: None,
                    node: ArtistsText::of(node.as_bytes().to_vec()),
                    output: ArtistsText::of(output.as_bytes().to_vec()),
                    the_nodes_base_file_name: None,
                    replaced: None,
                },
                at,
                WhereItComesFrom::AFileOutputNodeNamed(ArtistsText::of(node.as_bytes().to_vec())),
            )
        };
        let destinations = vec![
            a_destination(DestinationKind::TheRegularOutputPath, "//../Render/Animation", WhereItComesFrom::TheScenesOutputPath),
            a_node("PNG", "### - Box", "//../Render/PNG"),
            a_node("EXR", "Image", "//../Render/EXR"),
            // **A bake left at the `.blend`'s own folder takes no part** (P9).
            a_destination(
                DestinationKind::AnAddedWritePlace(AnAddedWritePlace::TheBakeOutputPath),
                "//",
                WhereItComesFrom::AnAddedWritePlaceNamed(AnAddedWritePlace::TheBakeOutputPath),
            ),
        ];
        let run = the_run_of(&destinations, blend, TheVersionInHand::of(5, 2));
        assert_eq!(run, a_run("/home/ubuntu/Render"));
        let mut machine = AMachine::with(&["/home", "/home/ubuntu"]);
        let sits = chosen(&run, None, &mut machine);
        assert_eq!(sits.path, format!("/home/ubuntu/Render/{NAME}"));
        assert_eq!(sits.where_it_is, WhereItIs::WhereTheFilePoints);
        assert_eq!(sits.the_folder_it_sits_in(), "/home/ubuntu/Render");
        assert_eq!(machine.made, vec!["/home/ubuntu/Render".to_string(), format!("/home/ubuntu/Render/{NAME}")]);
    }

    #[test]
    fn another_computers_paths_fall_back_with_case_one() {
        // A foreign form, a run of `/` alone, and a Mac's `/Users/…` that
        // would need a folder made directly under `/`.
        for run in [TheRun::ItIsAnotherComputers, a_run("/Users/alex/Render")] {
            let mut machine = AMachine::with(&[THE_JOB_AREA]);
            let sits = chosen(&run, None, &mut machine);
            assert_eq!(sits.path, format!("{THE_JOB_AREA}/{NAME}"));
            assert_eq!(sits.where_it_is, fell_back(Some(WhyTheFilesOwnPlaceWasNotUsed::ItIsAnotherComputers), false));
            assert_eq!(machine.made, vec![format!("{THE_JOB_AREA}/{NAME}")], "nothing made at `/`");
        }
        let destinations = vec![a_destination(
            DestinationKind::TheRegularOutputPath,
            "C:\\Renders\\",
            WhereItComesFrom::TheScenesOutputPath,
        )];
        assert_eq!(the_run_of(&destinations, b"/home/ubuntu", TheVersionInHand::of(5, 2)), TheRun::ItIsAnotherComputers);
    }

    #[test]
    fn smoozys_own_areas_a_temporary_area_a_refused_make_and_the_budget_fall_back_with_case_two() {
        for run in ["/ephemeral/smoozy/assets/set-1", "/ephemeral/smoozy", "/tmp", "/var/tmp/renders"] {
            let mut machine = AMachine::with(&[THE_JOB_AREA, "/tmp", "/var", "/var/tmp", "/ephemeral", "/ephemeral/smoozy"]);
            let sits = chosen(&a_run(run), None, &mut machine);
            assert_eq!(sits.where_it_is, fell_back(Some(WhyTheFilesOwnPlaceWasNotUsed::ItIsAFolderSmoozyCannotUse), false), "{run}");
            assert_eq!(sits.path, format!("{THE_JOB_AREA}/{NAME}"));
        }
        // **A refused make leaves what was made on the way** (P20).
        let mut machine = AMachine::with(&["/home", THE_JOB_AREA]);
        machine.refuse = Some("/home/ubuntu/Render".to_string());
        let sits = chosen(&a_run("/home/ubuntu/Render"), None, &mut machine);
        assert_eq!(sits.where_it_is, fell_back(Some(WhyTheFilesOwnPlaceWasNotUsed::ItIsAFolderSmoozyCannotUse), false));
        assert!(machine.folders.contains("/home/ubuntu"), "the folder made on the way is left");
        // **N8's budget, asked of the candidate and not of the job area.**
        let layout = a_layout();
        let mut asked = Vec::new();
        let mut machine = AMachine::with(&["/home", "/home/ubuntu", THE_JOB_AREA]);
        let sits = the_folder_is_chosen_and_made(
            &WhatTheChoiceIsMadeFrom { the_run: &a_run("/home/ubuntu"), layout: &layout, the_job_output_folders_name: NAME, the_servers_fallback_folder: None },
            &mut |path| {
                asked.push(path.to_string());
                false
            },
            &mut machine,
        )
        .unwrap();
        assert_eq!(sits.where_it_is, fell_back(Some(WhyTheFilesOwnPlaceWasNotUsed::ItIsAFolderSmoozyCannotUse), false));
        assert_eq!(asked, vec![format!("/home/ubuntu/{NAME}")]);
        assert!(!machine.folders.contains(&format!("/home/ubuntu/{NAME}")), "nothing made where it will not fit");
    }

    #[test]
    fn a_chosen_fallback_folder_is_used_and_empty_means_the_job_area() {
        let mut machine = AMachine::with(&["/mnt", THE_JOB_AREA]);
        let sits = chosen(&TheRun::ItIsAnotherComputers, Some(" /mnt/renders/ "), &mut machine);
        assert_eq!(sits.path, format!("/mnt/renders/{NAME}"));
        assert_eq!(sits.where_it_is, fell_back(Some(WhyTheFilesOwnPlaceWasNotUsed::ItIsAnotherComputers), false));
        for empty in [None, Some(""), Some("   ")] {
            let mut machine = AMachine::with(&[THE_JOB_AREA]);
            let sits = chosen(&TheRun::ItIsAnotherComputers, empty, &mut machine);
            assert_eq!(sits.path, format!("{THE_JOB_AREA}/{NAME}"));
            assert_eq!(sits.where_it_is, fell_back(Some(WhyTheFilesOwnPlaceWasNotUsed::ItIsAnotherComputers), false));
        }
        // **A job with no render destination takes the fallback's place
        // without a reason** (P18), so the chosen folder (P23).
        let mut machine = AMachine::with(&["/mnt", THE_JOB_AREA]);
        let sits = chosen(&TheRun::ThereIsNone, Some("/mnt/renders"), &mut machine);
        assert_eq!(sits.path, format!("/mnt/renders/{NAME}"));
        assert_eq!(sits.where_it_is, fell_back(None, false));
    }

    #[test]
    fn a_chosen_folder_that_fails_goes_to_the_job_area_and_says_it_was_passed_over() {
        let cases: [(&str, &[&str], Option<&str>); 6] = [
            ("/ephemeral/smoozy/mine", &["/ephemeral", "/ephemeral/smoozy"], None),
            ("/tmp/renders", &["/tmp"], None),
            ("renders", &[], None),
            ("/home/../tmp", &["/home"], None),
            ("/Volumes/Work", &[], None),
            ("/mnt/renders", &["/mnt"], Some("/mnt/renders")),
        ];
        for (folder, stands, refused) in cases {
            let mut machine = AMachine::with(stands);
            machine.folders.insert(THE_JOB_AREA.to_string());
            machine.refuse = refused.map(str::to_string);
            let sits = chosen(&TheRun::ItIsAnotherComputers, Some(folder), &mut machine);
            assert_eq!(sits.path, format!("{THE_JOB_AREA}/{NAME}"), "{folder}");
            assert_eq!(sits.where_it_is, fell_back(Some(WhyTheFilesOwnPlaceWasNotUsed::ItIsAnotherComputers), true), "{folder}");
        }
        // **A refused make at the chosen folder leaves the folders made on the
        // way** (P23, P20).
        let mut machine = AMachine::with(&["/mnt", THE_JOB_AREA]);
        machine.refuse = Some(format!("/mnt/renders/deep/{NAME}"));
        let sits = chosen(&TheRun::ItIsAnotherComputers, Some("/mnt/renders/deep"), &mut machine);
        assert_eq!(sits.where_it_is, fell_back(Some(WhyTheFilesOwnPlaceWasNotUsed::ItIsAnotherComputers), true));
        assert!(machine.folders.contains("/mnt/renders/deep"));
    }

    #[test]
    fn a_path_that_resolves_on_the_server_ignores_the_chosen_folder() {
        let mut machine = AMachine::with(&["/home", "/home/ubuntu", "/home/ubuntu/Render", "/mnt"]);
        let sits = chosen(&a_run("/home/ubuntu/Render"), Some("/mnt/renders"), &mut machine);
        assert_eq!(sits.path, format!("/home/ubuntu/Render/{NAME}"));
        assert_eq!(sits.where_it_is, WhereItIs::WhereTheFilePoints);
        assert!(!machine.folders.contains("/mnt/renders"), "nothing made in the chosen folder");
    }

    #[test]
    fn the_job_area_refusing_a_fresh_folder_is_n10s_fault() {
        let layout = a_layout();
        let mut machine = AMachine::with(&[THE_JOB_AREA]);
        machine.refuse = Some(format!("{THE_JOB_AREA}/{NAME}"));
        let refused = the_folder_is_chosen_and_made(
            &WhatTheChoiceIsMadeFrom { the_run: &TheRun::ItIsAnotherComputers, layout: &layout, the_job_output_folders_name: NAME, the_servers_fallback_folder: None },
            &mut |_| true,
            &mut machine,
        );
        assert_eq!(refused, Err(TheJobAreaRefusedTheFolder));
    }

    /// **`Job files and outputs` P21's note of 2026-11-02: the page and the
    /// machine ask one question and give one answer.** The live pass on real
    /// production files: `/shared/studio/…` on a machine with no `/shared`. Each
    /// run, the machine's own choice and the page's with the machine's top in
    /// hand. A folder missing below the top is simply made, as P20 has it.
    #[test]
    fn the_page_and_the_machine_give_one_answer_about_a_folder_at_the_top() {
        let layout = a_layout();
        let the_top = ["home".to_string(), "root".to_string(), "ephemeral".to_string()];
        for run in ["/shared/studio/weeklies/hjalti", "/Users/hjalti/Render", "/home/ubuntu/Render", "/home/gone/deep/Render", "/root/x"] {
            let the_run = a_run(run);
            let from = WhatTheChoiceIsMadeFrom { the_run: &the_run, layout: &layout, the_job_output_folders_name: NAME, the_servers_fallback_folder: None };
            let mut machine = AMachine::with(&["/home", "/home/ubuntu", "/root", "/ephemeral", "/ephemeral/smoozy", THE_JOB_AREA]);
            let chose = the_folder_is_chosen_and_made(&from, &mut |_| true, &mut machine).expect("a place");
            let says = where_it_would_sit(&from, Some(&the_top), &mut |_| true);
            assert_eq!(says, chose, "{run}");
        }
        let the_run = a_run("/shared/studio/weeklies/hjalti");
        let from = WhatTheChoiceIsMadeFrom { the_run: &the_run, layout: &layout, the_job_output_folders_name: NAME, the_servers_fallback_folder: None };
        let says = where_it_would_sit(&from, Some(&the_top), &mut |_| true);
        assert!(matches!(
            says.where_it_is,
            WhereItIs::TheFallback { why: Some(WhyTheFilesOwnPlaceWasNotUsed::ItIsAnotherComputers), .. }
        ));
        // **With no list every folder is taken to stand**: the page then claims nothing.
        assert_eq!(where_it_would_sit(&from, None, &mut |_| true).where_it_is, WhereItIs::WhereTheFilePoints);
    }

    #[test]
    fn what_the_page_says_is_the_same_choice_with_nothing_made() {
        let layout = a_layout();
        let ask = |run: &TheRun, fallback: Option<&str>| {
            where_it_would_sit(
                &WhatTheChoiceIsMadeFrom { the_run: run, layout: &layout, the_job_output_folders_name: NAME, the_servers_fallback_folder: fallback },
                None,
                &mut |_| true,
            )
        };
        assert_eq!(ask(&a_run("/home/ubuntu/Render"), None).the_folder_it_sits_in(), "/home/ubuntu/Render");
        let fell = ask(&a_run("/tmp"), Some("/mnt/renders"));
        assert_eq!(fell.the_folder_it_sits_in(), "/mnt/renders");
        let passed = ask(&a_run("/tmp"), Some("renders"));
        assert_eq!(passed.where_it_is, fell_back(Some(WhyTheFilesOwnPlaceWasNotUsed::ItIsAFolderSmoozyCannotUse), true));
        assert_eq!(passed.the_folder_it_sits_in(), THE_JOB_AREA);
    }

    #[test]
    fn the_record_reads_back_as_itself_and_a_manifests_folder_wins() {
        let sits = WhereTheJobOutputFolderSits {
            path: "/mnt/renders/job".to_string(),
            where_it_is: fell_back(Some(WhyTheFilesOwnPlaceWasNotUsed::ItIsAnotherComputers), true),
        };
        assert_eq!(WhereTheJobOutputFolderSits::read_back(&sits.as_written()), Some(sits.clone()));
        assert_eq!(the_job_output_folder_in_force(None, Some(&sits)).as_deref(), Some("/mnt/renders/job"));
        assert_eq!(the_job_output_folder_in_force(Some("/a/b"), Some(&sits)).as_deref(), Some("/a/b"));
        assert_eq!(the_job_output_folder_in_force(None, None), None);
    }
}
