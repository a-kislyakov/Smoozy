//! Three of this document's own durable stores in the package area: the hold
//! record, the machine-level setup-in-progress state, and the one role worker's
//! enabled-or-disabled desired state.
//!
//! **And, from 2026-10-16, one store whose content is not this document's**:
//! the Telegram worker's state store of `State, identity and recovery` §1.2,
//! whose content `Telegram` owns. It is written here because the supervisor is
//! its one writer — [`TheTelegramWorkersStateOnTheMachine`] says what it holds
//! and why it needs no store-version step.
//!
//! **[`crate::holds`], [`crate::setup_state`] and [`crate::workers`] hold the
//! rules.** This module is the write and the read.
//!
//! # Why these three are written here at all
//!
//! **All three are this document's own and each says so at itself.** U2's own
//! words: a hold record is *"a durable store of this document's, in the package
//! area"*. P1's: *"One machine-level durable state, in the package area, holding
//! whether a setup is in progress and the operation identity of the setup run
//! that set it."* W7's: the role worker is started, restarted and stopped *"from
//! a durable enabled-or-disabled desired state held at machine level"*, which
//! `State, identity and recovery` §1.2's machine-level desired states row gives
//! this document by name.
//!
//! **U3 is what keeps that from being an extension of somebody else's
//! inventory.** The hold record is a machine store kind
//! `State, identity and recovery` §1.2's inventory does not name; it is a seam,
//! recorded rather than written into that inventory from here — and it closed at
//! that stage's own seam close. **The two machine-side stores this crate only
//! reads — the operation record on the machine, and the event journal — are that
//! document's and no form for either is invented here.**
//!
//! # What each write is
//!
//! §14 declares placing a hold **atomic, by staging and one publication**, on
//! its test 2: a half-written hold read later would name a subject with no
//! reason or a reason with no subject. Releasing one is **atomic** on test 1:
//! the single write is what releases, and nothing is started before it.
//! [`crate::acting::durable::publish`] is that staging and that one publication,
//! and it is `State, identity and recovery` §1.3 rule 6's four steps.
//!
//! # Two rules kept at the writer rather than left to a caller
//!
//! **A release outcome is written once and never rewritten** (U2), so a second
//! write is refused here. U13's *the second acts on nothing* is
//! [`crate::holds::apply`]'s and is decided before this store is reached; the
//! refusal below cannot change that outcome and can only catch a caller that
//! never asked.
//!
//! **A setup run that finds the state naming a different run does not clear it**
//! (P8), so a clear by another run is refused here and names the standing
//! identity. **What that costs is recorded rather than hidden:** the plan's own
//! Section 16 item 27 says P8 describes an actor P2 and A11a forbid, and that
//! A5b's admission refusal is the one that actually fires — so this refusal is
//! the belt on a route the admission point already closes, and it invents no new
//! actor.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use smoozy_state::identities::Identity;

use crate::acting::durable;
use crate::holds::{HoldRecord, KeepsHoldRecords, Release};
use crate::setup_state::{
    what_a_setup_run_finding_the_state_does, SetsAndClearsTheState, SetupInProgressState,
    TheStateIsGoneAndReconciliationIsOwed,
    WhatASetupRunFindingTheStateDoes,
};
use crate::workers::{KeepsTheRolesDesiredState, RoleDesiredState};
use crate::written_form;

/// The hold records, one per hold, in the package area.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheHoldRecordsOnTheMachine {
    /// Where the hold records sit, inside the package area.
    pub area: PathBuf,
}

impl TheHoldRecordsOnTheMachine {
    /// In an area, made with the permission the package area's own stores carry.
    pub fn in_the_area(area: &Path) -> Result<Self, String> {
        durable::make_the_area(area).map_err(|why| why.as_text())?;
        Ok(TheHoldRecordsOnTheMachine {
            area: area.to_path_buf(),
        })
    }

    /// **A hold record is reached by its identity and never by listing.**
    ///
    /// `State, identity and recovery` §1.3 rule 5 forbids reading a file's
    /// presence or name as state anywhere in this document, and the plan's own
    /// Section 16 item 3 turns on it: the temporary file an interrupted
    /// publication leaves is read by nothing, precisely because nothing lists
    /// this directory.
    pub fn where_one_sits(&self, hold: &Identity) -> PathBuf {
        self.area.join(format!("{}.hold", hold.as_text()))
    }
}

impl KeepsHoldRecords for TheHoldRecordsOnTheMachine {
    fn place(&self, record: &HoldRecord) -> Result<(), String> {
        let bytes = written_form::write(record).map_err(|why| why.what_the_reader_said)?;
        durable::publish(&self.where_one_sits(&record.hold), &bytes)
            .map_err(|why| why.as_text())
    }

    /// **U12's single write.** It is what releases the subject.
    fn write_the_release(&self, hold: &Identity, release: &Release) -> Result<(), String> {
        let at = self.where_one_sits(hold);
        let mut record = self
            .read(hold)?
            .ok_or_else(|| format!("no hold record stands at {}", hold.as_text()))?;
        if let Some(already) = &record.release {
            return Err(format!(
                "the hold's release outcome is already written: {:?} at {:?}",
                already.outcome,
                already.at.taken_at()
            ));
        }
        record.release = Some(release.clone());
        let bytes = written_form::write(&record).map_err(|why| why.what_the_reader_said)?;
        durable::publish(&at, &bytes).map_err(|why| why.as_text())
    }

