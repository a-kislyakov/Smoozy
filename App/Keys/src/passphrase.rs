//! The remembered passphrase — `Keys` §8.
//!
//! **§8.1 is a pointer and not a second arrangement.** Where a key passphrase
//! is kept, what shape the item takes, how it travels, and how it is erased
//! are `Security and licence` §3, §4 and §5's, and **a second arrangement here
//! would be a defect of the first order**. What is in this file is Keys' own
//! half: the keying, the marker, the triggers, and who raises the ask.
//!
//! **The route is split at `Security and licence` §4 R2 and the split is this
//! document's line too.** For a **connection**, the transport's own key-loading
//! step inside Smoozy's process, which crosses no process boundary — that is
//! `Servers`'. For a **local read**, the askpass route and never any other way,
//! which is the only one this module performs.

use smoozy_security_and_licence::standing::StoreStanding;

use crate::fingerprint::Fingerprint;
use crate::rows::{KeyRow, Name};
use crate::usable::{a_passphrase_is_available_without_asking, Verdict};

/// **S3: the erase triggers, as this module raises them, and they are
/// `Security and licence` §5.2's and §2 item 2's exactly, with nothing
/// added.**
///
/// Every one runs §5.1's order — mark, erase, remove the reference and the
/// mark — and **none is blocked by the store failing**.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EraseTrigger {
    /// 1. The key deleted — the passphrase item, the fingerprint, the path and
    ///    the local key row.
    TheKeyDeleted,
    /// 2. The key removed from Smoozy — the same, files untouched.
    TheKeyRemovedFromSmoozy,
    /// 3. **The file swapped at the same path, detected by the fingerprint
    ///    changing** (F2/U4 clause **4b**, and never by 4a failing); and the
    ///    replacement offer's deliberate re-point raises the same trigger for
    ///    the same reason.
    TheFileSwappedAtTheSamePath,
    /// 4. The user clearing the remembered passphrase — that item and nothing
    ///    else.
    TheUserClearedIt,
    /// §2 item 2's last column: **the passphrase ceasing to work**, which S4
    /// establishes by a local load and never by a failed connection.
    ThePassphraseStoppedWorking,
}

/// **S3 item 3 and U4 clause 4b, joined in one place** so no caller has to
/// remember which failure costs the passphrase.
pub fn what_a_verdict_raises(verdict: &Verdict) -> Option<EraseTrigger> {
    verdict
        .raises_the_swap_event()
        .then_some(EraseTrigger::TheFileSwappedAtTheSamePath)
}

/// **S5: where the trigger fires, as Keys raises it.** When a held passphrase
/// is supplied for an attempt, the supply is **ordered**, and the order is what
/// makes the attribution honest.
///
/// **S6: nothing retries.** The load runs once per attempt, the passphrase is
/// supplied once per attempt, no tight loop exists anywhere in this module,
/// and the store is reached once per use — Fact 37's source-penalty finding is
/// honoured by construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupplyStep {
    /// 1. The U10 check first, **fresh, at the attempt's own moment**.
    TheCheck,
    /// 2. **One local load** — the tool's `-y` read of the key, fed the held
    ///    passphrase through the askpass route, **once**. Observed: with a
    ///    correct passphrase the tool invoked askpass once and exited 0; with
    ///    a wrong one, **once** and exit 255 — the tool does not loop the way
    ///    `ssh-add`'s 59 invocations did.
    OneLocalLoad,
    /// 3. Then the connection, which is `Servers`' act and not this module's.
    ThenTheConnection,
}

impl SupplyStep {
    /// The order, named once so no caller composes a different one.
    pub const ORDER: [SupplyStep; 3] = [
        SupplyStep::TheCheck,
        SupplyStep::OneLocalLoad,
        SupplyStep::ThenTheConnection,
    ];
}

/// **S4 and S5: what a local load's failure means, and it is attributed by
/// elimination and never by parsing a diagnostic.**
///
/// **S7's first half is this function.** The shared invariant with `Whole app`'s
/// machine-password rule is that a secret is forgotten only on a signal that
/// separates the secret's own failure from the operation's — which is what the
/// passing check buys — and **every unseparated failure forgets nothing**,
/// which is what returning `None` is. S7's other half is the store get, at
/// `Desktop app and delivery`'s `keys::supply::the_held_passphrase`.
///
/// A load that fails **after a passing check** is attributable to the
/// passphrase without parsing any string, because every other cause the tool
/// separates only by undocumented text — missing file, bad permissions,
/// malformed container — was just ruled out by the check in the same moment.
///
/// **Even the local signal is not proof** — `Security and licence` §4 records
/// the correct passphrase against a one-bit-corrupted key giving the identical
/// message — **and the cost is the one accepted there**: one re-entry, against
/// the spec's rule that a passphrase that opens nothing is never kept.
pub fn a_failed_load_forgets_the_passphrase(the_check_passed: bool) -> Option<EraseTrigger> {
    the_check_passed.then_some(EraseTrigger::ThePassphraseStoppedWorking)
}

