//! The asset area and the landing — `Dependency Upload & Remap` §5, Rows 169
//! and 170: the compositions and the judgements. **The transfers, the
//! directory creation and the handed unit's three acts are the acting
//! half's** ([`crate::arriving_later`]).
//!
//! **A1: the asset area is the sibling area `Job files and outputs` 1.2 L6
//! assigns, under the one root, outside every job folder, its contents this
//! document's.** It is read off that crate's own layout and never composed a
//! second time here.
//!
//! **A6: the assumption the rule rests on — two files on one computer cannot
//! sit at the same whole location — is weighed and the promise does not fall
//! with it.** Every way it is stressed in practice — the same location holding
//! a different file at a later time, or a second computer of the user's
//! carrying a different file at the same location — lands on a mirror
//! location that is already occupied, and Q5's comparison reads the
//! difference and Q8's warning fires before the write.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Job files and outputs` L6, L7, L8, N5, N6, N9,
//! N13. `Server protocol and transfers` S12, S22, S26, T3, T5, T6, T8, C1.
//! `File Browser` U12, W1a, W1b, X3. `Server Setup` R7b.
//!
//! **`File Browser` U15 is not listed above and is written out at the site**,
//! because this plan defines a U1–U4 series and a bare one would read as one
//! of this plan's.

use smoozy_job_files::layout::TheLayout;
use smoozy_job_files::names::{WhyThePathIsRefused, NAME_BUDGET_BYTES, PATH_BUDGET_BYTES};
use smoozy_server_protocol::transfers::Checkpoint;
use smoozy_state::identities::Identity;

use crate::bytes::{segments, MachineLocation, StoredLocation, SEPARATOR};
use crate::readings::Readings;

/// **The three fixed segments and the asset area's own directory, at the
/// paths they have always been reached at.** They live in
/// `smoozy-dependencies-machine`, because the remap material is kept under
/// `remaps` and the rented machine's own program reads it there. Split at step
/// 10c piece 3.
pub use smoozy_dependencies_machine::landing::{the_asset_area, FILES, FIXED, REMAPS};

/// **A5's two byte checks, by Smoozy's own arithmetic before any byte is
/// sent** — every single segment against 255 and the whole path against
/// 4095, **in bytes and never characters** (`Job files and outputs` N5, whose
/// constants these are; its N6 says the machine's refusal reads `Bad message`
/// and names nothing). **Both can genuinely fail**: macOS accepts a name of 255
/// UTF-16 code units, which reached 765 bytes in the one measured case (Fact
/// 24), so a legal Mac name can exceed the machine's name budget.
pub fn fits_the_budgets(path: &[u8]) -> Result<(), WhyThePathIsRefused> {
    if path.is_empty() {
        return Err(WhyThePathIsRefused::ItIsEmpty);
    }
    if path.len() > PATH_BUDGET_BYTES {
        return Err(WhyThePathIsRefused::TheWholePathIsOverTheBudget {
            it_is_this_many_bytes: path.len(),
        });
    }
    for segment in segments(path) {
        if segment.len() > NAME_BUDGET_BYTES {
            return Err(WhyThePathIsRefused::ASingleNameIsOverTheBudget {
                it_is_this_many_bytes: segment.len(),
            });
        }
    }
    Ok(())
}

/// **A5: a row whose mirrored location cannot fit is *could not be sent*, with
/// the reason named, before any transfer is arranged.** Smoozy never
/// truncates, folds or renames to make one fit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CouldNotBeSent {
    TheMirrorWillNotFit(WhyThePathIsRefused),
    /// **A4: a location from a Windows computer is permanently unresolved
    /// with the rest of Windows** (§12 item 3), and **a location pointing at
    /// another computer is permanently unresolved** (§12 item 14): the
    /// landing rule does not cover a location that is not an absolute
    /// location from the top of the user's disk, and nothing here pretends
    /// it does. No space is left for the answer.
    TheLocationsFormIsNotOneTheLandingRuleCovers,
}

