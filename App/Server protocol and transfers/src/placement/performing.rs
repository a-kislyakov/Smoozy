//! **The transaction itself, walked: the six ordered acts, the health exchange,
//! and the rollback.**
//!
//! # Where the decisions come from
//!
//! **Not from here.** Every branch below is answered by a function in
//! `smoozy_runtime::package` — `Rented-server runtime` G17's retry table,
//! G13's exchange, G14's restore, G14a's record, G21's block, G26's refusal,
//! G29's comparison. **This file is the order and the acts.**
//!
//! # The health exchange, and the one thing it needed that nothing had
//!
//! `Rented-server runtime` G12 makes the exchange *"one request over the
//! machine-local endpoint to the supervisor the commit has just activated"*.
//! **The desktop has no route to a machine-local endpoint** and needs none: the
//! machine's own program answers a bare invocation as the SSH command adapter,
//! whose whole job is to frame one request, pass it to the supervisor over that
//! endpoint, and return the correlated result. §1.1 is exact about the carriage
//! — after hand-over Smoozy sends *"one constant, installation-owned command —
//! the active package's fixed entry-point name … and sends every value on that
//! entry point's standard input in this document's length-delimited frame"*.
//!
//! **What was missing is that the entry point's path is not a constant.** It
//! sits in the package area, and where the package area sits is
//! `Job files and outputs` L4a's — a location chosen once per installation from
//! candidates read off the machine. §1.1 forbids a value of machine origin in a
//! command string outright, and a command string in this crate may hold only a
//! fixed literal, which is that prohibition kept by the type.
//!
//! **So the entry point is reached by §1.1's own planted-link carriage**, at
//! [`THE_ENTRY_POINTS_CARRIAGE_SITE`], and the constant command names the token
//! rather than the value. **The link points at the package area rather than at
//! the entry point**, and that is the whole of why this works: the machine's own
//! program takes its package area from the directory it was invoked from, so an
//! invocation at `<token>/smoozy-server` gives it `<token>` — which is the
//! package area, reached through one link, with every record under it resolving
//! on the machine. **A link planted at the entry point itself would give it the
//! carriage directory instead**, and it would then look for the endpoint's
//! recorded path somewhere that has none.
//!
//! # What the exchange's failure branch is worth
//!
//! `Rented-server runtime` G13: *"A supervisor that does not answer at all is
//! also a failed exchange"*, and the caller does not distinguish the two. **So
//! every way this can go wrong — no link, no adapter, a command that ended
//! non-zero, a frame that does not read, a result correlated to another
//! request — is the same *did not answer*, and it restores.** That is the safe
//! direction and it is why nothing here invents a third answer.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rented-server runtime` G10, G12, G13, G14, G14a,
//! G15, G16, G17, G18, G19, G20, G21, G22, G23, G25, G26, G27, G28, G29, G30,
//! G31, G31a, G1a, G1b, K2, K4, K5, K6, K7, K11, K15, K16, K16b, K17, K19, Z2,
//! Z3.
//! `Job files and outputs` L4a.
//!
//! **A marker this plan also defines is never listed there and is written out
//! with its document at the site**, because a header list cannot tell the two
//! apart and a bare one there would resolve to a real and wrong clause.
//! `Rented-server runtime` G1, `Rented-server runtime` G3,
//! `Rented-server runtime` G4, `Rented-server runtime` G5,
//! `Rented-server runtime` G6, `Rented-server runtime` G7 and
//! `Rented-server runtime` G9 are of that kind here.

use smoozy_framing::framing::{
    read_one_result, write_request_frame, Answered, Request,
    THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
};
use smoozy_framing::requests::{self as asked, THE_SUPERVISOR_ENDS_ITSELF_REQUEST};
use smoozy_runtime::machine_kind::MachineKind;
use smoozy_runtime::package::commit::{
    may_be_committed, the_health_exchange, the_ordered_pair, the_record_a_rollback_restores,
    what_the_exchange_produces, whether_the_registration_changes, CommitStep, HealthAnswer,
    TheOrderedPair, TheRestoresTwoSteps, WhatTheExchangeProduces,
    WhetherTheRegistrationChanges, WhetherTheStagedVersionMayBeCommitted,
};
use smoozy_runtime::package::phase::{
    a_fresh_attempts_first_write, ClaimOutcome, Phase, PhaseRecord, PreviousVersion,
    TransactionOwner,
};
use smoozy_runtime::package::release::{
    compare_at_rest as compare_the_recorded_values_at_rest, may_proceed, AtRestMeasurement,
    VerifiedReleaseDescriptor, WhetherTheRequestMayProceed,
};
use smoozy_runtime::package::retry::{
    staged_material_may_be_reused, what_a_different_identity_may_do,
    what_a_same_identity_retry_does, whether_the_commit_may_proceed, AcceptedWorkStanding,
    DifferentIdentity, SameIdentityRetry, WhatThePointerNames, WhetherTheCommitMayProceed,
};
use smoozy_runtime::registration::{RegistrationConfirmation, WhatTheRegistrationStepLeft};
use smoozy_state::identities::{Identity, IdentityKind};
use tokio::io::{AsyncRead, AsyncWrite};

use crate::bootstrap::WhatARecordHeld;
use crate::carriage::{plant, take_it_away, CarriageSite};
use crate::placement::claim::{
    release_the_claim_at_the_terminal_write, take_the_claim, WhatTakingTheClaimReached,
};
use crate::placement::records::{
    make_the_installation_identity_durable, read_the_boot_registration_record,
    read_the_phase_record, record_the_endpoints_path, rename_the_pointer_to,
    the_active_pointers_value, write_the_boot_registration_record, write_the_phase_record,
    WhetherTheDurabilityOrderingApplied,
};
use crate::placement::registration::{
    confirm_the_registration, enable_the_unit, grant_lingering, install_the_hook,
    place_the_registration, read_the_linger_flag_back, start_the_supervisor_the_pointer_names,
    the_container_hooks_content, the_user_manager_answers, TheRootStepOnAContainer,
};
use crate::placement::{remove_the_previous_package, sweep_the_abandoned_staging};
use crate::release::verifying::{
    compare_at_rest, make_the_artifact_executable, read_back_at_rest, AVerifiedRelease,
    TheDescriptorsValues, TheReadingAtRest, WhatTheReadBackFound,
};
use crate::subsystem::client::Subsystem;
use crate::subsystem::SubsystemFailure;
use crate::transfers::moving::THE_MOST_WRITES_IN_FLIGHT;
use crate::transport::ssh::Connection;
use crate::transport::CommandString;

// ---------------------------------------------------------------- the carriage

/// **The entry point's carriage site — §1.1's third route, for the one class of
/// value with no other.**
///
/// **The link points at the package area, not at the entry point.** See this
/// file's own opening for why: the machine's own program takes its package area
/// from the directory it was invoked from.
pub const THE_ENTRY_POINTS_CARRIAGE_SITE: CarriageSite = CarriageSite {
    link_path_on_the_subsystem: ".smoozy-carriage/package-area",
    named_in_a_command_as: "~/.smoozy-carriage/package-area",
};

/// **§1.1's one constant, installation-owned command after hand-over**, naming
/// the carriage token rather than the value.
pub const THE_HEALTH_EXCHANGE_COMMAND: CommandString =
    CommandString::constant("~/.smoozy-carriage/package-area/smoozy-server");

// --------------------------------------------------- the four things G12 asks

/// **`Rented-server runtime` G12's request name — the one spelling, re-exported.**
///
/// **This was a second copy of the machine's own until step 10c's first piece**,
/// held to it by a test that read that file's source, because the machine's
/// acting half is gated to a unix and the desktop runs on three systems. The
/// name now lives in `smoozy_framing::requests`, which both ends read, and the
/// machine's path re-exports the same constant. **The four field names below are
/// still a copy**, held by the test that reads the machine's source.
pub use smoozy_framing::requests::THE_HEALTH_ANSWER_REQUEST;

/// **`Rented-server runtime` G12's four things, and nothing else is taken as
/// evidence.**
pub const THE_FOUR_THINGS_THE_HEALTH_EXCHANGE_ASKS_FOR: [&str; 4] =
    ["package version", "run", "installation", "phase"];

/// The four things, read out of one framed result.
///
/// **A result carrying three of the four is not a health answer**, and it is
/// [`None`] rather than a partly-filled one: `Rented-server runtime` G13 makes
/// an incomplete answer the same failed exchange as no answer, and both restore.
pub fn the_health_answer_in(answered: &Answered) -> Option<HealthAnswer> {
    Some(HealthAnswer {
        package_version: answered
            .fields
            .text(THE_FOUR_THINGS_THE_HEALTH_EXCHANGE_ASKS_FOR[0])?
            .to_string(),
        run: Identity::read_back(
            IdentityKind::Run,
            answered
                .fields
                .text(THE_FOUR_THINGS_THE_HEALTH_EXCHANGE_ASKS_FOR[1])?,
        )?,
        installation: Identity::read_back(
            IdentityKind::Installation,
            answered
                .fields
                .text(THE_FOUR_THINGS_THE_HEALTH_EXCHANGE_ASKS_FOR[2])?,
        )?,
        phase: serde_json::from_str(
            answered
                .fields
                .text(THE_FOUR_THINGS_THE_HEALTH_EXCHANGE_ASKS_FOR[3])?,
        )
        .ok()?,
    })
}

/// **`Rented-server runtime` G12's exchange, asked through the surface.**
///
/// **Not answering is an answer**, so this returns `Ok(None)` for it rather than
/// an error a caller might treat differently — which is
/// `Rented-server runtime` G13's own rule that the caller does not distinguish
/// the two failures.
///
/// **The carriage is planted and taken away around the one invocation, and
/// where the removal will not run the next plant is what replaces the link.**
/// That is the carriage's own answer and this site does no more than it:
/// [`crate::carriage::plant`] removes whatever stands at the token before
/// planting, so a link a stopped run left behind is met and replaced rather
/// than being an obstruction. **Nothing here writes an owed removal down**, and
/// this says so rather than claiming §7.8's recorded-staging discipline it does
/// not perform — what a removal that could not run leaves is one of Smoozy's own
/// links in the login's own directory until the next health exchange.
/// **C1: the identity is the initiating side's, chosen before the request is
/// sent**, so it is handed in rather than issued here. This exchange is a pure
/// reading, so what it carries the identity for is correlation rather than
/// idempotency (C6).
pub async fn ask_for_the_health_answer<S>(
    subsystem: &mut Subsystem<S>,
    connection: &mut Connection,
    package_area: &str,
    identity: Identity,
) -> Result<Option<HealthAnswer>, SubsystemFailure>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    Ok(
        match asked_once(subsystem, connection, package_area, identity).await? {
            WhatOneAskingCameTo::NothingAnswered => None,
            WhatOneAskingCameTo::AFrameCameBack(answer) => answer,
        },
    )
}

/// **What one asking of the health exchange came to — and the difference is
/// only what the deadline acts on.** Both are G13's *did not answer* once the
/// deadline is spent.
enum WhatOneAskingCameTo {
    /// **No frame came back**: the carriage could not be planted, the command
    /// did not run, or the adapter ended without a frame — which is what it
    /// does when no supervisor holds the endpoint yet.
    NothingAnswered,
    /// **A frame came back**, and it is the answer or it is not. A frame that
    /// does not read, answers another request or lacks one of the four things
    /// is `None` here, and is **not** asked again: something answered, and
    /// asking it again would be asking for a different answer.
    AFrameCameBack(Option<HealthAnswer>),
}