/// **S8 and S10: what a server is marked with, and it is this row's one
/// behavioural change.**
///
/// `Servers` P3 leaves the server marked with what is wrong. **The two read
/// identically to a builder and differently to a user**: the first is a
/// property of their computer that will not change by waiting, the second is a
/// miss that their next connect fixes. **The words are Design's; the bound is
/// that a permanent condition may not be worded as a passing one.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheServerIsMarkedWith {
    /// The ordinary mark: a passphrase Smoozy is not holding.
    APassphraseSmoozyIsNotHolding,
    /// **Where the standing reads *never answered* and the key is protected**,
    /// the mark states that the passphrase **cannot be held on this
    /// computer**. S9's consequence: both connections Smoozy makes on its own
    /// are withheld for ever, and a connect the user starts works normally,
    /// asking each time.
    ItCannotBeHeldOnThisComputer,
}

/// S10's derivation, from the standing and the protection reading.
pub fn how_the_server_is_marked(
    standing: StoreStanding,
    the_key_is_protected: bool,
) -> WhatTheServerIsMarkedWith {
    if the_key_is_protected && standing == StoreStanding::NeverAnswered {
        WhatTheServerIsMarkedWith::ItCannotBeHeldOnThisComputer
    } else {
        WhatTheServerIsMarkedWith::APassphraseSmoozyIsNotHolding
    }
}

/// **S11: the repair does not spin against it.**
///
/// `Servers` 3.2's standing repair never ends while the app is open, and each
/// of its attempts would take U9's check and be refused. **Where the refusal is
/// the permanent one of S10, the repair for that server stops rather than
/// retrying for ever**, and the server is left disconnected and marked. It is
/// **not** a fourth entry in `Servers` 3.3's list — the period is not ended and
/// the key is usable — it is the standing repair recognising an answer that
/// cannot change while the standing does not.
pub fn the_standing_repair_stops(mark: WhatTheServerIsMarkedWith) -> bool {
    mark == WhatTheServerIsMarkedWith::ItCannotBeHeldOnThisComputer
}

/// **S14: `Keys` raises the ask, and it is an ordinary window of the
/// application rather than a tool's.**
///
/// `Servers` P4 hands the question away in as many words — *"Who holds and
/// supplies a passphrase is the module map's and `Keys`', and this document
/// neither holds one nor asks for one itself"*.
///
/// **Three already-standing readings raise it and no other combination
/// does**: the key is **usable**, it is **protected**, and U9's availability
/// answer is **no**. Where the first is no, `Servers` P3 applies and no attempt
/// is made, **so the box never appears for an unusable key**; where the second
/// is no, there is nothing to ask for.
///
/// **Only on a connect the user started, and by construction rather than by
/// suppression**: for every connection Smoozy makes on its own `Servers` P3
/// forbids the attempt, so no route to the ask exists from one.
pub fn raises_the_ask(
    verdict: &Verdict,
    the_key_is_protected: bool,
    row: &KeyRow,
    the_user_started_this_connect: bool,
) -> bool {
    the_user_started_this_connect
        && *verdict == Verdict::Usable
        && the_key_is_protected
        && !a_passphrase_is_available_without_asking(row)
}

/// **S15: what the box is handed, and every value is a display-only field of a
/// row that is already held.**
///
/// **Nothing else, and in particular nothing about the keeping.** No line here
/// is composed from a stored value and no get is made to draw one — `Security
/// and licence` §3.0 rule 6.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheAsk {
    /// **The server**: the row's label, with its username and host beside it
    /// because the label is optional — `Servers` D3's display-only and
    /// connection-identity fields, **handed over by `Servers` at the ask**
    /// (`Servers` P4a). **`Keys` does not read a server row for this**:
    /// W11a's read
    /// matches on the fingerprint, and one fingerprint can name several
    /// servers, so it cannot say which one is being connected to.
    pub server: ServerContext,
    /// **The key**: R1's name as last read where the row has one, and **the
    /// row's own fingerprint where it has none** — never a path. A key's own
    /// fingerprint is granted here rather than invented: `Security and licence`
    /// §5.4 says so in as many words.
    pub key: HowTheKeyIsNamed,
    /// **S19: the line that follows an erase, where this ask follows one.**
    /// What it may say is bounded — that the remembered passphrase no longer
    /// opens this key and that Smoozy has forgotten it — **it names no cause**
    /// and carries no part of any value. Its wording is Design's.
    pub the_forgotten_passphrase_line: bool,
}

