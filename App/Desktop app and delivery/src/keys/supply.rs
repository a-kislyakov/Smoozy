//! Supplying a passphrase for an attempt — `Keys` §8.3, §8.5 and §8.6.
//!
//! **This is the half of a connect that belongs to `Keys`.** `Servers` makes
//! the connection and hands the question away in as many words; what is here
//! is `Keys` S5's ordered supply, the one local load that separates a
//! passphrase's own failure from everything else, the erasure that follows a
//! failed one, and what the ask is handed.
//!
//! **The connection itself is not here and is not called from here.** S16 fixes
//! the order — the readings, then the box, then the check, the one local load,
//! and the connection — and the last step is `Servers`', arriving at step 4.
//!
//! **`Keys` S6: nothing retries.** The load runs once per attempt, the
//! passphrase is supplied once per attempt, and no loop exists anywhere in
//! this file.
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this crate's
//! own plan, `Preparation/Architecture/Desktop app and delivery.md`. Every other
//! document's marker is written with that document — at the site, or in this
//! file's own header list below. **Where a marker is a clause of this crate's
//! own plan *and* of the document meant, the header list is not enough and the
//! document is written out at the site**, because a bare one there resolves to a
//! real and wrong clause.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Keys` N1, S14, S16, S17, S18, S19, U7a, U9, U10.
//! `Security and licence` E-rules of §5.1, cited by section rather than by
//! marker.
//!
//! **A marker this crate's own plan also defines is deliberately not listed
//! above, whichever document is meant, and is written out with its document
//! at every site instead** — the header list cannot tell the two apart, and a
//! bare one would resolve to a real and wrong clause.

use std::path::{Path, PathBuf};
use std::time::Duration;

use smoozy_keys::diary::{APassphraseForgotten, Act};
use smoozy_keys::fingerprint::Reading;
use smoozy_keys::locations;
use smoozy_keys::passphrase::{self, HowTheKeyIsNamed, ServerContext, TheAsk};
use smoozy_keys::rows::{KeyRow, PassphraseReference};
use smoozy_keys::units::{Action, UnitRole};
use smoozy_keys::usable::{self, Verdict};
use smoozy_security_and_licence::erasure::{self, Erasure};
use smoozy_security_and_licence::held::{HeldSecret, StoredKind, Subject};
use smoozy_security_and_licence::plaintext::{OneUseBroker, Origin};
use smoozy_security_and_licence::plaintext::Plaintext;
use smoozy_security_and_licence::secret_store::{GetFailure, SecretStore};
use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_state::outcomes::{Keeps, UnitOutcome, Writing};

use super::askpass::{self, TheRoute};
use super::readings::OnThisComputer;
use crate::record::key_rows;

/// What a get of the row's held passphrase came to.
///
/// **`Keys` S7's other half.** The invariant is that a secret is forgotten
/// only on a signal that separates the secret's own failure from the
/// operation's, and a store that will not answer separates nothing. So two of
/// the three endings here forget nothing and ask the user for nothing.
#[derive(Debug)]
pub enum HeldPassphrase {
    /// The store answered and the value is in hand, for this attempt only.
    Held(Plaintext),
    /// **Nothing is held, so the user is asked** — the row carries no
    /// reference at all (`Keys` S2's marker absent, which is U9's answer
    /// *no*), and the store was not reached.
    NoneIsHeld,
    /// **The row's marker was ahead of the store: it names a subject and the
    /// store answered *not there*.** `Security and licence` §3.0 rule 4 makes
    /// *not there* the one class that asks the user again, whatever the
    /// marker says, and `Keys` U9 says what the marker then costs: the
    /// reference is erased under §5.1's order and the availability answer is
    /// no from then on — [`forget_a_reference_the_store_no_longer_holds`].
    /// *(Added 2026-09-18. This was folded into `NoneIsHeld`, and the ask then
    /// read the stale marker, said a passphrase was held, and raised no box.)*
    GoneFromTheStore,
    /// **The store failed as *locked or refused* or *store unavailable*, which
    /// is an ordinary failure and not a secret's own** (`Keys` S7; §3.0 rule
    /// 4). Reported on the thing that failed, the action offered again, the
    /// user **not** asked for the passphrase and the passphrase **not**
    /// forgotten.
    TheStoreFailed { failure: GetFailure },
}