    fn read(&self, hold: &Identity) -> Result<Option<HoldRecord>, String> {
        let at = self.where_one_sits(hold);
        match durable::read(&at).map_err(|why| why.as_text())? {
            None => Ok(None),
            Some(bytes) => written_form::read(&bytes)
                .map(Some)
                .map_err(|why| format!("{}: {}", at.display(), why.what_the_reader_said)),
        }
    }
}

/// The machine-level setup-in-progress state.
///
/// **It is one of this document's own two carve-outs from
/// `State, identity and recovery` §1.3 rule 5, and the plan names both since
/// 2026-09-12.** The rule bars reading a file's presence, name or size as state;
/// this store writes *no setup in progress* by taking the record away and reads
/// that absence back as that value. **The ground is one state, one meaning, and
/// no second record that could disagree with it** — and treating absent as
/// *unknown* instead would stop a machine no placement had ever written the
/// record on from reconciling at all, while closing nothing, since a record lost
/// some other way reads as absent whichever way the value is spelt. **The other
/// carve-out is the role's desired state below, and there is no third.**
///
/// **The reconciliation the clear runs is owed rather than held here.** P5 makes
/// the clear two acts in one durable write — the state goes, and reconciliation
/// runs immediately afterwards — and S8 makes reconciliation
/// `State, identity and recovery` §6 entire, taken and not restated. **So it is
/// not this module's to perform**: the clear hands back
/// [`TheStateIsGoneAndReconciliationIsOwed`], and the supervisor performs the
/// second act with it.
///
/// **It was a closure this struct carried until 2026-09-07**, which could not
/// work once the second act existed: reconciliation reads and writes the same
/// supervisor that holds this store, so a closure held here would have had to
/// borrow what holds it.
#[derive(Debug)]
pub struct TheSetupStateOnTheMachine {
    /// Where the state sits, inside the package area.
    pub at: PathBuf,
}

impl SetsAndClearsTheState for TheSetupStateOnTheMachine {
    /// **P4.** Durable before the restart a setup step causes, so the machine
    /// that comes back on its own comes back with the state still set.
    fn set(&self, by_the_run: &Identity) -> Result<(), String> {
        let state = SetupInProgressState {
            the_run_that_set_it: by_the_run.clone(),
        };
        let bytes = written_form::write(&state).map_err(|why| why.what_the_reader_said)?;
        durable::publish(&self.at, &bytes).map_err(|why| why.as_text())
    }

    /// **P5.** The state goes, and reconciliation is owed immediately afterwards.
    ///
    /// **The removal is durable before reconciliation runs**, because a
    /// reconciliation that ran over a state whose removal had not reached the
    /// device would be the one act P2 holds back arriving early on a machine
    /// that then did not come back cleanly. **That ordering is why the second act
    /// is a value handed back rather than something done here**: what this
    /// returns cannot exist before the removal has landed.
    fn clear(
        &self,
        by_the_run: &Identity,
    ) -> Result<TheStateIsGoneAndReconciliationIsOwed, String> {
        if let Some(standing) = self.read()? {
            if let WhatASetupRunFindingTheStateDoes::ReportsTheStateAndItsIdentityAndDoesNotClearIt(
                whose,
            ) = what_a_setup_run_finding_the_state_does(&standing, by_the_run)
            {
                return Err(format!(
                    "the setup-in-progress state names another run, {}, and is not cleared",
                    whose.as_text()
                ));
            }
        }
        durable::remove_durably(&self.at).map_err(|why| why.as_text())?;
        Ok(TheStateIsGoneAndReconciliationIsOwed::the_state_is_gone())
    }

    fn read(&self) -> Result<Option<SetupInProgressState>, String> {
        match durable::read(&self.at).map_err(|why| why.as_text())? {
            None => Ok(None),
            Some(bytes) => written_form::read(&bytes)
                .map(Some)
                .map_err(|why| format!("{}: {}", self.at.display(), why.what_the_reader_said)),
        }
    }
}

/// **W7's machine-level enabled-or-disabled desired state for the one role
/// worker.**
///
/// **The same shape as the setup-in-progress state above** — one durable value
/// in the package area, the supervisor its only writer, written through the
/// ordinary command path (W8) — and `State, identity and recovery` §1.2's
/// machine-level desired states row already names it with this document as its
/// owner, so no inventory is extended here.
///
/// **The removal is what *disabled* is written as**, rather than a second value
/// on the disk, because W7's own reading of an absent value is *disabled*: a
/// machine `Server Setup`'s second entry point has never reached has not been
/// given the role. **One state, one meaning, and no way for the two to
/// disagree.**
///
/// **So this is the second of this document's own two carve-outs from
/// `State, identity and recovery` §1.3 rule 5**, named in the plan's own
/// preamble since 2026-09-12, which had said until then that this document had
/// none. The reasoning is the paragraph above and the same as the
/// setup-in-progress state's; **the rule stands unweakened everywhere else in
/// this crate.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheRoleStateOnTheMachine {
    /// Where the state sits, inside the package area.
    pub at: PathBuf,
}

