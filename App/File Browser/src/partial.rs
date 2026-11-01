//! The partial, never counted — `File Browser` §4.4, X10 to X13, Row 197 and
//! Row 191's exclusion.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Server protocol and transfers` 7.1 clause 1, C11, S1, S4,
//! S5, S7, S9, S12, S15, S26, S27, S28.

use smoozy_server_protocol::transfers::{
    Ending, StableFileIdentity, StagingPath, StoppedReason, TransferRecord,
};
use smoozy_state::identities::Identity;

use crate::already_exists::{components, paths_are_equal};

/// **X10: a partial this architecture produces cannot be at a final name, so
/// the exclusion Row 191 needs holds by construction.**
///
/// `Server protocol and transfers` 7.1 clause 1: **every byte of every upload
/// sits at a staging path until publication, and the final name holds the
/// previous file or nothing.** The walk of X1 reads final names out of
/// listings; **a staging path is never a landing place and never compared as
/// one.**
///
/// The 40,000,000-byte hybrid — Fact 18's observed worst — **is a thing this
/// architecture cannot create**, because no write of its ever touches a final
/// name.
pub const A_STAGING_PATH_IS_NEVER_COMPARED_AS_A_LANDING_PLACE: bool = true;

/// **X11: the record that says which path on which machine is a partial, and
/// how far it reached, is `Server protocol and transfers` S9's transfer
/// record** — durable on the desktop, written before the first byte, readable
/// in a later sitting and after a restart.
///
/// **Nothing the machine can be asked distinguishes a partial from a whole
/// file** (Fact 18), and **a marker written on the machine at the moment things
/// go wrong is exactly what a full disk refuses** (Fact 28) — so the record is
/// where S7 already put it, **and this document adds no second one.**
pub const THIS_DOCUMENT_ADDS_NO_SECOND_RECORD_OF_A_PARTIAL: bool = true;

/// **X12's reserved folder, and the name is declared here.**
///
/// `Server protocol and transfers` S1 places staging *"on the same filesystem
/// as the final name, in a Smoozy-named directory beside it"* and leaves the
/// name to its caller; S4 and S5 name the object inside it. **X12 is where the
/// folder is met by a user**, so the name is fixed here, once, and the two
/// directions take it from the same constant rather than each choosing.
///
/// **X12 as amended 2026-10-02, out of Row 520: its name starts with a period,
/// so it is listed under L19 to L21 like any other hidden name** — left out of
/// the list while *Show hidden files* is off, and listed as a **reserved Smoozy
/// staging folder**, not as an ordinary user folder, while it is on. *(Until
/// then this said it is not hidden, because hiding it would make the listing
/// lie; X12 struck that reason.)* **Everything else in X12 holds in both
/// states**, and the user's told-about-it channel is U8's leftover
/// presentation, which the choice does not reach.
pub const THE_RESERVED_STAGING_FOLDER: &str = ".smoozy-staging";

/// **X12's last sentence: an arriving name that would land on the staging
/// directory's own name is refused in the pre-flight and named truthfully**,
/// rather than merged into the staging area and swept by S28.
pub fn would_land_on_the_reserved_folder(landing_path: &[u8]) -> bool {
    components(landing_path)
        .last()
        .is_some_and(|last| *last == THE_RESERVED_STAGING_FOLDER.as_bytes())
}

/// Whether a path is inside a reserved staging folder.
pub fn is_staging_material(path: &[u8]) -> bool {
    components(path)
        .iter()
        .any(|component| *component == THE_RESERVED_STAGING_FOLDER.as_bytes())
}