/// **`Keys` S7: the get, and what each of its three endings costs.**
///
/// Nothing else in this module reaches the store for a passphrase — U9's
/// availability answer is read from the local record precisely so that
/// answering it cannot raise a system unlock prompt (`usable`). This is the one
/// place a value is fetched, and it is fetched only once an attempt is actually
/// being made.
pub fn the_held_passphrase(
    row: &KeyRow,
    store: &dyn SecretStore,
    deadline: Duration,
) -> HeldPassphrase {
    let Some(item) = row.store_item() else {
        return HeldPassphrase::NoneIsHeld;
    };
    match store.get(&item, deadline) {
        Ok(value) => HeldPassphrase::Held(value),
        Err(failure) if failure.asks_the_user_again() => HeldPassphrase::GoneFromTheStore,
        Err(failure) => HeldPassphrase::TheStoreFailed { failure },
    }
}

/// **`Keys` U9's last clause: a marker that turns out to be ahead of the
/// store is corrected where it surfaces, inside the attempt.**
///
/// *"A get returning *not there* — surfaces there, erases the reference under
/// §5.1's order, and the availability answer is no from then on."* §5.1's
/// order is [`erasure::erase`]'s — mark, erase, remove the reference and the
/// mark together — and erasing an item the store no longer holds is reported
/// by every adapter as done.
///
/// **What comes back is the row as the ask must read it: holding no
/// reference, whatever the erasure came to.** The store's *not there* is what
/// asks again (§3.0 rule 4), and a mark that would not write leaves the stale
/// reference in the record without making the store hold anything. Where the
/// user then types a passphrase that opens the key, the put writes a fresh
/// reference over it.
///
/// **No outcome record and no diary line.** This is not one of `Keys` S3's
/// triggers and nothing the user did is being undone: the item was already
/// gone, and what is corrected is the record's note of it.
pub fn forget_a_reference_the_store_no_longer_holds(
    row: &KeyRow,
    record_lender: &dyn crate::keys::LendsTheRecord,
    store: &dyn SecretStore,
    deadline: Duration,
) -> (Erasure, KeyRow) {
    let erasure = match row.store_item() {
        Some(item) => {
            let mut marks = crate::keys::TheMarksLent(record_lender);
            erasure::erase(&mut marks, store, &item, deadline)
        }
        None => Erasure::Done,
    };
    let mut as_it_now_stands = row.clone();
    as_it_now_stands.passphrase = None;
    (erasure, as_it_now_stands)
}

/// **`Keys` S5's one local load: the tool's `-y` read of the key, fed the
/// passphrase through the askpass route, once.**
///
/// Observed on the writing throwaway: the load with a correct passphrase
/// invoked askpass once and exited 0; with a wrong one, **once** and exit 255 —
/// the tool does not loop.
///
/// **No diagnostic text is read.** What the exit status says is that the load
/// failed; which clause failed is Smoozy's own reading, taken a moment earlier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Load {
    Opened,
    Refused,
    /// The channel could not be arranged, so no load was attempted. **This is
    /// not the passphrase failing** and forgets nothing — on Windows there is
    /// no channel at all (`Keys` §13 item 2).
    NoChannelCouldBeArranged { why: askpass::CouldNotDeliver },
}

/// **The value arrives as one delivery and never as something this function
/// could use twice** — `Security and licence` §1.4's hop 2, whether the value
/// was typed a moment ago or came out of the store.
pub fn the_local_load(path: &Path, delivery: OneUseBroker, the_route: &TheRoute) -> Load {
    let arguments = vec!["-y".to_string(), "-f".to_string(), path.display().to_string()];
    match askpass::run_with_the_askpass_route(&arguments, delivery, the_route) {
        Ok(askpass::ToolRun::Finished {
            exit_was_success: true,
        }) => Load::Opened,
        Ok(askpass::ToolRun::Finished { .. }) => Load::Refused,
        Err(why) => Load::NoChannelCouldBeArranged { why },
    }
}

/// What a supply came to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Supplied {
    /// The key opened. **The connection is `Servers`' next step**, and nothing
    /// here makes it.
    TheKeyOpened,
    /// **The check failed before any load.** `Servers` P3 applies: no attempt
    /// is made, and the passphrase is untouched — a file that cannot be read at
    /// this moment is not this key's fingerprint changing.
    TheCheckFailed { verdict: Verdict },
    /// **`Keys` S5: the load failed after a passing check, so the passphrase
    /// is attributable and is forgotten.** The same act raises the ask, with
    /// S19's line.
    ThePassphraseStoppedWorking {
        erasure: Erasure,
        /// §9.4's record for this act, which the store put of a newly entered
        /// passphrase ends.
        outcome_record: Identity,
        ask: TheAsk,
    },
    /// The load never ran. **Nothing is forgotten**, because nothing separated
    /// the passphrase's own failure from anything else.
    TheLoadCouldNotBeRun { why: askpass::CouldNotDeliver },
}

