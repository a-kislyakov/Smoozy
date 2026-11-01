//! **`Preparation/Scenarios/The machine is destroyed.md`, played at the desk**
//! — step 12's piece SC2.
//!
//! The scenario's situation and its eight steps, unchanged, through the
//! application's own acts, against the desk machine. At each step the test
//! prints what the user did, what Smoozy hands the page to show, and which
//! clause says so, and asserts all three. **A step the desk cannot play is said
//! in the output and never skipped quietly.**
//!
//! **What is real here.** OpenSSH's own server behind the harness's line; the
//! application's own transport, opened by `Servers`' own driving of a connected
//! period with the plan's own figures; the run's own register of periods and
//! its own publisher of what `Servers` supplies, over the desk's own record;
//! `Servers` Q1 step 7 and the live-reading sequence on every connection; the
//! desktop's own submit act; *Download all to…* as the harness plays it
//! (`a_whole_job_is_downloaded_to`); the Servers, Queue and History screens'
//! own compositions; the removal box from `Servers` W19's own table; and the
//! removal itself, `the_removal_is_performed_and_its_drawings_go`, the act
//! the window's *Remove* runs. **What is stood in for**: Blender, by the
//! harness's script; the operating system's notification service, by a
//! recorder; and the provider's destroy, by the harness's destroy act.
//!
//! **Where the application's own wiring needs the window, it is mirrored and
//! said**, exactly as in SC1 (`tests/the_connection_drops_while_rendering.rs`,
//! whose period, hand-over and passes this file copies). **One thing more is
//! mirrored here, and it is the same fact twice**: the run's register knows a
//! period only when the window began it (`a_period_has_begun` is private to the
//! desktop), so the server list's row and the removal box are each handed the
//! facts the register would hand them — `the_facts_behind` the value `Servers`
//! supplied, a period standing — and the period is ended at the removal's
//! forget the way `ThePeriodsThisRun::the_row_was_forgotten` ends it
//! (`Servers` S10 item 6).
//!
//! **What the desk cannot do.**
//!
//! - **A provider's own destroy.** Whether a destroyed machine's address
//!   refuses or answers nothing is only a real destroy's to show. This plays
//!   the refusal: the open connection is reset and every new one refused. The
//!   harness's other shape, silence for ever, is not played here; the rented
//!   pass plays the real one, last on each provider.
//! - **Steps 2 and 3's browser.** The user's own browser and the provider's
//!   website are nothing of Smoozy's; Smoozy is left open and untouched.
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
use smoozy_desktop::presentation::the_repairing_server::the_word_for_the_repair;
use smoozy_desktop::record::failure_records::{DurableSubject, FailureRecords};
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
use smoozy_desktop::surface::the_history::{what_is_kept, what_is_kept_over, AnEnding, TheHistoryOnScreen};
use smoozy_desktop::surface::the_queue::{what_is_held, what_the_servers_are_doing};
use smoozy_desktop::surface::servers::{
    the_removal_box_for, the_removal_is_performed_and_its_drawings_go, the_screen_from, TheAllServersScreen, TheAnswer,
};
use smoozy_scenarios::the_desk_machine::{a_whole_job_is_downloaded_to, HowTheMachineGoes, TheDesk, TheDeskMachine, TheDesksRecord};
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
    /// **Every attempt `Servers` made, when it began and whether it opened** —
    /// a recorder on this mirror, so the runs' attempts can be counted.
    attempts: TheAttempts,
}

/// When each attempt began, and whether it opened.
type TheAttempts = Arc<Mutex<Vec<(Instant, bool)>>>;

impl OpenConnections for TheTransportThroughThePoint {
    type Connection = Connection;