impl KeepsTheRolesDesiredState for TheRoleStateOnTheMachine {
    /// **W8's write, durable before the supervisor acts on it.**
    fn write(&self, desired: RoleDesiredState) -> Result<(), String> {
        match desired {
            RoleDesiredState::Enabled => {
                let bytes =
                    written_form::write(&desired).map_err(|why| why.what_the_reader_said)?;
                durable::publish(&self.at, &bytes).map_err(|why| why.as_text())
            }
            RoleDesiredState::Disabled => {
                durable::remove_durably(&self.at).map_err(|why| why.as_text())
            }
        }
    }

    /// **An absent value is *disabled*** (W7), and a value that does not read
    /// back is a refusal rather than a guess in either direction — the same
    /// discipline the two stores above take.
    fn read(&self) -> Result<RoleDesiredState, String> {
        match durable::read(&self.at).map_err(|why| why.as_text())? {
            None => Ok(RoleDesiredState::Disabled),
            Some(bytes) => written_form::read(&bytes)
                .map_err(|why| format!("{}: {}", self.at.display(), why.what_the_reader_said)),
        }
    }
}

/// **The Telegram worker's state store, as it is held** — `State, identity and
/// recovery` §1.2's row of that name, whose content is `Telegram`'s and whose
/// one writer is the supervisor. Added 2026-10-16, the Telegram worker's first
/// piece.
///
/// **`Telegram` F2: the worker writes no durable store itself.** Every durable
/// fact it holds lives here and is written by the supervisor at the worker's
/// request, one fact to a request, the whole store published through a sibling
/// and a rename (`State, identity and recovery` §1.3 rule 6) before the answer
/// goes back. **The fields are §1.2's list**: the journal cursor, the Telegram
/// update position, the board, machinery and status-message identities, the
/// 16-byte machine-routing key and the action-context records keyed by their
/// action-context identities.
///
/// **One more is here and it is `Telegram` B9's**: each instruction's recorded
/// outcome, beside its identity. That clause records it *"through its
/// supervisor's command path"* in `Server protocol and transfers` C4's shape —
/// a repeat returns the recorded outcome — and `Telegram` F2 puts every
/// durable fact the worker holds in this store. **It is not a command
/// record**, because an
/// instruction's identity is the board's and not one of the machine's operation
/// identities (`State, identity and recovery` §2.2), so the operation record
/// store, which names its records by such an identity, has nowhere to put it.
///
/// **An action-context record is held as the worker wrote it and never read
/// here.** Its shape is `Telegram` R1's and is that crate's; the supervisor
/// keeps it and hands it back.
///
/// **Every field reads as *nothing* when the written form lacks it**, so a
/// field added later reads back from a store an earlier program wrote without a
/// step under `State, identity and recovery` §1.3 rule 7.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TheTelegramWorkersStateAsHeld {
    /// The next journal sequence the worker reads from — `State, identity and
    /// recovery` E2's cursor, on the reader's side.
    #[serde(default)]
    pub journal_cursor: Option<u64>,
    /// The next Telegram update identity the worker asks from (`Telegram` F3).
    #[serde(default)]
    pub update_position: Option<i64>,
    /// The board this machine last created (`Telegram` B3).
    #[serde(default)]
    pub board: Option<i64>,
    /// This machine's one machinery message (`Telegram` B10).
    #[serde(default)]
    pub machinery_message: Option<i64>,
    /// This machine's one status message, re-used across restarts (`Telegram`
    /// C7).
    #[serde(default)]
    pub status_message: Option<i64>,
    /// **The 16-byte machine-routing key, as text** — issued by the supervisor
    /// *"when this installation first enables the Telegram role"* (`State,
    /// identity and recovery` §2.2), and never issued again once held.
    #[serde(default)]
    pub routing_key: Option<String>,
    /// The action-context records, by their action-context identity, each as
    /// the worker wrote it.
    #[serde(default)]
    pub action_contexts: BTreeMap<String, String>,
    /// Each instruction's recorded outcome, by the instruction's identity —
    /// written once and never rewritten.
    #[serde(default)]
    pub instruction_outcomes: BTreeMap<String, String>,
    /// **`Telegram` R1b — the register of sent messages**, by message
    /// identity, each line as the worker wrote it and never read here. Added
    /// 2026-10-31, the Telegram discussion, the server half.
    #[serde(default)]
    pub sent_messages: BTreeMap<i64, String>,
    /// **`Telegram` V1d — the phone's choice**, as the worker wrote it; the
    /// queue reading carries it (`Queue & History` A1).
    #[serde(default)]
    pub phones_choice: Option<String>,
}

