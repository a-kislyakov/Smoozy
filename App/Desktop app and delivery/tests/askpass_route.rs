//! The askpass route, end to end, with the real tool and the real helper.
//!
//! **This is an integration test rather than a unit test for one reason**: the
//! helper is Smoozy's own binary, and only an integration test is handed the
//! built application's path. A unit test naming itself as the helper would
//! have `ssh-keygen` re-enter the test harness, which answers a different
//! question and answers it wrongly.
//!
//! **What it proves, on the one computer this project has.** `Keys` G3's route
//! — no terminal, `SSH_ASKPASS` naming the helper, `SSH_ASKPASS_REQUIRE=force`,
//! and the value crossing a channel Smoozy opened rather than an argument, an
//! environment value or a file — makes a key that **carries the passphrase**:
//! the container reads protected, and the tool's own empty-passphrase probe
//! refuses it.
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
//! it belongs to.** `Keys` U2, U7, U7b, U8.
//!
//! **A marker this crate's own plan also defines is deliberately not listed
//! above, whichever document is meant, and is written out with its document
//! at every site instead** — the header list cannot tell the two apart, and a
//! bare one would resolve to a real and wrong clause.
//!
//! **What it does not prove** is any of that on Windows or Linux, which is
//! `Keys` §13 item 3 and is permanently unresolved.

#![cfg(unix)]

use std::io::Write;
use std::os::unix::net::UnixListener;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;

/// The channel variable the application branches on. Named here as a literal
/// on purpose: if the constant in the application ever changes, this test
/// stops arranging the route and says so by failing.
const CHANNEL_VARIABLE: &str = "SMOOZY_ASKPASS_CHANNEL";

fn scratch(name: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("smoozy-askpass-route-{name}-{}", std::process::id()));
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

/// Runs the tool with the route open, serving `value` to every ask of that one
/// run — which is two asks at a generation, the prompt and its confirmation.
fn with_the_route(arguments: &[&str], value: &[u8], area: &Path) -> bool {
    let helper = env!("CARGO_BIN_EXE_smoozy-desktop");
    let channel = area.join("ap");
    let listener = UnixListener::bind(&channel).expect("a channel");
    listener.set_nonblocking(true).expect("non-blocking");

    let mut child = Command::new("ssh-keygen")
        .args(arguments)
        .env("SSH_ASKPASS", helper)
        .env("SSH_ASKPASS_REQUIRE", "force")
        .env(CHANNEL_VARIABLE, &channel)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("the tool runs");

    let ended = loop {
        match listener.accept() {
            Ok((mut asking, _)) => {
                asking.write_all(value).expect("the value crosses");
            }
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                match child.try_wait().expect("the child is watchable") {
                    Some(status) => break status.success(),
                    None => std::thread::sleep(Duration::from_millis(5)),
                }
            }
            Err(error) => panic!("the channel refused: {error}"),
        }
    };
    let _ = std::fs::remove_file(&channel);
    ended
}

#[test]
fn a_key_made_over_the_channel_carries_the_passphrase_it_was_given() {
    if !there_is_a_generator() {
        // `Keys` G6's own answer. There is nothing to prove about a route on a
        // computer with no tool to take it.
        return;
    }
    let area = scratch("made");
    let key = area.join("smoozy_ed25519");

    let made = with_the_route(
        &[
            "-q",
            "-t",
            "ed25519",
            "-C",
            "smoozy-test",
            "-f",
            key.to_str().expect("a path"),
        ],
        b"a passphrase the channel delivered",
        &area,
    );
    assert!(made, "the generation succeeded over the askpass route");

    // U2 and U8's own read, on the file the tool just wrote.
    let bytes = std::fs::read(&key).expect("the private half is there");
    let container = smoozy_keys::container::read(&bytes).expect("a private key container");
    assert_eq!(
        container.protected,
        Some(true),
        "the key carries a passphrase, so the channel's value reached the tool"
    );

    // And the tool's own empty-passphrase probe refuses it, which is `Security
    // and licence` §6's confirmation rather than this test's inference.
    let empty = Command::new("ssh-keygen")
        .args(["-y", "-P", "", "-f"])
        .arg(&key)
        .env("SSH_ASKPASS_REQUIRE", "never")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("the probe runs");
    assert!(
        !empty.success(),
        "an empty passphrase does not open a key the route protected"
    );

    // The other half of the same proof: the value that was delivered does open
    // it, over the same route, at the one `-y` load `Keys` S5 makes.
    let loaded = with_the_route(
        &["-y", "-f", key.to_str().expect("a path")],
        b"a passphrase the channel delivered",
        &area,
    );
    assert!(loaded, "the delivered passphrase opens the key it made");

    let _ = std::fs::remove_dir_all(&area);
}

