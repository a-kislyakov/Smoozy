//! **The connected periods this run holds, and the one thread each of them
//! runs on.**
//!
//! **Nothing here decides anything `Servers` decides.** The period, its six
//! endings, the repair, its two figures, the restart policy and the whole
//! first-connection sequence are `App/Servers/`'s — built and tested there.
//! What is here is **where a period runs, what starts and ends one, and what
//! the screen reads back off it**.
//!
//! # Where a period runs, and it is the decision this file takes for everything after it
//!
//! **`Servers`' driving half is asynchronous throughout** and its own wiring
//! waits on `tokio`'s clock and opens sockets on `tokio`'s reactor, while this
//! crate's habit everywhere else is `std::thread::spawn` and blocking calls.
//! **P5 binds either way: the period does not run on the drawing thread.**
//!
//! **The route taken is one thread per standing period, blocking on the
//! shell's own runtime** — `std::thread::spawn`, and
//! `tauri::async_runtime::block_on` inside it. **Three things decided it.**
//!
//! - **It adds no manifest line and moves no lock.** `tauri::async_runtime` is
//!   already this crate's — `src/surface/servers.rs` spawns blocking work on it
//!   — it is the shell's own runtime, and it exists on all three systems. The
//!   alternative was naming `tokio` as a plain dependency of this crate, which
//!   would have been **a second runtime beside the shell's**, with
//!   `src/notifications/linux.rs` already holding a standing third on Linux.
//! - **Nothing a period carries is `Send`, and nothing needs to be.**
//!   `tauri::async_runtime::spawn` wants a `Send + 'static` future; the acting
//!   half holds `&dyn LendsTheRecord` and `&dyn SecretStore`, neither of which
//!   is. **Making them `Send` would have meant editing `Keys`' own adapter to
//!   suit this file**, which is the shape of change this stage does not make.
//!   `block_on` asks for neither.
//! - **A period is a thread's whole life and not a task among many.** It runs
//!   for as long as the user leaves the server connected, and the standing
//!   repair runs for as long as the application is open. One parked thread per
//!   connected server is what that actually is.
//!
//! **What a reader must not conclude from it.** This is not a second runtime
//! and not a second timer: every wait inside a period is one of the repair's
//! own two figures, inside a sequence that is already running, which is
//! §3.3 P10a's first admitted shape.
//!
//! # What the screen reads back
//!
//! **The run holds one observed value per server and it is
//! `src/servers/published.rs`'s**, written there as `Servers` publishes it.
//! This file adds the one fact that value cannot carry — **whether a period
//! stands** — and [`the_facts_behind`] puts the two back together into the
//! shape `Servers`' own `condition` module takes. **Nothing here holds a second
//! copy of the observed value**, which is the fault this build stage keeps
//! paying for.
//!
//! # What a period lends, and where from
//!
//! **A period's live connection is lent from its own thread and from nowhere
//! else** — `crate::servers::sessions`, the per-server point `Server protocol
//! and transfers` T5 puts every session through. **`Servers`' driving holds the
//! connection and, while it is live, only waits for it to end**; the opener this
//! file hands that driving, [`TheTransportThroughThePoint`], makes that wait
//! the point's lending, so the connection stays where `Servers` keeps it and
//! its ending still wins. The register below keeps each period's point, and
//! [`ThePeriodsThisRun::the_sessions_of`] is how the rest of the application
//! reaches it. **`Servers` S14's additional connection is opened from here on
//! the point's asking**, by that crate's own sequence with the additional role,
//! so `Servers` Q16's three steps are that crate's and not this file's.
//!
//! **The moment the lending begins is also `Servers` Q1 step 7's.** The
//! hand-over `Servers` leaves to whoever drives its sequence is made there, to
//! `crate::servers::the_machines_work`, which runs step 7 and then P10d on a
//! thread of its own and stops with the connection it was started for.
//!
//! # How a clause marker is written in this crate, and it is a rule rather than
//! a habit
//!
//! A marker with no document beside it means a clause of this crate's own plan,
//! `Preparation/Architecture/Desktop app and delivery.md`. Every other
//! document's marker is written with that document — at the site, or in this
//! file's own header list below. **Where a marker is a clause of this crate's
//! own plan *and* of the document meant, the header list is not enough and the
//! document is written out at the site**, because a bare one there resolves to
//! a real and wrong clause.
//!
//! **Every marker below that belongs to another document is written out with
//! that document at its own site, and this file keeps no header list.** The
//! list cannot tell a marker both plans define from one only the other does,
//! and this file cites a document that collides with this plan on most of its
//! letters.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use tauri::{AppHandle, Manager, Runtime};

use smoozy_download::acting::fetching::TheFetchQueue;
use smoozy_download::acting::probing::TheProbesCadence;
use smoozy_download::acting::record_directory::{
    read_the_attempts_order, read_the_narrowing_material,
    read_the_recorded_reading_the_machine_wrote, read_what_smoozy_placed,
    what_the_narrowing_leaves, ReadsOneFileOnTheMachine, WhatSmoozyPlaced,
    WhatTheNarrowingLeaves, WhatTheRecordDirectoryHeld,
};
use smoozy_download::acting::running::{TheArrangementsRun, WhatIsInHand, WhatTheTurnDid};
use smoozy_download::acting::wiring::TheReadingRoute;
use smoozy_download::arrangement::{AnArrangement, DesiredState};
use smoozy_download::narrowing::TheNarrowingMaterial;
use smoozy_download::vouching::TheOrder;
use smoozy_download::watch::{the_pause_before_the_next_walk, TheWindow};
use smoozy_job_files::layout::TheLayout;
use smoozy_job_files::publication::AJobsPaths;
use smoozy_server_protocol::subsystem::SubsystemFailure;
use smoozy_server_protocol::transport::{ssh, Credential, DecideHostTrust, Ending, Endpoint, OpenFailure};
use smoozy_servers::acting::{OpenConnections, TheActingHalf, WhatServersPublishes};
use smoozy_servers::condition::{ObservedValue, WhatIsTrueOfTheServer};
use smoozy_security_and_licence::plaintext::Origin;
use smoozy_servers::driving::{APeriod, EndThePeriod, HowTheDrivingEnded, TheDesktopsSide};
use smoozy_servers::period::{OpeningAct, PeriodEnding, WhoTookIt};
use smoozy_servers::repair::{NextAttemptDue, WhatTheRecordSays};
use smoozy_servers::first_connection::ChangedIdentityWarning;
use smoozy_servers::key_moments::WhoStartedIt;
use smoozy_servers::sequence::{self, AnAttempt, Attempt, ConnectionRole, WhereItStopped};
use smoozy_servers::store::{RememberedHostKey, ServerRow};
use smoozy_servers::wiring::{TheDesktopsClock, TheTransport};
use smoozy_state::identities::{Identity, IdentityKind};

use crate::keys::askpass::TheRoute;
use crate::keys::for_a_connection::{TheCredentialForAConnection, WhyNothingWasSupplied};
use crate::notifications::TheNotificationService;
use crate::platform::locations;
use crate::record::download_records::DownloadArrangements;
use crate::record::job_records::JobRecords;
use crate::record::server_rows;
use crate::servers::installation::{OverTheConnection, TheMachinesInstallationIdentity};
use crate::servers::published::{LendsWhatTheRunHolds, WhatTheDesktopPublishes, WhatTheRunLends};
use crate::servers::sessions::{
    self, AHeldConnection, ItsOwnFuture, TheAdditionalConnection, ThisServersSessions,
};
use crate::start::Started;

// ----------------------------------------------------- reaching the run

/// **The run, reached through the shell's own handle rather than borrowed.**
///
/// `src/servers/published.rs`'s `TheRunItself` borrows the application's one
/// lock for its own lifetime, which a thread that outlives the command that
/// started it cannot hold. **This reaches the same lock the same way** — taken
/// inside the lend and dropped before it returns — and holds an `AppHandle`,
/// which the shell makes cheap to clone and safe to keep.
///
/// **The runtime is erased on purpose.** The register below is one value the
/// whole application shares, and a register carrying the shell's runtime type
/// would have to be named with it at every call site.
pub struct TheRunThroughTheApp {
    reach: Box<TheLend>,
}

/// One lend, as the seam's own method takes it.
type TheLend = dyn Fn(&mut dyn FnMut(WhatTheRunLends<'_>)) + Send + Sync;

impl TheRunThroughTheApp {
    /// **A run that lends nothing**, which is a record that would not open —
    /// and is what a test of the register itself holds, there being no shell
    /// behind it.
    pub fn lending_nothing() -> TheRunThroughTheApp {
        TheRunThroughTheApp {
            reach: Box::new(|_act| {}),
        }
    }

    /// **A run lending the record a start holds** — what a test of a screen
    /// that reads the record through the register holds, there being no shell
    /// behind it. Added 2026-09-27, step 10c's eighth-pass fix piece 6.
    pub fn lending_the_record_of(started: Arc<Mutex<Started>>) -> TheRunThroughTheApp {
        TheRunThroughTheApp {
            reach: Box::new(move |act| {
                let Ok(started) = started.lock() else {
                    return;
                };
                let Some(record) = started.record.as_ref() else {
                    return;
                };
                act(WhatTheRunLends {
                    record: record.connection(),
                    said_once: &started.said_once,
                    declarations: &started.what_modules_declare,
                });
            }),
        }
    }

    pub fn of<R: Runtime>(app: AppHandle<R>) -> TheRunThroughTheApp {
        TheRunThroughTheApp {
            reach: Box::new(move |act| {
                let Some(started) = app.try_state::<Mutex<Started>>() else {
                    return;
                };
                let Ok(started) = started.lock() else {
                    return;
                };
                let Some(record) = started.record.as_ref() else {
                    return;
                };
                act(WhatTheRunLends {
                    record: record.connection(),
                    said_once: &started.said_once,
                    declarations: &started.what_modules_declare,
                });
            }),
        }
    }
}

impl LendsWhatTheRunHolds for TheRunThroughTheApp {
    fn lend(&self, act: &mut dyn FnMut(WhatTheRunLends<'_>)) {
        (self.reach)(act)
    }
}

/// **The one publisher of the run, reached from whichever period is
/// publishing.**
///
/// `src/servers/published.rs` holds the run's readings for **every** server in
/// one table and says why; a publisher per period would be that table split in
/// as many pieces as there are connected servers. **So there is one, and this
/// is what a period's acting half carries in its place.**
///
/// **No lock is taken around it.** The publisher keeps what it holds about each
/// server behind its own short lock, taken for one change and given back before
/// it lends the run — so a publish never holds a lock of this crate's while it
/// waits for the application's. *(Repaired 2026-09-18, step 10b's first audit
/// round, item 2: this wrapper used to lock the whole publisher around every
/// publish, and a publish that lent the run then waited for the application's
/// lock while holding it — the opposite order to the two screen commands that
/// hold the application's lock and read the publisher.)*
#[derive(Clone)]
pub struct WhereThisPeriodPublishes(Arc<WhatTheDesktopPublishes<TheRunThroughTheApp>>);

impl WhatServersPublishes for WhereThisPeriodPublishes {
    fn the_observed_value(&mut self, server: &str, value: ObservedValue) {
        (&*self.0).the_observed_value(server, value);
    }

    fn the_next_attempt_due(&mut self, server: &str, reading: NextAttemptDue) {
        (&*self.0).the_next_attempt_due(server, reading);
    }

    fn raise_the_connection_condition(&mut self, server: &str, says: WhatTheRecordSays) {
        (&*self.0).raise_the_connection_condition(server, says);
    }

    fn withdraw_the_connection_condition(&mut self, server: &str) {
        (&*self.0).withdraw_the_connection_condition(server);
    }

    fn the_changed_identity_warning(&mut self, server: &str) {
        (&*self.0).the_changed_identity_warning(server);
    }

    fn record_the_scanned_host_key(&mut self, server: &str, key: RememberedHostKey) {
        (&*self.0).record_the_scanned_host_key(server, key);
    }
}

// ------------------------------------------------------------- the register

/// What the run holds about one standing period.
///
/// **`Servers` S4 fixes what a period holds and this is not that.** The period
/// itself is the driving thread's; what is here is the handles the rest of the
/// application needs — the way to end it, and where an edit's save says whether
/// it landed.
struct AStandingPeriod {
    /// **`Servers` S10 items 1, 2, 3 and 6 arrive on this**, from outside the
    /// thread that drives the period.
    end_it: EndThePeriod,
    /// **`Servers` E1 and E2: the edit's save ended this period, and whether
    /// the save landed is told here.** The period's own thread reads it once
    /// the period has left this register, and opens the put-back only on a yes
    /// — `Servers` §14's *Edit* row: *where the save fails the put-back is not
    /// attempted*.
    the_edits_save: Option<std::sync::mpsc::Receiver<bool>>,
    /// **`Server protocol and transfers` T5's point for this server**, made
    /// with the period and gone with it. It lends only while a connection under
    /// the period is live.
    sessions: ThisServersSessions,
    /// **`Servers` S4's *restarting, caused by Smoozy* mark on this period**,
    /// as the handle the period's own driving reads live.
    ///
    /// **It is held here so that `Server Setup` T4's setter can reach it** —
    /// the mark is set from the thread driving a setup run, before the restart
    /// command leaves, which is nowhere near the thread the period is driven
    /// on. **One copy of the fact**: this is `Servers`' own handle on the
    /// standing period and not a second bool of this crate's.
    the_restart_mark: smoozy_servers::restart::TheRestartMark,
}

/// **`Servers` E2's put-back, owed by an edit's save that ended a standing
/// period.** The command that saved says whether the save landed; a command
/// that ends without saying — dropping this — is a save that did not land.
pub struct ThePutBackIsOwed(std::sync::mpsc::Sender<bool>);

impl ThePutBackIsOwed {
    pub fn the_save_landed(self, landed: bool) {
        let _ = self.0.send(landed);
    }
}

/// **What the last primary connection to a server read, for the life of the
/// run** — `Servers` Q1 steps 5 and 6, handed back by the driving.
///
/// **Run-held and nothing else** (`Servers` S3). It exists so the list can take
/// `Servers` H1's state from `State, identity and recovery` Y9's branch rather
/// than from `Servers` H1's own table alone, which is what `Servers` H18 asks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhatTheLastConnectionRead {
    /// The branch step 5 settled on.
    pub check: smoozy_state::identities::InstallationCheck,
    /// **Whether step 6's fresh reading was written to the row** — `Servers`
    /// H4: it replaced the held one whole.
    pub a_fresh_reading_was_kept: bool,
    /// **`Servers` H1a: whether the connection that read it is the live one.**
    /// It stops being so the moment another attempt begins.
    pub on_the_connection_that_is_live: bool,
}

/// **Every connected period this application run holds.**
///
/// **`Servers` S2: a period is per server, and one server's period touches no
/// other's.** Nothing here holds a count of periods, a global connection state
/// or a shared connection — it is a table keyed by the local server
/// identifier. **`Servers` S3: it is process memory for the life of the run and
/// is written nowhere durable.**
pub struct ThePeriodsThisRun {
    publishes: Arc<WhatTheDesktopPublishes<TheRunThroughTheApp>>,
    standing: Mutex<HashMap<String, AStandingPeriod>>,
    what_connections_read: Mutex<HashMap<String, WhatTheLastConnectionRead>>,
    /// **`Security and licence` S34's sentence, owed on a server's row** —
    /// where the moment it is said at came inside a repair Smoozy was making on
    /// its own and no box was in front of the user (`Preparation/Design/Keys.md`,
    /// *On a computer with nowhere to keep a passphrase*). Run-held: the row
    /// carries it for as long as the application stays open.
    the_sentence_is_owed_on: Mutex<std::collections::HashSet<String>>,
    /// **What each server's connections have read of its machine's work** —
    /// `Servers` Q1 step 7's queue reading, period-held (`Queue & History` C6).
    /// The catch-up account is no longer held here: since the 38's piece 15 it
    /// is the local record's (`Queue & History` C2, record version 38). See
    /// `crate::servers::the_machines_work`.
    the_machines_work: crate::servers::the_machines_work::WhatThisRunHolds,
}

impl ThePeriodsThisRun {
    /// **Made once, at the application's start**, from the same queue and the
    /// same notification service every other module was handed.
    pub fn for_this_run<R: Runtime>(
        app: AppHandle<R>,
        queue: Arc<crate::events::TheOneQueue>,
        notifications: Arc<TheNotificationService>,
    ) -> ThePeriodsThisRun {
        ThePeriodsThisRun::over(TheRunThroughTheApp::of(app), queue, notifications)
    }

    /// The same register over a run handed in, **so the whole of it can be
    /// tested** — the shell's own managed state has no constructor outside a
    /// running application, which is the split every other file of this crate
    /// already uses.
    pub fn over(
        the_run: TheRunThroughTheApp,
        queue: Arc<crate::events::TheOneQueue>,
        notifications: Arc<TheNotificationService>,
    ) -> ThePeriodsThisRun {
        ThePeriodsThisRun {
            publishes: Arc::new(WhatTheDesktopPublishes::new(
                the_run,
                queue,
                notifications,
            )),
            standing: Mutex::new(HashMap::new()),
            what_connections_read: Mutex::new(HashMap::new()),
            the_sentence_is_owed_on: Mutex::new(std::collections::HashSet::new()),
            the_machines_work: Default::default(),
        }
    }

    /// **What this run holds of each server's work** — step 10c piece 7.
    pub fn the_machines_work(&self) -> &crate::servers::the_machines_work::WhatThisRunHolds {
        &self.the_machines_work
    }

    /// What a period's acting half publishes through.
    pub fn where_a_period_publishes(&self) -> WhereThisPeriodPublishes {
        WhereThisPeriodPublishes(Arc::clone(&self.publishes))
    }

    /// **The run's one publisher**, for the fourth-branch condition, which is
    /// raised through the same record and the same queue as `Servers`' own.
    pub(crate) fn the_publisher(&self) -> &WhatTheDesktopPublishes<TheRunThroughTheApp> {
        &self.publishes
    }

    /// **`Servers` S4's restart mark for this server, where a period stands
    /// for it** — `Server Setup` T4's *before the command leaves* is set on
    /// this, and `Servers` M1's two endings are what clear it.
    pub fn the_restart_mark_of(
        &self,
        server: &str,
    ) -> Option<smoozy_servers::restart::TheRestartMark> {
        self.standing
            .lock()
            .ok()
            .and_then(|standing| standing.get(server).map(|it| it.the_restart_mark.clone()))
    }

    /// **`Servers` K8a's one boolean: whether a connected period stands.**
    pub fn a_period_stands_for(&self, server: &str) -> bool {
        self.standing
            .lock()
            .map(|standing| standing.contains_key(server))
            .unwrap_or(false)
    }

    /// **What is true of one server, as `Servers`' own `condition` module takes
    /// it.**
    pub fn what_is_true_of(&self, server: &str) -> WhatIsTrueOfTheServer {
        let observed = self
            .publishes
            .what_is_held_about(server)
            .map(|(value, _)| value)
            .unwrap_or(ObservedValue::Disconnected);
        the_facts_behind(
            observed,
            self.a_period_stands_for(server),
            self.the_restart_mark_of(server)
                .is_some_and(|mark| mark.stands()),
        )
    }

    /// **Whether Smoozy's service on this server does not answer** — `State,
    /// identity and recovery` §8.3's supervisor-unavailable condition standing
    /// against it, read from the record (`Desktop app and delivery` V1). Added
    /// 2026-09-27, step 10c's eighth-pass fix piece 6.
    pub fn smoozy_does_not_answer_on(&self, server: &str) -> bool {
        self.publishes.the_supervisors_condition_stands(server)
    }

    /// **`Servers` N10a's moment, where the run holds one for this server.**
    pub fn the_next_attempt_due_for(&self, server: &str) -> NextAttemptDue {
        self.publishes
            .what_is_held_about(server)
            .map(|(_, due)| due)
            .unwrap_or(NextAttemptDue::Absent)
    }

    /// **What the last primary connection to this server read, where one has
    /// in this run.**
    pub fn what_the_last_connection_read(&self, server: &str) -> Option<WhatTheLastConnectionRead> {
        self.what_connections_read
            .lock()
            .ok()
            .and_then(|read| read.get(server).copied())
    }

    /// A live primary connection's branch and whether its reading was kept.
    pub(crate) fn a_connection_read(&self, server: &str, read: WhatTheLastConnectionRead) {
        if let Ok(mut held) = self.what_connections_read.lock() {
            held.insert(server.to_string(), read);
        }
    }

    /// **`Servers` H1a: another attempt has begun**, so whatever the last
    /// connection read is no longer the live connection's own.
    pub(crate) fn another_attempt_begins(&self, server: &str) {
        if let Ok(mut held) = self.what_connections_read.lock() {
            if let Some(read) = held.get_mut(server) {
                read.on_the_connection_that_is_live = false;
            }
        }
    }

    /// **The sentence is owed on this server's row** for the rest of the run.
    pub(crate) fn the_sentence_is_owed_on(&self, server: &str) {
        if let Ok(mut owed) = self.the_sentence_is_owed_on.lock() {
            owed.insert(server.to_string());
        }
    }

    /// Whether this server's row carries the sentence.
    pub fn the_row_carries_the_sentence(&self, server: &str) -> bool {
        self.the_sentence_is_owed_on
            .lock()
            .map(|owed| owed.contains(server))
            .unwrap_or(false)
    }

    /// **`Servers` S5: a period begins at the act**, and this is the register's
    /// half of that — the handle the six endings arrive on, put down before the
    /// first attempt is made.
    ///
    /// **A server that already has a standing period gets no second one**
    /// (`Servers` S2 is per server, and S5's list of period-opening acts is
    /// closed): the answer is that nothing was begun.
    pub(crate) fn a_period_has_begun(&self, server: &str, period: &APeriod) -> bool {
        let Ok(mut standing) = self.standing.lock() else {
            return false;
        };
        if standing.contains_key(server) {
            return false;
        }
        standing.insert(
            server.to_string(),
            AStandingPeriod {
                end_it: period.end_it(),
                the_edits_save: None,
                sessions: ThisServersSessions::for_one_server(),
                // **`Servers` S4's mark on this very period**, taken as a
                // handle rather than copied as a value.
                the_restart_mark: period.the_restart_mark(),
            },
        );
        true
    }

    /// **This server's point — `Server protocol and transfers` T5 — where a
    /// period stands for it.** Every session on its connections is asked for
    /// through what this answers: the reading session, one command at a time,
    /// a transfer's channel, and the supervisor asked something. **It lends
    /// only while the period's connection is live**, and says so otherwise.
    pub fn the_sessions_of(&self, server: &str) -> Option<ThisServersSessions> {
        self.standing
            .lock()
            .ok()
            .and_then(|standing| standing.get(server).map(|period| period.sessions.clone()))
    }

    /// **`Servers` S11: every period-held thing ends the moment the period
    /// ends**, and the register's entry is one of them. **What an edit's save
    /// left for the period's own thread comes out with it.**
    pub(crate) fn a_period_has_ended(
        &self,
        server: &str,
    ) -> Option<std::sync::mpsc::Receiver<bool>> {
        // **`Queue & History` C6: the last-known queue reading is period-held,
        // and the period ending drops it entirely.**
        self.the_machines_work.the_period_ended(server);
        // **`File Browser` L10 and L16: the `.blend` version tags are a live
        // reading and end with the period.** Step 10c's twelfth-pass fix
        // piece 6b, which first gave the application a reader for them.
        crate::surface::files::the_tags_period_ended(server);
        let Ok(mut standing) = self.standing.lock() else {
            return None;
        };
        standing
            .remove(server)
            .and_then(|period| period.the_edits_save)
    }

    /// **`Servers` S10 item 1: the user disconnects.** Available for as long as
    /// a period stands — including while the server reads as reconnecting, and
    /// including while it reads as disconnected between runs — and **taking it
    /// stops the repair**.
    pub fn the_user_disconnects(&self, server: &str) -> bool {
        self.it_ends_at(server, PeriodEnding::TheUserDisconnects)
    }

    /// **`Servers` S10 item 3: the edit's save on a server whose period
    /// stands**, and **`Servers` E1 is why it answers with the put-back owed**:
    /// saving a **connected** server's details is what is followed by the
    /// put-back, so where no period stands nothing is owed and nothing is
    /// opened. The put-back is opened by the old period's own thread, after it
    /// has left the register, so the order is kept rather than raced.
    pub fn the_edits_save_ends_it(&self, server: &str) -> Option<ThePutBackIsOwed> {
        let Ok(mut standing) = self.standing.lock() else {
            return None;
        };
        let period = standing.get_mut(server)?;
        let (tell, told) = std::sync::mpsc::channel();
        period.the_edits_save = Some(told);
        period.end_it.at(PeriodEnding::TheEditsSave);
        Some(ThePutBackIsOwed(tell))
    }

    /// **`Servers` S10 item 6: the row is removed**, at the moment the
    /// removal's local forget is performed and not at the moment it is asked
    /// for.
    pub fn the_row_was_forgotten(&self, server: &str) -> bool {
        self.it_ends_at(server, PeriodEnding::TheRowIsRemovedAtItsLocalForget)
    }

    /// **`Servers` S10 item 2: the application run ends, however it ends.**
    pub fn the_application_run_ends(&self) {
        let Ok(standing) = self.standing.lock() else {
            return;
        };
        for period in standing.values() {
            period.end_it.at(PeriodEnding::TheApplicationRunEnds);
        }
    }

    fn it_ends_at(&self, server: &str, ending: PeriodEnding) -> bool {
        let Ok(standing) = self.standing.lock() else {
            return false;
        };
        let Some(period) = standing.get(server) else {
            return false;
        };
        period.end_it.at(ending);
        true
    }
}

/// **The two facts a screen needs, put back together from the two places they
/// honestly live.**
///
/// `Servers`' `condition::observed_value` composes its three values out of
/// four facts. **The run holds the value rather than the four**, and the one
/// fact the value cannot carry is whether a period stands — `Servers` K8a's
/// own boolean, which is exactly why that clause exists.
///
/// **What is not read back, and it is stated rather than guessed.**
/// *A run of attempts is in flight* is recoverable from *reconnecting* and is
/// needed nowhere else, so it is written as what the value says.
///
/// **The restart mark is read off the standing period.** `Servers` M1 puts it
/// there at the moment `Server Setup`'s restart command leaves, and `Servers`
/// K8 composes the *restarting* presentation from it beside the observed value
/// — so a server Smoozy has just restarted reads *restarting* rather than
/// *reconnecting*, and `Servers` M8 raises no condition record under it.
/// *(Until step 10c's `Server Setup` §8 this was written false, which was a
/// true reading of an application that caused no restarts;
/// `crate::setting_up`'s restart step causes them now.)*
pub fn the_facts_behind(
    observed: ObservedValue,
    a_period_stands: bool,
    the_restart_mark_stands: bool,
) -> WhatIsTrueOfTheServer {
    WhatIsTrueOfTheServer {
        a_period_stands,
        the_primary_connection_is_live: a_period_stands
            && matches!(observed, ObservedValue::Connected),
        a_run_of_attempts_is_in_flight: a_period_stands
            && matches!(observed, ObservedValue::Reconnecting),
        the_restart_mark_stands: a_period_stands && the_restart_mark_stands,
    }
}

// --------------------------------------------------------- opening a period

/// **What the user's act asked for, and the two that open a period are
/// `Servers` S5's own.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhoAsked {
    /// **`Servers` S5's first opening act, `Servers` O1: the user's connect
    /// action on a saved server.**
    TheUsersConnectAction,
    /// **`Servers` E2: the put-back is `Servers` O1 performed by Smoozy inside the user's
    /// own save.** `Servers` S8 keeps it out of the replacement list, and
    /// `Servers` E6 and E7 are why who took the act is carried rather than
    /// derived: **no passphrase box may appear on it**.
    TheEditsPutBack,
    /// **`Servers` S5's second opening act, `Servers` O2: an add with a setup
    /// option ticked**, which the spec makes connect right away to carry the
    /// setup out. **When the setup ends the server stays connected**, so the
    /// period stands until the user disconnects or the run ends. Built at step
    /// 10c piece 9, which is the first thing in this application that could
    /// carry a setup out.
    AnAddWithASetupOptionTicked,
}

impl WhoAsked {
    fn as_the_period_reads_it(self) -> (OpeningAct, WhoTookIt) {
        match self {
            WhoAsked::TheUsersConnectAction => {
                (OpeningAct::TheUsersConnectAction, WhoTookIt::TheUser)
            }
            WhoAsked::AnAddWithASetupOptionTicked => (
                OpeningAct::AnAddWithASetupOptionTicked,
                WhoTookIt::TheUser,
            ),
            WhoAsked::TheEditsPutBack => (
                smoozy_servers::period::THE_PUT_BACK_IS_O1,
                smoozy_servers::period::THE_PUT_BACK_IS_TAKEN_BY,
            ),
        }
    }
}

/// **How an act that opened a period came out**, as much of it as `Servers`'
/// driving half reports back.
///
/// **Four answers and not two.** *Nothing was begun* used to cover a row that
/// is gone, a period already standing and a record that would not read, and
/// all three were told *"This server has been removed."* *(Split 2026-09-18,
/// step 10b's first audit round, item 14.)*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HowItWent {
    /// A period was begun and is being driven.
    APeriodStands,
    /// **`Servers` S2: a period already stands for this server**, so a second
    /// press begins nothing — and there is nothing to say: the screen redraws
    /// from what the run holds.
    OneAlreadyStands,
    /// **The row is not in the record** — another act removed it.
    TheServerIsGone,
    /// **The record would not read**, or the run's register is not there.
    TheRecordWouldNotRead,
}

/// **`Servers` S5: a period begins at the act.**
///
/// **The period is begun here, on the thread the act arrived on, and driven on
/// a thread of its own.** Beginning it here is what makes the register true the
/// instant the act is taken, so a second press finds a period standing rather
/// than opening a second one.
///
/// **P5: nothing that can block runs on the drawing thread.** The record is
/// read under the application's one lock and the lock is given back before the
/// thread starts; every connection, reading and wait after that is the new
/// thread's.
pub fn a_period_is_opened<R: Runtime>(
    app: &AppHandle<R>,
    server: &str,
    who: WhoAsked,
) -> HowItWent {
    a_period_is_opened_carrying(app, server, who, None)
}