async fn asked_once<S>(
    subsystem: &mut Subsystem<S>,
    connection: &mut Connection,
    package_area: &str,
    identity: Identity,
) -> Result<WhatOneAskingCameTo, SubsystemFailure>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    let mut planted = match plant(
        subsystem,
        THE_ENTRY_POINTS_CARRIAGE_SITE,
        package_area.as_bytes(),
    )
    .await
    {
        Ok(planted) => planted,
        // **A carriage that could not be planted is a supervisor that cannot be
        // asked**, which is the *did not answer* branch and never a third
        // outcome.
        Err(_) => return Ok(WhatOneAskingCameTo::NothingAnswered),
    };

    let request = Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        identity,
        THE_HEALTH_ANSWER_REQUEST,
    );

    let said = connection
        .run_one_command_carrying_a_framed_request_on_its_standard_input(
            THE_HEALTH_EXCHANGE_COMMAND,
            &write_request_frame(&request),
        )
        .await;
    take_it_away(subsystem, &mut planted).await;

    let Ok(said) = said else {
        return Ok(WhatOneAskingCameTo::NothingAnswered);
    };
    if !said.ended_cleanly() {
        // **The adapter produces no frame at all where the supervisor does not
        // answer**, and says so on its complaints stream with a non-zero
        // ending. That is what this branch meets.
        return Ok(WhatOneAskingCameTo::NothingAnswered);
    }
    let Ok(answered) = read_one_result(&mut &said.output[..]) else {
        return Ok(WhatOneAskingCameTo::AFrameCameBack(None));
    };
    // **C2 matches a result to a request by identity**, so a result that
    // answers some other request is not this exchange's evidence.
    if !answered.answers(&request) {
        return Ok(WhatOneAskingCameTo::AFrameCameBack(None));
    }
    Ok(WhatOneAskingCameTo::AFrameCameBack(the_health_answer_in(
        &answered,
    )))
}

/// **The pause between two askings of the health exchange.** A Build decision
/// that bounds nothing — the deadline is the caller's — and only keeps the
/// asking from being a spin over the connection.
pub const BETWEEN_TWO_ASKINGS_OF_THE_HEALTH_ANSWER: std::time::Duration =
    std::time::Duration::from_millis(500);

/// **[`ask_for_the_health_answer`], asked again while nothing answers and the
/// caller's own deadline has not run out** — `Rented-server runtime` G25's
/// *"no answering supervisor inside the caller's own deadline"*.
///
/// **What it waits out is a real window rather than a guess.** A service
/// manager's start returns once it has started the program, not once the
/// program has bound its endpoint, and the adapter asks the endpoint once: an
/// asking that lands between the two is the supervisor not up yet, not the
/// supervisor failing. **Any frame that comes back ends it at once** — a right
/// answer, a wrong one, or one that does not read as an answer at all —
/// because G13 makes all but the first the failed exchange and asking again
/// would be asking for a different one. Only *nothing came back* is asked
/// again.
async fn ask_for_the_health_answer_until_it_is_answered<S>(
    surface: &mut ThePlacementsSurface<'_, S>,
    area: &str,
    identity: Identity,
    deadline: std::time::Duration,
) -> Result<Option<HealthAnswer>, SubsystemFailure>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    let started = std::time::Instant::now();
    loop {
        match asked_once(surface.subsystem, surface.connection, area, identity.clone()).await? {
            WhatOneAskingCameTo::AFrameCameBack(answer) => return Ok(answer),
            WhatOneAskingCameTo::NothingAnswered if started.elapsed() >= deadline => {
                return Ok(None)
            }
            WhatOneAskingCameTo::NothingAnswered => {
                tokio::time::sleep(BETWEEN_TWO_ASKINGS_OF_THE_HEALTH_ANSWER).await
            }
        }
    }
}

// ------------------------------------------------------ G25's ending, asked

/// **What asking the machine to end the running supervisor came to.**
///
/// **An entry point that gave no answer and an answer that says nothing was
/// ended are not the same thing**, and this keeps them apart: the first is
/// read as `Rented-server runtime` G25's own *a first placement has no
/// supervisor to end*, the second is a program that may go on serving after
/// the pointer has moved.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhetherTheRunningSupervisorEnded {
    /// **The entry point gave no answer at all** — the command could not be
    /// run, or it ended non-zero. It is read as nothing holding the
    /// machine-local endpoint, so the step is the start alone. *(Since
    /// 2026-09-28 this build's adapter answers even when nothing holds the
    /// endpoint, so this is now reached only where the entry point itself
    /// failed; the health exchange that follows still decides.)*
    ThereWasNoneToEnd,
    /// **The answer carries *a recorded outcome* or *still in hand*.** From
    /// this build's adapter that means nothing held the endpoint, the holder
    /// already runs the pointer's program, or the holder was sent the
    /// termination signal and let the endpoint go; the keeper already running
    /// then starts the one the pointer names. From an older release's
    /// adapter it means the supervisor recorded the ending and ends itself
    /// after answering.
    ItIsEnding,
    /// **An answer came back and the ending did not happen** — the holder
    /// could not be identified, the signal could not be sent, or the holder
    /// still held the endpoint when the machine stopped waiting; or, from an
    /// older release's adapter, the supervisor refused the request or its
    /// record store refused. **Nothing is softened here**: the start still
    /// runs, and the health exchange that follows reads whichever supervisor
    /// answers, so `Rented-server runtime` G13 is what decides the placement.
    ItWouldNot(String),
}

/// **`Rented-server runtime` G25's ending on a container, asked of the
/// machine's entry point over the same route the health exchange uses.**
/// *(Corrected 2026-09-30: this said the supervisor records the request and
/// ends itself. Since transfer wave piece 10, on 2026-09-28, the adapter of
/// the program the pointer names answers the request itself and ends the old
/// supervisor by signal.)*
///
/// **Why this exists.** G25 orders the running supervisor ended before the
/// restarter is asked. On a systemd machine the restarter's own `restart` is
/// both acts. **On a container nothing ended it**: `Rented-server runtime` S5's
/// lock means the keeper the start step launches ends at once, and the keeper
/// already running never restarts a supervisor that never ended — so the
/// program the *old* pointer started went on serving. Step 10c's third and
/// fourth passes on Vast both watched that, and on the fourth it failed a run.
///
/// **Why the desktop asks rather than signals.** §1.1 admits no machine-origin
/// value into a command string, and a signal wants the running supervisor's own
/// process number. **So the desktop sends the request, and the signal is sent
/// on the machine.** The pointer has already moved, so the entry point that
/// reads the request is the new program's. Its adapter asks no supervisor
/// anything: it reads from the kernel which process holds the machine-local
/// endpoint and which program file that process runs, and only where that is
/// not the file the pointer names does it send that process the termination
/// signal, then waits up to 20 seconds for the endpoint to be let go. The
/// keeper already running — which `Rented-server runtime` S5 says is the same
/// restarter reached by the other door — starts the one the pointer now names.
///
/// **C1: the identity is the initiating side's.** This build's adapter keeps
/// no record of it, so it serves correlation only. **A retried ending still
/// cannot take down the newly started program**, because by then the holder
/// runs the pointer's program and the comparison leaves it alone. Only an
/// older release's adapter passes the request on to a supervisor, which
/// records it before it acts (C4).
pub async fn ask_the_running_supervisor_to_end<S>(
    subsystem: &mut Subsystem<S>,
    connection: &mut Connection,
    package_area: &str,
    identity: Identity,
) -> WhetherTheRunningSupervisorEnded
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    let mut planted = match plant(
        subsystem,
        THE_ENTRY_POINTS_CARRIAGE_SITE,
        package_area.as_bytes(),
    )
    .await
    {
        Ok(planted) => planted,
        // **A carriage that could not be planted is a supervisor that cannot be
        // asked**, which is the same reading the health exchange takes of it.
        Err(why) => {
            return WhetherTheRunningSupervisorEnded::ItWouldNot(format!(
                "the entry point's carriage could not be planted: {why:?}"
            ))
        }
    };

    let request = Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        identity,
        THE_SUPERVISOR_ENDS_ITSELF_REQUEST,
    );
    let said = connection
        .run_one_command_carrying_a_framed_request_on_its_standard_input(
            THE_HEALTH_EXCHANGE_COMMAND,
            &write_request_frame(&request),
        )
        .await;
    take_it_away(subsystem, &mut planted).await;

    // **An entry point that produced no frame, or ended non-zero, is read as
    // G25's *a first placement has no supervisor to end*.** This build's
    // adapter answers the ending itself, nothing holding the endpoint
    // included, so this is now reached only where the entry point itself
    // failed; G13's exchange below still decides.
    let Ok(said) = said else {
        return WhetherTheRunningSupervisorEnded::ThereWasNoneToEnd;
    };
    if !said.ended_cleanly() {
        return WhetherTheRunningSupervisorEnded::ThereWasNoneToEnd;
    }
    let Ok(answered) = read_one_result(&mut &said.output[..]) else {
        return WhetherTheRunningSupervisorEnded::ItWouldNot(
            "the machine answered with something that does not read as a result".to_string(),
        );
    };
    // **C2 matches a result to a request by identity.**
    if !answered.answers(&request) {
        return WhetherTheRunningSupervisorEnded::ItWouldNot(
            "the machine answered some other request".to_string(),
        );
    }
    match answered.fields.text(asked::WHAT_BECAME_OF_IT_FIELD) {
        // **Taken as an ending.** This build's adapter sends *a recorded
        // outcome* when the endpoint is free for the pointer's program or
        // already held by it, although it records nothing; an older release's
        // supervisor sends either value once it has recorded the asking.
        Some(said)
            if said == asked::IT_HAS_A_RECORDED_OUTCOME || said == asked::IT_IS_STILL_IN_HAND =>
        {
            WhetherTheRunningSupervisorEnded::ItIsEnding
        }
        _ => WhetherTheRunningSupervisorEnded::ItWouldNot(
            answered
                .fields
                .text(asked::THE_MACHINES_OWN_WORDS_FIELD)
                .unwrap_or("the supervisor did not take the ending")
                .to_string(),
        ),
    }
}

// ------------------------------------------------------------- the whole walk

/// The two mechanisms §1.1 is made of, held together for the length of one
/// transaction.
///
/// **There is no third**, and a reader looking for the machine-local endpoint on
/// this route finds it named nowhere.
pub struct ThePlacementsSurface<'a, S> {
    /// §1.1's second mechanism — every record, every byte and the claim.
    pub subsystem: &'a mut Subsystem<S>,
    /// §1.1's first — the three user-level commands and the health exchange.
    pub connection: &'a mut Connection,
    /// **S20's scoping as it turned out over this transaction's own record
    /// writes**, weakened by any one of them that the server offered no flush
    /// for.
    ///
    /// **It is here rather than threaded through every call** because every
    /// record write of one transaction goes over one connection, and it is the
    /// connection's server that answers the question. **It is carried out to the
    /// caller in [`ThePlacementsOutcome`]** so that a machine that took the
    /// records without the ordering is one Smoozy knows about rather than one
    /// it merely served.
    the_durability_ordering: WhetherTheDurabilityOrderingApplied,
}

