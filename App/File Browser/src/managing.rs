//! Managing files on the machine — `File Browser` §7, M1 to M18, Rows 194 and
//! 192, with the Creator-stated collision rulings.
//!
//! **Constraint:** the spec's *Managing files on the server* — delete, rename,
//! move, copy or duplicate, make a new folder.
//!
//! **§8.4's W11 has its removal half here**, because the acts that publish over
//! a machine location or remove one are four of this section's: a copy's and a
//! same-filesystem move's publication, a delete, and the source removal inside
//! a cross-filesystem move's ordered pair. **The two shapes owe different
//! numbers of writes**: this section's two publications owe X7a's two writes as
//! well as W11's — X7a's opening was widened from *an upload's* publication to
//! every publication of this document's on 2026-09-05 — while its two removals
//! owe W11's alone. Both go through [`crate::publication`], so there is one
//! composition for the publications and the removals alike.
//!
//! **Nothing here models a filesystem.** M10's whole claim is that one rule
//! serves all three faces: the comparison is X5's, the refusal-not-replacement
//! backstop is the base rename everywhere no Replace was answered, and the
//! deliberate replacement is the extension route where one was — **so no refusal
//! can arise from a folding rule this document guessed at.**
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Server protocol and transfers` S1, S4, S5, S9, S16, S26,
//! S28, G4. `Servers` S11, S12. `State, identity and recovery` §2.1, §9.2, §9.3,
//! §9.4, §11.1, D4. `Desktop app and delivery` §15.

use smoozy_server_protocol::transfers::publication::Intent;
use smoozy_server_protocol::transfers::{Ending, StableFileIdentity, TransferRecord};
use smoozy_state::identities::Identity;
use smoozy_state::outcomes::UnitOutcome;

use crate::already_exists::{is_inside, paths_are_equal, CollisionAnswer, Comparison};
use crate::partial::{is_staging_material, THE_RESERVED_STAGING_FOLDER};

/// **M1: deleting is permanent, and the machine-side half of that sentence is
/// observed rather than assumed.**
///
/// Fact 31, answered 2026-08-13: no — ext4, no trash tooling, no trash folder
/// for root, no snapshotting filesystem. **The spec's *is gone* wording stands
/// unchanged.**
///
/// **The limit rides with it: one machine, one filesystem** (§13 item 4,
/// permanently unresolved — a second machine to try recovery tooling on is a
/// rental taken to answer a question, and the Creator ruled that out on
/// 2026-08-31). **What holds is the warning, which is never wrong in the
/// dangerous direction.**
pub const A_DELETE_IS_PERMANENT: bool = true;

/// What the machine answered about a file act. **`Server protocol and
/// transfers` §1.3's separation is read rather than derived.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheMachineSaid {
    ThePathIsNotThere,
    TheLoginMayNot,
    /// **The base rename's own refusal over an existing target** (`Server
    /// protocol and transfers` S16, observed: status 4, `Failure`), and the
    /// plain make-directory's over an existing folder. **This is the judgement
    /// M4 rests on rather than a backstop.**
    ItIsAlreadyThere,
    /// Anything else the server refused. Nothing is concluded from it.
    ItRefused,
    TheChannelIsGone,
}

/// **The acts §7 performs on the machine, and no others.**
///
/// **There is no make-parents form here**, per N3's rule applied to the user's
/// own act (M9), and **no path is ever interpolated into a command string**:
/// every one of these crosses as a protocol field on the file-transfer
/// subsystem.
pub trait TheMachinesFiles {
    /// Remove one file.
    fn remove_one_file(
        &mut self,
        path: &[u8],
    ) -> impl std::future::Future<Output = Result<(), WhatTheMachineSaid>> + Send;

    /// Remove one folder. **Empty**, which is what M2's bottom-up order
    /// delivers.
    fn remove_one_empty_folder(
        &mut self,
        path: &[u8],
    ) -> impl std::future::Future<Output = Result<(), WhatTheMachineSaid>> + Send;

    /// **The base rename** — the one that refuses over an existing target.
    fn base_rename(
        &mut self,
        from: &[u8],
        to: &[u8],
    ) -> impl std::future::Future<Output = Result<(), WhatTheMachineSaid>> + Send;

    /// **The POSIX-rename extension route**, where the connection announced it.
    fn replacing_rename(
        &mut self,
        from: &[u8],
        to: &[u8],
    ) -> impl std::future::Future<Output = Result<(), WhatTheMachineSaid>> + Send;

    /// **The plain make-directory, never the make-parents form** (M9, N3).
    fn make_one_directory(
        &mut self,
        path: &[u8],
    ) -> impl std::future::Future<Output = Result<(), WhatTheMachineSaid>> + Send;

    /// The stable file identity of an object on the machine, where the channel
    /// can answer. **`None` is `Server protocol and transfers` S15's
    /// identity-unavailable branch and never a guess** (§13 item 9).
    fn stable_identity_of(
        &mut self,
        path: &[u8],
    ) -> impl std::future::Future<Output = Option<StableFileIdentity>> + Send;
}

/// **M2: a delete is performed per entry over the subsystem — files removed,
/// folders removed empty, bottom-up — and it is partial across entries.**
///
/// *"A refusal part-way reports which entries are gone, which were refused and
/// which were not attempted, per `State, identity and recovery` §9.4; a folder
/// half-deleted is a truth the next listing shows, and the report says the same
/// thing the listing does."*
///
/// The order is deepest first, which is what makes *folders removed empty* true
/// without a second rule.
pub fn the_delete_order(entries: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut ordered = entries.to_vec();
    ordered.sort_by(|one, other| {
        crate::already_exists::components(other)
            .len()
            .cmp(&crate::already_exists::components(one).len())
    });
    ordered
}

/// **M2's partition: partial across entries**, and the failure of one abandons
/// nothing else.
pub fn a_delete_is_partial_across_entries() -> bool {
    crate::permission::THE_FAILURE_OF_ONE_UNIT_ABANDONS_NOTHING_ELSE_EXCEPT_AN_ORDERED_PAIR
}

/// **M3: the delete warning is this document's one dismissible warning, and its
/// suppression is one row in the one table.**
///
/// Its identity is declared at [`crate::declarations::THE_SUPPRESSION_ROWS`] —
/// `file-browser.delete` (`Desktop app and delivery` §15) — **stable for the
/// life of the application.** *"The two disturb warnings of §8 are not
/// dismissible and no suppression row exists for them"* — Creator-stated.
pub fn the_delete_warnings_suppression_row() -> &'static str {
    crate::declarations::THE_SUPPRESSION_ROWS[0]
}

/// **M3's second half: the delete warning says one more true thing where the
/// target holds live staged material.**
///
/// X12 makes the Smoozy staging directory visible as reserved *(while *Show
/// hidden files* is on — X12 as amended 2026-10-02; with it off the directory
/// is left out of the list like any other hidden name, and this warning reads
/// the record whole in both states, L21)*, and `Server protocol and transfers`
/// S1 places staging *beside the destination* — inside the folder the user is
/// browsing. **Neither non-dismissible warning reaches it**, so this clause is
/// where the cost is said.
///
/// *"At the 1.2–1.3 MB/s uplink Fact 25 measured, 28 GB already staged is hours
/// of the user's own line on a machine billed by the hour."*
///
/// **The reading is the local record's, taken at the moment the victim is named**
/// (W2's shape), **then confirmed against the exact remote object before the
/// act** (X12's own ordering).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatALaterCarryOnCosts {
    /// The staging paths inside the target that a transfer still owns.
    pub staged_material: Vec<Vec<u8>>,
    /// **What `Server protocol and transfers` S12 step 1 then does: a later
    /// carry-on starts from zero.** The figure is not stated here — the real
    /// size belongs to the machine's filesystem (Row 511's reasoning) — and what
    /// the warning says is that the carry-on starts again, not how many bytes it
    /// is.
    pub the_carry_on_starts_from_zero: bool,
}

