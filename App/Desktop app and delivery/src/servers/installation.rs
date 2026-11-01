//! The real end of `Servers`' installation identity read.
//!
//! **What it is, in one sentence.** On every primary connection, before
//! anything machine-derived is used, Smoozy's own value is read out of the
//! machine's state area and the supervisor is asked to confirm it — and one of
//! four outcomes comes back, with the identity that was on the machine or its
//! explicit absence.
//!
//! **Both halves of the check already existed and neither was being asked.**
//! The reading is `Server protocol and transfers`' `C15`, on the pre-package
//! surface, from Smoozy's own state area, never against a path a provider also
//! writes. The confirmation is that document's health exchange, which carries
//! `Rented-server runtime` G12's request to the entry point through the planted
//! carriage. **What was missing is the desktop asking**, and that is this file.
//!
//! **Nothing here decides what a branch does.** The four outcomes are
//! `State, identity and recovery`'s own, composed by that module's
//! `from_the_two_halves` out of the two halves and out of nothing else. What is
//! here is the asking and the ordering.
//!
//! # The fourth branch, and why it has a file's worth of care
//!
//! *Present, and the supervisor did not answer* **discards nothing, uses
//! nothing, and is not the absent branch.** `State, identity and recovery` Y9
//! says so twice and that document's `V1` step 1 says it a third time. An
//! implementation that folds it into *absent* throws away a working machine's
//! whole record the first time a supervisor is slow, and the tests below assert
//! its distinctness rather than a comment claiming it.
//!
//! **Three more readings land on that same branch and each is named where it
//! is met**: a record that is there and will not read back, a read that could
//! not be taken at all, and a local record whose own values will not read back.
//! **None of them is absence.** Each is a reading that failed rather than a
//! machine that changed, and the rule they share is that nothing is discarded
//! on one.
//!
//! # What only a rented machine would show
//!
//! *"Nothing in the connection distinguishes a recycled machine from a rebooted
//! one"* — same address, same port, back in ninety seconds. **Every branch here
//! is driven at the desk by putting a value in the state area or leaving it
//! out**, which proves the composition and the ordering. What it cannot prove
//! is that a provider's recycle really presents as the absent branch on the
//! machine Smoozy placed an installation on.
//! `App/Servers/tests/against_a_rented_machine.rs` already takes its machine
//! from the environment and is where that is paid for.
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this
//! crate's own plan, `Preparation/Architecture/Desktop app and delivery.md`.
//! Every other document's marker is written with that document — at the site,
//! or in this file's own header list below. **Where a marker is a clause of
//! this crate's own plan *and* of the document meant, the header list is not
//! enough and the document is written out at the site**, because a bare one
//! there resolves to a real and wrong clause.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `State, identity and recovery` Y8, Y9, Y10, Y13, Y14.
//! `Servers` Q19. `Rented-server runtime` G12, G13. `Job files and outputs`
//! L4a.
//!
//! **A marker this crate's own plan also defines is deliberately not listed
//! above, whichever document is meant, and is written out with its document at
//! every site instead** — the header list cannot tell the two apart, and a bare
//! one would resolve to a real and wrong clause.

use std::future::Future;

use smoozy_job_files::layout::{Area, MachineRoot, TheLayout, WhereThePackageAreaSits};
use smoozy_server_protocol::asking_the_supervisor::WhatCameBack;
use smoozy_server_protocol::framing::{Request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS};
use smoozy_server_protocol::placement::performing::the_health_answer_in;
use smoozy_server_protocol::placement::THE_HEALTH_ANSWER_REQUEST;
use smoozy_server_protocol::reconnection::{read_the_installation_identity, WhatTheStateAreaHeld};
use smoozy_server_protocol::transport::ssh;
use smoozy_servers::acting::{TheIdentityRead, TheInstallationIdentityRead};
use smoozy_servers::store::ServerRow;
use smoozy_state::identities::{Identity, IdentityKind, InstallationCheck};

use crate::servers::sessions::{
    lend_until_it_ends, AHeldConnection, TheAdditionalConnection, ThisServersSessions,
};

// ------------------------------------------------------------- the two halves

/// **What the first half came back with** — the value read from Smoozy's own
/// state area.
///
/// **Four answers rather than three, and the fourth is not the third.** A
/// record that is there and will not read back says something is here and
/// Smoozy cannot say what; a read that could not be taken says nothing about
/// the machine at all. **Neither is absence** and neither may discard.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheReadFound {
    /// The record read back as an installation identity.
    AnIdentity(Identity),
    /// **No record at all.** Nothing of Smoozy's is on this machine — never
    /// prepared, recycled, reimaged, or a different machine at a re-used
    /// address.
    NoRecordAtAll,
    /// **The record is there and does not read back.**
    ARecordThatDoesNotReadBack,
    /// **The read could not be taken**, which is the far end refusing for a
    /// reason that is not *no such file*. `Server protocol and transfers`
    /// `C15`: a refusal from the far end is not read as absence.
    TheReadCouldNotBeTaken,
}

/// **What the second half came back with** — the supervisor's own answer.
///
/// **Not answering is an answer** (`Rented-server runtime` G13), so it is a
/// value here rather than a failure a caller might treat differently.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheSupervisorSaid {
    /// `Rented-server runtime` G12's health answer came back, naming the
    /// installation the supervisor is running under.
    ItIsRunningUnder(Identity),
    /// No answer, or an answer carrying fewer than G12's four things, which
    /// that document's G13 makes the same failure.
    Nothing,
}

/// **`State, identity and recovery` Y9's two halves, as one seam.**
///
/// **It exists so the composition below is driven at the desk.** The halves are
/// the only part of this file that needs a machine; the ordering between them,
/// the branch each reading takes and the identity that is carried out are the
/// part worth testing, and a judgement that can only be reached through a
/// socket is a judgement nobody tests. That is the same split `Servers`' own
/// acting half is built on.
///
/// **The package area is handed in at every call and no implementation composes
/// one.** `Server protocol and transfers` `C15` keeps *never against a path a
/// provider also writes* by naming where the reading is taken from, and a half
/// that chose its own area would be the one place that rule could be lost.
pub trait TheTwoHalvesOfTheCheck<C> {
    /// **The first half.** `Server protocol and transfers` `C15`'s read, on the
    /// pre-package surface, out of Smoozy's own state area.
    fn read_the_state_area(
        &mut self,
        channel: &mut C,
        the_package_area: &str,
    ) -> impl Future<Output = WhatTheReadFound> + Send;

    /// **The second half.** `Rented-server runtime` G12's health exchange,
    /// carried to the entry point on the command channel.
    fn ask_the_supervisor(
        &mut self,
        channel: &mut C,
        the_package_area: &str,
    ) -> impl Future<Output = WhatTheSupervisorSaid> + Send;

    /// **`Server protocol and transfers` B4's package inspection, asked only
    /// where the supervisor did not confirm** — so `Server protocol and
    /// transfers` P3b can tell a program that speaks no protocol this build
    /// speaks from one that is not answering. Added 2026-09-28, step 10c's
    /// transfer wave piece 8.
    ///
    /// **Nothing by default**, which leaves the branch exactly as it was: a
    /// half that takes no inspection raises V1's condition as before.
    fn inspect_the_package_area(
        &mut self,
        _channel: &mut C,
        _the_package_area: &str,
    ) -> impl Future<Output = Option<WhatThePackageAreaHolds>> + Send {
        async { None }
    }
}

// ------------------------------------------------ what the local record holds

/// **What the server row says before the machine is asked at all.**
///
/// The read needs two values out of the row — where the package area is, and
/// which installation this desktop last confirmed — and a row that cannot
/// supply them is answered without a request going out.
#[derive(Debug, Clone, PartialEq, Eq)]
enum WhatTheRowSupplies {
    /// Both values are in hand.
    TheAreaAndWhatIsRemembered {
        the_package_area: String,
        remembered: Option<Identity>,
    },
    /// **No placement is recorded against this row**, so there is no area to
    /// name. `Server protocol and transfers` `C15`: *a machine that has had no
    /// placement has no such area to name*, which is the absent branch and not
    /// a place to look harder.
    NoPlacementIsRecorded,
    /// **The row's own values will not read back** — a recorded root that is
    /// not a root, or a remembered identity that is not an identity.
    ///
    /// **This is a damaged local record and never a changed machine**, so it
    /// takes the branch that discards nothing. Reading it as *nothing is
    /// remembered* would make the machine's own identity *present and
    /// different* and would disqualify every machine-derived value the row
    /// holds, on the strength of a value Smoozy itself wrote and can no longer
    /// read.
    TheRowsOwnValuesWillNotReadBack,
}

/// **Where one server's package area is, read out of its row and out of
/// nothing else.**
///
/// **Lifted out of the read below at step 10c piece 4**, because the read is
/// not the only thing that asks the supervisor: pieces 5 and 6 of that step
/// send it a job and ask it for a job's record, and every request rides the one
/// constant command pointed at this area (`Server protocol and transfers`
/// §1.1 item 1). **One place works the area out**, so a second asker does not
/// compose a second answer to where it is.
///
/// **Nothing here reaches a machine and nothing is chosen.** `Job files and
/// outputs` L1 chooses the root once per installation at hand-over and records
/// it against the row; this rebuilds the area from what was written down.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhereThePackageAreaIs {
    /// The area, as the path the entry point is reached under.
    At(String),
    /// **No placement is recorded against this row**, so there is no area to
    /// name — `Server protocol and transfers` `C15`'s *a machine that has had
    /// no placement has no such area to name*.
    NoPlacementIsRecorded,
    /// **The row's own recorded root will not read back as a root.** A damaged
    /// local record, and never a changed machine.
    TheRowsOwnValuesWillNotReadBack,
}

/// See [`WhereThePackageAreaIs`].
pub fn where_the_package_area_is(row: &ServerRow) -> WhereThePackageAreaIs {
    match the_layout_the_row_records(row) {
        Ok(Some(layout)) => WhereThePackageAreaIs::At(layout.area(Area::Package)),
        Ok(None) => WhereThePackageAreaIs::NoPlacementIsRecorded,
        Err(TheRowsOwnValuesWillNotReadBack) => WhereThePackageAreaIs::TheRowsOwnValuesWillNotReadBack,
    }
}

/// **The machine's whole layout, as the row records it** — `Job files and
/// outputs` L1's root and L4a's arrangement — for a caller that wants more than
/// the package area. Nothing where no placement is recorded or the row's own
/// values will not read back.
pub fn the_layout_recorded_for(row: &ServerRow) -> Option<TheLayout> {
    the_layout_the_row_records(row).ok().flatten()
}