/// **X12: a delete aimed at the staging folder or anything inside it is
/// transfer-aware, and this is what it resolves to.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheTargetIs {
    /// **An exact live or ended transfer record owns it**, its synthesised
    /// staging name and its stable file identity both verified.
    OwnedBy {
        transfer: Identity,
        staging_path: StagingPath,
        /// Whether the transfer is still running. **An active transfer is
        /// stopped under the same ordering before its bytes can continue.**
        the_transfer_is_active: bool,
    },
    /// **A record or identity mismatch is not removed automatically and is
    /// named as an unowned staging entry.** The user may still remove that
    /// named orphan through a separate explicit delete, **but the act is never
    /// allowed to manufacture a successful publication outcome.**
    AnUnownedStagingEntry(WhyItIsUnowned),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyItIsUnowned {
    /// No record names this path at all.
    NoRecordNamesIt,
    /// A record names the path, and the object at it is not the object the
    /// record describes.
    TheIdentityDoesNotMatch,
    /// **`Server protocol and transfers` S9's honest absence.** The supplier
    /// could not answer for that attempt, so there is no identity to verify
    /// against — *"S28's automatic removal requires the recorded identity and
    /// does not run without it."*
    TheRecordedIdentityIsUnavailable,
}

/// **X12's resolution.** *"Before removing a path, File Browser resolves every
/// exact live or ended transfer record that owns it, verifies the synthesized
/// staging name and stable file identity, records* stopped: staging removed by
/// user *durably, then removes that object."*
///
/// `identity_now` is what the machine says the object at that path is; `None`
/// where the channel could not answer, which is §13 item 9's ground and is the
/// same reason S15 has an identity-unavailable branch.
pub fn resolve(
    target: &[u8],
    records: &[TransferRecord],
    identity_now: Option<&StableFileIdentity>,
) -> WhatTheTargetIs {
    for record in records {
        for (staging_path, recorded) in
            record.every_staging_path_with_the_identity_recorded_for_it()
        {
            if !paths_are_equal(&staging_path.as_bytes(), target) {
                continue;
            }
            let recorded = match recorded {
                Some(StableFileIdentity::The(recorded)) => recorded,
                // No identity recorded, or the supplier could not answer:
                // nothing to verify against, so nothing is removed
                // automatically.
                _ => {
                    return WhatTheTargetIs::AnUnownedStagingEntry(
                        WhyItIsUnowned::TheRecordedIdentityIsUnavailable,
                    )
                }
            };
            return match identity_now {
                Some(StableFileIdentity::The(now)) if *now == recorded => {
                    WhatTheTargetIs::OwnedBy {
                        transfer: record.transfer.clone(),
                        staging_path,
                        the_transfer_is_active: record.ending.is_none(),
                    }
                }
                _ => {
                    WhatTheTargetIs::AnUnownedStagingEntry(WhyItIsUnowned::TheIdentityDoesNotMatch)
                }
            };
        }
    }
    WhatTheTargetIs::AnUnownedStagingEntry(WhyItIsUnowned::NoRecordNamesIt)
}

/// **X12's ordering, and it is an ordering rather than a set.**
///
/// *"An active transfer is stopped under the same ordering before its bytes can
/// continue."* `Server protocol and transfers` C23 is why the record comes
/// first: *"a stop and a drop are never told apart from the wire, so the record
/// is written before the stop is issued"*, and a process that dies mid-stop
/// would otherwise leave material the next connection resumes as though the
/// user had never stopped it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemovalStep {
    /// **Durably, and first** — `Server protocol and transfers` S26's stopped
    /// row with [`StoppedReason::StagingRemovedByUser`].
    RecordStoppedStagingRemovedByUser,
    /// Where the transfer was still running.
    StopTheTransfer,
    /// **Only the exact object the record names.**
    RemoveThatObject,
}

/// The ordering, in order.
pub const THE_REMOVAL_ORDERING: [RemovalStep; 3] = [
    RemovalStep::RecordStoppedStagingRemovedByUser,
    RemovalStep::StopTheTransfer,
    RemovalStep::RemoveThatObject,
];

