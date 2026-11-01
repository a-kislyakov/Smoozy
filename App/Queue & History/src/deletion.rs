//! Deletion — `Queue & History` §14 X1, X4, X5, X6, X6a and X6b; X7's check
//! recorded.
//!
//! **Constraint:** Row 82. `State, identity and recovery` §10.1's tombstone
//! is built and is not re-invented ([`smoozy_state::tombstones`]); the
//! spec's own rule — *deleting history destroys the record, not the renders*
//! — bounds what the erase may reach.
//!
//! **What is here**: the local act's shape, what a disconnected deletion
//! leaves, the refusal that keeps a deleted entry from coming back, and the
//! three clearing acts with the figures their boxes draw. **What is the acting
//! half's**: X2's enumerated erase and X3's outcome record, run at the next
//! connection to a machine holding the identity.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** None.
//!
//! **`Download` R3 is written out at its site**, this plan defining an R3 of
//! its own.

use std::time::SystemTime;

use smoozy_state::identities::Identity;
use smoozy_state::tombstones::{HistoryTombstone, HistoryTombstones, MachineSideCopy};

use crate::record::LocalJobRecord;

/// **X1: deleting an entry removes the local record and writes the tombstone
/// in one atomic local act.** Removing the record removes everything
/// subject-held on it — the fetched derivatives and the arrangement record
/// (`Download` R3) among them. **The two writes are one unit** (`State,
/// identity and recovery` §9.3): a record cannot be gone with no tombstone,
/// and a tombstone cannot exist for a record still standing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ADeletion {
    pub remove_the_record_of: Identity,
    pub write_the_tombstone: HistoryTombstone,
}

impl ADeletion {
    /// Composed from the entry alone. **The tombstone is keyed by the job
    /// identity, holds the deletion moment on the desktop's clock, and
    /// starts with the machine-side copy still owed** — `State, identity and
    /// recovery` §10.1's own shape.
    pub fn of(record: &LocalJobRecord, now_on_this_computer: SystemTime) -> ADeletion {
        ADeletion {
            remove_the_record_of: record.job.clone(),
            write_the_tombstone: HistoryTombstone::for_a_deleted_entry(
                record.job.clone(),
                now_on_this_computer,
            ),
        }
    }

    /// The two halves name one identity.
    pub fn is_one_unit(&self) -> bool {
        self.remove_the_record_of == self.write_the_tombstone.job
    }
}

/// **X4: a deletion made while disconnected is remembered as a deletion, and
/// only as one.** The local record goes at once; the tombstone is the
/// something that survives — one identity in size, meeting no user surface.
/// The next connection to a machine holding that identity runs the erase as
/// a recorded debt. **The act reads no server row and needs no machine.**
pub fn a_deletion_made_disconnected(record: &LocalJobRecord, now: SystemTime) -> ADeletion {
    let deletion = ADeletion::of(record, now);
    debug_assert_eq!(
        deletion.write_the_tombstone.machine_side_copy,
        MachineSideCopy::StillOwed
    );
    deletion
}

/// **X5: filling in never brings a deleted entry back, and neither does
/// adoption.** Step 4 is refused unconditionally by the tombstone; step 6
/// never offers a tombstoned identity. **There is no third route by which a
/// machine's holdings become an entry.**
pub fn may_become_an_entry_again(tombstones: &dyn HistoryTombstones, job: &Identity) -> bool {
    tombstones.may_be_filled_in(job)
}

/// **One entry the history list draws, which is a local job record and the
/// one fact that decides whether the list draws it at all.**
///
/// **A local job record exists from submission** — C9's sentence, not C10's —
/// so the record alone does not say the job is in history, and handing the
/// bare record in wrote tombstones over live jobs and counted them in both
/// boxes' figures. `crate::two_records::is_in_history` takes a flag rather
/// than a record, so the test is folded in here rather than called as it
/// stands, and **each act below applies it itself rather than trusting its
/// caller.**
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AnEntryTheListDraws<'a> {
    pub record: &'a LocalJobRecord,
    /// **C9: a job is in history from the moment it starts rendering**, and
    /// the desktop learns that live or at `State, identity and recovery` §7
    /// step 3.
    pub the_render_start_has_been_learned: bool,
}

impl AnEntryTheListDraws<'_> {
    /// **C9's test, applied here and not left to the caller.**
    pub fn is_drawn(&self) -> bool {
        crate::two_records::is_in_history(self.the_render_start_has_been_learned)
    }
}

