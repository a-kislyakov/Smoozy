//! The remap — `Dependency Upload & Remap` §6.1, §6.2, §6.3's judgements, §6.4
//! and §6.7, Rows 174 to 177 and 312: what the material says, how a pair is
//! matched, where the material is kept, and the uploaded cache.
//!
//! **M1: the route is `Job files and outputs` 5.2 D9 and is not re-opened** —
//! the remap material rides the same launch material as the output redirect,
//! applied by the same lever at the same moment, composed per version,
//! printing the marker of its D16, containing no failure handling of its own
//! (D18), delivered inside the one command-line window that exists — after
//! the file, before the render instruction — and confirmed placed by a
//! read-back (D19).
//!
//! **M2: the artist's file is never written to, and the mechanism honours it
//! twice over** — the material sets values in the running Blender and nothing
//! saves the top file (D5), and every fixing write of 6.5 happens to Smoozy's
//! own copy.
//!
//! **M3: what is this document's** — the pairs and how each is applied, where
//! the remap is kept, the two copies of a linked `.blend`, and the reach
//! across kinds. **Applying the material inside a running Blender is the
//! acting half's**; what is here is the pair, its match text, the matching
//! rule, the marker's per-pair fields, and the material's document.
//!
//! **M9a: `image.has_data` is poison and nothing here reads it.** It reported
//! `False` throughout while the render was reading the file correctly, so
//! *"anything that gates on it, or reports from it, will call a working remap
//! broken"*. **Nothing in this crate reads it, no test stands one up as
//! evidence, and the marker reports what was assigned and never what a flag
//! says about it** — [`NOTHING_HERE_READS_HAS_DATA`] is held by a test that
//! reads this crate's own sources.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Job files and outputs` D5, D9, D16, D18, D19,
//! D20, D21e, L7a. `Download` N5a. `Server protocol and transfers` S13.
//!
//! **`Job files and outputs` M1 is not listed above and is written out at the
//! site**, because this plan defines an M1 of its own.

use smoozy_job_files::layout::TheLayout;
use smoozy_server_protocol::transfers::SourceReading;

use crate::bytes::{MachineLocation, RecordedLocation};
use crate::check::Kind;
use crate::record::{FixingRecord, HoldingFile, LocatedAnswer};

/// **The pairs, the material and the judgements over one pair, at the paths
/// they have always been reached at.** They live in
/// `smoozy-dependencies-machine`, the half of this plan the rented machine's
/// own program may take, and nothing about any of them changed with the move.
/// What stays here is the composition over the record. Split at step 10c
/// piece 3.
pub use smoozy_dependencies_machine::remap::{
    a_current_value_matches, assignment_alone_is, compose_pair, takes_a_reload_after_assignment,
    the_outcome_of, AssignmentAlone, ComposedFor, MatchText, NewLocation, NotComposed, Pair,
    PairOutcome, RemapMaterial, EVERY_PAIR_IS_APPLIED_BY_THE_PER_SETTING_ROUTE,
};

/// **M7's first half: no pair is ever composed for a curly-brace expression,
/// and it is kept by construction rather than by a test over the bytes.** G9
/// allows a curly-brace expression in the render output path and the File
/// Output node; both are [`crate::check::WriteOnlyDestination`]s that produce no
/// reference, so nothing this module ever composes a pair from can be one. **From
/// 5.3 a geometry-nodes bake folder may hold one too (G9a)**, and it is one of
/// [`THE_WRITE_PLACES_THE_REDIRECT_ASSIGNS`], which take no pair.
///
/// **This replaces a text test that cost the user a row.** `classify` used to
/// call any location holding a `{` anywhere and a `}` anywhere a template:
/// `Form::is_searched` then kept a folder honestly named `{hero}` out of the
/// walk, so the row read *not found* while the file sat where the `.blend`
/// said — and, the silent half, `compose_pair` refused a pair for it, so even
/// after the user located it by hand and sent it, the reference was never
/// re-pointed and the render looked for it on the artist's own computer.
/// Removed 2026-09-05, out of round 1 of this module's code audit. **M7 is
/// kept, on G9's own reason.**
pub const NO_PAIR_IS_COMPOSED_FOR_A_CURLY_BRACE_EXPRESSION: bool = true;

/// **M22a: the four write places `Job files and outputs` D20 maps under the
/// job's folder take the job-folder value the redirect assigns them, so this
/// module composes no pair for one** — the bake output path, the physics
/// point cache, the geometry-nodes bake folder and the sequencer proxy
/// folder. Where the user sent a bake for such a place, it is **placed** at
/// that job-folder location (M22c) rather than pointed at, which is what
/// dissolved the collision. **The material carries the one pair the ruling
/// settles, which is the redirect's own** (M22d, §11). Creator-stated
/// (2026-08-22).
pub const THE_WRITE_PLACES_THE_REDIRECT_ASSIGNS: [Kind; 4] = [
    Kind::BakeFolder,
    Kind::PhysicsPointCache,
    Kind::GeometryNodeBakeFolder,
    Kind::ProxyFolder,
];