/// **A4: an asset lands at the mirrored copy of its own whole location — the
/// `files` segment joined to the whole location the file came from on the
/// user's computer, from the top of the user's disk downward, as stored on
/// disk (F12), byte for byte.** Creator-stated (2026-07-28) in the spec; what
/// this document adds is the spelling — the stored name, not the reference's
/// — and the prefix. On macOS the location is as the system writes it,
/// `/Volumes/<name>` included.
pub fn mirror_of(
    layout: &TheLayout,
    source: &StoredLocation,
) -> Result<MachineLocation, CouldNotBeSent> {
    if !source.as_bytes().starts_with(&[SEPARATOR]) {
        return Err(CouldNotBeSent::TheLocationsFormIsNotOneTheLandingRuleCovers);
    }
    let mut path = format!("{}/{FILES}", the_asset_area(layout)).into_bytes();
    path.extend_from_slice(source.as_bytes());
    fits_the_budgets(&path).map_err(CouldNotBeSent::TheMirrorWillNotFit)?;
    Ok(MachineLocation::composed(path))
}

/// **A3: why a fixed copy's address could not be composed.** The row fails as
/// *could not be fixed* with the reason named, on the same terms A5 fails a
/// mirror that cannot fit; nothing is truncated, folded or renamed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CouldNotBeFixed {
    /// A fixed copy's address carries two segments more than the mirror
    /// does, so a linked `.blend` whose mirror fits can have a fixed copy
    /// that does not.
    TheAddressWillNotFit(WhyThePathIsRefused),
    /// Every machine path this module touches sits under the root (`Job
    /// files and outputs` L6); one that does not is a fault rather than a
    /// case.
    TheMachineCopyIsNotUnderTheRoot,
}

/// **A3: a fixed copy is addressed off the machine location of the linked
/// `.blend` it was made from, and never off a mirror location** — the `fixed`
/// segment joined to that file's own machine path taken from the root
/// downward. For a mirrored `.blend` that reads
/// `fixed/<the asset area's own segment>/files/<location>`; for one on the
/// machine by any other route — uploaded by hand, pointed at by Q9, or there
/// with no known local origin (K5) — the same join off its own machine path
/// gives it an address too. **Unique by construction**, since two files cannot
/// share one machine path: one fixed copy per linked `.blend` on the machine,
/// shared by every project and job that links it.
pub fn fixed_copy_address(
    layout: &TheLayout,
    machine_copy: &MachineLocation,
) -> Result<MachineLocation, CouldNotBeFixed> {
    let root = layout.root().as_path().as_bytes();
    let bytes = machine_copy.as_bytes();
    let remainder = bytes
        .strip_prefix(root)
        .and_then(|rest| rest.strip_prefix(&[SEPARATOR]))
        .ok_or(CouldNotBeFixed::TheMachineCopyIsNotUnderTheRoot)?;
    let mut path = format!("{}/{FIXED}/", the_asset_area(layout)).into_bytes();
    path.extend_from_slice(remainder);
    fits_the_budgets(&path).map_err(CouldNotBeFixed::TheAddressWillNotFit)?;
    Ok(MachineLocation::composed(path))
}

/// **A6 as a value**: the same-whole-location assumption is weighed, and the
/// contents comparison catches the collision before it becomes a silent
/// replacement (Q5, Q8).
pub const WHERE_THE_ASSUMPTION_BENDS_THE_CONTENTS_COMPARISON_CATCHES_IT: bool = true;

/// **A7: mirror directories are created with the plain create-directory form,
/// segment by segment, and an already-standing segment is the ordinary case
/// rather than a fault** — never the make-parents form (`Job files and
/// outputs` N9). A refusal of the plain form is resolved by one follow-up
/// attribute read: where the path now exists as a directory the landing
/// proceeds; anything else fails the row as *could not be sent*. The act is
/// the acting half's; the rule is here.
pub const A_MIRROR_DIRECTORY_IS_CREATED_SEGMENT_BY_SEGMENT_WITH_THE_PLAIN_FORM: bool = true;

/// **A8: every machine-side path this module touches travels as protocol
/// fields, never through a shell line.** Mirrored user paths carry spaces,
/// newlines and quotes as a matter of course.
pub const EVERY_MACHINE_PATH_TRAVELS_AS_PROTOCOL_FIELDS: bool = true;