/// **`Keys` S5: where a held passphrase is supplied for an attempt, the supply
/// is ordered** — the U10 check first, fresh at the attempt's own moment, then
/// one local load, then the connection.
///
/// **A load that fails after a passing check is attributable to the passphrase
/// without parsing any diagnostic string**, because every other cause the tool
/// separates only by undocumented text was just ruled out by the check in the
/// same moment.
///
/// **The record is lent for each read and each write and is never held across
/// the check or the load** — step 10a, the screens round's loose ends, L4.
/// Both are `ssh-keygen` runs, and the load carries the askpass wait.
#[allow(clippy::too_many_arguments)]
pub fn supply_a_held_passphrase(
    row: &KeyRow,
    held: OneUseBroker,
    server: ServerContext,
    record_lender: &dyn crate::keys::LendsTheRecord,
    store: &dyn SecretStore,
    deadline: Duration,
    the_route: &TheRoute,
) -> Supplied {
    let readings = OnThisComputer::over(the_route.channel_area.clone());
    let checked = usable::check(row, &readings);
    // **`Keys` R1's refresh, at the check that took the readings.**
    keep_what_the_check_read(record_lender, row, &checked);
    if checked.verdict != Verdict::Usable {
        return Supplied::TheCheckFailed {
            verdict: checked.verdict,
        };
    }

    match the_local_load(&row.path, held, the_route) {
        Load::Opened => Supplied::TheKeyOpened,
        Load::NoChannelCouldBeArranged { why } => Supplied::TheLoadCouldNotBeRun { why },
        Load::Refused => {
            // **§9.4: the enumeration is durable before the erasure runs.**
            // This act's units are smaller than itself, so the record is owed
            // by that document's rule rather than chosen here, and it
            // enumerates them **by role** — carrying no path, no fingerprint,
            // no name and no comment.
            let act = Action::APassphraseThatStoppedWorking;
            let mut records = crate::keys::TheOutcomeRecordsLent(record_lender);
            let Ok(mut record) = Writing::open(&mut records, act.as_action(), &act.roles()) else {
                // The act does not start without its enumeration, and the
                // passphrase is left exactly as it was.
                return Supplied::TheLoadCouldNotBeRun {
                    why: askpass::CouldNotDeliver {
                        reported: "the outcome record could not be opened".to_string(),
                        said: None,
                    },
                };
            };
            let outcome_record = record.identity().clone();

            let mut marks = crate::keys::TheMarksLent(record_lender);
            let erasure = match row.store_item() {
                Some(item) => erasure::erase(&mut marks, store, &item, deadline),
                None => Erasure::Done,
            };
            let _ = record.a_unit_ended(
                UnitRole::TheErasure.as_role(),
                if erasure == Erasure::Done {
                    UnitOutcome::Done
                } else {
                    UnitOutcome::Failed
                },
            );

            // **The row is not written here.** §5.1's three writes are the
            // erasure's own and they go through the marks adapter — a second
            // write of the row from this side would land on top of them.

            // **§12 act four: a remembered passphrase forgotten because it
            // stopped working.** Written **at the erasure unit's outcome**,
            // which is the moment §5.1's writes have landed. **An interruption
            // inside that unit writes no line**, exactly as it writes no unit
            // outcome and leaves S19's message unsaid (§13 item 19). The line
            // states `Keys` S5's declared trigger — a local load failing after
            // a passing check — and **never a cause read off a diagnostic**.
            record_lender.write_a_diary_line(
                &Act::APassphraseForgotten(APassphraseForgotten {
                    the_rows_path: row.path.clone(),
                    name: row.name.clone(),
                }),
            );

            Supplied::ThePassphraseStoppedWorking {
                erasure,
                outcome_record,
                // **S19: the same act raises the box with the line.**
                ask: TheAsk {
                    server,
                    key: HowTheKeyIsNamed::of(row),
                    the_forgotten_passphrase_line: passphrase::the_line_is_said(true, false),
                },
            }
        }
    }
}

