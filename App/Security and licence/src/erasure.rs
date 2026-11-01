//! Erasure — `Security and licence` §5.
//!
//! §5.1 E1: erasing one held thing is three writes, and the first is what
//! makes the other two safe. E3: a removal is never blocked by a store
//! failure. E4: the sweep acts on the marks and on nothing else.
//!
//! The marks live in the local record, which `Desktop app and delivery`
//! builds; what is here is the order, which is this document's. The order is
//! in one place because getting it backwards — the record's row first and the
//! store second — leaves a secret with nothing pointing at it and nothing that
//! would ever notice.

use crate::held::{HeldSecret, StoredKind};
use crate::secret_store::{SecretStore, StoreFailure};
use std::time::Duration;

/// Something the local record could not do. Its shape is the record's; what
/// this file needs of it is only that it can fail.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordFailure;

/// The three writes' record half — `Security and licence` §5.1 E1.
///
/// The local record owns these; they are named here because the **order** they
/// are performed in is this document's, and an implementation that reordered
/// them would keep every individual promise and break the one that matters.
pub trait EraseMarks {
    /// Write 1. Marks that subject as owing an erase. Until this has
    /// completed, nothing touches the store.
    fn mark_owing_an_erase(&mut self, item: &HeldSecret) -> Result<(), RecordFailure>;

    /// E3's *"the local record's row is removed regardless"*. Drops the
    /// reference to the store item and **leaves the mark standing**, so the
    /// sweep retries the erase.
    fn remove_reference(&mut self, item: &HeldSecret) -> Result<(), RecordFailure>;

    /// Write 3. Removes the local reference **and the mark together, never the
    /// mark alone** (E4). Performed only where the store has reported the item
    /// gone.
    fn clear_reference_and_mark(&mut self, item: &HeldSecret) -> Result<(), RecordFailure>;

    /// Every subject currently marked as owing an erase. The sweep's whole
    /// input: it **never treats the absence of a row as a reason to erase**, so
    /// a local record replaced by an empty one, or by a migration or a repair,
    /// carries no marks and costs nothing.
    fn marks(&self) -> Result<Vec<HeldSecret>, RecordFailure>;
}

/// How one erasure ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Erasure {
    /// All three writes completed. Nothing is owed.
    Done,
    /// The store did not confirm the erase — it refused, or it did not answer
    /// inside the deadline, in which case the erase may still land afterwards
    /// and the sweep's next erase is what confirms it. **Not a claim that the
    /// item is still held.** **The removal went ahead anyway** — the
    /// reference is gone, the mark stands, and the sweep will retry. The caller
    /// does not stop on this: blocking the removal would put a refusal in front
    /// of the user that no spec writes.
    ReferenceGoneAndOwedToTheSweep(StoreFailure),
    /// The mark could not be written, so nothing was touched. A store item
    /// erased with no mark behind it is exactly what E1's first write exists to
    /// prevent, so an unmarked erase is not attempted.
    NothingDoneTheMarkWouldNotWrite,
}

/// Erases one held thing, in E1's order and no other.
///
/// 1. The local record marks that subject as owing an erase.
/// 2. The secret-store item is erased.
/// 3. The local record's reference is removed, and the mark with it.
///
/// E2: a crash between 2 and 3 leaves a local reference to an item that is
/// gone, which reads as *not there* and asks the user again — the safe
/// direction. A crash between 1 and 2 leaves the mark, and the sweep is what
/// the mark is for.
pub fn erase(
    marks: &mut dyn EraseMarks,
    store: &dyn SecretStore,
    item: &HeldSecret,
    deadline: Duration,
) -> Erasure {
    if marks.mark_owing_an_erase(item).is_err() {
        return Erasure::NothingDoneTheMarkWouldNotWrite;
    }

    match store.erase(item, deadline) {
        Ok(()) => {
            // Where the third write itself fails, the mark stands and the
            // sweep completes it — which is the same state a crash here would
            // have left, and is already what E2 and E4 describe.
            let _ = marks.clear_reference_and_mark(item);
            Erasure::Done
        }
        Err(failure) => {
            let _ = marks.remove_reference(item);
            Erasure::ReferenceGoneAndOwedToTheSweep(failure)
        }
    }
}

