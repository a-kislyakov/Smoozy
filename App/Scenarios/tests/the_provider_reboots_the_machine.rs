//! **`Preparation/Scenarios/The provider reboots the machine.md`, played at
//! the desk** — step 12's piece SC3.
//!
//! The scenario's situation and its seven steps, unchanged, through the
//! application's own acts, against the desk machine. At each step the test
//! prints what the user did, what Smoozy hands the page to show, and which
//! clause says so, and asserts all three. **A step the desk cannot play is said
//! in the output and never skipped quietly.**
//!
//! **What is real here.** OpenSSH's own server behind the harness's line; the
//! application's own transport, opened by `Servers`' own driving of a connected
//! period — the user's *Connect*, the repair, its runs and its pauses — with
//! the plan's own figures (`Servers` K3's keepalive window, `Servers` N2's
//! attempts, `Servers` N2a's run span and `Servers` N6's standing pause); the
//! run's own register of periods and its own publisher of what `Servers`
//! supplies, over the desk's own record; `Servers` Q1 step 7 and the
//! application's own live-reading sequence (`Desktop app and delivery` P10d)
//! on every connection; the desktop's own submit act; the Queue and History
//! screens' own readings; and the machine's own program — its real
//! supervisor, killed with everything else on the machine and started again
//! from its package area, its real reconciliation and its real render worker.
//! **What is stood in for**: Blender, by the harness's script; the operating
//! system's notification service, by a recorder of what Smoozy asked it to
//! show; and the machine's boot registration, by the harness starting the
//! supervisor again through the package area's pointer, as that registration
//! starts it.
//!
//! **Where the application's own wiring needs the window, it is mirrored and
//! said**, exactly as in SC1 (`tests/the_connection_drops_while_rendering.rs`,
//! whose period, hand-over and passes this file copies): the period's thread,
//! the hand-over at the moment a connection is lent, and the thread step 7
//! runs on. What runs inside them is the application's own.
//!
//! **What the desk cannot do.**
//!
//! - **A real power cut.** The harness kills every program on the machine at
//!   once and drops every packet, which is what the desk sees of an abrupt
//!   reboot; it does not cut a disk's power, so what a real power cut leaves
//!   of a frame being written is not played (`Queue & History`'s spec calls
//!   that case unresolved). The stand-in writes each frame whole or not at
//!   all. The rented pass plays a provider's own restart.
//! - **Boot registration itself.** Whether a provider's machine really starts
//!   Smoozy's supervisor at boot is a rental's to show; here the harness
//!   starts it through the same pointer.
//! - **The notification on screen.** What Smoozy hands the system is recorded
//!   instead.
//!
//! **Nothing of the Creator's is touched**: see the head of
//! `src/the_desk_machine.rs`.

#![cfg(unix)]

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime};

use smoozy_desktop::downloading::TheRunsTransfers;
use smoozy_desktop::events::TheOneQueue;
use smoozy_desktop::notifications::kinds::Kind;
use smoozy_desktop::notifications::permission::WhatTheSystemSays;
use smoozy_desktop::notifications::{APost, TheMachinery, TheNotificationService, ThePlatform};
use smoozy_desktop::presentation::the_presentation_key::ThePresentationKey;
use smoozy_desktop::presentation::the_repairing_server::{the_word_for_the_repair, WhatTheColumnSays};
use smoozy_desktop::record::failure_records::{DurableSubject, FailureRecords};
use smoozy_desktop::record::job_records::JobRecords;
use smoozy_desktop::record::server_rows;
use smoozy_desktop::servers::a_period::{the_facts_behind, ThePeriodsThisRun, TheRunThroughTheApp, WhereThisPeriodPublishes};
use smoozy_desktop::servers::installation::{OverTheConnection, TheMachinesInstallationIdentity};
use smoozy_desktop::servers::live_reading::{the_live_reading_sequence, WhatAPassReads, WhatAPassTakes, WhatIsOnScreen};
use smoozy_desktop::servers::sessions::{lend_until_it_ends, TheAdditionalConnection, ThisServersSessions};
use smoozy_desktop::servers::the_machines_work::{
    a_read_forward_pass, the_queue_reading_from, the_step_seven, AsksTheSupervisor, OverThisServersPoint,
    WhatStepSevenDid, WhatThisRunHolds,
};
use smoozy_desktop::submitting::{the_act_is_performed_naming_its_jobs, WhatBecameOfOneJob, WhatTheActCameTo};
use smoozy_desktop::surface::the_history::{what_is_kept, AnEnding};
use smoozy_desktop::surface::the_queue::{what_is_held, ARowInTheQueue, TheQueueOnScreen, TheStopOrCancel};
use smoozy_queue_history::line::QueueState;
use smoozy_scenarios::the_desk_machine::{HowTheOldConnectionsEnd, TheDesk, TheDeskMachine, TheDesksRecord};
use smoozy_server_protocol::transport::ssh::{self, Connection};
use smoozy_server_protocol::transport::{Credential, DecideHostTrust, Ending, Endpoint, HostTrust, OpenFailure};
use smoozy_servers::acting::{OpenConnections, SupplyTheCredential, TheActingHalf, TheServerBeingConnectedTo, WhatServersPublishes};
use smoozy_servers::condition::{presentation, ObservedValue, Presentation};
use smoozy_servers::driving::{APeriod, EndThePeriod, HowTheDrivingEnded, TheDesktopsSide};
use smoozy_servers::hardware::HardwareReading;
use smoozy_servers::identity::KeyReference;
use smoozy_servers::key_moments::{TheTwoAnswers, WhoStartedIt};
use smoozy_servers::period::{OpeningAct, PeriodEnding, WhoTookIt};
use smoozy_servers::repair::{NextAttemptDue, PermanentRefusal, WhatTheRecordSays};
use smoozy_servers::store::{RememberedHostKey, ServerRow};
use smoozy_servers::wiring::{TheDesktopsClock, TheTransport, K3S_FIGURES};
use smoozy_state::identities::{Identity, IdentityKind, InstallationCheck};
use smoozy_state::observed::{Clock, Reading};