/// **M10: one located answer as the composition meets it, once for each kind
/// and setting the check read it as.** All of it comes from the record: the
/// answer, key included, and the kind and the setting from K4's last limb
/// ([`LocatedAnswer::read_as`]) — [`the_answers_to_compose`].
///
/// **The case this was first written for, and the case it now meets.** It
/// was written with the kind and the setting taken from the reading, because
/// K4 held neither; so the pairs could be composed only in the run that did
/// the check, and after a restart a checked and sent file was submitted with
/// none. K4 gained them on 2026-09-29 (step 10c's dependency wave piece 7b).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnAnswerToCompose {
    pub answer: LocatedAnswer,
    pub kind: Kind,
    /// The setting is a property path rather than an instance (M8).
    pub setting: String,
    /// **Q9: where the reference was pointed at a copy already on the
    /// machine**, that copy's own machine location. Where this is set the
    /// mirror rule is not used, because the file did not come up as a mirror.
    pub already_on_the_machine: Option<MachineLocation>,
}

/// **M10 from the record alone: each answer once for every kind and setting
/// K4 says the check read it as**, in the record's order and then the
/// reading's. **An answer no reading has written them into is handed back
/// beside the rest**, so M10 can say which answers gave nothing and why,
/// rather than composing a pair with a kind it does not know.
///
/// **Q9's copy already up is never pointed at here**: its lookup reads a
/// published transfer record, which can never answer it (step 10c's
/// dependency wave piece 4's seam), so [`AnAnswerToCompose::already_on_the_machine`]
/// is nothing and the mirror rule gives every new location.
pub fn the_answers_to_compose(
    answers: Vec<LocatedAnswer>,
) -> (Vec<AnAnswerToCompose>, Vec<RecordedLocation>) {
    let mut to_compose = Vec::new();
    let mut without_a_kind = Vec::new();
    for answer in answers {
        if answer.read_as.is_empty() {
            without_a_kind.push(answer.key.reference.clone());
            continue;
        }
        for read_as in &answer.read_as {
            to_compose.push(AnAnswerToCompose {
                answer: answer.clone(),
                kind: read_as.kind,
                setting: read_as.setting.clone(),
                already_on_the_machine: None,
            });
        }
    }
    (to_compose, without_a_kind)
}

/// **Why one located answer produced no pair.** M10 asks for the pairs *and*
/// the reason any answer produced none, so nothing is dropped silently.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NoPairFor {
    /// **M4: pairs cover the top file's direct references only** —
    /// everything below the top was fixed where it sits, so a reference
    /// belonging to another holding file is not this material's.
    ItIsNotTheTopFilesOwnReference,
    /// The row is *not found* — F7's absence of a resolution. There is
    /// nothing to point it at.
    NothingWasResolved,
    /// K4's set-aside mark: the user said this row is not wanted.
    TheRowWasSetAside,
    /// **M22a**: the setting takes the job-folder value the redirect assigns.
    TheRedirectAssignsThisSetting,
    /// A5: the mirrored location could not be composed for the resolved file.
    TheMirrorCouldNotBeComposed(crate::landing::CouldNotBeSent),
    /// **M4: a library pair never names a copy that has not been fixed**, and
    /// no fixing record on this machine names the machine copy this reference
    /// resolved to.
    NoFixedCopyHasBeenProducedForIt,
    /// What [`compose_pair`] itself refused.
    ItCouldNotBeComposed(NotComposed),
}

/// **M10's composition: the pairs, and the reason any answer produced none.**
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TheComposedPairs {
    pub pairs: Vec<Pair>,
    pub without_a_pair: Vec<(RecordedLocation, NoPairFor)>,
}

