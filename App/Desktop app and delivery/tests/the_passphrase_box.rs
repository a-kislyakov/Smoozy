//! **What the passphrase box takes, against a real protected key** — `Keys`
//! S16 and S17.
//!
//! **This is an integration test rather than a unit test for the same one
//! reason `tests/askpass_route.rs` is**: the askpass helper is Smoozy's own
//! binary, and only an integration test is handed the built application's path.
//! A unit test naming itself as the helper would have `ssh-keygen` re-enter the
//! test harness, which answers a different question and answers it wrongly.
//!
//! **What it proves, on the one computer this project has.** A key is made with
//! a passphrase over `Keys` G3's route; then `Keys` S5's supply is run against
//! it twice — **once with the passphrase that opens it and once with one that
//! does not** — and the two endings are the two S17 names. **The load
//! succeeding is what puts it** to the store, and **the load failing puts
//! nothing**, which is the spec's rule that a passphrase opening nothing is
//! never kept.
//!
//! **Why it is worth its own file.** S16 and S17 are the two clauses of `Keys`
//! most likely to be subtly wrong, and both were kept by code that had never
//! been driven end to end: `take_what_the_user_typed` had unit tests over a
//! store that answered whatever they asked it, and no test anywhere had run the
//! real tool against a real protected key through the real channel. **The one
//! thing that separates a passphrase which opens a key from one which does not
//! is that run**, and until this file it was the one step nothing exercised.
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
//! it belongs to.** `Keys` S16, S17, U8.
//!
//! **`Keys` G3, G4, G5 and S5 are deliberately not listed above**, this crate's
//! own plan defining a clause of each of those short names too, and they are
//! written out with their document at every site instead.
//!
//! **A marker this crate's own plan also defines is deliberately not listed
//! above, whichever document is meant, and is written out with its document
//! at every site instead** — the header list cannot tell the two apart, and a
//! bare one would resolve to a real and wrong clause.
//!
//! **What it does not prove** is any of it on Windows or Linux, which is
//! `Keys` §13 items 2 and 3 and is permanently unresolved.

#![cfg(unix)]

use std::cell::RefCell;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Duration;

use smoozy_desktop::keys::askpass::TheRoute;
use smoozy_desktop::keys::supply::{self, WhatTheBoxTook};
use smoozy_keys::rows::KeyRow;
use smoozy_security_and_licence::held::HeldSecret;
use smoozy_security_and_licence::plaintext::{Origin, Plaintext};
use smoozy_security_and_licence::secret_store::{
    GetFailure, ProbeOutcome, SecretStore, StoreFailure,
};
use smoozy_state::identities::{Identity, IdentityKind};

const THE_PASSPHRASE: &str = "the one that opens it";
const A_WRONG_ONE: &str = "not the one that opens it";