/// M3's staging reading, composed from the local record alone.
///
/// **Live staged material is a transfer record reading *interrupted*, or one
/// with no ending at all, under the machine's current installation identity.**
pub fn what_a_delete_of_staging_costs(
    target: &[u8],
    records: &[TransferRecord],
    installation_now: Option<&Identity>,
) -> Option<WhatALaterCarryOnCosts> {
    let mut staged_material = Vec::new();
    for record in records {
        if record.installation.as_ref() != installation_now {
            continue;
        }
        let still_live = match &record.ending {
            None => true,
            Some(Ending::Interrupted) => true,
            Some(_) => false,
        };
        if !still_live {
            continue;
        }
        for path in record.every_staging_path_it_has_named() {
            let bytes = path.as_bytes();
            if paths_are_equal(&bytes, target) || is_inside(&bytes, target) {
                staged_material.push(bytes);
            }
        }
    }
    if staged_material.is_empty() {
        return None;
    }
    Some(WhatALaterCarryOnCosts {
        staged_material,
        the_carry_on_starts_from_zero: true,
    })
}

/// **M3's last paragraph, stated so the consequence is not discovered.**
///
/// *"That clause is covered by the `file-browser.delete` row and nothing is
/// added to the table. A user who suppressed the delete warning made a standing
/// choice about deletes, and Rule 8 puts a problem the user brought on
/// themselves outside the plan's job … **A third non-dismissible warning is not
/// made**: M3's two are Creator-stated, and adding one would put a warning in
/// front of the user that no spec describes and no ruling grants."*
pub const NO_THIRD_NON_DISMISSIBLE_WARNING_IS_MADE: bool = true;

/// **M4: a rename is the base rename, and the filesystem's own refusal is the
/// judgement rather than the backstop.**
///
/// *"The listing's role is advisory"* — L14 makes it a per-folder cache holding
/// that folder's last complete listing and the moment it was taken, so **the
/// listing behind a refusal may be minutes old on a live connection while the
/// machine is the user's.** A pre-check that refused from it would state that a
/// name is taken when the machine holds nothing under it, **which is the fault
/// Row 192 names, arriving by staleness rather than by modelling.**
///
/// **So the browser may warn from the listing, the rename is attempted, and the
/// base rename's own refusal (`Server protocol and transfers` S16) is what
/// reports *taken*.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheListingMayDoAboutARename {
    /// It may warn. **It may not refuse.**
    WarnOnly,
}

/// M4, stated as the one thing the cache is allowed to do.
pub fn what_the_listing_may_do() -> WhatTheListingMayDoAboutARename {
    WhatTheListingMayDoAboutARename::WarnOnly
}

/// What a rename attempt came to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HowTheRenameEnded {
    /// **Atomic per item**: one base rename, refused or done.
    Done,
    /// **The machine holds something under that name.** *"The refused name is
    /// reported as taken and the user picks another."*
    TheNameIsTaken,
    /// Anything else, reported as it stands.
    Refused(WhatTheMachineSaid),
}

/// M4's reading of the machine's answer. **Nothing is modelled and nothing is
/// ever replaced by a rename.**
pub fn how_the_rename_ended(said: Result<(), WhatTheMachineSaid>) -> HowTheRenameEnded {
    match said {
        Ok(()) => HowTheRenameEnded::Done,
        Err(WhatTheMachineSaid::ItIsAlreadyThere) => HowTheRenameEnded::TheNameIsTaken,
        Err(other) => HowTheRenameEnded::Refused(other),
    }
}

/// **M5's paragraph of 2026-10-02, as seen at the desk: how a move's refused
/// rename is read.**
///
/// The plan wrote that the file service's refusal reads the same for a move
/// across filesystems as for a name already taken, as *an assumption, not
/// seen*. **It was seen on 2026-10-02**, with OpenSSH's own `sftp-server` and a
/// second filesystem attached at the desk: the base rename onto a name
/// already there, the base rename of a file and of a folder across the two
/// filesystems, and the base rename across them onto a name already there all
/// answered status 4, *Failure*; a source that was not there answered *no such
/// file*. **The replacing rename (`Server protocol and transfers` S16) across
/// the two answered *Failure* too, with or without something at the
/// destination**, while on one filesystem it replaced.
///
/// So the plan's rule holds for the base rename — something at the name read
/// straight after means *taken*, nothing means a crossing — **and the
/// replacing rename needs the other half said**: it replaces whatever is at a
/// file's name on one filesystem, so its refusal is a crossing whether or not
/// something is there. A refusal for permission is P1's and is neither.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HowTheMoveEnded {
    /// The rename took effect.
    Done,
    /// **M4's *taken***: the base rename was refused with something at the
    /// destination name.
    TheNameIsTaken,
    /// **M6's case**: the machine refused the rename as crossing filesystems,
    /// and the move goes to the ordered copy-then-delete pair.
    ItCrossesFilesystems,
    /// Anything else, reported as it stands.
    Refused(WhatTheMachineSaid),
}

/// M5's reading of a move's rename. `something_is_at_the_destination` is the
/// destination name read straight after a refusal, and is not consulted
/// otherwise.
pub fn how_the_move_ended(
    said: Result<(), WhatTheMachineSaid>,
    intent: Intent,
    something_is_at_the_destination: bool,
) -> HowTheMoveEnded {
    match said {
        Ok(()) => HowTheMoveEnded::Done,
        Err(refused @ (WhatTheMachineSaid::TheLoginMayNot
        | WhatTheMachineSaid::ThePathIsNotThere
        | WhatTheMachineSaid::TheChannelIsGone)) => HowTheMoveEnded::Refused(refused),
        Err(_) if intent == Intent::ReplaceIntended => HowTheMoveEnded::ItCrossesFilesystems,
        Err(_) if something_is_at_the_destination => HowTheMoveEnded::TheNameIsTaken,
        Err(_) => HowTheMoveEnded::ItCrossesFilesystems,
    }
}

/// **M5: a move within one filesystem is a rename per top-level item** —
/// create-intended (base rename) where no collision was answered,
/// replace-intended (the extension) where the user answered Replace on a
/// colliding move. **Atomic per item, partial across items.**
pub fn the_intent_for_a_move(comparison: Comparison, answer: Option<CollisionAnswer>) -> Intent {
    match (comparison.is_a_collision(), answer) {
        (true, Some(CollisionAnswer::Replace)) => Intent::ReplaceIntended,
        _ => Intent::CreateIntended,
    }
}

/// **M6: a move the machine refuses as crossing filesystems is performed per
/// file as an ordered copy-then-delete pair.**
///
/// *"The copy staged and published under M7's rule, the delete of the source
/// performed only after the copy's publication (`State, identity and recovery`
/// §9.3: the delete is unsafe unless the copy succeeded; on the copy's failure
/// the delete is **not attempted**, naming it). Partial across files; a tree
/// half-moved is reported as exactly that."*
///
/// **The case is reachable on real rentals rather than hypothetical** — an
/// attached volume presents its own filesystem at its mount (Fact 28's
/// more-than-one-disk clause).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheSecondHalfOfThePairDoes {
    /// The copy published, so the source's delete runs.
    DeleteTheSource,
    /// **§9.3's ordered pair: the delete is not attempted, naming the copy.**
    NotAttempted { stopped_by: String },
}