/// The ending a removal writes. **Stated as a function rather than a comment so
/// nobody writes *interrupted* here** — an *interrupted* record is the one
/// C11 step 4 resumes, and a staging the user has just removed would come back
/// as work waiting to be carried on.
pub fn the_ending_a_removal_writes() -> Ending {
    Ending::Stopped(StoppedReason::StagingRemovedByUser)
}

/// **X12: what the record does when the user explicitly removes a named
/// unowned staging entry.** *"The explicit removal of a named unowned staging
/// entry writes* stopped: staging removed by user *durably to every transfer
/// record whose staging path it exactly matches, before the object is removed,
/// under the same ordering the owned case already uses; where no record names
/// the path nothing is written."*
///
/// **On every route this build has, the owned branch is not the ordinary case
/// and a reader must not take it as one.**
/// [`crate::managing::TheMachinesFiles::stable_identity_of`] answers `None` over
/// the file-transfer subsystem — revision 3's attribute structure carries no
/// inode, so there is no route to the identity on that surface — which is
/// `File Browser` §13 item 9's ground and `Server protocol and transfers` S15's
/// identity-unavailable branch. That item is **permanently unresolved**, so this
/// is a standing condition rather than a pending answer: [`resolve`] can never
/// answer [`WhatTheTargetIs::OwnedBy`] here, **every removal is this one**, and
/// without it the durable half of X12's ordering would never run at all — U8's
/// leftover line composes from transfer records reading *interrupted*, so
/// material the user had removed would be offered back to them at every later
/// sitting.
///
/// **The stable file identity stays the gate on *automatic* removal** —
/// `Server protocol and transfers` S28's sweep and
/// [`the_directory_removal_may_run`] below — **and is not weakened by this.**
/// This is the user's own explicit act on an entry Smoozy has already named to
/// them as unowned.
///
/// **And the act still never manufactures a successful publication outcome**:
/// what it writes is [`the_ending_a_removal_writes`], which is a *stopped*
/// ending and nothing else.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnExplicitOrphanRemoval {
    /// **Every transfer record whose staging path the target exactly matches**,
    /// under the same byte comparison [`resolve`] uses. Empty where no record
    /// names the path, and then nothing is written.
    pub the_records_the_ending_is_written_to: Vec<Identity>,
    /// Whether any matched record is still running. **An active transfer is
    /// stopped under the same ordering before its bytes can continue.**
    pub a_matched_transfer_is_active: bool,
}

impl AnExplicitOrphanRemoval {
    /// The ordering this removal runs, and it is [`THE_REMOVAL_ORDERING`]'s own.
    ///
    /// **The first step is present exactly where there is a record to write
    /// it to**, which is the clause's *where no record names the path, nothing
    /// is written*.
    pub fn the_steps(&self) -> Vec<RemovalStep> {
        THE_REMOVAL_ORDERING
            .iter()
            .copied()
            .filter(|step| match step {
                RemovalStep::RecordStoppedStagingRemovedByUser => {
                    !self.the_records_the_ending_is_written_to.is_empty()
                }
                RemovalStep::StopTheTransfer => self.a_matched_transfer_is_active,
                RemovalStep::RemoveThatObject => true,
            })
            .collect()
    }
}

/// X12's added sentence, resolved over the records.
///
/// **It is only ever reached through the user's separate explicit delete of an
/// entry [`resolve`] answered [`WhatTheTargetIs::AnUnownedStagingEntry`] for.**
/// Nothing automatic calls it.
pub fn the_explicit_removal_of_a_named_orphan(
    target: &[u8],
    records: &[TransferRecord],
) -> AnExplicitOrphanRemoval {
    let mut the_records_the_ending_is_written_to = Vec::new();
    let mut a_matched_transfer_is_active = false;
    for record in records {
        let matches_this_record = record
            .every_staging_path_with_the_identity_recorded_for_it()
            .into_iter()
            .any(|(staging_path, _)| paths_are_equal(&staging_path.as_bytes(), target));
        if !matches_this_record {
            continue;
        }
        the_records_the_ending_is_written_to.push(record.transfer.clone());
        if record.ending.is_none() {
            a_matched_transfer_is_active = true;
        }
    }
    AnExplicitOrphanRemoval {
        the_records_the_ending_is_written_to,
        a_matched_transfer_is_active,
    }
}