/// **A9: this module's transfers are declared as one priority class — the
/// dependency-upload class — whose rank among the other classes Rows 189 and
/// 207 order in `File Browser` and `Download`.** `Server protocol and
/// transfers` T8 requires the class declared by the module that arranged the
/// transfer, in its own document; this is that declaration. Every asset
/// upload is a transfer under its §7, arranged per file, bound to its server
/// (T3), admitted through the per-server point (T5) inside the measured
/// two-to-four range (T6).
pub const THE_TRANSFER_PRIORITY_CLASS: &str = "dependency-upload";

/// **A9's second half: the remap-material placement of 6.4 rides the
/// submission's own class**, whatever `Rendering` declares it to be, because
/// it is a step of submission rather than of this module's uploading.
pub const THE_REMAP_MATERIAL_RIDES_THE_SUBMISSIONS_OWN_CLASS: bool = true;

/// **A10's verdict on a resume of one of this module's uploads.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResumeVerdict {
    /// The staged prefix is the current source's own prefix, so carrying on
    /// writes exactly the current source's bytes.
    Proceeds { from: u64 },
    /// The transfer starts over from zero into a fresh staging path under
    /// `Server protocol and transfers` S12 step 5, and the old staging is
    /// abandoned under that document's cleanup.
    StartsOverFromZero,
}

/// **A10: a contents test supplied for Row 201 to adopt or to overrule, and
/// not an answer this document owns.** In order: the staged size is read and
/// the highest checkpoint at or below it taken (S12); the source file on the
/// user's computer is read up to that checkpoint and digested; where that
/// digest equals the checkpoint's recorded running digest, resume proceeds;
/// where it does not, the transfer starts over. **A contents answer rather
/// than a timestamp answer, so it needs no fact about the user's filesystems
/// that nobody has established.** What it does not claim: that the source
/// cannot change *while* being read — `File Browser`'s Row 201.
pub fn a_resume_of_this_modules_upload(
    staged_size: u64,
    checkpoints: &[Checkpoint],
    source: &StoredLocation,
    readings: &dyn Readings,
) -> ResumeVerdict {
    let Some(checkpoint) = checkpoints
        .iter()
        .filter(|checkpoint| checkpoint.offset <= staged_size)
        .max_by_key(|checkpoint| checkpoint.offset)
    else {
        return ResumeVerdict::StartsOverFromZero;
    };
    match readings.the_sources_prefix_digest(source, checkpoint.offset) {
        Some(digest) if digest == checkpoint.digest_of_everything_up_to_it => {
            ResumeVerdict::Proceeds {
                from: checkpoint.offset,
            }
        }
        _ => ResumeVerdict::StartsOverFromZero,
    }
}

/// **A10 is offered, not owned**: where Row 201 settles the question
/// otherwise, this module follows Row 201 and this clause goes.
pub const WHERE_ROW_201_SETTLES_IT_OTHERWISE_THIS_MODULE_FOLLOWS_ROW_201: bool = true;

/// **A11: a row is tried again on its own, and a retry is a new transfer** —
/// only what did not publish, under a new transfer identity, and a retry that
/// fails again leaves the row as it was; the row's state comes from
/// publications and the record, so it can never be quietly turned into a
/// success. **A12: a lost connection behaves exactly as the contract makes it
/// behave everywhere**, and running the check again sends only what is still
/// missing — the resume passing through A10 where a partial exists.
pub const A_RETRY_IS_A_NEW_TRANSFER_AND_RESENDS_ONLY_WHAT_DID_NOT_PUBLISH: bool = true;

/// **A13: *once and stay up* rests on three things, and the third is
/// permanently unresolved.** First, the placement on the mount chosen once.
/// Second, the record: a published file's placement record (K7) is what says
/// it is up, so nothing ever silently *believes* an asset survived — a
/// vanished or corrupted asset is found by the four values moving or the
/// read-through differing, and becomes a row to send again rather than a
/// wrong render. Third, survival across the machine's own unattended
/// housekeeping over days is **not established and never will be** (§12 item
/// 4) — and it is not asserted. **What holds**: an asset the machine removes
/// costs a re-send the next time a check looks, never a silent reuse of
/// nothing.
pub const SURVIVAL_ACROSS_THE_MACHINES_OWN_HOUSEKEEPING_IS_NOT_ASSERTED: bool = true;

