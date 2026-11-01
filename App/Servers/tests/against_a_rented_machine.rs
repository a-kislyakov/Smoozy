//! **Step 4's own remaining debt, paid: the code driven against a real rented
//! Linux machine.**
//!
//! **Why this exists, and it is the one thing the loopback server cannot do.**
//! `against_a_real_machine.rs` drives the same sequence against a server this
//! process runs on the loopback address, and `Server protocol and transfers`'
//! own integration test drives §7 against OpenSSH's own `sftp-server` the same
//! way. Between them they prove the transport, the protocol, the ordering and
//! the whole publication contract. **What they cannot prove is a Linux box's
//! own `uname`, `lspci`, `nvidia-smi`, cgroup files and mounts**, which is what
//! §8's readings, `Rented-server runtime` K1's table and `Job files and
//! outputs` L2's choice were written against. Those answers come from a
//! machine or from nowhere.
//!
//! **The machine is named on the environment and nothing here rents one.** A
//! rental costs money and runs until somebody destroys it by hand, so no test
//! may start one: whoever rents the machine names it here and destroys it
//! afterwards. **Where no machine is named these tests say so and stop** — the
//! same shape the other two integration tests already use, never a quieter
//! assertion.
//!
//! ```text
//! SMOOZY_RENTED_HOST  the address
//! SMOOZY_RENTED_PORT  the port
//! SMOOZY_RENTED_USER  the login
//! SMOOZY_RENTED_KEY   the private key file, on this computer
//! SMOOZY_RENTED_ROOT  a writable location on the machine (default `/root`)
//! SMOOZY_RENTED_ROOT_2  a second writable location, where there is one
//! ```
//!
//! **No value of any of these is a secret and none is written down anywhere.**
//! The key is named by its path, which is what `Security and licence` P1
//! already requires of every credential Smoozy holds.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` F2, F16, S18.
//! `Job files and outputs` L2, L4a, L6, L10, L11, L12.
//! `Desktop app and delivery` F15a.
//!
//! **`Server protocol and transfers` S15 is deliberately not listed above and is
//! written out with its document at each site**, because this plan defines an
//! S15 of its own and a bare one could mean either.
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this
//! crate's own plan. Every other document's marker is written with that
//! document — at the site, or in the file's own header list of markers that
//! belong elsewhere. **Where a marker is a clause of this crate's own plan
//! *and* of the document meant, the header list is not enough and the document
//! is written out at the site**, because a bare one there does not fail to
//! resolve: it resolves to a real and wrong clause, which nobody reading it
//! can tell from a right one.
//!
//! **The plan here is `Preparation/Architecture/Servers.md`, and the words above
//! are the words `src/lib.rs` carries.** They are repeated because a file under
//! `tests/` is its own crate and never sees that one, and two of the three
//! sites the step 4 gate's fourth run caught were in a file under `tests/`.
//! Keep the wording identical wherever it appears.

use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use smoozy_job_files::acting::TheMachinesDirectories;
use smoozy_job_files::layout::{TheLayout, WhereThePackageAreaSits, EVERY_AREA};
use smoozy_job_files::placement::{confirm_the_layout, WhatTheConfirmationFound};
use smoozy_job_files::root_choice::{choose, Candidate};
use smoozy_server_protocol::free_space;
use smoozy_server_protocol::subsystem::client::Subsystem;
use smoozy_server_protocol::subsystem::{extension, Announcement};
use smoozy_server_protocol::transfers::integrity::Digest;
use smoozy_server_protocol::transfers::moving::{
    carry_out, DrawsFromTheSharedBudget, KeepsTheRecord, LocalFailure, NobodyIsWatching,
    TheUsersOwnComputer, Transfer,
};
use smoozy_server_protocol::transfers::publication::{Intent, WhatTheFinalObjectReadsAs};
use smoozy_server_protocol::transfers::{Direction, Ending, StableFileIdentity, TransferRecord};
use smoozy_server_protocol::transport::ssh::{open, Connection};
use smoozy_server_protocol::transport::{
    Credential, DecideHostTrust, Endpoint, HostTrust,
};
use smoozy_servers::acting::{
    SupplyTheCredential, TheActingHalf, TheIdentityRead, TheInstallationIdentityRead,
    TheServerBeingConnectedTo,
};
use smoozy_servers::condition::ObservedValue;
use smoozy_servers::first_connection::ChangedIdentityWarning;
use smoozy_servers::hardware::{BusScan, Cards, DriverAnswer, MachineKind, MachineMemory};
use smoozy_servers::identity::{Host, KeyReference, Port, Username};
use smoozy_servers::key_moments::{TheTwoAnswers, WhoStartedIt};
use smoozy_servers::readings::{self, WhichCgroupPair};
use smoozy_servers::repair::{NextAttemptDue, PermanentRefusal, WhatTheRecordSays};
use smoozy_servers::sequence::{self, AnAttempt, Attempt, ConnectionRole};
use smoozy_servers::store::{RecordedMachineRoot, RememberedHostKey, ServerRow};
use smoozy_servers::wiring::{TheDesktopsClock, TheTransport, K3S_FIGURES};
use smoozy_state::identities::{Identity, IdentityKind, InstallationCheck};

// ------------------------------------------------------- the machine, or not

/// What the environment names.
struct TheRental {
    host: String,
    port: u16,
    username: String,
    key: PathBuf,
    root: String,
    second_root: Option<String>,
}

fn the_named_machine() -> Option<TheRental> {
    let host = std::env::var("SMOOZY_RENTED_HOST").ok()?;
    let port = std::env::var("SMOOZY_RENTED_PORT").ok()?.parse().ok()?;
    let username = std::env::var("SMOOZY_RENTED_USER").ok()?;
    let key = PathBuf::from(std::env::var("SMOOZY_RENTED_KEY").ok()?);
    if host.is_empty() || !key.exists() {
        return None;
    }
    Some(TheRental {
        host,
        port,
        username,
        key,
        root: std::env::var("SMOOZY_RENTED_ROOT").unwrap_or_else(|_| "/root".to_string()),
        second_root: std::env::var("SMOOZY_RENTED_ROOT_2").ok().filter(|it| !it.is_empty()),
    })
}