/// **S14: the ask a user-started connect raises where no passphrase is held.**
///
/// **Three already-standing readings raise it and no other combination does**
/// — the key is usable, it is protected, and U9's availability answer is no.
/// Where the first is no, `Servers` P3 applies and the box never appears.
pub fn the_ask_for_a_key_with_nothing_held(
    row: &KeyRow,
    server: ServerContext,
    the_user_started_this_connect: bool,
    an_earlier_acts_erasure_is_on_the_record: bool,
    record_lender: &dyn crate::keys::LendsTheRecord,
    temporary_area: &Path,
) -> Option<TheAsk> {
    let readings = OnThisComputer::over(temporary_area.to_path_buf());
    let checked = usable::check(row, &readings);
    // **`Keys` R1's refresh, at the check that took the readings.**
    keep_what_the_check_read(record_lender, row, &checked);
    let protected = checked.protected.unwrap_or(false);
    if !passphrase::raises_the_ask(&checked.verdict, protected, row, the_user_started_this_connect) {
        return None;
    }
    Some(TheAsk {
        server,
        key: HowTheKeyIsNamed::of(row),
        // **S18 and `Keys` §13 item 19: an ask that follows an interrupted
        // act says no line**, wherever the interruption fell. §9.4's outcome
        // record names its units by role and carries no key, so no caller can
        // find an earlier act's record for this key; every caller in this
        // application hands `false` here.
        the_forgotten_passphrase_line: passphrase::the_line_is_said(
            false,
            an_earlier_acts_erasure_is_on_the_record,
        ),
    })
}

/// What the user typed, taken as S17 takes it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheBoxTook {
    /// **The load succeeding is what puts it.** The put is an independent unit
    /// whose failure costs the attempt nothing.
    TheKeyOpened {
        the_passphrase_is_held: bool,
        /// **`Security and licence` S34's sentence is owed**, this act having
        /// been the first in the run to take the probe — the caller carries it
        /// to its place (step 10b's first audit round, item 7).
        the_sentence_is_owed: bool,
    },
    /// **The load failing puts nothing**, which is the spec's rule that a
    /// passphrase opening nothing is never kept — and it keeps U9's marker from
    /// ever standing for a value that was never any good.
    ItDidNotOpenTheKey,
    /// The check failed at this attempt's own moment, so no load ran.
    TheCheckFailed { verdict: Verdict },
    TheLoadCouldNotBeRun { why: askpass::CouldNotDeliver },
}

/// **S17 and S19: what the user types opens a new attempt**, with its own check
/// and its own single load. It is not a retry: the failed load ended the
/// attempt that supplied the held passphrase.
///
/// `an_earlier_outcome_record` is the record of the act that forgot the old
/// passphrase, where this ask followed one — its store-put unit is what this
/// attempt's success ends. **Where the ask followed no erasure there is no
/// record and none is opened**: this act's units are then not smaller than
/// itself.
#[allow(clippy::too_many_arguments)]
pub fn take_what_the_user_typed(
    row: &KeyRow,
    typed: &Origin,
    an_earlier_outcome_record: Option<&Identity>,
    record_lender: &dyn crate::keys::LendsTheRecord,
    store: &dyn SecretStore,
    deadline: Duration,
    the_route: &TheRoute,
    this_run: &crate::the_store_standing::TheProbeThisRun,
) -> WhatTheBoxTook {
    let mut records = crate::keys::TheOutcomeRecordsLent(record_lender);
    let records = &mut records;
    let readings = OnThisComputer::over(the_route.channel_area.clone());
    let checked = usable::check(row, &readings);
    // **`Keys` R1's refresh, at the check that took the readings.**
    keep_what_the_check_read(record_lender, row, &checked);
    if checked.verdict != Verdict::Usable {
        return WhatTheBoxTook::TheCheckFailed {
            verdict: checked.verdict,
        };
    }

    match the_local_load(&row.path, typed.a_delivery(), the_route) {
        Load::NoChannelCouldBeArranged { why } => WhatTheBoxTook::TheLoadCouldNotBeRun { why },
        Load::Refused => WhatTheBoxTook::ItDidNotOpenTheKey,
        Load::Opened => {
            // **`Security and licence` S26, at §3.3: the first moment in this run
            // when a secret is actually needed.** The user typed a passphrase,
            // it opened the key, and Smoozy is about to hand it to the store.
            let the_sentence_is_owed = crate::the_store_standing::at_the_first_use_of_a_secret(
                this_run,
                record_lender,
                store,
                deadline,
            )
            .is_some_and(|write| write.say_it_once);
            // **`Keys` G5's shape, with the load's own signal as the moment.**
            //
            // **`Keys` S1's subject is its own identity, issued at this first
            // store** (`State, identity and recovery` §2.2), and never the key
            // row's own identifier — which never changes, so reusing it filed
            // every put for that key under one subject for ever and made
            // `Keys` F10's *"a first store takes a **new** secret-store
            // subject"* false by construction.
            let subject = match Identity::issue(IdentityKind::SecretStoreSubject) {
                Ok(issued) => Subject::issued_by_the_local_record(issued.as_text()),
                // **An independent unit whose failure costs the attempt
                // nothing**: the key opened, and the passphrase is simply not
                // held.
                Err(_) => {
                    if let Some(record) = an_earlier_outcome_record {
                        let _ = records.a_unit_ended(
                            record,
                            UnitRole::TheStorePut.as_role(),
                            UnitOutcome::Failed,
                        );
                    }
                    return WhatTheBoxTook::TheKeyOpened {
                        the_passphrase_is_held: false,
                        the_sentence_is_owed,
                    };
                }
            };
            let item = HeldSecret::new(StoredKind::KeyPassphrase, subject.clone());
            let put = store.put(&item, typed.for_a_destination(), deadline).is_ok();
            if put {
                let mut holding = row.clone();
                holding.passphrase = Some(PassphraseReference {
                    subject,
                    owing_an_erase: false,
                });
                record_lender.lend(&mut |connection| {
                    let _ = key_rows::write(connection, &holding);
                });
            }
            if let Some(record) = an_earlier_outcome_record {
                let _ = records.a_unit_ended(
                    record,
                    UnitRole::TheStorePut.as_role(),
                    if put {
                        UnitOutcome::Done
                    } else {
                        UnitOutcome::Failed
                    },
                );
            }
            WhatTheBoxTook::TheKeyOpened {
                the_passphrase_is_held: put,
                the_sentence_is_owed,
            }
        }
    }
}

