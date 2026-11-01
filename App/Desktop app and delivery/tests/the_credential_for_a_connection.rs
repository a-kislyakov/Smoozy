//! **`Keys`' supply, swapped into `Servers`' own acting half and driven by
//! `Servers`' own sequence** — with a real key, a real passphrase and a real
//! refusal.
//!
//! **Why this is the honest test of this piece.** `Servers` drives its whole
//! acting half against five stand-ins, and the one that supplies a credential
//! had exactly one implementation in the workspace: a test-only one that
//! answered yes to everything and handed back a path that does not exist.
//! **A real implementation that cannot be swapped for it has the wrong shape**,
//! and `TheActingHalf` makes the swap one line. So this file makes the swap and
//! runs `Servers`' own `sequence::run` through it.
//!
//! **It is an integration test rather than a unit test for the same one reason
//! `tests/the_passphrase_box.rs` is**: the askpass helper is Smoozy's own
//! binary, and only an integration test is handed the built application's path.
//! A unit test naming itself as the helper would have `ssh-keygen` re-enter the
//! test harness.
//!
//! **What is real here and what is not.** The key is real, made by the real
//! tool with a real passphrase; the check is the real `ssh-keygen` reading; the
//! one local load is the real tool fed the real passphrase over the real
//! askpass channel; the erasure and the box are the real ones. **What is stood
//! in for is the far end** — the opener records the credential it was handed
//! and reports that the address did not answer — because opening a socket is
//! `Servers`' seam and not this piece's, and because this crate has no SSH
//! server in its tests.
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
//! it belongs to.** `Keys` S16, S17, S19, U8, U9. `Servers` N8b, P4a.
//!
//! **`Keys` S5 and `Servers` P1a are deliberately not listed above**, this
//! crate's own plan defining a clause of each of those short names too, and
//! they are written out with their document at every site instead.
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
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, SystemTime};

use smoozy_desktop::keys::askpass::TheRoute;
use smoozy_desktop::keys::for_a_connection::{TheCredentialForAConnection, WhyNothingWasSupplied};
use smoozy_keys::rows::{KeyRow, PassphraseReference};
use smoozy_security_and_licence::held::{HeldSecret, StoredKind, Subject};
use smoozy_security_and_licence::plaintext::{Origin, Plaintext};
use smoozy_security_and_licence::secret_store::{
    GetFailure, ProbeOutcome, SecretStore, StoreFailure,
};
use smoozy_server_protocol::transport::ssh::CommandFailure;
use smoozy_server_protocol::transport::{
    CommandResult, CommandString, Credential, DecideHostTrust, Ending, Endpoint, OpenFailure,
};
use smoozy_servers::acting::{
    OpenConnections, SupplyTheCredential, TheActingHalf, TheIdentityRead,
    TheInstallationIdentityRead, Wait, WhatServersPublishes,
};
use smoozy_servers::condition::ObservedValue;
use smoozy_servers::first_connection::ChangedIdentityWarning;
use smoozy_servers::identity::{Host, KeyReference, Port, Username};
use smoozy_servers::key_moments::WhoStartedIt;
use smoozy_servers::repair::{NextAttemptDue, WhatTheRecordSays};
use smoozy_servers::sequence::{self, AnAttempt, Attempt, ConnectionRole, WhereItStopped};
use smoozy_servers::store::{RememberedHostKey, ServerRow};
use smoozy_state::identities::{Identity, IdentityKind, InstallationCheck};

const THE_PASSPHRASE: &str = "the one that opens it";
const A_WRONG_ONE: &str = "not the one that opens it";

// ------------------------------------------------------------------ the scrap

fn scratch(name: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("smoozy-credential-{name}-{}", std::process::id()));
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

/// **Every future driven here completes without ever pending**: the opener
/// answers at once, the identity read is never reached, and the waiter is never
/// called. So the whole executor this file needs is one poll, and it adds no
/// dependency to a crate whose only async runtime is Linux-only.
fn block_on<F: std::future::Future>(future: F) -> F::Output {
    let mut future = Box::pin(future);
    let waker = std::task::Waker::noop();
    let mut context = std::task::Context::from_waker(waker);
    for _ in 0..1_000 {
        if let std::task::Poll::Ready(out) = future.as_mut().poll(&mut context) {
            return out;
        }
        std::thread::yield_now();
    }
    panic!("the sequence waited on something this file does not drive")
}

// ----------------------------------------------------------------- the store

/// A store holding whatever a test planted in it.
#[derive(Default)]
struct AStore(RefCell<Vec<(HeldSecret, Vec<u8>)>>);

impl SecretStore for AStore {
    fn put(
        &self,
        item: &HeldSecret,
        value: Plaintext,
        _deadline: Duration,
    ) -> Result<(), StoreFailure> {
        let bytes = value.expose(<[u8]>::to_vec);
        self.0.borrow_mut().push((item.clone(), bytes));
        Ok(())
    }

    fn get(&self, item: &HeldSecret, _deadline: Duration) -> Result<Plaintext, GetFailure> {
        self.0
            .borrow()
            .iter()
            .find(|(held, _)| held == item)
            .map(|(_, bytes)| Plaintext::new(bytes.clone()))
            .ok_or(GetFailure::NotThere)
    }

    fn erase(&self, item: &HeldSecret, _deadline: Duration) -> Result<(), StoreFailure> {
        self.0.borrow_mut().retain(|(held, _)| held != item);
        Ok(())
    }

    fn probe(&self, _deadline: Duration) -> ProbeOutcome {
        ProbeOutcome::RoundTripWorked
    }

    fn enumerate(&self, _deadline: Duration) -> Result<Vec<HeldSecret>, StoreFailure> {
        Ok(self.0.borrow().iter().map(|(held, _)| held.clone()).collect())
    }
}

