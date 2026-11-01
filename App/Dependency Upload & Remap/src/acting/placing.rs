//! **§6.4 and §6.7 performed — M10, M11 and M22c.** The remap material placed
//! before the submission as one ordered unit, and the instruction for the
//! job-folder copy of an uploaded bake.
//!
//! # The ordered unit, and what it is for
//!
//! **M11: the placement and the submission are one ordered unit** — the
//! material is placed first, and where the placement fails **the submission is
//! not attempted**, naming the material as the unit that stopped it (`State,
//! identity and recovery` §9.3). **A job therefore never exists with its remap
//! material missing**, which is what joins `Rendering`'s whole-or-nothing rather
//! than weakening it.
//!
//! **M10's chain is what the unit protects**: manifest → remap reference →
//! remap material on the machine's own disk → launch material composed on the
//! machine. **Nothing in that chain needs a connection**, which is Row 177's
//! requirement met by placement rather than by hope.
//!
//! # The copy, and where each half of it lives
//!
//! **M22c's copy is the machine's and not the desktop's.** It cannot be taken
//! from here: the file-transfer surface has no copy operation, so a copy
//! through the desktop reads a bake down and writes it back up, and M22c's own
//! cost sentence forbids that in as many words — *"the cost is the machine's
//! disk and a local copy, **never a second crossing of the connection**"*.
//!
//! **And a link is not a copy and is refused.** M22a's whole gain over the
//! discarded alternative is that *"because it is a copy, the master is never at
//! risk — a render that recomputes the simulation anyway overwrites the copy,
//! and the user's uploaded original stands untouched"*. A link would let the
//! render write through to the master.
//!
//! **So the instruction lives with the machine's half of this plan and rides
//! the remap material** ([`TheCopyTheMachineOwes`], from
//! `smoozy_dependencies_machine::placed_copies`), **the render worker makes the
//! copy at its step 7 and writes M22f's four values down**, and what is here is
//! the one composition only the desktop can make: which located answers owe a
//! copy, and from which mirror ([`compose_the_copies_owed`]). Step 10c's
//! dependency wave piece 8, 2026-09-29; until then the act waited on the
//! package having a program of its own.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Job files and outputs` D20, L7a. `State,
//! identity and recovery` §9.3. `Download` N5a. `Server protocol and transfers`
//! S22.
//!
//! **`Job files and outputs` M1 is not listed above and is written out at the
//! site**, because this plan defines an M1 of its own — the same care
//! [`crate::arriving_later`] already takes with `Rented-server runtime` A4.

use smoozy_job_files::layout::TheLayout;
use smoozy_server_protocol::transfers::{Direction, TransferRecord};
use smoozy_state::identities::Identity;

use crate::bytes::RecordedLocation;
use crate::landing::fits_the_budgets;
use crate::record::HoldingFile;
use crate::remap::{AnAnswerToCompose, NoPairFor, RemapMaterial};

/// **M22c's instruction and the kind-to-place rule, at the machine's half of
/// this plan**, where the render worker reads them. Moved there on 2026-09-29.
pub use smoozy_dependencies_machine::placed_copies::{
    the_write_place_a_kind_is, TheCopyTheMachineOwes,
};

/// **M22f's list as the machine writes it, its two file names, and its one
/// entry** — read back on this side by `Download` N5a's ending walk and by a
/// whole job's manual download. Re-exported 2026-10-02, the fixes before
/// 0.1.8, fault 3: until then nothing on the desktop read either file.
pub use smoozy_dependencies_machine::placed_copies::{
    the_placed_copies_after_the_render_file_name, the_placed_copies_file_name, APlacedFile,
    ThePlacedCopies,
};

/// **M11's unit, and its two endings.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheOrderedUnit {
    /// The material published. The submission may be attempted.
    TheMaterialIsPlacedAndTheSubmissionMayGoAhead,
    /// **The submission is not attempted**, and the unit that stopped it is
    /// named. It is never *the submission failed*, because the submission was
    /// never made.
    TheSubmissionIsNotAttempted {
        the_unit_that_stopped_it: &'static str,
    },
}

/// The name M11 gives the unit that stops the submission. **A constant so the
/// name a reader meets is the one the clause uses**, and so that nothing
/// composes a sentence at the moment of failure.
pub const THE_UNIT_THAT_STOPS_IT: &str = "the job's remap material";