/// **`Keys` R1 and U7a's first rule: what the row keeps of a check's readings,
/// written back at the check that took them.**
///
/// `Keys` R1 holds *"the name as last read — at entry, and **refreshed from
/// the `.pub` at a check** where one is readable"*, and U7a's first rule
/// decides the fingerprint: **a value read from the public half is kept only
/// where the private file's own read has never succeeded for this row**, so
/// once a private-file read has answered, that value is the row's and the
/// fallback is not consulted again for it.
///
/// **The refresh goes through [`Reading::what_the_row_keeps`] and never writes
/// `fingerprint_now` onto the row.** That function was defined, documented and
/// tested and called by nothing outside its own tests, while every check here
/// discarded both readings — so a row's name never moved after entry, and a
/// row that had only ever held a fallback value never took the private half's
/// own once it started reading.
///
/// **Nothing else on the row moves**: not the path, not N1's bit, not the
/// passphrase reference. A check that read nothing writes nothing.
///
/// **The two screen callers do not exist yet and take this when they are
/// built**; this is the one U10 caller there is today.
fn keep_what_the_check_read(
    record_lender: &dyn crate::keys::LendsTheRecord,
    row: &KeyRow,
    checked: &usable::Checked,
) {
    let mut kept = row.clone();
    if let Some(fresh) = checked.fingerprint_now.clone() {
        kept.fingerprint = Some(Reading::what_the_row_keeps(row.fingerprint.as_ref(), fresh));
    }
    if checked.name_now.is_some() {
        kept.name = checked.name_now.clone();
    }
    if kept.fingerprint != row.fingerprint || kept.name != row.name {
        // **The write is lent for the write alone** (L4): the check above is a
        // tool run and the record was held across it until 2026-09-17.
        record_lender.lend(&mut |connection| {
            let _ = key_rows::write(connection, &kept);
        });
    }
}