// ════════════════════════════════════════════════════════════════════════════
// What the operating system was asked to show
// ════════════════════════════════════════════════════════════════════════════

/// **The system's notification service, standing in**: it is in place, the
/// permission is granted, and every post and every take-down is written down
/// rather than shown.
#[derive(Clone, Default)]
struct WhatTheSystemWasAsked(Arc<Mutex<Vec<String>>>);

impl WhatTheSystemWasAsked {
    fn every(&self) -> Vec<String> {
        self.0.lock().expect("the posts").clone()
    }
}

impl ThePlatform for WhatTheSystemWasAsked {
    fn the_machinery(&self) -> TheMachinery {
        TheMachinery::ItIsInPlace
    }
    fn the_permission(&self) -> WhatTheSystemSays {
        WhatTheSystemSays::ItIsGranted
    }
    fn ask_for_permission_now(&self) {}
    fn post(&self, post: &APost) {
        self.0.lock().expect("the posts").push(format!(
            "posted {:?}: {:?} {:?} {:?}",
            post.kind, post.about, post.said, post.body
        ));
    }
    fn take_down(&self, key: &ThePresentationKey) {
        self.0.lock().expect("the posts").push(format!("taken down {key:?}"));
    }
}

// ════════════════════════════════════════════════════════════════════════════
// The user's Connect: a period, driven by `Servers`' own driving
// ════════════════════════════════════════════════════════════════════════════

/// **What `Servers` supplied, each value with its moment** — forwarded whole to
/// the run's own publisher, which holds the value for the screens, writes the
/// condition record and tells the operating system (`Servers` N5,
/// `Desktop app and delivery` T13).
#[derive(Clone)]
struct WhatServersSupplied {
    the_runs_own: WhereThisPeriodPublishes,
    values: Arc<Mutex<Vec<(Instant, ObservedValue)>>>,
    raised: Arc<Mutex<Vec<Instant>>>,
    withdrawn: Arc<AtomicUsize>,
}

impl WhatServersSupplied {
    fn now(&self) -> Option<ObservedValue> {
        self.values.lock().expect("the values").last().map(|(_, value)| *value)
    }

    fn every_value(&self) -> Vec<ObservedValue> {
        self.values.lock().expect("the values").iter().map(|(_, value)| *value).collect()
    }

    /// When the value first left *connected* after `since`.
    fn left_connected_after(&self, since: Instant) -> Option<Instant> {
        self.values
            .lock()
            .expect("the values")
            .iter()
            .find(|(at, value)| *at >= since && *value != ObservedValue::Connected)
            .map(|(at, _)| *at)
    }
}

impl WhatServersPublishes for WhatServersSupplied {
    fn the_observed_value(&mut self, server: &str, value: ObservedValue) {
        self.values.lock().expect("the values").push((Instant::now(), value));
        self.the_runs_own.the_observed_value(server, value);
    }
    fn the_next_attempt_due(&mut self, server: &str, reading: NextAttemptDue) {
        self.the_runs_own.the_next_attempt_due(server, reading);
    }
    fn raise_the_connection_condition(&mut self, server: &str, says: WhatTheRecordSays) {
        self.raised.lock().expect("the raises").push(Instant::now());
        self.the_runs_own.raise_the_connection_condition(server, says);
    }
    fn withdraw_the_connection_condition(&mut self, server: &str) {
        self.withdrawn.fetch_add(1, Ordering::SeqCst);
        self.the_runs_own.withdraw_the_connection_condition(server);
    }
    fn the_changed_identity_warning(&mut self, server: &str) {
        self.the_runs_own.the_changed_identity_warning(server);
    }
    fn record_the_scanned_host_key(&mut self, server: &str, key: RememberedHostKey) {
        self.the_runs_own.record_the_scanned_host_key(server, key);
    }
}

/// **`Keys`' end, standing in**: the key has no passphrase and is handed over
/// for every attempt — the scenario's *its key has no passphrase*.
struct TheKeyWithNoPassphrase(std::path::PathBuf);