/// The row's own recorded root would not read back as a root.
struct TheRowsOwnValuesWillNotReadBack;

fn the_layout_the_row_records(
    row: &ServerRow,
) -> Result<Option<TheLayout>, TheRowsOwnValuesWillNotReadBack> {
    let Some(root) = row.recorded_machine_root.as_ref() else {
        return Ok(None);
    };
    let chosen = MachineRoot::chosen(&root.path).map_err(|_| TheRowsOwnValuesWillNotReadBack)?;
    // **`Job files and outputs` L4a's two arrangements and there is no third.**
    // An absence here is the package area sitting under the root, and never a
    // value nobody wrote down.
    let where_it_sits = match &root.the_package_areas_own_location {
        None => WhereThePackageAreaSits::UnderTheRoot,
        Some(apart) => WhereThePackageAreaSits::ApartAt(
            MachineRoot::chosen(apart).map_err(|_| TheRowsOwnValuesWillNotReadBack)?,
        ),
    };
    // **`Job files and outputs` L1: the root is chosen once per installation at
    // hand-over and never re-chosen**, so the layout is rebuilt from what was
    // written down rather than chosen again.
    Ok(Some(TheLayout::of(chosen, where_it_sits)))
}

fn what_the_row_supplies(row: &ServerRow) -> WhatTheRowSupplies {
    let the_package_area = match where_the_package_area_is(row) {
        WhereThePackageAreaIs::At(area) => area,
        WhereThePackageAreaIs::NoPlacementIsRecorded => {
            return WhatTheRowSupplies::NoPlacementIsRecorded
        }
        WhereThePackageAreaIs::TheRowsOwnValuesWillNotReadBack => {
            return WhatTheRowSupplies::TheRowsOwnValuesWillNotReadBack
        }
    };
    let remembered = match row.last_confirmed_installation_identity.as_deref() {
        None => None,
        Some(text) => match Identity::read_back(IdentityKind::Installation, text) {
            Some(identity) => Some(identity),
            None => return WhatTheRowSupplies::TheRowsOwnValuesWillNotReadBack,
        },
    };

    WhatTheRowSupplies::TheAreaAndWhatIsRemembered {
        the_package_area,
        remembered,
    }
}

// ------------------------------------------------------------- the read itself

/// **The real end of `Servers`' installation identity read, for one server.**
///
/// **It is per server rather than per application**, because the check is made
/// against what the local record remembers for that row and the seam's own
/// method takes a connection and nothing else. `Servers` `Q1` step 5 runs it on
/// every primary connection, and that document's Q19 re-takes it alone on the
/// fourth branch until it settles — **this type counts nothing and holds no
/// state between reads**, so a re-take is the same call again.
pub struct TheMachinesInstallationIdentity<H, T = NobodyIsTold> {
    supplies: WhatTheRowSupplies,
    /// **The server this reader is for**, so a condition of V13's is raised
    /// against it (`Server protocol and transfers` P3b).
    server: String,
    /// **The row as the local record holds it at the moment of each read**,
    /// where the caller can reach the record — see
    /// [`TheMachinesInstallationIdentity::reading_the_row_as_it_stands`].
    the_row_now: Option<TheRowNow>,
    halves: H,
    told: T,
}

/// **The row, read again at the moment of a read.** Nothing where it cannot be
/// read, and the last row read then stands.
pub type TheRowNow = Box<dyn FnMut() -> Option<ServerRow> + Send>;

impl<H> TheMachinesInstallationIdentity<H> {
    /// **Composed from the server row, once, before the connection is used.**
    pub fn for_this_server(row: &ServerRow, halves: H) -> TheMachinesInstallationIdentity<H> {
        TheMachinesInstallationIdentity {
            supplies: what_the_row_supplies(row),
            server: row.local_server_identifier.clone(),
            the_row_now: None,
            halves,
            told: NobodyIsTold,
        }
    }
}

impl<H, T> TheMachinesInstallationIdentity<H, T> {
    /// **The same read, telling `told` what the supervisor did** — which is
    /// how `State, identity and recovery` Y9's *the supervisor-unavailable
    /// condition is raised* reaches the record. This reader is the one place
    /// that knows which case it met.
    pub fn telling<U: WhatTheSupervisorDid>(self, told: U) -> TheMachinesInstallationIdentity<H, U> {
        TheMachinesInstallationIdentity {
            supplies: self.supplies,
            server: self.server,
            the_row_now: self.the_row_now,
            halves: self.halves,
            told,
        }
    }

    /// **The same read, taken against the row as the local record holds it at
    /// the moment of each read** rather than as it stood when the reader was
    /// composed. Added 2026-09-27, step 10c's eighth-pass fix piece 3.
    ///
    /// **Why.** A period composes one reader and every primary connection under
    /// it reads through that reader — the opening one, each repair attempt, and
    /// the return after a restart Smoozy caused. **A reader composed once froze
    /// the package area and the remembered installation at the period's
    /// start**, so a period that began on a bare machine read *absent* on every
    /// later connection without asking the machine at all, even after the
    /// ready action had placed Smoozy and recorded the installation — and a
    /// period that began before a *Set up again* read the new installation as
    /// *present and different*. Both are `State, identity and recovery` Y9 read
    /// against a value the record no longer holds, which is the check compared
    /// against nothing Smoozy remembers; `Servers` D8's rule for the host key is
    /// the same one: *"each attempt reads the row as the record holds it at
    /// that moment"*. **The eighth pass's empty Queue after the restart is this
    /// read**: the return after the driver's restart read *absent*, so
    /// `Servers` Q1 step 7 never ran on it.
    pub fn reading_the_row_as_it_stands(
        mut self,
        the_row_now: impl FnMut() -> Option<ServerRow> + Send + 'static,
    ) -> Self {
        self.the_row_now = Some(Box::new(the_row_now));
        self
    }
}

/// **What the reader tells about the supervisor**, and only where it knows.
///
/// *(Added 2026-09-18, step 10b's first audit round, item 8.)* `State,
/// identity and recovery` Y9 and V1 step 1 raise the supervisor-unavailable
/// condition on the fourth branch, and nothing did. **One reading that reaches
/// that branch tells nothing**: a row whose own stored values will not read
/// back, where the machine was not asked at all — *the supervisor is
/// unavailable* would there be a claim Smoozy does not know to be true.
pub trait WhatTheSupervisorDid {
    /// The fourth branch was met on the machine.
    fn it_did_not_answer(&mut self);
    /// The supervisor confirmed the value that was read.
    fn it_answered(&mut self);
}

/// **Nobody is told** — a reader composed for a test, or for a caller with no
/// record to raise a condition in.
pub struct NobodyIsTold;

impl WhatTheSupervisorDid for NobodyIsTold {
    fn it_did_not_answer(&mut self) {}
    fn it_answered(&mut self) {}
}

/// **The branch a reading that failed takes, in one place.**
///
/// `State, identity and recovery` Y9's fourth outcome: the check is unconfirmed,
/// **nothing is discarded**, the supervisor-unavailable condition is raised, and
/// desktop reconciliation stops at step 1. **It is never the absent branch and
/// never the confirmed one.**
///
/// **The identity carried out is absent** because none of the readings that
/// reach here can name one — `Servers` H17's *or the explicit absence of one*,
/// and the branch beside it is what says which of the two an absence means.
fn nothing_is_discarded() -> TheIdentityRead {
    TheIdentityRead {
        check: InstallationCheck::PresentAndTheSupervisorDidNotAnswer,
        identity_on_the_machine: None,
    }
}

impl<C, H, T> TheInstallationIdentityRead<C> for TheMachinesInstallationIdentity<H, T>
where
    C: Send,
    H: TheTwoHalvesOfTheCheck<C> + Send,
    T: WhatTheSupervisorDid + Send,
{
    /// **The two halves, weighed in `State, identity and recovery` Y9's own
    /// order.**
    ///
    /// **The supervisor is asked on one reading and no other.** *Absent* is
    /// decided by the reading alone — an absent identity is *never prepared,
    /// recycled, reimaged, or a different machine at a re-used address*
    /// whatever the supervisor is doing, and that document's Y14 forbids the
    /// check resting on reachability. **A value that is present and different
    /// is likewise decided by the reading alone**, because a supervisor
    /// answering would not make another installation's state Smoozy's. **Only
    /// the equal case asks the second half**, which is where the confirmation
    /// is the whole of what separates the fourth outcome from the first.
    ///
    /// **Confirming means naming the same installation.** An answer carrying a
    /// different one has not confirmed the value that was read, so it takes the
    /// unconfirmed branch rather than the confirmed one. **It is not the
    /// present-and-different branch either**: that branch is the reading's, and
    /// nothing a supervisor says moves a reading.
    async fn read(&mut self, channel: &mut C) -> TheIdentityRead {
        // **The row as the record holds it now**, where the caller can reach
        // it — see [`TheMachinesInstallationIdentity::reading_the_row_as_it_stands`].
        if let Some(row) = self.the_row_now.as_mut().and_then(|now| now()) {
            self.supplies = what_the_row_supplies(&row);
        }
        let (the_package_area, remembered) = match &self.supplies {
            WhatTheRowSupplies::NoPlacementIsRecorded => {
                return TheIdentityRead {
                    check: InstallationCheck::Absent,
                    identity_on_the_machine: None,
                }
            }
            WhatTheRowSupplies::TheRowsOwnValuesWillNotReadBack => return nothing_is_discarded(),
            WhatTheRowSupplies::TheAreaAndWhatIsRemembered {
                the_package_area,
                remembered,
            } => (the_package_area.clone(), remembered.clone()),
        };

        let found = self
            .halves
            .read_the_state_area(channel, &the_package_area)
            .await;

        let on_the_machine = match found {
            // **`State, identity and recovery` Y13's discard, and it is not a
            // deletion**: what the absent branch does to the values the row
            // holds is that module's and is not retaken here.
            WhatTheReadFound::NoRecordAtAll => {
                return TheIdentityRead {
                    check: InstallationCheck::from_the_two_halves(None, remembered.as_ref(), false),
                    identity_on_the_machine: None,
                }
            }
            WhatTheReadFound::ARecordThatDoesNotReadBack
            | WhatTheReadFound::TheReadCouldNotBeTaken => {
                // **The fourth branch, met on the machine.** `State, identity
                // and recovery` Y9 raises the condition on it.
                self.told.it_did_not_answer();
                return nothing_is_discarded();
            }
            WhatTheReadFound::AnIdentity(identity) => identity,
        };

        // **The equal case is the only one that asks the supervisor.**
        let the_supervisor_answered = if remembered.as_ref() == Some(&on_the_machine) {
            let answered = match self
                .halves
                .ask_the_supervisor(channel, &the_package_area)
                .await
            {
                WhatTheSupervisorSaid::ItIsRunningUnder(named) => named == on_the_machine,
                WhatTheSupervisorSaid::Nothing => false,
            };
            // **The supervisor was asked, so what it did is known** — the
            // condition is raised where it did not confirm, and withdrawn by
            // its answering (`State, identity and recovery` §8.3).
            //
            // **`Server protocol and transfers` P3b: a program that speaks no
            // protocol this build speaks is *incompatible*, with its direction,
            // and never *the supervisor not answering*.** So where it did not
            // confirm, the package area is inspected first, and V1's condition
            // is raised only where the inspection does not show that. Added
            // 2026-09-28, step 10c's transfer wave piece 8. *(Until then every
            // program of another protocol read as Smoozy not responding.)*
            if answered {
                self.told.it_answered();
                if matches!(
                    crate::servers::published::what_the_server_carries(&self.server),
                    Some(crate::servers::published::WhatSmoozyOnTheServerIs::TooOldAndItsWorkCannotBeControlledFromHere)
                        | Some(crate::servers::published::WhatSmoozyOnTheServerIs::TooNew)
                ) {
                    // **The condition ends when it ends**: a program that
                    // answers speaks this build's protocol. Whether it is
                    // older or newer is `Servers` Q1 step 9's to read again.
                    crate::servers::published::the_server_carries(&self.server, None);
                }
            } else {
                let held = self
                    .halves
                    .inspect_the_package_area(channel, &the_package_area)
                    .await;
                match held.as_ref().and_then(|held| held.what_it_is_where_no_protocol_is_shared(
                    crate::setting_up::this_copys_own_release_sequence_if_known(env!("CARGO_PKG_VERSION")),
                )) {
                    Some(incompatible) => {
                        crate::servers::published::the_server_carries(&self.server, Some(incompatible));
                    }
                    None => self.told.it_did_not_answer(),
                }
            }
            answered
        } else {
            false
        };

        TheIdentityRead {
            // **The four outcomes stay `State, identity and recovery`'s own**,
            // composed there out of the two halves and out of nothing else, so
            // the complete set has one author.
            check: InstallationCheck::from_the_two_halves(
                Some(&on_the_machine),
                remembered.as_ref(),
                the_supervisor_answered,
            ),
            // **`Servers` H17: the identity present on the machine when the
            // reading was taken**, which the fresh hardware reading of that
            // document's H18 carries.
            identity_on_the_machine: Some(on_the_machine.as_text()),
        }
    }
}