impl<'a, S> ThePlacementsSurface<'a, S> {
    /// The two mechanisms, held for one transaction.
    pub fn of(
        subsystem: &'a mut Subsystem<S>,
        connection: &'a mut Connection,
    ) -> ThePlacementsSurface<'a, S> {
        ThePlacementsSurface {
            subsystem,
            connection,
            the_durability_ordering:
                WhetherTheDurabilityOrderingApplied::TheStagedBytesWereMadeDurableFirst,
        }
    }

    /// **Weakened by any write that did not get the ordering, and never
    /// strengthened back.** One record written without it is what the caller
    /// has to know about.
    fn note(&mut self, this_write: WhetherTheDurabilityOrderingApplied) {
        if this_write
            == WhetherTheDurabilityOrderingApplied::TheServerOffersNoFlushSoTheOrderingDidNotApply
        {
            self.the_durability_ordering = this_write;
        }
    }
}

/// What one placement came to: `Rented-server runtime` G3's phase in the
/// caller's terms, and whether S20's ordering held for the records it wrote.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThePlacementsOutcome {
    pub reached: WhatThePlacementReached,
    /// **S20 is scoped *"where the server offers it"***, so a machine whose file
    /// server announces no flush is served without the ordering and this says
    /// so. §12 item 7 is what holds without it.
    pub the_durability_ordering: WhetherTheDurabilityOrderingApplied,
}

/// What one placement is handed, and every field is a value somebody else
/// decided.
///
/// **Nothing here is read off the machine by this module.** The machine kind is
/// `Rented-server runtime` K2's, re-taken at every placement by `Server Setup`;
/// the tuple selection is B8's and happened before the request was formed; the
/// previous version is `Rented-server runtime` G4's carried-forward value.
pub struct WhatThePlacementNeeds<'a> {
    /// Where `Job files and outputs` L4a put the package area.
    pub package_area: String,
    /// B1's idempotency key, which is the operation identity.
    pub attempt: Identity,
    /// `Rented-server runtime` G2's owner fields, written only while the claim
    /// is held.
    pub owner: TransactionOwner,
    /// `Rented-server runtime` G10's installation identity.
    pub installation: Identity,
    /// B10's verified release, which is the only thing that can carry a
    /// descriptor here.
    pub verified: &'a AVerifiedRelease,
    /// The artifact's own bytes, already verified against the manifest.
    pub artifact: &'a [u8],
    /// `Rented-server runtime` K2's detected kind.
    pub kind: MachineKind,
    // **Where this attempt's material is staged is not handed in.** S4 makes
    // that path this document's to synthesise, from the attempt's identity, and
    // [`crate::placement::the_staging_location_for`] is the one composition.
    // **It was a caller's value until step 10c piece 12**, and the one caller
    // composed it as the pointer's own staging name — so the pointer's switch
    // deleted the program it was switching to, on every machine.
    /// `Rented-server runtime` G4's carried-forward previous version, where
    /// there is one.
    pub previous: Option<PreviousVersion>,
    /// **`Rented-server runtime` K11's and K19's root step on a container**,
    /// with `Server Setup`'s own commands in it. [`None`] on a systemd machine,
    /// where no step of the registration is elevated.
    ///
    /// ~~The hook's content on a container, composed by the step that installs
    /// it, and the provider's own hook path.~~ **Replaced 2026-09-21, step 10c:
    /// the one caller handed in no content and an empty path, so every
    /// container placement failed at K15 reading back a path that was not
    /// there.** The content is now composed inside the walk, from the machine's
    /// own login name and whatever the provider's file already holds.
    pub the_root_step_on_a_container: Option<TheRootStepOnAContainer>,
    /// What `Rented-server runtime` G21 reads: every operation accepted under
    /// either retained protocol version, with its standing.
    pub accepted_work: Vec<(String, AcceptedWorkStanding)>,
    /// **The health exchange's own correlation identity, chosen before the
    /// request is sent** — C1, which puts it on the initiating side, and this
    /// walk is the initiating side. **It is not the attempt's**: the attempt's
    /// identity is B1's idempotency key and a reading is not a second attempt at
    /// anything (C6).
    pub the_health_exchanges_identity: Identity,
    /// **The waiter's own deadline for the health answer** — `Server protocol
    /// and transfers` C16, and `Rented-server runtime` G25's *"Where the
    /// restarter produces no answering supervisor inside the caller's own
    /// deadline, that is G13's did not answer branch"*.
    ///
    /// **The exchange is asked again until it is answered or this runs out**,
    /// because a service manager's start returns before the program it
    /// started has bound its endpoint, and one asking in that window is a
    /// supervisor that is merely not up yet. **An answer is never asked for a
    /// second time**: a wrong one is G13's failed exchange at once. **Nought
    /// asks once.** K17 is kept: this is the caller's own figure, not a wait
    /// budgeted for a machine.
    pub how_long_the_health_answer_is_waited_for: std::time::Duration,
    /// **The waiter's own deadline for the user's service manager to answer,
    /// after lingering is granted and before the unit is enabled** —
    /// `Rented-server runtime` K7 as corrected on 2026-09-21, and
    /// `Server protocol and transfers` C16's per-waiter deadline.
    ///
    /// **The grant starts the manager and does not wait for it**, so on a
    /// machine whose login opened no session the enable is not sent until the
    /// manager answers or this runs out. **Where a session exists the manager
    /// answers the first time and nothing is waited for.** Nought asks once.
    /// K17 is kept: this is the caller's figure, not a wait budgeted for a
    /// machine.
    pub how_long_the_user_manager_is_waited_for: std::time::Duration,
}

/// What a placement reached, and it is `Rented-server runtime` G3's phase said
/// in the caller's terms.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatThePlacementReached {
    /// The transaction succeeded.
    Verified,
    /// The machine is on the previous package.
    RolledBack,
    /// The machine is on the previous package and staged material was swept.
    FailedBeforeCommit { because: String },
    /// **`Rented-server runtime` G22: a blocked commit is not a failure.** The
    /// staged material is kept.
    Deferred { because: String },
    /// **`Rented-server runtime` G16: a rollback that cannot complete is not a
    /// silent one.** The next request under any identity completes it before
    /// considering anything else.
    RollingBack { because: String },
    /// A claim stands and it is not this installation's. **Nothing was
    /// changed** — no phase, no pointer, no staging path, no health result.
    APackageTransactionIsInProgress {
        owner: TransactionOwner,
        phase: Phase,
    },
    /// **`Rented-server runtime` G1a's window: a claim stands and its owner
    /// could not be read.** The claim's create and the write of its owner
    /// fields are two operations, and a claimant can meet the gap between them.
    /// **Nothing was changed** — no phase, no pointer, no staging path, no
    /// health result — and **this is not the machine's file server failing**,
    /// which is what it was reported as until a code round said otherwise.
    AClaimStandsWhoseOwnerCouldNotBeRead,
}

/// What a phase reached means to a caller.
pub fn what_the_placement_reached(record: &PhaseRecord) -> WhatThePlacementReached {
    match record.phase {
        Phase::Verified => WhatThePlacementReached::Verified,
        Phase::RolledBack => WhatThePlacementReached::RolledBack,
        Phase::RollingBack => WhatThePlacementReached::RollingBack {
            because: "the rollback did not complete".to_string(),
        },
        Phase::Deferred => WhatThePlacementReached::Deferred {
            because: record
                .deferred_because
                .clone()
                .unwrap_or_else(|| "live or held work depends on the runtime".to_string()),
        },
        other => WhatThePlacementReached::FailedBeforeCommit {
            because: format!("the transaction stopped at {}", other.what_a_later_reading_means()),
        },
    }
}

/// Why the surface itself would not carry the transaction.
///
/// **This is not a refusal of the transaction** — those are
/// [`WhatThePlacementReached`]'s. It is the connection or the machine's own
/// file-transfer server failing under it, which nothing here can act on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhyThePlacementCouldNotBeCarriedOut {
    /// The subsystem refused an operation, with the protocol's own status.
    TheSubsystem(SubsystemFailure),
    /// The machine would not answer a user-level command, in its own words.
    TheMachineWouldNot(String),
    /// A value would not go into a registration.
    ThatValueDoesNotGoIntoARegistration(String),
}

impl From<SubsystemFailure> for WhyThePlacementCouldNotBeCarriedOut {
    fn from(why: SubsystemFailure) -> Self {
        WhyThePlacementCouldNotBeCarriedOut::TheSubsystem(why)
    }
}

/// **The phase record's own descriptor, from the one a verified release
/// carries.**
///
/// **The two types are deliberately not one.** The machine side's is
/// `Rented-server runtime` G27's, pinned in a record that crate owns and reads
/// back; the desktop's is what B1 carries on the wire. **G26 says outright that
/// this transaction consumes rather than redefines the release evidence**, so
/// what crosses here is the seven values and nothing composed from them.
pub fn the_phase_records_descriptor_for(
    descriptor: &TheDescriptorsValues,
) -> VerifiedReleaseDescriptor {
    VerifiedReleaseDescriptor {
        manifest_format: descriptor.manifest_format.clone(),
        verified_release_sequence: descriptor.verified_release_sequence,
        requested_package_version: descriptor.requested_package_version.clone(),
        manifest_digest: descriptor.manifest_digest,
        artifact_identity: descriptor.artifact_identity.clone(),
        artifact_byte_length: descriptor.artifact_byte_length,
        artifact_digest: descriptor.artifact_digest,
    }
}

/// **The whole transaction, performed through the surface — the caller
/// `accept_a_release` never had.**
///
/// The order is [`crate::placement::AnActOfThePlacement::IN_ORDER`] and every
/// phase is written before the act it names, which is `Rented-server runtime`
/// G3.
pub async fn place_a_verified_release<S>(
    surface: &mut ThePlacementsSurface<'_, S>,
    needs: WhatThePlacementNeeds<'_>,
) -> Result<ThePlacementsOutcome, WhyThePlacementCouldNotBeCarriedOut>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    let reached = the_transaction(surface, needs).await?;
    Ok(ThePlacementsOutcome {
        reached,
        the_durability_ordering: surface.the_durability_ordering,
    })
}