/// **M11, as a judgement over what the placement came to.**
///
/// **Only a publication lets the submission go ahead.** Every other ending of
/// the transfer contract — stopped, failed, stalled, interrupted — leaves the
/// material unplaced, and `Server protocol and transfers` 7.1's own rule is
/// that part-arrived bytes never sit at a final name. **There is no ending here
/// that is judged by reading the machine.**
pub fn the_ordered_unit(the_placement_published: bool) -> TheOrderedUnit {
    if the_placement_published {
        TheOrderedUnit::TheMaterialIsPlacedAndTheSubmissionMayGoAhead
    } else {
        TheOrderedUnit::TheSubmissionIsNotAttempted {
            the_unit_that_stopped_it: THE_UNIT_THAT_STOPS_IT,
        }
    }
}

/// **M10: the material's placement, arranged under the transfer contract.**
///
/// **Staged, integrity-checked, published by one rename** — every one of those
/// is `Server protocol and transfers` §7's loop, and this composes the record it
/// drives. **The class it rides is the submission's own**, whatever `Rendering`
/// declares it to be, *"because it is a step of submission rather than of this
/// module's uploading"* (A9) — so no class of this module's is named here.
///
/// **The material is immutable** and its location is a function of the job
/// identity alone, so a second placement for one job composes the same path and
/// the contract's own publication rules govern it.
pub fn arrange_the_materials_placement(
    layout: &TheLayout,
    material: &RemapMaterial,
    job: &Identity,
    transfer: Identity,
    server_row: Identity,
    installation: Identity,
    run: Identity,
    the_source_the_desktop_wrote_it_to: &str,
    staging_directory: &str,
) -> Result<TransferRecord, WhyTheMaterialCannotBePlaced> {
    let at = RemapMaterial::location_in(layout, job);
    fits_the_budgets(at.as_bytes())
        .map_err(|_| WhyTheMaterialCannotBePlaced::TheAddressWillNotFit)?;
    let final_path = std::str::from_utf8(at.as_bytes())
        .map_err(|_| WhyTheMaterialCannotBePlaced::TheAddressWillNotFit)?
        .to_string();
    let bytes = material.as_written();
    Ok(TransferRecord::arranged(
        transfer,
        Direction::Up,
        server_row,
        Some(installation),
        the_source_the_desktop_wrote_it_to.to_string(),
        final_path,
        staging_directory,
        bytes.len() as u64,
        run,
    ))
}

/// Why a material cannot be placed at all.
///
/// **One reason, because there is one thing that can be wrong before a transfer
/// exists**: the address, which is composed from the asset area and the job
/// identity's textual form and is checked in bytes like every other path this
/// module writes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyTheMaterialCannotBePlaced {
    TheAddressWillNotFit,
}

/// **M22c composed at submission: the copies the machine owes, and the reason
/// any answer of D20's four write places owes none.**
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TheComposedCopies {
    pub copies: Vec<TheCopyTheMachineOwes>,
    pub without_a_copy: Vec<(RecordedLocation, NoPairFor)>,
}

/// **M22a and M22c: every located answer of one of D20's four write places —
/// the kinds the pairs refuse with [`NoPairFor::TheRedirectAssignsThisSetting`]
/// — owes a copy of the bake it was resolved to, from that bake's mirror.**
///
/// **The same refusals as the pairs', for the same reasons** (M4): a reference
/// below the top file, a row set aside, a row not found, a mirror that cannot
/// be composed. **Every other kind is the pairs' and is passed over here**, so
/// the two compositions together cover each answer once. Where the user was
/// pointed at a copy already on the machine (Q9), that copy is the source.
pub fn compose_the_copies_owed(
    layout: &TheLayout,
    the_top_file: &HoldingFile,
    answers: &[AnAnswerToCompose],
) -> TheComposedCopies {
    let mut composed = TheComposedCopies::default();
    for answer in answers {
        let Some(place) = the_write_place_a_kind_is(answer.kind) else {
            continue;
        };
        let reference = answer.answer.key.reference.clone();
        match one_copy(layout, the_top_file, answer, place) {
            Ok(copy) => composed.copies.push(copy),
            Err(why) => composed.without_a_copy.push((reference, why)),
        }
    }
    composed
}