impl SupplyTheCredential for TheKeyWithNoPassphrase {
    fn the_two_answers(&mut self, _key: &KeyReference) -> TheTwoAnswers {
        TheTwoAnswers {
            the_key_is_usable: true,
            a_passphrase_is_available_without_asking: true,
        }
    }
    fn for_one_attempt(&mut self, _key: &KeyReference, _who: WhoStartedIt, _naming: TheServerBeingConnectedTo<'_>) -> Option<Credential> {
        Some(Credential {
            private_key_file: self.0.clone(),
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

/// **The desktop's side of the driving, as `src/servers/a_period.rs` keeps
/// it**: the row read again from the record at each attempt, the fresh
/// hardware reading written to the row, and each connection's branch kept for
/// the hand-over.
#[derive(Clone)]
struct TheDesktopsOwnSide {
    record: Arc<TheDesksRecord>,
    branches: Arc<Mutex<Vec<InstallationCheck>>>,
}

impl TheDesktopsSide for TheDesktopsOwnSide {
    fn the_row_now(&mut self, server: &str) -> Option<ServerRow> {
        self.record.read(|connection| server_rows::one(connection, server).ok().flatten())
    }

    fn the_connection_read(&mut self, server: &str, check: InstallationCheck, hardware: Option<HardwareReading>) {
        if let Some(reading) = hardware {
            self.record.read(|connection| {
                if let Ok(Some(mut row)) = server_rows::one(connection, server) {
                    row.remembered_hardware_reading = Some(reading);
                    let _ = server_rows::write(connection, &row);
                }
            });
        }
        self.branches.lock().expect("the branches").push(check);
    }
}

/// The host-trust decision an additional connection makes: whatever key is
/// offered is taken, as the harness's own connections take it.
struct Proceeds;

impl DecideHostTrust for Proceeds {
    fn decide(&mut self, _offered: &[u8]) -> HostTrust {
        HostTrust::Proceed
    }
}

/// **`Servers`' own transport, with the wait on a live connection made the
/// point's lending** — `src/servers/a_period.rs`'s `TheTransportThroughThePoint`,
/// mirrored: the opening, the ending and the tear-down are `Servers`' own
/// transport unchanged, and the wait lends the connection through the server's
/// point and hands the connection over the moment it is first lent.
struct TheTransportThroughThePoint {
    sessions: ThisServersSessions,
    additional: TheAdditionalConnection<Connection>,
    when_it_is_lent: Arc<dyn Fn(&ThisServersSessions) + Send + Sync>,
    when_it_ends: Arc<dyn Fn() + Send + Sync>,
}

impl OpenConnections for TheTransportThroughThePoint {
    type Connection = Connection;

    async fn open(
        &mut self,
        endpoint: Endpoint,
        credential: Credential,
        decide: Box<dyn DecideHostTrust>,
        deadline: Duration,
    ) -> Result<Connection, OpenFailure> {
        TheTransport.open(endpoint, credential, decide, deadline).await
    }

    fn ended(&mut self, connection: &Connection) -> Option<Ending> {
        TheTransport.ended(connection)
    }

    async fn wait_until_it_ends(&mut self, connection: &mut Connection) -> Ending {
        let sessions = self.sessions.clone();
        let additional = Arc::clone(&self.additional);
        let when_it_is_lent = Arc::clone(&self.when_it_is_lent);
        let lending = std::pin::pin!(lend_until_it_ends(&sessions, connection, &additional));
        let handed_over = async {
            when_it_is_lent(&sessions);
            std::future::pending::<std::convert::Infallible>().await
        };
        let ending = tokio::select! {
            biased;
            ending = lending => ending,
            never = handed_over => match never {},
        };
        (self.when_it_ends)();
        ending
    }

    async fn tear_down(&mut self, connection: Connection) -> Ending {
        TheTransport.tear_down(connection).await
    }
}

/// **What one connection's step 7 did**, with the connection's number.
type WhatEachConnectionDid = Arc<Mutex<Vec<(u64, WhatStepSevenDid)>>>;

/// **A connected period standing for the desk machine's server**, its driving
/// on a thread of its own as the application drives one.
struct TheConnectedPeriod {
    supplied: WhatServersSupplied,
    branches: Arc<Mutex<Vec<InstallationCheck>>>,
    each_connection: WhatEachConnectionDid,
    sessions: ThisServersSessions,
    ending: EndThePeriod,
    driving: Option<std::thread::JoinHandle<HowTheDrivingEnded>>,
}

impl TheConnectedPeriod {
    /// **The user's *Connect*** — `Servers` S5's first opening act, taken by
    /// the user, and a period begun at the act.
    fn begun(machine: &TheDeskMachine, desk: &TheDesk, periods: &Arc<ThePeriodsThisRun>) -> TheConnectedPeriod {
        let row = machine.the_row().clone();
        let server = row.local_server_identifier.clone();
        let supplied = WhatServersSupplied {
            the_runs_own: periods.where_a_period_publishes(),
            values: Arc::default(),
            raised: Arc::default(),
            withdrawn: Arc::default(),
        };
        let side = TheDesktopsOwnSide {
            record: Arc::clone(desk.record()),
            branches: Arc::default(),
        };
        let each_connection: WhatEachConnectionDid = Arc::default();
        let sessions = ThisServersSessions::for_one_server();
        let additional: TheAdditionalConnection<Connection> = {
            let (endpoint, key) = (machine.the_endpoint(), machine.the_credential().private_key_file);
            Arc::new(move || {
                let endpoint = endpoint.clone();
                let credential = Credential {
                    private_key_file: key.clone(),
                    passphrase: None,
                };
                Box::pin(async move {
                    tokio::time::timeout(
                        smoozy_servers::repair::CONNECT_DEADLINE,
                        ssh::open(&endpoint, credential, K3S_FIGURES, Box::new(Proceeds)),
                    )
                    .await
                    .ok()?
                    .ok()
                })
            })
        };
        // **The hand-over, the moment a connection is lent** — what
        // `the_machines_work_begins` does in the running application: the
        // connection is counted, and step 7 and the passes after it run on a
        // thread of their own, only on the confirmed branch.
        let when_it_is_lent: Arc<dyn Fn(&ThisServersSessions) + Send + Sync> = {
            let (periods, server, record) = (Arc::clone(periods), server.clone(), Arc::clone(desk.record()));
            let (branches, each_connection, package_area) =
                (Arc::clone(&side.branches), Arc::clone(&each_connection), machine.package_area());
            Arc::new(move |sessions: &ThisServersSessions| {
                let connection = periods.the_machines_work().a_connection_begins(&server);
                let confirmed = branches.lock().expect("the branches").last()
                    == Some(&InstallationCheck::PresentAndEqualConfirmed);
                let (periods, server, record, sessions, package_area, each_connection) = (
                    Arc::clone(&periods),
                    server.clone(),
                    Arc::clone(&record),
                    sessions.clone(),
                    package_area.clone(),
                    Arc::clone(&each_connection),
                );
                std::thread::spawn(move || {
                    if confirmed {
                        the_work_on_one_connection(&periods, &server, &sessions, connection, &record, package_area, &each_connection);
                    }
                });
            })
        };
        // **`Servers` K3 and `Servers` K5: the connection's ending**, the moment the
        // transport reports it — `the_machines_work_stops`.
        let when_it_ends: Arc<dyn Fn() + Send + Sync> = {
            let (periods, server) = (Arc::clone(periods), server.clone());
            Arc::new(move || periods.the_machines_work().the_connection_ended(&server))
        };
        let mut half = TheActingHalf {
            opener: TheTransportThroughThePoint {
                sessions: sessions.clone(),
                additional,
                when_it_is_lent,
                when_it_ends,
            },
            credentials: TheKeyWithNoPassphrase(machine.the_credential().private_key_file),
            installation: TheMachinesInstallationIdentity::for_this_server(&row, OverTheConnection).reading_the_row_as_it_stands({
                let (record, server) = (Arc::clone(desk.record()), server.clone());
                move || record.read(|connection| server_rows::one(connection, &server).ok().flatten())
            }),
            publishes: supplied.clone(),
            wait: TheDesktopsClock,
        };
        let mut period = APeriod::begun(&server, OpeningAct::TheUsersConnectAction, WhoTookIt::TheUser, SystemTime::now());
        let ending = period.end_it();
        let branches = Arc::clone(&side.branches);
        let runtime = tokio::runtime::Handle::current();
        let driving = std::thread::spawn(move || {
            let mut side = side;
            runtime.block_on(period.drive(&row, &mut half, &mut side))
        });
        TheConnectedPeriod {
            supplied,
            branches,
            each_connection,
            sessions,
            ending,
            driving: Some(driving),
        }
    }

    /// How many connections' step 7 has run, and what the last one did.
    fn the_step_sevens(&self) -> Vec<(u64, WhatStepSevenDid)> {
        self.each_connection.lock().expect("the connections").clone()
    }

    /// **`Servers` S10 item 1: the user disconnects**, and the driving is
    /// waited for no longer than `within`.
    async fn the_user_disconnects(mut self, within: Duration) -> Option<HowTheDrivingEnded> {
        self.ending.at(PeriodEnding::TheUserDisconnects);
        let driving = self.driving.take()?;
        let began = Instant::now();
        while !driving.is_finished() {
            if began.elapsed() > within {
                return None;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        driving.join().ok()
    }
}

impl Drop for TheConnectedPeriod {
    fn drop(&mut self) {
        self.ending.at(PeriodEnding::TheApplicationRunEnds);
    }
}

/// **`Servers` Q1 step 7 on one connection, then the live-reading sequence**
/// — `the_work_on_one_connection` in `src/servers/the_machines_work.rs`, with
/// what needs the window left out: the staging sweep, the words and previews
/// fetched beside, the screens told again, step 9's update and the driver's
/// hold, and the idle re-check. None of them is this scenario's.
fn the_work_on_one_connection(
    periods: &ThePeriodsThisRun,
    server: &str,
    sessions: &ThisServersSessions,
    connection: u64,
    record: &TheDesksRecord,
    package_area: String,
    each_connection: &WhatEachConnectionDid,
) {
    let identity = Identity::read_back(IdentityKind::LocalServer, server).expect("the row's own identity");
    let holds = periods.the_machines_work();
    let is_live = || sessions.a_connection_is_live() && holds.is_current(server, connection);
    let mut machine = OverThisServersPoint {
        sessions: sessions.clone(),
        package_area,
    };
    let did = the_step_seven(&identity, connection, record, &mut machine, holds, &is_live);
    each_connection.lock().expect("the connections").push((connection, did));
    let mut passes = ThePassesAtTheDesk {
        holds,
        server: &identity,
        connection,
        record,
        machine: &mut machine,
        is_live: &is_live,
    };
    let Ok(runtime) = tokio::runtime::Builder::new_current_thread().enable_all().build() else {
        return;
    };
    runtime.block_on(the_live_reading_sequence(server, &mut passes, &mut TheDesktopsClock));
}

/// **What a pass of `Desktop app and delivery` P10d takes at the desk**, as `ThePassesOverThisConnection`
/// takes it: the read forward every pass, and the queue reading because the
/// user is watching the queue. The commentary is not on screen.
struct ThePassesAtTheDesk<'a> {
    holds: &'a WhatThisRunHolds,
    server: &'a Identity,
    connection: u64,
    record: &'a TheDesksRecord,
    machine: &'a mut dyn AsksTheSupervisor,
    is_live: &'a dyn Fn() -> bool,
}

impl WhatAPassTakes for ThePassesAtTheDesk<'_> {
    fn is_on_screen(&mut self, _server: &str, what: WhatIsOnScreen) -> bool {
        what == WhatIsOnScreen::TheServersQueueOrRunningEntry
    }

    fn the_period_stands_on_a_live_connection(&mut self, _server: &str) -> bool {
        (self.is_live)()
    }

    fn take(&mut self, server: &str, read: WhatAPassReads) {
        match read {
            WhatAPassReads::TheReadForwardFromTheEventCursor => {
                let _ = a_read_forward_pass(self.server, self.record, &mut *self.machine, self.holds);
            }
            WhatAPassReads::TheQueueReading => {
                if let Ok(state) = the_queue_reading_from(&mut *self.machine) {
                    if (self.is_live)() {
                        self.holds.a_reading_was_taken(
                            server,
                            self.connection,
                            Reading::taken(state, SystemTime::now(), Clock::ThisComputer),
                            None,
                        );
                    }
                }
            }
            WhatAPassReads::TheRunningEntrysCommentaryRead => {}
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// Reading what the user sees
// ════════════════════════════════════════════════════════════════════════════

/// **A wait with its own bound**: `seen` is asked every quarter second, and a
/// wait past `within` fails the test with what it was waiting for.
async fn until(said: &str, within: Duration, mut seen: impl FnMut() -> bool) -> Duration {
    let began = Instant::now();
    while !seen() {
        assert!(began.elapsed() < within, "{said}: not seen within {within:?}");
        tokio::time::sleep(Duration::from_millis(250)).await;
    }
    began.elapsed()
}

/// **The job as the period-held reading last read it** (`Queue & History`
/// C6), where one is held.
fn the_job_as_last_read(
    periods: &ThePeriodsThisRun,
    server: &str,
    job: &Identity,
) -> Option<smoozy_queue_history::readings::AJobAsTheMachineReadsIt> {
    periods
        .the_machines_work()
        .every_server()
        .into_iter()
        .find(|(held, _)| held == server)
        .and_then(|(_, period)| period.reading)
        .and_then(|reading| reading.value().jobs.iter().find(|one| &one.job == job).cloned())
}

/// The newest frame the worker's figures say is saved, where a rendering
/// job carries figures (`Queue & History` R5, `Queue & History` P1a).
fn the_newest_frame_saved(job: &smoozy_queue_history::readings::AJobAsTheMachineReadsIt) -> Option<i64> {
    job.live.the_figures().and_then(|figures| figures.newest_frame_saved)
}

/// **What the server row's connection reads, composed as the server list
/// composes it** — `Servers` K8a's presentation out of what `Servers` supplied
/// and the one fact the value cannot carry, that a period stands.
fn the_rows_presentation(supplied: &WhatServersSupplied) -> Presentation {
    presentation(the_facts_behind(
        supplied.now().unwrap_or(ObservedValue::Disconnected),
        true,
        false,
    ))
}

/// **The condition records standing against the server's row.**
fn the_records_against(desk: &TheDesk, server: &Identity) -> Vec<smoozy_desktop::record::failure_records::Standing> {
    desk.record().read(|connection| {
        FailureRecords::over(connection)
            .standing_against(&DurableSubject::ServerRow(server.clone()))
            .unwrap_or_default()
    })
}

/// **The job's row in the queue**, where it has one.
fn the_row_of<'a>(screen: &'a TheQueueOnScreen, job: &str) -> Option<&'a ARowInTheQueue> {
    screen.lines.iter().flat_map(|line| line.rows.iter()).find(|row| row.job == job)
}

/// One step's account line, printed for the output.
fn said(step: &str, what: impl AsRef<str>) {
    println!("SC3 {step}: {}", what.as_ref());
}

// ════════════════════════════════════════════════════════════════════════════
// The scenario
// ════════════════════════════════════════════════════════════════════════════

/// **The provider reboots the machine**, its situation and its seven steps.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn the_provider_reboots_the_machine() {
    let desk = TheDesk::opened("sc3").expect("the desk's record opens");
    let Some(machine) = TheDeskMachine::stood_up("sc3", &desk, "RunPod 4090").await else {
        return;
    };
    let server_text = machine.the_row().local_server_identifier.clone();
    let server = Identity::read_back(IdentityKind::LocalServer, &server_text).expect("the row's own identity");

    // ── The situation ────────────────────────────────────────────────────
    let the_system = WhatTheSystemWasAsked::default();
    let notifications = Arc::new(TheNotificationService::on(Box::new(the_system.clone())));
    notifications.the_switches_are(Some(smoozy_desktop::record::settings::TheSwitches::nobody_has_turned_one()));
    let periods = Arc::new(ThePeriodsThisRun::over(
        TheRunThroughTheApp::lending_the_record_of(Arc::clone(desk.record().the_start())),
        Arc::new(TheOneQueue::new()),
        Arc::clone(&notifications),
    ));
    // **"Smoozy is connected to it"**: the user's *Connect*, and the period's
    // first connection reads the confirmed branch and runs step 7.
    let connected = TheConnectedPeriod::begun(&machine, &desk, &periods);
    until("the server reads connected and its first connection's step 7 has run", Duration::from_secs(90), || {
        connected.supplied.now() == Some(ObservedValue::Connected) && !connected.the_step_sevens().is_empty()
    })
    .await;
    assert_eq!(
        connected.branches.lock().expect("the branches").last(),
        Some(&InstallationCheck::PresentAndEqualConfirmed),
        "`State, identity and recovery` V1: the connection settled on the confirmed branch: {}",
        machine.the_supervisors_complaints()
    );

    // **"A 100-frame animation job is submitted … actively rendering … the
    // first 20 frames have finished. Auto-download is off."** By the
    // desktop's own submit act, the stand-in held before frame 21.
    machine.the_stand_in().takes_this_long_a_frame(Duration::from_millis(50));
    machine.the_stand_in().holds_before_frame(21);
    let given = machine.what_the_page_hands_over(&desk, 1..=100, "Shot 40");
    assert!(!given.the_download_as_it_renders_choice, "auto-download is off");
    let (sessions, record) = (connected.sessions.clone(), Arc::clone(desk.record()));
    let (came_to, jobs) = tokio::task::spawn_blocking(move || {
        the_act_is_performed_naming_its_jobs(given, &sessions, &TheRunsTransfers::for_this_run(), record.as_ref())
    })
    .await
    .expect("the act ran");
    assert_eq!(
        came_to,
        WhatTheActCameTo::TheseJobs(vec![WhatBecameOfOneJob::ItIsInTheLine]),
        "the job is in the machine's line: {}",
        machine.the_supervisors_complaints()
    );
    let job = jobs[0].job.clone();
    let job_text = job.as_text();
    until("the queue's reading shows frames 1 to 20 saved", Duration::from_secs(90), || {
        the_job_as_last_read(&periods, &server_text, &job)
            .is_some_and(|read| matches!(read.state, QueueState::Rendering { .. }) && the_newest_frame_saved(&read) == Some(20))
    })
    .await;

    // ── Step 1: the user watches the progress as frames complete ─────────
    {
        let read = the_job_as_last_read(&periods, &server_text, &job).expect("the job is read");
        let figures = read.live.the_figures().expect("`Queue & History` R5: a rendering job carries the worker's figures");
        let held = desk.record().held();
        let screen = what_is_held(periods.the_machines_work(), Some(&*held), Some(&job_text));
        let row = the_row_of(&screen, &job_text).expect("the job has its row in the queue");
        assert!(!row.not_updating, "`Queue & History` C7: the reading is live");
        let entry = screen.entry.as_ref().expect("the job's entry");
        assert_eq!(entry.name, "Shot 40");
        assert_eq!(entry.stop_or_cancel, Some(TheStopOrCancel::Stop), "a rendering row offers Stop");
        let progress = entry.progress.as_ref().expect("Design, *A rendering entry*: the progress is drawn");
        assert_eq!(figures.newest_frame_saved, Some(20), "`Queue & History` P1a: frames 1 to 20 are saved");
        said(
            "STEP 1 (played)",
            format!(
                "the queue shows Shot 40 rendering, frame in hand {:?}, {} frames reached, the newest saved {:?}; \
                 its bars {:?} and {:?}; the row's state {:?}, updating",
                figures.readings.value().frame_in_hand,
                figures.readings.value().frames_completed,
                figures.newest_frame_saved,
                progress.now_at,
                progress.whole,
                row.state,
            ),
        );
    }
    assert_eq!(machine.the_stand_in().frames_written(), (1..=20).collect::<Vec<i64>>());

    // ── Step 2: the provider reboots the machine, abruptly ───────────────
    // ── Step 3: the user looks at what Smoozy reports ────────────────────
    // ── Step 4: three minutes pass, and the machine comes back ───────────
    // **The harness's reboot**: the line goes silent, every program on the
    // machine is killed at once — the supervisor, the render worker, the
    // stand-in Blender — and after three minutes the supervisor is started
    // again from its package area and the line comes back, the connection
    // made before the reboot answered with a reset, as a fresh boot answers
    // a connection it knows nothing of. **Steps 3 and 4 are read while it is
    // down.**
    let rebooted_at = Instant::now();
    let three_minutes = Duration::from_secs(180);
    let (rebooted, ()) = tokio::join!(machine.reboots(three_minutes, HowTheOldConnectionsEnd::WithAReset), async {
        // **Step 2's consequences, as the machine holds them.**
        assert!(!machine.the_supervisor_is_running(), "the reboot killed the supervisor");
        tokio::time::sleep(Duration::from_secs(1)).await;
        let written = machine.the_stand_in().frames_written();
        assert_eq!(written, (1..=20).collect::<Vec<i64>>(), "frame 21 was in hand and never written");
        // **The stand-in for the second attempt**: the hold on frame 21 goes
        // with the killed render, and the re-run is held before frame 60, so
        // it is still rendering when Smoozy reconnects.
        machine.the_stand_in().lets_frame_go(21);
        machine.the_stand_in().holds_before_frame(60);
        said(
            "STEP 2 (played, by the harness)",
            "the line went silent and every program on the machine was killed at once — the supervisor, the \
             render worker and the stand-in Blender, frame 21 in hand and unwritten; the machine stays down for \
             three minutes. No disk loses its power: what a real power cut leaves is the rented pass's",
        );

        // **Step 3: inside `Servers` K3's window nothing is declared**; then
        // the break, the repair with the user doing nothing (`Servers` N1),
        // the row repairing (`Servers` K8a), and the queue's row not updating
        // (`Queue & History` C7). **`Servers` N5: nothing raised at the
        // break.**
        assert_eq!(the_rows_presentation(&connected.supplied), Presentation::Connected);
        until("`Servers` K3: the break is declared", Duration::from_secs(75), || {
            connected.supplied.left_connected_after(rebooted_at).is_some()
        })
        .await;
        let declared = connected
            .supplied
            .left_connected_after(rebooted_at)
            .map(|at| at.saturating_duration_since(rebooted_at))
            .expect("the break's moment");
        assert!(
            declared >= Duration::from_secs(40) && declared <= Duration::from_secs(66),
            "`Servers` K3: the break is declared 45 to 60 seconds after the line goes silent, and was {declared:?}"
        );
        assert_eq!(the_rows_presentation(&connected.supplied), Presentation::Repairing);
        assert_eq!(
            the_word_for_the_repair(true),
            Some(WhatTheColumnSays::Reconnecting),
            "`Desktop app and delivery` M29: the work column reads Reconnecting… for the whole repair"
        );
        {
            let held = desk.record().held();
            let screen = what_is_held(periods.the_machines_work(), Some(&*held), Some(&job_text));
            let row = the_row_of(&screen, &job_text).expect("the job still has its row, as last read");
            assert!(row.not_updating, "`Queue & History` C7: the job's row reads as not updating");
        }
        assert!(connected.supplied.raised.lock().expect("the raises").is_empty());
        assert!(the_records_against(&desk, &server).is_empty(), "no condition record at the break");
        assert!(the_system.every().is_empty(), "nothing is posted at the break: {:?}", the_system.every());
        said(
            "STEP 3 (played)",
            format!(
                "the break was declared {declared:?} after the machine went down; the server reads {:?} and its \
                 work column {:?}; the job's row in the queue reads as not updating; no record, no notification",
                the_rows_presentation(&connected.supplied),
                the_word_for_the_repair(true).map(|word| word.the_word()),
            ),
        );

        // **Step 4, while the machine is still down: `Servers` N2a and `Servers` N5 —
        // the first run of attempts fails at its 105 seconds and raises the
        // condition**, and `Desktop app and delivery` T13 tells the system.
        until(
            "`Servers` N5: the first run fails and the condition is raised",
            Duration::from_secs(150),
            || !connected.supplied.raised.lock().expect("the raises").is_empty(),
        )
        .await;
        let raised_after = rebooted_at.elapsed();
        // **`Servers` N2a and `Servers` N5's figure: the record lands a determinate 105
        // seconds after the break's declaration**, however many of the run's
        // attempts ran to their 20-second deadline.
        {
            let raised = connected.supplied.raised.lock().expect("the raises")[0];
            let after_the_declaration = raised.saturating_duration_since(rebooted_at + declared);
            let as_the_clause_says = after_the_declaration >= Duration::from_secs(104)
                && after_the_declaration <= Duration::from_secs(108);
            if !as_the_clause_says {
                said(
                    "STEP 4 (FAILED — a finding)",
                    format!(
                        "the first run of attempts, each running to its 20-second deadline in the silence, raised the \
                         condition {after_the_declaration:?} after the break's declaration. `Servers` N2a and `Servers` N5: a \
                         run's failure lands at a determinate 105 seconds after the declaration. The scenario stops \
                         here"
                    ),
                );
            }
            assert!(
                as_the_clause_says,
                "`Servers` N2a and `Servers` N5: the first run raised its record {after_the_declaration:?} after the declaration, \
                 where the clauses give 105 seconds"
            );
        }
        assert!(raised_after < three_minutes, "the first run failed while the machine was down, at {raised_after:?}");
        assert_eq!(the_records_against(&desk, &server).len(), 1, "`Servers` N5: one condition record");
        assert!(
            the_system.every().iter().any(|post| post.contains(&format!("{:?}", Kind::AConnectionCouldNotBeRestored))),
            "`Desktop app and delivery` T13: the could-not-restore notification is handed to the system: {:?}",
            the_system.every()
        );
        assert_eq!(the_rows_presentation(&connected.supplied), Presentation::Repairing, "`Servers` N6: still repairing");
    });
    rebooted.expect("the machine came back: its supervisor answered");
    let back_at = Instant::now();
    assert!(machine.the_supervisor_is_running(), "the supervisor runs again");
    said(
        "STEP 4 (played)",
        format!(
            "the first run of attempts failed while the machine was down: one condition record stands and the \
             system was asked {:?}; the server still reads {:?}. After {:?} the machine came back: its supervisor \
             was started again from its package area and answered, and the line carries again",
            the_system.every(),
            the_rows_presentation(&connected.supplied),
            back_at.saturating_duration_since(rebooted_at),
        ),
    );

    // **`Queue & History`: "Nothing waits on the user, and nothing waits on
    // Smoozy being open."** The re-run is started by the machine itself, and
    // is held at frame 60 before Smoozy has reconnected.
    let rerun_seen = until("the machine re-runs the job by itself", Duration::from_secs(60), || {
        machine.the_stand_in().frames_written().last() == Some(&59)
    })
    .await;
    let connections_when_it_ran = connected.the_step_sevens().len();
    assert_eq!(connections_when_it_ran, 1, "the re-run started with Smoozy not yet reconnected");
    let written = machine.the_stand_in().frames_written();
    let the_second_attempt: Vec<i64> = written[20..].to_vec();

    // ── Step 5: did it reconnect by itself ───────────────────────────────
    {
        let took = until(
            "`Servers` N6: the next run's attempt connects and its step 7 runs",
            Duration::from_secs(150),
            || connected.supplied.now() == Some(ObservedValue::Connected) && connected.the_step_sevens().len() >= 2,
        )
        .await;
        assert_eq!(the_rows_presentation(&connected.supplied), Presentation::Connected, "it reconnected by itself");
        assert_eq!(
            connected.branches.lock().expect("the branches").last(),
            Some(&InstallationCheck::PresentAndEqualConfirmed),
            "`State, identity and recovery` V1: the machine that came back is the same installation"
        );
        assert!(connected.supplied.withdrawn.load(Ordering::SeqCst) >= 1, "`Servers` N10: withdrawn");
        assert!(
            the_records_against(&desk, &server)
                .iter()
                .all(|record| !record.still_holds_at(smoozy_desktop::servers::published::the_written_form(ObservedValue::Connected))),
            "no condition record still holds once connected"
        );
        let (_, step_seven) = connected.the_step_sevens().last().cloned().expect("the second connection's step 7");
        said(
            "STEP 5 (played)",
            format!(
                "it reconnected by itself {took:?} after the machine came back ({rerun_seen:?} after it came back the \
                 machine had already re-run the job up to frame 59 with nobody connected), on the confirmed branch; \
                 the record was withdrawn; step 7 applied {} events; the system was asked {:?}",
                step_seven.events_applied,
                the_system.every(),
            ),
        );
    }

    // ── Step 6: the queue, the history, and what is on the machine ───────
    {
        until("the queue's reading shows the re-run rendering", Duration::from_secs(60), || {
            the_job_as_last_read(&periods, &server_text, &job)
                .is_some_and(|read| matches!(read.state, QueueState::Rendering { .. }) && the_newest_frame_saved(&read) == Some(59))
        })
        .await;
        let held = desk.record().held();
        let screen = what_is_held(periods.the_machines_work(), Some(&*held), Some(&job_text));
        let row = the_row_of(&screen, &job_text).expect("the job is in the queue again");
        assert!(!row.not_updating, "`Queue & History` C7: live again");
        let entry = screen.entry.as_ref().expect("the job's entry");
        assert_eq!(entry.stop_or_cancel, Some(TheStopOrCancel::Stop), "a rendering row offers Stop");
        let marks: Vec<String> = entry
            .marks
            .iter()
            .map(|mark| format!("{} {:?}", mark.said, mark.parts.iter().map(|part| format!("{} {}", part.lead, part.value)).collect::<Vec<_>>()))
            .collect();
        let record = JobRecords::over(held.connection()).read(&job).expect("it reads").expect("it stands");
        let history = what_is_kept(Some(&*held), None);
        let in_history = history.rows.iter().find(|one| one.job == job_text).map(|one| one.ending);
        let notice = screen.notice.clone();
        drop(held);
        let folder = record
            .the_job_output_folder(machine.layout())
            .expect("`Queue & History` N9: the folder chosen at render start is known");
        let on_the_machine: Vec<String> = {
            let mut names: Vec<String> = std::fs::read_dir(&folder)
                .map(|listed| listed.flatten().map(|one| one.file_name().to_string_lossy().into_owned()).collect())
                .unwrap_or_default();
            names.sort();
            names
        };
        assert_eq!(on_the_machine.len(), 59, "the machine holds frames 1 to 59: {on_the_machine:?}");
        // **`Queue & History`, *When the machine goes down*: a re-run covers
        // only the frames that are not there** — the stand-in writes what the
        // attempt's own inventory names, so its list is the machine's.
        assert!(
            the_second_attempt.first().is_some_and(|first| *first >= 20) ,
            "the re-run left the frames already there alone: it wrote {the_second_attempt:?}"
        );
        assert!(
            marks.iter().any(|mark| mark.starts_with("Interrupted and re-run")),
            "`Queue & History` N1, the spec's *Both are marks on the job's own entry*: the queue entry says it was \
             interrupted and re-run: {marks:?}; the record carries {:?}",
            record.what_the_note_carried.re_run_marks
        );
        // **`Design/Queue & History`, *The marks*, the fifth: "it was
        // interrupted and re-run, and from which frame"** — the first frame of
        // the re-run attempt's own list (`Queue & History` N1).
        let from = the_second_attempt.first().copied().expect("the re-run wrote frames");
        assert!(
            entry.marks.iter().any(|mark| mark.said == "Interrupted and re-run"
                && mark.parts.iter().any(|part| part.value == from.to_string())),
            "the re-run mark names the frame the re-run proceeded from, {from}: {marks:?}"
        );
        said(
            "STEP 6 (played)",
            format!(
                "the queue shows Shot 40 rendering again, live, with Stop, its status {:?}, and its marks {marks:?}; \
                 the catch-up notice {notice:?}; History {}; the machine holds {} files in the job's folder, frames 1 \
                 to 20 from before the reboot and the re-run's from frame {:?}",
                entry.status.iter().map(|part| format!("{}{}{}", part.lead, part.value, part.trail)).collect::<String>(),
                match in_history {
                    Some(ending) => format!("holds it, {ending:?}"),
                    None => "does not hold it yet".to_string(),
                },
                on_the_machine.len(),
                the_second_attempt.first(),
            ),
        );
    }

    // ── Step 7: the user looks for a way to get the rest rendered ────────
    {
        // **Nothing is the user's to do** (`Queue & History`: *"The machine
        // re-runs it, without asking … The ordinary case — a machine
        // rebooting once — needs nothing from the user"*). The re-run goes
        // on, and the job ends finished with every frame.
        machine.the_stand_in().lets_frame_go(60);
        until("the re-run finishes and History holds the job finished", Duration::from_secs(90), || {
            let held = desk.record().held();
            what_is_kept(Some(&*held), Some(&job_text))
                .rows
                .iter()
                .any(|row| row.job == job_text && row.ending == Some(AnEnding::Finished))
        })
        .await;
        let held = desk.record().held();
        let history = what_is_kept(Some(&*held), Some(&job_text));
        let row = history.rows.iter().find(|row| row.job == job_text).expect("in History");
        let entry = history.entry.as_ref().expect("its History entry");
        let marks: Vec<String> = entry
            .entry
            .marks
            .iter()
            .map(|mark| format!("{} {:?}", mark.said, mark.parts.iter().map(|part| format!("{} {}", part.lead, part.value)).collect::<Vec<_>>()))
            .collect();
        let screen = what_is_held(periods.the_machines_work(), Some(&*held), None);
        assert!(
            screen.lines.iter().all(|line| line.rows.iter().all(|one| one.job != job_text)),
            "`Queue & History` C11: it has left the queue"
        );
        let record = JobRecords::over(held.connection()).read(&job).expect("it reads").expect("it stands");
        drop(held);
        assert!(
            marks.iter().any(|mark| mark.starts_with("Interrupted and re-run")),
            "`Queue & History` N1: History's entry keeps the re-run mark: {marks:?}"
        );
        let folder = record.the_job_output_folder(machine.layout()).expect("the folder");
        let pictures = std::fs::read_dir(&folder)
            .map(|listed| listed.flatten().filter(|one| one.path().extension().is_some_and(|ext| ext == "png")).count())
            .unwrap_or_default();
        assert_eq!(pictures, 100, "every frame is on the machine");
        let written = machine.the_stand_in().frames_written();
        said(
            "STEP 7 (played)",
            format!(
                "nothing was the user's to do: the machine's re-run carried on and History reads Shot 40 {:?}, \
                 frames {:?}, with its marks {marks:?} and its ways to render again {:?}; the machine holds all 100 \
                 frames, the second attempt having written frames {:?} to {:?}",
                row.ending,
                row.frames,
                entry.again,
                written.get(20),
                written.last(),
            ),
        );
    }

    let values = connected.supplied.every_value();
    let ended = connected.the_user_disconnects(Duration::from_secs(30)).await;
    assert!(
        matches!(ended, Some(HowTheDrivingEnded::ThePeriodEnded(PeriodEnding::TheUserDisconnects))),
        "the period ends at the user's disconnect: {ended:?}"
    );
    said("END", format!("`Servers` supplied {values:?} over the whole period, which the user then ended"));
}