/// When the sweep may run — `Security and licence` §5.1 E4, and the check
/// §15 records for it.
///
/// **It does not run at all where the local record did not open.** And **it
/// runs after the first successful store operation of a run rather than before
/// one**, so it never reaches the store in a sitting where nothing was going
/// to be handed over. That is the whole reason it is lazy, and it is why the
/// sweep is not a moment that reaches the Creator: nothing is asked of the
/// user that the act they started does not already ask.
#[derive(Debug, Default)]
pub struct SweepGate {
    a_store_operation_has_succeeded: bool,
}

impl SweepGate {
    /// Opened at the start of a run in which the local record opened. Where
    /// the record did not open, no gate is made and the sweep never runs.
    pub fn for_a_run_whose_record_opened() -> Self {
        SweepGate::default()
    }

    /// Called by the adapter after any store operation of this run succeeded.
    pub fn a_store_operation_succeeded(&mut self) {
        self.a_store_operation_has_succeeded = true;
    }

    pub fn may_sweep(&self) -> bool {
        self.a_store_operation_has_succeeded
    }
}

/// What one sweep did.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct SweepReport {
    /// Marks the store reported gone, whose reference and mark were cleared.
    pub cleared: usize,
    /// Marks the store would not erase. They stand, and the next sweep retries
    /// them.
    pub still_owed: usize,
}

/// The one sweep. It erases each marked subject and clears the mark when the
/// store reports the item gone. **That is the only sweep, it is not a
/// background watcher**, and it is what makes Row 41's *forget it* true after
/// an interrupted removal.
///
/// Returns `None` where the gate is shut, which is not a failure.
pub fn sweep(
    gate: &SweepGate,
    marks: &mut dyn EraseMarks,
    store: &dyn SecretStore,
    deadline: Duration,
) -> Option<SweepReport> {
    if !gate.may_sweep() {
        return None;
    }

    let owed = marks.marks().ok()?;
    let mut report = SweepReport::default();
    for item in owed {
        match store.erase(&item, deadline) {
            Ok(()) => {
                if marks.clear_reference_and_mark(&item).is_ok() {
                    report.cleared += 1;
                } else {
                    report.still_owed += 1;
                }
            }
            Err(_) => report.still_owed += 1,
        }
    }
    Some(report)
}

/// The events that erase something — `Security and licence` §5.2.
///
/// Each is a fact about the user's own act or about a machine, and each names
/// exactly what goes. The module that owns the act raises the event; this list
/// is what it means.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErasureEvent {
    /// E5. A key deleted, or removed from Smoozy. Both removal paths drop
    /// everything Keys held; only the delete path touches files, which is
    /// `Keys`'.
    KeyDeletedOrRemoved,
    /// E6. A key's file swapped at the same path, detected by the fingerprint
    /// changing. The passphrase belonged to the key that left.
    KeyFileSwappedAtTheSamePath,
    /// E7. The user clearing a remembered passphrase.
    UserClearedThePassphrase,
    /// E8. A server edited: username changed. Without asking — a password
    /// belongs to a login.
    ServerUsernameChanged,
    /// E9. A server edited: the user clearing the password.
    UserClearedTheMachinePassword,
    /// E10. A server edited: host or port changed. The remembered identity
    /// starts over. **Not a secret-store operation.**
    ServerHostOrPortChanged,
    /// E11. A server removed. **Without reaching the machine**, which Row 41
    /// requires: a disconnected server's removal starts no connection and asks
    /// for no passphrase.
    ServerRemoved,
    /// E12. A connection ending, however it ended.
    ConnectionEnded,
    /// E14. The bot token replaced — by a new token for the same bot, or by a
    /// different bot's. What becomes of the copies on machines is §10's.
    /// **Telegram switched off is not an erasure event** since 2026-10-31
    /// (`Telegram` K1, the Creator's fifteenth notes, note 1): the same token
    /// stays on every machine it was placed on, so erasing the desktop's copy
    /// ended nobody's hold on the bot, and turning Telegram on again cost a
    /// revoked token, a paste and every server reconnected.
    BotTokenReplaced,
}