// ------------------- `Server protocol and transfers` B4, read by the desktop

/// **What one package area holds, read by `Server protocol and transfers` B4's
/// package inspection** — the first desktop caller that inspection has had.
/// Added 2026-09-28, step 10c's transfer wave piece 8.
///
/// **What it was written for, and what it now also meets.** The reading was
/// built in step 8a for a placement's own continuation, and nothing on this
/// side ever asked it. It now answers `Desktop app and delivery` V11 and V14
/// and `Server protocol and transfers` P3b: which release the machine runs, and
/// whether it speaks a protocol this build speaks — both read off records the
/// placement wrote, **so a dead supervisor or a program of another release
/// changes nothing about the answer**.
///
/// **A reading, and never durable state on this side** (V8, and
/// `Server protocol and transfers` B5): nothing here is written down.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WhatThePackageAreaHolds {
    /// **Whether the reading was taken at all.** A read the point could not
    /// lend a session for, or one the far end refused, says nothing about the
    /// machine and is never read as a machine with nothing on it.
    pub it_was_read: bool,
    /// **The active package's release sequence** — the pointer's own
    /// descriptor, `Rented-server runtime` G31a's reading — or nothing where no
    /// package is active.
    pub its_release_sequence: Option<u64>,
    /// **Each retained adapter's declared protocol interval**, as the phase
    /// record carries it by value (`Server protocol and transfers` §1.2 item 3).
    pub the_declared_intervals: Vec<(u32, u32)>,
    /// **The phase record, whole, where it read back** — V11's update carries
    /// `Rented-server runtime` G4's previous version out of it, and V12's
    /// re-send its recorded attempt.
    pub the_phase_record: Option<smoozy_runtime::package::phase::PhaseRecord>,
    /// **The boot-registration record's own content, by value** (`Rented-server
    /// runtime` K16) — what `Rented-server runtime` G4 carries forward.
    pub the_boot_registration: Option<Vec<u8>>,
}

impl WhatThePackageAreaHolds {
    /// **Whether a retained adapter speaks the protocol this build speaks.**
    /// A package that records no adapter at all says nothing against it, and is
    /// taken as speaking it — the placement always records one, so this is a
    /// record from before that and not a program of another protocol.
    pub fn it_speaks_what_this_build_speaks(&self) -> bool {
        let ours = THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS.0;
        self.the_declared_intervals.is_empty()
            || self
                .the_declared_intervals
                .iter()
                .any(|(low, high)| *low <= ours && ours <= *high)
    }

    /// **`Server protocol and transfers` P3b: where no protocol is shared, the
    /// program is *incompatible*, with its direction** — and nothing where one
    /// is shared, or where nothing is active, which are not this clause's.
    ///
    /// **The direction is the release sequences' where this copy knows its own
    /// in this run**, which is that clause's rule. **Where it does not, the protocol
    /// intervals give it**, and they point the same way: a protocol version is
    /// raised only by a release, so a program speaking only versions below this
    /// build's is from an earlier release. Reading this copy's own release over
    /// the internet inside the connection's own check, to learn what the
    /// intervals already say, was weighed and not taken.
    ///
    /// **Too old is V13's *its work cannot be controlled from here*** — no
    /// retained adapter reaches it, which is that condition's own definition,
    /// whether or not work stands.
    pub fn what_it_is_where_no_protocol_is_shared(
        &self,
        own_release_sequence: Option<u64>,
    ) -> Option<crate::servers::published::WhatSmoozyOnTheServerIs> {
        use crate::servers::published::{the_direction, TheDirection, WhatSmoozyOnTheServerIs};
        let its = self.its_release_sequence?;
        if self.it_speaks_what_this_build_speaks() {
            return None;
        }
        let newer = match own_release_sequence.and_then(|own| the_direction(Some(its), own)) {
            Some(TheDirection::TheMachineIsNewer) => true,
            Some(TheDirection::TheMachineIsOlder) => false,
            // **Equal releases with no protocol shared is a contradiction** — one
            // release carries one protocol — so the intervals decide.
            Some(TheDirection::TheSame) | None => {
                let ours = THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS.0;
                self.the_declared_intervals.iter().all(|(low, _)| *low > ours)
            }
        };
        Some(if newer {
            WhatSmoozyOnTheServerIs::TooNew
        } else {
            WhatSmoozyOnTheServerIs::TooOldAndItsWorkCannotBeControlledFromHere
        })
    }
}

/// **`Server protocol and transfers` B4's inspection of one package area, over
/// the point's reading session**, with the phase record and the boot
/// registration's content read beside it in the same session.
///
/// **The area is handed in and every path is composed inside the protocol
/// crate's own readers**, which is `Server protocol and transfers` C15's rule
/// for the installation identity carried over: no provider-written path reaches
/// the reading.
pub async fn what_the_package_area_holds<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    the_package_area: &str,
) -> WhatThePackageAreaHolds {
    let area = the_package_area.to_string();
    let read = sessions
        .a_reading(true, move |subsystem| {
            Box::pin(async move {
                let inspection =
                    smoozy_server_protocol::bootstrap::inspect(subsystem, area.as_bytes()).await?;
                let phase_record =
                    smoozy_server_protocol::placement::read_the_phase_record(subsystem, &area).await?;
                let registration =
                    smoozy_server_protocol::placement::read_the_boot_registration_record(
                        subsystem, &area,
                    )
                    .await?;
                Ok((inspection, phase_record, registration))
            })
        })
        .await;
    match read {
        Ok(Ok((inspection, phase_record, registration))) => {
            what_an_inspection_shows(inspection.value(), phase_record, registration)
        }
        _ => WhatThePackageAreaHolds::default(),
    }
}

/// **The reading, composed from what the three reads brought back** — split
/// from the reading so it is driven at the desk.
pub fn what_an_inspection_shows(
    inspection: &smoozy_server_protocol::bootstrap::AnInspection,
    phase_record: smoozy_server_protocol::bootstrap::WhatARecordHeld<
        smoozy_runtime::package::phase::PhaseRecord,
    >,
    registration: smoozy_server_protocol::bootstrap::WhatARecordHeld<
        smoozy_runtime::registration::BootRegistrationRecord,
    >,
) -> WhatThePackageAreaHolds {
    use smoozy_server_protocol::bootstrap::WhatARecordHeld;
    WhatThePackageAreaHolds {
        it_was_read: true,
        its_release_sequence: inspection
            .the_active_packages_descriptor
            .as_ref()
            .map(|descriptor| descriptor.verified_release_sequence),
        the_declared_intervals: inspection
            .the_retained_adapters
            .iter()
            .map(|adapter| adapter.declared_interval)
            .collect(),
        the_phase_record: match phase_record {
            WhatARecordHeld::It(record) => Some(record),
            _ => None,
        },
        the_boot_registration: match registration {
            WhatARecordHeld::It(record) => Some(record.content),
            _ => None,
        },
    }
}

// ----------------------------------------------------- the halves, for real

/// **The two halves over a live connection**, and the only part of this file
/// that reaches a machine.
///
/// **`Server protocol and transfers` `A6`: an authenticated connection is not
/// evidence about the machine at the other end of it.** Neither half here is
/// answered from the connection. The first reads a record Smoozy wrote; the
/// second waits for a program Smoozy placed to answer a framed request. **A
/// connection that opened, answered and authenticated produces neither.**
///
/// # Both halves go through the connection's own point, and why
///
/// **Until step 10c piece 4 the second half held a file-transfer session open
/// across its command**, which is two sessions at once. On a machine that
/// allows one session per connection the command was refused, the exchange
/// answered nothing, and **every connection took the fourth branch and raised
/// *the supervisor is unavailable* on a healthy supervisor** — piece 2 of that
/// step watched it happen on the loopback server at a limit of one.
///
/// **`Server protocol and transfers` T5 is the rule that was broken**: at a
/// learned limit of one, the reading session and a command take turns. **The
/// taking of turns is already built once**, at `crate::servers::sessions`, and
/// piece 2's general supervisor ask already goes through it — plant on a
/// reading, close the reading, run the command, reopen, take the link away. **So
/// the check lends the connection to a point of its own for exactly its own
/// length and asks through it**, rather than writing that sequence a second time
/// here: [`TheInstallationIdentityRead`] below does the lending, and the two
/// halves are asked of the point.
pub struct OverTheConnection;