/// The transaction itself. **What it reaches is the phase; whether S20's
/// ordering held is the surface's**, and the caller above puts the two
/// together.
async fn the_transaction<S>(
    surface: &mut ThePlacementsSurface<'_, S>,
    needs: WhatThePlacementNeeds<'_>,
) -> Result<WhatThePlacementReached, WhyThePlacementCouldNotBeCarriedOut>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    let area = needs.package_area.as_str();
    let descriptor = the_phase_records_descriptor_for(&needs.verified.the_release_descriptors_values());

    // **`Rented-server runtime` G26, before anything is staged.** A request that
    // does not carry a verified release descriptor reaches *failed before
    // commit* without any downloaded byte being made executable or invoked. It
    // cannot fail here — the type this function takes has one producer, which is
    // the verification — and the check is made all the same, because the clause
    // is about what may proceed rather than about what happens to be reachable.
    if may_proceed(Some(&descriptor)) != WhetherTheRequestMayProceed::ItCarriesAVerifiedDescriptor {
        return Ok(WhatThePlacementReached::FailedBeforeCommit {
            because: "the request carries no verified release descriptor".to_string(),
        });
    }

    // ------------------------------------------ act one: the claim (G1b)
    let (taken, ordering) = take_the_claim(
        surface.subsystem,
        area,
        &needs.owner,
        Phase::Inspecting,
    )
    .await?;
    surface.note(ordering);
    let the_claim = match taken {
        // **G1a's window, and the caller is told what actually stands** rather
        // than that the machine's file server failed under the transaction.
        WhatTakingTheClaimReached::AClaimStandsWhoseOwnerCouldNotBeRead => {
            return Ok(WhatThePlacementReached::AClaimStandsWhoseOwnerCouldNotBeRead)
        }
        WhatTakingTheClaimReached::TheRulesHalfSays(ClaimOutcome::InProgress { owner, phase }) => {
            return Ok(WhatThePlacementReached::APackageTransactionIsInProgress { owner, phase })
        }
        WhatTakingTheClaimReached::TheRulesHalfSays(held) => held,
    };

    // **Every way the walk below can end early leaves no claim standing that
    // names an owner already gone.** Added 2026-09-21, step 10c: the second
    // pass on Sesterce met a registration failure that returned straight out of
    // the walk, and every later setup on that machine was told *a package
    // transaction is in progress* — through a restart of Smoozy, for good. **The
    // walk's own endings each take `Rented-server runtime` G3's terminal write
    // and G1b's release**; what this catches is the rest — the connection or the
    // machine's file server failing under the transaction, and a value that
    // would not go into a registration — which return as errors and used to
    // leave the claim behind.
    match the_walk_under_the_claim(surface, area, &descriptor, &the_claim, &needs).await {
        Ok(reached) => Ok(reached),
        Err(why) => {
            the_claim_is_not_left_standing_after(surface, area).await;
            Err(why)
        }
    }
}

/// **Everything after G1b's claim is held, and nothing before it.**
async fn the_walk_under_the_claim<S>(
    surface: &mut ThePlacementsSurface<'_, S>,
    area: &str,
    descriptor: &VerifiedReleaseDescriptor,
    the_claim: &ClaimOutcome,
    needs: &WhatThePlacementNeeds<'_>,
) -> Result<WhatThePlacementReached, WhyThePlacementCouldNotBeCarriedOut>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    // **`Rented-server runtime` G17 and G18, before a fresh attempt is
    // written.** Which of the two governs is decided by the recorded attempt
    // identity and by nothing else, and **each of G17's ten rows has an arm of
    // its own** — a fall-through arm here wrote a fresh attempt over six of
    // them, resetting the phase, replacing the carried-forward previous value,
    // sweeping the staging and staging the whole artifact again.
    if let WhatARecordHeld::It(standing) = read_the_phase_record(surface.subsystem, area).await? {
        if standing.attempt == needs.attempt {
            if let Some(reached) =
                what_this_attempts_own_retry_does(surface, area, standing, needs).await?
            {
                return Ok(reached);
            }
        } else if let Some(reached) =
            what_another_identity_does(surface, area, standing, the_claim, needs).await?
        {
            return Ok(reached);
        }
    }

    // ------------------- act two: G4's one write, carrying everything together
    let mut record = a_fresh_attempts_first_write(
        needs.attempt.clone(),
        (*descriptor).clone(),
        needs.previous.clone(),
        needs.kind,
        crate::placement::the_staging_location_for(area, &needs.attempt),
    );
    record.owner = Some(needs.owner.clone());
    let ordering = write_the_phase_record(surface.subsystem, area, &record).await?;
    surface.note(ordering);

    // **`Rented-server runtime` G5's other removal**: a fresh replacement that
    // begins with a predecessor still present removes it as its first act. That
    // is not this attempt's — the predecessor is what the machine is running —
    // so what is removed here is only what an abandoned attempt left at this
    // attempt's own staging location, which is `Rented-server runtime` G20's
    // bounded sweep.
    sweep_the_abandoned_staging(surface.subsystem, &record).await?;

    from_the_staging(surface, area, &mut record, needs, None).await
}

/// **What a walk that ended in an error leaves, made safe as far as the
/// connection still allows** — and the error the caller is told stays the one
/// that ended the walk.
///
/// The phase record is read again rather than trusted from memory, because the
/// error can come from any act, and **what is done is `Rented-server runtime`
/// G3's own ending for whatever phase stands**:
///
/// - *inspecting*, *staging*, *staged and validated* — **G20's sweep and the
///   *failed before commit* terminal write**, which releases the claim;
/// - *committing*, *committed, unverified*, *rolling back* — **G14's restore,
///   completed** (G16); a restore that cannot complete keeps the claim, which
///   is G16's own rule, and the next request under this installation takes it
///   back (G1b);
/// - a terminal phase, or no record at all — **G1b's release, alone**, the
///   transaction having nothing left to end.
///
/// **Every act here is best effort, and none is reported.** Where the
/// connection is what failed, nothing here reaches the machine either, and
/// that is the case this cannot help: `Rented-server runtime` G1b's
/// own-installation take-back is what answers it on the next setup.
async fn the_claim_is_not_left_standing_after<S>(
    surface: &mut ThePlacementsSurface<'_, S>,
    area: &str,
) where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    let standing = match read_the_phase_record(surface.subsystem, area).await {
        Ok(WhatARecordHeld::It(standing)) => standing,
        Ok(_) => {
            let _ = release_the_claim_at_the_terminal_write(surface.subsystem, area).await;
            return;
        }
        Err(_) => return,
    };
    match standing.phase {
        Phase::Inspecting | Phase::Staging | Phase::StagedAndValidated => {
            let mut standing = standing;
            let _ = the_attempt_failed_before_commit(
                surface,
                area,
                &mut standing,
                "the transaction was cut off part-way",
            )
            .await;
        }
        Phase::Committing | Phase::CommittedUnverified | Phase::RollingBack => {
            let _ = complete_the_rollback(surface, area, &standing).await;
        }
        Phase::Verified | Phase::RolledBack | Phase::FailedBeforeCommit | Phase::Deferred => {
            let _ = release_the_claim_at_the_terminal_write(surface.subsystem, area).await;
        }
    }
}

/// **`Rented-server runtime` G17's table, every row of it, with no catch-all
/// arm — so the compiler sends the next person who adds a row to this site.**
///
/// **Nothing means the table says start a fresh replacement**, which is what the
/// caller does next; every other row ends here.
///
/// **What the fall-through used to do instead**, on six of the ten rows: it
/// wrote a fresh attempt's first record over the standing one, resetting the
/// phase to *inspecting*, clearing the health exchange and the deferred reason,
/// replacing G4's carried-forward previous value with whatever the caller
/// handed in, sweeping the staging location — **which at *committed, unverified*
/// is the file the active pointer names** — and staging the whole artifact from
/// zero.
async fn what_this_attempts_own_retry_does<S>(
    surface: &mut ThePlacementsSurface<'_, S>,
    area: &str,
    standing: PhaseRecord,
    needs: &WhatThePlacementNeeds<'_>,
) -> Result<Option<WhatThePlacementReached>, WhyThePlacementCouldNotBeCarriedOut>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    let pointer = what_the_pointer_names(surface.subsystem, area, &standing).await?;
    // **The claim now names this run**, and G1a writes the owner only while the
    // claim is held — so a continuation carries the owner forward rather than
    // leaving the record naming a run that has ended. **This is the same
    // attempt identity throughout**, which is what separates it from G18's
    // prohibition on replacing an owner.
    let mut standing = standing;
    standing.owner = Some(needs.owner.clone());

    match what_a_same_identity_retry_does(standing.phase, Some(pointer)) {
        // *inspecting*, *staging* — **the staged material is re-validated by
        // content, the parts that match are kept and the rest is re-staged**
        // (G19). What is staged here is one artifact, so *the parts that match*
        // is all of it or none of it, and the reading that decides is B10's
        // at-rest read-back rather than the file being there.
        SameIdentityRetry::ContinueRevalidatingStagedMaterial => {
            let reusable = what_the_staged_material_may_be_reused_as(surface, &standing).await?;
            if reusable.is_none() {
                sweep_the_abandoned_staging(surface.subsystem, &standing).await?;
            }
            Ok(Some(
                from_the_staging(surface, area, &mut standing, needs, reusable).await?,
            ))
        }
        // *staged and validated* — **continues from the registration step**, and
        // stages nothing again.
        SameIdentityRetry::ContinueFromTheRegistrationStep => Ok(Some(
            from_the_registration_step(surface, area, &mut standing, needs).await?,
        )),
        // *committing* with the pointer naming the requested version, *committing*
        // with it naming the previous one, and *committed, unverified* — **all
        // three enter G25's ordered steps on the standing record**, and none of
        // them re-stages.
        SameIdentityRetry::ProceedToActivationAndTheHealthExchange
        | SameIdentityRetry::ReEnterCommitting
        | SameIdentityRetry::CompleteTheActivationAndTakeG14OrG15 => Ok(Some(
            continue_the_standing_commit(surface, area, &mut standing, needs).await?,
        )),
        // *committing* with the pointer naming neither — **the interrupted-rename
        // state**, where the pointer is restored from G4's recorded previous
        // value under the *rolling back* phase, exactly as G14 restores it after
        // a failed exchange.
        SameIdentityRetry::RestoreUnderRollingBack => Ok(Some(
            complete_the_rollback(surface, area, &standing).await?,
        )),
        // *rolling back* — G16.
        SameIdentityRetry::CompleteTheRollback => Ok(Some(
            complete_the_rollback(surface, area, &standing).await?,
        )),
        // *verified* — **acts again on nothing and returns the recorded
        // outcome** (`Server protocol and transfers` C4). **The claim is
        // released**: this ending is a terminal one and G1b removes the claim at
        // a terminal write, so a retry that took the claim and answered from the
        // record must give it back. Nothing else in Smoozy ever would.
        SameIdentityRetry::ActAgainOnNothingAndReturnTheRecordedOutcome => {
            release_the_claim_at_the_terminal_write(surface.subsystem, area).await?;
            Ok(Some(what_the_placement_reached(&standing)))
        }
        // *deferred* — re-evaluates §2.6 and either proceeds or answers deferred
        // again. **The claim is released on the deferred answer** for the same
        // reason as above; G22 makes *deferred* a terminal write and the main
        // path already releases it there.
        SameIdentityRetry::ReEvaluateWhatBlocksTheCommit => {
            match whether_the_commit_may_proceed(&needs.accepted_work) {
                WhetherTheCommitMayProceed::ItMayProceed => Ok(Some(
                    continue_the_standing_commit(surface, area, &mut standing, needs).await?,
                )),
                WhetherTheCommitMayProceed::Deferred { because } => {
                    standing.deferred_because = Some(because.clone());
                    let ordering =
                        write_the_phase_record(surface.subsystem, area, &standing).await?;
                    surface.note(ordering);
                    release_the_claim_at_the_terminal_write(surface.subsystem, area).await?;
                    Ok(Some(WhatThePlacementReached::Deferred { because }))
                }
            }
        }
        // *rolled back*, *failed before commit* — **starts a fresh replacement,
        // under the same identity, after sweeping the abandoned staging of the
        // failed attempt**, which is what act two does next.
        SameIdentityRetry::FreshReplacementAfterSweepingTheAbandonedStaging => Ok(None),
    }
}