/// What one event erases, said as the store item it reaches — where it reaches
/// one at all.
///
/// The rest of what each event erases is the local record's and the owning
/// module's: E5 also drops the fingerprint, the path and the local key row;
/// E10 and E11 drop the remembered identity; E11 drops the remembered setup
/// and the server row; E12 drops process-memory holds
/// ([`crate::held::UnstoredHolder`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErasesStoreItem {
    /// The event reaches one item of this kind, against the subject the
    /// raising module names.
    OfKind(StoredKind),
    /// The event touches no secret-store item at all.
    None,
}

impl ErasureEvent {
    pub fn erases_store_item(self) -> ErasesStoreItem {
        use ErasureEvent::*;
        match self {
            KeyDeletedOrRemoved | KeyFileSwappedAtTheSamePath | UserClearedThePassphrase => {
                ErasesStoreItem::OfKind(StoredKind::KeyPassphrase)
            }
            ServerUsernameChanged | UserClearedTheMachinePassword | ServerRemoved => {
                ErasesStoreItem::OfKind(StoredKind::MachinePassword)
            }
            BotTokenReplaced => ErasesStoreItem::OfKind(StoredKind::BotToken),
            // E10 starts the remembered identity over, which lives in the
            // local record and not the store (§2 item 6, H2).
            ServerHostOrPortChanged => ErasesStoreItem::None,
            // E12's holds are process memory and were never in a store.
            ConnectionEnded => ErasesStoreItem::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::held::Subject;
    use crate::plaintext::Plaintext;
    use crate::secret_store::{GetFailure, ProbeOutcome};
    use std::cell::RefCell;

    fn an_item() -> HeldSecret {
        HeldSecret::new(
            StoredKind::MachinePassword,
            Subject::issued_by_the_local_record("server-7"),
        )
    }

    #[derive(Default)]
    struct Marks {
        marked: Vec<HeldSecret>,
        references: Vec<HeldSecret>,
        refuse_the_mark: bool,
    }

    impl EraseMarks for Marks {
        fn mark_owing_an_erase(&mut self, item: &HeldSecret) -> Result<(), RecordFailure> {
            if self.refuse_the_mark {
                return Err(RecordFailure);
            }
            self.marked.push(item.clone());
            Ok(())
        }
        fn remove_reference(&mut self, item: &HeldSecret) -> Result<(), RecordFailure> {
            self.references.retain(|held| held != item);
            Ok(())
        }
        fn clear_reference_and_mark(&mut self, item: &HeldSecret) -> Result<(), RecordFailure> {
            self.references.retain(|held| held != item);
            self.marked.retain(|held| held != item);
            Ok(())
        }
        fn marks(&self) -> Result<Vec<HeldSecret>, RecordFailure> {
            Ok(self.marked.clone())
        }
    }

    struct Store {
        erases: RefCell<Vec<HeldSecret>>,
        refuse: bool,
    }

    impl Store {
        fn answering() -> Self {
            Store {
                erases: RefCell::new(Vec::new()),
                refuse: false,
            }
        }
        fn refusing() -> Self {
            Store {
                erases: RefCell::new(Vec::new()),
                refuse: true,
            }
        }
    }

    impl SecretStore for Store {
        fn put(&self, _: &HeldSecret, _: Plaintext, _: Duration) -> Result<(), StoreFailure> {
            Ok(())
        }
        fn get(&self, _: &HeldSecret, _: Duration) -> Result<Plaintext, GetFailure> {
            Err(GetFailure::NotThere)
        }
        fn erase(&self, item: &HeldSecret, _: Duration) -> Result<(), StoreFailure> {
            self.erases.borrow_mut().push(item.clone());
            if self.refuse {
                Err(StoreFailure::LockedOrRefused)
            } else {
                Ok(())
            }
        }
        fn probe(&self, _: Duration) -> ProbeOutcome {
            ProbeOutcome::RoundTripWorked
        }
        fn enumerate(&self, _: Duration) -> Result<Vec<HeldSecret>, StoreFailure> {
            Ok(Vec::new())
        }
    }

    fn a_second() -> Duration {
        Duration::from_secs(1)
    }

    #[test]
    fn the_mark_is_written_before_the_store_is_touched() {
        let mut marks = Marks {
            refuse_the_mark: true,
            ..Marks::default()
        };
        let store = Store::answering();
        let outcome = erase(&mut marks, &store, &an_item(), a_second());
        assert_eq!(outcome, Erasure::NothingDoneTheMarkWouldNotWrite);
        assert!(store.erases.borrow().is_empty());
    }

    #[test]
    fn a_completed_erasure_leaves_no_mark() {
        let mut marks = Marks::default();
        marks.references.push(an_item());
        let store = Store::answering();
        assert_eq!(
            erase(&mut marks, &store, &an_item(), a_second()),
            Erasure::Done
        );
        assert!(marks.marked.is_empty());
        assert!(marks.references.is_empty());
    }

    #[test]
    fn a_store_failure_does_not_block_the_removal_and_leaves_the_debt() {
        let mut marks = Marks::default();
        marks.references.push(an_item());
        let store = Store::refusing();
        assert_eq!(
            erase(&mut marks, &store, &an_item(), a_second()),
            Erasure::ReferenceGoneAndOwedToTheSweep(StoreFailure::LockedOrRefused)
        );
        assert!(marks.references.is_empty(), "the reference goes regardless");
        assert_eq!(marks.marked, vec![an_item()], "the debt stands");
    }

    #[test]
    fn the_sweep_does_not_run_before_a_store_operation_has_succeeded() {
        let gate = SweepGate::for_a_run_whose_record_opened();
        let mut marks = Marks::default();
        marks.marked.push(an_item());
        let store = Store::answering();
        assert_eq!(sweep(&gate, &mut marks, &store, a_second()), None);
        assert!(store.erases.borrow().is_empty());
    }

    #[test]
    fn the_sweep_clears_what_the_store_reports_gone() {
        let mut gate = SweepGate::for_a_run_whose_record_opened();
        gate.a_store_operation_succeeded();
        let mut marks = Marks::default();
        marks.marked.push(an_item());
        let store = Store::answering();
        assert_eq!(
            sweep(&gate, &mut marks, &store, a_second()),
            Some(SweepReport {
                cleared: 1,
                still_owed: 0
            })
        );
        assert!(marks.marked.is_empty());
    }

    #[test]
    fn the_sweep_keeps_a_mark_the_store_would_not_erase() {
        let mut gate = SweepGate::for_a_run_whose_record_opened();
        gate.a_store_operation_succeeded();
        let mut marks = Marks::default();
        marks.marked.push(an_item());
        let store = Store::refusing();
        assert_eq!(
            sweep(&gate, &mut marks, &store, a_second()),
            Some(SweepReport {
                cleared: 0,
                still_owed: 1
            })
        );
        assert_eq!(marks.marked, vec![an_item()]);
    }

    #[test]
    fn an_empty_record_costs_nothing_rather_than_erasing_everything() {
        let mut gate = SweepGate::for_a_run_whose_record_opened();
        gate.a_store_operation_succeeded();
        let mut marks = Marks::default();
        let store = Store::answering();
        assert_eq!(
            sweep(&gate, &mut marks, &store, a_second()),
            Some(SweepReport::default())
        );
        assert!(store.erases.borrow().is_empty());
    }

    #[test]
    fn each_event_erases_what_section_five_two_says_it_does() {
        use ErasesStoreItem::*;
        use ErasureEvent::*;
        assert_eq!(
            KeyDeletedOrRemoved.erases_store_item(),
            OfKind(StoredKind::KeyPassphrase)
        );
        assert_eq!(
            ServerRemoved.erases_store_item(),
            OfKind(StoredKind::MachinePassword)
        );
        assert_eq!(
            BotTokenReplaced.erases_store_item(),
            OfKind(StoredKind::BotToken)
        );
        assert_eq!(ServerHostOrPortChanged.erases_store_item(), None);
        assert_eq!(ConnectionEnded.erases_store_item(), None);
    }
}