fn one_copy(
    layout: &TheLayout,
    the_top_file: &HoldingFile,
    answer: &AnAnswerToCompose,
    place: smoozy_job_files::reading::AnAddedWritePlace,
) -> Result<TheCopyTheMachineOwes, NoPairFor> {
    if &answer.answer.key.holding_file != the_top_file {
        return Err(NoPairFor::ItIsNotTheTopFilesOwnReference);
    }
    if answer.answer.set_aside {
        return Err(NoPairFor::TheRowWasSetAside);
    }
    let resolution = answer
        .answer
        .resolution
        .as_ref()
        .ok_or(NoPairFor::NothingWasResolved)?;
    let from = match &answer.already_on_the_machine {
        Some(at) => at.clone(),
        None => crate::landing::mirror_of(layout, resolution.to.location())
            .map_err(NoPairFor::TheMirrorCouldNotBeComposed)?,
    };
    Ok(TheCopyTheMachineOwes {
        from,
        place,
        recorded: answer.answer.key.reference.clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bytes::MachineLocation;
    use crate::check::Kind;
    use crate::remap::{compose_pair, NewLocation};
    use smoozy_job_files::layout::{MachineRoot, WhereThePackageAreaSits};
    use smoozy_render_settings::file_half::BlenderIdentity;
    use smoozy_server_setup::blender::Version;
    use smoozy_state::identities::IdentityKind;

    fn a_layout() -> TheLayout {
        TheLayout::of(
            MachineRoot::chosen("/ephemeral/smoozy").expect("a root"),
            WhereThePackageAreaSits::UnderTheRoot,
        )
    }

    fn an_identity(kind: IdentityKind) -> Identity {
        Identity::issue(kind).expect("this machine supplies random bytes")
    }

    fn a_material(job: &Identity) -> RemapMaterial {
        let pair = compose_pair(
            Kind::Image,
            "filepath",
            &RecordedLocation::as_the_reading_carried_it(b"//tex/x.png"),
            NewLocation::AMirror(MachineLocation::composed(
                b"/ephemeral/smoozy/assets/files/Users/a/tex/x.png".to_vec(),
            )),
        )
        .expect("the pair composes");
        RemapMaterial::composed(
            job,
            &BlenderIdentity {
                path: "/opt/blender/blender".to_string(),
                version: Version {
                    major: 5,
                    minor: 2,
                    patch: 0,
                },
            },
            vec![pair],
        )
    }

    /// **M11: a placement that did not publish stops the submission and names
    /// the material.**
    ///
    /// **This is the test that keeps a job from existing with its remap
    /// material missing** — the one shape where a re-run with nobody connected
    /// composes its launch material without the pairs and renders the artist's
    /// own recorded locations, at success, with nothing pointing where it
    /// should.
    #[test]
    fn a_placement_that_did_not_publish_stops_the_submission_and_names_the_material() {
        assert_eq!(
            the_ordered_unit(false),
            TheOrderedUnit::TheSubmissionIsNotAttempted {
                the_unit_that_stopped_it: THE_UNIT_THAT_STOPS_IT,
            }
        );
        assert_eq!(
            the_ordered_unit(true),
            TheOrderedUnit::TheMaterialIsPlacedAndTheSubmissionMayGoAhead
        );
        const {
            assert!(crate::remap::THE_MATERIAL_IS_PLACED_FIRST_AND_A_FAILED_PLACEMENT_STOPS_THE_SUBMISSION)
        };
    }

    /// **M10: the material goes to `remaps/<job identity's textual form>` in the
    /// asset area, as an upload under the contract.**
    #[test]
    fn the_material_is_placed_at_its_own_address_under_the_contract() {
        let layout = a_layout();
        let job = an_identity(IdentityKind::Job);
        let material = a_material(&job);
        let record = arrange_the_materials_placement(
            &layout,
            &material,
            &job,
            an_identity(IdentityKind::Transfer),
            an_identity(IdentityKind::LocalServer),
            an_identity(IdentityKind::Installation),
            an_identity(IdentityKind::Run),
            "/Users/a/Library/Application Support/Smoozy/remap-material",
            "/ephemeral/smoozy/assets/.smoozy-staging",
        )
        .expect("it arranges");
        assert_eq!(record.direction, Direction::Up);
        assert_eq!(
            record.final_path,
            String::from_utf8_lossy(RemapMaterial::location_in(&layout, &job).as_bytes())
        );
        assert!(record.final_path.contains("/remaps/"));
        assert!(record.final_path.ends_with(&job.as_text()));
        assert_eq!(
            record.total_size_expected,
            material.as_written().len() as u64
        );
        assert_eq!(record.ending, None);
    }

    fn stored(bytes: &[u8]) -> crate::bytes::StoredLocation {
        crate::bytes::StoredLocation::as_the_system_returned_it(bytes)
    }

    fn the_top_file() -> HoldingFile {
        HoldingFile::OnTheUsersComputer(stored(b"/Users/a/proj/scene.blend"))
    }

    fn an_answer(kind: Kind, reference: &[u8], resolved: Option<&[u8]>) -> AnAnswerToCompose {
        AnAnswerToCompose {
            answer: crate::record::LocatedAnswer {
                key: crate::record::LocatedAnswerKey {
                    holding_file: the_top_file(),
                    reference: RecordedLocation::as_the_reading_carried_it(reference),
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
            setting: "render.bake.filepath".to_string(),
            already_on_the_machine: None,
        }
    }

    /// **M22a and M22c: an answer of one of D20's four write places owes a
    /// copy from its mirror, keyed to the place and to what it recorded** —
    /// and the pairs' own refusals hold for it, each with its reason.
    #[test]
    fn a_bake_the_check_found_owes_a_copy_from_its_mirror_and_nothing_else_does() {
        let layout = a_layout();
        let bake = an_answer(Kind::BakeFolder, b"//bake/", Some(b"/Users/a/proj/bake"));
        let image = an_answer(Kind::Image, b"//tex/x.png", Some(b"/Users/a/proj/tex/x.png"));
        let not_found = an_answer(Kind::PhysicsPointCache, b"//cache/", None);
        let mut set_aside = an_answer(
            Kind::GeometryNodeBakeFolder,
            b"//gn/",
            Some(b"/Users/a/proj/gn"),
        );
        set_aside.answer.set_aside = true;
        let mut up_already = an_answer(Kind::ProxyFolder, b"//proxy/", Some(b"/Users/a/proj/proxy"));
        up_already.already_on_the_machine = Some(MachineLocation::composed(
            b"/ephemeral/smoozy/assets/files/elsewhere/proxy".to_vec(),
        ));

        let composed = compose_the_copies_owed(
            &layout,
            &the_top_file(),
            &[bake, image, not_found, set_aside, up_already],
        );
        assert_eq!(
            composed.copies,
            vec![
                TheCopyTheMachineOwes {
                    from: crate::landing::mirror_of(&layout, &stored(b"/Users/a/proj/bake"))
                        .expect("a mirror"),
                    place: smoozy_job_files::reading::AnAddedWritePlace::TheBakeOutputPath,
                    recorded: RecordedLocation::as_the_reading_carried_it(b"//bake/"),
                },
                TheCopyTheMachineOwes {
                    from: MachineLocation::composed(
                        b"/ephemeral/smoozy/assets/files/elsewhere/proxy".to_vec()
                    ),
                    place: smoozy_job_files::reading::AnAddedWritePlace::TheSequencerProxyFolder,
                    recorded: RecordedLocation::as_the_reading_carried_it(b"//proxy/"),
                },
            ]
        );
        assert_eq!(
            composed.without_a_copy,
            vec![
                (
                    RecordedLocation::as_the_reading_carried_it(b"//cache/"),
                    NoPairFor::NothingWasResolved
                ),
                (
                    RecordedLocation::as_the_reading_carried_it(b"//gn/"),
                    NoPairFor::TheRowWasSetAside
                ),
            ],
            "the image is the pairs' and is passed over here"
        );
    }

    /// **The four kinds that owe a copy are exactly the four the pairs refuse
    /// for the redirect** (M22a), so the two compositions cover each answer
    /// once.
    #[test]
    fn the_kinds_that_owe_a_copy_are_the_kinds_the_pairs_leave_to_the_redirect() {
        for kind in Kind::ALL {
            assert_eq!(
                the_write_place_a_kind_is(kind).is_some(),
                crate::remap::THE_WRITE_PLACES_THE_REDIRECT_ASSIGNS.contains(&kind),
                "{kind:?}"
            );
        }
    }

    /// **M22c: it is a copy and never a link, and it rides the material** —
    /// a material carrying one reads back carrying it.
    #[test]
    fn the_copy_rides_the_remap_material_and_reads_back_with_it() {
        const { assert!(crate::remap::THE_JOB_FOLDER_COPY_IS_MADE_ON_THE_MACHINE_FROM_THE_MIRROR) };
        const {
            assert!(crate::remap::THE_SETTING_TAKES_THE_JOB_FOLDER_VALUE_AND_THE_UPLOADED_BAKE_IS_PLACED_THERE)
        };
        let job = an_identity(IdentityKind::Job);
        let copy = TheCopyTheMachineOwes::where_a_bake_was_sent(
            Some(&MachineLocation::composed(b"/ephemeral/smoozy/assets/files/c".to_vec())),
            smoozy_job_files::reading::AnAddedWritePlace::ThePointCache,
            &RecordedLocation::as_the_reading_carried_it(b"//cache/"),
        )
        .expect("a bake was sent");
        let material = a_material(&job).with_the_copies_owed(vec![copy.clone()]);
        let read = RemapMaterial::read_back(&material.as_written()).expect("it reads");
        assert_eq!(read.copies_owed, vec![copy]);
        let bare = a_material(&job);
        assert!(
            !String::from_utf8_lossy(&bare.as_written()).contains("copies_owed"),
            "a material with no bake is written as it always was"
        );
    }
}