/// **`Rented-server runtime` G18, asked rather than assumed.**
///
/// **The function that answers it existed and had no caller outside its own
/// tests.** What stood here was one branch of the rule — a record at *rolling
/// back* — and a fall-through that wrote a fresh attempt over every other
/// phase, under an identity that had no licence to touch it.
///
/// **The sharpest of those is the *deferred* carve-out.** A render is running,
/// the commit was deferred and the staged package is sitting on the machine;
/// automatic repair fires with its own identity, or the user presses the
/// fallback re-run. G18 answers *with the deferred phase, and acts on nothing* —
/// and the fresh attempt would have swept the staging location, which is the
/// very material G22 says a deferral keeps.
///
/// **Nothing means the recorded attempt is already at a terminal phase**, so a
/// fresh replacement may begin, which is what the caller does next.
async fn what_another_identity_does<S>(
    surface: &mut ThePlacementsSurface<'_, S>,
    area: &str,
    standing: PhaseRecord,
    the_claim: &ClaimOutcome,
    needs: &WhatThePlacementNeeds<'_>,
) -> Result<Option<WhatThePlacementReached>, WhyThePlacementCouldNotBeCarriedOut>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    match what_a_different_identity_may_do(the_claim, standing.phase) {
        // **It could not acquire the claim.** Act one returns on that outcome
        // before this is reached, so nothing here holds a claim to give back —
        // the arm is written out rather than defaulted because the answer set is
        // the rules half's and not this file's to narrow.
        DifferentIdentity::ReceivesTheInProgressResultAndActsOnNothing => {
            Ok(Some(WhatThePlacementReached::APackageTransactionIsInProgress {
                owner: standing.owner.clone().unwrap_or_else(|| needs.owner.clone()),
                phase: standing.phase,
            }))
        }
        // **The one carve-out.** B6 and G22 are exact that a deferred result
        // *"does not license a second attempt under a fresh identity"*, so
        // nothing on the machine is touched and the staged material stays. **The
        // claim this request itself created is given back**, which is not acting
        // on the transaction and is what stops it standing for ever.
        DifferentIdentity::AnsweredWithTheDeferredPhaseAndActsOnNothing => {
            release_the_claim_at_the_terminal_write(surface.subsystem, area).await?;
            Ok(Some(what_the_placement_reached(&standing)))
        }
        // **The recorded attempt is driven to a terminal phase under its
        // recorded identity** — the owner and attempt fields are not replaced
        // until that terminal write, which is what the three drives below do.
        // **The fresh replacement is the next request's**, which G17's *rolled
        // back* and *failed before commit* row then admits.
        DifferentIdentity::DrivesTheRecordedAttemptToATerminalPhaseFirst => {
            let mut standing = standing;
            match standing.phase {
                // Already terminal: there is nothing to drive, so a fresh
                // replacement may begin.
                Phase::Verified | Phase::RolledBack | Phase::FailedBeforeCommit => Ok(None),
                // **Completing a rollback (G16).**
                Phase::RollingBack => {
                    Ok(Some(complete_the_rollback(surface, area, &standing).await?))
                }
                // **Completing a commit**, on the standing record and under its
                // own recorded attempt identity.
                Phase::Committing | Phase::CommittedUnverified => Ok(Some(
                    continue_the_standing_commit(surface, area, &mut standing, needs).await?,
                )),
                // **Sweeping failed staging (G20)**, which is the terminal write
                // for an attempt that never reached the commit — **and then this
                // request's own fresh replacement, under the claim it still
                // holds.** G18: *"Only after that terminal write may it replace
                // the owner and attempt fields and begin a fresh replacement."*
                // ~~The claim was released here and the request ended *failed
                // before commit*, leaving the replacement to the next press.~~
                // **Changed 2026-09-21, step 10c**: the user pressed *Set up*
                // once and was told it failed for a reason that was not theirs,
                // and G18 already permits going on.
                Phase::Inspecting | Phase::Staging | Phase::StagedAndValidated => {
                    standing.phase = Phase::FailedBeforeCommit;
                    let ordering = write_the_phase_record(surface.subsystem, area, &standing).await?;
                    surface.note(ordering);
                    sweep_the_abandoned_staging(surface.subsystem, &standing).await?;
                    Ok(None)
                }
                // The rules half answers the deferred carve-out above, so this
                // phase cannot reach here.
                Phase::Deferred => Ok(Some(what_the_placement_reached(&standing))),
            }
        }
    }
}

/// **G19's reading: whether the staged material a retry finds may be reused.**
///
/// *"A retry that finds staged material at the location the phase record names
/// re-verifies G27's recorded release sequence, manifest digest, artifact
/// identity, byte length and artifact digest through B10's at-rest read-back
/// comparison before reusing or invoking a byte."*
///
/// **The reading is handed back rather than thrown away**, so the caller that
/// reuses the material does not read hundreds of megabytes off the machine a
/// second time to compare the same values.
///
/// **Nothing readable at the staging location is nothing to reuse**, and the
/// answer is *no* rather than a failure: the material is re-staged, which is
/// what the row says happens to the parts that do not match.
async fn what_the_staged_material_may_be_reused_as<S>(
    surface: &mut ThePlacementsSurface<'_, S>,
    standing: &PhaseRecord,
) -> Result<Option<TheReadingAtRest>, WhyThePlacementCouldNotBeCarriedOut>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    let recorded = &standing.requested;
    let chunk = surface.subsystem_read_chunk();
    let found = read_back_at_rest(
        surface.subsystem,
        standing.staging_location.as_bytes(),
        recorded.artifact_byte_length,
        chunk,
    )
    .await;
    let Ok(WhatTheReadBackFound::ItRead(reading)) = found else {
        return Ok(None);
    };

    // **The three values the staged bytes cannot answer for are the record's
    // own** — a release sequence, a manifest digest and an artifact identity are
    // not properties of a run of bytes — so what this comparison actually
    // decides is the byte length and the digest, which is exactly what B10's
    // at-rest read-back is.
    let measured = VerifiedReleaseDescriptor {
        manifest_format: recorded.manifest_format.clone(),
        verified_release_sequence: recorded.verified_release_sequence,
        requested_package_version: recorded.requested_package_version.clone(),
        manifest_digest: recorded.manifest_digest,
        artifact_identity: recorded.artifact_identity.clone(),
        artifact_byte_length: reading.byte_length,
        artifact_digest: reading.digest,
    };
    let at_rest = compare_the_recorded_values_at_rest(
        recorded,
        AtRestMeasurement {
            byte_length: reading.byte_length,
            digest: reading.digest,
        },
    );
    Ok(staged_material_may_be_reused(recorded, &measured, at_rest).then_some(reading))
}

/// **Act three and act four: the staging, then G29's at-rest comparison and the
/// mode it earns.**
///
/// **Where the material is already staged and has been re-validated, the
/// reading is handed in and nothing is uploaded again** — which is G17's
/// *the parts that match are kept*.
async fn from_the_staging<S>(
    surface: &mut ThePlacementsSurface<'_, S>,
    area: &str,
    record: &mut PhaseRecord,
    needs: &WhatThePlacementNeeds<'_>,
    already_staged: Option<TheReadingAtRest>,
) -> Result<WhatThePlacementReached, WhyThePlacementCouldNotBeCarriedOut>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    let staged_path = record.staging_location.as_bytes().to_vec();

    let reading = match already_staged {
        Some(reading) => reading,
        None => {
            // ------------------------------------------ act three: the staging
            record.phase = Phase::Staging;
            let ordering = write_the_phase_record(surface.subsystem, area, record).await?;
            surface.note(ordering);
            stage_the_artifact(surface.subsystem, &record.staging_location, needs.artifact).await?;

            let chunk = surface.subsystem_read_chunk();
            let found = read_back_at_rest(
                surface.subsystem,
                &staged_path,
                record.requested.artifact_byte_length,
                chunk,
            )
            .await?;
            let WhatTheReadBackFound::ItRead(reading) = found else {
                return the_attempt_failed_before_commit(
                    surface,
                    area,
                    record,
                    "the staged artifact reads as more bytes than the manifest names",
                )
                .await;
            };
            reading
        }
    };

    // -------------- act four: G29's at-rest comparison, then the mode it earns
    let wire_descriptor = needs.verified.the_release_descriptors_values();
    let Some(agreed) = compare_at_rest(&wire_descriptor, &staged_path, reading) else {
        return the_attempt_failed_before_commit(
            surface,
            area,
            record,
            "the staged artifact is not the one the verified manifest names",
        )
        .await;
    };
    // **A mode the machine would not set is a failure before the commit**, and
    // takes the rollback rather than leaving the claim standing. Changed
    // 2026-09-21, step 10c.
    if let Err(why) = make_the_artifact_executable(
        surface.subsystem,
        &staged_path,
        &wire_descriptor.artifact_digest,
        agreed,
    )
    .await
    {
        return the_attempt_failed_before_commit(
            surface,
            area,
            record,
            &format!(
                "the staged artifact could not be made runnable: {}",
                match &why {
                    crate::release::verifying::WhyTheArtifactWasNotMadeExecutable::ThatAgreementWasNotAboutThisArtifact =>
                        "the comparison that allowed it was about another file".to_string(),
                    crate::release::verifying::WhyTheArtifactWasNotMadeExecutable::TheSubsystemRefusedIt(failure) =>
                        failure.in_words(),
                }
            ),
        )
        .await;
    }

    // **`Rented-server runtime` G8, asked before the phase is written.** A
    // package version that cannot be started through the fixed path the
    // registration names is refused at *staged and validated*, and the machine
    // is untouched. What answers it here is the pointer's own name: the
    // registration starts whatever the pointer names, so what has to be true is
    // that the staged artifact is what the pointer will come to name.
    if may_be_committed(!record.staging_location.is_empty())
        != WhetherTheStagedVersionMayBeCommitted::ItStartsThroughTheFixedPath
    {
        return the_attempt_failed_before_commit(
            surface,
            area,
            record,
            "the staged version cannot be started through the fixed path the registration names",
        )
        .await;
    }

    record.phase = Phase::StagedAndValidated;
    let ordering = write_the_phase_record(surface.subsystem, area, record).await?;
    surface.note(ordering);

    from_the_registration_step(surface, area, record, needs).await
}

/// **The pause between two askings of the user's service manager.** A Build
/// decision that bounds nothing — the deadline is the caller's.
pub const BETWEEN_TWO_ASKINGS_OF_THE_USER_MANAGER: std::time::Duration =
    std::time::Duration::from_millis(500);

