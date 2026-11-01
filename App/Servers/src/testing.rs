//! The desk the acting half is driven on.
//!
//! **One machine, one clock, and stand-ins for the five seams of
//! [`crate::acting`]** — so the sequence, the period and the whole standing
//! repair are exercised without a socket, without a rented machine and without
//! waiting for a single one of the repair's own pauses.
//!
//! **Nothing here decides anything.** Every judgement is the module's; what is
//! here answers questions and counts what it was asked.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` C16, F1,
//! F16.
//!
//! **`Server protocol and transfers` A5 is deliberately not listed above and is
//! written out with its document at the site**, because this plan defines an A5
//! of its own and a bare one could mean either.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

use smoozy_server_protocol::transport::ssh::CommandFailure;
use smoozy_server_protocol::transport::{
    CommandResult, CommandString, Credential, DecideHostTrust, Ending, Endpoint, HostTrust,
    OpenFailure,
};
use smoozy_state::identities::InstallationCheck;

use crate::acting::{
    OpenConnections, SupplyTheCredential, TheActingHalf, TheIdentityRead,
    TheInstallationIdentityRead, TheServerBeingConnectedTo, Wait, WhatServersPublishes,
};
use crate::condition::ObservedValue;
use crate::identity::KeyReference;
use crate::key_moments::{TheTwoAnswers, WhoStartedIt};
use crate::readings::CommandChannel;
use crate::repair::{NextAttemptDue, WhatTheRecordSays};
use crate::store::RememberedHostKey;

/// The desktop's own clock, shared by the waiter and by whatever costs time.
///
/// **It moves only when something says it did**, so a run's 105-second span is
/// reproduced exactly rather than approximately.
#[derive(Clone)]
pub struct TheClock(Arc<Mutex<SystemTime>>);

impl TheClock {
    pub fn at_the_epoch() -> TheClock {
        TheClock(Arc::new(Mutex::new(
            SystemTime::UNIX_EPOCH + Duration::from_secs(1_000_000),
        )))
    }

    pub fn now(&self) -> SystemTime {
        *self.0.lock().expect("the clock")
    }

    pub fn advance(&self, by: Duration) {
        let mut held = self.0.lock().expect("the clock");
        *held += by;
    }
}

/// What the machine at the far end answers.
pub struct AMachine {
    pub host_key: Vec<u8>,
    pub answers: HashMap<&'static str, CommandResult>,
    /// Whether each attempt in turn opens. **The last value stands for every
    /// attempt after it**, so a machine that never answers is one `false`.
    pub opens: Vec<bool>,
    /// **K2's report**, already made before the caller looked.
    pub already_ended: Option<Ending>,
    /// What one attempt costs on the desktop's clock.
    pub each_attempt_costs: Duration,
    /// How many live connections break. **The connections after them do not**,
    /// which is the application run ending while a server is connected.
    pub live_connections_that_break: usize,
}

impl AMachine {
    pub const THE_HOST_KEY: &'static [u8] = b"ssh-ed25519 AAAAC3NzaC1lZDI1NTE5";

    /// A machine that answers everything the sequence asks it.
    pub fn answering() -> AMachine {
        let mut answers = HashMap::new();
        for (command, status, output) in [
            ("uname -s", 0, "Linux\n"),
            ("test -e /.dockerenv", 1, ""),
            ("systemd-detect-virt --container", 1, ""),
            ("lspci -d 10de:", 0, "01:00.0 3D controller: NVIDIA\n"),
            (
                "nvidia-smi --query-gpu=uuid,pci.bus_id,name,memory.total --format=csv,noheader",
                0,
                "GPU-aaa, 00000000:01:00.0, NVIDIA GeForce RTX 4090, 24564 MiB\n",
            ),
            ("cat /sys/fs/cgroup/memory.max", 0, "16051527680\n"),
            ("cat /sys/fs/cgroup/memory/memory.limit_in_bytes", 1, ""),
            ("cat /proc/meminfo", 0, "MemTotal:       65805484 kB\n"),
            ("date +%s", 0, "1770000000\n"),
        ] {
            answers.insert(
                command,
                CommandResult {
                    output: output.as_bytes().to_vec(),
                    complaint: Vec::new(),
                    status: Some(status),
                },
            );
        }
        AMachine {
            host_key: AMachine::THE_HOST_KEY.to_vec(),
            answers,
            opens: vec![true],
            already_ended: None,
            each_attempt_costs: Duration::ZERO,
            live_connections_that_break: 1,
        }
    }

    pub fn naming_the_operating_system(mut self, named: &str) -> AMachine {
        self.answers.insert(
            "uname -s",
            CommandResult {
                output: named.as_bytes().to_vec(),
                complaint: Vec::new(),
                status: Some(0),
            },
        );
        self
    }