/// **A store that refuses every get and answers nothing else.** `Security and
/// licence` §3.0 rule 4's *locked or refused* — the class that is an ordinary
/// failure rather than a secret's own.
struct AStoreThatRefuses;

impl SecretStore for AStoreThatRefuses {
    fn put(
        &self,
        _item: &HeldSecret,
        _value: Plaintext,
        _deadline: Duration,
    ) -> Result<(), StoreFailure> {
        Err(StoreFailure::LockedOrRefused)
    }

    fn get(&self, _item: &HeldSecret, _deadline: Duration) -> Result<Plaintext, GetFailure> {
        Err(GetFailure::LockedOrRefused)
    }

    fn erase(&self, _item: &HeldSecret, _deadline: Duration) -> Result<(), StoreFailure> {
        Err(StoreFailure::LockedOrRefused)
    }

    fn probe(&self, _deadline: Duration) -> ProbeOutcome {
        ProbeOutcome::RoundTripFailed
    }

    fn enumerate(&self, _deadline: Duration) -> Result<Vec<HeldSecret>, StoreFailure> {
        Err(StoreFailure::LockedOrRefused)
    }
}

// ----------------------------------------------------------- the other seams

/// A connection nothing is ever run on. **The opener below never answers with
/// one**, and this exists so the acting half has an associated type.
struct NoConnection;

impl smoozy_servers::readings::CommandChannel for NoConnection {
    async fn run(&mut self, _command: CommandString) -> Result<CommandResult, CommandFailure> {
        unreachable!("no connection is ever opened in this file")
    }
}

impl smoozy_servers::readings::TakesAFreeSpaceReading for NoConnection {
    async fn free_space(&mut self, _path: &str) -> Option<smoozy_servers::hardware::FreeSpace> {
        unreachable!("no connection is ever opened in this file")
    }
}

/// **The far end, stood in for.** It records what it was handed and reports
/// that the address did not answer — which is `Servers`' own `Q9`: a connection
/// that could not be made, **with no claim about the machine's identity**.
#[derive(Default)]
struct WhatTheTransportWasHanded {
    credentials: Vec<(PathBuf, bool)>,
}

impl OpenConnections for WhatTheTransportWasHanded {
    type Connection = NoConnection;

    async fn open(
        &mut self,
        _endpoint: Endpoint,
        credential: Credential,
        _decide: Box<dyn DecideHostTrust>,
        _deadline: Duration,
    ) -> Result<NoConnection, OpenFailure> {
        self.credentials
            .push((credential.private_key_file.clone(), credential.passphrase.is_some()));
        Err(OpenFailure::TheAddressDidNotAnswer)
    }

    fn ended(&mut self, _connection: &NoConnection) -> Option<Ending> {
        None
    }

    async fn wait_until_it_ends(&mut self, _connection: &mut NoConnection) -> Ending {
        Ending::TornDownBySmoozy
    }

    async fn tear_down(&mut self, _connection: NoConnection) -> Ending {
        Ending::TornDownBySmoozy
    }
}

struct TheIdentityIsAbsent;

impl<C: Send> TheInstallationIdentityRead<C> for TheIdentityIsAbsent {
    async fn read(&mut self, _channel: &mut C) -> TheIdentityRead {
        TheIdentityRead {
            check: InstallationCheck::Absent,
            identity_on_the_machine: None,
        }
    }
}

#[derive(Default)]
struct Caught {
    changed_identity_warnings: usize,
    host_keys_recorded: usize,
}

impl WhatServersPublishes for Caught {
    fn the_observed_value(&mut self, _server: &str, _value: ObservedValue) {}
    fn the_next_attempt_due(&mut self, _server: &str, _reading: NextAttemptDue) {}
    fn raise_the_connection_condition(&mut self, _server: &str, _says: WhatTheRecordSays) {}
    fn withdraw_the_connection_condition(&mut self, _server: &str) {}

    fn the_changed_identity_warning(&mut self, _server: &str) {
        self.changed_identity_warnings += 1;
    }

    fn record_the_scanned_host_key(&mut self, _server: &str, _key: RememberedHostKey) {
        self.host_keys_recorded += 1;
    }
}

struct NoWaiting;

impl Wait for NoWaiting {
    async fn until(&mut self, _how_long: Duration) {
        unreachable!("one attempt waits for nothing")
    }

    fn now(&mut self) -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_secs(1_000_000)
    }
}

// -------------------------------------------------------------- the material

fn a_record(at: &Path) -> smoozy_desktop::record::LocalRecord {
    smoozy_desktop::record::LocalRecord::open_at(&at.join("local-record.sqlite3"))
        .expect("a record")
}

fn the_route(area: &Path) -> TheRoute {
    TheRoute::with_the_helper_at(
        area.to_path_buf(),
        PathBuf::from(env!("CARGO_BIN_EXE_smoozy-desktop")),
    )
}

/// **A real key with a real passphrase, made over `Keys` G3's real route.**
fn a_protected_key(area: &Path) -> PathBuf {
    let typed = Origin::what_the_user_typed(THE_PASSPHRASE);
    let record = a_record(area);
    let store = AStore::default();
    let made = smoozy_desktop::keys::making::generate(
        &smoozy_desktop::keys::making::TheAsk {
            key_location: area.join(".ssh"),
            key_type: smoozy_keys::types::KeyType::Ed25519,
            comment: "smoozy-credential".to_string(),
            passphrase: Some(&typed),
            the_default_name_now: "somebody@somewhere".to_string(),
            the_route: the_route(area),
        },
        &smoozy_desktop::keys::TheRecordItself::over(record.connection()),
        &store,
        Duration::from_secs(20),
        &smoozy_desktop::the_store_standing::TheProbeThisRun::new(),
    )
    .expect("the act ran");

    match made.pair {
        smoozy_desktop::keys::making::ThePair::Made { at } => {
            // **`Keys` U8's protection reading says the key really carries
            // one**, which is what makes the rest of this file a test of the
            // passphrase route rather than of a plain key.
            let bytes = std::fs::read(&at).expect("the private half");
            let container = smoozy_keys::container::read(&bytes).expect("a container");
            assert_eq!(container.protected, Some(true));
            at
        }
        other => panic!("the pair was {other:?}"),
    }
}