/// **`Rented-server runtime` K4's two acts on a systemd machine, in the order
/// K7 as corrected on 2026-09-21 gives them.**
///
/// 1. **K6's grant**, by the machine's own name for the login;
/// 2. **K5's read-back**, which is what separates a registration from a report
///    of one — a read-back not showing lingering granted fails;
/// 3. **the user's service manager, asked until it answers inside the caller's
///    own deadline** — the grant starts it and does not wait for it, and on a
///    login with no session nothing else starts it;
/// 4. **K4's enable.**
///
/// **Until 2026-09-21 the enable came first**, which worked only where the
/// login had opened a session: on Sesterce, whose image sets `UsePAM no`, it
/// failed with *"Failed to connect to bus: No such file or directory"*.
///
/// **What comes back on a refusal is the machine's own words**, whole — §1.3
/// forbids mining them for a cause.
async fn the_user_level_registration<S>(
    surface: &mut ThePlacementsSurface<'_, S>,
    deadline: std::time::Duration,
) -> Result<(), String>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    grant_lingering(surface.connection).await?;
    let read_back = read_the_linger_flag_back(surface.connection).await?;
    if !read_back.lingering_is_granted {
        return Err(
            "the login manager did not grant lingering, so the supervisor would not come back"
                .to_string(),
        );
    }
    let asked_since = std::time::Instant::now();
    loop {
        match the_user_manager_answers(surface.connection).await {
            Ok(()) => break,
            Err(said) if asked_since.elapsed() >= deadline => {
                return Err(format!(
                    "{said} (asked for {} seconds after lingering was granted)",
                    asked_since.elapsed().as_secs()
                ))
            }
            Err(_) => tokio::time::sleep(BETWEEN_TWO_ASKINGS_OF_THE_USER_MANAGER).await,
        }
    }
    enable_the_unit(surface.connection).await
}

/// **Act five and act six: the registration before *committing*, and then the
/// commit.**
async fn from_the_registration_step<S>(
    surface: &mut ThePlacementsSurface<'_, S>,
    area: &str,
    record: &mut PhaseRecord,
    needs: &WhatThePlacementNeeds<'_>,
) -> Result<WhatThePlacementReached, WhyThePlacementCouldNotBeCarriedOut>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    let standing_record = match read_the_boot_registration_record(surface.subsystem, area).await? {
        WhatARecordHeld::It(standing) => Some(standing),
        _ => None,
    };
    // **On a container the hook's content is read and composed first**:
    // whatever the provider's own file holds is kept, and the machine names its
    // own login for the hook to drop privilege to. **A container with no root
    // step handed in, or one whose hook cannot be read, fails before the
    // commit** rather than writing a record naming nothing.
    let the_hook = match (record.machine_kind, needs.the_root_step_on_a_container.as_ref()) {
        (MachineKind::Container, Some(step)) => {
            match the_container_hooks_content(surface.subsystem, surface.connection, area, step)
                .await
            {
                Ok(content) => Some((content, step.the_hooks_path.clone())),
                Err(said) => {
                    return the_attempt_failed_before_commit(
                        surface,
                        area,
                        record,
                        &format!("the boot hook could not be composed: {said}"),
                    )
                    .await
                }
            }
        }
        (MachineKind::Container, None) => {
            return the_attempt_failed_before_commit(
                surface,
                area,
                record,
                "this placement was handed no way to install the container's boot hook",
            )
            .await
        }
        (MachineKind::Systemd, _) => None,
    };
    let this_ones_record = crate::placement::registration::the_registration_this_placement_would_make(
        record.machine_kind,
        area,
        the_hook.as_ref().map(|(content, _)| content.as_slice()),
        the_hook.as_ref().map(|(_, at)| at.as_str()).unwrap_or_default(),
    )
    .map_err(|why| {
        WhyThePlacementCouldNotBeCarriedOut::ThatValueDoesNotGoIntoARegistration(why.as_text())
    });
    // **A value that will not go into a registration is a failure before the
    // commit like any other**, and takes the rollback rather than leaving the
    // claim standing. Changed 2026-09-21, step 10c.
    let this_ones_record = match this_ones_record {
        Ok(it) => it,
        Err(why) => {
            let said = match &why {
                WhyThePlacementCouldNotBeCarriedOut::ThatValueDoesNotGoIntoARegistration(said) => {
                    said.clone()
                }
                _ => String::new(),
            };
            return the_attempt_failed_before_commit(
                surface,
                area,
                record,
                &format!("a value would not go into the boot registration: {said}"),
            )
            .await;
        }
    };

    // **`Rented-server runtime` G9**, asked of content and answered before the
    // registration is written, because it is what decides whether the rollback
    // has a registration to restore at all.
    let registration_changes = whether_the_registration_changes(
        standing_record
            .as_ref()
            .map(|standing| standing.content.as_slice())
            .unwrap_or_default(),
        &this_ones_record.content,
    );

    let (left, ordering) = place_the_registration(surface.subsystem, area, &this_ones_record).await?;
    surface.note(ordering);
    // **`Rented-server runtime` K11 and K19, after K16b's record and before
    // K15's read-back.** The copy's own exit is not the evidence; the read-back
    // below is.
    if let (WhatTheRegistrationStepLeft::TheRecordAndTheHookLeftToTheRootStep, Some(step)) =
        (left, needs.the_root_step_on_a_container.as_ref())
    {
        if let Err(said) =
            install_the_hook(surface.subsystem, surface.connection, &this_ones_record, step).await
        {
            return the_attempt_failed_before_commit(
                surface,
                area,
                record,
                &format!("the boot hook could not be installed: {said}"),
            )
            .await;
        }
    }
    if left == WhatTheRegistrationStepLeft::TheRecordAndTheRegistrationItself {
        // **Every refusal of the three user-level acts is a failure before the
        // commit, in the machine's own words, and takes the rollback.** Until
        // 2026-09-21 each returned straight out of the walk and left the claim
        // standing — the second pass on Sesterce.
        if let Err(said) =
            the_user_level_registration(surface, needs.how_long_the_user_manager_is_waited_for)
                .await
        {
            return the_attempt_failed_before_commit(surface, area, record, &said).await;
        }
    }

    // **K15's read-back, of content and never of presence.**
    match confirm_the_registration(surface.subsystem, &this_ones_record).await {
        RegistrationConfirmation::ContentMatchesTheRecord => {}
        RegistrationConfirmation::ContentDiffers {
            bytes_on_the_machine,
            bytes_recorded,
        } => {
            return the_attempt_failed_before_commit(
                surface,
                area,
                record,
                &format!(
                    "registration content differs from the record Smoozy wrote \
                     ({bytes_on_the_machine} bytes on the machine, {bytes_recorded} recorded)"
                ),
            )
            .await
        }
        RegistrationConfirmation::TheContentCouldNotBeRead {
            what_the_machine_said,
        } => {
            return the_attempt_failed_before_commit(
                surface,
                area,
                record,
                &format!("the registration could not be read back: {what_the_machine_said}"),
            )
            .await
        }
    }

    // ---------------------------------------------------- act six: the commit
    match whether_the_commit_may_proceed(&needs.accepted_work) {
        WhetherTheCommitMayProceed::ItMayProceed => {}
        // **`Rented-server runtime` G22: a blocked commit is *deferred* and is
        // not a failure.** The staged material is kept.
        WhetherTheCommitMayProceed::Deferred { because } => {
            record.phase = Phase::Deferred;
            record.deferred_because = Some(because.clone());
            let ordering = write_the_phase_record(surface.subsystem, area, record).await?;
            surface.note(ordering);
            release_the_claim_at_the_terminal_write(surface.subsystem, area).await?;
            return Ok(WhatThePlacementReached::Deferred { because });
        }
    }

    perform_the_commit(surface, area, record, needs, registration_changes).await
}

/// **G25's ordered steps entered on a record that is already past the staging**
/// — G17's three activation rows, and G18's *completing a commit*.
///
/// **G9's answer is recovered from the standing record rather than recomputed.**
/// The registration this placement makes is already on the machine, so
/// comparing it with itself would always answer *unchanged* and the rollback
/// would restore nothing. What the comparison wants is G4's carried-forward
/// previous content against what stands now, which is the same pair
/// [`complete_the_rollback`] already compares.
async fn continue_the_standing_commit<S>(
    surface: &mut ThePlacementsSurface<'_, S>,
    area: &str,
    standing: &mut PhaseRecord,
    needs: &WhatThePlacementNeeds<'_>,
) -> Result<WhatThePlacementReached, WhyThePlacementCouldNotBeCarriedOut>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    let on_the_machine = match read_the_boot_registration_record(surface.subsystem, area).await? {
        WhatARecordHeld::It(it) => it.content,
        _ => Vec::new(),
    };
    let carried_forward = standing
        .previous
        .as_ref()
        .map(|previous| previous.previous_boot_registration_content.clone())
        .unwrap_or_default();
    let registration_changes =
        whether_the_registration_changes(&carried_forward, &on_the_machine);

    perform_the_commit(surface, area, standing, needs, registration_changes).await
}