    /// Q13's third outcome: no answer, a refused command, an unreadable one.
    pub fn saying_nothing_about_its_operating_system(mut self) -> AMachine {
        self.answers.remove("uname -s");
        self
    }

    pub fn whose_connection_has_ended(mut self) -> AMachine {
        self.already_ended = Some(Ending::Announced);
        self
    }

    /// **Nothing ever opens**, which is the opening act's own connection never
    /// becoming live.
    pub fn refusing_for_ever(mut self) -> AMachine {
        self.opens = vec![false];
        self
    }

    /// The opening act's connection comes up and breaks; then `times` attempts
    /// fail and the one after them succeeds.
    pub fn opening_then_refusing(mut self, times: usize) -> AMachine {
        self.opens = std::iter::once(true)
            .chain(std::iter::repeat_n(false, times))
            .chain(std::iter::once(true))
            .collect();
        self
    }

    /// The opening act's connection comes up and breaks, and nothing after it
    /// ever answers — the standing repair's own case.
    pub fn opening_then_refusing_for_ever(mut self) -> AMachine {
        self.opens = vec![true, false];
        self
    }

    /// A connection that is never broken, so the period runs until something
    /// else ends it.
    pub fn whose_connection_never_breaks(mut self) -> AMachine {
        self.live_connections_that_break = 0;
        self
    }
}

/// A connection to the machine above.
pub struct AConnection {
    answers: HashMap<&'static str, CommandResult>,
    commands: Arc<Mutex<Vec<&'static str>>>,
    ending: Arc<Mutex<Option<Ending>>>,
    /// Every path a free-space reading was asked for, so a test can assert
    /// **which** path was read rather than only that a figure came back.
    pub free_space_asked_for: Arc<Mutex<Vec<String>>>,
}

impl AConnection {
    pub fn commands_run(&self) -> usize {
        self.commands.lock().expect("the commands").len()
    }

    pub fn free_space_paths(&self) -> Vec<String> {
        self.free_space_asked_for.lock().expect("the paths").clone()
    }
}

impl CommandChannel for AConnection {
    async fn run(&mut self, command: CommandString) -> Result<CommandResult, CommandFailure> {
        self.commands
            .lock()
            .expect("the commands")
            .push(command.as_str());
        self.answers
            .get(command.as_str())
            .cloned()
            .ok_or(CommandFailure::TheRequestDidNotReachTheMachine)
    }
}

impl crate::readings::TakesAFreeSpaceReading for AConnection {
    /// **The reading names the path it was taken against** (F1), so this
    /// stand-in answers against whatever path it was handed and records it. **A
    /// machine with no recorded root is never asked at all** — the sequence
    /// takes no reading, rather than a stand-in inventing a figure, which is
    /// what H10 and F16 forbid.
    async fn free_space(&mut self, path: &str) -> Option<crate::hardware::FreeSpace> {
        self.free_space_asked_for
            .lock()
            .expect("the paths")
            .push(path.to_string());
        Some(crate::hardware::FreeSpace {
            available_bytes: 64_423_735_296,
            the_path_it_was_taken_against: path.to_string(),
        })
    }
}

/// The transport, stood in for.
///
/// **It calls the caller's comparison the way the real one does** — inside what
/// stands for key exchange, before any credential is counted — so
/// `Server protocol and transfers` A5's ordering is exercised here rather than
/// assumed.
pub struct TheOpener {
    machine: AMachine,
    clock: TheClock,
    attempts: usize,
    credentials: usize,
    tear_downs: usize,
    breaks_left: usize,
    ends_the_period: Option<crate::driving::EndThePeriod>,
    pub deadlines: Vec<Duration>,
}

impl TheOpener {
    /// **A connection nothing breaks ends with the application run**, which is
    /// S10 item 2 and is how a test that drives a whole period stops.
    pub fn when_nothing_more_breaks(&mut self, end: crate::driving::EndThePeriod) {
        self.ends_the_period = Some(end);
    }

    pub fn attempts_made(&self) -> usize {
        self.attempts
    }

    pub fn credentials_presented(&self) -> usize {
        self.credentials
    }

    pub fn torn_down(&self) -> usize {
        self.tear_downs
    }
}

impl OpenConnections for TheOpener {
    type Connection = AConnection;