/// The row, with a passphrase reference against a subject of its own — `Keys`
/// U9's marker, which is what `Servers` P1's second question reads.
fn a_row_holding(at: &Path, subject: &Subject) -> KeyRow {
    let mut row = KeyRow::entering(
        Identity::issue(IdentityKind::LocalKey).expect("randomness"),
        at.to_path_buf(),
        None,
        None,
        false,
    )
    .expect("a row");
    row.passphrase = Some(PassphraseReference {
        subject: subject.clone(),
        owing_an_erase: false,
    });
    row
}

fn a_subject() -> Subject {
    Subject::issued_by_the_local_record(
        Identity::issue(IdentityKind::SecretStoreSubject)
            .expect("randomness")
            .as_text(),
    )
}

fn the_fingerprint_of(row: &KeyRow, area: &Path) -> String {
    let readings = smoozy_desktop::keys::readings::OnThisComputer::over(area.to_path_buf());
    smoozy_keys::usable::check(row, &readings)
        .fingerprint_now
        .expect("the tool read a fingerprint")
        .value
        .as_str()
        .to_string()
}

fn a_server_row(key: KeyReference) -> ServerRow {
    ServerRow::saved(
        Identity::issue(IdentityKind::LocalServer)
            .expect("randomness")
            .as_text(),
        Host::what_the_user_typed("5.180.183.77").expect("a host"),
        Port::what_the_user_typed(33510).expect("a port"),
        Username::what_the_user_typed("root").expect("a username"),
        key,
    )
}

fn an_attempt(row: &ServerRow, who: WhoStartedIt) -> AnAttempt<'_> {
    AnAttempt {
        row,
        who,
        the_first_connection_to_this_server: true,
        role: ConnectionRole::Primary,
        deadline: smoozy_servers::repair::CONNECT_DEADLINE,
    }
}

fn nothing_to_publish(_publishes: &mut Caught) {}

/// **The swap, and it is one line of wiring** — `Servers`' own five-seam value,
/// with `Keys`' real supply in the credentials slot.
fn run_the_sequence<'a>(
    row: &ServerRow,
    who: WhoStartedIt,
    supply: TheCredentialForAConnection<'a>,
) -> (
    Attempt<NoConnection>,
    TheActingHalf<
        WhatTheTransportWasHanded,
        TheCredentialForAConnection<'a>,
        TheIdentityIsAbsent,
        Caught,
        NoWaiting,
    >,
) {
    let mut half = TheActingHalf {
        opener: WhatTheTransportWasHanded::default(),
        credentials: supply,
        installation: TheIdentityIsAbsent,
        publishes: Caught::default(),
        wait: NoWaiting,
    };
    let ended = block_on(sequence::run(
        an_attempt(row, who),
        &mut half,
        &mut ChangedIdentityWarning::none_yet(),
        &mut nothing_to_publish,
    ));
    (ended, half)
}

// ------------------------------------------------------------------ the tests

/// **`Keys` S5's whole ordered supply, run by `Servers`' own sequence, with a
/// real passphrase that opens a real key.**
///
/// What it says: the check ran, the one local load ran over the real channel
/// and opened the key, and **the transport was handed the key's own path with a
/// passphrase beside it**. That last half is the join this piece exists to
/// make: before it, `Servers`' only supply handed over a path that does not
/// exist.
#[test]
fn a_real_passphrase_opens_a_real_key_and_reaches_the_transport() {
    let this_run = smoozy_desktop::the_store_standing::TheProbeThisRun::new();
    if !there_is_a_generator() {
        return;
    }
    let area = scratch("opens");
    let key = a_protected_key(&area);
    let subject = a_subject();
    let row = a_row_holding(&key, &subject);
    let record = a_record(&area);
    smoozy_desktop::record::key_rows::add(record.connection(), &row).expect("the row");

    let store = AStore::default();
    store
        .put(
            &HeldSecret::new(StoredKind::KeyPassphrase, subject),
            Plaintext::new(THE_PASSPHRASE),
            Duration::from_secs(5),
        )
        .expect("the passphrase is held");

    let lent = smoozy_desktop::keys::TheRecordItself::over(record.connection());
    let supply = TheCredentialForAConnection::over(
        &lent,
        &store,
        the_route(&area),
        Duration::from_secs(20),
        &this_run,
    );
    let server = a_server_row(KeyReference {
        local_key_identifier: row.identifier.as_text(),
        fingerprint: the_fingerprint_of(&row, &area),
    });

    let (ended, half) = run_the_sequence(&server, WhoStartedIt::TheUser, supply);

    // **The transport was reached, with the key's own path and a passphrase.**
    assert_eq!(
        half.opener.credentials,
        vec![(key.clone(), true)],
        "the transport was not handed the key and its passphrase"
    );
    // The far end is stood in for, so the attempt ends where the far end is.
    assert!(matches!(
        ended,
        Attempt::ItStopped(WhereItStopped::TheAddressDidNotAnswer)
    ));
    // **`Keys` S17: the load opened the key, so nothing is forgotten.**
    assert_eq!(half.credentials.store.enumerate(Duration::from_secs(1)).expect("the store").len(), 1);
    assert!(half.credentials.the_ask.is_none());

    let _ = std::fs::remove_dir_all(&area);
}