/// M6's ordering, as one function so no caller invents a second.
pub fn what_the_second_half_does(the_copy_published: bool) -> WhatTheSecondHalfOfThePairDoes {
    if the_copy_published {
        WhatTheSecondHalfOfThePairDoes::DeleteTheSource
    } else {
        WhatTheSecondHalfOfThePairDoes::NotAttempted {
            stopped_by: "the copy".to_string(),
        }
    }
}

/// M6's unit outcome for the delete half, written into the §9.4 record.
pub fn the_delete_halfs_outcome(what: &WhatTheSecondHalfOfThePairDoes) -> Option<UnitOutcome> {
    match what {
        WhatTheSecondHalfOfThePairDoes::DeleteTheSource => None,
        WhatTheSecondHalfOfThePairDoes::NotAttempted { stopped_by } => {
            Some(UnitOutcome::NotAttempted {
                stopped_by: stopped_by.clone(),
            })
        }
    }
}

/// **M7: a machine-side copy never writes at a final name, for the same reason a
/// transfer never does.**
///
/// *"A copy killed part-way is Fact 18's partial arriving without a wire; so a
/// copy is written to a staging path and published by one rename, create- or
/// replace-intended per the answer, exactly the contract's shape."*
///
/// **The staging path for a copy is this document's own path kind, synthesised
/// from the copy's operation identity** under `State, identity and recovery`
/// §2.1 rule 7's exemption for the owning document, **placed in the Smoozy
/// staging directory beside the destination.**
///
/// **It is not a transfer's staging path and `Server protocol and transfers` S28
/// never sweeps it as transfer material** — which is why the name is composed
/// here rather than by that document's S4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CopyStagingPath {
    directory: String,
    name: String,
}

impl CopyStagingPath {
    /// The one way to make one. **The directory is beside the destination**, so
    /// the publishing rename stays inside one filesystem.
    pub fn synthesise(beside_the_destination: &str, copy: &Identity) -> CopyStagingPath {
        CopyStagingPath {
            directory: format!(
                "{}/{THE_RESERVED_STAGING_FOLDER}",
                beside_the_destination.trim_end_matches('/')
            ),
            // The identity's written form is lower-case hexadecimal, the hyphen
            // and the suffix the rest of it. **Nothing of the file's own name is
            // here**, which is S4's reasoning applied to this document's own
            // path kind.
            name: format!("{}-copy.smoozy-part", copy.as_text()),
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        format!("{}/{}", self.directory, self.name).into_bytes()
    }

    pub fn directory(&self) -> &str {
        &self.directory
    }

    /// The same class-2 alphabet `Server protocol and transfers` S4 fixes for
    /// every path it synthesises — lower-case letters, digits, hyphens and the
    /// dot. **A token so drawn is syntax on no route.**
    pub fn is_drawn_from_class_twos_alphabet(&self) -> bool {
        self.name.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-' || byte == b'.'
        })
    }
}

/// **M7's copy operation record: the exact path, the installation identity and
/// the staged object's stable file identity, stored before any byte is
/// written.**
///
/// *"File Browser removes abandoned copy staging only from a live or ended copy
/// record in a removable state and only when all three identities match;
/// `Server protocol and transfers` S28 never sweeps it as transfer material. An
/// absent record or mismatched object is left visible for X12's explicit orphan
/// deletion."*
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CopyOperationRecord {
    pub copy: Identity,
    pub staging: CopyStagingPath,
    pub installation: Option<Identity>,
    /// Written **before any byte** — absent until the staged object exists,
    /// which is the honest state and not a blank.
    pub staged_identity: Option<StableFileIdentity>,
    pub ended: bool,
}

/// **M7's removal rule: all three identities match, or nothing is removed
/// automatically.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhetherTheCopyStagingMayBeRemoved {
    /// The record owns it and every identity matches.
    Yes,
    /// **Left visible for X12's explicit orphan deletion.**
    NoItIsAnOrphan,
}

/// M7's removal rule.
pub fn may_the_copy_staging_be_removed(
    target: &[u8],
    records: &[CopyOperationRecord],
    installation_now: Option<&Identity>,
    identity_now: Option<&StableFileIdentity>,
) -> WhetherTheCopyStagingMayBeRemoved {
    for record in records {
        if !paths_are_equal(&record.staging.as_bytes(), target) {
            continue;
        }
        if record.installation.as_ref() != installation_now {
            return WhetherTheCopyStagingMayBeRemoved::NoItIsAnOrphan;
        }
        let recorded = match record.staged_identity.as_ref() {
            Some(StableFileIdentity::The(recorded)) => recorded,
            _ => return WhetherTheCopyStagingMayBeRemoved::NoItIsAnOrphan,
        };
        return match identity_now {
            Some(StableFileIdentity::The(now)) if now == recorded => {
                WhetherTheCopyStagingMayBeRemoved::Yes
            }
            _ => WhetherTheCopyStagingMayBeRemoved::NoItIsAnOrphan,
        };
    }
    WhetherTheCopyStagingMayBeRemoved::NoItIsAnOrphan
}

/// **M7's last paragraph: what a copy reports while it runs, because the answer
/// differs by shape.**
///
/// *"A multi-file copy reports the **per-file unit outcomes** of its §9.4 record
/// … **A single very large file's copy has one unit and therefore nothing to
/// report between its two ends**, and that is stated plainly rather than left to
/// look like a hang: the report is submission and completion, the bytes move on
/// the machine and never across the wire, and **no byte count is invented for a
/// movement Smoozy is not performing.**"*
///
/// `Server protocol and transfers` G4's reporting rule binds that document's own
/// requests rather than this document's copy, **so what is owed here is the
/// statement and not a hole in that document.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatACopyReports {
    /// Per-file unit outcomes, moving as each file publishes.
    ThePerFileUnitOutcomes,
    /// **Submission and completion, and nothing between.**
    SubmissionAndCompletionOnly,
}

/// M7's report shape, by how many files the copy is over.
pub fn what_a_copy_reports(files: usize) -> WhatACopyReports {
    if files > 1 {
        WhatACopyReports::ThePerFileUnitOutcomes
    } else {
        WhatACopyReports::SubmissionAndCompletionOnly
    }
}

/// **M7's own prohibition, stated so nothing composes one.**
pub const NO_BYTE_COUNT_IS_INVENTED_FOR_A_MOVEMENT_SMOOZY_IS_NOT_PERFORMING: bool = true;

/// **M8: a colliding copy offers Replace, Keep both, or Cancel; a colliding move
/// offers Replace or Cancel** — the spec's split, with Keep both composing its
/// name under X9's rule.
///
/// **Both are judged by X5's comparison against the destination listing at the
/// moment of the act.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheAnswersOffered {
    /// A copy: Replace, Keep both, Cancel.
    ReplaceKeepBothOrCancel,
    /// A move: Replace, Cancel.
    ReplaceOrCancel,
}

/// **M18: which act it was is what M8 reads**, so *"Paste has to remember which
/// it was"* is kept by M12's second field and needs nothing further.
pub fn the_answers_offered(act: CutOrCopy) -> TheAnswersOffered {
    match act {
        CutOrCopy::Copy => TheAnswersOffered::ReplaceKeepBothOrCancel,
        CutOrCopy::Cut => TheAnswersOffered::ReplaceOrCancel,
    }
}

/// **M9: a new folder is the plain make-directory, never the make-parents
/// form** (N3's rule, applied to the user's own act).
///
/// *"Already exists is answered before the attempt from the listing the user is
/// looking at, and by the plain form's own refusal where the race lands; nothing
/// is silently merged."*
///
/// **Atomic entire** — the plain form, refused or done.
pub const A_NEW_FOLDER_IS_THE_PLAIN_FORM_AND_NEVER_THE_MAKE_PARENTS_ONE: bool = true;