    async fn open(
        &mut self,
        _endpoint: Endpoint,
        _credential: Credential,
        mut decide: Box<dyn DecideHostTrust>,
        deadline: Duration,
    ) -> Result<AConnection, OpenFailure> {
        self.deadlines.push(deadline);
        let attempt = self.attempts;
        self.attempts += 1;
        // **The deadline handed in is honoured**, which is what makes N2a's
        // abandonment observable at the desk: an attempt whose own cost
        // outruns the deadline it was given ends at the deadline, having
        // changed nothing, exactly as `Server protocol and transfers` C16's
        // expiry does.
        if deadline < self.machine.each_attempt_costs {
            self.clock.advance(deadline);
            return Err(OpenFailure::TheDeadlineExpired);
        }
        self.clock.advance(self.machine.each_attempt_costs);

        // The comparison, at the one moment it can be made before a credential
        // exists.
        if decide.decide(&self.machine.host_key) == HostTrust::Stop {
            return Err(OpenFailure::HostTrustStoppedIt);
        }
        let opens = *self
            .machine
            .opens
            .get(attempt)
            .or_else(|| self.machine.opens.last())
            .unwrap_or(&true);
        if !opens {
            return Err(OpenFailure::TheAddressDidNotAnswer);
        }
        self.credentials += 1;
        Ok(AConnection {
            answers: self.machine.answers.clone(),
            commands: Arc::new(Mutex::new(Vec::new())),
            ending: Arc::new(Mutex::new(self.machine.already_ended)),
            free_space_asked_for: Arc::new(Mutex::new(Vec::new())),
        })
    }

    fn ended(&mut self, connection: &AConnection) -> Option<Ending> {
        *connection.ending.lock().expect("the ending")
    }

    async fn wait_until_it_ends(&mut self, connection: &mut AConnection) -> Ending {
        if let Some(ending) = *connection.ending.lock().expect("the ending") {
            return ending;
        }
        if self.breaks_left > 0 {
            self.breaks_left -= 1;
            // **K2's second bullet** — the case that matters, and the one that
            // went undetected for 21 minutes 45 seconds with no keepalive.
            return Ending::TheKeepaliveWentUnanswered;
        }
        // Nothing more breaks, so the application run is what ends this.
        if let Some(end) = &self.ends_the_period {
            end.at(crate::period::PeriodEnding::TheApplicationRunEnds);
        }
        Ending::TornDownBySmoozy
    }

    async fn tear_down(&mut self, _connection: AConnection) -> Ending {
        self.tear_downs += 1;
        Ending::TornDownBySmoozy
    }
}

/// `Keys`' two answers and its supply.
pub struct TheCredentials {
    pub usable: bool,
    pub a_passphrase_is_available: bool,
    /// **A key can go away between one attempt and the next** (P2), so the
    /// answers are per ask rather than fixed for the run.
    pub usable_for_the_first: Option<usize>,
    pub permanent: bool,
    pub answers_given: usize,
    pub asks: usize,
}

impl SupplyTheCredential for TheCredentials {
    fn the_two_answers(&mut self, _key: &KeyReference) -> TheTwoAnswers {
        self.answers_given += 1;
        let usable = match self.usable_for_the_first {
            Some(first) => self.usable && self.answers_given <= first,
            None => self.usable,
        };
        TheTwoAnswers {
            the_key_is_usable: usable,
            a_passphrase_is_available_without_asking: self.a_passphrase_is_available,
        }
    }

    fn the_refusal_is_permanent(&mut self, _key: &KeyReference) -> crate::repair::PermanentRefusal {
        crate::repair::PermanentRefusal {
            the_store_standing_reads_never_answered: self.permanent,
            the_rows_key_is_protected: self.permanent,
        }
    }

    fn for_one_attempt(
        &mut self,
        _key: &KeyReference,
        _who: WhoStartedIt,
        _naming: TheServerBeingConnectedTo<'_>,
    ) -> Option<Credential> {
        self.asks += 1;
        Some(Credential {
            private_key_file: std::path::PathBuf::from("/nowhere/id_ed25519"),
            passphrase: None,
        })
    }
}

/// **The observed value as it stands right now**, shared between the publisher
/// that writes it and whatever wants to read it at a moment inside a sequence.
///
/// It exists so that *what did a server read while the identity read was being
/// re-taken* can be asked as a question rather than inferred from the order of
/// a list.
#[derive(Clone, Default)]
pub struct TheValueStanding(Arc<Mutex<Option<ObservedValue>>>);

impl TheValueStanding {
    pub fn now(&self) -> Option<ObservedValue> {
        *self.0.lock().expect("the value")
    }

    fn moved_to(&self, value: ObservedValue) {
        *self.0.lock().expect("the value") = Some(value);
    }
}

/// Step 5's read, which belongs to modules built at step 6.
pub struct TheIdentityReader {
    /// The branches it answers with, in order; the last stands for ever.
    pub answers: Vec<InstallationCheck>,
    pub identity: Option<String>,
    pub reads_taken: usize,
    standing: TheValueStanding,
    /// **What the server read at each of this reader's own reads** — Q19's
    /// re-take, watched from inside.
    pub value_at_each_read: Vec<Option<ObservedValue>>,
}

