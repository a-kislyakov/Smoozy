//! Copy, delete and remove — `Keys` §9.5 to §9.7.
//!
//! **W8's order:** confirmation (W6, with W11's warning where servers rely on
//! the key) → **the file units, private half first**, each an unlink through
//! the platform file adapter with the outcome established by re-check-by-path
//! after (`Desktop app and delivery` F1's discipline — **gone is the
//! outcome**) → **the erasure unit**: `Security and licence` §5.1's three
//! writes.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `State, identity and recovery` P4, P5.

use std::path::{Path, PathBuf};
use std::time::Duration;

use smoozy_security_and_licence::erasure::{self, EraseMarks, Erasure};
use smoozy_security_and_licence::secret_store::SecretStore;
use smoozy_state::outcomes::{Keeps, Refused, UnitOutcome, Writing};

use crate::fingerprint::Fingerprint;
use crate::ownership::{FileSet, Ownership};
use crate::rows::KeyRow;
use crate::units::{Action, UnitRole};

/// **W7: copy reads `path.pub` and only that.**
///
/// **Present:** its contents go to the clipboard. **Absent:** the control is
/// unavailable — Smoozy does not go looking for a public half that isn't
/// there, and **does not reconstruct one from the private file**: under the
/// construction every local read runs with, `ssh-keygen -y` against an
/// encrypted key fails with an exit status rather than asking, and **the
/// control failing soft is the spec's own shape**.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Copy {
    /// The contents of the `.pub`, on their way to the clipboard, **with W7a's
    /// comparison run and agreeing**.
    ToTheClipboard(String),
    /// **W7a: copy stays available and says what it could not check.**
    ///
    /// The private half's own fingerprint would not read, so the `.pub` beside
    /// it was **not** compared against it and this copy is unchecked. **The
    /// type had two endings and no third, so a caller could not tell a checked
    /// copy from an unchecked one** — and under U7b that is not an edge case:
    /// the private read is refused for **every** protected key, so the guard is
    /// off for exactly the keys a user is most likely to have.
    ///
    /// The contents go to the clipboard either way, which is the honest ending
    /// §13 item 6 already takes. What the row says beside it is Design's under
    /// Rule 10.
    ///
    /// **The words, ruled 2026-09-13 at step 10a piece 4** under the Creator's
    /// standing delegation of 2026-08-31, the Design stage being closed. They
    /// are composed at `src/surface/keys.rs` in the desktop crate, where every
    /// word this screen shows is composed, and the reasoning is in that piece's
    /// account and in both decisions records.
    ToTheClipboardWithoutTheComparison(String),
    /// **In the same shape as an absent `.pub`**: the control is unavailable
    /// with the reason said. Nothing is deleted, nothing is rewritten, and no
    /// `.pub` is reconstructed — the file is the user's and stays exactly as it
    /// is.
    Unavailable(WhyNot),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyNot {
    /// No public half sits beside the private one.
    ThereIsNoPublicHalf,
    /// **W7a: the `.pub` beside it is a different key's.** What is not past the
    /// line is Smoozy showing one fingerprint on the row and copying a
    /// different key's public half without noticing — the app being wrong on
    /// screen about its own state, which costs the user a rented machine they
    /// can never connect to.
    ItIsADifferentKeysPublicHalf,
}

