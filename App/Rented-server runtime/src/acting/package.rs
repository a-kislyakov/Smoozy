//! The package transaction's machine half: the claim, the phase record, the two
//! reads the commit rests on, the deciding write, and the inspection evidence.
//!
//! **[`crate::package`] holds the rules** — the closed phase set, G17's branch
//! table, what a rollback restores and what the health exchange produces. This
//! module is the acts.
//!
//! # Which of the transaction's two performers this is
//!
//! **G1b divides them and this module is the first.** A verified runtime that is
//! alive performs its own half of the transaction, and *"that program opens the
//! lock file itself and G1a stands exactly as written, its process exit
//! releasing the claim without a timer"*. **The other performer is the desktop,
//! through the pre-package surface**, on every bare machine and every machine
//! whose supervisor is dead — and there the claim is an exclusive create through
//! the file-transfer subsystem, which is `Server protocol and transfers`' act and
//! is not this crate's.
//!
//! # The lock, and the one property the whole arrangement rests on
//!
//! `flock(2)`, read 2026-09-04 at
//! `https://man7.org/linux/man-pages/man2/flock.2.html`.
//!
//! - *"Place an exclusive lock. Only one process may hold an exclusive lock for
//!   a given file at a given time."*
//! - *"To make a nonblocking request, include LOCK_NB (by ORing) with any of the
//!   above operations."*, the call then failing rather than waiting.
//! - *"Locks created by flock() are associated with an open file description."*
//!   and *"the lock is released either by an explicit LOCK_UN operation on any
//!   of these duplicate file descriptors, or when all such file descriptors have
//!   been closed."*
//!
//! **That last property is the whole reason G1a needs no timer**: a process exit
//! makes a genuinely abandoned transaction claimable, while a live holder remains
//! the owner. G18 refuses a deadline outright, on the ground that a slow live
//! transaction and a dead one read alike.
//!
//! **The descriptor is marked close-on-exec and passed to no child**, which G1a
//! requires in as many words. `F_SETFD` and `FD_CLOEXEC`, read 2026-09-04 at
//! `https://man7.org/linux/man-pages/man2/F_SETFD.2const.html`: *"Set the file
//! descriptor flags to the value specified by arg."*, and *"If the FD_CLOEXEC
//! bit is set, the file descriptor will automatically be closed during a
//! successful execve(2)."* **Without it a worker this supervisor starts inherits
//! the descriptor, and the lock then outlives the transaction that took it** —
//! because the lock is released only when every descriptor referring to the open
//! file description has been closed.
//!
//! # One window this module has and does not paper over
//!
//! **The claim is the exclusive create and the owner's fields are written into
//! the file afterwards**, so there is a window in which the file exists and holds
//! nothing readable. A claimant meeting it is told that a claim stands and that
//! its owner could not be read, and **it changes no phase, no pointer, no
//! staging path and no health result** — which is exactly what G1a says a failed
//! acquisition does. It is a refusal rather than an owner invented to fill the
//! answer.
//!
//! # What is left as written rather than repaired
//!
//! **A claim naming another installation is never taken over, whatever its
//! age.** That is the plan's own Section 16 item 23, and it is the one item in
//! that section waiting on a person. **No function here takes an age or a
//! deadline**, and [`crate::package::phase::claim`] is where the rule lives.

use std::fs::File;
use std::os::fd::AsRawFd;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use smoozy_framing::framing::{
    Answered, Fields, Request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
};
use smoozy_state::identities::{Identity, IdentityKind};

use crate::acting::durable;
use crate::acting::execution_group::ThePairAsTheKernelReportsIt;
use crate::acting::machine::{run, TheMachinesOwnLogin};
use crate::execution_group::{RecordedPair, ValidatesTheRecordedPair};
use crate::machine_kind::MachineKind;
use crate::package::commit::{
    AsksForTheHealthAnswer, HealthAnswer, PerformsTheCommit, ReadsWhatTheCommitRestsOn,
};
use crate::package::phase::{
    claim, Claim, ClaimOutcome, ClaimsTheTransaction, PhaseRecord, TransactionOwner,
    WritesThePhaseRecord,
};
use crate::package::release::{
    InspectionEvidence, ReadsTheDurableRecordsAnInspectionIsComposedFrom, RetainedAdapter,
};
use crate::written_form;

/// The one claim, at one fixed path in the package area.
#[derive(Debug)]
pub struct TheClaimOnTheMachine {
    /// The path G1a and G1b both name — the same one.
    pub at: PathBuf,
    held: Mutex<Option<File>>,
}

impl TheClaimOnTheMachine {
    /// At the package area's own fixed path.
    pub fn at(path: &Path) -> Self {
        TheClaimOnTheMachine {
            at: path.to_path_buf(),
            held: Mutex::new(None),
        }
    }

    /// **The create branch's whole act: fill the file the exclusive create made,
    /// and where any part of that fails, leave no file behind.**
    ///
    /// Each of the three acts can fail, and a file left by a failed one holds
    /// nothing readable for ever: every later attempt then takes the
    /// standing-claim branch, reads zero bytes and refuses with *a claim stands
    /// and its owner could not be read*, while nothing anywhere removes it —
    /// G20's bounded sweep does not reach the claim path and
    /// [`ClaimsTheTransaction::release_at_the_terminal_write`] is reached only by
    /// a caller that acquired. **Nothing is being taken from anybody by the
    /// removal**, because the claim was never taken.
    fn fill_the_created_claim(
        &self,
        file: &File,
        mine: &TransactionOwner,
        phase: crate::package::phase::Phase,
    ) -> Result<(), String> {
        if let Err(why) = self.the_three_acts_that_make_a_claim(file, mine, phase) {
            durable::remove_durably(&self.at).ok();
            return Err(why);
        }
        Ok(())
    }

    /// The three acts that turn the created file into a claim that reads.
    fn the_three_acts_that_make_a_claim(
        &self,
        file: &File,
        mine: &TransactionOwner,
        phase: crate::package::phase::Phase,
    ) -> Result<(), String> {
        pass_it_to_no_child(file)?;
        if !take_the_exclusive_lock(file)? {
            // Nothing can hold a lock on a file this call has just created,
            // so this is said rather than treated as ownership.
            return Err(
                "the claim was created and its lock was refused, which nothing explains".to_string(),
            );
        }
        write_the_claim_into(file, mine, phase)
    }

    /// Whether this program is inside the transaction.
    pub fn is_held_here(&self) -> bool {
        self.held.lock().expect("the claim is not poisoned").is_some()
    }
}

/// **The exclusive lock, taken without waiting.**
///
/// `true` where it was taken, `false` where something else holds it — which
/// G18 reads as live ownership, and never as a deadline having passed.
fn take_the_exclusive_lock(file: &File) -> Result<bool, String> {
    let taken = unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) };
    if taken == 0 {
        return Ok(true);
    }
    let why = std::io::Error::last_os_error();
    match why.raw_os_error() {
        Some(code) if code == libc::EWOULDBLOCK || code == libc::EAGAIN => Ok(false),
        _ => Err(format!("the claim's lock could not be taken: {why}")),
    }
}

/// **G1a's close-on-exec**, so no child this program starts inherits the claim.
fn pass_it_to_no_child(file: &File) -> Result<(), String> {
    let set = unsafe { libc::fcntl(file.as_raw_fd(), libc::F_SETFD, libc::FD_CLOEXEC) };
    if set == -1 {
        return Err(format!(
            "the claim's descriptor could not be marked close-on-exec: {}",
            std::io::Error::last_os_error()
        ));
    }
    Ok(())
}

fn write_the_claim_into(file: &File, mine: &TransactionOwner, phase: crate::package::phase::Phase) -> Result<(), String> {
    use std::io::{Seek, SeekFrom, Write};
    let standing = Claim {
        owner: mine.clone(),
        phase,
    };
    let bytes = written_form::write(&standing).map_err(|why| why.what_the_reader_said)?;
    let mut handle = file.try_clone().map_err(|why| why.to_string())?;
    handle.set_len(0).map_err(|why| why.to_string())?;
    handle
        .seek(SeekFrom::Start(0))
        .map_err(|why| why.to_string())?;
    handle.write_all(&bytes).map_err(|why| why.to_string())?;
    handle.sync_all().map_err(|why| why.to_string())
}