/// **A real refusal: `Keys` S5's local load fails after a passing check, so the
/// held passphrase is attributable and is gone.**
///
/// Three things are asserted together because each alone would pass against
/// code that was wrong in a different way: **the transport is never reached**,
/// **the item is erased** and the row's reference goes with it (`Security and
/// licence` §5.1), and **`Keys` S19's box is raised by the same act**.
#[test]
fn a_passphrase_that_stopped_working_is_forgotten_and_the_box_is_raised() {
    let this_run = smoozy_desktop::the_store_standing::TheProbeThisRun::new();
    if !there_is_a_generator() {
        return;
    }
    let area = scratch("refused");
    let key = a_protected_key(&area);
    let subject = a_subject();
    let row = a_row_holding(&key, &subject);
    let record = a_record(&area);
    smoozy_desktop::record::key_rows::add(record.connection(), &row).expect("the row");

    let store = AStore::default();
    store
        .put(
            &HeldSecret::new(StoredKind::KeyPassphrase, subject),
            Plaintext::new(A_WRONG_ONE),
            Duration::from_secs(5),
        )
        .expect("a passphrase is held");

    let lent = smoozy_desktop::keys::TheRecordItself::over(record.connection());
    let supply = TheCredentialForAConnection::over(
        &lent,
        &store,
        the_route(&area),
        Duration::from_secs(20),
        &this_run,
    );
    let server = a_server_row(KeyReference {
        local_key_identifier: row.identifier.as_text(),
        fingerprint: the_fingerprint_of(&row, &area),
    });

    let (ended, half) = run_the_sequence(&server, WhoStartedIt::TheUser, supply);

    assert!(
        half.opener.credentials.is_empty(),
        "**the load precedes the connection**, and the transport was reached with a \
         passphrase that opens nothing"
    );
    assert!(matches!(
        ended,
        Attempt::ItStopped(WhereItStopped::NoCredentialWasSupplied)
    ));
    assert!(matches!(
        half.credentials.why_nothing_was_supplied,
        Some(WhyNothingWasSupplied::ThePassphraseStoppedWorking { .. })
    ));
    // **`Security and licence` §5.1: the item is erased and the row's reference
    // goes with it**, so `Keys` U9 now answers no.
    assert!(half
        .credentials
        .store
        .enumerate(Duration::from_secs(1))
        .expect("the store")
        .is_empty());
    let held = smoozy_desktop::record::key_rows::all(record.connection())
        .expect("the rows")
        .into_iter()
        .find(|one| one.identifier == row.identifier)
        .expect("the row");
    assert!(!smoozy_keys::usable::a_passphrase_is_available_without_asking(&held));
    // **`Keys` S19: the same act raises the box, with the line.**
    let ask = half.credentials.the_ask.as_ref().expect("the box was raised");
    assert!(ask.the_forgotten_passphrase_line);
    // **`Servers` P4a: three display-only fields and no secret.**
    assert_eq!(ask.server.username.as_deref(), Some("root"));
    assert_eq!(ask.server.host.as_deref(), Some("5.180.183.77"));

    let _ = std::fs::remove_dir_all(&area);
}

/// **`Servers` P3 and P4, and `Keys` S13: a connection Smoozy makes on its own
/// never puts a passphrase prompt on screen, and it keeps that promise by not
/// making the attempt.**
///
/// The same key, the same row with nothing held, driven twice: once by the user
/// and once by Smoozy. **The user's raises `Keys` S14's box; Smoozy's stops at
/// the key check and raises nothing.**
#[test]
fn an_automatic_attempt_is_never_made_where_a_user_started_one_would_ask() {
    let this_run = smoozy_desktop::the_store_standing::TheProbeThisRun::new();
    if !there_is_a_generator() {
        return;
    }
    let area = scratch("whoasks");
    let key = a_protected_key(&area);
    let mut row = a_row_holding(&key, &a_subject());
    // **Nothing is held**, which is `Keys` U9's answer *no*.
    row.passphrase = None;
    let record = a_record(&area);
    smoozy_desktop::record::key_rows::add(record.connection(), &row).expect("the row");
    let reference = KeyReference {
        local_key_identifier: row.identifier.as_text(),
        fingerprint: the_fingerprint_of(&row, &area),
    };
    let server = a_server_row(reference);

    // **Smoozy on its own.** `Servers` P3: no attempt is made at all.
    let store = AStore::default();
    let lent = smoozy_desktop::keys::TheRecordItself::over(record.connection());
    let (ended, half) = run_the_sequence(
        &server,
        WhoStartedIt::SmoozyOnItsOwn,
        TheCredentialForAConnection::over(&lent, &store, the_route(&area), Duration::from_secs(20), &this_run),
    );
    assert!(matches!(
        ended,
        Attempt::ItStopped(WhereItStopped::TheKeyCheck(_))
    ));
    assert!(half.opener.credentials.is_empty());
    assert!(
        half.credentials.the_ask.is_none(),
        "**an automatic attempt put a passphrase prompt on screen**"
    );

    // **And the user's own connect, which may ask.**
    let (ended, half) = run_the_sequence(
        &server,
        WhoStartedIt::TheUser,
        TheCredentialForAConnection::over(&lent, &store, the_route(&area), Duration::from_secs(20), &this_run),
    );
    assert!(matches!(
        ended,
        Attempt::ItStopped(WhereItStopped::NoCredentialWasSupplied)
    ));
    assert!(half.opener.credentials.is_empty());
    let ask = half.credentials.the_ask.as_ref().expect("the box was raised");
    // **`Keys` S19: this ask follows no erase, so it carries no line.**
    assert!(!ask.the_forgotten_passphrase_line);

    let _ = std::fs::remove_dir_all(&area);
}