/// Says the miss out loud rather than passing quietly.
macro_rules! the_rental_or_say_so {
    () => {
        match the_named_machine() {
            Some(rental) => rental,
            None => {
                println!(
                    "SKIPPED: no rented machine is named on the environment. \
                     Everything below is unproved against a real Linux box here."
                );
                return;
            }
        }
    };
}

// ------------------------------------------------------------- the other ends

/// `Keys`' supply, answering for a key with no passphrase.
struct AKeyThatWorks {
    file: PathBuf,
}

impl SupplyTheCredential for AKeyThatWorks {
    fn the_two_answers(&mut self, _key: &KeyReference) -> TheTwoAnswers {
        TheTwoAnswers {
            the_key_is_usable: true,
            a_passphrase_is_available_without_asking: true,
        }
    }

    fn for_one_attempt(
        &mut self,
        _key: &KeyReference,
        _who: WhoStartedIt,
        _naming: TheServerBeingConnectedTo<'_>,
    ) -> Option<Credential> {
        Some(Credential {
            private_key_file: self.file.clone(),
            passphrase: None,
        })
    }

    fn the_refusal_is_permanent(&mut self, _key: &KeyReference) -> PermanentRefusal {
        PermanentRefusal {
            the_store_standing_reads_never_answered: false,
            the_rows_key_is_protected: false,
        }
    }
}

/// Step 5's read, which belongs to modules built at step 6.
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
    observed_values: Vec<ObservedValue>,
    conditions_raised: Vec<WhatTheRecordSays>,
    changed_identity_warnings: usize,
    host_keys_recorded: Vec<Vec<u8>>,
}

impl smoozy_servers::acting::WhatServersPublishes for Caught {
    fn the_observed_value(&mut self, _server: &str, value: ObservedValue) {
        self.observed_values.push(value);
    }

    fn the_next_attempt_due(&mut self, _server: &str, _reading: NextAttemptDue) {}

    fn raise_the_connection_condition(&mut self, _server: &str, says: WhatTheRecordSays) {
        self.conditions_raised.push(says);
    }

    fn withdraw_the_connection_condition(&mut self, _server: &str) {}

    fn the_changed_identity_warning(&mut self, _server: &str) {
        self.changed_identity_warnings += 1;
    }

    fn record_the_scanned_host_key(&mut self, _server: &str, key: RememberedHostKey) {
        self.host_keys_recorded
            .push(key.as_the_local_record_writes_it().to_vec());
    }
}

type Half = TheActingHalf<TheTransport, AKeyThatWorks, TheIdentityIsAbsent, Caught, TheDesktopsClock>;

fn a_half(rental: &TheRental) -> Half {
    TheActingHalf {
        opener: TheTransport,
        credentials: AKeyThatWorks {
            file: rental.key.clone(),
        },
        installation: TheIdentityIsAbsent,
        publishes: Caught::default(),
        wait: TheDesktopsClock,
    }
}

fn a_row(rental: &TheRental) -> ServerRow {
    ServerRow::saved(
        Identity::issue(IdentityKind::LocalServer)
            .expect("randomness")
            .as_text(),
        Host::what_the_user_typed(&rental.host).expect("a host"),
        Port::what_the_user_typed(rental.port).expect("a port"),
        Username::what_the_user_typed(&rental.username).expect("a username"),
        KeyReference {
            local_key_identifier: "the rental's key".into(),
            fingerprint: "SHA256:not-compared-here".into(),
        },
    )
}

fn an_attempt(row: &ServerRow) -> AnAttempt<'_> {
    AnAttempt {
        row,
        who: WhoStartedIt::TheUser,
        the_first_connection_to_this_server: true,
        role: ConnectionRole::Primary,
        // An attempt that belongs to no run carries N2's own connect deadline
        // and nothing shorter.
        deadline: smoozy_servers::repair::CONNECT_DEADLINE,
    }
}

/// The callback Q1 step 3 calls the instant the transport reports the
/// connection authenticated and usable. **K6's *connected* is the period's to
/// compose**, and these tests drive the sequence rather than a period.
fn nothing_to_publish(_publishes: &mut Caught) {}

struct Approves;
impl DecideHostTrust for Approves {
    fn decide(&mut self, _offered: &[u8]) -> HostTrust {
        HostTrust::Proceed
    }
}

/// A plain connection, for the readings that are not the sequence's own.
async fn a_connection(rental: &TheRental) -> Connection {
    open(
        &Endpoint {
            host: rental.host.clone(),
            port: rental.port,
            username: rental.username.clone(),
        },
        Credential {
            private_key_file: rental.key.clone(),
            passphrase: None,
        },
        K3S_FIGURES,
        Box::new(Approves),
    )
    .await
    .expect("the rented machine takes the connection")
}

async fn a_subsystem(
    connection: &Connection,
) -> Subsystem<russh::ChannelStream<russh::client::Msg>> {
    let channel = connection
        .open_a_subsystem_channel()
        .await
        .expect("the rented machine serves the file-transfer subsystem");
    Subsystem::negotiate(channel)
        .await
        .expect("the subsystem negotiates")
}

// ------------------------------------------------------------------ the desk

struct Scratch(PathBuf);