impl ClaimsTheTransaction for TheClaimOnTheMachine {
    fn take(&self, mine: &TransactionOwner) -> Result<ClaimOutcome, String> {
        let mut held = self.held.lock().expect("the claim is not poisoned");
        if held.is_some() {
            return Ok(ClaimOutcome::Acquired);
        }
        if let Some(area) = self.at.parent() {
            durable::make_the_area(area).map_err(|why| why.as_text())?;
        }
        // **The exclusive create is the claim itself**, and it is the same
        // operation on both performers — G1b's, taken here through the machine's
        // own filesystem rather than through the subsystem.
        if let Some(file) = durable::create_only_if_absent(&self.at).map_err(|why| why.as_text())? {
            // **A claim that was not taken leaves no file behind.**
            self.fill_the_created_claim(&file, mine, crate::package::phase::Phase::Inspecting)?;
            *held = Some(file);
            return Ok(ClaimOutcome::Acquired);
        }

        // A claim stands. What is done about it is
        // `crate::package::phase::claim`'s and is not re-decided here.
        let standing = self.what_is_claimed()?;
        match claim(Some(&standing), &mine.installation) {
            ClaimOutcome::InProgress { owner, phase } => {
                Ok(ClaimOutcome::InProgress { owner, phase })
            }
            ClaimOutcome::TakenBackFromThisInstallation => {
                let file = std::fs::OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open(&self.at)
                    .map_err(|why| format!("{}: {why}", self.at.display()))?;
                pass_it_to_no_child(&file)?;
                if !take_the_exclusive_lock(&file)? {
                    // The claim names this installation and a live program of
                    // it is inside the transaction. G1a's release-on-exit is
                    // what tells a later reader otherwise, and nothing here
                    // takes it from a running holder.
                    return Ok(ClaimOutcome::InProgress {
                        owner: standing.owner,
                        phase: standing.phase,
                    });
                }
                write_the_claim_into(&file, mine, standing.phase)?;
                *held = Some(file);
                Ok(ClaimOutcome::TakenBackFromThisInstallation)
            }
            // The rules half answers `Acquired` only for an absent claim, which
            // the branch above has already handled.
            ClaimOutcome::Acquired => Ok(ClaimOutcome::InProgress {
                owner: standing.owner,
                phase: standing.phase,
            }),
        }
    }

    /// **Removed at the terminal write, and by nothing else.**
    ///
    /// **The name goes first and the descriptor second**, so the exclusive lock
    /// is held for the whole of the terminal write — G1a names that as one of
    /// the spans the lock is held for. Closing first would release the lock with
    /// the descriptor, per `flock(2)`, and in that window a second attempt meets
    /// a standing claim, finds the lock free and writes its own claim in; the
    /// removal would then take away a claim that is no longer this one's. **The
    /// unlink is safe while the descriptor is open**, the open file description
    /// outliving the name.
    fn release_at_the_terminal_write(&self) -> Result<(), String> {
        let mut held = self.held.lock().expect("the claim is not poisoned");
        let removed = durable::remove_durably(&self.at).map_err(|why| why.as_text());
        // Closing the descriptor is what releases the lock, per `flock(2)`.
        *held = None;
        removed
    }
}

impl TheClaimOnTheMachine {
    /// What the standing claim says, read as content.
    pub fn what_is_claimed(&self) -> Result<Claim, String> {
        let bytes = durable::read(&self.at)
            .map_err(|why| why.as_text())?
            .ok_or_else(|| format!("{}: no claim stands", self.at.display()))?;
        written_form::read(&bytes).map_err(|why| {
            format!(
                "{}: a claim stands and its owner could not be read: {}",
                self.at.display(),
                why.what_the_reader_said
            )
        })
    }
}

/// The one phase record per machine, in the package area.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThePhaseRecordOnTheMachine {
    /// Where it sits.
    pub at: PathBuf,
}

impl ThePhaseRecordOnTheMachine {
    /// At a path in the package area.
    pub fn at(path: &Path) -> Self {
        ThePhaseRecordOnTheMachine {
            at: path.to_path_buf(),
        }
    }
}

impl WritesThePhaseRecord for ThePhaseRecordOnTheMachine {
    /// **Durable in `State, identity and recovery` §1.3 rule 6's sense**, which
    /// is what G3's *written durably before the act it names* rests on entirely.
    fn write(&self, record: &PhaseRecord) -> Result<(), String> {
        if let Some(area) = self.at.parent() {
            durable::make_the_area(area).map_err(|why| why.as_text())?;
        }
        let bytes = written_form::write(record).map_err(|why| why.what_the_reader_said)?;
        durable::publish(&self.at, &bytes).map_err(|why| why.as_text())
    }

    fn read(&self) -> Result<Option<PhaseRecord>, String> {
        match durable::read(&self.at).map_err(|why| why.as_text())? {
            None => Ok(None),
            Some(bytes) => written_form::read(&bytes)
                .map(Some)
                .map_err(|why| format!("{}: {}", self.at.display(), why.what_the_reader_said)),
        }
    }
}

/// The two things the commit rests on, read as values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheCommitRestsOnAsRead {
    /// The active-package pointer.
    pub pointer: PathBuf,
    /// Where Smoozy's own state area holds the installation identity.
    pub installation_identity: PathBuf,
}

impl ReadsWhatTheCommitRestsOn for WhatTheCommitRestsOnAsRead {
    /// **G11.** The pointer's target, as a value.
    ///
    /// **The link is read rather than followed**, so what comes back is the
    /// name the pointer holds and never a statement that something is there —
    /// which is the whole of *nothing about the commit is inferred from a
    /// directory or a file being there*.
    fn the_active_pointers_value(&self) -> Result<Option<String>, String> {
        match std::fs::read_link(&self.pointer) {
            Ok(target) => Ok(Some(target.display().to_string())),
            Err(why) if why.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(why) => Err(format!("{}: {why}", self.pointer.display())),
        }
    }

    /// **G11's second half.** Read from Smoozy's own state area and never from
    /// a path a provider also writes.
    fn the_installation_identity(&self) -> Result<Option<Identity>, String> {
        let Some(bytes) = durable::read(&self.installation_identity).map_err(|why| why.as_text())?
        else {
            return Ok(None);
        };
        let text = String::from_utf8_lossy(&bytes).trim().to_string();
        Identity::read_back(IdentityKind::Installation, &text)
            .map(Some)
            .ok_or_else(|| {
                format!(
                    "{}: the installation identity does not read back",
                    self.installation_identity.display()
                )
            })
    }
}

/// Write the installation identity where the commit's read expects it.
///
/// **G10 orders it before the pointer switch**, so a machine that has a pointer
/// always has an identity; §14 makes the ordered pair one unit and says that
/// where the identity cannot be written **the pointer is not attempted**.
pub fn make_the_installation_identity_durable(at: &Path, installation: &Identity) -> Result<(), String> {
    if let Some(area) = at.parent() {
        durable::make_the_area(area).map_err(|why| why.as_text())?;
    }
    durable::publish(at, installation.as_text().as_bytes()).map_err(|why| why.as_text())
}

/// The commit's own three acts on the machine.
pub struct TheCommitOnTheMachine {
    /// The active-package pointer, which the deciding write renames over.
    pub pointer: PathBuf,
    /// The kind K1's table placed this machine as, read at this placement.
    pub kind: MachineKind,
    /// The supervisor's unit, by name, on a systemd machine.
    pub unit_name: String,
    /// The service manager's control tool.
    pub the_service_tool: String,
    /// **The machine's own name and number for the login this runs as** (K6,
    /// K7), which is the same value the registration holds.
    ///
    /// **G25's start is the third user-level command this module issues**, and
    /// K7 is exact that on a provider creating no login session the user-level
    /// control tool cannot reach a bus without the exported runtime directory.
    /// Without the login here the value could not be supplied even by a caller
    /// that wanted to.
    pub login: TheMachinesOwnLogin,
    /// Whether this program's own environment already carries the runtime
    /// directory. Where it does not, K7's value is exported by hand.
    pub the_runtime_directory_is_already_set: bool,
    /// **How long the service manager is given to answer, carried from the
    /// caller** (K17).
    pub how_long_a_machine_command_is_given: std::time::Duration,
    /// The running supervisor's recorded execution pair, where there is one.
    /// **A first placement has none and the step is the start alone.**
    pub the_running_supervisor: Option<RecordedPair>,
    /// The reading that validates that pair before anything is sent to it.
    pub kernel: ThePairAsTheKernelReportsIt,
}