/// **W7a: where the private file's own fingerprint is readable, the `.pub`
/// beside it is fingerprinted and the two are compared before the contents
/// reach the clipboard.**
///
/// **Nothing new is read for it**: for any key whose private half yields a
/// fingerprint, U4 clause 4a reads exactly that value at the same moment, and
/// the public half's own fingerprint is U7's existing reading.
///
/// **Where the private read is refused, the comparison cannot run and the
/// limit is stated rather than a verdict invented** — that is **every
/// protected key**, whose row takes its fingerprint from the `.pub` in the
/// first place, and where comparing the file against itself would answer
/// nothing. **Copy stays available there and says what it could not check**,
/// which is its own third ending rather than a silent pass.
pub fn copy(
    contents_of_the_public_half: Option<String>,
    the_private_files_own_fingerprint: Option<&Fingerprint>,
    the_public_halfs_fingerprint: Option<&Fingerprint>,
) -> Copy {
    let Some(contents) = contents_of_the_public_half else {
        return Copy::Unavailable(WhyNot::ThereIsNoPublicHalf);
    };
    match (the_private_files_own_fingerprint, the_public_halfs_fingerprint) {
        (Some(private), Some(public)) if private != public => {
            Copy::Unavailable(WhyNot::ItIsADifferentKeysPublicHalf)
        }
        (Some(_), Some(_)) => Copy::ToTheClipboard(contents),
        // **One of the two values is missing, so the comparison did not run.**
        // The contents still go to the clipboard, and the copy says so rather
        // than presenting itself as checked.
        _ => Copy::ToTheClipboardWithoutTheComparison(contents),
    }
}

/// How one file unit ended. **Gone is the outcome**, established by re-reading
/// the path after the unlink and never by the call's return value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileUnit {
    Gone,
    /// **W9: the outcome names the file left behind, by path**, so nothing is
    /// left for the user to discover.
    LeftBehind { reported: String },
    /// **W9: the private half's unit failing stops the act**, and the rest is
    /// not attempted.
    NotAttempted,
    /// **A delete of a key whose files are already missing removes nothing and
    /// says the files were already gone** — and still runs the erasure unit,
    /// which is how a dead managed row is cleared.
    WasAlreadyGone,
}

/// The one file operation this act needs, behind the platform file adapter.
pub trait Unlinks {
    /// Unlink, then re-check by path. `Ok(true)` means the path is gone;
    /// `Ok(false)` means the file was not there to begin with; `Err` carries
    /// the machine's own words.
    fn unlink_and_recheck(&self, path: &Path) -> Result<bool, String>;
}

/// Why a delete did not run at all. **Neither of these is a unit failing** —
/// both happen before the first unit is attempted, so no outcome record has
/// units to report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotRun {
    /// **W2 re-derives ownership at the act's own moment.** Destroying a file
    /// Smoozy promised only to borrow is Row 251's unrecoverable branch, so
    /// this refuses rather than trusting the control that called it.
    TheKeyIsNotSmoozysToDelete,
    /// **The enumeration could not be made durable, so the act does not
    /// start.** `State, identity and recovery` §9.4 P5 item 1 puts the
    /// enumeration before the first unit precisely so an interruption cannot
    /// lose the fact that a unit was owed; an act that ran without it could
    /// destroy the private half and have nowhere to say so.
    TheOutcomeRecordCouldNotBeOpened(Refused),
}

/// What a delete came to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Deleted {
    pub private_half: FileUnit,
    pub public_half: Option<FileUnit>,
    pub certificate: Option<FileUnit>,
    /// `None` where the act stopped before the erasure unit was reached.
    pub erasure: Option<Erasure>,
    /// **W9: the row stays where the private half's unit failed.** Nothing
    /// claims the key is gone.
    pub the_row_is_removed: bool,
}

impl Deleted {
    /// **The act's outcome is derived** rather than written as a fourth value.
    pub fn the_key_is_gone(&self) -> bool {
        matches!(self.private_half, FileUnit::Gone | FileUnit::WasAlreadyGone)
    }

    /// The files a companion's failed unit left standing, by path — what the
    /// outcome names so nothing is left for the user to discover.
    pub fn left_behind<'a>(&self, set: &'a FileSet) -> Vec<&'a Path> {
        let mut left = Vec::new();
        if matches!(self.public_half, Some(FileUnit::LeftBehind { .. })) {
            left.extend(set.public_half.as_deref());
        }
        if matches!(self.certificate, Some(FileUnit::LeftBehind { .. })) {
            left.extend(set.certificate.as_deref());
        }
        left
    }
}