/// **Step 10b's first audit round, item 7: a key made with a passphrase on a
/// computer with nowhere to keep it says so once** — `Security and licence`
/// S34's sentence, carried to the new-key dialog's last step — **and a second
/// key made in the same run does not say it again.** The act used to throw the
/// writer's *say it once* away.
#[test]
fn a_key_made_where_there_is_nowhere_to_keep_its_passphrase_says_so_once() {
    use smoozy_desktop::keys::{askpass, making};
    use smoozy_security_and_licence::held::HeldSecret;
    use smoozy_security_and_licence::plaintext::{Origin, Plaintext};
    use smoozy_security_and_licence::secret_store::{
        GetFailure, ProbeOutcome, SecretStore, StoreFailure,
    };

    if !there_is_a_generator() {
        return;
    }

    /// **Nowhere to remember**: every put fails and so does the probe.
    struct NowhereToKeepIt;
    impl SecretStore for NowhereToKeepIt {
        fn put(&self, _: &HeldSecret, _: Plaintext, _: Duration) -> Result<(), StoreFailure> {
            Err(StoreFailure::StoreUnavailable)
        }
        fn get(&self, _: &HeldSecret, _: Duration) -> Result<Plaintext, GetFailure> {
            Err(GetFailure::NotThere)
        }
        fn erase(&self, _: &HeldSecret, _: Duration) -> Result<(), StoreFailure> {
            Err(StoreFailure::StoreUnavailable)
        }
        fn probe(&self, _: Duration) -> ProbeOutcome {
            ProbeOutcome::RoundTripFailed
        }
        fn enumerate(&self, _: Duration) -> Result<Vec<HeldSecret>, StoreFailure> {
            Ok(Vec::new())
        }
    }

    let area = scratch("once");
    let connection = rusqlite::Connection::open_in_memory().expect("a record");
    smoozy_desktop::record::migrations::bring_up_to_date(
        &connection,
        std::path::Path::new("a record held in memory"),
        false,
    )
    .expect("it migrates");
    let this_run = smoozy_desktop::the_store_standing::TheProbeThisRun::new();
    let typed = Origin::what_the_user_typed("a passphrase with nowhere to go");
    let a_key_is_made = |folder: &str| {
        making::generate(
            &making::TheAsk {
                key_location: area.join(folder),
                key_type: smoozy_keys::types::KeyType::Ed25519,
                comment: "someone@their-mac".to_string(),
                passphrase: Some(&typed),
                the_default_name_now: "someone@their-mac".to_string(),
                the_route: askpass::TheRoute::with_the_helper_at(
                    area.clone(),
                    PathBuf::from(env!("CARGO_BIN_EXE_smoozy-desktop")),
                ),
            },
            &smoozy_desktop::keys::TheRecordItself::over(&connection),
            &NowhereToKeepIt,
            Duration::from_secs(20),
            &this_run,
        )
        .expect("the act ran")
    };

    let first = a_key_is_made("a");
    assert!(matches!(first.pair, making::ThePair::Made { .. }), "{:?}", first.pair);
    assert!(!first.the_passphrase_is_held);
    assert!(first.the_sentence_is_owed, "the first key made says it");
    let second = a_key_is_made("b");
    assert!(matches!(second.pair, making::ThePair::Made { .. }));
    assert!(!second.the_sentence_is_owed, "**and a second one does not**");

    let _ = std::fs::remove_dir_all(&area);
}