impl std::fmt::Debug for TheCommitOnTheMachine {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        out.debug_struct("TheCommitOnTheMachine")
            .field("pointer", &self.pointer)
            .field("kind", &self.kind)
            .finish_non_exhaustive()
    }
}

/// The staging name the pointer's own rename publishes from.
fn the_pointers_staging_name(pointer: &Path) -> PathBuf {
    durable::the_staging_name_for(pointer)
}

impl PerformsTheCommit for TheCommitOnTheMachine {
    /// **G7's single deciding write.**
    ///
    /// The pointer is a link whose target is a value, and the switch is one
    /// rename — *"atomic by `rename(2)`'s own words rather than by an ext4
    /// observation"*. **The staged link is made under a name of its own and
    /// renamed over the pointer**, because a link cannot be repointed in place
    /// and a remove-then-make would be the two writes G7 exists to remove.
    fn rename_the_pointer_to(&self, target: &str) -> Result<(), String> {
        if let Some(area) = self.pointer.parent() {
            durable::make_the_area(area).map_err(|why| why.as_text())?;
        }
        let staging = the_pointers_staging_name(&self.pointer);
        // A staged name left by an interrupted attempt is replaced rather than
        // read: nothing anywhere reads it, and G19 re-validates staged material
        // by content in any case.
        std::fs::remove_file(&staging).ok();
        std::os::unix::fs::symlink(target, &staging)
            .map_err(|why| format!("{}: {why}", staging.display()))?;
        std::fs::rename(&staging, &self.pointer)
            .map_err(|why| format!("{}: {why}", staging.display()))?;
        durable::flush_the_directory_holding(&self.pointer).map_err(|why| why.as_text())
    }

    /// **G25's ending, and it is the supervisor's one deliberate end.**
    ///
    /// **The target is validated before anything is sent to it**, for the same
    /// reason every other target in this crate is: a bare identifier is
    /// re-handed-out, which X2 showed and `State, identity and recovery` §2.1
    /// rule 5 forbids.
    ///
    /// **The polite terminate is the signal**, on the observation of record —
    /// on the container batch a long-lived detached process was sent one, *"the
    /// process logged its own clean ending and exited within two seconds"*. It
    /// is a signal to the process rather than a stop asked of the service
    /// manager, and that is the point: **both restarters act after an exit**
    /// (G25), so an ending that the machine's own restarter does not see is an
    /// ending the transaction cannot recover from.
    ///
    /// **A first placement has no supervisor to end**, and neither has a
    /// machine whose recorded supervisor no longer validates — its ending has
    /// already happened, which is what this step wanted.
    fn end_the_running_supervisor(&self) -> Result<(), String> {
        let Some(recorded) = self.the_running_supervisor else {
            return Ok(());
        };
        let what_is_there = self.kernel.what_is_at(recorded.identifier)?;
        if recorded.validates_against(what_is_there).the_group_is_gone() {
            return Ok(());
        }
        let sent = unsafe { libc::kill(recorded.identifier as libc::pid_t, libc::SIGTERM) };
        if sent == 0 {
            Ok(())
        } else {
            Err(format!(
                "the running supervisor at {} could not be ended: {}",
                recorded.identifier,
                std::io::Error::last_os_error()
            ))
        }
    }

    /// **G25's start**, asked of the restarter §3.2 names for this machine kind.
    ///
    /// **On a systemd machine that is the plain start**, which is also the one
    /// act the plan's own Section 16 item 12 names as the whole of the recovery
    /// after a full state mount. **On a container it is nothing here, and the
    /// plan asks for more**: G25 has the commit invoke S5's keeper itself as
    /// this step, once, and S5 makes a second start of a keeper already running
    /// harmless by the lock the keeper holds on its own file. That start is not
    /// built on either side, for the one reason
    /// `smoozy_server_protocol::placement::registration::start_the_supervisor_the_pointer_names`
    /// gives: the command a shell loop takes the lock with has not had its
    /// publication read. **What this side meets is a replacement**, a live
    /// runtime being what performs it here, so on a container a keeper is
    /// already running and is what starts the supervisor again after the
    /// ending. *(Corrected 2026-09-19: this said a start from here would be the
    /// second restarter §3.2 exists to have one of, which is what the plan said
    /// before 2026-09-12 and the reverse of G25 and S5 since.)*
    ///
    /// **Where the restarter produces no answering supervisor inside the
    /// caller's own deadline, that is G13's *did not answer* branch**, and
    /// nothing here waits, retries or softens it. K17 forbids budgeting a fixed
    /// wait in any case.
    fn start_the_supervisor_the_pointer_names(&self) -> Result<(), String> {
        match self.kind {
            MachineKind::Container => Ok(()),
            MachineKind::Systemd => {
                // **K7's value, the same one the enable and the lingering grant
                // are handed**, so all three user-level commands are given the
                // same thing.
                let said = run(
                    &self.the_service_tool,
                    &["--user", "start", &self.unit_name],
                    &crate::acting::registration::the_environment_a_user_level_command_needs(
                        &self.login,
                        self.the_runtime_directory_is_already_set,
                    ),
                    self.how_long_a_machine_command_is_given,
                )?;
                if said.ended_at_zero {
                    Ok(())
                } else {
                    Err(format!(
                        "the service manager refused to start {}: {}",
                        self.unit_name,
                        said.as_text()
                    ))
                }
            }
        }
    }
}

/// `Server protocol and transfers` B4's inspection evidence, composed from the
/// durable records the placement wrote.
///
/// **No program answers an inspection in either phase** (G31a), so there is no
/// live-program handle here and none is wanted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheInspectionEvidenceOnTheMachine {
    /// The phase record, which carries both descriptors by value.
    pub phase_record: ThePhaseRecordOnTheMachine,
    /// The active-package pointer, whose own value decides which of the two is
    /// the active one.
    pub pointer: PathBuf,
}

impl ReadsTheDurableRecordsAnInspectionIsComposedFrom for TheInspectionEvidenceOnTheMachine {
    /// **Absent everywhere is the never-held answer, and it is a real answer.**
    fn the_inspection_evidence(&self) -> Result<InspectionEvidence, String> {
        let Some(record) = self.phase_record.read()? else {
            return Ok(InspectionEvidence {
                the_active_pointers_descriptor: None,
                retained_adapters: Vec::new(),
            });
        };
        let pointing_at = match std::fs::read_link(&self.pointer) {
            Ok(target) => Some(target.display().to_string()),
            Err(why) if why.kind() == std::io::ErrorKind::NotFound => None,
            Err(why) => return Err(format!("{}: {why}", self.pointer.display())),
        };
        let mut retained_adapters: Vec<RetainedAdapter> =
            record.staged_packages_retained_adapters.clone();
        if let Some(previous) = &record.previous {
            retained_adapters.push(previous.retained_adapter.clone());
        }
        Ok(InspectionEvidence {
            the_active_pointers_descriptor: which_descriptor_the_pointer_names(&record, pointing_at.as_deref()),
            retained_adapters,
        })
    }
}

/// **G31a's own rule, at the path it has always been reached at.**
///
/// **The rule itself moved into the rules half on 2026-09-06** — it is
/// [`crate::package::phase::which_descriptor_the_pointer_names`] — because it
/// reaches no machine and this module is gated to a unix, and
/// `Server protocol and transfers` B4 has the **desktop** compose the same
/// answer on all three of the user's systems. This line is what keeps every
/// existing caller of the old path resolving.
pub use crate::package::phase::which_descriptor_the_pointer_names;

/// **G12's request, by name — spelled once, in the crate both ends read.**
///
/// The request set for an installed package version is declared by package
/// inspection (`Server protocol and transfers` §1.2 item 3); **the name lives at
/// `smoozy_framing::requests` since step 10c's first piece**, because the
/// desktop reads it too and this module is gated to a unix. This line keeps
/// every existing caller of this path resolving.
pub use smoozy_framing::requests::THE_HEALTH_ANSWER_REQUEST;

/// **G12's four things, as the fields of one framed result.** *"It asks for four
/// things and takes nothing else as evidence."*
pub const THE_FOUR_THINGS_THE_HEALTH_EXCHANGE_ASKS_FOR: [&str; 4] = [
    "package version",
    "run",
    "installation",
    "phase",
];