/// **The same, carrying what `Keys`' passphrase box took for this connect.**
///
/// **`Preparation/Design/Keys.md` puts the box on the connection** — *"So the
/// box is the connection's, not this screen's"* — and `Keys` S16 orders it
/// *the readings, then the box, then `Keys` S5's supply*. So pressing the box's
/// *Connect* opens this server's connect with the typed value carried to its
/// one attempt as `what_the_box_took`: **borrowed for the connect and dropped
/// with it, and never read back out of the store** — which is `Keys` S9's
/// *a connect the user starts works normally, asking each time* on a computer
/// with nowhere to remember. *(Added 2026-09-18, step 10b's first audit round,
/// item 3: the box used to put what was typed into the store and connect
/// nothing.)*
pub fn a_period_is_opened_carrying<R: Runtime>(
    app: &AppHandle<R>,
    server: &str,
    who: WhoAsked,
    what_the_box_took: Option<Origin>,
) -> HowItWent {
    let Some(periods) = app.try_state::<Arc<ThePeriodsThisRun>>() else {
        return HowItWent::TheRecordWouldNotRead;
    };
    let periods = periods.inner().clone();
    let row = match the_row_named(app, server) {
        Ok(Some(row)) => row,
        Ok(None) => return HowItWent::TheServerIsGone,
        Err(TheRecordWouldNotRead) => return HowItWent::TheRecordWouldNotRead,
    };

    let (by, taken_by) = who.as_the_period_reads_it();
    let mut period = APeriod::begun(server, by, taken_by, SystemTime::now());
    if !periods.a_period_has_begun(server, &period) {
        return HowItWent::OneAlreadyStands;
    }

    let taking = app.clone();
    let server = server.to_string();
    std::thread::spawn(move || {
        let outcome = the_period_is_driven(
            &taking,
            &periods,
            &mut period,
            row,
            what_the_box_took.as_ref(),
        );
        // **`Servers` S11: every period-held thing ends the moment the period
        // ends.** The register's entry goes **before anything is said**, so a
        // screen drawn from the answer already reads *no period stands*, and a
        // box raised by this connect opens a period of its own when it is
        // answered rather than meeting this one still standing.
        let the_edits_save = periods.a_period_has_ended(&server);
        // **A decline of the queue's *Ignore* made during this period ends
        // with it** — `Servers` S11, and the other Subject's must-do 6 in
        // step 10b's first audit round (the runner's R5 to this one).
        crate::surface::the_queue::a_connected_period_ended(&server);
        // **`Server Setup` R1b's period-held answer ends with the period** —
        // a swap owed is lost, and the next comparison finds it again.
        // Added 2026-10-12.
        crate::servers::the_driver_swap::the_period_ended(&server);
        // **`File Browser` U7's paragraph of 2026-09-28, and `Download`
        // M13's: a period's end does to that server's uploads and manual
        // downloads what closing Smoozy does.** Told to the run's register
        // of transfers here, at the moment the register of periods has let
        // the period go, so a request waiting for this period's connection
        // ends rather than waiting for a point that will never lend again.
        // Step 10c's transfer wave piece 6; the reason it is told rather
        // than read is at `TheRunsTransfers::a_servers_period_ended`.
        if let Some(transfers) = taking.try_state::<Arc<crate::downloading::TheRunsTransfers>>() {
            transfers.a_servers_period_ended(&server);
        }
        drop(what_the_box_took);
        what_the_period_left_is_said(&taking, &server, outcome, the_edits_save);
    });
    HowItWent::APeriodStands
}

/// **What the driving thread brings back**, to be said once the period has
/// left the register.
struct TheDrivingsOutcome {
    ended: HowTheDrivingEnded,
    /// **`Keys` S19's box, where this connect raised one**, with the key row it
    /// names.
    the_box: Option<(smoozy_keys::rows::KeyRow, smoozy_keys::passphrase::TheAsk)>,
    /// **Whether the box was raised at all** — `Servers` P3's *marked with
    /// what is wrong*, read rather than composed.
    the_box_was_raised: bool,
    /// **`Security and licence` S34's sentence, owed with the box** where this
    /// connect's supply was the first caller told to say it.
    the_sentence_goes_with_the_box: bool,
    /// **Why no passphrase could be handed over, where Smoozy knows** — the
    /// channel's own sentence (L1, 2026-09-18). `None` everywhere else.
    the_passphrase_could_not_be_used: Option<&'static str>,
}

/// **The whole life of one period, on its own thread.**
///
/// **`tauri::async_runtime::block_on` is the shell's own runtime** — the same
/// one this crate already spawns blocking work on — and it is what gives
/// `Servers`' wiring the clock its waits are measured on and the reactor its
/// sockets are opened on. The file's head says why this rather than a second
/// runtime.
///
/// **Answers nothing where the driving could not begin** — the application's
/// state is not there, or Smoozy's own temporary area cannot be named for the
/// passphrase channel — and the caller then says the connection was not made,
/// rather than returning in silence. *(Step 10b's first audit round, items 15
/// and 16.)*
fn the_period_is_driven<R: Runtime>(
    app: &AppHandle<R>,
    periods: &ThePeriodsThisRun,
    period: &mut APeriod,
    row: ServerRow,
    what_the_box_took: Option<&Origin>,
) -> Option<TheDrivingsOutcome> {
    let started = app.try_state::<Mutex<Started>>()?;
    // **L4: the record is lent for one read or one write and never held across
    // an act.** A connection's work is seconds long and the application has one
    // lock.
    let record = crate::keys::TheRunsRecord {
        started: started.inner(),
    };
    let store = crate::surface::keys::the_store_for_this_system();
    // **L3a: the channel is opened in Smoozy's own temporary area and never in
    // the operating system's shared one**, so where the area cannot be named no
    // attempt is made.
    let route = TheRoute::of_this_application(locations::smoozys_own_temporary_area()?).ok()?;

    // **`Keys`' end of the credential seam, one per connect and dropped with
    // it.** `Servers` P1, `Servers` P2, `Servers` P4, `Servers` P4a and
    // `Servers` N8b are all answered through it and none of them is composed
    // here. **`what_the_box_took` is the box's own value where this connect is
    // the box's *Connect*** — see [`a_period_is_opened_carrying`].
    let credentials = the_credential_for_this_connect(
        &record,
        crate::surface::keys::the_store(&store),
        route,
        crate::surface::keys::THE_STORE_DEADLINE,
        &crate::the_store_standing::THIS_RUN,
        what_the_box_took,
    );
    // **`Server protocol and transfers` T5: the connection is lent from this
    // thread, through this server's point**, while `Servers`' driving waits on
    // it — see [`TheTransportThroughThePoint`].
    let opener = TheTransportThroughThePoint {
        sessions: periods
            .the_sessions_of(&row.local_server_identifier)
            .unwrap_or_default(),
        the_additional: the_additional_connection_for(
            app.clone(),
            row.local_server_identifier.clone(),
            periods.where_a_period_publishes(),
        ),
        when_it_is_lent: the_machines_work_begins(app.clone(), row.local_server_identifier.clone()),
        when_it_ends: the_machines_work_stops(app.clone(), row.local_server_identifier.clone()),
        when_it_authenticates: the_remembered_setup_is_taken(
            app.clone(),
            row.local_server_identifier.clone(),
        ),
        the_remembered_setup: Arc::new(Mutex::new(None)),
        the_server: row.local_server_identifier.clone(),
    };
    let mut half = TheActingHalf {
        opener,
        credentials,
        // **Every connection under this period reads against the row as the
        // record holds it then** — the return after a restart the ready action
        // caused included, which until step 10c's eighth-pass fix piece 3 read
        // the package area and installation the period began with, and so read
        // a machine Smoozy had just placed itself on as *absent*.
        installation: TheMachinesInstallationIdentity::for_this_server(&row, OverTheConnection)
            .reading_the_row_as_it_stands({
                let (app, server) = (app.clone(), row.local_server_identifier.clone());
                move || the_row_named(&app, &server).ok().flatten()
            })
            .telling(TheSupervisorsConditionIsRaised {
                periods,
                server: row.local_server_identifier.clone(),
            }),
        // **`Servers` §12's act one, written where it is owed** — added
        // 2026-09-22, step 10c's fourth pass on Vast, fault 7: that module's
        // declaration had been written and registered by nothing, so a whole
        // pass left three lines in the diary and a connected server left none.
        publishes: TheConnectedLineIsWritten {
            publishes: periods.where_a_period_publishes(),
            app: app.clone(),
            row: row.clone(),
            opened_by: period.opened_by(),
            written: false,
        },
        wait: TheDesktopsClock,
    };
    let mut desk = WhatTheDesktopKeeps {
        record: &record,
        periods,
    };

    let ended = tauri::async_runtime::block_on(period.drive(&row, &mut half, &mut desk));

    // **`Keys` S19's box, where this connect raised one.** Piece 2 builds the
    // ask and `src/surface/keys.rs` posts it; nothing here composes a word of
    // it. **It is read after the driving rather than during it**: `Servers` P3
    // forbids an automatic attempt asking at all, so the only attempt that can
    // raise one is the opening one, and an attempt that raised one supplied no
    // credential and ended the period at `Servers` S10 item 5.
    let the_box = half
        .credentials
        .the_ask
        .clone()
        .and_then(|ask| the_key_row_named(app, &row).map(|key| (key, ask)));
    let the_box_was_raised = matches!(
        half.credentials.why_nothing_was_supplied,
        Some(WhyNothingWasSupplied::NothingIsHeldAndTheBoxIsRaised)
    );
    // **`Security and licence` S34's sentence, carried to the place
    // `Preparation/Design/Keys.md` gives it**: with the box where this connect
    // raised one, and on the server's row where the moment came with nobody in
    // front of a box.
    let the_sentence_is_owed = half.credentials.the_sentence_is_owed;
    let the_sentence_goes_with_the_box = the_sentence_is_owed && the_box.is_some();
    if the_sentence_is_owed && the_box.is_none() {
        periods.the_sentence_is_owed_on(&row.local_server_identifier);
    }
    let the_passphrase_could_not_be_used = the_reason_no_passphrase_was_handed_over(
        half.credentials.why_nothing_was_supplied.as_ref(),
    );
    Some(TheDrivingsOutcome {
        ended,
        the_box,
        the_box_was_raised,
        the_sentence_goes_with_the_box,
        the_passphrase_could_not_be_used,
    })
}

/// **`Servers` §12's act one, written the first moment the observed value
/// reaches *connected* under the period this act opened.** Added 2026-09-22.
///
/// **Why it wraps the publisher rather than sitting beside it.** That module's
/// declaration fixes the moment exactly: *"the first moment the observed value
/// reaches connected under the period that act opened"*, **not at
/// authentication** — `Servers` Q13's outcomes 2 and 3 abandon the sequence
/// after it and S10 item 5 ends the period with no connection ever live, so a
/// line written at `Servers` B2's instant would say *connected* of a server
/// that reads disconnected. The publisher is where that moment is, so this is
/// where the line is written.
///
/// **One line per period-opening act** — `written` is what keeps it to one, and
/// a period is one task: a repair attempt and its return, the return after a
/// restart and the additional connection each publish *connected* again under
/// the same period and write nothing, which is the declaration's own bar.
///
/// **The line is `Servers`' own composition and nothing is composed here** —
/// [`smoozy_servers::diary::Act::line`] fills the fixed form, and this hands it
/// the row's values.
struct TheConnectedLineIsWritten<R: Runtime> {
    publishes: WhereThisPeriodPublishes,
    app: AppHandle<R>,
    row: ServerRow,
    opened_by: smoozy_servers::period::OpeningAct,
    written: bool,
}

impl<R: Runtime> TheConnectedLineIsWritten<R> {
    /// **The line, handed to the diary service through the module's own
    /// registration.** The answer goes nowhere, which is
    /// `Desktop app and delivery` L16's one legitimate use of it: a refused
    /// write loses the entry and nothing else, and the user never meets the
    /// diary.
    fn the_line_is_written(&self) {
        use tauri::Manager;
        let Some(started) = self.app.try_state::<std::sync::Mutex<crate::start::Started>>() else {
            return;
        };
        let Ok(started) = started.lock() else {
            return;
        };
        let (Some(service), Some(run)) = (started.diary.as_ref(), started.run.as_ref()) else {
            return;
        };
        let act = smoozy_servers::diary::Act::AServerConnected(
            smoozy_servers::diary::AServerConnected {
                opening_act: self.opened_by,
                local_server_identifier: self.row.local_server_identifier.clone(),
                label: self.row.label.clone(),
                host: self.row.host.as_str().to_string(),
                port: self.row.port.number(),
                username: self.row.username.as_str().to_string(),
            },
        );
        let diary = crate::diary_service::ModulesDiary {
            service,
            declarations: &started.declarations,
            run: run.clone(),
            module: crate::servers::published::THE_MODULE,
        };
        let _ = diary.write(&act.line(run.clone(), std::time::SystemTime::now()));
    }
}

impl<R: Runtime> WhatServersPublishes for TheConnectedLineIsWritten<R> {
    fn the_observed_value(&mut self, server: &str, value: ObservedValue) {
        if value == ObservedValue::Connected && !self.written {
            self.written = true;
            self.the_line_is_written();
        }
        self.publishes.the_observed_value(server, value);
    }

    fn the_next_attempt_due(&mut self, server: &str, reading: NextAttemptDue) {
        self.publishes.the_next_attempt_due(server, reading);
    }

    fn raise_the_connection_condition(&mut self, server: &str, says: WhatTheRecordSays) {
        self.publishes.raise_the_connection_condition(server, says);
    }

    fn withdraw_the_connection_condition(&mut self, server: &str) {
        self.publishes.withdraw_the_connection_condition(server);
    }

    fn the_changed_identity_warning(&mut self, server: &str) {
        self.publishes.the_changed_identity_warning(server);
    }

    fn record_the_scanned_host_key(&mut self, server: &str, key: RememberedHostKey) {
        self.publishes.record_the_scanned_host_key(server, key);
    }
}

/// **What the user is told where the supply could not hand a passphrase to
/// the SSH tools, and knows why** — step 10b's audit round's loose ends, L1.
///
/// The supply held the reason beside `Servers`' claimless ending and nothing
/// read it, so a channel whose path was too long reached the user as *could
/// not connect*. Only Smoozy's own sentence is taken; what the system reported
/// is a diagnostic and stays off the screen.
pub fn the_reason_no_passphrase_was_handed_over(
    why: Option<&WhyNothingWasSupplied>,
) -> Option<&'static str> {
    match why? {
        WhyNothingWasSupplied::TheLoadCouldNotBeRun { why } => why.said,
        _ => None,
    }
}

/// **`Keys`' side of one connect, composed the one way every connect composes
/// it** — including the passphrase box's *Connect*, whose typed value is
/// carried here to the connect's one attempt and nowhere else. Public so the
/// route can be driven end to end at the desk
/// (`tests/the_credential_for_a_connection.rs`).
pub fn the_credential_for_this_connect<'a>(
    record: &'a dyn crate::keys::LendsTheRecord,
    store: &'a dyn smoozy_security_and_licence::secret_store::SecretStore,
    route: TheRoute,
    deadline: std::time::Duration,
    this_run: &'a crate::the_store_standing::TheProbeThisRun,
    what_the_box_took: Option<&'a Origin>,
) -> TheCredentialForAConnection<'a> {
    let mut credentials = TheCredentialForAConnection::over(record, store, route, deadline, this_run);
    credentials.what_the_box_took = what_the_box_took;
    credentials
}

/// **What a period that has ended leaves to be said**, once it has left the
/// register.
fn what_the_period_left_is_said<R: Runtime>(
    app: &AppHandle<R>,
    server: &str,
    outcome: Option<TheDrivingsOutcome>,
    the_edits_save: Option<std::sync::mpsc::Receiver<bool>>,
) {
    let Some(outcome) = outcome else {
        // **The driving could not begin, and saying so is the whole answer**
        // (step 10b's first audit round, item 15).
        crate::surface::servers::the_connection_was_not_made(app, server);
        return;
    };
    if let Some((key, ask)) = &outcome.the_box {
        crate::surface::keys::the_ask_is_posted(
            app,
            key,
            ask,
            Some(server),
            outcome.the_sentence_goes_with_the_box,
        );
    }
    crate::surface::servers::the_period_ended(
        app,
        server,
        outcome.ended,
        outcome.the_box_was_raised,
        outcome.the_passphrase_could_not_be_used,
    );

    // **`Servers` E2: the put-back, opened from here.** This is the one place
    // that knows the old period has gone — it has just left the register — and
    // it waits for the save to say whether it landed, so the put-back is
    // opened after both and never raced against either. **`Servers` §14's
    // *Edit* row: where the save fails the put-back is not attempted.**
    if outcome.ended == HowTheDrivingEnded::ThePeriodEnded(PeriodEnding::TheEditsSave) {
        let landed = the_edits_save.is_some_and(|told| told.recv() == Ok(true));
        if landed {
            match a_period_is_opened(app, server, WhoAsked::TheEditsPutBack) {
                HowItWent::APeriodStands | HowItWent::OneAlreadyStands => {}
                // **Where the put-back cannot begin, the server is not left
                // disconnected in silence.**
                HowItWent::TheServerIsGone | HowItWent::TheRecordWouldNotRead => {
                    crate::surface::servers::the_connection_was_not_made(app, server);
                }
            }
        }
    }
}

/// **`Servers` D8 and Q1 steps 5 and 6: the desktop's side of the driving.**
///
/// **Each attempt reads the row as the record holds it at that moment**, so the
/// host key the first connection wrote is the one the next attempt compares
/// against, and an accept that cleared it takes effect at the next attempt of
/// the same repair. **What each primary connection read comes back here**: the
/// fresh hardware reading is written to the row where `Servers` H18 takes one
/// (`Servers` H4: it replaces the held one whole), and the branch is held for
/// the run so the list draws `Servers` H1's state from it.
pub(crate) struct WhatTheDesktopKeeps<'a> {
    pub(crate) record: &'a dyn crate::keys::LendsTheRecord,
    pub(crate) periods: &'a ThePeriodsThisRun,
}

impl TheDesktopsSide for WhatTheDesktopKeeps<'_> {
    fn the_row_now(&mut self, server: &str) -> Option<ServerRow> {
        self.periods.another_attempt_begins(server);
        let mut row = None;
        self.record.lend(&mut |connection| {
            row = server_rows::one(connection, server).ok().flatten();
        });
        row
    }

    fn the_connection_read(
        &mut self,
        server: &str,
        check: smoozy_state::identities::InstallationCheck,
        hardware: Option<smoozy_servers::hardware::HardwareReading>,
    ) {
        let mut a_fresh_reading_was_kept = false;
        if let Some(reading) = hardware {
            self.record.lend(&mut |connection| {
                let Ok(Some(mut row)) = server_rows::one(connection, server) else {
                    return;
                };
                row.remembered_hardware_reading = Some(reading.clone());
                a_fresh_reading_was_kept = server_rows::write(connection, &row).is_ok();
            });
        }
        // **`State, identity and recovery` V1 step 1, at that document's §7:
        // where the branch is absent or different, one thing is written before
        // stopping** — every local job record of this server whose render
        // start was never learned is marked *start never heard* and enters
        // history (`Queue & History` C9's second way in). **The Creator's ruling
        // C1 of 2026-09-18**, built by step 10b at the store and first called
        // here, by step 10c piece 7. **Neither the confirmed branch nor the
        // fourth — present, and the supervisor did not answer — writes
        // anything**: that one stops at step 1 with nothing discarded.
        if matches!(
            check,
            smoozy_state::identities::InstallationCheck::Absent
                | smoozy_state::identities::InstallationCheck::PresentAndDifferent
        ) {
            the_server_is_gone_from_its_machine(self.record, server);
        }
        self.periods.a_connection_read(
            server,
            WhatTheLastConnectionRead {
                check,
                a_fresh_reading_was_kept,
                on_the_connection_that_is_live: true,
            },
        );
        // **The list draws the hardware from the row and the branch**, and
        // both have just moved.
        self.periods.the_publisher().the_server_is_told_again(server);
    }
}

/// **`Queue & History` C9's second way in, performed for `State, identity and
/// recovery` V1 step 1's absent or different branch** — the Creator's ruling
/// C1 of 2026-09-18. The statement is the store's own
/// (`crate::record::job_records::JobRecords::the_server_is_gone`), which the
/// server row's removal runs too, so the two cannot mark different records.
/// **No absence, of any length, calls it**: only a branch that read the
/// machine and found Smoozy's installation gone.
pub(crate) fn the_server_is_gone_from_its_machine(
    record: &dyn crate::keys::LendsTheRecord,
    server: &str,
) -> Option<usize> {
    let server = smoozy_state::identities::Identity::read_back(
        smoozy_state::identities::IdentityKind::LocalServer,
        server,
    )?;
    let mut marked = None;
    record.lend(&mut |connection| {
        marked = crate::record::job_records::JobRecords::over(connection)
            .the_server_is_gone(&server)
            .ok();
    });
    marked
}

/// **`State, identity and recovery` Y9's fourth branch raises the
/// supervisor-unavailable condition**, and `State, identity and recovery` V1
/// step 1 says the same. The reader is the one that knows which case it met,
/// so it tells this — see `crate::servers::installation`.
struct TheSupervisorsConditionIsRaised<'a> {
    periods: &'a ThePeriodsThisRun,
    server: String,
}

impl crate::servers::installation::WhatTheSupervisorDid for TheSupervisorsConditionIsRaised<'_> {
    fn it_did_not_answer(&mut self) {
        self.periods
            .the_publisher()
            .the_supervisor_is_unavailable(&self.server);
    }

    fn it_answered(&mut self) {
        self.periods
            .the_publisher()
            .the_supervisor_answered(&self.server);
    }
}

/// Why a read of the row came back with nothing to act on.
struct TheRecordWouldNotRead;

/// The server row this act is about, read under the one lock and nothing else
/// done under it — **or the reason there is none**, because a row that is gone
/// and a record that would not read are two different things to tell a user.
fn the_row_named<R: Runtime>(
    app: &AppHandle<R>,
    server: &str,
) -> Result<Option<ServerRow>, TheRecordWouldNotRead> {
    let started = app.try_state::<Mutex<Started>>().ok_or(TheRecordWouldNotRead)?;
    let started = started.lock().map_err(|_| TheRecordWouldNotRead)?;
    let record = started.record.as_ref().ok_or(TheRecordWouldNotRead)?;
    server_rows::one(record.connection(), server).map_err(|_| TheRecordWouldNotRead)
}

/// The key row the server row's reference names, for the ask's posting.
fn the_key_row_named<R: Runtime>(
    app: &AppHandle<R>,
    row: &ServerRow,
) -> Option<smoozy_keys::rows::KeyRow> {
    let named = smoozy_state::identities::Identity::read_back(
        smoozy_state::identities::IdentityKind::LocalKey,
        &row.key.local_key_identifier,
    )?;
    let started = app.try_state::<Mutex<Started>>()?;
    let started = started.lock().ok()?;
    let record = started.record.as_ref()?;
    crate::record::key_rows::all(record.connection())
        .ok()?
        .into_iter()
        .find(|held| held.identifier == named)
}

/// **`Servers` Q6's accept path, performed: the recorded value is replaced and
/// the first-connection path is repeated.**
///
/// **Replacing it is clearing it, and that is the clause read literally.** Q6's
/// accept *"replaces the recorded value and repeats the first-connection
/// path"*, and the first-connection path is the one where nothing is
/// remembered: the next attempt trusts what the machine offers and records it,
/// with nothing asked and nothing shown. **The offered key is not kept anywhere
/// between the attempt and the answer** — `Servers` D9 bars every destination
/// but the row, and the comparison that held it went with the attempt — so
/// there is no value here to write in its place.
///
/// **What this does not do is tell a standing period the question was
/// answered.** `Servers` Q17's bound lives on `APeriod`'s own
/// `the_user_accepted_the_changed_identity`, and the only route to it is
/// `&mut APeriod`, which the driving thread holds for the period's whole life.
/// The consequence is narrow and is recorded rather than worked around: inside
/// one standing repair, a **second** differing identity raises no second
/// warning.
pub fn the_changed_identity_is_accepted<R: Runtime>(app: &AppHandle<R>, server: &str) -> bool {
    let Some(started) = app.try_state::<Mutex<Started>>() else {
        return false;
    };
    let wrote = {
        let Ok(started) = started.lock() else {
            return false;
        };
        let Some(record) = started.record.as_ref() else {
            return false;
        };
        let Ok(Some(mut row)) = server_rows::one(record.connection(), server) else {
            return false;
        };
        row.remembered_host_key = None;
        server_rows::write(record.connection(), &row).is_ok()
    };
    if !wrote {
        return false;
    }
    // **`Servers` Q6: *the attempt goes ahead*.** Where a period stands, the
    // repair's next attempt is the one that goes ahead and nothing is started
    // here; where none does, the accept is the user asking to connect and the
    // period the declined attempt ended is opened again.
    if let Some(periods) = app.try_state::<Arc<ThePeriodsThisRun>>() {
        if periods.a_period_stands_for(server) {
            return true;
        }
    }
    matches!(
        a_period_is_opened(app, server, WhoAsked::TheUsersConnectAction),
        HowItWent::APeriodStands
    )
}

// ------------------------------------------------ lending the live connection

/// **`Servers`' transport, with the wait on a live connection made the
/// per-server point's lending.**
///
/// **Everything but the wait is `Servers`' own transport, unchanged** — K1's
/// opening, the ending read without asking, and the tear-down. **The wait is
/// where `Servers`' driving holds a live connection and does nothing else with
/// it**, so it is where the point serves: `crate::servers::sessions::lend_until_it_ends`
/// answers with the transport's own report exactly as the plain wait did, and
/// `Servers`' driving races it against the period ending exactly as before. A
/// period that ends mid-lend drops the lending with the wait.
pub struct TheTransportThroughThePoint {
    sessions: ThisServersSessions,
    the_additional: TheAdditionalConnection<ssh::Connection>,
    /// **`Servers` Q1 step 7's hand-over**: called once, the moment a live
    /// connection is first lent — see [`the_machines_work_begins`].
    when_it_is_lent: Arc<dyn Fn(&ThisServersSessions, Option<Vec<String>>) + Send + Sync>,
    /// **`Servers` K3 and K5: the connection's ending**, the moment the
    /// transport reports it — see [`the_machines_work_stops`].
    when_it_ends: Arc<dyn Fn() + Send + Sync>,
    /// **`Servers` B2 and `Servers` B5 item 1: the moment the transport reports the
    /// connection authenticated and usable** — see
    /// [`the_remembered_setup_is_taken`].
    when_it_authenticates: Arc<dyn Fn() -> Option<Vec<String>> + Send + Sync>,
    /// **What that write carried**, held from the authentication to the
    /// hand-over. **`Servers` B5 item 1 is exact that the clearing does not
    /// depend on the hand-over happening**, so the options are taken out of the
    /// record at the earlier moment and carried here.
    the_remembered_setup: Arc<Mutex<Option<Vec<String>>>>,
    /// **Whose transport this is**, so the Set up tab can read whether an
    /// add's setup still waits in it — see [`the_login_carried`].
    the_server: String,
}

impl OpenConnections for TheTransportThroughThePoint {
    type Connection = ssh::Connection;

    async fn open(
        &mut self,
        endpoint: Endpoint,
        credential: Credential,
        decide: Box<dyn DecideHostTrust>,
        deadline: std::time::Duration,
    ) -> Result<ssh::Connection, OpenFailure> {
        let opened = TheTransport.open(endpoint, credential, decide, deadline).await;
        // **`Servers` B2's instant, named exactly: the transport reports the
        // connection authenticated and usable.** **`Servers` B5 item 1 clears
        // the bit in the same durable write as that moment**, and the
        // remembered options are handed to `Server Setup` from what that write
        // carried — so no path that is established and then goes no further
        // leaves the bit standing.
        if opened.is_ok() {
            let carried = (self.when_it_authenticates)();
            the_login_carried(&self.the_remembered_setup, &self.the_server, carried);
        }
        opened
    }

    fn ended(&mut self, connection: &ssh::Connection) -> Option<Ending> {
        TheTransport.ended(connection)
    }

    /// **The lending begins, and step 7 is handed over the moment it has.**
    ///
    /// The lending is polled first — `biased` — so the point is live before
    /// the hand-over is made: its first poll marks the connection lent before it
    /// waits on anything. **The hand-over then waits for ever**, so it never
    /// ends the wait; only the connection's ending does, exactly as before.
    async fn wait_until_it_ends(&mut self, connection: &mut ssh::Connection) -> Ending {
        let TheTransportThroughThePoint {
            sessions,
            the_additional,
            when_it_is_lent,
            when_it_ends,
            the_remembered_setup,
            ..
        } = self;
        // **`Servers` Q1 step 8: a remembered setup, handed to `Server
        // Setup`.** It is taken from here rather than read again, because the
        // bit was cleared at `Servers` B2's instant and this is what that write
        // carried — **and only once the hand-over has run**, so the Set up tab
        // reads *Connecting…* until the run has the card (piece M).
        let lending = std::pin::pin!(sessions::lend_until_it_ends(sessions, connection, the_additional));
        let handed_over = async {
            the_setup_is_handed_over(the_remembered_setup, |remembered| {
                when_it_is_lent(sessions, remembered)
            });
            std::future::pending::<std::convert::Infallible>().await
        };
        let ending = tokio::select! {
            biased;
            ending = lending => ending,
            never = handed_over => match never {},
        };
        when_it_ends();
        ending
    }

    async fn tear_down(&mut self, connection: ssh::Connection) -> Ending {
        TheTransport.tear_down(connection).await
    }
}

/// **What the login carried, held in the transport until step 8** — and, where
/// it carried an add's setup, the Set up tab told where to read that it waits
/// ([`crate::setting_up::a_setup_waits_to_be_handed_over`]). A login that
/// carried none leaves nothing held, as before. Added 2026-10-07, the
/// Creator's fourth note 1, piece M.
pub(crate) fn the_login_carried(
    held: &crate::setting_up::TheSetupHeldForTheHandOver,
    server: &str,
    carried: Option<Vec<String>>,
) {
    let waits = carried.is_some();
    if let Ok(mut memory) = held.lock() {
        *memory = carried;
    }
    if waits {
        crate::setting_up::a_setup_waits_to_be_handed_over(server, held);
    } else {
        crate::setting_up::a_login_carried_no_setup(server);
    }
}