/// **`Keys` S16's order carried across the box: what the user typed opens the
/// attempt, and the connection is made whether or not the value was
/// remembered.**
///
/// **`Keys` S9 is what this is for.** On a computer with nowhere to remember,
/// the put fails and nothing is ever held, and *"a connect the user starts
/// works normally, asking for the passphrase each time"* — which is only true
/// if the typed value reaches this attempt. The store here answers nothing on a
/// get, so a supply that went back to it would fail.
#[test]
fn what_the_box_took_opens_the_attempt_it_was_raised_for() {
    let this_run = smoozy_desktop::the_store_standing::TheProbeThisRun::new();
    if !there_is_a_generator() {
        return;
    }
    let area = scratch("typed");
    let key = a_protected_key(&area);
    let mut row = a_row_holding(&key, &a_subject());
    row.passphrase = None;
    let record = a_record(&area);
    smoozy_desktop::record::key_rows::add(record.connection(), &row).expect("the row");
    let server = a_server_row(KeyReference {
        local_key_identifier: row.identifier.as_text(),
        fingerprint: the_fingerprint_of(&row, &area),
    });

    let store = AStore::default();
    let lent = smoozy_desktop::keys::TheRecordItself::over(record.connection());
    let typed = Origin::what_the_user_typed(THE_PASSPHRASE);
    let mut supply = TheCredentialForAConnection::over(
        &lent,
        &store,
        the_route(&area),
        Duration::from_secs(20),
        &this_run,
    );
    supply.what_the_box_took = Some(&typed);

    let (ended, half) = run_the_sequence(&server, WhoStartedIt::TheUser, supply);

    assert_eq!(
        half.opener.credentials,
        vec![(key.clone(), true)],
        "what the box took did not reach the attempt it was raised for"
    );
    assert!(matches!(
        ended,
        Attempt::ItStopped(WhereItStopped::TheAddressDidNotAnswer)
    ));
    // **`Keys` S17: the load succeeding is what puts it**, under a subject of
    // its own.
    let put = half.credentials.store.enumerate(Duration::from_secs(1)).expect("the store");
    assert_eq!(put.len(), 1);
    assert_ne!(put[0].subject.as_str(), row.identifier.as_text());

    let _ = std::fs::remove_dir_all(&area);
}