/// **G12's health exchange, over the machine-local endpoint.**
///
/// **One request over the machine-local endpoint to the supervisor the commit has
/// just activated, and it is framed like any other request.** `Security and
/// licence` §8 requirement 3 binds it: *"The endpoint is not a trusted channel
/// that skips validation; it is a channel whose peer is known."*
///
/// **Not answering is an answer.** G13 makes a supervisor that does not answer at
/// all the same failed exchange as one that answers wrongly, and the caller does
/// not distinguish the two — so this returns `Ok(None)` for it rather than an
/// error a caller might treat differently.
///
/// **The deadline is G25's and is carried rather than defaulted.** G25 orders the
/// restarter asked before the exchange is attempted, and *"where the restarter
/// produces no answering supervisor inside the caller's own deadline, that is
/// G13's did not answer branch and nothing here softens it."*
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheHealthExchangeOverTheEndpoint {
    /// Z2's recorded path.
    pub endpoint: crate::acting::endpoint::TheEndpointAsACaller,
    /// G25's own deadline, waiting for the restarter's supervisor to be holding
    /// the endpoint at all.
    pub how_long_the_restarter_is_given: std::time::Duration,
}

impl TheHealthExchangeOverTheEndpoint {
    pub fn at(
        recorded_endpoint_path: &str,
        how_long_a_result_may_take: std::time::Duration,
        how_long_the_restarter_is_given: std::time::Duration,
    ) -> Self {
        TheHealthExchangeOverTheEndpoint {
            endpoint: crate::acting::endpoint::TheEndpointAsACaller::at(
                recorded_endpoint_path,
                how_long_a_result_may_take,
            ),
            how_long_the_restarter_is_given,
        }
    }
}

/// **G12's four things, read off one framed result.**
///
/// **A result missing any of the four, or carrying one that does not read back,
/// is not a health answer**: G12 takes nothing else as evidence, and a partial
/// answer read as a whole one would let G13 compare against a value nobody sent.
/// It is `None`, which G13 reads as the *did not answer* branch — the same
/// failure, which is what that clause says.
pub fn the_health_answer_in(answered: &Answered) -> Option<HealthAnswer> {
    Some(HealthAnswer {
        package_version: answered.fields.text("package version")?.to_string(),
        run: Identity::read_back(IdentityKind::Run, answered.fields.text("run")?)?,
        installation: Identity::read_back(
            IdentityKind::Installation,
            answered.fields.text("installation")?,
        )?,
        phase: serde_json::from_str(answered.fields.text("phase")?).ok()?,
    })
}

/// **G12's four things, put into one framed result** — the supervisor's own side
/// of the exchange, so that the two halves are composed from one description.
pub fn the_health_answer_as_fields(answer: &HealthAnswer) -> Fields {
    Fields::none()
        .and("package version", answer.package_version.as_str())
        .and("run", answer.run.as_text())
        .and("installation", answer.installation.as_text())
        .and(
            "phase",
            serde_json::to_string(&answer.phase).expect("a phase writes back"),
        )
}

impl AsksForTheHealthAnswer for TheHealthExchangeOverTheEndpoint {
    fn ask(&self) -> Result<Option<HealthAnswer>, String> {
        // **`Server protocol and transfers` C1**: the identity is chosen by the
        // initiating side before the request is sent, and this exchange is a
        // pure reading, so it carries one for correlation and reporting rather
        // than for idempotency (`Server protocol and transfers` C6).
        let identity = Identity::issue(IdentityKind::Operation)
            .map_err(|_| "the system would not supply random bytes".to_string())?;
        let request = Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            identity,
            THE_HEALTH_ANSWER_REQUEST,
        );
        let answered = self
            .endpoint
            .ask_waiting_for_a_supervisor(&request, self.how_long_the_restarter_is_given)?;
        Ok(answered.as_ref().and_then(the_health_answer_in))
    }
}

// ------------------------------------------------------- G25's ending, performed

/// **What G25's ending came to, performed by the program the pointer names.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheEndingCameTo {
    /// Nothing held the endpoint: a first placement's case.
    ThereWasNothingToEnd,
    /// The holder already runs the pointer's program, and nothing was ended.
    ItAlreadyRanThePointersProgram { process: u32 },
    /// The holder ran another program, was sent the ending, and let the
    /// endpoint go.
    ItWasEnded { process: u32 },
    /// The holder was sent the ending and still held the endpoint when the
    /// bound ran out. **Said rather than hidden**: G13's exchange then reaches
    /// whichever supervisor answers and decides the placement.
    ItWasSentTheEndingAndStillHoldsIt { process: u32 },
    /// Nothing was ended, and why.
    NothingWasEnded(String),
}

/// **How long the ending waits for the holder to let the endpoint go.**
///
/// **A Build decision and not a deadline of the plan's, and nothing is sized on
/// it.** It keeps the invocation from answering before the old supervisor has
/// gone, which would send the desktop's health exchange to it; and it keeps an
/// invocation on a machine nobody is watching from waiting for ever. The
/// desktop's own deadline on the health exchange is what decides the
/// placement.
pub const HOW_LONG_THE_ENDING_WAITS_FOR_THE_ENDPOINT_TO_BE_LET_GO: std::time::Duration =
    std::time::Duration::from_secs(20);