/// **M10: one rule serves all three faces.**
///
/// The comparison is X5's; the refusal-not-replacement backstop is the base
/// rename everywhere no Replace was answered; and the deliberate replacement is
/// the extension route where one was. **Nothing is modelled anywhere here, so no
/// refusal can arise from a folding rule this document guessed at** — and that
/// is the whole of what M10 claims. **The other route to a wrong refusal, a
/// listing gone stale under a live connection, is closed at M4 by letting the
/// filesystem answer rather than the cache.**
pub fn the_intent_for_any_face(comparison: Comparison, answer: Option<CollisionAnswer>) -> Intent {
    the_intent_for_a_move(comparison, answer)
}

/// **M11: there is one pasteboard, it holds one thing, and it is period-held.**
///
/// Design makes cut, copy and paste the shape of a move and a copy
/// (**Creator-stated**, 2026-08-10), and **a pasteboard is the only thing in
/// this module that persists between acts.** Everything else names its victim
/// and its destination in one moment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pasteboard {
    /// **M12: what it holds** — the server row and the installation identity it
    /// was taken under; which act it was; and the named entries, as paths on
    /// that machine, exactly as the listing gave them. **Nothing else, and no
    /// reading of any of them.**
    pub server_row: Identity,
    pub installation: Option<Identity>,
    pub act: CutOrCopy,
    pub entries: Vec<Vec<u8>>,
}

/// Which act the pasteboard was filled by.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CutOrCopy {
    Cut,
    Copy,
}

/// **M13: its lifetime is the connected period's**, on `Servers` S11's list
/// beside the other machine-keyed readings — **not the application run's, and
/// nothing durable is written from it** (`State, identity and recovery` D4, and
/// its §11.1 period-held class, this clause naming it a member).
///
/// **A drop does not clear it**, because a drop does not end a period (`Servers`
/// S12), **which is what lets a cut survive the connection being repaired under
/// the user.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatBecomesOfThePasteboard {
    ItStands,
    /// The period ended, so it goes with the period.
    ItGoesWithThePeriod,
}

/// M13, per moment.
pub fn what_a_drop_does_to_the_pasteboard() -> WhatBecomesOfThePasteboard {
    // `Servers` S12: a drop does not end a period, so nothing here fires.
    WhatBecomesOfThePasteboard::ItStands
}

/// M13, at a period ending. **The period's own list is `Servers` S10's and is
/// read rather than re-derived** — [`crate::cache::what_an_ending_does`] already
/// reads it for the listing tree, and the pasteboard's answer is the simpler one
/// because it is period-held outright.
pub fn what_a_period_ending_does_to_the_pasteboard() -> WhatBecomesOfThePasteboard {
    WhatBecomesOfThePasteboard::ItGoesWithThePeriod
}

impl Pasteboard {
    /// **M14: it survives navigation, which is the whole point of it**, and
    /// *"one copy can be pasted into several folders in turn"* is exactly that:
    /// **a paste does not clear the pasteboard, whichever act it was.**
    ///
    /// *"A cut pasted twice is two moves, and the second finds its source gone
    /// and refuses at the moment of the act like any other missing source."*
    pub fn a_paste_does_not_clear_it(&self) -> bool {
        true
    }

    /// **M15: paste is offered only where the browsing server row and
    /// installation identity are the pasteboard's**, and **nowhere else is it
    /// greyed, refused or explained — it is not offered at all.**
    ///
    /// *"That is what keeps* Both stay within the one machine *true by
    /// construction rather than by a check at the moment of the act: **there is
    /// no route by which a cross-machine paste can be attempted.**"*
    ///
    /// **M16: a machine that failed the identity gate takes the pasteboard with
    /// it.** The identity is on the pasteboard, so a recycled machine simply
    /// does not match and the paste is not offered; **nothing is cleared by hand
    /// and no rule of its own is needed.**
    pub fn paste_is_offered(
        &self,
        browsing_row: &Identity,
        installation_now: Option<&Identity>,
    ) -> bool {
        &self.server_row == browsing_row && self.installation.as_ref() == installation_now
    }

    /// **M17: nothing on the pasteboard is validated while it sits there.**
    ///
    /// *"The entries are **names**, not identities, and no reading is taken to
    /// keep them true — a source removed by the user's own second login is
    /// discovered by the machine's own refusal when the paste is taken, which is
    /// W2's ordering already. **A pasteboard that quietly emptied itself would
    /// be worse**: the user would lose a cut for a reason nothing on the screen
    /// could show."*
    pub fn nothing_is_validated_while_it_sits(&self) -> bool {
        true
    }
}

/// **M15's own consequence, stated as a value.** There is no *cross-machine
/// paste refused* state, because there is no route to attempt one.
pub const THERE_IS_NO_ROUTE_BY_WHICH_A_CROSS_MACHINE_PASTE_CAN_BE_ATTEMPTED: bool = true;

/// **W11's removal half: the acts of this document's that owe the check
/// standing's *disturbed* mark, and §7 carries four of the five.**
///
/// W11 names them: an upload's publication (X7), **a copy's and a
/// same-filesystem move's publication (M7, M5), a delete (M2), and the source
/// removal inside a cross-filesystem move's ordered pair (M6)**. Those are
/// `Dependency Upload & Remap` V13a's *replaced* and *deleted* on this side;
/// its *refixed* is that module's own act and is never this document's.
///
/// **The five owe different numbers of writes, and that is what the unit's kind
/// decides.** **Three of them publish over a location** — an upload's, a copy's
/// and a same-filesystem move's publication — and after X7a's widening of
/// 2026-09-05 those three owe **all three** writes: the standing marked
/// replaced-since-check, the recorded reading dropped, and the disturbance
/// mark. **Two of them remove one** — a delete, and the source removal inside a
/// cross-filesystem move's pair — and those owe **the disturbance mark alone**,
/// there being no publication for X7a's two to be made at.
///
/// **So [`at_this_unit_of_a_managing_act`] is told which of the two the unit
/// was**, and the publication kind routes through
/// [`crate::publication::at_this_publication`] so the composition stays in one
/// place rather than becoming two that can drift.
pub const THE_ACTS_OF_THIS_DOCUMENT_THAT_OWE_THE_DISTURBANCE_MARK: [&str; 7] = [
    "an upload's publication",
    "a copy's publication",
    "a same-filesystem move's publication",
    "a delete",
    "the source removal inside a cross-filesystem move's ordered pair",
    // **W11 as widened 2026-10-02, closing §13 item 20**: a rename and a
    // same-filesystem move owe the mark at the place they moved away from.
    "a rename, at the place it moved away from",
    "a same-filesystem move, at the place it moved away from",
];

/// **What one unit of an act of §7's did to a machine location**, which is what
/// decides how many writes it owes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheUnitDid {
    /// A copy's or a same-filesystem move's publication (M7, M5) — X7a's two
    /// writes and W11's.
    PublishedOverIt,
    /// A delete (M2), or the source removal inside a cross-filesystem move's
    /// ordered pair (M6) — W11's write alone.
    RemovedIt,
    /// **A rename (M4) or a same-filesystem move (M5), at the place it moved
    /// away from** — W11's write alone, under `Dependency Upload & Remap`
    /// V13a's *moved or renamed away*. Added 2026-10-02, step 10c's Files'
    /// owed acts piece 3, when W11 was widened to cover it.
    MovedAwayFromIt,
}