/// **The Telegram worker's state store, in the package area.**
///
/// **An absent store reads as *nothing recorded yet*, and that is not a fifth
/// place `State, identity and recovery` §1.3 rule 5 reads an absence.** The
/// four places that rule names each write one of their values by taking a
/// record away and read the absence back as that value. **Nothing here is ever
/// taken away**: the store is written whole or not at all, so an absent one is
/// one no request has ever written — the same never-held reading a hold record
/// that was never placed gets above, and every fact in it is simply not yet
/// recorded. **A store that is there and does not read back is a refusal**,
/// never an empty store made up.
///
/// **So it needs no store-version step.** `State, identity and recovery` §1.3
/// rule 7 is about a store's shape changing under a newer program; a store no
/// earlier program wrote has no earlier shape, and a placement that brings this
/// program onto a machine finds it absent and reads that as above.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheTelegramWorkersStateOnTheMachine {
    /// Where the store sits, inside the package area.
    pub at: PathBuf,
}

impl TheTelegramWorkersStateOnTheMachine {
    /// The whole store, or *nothing recorded yet* where none was ever written.
    pub fn read(&self) -> Result<TheTelegramWorkersStateAsHeld, String> {
        match durable::read(&self.at).map_err(|why| why.as_text())? {
            None => Ok(TheTelegramWorkersStateAsHeld::default()),
            Some(bytes) => written_form::read(&bytes)
                .map_err(|why| format!("{}: {}", self.at.display(), why.what_the_reader_said)),
        }
    }

    /// **One fact, written**: the store is read, changed by `how`, and
    /// published whole — durable before this returns. **A store that does not
    /// read back is not overwritten**, because writing one fact over it would
    /// lose every other fact it held.
    ///
    /// Answers with the store as it now stands.
    pub fn change(
        &self,
        how: impl FnOnce(&mut TheTelegramWorkersStateAsHeld),
    ) -> Result<TheTelegramWorkersStateAsHeld, String> {
        let mut held = self.read()?;
        how(&mut held);
        let bytes = written_form::write(&held).map_err(|why| why.what_the_reader_said)?;
        durable::publish(&self.at, &bytes).map_err(|why| why.as_text())?;
        Ok(held)
    }

    /// **The routing key, issued once and kept.** Where the store already
    /// holds one it is handed back and nothing is written; where it holds none
    /// a fresh one is drawn by `fresh` and written durably first.
    pub fn the_routing_key_issued_if_absent(
        &self,
        fresh: impl FnOnce() -> Result<String, String>,
    ) -> Result<String, String> {
        if let Some(held) = self.read()?.routing_key {
            return Ok(held);
        }
        let key = fresh()?;
        let held = self.change(|held| held.routing_key = Some(key.clone()))?;
        held.routing_key
            .ok_or_else(|| "the routing key was written and does not read back".to_string())
    }
}

/// **W8's two acts, by name**, each an ordinary command on the same path.
///
/// *"`Server Setup` sets that desired state through the same command path as
/// every other command, and never starts or stops the process."* **They are two
/// names rather than one name with a value inside it**, for the same reason
/// P3's two acts are: a caller asking for something this package does not serve
/// is refused by name.
/// **They are named in `smoozy-framing` and re-exported here at the paths they
/// have always had. Moved 2026-09-20.** `Server Setup` R1 item 8's step is the
/// desktop's, and this crate's acting half is built for the machine alone — so
/// a desktop that must build for Windows could never name what it has to send.
/// **It is the same move and the same reason as
/// [`THE_SETUP_IN_PROGRESS_IS_SET_REQUEST`]'s**, and nothing on this side moved
/// with it.
pub use smoozy_framing::requests::{
    THE_TELEGRAM_ROLE_IS_DISABLED_REQUEST, THE_TELEGRAM_ROLE_IS_ENABLED_REQUEST,
    WHAT_THE_ROLE_WORKER_DID_FIELD,
};

/// **U9's command and the ten names it and its result carry, by name.**
///
/// *"An answer is an ordinary command with its own operation identity, naming
/// the hold identity and carrying the answer."* **U10: both routes arrive at
/// the same command path and there is no second one** — the desk's over SSH
/// through the command adapter and the machine-local endpoint, the phone's
/// submitted by the role worker to the same path.
///
/// **The names are Build's own**, the way the names of this plan's own records
/// in the package area are; what the plan fixes is the command, its three
/// parts, and that there is one path for both routes.
///
/// **They are declared in `smoozy-framing` and re-exported here at the paths
/// they have always had. Moved 2026-09-21.** U10 makes the desk one of the two
/// callers, and this crate's acting half is built for the machine alone — so a
/// desktop that must build for Windows could never name what it has to send.
/// **It is the same move and the same reason as
/// [`THE_SETUP_IN_PROGRESS_IS_SET_REQUEST`]'s**, and nothing on this side moved
/// with it: the dispatch and everything that calls it are untouched.
///
/// **The route's closed set of two moved with the field**, because U10's *there
/// is no second one* is only checkable where all three sit in one place. The
/// same holds for the answer's two values.
///
/// **What did not move is [`the_release_moment_as_text`]**, which composes
/// U14's moment out of this crate's own written form and is the machine's own
/// act rather than a name either side spells.
pub use smoozy_framing::requests::{
    THE_ANSWER_FIELD, THE_ANSWER_IS_NO, THE_ANSWER_IS_YES, THE_ANSWER_TO_A_HOLD_REQUEST,
    THE_HOLD_FIELD, THE_PRODUCED_BY_FIELD, THE_RELEASE_MOMENT_FIELD, THE_RELEASE_OUTCOME_FIELD,
    THE_ROUTE_FIELD, THE_ROUTE_IS_THE_DESK, THE_ROUTE_IS_THE_PHONE,
};