/// **`Keys` G4, F-1's own test: a generation *with a passphrase*, driven
/// through the real helper.**
///
/// **This is the test that was missing and the fault it would have caught was
/// the sharpest in the round.** `verify_or_undo` took the private half's own
/// fingerprint line alone, which U7b's `-y -f` refuses for a key that has just
/// been protected — so `Keys` G4 answered *the fingerprint would not read*,
/// the undo unlinked the private half, the `.pub` and the certificate, and no
/// row was written. **Every key made with a passphrase failed and was
/// destroyed a moment after it was made.** The route itself was proved by the
/// two tests above and `generate` was never driven through it, which is
/// exactly where the gap sat.
///
/// **What it proves.** The pair is made and **verified** — `Keys` G4's four
/// checks, including U8's protection reading agreeing with what was asked; the
/// row is written with the fingerprint the composite reading answered; and
/// `Keys` G5's store put lands with a subject of its own.
#[test]
fn a_generation_with_a_passphrase_is_verified_and_rowed_rather_than_destroyed() {
    use std::cell::RefCell;
    use smoozy_desktop::keys::{askpass, making};
    use smoozy_security_and_licence::held::HeldSecret;
    use smoozy_security_and_licence::plaintext::{Origin, Plaintext};
    use smoozy_security_and_licence::secret_store::{
        GetFailure, ProbeOutcome, SecretStore, StoreFailure,
    };
    use smoozy_state::outcomes::Keeps;

    if !there_is_a_generator() {
        return;
    }

    /// A store that remembers what it was given. The real three adapters have
    /// their own tests; what this one is for is `Keys` G5's unit and its
    /// subject.
    #[derive(Default)]
    struct AStore {
        puts: RefCell<Vec<HeldSecret>>,
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
            Err(GetFailure::NotThere)
        }
        fn erase(&self, _item: &HeldSecret, _deadline: Duration) -> Result<(), StoreFailure> {
            Ok(())
        }
        fn probe(&self, _deadline: Duration) -> ProbeOutcome {
            ProbeOutcome::RoundTripWorked
        }
        fn enumerate(&self, _deadline: Duration) -> Result<Vec<HeldSecret>, StoreFailure> {
            Ok(Vec::new())
        }
    }

    // **A short scratch name on purpose.** A Unix socket's path has a length
    // limit far shorter than a path's, the system's temporary directory is
    // already long on this machine, and a name that overran it would fail the
    // route rather than the thing under test.
    let area = scratch("gen");
    let location = area.join(".ssh");

    let connection = rusqlite::Connection::open_in_memory().expect("a record");
    // **Migrated the way a real record is**, rather than by running every
    // statement by hand: version 16 decides what it runs from the record in
    // front of it, so a hand-run of the list is no longer the same act.
    smoozy_desktop::record::migrations::bring_up_to_date(
        &connection,
        std::path::Path::new("a record held in memory"),
        false,
    )
    .expect("it migrates");
    let store = AStore::default();
    let typed = Origin::what_the_user_typed("a passphrase the channel delivered");

    let made = making::generate(
        &making::TheAsk {
            key_location: location.clone(),
            key_type: smoozy_keys::types::KeyType::Ed25519,
            comment: "someone@their-mac".to_string(),
            passphrase: Some(&typed),
            the_default_name_now: "someone@their-mac".to_string(),
            // **The real helper**: the built application's own binary, which is
            // what `SSH_ASKPASS` names and what the tool execs. A test binary
            // here would have `ssh-keygen` re-enter the test harness.
            the_route: askpass::TheRoute::with_the_helper_at(
                area.clone(),
                PathBuf::from(env!("CARGO_BIN_EXE_smoozy-desktop")),
            ),
        },
        &smoozy_desktop::keys::TheRecordItself::over(&connection),
        &store,
        Duration::from_secs(20),
        &smoozy_desktop::the_store_standing::TheProbeThisRun::new(),
    )
    .expect("the act ran");

    // **The pair is made and verified**, where before it was made and then
    // unlinked.
    let at = match &made.pair {
        making::ThePair::Made { at } => at.clone(),
        other => panic!("the pair was {other:?}"),
    };
    assert!(at.exists(), "the private half stands");
    assert!(at.with_extension("pub").exists(), "and its public half");

    // **U8's protection reading, on the file the tool just wrote**: the key
    // really carries the passphrase, which is what `Keys` G4 compared against
    // what was asked.
    let container =
        smoozy_keys::container::read(&std::fs::read(&at).expect("bytes")).expect("a container");
    assert_eq!(container.protected, Some(true));

    // **The row is written**, with the fingerprint U7's composite reading
    // answered — from the `.pub` beside it, since the private half's own read is
    // refused for a protected key. It is recorded as the fallback value it is.
    let rows = smoozy_desktop::record::key_rows::all(&connection).expect("the rows");
    assert_eq!(rows.len(), 1, "a row was written for the key");
    assert_eq!(rows[0].path, at);
    assert_eq!(
        rows[0].fingerprint.as_ref().map(|reading| reading.read_from),
        Some(smoozy_keys::fingerprint::ReadFrom::PublicHalfBeside),
        "a protected key's private half does not read, so U7's fallback answered"
    );

    // **`Keys` G5: the passphrase is put, under a subject of its own** — never
    // the key row's own identifier.
    assert!(made.the_passphrase_is_held);
    assert_eq!(store.puts.borrow().len(), 1);
    let put = store.puts.borrow()[0].clone();
    assert_ne!(
        put.subject.as_str(),
        rows[0].identifier.as_text(),
        "the secret-store subject is its own identity, issued at first store"
    );
    assert_eq!(
        rows[0]
            .passphrase
            .as_ref()
            .expect("the row holds the reference")
            .subject,
        put.subject,
        "and the row records the subject the put used"
    );

    // §9.4: four units and the act whole.
    let records = smoozy_desktop::record::outcome_records::OutcomeRecords::over(&connection);
    let record = records.read(&made.outcome_record).unwrap().unwrap();
    assert_eq!(
        record.the_actions_outcome(),
        Some(smoozy_state::outcomes::ActionOutcome::Whole)
    );

    let _ = std::fs::remove_dir_all(&area);
}

#[test]
fn a_wrong_passphrase_fails_the_load_once_rather_than_being_asked_again() {
    if !there_is_a_generator() {
        return;
    }
    let area = scratch("wrong");
    let key = area.join("smoozy_ed25519");

    assert!(with_the_route(
        &[
            "-q",
            "-t",
            "ed25519",
            "-C",
            "smoozy-test",
            "-f",
            key.to_str().expect("a path"),
        ],
        b"the right one",
        &area,
    ));

    // `Keys` S5 and `Keys` S6: the load runs once and the tool does not loop.
    // A wrong
    // value ends the attempt rather than provoking a second ask, which is what
    // makes the failure attributable to the passphrase by elimination.
    let loaded = with_the_route(
        &["-y", "-f", key.to_str().expect("a path")],
        b"the wrong one",
        &area,
    );
    assert!(!loaded, "a wrong passphrase fails the load");

    let _ = std::fs::remove_dir_all(&area);
}