/// **X7a and W11 at one unit of an act of §7's: what the location the unit
/// published over or removed owes the neighbouring records.**
///
/// **A unit that published over a location owes all three writes and a unit
/// that removed one owes the disturbance mark alone** — X7a's two are writes
/// made *at a publication*, and a removal is not one. The publication route
/// goes through [`crate::publication::at_this_publication`] so the composition
/// of the three is in one place.
///
/// **Only for a unit that took effect** — published, for a publication;
/// removed, for a removal. That is W9's rule taken for the same reason: a
/// request whose units disagreed would otherwise mark a standing over a file
/// that never moved. **Took effect is [`UnitOutcome::Done`] and neither of the
/// other two**, so a unit that failed, and the cross-filesystem pair's delete
/// that was **not attempted** because the copy did not publish (M6), each write
/// nothing.
///
/// **The composition, the readings and the comparison are
/// [`crate::publication`]'s**, so the removal half and the publication half are
/// one write rather than two that could drift apart. It reads no machine.
pub fn at_this_unit_of_a_managing_act(
    location: &[u8],
    did: WhatTheUnitDid,
    outcome: Option<&UnitOutcome>,
    records: &mut dyn crate::publication::TheNeighbouringRecords,
) -> crate::publication::WhatThePublicationWrote {
    match did {
        WhatTheUnitDid::PublishedOverIt => {
            crate::publication::at_this_publication(location, outcome, records)
        }
        WhatTheUnitDid::RemovedIt | WhatTheUnitDid::MovedAwayFromIt => {
            let mut wrote = crate::publication::WhatThePublicationWrote::nothing();
            if outcome != Some(&UnitOutcome::Done) {
                return wrote;
            }
            wrote.the_standings_marked_disturbed =
                crate::publication::the_disturbance_mark(location, records);
            wrote
        }
    }
}

/// **A rename, and a same-filesystem move away from a location, mark the check
/// standings whose tree resolved the place they left** — W11 as widened
/// 2026-10-02, closing §13 item 20, under `Dependency Upload & Remap` V13a's
/// *moved or renamed away*. **Nothing is marked at the place they land unless
/// it published over something**, which is the move's own publication.
///
/// *(Until step 10c's Files' owed acts piece 3 this constant said a rename and a
/// move away marked nothing — §13 item 20, routed and never ruled. The same
/// gesture, a texture moved to another folder, then marked the standing on one
/// disk layout, where the move crossed disks and its delete half removed the
/// file, and not on another.)*
///
/// [`WhatTheUnitDid::MovedAwayFromIt`] is the unit's kind, and it writes W11's
/// disturbance mark alone.
pub const A_RENAME_AND_A_MOVE_AWAY_MARK_THE_PLACE_THEY_LEFT: bool = true;

/// **M9's paragraph of 2026-10-02: why a typed name is refused before anything
/// is sent**, for a new folder and a rename (M4). Nothing else is judged in
/// advance, so M4's *the filesystem answers* stands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyTheNameIsRefused {
    Empty,
    /// It holds a `/` — that is a path, and it would move the file.
    HoldsASlash,
    /// `.` or `..`.
    IsADotName,
    /// Longer than X9's bound of 255 bytes.
    TooLong,
    /// The staging directory's own name (X12).
    IsTheStagingFoldersName,
}

/// **X9's bound on one name**, the observed accepted maximum.
pub const THE_LONGEST_NAME: usize = 255;

/// M9's refusal before any attempt, or nothing where the name may be tried.
pub fn a_typed_name_is_refused(name: &[u8]) -> Option<WhyTheNameIsRefused> {
    if name.is_empty() {
        return Some(WhyTheNameIsRefused::Empty);
    }
    if name.contains(&b'/') {
        return Some(WhyTheNameIsRefused::HoldsASlash);
    }
    if name == b"." || name == b".." {
        return Some(WhyTheNameIsRefused::IsADotName);
    }
    if name.len() > THE_LONGEST_NAME {
        return Some(WhyTheNameIsRefused::TooLong);
    }
    if name == THE_RESERVED_STAGING_FOLDER.as_bytes() {
        return Some(WhyTheNameIsRefused::IsTheStagingFoldersName);
    }
    None
}