/// **`Servers` Q1 step 8's hand-over, and only then the memory let go.**
///
/// **The setup is read, handed over, and only then taken out**, so from the
/// login until step 8 has run — `Server Setup` Y9's exclusion taken or
/// refused, the tab told — the setup still waits, and *Connecting…* gives way
/// straight to the run. It is handed over exactly once: what was read is taken
/// out after. Added 2026-10-07, the Creator's fourth note 1, piece M.
pub(crate) fn the_setup_is_handed_over(
    held: &crate::setting_up::TheSetupHeldForTheHandOver,
    hand_over: impl FnOnce(Option<Vec<String>>),
) {
    let remembered = held.lock().ok().and_then(|memory| memory.clone());
    hand_over(remembered);
    if let Ok(mut memory) = held.lock() {
        *memory = None;
    }
    // **Step 8 has run**, so the Set up tab no longer reads this as an add's
    // setup that the first checks stopped (piece M2).
    crate::setting_up::the_setup_reached_step_8(held);
}

/// **`Servers` Q1 step 7, handed over at the moment its connection is lent.**
///
/// **This is the hand-over `smoozy_servers::sequence::STEPS_NOT_BUILT` leaves
/// to whoever drives the sequence.** Step 7 runs only where the connection
/// settled on the confirmed branch, which the step-7 thread checks against what
/// the driving handed back; **it runs on a thread of its own** because every
/// request it makes is served from this period's thread
/// (`crate::servers::the_machines_work`).
fn the_machines_work_begins<R: Runtime>(
    app: AppHandle<R>,
    server: String,
) -> Arc<dyn Fn(&ThisServersSessions, Option<Vec<String>>) + Send + Sync> {
    Arc::new(
        move |sessions: &ThisServersSessions, remembered: Option<Vec<String>>| {
            let Some(periods) = app.try_state::<Arc<ThePeriodsThisRun>>() else {
                return;
            };
            let connection = periods.the_machines_work().a_connection_begins(&server);
            // The point is serving requests now; Connected was published
            // earlier. Refresh the existing settings panel on every return.
            crate::surface::render_settings::the_connection_is_lent(&app, &server);
            crate::servers::the_machines_work::the_machines_work_is_read(
                app.clone(),
                server.clone(),
                sessions.clone(),
                connection,
            );
            // **`Download` R12: an arrangement whose desired state is on
            // resumes on any connection to its server**, replacement or
            // period-opening. This is that resumption, started with the
            // connection and ending with it.
            the_arrangements_are_driven(
                app.clone(),
                server.clone(),
                sessions.clone(),
                connection,
            );
            // **`Telegram` L2: what this server was not told — a new token,
            // Telegram switched on or off, a new label — is told on the
            // ordinary command path, on every connection** (`Telegram` L2).
            // On its own thread,
            // checked against the confirmed branch there. The 38's piece 11.
            crate::setting_up::the_tellings_are_delivered_on(
                app.clone(),
                server.clone(),
                sessions.clone(),
            );
            // **`Servers` Q1 step 8: a remembered setup, handed to
            // `Server Setup`.** It runs on a connect carrying one and on no
            // other, and it runs exactly once — the bit was cleared at
            // `Servers` B2's instant and this is what that write carried.
            if let Some(options) = remembered {
                crate::setting_up::the_remembered_setup_is_carried_out(
                    app.clone(),
                    server.clone(),
                    options,
                );
            }
        },
    )
}

/// **`Servers` Q19a, at the placement's commit** — the ready action's call,
/// made the moment the placement it performed has committed and the
/// installation is recorded against the row. Added 2026-09-27, step 10c's
/// eighth-pass fix piece 3.
///
/// **The clause is kept by
/// [`crate::servers::installation::the_retake_at_the_commit`]**, and what is
/// here is where it runs and what it hands on: **on a thread of its own**, for
/// the reason step 7's is — every read it takes is served by this period's
/// point — and **returning at once**, so the setup carries on while the re-take
/// is taken. **Nothing is started where this period's live connection did not
/// read the absent branch** — *Set up again* among them — and that is answered
/// before a thread is made.
///
/// **What the re-take read is kept the way every connection's is**
/// ([`WhatTheDesktopKeeps`]), and on the confirmed branch **`Servers` Q1 step 7
/// is started on the connection that is already live**, which starts
/// `Desktop app and delivery` P10d's passes after it — and **`Download` R12's
/// arrangements are driven on it too**: they were handed this connection when
/// it was lent and found no placement recorded to read a job's records under.
pub fn a_placement_committed<R: Runtime>(app: &AppHandle<R>, server: &str) {
    let Some(periods) = app.try_state::<Arc<ThePeriodsThisRun>>() else {
        return;
    };
    let periods = periods.inner().clone();
    let read = periods.what_the_last_connection_read(server);
    let owed = read.is_some_and(|read| {
        read.check == smoozy_state::identities::InstallationCheck::Absent
            && read.on_the_connection_that_is_live
    });
    if !owed {
        return;
    }
    let (app, server) = (app.clone(), server.to_string());
    let _ = std::thread::Builder::new()
        .name("smoozy re-take at the commit".to_string())
        .spawn(move || the_retake_is_taken(&app, &periods, &server, read));
}

fn the_retake_is_taken<R: Runtime>(
    app: &AppHandle<R>,
    periods: &ThePeriodsThisRun,
    server: &str,
    read: Option<WhatTheLastConnectionRead>,
) {
    let Some(sessions) = periods.the_sessions_of(server) else {
        return;
    };
    let holds = periods.the_machines_work();
    let Some(connection) = holds.the_connection_now(server) else {
        return;
    };
    let is_live = || sessions.a_connection_is_live() && holds.is_current(server, connection);
    // **The row as it stands after the commit**, which is what makes the
    // machine's new installation read as Smoozy's.
    let Some(row) = the_row_named(app, server).ok().flatten() else {
        return;
    };
    let mut reading = TheMachinesInstallationIdentity::for_this_server(&row, OverTheConnection)
        .telling(TheSupervisorsConditionIsRaised {
            periods,
            server: server.to_string(),
        });
    let mut channel = sessions.clone();
    let mut the_machine =
        crate::servers::installation::TheHardwareReadingOverThePoint { sessions: &sessions };
    let Ok(runtime) = tokio::runtime::Builder::new_current_thread().enable_all().build() else {
        return;
    };
    let Some((check, hardware)) = runtime.block_on(
        crate::servers::installation::the_retake_at_the_commit(
            read,
            &mut reading,
            &mut channel,
            &mut the_machine,
            &row,
            &mut TheDesktopsClock,
            &is_live,
        ),
    ) else {
        return;
    };
    let Some(started) = app.try_state::<Mutex<Started>>() else {
        return;
    };
    let record = crate::keys::TheRunsRecord {
        started: started.inner(),
    };
    WhatTheDesktopKeeps {
        record: &record,
        periods,
    }
    .the_connection_read(server, check, hardware);
    if check == smoozy_state::identities::InstallationCheck::PresentAndEqualConfirmed {
        crate::servers::the_machines_work::the_machines_work_is_read(
            app.clone(),
            server.to_string(),
            sessions.clone(),
            connection,
        );
        the_arrangements_are_driven(app.clone(), server.to_string(), sessions.clone(), connection);
    }
}

/// **`Servers` B5 item 1's own durable write, at `Servers` B2's instant.**
///
/// **The bit is cleared whatever the setup then does** — succeed, partly
/// succeed or fail — which is the spec's rule kept literally: *"Once Smoozy has
/// reached the machine, nothing about the setup is remembered."* **What the
/// write carried is handed back**, so the hand-over at step 8 reads it from
/// here rather than from a record that no longer holds it.
fn the_remembered_setup_is_taken<R: Runtime>(
    app: AppHandle<R>,
    server: String,
) -> Arc<dyn Fn() -> Option<Vec<String>> + Send + Sync> {
    Arc::new(move || {
        let started = app.try_state::<Mutex<Started>>()?;
        let mut carried = None;
        let record = crate::keys::TheRunsRecord {
            started: started.inner(),
        };
        crate::keys::LendsTheRecord::lend(&record, &mut |connection| {
            let Ok(Some(mut row)) = server_rows::one(connection, &server) else {
                return;
            };
            let Some(remembered) = row.remembered_setup.take() else {
                return;
            };
            if server_rows::write(connection, &row).is_ok() {
                carried = Some(remembered.options);
            }
        });
        carried
    })
}

/// **`Servers` K3 and K5: the break is declared**, so the queue reading this
/// connection took reads as not updating from now (`Queue & History` C7), its
/// P10d passes end, and the queue is drawn again to say so.
///
/// **The work column is told again here too** (`Design/Servers § A row`,
/// `Queue & History` C7): the break moves it to the dash without a reading,
/// and unless the detector hears that, the next reading's *Idle* matches what
/// it last told and the dash stands. Step 10c's tenth-pass fix piece 2.
fn the_machines_work_stops<R: Runtime>(app: AppHandle<R>, server: String) -> Arc<dyn Fn() + Send + Sync> {
    Arc::new(move || {
        crate::surface::render_settings::the_connection_ended(&server);
        if let Some(periods) = app.try_state::<Arc<ThePeriodsThisRun>>() {
            periods.the_machines_work().the_connection_ended(&server);
        }
        crate::surface::the_queue::the_queue_is_told_again(&app);
        crate::surface::servers::the_work_column_is_told_again(&app);
    })
}

/// **`Servers` S14's additional connection, opened when the point asks for
/// one.**
///
/// **It is made on a thread of its own**, the way a period is, because the
/// credential supply it needs is not `Send` and the point's lending must be.
/// **It runs `Servers`' own sequence with the additional role**, so `Servers`
/// Q16's steps 1 to 3 and no others are that crate's, and `Servers` S14
/// condition 3 is kept there too: it is a connection Smoozy makes on its own,
/// so where the key needs a passphrase Smoozy is not holding, no attempt is made
/// and the point hears nothing back. **`Servers` S14 condition 6: nothing about
/// it moves the server's observed value**, so the moment it is live is told to
/// nobody.
fn the_additional_connection_for<R: Runtime>(
    app: AppHandle<R>,
    server: String,
    publishes: WhereThisPeriodPublishes,
) -> TheAdditionalConnection<ssh::Connection> {
    Arc::new(move || -> ItsOwnFuture<'static, Option<ssh::Connection>> {
        let (tell, told) = tokio::sync::oneshot::channel();
        let (app, server, publishes) = (app.clone(), server.clone(), publishes.clone());
        std::thread::spawn(move || {
            let _ = tell.send(an_additional_connection_is_made(&app, &server, publishes));
        });
        Box::pin(async move { told.await.ok().flatten() })
    })
}

/// One additional connection, made on the thread this runs on.
fn an_additional_connection_is_made<R: Runtime>(
    app: &AppHandle<R>,
    server: &str,
    publishes: WhereThisPeriodPublishes,
) -> Option<ssh::Connection> {
    let row = the_row_named(app, server).ok().flatten()?;
    let started = app.try_state::<Mutex<Started>>()?;
    let record = crate::keys::TheRunsRecord {
        started: started.inner(),
    };
    let store = crate::surface::keys::the_store_for_this_system();
    let route = TheRoute::of_this_application(locations::smoozys_own_temporary_area()?).ok()?;
    let credentials = the_credential_for_this_connect(
        &record,
        crate::surface::keys::the_store(&store),
        route,
        crate::surface::keys::THE_STORE_DEADLINE,
        &crate::the_store_standing::THIS_RUN,
        None,
    );
    let mut half = TheActingHalf {
        opener: TheTransport,
        credentials,
        installation: TheMachinesInstallationIdentity::for_this_server(&row, OverTheConnection),
        publishes,
        wait: TheDesktopsClock,
    };
    let attempt = AnAttempt {
        row: &row,
        who: WhoStartedIt::SmoozyOnItsOwn,
        the_first_connection_to_this_server: false,
        role: ConnectionRole::Additional,
        deadline: smoozy_servers::repair::CONNECT_DEADLINE,
    };
    let mut warning = ChangedIdentityWarning::none_yet();
    match tauri::async_runtime::block_on(sequence::run(
        attempt,
        &mut half,
        &mut warning,
        &mut |_: &mut WhereThisPeriodPublishes| {},
    )) {
        Attempt::ItIsLive(gave) => Some(gave.connection),
        Attempt::ItStopped(_) => None,
    }
}

/// **What the period's own outcome says, where it says anything.**
///
/// *(Repaired 2026-09-18. This enum used to stand beside a constant recording
/// that the step which stopped an attempt never left `App/Servers/`, so
/// `Servers` Q13 outcome 2's sentence could not be said by anything driving a
/// period and the claimless one was said in its place. The driving half now
/// answers with the attempt's own stopping step where the ending is its own,
/// and the constant is gone because the thing it recorded is fixed.)*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheDrivingLeft {
    /// **`Servers` S10 item 5: the connection the opening act was to make never
    /// became live.** Smoozy says so and claims nothing about why.
    TheConnectionWasNotMade,
    /// **`Servers` Q13 outcome 2: the machine named something other than
    /// Linux**, so Smoozy says plainly that it is not a Linux machine and
    /// leaves the server disconnected.
    ///
    /// **One outcome of one check and nothing wider.** `Servers` Q13 outcome 3
    /// — a machine that answered nothing Smoozy could read — takes
    /// [`WhatTheDrivingLeft::TheConnectionWasNotMade`] with the other eight
    /// reasons, because the clause puts the two side by side precisely so that
    /// the claimless one stays claimless.
    ItIsNotALinuxMachine,
    /// **`Servers` Q6's third path: the machine offered an identity that is
    /// not the one remembered.** **The warning is the answer** — it was
    /// published while the attempt ran and it is what the user decides on — so
    /// no sentence is said beside it, the way none is said beside a raised
    /// passphrase box. *(Step 10b's first audit round, item 17: this took the
    /// claimless sentence as well, so the user was told twice.)*
    TheWarningIsTheAnswer,
    /// **`Servers` N8b: one refusal is permanent and the repair stops on it.**
    /// The period stands and a user-started connect still works and still asks.
    TheRepairStoppedForGood,
    /// A period that ran and was ended — by the user, by an edit, by a removal,
    /// or by the run ending. **Nothing is said**: the user asked for it.
    NothingToSay,
}

/// What to say, taken from what the driving answered.
pub fn what_the_driving_left(ended: HowTheDrivingEnded) -> WhatTheDrivingLeft {
    match ended {
        HowTheDrivingEnded::TheRepairStoppedForGood => WhatTheDrivingLeft::TheRepairStoppedForGood,
        // **`Servers` Q13 outcome 2, and it is asked rather than composed.**
        // The outcome carries its own answer to *does this claim what the
        // machine is* — `Servers` Q13 puts outcome 2 and outcome 3 side by side
        // precisely so the two stay apart — so nothing here reads the value a
        // second time and nothing here decides which outcomes claim.
        HowTheDrivingEnded::TheOpeningAttemptStopped(WhereItStopped::TheNotLinuxCheck(outcome))
            if outcome.claims_what_the_machine_is() =>
        {
            WhatTheDrivingLeft::ItIsNotALinuxMachine
        }
        // **`Servers` Q6's third path: the changed-identity warning was
        // published, and it is the answer.**
        HowTheDrivingEnded::TheOpeningAttemptStopped(WhereItStopped::TheIdentityDiffers) => {
            WhatTheDrivingLeft::TheWarningIsTheAnswer
        }
        // **Every other reason, including `Servers` Q13 outcome 3.** Smoozy says
        // the connection was not made and claims nothing about why, which is
        // `Preparation/Specs/Whole app.md`'s requirement.
        HowTheDrivingEnded::TheOpeningAttemptStopped(_) => {
            WhatTheDrivingLeft::TheConnectionWasNotMade
        }
        HowTheDrivingEnded::ThePeriodEnded(PeriodEnding::TheOpeningConnectionNeverBecameLive) => {
            WhatTheDrivingLeft::TheConnectionWasNotMade
        }
        // **`Servers` S10 item 4: a break declared while the key is not usable
        // ends the period**, and the key mark on the row is what already says
        // so — `Servers` K9 makes the two readings independent and the mark is
        // computed at every list draw.
        HowTheDrivingEnded::ThePeriodEnded(_) => WhatTheDrivingLeft::NothingToSay,
    }
}

// ------------------------------------ the arrangements a connected period drives

// **`Download` E1's turns, driven for as long as a connection to the server
// stands — step 10c, and the caller `Download`'s own sequencer never had.**
//
// **Nothing here decides anything `Download` decides.** The walk, the two
// vouching mechanisms, the name check, the ending walk, the fetch queue and
// every durable write are that crate's, performed by
// [`crate::downloading::one_turn_of_an_arrangement`]; **what is here is who
// calls it, how often, and what it is handed.**
//
// **`Download` W2's cadence is read and never written a second time.** The
// pause before the next walk — ten seconds, or the previous walk's own
// duration where that was longer, times twice the number of this server's
// walking arrangements less one — is
// [`smoozy_download::watch::the_pause_before_the_next_walk`]'s, and the turn
// itself hands back the figure it computed. **The aggregate bound is that
// clause's own**: all of a server's walks take at most one half of the
// connection's one reading channel between them.
//
// **It is a running sequence with a pause inside it and never a standing
// timer**, which is §3.3 P10a's first admitted shape and the same one
// `crate::servers::live_reading`'s passes and `Servers`' own repair run in. It
// begins when a connection is lent and ends with that connection.
//
// **`Download` R12 is why it does not wait for anything else**: an
// arrangement whose desired state is on *"survives every application run and
// resumes on any connection to its server — replacement or period-opening"*.
// So an arrangement made at a submission in one run is picked up by a later
// run's period, and one made during this period is picked up at the next pass.

/// **One arrangement's run-held activity, for the life of one connection.**
///
/// **`Download` R11: the arrangement's activity is run-held and dies with the
/// process; the record does not.** Everything here is that activity — the walk
/// series and its baseline, the ending in hand, the fetch queue, the probe's
/// cadence — and **none of it is durable**. A fresh connection re-walks and
/// re-derives, and reads the reckoning for what is already taken.
struct WhatOneArrangementRunsOn {
    generation: Option<Option<i64>>,
    running: TheArrangementsRun,
    queue: TheFetchQueue,
    probing: TheProbesCadence,
    /// **`Download` N2's read of the job's record directory: the manifest**,
    /// taken once for this connection. **`Job files and outputs` M8 never
    /// rewrites a manifest**, so a second read of the same connection would ask
    /// a question already answered; the reading is still a reading and is
    /// written nowhere.
    the_record_directory: Option<WhatTheRecordDirectoryHeld>,
    /// **`Download` N2's second home: the recorded reading the render worker
    /// wrote at render start, with the attempt it is for**, where the manifest
    /// carries none. **Kept only once it has landed**, and read again for a
    /// new attempt (`Job files and outputs` R13). Added 2026-09-28, step 10c's
    /// ninth-pass fix piece 1.
    the_recorded_reading: Option<(u32, WhatTheRecordDirectoryHeld)>,
    /// **The attempt whose ordered frame list is in hand** (`Download` J2), so
    /// a new attempt's list is read rather than the previous one reused.
    /// **Set only once the list has been read**, never on a read that found
    /// nothing.
    the_order_was_read_for: Option<u32>,
    order: TheOrder,
    /// **`Download` N5a's two inputs, with the attempt they are for**: what
    /// Smoozy placed in the job's folder before the render, and how each
    /// placed file stood once the render ended. **Read once the ending is in
    /// hand and kept once read**, a read that did not answer being asked again
    /// at the next pass. Added 2026-10-02, the fixes before 0.1.8, fault 3.
    what_smoozy_placed: Option<(u32, Option<WhatSmoozyPlaced>)>,
}

impl WhatOneArrangementRunsOn {
    /// **`Download` J6: on every (re)connection the baseline starts again**, so
    /// mechanism two may judge once more on the new connection.
    fn fresh() -> WhatOneArrangementRunsOn {
        let mut running = TheArrangementsRun::fresh();
        running.a_connection_began();
        WhatOneArrangementRunsOn {
            generation: None,
            running,
            queue: TheFetchQueue::at_a_server(),
            probing: TheProbesCadence::fresh(),
            the_record_directory: None,
            the_recorded_reading: None,
            the_order_was_read_for: None,
            order: TheOrder::ItCouldNotBeRead,
            what_smoozy_placed: None,
        }
    }
}

/// **Every arrangement one connection is driving, keyed by its job.**
#[derive(Default)]
pub struct TheArrangementsThisConnectionDrives {
    per_job: HashMap<String, WhatOneArrangementRunsOn>,
    /// `Download` J6: job -> (attempt, newest saved frame), from this
    /// connection's queue reading, refreshed before each existing walk pass.
    saved_prefixes: HashMap<String, (u32, i64)>,
    #[cfg(test)]
    recovery_stages: tests::RecoveryStages,
    /// **`Download` N6's whole for each job whose inventory a pass on this
    /// connection had in hand**, keyed by the job identity, and handed to the
    /// run after every pass. Added 2026-09-29, step 10c's transfer wave piece
    /// 4.
    wholes: HashMap<String, crate::servers::the_machines_work::TheJobsWhole>,
}

impl TheArrangementsThisConnectionDrives {
    pub fn fresh() -> TheArrangementsThisConnectionDrives {
        TheArrangementsThisConnectionDrives::default()
    }

    /// **Every whole the passes on this connection have read, handed to the
    /// run** — where the Transfers screen reads it
    /// (`crate::surface::transfers`), in place of the reckoning's length.
    /// Nothing durable is written (`Download` R5). Added 2026-09-29, step 10c's
    /// transfer wave piece 4.
    pub fn the_wholes_are_handed_to(
        &self,
        run: &crate::servers::the_machines_work::WhatThisRunHolds,
    ) {
        for (job, whole) in &self.wholes {
            run.a_jobs_whole_was_read(job, *whole);
        }
    }
}

/// **What this job's own local record says**, read in one short act with the
/// arrangement beside it. **Nothing here reads a machine**, which is what makes
/// it affordable at every pass.
struct WhatTheJobRecordSays {
    start_order: Option<i64>,
    /// **`Download` P10's landing folder name**, and absent is a value: the
    /// folder is then the bare job identity, which is safe in every system's
    /// unit.
    label: Option<String>,
    /// **`Download` W3's third condition, first half**: the job is between *a*
    /// render-start and the completion of the ending walk. Before the first
    /// render-start there is nothing to watch.
    the_render_start_was_learned: bool,
    /// **`Download` E1 step 1: the ending, one of `Queue & History` E2's four**,
    /// read off the record rather than found out here.
    ending: Option<smoozy_queue_history::ending::Ending>,
    /// **`Queue & History` N1's attempt count**, which is the attempt whose
    /// ordered frame list `Download` J2 reads. **Nought is not attempt one**:
    /// no note has arrived, so the attempt is not in hand and nothing is
    /// vouched for on a guessed order.
    attempts: u32,
    /// **The first-frame pair the job is one of, check then range**
    /// (`Queue & History` P5a's record), for `Download` P10's landing name and
    /// the range job's ending walk (`Job files and outputs` P18a). Added
    /// 2026-10-31, the twenty-first notes, note 6.
    pair: Option<(Identity, Identity)>,
    /// **`Download` E1 step 2's dated note of 2026-10-31: what the pair's
    /// other half holds *fetched* into this arrangement's destination**, so a
    /// file it already brought home is taken here rather than fetched. Empty
    /// for every job that is not one of a pair. The twenty-sixth notes, note 4.
    the_other_half_fetched: Vec<Vec<u8>>,
    /// **`Download` R5's dated note of 2026-10-31: the ending walk is written
    /// down as completed for this arrangement's start** (record version 57).
    the_ending_walk_is_written_down: bool,
}

