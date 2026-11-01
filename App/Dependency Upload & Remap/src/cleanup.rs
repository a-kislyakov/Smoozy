//! Cleanup and lifetimes — `Dependency Upload & Remap` §8: U1–U4.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `State, identity and recovery` §11.4. `Job files
//! and outputs` Section 11.

use smoozy_state::lifetimes::Lifetime;

use crate::bytes::MachineLocation;
use crate::record::{lifetime_of, MachineKey, RecordKind, TheRecord};

/// **U1: Smoozy never removes an asset, a fixed copy, a mirror directory or a
/// job's remap material from a machine by itself — no sweep, no age, no cap,
/// no free-space trigger.** The spec's rule (Creator-stated 2026-07-28) and
/// `State, identity and recovery` §11.4's — nothing on a machine is cleared
/// on the strength of a free-space reading alone — meet in the same place.
/// The user deletes through `File Browser`, warned per V3.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NeverRemovedBySmoozyItself {
    AnAsset,
    AFixedCopy,
    AMirrorDirectory,
    AJobsRemapMaterial,
}

impl NeverRemovedBySmoozyItself {
    pub const ALL: [NeverRemovedBySmoozyItself; 4] = [
        NeverRemovedBySmoozyItself::AnAsset,
        NeverRemovedBySmoozyItself::AFixedCopy,
        NeverRemovedBySmoozyItself::AMirrorDirectory,
        NeverRemovedBySmoozyItself::AJobsRemapMaterial,
    ];

    /// **The only act that ever removes one is the user's own delete, or
    /// U2's for the remap material.** No sweep, no age, no cap, no free-space
    /// trigger.
    pub fn is_removed_by_a_sweep_an_age_a_cap_or_a_free_space_trigger(self) -> bool {
        false
    }
}

/// **The remap material's location, at the path it has always been reached at.** It lives in
/// `smoozy-dependencies-machine`, because `Queue & History` removes it with a
/// job's machine-side record and that removal is the machine's too. Split at
/// step 10c piece 3.
pub use smoozy_dependencies_machine::cleanup::what_goes_with_the_jobs_machine_side_record;

/// **U3: local lifetimes are K10's and nothing here invents one.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalThing {
    ARecord(RecordKind),
    /// F1's searched-folder list.
    TheSearchedFolderList,
    /// F7's per-folder search outcome.
    ASearchResult,
    /// Staged transfer material, under the contract.
    StagedTransferMaterial,
}

pub fn lifetime_of_the_local_thing(thing: LocalThing) -> Lifetime {
    match thing {
        LocalThing::ARecord(kind) => lifetime_of(kind),
        LocalThing::TheSearchedFolderList => Lifetime::UserHeld,
        LocalThing::ASearchResult => Lifetime::RunHeld,
        LocalThing::StagedTransferMaterial => Lifetime::ActionHeld,
    }
}

/// **U4: what a Smoozy-performed delete of a machine-side asset changes in
/// the record — placement records and standings — and what it leaves alone —
/// the located answers.** The user's decisions describe their own computer
/// and outlive any machine's contents (K10); what the delete changes is
/// exactly what makes the next check offer to send the file again rather
/// than believe it still up.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatADeleteChanges {
    /// The placement records naming the deleted location, which go.
    pub placement_records_that_go: Vec<MachineLocation>,
    /// The fixing records whose fixed copy is the deleted location, which go.
    pub fixing_records_that_go: Vec<MachineLocation>,
    /// **Always zero**: no located answer is touched by a delete.
    pub located_answers_touched: usize,
}

pub fn what_a_delete_changes(
    location: &MachineLocation,
    machine: &MachineKey,
    record: &dyn TheRecord,
) -> WhatADeleteChanges {
    WhatADeleteChanges {
        placement_records_that_go: record
            .placements_on(machine)
            .into_iter()
            .filter(|placement| &placement.mirrored_location == location)
            .map(|placement| placement.mirrored_location)
            .collect(),
        fixing_records_that_go: record
            .fixings_on(machine)
            .into_iter()
            .filter(|fixing| &fixing.fixed_copy == location)
            .map(|fixing| fixing.fixed_copy)
            .collect(),
        located_answers_touched: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::landing::tests::a_layout;
    use crate::record::tests::{a_machine, a_publication};
    use crate::record::{InMemoryRecord, PlacementRecord};
    use crate::remap::RemapMaterial;
    use smoozy_state::identities::{Identity, IdentityKind};

    #[test]
    fn nothing_on_a_machine_is_removed_by_smoozy_itself() {
        for thing in NeverRemovedBySmoozyItself::ALL {
            assert!(!thing.is_removed_by_a_sweep_an_age_a_cap_or_a_free_space_trigger());
        }
    }

    #[test]
    fn the_remap_material_goes_with_the_jobs_machine_side_record() {
        let job = Identity::issue(IdentityKind::Job).expect("randomness");
        assert_eq!(
            what_goes_with_the_jobs_machine_side_record(&a_layout(), &job),
            RemapMaterial::location_in(&a_layout(), &job)
        );
    }

    #[test]
    fn local_lifetimes_are_k10s() {
        assert_eq!(
            lifetime_of_the_local_thing(LocalThing::ARecord(RecordKind::Placement)),
            Lifetime::SubjectHeld
        );
        assert_eq!(
            lifetime_of_the_local_thing(LocalThing::TheSearchedFolderList),
            Lifetime::UserHeld
        );
        assert_eq!(
            lifetime_of_the_local_thing(LocalThing::ASearchResult),
            Lifetime::RunHeld
        );
        assert_eq!(
            lifetime_of_the_local_thing(LocalThing::StagedTransferMaterial),
            Lifetime::ActionHeld
        );
    }

    #[test]
    fn a_delete_changes_placements_and_leaves_every_located_answer_alone() {
        let machine = a_machine();
        let at = MachineLocation::composed(b"/root/assets/files/Users/a/x.png".to_vec());
        let mut record = InMemoryRecord::default();
        record.placements.push(PlacementRecord::at_publication(
            machine.clone(),
            a_publication(at.as_bytes(), b"/Users/a/x.png", b"red"),
        ));
        let changes = what_a_delete_changes(&at, &machine, &record);
        assert_eq!(changes.placement_records_that_go, vec![at]);
        assert!(changes.fixing_records_that_go.is_empty());
        assert_eq!(changes.located_answers_touched, 0);
    }
}