/// The scope of a clearing act (X6, X6a, X6b).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClearingScope {
    /// **X6: wiping a machine's history** — *for that machine* read off the
    /// copied local server identifier (N12).
    OneMachines(Identity),
    /// **X6a: the whole of history** — the spec's own third act,
    /// Creator-stated.
    TheWholeOfHistory,
    /// **X6b: the history of every server no longer in Smoozy** — the spec's
    /// fourth act, Creator-stated (the eighteenth notes, note 13). **The set is
    /// read at the act, with N17's lookup and nothing else**: an entry whose
    /// copied identifier is not among `the_rows_that_stand` is in it, and every
    /// entry whose server row stands is not. **It reaches no machine**: no row
    /// stands to connect through, and each tombstone's machine-side copy stays
    /// owed, as X6a's dangling-identifier bullet has it.
    TheServersNoLongerInSmoozy { the_rows_that_stand: Vec<Identity> },
}

impl ClearingScope {
    /// **Whether this scope covers one entry**, read off the entry's copied
    /// local server identifier (N12) alone.
    fn covers(&self, record: &LocalJobRecord) -> bool {
        match self {
            ClearingScope::OneMachines(server) => &record.server.local_server == server,
            ClearingScope::TheWholeOfHistory => true,
            ClearingScope::TheServersNoLongerInSmoozy { the_rows_that_stand } => {
                !the_rows_that_stand.contains(&record.server.local_server)
            }
        }
    }
}

/// **X6, X6a and X6b: the per-entry deletion over every entry the scope covers,
/// tombstones and all** — one user act, partial across entries (and, for the
/// whole of history or every removed server, across machines), atomic within each entry's
/// record-plus-tombstone unit. An interrupted clear leaves the remaining
/// entries standing and drawn, and the act is the user's to take again;
/// each deleted entry's tombstone and debt is complete on its own.
///
/// **It applies C9's in-history test itself.** A local job record exists from
/// submission, so a clearing act handed every local record would write a
/// tombstone over a job that is still waiting or still rendering — and a
/// tombstone refuses the fill-in for good.
pub fn a_clearing(
    entries: &[AnEntryTheListDraws<'_>],
    scope: &ClearingScope,
    now_on_this_computer: SystemTime,
) -> Vec<ADeletion> {
    entries
        .iter()
        .filter(|entry| entry.is_drawn())
        .filter(|entry| scope.covers(entry.record))
        .map(|entry| ADeletion::of(entry.record, now_on_this_computer))
        .collect()
}

/// **X6a: both boxes' figures are composed at the draw, over the entries the
/// list draws, and stored nowhere.** **Each act applies C9's in-history test
/// itself**, because a local job record exists from submission and a figure
/// over every local record counts jobs that are still in the queue. The
/// per-server box counts the
/// entries the list draws for the open entry's server; the all-history box
/// lists those entries grouped on the copied local server identifier, each
/// group drawn under the copied label and hue and counted by its entries.
/// **No server row is consulted, matched or reconstructed** — a dangling
/// identifier still keys its group and the copies are what draw it. An
/// entry predating N12's fields groups on the absent value and is stated
/// plainly. **Tombstones are counted in no figure.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AGroupInTheBox<'a> {
    pub local_server: &'a Identity,
    pub the_copied_label: Option<&'a str>,
    pub the_copied_hue: Option<crate::record::Hue>,
    pub entries: usize,
}

pub fn the_per_server_boxs_count(entries: &[AnEntryTheListDraws<'_>], server: &Identity) -> usize {
    entries
        .iter()
        .filter(|entry| entry.is_drawn() && &entry.record.server.local_server == server)
        .count()
}

pub fn the_all_history_boxs_groups<'a>(
    entries: &[AnEntryTheListDraws<'a>],
) -> Vec<AGroupInTheBox<'a>> {
    the_groups_a_box_lists(entries, &ClearingScope::TheWholeOfHistory)
}

/// **X6b: the removed servers' box lists each removed server under its copied
/// label with its own count**, composed at the draw over the same set the act
/// takes, grouped on the copied identifier and counted by entries — X6a's
/// all-history composition scoped to that set. Nothing is stored.
pub fn the_removed_servers_boxs_groups<'a>(
    entries: &[AnEntryTheListDraws<'a>],
    the_rows_that_stand: &[Identity],
) -> Vec<AGroupInTheBox<'a>> {
    the_groups_a_box_lists(
        entries,
        &ClearingScope::TheServersNoLongerInSmoozy { the_rows_that_stand: the_rows_that_stand.to_vec() },
    )
}

fn the_groups_a_box_lists<'a>(
    entries: &[AnEntryTheListDraws<'a>],
    scope: &ClearingScope,
) -> Vec<AGroupInTheBox<'a>> {
    let mut groups: Vec<AGroupInTheBox<'a>> = Vec::new();
    for entry in entries.iter().filter(|entry| entry.is_drawn() && scope.covers(entry.record)) {
        let record = entry.record;
        match groups
            .iter_mut()
            .find(|group| group.local_server == &record.server.local_server)
        {
            Some(group) => group.entries += 1,
            None => groups.push(AGroupInTheBox {
                local_server: &record.server.local_server,
                the_copied_label: record.server.label.as_deref(),
                the_copied_hue: record.server.hue,
                entries: 1,
            }),
        }
    }
    groups
}