/// **One pass over every arrangement this server holds.**
///
/// It answers **how long to wait before the next pass**, which is `Download`
/// W2's own pause for the walks this pass ran — measured from the end, never
/// from the start, so walks never overlap and a slow walk cannot pile requests
/// onto a struggling connection.
///
/// **The order inside the pass is: read the record, then walk and fetch.** The
/// local record is read once for everything — the arrangements and each job's
/// own record — and the lock is let go before the first machine reading, which
/// is the shape every act in this crate takes.
pub async fn one_pass_over_this_servers_arrangements<C: AHeldConnection>(
    holds: &mut TheArrangementsThisConnectionDrives,
    sessions: &ThisServersSessions<C>,
    the_run: &crate::downloading::TheRunsTransfers,
    record: &dyn crate::downloading::TheRecordADownloadWrites,
    server_row: &Identity,
    layout: &TheLayout,
    run: &Identity,
) -> std::time::Duration {
    let a_connection_to_the_server_stands = sessions.a_connection_is_live();
    let mut held: Vec<(AnArrangement, WhatTheJobRecordSays)> = Vec::new();
    record.with_the_record(&mut |connection| {
        held.clear();
        // **`Download` R12: every arrangement held against this server row is
        // what a connection resumes**, whichever label the connection carries.
        let Ok(every) = DownloadArrangements::over(connection).every_arrangement_for(server_row)
        else {
            return;
        };
        let jobs = JobRecords::over(connection);
        for arrangement in every {
            // **`Download` R3: switching it off ends the arranging of new
            // fetches from that moment** and removes nothing. What it stops is
            // the arranging, so this pass does not walk it and does not fetch
            // for it; transfers already arranged are the run's own and end
            // where they end.
            if arrangement.desired_state != DesiredState::On {
                continue;
            }
            // **The arrangement rides the job's entry** (`Download` R3), so a
            // job the local record no longer holds is walked for by nobody.
            let Ok(Some(job)) = jobs.read(&arrangement.job) else {
                continue;
            };
            let pair = the_first_frame_pair_of(&jobs, &arrangement.job);
            let stores = DownloadArrangements::over(connection);
            let the_other_half_fetched = pair
                .as_ref()
                .map(|(the_check, the_range)| if *the_check == arrangement.job { the_range } else { the_check })
                .and_then(|other| stores.the_paths_fetched_into(other, &arrangement.destination).ok())
                .unwrap_or_default();
            let the_ending_walk_is_written_down =
                stores.the_ending_walk_is_written_down(&arrangement.job).unwrap_or(false);
            let start_order = crate::record::transfer_records::parent_start(connection,
                "download_arrangement", "job", &arrangement.job.as_text()).ok().flatten().map(|start| start.start_order);
            held.push((
                arrangement,
                WhatTheJobRecordSays {
                    start_order,
                    label: job.label.clone(),
                    the_render_start_was_learned: job.the_render_start_has_been_learned,
                    // **`Queue & History` E2's mapping has one home** and
                    // this is not a second one: that module's own record
                    // answers it.
                    ending: job.the_ending_learned(),
                    attempts: job.what_the_note_carried.attempts,
                    pair,
                    the_other_half_fetched,
                    the_ending_walk_is_written_down,
                },
            ));
        }
    });

    #[cfg(test)]
    tests::a_recovery_stage(&mut holds.recovery_stages, "arrangements admitted");

    // **`Download` W2's own input: how many of this server's arrangements are
    // walking.** It is counted before any of them walks, so every walk of this
    // pass takes the same pause and the aggregate bound holds for the pass
    // rather than for whichever walk happened to be last.
    let mut walking = 0usize;
    let mut windows: Vec<bool> = Vec::with_capacity(held.len());
    for (arrangement, says) in &held {
        let one = holds
            .per_job
            .entry(arrangement.job.as_text())
            .or_insert_with(WhatOneArrangementRunsOn::fresh);
        // `Server protocol and transfers` S0a: a newly switched-on arrangement needs its own final walk,
        // including when an older arrangement of this job ended here.
        if one.generation != Some(says.start_order) {
            *one = WhatOneArrangementRunsOn::fresh();
            one.generation = Some(says.start_order);
            // **`Download` R5's dated note of 2026-10-31: a completion written
            // down for this start survives the restart**, where every unit is
            // terminal — so an ended job's folder is not walked to its end
            // once more, and a first-frame check's walk does not run again.
            if says.the_ending_walk_is_written_down {
                one.running.the_ending_walk_was_written_down(arrangement);
            }
        }
        // **`Download` E1 step 1: the ending arrives, live or at catch-up.**
        // The module never finds out on its own and this is where it is told.
        if let Some(ending) = says.ending {
            one.running.the_ending_arrived(ending);
        }
        // **`Download` W3's three conditions.** The desired state is on — an
        // arrangement that is off is not in this list at all; a connection
        // stands; and the job is between a render-start and the completion of
        // the ending walk.
        let window_is_open = a_connection_to_the_server_stands
            && says.the_render_start_was_learned
            && !one.running.the_ending_walk_has_completed();
        windows.push(window_is_open);
        if window_is_open {
            walking += 1;
        }
    }

    let mut longest = the_pause_before_the_next_walk(std::time::Duration::ZERO, walking.max(1));
    let mut walked: Vec<WhatOneArrangementWalkedWith> = Vec::with_capacity(held.len());
    for ((mut arrangement, says), window_is_open) in held.into_iter().zip(windows) {
        let key = arrangement.job.as_text();
        let generation = says.start_order;
        let Some(one) = holds.per_job.get_mut(&key) else {
            continue;
        };
        the_run.an_auto_ending_walk_was_read(&key, generation, one.running.the_ending_walk_has_completed());
        // **`Job files and outputs` N3's own composition of the job record
        // directory**, which is named by the job identity alone — the one path
        // this side can compose without reading anything, and the only way to
        // reach the manifest at all.
        let Ok(paths) = AJobsPaths::composed(layout, &arrangement.job, says.label.as_deref())
        else {
            continue;
        };
        if window_is_open && one.the_record_directory.is_none() {
            let mut reader = TheRecordDirectorysReadingRoute { sessions };
            let read =
                read_the_narrowing_material(&mut reader, paths.job_record_directory.as_bytes())
                    .await;
            // **A read that did not answer is not a reading and is not kept.**
            // `read_the_narrowing_material` answers *it has not landed* both
            // for a manifest that is not there and for one the connection
            // would not give up, and keeping the second would leave this
            // arrangement walking nothing for the rest of the connection on
            // one hiccup. **The manifest is the one thing that is kept**, and
            // it is kept only once it has been read: `Job files and outputs`
            // M8 never rewrites one, so a second read of the same connection
            // asks a question already answered.
            if read.job_output_folder.is_some() {
                one.the_record_directory = Some(read);
            }
        }
        // **The job output folder is the manifest's and is never composed a
        // second time here.** `Job files and outputs` M1 makes the folder the
        // manifest names and the folder that is actually made one value,
        // *"rather than two that have to be checked against each other"*. So
        // where the manifest has not been read there is no folder to walk, and
        // the pass leaves this arrangement alone rather than walking a path it
        // composed itself.
        let Some(the_job_output_folder) = one
            .the_record_directory
            .as_ref()
            .and_then(|read| read.job_output_folder.clone())
        else {
            continue;
        };

        // **`Download` N2's second home: the recorded reading the render
        // worker wrote at render start**, where the manifest carries none —
        // every job submitted without *Pre-load Settings*. Added 2026-09-28,
        // step 10c's ninth-pass fix piece 1: **this read was never taken**, so
        // such a job had no inventory in hand, `Download` J3 could judge no
        // frame, and every file waited for the ending (`Download` J8's rule for
        // a file with no frame). **The case the manifest read was written for
        // is a job pre-loaded before submitting; the case it now also meets is
        // every other job.**
        //
        // **Read at every pass until it lands, and kept once it has**, for the
        // attempt the note names. `Rendering` P1 writes it at step 4, after the
        // render start is learned, so a read taken before it lands finds
        // nothing — which is not a reading and is not kept, the manifest's own
        // rule above. **A new attempt's reading is read again** (`Job files
        // and outputs` R13 keeps one per attempt).
        let the_manifest_carries_no_reading = one
            .the_record_directory
            .as_ref()
            .is_some_and(|read| read.narrowing_material == TheNarrowingMaterial::ItHasNotLanded);
        if window_is_open
            && the_manifest_carries_no_reading
            && says.attempts > 0
            && one.the_recorded_reading.as_ref().map(|(attempt, _)| *attempt)
                != Some(says.attempts)
        {
            let mut reader = TheRecordDirectorysReadingRoute { sessions };
            if let Some(read) = read_the_recorded_reading_the_machine_wrote(
                &mut reader,
                paths.job_record_directory.as_bytes(),
                says.attempts,
                &the_job_output_folder,
            )
            .await
            {
                one.the_recorded_reading = Some((says.attempts, read));
            }
        }
        let the_job_output_folder = the_job_output_folder.into_bytes();

        // **`Download` J2: the attempt's ordered frame list is read from the
        // attempt record, for every attempt.** A range is never the order, so
        // nothing falls back to the manifest's range — and **an attempt count
        // of nought is not attempt one**: no note has arrived, the order is not
        // in hand, and mechanism two alone decides.
        //
        // **Read at every pass until it lands, and kept once it has.** Changed
        // 2026-09-28, step 10c's ninth-pass fix piece 1: the order was read
        // once, at the first pass after the render start was learned, and
        // marked read whatever came back. `Rendering` P1 writes the attempt
        // record at step 7, well after that pass, so the ninth pass read it
        // before it existed and kept *not in hand* for the whole connection —
        // which switched `Download` J3's live judgement off by itself.
        if window_is_open && says.attempts > 0 && one.the_order_was_read_for != Some(says.attempts)
        {
            let mut reader = TheRecordDirectorysReadingRoute { sessions };
            one.order = read_the_attempts_order(
                &mut reader,
                paths.job_record_directory.as_bytes(),
                says.attempts,
            )
            .await;
            if matches!(one.order, TheOrder::The(_)) {
                one.the_order_was_read_for = Some(says.attempts);
            }
        }

        // **`Download` N4, `Download` N5 and `Download` N6: the inventory cut
        // by this arrangement's own narrowing, in this document's own keys.**
        // **The reading in force is the attempt's own where the machine wrote
        // one, and the manifest's otherwise.** Where no reading landed there is
        // no inventory and both halves are empty — the honest state that
        // document names, and the walk's own files then come home as
        // `Download` J8's unmatched ones.
        let the_reading_in_force = match &one.the_recorded_reading {
            Some((attempt, read)) if *attempt == says.attempts => Some(read),
            _ => one.the_record_directory.as_ref(),
        };
        let cut = match the_reading_in_force
            .and_then(|read| read.inventory.as_ref().map(|inventory| (read, inventory)))
        {
            Some((read, inventory)) => what_the_narrowing_leaves(
                inventory,
                match &read.narrowing_material {
                    TheNarrowingMaterial::OnTheDesktop(reading) => &reading.destinations,
                    TheNarrowingMaterial::ItHasNotLanded => &[],
                },
                &arrangement.narrowing,
                &the_job_output_folder,
            ),
            None => WhatTheNarrowingLeaves::default(),
        };
        // **`Download` N6's whole, where this pass has the inventory in hand**:
        // the narrowed entries' count and the video statement, kept for the
        // run to read (`crate::servers::the_machines_work::TheJobsWhole`) —
        // the cut this pass already made, and no second count. Where no
        // reading has landed there is no whole, and the Transfers row keeps
        // the reckoning's length. Added 2026-09-29, step 10c's transfer wave
        // piece 4.
        if the_reading_in_force.is_some_and(|read| read.inventory.is_some()) {
            holds.wholes.insert(
                key.clone(),
                crate::servers::the_machines_work::TheJobsWhole {
                    files: cut.narrowed.len(),
                    it_may_grow: cut.it_may_grow,
                },
            );
        }

        // **`Download` N5a: what Smoozy placed, read once the ending is in
        // hand** — the ending walk is where the clause is asked, and the
        // machine reads the placed files again the moment the render ends,
        // before the ending reaches the desktop. Added 2026-10-02, the fixes
        // before 0.1.8, fault 3: until then this was handed nothing, *"no
        // producer anywhere in this workspace"*, and every copy came home.
        if window_is_open
            && says.ending.is_some()
            && says.attempts > 0
            && one.what_smoozy_placed.as_ref().map(|(attempt, _)| *attempt) != Some(says.attempts)
        {
            let mut reader = TheRecordDirectorysReadingRoute { sessions };
            if let Ok(read) = read_what_smoozy_placed(
                &mut reader,
                paths.job_record_directory.as_bytes(),
                says.attempts,
            )
            .await
            {
                one.what_smoozy_placed = Some((says.attempts, read));
            }
        }
        let placed = one
            .what_smoozy_placed
            .as_ref()
            .and_then(|(_, read)| read.as_ref());

        let window = TheWindow {
            the_desired_state_is_on: true,
            a_connection_to_the_server_stands,
            the_attempts_window_is_open: says.the_render_start_was_learned
                && !one.running.the_ending_walk_has_completed(),
        };
        one.running.the_saved_prefix_was_reported(
            holds.saved_prefixes.get(&key)
                .filter(|(attempt, _)| *attempt == says.attempts && one.the_order_was_read_for == Some(*attempt))
                .map(|(_, frame)| *frame),
        );
        let in_hand = WhatIsInHand {
            the_job_output_folder: &the_job_output_folder,
            narrowed_entries: &cut.narrowed,
            the_whole_may_grow: cut.it_may_grow,
            the_entries_the_narrowing_excludes: &cut.excluded,
            order: &one.order,
            the_jobs_label: says.label.as_deref(),
            the_first_frame_pair: says.pair.as_ref().map(|(the_check, the_range)| {
                smoozy_download::acting::running::TheFirstFramePair { the_check, the_range }
            }),
            the_pairs_other_half_fetched: &says.the_other_half_fetched,
            window,
            // **`Download` N5a's placed-copy list and the four values after
            // the render**, where Smoozy placed anything. Where the reading
            // after the render is not there, each placed path is one that
            // could not be judged and comes home — the clause's safe
            // direction.
            the_placed_copies: placed.map(|read| &read.list),
            the_four_values_of_the_placed_copies: placed
                .map(|read| read.four_values_now.as_slice())
                .unwrap_or(&[]),
            arrangements_walking_this_server: walking.max(1),
        };
        let came = crate::downloading::one_turn_of_an_arrangement(
            &mut one.running,
            sessions,
            record,
            &mut one.queue,
            &mut arrangement,
            in_hand,
        )
        .await;
        if the_run.an_auto_ending_walk_was_read(&key, generation, one.running.the_ending_walk_has_completed()) {
            // **`Download` R5's dated note of 2026-10-31: the completion is
            // written down when it happens**, against the arrangement's start.
            // A write the record would not take leaves it run-held, as before.
            if one.running.the_ending_walk_has_completed() && !says.the_ending_walk_is_written_down {
                let job = arrangement.job.clone();
                record.with_the_record(&mut |connection| {
                    let _ = DownloadArrangements::over(connection).the_ending_walk_completed(&job);
                });
            }
            record.a_transfer_moved();
        }
        #[cfg(test)]
        if matches!(&came, Ok(WhatTheTurnDid::ItWalked { .. } | WhatTheTurnDid::TheEndingWalkRan(_))) {
            tests::a_recovery_stage(&mut holds.recovery_stages, "complete walk");
        }
        // **`Download` W2's pause is the turn's own figure**, taken from the
        // walk that just
        // ran. A turn that did not walk — the window shut, the ending walk
        // already done, the arrangement done — offers none, and the pass keeps
        // the floor it started with.
        if let Ok(came) = came {
            let pause = match came {
                WhatTheTurnDid::ItWalked {
                    pause_before_the_next,
                    ..
                }
                | WhatTheTurnDid::TheWalkWasDiscarded {
                    pause_before_the_next,
                    ..
                } => Some(pause_before_the_next),
                WhatTheTurnDid::TheWindowIsShut
                | WhatTheTurnDid::TheEndingWalkRan(_)
                | WhatTheTurnDid::ItIsDone => None,
            };
            if let Some(pause) = pause {
                longest = longest.max(pause);
            }
        }
        walked.push(WhatOneArrangementWalkedWith {
            key,
            arrangement,
            cut,
            the_job_output_folder,
            label: says.label,
            pair: says.pair,
            the_other_half_fetched: says.the_other_half_fetched,
            window,
        });
    }

    // **`Download` E1 step 3, once every arrangement has walked: the owed
    // units come down inside the pause before the next walks.** Changed
    // 2026-09-28, step 10c's ninth-pass fix piece 1 — each turn used to drain
    // its own queue whole before the next arrangement walked, so one job's
    // long ending drain held every other job's walks and fetches (`Download`
    // W2 and W3). **The pause is still `Download` W2's own figure**, and what
    // the fetches leave of it is what the sequence waits.
    let mut in_hand_for_each: HashMap<&String, &mut WhatOneArrangementRunsOn> =
        holds.per_job.iter_mut().collect();
    let mut owing: Vec<crate::downloading::AnArrangementOwingFetches<'_>> =
        Vec::with_capacity(walked.len());
    for WhatOneArrangementWalkedWith {
        key,
        arrangement,
        cut,
        the_job_output_folder,
        label,
        pair,
        the_other_half_fetched,
        window,
    } in walked.iter_mut()
    {
        let Some(one) = in_hand_for_each.remove(key) else {
            continue;
        };
        let WhatOneArrangementRunsOn {
            running,
            queue,
            probing,
            order,
            ..
        } = one;
        owing.push(crate::downloading::AnArrangementOwingFetches {
            running,
            queue,
            probing,
            arrangement,
            in_hand: WhatIsInHand {
                the_job_output_folder,
                narrowed_entries: &cut.narrowed,
                the_whole_may_grow: cut.it_may_grow,
                the_entries_the_narrowing_excludes: &cut.excluded,
                order,
                the_jobs_label: label.as_deref(),
                the_first_frame_pair: pair.as_ref().map(|(the_check, the_range)| {
                    smoozy_download::acting::running::TheFirstFramePair { the_check, the_range }
                }),
                the_pairs_other_half_fetched: the_other_half_fetched,
                window: *window,
                the_placed_copies: None,
                the_four_values_of_the_placed_copies: &[],
                arrangements_walking_this_server: walking.max(1),
            },
        });
    }
    // **`Server Setup` R1b counts a transfer under way as live work**, and the
    // automatic downloads are under way from the moment a file is owed until
    // the last owed file has come down — across the pauses between passes, so
    // the swap never lands between two passes of an ending's downloads. Read
    // before the fetches and again after them. Added 2026-10-12.
    let they_owe = |owing: &[crate::downloading::AnArrangementOwingFetches<'_>]| {
        owing
            .iter()
            .any(|one| one.queue.owed() > 0 || one.queue.in_flight() > 0)
    };
    let this_server = server_row.as_text();
    if they_owe(&owing) {
        crate::servers::the_driver_swap::the_automatic_downloads_owe(&this_server, true);
    }
    let left = crate::downloading::the_owed_fetches_come_down(
        &mut owing, sessions, the_run, record, run, longest,
    )
    .await
    .what_is_left_of_the_pause;
    crate::servers::the_driver_swap::the_automatic_downloads_owe(&this_server, they_owe(&owing));
    left
}

/// **What one arrangement walked with in this pass**, kept for the fetches
/// that follow every walk of the pass.
struct WhatOneArrangementWalkedWith {
    key: String,
    arrangement: AnArrangement,
    cut: WhatTheNarrowingLeaves,
    the_job_output_folder: Vec<u8>,
    label: Option<String>,
    pair: Option<(Identity, Identity)>,
    the_other_half_fetched: Vec<Vec<u8>>,
    window: TheWindow,
}

/// **The first-frame pair a job is one of, check then range, as its local
/// record holds it** (`Queue & History` P5a) — nothing for any other job, or
/// for a record that will not read. Added 2026-10-31, the twenty-first notes,
/// note 6.
fn the_first_frame_pair_of(jobs: &JobRecords<'_>, job: &Identity) -> Option<(Identity, Identity)> {
    let purpose = jobs.first_frame_purpose(job).ok().flatten()?;
    let read = |text: &str| Identity::read_back(smoozy_state::identities::IdentityKind::Job, text);
    Some((read(&purpose.check_job)?, read(&purpose.range_job)?))
}

/// **The sequence, run until the connection it was started for ends.**
///
/// **Every wait in it is bounded and has a condition it reaches**: the pass
/// itself ends, and the pause before the next one is `Download` W2's own
/// figure. **It ends with the connection** — `it_is_live` is the same answer
/// `crate::servers::the_machines_work` asks, so a period that ends takes this
/// with it, at worst one pause later.
///
/// **A caller with no run to hand the wholes to** — a test driving the
/// sequence alone — gets [`this_connections_arrangements_are_driven_for_the_run`]
/// over a run of its own.
pub async fn this_connections_arrangements_are_driven<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    the_run: &crate::downloading::TheRunsTransfers,
    record: &dyn crate::downloading::TheRecordADownloadWrites,
    server_row: &Identity,
    layout: &TheLayout,
    run: &Identity,
    it_is_live: &dyn Fn() -> bool,
) {
    this_connections_arrangements_are_driven_for_the_run(
        sessions,
        the_run,
        record,
        server_row,
        layout,
        run,
        it_is_live,
        &crate::servers::the_machines_work::WhatThisRunHolds::default(),
    )
    .await;
}

/// **The same sequence, and after every pass each job's whole the pass read
/// is handed to the run** (`Download` N6), where the Transfers screen reads
/// it. This is what a period's downloads thread runs. Added 2026-09-29, step
/// 10c's transfer wave piece 4.
#[allow(clippy::too_many_arguments)]
pub async fn this_connections_arrangements_are_driven_for_the_run<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    the_run: &crate::downloading::TheRunsTransfers,
    record: &dyn crate::downloading::TheRecordADownloadWrites,
    server_row: &Identity,
    layout: &TheLayout,
    run: &Identity,
    it_is_live: &dyn Fn() -> bool,
    what_this_run_holds: &crate::servers::the_machines_work::WhatThisRunHolds,
) {
    let mut holds = TheArrangementsThisConnectionDrives::fresh();
    while it_is_live() {
        holds.saved_prefixes = what_this_run_holds.the_saved_prefixes_reported_on_this_connection(&server_row.as_text());
        let pause = one_pass_over_this_servers_arrangements(
            &mut holds, sessions, the_run, record, server_row, layout, run,
        )
        .await;
        holds.the_wholes_are_handed_to(what_this_run_holds);
        if !it_is_live() {
            return;
        }
        tokio::time::sleep(pause).await;
    }
}

/// **`Download` N2's and `Download` J2's reads of a job's record directory, over
/// `Server protocol and transfers` T5's per-server point.**
///
/// **Nobody is waiting on either of them**, so a user's own listing on the
/// Files screen goes first, every time (`Server protocol and transfers` §1.1
/// item 2) — the same standing the walk itself takes.
pub(crate) struct TheRecordDirectorysReadingRoute<'a, C: AHeldConnection> {
    pub(crate) sessions: &'a ThisServersSessions<C>,
}

impl<C: AHeldConnection> ReadsOneFileOnTheMachine for TheRecordDirectorysReadingRoute<'_, C> {
    /// **A point that lends nothing is the channel being gone**, which is the
    /// honest value: `Download`'s own reads turn it into *the order could not
    /// be read* and *the material has not landed*, both of which are states
    /// that document already has.
    async fn read_one_whole_file(&mut self, path: &[u8]) -> Result<Vec<u8>, SubsystemFailure> {
        let path = path.to_vec();
        self.sessions
            .a_reading(false, move |subsystem| {
                Box::pin(async move {
                    TheReadingRoute::on(subsystem)
                        .read_one_whole_file(&path)
                        .await
                })
            })
            .await
            .unwrap_or(Err(SubsystemFailure::TheChannelIsGone))
    }
}

/// **The arrangements are driven from the moment a connection is lent**, on a
/// thread of its own and for that connection alone.
///
/// **It runs on its own thread for the reason step 7 does**: every reading it
/// takes is served from the period's own thread, inside the wait for the
/// connection to end, so a reading asked from that thread would wait for
/// itself.
///
/// **It is not gated on the installation branch, and step 7 is.** `Download`
/// R12 says an arrangement resumes on **any** connection to its server, and a
/// machine that is not the one the job ran on refuses the listing rather than
/// answering it — a discarded walk, which `Download` W5 already makes harmless.
fn the_arrangements_are_driven<R: Runtime>(
    app: AppHandle<R>,
    server: String,
    sessions: ThisServersSessions,
    connection: u64,
) {
    let _ = std::thread::Builder::new()
        .name("smoozy downloads".to_string())
        .spawn(move || {
            let Some(periods) = app.try_state::<Arc<ThePeriodsThisRun>>() else {
                return;
            };
            let periods = periods.inner().clone();
            // **`Queue & History` X4: the owed erases, before the arrangements
            // are driven** — they run until the connection ends. The 38's
            // piece 16.
            the_owed_erases_are_sent_on(&app, &periods, &server, &sessions, connection);
            let Some(server_row) = Identity::read_back(IdentityKind::LocalServer, &server) else {
                return;
            };
            let Some(the_run) = app.try_state::<Arc<crate::downloading::TheRunsTransfers>>() else {
                return;
            };
            let the_run = Arc::clone(the_run.inner());
            let Some(row) = the_row_named(&app, &server).ok().flatten() else {
                return;
            };
            // **A server with no recorded placement has no layout**, so there
            // is no job record directory to read and nothing to walk.
            let Some(layout) = crate::servers::installation::the_layout_recorded_for(&row) else {
                return;
            };
            let Some(run) = app
                .try_state::<Mutex<Started>>()
                .and_then(|started| started.lock().ok().and_then(|started| started.run.clone()))
            else {
                return;
            };
            let record = TheRecordTheDownloadsWriteThrough { app: app.clone() };
            this_connection_drives_its_arrangements(
                periods.the_machines_work(),
                &server,
                connection,
                &sessions,
                &the_run,
                &record,
                &server_row,
                &layout,
                &run,
            );
        });
}

/// **`Queue & History` X4 at a connection: the owed erases, sent on the
/// arrangements' thread before they are driven** — every reading asked from
/// the period's own thread would wait for itself, and this thread is already
/// one of its own. The 38's piece 16.
///
/// **Only on the confirmed branch and the live connection**, as step 7: a
/// machine that is not this row's installation is not asked to erase
/// anything.
fn the_owed_erases_are_sent_on<R: Runtime>(
    app: &AppHandle<R>,
    periods: &ThePeriodsThisRun,
    server: &str,
    sessions: &ThisServersSessions,
    connection: u64,
) {
    let Some(read) = periods.what_the_last_connection_read(server) else {
        return;
    };
    if read.check != smoozy_state::identities::InstallationCheck::PresentAndEqualConfirmed
        || !read.on_the_connection_that_is_live
    {
        return;
    }
    let Some(row) = the_row_named(app, server).ok().flatten() else {
        return;
    };
    let crate::servers::installation::WhereThePackageAreaIs::At(package_area) =
        crate::servers::installation::where_the_package_area_is(&row)
    else {
        return;
    };
    let Some(started) = app.try_state::<Mutex<Started>>() else {
        return;
    };
    let record = crate::keys::TheRunsRecord {
        started: started.inner(),
    };
    let mut machine = crate::servers::the_machines_work::OverThisServersPoint {
        sessions: sessions.clone(),
        package_area,
    };
    let holds = periods.the_machines_work();
    let it_is_live = || sessions.a_connection_is_live() && holds.is_current(server, connection);
    the_owed_erases_are_sent(&record, &mut machine, &it_is_live);
    // `Rendering` S4b: debt recorded by a previous definitive resolution.
    // This branch has confirmed the installation on this live connection.
    let Some(server_row) = Identity::read_back(IdentityKind::LocalServer, server) else {
        return;
    };
    let Some(installation) = row
        .last_confirmed_installation_identity
        .as_deref()
        .and_then(|text| Identity::read_back(IdentityKind::Installation, text))
    else {
        return;
    };
    if !it_is_live() {
        return;
    }
    if let Ok(runtime) = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        let app = app.clone();
        let server = server.to_string();
        // Keep every removal on this one confirmed connection. A repair must
        // not move a later removal onto a different installation.
        let _ = runtime.block_on(sessions.a_reading(false, move |subsystem| {
            Box::pin(async move {
                let is_current = app
                    .try_state::<Arc<ThePeriodsThisRun>>()
                    .is_some_and(|periods| {
                        periods.the_machines_work().is_current(&server, connection)
                    });
                if !is_current {
                    return Err(
                        smoozy_server_protocol::subsystem::SubsystemFailure::TheChannelIsGone,
                    );
                }
                let record = crate::surface::render_settings::the_applications_record(app);
                let mut files = smoozy_server_setup::wiring::TheMachinesFiles::on(subsystem);
                Ok(
                    crate::submission_cleanup::pay(&record, &mut files, &server_row, &installation)
                        .await,
                )
            })
        }));
    }


}

/// **`Queue & History` X3 and X4, the desktop's half: for every deletion
/// still owing its machine half, the erase is sent, what each unit came to is
/// written, and the debt is marked paid only when every unit is done.**
/// Whose debts were paid is handed back.
///
/// **X3's record is opened before the ask**, durable with the enumeration and
/// no outcome, under the identity the request is sent with. **The local
/// record is never held across the ask.** Then, by what came back:
///
/// - ***Erased*** or ***not wholly erased***: each unit's outcome written,
///   and the flag set where every unit is done. A unit not done leaves the
///   debt standing, and **the next connection's erase is a new action under a
///   new identity** (`State, identity and recovery` §9.5).
/// - ***Nothing of it here***, **or a request the machine's program does not
///   answer, or one that never reached it**: no unit was attempted on this
///   machine, so the record is taken away again and the debt waits for a
///   machine holding that identity (`Queue & History` X4).
/// - **Anything else** — nothing correlated came back, or the outcome is not
///   recorded yet — leaves the record with no outcome, which is the truth:
///   nobody here knows what the machine did.
///
/// **Public so the desk can drive it over a real line** (the dead-line desk
/// coverage, 2026-10-19); the running application reaches it only through
/// the confirmed connection's own branch above.
pub fn the_owed_erases_are_sent(
    record: &dyn crate::keys::LendsTheRecord,
    machine: &mut dyn crate::servers::the_machines_work::AsksTheSupervisor,
    it_is_live: &dyn Fn() -> bool,
) -> Vec<Identity> {
    use crate::record::deletions::TheDeletions;
    use crate::record::outcome_records::OutcomeRecords;
    use crate::servers::the_machines_work::with_the_record;
    use smoozy_queue_history::acting::erase::{
        open_the_record, what_the_answer_says, write_what_the_machine_did,
    };
    use smoozy_server_protocol::asking_the_supervisor::WhatCameBack;
    use smoozy_server_protocol::framing::{Fields, Request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS};
    use smoozy_server_protocol::requests::{
        IT_WAS_ERASED, IT_WAS_NOT_WHOLLY_ERASED, NOTHING_OF_IT_IS_HERE, THE_ERASES_OUTCOME_FIELD,
        THE_ERASE_REQUEST, THE_JOB_FIELD, THE_UNITS_NOT_DONE_FIELD,
    };
    use smoozy_state::outcomes::Keeps;
    use smoozy_state::tombstones::MachineSideCopy;

    let owed = with_the_record(record, |connection| {
        TheDeletions::over(connection).the_jobs_still_owing_their_erase().ok()
    })
    .flatten()
    .unwrap_or_default();
    let mut paid = Vec::new();
    for job in owed {
        if !it_is_live() {
            break;
        }
        let Some(Some(identity)) = with_the_record(record, |connection| {
            let mut keeper = OutcomeRecords::over(connection);
            open_the_record(&mut keeper)
                .ok()
                .map(|opened| opened.identity().clone())
        }) else {
            continue;
        };
        let request = Request::speaking(THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, identity.clone(), THE_ERASE_REQUEST)
            .carrying(Fields::none().and(THE_JOB_FIELD, job.as_text()));
        let take_the_record_away = || {
            with_the_record(record, |connection| {
                let _ = OutcomeRecords::over(connection).the_user_removed_it(&identity);
            });
        };
        let answered = match machine.ask(&request) {
            WhatCameBack::ItAnswered(answered) => answered,
            WhatCameBack::ItWasNeverAsked => {
                take_the_record_away();
                continue;
            }
            _ => continue,
        };
        if answered.the_request_name_it_says_it_does_not_answer().is_some()
            || answered.the_protocol_version_it_says_it_does_not_speak().is_some()
        {
            take_the_record_away();
            continue;
        }
        let did = match answered.fields.text(THE_ERASES_OUTCOME_FIELD) {
            Some(word) if word == NOTHING_OF_IT_IS_HERE => {
                take_the_record_away();
                continue;
            }
            Some(word) if word == IT_WAS_ERASED || word == IT_WAS_NOT_WHOLLY_ERASED => {
                what_the_answer_says(answered.fields.text(THE_UNITS_NOT_DONE_FIELD).unwrap_or_default())
            }
            _ => continue,
        };
        let removed = with_the_record(record, |connection| {
            let mut keeper = OutcomeRecords::over(connection);
            match write_what_the_machine_did(&mut keeper, &identity, &did) {
                Ok(MachineSideCopy::Removed) => TheDeletions::over(connection)
                    .the_machine_side_copy_was_removed(&job)
                    .is_ok(),
                _ => false,
            }
        });
        if removed == Some(true) {
            paid.push(job);
        }
    }
    paid
}

/// **One connection drives this server's arrangements, once, until it ends** —
/// what [`the_arrangements_are_driven`]'s thread does, over any point, so the
/// whole of it is driven at the desk. **It answers whether it drove them**:
/// nothing where they are already driven on this connection.
///
/// **The guard is keyed on the connection's number**, and that number is the
/// run's own for the server (`WhatThisRunHolds::a_connection_begins`): a
/// second period's first connection is a new number, so the last period's
/// driving — which can still be asleep in `Download` W2's pause when the user
/// connects again — never holds the new connection's guard. Step 10c's
/// transfer wave piece 3.
#[allow(clippy::too_many_arguments)]
pub(crate) fn this_connection_drives_its_arrangements<C: AHeldConnection>(
    holds: &crate::servers::the_machines_work::WhatThisRunHolds,
    server: &str,
    connection: u64,
    sessions: &ThisServersSessions<C>,
    the_run: &crate::downloading::TheRunsTransfers,
    record: &dyn crate::downloading::TheRecordADownloadWrites,
    server_row: &Identity,
    layout: &TheLayout,
    run: &Identity,
) -> bool {
    let it_is_live = || sessions.a_connection_is_live() && holds.is_current(server, connection);
    let Ok(runtime) = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    else {
        return false;
    };
    // **Once per connection** — `Servers` Q19a's re-take hands the
    // arrangements a connection they may already be driven on.
    if !holds.the_arrangements_begin(server, connection) {
        return false;
    }
    runtime.block_on(this_connections_arrangements_are_driven_for_the_run(
        sessions,
        the_run,
        record,
        server_row,
        layout,
        run,
        &it_is_live,
        holds,
    ));
    holds.the_arrangements_ended(server, connection);
    true
}

/// **The record door a download's writes go through** — the application's own
/// record, the lock taken for each act and let go at once.
struct TheRecordTheDownloadsWriteThrough<R: Runtime> {
    app: AppHandle<R>,
}

impl<R: Runtime> crate::downloading::TheRecordADownloadWrites
    for TheRecordTheDownloadsWriteThrough<R>
{
    fn with_the_record(&self, act: &mut dyn FnMut(&rusqlite::Connection)) -> bool {
        let Some(started) = self.app.try_state::<Mutex<Started>>() else {
            return false;
        };
        let Ok(started) = started.lock() else {
            return false;
        };
        match started.record.as_ref() {
            Some(record) => {
                act(record.connection());
                true
            }
            None => false,
        }
    }

    /// **An auto-download's row moves as its files land** — the same one event
    /// a manual download's and an upload's doors post.
    fn a_transfer_moved(&self) {
        crate::surface::transfers::the_transfers_moved(&self.app);
    }
}