/// **W8: delete a managed key.**
///
/// **W2 re-derives ownership at the act's own moment**, and this function
/// refuses to run for a borrowed key rather than trusting the control that
/// called it — destroying a file Smoozy promised only to borrow is Row 251's
/// unrecoverable branch.
pub fn delete(
    row: &KeyRow,
    ownership: Ownership,
    set: &FileSet,
    files: &dyn Unlinks,
    records: &mut dyn Keeps,
    marks: &mut dyn EraseMarks,
    store: &dyn SecretStore,
    deadline: Duration,
) -> Result<Deleted, NotRun> {
    if ownership != Ownership::Managed {
        return Err(NotRun::TheKeyIsNotSmoozysToDelete);
    }

    // **`State, identity and recovery` §9.4 P5 item 1: the enumeration is
    // durable before the first unit is attempted**, and §11 enumerates it by
    // role.
    let act = Action::DeleteAManagedKey {
        there_is_a_public_half: set.public_half.is_some(),
        there_is_a_certificate: set.certificate.is_some(),
    };
    let mut record = Writing::open(records, act.as_action(), &act.roles())
        .map_err(NotRun::TheOutcomeRecordCouldNotBeOpened)?;

    let private_half = one_unit(&set.private_half, files);
    let _ = record.a_unit_ended(UnitRole::ThePrivateHalf.as_role(), outcome(&private_half));

    // **W9: the private half's unit failing stops the act.** The rest is not
    // attempted, the row stays, the key stays listed — likely unusable, and
    // saying why — and nothing claims the key is gone.
    if matches!(private_half, FileUnit::LeftBehind { .. }) {
        // **`State, identity and recovery` §9.3 P4: each remaining unit is written *not attempted*, naming
        // the unit that stopped it.** It is a write and not a silence: a unit
        // left with no outcome at all reads as one whose ending was lost.
        let _ = record.the_rest_were_not_attempted(UnitRole::ThePrivateHalf.as_role());
        return Ok(Deleted {
            private_half,
            public_half: set.public_half.as_ref().map(|_| FileUnit::NotAttempted),
            certificate: set.certificate.as_ref().map(|_| FileUnit::NotAttempted),
            erasure: None,
            the_row_is_removed: false,
        });
    }

    let mut companion = |path: &Option<PathBuf>, role: UnitRole| {
        path.as_ref().map(|path| {
            let ended = one_unit(path, files);
            let _ = record.a_unit_ended(role.as_role(), outcome(&ended));
            ended
        })
    };
    let public_half = companion(&set.public_half, UnitRole::ThePublicHalf);
    let certificate = companion(&set.certificate, UnitRole::TheCertificate);

    let erasure = the_erasure_unit(row, marks, store, deadline);
    let _ = record.a_unit_ended(
        UnitRole::TheErasure.as_role(),
        if erasure == Erasure::Done {
            UnitOutcome::Done
        } else {
            UnitOutcome::Failed
        },
    );

    Ok(Deleted {
        private_half,
        public_half,
        certificate,
        erasure: Some(erasure),
        // **A store failure never blocks any of it** (§5.1): the row goes, the
        // mark stays, the sweep retries.
        the_row_is_removed: true,
    })
}

/// How a file unit's ending is written into the record.
///
/// **A file that was already gone is *done*.** The unit's promise is that the
/// file is not there afterwards, and it is not — W8's own *"says the files were
/// already gone"* is a sentence about what happened, not a failed unit.
fn outcome(unit: &FileUnit) -> UnitOutcome {
    match unit {
        FileUnit::Gone | FileUnit::WasAlreadyGone => UnitOutcome::Done,
        FileUnit::LeftBehind { .. } => UnitOutcome::Failed,
        FileUnit::NotAttempted => UnitOutcome::NotAttempted {
            stopped_by: UnitRole::ThePrivateHalf.as_role().to_string(),
        },
    }
}

/// **W10: remove from Smoozy is the erasure unit alone — §5.1's three writes —
/// and no file is touched, ever.**
///
/// No confirmation of its own, because nothing is destroyed; the relied-on
/// warning still shows where servers hold the fingerprint. **It is one unit and
/// writes no outcome record at all**, its units not being smaller than the act.
///
/// After it, **Keys holds no value of the key's**: no fingerprint, no path, no
/// name, no bit, no store item, no tombstone — deletion tombstones are
/// history's and Keys files none.
pub fn remove_from_smoozy(
    row: &KeyRow,
    marks: &mut dyn EraseMarks,
    store: &dyn SecretStore,
    deadline: Duration,
) -> Erasure {
    the_erasure_unit(row, marks, store, deadline)
}