impl Scratch {
    fn new(name: &str) -> Scratch {
        let path = std::env::temp_dir().join(format!(
            "smoozy-rented-{name}-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("a temporary directory can be made");
        Scratch(path)
    }

    fn at(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

// -------------------------------------------------- 1. Q1's whole sequence

/// **`Servers` Q1's first-connection sequence, whole, against a rented Linux
/// machine.**
///
/// Host trust, the key check, authentication, the not-Linux check, the
/// installation identity and the hardware reading — in order, over one real
/// connection to a real box.
#[tokio::test]
async fn q1s_first_connection_sequence_runs_whole_against_a_rented_machine() {
    let rental = the_rental_or_say_so!();
    let row = a_row(&rental);
    let mut half = a_half(&rental);

    let ended = sequence::run(
        an_attempt(&row),
        &mut half,
        &mut ChangedIdentityWarning::none_yet(),
        &mut nothing_to_publish,
    )
    .await;
    let Attempt::ItIsLive(gave) = ended else {
        panic!("the rented machine answered every step the sequence asked it")
    };

    // **Q6's first path, on the machine's own key, byte for byte.** Nothing
    // remembered, so the scanned key is recorded and nothing is warned about.
    assert_eq!(half.publishes.host_keys_recorded.len(), 1);
    assert!(!half.publishes.host_keys_recorded[0].is_empty());
    assert_eq!(half.publishes.changed_identity_warnings, 0);
    assert!(half.publishes.conditions_raised.is_empty());
    // **Nothing is raised and nothing is observed by the sequence itself.**
    // The observed value belongs to the period, which is `driving.rs`'s and is
    // proved against the loopback machine.
    assert!(half.publishes.observed_values.is_empty());

    // **Step 5's branch**, which belongs to step 6's modules and answers absent.
    assert_eq!(gave.installation, Some(InstallationCheck::Absent));

    // **Step 6, on the branch that takes a fresh reading (H18).**
    let reading = gave.hardware.expect("a fresh reading off the rented machine");
    println!("--- the rented machine's own hardware reading ---");
    println!("cards: {:?}", reading.cards);
    println!("machine memory: {:?}", reading.machine_memory);
    println!("free space: {:?}", reading.free_space);
    println!(
        "the machine's own moment: {:?}",
        reading.the_machines_own_moment.taken_at()
    );
    println!("read under: {:?}", reading.read_under);

    // **H2: the moment is the machine's own**, read from the machine and not
    // borrowed from this computer. Within a day of the desk's clock is all that
    // is claimed — **`State, identity and recovery` D5 forbids subtracting the
    // two**, so nothing tighter is
    // asserted here.
    let machine_moment = reading.the_machines_own_moment.taken_at();
    let a_day = Duration::from_secs(24 * 60 * 60);
    assert!(
        machine_moment > SystemTime::now() - a_day && machine_moment < SystemTime::now() + a_day,
        "the machine named its own clock"
    );
    // **H17: the explicit absence of an installation identity.**
    assert_eq!(reading.read_under, None);
    // **`State, identity and recovery` D5: the two moments are never
    // subtracted from each other.**
    assert_eq!(reading.how_long_between_the_two_moments(), None);
    // **H10: no root is recorded on a machine nothing has been placed on, so
    // the figure is absent and nothing is substituted for it.**
    assert!(reading.free_space.is_none());

    let _ = half.opener;
    drop(gave.connection);
}

/// **The not-Linux check reading a real `uname`, and Q13's three outcomes as
/// far as a Linux box can show them.**
#[tokio::test]
async fn q13s_check_reads_a_real_uname_and_the_machine_names_linux() {
    let rental = the_rental_or_say_so!();
    let mut connection = a_connection(&rental).await;

    let outcome = readings::read_the_operating_system(&mut connection).await;
    println!("--- the not-Linux check, against the rental's own uname ---");
    println!("outcome: {outcome:?}");
    assert_eq!(
        outcome,
        smoozy_servers::first_connection::NotLinuxOutcome::ItNamedLinux
    );
    // **Only outcome 2 says what the machine is**, and this is outcome 1: the
    // sequence carries on and the period is not ended.
    assert!(!outcome.claims_what_the_machine_is());
    assert!(outcome.ends_the_period().is_none());

    connection.tear_down().await;
}

// ------------------------------------- 2. the readings, against real programs

/// **§8's readings against the machine's own programs and files.**
///
/// H11's bus scan and its driver column, the machine kind performing
/// `Rented-server runtime` K1's own table, and the machine memory off real
/// cgroup files. **Every state is printed rather than only asserted**, because
/// which state a real provider image lands in is the thing this rental exists
/// to find out.
///
/// **What the card list itself has to say is asserted in the test below**,
/// which is the one that needs the rental to have a card. Nothing asserted
/// here needs one, so this test holds on a cardless rental.
#[tokio::test]
async fn the_hardware_readings_are_taken_off_the_machines_own_programs() {
    let rental = the_rental_or_say_so!();
    let mut connection = a_connection(&rental).await;

    let kind = readings::read_the_machine_kind(&mut connection).await;
    let scan = readings::scan_the_bus(&mut connection).await;
    let driver = readings::ask_the_driver(&mut connection).await;
    let newer = readings::read_a_cgroup_pair(&mut connection, WhichCgroupPair::TheNewer).await;
    let older = readings::read_a_cgroup_pair(&mut connection, WhichCgroupPair::TheOlder).await;
    let own = readings::read_the_machines_own_memory(&mut connection).await;
    let placed = readings::where_k1_places_it(&mut connection, kind).await;
    let clock = readings::read_the_machines_own_clock(&mut connection).await;

    println!("--- §8's readings, off the rented machine ---");
    println!("`Rented-server runtime` K1's machine kind: {kind:?}");
    println!("H11 bus scan: {scan:?}");
    println!("H11 driver column: {driver:?}");
    println!("H9a newer cgroup pair: {newer:?}");
    println!("H9a older cgroup pair: {older:?}");
    println!("H9 /proc figure: {own:?}");
    println!("H2 machine clock: {clock:?}");

    let cards = smoozy_servers::hardware::read_the_cards(scan, driver.clone(), kind);
    println!("H11's joined answer: {cards:?}");
    println!("`Rented-server runtime` K1's whole table, for H9a: {placed:?}");
    let memory = smoozy_servers::hardware::read_the_machines_memory(placed, newer, older, own);
    println!("H9's joined answer: {memory:?}");

    // **The machine named its own clock**, which is what H2's second half asks
    // of it and what H4 makes a whole reading impossible without.
    assert!(clock.is_some());
    // **H9: the machine named its own memory.**
    // **The figure is *a figure* or *nothing came back*** (H9a's third row),
    // and a real machine names its own memory.
    assert!(own.is_some_and(|figure| figure > 0));
    assert_ne!(memory, MachineMemory::CouldNotBeEstablished);

    // **The three states of the scan are never collapsed**, so whichever this
    // machine is in is recorded as itself.
    match scan {
        BusScan::Populated => println!("the scan is populated"),
        BusScan::Empty => println!("the scan answered empty"),
        BusScan::NoRoute => println!("the scan did not run on this image"),
    }
    match kind {
        MachineKind::AContainer => {
            println!("`Rented-server runtime` K1's table: a container")
        }
        MachineKind::NotAContainer => {
            println!("`Rented-server runtime` K1's table: not a container")
        }
    }
    match &cards {
        Cards::TheCards(_) => {}
        other => println!("H11's joined answer is not a card list here: {other:?}"),
    }

    connection.tear_down().await;
}

// ---------------------------- 2a. H11's card list, off a card-bearing rental

/// **H11's card list, read off a real driver.**
///
/// Split out of the readings test above on 2026-09-04. These are the only
/// assertions in this file that need the rental to **have** a card: on a
/// cardless rental there is no card list to read, and before the split a
/// rental with no card turned the whole readings test red — a red that meant
/// *this rental has no card* and read as *the code is broken*.
///
/// **The miss is said out loud and the machine's own answer is printed with
/// it**, which is the shape the file's own rental gate uses and the shape both
/// of this workspace's other integration tests use. **A test that skips when
/// its subject is absent is honest; a test that passes when its subject is
/// absent is not** — so nothing below is softened, and every assertion that
/// runs asserts what it always asserted.
#[tokio::test]
async fn h11s_card_list_is_read_off_a_real_driver() {
    let rental = the_rental_or_say_so!();
    let mut connection = a_connection(&rental).await;

    let driver = readings::ask_the_driver(&mut connection).await;
    println!("--- H11's driver column, off the rented machine ---");
    println!("H11 driver column: {driver:?}");

    // **The one thing a card-bearing rental must show: the driver answered.**
    // A card has to be present for H11 and the driver query to answer at all.
    // **Where it did not answer, this rental has no card**, and everything
    // below has no subject to assert against.
    let DriverAnswer::Answered(rows) = &driver else {
        println!(
            "SKIPPED: the rented machine's driver did not answer ({driver:?}), \
             so this rental carries no card. H11's card list is unproved here."
        );
        connection.tear_down().await;
        return;
    };
    // **H5's count rule: the count is the rows.** Nothing reads a count any
    // other way.
    assert!(!rows.is_empty());
    for card in rows {
        println!(
            "card: {} at {} — {} — {:?} bytes",
            card.unique_identifier,
            card.bus_location,
            card.make.the_drivers_name_for_the_card(),
            card.total_memory_bytes
        );
        assert!(!card.unique_identifier.is_empty());
        assert!(!card.bus_location.is_empty());
    }

    connection.tear_down().await;
}

// ------------------------------------------- 3. H10's per-path free space

/// **H10's free space, per path, against a real mount.**
///
/// Two paths are read and the reading names the path it was taken against —
/// which is the whole of what makes it a per-path figure rather than a
/// whole-machine one. A path that does not exist yet is read at its deepest
/// existing ancestor, and the reading says so.
#[tokio::test]
async fn h10s_free_space_is_per_path_against_a_real_mount() {
    let rental = the_rental_or_say_so!();
    let connection = a_connection(&rental).await;
    let mut subsystem = a_subsystem(&connection).await;

    assert!(
        subsystem.announcement().announces(extension::STATVFS),
        "F2 route 1 is refused before it is attempted where it is not announced"
    );

    let at_the_root = free_space::read_it(&mut subsystem, &rental.root)
        .await
        .expect("the rental answers a per-path reading");
    println!("--- H10, against a real mount ---");
    println!(
        "{} — {} bytes available, taken against {}",
        rental.root, at_the_root.available_bytes, at_the_root.the_path_it_was_taken_against
    );
    assert_eq!(at_the_root.the_path_it_was_taken_against, rental.root);
    assert!(at_the_root.available_bytes > 0);

    // **A path that is not there yet is read at its deepest existing
    // ancestor**, and the reading records which path that was rather than
    // claiming the figure belongs to the path asked for.
    let not_there = format!("{}/smoozy-not-here-yet/deeper/still", rental.root);
    let climbed = free_space::read_it_at_the_deepest_ancestor_that_exists(&mut subsystem, &not_there)
        .await
        .expect("the walk finds an ancestor that exists");
    println!(
        "{} — {} bytes available, taken against {}",
        not_there, climbed.available_bytes, climbed.the_path_it_was_taken_against
    );
    assert_eq!(climbed.the_path_it_was_taken_against, rental.root);

    // **A second mount, where the rental has one.** Two different mounts
    // answering two different figures is what F16 and L2 turn on.
    if let Some(second) = &rental.second_root {
        let there = free_space::read_it(&mut subsystem, second)
            .await
            .expect("the second location answers");
        println!(
            "{} — {} bytes available, taken against {}",
            second, there.available_bytes, there.the_path_it_was_taken_against
        );
        assert_eq!(&there.the_path_it_was_taken_against, second);
    }
}

// ------------------------------- 4. the milestone, over a real network

/// The desk's stand-in for `Desktop app and delivery`'s platform file adapter.
///
/// **It is not the adapter and does not claim to be.** `Server protocol and
/// transfers` S18's four requirements are the adapter's; what this proves is
/// that the contract drives correctly against a local end that keeps them.
struct APlainDisk;

/// **`Desktop app and delivery` F15a's route, per system**, copied here because
/// this crate does not depend on that one and a test crate cannot reach its
/// adapter. **The same reading `Server protocol and transfers`' own integration
/// test takes**, written from the same call the real adapter uses.
///
/// macOS and Linux: the `st_dev`/`st_ino` pair.
///
/// **Why it is per system rather than Unix-only.** A stand-in that answers on
/// one system and reports the identity unavailable on the others is weaker than
/// the adapter it stands in for, and the difference is not quiet: a recorded
/// value with no readable counterpart means `Server protocol and transfers` S15
/// cannot compose its recovery proof, so recovery correctly starts over and the
/// fresh attempt correctly refuses a final name already taken. That failed a
/// transfer test on Windows on 2026-09-03 and was repaired there.
#[cfg(unix)]
fn the_platforms_own_file_identity(path: &Path) -> Option<String> {
    use std::os::unix::fs::MetadataExt;
    let found = std::fs::metadata(path).ok()?;
    Some(format!("{}:{}", found.dev(), found.ino()))
}

/// **Windows: `GetFileInformationByHandleEx` with `FileIdInfo`.** Microsoft's
/// own words for that structure: *"The file identifier and the volume serial
/// number uniquely identify a file on a single computer."* Documented,
/// `FILE_ID_INFO` (winbase.h), read 2026-09-03 at
/// `https://learn.microsoft.com/en-us/windows/win32/api/winbase/ns-winbase-file_id_info`.
///
/// **Where the call refuses, the identity is reported unavailable rather than
/// substituted for** — `Desktop app and delivery` F15a's own branch, and the
/// same thing the real adapter does.
#[cfg(windows)]
fn the_platforms_own_file_identity(path: &Path) -> Option<String> {
    use std::os::windows::fs::OpenOptionsExt;
    use std::os::windows::io::AsRawHandle;
    use windows_sys::Win32::Storage::FileSystem::{
        FileIdInfo, GetFileInformationByHandleEx, FILE_FLAG_BACKUP_SEMANTICS, FILE_ID_INFO,
    };

    let file = std::fs::OpenOptions::new()
        .read(true)
        .custom_flags(FILE_FLAG_BACKUP_SEMANTICS)
        .open(path)
        .ok()?;

    let mut info: FILE_ID_INFO = unsafe { std::mem::zeroed() };
    let answered = unsafe {
        GetFileInformationByHandleEx(
            file.as_raw_handle() as _,
            FileIdInfo,
            &mut info as *mut _ as *mut core::ffi::c_void,
            std::mem::size_of::<FILE_ID_INFO>() as u32,
        )
    };
    if answered == 0 {
        return None;
    }
    Some(format!(
        "{}:{}",
        info.VolumeSerialNumber,
        u128::from_le_bytes(info.FileId.Identifier)
    ))
}

/// **The stand-in answers the identity on the system this is running on**, and
/// this test needs no rented machine, so it runs everywhere the workspace does.
///
/// **It is the test the Unix-only stand-in would have failed.** `Server protocol
/// and transfers` S15 composes its recovery proof out of a recorded identity and
/// the counterpart read back; a stand-in that reports the identity unavailable
/// on a system where the real adapter answers makes that proof uncomposable, so
/// recovery starts over and the fresh attempt meets a final name already taken.
/// That is what failed a transfer test on Windows on 2026-09-03.
#[test]
fn the_stand_in_reads_the_identity_on_this_system_the_way_the_adapter_does() {
    let scratch = Scratch::new("identity");
    let path = scratch.at("a-file");
    std::fs::write(&path, b"six by").expect("a file can be written");

    let mut disk = APlainDisk;
    let identity = match disk.stable_identity_of(path.to_str().expect("a path")) {
        StableFileIdentity::The(identity) => identity,
        StableFileIdentity::TheSupplierCouldNotAnswer => panic!(
            "the stand-in reported the identity unavailable for a file it just \
             wrote; `Desktop app and delivery` F15a answers on all three systems"
        ),
    };

    // The same file twice is the same identity, which is what `Server protocol
    // and transfers` S15's comparison rests on.
    assert_eq!(
        disk.stable_identity_of(path.to_str().expect("a path")),
        StableFileIdentity::The(identity)
    );

    // **A path that is not there is the unavailable branch and not a guess** —
    // F15a's own rule, and the real adapter's.
    assert_eq!(
        disk.stable_identity_of(scratch.at("not-there").to_str().expect("a path")),
        StableFileIdentity::TheSupplierCouldNotAnswer
    );
}

impl TheUsersOwnComputer for APlainDisk {
    fn staged_size(&mut self, path: &str) -> Option<u64> {
        std::fs::metadata(path).ok().map(|read| read.len())
    }

    fn read_at(&mut self, path: &str, offset: u64, length: usize) -> Result<Vec<u8>, LocalFailure> {
        use std::io::{Read, Seek, SeekFrom};
        let mut file = std::fs::File::open(path).map_err(|why| LocalFailure(why.to_string()))?;
        file.seek(SeekFrom::Start(offset))
            .map_err(|why| LocalFailure(why.to_string()))?;
        let mut buffer = vec![0u8; length];
        let mut filled = 0;
        while filled < length {
            match file.read(&mut buffer[filled..]) {
                Ok(0) => break,
                Ok(read) => filled += read,
                Err(why) => return Err(LocalFailure(why.to_string())),
            }
        }
        buffer.truncate(filled);
        Ok(buffer)
    }

    fn create_staging_exclusively(&mut self, path: &str) -> Result<(), LocalFailure> {
        if let Some(parent) = Path::new(path).parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .map(|_| ())
            .map_err(|why| LocalFailure(why.to_string()))
    }

    fn write_at(&mut self, path: &str, offset: u64, bytes: &[u8]) -> Result<(), LocalFailure> {
        use std::io::{Seek, SeekFrom, Write};
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .open(path)
            .map_err(|why| LocalFailure(why.to_string()))?;
        file.seek(SeekFrom::Start(offset))
            .map_err(|why| LocalFailure(why.to_string()))?;
        file.write_all(bytes)
            .map_err(|why| LocalFailure(why.to_string()))
    }

    fn make_durable(&mut self, path: &str) -> Result<(), LocalFailure> {
        let file = std::fs::File::open(path).map_err(|why| LocalFailure(why.to_string()))?;
        file.sync_all().map_err(|why| LocalFailure(why.to_string()))
    }

    /// **`Server protocol and transfers` S15c** is that crate's own tests';
    /// this disk keeps no time, and the transfer is unchanged without one.
    fn the_sources_time(&mut self, _source_path: &str) -> Option<i64> {
        None
    }

    fn keep_the_time(&mut self, _staging: &str, _seconds: i64) -> Result<(), LocalFailure> {
        Ok(())
    }

    fn stable_identity_of(&mut self, path: &str) -> StableFileIdentity {
        match the_platforms_own_file_identity(Path::new(path)) {
            Some(identity) => StableFileIdentity::The(identity),
            None => StableFileIdentity::TheSupplierCouldNotAnswer,
        }
    }

    fn publish(
        &mut self,
        staging: &str,
        final_path: &str,
        intent: Intent,
    ) -> Result<(), LocalFailure> {
        if intent == Intent::CreateIntended && Path::new(final_path).exists() {
            return Err(LocalFailure("the final name is taken".to_string()));
        }
        std::fs::rename(staging, final_path).map_err(|why| LocalFailure(why.to_string()))
    }

    fn remove(&mut self, path: &str) -> Result<(), LocalFailure> {
        std::fs::remove_file(path).map_err(|why| LocalFailure(why.to_string()))
    }

    fn read_the_final_object(&mut self, path: &str) -> WhatTheFinalObjectReadsAs {
        match std::fs::metadata(path) {
            Ok(read) => WhatTheFinalObjectReadsAs {
                exists: true,
                byte_length: Some(read.len()),
                identity: match self.stable_identity_of(path) {
                    StableFileIdentity::The(identity) => Some(identity),
                    StableFileIdentity::TheSupplierCouldNotAnswer => None,
                },
                at_rest_digest: None,
            },
            Err(_) => WhatTheFinalObjectReadsAs::default(),
        }
    }
}

#[derive(Default)]
struct WritesItDown {
    writes: usize,
}

impl KeepsTheRecord for WritesItDown {
    fn write_it_down(&mut self, _record: &TransferRecord) -> Result<(), LocalFailure> {
        self.writes += 1;
        Ok(())
    }
}

/// A budget that hands out a fixed amount and then refuses.
struct ACapThatBites {
    left: usize,
}

impl DrawsFromTheSharedBudget for ACapThatBites {
    fn draw(&mut self, wanted: usize) -> usize {
        let allowed = wanted.min(self.left);
        self.left -= allowed;
        allowed
    }
    fn give_back(&mut self, unused: usize) {
        self.left += unused;
    }
}

fn a_file_of(bytes: usize) -> Vec<u8> {
    (0..bytes).map(|at| (at % 251) as u8).collect()
}

fn a_transfer_record(
    direction: Direction,
    from: &str,
    to: &str,
    staging_directory: &str,
    size: u64,
    digest: Digest,
) -> TransferRecord {
    let issue = |kind| Identity::issue(kind).expect("random bytes");
    let mut record = TransferRecord::arranged(
        issue(IdentityKind::Transfer),
        direction,
        issue(IdentityKind::LocalServer),
        Some(issue(IdentityKind::Installation)),
        from.to_string(),
        to.to_string(),
        staging_directory,
        size,
        issue(IdentityKind::Run),
    );
    record.expected_digest = Some(digest);
    record
}

/// **Step 4's own milestone, over a real network rather than the loopback: a
/// file goes up, comes back whole, and resumes correctly after being
/// interrupted.**
#[tokio::test]
async fn a_file_goes_up_comes_back_and_resumes_over_a_real_network() {
    let rental = the_rental_or_say_so!();
    let scratch = Scratch::new("milestone");
    let connection = a_connection(&rental).await;
    let mut subsystem = a_subsystem(&connection).await;

    // **§1.2: the rental's own server announces revision 3 and the three
    // extensions §7 leans on.** Asserted against a provider image rather than
    // against this computer's OpenSSH.
    println!("--- what the rental's own file-transfer server announced ---");
    println!("version: {}", subsystem.announcement().version);
    println!(
        "posix-rename: {}, fsync: {}, statvfs: {}",
        subsystem.announcement().announces(extension::POSIX_RENAME),
        subsystem.announcement().announces(extension::FSYNC),
        subsystem.announcement().announces(extension::STATVFS),
    );
    assert_eq!(
        subsystem.announcement().version,
        Announcement::REVISION_THIS_DOCUMENT_USES
    );
    assert!(subsystem.announcement().announces(extension::POSIX_RENAME));
    assert!(subsystem.announcement().announces(extension::FSYNC));
    assert!(subsystem.announcement().announces(extension::STATVFS));

    let there = format!("{}/smoozy-milestone", rental.root);
    subsystem
        .make_directory(there.as_bytes())
        .await
        .expect("a folder for the milestone");

    let mut local = APlainDisk;
    let mut keeper = WritesItDown::default();
    let mut watcher = NobodyIsWatching;

    // ---- a file goes up.
    let bytes = a_file_of(3 * 1024 * 1024);
    let source = scratch.at("scene.blend");
    std::fs::write(&source, &bytes).expect("the scratch takes a file");
    let destination = format!("{there}/scene.blend");
    let mut record = a_transfer_record(
        Direction::Up,
        &source.to_string_lossy(),
        &destination,
        &format!("{there}/.smoozy-staging"),
        bytes.len() as u64,
        Digest::of(&bytes),
    );
    let mut budget = smoozy_server_protocol::transfers::moving::NoCapIsInForce;
    let happened = carry_out(
        Transfer {
            subsystem: &mut subsystem,
            local: &mut local,
            budget: &mut budget,
            keeper: &mut keeper,
            watcher: &mut watcher,
            inactivity_deadline: Duration::from_secs(120),
            intent: Intent::CreateIntended,
        },
        &mut record,
    )
    .await;
    assert_eq!(happened.ending, Ending::Published);
    // **The final name holds it whole, checked from the machine.**
    let landed = subsystem
        .stat(destination.as_bytes())
        .await
        .expect("the final name is there");
    assert_eq!(landed.size, Some(bytes.len() as u64));
    // **The staging name is gone**, the rename being the cleanup.
    assert!(subsystem
        .stat(record.staging_path.as_bytes().as_slice())
        .await
        .is_err());
    assert!(keeper.writes >= 2);

    // ---- and comes back whole.
    let here = scratch.at("back/scene.blend");
    std::fs::create_dir_all(here.parent().expect("a folder")).expect("made");
    let mut back = a_transfer_record(
        Direction::Down,
        &destination,
        &here.to_string_lossy(),
        &here
            .parent()
            .expect("a folder")
            .join(".smoozy-staging")
            .to_string_lossy(),
        bytes.len() as u64,
        Digest::of(&bytes),
    );
    let mut budget = smoozy_server_protocol::transfers::moving::NoCapIsInForce;
    let came_back = carry_out(
        Transfer {
            subsystem: &mut subsystem,
            local: &mut local,
            budget: &mut budget,
            keeper: &mut keeper,
            watcher: &mut watcher,
            inactivity_deadline: Duration::from_secs(120),
            intent: Intent::CreateIntended,
        },
        &mut back,
    )
    .await;
    assert_eq!(came_back.ending, Ending::Published);
    let whole = std::fs::read(&here).expect("the final name holds it");
    assert_eq!(Digest::of(&whole), Digest::of(&bytes));

    // ---- and resumes correctly after being interrupted.
    let big = a_file_of(12 * 1024 * 1024);
    let big_source = scratch.at("big.blend");
    std::fs::write(&big_source, &big).expect("written");
    let big_destination = format!("{there}/big.blend");
    let mut big_record = a_transfer_record(
        Direction::Up,
        &big_source.to_string_lossy(),
        &big_destination,
        &format!("{there}/.smoozy-staging"),
        big.len() as u64,
        Digest::of(&big),
    );
    let mut a_cap = ACapThatBites {
        left: 5 * 1024 * 1024,
    };
    let first = tokio::time::timeout(
        Duration::from_secs(300),
        carry_out(
            Transfer {
                subsystem: &mut subsystem,
                local: &mut local,
                budget: &mut a_cap,
                keeper: &mut keeper,
                watcher: &mut watcher,
                inactivity_deadline: Duration::from_secs(120),
                intent: Intent::ReplaceIntended,
            },
            &mut big_record,
        ),
    )
    .await;
    assert!(first.is_err(), "the capped run does not finish");

    // **7.1 clause 1, mid-transfer, checked from the machine**: the staged
    // prefix is there and the final name is not.
    let staged = subsystem
        .stat(big_record.staging_path.as_bytes().as_slice())
        .await
        .expect("the staging path is there");
    let staged_size = staged.size.expect("a size comes back");
    println!("the interrupted run left {staged_size} bytes staged on the rental");
    assert!(staged_size >= 4 * 1024 * 1024);
    assert!(
        subsystem.stat(big_destination.as_bytes()).await.is_err(),
        "the final name is absent until publication"
    );

    // **The resume runs on a fresh connection**, which is what a real
    // interruption leaves behind.
    drop(subsystem);
    connection.tear_down().await;
    let second_connection = a_connection(&rental).await;
    let mut fresh = a_subsystem(&second_connection).await;
    let mut budget = smoozy_server_protocol::transfers::moving::NoCapIsInForce;
    let resumed = tokio::time::timeout(
        Duration::from_secs(600),
        carry_out(
            Transfer {
                subsystem: &mut fresh,
                local: &mut local,
                budget: &mut budget,
                keeper: &mut keeper,
                watcher: &mut watcher,
                inactivity_deadline: Duration::from_secs(120),
                intent: Intent::ReplaceIntended,
            },
            &mut big_record,
        ),
    )
    .await
    .expect("the resumed run finishes");
    assert_eq!(resumed.ending, Ending::Published);
    let published = fresh
        .stat(big_destination.as_bytes())
        .await
        .expect("the final name is there");
    assert_eq!(published.size, Some(big.len() as u64));

    // **And it is the same file**: brought back and compared byte for byte,
    // which is the only thing that says a resume joined the two halves right.
    let back_here = scratch.at("back/big.blend");
    let mut check = a_transfer_record(
        Direction::Down,
        &big_destination,
        &back_here.to_string_lossy(),
        &back_here
            .parent()
            .expect("a folder")
            .join(".smoozy-staging")
            .to_string_lossy(),
        big.len() as u64,
        Digest::of(&big),
    );
    let mut budget = smoozy_server_protocol::transfers::moving::NoCapIsInForce;
    let checked = carry_out(
        Transfer {
            subsystem: &mut fresh,
            local: &mut local,
            budget: &mut budget,
            keeper: &mut keeper,
            watcher: &mut watcher,
            inactivity_deadline: Duration::from_secs(120),
            intent: Intent::CreateIntended,
        },
        &mut check,
    )
    .await;
    assert_eq!(checked.ending, Ending::Published);
    let joined = std::fs::read(&back_here).expect("it came back");
    assert_eq!(Digest::of(&joined), Digest::of(&big));

    // Tidy what this test put on the machine.
    let _ = fresh.remove(destination.as_bytes()).await;
    let _ = fresh.remove(big_destination.as_bytes()).await;
    let _ = fresh.remove_directory(format!("{there}/.smoozy-staging").as_bytes()).await;
    let _ = fresh.remove_directory(there.as_bytes()).await;
}

// ------------------------- 5. the root choice and the five areas, for real

/// **`Job files and outputs` L2 and L4a's choice made on a real machine, and
/// that document's five areas made under it.** Every marker in this section
/// that names a root, an area or a directory is `Job files and outputs`'.
/// `Servers` has an N9 and an N11 of its own — a fresh connection per attempt,
/// and the restart mark's suppression — and neither is meant anywhere here.
///
/// **This is not `Server Setup`'s hand-over**, which is step 6's and is built —
/// its rules at `App/Server Setup/src/placement.rs` and `src/elevated.rs`, and
/// its performance at that crate's `acts::carry_out_the_placement`. *(Corrected
/// 2026-09-07: this said the hand-over was not built, which stopped being true
/// when step 6 closed. The sentence it introduces — that this test is not that
/// hand-over — was and is right.)*
/// What is driven here is every piece of this document step 4 built, against a
/// real filesystem: the per-path readings the choice is made on, the choice
/// itself, the root and the five areas made one segment at a time with
/// `Job files and outputs` N9's plain form, and that document's L11 and L12's
/// confirmation by a real write and a read-back.
#[tokio::test]
async fn the_root_choice_and_its_five_areas_are_made_on_a_real_machine() {
    let rental = the_rental_or_say_so!();
    let connection = a_connection(&rental).await;
    let mut subsystem = a_subsystem(&connection).await;

    // **L2: the candidate set is the writable locations `Server Setup` was
    // given.** Nothing here enumerates the machine's filesystems, which is
    // Fact 28's trap. The rental names them.
    let mut candidates = Vec::new();
    for location in [Some(rental.root.clone()), rental.second_root.clone()]
        .into_iter()
        .flatten()
    {
        let reading = free_space::read_it(&mut subsystem, &location).await.ok();
        println!(
            "candidate {location}: {:?}",
            reading.as_ref().map(|it| (
                it.available_bytes,
                it.the_path_it_was_taken_against.clone()
            ))
        );
        candidates.push(Candidate {
            writable_location: location,
            reading,
        });
    }

    let choice = choose(&candidates).expect("a root is chosen on a real machine");
    println!("--- L2 and L4a, on the rental's own figures ---");
    println!("root: {}", choice.layout.root().as_path());
    println!(
        "package area: {:?}",
        choice.layout.where_the_package_area_sits()
    );

    let layout: &TheLayout = &choice.layout;
    // **`Job files and outputs` N11: the five areas exist before the first
    // job**, made one segment at a time with that document's N9 plain form, so
    // no area is ever created deep. **L4a's second location is made before the
    // area that sits in it**, which is the ordering a hand-over has to keep on
    // a machine with two mounts.
    subsystem
        .make_one_directory(layout.root().as_path())
        .await
        .expect("the root is made on the machine");
    if let WhereThePackageAreaSits::ApartAt(apart) = layout.where_the_package_area_sits() {
        subsystem
            .make_one_directory(apart.as_path())
            .await
            .expect("L4a's second location is made");
    }
    for area in EVERY_AREA {
        subsystem
            .make_one_directory(&layout.area(area))
            .await
            .expect("an area is made on the machine");
    }

    // **`Job files and outputs` N9's plain form refuses the second creation**,
    // which is the silent
    // collision path not existing to be taken. Observed against the rental's
    // own server.
    assert!(subsystem
        .make_directory(layout.root().as_path().as_bytes())
        .await
        .is_err());

    // **L11 and L12 against a real machine**: a real write of at least one
    // byte at the exact path, read back under the login the render will run
    // as, and nothing left behind.
    let confirmed = confirm_the_layout(&mut subsystem, layout).await;
    println!("L10's confirmation: {confirmed:?}");
    for (_, found) in &confirmed {
        assert_eq!(found, &WhatTheConfirmationFound::ItIsPlaced);
    }

    // **The five areas are there, read off the machine's own listing.**
    //
    // **L6: no path in any one of them is ever under another**, so where L4a
    // put the package area on a second mount the root lists four and that
    // mount lists the fifth. The listing is asked of whichever directory the
    // layout says the area is in, rather than of the root alone.
    let mut listings: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();
    for area in EVERY_AREA {
        let path = layout.area(area);
        let holder = path
            .rsplit_once('/')
            .expect("an area sits under something")
            .0
            .to_string();
        let names = match listings.get(&holder) {
            Some(already) => already.clone(),
            None => {
                let read = subsystem
                    .list_one_directory(&holder)
                    .await
                    .expect("the directory lists");
                let names: Vec<String> = read
                    .iter()
                    .map(|name| String::from_utf8_lossy(name).into_owned())
                    .collect();
                println!("what the machine lists under {holder}: {names:?}");
                listings.insert(holder.clone(), names.clone());
                names
            }
        };
        assert!(
            names.iter().any(|name| name == area.segment()),
            "{area:?} is on the machine"
        );
    }

    // **The root records what H10's reading needs**, which is the field D3
    // gained: a path and the installation identity it was read under.
    let recorded = RecordedMachineRoot {
        path: layout.root().as_path().to_string(),
        the_package_areas_own_location: match layout.where_the_package_area_sits() {
            WhereThePackageAreaSits::ApartAt(apart) => Some(apart.as_path().to_string()),
            WhereThePackageAreaSits::UnderTheRoot => None,
        },
        read_under: Identity::issue(IdentityKind::Installation)
            .expect("randomness")
            .as_text(),
    };
    let mut row = a_row(&rental);
    row.recorded_machine_root = Some(recorded);

    // **And with a root recorded, H10's figure is taken and names the path it
    // was taken against** — the whole point of the column.
    let mut command_connection = a_connection(&rental).await;
    let reading = readings::take_the_hardware_reading(
        &mut command_connection,
        &row.local_server_identifier,
        None,
        row.recorded_machine_root.as_ref().map(|it| it.path.as_str()),
        SystemTime::now(),
    )
    .await
    .expect("a reading off the rented machine");
    let free = reading
        .free_space
        .as_ref()
        .expect("a root was recorded, so the figure is taken");
    println!(
        "H10 with a recorded root: {} bytes against {}",
        free.available_bytes, free.the_path_it_was_taken_against
    );
    assert_eq!(
        free.the_path_it_was_taken_against,
        layout.root().as_path()
    );
    command_connection.tear_down().await;

    // Tidy what this test made.
    for area in EVERY_AREA {
        let _ = subsystem.remove_directory(layout.area(area).as_bytes()).await;
    }
    if let WhereThePackageAreaSits::ApartAt(apart) = layout.where_the_package_area_sits() {
        let _ = subsystem.remove_directory(apart.as_path().as_bytes()).await;
    }
    let _ = subsystem
        .remove_directory(layout.root().as_path().as_bytes())
        .await;
}