#[cfg(test)]
mod tests {
    pub(super) type RecoveryStages = Vec<(&'static str, std::time::Instant)>;

    pub(super) fn a_recovery_stage(stages: &mut RecoveryStages, name: &'static str) {
        stages.push((name, std::time::Instant::now()));
    }

    use super::*;
    use smoozy_servers::condition::{self, Menu, Presentation};
    use smoozy_servers::first_connection::NotLinuxOutcome;
    use smoozy_servers::key_moments::TheTwoAnswers;

    /// **The inverse is the inverse, over every combination `Servers` composes
    /// from.**
    ///
    /// **This is the guard against the fault this build stage keeps meeting** —
    /// a fact kept in two places. The run holds the observed value and whether
    /// a period stands; `Servers` composes the value out of four facts. If
    /// [`the_facts_behind`] and `condition::observed_value` ever disagree, the
    /// list would draw one thing and the module would mean another, and nothing
    /// would say so.
    #[test]
    fn what_the_run_reads_back_composes_the_same_value_the_module_did() {
        for a_period_stands in [false, true] {
            for the_primary_connection_is_live in [false, true] {
                for a_run_of_attempts_is_in_flight in [false, true] {
                    for the_restart_mark_stands in [false, true] {
                        let of = WhatIsTrueOfTheServer {
                            a_period_stands,
                            the_primary_connection_is_live,
                            a_run_of_attempts_is_in_flight,
                            // **A mark is period-held**, so a mark standing on
                            // no period is not a case this composes.
                            the_restart_mark_stands: a_period_stands && the_restart_mark_stands,
                        };
                        let published = condition::observed_value(of);
                        let read_back = the_facts_behind(
                            published,
                            a_period_stands,
                            the_restart_mark_stands,
                        );
                        assert_eq!(condition::observed_value(read_back), published);
                        assert_eq!(
                            condition::presentation(read_back),
                            condition::presentation(of)
                        );
                        assert_eq!(condition::menu(read_back), condition::menu(of));
                    }
                }
            }
        }
    }

    /// **`Servers` K8 and `Server Setup` T4: a server Smoozy is restarting
    /// reads *restarting*, and it is the standing period's own mark that says
    /// so.**
    ///
    /// **The register is what the screen asks**, so this drives the mark the
    /// way `crate::setting_up`'s restart step does — through the handle the
    /// period holds — rather than through the value the composition takes.
    #[test]
    fn a_server_smoozy_restarted_reads_restarting_off_its_periods_own_mark() {
        let register = a_register();
        let period = a_period("server-1");
        let mark = period.the_restart_mark();
        assert!(register.a_period_has_begun("server-1", &period));

        // Before the restart command leaves: an ordinary repairing server.
        assert_eq!(
            condition::presentation(register.what_is_true_of("server-1")),
            Presentation::Repairing
        );

        // **`Server Setup` T4's mark, set before the command leaves.**
        mark.set();
        assert_eq!(
            condition::presentation(register.what_is_true_of("server-1")),
            Presentation::Restarting
        );

        // **`Servers` M6 item 1: the machine answers and the mark clears**, so
        // the row stops reading *restarting* without anything being told twice.
        mark.cleared_by(smoozy_servers::restart::HowTheWaitEnds::TheMachineAnswers);
        assert_eq!(
            condition::presentation(register.what_is_true_of("server-1")),
            Presentation::Repairing
        );
    }

    /// **`Servers` D22 and S3: a run knows *disconnected* about every server
    /// until something says otherwise**, and nothing durable says otherwise.
    #[test]
    fn a_server_with_no_period_reads_disconnected_and_takes_the_disconnected_menu() {
        let of = the_facts_behind(ObservedValue::Connected, false, false);
        assert_eq!(condition::observed_value(of), ObservedValue::Disconnected);
        assert_eq!(condition::presentation(of), Presentation::Disconnected);
        assert_eq!(condition::menu(of), Menu::TheDisconnectedMenu);
    }

    /// **`Servers` K8a: a period standing with the connection down is
    /// *repairing*, and it takes the connected menu whose first slot reads
    /// disconnect** — which is `Servers` S10 item 1's promise, available for as
    /// long as a period stands.
    #[test]
    fn a_standing_period_with_the_connection_down_still_offers_the_way_out() {
        for value in [ObservedValue::Reconnecting, ObservedValue::Disconnected] {
            let of = the_facts_behind(value, true, false);
            assert_eq!(condition::presentation(of), Presentation::Repairing);
            assert_eq!(condition::menu(of), Menu::TheConnectedMenu);
        }
    }

    /// **`Servers` Q13 outcome 2 is sayable, and outcome 3 is not.**
    ///
    /// The clause promises that a machine naming something other than Linux is
    /// said to be **not a Linux machine**, and puts outcome 3 — a machine that
    /// answered nothing Smoozy could read — beside it as the one that claims
    /// nothing. **Two outcomes of one check, and telling them apart is the
    /// whole of the clause**, so this asserts both halves together: outcome 2
    /// reaches its own answer and outcome 3 reaches the claimless one.
    #[test]
    fn the_machine_that_named_something_else_is_told_apart_from_the_one_that_named_nothing() {
        assert_eq!(
            what_the_driving_left(
                HowTheDrivingEnded::TheOpeningAttemptStopped(WhereItStopped::TheNotLinuxCheck(
                    NotLinuxOutcome::ItNamedSomethingElse
                )),
            ),
            WhatTheDrivingLeft::ItIsNotALinuxMachine,
            "**`Servers` Q13 outcome 2**: *says plainly that it is not a Linux \
             machine*. The answer is composed from the attempt's own step and \
             from nothing else."
        );
        assert_eq!(
            what_the_driving_left(
                HowTheDrivingEnded::TheOpeningAttemptStopped(WhereItStopped::TheNotLinuxCheck(
                    NotLinuxOutcome::NothingSmoozyCouldRead
                )),
            ),
            WhatTheDrivingLeft::TheConnectionWasNotMade,
            "**`Servers` Q13 outcome 3 claims nothing about what the machine \
             is**, and saying outcome 2's sentence here would be Smoozy \
             claiming what it does not know."
        );
    }

    /// **Seven of the other eight reasons still answer with the one claimless
    /// sentence**, which `Preparation/Specs/Whole app.md` requires: Smoozy
    /// cannot tell a network problem from a machine switched off from a
    /// machine that no longer exists. **The eighth, `Servers` Q6's third path,
    /// is answered by the changed-identity warning** and by no sentence beside
    /// it — [`a_changed_identity_is_answered_by_its_warning_alone`].
    ///
    /// **The list is written out rather than walked, and that is deliberate.**
    /// A ninth reason arriving in `Servers`' own enum must be met here by
    /// somebody deciding what Smoozy says about it, not swept into the
    /// claimless sentence by a loop that never fails.
    #[test]
    fn every_other_reason_an_attempt_stopped_says_only_that_the_connection_was_not_made() {
        let answers = TheTwoAnswers {
            the_key_is_usable: false,
            a_passphrase_is_available_without_asking: false,
        };
        for stopped in [
            WhereItStopped::TheKeyCheck(answers),
            WhereItStopped::NoCredentialWasSupplied,
            WhereItStopped::TheAddressDidNotAnswer,
            WhereItStopped::TheExchangeDidNotComplete,
            WhereItStopped::TheKeyCouldNotBeLoaded,
            WhereItStopped::TheMachineRefusedTheCredential,
            WhereItStopped::TheConnectDeadlineExpired,
            WhereItStopped::TheNotLinuxCheck(NotLinuxOutcome::NothingSmoozyCouldRead),
        ] {
            assert_eq!(
                what_the_driving_left(HowTheDrivingEnded::TheOpeningAttemptStopped(stopped)),
                WhatTheDrivingLeft::TheConnectionWasNotMade,
                "{stopped:?} is not `Servers` Q13 outcome 2, so Smoozy says the \
                 connection was not made and claims nothing about why"
            );
        }
    }

    /// **`Servers` Q6's third path is answered by its warning alone.** The
    /// warning was published while the attempt ran; a second sentence saying
    /// the connection was not made would be Smoozy answering one act twice.
    #[test]
    fn a_changed_identity_is_answered_by_its_warning_alone() {
        assert_eq!(
            what_the_driving_left(HowTheDrivingEnded::TheOpeningAttemptStopped(
                WhereItStopped::TheIdentityDiffers
            )),
            WhatTheDrivingLeft::TheWarningIsTheAnswer
        );
    }

    /// **`Servers` S10's six endings, and which of them Smoozy says anything
    /// about.**
    #[test]
    fn only_a_connection_that_never_became_live_is_answered() {
        assert_eq!(
            what_the_driving_left(
                HowTheDrivingEnded::ThePeriodEnded(
                    PeriodEnding::TheOpeningConnectionNeverBecameLive
                ),
            ),
            WhatTheDrivingLeft::TheConnectionWasNotMade
        );
        assert_eq!(
            what_the_driving_left(HowTheDrivingEnded::TheRepairStoppedForGood),
            WhatTheDrivingLeft::TheRepairStoppedForGood
        );
        for ending in PeriodEnding::ALL {
            if matches!(ending, PeriodEnding::TheOpeningConnectionNeverBecameLive) {
                continue;
            }
            assert_eq!(
                what_the_driving_left(HowTheDrivingEnded::ThePeriodEnded(ending)),
                WhatTheDrivingLeft::NothingToSay,
                "**`Servers` S10 item {ending:?}**: the user asked for the \
                 period to end, so there is nothing to answer"
            );
        }
    }

    // ------------------------------------------------- the register itself

    /// A register with nothing behind it — no record, no shell — which is
    /// enough to drive `Servers` S10's endings.
    fn a_register() -> ThePeriodsThisRun {
        ThePeriodsThisRun::over(
            TheRunThroughTheApp::lending_nothing(),
            Arc::new(crate::events::TheOneQueue::new()),
            Arc::new(TheNotificationService::on(Box::new(NothingIsPosted))),
        )
    }

    struct NothingIsPosted;

    impl crate::notifications::ThePlatform for NothingIsPosted {
        fn the_machinery(&self) -> crate::notifications::TheMachinery {
            crate::notifications::TheMachinery::ItIsInPlace
        }
        fn the_permission(&self) -> crate::notifications::permission::WhatTheSystemSays {
            crate::notifications::permission::WhatTheSystemSays::ItIsGranted
        }
        fn ask_for_permission_now(&self) {}
        fn post(&self, _post: &crate::notifications::APost) {}
        fn take_down(&self, _key: &crate::presentation::the_presentation_key::ThePresentationKey) {}
    }

    fn a_period(server: &str) -> APeriod {
        APeriod::begun(
            server,
            OpeningAct::TheUsersConnectAction,
            WhoTookIt::TheUser,
            SystemTime::UNIX_EPOCH,
        )
    }

    /// **`Servers` S2: a period is per server, and one server's period touches
    /// no other's.** Nothing holds a count of periods or a global connection
    /// state, and a second act on a server whose period stands begins nothing.
    #[test]
    fn a_period_is_per_server_and_a_second_act_begins_no_second_one() {
        let register = a_register();
        let one = a_period("server-1");
        let two = a_period("server-2");
        assert!(register.a_period_has_begun("server-1", &one));
        assert!(register.a_period_has_begun("server-2", &two));
        let again = a_period("server-1");
        assert!(
            !register.a_period_has_begun("server-1", &again),
            "**`Servers` S5's list of period-opening acts is closed and a \
             period is per server**, so a second press is not a second period"
        );
        assert!(register.a_period_stands_for("server-1"));
        assert!(register.a_period_stands_for("server-2"));
    }

    /// **`Servers` S10 items 1, 3 and 6 all reach a standing period from
    /// outside the thread driving it**, and each says whether there was one to
    /// end.
    #[test]
    fn the_four_endings_that_arrive_from_outside_reach_a_standing_period() {
        for (name, end_it) in [
            (
                "the user disconnects",
                Box::new(|register: &ThePeriodsThisRun, server: &str| {
                    register.the_user_disconnects(server)
                }) as Box<dyn Fn(&ThePeriodsThisRun, &str) -> bool>,
            ),
            (
                "the edit's save",
                Box::new(|register: &ThePeriodsThisRun, server: &str| {
                    register.the_edits_save_ends_it(server).is_some()
                }),
            ),
            (
                "the row is removed",
                Box::new(|register: &ThePeriodsThisRun, server: &str| {
                    register.the_row_was_forgotten(server)
                }),
            ),
        ] {
            let register = a_register();
            let period = a_period("server-1");
            let ended = period.end_it();
            assert!(
                !end_it(&register, "server-1"),
                "{name}: **`Servers` N13: a server no period stands for is \
                 repaired by nothing**, and there is nothing to end"
            );
            register.a_period_has_begun("server-1", &period);
            assert!(end_it(&register, "server-1"), "{name}");
            assert!(
                ended.has_ended().is_some(),
                "{name}: the driving is told, which is what stops the repair"
            );
        }
    }

    /// **`Servers` S10 item 2: the application run ends, however it ends**, and
    /// every period ends with it.
    #[test]
    fn the_run_ending_ends_every_period_there_is() {
        let register = a_register();
        let one = a_period("server-1");
        let two = a_period("server-2");
        let (first, second) = (one.end_it(), two.end_it());
        register.a_period_has_begun("server-1", &one);
        register.a_period_has_begun("server-2", &two);
        register.the_application_run_ends();
        assert_eq!(first.has_ended(), Some(PeriodEnding::TheApplicationRunEnds));
        assert_eq!(
            second.has_ended(),
            Some(PeriodEnding::TheApplicationRunEnds)
        );
    }

    /// **`Servers` S11: every period-held thing ends the moment the period
    /// ends**, and the register's own entry goes with it — so the screen reads
    /// *no period stands* rather than one that is no longer being driven.
    #[test]
    fn a_period_that_has_ended_is_no_longer_one_the_run_holds() {
        let register = a_register();
        let period = a_period("server-1");
        register.a_period_has_begun("server-1", &period);
        register.a_period_has_ended("server-1");
        assert!(!register.a_period_stands_for("server-1"));
        assert_eq!(
            condition::observed_value(register.what_is_true_of("server-1")),
            ObservedValue::Disconnected
        );
    }

    /// **The join between the driving and the screen, in one test.**
    ///
    /// A period's acting half publishes through the run's one publisher, and
    /// the screen reads what is true back off the same one. **If the two ever
    /// stop being the same value the row would draw one thing while the module
    /// meant another**, which is the fault this build stage keeps meeting.
    #[test]
    fn what_a_period_publishes_is_what_the_screen_reads_back() {
        let register = a_register();
        let period = a_period("server-1");
        register.a_period_has_begun("server-1", &period);

        let mut publishes = register.where_a_period_publishes();
        publishes.the_observed_value("server-1", ObservedValue::Connected);
        assert_eq!(
            condition::observed_value(register.what_is_true_of("server-1")),
            ObservedValue::Connected
        );
        assert_eq!(
            condition::presentation(register.what_is_true_of("server-1")),
            Presentation::Connected
        );

        publishes.the_observed_value("server-1", ObservedValue::Reconnecting);
        assert_eq!(
            condition::presentation(register.what_is_true_of("server-1")),
            Presentation::Repairing
        );

        // **`Servers` N10a's moment travels the same way**, and *absent* is a
        // value rather than a gap.
        assert_eq!(
            register.the_next_attempt_due_for("server-1"),
            NextAttemptDue::Absent
        );
        publishes.the_next_attempt_due("server-1", NextAttemptDue::At(SystemTime::UNIX_EPOCH));
        assert_eq!(
            register.the_next_attempt_due_for("server-1"),
            NextAttemptDue::At(SystemTime::UNIX_EPOCH)
        );
    }

    /// **Step 10b's first audit round, item 2: the two locks are taken in one
    /// order.** A period publishing used to hold the publisher's lock and then
    /// take the application's, through the lend; a screen command held the
    /// application's and then read the publisher. **This holds the application's
    /// lock, has a publish wait for it inside its lend, and then reads what is
    /// true of the server through the other path** — which must answer rather
    /// than wait. Before the repair it waited for good.
    #[test]
    fn a_publish_waiting_for_the_applications_lock_holds_no_lock_a_reader_needs() {
        use std::sync::mpsc;
        use std::time::Duration;

        let the_applications_lock = Arc::new(Mutex::new(()));
        let (inside_the_lend, it_is_inside) = mpsc::channel::<()>();
        let reach_lock = Arc::clone(&the_applications_lock);
        let the_run = TheRunThroughTheApp {
            reach: Box::new(move |_act| {
                let _ = inside_the_lend.send(());
                let _held = reach_lock.lock();
            }),
        };
        let register = Arc::new(ThePeriodsThisRun::over(
            the_run,
            Arc::new(crate::events::TheOneQueue::new()),
            Arc::new(TheNotificationService::on(Box::new(NothingIsPosted))),
        ));
        let server = smoozy_state::identities::Identity::issue(
            smoozy_state::identities::IdentityKind::LocalServer,
        )
        .expect("randomness")
        .as_text();

        let held = the_applications_lock.lock().expect("the application's lock");
        let publishing = {
            let register = Arc::clone(&register);
            let server = server.clone();
            std::thread::spawn(move || {
                let mut publishes = register.where_a_period_publishes();
                publishes.the_observed_value(&server, ObservedValue::Connected);
            })
        };
        it_is_inside
            .recv_timeout(Duration::from_secs(10))
            .expect("the publish reached its lend");

        let (answered, the_answer) = mpsc::channel();
        {
            let register = Arc::clone(&register);
            let server = server.clone();
            std::thread::spawn(move || {
                let _ = answered.send(register.what_is_true_of(&server));
            });
        }
        let read = the_answer.recv_timeout(Duration::from_secs(5));
        drop(held);
        publishing.join().expect("the publish ended");
        assert!(
            read.is_ok(),
            "**the reader waited on a lock the waiting publish held** — the two \
             locks are taken in opposite orders and the window can freeze"
        );
    }

    /// **Step 10b's first audit round, item 5: the put-back is owed only where a
    /// period stood** (`Servers` E1), and **the old period's own thread learns
    /// whether the save landed**, after it has left the register.
    #[test]
    fn an_edits_save_owes_the_put_back_only_where_a_period_stood() {
        let register = a_register();
        assert!(
            register.the_edits_save_ends_it("server-1").is_none(),
            "**`Servers` E1: a *connected* server's save**. An edit to a server \
             no period stands for owes no put-back, so nothing connects it"
        );

        let period = a_period("server-1");
        let ended = period.end_it();
        register.a_period_has_begun("server-1", &period);
        let owed = register
            .the_edits_save_ends_it("server-1")
            .expect("a standing period's edit owes the put-back");
        assert_eq!(ended.has_ended(), Some(PeriodEnding::TheEditsSave));
        let told = register
            .a_period_has_ended("server-1")
            .expect("the period's own thread is handed what the save says");
        assert!(!register.a_period_stands_for("server-1"));
        owed.the_save_landed(true);
        assert_eq!(told.recv(), Ok(true));

        // **A save that did not say — the command ended first — is a save
        // that did not land**, and no put-back follows it.
        let again = a_period("server-2");
        register.a_period_has_begun("server-2", &again);
        let owed = register.the_edits_save_ends_it("server-2").expect("owed");
        let told = register.a_period_has_ended("server-2").expect("handed over");
        drop(owed);
        assert!(told.recv().is_err());
    }

    /// **The edit's command opens no put-back of its own** — the old period's
    /// thread does, after it has left the register (step 10b's first audit
    /// round, item 5). Asserted over the command's own text because what is
    /// held is an absence.
    #[test]
    fn the_edits_command_opens_no_period_itself() {
        let text = include_str!("../surface/servers.rs");
        let at = text.find("pub fn an_edit_is_saved<").expect("the command");
        let body = &text[at..at + text[at..].find("\n}\n").expect("it closes")];
        assert!(
            !body.contains("a_period_is_opened"),
            "the edit's save opens the put-back itself, racing the old period"
        );
        assert!(body.contains("the_save_landed"));
    }

    /// **Step 10b's first audit round, items 4 and 8: the desktop's side of the
    /// driving.** Each attempt reads the row as the record holds it then, and
    /// marks what the last connection read as no longer the live connection's;
    /// what a live connection read is kept — the fresh reading written to the
    /// row whole (`Servers` H4), the branch held for the run — and where no
    /// reading came back, the row is left exactly as it was (`Servers` H21).
    #[test]
    fn the_desktops_side_reads_the_row_at_each_attempt_and_keeps_what_a_connection_read() {
        use smoozy_servers::hardware::{Cards, HardwareReading, MachineMemory};
        use smoozy_servers::identity::{Host, KeyReference, Port, Username};
        use smoozy_state::identities::{Identity, IdentityKind, InstallationCheck};

        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-the-desktops-side-{}.sqlite3", std::process::id()));
        let _ = std::fs::remove_file(&path);
        let record = crate::record::LocalRecord::open_at(&path).expect("a record");
        let server = Identity::issue(IdentityKind::LocalServer)
            .expect("randomness")
            .as_text();
        let row = ServerRow::saved(
            server.clone(),
            Host::what_the_user_typed("198.51.100.10").expect("a host"),
            Port::STANDARD_SSH,
            Username::what_the_user_typed("root").expect("a username"),
            KeyReference {
                local_key_identifier: "key-1".into(),
                fingerprint: "SHA256:aaa".into(),
            },
        );
        server_rows::add(record.connection(), &row).expect("the row");
        let lent = crate::keys::TheRecordItself::over(record.connection());
        let register = a_register();
        let mut desk = WhatTheDesktopKeeps {
            record: &lent,
            periods: &register,
        };

        assert_eq!(desk.the_row_now(&server), Some(row.clone()));

        let reading = HardwareReading::taken(
            Cards::TheCards(Vec::new()),
            MachineMemory::TheAllocation(16 * 1024 * 1024 * 1024),
            None,
            SystemTime::UNIX_EPOCH,
            &server,
            SystemTime::UNIX_EPOCH,
            None,
        );
        desk.the_connection_read(&server, InstallationCheck::Absent, Some(reading.clone()));
        let kept = server_rows::one(record.connection(), &server)
            .expect("the record reads")
            .expect("the row");
        assert_eq!(kept.remembered_hardware_reading, Some(reading.clone()));
        assert_eq!(
            register.what_the_last_connection_read(&server),
            Some(WhatTheLastConnectionRead {
                check: InstallationCheck::Absent,
                a_fresh_reading_was_kept: true,
                on_the_connection_that_is_live: true,
            })
        );

        // **Another attempt begins**: what the last connection read is no
        // longer the live connection's own.
        let _ = desk.the_row_now(&server);
        assert!(!register
            .what_the_last_connection_read(&server)
            .expect("still held")
            .on_the_connection_that_is_live);

        // **The fourth branch takes no reading and changes nothing**.
        desk.the_connection_read(
            &server,
            InstallationCheck::PresentAndTheSupervisorDidNotAnswer,
            None,
        );
        let unchanged = server_rows::one(record.connection(), &server)
            .expect("the record reads")
            .expect("the row");
        assert_eq!(unchanged.remembered_hardware_reading, Some(reading));
        assert!(!register
            .what_the_last_connection_read(&server)
            .expect("held")
            .a_fresh_reading_was_kept);

        drop(record);
        for beside in ["", "-wal", "-shm"] {
            let mut name = path.clone().into_os_string();
            name.push(beside);
            let _ = std::fs::remove_file(std::path::PathBuf::from(name));
        }
    }

    /// **Step 10b's first audit round, items 15 and 16: a connect whose driving
    /// could not begin says so**, and the passphrase channel is never opened
    /// in the shared temporary directory. Asserted over this file's text,
    /// because what is held is an absence: the thread's body answers nothing
    /// by returning, and whatever did not begin reaches the claimless sentence.
    #[test]
    fn a_connect_that_could_not_begin_says_so_rather_than_returning_in_silence() {
        let working = the_working_half();
        let at = working
            .find("fn the_period_is_driven<")
            .expect("the thread's body");
        let body = &working[at..at + working[at..].find("\n}\n").expect("it closes")];
        assert!(
            !body.contains("return;"),
            "the thread's body returns in silence"
        );
        assert!(body.contains("-> Option<TheDrivingsOutcome>"));
        assert!(body.contains("smoozys_own_temporary_area()?"));
        let said = working
            .find("fn what_the_period_left_is_said<")
            .expect("what is said");
        let said = &working[said..said + working[said..].find("\n}\n").expect("it closes")];
        assert!(said.contains("let Some(outcome) = outcome else"));
        assert!(said.contains("the_connection_was_not_made(app, server)"));
    }

    /// **The runner's R5: a queue decline ends with its period.** The call
    /// sits beside the register's own ending, in the one place every period
    /// ends; `the_queue`'s own test drives what it does.
    #[test]
    fn a_period_that_ends_ends_the_queues_decline_for_its_server() {
        let working = the_working_half();
        let at = working
            .find("periods.a_period_has_ended(&server);")
            .expect("the register's ending");
        let beside = &working[at..at + 200];
        assert!(beside.contains("crate::surface::the_queue::a_connected_period_ended(&server);"));
    }

    /// **Step 10c's transfer wave piece 6: a period's end is told to the run's
    /// register of transfers**, in the same one place every period ends, so a
    /// request waiting for this period's connection ends with it (`File
    /// Browser` U7's paragraph of 2026-09-28, `Download` M13's).
    /// `crate::downloading`'s own tests drive what the register does with it.
    #[test]
    fn a_period_that_ends_ends_its_servers_transfers() {
        let working = the_working_half();
        let at = working
            .find("periods.a_period_has_ended(&server);")
            .expect("the register's ending");
        let the_thread = &working[at..at + working[at..].find("});").expect("the thread ends")];
        assert!(the_thread.contains("try_state::<Arc<crate::downloading::TheRunsTransfers>>()"));
        assert!(the_thread.contains("transfers.a_servers_period_ended(&server);"));
    }

    /// **Step 10b's first audit round, item 3: the box's *Connect* is the
    /// connect it was raised for**, opened with what was typed carried to it,
    /// and *Cancel* is answered. Asserted over the surface's text, because the
    /// command needs a running shell to be called at all; the route below the
    /// command is driven end to end in `tests/the_credential_for_a_connection.rs`.
    #[test]
    fn the_boxs_connect_opens_the_connect_it_was_raised_for() {
        let keys = include_str!("../surface/keys.rs");
        let keys = keys.split("#[cfg(test)]").next().expect("the working half");
        let at = keys
            .find("pub(crate) fn the_box_opens_its_connect<")
            .expect("the box's connect");
        let body = &keys[at..at + keys[at..].find("\n}\n").expect("it closes")];
        assert!(body.contains("a_period_is_opened_carrying("));
        assert!(body.contains("Some(typed)"));
        let answered = keys
            .find("pub fn the_passphrase_box_was_answered<")
            .expect("the command");
        let answered = &keys[answered..answered + keys[answered..].find("\n}\n").expect("it closes")];
        assert!(answered.contains("the_box_opens_its_connect("));
        assert!(
            !answered.contains("take_what_the_user_typed"),
            "the box puts what was typed into the store and connects nothing"
        );
        assert!(keys.contains("pub fn the_passphrase_box_was_cancelled<"));
        let page = include_str!("../../web/keys.js");
        assert!(page.contains("\"the_passphrase_box_was_cancelled\""));
        assert!(page.contains("server,\n    });"));

        // **`Keys` S17 on *Connect* with nothing typed, and Return in the box**
        // — the Gemini catch-up of step 10a, Area 5 (F-3, F-4). Before
        // 2026-09-19 an empty *Connect* closed the box and returned without
        // reporting the connect as one that did not happen, and the field had
        // no key handler, so Return did nothing.
        let connect = page
            .split_once("box.connect.onclick = () => {")
            .expect("the box's Connect")
            .1;
        let connect = &connect[..connect.find("\n  };").expect("it closes")];
        let (empty, _) = connect
            .split_once("box.field.value = \"\";")
            .expect("the field is cleared on a value");
        assert!(
            empty.contains("if (!typed) {") && empty.contains("box.cancel.onclick();"),
            "an empty Connect does not do what Cancel does, so the connect ends \
             in silence. `Keys` S17: the connect is reported as one that did not \
             happen"
        );
        let enter = page
            .split_once("box.field.onkeydown = (event) => {")
            .expect("Return in the passphrase field is handled")
            .1;
        let enter = &enter[..enter.find("\n  };").expect("it closes")];
        assert!(enter.contains("event.key !== \"Enter\""));
        assert!(
            enter.contains("box.connect.onclick();"),
            "Return does not press Connect through the same handler"
        );
    }

    /// **The decision this file takes, measured rather than argued: the shell's
    /// own runtime carries the clock and the reactor a period needs.**
    ///
    /// **This is the one thing the route could have been wrong about.**
    /// `Servers`' wiring waits on a clock and opens sockets on a reactor, and a
    /// runtime built without either gives a period that panics the first time
    /// it pauses. **The shell builds its own runtime and this crate does not
    /// choose its parts**, so the only honest answer is to ask it.
    ///
    /// **Both halves are asked through `Servers`' own two real ends**, so what
    /// is proved is what a period actually does: the desktop's clock waits, and
    /// the transport's connect deadline arrives. The address is a documented
    /// test-net one that nothing routes to, so nothing leaves this computer.
    #[test]
    fn the_shells_own_runtime_carries_the_clock_and_the_reactor_a_period_needs() {
        use smoozy_server_protocol::transport::{
            Credential, DecideHostTrust, Endpoint, HostTrust, OpenFailure,
        };
        use smoozy_servers::acting::{OpenConnections, Wait};

        struct Refuses;
        impl DecideHostTrust for Refuses {
            fn decide(&mut self, _offered: &[u8]) -> HostTrust {
                HostTrust::Stop
            }
        }

        // **On a thread of its own, which is where a period runs** (P5).
        let on_its_own = std::thread::spawn(|| {
            tauri::async_runtime::block_on(async {
                let mut clock = TheDesktopsClock;
                clock.until(std::time::Duration::from_millis(1)).await;
                let mut transport = TheTransport;
                transport
                    .open(
                        Endpoint {
                            host: "192.0.2.1".to_string(),
                            port: 22,
                            username: "nobody".to_string(),
                        },
                        Credential {
                            private_key_file: std::path::PathBuf::from("/nowhere/id_ed25519"),
                            passphrase: None,
                        },
                        Box::new(Refuses),
                        std::time::Duration::from_millis(50),
                    )
                    .await
                    .err()
            })
        });
        let failure = on_its_own.join().expect("the period's thread");
        assert!(
            matches!(failure, Some(OpenFailure::TheDeadlineExpired)),
            "**the shell's runtime has no clock or no reactor**, and a period \
             run on it would fail at its first pause or its first attempt. \
             This file's head takes the route on the strength of this"
        );
    }

    /// **P5, and it is asserted over this file's own text because what is being
    /// held is an absence.**
    ///
    /// A Tauri command that is not `async` runs on the thread that draws, so a
    /// period started inside one would be a connection, a wait and a repair on
    /// the drawing thread. **Every route into a period goes through
    /// `std::thread::spawn` first**, and `block_on` is reached only from inside
    /// one — so a `block_on` that is not behind a spawn is the fault P5 names.
    ///
    /// **Two threads and not one, from step 10c piece 2**, and the second is
    /// held as tightly as the first. `Servers` S14's additional connection is
    /// made on a thread of its own and blocked on inside it, and **it is asked
    /// for only from a period's own thread**: the one call that builds the
    /// route to it sits inside the period's driving, before that driving's own
    /// wait. So neither thread can be started from the drawing.
    ///
    /// **Three from step 10c's two callers**, and the third is held the same
    /// way: the arrangements a connection drives run on a thread of their own,
    /// for the reason step 7's do — every reading they take is served from the
    /// period's own thread, so one asked from that thread would wait for
    /// itself.
    ///
    /// **Four from step 10c's eighth-pass fix piece 3**, and the fourth is held
    /// the same way: `Servers` Q19a's re-take at the placement's commit runs on
    /// a thread of its own for the same reason, and it is started only from
    /// the ready action's own thread, which is never the drawing one.
    #[test]
    fn nothing_in_this_file_drives_a_period_on_the_thread_that_draws() {
        let working = the_working_half();
        let working = working.as_str();
        // **A thread of its own, however it is started** — the plain spawn and
        // the named one are the same act, and counting only the first would let
        // the second put a wait on the drawing thread.
        let mut spawns: Vec<usize> = working
            .match_indices("std::thread::spawn")
            .chain(working.match_indices("std::thread::Builder::new()"))
            .map(|(at, _)| at)
            .collect();
        spawns.sort_unstable();
        let blocks: Vec<usize> = working
            .match_indices("block_on")
            .map(|(at, _)| at)
            .collect();
        assert_eq!(
            spawns.len(),
            4,
            "**P5: one thread draws the interface and nothing that can block \
             runs on it.** This file opens a period on a thread of its own and \
             in exactly one place, the additional connection on another, the \
             arrangements a connection drives on a third, and the re-take at a \
             placement's commit on a fourth; a fifth route is a fifth chance to \
             open one where the drawing is"
        );
        assert_eq!(
            blocks.len(),
            5,
            "**P5**: cleanup and downloads wait on the same fourth thread"
        );
        for (which, start) in spawns.iter().enumerate() {
            let end = spawns.get(which + 1).copied().unwrap_or(working.len());
            let waits = blocks.iter().filter(|block| start < *block && **block < end).count();
            assert_eq!(waits, [1, 1, 1, 2][which],
                "**P5**: every wait follows its thread's start; the downloads thread also clears submission material");
        }
        // **The additional connection's route is built on a period's own
        // thread and nowhere else.**
        let the_calls: Vec<usize> = working
            .match_indices("the_additional_connection_for")
            .map(|(at, _)| at)
            .filter(|at| !working[..*at].ends_with("fn "))
            .collect();
        assert_eq!(the_calls.len(), 1, "one caller and no other");
        let driven_from = working
            .find("fn the_period_is_driven")
            .expect("the period's own driving");
        assert!(
            driven_from < the_calls[0] && the_calls[0] < blocks[0],
            "**P5**: the route to the additional connection is built inside the \
             period's own driving, on the period's own thread"
        );
    }

    /// **P10, P10a and P10b: a period's waits are the repair's own figures
    /// inside a sequence that is already running, and this file adds no timer
    /// of its own.**
    ///
    /// *"There is exactly one timer on this side and it is the update check."*
    /// **A period is not one**: a declared act started it, it ends when the
    /// period ends, and nothing outside the period outlives it. **What would
    /// make it one is a schedule here**, so there is none — no interval, no
    /// sleep and no figure.
    #[test]
    fn this_file_holds_no_timer_and_no_figure_of_its_own() {
        let working = the_working_half();
        let working = working.as_str();
        for reached_for in [
            "Duration::from_",
            "thread::sleep",
            "Instant",
            "set_interval",
            "recv_timeout",
        ] {
            assert!(
                !working.contains(reached_for),
                "this file names {reached_for}. **P10: there is exactly one \
                 timer on this side and it is the update check**, and every \
                 figure a period waits on is `Servers`' own — its repair's two \
                 pauses and its connect deadline, none of which is restated \
                 here"
            );
        }
    }

    /// **This file's working half, past its comments.**
    ///
    /// The same reason `src/presentation/mod.rs` has one: a test that forbids a
    /// word has to write the word down, and so does the paragraph explaining
    /// why it is forbidden. **A sweep that could not tell a prohibition from an
    /// instance would be one nobody could write the prohibition beside.**
    fn the_working_half() -> String {
        include_str!("a_period.rs")
            .split("\nmod tests {")
            .next()
            .expect("the file has a working half")
            .lines()
            .filter(|line| {
                let line = line.trim_start();
                !line.starts_with("//")
            })
            .collect::<Vec<&str>>()
            .join("\n")
    }

    /// **The Creator's ruling C1 of 2026-09-18, and step 10c piece 7 is its
    /// first caller.** `State, identity and recovery` V1 step 1: where the
    /// installation identity is absent or different, every record of the
    /// server whose render start was never learned is marked *start never
    /// heard* and enters history. **The confirmed branch and the fourth branch
    /// mark nothing** — the fourth stops at step 1 with nothing discarded.
    #[test]
    fn an_installation_found_gone_marks_the_jobs_smoozy_never_heard_start() {
        use smoozy_state::identities::{Identity, IdentityKind, InstallationCheck};

        struct ARecordInHand(rusqlite::Connection);
        impl crate::keys::LendsTheRecord for ARecordInHand {
            fn lend(&self, act: &mut dyn FnMut(&rusqlite::Connection)) {
                act(&self.0)
            }
            fn write_a_diary_line(&self, _act: &smoozy_keys::diary::Act) {}
        }
        struct NothingIsPosted;
        impl crate::notifications::ThePlatform for NothingIsPosted {
            fn the_machinery(&self) -> crate::notifications::TheMachinery {
                crate::notifications::TheMachinery::ItIsInPlace
            }
            fn the_permission(&self) -> crate::notifications::permission::WhatTheSystemSays {
                crate::notifications::permission::WhatTheSystemSays::ItIsGranted
            }
            fn ask_for_permission_now(&self) {}
            fn post(&self, _post: &crate::notifications::APost) {}
            fn take_down(&self, _key: &crate::presentation::the_presentation_key::ThePresentationKey) {}
        }

        for (check, marks) in [
            (InstallationCheck::PresentAndEqualConfirmed, false),
            (InstallationCheck::PresentAndTheSupervisorDidNotAnswer, false),
            (InstallationCheck::Absent, true),
            (InstallationCheck::PresentAndDifferent, true),
        ] {
            let connection = rusqlite::Connection::open_in_memory().unwrap();
            for migration in crate::record::migrations::ALL {
                if let crate::record::migrations::WhatItRuns::TheseStatements(statements) =
                    migration.what_it_runs
                {
                    for statement in statements {
                        connection.execute(statement, []).unwrap();
                    }
                }
            }
            let record = ARecordInHand(connection);
            let server = Identity::issue(IdentityKind::LocalServer).unwrap();
            let mut waiting = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen");
            waiting.standing = smoozy_queue_history::record::Standing::Provisional;
            waiting.the_render_start_has_been_learned = false;
            crate::record::job_records::JobRecords::over(&record.0)
                .write_the_provisional_record(&waiting)
                .unwrap();

            let periods = ThePeriodsThisRun::over(
                TheRunThroughTheApp::lending_nothing(),
                Arc::new(crate::events::TheOneQueue::new()),
                Arc::new(TheNotificationService::on(Box::new(NothingIsPosted))),
            );
            let mut desk = WhatTheDesktopKeeps {
                record: &record,
                periods: &periods,
            };
            desk.the_connection_read(&server.as_text(), check, None);

            let history = crate::record::job_records::JobRecords::over(&record.0)
                .in_history()
                .unwrap();
            assert_eq!(
                history.entries.iter().filter(|entry| entry.start_never_heard).count(),
                usize::from(marks),
                "{check:?}"
            );
            assert!(
                crate::record::job_records::JobRecords::over(&record.0)
                    .read(&waiting.job)
                    .unwrap()
                    .is_some(),
                "**nothing is removed**, on any branch ({check:?})"
            );
        }
    }

    /// **`Download` W2's cadence is the crate's own and this file writes no
    /// second one.** A pass with nothing to walk answers with that clause's own
    /// floor for one arrangement, and the figure is taken from the clause's own
    /// function rather than written here.
    #[test]
    fn the_pause_between_two_passes_is_the_clauses_own_figure() {
        let record = ARecordForAPass::made();
        let sessions: ThisServersSessions = ThisServersSessions::for_one_server();
        let server = Identity::issue(IdentityKind::LocalServer).expect("randomness");
        let run = Identity::issue(IdentityKind::Run).expect("randomness");
        let layout = TheLayout::of(
            smoozy_job_files::layout::MachineRoot::chosen("/srv/smoozy").expect("a root"),
            smoozy_job_files::layout::WhereThePackageAreaSits::UnderTheRoot,
        );
        let the_run = crate::downloading::TheRunsTransfers::for_this_run();
        let mut holds = TheArrangementsThisConnectionDrives::fresh();
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("a runtime");
        let pause = runtime.block_on(one_pass_over_this_servers_arrangements(
            &mut holds, &sessions, &the_run, &record, &server, &layout, &run,
        ));
        assert_eq!(
            pause,
            the_pause_before_the_next_walk(std::time::Duration::ZERO, 1)
        );
        assert_eq!(pause, smoozy_download::watch::THE_PAUSE);
    }

    /// **The driving ends with the connection it was started for**, and never
    /// waits on something it cannot reach: a connection that is already gone
    /// gives no pass at all and the sequence returns.
    #[test]
    fn the_driving_ends_with_the_connection_it_was_started_for() {
        let record = ARecordForAPass::made();
        let sessions: ThisServersSessions = ThisServersSessions::for_one_server();
        let server = Identity::issue(IdentityKind::LocalServer).expect("randomness");
        let run = Identity::issue(IdentityKind::Run).expect("randomness");
        let layout = TheLayout::of(
            smoozy_job_files::layout::MachineRoot::chosen("/srv/smoozy").expect("a root"),
            smoozy_job_files::layout::WhereThePackageAreaSits::UnderTheRoot,
        );
        let the_run = crate::downloading::TheRunsTransfers::for_this_run();
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("a runtime");
        let it_is_live = || false;
        let began = std::time::Instant::now();
        runtime.block_on(this_connections_arrangements_are_driven(
            &sessions,
            &the_run,
            &record,
            &server,
            &layout,
            &run,
            &it_is_live,
        ));
        assert!(
            began.elapsed() < smoozy_download::watch::THE_PAUSE,
            "the sequence waited on a connection that had already gone"
        );
    }

    /// A local record in memory, at the version this build knows.
    struct ARecordForAPass(Mutex<rusqlite::Connection>);

    impl ARecordForAPass {
        fn made() -> ARecordForAPass {
            let connection = rusqlite::Connection::open_in_memory().expect("a record");
            for migration in crate::record::migrations::ALL {
                if let crate::record::migrations::WhatItRuns::TheseStatements(statements) =
                    migration.what_it_runs
                {
                    for statement in statements {
                        connection
                            .execute(statement, [])
                            .expect("the migration runs");
                    }
                }
            }
            ARecordForAPass(Mutex::new(connection))
        }
    }

    impl crate::downloading::TheRecordADownloadWrites for ARecordForAPass {
        fn with_the_record(&self, act: &mut dyn FnMut(&rusqlite::Connection)) -> bool {
            let held = self
                .0
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            act(&held);
            true
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Step 10c's ninth-pass fix piece 1 — frames home while the job renders
    // ═══════════════════════════════════════════════════════════════════════
    //
    // **What the ninth pass saw, and what these two measure as a person meets
    // it.** A 240-frame job rendered for three minutes and Transfers said *0 /
    // 0* the whole time; the first frame landed 11 s after the render ended;
    // and a second job's first frame waited until the first job's download had
    // finished. **So what is counted here is files in the user's own folder**,
    // pass by pass, while the machine renders.
    //
    // **The stand-in machine writes its files in the order and at the moments
    // the real one does** (`Rendering` P1): the render start is learned at
    // step 1, when nothing is in the record directory yet; the recorded
    // reading lands at step 4; the attempt record at step 7; and frames after
    // that, one by one. **The recorded reading is the machine's own
    // composition** — `Job files and outputs`' composer, run over a printed
    // reading of this test's own shapes — and it is written under the one file
    // name both ends read. **Nothing here says what a real Blender prints.**
    //
    // **The far end is OpenSSH's own `sftp-server`**, one process per session
    // on a Unix socket pair, lent through this crate's own per-server point
    // (`Server protocol and transfers` T5) exactly as a live connection lends
    // it. **Where `sftp-server` is not on this system the tests say so and
    // stop**, the shape `tests/a_download_lands_in_the_chosen_folder.rs`
    // already uses.
    #[cfg(unix)]
    mod frames_home_while_the_job_renders {
        use std::path::{Path, PathBuf};
        use std::sync::{Arc, Mutex};
        use std::time::Duration;

        use smoozy_download::arrangement::TheNarrowing;
        use smoozy_job_files::attempts::{AttemptRecord, TheMapInForce};
        use smoozy_job_files::composing::{
            the_reading_and_everything_derived_from, TheReadingAndWhatFollows,
            WhatTheReadingIsComposedAgainst,
        };
        use smoozy_job_files::inventory::WhatTheInventoryIs;
        use smoozy_job_files::layout::{MachineRoot, TheLayout, WhereThePackageAreaSits};
        use smoozy_job_files::main_output::WhereTheRenderEnters;
        use smoozy_job_files::manifest::{
            the_recorded_readings_file_name, AnchorState, CarriedAndNotInterpretedHere,
            FrameRange, SceneField, SubmissionManifest, TheBlenderToRenderBy,
            TheCardMakeAtSubmission, TheRecordedReading, MANIFEST_FILE_NAME,
        };
        use smoozy_job_files::publication::AJobsPaths;
        use smoozy_job_files::reading::{
            ArtistsText, AtOneFrame, Destination, DestinationKind, Format, GatingConditions,
            HowManyViewsTheSceneWrites, RecordedLocation, TheVersionInHand, WhereItComesFrom,
            WhichScene,
        };
        use smoozy_job_files::reading_script::{TheObservedOutputs, ThePrintedReading};
        use smoozy_queue_history::acting::note::{
            TheEndingOnTheNote, TheNote, TheOutcomeAsWritten,
        };
        use smoozy_server_protocol::transfers::SourceReading;
        use smoozy_server_protocol::transport::{CommandResult, CommandString, Ending};
        use smoozy_state::identities::{Identity, IdentityKind};
        use smoozy_state::observed::{Clock, MachineClock, Reading};

        use super::ARecordForAPass;
        use crate::downloading::{
            the_arrangement_is_made_at_submission, TheArrangementsKeeper, TheRunsTransfers,
            WhatTheSubmissionChose,
        };
        use crate::record::job_records::JobRecords;
        use crate::servers::a_period::{
            one_pass_over_this_servers_arrangements, this_connection_drives_its_arrangements,
            TheArrangementsThisConnectionDrives,
        };
        use crate::servers::sessions::{
            lend_until_it_ends, AHeldConnection, NoSession, TheAdditionalConnection,
            ThisServersSessions,
        };

        // ------------------------------------------ the far end, per session

        /// The places each system's own OpenSSH puts its file-transfer server.
        const WHERE_SFTP_SERVER_LIVES: &[&str] = &[
            "/usr/libexec/sftp-server",
            "/usr/lib/openssh/sftp-server",
            "/usr/libexec/openssh/sftp-server",
            "/usr/lib/ssh/sftp-server",
        ];

        fn the_machines_own_file_transfer_server() -> Option<PathBuf> {
            WHERE_SFTP_SERVER_LIVES
                .iter()
                .map(PathBuf::from)
                .find(|path| path.exists())
        }

        /// **A rented machine, standing in**: every session this crate's point
        /// opens is one `sftp-server` of its own, over a socket pair.
        ///
        /// **`each_request_waits` stands in for the connection's round trip**:
        /// every request the desktop sends reaches the machine that much later.
        /// The ninth pass measured about 1.7 s a 665 KB file on Hyperstack
        /// against a line `Download`'s Fact 25 measured at 12 to 15 MB a
        /// second, so almost all of a file's cost there is round trips rather
        /// than bytes. **It is the connection's, not the machine's**, and it is
        /// not a figure of any plan's.
        struct AStandInMachine {
            program: PathBuf,
            each_request_waits: Duration,
            processes: Arc<Mutex<Vec<std::process::Child>>>,
            /// **How many sessions stand at once, and the most there ever
            /// were** — which is how `Download` Q1's two is watched from the
            /// machine's side.
            sessions: Arc<TheSessionsStanding>,
            /// **How many more bytes the desktop may read before the line is
            /// cut**, across every session — the user's *Disconnect* in the
            /// middle of a file, as ninth-pass fix piece 6 needs it. Never, for
            /// every machine made by [`TheDesk::the_machine`].
            the_line_is_cut_after: Arc<std::sync::atomic::AtomicU64>,
        }

        #[derive(Default)]
        struct TheSessionsStanding {
            now: std::sync::atomic::AtomicUsize,
            most: std::sync::atomic::AtomicUsize,
        }

        /// **One session's bytes, each write reaching the machine late.**
        struct ALateChannel {
            inner: tokio::net::UnixStream,
            each_request_waits: Duration,
            waiting: Option<std::pin::Pin<Box<tokio::time::Sleep>>>,
            the_wait_is_paid: bool,
            sessions: Arc<TheSessionsStanding>,
            the_line_is_cut_after: Arc<std::sync::atomic::AtomicU64>,
        }

        impl ALateChannel {
            fn the_line_is_cut(&self) -> bool {
                self.the_line_is_cut_after
                    .load(std::sync::atomic::Ordering::SeqCst)
                    == 0
            }
        }

        impl Drop for ALateChannel {
            fn drop(&mut self) {
                self.sessions
                    .now
                    .fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
            }
        }

        impl tokio::io::AsyncRead for ALateChannel {
            fn poll_read(
                self: std::pin::Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
                buf: &mut tokio::io::ReadBuf<'_>,
            ) -> std::task::Poll<std::io::Result<()>> {
                let this = self.get_mut();
                if this.the_line_is_cut() {
                    return std::task::Poll::Ready(Err(std::io::ErrorKind::ConnectionReset.into()));
                }
                let before = buf.filled().len();
                let read = std::pin::Pin::new(&mut this.inner).poll_read(cx, buf);
                let taken = (buf.filled().len() - before) as u64;
                let _ = this.the_line_is_cut_after.fetch_update(
                    std::sync::atomic::Ordering::SeqCst,
                    std::sync::atomic::Ordering::SeqCst,
                    |left| Some(left.saturating_sub(taken)),
                );
                read
            }
        }

        impl tokio::io::AsyncWrite for ALateChannel {
            fn poll_write(
                self: std::pin::Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
                buf: &[u8],
            ) -> std::task::Poll<std::io::Result<usize>> {
                use std::future::Future;
                let this = self.get_mut();
                if this.the_line_is_cut() {
                    return std::task::Poll::Ready(Err(std::io::ErrorKind::BrokenPipe.into()));
                }
                if !this.each_request_waits.is_zero() && !this.the_wait_is_paid {
                    let wait = this.each_request_waits;
                    let sleep = this
                        .waiting
                        .get_or_insert_with(|| Box::pin(tokio::time::sleep(wait)));
                    if sleep.as_mut().poll(cx).is_pending() {
                        return std::task::Poll::Pending;
                    }
                    this.waiting = None;
                    this.the_wait_is_paid = true;
                }
                let written = std::pin::Pin::new(&mut this.inner).poll_write(cx, buf);
                if written.is_ready() {
                    this.the_wait_is_paid = false;
                }
                written
            }

            fn poll_flush(
                self: std::pin::Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
            ) -> std::task::Poll<std::io::Result<()>> {
                std::pin::Pin::new(&mut self.get_mut().inner).poll_flush(cx)
            }

            fn poll_shutdown(
                self: std::pin::Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
            ) -> std::task::Poll<std::io::Result<()>> {
                std::pin::Pin::new(&mut self.get_mut().inner).poll_shutdown(cx)
            }
        }

        impl AHeldConnection for AStandInMachine {
            type Channel = ALateChannel;

            async fn open_a_session(&mut self) -> Result<ALateChannel, NoSession> {
                use std::os::fd::OwnedFd;
                let (ours, theirs) = std::os::unix::net::UnixStream::pair()
                    .map_err(|_| NoSession::TheConnectionIsGone)?;
                let reading = theirs
                    .try_clone()
                    .map_err(|_| NoSession::TheConnectionIsGone)?;
                let child = std::process::Command::new(&self.program)
                    .stdin(std::process::Stdio::from(OwnedFd::from(reading)))
                    .stdout(std::process::Stdio::from(OwnedFd::from(theirs)))
                    .stderr(std::process::Stdio::null())
                    .spawn()
                    .map_err(|_| NoSession::TheConnectionIsGone)?;
                self.processes
                    .lock()
                    .unwrap_or_else(|poisoned| poisoned.into_inner())
                    .push(child);
                ours.set_nonblocking(true)
                    .map_err(|_| NoSession::TheConnectionIsGone)?;
                let inner = tokio::net::UnixStream::from_std(ours)
                    .map_err(|_| NoSession::TheConnectionIsGone)?;
                let now = self
                    .sessions
                    .now
                    .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
                    + 1;
                self.sessions
                    .most
                    .fetch_max(now, std::sync::atomic::Ordering::SeqCst);
                Ok(ALateChannel {
                    inner,
                    each_request_waits: self.each_request_waits,
                    waiting: None,
                    the_wait_is_paid: false,
                    sessions: Arc::clone(&self.sessions),
                    the_line_is_cut_after: Arc::clone(&self.the_line_is_cut_after),
                })
            }

            /// **No download asks for a command**, so the stand-in runs none.
            async fn a_command(
                &mut self,
                _command: CommandString,
                _frame: Option<Vec<u8>>,
            ) -> Result<CommandResult, NoSession> {
                Err(NoSession::Refused)
            }

            /// **The connection stands until the test is done with it**, which
            /// is when the lending is dropped.
            async fn until_it_ends(&mut self) -> Ending {
                std::future::pending().await
            }

            async fn torn_down(self) -> Ending {
                Ending::Announced
            }
        }

        // ------------------------------------------------------ the desk

        /// A folder of this test's own, deleted when it goes, and every
        /// `sftp-server` it started reaped with it.
        struct TheDesk {
            at: PathBuf,
            record: ARecordForAPass,
            server: Identity,
            run: Identity,
            layout: TheLayout,
            processes: Arc<Mutex<Vec<std::process::Child>>>,
        }

        impl TheDesk {
            fn new(named: &str) -> TheDesk {
                // **Beside the test's own program and not in the system's
                // temporary folder**: the machine's root sits in here, and
                // `Job files and outputs` L3 refuses a root under a shared
                // temporary area — which on Linux the system's temporary
                // folder is (`/tmp`), and on this Mac it is not. The build's
                // own folder is never one.
                let at = std::env::current_exe()
                    .expect("the test's own program")
                    .parent()
                    .expect("its folder")
                    .join(format!("smoozy-frames-home-{named}-{}", std::process::id()));
                let _ = std::fs::remove_dir_all(&at);
                std::fs::create_dir_all(at.join("machine")).expect("the machine's root is made");
                let layout = TheLayout::of(
                    MachineRoot::chosen(
                        at.join("machine").to_str().expect("the scratch path is text"),
                    )
                    .expect("a root"),
                    WhereThePackageAreaSits::UnderTheRoot,
                );
                TheDesk {
                    at,
                    record: ARecordForAPass::made(),
                    server: Identity::issue(IdentityKind::LocalServer).expect("randomness"),
                    run: Identity::issue(IdentityKind::Run).expect("randomness"),
                    layout,
                    processes: Arc::new(Mutex::new(Vec::new())),
                }
            }

            fn the_machine(&self, program: PathBuf, each_request_waits: Duration) -> AStandInMachine {
                AStandInMachine {
                    program,
                    each_request_waits,
                    processes: Arc::clone(&self.processes),
                    sessions: Arc::default(),
                    the_line_is_cut_after: Arc::new(std::sync::atomic::AtomicU64::new(u64::MAX)),
                }
            }
        }

        impl Drop for TheDesk {
            fn drop(&mut self) {
                for mut child in self
                    .processes
                    .lock()
                    .unwrap_or_else(|poisoned| poisoned.into_inner())
                    .drain(..)
                {
                    let _ = child.kill();
                    let _ = child.wait();
                }
                let _ = std::fs::remove_dir_all(&self.at);
            }
        }

        /// **One job, submitted unread with downloads on, and what its machine
        /// will write for it.**
        struct AJob {
            job: Identity,
            paths: AJobsPaths,
            /// The folder the arrangement was given at submission.
            destination: PathBuf,
            /// **What the render worker will compose at step 4** — held here
            /// and written only at the moment the real worker writes it.
            composed: TheReadingAndWhatFollows,
            /// Each frame's predicted file, whole, on the machine.
            frames: Vec<(i64, PathBuf)>,
        }

        /// **What a reading of this job's `.blend` prints, as the stand-in
        /// prints it**: the scene's own output path, one PNG a frame.
        fn what_the_reading_prints(frames: &[i64]) -> ThePrintedReading {
            let layer = || ArtistsText::of(b"ViewLayer".to_vec());
            let gating = GatingConditions {
                compositing_switch: true,
                a_compositing_tree_is_present: false,
                a_live_final_output_node: false,
                the_final_output_node_is_muted: false,
                four_fives_use_nodes_switch: None,
                save_output_switch: None,
                the_render_goes_out_through_a_sequencer_holding_a_scene_strip: false,
            };
            ThePrintedReading {
                outputs: TheObservedOutputs {
                    taken_by: TheVersionInHand::of(5, 2),
                    gating: gating.clone(),
                    destinations: vec![Destination {
                        kind: DestinationKind::TheRegularOutputPath,
                        recorded_location: Some(RecordedLocation::of(b"//out/f_".to_vec())),
                        format: Some(Format {
                            as_the_file_records_it: ArtistsText::of(b"PNG".to_vec()),
                            the_ending_it_writes: Some(ArtistsText::of(b".png".to_vec())),
                            is_video_shaped: false,
                        }),
                        origin: WhereItComesFrom::TheScenesOutputPath,
                        scene: WhichScene::TheJobsOwnScene,
                        the_view_layers_that_feed_it: vec![layer()],
                        the_location_with_its_folder_replaced: None,
                    }],
                    scenes_reached_through_a_sequencer_strip: Vec::new(),
                    the_view_transform: None,
                    the_tree_is_shared_with: None,
                    per_frame: frames
                        .iter()
                        .map(|frame| AtOneFrame {
                            frame: *frame,
                            view_layers_that_render: vec![layer()],
                            gating: gating.clone(),
                            the_regular_output_paths_answered_path: Some(RecordedLocation::of(
                                format!("/artist/out/f_{frame:04}.png").into_bytes(),
                            )),
                        })
                        .collect(),
                    recorded_locations: Vec::new(),
                    views: HowManyViewsTheSceneWrites::One,
                    the_compositing_tree: None,
                },
                settings: CarriedAndNotInterpretedHere(serde_json::json!({})),
                dependencies: CarriedAndNotInterpretedHere(serde_json::json!({})),
                errors: std::collections::BTreeMap::new(),
                the_baked_caches_on_the_machine: Vec::new(),
                the_scene_holds_a_simulation_not_baked: false,
                the_scene_it_read: None,
                the_open_files_path: None,
                the_check_is_written_as_a_picture: false,
            }
        }

        /// **Submitted, placed, with downloads on** — the manifest says there
        /// is no recorded reading, which is every job the ninth pass rendered.
        fn a_job_submitted_unread(desk: &TheDesk, label: &str, frames: u32) -> AJob {
            let job = Identity::issue(IdentityKind::Job).expect("randomness");
            let paths = AJobsPaths::composed(&desk.layout, &job, Some(label)).expect("the paths");
            std::fs::create_dir_all(&paths.job_record_directory).expect("the record directory");
            std::fs::create_dir_all(&paths.job_output_folder).expect("the job output folder");

            let manifest = SubmissionManifest::composed(
                &job,
                None,
                TheBlenderToRenderBy {
                    version: "5.2.2".into(),
                    build_path: "/opt/blender".into(),
                },
                SceneField {
                    name: "Scene".into(),
                    anchor: AnchorState::ToAnchor {
                        provenance: "typed".into(),
                    },
                },
                FrameRange::TheUsersParsedIntegers((1..=i64::from(frames)).collect()),
                CarriedAndNotInterpretedHere(serde_json::json!({})),
                CarriedAndNotInterpretedHere(serde_json::json!({})),
                CarriedAndNotInterpretedHere(serde_json::json!({})),
                "/artist/scene.blend".into(),
                SourceReading {
                    size: 1,
                    inode: "1".into(),
                    modification_time: 1,
                    change_time: 1,
                },
                TheRecordedReading::ThereIsNone,
                TheCardMakeAtSubmission::NoReadingStood,
                smoozy_job_files::manifest::WhereTheJobOutputFolderIs::named_by(&paths),
            );
            std::fs::write(
                Path::new(&paths.job_record_directory).join(MANIFEST_FILE_NAME),
                manifest.as_written(),
            )
            .expect("the manifest is placed");

            let enters = WhereTheRenderEnters {
                scene: ArtistsText::of(b"Scene".to_vec()),
                view_layer: ArtistsText::of(b"ViewLayer".to_vec()),
            };
            let numbers: Vec<i64> = (1..=i64::from(frames)).collect();
            let composed = the_reading_and_everything_derived_from(
                what_the_reading_prints(&numbers),
                WhatTheReadingIsComposedAgainst {
                    the_folder_the_blend_sits_in: b"/artist",
                    job_output_folder: &paths.job_output_folder,
                    enters_at: &enters,
                    the_jobs_pins: &[],
                },
            )
            .expect("the stand-in's reading is a reading");
            let WhatTheInventoryIs::It(inventory) = &composed.inventory else {
                panic!("the stand-in's names are predictable: {:?}", composed.inventory);
            };
            let frames: Vec<(i64, PathBuf)> = inventory
                .entries
                .iter()
                .map(|entry| {
                    use std::os::unix::ffi::OsStrExt;
                    (
                        entry.frame.expect("every entry here has a frame"),
                        PathBuf::from(std::ffi::OsStr::from_bytes(&entry.predicted_path)),
                    )
                })
                .collect();
            assert_eq!(frames.len(), numbers.len(), "one file a frame");

            // **The local record, as a submission leaves it**: the job's own
            // record with no render start learned, and the arrangement made
            // at submission (`Download` R1).
            let mut record = crate::record::deletions::tests::a_record_in_history(&desk.server, label);
            record.job = job.clone();
            record.the_render_start_has_been_learned = false;
            desk.record.0.lock().map(|held| {
                JobRecords::over(&held)
                    .write_the_provisional_record(&record)
                    .expect("the job's record is written");
            })
            .expect("the record opens");
            let destination = desk.at.join("downloads").join(label);
            std::fs::create_dir_all(&destination).expect("the chosen folder");
            the_arrangement_is_made_at_submission(
                &mut TheArrangementsKeeper {
                    record: &desk.record,
                },
                true,
                WhatTheSubmissionChose {
                    job: job.clone(),
                    server_row: desk.server.clone(),
                    destination: {
                        use std::os::unix::ffi::OsStrExt;
                        destination.as_os_str().as_bytes().to_vec()
                    },
                    narrowing: TheNarrowing::Everything,
                },
            )
            .expect("the arrangement is made");

            AJob {
                job,
                paths,
                destination,
                composed,
                frames,
            }
        }

        /// The machine's note, as the period's own read carries it over.
        fn the_note_says(desk: &TheDesk, job: &AJob, ended: bool) {
            let clock = Clock::AMachine(MachineClock(desk.server.as_text()));
            let started = std::time::SystemTime::UNIX_EPOCH + Duration::from_secs(1_790_000_000);
            let mut note = TheNote::created_with_the_job_record("job");
            note.started = Some(Reading::taken((), started, clock.clone()));
            note.attempts = 1;
            if ended {
                note.ended = Some(Reading::taken((), started + Duration::from_secs(60), clock));
                note.ending = Some(TheEndingOnTheNote {
                    outcome: TheOutcomeAsWritten::Finished,
                    exit_status: Some(0),
                    the_attempt_whose_capture_record_holds_the_words: 1,
                    the_reason_the_machine_recorded: None,
                    the_device_line_as_observed: None,
                });
            }
            desk.record
                .0
                .lock()
                .map(|held| {
                    JobRecords::over(&held)
                        .the_fill_in(&job.job, &note)
                        .expect("the note is carried over");
                })
                .expect("the record opens");
        }

        /// **`Rendering` P1 step 1: the render starts** — the render start is
        /// learned, attempt 1, and nothing is in the record directory yet.
        fn the_render_starts(desk: &TheDesk, job: &AJob) {
            the_note_says(desk, job, false);
        }

        /// **`Rendering` P1 step 4: the reading is taken and written**, under
        /// the one name both ends read.
        fn the_machine_writes_the_recorded_reading(job: &AJob) {
            std::fs::write(
                Path::new(&job.paths.job_record_directory).join(the_recorded_readings_file_name(1)),
                serde_json::to_vec(&job.composed).expect("the reading renders"),
            )
            .expect("the recorded reading is written");
        }

        /// **`Rendering` P1 step 7: the attempt record, before the render
        /// starts** (`Job files and outputs` M8).
        fn the_machine_writes_the_attempt_record(job: &AJob) {
            let record = AttemptRecord {
                number: 1,
                frame_list_in_the_order_it_was_written: job.frames.iter().map(|(frame, _)| *frame).collect(),
                map_in_force: TheMapInForce::It(job.composed.map.clone()),
                file_identity_at_this_attempt: SourceReading {
                    size: 1,
                    inode: "1".into(),
                    modification_time: 1,
                    change_time: 1,
                },
                the_file_changed_before_this_attempt: false,
                a_frame_an_earlier_attempt_wrote_was_kept: false,
            };
            std::fs::write(
                Path::new(&job.paths.job_record_directory).join(record.file_name()),
                record.as_written(),
            )
            .expect("the attempt record is written");
        }

        /// **Blender writes one frame's file at its own final name.**
        fn blender_writes_frame(job: &AJob, frame: i64) {
            let (_, at) = job
                .frames
                .iter()
                .find(|(which, _)| *which == frame)
                .expect("the frame is in the job");
            std::fs::create_dir_all(at.parent().expect("a folder")).expect("the folder");
            std::fs::write(at, vec![frame as u8; 1_024]).expect("the frame is written");
        }

        /// **The ending, learned** (`Download` E1 step 1).
        fn the_job_ends(desk: &TheDesk, job: &AJob) {
            the_note_says(desk, job, true);
        }

        /// **Every file that has landed in the user's own folder for this
        /// job**, with the moment it was written — what a person sees.
        /// **When a file landed, and not its modification time**, which
        /// since the Creator's second notes, piece 10 (`Server protocol and
        /// transfers` S15c) is the server file's own. The change time moves
        /// when the staged file takes that time and when it is published;
        /// where there is none to read, the creation time is the moment its
        /// download began.
        fn the_moment_it_landed(read: &std::fs::Metadata) -> std::time::SystemTime {
            #[cfg(unix)]
            {
                use std::os::unix::fs::MetadataExt;
                std::time::UNIX_EPOCH
                    + std::time::Duration::new(read.ctime() as u64, read.ctime_nsec() as u32)
            }
            #[cfg(not(unix))]
            {
                read.created().expect("a creation time")
            }
        }

        fn what_came_home(job: &AJob) -> Vec<(PathBuf, std::time::SystemTime)> {
            fn under(folder: &Path, out: &mut Vec<(PathBuf, std::time::SystemTime)>) {
                let Ok(entries) = std::fs::read_dir(folder) else {
                    return;
                };
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.file_name().and_then(|name| name.to_str())
                        == Some(smoozy_file_browser::partial::THE_RESERVED_STAGING_FOLDER)
                    {
                        continue;
                    }
                    if path.is_dir() {
                        under(&path, out);
                    } else if let Ok(read) = entry.metadata() {
                        out.push((path, the_moment_it_landed(&read)));
                    }
                }
            }
            let mut out = Vec::new();
            under(&job.destination, &mut out);
            out
        }

        fn a_runtime() -> tokio::runtime::Runtime {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("a runtime")
        }

        /// **The passes begin once the lending has begun**, which is how a
        /// period starts them (`the_arrangements_are_driven`, from the moment a
        /// connection is lent). **Bounded**: the lending begins at its first
        /// poll, so a handful of turns is plenty.
        async fn the_connection_is_lent(sessions: &ThisServersSessions<AStandInMachine>) {
            for _ in 0..1_000 {
                if sessions.a_connection_is_live() {
                    return;
                }
                tokio::task::yield_now().await;
            }
            panic!("the stand-in's connection was never lent");
        }

        fn nothing_more() -> TheAdditionalConnection<AStandInMachine> {
            Arc::new(|| Box::pin(async { None }))
        }

        #[test]
        fn seventeenth_the_connection_driver_uses_the_report_without_another_frame_write() {
            let program = the_machines_own_file_transfer_server().expect("the measurement needs sftp-server");
            let desk = TheDesk::new("seventeenth-driver");
            let job = a_job_submitted_unread(&desk, "driver", 4);
            the_render_starts(&desk, &job);
            the_machine_writes_the_recorded_reading(&job);
            the_machine_writes_the_attempt_record(&job);
            for frame in 1..=3 { blender_writes_frame(&job, frame); }
            let sessions = ThisServersSessions::for_one_server();
            let mut machine = desk.the_machine(program, Duration::ZERO);
            let additional = nothing_more();
            let transfers = TheRunsTransfers::for_this_run();
            let work = crate::servers::the_machines_work::WhatThisRunHolds::default();
            let connection = work.a_connection_begins(&desk.server.as_text());
            work.a_reading_was_taken(&desk.server.as_text(), connection,
                crate::servers::the_machines_work::tests::a_reading_with_a_saved_prefix(&job.job, 1, 3), None);
            a_runtime().block_on(async {
                let recovery = async {
                    the_connection_is_lent(&sessions).await;
                    let live = || sessions.a_connection_is_live() && what_came_home(&job).len() < 3;
                    super::super::this_connections_arrangements_are_driven_for_the_run(
                        &sessions, &transfers, &desk.record, &desk.server, &desk.layout,
                        &Identity::issue(IdentityKind::Run).unwrap(), &live, &work,
                    ).await;
                };
                tokio::select! {
                    _ = lend_until_it_ends(&sessions, &mut machine, &additional) => panic!("the test's connection ended"),
                    result = tokio::time::timeout(smoozy_download::watch::THE_PAUSE, recovery) => result.expect("the first walk owes no previous-walk pause"),
                }
            });
            assert_eq!(what_came_home(&job).len(), 3);
        }

        #[test]
        fn seventeenth_a_saved_report_downloads_closed_frames_on_the_first_walk() {
            let program = the_machines_own_file_transfer_server().expect("the measurement needs sftp-server");
            let desk = TheDesk::new("seventeenth-saved-report");
            let job = a_job_submitted_unread(&desk, "saved-report", 4);
            the_render_starts(&desk, &job);
            the_machine_writes_the_recorded_reading(&job);
            the_machine_writes_the_attempt_record(&job);
            for frame in 1..=3 { blender_writes_frame(&job, frame); }
            // A fresh application run takes up the saved arrangement.
            let run = Identity::issue(IdentityKind::Run).unwrap();
            let sessions = ThisServersSessions::for_one_server();
            let mut machine = desk.the_machine(program, Duration::ZERO);
            let additional = nothing_more();
            let transfers = TheRunsTransfers::for_this_run();
            a_runtime().block_on(async {
                let recovery = async {
                    the_connection_is_lent(&sessions).await;
                    let began = std::time::Instant::now();
                    let wall = std::time::SystemTime::now();
                    let work = crate::servers::the_machines_work::WhatThisRunHolds::default();
                    let connection = work.a_connection_begins(&desk.server.as_text());
                    work.a_reading_was_taken(&desk.server.as_text(), connection,
                        crate::servers::the_machines_work::tests::a_reading_with_a_saved_prefix(&job.job, 1, 3), None);
                    println!("seventeenth: connection usable to job catch-up: {:?}", began.elapsed());
                    let mut holds = TheArrangementsThisConnectionDrives::fresh();
                    holds.saved_prefixes = work.the_saved_prefixes_reported_on_this_connection(&desk.server.as_text());
                    one_pass_over_this_servers_arrangements(&mut holds, &sessions, &transfers,
                        &desk.record, &desk.server, &desk.layout, &run).await;
                    let files = what_came_home(&job);
                    assert_eq!(files.len(), 3, "saved frames must resume without waiting for frame 4");
                    for (stage, at) in &holds.recovery_stages {
                        println!("seventeenth: connection usable to {stage}: {:?}", at.duration_since(began));
                    }
                    let first = files.iter().map(|(_, at)| at.duration_since(wall).unwrap_or_default()).min().unwrap();
                    println!("seventeenth: connection usable to first resumed file: {first:?}; all three: {:?}", began.elapsed());
                    assert!(began.elapsed() < smoozy_download::watch::THE_PAUSE,
                        "the first walk owes no previous-walk pause");
                };
                tokio::select! {
                    _ = lend_until_it_ends(&sessions, &mut machine, &additional) => panic!("the test's connection ended"),
                    result = tokio::time::timeout(Duration::from_secs(30), recovery) => result.expect("recovery exceeded 30 seconds"),
                }
            });
        }

        #[test]
        fn seventeenth_reopening_downloads_wait_for_the_next_frame_signal() {
            let Some(program) = the_machines_own_file_transfer_server() else {
                panic!("the recovery measurement needs the test's file-transfer server");
            };
            let desk = TheDesk::new("seventeenth-reopening");
            let job = a_job_submitted_unread(&desk, "reopening", 4);
            the_render_starts(&desk, &job);
            the_machine_writes_the_recorded_reading(&job);
            the_machine_writes_the_attempt_record(&job);
            // These frames were published while the desktop was closed.
            for frame in 1..=3 {
                blender_writes_frame(&job, frame);
            }
            let sessions = ThisServersSessions::for_one_server();
            let mut machine = desk.the_machine(program, Duration::ZERO);
            let additional = nothing_more();
            let the_run = TheRunsTransfers::for_this_run();
            a_runtime().block_on(async {
                let recovery = async {
                    the_connection_is_lent(&sessions).await;
                    let began = std::time::Instant::now();
                    let mut holds = TheArrangementsThisConnectionDrives::fresh();
                    // A different attempt's report cannot release this attempt.
                    holds.saved_prefixes.insert(job.job.as_text(), (2, 3));
                    let first_pause = one_pass_over_this_servers_arrangements(
                        &mut holds, &sessions, &the_run, &desk.record,
                        &desk.server, &desk.layout, &desk.run,
                    ).await;
                    let first_walk = began.elapsed();
                    let one = holds.per_job.get(&job.job.as_text()).unwrap();
                    assert_eq!(one.running.the_last_complete_walk().iter().filter(|entry| !entry.is_a_folder).count(), 3);
                    assert!(what_came_home(&job).is_empty());
                    assert_eq!(first_pause, smoozy_download::watch::THE_PAUSE);
                    holds.saved_prefixes.clear();
                    println!("seventeenth: usable connection to material admission and first complete walk: {first_walk:?}; first resumed file: none; next pause: {first_pause:?}");

                    // Advance the sequence by eighteen of its own pauses,
                    // without sleeping: 180 seconds of modeled render time.
                    // Every real walk still runs over the test's machine.
                    let mut modeled = Duration::ZERO;
                    for _ in 0..18 {
                        modeled += one_pass_over_this_servers_arrangements(
                            &mut holds, &sessions, &the_run, &desk.record,
                            &desk.server, &desk.layout, &desk.run,
                        ).await;
                        assert!(what_came_home(&job).is_empty());
                    }
                    assert_eq!(modeled, smoozy_download::watch::THE_PAUSE * 18);
                    println!("seventeenth: {modeled:?} of modeled pauses, no new frame activity, no fetch admitted");
                    blender_writes_frame(&job, 4);
                    let signal = std::time::Instant::now();
                    one_pass_over_this_servers_arrangements(
                        &mut holds, &sessions, &the_run, &desk.record,
                        &desk.server, &desk.layout, &desk.run,
                    ).await;
                    let files = what_came_home(&job);
                    assert_eq!(files.len(), 3, "the new frame signal releases the three older frames");
                    assert!(signal.elapsed() < smoozy_download::watch::THE_PAUSE,
                        "once the signal is read, no extra walk pause is due");
                    println!("seventeenth: new frame signal to three resumed files: {:?}", signal.elapsed());
                };
                let bounded = tokio::time::timeout(Duration::from_secs(30), recovery);
                tokio::select! {
                    _ = lend_until_it_ends(&sessions, &mut machine, &additional) => panic!("the test's connection ended"),
                    result = bounded => result.expect("the recovery measurement exceeded 30 seconds"),
                }
            });
        }

        /// **`Download` R5's dated note of 2026-10-31: the ending walk's
        /// completion is written down, and a fresh run does not walk an ended
        /// job's folder again.** A job ends and its frames come home; then
        /// Smoozy opens again — fresh holds, a fresh run's register — and the
        /// first pass reads the arrangement done without walking. The
        /// twenty-sixth notes, note 4: every ended job's folder was walked to
        /// its end once more on the first connection after a restart, which
        /// is what set a first-frame check taking its range job's frames.
        #[test]
        fn twenty_sixth_4_an_ended_jobs_folder_is_not_walked_again_after_a_restart() {
            let program = the_machines_own_file_transfer_server().expect("the measurement needs sftp-server");
            let desk = TheDesk::new("twenty-sixth-4-restart");
            let job = a_job_submitted_unread(&desk, "n4 restart", 3);
            the_render_starts(&desk, &job);
            the_machine_writes_the_recorded_reading(&job);
            the_machine_writes_the_attempt_record(&job);
            for frame in 1..=3 {
                blender_writes_frame(&job, frame);
            }
            the_job_ends(&desk, &job);
            let written_down = || {
                desk.record
                    .0
                    .lock()
                    .map(|held| crate::record::download_records::DownloadArrangements::over(&held).the_ending_walk_is_written_down(&job.job).unwrap())
                    .expect("the record opens")
            };
            let sessions = ThisServersSessions::for_one_server();
            let mut machine = desk.the_machine(program, Duration::ZERO);
            let additional = nothing_more();
            a_runtime().block_on(async {
                let passes = async {
                    the_connection_is_lent(&sessions).await;
                    let the_run = TheRunsTransfers::for_this_run();
                    let mut holds = TheArrangementsThisConnectionDrives::fresh();
                    for _ in 0..8 {
                        let _ = one_pass_over_this_servers_arrangements(
                            &mut holds, &sessions, &the_run, &desk.record, &desk.server, &desk.layout, &desk.run,
                        )
                        .await;
                        if what_came_home(&job).len() == 3 {
                            break;
                        }
                    }
                    assert_eq!(what_came_home(&job).len(), 3);
                    assert!(written_down(), "the ending walk's completion was not written down");
                    let (_, a_frame) = &job.frames[0];
                    let key = {
                        use std::os::unix::ffi::OsStrExt;
                        a_frame
                            .strip_prefix(&job.paths.job_output_folder)
                            .expect("the frame is in the job's folder")
                            .as_os_str()
                            .as_bytes()
                            .to_vec()
                    };
                    let one = holds.per_job.get(&job.job.as_text()).expect("the arrangement was taken up");
                    assert!(one.running.what_a_walk_read_of(&key).is_some(), "the first run's ending walk read nothing");

                    // **Smoozy opens again**: nothing run-held survives.
                    let the_run = TheRunsTransfers::for_this_run();
                    let mut holds = TheArrangementsThisConnectionDrives::fresh();
                    let _ = one_pass_over_this_servers_arrangements(
                        &mut holds, &sessions, &the_run, &desk.record, &desk.server, &desk.layout,
                        &Identity::issue(IdentityKind::Run).unwrap(),
                    )
                    .await;
                    let one = holds.per_job.get(&job.job.as_text()).expect("the arrangement was taken up");
                    assert!(one.running.the_ending_walk_has_completed());
                    assert!(one.running.what_a_walk_read_of(&key).is_none(), "the ended job's folder was walked again");
                };
                tokio::select! {
                    _ = lend_until_it_ends(&sessions, &mut machine, &additional) => panic!("the test's connection ended"),
                    result = tokio::time::timeout(Duration::from_secs(30), passes) => result.expect("the passes exceeded 30 seconds"),
                }
            });
        }

        /// **The fault, as a person meets it: a job renders over several
        /// passes, its frames land on the machine one by one, and they come
        /// home while it renders — one frame behind, not after the ending.**
        ///
        /// On the code the ninth pass ran, nothing comes home before the
        /// ending: the desktop read the manifest alone, which holds no reading
        /// for a job submitted unread, so no frame had an inventory entry and
        /// `Download` J3 could judge none; and the attempt's order was read
        /// once, at the first pass after the render start and before step 7
        /// wrote it, and was kept as the answer (`Download` J2).
        #[test]
        fn a_jobs_frames_come_home_while_it_renders() {
            let Some(program) = the_machines_own_file_transfer_server() else {
                println!(
                    "SKIPPED: no OpenSSH sftp-server on this system ({}). Frames home while \
                     the job renders is unproved here.",
                    std::env::consts::OS
                );
                return;
            };
            let desk = TheDesk::new("one");
            let job = a_job_submitted_unread(&desk, "p12h9 long", 6);
            let sessions: ThisServersSessions<AStandInMachine> = ThisServersSessions::for_one_server();
            let mut machine = desk.the_machine(program, Duration::ZERO);
            let the_run = TheRunsTransfers::for_this_run();
            let additional = nothing_more();

            let came_home_pass_by_pass: Vec<(String, usize)> = a_runtime().block_on(async {
                let passes = async {
                    the_connection_is_lent(&sessions).await;
                    let mut holds = TheArrangementsThisConnectionDrives::fresh();
                    let mut seen = Vec::new();
                    macro_rules! a_pass {
                        ($after:expr) => {
                            let _ = one_pass_over_this_servers_arrangements(
                                &mut holds,
                                &sessions,
                                &the_run,
                                &desk.record,
                                &desk.server,
                                &desk.layout,
                                &desk.run,
                            )
                            .await;
                            seen.push(($after.to_string(), what_came_home(&job).len()));
                        };
                    }
                    // Steps 1 to 3: the render has started and the record
                    // directory holds nothing new.
                    the_render_starts(&desk, &job);
                    a_pass!("the render started");
                    the_machine_writes_the_recorded_reading(&job);
                    a_pass!("step 4 wrote the reading");
                    the_machine_writes_the_attempt_record(&job);
                    a_pass!("step 7 wrote the attempt record");
                    for frame in 1..=6 {
                        blender_writes_frame(&job, frame);
                        a_pass!(format!("frame {frame} was written"));
                    }
                    the_job_ends(&desk, &job);
                    a_pass!("the job ended");
                    a_pass!("one pass after the ending");
                    seen
                };
                tokio::select! {
                    _ = lend_until_it_ends(&sessions, &mut machine, &additional) => {
                        panic!("the stand-in's connection never ends on its own")
                    }
                    seen = passes => seen,
                }
            });

            let before_the_ending: Vec<usize> = came_home_pass_by_pass
                .iter()
                .take_while(|(after, _)| after != "the job ended")
                .map(|(_, home)| *home)
                .collect();
            assert!(
                before_the_ending.iter().any(|home| *home > 0),
                "**nothing came home while the job rendered** — the ninth pass's fault. Pass by \
                 pass: {came_home_pass_by_pass:?}"
            );
            // **`Download` J3: a frame is judged finished when a strictly later
            // frame shows activity**, so after frame n is written the frames
            // before it are home — one frame behind, as `Download` J4 says.
            for (after, home) in &came_home_pass_by_pass {
                if let Some(frame) = after
                    .strip_prefix("frame ")
                    .and_then(|rest| rest.strip_suffix(" was written"))
                    .and_then(|n| n.parse::<usize>().ok())
                {
                    assert_eq!(
                        *home,
                        frame - 1,
                        "after frame {frame} was written, the frames before it should be home. \
                         Pass by pass: {came_home_pass_by_pass:?}"
                    );
                }
            }
            assert_eq!(
                came_home_pass_by_pass.last().map(|(_, home)| *home),
                Some(6),
                "every frame is home once the job has ended: {came_home_pass_by_pass:?}"
            );
        }

        /// **What `Rendering` P1 step 7 leaves in the job's folder where a
        /// baked cache was found or sent** (`Dependency Upload & Remap` M22c and
        /// `Dependency Upload & Remap` M22f): three cache files copied into the point cache's job-folder
        /// place, the list of what was placed with each file's four values read
        /// the moment it was made, and the empty staging folder the upload of
        /// the user's own cache folder left in it and the copy carried along.
        fn the_machine_places_a_baked_cache(job: &AJob) -> PathBuf {
            use smoozy_dependencies::acting::placing::{
                the_placed_copies_file_name, APlacedFile, ThePlacedCopies,
            };
            use std::os::unix::ffi::OsStrExt;
            use std::os::unix::fs::MetadataExt;
            let cache = Path::new(&job.paths.job_output_folder).join("pointcache");
            let staging = cache.join(smoozy_file_browser::partial::THE_RESERVED_STAGING_FOLDER);
            std::fs::create_dir_all(&staging)
                .expect("the cache's folder and the staging folder it carried");
            std::fs::write(staging.join("abc-1.smoozy-part"), b"staged bytes")
                .expect("an upload's staged bytes, carried along");
            let mut placed = ThePlacedCopies::default();
            for frame in 1..=3 {
                let at = cache.join(format!("436C6F7468_{frame:06}_00.bphys"));
                std::fs::write(&at, vec![9u8; 4_096]).expect("a cache file is copied");
                let read = std::fs::metadata(&at).expect("the copy");
                placed.files.push(APlacedFile {
                    job_folder_path: smoozy_dependencies::bytes::MachineLocation::composed(
                        at.as_os_str().as_bytes().to_vec(),
                    ),
                    size: read.size(),
                    inode: read.ino().to_string(),
                    modification_time: read.mtime(),
                    change_time: read.ctime(),
                });
            }
            std::fs::write(
                Path::new(&job.paths.job_record_directory).join(the_placed_copies_file_name(1)),
                placed.as_written(),
            )
            .expect("the placed-copy list is written");
            cache
        }

        /// **The render ended, and the machine read the placed files again**
        /// (`the_placed_copies_after_the_render_file_name`) — as the render
        /// worker does the moment its render has ended.
        fn the_machine_reads_the_placed_copies_again(job: &AJob) {
            use smoozy_dependencies::acting::placing::{
                the_placed_copies_after_the_render_file_name, the_placed_copies_file_name,
                ThePlacedCopies,
            };
            use std::os::unix::fs::MetadataExt;
            let dir = Path::new(&job.paths.job_record_directory);
            let mut placed = ThePlacedCopies::read_back(
                &std::fs::read(dir.join(the_placed_copies_file_name(1))).expect("the list"),
            )
            .expect("it reads");
            for file in &mut placed.files {
                let at = {
                    use std::os::unix::ffi::OsStrExt;
                    PathBuf::from(std::ffi::OsStr::from_bytes(file.job_folder_path.as_bytes()))
                };
                let read = std::fs::metadata(&at).expect("the placed file");
                file.size = read.size();
                file.inode = read.ino().to_string();
                file.modification_time = read.mtime();
                file.change_time = read.ctime();
            }
            std::fs::write(
                dir.join(the_placed_copies_after_the_render_file_name(1)),
                placed.as_written(),
            )
            .expect("the reading after the render is written");
        }

        /// Every staging folder that landed in the user's own folder for this
        /// job — `what_came_home` steps over them, so they are counted here.
        ///
        /// **The one in the destination folder itself is the desktop's own**,
        /// where this side stages what it fetches (`Server protocol and
        /// transfers` S1, `Download` D7), and is left out: what is counted is
        /// a staging folder brought home from the machine.
        fn staging_folders_home(job: &AJob) -> Vec<PathBuf> {
            fn under(folder: &Path, out: &mut Vec<PathBuf>) {
                let Ok(entries) = std::fs::read_dir(folder) else {
                    return;
                };
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.file_name().and_then(|name| name.to_str())
                        == Some(smoozy_file_browser::partial::THE_RESERVED_STAGING_FOLDER)
                    {
                        out.push(path.clone());
                    }
                    if path.is_dir() {
                        under(&path, out);
                    }
                }
            }
            let mut out = Vec::new();
            under(&job.destination, &mut out);
            let the_desktops_own =
                job.destination.join(smoozy_file_browser::partial::THE_RESERVED_STAGING_FOLDER);
            out.retain(|folder| *folder != the_desktops_own);
            out
        }

        /// **The fixes before 0.1.8, fault 3, as a person meets it: a job's
        /// download brings home the frames and nothing of Smoozy's own** — not
        /// the baked cache Smoozy copied into the job's folder before the render,
        /// and not a staging folder. **A cache file the render wrote over is
        /// the render's and comes home** (`Download` N5a's last bullet: *a copy
        /// the render wrote over is no longer that copy*).
        ///
        /// **The case it bites on.** The arrangement's ending walk was handed no
        /// placed-copy list at all — *"has no producer anywhere in this
        /// workspace"* — so every file Smoozy placed came home with the frames:
        /// 2.8 GB of the user's own cache on the check before 0.1.8.
        #[test]
        fn a_jobs_download_leaves_smoozys_own_copies_on_the_machine() {
            let Some(program) = the_machines_own_file_transfer_server() else {
                println!(
                    "SKIPPED: no OpenSSH sftp-server on this system ({}). Smoozy's own copies \
                     staying on the machine is unproved here.",
                    std::env::consts::OS
                );
                return;
            };
            let desk = TheDesk::new("placed");
            let job = a_job_submitted_unread(&desk, "p18 cache", 3);
            let sessions: ThisServersSessions<AStandInMachine> = ThisServersSessions::for_one_server();
            let mut machine = desk.the_machine(program, Duration::ZERO);
            let the_run = TheRunsTransfers::for_this_run();
            let additional = nothing_more();

            a_runtime().block_on(async {
                let passes = async {
                    the_connection_is_lent(&sessions).await;
                    let mut holds = TheArrangementsThisConnectionDrives::fresh();
                    macro_rules! a_pass {
                        () => {
                            let _ = one_pass_over_this_servers_arrangements(
                                &mut holds,
                                &sessions,
                                &the_run,
                                &desk.record,
                                &desk.server,
                                &desk.layout,
                                &desk.run,
                            )
                            .await;
                        };
                    }
                    the_render_starts(&desk, &job);
                    the_machine_writes_the_recorded_reading(&job);
                    let cache = the_machine_places_a_baked_cache(&job);
                    the_machine_writes_the_attempt_record(&job);
                    a_pass!();
                    for frame in 1..=3 {
                        blender_writes_frame(&job, frame);
                        a_pass!();
                    }
                    // **The render writes over one of the three**, the
                    // Creator's own *both* case (`Dependency Upload & Remap`
                    // M22b): what it wrote is the render's result.
                    std::fs::write(cache.join("436C6F7468_000002_00.bphys"), vec![5u8; 6_000])
                        .expect("the render writes over a placed file");
                    the_machine_reads_the_placed_copies_again(&job);
                    the_job_ends(&desk, &job);
                    a_pass!();
                    a_pass!();
                };
                tokio::select! {
                    _ = lend_until_it_ends(&sessions, &mut machine, &additional) => {
                        panic!("the stand-in's connection never ends on its own")
                    }
                    () = passes => {}
                }
            });

            let home: Vec<String> = what_came_home(&job)
                .iter()
                .map(|(path, _)| {
                    path.file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or_default()
                        .to_string()
                })
                .collect();
            let frames = home.iter().filter(|name| name.ends_with(".png")).count();
            assert_eq!(frames, 3, "every frame comes home: {home:?}");
            let caches: Vec<&String> = home.iter().filter(|name| name.ends_with(".bphys")).collect();
            assert_eq!(
                caches,
                vec![&"436C6F7468_000002_00.bphys".to_string()],
                "**only the cache file the render wrote over comes home** — the two Smoozy \
                 placed and nothing touched stay on the machine: {home:?}"
            );
            assert!(
                staging_folders_home(&job).is_empty(),
                "a staging folder came home: {:?}",
                staging_folders_home(&job)
            );
        }

        /// **The second fault, as a person meets it: a second job's frames do
        /// not wait for the first job's download to finish.**
        ///
        /// The first job rendered and ended with nothing fetching, so its whole
        /// backlog is on the machine when the connection's first pass runs; the
        /// second starts rendering behind it, one render at a time. **Every
        /// request here reaches the machine 30 ms late**, standing in for a
        /// rental's round trip, so the first job's backlog takes longer to
        /// bring down than two of `Download` W2's pauses. On the code the ninth
        /// pass ran, one arrangement's turn drained its whole queue before the
        /// next arrangement walked, so the second job's first frame landed only
        /// after the first job's last. `Download` W2 and W3 give each
        /// arrangement its own walk at its own pause.
        #[test]
        fn a_second_jobs_frames_do_not_wait_for_the_first_jobs_download() {
            let Some(program) = the_machines_own_file_transfer_server() else {
                println!(
                    "SKIPPED: no OpenSSH sftp-server on this system ({}). One job's download \
                     holding another's is unproved here.",
                    std::env::consts::OS
                );
                return;
            };
            // **Twice the ninth pass's 180.** The test holds only while the
            // first job's backlog outlasts two of `Download` W2's 10-second
            // pauses. Since the transfer waves, two files come home about
            // every 190 ms here, so 180 came home in 17 s on a quiet desk and
            // the second job's frames had nothing to overtake — it passed only
            // when the whole suite slowed it. 360 take about 35 s; the second
            // job's first frame lands near 21 s.
            const THE_FIRST_JOBS_FRAMES: u32 = 360;
            let desk = TheDesk::new("two");
            let first = a_job_submitted_unread(&desk, "p12h9 long", THE_FIRST_JOBS_FRAMES);
            let second = a_job_submitted_unread(&desk, "p12h9 long2", 6);
            let sessions: ThisServersSessions<AStandInMachine> = ThisServersSessions::for_one_server();
            let mut machine = desk.the_machine(program, Duration::from_millis(30));
            let standing = Arc::clone(&machine.sessions);
            let the_run = TheRunsTransfers::for_this_run();
            let additional = nothing_more();

            // **The first job rendered and ended while nothing was fetching.**
            the_render_starts(&desk, &first);
            the_machine_writes_the_recorded_reading(&first);
            the_machine_writes_the_attempt_record(&first);
            for frame in 1..=i64::from(THE_FIRST_JOBS_FRAMES) {
                blender_writes_frame(&first, frame);
            }
            the_job_ends(&desk, &first);

            a_runtime().block_on(async {
                let passes = async {
                    the_connection_is_lent(&sessions).await;
                    let mut holds = TheArrangementsThisConnectionDrives::fresh();
                    macro_rules! a_pass {
                        () => {
                            one_pass_over_this_servers_arrangements(
                                &mut holds,
                                &sessions,
                                &the_run,
                                &desk.record,
                                &desk.server,
                                &desk.layout,
                                &desk.run,
                            )
                        };
                    }
                    // The connection's first pass: the first job's ending walk.
                    let _ = a_pass!().await;
                    // **The second job's render starts behind it**, and its
                    // record directory fills in `Rendering` P1's order.
                    the_render_starts(&desk, &second);
                    the_machine_writes_the_recorded_reading(&second);
                    the_machine_writes_the_attempt_record(&second);
                    blender_writes_frame(&second, 1);
                    let _ = a_pass!().await;
                    blender_writes_frame(&second, 2);
                    blender_writes_frame(&second, 3);
                    let _ = a_pass!().await;
                    for frame in 4..=6 {
                        blender_writes_frame(&second, frame);
                    }
                    the_job_ends(&desk, &second);
                    // **Bounded**: enough passes to bring both home however
                    // the fetches are shaped, and never a wait with no end.
                    for _ in 0..8 {
                        if what_came_home(&first).len() == THE_FIRST_JOBS_FRAMES as usize
                            && what_came_home(&second).len() == 6
                        {
                            break;
                        }
                        let _ = a_pass!().await;
                    }
                };
                tokio::select! {
                    _ = lend_until_it_ends(&sessions, &mut machine, &additional) => {
                        panic!("the stand-in's connection never ends on its own")
                    }
                    () = passes => {}
                }
            });

            let first_home = what_came_home(&first);
            let second_home = what_came_home(&second);
            assert_eq!(first_home.len(), THE_FIRST_JOBS_FRAMES as usize, "the first job came home");
            assert_eq!(second_home.len(), 6, "the second job came home");
            let the_first_jobs_last = first_home.iter().map(|(_, at)| *at).max().expect("some");
            let the_second_jobs_first = second_home.iter().map(|(_, at)| *at).min().expect("some");
            let landed_before_it = first_home
                .iter()
                .filter(|(_, at)| *at < the_second_jobs_first)
                .count();
            // **`Download` Q1: two download channels at a server, and never
            // more.** The reading session stands throughout, so the most the
            // machine ever saw is that one and two transfers.
            assert_eq!(
                standing.most.load(std::sync::atomic::Ordering::SeqCst),
                3,
                "the reading session and Q1's two transfer channels, at once and never more"
            );
            assert!(
                the_second_jobs_first < the_first_jobs_last,
                "**the second job's first frame waited for the first job's whole download** — \
                 {landed_before_it} of the first job's {THE_FIRST_JOBS_FRAMES} files had landed \
                 before it"
            );
        }

        // ═══════════════════════════════════════════════════════════════════
        // Step 10c's ninth-pass fix piece 6 — a cut-off download leaves no
        // partial
        // ═══════════════════════════════════════════════════════════════════

        /// **Every part-file left in the user's own folder**, which is what a
        /// person finds in `.smoozy-staging` after the download has finished.
        fn the_part_files_left_in(folder: &Path) -> Vec<PathBuf> {
            fn under(folder: &Path, out: &mut Vec<PathBuf>) {
                let Ok(entries) = std::fs::read_dir(folder) else {
                    return;
                };
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        under(&path, out);
                    } else if path.parent().and_then(|parent| parent.file_name()).and_then(|name| name.to_str())
                        == Some(smoozy_file_browser::partial::THE_RESERVED_STAGING_FOLDER)
                    {
                        out.push(path);
                    }
                }
            }
            let mut out = Vec::new();
            under(folder, &mut out);
            out
        }