/// **M10: the composition happens on the desktop, from the record, at
/// submission — the located answers and the landing rule give the pairs.**
/// This is that judgement: it walks the root's located answers, keeps the top
/// file's own references, takes each one's kind and setting from what the check
/// read it as (K4),
/// takes its new location from the mirror rule, the fixed-copy record or Q9's
/// already-up copy, and refuses what M4, M7 and M22a exclude.
///
/// **What it refuses by construction rather than by care.**
///
/// - **A reference below the top file** — decided from the answer's own key
///   against the top file, never from a flag a caller set (M4).
/// - **A library pair naming a copy that has not been fixed** — the fixed
///   copy's address comes from a fixing record naming the machine copy this
///   reference resolved to, so there is nothing to compose from where no
///   fixing has been produced (M4).
/// - **A curly-brace expression** — [`NO_PAIR_IS_COMPOSED_FOR_A_CURLY_BRACE_EXPRESSION`],
///   G9's own reason, and **a path held in a custom property** —
///   [`NO_PAIR_IS_COMPOSED_FOR_A_CUSTOM_PROPERTY`], because there is no
///   [`Kind`] for one (M7).
/// - **D20's four write places** — [`THE_WRITE_PLACES_THE_REDIRECT_ASSIGNS`]
///   (M22a).
pub fn compose_the_pairs(
    layout: &TheLayout,
    the_top_file: &HoldingFile,
    answers: &[AnAnswerToCompose],
    fixings: &[FixingRecord],
) -> TheComposedPairs {
    let mut composed = TheComposedPairs::default();
    for answer in answers {
        let reference = answer.answer.key.reference.clone();
        match one_pair(layout, the_top_file, answer, fixings) {
            Ok(pair) => composed.pairs.push(pair),
            Err(why) => composed.without_a_pair.push((reference, why)),
        }
    }
    composed
}

fn one_pair(
    layout: &TheLayout,
    the_top_file: &HoldingFile,
    answer: &AnAnswerToCompose,
    fixings: &[FixingRecord],
) -> Result<Pair, NoPairFor> {
    if &answer.answer.key.holding_file != the_top_file {
        return Err(NoPairFor::ItIsNotTheTopFilesOwnReference);
    }
    if answer.answer.set_aside {
        return Err(NoPairFor::TheRowWasSetAside);
    }
    if THE_WRITE_PLACES_THE_REDIRECT_ASSIGNS.contains(&answer.kind) {
        return Err(NoPairFor::TheRedirectAssignsThisSetting);
    }
    let resolution = answer
        .answer
        .resolution
        .as_ref()
        .ok_or(NoPairFor::NothingWasResolved)?;

    // The machine copy the reference now reads from: Q9's already-up copy
    // where the user was pointed at one, else A4's mirror of what it
    // resolved to.
    let machine_copy = match &answer.already_on_the_machine {
        Some(at) => at.clone(),
        None => crate::landing::mirror_of(layout, resolution.to.location())
            .map_err(NoPairFor::TheMirrorCouldNotBeComposed)?,
    };
    let machine_copy = the_sets_file_named(machine_copy, resolution, &answer.answer.key.reference)
        .map_err(NoPairFor::TheMirrorCouldNotBeComposed)?;

    let new = if answer.kind == Kind::Library {
        // **M4: a library pair never names a copy that has not been fixed.**
        // The address is the fixing record's, so an unfixed copy has nothing
        // to compose from.
        let fixed = fixings
            .iter()
            .find(|fixing| fixing.derived_from == machine_copy)
            .map(|fixing| fixing.fixed_copy.clone())
            .ok_or(NoPairFor::NoFixedCopyHasBeenProducedForIt)?;
        NewLocation::AFixedCopy(fixed)
    } else if answer.already_on_the_machine.is_some() {
        NewLocation::ACopyAlreadyUp(machine_copy)
    } else {
        NewLocation::AMirror(machine_copy)
    };

    compose_pair(
        answer.kind,
        &answer.setting,
        &answer.answer.key.reference,
        new,
    )
    .map_err(NoPairFor::ItCouldNotBeComposed)
}

/// **M4b: a set row's new location keeps the file's name, marks and all.** The
/// answer is the folder the check looked in (K4, G22), and the setting holds a
/// file's name. **Only where the recorded location's own last segment carries a
/// set's marks** (G9): a bake's folder is answered the same way, carries none and
/// its setting holds a folder.
fn the_sets_file_named(
    machine_copy: MachineLocation,
    resolution: &crate::record::Resolution,
    reference: &RecordedLocation,
) -> Result<MachineLocation, crate::landing::CouldNotBeSent> {
    let a_set = matches!(resolution.to, crate::record::ResolvedTo::AFolderForASetRow(_))
        && matches!(
            crate::check::classify(reference, crate::check::StoredSource::NotSaidToBeOneFile),
            crate::check::Form::ASet(_)
        );
    if !a_set {
        return Ok(machine_copy);
    }
    let whole = crate::bytes::join(machine_copy.as_bytes(), reference.wanted_name());
    crate::landing::fits_the_budgets(&whole).map_err(crate::landing::CouldNotBeSent::TheMirrorWillNotFit)?;
    Ok(MachineLocation::composed(whole))
}

/// **M7's other half: no pair is ever composed for a path held in a custom
/// property, because the check never finds one (G7)** — there is no
/// [`Kind`] for it, so no pair can name one.
///
/// **M20: the reach limits arrive symmetrically in the check and the remap.**
/// This is the remap's half of the one limit and
/// [`crate::check::A_CUSTOM_PROPERTY_PATH_IS_NOT_FOUND_AT_ALL`] is the check's.
pub const NO_PAIR_IS_COMPOSED_FOR_A_CUSTOM_PROPERTY: bool = true;