/// **U14's moment, as one field.**
///
/// **The seconds and the nanoseconds are the pair the record's own written form
/// already lays down** (`State, identity and recovery` D5's reading, at
/// [`crate::written_form`]), so the wire and the disk say the same thing about
/// the same moment. The nanoseconds are the part **after** that second and are
/// never negative, so a moment before the usual epoch reads with a negative
/// second and a positive remainder — the ordinary way a clock's own value is
/// written down, and a machine's clock is its own here (nothing corrects one).
pub fn the_release_moment_as_text(at: &smoozy_state::observed::Reading<()>) -> String {
    let (seconds, nanoseconds) = crate::written_form::the_moment_as_a_pair(at.taken_at());
    format!("{seconds}.{nanoseconds:09}")
}

/// **P3's two acts, by name**, each an ordinary command on the same path.
///
/// *"`Server Setup` sets it and clears it through the ordinary command path, and
/// the supervisor never sets or clears it itself."* **They are two names rather
/// than one name with an act inside it** so that a caller asking for something
/// this package does not serve is refused by name, which is the answer
/// `Server protocol and transfers` P5's reserved field carries.
///
/// **The caller is `Server Setup`'s and the state is this document's.** P1 puts
/// the state in the package area, §1.3 rule 1 makes the supervisor its only
/// writer, and the trait above is what that document reaches through.
///
/// **The two names are declared in `smoozy-framing` and re-exported here from
/// step 10c**, where the desktop first sent either: `Server Setup` Y1 sets the
/// class of a run by the act a step performs and `Server Setup` Y7 puts every
/// elevated act on the desktop, so **the side that sends these is a build that
/// cannot reach this crate at all** — the same reason [`THE_SETUP_RUN_FIELD`]
/// moved at piece 8. ~~They used to be declared here.~~ **Both kept their
/// path**, so the dispatch and everything that calls it are untouched.
pub use smoozy_framing::requests::{
    THE_SETUP_IN_PROGRESS_IS_CLEARED_REQUEST, THE_SETUP_IN_PROGRESS_IS_SET_REQUEST,
};
/// **P1's own field: the operation identity of the setup run.** Set or clear,
/// the run names itself — P8 is decided on it, and P3 gives the supervisor no
/// clearing route that does not name a run.
///
/// **It is declared in `smoozy-framing` and re-exported here from step 10c
/// piece 8**, where the desktop first needed it: `Server Setup` Y7's step
/// command and its run-state reading name the same run, and they are sent from
/// a build that cannot reach this crate at all. **It kept its path**, so the
/// two acts above and everything that calls them are untouched.
pub use smoozy_framing::requests::THE_SETUP_RUN_FIELD;

/// **G15a's store version record, in the package area** — `State, identity and
/// recovery` §1.2's last row, whose content that document owns and whose one
/// writer is the supervisor.
///
/// **Its written form is the decimal number and nothing else**, a Build
/// choice: the record holds one number, and a number read back cannot be
/// half a shape. **An absent record reads as nothing**, which that document's
/// rule 7 makes the first version; a record that does not read as a number is
/// a refusal and never a version made up.
#[derive(Debug)]
pub struct TheStoreVersionOnTheMachine {
    /// Where the record sits, inside the package area.
    pub at: PathBuf,
}

impl smoozy_state::store_version::KeepsTheStoreVersion for TheStoreVersionOnTheMachine {
    fn read(&self) -> Result<Option<smoozy_state::store_version::StoreVersion>, String> {
        let Some(bytes) = durable::read(&self.at).map_err(|why| why.as_text())? else {
            return Ok(None);
        };
        String::from_utf8_lossy(&bytes)
            .trim()
            .parse::<u32>()
            .map(|version| Some(smoozy_state::store_version::StoreVersion(version)))
            .map_err(|_| {
                format!(
                    "{}: the store version record does not read as a number",
                    self.at.display()
                )
            })
    }