/// **S3 item 4: the user clearing the remembered passphrase — that item and
/// nothing else.**
///
/// `Preparation/Specs/Keys.md` puts the control in the standalone view's list
/// and `Preparation/Design/Keys.md` puts it on the row — *"One line on the row,
/// only where there is one, with the control that clears it"* — and §11
/// declares it as one partition unit, the §5.1 ordered write set, exactly as
/// [`remove_from_smoozy`].
///
/// **It is the same erasure and a different act, which is why it is its own
/// function.** `remove_from_smoozy` runs the same unit, but the act it belongs
/// to also drops the row; this one **leaves the row standing**, which is the
/// whole of what the user asked for. Nothing else on the row moves: not the
/// fingerprint, not the path, not the name, not N1's bit — and no file is
/// touched, ever.
///
/// **A crash inside leaves the mark and the sweep finishes the erase**, and the
/// row removal that never happens here is never blocked by the store (§5.1's
/// own rule).
pub fn clear_a_remembered_passphrase(
    row: &KeyRow,
    marks: &mut dyn EraseMarks,
    store: &dyn SecretStore,
    deadline: Duration,
) -> Erasure {
    the_erasure_unit(row, marks, store, deadline)
}

/// The erasure unit — **one ordered unit whose internal order is §5.1's own**.
///
/// A row carrying no store item has nothing to erase and the unit is done:
/// F12's *"Where none is carried there is nothing to lose"* seen from the act's
/// end.
fn the_erasure_unit(
    row: &KeyRow,
    marks: &mut dyn EraseMarks,
    store: &dyn SecretStore,
    deadline: Duration,
) -> Erasure {
    match row.store_item() {
        Some(item) => erasure::erase(marks, store, &item, deadline),
        None => Erasure::Done,
    }
}