/// **M9b: the five kinds not posed and not claimed.** Cache file — dropped on
/// save, not present at load to be tested. Sound — a sound has no render to
/// observe, so *is it in force for the render* cannot be asked of it at all,
/// which is structural. Font, movie clip, volume — not posed. **Where the
/// entry says a kind may render silently missing its assets at success, that
/// consequence is recorded here rather than guarded against**: a remapped
/// reference of such a kind that silently needs a reload renders as missing —
/// a finished render, wrong, saying nothing — and what holds is the marker's
/// per-pair record and [`AssignmentAlone::NotEstablished`] on every such
/// pair.
pub const THE_FIVE_KINDS_NOT_POSED: [Kind; 5] = [
    Kind::CacheArchive,
    Kind::Sound,
    Kind::Font,
    Kind::MovieClip,
    Kind::Volume,
];

/// **M9a: nothing in this crate reads `image.has_data`.** A test reads this
/// crate's own sources and fails on the first line of code that names it.
pub const NOTHING_HERE_READS_HAS_DATA: bool = true;

/// **M19: the rewrite of seven kinds is inferred rather than watched** — a
/// font, a sound, a volume, a point cache, a bake folder, a geometry-nodes
/// bake folder and a sequencer proxy folder (Fact 77's limit list; **the
/// count is this document's own arithmetic over that list, and the list is
/// seven**, not the spec's differently counted six). **What breaks where it
/// fails for a kind**: a pair for that kind assigns nothing or assigns
/// without effect, the reference keeps pointing at the artist's machine, and
/// the render is silently missing it (§12 item 2, permanently unresolved).
/// What holds: M9d's field carries every one of these as *not established*.
pub const THE_SEVEN_INFERRED_KINDS: [Kind; 7] = [
    Kind::Font,
    Kind::Sound,
    Kind::Volume,
    Kind::PhysicsPointCache,
    Kind::BakeFolder,
    Kind::GeometryNodeBakeFolder,
    Kind::ProxyFolder,
];

/// **M8a: a whole-file comparison, or a checksum, says nothing about whether
/// a recorded place moved.** The two questions are independent and only the
/// path text answers the second one — which is why M4a matches on text and
/// why nothing anywhere in this module decides a pair's fate from a digest.
pub const NO_PAIRS_FATE_IS_DECIDED_FROM_A_DIGEST: bool = true;

/// **M11: the placement and the submission are one ordered unit** — the
/// material is placed first, and where the placement fails the submission is
/// **not attempted**, naming the material as the unit that stopped it. A job
/// therefore never exists with its remap material missing. Declared in
/// Section 10 ([`crate::units`]).
pub const THE_MATERIAL_IS_PLACED_FIRST_AND_A_FAILED_PLACEMENT_STOPS_THE_SUBMISSION: bool = true;

/// **M12: what Row 177 inherits and does not repair** — where the supervisor
/// cannot run on a full state mount, no re-run happens and the remap material
/// sits unread; `Rented-server runtime` carries the condition. The material's
/// survival story is the disk's, shared with every asset (A13).
pub const THE_MATERIALS_SURVIVAL_IS_THE_DISKS: bool = true;

/// **M22 and M22a — ruled by the Creator on 2026-08-22: the setting always
/// takes the job-folder value `Job files and outputs` D20 assigns it, and
/// where the user sent a bake for that write place, Smoozy places it at that
/// job-folder location before the render starts.** The collision dissolves
/// rather than picking a side. Label: Creator-stated.
pub const THE_SETTING_TAKES_THE_JOB_FOLDER_VALUE_AND_THE_UPLOADED_BAKE_IS_PLACED_THERE: bool = true;

/// **M22b: three wants, and the one value serves all three** — bake at home
/// and send it up; let the server bake it and bring it home; and both — the
/// third needing M22f's placed-copy record and nothing else.
pub const ONE_VALUE_SERVES_ALL_THREE_WANTS: bool = true;

/// **M22c: the upload still lands in the asset area first, immutable and
/// shared; what goes into the job folder is a copy of it, made on the
/// machine.** A second job wanting the same bake copies on the machine rather
/// than uploading again, and the master is never at risk.
pub const THE_JOB_FOLDER_COPY_IS_MADE_ON_THE_MACHINE_FROM_THE_MIRROR: bool = true;

/// **M22d: what the ruling obliges elsewhere, all written** — `Job files and
/// outputs` L7a's one exception, `Download` N5a's skip on M22f's record, and
/// the remap material carrying the one pair the ruling settles.
pub const THE_MATERIAL_CARRIES_THE_ONE_PAIR_THE_RULING_SETTLES: bool = true;