    async fn open(
        &mut self,
        endpoint: Endpoint,
        credential: Credential,
        decide: Box<dyn DecideHostTrust>,
        deadline: Duration,
    ) -> Result<Connection, OpenFailure> {
        let began = Instant::now();
        let opened = TheTransport.open(endpoint, credential, decide, deadline).await;
        self.attempts.lock().expect("the attempts").push((began, opened.is_ok()));
        opened
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
    attempts: TheAttempts,
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
        let attempts: TheAttempts = Arc::default();
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
                attempts: Arc::clone(&attempts),
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
            attempts,
            ending,
            driving: Some(driving),
        }
    }

    /// How many connections' step 7 has run, and what the last one did.
    fn the_step_sevens(&self) -> Vec<(u64, WhatStepSevenDid)> {
        self.each_connection.lock().expect("the connections").clone()
    }

    /// **The period ended from outside, at `ending`** (`Servers` S10), and the
    /// driving waited for no longer than `within`.
    async fn ends_at(mut self, ending: PeriodEnding, within: Duration) -> Option<HowTheDrivingEnded> {
        self.ending.at(ending);
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

/// One step's account line, printed for the output.
fn said(step: &str, what: impl AsRef<str>) {
    println!("SC2 {step}: {}", what.as_ref());
}

// ════════════════════════════════════════════════════════════════════════════
// The scenario
// ════════════════════════════════════════════════════════════════════════════

/// **The server list as `the_all_servers_screen` composes it**, over the
/// desk's record, with no keys asked of and the run's held work.
fn the_server_list(desk: &TheDesk, periods: &ThePeriodsThisRun) -> TheAllServersScreen {
    let held = desk.record().held();
    let rows = server_rows::all(held.connection()).expect("the rows read");
    let work = what_the_servers_are_doing(periods.the_machines_work(), Some(&*held));
    the_screen_from(&rows, &[], None, Some(periods), &work)
}

/// **History as `the_history_is_told_again` composes it**, with the servers
/// this run holds a connected reading for — the desktop's crate-private
/// `the_servers_connected`, mirrored: a period with a queue reading whose
/// break is not declared.
fn the_history(desk: &TheDesk, periods: &ThePeriodsThisRun, chosen: &str) -> TheHistoryOnScreen {
    let connected: std::collections::BTreeSet<String> = periods
        .the_machines_work()
        .every_server()
        .into_iter()
        .filter(|(_, period)| period.reading.is_some() && !period.the_break_is_declared)
        .map(|(server, _)| server)
        .collect();
    let held = desk.record().held();
    what_is_kept_over(Some(&*held), Some(chosen), &|server| connected.contains(server))
}

/// A screen's words, as the page reads them.
macro_rules! the_words {
    ($words:expr) => {
        serde_json::to_string($words).expect("the words render")
    };
}

/// **The user's own *Remove*, as `a_server_is_removed` runs it for a removal
/// that does not reach the machine**, and the period ended at the forget.
fn the_server_is_removed(
    desk: &TheDesk,
    queue: &TheOneQueue,
    notifications: &TheNotificationService,
    server: &str,
) -> TheAnswer {
    let started = desk.record().the_start().lock().expect("the start");
    the_removal_is_performed_and_its_drawings_go(&started, None, Some(queue), Some(notifications), server, false)
}

/// **The machine is destroyed**, its situation and its eight steps.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn the_machine_is_destroyed() {
    let desk = TheDesk::opened("sc2").expect("the desk's record opens");
    let Some(machine) = TheDeskMachine::stood_up("sc2", &desk, "RunPod 4090").await else {
        return;
    };
    let server_text = machine.the_row().local_server_identifier.clone();
    let server = Identity::read_back(IdentityKind::LocalServer, &server_text).expect("the row's own identity");

    // ── The situation ────────────────────────────────────────────────────
    let the_system = WhatTheSystemWasAsked::default();
    let notifications = Arc::new(TheNotificationService::on(Box::new(the_system.clone())));
    notifications.the_switches_are(Some(smoozy_desktop::record::settings::TheSwitches::nobody_has_turned_one()));
    let queue = Arc::new(TheOneQueue::new());
    let periods = Arc::new(ThePeriodsThisRun::over(
        TheRunThroughTheApp::lending_the_record_of(Arc::clone(desk.record().the_start())),
        Arc::clone(&queue),
        Arc::clone(&notifications),
    ));
    // **"Smoozy is connected to it. It is the only server in the list."**
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
    // **"One render job — an animation — was submitted a few hours ago and
    // has finished."** A short animation, by the desktop's own submit act,
    // rendered to its end while Smoozy watched.
    machine.the_stand_in().takes_this_long_a_frame(Duration::from_millis(50));
    let given = machine.what_the_page_hands_over(&desk, 1..=24, "Shot 40");
    assert!(!given.the_download_as_it_renders_choice, "auto-download was never switched on");
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
    // **"Submitted a few hours ago and has finished"**: the job is in History
    // and its record holds what the render's start recorded, its output
    // folder among it (`Queue & History` N9). One pass writes the ending
    // and then that reading, in two writes, so a run that downloaded the
    // moment History first said *finished* once found no folder yet (one
    // run in seven, 2026-10-24) — a moment hours before this situation's.
    until("History holds the job, finished, and its record holds the job's folder", Duration::from_secs(120), || {
        let held = desk.record().held();
        let finished = what_is_kept(Some(&*held), None)
            .rows
            .iter()
            .any(|row| row.job == job_text && row.ending == Some(AnEnding::Finished));
        finished
            && smoozy_desktop::record::job_records::JobRecords::over(held.connection())
                .read(&job)
                .ok()
                .flatten()
                .is_some_and(|record| record.the_job_output_folder(machine.layout()).is_some())
    })
    .await;
    // **"The user has already downloaded the outputs to their own machine and
    // checked them."** *Download all to…*, into a folder of their own.
    let to = desk.the_users_folder().to_path_buf();
    let downloaded = a_whole_job_is_downloaded_to(&desk, &connected.sessions, &job, &to)
        .await
        .expect("`Download` M1: the whole job's request is made and run");
    assert_eq!(downloaded.came_to.fetched, 24, "all 24 frames came down: {downloaded:?}");
    let pictures: Vec<_> = downloaded.landed.iter().filter(|path| path.extension().is_some_and(|ext| ext == "png")).cloned().collect();
    assert_eq!(pictures.len(), 24, "{:?}", downloaded.landed);
    let the_pictures_bytes: Vec<Vec<u8>> = pictures.iter().map(|path| std::fs::read(path).expect("it reads")).collect();
    said(
        "SITUATION (played)",
        format!(
            "connected on the confirmed branch; Shot 40, 24 frames, rendered to its end and in History as finished; \
             Download all to… the user's own folder brought all {} frames down",
            downloaded.came_to.fetched
        ),
    );

    // ── Step 1: the server list, the queue and the history ───────────────
    {
        let list = the_server_list(&desk, &periods);
        assert_eq!(list.rows.len(), 1, "one server in the list");
        assert_eq!(list.rows[0].name.as_str(), "RunPod 4090");
        assert_eq!(the_rows_presentation(&connected.supplied), Presentation::Connected);
        let held = desk.record().held();
        let screen = what_is_held(periods.the_machines_work(), Some(&*held), None);
        assert!(
            screen.lines.iter().all(|line| line.rows.is_empty()),
            "`Queue & History` C11: nothing queued, nothing running"
        );
        let history = what_is_kept(Some(&*held), Some(&job_text));
        let row = history.rows.iter().find(|row| row.job == job_text).expect("in History");
        assert_eq!(row.ending, Some(AnEnding::Finished));
        said(
            "STEP 1 (played)",
            format!(
                "the server list holds one server, {}, reading {:?} (its footer {}); the queue holds {} jobs; \
                 History reads Shot 40 {:?}, frames {:?}, on {}",
                list.rows[0].name.as_str(),
                the_rows_presentation(&connected.supplied),
                the_words!(&list.footer.how_many),
                screen.jobs,
                row.ending,
                row.frames,
                row.server,
            ),
        );
    }

    // ── Steps 2 and 3: the user destroys the machine at the provider ─────
    said(
        "STEP 2 (nothing of Smoozy's to play)",
        "the user leaves Smoozy open and goes to the provider's website in their own browser; Smoozy is not touched",
    );
    let destroyed_at = Instant::now();
    machine.is_destroyed(HowTheMachineGoes::WithARefusal).await;
    said(
        "STEP 3 (played, by the harness)",
        "the machine stops existing: every program on it is ended, the open connection is reset and every new \
         connection is refused. A provider's own destroy may look like silence instead; the rented pass shows which",
    );

    // ── Step 4: the user looks at the server list ────────────────────────
    {
        // **`Servers` K2: the reset is read as the connection's end at once**,
        // and `Servers` N1's repair starts with the user doing nothing.
        until("the connection's end is read and the repair begins", Duration::from_secs(30), || {
            connected.supplied.left_connected_after(destroyed_at).is_some()
        })
        .await;
        let declared = connected
            .supplied
            .left_connected_after(destroyed_at)
            .map(|at| at.saturating_duration_since(destroyed_at))
            .expect("the moment");
        assert!(declared < Duration::from_secs(10), "`Servers` K2: a reset is read at once, not after `Servers` K3's window: {declared:?}");
        assert_eq!(the_rows_presentation(&connected.supplied), Presentation::Repairing);
        let list = the_server_list(&desk, &periods);
        assert_eq!(list.rows.len(), 1, "the server stays in the list");
        assert!(connected.supplied.raised.lock().expect("the raises").is_empty(), "`Servers` N5: nothing raised yet");
        assert!(the_records_against(&desk, &server).is_empty());
        said(
            "STEP 4 (played)",
            format!(
                "the reset was read {declared:?} after the machine went; the list still holds {} reading {:?}, its \
                 work column {:?}; no record and no notification yet",
                list.rows[0].name.as_str(),
                the_rows_presentation(&connected.supplied),
                the_word_for_the_repair(true).map(|word| word.the_word()),
            ),
        );
    }

    // ── Step 5: the user keeps watching it ───────────────────────────────
    {
        // **`Servers` N2a and `Servers` N5: the first run of attempts, every one refused
        // at once, fails at its 105 seconds and raises the condition**;
        // `Desktop app and delivery` T13 tells the system from that record.
        until("`Servers` N5: the first run fails and the condition is raised", Duration::from_secs(150), || {
            !connected.supplied.raised.lock().expect("the raises").is_empty()
        })
        .await;
        let raised_at = destroyed_at.elapsed();
        // **`Servers` N2a and `Servers` N5's figure: the record lands a determinate 105
        // seconds after the break's declaration, and with every attempt
        // refused at once the run is eight attempts** — checked before
        // anything else here, because it is the step's own clause.
        {
            let declared = connected.supplied.left_connected_after(destroyed_at).expect("the declaration");
            let raised = connected.supplied.raised.lock().expect("the raises")[0];
            let after_the_declaration = raised.saturating_duration_since(declared);
            let in_the_run: Vec<Duration> = connected
                .attempts
                .lock()
                .expect("the attempts")
                .iter()
                .filter(|(began, _)| *began >= declared && *began <= raised)
                .map(|(began, _)| began.saturating_duration_since(declared))
                .collect();
            let as_the_clause_says = in_the_run.len() as u32 == smoozy_servers::repair::ATTEMPTS_IN_A_RUN
                && after_the_declaration >= Duration::from_secs(104)
                && after_the_declaration <= Duration::from_secs(108);
            if !as_the_clause_says {
                said(
                    "STEP 5 (FAILED — a finding)",
                    format!(
                        "the first run of attempts, every one refused at once, was {} attempts, begun at {in_the_run:?} \
                         after the break's declaration, and the condition was raised {after_the_declaration:?} after \
                         it. `Servers` N2a: eight attempts fill the 105-second span exactly; `Servers` N5: the record \
                         is raised at a determinate 105 seconds after the declaration. The scenario stops here",
                        in_the_run.len(),
                    ),
                );
            }
            assert!(
                as_the_clause_says,
                "`Servers` N2a and `Servers` N5: a run of {} refused attempts raised its record {after_the_declaration:?} after \
                 the declaration, where the clauses give eight attempts and 105 seconds: {in_the_run:?}",
                in_the_run.len()
            );
        }
        assert_eq!(the_records_against(&desk, &server).len(), 1, "`Servers` N5: one condition record");
        assert!(
            the_system.every().iter().any(|post| post.contains(&format!("{:?}", Kind::AConnectionCouldNotBeRestored))),
            "`Desktop app and delivery` T13: the could-not-restore notification is handed to the system: {:?}",
            the_system.every()
        );
        let due = periods.the_next_attempt_due_for(&server_text);
        assert!(matches!(due, NextAttemptDue::At(_)), "`Servers` N10a: the next attempt has its moment: {due:?}");
        // **`Servers` N8: nothing gives up.** The standing pause ends and the
        // next run begins; the server is still Smoozy's, still repairing.
        let values_before = connected.supplied.every_value().len();
        until("`Servers` N6: the next run begins after the standing pause", Duration::from_secs(90), || {
            connected.supplied.every_value()[values_before..].contains(&ObservedValue::Reconnecting)
        })
        .await;
        let next_run_at = destroyed_at.elapsed();
        let the_attempts: Vec<Duration> = connected
            .attempts
            .lock()
            .expect("the attempts")
            .iter()
            .filter(|(began, _)| *began >= destroyed_at)
            .map(|(began, _)| began.saturating_duration_since(destroyed_at))
            .collect();
        assert_eq!(the_rows_presentation(&connected.supplied), Presentation::Repairing);
        let list = the_server_list(&desk, &periods);
        assert_eq!(list.rows.len(), 1, "Smoozy never decides on its own that a machine is gone (`Specs/Whole app`)");
        said(
            "STEP 5 (played)",
            format!(
                "the first run of attempts failed {raised_at:?} after the machine went: one condition record stands \
                 and the system was asked {:?}; the next attempt was due {due:?}, and the next run began \
                 {next_run_at:?} after the machine went; the attempts began at {the_attempts:?}. The server stays in the list, reading {:?}: nothing gives \
                 up, and Smoozy does not decide the machine is gone",
                the_system.every(),
                the_rows_presentation(&connected.supplied),
            ),
        );
    }

    // ── Step 6: the user opens History to look at the finished job ───────
    {
        let history = the_history(&desk, &periods, &job_text);
        let row = history.rows.iter().find(|row| row.job == job_text).expect("still in History");
        assert_eq!(row.ending, Some(AnEnding::Finished), "`Queue & History`: History survives the machine");
        assert_eq!(row.server, "RunPod 4090");
        let entry = history.entry.as_ref().expect("its entry");
        assert!(
            entry.entry.the_server_is_not_connected,
            "the server stands and is not connected, so *Files on the server* is not drawn (`Design/File Browser`)"
        );
        said(
            "STEP 6 (played)",
            format!(
                "History reads Shot 40 {:?}, frames {:?}, on {}, how long {:?}; its entry offers the machine's \
                 controls: {}, the server not connected: {}; its ways to render again {:?}",
                row.ending,
                row.frames,
                row.server,
                row.how_long,
                entry.entry.reach_backs,
                entry.entry.the_server_is_not_connected,
                entry.again,
            ),
        );
    }

    // ── Step 7: the user removes "RunPod 4090" from Smoozy ───────────────
    let removal = {
        let of = the_facts_behind(connected.supplied.now().unwrap_or(ObservedValue::Disconnected), true, false);
        let the_box = {
            let started = desk.record().the_start().lock().expect("the start");
            the_removal_box_for(&started, of, Some(periods.the_machines_work()), &server_text).expect("the box is composed")
        };
        // **`Servers` W19: nothing queued and nothing running is the whole of
        // a removal** — a plain confirmation, no jobs warning, nothing offered
        // to stop or clear (`Specs/Servers`, *Where the machine has nothing on
        // it*; `Design/Servers § Removing a server`).
        assert_eq!(the_box.offers.as_str(), "a-plain-confirmation", "{the_box:?}");
        assert!(the_box.the_warning.is_none() && the_box.stop_and_clear.is_none() && the_box.leave_it_running.is_none(), "{the_box:?}");
        assert!(the_box.remove.is_some(), "the plain confirmation's own Remove");
        let answer = the_server_is_removed(&desk, &queue, &notifications, &server_text);
        assert_eq!(answer, TheAnswer::ItWasSaved, "the removal landed");
        said(
            "STEP 7 (played)",
            format!(
                "the removal box: {} — {}; it offers {}, with {} and {}, and no warning; Remove pressed, and the \
                 removal landed",
                the_words!(&the_box.heading),
                the_box.said.as_str(),
                the_box.offers.as_str(),
                the_words!(&the_box.cancel),
                the_words!(&the_box.remove),
            ),
        );
        the_box
    };

    // ── Step 8: the server list, the queue and the history ───────────────
    {
        // **`Servers` S10 item 6: the period ends at the removal's local
        // forget** — what `the_row_was_forgotten` does for a period the
        // window began.
        let ended = connected.ends_at(PeriodEnding::TheRowIsRemovedAtItsLocalForget, Duration::from_secs(60)).await;
        assert!(
            matches!(ended, Some(HowTheDrivingEnded::ThePeriodEnded(PeriodEnding::TheRowIsRemovedAtItsLocalForget))),
            "the period ends at the forget: {ended:?}"
        );
        let list = the_server_list(&desk, &periods);
        assert!(list.rows.is_empty(), "the list is empty");
        let held = desk.record().held();
        let screen = what_is_held(periods.the_machines_work(), Some(&*held), None);
        assert!(screen.lines.iter().all(|line| line.rows.is_empty()), "the queue holds nothing");
        drop(held);
        let history = the_history(&desk, &periods, &job_text);
        let row = history.rows.iter().find(|row| row.job == job_text).expect("`Queue & History`: History survives the removal");
        assert_eq!(row.ending, Some(AnEnding::Finished));
        assert_eq!(row.server, "RunPod 4090", "`Queue & History`: the entry keeps the server's name as it stood");
        let entry = history.entry.as_ref().expect("its entry");
        assert!(
            !entry.entry.reach_backs,
            "`Specs/Queue & History`: an entry outliving its server keeps its setup but loses its reach"
        );
        assert!(the_records_against(&desk, &server).is_empty(), "the condition record went with the row");
        // **`Desktop app and delivery` T24b: the record's ending at its
        // subject's removal takes its post down**, on the identity it was
        // posted on — the could-not-restore notification does not stay in
        // the system's list for a server Smoozy no longer has.
        let asked = the_system.every();
        let posted_at = asked
            .iter()
            .position(|said| said.contains(&format!("{:?}", Kind::AConnectionCouldNotBeRestored)))
            .expect("the could-not-restore notification was posted");
        assert!(
            asked[posted_at..].iter().any(|said| said.starts_with("taken down ACondition(")),
            "`Desktop app and delivery` T24b: the could-not-restore notification is taken down with its server: {asked:?}"
        );
        let still: Vec<Vec<u8>> = pictures.iter().map(|path| std::fs::read(path).expect("still there")).collect();
        assert_eq!(still, the_pictures_bytes, "what the user downloaded is untouched");
        said(
            "STEP 8 (played)",
            format!(
                "the server list holds no server, its footer {} and its guidance {:?}; the queue holds {} jobs; History \
                 still reads Shot 40 {:?}, frames {:?}, on {}, its entry offering the machine's controls: {}, the \
                 server not connected: {}, its ways to render again {:?}; the condition record is gone; the 24 \
                 downloaded frames are untouched; the system was asked, over the whole run, {:?}. (The box was {})",
                the_words!(&list.footer),
                list.guidance.heading.as_str(),
                screen.jobs,
                row.ending,
                row.frames,
                row.server,
                entry.entry.reach_backs,
                entry.entry.the_server_is_not_connected,
                entry.again,
                the_system.every(),
                removal.offers.as_str(),
            ),
        );
    }
}