/// **X12: deleting the whole reserved folder applies this sequence per owned
/// entry and refuses the directory removal while any entry is unresolved.**
///
/// **This is the automatic case and the added sentence does not reach it**: the
/// stable file identity stays the gate here, exactly as it stays the gate on
/// `Server protocol and transfers` S28's sweep.
pub fn the_directory_removal_may_run(entries: &[WhatTheTargetIs]) -> bool {
    entries
        .iter()
        .all(|entry| matches!(entry, WhatTheTargetIs::OwnedBy { .. }))
}

/// **X13: a partial left at a final name by something that is not this
/// architecture is an ordinary file here.**
///
/// The user's earlier tools, another program, a copy killed before Smoozy
/// existed — **such a file carries no record**, and the spec's own rule decides
/// it: **comparison is by name and place only, never contents**, so it is
/// counted as already being there like any file.
///
/// *"Smoozy's promise is that **Smoozy** never leaves a half-sent file looking
/// finished, and that promise this section keeps; it makes no claim about files
/// it never touched."*
pub const A_FILE_SMOOZY_NEVER_TOUCHED_IS_COUNTED_LIKE_ANY_OTHER: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_server_protocol::transfers::Direction;
    use smoozy_state::identities::IdentityKind;

    fn an_identity(kind: IdentityKind) -> Identity {
        Identity::issue(kind).expect("this machine supplies random bytes")
    }

    fn a_record() -> TransferRecord {
        TransferRecord::arranged(
            an_identity(IdentityKind::Transfer),
            Direction::Up,
            an_identity(IdentityKind::LocalServer),
            Some(an_identity(IdentityKind::Installation)),
            "/home/user/scene.blend".to_string(),
            "/jobs/scene.blend".to_string(),
            "/jobs/.smoozy-staging",
            4_096,
            an_identity(IdentityKind::Run),
        )
    }

    #[test]
    fn an_arriving_name_that_would_land_on_the_reserved_folder_is_caught_in_the_pre_flight() {
        assert!(would_land_on_the_reserved_folder(b"/jobs/.smoozy-staging"));
        assert!(!would_land_on_the_reserved_folder(b"/jobs/scene.blend"));
        // And something merely inside it is not the folder's own name.
        assert!(!would_land_on_the_reserved_folder(
            b"/jobs/.smoozy-staging/x"
        ));
        assert!(is_staging_material(b"/jobs/.smoozy-staging/x"));
        assert!(!is_staging_material(b"/jobs/scene.blend"));
    }

    #[test]
    fn an_object_whose_recorded_identity_matches_is_owned_and_the_removal_ordering_is_fixed() {
        let mut record = a_record();
        record.staging_identity = Some(StableFileIdentity::The("ino-77".to_string()));
        let target = record.staging_path.as_bytes();

        let resolved = resolve(
            &target,
            std::slice::from_ref(&record),
            Some(&StableFileIdentity::The("ino-77".to_string())),
        );
        match resolved {
            WhatTheTargetIs::OwnedBy {
                the_transfer_is_active,
                ..
            } => assert!(
                the_transfer_is_active,
                "a record with no ending is still running"
            ),
            other => panic!("{other:?}"),
        }

        assert_eq!(
            THE_REMOVAL_ORDERING[0],
            RemovalStep::RecordStoppedStagingRemovedByUser
        );
        assert_eq!(
            the_ending_a_removal_writes(),
            Ending::Stopped(StoppedReason::StagingRemovedByUser)
        );
        // And that ending is never one a repair carries on.
        assert!(!the_ending_a_removal_writes().may_the_work_be_carried_on());
    }

    #[test]
    fn an_identity_that_does_not_match_is_an_unowned_entry_and_is_not_removed_automatically() {
        let mut record = a_record();
        record.staging_identity = Some(StableFileIdentity::The("ino-77".to_string()));
        let target = record.staging_path.as_bytes();
        assert_eq!(
            resolve(
                &target,
                std::slice::from_ref(&record),
                Some(&StableFileIdentity::The("ino-88".to_string()))
            ),
            WhatTheTargetIs::AnUnownedStagingEntry(WhyItIsUnowned::TheIdentityDoesNotMatch)
        );
        // And a channel that could not answer is the same refusal.
        assert_eq!(
            resolve(&target, std::slice::from_ref(&record), None),
            WhatTheTargetIs::AnUnownedStagingEntry(WhyItIsUnowned::TheIdentityDoesNotMatch)
        );
    }

    #[test]
    fn a_record_that_never_recorded_an_identity_leaves_the_object_unowned() {
        let record = a_record();
        let target = record.staging_path.as_bytes();
        assert_eq!(
            resolve(
                &target,
                std::slice::from_ref(&record),
                Some(&StableFileIdentity::The("ino-77".to_string()))
            ),
            WhatTheTargetIs::AnUnownedStagingEntry(
                WhyItIsUnowned::TheRecordedIdentityIsUnavailable
            )
        );
    }

    #[test]
    fn a_supplier_that_could_not_answer_is_recorded_as_a_fact_and_still_blocks_removal() {
        let mut record = a_record();
        record.staging_identity = Some(StableFileIdentity::TheSupplierCouldNotAnswer);
        let target = record.staging_path.as_bytes();
        assert_eq!(
            resolve(
                &target,
                std::slice::from_ref(&record),
                Some(&StableFileIdentity::The("ino-77".to_string()))
            ),
            WhatTheTargetIs::AnUnownedStagingEntry(
                WhyItIsUnowned::TheRecordedIdentityIsUnavailable
            )
        );
    }

    #[test]
    fn a_path_no_record_names_is_an_orphan_rather_than_something_to_sweep() {
        let record = a_record();
        assert_eq!(
            resolve(
                b"/jobs/.smoozy-staging/left-by-something-else",
                std::slice::from_ref(&record),
                None
            ),
            WhatTheTargetIs::AnUnownedStagingEntry(WhyItIsUnowned::NoRecordNamesIt)
        );
    }

    #[test]
    fn a_superseded_attempts_object_is_owned_too() {
        // `Server protocol and transfers` S9 keeps the identity beside each
        // retained attempt number, which is what lets a superseded path be
        // reached at all.
        let mut record = a_record();
        record.staging_identity = Some(StableFileIdentity::The("ino-1".to_string()));
        let first = record.staging_path.clone();
        record.start_over();
        record.staging_identity = Some(StableFileIdentity::The("ino-2".to_string()));

        assert_eq!(
            resolve(
                &first.as_bytes(),
                std::slice::from_ref(&record),
                Some(&StableFileIdentity::The("ino-1".to_string()))
            ),
            WhatTheTargetIs::OwnedBy {
                transfer: record.transfer.clone(),
                staging_path: first,
                the_transfer_is_active: true,
            }
        );
    }

    #[test]
    fn a_named_orphan_the_user_removed_leaves_the_record_stopped_rather_than_interrupted() {
        // The ordinary case on every route this build has: the channel answers
        // no identity, so the entry is named to the user as unowned and they
        // remove it themselves.
        let mut record = a_record();
        record.ending = Some(Ending::Interrupted);
        let target = record.staging_path.as_bytes();
        assert_eq!(
            resolve(&target, std::slice::from_ref(&record), None),
            WhatTheTargetIs::AnUnownedStagingEntry(
                WhyItIsUnowned::TheRecordedIdentityIsUnavailable
            )
        );

        let removal =
            the_explicit_removal_of_a_named_orphan(&target, std::slice::from_ref(&record));
        assert_eq!(
            removal.the_records_the_ending_is_written_to,
            vec![record.transfer.clone()],
            "the record whose staging path this exactly matches is written to"
        );
        // **The ending is written, and it is written first.**
        assert_eq!(
            removal.the_steps()[0],
            RemovalStep::RecordStoppedStagingRemovedByUser
        );
        assert_eq!(
            *removal.the_steps().last().expect("the object is removed"),
            RemovalStep::RemoveThatObject
        );

        // And what is written is *stopped: staging removed by user* — never
        // *interrupted*, which is the reading U8 composes a leftover line from.
        record.ending = Some(the_ending_a_removal_writes());
        assert_eq!(
            record.ending,
            Some(Ending::Stopped(StoppedReason::StagingRemovedByUser))
        );
        assert_ne!(record.ending, Some(Ending::Interrupted));
        // So the same material is not offered back at the next sitting.
        assert!(!record
            .ending
            .as_ref()
            .expect("an ending was written")
            .may_the_work_be_carried_on());
    }

    #[test]
    fn a_superseded_attempts_path_and_an_active_transfer_are_both_carried_by_the_explicit_removal()
    {
        let mut record = a_record();
        let first = record.staging_path.clone();
        record.start_over();

        let removal = the_explicit_removal_of_a_named_orphan(
            &first.as_bytes(),
            std::slice::from_ref(&record),
        );
        assert_eq!(
            removal.the_records_the_ending_is_written_to,
            vec![record.transfer.clone()]
        );
        assert!(removal.a_matched_transfer_is_active);
        assert_eq!(removal.the_steps(), THE_REMOVAL_ORDERING.to_vec());
    }

    #[test]
    fn where_no_record_names_the_path_nothing_is_written_and_the_object_is_still_removed() {
        let record = a_record();
        let removal = the_explicit_removal_of_a_named_orphan(
            b"/jobs/.smoozy-staging/left-by-something-else",
            std::slice::from_ref(&record),
        );
        assert!(removal.the_records_the_ending_is_written_to.is_empty());
        assert!(!removal.a_matched_transfer_is_active);
        assert_eq!(removal.the_steps(), vec![RemovalStep::RemoveThatObject]);
    }

    #[test]
    fn automatic_removal_still_does_not_run_without_the_recorded_identity() {
        // The added sentence is the user's own explicit act and reaches nothing
        // automatic: the folder-wide delete still refuses over an entry whose
        // recorded identity is unavailable, which is the same gate `Server
        // protocol and transfers` S28's sweep keeps.
        let record = a_record();
        let target = record.staging_path.as_bytes();
        let entry = resolve(&target, std::slice::from_ref(&record), None);
        assert_eq!(
            entry,
            WhatTheTargetIs::AnUnownedStagingEntry(
                WhyItIsUnowned::TheRecordedIdentityIsUnavailable
            )
        );
        assert!(!the_directory_removal_may_run(std::slice::from_ref(&entry)));
    }

    #[test]
    fn the_whole_folder_refuses_while_any_entry_is_unresolved() {
        let mut record = a_record();
        record.staging_identity = Some(StableFileIdentity::The("ino-77".to_string()));
        let owned = WhatTheTargetIs::OwnedBy {
            transfer: record.transfer.clone(),
            staging_path: record.staging_path.clone(),
            the_transfer_is_active: false,
        };
        assert!(the_directory_removal_may_run(std::slice::from_ref(&owned)));
        assert!(!the_directory_removal_may_run(&[
            owned,
            WhatTheTargetIs::AnUnownedStagingEntry(WhyItIsUnowned::NoRecordNamesIt),
        ]));
        assert!(the_directory_removal_may_run(&[]));
    }
}