/// The three fields `Servers` hands over at the ask. **Any may be empty and
/// this document composes nothing from them**; which of the three the box shows
/// is Design's.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ServerContext {
    pub label: Option<String>,
    pub username: Option<String>,
    pub host: Option<String>,
}

/// **S15's rule is U11's own shape: what exists, and nothing where there is
/// none.** This document supplies the two values and composes no third.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HowTheKeyIsNamed {
    ByItsName(Name),
    ByItsFingerprint(Fingerprint),
    /// A row with neither — no name and a fingerprint that has never read.
    /// **Nothing is invented in its place.**
    ByNeither,
}

impl HowTheKeyIsNamed {
    pub fn of(row: &KeyRow) -> HowTheKeyIsNamed {
        match (&row.name, &row.fingerprint) {
            (Some(name), _) => HowTheKeyIsNamed::ByItsName(name.clone()),
            (None, Some(held)) => HowTheKeyIsNamed::ByItsFingerprint(held.value.clone()),
            (None, None) => HowTheKeyIsNamed::ByNeither,
        }
    }
}

/// **S18 and S19: what carries the message to the ask.**
///
/// **In the ordinary case it is the act**: S5's local load fails, §5.1's
/// erasure runs, and the same act raises S14's box with the line — run-held and
/// gone with the act.
///
/// **Where the act is interrupted, the line is lost, wherever the interruption
/// falls** — §13 item 19, widened 2026-09-18. `State, identity and recovery`
/// §9.4's outcome record for the act names its units by role and carries no
/// key, so a later connect for this key cannot find it, and every caller hands
/// `false` for the second argument. The user meets a first ask; **what is lost
/// is the explanation and never the passphrase**, which had already stopped
/// working and was already gone. *(Corrected 2026-09-18, step 10b's audit
/// round's loose ends, L3. This said §9.4's record carried the line past an
/// interruption after the erasure unit's outcome had landed, which step 10b's
/// first audit round, U1, found cannot be done.)*
pub fn the_line_is_said(
    this_act_forgot_it: bool,
    an_earlier_acts_erasure_outcome_is_on_the_record: bool,
) -> bool {
    this_act_forgot_it || an_earlier_acts_erasure_outcome_is_on_the_record
}