/// **X2's own line, which neither clearing act touches**: no job output
/// folder, no render, and nothing on the user's own computer — the erase
/// each deletion owes reaches the record material X2 enumerates and never
/// the renders.
pub const NEITHER_ACT_TOUCHES_A_RENDER_OR_ANYTHING_ON_THE_USERS_COMPUTER: bool = true;

/// **X7: the residue was checked against the third escalation and the check
/// is recorded.** After a deletion with the erase complete, what remains on
/// a machine the user pays for is the job output folder — which the spec
/// names as staying, names in the delete confirmation as staying, and gives
/// the user their own routes to. Nothing survives a completed deletion that
/// the spec does not tell the user survives.
pub const NOTHING_SURVIVES_A_COMPLETED_DELETION_THE_SPEC_DOES_NOT_NAME: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::record::tests::{a_record, a_server};
    use crate::record::Hue;

    struct Tombstones(Vec<HistoryTombstone>);

    impl HistoryTombstones for Tombstones {
        fn tombstone_for(&self, job: &Identity) -> Option<&HistoryTombstone> {
            self.0.iter().find(|tombstone| &tombstone.job == job)
        }
    }

    #[test]
    fn a_deletion_is_the_record_and_the_tombstone_as_one_unit_with_the_erase_still_owed() {
        let server = a_server();
        let record = a_record(&server);
        let deletion = ADeletion::of(&record, SystemTime::UNIX_EPOCH);
        assert!(deletion.is_one_unit());
        assert_eq!(deletion.write_the_tombstone.job, record.job);
        assert_eq!(
            deletion.write_the_tombstone.machine_side_copy,
            MachineSideCopy::StillOwed
        );
        assert!(deletion.write_the_tombstone.refuses_filling_in());
    }

    #[test]
    fn a_deletion_made_disconnected_is_remembered_as_a_deletion_and_only_as_one() {
        let server = a_server();
        let record = a_record(&server);
        let deletion = a_deletion_made_disconnected(&record, SystemTime::UNIX_EPOCH);
        assert_eq!(
            deletion.write_the_tombstone.machine_side_copy,
            MachineSideCopy::StillOwed
        );
    }

    #[test]
    fn a_deleted_entry_never_comes_back_by_fill_in_or_by_adoption() {
        let server = a_server();
        let record = a_record(&server);
        let deletion = ADeletion::of(&record, SystemTime::UNIX_EPOCH);
        let mut removed = deletion.write_the_tombstone.clone();
        removed.machine_side_copy = MachineSideCopy::Removed;
        for tombstone in [deletion.write_the_tombstone.clone(), removed] {
            let tombstones = Tombstones(vec![tombstone]);
            assert!(!may_become_an_entry_again(&tombstones, &record.job));
        }
        assert!(may_become_an_entry_again(
            &Tombstones(Vec::new()),
            &record.job
        ));
    }

    fn drawn(record: &LocalJobRecord) -> AnEntryTheListDraws<'_> {
        AnEntryTheListDraws {
            record,
            the_render_start_has_been_learned: true,
        }
    }

    fn not_drawn(record: &LocalJobRecord) -> AnEntryTheListDraws<'_> {
        AnEntryTheListDraws {
            record,
            the_render_start_has_been_learned: false,
        }
    }

    #[test]
    fn clearing_one_machines_history_deletes_its_entries_alone_and_the_whole_deletes_every_one() {
        let a = a_server();
        let b = a_server();
        let records = [a_record(&a), a_record(&a), a_record(&b)];
        let entries: Vec<AnEntryTheListDraws<'_>> = records.iter().map(drawn).collect();
        let one = a_clearing(
            &entries,
            &ClearingScope::OneMachines(a.clone()),
            SystemTime::UNIX_EPOCH,
        );
        assert_eq!(one.len(), 2);
        assert!(one.iter().all(ADeletion::is_one_unit));
        let all = a_clearing(
            &entries,
            &ClearingScope::TheWholeOfHistory,
            SystemTime::UNIX_EPOCH,
        );
        assert_eq!(all.len(), 3);
    }

    /// **C9: a local job record exists from submission, so a clearing act
    /// applies the in-history test itself rather than trusting its caller.**
    ///
    /// **The fault this catches is a tombstone written over a live job** —
    /// and a tombstone refuses the fill-in unconditionally and for good, so
    /// the job could never become an entry again.
    #[test]
    fn a_clearing_never_writes_a_tombstone_over_a_job_that_is_not_in_history_yet() {
        let server = a_server();
        let started = a_record(&server);
        let submitted = a_record(&server);
        let entries = vec![drawn(&started), not_drawn(&submitted)];
        let deletions = a_clearing(
            &entries,
            &ClearingScope::TheWholeOfHistory,
            SystemTime::UNIX_EPOCH,
        );
        assert_eq!(deletions.len(), 1);
        assert_eq!(deletions[0].remove_the_record_of, started.job);
        assert_eq!(
            a_clearing(
                &entries,
                &ClearingScope::OneMachines(server.clone()),
                SystemTime::UNIX_EPOCH
            )
            .len(),
            1
        );
        // **X6a's figures are over the entries the list draws**, so the job
        // still in the queue is in neither box.
        assert_eq!(the_per_server_boxs_count(&entries, &server), 1);
        let groups = the_all_history_boxs_groups(&entries);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].entries, 1);
    }

    /// **X6b: clearing the removed servers' history takes every entry whose
    /// copied identifier resolves to no server row, and none whose row
    /// stands** — read at the act, over the local records alone; each deletion
    /// one record-plus-tombstone unit with the machine-side copy still owed,
    /// since no machine is reached; and the box's groups are over the same set.
    /// The eighteenth notes, piece 7, note 13.
    #[test]
    fn clearing_the_removed_servers_history_takes_the_entries_of_servers_no_longer_in_smoozy_alone() {
        let standing = a_server();
        let gone = a_server();
        let also_gone = a_server();
        let mut old = a_record(&also_gone);
        old.server.label = Some("Paperspace A6000".to_string());
        let records = [a_record(&standing), a_record(&gone), a_record(&gone), old];
        let mut entries: Vec<AnEntryTheListDraws<'_>> = records.iter().map(drawn).collect();
        let still_in_the_queue = a_record(&gone);
        entries.push(not_drawn(&still_in_the_queue));
        let scope = ClearingScope::TheServersNoLongerInSmoozy { the_rows_that_stand: vec![standing.clone()] };
        let deletions = a_clearing(&entries, &scope, SystemTime::UNIX_EPOCH);
        assert_eq!(deletions.len(), 3, "the three entries of the two removed servers");
        assert!(deletions.iter().all(ADeletion::is_one_unit));
        assert!(deletions.iter().all(|one| one.remove_the_record_of != records[0].job));
        assert!(deletions.iter().all(|one| one.remove_the_record_of != still_in_the_queue.job));
        assert!(deletions
            .iter()
            .all(|one| one.write_the_tombstone.machine_side_copy == MachineSideCopy::StillOwed));
        let groups = the_removed_servers_boxs_groups(&entries, &[standing.clone()]);
        assert_eq!(groups.len(), 2);
        assert_eq!((groups[0].local_server, groups[0].entries), (&gone, 2));
        assert_eq!((groups[1].local_server, groups[1].the_copied_label, groups[1].entries), (&also_gone, Some("Paperspace A6000"), 1));
        // **With every row standing there is nothing to clear**, and with none
        // standing it is every entry the list draws.
        let every_row = vec![standing.clone(), gone.clone(), also_gone.clone()];
        assert!(a_clearing(&entries, &ClearingScope::TheServersNoLongerInSmoozy { the_rows_that_stand: every_row }, SystemTime::UNIX_EPOCH).is_empty());
        assert_eq!(
            a_clearing(&entries, &ClearingScope::TheServersNoLongerInSmoozy { the_rows_that_stand: Vec::new() }, SystemTime::UNIX_EPOCH).len(),
            4
        );
    }

    #[test]
    fn the_boxes_figures_are_composed_over_the_local_records_alone_and_group_on_the_copied_identifier(
    ) {
        let a = a_server();
        let b = a_server();
        let mut old = a_record(&b);
        old.server.label = None;
        old.server.hue = None;
        let records = [a_record(&a), a_record(&a), old];
        let entries: Vec<AnEntryTheListDraws<'_>> = records.iter().map(drawn).collect();
        assert_eq!(the_per_server_boxs_count(&entries, &a), 2);
        let groups = the_all_history_boxs_groups(&entries);
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].local_server, &a);
        assert_eq!(groups[0].entries, 2);
        assert_eq!(groups[0].the_copied_label, Some("the big box"));
        assert_eq!(groups[0].the_copied_hue, Some(Hue(210)));
        // A dangling identifier still keys its group; an old record states
        // its absence plainly.
        assert_eq!(groups[1].local_server, &b);
        assert_eq!(groups[1].the_copied_label, None);
        assert_eq!(groups[1].the_copied_hue, None);
    }
}