/// The `.pub` beside a key, named here only so a caller need not build the
/// path itself.
pub fn the_public_half_of(row: &KeyRow) -> PathBuf {
    locations::companions(&row.path).public_half
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_keys::fingerprint::ReadFrom;
    use smoozy_security_and_licence::plaintext::Plaintext;
    use smoozy_security_and_licence::secret_store::{GetFailure, ProbeOutcome, StoreFailure};
    use smoozy_state::identities::IdentityKind;
    use std::cell::RefCell;

    fn scratch(name: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-supply-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("a scratch directory");
        // **Left exactly as the operating system gave it**, because this is
        // the path handed to `ssh-keygen`. What Smoozy canonicalises is the
        // path it records, and that is the adapter's job rather than a test's.
        path
    }

    fn a_record() -> rusqlite::Connection {
        let connection = rusqlite::Connection::open_in_memory().expect("a record");
        for migration in crate::record::migrations::ALL {
            crate::record::migrations::run_one_for_tests(&connection, migration)
                .expect("a migration");
        }
        connection
    }

    #[derive(Default)]
    struct AStore {
        puts: RefCell<Vec<HeldSecret>>,
        erased: RefCell<Vec<HeldSecret>>,
        /// What a get answers. `None` keeps the older tests' answer, which is
        /// that nothing is held.
        get: Option<Result<Vec<u8>, GetFailure>>,
    }

    impl SecretStore for AStore {
        fn put(
            &self,
            item: &HeldSecret,
            _value: Plaintext,
            _deadline: Duration,
        ) -> Result<(), StoreFailure> {
            self.puts.borrow_mut().push(item.clone());
            Ok(())
        }
        fn get(&self, _item: &HeldSecret, _deadline: Duration) -> Result<Plaintext, GetFailure> {
            match &self.get {
                Some(Ok(value)) => Ok(Plaintext::new(value.clone())),
                Some(Err(failure)) => Err(*failure),
                None => Err(GetFailure::NotThere),
            }
        }
        fn erase(&self, item: &HeldSecret, _deadline: Duration) -> Result<(), StoreFailure> {
            self.erased.borrow_mut().push(item.clone());
            Ok(())
        }
        fn probe(&self, _deadline: Duration) -> ProbeOutcome {
            ProbeOutcome::RoundTripWorked
        }
        fn enumerate(&self, _deadline: Duration) -> Result<Vec<HeldSecret>, StoreFailure> {
            Ok(Vec::new())
        }
    }

    /// **This test binary is never the helper**, so no test here drives the
    /// route: the one that does is at `tests/askpass_route.rs`, with the built
    /// application's own path.
    fn a_route(area: &Path) -> TheRoute {
        TheRoute::with_the_helper_at(
            area.to_path_buf(),
            PathBuf::from("/nonexistent/never-run-from-a-unit-test"),
        )
    }

    fn there_is_a_generator() -> bool {
        !matches!(
            super::super::readings::run(&["-h".to_string()]),
            smoozy_keys::usable::ToolAnswer::NoGenerator
        )
    }

    /// A real key with a real passphrase cannot be made from a unit test — the
    /// askpass helper is Smoozy's own binary re-entered — so this makes an
    /// unprotected one, which is enough for the two things these tests are
    /// about: the ask's own three readings, and the check running before any
    /// load.
    fn a_real_key(at: &Path, name: &str) -> PathBuf {
        let path = at.join(name);
        let printed = super::super::readings::run(&[
            "-t".to_string(),
            "ed25519".to_string(),
            "-C".to_string(),
            "someone@their-mac".to_string(),
            "-N".to_string(),
            String::new(),
            "-f".to_string(),
            path.display().to_string(),
        ]);
        assert!(matches!(
            printed,
            smoozy_keys::usable::ToolAnswer::Printed(_)
        ));
        path
    }

    fn a_row(path: &Path) -> KeyRow {
        KeyRow::entering(
            Identity::issue(IdentityKind::LocalKey).expect("randomness"),
            path.to_path_buf(),
            None,
            None,
            false,
        )
        .expect("a row")
    }

    /// **S14: the box never appears for a key with no passphrase on it** —
    /// where the key is not protected there is nothing to ask for.
    #[test]
    fn an_unprotected_key_raises_no_ask() {
        if !there_is_a_generator() {
            return;
        }
        let area = scratch("noask");
        let path = a_real_key(&area, "plain");
        let row = a_row(&path);
        assert_eq!(
            the_ask_for_a_key_with_nothing_held(
                &row,
                ServerContext::default(),
                true,
                false,
                &crate::keys::TheRecordItself::over(&a_record()),
                &area
            ),
            None
        );
        let _ = std::fs::remove_dir_all(&area);
    }

    /// **S14: and never on a connection Smoozy makes on its own**, by
    /// construction rather than by suppression.
    #[test]
    fn an_automatic_connect_raises_no_ask_whatever_the_key_is() {
        if !there_is_a_generator() {
            return;
        }
        let area = scratch("automatic");
        let path = a_real_key(&area, "plain");
        let row = a_row(&path);
        assert_eq!(
            the_ask_for_a_key_with_nothing_held(
                &row,
                ServerContext::default(),
                false,
                false,
                &crate::keys::TheRecordItself::over(&a_record()),
                &area
            ),
            None
        );
        let _ = std::fs::remove_dir_all(&area);
    }

    /// **`Keys` S7: a store that will not answer separates nothing, so it
    /// forgets nothing and asks for nothing.**
    #[test]
    fn the_get_asks_only_where_the_item_is_gone_and_forgets_on_neither_other() {
        let area = scratch("the get");
        let bare = a_row(&area.join("a key"));
        let mut row = bare.clone();
        row.passphrase = Some(PassphraseReference {
            subject: Subject::issued_by_the_local_record("s-get"),
            owing_an_erase: false,
        });
        let deadline = Duration::from_secs(2);

        // No reference on the row at all — U9's answer is no, so the user is
        // asked. Nothing reached the store.
        let store = AStore::default();
        assert!(matches!(
            the_held_passphrase(&bare, &store, deadline),
            HeldPassphrase::NoneIsHeld
        ));

        // The store says the item is gone. The one class that asks again —
        // and it is told apart from a row holding nothing, because the stale
        // reference has to be erased (`Keys` U9).
        let store = AStore {
            get: Some(Err(GetFailure::NotThere)),
            ..AStore::default()
        };
        assert!(matches!(
            the_held_passphrase(&row, &store, deadline),
            HeldPassphrase::GoneFromTheStore
        ));

        // Locked, and unavailable. Both are ordinary failures: the user is not
        // asked, and nothing is erased.
        for failure in [GetFailure::LockedOrRefused, GetFailure::StoreUnavailable] {
            let store = AStore {
                get: Some(Err(failure)),
                ..AStore::default()
            };
            match the_held_passphrase(&row, &store, deadline) {
                HeldPassphrase::TheStoreFailed { failure: said } => assert_eq!(said, failure),
                other => panic!("an ordinary store failure became {other:?}"),
            }
            assert!(store.erased.borrow().is_empty());
        }

        // And the answering case hands over a value.
        let store = AStore {
            get: Some(Ok(b"a passphrase".to_vec())),
            ..AStore::default()
        };
        assert!(matches!(
            the_held_passphrase(&row, &store, deadline),
            HeldPassphrase::Held(_)
        ));
        let _ = std::fs::remove_dir_all(&area);
    }

    /// **`Keys` S5's order: the check comes first, and a failing check means
    /// no load and nothing forgotten.**
    #[test]
    fn a_missing_file_ends_the_supply_before_any_load_and_forgets_nothing() {
        let area = scratch("missing");
        let row = a_row(&area.join("not there"));
        let connection = a_record();
        let store = AStore::default();
        let held = Origin::what_the_user_typed("whatever");

        let supplied = supply_a_held_passphrase(
            &row,
            held.a_delivery(),
            ServerContext::default(),
            &crate::keys::TheRecordItself::over(&connection),
            &store,
            Duration::from_secs(2),
            &a_route(&area),
        );
        assert!(matches!(supplied, Supplied::TheCheckFailed { .. }));
        // Nothing was erased and no outcome record was opened.
        assert!(store.erased.borrow().is_empty());
        let records_written: i64 = connection
            .query_row("SELECT count(*) FROM outcome_record", [], |row| row.get(0))
            .unwrap();
        assert_eq!(records_written, 0);
        let _ = std::fs::remove_dir_all(&area);
    }

    /// **S19's line is said by the act that forgot it.** `Keys`' own function
    /// also answers yes for an earlier act's recorded outcome, and nothing in
    /// this application can find one for a key — `Keys` §13 item 19, widened
    /// 2026-09-18.
    #[test]
    fn the_line_follows_the_erasure_and_not_the_row() {
        assert!(passphrase::the_line_is_said(true, false));
        assert!(passphrase::the_line_is_said(false, true));
        // §13 item 19: an ask that follows an interrupted act leaves nothing
        // to carry a line, and a first ask is what the user meets.
        assert!(!passphrase::the_line_is_said(false, false));
    }

    /// **The store put ends the earlier act's second unit**, which is what
    /// makes one record span the erasure and the ask that followed it.
    #[test]
    fn a_newly_entered_passphrase_ends_the_store_put_unit_of_the_act_that_forgot_the_old_one() {
        if !there_is_a_generator() {
            return;
        }
        let area = scratch("put");
        let path = a_real_key(&area, "plain");
        let mut row = a_row(&path);
        row.fingerprint = None;
        let connection = a_record();
        key_rows::add(&connection, &row).expect("the row");
        let store = AStore::default();
        let mut records = crate::record::outcome_records::OutcomeRecords::over(&connection);

        // The act that forgot the old passphrase, as the supply would have
        // written it.
        let act = Action::APassphraseThatStoppedWorking;
        let record = {
            let mut writing =
                Writing::open(&mut records, act.as_action(), &act.roles()).expect("a record");
            writing
                .a_unit_ended(UnitRole::TheErasure.as_role(), UnitOutcome::Done)
                .expect("the erasure landed");
            writing.identity().clone()
        };

        // The key is unprotected, so the load opens it with any value — which
        // is what this test wants: the put, and the unit it ends.
        let typed = Origin::what_the_user_typed("a passphrase");
        let took = take_what_the_user_typed(
            &row,
            &typed,
            Some(&record),
            &crate::keys::TheRecordItself::over(&connection),
            &store,
            Duration::from_secs(5),
            &a_route(&area),
            &crate::the_store_standing::TheProbeThisRun::new(),
        );

        match took {
            WhatTheBoxTook::TheKeyOpened {
                the_passphrase_is_held,
                ..
            } => {
                assert!(the_passphrase_is_held);
                assert_eq!(store.puts.borrow().len(), 1);
                let read = records.read(&record).unwrap().unwrap();
                assert_eq!(
                    read.the_actions_outcome(),
                    Some(smoozy_state::outcomes::ActionOutcome::Whole)
                );
                // And the row now holds the reference.
                let after = &key_rows::all(&connection).unwrap()[0];
                assert!(after.passphrase.is_some());
            }
            // Where no channel can be arranged the load never ran, which is
            // the Windows answer and is not a failure of this rule.
            WhatTheBoxTook::TheLoadCouldNotBeRun { .. } => {}
            other => panic!("the box took {other:?}"),
        }
        let _ = std::fs::remove_dir_all(&area);
    }

    /// **F-7: a check writes back what it read, through `what_the_row_keeps`.**
    ///
    /// `Reading::what_the_row_keeps` was defined, documented, tested and
    /// called by nothing outside its own tests, and every check here discarded
    /// `fingerprint_now` and `name_now` — so `Keys` R1's *"refreshed from the
    /// `.pub` at a check"* never happened, and a row that had only ever held a
    /// fallback value never took the private half's own once it started
    /// reading.
    #[test]
    fn a_check_refreshes_the_row_and_the_fingerprint_goes_through_what_the_row_keeps() {
        if !there_is_a_generator() {
            return;
        }
        let area = scratch("refresh");
        let path = a_real_key(&area, "plain");
        let connection = a_record();

        // A row holding a **fallback** value and a stale name — which is the
        // state a protected key's row is in before its private half ever reads.
        let mut row = a_row(&path);
        row.fingerprint = Some(Reading {
            value: smoozy_keys::fingerprint::Fingerprint::from_the_printed_form("SHA256:stale")
                .expect("a form"),
            read_from: smoozy_keys::fingerprint::ReadFrom::PublicHalfBeside,
        });
        row.name = smoozy_keys::rows::Name::what_the_user_typed("what it used to be called");
        key_rows::add(&connection, &row).expect("the row");

        // The ask's own check is one of the three, and it takes the readings.
        let _ = the_ask_for_a_key_with_nothing_held(
            &row,
            ServerContext::default(),
            true,
            false,
            &crate::keys::TheRecordItself::over(&connection),
            &area,
        );

        let after = &key_rows::all(&connection).expect("the rows")[0];
        let kept = after.fingerprint.as_ref().expect("a reading");
        // **U7a's first rule: the private half's own read replaces a fallback
        // value the moment it answers.**
        assert_eq!(
            kept.read_from,
            smoozy_keys::fingerprint::ReadFrom::PrivateFile
        );
        assert_ne!(kept.value.as_str(), "SHA256:stale");
        // **`Keys` R1: the name as last read, refreshed at the check.**
        assert_eq!(
            after.name.as_ref().map(|name| name.as_str()),
            Some("someone@their-mac")
        );
        let _ = std::fs::remove_dir_all(&area);
    }

    /// **And the other direction of U7a's first rule, which is the one that
    /// matters**: a fallback value never replaces a private-file one. A row
    /// whose private half reads keeps its own value whatever sits beside it.
    #[test]
    fn a_check_never_writes_a_fallback_value_over_a_private_file_one() {
        let held = Reading {
            value: smoozy_keys::fingerprint::Fingerprint::from_the_printed_form("SHA256:mine")
                .expect("a form"),
            read_from: smoozy_keys::fingerprint::ReadFrom::PrivateFile,
        };
        let fresh = Reading {
            value: smoozy_keys::fingerprint::Fingerprint::from_the_printed_form("SHA256:beside")
                .expect("a form"),
            read_from: smoozy_keys::fingerprint::ReadFrom::PublicHalfBeside,
        };
        assert_eq!(Reading::what_the_row_keeps(Some(&held), fresh), held);
    }

    /// A row whose fingerprint has never read is named by neither, and nothing
    /// is invented in its place.
    #[test]
    fn the_ask_names_what_the_row_has_and_never_a_path() {
        let row = a_row(Path::new("/k/.ssh/smoozy_ed25519"));
        assert_eq!(HowTheKeyIsNamed::of(&row), HowTheKeyIsNamed::ByNeither);
        let _ = ReadFrom::PrivateFile;
    }
}