/// **S12: nothing here is a second notice.** `Security and licence` §3.3 says
/// the one sentence once per installation, at the standing's first transition,
/// and this module adds none of its own. **The mark of S10 is a server's
/// condition and not a notice**, which is why it may stand permanently without
/// repeating anything.
pub const THIS_MODULE_ADDS_NO_NOTICE_OF_ITS_OWN: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fingerprint::{ReadFrom, Reading};
    use crate::rows::PassphraseReference;
    use crate::usable::Unusable;
    use smoozy_security_and_licence::held::Subject;
    use smoozy_state::identities::{Identity, IdentityKind};
    use std::path::PathBuf;

    fn sha(body: &str) -> Fingerprint {
        Fingerprint::from_the_printed_form(&format!("SHA256:{body}")).expect("a form")
    }

    fn a_row(name: Option<&str>, holding: bool) -> KeyRow {
        let mut row = KeyRow::entering(
            Identity::issue(IdentityKind::LocalKey).expect("randomness"),
            PathBuf::from("/k/id_ed25519"),
            Some(Reading {
                value: sha("one"),
                read_from: ReadFrom::PrivateFile,
            }),
            name.and_then(Name::what_the_user_typed),
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

    #[test]
    fn only_the_swap_verdict_raises_an_erase() {
        assert_eq!(
            what_a_verdict_raises(&Verdict::Unusable(Unusable::NoLongerTheSameKey)),
            Some(EraseTrigger::TheFileSwappedAtTheSamePath)
        );
        assert_eq!(
            what_a_verdict_raises(&Verdict::Unusable(Unusable::Unreadable)),
            None
        );
        assert_eq!(
            what_a_verdict_raises(&Verdict::Unusable(Unusable::FileMissing)),
            None
        );
        assert_eq!(what_a_verdict_raises(&Verdict::NotChecked), None);
    }

    #[test]
    fn the_check_comes_before_the_load_and_the_load_before_the_connection() {
        assert_eq!(
            SupplyStep::ORDER,
            [
                SupplyStep::TheCheck,
                SupplyStep::OneLocalLoad,
                SupplyStep::ThenTheConnection
            ]
        );
    }

    #[test]
    fn a_load_that_failed_without_a_passing_check_forgets_nothing() {
        assert_eq!(
            a_failed_load_forgets_the_passphrase(true),
            Some(EraseTrigger::ThePassphraseStoppedWorking)
        );
        assert_eq!(a_failed_load_forgets_the_passphrase(false), None);
    }

    #[test]
    fn a_permanent_condition_is_not_worded_as_a_passing_one() {
        assert_eq!(
            how_the_server_is_marked(StoreStanding::NeverAnswered, true),
            WhatTheServerIsMarkedWith::ItCannotBeHeldOnThisComputer
        );
        assert_eq!(
            how_the_server_is_marked(StoreStanding::NeverAnswered, false),
            WhatTheServerIsMarkedWith::APassphraseSmoozyIsNotHolding
        );
        assert_eq!(
            how_the_server_is_marked(StoreStanding::HasAnswered, true),
            WhatTheServerIsMarkedWith::APassphraseSmoozyIsNotHolding
        );
        assert_eq!(
            how_the_server_is_marked(StoreStanding::NeverProbed, true),
            WhatTheServerIsMarkedWith::APassphraseSmoozyIsNotHolding
        );
    }

    #[test]
    fn the_repair_stops_against_the_permanent_answer_and_spins_against_nothing_else() {
        assert!(the_standing_repair_stops(
            WhatTheServerIsMarkedWith::ItCannotBeHeldOnThisComputer
        ));
        assert!(!the_standing_repair_stops(
            WhatTheServerIsMarkedWith::APassphraseSmoozyIsNotHolding
        ));
    }

    #[test]
    fn the_box_never_appears_for_an_unusable_key() {
        let row = a_row(None, false);
        assert!(!raises_the_ask(
            &Verdict::Unusable(Unusable::FileMissing),
            true,
            &row,
            true
        ));
        assert!(!raises_the_ask(&Verdict::NotChecked, true, &row, true));
    }

    #[test]
    fn the_box_never_appears_on_a_connection_smoozy_made_by_itself() {
        let row = a_row(None, false);
        assert!(!raises_the_ask(&Verdict::Usable, true, &row, false));
    }

    #[test]
    fn an_unprotected_key_has_nothing_to_ask_for() {
        let row = a_row(None, false);
        assert!(!raises_the_ask(&Verdict::Usable, false, &row, true));
    }

    #[test]
    fn a_held_passphrase_is_not_asked_for_again() {
        assert!(!raises_the_ask(
            &Verdict::Usable,
            true,
            &a_row(None, true),
            true
        ));
        assert!(raises_the_ask(
            &Verdict::Usable,
            true,
            &a_row(None, false),
            true
        ));
    }

    #[test]
    fn a_marked_reference_puts_the_ask_back_at_that_instant() {
        let mut row = a_row(None, true);
        row.passphrase.as_mut().expect("a reference").owing_an_erase = true;
        assert!(raises_the_ask(&Verdict::Usable, true, &row, true));
    }

    #[test]
    fn a_nameless_key_is_named_by_its_fingerprint_and_never_by_its_path() {
        let row = a_row(None, false);
        assert_eq!(
            HowTheKeyIsNamed::of(&row),
            HowTheKeyIsNamed::ByItsFingerprint(sha("one"))
        );
        let named = a_row(Some("work"), false);
        assert!(matches!(
            HowTheKeyIsNamed::of(&named),
            HowTheKeyIsNamed::ByItsName(_)
        ));
    }

    #[test]
    fn a_row_with_neither_value_has_nothing_invented_for_it() {
        let mut row = a_row(None, false);
        row.fingerprint = None;
        assert_eq!(HowTheKeyIsNamed::of(&row), HowTheKeyIsNamed::ByNeither);
    }

    #[test]
    fn the_line_rides_the_act_and_the_record_carries_it_across_an_interruption() {
        assert!(the_line_is_said(true, false));
        assert!(the_line_is_said(false, true));
        // §13 item 19: nothing was left to carry one.
        assert!(!the_line_is_said(false, false));
    }
}