/// **`Rented-server runtime` G25's six ordered steps, performed.**
///
/// The order is `smoozy_runtime::package::commit::CommitStep::IN_ORDER` and it
/// is read from there rather than repeated: **the identity durable, the single
/// rename, the *committed, unverified* phase, the supervisor ended, the
/// restarter asked, and only then the exchange.**
async fn perform_the_commit<S>(
    surface: &mut ThePlacementsSurface<'_, S>,
    area: &str,
    record: &mut PhaseRecord,
    needs: &WhatThePlacementNeeds<'_>,
    registration_changes: WhetherTheRegistrationChanges,
) -> Result<WhatThePlacementReached, WhyThePlacementCouldNotBeCarriedOut>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    // **`Rented-server runtime` Z2 and Z3, before the pointer moves**: the
    // program the pointer is about to name refuses to start a supervisor where
    // no endpoint path is recorded, so a commit without this record commits a
    // program that cannot run. It is written before *committing* because it is
    // not the commit's act and a crash after it changes nothing a later reading
    // relies on — the record is fixed for the installation and kept once it
    // stands.
    if let Some(ordering) = record_the_endpoints_path(surface.subsystem, area).await? {
        surface.note(ordering);
    }

    record.phase = Phase::Committing;
    let ordering = write_the_phase_record(surface.subsystem, area, record).await?;
    surface.note(ordering);

    // **Whether G25's start came back refused.** Where it did, no supervisor
    // was started for the exchange to reach, and that is G13's *did not answer*
    // — the restore follows, and the claim is given back at its terminal write.
    // **Until step 10c piece 12 this ended the walk with the machine's words
    // and nothing else**: the phase stood at *committed, unverified*, the
    // pointer named whatever the commit had made it name, and the claim stood,
    // so the next setup was told *a package transaction is in progress* and
    // started nothing.
    let mut the_start_was_refused = false;
    // **Whether a container's running supervisor answered G25's ending and
    // would not take it.** It is said on the way out rather than acted on here:
    // the start still runs, the exchange still goes out, and `Rented-server
    // runtime` G13 decides — a supervisor that would not end is the one that
    // answers, so the version it names is the old one and the placement rolls
    // back. **This exists so the reason is not lost**, a rollback for a stale
    // version otherwise looking exactly like a rollback for a bad package.
    let mut the_running_supervisor_would_not_end: Option<String> = None;

    for step in CommitStep::IN_ORDER {
        match step {
            // **`Rented-server runtime` G10.** Where the identity cannot be
            // written the pointer is not attempted, naming the identity as the
            // unit that stopped it.
            CommitStep::TheInstallationIdentityIsMadeDurable => {
                let written = make_the_installation_identity_durable(
                    surface.subsystem,
                    area,
                    &needs.installation,
                )
                .await;
                if let Ok(ordering) = written {
                    surface.note(ordering);
                }
                if written.is_err()
                    || the_ordered_pair(written.is_ok()) != TheOrderedPair::TheIdentityIsDurableSoThePointerMayMove
                {
                    return the_attempt_failed_before_commit(
                        surface,
                        area,
                        record,
                        "the installation identity could not be made durable, so the pointer was not attempted",
                    )
                    .await;
                }
            }
            // **`Rented-server runtime` G7.**
            // **`Rented-server runtime` G7, on the record's own staging
            // location** — which is the attempt being committed, and is the
            // caller's own value on a fresh attempt and the standing record's on
            // a continuation.
            CommitStep::TheDecidingWriteTheSingleRenameOfThePointer => {
                let staging_location = record.staging_location.clone();
                rename_the_pointer_to(surface.subsystem, area, &staging_location).await?;
            }
            CommitStep::TheCommittedUnverifiedPhaseIsWritten => {
                record.phase = Phase::CommittedUnverified;
                let ordering = write_the_phase_record(surface.subsystem, area, record).await?;
                surface.note(ordering);
            }
            // **`Rented-server runtime` G25's ending, and it is performed as
            // nothing.** A rename starts no process and both restarters act
            // after an exit. **This side has no route to a signal**, so nothing
            // here ends a running supervisor, and that is said rather than
            // performed badly — the entry for it is [`crate::arriving_later`].
            //
            // **What the start below then does to a running unit on a systemd
            // machine is not established anywhere in this project**, and no
            // claim about it is made here. It was stated as fact with no source
            // until a code round struck it; the question is parked in
            // `Preparation/Groundwork/Open Threads.md`, with what breaks if the
            // answer is that a plain start of an already-active unit does
            // nothing. **Nothing is guessed in its place** — Rule 2 forbids
            // writing a remembered fact about somebody else's software into the
            // code, and a stop-then-start added on a guess would be exactly
            // that.
            //
            // ~~**On a container `Rented-server runtime` G25 has the start step
            // below invoke S5's keeper itself, and that is not built** —
            // [`start_the_supervisor_the_pointer_names`] says why, and the same
            // entry carries it.~~ **Built 2026-09-21, step 10c: on a container
            // the start step below starts S5's keeper, once.** *(Corrected
            // 2026-09-19: this called it a plan hole, and the plan was repaired
            // on 2026-09-12.)*
            //
            // **CHANGED 2026-09-22, step 10c's third pass: on a systemd machine
            // the ending is performed, by the start step below.** That step is
            // now `systemctl --user restart`, which the service manager's own
            // manual says stops and then starts the unit — so the running
            // supervisor is ended and the one the pointer names is started, in
            // G25's order, with no signal from here. The question parked in
            // `Open Threads` about a plain start is sidestepped rather than
            // answered.
            //
            // **CHANGED AGAIN 2026-09-22, out of the fourth pass: on a
            // container the ending is performed too.** The keeper already
            // running holds `Rented-server runtime` S5's lock, so the keeper
            // the start step launches ends at once and starts nothing; the
            // supervisor the old pointer started went on serving, and on the
            // fourth pass a run failed because of it. **§1.1 admits no
            // machine-origin value into a command string**, so nothing here
            // can signal a process; the machine is asked to end it instead.
            //
            // **CHANGED 2026-09-28, transfer wave piece 10 (words corrected
            // 2026-09-30): the machine ends it, not the supervisor.** The
            // pointer has already moved, so the entry point that reads the
            // asking is the new program's, and its adapter answers it itself:
            // it finds from the kernel which process holds the machine-local
            // endpoint and which program file it runs, and ends it by signal
            // only where that is not the file the pointer names. The keeper
            // already running then starts the one the pointer now names.
            // **Nothing is softened where it will not**: the start still runs
            // and G13's exchange is what decides.
            CommitStep::TheRunningSupervisorIsEnded => {
                if record.machine_kind == MachineKind::Container {
                    // **C1: the identity is the initiating side's, chosen
                    // before the request is sent.** It is an operation
                    // identity, which `State, identity and recovery` §2.1 makes
                    // the idempotency key. This build's adapter records
                    // nothing, so here it serves correlation; only an older
                    // release's supervisor records it before it acts (C4).
                    let ended = match Identity::issue(IdentityKind::Operation) {
                        Ok(identity) => {
                            ask_the_running_supervisor_to_end(
                                surface.subsystem,
                                surface.connection,
                                area,
                                identity,
                            )
                            .await
                        }
                        Err(_) => WhetherTheRunningSupervisorEnded::ItWouldNot(
                            "this machine would not supply random bytes for the ending's own \
                             identity"
                                .to_string(),
                        ),
                    };
                    if let WhetherTheRunningSupervisorEnded::ItWouldNot(why) = ended {
                        the_running_supervisor_would_not_end = Some(why);
                    }
                }
            }
            CommitStep::TheRestarterStartsTheSupervisorThePointerNames => {
                the_start_was_refused = start_the_supervisor_the_pointer_names(
                    surface.subsystem,
                    surface.connection,
                    record.machine_kind,
                    area,
                )
                .await
                .is_err();
            }
            CommitStep::TheHealthExchangeIsAttempted => {}
        }
    }

    if let Some(why) = &the_running_supervisor_would_not_end {
        eprintln!(
            "smoozy: the supervisor already running on this container would not end for the \
             package commit, so the exchange below reaches it rather than the one just placed: \
             {why}"
        );
    }

    // **`Rented-server runtime` G12 and G13, inside the caller's own
    // deadline.** A start that was refused is not asked at all.
    let answered = if the_start_was_refused {
        None
    } else {
        ask_for_the_health_answer_until_it_is_answered(
            surface,
            area,
            needs.the_health_exchanges_identity.clone(),
            needs.how_long_the_health_answer_is_waited_for,
        )
        .await?
    };
    let exchange = the_health_exchange(record, answered.as_ref(), &needs.installation);
    record.health_exchange = Some(exchange);

    match what_the_exchange_produces(record, exchange, registration_changes) {
        // **`Rented-server runtime` G15.** The previous package is removed only
        // after the exchange succeeds.
        WhatTheExchangeProduces::RemoveThePreviousPackage => {
            record.phase = Phase::Verified;
            let ordering = write_the_phase_record(surface.subsystem, area, record).await?;
            surface.note(ordering);
            remove_the_previous_package(surface.subsystem, record).await?;
            release_the_claim_at_the_terminal_write(surface.subsystem, area).await?;
            Ok(WhatThePlacementReached::Verified)
        }
        // **`Rented-server runtime` G6.** A rollback with no previous version to
        // restore leaves no active pointer, which is safe: nothing starts and
        // nothing claims to be Smoozy's.
        WhatTheExchangeProduces::RestoreLeavesNoActivePointer => {
            record.phase = Phase::RollingBack;
            let ordering = write_the_phase_record(surface.subsystem, area, record).await?;
            surface.note(ordering);
            match surface
                .subsystem
                .remove(smoozy_runtime::the_program::the_active_package_pointer_at(area).as_bytes())
                .await
            {
                Ok(()) => {}
                Err(why) if why.is_the_path_not_being_there() => {}
                Err(why) => return Err(why.into()),
            }
            sweep_what_the_rolled_back_attempt_staged(surface.subsystem, record).await?;
            record.phase = Phase::RolledBack;
            let ordering = write_the_phase_record(surface.subsystem, area, record).await?;
            surface.note(ordering);
            release_the_claim_at_the_terminal_write(surface.subsystem, area).await?;
            Ok(WhatThePlacementReached::RolledBack)
        }
        WhatTheExchangeProduces::RestoreFromTheRecordedPreviousValue { .. } => {
            record.phase = Phase::RollingBack;
            let ordering = write_the_phase_record(surface.subsystem, area, record).await?;
            surface.note(ordering);
            let standing = record.clone();
            complete_the_rollback(surface, area, &standing).await
        }
    }
}

/// **`Rented-server runtime` G14 and G14a, performed — and they are one act.**
///
/// **G14.** A failed exchange restores the previous pointer from
/// `Rented-server runtime` G4's recorded value, and the phase record moves to
/// *rolling back* before it. The previous package is present because
/// `Rented-server runtime` G5 keeps it until *verified*, so the restore is the
/// same single rename in the other direction. **The boot registration is
/// restored only in G9's case**, where this placement changed its content.
///
/// **G14a.** The boot-registration record is restored with the registration,
/// from the same recorded value and in the same case. Without it a restore that
/// puts the previous content back on the machine and leaves the record holding
/// this placement's content **makes every later read-back answer *content
/// differs*** — silently at the moment it happens, permanently afterwards, on a
/// machine the plan calls correct.
///
/// **The order of the two is K16b's, in this direction as in the other**, and it
/// is read from `TheRestoresTwoSteps::IN_ORDER` rather than from the order two
/// calls happen to sit in.
///
/// **G16.** A rollback that cannot complete is not a silent one: the phase
/// record stays at *rolling back* and the next request under any identity
/// completes it before considering anything else. **The claim is not released on
/// that path**, because a released claim over an incomplete rollback is exactly
/// the fresh transaction G16 forbids.
pub async fn complete_the_rollback<S>(
    surface: &mut ThePlacementsSurface<'_, S>,
    area: &str,
    record: &PhaseRecord,
) -> Result<WhatThePlacementReached, WhyThePlacementCouldNotBeCarriedOut>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    let mut rolling_back = record.clone();
    if rolling_back.phase != Phase::RollingBack {
        rolling_back.phase = Phase::RollingBack;
        let ordering = write_the_phase_record(surface.subsystem, area, &rolling_back).await?;
        surface.note(ordering);
    }

    let Some(previous) = rolling_back.previous.clone() else {
        // G6's branch again, reached where a rollback is completed by a later
        // request rather than by the attempt that began it.
        match surface
            .subsystem
            .remove(smoozy_runtime::the_program::the_active_package_pointer_at(area).as_bytes())
            .await
        {
            Ok(()) => {}
            Err(why) if why.is_the_path_not_being_there() => {}
            Err(why) => return Err(why.into()),
        }
        sweep_what_the_rolled_back_attempt_staged(surface.subsystem, &rolling_back).await?;
        rolling_back.phase = Phase::RolledBack;
        let ordering = write_the_phase_record(surface.subsystem, area, &rolling_back).await?;
        surface.note(ordering);
        release_the_claim_at_the_terminal_write(surface.subsystem, area).await?;
        return Ok(WhatThePlacementReached::RolledBack);
    };

    // **G9's case and no other**, decided by comparing the record on the machine
    // with the content the previous version carried: where the two are the same
    // the registration is version-independent and there is nothing to restore on
    // either half.
    let standing = match read_the_boot_registration_record(surface.subsystem, area).await? {
        WhatARecordHeld::It(standing) => Some(standing),
        _ => None,
    };
    if let Some(standing) = standing {
        if whether_the_registration_changes(
            &previous.previous_boot_registration_content,
            &standing.content,
        ) == WhetherTheRegistrationChanges::ThisPlacementChangesItsShape
        {
            let restored = the_record_a_rollback_restores(&standing, &previous);
            for step in TheRestoresTwoSteps::IN_ORDER {
                match step {
                    // **G14a, and K16b's order puts it first.**
                    TheRestoresTwoSteps::TheRecordHoldingThePreviousContent => {
                        let ordering =
                            write_the_boot_registration_record(surface.subsystem, area, &restored)
                                .await?;
                        surface.note(ordering);
                    }
                    // **G14's own half.**
                    TheRestoresTwoSteps::TheRegistrationItself => {
                        let ordering = crate::placement::records::publish_durably(
                            surface.subsystem,
                            &restored.at_path,
                            &restored.content,
                        )
                        .await?;
                        surface.note(ordering);
                    }
                }
            }
        }
    }

    // **G14's pointer restore — the same single rename in the other
    // direction**, from the recorded value and never from a second reading of
    // the machine.
    rename_the_pointer_to(surface.subsystem, area, &previous.location).await?;
    sweep_what_the_rolled_back_attempt_staged(surface.subsystem, &rolling_back).await?;

    rolling_back.phase = Phase::RolledBack;
    let ordering = write_the_phase_record(surface.subsystem, area, &rolling_back).await?;
    surface.note(ordering);
    release_the_claim_at_the_terminal_write(surface.subsystem, area).await?;
    Ok(WhatThePlacementReached::RolledBack)
}