/// **Step 10b's first audit round, item 3: the passphrase box's *Connect*
/// connects, on a computer with nowhere to remember.** The box's own route —
/// `smoozy_desktop::servers::a_period::the_credential_for_this_connect`, which
/// every connect composes its supply through and which the box's *Connect*
/// hands what was typed — is driven against a store that holds nothing, whose
/// every put fails. **The transport is handed the key with a passphrase beside
/// it, and the store is never read**: what was typed reaches the one attempt it
/// was raised for, borrowed for the connect and never read back out of the
/// store. Before the repair the box put the value into the store and connected
/// nothing, so on such a computer a protected key could never connect.
#[test]
fn the_boxs_connect_carries_what_was_typed_to_its_attempt_where_nothing_is_kept() {
    let this_run = smoozy_desktop::the_store_standing::TheProbeThisRun::new();
    if !there_is_a_generator() {
        return;
    }

    /// Nowhere to remember, and it counts every read.
    #[derive(Default)]
    struct NowhereToRemember {
        gets: std::cell::Cell<usize>,
    }
    impl SecretStore for NowhereToRemember {
        fn put(&self, _: &HeldSecret, _: Plaintext, _: Duration) -> Result<(), StoreFailure> {
            Err(StoreFailure::StoreUnavailable)
        }
        fn get(&self, _: &HeldSecret, _: Duration) -> Result<Plaintext, GetFailure> {
            self.gets.set(self.gets.get() + 1);
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

    let area = scratch("the-boxs-connect");
    let key = a_protected_key(&area);
    let mut row = a_row_holding(&key, &a_subject());
    row.passphrase = None;
    let record = a_record(&area);
    smoozy_desktop::record::key_rows::add(record.connection(), &row).expect("the row");
    // **A fresh installation on a computer with nowhere to remember**: the key
    // above was made over a store that answers, which wrote its standing into
    // this same record, so that standing is taken away first.
    record
        .connection()
        .execute("DELETE FROM store_standing", [])
        .expect("the standing goes");
    let server = a_server_row(KeyReference {
        local_key_identifier: row.identifier.as_text(),
        fingerprint: the_fingerprint_of(&row, &area),
    });

    let store = NowhereToRemember::default();
    let lent = smoozy_desktop::keys::TheRecordItself::over(record.connection());
    let typed = Origin::what_the_user_typed(THE_PASSPHRASE);
    let supply = smoozy_desktop::servers::a_period::the_credential_for_this_connect(
        &lent,
        &store,
        the_route(&area),
        Duration::from_secs(20),
        &this_run,
        Some(&typed),
    );

    let (ended, half) = run_the_sequence(&server, WhoStartedIt::TheUser, supply);

    assert_eq!(
        half.opener.credentials,
        vec![(key.clone(), true)],
        "**the box's *Connect* connected nothing**: what was typed did not reach \
         the attempt it was raised for"
    );
    assert!(matches!(
        ended,
        Attempt::ItStopped(WhereItStopped::TheAddressDidNotAnswer)
    ));
    assert_eq!(store.gets.get(), 0, "**never read back out of the store**");
    assert!(
        half.credentials.the_ask.is_none(),
        "**no second box**: the value the box took was used"
    );
    assert!(
        half.credentials.the_sentence_is_owed,
        "`Security and licence` S34: this connect found there is nowhere to keep it"
    );

    let _ = std::fs::remove_dir_all(&area);
}

/// **`Keys` S17 and S19: a value that opens nothing is never kept, and a second
/// failure is a failed load reported rather than a third box.**
#[test]
fn what_the_box_took_that_opens_nothing_is_never_kept_and_raises_no_second_box() {
    let this_run = smoozy_desktop::the_store_standing::TheProbeThisRun::new();
    if !there_is_a_generator() {
        return;
    }
    let area = scratch("typedwrong");
    let key = a_protected_key(&area);
    let mut row = a_row_holding(&key, &a_subject());
    row.passphrase = None;
    let record = a_record(&area);
    smoozy_desktop::record::key_rows::add(record.connection(), &row).expect("the row");
    let server = a_server_row(KeyReference {
        local_key_identifier: row.identifier.as_text(),
        fingerprint: the_fingerprint_of(&row, &area),
    });

    let store = AStore::default();
    let lent = smoozy_desktop::keys::TheRecordItself::over(record.connection());
    let typed = Origin::what_the_user_typed(A_WRONG_ONE);
    let mut supply = TheCredentialForAConnection::over(
        &lent,
        &store,
        the_route(&area),
        Duration::from_secs(20),
        &this_run,
    );
    supply.what_the_box_took = Some(&typed);

    let (ended, half) = run_the_sequence(&server, WhoStartedIt::TheUser, supply);

    assert!(half.opener.credentials.is_empty());
    assert!(matches!(
        ended,
        Attempt::ItStopped(WhereItStopped::NoCredentialWasSupplied)
    ));
    assert!(matches!(
        half.credentials.why_nothing_was_supplied,
        Some(WhyNothingWasSupplied::WhatWasTypedDidNotOpenTheKey)
    ));
    assert!(half
        .credentials
        .store
        .enumerate(Duration::from_secs(1))
        .expect("the store")
        .is_empty());
    assert!(
        half.credentials.the_ask.is_none(),
        "**nothing retries and no second box is raised inside one attempt**"
    );

    let _ = std::fs::remove_dir_all(&area);
}

/// **`Keys` S7 and `Security and licence` §3.0 rule 4: a store that will not
/// answer is an ordinary failure and not a secret's own.**
///
/// The row says a passphrase is held and the store refuses the get. **The user
/// is not asked, the passphrase is not forgotten, and the row keeps its
/// reference** — which is the whole of rule 4's separation. A supply that
/// treated every get failure as *not there* would ask the user to retype a
/// secret the machine still holds, and this test fails against that.
#[test]
fn a_store_that_will_not_answer_asks_nobody_and_forgets_nothing() {
    let this_run = smoozy_desktop::the_store_standing::TheProbeThisRun::new();
    if !there_is_a_generator() {
        return;
    }
    let area = scratch("storefails");
    let key = a_protected_key(&area);
    let subject = a_subject();
    let row = a_row_holding(&key, &subject);
    let record = a_record(&area);
    smoozy_desktop::record::key_rows::add(record.connection(), &row).expect("the row");
    let server = a_server_row(KeyReference {
        local_key_identifier: row.identifier.as_text(),
        fingerprint: the_fingerprint_of(&row, &area),
    });

    // **The store is locked or refused** — not *not there*, which is the one
    // class rule 4 lets ask again.
    let store = AStoreThatRefuses;
    let lent = smoozy_desktop::keys::TheRecordItself::over(record.connection());
    let (ended, half) = run_the_sequence(
        &server,
        WhoStartedIt::TheUser,
        TheCredentialForAConnection::over(&lent, &store, the_route(&area), Duration::from_secs(20), &this_run),
    );

    assert!(matches!(
        ended,
        Attempt::ItStopped(WhereItStopped::NoCredentialWasSupplied)
    ));
    assert!(half.opener.credentials.is_empty());
    assert!(matches!(
        half.credentials.why_nothing_was_supplied,
        Some(WhyNothingWasSupplied::TheStoreWouldNotAnswer {
            failure: GetFailure::LockedOrRefused
        })
    ));
    assert!(
        half.credentials.the_ask.is_none(),
        "**the user was asked to retype a secret the machine still holds**"
    );
    let held = smoozy_desktop::record::key_rows::all(record.connection())
        .expect("the rows")
        .into_iter()
        .find(|one| one.identifier == row.identifier)
        .expect("the row");
    assert!(
        smoozy_keys::usable::a_passphrase_is_available_without_asking(&held),
        "the passphrase reference was dropped by an ordinary store failure"
    );

    let _ = std::fs::remove_dir_all(&area);
}

/// **A passphrase gone from the store while the row still says it is held:
/// the store's *not there* asks again, whatever the row's marker says.**
///
/// *(Added 2026-09-18, step 10b, "a passphrase that is gone".)* A user who
/// deletes Smoozy's item in Keychain Access leaves the key row naming a
/// subject the store no longer holds. `Keys` U9: *"a marker that
/// turns out to be ahead of the store — a get returning *not there* — surfaces
/// there, erases the reference under §5.1's order, and the availability
/// answer is no from then on"*; `Security and licence` §3.0 rule 4: *not
/// there* is the one class that asks the user again. **Before the fix the
/// ask read the row's marker, which still said a passphrase was held, so no
/// box was raised and the user was told only that Smoozy could not connect.**
///
/// Driven the way the running copy drives it — the route every connect
/// composes, first with nothing typed, then with what the box took.
#[test]
fn a_passphrase_gone_from_the_store_raises_the_box_and_the_right_one_connects() {
    let this_run = smoozy_desktop::the_store_standing::TheProbeThisRun::new();
    if !there_is_a_generator() {
        return;
    }
    let area = scratch("gone");
    let key = a_protected_key(&area);
    // **The row says a passphrase is held**, and the store holds nothing.
    let row = a_row_holding(&key, &a_subject());
    let record = a_record(&area);
    smoozy_desktop::record::key_rows::add(record.connection(), &row).expect("the row");
    let server = a_server_row(KeyReference {
        local_key_identifier: row.identifier.as_text(),
        fingerprint: the_fingerprint_of(&row, &area),
    });
    let the_row_now = || {
        smoozy_desktop::record::key_rows::all(record.connection())
            .expect("the rows")
            .into_iter()
            .find(|one| one.identifier == row.identifier)
            .expect("the row")
    };
    assert!(smoozy_keys::usable::a_passphrase_is_available_without_asking(&the_row_now()));

    let store = AStore::default();
    let lent = smoozy_desktop::keys::TheRecordItself::over(record.connection());

    // **The first press: *Connect*, with nothing typed.**
    let supply = smoozy_desktop::servers::a_period::the_credential_for_this_connect(
        &lent,
        &store,
        the_route(&area),
        Duration::from_secs(20),
        &this_run,
        None,
    );
    let (ended, half) = run_the_sequence(&server, WhoStartedIt::TheUser, supply);
    assert!(half.opener.credentials.is_empty());
    assert!(matches!(
        ended,
        Attempt::ItStopped(WhereItStopped::NoCredentialWasSupplied)
    ));
    let ask = half
        .credentials
        .the_ask
        .as_ref()
        .expect("**no box was raised for a passphrase the store no longer holds**");
    // **Asked as though nothing was remembered** (§3.0 rule 4): the store
    // lost it, so the line about a passphrase that stopped opening the key
    // is not said.
    assert!(!ask.the_forgotten_passphrase_line);
    assert!(matches!(
        half.credentials.why_nothing_was_supplied,
        Some(WhyNothingWasSupplied::NothingIsHeldAndTheBoxIsRaised)
    ));
    drop(half);
    // **`Keys` U9: the stale reference is erased, and the answer is no from
    // then on.**
    assert!(
        !smoozy_keys::usable::a_passphrase_is_available_without_asking(&the_row_now()),
        "the row still says a passphrase is held"
    );

    // **The second press: the box's *Connect*, with the right passphrase.**
    let typed = Origin::what_the_user_typed(THE_PASSPHRASE);
    let supply = smoozy_desktop::servers::a_period::the_credential_for_this_connect(
        &lent,
        &store,
        the_route(&area),
        Duration::from_secs(20),
        &this_run,
        Some(&typed),
    );
    let (ended, half) = run_the_sequence(&server, WhoStartedIt::TheUser, supply);
    assert_eq!(
        half.opener.credentials,
        vec![(key.clone(), true)],
        "the right passphrase did not reach the transport"
    );
    assert!(matches!(
        ended,
        Attempt::ItStopped(WhereItStopped::TheAddressDidNotAnswer)
    ));
    drop(half);
    // **`Keys` S17: the load succeeding is what puts it**, so it is held again.
    assert_eq!(store.enumerate(Duration::from_secs(1)).expect("the store").len(), 1);
    assert!(smoozy_keys::usable::a_passphrase_is_available_without_asking(&the_row_now()));

    let _ = std::fs::remove_dir_all(&area);
}

/// **And a connection Smoozy makes on its own, meeting the same gone
/// passphrase, raises no box but still corrects the marker** — `Keys` U9's
/// erasure is not conditioned on who started the attempt, and `Servers` P3
/// then withholds every automatic attempt after it.
#[test]
fn a_passphrase_gone_from_the_store_meets_an_automatic_attempt_with_no_box() {
    let this_run = smoozy_desktop::the_store_standing::TheProbeThisRun::new();
    if !there_is_a_generator() {
        return;
    }
    let area = scratch("gone-auto");
    let key = a_protected_key(&area);
    let row = a_row_holding(&key, &a_subject());
    let record = a_record(&area);
    smoozy_desktop::record::key_rows::add(record.connection(), &row).expect("the row");
    let server = a_server_row(KeyReference {
        local_key_identifier: row.identifier.as_text(),
        fingerprint: the_fingerprint_of(&row, &area),
    });

    let store = AStore::default();
    let lent = smoozy_desktop::keys::TheRecordItself::over(record.connection());
    let (_ended, half) = run_the_sequence(
        &server,
        WhoStartedIt::SmoozyOnItsOwn,
        TheCredentialForAConnection::over(&lent, &store, the_route(&area), Duration::from_secs(20), &this_run),
    );
    assert!(half.opener.credentials.is_empty());
    assert!(
        half.credentials.the_ask.is_none(),
        "**an automatic attempt put a passphrase prompt on screen**"
    );
    drop(half);
    let held = smoozy_desktop::record::key_rows::all(record.connection())
        .expect("the rows")
        .into_iter()
        .find(|one| one.identifier == row.identifier)
        .expect("the row");
    assert!(!smoozy_keys::usable::a_passphrase_is_available_without_asking(&held));

    let _ = std::fs::remove_dir_all(&area);
}

/// **`Servers` N8b, on a real protected key.**
///
/// Both halves are read rather than composed: the standing is the record's own
/// value and the protection is `Keys` U8's reading of this key's container.
/// **With the standing at *never answered* and the key protected, the repair
/// stops for good**; with the standing at anything else it does not.
#[test]
fn the_permanent_refusal_needs_the_standing_and_a_protected_key() {
    let this_run = smoozy_desktop::the_store_standing::TheProbeThisRun::new();
    if !there_is_a_generator() {
        return;
    }
    let area = scratch("n8b");
    let key = a_protected_key(&area);
    let mut row = a_row_holding(&key, &a_subject());
    row.passphrase = None;
    let record = a_record(&area);
    smoozy_desktop::record::key_rows::add(record.connection(), &row).expect("the row");
    let reference = KeyReference {
        local_key_identifier: row.identifier.as_text(),
        fingerprint: the_fingerprint_of(&row, &area),
    };

    let store = AStore::default();
    let lent = smoozy_desktop::keys::TheRecordItself::over(record.connection());
    for (standing, stops) in [("has answered", false), ("never answered", true)] {
        record
            .connection()
            .execute("DELETE FROM store_standing", [])
            .expect("the row goes");
        record
            .connection()
            .execute(
                "INSERT INTO store_standing (only_row, standing) VALUES (1, ?1)",
                [standing],
            )
            .expect("the standing");
        let mut supply = TheCredentialForAConnection::over(
            &lent,
            &store,
            the_route(&area),
            Duration::from_secs(20),
            &this_run,
        );
        let refusal = supply.the_refusal_is_permanent(&reference);
        assert!(
            refusal.the_rows_key_is_protected,
            "`Keys` U8 read a real protected key as unprotected"
        );
        assert_eq!(
            refusal.stops_the_repair(),
            stops,
            "the standing written was {standing}"
        );
    }

    let _ = std::fs::remove_dir_all(&area);
}

// ------------------------------------------- where the running copy puts it

/// **The box's *Connect*, driven the way the running copy drives it** — the
/// channel opened in Smoozy's own temporary area as a copy pointed at
/// `folder` composes it, rather than in a short scratch folder.
///
/// *(Added 2026-09-18, step 10b's audit round's loose ends, L1.)* The route
/// passed in every test above and failed in a running copy with the right
/// passphrase: every test here opened its channel directly under the system's
/// temporary folder, the copy opened it under its own application-data root,
/// and there the channel's path was longer than the system takes. The key is
/// made under a short folder, because making it goes down the same channel.
///
/// Answers what reached the transport, and the sentence the user is told
/// where no passphrase could be handed over.
fn the_boxs_connect_with_its_channel_under(
    folder: &Path,
    name: &str,
) -> (Vec<(PathBuf, bool)>, Option<&'static str>) {
    let this_run = smoozy_desktop::the_store_standing::TheProbeThisRun::new();
    let made_in = scratch(&format!("made-for-{name}"));
    let key = a_protected_key(&made_in);
    let mut row = a_row_holding(&key, &a_subject());
    row.passphrase = None;
    let record = a_record(&made_in);
    smoozy_desktop::record::key_rows::add(record.connection(), &row).expect("the row");
    let server = a_server_row(KeyReference {
        local_key_identifier: row.identifier.as_text(),
        fingerprint: the_fingerprint_of(&row, &made_in),
    });

    let area = smoozy_desktop::platform::locations::throwaway::the_temporary_area_under(folder);
    std::fs::create_dir_all(&area).expect("the copy's temporary area");

    let store = AStore::default();
    let lent = smoozy_desktop::keys::TheRecordItself::over(record.connection());
    let typed = Origin::what_the_user_typed(THE_PASSPHRASE);
    let supply = smoozy_desktop::servers::a_period::the_credential_for_this_connect(
        &lent,
        &store,
        the_route(&area),
        Duration::from_secs(20),
        &this_run,
        Some(&typed),
    );
    let (_ended, half) = run_the_sequence(&server, WhoStartedIt::TheUser, supply);
    let said = smoozy_desktop::servers::a_period::the_reason_no_passphrase_was_handed_over(
        half.credentials.why_nothing_was_supplied.as_ref(),
    );
    let handed = half.opener.credentials.clone();
    drop(half);
    let _ = std::fs::remove_dir_all(&made_in);
    (handed, said)
}

/// A folder whose copy's temporary area is exactly `length` bytes long.
///
/// **Under `/tmp` rather than the system's temporary folder**, which can
/// itself be deep enough to leave no room to choose a length — on this desk it
/// is inside the agent's own scratch area.
fn a_copy_whose_temporary_area_is(length: usize, name: &str) -> PathBuf {
    let base = PathBuf::from(format!("/tmp/sz-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base).expect("a scratch directory");
    let bare = smoozy_desktop::platform::locations::throwaway::the_temporary_area_under(&base)
        .as_os_str()
        .len();
    assert!(bare + 2 <= length, "the system's temporary folder is already too deep for this test");
    // One folder of padding, whose own separator is one of the bytes.
    let folder = base.join("x".repeat(length - bare - 1));
    assert_eq!(
        smoozy_desktop::platform::locations::throwaway::the_temporary_area_under(&folder)
            .as_os_str()
            .len(),
        length
    );
    folder
}

/// **L1: a copy whose data folder is deep enough that no channel fits is told
/// why, and is never told *could not connect*.** Before the fix nothing read
/// the reason the supply held, and the user was told the claimless sentence.
#[test]
fn where_no_channel_fits_the_user_is_told_why_and_not_could_not_connect() {
    if !there_is_a_generator() {
        return;
    }
    let longest = smoozy_desktop::keys::askpass::the_longest_channel_path();
    let folder = a_copy_whose_temporary_area_is(longest, "too-long");

    let (handed, said) = the_boxs_connect_with_its_channel_under(&folder, "too-long");

    assert!(handed.is_empty(), "no passphrase could be handed over, so nothing was");
    assert_eq!(
        said,
        Some(smoozy_desktop::keys::askpass::THE_PATH_IS_TOO_LONG),
        "**the user was told nothing true about why**"
    );
    let _ = std::fs::remove_dir_all(folder.parent().expect("the scratch folder"));
}

/// **L1: a copy whose data folder the old channel name did not fit under
/// connects.** The name was `ap<process id>-<number>`; eight bytes of room
/// after the area hold `/a` and a number, and never that.
#[test]
fn a_data_folder_the_old_channel_name_did_not_fit_under_connects() {
    if !there_is_a_generator() {
        return;
    }
    let longest = smoozy_desktop::keys::askpass::the_longest_channel_path();
    let folder = a_copy_whose_temporary_area_is(longest - 8, "just-fits");

    let (handed, said) = the_boxs_connect_with_its_channel_under(&folder, "just-fits");

    assert_eq!(said, None);
    assert_eq!(
        handed.len(),
        1,
        "**the box's *Connect* connected nothing**: the channel did not fit"
    );
    assert!(handed[0].1, "the passphrase went with the key");
    let _ = std::fs::remove_dir_all(folder.parent().expect("the scratch folder"));
}