/// **A14: a part-arrived file cannot become *already on the machine*, cannot
/// become a placement record, and cannot survive a retry as a hidden prefix
/// of something else.** Four defences, listed together because Fact 18 is the
/// single nastiest observation this module inherits: the contract (part-
/// arrived bytes never sit at a final name), Q11 (no record without a
/// publication), A10 (a stale staged prefix verified by contents), and Q1 (no
/// contents equality satisfied by a partial).
pub const THE_FOUR_DEFENCES_AGAINST_A_PARTIAL: [&str; 4] = [
    "the contract: part-arrived bytes never sit at a final name",
    "Q11: publication is the only event that creates a placement record",
    "A10: a stale staged prefix is verified by contents before a byte lands on it",
    "Q1: no contents equality can be satisfied by a partial",
];

/// **`File Browser` W1a's three-way class for a machine path, as this module's
/// records answer it** — read off K7's placement records and K8's fixing
/// records and off nothing that document keeps. This is the type that fills
/// the class slot of that document's `WhatCrosses` (`File Browser` U15).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlacedOrFixed {
    /// A K7 placement record names the path.
    Placed,
    /// A K8 fixing record's fixed copy sits at the path.
    Fixed,
}

/// **A15: what arrives from `File Browser`'s hand-over is four fields per
/// unit and nothing else** — the file's source path on the user's own
/// computer as stored on disk (K3's holding-file key and K7's source
/// location); the server row and the installation identity the upload
/// request was arranged under; the landing path its walk found occupied; and
/// its W1a class for that path. **Nothing of that request's own state
/// crosses and nothing here reads any.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandedUnit {
    pub source_path: StoredLocation,
    pub server_row: Identity,
    pub installation: Option<Identity>,
    pub occupied_landing_path: MachineLocation,
    pub class: PlacedOrFixed,
}

/// **A16: the one case in which no act opens is a superseded installation
/// identity.** The crossed identity is compared against the server row's
/// current one, and a disqualified unit opens nothing and writes nothing of
/// this module's — `File Browser` U12's own *a hand-over that does not open
/// leaves the entries as the collision answer left them* governs it. Nothing
/// else refuses: a unit whose mirror cannot fit A5's two byte checks opens its
/// act and fails as *could not be sent* with the reason named.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoesNotOpen {
    ASupersededInstallationIdentity,
}

pub fn opens(
    unit: &HandedUnit,
    the_rows_current_installation: &Identity,
) -> Result<(), DoesNotOpen> {
    match &unit.installation {
        Some(crossed) if crossed == the_rows_current_installation => Ok(()),
        _ => Err(DoesNotOpen::ASupersededInstallationIdentity),
    }
}

/// **A16: a hand-over of N units is N independent acts, one per unit, on
/// F22's terms; the operation identity the caller is given is that act's
/// transfer request's, chosen before the request is sent (`Server protocol
/// and transfers` C1) and returned at the moment the act opens.**
pub const ONE_ACT_PER_UNIT_AND_ITS_IDENTITY_IS_THE_TRANSFER_REQUESTS: bool = true;

/// **A17 and A18: the act is a send under this module's own landing rule,
/// then the reading, then the fixing — three acts this document already
/// describes and no fourth — and the occupied landing path is what the offer
/// was made about, never the act's destination.** A4 is this module's one
/// landing rule and the act takes no second one: the destination is the
/// mirror of the crossed source path, whatever the walk found occupied.
pub fn the_destination_of(
    layout: &TheLayout,
    unit: &HandedUnit,
) -> Result<MachineLocation, CouldNotBeSent> {
    mirror_of(layout, &unit.source_path)
}

/// **A17: a handed unit is not a check row and creates none, and no check
/// standing is written for it** — a standing belongs to a root the user asked
/// to check (V9, V10), and nobody asked for one here.
pub const A_HANDED_UNIT_IS_NOT_A_ROW_AND_WRITES_NO_STANDING: bool = true;