fn scratch(name: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("smoozy-passphrase-box-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&path);
    std::fs::create_dir_all(&path).expect("a scratch directory");
    path
}

fn there_is_a_generator() -> bool {
    Command::new("ssh-keygen")
        .arg("-?")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok()
}

/// A store that remembers what it was given. **The real three adapters have
/// their own tests**; what this one is for is `Keys` G5's and `Keys` S17's unit
/// — whether a put happened at all, and under a subject of its own.
#[derive(Default)]
struct WhatWasPut(RefCell<Vec<HeldSecret>>);

impl SecretStore for WhatWasPut {
    fn put(
        &self,
        item: &HeldSecret,
        _value: Plaintext,
        _deadline: Duration,
    ) -> Result<(), StoreFailure> {
        self.0.borrow_mut().push(item.clone());
        Ok(())
    }

    fn get(&self, _item: &HeldSecret, _deadline: Duration) -> Result<Plaintext, GetFailure> {
        Err(GetFailure::NotThere)
    }

    fn erase(&self, _item: &HeldSecret, _deadline: Duration) -> Result<(), StoreFailure> {
        Ok(())
    }

    fn probe(&self, _deadline: Duration) -> ProbeOutcome {
        ProbeOutcome::RoundTripWorked
    }

    fn enumerate(&self, _deadline: Duration) -> Result<Vec<HeldSecret>, StoreFailure> {
        Ok(self.0.borrow().clone())
    }
}

/// The one thing this file needs the application's own machinery for: a record
/// to write a row into.
fn a_record(at: &std::path::Path) -> smoozy_desktop::record::LocalRecord {
    smoozy_desktop::record::LocalRecord::open_at(&at.join("local-record.sqlite3"))
        .expect("a record")
}

/// **A real key with a real passphrase, made over `Keys` G3's real route.**
fn a_protected_key(area: &std::path::Path) -> PathBuf {
    let route = TheRoute::with_the_helper_at(
        area.to_path_buf(),
        PathBuf::from(env!("CARGO_BIN_EXE_smoozy-desktop")),
    );
    let location = area.join(".ssh");
    let typed = Origin::what_the_user_typed(THE_PASSPHRASE);
    let record = a_record(area);
    let store = WhatWasPut::default();

    let made = smoozy_desktop::keys::making::generate(
        &smoozy_desktop::keys::making::TheAsk {
            key_location: location.clone(),
            key_type: smoozy_keys::types::KeyType::Ed25519,
            comment: "smoozy-passphrase-box".to_string(),
            passphrase: Some(&typed),
            the_default_name_now: "somebody@somewhere".to_string(),
            the_route: route,
        },
        &smoozy_desktop::keys::TheRecordItself::over(record.connection()),
        &store,
        Duration::from_secs(20),
        &smoozy_desktop::the_store_standing::TheProbeThisRun::new(),
    )
    .expect("the act ran");

    match made.pair {
        smoozy_desktop::keys::making::ThePair::Made { at } => {
            // **`Keys` G4's verification agreed and `Keys` U8's protection
            // reading says the key really carries one**, which is what makes the
            // rest of this file a test of S17 rather than of a plain key.
            let bytes = std::fs::read(&at).expect("the private half");
            let container = smoozy_keys::container::read(&bytes).expect("a container");
            assert_eq!(container.protected, Some(true));
            at
        }
        other => panic!("the pair was {other:?}"),
    }
}

fn a_row_for(at: &std::path::Path) -> KeyRow {
    KeyRow::entering(
        Identity::issue(IdentityKind::LocalKey).expect("randomness"),
        at.to_path_buf(),
        None,
        None,
        false,
    )
    .expect("a row")
}

/// **`Keys` S17: the load succeeding is what puts it, and the load failing puts
/// nothing.**
///
/// **Both halves are one test on purpose.** The failing half alone would pass
/// against a box that never put anything at all, and the succeeding half alone
/// would pass against one that put whatever it was handed — **the two together
/// are what say the put is the load's signal and not the box's.**
#[test]
fn what_the_box_takes_is_put_only_where_the_load_opened_the_key() {
    if !there_is_a_generator() {
        // `Keys` G6's own answer. There is nothing to prove about a route on a
        // computer with no tool to take it.
        return;
    }
    let area = scratch("took");
    let key = a_protected_key(&area);
    let row = a_row_for(&key);
    let record = a_record(&area);
    smoozy_desktop::record::key_rows::add(record.connection(), &row).expect("the row");
    let route = TheRoute::with_the_helper_at(
        area.clone(),
        PathBuf::from(env!("CARGO_BIN_EXE_smoozy-desktop")),
    );

    // **The wrong one first**, so that a put from it would still be standing
    // when the right one is checked.
    let store = WhatWasPut::default();
    let refused = supply::take_what_the_user_typed(
        &row,
        &Origin::what_the_user_typed(A_WRONG_ONE),
        None,
        &smoozy_desktop::keys::TheRecordItself::over(record.connection()),
        &store,
        Duration::from_secs(20),
        &route,
        &smoozy_desktop::the_store_standing::TheProbeThisRun::new(),
    );
    assert_eq!(
        refused,
        WhatTheBoxTook::ItDidNotOpenTheKey,
        "a passphrase that does not open the key was not refused"
    );
    assert!(
        store.0.borrow().is_empty(),
        "**a passphrase that opens nothing is never kept**, and one was put"
    );

    // **And the one that opens it.**
    let opened = supply::take_what_the_user_typed(
        &row,
        &Origin::what_the_user_typed(THE_PASSPHRASE),
        None,
        &smoozy_desktop::keys::TheRecordItself::over(record.connection()),
        &store,
        Duration::from_secs(20),
        &route,
        &smoozy_desktop::the_store_standing::TheProbeThisRun::new(),
    );
    assert!(
        matches!(
            opened,
            WhatTheBoxTook::TheKeyOpened {
                the_passphrase_is_held: true,
                ..
            }
        ),
        "{opened:?}"
    );
    assert_eq!(store.0.borrow().len(), 1);
    // **`Keys` S1: a first store takes a subject of its own** and never the key
    // row's identifier, which never changes.
    let put = store.0.borrow()[0].clone();
    assert_ne!(put.subject.as_str(), row.identifier.as_text());
    // **And the row now carries the reference**, which is `Keys` U9's marker and
    // is what `Servers` P1's second question reads.
    let held = smoozy_desktop::record::key_rows::all(record.connection())
        .expect("the rows")
        .into_iter()
        .find(|one| one.identifier == row.identifier)
        .expect("the row");
    assert!(smoozy_keys::usable::a_passphrase_is_available_without_asking(&held));

    let _ = std::fs::remove_dir_all(&area);
}

/// **`Keys` S5's order, and the check comes first.**
///
/// A key whose file has gone is refused **before any load is attempted**, so
/// nothing reaches the channel and nothing is put. This is the ordering S16
/// fixes, seen from the end that can be watched: the check, then the one local
/// load, then the connection.
#[test]
fn the_check_comes_before_the_load_and_a_failed_check_reaches_no_channel() {
    if !there_is_a_generator() {
        return;
    }
    let area = scratch("order");
    let key = a_protected_key(&area);
    let row = a_row_for(&key);
    let record = a_record(&area);
    // The file goes, so `Keys` U4 clause 1 fails at the attempt's own moment.
    std::fs::remove_file(&key).expect("the private half goes");

    let store = WhatWasPut::default();
    // **The helper is a path that is not there**, so a load that ran at all
    // would fail loudly rather than quietly passing this test.
    let route = TheRoute::with_the_helper_at(
        area.clone(),
        PathBuf::from("/nonexistent/never-run-from-this-test"),
    );
    let took = supply::take_what_the_user_typed(
        &row,
        &Origin::what_the_user_typed(THE_PASSPHRASE),
        None,
        &smoozy_desktop::keys::TheRecordItself::over(record.connection()),
        &store,
        Duration::from_secs(20),
        &route,
        &smoozy_desktop::the_store_standing::TheProbeThisRun::new(),
    );
    assert!(
        matches!(took, WhatTheBoxTook::TheCheckFailed { .. }),
        "the load ran before the check: {took:?}"
    );
    assert!(store.0.borrow().is_empty());

    let _ = std::fs::remove_dir_all(&area);
}