/// **G25's ending: the endpoint's holder is found from the kernel, and ended
/// where it runs another program than the pointer names.**
///
/// The rule is [`crate::package::commit::what_the_ending_does`]; this is the
/// three readings and the one act, and each carries its publication.
///
/// # The holder, from the kernel
///
/// **Connecting to the endpoint names the process that listens on it.**
/// `unix(7)`, read 2026-09-04 and quoted at the head of
/// [`crate::acting::endpoint`]: the peer-credentials option *"returns the
/// credentials of the peer process connected to this socket"*, and *"The
/// returned credentials are those that were in effect at the time of the call
/// to connect(2), listen(2), or socketpair(2)"* — so for a caller, the listener.
/// **Nothing is sent**: the connection is closed at once, and the holder reads
/// it as a caller that sent no frame, which is one of its ordinary turns.
///
/// # The program each runs, from the kernel
///
/// **On the machine**, `proc_pid_exe(5)`, Linux man-pages 6.19 of 2026-02-08,
/// read 2026-09-28 at
/// `https://man7.org/linux/man-pages/man5/proc_pid_exe.5.html`: *"this file is
/// a symbolic link containing the actual pathname of the executed command.
/// This symbolic link can be dereferenced normally; attempting to open it will
/// open the executable."* **So it is followed, never read as a path**, and the
/// file it reaches is compared by the pair `inode(7)` names, same pages, same
/// day, at `https://man7.org/linux/man-pages/man7/inode.7.html`: the device —
/// *"Each inode (as well as the associated file) resides in a filesystem that
/// is hosted on a device"* — and the inode number, *"Each file in a filesystem
/// has a unique inode number. Inode numbers are guaranteed to be unique only
/// within a filesystem"*. **The pointer's program is the file the pointer's
/// link reaches**, compared the same way. A replaced program whose name was
/// unlinked is still reached, the page adding that such a link *"will contain
/// the string ' (deleted)' appended to the original pathname"* — which is why
/// the name is never compared.
///
/// **The page also says that following the link is subject to the kernel's own
/// access check** (*"governed by a ptrace access mode PTRACE_MODE_READ_FSCREDS
/// check"*). **Where it refuses, nothing is ended**: that is
/// [`crate::package::commit::WhatTheEndingDoes::ItCannotBeTold`].
///
/// **At the desk**, which is macOS and has no such file, the kernel's own call
/// for a process's executable path is taken instead, then followed the same
/// way: `proc_pidpath`, declared in the system's development files and read on
/// 2026-09-28 at `MacOSX.sdk/usr/include/libproc.h` — *"int proc_pidpath(int
/// pid, void * buffer, uint32_t  buffersize)"*, with its room
/// `PROC_PIDPATHINFO_MAXSIZE` from `sys/proc_info.h` — macOS 26.6.2 build
/// 25G83. **It serves the desk's tests and no rented machine.**
///
/// # The ending
///
/// **The signal is the termination signal.** `signal(7)`, same pages, same
/// day, at `https://man7.org/linux/man-pages/man7/signal.7.html`: *"SIGTERM
/// P1990 Term Termination signal"*, and *"Term Default action is to terminate
/// the process."* **No supervisor this project has built sets a disposition
/// for it**, so every program ever placed ends on it. **It is sent to that
/// one process**: `kill(2)`, Linux man-pages 6.19 of 2026-06-05, same day, at
/// `https://man7.org/linux/man-pages/man2/kill.2.html` — *"If pid is positive,
/// then signal sig is sent to the process with the ID specified by pid."* The
/// supervisor's workers are its children and are not signalled, which
/// `Rented-server runtime` K18 is about on a systemd machine and which G21
/// makes moot here: no live or held work depends on the runtime once the
/// commit has been entered.
///
/// **What the ending then waits for is the endpoint being let go**, read the
/// same way the holder was found. A process that has ended has closed the
/// socket it listened on, whether or not its parent has yet taken its exit —
/// `kill(2)` warns that *"an existing process might be a zombie"*, which is why
/// the process's own existence is not what is asked.
pub fn g25s_ending(package_area: &str, recorded_path: &str) -> WhatTheEndingCameTo {
    use crate::package::commit::{what_the_ending_does, TheEndpointsHolder, WhatTheEndingDoes};

    let holder = match the_endpoints_holder(recorded_path) {
        Ok(holder) => holder,
        Err(why) => return WhatTheEndingCameTo::NothingWasEnded(why),
    };
    let named = the_program_the_pointer_names(package_area);
    match what_the_ending_does(&holder, &named) {
        WhatTheEndingDoes::ThereIsNothingToEnd => WhatTheEndingCameTo::ThereWasNothingToEnd,
        WhatTheEndingDoes::ItAlreadyRunsThePointersProgram => {
            let TheEndpointsHolder::AProcess { process, .. } = holder else {
                return WhatTheEndingCameTo::ThereWasNothingToEnd;
            };
            WhatTheEndingCameTo::ItAlreadyRanThePointersProgram { process }
        }
        WhatTheEndingDoes::ItCannotBeTold(why) => WhatTheEndingCameTo::NothingWasEnded(why),
        WhatTheEndingDoes::EndIt { process } => {
            if let Err(why) = the_ending_is_sent_to(process) {
                return WhatTheEndingCameTo::NothingWasEnded(why);
            }
            let started = std::time::Instant::now();
            loop {
                match the_endpoints_holder(recorded_path) {
                    Ok(TheEndpointsHolder::AProcess { process: still, .. }) if still == process => {}
                    // **Nothing holds it, or another process does** — the
                    // restarter's next start, which is the one the pointer
                    // names.
                    Ok(_) => return WhatTheEndingCameTo::ItWasEnded { process },
                    Err(_) => {}
                }
                if started.elapsed() >= HOW_LONG_THE_ENDING_WAITS_FOR_THE_ENDPOINT_TO_BE_LET_GO {
                    return WhatTheEndingCameTo::ItWasSentTheEndingAndStillHoldsIt { process };
                }
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
        }
    }
}

/// **Who holds the endpoint**, by connecting and asking the kernel who
/// listens. A refused or absent path is nothing holding it, which is what
/// Z12's probe reads the same two answers as.
pub fn the_endpoints_holder(
    recorded_path: &str,
) -> Result<crate::package::commit::TheEndpointsHolder, String> {
    use crate::package::commit::TheEndpointsHolder;
    let stream = match std::os::unix::net::UnixStream::connect(recorded_path) {
        Ok(stream) => stream,
        Err(why)
            if matches!(
                why.kind(),
                std::io::ErrorKind::ConnectionRefused | std::io::ErrorKind::NotFound
            ) =>
        {
            return Ok(TheEndpointsHolder::NothingHoldsIt)
        }
        Err(why) => return Err(format!("{recorded_path}: the endpoint could not be reached: {why}")),
    };
    let process = crate::acting::endpoint::the_kernels_answer_about(&stream)?.process;
    drop(stream);
    Ok(TheEndpointsHolder::AProcess {
        process,
        runs: the_file_a_process_runs(process),
    })
}

/// **The file the pointer's link reaches**, compared by device and inode.
pub fn the_program_the_pointer_names(
    package_area: &str,
) -> Result<crate::package::commit::TheFileAProgramIs, String> {
    let pointer = crate::the_program::the_active_package_pointer_at(package_area);
    the_file_at(Path::new(&pointer))
}

fn the_file_at(path: &Path) -> Result<crate::package::commit::TheFileAProgramIs, String> {
    use std::os::unix::fs::MetadataExt;
    let followed = std::fs::metadata(path)
        .map_err(|why| format!("{}: could not be followed: {why}", path.display()))?;
    Ok(crate::package::commit::TheFileAProgramIs {
        device: followed.dev() as u64,
        inode: followed.ino(),
    })
}

/// **The file a process runs, from the kernel** — see [`g25s_ending`].
pub fn the_file_a_process_runs(
    process: u32,
) -> Result<crate::package::commit::TheFileAProgramIs, String> {
    #[cfg(target_os = "linux")]
    {
        the_file_at(Path::new(&format!("/proc/{process}/exe")))
    }
    #[cfg(target_os = "macos")]
    {
        let mut room = vec![0u8; libc::PROC_PIDPATHINFO_MAXSIZE as usize];
        let written = unsafe {
            libc::proc_pidpath(
                process as libc::c_int,
                room.as_mut_ptr() as *mut libc::c_void,
                room.len() as u32,
            )
        };
        if written <= 0 {
            return Err(format!(
                "the kernel did not say what process {process} runs: {}",
                std::io::Error::last_os_error()
            ));
        }
        room.truncate(written as usize);
        the_file_at(Path::new(&String::from_utf8_lossy(&room).into_owned()))
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    {
        Err(format!(
            "this system publishes no reading of what process {process} runs that this program \
             takes"
        ))
    }
}

/// **The termination signal, to that one process** — see [`g25s_ending`].
fn the_ending_is_sent_to(process: u32) -> Result<(), String> {
    let sent = unsafe { libc::kill(process as libc::pid_t, libc::SIGTERM) };
    if sent != 0 {
        return Err(format!(
            "the ending could not be sent to process {process}: {}",
            std::io::Error::last_os_error()
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::package::phase::{a_fresh_attempts_first_write, Phase, PreviousVersion, Route};
    use crate::package::release::{RetainedAdapter, VerifiedReleaseDescriptor};

    fn an_area() -> PathBuf {
        let area = std::env::temp_dir().join(format!(
            "smoozy-runtime-package-{}-{:?}",
            std::process::id(),
            {
                static NEXT: std::sync::atomic::AtomicUsize =
                    std::sync::atomic::AtomicUsize::new(0);
                NEXT.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
            }
        ));
        std::fs::create_dir_all(&area).expect("the area is made");
        area
    }

    fn an_identity(kind: IdentityKind) -> Identity {
        Identity::issue(kind).expect("the desk has randomness")
    }

    fn a_login() -> TheMachinesOwnLogin {
        TheMachinesOwnLogin {
            number: crate::acting::machine::THE_TEST_LOGINS_NUMBER,
            name: "the machines own name".into(),
        }
    }

    /// A service tool that answers as the real one does and records what it was
    /// handed, and what its environment carried.
    ///
    /// **This stands in for the tool and never for the act.** The act is
    /// [`TheCommitOnTheMachine::start_the_supervisor_the_pointer_names`] run
    /// whole; what is replaced is the program at the other end of it.
    fn a_service_tool_at(at: &Path, ends_at: i32) {
        use std::io::Write;
        use std::os::unix::fs::PermissionsExt;
        let mut file = std::fs::File::create(at).expect("the stand-in is written");
        write!(
            file,
            "#!/bin/sh\nprintf '%s\\n' \"$@\" > \"$0.arguments\"\nprintf '%s' \"$XDG_RUNTIME_DIR\" > \"$0.runtime\"\nexit {ends_at}\n"
        )
        .expect("the stand-in is written");
        drop(file);
        let mut permission = std::fs::metadata(at).unwrap().permissions();
        permission.set_mode(0o700);
        std::fs::set_permissions(at, permission).expect("the stand-in may be run");
    }

    fn an_owner(installation: &Identity) -> TransactionOwner {
        TransactionOwner {
            invocation: an_identity(IdentityKind::Operation),
            route: Route::AutomaticRepair,
            process: (std::process::id(), 4242),
            installation: installation.clone(),
        }
    }

    fn a_descriptor(version: &str) -> VerifiedReleaseDescriptor {
        VerifiedReleaseDescriptor {
            manifest_format: "the one format".into(),
            verified_release_sequence: 7,
            requested_package_version: version.into(),
            manifest_digest: [1u8; 32],
            artifact_identity: "the exact artifact".into(),
            artifact_byte_length: 1,
            artifact_digest: [2u8; 32],
        }
    }

    /// **G12's exchange, run over a real endpoint.** The four things go out as
    /// the fields of one framed result and come back as the four things.
    #[test]
    fn the_health_exchange_asks_over_a_real_endpoint_and_gets_the_four_things() {
        use crate::acting::endpoint::as_a_supervisor;
        let path = as_a_supervisor::a_path();
        let installation = an_identity(IdentityKind::Installation);
        let answer = HealthAnswer {
            package_version: "2026.09.04".into(),
            run: an_identity(IdentityKind::Run),
            installation: installation.clone(),
            phase: Phase::CommittedUnverified,
        };
        let supervisor = as_a_supervisor::answering(&path, 1, {
            let answer = answer.clone();
            move |request| {
                assert_eq!(request.name, THE_HEALTH_ANSWER_REQUEST);
                Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "2026.09.04")
                    .carrying(the_health_answer_as_fields(&answer))
            }
        });

        let exchange = TheHealthExchangeOverTheEndpoint::at(
            &path,
            std::time::Duration::from_millis(2000),
            std::time::Duration::from_millis(2000),
        );
        assert_eq!(exchange.ask().unwrap(), Some(answer));
        supervisor.join().unwrap();
        std::fs::remove_file(&path).ok();
    }

    /// **G12 takes nothing else as evidence**, so a result carrying three of the
    /// four is not a health answer at all — and G13 reads that as the *did not
    /// answer* branch, which is the same failure as answering wrongly.
    #[test]
    fn a_result_carrying_three_of_the_four_is_not_a_health_answer() {
        use crate::acting::endpoint::as_a_supervisor;
        let path = as_a_supervisor::a_path();
        let installation = an_identity(IdentityKind::Installation);
        let supervisor = as_a_supervisor::answering(&path, 1, move |request| {
            Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "2026.09.04").carrying(
                Fields::none()
                    .and("package version", "2026.09.04")
                    .and("installation", installation.as_text())
                    .and(
                        "phase",
                        serde_json::to_string(&Phase::Verified).unwrap(),
                    ),
            )
        });

        let exchange = TheHealthExchangeOverTheEndpoint::at(
            &path,
            std::time::Duration::from_millis(2000),
            std::time::Duration::from_millis(2000),
        );
        assert_eq!(exchange.ask().unwrap(), None);
        supervisor.join().unwrap();
        std::fs::remove_file(&path).ok();
    }

    /// **A supervisor that does not answer at all is a failed exchange**, and the
    /// caller does not distinguish it from one that answered wrongly: both
    /// restore. This is the whole route, from nothing at the path to G13's own
    /// verdict.
    #[test]
    fn a_supervisor_that_never_arrives_is_the_same_failed_exchange_as_a_wrong_answer() {
        use crate::acting::endpoint::as_a_supervisor;
        use crate::package::commit::{the_health_exchange, HealthExchange};
        let path = as_a_supervisor::a_path();
        let area = an_area();
        let installation = an_identity(IdentityKind::Installation);
        let record = a_fresh_attempts_first_write(
            an_identity(IdentityKind::Operation),
            a_descriptor("2026.09.04"),
            None,
            MachineKind::Systemd,
            area.join("staging").display().to_string(),
        );

        let exchange = TheHealthExchangeOverTheEndpoint::at(
            &path,
            std::time::Duration::from_millis(150),
            std::time::Duration::from_millis(150),
        );
        let answered = exchange.ask().expect("the system let it through");
        assert_eq!(answered, None);
        assert_eq!(
            the_health_exchange(&record, answered.as_ref(), &installation),
            HealthExchange::Failed
        );
        std::fs::remove_dir_all(&area).ok();
    }

    #[test]
    fn a_first_claimant_acquires_and_the_file_holds_its_own_owner() {
        let area = an_area();
        let claim_file = TheClaimOnTheMachine::at(&area.join("claim"));
        let installation = an_identity(IdentityKind::Installation);
        let mine = an_owner(&installation);
        assert_eq!(claim_file.take(&mine).unwrap(), ClaimOutcome::Acquired);
        assert!(claim_file.is_held_here());
        assert_eq!(claim_file.what_is_claimed().unwrap().owner, mine);
        std::fs::remove_dir_all(&area).ok();
    }

    /// **A claim naming another installation is never taken over, whatever its
    /// age** — the plan's own Section 16 item 23, performed rather than argued.
    /// No function here takes an age or a deadline.
    #[test]
    fn a_claim_naming_another_installation_is_never_taken_over() {
        let area = an_area();
        let at = area.join("claim");
        let somebody_else = an_identity(IdentityKind::Installation);
        {
            let theirs = TheClaimOnTheMachine::at(&at);
            theirs.take(&an_owner(&somebody_else)).unwrap();
            // Their process ends: `flock(2)` releases the lock with the
            // descriptor, and the claim file stays.
        }
        let mine = TheClaimOnTheMachine::at(&at);
        let outcome = mine.take(&an_owner(&an_identity(IdentityKind::Installation))).unwrap();
        match outcome {
            ClaimOutcome::InProgress { owner, phase } => {
                assert_eq!(owner.installation, somebody_else);
                assert_eq!(phase, Phase::Inspecting);
            }
            other => panic!("{other:?}"),
        }
        assert!(!mine.is_held_here());
        std::fs::remove_dir_all(&area).ok();
    }

    /// **The one take-over G1b permits**, and it needs no timer.
    #[test]
    fn a_claim_naming_this_installation_is_taken_back() {
        let area = an_area();
        let at = area.join("claim");
        let installation = an_identity(IdentityKind::Installation);
        {
            let earlier = TheClaimOnTheMachine::at(&at);
            earlier.take(&an_owner(&installation)).unwrap();
        }
        let mine = TheClaimOnTheMachine::at(&at);
        let now = an_owner(&installation);
        assert_eq!(
            mine.take(&now).unwrap(),
            ClaimOutcome::TakenBackFromThisInstallation
        );
        assert!(mine.is_held_here());
        assert_eq!(mine.what_is_claimed().unwrap().owner, now);
        std::fs::remove_dir_all(&area).ok();
    }

    /// **A live holder of this installation's own claim is not taken from.**
    /// The lock is what says so, and it is refused without waiting.
    #[test]
    fn a_live_holder_of_this_installations_claim_keeps_it() {
        let area = an_area();
        let at = area.join("claim");
        let installation = an_identity(IdentityKind::Installation);
        let live = TheClaimOnTheMachine::at(&at);
        let theirs = an_owner(&installation);
        live.take(&theirs).unwrap();

        let second = TheClaimOnTheMachine::at(&at);
        match second.take(&an_owner(&installation)).unwrap() {
            ClaimOutcome::InProgress { owner, .. } => assert_eq!(owner, theirs),
            other => panic!("{other:?}"),
        }
        assert!(!second.is_held_here());
        std::fs::remove_dir_all(&area).ok();
    }

    #[test]
    fn the_claim_is_removed_at_the_terminal_write_and_by_nothing_else() {
        let area = an_area();
        let at = area.join("claim");
        let mine = TheClaimOnTheMachine::at(&at);
        mine.take(&an_owner(&an_identity(IdentityKind::Installation)))
            .unwrap();
        assert!(at.exists());
        mine.release_at_the_terminal_write().unwrap();
        assert!(!at.exists());
        assert!(!mine.is_held_here());
        std::fs::remove_dir_all(&area).ok();
    }

    /// **A claim that was not taken leaves no file behind, and the next asker
    /// gets it.**
    ///
    /// **The write is made to fail for real rather than stood in for**: the
    /// handle the fill is given is open for reading, so the truncate the write
    /// begins with is refused by the machine — the same shape as the disk
    /// refusals that are the only way this branch is reached on a rented
    /// machine, and the only one a desk can produce. What is run is the fill and
    /// its undo whole, at the real path.
    #[test]
    fn a_claim_whose_write_failed_leaves_nothing_behind_and_the_next_asker_gets_it() {
        let area = an_area();
        let at = area.join("claim");
        let installation = an_identity(IdentityKind::Installation);

        let claim = TheClaimOnTheMachine::at(&at);
        std::fs::write(&at, b"").expect("the exclusive create's own empty file");
        let only_readable = std::fs::File::open(&at).expect("it opens for reading");
        let why = claim
            .fill_the_created_claim(
                &only_readable,
                &an_owner(&installation),
                Phase::Inspecting,
            )
            .expect_err("a read-only handle cannot be written");
        drop(only_readable);
        assert!(!why.is_empty());
        assert!(!at.exists(), "a claim that was never taken leaves no file");

        let next = TheClaimOnTheMachine::at(&at);
        assert_eq!(next.take(&an_owner(&installation)).unwrap(), ClaimOutcome::Acquired);
        assert_eq!(next.what_is_claimed().unwrap().owner.installation, installation);
        std::fs::remove_dir_all(&area).ok();
    }

    /// **G25's start on a systemd machine, driven against a stand-in tool.**
    /// The unit is named, and K7's exported runtime-directory value reaches the
    /// command — the same value the enable and the lingering grant are handed.
    #[test]
    fn the_systemd_start_names_the_unit_and_carries_k7s_exported_value() {
        let area = an_area();
        let tool = area.join("the service tool");
        a_service_tool_at(&tool, 0);
        let commit = TheCommitOnTheMachine {
            pointer: area.join("active"),
            kind: MachineKind::Systemd,
            unit_name: "smoozy-supervisor.service".into(),
            the_service_tool: tool.display().to_string(),
            login: a_login(),
            the_runtime_directory_is_already_set: false,
            how_long_a_machine_command_is_given: std::time::Duration::from_secs(30),
            the_running_supervisor: None,
            kernel: ThePairAsTheKernelReportsIt::the_rented_machine(),
        };
        commit
            .start_the_supervisor_the_pointer_names()
            .expect("the service manager started it");
        let handed: Vec<String> = std::fs::read_to_string(format!("{}.arguments", tool.display()))
            .expect("the stand-in recorded its arguments")
            .lines()
            .map(str::to_string)
            .collect();
        assert_eq!(handed, vec!["--user", "start", "smoozy-supervisor.service"]);
        // **The number is the tests' own and not a real login's**, so the value
        // the stand-in saw can only have come from the export. See
        // [`crate::acting::machine::THE_TEST_LOGINS_NUMBER`].
        assert_eq!(
            std::fs::read_to_string(format!("{}.runtime", tool.display())).unwrap(),
            crate::acting::machine::the_test_logins_runtime_directory()
        );
        std::fs::remove_dir_all(&area).ok();
    }

    /// **And where the provider made a session, nothing is exported** — the
    /// start is handed the same nothing the other two user-level commands are.
    ///
    /// **What K7 means by *nothing* is that Smoozy adds nothing**, not that the
    /// command runs without the value. Where the provider made a session the
    /// value is already in this program's own environment, the child inherits
    /// it, and that is the whole point: the control tool reaches the bus over
    /// the session's own directory rather than over a path Smoozy guessed. K7
    /// says outright that the prefix *"stands on the observed box alone"*, so
    /// exporting the derived value over a live session's own would be the fault.
    ///
    /// **So the assertion is that the derived value did not arrive**, and the
    /// number [`crate::acting::machine::THE_TEST_LOGINS_NUMBER`] is what makes
    /// it answerable: no machine's session directory is that one, so the
    /// inherited value can never be mistaken for the exported one. **This test
    /// used to assert the child saw nothing at all, which was a reading of the
    /// desk rather than of the act** — macOS carries no such variable, so it
    /// passed there and failed the first time it ran on Linux.
    #[test]
    fn the_systemd_start_exports_nothing_where_the_provider_made_a_session() {
        let area = an_area();
        let tool = area.join("the service tool");
        a_service_tool_at(&tool, 0);
        let commit = TheCommitOnTheMachine {
            pointer: area.join("active"),
            kind: MachineKind::Systemd,
            unit_name: "smoozy-supervisor.service".into(),
            the_service_tool: tool.display().to_string(),
            login: a_login(),
            the_runtime_directory_is_already_set: true,
            how_long_a_machine_command_is_given: std::time::Duration::from_secs(30),
            the_running_supervisor: None,
            kernel: ThePairAsTheKernelReportsIt::the_rented_machine(),
        };
        commit.start_the_supervisor_the_pointer_names().expect("it started");
        // The tool ran, so the reading below is of a command that happened.
        assert_eq!(
            std::fs::read_to_string(format!("{}.arguments", tool.display()))
                .expect("the stand-in recorded its arguments")
                .lines()
                .collect::<Vec<_>>(),
            vec!["--user", "start", "smoozy-supervisor.service"]
        );
        assert_ne!(
            std::fs::read_to_string(format!("{}.runtime", tool.display())).unwrap(),
            crate::acting::machine::the_test_logins_runtime_directory(),
            "K7's derived value was exported over a session that already had one"
        );
        std::fs::remove_dir_all(&area).ok();
    }

    /// **A service manager that refuses is carried verbatim** and never read as
    /// a start.
    #[test]
    fn a_service_manager_that_refuses_the_start_is_carried_verbatim() {
        let area = an_area();
        let tool = area.join("the service tool");
        a_service_tool_at(&tool, 1);
        let commit = TheCommitOnTheMachine {
            pointer: area.join("active"),
            kind: MachineKind::Systemd,
            unit_name: "smoozy-supervisor.service".into(),
            the_service_tool: tool.display().to_string(),
            login: a_login(),
            the_runtime_directory_is_already_set: false,
            how_long_a_machine_command_is_given: std::time::Duration::from_secs(30),
            the_running_supervisor: None,
            kernel: ThePairAsTheKernelReportsIt::the_rented_machine(),
        };
        let why = commit
            .start_the_supervisor_the_pointer_names()
            .expect_err("it refused");
        assert!(why.contains("ended at 1"));
        std::fs::remove_dir_all(&area).ok();
    }

    #[test]
    fn a_phase_record_is_written_durably_and_read_back_whole() {
        let area = an_area();
        let store = ThePhaseRecordOnTheMachine::at(&area.join("phase"));
        assert_eq!(store.read().unwrap(), None);
        let record = a_fresh_attempts_first_write(
            an_identity(IdentityKind::Operation),
            a_descriptor("0.2.0"),
            None,
            MachineKind::Systemd,
            area.join("staging").display().to_string(),
        );
        store.write(&record).unwrap();
        assert_eq!(store.read().unwrap(), Some(record));
        std::fs::remove_dir_all(&area).ok();
    }

    /// **G7's one deciding write, run.** The pointer names exactly one version
    /// before and after, and the switch is a rename.
    #[test]
    fn the_pointer_switch_is_one_rename_and_names_one_version_throughout() {
        let area = an_area();
        let pointer = area.join("active");
        let commit = TheCommitOnTheMachine {
            pointer: pointer.clone(),
            kind: MachineKind::Container,
            unit_name: "smoozy-supervisor.service".into(),
            the_service_tool: "/no/such/tool".into(),
            login: a_login(),
            the_runtime_directory_is_already_set: false,
            how_long_a_machine_command_is_given: std::time::Duration::from_secs(30),
            the_running_supervisor: None,
            kernel: ThePairAsTheKernelReportsIt::the_rented_machine(),
        };
        let reading = WhatTheCommitRestsOnAsRead {
            pointer: pointer.clone(),
            installation_identity: area.join("installation"),
        };
        assert_eq!(reading.the_active_pointers_value().unwrap(), None);

        commit.rename_the_pointer_to("/smoozy/package/0.1.0").unwrap();
        assert_eq!(
            reading.the_active_pointers_value().unwrap().as_deref(),
            Some("/smoozy/package/0.1.0")
        );

        commit.rename_the_pointer_to("/smoozy/package/0.2.0").unwrap();
        assert_eq!(
            reading.the_active_pointers_value().unwrap().as_deref(),
            Some("/smoozy/package/0.2.0")
        );
        assert!(
            !the_pointers_staging_name(&pointer).exists(),
            "the staged name is what the rename consumed"
        );
        std::fs::remove_dir_all(&area).ok();
    }

    /// **The pointer is read as a value and never followed.** A pointer naming
    /// a version that is not on the machine still reads as that name.
    #[test]
    fn the_pointer_is_read_as_a_value_and_never_as_a_thing_being_there() {
        let area = an_area();
        let pointer = area.join("active");
        std::os::unix::fs::symlink("/a/version/that/is/not/there", &pointer).unwrap();
        let reading = WhatTheCommitRestsOnAsRead {
            pointer,
            installation_identity: area.join("installation"),
        };
        assert_eq!(
            reading.the_active_pointers_value().unwrap().as_deref(),
            Some("/a/version/that/is/not/there")
        );
        std::fs::remove_dir_all(&area).ok();
    }

    /// **G10's ordering, and its read.**
    #[test]
    fn the_installation_identity_is_written_and_read_back_as_itself() {
        let area = an_area();
        let at = area.join("state/installation");
        let reading = WhatTheCommitRestsOnAsRead {
            pointer: area.join("active"),
            installation_identity: at.clone(),
        };
        assert_eq!(reading.the_installation_identity().unwrap(), None);
        let installation = an_identity(IdentityKind::Installation);
        make_the_installation_identity_durable(&at, &installation).unwrap();
        assert_eq!(
            reading.the_installation_identity().unwrap(),
            Some(installation)
        );
        std::fs::remove_dir_all(&area).ok();
    }

    #[test]
    fn an_installation_identity_that_does_not_read_back_is_said_rather_than_guessed() {
        let area = an_area();
        let at = area.join("installation");
        std::fs::write(&at, b"not an identity").unwrap();
        let reading = WhatTheCommitRestsOnAsRead {
            pointer: area.join("active"),
            installation_identity: at,
        };
        let why = reading
            .the_installation_identity()
            .expect_err("it does not read back");
        assert!(why.contains("does not read back"));
        std::fs::remove_dir_all(&area).ok();
    }

    /// **A first placement has no supervisor to end**, and the step is the
    /// start alone.
    #[test]
    fn a_first_placement_ends_nothing() {
        let area = an_area();
        let commit = TheCommitOnTheMachine {
            pointer: area.join("active"),
            kind: MachineKind::Container,
            unit_name: "smoozy-supervisor.service".into(),
            the_service_tool: "/no/such/tool".into(),
            login: a_login(),
            the_runtime_directory_is_already_set: false,
            how_long_a_machine_command_is_given: std::time::Duration::from_secs(30),
            the_running_supervisor: None,
            kernel: ThePairAsTheKernelReportsIt::the_rented_machine(),
        };
        commit.end_the_running_supervisor().expect("there is none");
        commit
            .start_the_supervisor_the_pointer_names()
            .expect("on a container the keeper is the restarter");
        std::fs::remove_dir_all(&area).ok();
    }

    /// **A recorded supervisor that no longer validates is not signalled**, and
    /// the ending this step wanted has already happened.
    ///
    /// **The target here is a live process whose recorded pair does not match
    /// what the kernel reports** — X2's own shape — so what the test proves is
    /// that nothing was sent to it, rather than that nothing happened to a
    /// process that was not there.
    #[test]
    fn a_supervisor_whose_pair_no_longer_validates_is_sent_nothing() {
        let area = an_area();
        let mut somebody_else = std::process::Command::new("/bin/sleep")
            .arg("30")
            .spawn()
            .expect("the desk starts it");
        let kernel = ThePairAsTheKernelReportsIt {
            where_the_kernel_publishes: area.join("proc"),
        };
        // The kernel reports a different start time at that identifier, which
        // is what a re-handed-out identifier looks like.
        crate::acting::execution_group::a_record_in_the_kernels_shape(
            &kernel.where_the_kernel_publishes,
            somebody_else.id(),
            "somebody else",
            'S',
            111,
        );
        let commit = TheCommitOnTheMachine {
            pointer: area.join("active"),
            kind: MachineKind::Container,
            unit_name: "smoozy-supervisor.service".into(),
            the_service_tool: "/no/such/tool".into(),
            login: a_login(),
            the_runtime_directory_is_already_set: false,
            how_long_a_machine_command_is_given: std::time::Duration::from_secs(30),
            the_running_supervisor: Some(RecordedPair {
                identifier: somebody_else.id(),
                start_time: 999,
            }),
            kernel,
        };
        commit.end_the_running_supervisor().expect("it does not validate");
        std::thread::sleep(std::time::Duration::from_millis(100));
        assert_eq!(
            somebody_else.try_wait().expect("the wait is taken"),
            None,
            "nothing was sent to it"
        );
        somebody_else.kill().ok();
        somebody_else.wait().ok();
        std::fs::remove_dir_all(&area).ok();
    }

    /// **The ending, run against a real process**, with the pair validated
    /// first.
    #[test]
    fn a_recorded_supervisor_that_validates_is_ended() {
        let area = an_area();
        let it = std::process::Command::new("/bin/sleep")
            .arg("30")
            .spawn()
            .expect("the desk starts it");
        let pair = RecordedPair {
            identifier: it.id(),
            start_time: 5150,
        };
        let kernel = ThePairAsTheKernelReportsIt {
            where_the_kernel_publishes: area.join("proc"),
        };
        crate::acting::execution_group::a_record_in_the_kernels_shape(
            &kernel.where_the_kernel_publishes,
            pair.identifier,
            "the supervisor",
            'S',
            5150,
        );
        let commit = TheCommitOnTheMachine {
            pointer: area.join("active"),
            kind: MachineKind::Container,
            unit_name: "smoozy-supervisor.service".into(),
            the_service_tool: "/no/such/tool".into(),
            login: a_login(),
            the_runtime_directory_is_already_set: false,
            how_long_a_machine_command_is_given: std::time::Duration::from_secs(30),
            the_running_supervisor: Some(pair),
            kernel,
        };
        commit.end_the_running_supervisor().expect("it is ended");
        let mut it = it;
        let ended = it.wait().expect("it ends");
        assert_eq!(
            crate::acting::execution_group::the_number_an_ending_is_recorded_as(ended),
            -libc::SIGTERM
        );
        std::fs::remove_dir_all(&area).ok();
    }

    fn a_record_with_two_versions(area: &Path) -> PhaseRecord {
        a_fresh_attempts_first_write(
            an_identity(IdentityKind::Operation),
            a_descriptor("0.2.0"),
            Some(PreviousVersion {
                descriptor: a_descriptor("0.1.0"),
                retained_adapter: RetainedAdapter {
                    declared_interval: (1, 2),
                    declared_request_set: vec!["inspect".into()],
                },
                location: area.join("previous").display().to_string(),
                previous_boot_registration_content: b"the previous hook".to_vec(),
            }),
            MachineKind::Systemd,
            area.join("staging").display().to_string(),
        )
    }

    /// **G31a's composed reading**, in its three cases.
    #[test]
    fn the_inspection_reads_which_descriptor_the_pointer_names() {
        let area = an_area();
        let phase_record = ThePhaseRecordOnTheMachine::at(&area.join("phase"));
        let pointer = area.join("active");
        let mut record = a_record_with_two_versions(&area);
        record.staged_packages_retained_adapters = vec![RetainedAdapter {
            declared_interval: (2, 3),
            declared_request_set: vec!["stop".into()],
        }];
        phase_record.write(&record).unwrap();
        let inspection = TheInspectionEvidenceOnTheMachine {
            phase_record,
            pointer: pointer.clone(),
        };

        // The pointer names the staged location: the requested descriptor.
        std::os::unix::fs::symlink(&record.staging_location, &pointer).unwrap();
        let evidence = inspection.the_inspection_evidence().unwrap();
        assert_eq!(
            evidence
                .the_active_pointers_descriptor
                .as_ref()
                .unwrap()
                .requested_package_version,
            "0.2.0"
        );
        assert_eq!(evidence.retained_adapters.len(), 2);

        // The pointer names the previous location: the previous descriptor.
        std::fs::remove_file(&pointer).unwrap();
        std::os::unix::fs::symlink(
            &record.previous.as_ref().unwrap().location,
            &pointer,
        )
        .unwrap();
        assert_eq!(
            inspection
                .the_inspection_evidence()
                .unwrap()
                .the_active_pointers_descriptor
                .unwrap()
                .requested_package_version,
            "0.1.0"
        );

        // The pointer names neither: no descriptor is claimed for it.
        std::fs::remove_file(&pointer).unwrap();
        std::os::unix::fs::symlink("/somewhere/else", &pointer).unwrap();
        assert_eq!(
            inspection
                .the_inspection_evidence()
                .unwrap()
                .the_active_pointers_descriptor,
            None
        );
        std::fs::remove_dir_all(&area).ok();
    }

    /// **Absent everywhere is the never-held answer.**
    #[test]
    fn a_machine_that_has_never_held_a_package_answers_the_never_held_answer() {
        let area = an_area();
        let inspection = TheInspectionEvidenceOnTheMachine {
            phase_record: ThePhaseRecordOnTheMachine::at(&area.join("phase")),
            pointer: area.join("active"),
        };
        let evidence = inspection.the_inspection_evidence().unwrap();
        assert_eq!(evidence.the_active_pointers_descriptor, None);
        assert!(evidence.retained_adapters.is_empty());
        std::fs::remove_dir_all(&area).ok();
    }
}