/// **X12's first line applied to §7: an arriving delete aimed at the reserved
/// staging folder is transfer-aware.** Stated here so a reader of §7's delete
/// meets it rather than only a reader of §4.4.
pub fn a_delete_target_is_staging_material(target: &[u8]) -> bool {
    is_staging_material(target)
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_server_protocol::transfers::Direction;
    use smoozy_state::identities::IdentityKind;

    fn an_identity(kind: IdentityKind) -> Identity {
        Identity::issue(kind).expect("this machine supplies random bytes")
    }

    fn a_transfer_record(installation: Option<Identity>, ending: Option<Ending>) -> TransferRecord {
        let mut record = TransferRecord::arranged(
            an_identity(IdentityKind::Transfer),
            Direction::Up,
            an_identity(IdentityKind::LocalServer),
            installation,
            "/home/a.png".to_string(),
            "/jobs/a.png".to_string(),
            "/jobs/.smoozy-staging",
            100,
            an_identity(IdentityKind::Run),
        );
        record.ending = ending;
        record
    }

    #[test]
    fn a_delete_removes_the_deepest_entries_first() {
        let ordered = the_delete_order(&[
            b"/jobs/scene".to_vec(),
            b"/jobs/scene/textures/a.png".to_vec(),
            b"/jobs/scene/textures".to_vec(),
        ]);
        assert_eq!(
            ordered,
            vec![
                b"/jobs/scene/textures/a.png".to_vec(),
                b"/jobs/scene/textures".to_vec(),
                b"/jobs/scene".to_vec(),
            ]
        );
        assert!(a_delete_is_partial_across_entries());
        const { assert!(A_DELETE_IS_PERMANENT) };
    }

    #[test]
    fn the_one_dismissible_warning_is_the_delete_and_no_third_is_made() {
        assert_eq!(the_delete_warnings_suppression_row(), "file-browser.delete");
        assert_eq!(crate::declarations::THE_SUPPRESSION_ROWS.len(), 1);
        const { assert!(NO_THIRD_NON_DISMISSIBLE_WARNING_IS_MADE) };
    }

    #[test]
    fn a_delete_over_live_staging_says_what_a_later_carry_on_costs() {
        let installation = an_identity(IdentityKind::Installation);
        let live = a_transfer_record(Some(installation.clone()), Some(Ending::Interrupted));
        let moving = a_transfer_record(Some(installation.clone()), None);
        let over = a_transfer_record(Some(installation.clone()), Some(Ending::Published));
        let cost = what_a_delete_of_staging_costs(
            b"/jobs/.smoozy-staging",
            &[live.clone(), moving.clone(), over],
            Some(&installation),
        )
        .expect("live staged material is named");
        assert_eq!(cost.staged_material.len(), 2);
        assert!(cost.the_carry_on_starts_from_zero);
        // No byte figure is composed: the type has no field for one.
        let shown = format!("{cost:?}");
        assert!(!shown.contains("bytes"));

        // A superseded installation names nothing.
        assert!(what_a_delete_of_staging_costs(
            b"/jobs/.smoozy-staging",
            &[live, moving],
            Some(&an_identity(IdentityKind::Installation))
        )
        .is_none());
    }

    #[test]
    fn a_delete_somewhere_else_names_no_staging() {
        let installation = an_identity(IdentityKind::Installation);
        let live = a_transfer_record(Some(installation.clone()), Some(Ending::Interrupted));
        assert!(
            what_a_delete_of_staging_costs(b"/elsewhere", &[live], Some(&installation)).is_none()
        );
    }

    #[test]
    fn the_listing_may_warn_about_a_rename_and_never_refuse_from_it() {
        assert_eq!(
            what_the_listing_may_do(),
            WhatTheListingMayDoAboutARename::WarnOnly
        );
        assert_eq!(
            how_the_rename_ended(Err(WhatTheMachineSaid::ItIsAlreadyThere)),
            HowTheRenameEnded::TheNameIsTaken
        );
        assert_eq!(how_the_rename_ended(Ok(())), HowTheRenameEnded::Done);
        assert_eq!(
            how_the_rename_ended(Err(WhatTheMachineSaid::TheLoginMayNot)),
            HowTheRenameEnded::Refused(WhatTheMachineSaid::TheLoginMayNot)
        );
    }

    /// **M5's rule for a refused move, as seen at the desk on 2026-10-02**:
    /// the base rename's refusal is *taken* with something at the name and a
    /// crossing with nothing there; the replacing rename's refusal is a
    /// crossing either way; a refusal for permission, a missing source and a
    /// line gone are neither.
    #[test]
    fn a_refused_move_is_taken_or_a_crossing_by_what_stands_at_the_name() {
        use smoozy_server_protocol::transfers::publication::Intent;
        let refused = || Err(WhatTheMachineSaid::ItIsAlreadyThere);
        assert_eq!(how_the_move_ended(Ok(()), Intent::CreateIntended, true), HowTheMoveEnded::Done);
        assert_eq!(how_the_move_ended(refused(), Intent::CreateIntended, true), HowTheMoveEnded::TheNameIsTaken);
        assert_eq!(how_the_move_ended(refused(), Intent::CreateIntended, false), HowTheMoveEnded::ItCrossesFilesystems);
        assert_eq!(
            how_the_move_ended(Err(WhatTheMachineSaid::ItRefused), Intent::ReplaceIntended, true),
            HowTheMoveEnded::ItCrossesFilesystems
        );
        for neither in [
            WhatTheMachineSaid::TheLoginMayNot,
            WhatTheMachineSaid::ThePathIsNotThere,
            WhatTheMachineSaid::TheChannelIsGone,
        ] {
            assert_eq!(
                how_the_move_ended(Err(neither.clone()), Intent::CreateIntended, false),
                HowTheMoveEnded::Refused(neither)
            );
        }
    }

    #[test]
    fn a_move_replaces_only_where_replace_was_answered() {
        assert_eq!(
            the_intent_for_a_move(
                Comparison::CollidesWithAFile,
                Some(CollisionAnswer::Replace)
            ),
            Intent::ReplaceIntended
        );
        assert_eq!(
            the_intent_for_a_move(Comparison::CollidesWithAFile, None),
            Intent::CreateIntended
        );
        assert_eq!(
            the_intent_for_a_move(Comparison::LandsFresh, Some(CollisionAnswer::Replace)),
            Intent::CreateIntended,
            "no collision, so nothing is replaced however the question was answered"
        );
        // M10: one rule over all three faces.
        assert_eq!(
            the_intent_for_any_face(
                Comparison::CollidesWithAFile,
                Some(CollisionAnswer::Replace)
            ),
            Intent::ReplaceIntended
        );
    }

    #[test]
    fn the_cross_filesystem_pairs_delete_is_not_attempted_where_the_copy_did_not_publish() {
        assert_eq!(
            what_the_second_half_does(true),
            WhatTheSecondHalfOfThePairDoes::DeleteTheSource
        );
        let refused = what_the_second_half_does(false);
        assert_eq!(
            the_delete_halfs_outcome(&refused),
            Some(UnitOutcome::NotAttempted {
                stopped_by: "the copy".to_string()
            }),
            "named, rather than reported as failed"
        );
        assert_eq!(
            the_delete_halfs_outcome(&what_the_second_half_does(true)),
            None
        );
    }

    #[test]
    fn a_copys_staging_is_this_documents_own_path_kind_and_holds_no_file_name() {
        let copy = an_identity(IdentityKind::Operation);
        let staging = CopyStagingPath::synthesise("/jobs/scene", &copy);
        assert_eq!(staging.directory(), "/jobs/scene/.smoozy-staging");
        assert!(staging.is_drawn_from_class_twos_alphabet());
        let text = String::from_utf8(staging.as_bytes()).expect("the path is composed by Smoozy");
        assert!(text.contains(&copy.as_text()));
        assert!(
            !text.contains("scene.blend"),
            "nothing of the file's own name is in it"
        );
    }

    #[test]
    fn copy_staging_is_removed_only_where_all_three_identities_match() {
        let copy = an_identity(IdentityKind::Operation);
        let installation = an_identity(IdentityKind::Installation);
        let staging = CopyStagingPath::synthesise("/jobs", &copy);
        let record = CopyOperationRecord {
            copy: copy.clone(),
            staging: staging.clone(),
            installation: Some(installation.clone()),
            staged_identity: Some(StableFileIdentity::The("1234".to_string())),
            ended: true,
        };
        let target = staging.as_bytes();
        assert_eq!(
            may_the_copy_staging_be_removed(
                &target,
                std::slice::from_ref(&record),
                Some(&installation),
                Some(&StableFileIdentity::The("1234".to_string()))
            ),
            WhetherTheCopyStagingMayBeRemoved::Yes
        );
        // A different object at the path.
        assert_eq!(
            may_the_copy_staging_be_removed(
                &target,
                std::slice::from_ref(&record),
                Some(&installation),
                Some(&StableFileIdentity::The("9999".to_string()))
            ),
            WhetherTheCopyStagingMayBeRemoved::NoItIsAnOrphan
        );
        // The channel could not answer.
        assert_eq!(
            may_the_copy_staging_be_removed(
                &target,
                std::slice::from_ref(&record),
                Some(&installation),
                None
            ),
            WhetherTheCopyStagingMayBeRemoved::NoItIsAnOrphan
        );
        // No record names it at all.
        assert_eq!(
            may_the_copy_staging_be_removed(
                b"/jobs/.smoozy-staging/somebody-elses",
                &[record],
                Some(&installation),
                Some(&StableFileIdentity::The("1234".to_string()))
            ),
            WhetherTheCopyStagingMayBeRemoved::NoItIsAnOrphan
        );
    }

    #[test]
    fn a_single_file_copy_reports_its_two_ends_and_invents_no_byte_count() {
        assert_eq!(
            what_a_copy_reports(1),
            WhatACopyReports::SubmissionAndCompletionOnly
        );
        assert_eq!(
            what_a_copy_reports(300),
            WhatACopyReports::ThePerFileUnitOutcomes
        );
        const { assert!(NO_BYTE_COUNT_IS_INVENTED_FOR_A_MOVEMENT_SMOOZY_IS_NOT_PERFORMING) };
    }

    #[test]
    fn a_colliding_copy_offers_three_answers_and_a_colliding_move_two() {
        assert_eq!(
            the_answers_offered(CutOrCopy::Copy),
            TheAnswersOffered::ReplaceKeepBothOrCancel
        );
        assert_eq!(
            the_answers_offered(CutOrCopy::Cut),
            TheAnswersOffered::ReplaceOrCancel
        );
    }

    fn a_pasteboard(act: CutOrCopy) -> (Pasteboard, Identity, Identity) {
        let server_row = an_identity(IdentityKind::LocalServer);
        let installation = an_identity(IdentityKind::Installation);
        (
            Pasteboard {
                server_row: server_row.clone(),
                installation: Some(installation.clone()),
                act,
                entries: vec![b"/jobs/a.png".to_vec()],
            },
            server_row,
            installation,
        )
    }

    #[test]
    fn paste_is_offered_only_on_the_row_and_installation_it_was_taken_under() {
        let (board, row, installation) = a_pasteboard(CutOrCopy::Cut);
        assert!(board.paste_is_offered(&row, Some(&installation)));
        // Another machine: not offered at all, and there is no refused state.
        assert!(
            !board.paste_is_offered(&an_identity(IdentityKind::LocalServer), Some(&installation))
        );
        // A recycled machine takes it with it (M16).
        assert!(!board.paste_is_offered(&row, Some(&an_identity(IdentityKind::Installation))));
        const { assert!(THERE_IS_NO_ROUTE_BY_WHICH_A_CROSS_MACHINE_PASTE_CAN_BE_ATTEMPTED) };
    }

    #[test]
    fn the_pasteboard_holds_four_things_and_no_reading_of_any_of_them() {
        let (board, _, _) = a_pasteboard(CutOrCopy::Copy);
        let shown = format!("{board:?}");
        assert!(shown.contains("server_row"));
        assert!(shown.contains("installation"));
        assert!(shown.contains("act"));
        assert!(shown.contains("entries"));
        // No size, no kind, no time: the entries are names and nothing else.
        for reading in ["size", "kind", "last_changed", "identity_now"] {
            assert!(
                !shown.contains(reading),
                "{reading} is a reading, not a name"
            );
        }
        assert!(board.nothing_is_validated_while_it_sits());
    }

    #[test]
    fn a_drop_does_not_clear_the_pasteboard_and_a_period_ending_does() {
        assert_eq!(
            what_a_drop_does_to_the_pasteboard(),
            WhatBecomesOfThePasteboard::ItStands
        );
        assert_eq!(
            what_a_period_ending_does_to_the_pasteboard(),
            WhatBecomesOfThePasteboard::ItGoesWithThePeriod
        );
    }

    #[test]
    fn a_paste_does_not_clear_the_board_whichever_act_it_was() {
        for act in [CutOrCopy::Cut, CutOrCopy::Copy] {
            let (board, _, _) = a_pasteboard(act);
            assert!(board.a_paste_does_not_clear_it());
        }
    }

    #[test]
    fn a_delete_inside_the_reserved_folder_is_transfer_aware() {
        assert!(a_delete_target_is_staging_material(
            b"/jobs/.smoozy-staging/abc-1.smoozy-part"
        ));
        assert!(!a_delete_target_is_staging_material(b"/jobs/a.png"));
    }

    #[test]
    fn the_five_acts_this_section_performs_are_the_five_the_diary_declares() {
        // **B3's closed set of five is this section's own five**, and the
        // declaration is only true while the two agree. A sixth act here would
        // be a diary line nobody declared, which `Desktop app and delivery` A7
        // makes write nothing, silently.
        use crate::diary::FileAct;
        let declared: Vec<&str> = crate::declarations::THE_PARTITION_UNITS
            .iter()
            .map(|row| row.action)
            .filter(|action| {
                [
                    "delete",
                    "rename",
                    "move, same filesystem",
                    "move, across filesystems",
                    "copy",
                    "new folder",
                ]
                .contains(action)
            })
            .collect();
        assert_eq!(
            declared.len(),
            6,
            "the two moves are one act with two units"
        );
        assert_eq!(FileAct::ALL.len(), 5);
        // A move and a copy carry a destination on the line; the other three do
        // not, which is the one place the five differ from each other.
        assert!(FileAct::Move.carries_a_destination());
        assert!(FileAct::Copy.carries_a_destination());
        for act in [FileAct::Delete, FileAct::Rename, FileAct::NewFolder] {
            assert!(!act.carries_a_destination());
        }
    }

    /// The neighbouring records written by hand, for §7's two shapes — the
    /// removals that owe W11's mark alone, and the publications that owe X7a's
    /// two beside it. **Every reading is of the desktop's own records**, so
    /// this is testable at the desk with no machine anywhere.
    #[derive(Default)]
    struct TheRecordsNextDoor {
        standings: Vec<Vec<u8>>,
        readings: Vec<Vec<u8>>,
        trees: Vec<(Vec<u8>, Vec<u8>)>,
        marked: Vec<Vec<u8>>,
        dropped: Vec<Vec<u8>>,
        disturbed: Vec<Vec<u8>>,
    }

    impl crate::publication::TheNeighbouringRecords for TheRecordsNextDoor {
        fn the_check_standings_root_at(&self, published: &[u8]) -> Option<Vec<u8>> {
            self.standings
                .iter()
                .find(|root| paths_are_equal(root, published))
                .cloned()
        }
        fn the_recorded_readings_path_at(&self, published: &[u8]) -> Option<Vec<u8>> {
            self.readings
                .iter()
                .find(|path| paths_are_equal(path, published))
                .cloned()
        }
        fn mark_the_standing_replaced_since_check(&mut self, root: &[u8]) {
            self.marked.push(root.to_vec());
        }
        fn drop_the_recorded_reading(&mut self, path: &[u8]) {
            self.dropped.push(path.to_vec());
        }
        fn the_roots_whose_tree_resolved(
            &self,
            location: &[u8],
        ) -> Vec<crate::publication::ARootWhoseTreeResolvedIt> {
            self.trees
                .iter()
                .filter(|(at, _)| paths_are_equal(at, location))
                .map(|(at, root)| crate::publication::ARootWhoseTreeResolvedIt {
                    the_location_as_the_record_holds_it: at.clone(),
                    root: root.clone(),
                })
                .collect()
        }
        fn the_fixed_copies_made_from(
            &self,
            _location: &[u8],
        ) -> Vec<crate::publication::AFixedCopyMadeFromIt> {
            Vec::new()
        }
        fn mark_the_standing_disturbed(&mut self, root: &[u8]) {
            self.disturbed.push(root.to_vec());
        }
    }

    const A_TEXTURE: &[u8] = b"/root/assets/files/Users/a/tex/x.png";
    const A_CHECKED_ROOT: &[u8] = b"/root/assets/files/Users/a/scene.blend";

    /// **W11's removal half: a delete of something a checked root's tree
    /// resolved marks that root's standing disturbed.**
    #[test]
    fn a_delete_of_something_a_checked_roots_tree_resolved_marks_that_standing() {
        let mut records = TheRecordsNextDoor {
            trees: vec![(A_TEXTURE.to_vec(), A_CHECKED_ROOT.to_vec())],
            ..TheRecordsNextDoor::default()
        };
        let wrote = at_this_unit_of_a_managing_act(
            A_TEXTURE,
            WhatTheUnitDid::RemovedIt,
            Some(&UnitOutcome::Done),
            &mut records,
        );
        assert_eq!(
            wrote.the_standings_marked_disturbed,
            vec![A_CHECKED_ROOT.to_vec()]
        );
        assert_eq!(records.disturbed, vec![A_CHECKED_ROOT.to_vec()]);

        // A removal somewhere else marks nothing.
        let mut elsewhere = TheRecordsNextDoor {
            trees: vec![(A_TEXTURE.to_vec(), A_CHECKED_ROOT.to_vec())],
            ..TheRecordsNextDoor::default()
        };
        assert!(at_this_unit_of_a_managing_act(
            b"/root/elsewhere.png",
            WhatTheUnitDid::RemovedIt,
            Some(&UnitOutcome::Done),
            &mut elsewhere
        )
        .is_nothing());
        assert!(elsewhere.disturbed.is_empty());
    }

    /// **X7a after its widening of 2026-09-05: a copy's publication over a
    /// check standing's own root marks it replaced-since-check and drops the
    /// recorded reading — and a delete over the very same path does neither,
    /// while still writing W11's disturbance mark.**
    ///
    /// This is the case that used to write nothing at all: X7a's opening named
    /// *an upload's* publication, so a copy answering **Replace** over a
    /// checked `scene.blend` marked no standing, `Rendering` W2 fired no
    /// submission warning, and the render was paid for against a file the check
    /// no longer described. **W11 did not cover it** — it marks the standings
    /// whose *tree* resolved the location, and a root is not a member of its own
    /// tree, which is what the third assertion below says.
    #[test]
    fn a_copy_publishing_over_a_standings_root_owes_three_writes_and_a_delete_owes_one() {
        let records_holding = || TheRecordsNextDoor {
            standings: vec![A_CHECKED_ROOT.to_vec()],
            readings: vec![A_CHECKED_ROOT.to_vec()],
            ..TheRecordsNextDoor::default()
        };

        let mut published = records_holding();
        let wrote = at_this_unit_of_a_managing_act(
            A_CHECKED_ROOT,
            WhatTheUnitDid::PublishedOverIt,
            Some(&UnitOutcome::Done),
            &mut published,
        );
        assert_eq!(
            wrote.the_standing_marked_replaced_since_check,
            Some(A_CHECKED_ROOT.to_vec())
        );
        assert_eq!(
            wrote.the_recorded_reading_dropped,
            Some(A_CHECKED_ROOT.to_vec())
        );
        assert_eq!(published.marked, vec![A_CHECKED_ROOT.to_vec()]);
        assert_eq!(published.dropped, vec![A_CHECKED_ROOT.to_vec()]);

        let mut removed = records_holding();
        let wrote = at_this_unit_of_a_managing_act(
            A_CHECKED_ROOT,
            WhatTheUnitDid::RemovedIt,
            Some(&UnitOutcome::Done),
            &mut removed,
        );
        assert!(
            wrote.the_standing_marked_replaced_since_check.is_none(),
            "a removal is not a publication and owes neither of X7a's two writes"
        );
        assert!(wrote.the_recorded_reading_dropped.is_none());
        assert!(removed.marked.is_empty() && removed.dropped.is_empty());

        // **And the removal still writes W11's mark**, which is the write the
        // two shapes share. A root is not a member of its own tree, so the
        // standing reached here is a second root whose tree resolved the file.
        let mut removed = TheRecordsNextDoor {
            trees: vec![(A_CHECKED_ROOT.to_vec(), b"/root/other.blend".to_vec())],
            ..records_holding()
        };
        let wrote = at_this_unit_of_a_managing_act(
            A_CHECKED_ROOT,
            WhatTheUnitDid::RemovedIt,
            Some(&UnitOutcome::Done),
            &mut removed,
        );
        assert_eq!(
            wrote.the_standings_marked_disturbed,
            vec![b"/root/other.blend".to_vec()]
        );
    }

    /// **W11 marks only for a unit that took effect**, which is W9's rule taken
    /// for the same reason — and the cross-filesystem pair's delete that was
    /// not attempted because the copy did not publish is exactly such a unit.
    #[test]
    fn a_unit_that_did_not_take_effect_marks_nothing() {
        let refused = what_the_second_half_does(false);
        let outcome = the_delete_halfs_outcome(&refused);
        for did_not in [outcome, Some(UnitOutcome::Failed), None] {
            let mut records = TheRecordsNextDoor {
                trees: vec![(A_TEXTURE.to_vec(), A_CHECKED_ROOT.to_vec())],
                ..TheRecordsNextDoor::default()
            };
            for did in [WhatTheUnitDid::RemovedIt, WhatTheUnitDid::PublishedOverIt] {
                assert!(at_this_unit_of_a_managing_act(
                    A_TEXTURE,
                    did,
                    did_not.as_ref(),
                    &mut records
                )
                .is_nothing());
            }
            assert!(
                records.disturbed.is_empty(),
                "a standing was marked over a file that never moved"
            );
        }
    }

    /// **W11's acts, as widened 2026-10-02**: a rename and a same-filesystem
    /// move owe the mark at the place they left (§13 item 20, closed), and
    /// write it alone — no publication was made there for X7a's two writes.
    #[test]
    fn a_rename_and_a_move_away_mark_the_place_they_left_and_nothing_else() {
        assert_eq!(
            THE_ACTS_OF_THIS_DOCUMENT_THAT_OWE_THE_DISTURBANCE_MARK.len(),
            7
        );
        assert!(THE_ACTS_OF_THIS_DOCUMENT_THAT_OWE_THE_DISTURBANCE_MARK.contains(&"a delete"));
        assert!(THE_ACTS_OF_THIS_DOCUMENT_THAT_OWE_THE_DISTURBANCE_MARK
            .contains(&"a rename, at the place it moved away from"));
        const { assert!(A_RENAME_AND_A_MOVE_AWAY_MARK_THE_PLACE_THEY_LEFT) };
        let mut records = TheRecordsNextDoor {
            trees: vec![(A_TEXTURE.to_vec(), A_CHECKED_ROOT.to_vec())],
            standings: vec![A_TEXTURE.to_vec()],
            readings: vec![A_TEXTURE.to_vec()],
            ..TheRecordsNextDoor::default()
        };
        let wrote = at_this_unit_of_a_managing_act(
            A_TEXTURE,
            WhatTheUnitDid::MovedAwayFromIt,
            Some(&UnitOutcome::Done),
            &mut records,
        );
        assert_eq!(wrote.the_standings_marked_disturbed, vec![A_CHECKED_ROOT.to_vec()]);
        assert!(wrote.the_standing_marked_replaced_since_check.is_none());
        assert!(wrote.the_recorded_reading_dropped.is_none());
        // A rename the machine refused moved nothing.
        let mut refused = TheRecordsNextDoor {
            trees: vec![(A_TEXTURE.to_vec(), A_CHECKED_ROOT.to_vec())],
            ..TheRecordsNextDoor::default()
        };
        assert!(at_this_unit_of_a_managing_act(
            A_TEXTURE,
            WhatTheUnitDid::MovedAwayFromIt,
            Some(&UnitOutcome::Failed),
            &mut refused
        )
        .is_nothing());
    }

    /// **M9's names refused before any attempt**, and nothing else.
    #[test]
    fn a_typed_name_is_refused_only_for_the_five_reasons() {
        assert_eq!(a_typed_name_is_refused(b""), Some(WhyTheNameIsRefused::Empty));
        assert_eq!(a_typed_name_is_refused(b"a/b"), Some(WhyTheNameIsRefused::HoldsASlash));
        assert_eq!(a_typed_name_is_refused(b"."), Some(WhyTheNameIsRefused::IsADotName));
        assert_eq!(a_typed_name_is_refused(b".."), Some(WhyTheNameIsRefused::IsADotName));
        assert_eq!(a_typed_name_is_refused(&[b'a'; 256]), Some(WhyTheNameIsRefused::TooLong));
        assert_eq!(a_typed_name_is_refused(&[b'a'; 255]), None);
        assert_eq!(
            a_typed_name_is_refused(b".smoozy-staging"),
            Some(WhyTheNameIsRefused::IsTheStagingFoldersName)
        );
        for fine in [&b"New folder"[..], b"...", b".hidden", b"caf\xc3\xa9", b"a b"] {
            assert_eq!(a_typed_name_is_refused(fine), None, "{fine:?}");
        }
    }

    #[test]
    fn the_new_folder_is_the_plain_form() {
        const { assert!(A_NEW_FOLDER_IS_THE_PLAIN_FORM_AND_NEVER_THE_MAKE_PARENTS_ONE) };
        // **The trait offers no make-parents operation to reach for**, which is
        // the whole of how N3's rule is kept for the user's own act. The check
        // is the source itself: nothing in this file names one.
        let source = include_str!("managing.rs");
        let make_parents = ["make", "parents"].join("_");
        assert!(!source.contains(&format!("fn {make_parents}")));
        assert!(source.contains("fn make_one_directory"));
    }
}