        /// **The fault, as a person meets it: a frame's download is cut off
        /// part way by *Disconnect*, the user connects again, the frame comes
        /// down whole — and no part-file is left in `.smoozy-staging`.**
        ///
        /// The ninth pass on Hyperstack left
        /// `~/Downloads/.smoozy-staging/308fa234…-1.smoozy-part` (522,240
        /// bytes) there after all 240 files had landed and the app had quit.
        /// **Why**: the cut-off fetch ended *interrupted*, which `Server protocol
        /// and transfers` S28a keeps off the sweep's list because the work may
        /// be carried on; the arrangement then fetched the same file again under
        /// a new transfer identity (`Download` P3) and never said it had given
        /// the first one up. **That clause's requirement on the module that abandons**
        /// — write the old record *stopped, abandoned by its request* before the
        /// replacement is arranged — was not kept, so nothing ever removed it.
        ///
        /// Two periods, as the ninth pass had them: the first cut off mid-file,
        /// the second a new connection after a *Disconnect* and *Connect*, which
        /// `Server protocol and transfers` C11 step 4 does not resume on.
        #[test]
        fn a_download_cut_off_by_a_disconnect_leaves_no_part_file_once_it_lands() {
            let Some(program) = the_machines_own_file_transfer_server() else {
                println!(
                    "SKIPPED: no OpenSSH sftp-server on this system ({}). A cut-off download \
                     leaving no partial is unproved here.",
                    std::env::consts::OS
                );
                return;
            };
            /// Big enough that the line is cut well inside it.
            const THE_FRAMES_SIZE: usize = 4 * 1024 * 1024;
            const THE_LINE_IS_CUT_AFTER: u64 = 1024 * 1024;

            let desk = TheDesk::new("cut");
            let job = a_job_submitted_unread(&desk, "p12h9 long", 1);
            // **The job rendered and ended**; its one frame is on the machine.
            the_render_starts(&desk, &job);
            the_machine_writes_the_recorded_reading(&job);
            the_machine_writes_the_attempt_record(&job);
            let (_, the_frame) = job.frames[0].clone();
            std::fs::create_dir_all(the_frame.parent().expect("a folder")).expect("the folder");
            std::fs::write(
                &the_frame,
                (0..THE_FRAMES_SIZE).map(|at| (at % 251) as u8).collect::<Vec<u8>>(),
            )
            .expect("the frame is written");
            the_job_ends(&desk, &job);

            let the_run = TheRunsTransfers::for_this_run();
            let additional = nothing_more();

            // **The first period: the frame starts coming down, and the line is
            // cut part way through it** — the user's *Disconnect*.
            {
                let sessions: ThisServersSessions<AStandInMachine> =
                    ThisServersSessions::for_one_server();
                let mut machine = desk.the_machine(program.clone(), Duration::ZERO);
                machine
                    .the_line_is_cut_after
                    .store(THE_LINE_IS_CUT_AFTER, std::sync::atomic::Ordering::SeqCst);
                a_runtime().block_on(async {
                    let passes = async {
                        the_connection_is_lent(&sessions).await;
                        let mut holds = TheArrangementsThisConnectionDrives::fresh();
                        let _ = one_pass_over_this_servers_arrangements(
                            &mut holds,
                            &sessions,
                            &the_run,
                            &desk.record,
                            &desk.server,
                            &desk.layout,
                            &desk.run,
                        )
                        .await;
                    };
                    tokio::select! {
                        _ = lend_until_it_ends(&sessions, &mut machine, &additional) => {
                            panic!("the stand-in's connection never ends on its own")
                        }
                        () = passes => {}
                    }
                });
            }
            assert!(
                what_came_home(&job).is_empty(),
                "the line was cut before the frame could land, or the test proves nothing"
            );
            // **The cut was mid-file**: the first fetch's record ended
            // *interrupted* with bytes staged, which is the ninth pass's shape.
            let the_first_fetch = {
                let mut every = Vec::new();
                desk.record
                    .0
                    .lock()
                    .map(|held| {
                        every = crate::record::transfer_records::all(&held).expect("the records read");
                    })
                    .expect("the record opens");
                every
            };
            assert_eq!(the_first_fetch.len(), 1, "one fetch was arranged: {the_first_fetch:?}");
            assert!(
                matches!(
                    the_first_fetch[0].ending,
                    Some(smoozy_server_protocol::transfers::Ending::Interrupted)
                        | Some(smoozy_server_protocol::transfers::Ending::Stopped(
                            smoozy_server_protocol::transfers::StoppedReason::AbandonedByItsRequest
                        ))
                ),
                "the cut-off fetch ended interrupted, or was given up by its arrangement: {:?}",
                the_first_fetch[0].ending
            );
            assert!(
                !the_first_fetch[0].checkpoints.is_empty()
                    || the_first_fetch[0].staging_identity.is_some(),
                "the first fetch staged part of the frame before the line was cut"
            );

            // **The second period: *Connect* again.** A new connection, a new
            // point, and the arrangement resumes on it (`Download` R12).
            {
                let sessions: ThisServersSessions<AStandInMachine> =
                    ThisServersSessions::for_one_server();
                let mut machine = desk.the_machine(program, Duration::ZERO);
                a_runtime().block_on(async {
                    let passes = async {
                        the_connection_is_lent(&sessions).await;
                        let mut holds = TheArrangementsThisConnectionDrives::fresh();
                        // **Bounded**: a few passes, and never a wait with no
                        // end.
                        for _ in 0..4 {
                            if what_came_home(&job).len() == 1 {
                                break;
                            }
                            let _ = one_pass_over_this_servers_arrangements(
                                &mut holds,
                                &sessions,
                                &the_run,
                                &desk.record,
                                &desk.server,
                                &desk.layout,
                                &desk.run,
                            )
                            .await;
                        }
                    };
                    tokio::select! {
                        _ = lend_until_it_ends(&sessions, &mut machine, &additional) => {
                            panic!("the stand-in's connection never ends on its own")
                        }
                        () = passes => {}
                    }
                });
            }

            let home = what_came_home(&job);
            assert_eq!(home.len(), 1, "the frame came down on the new connection");
            assert_eq!(
                std::fs::metadata(&home[0].0).expect("the frame").len(),
                THE_FRAMES_SIZE as u64,
                "and it came down whole"
            );
            let left = the_part_files_left_in(&job.destination);
            assert!(
                left.is_empty(),
                "**a part-file is left in the user's own folder after the download finished** — \
                 the ninth pass's fault 5: {left:?}"
            );
            // **And the record says why**: the cut-off fetch was given up by
            // its arrangement (`Server protocol and transfers` S28a), and its
            // removal was confirmed, so no cleanup is owed (`Server protocol and
            // transfers` S27).
            let mut the_cut_off_fetch = None;
            desk.record
                .0
                .lock()
                .map(|held| {
                    the_cut_off_fetch = crate::record::transfer_records::read(
                        &held,
                        &the_first_fetch[0].transfer,
                    )
                    .expect("the record reads");
                })
                .expect("the record opens");
            let the_cut_off_fetch = the_cut_off_fetch.expect("the first fetch's record stands");
            assert_eq!(
                the_cut_off_fetch.ending,
                Some(smoozy_server_protocol::transfers::Ending::Stopped(
                    smoozy_server_protocol::transfers::StoppedReason::AbandonedByItsRequest
                ))
            );
            assert!(!the_cut_off_fetch.cleanup_is_owed);
        }