    fn write(&self, version: smoozy_state::store_version::StoreVersion) -> Result<(), String> {
        durable::publish(&self.at, version.0.to_string().as_bytes()).map_err(|why| why.as_text())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::holds::{Origin, ReleaseOutcome, Subject};
    use crate::setup_state::reconciliation_may_run;
    use smoozy_state::identities::IdentityKind;
    use smoozy_state::observed::{Clock, MachineClock, Reading};
    use std::time::SystemTime;

    fn an_area() -> PathBuf {
        let area = std::env::temp_dir().join(format!(
            "smoozy-runtime-stores-{}-{:?}",
            std::process::id(),
            {
                static NEXT: std::sync::atomic::AtomicUsize =
                    std::sync::atomic::AtomicUsize::new(0);
                NEXT.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
            }
        ));
        area
    }

    fn an_identity(kind: IdentityKind) -> Identity {
        Identity::issue(kind).expect("the desk has randomness")
    }

    fn a_hold() -> HoldRecord {
        HoldRecord {
            hold: an_identity(IdentityKind::Operation),
            subject: Subject::Job(an_identity(IdentityKind::Job)),
            reason: "the first frame has not arrived".into(),
            origin: Origin::Submission,
            dependency: None,
            release: None,
        }
    }

    fn a_release(outcome: ReleaseOutcome) -> Release {
        Release {
            outcome,
            produced_by: "an answer, over TheDesk".into(),
            at: Reading::taken(
                (),
                SystemTime::now(),
                Clock::AMachine(MachineClock("the machine".into())),
            ),
        }
    }

    #[test]
    fn a_hold_is_placed_and_read_back_by_its_identity() {
        let area = an_area();
        let store = TheHoldRecordsOnTheMachine::in_the_area(&area).unwrap();
        let record = a_hold();
        store.place(&record).expect("it is placed");
        assert_eq!(store.read(&record.hold).unwrap(), Some(record));
        std::fs::remove_dir_all(&area).ok();
    }

    #[test]
    fn a_hold_that_was_never_placed_reads_as_nothing() {
        let area = an_area();
        let store = TheHoldRecordsOnTheMachine::in_the_area(&area).unwrap();
        assert_eq!(store.read(&an_identity(IdentityKind::Operation)).unwrap(), None);
        std::fs::remove_dir_all(&area).ok();
    }

    /// **U12's single write releases the subject**, and the record carries which
    /// answer was applied and when — which is what U14 tells the loser.
    #[test]
    fn the_release_is_one_write_and_the_record_carries_it() {
        let area = an_area();
        let store = TheHoldRecordsOnTheMachine::in_the_area(&area).unwrap();
        let record = a_hold();
        store.place(&record).unwrap();
        assert!(crate::holds::blocks_a_package_commit(&record));

        let release = a_release(ReleaseOutcome::Released);
        store.write_the_release(&record.hold, &release).unwrap();
        let back = store.read(&record.hold).unwrap().unwrap();
        assert_eq!(back.release.as_ref().unwrap().outcome, ReleaseOutcome::Released);
        assert!(!crate::holds::blocks_a_package_commit(&back));
        assert!(crate::holds::reconciliation_may_start(&back));
        std::fs::remove_dir_all(&area).ok();
    }

    /// **Written once and never rewritten.**
    #[test]
    fn a_second_release_is_refused_at_the_writer() {
        let area = an_area();
        let store = TheHoldRecordsOnTheMachine::in_the_area(&area).unwrap();
        let record = a_hold();
        store.place(&record).unwrap();
        store
            .write_the_release(&record.hold, &a_release(ReleaseOutcome::Released))
            .unwrap();
        let why = store
            .write_the_release(&record.hold, &a_release(ReleaseOutcome::Stopped))
            .expect_err("it is written once");
        assert!(why.contains("already written"));
        assert_eq!(
            store.read(&record.hold).unwrap().unwrap().release.unwrap().outcome,
            ReleaseOutcome::Released,
            "the first answer stands"
        );
        std::fs::remove_dir_all(&area).ok();
    }

    #[test]
    fn a_release_against_a_hold_that_does_not_stand_is_refused() {
        let area = an_area();
        let store = TheHoldRecordsOnTheMachine::in_the_area(&area).unwrap();
        let why = store
            .write_the_release(
                &an_identity(IdentityKind::Operation),
                &a_release(ReleaseOutcome::Released),
            )
            .expect_err("no hold stands");
        assert!(why.contains("no hold record stands"));
        std::fs::remove_dir_all(&area).ok();
    }

    /// **A hold survives every restart the machine can have** (U24): the record
    /// is on the disk and a fresh reader finds it exactly as it was.
    #[test]
    fn a_hold_is_read_back_by_a_reader_that_did_not_place_it() {
        let area = an_area();
        let record = a_hold();
        {
            let placing = TheHoldRecordsOnTheMachine::in_the_area(&area).unwrap();
            placing.place(&record).unwrap();
        }
        let a_later_run = TheHoldRecordsOnTheMachine::in_the_area(&area).unwrap();
        assert_eq!(a_later_run.read(&record.hold).unwrap(), Some(record));
        std::fs::remove_dir_all(&area).ok();
    }

    fn a_state(area: &Path) -> TheSetupStateOnTheMachine {
        std::fs::create_dir_all(area).unwrap();
        TheSetupStateOnTheMachine {
            at: area.join("setup in progress"),
        }
    }

    #[test]
    fn the_state_is_set_read_back_and_holds_reconciliation() {
        let area = an_area();
        let state = a_state(&area);
        let run = an_identity(IdentityKind::Operation);
        assert!(reconciliation_may_run(state.read().unwrap().as_ref()));
        state.set(&run).unwrap();
        let standing = state.read().unwrap();
        assert_eq!(standing.as_ref().unwrap().the_run_that_set_it, run);
        assert!(!reconciliation_may_run(standing.as_ref()));
        std::fs::remove_dir_all(&area).ok();
    }

    /// **P5's clear is two acts**: the state goes, and reconciliation is owed
    /// immediately afterwards. **The second act is a value that cannot exist
    /// before the removal has landed**, which is the ordering held by the type.
    #[test]
    fn the_clear_takes_the_state_away_and_owes_reconciliation() {
        let area = an_area();
        let state = a_state(&area);
        let run = an_identity(IdentityKind::Operation);
        state.set(&run).unwrap();
        let owed = state.clear(&run).unwrap();
        assert_eq!(state.read().unwrap(), None);
        assert!(reconciliation_may_run(None));
        // **The witness is what the supervisor's own clear moment takes**, and
        // it is consumed here so that this test does not sit on a warning.
        let _: crate::setup_state::TheStateIsGoneAndReconciliationIsOwed = owed;
        std::fs::remove_dir_all(&area).ok();
    }

    /// **P8 at the writer**: another run does not clear it, so nothing is owed
    /// and reconciliation's clear moment is never reached.
    #[test]
    fn a_run_the_state_does_not_name_neither_clears_it_nor_reconciles() {
        let area = an_area();
        let state = a_state(&area);
        let whose = an_identity(IdentityKind::Operation);
        state.set(&whose).unwrap();
        let why = state
            .clear(&an_identity(IdentityKind::Operation))
            .err()
            .expect("it names another run");
        assert!(why.contains(&whose.as_text()));
        assert!(state.read().unwrap().is_some(), "the state still stands");
        std::fs::remove_dir_all(&area).ok();
    }

    /// **P7: nothing clears it on time or on a reconnection**, so a state set
    /// by a run that never came back is still standing for the next reader.
    #[test]
    fn a_state_set_by_a_run_that_never_came_back_is_still_standing() {
        let area = an_area();
        let run = an_identity(IdentityKind::Operation);
        {
            let abandoned = a_state(&area);
            abandoned.set(&run).unwrap();
        }
        let a_later_supervisor = a_state(&area);
        assert_eq!(
            a_later_supervisor.read().unwrap().unwrap().the_run_that_set_it,
            run
        );
        std::fs::remove_dir_all(&area).ok();
    }

    // --- W7's role state ---

    fn a_role_state(area: &Path) -> TheRoleStateOnTheMachine {
        std::fs::create_dir_all(area).unwrap();
        TheRoleStateOnTheMachine {
            at: area.join("telegram-role"),
        }
    }

    /// **W7: a machine nothing has written it on reads as *disabled***, and the
    /// value survives the run that wrote it — which is the whole of what a
    /// durable desired state is for.
    #[test]
    fn the_role_state_reads_as_disabled_until_it_is_written_and_survives_the_writer() {
        let area = an_area();
        let run = a_role_state(&area);
        assert_eq!(run.read().unwrap(), RoleDesiredState::Disabled);
        run.write(RoleDesiredState::Enabled).unwrap();
        assert_eq!(run.read().unwrap(), RoleDesiredState::Enabled);
        // A later supervisor, reading what the earlier one wrote.
        let a_later_run = a_role_state(&area);
        assert_eq!(a_later_run.read().unwrap(), RoleDesiredState::Enabled);
        a_later_run.write(RoleDesiredState::Disabled).unwrap();
        assert_eq!(a_role_state(&area).read().unwrap(), RoleDesiredState::Disabled);
        std::fs::remove_dir_all(&area).ok();
    }

    // --- the Telegram worker's state store ---

    fn a_workers_store(area: &Path) -> TheTelegramWorkersStateOnTheMachine {
        std::fs::create_dir_all(area).unwrap();
        TheTelegramWorkersStateOnTheMachine {
            at: area.join("telegram-worker-state"),
        }
    }

    /// **Absent is *nothing recorded yet***, and one fact written is read back
    /// by a store opened afresh with every other fact still standing.
    #[test]
    fn the_workers_store_reads_as_nothing_until_written_and_keeps_each_fact() {
        let area = an_area();
        let store = a_workers_store(&area);
        assert_eq!(store.read().unwrap(), TheTelegramWorkersStateAsHeld::default());
        store.change(|held| held.update_position = Some(41)).unwrap();
        store.change(|held| held.board = Some(7)).unwrap();
        let again = a_workers_store(&area).read().unwrap();
        assert_eq!(again.update_position, Some(41));
        assert_eq!(again.board, Some(7));
        std::fs::remove_dir_all(&area).ok();
    }

    /// **A store that does not read back is refused and never overwritten.**
    #[test]
    fn a_workers_store_that_does_not_read_back_is_not_written_over() {
        let area = an_area();
        let store = a_workers_store(&area);
        std::fs::write(&store.at, b"not a store").unwrap();
        assert!(store.read().is_err());
        assert!(store.change(|held| held.board = Some(1)).is_err());
        assert_eq!(std::fs::read(&store.at).unwrap(), b"not a store");
        std::fs::remove_dir_all(&area).ok();
    }

    /// **The routing key is drawn once**: a second ask hands back the first
    /// key and draws nothing.
    #[test]
    fn the_routing_key_is_issued_once() {
        let area = an_area();
        let store = a_workers_store(&area);
        let first = store
            .the_routing_key_issued_if_absent(|| Ok("00".repeat(16)))
            .unwrap();
        let second = store
            .the_routing_key_issued_if_absent(|| panic!("nothing is drawn a second time"))
            .unwrap();
        assert_eq!(first, second);
        std::fs::remove_dir_all(&area).ok();
    }

    /// **U14's moment, as the wire carries it**, and it is the same figure the
    /// record's own written form lays down — one derivation, so the two cannot
    /// disagree.
    #[test]
    fn the_release_moment_on_the_wire_is_the_moment_on_the_record() {
        use std::time::{Duration, UNIX_EPOCH};
        let at = Reading::taken(
            (),
            UNIX_EPOCH + Duration::new(1_777_000_000, 123_456_789),
            Clock::AMachine(MachineClock("the machine".into())),
        );
        assert_eq!(the_release_moment_as_text(&at), "1777000000.123456789");
        // Before the usual epoch: a negative second and the part after it.
        let earlier = Reading::taken(
            (),
            UNIX_EPOCH - Duration::new(5, 400_000_000),
            Clock::AMachine(MachineClock("the machine".into())),
        );
        assert_eq!(the_release_moment_as_text(&earlier), "-6.600000000");
    }

    /// **`State, identity and recovery` §1.3 rule 7: one reading keeps one
    /// shape in every release, and it is what `Rented-server runtime` G21
    /// reads** — whether each operation record carries its recorded outcome,
    /// whether that outcome is acknowledged, and whether any hold record
    /// stands.
    ///
    /// **The bytes below were written by release 0.1.6's own code**, the last
    /// release placed before the rule: its source taken from the repository at
    /// the commit that published it (`06b6892b`) into a folder outside the
    /// repository on 2026-09-28, and its own stores asked to write one open
    /// operation record, one closed one, one standing hold and the desktop's
    /// journal cursor. **They are pasted here whole and never regenerated**, so
    /// a change to any of the three shapes fails here rather than on a machine
    /// a 0.1.6 program set up.
    #[test]
    fn what_g21_reads_parses_the_same_from_what_release_0_1_6_wrote() {
        use crate::acting::journal::TheEventJournalOnTheMachine;
        use crate::acting::operation_records::TheOperationRecordsOnTheMachine;
        use smoozy_state::journal::ARegisteredReader;
        use smoozy_state::machine_operations::{Keeps, RecordedOutcome};

        const OPEN: &str = r#"{"operation":{"kind":"Operation","text":"8b8aaf99dbad787001a1ef3180bbc2c4"},"subject":{"kind":"Job","text":"5b0b4ae2e09dd3774749f10fa9bf6541"},"kind":"render","desired_state":"ShouldRun","attempt_count":1,"execution_group":null,"the_kinds_safe_boundary_marker":null,"latest_disposition":null,"recorded_outcome":null}"#;
        const CLOSED: &str = r#"{"operation":{"kind":"Operation","text":"19c4f2149b02b0f8ad4e05d518fcf057"},"subject":{"kind":"Job","text":"422450dc57dc4dfe4f59946ef730afc6"},"kind":"render","desired_state":"Ended","attempt_count":1,"execution_group":null,"the_kinds_safe_boundary_marker":null,"latest_disposition":null,"recorded_outcome":"GivenUp"}"#;
        const HOLD: &str = r#"{"hold":{"kind":"Operation","text":"4cd1cfee67f57619709f727ef5e0ce28"},"subject":{"Job":{"kind":"Job","text":"b18e4c1d34d2549180203a074351fdbc"}},"reason":"the first frame has not arrived","origin":"Submission","dependency":null,"release":null}"#;
        const CURSORS: &str = r#"[["the desktop",1]]"#;

        let area = an_area();
        let operations = area.join("operation-records");
        let holds = area.join("hold-records");
        let journal = area.join("event-journal");
        for (folder, name, bytes) in [
            (&operations, "8b8aaf99dbad787001a1ef3180bbc2c4.operation", OPEN),
            (&operations, "19c4f2149b02b0f8ad4e05d518fcf057.operation", CLOSED),
            (&holds, "4cd1cfee67f57619709f727ef5e0ce28.hold", HOLD),
            (&journal, "registrations", CURSORS),
        ] {
            std::fs::create_dir_all(folder).unwrap();
            std::fs::write(folder.join(name), bytes).unwrap();
        }
        let operation = |text: &str| Identity::read_back(IdentityKind::Operation, text).unwrap();

        let records = TheOperationRecordsOnTheMachine::in_the_area(&operations).unwrap();
        assert_eq!(
            records
                .what_is_held_under(&operation("8b8aaf99dbad787001a1ef3180bbc2c4"))
                .unwrap()
                .recorded_outcome(),
            None
        );
        assert_eq!(
            records
                .what_is_held_under(&operation("19c4f2149b02b0f8ad4e05d518fcf057"))
                .unwrap()
                .recorded_outcome(),
            Some(&RecordedOutcome::GivenUp)
        );

        let hold = TheHoldRecordsOnTheMachine::in_the_area(&holds)
            .unwrap()
            .read(&operation("4cd1cfee67f57619709f727ef5e0ce28"))
            .unwrap()
            .expect("the hold is read back");
        assert_eq!(hold.release, None, "it stands");

        let journal = TheEventJournalOnTheMachine::in_the_area(&journal).unwrap();
        assert_eq!(
            journal.every_registration().get(&ARegisteredReader::TheDesktop),
            Some(&1),
            "what the desktop has acknowledged"
        );
        std::fs::remove_dir_all(&area).ok();
    }
}