fn one_unit(path: &Path, files: &dyn Unlinks) -> FileUnit {
    match files.unlink_and_recheck(path) {
        Ok(true) => FileUnit::Gone,
        Ok(false) => FileUnit::WasAlreadyGone,
        Err(reported) => FileUnit::LeftBehind { reported },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fingerprint::{ReadFrom, Reading};
    use crate::rows::PassphraseReference;
    use smoozy_security_and_licence::held::{HeldSecret, Subject};
    use smoozy_security_and_licence::plaintext::Plaintext;
    use smoozy_security_and_licence::secret_store::{
        GetFailure, ProbeOutcome, StoreFailure,
    };
    use smoozy_state::identities::{Identity, IdentityKind};
    use smoozy_state::outcomes::ActionOutcome;
    use std::cell::RefCell;
    use std::collections::HashSet;

    fn sha(body: &str) -> Fingerprint {
        Fingerprint::from_the_printed_form(&format!("SHA256:{body}")).expect("a form")
    }

    fn a_row(holding: bool) -> KeyRow {
        let mut row = KeyRow::entering(
            Identity::issue(IdentityKind::LocalKey).expect("randomness"),
            PathBuf::from("/k/.ssh/smoozy_ed25519"),
            Some(Reading {
                value: sha("one"),
                read_from: ReadFrom::PrivateFile,
            }),
            None,
            false,
        )
        .expect("a row");
        if holding {
            row.passphrase = Some(PassphraseReference {
                subject: Subject::issued_by_the_local_record("s-1"),
                owing_an_erase: false,
            });
        }
        row
    }

    struct Disk {
        stubborn: HashSet<PathBuf>,
        absent: HashSet<PathBuf>,
        unlinked: RefCell<Vec<PathBuf>>,
    }

    impl Disk {
        fn new() -> Disk {
            Disk {
                stubborn: HashSet::new(),
                absent: HashSet::new(),
                unlinked: RefCell::new(Vec::new()),
            }
        }
    }

    impl Unlinks for Disk {
        fn unlink_and_recheck(&self, path: &Path) -> Result<bool, String> {
            self.unlinked.borrow_mut().push(path.to_path_buf());
            if self.stubborn.contains(path) {
                return Err("permission refused".into());
            }
            Ok(!self.absent.contains(path))
        }
    }

    /// `State, identity and recovery` §9.4's record, kept in a list. The
    /// engine-backed one is `Desktop app and delivery`'s and has its own
    /// tests; what these tests want is what this act wrote and in what order.
    #[derive(Default)]
    struct Book {
        records: Vec<smoozy_state::outcomes::Recorded>,
        /// Every write, in the order it was made — which is what `State,
        /// identity and recovery` §9.4 P5's *before* and *at the moment* are
        /// checked with.
        writes: Vec<String>,
    }

    impl Book {
        fn only(&self) -> &smoozy_state::outcomes::Recorded {
            assert_eq!(self.records.len(), 1, "one act, one record");
            &self.records[0]
        }
    }

    impl Keeps for Book {
        fn enumerate(
            &mut self,
            action: &str,
            units: &[&str],
        ) -> Result<Identity, Refused> {
            let identity = smoozy_state::outcomes::issue().expect("randomness");
            self.writes.push(format!("owed: {}", units.join(", ")));
            self.records.push(smoozy_state::outcomes::Recorded {
                identity: identity.clone(),
                action: action.to_string(),
                units: units.iter().map(|unit| (unit.to_string(), None)).collect(),
            });
            Ok(identity)
        }

        fn a_unit_ended(
            &mut self,
            record: &Identity,
            unit: &str,
            outcome: UnitOutcome,
        ) -> Result<(), Refused> {
            self.writes.push(format!("{unit}: {outcome:?}"));
            let held = self
                .records
                .iter_mut()
                .find(|held| held.identity == *record)
                .expect("a record");
            let entry = held
                .units
                .iter_mut()
                .find(|(owed, _)| owed == unit)
                .ok_or(Refused::ThatUnitWasNotOwed)?;
            if entry.1.is_some() {
                return Err(Refused::ThatUnitHasAlreadyEnded);
            }
            entry.1 = Some(outcome);
            Ok(())
        }

        fn read(
            &self,
            record: &Identity,
        ) -> Result<Option<smoozy_state::outcomes::Recorded>, Refused> {
            Ok(self
                .records
                .iter()
                .find(|held| held.identity == *record)
                .cloned())
        }

        fn the_user_removed_it(&mut self, record: &Identity) -> Result<(), Refused> {
            self.records.retain(|held| held.identity != *record);
            Ok(())
        }
    }

    #[derive(Default)]
    struct Marks {
        marked: Vec<HeldSecret>,
        cleared: Vec<HeldSecret>,
    }

    impl EraseMarks for Marks {
        fn mark_owing_an_erase(
            &mut self,
            item: &HeldSecret,
        ) -> Result<(), smoozy_security_and_licence::erasure::RecordFailure> {
            self.marked.push(item.clone());
            Ok(())
        }
        fn remove_reference(
            &mut self,
            _item: &HeldSecret,
        ) -> Result<(), smoozy_security_and_licence::erasure::RecordFailure> {
            Ok(())
        }
        fn clear_reference_and_mark(
            &mut self,
            item: &HeldSecret,
        ) -> Result<(), smoozy_security_and_licence::erasure::RecordFailure> {
            self.cleared.push(item.clone());
            Ok(())
        }
        fn marks(
            &self,
        ) -> Result<Vec<HeldSecret>, smoozy_security_and_licence::erasure::RecordFailure> {
            Ok(self.marked.clone())
        }
    }

    struct Store {
        erases: bool,
    }

    impl SecretStore for Store {
        fn put(
            &self,
            _item: &HeldSecret,
            _value: Plaintext,
            _deadline: Duration,
        ) -> Result<(), StoreFailure> {
            Ok(())
        }
        fn get(&self, _item: &HeldSecret, _deadline: Duration) -> Result<Plaintext, GetFailure> {
            Err(GetFailure::NotThere)
        }
        fn erase(&self, _item: &HeldSecret, _deadline: Duration) -> Result<(), StoreFailure> {
            if self.erases {
                Ok(())
            } else {
                Err(StoreFailure::StoreUnavailable)
            }
        }
        fn probe(&self, _deadline: Duration) -> ProbeOutcome {
            ProbeOutcome::RoundTripWorked
        }
        fn enumerate(&self, _deadline: Duration) -> Result<Vec<HeldSecret>, StoreFailure> {
            Ok(Vec::new())
        }
    }

    fn a_set() -> FileSet {
        FileSet::at_this_moment(Path::new("/k/.ssh/smoozy_ed25519"), true, false)
    }

    fn a_moment() -> Duration {
        Duration::from_secs(2)
    }

    #[test]
    fn a_delete_never_runs_for_a_borrowed_key() {
        let mut book = Book::default();
        let mut marks = Marks::default();
        assert_eq!(
            delete(
                &a_row(true),
                Ownership::Borrowed,
                &a_set(),
                &Disk::new(),
                &mut book,
                &mut marks,
                &Store { erases: true },
                a_moment()
            ),
            Err(NotRun::TheKeyIsNotSmoozysToDelete)
        );
        // Not one write was made, the enumeration included.
        assert!(book.writes.is_empty());
        assert!(marks.marked.is_empty());
    }

    #[test]
    fn the_units_are_owed_before_the_first_unlink_and_named_by_role() {
        let mut book = Book::default();
        let mut marks = Marks::default();
        delete(
            &a_row(true),
            Ownership::Managed,
            &a_set(),
            &Disk::new(),
            &mut book,
            &mut marks,
            &Store { erases: true },
            a_moment(),
        )
        .expect("a delete");
        assert_eq!(
            book.only()
                .units
                .iter()
                .map(|(role, _)| role.as_str())
                .collect::<Vec<_>>(),
            vec!["the private half", "the public half", "the erasure"]
        );
        assert_eq!(book.only().action, "delete a managed key");
        // `State, identity and recovery` §9.4 P5 item 1: the enumeration is
        // durable before the first unit's outcome is, and this is the order the
        // writes were actually made in.
        assert_eq!(
            book.writes[0],
            "owed: the private half, the public half, the erasure"
        );
    }

    #[test]
    fn the_private_half_goes_first() {
        let disk = Disk::new();
        let mut book = Book::default();
        let mut marks = Marks::default();
        delete(
            &a_row(true),
            Ownership::Managed,
            &a_set(),
            &disk,
            &mut book,
            &mut marks,
            &Store { erases: true },
            a_moment(),
        )
        .expect("a delete");
        assert_eq!(
            disk.unlinked.borrow()[0],
            PathBuf::from("/k/.ssh/smoozy_ed25519")
        );
    }

    #[test]
    fn a_failed_private_half_stops_the_act_and_nothing_claims_the_key_is_gone() {
        let mut disk = Disk::new();
        disk.stubborn
            .insert(PathBuf::from("/k/.ssh/smoozy_ed25519"));
        let mut book = Book::default();
        let mut marks = Marks::default();
        let deleted = delete(
            &a_row(true),
            Ownership::Managed,
            &a_set(),
            &disk,
            &mut book,
            &mut marks,
            &Store { erases: true },
            a_moment(),
        )
        .expect("a delete");

        assert!(matches!(deleted.private_half, FileUnit::LeftBehind { .. }));
        assert_eq!(deleted.public_half, Some(FileUnit::NotAttempted));
        assert_eq!(deleted.erasure, None);
        assert!(!deleted.the_row_is_removed);
        assert!(!deleted.the_key_is_gone());
        // The erasure was never raised, so nothing was marked.
        assert!(marks.marked.is_empty());
        assert_eq!(disk.unlinked.borrow().len(), 1);

        // **`State, identity and recovery` §9.3 P4: the units that were never tried are written down, naming
        // the one that stopped them** — not left blank, which would read as
        // endings an interruption lost.
        let record = book.only();
        assert_eq!(record.the_actions_outcome(), Some(ActionOutcome::Nothing));
        assert_eq!(
            record.units[1].1,
            Some(UnitOutcome::NotAttempted {
                stopped_by: "the private half".to_string()
            })
        );
        assert_eq!(
            record.units[2].1,
            Some(UnitOutcome::NotAttempted {
                stopped_by: "the private half".to_string()
            })
        );
        assert!(record.unfinished().next().is_none());
    }

    #[test]
    fn a_delete_whose_files_were_already_gone_still_ends_every_unit_well() {
        let mut disk = Disk::new();
        disk.absent.insert(PathBuf::from("/k/.ssh/smoozy_ed25519"));
        disk.absent
            .insert(PathBuf::from("/k/.ssh/smoozy_ed25519.pub"));
        let mut book = Book::default();
        let mut marks = Marks::default();
        delete(
            &a_row(true),
            Ownership::Managed,
            &a_set(),
            &disk,
            &mut book,
            &mut marks,
            &Store { erases: true },
            a_moment(),
        )
        .expect("a delete");
        // The unit's promise is that the file is not there afterwards, and it
        // is not.
        assert_eq!(
            book.only().the_actions_outcome(),
            Some(ActionOutcome::Whole)
        );
    }

    #[test]
    fn a_failed_companion_leaves_the_key_destroyed_and_names_what_stayed() {
        let mut disk = Disk::new();
        disk.stubborn
            .insert(PathBuf::from("/k/.ssh/smoozy_ed25519.pub"));
        let mut book = Book::default();
        let mut marks = Marks::default();
        let set = a_set();
        let deleted = delete(
            &a_row(true),
            Ownership::Managed,
            &set,
            &disk,
            &mut book,
            &mut marks,
            &Store { erases: true },
            a_moment(),
        )
        .expect("a delete");

        assert_eq!(deleted.private_half, FileUnit::Gone);
        assert!(deleted.the_key_is_gone());
        assert!(deleted.the_row_is_removed);
        assert_eq!(deleted.erasure, Some(Erasure::Done));
        assert_eq!(
            deleted.left_behind(&set),
            vec![Path::new("/k/.ssh/smoozy_ed25519.pub")]
        );
    }

    #[test]
    fn a_store_failure_never_blocks_the_removal() {
        let mut book = Book::default();
        let mut marks = Marks::default();
        let deleted = delete(
            &a_row(true),
            Ownership::Managed,
            &a_set(),
            &Disk::new(),
            &mut book,
            &mut marks,
            &Store { erases: false },
            a_moment(),
        )
        .expect("a delete");
        assert!(deleted.the_row_is_removed);
        assert!(matches!(
            deleted.erasure,
            Some(Erasure::ReferenceGoneAndOwedToTheSweep(_))
        ));
        // The mark stands and the sweep retries.
        assert_eq!(marks.marked.len(), 1);
        assert!(marks.cleared.is_empty());
    }

    #[test]
    fn a_dead_managed_row_is_cleared_even_though_nothing_was_removed() {
        let mut disk = Disk::new();
        disk.absent.insert(PathBuf::from("/k/.ssh/smoozy_ed25519"));
        let mut book = Book::default();
        let mut marks = Marks::default();
        let deleted = delete(
            &a_row(true),
            Ownership::Managed,
            &FileSet::at_this_moment(Path::new("/k/.ssh/smoozy_ed25519"), false, false),
            &disk,
            &mut book,
            &mut marks,
            &Store { erases: true },
            a_moment(),
        )
        .expect("a delete");
        assert_eq!(deleted.private_half, FileUnit::WasAlreadyGone);
        assert!(deleted.the_row_is_removed);
        assert_eq!(deleted.erasure, Some(Erasure::Done));
    }

    #[test]
    fn remove_from_smoozy_touches_no_file_at_all() {
        let disk = Disk::new();
        let mut marks = Marks::default();
        let erasure = remove_from_smoozy(
            &a_row(true),
            &mut marks,
            &Store { erases: true },
            a_moment(),
        );
        assert_eq!(erasure, Erasure::Done);
        assert!(disk.unlinked.borrow().is_empty());
        assert_eq!(marks.cleared.len(), 1);
    }

    /// **S3 item 4: something performs *clear a remembered passphrase*.** The
    /// action was declared at §11 and the trigger at S3, the row's control is
    /// drawn, and **no function ran it** — `remove_from_smoozy` runs the same
    /// erasure but its act also drops the row, so it was not a stand-in.
    #[test]
    fn clearing_a_remembered_passphrase_runs_the_same_erasure_and_touches_no_file() {
        let disk = Disk::new();
        let mut marks = Marks::default();
        let row = a_row(true);
        let erasure =
            clear_a_remembered_passphrase(&row, &mut marks, &Store { erases: true }, a_moment());
        assert_eq!(erasure, Erasure::Done);
        // §5.1's three writes ran, and no file was touched.
        assert_eq!(marks.marked.len(), 1);
        assert_eq!(marks.cleared.len(), 1);
        assert!(disk.unlinked.borrow().is_empty());
    }

    /// **And a row carrying no reference has nothing to clear**, which is F12's
    /// *"Where none is carried there is nothing to lose"* seen from this act.
    #[test]
    fn clearing_a_passphrase_a_row_never_held_erases_nothing() {
        let mut marks = Marks::default();
        assert_eq!(
            clear_a_remembered_passphrase(
                &a_row(false),
                &mut marks,
                &Store { erases: false },
                a_moment()
            ),
            Erasure::Done
        );
        assert!(marks.marked.is_empty());
    }

    #[test]
    fn a_row_carrying_no_reference_has_nothing_to_lose() {
        let mut marks = Marks::default();
        assert_eq!(
            remove_from_smoozy(&a_row(false), &mut marks, &Store { erases: false }, a_moment()),
            Erasure::Done
        );
        assert!(marks.marked.is_empty());
    }

    #[test]
    fn copy_is_unavailable_where_there_is_no_public_half_and_reconstructs_none() {
        assert_eq!(
            copy(None, Some(&sha("one")), None),
            Copy::Unavailable(WhyNot::ThereIsNoPublicHalf)
        );
    }

    #[test]
    fn a_stale_public_half_is_never_copied_under_the_rows_own_fingerprint() {
        assert_eq!(
            copy(
                Some("ssh-ed25519 AAAA other\n".into()),
                Some(&sha("one")),
                Some(&sha("two"))
            ),
            Copy::Unavailable(WhyNot::ItIsADifferentKeysPublicHalf)
        );
    }

    /// **W7a: where the private read is refused, copy stays available and says
    /// what it could not check.** It had two endings and no third, so a caller
    /// could not tell a checked copy from an unchecked one — and under U7b the
    /// private read is refused for **every** protected key, so the guard is off
    /// for exactly the keys a user is most likely to have.
    #[test]
    fn where_the_private_read_is_refused_copy_says_what_it_could_not_check() {
        assert_eq!(
            copy(Some("ssh-rsa AAAA legacy\n".into()), None, Some(&sha("two"))),
            Copy::ToTheClipboardWithoutTheComparison("ssh-rsa AAAA legacy\n".into())
        );
        // And the other way the comparison can fail to run: the `.pub` is
        // there and its own fingerprint would not read.
        assert_eq!(
            copy(Some("ssh-rsa AAAA legacy\n".into()), Some(&sha("one")), None),
            Copy::ToTheClipboardWithoutTheComparison("ssh-rsa AAAA legacy\n".into())
        );
    }

    #[test]
    fn a_matching_public_half_reaches_the_clipboard() {
        assert_eq!(
            copy(
                Some("ssh-ed25519 AAAA someone\n".into()),
                Some(&sha("one")),
                Some(&sha("one"))
            ),
            Copy::ToTheClipboard("ssh-ed25519 AAAA someone\n".into())
        );
    }
}