/// **M22e: what this does not claim** — that the render will read the placed
/// bake rather than recompute anyway (`Job files and outputs` D21e; §12 item
/// 7, permanently unresolved). What the ruling guarantees is that the bake is
/// where the setting points.
pub const NOTHING_CLAIMS_THE_RENDER_READS_THE_PLACED_BAKE: bool = true;

/// **M22f: the placed copy is recorded, and that record is what `Download`
/// skips on.** Per copy: the **exact job-folder path** written, and the
/// **four-value identity of that copy read the moment it was made** — the one
/// shape `Server protocol and transfers` S13, `Job files and outputs` R10 and
/// `Dependency Upload & Remap` K7 already use. A field of the job's own local
/// record, not a sixth kind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlacedCopy {
    pub job_folder_path: MachineLocation,
    pub four_values_at_placement: SourceReading,
}

impl PlacedCopy {
    /// **M22f: what the four values separate — *untouched* from *written
    /// over*.** A render that only reads the placed bake moves none of them;
    /// a render that writes there moves at least the modification and change
    /// times, and a save-by-rename moves the inode as well. It says nothing
    /// about whether the render read the bake (M22e).
    pub fn was_written_over(&self, four_values_now: &SourceReading) -> bool {
        !four_values_now.is_the_same_file_as(&self.four_values_at_placement)
    }
}