        // ═══════════════════════════════════════════════════════════════════
        // Step 10c's transfer wave piece 3 — downloads come back on every
        // connection (the tenth-pass map's piece 1, item (ii))
        // ═══════════════════════════════════════════════════════════════════

        /// **A connection lent to the stand-in machine on a thread of its own,
        /// the way a period's own thread lends one**, until `stop` is sent —
        /// which is the user's *Disconnect* taking the connection away.
        fn a_connection_is_lent_until_stopped(
            sessions: ThisServersSessions<AStandInMachine>,
            mut machine: AStandInMachine,
            stop: tokio::sync::oneshot::Receiver<()>,
        ) -> std::thread::JoinHandle<()> {
            std::thread::spawn(move || {
                let additional = nothing_more();
                a_runtime().block_on(async {
                    tokio::select! {
                        _ = lend_until_it_ends(&sessions, &mut machine, &additional) => {}
                        _ = stop => {}
                    }
                });
            })
        }

        /// **Bounded**: the lending begins at its first poll.
        fn until_it_is_lent(sessions: &ThisServersSessions<AStandInMachine>) {
            let began = std::time::Instant::now();
            while !sessions.a_connection_is_live() {
                assert!(began.elapsed() < Duration::from_secs(10), "the connection was never lent");
                std::thread::sleep(Duration::from_millis(2));
            }
        }