/// **A18's class-*fixed* case**: a fixed copy's address begins at the `fixed`
/// segment and no transfer of this module's ever publishes to one — a fixed
/// copy is only ever produced by M14 — so the two never coincide, and an
/// occupied fixed copy is replaced by step 3's rename.
pub fn no_transfer_of_this_modules_publishes_to(layout: &TheLayout, at: &MachineLocation) -> bool {
    let fixed = format!("{}/{FIXED}/", the_asset_area(layout)).into_bytes();
    at.as_bytes().starts_with(&fixed)
}

/// **A9a: what one row sends, as the reading of what is already on its way
/// compares it** — read off the row's answer, and never off a listing of the
/// machine.
#[derive(Debug, Clone, Copy)]
pub enum WhatARowSends<'a> {
    /// **One file at this location, or every file under it** where the
    /// location is a folder a folder-shaped row was answered with (F28).
    TheLocation(&'a StoredLocation),
    /// **A set row's members** (G22): the files directly in this folder whose
    /// names match the set's pattern.
    TheSetIn {
        folder: &'a StoredLocation,
        pattern: &'a [u8],
    },
}

/// **A9a: whether this file is already on its way to this server** — a
/// running or waiting transfer of the dependency-upload class (A9) carries it
/// from the same source location to the same landing place.
///
/// `on_its_way` is the landing place of every file such a transfer carries to
/// this server, read from the desktop's transfer store at the moment of
/// asking. **A transfer that has ended — done, failed or stopped — is not in
/// it**, so its file is free to go again. **A landing place names its whole
/// source location** (A4: the `files` segment joined to it, byte for byte), so
/// comparing it with the mirror this source has under this server's layout is
/// the comparison on both keys at once. Nothing new is stored: the per-file
/// transfers are the record.
pub fn is_on_its_way(
    layout: &TheLayout,
    source: &StoredLocation,
    on_its_way: &[MachineLocation],
) -> bool {
    mirror_of(layout, source).is_ok_and(|mirror| on_its_way.contains(&mirror))
}