impl<C: AHeldConnection> TheTwoHalvesOfTheCheck<ThisServersSessions<C>> for OverTheConnection {
    /// **`Server protocol and transfers` `C15`, on the point's reading
    /// session.** The area is handed in and the record's own path is composed
    /// inside that call, which is what keeps a provider-written path out of the
    /// reading.
    async fn read_the_state_area(
        &mut self,
        sessions: &mut ThisServersSessions<C>,
        the_package_area: &str,
    ) -> WhatTheReadFound {
        let area = the_package_area.as_bytes().to_vec();
        let read = sessions
            .a_reading(true, move |subsystem| {
                Box::pin(async move {
                    Ok(read_the_installation_identity(subsystem, &area).await)
                })
            })
            .await;
        match read {
            Ok(Ok(Ok(WhatTheStateAreaHeld::AnIdentity(identity)))) => {
                WhatTheReadFound::AnIdentity(identity)
            }
            Ok(Ok(Ok(WhatTheStateAreaHeld::Nothing))) => WhatTheReadFound::NoRecordAtAll,
            Ok(Ok(Ok(WhatTheStateAreaHeld::ARecordThatDoesNotReadBack))) => {
                WhatTheReadFound::ARecordThatDoesNotReadBack
            }
            // **A read the point could not lend a session for, and a read the
            // far end refused, are both a read that could not be taken** — and
            // neither is absence (`Server protocol and transfers` `C15`).
            _ => WhatTheReadFound::TheReadCouldNotBeTaken,
        }
    }

    /// **`Rented-server runtime` G12's health exchange, asked through piece 2's
    /// general supervisor ask** — `smoozy_server_protocol::asking_the_supervisor`,
    /// through [`ThisServersSessions::ask_the_supervisor`].
    ///
    /// **The identity is issued here and handed in**, because the initiating
    /// side chooses it before the request is sent and what it is carried for is
    /// correlation. **A desk with no randomness cannot correlate a result**, so
    /// that is the *did not answer* branch and never a request sent without
    /// one.
    async fn ask_the_supervisor(
        &mut self,
        sessions: &mut ThisServersSessions<C>,
        the_package_area: &str,
    ) -> WhatTheSupervisorSaid {
        let Ok(correlating) = Identity::issue(IdentityKind::Operation) else {
            return WhatTheSupervisorSaid::Nothing;
        };
        let request = Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            correlating,
            THE_HEALTH_ANSWER_REQUEST,
        );
        match sessions.ask_the_supervisor(the_package_area, &request).await {
            WhatCameBack::ItAnswered(answered) => match the_health_answer_in(&answered) {
                Some(answer) => WhatTheSupervisorSaid::ItIsRunningUnder(answer.installation),
                // **`Rented-server runtime` G13: an answer carrying fewer than
                // G12's four things is the same failure as no answer.**
                None => WhatTheSupervisorSaid::Nothing,
            },
            // **G13 again: a supervisor that does not answer, and one that
            // could not be asked because the carriage could not be planted,
            // arrive at the same branch.**
            WhatCameBack::NothingCorrelatedCameBack | WhatCameBack::ItWasNeverAsked => {
                WhatTheSupervisorSaid::Nothing
            }
        }
    }

    /// **`Server protocol and transfers` B4, on the point's reading session** —
    /// [`what_the_package_area_holds`], and nothing where the reading could not
    /// be taken, so the branch then raises V1's condition exactly as before.
    async fn inspect_the_package_area(
        &mut self,
        sessions: &mut ThisServersSessions<C>,
        the_package_area: &str,
    ) -> Option<WhatThePackageAreaHolds> {
        let held = what_the_package_area_holds(sessions, the_package_area).await;
        held.it_was_read.then_some(held)
    }
}

/// **The check, on a live connection: the connection is lent to a point of its
/// own for the check's whole length, and both halves are asked of that point.**
///
/// **One point for both halves, and not one per half**, because the point is
/// what knows a session it let go of may not be free yet at the far end: a
/// second point, started straight after the first closed its reading session,
/// would meet that refusal knowing nothing and take it as the machine's limit.
///
/// **The point goes when the check is done**, and its reading session with it,
/// so `Servers`' sequence gets its connection back exactly as it lent it.
/// **Where the connection ends during the check**, the check is the fourth
/// branch's — nothing is discarded — and the supervisor is not said to be
/// unavailable, because nothing was learned about it: the ending is the
/// transport's to report and `Servers` reads it next.
impl<T> TheInstallationIdentityRead<ssh::Connection>
    for TheMachinesInstallationIdentity<OverTheConnection, T>
where
    T: WhatTheSupervisorDid + Send,
{
    async fn read(&mut self, channel: &mut ssh::Connection) -> TheIdentityRead {
        the_check_through_a_point_of_its_own(self, channel).await
    }
}

/// The lending above, for any connection a point can lend — **so it is driven
/// at the desk** over a stand-in with a session limit, as well as over the real
/// connection `Servers`' sequence hands it.
pub async fn the_check_through_a_point_of_its_own<C, T>(
    reading: &mut TheMachinesInstallationIdentity<OverTheConnection, T>,
    channel: &mut C,
) -> TheIdentityRead
where
    C: AHeldConnection,
    T: WhatTheSupervisorDid + Send,
{
    let sessions = ThisServersSessions::<C>::for_one_server();
    // **`Servers` S14's additional connection is not the check's**: it carries
    // transfers alone, and the check moves none.
    let no_additional: TheAdditionalConnection<C> =
        std::sync::Arc::new(|| Box::pin(async { None }));
    let mut asking = sessions.clone();
    let lending = lend_until_it_ends(&sessions, channel, &no_additional);
    let checked = TheInstallationIdentityRead::<ThisServersSessions<C>>::read(reading, &mut asking);
    tokio::pin!(lending, checked);
    tokio::select! {
        // **The lending is polled first**, so the point is live before the
        // first half asks it for anything.
        biased;
        _ended = &mut lending => nothing_is_discarded(),
        read = &mut checked => read,
    }
}

// ------------------------------------ `Servers` Q19a, the re-take at the commit

/// **`Servers` Q19a: where step 5 took the absent branch and a ready action
/// then commits a placement on that same live primary connection, step 5 is
/// taken again on it once, at that commit, and `Servers` Q1 carries on from
/// step 6 under the branch it now reads.** Added 2026-09-27, step 10c's
/// eighth-pass fix piece 3.
///
/// **What comes back is what the driving hands back after steps 5 and 6** —
/// the branch and, on the three branches that take one, the fresh hardware
/// reading (`Servers` H18) — so the caller keeps them the one way every other
/// connection's are kept, and starts step 7 where the branch is the confirmed
/// one. **Nothing comes back where nothing is re-taken**, and the list of those
/// is the clause's own:
///
/// - **Only where step 7 has not run.** The period's live connection must have
///   read the absent branch. *Set up again* on a machine whose period read the
///   confirmed branch has step 7 and `Desktop app and delivery` P10d's
///   sequence running already, and **nothing is re-taken there and no second
///   sequence is started beside the first** (`Servers` Q18). A branch read on a
///   connection that is no longer the live one is not the period's to re-take
///   either (`Servers` H1a).
/// - **One re-take, not a timer.** Taken once, at the commit. **Where it reads
///   the fourth branch, `Servers` Q19 governs from there**: that step alone is
///   re-taken on the live connection, on `Servers` Q19's own two figures and
///   no third, until it settles.
/// - **What ends it before it runs, or while `Servers` Q19 re-takes it**: the
///   connection no longer being live, which is the break declaration or the
///   period ending (`Servers` K3, K5, S11).
///
/// **The read is the period's own reader, over the period's own point** — the
/// connection is lent there, so nothing here holds it — and **the reader is
/// composed from the row as it stands after the commit**, which is what makes
/// the machine's new installation read as Smoozy's.
#[allow(clippy::too_many_arguments)]
pub async fn the_retake_at_the_commit<Ch, I, M, W>(
    what_the_period_read: Option<crate::servers::a_period::WhatTheLastConnectionRead>,
    reading: &mut I,
    channel: &mut Ch,
    the_machine: &mut M,
    row: &ServerRow,
    wait: &mut W,
    is_live: &dyn Fn() -> bool,
) -> Option<(InstallationCheck, Option<smoozy_servers::hardware::HardwareReading>)>
where
    I: TheInstallationIdentityRead<Ch>,
    M: smoozy_servers::readings::CommandChannel + smoozy_servers::readings::TakesAFreeSpaceReading,
    W: smoozy_servers::acting::Wait,
{
    let read = what_the_period_read?;
    if read.check != InstallationCheck::Absent || !read.on_the_connection_that_is_live {
        return None;
    }
    if !is_live() {
        return None;
    }
    let mut got = reading.read(channel).await;
    let mut retake = smoozy_servers::first_connection::TheFourthBranchRetake::begun();
    while !retake.settled(got.check) {
        if !is_live() {
            return None;
        }
        wait.until(retake.pause_before_the_next_read()).await;
        retake.reads_taken = retake.reads_taken.saturating_add(1);
        if !is_live() {
            return None;
        }
        got = reading.read(channel).await;
    }
    // **Step 6, under the branch now read** (`Servers` H18), against the root
    // the hand-over recorded (`Servers` H10) — so the hardware reading carries
    // the identity just placed (`Servers` H17).
    let hardware = if smoozy_servers::hardware::what_the_branch_does(got.check).a_fresh_reading_is_taken {
        let now = wait.now();
        smoozy_servers::readings::take_the_hardware_reading(
            the_machine,
            &row.local_server_identifier,
            got.identity_on_the_machine.clone(),
            row.recorded_machine_root
                .as_ref()
                .map(|root| root.path.as_str()),
            now,
        )
        .await
    } else {
        None
    };
    Some((got.check, hardware))
}