        /// **The fault, as a person meets it: after a *Disconnect* and a
        /// *Connect* made while the last period's downloads are still in
        /// their pause, the downloads never come back, and nothing says so.**
        ///
        /// The tenth pass on Hyperstack connected again 18 s after a
        /// *Disconnect* and no frame came home for ten minutes, with the rows
        /// reading *ended · 160 / 240* and nothing waiting or wrong. **What
        /// this drives is the period's own guard, through the function the
        /// period's downloads thread runs** (`this_connection_drives_its_arrangements`):
        /// the first period's driving sleeps in `Download` W2's pause when the
        /// user disconnects; the frames are written on the machine while no
        /// connection stands; and the user connects again within that pause.
        /// **Until this piece the second period's first connection took the
        /// first period's number again**, the first driving still held the
        /// guard under that number, and the second connection's downloads were
        /// refused. The first driving then woke, found its connection gone,
        /// ended, and took the guard with it — so nothing drove a download for
        /// the rest of the second period.
        #[test]
        fn downloads_come_back_on_a_connect_made_while_the_last_periods_downloads_sleep() {
            let Some(program) = the_machines_own_file_transfer_server() else {
                println!(
                    "SKIPPED: no OpenSSH sftp-server on this system ({}). Downloads coming \
                     back on every connection are unproved here.",
                    std::env::consts::OS
                );
                return;
            };
            /// **Past one `Download` W2 pause of one arrangement, twice over**:
            /// time enough for the second connection's first pass to bring
            /// every frame home, and never a wait with no end.
            const THE_TESTS_OWN_BOUND: Duration = Duration::from_secs(35);

            let desk = TheDesk::new("again");
            let job = a_job_submitted_unread(&desk, "p12h10 long2", 6);
            let register = super::a_register();
            let holds = register.the_machines_work();
            let server = desk.server.as_text();
            let the_run = TheRunsTransfers::for_this_run();

            // **The render has started; nothing has been written yet.**
            the_render_starts(&desk, &job);
            the_machine_writes_the_recorded_reading(&job);
            the_machine_writes_the_attempt_record(&job);

            std::thread::scope(|scope| {
                // **The user's *Connect*: a period, and its first connection.**
                assert!(register.a_period_has_begun(&server, &super::a_period(&server)));
                let first: ThisServersSessions<AStandInMachine> = ThisServersSessions::for_one_server();
                let the_first_machine = desk.the_machine(program.clone(), Duration::ZERO);
                let standing = Arc::clone(&the_first_machine.sessions);
                let (disconnect, disconnected) = tokio::sync::oneshot::channel();
                let lending = a_connection_is_lent_until_stopped(first.clone(), the_first_machine, disconnected);
                until_it_is_lent(&first);
                let the_first_connection = holds.a_connection_begins(&server);
                let first_driving = scope.spawn({
                    let first = first.clone();
                    let (server, the_run, desk) = (&server, &the_run, &desk);
                    move || {
                        this_connection_drives_its_arrangements(
                            holds,
                            server,
                            the_first_connection,
                            &first,
                            the_run,
                            &desk.record,
                            &desk.server,
                            &desk.layout,
                            &desk.run,
                        )
                    }
                });
                // **The first pass has read the machine**, and the driving is
                // now asleep in `Download` W2's pause.
                let began = std::time::Instant::now();
                while standing.most.load(std::sync::atomic::Ordering::SeqCst) == 0 {
                    assert!(began.elapsed() < Duration::from_secs(10), "the first pass never read the machine");
                    std::thread::sleep(Duration::from_millis(5));
                }
                std::thread::sleep(Duration::from_millis(500));

                // **The user's *Disconnect*.** The connection goes, the break is
                // declared and the period ends, in the order a period's own
                // thread takes them.
                let _ = disconnect.send(());
                lending.join().expect("the first connection's lending ended");
                holds.the_connection_ended(&server);
                let _ = register.a_period_has_ended(&server);

                // **The render goes on with no connection standing**, and ends.
                for frame in 1..=6 {
                    blender_writes_frame(&job, frame);
                }
                the_job_ends(&desk, &job);

                // **The user's *Connect* again, inside the first driving's pause.**
                assert!(register.a_period_has_begun(&server, &super::a_period(&server)));
                let second: ThisServersSessions<AStandInMachine> = ThisServersSessions::for_one_server();
                let (disconnect_again, disconnected_again) = tokio::sync::oneshot::channel();
                let lending_again = a_connection_is_lent_until_stopped(
                    second.clone(),
                    desk.the_machine(program.clone(), Duration::ZERO),
                    disconnected_again,
                );
                until_it_is_lent(&second);
                let the_second_connection = holds.a_connection_begins(&server);
                let second_driving = scope.spawn({
                    let second = second.clone();
                    let (server, the_run, desk) = (&server, &the_run, &desk);
                    move || {
                        this_connection_drives_its_arrangements(
                            holds,
                            server,
                            the_second_connection,
                            &second,
                            the_run,
                            &desk.record,
                            &desk.server,
                            &desk.layout,
                            &desk.run,
                        )
                    }
                });

                // **What a person watches: the frames coming home.** Bounded.
                let began = std::time::Instant::now();
                while what_came_home(&job).len() < 6 && began.elapsed() < THE_TESTS_OWN_BOUND {
                    std::thread::sleep(Duration::from_millis(200));
                }
                let home = what_came_home(&job).len();

                // **Everything this test started ends**: the second connection
                // goes, and both drivings end within one pause.
                let _ = disconnect_again.send(());
                lending_again.join().expect("the second connection's lending ended");
                holds.the_connection_ended(&server);
                let _ = register.a_period_has_ended(&server);
                let the_first_drove = first_driving.join().expect("the first driving ended");
                let the_second_drove = second_driving.join().expect("the second driving ended");

                assert_eq!(
                    home, 6,
                    "**after a Disconnect and a Connect, the downloads never came back**: {home} of \
                     6 frames home after {THE_TESTS_OWN_BOUND:?}"
                );
                assert!(the_first_drove, "the first connection drove its downloads");
                assert!(
                    the_second_drove,
                    "**the second connection's downloads were refused** — the last period's \
                     driving held the guard under the same connection number"
                );
                // **Step 10c's transfer wave piece 4: the driving hands the
                // job's whole to the run** after its passes, where Transfers
                // reads it (`Download` N6).
                assert_eq!(
                    holds.the_whole_of(&job.job.as_text()).map(|whole| whole.files),
                    Some(6),
                    "**the driving never handed the job's whole to the run**"
                );
            });
        }

        // ═══════════════════════════════════════════════════════════════════
        // Step 10c's transfer wave piece 4 — Transfers shows the job's whole
        // ═══════════════════════════════════════════════════════════════════

        /// **The fault, as a person meets it: a 240-frame job renders, six of
        /// its frames are home, and the Transfers row reads *6 / 6* rather
        /// than *6 / 240*.** The tenth pass on Hyperstack read *6 / 73* and
        /// *36 / 188*. The pass has the job's inventory in hand from its first
        /// walk after the recorded reading lands (`Download` N6), and the row
        /// drew the reckoning's length — the files acted on so far.
        ///
        /// **And before any pass in this run has read the inventory, the row
        /// still reads the reckoning's length**, which understates rather than
        /// overstates.
        #[test]
        fn transfers_reads_the_jobs_whole_from_the_first_pass_that_has_its_inventory() {
            let Some(program) = the_machines_own_file_transfer_server() else {
                println!(
                    "SKIPPED: no OpenSSH sftp-server on this system ({}). The job's whole on \
                     Transfers is unproved here.",
                    std::env::consts::OS
                );
                return;
            };
            let desk = TheDesk::new("whole");
            let job = a_job_submitted_unread(&desk, "p12h10 long", 240);
            let sessions: ThisServersSessions<AStandInMachine> = ThisServersSessions::for_one_server();
            let mut machine = desk.the_machine(program, Duration::ZERO);
            let the_run = TheRunsTransfers::for_this_run();
            let additional = nothing_more();
            let what_this_run_holds = crate::servers::the_machines_work::WhatThisRunHolds::default();
            let the_row = |holds: &crate::servers::the_machines_work::WhatThisRunHolds| {
                let mut rows = Vec::new();
                desk.record
                    .0
                    .lock()
                    .map(|held| {
                        rows = crate::surface::transfers::the_requests_the_two_stores_hold(
                            &held,
                            &[crate::surface::transfers::AServerKnown {
                                row: desk.server.as_text(),
                                name: "Hyperstack A6000".into(),
                                badge: None,
                            }],
                        );
                    })
                    .expect("the record opens");
                crate::surface::transfers::the_wholes_this_run_has_read(&mut rows, &|job| {
                    holds.the_whole_of(job)
                });
                let row = rows
                    .into_iter()
                    .find(|row| row.id == job.job.as_text())
                    .expect("the job's auto-download row");
                (row.files_done, row.files_in_the_whole)
            };

            a_runtime().block_on(async {
                let passes = async {
                    the_connection_is_lent(&sessions).await;
                    let mut holds = TheArrangementsThisConnectionDrives::fresh();
                    the_render_starts(&desk, &job);
                    the_machine_writes_the_recorded_reading(&job);
                    the_machine_writes_the_attempt_record(&job);
                    let _ = one_pass_over_this_servers_arrangements(
                        &mut holds,
                        &sessions,
                        &the_run,
                        &desk.record,
                        &desk.server,
                        &desk.layout,
                        &desk.run,
                    )
                    .await;
                    for frame in 1..=7 {
                        blender_writes_frame(&job, frame);
                    }
                    // **Bounded**: a few passes bring the judged frames home.
                    for _ in 0..6 {
                        if what_came_home(&job).len() >= 6 {
                            break;
                        }
                        let _ = one_pass_over_this_servers_arrangements(
                            &mut holds,
                            &sessions,
                            &the_run,
                            &desk.record,
                            &desk.server,
                            &desk.layout,
                            &desk.run,
                        )
                        .await;
                    }
                    // **What the driving does after every pass.**
                    holds.the_wholes_are_handed_to(&what_this_run_holds);
                };
                tokio::select! {
                    _ = lend_until_it_ends(&sessions, &mut machine, &additional) => {
                        panic!("the stand-in's connection never ends on its own")
                    }
                    () = passes => {}
                }
            });

            let (done, _) = the_row(&what_this_run_holds);
            assert_eq!(done, 6, "the six judged frames are home");
            let (_, before_any_pass) = the_row(&crate::servers::the_machines_work::WhatThisRunHolds::default());
            assert!(
                before_any_pass < 240,
                "with no pass in this run, the row reads the reckoning's length: {before_any_pass}"
            );
            assert_eq!(
                the_row(&what_this_run_holds),
                (6, 240),
                "**the Transfers row read the reckoning's length rather than the job's whole**"
            );
        }
    }

    // ── The 38's piece 16: the owed erase ──────────────────────────────────

    mod the_owed_erase {
        use std::collections::VecDeque;
        use std::time::{Duration, SystemTime};

        use smoozy_queue_history::deletion::ADeletion;
        use smoozy_server_protocol::asking_the_supervisor::WhatCameBack;
        use smoozy_server_protocol::framing::{
            Answered, Fields, Request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        };
        use smoozy_server_protocol::requests::{
            IT_WAS_ERASED, IT_WAS_NOT_WHOLLY_ERASED, NOTHING_OF_IT_IS_HERE,
            THE_ERASES_OUTCOME_FIELD, THE_ERASE_REQUEST, THE_JOB_FIELD, THE_UNITS_NOT_DONE_FIELD,
        };
        use smoozy_state::identities::{Identity, IdentityKind};
        use smoozy_state::tombstones::HistoryTombstones;

        use crate::record::deletions::tests::{a_record_in_history, a_record_store};
        use crate::record::deletions::TheDeletions;
        use crate::record::job_records::JobRecords;
        use crate::servers::the_machines_work::AsksTheSupervisor;

        struct ARecordInHand(rusqlite::Connection);

        impl crate::keys::LendsTheRecord for ARecordInHand {
            fn lend(&self, act: &mut dyn FnMut(&rusqlite::Connection)) {
                act(&self.0)
            }
            fn write_a_diary_line(&self, _act: &smoozy_keys::diary::Act) {}
        }

        /// A machine that answers each erase with the next fields in hand.
        struct AMachine {
            asked: Vec<Request>,
            answers: VecDeque<Fields>,
        }

        impl AsksTheSupervisor for AMachine {
            fn ask(&mut self, request: &Request) -> WhatCameBack {
                self.asked.push(request.clone());
                match self.answers.pop_front() {
                    Some(fields) => WhatCameBack::ItAnswered(
                        Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "test")
                            .carrying(fields),
                    ),
                    None => WhatCameBack::NothingCorrelatedCameBack,
                }
            }
        }

        fn the_erase_came_to(word: &str) -> Fields {
            Fields::none().and(THE_ERASES_OUTCOME_FIELD, word)
        }

        fn outcome_records(record: &ARecordInHand) -> i64 {
            record
                .0
                .query_row("SELECT COUNT(*) FROM outcome_record", [], |row| row.get(0))
                .unwrap()
        }

        /// **`Queue & History` X3 and X4: a deletion made while disconnected
        /// is erased on the next connection to a machine holding the job, and
        /// only then marked done** — not by a machine that never held it, and
        /// not while a unit is left.
        #[test]
        fn a_deletion_made_while_disconnected_is_erased_at_the_next_connection_and_only_then_marked_done() {
            let record = ARecordInHand(a_record_store());
            let server = Identity::issue(IdentityKind::LocalServer).unwrap();
            let entry = a_record_in_history(&server, "Kitchen");
            JobRecords::over(&record.0).write_the_provisional_record(&entry).unwrap();
            // **Deleted with no machine connected.**
            let deletion = ADeletion::of(&entry, SystemTime::UNIX_EPOCH + Duration::from_secs(1_754_700_000));
            TheDeletions::over(&record.0).perform(&deletion).unwrap();
            let owed = || TheDeletions::over(&record.0).the_jobs_still_owing_their_erase().unwrap();
            assert_eq!(owed(), vec![entry.job.clone()]);
            let live = || true;

            // **A machine that never held it**: the debt waits, and no record
            // of an act that never ran is left behind.
            let mut elsewhere = AMachine {
                asked: Vec::new(),
                answers: VecDeque::from([the_erase_came_to(NOTHING_OF_IT_IS_HERE)]),
            };
            assert!(super::the_owed_erases_are_sent(&record, &mut elsewhere, &live).is_empty());
            assert_eq!(elsewhere.asked.len(), 1);
            assert_eq!(elsewhere.asked[0].name, THE_ERASE_REQUEST);
            assert_eq!(elsewhere.asked[0].fields.text(THE_JOB_FIELD), Some(entry.job.as_text().as_str()));
            assert_eq!(owed(), vec![entry.job.clone()]);
            assert_eq!(outcome_records(&record), 0);

            // **A unit left**: written, and the debt still stands.
            let mut machine = AMachine {
                asked: Vec::new(),
                answers: VecDeque::from([
                    the_erase_came_to(IT_WAS_NOT_WHOLLY_ERASED)
                        .and(THE_UNITS_NOT_DONE_FIELD, "the hold record naming it"),
                    the_erase_came_to(IT_WAS_ERASED),
                ]),
            };
            assert!(super::the_owed_erases_are_sent(&record, &mut machine, &live).is_empty());
            assert_eq!(owed(), vec![entry.job.clone()], "not marked done while a unit is left");
            let failed: i64 = record
                .0
                .query_row(
                    "SELECT COUNT(*) FROM outcome_record_unit WHERE outcome = 'failed'",
                    [],
                    |row| row.get(0),
                )
                .unwrap_or(-1);
            assert_eq!(failed, 1, "the one unit left is written as failed");

            // **The next connection: a new action, every unit done.**
            let paid = super::the_owed_erases_are_sent(&record, &mut machine, &live);
            assert_eq!(paid, vec![entry.job.clone()]);
            assert!(owed().is_empty(), "marked done only now");
            assert_ne!(machine.asked[0].identity, machine.asked[1].identity, "a new identity");
            let tombstones = JobRecords::over(&record.0).tombstones().unwrap();
            assert!(!tombstones.may_be_filled_in(&entry.job), "it does not come back");

            // **Nothing is owed now**, so nothing is sent.
            super::the_owed_erases_are_sent(&record, &mut machine, &live);
            assert_eq!(machine.asked.len(), 2);
        }
    }
}