/// **A9a: whether any file a row sends is already on its way to this server**
/// — the row the panel draws as on its way, and the row the upload act leaves
/// out at the press. **A row is on its way where any one of its files is**,
/// so the button's count and what the press sends are the same rows.
pub fn a_row_is_on_its_way(
    layout: &TheLayout,
    row: WhatARowSends<'_>,
    on_its_way: &[MachineLocation],
) -> bool {
    let location = match row {
        WhatARowSends::TheLocation(location) => location,
        WhatARowSends::TheSetIn { folder, .. } => folder,
    };
    let Ok(mirror) = mirror_of(layout, location) else {
        return false;
    };
    let whole = mirror.as_bytes();
    let folder = crate::bytes::without_trailing_separators(whole);
    on_its_way.iter().any(|landing| {
        let landing = landing.as_bytes();
        match row {
            WhatARowSends::TheLocation(_) => {
                landing == whole
                    || (landing.len() > folder.len()
                        && landing.starts_with(folder)
                        && landing[folder.len()] == SEPARATOR)
            }
            WhatARowSends::TheSetIn { pattern, .. } => {
                crate::bytes::folder_of(landing) == folder
                    && crate::check::set_member_matches(
                        pattern,
                        crate::bytes::last_segment(landing),
                    )
            }
        }
    })
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::contents::ContentIdentity;
    use crate::readings::{
        EnumerationCasing, FolderListing, FourValuesNow, MachineContents, StoredNameAnswer,
    };
    use crate::record::MachineKey;
    use smoozy_job_files::layout::{MachineRoot, WhereThePackageAreaSits};
    use smoozy_server_protocol::transfers::integrity::Digest;
    use smoozy_state::identities::IdentityKind;

    pub fn a_layout() -> TheLayout {
        TheLayout::of(
            MachineRoot::chosen("/ephemeral/smoozy").expect("a root"),
            WhereThePackageAreaSits::UnderTheRoot,
        )
    }

    fn stored(bytes: &[u8]) -> StoredLocation {
        StoredLocation::as_the_system_returned_it(bytes)
    }

    #[test]
    fn the_mirror_is_the_files_segment_joined_to_the_whole_stored_location() {
        let mirror =
            mirror_of(&a_layout(), &stored(b"/Users/alice/proj/tex/METallic.png")).expect("fits");
        assert_eq!(
            mirror.as_bytes(),
            b"/ephemeral/smoozy/assets/files/Users/alice/proj/tex/METallic.png"
        );
        let on_a_volume =
            mirror_of(&a_layout(), &stored("/Volumes/Tex Drive/x.png".as_bytes())).expect("fits");
        assert_eq!(
            on_a_volume.as_bytes(),
            "/ephemeral/smoozy/assets/files/Volumes/Tex Drive/x.png".as_bytes()
        );
    }

    #[test]
    fn a_location_the_landing_rule_does_not_cover_is_refused_with_the_reason_named() {
        assert_eq!(
            mirror_of(&a_layout(), &stored(b"C:\\Users\\alice\\x.png")),
            Err(CouldNotBeSent::TheLocationsFormIsNotOneTheLandingRuleCovers)
        );
    }

    #[test]
    fn the_budgets_are_bytes_and_a_legal_mac_name_can_exceed_them() {
        // Two hundred three-byte characters: 204 characters with the
        // extension, well inside a character count, and 604 bytes — a legal
        // Mac name that the machine's name budget refuses (Fact 24's shape).
        let name: String = "\u{4e2d}".repeat(200);
        let mut source = b"/Users/a/".to_vec();
        source.extend_from_slice(name.as_bytes());
        source.extend_from_slice(b".png");
        match mirror_of(&a_layout(), &stored(&source)) {
            Err(CouldNotBeSent::TheMirrorWillNotFit(
                WhyThePathIsRefused::ASingleNameIsOverTheBudget {
                    it_is_this_many_bytes,
                },
            )) => assert_eq!(it_is_this_many_bytes, 604),
            other => panic!("{other:?}"),
        }
        let long: Vec<u8> = std::iter::repeat_n(b"/abcdefghij".as_slice(), 400)
            .flatten()
            .copied()
            .collect();
        assert!(matches!(
            mirror_of(&a_layout(), &stored(&long)),
            Err(CouldNotBeSent::TheMirrorWillNotFit(
                WhyThePathIsRefused::TheWholePathIsOverTheBudget { .. }
            ))
        ));
    }

    #[test]
    fn a_fixed_copy_is_addressed_off_the_machine_path_and_the_fixed_tree_repeats_the_mirror_tree() {
        let layout = a_layout();
        let mirror = mirror_of(&layout, &stored(b"/Users/a/lib/chairs.blend")).expect("fits");
        let fixed = fixed_copy_address(&layout, &mirror).expect("fits");
        assert_eq!(
            fixed.as_bytes(),
            b"/ephemeral/smoozy/assets/fixed/assets/files/Users/a/lib/chairs.blend"
        );
        // A `.blend` on the machine by another route gets an address off its
        // own machine path.
        let by_hand =
            MachineLocation::composed(b"/ephemeral/smoozy/jobs/hand/chairs.blend".to_vec());
        assert_eq!(
            fixed_copy_address(&layout, &by_hand)
                .expect("fits")
                .as_bytes(),
            b"/ephemeral/smoozy/assets/fixed/jobs/hand/chairs.blend"
        );
        assert_eq!(
            fixed_copy_address(
                &layout,
                &MachineLocation::composed(b"/elsewhere/x.blend".to_vec())
            ),
            Err(CouldNotBeFixed::TheMachineCopyIsNotUnderTheRoot)
        );
        assert!(no_transfer_of_this_modules_publishes_to(&layout, &fixed));
        assert!(!no_transfer_of_this_modules_publishes_to(&layout, &mirror));
    }

    #[test]
    fn a_fixed_copy_can_fail_to_fit_where_its_mirror_fits() {
        let layout = a_layout();
        // Long enough that the mirror fits and the two extra segments push
        // the fixed copy over the whole-path budget.
        let filler: Vec<u8> = std::iter::repeat_n(b"/0123456789".as_slice(), 368)
            .flatten()
            .copied()
            .collect();
        let mut source = b"/U".to_vec();
        source.extend_from_slice(&filler);
        source.extend_from_slice(b"/a.blend");
        let mirror = mirror_of(&layout, &stored(&source)).expect("the mirror fits");
        assert!(matches!(
            fixed_copy_address(&layout, &mirror),
            Err(CouldNotBeFixed::TheAddressWillNotFit(_))
        ));
    }

    struct Prefixes(Vec<(u64, Digest)>);

    impl Readings for Prefixes {
        fn the_local_files_content_identity(
            &self,
            _file: &StoredLocation,
        ) -> Option<ContentIdentity> {
            None
        }
        fn the_four_values_now(
            &self,
            _machine: &MachineKey,
            _at: &MachineLocation,
        ) -> FourValuesNow {
            FourValuesNow::TheChannelCannotAnswer
        }
        fn the_machine_copys_contents(
            &self,
            _machine: &MachineKey,
            _at: &MachineLocation,
        ) -> MachineContents {
            MachineContents::DidNotComplete
        }
        fn enumerate_folder(&self, _folder: &[u8]) -> FolderListing {
            FolderListing::CouldNotBeRead
        }
        fn enumeration_casing_of(&self, _folder: &[u8]) -> EnumerationCasing {
            EnumerationCasing::IsTheStoredName
        }
        fn the_stored_name_reading(&self, _path: &[u8]) -> StoredNameAnswer {
            StoredNameAnswer::NotThere
        }
        fn the_sources_prefix_digest(
            &self,
            _source: &StoredLocation,
            up_to: u64,
        ) -> Option<Digest> {
            self.0
                .iter()
                .find(|(offset, _)| *offset == up_to)
                .map(|(_, digest)| digest.clone())
        }
    }

    #[test]
    fn a_resume_proceeds_only_where_the_sources_prefix_equals_the_checkpoints_digest() {
        let checkpoints = vec![
            Checkpoint {
                offset: 4,
                digest_of_everything_up_to_it: Digest::of(b"abcd"),
            },
            Checkpoint {
                offset: 8,
                digest_of_everything_up_to_it: Digest::of(b"abcdefgh"),
            },
        ];
        let source = stored(b"/Users/a/big.bin");
        let unchanged = Prefixes(vec![(4, Digest::of(b"abcd")), (8, Digest::of(b"abcdefgh"))]);
        assert_eq!(
            a_resume_of_this_modules_upload(10, &checkpoints, &source, &unchanged),
            ResumeVerdict::Proceeds { from: 8 }
        );
        assert_eq!(
            a_resume_of_this_modules_upload(6, &checkpoints, &source, &unchanged),
            ResumeVerdict::Proceeds { from: 4 }
        );
        let changed = Prefixes(vec![(4, Digest::of(b"abcd")), (8, Digest::of(b"abcdefXX"))]);
        assert_eq!(
            a_resume_of_this_modules_upload(10, &checkpoints, &source, &changed),
            ResumeVerdict::StartsOverFromZero
        );
        assert_eq!(
            a_resume_of_this_modules_upload(3, &checkpoints, &source, &unchanged),
            ResumeVerdict::StartsOverFromZero
        );
        let unreadable = Prefixes(vec![]);
        assert_eq!(
            a_resume_of_this_modules_upload(10, &checkpoints, &source, &unreadable),
            ResumeVerdict::StartsOverFromZero
        );
    }

    #[test]
    fn a_handed_unit_opens_under_the_current_installation_and_lands_at_its_own_mirror() {
        let current = Identity::issue(IdentityKind::Installation).expect("randomness");
        let unit = HandedUnit {
            source_path: stored(b"/Users/a/x.png"),
            server_row: Identity::issue(IdentityKind::LocalServer).expect("randomness"),
            installation: Some(current.clone()),
            occupied_landing_path: MachineLocation::composed(
                b"/ephemeral/smoozy/jobs/hand/x.png".to_vec(),
            ),
            class: PlacedOrFixed::Placed,
        };
        assert_eq!(opens(&unit, &current), Ok(()));
        let recycled = Identity::issue(IdentityKind::Installation).expect("randomness");
        assert_eq!(
            opens(&unit, &recycled),
            Err(DoesNotOpen::ASupersededInstallationIdentity)
        );
        // A18: the destination is the mirror of the crossed source path, not
        // the occupied path.
        assert_eq!(
            the_destination_of(&a_layout(), &unit)
                .expect("fits")
                .as_bytes(),
            b"/ephemeral/smoozy/assets/files/Users/a/x.png"
        );
        let no_installation = HandedUnit {
            installation: None,
            ..unit
        };
        assert_eq!(
            opens(&no_installation, &current),
            Err(DoesNotOpen::ASupersededInstallationIdentity)
        );
    }

    fn landing(bytes: &[u8]) -> MachineLocation {
        let mut path = b"/ephemeral/smoozy/assets/files".to_vec();
        path.extend_from_slice(bytes);
        MachineLocation::composed(path)
    }

    /// **A9a: a file is on its way where a running or waiting transfer carries
    /// it from the same source to the same landing place** — and not where
    /// another file of the same name is going, or another layout would land it
    /// elsewhere.
    #[test]
    fn a_file_is_on_its_way_only_where_its_own_landing_place_is_being_sent_to() {
        let going = vec![landing(b"/Users/a/tex/wood.png")];
        assert!(is_on_its_way(&a_layout(), &stored(b"/Users/a/tex/wood.png"), &going));
        assert!(!is_on_its_way(&a_layout(), &stored(b"/Users/a/other/wood.png"), &going));
        assert!(!is_on_its_way(&a_layout(), &stored(b"/Users/a/tex/stone.png"), &going));
        // **An ended transfer is not in the reading**, so nothing is on its way.
        assert!(!is_on_its_way(&a_layout(), &stored(b"/Users/a/tex/wood.png"), &[]));
        // **Another layout lands the same source elsewhere** — another landing
        // place, and the file is not going there.
        let elsewhere = TheLayout::of(
            MachineRoot::chosen("/data/smoozy").expect("a root"),
            WhereThePackageAreaSits::UnderTheRoot,
        );
        assert!(!is_on_its_way(&elsewhere, &stored(b"/Users/a/tex/wood.png"), &going));
    }

    /// **A9a over the three shapes a row sends**: one file, every file under a
    /// folder (F28), a set's members (G22). A row is on its way where any one
    /// of its files is, and a file beside it in the same folder does not make
    /// it so.
    #[test]
    fn a_row_is_on_its_way_where_any_file_it_sends_is() {
        let layout = a_layout();
        let file = stored(b"/Users/a/tex/wood.png");
        let folder = stored(b"/Users/a/cache/fluid");
        let set_folder = stored(b"/Users/a/seq");
        let a_set = WhatARowSends::TheSetIn {
            folder: &set_folder,
            pattern: b"pipe_#####.png",
        };
        let on = |row, going: &[&[u8]]| {
            a_row_is_on_its_way(
                &layout,
                row,
                &going.iter().map(|one| landing(one)).collect::<Vec<_>>(),
            )
        };

        assert!(on(WhatARowSends::TheLocation(&file), &[b"/Users/a/tex/wood.png"]));
        assert!(!on(WhatARowSends::TheLocation(&file), &[b"/Users/a/tex/wood.png.bak"]));

        assert!(on(
            WhatARowSends::TheLocation(&folder),
            &[b"/Users/a/cache/fluid/data/fluid_0001.vdb"]
        ));
        assert!(!on(
            WhatARowSends::TheLocation(&folder),
            &[b"/Users/a/cache/fluid_old/fluid_0001.vdb"]
        ));

        assert!(on(a_set, &[b"/Users/a/seq/pipe_00001.png"]));
        assert!(!on(a_set, &[b"/Users/a/seq/title.png"]));
        assert!(!on(a_set, &[b"/Users/a/seq/more/pipe_00001.png"]));
        assert!(!on(a_set, &[]));
    }
}