/// **`Servers` Q1 step 6's two seams over a period's own point**, for
/// [`the_retake_at_the_commit`]: each command one command at the point, and
/// the free-space figure one reading on its reading session
/// (`Server protocol and transfers` F2's first route, the route `Servers`'
/// own reading takes over a connection of its own).
pub struct TheHardwareReadingOverThePoint<'s, C: AHeldConnection> {
    pub sessions: &'s ThisServersSessions<C>,
}

impl<C: AHeldConnection> smoozy_servers::readings::CommandChannel
    for TheHardwareReadingOverThePoint<'_, C>
{
    async fn run(
        &mut self,
        command: smoozy_server_protocol::transport::CommandString,
    ) -> Result<
        smoozy_server_protocol::transport::CommandResult,
        smoozy_server_protocol::transport::ssh::CommandFailure,
    > {
        let mut one = crate::submitting::OnThisServersPoint {
            sessions: self.sessions,
        };
        smoozy_servers::readings::CommandChannel::run(&mut one, command).await
    }
}

impl<C: AHeldConnection> smoozy_servers::readings::TakesAFreeSpaceReading
    for TheHardwareReadingOverThePoint<'_, C>
{
    async fn free_space(&mut self, path: &str) -> Option<smoozy_servers::hardware::FreeSpace> {
        let at = path.to_string();
        let read = self
            .sessions
            .a_reading(false, move |subsystem| {
                Box::pin(async move {
                    Ok(
                        smoozy_server_protocol::free_space::read_it_at_the_deepest_ancestor_that_exists(
                            subsystem, &at,
                        )
                        .await,
                    )
                })
            })
            .await;
        match read {
            Ok(Ok(Ok(reading))) => Some(smoozy_servers::hardware::FreeSpace {
                available_bytes: reading.available_bytes,
                the_path_it_was_taken_against: reading.the_path_it_was_taken_against,
            }),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, SystemTime};

    use smoozy_server_protocol::transport::ssh::CommandFailure;
    use smoozy_server_protocol::transport::{
        CommandResult, CommandString, Credential, DecideHostTrust, Ending, Endpoint, HostTrust,
        OpenFailure,
    };
    use smoozy_servers::acting::{
        OpenConnections, SupplyTheCredential, TheActingHalf, TheServerBeingConnectedTo, Wait,
        WhatServersPublishes,
    };
    use smoozy_servers::condition::ObservedValue;
    use smoozy_servers::first_connection::ChangedIdentityWarning;
    use smoozy_servers::hardware::FreeSpace;
    use smoozy_servers::identity::{Host, KeyReference, Port, Username};
    use smoozy_servers::key_moments::{TheTwoAnswers, WhoStartedIt};
    use smoozy_servers::readings::{CommandChannel, TakesAFreeSpaceReading};
    use smoozy_servers::repair::{NextAttemptDue, PermanentRefusal, WhatTheRecordSays};
    use smoozy_servers::sequence::{self, AnAttempt, Attempt, ConnectionRole};
    use smoozy_servers::store::{RecordedMachineRoot, RememberedHostKey};

    use super::*;

    fn an_identity() -> Identity {
        Identity::issue(IdentityKind::Installation).expect("the desk has randomness")
    }

    /// A saved server that a hand-over has reached, remembering `remembered`.
    fn a_row(remembered: Option<&Identity>) -> ServerRow {
        ServerRow {
            local_server_identifier: "server-1".into(),
            host: Host::what_the_user_typed("198.51.100.10").expect("a host"),
            port: Port::STANDARD_SSH,
            username: Username::what_the_user_typed("root").expect("a username"),
            key: KeyReference {
                local_key_identifier: "key-1".into(),
                fingerprint: "SHA256:aaa".into(),
            },
            label: None,
            colour: None,
            remembered_setup: None,
            remembered_host_key: None,
            remembered_hardware_reading: None,
            recorded_machine_root: Some(RecordedMachineRoot {
                path: "/ephemeral/smoozy".into(),
                the_package_areas_own_location: None,
                read_under: remembered.map(Identity::as_text).unwrap_or_default(),
            }),
            last_confirmed_installation_identity: remembered.map(Identity::as_text),
            event_cursor: None,
            machine_password_subject: None,
            chosen_fallback_folder: None,
            the_driver_missed_the_optix_bar: false,
        }
    }

    // ------------------------------------------------- the halves, at the desk

    /// **The two halves, answered from a script rather than from a machine.**
    ///
    /// This is the stand-in the real [`OverTheConnection`] is swapped for, and
    /// the swap being one line is the whole shape check: a read that could only
    /// be reached through a socket would be a read nobody drives.
    struct TheHalvesAtTheDesk {
        the_state_area_holds: WhatTheReadFound,
        the_supervisor_says: WhatTheSupervisorSaid,
        /// Every area each half was asked against, so *which path was read* is
        /// an assertion rather than an inference.
        areas_asked: Arc<Mutex<Vec<String>>>,
        supervisor_asks: Arc<Mutex<usize>>,
    }

    impl TheHalvesAtTheDesk {
        fn holding(the_state_area_holds: WhatTheReadFound) -> TheHalvesAtTheDesk {
            TheHalvesAtTheDesk {
                the_state_area_holds,
                the_supervisor_says: WhatTheSupervisorSaid::Nothing,
                areas_asked: Arc::new(Mutex::new(Vec::new())),
                supervisor_asks: Arc::new(Mutex::new(0)),
            }
        }

        fn whose_supervisor_says(mut self, said: WhatTheSupervisorSaid) -> TheHalvesAtTheDesk {
            self.the_supervisor_says = said;
            self
        }
    }

    impl<C: Send> TheTwoHalvesOfTheCheck<C> for TheHalvesAtTheDesk {
        async fn read_the_state_area(
            &mut self,
            _channel: &mut C,
            the_package_area: &str,
        ) -> WhatTheReadFound {
            self.areas_asked
                .lock()
                .expect("the areas")
                .push(the_package_area.to_string());
            self.the_state_area_holds.clone()
        }

        async fn ask_the_supervisor(
            &mut self,
            _channel: &mut C,
            _the_package_area: &str,
        ) -> WhatTheSupervisorSaid {
            *self.supervisor_asks.lock().expect("the asks") += 1;
            self.the_supervisor_says.clone()
        }
    }

    /// One read, driven with no connection under it.
    fn read_with(
        row: &ServerRow,
        halves: TheHalvesAtTheDesk,
    ) -> (TheIdentityRead, usize, Vec<String>) {
        let asks = Arc::clone(&halves.supervisor_asks);
        let areas = Arc::clone(&halves.areas_asked);
        let mut reading = TheMachinesInstallationIdentity::for_this_server(row, halves);
        let mut nothing = ();
        let read = tauri::async_runtime::block_on(reading.read(&mut nothing));
        let asks = *asks.lock().expect("the asks");
        let areas = areas.lock().expect("the areas").clone();
        (read, asks, areas)
    }

    // ------------------------------------------------------- the four branches

    /// **Branch one: present and equal, confirmed.**
    #[test]
    fn a_value_that_matches_and_a_supervisor_that_names_it_is_the_confirmed_branch() {
        let placed = an_identity();
        let row = a_row(Some(&placed));
        let (read, asks, areas) = read_with(
            &row,
            TheHalvesAtTheDesk::holding(WhatTheReadFound::AnIdentity(placed.clone()))
                .whose_supervisor_says(WhatTheSupervisorSaid::ItIsRunningUnder(placed.clone())),
        );
        assert_eq!(read.check, InstallationCheck::PresentAndEqualConfirmed);
        assert_eq!(read.identity_on_the_machine, Some(placed.as_text()));
        assert_eq!(asks, 1, "the equal case is the one that asks");
        // **The read is taken against the area the placement recorded and no
        // other path**, which is what `Server protocol and transfers` `C15`'s
        // rule about a provider-written path comes down to here.
        assert_eq!(areas, vec!["/ephemeral/smoozy/package".to_string()]);
    }

    /// **Branch two: absent.**
    #[test]
    fn no_record_at_all_is_the_absent_branch_and_the_supervisor_is_not_asked() {
        let placed = an_identity();
        let row = a_row(Some(&placed));
        let (read, asks, _) = read_with(
            &row,
            TheHalvesAtTheDesk::holding(WhatTheReadFound::NoRecordAtAll),
        );
        assert_eq!(read.check, InstallationCheck::Absent);
        assert_eq!(read.identity_on_the_machine, None);
        // **`State, identity and recovery` Y14**: the check never rests on
        // reachability, so nothing a supervisor could say is consulted here.
        assert_eq!(asks, 0);
    }

    /// **Branch three: present and different.**
    #[test]
    fn another_installations_value_is_present_and_different_whatever_the_supervisor_says() {
        let somebody_elses = an_identity();
        let row = a_row(Some(&an_identity()));
        let (read, asks, _) = read_with(
            &row,
            TheHalvesAtTheDesk::holding(WhatTheReadFound::AnIdentity(somebody_elses.clone()))
                .whose_supervisor_says(WhatTheSupervisorSaid::ItIsRunningUnder(
                    somebody_elses.clone(),
                )),
        );
        assert_eq!(read.check, InstallationCheck::PresentAndDifferent);
        assert_eq!(read.identity_on_the_machine, Some(somebody_elses.as_text()));
        assert_eq!(asks, 0, "a supervisor answering would not make it Smoozy's");
    }

    /// **Branch four: present, and the supervisor did not answer — and it is
    /// not the absent branch.**
    ///
    /// **This is the assertion the whole file is written around.** The two
    /// branches are told apart by what becomes of the values the row holds:
    /// *absent* disqualifies every one of them and this one leaves every one of
    /// them exactly as it is.
    #[test]
    fn a_value_that_matches_and_a_silent_supervisor_discards_nothing_and_is_not_absent() {
        use smoozy_state::identities::MachineDerivedValue;

        let placed = an_identity();
        let row = a_row(Some(&placed));
        let (read, asks, _) = read_with(
            &row,
            TheHalvesAtTheDesk::holding(WhatTheReadFound::AnIdentity(placed.clone())),
        );
        assert_eq!(
            read.check,
            InstallationCheck::PresentAndTheSupervisorDidNotAnswer
        );
        assert_eq!(asks, 1);
        assert_ne!(read.check, InstallationCheck::Absent);
        assert_eq!(
            read.check.what_becomes_of_machine_derived_values(),
            MachineDerivedValue::StandsUnchanged,
            "nothing is discarded on the fourth branch"
        );
        assert_ne!(
            read.check.what_becomes_of_machine_derived_values(),
            InstallationCheck::Absent.what_becomes_of_machine_derived_values(),
            "the fourth branch and the absent branch do different things to the \
             values the row holds, which is what makes them two branches"
        );
        // **`State, identity and recovery` V1 step 1**: reconciliation stops
        // here, and it is the only outcome that stops it.
        assert!(read.check.reconciliation_stops_at_step_one());
        // **The identity is present on the machine and unconfirmed**, so it is
        // carried as an identity and never as the explicit absence the absent
        // branch carries.
        assert_eq!(read.identity_on_the_machine, Some(placed.as_text()));
    }

    /// **A supervisor answering for another installation has not confirmed this
    /// one**, and it is the unconfirmed branch rather than either that
    /// discards.
    #[test]
    fn a_supervisor_naming_a_different_installation_confirms_nothing_and_discards_nothing() {
        let placed = an_identity();
        let row = a_row(Some(&placed));
        let (read, _, _) = read_with(
            &row,
            TheHalvesAtTheDesk::holding(WhatTheReadFound::AnIdentity(placed.clone()))
                .whose_supervisor_says(WhatTheSupervisorSaid::ItIsRunningUnder(an_identity())),
        );
        assert_eq!(
            read.check,
            InstallationCheck::PresentAndTheSupervisorDidNotAnswer
        );
    }

    // --------------------------------- the readings that failed, and they are
    // --------------------------------- never absence

    /// **A record that will not read back is never folded into absent**, and
    /// neither is a read that could not be taken.
    #[test]
    fn a_reading_that_failed_never_discards() {
        let placed = an_identity();
        let row = a_row(Some(&placed));
        for found in [
            WhatTheReadFound::ARecordThatDoesNotReadBack,
            WhatTheReadFound::TheReadCouldNotBeTaken,
        ] {
            let (read, asks, _) = read_with(&row, TheHalvesAtTheDesk::holding(found.clone()));
            assert_ne!(read.check, InstallationCheck::Absent, "{found:?}");
            assert_ne!(
                read.check,
                InstallationCheck::PresentAndDifferent,
                "{found:?}"
            );
            assert_eq!(
                read.check,
                InstallationCheck::PresentAndTheSupervisorDidNotAnswer,
                "{found:?}"
            );
            assert_eq!(asks, 0, "{found:?}");
        }
    }

    /// **A row Smoozy itself wrote and can no longer read discards nothing**,
    /// and no request goes out on it.
    #[test]
    fn a_row_whose_own_values_will_not_read_back_discards_nothing() {
        let mut row = a_row(Some(&an_identity()));
        row.last_confirmed_installation_identity = Some("not an identity".into());
        let (read, asks, areas) = read_with(
            &row,
            TheHalvesAtTheDesk::holding(WhatTheReadFound::AnIdentity(an_identity())),
        );
        assert_eq!(
            read.check,
            InstallationCheck::PresentAndTheSupervisorDidNotAnswer
        );
        assert_eq!(asks, 0);
        assert!(areas.is_empty(), "the machine is not asked at all");
    }

    /// **What the reader told, in order**, for the tests below.
    #[derive(Clone, Default)]
    struct WhatWasTold(Arc<Mutex<Vec<&'static str>>>);

    impl WhatTheSupervisorDid for WhatWasTold {
        fn it_did_not_answer(&mut self) {
            self.0.lock().expect("told").push("did not answer");
        }
        fn it_answered(&mut self) {
            self.0.lock().expect("told").push("answered");
        }
    }

    fn told_by(row: &ServerRow, halves: TheHalvesAtTheDesk) -> (InstallationCheck, Vec<&'static str>) {
        let told = WhatWasTold::default();
        let mut reading =
            TheMachinesInstallationIdentity::for_this_server(row, halves).telling(told.clone());
        let mut nothing = ();
        let read = tauri::async_runtime::block_on(reading.read(&mut nothing));
        let said = told.0.lock().expect("told").clone();
        (read.check, said)
    }

    /// **Step 10b's first audit round, item 8: the reader says what the
    /// supervisor did, and only where it knows.** The supervisor asked and not
    /// confirming is the fourth branch met on the machine; confirming is its
    /// answer; a record that would not read back is the fourth branch too. **A
    /// row whose own values will not read back tells nothing** — the machine
    /// was not asked, and *the supervisor is unavailable* would be a claim
    /// Smoozy does not know. The two branches that do not ask tell nothing.
    #[test]
    fn the_reader_tells_what_the_supervisor_did_and_only_where_it_knows() {
        let placed = an_identity();
        let row = a_row(Some(&placed));

        let (check, said) = told_by(
            &row,
            TheHalvesAtTheDesk::holding(WhatTheReadFound::AnIdentity(placed.clone())),
        );
        assert_eq!(check, InstallationCheck::PresentAndTheSupervisorDidNotAnswer);
        assert_eq!(said, vec!["did not answer"]);

        let (check, said) = told_by(
            &row,
            TheHalvesAtTheDesk::holding(WhatTheReadFound::AnIdentity(placed.clone()))
                .whose_supervisor_says(WhatTheSupervisorSaid::ItIsRunningUnder(placed.clone())),
        );
        assert_eq!(check, InstallationCheck::PresentAndEqualConfirmed);
        assert_eq!(said, vec!["answered"]);

        let (_, said) = told_by(
            &row,
            TheHalvesAtTheDesk::holding(WhatTheReadFound::ARecordThatDoesNotReadBack),
        );
        assert_eq!(said, vec!["did not answer"]);

        let mut unreadable = row.clone();
        unreadable.last_confirmed_installation_identity = Some("not an identity".into());
        let (check, said) = told_by(
            &unreadable,
            TheHalvesAtTheDesk::holding(WhatTheReadFound::AnIdentity(placed.clone())),
        );
        assert_eq!(check, InstallationCheck::PresentAndTheSupervisorDidNotAnswer);
        assert!(said.is_empty(), "the machine was not asked, so nothing is claimed");

        for found in [WhatTheReadFound::NoRecordAtAll, WhatTheReadFound::AnIdentity(an_identity())] {
            let (_, said) = told_by(&row, TheHalvesAtTheDesk::holding(found));
            assert!(said.is_empty(), "absent and different ask no supervisor");
        }
    }

    /// **A row with no placement recorded is the absent branch**, and nothing
    /// looks harder for an area to read.
    #[test]
    fn a_row_with_no_placement_recorded_is_absent_and_reads_nothing() {
        let mut row = a_row(None);
        row.recorded_machine_root = None;
        let (read, asks, areas) = read_with(
            &row,
            TheHalvesAtTheDesk::holding(WhatTheReadFound::AnIdentity(an_identity())),
        );
        assert_eq!(read.check, InstallationCheck::Absent);
        assert_eq!(read.identity_on_the_machine, None);
        assert_eq!(asks, 0);
        assert!(areas.is_empty());
    }

    /// **`Job files and outputs` L4a's second arrangement**: where the package
    /// area was placed on a mount of its own, that is where the record is read
    /// from.
    #[test]
    fn the_read_follows_the_package_area_to_the_mount_it_was_placed_on() {
        let placed = an_identity();
        let mut row = a_row(Some(&placed));
        row.recorded_machine_root = Some(RecordedMachineRoot {
            path: "/ephemeral/smoozy".into(),
            the_package_areas_own_location: Some("/state/smoozy".into()),
            read_under: placed.as_text(),
        });
        let (_, _, areas) = read_with(
            &row,
            TheHalvesAtTheDesk::holding(WhatTheReadFound::AnIdentity(placed)),
        );
        assert_eq!(areas, vec!["/state/smoozy/package".to_string()]);
    }

    // ---------------------------------------------- the sequence, run on it

    /// The five seams `Servers` reaches through a trait, four of them stood in
    /// for here so the fifth can be the real one.
    ///
    /// **`Servers`' own desk is `App/Servers/src/testing.rs` and it cannot be
    /// reached from here** — it is `#[cfg(test)]` and crate-private, so it
    /// compiles only when that crate is under test. That crate's own
    /// integration tests write these same stand-ins for the same reason. **What
    /// the swap has to show is unchanged**: the real read goes into
    /// `TheActingHalf` where the stand-in went, one field, and the sequence runs
    /// on it.
    struct AConnection {
        answers: HashMap<&'static str, CommandResult>,
    }

    impl CommandChannel for AConnection {
        async fn run(&mut self, command: CommandString) -> Result<CommandResult, CommandFailure> {
            self.answers
                .get(command.as_str())
                .cloned()
                .ok_or(CommandFailure::TheRequestDidNotReachTheMachine)
        }
    }

    impl TakesAFreeSpaceReading for AConnection {
        async fn free_space(&mut self, _path: &str) -> Option<FreeSpace> {
            None
        }
    }

    struct TheOpener;

    impl OpenConnections for TheOpener {
        type Connection = AConnection;

        async fn open(
            &mut self,
            _endpoint: Endpoint,
            _credential: Credential,
            mut decide: Box<dyn DecideHostTrust>,
            _deadline: Duration,
        ) -> Result<AConnection, OpenFailure> {
            if decide.decide(b"ssh-ed25519 AAAAC3NzaC1lZDI1NTE5") == HostTrust::Stop {
                return Err(OpenFailure::HostTrustStoppedIt);
            }
            let mut answers = HashMap::new();
            answers.insert(
                "uname -s",
                CommandResult {
                    output: b"Linux\n".to_vec(),
                    complaint: Vec::new(),
                    status: Some(0),
                },
            );
            Ok(AConnection { answers })
        }

        fn ended(&mut self, _connection: &AConnection) -> Option<Ending> {
            None
        }

        async fn wait_until_it_ends(&mut self, _connection: &mut AConnection) -> Ending {
            Ending::TornDownBySmoozy
        }

        async fn tear_down(&mut self, _connection: AConnection) -> Ending {
            Ending::TornDownBySmoozy
        }
    }

    struct TheCredentials;

    impl SupplyTheCredential for TheCredentials {
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
                private_key_file: std::path::PathBuf::from("/nowhere/id_ed25519"),
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

    #[derive(Default)]
    struct ThePublisher;

    impl WhatServersPublishes for ThePublisher {
        fn the_observed_value(&mut self, _server: &str, _value: ObservedValue) {}
        fn the_next_attempt_due(&mut self, _server: &str, _reading: NextAttemptDue) {}
        fn raise_the_connection_condition(&mut self, _server: &str, _says: WhatTheRecordSays) {}
        fn withdraw_the_connection_condition(&mut self, _server: &str) {}
        fn the_changed_identity_warning(&mut self, _server: &str) {}
        fn record_the_scanned_host_key(&mut self, _server: &str, _key: RememberedHostKey) {}
    }

    struct TheWaiter {
        waits: usize,
    }

    impl Wait for TheWaiter {
        async fn until(&mut self, _how_long: Duration) {
            self.waits += 1;
        }

        fn now(&mut self) -> SystemTime {
            SystemTime::UNIX_EPOCH + Duration::from_secs(1_000_000)
        }
    }

    /// **`Servers` `Q1` step 5, run through the real read.**
    ///
    /// The sequence takes the acting half as one value, so the real
    /// implementation goes in where the stand-in went and nothing else about
    /// the desk changes. **A real implementation that could not be put there
    /// would have the wrong shape**, and this is what says it has the right one.
    #[test]
    fn the_sequence_runs_step_five_through_the_real_read() {
        let placed = an_identity();
        let row = a_row(Some(&placed));
        let mut half = TheActingHalf {
            opener: TheOpener,
            credentials: TheCredentials,
            installation: TheMachinesInstallationIdentity::for_this_server(
                &row,
                TheHalvesAtTheDesk::holding(WhatTheReadFound::AnIdentity(placed.clone()))
                    .whose_supervisor_says(WhatTheSupervisorSaid::ItIsRunningUnder(placed)),
            ),
            publishes: ThePublisher,
            wait: TheWaiter { waits: 0 },
        };
        let attempt = AnAttempt {
            row: &row,
            who: WhoStartedIt::TheUser,
            the_first_connection_to_this_server: true,
            role: ConnectionRole::Primary,
            deadline: Duration::from_secs(20),
        };
        let ended = tauri::async_runtime::block_on(sequence::run(
            attempt,
            &mut half,
            &mut ChangedIdentityWarning::none_yet(),
            &mut |_| {},
        ));
        let Attempt::ItIsLive(gave) = ended else {
            panic!("the connection is live")
        };
        assert_eq!(
            gave.installation,
            Some(InstallationCheck::PresentAndEqualConfirmed)
        );
    }

    /// **`Servers` Q19's re-take, run through the real read.**
    ///
    /// The fourth branch is re-taken alone until it settles, so a read that
    /// never settles is re-taken and the sequence never reaches step 6. **This
    /// drives one re-take and then breaks out on the connection ending**, which
    /// is one of the three things Q19 closes the re-take on.
    #[test]
    fn the_fourth_branch_is_retaken_on_the_live_connection() {
        let placed = an_identity();
        let row = a_row(Some(&placed));
        let halves = TheHalvesAtTheDesk::holding(WhatTheReadFound::AnIdentity(placed));
        let asks = Arc::clone(&halves.areas_asked);
        let mut half = TheActingHalf {
            opener: TheOpenerThatEnds { opened: false },
            credentials: TheCredentials,
            installation: TheMachinesInstallationIdentity::for_this_server(&row, halves),
            publishes: ThePublisher,
            wait: TheWaiter { waits: 0 },
        };
        let attempt = AnAttempt {
            row: &row,
            who: WhoStartedIt::TheUser,
            the_first_connection_to_this_server: true,
            role: ConnectionRole::Primary,
            deadline: Duration::from_secs(20),
        };
        let ended = tauri::async_runtime::block_on(sequence::run(
            attempt,
            &mut half,
            &mut ChangedIdentityWarning::none_yet(),
            &mut |_| {},
        ));
        assert!(matches!(ended, Attempt::ItStopped(_)));
        assert_eq!(
            asks.lock().expect("the areas").len(),
            1,
            "the read is taken once and the re-take stops at the break declaration"
        );
    }

    /// The opener above, but the connection it gives has already ended — which
    /// is what closes the re-take.
    struct TheOpenerThatEnds {
        opened: bool,
    }

    impl OpenConnections for TheOpenerThatEnds {
        type Connection = AConnection;

        async fn open(
            &mut self,
            _endpoint: Endpoint,
            _credential: Credential,
            mut decide: Box<dyn DecideHostTrust>,
            _deadline: Duration,
        ) -> Result<AConnection, OpenFailure> {
            if decide.decide(b"ssh-ed25519 AAAAC3NzaC1lZDI1NTE5") == HostTrust::Stop {
                return Err(OpenFailure::HostTrustStoppedIt);
            }
            self.opened = true;
            let mut answers = HashMap::new();
            answers.insert(
                "uname -s",
                CommandResult {
                    output: b"Linux\n".to_vec(),
                    complaint: Vec::new(),
                    status: Some(0),
                },
            );
            Ok(AConnection { answers })
        }

        fn ended(&mut self, _connection: &AConnection) -> Option<Ending> {
            Some(Ending::Announced)
        }

        async fn wait_until_it_ends(&mut self, _connection: &mut AConnection) -> Ending {
            Ending::Announced
        }

        async fn tear_down(&mut self, _connection: AConnection) -> Ending {
            Ending::TornDownBySmoozy
        }
    }

    // ----------------------------- step 10c's eighth-pass fix piece 3

    /// **A period's reader reads the row as the record holds it at each
    /// read.** The period began on a bare machine — no placement recorded — and
    /// the ready action then placed Smoozy and recorded the installation. **The
    /// next connection under the same period** (the return after the driver's
    /// restart is the case the eighth pass met) **must read the machine's new
    /// installation as Smoozy's, confirmed.** With the reader frozen at the
    /// period's start it read *absent* without asking the machine at all, and
    /// `Servers` Q1 step 7 never ran on it.
    #[test]
    fn a_periods_reader_reads_the_row_as_it_stands_at_each_read() {
        let placed = an_identity();
        let mut bare = a_row(None);
        bare.recorded_machine_root = None;
        let the_row = Arc::new(Mutex::new(bare.clone()));
        let halves = TheHalvesAtTheDesk::holding(WhatTheReadFound::AnIdentity(placed.clone()))
            .whose_supervisor_says(WhatTheSupervisorSaid::ItIsRunningUnder(placed.clone()));
        let areas = Arc::clone(&halves.areas_asked);
        let mut reading = TheMachinesInstallationIdentity::for_this_server(&bare, halves)
            .reading_the_row_as_it_stands({
                let the_row = Arc::clone(&the_row);
                move || Some(the_row.lock().expect("the row").clone())
            });
        let mut nothing = ();

        let first = tauri::async_runtime::block_on(reading.read(&mut nothing));
        assert_eq!(first.check, InstallationCheck::Absent, "a bare machine reads absent");
        assert!(areas.lock().expect("the areas").is_empty());

        // **The ready action places Smoozy and records it against the row.**
        *the_row.lock().expect("the row") = a_row(Some(&placed));

        let after = tauri::async_runtime::block_on(reading.read(&mut nothing));
        assert_eq!(
            after.check,
            InstallationCheck::PresentAndEqualConfirmed,
            "the connection after the placement read the row the period began with"
        );
        assert_eq!(after.identity_on_the_machine, Some(placed.as_text()));
    }

    /// **A *Set up again* issues a new installation, and the next connection
    /// under the same period reads it as Smoozy's** rather than as another
    /// installation's — which would have discarded the row's machine-derived
    /// values and marked every job whose start was never heard.
    #[test]
    fn a_new_installation_recorded_mid_period_is_not_read_as_another_ones() {
        let first = an_identity();
        let second = an_identity();
        let the_row = Arc::new(Mutex::new(a_row(Some(&first))));
        let halves = TheHalvesAtTheDesk::holding(WhatTheReadFound::AnIdentity(second.clone()))
            .whose_supervisor_says(WhatTheSupervisorSaid::ItIsRunningUnder(second.clone()));
        let mut reading = TheMachinesInstallationIdentity::for_this_server(&a_row(Some(&first)), halves)
            .reading_the_row_as_it_stands({
                let the_row = Arc::clone(&the_row);
                move || Some(the_row.lock().expect("the row").clone())
            });
        *the_row.lock().expect("the row") = a_row(Some(&second));
        let mut nothing = ();
        let read = tauri::async_runtime::block_on(reading.read(&mut nothing));
        assert_eq!(read.check, InstallationCheck::PresentAndEqualConfirmed);
    }

    /// The two halves, the supervisor answering from a list in turn — the
    /// state area holding the installation throughout.
    struct TheHalvesInTurn {
        holds: Identity,
        answers: std::collections::VecDeque<WhatTheSupervisorSaid>,
        reads: Arc<Mutex<usize>>,
    }

    impl<C: Send> TheTwoHalvesOfTheCheck<C> for TheHalvesInTurn {
        async fn read_the_state_area(&mut self, _channel: &mut C, _area: &str) -> WhatTheReadFound {
            *self.reads.lock().expect("the reads") += 1;
            WhatTheReadFound::AnIdentity(self.holds.clone())
        }

        async fn ask_the_supervisor(&mut self, _channel: &mut C, _area: &str) -> WhatTheSupervisorSaid {
            if self.answers.len() > 1 {
                self.answers.pop_front().expect("an answer")
            } else {
                self.answers.front().cloned().expect("an answer")
            }
        }
    }

    /// A machine whose clock answers, so step 6's reading is taken.
    fn a_machine_whose_clock_answers() -> AConnection {
        let mut answers = HashMap::new();
        answers.insert(
            "date +%s",
            CommandResult {
                output: b"1790000000\n".to_vec(),
                complaint: Vec::new(),
                status: Some(0),
            },
        );
        AConnection { answers }
    }

    fn what_an_absent_period_read(live: bool) -> Option<crate::servers::a_period::WhatTheLastConnectionRead> {
        Some(crate::servers::a_period::WhatTheLastConnectionRead {
            check: InstallationCheck::Absent,
            a_fresh_reading_was_kept: true,
            on_the_connection_that_is_live: live,
        })
    }

    /// `Servers` Q19a's re-take, driven with no connection under it.
    fn the_retake(
        read: Option<crate::servers::a_period::WhatTheLastConnectionRead>,
        row: &ServerRow,
        halves: TheHalvesInTurn,
        is_live: &dyn Fn() -> bool,
    ) -> (
        Option<(InstallationCheck, Option<smoozy_servers::hardware::HardwareReading>)>,
        usize,
    ) {
        let mut reading = TheMachinesInstallationIdentity::for_this_server(row, halves);
        let mut nothing = ();
        let mut the_machine = a_machine_whose_clock_answers();
        let mut wait = TheWaiter { waits: 0 };
        let got = tauri::async_runtime::block_on(the_retake_at_the_commit(
            read,
            &mut reading,
            &mut nothing,
            &mut the_machine,
            row,
            &mut wait,
            is_live,
        ));
        (got, wait.waits)
    }

    /// **`Servers` Q19a: a period whose live connection read *absent*, then
    /// placed, then answering, reads the confirmed branch at the commit and
    /// takes step 6 under it** — the hardware reading carrying the identity
    /// just placed (`Servers` H17, H18). No reconnect, no new attempt, and no
    /// pause: it is one re-take at one moment.
    #[test]
    fn at_the_commit_an_absent_period_is_read_again_and_reads_the_placed_installation() {
        let placed = an_identity();
        let row = a_row(Some(&placed));
        let reads = Arc::new(Mutex::new(0));
        let (got, waits) = the_retake(
            what_an_absent_period_read(true),
            &row,
            TheHalvesInTurn {
                holds: placed.clone(),
                answers: vec![WhatTheSupervisorSaid::ItIsRunningUnder(placed.clone())].into(),
                reads: Arc::clone(&reads),
            },
            &|| true,
        );
        let (check, hardware) = got.expect("step 5 is taken again at the commit");
        assert_eq!(check, InstallationCheck::PresentAndEqualConfirmed);
        let hardware = hardware.expect("step 6 is taken again under the branch it now reads");
        assert_eq!(hardware.read_under, Some(placed.as_text()));
        assert_eq!(*reads.lock().expect("the reads"), 1, "one re-take");
        assert_eq!(waits, 0, "one re-take is not a timer");
    }

    /// **Only where step 7 has not run.** *Set up again* on a period that read
    /// the confirmed branch — step 7 and `Desktop app and delivery` P10d's
    /// sequence already running — re-takes nothing and asks the machine
    /// nothing, so no second sequence is started beside the first (`Servers`
    /// Q18). **Nor does a branch read on a connection that is no longer the
    /// live one**, nor a connection that has ended by the commit.
    #[test]
    fn at_the_commit_nothing_is_read_again_where_step_seven_runs_or_the_connection_went() {
        let placed = an_identity();
        let row = a_row(Some(&placed));
        let halves = |reads: &Arc<Mutex<usize>>| TheHalvesInTurn {
            holds: placed.clone(),
            answers: vec![WhatTheSupervisorSaid::ItIsRunningUnder(placed.clone())].into(),
            reads: Arc::clone(reads),
        };
        for (read, live) in [
            (
                Some(crate::servers::a_period::WhatTheLastConnectionRead {
                    check: InstallationCheck::PresentAndEqualConfirmed,
                    a_fresh_reading_was_kept: true,
                    on_the_connection_that_is_live: true,
                }),
                true,
            ),
            (
                Some(crate::servers::a_period::WhatTheLastConnectionRead {
                    check: InstallationCheck::PresentAndDifferent,
                    a_fresh_reading_was_kept: true,
                    on_the_connection_that_is_live: true,
                }),
                true,
            ),
            (what_an_absent_period_read(false), true),
            (what_an_absent_period_read(true), false),
            (None, true),
        ] {
            let reads = Arc::new(Mutex::new(0));
            let (got, _) = the_retake(read, &row, halves(&reads), &|| live);
            assert!(got.is_none(), "{read:?} live {live}");
            assert_eq!(*reads.lock().expect("the reads"), 0, "{read:?} live {live}");
        }
    }

    /// **Where the re-take reads the fourth branch, `Servers` Q19 governs from
    /// there**: that step alone is re-taken on `Servers` Q19's own figures until
    /// it settles, and step 6 is taken once it does.
    #[test]
    fn at_the_commit_a_silent_supervisor_is_read_again_until_it_settles() {
        let placed = an_identity();
        let row = a_row(Some(&placed));
        let reads = Arc::new(Mutex::new(0));
        let (got, waits) = the_retake(
            what_an_absent_period_read(true),
            &row,
            TheHalvesInTurn {
                holds: placed.clone(),
                answers: vec![
                    WhatTheSupervisorSaid::Nothing,
                    WhatTheSupervisorSaid::Nothing,
                    WhatTheSupervisorSaid::ItIsRunningUnder(placed.clone()),
                ]
                .into(),
                reads: Arc::clone(&reads),
            },
            &|| true,
        );
        let (check, hardware) = got.expect("it settles");
        assert_eq!(check, InstallationCheck::PresentAndEqualConfirmed);
        assert!(hardware.is_some());
        assert_eq!(*reads.lock().expect("the reads"), 3);
        assert_eq!(waits, 2);
    }

    // ------------------------ `Server protocol and transfers` P3b, the fourth branch

    /// The two halves at the desk, with an inspection to answer as well.
    struct TheHalvesThatInspect {
        halves: TheHalvesAtTheDesk,
        the_package_area_holds: Option<WhatThePackageAreaHolds>,
    }

    impl<C: Send> TheTwoHalvesOfTheCheck<C> for TheHalvesThatInspect {
        async fn read_the_state_area(&mut self, channel: &mut C, area: &str) -> WhatTheReadFound {
            self.halves.read_the_state_area(channel, area).await
        }

        async fn ask_the_supervisor(&mut self, channel: &mut C, area: &str) -> WhatTheSupervisorSaid {
            self.halves.ask_the_supervisor(channel, area).await
        }

        async fn inspect_the_package_area(
            &mut self,
            _channel: &mut C,
            _area: &str,
        ) -> Option<WhatThePackageAreaHolds> {
            self.the_package_area_holds.clone()
        }
    }

    /// A package area holding a program of `interval` at `sequence`.
    fn a_program_of(sequence: u64, interval: (u32, u32)) -> WhatThePackageAreaHolds {
        WhatThePackageAreaHolds {
            it_was_read: true,
            its_release_sequence: Some(sequence),
            the_declared_intervals: vec![interval],
            ..Default::default()
        }
    }

    /// **`Server protocol and transfers` P3b: a program that speaks no protocol
    /// this build speaks is *incompatible*, with its direction, and V1's
    /// condition is not raised.** Step 10c's transfer wave piece 8; the gap it
    /// proves is that until then every such program read as Smoozy not
    /// responding. **A program that shares a protocol and does not answer is
    /// still V1's**, and a supervisor that answers again takes the
    /// incompatibility away.
    #[test]
    fn a_program_of_no_shared_protocol_is_incompatible_and_never_v1() {
        use crate::servers::published::{what_the_server_carries, WhatSmoozyOnTheServerIs};
        let ours = THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS.0;
        let placed = an_identity();
        let mut row = a_row(Some(&placed));
        row.local_server_identifier = Identity::issue(IdentityKind::LocalServer)
            .expect("randomness")
            .as_text();
        let server = row.local_server_identifier.clone();

        let read_it = |holds: Option<WhatThePackageAreaHolds>, says: WhatTheSupervisorSaid| {
            let told = WhatWasTold::default();
            let halves = TheHalvesThatInspect {
                halves: TheHalvesAtTheDesk::holding(WhatTheReadFound::AnIdentity(placed.clone()))
                    .whose_supervisor_says(says),
                the_package_area_holds: holds,
            };
            let mut reading =
                TheMachinesInstallationIdentity::for_this_server(&row, halves).telling(told.clone());
            let mut nothing = ();
            let read = tauri::async_runtime::block_on(reading.read(&mut nothing));
            let told = told.0.lock().expect("told").clone();
            (read.check, told)
        };

        // An older program, speaking only a version below this build's.
        let (check, told) = read_it(Some(a_program_of(3, (ours - 1, ours - 1))), WhatTheSupervisorSaid::Nothing);
        assert_eq!(check, InstallationCheck::PresentAndTheSupervisorDidNotAnswer);
        assert!(told.is_empty(), "V1 was raised on a program of another protocol: {told:?}");
        assert_eq!(
            what_the_server_carries(&server),
            Some(WhatSmoozyOnTheServerIs::TooOldAndItsWorkCannotBeControlledFromHere),
            "an older program of another protocol is not marked too old"
        );

        // A newer program, speaking only a version above it.
        let (_, told) = read_it(Some(a_program_of(3, (ours + 1, ours + 1))), WhatTheSupervisorSaid::Nothing);
        assert!(told.is_empty());
        assert_eq!(what_the_server_carries(&server), Some(WhatSmoozyOnTheServerIs::TooNew));

        // **A program that shares the protocol and does not answer is V1's.**
        let (_, told) = read_it(Some(a_program_of(3, (ours, ours))), WhatTheSupervisorSaid::Nothing);
        assert_eq!(told, vec!["did not answer"]);
        // **And a reading that could not be taken changes nothing either.**
        let (_, told) = read_it(None, WhatTheSupervisorSaid::Nothing);
        assert_eq!(told, vec!["did not answer"]);

        // **A supervisor that answers speaks this build's protocol**, so the
        // incompatibility it stood under is withdrawn.
        let (check, told) = read_it(None, WhatTheSupervisorSaid::ItIsRunningUnder(placed.clone()));
        assert_eq!(check, InstallationCheck::PresentAndEqualConfirmed);
        assert_eq!(told, vec!["answered"]);
        assert_eq!(what_the_server_carries(&server), None);
    }

    /// **`Server protocol and transfers` B4's reading, composed**: the active
    /// release, the intervals, the phase record and the registration's content,
    /// each from its own record — and a machine that has never held a package
    /// reads as nothing active rather than as a zero.
    #[test]
    fn an_inspection_reads_the_active_release_and_nothing_where_none_is_active() {
        use smoozy_server_protocol::bootstrap::{compose_the_inspection, WhatARecordHeld, WhatTheRecordsHeld};
        use smoozy_server_protocol::reconnection::WhatTheStateAreaHeld;
        let never = compose_the_inspection(WhatTheRecordsHeld {
            phase_record: WhatARecordHeld::Nothing,
            boot_registration: WhatARecordHeld::Nothing,
            installation_identity: WhatTheStateAreaHeld::Nothing,
            the_pointer_points_at: None,
        });
        let read = what_an_inspection_shows(&never, WhatARecordHeld::Nothing, WhatARecordHeld::Nothing);
        assert!(read.it_was_read);
        assert_eq!(read.its_release_sequence, None);
        assert!(read.the_phase_record.is_none());
        assert!(read.it_speaks_what_this_build_speaks());
        assert_eq!(read.what_it_is_where_no_protocol_is_shared(Some(9)), None);
    }
}