impl TheInstallationIdentityRead<AConnection> for TheIdentityReader {
    async fn read(&mut self, _channel: &mut AConnection) -> TheIdentityRead {
        let check = *self
            .answers
            .get(self.reads_taken)
            .or_else(|| self.answers.last())
            .unwrap_or(&InstallationCheck::Absent);
        self.reads_taken += 1;
        self.value_at_each_read.push(self.standing.now());
        TheIdentityRead {
            check,
            identity_on_the_machine: self.identity.clone(),
        }
    }
}

/// What the module supplies, caught rather than drawn.
pub struct ThePublisher {
    pub observed_values: Vec<ObservedValue>,
    pub next_attempt_due: Vec<NextAttemptDue>,
    pub conditions_raised: Vec<WhatTheRecordSays>,
    /// **The moment each condition was raised, on the desktop's own clock**, so
    /// N2a's *a run fails at 105 seconds* is an assertion rather than a count of
    /// attempts.
    pub conditions_raised_at: Vec<SystemTime>,
    pub conditions_withdrawn: usize,
    pub changed_identity_warnings: usize,
    pub host_keys_recorded: usize,
    clock: TheClock,
    standing: TheValueStanding,
}

impl WhatServersPublishes for ThePublisher {
    fn the_observed_value(&mut self, _server: &str, value: ObservedValue) {
        self.observed_values.push(value);
        self.standing.moved_to(value);
    }

    fn the_next_attempt_due(&mut self, _server: &str, reading: NextAttemptDue) {
        self.next_attempt_due.push(reading);
    }

    fn raise_the_connection_condition(&mut self, _server: &str, says: WhatTheRecordSays) {
        self.conditions_raised.push(says);
        self.conditions_raised_at.push(self.clock.now());
    }

    fn withdraw_the_connection_condition(&mut self, _server: &str) {
        self.conditions_withdrawn += 1;
    }

    fn the_changed_identity_warning(&mut self, _server: &str) {
        self.changed_identity_warnings += 1;
    }

    fn record_the_scanned_host_key(&mut self, _server: &str, _key: RememberedHostKey) {
        self.host_keys_recorded += 1;
    }
}

/// The waiter, which moves the clock rather than the wall.
pub struct TheWaiter {
    clock: TheClock,
    pub waited_for: Vec<Duration>,
    /// **S10 item 1 arriving from outside the task**: the user disconnects
    /// after this many pauses, which is what a repair being stopped by hand
    /// looks like from in here.
    pub ends_the_period_after:
        Option<(usize, crate::driving::EndThePeriod, crate::period::PeriodEnding)>,
    /// **How late each wait wakes**, which a real one does and never early.
    /// Zero unless a test says otherwise; the run's 105 seconds have to land
    /// on a clock where it is not.
    pub wakes_late_by: Duration,
}

impl Wait for TheWaiter {
    async fn until(&mut self, how_long: Duration) {
        self.waited_for.push(how_long);
        self.clock.advance(how_long + self.wakes_late_by);
        if let Some((after, end, ending)) = &self.ends_the_period_after {
            if self.waited_for.len() >= *after {
                end.at(*ending);
            }
        }
    }

    fn now(&mut self) -> SystemTime {
        self.clock.now()
    }
}

/// The five seams, wired to one machine and one clock.
pub struct TheDesk {
    pub half: TheActingHalf<TheOpener, TheCredentials, TheIdentityReader, ThePublisher, TheWaiter>,
    pub clock: TheClock,
}

impl TheDesk {
    pub fn with(machine: AMachine) -> TheDesk {
        let clock = TheClock::at_the_epoch();
        let standing = TheValueStanding::default();
        TheDesk {
            half: TheActingHalf {
                opener: TheOpener {
                    breaks_left: machine.live_connections_that_break,
                    machine,
                    clock: clock.clone(),
                    attempts: 0,
                    credentials: 0,
                    tear_downs: 0,
                    ends_the_period: None,
                    deadlines: Vec::new(),
                },
                credentials: TheCredentials {
                    usable: true,
                    a_passphrase_is_available: true,
                    usable_for_the_first: None,
                    permanent: false,
                    answers_given: 0,
                    asks: 0,
                },
                installation: TheIdentityReader {
                    answers: vec![InstallationCheck::Absent],
                    identity: None,
                    reads_taken: 0,
                    standing: standing.clone(),
                    value_at_each_read: Vec::new(),
                },
                publishes: ThePublisher {
                    observed_values: Vec::new(),
                    next_attempt_due: Vec::new(),
                    conditions_raised: Vec::new(),
                    conditions_raised_at: Vec::new(),
                    conditions_withdrawn: 0,
                    changed_identity_warnings: 0,
                    host_keys_recorded: 0,
                    clock: clock.clone(),
                    standing,
                },
                wait: TheWaiter {
                    clock: clock.clone(),
                    waited_for: Vec::new(),
                    ends_the_period_after: None,
                    wakes_late_by: Duration::ZERO,
                },
            },
            clock,
        }
    }
}