/// **`Rented-server runtime` G5 and §13 R1: the material a rolled-back attempt
/// leaves is swept when the transaction reaches *rolled back*.**
///
/// `Rented-server runtime` G5: *"a committed package is still at the staging location the phase record
/// names, so after G14 that material is this attempt's own staged material, and
/// G20's bounded sweep removes it on Section 13 R1's terms when the transaction
/// reaches rolled back"*. **Nothing swept it until step 10c piece 12**, and a
/// later attempt under another identity sweeps only its own location — so each
/// failed commit left one program on the machine for the life of the rental.
///
/// **It runs before *rolled back* is written**, so a rollback interrupted
/// between the two is still *rolling back* and the next request sweeps again,
/// rather than a terminal record over material nothing will ever reach. **It
/// never reaches the previous package**: that is what the pointer has just been
/// restored to name, and a location equal to it is left where it is.
async fn sweep_what_the_rolled_back_attempt_staged<S>(
    subsystem: &mut Subsystem<S>,
    record: &PhaseRecord,
) -> Result<(), SubsystemFailure>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    let is_the_previous_package = record
        .previous
        .as_ref()
        .is_some_and(|previous| previous.location == record.staging_location);
    if is_the_previous_package {
        return Ok(());
    }
    sweep_the_abandoned_staging(subsystem, record).await
}

/// The *failed before commit* ending, with the sweep the phase owes.
///
/// **`Rented-server runtime` G3's own third column**: the machine is on the
/// previous package and staged material is owed a sweep.
async fn the_attempt_failed_before_commit<S>(
    surface: &mut ThePlacementsSurface<'_, S>,
    area: &str,
    record: &mut PhaseRecord,
    because: &str,
) -> Result<WhatThePlacementReached, WhyThePlacementCouldNotBeCarriedOut>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    record.phase = Phase::FailedBeforeCommit;
    let ordering = write_the_phase_record(surface.subsystem, area, record).await?;
    surface.note(ordering);
    sweep_the_abandoned_staging(surface.subsystem, record).await?;
    release_the_claim_at_the_terminal_write(surface.subsystem, area).await?;
    Ok(WhatThePlacementReached::FailedBeforeCommit {
        because: because.to_string(),
    })
}

/// Which of `Rented-server runtime` G17's three pointer answers the machine
/// gives.
///
/// **A pointer that could not be read is not a third answer**: it reads as
/// naming neither, which is the safe direction — the restore from a recorded
/// value rather than from the machine.
async fn what_the_pointer_names<S>(
    subsystem: &mut Subsystem<S>,
    area: &str,
    record: &PhaseRecord,
) -> Result<WhatThePointerNames, SubsystemFailure>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    let pointing_at = the_active_pointers_value(subsystem, area).await?;
    Ok(match pointing_at.as_deref() {
        Some(target) if target == record.staging_location => {
            WhatThePointerNames::TheRequestedVersion
        }
        Some(target)
            if record
                .previous
                .as_ref()
                .is_some_and(|previous| previous.location == target) =>
        {
            WhatThePointerNames::ThePreviousVersion
        }
        _ => WhatThePointerNames::Neither,
    })
}

/// The artifact's bytes, placed at the staging location.
///
/// **The create is exclusive**, so a staging path an abandoned attempt left
/// behind is met rather than written over blindly — the sweep above has already
/// removed it, and a create that is refused here says that something else is
/// inside this attempt's own location.
///
/// **The writes are pipelined and the pipeline is bounded**, at the transfers'
/// own [`THE_MOST_WRITES_IN_FLIGHT`] and for the reason written there: every
/// write's answer comes back on this same channel, and a loop that never
/// collects them fills the channel's own queue until neither end can move.
/// **Until 2026-09-21 this loop handed every write over before collecting any
/// answer**, and the SSH library's per-channel queue holds a hundred unread
/// messages (`russh` 0.63.1's own default, read in its source), after which
/// the connection's whole loop stops — keepalives included, so not even a
/// broken line is declared. At the 261,120-byte write OpenSSH announces that
/// is an artifact of about 26 MB; at the 32,768-byte write used where a server
/// announces nothing it is about 3.3 MB, **which today's 4 MB release is
/// already over**. Past it the placement waited for ever. Hosted runs `35595657437` and
/// `35601023619` hung three and a half hours on it, placing the workspace's
/// own unstripped program; measured at the desk as ninety-three writes passing
/// and a hundred and one hanging.
async fn stage_the_artifact<S>(
    subsystem: &mut Subsystem<S>,
    at: &str,
    bytes: &[u8],
) -> Result<(), SubsystemFailure>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    let handle = subsystem.create_exclusively(at.as_bytes()).await?;
    let chunk = subsystem.announcement().limits.write_chunk().max(1);
    let mut at_byte = 0u64;
    for piece in bytes.chunks(chunk) {
        subsystem.send_a_write(&handle, at_byte, piece).await?;
        at_byte += piece.len() as u64;
        if subsystem.writes_in_flight() >= THE_MOST_WRITES_IN_FLIGHT {
            subsystem.settle_the_writes().await?;
        }
    }
    subsystem.settle_the_writes().await?;
    subsystem.close(&handle).await
}

impl<S: AsyncRead + AsyncWrite + Unpin + Send> ThePlacementsSurface<'_, S> {
    /// §1.2 item 2's declared read size, taken from the announcement rather
    /// than assumed.
    fn subsystem_read_chunk(&self) -> usize {
        self.subsystem.announcement().limits.read_chunk()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_runtime::the_program::THE_FIXED_ENTRY_POINT_NAME;

    /// **The command names the carriage's own token and the fixed entry-point
    /// name**, and neither is written twice by hand: this is what holds the
    /// three constants together.
    #[test]
    fn the_health_exchange_command_names_the_token_and_the_entry_point() {
        assert_eq!(
            THE_HEALTH_EXCHANGE_COMMAND.as_str(),
            format!(
                "{}/{THE_FIXED_ENTRY_POINT_NAME}",
                THE_ENTRY_POINTS_CARRIAGE_SITE.named_in_a_command_as
            )
        );
        assert!(THE_ENTRY_POINTS_CARRIAGE_SITE.is_drawn_from_class_twos_alphabet());
        assert!(THE_ENTRY_POINTS_CARRIAGE_SITE
            .named_in_a_command_as
            .ends_with(THE_ENTRY_POINTS_CARRIAGE_SITE.link_path_on_the_subsystem));
    }

    /// **The link's directory is what the machine's own program will call the
    /// package area**, which is the whole reason the link points at the area
    /// rather than at the entry point.
    #[test]
    fn the_directory_the_command_invokes_from_is_the_carriage_token() {
        let invoked = THE_HEALTH_EXCHANGE_COMMAND.as_str();
        let (directory, name) = invoked.rsplit_once('/').expect("it names a directory");
        assert_eq!(name, THE_FIXED_ENTRY_POINT_NAME);
        assert_eq!(
            directory,
            THE_ENTRY_POINTS_CARRIAGE_SITE.named_in_a_command_as
        );
    }

    /// **The four field names are the machine's own**, and this reads that
    /// file's source rather than trusting that the two copies still agree.
    ///
    /// **The copy exists because that module is gated to a unix** and the
    /// desktop application runs on three systems. **The request name is no
    /// longer a copy**: both paths re-export `smoozy_framing::requests`' one
    /// constant, and that crate's own test fails on a second spelling anywhere
    /// in the workspace — so what is checked of the name here is only that the
    /// machine's path still re-exports it.
    #[test]
    fn the_health_answers_names_are_the_ones_the_machine_answers_with() {
        let theirs = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../Rented-server runtime/src/acting/package.rs"),
        )
        .expect("the machine's own half is readable");
        assert!(
            theirs.contains("pub use smoozy_framing::requests::THE_HEALTH_ANSWER_REQUEST;"),
            "the machine no longer answers under the one spelling"
        );
        for field in THE_FOUR_THINGS_THE_HEALTH_EXCHANGE_ASKS_FOR {
            assert!(
                theirs.contains(&format!("\"{field}\"")),
                "the machine's own answer does not carry {field}"
            );
        }
    }

    /// **A result carrying three of the four is not a health answer.**
    #[test]
    fn a_result_missing_one_of_the_four_is_not_an_answer() {
        use smoozy_framing::framing::Fields;
        let identity = Identity::issue(IdentityKind::Operation).expect("randomness");
        let request = Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            identity,
            THE_HEALTH_ANSWER_REQUEST,
        );
        let three = Answered::to(&request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "0.1.0")
            .carrying(
                Fields::none()
                    .and(THE_FOUR_THINGS_THE_HEALTH_EXCHANGE_ASKS_FOR[0], "0.1.0")
                    .and(
                        THE_FOUR_THINGS_THE_HEALTH_EXCHANGE_ASKS_FOR[1],
                        Identity::issue(IdentityKind::Run)
                            .expect("randomness")
                            .as_text(),
                    )
                    .and(
                        THE_FOUR_THINGS_THE_HEALTH_EXCHANGE_ASKS_FOR[2],
                        Identity::issue(IdentityKind::Installation)
                            .expect("randomness")
                            .as_text(),
                    ),
            );
        assert!(the_health_answer_in(&three).is_none());
    }

    /// **The seven values cross and nothing is composed from them**, which is
    /// `Rented-server runtime` G26's *consumes rather than redefines*.
    #[test]
    fn the_two_descriptors_carry_the_same_seven_values() {
        let wire = TheDescriptorsValues {
            manifest_format: "smoozy-server-package-manifest-1".to_string(),
            verified_release_sequence: 7,
            requested_package_version: "0.1.0".to_string(),
            manifest_digest: [7; 32],
            artifact_identity: "linux-x86_64-gnu".to_string(),
            artifact_byte_length: 4096,
            artifact_digest: [9; 32],
        };
        let record = the_phase_records_descriptor_for(&wire);
        assert_eq!(record.manifest_format, wire.manifest_format);
        assert_eq!(record.verified_release_sequence, wire.verified_release_sequence);
        assert_eq!(
            record.requested_package_version,
            wire.requested_package_version
        );
        assert_eq!(record.manifest_digest, wire.manifest_digest);
        assert_eq!(record.artifact_identity, wire.artifact_identity);
        assert_eq!(record.artifact_byte_length, wire.artifact_byte_length);
        assert_eq!(record.artifact_digest, wire.artifact_digest);
    }
}