/// The job's placed-copy list.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PlacedCopyList {
    pub copies: Vec<PlacedCopy>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::check::tests::a_blender;
    use crate::landing::tests::a_layout;
    use smoozy_state::identities::{Identity, IdentityKind};

    fn recorded(bytes: &[u8]) -> RecordedLocation {
        RecordedLocation::as_the_reading_carried_it(bytes)
    }

    fn mirror(bytes: &[u8]) -> NewLocation {
        NewLocation::AMirror(MachineLocation::composed(bytes.to_vec()))
    }

    fn stored(bytes: &[u8]) -> crate::bytes::StoredLocation {
        crate::bytes::StoredLocation::as_the_system_returned_it(bytes)
    }

    fn the_top_file() -> HoldingFile {
        HoldingFile::OnTheUsersComputer(stored(b"/Users/a/proj/scene.blend"))
    }

    fn an_answer(kind: Kind, reference: &[u8], resolved: Option<&[u8]>) -> AnAnswerToCompose {
        AnAnswerToCompose {
            answer: LocatedAnswer {
                key: crate::record::LocatedAnswerKey {
                    holding_file: the_top_file(),
                    reference: recorded(reference),
                },
                resolution: resolved.map(|at| crate::record::Resolution {
                    to: crate::record::ResolvedTo::AWholeLocation(stored(at)),
                    how: crate::record::HowResolved::TheRecordedLocationItself,
                }),
                set_aside: false,
                rejected_guess: None,
                read_as: Vec::new(),
            },
            kind,
            setting: "filepath".to_string(),
            already_on_the_machine: None,
        }
    }

    /// **M4b: a set row's new location keeps the file's name, marks and all.**
    /// A live pass on a real shot: the location ended at the folder, Blender
    /// said *is a directory, cannot use as image*, and the job ended
    /// *Finished* with the textures missing.
    #[test]
    fn a_set_rows_new_location_keeps_the_files_name() {
        let layout = a_layout();
        let in_a_folder = |reference: &[u8], folder: &[u8]| {
            let mut answer = an_answer(Kind::Image, reference, Some(folder));
            answer.answer.resolution = Some(crate::record::Resolution {
                to: crate::record::ResolvedTo::AFolderForASetRow(stored(folder)),
                how: crate::record::HowResolved::TheRecordedLocationItself,
            });
            answer
        };
        for (reference, name) in [
            (&b"//maps/set/tile_<UDIM>.tif"[..], &b"tile_<UDIM>.tif"[..]),
            (b"//maps/set/tile.<UVTILE>.png", b"tile.<UVTILE>.png"),
            (b"//seq/frame_####.exr", b"frame_####.exr"),
            (b"//views/eye_%.png", b"eye_%.png"),
        ] {
            let composed = compose_the_pairs(
                &layout,
                &the_top_file(),
                &[in_a_folder(reference, b"/Work/maps/set")],
                &[],
            );
            let folder = crate::landing::mirror_of(&layout, &stored(b"/Work/maps/set")).unwrap();
            let mut wanted = folder.as_bytes().to_vec();
            wanted.push(b'/');
            wanted.extend_from_slice(name);
            assert_eq!(composed.pairs[0].new_location.as_bytes(), &wanted[..]);
        }

        // **No marks in the last segment: the folder, as it always was** — a
        // folder-shaped reference answered the same way, whose setting holds a folder.
        let mut folder_shaped = in_a_folder(b"//strips/", b"/Work/strips");
        folder_shaped.kind = Kind::Volume;
        let composed = compose_the_pairs(&layout, &the_top_file(), &[folder_shaped], &[]);
        assert_eq!(
            composed.pairs[0].new_location,
            crate::landing::mirror_of(&layout, &stored(b"/Work/strips")).unwrap()
        );

        // **A name held whole stays as written.**
        let whole = an_answer(
            Kind::Image,
            b"//tex/tile_<UDIM>.png",
            Some(b"/Work/tex/tile_<UDIM>.png"),
        );
        let composed = compose_the_pairs(&layout, &the_top_file(), &[whole], &[]);
        assert_eq!(
            composed.pairs[0].new_location,
            crate::landing::mirror_of(&layout, &stored(b"/Work/tex/tile_<UDIM>.png")).unwrap()
        );
    }

    #[test]
    fn the_composition_takes_the_top_files_answers_and_names_the_reason_for_every_one_without_a_pair(
    ) {
        let layout = a_layout();
        let image = an_answer(
            Kind::Image,
            b"//tex/metallic.png",
            Some(b"/Users/a/proj/tex/metallic.png"),
        );
        let not_found = an_answer(Kind::Image, b"//tex/gone.png", None);
        let mut set_aside = an_answer(
            Kind::Image,
            b"//tex/wood.png",
            Some(b"/Users/a/proj/tex/wood.png"),
        );
        set_aside.answer.set_aside = true;
        let bake = an_answer(Kind::BakeFolder, b"//bake/", Some(b"/Users/a/proj/bake"));
        let mut below = an_answer(
            Kind::Image,
            b"//tex/below.png",
            Some(b"/Users/a/proj/tex/below.png"),
        );
        below.answer.key.holding_file =
            HoldingFile::OnTheUsersComputer(stored(b"/Users/a/proj/lib/chairs.blend"));
        let library = an_answer(
            Kind::Library,
            b"//lib/chairs.blend",
            Some(b"/Users/a/proj/lib/chairs.blend"),
        );

        let answers = vec![image, not_found, set_aside, bake, below, library];
        let composed = compose_the_pairs(&layout, &the_top_file(), &answers, &[]);

        assert_eq!(composed.pairs.len(), 1);
        assert_eq!(composed.pairs[0].kind, Kind::Image);
        assert!(composed.pairs[0]
            .new_location
            .as_bytes()
            .ends_with(b"/files/Users/a/proj/tex/metallic.png"));
        let reasons: Vec<&NoPairFor> = composed.without_a_pair.iter().map(|(_, why)| why).collect();
        assert_eq!(
            reasons,
            vec![
                &NoPairFor::NothingWasResolved,
                &NoPairFor::TheRowWasSetAside,
                &NoPairFor::TheRedirectAssignsThisSetting,
                &NoPairFor::ItIsNotTheTopFilesOwnReference,
                &NoPairFor::NoFixedCopyHasBeenProducedForIt,
            ]
        );
    }

    #[test]
    fn a_library_pair_names_the_fixed_copy_the_record_holds_and_nothing_else() {
        let layout = a_layout();
        let library = an_answer(
            Kind::Library,
            b"//lib/chairs.blend",
            Some(b"/Users/a/proj/lib/chairs.blend"),
        );
        let mirror_of_it =
            crate::landing::mirror_of(&layout, &stored(b"/Users/a/proj/lib/chairs.blend"))
                .expect("a mirror");
        let fixed = crate::landing::fixed_copy_address(&layout, &mirror_of_it).expect("an address");
        let fixing = FixingRecord {
            machine: crate::record::tests::a_machine(),
            derived_from: mirror_of_it,
            derived_from_content: crate::contents::ContentIdentity::of(b"chairs"),
            fixed_copy: fixed.clone(),
            points_at: vec![],
        };
        let composed = compose_the_pairs(
            &layout,
            &the_top_file(),
            std::slice::from_ref(&library),
            &[fixing],
        );
        assert_eq!(composed.pairs.len(), 1);
        assert_eq!(composed.pairs[0].new_location, fixed);
        assert!(composed.without_a_pair.is_empty());

        // A fixing record for some other machine copy is not this one's.
        let composed = compose_the_pairs(&layout, &the_top_file(), &[library], &[]);
        assert!(composed.pairs.is_empty());
        assert_eq!(
            composed.without_a_pair[0].1,
            NoPairFor::NoFixedCopyHasBeenProducedForIt
        );
    }

    #[test]
    fn the_match_text_replaces_backslashes_and_nothing_else() {
        assert_eq!(
            MatchText::of(b"//tex\\sub\\METallic .png").as_bytes(),
            b"//tex/sub/METallic .png"
        );
        assert_eq!(
            MatchText::of(b"//tex//x/../y.png").as_bytes(),
            b"//tex//x/../y.png"
        );
        assert_eq!(
            MatchText::of("//tex/stra\u{df}e.png".as_bytes()).as_bytes(),
            "//tex/stra\u{df}e.png".as_bytes()
        );
    }

    #[test]
    fn a_pair_survives_the_separator_re_spelling_in_either_direction_and_misses_a_re_pointed_reference(
    ) {
        let pair = compose_pair(
            Kind::Image,
            "filepath",
            &recorded(b"//tex\\metallic.png"),
            mirror(b"/m/x"),
        )
        .expect("composed");
        assert!(a_current_value_matches(&pair, b"//tex/metallic.png"));
        assert!(a_current_value_matches(&pair, b"//tex\\metallic.png"));
        assert!(!a_current_value_matches(&pair, b"//tex/Metallic.png"));
        assert!(!a_current_value_matches(&pair, b"//other/metallic.png"));
    }

    #[test]
    fn a_pair_is_composed_for_curly_braces_in_a_name_and_a_library_pair_names_only_a_fixed_copy() {
        // A folder honestly named `{hero}` is an ordinary folder, and its
        // reference gets a pair like any other. Only a write-only
        // destination can hold a curly-brace expression (G9), and this
        // module composes no pair for one of those at all.
        let pair = compose_pair(
            Kind::Image,
            "filepath",
            &recorded(b"//{hero}/tex/x.png"),
            mirror(b"/m/x.png"),
        )
        .expect("an ordinary pair");
        assert_eq!(pair.expected.as_bytes(), b"//{hero}/tex/x.png");
        assert_eq!(pair.match_text.as_bytes(), b"//{hero}/tex/x.png");
        assert_eq!(
            compose_pair(
                Kind::Library,
                "filepath",
                &recorded(b"//lib/chairs.blend"),
                mirror(b"/m/chairs.blend")
            ),
            Err(NotComposed::ALibraryPairMustNameAFixedCopy)
        );
        assert_eq!(
            compose_pair(
                Kind::Library,
                "filepath",
                &recorded(b"//lib/chairs.blend"),
                NewLocation::ACopyAlreadyUp(MachineLocation::composed(b"/m/chairs.blend".to_vec()))
            ),
            Err(NotComposed::ALibraryPairMustNameAFixedCopy)
        );
        assert!(compose_pair(
            Kind::Library,
            "filepath",
            &recorded(b"//lib/chairs.blend"),
            NewLocation::AFixedCopy(MachineLocation::composed(b"/f/chairs.blend".to_vec()))
        )
        .is_ok());
        assert_eq!(
            compose_pair(
                Kind::Image,
                "filepath",
                &recorded(b"//x.png"),
                NewLocation::AFixedCopy(MachineLocation::composed(b"/f/x.png".to_vec()))
            ),
            Err(NotComposed::OnlyALibraryPairNamesAFixedCopy)
        );
    }

    #[test]
    fn the_marker_says_applied_and_to_how_many_and_carries_m9ds_field_from_the_kind_alone() {
        let image = compose_pair(
            Kind::Image,
            "filepath",
            &recorded(b"//tex/x.png"),
            mirror(b"/m/x.png"),
        )
        .expect("composed");
        assert_eq!(
            the_outcome_of(
                &image,
                &[
                    b"//tex/x.png".to_vec(),
                    b"//tex\\x.png".to_vec(),
                    b"//other.png".to_vec()
                ]
            ),
            PairOutcome::Applied {
                data_blocks: 2,
                assignment_alone: AssignmentAlone::Established,
                reloaded: false,
            }
        );
        assert_eq!(
            the_outcome_of(&image, &[b"//other.png".to_vec()]),
            PairOutcome::NotApplied
        );
        assert_eq!(the_outcome_of(&image, &[]), PairOutcome::NotApplied);

        let library = compose_pair(
            Kind::Library,
            "filepath",
            &recorded(b"//lib/c.blend"),
            NewLocation::AFixedCopy(MachineLocation::composed(b"/f/c.blend".to_vec())),
        )
        .expect("composed");
        assert_eq!(
            the_outcome_of(&library, &[b"//lib/c.blend".to_vec()]),
            PairOutcome::Applied {
                data_blocks: 1,
                assignment_alone: AssignmentAlone::Established,
                reloaded: true,
            }
        );

        let sound = compose_pair(
            Kind::Sound,
            "filepath",
            &recorded(b"//s.wav"),
            mirror(b"/m/s.wav"),
        )
        .expect("composed");
        assert_eq!(
            the_outcome_of(&sound, &[b"//s.wav".to_vec()]),
            PairOutcome::Applied {
                data_blocks: 1,
                assignment_alone: AssignmentAlone::NotEstablished,
                reloaded: false,
            }
        );
    }

    #[test]
    fn only_the_image_and_the_library_are_established_and_only_the_library_reloads() {
        for kind in THE_FIVE_KINDS_NOT_POSED {
            assert_eq!(
                assignment_alone_is(kind),
                AssignmentAlone::NotEstablished,
                "{kind:?}"
            );
            assert!(!takes_a_reload_after_assignment(kind));
        }
        for kind in THE_SEVEN_INFERRED_KINDS {
            assert_eq!(
                assignment_alone_is(kind),
                AssignmentAlone::NotEstablished,
                "{kind:?}"
            );
        }
        assert_eq!(
            assignment_alone_is(Kind::Image),
            AssignmentAlone::Established
        );
        assert_eq!(
            assignment_alone_is(Kind::Library),
            AssignmentAlone::Established
        );
        assert!(takes_a_reload_after_assignment(Kind::Library));
        assert!(!takes_a_reload_after_assignment(Kind::Image));
    }

    #[test]
    fn the_material_lives_at_remaps_under_the_job_identity_and_reads_back_whole() {
        let job = Identity::issue(IdentityKind::Job).expect("randomness");
        let layout = a_layout();
        let at = RemapMaterial::location_in(&layout, &job);
        assert_eq!(
            at.as_bytes(),
            format!("/ephemeral/smoozy/assets/remaps/{}", job.as_text()).as_bytes()
        );
        let pair = compose_pair(
            Kind::Image,
            "filepath",
            &recorded(b"//tex\\x.png"),
            mirror(b"/m/x.png"),
        )
        .expect("composed");
        let material = RemapMaterial::composed(&job, &a_blender(), vec![pair]);
        assert_eq!(material.composed_for.version, "4.5.12");
        let back = RemapMaterial::read_back(&material.as_written()).expect("reads back");
        assert_eq!(back, material);
        assert_eq!(
            RemapMaterial::manifest_reference(&layout, &job).0["remap_material"],
            serde_json::Value::String(String::from_utf8(at.as_bytes().to_vec()).expect("text"))
        );
    }

    #[test]
    fn a_placed_copy_is_written_over_where_any_of_its_four_values_moved() {
        let placed = PlacedCopy {
            job_folder_path: MachineLocation::composed(
                b"/ephemeral/smoozy/jobs/j/cache/bake.vdb".to_vec(),
            ),
            four_values_at_placement: SourceReading {
                size: 10,
                inode: "5".to_string(),
                modification_time: 100,
                change_time: 100,
            },
        };
        let same = placed.four_values_at_placement.clone();
        assert!(!placed.was_written_over(&same));
        let written = SourceReading {
            change_time: 101,
            ..same
        };
        assert!(placed.was_written_over(&written));
    }

    /// **M9a, held by reading this crate's own sources**: no line of code
    /// names `has_data`. A doc comment may name it to say why not.
    ///
    /// **The walk goes to any depth**, and that is not tidiness: it read one
    /// level until 2026-09-04, when the acting half arrived in a folder of its
    /// own — including the script that runs inside Blender, which is the one
    /// place in this module a reader would most plausibly reach for the flag.
    /// **A check that cannot see a file is a check nothing guards.**
    #[test]
    fn nothing_here_reads_the_flag_m9a_names() {
        fn every_file_under(folder: &std::path::Path, into: &mut Vec<std::path::PathBuf>) {
            for entry in std::fs::read_dir(folder).expect("readable") {
                let path = entry.expect("readable").path();
                if path.is_dir() {
                    every_file_under(&path, into);
                } else {
                    into.push(path);
                }
            }
        }
        // **Both folders of this module**, because the fragment that runs inside
        // Blender moved to `smoozy-dependencies-machine` on 2026-09-19 and a
        // walk of this folder alone would no longer see it.
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let the_machine_half = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../Dependency Upload & Remap — machine/src");
        let mut files = Vec::new();
        every_file_under(&root, &mut files);
        assert!(files.len() > 20, "the walk found almost nothing");
        let here = files.len();
        every_file_under(&the_machine_half, &mut files);
        assert!(
            files.len() - here > 5,
            "the walk found almost nothing in the machine half"
        );
        let mut offenders = Vec::new();
        for path in files {
            let text = std::fs::read_to_string(&path).expect("readable");
            for (number, line) in text.lines().enumerate() {
                let code = line.trim_start();
                if code.starts_with("//") {
                    continue;
                }
                if code.contains(concat!("has", "_data")) && !code.contains("concat!") {
                    offenders.push(format!("{}:{}", path.display(), number + 1));
                }
            }
        }
        assert!(offenders.is_empty(), "{offenders:?}");
    }
}
