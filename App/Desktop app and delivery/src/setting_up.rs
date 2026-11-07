//! **The desktop drives a setup** — `Server Setup` R2's order, performed on a
//! machine over `Server protocol and transfers` T5's per-server point.
//!
//! **Nothing here decides what a setup is.** Every judgement below is `Server
//! Setup`'s, in `App/Server Setup/` and `App/Server Setup — machine/`, and
//! every act on the machine is that module's or `Server protocol and
//! transfers`'. What this file is is **the order, the record and the waiting**:
//! which reading comes first, what a refusal stops, what is written down at the
//! hand-over, and how a run is driven one step at a time.
//!
//! # `Server Setup` R2's order, and nothing out of it
//!
//! On a machine never prepared:
//!
//! 1. **`Server Setup` M1's kind and `Server Setup` M1a's family, read first,
//!    and nothing else if either refuses.** Both are readings; nothing on the
//!    machine is touched by either, which is what makes a refusal here a
//!    reading rather than a part-finished run.
//! 2. **`Server protocol and transfers` B8's detected rental tuple**, on the
//!    pre-package surface, so `Rented-server runtime` G28's unsupported-machine
//!    answer comes before any package byte exists.
//! 3. **`Server Setup` R6's candidate set and `Server Setup` R7's hand-over** —
//!    the root's directory created, the mount handed over, the five areas made,
//!    and `Server Setup` R7b's recording.
//! 4. **The verified release fetched** — `Server protocol and transfers` B8d's
//!    own-release address, then `Server protocol and transfers` B9 and
//!    `Server protocol and transfers` B10's whole admission.
//! 5. **The placement**, through the bootstrap-and-repair request
//!    (`Server protocol and transfers` §5; `Rented-server runtime` §2).
//! 6. **The setup run**, driven one step at a time over the command route
//!    (`Server Setup` Y7).
//!
//! # What this file writes down, and why each write is here rather than
//! anywhere else
//!
//! - **`Job files and outputs` L1's root, in both places it names**:
//!   `<package area>/root` on the machine, which the machine's own supervisor
//!   reads before it will accept a job, and the server row on this side.
//!   `Server Setup` R7b makes the recording a step of the hand-over, and the
//!   hand-over is here. **Without it every job on a real rental is refused**,
//!   which is what step 10c piece 5 found.
//! - **`State, identity and recovery` §2.3's installation identity against the
//!   row, with the row's event cursor cleared in the same durable write.** The
//!   cursor is reader state keyed to an installation (`State, identity and
//!   recovery` §4 E2); a cursor left pointing into a machine that no longer
//!   exists reads another machine's work as this one's. Step 10c piece 7 named
//!   this as owed here.
//!
//! # What is *not* here
//!
//! **`Server Setup` R6's attached volumes are not enumerated**, and
//! [`the_locations_this_login_can_offer`] carries what that costs. **The two
//! steps `Server Setup`'s own `driving::who_performs` gives the setup worker
//! that the worker cannot perform are taken here**, which is what step 10c
//! piece 8 found and refused by name on its side.
//!
//! # How a clause marker is written in this crate, and it is a rule rather than
//! a habit
//!
//! A marker with no document beside it means a clause of this crate's own plan,
//! `Preparation/Architecture/Desktop app and delivery.md`. **Every marker of
//! another document is written out with that document at its own site, and this
//! file keeps no header list** — the list cannot tell a marker both plans
//! define from one only the other does, and this file cites documents that
//! collide with this plan on most of their letters.

use std::time::Duration;

use smoozy_server_protocol::requests as asked;
use smoozy_job_files::layout::{Area, MachineRoot, TheLayout, WhereThePackageAreaSits};
use smoozy_job_files::root_choice::{self, Candidate};
use smoozy_security_and_licence::plaintext::Origin;
use smoozy_server_protocol::framing::{Answered, Fields, Request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS};
use smoozy_server_protocol::free_space::FreeSpaceReading;
use smoozy_server_protocol::placement::performing::{
    ThePlacementsOutcome, WhatThePlacementNeeds, WhatThePlacementReached,
    WhyThePlacementCouldNotBeCarriedOut,
};
use smoozy_server_protocol::release::composing::WhyTheFetchFailed;
use smoozy_server_protocol::release::PublicKeyBytes;
use smoozy_server_protocol::release::verifying::{
    accept_a_release, verify_what_was_downloaded, AVerifiedRelease, AnOfferedRelease,
    WhyTheDownloadWasRefused, WhyTheReleaseWasNotAccepted,
};
use smoozy_server_setup::acts;
use smoozy_server_setup::driving;
use smoozy_server_setup::elevated::{self, AnElevatedStep, HowElevationIsObtained};
use smoozy_server_setup::kind::{
    DistributionFamily, FamilyReading, MachineKind, TheRentalsTuple,
};
use smoozy_server_setup::placement::{
    compose_the_candidate_set, what_the_hand_over_records, AWritableLocation,
    WhatTheLocationIs, WhyTheSetIsEmpty,
};
use smoozy_server_setup::readings::{self, TheMachinesFileSurface};
use smoozy_server_setup::ready_action::{self, Step, WhatTheMachineNeeds};
use smoozy_server_setup::run::{compose_the_boundary_list, Boundary, SetupKind, WhatTheStepDid};
use smoozy_servers::hardware::Cards;
use smoozy_servers::restart::{HowTheWaitEnds, TheRestartMark};
use smoozy_servers::store::{RecordedMachineRoot, ServerRow};
use smoozy_runtime::package::phase::{Route, TransactionOwner};
use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_telegram::material::TheMaterial;

use crate::servers::installation::{what_the_package_area_holds, where_the_package_area_is};
use crate::servers::published::{the_direction, TheDirection, THE_MACHINE_IS_NEWER};
use crate::servers::sessions::{AHeldConnection, APiecePrinted, ItsOwnFuture, ThisServersSessions};
use crate::submitting::OnThisServersPoint;

// ════════════════════════════════════════════════════════════════════════════
// `Server protocol and transfers` B8d — a desktop reads its own release, and no
// other
// ════════════════════════════════════════════════════════════════════════════

/// **The tag this build's own release is published under** — the letter `v`
/// and the package version exactly as the workspace carries it, and nothing
/// else. Step 10c's transfer wave, W0-1: the form is fixed in one sentence of
/// `Preparation/Groundwork/Templates/Release Runner.md` step 6, which the
/// release runner publishes by, and
/// [`tests::the_tag_is_the_release_runners_own_form`] reads that sentence's
/// example rather than a copy of it here.
pub const THE_TAG_OF_THIS_BUILDS_OWN_RELEASE: &str = concat!("v", env!("CARGO_PKG_VERSION"));

/// **`Server protocol and transfers` B8d: the address a desktop reads its own
/// release's file from, and nowhere else.** *(Until 2026-09-28 this was
/// `Server protocol and transfers` B8b's latest address,
/// `…/releases/latest/download/release`, so a desktop not yet updated placed a
/// program newer than itself. That address now has one reader, the release
/// runner, and no desktop reads it.)*
///
/// **The repository is `Server protocol and transfers` B8a's** — the Creator's
/// public releases repository, whose owner and name that clause deliberately
/// leaves outside itself; they are written here because a build has to carry
/// them. **The file name is Build's**, and it is the name step 8a's own
/// composing tool publishes the release file under. **The tag is
/// [`THE_TAG_OF_THIS_BUILDS_OWN_RELEASE`]**, whose form is the release runner's.
///
/// **The form is `/releases/download/<tag>/<file>`**, observed on every release
/// since the fifth (the release runner fetches each one by it after
/// publishing) and shown in GitHub's own example answer, as
/// `Server protocol and transfers` B8d cites. **It carries the build's own
/// package version and nothing derived from a commit or a count**, which is the
/// second narrowing of `Server protocol and transfers` B8a's sentence and says
/// so: every artifact source is still a value the verified manifest carries.
///
/// **Nothing is trusted about what this address serves.**
/// `Server protocol and transfers` B9 and `Server protocol and transfers` B10
/// check the key-set statement's root signature, its non-rollback rules, the
/// manifest's signature and canonical digest, and the artifact's length and
/// digest, and [`this_builds_own_release`] refuses a manifest naming any
/// package version but this build's.
///
/// **The name is kept from `Server protocol and transfers` B8b's day**: *the
/// current release* is now this build's own, and the desk tests that stand a
/// hosting table on this key read it by that name.
pub const WHERE_THE_CURRENT_RELEASE_IS_READ_FROM: &str = concat!(
    "https://github.com/a-kislyakov/Smoozy-releases/releases/download/v",
    env!("CARGO_PKG_VERSION"),
    "/release"
);

/// **The most bytes the release file itself may be.**
///
/// The release file carries two signed documents and two signatures and nothing
/// else (`Server protocol and transfers`' own `AnOfferedRelease`), so it is
/// small. **The bound is on the fetch rather than on the file**, because a
/// length taken off a far end is a number the far end chose.
pub const THE_MOST_THE_RELEASE_FILE_MAY_BE: u64 = 1024 * 1024;

// ════════════════════════════════════════════════════════════════════════════
// What a setup run is asked for, and what it came to
// ════════════════════════════════════════════════════════════════════════════

/// What the user asked this setup to do.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WhatWasAskedFor {
    /// **`Server Setup` B2's pick, where the user asked for a Blender.** The
    /// version travels and no address ever does — `Server Setup` B4 derives the
    /// archive's address and its checksum's from the version, on the machine.
    pub install_this_blender: Option<String>,
    /// Whether `Server Setup` §7's Telegram material is placed by this run.
    pub telegram_is_on: bool,
    /// **`Server Setup` R1b: every Blender `Server Setup` B10's search found on
    /// the server**, which `Server Setup` R1 item 5's bar is taken over beside
    /// the one this run stages — the bar of the newest Blender on the server.
    /// Added 2026-10-12. **Empty where the caller took no search**, and then
    /// only the staged build gives a bar, as before that date.
    pub the_blenders_on_the_server: Vec<smoozy_server_setup::blender::Version>,
    /// **The local server identifier, where the caller has one** — what this
    /// desktop's transfers with the machine are read by, because `Server
    /// Setup` R1b counts a transfer under way as live work
    /// ([`crate::servers::the_driver_swap::a_transfer_is_under_way_with`]).
    /// Added 2026-10-12. **Nothing where the caller has none**, and then the
    /// machine's own live set is the whole of what is read.
    pub the_server: Option<String>,
}

/// **Where a setup got to, and every one of these is an answer rather than a
/// silence.**
///
/// Rule 8 makes Smoozy going quiet a fault however the trouble arose, so every
/// way this can stop has a word of its own and carries the machine's own words
/// where the machine said anything.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhereTheSetUpGot {
    /// **`Server Setup` R2's first reading refused**, and nothing else was
    /// asked. **Nothing on the machine was touched.**
    TheKindOrTheFamilyWouldNotRead { which: WhichReadingRefused },
    /// **`Server protocol and transfers` B8's tuple could not be read**, so no
    /// artifact can be selected against it and nothing is placed.
    /// `Rented-server runtime` G28's answer, reached before any package byte
    /// exists on the machine.
    TheRentalWouldNotNameItself,
    /// **`Server Setup` R6's set came out empty**, in that clause's own two
    /// ways.
    NoCandidateCouldBeOffered { why: WhyTheSetIsEmpty },
    /// `Job files and outputs` L1's choice refused every candidate.
    NoRootCouldBeChosen { said: String },
    /// **`Server Setup` R7's hand-over did not land.** The machine's own words
    /// travel.
    TheHandOverDidNotLand { said: String },
    /// The root was handed over and the record of it would not take.
    /// **`Job files and outputs` L1 records it in two places and a machine
    /// missing the first refuses every job**, so a half-written record stops
    /// here rather than going on.
    TheRootCouldNotBeRecorded { said: String },
    /// **This build's own release could not be fetched or would not be
    /// admitted** — `Server protocol and transfers` B8d,
    /// `Server protocol and transfers` B9 and `Server protocol and transfers`
    /// B10. Nothing was placed.
    TheReleaseWasNotAdmitted { said: String },
    /// **The placement ran and this is `Rented-server runtime` G3's phase in
    /// the caller's own terms.**
    ThePlacement { reached: WhatThePlacementReached },
    /// **The placement was deferred, and the program there holds jobs held
    /// for an answer that no new setup can give** — `Rented-server runtime`
    /// G21 and `Rented-server runtime` G23, with `State, identity and recovery` Y9, and `Design/Server
    /// Setup § When work on the server holds the setup back`. **A held job
    /// does not end on its own**, so unlike waiting or rendering work this
    /// deferral never lifts by itself; the card says how many and that this
    /// copy cannot answer them. `because` is the deferral's own record.
    /// Added 2026-10-01, step 10c's thirteenth-pass fix piece 2.
    JobsOfAnEarlierSetupAreHeld { how_many: usize, because: String },
    /// The surface itself would not carry the transaction.
    ThePlacementCouldNotBeCarriedOut { said: String },
    /// **The setup run was driven and this is where it ended** — the boundary
    /// it reached and `Server Setup` Y4's own ending.
    TheRunEnded {
        at: Boundary,
        ending: TheRunsEnding,
    },
    /// **The connection went while the setup was being driven.**
    /// `Server Setup` Y7: *"A connection lost part-way through stops it, and
    /// nothing resumes on its own."*
    TheConnectionWent,
    /// **A Blender install asked of a server with no program of Smoozy's placed
    /// on it.** A Blender install is a setup run (`Server Setup` Y1 and `Server Setup` Y6b), a run
    /// is opened with the machine's own program (`Server Setup` R2), and a
    /// server whose row confirms no installation has had no program placed.
    /// ~~a server with no recorded root has had no program placed by this
    /// copy~~ — **corrected the same day, 2026-09-21: a root is recorded before
    /// the placement, so a failed placement leaves one.** **Nothing was sent.**
    /// Added 2026-09-21.
    TheServerIsNotSetUpYet,
    /// **`Server Setup` B3's *latest finished release* could not be read off the
    /// shop-front's listing**, so there was no version to fetch and nothing was
    /// started. **Nothing is substituted** — `Server Setup` B5's own rule. Added 2026-09-21.
    TheBlenderListCouldNotBeRead { said: String },
}

/// Which of `Server Setup` R2's two opening readings refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhichReadingRefused {
    /// `Server Setup` M1: `Rented-server runtime` K1's table does not place it.
    TheKind,
    /// `Server Setup` M1a: the machine would not name a family this module
    /// supports.
    TheFamily,
}

/// How a driven run ended, in `Server Setup` Y4's own terms.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheRunsEnding {
    /// **Every composed step reached its boundary and *done* was written.**
    ///
    /// **Whether a Blender was published is said beside it**, because only then
    /// may the screen say the server is ready to render. Added 2026-09-22, out
    /// of step 10c's third pass on Vast: *"The server is ready to render."* was
    /// shown on a machine that held no Blender at all.
    ItReachedDone {
        a_blender_was_published: bool,
        /// **`Server Setup` R1 item 5: the run ended with the driver answering
        /// below the OptiX bar of the Blender it installed**, on a machine
        /// Smoozy manages — the swap was refused, went through and still read
        /// below, or the family has no swap (Debian). Added 2026-10-11. **The
        /// run is still *done*** (*"setup does not fail for it"*); this is what
        /// `Servers` H11b's statement is said from on such a machine
        /// (`smoozy_servers::hardware::the_statement_is_said_on`'s second
        /// input). **Keeping it in the stored record is the fourth piece's.**
        the_driver_missed_the_optix_bar: bool,
        /// **`Server Setup` R1b: item 5's swap was owed and was not made in
        /// this run, because work was live on the server** — at the driver
        /// step, or at the last guard once the machine-level state stood.
        /// Added 2026-10-12. **It is not a miss**: the swap waits, and the
        /// desktop starts it once the server's work has ended
        /// ([`crate::servers::the_driver_swap`]).
        the_swap_waits: bool,
    },
    /// **A step failed**, and the machine's own words travel with it.
    AStepFailed { step: Step, said: String },
    /// **`Server Setup` R1 item 8 failed on this computer: `Telegram` §20's
    /// list could not be composed**, so nothing was placed. Added 2026-10-17,
    /// the Creator's eleventh notes, note 2.
    ///
    /// **It is told apart from [`TheRunsEnding::AStepFailed`] because no server
    /// said anything**: composing is the desktop's, from its own record and
    /// its secret store. The step's own sentence — *The server could not
    /// install Telegram reporting.* — would blame the server, and the reason
    /// went behind *What the server said* as its internal name. **`said` is
    /// Smoozy's own one plain sentence and is the whole of what the card
    /// reads** (`told_record::WhyTheMaterialCannotBeComposed::in_words`). It
    /// is otherwise a failed step: written onto the machine (`Server Setup`
    /// S2a) and read as one wherever a failed run is.
    TheTelegramMaterialWasNotComposed { said: String },
    /// **`Server Setup` §8's restart command was refused while the connection
    /// stayed up** — the runner's ruling of 2026-09-27. Added the same day.
    ///
    /// **It ends as the restart step failed**, recorded on the machine as any
    /// failed step is (`Server Setup` S2a), which is what takes *restart
    /// issued* back ([`the_restart_mark_was_taken_back`]). **It is told apart
    /// from [`TheRunsEnding::AStepFailed`] only so the box can say the restart
    /// was refused**, which is the ruling's own words; the machine's words
    /// travel with it where it printed any, and Smoozy's own record of the
    /// refusal where it printed none.
    TheRestartWasRefused { said: String },
    /// **Smoozy's program on the server stopped answering, so what the step was
    /// doing is not known.** Added 2026-09-22, out of step 10c's fourth pass on
    /// Vast.
    ///
    /// **It is not [`TheRunsEnding::AStepFailed`] and the difference is the
    /// whole of the point.** A failed step is named by the sentence of the
    /// boundary it feeds, and the final verification's sentence is *the server
    /// did not pass Smoozy's final check* — which the pass put in front of the
    /// user on a run where nothing had been checked at all: the supervisor had
    /// stopped answering while the step was in flight. **The step is carried
    /// beside it** so the box can say which step was under way, and Smoozy's
    /// own record of the silence travels as the words.
    TheServerStoppedAnswering { step: Step, said: String },
    /// **The machine ended the run while this side was driving it**, and this
    /// is the ending its own record holds. Added 2026-09-27, step 10c's
    /// eighth-pass fix piece 1.
    ///
    /// **It is not [`TheRunsEnding::AStepFailed`], and step 10c's eighth pass
    /// on Hyperstack is why.** The machine's `Server Setup` Y7 window ended the
    /// run at *started* while the driver install ran; the desk wrote *driver
    /// placed* after it, the machine answered *"no setup run is open on this
    /// machine"*, and the user was told the server could not install the
    /// graphics driver — which it had. **The run's own record said *ended at
    /// its boundary, not completed***, which is `Design/Server Setup § Cut off
    /// part-way`, and that is what the box says. **The step is the one in
    /// hand**, so the box can say which was under way; **the words are the
    /// machine's** where it gave any — its recorded words, or its refusal of
    /// the write — and go behind *What the server said*.
    TheServerEndedTheRun { step: Step, said: Option<String> },
    /// **The unpack failed** — the worker's half of `Server Setup` R1 item 3,
    /// after the unpack tool was found or installed. Added 2026-09-22.
    ///
    /// **It is the tooling step's and is told apart from the tool's install**,
    /// because the two fail for different reasons and the box names what
    /// failed: step 10c's third pass on Vast was told *"The server could not
    /// install the unpack tool."* when the tool had installed cleanly and the
    /// unpack was what refused.
    TheUnpackFailed { said: String },
    /// The user's archive did not land. These are the upload's words, kept
    /// as Smoozy's own; no unpack or install step has run.
    TheBuildCouldNotBeSent { said: String },
    /// **`Server Setup` Y8: the user stopped it**, honoured at a boundary and
    /// never by a signal.
    TheUserStoppedIt,
    /// **`Server Setup` E4's fourth bullet: a step needed admin access, no
    /// password was held, and nothing was sent.** Added 2026-09-21.
    ///
    /// **Inside a run it is the moment the offer is made** (`Server Setup` E2),
    /// and [`TheRunBeingDriven::driven`] asks for the password there and takes
    /// the same step again with it. **It ends the run only where the offer was
    /// declined**, or nobody was there to be asked — and then the run is stopped
    /// at its boundary with the machine's own durable stop (`Server Setup` Y8),
    /// so it is not left open for `Server Setup` Y7's window to end.
    AStepNeedsAdminAccess { step: Step },
    /// The supervisor would not open a run at all.
    NoRunCouldBeOpened { said: String },
    /// **The supervisor would not open a run because a setup that did not
    /// finish still holds the machine** — `Rented-server runtime` A5b's refusal
    /// under `Rented-server runtime` A11, carrying the run the standing state
    /// names (`Rented-server runtime` P1) in `Rented-server runtime` V9's field,
    /// which `Server Setup` Y6 says the refused submitter is told. Added
    /// 2026-09-21, step 10c.
    ///
    /// **On the ready action it is not an ending at all**: `Server Setup` S3 and
    /// `Server Setup` S4 item 2 make running the setup again the recovery run that takes the
    /// standing one over, and [`TheRunBeingDriven::opened`] opens that instead.
    /// **It ends a run of any other kind**, which no recovery serves — the words
    /// say which setup stands and what to do about it.
    ASetupThatStoppedPartWayStands { standing: Identity, said: String },
    /// **A recovery was opened, and the setup it took over stopped genuinely
    /// inside a step that feeds more than one boundary.** Added 2026-09-21,
    /// step 10c. The run says so rather than writing a boundary it cannot vouch
    /// for.
    ///
    /// **Narrowed 2026-09-27 by the runner's ruling, out of step 10c's seventh
    /// passes on Hyperstack and Sesterce, where it fired wrongly.** The driver
    /// step feeds *driver placed* and *card answering* with the restart's own
    /// *restart issued* between them (`Server Setup` Y4), and **a marker at
    /// *driver placed* means the driver step finished**: the recovery runs the
    /// restart step afresh, its check first (`Server Setup` S4 item 3 and
    /// §4.3's restart bullet). A marker at *restart issued* carries on at the
    /// check (`Server Setup` T5). **So neither is inside the step, and this
    /// ending is kept only for a list that holds no *restart issued* between
    /// the driver's two boundaries** — which no composition this build makes
    /// produces, and a record from elsewhere might.
    ItStoppedInsideAStep { step: Step },
    /// **`Server Setup` T6's second ending: the user stopped waiting for the
    /// machine to come back, and that clears nothing.**
    ///
    /// *"The mark stands, the machine-level state stands, `Servers`' reconnect
    /// policy for that marked period goes on running, and the setup is picked
    /// up on return; what ends is the user's waiting and nothing else."* **So
    /// this run ends and the setup does not** — `Server Setup` Y8a says the
    /// earliest end available to the user after a restart is the recovery run's
    /// next boundary.
    TheMachineIsRestarting,
    /// **`Server Setup` T6's third ending: `Servers`' reconnect policy for the
    /// marked period gave up.**
    ///
    /// **What Smoozy says is only what it knows** — the connection is down and
    /// could not be restored, **with no claim to know why, because a machine
    /// still starting cannot be told from one that will never answer**. The
    /// setup is reported cut off part-way per `Server Setup` S2, off the
    /// machine's own record on the next connection.
    TheMachineDidNotComeBack { said: String },
}

// ════════════════════════════════════════════════════════════════════════════
// The seams — everything this driver reaches that is not the point's
// ════════════════════════════════════════════════════════════════════════════

/// **What the server row holds, and the two durable writes this driver makes to
/// it.**
///
/// **A seam, so the whole order is driven at the desk**: a judgement that can
/// only be reached through a socket and a database is a judgement nobody tests.
pub trait KeepsTheServersRow {
    /// The row as it stands, or nothing where the server is gone.
    fn the_row(&mut self) -> Option<ServerRow>;

    /// **`Server Setup` R7b and `Job files and outputs` L1's desktop half**:
    /// the root recorded against the row as a machine-derived value carrying
    /// the installation identity it was read under.
    fn record_the_root(&mut self, recorded: RecordedMachineRoot) -> Result<(), String>;

    /// **`State, identity and recovery` §2.3's identity, written with the
    /// row's event cursor cleared in the same durable act.**
    ///
    /// **The cursor is reader state keyed to an installation**
    /// (`State, identity and recovery` §4 E2). A new installation is a new
    /// journal, so a cursor carried over reads another machine's work as this
    /// one's — which is what step 10c piece 7 owed here.
    fn record_the_installation_and_clear_the_cursor(
        &mut self,
        identity: &Identity,
    ) -> Result<(), String>;

    /// **`Servers` B5 item 1: a connection is established, so the remembered
    /// setup is cleared**, whatever the setup then does.
    fn clear_the_remembered_setup(&mut self) -> Result<(), String>;

    /// **`Servers` Q19a's moment: the placement has committed and the
    /// installation is recorded against the row.** Told once per placement
    /// that commits and never for one that does not. **Nothing is done by
    /// default**: what it starts is the running application's
    /// (`crate::servers::a_period::a_placement_committed`), and whether
    /// anything is re-taken is that clause's own judgement, not this order's.
    /// Added 2026-09-27, step 10c's eighth-pass fix piece 3.
    fn the_placement_committed(&mut self) {}
}

/// **The bootstrap-and-repair transaction, over whatever connection the point
/// lends.**
///
/// **It is a seam because `Server protocol and transfers`' own performer takes
/// the two mechanisms as concrete things** — a negotiated file-transfer
/// subsystem and an SSH connection — and the desk has no SSH server of its own
/// to make one. **The real end is [`OverTheLentConnection`]**, which opens the
/// subsystem session on the connection the point lent and hands both to that
/// crate's own walk.
pub trait PlacesTheVerifiedRelease<C>: Send + 'static {
    fn place<'a>(
        &'a mut self,
        connection: &'a mut C,
        needs: WhatThePlacementNeeds<'a>,
    ) -> ItsOwnFuture<'a, Result<ThePlacementsOutcome, WhyThePlacementCouldNotBeCarriedOut>>;
}

/// **Everything one placement is handed, owned** — the same values
/// `Server protocol and transfers`' own `WhatThePlacementNeeds` takes, held
/// rather than borrowed so the act can be lent the whole connection.
pub struct WhatThePlacementIsGiven {
    pub package_area: String,
    pub attempt: Identity,
    pub owner: TransactionOwner,
    pub installation: Identity,
    pub verified: AVerifiedRelease,
    pub artifact: Vec<u8>,
    pub kind: smoozy_runtime::machine_kind::MachineKind,
    pub the_health_exchanges_identity: Identity,
    /// **`Rented-server runtime` K11's and K19's root step on a container**, with
    /// `Server Setup`'s own commands, for how elevation stands on this login.
    /// [`None`] on a systemd machine.
    pub the_root_step_on_a_container: Option<smoozy_server_protocol::placement::TheRootStepOnAContainer>,
    /// **What `Rented-server runtime` G21 reads**: every job the program
    /// already at this package area holds, with its standing. Empty where no
    /// program stands there. Added 2026-09-28, step 10c's transfer wave piece
    /// 8: until then the walk was always handed an empty list.
    pub accepted_work: Vec<(String, smoozy_runtime::package::retry::AcceptedWorkStanding)>,
    /// **`Rented-server runtime` G4's carried-forward previous version**, where
    /// a program is being replaced — `Desktop app and delivery` V11's update.
    /// Nothing on `Server Setup` R2's placement on a machine never prepared.
    pub previous: Option<smoozy_runtime::package::phase::PreviousVersion>,
}

/// **How long a setup's placement waits for the health answer** —
/// `Server protocol and transfers` C16's per-waiter deadline, which is the
/// setup's own. **A Build decision**: two minutes, far past a program starting
/// and binding its endpoint, so that only a supervisor that is not coming up
/// meets it. It is not a wait budgeted for a machine (`Rented-server runtime`
/// K17): the machine is already up and answering the connection.
pub const HOW_LONG_A_SETUP_WAITS_FOR_THE_HEALTH_ANSWER: std::time::Duration =
    std::time::Duration::from_secs(120);

/// **How long a setup's placement waits for the login's own service manager
/// to answer after lingering is granted** — `Rented-server runtime` K7 as
/// corrected on 2026-09-21, and `Server protocol and transfers` C16's
/// per-waiter deadline. **A Build decision**: sixty seconds. The login manager
/// starts the manager at the grant and does not wait for it, and a manager
/// that has not answered in a minute is not coming; where a session exists it
/// answers the first time and nothing is waited for. Not a wait budgeted for a
/// machine (`Rented-server runtime` K17): the machine is up and answering the connection.
pub const HOW_LONG_A_SETUP_WAITS_FOR_THE_USER_MANAGER: std::time::Duration =
    std::time::Duration::from_secs(60);

impl WhatThePlacementIsGiven {
    /// The borrowed shape that crate's own walk takes.
    ///
    /// **`Rented-server runtime` G4's carried-forward previous version is what
    /// the caller handed in**: nothing on `Server Setup` R2's placement on a
    /// machine never prepared, and the standing package's on V11's update.
    fn as_the_walk_takes_it(&self) -> WhatThePlacementNeeds<'_> {
        WhatThePlacementNeeds {
            package_area: self.package_area.clone(),
            attempt: self.attempt.clone(),
            owner: self.owner.clone(),
            installation: self.installation.clone(),
            verified: &self.verified,
            artifact: &self.artifact,
            kind: self.kind,
            previous: self.previous.clone(),
            // **The hook is `Rented-server runtime` K11's elevated step and its
            // commands are `Server Setup`'s**, handed to the walk here.
            // ~~so the walk is handed none and writes K16's record alone —
            // `Rented-server runtime` K16a's own ending.~~ **Struck 2026-09-21,
            // step 10c: handed none, the walk wrote a record naming no path and
            // no content, and every container placement failed at K15.**
            the_root_step_on_a_container: self.the_root_step_on_a_container.clone(),
            // **`Rented-server runtime` G21's block reads accepted work**, and
            // what it reads is the caller's reading of the program already at
            // this area — handed in, never assumed empty (step 10c's transfer
            // wave piece 8).
            accepted_work: self.accepted_work.clone(),
            the_health_exchanges_identity: self.the_health_exchanges_identity.clone(),
            how_long_the_health_answer_is_waited_for: HOW_LONG_A_SETUP_WAITS_FOR_THE_HEALTH_ANSWER,
            how_long_the_user_manager_is_waited_for:
                HOW_LONG_A_SETUP_WAITS_FOR_THE_USER_MANAGER,
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// `Server Setup` R2, step 1 — the kind and the family, and nothing else if
// either refuses
// ════════════════════════════════════════════════════════════════════════════

/// **The two opening readings, and what they settle.**
///
/// **`Server Setup` M1a's own last bullet is why they come first**: both are
/// taken over surfaces that need no elevation, nothing placed and no hand-over,
/// **so nothing on the machine is changed by either** — which is what makes a
/// refusal here a reading rather than a part-finished run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheOpeningReadingsSaid {
    pub kind: MachineKind,
    pub family: DistributionFamily,
}

/// **The two answers, taken from the machine.**
///
/// **A seam, and this is the one place in this file where that is not obvious,
/// so the reason is written here.** Both readings name **absolute paths on the
/// machine** — `/proc/1/comm` and `/etc/os-release` — which a desk cannot
/// answer at all: this computer has neither, and nothing may write one. **A
/// judgement that can only be reached through a socket is a judgement nobody
/// tests**, so the two answers cross a seam and the order around them is driven
/// at the desk whole.
pub trait TakesTheOpeningReadings {
    fn taken(&mut self) -> ItsOwnFuture<'_, readings::TheMachineBeforeAnythingIsTouched>;
}

/// **The two answers, over the per-server point** — `Server Setup` M1a's own
/// last bullet: a command-channel reading and a plain read over the
/// file-transfer subsystem, **needing no elevation, nothing placed and no
/// hand-over.**
pub struct TheOpeningReadingsOverThePoint<'s, C: AHeldConnection> {
    pub sessions: &'s ThisServersSessions<C>,
}

impl<C: AHeldConnection> TakesTheOpeningReadings for TheOpeningReadingsOverThePoint<'_, C> {
    fn taken(&mut self) -> ItsOwnFuture<'_, readings::TheMachineBeforeAnythingIsTouched> {
        Box::pin(async move {
            let mut channel = OnThisServersPoint {
                sessions: self.sessions,
            };
            let mut files = OnThisServersPoint {
                sessions: self.sessions,
            };
            readings::take_the_opening_readings(&mut channel, &mut files).await
        })
    }
}

/// `Server Setup` R2's first act, performed.
///
/// **A refusal reached because the connection went under a reading is not a
/// refusal**, and is told as the connection going (`Server Setup` Y7: *"a
/// connection lost part-way through stops it"*) — never as a machine Smoozy
/// does not support, which nobody read (Rule 8). The worldwide pass on Vast,
/// 2026-10-26: an Ubuntu 22.04 container was told *not a Linux server Smoozy
/// supports* at the moment a three-second reconnect took the connection from
/// under these readings, and the same machine set up on the next press.
/// **Nothing resumes on its own**: the spec's *"A connection that drops while
/// a setup is under way … nothing resumes on its own"* covers these readings,
/// and `Servers`' own repair is what puts the connection back.
pub async fn the_opening_readings(
    taking: &mut dyn TakesTheOpeningReadings,
) -> Result<WhatTheOpeningReadingsSaid, WhereTheSetUpGot> {
    what_the_opening_readings_came_to(taking.taken().await)
}

/// **[`what_the_opening_readings_settle`], with a reading that never reached
/// the machine told as the connection going.** Added 2026-10-26, the worldwide
/// pass's setup faults, fault 2. **A reading that never reached the machine
/// does not stop readings that did place it**: where the kind and the family
/// were both read, the setup goes on as before.
pub fn what_the_opening_readings_came_to(
    read: readings::TheMachineBeforeAnythingIsTouched,
) -> Result<WhatTheOpeningReadingsSaid, WhereTheSetUpGot> {
    let the_connection_went = read.a_reading_never_reached_the_machine();
    what_the_opening_readings_settle(read).map_err(|which| {
        if the_connection_went {
            WhereTheSetUpGot::TheConnectionWent
        } else {
            WhereTheSetUpGot::TheKindOrTheFamilyWouldNotRead { which }
        }
    })
}

/// The judgement over the two answers, with the answers handed in — **so the
/// whole of it is driven at the desk.**
pub fn what_the_opening_readings_settle(
    read: readings::TheMachineBeforeAnythingIsTouched,
) -> Result<WhatTheOpeningReadingsSaid, WhichReadingRefused> {
    let mut read = read;
    let opening = smoozy_server_setup::kind::read_before_anything_is_touched(&mut read);
    // **`Server Setup` M1: a machine `Rented-server runtime` K1's table does
    // not place is refused, and nothing else is asked.**
    let kind = match opening.kind {
        MachineKind::TheTableDoesNotPlaceIt => {
            return Err(WhichReadingRefused::TheKind);
        }
        placed => placed,
    };
    // **`Server Setup` M1a: an absent answer is not an answer.** A family this
    // module does not support, and a machine that would not say, are the same
    // refusal here — neither is read as Ubuntu.
    let family = match opening.family {
        FamilyReading::Supported(family) => family,
        _ => return Err(WhichReadingRefused::TheFamily),
    };
    Ok(WhatTheOpeningReadingsSaid { kind, family })
}

// ════════════════════════════════════════════════════════════════════════════
// `Server Setup` R2, step 2 — the rental's own tuple
// ════════════════════════════════════════════════════════════════════════════

/// **`Server protocol and transfers` B8's detected rental tuple**, asked of the
/// machine on the pre-package surface.
///
/// **Nothing is selected here and nothing falls back.** A machine that will not
/// answer composes no tuple at all, so `Rented-server runtime` G28's exact match
/// catches what it exists to catch rather than matching this build's own tuple
/// against itself.
pub async fn the_rentals_tuple<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
) -> TheRentalsTuple {
    let mut channel = OnThisServersPoint { sessions };
    readings::read_the_rentals_tuple(&mut channel).await
}

// ════════════════════════════════════════════════════════════════════════════
// `Server Setup` R2, step 3 — the candidate set, the hand-over, the recording
// ════════════════════════════════════════════════════════════════════════════

/// **`Server Setup` R6's candidate set, as this application can offer it
/// today — and what that costs is written here rather than left to be
/// discovered.**
///
/// R6's set is *"the mount the connection login's own home area sits on, plus
/// every attached volume"*. **The first is offered and the second is not.**
///
/// **Why the second is not.** Enumerating a machine's mounts and telling an
/// attached volume from a memory-backed filesystem, a read-only mount and a
/// host disk bind-mounted into a container needs a command and an output format
/// that no document in this project has read and dated. **Rule 2 forbids
/// composing either from an agent's memory**, and `Job files and outputs` L2's
/// own Fact 28 is what a guess costs: a plain reading on one container returned
/// **51 filesystems**, 48 memory-backed, one of them the host's disk showing 20
/// GiB the rental may not touch.
///
/// **What breaks while it is not offered, stated plainly.**
/// `Job files and outputs` L4 records a seller delivering `/dev/vda1` at 97 GB
/// on `/` and `/dev/vdb` at 787 GB on `/ephemeral` as standard, and says in as
/// many words that **on a machine of that shape the conventional root is the
/// wrong one by a factor of eight**. With one candidate offered, that is the
/// root this application chooses, and `Job files and outputs` L4a's second
/// choice never fires, so the package area sits under it. **Nothing is
/// corrupted and nothing is silent** — the machine is set up, the free-space
/// reading against the chosen root is real, and a full root is reported as a
/// full root — but the large mount is unused.
///
/// **What holds.** R6's exclusions decide what a location *is* rather than what
/// it is named, and the login's own home area is none of the three: it is not
/// memory-backed, it is not read-only, and it is the rental's own rather than
/// the host's. `compose_the_candidate_set` still applies every exclusion to
/// what is offered.
pub async fn the_locations_this_login_can_offer<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
) -> Vec<AWritableLocation> {
    let home = match the_logins_own_area(sessions).await {
        Some(home) => home,
        None => return Vec::new(),
    };
    let reading = the_free_space_at(sessions, &home).await;
    vec![AWritableLocation {
        path: home,
        what_it_is: WhatTheLocationIs::TheMountTheLoginsHomeAreaSitsOn,
        the_reading: reading,
    }]
}

/// **The login's own area, resolved by the machine rather than composed.**
///
/// **`Server protocol and transfers` §1.1 forbids a machine-origin value in a
/// command string**, and a home path is one — so this is the file-transfer
/// subsystem's own resolution of the relative path every session starts at, and
/// no command is sent at all.
async fn the_logins_own_area<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
) -> Option<String> {
    let resolved = sessions
        .a_reading(true, move |subsystem| {
            Box::pin(async move { subsystem.resolve(b".").await })
        })
        .await;
    match resolved {
        Ok(Ok(bytes)) => String::from_utf8(bytes).ok(),
        _ => None,
    }
}

/// **`Job files and outputs` L2's reading, per path, at the candidate's own
/// path** — through `Server protocol and transfers` §9.1's first route.
async fn the_free_space_at<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    path: &str,
) -> Option<FreeSpaceReading> {
    let at = path.to_string();
    let read = sessions
        .a_reading(true, move |subsystem| {
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
        Ok(Ok(Ok(reading))) => Some(reading),
        _ => None,
    }
}

/// **`Job files and outputs` 1.1's choice, made from `Server Setup` R6's
/// set** — the root, and `Job files and outputs` L4a's second choice where the
/// set holds more than one
/// candidate.
///
/// **Neither choice is made here and neither is re-derived.** Both are read off
/// that crate.
pub fn the_layout_this_machine_gets(
    locations: &[AWritableLocation],
) -> Result<TheLayout, WhereTheSetUpGot> {
    let set = compose_the_candidate_set(locations)
        .map_err(|why| WhereTheSetUpGot::NoCandidateCouldBeOffered { why })?;
    let set: Vec<Candidate> = set;
    let chosen = root_choice::choose(&set)
        .map_err(|why| WhereTheSetUpGot::NoRootCouldBeChosen {
            said: format!("{why:?}"),
        })?;
    Ok(the_layout_of(&chosen))
}

/// The choice, as a layout. **`Job files and outputs` L4a's two arrangements
/// and there is no third.**
fn the_layout_of(chosen: &root_choice::TheChoice) -> TheLayout {
    let recorded = chosen.what_is_recorded();
    root_choice::the_layout_again(&recorded).unwrap_or_else(|| {
        // **Unreachable by construction** — the choice composed the root — and
        // it is written rather than unwrapped so a change next door is a
        // refusal rather than a panic on the user's own machine.
        TheLayout::of(
            MachineRoot::chosen(&recorded.root_path).expect("the choice composed this root"),
            WhereThePackageAreaSits::UnderTheRoot,
        )
    })
}

/// **`Server Setup` R7's hand-over, performed, and `Server Setup` R7b's
/// recording made a step of it.**
///
/// **The recording is two writes and they are one act to the caller**: the
/// machine's own `<package area>/root`, which its supervisor reads before it
/// will accept a job, and the desktop's server row. **A machine with no root
/// recorded refuses every job in its own words** — step 10c piece 5 found that
/// on a real rental nothing anywhere wrote it.
pub async fn the_hand_over<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    row: &mut dyn KeepsTheServersRow,
    layout: &TheLayout,
    installation: &Identity,
    how: HowElevationIsObtained<'_>,
) -> Result<(), WhereTheSetUpGot> {
    let mut channel = OnThisServersPoint { sessions };
    let mut files = OnThisServersPoint { sessions };
    let placed = acts::carry_out_the_placement(&mut channel, &mut files, layout, how).await;
    if !placed.it_is_complete() {
        return Err(WhereTheSetUpGot::TheHandOverDidNotLand {
            said: what_the_hand_over_said(&placed),
        });
    }
    the_root_is_recorded(sessions, row, layout, installation).await
}

/// The machine's own words from a hand-over that did not land, whole and
/// never mined for a cause.
fn what_the_hand_over_said(placed: &acts::ThePlacement) -> String {
    // **Each failed act's own words, under the path it was for** — never a
    // debugging rendering, which is what reached the screen until 2026-09-21.
    let mut said = Vec::new();
    for one in &placed.hand_overs {
        if !one.the_ownership_landed() {
            for went in [
                &one.the_directory_was_created,
                &one.the_command,
                &one.the_landing_check,
            ] {
                if !went.it_was_carried_out() {
                    said.push(format!("{}: {}", one.the_target_path, went.in_words()));
                }
            }
        }
    }
    for (area, went) in &placed.areas {
        if !went.it_was_carried_out() {
            said.push(format!("{area}: {}", went.in_words()));
        }
    }
    said.join("\n")
}

/// **`Job files and outputs` L1's recording, in both places that clause
/// names.**
pub async fn the_root_is_recorded<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    row: &mut dyn KeepsTheServersRow,
    layout: &TheLayout,
    installation: &Identity,
) -> Result<(), WhereTheSetUpGot> {
    let recorded = what_the_hand_over_records(layout);
    let package_area = layout.area(Area::Package);
    let at = smoozy_server_protocol::placement::records::the_record_at(
        &package_area,
        smoozy_job_files::layout::THE_RECORD_OF_THE_ROOT,
    );
    // **The machine's own copy, written over the file-transfer subsystem** —
    // it is the one the supervisor reads before it will name a job's record
    // directory or stage a Blender.
    let mut files = OnThisServersPoint { sessions };
    if let Err(said) = files.write_whole(&at, recorded.root_path.as_bytes()).await {
        return Err(WhereTheSetUpGot::TheRootCouldNotBeRecorded {
            said: format!("{at}: {}", said.in_words()),
        });
    }
    // **And the desktop's, as a machine-derived value carrying the installation
    // identity it was read under** (`State, identity and recovery` §2.3).
    row.record_the_root(RecordedMachineRoot {
        path: recorded.root_path,
        the_package_areas_own_location: recorded.the_package_areas_own_location,
        read_under: installation.as_text(),
    })
    .map_err(|said| WhereTheSetUpGot::TheRootCouldNotBeRecorded { said })
}

// ════════════════════════════════════════════════════════════════════════════
// `Server Setup` R2, step 4 — the verified release
// ════════════════════════════════════════════════════════════════════════════

/// **`Server protocol and transfers` B9 and `Server protocol and transfers` B10
/// — one source, read at a time.**
///
/// **The root this build carries is the only one it will trust**
/// (`Server protocol and transfers` B9). It is handed in rather than read here
/// so a test can stand where a release stands without a second root ever
/// existing in a shipped build — which is what the runner ruled on 2026-09-19.
/// **One source, read whole, with a ceiling on what will be held.**
///
/// **A seam of this file's own rather than the fetcher's, because one release
/// read is two fetches whose sizes are nothing like each other**: the release
/// file is a few kilobytes and the artifact is the whole machine program, whose
/// length the verified manifest carries. **The ceiling on the second is that
/// length**, so a source offering more than the manifest describes is refused
/// at the moment it says so rather than after the extra bytes are in memory.
pub trait FetchesOneSource {
    fn fetch(&mut self, source: &str, ceiling: u64) -> Result<Vec<u8>, String>;
}

impl FetchesOneSource for smoozy_server_protocol::release::fetching::AFetchOverHttps {
    fn fetch(&mut self, source: &str, ceiling: u64) -> Result<Vec<u8>, String> {
        self.the_most_bytes_it_will_read = ceiling;
        self.continue_a_body(source, Vec::new(), ceiling)
            .map_err(|why| the_fetchs_words(source, &why))
    }
}

/// **`Server protocol and transfers` B9 and `Server protocol and transfers` B10
/// — the whole admission of whatever release file `from` serves, and the
/// artifact's bytes with it.** `Server protocol and transfers` B8d's rule that
/// the file be this build's own is [`this_builds_own_release`]'s, on top of
/// this.
///
/// **The root this build carries is the only one it will trust**
/// (`Server protocol and transfers` B9). It is handed in rather than read here
/// so a test can stand where a release stands **without a second root ever
/// existing in a shipped build**, which is what the runner ruled on
/// 2026-09-19.
pub fn the_current_release(
    root: &PublicKeyBytes,
    fetch: &mut dyn FetchesOneSource,
    store: &mut dyn RemembersTheTrustRecordHere,
    detected: &smoozy_server_protocol::release::manifest::TargetTuple,
    from: &str,
) -> Result<(AVerifiedRelease, Vec<u8>), String> {
    let verified = the_release_file_admitted(root, fetch, store, detected, from, None)?;
    the_artifact_of(verified, fetch, from)
}

/// **`Server protocol and transfers` B8d: this build's own release, and no
/// other** — [`the_current_release`]'s admission, with the one rule that clause adds
/// on top of it: **the manifest must name the build's own package version.**
///
/// **Every reader a user's copy runs goes through this**
/// ([`TheCurrentReleaseOverHttps`]); [`the_current_release`] is the admission
/// alone, which the desk's own hosting tables stand a release of any version
/// in front of.
///
/// **The refusal comes before anything durable moves.** The version is read off
/// the offered manifest's own bytes before `Server protocol and transfers` B9's
/// acceptance writes the trust record, so a release of another version never
/// raises this copy's floor — which is exactly how a copy could reach
/// `Server protocol and transfers` B8d's one edge case, a remembered floor
/// above its own release. Reading unverified bytes to *refuse* trusts nothing:
/// a manifest that names this version is then verified whole, and its version
/// is asked again of what verified.
///
/// **A copy whose remembered floor is above its own release refuses its own
/// manifest** at the floor in force, and that is the ordinary verification failure
/// (`Server protocol and transfers` B8d's own words); nothing is placed and its
/// update ends it.
///
/// **What it was written for, and what it now also meets.** The admission was
/// written for *whatever release is latest*, where any signed version would do.
/// It now meets a copy that must place its own release even after a newer one
/// is published — the Linux user who declined an update, or an update held
/// while jobs run (Z31).
pub fn this_builds_own_release(
    root: &PublicKeyBytes,
    fetch: &mut dyn FetchesOneSource,
    store: &mut dyn RemembersTheTrustRecordHere,
    detected: &smoozy_server_protocol::release::manifest::TargetTuple,
    from: &str,
    own_package_version: &str,
) -> Result<(AVerifiedRelease, Vec<u8>), String> {
    let verified =
        the_release_file_admitted(root, fetch, store, detected, from, Some(own_package_version))?;
    this_copys_own_release_is_known(own_package_version, verified.manifest.release_sequence);
    the_artifact_of(verified, fetch, from)
}

/// **The release file fetched, read, and admitted under
/// `Server protocol and transfers` B9** — with, where one is named, the package
/// version its manifest must carry (`Server protocol and transfers` B8d).
fn the_release_file_admitted(
    root: &PublicKeyBytes,
    fetch: &mut dyn FetchesOneSource,
    store: &mut dyn RemembersTheTrustRecordHere,
    detected: &smoozy_server_protocol::release::manifest::TargetTuple,
    from: &str,
    must_name: Option<&str>,
) -> Result<AVerifiedRelease, String> {
    let offered = fetch.fetch(from, THE_MOST_THE_RELEASE_FILE_MAY_BE)?;
    let offered = AnOfferedRelease::read(&offered)
        .ok_or_else(|| format!("{from}: not a release file."))?;
    if let Some(own) = must_name {
        // **Refused before anything durable moves.** A manifest that will not
        // read back is left to the acceptance, which says so in its own words.
        if let Ok(named) =
            smoozy_server_protocol::release::manifest::ReleaseManifest::read(&offered.the_manifests_bytes)
        {
            if named.package_version != own {
                return Err(the_wrong_versions_words(from, &named.package_version, own));
            }
        }
    }
    let mut store = store;
    let verified = accept_a_release(root, &mut store, &offered, detected, None)
        .map_err(|why| the_admissions_words(&why))?;
    if let Some(own) = must_name {
        if verified.manifest.package_version != own {
            return Err(the_wrong_versions_words(from, &verified.manifest.package_version, own));
        }
    }
    Ok(verified)
}

/// **A release file naming another version, said as that** — Rule 9's standard
/// words, and only what Smoozy knows.
fn the_wrong_versions_words(from: &str, named: &str, own: &str) -> String {
    format!(
        "{from} holds Smoozy {named}, not {own}. Not used."
    )
}

/// **`Server protocol and transfers` B10: the artifact from an exact source the
/// verified manifest named**, or the words of the last refusal.
fn the_artifact_of(
    verified: AVerifiedRelease,
    fetch: &mut dyn FetchesOneSource,
    from: &str,
) -> Result<(AVerifiedRelease, Vec<u8>), String> {
    // **`Server protocol and transfers` B10: the artifact comes from an exact
    // source the verified manifest
    // named, and from nowhere else.** Every source is tried in the manifest's
    // own order, and the first whose bytes pass the length and the digest is
    // the artifact.
    let mut last = String::new();
    for source in verified.selected.sources.clone() {
        let bytes = match fetch.fetch(&source, verified.selected.byte_length) {
            Ok(bytes) => bytes,
            Err(said) => {
                last = said;
                continue;
            }
        };
        match verify_what_was_downloaded(&verified.selected, &source, &bytes) {
            Ok(()) => return Ok((verified, bytes)),
            Err(why) => last = the_downloads_words(&source, &why),
        }
    }
    Err(if last.is_empty() {
        format!("{from}: no download for this server's platform.")
    } else {
        last
    })
}

// ════════════════════════════════════════════════════════════════════════════
// `Server protocol and transfers` B8d — this copy's own release sequence
// ════════════════════════════════════════════════════════════════════════════

/// **This copy's own release sequence, once its own manifest has verified in
/// this run** — `Server protocol and transfers` B8d's *the desktop's own release
/// sequence is that manifest's*, which `Server protocol and transfers` P3b and
/// this plan's own V11 compare a machine against.
///
/// **Run-held, keyed by the package version it was read for, and nothing new
/// is durable.** That clause keeps it *with the server-package trust record once
/// verified*, and the acceptance that yields it writes that record. **The
/// record's own greatest sequence is not read in its place**: after the desktop
/// updates itself the record still holds the previous version's sequence until
/// the new version's own manifest is accepted, so the number there is a floor
/// and not this copy's own. A copy that has not yet read its own manifest in
/// this run reads it once, the first time a comparison needs it
/// ([`this_copys_own_release_sequence`]).
fn this_copys_own_release_held() -> &'static std::sync::Mutex<Option<(String, u64)>> {
    static HELD: std::sync::OnceLock<std::sync::Mutex<Option<(String, u64)>>> =
        std::sync::OnceLock::new();
    HELD.get_or_init(|| std::sync::Mutex::new(None))
}

/// The own manifest verified: its sequence is this copy's for the run.
fn this_copys_own_release_is_known(package_version: &str, release_sequence: u64) {
    if let Ok(mut held) = this_copys_own_release_held().lock() {
        *held = Some((package_version.to_string(), release_sequence));
    }
}

/// **This copy's own release sequence where this run already knows it**, and
/// nothing where it does not.
///
/// **A development copy given a desk program answers nought and reads nothing**
/// — step 10c's transfer wave, W0-2, written into
/// `Server protocol and transfers` B10a: *a development copy that was given a
/// desk program takes that program as its own release for V11, V12 and V14.
/// Its own release sequence is the desk program's zero, which the placement
/// already records.* So a machine the shortcut placed reads equal and is never
/// updated, and a machine any published release placed reads *too new*.
pub fn this_copys_own_release_sequence_if_known(package_version: &str) -> Option<u64> {
    the_own_release_sequence_known_to(a_desk_program_is_this_copys_own_release(), package_version)
}

/// The same, with whether this copy was given a desk program handed in — so
/// W0-2's answer is driven at the desk, where the throwaway setting is the
/// process's own command line and cannot be set by a test.
fn the_own_release_sequence_known_to(a_desk_program_was_given: bool, package_version: &str) -> Option<u64> {
    // **A release build has no desk program and no route to one**
    // (`Server protocol and transfers` B10a), so the answer is compiled only
    // where the module is.
    #[cfg(debug_assertions)]
    if a_desk_program_was_given {
        return Some(crate::the_desk_program::THE_SEQUENCE_A_DESK_PROGRAM_IS_PLACED_UNDER);
    }
    #[cfg(not(debug_assertions))]
    let _ = a_desk_program_was_given;
    this_copys_own_release_held()
        .lock()
        .ok()
        .and_then(|held| held.clone())
        .filter(|(version, _)| version == package_version)
        .map(|(_, sequence)| sequence)
}

/// **Whether this copy was given a desk program** — `Server protocol and
/// transfers` B10a, a development build only. A release build compiles no route
/// to one and answers no.
pub fn a_desk_program_is_this_copys_own_release() -> bool {
    #[cfg(debug_assertions)]
    {
        crate::the_desk_program::the_one_this_copy_was_given().is_some()
    }
    #[cfg(not(debug_assertions))]
    {
        false
    }
}

/// **This copy's own release sequence, read once per run where it is not yet
/// known** — the release file alone, from this build's own address, admitted
/// under `Server protocol and transfers` B9 and held to this build's package
/// version (`Server protocol and transfers` B8d). **The artifact is not
/// fetched**: a comparison needs the manifest and nothing else.
///
/// **The artifact tuple the admission selects against is this build's own**,
/// the one tuple every release carries a program for; the machine's own tuple
/// is the placement's to match (`Server protocol and transfers` B8) and is
/// asked there.
pub fn this_copys_own_release_sequence(
    root: &PublicKeyBytes,
    fetch: &mut dyn FetchesOneSource,
    store: &mut dyn RemembersTheTrustRecordHere,
    from: &str,
    own_package_version: &str,
) -> Result<u64, String> {
    if let Some(known) = this_copys_own_release_sequence_if_known(own_package_version) {
        return Ok(known);
    }
    let verified = the_release_file_admitted(
        root,
        fetch,
        store,
        &smoozy_server_protocol::release::composing::the_tuple_this_build_is_made_for(),
        from,
        Some(own_package_version),
    )?;
    this_copys_own_release_is_known(own_package_version, verified.manifest.release_sequence);
    Ok(verified.manifest.release_sequence)
}

/// **The same, over this run's own record and the one HTTPS client**, for the
/// running application.
pub fn this_copys_own_release_sequence_here(
    record: &dyn crate::keys::LendsTheRecord,
) -> Result<u64, String> {
    if let Some(known) = this_copys_own_release_sequence_if_known(env!("CARGO_PKG_VERSION")) {
        return Ok(known);
    }
    let Some(root) = smoozy_server_protocol::release::trust::THE_RELEASE_ROOT_THIS_BUILD_CARRIES
    else {
        return Err("This build of Smoozy has no release key.".to_string());
    };
    let mut fetch = smoozy_server_protocol::release::fetching::AFetchOverHttps::reaching_the_internet(
        THE_MOST_THE_RELEASE_FILE_MAY_BE,
    );
    let mut store = TheTrustRecordThroughTheSpine { record };
    this_copys_own_release_sequence(
        &root,
        &mut fetch,
        &mut store,
        WHERE_THE_CURRENT_RELEASE_IS_READ_FROM,
        env!("CARGO_PKG_VERSION"),
    )
}

// **The three below say what happened in standard software words and name the
// address as a value** (Rule 9). They were debugging renderings until
// 2026-09-21, when the first pass on a real rental showed one on the screen.
// **Each says only what Smoozy knows** (Rule 8): which check refused, never a
// guessed cause.

fn the_fetchs_words(source: &str, why: &WhyTheFetchFailed) -> String {
    let what = match why {
        WhyTheFetchFailed::ItCouldNotBeReached { said } => format!("could not be reached: {}", a_sentence(said)),
        WhyTheFetchFailed::TheSourceIsNotAnExactHttpsAddress { .. } => {
            "is not an HTTPS address. It was not downloaded.".to_string()
        }
        WhyTheFetchFailed::TheSourceHasNoSuchThing => "was not found (HTTP 404).".to_string(),
        WhyTheFetchFailed::TheSourceAnsweredWith { status } => format!("returned HTTP {status}."),
        WhyTheFetchFailed::ItIsLargerThanTheCeiling { ceiling } => {
            format!(
                "is larger than the expected {}. The download was stopped.",
                crate::typography::a_figure_with_its_unit(ceiling, "bytes")
            )
        }
        WhyTheFetchFailed::TheResumeDidNotLineUp { said } => {
            format!("could not resume the download: {}", a_sentence(said))
        }
    };
    format!("{source} {what}")
}

fn the_admissions_words(why: &WhyTheReleaseWasNotAccepted) -> String {
    match why {
        WhyTheReleaseWasNotAccepted::TheRelease(_) => {
            "The release failed the signature check.".to_string()
        }
        WhyTheReleaseWasNotAccepted::TheStore { said } => {
            format!("Could not read or save Smoozy's list of trusted releases: {}", a_sentence(said))
        }
    }
}

fn the_downloads_words(source: &str, why: &WhyTheDownloadWasRefused) -> String {
    let what = match why {
        WhyTheDownloadWasRefused::ThatSourceIsNotOneTheManifestNamed
        | WhyTheDownloadWasRefused::ThatSourceIsNotAnHttpsAddress => {
            "is not the address in the signed release."
        }
        WhyTheDownloadWasRefused::ItsByteLengthIsNotTheOneTheManifestCarries { .. } => {
            "returned a file whose size does not match the signed release."
        }
        WhyTheDownloadWasRefused::ItsDigestIsNotTheOneTheManifestCarries => {
            "returned a file whose checksum does not match the signed release."
        }
    };
    format!("{source} {what}")
}

// ════════════════════════════════════════════════════════════════════════════
// `Server Setup` R2, step 6 — the run, driven one step at a time
// ════════════════════════════════════════════════════════════════════════════

/// **`Server Setup` R1's composition, made from what the machine was found to
/// need.**
///
/// **Nothing here is remembered from a previous sitting** (`Rented-server
/// runtime` K2): every field is this connection's own reading or the user's own
/// instruction.
///
/// **`Server Setup` Y7a item 1: every reading here is read as printed**, its
/// two streams handed to `words` under the run's first step as they arrive,
/// where a screen keeps the run's words. Added 2026-10-24, the Creator's
/// ruling on the sixteenth notes, note 1: until then the readings' words were
/// kept nowhere, so a run that installed nothing showed nothing.
pub async fn what_this_machine_needs<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    said: &WhatTheOpeningReadingsSaid,
    asked_for: &WhatWasAskedFor,
    the_machine_already_holds_a_blender: bool,
    words: Option<&TheWordsGoHere>,
) -> WhatTheMachineNeeds {
    let mut channel = the_readings_channel(sessions, words);
    // **`Server Setup` R1 item 1: card presence from the bus scan and the
    // driver's own answer, and never from the card tool's absence.** The
    // judgement is `Server Setup`'s and the two readings are `Servers`'.
    let cards = readings::read_the_cards(&mut channel, said.kind).await;
    let driver = readings::run_the_driver_check(&mut channel).await;
    // **`Server Setup` R1 item 5: a driver counts as installed only where the
    // running kernel has its module** — asked only where the check ended at 9.
    // Added 2026-09-30, step 10c's eleventh-pass fix piece 10.
    let module = readings::read_the_running_kernels_module(&mut channel, said.family, &driver).await;
    // **`Server Setup` R1 item 3's probe** — the machine's own package system
    // asked whether the unpack tool's package is on it, *"rather than
    // assumes"*. **It is taken on every run and not only on one that fetches a
    // Blender**, so the field beside it means the same thing in every run
    // rather than *not established* in some of them.
    let the_unpack_tool = readings::probe_the_unpack_tool(&mut channel).await;
    // **`Server Setup` R1 item 5's bar and version reading, added
    // 2026-10-11**: the bar from the one table, and the driver's version where
    // it can change what the step does. **Since `Server Setup` R1b
    // (2026-10-12) the bar is the newest Blender on the server's** — every
    // build the search found, and the one this run stages.
    let the_optix_bar = ready_action::the_bar_for_the_newest_blender_on_the_server(
        asked_for.install_this_blender.as_deref(),
        &asked_for.the_blenders_on_the_server,
    );
    let the_drivers_version = {
        let bare = WhatTheMachineNeeds {
            kind: said.kind,
            family: said.family,
            this_run_fetches_a_blender: false,
            the_machine_already_holds_a_blender: false,
            the_unpack_tool_is_present: true,
            the_driver_answers: false,
            a_driver_is_already_installed: false,
            cards: Cards::CouldNotBeEstablished,
            telegram_is_on: false,
            the_drivers_version: None,
            the_optix_bar,
            anything_is_live: false,
            the_driver_packages: readings::WhatTheDriverPackagesSay::NotRead,
        };
        acts::the_drivers_version_where_a_bar_applies(&mut channel, &bare, &driver).await
    };
    WhatTheMachineNeeds {
        kind: said.kind,
        family: said.family,
        this_run_fetches_a_blender: asked_for.install_this_blender.is_some(),
        the_machine_already_holds_a_blender,
        // **`Server Setup` R1 item 3's *where absent*, from the probe above.**
        // **Only an answer saying the package is installed keeps the install
        // back**; a probe that established nothing reads as absent, which is
        // the direction whose cost is one install nobody needed rather than a
        // run that cannot unpack what it fetched.
        the_unpack_tool_is_present: the_unpack_tool.the_tool_is_there(),
        the_driver_answers: matches!(driver, readings::WhatTheDriverCheckSaid::ItAnswered),
        // **`Server Setup` R1 item 5's other branch** — *"where a driver is
        // already installed, the restart is tried before any install"* — off
        // the same one check's published exit **and, since 2026-09-30, the
        // running kernel's module** (*"A driver counts as installed for this
        // rule only where the running kernel has its module"*). The whole
        // reasoning is at `Server Setup`'s
        // `readings::WhatTheDriverCheckSaid::a_driver_is_already_installed`.
        a_driver_is_already_installed: driver.a_driver_is_already_installed(module),
        cards,
        telegram_is_on: asked_for.telegram_is_on,
        the_drivers_version,
        the_optix_bar,
        // **`Server Setup` R1b's live set is read at the driver step itself**,
        // where the swap would be made, so no other run pays a reading for it.
        anything_is_live: false,
        // **`Server Setup` R1 item 5's reading of the driver's packages is
        // taken at the driver step itself**, for the same reason.
        the_driver_packages: readings::WhatTheDriverPackagesSay::NotRead,
    }
}

/// **`Server Setup` Y7's driving, in one place: a run opened, its steps
/// commanded one at a time, and its state refreshed while it stands.**
pub struct TheRunBeingDriven<'s, C: AHeldConnection> {
    sessions: &'s ThisServersSessions<C>,
    package_area: String,
    run: Identity,
    /// **`Server Setup` Y6b's kind**, which a recovery keeps (it takes the run
    /// over and carries no kind of its own). Held so the screen behind the run
    /// can read back the run of the kind it draws. Added 2026-09-21.
    kind: SetupKind,
    /// **`Server Setup` Y4's composed list**, as the desktop composed it and
    /// sent it.
    composed: Vec<Boundary>,
    /// The steps in `Server Setup` R1's order, which is what is driven.
    steps: Vec<Step>,
    /// **What this run was found to need**, as `Server Setup` R1 item 1's
    /// readings settled it and [`ready_action::compose_the_run`] composed the
    /// run from it.
    ///
    /// **It rides the run whole rather than in pieces.** `Server Setup` M1a's
    /// family, `Server Setup` M1's kind and `Server Setup` R1 item 3's probe
    /// were each carried separately, and `Server Setup` R1 item 5's judgement
    /// wants the lot — so the readings are one value here, the same one the
    /// composition was made from, and no field of it is re-decided at a step.
    needs: ready_action::WhatTheMachineNeeds,
    /// **`Server Setup` T4's mark on this server's connected period, where one
    /// stands.**
    ///
    /// **The mark is set on this handle before the restart command leaves**,
    /// and `Servers` M1's two endings are what clear it; `Server Setup` T6's
    /// wait is a wait on this and on nothing of this file's own. **Nothing
    /// where no period stands** — and then no restart is issued at all, because
    /// `Server Setup`'s own witness cannot be made without a period to mark and
    /// a restart issued unmarked is exactly Row 107's fault.
    the_period: Option<TheRestartMark>,
    /// **What `Server Setup` R1 item 5's driver step came to**, held between
    /// that step and item 6's restart because `Server Setup` Y4 puts three
    /// boundaries across the two: *driver placed*, then *restart issued*, then
    /// *card answering*.
    ///
    /// **The mark is the driver step's** (`Server Setup` Y4a marks a step and
    /// not a boundary), so both of the driver step's own boundaries carry it.
    /// **Whether the check answers is what decides item 6** — *"only where step
    /// 5's check failed after its work"* — and it is the check taken after the
    /// step rather than anything remembered.
    the_driver_step: Option<TheDriverStepsOwnAnswer>,
    /// **`Server Setup` R1 item 5: whether the driver's last check found it
    /// answering below the OptiX bar of the Blender this run installs.**
    /// Added 2026-10-11. Set where the last check is taken — the driver step
    /// where no restart follows, the check on return where one does — and
    /// carried onto *done*.
    the_driver_missed_the_optix_bar: bool,
    /// **`Server Setup` R1b: the swap was owed and left out of this run for
    /// live work.** Added 2026-10-12, carried onto *done*.
    the_swap_waits: bool,
    /// **`Server Setup` R1c: the server whose driver holds this run's item 5
    /// is told to** ([`crate::servers::the_driver_swap::item_5_compared_and_acted`]),
    /// set at the driver step from [`WhatWasAskedFor::the_server`]. Nothing
    /// where the caller named no server, and then nothing is told. Added
    /// 2026-10-12, held-job piece A.
    the_server_item_5_is_told_for: Option<String>,
    /// **Whether `Server Setup` Y1's machine-level setup-in-progress state
    /// stands for this run**, because this driving set it.
    ///
    /// **`Server Setup` Y1 decides the class by the act a step performs, never
    /// by the step's number**, and `Server Setup` Y7 puts every elevated act on
    /// the desktop — so **the one side that can know an act is about to be
    /// performed is the side about to perform it**, and this is set immediately
    /// before the first act [`AnElevatedStep`] classes as machine-changing.
    /// `Server Setup`'s own `elevated::WhatAnActDoesToTheMachine` carries that
    /// judgement at each command, so nothing here classes an act a second time.
    ///
    /// **It is carried across `Server Setup` S4 item 2's take-over**, because
    /// the machine re-points the standing state at the recovery run in the
    /// take-over's own durable act and never clears it in between.
    ///
    /// **What it is for is the clear.** `Server Setup` S6 closes the clearing
    /// set at two and `Server Setup` Y4's *done* row is the one this driving
    /// reaches, so the clear goes out at *done* and nowhere else. **A run that
    /// ends anywhere earlier leaves the state standing**, which is
    /// `Rented-server runtime` P7 and `Server Setup` Y8b working rather than
    /// something left undone.
    the_state_stands: bool,
    /// **The last boundary this side saw the run stand at**, kept only so that
    /// a run whose machine has stopped answering is not reported as standing
    /// where it began.
    ///
    /// **The machine's record is the authority and this is never preferred to
    /// it** — it is read whenever the machine answers, and this is what is said
    /// when it does not. A restart is exactly that case: the run ends while the
    /// machine is down, and `Server Setup` Y4's *restart issued* is what it
    /// reached. **Saying *started* there would be a claim about the run that is
    /// false**, which `Server Setup` R4 and W1 forbid as much as any other.
    the_last_one_seen: Option<Boundary>,
    /// **Whether `Server Setup` Y8's stop has been written as the machine's own
    /// durable desired state for this run**, so it is written once.
    the_stop_was_written: bool,
    /// **Where a recovery picks the restart step up, when it resumes between
    /// the driver step's two boundaries.** Added 2026-09-27 by the runner's
    /// ruling; nothing on a fresh run. See [`HowTheRestartStepResumes`].
    the_restart_step_resumes: Option<HowTheRestartStepResumes>,
    /// **`Server Setup` B7: the bare name of the archive the user uploaded**,
    /// carried on the unpack and the publication of a run that installs the
    /// user's own build. Nothing on every other run. Added 2026-10-09, the
    /// 38's piece 14b.
    the_users_own_archive: Option<String>,
    /// **`Server Setup` B7a's question, as the last state reading held it** —
    /// set where the publication came back held for a person.
    the_question: Option<String>,
    /// **The user's answer to it**, carried on the publication commanded again.
    the_users_answer: Option<&'static str>,
    /// **The version the user's build answered, as the last state reading
    /// that carried one held it** — what `Server Setup` §12's *a Blender was
    /// installed* names for a build the user pointed at. Added 2026-10-16,
    /// step 10c's owed piece 5.
    the_version_answered: Option<String>,
    /// **`Server Setup` B15: what a removal run removes** — carried on its
    /// removal step and nothing else. Nothing on every other run. Added
    /// 2026-10-10, the 38's piece 14c.
    the_removal: Option<TheRemovalAsked>,
    /// **`Server Setup` Y7a: where what this run's commands print goes** —
    /// the screen's, asked once per run
    /// ([`AsksForThePassword::where_the_words_go`]), before the readings where
    /// the run is composed here. Nothing where no screen keeps them. Added
    /// 2026-10-18, the Creator's thirteenth notes, piece H; every step's words
    /// from 2026-10-24, the sixteenth notes, note 1.
    the_words_go: Option<TheWordsGoHere>,
    /// **`Server Setup` Y7b and `Server Setup` Y7c: where how far the download and the unpack
    /// have got goes, each time the run is read while one of them runs** — the
    /// screen's, asked once as the run is driven
    /// ([`AsksForThePassword::where_the_figure_goes`]). Nothing where no
    /// screen draws it. Added 2026-10-18, the Creator's fourteenth notes,
    /// piece U.
    the_figure_goes: Option<TheFigureGoesHere>,
    /// **`Server Setup` Y7d: which of the two cadences the next refresh
    /// waits** — the quick one while words move, the keepalive one after five
    /// quiet readings. A run opens at the quick one. Added 2026-10-25, the
    /// eighteenth notes, piece 2c.
    the_cadence: driving::TheRefreshCadence,
    /// **The run's record as the last state reading carried it, while nothing
    /// has been written to the run since** — so the screen is told the next
    /// step from it rather than from a reading taken again. **The state reading
    /// and the record are one request** ([`a_runs_record`]). Let go at every
    /// write to the run and every step commanded. Added 2026-10-26, the
    /// worldwide pass's setup faults, fault 4: each step began with a second
    /// reading of what the last had just read, which at 300 ms each way was
    /// 3.6 seconds a step.
    the_record_as_last_read: Option<TheSetupAsRead>,
}

/// **The removal step's own inputs, on the command** — the executable, the
/// answered version where one was read, and `Server Setup` B14's covered jobs as a count and
/// its items, the one set the warning named (`Queue & History` L21). The 38's
/// piece 14c.
pub fn the_removal_steps_fields(mut fields: Fields, removal: &TheRemovalAsked) -> Fields {
    fields = fields
        .and(asked::THE_EXECUTABLE_FIELD, removal.executable.clone())
        .and(asked::HOW_MANY_COVERED_JOBS_FIELD, removal.covered.len().to_string());
    if !removal.the_answered_version.is_empty() {
        fields = fields.and(asked::THE_BLENDER_VERSION_FIELD, removal.the_answered_version.clone());
    }
    for (at, job) in removal.covered.iter().enumerate() {
        fields = fields.and(
            &asked::the_field_of_item(asked::A_COVERED_JOB, at, asked::THE_JOB_FIELD),
            job.as_text(),
        );
    }
    // **`Server Setup` B15's second route**: the manager's package, so the
    // machine reads the removal rather than deleting anything. Step 10c's owed
    // piece 6.
    if let Some(package) = removal.the_managers_package {
        fields = fields.and(asked::THE_MANAGERS_PACKAGE_FIELD, package);
    }
    fields
}

/// **`Server Setup` B13 to `Server Setup` B15: what one removal is asked to remove.** The 38's
/// piece 14c.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheRemovalAsked {
    /// **`Server Setup` B13: the build, by its executable's path.**
    pub executable: String,
    /// **What the build answered**, which `Queue & History` L21's mark holds
    /// and `Queue & History` L22's drop names. Empty where the table read none.
    pub the_answered_version: String,
    /// **`Server Setup` B14's covered jobs** — the one set the warning named, composed once
    /// (`Queue & History` L21).
    pub covered: Vec<Identity>,
    /// **`Server Setup` B15's second route: the package the machine's own
    /// software manager holds the build under**, where the build is that
    /// manager's — `Server Setup`'s own `blender::the_managed_package_at`.
    /// **Nothing is the plain deletion.** Present, the removal step sends that
    /// manager's own removal, elevated, and the machine then reads the build
    /// gone. Added 2026-10-16, step 10c's owed piece 6.
    pub the_managers_package: Option<&'static str>,
}

/// **Where a recovery picks `Server Setup` R1 item 6 up, between the driver
/// step's *driver placed* and its *card answering*.** Added 2026-09-27, the
/// runner's ruling 2 of that day.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HowTheRestartStepResumes {
    /// **The marker is at *driver placed*: the driver step finished, and the
    /// restart step runs afresh** — `Server Setup` S4 item 3, and §4.3's
    /// restart bullet: *"A recovery resuming at the restart step re-runs step
    /// 5's check first and issues the restart only where it fails again."*
    ///
    /// **`the_mark_is_already_in_this_record`** is the one case the machine's
    /// take-over makes awkward: a refused restart's *restart issued* was taken
    /// back by the ruling's reading, but the machine's take-over copied the
    /// reached list whole, so the mark stands in the recovery's own record and
    /// the machine refuses it written a second time (`Server Setup` Y3's rule at
    /// the write). **It is not written again**; it is already durable before
    /// the command, which is all `Server Setup` T3 asks.
    Afresh { the_mark_is_already_in_this_record: bool },
    /// **The marker is at *restart issued*: the restart went out and the setup
    /// carries on at the check** — `Server Setup` T5, *"carries on at the
    /// check"*, from a press rather than from the wait.
    AtTheCheck,
}

/// **What `Server Setup` R1 item 5 came to on this run**, as its own two
/// boundaries and item 6 read it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TheDriverStepsOwnAnswer {
    /// `Server Setup` Y4a's mark for the step: **performed** where the
    /// distribution's own route went out, **nothing to do** where the check
    /// answered first and nothing was installed or touched.
    mark: WhatTheStepDid,
    /// **`Server Setup` R4's claim and no more**: the driver answers — **and,
    /// from 2026-10-11, after a swap that went through, answers at or above
    /// the OptiX bar** (`Server Setup` R1 item 5's *the check after it passes
    /// only where the driver answers at or above the bar*). It is what decides
    /// item 6's restart.
    the_driver_answers: bool,
}

/// **What a setup run is opened as, and the two are exclusive by the machine's
/// own rule.**
///
/// **A recovery carries the run it takes over and no kind** — `Server Setup`
/// Y6b gives it the taken-over run's kind, and the machine's own dispatch
/// refuses a request carrying both because two fields could disagree.
#[derive(Debug, Clone)]
enum WhatOpensTheRun {
    /// **A fresh run of one of `Server Setup` Y6b's five kinds.**
    AFreshRun(SetupKind),
    /// **`Server Setup` S4 item 2's recovery, naming the setup operation it
    /// takes over.**
    ///
    /// **The take-over is the machine's**: it validates the recorded group,
    /// writes the taken-over run's recorded outcome, re-points the machine-level
    /// state at this run and writes the inherited boundary list and marker into
    /// this run's own record, all in one durable act — *"the take-over is
    /// required, not stylistic"*. **So what this side sends is the identity and
    /// nothing else about where to resume.**
    ARecoveryOf(Identity),
    /// **`Server Setup` S6's second clear: a recovery of the named run that the
    /// user asked to abandon the machine-changing work.** Added 2026-09-22. It
    /// takes the state over exactly as [`WhatOpensTheRun::ARecoveryOf`] does,
    /// and says so beside it.
    AnAbandonOf(Identity),
}

/// **`Server Setup` Y1: one setup-kind operation, one operation record written
/// before its worker starts, one recorded execution group** — opened over the
/// command route.
///
/// **The request's own operation identity is the run's** — `Server protocol and
/// transfers` C4 records it because the request starts a worker, and that
/// identity is what `Rented-server runtime` P1 calls the setup run's operation
/// identity.
async fn a_run_is_opened<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    package_area: &str,
    opens_it: WhatOpensTheRun,
    composed: &[Boundary],
    stop: &dyn AsksToStop,
    asker: &mut dyn AsksForThePassword,
) -> Result<Identity, TheRunsEnding> {
    // **`Server Setup` S5's wait, for *Set up again* and *Abandon setup*
    // alike.** A recovery the machine refuses because the run that stopped is
    // still running *"costs a wait, never a second run"*, and
    // `Design/Server Setup § The two ways out` promises that *"either one waits
    // if the server has not finished with the run that stopped"*. **The machine
    // says which refusal it is in a field of its own**, so the opening is asked
    // again at `Server Setup` Y7's cadence until it is admitted, the user
    // stops, the machine stops answering, or
    // [`HOW_LONG_THE_RUN_THAT_STOPPED_IS_WAITED_FOR`] passes. A program placed
    // before 2026-09-22 never sends the field, and its refusal is shown in its
    // own words as before. Added 2026-09-22.
    let waited = std::time::Instant::now();
    let mut waiting = false;
    let result = loop {
        // **`Server Setup` Y8: a stop on a run must be recorded by its
        // driver.** Only a stop during this wait ends before a run is opened.
        if waiting && stop.the_user_asked_to_stop() {
            break Err(TheRunsEnding::TheUserStoppedIt);
        }
        match a_run_is_opened_once(sessions, package_area, &opens_it, composed).await {
            Err(TheOpeningWasRefused::TheRunThatStoppedIsStillRunning) => {
                if !waiting {
                    asker.the_cut_run_is_being_waited_for(true);
                    waiting = true;
                }
                if waited.elapsed() > HOW_LONG_THE_RUN_THAT_STOPPED_IS_WAITED_FOR {
                    break Err(TheRunsEnding::NoRunCouldBeOpened {
                        said: crate::surface::set_up::words::THE_SERVER_IS_FINISHING_THE_CUT_RUN.to_string(),
                    });
                }
                wait_a_moment(driving::THE_REFRESH_CADENCE).await;
            }
            Err(TheOpeningWasRefused::ForGood(ending)) => break Err(ending),
            Ok(run) => break Ok(run),
        }
    };
    if waiting {
        asker.the_cut_run_is_being_waited_for(false);
    }
    result
}

/// **How long `Server Setup` S5's wait lasts before it says so.**
///
/// **Long enough for the run that stopped to end by its own rules**: a stopped
/// run's worker ends at its next boundary, a run nobody drives is ended by
/// `Server Setup` Y7's window, and a setup worker given nothing to do stops
/// itself after thirty minutes (the machine program's own
/// `HOW_LONG_A_WORKER_WAITS_WITH_NOTHING_TO_DO`). **Past that, a run that still
/// reads as live is `Server Setup` S5's defunct-entry case**, which no wait
/// here reaches, and the refusal is shown in the machine's own words.
pub const HOW_LONG_THE_RUN_THAT_STOPPED_IS_WAITED_FOR: Duration = Duration::from_secs(35 * 60);

/// **Smoozy's own record of why a recovery was not carried on**, written as
/// the failed step's words (`Server Setup` S2a: *"the recorded fact of that
/// read is the ending's content"*). The user reads it on the stopped-setup
/// panel. Added 2026-09-26. Shortened 2026-10-28 (the loose ends after the
/// words sweep). **Shortened again 2026-11-06, the check after the last fix**:
/// the panel's own lead above it already says *Cannot resume mid-step.*, so
/// these words say only what stopped. They are shown verbatim, past the
/// typography point, so they hold no apostrophe.
pub const THE_RECOVERY_WAS_NOT_CARRIED_ON: &str = "The previous setup stopped mid-step.";

/// **The same record as written from 2026-10-28 to 2026-11-06.** A server may
/// still hold it as a failed step's words, so it is matched as well. Never
/// changed.
pub const THE_RECOVERY_WAS_NOT_CARRIED_ON_UNTIL_2026_11_06: &str =
    "This run took over a setup that stopped mid-step. A step is not resumed mid-step, so the \
     setup was not continued.";

/// **The same record as written before 2026-10-28.** A server may still hold
/// it as a failed step's words, so it is matched as well. Never changed.
pub const THE_RECOVERY_WAS_NOT_CARRIED_ON_EARLIER: &str =
    "The setup this run took over had stopped part-way through this step. Smoozy does not \
     restart a step part-way through, so it did not carry the setup on.";

/// **Whether a failed step's words are Smoozy's record that a recovery was
/// not carried on**, in any of its three wordings.
pub fn the_recovery_was_not_carried_on(words: &str) -> bool {
    words == THE_RECOVERY_WAS_NOT_CARRIED_ON
        || words == THE_RECOVERY_WAS_NOT_CARRIED_ON_UNTIL_2026_11_06
        || words == THE_RECOVERY_WAS_NOT_CARRIED_ON_EARLIER
}

/// **Why one opening was refused**, told apart only where the machine said so
/// in its own field.
enum TheOpeningWasRefused {
    /// `Server Setup` S5: the run a recovery would take over is still running.
    TheRunThatStoppedIsStillRunning,
    /// Every other refusal, as the run's ending.
    ForGood(TheRunsEnding),
}

/// **One opening, asked once.**
async fn a_run_is_opened_once<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    package_area: &str,
    opens_it: &WhatOpensTheRun,
    composed: &[Boundary],
) -> Result<Identity, TheOpeningWasRefused> {
    let Ok(run) = Identity::issue(IdentityKind::Operation) else {
        return Err(TheOpeningWasRefused::ForGood(TheRunsEnding::NoRunCouldBeOpened {
            said: "Could not generate a run ID.".to_string(),
        }));
    };
    let request = Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        run.clone(),
        asked::THE_SETUP_RUN_IS_OPENED_REQUEST,
    );
    let mut fields = Fields::none().and(
        asked::HOW_MANY_COMPOSED_BOUNDARIES_FIELD,
        composed.len().to_string(),
    );
    fields = match opens_it {
        WhatOpensTheRun::AFreshRun(kind) => {
            fields.and(asked::THE_KIND_OF_SETUP_FIELD, the_word_for(*kind))
        }
        WhatOpensTheRun::ARecoveryOf(taken_over) => {
            fields.and(asked::THE_RUN_TAKEN_OVER_FIELD, taken_over.as_text())
        }
        WhatOpensTheRun::AnAbandonOf(taken_over) => fields
            .and(asked::THE_RUN_TAKEN_OVER_FIELD, taken_over.as_text())
            .and(asked::THE_RECOVERY_ABANDONS_FIELD, asked::THE_RECOVERY_ABANDONS),
    };
    for (at, boundary) in composed.iter().enumerate() {
        fields = fields.and(
            &asked::the_field_of_item(asked::A_COMPOSED_BOUNDARY, at, asked::THE_BOUNDARY_FIELD),
            the_word_for_boundary(*boundary),
        );
    }
    let request = request.carrying(fields);
    let answered = match ask(sessions, package_area, &request).await {
        Some(answered) => answered,
        None => {
            return Err(TheOpeningWasRefused::ForGood(TheRunsEnding::NoRunCouldBeOpened {
                said: "Smoozy's service on the server did not respond.".to_string(),
            }))
        }
    };
    if !it_is_still_in_hand(&answered) {
        if answered
            .fields
            .text(asked::THE_RUN_THAT_STOPPED_IS_STILL_RUNNING_FIELD)
            .is_some()
        {
            return Err(TheOpeningWasRefused::TheRunThatStoppedIsStillRunning);
        }
        // **`Server Setup` Y6: the refusal names the run the standing state
        // holds, in its own field**, and that is what a recovery takes over.
        if let Some(standing) = answered
            .fields
            .text(asked::THE_SETUP_IN_PROGRESS_FIELD)
            .and_then(|text| Identity::read_back(IdentityKind::Operation, text))
        {
            return Err(TheOpeningWasRefused::ForGood(
                TheRunsEnding::ASetupThatStoppedPartWayStands {
                    standing,
                    said: the_machines_own_words(&answered),
                },
            ));
        }
        return Err(TheOpeningWasRefused::ForGood(TheRunsEnding::NoRunCouldBeOpened {
            said: the_machines_own_words(&answered),
        }));
    }
    // **An abandon the machine did not say back was not honoured as one.** A
    // program placed before 2026-09-22 reads the recovery and not the mark, and
    // would run it as an ordinary recovery that never clears the state — so the
    // abandon stops here, in words, rather than waiting on an ending that
    // program never writes. The run it opened is ended by `Server Setup` Y7's
    // window like any run nobody drives.
    if matches!(opens_it, WhatOpensTheRun::AnAbandonOf(_))
        && answered.fields.text(asked::THE_RECOVERY_ABANDONS_FIELD)
            != Some(asked::THE_RECOVERY_ABANDONS)
    {
        return Err(TheOpeningWasRefused::ForGood(TheRunsEnding::NoRunCouldBeOpened {
            said: "Smoozy's server software is too old to abandon a setup; it ran a recovery \
                   instead. Nothing was cleared."
                .to_string(),
        }));
    }
    Ok(run)
}

impl<'s, C: AHeldConnection> TheRunBeingDriven<'s, C> {
    /// **`Server Setup` Y6b's kind, and the run opened over the command
    /// route.**
    ///
    /// **The request's own operation identity is the run's** — `Server protocol
    /// and transfers` C4 records it because the request starts a worker, and
    /// that identity is what `Rented-server runtime` P1 calls the setup run's
    /// operation identity.
    #[allow(clippy::too_many_arguments)]
    pub async fn opened(
        sessions: &'s ThisServersSessions<C>,
        package_area: &str,
        kind: SetupKind,
        steps: Vec<Step>,
        needs: ready_action::WhatTheMachineNeeds,
        the_period: Option<TheRestartMark>,
        the_runs_this_copy_drove_here: Vec<Identity>,
        stop: &dyn AsksToStop,
        asker: &mut dyn AsksForThePassword,
    ) -> Result<TheRunBeingDriven<'s, C>, TheRunsEnding> {
        Self::opened_with(
            sessions,
            package_area,
            kind,
            steps,
            needs,
            the_period,
            the_runs_this_copy_drove_here,
            stop,
            true,
            asker,
        )
        .await
    }

    /// **The same, saying whether the run may take a stopped setup over.**
    /// Added 2026-10-12 for `Server Setup` R1b's swap run, which is of the
    /// ready action's kind and **takes nothing over**: a run nobody pressed
    /// for never carries on a setup the user left standing — that is the
    /// user's own *Set up again* (`Server Setup` S3, S4). Where one stands,
    /// the swap run is refused as any fresh run is, and nothing is done.
    #[allow(clippy::too_many_arguments)]
    async fn opened_with(
        sessions: &'s ThisServersSessions<C>,
        package_area: &str,
        kind: SetupKind,
        steps: Vec<Step>,
        needs: ready_action::WhatTheMachineNeeds,
        the_period: Option<TheRestartMark>,
        the_runs_this_copy_drove_here: Vec<Identity>,
        stop: &dyn AsksToStop,
        it_may_take_a_setup_over: bool,
        asker: &mut dyn AsksForThePassword,
    ) -> Result<TheRunBeingDriven<'s, C>, TheRunsEnding> {
        let composed = compose_the_boundary_list(&steps);
        // **`Server Setup` S3's other half: *or when the user runs the setup
        // again*, for a run that stopped and set no machine-level state.**
        // Added 2026-09-22, step 10c's fourth pass on Vast, fault 6.
        //
        // **`Server Setup` Y1's additive class sets no state**, deliberately —
        // a Blender install on a machine that already holds the unpack tool
        // changes nothing reconciliation or admission must be held back
        // against. So nothing refuses the next press, the refusal route below
        // never fires, and the stopped run was simply forgotten with the
        // archive it had fetched still on the machine's disk.
        //
        // **What is taken over is the taken-over run's own run** —
        // `Server Setup` S4 item 4: the recovery drives that run's remaining
        // steps in its inherited list, with the Blender version and the name
        // its staged material sits under, and not the steps of the press that
        // opened it. So the fetched archive is used rather than fetched again.
        //
        // **Only where no state stands**, because a state that stands names the
        // one run a recovery may take over (`Server Setup` S4 item 2), and the
        // refusal route below is what finds it.
        let taken_over = match kind {
            SetupKind::TheReadyAction if it_may_take_a_setup_over => {
                the_run_that_stopped_and_left_no_state(
                    sessions,
                    package_area,
                    &the_runs_this_copy_drove_here,
                )
                .await
            }
            _ => None,
        };
        if let Some(earlier) = taken_over {
            let run = a_run_is_opened(
                sessions,
                package_area,
                WhatOpensTheRun::ARecoveryOf(earlier.clone()),
                &composed,
                stop,
                asker,
            )
            .await?;
            let mut driven = TheRunBeingDriven {
                sessions,
                package_area: package_area.to_string(),
                run,
                kind,
                composed,
                steps,
                needs,
                the_period,
                the_driver_step: None,
                the_driver_missed_the_optix_bar: false,
                the_swap_waits: false,
                the_server_item_5_is_told_for: None,
                // **The taken-over run set no state, so none stands**
                // (`Server Setup` Y1): the machine's take-over re-points a
                // state only where one was there to re-point.
                the_state_stands: false,
                the_last_one_seen: None,
                the_stop_was_written: false,
                the_restart_step_resumes: None,
                the_users_own_archive: None,
                the_question: None,
                the_users_answer: None,
                the_version_answered: None,
                the_removal: None,
                the_words_go: None,
                the_figure_goes: None,
                the_cadence: driving::TheRefreshCadence::as_a_run_opens(),
                the_record_as_last_read: None,
            };
            driven.the_recovery_resumes_or_is_ended(&earlier).await?;
            return Ok(driven);
        }
        // **`Server Setup` S3 and `Server Setup` S4 item 2: where a setup that did not finish
        // still holds the machine, running the ready action again is the
        // recovery run that takes it over** — *"issued by the desktop … when the
        // user runs the setup again"*. Added 2026-09-21, step 10c: the second
        // pass on Hyperstack found a failed run's state refusing every later
        // setup through a restart, because the press only ever opened a fresh
        // run and the machine rightly refused it (`Rented-server runtime` A5b and
        // `Rented-server runtime` A11).
        let (run, taken_over) = match a_run_is_opened(
            sessions,
            package_area,
            WhatOpensTheRun::AFreshRun(kind),
            &composed,
            stop,
            asker,
        )
        .await
        {
            Ok(run) => (run, None),
            Err(TheRunsEnding::ASetupThatStoppedPartWayStands { standing, .. })
                if kind == SetupKind::TheReadyAction && it_may_take_a_setup_over =>
            {
                let run = a_run_is_opened(
                    sessions,
                    package_area,
                    WhatOpensTheRun::ARecoveryOf(standing.clone()),
                    &composed,
                    stop,
                    asker,
                )
                .await?;
                (run, Some(standing))
            }
            Err(ending) => return Err(ending),
        };
        let mut driven = TheRunBeingDriven {
            sessions,
            package_area: package_area.to_string(),
            run,
            kind,
            composed,
            steps,
            needs,
            the_period,
            the_driver_step: None,
            the_driver_missed_the_optix_bar: false,
            the_swap_waits: false,
            the_server_item_5_is_told_for: None,
            // **`Server Setup` Y4's *started* row: no machine-level state is
            // set yet, and the machine is untouched** — on a fresh run. **A
            // recovery takes the standing state over** (`Server Setup` S4 item
            // 2), so on one it stands, and only *done* clears it
            // (`Server Setup` S6).
            the_state_stands: taken_over.is_some(),
            the_last_one_seen: None,
            the_stop_was_written: false,
            the_restart_step_resumes: None,
            the_users_own_archive: None,
            the_question: None,
            the_users_answer: None,
            the_version_answered: None,
            the_removal: None,
            the_words_go: None,
            the_figure_goes: None,
            the_cadence: driving::TheRefreshCadence::as_a_run_opens(),
            the_record_as_last_read: None,
        };
        if let Some(taken_over) = taken_over {
            driven.the_recovery_resumes_or_is_ended(&taken_over).await?;
        }
        Ok(driven)
    }

    /// **A recovery this side opened and will not carry on is ended on the
    /// machine before it is given up.** Added 2026-09-26, out of step 10c's
    /// fifth pass on Hyperstack.
    ///
    /// **What the pass found.** After Smoozy's own restart, *Set up this
    /// server* opened a recovery; the machine took the setup over from *driver
    /// placed*; this side then refused to carry it on
    /// ([`TheRunsEnding::ItStoppedInsideAStep`]) and returned, **leaving the
    /// recovery open with no ending**. The stopped-setup panel draws only over a
    /// run that ended, so *Abandon setup* was on no screen, and every later
    /// press did the same. **Destroying the rental was the only way out.**
    ///
    /// **How it is ended.** Where the refusal names a step, by `Server Setup`
    /// S2a's desk half: the failed step and Smoozy's own record of why, in one
    /// durable act that the machine performs at once — **not** `Server Setup`
    /// Y8's stop, which ends a run only when its worker next asks, and **not**
    /// left to `Server Setup` Y7's window, which the screen's own readings keep
    /// from elapsing. Where it names none — the machine did not say how far the
    /// setup had got — the stop is written, because no step can be named
    /// truthfully. **Neither clears the machine-level state** (`Server Setup`
    /// S6), so the panel's two ways out are what follows.
    async fn the_recovery_resumes_or_is_ended(
        &mut self,
        taken_over: &Identity,
    ) -> Result<(), TheRunsEnding> {
        let Err(ending) = self.the_steps_resume_after_the_inherited_marker(taken_over).await else {
            return Ok(());
        };
        match &ending {
            TheRunsEnding::ItStoppedInsideAStep { step } => {
                self.the_failed_step_is_recorded(*step, THE_RECOVERY_WAS_NOT_CARRIED_ON)
                    .await
            }
            _ => self.the_stop_is_written().await,
        }
        Err(ending)
    }

    /// **`Server Setup` S4 item 3: *"The recovery run resumes at the recorded
    /// boundary and never before it. Steps at or before the marker are not run
    /// again"*.**
    ///
    /// **The machine's take-over wrote the inherited list and marker into this
    /// run's own record**, so they are read back from there rather than
    /// composed again here. **`Server Setup` S4 item 4: the steps driven are
    /// the inherited run's own steps after the marker, and not this press's** —
    /// `Server Setup` Y6b gives
    /// the recovery the taken-over run's kind, so a recovery carries that run
    /// on. **A step with a boundary on each side of the marker is not guessed
    /// at** — see [`TheRunsEnding::ItStoppedInsideAStep`].
    ///
    /// *(Changed 2026-09-22, out of step 10c's third pass on Vast. This kept
    /// only the steps this press had composed itself, so *Set up this server*
    /// taking over a Blender install stopped after *fetched* kept *done* alone,
    /// dropped the unpack, the libraries and the publication without a word,
    /// ran the final verification and wrote *done* — and the screen said the
    /// server was ready to render with no Blender on it.)*
    async fn the_steps_resume_after_the_inherited_marker(
        &mut self,
        taken_over: &Identity,
    ) -> Result<(), TheRunsEnding> {
        let Some((state, recorded)) =
            a_runs_record(self.sessions, &self.package_area, &self.run).await
        else {
            return Err(TheRunsEnding::NoRunCouldBeOpened {
                said: "The server did not report the earlier setup's progress.".to_string(),
            });
        };
        if let Some(kind) = state.kind {
            self.kind = kind;
        }
        // **A machine that says nothing of the inherited list** leaves this
        // press's own steps, filtered by what it did say, which is how every
        // recovery was driven before 2026-09-22.
        let the_steps_in_the_list: Vec<Step> = if state.composed.is_empty() {
            self.steps.clone()
        } else {
            self.composed = state.composed.clone();
            Step::IN_ORDER
                .into_iter()
                .filter(|step| self.composed.iter().any(|boundary| boundary.its_step() == *step))
                .collect()
        };
        // **The runner's ruling 1 of 2026-09-27, read through the take-over.**
        // The machine's take-over copied the taken-over run's reached list into
        // this run's own record, and the failure that takes a refused
        // restart's mark back is on the taken-over run's record and not on
        // this one. **So the taken-over run is read**, and where its ending
        // names the restart step, *restart issued* is not a boundary this
        // recovery stands behind — however this run's own record reads.
        let in_this_record = state.reached.clone();
        let mut stood_behind = in_this_record.clone();
        let the_mark_was_taken_back =
            match a_runs_record(self.sessions, &self.package_area, taken_over).await {
                Some((earlier, _)) => the_restart_mark_was_taken_back(
                    earlier.ending.as_deref(),
                    earlier.the_failed_step,
                ),
                None => false,
            };
        if the_mark_was_taken_back {
            stood_behind.retain(|boundary| *boundary != Boundary::RestartIssued);
        }
        let where_in_order =
            |boundary: Boundary| Boundary::IN_ORDER.iter().position(|it| *it == boundary);
        let marker = stood_behind.last().copied().and_then(where_in_order);
        let reached = |boundary: Boundary| match (where_in_order(boundary), marker) {
            (Some(at), Some(marker)) => at <= marker,
            _ => false,
        };
        let mut kept = Vec::new();
        for step in the_steps_in_the_list {
            // **The restart step a recovery picks up between the driver's two
            // boundaries is driven whatever its own boundary reads**, because
            // [`HowTheRestartStepResumes::AtTheCheck`] is its check and that is
            // exactly what is left to do.
            if step == Step::Restart && self.the_restart_step_resumes.is_some() {
                kept.push(step);
                continue;
            }
            let its: Vec<Boundary> = self
                .composed
                .iter()
                .copied()
                .filter(|boundary| boundary.its_step() == step)
                .collect();
            let done = its.iter().filter(|boundary| reached(**boundary)).count();
            match (its.len(), done) {
                // **Not in the inherited list**: the machine composed that run
                // without this step, and a boundary for it would be refused.
                (0, _) => {}
                (all, done) if done == all => {}
                (_, 0) => kept.push(step),
                // **The runner's ruling 2 of 2026-09-27: a marker at *driver
                // placed* means the driver step finished.** Its second boundary,
                // *card answering*, is the check after the restart, and the
                // restart step is what carries it — afresh where *restart
                // issued* is not stood behind, at the check where it is.
                // **Only where the list holds *restart issued* between the
                // driver's two boundaries**, which is what makes the marker a
                // boundary between two steps rather than one inside a step.
                _ if step == Step::Driver
                    && self.composed.contains(&Boundary::RestartIssued)
                    && reached(Boundary::DriverPlaced)
                    && !reached(Boundary::CardAnswering) =>
                {
                    // **`Server Setup` Y4a's mark is the driver step's**, and
                    // *card answering* carries it too, so it is read off the
                    // record rather than guessed.
                    let mark = recorded
                        .as_ref()
                        .and_then(|read| {
                            read.record
                                .the_boundaries_reached
                                .iter()
                                .find(|it| it.boundary == Boundary::DriverPlaced)
                                .map(|it| it.what_the_step_did)
                        })
                        .unwrap_or(WhatTheStepDid::Performed);
                    self.the_driver_step = Some(TheDriverStepsOwnAnswer {
                        mark,
                        the_driver_answers: false,
                    });
                    self.the_restart_step_resumes = Some(if reached(Boundary::RestartIssued) {
                        HowTheRestartStepResumes::AtTheCheck
                    } else {
                        HowTheRestartStepResumes::Afresh {
                            the_mark_is_already_in_this_record: in_this_record
                                .contains(&Boundary::RestartIssued),
                        }
                    });
                }
                _ => return Err(TheRunsEnding::ItStoppedInsideAStep { step }),
            }
        }
        self.steps = kept;
        self.the_last_one_seen = stood_behind.last().copied();
        Ok(())
    }

    /// The run's own operation identity.
    pub fn run(&self) -> &Identity {
        &self.run
    }

    /// `Server Setup` Y4's composed list, as it was sent.
    pub fn composed(&self) -> &[Boundary] {
        &self.composed
    }

    /// **The steps, driven in `Server Setup` R1's order.**
    ///
    /// **Two sides and they are driven differently** (`Server Setup` Y7):
    /// a worker step is commanded and waited on at the run's own state reading;
    /// an elevated step is performed here over the command channel by the
    /// machine's own elevating command.
    pub async fn driven(
        &mut self,
        elevation: HowElevationIsObtained<'_>,
        asked_for: &WhatWasAskedFor,
        telegram: &mut dyn ComposesTheTelegramMaterial,
        asker: &mut dyn AsksForThePassword,
        stop: &dyn AsksToStop,
    ) -> WhereTheSetUpGot {
        // **`Server Setup` E2: a password given at the offer is held for the
        // rest of this run, and for nothing longer.** Added 2026-09-21.
        //
        // **Which of `Server Setup` E5's lifetimes this keeps, said plainly.**
        // The whole-app three are one action, the connection, and until
        // cleared. **This run is the action**, so *do not remember it* is kept
        // exactly and *while connected*'s own drawn promise — *"a long setup
        // asks once"* — holds inside the run. **Past the run nothing is kept**:
        // no hold on the connected period and no put to the secret store is
        // built, so a later run asks again. That is recorded as owed rather than
        // passed over.
        let mut given: Option<Origin> = None;
        asker.a_run_stands(self.kind, &self.run);
        // **`Server Setup` Y7a: this is the next run**, so the screen lets the
        // last run's words go and says where this one's are kept — **unless
        // the readings that composed it already asked** (item 1: the run's
        // words begin with those readings), in which case they are kept on.
        if self.the_words_go.is_none() {
            self.the_words_go = asker.where_the_words_go();
        }
        // **`Server Setup` Y7b and `Server Setup` Y7c: and where the download's and the
        // unpack's figures go**, with the build this run downloads, so its
        // steps can be named for it.
        self.the_figure_goes = asker.where_the_figure_goes(asked_for.install_this_blender.as_deref());
        let steps = self.steps.clone();
        for step in steps {
            loop {
                // **`Server Setup` Y8: the stop is honoured at a boundary and
                // never by a signal.** It is read between two steps, which is
                // where a boundary is.
                if stop.the_user_asked_to_stop() {
                    self.the_stop_is_written().await;
                    return WhereTheSetUpGot::TheRunEnded {
                        at: self.where_it_stands().await,
                        ending: TheRunsEnding::TheUserStoppedIt,
                    };
                }
                let how = match &given {
                    Some(password) => HowElevationIsObtained::APasswordIsHeld(password),
                    None => elevation,
                };
                // **`Design/Server Setup § A long step`: shown as its steps,
                // with the one under way marked** — told before each step, from
                // the run's own record read now (`Server Setup` S1), so the
                // panel is the machine's reading and never a guess. Added
                // 2026-09-27, fix piece 9.
                let under_way = self.the_first_boundary_of(step);
                self.the_screen_is_told(asker, under_way).await;
                let went = self
                    .one_step(step, how, asked_for, telegram, asker, stop)
                    .await;
                // **A recovery takes the run over mid-step**, so the screen is
                // told the run as it now is after every step.
                asker.a_run_stands(self.kind, &self.run);
                let ending = match went {
                    None => break,
                    Some(ending) => ending,
                };
                // **`Server Setup` E4's fourth bullet is the moment the offer is
                // made**, and nothing was sent: the step is taken again, whole,
                // with the password the user gave. **Taking it again is safe**
                // because every step it can be is check-first
                // (`Server Setup` §4.3), and because nothing of it reached the
                // package system — the refusal is decided before any state,
                // mark or command goes out.
                //
                // **One offer per refusal and no loop on an authentication**
                // (`Server Setup` E6): a password that then fails is the step's
                // own failure, in the machine's own words.
                if let TheRunsEnding::AStepNeedsAdminAccess { step: refused } = ending {
                    match self.the_password_is_asked_for(refused, asker, stop).await {
                        HowTheAskEnded::Given(password) => {
                            given = Some(password);
                            continue;
                        }
                        HowTheAskEnded::TheUserStoppedTheRun => {
                            self.the_stop_is_written().await;
                            return WhereTheSetUpGot::TheRunEnded {
                                at: self.where_it_stands().await,
                                ending: TheRunsEnding::TheUserStoppedIt,
                            };
                        }
                        HowTheAskEnded::TheMachineStoppedAnswering => {
                            return WhereTheSetUpGot::TheConnectionWent;
                        }
                        // **Declined: the run cannot go on, and it is not left
                        // open.** `Server Setup` Y8's own durable stop ends it
                        // at the boundary it reached, which is what the user
                        // declining the one thing it needed amounts to.
                        HowTheAskEnded::Declined => {
                            self.the_stop_is_written().await;
                            return WhereTheSetUpGot::TheRunEnded {
                                at: self.where_it_stands().await,
                                ending: TheRunsEnding::AStepNeedsAdminAccess { step: refused },
                            };
                        }
                    }
                }
                // **`Server Setup` S2a: a step that failed here is written onto
                // the machine, with its words**, before anything else is read
                // — so the run's own record says *failed* rather than being
                // left open for `Server Setup` Y7's window to call it cut.
                // Added 2026-09-21.
                if let TheRunsEnding::AStepFailed { step, said }
                | TheRunsEnding::TheServerStoppedAnswering { step, said } = &ending
                {
                    self.the_failed_step_is_recorded(*step, said).await;
                }
                if let TheRunsEnding::TheUnpackFailed { said } = &ending {
                    self.the_failed_step_is_recorded(Step::Tooling, said).await;
                }
                if let TheRunsEnding::TheTelegramMaterialWasNotComposed { said } = &ending {
                    self.the_failed_step_is_recorded(Step::TelegramMaterial, said).await;
                }
                // **The runner's ruling 1 of 2026-09-27: a refused restart is
                // recorded as the restart step failed**, which is what takes
                // *restart issued* back ([`the_restart_mark_was_taken_back`]).
                if let TheRunsEnding::TheRestartWasRefused { said } = &ending {
                    self.the_failed_step_is_recorded(Step::Restart, said).await;
                }
                return WhereTheSetUpGot::TheRunEnded {
                    at: self.where_it_stands().await,
                    ending,
                };
            }
        }
        // **`Server Setup` Y4's last row is read back rather than assumed.**
        //
        // **This used to answer *it reached done* as soon as every composed
        // step had been driven**, which was a stand-in: *done* is the desktop's
        // to write (`Server Setup` Y7) and until step 10c's boundary route no
        // request carried one, so the row was unreachable and the sentence said
        // so in its own code. **The route exists now**, so what is claimed here
        // is what the machine's own record says, read fresh over the
        // connection — **and the screen is told it** (2026-10-26, fault 4).
        let at = self.where_it_stands_as_the_screen_is_told(asker).await;
        if at == Boundary::Done {
            return WhereTheSetUpGot::TheRunEnded {
                at,
                ending: TheRunsEnding::ItReachedDone {
                    a_blender_was_published: self.composed.contains(&Boundary::Blender),
                    the_driver_missed_the_optix_bar: self.the_driver_missed_the_optix_bar,
                    the_swap_waits: self.the_swap_waits,
                },
            };
        }
        let said = format!(
            "Every step of this setup ran, but the server's record of it ends at {}.",
            the_word_for_boundary(at)
        );
        // **`Server Setup` S2a, for the one failure this side finds by reading
        // rather than by a step**: the recorded fact of that read is the
        // ending's content.
        self.the_failed_step_is_recorded(Step::Done, &said).await;
        WhereTheSetUpGot::TheRunEnded {
            at,
            ending: TheRunsEnding::AStepFailed {
                step: Step::Done,
                said,
            },
        }
    }

    /// **`Server Setup` Y7: one boundary written from the desktop, with
    /// `Server Setup` Y4a's mark in the same durable act.**
    ///
    /// *"The desktop records the step's start against the run, and the boundary
    /// is written when the desktop's verification read passes."* **So every
    /// caller of this calls it after its read and never on issuing a command.**
    ///
    /// **`Server Setup` Y3's moment is the machine's and is never sent**: the
    /// machine takes it on its own clock when it writes what it is told, which
    /// is what `State, identity and recovery` D5 requires.
    ///
    /// **`Server Setup` Y8's stop is read here as well as between steps**,
    /// because a `Server Setup` R1 step can feed more than one boundary — the
    /// driver's feeds two — and a stopped run may not be given a boundary past
    /// the last it passed.
    async fn the_boundary_is_written(
        &mut self,
        boundary: Boundary,
        did: WhatTheStepDid,
        stop: &dyn AsksToStop,
    ) -> Option<TheRunsEnding> {
        // **A write to the run, or a step commanded, so the record as last
        // read no longer stands** (2026-10-26, fault 4).
        self.the_record_as_last_read = None;
        if stop.the_user_asked_to_stop() {
            self.the_stop_is_written().await;
            return Some(TheRunsEnding::TheUserStoppedIt);
        }
        let step = boundary.its_step();
        let Ok(command) = Identity::issue(IdentityKind::Operation) else {
            return Some(TheRunsEnding::AStepFailed {
                step,
                said: "Could not generate a command ID.".to_string(),
            });
        };
        let request = Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            command,
            asked::THE_BOUNDARY_IS_WRITTEN_REQUEST,
        )
        .carrying(
            Fields::none()
                .and(asked::THE_SETUP_RUN_FIELD, self.run.as_text())
                .and(asked::THE_BOUNDARY_FIELD, the_word_for_boundary(boundary))
                .and(
                    asked::WHAT_THE_STEP_DID_FIELD,
                    match did {
                        WhatTheStepDid::Performed => asked::IT_PERFORMED,
                        WhatTheStepDid::NothingToDo => asked::IT_HAD_NOTHING_TO_DO,
                    },
                ),
        );
        let Some(answered) = ask(self.sessions, &self.package_area, &request).await else {
            return Some(TheRunsEnding::TheServerStoppedAnswering {
                step,
                said: "Smoozy's service on the server did not respond while the step's progress was saved.".to_string(),
            });
        };
        if answered.fields.text(asked::WHAT_BECAME_OF_IT_FIELD)
            == Some(asked::IT_HAS_A_RECORDED_OUTCOME)
        {
            self.the_last_one_seen = Some(boundary);
            return None;
        }
        // **A refusal from a machine that has already ended the run is told as
        // the run's own recorded ending, and not as a failure of the step in
        // hand.** Added 2026-09-27, step 10c's eighth-pass fix piece 1: on the
        // eighth pass on Hyperstack the machine's window had ended the run, the
        // write of *driver placed* was answered *"no setup run is open on this
        // machine"*, and the box said the server could not install the
        // graphics driver — which it had. **The run's state is read to know**,
        // and nothing is read out of the refusal's words (`Security and
        // licence` §13 rule 5).
        let refused_with = the_machines_own_words(&answered);
        if let Some(state) = self.the_runs_state().await {
            if state.ending.is_some() {
                return Some(the_recorded_ending(step, &state, Some(refused_with)));
            }
        }
        Some(TheRunsEnding::AStepFailed {
            step,
            said: refused_with,
        })
    }

    /// **One step of `Server Setup` R1's order. Nothing where it went well.**
    ///
    /// **`Server Setup` Y7's two granularities are both kept here.** R1's step
    /// is what a boundary hangs on; `Server Setup` Y7's machine-side
    /// list — *"fetch, unpack,
    /// `ldd` reads, publication, read-backs"* — is what the worker is
    /// commanded. **Two of R1's steps are made of both**: the tooling step's
    /// install is elevated and the unpack is the worker's, and the libraries
    /// step's install is elevated between the worker's two `ldd` reads.
    async fn one_step(
        &mut self,
        step: Step,
        elevation: HowElevationIsObtained<'_>,
        asked_for: &WhatWasAskedFor,
        telegram: &mut dyn ComposesTheTelegramMaterial,
        asker: &mut dyn AsksForThePassword,
        stop: &dyn AsksToStop,
    ) -> Option<TheRunsEnding> {
        match step {
            // **R1 item 3: the tool where it is absent, then the unpack with
            // it and nothing else.** The install comes first because the
            // unpack is what needs the tool.
            //
            // **`Server Setup` Y1's class for this step is the install's**, and
            // the install is `Server Setup`'s own
            // `elevated::THE_TOOLING_INSTALL`, which that crate classes as an
            // elevated package install. **So a run that fetches a Blender sets
            // the state here**, which is `Server Setup` Y4's *tooling* row word
            // for word.
            //
            // **`Server Setup` R1 item 3's *where absent*, decided by the
            // probe this run's readings took** —
            // `Server Setup`'s own `readings::probe_the_unpack_tool`, asked of
            // the machine's package system by its own documented query.
            // **Where the tool is there, nothing elevated goes out for it**,
            // so no state is set on its account and the run is additive on
            // this step exactly as `Server Setup` Y1's second example says.
            //
            // **`Server Setup` Y4a's mark is the probe's answer**, which is
            // that clause's own rule: *"the tooling step installs the unpack
            // tool where absent"*, and the mark is **performed** or **nothing
            // to do** by the same test `Server Setup` Y1 classes runs by. **The
            // unpack itself is not what the mark is about** — the step always
            // unpacks, and the act the clause weighs is the install.
            Step::Tooling => {
                let mark = if ready_action::the_tooling_step_installs_the_tool(&self.needs) {
                    if let Some(ending) = self.an_elevated_step(step, elevation, stop).await {
                        return Some(ending);
                    }
                    WhatTheStepDid::Performed
                } else {
                    WhatTheStepDid::NothingToDo
                };
                // **What failed here is the unpack and not the tool's install**,
                // and the box says so (`TheRunsEnding::TheUnpackFailed`).
                match self
                    .a_worker_step(step, asked::THE_UNPACK_STEP, asked_for, Some(mark))
                    .await
                {
                    Some(TheRunsEnding::AStepFailed { said, .. }) => {
                        Some(TheRunsEnding::TheUnpackFailed { said })
                    }
                    other => other,
                }
            }
            // **R1 item 4: the whole not-found list read at once, the named
            // packages installed explicitly, `ldd` re-read and it must come
            // back clean.**
            //
            // **The list comes back on the refresh and this side decides with
            // it — both what to install and `Server Setup` Y1's class.**
            //
            // **The two questions are one reading.** `Server Setup` E7 makes
            // the machine's `ldd` list *select, and never compose,* the
            // command, so the list is what says whether an elevated package
            // install happens at all — and that is exactly `Server Setup` Y1's
            // test for this step. **Whatever route carried the list would have
            // settled both**, which is why they could not be ruled apart.
            //
            // **The route is the run's own state reading**, which
            // `Server Setup` Y7 already has this side taking at its own
            // cadence: the step command is answered *still in hand*
            // (`Server protocol and transfers` C8) and cannot carry a reading
            // the worker has not taken, and a second request would let a
            // desktop refresh without learning where the run had got to.
            //
            // **The install is `Server Setup`'s own selection over the whole
            // list at once** — never one library at a time, never a partial
            // install, and a library the mapping does not name fails the step
            // honestly with the machine's own words.
            Step::Libraries => {
                if let Some(ending) = self
                    .a_worker_step(step, asked::THE_LIBRARY_CHAIN_IS_READ_STEP, asked_for, None)
                    .await
                {
                    return Some(ending);
                }
                let mark = match self.the_libraries_install(step, elevation, stop).await {
                    Ok(mark) => mark,
                    Err(ending) => return Some(ending),
                };
                self.a_worker_step(
                    step,
                    asked::THE_LIBRARY_CHAIN_IS_READ_AGAIN_STEP,
                    asked_for,
                    Some(mark),
                )
                .await
            }
            Step::Driver => self.the_driver_step(elevation, asked_for, stop).await,
            Step::Restart => self.the_restart_step(elevation, asker, stop).await,
            // **R1 item 1's readings are taken before the run is composed** —
            // step 10c piece 8 refused that step by name on the machine's side,
            // because both command strings live in a crate the machine's
            // program may never reach. `Server Setup` Y4's *started* boundary
            // is the machine's own, written with the record at run start.
            Step::Readings => None,
            // **R1 item 8's Telegram material: nothing places it, and this says
            // so rather than passing quietly.**
            //
            // **`Server Setup` Y4's *telegram* boundary guarantees that *"the
            // role's material is placed and the desired state set"***, and
            // `Server Setup` G1 makes that material `Telegram` §20's list whole
            // — the token file, the role configuration and the durable desired
            // state. **The step is composed only where the user asked for
            // Telegram**, so a run with it off never meets this.
            Step::TelegramMaterial => self.the_telegram_material(telegram, stop).await,
            Step::Fetch => {
                self.a_worker_step(step, asked::THE_FETCH_STEP, asked_for, None)
                    .await
            }
            // **`Server Setup` B15's plain deletion**, the worker's own: the
            // executable, its answered version and `Server Setup` B14's covered jobs ride the
            // command ([`TheRunBeingDriven::the_removal`]). The 38's piece 14c.
            //
            // **`Server Setup` B15's second route rides the same step** where
            // the removal names the manager's package: the manager's own
            // removal goes out first, elevated, from this side
            // (`Server Setup` Y7, E2, E3), and `Server Setup` Y4a's mark rides
            // the worker's step, which reads the build gone. Step 10c's owed
            // piece 6.
            Step::BlenderRemoval => {
                let managed = self.the_removal.as_ref().and_then(|removal| {
                    removal
                        .the_managers_package
                        .map(|package| (package, removal.executable.clone()))
                });
                let mark = match managed {
                    None => None,
                    Some((package, executable)) => match self
                        .the_managers_removal(step, package, &executable, elevation, stop)
                        .await
                    {
                        Ok(mark) => Some(mark),
                        Err(ending) => return Some(ending),
                    },
                };
                self.a_worker_step(step, asked::THE_BUILD_IS_REMOVED_STEP, asked_for, mark)
                    .await
            }
            Step::BlenderPublication => {
                if let Some(ending) = self
                    .a_worker_step(step, asked::THE_BUILD_IS_PUBLISHED_STEP, asked_for, None)
                    .await
                {
                    return Some(ending);
                }
                self.the_question_below_the_floor(step, asked_for, asker, stop).await
            }
            // **R1 item 9: the final verification, then `Server Setup` Y4's
            // *done*.**
            //
            // **The verification is the worker's and the boundary is the
            // desktop's** — `Server Setup` Y4's last row guarantees the
            // verification passed *and* that the state, where set, was cleared
            // and reconciliation ran, and that clear is
            // `Rented-server runtime` P3's act on the ordinary command path.
            // **The step reaches no boundary of its own**, so it is waited on
            // at the run's own state reading until the step under way is no
            // longer this one.
            Step::Done => {
                if let Some(ending) = self
                    .a_worker_step(step, asked::THE_FINAL_VERIFICATION_STEP, asked_for, None)
                    .await
                {
                    return Some(ending);
                }
                // **The clear comes before the boundary, because the boundary
                // is what guarantees it happened.** `Server Setup` Y4's *done*
                // row is *"the final verification passed; the state (where set)
                // is cleared and reconciliation ran"*, and
                // `Rented-server runtime` P5 makes the clear and the
                // reconciliation one durable act on the machine. **A *done*
                // written first would claim a clear that had not been made.**
                if let Some(ending) = self.the_machine_level_state_is_cleared().await {
                    return Some(ending);
                }
                self.the_boundary_is_written(Boundary::Done, WhatTheStepDid::Performed, stop)
                    .await
            }
        }
    }

    /// **`Server Setup` R1 item 8, performed: `Telegram` §20's list composed and
    /// placed, and the role's desired state set.**
    ///
    /// **`Server Setup` Y4's *telegram* row guarantees *"the role's material is
    /// placed and the desired state set"***, and `Server Setup` G1 makes that
    /// material §20's list whole — the token file, the role configuration and
    /// the durable enabled-or-disabled desired state. **Until this was written
    /// the step composed nothing, so a run with Telegram switched on stopped
    /// here rather than passing quietly**, which was right: writing the boundary
    /// would have claimed a step that did not run.
    ///
    /// **The composing is the caller's** ([`ComposesTheTelegramMaterial`]),
    /// because §20's list is made of the desktop's own record and its secret
    /// store, neither of which this driving holds. **The secret never reaches
    /// this file**: `Telegram`'s own `TheMaterial` shows nothing of the token,
    /// writes over its bytes when it goes, and what this side handles is the
    /// paths and the placement's outcome.
    ///
    /// **The two files are placed inside the authenticated connection, each
    /// path a protocol field and never a command argument**, which is
    /// `Security and licence` §10 and is `Server Setup`'s own
    /// `acts::place_the_telegram_material`. **The desired state is not a file**
    /// and goes on the ordinary command path (`Rented-server runtime` W8), which
    /// is what §20's third item says it is.
    ///
    /// **The order is the files first and the state after**, because the role
    /// worker reads the token at start (`Telegram` K2): a state written first
    /// would start a worker with nothing to read.
    ///
    /// **`Server Setup` Y4a's mark is *performed***. The step has no
    /// check-first half — there is nothing on the machine it could find already
    /// done that would make the placement unnecessary — so the closed set of two
    /// has one value it can take here.
    async fn the_telegram_material(
        &mut self,
        telegram: &mut dyn ComposesTheTelegramMaterial,
        stop: &dyn AsksToStop,
    ) -> Option<TheRunsEnding> {
        // **A write to the run, or a step commanded, so the record as last
        // read no longer stands** (2026-10-26, fault 4).
        self.the_record_as_last_read = None;
        let step = Step::TelegramMaterial;
        // **Composing is this computer's, so a composing that fails is said as
        // Smoozy's own sentence and not as the server's failure** — the
        // Creator's eleventh notes, note 2.
        let material = match telegram.composed() {
            Ok(material) => material,
            Err(said) => return Some(TheRunsEnding::TheTelegramMaterialWasNotComposed { said }),
        };
        // **`Security and licence` §10: the token file sits in the package
        // area, outside every job folder and never under a shared temporary
        // area.** The composing side works the area out from the root this run
        // recorded; where the two disagree the file is not placed at all,
        // because a token landing somewhere else is the one thing that clause
        // exists to stop.
        if !the_material_lands_in_the_package_area(&material, &self.package_area) {
            return Some(TheRunsEnding::AStepFailed {
                step,
                said: THE_FILES_WOULD_NOT_HAVE_LANDED.to_string(),
            });
        }
        // **Read before anything is placed**: whether a worker is running there
        // that will not read what is about to land (`Telegram` L4's sentence of
        // 2026-10-12).
        let the_role_is_enabled_there = telegram.the_role_is_enabled_there();
        let mut files = OnThisServersPoint {
            sessions: self.sessions,
        };
        let placed = acts::place_the_telegram_material(&mut files, material.the_files()).await;
        // **A file the machine would not take stops the step in the machine's
        // own words**, and no boundary is written: `Server Setup` Y4's row
        // promises the material is placed, and half of it is not it.
        let refused: Vec<String> = placed
            .iter()
            .filter(|(_, went)| !went.it_was_carried_out())
            .map(|(path, went)| format!("{path}: {}", went.in_words()))
            .collect();
        if !refused.is_empty() {
            // **What the machine said about each file is handed back for
            // `Telegram` L1's told-record bookkeeping**, which is that
            // document's and is not done here, so a file the machine refused
            // moves no telling and a file it took moves its own.
            telegram.the_placement_was_answered(&material, &placed);
            return Some(TheRunsEnding::AStepFailed {
                step,
                said: refused.join("; "),
            });
        }
        if the_role_is_enabled_there {
            // **`Telegram` L4's sentence of 2026-10-12: *disabled* then
            // *enabled***, and the files are handed back only once the pair is answered — a
            // running worker that never read them was never told them, so a
            // pair cut off leaves them owed to the next connection.
            if let Err(why) = the_running_worker_reads_what_was_placed(
                self.sessions,
                &self.package_area,
                telegram,
            )
            .await
            {
                return Some(the_ending_where_the_state_was_not_written(step, why, false));
            }
            telegram.the_placement_was_answered(&material, &placed);
        } else {
            telegram.the_placement_was_answered(&material, &placed);
            if let Some(ending) = self.the_roles_desired_state_is_enabled(step).await {
                return Some(ending);
            }
            // **`Telegram` L1 and L2: the role's desired state is a told thing**,
            // written on the acknowledgement like the two files' tellings — until
            // the 38's piece 11 nothing wrote it, so every server set up with
            // Telegram on was named as not reached in Settings.
            telegram.the_desired_state_was_acknowledged();
        }
        self.the_boundary_is_written(Boundary::Telegram, WhatTheStepDid::Performed, stop)
            .await
    }

    /// **`Telegram` §20's third item and `Rented-server runtime` W8's act: the
    /// role's durable desired state, written to *enabled* through the ordinary
    /// command path.**
    ///
    /// **Never a direct process start and never a restart of anything else**
    /// (`Rented-server runtime` W8 and `Rented-server runtime` W9): the supervisor
    /// does every start and stop,
    /// and a change of this state restarts nothing but the role worker.
    async fn the_roles_desired_state_is_enabled(&mut self, step: Step) -> Option<TheRunsEnding> {
        the_roles_desired_state_is_written(self.sessions, &self.package_area, true)
            .await
            .err()
            .map(|why| the_ending_where_the_state_was_not_written(step, why, true))
    }

    /// **`Server Setup` R1 item 4's install, selected from the whole not-found
    /// list the worker's first `ldd` read produced.**
    ///
    /// **The list is read off the run's own state reading** — the refresh this
    /// side is already taking (`Server Setup` Y7) — and the selection is
    /// `Server Setup`'s own `elevated::the_libraries_install` over
    /// `ready_action::what_to_install_for`'s answer. **Nothing is composed
    /// here**: `Server Setup` E7 requires the list to *select* a command every
    /// byte of which is fixed at build time, and a name read off a machine
    /// never reaches a command string.
    ///
    /// **What comes back is `Server Setup` Y4a's mark for the step**, which the
    /// re-read carries to its boundary because the act was this side's.
    ///
    /// - **An empty list is a reading and not an absence** — *"clean means the
    ///   list is empty"* — so the step has nothing to do and the run stays
    ///   additive if nothing else changes the machine.
    /// - **A library the mapping does not name fails the step**, with the
    ///   machine's own names in the words. `Server Setup` R1 item 4's own rule
    ///   is that there is never a partial install, and a step that installed
    ///   what it could would leave `ldd` unclean and the action claiming a
    ///   check that did not pass.
    async fn the_libraries_install(
        &mut self,
        step: Step,
        elevation: HowElevationIsObtained<'_>,
        stop: &dyn AsksToStop,
    ) -> Result<WhatTheStepDid, TheRunsEnding> {
        let Some(state) = self.the_runs_state().await else {
            return Err(TheRunsEnding::TheServerStoppedAnswering {
                step,
                said: "Smoozy's service on the server stopped responding. The missing libraries \
                       could not be read."
                    .to_string(),
            });
        };
        let what = ready_action::what_to_install_for(&state.the_libraries_not_found);
        match &what {
            ready_action::WhatTheLibrariesStepInstalls::NothingWasMissing => {
                Ok(WhatTheStepDid::NothingToDo)
            }
            ready_action::WhatTheLibrariesStepInstalls::TheMappingDoesNotNameThese(these) => {
                Err(TheRunsEnding::AStepFailed {
                    step,
                    said: format!(
                        "This server is missing libraries with no known package: {}. Nothing was \
                         installed.",
                        these.join(", ")
                    ),
                })
            }
            ready_action::WhatTheLibrariesStepInstalls::ThesePackages(_) => {
                // **The selection is one of a fixed set or it is none**, and
                // none means the same refusal: a command built from two entries
                // is a command nobody fixed at build time.
                let Some(install) = elevated::the_libraries_install(&what) else {
                    return Err(TheRunsEnding::AStepFailed {
                        step,
                        said: "No single package list covers the libraries this server is missing. \
                               Nothing was installed."
                            .to_string(),
                    });
                };
                match self.an_elevated_act(step, install, elevation, stop).await {
                    Some(ending) => Err(ending),
                    None => Ok(WhatTheStepDid::Performed),
                }
            }
        }
    }

    /// **`Server Setup` R1 item 5, in the order the clause fixes, and the order
    /// is the whole of the safety.**
    ///
    /// **Check first.** *"`nvidia-smi` run and its exit read; where it answers,
    /// nothing is installed and nothing is touched."* `Server Setup` R5 forbids
    /// repeating a driver install whose check passes, and the
    /// reinstall-becomes-upgrade observation is the exact shape of *running it
    /// again makes it worse* — **so the check is what decides whether anything
    /// goes out at all.**
    ///
    /// **Then the verification read, and only then the boundary.** That is
    /// `Server Setup` Y7's elevated-step rule word for word, and
    /// `Server Setup` Y4's *driver placed* is what it writes: *"the driver
    /// step's work ran to its own end and the check's outcome after it is
    /// recorded; no restart has been issued."*
    ///
    /// **`Server Setup` Y4a's mark is the step's**: *nothing to do* where the
    /// check answered first or a driver is already on the machine, *performed*
    /// where the distribution's own route went out.
    ///
    /// **The step is `Server Setup`'s own `acts::carry_out_the_driver_step` and
    /// this file performs none of it. Changed 2026-09-20.** That function keeps
    /// R1 item 5 whole — the branch, `Server Setup` M1b's components step on
    /// Debian, the family's own route, the check again — and **it was called by
    /// nothing but its own tests**, while this file kept the same clause its own
    /// way with no already-installed branch at all. **Two implementations of one
    /// clause is the fault this stage keeps paying for**, and what the driving
    /// kept was the one that could not perform R1 item 5's
    /// restart-before-any-install case: on a machine whose driver was installed
    /// and unbooted it issued the distribution's install, which is the
    /// reinstall-becomes-upgrade harm `Server Setup` R5 forbids outright.
    ///
    /// **`Server Setup` Y1's state is set here and not inside the act**,
    /// because the state is the desktop's own request (`Rented-server runtime`
    /// P3) and `Server Setup` Y1 sets it *before* the first machine-changing
    /// act. **The class is not decided here**: the judgement says which branch
    /// the step will take, and the act's own
    /// `elevated::WhatAnActDoesToTheMachine` says what that branch's command
    /// does to the machine.
    async fn the_driver_step(
        &mut self,
        elevation: HowElevationIsObtained<'_>,
        asked_for: &WhatWasAskedFor,
        stop: &dyn AsksToStop,
    ) -> Option<TheRunsEnding> {
        // **R1 item 5's *check first*, taken once and read twice** — whether the
        // driver answers, and, where it did not, whether one is installed at all
        // (`Server Setup`'s own
        // `readings::WhatTheDriverCheckSaid::a_driver_is_already_installed`).
        // **Row 104: the readings are taken afresh at the step** rather than
        // carried from the composition.
        let check = self.the_driver_check_taken().await;
        // **`Server Setup` R1 item 5 as corrected on 2026-09-30: the running
        // kernel's module, read afresh here too**, and only where the check
        // ended at 9. Eleventh-pass fix piece 10.
        let module = {
            let mut channel = OnThisServersPoint {
                sessions: self.sessions,
            };
            readings::read_the_running_kernels_module(&mut channel, self.needs.family, &check).await
        };
        // **`Server Setup` R1 item 5's version, read afresh with the check**,
        // where the run has a bar to read it against. Added 2026-10-11.
        let the_drivers_version = {
            let mut channel = OnThisServersPoint {
                sessions: self.sessions,
            };
            acts::the_drivers_version_where_a_bar_applies(&mut channel, &self.needs, &check).await
        };
        // **`Server Setup` R1 item 5, as written 2026-11-03: where the check
        // failed and no restart is owed, what the package system holds is read**,
        // so a removal that stopped part-way is finished rather than walked past.
        let the_driver_packages = if matches!(check, readings::WhatTheDriverCheckSaid::ItAnswered)
            || check.a_driver_is_already_installed(module)
        {
            readings::WhatTheDriverPackagesSay::NotRead
        } else {
            let mut channel = OnThisServersPoint {
                sessions: self.sessions,
            };
            readings::read_the_driver_packages(&mut channel, self.needs.family).await
        };
        let mut needs = ready_action::WhatTheMachineNeeds {
            the_driver_answers: matches!(check, readings::WhatTheDriverCheckSaid::ItAnswered),
            a_driver_is_already_installed: check.a_driver_is_already_installed(module),
            the_drivers_version,
            the_driver_packages,
            ..self.needs.clone()
        };
        // **`Server Setup` R1b: where the swap would be made, the live set is
        // read first**, and where anything is live the swap is not made in
        // this run, and it waits. Read only where the swap is the step's act,
        // so no other run pays a reading for it. Added 2026-10-12.
        if ready_action::what_the_driver_step_does(&needs)
            == ready_action::WhatTheDriverStepDoes::TheSwapUpToTheBar
            && self.anything_is_live(asked_for).await
        {
            needs.anything_is_live = true;
            self.the_swap_waits = true;
        }

        // **The one judgement, asked before the act so `Server Setup` Y1's
        // state can be set before it.** The distribution's own detection route
        // and, from 2026-10-11, `Server Setup` R1 item 5's swap up to the
        // OptiX bar are the two branches that reach the package system.
        let the_familys_route = matches!(
            ready_action::what_the_driver_step_does(&needs),
            ready_action::WhatTheDriverStepDoes::TheDistributionsOwnDetectionRoute
                | ready_action::WhatTheDriverStepDoes::TheSwapUpToTheBar
                | ready_action::WhatTheDriverStepDoes::TheStoppedRemovalIsFinished
        );
        // **`Server Setup` E4's fourth bullet, before the state and before
        // anything goes out** — the family's route is elevated from its first
        // command. Added 2026-09-21.
        if the_familys_route && elevation == HowElevationIsObtained::APasswordIsOwedAndNoneIsHeld {
            return Some(TheRunsEnding::AStepNeedsAdminAccess { step: Step::Driver });
        }

        // **`Server Setup` Y7's second bullet: the step's start is recorded
        // against the run before anything of it goes out**, the state included
        // — the one thing that lets `Server Setup` S1 read this step as
        // *stopped part-way* rather than *not started* when the connection is
        // cut during the install. **After E4's refusal**, so a declined offer
        // leaves no step recorded as started. Added 2026-09-29, step 10c's
        // transfer wave piece 12.
        if let Some(ending) = self.the_elevated_steps_start_is_recorded(Step::Driver).await {
            return Some(ending);
        }

        if the_familys_route {
            let machine_changing = the_elevated_step_for(Step::Driver, needs.family)
                .is_some_and(|act| act.what_it_does_to_the_machine.is_machine_changing());
            if machine_changing {
                if let Some(ending) = self.the_machine_level_state_is_set(Step::Driver).await {
                    return Some(ending);
                }
            }
        }

        // **`Server Setup` R1b's last guard: once the machine-level state
        // stands, which holds every admission, the live set is read again
        // before the swap's first act.** Where it is no longer empty no swap
        // is made — the run carries on without it, and the swap waits again.
        // **Nothing is recorded as missed**: the swap was not tried. This
        // closes the gap between the first reading and the act, as
        // `Rented-server runtime` G21 does for the update. Added 2026-10-12.
        if ready_action::what_the_driver_step_does(&needs)
            == ready_action::WhatTheDriverStepDoes::TheSwapUpToTheBar
            && self.anything_is_live(asked_for).await
        {
            needs.anything_is_live = true;
            self.the_swap_waits = true;
        }

        // **`Server Setup` R1c: item 5 compares and acts from here** — on a
        // machine Smoozy manages, with a bar to read against, and with the
        // swap not left out for live work, past both readings of the live
        // set. So the run's ending, however it comes, releases the holds of
        // the driver's reason; a run that backed off releases none. Added
        // 2026-10-12, held-job piece A.
        self.the_server_item_5_is_told_for = asked_for.the_server.clone();
        if !self.the_swap_waits
            && needs.the_optix_bar.is_some()
            && matches!(needs.kind, smoozy_server_setup::kind::MachineKind::ASystemdMachine)
        {
            if let Some(server) = &self.the_server_item_5_is_told_for {
                crate::servers::the_driver_swap::item_5_compared_and_acted(
                    server,
                    needs.the_drivers_version.as_ref(),
                );
            }
        }

        // **`Server Setup` Y7's refresh and `Server Setup` Y8's stop go on
        // through every command of the step, and once either ends it nothing
        // further is sent** — [`TheAdminStepsChannel`]. Added 2026-09-27, step 10c's
        // eighth-pass fix piece 1: the eighth pass on Hyperstack watched the
        // machine end the run during this step's install and the desk send the
        // support binaries' install after it.
        let mut channel = self.an_admin_steps_channel(Step::Driver, stop);
        let mut files = OnThisServersPoint {
            sessions: self.sessions,
        };
        let went = acts::carry_out_the_driver_step(
            &mut channel,
            &mut files,
            &needs,
            check,
            elevation,
        )
        .await;
        let seen = channel.seen();
        if let Some(ending) = self.what_the_admin_step_saw(Step::Driver, seen).await {
            return Some(ending);
        }

        // **A step that did not run to its end ends the run in the machine's
        // own words**, and no boundary is written — `Server Setup` Y4's *driver
        // placed* row guarantees *"the driver step's work ran to its own end"*.
        if let Some(said) = went.what_went_wrong() {
            return Some(TheRunsEnding::AStepFailed {
                step: Step::Driver,
                said,
            });
        }
        // **`Server Setup` R1 item 5 as extended 2026-10-11: after a swap that
        // went through, the check passes only at or above the bar**, so a
        // driver still below it gets item 6's restart and the check again.
        // **Where no restart follows, this is the last check**, and whether the
        // driver missed the bar is read here.
        let the_check_passes = went.the_check_passes(&needs);
        self.the_driver_missed_the_optix_bar =
            the_check_passes && went.the_driver_missed_the_bar(&needs);
        // **`Server Setup` R1c: where no restart follows, this is item 5's last
        // check**, and what it read is told for the release.
        if the_check_passes {
            if let Some(server) = &self.the_server_item_5_is_told_for {
                crate::servers::the_driver_swap::item_5s_last_check_read(
                    server,
                    went.the_drivers_version_at_the_end(&needs),
                    self.the_driver_missed_the_optix_bar,
                );
            }
        }
        self.the_driver_step = Some(TheDriverStepsOwnAnswer {
            mark: went.what_the_step_did(),
            the_driver_answers: the_check_passes,
        });
        self.the_boundary_is_written(Boundary::DriverPlaced, went.what_the_step_did(), stop)
            .await
    }

    /// **`Server Setup` R1 item 6: the restart, *"only where step 5's check
    /// failed after its work"* — Fact 80's rule as a step.**
    ///
    /// **Where the check passed, this step has nothing to do**, and
    /// `Server Setup` Y4a marks it so rather than leaving its boundary out:
    /// that clause names the restart among the check-first steps whose mark is
    /// *nothing to do*. **`Server Setup` Y4's *card answering* follows it** —
    /// *"the check passed — on this boot"* — and it carries the driver step's
    /// own mark, because `Server Setup` Y4a marks a step and both of those
    /// boundaries are the driver step's.
    ///
    /// **Where the check failed, the mark is set, the restart goes out, and the
    /// run waits for the machine.** This is `Server Setup` §8 whole, and its
    /// three parts are useless apart.
    ///
    /// **1. The mark, before the command leaves** (`Server Setup` T4). *"This
    /// drop is mine* is a state Smoozy sets before the act, never a reading it
    /// takes after it"*, because a machine closing politely and a provider
    /// powering one off are detected in the same second. **It is set through
    /// `Server Setup`'s own witness** — `acts::TheRestartMayBeIssued`'s *yes*
    /// cannot be built without one, and the witness cannot be made without a
    /// standing period's mark to set, so the restart cannot go out unmarked.
    ///
    /// **2. `Server Setup` Y4's *restart issued*, written just before the
    /// command is sent** — the paragraph under Y4's table: *"This mark is
    /// written durably just before the restart command is sent, because the
    /// command takes the connection and the machine down and nothing can be
    /// written after it — so here T3's *durable before the command is issued*
    /// governs."* **And `Server Setup` Y1's state before the mark**, because
    /// the row the mark writes guarantees *"the state is durable"*.
    /// *(Changed 2026-09-27, fix piece 6. This wrote the mark after the command
    /// returned and said a write that lost the race cost nothing. **It cost the
    /// whole recovery**: on step 10c's seventh passes the restart took the
    /// connection before the write, no *restart issued* was ever written, and
    /// `Server Setup` T5's recovery — built, tested and right — never ran, on
    /// Hyperstack, Sesterce and the fifth pass's Hyperstack alike.)*
    ///
    /// **3. The command's own outcome, told two ways** — the runner's ruling 1
    /// of 2026-09-27. **Refused while the connection stays up** — the command
    /// came back with an exit status that is not success and the machine still
    /// answers — ends as the restart step failed, recorded as any failed step
    /// is, which takes the mark back
    /// ([`the_restart_mark_was_taken_back`]); the mark on the period is cleared
    /// by the machine answering, which is `Server Setup` T4's first ending.
    /// **Anything else** — the command did not come back, came back with no
    /// status, or the machine does not answer after it — **is the restart
    /// taking the connection with it**, and the run waits.
    ///
    /// **4. The wait, whose figures are none of this document's**
    /// (`Server Setup` T6). Return was measured at 9 to 90 seconds across three
    /// machines and a container recycle at about 90, so *"anything budgeting a
    /// fixed wait is wrong on most machines"*. **Nothing here budgets one**:
    /// the wait ends when `Servers`' own policy for a restart-marked period
    /// ends it, and the three endings are that document's closed set.
    ///
    /// **A recovery picking the step up between the driver's two boundaries**
    /// ([`HowTheRestartStepResumes`]) takes the check first either way — §4.3's
    /// restart bullet, and T5's *"carries on at the check"*.
    async fn the_restart_step(
        &mut self,
        elevation: HowElevationIsObtained<'_>,
        asker: &mut dyn AsksForThePassword,
        stop: &dyn AsksToStop,
    ) -> Option<TheRunsEnding> {
        let mut went = self.the_driver_step.unwrap_or(TheDriverStepsOwnAnswer {
            mark: WhatTheStepDid::NothingToDo,
            the_driver_answers: false,
        });
        let mut the_mark_already_stands = false;
        match self.the_restart_step_resumes.take() {
            None => {}
            // **`Server Setup` T5: the restart went out before this run took
            // the setup over, so the setup carries on at the check** — on this
            // boot, which is what *card answering* guarantees.
            Some(HowTheRestartStepResumes::AtTheCheck) => {
                if !self.the_driver_check().await {
                    return Some(TheRunsEnding::AStepFailed {
                        step: Step::Driver,
                        said: "The server is back, but its GPU still does not respond."
                            .to_string(),
                    });
                }
                self.the_bar_is_read_at_the_last_check().await;
                return self
                    .the_boundary_is_written(Boundary::CardAnswering, went.mark, stop)
                    .await;
            }
            // **`Server Setup` §4.3: *"A recovery resuming at the restart step
            // re-runs step 5's check first and issues the restart only where
            // it fails again."*** Whether a restart is owed is a reading of the
            // machine now, never an outcome recorded in an earlier sitting.
            Some(HowTheRestartStepResumes::Afresh {
                the_mark_is_already_in_this_record,
            }) => {
                went.the_driver_answers = self.the_driver_check().await;
                if went.the_driver_answers {
                    self.the_bar_is_read_at_the_last_check().await;
                }
                the_mark_already_stands = the_mark_is_already_in_this_record;
            }
        }
        if went.the_driver_answers {
            if !the_mark_already_stands {
                if let Some(ending) = self
                    .the_boundary_is_written(
                        Boundary::RestartIssued,
                        WhatTheStepDid::NothingToDo,
                        stop,
                    )
                    .await
                {
                    return Some(ending);
                }
            }
            return self
                .the_boundary_is_written(Boundary::CardAnswering, went.mark, stop)
                .await;
        }

        // **`Server Setup` E4's fourth bullet, before the mark.** The restart is
        // elevated, and a mark set for a restart that is never sent would read
        // the next drop as one Smoozy caused. Added 2026-09-21.
        if elevation == HowElevationIsObtained::APasswordIsOwedAndNoneIsHeld {
            return Some(TheRunsEnding::AStepNeedsAdminAccess { step: Step::Restart });
        }

        // **`Server Setup` T4 and T1, in one value.** The mark is set on the
        // period first and the witness comes out of setting it; `it_may` is
        // then `Server Setup` T1's own bound — the systemd kind, inside a setup
        // step — read from the kind the opening readings took rather than
        // inferred from this step having been composed.
        let Some(period) = self.the_period.clone() else {
            return Some(TheRunsEnding::AStepFailed {
                step: Step::Restart,
                said: "The server was not restarted: its connection could not be marked for the \
                       restart."
                    .to_string(),
            });
        };
        let Some(elevated) = the_elevated_step_for(Step::Restart, self.needs.family) else {
            return Some(TheRunsEnding::AStepFailed {
                step: Step::Restart,
                said: "Not restarted: no restart command known for this server."
                    .to_string(),
            });
        };
        // **`Server Setup` Y1 and `Rented-server runtime` P4: the state is
        // durable before the restart**, and before the mark that says so.
        if elevated.what_it_does_to_the_machine.is_machine_changing() {
            if let Some(ending) = self.the_machine_level_state_is_set(Step::Restart).await {
                return Some(ending);
            }
        }
        let may = acts::TheRestartMayBeIssued::on(
            self.needs.kind,
            true,
            smoozy_server_setup::restart::mark_the_connected_period_restarting(&period),
        );
        if !may.it_may() {
            return Some(TheRunsEnding::AStepFailed {
                step: Step::Restart,
                said: "Restarting this kind of server is not supported.".to_string(),
            });
        }
        // **`Server Setup` Y4's *restart issued*, just before the command** —
        // the one boundary written before its step (`Server Setup` T3).
        if !the_mark_already_stands {
            if let Some(ending) = self
                .the_boundary_is_written(Boundary::RestartIssued, WhatTheStepDid::Performed, stop)
                .await
            {
                return Some(ending);
            }
        }
        // **The screen is told the restart is under way** — `Design/Server
        // Setup § A long step`: *"the panel says the server is restarting"*.
        self.the_screen_is_told(asker, Some(Boundary::RestartIssued)).await;

        match self.the_restart_command_is_sent(elevated, elevation).await {
            HowTheRestartCommandWent::TheConnectionWentWithIt => {}
            // **The runner's ruling 1 of 2026-09-27.** The machine answered, so
            // `Server Setup` T4's first ending clears the period's mark; the
            // failure is recorded by the driving, which is what takes *restart
            // issued* back.
            HowTheRestartCommandWent::ItWasRefused { said } => {
                period.cleared_by(HowTheWaitEnds::TheMachineAnswers);
                return Some(TheRunsEnding::TheRestartWasRefused { said });
            }
        }

        match self.the_wait_for_the_machine(&period, stop).await {
            // **`Server Setup` T5: the setup resumes from the *restart issued*
            // boundary**, and the recovery run is what carries it on.
            HowTheWaitEnds::TheMachineAnswers => {
                self.the_setup_is_picked_up(went, asker, stop).await
            }
            // **`Server Setup` T6: the user stopping the wait clears nothing.**
            // The mark stands, the machine-level state stands, the policy goes
            // on running, and the setup is picked up on return — **what ends is
            // the waiting**, which is this run.
            HowTheWaitEnds::TheUserStopsWaiting => Some(TheRunsEnding::TheMachineIsRestarting),
            // **`Server Setup` T6 and S2: only what Smoozy knows.**
            HowTheWaitEnds::ThePolicyGivesUp => Some(TheRunsEnding::TheMachineDidNotComeBack {
                said: smoozy_server_setup::restart::what_is_claimed_when_the_policy_gives_up()
                    .to_string(),
            }),
        }
    }

    /// **`Server Setup` T1's command, sent, and its outcome told apart the way
    /// the runner's ruling 1 of 2026-09-27 asks.** Added the same day.
    ///
    /// **A refusal wants two positive readings, and anything short of both is
    /// the restart going.** The command came back with an exit status that is
    /// not success — *"the exit status, absent where the channel closed without
    /// one"* is `Server protocol and transfers`' own transport's rule, so a
    /// status is the one thing a command cut by the reboot cannot bring back —
    /// **and** the machine answers the run's own state reading straight after.
    /// **The safe direction is the restart**: a refusal misread as a restart
    /// costs `Servers`' reconnect policy's wait and then the check on the boot
    /// that answers, which says the card does not answer; a restart misread as
    /// a refusal would leave a machine rebooting under a setup told it never
    /// went.
    ///
    /// **The words are the machine's where it printed any**, and Smoozy's own
    /// record of the refusal where it printed none (`Server Setup` S2a's third
    /// bullet).
    async fn the_restart_command_is_sent(
        &mut self,
        elevated: AnElevatedStep,
        elevation: HowElevationIsObtained<'_>,
    ) -> HowTheRestartCommandWent {
        let mut channel = TheRestartsOwnChannel {
            inner: TheElevatedChannel {
                sessions: self.sessions,
                words: self.the_words_go.clone().map(|go| (Step::Restart, go)),
            },
            the_last_status: None,
        };
        let said = match elevated::issue(&mut channel, elevated, elevation).await {
            Ok(said) if said.it_ended_cleanly => {
                return HowTheRestartCommandWent::TheConnectionWentWithIt
            }
            Ok(said) => said,
            Err(_) => return HowTheRestartCommandWent::TheConnectionWentWithIt,
        };
        let Some(Some(status)) = channel.the_last_status else {
            return HowTheRestartCommandWent::TheConnectionWentWithIt;
        };
        if self.the_runs_state().await.is_none() {
            return HowTheRestartCommandWent::TheConnectionWentWithIt;
        }
        let words = said.the_machines_own_words.trim();
        HowTheRestartCommandWent::ItWasRefused {
            said: if words.is_empty() {
                format!(
                    "The restart command exited with status {status}; the server stayed \
                     connected."
                )
            } else {
                said.the_machines_own_words.clone()
            },
        }
    }

    /// **`Server Setup` T6's wait, which has no constant of that document's and
    /// gains none here.**
    ///
    /// **The two endings that clear the mark are `Servers`' to declare**, and
    /// this waits on the period's own mark for whichever of them happens. **The
    /// third is the user's and is the only one this side decides**: it clears
    /// nothing, so it is read here rather than waited for, at the same cadence
    /// `Server Setup` Y7's refresh already uses. **That cadence is a reading
    /// interval and not a budget for the machine's return** — nothing here is
    /// derived from any machine's observed cost, which is T6's one requirement
    /// on the figures.
    ///
    /// **The wait has a condition it can certainly reach.** `Servers`' policy
    /// for a marked period gives up after four runs, and a period that ends
    /// under the mark ends this wait with it (S11), so nothing waits on a
    /// period that has gone.
    async fn the_wait_for_the_machine(
        &mut self,
        period: &TheRestartMark,
        stop: &dyn AsksToStop,
    ) -> HowTheWaitEnds {
        let told = period.how_the_wait_ended();
        tokio::pin!(told);
        loop {
            tokio::select! {
                how = &mut told => {
                    // **A period that ended under the mark is not one of the
                    // three**, and there is nothing left to wait on: the run is
                    // over the way a lost connection leaves it, which is
                    // `Server Setup` Y8's *stopping leaves exactly what a lost
                    // connection leaves* read from the other side.
                    return how.unwrap_or(HowTheWaitEnds::TheUserStopsWaiting);
                }
                () = wait_a_moment(driving::THE_REFRESH_CADENCE) => {
                    if stop.the_user_asked_to_stop() {
                        return HowTheWaitEnds::TheUserStopsWaiting;
                    }
                }
            }
        }
    }

    /// **`Server Setup` T5 and S4 item 2: the machine answered, so the setup is
    /// picked up by a recovery run from the *restart issued* boundary.**
    ///
    /// *"The reboot reaped the old worker, its group validates gone, and the
    /// recovery takes the state over and carries on at the check."*
    ///
    /// **Nothing resumes with nobody connected**, so the point is waited on
    /// until it lends again — `Servers` opened the connection, and this side's
    /// own sessions are lent a moment later. **The wait is bounded by the
    /// connect deadline `Servers` already uses**, which is that document's
    /// figure and not one of this document's.
    ///
    /// **The recovery is a run of no kind of its own** (`Server Setup` Y6b): it
    /// names the run it takes over, and the machine's own take-over gives it
    /// that run's kind, its composed list and its inherited marker in one
    /// durable act. **So this side sends no marker and resumes no step by
    /// number** — S4 item 3's *steps at or before the marker are not run again*
    /// holds because R1's order puts every remaining step after this one.
    ///
    /// **Then `Server Setup` Y4's *card answering*: *"the check passed — on
    /// this boot, not on a reading taken in any hand-reloaded state"*.** The
    /// check is run on the machine that came back, and where it still does not
    /// answer the boundary is not written and the run says so — writing it
    /// would claim a card that does not answer, which `Server Setup` R4 and W1
    /// forbid.
    async fn the_setup_is_picked_up(
        &mut self,
        went: TheDriverStepsOwnAnswer,
        asker: &mut dyn AsksForThePassword,
        stop: &dyn AsksToStop,
    ) -> Option<TheRunsEnding> {
        let waited = std::time::Instant::now();
        while !self.sessions.a_connection_is_live() {
            if waited.elapsed() > smoozy_servers::repair::CONNECT_DEADLINE {
                return Some(TheRunsEnding::AStepFailed {
                    step: Step::Restart,
                    said: "The server responded, but a session could not be opened on it."
                        .to_string(),
                });
            }
            wait_a_moment(driving::THE_REFRESH_CADENCE).await;
        }
        let taken_over = self.run.clone();
        self.run = match a_run_is_opened(
            self.sessions,
            &self.package_area,
            WhatOpensTheRun::ARecoveryOf(taken_over),
            &self.composed,
            stop,
            asker,
        )
        .await
        {
            Ok(run) => run,
            Err(ending) => return Some(ending),
        };
        // **The screen is told the run it now stands behind**, with the check
        // under way — the recovery is a new run, and the panel reads it.
        // **A record kept from the run it took over is not this run's**
        // (2026-10-26, fault 4).
        self.the_record_as_last_read = None;
        asker.a_run_stands(self.kind, &self.run);
        self.the_screen_is_told(asker, Some(Boundary::CardAnswering)).await;
        // **`Server Setup` R1 item 5's check, on this boot.** It is the same
        // one reading of the driver the run has always taken, and it is what
        // `Server Setup` Y4's *card answering* guarantees.
        if !self.the_driver_check().await {
            return Some(TheRunsEnding::AStepFailed {
                step: Step::Driver,
                said: "The server is back, but its GPU still does not respond."
                    .to_string(),
            });
        }
        self.the_bar_is_read_at_the_last_check().await;
        self.the_boundary_is_written(Boundary::CardAnswering, went.mark, stop)
            .await
    }

    /// **`Server Setup` R1 item 5: the driver's version read at the check on
    /// return, against the bar of the Blender this run installs.** Added
    /// 2026-10-11. A driver that answers below it — a swap that went through
    /// and came back below, or a family's route that chose a lower branch — is
    /// recorded as having missed the bar; **the run still reaches *done***.
    async fn the_bar_is_read_at_the_last_check(&mut self) {
        let mut channel = OnThisServersPoint {
            sessions: self.sessions,
        };
        let version = acts::the_drivers_version_where_a_bar_applies(
            &mut channel,
            &self.needs,
            &readings::WhatTheDriverCheckSaid::ItAnswered,
        )
        .await;
        self.the_driver_missed_the_optix_bar =
            ready_action::the_driver_is_below_the_bar(&self.needs, version.as_ref());
        // **`Server Setup` R1c: the check on return is item 5's last**, and
        // what it read is told for the release.
        if let Some(server) = &self.the_server_item_5_is_told_for {
            crate::servers::the_driver_swap::item_5s_last_check_read(
                server,
                version.as_ref(),
                self.the_driver_missed_the_optix_bar,
            );
        }
    }

    /// **`Server Setup` R1 item 5's check, which is the one reading ever taken
    /// of the driver**, and it is `Server Setup`'s own rather than a command
    /// composed here.
    async fn the_driver_check(&mut self) -> bool {
        matches!(
            self.the_driver_check_taken().await,
            readings::WhatTheDriverCheckSaid::ItAnswered
        )
    }

    /// **The same one reading, whole**, because R1 item 5's other branch is read
    /// off its exit and not off whether it answered.
    async fn the_driver_check_taken(&mut self) -> readings::WhatTheDriverCheckSaid {
        let mut channel = OnThisServersPoint {
            sessions: self.sessions,
        };
        readings::run_the_driver_check(&mut channel).await
    }

    /// **`Server Setup` R1b's live set, read now**: a transfer under way
    /// between this desktop and the machine, or any job the machine's queue
    /// reading (`Queue & History` A1) holds rendering, waiting or held
    /// (`Queue & History` §17). Added 2026-10-12.
    ///
    /// **A reading that could not be had is not an empty queue** — the swap
    /// restarts the machine, so it is read as live and the swap waits, the
    /// same way `Desktop app and delivery` V11's update waits on a work
    /// reading it could not have.
    async fn anything_is_live(&self, asked_for: &WhatWasAskedFor) -> bool {
        if asked_for
            .the_server
            .as_deref()
            .is_some_and(crate::servers::the_driver_swap::a_transfer_is_under_way_with)
        {
            return true;
        }
        let Some(request) = crate::servers::the_machines_work::the_queue_reading_request() else {
            return true;
        };
        match crate::servers::the_machines_work::the_queue_reading_in(
            self.sessions.ask_the_supervisor(&self.package_area, &request).await,
        ) {
            Ok(reading) => crate::servers::the_driver_swap::anything_is_live_in(&reading),
            Err(_) => true,
        }
    }

    /// **A step the setup worker performs**, commanded one at a time and waited
    /// on at the run's own state reading.
    ///
    /// **Two of `Server Setup`'s own `driving::who_performs` worker steps are
    /// not the worker's and are taken here** — step 10c piece 8 found both and
    /// refused them by name on the machine's side:
    ///
    /// - **`Server Setup` R1 item 1's readings** are `Servers` H11's table, and
    ///   both command strings are declared in a crate the machine's program may
    ///   never reach. They are taken over the command channel here, at
    ///   [`what_this_machine_needs`], before the run is even composed.
    /// - **`Server Setup` R1 item 8's Telegram material** is a built desktop
    ///   act (`Server Setup`'s own `acts::place_the_telegram_material`).
    ///
    /// **`Server Setup` Y4a's mark rides the command where the `Server Setup`
    /// R1 step's own act was this side's.** The worker did not perform that act
    /// and cannot say what it came to, so a *performed* mark reported by the
    /// worker for a libraries step that installed nothing would be exactly the
    /// false claim `Server Setup` Y4a exists to stop. **Nothing where the act is the worker's
    /// own**, and the worker's own mark is then what the boundary carries.
    async fn a_worker_step(
        &mut self,
        step: Step,
        commanded: &str,
        asked_for: &WhatWasAskedFor,
        mark: Option<WhatTheStepDid>,
    ) -> Option<TheRunsEnding> {
        // **A write to the run, or a step commanded, so the record as last
        // read no longer stands** (2026-10-26, fault 4).
        self.the_record_as_last_read = None;
        let Ok(command) = Identity::issue(IdentityKind::Operation) else {
            return Some(TheRunsEnding::AStepFailed {
                step,
                said: "Could not generate a command ID.".to_string(),
            });
        };
        let request = Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            command,
            asked::THE_ONE_SETUP_STEP_REQUEST,
        );
        let mut fields = Fields::none()
            .and(asked::THE_SETUP_RUN_FIELD, self.run.as_text())
            .and(asked::THE_STEP_FIELD, commanded.to_string());
        // **The step's own inputs, and no address anywhere** — `Server Setup`
        // B4 derives the archive's address and its checksum's from the version,
        // on the machine.
        //
        // **Every step that reads the version is sent it, from the one list
        // both sides read.** Until 2026-09-22 this named the fetch and the
        // publication by hand and left the unpack out, so every Blender install
        // fetched its archive and then stopped at the unpack — step 10c's third
        // pass on Vast, with 5.2.2 and 5.1.2 alike.
        if let Some(version) = &asked_for.install_this_blender {
            if asked::THE_STEPS_THAT_READ_THE_BLENDER_VERSION.contains(&commanded) {
                fields = fields.and(asked::THE_BLENDER_VERSION_FIELD, version.clone());
            }
        }
        // **`Server Setup` B7: the user's own archive, by its bare name, on the
        // unpack and the publication** — and no version, since the build is
        // asked its own. **The user's answer to `Server Setup` B7a rides the publication
        // commanded again after a hold.** Added 2026-10-09, the 38's piece 14b.
        if let Some(name) = &self.the_users_own_archive {
            if commanded == asked::THE_UNPACK_STEP || commanded == asked::THE_BUILD_IS_PUBLISHED_STEP {
                fields = fields.and(asked::THE_USERS_OWN_ARCHIVE_FIELD, name.clone());
            }
        }
        // **`Server Setup` B15: the removal's own inputs** — the executable,
        // the answered version and `Server Setup` B14's covered jobs, the one set the warning
        // named (`Queue & History` L21). The 38's piece 14c.
        if commanded == asked::THE_BUILD_IS_REMOVED_STEP {
            if let Some(removal) = &self.the_removal {
                fields = the_removal_steps_fields(fields, removal);
            }
        }
        if commanded == asked::THE_BUILD_IS_PUBLISHED_STEP {
            if let Some(answer) = self.the_users_answer {
                fields = fields.and(asked::THE_USERS_ANSWER_FIELD, answer);
            }
        }
        if let Some(mark) = mark {
            fields = fields.and(
                asked::WHAT_THE_STEP_DID_FIELD,
                match mark {
                    WhatTheStepDid::Performed => asked::IT_PERFORMED,
                    WhatTheStepDid::NothingToDo => asked::IT_HAD_NOTHING_TO_DO,
                },
            );
        }
        let request = request.carrying(fields);
        let answered = ask(self.sessions, &self.package_area, &request).await;
        let Some(answered) = answered else {
            return Some(TheRunsEnding::TheServerStoppedAnswering {
                step,
                said: "Smoozy's service on the server did not respond to the step.".to_string(),
            });
        };
        if !it_is_still_in_hand(&answered) {
            return Some(TheRunsEnding::AStepFailed {
                step,
                said: the_machines_own_words(&answered),
            });
        }
        self.waited_for(step, commanded).await
    }

    /// **`Server Setup` Y7: the run is refreshed at its own short cadence while
    /// it is open, and the refresh is the run's own state reading.**
    ///
    /// **A step in flight answers *still in hand*** (`Server protocol and
    /// transfers` C8), **so the desktop learns a step finished from the reading
    /// it is already refreshing on.** Nothing here polls a second thing.
    async fn waited_for(&mut self, step: Step, commanded: &str) -> Option<TheRunsEnding> {
        let counted = TheCountedAct::of(commanded);
        let ended = self.waited_for_telling(step, commanded, counted).await;
        // **The figure goes with its step**, whichever way the step ended.
        if counted.is_some() {
            if let Some(go) = &self.the_figure_goes {
                go(None);
            }
        }
        ended
    }

    /// **[`Self::waited_for`]'s wait**, with `Server Setup` Y7b's and `Server Setup` Y7c's
    /// figure handed to the screen at every reading while the download or the
    /// unpack runs — the reading the refresh already takes, and no other.
    /// Added 2026-10-18, the Creator's fourteenth notes, piece U.
    async fn waited_for_telling(
        &mut self,
        step: Step,
        commanded: &str,
        counted: Option<TheCountedAct>,
    ) -> Option<TheRunsEnding> {
        let want = the_boundary_a_step_reaches(commanded);
        let waited = std::time::Instant::now();
        // **`Server Setup` Y7a item 2: what of this step's words is kept
        // here** — each piece once, as the readings bring it (2026-10-25,
        // the eighteenth notes, piece 2c).
        let mut held_here = TheStepsWordsHeldHere::default();
        // **`Server Setup` Y7d: the last count, so a moved one counts as
        // something new.**
        let mut the_last_count = None;
        let mut readings_for_the_rest = 0;
        loop {
            // **`Server Setup` Y7d's second is measured from this reading's
            // start** (2026-10-26, the worldwide pass's setup faults, fault 1):
            // a reading that took longer than the cadence is followed at once,
            // rather than by a further second on top of its own length.
            let this_reading_began = tokio::time::Instant::now();
            let state = self.the_runs_state_naming(&held_here).await;
            match state {
                None => {
                    return Some(TheRunsEnding::TheServerStoppedAnswering {
                        step,
                        said: "Smoozy's service on the server stopped responding."
                            .to_string(),
                    })
                }
                Some(state) => {
                    // **`Server Setup` Y7a item 2: the step's words, each piece
                    // kept as its reading lands** — while the step runs, and
                    // its last piece at the reading that shows it ended,
                    // whichever way it ended, before anything further is
                    // issued, so the run's words stay in the order the run
                    // printed them. Until 2026-10-25 a worker step's words
                    // were kept once, whole, only once it had ended.
                    let new_words = held_here.a_reading_is_kept(self.the_words_go.as_ref(), step, commanded, &state);
                    // **`Server Setup` Y7d: words moving, or a moved count,
                    // keep the refresh at about a second.**
                    let the_count_moved = state.the_count.is_some() && state.the_count != the_last_count;
                    the_last_count = state.the_count;
                    self.the_cadence.a_reading_brought(new_words || the_count_moved);
                    // **`Server Setup` Y7a item 5: a step that has ended with
                    // more of its words to come is read again at once**, so its
                    // last piece is kept before this side issues anything
                    // further.
                    let it_ended =
                        state.ending.is_some() || state.the_step_in_flight.as_deref() != Some(commanded);
                    if it_ended
                        && held_here.more_follow(commanded, &state)
                        && readings_for_the_rest < THE_MOST_READINGS_FOR_THE_REST_OF_A_STEPS_WORDS
                    {
                        readings_for_the_rest += 1;
                        continue;
                    }
                    // **`Server Setup` Y7b and `Server Setup` Y7c: how far it has got, as this
                    // reading says** — nothing where it says nothing, so a
                    // machine program older than the count shows the name
                    // alone.
                    if let (Some(act), Some(go)) = (counted, &self.the_figure_goes) {
                        go(state.the_count.map(|count| HowFarTheStepHasGot { act, count }));
                    }
                    // **The version the user's build answered, kept as the
                    // machine reports it** (`Server Setup` §12). Added
                    // 2026-10-16, step 10c's owed piece 5.
                    if state.the_version_answered.is_some() {
                        self.the_version_answered = state.the_version_answered.clone();
                    }
                    // **The machine's recorded reason, and not only the name of
                    // its ending.** `Server Setup` S2a writes the failed step's
                    // words into the run's ending, and the state reading
                    // carries them; until 2026-09-22 this handed on the ending's
                    // own word, so *What the server said* read *a step failed*
                    // while the record held the real sentence.
                    if let Some(ending) = state.ending.clone() {
                        return Some(TheRunsEnding::AStepFailed {
                            step,
                            said: state
                                .the_machines_own_words
                                .clone()
                                .filter(|words| !words.trim().is_empty())
                                .unwrap_or(ending),
                        });
                    }
                    // **`Server Setup` B7a: the publication held for a
                    // person** — nothing published, no boundary, the run open.
                    // The question is handed back to the step, which asks it.
                    // **And the decline's discard ends the step** without its
                    // boundary, which the machine struck from the list.
                    // Added 2026-10-09, the 38's piece 14b.
                    if commanded == asked::THE_BUILD_IS_PUBLISHED_STEP
                        && state.the_step_in_flight.as_deref() != Some(commanded)
                    {
                        if self.the_users_answer.is_none() {
                            if let Some(version) = state.the_question_stands_at.clone() {
                                self.the_question = Some(version);
                                return None;
                            }
                        }
                        if state.the_discard.is_some() {
                            return None;
                        }
                    }
                    match want {
                        Some(want) if state.reached.contains(&want) => return None,
                        // **A step that reaches no boundary of its own is
                        // waited on by the step under way** — the machine
                        // records it when the step is commanded and clears it
                        // on the worker's report. **`Server Setup` R1 item 9's
                        // final verification is the one such step**, because
                        // `Server Setup` Y4's *done* is the desktop's to write.
                        None if state.the_step_in_flight.as_deref() != Some(commanded) => {
                            return None
                        }
                        _ => {}
                    }
                }
            }
            if waited.elapsed() > HOW_LONG_ONE_STEP_IS_WAITED_FOR {
                return Some(TheRunsEnding::AStepFailed {
                    step,
                    said: "The step did not finish in time.".to_string(),
                });
            }
            // **`Server Setup` Y7d: about a second while words move, the
            // keepalive cadence after five quiet readings** — from the start of
            // the reading just taken.
            tokio::time::sleep_until(this_reading_began + self.the_cadence.until_the_next_reading()).await;
        }
    }

    /// **`Server Setup` B15's second route, the desk's half: the machine's own
    /// software manager asked to remove exactly the one package.** Added
    /// 2026-10-16, step 10c's owed piece 6.
    ///
    /// **The command is the pairing's own and no other**
    /// (`Server Setup`'s `search::the_manager_route_for`): a constant naming
    /// that manager and that package, because `Server protocol and transfers`
    /// 1.1 forbids a machine-read name in a command string. **A package
    /// outside the pairing sends nothing** and fails the step, which is
    /// `Server Setup` §13 item 21's own answer.
    ///
    /// **Check-first, so a re-run is safe** (`Server Setup` S7): where the
    /// executable is already gone from its folder, nothing is sent and the mark
    /// is *nothing to do*. Where the folder cannot be listed, the removal is
    /// sent and the manager answers for itself.
    async fn the_managers_removal(
        &mut self,
        step: Step,
        package: &str,
        executable: &str,
        elevation: HowElevationIsObtained<'_>,
        stop: &dyn AsksToStop,
    ) -> Result<WhatTheStepDid, TheRunsEnding> {
        let Some(pairing) = smoozy_server_setup::search::the_manager_route_for(package) else {
            return Err(TheRunsEnding::AStepFailed {
                step,
                said: format!("Removing the package {package} is not supported."),
            });
        };
        if !self.it_is_still_on_the_machine(executable).await {
            return Ok(WhatTheStepDid::NothingToDo);
        }
        match self.an_elevated_act(step, pairing.removal, elevation, stop).await {
            None => Ok(WhatTheStepDid::Performed),
            Some(ending) => Err(ending),
        }
    }

    /// **Whether a path is still in its folder on the machine**, read over the
    /// file-transfer subsystem, where every path is a protocol field. **A folder
    /// that cannot be listed is not read as empty**: the answer is then that it
    /// may still be there.
    async fn it_is_still_on_the_machine(&self, path: &str) -> bool {
        let Some((folder, name)) = path.rsplit_once('/') else {
            return true;
        };
        let folder = if folder.is_empty() { "/" } else { folder };
        let mut files = OnThisServersPoint {
            sessions: self.sessions,
        };
        match files.list_one_directory(folder).await {
            Ok(held) => held.iter().any(|thing| thing.name == name.as_bytes()),
            Err(_) => true,
        }
    }

    /// **An elevated step, performed over the command channel by the machine's
    /// own elevating command** (`Server Setup` Y7 and `Server Setup` E3).
    ///
    /// **`Server Setup` R1 item 5's driver step is check-first**, and the check
    /// is the step's own; the restart is `Server Setup` §8's and is issued only
    /// where the check failed after the driver step's work.
    async fn an_elevated_step(
        &mut self,
        step: Step,
        elevation: HowElevationIsObtained<'_>,
        stop: &dyn AsksToStop,
    ) -> Option<TheRunsEnding> {
        let Some(elevated) = the_elevated_step_for(step, self.needs.family) else {
            return None;
        };
        self.an_elevated_act(step, elevated, elevation, stop).await
    }

    /// **The one place an elevated act leaves this desk, and therefore the one
    /// place `Server Setup` Y1's state is set.**
    ///
    /// **`Server Setup` Y1: *"a run is machine-changing exactly where it
    /// performs an elevated package install, a driver act, or a restart"*, and
    /// the class is decided *"by the act a step performs, never by the step's
    /// number"*.** So the test is applied to the act, at the moment before the
    /// act, and the judgement is the act's own —
    /// `Server Setup`'s `elevated::WhatAnActDoesToTheMachine`, written beside
    /// each command. **Nothing here classes an act a second time**, which is
    /// what would make one fact live in two places.
    ///
    /// **Before, never after.** `Server Setup` Y1 sets the state *"before its
    /// first such step"* and `Rented-server runtime` P4 makes it durable
    /// *before the restart a setup step causes* — and the restart is one of
    /// these acts, so setting it here is what keeps P4 rather than a rule of
    /// its own. **`Server Setup` Y4a's mark cannot stand in for this**: it is
    /// written after the act.
    ///
    /// **A state that could not be set stops the act.** Going on would perform
    /// a machine-changing act with nothing holding the interrupted job back,
    /// which is Row 110's forbidden outcome.
    ///
    /// **`Server Setup` Y7's refresh and `Server Setup` Y8's stop go on while
    /// it runs** — see [`TheAdminStepsChannel`]. Added 2026-09-27, step 10c's
    /// eighth-pass fix piece 1.
    async fn an_elevated_act(
        &mut self,
        step: Step,
        elevated: AnElevatedStep,
        elevation: HowElevationIsObtained<'_>,
        stop: &dyn AsksToStop,
    ) -> Option<TheRunsEnding> {
        // **`Server Setup` E4's fourth bullet, decided before anything goes
        // out.** `Server Setup`'s own `elevated::issue` sends nothing for it
        // either; deciding it here as well keeps `Server Setup` Y1's state from
        // being set for an act that is never sent, so a declined offer leaves no
        // machine-level state standing. Added 2026-09-21.
        if elevation == HowElevationIsObtained::APasswordIsOwedAndNoneIsHeld {
            return Some(TheRunsEnding::AStepNeedsAdminAccess { step });
        }
        // **`Server Setup` Y7's second bullet, before anything of the act
        // goes out, the state included**: the step's start is recorded against
        // the run. **After E4's refusal above**, so a declined offer leaves no
        // step recorded as started. Added 2026-09-29.
        if let Some(ending) = self.the_elevated_steps_start_is_recorded(step).await {
            return Some(ending);
        }
        if elevated.what_it_does_to_the_machine.is_machine_changing() {
            if let Some(ending) = self.the_machine_level_state_is_set(step).await {
                return Some(ending);
            }
        }
        let mut channel = self.an_admin_steps_channel(step, stop);
        let went = elevated::issue(&mut channel, elevated, elevation).await;
        let seen = channel.seen();
        if let Some(ending) = self.what_the_admin_step_saw(step, seen).await {
            return Some(ending);
        }
        match went {
            Ok(said) if said.it_ended_cleanly => None,
            Ok(said) => Some(TheRunsEnding::AStepFailed {
                step,
                said: said.the_machines_own_words,
            }),
            Err(elevated::WhyTheStepDidNotRun::NoPasswordIsHeldYet) => {
                Some(TheRunsEnding::AStepNeedsAdminAccess { step })
            }
            Err(why) => Some(TheRunsEnding::AStepFailed {
                step,
                said: why.in_words().to_string(),
            }),
        }
    }

    /// **`Server Setup` E2's offer, with the run held while it stands.** Added
    /// 2026-09-21.
    ///
    /// **The refresh goes on at its own cadence while the question stands** —
    /// `Server Setup` Y7: *"it continues at its cadence for as long as the run
    /// is open"*, and `Server Setup` B7a is the shape for a run held on a human
    /// answer. **So a person taking their time does not end the run**, and a
    /// machine that stops answering does. **The user's stop is read at the same
    /// cadence**, since stopping is offered throughout.
    async fn the_password_is_asked_for(
        &mut self,
        step: Step,
        asker: &mut dyn AsksForThePassword,
        stop: &dyn AsksToStop,
    ) -> HowTheAskEnded {
        let answer = asker.the_password_for(step);
        tokio::pin!(answer);
        loop {
            tokio::select! {
                given = &mut answer => {
                    return match given {
                        Some(password) => HowTheAskEnded::Given(password),
                        None => HowTheAskEnded::Declined,
                    };
                }
                () = wait_a_moment(driving::THE_REFRESH_CADENCE) => {
                    if stop.the_user_asked_to_stop() {
                        return HowTheAskEnded::TheUserStoppedTheRun;
                    }
                    if self.the_runs_state().await.is_none() {
                        return HowTheAskEnded::TheMachineStoppedAnswering;
                    }
                }
            }
        }
    }

    /// **`Server Setup` B7a: the question, asked where the publication was
    /// held for a person, and the answer sent back as the publication
    /// commanded again.** Added 2026-10-09, the 38's piece 14b.
    ///
    /// **The run is held while it stands, the way the password offer holds
    /// it**: the refresh goes on at its cadence, so a person taking their time
    /// does not end the run, and the user's stop is read at the same cadence.
    /// **A decline strikes *blender* from this side's list too**, as the
    /// machine struck it from the run's, so the ending claims no build.
    async fn the_question_below_the_floor(
        &mut self,
        step: Step,
        asked_for: &WhatWasAskedFor,
        asker: &mut dyn AsksForThePassword,
        stop: &dyn AsksToStop,
    ) -> Option<TheRunsEnding> {
        let version = self.the_question.take()?;
        // **The version asked about is the one the build answered**, which a
        // decline's diary line names (`Server Setup` §12). Added 2026-10-16,
        // step 10c's owed piece 5.
        self.the_version_answered = Some(version.clone());
        let answer = asker.the_answer_below_the_floor(&version);
        tokio::pin!(answer);
        let install_anyway = loop {
            tokio::select! {
                given = &mut answer => break given,
                () = wait_a_moment(driving::THE_REFRESH_CADENCE) => {
                    if stop.the_user_asked_to_stop() {
                        return Some(TheRunsEnding::TheUserStoppedIt);
                    }
                    if self.the_runs_state().await.is_none() {
                        return Some(TheRunsEnding::TheServerStoppedAnswering {
                            step,
                            said: "Smoozy's service on the server stopped responding."
                                .to_string(),
                        });
                    }
                }
            }
        };
        self.the_users_answer = Some(if install_anyway {
            asked::INSTALL_ANYWAY
        } else {
            asked::THE_USER_DECLINED
        });
        let went = self
            .a_worker_step(step, asked::THE_BUILD_IS_PUBLISHED_STEP, asked_for, None)
            .await;
        if went.is_none() && !install_anyway {
            self.composed.retain(|boundary| *boundary != Boundary::Blender);
        }
        went
    }

    /// **`Server Setup` Y8's stop, written as the machine's own durable desired
    /// state — through `Rented-server runtime` X8's ordinary command path and
    /// not a second path invented for it.**
    ///
    /// *"The user's stop is the durable desired state, honoured at a boundary
    /// and never by a signal. The stop command writes *should be stopped*
    /// durably first."* **Until this was written the stop was the desktop's
    /// waiting and nothing else**: a user who stopped a setup stopped the
    /// desktop watching it and did not stop the machine, so the worker went on
    /// taking steps for as long as anything commanded them and only
    /// `Server Setup` Y7's refresh window ever ended the run.
    ///
    /// **`Server Setup` Y8a is respected rather than tested around.** *"Where
    /// the machine is down, which is the restart wait and nothing else, the
    /// durable write cannot be made and no request leaves at all — so the stop
    /// reaches the desktop's waiting alone."* **That is a property of the
    /// machine's unreachability**: this is not called from the wait at all, and
    /// where it is called and the machine does not answer, nothing was written
    /// and the run ends the same way. **No pending-command record is left to
    /// resolve**, because no request left (`Server protocol and transfers` C3
    /// and C20).
    ///
    /// **Sent once.** A second send would be a second durable write claiming
    /// nothing new, and the machine would answer it under
    /// `Server protocol and transfers` C8 anyway.
    ///
    /// **Nothing is read back and nothing waits on it.** What the stop leaves is
    /// `Server Setup` Y8b's: the machine-level state stands, the interrupted job
    /// stays held back, and `Server Setup` S4's recovery run is the route on.
    /// **A write the machine did not take is not an error to report either** —
    /// the user asked for the waiting to end and it ends.
    async fn the_stop_is_written(&mut self) {
        // **A write to the run, or a step commanded, so the record as last
        // read no longer stands** (2026-10-26, fault 4).
        self.the_record_as_last_read = None;
        if self.the_stop_was_written {
            return;
        }
        self.the_stop_was_written = true;
        let Ok(command) = Identity::issue(IdentityKind::Operation) else {
            return;
        };
        let request = Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            command,
            asked::THE_SETUP_RUN_IS_STOPPED_REQUEST,
        )
        .carrying(Fields::none().and(asked::THE_SETUP_RUN_FIELD, self.run.as_text()));
        let _ = ask(self.sessions, &self.package_area, &request).await;
    }

    /// **`Server Setup` Y7's second bullet, sent: *"the desktop records the
    /// step's start against the run"*** — named by the step's first boundary
    /// in this run's composed list, as `Server Setup` S2a's failed step is
    /// named. Added 2026-09-29, step 10c's transfer wave piece 12 (the
    /// eighth-pass map's piece 7).
    ///
    /// **Why.** An admin step's process is the connection's child, so the
    /// machine learns nothing of it from its own worker; **until this was sent
    /// the machine's record named no step in flight during an admin step**, and
    /// a run cut during the driver install read *Install the graphics driver —
    /// not started* at every later connection (`Server Setup` S1), which is
    /// what the eighth pass on Hyperstack saw.
    ///
    /// **What each answer does**, and the rule is that a start the machine
    /// could not take never stops a run the machine is still holding open:
    ///
    /// - **Recorded**: the act goes out.
    /// - **No answer at all**: the run ends as the server having stopped
    ///   answering, as a boundary write that is not answered does — nothing
    ///   machine-changing is sent to a machine that has gone quiet.
    /// - **Refused by name**, by a program placed before this request existed:
    ///   the act goes out, as it always did there.
    /// - **Refused otherwise**: the run's state is read. **A run the machine has
    ///   already ended is told as its own record holds it**
    ///   ([`the_recorded_ending`]), and nothing further is sent. **Otherwise
    ///   the act goes out** — a start that could not be written costs only
    ///   `Server Setup` S1's reading of this one step, which is exactly what it
    ///   cost before; ending a proceeding run over it would cost the user the
    ///   whole setup.
    ///
    /// **Nothing for a step that has no boundary in this run's list**, which no
    /// admin step is; and nothing for the restart, whose own *restart issued*
    /// is written before its command (`Server Setup` T3) and is its start.
    async fn the_elevated_steps_start_is_recorded(&mut self, step: Step) -> Option<TheRunsEnding> {
        // **A write to the run, or a step commanded, so the record as last
        // read no longer stands** (2026-10-26, fault 4).
        self.the_record_as_last_read = None;
        let boundary = self.the_first_boundary_of(step)?;
        let Ok(command) = Identity::issue(IdentityKind::Operation) else {
            return Some(TheRunsEnding::AStepFailed {
                step,
                said: "Could not generate a command ID.".to_string(),
            });
        };
        let request = Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            command,
            asked::THE_ELEVATED_STEPS_START_IS_RECORDED_REQUEST,
        )
        .carrying(
            Fields::none()
                .and(asked::THE_SETUP_RUN_FIELD, self.run.as_text())
                .and(asked::THE_BOUNDARY_FIELD, the_word_for_boundary(boundary)),
        );
        let Some(answered) = ask(self.sessions, &self.package_area, &request).await else {
            return Some(TheRunsEnding::TheServerStoppedAnswering {
                step,
                said: "Smoozy's service on the server did not respond. Nothing was changed."
                    .to_string(),
            });
        };
        if answered.fields.text(asked::WHAT_BECAME_OF_IT_FIELD)
            == Some(asked::IT_HAS_A_RECORDED_OUTCOME)
        {
            return None;
        }
        // **A refusal from a machine that has already ended the run is the
        // run's own ending**, read off the run's state and never out of the
        // refusal's words (`Security and licence` §13 rule 5).
        if answered.the_request_name_it_says_it_does_not_answer().is_none() {
            if let Some(state) = self.the_runs_state().await {
                if state.ending.is_some() {
                    return Some(the_recorded_ending(
                        step,
                        &state,
                        Some(the_machines_own_words(&answered)),
                    ));
                }
            }
        }
        None
    }

    /// **`Server Setup` S2a's desk half, sent: the run's ending is written onto
    /// the machine with the failed step named and its words beside it** —
    /// through `Rented-server runtime` X8's ordinary command path, the route
    /// [`Self::the_stop_is_written`] takes for the stop. Added 2026-09-21.
    ///
    /// *"An elevated step's process is the connection's child, so its streams
    /// land in the desktop's own loop and nowhere on the machine; the desktop,
    /// which classifies the failure with those words in hand, writes the ending
    /// with the words."* **Until this was written nothing did**: the run stayed
    /// open on the machine until `Server Setup` Y7's window ended it as *ended
    /// at its boundary, not completed*, with no words, and every later reading
    /// said the connection had dropped where the plan says *failed, with the
    /// words*. **That is Smoozy telling the user less than it knows.**
    ///
    /// **It is sent for every step this side ends the run on**, not only the
    /// elevated ones, because every such failure is held here and nowhere on
    /// the machine — and where the failure was Smoozy's own verification rather
    /// than a tool's refusal, the words are the recorded fact of that read,
    /// which is `Server Setup` S2a's third bullet. **A failure the machine already recorded is
    /// not written over**: the machine answers that the run is no longer open
    /// and keeps the ending it has, so a worker's own failure, a stop, or the
    /// window are never replaced by this.
    ///
    /// **Sent once, and nothing waits on its answer.** The run is over either
    /// way; a write the machine did not take leaves the machine's own record as
    /// it stands, which is what the next reading shows, and this side claims
    /// nothing about it.
    async fn the_failed_step_is_recorded(&mut self, step: Step, said: &str) {
        // **A write to the run, or a step commanded, so the record as last
        // read no longer stands** (2026-10-26, fault 4).
        self.the_record_as_last_read = None;
        let Some(first) = Boundary::IN_ORDER
            .into_iter()
            .find(|boundary| boundary.its_step() == step)
        else {
            return;
        };
        let Ok(command) = Identity::issue(IdentityKind::Operation) else {
            return;
        };
        let request = Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            command,
            asked::THE_FAILED_STEP_IS_RECORDED_REQUEST,
        )
        .carrying(
            Fields::none()
                .and(asked::THE_SETUP_RUN_FIELD, self.run.as_text())
                .and(asked::THE_FAILED_STEP_FIELD, the_word_for_boundary(first))
                .and(asked::THE_MACHINES_OWN_WORDS_FIELD, said),
        );
        let _ = ask(self.sessions, &self.package_area, &request).await;
    }

    /// **`Rented-server runtime` P3's first act, sent: the machine-level
    /// setup-in-progress state is set, naming this run.**
    ///
    /// **`Rented-server runtime` P1 makes the state name the operation identity
    /// of the run that set it**, which is this run's, and that name is what
    /// `Rented-server runtime` A11a admits the recovery's worker start on and
    /// what P8 decides a clear by.
    ///
    /// **Sent once.** A second send would be a second durable write claiming
    /// nothing new; the run knows it already stands.
    async fn the_machine_level_state_is_set(&mut self, step: Step) -> Option<TheRunsEnding> {
        // **A write to the run, or a step commanded, so the record as last
        // read no longer stands** (2026-10-26, fault 4).
        self.the_record_as_last_read = None;
        if self.the_state_stands {
            return None;
        }
        let Ok(command) = Identity::issue(IdentityKind::Operation) else {
            return Some(TheRunsEnding::AStepFailed {
                step,
                said: "Could not generate a command ID.".to_string(),
            });
        };
        let request = Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            command,
            asked::THE_SETUP_IN_PROGRESS_IS_SET_REQUEST,
        )
        .carrying(Fields::none().and(asked::THE_SETUP_RUN_FIELD, self.run.as_text()));
        let Some(answered) = ask(self.sessions, &self.package_area, &request).await else {
            return Some(TheRunsEnding::TheServerStoppedAnswering {
                step,
                said: "Smoozy's service on the server did not respond. Nothing was changed."
                    .to_string(),
            });
        };
        // **The machine answers with the run it set the state under.** An
        // answer carrying anything else is the store having refused, and the
        // act does not go out.
        if answered.fields.text(asked::THE_SETUP_RUN_FIELD) != Some(self.run.as_text().as_str()) {
            return Some(TheRunsEnding::AStepFailed {
                step,
                said: "The server did not record that a setup is in progress. Nothing was changed."
                    .to_string(),
            });
        }
        self.the_state_stands = true;
        None
    }

    /// **`Rented-server runtime` P3's second act, sent at `Server Setup` Y4's
    /// *done* and nowhere else** — *"the state (where set) is cleared and
    /// reconciliation ran"*.
    ///
    /// **`Rented-server runtime` P5 makes the clear two acts in one durable
    /// write**, the second being reconciliation, and both are the machine's:
    /// what this side does is name the run and read what happened.
    ///
    /// **`Server Setup` S6 closes the clearing set at two and this is the
    /// first.** The second — a recovery run the user asked to abandon the
    /// machine-changing work — sends the same request from
    /// [`a_setup_is_abandoned`], since 2026-09-22.
    ///
    /// **Nothing is sent for a run that set nothing.** An additive run has no
    /// state to clear (`Server Setup` Y1), and a clear sent for one would run
    /// reconciliation on the strength of nothing.
    async fn the_machine_level_state_is_cleared(&mut self) -> Option<TheRunsEnding> {
        // **A write to the run, or a step commanded, so the record as last
        // read no longer stands** (2026-10-26, fault 4).
        self.the_record_as_last_read = None;
        if !self.the_state_stands {
            return None;
        }
        if let Some(ending) =
            the_state_is_cleared_by(self.sessions, &self.package_area, &self.run).await
        {
            return Some(ending);
        }
        self.the_state_stands = false;
        None
    }

    /// Where the run stands now, as the machine's own record says.
    ///
    /// **Where the machine does not answer, the last boundary this side saw** —
    /// which is what a run ending over a machine that has just been restarted
    /// has, and it is a reading rather than a guess. **`Server Setup` S1 and S2
    /// are unchanged**: what the user meets is read fresh off the machine on
    /// the next connection, and this is only what the run says as it ends.
    async fn where_it_stands(&mut self) -> Boundary {
        match self.the_runs_state().await {
            Some(state) => {
                let at = state.reached.last().copied().unwrap_or(Boundary::Started);
                self.the_last_one_seen = Some(at);
                at
            }
            None => self.the_last_one_seen.unwrap_or(Boundary::Started),
        }
    }

    /// **The boundary the screen marks as under way while a step runs** — the
    /// first boundary of that step in the run's composed list. Added
    /// 2026-09-27, fix piece 9.
    ///
    /// **Nothing for two steps, and each for its own reason.** `Server Setup`
    /// R1 item 1's readings are taken before the run is composed, so nothing is
    /// under way there. **The restart step marks itself**, once *restart
    /// issued* is written and not before: the step begins with the driver
    /// check, and marking *Restarting the server* while only a check runs would
    /// say a restart had begun that may never be sent.
    fn the_first_boundary_of(&self, step: Step) -> Option<Boundary> {
        if matches!(step, Step::Readings | Step::Restart) {
            return None;
        }
        self.composed
            .iter()
            .copied()
            .find(|boundary| boundary.its_step() == step)
    }

    /// **The run, as the machine's own record holds it now, told to the screen
    /// with the step under way marked** — `Design/Server Setup § A long step`,
    /// *"Shown as its steps, with the one under way marked"*, fed from the
    /// boundaries the run already reaches. Added 2026-09-27, fix piece 9.
    ///
    /// **The record is read fresh** (`Server Setup` S1 and S2: never from
    /// anything remembered), and **the step under way is the one thing this
    /// side adds**, because it is this side that is about to drive it — the
    /// machine records a worker step as under way only once it is commanded,
    /// and an elevated step only once this side has recorded its start
    /// ([`Self::the_elevated_steps_start_is_recorded`], from 2026-09-29), which
    /// is after the screen is told. **A machine that does not answer is told
    /// nothing**, rather than the screen being handed a reading that was not
    /// taken.
    ///
    /// **Where the last state reading's record still stands — nothing written
    /// to the run since — it is the reading the screen is told from**, and no
    /// request goes out: it is the same request's answer a reading taken again
    /// would bring. Added 2026-10-26, the worldwide pass's setup faults, fault
    /// 4.
    async fn the_screen_is_told(
        &mut self,
        asker: &mut dyn AsksForThePassword,
        under_way: Option<Boundary>,
    ) {
        let read = match self.the_record_as_last_read.take() {
            Some(read) => Some(read),
            None => a_runs_record(self.sessions, &self.package_area, &self.run)
                .await
                .and_then(|(_, read)| read),
        };
        let Some(mut read) = read else {
            return;
        };
        read.the_step_in_flight = under_way;
        asker.the_run_is_as_read(&read);
    }

    /// **Where the run stands as it ends, read once, and the screen told it
    /// with no step under way** — so the last step reads *done* on the card the
    /// moment it is, and not only once the card is read again. Added
    /// 2026-10-26, the worldwide pass's setup faults, fault 4: on every server
    /// of the pass the final check was never marked *done* on the running list,
    /// which stood for the 27 to 36 seconds the card's own reading took.
    /// **Where the machine does not answer, the last boundary this side saw**,
    /// as [`Self::where_it_stands`] answers.
    async fn where_it_stands_as_the_screen_is_told(&mut self, asker: &mut dyn AsksForThePassword) -> Boundary {
        self.the_record_as_last_read = None;
        match a_runs_record(self.sessions, &self.package_area, &self.run).await {
            Some((state, read)) => {
                if let Some(mut read) = read {
                    read.the_step_in_flight = None;
                    asker.the_run_is_as_read(&read);
                }
                let at = state.reached.last().copied().unwrap_or(Boundary::Started);
                self.the_last_one_seen = Some(at);
                at
            }
            None => self.the_last_one_seen.unwrap_or(Boundary::Started),
        }
    }

    /// **`Server Setup` Y7's refresh, which is the run's own state reading.**
    pub async fn the_runs_state(&mut self) -> Option<TheRunsState> {
        the_state_of_the_run(self.sessions, &self.package_area, &self.run).await
    }

    /// **The same reading, naming what of the step in hand's words this side
    /// holds** (`Server Setup` Y7a item 2). Added 2026-10-25, piece 2c.
    async fn the_runs_state_naming(&mut self, held: &TheStepsWordsHeldHere) -> Option<TheRunsState> {
        let (state, record) =
            the_state_and_the_record_naming(self.sessions, &self.package_area, &self.run, held).await?;
        // **Kept for the screen's next telling** (2026-10-26, fault 4).
        self.the_record_as_last_read = record;
        Some(state)
    }

    /// **The channel every admin step of this run goes out on** — kept alive
    /// by `Server Setup` Y7's refresh and read for `Server Setup` Y8's stop
    /// while each command runs. Added 2026-09-27, step 10c's eighth-pass fix
    /// piece 1. **Not the restart's** ([`Self::the_restart_command_is_sent`]):
    /// the machine going down is that command's success.
    fn an_admin_steps_channel<'a>(&self, step: Step, stop: &'a dyn AsksToStop) -> TheAdminStepsChannel<'s, 'a, C> {
        // **`Server Setup` Y7d: when the command's streams last delivered
        // bytes**, stamped as each piece goes to the screen, so the refresh
        // can tell words are moving. Added 2026-10-25, piece 2c.
        let the_last_piece = std::sync::Arc::new(std::sync::Mutex::new(None::<std::time::Instant>));
        let stamping = the_last_piece.clone();
        let words = self.the_words_go.clone().map(|go| {
            let stamped: TheWordsGoHere = std::sync::Arc::new(move |step: Step, printed: &WhatTheServerPrinted| {
                if let Ok(mut at) = stamping.lock() {
                    *at = Some(std::time::Instant::now());
                }
                go(step, printed)
            });
            (step, stamped)
        });
        let cadence = driving::TheRefreshCadence::as_a_run_opens();
        TheAdminStepsChannel {
            sessions: self.sessions,
            words,
            package_area: self.package_area.clone(),
            run: self.run.clone(),
            stop,
            the_next_refresh: tokio::time::Instant::now() + cadence.until_the_next_reading(),
            seen: None,
            cadence,
            the_last_piece,
        }
    }

    /// **What an admin step came to where its channel saw the run end or the
    /// user stop** — nothing where it saw neither. Added 2026-09-27, step
    /// 10c's eighth-pass fix piece 1.
    ///
    /// **A stop is `Server Setup` Y8's, written as the machine's own durable
    /// desired state first**; the channel of the command in flight is already
    /// closed and nothing further was sent. **A run the machine ended is told
    /// as its own record holds it** ([`the_recorded_ending`]).
    async fn what_the_admin_step_saw(
        &mut self,
        step: Step,
        seen: Option<WhatTheAdminStepSaw>,
    ) -> Option<TheRunsEnding> {
        match seen? {
            WhatTheAdminStepSaw::TheUserStopped => {
                self.the_stop_is_written().await;
                Some(TheRunsEnding::TheUserStoppedIt)
            }
            WhatTheAdminStepSaw::TheRunEnded(state) => {
                if let Some(last) = state.reached.last() {
                    self.the_last_one_seen = Some(*last);
                }
                Some(the_recorded_ending(step, &state, None))
            }
        }
    }
}

/// **`Server Setup` Y7's refresh, which is the run's own state reading** —
/// the one request, whoever takes it. Moved out of
/// [`TheRunBeingDriven::the_runs_state`] on 2026-09-27 so the admin steps'
/// channel ([`TheAdminStepsChannel`]) takes the same reading.
async fn the_state_of_the_run<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    package_area: &str,
    run: &Identity,
) -> Option<TheRunsState> {
    the_state_of_the_run_naming(sessions, package_area, run, &TheStepsWordsHeldHere::default()).await
}

/// **[`the_state_of_the_run`], naming what of the step in hand's words this
/// side already holds** — `Server Setup` Y7a item 2's *"the reading request
/// names those two lengths"*. Added 2026-10-25, the eighteenth notes, piece
/// 2c.
async fn the_state_of_the_run_naming<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    package_area: &str,
    run: &Identity,
    held: &TheStepsWordsHeldHere,
) -> Option<TheRunsState> {
    the_state_and_the_record_naming(sessions, package_area, run, held)
        .await
        .map(|(state, _)| state)
}

/// **[`the_state_of_the_run_naming`], with the run's record read off the same
/// answer** — one request, two readings, as [`a_runs_record`] takes them.
/// Added 2026-10-26, the worldwide pass's setup faults, fault 4.
async fn the_state_and_the_record_naming<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    package_area: &str,
    run: &Identity,
    held: &TheStepsWordsHeldHere,
) -> Option<(TheRunsState, Option<TheSetupAsRead>)> {
    let Ok(identity) = Identity::issue(IdentityKind::Operation) else {
        return None;
    };
    let request = Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        identity,
        asked::THE_SETUP_RUNS_STATE_REQUEST,
    )
    .carrying(held.the_reading_names(Fields::none().and(asked::THE_SETUP_RUN_FIELD, run.as_text())));
    let answered = ask(sessions, package_area, &request).await?;
    Some((the_state_in(&answered), the_setup_as_the_machine_answered(&answered)))
}

/// **The ending a run's own record holds, told for the step in hand** —
/// `Design/Server Setup § Cut off part-way` rather than a failure of that
/// step. Added 2026-09-27, step 10c's eighth-pass fix piece 1.
///
/// **The words are the machine's**: its recorded words where it wrote any,
/// and otherwise what it answered the write it refused (`refused_with`).
/// **Nothing is read out of them** (`Security and licence` §13 rule 5).
fn the_recorded_ending(step: Step, state: &TheRunsState, refused_with: Option<String>) -> TheRunsEnding {
    let said = state
        .the_machines_own_words
        .clone()
        .filter(|words| !words.trim().is_empty())
        .or(refused_with);
    TheRunsEnding::TheServerEndedTheRun { step, said }
}

// ════════════════════════════════════════════════════════════════════════════
// `Server Setup` S6's second clear — *Abandon setup*
// ════════════════════════════════════════════════════════════════════════════

/// **What an abandon came to.** Added 2026-09-22.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheAbandonCameTo {
    /// **No setup-in-progress state stands on the machine**, so there was no
    /// stopped machine-changing setup to give up and nothing was sent.
    NothingStoodToAbandon,
    /// **Abandoned: the final verification ran, the run ended *abandoned*, and
    /// the state is cleared** — which ran reconciliation (`Rented-server
    /// runtime` P5). **What the verification found rides beside it**: the
    /// machine's words where it did not pass, nothing where it passed.
    Abandoned {
        what_the_final_check_found: Option<String>,
    },
    /// **It did not get as far as the clear**, and the ending says where.
    ItDidNotFinish(TheRunsEnding),
}

/// **`Server Setup` S6's second clear, performed: *"a recovery run the user
/// asked to abandon the machine-changing work — which still runs the final
/// verification first and reports what it found."*** Added 2026-09-22, out of
/// step 10c's third pass: a driver step that failed the same way on every *Set
/// up again* left the machine held for good, and *Abandon setup* — drawn,
/// worded and on the panel's own composition — sent nothing.
///
/// **The order, and each part is a clause.**
///
/// 1. **The run the state names**, read off `Rented-server runtime` V9's field
///    on the health answer. **Where none stands there is nothing to abandon**
///    and nothing is sent — `Server Setup` S6 is a way to clear the state, and an abandon that
///    cleared nothing would be a run for no reason.
/// 2. **A recovery of that run, marked as an abandon** — `Server Setup` S4 item
///    2's take-over, so the state is re-pointed and never cleared in between.
/// 3. **`Server Setup` R1 item 9's final verification, and nothing else.** The
///    machine ends the run *abandoned* on its report, with the verification's
///    words where it did not pass.
/// 4. **`Rented-server runtime` P3's second act**, the same clear a run reaching
///    *done* sends. **No boundary is written**, *done* least of all, so
///    `Server Setup` Y6a's card then reads *Not set up*.
///
/// ~~**`Server Setup` S5's wait is not built here**, as it is not built for *Set
/// up again*: a recovery the machine refuses because the stopped run's group
/// still reads as live comes back as a refusal in the machine's own words.~~
/// **Built 2026-09-22, for both, in [`a_run_is_opened`]**: the abandon waits
/// while the machine says the run that stopped is still running. Nothing here
/// can stop that wait, because the abandon has no stop control; it ends at
/// [`HOW_LONG_THE_RUN_THAT_STOPPED_IS_WAITED_FOR`].
pub async fn a_setup_is_abandoned<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    package_area: &str,
    told: &mut dyn AsksForThePassword,
) -> WhatTheAbandonCameTo {
    let Some(standing) = the_run_the_state_stands_for(sessions, package_area).await else {
        return WhatTheAbandonCameTo::NothingStoodToAbandon;
    };
    // **`Server Setup` Y6b: a recovery takes the taken-over run's kind**, and the
    // screen asks the machine again by that kind.
    let kind = the_setup_as_read(sessions, package_area, Some(standing.clone()))
        .await
        .map(|read| read.record.kind)
        .unwrap_or(SetupKind::TheReadyAction);
    let run = match a_run_is_opened(
        sessions,
        package_area,
        WhatOpensTheRun::AnAbandonOf(standing),
        &[],
        &NobodyAsked,
        told,
    )
    .await
    {
        Ok(run) => run,
        Err(ending) => return WhatTheAbandonCameTo::ItDidNotFinish(ending),
    };
    told.a_run_stands(kind, &run);
    // **`Server Setup` Y7a item 2: the final check's words are among this
    // run's words**, as every other worker step's are — asked once the run
    // stands, so an abandon that found nothing to abandon, or opened no run,
    // leaves the last run's words where they were. Added 2026-10-25, the
    // eighteenth notes, note 5 (the sixteenth notes' *Not built: the abandon*).
    let the_words_go = told.where_the_words_go();
    // **Each piece kept as its reading lands, and the readings about a second
    // apart while words move** (`Server Setup` Y7a item 2, `Server Setup` Y7d; 2026-10-25,
    // the eighteenth notes, piece 2c).
    let mut held_here = TheStepsWordsHeldHere::default();
    let mut cadence = driving::TheRefreshCadence::as_a_run_opens();
    let mut readings_for_the_rest = 0;

    let step = Step::Done;
    let Ok(command) = Identity::issue(IdentityKind::Operation) else {
        return WhatTheAbandonCameTo::ItDidNotFinish(TheRunsEnding::AStepFailed {
            step,
            said: "Could not generate a command ID.".to_string(),
        });
    };
    let request = Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        command,
        asked::THE_ONE_SETUP_STEP_REQUEST,
    )
    .carrying(
        Fields::none()
            .and(asked::THE_SETUP_RUN_FIELD, run.as_text())
            .and(asked::THE_STEP_FIELD, asked::THE_FINAL_VERIFICATION_STEP),
    );
    match ask(sessions, package_area, &request).await {
        Some(answered) if it_is_still_in_hand(&answered) => {}
        Some(answered) => {
            return WhatTheAbandonCameTo::ItDidNotFinish(TheRunsEnding::AStepFailed {
                step,
                said: the_machines_own_words(&answered),
            })
        }
        None => {
            return WhatTheAbandonCameTo::ItDidNotFinish(TheRunsEnding::AStepFailed {
                step,
                said: "Smoozy's service on the server did not respond to the step.".to_string(),
            })
        }
    }

    // **The run's own state reading, at `Server Setup` Y7's cadence, until the
    // machine records the ending** — which it writes on the worker's report.
    let waited = std::time::Instant::now();
    let found = loop {
        // **From this reading's start**, as every step's wait measures it
        // (2026-10-26, fault 1).
        let this_reading_began = tokio::time::Instant::now();
        let Ok(identity) = Identity::issue(IdentityKind::Operation) else {
            return WhatTheAbandonCameTo::ItDidNotFinish(TheRunsEnding::AStepFailed {
                step,
                said: "Could not generate a command ID.".to_string(),
            });
        };
        let reading = Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            identity,
            asked::THE_SETUP_RUNS_STATE_REQUEST,
        )
        .carrying(held_here.the_reading_names(Fields::none().and(asked::THE_SETUP_RUN_FIELD, run.as_text())));
        if let Some(answered) = ask(sessions, package_area, &reading).await {
            // **Each piece kept as its reading lands, the last before anything
            // is said of what it found** — the same keeping
            // `waited_for_telling` does for every step of a setup.
            let state = the_state_in(&answered);
            let new_words =
                held_here.a_reading_is_kept(the_words_go.as_ref(), step, asked::THE_FINAL_VERIFICATION_STEP, &state);
            cadence.a_reading_brought(new_words);
            if answered.fields.text(asked::THE_RUNS_ENDING_FIELD).is_some()
                && held_here.more_follow(asked::THE_FINAL_VERIFICATION_STEP, &state)
                && readings_for_the_rest < THE_MOST_READINGS_FOR_THE_REST_OF_A_STEPS_WORDS
            {
                readings_for_the_rest += 1;
                continue;
            }
            match answered.fields.text(asked::THE_RUNS_ENDING_FIELD) {
                Some(ending) if ending == asked::IT_WAS_ABANDONED => {
                    break answered
                        .fields
                        .text(asked::THE_MACHINES_OWN_WORDS_FIELD)
                        .map(str::to_string);
                }
                Some(other) => {
                    return WhatTheAbandonCameTo::ItDidNotFinish(TheRunsEnding::AStepFailed {
                        step,
                        said: answered
                            .fields
                            .text(asked::THE_MACHINES_OWN_WORDS_FIELD)
                            .unwrap_or(other)
                            .to_string(),
                    })
                }
                None => {}
            }
        }
        if waited.elapsed() > HOW_LONG_ONE_STEP_IS_WAITED_FOR {
            return WhatTheAbandonCameTo::ItDidNotFinish(TheRunsEnding::AStepFailed {
                step,
                said: "The final check did not finish in time.".to_string(),
            });
        }
        tokio::time::sleep_until(this_reading_began + cadence.until_the_next_reading()).await;
    };

    if let Some(ending) = the_state_is_cleared_by(sessions, package_area, &run).await {
        return WhatTheAbandonCameTo::ItDidNotFinish(ending);
    }
    WhatTheAbandonCameTo::Abandoned {
        what_the_final_check_found: found,
    }
}

/// **[`a_setup_is_abandoned`] on one connected server**, off the drawing thread
/// (P5) and blocking the thread it is given — the shape of
/// [`the_ready_action_on`]. Added 2026-09-22.
pub fn the_setup_is_abandoned_on<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    point: &ThisServersSessions,
    server: &str,
) -> WhatTheAbandonCameTo {
    use tauri::Manager;
    let not_started = |said: &str| {
        WhatTheAbandonCameTo::ItDidNotFinish(TheRunsEnding::NoRunCouldBeOpened {
            said: said.to_string(),
        })
    };
    let Some(started) = app.try_state::<std::sync::Mutex<crate::start::Started>>() else {
        return not_started("Smoozy's database is not available.");
    };
    let record = crate::keys::TheRunsRecord {
        started: started.inner(),
    };
    let mut row = TheServersRowHere {
        record: &record,
        server: server.to_string(),
    };
    let Some(package_area) = row.the_row().as_ref().and_then(the_package_area_of) else {
        return not_started("No Smoozy folder is recorded for this server.");
    };
    let Ok(runtime) = tokio::runtime::Builder::new_current_thread().enable_all().build() else {
        return not_started("Could not start the task.");
    };
    runtime.block_on(a_setup_is_abandoned(
        point,
        &package_area,
        &mut TheSetUpTabAsks {
            app: app.clone(),
            server: server.to_string(),
        },
    ))
}

/// **`Rented-server runtime` P3's second act, sent by the run the state names**
/// — `Server Setup` S6's two clears share it: a run reaching *done*, and a
/// recovery the user asked to abandon the machine-changing work. **Nothing
/// where the machine says the state is gone**; otherwise what stood in the way,
/// in words.
async fn the_state_is_cleared_by<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    package_area: &str,
    run: &Identity,
) -> Option<TheRunsEnding> {
    let step = Step::Done;
    let Ok(command) = Identity::issue(IdentityKind::Operation) else {
        return Some(TheRunsEnding::AStepFailed {
            step,
            said: "Could not generate a command ID.".to_string(),
        });
    };
    let request = Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        command,
        asked::THE_SETUP_IN_PROGRESS_IS_CLEARED_REQUEST,
    )
    .carrying(Fields::none().and(asked::THE_SETUP_RUN_FIELD, run.as_text()));
    let Some(answered) = ask(sessions, package_area, &request).await else {
        return Some(TheRunsEnding::TheServerStoppedAnswering {
            step,
            said: "Smoozy's service on the server did not respond to the clear. Work on the \
                   server is still held back."
                .to_string(),
        });
    };
    if answered
        .fields
        .text(asked::THE_SETUP_RUN_THE_CLEAR_REMOVED_FIELD)
        != Some(run.as_text().as_str())
    {
        // **`Rented-server runtime` P8's report where the state names another
        // run, and trouble where it carries neither name.** Either way the
        // state stands, so *done* is not written.
        return Some(TheRunsEnding::AStepFailed {
            step,
            said: match answered.fields.text(asked::THE_SETUP_RUN_FIELD) {
                Some(whose) => format!(
                    "The server is holding its work back for another setup run ({whose}). This \
                     one did not finish."
                ),
                None => "The server is still holding its work back. This setup did not finish."
                    .to_string(),
            },
        });
    }
    None
}

/// **How long one step is waited on before the run is called stopped.**
///
/// **Not a figure of any plan's.** `Server Setup` Y7's own window is the
/// machine's — the gap between two refreshes — and this is the desktop's own
/// bound on one step, so a step that never reaches its boundary does not leave
/// a user watching a run for ever. **It is generous on purpose**: the fetch and
/// the unpack are the long ones, and `Server Setup` R1 item 3 timed the unpack
/// at 62 to 67 seconds on four cores with the download at 4 to 5 seconds, on
/// one box. Twenty minutes is far past both and far past the driver install's
/// observed 149 seconds plus a restart.
pub const HOW_LONG_ONE_STEP_IS_WAITED_FOR: Duration = Duration::from_secs(20 * 60);

/// **`Server Setup` Y8's stop, asked between two steps — and, from 2026-09-27,
/// at the run's cadence while an admin step runs** ([`TheAdminStepsChannel`]).
///
/// **A seam, because what holds the user's asking is the screen's** — this file
/// reads it at a boundary and never sends a signal
/// (`Server Setup`'s own `driving::a_signal_is_sent_to_the_worker_or_its_children`).
/// **Shared between threads**, because the admin step's channel reads it inside
/// a command the point may run on any thread of the runtime.
pub trait AsksToStop: Sync {
    fn the_user_asked_to_stop(&self) -> bool;
}

/// **`Server Setup` E2: the offer, made at the moment a step is refused for
/// lack of permission and never before.** Added 2026-09-21.
///
/// **A seam, because the asking is the screen's** — `Design/Server Setup §
/// Admin access`'s box, which names the step that was refused. **What comes back
/// is what the user typed, or nothing where they declined**, and nothing here
/// ever holds it longer than the run it was asked for (`Server Setup` E5 names
/// the lifetimes; see [`TheRunBeingDriven::driven`] for which of them this file
/// keeps).
///
/// **The run is held while the question stands**, and held the way
/// `Server Setup` B7a holds a run on a human answer: the refresh goes on at its
/// cadence, so `Server Setup` Y7's window does not end a run that is waiting on
/// a person.
pub trait AsksForThePassword {
    /// **`Server Setup` S5 and `Server Setup` Y9: the previous run still holds the server.**
    fn the_cut_run_is_being_waited_for(&mut self, _waiting: bool) {}

    fn the_password_for(&mut self, step: Step) -> ItsOwnFuture<'_, Option<Origin>>;

    /// **The run this screen stands behind, as it now is** — told at the
    /// run's start and after every step, so a recovery's take-over is told
    /// too. **What is done with it is the screen's**: the Set up tab keeps the
    /// identity for as long as this copy runs, and reads that run back off the
    /// machine (`Server Setup` S1, S2). Added 2026-09-21.
    fn a_run_stands(&mut self, _kind: SetupKind, _run: &Identity) {}

    /// **Every setup run this copy has driven on this server, most recent
    /// first** — the identities and nothing else. Added 2026-09-22, step 10c's
    /// fourth pass on Vast, fault 6.
    ///
    /// **What it is for is `Server Setup` S3's *or when the user runs the setup
    /// again*.** A run that stopped at a boundary and set no machine-level
    /// state — `Server Setup` Y1's additive class, which a Blender install on a
    /// machine that already has the unpack tool is — is refused by nothing when
    /// the setup is run again, so the press opened a fresh run and the stopped
    /// one was forgotten with its fetched archive still on the disk. **The
    /// screen is the only thing that knows which runs this copy drove**, so it
    /// is what hands them over.
    fn the_runs_this_copy_drove_here(&self) -> Vec<Identity> {
        Vec::new()
    }

    /// **The run, read off the machine now, with the step under way marked** —
    /// told before each step and once the restart is under way, so the screen
    /// can draw `Design/Server Setup § A long step`'s list. **What is done
    /// with it is the screen's.** Added 2026-09-27, fix piece 9.
    fn the_run_is_as_read(&mut self, _read: &TheSetupAsRead) {}

    /// **`Server Setup` B7a's question, asked of the user**: the user's build
    /// answered `version`, below 4.5. **True is *Install anyway*; false is
    /// the decline.** Where nobody is there to ask, it is declined — nothing
    /// below the floor is ever installed without a person saying so. Added
    /// 2026-10-09, the 38's piece 14b.
    fn the_answer_below_the_floor(&mut self, _version: &str) -> ItsOwnFuture<'_, bool> {
        Box::pin(async { false })
    }

    /// **`Server Setup` Y7a: where this run's words go** — asked once per run,
    /// which is the next run, so the screen lets the last run's words go here.
    /// **Nothing where no screen keeps them**, and then nothing is read before
    /// a command ends. Added 2026-10-18, the Creator's thirteenth notes, piece
    /// H. **From 2026-10-24 it is asked before the readings that compose the
    /// run**, and every step's words go here — the readings' and the elevated
    /// steps' as they print, a worker step's once it has ended (the sixteenth
    /// notes, note 1).
    fn where_the_words_go(&mut self) -> Option<TheWordsGoHere> {
        None
    }

    /// **`Server Setup` R1b's swap: where its own words go**, apart from the
    /// setup's, which it never empties (`Server Setup` Y7a as changed
    /// 2026-10-30, the twenty-third notes, note 3). **Nothing where no screen
    /// keeps them.**
    fn where_the_swaps_words_go(&mut self) -> Option<TheWordsGoHere> {
        None
    }

    /// **`Server Setup` Y7b and `Server Setup` Y7c: where how far the download and the unpack
    /// have got goes** — asked once as the run is driven, with the build the
    /// run downloads where it downloads one (as it was asked for, which the
    /// screen names as the list named it). **Nothing where no screen draws
    /// it.** Added 2026-10-18, the Creator's fourteenth notes, piece U.
    fn where_the_figure_goes(&mut self, _the_build: Option<&str>) -> Option<TheFigureGoesHere> {
        None
    }
}

/// **Nobody to ask**: every offer is declined. For a run with no screen behind
/// it, and for the desk's own tests of what a decline does.
pub struct NobodyIsThereToAsk;

impl AsksForThePassword for NobodyIsThereToAsk {
    fn the_password_for(&mut self, _step: Step) -> ItsOwnFuture<'_, Option<Origin>> {
        Box::pin(async { None })
    }
}

/// **How the wait on the offer ended** — four ways, each its own.
enum HowTheAskEnded {
    Given(Origin),
    Declined,
    TheUserStoppedTheRun,
    TheMachineStoppedAnswering,
}

/// **`Telegram` §20's list, composed for one machine — a seam, because the
/// list is made of things this driving does not hold.**
///
/// **`Server Setup` G1 places and does not decide**: *"the material is
/// `Telegram` §20's list whole"*, and that document decides what the three
/// things hold — the token from the secret store, the proved conversation, the
/// machine's own identity and label, the reporting settings. All of those are
/// the desktop's record and its store, so the composing sits where they are and
/// this driving is handed the answer.
///
/// **The token never reaches the driving.** What comes back shows nothing of it,
/// writes over its bytes when it goes, and is handed straight to the placement.
pub trait ComposesTheTelegramMaterial {
    /// **§20's list, composed.** **The package area is not handed in**: the
    /// composing side takes it from the root this run recorded at the hand-over
    /// (`Server Setup` R7b), which is one fact in one place. **The step checks
    /// what comes back landed under the area it placed**, which is
    /// `Security and licence` §10's own requirement rather than a matter of
    /// trust.
    fn composed(&mut self) -> Result<TheMaterial, String>;

    /// **`Telegram` L1's told-record bookkeeping, which is that document's.**
    /// What the machine said about each file is handed over; a file it did not
    /// take moves no telling.
    fn the_placement_was_answered(
        &mut self,
        material: &TheMaterial,
        placed: &[(String, acts::HowTheActWent)],
    );

    /// **`Telegram` L1 and L2: the role's desired state, *enabled*, was
    /// acknowledged by the machine**, and the told-record moves on it. Added
    /// by the 38's piece 11; a composer with no record keeps nothing.
    fn the_desired_state_was_acknowledged(&mut self) {}

    /// **`Telegram` L4's sentence of 2026-10-12: whether the role is
    /// enabled on this machine**, as the told-record has it under the installation the machine
    /// answers under now — an *enabled* the machine acknowledged. **Where it
    /// is, the placement is followed by *disabled* then *enabled***, because a
    /// running worker reads its material only at its start (`Telegram` K2).
    /// A composer with no record knows of no running worker. Added 2026-10-12,
    /// the Telegram worker's piece 7.
    fn the_role_is_enabled_there(&mut self) -> bool {
        false
    }

    /// **The *disabled* of that pair was acknowledged**, and the told-record
    /// moves on it, so a pair cut off before its *enabled* leaves the server
    /// owed the switch rather than recorded as on (`Telegram` L1, L2).
    fn the_disabled_state_was_acknowledged(&mut self) {}
}

/// **A run on a machine the user has not asked for Telegram on**, which is
/// every run whose composition holds no `Server Setup` R1 item 8 at all.
///
/// **It composes nothing and is never asked to**: `Server Setup` G2 places the
/// material only where Telegram is on, so the step is not in the run and this is
/// never reached. It exists so a caller that has no Telegram has something to
/// hand in rather than an option to unwrap.
pub struct TelegramIsNotOn;

impl ComposesTheTelegramMaterial for TelegramIsNotOn {
    fn composed(&mut self) -> Result<TheMaterial, String> {
        Err("Telegram is not switched on for this copy of Smoozy".to_string())
    }

    fn the_placement_was_answered(
        &mut self,
        _material: &TheMaterial,
        _placed: &[(String, acts::HowTheActWent)],
    ) {
    }
}

/// A run nobody has asked to stop.
pub struct NobodyAsked;

impl AsksToStop for NobodyAsked {
    fn the_user_asked_to_stop(&self) -> bool {
        false
    }
}

/// The run's state, as the machine's own record says it.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TheRunsState {
    /// `Server Setup` Y4's composed list, as the machine holds it.
    pub composed: Vec<Boundary>,
    /// The boundaries reached, in order.
    pub reached: Vec<Boundary>,
    /// `Server Setup` Y4's ending, where the run has one.
    pub ending: Option<String>,
    /// **The step under way, from the moment it was commanded** — Row 358's
    /// *the one under way marked*, and what a step that reaches no boundary of
    /// its own is waited on by.
    pub the_step_in_flight: Option<String>,
    /// **`Server Setup` S2a's words for the step that failed**, where the run
    /// ended with some. Added 2026-09-22.
    pub the_machines_own_words: Option<String>,
    /// **`Server Setup` Y6b's kind**, as the machine's record holds it — which
    /// on a recovery is the taken-over run's. Added 2026-09-22.
    pub kind: Option<SetupKind>,
    /// **`Server Setup` S2a: the step a failed run's ending names**, as the
    /// first boundary of that step. Added 2026-09-27.
    pub the_failed_step: Option<Boundary>,
    /// **`Server Setup` R1 item 4's whole not-found list**, as the run's last
    /// library read reported it, in the order the worker printed it.
    ///
    /// **Empty is a reading rather than an absence** — *"clean means the list
    /// is empty"* — and what tells the two apart is which step the run has
    /// reached, not this field.
    pub the_libraries_not_found: Vec<String>,
    /// **Where the open run's staging area is on the machine** — what the
    /// user's own archive is uploaded into (`Server Setup` B7, the runner's
    /// ruling on piece 14). Added 2026-10-09, the 38's piece 14b.
    pub the_staging_area: Option<String>,
    /// **`Server Setup` B7a's question, while it stands**: the version below
    /// 4.5 the user's build answered. Added 2026-10-09, the 38's piece 14b.
    pub the_question_stands_at: Option<String>,
    /// **`Server Setup` B7a's recorded discard**, once the user declined.
    /// Smoozy's own words, never drawn as they stand.
    pub the_discard: Option<String>,
    /// **`Server Setup` B1's answer, as the user's build gave it in staging**
    /// — the version it was published under or declined at — which
    /// `Server Setup` §12's *a Blender was installed* names. Added 2026-10-16,
    /// step 10c's owed piece 5.
    pub the_version_answered: Option<String>,
    /// **`Server Setup` Y7b and `Server Setup` Y7c: how far the step in hand has got, as the
    /// worker last counted it** — nothing where the reading carries no count,
    /// which is every step but the download and the unpack, and every reading
    /// from a machine program older than the count. Added 2026-10-18, the
    /// Creator's fourteenth notes, piece U.
    pub the_count: Option<TheStepsCount>,
    /// **`Server Setup` Y7a item 2: the last worker step's words, as the
    /// machine's program holds them for the run** — from the step's report
    /// until the run's next step is commanded. Nothing on a reading from a
    /// later connection or an older machine program. Added 2026-10-24, the
    /// Creator's ruling on the sixteenth notes, note 1.
    pub the_steps_words: Option<TheStepsWords>,
}

/// **A worker step's words, as the run's state reading carries them.**
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TheStepsWords {
    /// **The worker step they belong to**, in the wire's own word.
    pub step: String,
    /// **Its two streams, each every command's in turn** — or nothing where
    /// the reading carried `Server Setup` Y7a item 5's mark in their place.
    ///
    /// **From a machine program that carries a turn** (2026-10-25, the
    /// eighteenth notes, piece 2b), **these are pieces**: what the step printed
    /// after the lengths this side named, while it runs as much as once it
    /// has ended, and the mark rides beside them in
    /// [`Self::the_rest_was_not_kept`].
    pub printed: Option<(String, String)>,
    /// **Whose words the pieces are** — the step command's turn — where the
    /// machine's program hands them on as they print. Nothing from a program
    /// older than that, whose words are the finished step's, whole.
    pub turn: Option<String>,
    /// **`Server Setup` Y7a item 5: more of the step's words follow** what
    /// this reading carried.
    pub more_follow: bool,
    /// **The rest of the step's words were not kept** on the machine.
    pub the_rest_was_not_kept: bool,
}

/// **`Server Setup` Y7a item 2's words, read off the run's state** — nothing
/// where the reading names no step for them, or carries neither the words nor
/// the mark.
fn the_steps_words_in(answered: &Answered) -> Option<TheStepsWords> {
    let step = answered.fields.text(asked::THE_STEP_THE_WORDS_BELONG_TO_FIELD)?.to_string();
    let turn = answered.fields.text(asked::THE_STEPS_WORDS_TURN_FIELD).map(str::to_string);
    let marked = answered.fields.text(asked::THE_STEPS_WORDS_WERE_NOT_KEPT_FIELD) == Some(asked::THE_WORDS_WERE_NOT_KEPT);
    let more_follow = answered.fields.text(asked::MORE_OF_THE_STEPS_WORDS_FIELD) == Some(asked::THEY_FOLLOW);
    let ordinary = answered.fields.text(asked::THE_STEPS_ORDINARY_WORDS_FIELD);
    let complaints = answered.fields.text(asked::THE_STEPS_COMPLAINT_WORDS_FIELD);
    // **A program older than the pieces** says the mark in the words' place.
    if marked && turn.is_none() {
        return Some(TheStepsWords {
            step,
            the_rest_was_not_kept: true,
            ..TheStepsWords::default()
        });
    }
    if ordinary.is_none() && complaints.is_none() && !marked {
        return None;
    }
    Some(TheStepsWords {
        step,
        printed: Some((
            ordinary.unwrap_or_default().to_string(),
            complaints.unwrap_or_default().to_string(),
        )),
        turn,
        more_follow,
        the_rest_was_not_kept: marked,
    })
}

/// **`Server Setup` Y7a item 2 on the desk, from 2026-10-25: what of the step
/// in hand's words this side already holds, and the keeping of each new piece
/// once, in order, as the readings bring them** — the eighteenth notes, piece
/// 2c.
///
/// **Every reading names what is held** — the turn the machine gave and a
/// length on each stream ([`Self::the_reading_names`]) — and the machine
/// answers with what follows. **Each piece is handed to the keeping the
/// moment its reading lands**, while the step runs as much as once it has
/// ended, so the window and *Server output* take a worker step's words as it
/// prints them. **A reading under another turn** is another commanding of the
/// step, and what was held of the last is let go first.
///
/// **A machine program older than the pieces** carries no turn and the
/// finished step's words whole; those are kept once, at the reading that
/// shows the step ended, exactly as before
/// ([`a_worker_steps_words_are_kept_once`]).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TheStepsWordsHeldHere {
    turn: Option<String>,
    ordinary: usize,
    complaints: usize,
    marked: bool,
    kept_whole: bool,
}

impl TheStepsWordsHeldHere {
    /// **The reading's request, naming what this side holds** — nothing
    /// before the machine has given a turn, so the first reading is answered
    /// from the start.
    pub fn the_reading_names(&self, fields: Fields) -> Fields {
        match &self.turn {
            Some(turn) => fields
                .and(asked::THE_STEPS_WORDS_TURN_FIELD, turn.clone())
                .and(asked::THE_ORDINARY_WORDS_HELD_FIELD, self.ordinary.to_string())
                .and(asked::THE_COMPLAINT_WORDS_HELD_FIELD, self.complaints.to_string()),
            None => fields,
        }
    }

    /// **A reading's words for the step this side is waiting on, kept** —
    /// each new piece handed to `words_go` once, the ordinary stream's before
    /// the complaints', and the mark once. **True where anything new came**,
    /// which is what `Server Setup` Y7d's cadence reads.
    pub fn a_reading_is_kept(
        &mut self,
        words_go: Option<&TheWordsGoHere>,
        step: Step,
        commanded: &str,
        state: &TheRunsState,
    ) -> bool {
        let Some(words) = state.the_steps_words.as_ref().filter(|words| words.step == commanded) else {
            return false;
        };
        let Some(turn) = &words.turn else {
            let was = self.kept_whole;
            a_worker_steps_words_are_kept_once(words_go, step, commanded, state, &mut self.kept_whole);
            return !was
                && self.kept_whole
                && words
                    .printed
                    .as_ref()
                    .is_none_or(|(ordinary, complaints)| !ordinary.is_empty() || !complaints.is_empty());
        };
        if self.turn.as_deref() != Some(turn.as_str()) {
            *self = TheStepsWordsHeldHere {
                turn: Some(turn.clone()),
                ..TheStepsWordsHeldHere::default()
            };
        }
        let mut something_new = false;
        if let Some((ordinary, complaints)) = &words.printed {
            if !ordinary.is_empty() {
                self.ordinary += ordinary.len();
                something_new = true;
                if let Some(go) = words_go {
                    go(step, &WhatTheServerPrinted::APiece(APiecePrinted::Output(ordinary.as_bytes().to_vec())));
                }
            }
            if !complaints.is_empty() {
                self.complaints += complaints.len();
                something_new = true;
                if let Some(go) = words_go {
                    go(step, &WhatTheServerPrinted::APiece(APiecePrinted::Complaint(complaints.as_bytes().to_vec())));
                }
            }
        }
        if words.the_rest_was_not_kept && !self.marked {
            self.marked = true;
            something_new = true;
            if let Some(go) = words_go {
                go(step, &WhatTheServerPrinted::SomeOfItCouldNotBeBroughtBack);
            }
        }
        something_new
    }

    /// **Whether the reading said more of this step's words follow** — which
    /// a step that has ended reads again for at once, so its last piece is
    /// kept before anything further is issued (`Server Setup` Y7a item 5).
    pub fn more_follow(&self, commanded: &str, state: &TheRunsState) -> bool {
        state.the_steps_words.as_ref().is_some_and(|words| {
            words.step == commanded && words.more_follow && words.turn.is_some() && words.turn == self.turn
        })
    }
}

/// **How many readings, at most, a step that has ended takes at once for the
/// rest of its words** before this side goes on. Each carries up to the
/// largest frame, so sixty-four bring back sixty-four megabytes — four times
/// what the machine holds of one step at most — and a machine that kept
/// saying more follows past that could not hold this side forever.
pub const THE_MOST_READINGS_FOR_THE_REST_OF_A_STEPS_WORDS: u32 = 64;

/// **`Server Setup` Y7a item 2 on the desk: a worker step's words, handed to the
/// keeping once, at the first reading that shows the step ended, under the
/// step this side commanded.** The reading carries them from the step's report
/// until this side commands anything further, and this is called on every
/// reading of the wait — so `kept` is what makes it once.
///
/// **Only the step this side is waiting on**: words naming another step are
/// not this wait's. **A reading with the step still in flight carries none**,
/// since the machine holds them only from the report; it is checked anyway. The
/// ordinary stream goes first and the complaints after it, which keeps each
/// stream in the order printed — the two are kept apart, so their order to each
/// other is nothing anybody reads. Added 2026-10-24.
pub fn a_worker_steps_words_are_kept_once(
    words_go: Option<&TheWordsGoHere>,
    step: Step,
    commanded: &str,
    state: &TheRunsState,
    kept: &mut bool,
) {
    if *kept || state.the_step_in_flight.as_deref() == Some(commanded) {
        return;
    }
    let Some(words) = state.the_steps_words.as_ref().filter(|words| words.step == commanded) else {
        return;
    };
    *kept = true;
    let Some(go) = words_go else {
        return;
    };
    match &words.printed {
        None => go(step, &WhatTheServerPrinted::SomeOfItCouldNotBeBroughtBack),
        Some((ordinary, complaints)) => {
            if !ordinary.is_empty() {
                go(step, &WhatTheServerPrinted::APiece(APiecePrinted::Output(ordinary.as_bytes().to_vec())));
            }
            if !complaints.is_empty() {
                go(step, &WhatTheServerPrinted::APiece(APiecePrinted::Complaint(complaints.as_bytes().to_vec())));
            }
        }
    }
}

/// **`Server Setup` Y7b's and `Server Setup` Y7c's count**: the bytes so far, and the whole
/// they are counted against where it is known. **Smoozy's own figure, never
/// the server's output.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheStepsCount {
    pub so_far: u64,
    pub whole: Option<u64>,
}

/// **Which act a count is of** — the download (`Server Setup` Y7b) or the
/// unpack (`Server Setup` Y7c), read off the step commanded and nothing else.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheCountedAct {
    TheDownload,
    TheUnpack,
}

impl TheCountedAct {
    /// The act a commanded worker step counts, where it counts one.
    pub fn of(commanded: &str) -> Option<TheCountedAct> {
        match commanded {
            asked::THE_FETCH_STEP => Some(TheCountedAct::TheDownload),
            asked::THE_UNPACK_STEP => Some(TheCountedAct::TheUnpack),
            _ => None,
        }
    }
}

/// **How far the download or the unpack has got, as the run was last read** —
/// what the Set up card's step says beside its name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HowFarTheStepHasGot {
    pub act: TheCountedAct,
    pub count: TheStepsCount,
}

/// **Where how far a step has got goes, each time the run is read while it
/// runs** — and nothing once the step is over
/// ([`AsksForThePassword::where_the_figure_goes`]).
pub type TheFigureGoesHere = std::sync::Arc<dyn Fn(Option<HowFarTheStepHasGot>) + Send + Sync>;

// ════════════════════════════════════════════════════════════════════════════
// The whole order
// ════════════════════════════════════════════════════════════════════════════

/// **`Server Setup` R2's order, performed on a machine never prepared — and
/// `Server Setup` R1's run alone on a machine this copy already set up.**
///
/// **Nothing is taken out of order and nothing is skipped on a guess.** A
/// refusal at any step is answered in that step's own words and everything
/// after it is not attempted.
///
/// **Which of the two is the live connection's reading, taken before the press
/// and handed in** — `State, identity and recovery` Y9, the same test
/// `Servers` Q1 step 7 runs on. *Present and equal, confirmed* is this copy's
/// own installation, and `Server Setup` R2 places only on a machine never
/// prepared — so there the machine-changing prefix below is not taken at all
/// and the run is opened on the package area the row records. **Every other
/// reading, and no reading, runs R2 whole**, exactly as before 2026-09-28:
/// absent and different are machines to place on, and *the supervisor did not
/// answer* may never stand in for *present and equal* — which is
/// `State, identity and recovery` Y9's own rule, and `Server Setup` F1's
/// ground, which this act does not widen. **A confirmed machine whose row
/// records no package area also runs R2 whole**, rather than guessing where the
/// run's program is. *(Until 2026-09-28 every machine got R2 whole: the ninth
/// pass on Hyperstack pressed *Set up* on a set-up machine with a job rendering
/// and got a hand-over, a release, a placement under a new installation
/// identity, the supervisor restarted under the render and the event cursor
/// cleared — step 10c's ninth-pass fix piece 3.)*
#[allow(clippy::too_many_arguments)]
pub async fn a_machine_is_set_up<C: AHeldConnection, P: PlacesTheVerifiedRelease<C>>(
    sessions: &ThisServersSessions<C>,
    taking: &mut dyn TakesTheOpeningReadings,
    row: &mut dyn KeepsTheServersRow,
    placement: P,
    releases: &mut dyn ReadsTheCurrentRelease,
    asked_for: &WhatWasAskedFor,
    elevation: HowElevationIsObtained<'_>,
    telegram: &mut dyn ComposesTheTelegramMaterial,
    asker: &mut dyn AsksForThePassword,
    stop: &dyn AsksToStop,
    the_period: Option<TheRestartMark>,
    the_live_connection_read: Option<smoozy_state::identities::InstallationCheck>,
) -> WhereTheSetUpGot {
    // ── 1. `Server Setup` M1 and `Server Setup` M1a, and nothing else if
    //       either refuses.
    let said = match the_opening_readings(taking).await {
        Ok(said) => said,
        Err(stopped) => return stopped,
    };

    // ── 1a. `Server Setup` M3: elevation is probed before it is offered.
    //        Added 2026-09-21, after the first pass on a real rental found it
    //        never taken — every run went out owing a password on a login that
    //        needed none.
    // **`Server Setup` Y7a item 1: the run's words begin with the readings
    // that compose it**, so where they go is asked here, once, and the probe
    // below and `Server Setup` R1 item 1's readings are read as printed.
    // Added 2026-10-24.
    let words = asker.where_the_words_go();
    let elevation = the_elevation_this_login_has_telling(sessions, elevation, words.as_ref()).await;

    // ── 1b. **A machine this copy already set up: `Server Setup` R1's run
    //        alone.** Everything above changes nothing on the machine, and
    //        everything from here to step 6 is R2's placement on a machine
    //        never prepared. `Server Setup` Y6b: *"On a machine prepared
    //        earlier, the placement replaces nothing."* So there is no tuple,
    //        no hand-over, no release and no placement; no new installation
    //        identity, which `State, identity and recovery` Y8 issues at the
    //        hand-over; no cursor cleared, the cursor still pointing into this
    //        installation's journal (`State, identity and recovery` E2); and
    //        no `Servers` Q19a commit. **A package older than the current
    //        release is not this act's to replace** — that is the automatic
    //        repair's and `Server Setup` F1's, and neither is widened here.
    //        Step 10c's ninth-pass fix piece 3.
    if let Some(package_area) =
        the_package_area_of_a_set_up_machine(row, the_live_connection_read)
    {
        return the_ready_run_is_opened_and_driven(
            sessions,
            &said,
            &package_area,
            asked_for,
            elevation,
            telegram,
            asker,
            stop,
            the_period,
            words,
        )
        .await;
    }

    // ── 2. `Server protocol and transfers` B8's detected rental tuple.
    let tuple = match the_rentals_tuple(sessions).await {
        TheRentalsTuple::ItSaid(tuple) => tuple,
        _ => return WhereTheSetUpGot::TheRentalWouldNotNameItself,
    };

    // ── 3. `Server Setup` R6's set, `Job files and outputs` 1.1's choice, and
    //       `Server Setup` R7's hand-over with `Server Setup` R7b's recording.
    let locations = the_locations_this_login_can_offer(sessions).await;
    let layout = match the_layout_this_machine_gets(&locations) {
        Ok(layout) => layout,
        Err(stopped) => return stopped,
    };
    let installation = match the_installation_a_setup_left_unconfirmed(row) {
        Some(it) => it,
        None => match Identity::issue(IdentityKind::Installation) {
            Ok(it) => it,
            Err(_) => {
                return WhereTheSetUpGot::TheRootCouldNotBeRecorded {
                    said: "Could not generate an installation ID."
                        .to_string(),
                }
            }
        },
    };
    if let Err(stopped) = the_hand_over(sessions, row, &layout, &installation, elevation).await {
        return stopped;
    }

    // ── 4. `Server protocol and transfers` B8d's own-release address, then
    //       `Server protocol and transfers` B9 and
    //       `Server protocol and transfers` B10's whole admission.
    let (verified, artifact) = match releases.the_current_release(&tuple) {
        Ok(got) => got,
        Err(said) => return WhereTheSetUpGot::TheReleaseWasNotAdmitted { said },
    };

    // ── 4a. **`Desktop app and delivery` V14: nothing this desktop sends places
    //        a release below the machine's own**, and `Server Setup` F1 step 2
    //        names this route among the three. The machine is read by
    //        `Server protocol and transfers` B4's inspection at the package area
    //        the placement would use, and its active release sequence is
    //        compared with this build's own — the verified manifest's, which in
    //        a copy given a desk program is the desk program's nought (W0-2, in
    //        `Server protocol and transfers` B10a). **Too new is refused
    //        before anything is staged**, in the words `Design/Servers` gives
    //        the too-new line. Added 2026-09-28, step 10c's transfer wave piece
    //        8.
    let package_area = layout.area(Area::Package);
    let on_the_machine = what_the_package_area_holds(sessions, &package_area).await;
    if let Some(refused) = what_v14_refuses(&on_the_machine, &verified) {
        return refused;
    }
    // ── 4b. **`Rented-server runtime` G21's reading, and the real one.** A
    //        program already answering at this package area is asked for its
    //        queue (`Queue & History` A1), and the jobs it holds are what the
    //        commit is held on; a package area with no active package has no
    //        program to have accepted anything. *(Until 2026-09-28 an empty
    //        list was handed in whatever the machine held.)*
    let accepted_work = if on_the_machine.its_release_sequence.is_some() {
        what_the_program_there_has_accepted(sessions, &package_area).await
    } else {
        Vec::new()
    };
    let what_g21_read = accepted_work.clone();

    // ── 5. The placement, through the bootstrap-and-repair request.
    //
    // **A deferred phase found on the machine is sent again under its recorded
    // attempt, and never under a fresh one** — `Server protocol and transfers`
    // B6 (*"it does not license a second attempt under a fresh identity"*) and
    // `Rented-server runtime` G17's *deferred* row, the way `Desktop app and
    // delivery` V12 sends an update's. **Written for a machine never prepared,
    // where no phase record stands; it now also meets a machine whose last
    // setup was deferred.** Until step 10c's thirteenth-pass fix piece 2 every
    // press issued a fresh attempt, and `Rented-server runtime` G18 answers a
    // fresh attempt that meets
    // *deferred* with that deferral and acts on nothing — so once one press
    // was deferred, no later press could set the machine up, whatever then
    // stood on it (the thirteenth pass on Hyperstack, 04:09 and 04:53).
    let attempt = match on_the_machine
        .the_phase_record
        .as_ref()
        .filter(|record| record.phase == smoozy_runtime::package::phase::Phase::Deferred)
    {
        Some(deferred) => deferred.attempt.clone(),
        None => match Identity::issue(IdentityKind::Operation) {
            Ok(attempt) => attempt,
            Err(_) => {
                return WhereTheSetUpGot::ThePlacementCouldNotBeCarriedOut {
                    said: "Could not generate an install attempt ID.".to_string(),
                }
            }
        },
    };
    let Ok(exchange) = Identity::issue(IdentityKind::Operation) else {
        return WhereTheSetUpGot::ThePlacementCouldNotBeCarriedOut {
            said: "Could not generate a request ID.".to_string(),
        };
    };
    // **Where the artifact is staged is not composed here.** `Server protocol
    // and transfers` S4 makes that path the placement's own, from the attempt's
    // identity. **It was composed here until step 10c piece 12**, as the fixed
    // entry-point name's staging name — which is the pointer's own staging
    // name, so the commit deleted the program it was committing.
    let reached = the_placement_is_performed(
        sessions,
        placement,
        WhatThePlacementIsGiven {
            package_area: package_area.clone(),
            attempt,
            owner: the_owner(&installation),
            installation: installation.clone(),
            verified,
            artifact,
            kind: the_runtimes_kind(said.kind),
            the_health_exchanges_identity: exchange,
            accepted_work,
            previous: None,
            // **`Rented-server runtime` K11 and K19**, on the one kind whose
            // registration is elevated, for the elevation M3's probe found.
            the_root_step_on_a_container: (the_runtimes_kind(said.kind)
                == smoozy_runtime::machine_kind::MachineKind::Container)
                .then(|| elevated::the_root_step_on_a_container(elevation)),
        },
    )
    .await;
    let reached = match reached {
        Ok(outcome) => outcome.reached,
        Err(said) => return WhereTheSetUpGot::ThePlacementCouldNotBeCarriedOut { said },
    };
    if reached != WhatThePlacementReached::Verified {
        return what_a_placement_not_verified_came_to(reached, &what_g21_read);
    }

    // **The installation is this machine's now, and the row's event cursor goes
    // with the journal it pointed into** — step 10c piece 7's owed item, and
    // `State, identity and recovery` §4 E2's own rule.
    if let Err(said) = row.record_the_installation_and_clear_the_cursor(&installation) {
        return WhereTheSetUpGot::TheRootCouldNotBeRecorded { said };
    }
    // **`Servers` Q19a: the commit is where step 5 is taken again**, on a
    // period whose live connection read the absent branch — so the first
    // connection a user ever makes reads the machine's work without a
    // reconnect. Step 10c's eighth-pass fix piece 3.
    row.the_placement_committed();

    // ── 6. `Server Setup` R1's run, driven one step at a time.
    the_ready_run_is_opened_and_driven(
        sessions,
        &said,
        &package_area,
        asked_for,
        elevation,
        telegram,
        asker,
        stop,
        the_period,
        words,
    )
    .await
}

/// **V14 at `Server Setup` R2's placement: a machine whose active release is
/// above the one this placement would place is refused**, and nothing where it
/// is not — the same release, an older one, or no active package at all.
///
/// **In a copy given a desk program the release placed is the desk program's
/// nought** (W0-2, in `Server protocol and transfers` B10a), so a machine the
/// shortcut placed is *the same* and is set up again as before, and a machine
/// any published release placed is refused rather than replaced by a program
/// with no release.
pub fn what_v14_refuses(
    on_the_machine: &crate::servers::installation::WhatThePackageAreaHolds,
    placing: &AVerifiedRelease,
) -> Option<WhereTheSetUpGot> {
    (the_direction(on_the_machine.its_release_sequence, placing.manifest.release_sequence)
        == Some(TheDirection::TheMachineIsNewer))
    .then(|| WhereTheSetUpGot::ThePlacementCouldNotBeCarriedOut {
        said: THE_MACHINE_IS_NEWER.to_string(),
    })
}

/// **The package area of a machine this copy already set up, where the live
/// connection read it so** — and nothing on every other reading. Step 10c's
/// ninth-pass fix piece 3.
///
/// **Both halves are needed.** `State, identity and recovery` Y9's *present and
/// equal, confirmed* is the one reading under which the machine is this copy's
/// installation; and the run needs the package area the row records —
/// `Job files and outputs` L1 — which is also what the Blender install's own
/// act reads a placed program by, the row's confirmed installation beside it.
/// **A row with either missing answers nothing**, and the order runs whole.
fn the_package_area_of_a_set_up_machine(
    row: &mut dyn KeepsTheServersRow,
    the_live_connection_read: Option<smoozy_state::identities::InstallationCheck>,
) -> Option<String> {
    if the_live_connection_read
        != Some(smoozy_state::identities::InstallationCheck::PresentAndEqualConfirmed)
    {
        return None;
    }
    let row = row.the_row()?;
    row.last_confirmed_installation_identity.as_ref()?;
    the_package_area_of(&row)
}

/// **`Server Setup` R1's run, opened on a package area and driven one step at a
/// time** — the ready action's own run, whether R2's placement came before it
/// or the machine was set up already.
#[allow(clippy::too_many_arguments)]
async fn the_ready_run_is_opened_and_driven<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    said: &WhatTheOpeningReadingsSaid,
    package_area: &str,
    asked_for: &WhatWasAskedFor,
    elevation: HowElevationIsObtained<'_>,
    telegram: &mut dyn ComposesTheTelegramMaterial,
    asker: &mut dyn AsksForThePassword,
    stop: &dyn AsksToStop,
    the_period: Option<TheRestartMark>,
    words: Option<TheWordsGoHere>,
) -> WhereTheSetUpGot {
    let needs = what_this_machine_needs(sessions, said, asked_for, false, words.as_ref()).await;
    let steps = ready_action::compose_the_run(&needs);
    let kind = the_kind_of_setup(asked_for);
    let mut run = match TheRunBeingDriven::opened(
        sessions,
        package_area,
        kind,
        steps,
        // **`Server Setup` R1 item 1's readings, carried whole into the run**,
        // so every judgement the steps want is made from the one set the
        // composition was made from.
        needs,
        the_period,
        asker.the_runs_this_copy_drove_here(),
        stop,
        asker,
    )
    .await
    {
        Ok(run) => run,
        Err(ending) => {
            return WhereTheSetUpGot::TheRunEnded {
                at: Boundary::Started,
                ending,
            }
        }
    };
    run.the_words_go = words;
    run.driven(elevation, asked_for, telegram, asker, stop).await
}

/// **`Server Setup` W3's first row: whether the ready action, composed now,
/// carries driver work or a restart.** Added 2026-09-27, the eighth-pass fix
/// for W3's warning.
///
/// **The warning is a property of the step and the table is closed**, so this
/// reads the machine exactly as the ready action's own order would — `Server
/// Setup` M1's and `Server Setup` M1a's opening readings, then `Server Setup`
/// R1 item 1's —
/// and asks `Server Setup` R1 item 5's own answer to what the driver step
/// does, rather than judging anything itself. **A driver step that is composed
/// and does nothing** — the check answered, *"nothing is installed and nothing
/// is touched"* — **carries neither**, and its restart runs only after work
/// the step did. **It changes nothing on the machine**: every reading here is
/// one the order takes before its first machine-changing step. **Where the
/// opening readings refuse, the order would refuse at the same place and touch
/// nothing**, so there is nothing to warn about.
pub async fn the_ready_action_carries_a_restart<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    taking: &mut dyn TakesTheOpeningReadings,
) -> bool {
    let Ok(said) = the_opening_readings(taking).await else {
        return false;
    };
    let asked_for = WhatWasAskedFor {
        install_this_blender: None,
        telegram_is_on: false,
        ..WhatWasAskedFor::default()
    };
    let needs = what_this_machine_needs(sessions, &said, &asked_for, false, None).await;
    matches!(
        ready_action::what_the_driver_step_does(&needs),
        ready_action::WhatTheDriverStepDoes::TheRestartBeforeAnyInstall
            | ready_action::WhatTheDriverStepDoes::TheDistributionsOwnDetectionRoute
    )
}

/// **`Server Setup` M3: elevation is probed before it is offered, with the
/// elevating command's non-interactive form of a command that does nothing.**
/// Added 2026-09-21.
///
/// **The probe is `Server Setup`'s own** — `kind::THE_NON_INTERACTIVE_PROBE`,
/// `sudo -n true`, with M3's publication and its dated observation written at
/// that constant, and `readings::run_the_probe` reading it. **This file composes
/// no command and reads no words**: the exit is the whole of the reading.
///
/// - **It is taken only where a password is owed and none is held.** A held
///   password already answers the question, and a caller that knows elevation
///   is passwordless has nothing to learn.
/// - **A probe that succeeds is `Server Setup` M3's passwordless elevation**, so
///   every elevated step goes out in the non-interactive form and nobody is
///   asked for a password that does not exist — which is what the first pass on
///   a real rental found every run doing, on a login where `sudo -n true` ended
///   at 0.
/// - **A probe that fails, or that does not reach the machine, changes
///   nothing**: a password is owed, and `Server Setup` E2's offer is made at the
///   moment a step needs one and never before. **That is the safe direction**:
///   M3's own note says an over-optimistic probe costs a refusal, and a
///   pessimistic one costs an offer the user can answer.
/// - **It is taken after `Server Setup` M1 and `Server Setup` M1a**, which `Server Setup` R2
///   puts first with nothing else beside them, and it changes nothing on the
///   machine.
pub async fn the_elevation_this_login_has<'a, C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    given: HowElevationIsObtained<'a>,
) -> HowElevationIsObtained<'a> {
    the_elevation_this_login_has_telling(sessions, given, None).await
}

/// **[`the_elevation_this_login_has`], with `Server Setup` M3's probe read as
/// printed** — its two streams handed to `words` under the run's first step,
/// where a screen keeps the run's words (`Server Setup` Y7a item 1). Added
/// 2026-10-24.
pub async fn the_elevation_this_login_has_telling<'a, C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    given: HowElevationIsObtained<'a>,
    words: Option<&TheWordsGoHere>,
) -> HowElevationIsObtained<'a> {
    if given != HowElevationIsObtained::APasswordIsOwedAndNoneIsHeld {
        return given;
    }
    let mut channel = the_readings_channel(sessions, words);
    match readings::run_the_probe(&mut channel).await {
        smoozy_server_setup::kind::Probe::Succeeded => HowElevationIsObtained::Passwordless,
        smoozy_server_setup::kind::Probe::Failed => given,
    }
}

/// **The placement, over the connection the point lends for its whole length.**
///
/// **`Server protocol and transfers` T5 is kept by the lend rather than
/// worked around**: the transaction holds a file-transfer session and runs
/// commands, so it takes the connection whole and gives it back, and no other
/// session is lent while it stands.
async fn the_placement_is_performed<C: AHeldConnection, P: PlacesTheVerifiedRelease<C>>(
    sessions: &ThisServersSessions<C>,
    mut placement: P,
    given: WhatThePlacementIsGiven,
) -> Result<ThePlacementsOutcome, String> {
    let lent = sessions
        .the_whole_connection(move |connection| {
            Box::pin(async move { placement.place(connection, given.as_the_walk_takes_it()).await })
        })
        .await;
    match lent {
        Ok(Ok(outcome)) => Ok(outcome),
        Ok(Err(why)) => Err(the_words_of(&why)),
        Err(_) => Err("No connection to the server.".to_string()),
    }
}

/// **Why the surface would not carry the placement, in the machine's own
/// words where it printed any.** Added 2026-09-21: until then this was a
/// debugging rendering, and the first pass on a real rental put
/// `TheMachineWouldNot("…")` on the screen with its escapes written out.
pub(crate) fn the_words_of(why: &WhyThePlacementCouldNotBeCarriedOut) -> String {
    match why {
        WhyThePlacementCouldNotBeCarriedOut::TheMachineWouldNot(said) => said.clone(),
        WhyThePlacementCouldNotBeCarriedOut::TheSubsystem(failure) => {
            readings::WhatTheMachineSaid::from(failure.clone())
                .in_words()
                .to_string()
        }
        WhyThePlacementCouldNotBeCarriedOut::ThatValueDoesNotGoIntoARegistration(said) => {
            said.clone()
        }
    }
}

/// **`Rented-server runtime` G21's reading of what the program at this package
/// area has accepted** — `Queue & History` A1's queue reading, asked of it, and
/// read into that clause's four standings by
/// [`crate::servers::the_machines_work::what_g21_reads`]. Added 2026-09-28,
/// step 10c's transfer wave piece 8.
///
/// **Where no reading comes back, nothing is handed in, exactly as before.**
/// A program that does not answer cannot have its work read live, and the
/// reading `Server protocol and transfers` P3b names for that case — the
/// records, through the surface — rests on `State, identity and recovery` §1.3
/// rule 7's one shape, which the machine's half of this wave writes (the
/// transfer wave's piece 10) and which is not read here. **Holding the commit
/// on no reading was weighed and not taken**: `Server Setup` F1's re-run exists
/// to repair a machine whose supervisor is dead, and a dead supervisor gives
/// no reading, so every such repair would end *deferred* for good.
///
/// **On `Server Setup` R2's placement the program found here is not this
/// copy's installation**, so no job it holds is this copy's result to collect:
/// an ended job counts as nothing, and waiting, held and rendering ones count
/// as they always do.
pub async fn what_the_program_there_has_accepted<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    package_area: &str,
) -> Vec<(String, smoozy_runtime::package::retry::AcceptedWorkStanding)> {
    use crate::servers::the_machines_work::{
        the_queue_reading_in, the_queue_reading_request, what_g21_reads,
    };
    let Some(request) = the_queue_reading_request() else {
        return Vec::new();
    };
    match the_queue_reading_in(sessions.ask_the_supervisor(package_area, &request).await) {
        Ok(reading) => what_g21_reads(&reading, &|_| None),
        Err(_) => Vec::new(),
    }
}

/// **What a placement that did not verify comes to, with what `Rented-server
/// runtime` G21 read beside it.** A deferral over any held job is said as the held jobs of an earlier
/// setup, counted, because those block for good (`Rented-server runtime` G23)
/// and nothing this copy holds can answer them (`State, identity and
/// recovery` Y9); every other phase is said as it is. **Only held jobs are
/// counted**: waiting and rendering work ends on its own, and the next press
/// then goes ahead under the deferral's own attempt. Added 2026-10-01, step
/// 10c's thirteenth-pass fix piece 2.
pub fn what_a_placement_not_verified_came_to(
    reached: WhatThePlacementReached,
    what_g21_read: &[(String, smoozy_runtime::package::retry::AcceptedWorkStanding)],
) -> WhereTheSetUpGot {
    use smoozy_runtime::package::retry::AcceptedWorkStanding;
    let held = what_g21_read
        .iter()
        .filter(|(_, standing)| *standing == AcceptedWorkStanding::Held)
        .count();
    match reached {
        WhatThePlacementReached::Deferred { because } if held > 0 => {
            WhereTheSetUpGot::JobsOfAnEarlierSetupAreHeld {
                how_many: held,
                because,
            }
        }
        reached => WhereTheSetUpGot::ThePlacement { reached },
    }
}

/// **`Desktop app and delivery` V11's update, sent** — the bootstrap-and-repair
/// request for this build's own release, over an older program, under the same
/// claim (`Rented-server runtime` G1b), commit, health exchange and rollback
/// (`Rented-server runtime` G3 to `Rented-server runtime` G23) as `Server
/// Setup` R2's placement, **under `Rented-server runtime` G2's *automatic
/// repair* route.** Added 2026-09-28, step 10c's transfer wave piece 8.
///
/// **What it was written for, and what it now also meets.** The walk it hands
/// over to was written for a machine never prepared, with no previous version
/// and no work. It now meets a machine running an older program: `Rented-server
/// runtime` G4's previous version is carried forward out of the standing phase
/// record — the active package's descriptor, its retained adapter, where it
/// sits, and the boot registration's content by value — and `Rented-server
/// runtime` G21's reading is the one step 7 took.
///
/// **V12: a deferred phase is re-sent under its recorded identity** — `under`
/// is that identity, and a fresh one is issued only where none is handed in.
/// **Nothing here retries**: whatever the walk reached is the answer.
#[allow(clippy::too_many_arguments)]
pub async fn the_update_is_placed<C: AHeldConnection, P: PlacesTheVerifiedRelease<C>>(
    sessions: &ThisServersSessions<C>,
    placement: P,
    package_area: &str,
    installation: Identity,
    held: &crate::servers::installation::WhatThePackageAreaHolds,
    under: Option<Identity>,
    verified: AVerifiedRelease,
    artifact: Vec<u8>,
    accepted_work: Vec<(String, smoozy_runtime::package::retry::AcceptedWorkStanding)>,
) -> Result<WhatThePlacementReached, String> {
    let Some(standing) = held.the_phase_record.as_ref() else {
        return Err("Could not read the server's record of Smoozy's server software.".to_string());
    };
    let attempt = match under {
        Some(recorded) => recorded,
        None => Identity::issue(IdentityKind::Operation)
            .map_err(|_| "Could not generate an install attempt ID.".to_string())?,
    };
    let exchange = Identity::issue(IdentityKind::Operation)
        .map_err(|_| "Could not generate a request ID.".to_string())?;
    let kind = standing.machine_kind;
    let elevation = the_elevation_this_login_has(
        sessions,
        HowElevationIsObtained::APasswordIsOwedAndNoneIsHeld,
    )
    .await;
    let given = WhatThePlacementIsGiven {
        package_area: package_area.to_string(),
        attempt,
        owner: TransactionOwner {
            route: Route::AutomaticRepair,
            ..the_owner(&installation)
        },
        installation,
        verified,
        artifact,
        kind,
        the_health_exchanges_identity: exchange,
        the_root_step_on_a_container: (kind
            == smoozy_runtime::machine_kind::MachineKind::Container)
            .then(|| elevated::the_root_step_on_a_container(elevation)),
        accepted_work,
        previous: the_previous_version_in(standing, held),
    };
    the_placement_is_performed(sessions, placement, given)
        .await
        .map(|outcome| outcome.reached)
}

/// **`Rented-server runtime` G4's previous version, carried forward out of the
/// standing phase record** — the package the pointer names now, by value.
///
/// **Which of the record's two descriptors is active is read off the release
/// sequence inspection found**, which is `Rented-server runtime` G31a's own
/// reading of the pointer: the requested one where the last attempt landed, the
/// recorded previous one where it rolled back. Nothing where neither is the
/// active one, and the walk then carries nothing forward.
fn the_previous_version_in(
    standing: &smoozy_runtime::package::phase::PhaseRecord,
    held: &crate::servers::installation::WhatThePackageAreaHolds,
) -> Option<smoozy_runtime::package::phase::PreviousVersion> {
    let active = held.its_release_sequence?;
    if standing.requested.verified_release_sequence == active {
        return Some(smoozy_runtime::package::phase::PreviousVersion {
            descriptor: standing.requested.clone(),
            retained_adapter: standing
                .staged_packages_retained_adapters
                .first()
                .cloned()
                .unwrap_or(smoozy_runtime::package::release::RetainedAdapter {
                    declared_interval: (
                        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS.0,
                        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS.0,
                    ),
                    declared_request_set: Vec::new(),
                }),
            location: standing.staging_location.clone(),
            previous_boot_registration_content: held.the_boot_registration.clone().unwrap_or_default(),
        });
    }
    standing
        .previous
        .clone()
        .filter(|previous| previous.descriptor.verified_release_sequence == active)
}

/// **What reads the current release**, as a seam, so the whole order runs at a
/// desk with no line to the internet and with no second trust root anywhere in
/// a shipped build.
pub trait ReadsTheCurrentRelease {
    fn the_current_release(
        &mut self,
        detected: &smoozy_server_protocol::release::manifest::TargetTuple,
    ) -> Result<(AVerifiedRelease, Vec<u8>), String>;
}

// ════════════════════════════════════════════════════════════════════════════
// The small translations
// ════════════════════════════════════════════════════════════════════════════

/// `Server Setup` Y6b's five kinds, as the wire words.
fn the_word_for(kind: SetupKind) -> String {
    match kind {
        SetupKind::TheReadyAction => asked::THE_READY_ACTION,
        SetupKind::ABlenderInstall => asked::A_BLENDER_INSTALL,
        SetupKind::ABlenderRemoval => asked::A_BLENDER_REMOVAL,
        SetupKind::AnAddOnInstall => asked::AN_ADD_ON_INSTALL,
        SetupKind::TheTelegramPlacement => asked::THE_TELEGRAM_PLACEMENT,
    }
    .to_string()
}

/// `Server Setup` Y4's boundary words, as the wire words.
fn the_word_for_boundary(boundary: Boundary) -> &'static str {
    match boundary {
        Boundary::Started => asked::THE_BOUNDARY_STARTED,
        Boundary::Fetched => asked::THE_BOUNDARY_FETCHED,
        Boundary::Tooling => asked::THE_BOUNDARY_TOOLING,
        Boundary::Libraries => asked::THE_BOUNDARY_LIBRARIES,
        Boundary::DriverPlaced => asked::THE_BOUNDARY_DRIVER_PLACED,
        Boundary::RestartIssued => asked::THE_BOUNDARY_RESTART_ISSUED,
        Boundary::CardAnswering => asked::THE_BOUNDARY_CARD_ANSWERING,
        Boundary::Blender => asked::THE_BOUNDARY_BLENDER,
        Boundary::Removed => asked::THE_BOUNDARY_REMOVED,
        Boundary::Telegram => asked::THE_BOUNDARY_TELEGRAM,
        Boundary::Done => asked::THE_BOUNDARY_DONE,
    }
}

/// The boundary word, read back.
fn the_boundary_named(word: &str) -> Option<Boundary> {
    [
        Boundary::Started,
        Boundary::Fetched,
        Boundary::Tooling,
        Boundary::Libraries,
        Boundary::DriverPlaced,
        Boundary::RestartIssued,
        Boundary::CardAnswering,
        Boundary::Blender,
        Boundary::Removed,
        Boundary::Telegram,
        Boundary::Done,
    ]
    .into_iter()
    .find(|boundary| the_word_for_boundary(*boundary) == word)
}

/// **Which boundary the worker's own completion of a commanded step lets the
/// supervisor write**, where it completes a `Server Setup` R1 step outright.
///
/// **It is the machine's own mapping, read back here rather than written a
/// second time** — the words are `smoozy_framing`'s and the mapping is
/// `Server Setup`'s own `acting::worker::WhichStep::the_boundary_it_feeds`.
///
/// **`Server Setup` Y4's *done* row is not a worker's.** That row guarantees
/// the final verification passed **and** that the state was cleared and
/// reconciliation ran, and the clear is `Rented-server runtime` P3's act
/// through the ordinary command path — the desktop's. **So the final
/// verification reaches no boundary here**, and the same holds for every
/// boundary an elevated step feeds: all five are written from this side by
/// [`TheRunBeingDriven::the_boundary_is_written`], which is step 10c's route
/// for `Server Setup` Y7's elevated-step bullet. **A step with no boundary here
/// is waited on by the step under way**, which the machine's own record carries
/// from the moment the step is commanded.
fn the_boundary_a_step_reaches(commanded: &str) -> Option<Boundary> {
    match commanded {
        c if c == asked::THE_FETCH_STEP => Some(Boundary::Fetched),
        c if c == asked::THE_UNPACK_STEP => Some(Boundary::Tooling),
        c if c == asked::THE_LIBRARY_CHAIN_IS_READ_AGAIN_STEP => Some(Boundary::Libraries),
        c if c == asked::THE_BUILD_IS_PUBLISHED_STEP => Some(Boundary::Blender),
        c if c == asked::THE_BUILD_IS_REMOVED_STEP => Some(Boundary::Removed),
        _ => None,
    }
}

/// **The elevated command one step of `Server Setup` R1's order runs**, where
/// it has one, and it is `Server Setup`'s own constant rather than a string
/// composed here.
///
/// **`Server Setup` R1 item 5's driver install is per family**
/// (`Server Setup` M1b): Ubuntu's own documented detection route, and Debian's
/// own equivalent. **Naming a flavour is forbidden** and neither constant names
/// one.
fn the_elevated_step_for(step: Step, family: DistributionFamily) -> Option<AnElevatedStep> {
    match step {
        Step::Tooling => Some(elevated::THE_TOOLING_INSTALL),
        // **The libraries install is selected rather than looked up by step**,
        // because `Server Setup` E7 puts the choice on the machine's own `ldd`
        // list: [`TheRunBeingDriven::the_libraries_install`] reads that list off
        // the run's state and hands the chosen constant to the same act every
        // other elevated step goes through.
        Step::Libraries => None,
        Step::Driver => Some(match family {
            DistributionFamily::Ubuntu => elevated::THE_UBUNTU_DRIVER_INSTALL,
            DistributionFamily::Debian => elevated::THE_DEBIAN_DRIVER_INSTALL,
        }),
        Step::Restart => Some(elevated::THE_RESTART),
        _ => None,
    }
}

/// **The installation identity this server row's own last hand-over was
/// recorded under, where no placement ever confirmed it** — so that a setup run
/// again presents the identity its own earlier placement claimed the machine
/// with.
///
/// **Why.** `Rented-server runtime` G1b permits one take-over, and only one:
/// *"a claim naming the claimant's own installation identity"*, because
/// `Desktop app and delivery` P2 admits one running instance, so *"a claim
/// written by this installation cannot also be held by a live process of it"*;
/// and §16 item 23 names *"the same installation coming back"* as the ordinary
/// case that holds. **But every setup issued a fresh identity**
/// (`State, identity and recovery` Y8's *issued at hand-over*), so a placement
/// cut off part-way left a claim no later setup from this same computer could
/// ever match — the Sesterce pass's machine, locked through a restart of
/// Smoozy. Added 2026-09-21, step 10c; the reading is recorded in this module's
/// decisions record.
///
/// **Only an identity no placement confirmed is presented again.** Where the
/// row's last confirmed installation is the one the root was recorded under,
/// the machine carries it, and a new setup issues a new one, as
/// `State, identity and recovery` Y8 says. **One setup runs per server at a
/// time in this process** (`Server Setup` Y9's
/// `a_run_begins`), so the claim this can take back is never a live one.
pub fn the_installation_a_setup_left_unconfirmed(
    row: &mut dyn KeepsTheServersRow,
) -> Option<Identity> {
    let row = row.the_row()?;
    let root = row.recorded_machine_root?;
    if row.last_confirmed_installation_identity.as_deref() == Some(root.read_under.as_str()) {
        return None;
    }
    Identity::read_back(IdentityKind::Installation, &root.read_under)
}

/// **`Rented-server runtime` G2's transaction owner, written only while the
/// claim is held.**
///
/// **The route is `SetupPlacement` and never another**: this is `Server Setup`
/// R2's own placement, not automatic repair and not `Server Setup` F1's
/// user-facing fallback.
fn the_owner(installation: &Identity) -> TransactionOwner {
    TransactionOwner {
        invocation: installation.clone(),
        route: Route::SetupPlacement,
        // **The pair, never the identifier alone** — `Rented-server runtime`
        // X1's reason applied to the owner. **This process has no kernel-set
        // start time it can read on every system Smoozy ships to**, so the
        // second half is nought here: what `Rented-server runtime` G1b's
        // take-over rule actually reads
        // is the claiming installation identity, and that is exact.
        process: (std::process::id(), 0),
        installation: installation.clone(),
    }
}

/// `Rented-server runtime` K2's kind, as that crate names it.
fn the_runtimes_kind(kind: MachineKind) -> smoozy_runtime::machine_kind::MachineKind {
    match kind {
        MachineKind::AContainer => smoozy_runtime::machine_kind::MachineKind::Container,
        _ => smoozy_runtime::machine_kind::MachineKind::Systemd,
    }
}

/// **`Server Setup` Y6b's kind, and this driver opens one of the five.**
///
/// **The ready action, always.** A Blender install, a Blender removal, an
/// add-on install and the Telegram placement are runs of their own kinds and
/// their own entry points; what this file drives is `Server Setup` R1's
/// machine-preparation run, which takes a Blender with it where the user asked
/// for one rather than becoming a different kind.
///
/// **A recovery run is not a sixth kind and none is written.**
/// `Server Setup` S4 item 2's take-over gives the recovery the taken-over
/// run's kind, and `Server Setup` Y6b's per-kind retention rests on that: a
/// recovery with a kind of its own would keep a record of its own, so a
/// recovery of a ready run would stop replacing the ready run's record and the
/// card would read *never set up* on a machine that had just been recovered.
/// **A recovery carries the run it takes over and no kind**
/// (`smoozy_framing::requests::THE_RUN_TAKEN_OVER_FIELD`), which is what the
/// machine's own dispatch refuses a kind beside.
fn the_kind_of_setup(_asked_for: &WhatWasAskedFor) -> SetupKind {
    SetupKind::TheReadyAction
}

// ════════════════════════════════════════════════════════════════════════════
// Telegram's tellings — `Server Setup` R1 item 8, `Server Setup` G2's
// dedicated entry point, and `Telegram` L2's delivery on a connection
// ════════════════════════════════════════════════════════════════════════════

/// **`Security and licence` §10's own requirement, in one place: every file
/// of the material lands under the package area it is placed on.** Used by
/// `Server Setup` R1 item 8's step and by `Telegram` L2's delivery alike.
fn the_material_lands_in_the_package_area(material: &TheMaterial, package_area: &str) -> bool {
    material
        .the_files()
        .iter()
        .all(|(path, _)| path.starts_with(&format!("{package_area}/")))
}

/// What is said where the material would not have landed in the package area.
const THE_FILES_WOULD_NOT_HAVE_LANDED: &str =
    "Telegram's files are outside this server's Smoozy folder. None was placed.";

/// **How a run ends where the role's desired state was not written** —
/// `switching_it_on` for a first *enabled*, otherwise the pair restarting a
/// worker that was already running (`Telegram` L4).
fn the_ending_where_the_state_was_not_written(
    step: Step,
    why: WhyTheRolesStateWasNotWritten,
    switching_it_on: bool,
) -> TheRunsEnding {
    match why {
        WhyTheRolesStateWasNotWritten::NoRandomness => TheRunsEnding::AStepFailed {
            step,
            said: "Could not generate a command ID.".to_string(),
        },
        WhyTheRolesStateWasNotWritten::TheSupervisorDidNotAnswer => {
            TheRunsEnding::TheServerStoppedAnswering {
                step,
                said: if switching_it_on {
                    "Smoozy's service on the server did not respond. Telegram was not switched \
                     on for this server."
                } else {
                    "Smoozy's service on the server did not respond. Telegram on this server was \
                     not restarted with its new settings."
                }
                .to_string(),
            }
        }
        WhyTheRolesStateWasNotWritten::TheMachineSaid(said) => {
            TheRunsEnding::AStepFailed { step, said }
        }
    }
}

/// **Why `Rented-server runtime` W8's write of the role's desired state did
/// not happen.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhyTheRolesStateWasNotWritten {
    NoRandomness,
    TheSupervisorDidNotAnswer,
    /// **The supervisor answered without saying what it did with the role
    /// worker** (`Rented-server runtime` W9), in its own words.
    TheMachineSaid(String),
}

/// **`Rented-server runtime` W8's act, either way: the role's durable desired
/// state written through the ordinary command path** — `Telegram` §20's third
/// item and `Telegram` L2. **One home for it**, used by `Server Setup` R1 item
/// 8's step and by the delivery on a connection (the 38's piece 11).
///
/// **The supervisor says what it did with the role worker**
/// (`Rented-server runtime` W9), and an answer that says nothing is not a state
/// that was written.
pub async fn the_roles_desired_state_is_written<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    package_area: &str,
    enabled: bool,
) -> Result<(), WhyTheRolesStateWasNotWritten> {
    let Ok(command) = Identity::issue(IdentityKind::Operation) else {
        return Err(WhyTheRolesStateWasNotWritten::NoRandomness);
    };
    let request = Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        command,
        if enabled {
            asked::THE_TELEGRAM_ROLE_IS_ENABLED_REQUEST
        } else {
            asked::THE_TELEGRAM_ROLE_IS_DISABLED_REQUEST
        },
    );
    let Some(answered) = ask(sessions, package_area, &request).await else {
        return Err(WhyTheRolesStateWasNotWritten::TheSupervisorDidNotAnswer);
    };
    if answered
        .fields
        .text(asked::WHAT_THE_ROLE_WORKER_DID_FIELD)
        .is_none()
    {
        return Err(WhyTheRolesStateWasNotWritten::TheMachineSaid(the_machines_own_words(
            &answered,
        )));
    }
    Ok(())
}

/// **`Telegram` L4's sentence of 2026-10-12 and §20's: after placing the
/// role's material on a machine whose role is enabled, *disabled* then
/// *enabled* on the same connection**, so the running worker — which reads its
/// material only at its start (`Telegram` K2) — is stopped and a new one reads
/// what was placed. Two requests that already exist;
/// `Rented-server runtime` W9 confines the restart to the role worker, so a
/// render going on there is left alone.
///
/// **Each half moves the told-record on its own acknowledgement**, so a pair
/// cut off after its *disabled* leaves the server owed *enabled* — which the
/// next connection sends — rather than recorded as on while it is off.
/// Added 2026-10-12, the Telegram worker's piece 7.
async fn the_running_worker_reads_what_was_placed<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    package_area: &str,
    telegram: &mut dyn ComposesTheTelegramMaterial,
) -> Result<(), WhyTheRolesStateWasNotWritten> {
    the_roles_desired_state_is_written(sessions, package_area, false).await?;
    telegram.the_disabled_state_was_acknowledged();
    the_roles_desired_state_is_written(sessions, package_area, true).await?;
    telegram.the_desired_state_was_acknowledged();
    Ok(())
}

/// **What one server has not been told, by `Telegram` L3's own derivation for
/// it** — the derivation Settings' *not reached* list is drawn from, so a
/// delivery that answers it is what empties that list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct WhatThisServerIsOwed {
    /// **The token file and the role configuration**: the token generation,
    /// the proved conversation, the label or the reporting settings is behind.
    /// Both files go together, because `Telegram` §20's material is composed
    /// whole and `Server Setup` G4's one vehicle places it whole.
    pub the_files: bool,
    /// **The role's desired state, where it is behind**: `true` to enable,
    /// `false` to disable.
    pub the_desired_state: Option<bool>,
}

impl WhatThisServerIsOwed {
    pub fn is_nothing(&self) -> bool {
        !self.the_files && self.the_desired_state.is_none()
    }
}

/// **What one connection's telling came to** (`Telegram` §19's ordered unit
/// per machine: placed, acknowledged, then the told-record's mark — a failure
/// earlier leaves the later steps not attempted).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheTellingCameTo {
    /// The server holds no material under its installation now, or is told
    /// everything already.
    NothingWasOwed,
    /// **Everything owed was told and acknowledged**, and the told-record moved.
    Told(WhatThisServerIsOwed),
    /// The material would not compose, so nothing was placed or switched.
    TheMaterialWouldNotCompose(String),
    /// **A file the machine did not take**, in its own words. The role's state
    /// is not switched after it: a state written over files that did not land
    /// would start a worker with nothing current to read (`Telegram` K2).
    AFileWasNotTaken(String),
    /// The role's desired state was not written.
    TheStateWasNotWritten(WhyTheRolesStateWasNotWritten),
}

/// **`Telegram` L2: what a server was not told, delivered on the ordinary
/// command path over a connection that stands** — the 38's piece 11.
///
/// **Delivered only to a server that holds the material** under the
/// installation it answers under now (`Telegram` L1's entries count only under
/// it): the 38's map, hole 14 — *"a machine that holds the material is told on
/// the ordinary path (L2); `Server Setup` G2's entry point is only for a machine
/// with none"*.
///
/// **The order is `Server Setup` R1 item 8's**: the files first, through
/// `Server Setup` G4's one vehicle (`acts::place_the_telegram_material`,
/// reused and never copied), and the state after, because the role worker reads
/// the token at start (`Telegram` K2). **Each telling moves the told-record on
/// its own acknowledgement and on nothing else** (`Telegram` L1, L2).
///
/// **Where the role is enabled there, the files are followed by *disabled*
/// then *enabled*** (`Telegram` L4's sentence of 2026-10-12), and the files
/// count as told only once that pair is answered. Added 2026-10-12, the Telegram
/// worker's piece 7.
///
/// **The lock policy is not told here**: the machine's program answers no
/// request that carries one and holds no store for it, so nothing could
/// acknowledge it (the 38's piece 11's account).
pub async fn what_the_server_was_not_told_is_told<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    package_area: &str,
    telegram: &mut TheMaterialFromTheRecord<'_>,
) -> WhatTheTellingCameTo {
    let owed = telegram.what_this_server_is_owed();
    if owed.is_nothing() {
        return WhatTheTellingCameTo::NothingWasOwed;
    }
    if owed.the_files {
        let material = match telegram.composed() {
            Ok(material) => material,
            Err(said) => return WhatTheTellingCameTo::TheMaterialWouldNotCompose(said),
        };
        if !the_material_lands_in_the_package_area(&material, package_area) {
            return WhatTheTellingCameTo::AFileWasNotTaken(THE_FILES_WOULD_NOT_HAVE_LANDED.to_string());
        }
        let the_role_is_enabled_there = telegram.the_role_is_enabled_there();
        let mut files = OnThisServersPoint { sessions };
        let placed = acts::place_the_telegram_material(&mut files, material.the_files()).await;
        let refused: Vec<String> = placed
            .iter()
            .filter(|(_, went)| !went.it_was_carried_out())
            .map(|(path, went)| format!("{path}: {}", went.in_words()))
            .collect();
        if !refused.is_empty() {
            telegram.the_placement_was_answered(&material, &placed);
            return WhatTheTellingCameTo::AFileWasNotTaken(refused.join("; "));
        }
        // **`Telegram` L4's sentence of 2026-10-12: a running worker is
        // started again before the files count as told**, because a worker
        // that has not read them has not been told them. A pair that does not finish leaves the
        // files owed, and the next connection places them and tries again.
        if the_role_is_enabled_there {
            if let Err(why) =
                the_running_worker_reads_what_was_placed(sessions, package_area, telegram).await
            {
                return WhatTheTellingCameTo::TheStateWasNotWritten(why);
            }
        }
        telegram.the_placement_was_answered(&material, &placed);
    }
    if let Some(enabled) = owed.the_desired_state {
        if let Err(why) = the_roles_desired_state_is_written(sessions, package_area, enabled).await {
            return WhatTheTellingCameTo::TheStateWasNotWritten(why);
        }
        telegram.the_roles_state_was_acknowledged(enabled);
    }
    WhatTheTellingCameTo::Told(owed)
}

/// **`Telegram` L2 on a connection that has just become live** — the 38's
/// piece 11. Started by `crate::servers::a_period` at `Servers` Q1 step 7's
/// hand-over, **on a thread of its own**, because every request it makes is
/// served from the period's own thread.
///
/// **Only on the confirmed branch, and only for the connection that read it**,
/// the rule step 7 keeps: a machine whose installation is not Smoozy's
/// confirmed one holds nothing Smoozy told. **Nothing while a setup run is
/// being driven on that server**: `Server Setup` Y9 holds one setup at a time,
/// and a run with Telegram in it carries R1 item 8.
///
/// **Settings' section is posted again where anything was told**, so its *not
/// reached* list empties on the connection (`Telegram` L3, derived when shown).
///
/// **One telling per server at a time** (the live Telegram pass, fault 11):
/// one asked for while another runs on that server is run once that one ends,
/// over the newest connection it was asked on — see [`TheTellingsUnderWay`].
pub fn the_tellings_are_delivered_on<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    server: String,
    sessions: ThisServersSessions,
) {
    let under_way = the_tellings_under_way();
    let Some(sessions) = under_way.asked(&server, sessions) else {
        return;
    };
    let the_server = server.clone();
    let started = std::thread::Builder::new()
        .name("smoozy telling".to_string())
        .spawn(move || {
            let mut sessions = sessions;
            loop {
                let came_to = the_tellings_on_one_connection(&app, &server, &sessions);
                // **`Server Setup` G2a: taken after the tellings**, which can
                // move the role, so a connection and a move of the switch
                // both read it here.
                the_telegram_role_is_read(&app, &server);
                // **And where the material would not compose**: a token the store
                // no longer holds has forgotten the stored fact, which the section
                // draws (the Creator's eleventh notes, note 2).
                if matches!(
                    came_to,
                    Some(WhatTheTellingCameTo::Told(_) | WhatTheTellingCameTo::TheMaterialWouldNotCompose(_))
                ) {
                    the_telegram_section_is_posted_again(&app);
                }
                match the_tellings_under_way().ended(&server) {
                    Some(newest) => sessions = newest,
                    None => break,
                }
            }
        });
    if started.is_err() {
        under_way.not_started(&the_server);
    }
}

/// **`Server Setup` G2a: whether the role is enabled, read off the machine and
/// held for the page and Queue**, which take no reading themselves and so wait
/// on nothing. **Off the drawing thread, and only while Telegram is on.** With
/// no package area recorded, or no answer, it holds nothing and nothing is said.
/// The page is told whatever came back, since a run's ending may be waiting on
/// it; Queue only where it moved.
pub fn the_telegram_role_is_read<R: tauri::Runtime>(app: &tauri::AppHandle<R>, server: &str) {
    use tauri::Manager;
    let Some(point) = app
        .try_state::<std::sync::Arc<crate::servers::a_period::ThePeriodsThisRun>>()
        .and_then(|periods| periods.the_sessions_of(server))
        .filter(ThisServersSessions::a_connection_is_live)
    else {
        return;
    };
    let Some(started) = app.try_state::<std::sync::Mutex<crate::start::Started>>() else {
        return;
    };
    let package_area = {
        let Ok(started) = started.lock() else {
            return;
        };
        let Some(record) = started.record.as_ref() else {
            return;
        };
        if !crate::record::settings::the_telegram_settings(record.connection()).is_ok_and(|telegram| telegram.switched_on) {
            return;
        }
        crate::record::server_rows::all(record.connection())
            .ok()
            .and_then(|rows| rows.into_iter().find(|row| row.local_server_identifier == server))
            .as_ref()
            .and_then(the_package_area_of)
    };
    let asked = crate::servers::published::a_reading_of_the_telegram_role_begins(server);
    let (app, server) = (app.clone(), server.to_string());
    let _ = std::thread::Builder::new().name("smoozy role reading".to_string()).spawn(move || {
        let enabled = match (package_area, tokio::runtime::Builder::new_current_thread().enable_all().build()) {
            (Some(area), Ok(runtime)) => runtime.block_on(the_telegram_roles_reading(&point, &area)),
            _ => None,
        };
        let moved = crate::servers::published::a_reading_of_the_telegram_role_came_back(&server, asked, enabled);
        crate::surface::render_settings::the_telegram_role_was_read(&app, &server);
        if moved {
            crate::surface::the_queue::the_queue_is_told_again(&app);
        }
    });
}

/// **The live Telegram pass, fault 11: what changed is told at once to every
/// server connected now**, not on its next connection (Specs `Telegram` :37 as
/// changed 2026-10-31). Called by the act that changed it — the switch either
/// way, a bot kept, a replacement token saved — once the change is written and
/// the application lock let go. **Each server is told exactly as a connection
/// becoming live tells it** ([`the_tellings_are_delivered_on`]), so what is
/// owed, the order and the told-record's marks are that path's; a server that
/// is not connected is told on its next connection, as before, and stays named
/// in Settings until then.
pub fn the_tellings_are_delivered_on_every_live_connection<R: tauri::Runtime>(app: &tauri::AppHandle<R>) {
    use tauri::Manager;
    let Some(periods) = app.try_state::<std::sync::Arc<crate::servers::a_period::ThePeriodsThisRun>>() else {
        return;
    };
    let servers = {
        let Some(started) = app.try_state::<std::sync::Mutex<crate::start::Started>>() else {
            return;
        };
        let Ok(started) = started.lock() else {
            return;
        };
        let Some(record) = started.record.as_ref() else {
            return;
        };
        match crate::record::server_rows::all(record.connection()) {
            Ok(rows) => rows.into_iter().map(|row| row.local_server_identifier).collect::<Vec<_>>(),
            Err(_) => return,
        }
    };
    for (server, sessions) in the_servers_connected_now(&periods, servers) {
        the_tellings_are_delivered_on(app.clone(), server, sessions);
    }
}

/// **The servers whose connection is live now and settled on the confirmed
/// branch**, each with its point — the rule [`the_tellings_on_one_connection`]
/// checks again when it runs. A server whose connection is being made again,
/// or that has none, is not here.
pub(crate) fn the_servers_connected_now(
    periods: &crate::servers::a_period::ThePeriodsThisRun,
    servers: impl IntoIterator<Item = String>,
) -> Vec<(String, ThisServersSessions)> {
    servers
        .into_iter()
        .filter_map(|server| {
            let read = periods.what_the_last_connection_read(&server)?;
            if read.check != smoozy_state::identities::InstallationCheck::PresentAndEqualConfirmed
                || !read.on_the_connection_that_is_live
            {
                return None;
            }
            let sessions = periods.the_sessions_of(&server)?;
            Some((server, sessions))
        })
        .collect()
}

/// **Which servers a telling runs on now, and the connection the next one is
/// owed on.** A telling reads what is owed when it starts; two at once on one
/// server could each write the role's state, and one started before a change
/// would not carry it. So a telling asked while one runs is held — the newest
/// connection only — and run once that one ends.
pub(crate) struct TheTellingsUnderWay<S> {
    held: std::sync::Mutex<std::collections::HashMap<String, Option<S>>>,
}

impl<S> Default for TheTellingsUnderWay<S> {
    fn default() -> Self {
        TheTellingsUnderWay { held: std::sync::Mutex::new(std::collections::HashMap::new()) }
    }
}

impl<S> TheTellingsUnderWay<S> {
    /// A telling is asked for: `Some` is the connection to start it on now;
    /// `None` is a telling already running there, which runs this one after.
    pub(crate) fn asked(&self, server: &str, sessions: S) -> Option<S> {
        let Ok(mut held) = self.held.lock() else {
            return Some(sessions);
        };
        match held.get_mut(server) {
            Some(next) => {
                *next = Some(sessions);
                None
            }
            None => {
                held.insert(server.to_string(), None);
                Some(sessions)
            }
        }
    }

    /// The running telling ended: the connection to run the next one on, or
    /// `None`, and then nothing runs there any more.
    pub(crate) fn ended(&self, server: &str) -> Option<S> {
        let Ok(mut held) = self.held.lock() else {
            return None;
        };
        let next = held.get_mut(server).and_then(Option::take);
        if next.is_none() {
            held.remove(server);
        }
        next
    }

    /// A telling that could not be started holds nothing.
    pub(crate) fn not_started(&self, server: &str) {
        if let Ok(mut held) = self.held.lock() {
            held.remove(server);
        }
    }
}

fn the_tellings_under_way() -> &'static TheTellingsUnderWay<ThisServersSessions> {
    static UNDER_WAY: std::sync::OnceLock<TheTellingsUnderWay<ThisServersSessions>> = std::sync::OnceLock::new();
    UNDER_WAY.get_or_init(TheTellingsUnderWay::default)
}

fn the_tellings_on_one_connection<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    server: &str,
    sessions: &ThisServersSessions,
) -> Option<WhatTheTellingCameTo> {
    use tauri::Manager;
    let periods = app.try_state::<std::sync::Arc<crate::servers::a_period::ThePeriodsThisRun>>()?;
    let read = periods.what_the_last_connection_read(server)?;
    if read.check != smoozy_state::identities::InstallationCheck::PresentAndEqualConfirmed
        || !read.on_the_connection_that_is_live
    {
        return None;
    }
    if the_kind_being_driven_on(server).is_some() {
        return None;
    }
    let started = app.try_state::<std::sync::Mutex<crate::start::Started>>()?;
    let record = crate::keys::TheRunsRecord {
        started: started.inner(),
    };
    let package_area = TheServersRowHere {
        record: &record,
        server: server.to_string(),
    }
    .the_row()
    .as_ref()
    .and_then(the_package_area_of)?;
    let mut telegram = TheMaterialFromTheRecord {
        record: &record,
        server: server.to_string(),
        store: crate::surface::keys::the_store_for_this_system(),
    };
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .ok()?;
    Some(runtime.block_on(what_the_server_was_not_told_is_told(
        sessions,
        &package_area,
        &mut telegram,
    )))
}

/// Settings' Telegram section, posted again from the record as it now stands.
fn the_telegram_section_is_posted_again<R: tauri::Runtime>(app: &tauri::AppHandle<R>) {
    use tauri::Manager;
    let (Some(started), Some(queue)) = (
        app.try_state::<std::sync::Mutex<crate::start::Started>>(),
        app.try_state::<std::sync::Arc<crate::events::TheOneQueue>>(),
    ) else {
        return;
    };
    let Ok(held) = started.lock() else {
        return;
    };
    crate::surface::telegram::the_section_is_posted(&held, &queue, None);
}

/// **`Server Setup` G2's dedicated entry point, on one connected server** —
/// *Install* on the Set up card (`Design/Server Setup § Adding Telegram to a
/// server already set up`; the 38's piece 11, hole 14).
///
/// **A run of `Server Setup` Y6b's own kind, *the Telegram placement***, so it
/// replaces nothing of the ready action's record. **It places the material,
/// sets the desired state to enabled, and does nothing else**: no other setup
/// step, no restart (`Server Setup` G2). **`Server Setup` §12's act line is
/// written where it reached *done***.
///
/// **It runs off the drawing thread** (P5) and blocks the thread it is given.
pub fn the_telegram_placement_on<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    point: &ThisServersSessions,
    server: &str,
) -> WhereTheSetUpGot {
    use tauri::Manager;
    let Some(started) = app.try_state::<std::sync::Mutex<crate::start::Started>>() else {
        return WhereTheSetUpGot::TheServerIsNotSetUpYet;
    };
    let record = crate::keys::TheRunsRecord {
        started: started.inner(),
    };
    let mut row = TheServersRowHere {
        record: &record,
        server: server.to_string(),
    };
    let mut telegram = TheMaterialFromTheRecord {
        record: &record,
        server: server.to_string(),
        store: crate::surface::keys::the_store_for_this_system(),
    };
    let runtime = match tokio::runtime::Builder::new_current_thread().enable_all().build() {
        Ok(runtime) => runtime,
        Err(_) => {
            return WhereTheSetUpGot::ThePlacementCouldNotBeCarriedOut {
                said: "Could not start the task.".to_string(),
            }
        }
    };
    let mut taking = TheOpeningReadingsOverThePoint { sessions: point };
    let came_to = runtime.block_on(the_telegram_material_is_placed(
        point,
        &mut taking,
        &mut row,
        &mut telegram,
        &mut TheSetUpTabAsks {
            app: app.clone(),
            server: server.to_string(),
        },
        &TheStopThisServerWasAsked {
            server: server.to_string(),
        },
    ));
    if matches!(
        came_to,
        WhereTheSetUpGot::TheRunEnded {
            ending: TheRunsEnding::ItReachedDone { .. },
            ..
        }
    ) {
        the_act_is_in_the_diary(
            app,
            server,
            smoozy_server_setup::diary::ActLine::TelegramsPieceWasPlacedByItsOwnAction {
                the_desired_state_was_set_to_enabled: true,
            },
        );
    }
    came_to
}

/// **`Server Setup` G2's dedicated entry point, driven** — the opening
/// readings every setup run takes afresh (`Server Setup` M1, `Server Setup`
/// M1a; `Rented-server runtime` K2), and a run of *the Telegram placement*
/// composed of `Server Setup`'s own [`ready_action::compose_the_telegram_placement`],
/// driven exactly as every other run is. **Nothing else is read or run**: the
/// steps' composition holds no step that would read the cards, the driver or
/// the unpack tool, so none of those readings is taken.
///
/// **`Server Setup` G3 is not reached from here**: the card offers *Install*
/// only on a connected server, and a server connects only where Smoozy's
/// program answers under a confirmed installation — so a package is always
/// there. A row with no confirmed installation answers that the server is not
/// set up yet, as a Blender install does.
pub async fn the_telegram_material_is_placed<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    taking: &mut dyn TakesTheOpeningReadings,
    row: &mut dyn KeepsTheServersRow,
    telegram: &mut dyn ComposesTheTelegramMaterial,
    asker: &mut dyn AsksForThePassword,
    stop: &dyn AsksToStop,
) -> WhereTheSetUpGot {
    let said = match the_opening_readings(taking).await {
        Ok(said) => said,
        Err(stopped) => return stopped,
    };
    let row_now = row.the_row();
    let Some(package_area) = row_now
        .as_ref()
        .filter(|row| row.last_confirmed_installation_identity.is_some())
        .and_then(the_package_area_of)
    else {
        return WhereTheSetUpGot::TheServerIsNotSetUpYet;
    };
    let asked_for = WhatWasAskedFor {
        install_this_blender: None,
        telegram_is_on: true,
        ..WhatWasAskedFor::default()
    };
    // **Only the two readings this run has.** No step composed here reads
    // anything else of `WhatTheMachineNeeds`, so nothing else is asked of the
    // machine and nothing is claimed for it.
    let needs = WhatTheMachineNeeds {
        kind: said.kind,
        family: said.family,
        this_run_fetches_a_blender: false,
        the_machine_already_holds_a_blender: false,
        the_unpack_tool_is_present: true,
        the_driver_answers: false,
        a_driver_is_already_installed: false,
        cards: smoozy_servers::hardware::Cards::CouldNotBeEstablished,
        telegram_is_on: true,
        the_drivers_version: None,
        the_optix_bar: None,
        anything_is_live: false,
        the_driver_packages: readings::WhatTheDriverPackagesSay::NotRead,
    };
    let mut run = match TheRunBeingDriven::opened(
        sessions,
        &package_area,
        SetupKind::TheTelegramPlacement,
        ready_action::compose_the_telegram_placement(),
        needs,
        // **No restart is composed**, so no period's mark is wanted.
        None,
        Vec::new(),
        stop,
        asker,
    )
    .await
    {
        Ok(run) => run,
        Err(ending) => {
            return WhereTheSetUpGot::TheRunEnded {
                at: Boundary::Started,
                ending,
            }
        }
    };
    run.driven(
        HowElevationIsObtained::APasswordIsOwedAndNoneIsHeld,
        &asked_for,
        telegram,
        asker,
        stop,
    )
    .await
}

/// **`Server Setup` B15's removal, as `Server Setup` Y6b's own kind of run**, started off the
/// drawing thread (P5) and blocking the thread it is given — the Telegram
/// placement's shape. The 38's piece 14c.
pub fn the_removal_on<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    point: &ThisServersSessions,
    server: &str,
    removal: TheRemovalAsked,
) -> WhereTheSetUpGot {
    use tauri::Manager;
    let Some(started) = app.try_state::<std::sync::Mutex<crate::start::Started>>() else {
        return WhereTheSetUpGot::TheServerIsNotSetUpYet;
    };
    let record = crate::keys::TheRunsRecord {
        started: started.inner(),
    };
    let mut row = TheServersRowHere {
        record: &record,
        server: server.to_string(),
    };
    let runtime = match tokio::runtime::Builder::new_current_thread().enable_all().build() {
        Ok(runtime) => runtime,
        Err(_) => {
            return WhereTheSetUpGot::ThePlacementCouldNotBeCarriedOut {
                said: "Could not start the task.".to_string(),
            }
        }
    };
    let mut taking = TheOpeningReadingsOverThePoint { sessions: point };
    runtime.block_on(a_blender_is_removed(
        point,
        &mut taking,
        &mut row,
        removal,
        &mut TheSetUpTabAsks {
            app: app.clone(),
            server: server.to_string(),
        },
        &TheStopThisServerWasAsked {
            server: server.to_string(),
        },
        // **`Server Setup` §12's act line, through the module's own diary.**
        // Added 2026-10-16, step 10c's owed piece 12.
        &mut |line| the_act_is_in_the_diary(app, server, line),
    ))
}

/// **`Server Setup` B15's removal, driven**: the opening readings every setup
/// run takes afresh, and a run of *a Blender removal* composed of
/// `Server Setup`'s own [`ready_action::compose_a_blender_removal`] — the
/// readings, the removal, *done* — driven exactly as every other run is. The
/// removal step carries the executable, its answered version and `Server Setup` B14's
/// covered jobs ([`TheRemovalAsked`]); *done* carries no executable, so the
/// final check reads nothing, which is a run with no Blender in it. The 38's
/// piece 14c.
///
/// **A row with no confirmed installation answers that the server is not set
/// up yet**, as a Blender install does.
///
/// **`diary` is handed `Server Setup` §12's *a Blender was taken off***, once,
/// where the run reached *done* — which it reaches only past the removal's own
/// boundary *removed* — on either of `Server Setup` B15's routes
/// ([`the_removal_line`]). Added 2026-10-16, step 10c's owed piece 12.
pub async fn a_blender_is_removed<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    taking: &mut dyn TakesTheOpeningReadings,
    row: &mut dyn KeepsTheServersRow,
    removal: TheRemovalAsked,
    asker: &mut dyn AsksForThePassword,
    stop: &dyn AsksToStop,
    diary: &mut dyn FnMut(smoozy_server_setup::diary::ActLine),
) -> WhereTheSetUpGot {
    let said = match the_opening_readings(taking).await {
        Ok(said) => said,
        Err(stopped) => return stopped,
    };
    let row_now = row.the_row();
    let Some(package_area) = row_now
        .as_ref()
        .filter(|row| row.last_confirmed_installation_identity.is_some())
        .and_then(the_package_area_of)
    else {
        return WhereTheSetUpGot::TheServerIsNotSetUpYet;
    };
    let asked_for = WhatWasAskedFor {
        install_this_blender: None,
        telegram_is_on: false,
        ..WhatWasAskedFor::default()
    };
    // **Only the two readings this run has**, as the Telegram placement's.
    let needs = WhatTheMachineNeeds {
        kind: said.kind,
        family: said.family,
        this_run_fetches_a_blender: false,
        the_machine_already_holds_a_blender: false,
        the_unpack_tool_is_present: true,
        the_driver_answers: false,
        a_driver_is_already_installed: false,
        cards: smoozy_servers::hardware::Cards::CouldNotBeEstablished,
        telegram_is_on: false,
        the_drivers_version: None,
        the_optix_bar: None,
        anything_is_live: false,
        the_driver_packages: readings::WhatTheDriverPackagesSay::NotRead,
    };
    // **`Server Setup` B15's second route is an elevated act**, so the login's
    // elevation is read as every elevated run reads it (`Server Setup` M3's
    // probe) — passwordless where the probe succeeds, and otherwise
    // `Server Setup` E2's offer at the moment the removal needs it. **The plain
    // deletion needs none and asks nothing.** Step 10c's owed piece 6.
    let words = asker.where_the_words_go();
    let elevation = if removal.the_managers_package.is_some() {
        the_elevation_this_login_has_telling(
            sessions,
            HowElevationIsObtained::APasswordIsOwedAndNoneIsHeld,
            words.as_ref(),
        )
        .await
    } else {
        HowElevationIsObtained::APasswordIsOwedAndNoneIsHeld
    };
    let mut run = match TheRunBeingDriven::opened(
        sessions,
        &package_area,
        SetupKind::ABlenderRemoval,
        ready_action::compose_a_blender_removal(),
        needs,
        None,
        Vec::new(),
        stop,
        asker,
    )
    .await
    {
        Ok(run) => run,
        Err(ending) => {
            return WhereTheSetUpGot::TheRunEnded {
                at: Boundary::Started,
                ending,
            }
        }
    };
    let asked = removal.clone();
    run.the_removal = Some(removal);
    run.the_words_go = words;
    let came_to = run
        .driven(elevation, &asked_for, &mut TelegramIsNotOn, asker, stop)
        .await;
    if let Some(line) = the_removal_line(&came_to, &asked) {
        diary(line);
    }
    came_to
}

/// **`Server Setup` §12's *a Blender was taken off*** — the version the
/// removed build answered, and which of `Server Setup` B15's two routes
/// performed it: the machine's own software manager where the removal named
/// that manager's package, the folder deletion otherwise. Added 2026-10-16,
/// step 10c's owed piece 12.
///
/// **Written only where the run reached *done***, which comes after the
/// removal's boundary *removed*: a removal that failed, stopped or lost its
/// server took nothing off, and its card's sentence says what it left. **A
/// re-run that found the build already gone also reaches *done***
/// (`Server Setup` B15's *nothing to do*) and writes the line: it is still the
/// one removal the user asked for, and `Server Setup` §12 counts one per
/// removal asked for. **No line where the table read no version**: the form
/// `Server Setup` §12 fixes names the version the removed build
/// answered, and a build that never answered has none to name — as an install
/// that never got its answer writes none.
pub fn the_removal_line(
    came_to: &WhereTheSetUpGot,
    removal: &TheRemovalAsked,
) -> Option<smoozy_server_setup::diary::ActLine> {
    if !matches!(
        came_to,
        WhereTheSetUpGot::TheRunEnded {
            ending: TheRunsEnding::ItReachedDone { .. },
            ..
        }
    ) {
        return None;
    }
    let answered = smoozy_server_setup::blender::Version::parse(&removal.the_answered_version)?;
    Some(smoozy_server_setup::diary::ActLine::ABlenderWasTakenOff {
        answered,
        route: if removal.the_managers_package.is_some() {
            smoozy_server_setup::search::RemovalRoute::TheMachinesOwnSoftwareManager
        } else {
            smoozy_server_setup::search::RemovalRoute::TheFolderDeletion
        },
    })
}

/// One request to the machine's supervisor, over the per-server point.
async fn ask<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    package_area: &str,
    request: &Request,
) -> Option<Answered> {
    match sessions.ask_the_supervisor(package_area, request).await {
        smoozy_server_protocol::asking_the_supervisor::WhatCameBack::ItAnswered(answered) => {
            Some(answered)
        }
        _ => None,
    }
}

/// **`Server protocol and transfers` C8's *still in hand***, which is what an
/// accepted command answers.
fn it_is_still_in_hand(answered: &Answered) -> bool {
    answered
        .fields
        .text(asked::WHAT_BECAME_OF_IT_FIELD)
        .is_some_and(|said| said == asked::IT_IS_STILL_IN_HAND)
}

/// The machine's own words, whole and never mined for a cause.
fn the_machines_own_words(answered: &Answered) -> String {
    answered
        .fields
        .text(asked::THE_MACHINES_OWN_WORDS_FIELD)
        .unwrap_or("No details from the server.")
        .to_string()
}

/// **The runner's ruling of 2026-09-27, as a record is read: a restart command
/// refused while the connection stays up takes *restart issued* back.**
///
/// **Why a mark is taken back at all.** `Server Setup` Y4's paragraph under the
/// table makes *restart issued* the one boundary written just before its
/// command is sent (`Server Setup` T3), because nothing can be written after a
/// restart that goes. **A command the machine refuses leaves that mark standing
/// over a machine that never went down**, and a recovery reading it would carry
/// the setup on at the card check, on a boot that never happened, and report a
/// card that does not answer. **So the refusal is recorded the way any other
/// failed step is** (`Server Setup` S2a) — the ending *a step failed*, naming
/// the restart step, whose first boundary is *restart issued* — **and a record
/// that says so is read as standing at the boundary before it.**
///
/// **Narrow on purpose.** It is this one boundary and no other: every other
/// boundary is written after its step completed (`Server Setup` Y3), so a
/// failure recorded after it names a later step, never it. **The machine's own
/// record is not rewritten** — its reached list still holds the mark — so the
/// rule is kept wherever this desktop reads a record, and the machine's own
/// take-over (`Server Setup` S4 item 2) is read through the taken-over run's
/// record, which carries the failure (see
/// [`TheRunBeingDriven::the_steps_resume_after_the_inherited_marker`]).
fn the_restart_mark_was_taken_back(ending: Option<&str>, failed: Option<Boundary>) -> bool {
    ending == Some(asked::A_STEP_FAILED) && failed == Some(Boundary::RestartIssued)
}

/// The run's state, read off the answer.
fn the_state_in(answered: &Answered) -> TheRunsState {
    let how_many = |field: &str| -> usize {
        answered
            .fields
            .text(field)
            .and_then(|text| text.parse().ok())
            .unwrap_or(0)
    };
    let mut composed = Vec::new();
    for at in 0..how_many(asked::HOW_MANY_COMPOSED_BOUNDARIES_FIELD) {
        if let Some(named) = answered.fields.text(&asked::the_field_of_item(
            asked::A_COMPOSED_BOUNDARY,
            at,
            asked::THE_BOUNDARY_FIELD,
        )) {
            if let Some(boundary) = the_boundary_named(named) {
                composed.push(boundary);
            }
        }
    }
    let mut reached = Vec::new();
    for at in 0..how_many(asked::HOW_MANY_BOUNDARIES_REACHED_FIELD) {
        if let Some(named) = answered.fields.text(&asked::the_field_of_item(
            asked::A_BOUNDARY_REACHED,
            at,
            asked::THE_BOUNDARY_FIELD,
        )) {
            if let Some(boundary) = the_boundary_named(named) {
                reached.push(boundary);
            }
        }
    }
    // **`Server Setup` R1 item 4's whole not-found list, read at once.** A
    // list with a hole in it is not the whole list, so a missing item drops the
    // reading rather than half-taking it — which the selection then meets as
    // *nothing was missing*, the safe direction: nothing is installed and the
    // re-read says in the machine's own words that the chain is still unclean.
    let mut the_libraries_not_found = Vec::new();
    for at in 0..how_many(asked::HOW_MANY_LIBRARIES_NOT_FOUND_FIELD) {
        match answered.fields.text(&asked::the_field_of_item(
            asked::A_LIBRARY_NOT_FOUND,
            at,
            asked::THE_LIBRARY_FIELD,
        )) {
            Some(library) => the_libraries_not_found.push(library.to_string()),
            None => {
                the_libraries_not_found.clear();
                break;
            }
        }
    }
    let ending = answered
        .fields
        .text(asked::THE_RUNS_ENDING_FIELD)
        .map(str::to_string);
    let the_failed_step = (ending.as_deref() == Some(asked::A_STEP_FAILED))
        .then(|| answered.fields.text(asked::THE_FAILED_STEP_FIELD))
        .flatten()
        .and_then(the_boundary_named);
    // **The runner's ruling of 2026-09-27: a refused restart takes *restart
    // issued* back** — see [`the_restart_mark_was_taken_back`].
    if the_restart_mark_was_taken_back(ending.as_deref(), the_failed_step) {
        reached.retain(|boundary| *boundary != Boundary::RestartIssued);
    }
    TheRunsState {
        the_machines_own_words: answered
            .fields
            .text(asked::THE_MACHINES_OWN_WORDS_FIELD)
            .map(str::to_string),
        kind: answered
            .fields
            .text(asked::THE_RUNS_KIND_FIELD)
            .and_then(the_kind_named),
        composed,
        reached,
        the_failed_step,
        ending,
        the_step_in_flight: answered
            .fields
            .text(asked::THE_STEP_FIELD)
            .map(str::to_string),
        the_libraries_not_found,
        the_staging_area: answered
            .fields
            .text(asked::THE_SETUP_RUNS_STAGING_AREA_FIELD)
            .map(str::to_string),
        the_question_stands_at: answered
            .fields
            .text(asked::THE_QUESTION_STANDS_AT_VERSION_FIELD)
            .map(str::to_string),
        the_discard: answered.fields.text(asked::THE_DISCARD_FIELD).map(str::to_string),
        the_version_answered: answered
            .fields
            .text(asked::THE_VERSION_ANSWERED_FIELD)
            .map(str::to_string),
        the_count: the_count_in(answered),
        the_steps_words: the_steps_words_in(answered),
    }
}

/// **`Server Setup` Y7b's and `Server Setup` Y7c's count, read off the run's state** —
/// nothing where the bytes so far are absent or do not read; a whole that does
/// not read is read as none, and the count still stands.
fn the_count_in(answered: &Answered) -> Option<TheStepsCount> {
    let read = |field: &str| answered.fields.text(field).and_then(|text| text.parse::<u64>().ok());
    Some(TheStepsCount {
        so_far: read(asked::THE_BYTES_SO_FAR_FIELD)?,
        whole: read(asked::THE_BYTES_IN_THE_WHOLE_FIELD),
    })
}

/// A wait of the refresh cadence, on whatever runtime the driver is on.
async fn wait_a_moment(how_long: Duration) {
    tokio::time::sleep(how_long).await;
}

/// **The elevated command channel, over the per-server point.**
///
/// **It carries a machine password on a command's standard input**
/// (`Server protocol and transfers` §1.3; `Security and licence` §4), which is
/// the one value this seam carries and the reason it is not
/// [`crate::submitting::OnThisServersPoint`]: that one refuses the carriage by
/// name, because nothing it drives asks elevated.
struct TheElevatedChannel<'s, C: AHeldConnection> {
    sessions: &'s ThisServersSessions<C>,
    /// **`Server Setup` Y7a: the step these commands are, and where what they
    /// print goes as it prints.** Nothing where no screen keeps the run's
    /// words, and then each command is read whole at its end as before.
    words: Option<(Step, TheWordsGoHere)>,
}

impl<C: AHeldConnection> TheElevatedChannel<'_, C> {
    /// **One command through the point, its two streams told as they print
    /// where anyone keeps them** — `Server Setup` Y7a. The answer is the same
    /// whole result either way.
    async fn one_command(
        &self,
        command: smoozy_server_protocol::transport::CommandString,
        frame: Option<Vec<u8>>,
    ) -> Result<
        smoozy_server_protocol::transport::CommandResult,
        smoozy_server_protocol::transport::ssh::CommandFailure,
    > {
        let ran = match &self.words {
            None => self.sessions.a_command(command, frame).await,
            Some((step, go)) => {
                let (up, mut pieces) = tokio::sync::mpsc::unbounded_channel();
                let step = *step;
                the_pieces_are_told_while(
                    self.sessions.a_command_passing_up(command, frame, up),
                    &mut pieces,
                    |piece| go(step, &WhatTheServerPrinted::APiece(piece)),
                )
                .await
            }
        };
        ran.map_err(|_| smoozy_server_protocol::transport::ssh::CommandFailure::TheRequestDidNotReachTheMachine)
    }
}

/// **`Server Setup` Y7a item 1: the channel the readings that compose a run go
/// out on** — `Server Setup` R1 item 1's and `Server Setup` M3's probe, each
/// command's two streams told as they print under the run's first step
/// (`Step::Readings`), where `words` says a screen keeps them. **No value ever
/// rides it**: the readings ask nothing elevated. Added 2026-10-24.
fn the_readings_channel<'s, C: AHeldConnection>(
    sessions: &'s ThisServersSessions<C>,
    words: Option<&TheWordsGoHere>,
) -> TheElevatedChannel<'s, C> {
    TheElevatedChannel {
        sessions,
        words: words.cloned().map(|go| (Step::Readings, go)),
    }
}

/// **`Server Setup` Y7a: each piece a command prints, told the moment it
/// arrives, while the command is still running** — and every piece still
/// waiting told before the answer is handed back, so nothing printed is
/// missed and nothing is told after the step it belongs to. Added 2026-10-18,
/// the Creator's thirteenth notes, piece H.
async fn the_pieces_are_told_while<T>(
    ran: impl std::future::Future<Output = T>,
    pieces: &mut tokio::sync::mpsc::UnboundedReceiver<APiecePrinted>,
    mut told: impl FnMut(APiecePrinted),
) -> T {
    tokio::pin!(ran);
    loop {
        tokio::select! {
            biased;
            Some(piece) = pieces.recv() => told(piece),
            came = &mut ran => {
                while let Ok(piece) = pieces.try_recv() {
                    told(piece);
                }
                return came;
            }
        }
    }
}

impl<C: AHeldConnection> smoozy_servers::readings::CommandChannel for TheElevatedChannel<'_, C> {
    async fn run(
        &mut self,
        command: smoozy_server_protocol::transport::CommandString,
    ) -> Result<
        smoozy_server_protocol::transport::CommandResult,
        smoozy_server_protocol::transport::ssh::CommandFailure,
    > {
        self.one_command(command, None).await
    }
}

impl<C: AHeldConnection> smoozy_server_protocol::transport::CarriesAValueOnACommandsStandardInput
    for TheElevatedChannel<'_, C>
{
    async fn run_carrying_a_value_on_its_standard_input(
        &mut self,
        command: smoozy_server_protocol::transport::CommandString,
        value: smoozy_security_and_licence::plaintext::Plaintext,
    ) -> Result<
        smoozy_server_protocol::transport::CommandResult,
        smoozy_server_protocol::transport::ssh::CommandFailure,
    > {
        let carried = value.expose(|bytes| bytes.to_vec());
        self.one_command(command, Some(carried)).await
    }
}

/// **What an admin step's channel saw while a command ran, that ended the
/// step.** Added 2026-09-27, step 10c's eighth-pass fix piece 1.
#[derive(Debug, Clone)]
enum WhatTheAdminStepSaw {
    /// **`Server Setup` Y8: the user stopped the run.**
    TheUserStopped,
    /// **The run's own state reading showed an ending**, as the machine holds
    /// it.
    TheRunEnded(TheRunsState),
}

/// **The elevated channel for a run's admin steps, with `Server Setup` Y7's
/// refresh and `Server Setup` Y8's stop kept through every command.** Added
/// 2026-09-27, step 10c's eighth-pass fix piece 1.
///
/// **What it keeps, and why here.** `Server Setup` Y7's third bullet: the
/// refresh *"continues at its cadence for as long as the run is open, elevated
/// steps included … a driver install or a long dependency chain runs for
/// minutes without a worker command and must not end a run that is proceeding
/// correctly"*. **Until this was written nothing refreshed the run while an
/// admin command ran** — the refresh was taken between worker steps and inside
/// the password ask only — and step 10c's eighth pass on Hyperstack watched the
/// machine end a first setup at *started* for want of one. `Server Setup` Y8:
/// *"A stop arriving during an elevated step ends the driving side's part at
/// once — the desktop stops issuing and closes the step's channel."*
///
/// **So every command waits here against the run's cadence.** At each tick the
/// stop is read, then the run's own state; a stop, or a reading that shows an
/// ending, **gives the command up** — which closes its channel at the point
/// (`crate::servers::sessions`) — **and every later command on this channel is
/// refused without being sent.** Whoever called it then reads
/// [`Self::seen`].
///
/// **A reading that does not come back is not a dead run** (`Server protocol
/// and transfers` C17a): only one that shows an ending stops the step.
///
/// **The cadence is kept across commands, not started again at each**, so a
/// string of short admin commands cannot starve the refresh.
///
/// **At a learned session limit of one the refresh cannot go out beside the
/// command** — `Server Setup` §13 item 7's paragraph of 2026-09-27 — and waits
/// at the point until the command ends. That is the residue the plan names.
struct TheAdminStepsChannel<'s, 'a, C: AHeldConnection> {
    sessions: &'s ThisServersSessions<C>,
    /// **`Server Setup` Y7a: the step, and where its words go as they print.**
    words: Option<(Step, TheWordsGoHere)>,
    package_area: String,
    run: Identity,
    stop: &'a dyn AsksToStop,
    the_next_refresh: tokio::time::Instant,
    seen: Option<WhatTheAdminStepSaw>,
    /// **`Server Setup` Y7d: about a second while the command's streams
    /// deliver bytes, the keepalive cadence after five quiet refreshes.**
    /// Added 2026-10-25, the eighteenth notes, piece 2c.
    cadence: driving::TheRefreshCadence,
    /// When the command's streams last delivered a piece.
    the_last_piece: std::sync::Arc<std::sync::Mutex<Option<std::time::Instant>>>,
}

impl<C: AHeldConnection> TheAdminStepsChannel<'_, '_, C> {
    /// **One command, kept alive.** Nothing is sent once the run is seen
    /// ended or stopped.
    async fn kept_alive<F>(
        &mut self,
        sent: F,
    ) -> Result<
        smoozy_server_protocol::transport::CommandResult,
        smoozy_server_protocol::transport::ssh::CommandFailure,
    >
    where
        F: std::future::Future<
            Output = Result<
                smoozy_server_protocol::transport::CommandResult,
                smoozy_server_protocol::transport::ssh::CommandFailure,
            >,
        >,
    {
        let given_up =
            || Err(smoozy_server_protocol::transport::ssh::CommandFailure::TheRequestDidNotReachTheMachine);
        tokio::pin!(sent);
        loop {
            tokio::select! {
                biased;
                ran = &mut sent => return ran,
                () = tokio::time::sleep_until(self.the_next_refresh) => {
                    // **`Server Setup` Y7d: words are moving where a piece
                    // came within the last second.**
                    let moving = self
                        .the_last_piece
                        .lock()
                        .ok()
                        .and_then(|at| *at)
                        .is_some_and(|at| at.elapsed() <= driving::THE_REFRESH_CADENCE_WHILE_WORDS_MOVE);
                    self.cadence.a_reading_brought(moving);
                    self.the_next_refresh =
                        tokio::time::Instant::now() + self.cadence.until_the_next_reading();
                    if self.stop.the_user_asked_to_stop() {
                        self.seen = Some(WhatTheAdminStepSaw::TheUserStopped);
                        return given_up();
                    }
                    if let Some(state) = the_state_of_the_run(
                        self.sessions,
                        &self.package_area,
                        &self.run,
                    )
                    .await
                    {
                        if state.ending.is_some() {
                            self.seen = Some(WhatTheAdminStepSaw::TheRunEnded(state));
                            return given_up();
                        }
                    }
                }
            }
        }
    }

    /// **What it saw that ended the step**, where it saw anything.
    fn seen(&mut self) -> Option<WhatTheAdminStepSaw> {
        self.seen.take()
    }
}

impl<C: AHeldConnection> smoozy_servers::readings::CommandChannel for TheAdminStepsChannel<'_, '_, C> {
    async fn run(
        &mut self,
        command: smoozy_server_protocol::transport::CommandString,
    ) -> Result<
        smoozy_server_protocol::transport::CommandResult,
        smoozy_server_protocol::transport::ssh::CommandFailure,
    > {
        if self.seen.is_some() {
            return Err(smoozy_server_protocol::transport::ssh::CommandFailure::TheRequestDidNotReachTheMachine);
        }
        let sessions = self.sessions;
        let words = self.words.clone();
        self.kept_alive(async move {
            let mut channel = TheElevatedChannel { sessions, words };
            smoozy_servers::readings::CommandChannel::run(&mut channel, command).await
        })
        .await
    }
}

impl<C: AHeldConnection> smoozy_server_protocol::transport::CarriesAValueOnACommandsStandardInput
    for TheAdminStepsChannel<'_, '_, C>
{
    async fn run_carrying_a_value_on_its_standard_input(
        &mut self,
        command: smoozy_server_protocol::transport::CommandString,
        value: smoozy_security_and_licence::plaintext::Plaintext,
    ) -> Result<
        smoozy_server_protocol::transport::CommandResult,
        smoozy_server_protocol::transport::ssh::CommandFailure,
    > {
        if self.seen.is_some() {
            return Err(smoozy_server_protocol::transport::ssh::CommandFailure::TheRequestDidNotReachTheMachine);
        }
        let sessions = self.sessions;
        let words = self.words.clone();
        self.kept_alive(async move {
            let mut channel = TheElevatedChannel { sessions, words };
            smoozy_server_protocol::transport::CarriesAValueOnACommandsStandardInput::run_carrying_a_value_on_its_standard_input(
                &mut channel,
                command,
                value,
            )
            .await
        })
        .await
    }
}

/// **How `Server Setup` T1's restart command went**, told apart the way the
/// runner's ruling 1 of 2026-09-27 asks. Added the same day.
enum HowTheRestartCommandWent {
    /// **The restart took the connection with it**, or nothing says it did
    /// not — the safe direction ([`TheRunBeingDriven::the_restart_command_is_sent`]).
    TheConnectionWentWithIt,
    /// **Refused while the connection stayed up**, with the machine's words,
    /// or Smoozy's own record of the refusal where it printed none.
    ItWasRefused { said: String },
}

/// **The elevated channel, keeping the one reading the restart's outcome turns
/// on: whether its last command came back with an exit status at all.** Added
/// 2026-09-27.
///
/// **Nothing is read out of the machine's words** — `Security and licence` §13
/// rule 5 — only whether the transport brought a status back, which
/// `Server protocol and transfers`' own `CommandResult` says is *"absent where
/// the channel closed without one"*. `Some(None)` is a command that came back
/// with no status; `None` is a command that did not come back at all.
struct TheRestartsOwnChannel<'s, C: AHeldConnection> {
    inner: TheElevatedChannel<'s, C>,
    the_last_status: Option<Option<u32>>,
}

impl<C: AHeldConnection> smoozy_servers::readings::CommandChannel for TheRestartsOwnChannel<'_, C> {
    async fn run(
        &mut self,
        command: smoozy_server_protocol::transport::CommandString,
    ) -> Result<
        smoozy_server_protocol::transport::CommandResult,
        smoozy_server_protocol::transport::ssh::CommandFailure,
    > {
        self.the_last_status = None;
        let result = self.inner.run(command).await;
        if let Ok(result) = &result {
            self.the_last_status = Some(result.status);
        }
        result
    }
}

impl<C: AHeldConnection> smoozy_server_protocol::transport::CarriesAValueOnACommandsStandardInput
    for TheRestartsOwnChannel<'_, C>
{
    async fn run_carrying_a_value_on_its_standard_input(
        &mut self,
        command: smoozy_server_protocol::transport::CommandString,
        value: smoozy_security_and_licence::plaintext::Plaintext,
    ) -> Result<
        smoozy_server_protocol::transport::CommandResult,
        smoozy_server_protocol::transport::ssh::CommandFailure,
    > {
        self.the_last_status = None;
        let result = self
            .inner
            .run_carrying_a_value_on_its_standard_input(command, value)
            .await;
        if let Ok(result) = &result {
            self.the_last_status = Some(result.status);
        }
        result
    }
}

/// Where an origin is what a step is handed.
pub fn how_elevation_is_obtained(held: Option<&Origin>) -> HowElevationIsObtained<'_> {
    match held {
        Some(origin) => HowElevationIsObtained::APasswordIsHeld(origin),
        None => HowElevationIsObtained::APasswordIsOwedAndNoneIsHeld,
    }
}

/// **Where the package area of a server is, read out of its row** — one place
/// works it out and this driver asks it rather than composing a second answer.
pub fn the_package_area_of(row: &ServerRow) -> Option<String> {
    match where_the_package_area_is(row) {
        crate::servers::installation::WhereThePackageAreaIs::At(area) => Some(area),
        _ => None,
    }
}

/// `Server Setup` R1 item 1's own card reading, as the run composition takes
/// it. **Kept here so the one translation from `Servers`' two readings to
/// `Server Setup`'s conclusion is written once.**
pub fn the_cards_from(scan: smoozy_servers::hardware::BusScan, driver: smoozy_servers::hardware::DriverAnswer, kind: MachineKind) -> Cards {
    ready_action::read_card_presence(scan, driver, kind)
}

// ════════════════════════════════════════════════════════════════════════════
// The real ends
// ════════════════════════════════════════════════════════════════════════════

/// **`Server protocol and transfers` §5's transaction, over the connection the
/// point lent whole.**
///
/// **It opens the file-transfer session itself and gives it back with the
/// act**, which is what `ThePlacementsSurface` is: §1.1's two mechanisms held
/// together for the length of one transaction.
pub struct OverTheLentConnection;

impl PlacesTheVerifiedRelease<smoozy_server_protocol::transport::ssh::Connection>
    for OverTheLentConnection
{
    fn place<'a>(
        &'a mut self,
        connection: &'a mut smoozy_server_protocol::transport::ssh::Connection,
        needs: WhatThePlacementNeeds<'a>,
    ) -> ItsOwnFuture<'a, Result<ThePlacementsOutcome, WhyThePlacementCouldNotBeCarriedOut>> {
        Box::pin(async move {
            let channel = connection.open_a_subsystem_channel().await.map_err(|why| {
                WhyThePlacementCouldNotBeCarriedOut::TheMachineWouldNot(format!(
                    "Could not open an SFTP session: {}",
                    why.in_words()
                ))
            })?;
            let mut subsystem =
                smoozy_server_protocol::subsystem::client::Subsystem::negotiate(channel).await?;
            let mut surface = smoozy_server_protocol::placement::ThePlacementsSurface::of(
                &mut subsystem,
                connection,
            );
            smoozy_server_protocol::placement::place_a_verified_release(&mut surface, needs).await
        })
    }
}

/// **The server row, over this run's own record.**
pub struct TheServersRowHere<'a> {
    pub record: &'a dyn crate::keys::LendsTheRecord,
    pub server: String,
}

/// **`Telegram` §20's list for one server, out of the desktop's own record and
/// its secret store** — the composing half of `Server Setup` R1 item 8.
///
/// **Nothing here decides what the list holds.** `Telegram`'s own
/// `the_material_for` composes it — the proved bot and its conversation, the
/// token the store holds for *that* bot, the server's recorded root and
/// installation, its label and the idle switch — and this is the caller that
/// module had never had.
pub struct TheMaterialFromTheRecord<'a> {
    pub record: &'a dyn crate::keys::LendsTheRecord,
    pub server: String,
    /// **The store, where this system has one.** No store, no token, and then
    /// no material — which is the honest answer rather than a placement with
    /// nothing in it.
    pub store: Option<Box<dyn smoozy_security_and_licence::secret_store::SecretStore>>,
}

impl TheMaterialFromTheRecord<'_> {
    /// **`Server Setup` G2's *while Telegram is on*, asked of the record: the
    /// switch, the bot on record, and the record's own *token stored* fact for
    /// that bot** (`Security and licence` §3.0 rule 6). A read that does not
    /// answer is *off*, the side that claims nothing.
    ///
    /// **Changed 2026-10-17, the Creator's eleventh notes, note 2.** It used to
    /// compose the material there and then and answer *on* only where that
    /// worked — but a new server's folder and installation are recorded by the
    /// hand-over, later in the same run, so on every new server the answer was
    /// *off* and `Server Setup` R1 item 8 was dropped without a word, against
    /// `Specs/Telegram`'s *"automatically, as part of the ordinary first setup
    /// of a machine added while Telegram is on"*. **The material is composed at
    /// the step, after the hand-over**, and a composing that fails there is said
    /// in words ([`TheRunsEnding::TheTelegramMaterialWasNotComposed`]). **No
    /// store read is taken here.**
    pub fn it_is_on(&mut self) -> bool {
        let mut on = false;
        self.record.lend(&mut |connection| {
            on = the_record_says_telegram_is_on(connection);
        });
        on
    }

    /// **What this server has not been told, by `Telegram` L3's own
    /// derivation** — `told_record::the_servers_not_reached` over Settings' own
    /// current values ([`crate::surface::telegram::what_smoozy_tells_now`]),
    /// read for this server. The 38's piece 11.
    ///
    /// **Nothing is owed to a server that holds no material** under the
    /// installation it answers under now: one told nothing under it was never
    /// given Telegram there, and *Install* is its route (the map's hole 14).
    /// **Off, only the switch is owed**; on, the files where any of what they
    /// carry is behind, and the switch where it is.
    pub fn what_this_server_is_owed(&mut self) -> WhatThisServerIsOwed {
        let mut owed = WhatThisServerIsOwed::default();
        self.record.lend(&mut |connection| {
            owed = the_owed_reading(connection, &self.server).unwrap_or_default();
        });
        owed
    }

    /// **`Telegram` L1 and L2: the role's desired state acknowledged**, written
    /// under the installation the row confirms now, which is the one it was
    /// told under. A write that does not land costs a re-send on the next
    /// connection, which acts again on nothing (`Telegram` L2).
    pub fn the_roles_state_was_acknowledged(&mut self, enabled: bool) {
        let at = std::time::SystemTime::now();
        self.record.lend(&mut |connection| {
            let Ok(Some(row)) = crate::record::server_rows::one(connection, &self.server) else {
                return;
            };
            let Some(installation) = row.last_confirmed_installation_identity.as_deref() else {
                return;
            };
            let _ = crate::record::told_record::the_telling_was_acknowledged(
                connection,
                &self.server,
                &smoozy_telegram::told::WhatSmoozyTells::the_roles_desired_state(enabled),
                at,
                installation,
            );
        });
    }
}

/// [`TheMaterialFromTheRecord::it_is_on`], over one lent connection.
fn the_record_says_telegram_is_on(connection: &rusqlite::Connection) -> bool {
    use smoozy_telegram::proof::KeepsTheProvedBot;
    let switched_on = crate::record::settings::the_telegram_settings(connection)
        .map(|held| held.switched_on)
        .unwrap_or(false);
    if !switched_on {
        return false;
    }
    let keeper = crate::record::telegram_bot::TheProvedBotRecord::over(connection);
    match (keeper.the_proved_bot(), keeper.the_bot_whose_token_is_stored()) {
        (Ok(Some((bot, _))), Ok(Some(stored_for))) => bot.id == stored_for,
        _ => false,
    }
}

/// [`TheMaterialFromTheRecord::what_this_server_is_owed`], over one lent
/// connection. `None` where the record does not answer, which owes nothing.
fn the_owed_reading(connection: &rusqlite::Connection, server: &str) -> Option<WhatThisServerIsOwed> {
    use smoozy_telegram::proof::KeepsTheProvedBot;
    use smoozy_telegram::told::AToldThing;
    let row = crate::record::server_rows::one(connection, server).ok()??;
    let installation = row.last_confirmed_installation_identity?;
    let holds_the_material = crate::record::told_record::what_this_server_was_told(connection, server)
        .ok()?
        .iter()
        .any(|entry| entry.installation == installation);
    if !holds_the_material {
        return Some(WhatThisServerIsOwed::default());
    }
    let settings = crate::record::settings::the_telegram_settings(connection).ok()?;
    let conversation = crate::record::telegram_bot::TheProvedBotRecord::over(connection)
        .the_proved_bot()
        .ok()
        .flatten()
        .map(|(_, conversation)| conversation);
    let now = crate::surface::telegram::what_smoozy_tells_now(connection, &settings, conversation)?;
    let not_told = crate::record::told_record::the_servers_not_reached(connection, &now)
        .ok()?
        .into_iter()
        .find(|one| one.server == server)
        .map(|one| one.not_told)
        .unwrap_or_default();
    // **Off, the switch alone is owed and never the files.** The derivation
    // adds each server's own label to Settings' current values whatever the
    // switch says, so a server renamed while Telegram is off would otherwise be
    // sent a token it is to stop using.
    Some(WhatThisServerIsOwed {
        the_files: settings.switched_on
            && not_told.iter().any(|thing| {
                matches!(
                    thing,
                    AToldThing::TheTokenGeneration
                        | AToldThing::TheProvedConversation
                        | AToldThing::TheLabel
                        | AToldThing::TheReportingSettings
                )
            }),
        the_desired_state: not_told
            .contains(&AToldThing::TheRolesDesiredState)
            .then_some(settings.switched_on),
    })
}

impl ComposesTheTelegramMaterial for TheMaterialFromTheRecord<'_> {
    /// **Every reason it does not compose is one plain sentence the user reads**
    /// (Rule 9) — the Creator's eleventh notes, note 2, where the card showed
    /// the reason's internal name.
    fn composed(&mut self) -> Result<TheMaterial, String> {
        // **The sentences are the one closed list's** (the Creator's twelfth
        // notes, note 2), which is what the run panel reads them back by.
        use crate::record::told_record::the_composing_sentences as said;
        let Some(store) = self.store.as_deref() else {
            return Err(said::NOWHERE_TO_STORE_THE_TOKEN.to_string());
        };
        let mut composed = Err(said::THE_SETTINGS_COULD_NOT_BE_READ.to_string());
        self.record.lend(&mut |connection| {
            let row = match crate::record::server_rows::one(connection, &self.server) {
                Ok(Some(row)) => row,
                _ => {
                    composed = Err(said::NO_RECORD_OF_THIS_SERVER.to_string());
                    return;
                }
            };
            let idle = crate::record::settings::read(connection)
                .map(|held| {
                    held.the_switches
                        .attempts(crate::notifications::kinds::Kind::AMachineHasBeenIdle)
                })
                .unwrap_or(false);
            composed = crate::record::told_record::the_material_for(connection, store, &row, idle)
                // **The reason is said in words, and the token is nowhere in
                // them.** *Not there* has already forgotten the stored fact
                // (`Security and licence` §3.0 rule 4).
                .map_err(|why| why.in_words().to_string());
        });
        composed
    }

    fn the_placement_was_answered(
        &mut self,
        material: &TheMaterial,
        placed: &[(String, acts::HowTheActWent)],
    ) {
        let at = std::time::SystemTime::now();
        self.record.lend(&mut |connection| {
            // **`Telegram` L1's told entries move on the machine's own answer
            // and on nothing else.** A failure to write them is not a failure
            // of the placement: the files are on the machine either way, and
            // the told record catches up on the next delivery.
            let _ = crate::record::told_record::the_placement_was_answered(
                connection,
                &self.server,
                material,
                placed,
                at,
            );
        });
    }

    fn the_desired_state_was_acknowledged(&mut self) {
        self.the_roles_state_was_acknowledged(true);
    }

    fn the_role_is_enabled_there(&mut self) -> bool {
        let mut enabled = false;
        self.record.lend(&mut |connection| {
            enabled = the_role_as_told(connection, &self.server).unwrap_or(false);
        });
        enabled
    }

    fn the_disabled_state_was_acknowledged(&mut self) {
        self.the_roles_state_was_acknowledged(false);
    }
}

/// **The role's desired state as this server acknowledged it**, under the
/// installation it answers under now (`Telegram` L1's entries count only
/// under it). `None` where the record does not answer, which knows of no
/// running worker.
fn the_role_as_told(connection: &rusqlite::Connection, server: &str) -> Option<bool> {
    let row = crate::record::server_rows::one(connection, server).ok()??;
    let installation = row.last_confirmed_installation_identity?;
    let enabled = smoozy_telegram::told::WhatSmoozyTells::the_roles_desired_state(true);
    Some(
        crate::record::told_record::what_this_server_was_told(connection, server)
            .ok()?
            .iter()
            .any(|entry| {
                entry.installation == installation
                    && entry.thing == smoozy_telegram::told::AToldThing::TheRolesDesiredState
                    && entry.value == enabled.value()
            }),
    )
}

impl KeepsTheServersRow for TheServersRowHere<'_> {
    fn the_row(&mut self) -> Option<ServerRow> {
        let mut row = None;
        self.record.lend(&mut |connection| {
            row = crate::record::server_rows::one(connection, &self.server)
                .ok()
                .flatten();
        });
        row
    }

    fn record_the_root(&mut self, recorded: RecordedMachineRoot) -> Result<(), String> {
        self.one_write(&mut |row| {
            row.recorded_machine_root = Some(recorded.clone());
        })
    }

    /// **The identity and the cursor move in one durable write.**
    ///
    /// **The cursor is cleared rather than carried** — step 10c piece 7's owed
    /// item. `State, identity and recovery` §4 E2 keys reader state to the
    /// installation it was read under, and a new installation is a new journal
    /// starting at nought; a cursor kept from the last one would have the
    /// desktop skip every event before it and read another machine's work as
    /// this one's.
    fn record_the_installation_and_clear_the_cursor(
        &mut self,
        identity: &Identity,
    ) -> Result<(), String> {
        let written = identity.as_text();
        self.one_write(&mut |row| {
            row.last_confirmed_installation_identity = Some(written.clone());
            row.event_cursor = None;
        })
    }

    /// **`Servers` B5 item 1**, and it is unconditional: the bit is cleared
    /// whether the setup succeeds, partly succeeds or fails.
    fn clear_the_remembered_setup(&mut self) -> Result<(), String> {
        self.one_write(&mut |row| {
            row.remembered_setup = None;
        })
    }
}

impl TheServersRowHere<'_> {
    /// One durable write over the row, read back first. **A server another act
    /// removed is said as that and never as a refused write.**
    fn one_write(&mut self, change: &mut dyn FnMut(&mut ServerRow)) -> Result<(), String> {
        let server = self.server.clone();
        let mut said = Err("Could not write to Smoozy's database.".to_string());
        self.record.lend(&mut |connection| {
            match crate::record::server_rows::one(connection, &server) {
                Ok(Some(mut row)) => {
                    change(&mut row);
                    said = crate::record::server_rows::write(connection, &row)
                        .map_err(|why| format!("{why:?}"));
                }
                Ok(None) => said = Err("Server removed.".to_string()),
                Err(why) => said = Err(format!("{why:?}")),
            }
        });
        said
    }
}

/// **`Server protocol and transfers` B8d's read, over the one HTTPS client this
/// workspace has.** *(It was `Server protocol and transfers` B8b's latest
/// address until 2026-09-28.)*
///
/// **The root is this build's own and there is no second one**
/// (`Server protocol and transfers` B9): a build carrying nothing refuses every
/// release rather than trusting one, which is what a copy with no root should
/// do.
pub struct TheCurrentReleaseOverHttps<'a> {
    pub store: &'a mut dyn RemembersTheTrustRecordHere,
    pub from: &'a str,
}

/// The server-package trust record, as a seam this file can take by reference.
pub trait RemembersTheTrustRecordHere:
    smoozy_server_protocol::release::trust::RemembersTheServerPackageTrustRecord
{
}

impl<T> RemembersTheTrustRecordHere for T where
    T: smoozy_server_protocol::release::trust::RemembersTheServerPackageTrustRecord
{
}

/// **So the record can be taken by reference and still be what the admission
/// takes**, which is a value of its own.
impl smoozy_server_protocol::release::trust::RemembersTheServerPackageTrustRecord
    for &mut dyn RemembersTheTrustRecordHere
{
    fn read_it(
        &self,
    ) -> Result<Option<smoozy_server_protocol::release::trust::TheTrustRecord>, String> {
        (**self).read_it()
    }

    fn write_it(
        &mut self,
        record: &smoozy_server_protocol::release::trust::TheTrustRecord,
    ) -> Result<(), String> {
        (**self).write_it(record)
    }

    fn discard_it(&mut self) -> Result<(), String> {
        (**self).discard_it()
    }
}

impl ReadsTheCurrentRelease for TheCurrentReleaseOverHttps<'_> {
    fn the_current_release(
        &mut self,
        detected: &smoozy_server_protocol::release::manifest::TargetTuple,
    ) -> Result<(AVerifiedRelease, Vec<u8>), String> {
        let Some(root) =
            smoozy_server_protocol::release::trust::THE_RELEASE_ROOT_THIS_BUILD_CARRIES
        else {
            return Err(
                "This build of Smoozy has no release key.".to_string(),
            );
        };
        // **One fetch for both reads, kept rather than made per request** — it
        // holds a connection pool and the second read is the large one.
        let mut fetch = smoozy_server_protocol::release::fetching::AFetchOverHttps::
            reaching_the_internet(THE_MOST_THE_RELEASE_FILE_MAY_BE);
        // **`Server protocol and transfers` B8d: this build's own release and
        // no other**, which is what the real reader asks for.
        this_builds_own_release(
            &root,
            &mut fetch,
            self.store,
            detected,
            self.from,
            env!("CARGO_PKG_VERSION"),
        )
    }
}

// ════════════════════════════════════════════════════════════════════════════
// What the running application reaches
// ════════════════════════════════════════════════════════════════════════════

/// **`Server Setup` Y9: one setup run per server at a time, and `Server Setup`
/// Y8's stop, both held for the life of the run.**
///
/// **Run-held and never durable** (`Server Setup` S2 and `Server Setup` Y6a:
/// the desktop
/// stores nothing about a machine's setup). What is here is which servers this
/// copy is driving a run on now, and which of those the user has asked to stop.
fn the_runs_this_copy_is_driving() -> &'static std::sync::Mutex<TheRunsHeld> {
    static HELD: std::sync::OnceLock<std::sync::Mutex<TheRunsHeld>> = std::sync::OnceLock::new();
    HELD.get_or_init(|| std::sync::Mutex::new(TheRunsHeld::default()))
}

#[derive(Default)]
struct TheRunsHeld {
    /// **Which server a run is being driven on, and of which kind.** The kind
    /// is here because the screen has to name what it is stopping: step 10c's
    /// fifth pass on Vast watched *Setup stopped.* answer a **Blender
    /// install**'s own *Stop*.
    driving: Vec<(String, SetupKind)>,
    asked_to_stop: Vec<String>,
    /// **The run each server's driving last told the screen**, read off the
    /// machine with the step under way marked — `Design/Server Setup § A long
    /// step`. **Run-held like the rest of this, and let go when the run ends**:
    /// it is what the panel of a run still going is drawn from, and nothing
    /// else. Added 2026-09-27, fix piece 9.
    under_way: Vec<(String, TheSetupAsRead)>,
    /// **Which runs began after `Server Setup` W3's warning named a render
    /// running on that server** — the one reading `Design/Server Setup § A long
    /// step`'s render sentence is drawn on (`Server Setup` T6a, T7). **Carried
    /// from the warning the user was shown and never read again from the
    /// machine**; where no warning was shown, nothing is carried. Added
    /// 2026-09-27, the eighth-pass fix for W3's warning.
    a_render_was_running: Vec<String>,
    /// **The acts still waiting behind the run being driven on each server**,
    /// in the order they were asked for — `Server Setup` Y9a's line. **Held
    /// here because the next one is begun under the same lock that lets the
    /// last one go**, so no other run slips between two acts (`Server Setup`
    /// Y9) and a stop pressed between them is not lost. Run-held; the local
    /// record keeps a copy only so a closed Smoozy can name them. Added
    /// 2026-10-08 for installs picked together (the Creator's third notes,
    /// piece H); every card act from 2026-10-27, the nineteenth notes, note 2.
    waiting: Vec<(String, Vec<ASetUpAct>)>,
    /// **The acts a run left waiting when it ended without moving the line**
    /// — `Server Setup` Y9a item 8: `Server Setup` R1b's swap run lets its exclusion go by
    /// [`a_run_ends`], and what joined during it is not begun. Held until its
    /// ending names them ([`the_line_stopped_on`]) or the next run begins.
    /// Added 2026-10-27, the nineteenth notes, note 2.
    the_line_stopped: Vec<(String, Vec<ASetUpAct>)>,
    /// **`Server Setup` Y7b and `Server Setup` Y7c: what the Set up card says of the build a
    /// run downloads and unpacks** — the build as it was asked for, and how
    /// far the download or the unpack has got as the run was last read.
    /// Run-held like the rest, and let go when the run ends or the next one
    /// begins. Added 2026-10-18, the Creator's fourteenth notes, piece U.
    the_builds_figures: Vec<(String, TheBuildsFigures)>,
    /// **The run each server's driving last told the screen, kept from the
    /// run's end until the next run begins there** — so the card can keep the
    /// steps as the run left them, its last step marked as the run's own last
    /// reading marks it, while the card's own reading is taken. Added
    /// 2026-10-26, the worldwide pass's setup faults, fault 4: the card kept
    /// the last panel it had drawn, and the run's last reading could end the
    /// run before the card asked for it.
    ended: Vec<(String, SetupKind, TheSetupAsRead)>,
}

/// **What the Set up card says of the build a run downloads and unpacks.**
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TheBuildsFigures {
    /// **The build as the run asked for it**, which the card names as the list
    /// named it. Nothing on a run that downloads no build.
    pub the_build: Option<String>,
    /// **How far the download or the unpack has got**, while one runs.
    pub how_far: Option<HowFarTheStepHasGot>,
}

/// **`Server Setup` Y9's exclusion, taken.** Nothing where a run already
/// stands on that server — **whatever kind either run is**, which is that
/// clause's own rule: one setup runs on a server at a time.
pub fn a_run_begins(server: &str, kind: SetupKind) -> bool {
    if !a_run_nobody_pressed_for_begins(server, kind) {
        return false;
    }
    // **`Server Setup` Y7a: the act the user asked for lets the last act's
    // words go.**
    a_runs_words_begin(server);
    true
}

/// **[`a_run_begins`]'s exclusion, taken for a run nobody pressed for** —
/// `Server Setup` R1b's swap — **and the words kept as they are** (`Server
/// Setup` Y7a as changed 2026-10-30, the twenty-third notes, note 3).
pub fn a_run_nobody_pressed_for_begins(server: &str, kind: SetupKind) -> bool {
    let Ok(mut held) = the_runs_this_copy_is_driving().lock() else {
        return false;
    };
    if held.driving.iter().any(|(one, _)| one == server) {
        return false;
    }
    a_run_is_begun_in(&mut held, server, kind);
    true
}

/// **The run's place taken, under the lock the caller holds**, and whatever
/// the last run on that server left let go.
fn a_run_is_begun_in(held: &mut TheRunsHeld, server: &str, kind: SetupKind) {
    held.driving.push((server.to_string(), kind));
    held.ended.retain(|(one, ..)| one != server);
    held.asked_to_stop.retain(|one| one != server);
    held.a_render_was_running.retain(|one| one != server);
    held.the_builds_figures.retain(|(one, _)| one != server);
    held.the_line_stopped.retain(|(one, _)| one != server);
}

/// **An add's setup, held from the login to step 8's hand-over** — the very
/// memory the transport keeps (`crate::servers::a_period`'s
/// `TheTransportThroughThePoint`), since `Servers` B5 item 1 clears the row's
/// bit at the login. Added 2026-10-07, the Creator's fourth note 1, piece M.
pub type TheSetupHeldForTheHandOver = std::sync::Arc<std::sync::Mutex<Option<Vec<String>>>>;

/// **Which server's transport holds an add's setup not yet handed over.**
///
/// **Each entry only points at the transport's own memory**, so it can never
/// say more than that memory does: the setup waits while the memory still
/// holds it, and stops waiting the moment step 8 takes it, a later login of
/// the same period carries none, or the period ends and the transport with it.
/// Run-held and never durable.
///
/// **An entry stays after its memory has gone, until step 8 or the server's
/// next login takes it out** — so an entry whose memory has gone is an add's
/// setup the login carried and step 8 never received: the period ended at
/// `Servers` Q1 steps 4 to 6 (piece M2,
/// [`an_adds_setup_never_reached_step_8_on`]). One entry per server, so it
/// stays small.
fn the_setups_waiting_to_be_handed_over(
) -> &'static std::sync::Mutex<Vec<(String, std::sync::Weak<std::sync::Mutex<Option<Vec<String>>>>)>> {
    static HELD: std::sync::OnceLock<
        std::sync::Mutex<Vec<(String, std::sync::Weak<std::sync::Mutex<Option<Vec<String>>>>)>>,
    > = std::sync::OnceLock::new();
    HELD.get_or_init(|| std::sync::Mutex::new(Vec::new()))
}

/// **The login carried an add's setup, and this transport holds it** until
/// step 8 — so the Set up tab's card keeps reading *Connecting…*
/// (`Design/Server Setup § This server`: *"Then the run's steps take the
/// card."*).
pub fn a_setup_waits_to_be_handed_over(server: &str, held: &TheSetupHeldForTheHandOver) {
    if let Ok(mut waiting) = the_setups_waiting_to_be_handed_over().lock() {
        waiting.retain(|(one, _)| one != server);
        waiting.push((server.to_string(), std::sync::Arc::downgrade(held)));
    }
}

/// **A login that carried no setup**: an earlier add's entry for that server
/// whose memory has gone is taken out, since this login is the next act on
/// the server. **An entry whose transport still stands is left**, so piece M's
/// *Connecting…* is not ended by anything but its own transport. Added
/// 2026-10-08, piece M2.
pub fn a_login_carried_no_setup(server: &str) {
    if let Ok(mut waiting) = the_setups_waiting_to_be_handed_over().lock() {
        waiting.retain(|(one, memory)| one != server || memory.strong_count() > 0);
    }
}

/// **Step 8 has run on this transport**, so its entry is taken out: an add's
/// setup that reached step 8 is not one that failed the first checks. Added
/// 2026-10-08, piece M2.
pub fn the_setup_reached_step_8(held: &TheSetupHeldForTheHandOver) {
    if let Ok(mut waiting) = the_setups_waiting_to_be_handed_over().lock() {
        let this = std::sync::Arc::downgrade(held);
        waiting.retain(|(_, memory)| !memory.ptr_eq(&this));
    }
}

/// **Whether an add's setup on that server was carried at the login and never
/// reached step 8** — the transport that held it is gone without handing it
/// over, so the period ended at `Servers` Q1 steps 4 to 6. **`Servers` B5
/// cleared the row's bit at that login**, so the row can no longer say an add
/// asked for a setup, and this is what lets the Set up tab's card say why the
/// connection stopped instead of *Smoozy has not read this server yet.* Added
/// 2026-10-08, the Creator's fourth note 1, piece M2.
pub fn an_adds_setup_never_reached_step_8_on(server: &str) -> bool {
    let Ok(waiting) = the_setups_waiting_to_be_handed_over().lock() else {
        return false;
    };
    waiting
        .iter()
        .any(|(one, memory)| one == server && memory.strong_count() == 0)
}

/// **Whether an add's setup on that server is still waiting to be handed
/// over** — read from the transport's own memory, never from the record.
pub fn an_adds_setup_waits_on(server: &str) -> bool {
    let Ok(waiting) = the_setups_waiting_to_be_handed_over().lock() else {
        return false;
    };
    waiting
        .iter()
        .filter(|(one, _)| one == server)
        .filter_map(|(_, memory)| memory.upgrade())
        .any(|memory| memory.lock().is_ok_and(|held| held.is_some()))
}

/// **The run just begun on this server began after `Server Setup` W3's warning
/// named a render running there.** Added 2026-09-27.
pub fn the_run_began_with_a_render_running(server: &str) {
    if let Ok(mut held) = the_runs_this_copy_is_driving().lock() {
        if held.driving.iter().any(|(one, _)| one == server)
            && !held.a_render_was_running.iter().any(|one| one == server)
        {
            held.a_render_was_running.push(server.to_string());
        }
    }
}

/// **Whether the run this copy is driving on that server began with a render
/// running there**, as `Server Setup` W3's warning named it — and false where
/// no run is being driven or no warning named one.
pub fn a_render_was_running_when_the_run_began(server: &str) -> bool {
    the_runs_this_copy_is_driving()
        .lock()
        .map(|held| held.a_render_was_running.iter().any(|one| one == server))
        .unwrap_or(false)
}

/// **Which kind of run this copy is driving on that server**, or nothing where
/// it is driving none. **Run-held, like the exclusion itself**, and read by the
/// screen so that what it says about a run names the run it is about.
pub fn the_kind_being_driven_on(server: &str) -> Option<SetupKind> {
    the_runs_this_copy_is_driving().lock().ok().and_then(|held| {
        held.driving
            .iter()
            .find(|(one, _)| one == server)
            .map(|(_, kind)| *kind)
    })
}

/// The exclusion, given back — **whether the run reached *done*, stopped or
/// failed**.
pub fn a_run_ends(server: &str) {
    if let Ok(mut held) = the_runs_this_copy_is_driving().lock() {
        // **The run as it was last told, kept for the card** (2026-10-26,
        // fault 4).
        let kind = held.driving.iter().find(|(one, _)| one == server).map(|(_, kind)| *kind);
        let last = held.under_way.iter().find(|(one, _)| one == server).map(|(_, read)| read.clone());
        held.ended.retain(|(one, ..)| one != server);
        if let (Some(kind), Some(last)) = (kind, last) {
            held.ended.push((server.to_string(), kind, last));
        }
        held.driving.retain(|(one, _)| one != server);
        held.asked_to_stop.retain(|one| one != server);
        held.under_way.retain(|(one, _)| one != server);
        held.a_render_was_running.retain(|one| one != server);
        // **`Server Setup` Y9a item 8: a run that does not move the line
        // stops it**, and what was waiting is kept for its ending to name.
        if let Some(at) = held.waiting.iter().position(|(one, _)| one == server) {
            let (_, left) = held.waiting.remove(at);
            held.the_line_stopped.retain(|(one, _)| one != server);
            if !left.is_empty() {
                held.the_line_stopped.push((server.to_string(), left));
            }
        }
        held.the_builds_figures.retain(|(one, _)| one != server);
    }
    // **An act of Smoozy's own ending on this machine is the one event that
    // says what Blenders it holds may have changed**, so the Set up tab's held
    // search is let go and the next opening takes it again
    // (`Server Setup` B12).
    crate::surface::set_up::the_search_is_taken_again_on(server);
    // **`Servers` Q1: step 9 follows whatever step 8 did**, so a step 9 this
    // run held back is taken now (the twenty-fourth notes, note 4).
    if let Some(then) = WHEN_A_RUN_ENDS.get() {
        then(server);
    }
}

/// **What the app does when a run is let go**, registered once at start
/// ([`crate::servers::the_machines_work::a_held_back_step_nine_follows_every_run`]);
/// nothing where none was registered. Added 2026-10-31, the twenty-fourth
/// notes, note 4.
static WHEN_A_RUN_ENDS: std::sync::OnceLock<Box<dyn Fn(&str) + Send + Sync>> = std::sync::OnceLock::new();

/// **The one hook a run's ending calls**, set once.
pub fn when_a_run_ends(then: Box<dyn Fn(&str) + Send + Sync>) {
    let _ = WHEN_A_RUN_ENDS.set(then);
}

/// **The release a ready action's placement was handed, noted as it passes**,
/// so a run that reaches *done* can say what the machine now runs. **Nothing
/// is noted on a machine this copy already set up**, where the placement
/// replaces nothing and the reader is never asked (`Server Setup` Y6b). Added
/// 2026-10-31, the twenty-fourth notes, note 4.
pub struct TheReleaseThisRunPlaces<'a> {
    pub reads: &'a mut dyn ReadsTheCurrentRelease,
    pub placed: Option<u64>,
}

impl ReadsTheCurrentRelease for TheReleaseThisRunPlaces<'_> {
    fn the_current_release(
        &mut self,
        detected: &smoozy_server_protocol::release::manifest::TargetTuple,
    ) -> Result<(AVerifiedRelease, Vec<u8>), String> {
        let got = self.reads.the_current_release(detected);
        if let Ok((verified, _)) = &got {
            self.placed = Some(verified.manifest.release_sequence);
        }
        got
    }
}

/// **A ready action that placed a release and reached *done*: the machine now
/// runs that release**, so the queue's `Telegram` V1c control does not wait for
/// step 9's reading, which a setup driving the server holds back on this
/// connection. **The *done* ending only.** True where it was kept. Added
/// 2026-10-31, the twenty-fourth notes, note 4.
pub fn the_release_a_finished_setup_placed_is_kept(
    server: &str,
    came_to: &WhereTheSetUpGot,
    placed: Option<u64>,
) -> bool {
    let done = matches!(
        came_to,
        WhereTheSetUpGot::TheRunEnded { ending: TheRunsEnding::ItReachedDone { .. }, .. }
    );
    match placed {
        Some(sequence) if done => {
            crate::servers::published::the_servers_release_is_read(server, Some(sequence));
            true
        }
        _ => false,
    }
}

/// **The Blenders picked after the first, waiting their turn on that server**
/// — the first is the run [`a_run_begins`] just took. In the order the list
/// showed them. Added 2026-10-08, the Creator's third notes, piece H; each is
/// a [`ASetUpAct::AnInstallFromTheList`] of the line from 2026-10-27.
pub fn the_installs_wait_on(server: &str, waiting: Vec<String>) {
    if let Ok(mut held) = the_runs_this_copy_is_driving().lock() {
        held.waiting.retain(|(one, _)| one != server);
        if !waiting.is_empty() {
            held.waiting
                .push((server.to_string(), waiting.into_iter().map(ASetUpAct::AnInstallFromTheList).collect()));
        }
    }
}

/// **The Blenders from the list still waiting their turn on that server**, in
/// the order asked — the installs among [`the_acts_still_waiting_on`]. Empty
/// where none wait. Added 2026-10-16, step 10c's owed piece 4.
pub fn the_installs_still_waiting_on(server: &str) -> Vec<String> {
    the_acts_still_waiting_on(server)
        .into_iter()
        .filter_map(|act| match act {
            ASetUpAct::AnInstallFromTheList(version) => Some(version),
            _ => None,
        })
        .collect()
}

/// **The acts still waiting their turn on that server**, in the order they
/// were asked for — `Server Setup` Y9a's line, which the Set up card lists
/// under the run in hand. Empty where none wait. Added 2026-10-27, the
/// nineteenth notes, note 2.
pub fn the_acts_still_waiting_on(server: &str) -> Vec<ASetUpAct> {
    the_runs_this_copy_is_driving()
        .lock()
        .ok()
        .and_then(|held| {
            held.waiting
                .iter()
                .find(|(one, _)| one == server)
                .map(|(_, waiting)| waiting.clone())
        })
        .unwrap_or_default()
}

/// **The acts a run left waiting when it ended without moving the line**,
/// taken once for its ending to name (`Server Setup` Y9a item 8). Empty where
/// none were. Added 2026-10-27.
pub fn the_line_stopped_on(server: &str) -> Vec<ASetUpAct> {
    the_runs_this_copy_is_driving()
        .lock()
        .ok()
        .and_then(|mut held| {
            let at = held.the_line_stopped.iter().position(|(one, _)| one == server)?;
            Some(held.the_line_stopped.remove(at).1)
        })
        .unwrap_or_default()
}

/// **A run pressed on the Set up card is taken, and the installs still
/// waiting on that server are kept as this press leaves them** — `Server
/// Setup` Y9a. Added 2026-10-16, step 10c's owed piece 4.
///
/// **`keep` is the local record's write**, handed the picks after the first
/// for a Blender install from the list and nothing for any other run: *Set up
/// again*, *Set up this server* and a build of the user's own each replace
/// what an earlier run left, which is what ends the card's line about
/// installs Smoozy closed before it began. **Nothing is kept where the run
/// could not be taken** (`Server Setup` Y9): no press began, so what stood
/// still stands. **The card's own presses go through
/// [`the_acts_are_taken_from_the_card`] from 2026-10-27**; this stays for the
/// add's setup, which never joins a line.
pub fn a_run_is_taken_from_the_card(
    server: &str,
    kind: SetupKind,
    waiting: Vec<String>,
    keep: &mut dyn FnMut(&[String]),
) -> bool {
    if !a_run_begins(server, kind) {
        return false;
    }
    keep(&waiting);
    if kind == SetupKind::ABlenderInstall {
        the_installs_wait_on(server, waiting);
    }
    true
}

/// **One act on the Set up card, as `Server Setup` Y9a's line holds it** —
/// what is carried from the press to the moment its run begins. Added
/// 2026-10-27, the nineteenth notes, note 2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ASetUpAct {
    /// **A Blender from the list**, by the version as the list carried it.
    AnInstallFromTheList(String),
    /// **The user's own build** (`Server Setup` B7), by the file picked at the
    /// press.
    TheUsersBuild(std::path::PathBuf),
    /// **A removal** (`Server Setup` B15), with `Server Setup` B14's covered
    /// set composed when the warning was shown (`Server Setup` Y9a item 9).
    ARemoval(TheRemovalAsked),
    /// **The ready action — *Set up again***, and whether `Server Setup` W3's
    /// warning, shown at the press, named a render running (`Server Setup` T6a).
    TheReadyAction { a_render_was_running: bool },
    /// **The Telegram placement** (`Server Setup` G2).
    TheTelegramPlacement,
}

impl ASetUpAct {
    /// **The run's own kind** (`Server Setup` Y6b).
    pub fn kind(&self) -> SetupKind {
        match self {
            ASetUpAct::AnInstallFromTheList(_) | ASetUpAct::TheUsersBuild(_) => SetupKind::ABlenderInstall,
            ASetUpAct::ARemoval(_) => SetupKind::ABlenderRemoval,
            ASetUpAct::TheReadyAction { .. } => SetupKind::TheReadyAction,
            ASetUpAct::TheTelegramPlacement => SetupKind::TheTelegramPlacement,
        }
    }

    /// **As the local record keeps it** — `Server Setup` Y9a item 10: its
    /// kind and what it names.
    pub fn as_kept(&self) -> crate::record::installs_not_begun::AnActKept {
        use crate::record::installs_not_begun::{AnActKept, TheActsKind};
        let (kind, named) = match self {
            ASetUpAct::AnInstallFromTheList(version) => (TheActsKind::AnInstallFromTheList, version.clone()),
            ASetUpAct::TheUsersBuild(file) => (
                TheActsKind::TheUsersBuild,
                file.file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_default(),
            ),
            ASetUpAct::ARemoval(asked) => (
                TheActsKind::ARemoval,
                if asked.the_answered_version.is_empty() {
                    asked.executable.clone()
                } else {
                    asked.the_answered_version.clone()
                },
            ),
            ASetUpAct::TheReadyAction { .. } => (TheActsKind::TheReadyAction, String::new()),
            ASetUpAct::TheTelegramPlacement => (TheActsKind::TheTelegramPlacement, String::new()),
        };
        AnActKept { kind, named }
    }
}

/// **How the card's press was taken.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HowThePressWasTaken {
    /// **Its first act began now**, and any after it wait behind it.
    ItBegan,
    /// **A run already stood on that server**, so every act pressed joined the
    /// end of its line.
    ItWaitsItsTurn,
}

/// **`Server Setup` Y9a items 6 and 10: the acts one press on the Set up card
/// asks for, taken** — the first begun at once where no run stands on that
/// server, the rest behind it; **and every one of them appended to the line
/// where a run already stands**, rather than refused. Under one lock, so a run
/// ending between the look and the act cannot strand a press. Added
/// 2026-10-27, the nineteenth notes, note 2.
///
/// **`keep` is the local record's write, handed the whole line as this press
/// leaves it** — the acts behind the first where it began (which replaces what
/// an earlier Smoozy closed before beginning), and the line with these
/// appended where they joined it. Nothing is asked of `keep` for an empty
/// press.
pub fn the_acts_are_taken_from_the_card(
    server: &str,
    acts: Vec<ASetUpAct>,
    keep: &mut dyn FnMut(&[ASetUpAct]),
) -> HowThePressWasTaken {
    let Some(first) = acts.first().cloned() else {
        return HowThePressWasTaken::ItWaitsItsTurn;
    };
    let Ok(mut held) = the_runs_this_copy_is_driving().lock() else {
        return HowThePressWasTaken::ItWaitsItsTurn;
    };
    let (taken, line) = if held.driving.iter().any(|(one, _)| one == server) {
        let line = match held.waiting.iter_mut().find(|(one, _)| one == server) {
            Some((_, line)) => {
                line.extend(acts);
                line.clone()
            }
            None => {
                held.waiting.push((server.to_string(), acts.clone()));
                acts
            }
        };
        (HowThePressWasTaken::ItWaitsItsTurn, line)
    } else {
        a_run_is_begun_in(&mut held, server, first.kind());
        let rest = acts[1..].to_vec();
        held.waiting.retain(|(one, _)| one != server);
        if !rest.is_empty() {
            held.waiting.push((server.to_string(), rest.clone()));
        }
        (HowThePressWasTaken::ItBegan, rest)
    };
    drop(held);
    if taken == HowThePressWasTaken::ItBegan {
        // **`Server Setup` Y7a: the next run lets the last run's words go.**
        a_runs_words_begin(server);
    }
    keep(&line);
    taken
}

/// **What follows a run in the line on that server.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatFollowsARun {
    /// **The next act, begun** — the run's exclusion was never let go, so it
    /// is already this act's.
    TheNextBegins(ASetUpAct),
    /// Nothing was waiting, and the exclusion is let go.
    NothingWaits,
    /// **The acts still waiting, not begun** — the run did not finish, the
    /// user asked to stop, or it restarted the server. The exclusion is let go
    /// and nothing more is begun (`Server Setup` Y9a item 7).
    TheseWereNotBegun(Vec<ASetUpAct>),
}

/// **A run in the line ends, and the next act begins only where this one
/// finished and nobody asked to stop** — `Server Setup` Y9a item 7. Added
/// 2026-10-08 for installs (the Creator's third notes, piece H); every card
/// act from 2026-10-27.
///
/// **`it_finished` is the caller's reading of item 7**: *done*, its Blender
/// published for an install from the list, and no restart issued
/// ([`the_run_restarted_the_server`]). **Every other ending stops the rest**,
/// and so does the user's own stop, which asked for the setup to stop and not
/// for the next act to start. **Nothing on the machine changes**: each act is
/// its own run of `Server Setup` Y6b's kind, begun here as a press would begin
/// it, and `Server Setup` Y9's one run at a time holds because the exclusion
/// passes from one to the next under one lock.
pub fn a_run_in_the_line_ends(server: &str, it_finished: bool) -> WhatFollowsARun {
    let Ok(mut held) = the_runs_this_copy_is_driving().lock() else {
        return WhatFollowsARun::NothingWaits;
    };
    let mut waiting = held
        .waiting
        .iter()
        .position(|(one, _)| one == server)
        .map(|at| held.waiting.remove(at).1)
        .unwrap_or_default();
    let stopped = held.asked_to_stop.iter().any(|one| one == server);
    if !it_finished || stopped || waiting.is_empty() {
        drop(held);
        a_run_ends(server);
        return if waiting.is_empty() {
            WhatFollowsARun::NothingWaits
        } else {
            WhatFollowsARun::TheseWereNotBegun(waiting)
        };
    }
    let next = waiting.remove(0);
    if !waiting.is_empty() {
        held.waiting.push((server.to_string(), waiting));
    }
    // **The next act's run is of its own kind**, and the last run's reading
    // goes with it, so the card draws the new run from its own steps and not
    // the last one's.
    if let Some((_, kind)) = held.driving.iter_mut().find(|(one, _)| one == server) {
        *kind = next.kind();
    }
    held.under_way.retain(|(one, _)| one != server);
    held.a_render_was_running.retain(|one| one != server);
    held.the_builds_figures.retain(|(one, _)| one != server);
    drop(held);
    // **`Server Setup` Y7a: acts in one line keep adding to one record**,
    // each act's words under its own steps — nobody pressed anything now, so
    // nothing is emptied (changed 2026-10-30, the twenty-third notes, note 3).
    // **The Blender table is read again between two acts**, as at any run's
    // ending (`Server Setup` B12).
    crate::surface::set_up::the_search_is_taken_again_on(server);
    WhatFollowsARun::TheNextBegins(next)
}

/// **Whether the run being driven on that server issued a restart** — `Server
/// Setup` Y9a item 7: *restart issued* among the boundaries its driving last
/// read. Asked before [`a_run_in_the_line_ends`], which lets that reading go.
/// Added 2026-10-27, the nineteenth notes, note 2.
pub fn the_run_restarted_the_server(server: &str) -> bool {
    the_run_under_way_on(server).is_some_and(|read| {
        read.record
            .the_boundaries_reached
            .iter()
            .any(|reached| reached.boundary == Boundary::RestartIssued)
    })
}

/// **The run this copy is driving on that server, as its driving last read it,
/// with the step under way marked** — or nothing where no run is being driven.
/// Added 2026-09-27, fix piece 9.
pub fn the_run_under_way_on(server: &str) -> Option<TheSetupAsRead> {
    let held = the_runs_this_copy_is_driving().lock().ok()?;
    if !held.driving.iter().any(|(one, _)| one == server) {
        return None;
    }
    held.under_way
        .iter()
        .find(|(one, _)| one == server)
        .map(|(_, read)| read.clone())
}

/// **The run as its driving last told it, and its kind, once it has ended** —
/// until the next run begins on that server; nothing while a run is driven
/// there, or where none was. Added 2026-10-26, the worldwide pass's setup
/// faults, fault 4.
pub fn the_run_as_it_ended_on(server: &str) -> Option<(SetupKind, TheSetupAsRead)> {
    let held = the_runs_this_copy_is_driving().lock().ok()?;
    if held.driving.iter().any(|(one, _)| one == server) {
        return None;
    }
    held.ended
        .iter()
        .find(|(one, ..)| one == server)
        .map(|(_, kind, read)| (*kind, read.clone()))
}

/// **The run's latest reading, kept while the run is driven** — and nothing
/// kept where no run is being driven on that server, so a reading arriving
/// after the run ended is not held against the next one. Public from
/// 2026-10-18 (piece S) so the desk keeps a run's reading the way the Set up
/// tab does.
pub fn the_run_is_read_under_way(server: &str, read: &TheSetupAsRead) -> bool {
    let Ok(mut held) = the_runs_this_copy_is_driving().lock() else {
        return false;
    };
    if !held.driving.iter().any(|(one, _)| one == server) {
        return false;
    }
    held.under_way.retain(|(one, _)| one != server);
    held.under_way.push((server.to_string(), read.clone()));
    true
}

/// **`Server Setup` Y7b and `Server Setup` Y7c: where the figures of the run begun on
/// `server` go for the Set up card** — the build it downloads kept now, and
/// each figure kept as it comes with `told` called where it moved, which is
/// the tab being told. **Nothing is kept where no run is being driven on that
/// server**, so a figure arriving after the run ended is not held against the
/// next one. The Set up tab's own route ([`TheSetUpTabAsks`]), written apart so
/// the desk drives the same keeping without a window. Added 2026-10-18, the
/// Creator's fourteenth notes, piece U.
pub fn the_figure_goes_to_the_card(
    server: &str,
    the_build: Option<&str>,
    told: impl Fn() + Send + Sync + 'static,
) -> TheFigureGoesHere {
    if let Ok(mut held) = the_runs_this_copy_is_driving().lock() {
        held.the_builds_figures.retain(|(one, _)| one != server);
        if held.driving.iter().any(|(one, _)| one == server) {
            held.the_builds_figures.push((
                server.to_string(),
                TheBuildsFigures {
                    the_build: the_build.map(str::to_string),
                    how_far: None,
                },
            ));
        }
    }
    let server = server.to_string();
    std::sync::Arc::new(move |how_far: Option<HowFarTheStepHasGot>| {
        let moved = the_runs_this_copy_is_driving().lock().is_ok_and(|mut held| {
            match held.the_builds_figures.iter_mut().find(|(one, _)| *one == server) {
                Some((_, kept)) if kept.how_far != how_far => {
                    kept.how_far = how_far;
                    true
                }
                _ => false,
            }
        });
        if moved {
            told();
        }
    })
}

/// **What the Set up card says of the build the run on `server` downloads and
/// unpacks** — nothing where no run is being driven there.
pub fn the_builds_figures_on(server: &str) -> Option<TheBuildsFigures> {
    let held = the_runs_this_copy_is_driving().lock().ok()?;
    if !held.driving.iter().any(|(one, _)| one == server) {
        return None;
    }
    held.the_builds_figures
        .iter()
        .find(|(one, _)| one == server)
        .map(|(_, kept)| kept.clone())
}

// ── `Server Setup` Y7a: what a run printed, kept on the desktop ─────────────
//
// Added 2026-10-18, the Creator's thirteenth notes, piece H. **The keeping is
// `Server Setup`'s** (`elevated::WhatTheRunPrinted`); what is here is where it
// is held — process memory, one run per server, from the run being taken until
// the next run on that server or until Smoozy closes — and M13b's second
// narrowing: the newest three complete lines of the ordinary stream the run
// has printed, kept until another step prints or the run ends, which is all
// the Set up card's window shows (changed 2026-10-18, piece S: until then the
// lines were the step under way's alone, and went when it ended).

/// **Where what a run's commands print goes** — the step it belongs to and
/// what it printed, handed over the moment it arrives
/// ([`AsksForThePassword::where_the_words_go`]): an elevated step's and a
/// reading's as they print, a worker step's once, at the reading that shows the
/// step ended (`Server Setup` Y7a items 1 and 2).
pub type TheWordsGoHere = std::sync::Arc<dyn Fn(Step, &WhatTheServerPrinted) + Send + Sync>;

/// **What reaches `Server Setup` Y7a's keeping for one step.** Added
/// 2026-10-24, the Creator's ruling on the sixteenth notes, note 1.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheServerPrinted {
    /// **A piece one of the step's commands printed**, on the stream it came
    /// on.
    APiece(APiecePrinted),
    /// **`Server Setup` Y7a item 5: some of what a worker step printed could not be brought
    /// back from the server**, and the machine carried the mark in its place.
    /// No words come with it.
    SomeOfItCouldNotBeBroughtBack,
}

/// **How many lines the Set up card's window shows** — `Design/Server Setup §
/// A long step`: *"the newest three lines the run's steps have printed, from
/// either stream, in the order they arrived"* (from 2026-10-24; it was the
/// ordinary stream's alone).
pub const THE_WINDOWS_LINES: usize = 3;

/// **The most of a line not yet ended that is kept** since its last carriage
/// return. Past this the line is far wider than the window, which cuts it at
/// its start anyway; the figure is the queue's own for its entry's lines.
const A_LINE_IS_KEPT_UP_TO: usize = 4096;

/// **One stream's line being printed and not ended yet**, as a terminal holds
/// it: what the line shows up to its last carriage return, and what was
/// printed since, which writes over it from its start
/// ([`elevated::as_a_terminal_shows_it`]). Added 2026-10-25, the eighteenth
/// notes, piece 2c.
#[derive(Debug, Clone)]
struct TheLineBeingPrinted {
    /// The step printing it. **Another step's piece on the same stream lets
    /// it go**, so two steps' words are never joined into one line.
    step: Step,
    /// What the line shows as of its last carriage return.
    shown: Vec<char>,
    /// What was printed since that carriage return, not yet read as text.
    since: Vec<u8>,
    /// **Its place among the window's lines**: lines are shown in the order
    /// they began to arrive.
    began: u64,
}

impl TheLineBeingPrinted {
    /// **The line as a terminal shows it now** — what was printed since the
    /// last carriage return written over what it showed, character by
    /// character. A character whose bytes have not all arrived is left off
    /// until they have.
    fn as_shown(&self) -> String {
        let mut line = self.shown.clone();
        for (at, character) in the_text_so_far(&self.since).chars().enumerate() {
            if at < line.len() {
                line[at] = character;
            } else {
                line.push(character);
            }
        }
        line.into_iter().collect()
    }
}

/// **The bytes as text, leaving off the first bytes of a character still on
/// its way**; anything that is not text at all is read with the replacement
/// mark, the one way a page's text can carry it.
fn the_text_so_far(bytes: &[u8]) -> std::borrow::Cow<'_, str> {
    let mut whole = bytes.len();
    for back in 1..=3.min(bytes.len()) {
        let byte = bytes[bytes.len() - back];
        if byte & 0b1100_0000 == 0b1000_0000 {
            continue;
        }
        let its_length = match byte {
            0xC0..=0xDF => 2,
            0xE0..=0xEF => 3,
            0xF0..=0xF7 => 4,
            _ => 1,
        };
        if its_length > back {
            whole = bytes.len() - back;
        }
        break;
    }
    String::from_utf8_lossy(&bytes[..whole])
}

/// **One run's words on one server, and the window's lines.** Run-held, and
/// never written anywhere.
#[derive(Debug, Clone, Default)]
pub struct TheRunsWords {
    /// `Server Setup` Y7e: held apart from the server's streams.
    failure_reason: Option<String>,
    /// **`Server Setup` Y7a's keeping, apart by stream and by step**, every
    /// byte as printed (M9).
    printed: elevated::WhatTheRunPrinted,
    /// **Each stream's line being printed**, the ordinary stream's first.
    /// Changed 2026-10-25, the eighteenth notes, piece 2c: there was one,
    /// for the ordinary stream alone, and it was never shown.
    being_printed: [Option<TheLineBeingPrinted>; 2],
    /// **The newest complete lines of either stream, oldest first, each with
    /// the stream it came from** — `Desktop app and delivery` M13b's second
    /// narrowing as changed on 2026-10-24: *"each line drawn so that which
    /// stream it came from is never lost"*. **The complete lines stay** until
    /// newer ones push them out or the run ends (the fourteenth notes, piece
    /// S).
    lines: Vec<(elevated::WhichStream, String)>,
    /// How many lines have begun, which gives each its place.
    lines_begun: u64,
}

/// **Which slot of [`TheRunsWords::being_printed`] a stream's line is in.**
fn the_slot_of(stream: elevated::WhichStream) -> usize {
    match stream {
        elevated::WhichStream::TheOrdinaryOutput => 0,
        elevated::WhichStream::TheComplaints => 1,
    }
}

impl TheRunsWords {
    /// **A piece, kept** — under its step and its stream, every byte as
    /// printed, and taken into the window's lines. **From 2026-10-25 both
    /// streams reach the lines**, in the order they arrive, each marked by
    /// its stream; **a line not yet ended shows as it stands**; **a line a
    /// program rewrites in place shows in its newest form** (M13b, the
    /// eighteenth notes, piece 2c). Until then the complaints stream never
    /// reached the lines and an unended line was never shown.
    ///
    /// **A line ends at a line feed.** A carriage return sends it back to its
    /// start, as a terminal does, so a carriage return before the feed is not
    /// part of it. Nothing else in a line is changed.
    pub fn a_piece_is_printed(&mut self, step: Step, piece: &APiecePrinted) {
        let (stream, bytes) = match piece {
            APiecePrinted::Output(bytes) => (elevated::WhichStream::TheOrdinaryOutput, bytes),
            APiecePrinted::Complaint(bytes) => (elevated::WhichStream::TheComplaints, bytes),
        };
        self.printed.printed(step, stream, bytes);
        let slot = the_slot_of(stream);
        if self.being_printed[slot].as_ref().is_some_and(|line| line.step != step) {
            self.being_printed[slot] = None;
        }
        for &byte in bytes.iter() {
            if self.being_printed[slot].is_none() {
                self.lines_begun += 1;
                self.being_printed[slot] = Some(TheLineBeingPrinted {
                    step,
                    shown: Vec::new(),
                    since: Vec::new(),
                    began: self.lines_begun,
                });
            }
            let Some(line) = self.being_printed[slot].as_mut() else {
                continue;
            };
            match byte {
                b'\n' => {
                    let ended = line.as_shown();
                    self.being_printed[slot] = None;
                    self.lines.push((stream, ended));
                    if self.lines.len() > THE_WINDOWS_LINES {
                        self.lines.remove(0);
                    }
                }
                b'\r' => {
                    line.shown = line.as_shown().chars().collect();
                    line.since.clear();
                }
                byte => {
                    if line.since.len() < A_LINE_IS_KEPT_UP_TO {
                        line.since.push(byte);
                    }
                }
            }
        }
    }

    /// **`Server Setup` Y7a item 5: some of what `step` printed could not be
    /// brought back**, kept under it as the mark. **The window's lines are not
    /// touched**: nothing was printed into them. Added 2026-10-24.
    pub fn its_words_were_not_kept(&mut self, step: Step) {
        self.printed.its_words_were_not_kept(step);
    }

    /// **Whether any step's words could not be brought back.**
    pub fn some_words_were_not_kept(&self) -> bool {
        self.printed.some_words_were_not_kept()
    }

    /// **The window**: the newest lines the run has printed, from either
    /// stream, oldest first, each with the stream it came from — the complete
    /// lines in the order they ended, then each line still being printed, in
    /// the order it began, as it stands now. Whichever step printed them, so
    /// a step that prints nothing leaves the last lines standing. Changed
    /// 2026-10-25, the eighteenth notes, piece 2c.
    pub fn the_window(&self) -> Vec<(elevated::WhichStream, String)> {
        let mut window = self.lines.clone();
        let mut still_printing: Vec<(&TheLineBeingPrinted, elevated::WhichStream)> = [
            (self.being_printed[0].as_ref(), elevated::WhichStream::TheOrdinaryOutput),
            (self.being_printed[1].as_ref(), elevated::WhichStream::TheComplaints),
        ]
        .into_iter()
        .filter_map(|(line, stream)| line.map(|line| (line, stream)))
        .collect();
        still_printing.sort_by_key(|(line, _)| line.began);
        for (line, stream) in still_printing {
            let shown = line.as_shown();
            if !shown.is_empty() {
                window.push((stream, shown));
            }
        }
        let past = window.len().saturating_sub(THE_WINDOWS_LINES);
        window.split_off(past)
    }

    /// **The window's lines, as text alone** — [`Self::the_window`] without
    /// which stream each came from.
    pub fn the_newest_lines(&self) -> Vec<String> {
        self.the_window().into_iter().map(|(_, line)| line).collect()
    }

    /// **One stream, whole: every step's record of it in turn**, every byte
    /// as printed — what *Copy* takes.
    pub fn the_whole_of(&self, stream: elevated::WhichStream) -> String {
        String::from_utf8_lossy(&self.printed.the_whole_of(stream)).into_owned()
    }
}

type TheWordsHeld = std::sync::Mutex<Vec<(String, TheRunsWords)>>;

fn the_words_kept() -> &'static TheWordsHeld {
    static HELD: std::sync::OnceLock<TheWordsHeld> = std::sync::OnceLock::new();
    HELD.get_or_init(|| std::sync::Mutex::new(Vec::new()))
}

/// **`Server Setup` R1b's swap's own words, apart from the setup's** —
/// `Server Setup` Y7a as changed 2026-10-30: a swap never empties the setup's
/// words. Each swap's are its own; nothing shows them yet.
fn the_swaps_words_kept() -> &'static TheWordsHeld {
    static HELD: std::sync::OnceLock<TheWordsHeld> = std::sync::OnceLock::new();
    HELD.get_or_init(|| std::sync::Mutex::new(Vec::new()))
}

/// **`Server Setup` Y7a: the user asked for an act on `server`, so the last
/// act's words go** and this one's are kept from nothing. **Only an act the
/// user asked for** (changed 2026-10-30, the twenty-third notes, note 3): the
/// next act in a line, a run taken up again and the swap never call this.
pub fn a_runs_words_begin(server: &str) {
    if let Ok(mut kept) = the_words_kept().lock() {
        kept.retain(|(one, _)| one != server);
        kept.push((server.to_string(), TheRunsWords::default()));
    }
}

/// **The record a run's words go to on `server`, begun only where none is
/// kept** — a run taken up again, or the next act in a line, adds to the
/// words already there. Added 2026-10-30, the twenty-third notes, note 3.
fn the_words_are_kept_on(held: &TheWordsHeld, server: &str) {
    if let Ok(mut kept) = held.lock() {
        if !kept.iter().any(|(one, _)| one == server) {
            kept.push((server.to_string(), TheRunsWords::default()));
        }
    }
}

/// **Something a step printed, kept in `held` on `server`.** False where no
/// run's words are being kept there.
fn kept_in(held: &TheWordsHeld, server: &str, keep: impl FnOnce(&mut TheRunsWords)) -> bool {
    let Ok(mut kept) = held.lock() else {
        return false;
    };
    match kept.iter_mut().find(|(one, _)| one == server) {
        Some((_, words)) => {
            keep(words);
            true
        }
        None => false,
    }
}

/// **A piece a step printed on `server`, kept.** False where no run's words
/// are being kept there.
pub fn a_piece_is_kept(server: &str, step: Step, piece: &APiecePrinted) -> bool {
    kept_in(the_words_kept(), server, |words| words.a_piece_is_printed(step, piece))
}

/// **`Server Setup` Y7a item 5: a step's mark that its words were not kept,
/// kept on `server`.** False where no run's words are being kept there. Added
/// 2026-10-24.
pub fn a_mark_is_kept(server: &str, step: Step) -> bool {
    kept_in(the_words_kept(), server, |words| words.its_words_were_not_kept(step))
}

/// **The whole of what the last swap on `server` printed**, the ordinary
/// stream and then the complaints — kept apart from the setup's. Added
/// 2026-10-30, the twenty-third notes, note 3.
pub fn the_whole_of_what_the_swap_printed_on(server: &str) -> Option<(String, String)> {
    let kept = the_swaps_words_kept().lock().ok()?;
    let (_, words) = kept.iter().find(|(one, _)| one == server)?;
    Some((
        words.the_whole_of(elevated::WhichStream::TheOrdinaryOutput),
        words.the_whole_of(elevated::WhichStream::TheComplaints),
    ))
}

/// **Each step's words on `server`, in the order the run printed them** — what
/// `Server Setup` Y7a keeps, apart by stream and by step. Added 2026-10-24.
pub fn the_words_of_each_step_on(server: &str) -> Option<Vec<elevated::OneStepsWords>> {
    let kept = the_words_kept().lock().ok()?;
    let (_, words) = kept.iter().find(|(one, _)| one == server)?;
    Some(words.printed.the_steps().to_vec())
}

/// **Whether some of what the run on `server` printed could not be brought
/// back** — what *Server output* says under the words it has.
pub fn some_of_what_was_printed_was_not_kept_on(server: &str) -> bool {
    the_words_kept()
        .lock()
        .ok()
        .and_then(|kept| {
            kept.iter()
                .find(|(one, _)| one == server)
                .map(|(_, words)| words.some_words_were_not_kept())
        })
        .unwrap_or(false)
}

/// **Whether a run's words are kept for `server`** — from the run being taken
/// until the next act the user asks for or until Smoozy closes, whatever it
/// printed.
pub fn the_words_are_kept_for(server: &str) -> bool {
    the_words_kept()
        .lock()
        .map(|kept| kept.iter().any(|(one, _)| one == server))
        .unwrap_or(false)
}

/// **The window's lines on `server`**, whichever step printed them.
pub fn the_newest_lines_on(server: &str) -> Vec<String> {
    the_words_kept()
        .lock()
        .ok()
        .and_then(|kept| {
            kept.iter()
                .find(|(one, _)| one == server)
                .map(|(_, words)| words.the_newest_lines())
        })
        .unwrap_or_default()
}

/// **The window on `server`, each line with the stream it came from** —
/// `Desktop app and delivery` M13b's second narrowing as changed on
/// 2026-10-24. Added 2026-10-25, the eighteenth notes, piece 2c.
pub fn the_window_on(server: &str) -> Vec<(elevated::WhichStream, String)> {
    the_words_kept()
        .lock()
        .ok()
        .and_then(|kept| {
            kept.iter()
                .find(|(one, _)| one == server)
                .map(|(_, words)| words.the_window())
        })
        .unwrap_or_default()
}

/// **The whole of what the run on `server` printed**, the ordinary stream and
/// then the complaints, each whole.
/// `Server Setup` Y7e: the run's own reason, cleared with a new user's run.
pub fn the_setup_reason_is_kept(server: &str, reason: String) {
    the_words_are_kept_on(the_words_kept(), server);
    kept_in(the_words_kept(), server, |words| words.failure_reason = Some(reason));
}

pub fn the_setup_reason_on(server: &str) -> Option<String> {
    the_words_kept().lock().ok()?.iter().find(|(one, _)| one == server)?
        .1.failure_reason.clone()
}

pub fn the_whole_of_what_was_printed_on(server: &str) -> Option<(String, String)> {
    let kept = the_words_kept().lock().ok()?;
    let (_, words) = kept.iter().find(|(one, _)| one == server)?;
    Some((
        words.the_whole_of(elevated::WhichStream::TheOrdinaryOutput),
        words.the_whole_of(elevated::WhichStream::TheComplaints),
    ))
}

/// **`Server Setup` Y8: the user's stop, written down and honoured at a
/// boundary.** **No signal is ever sent to the worker or its children.**
pub fn the_user_asked_to_stop(server: &str) {
    if let Ok(mut held) = the_runs_this_copy_is_driving().lock() {
        if !held.asked_to_stop.iter().any(|one| one == server) {
            held.asked_to_stop.push(server.to_string());
        }
    }
}

/// The stop, as the run reads it between two steps.
pub struct TheStopThisServerWasAsked {
    pub server: String,
}

impl AsksToStop for TheStopThisServerWasAsked {
    fn the_user_asked_to_stop(&self) -> bool {
        the_runs_this_copy_is_driving()
            .lock()
            .map(|held| held.asked_to_stop.iter().any(|one| one == &self.server))
            .unwrap_or(false)
    }
}

/// **`Server Setup` R1's ready action on one connected server, whole.**
///
/// **It runs off the drawing thread** (P5) and blocks the thread it is given,
/// which is what the Set up screen's act already does for a submission.
pub fn the_ready_action_on<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    point: &ThisServersSessions,
    server: &str,
    install_this_blender: Option<String>,
) -> WhereTheSetUpGot {
    // **`Server Setup` T4's mark on this server's connected period**, taken
    // before anything is driven because `Server Setup` R1 item 6 is the step
    // that needs it and the mark is the period's rather than the run's.
    // **Nothing where no period stands**, and then no restart is issued at all.
    let the_period = tauri::Manager::try_state::<
        std::sync::Arc<crate::servers::a_period::ThePeriodsThisRun>,
    >(app)
        .and_then(|periods| periods.the_restart_mark_of(server));
    // **`State, identity and recovery` Y9's check, as this server's live
    // connection read it before the press** — the same reading and the same
    // test `Servers` Q1 step 7 runs on, called rather than taken again. **A
    // reading an earlier connection took is not the live one's**
    // (`Servers` H1a), so it is handed in as no reading, and the order runs
    // whole. Step 10c's ninth-pass fix piece 3.
    let the_live_connection_read = tauri::Manager::try_state::<
        std::sync::Arc<crate::servers::a_period::ThePeriodsThisRun>,
    >(app)
        .and_then(|periods| periods.what_the_last_connection_read(server))
        .filter(|read| read.on_the_connection_that_is_live)
        .map(|read| read.check);
    let stop = TheStopThisServerWasAsked {
        server: server.to_string(),
    };
    use tauri::Manager;
    let Some(started) = app.try_state::<std::sync::Mutex<crate::start::Started>>() else {
        return WhereTheSetUpGot::TheRootCouldNotBeRecorded {
            said: "Smoozy's database is not available.".to_string(),
        };
    };
    // **The record is lent for each read and each write alone** (L4), so
    // nothing holds the spine's lock across a machine act.
    let record = crate::keys::TheRunsRecord {
        started: started.inner(),
    };
    let mut row = AndTheCommitIsTold {
        row: TheServersRowHere {
            record: &record,
            server: server.to_string(),
        },
        app,
    };
    let mut store = TheTrustRecordThroughTheSpine { record: &record };
    let mut over_https = TheCurrentReleaseOverHttps {
        store: &mut store,
        from: WHERE_THE_CURRENT_RELEASE_IS_READ_FROM,
    };
    // **`Server protocol and transfers` B10a: a development copy pointed at a
    // throwaway folder and given `--desk-program=<file>` places that file
    // instead of fetching the current release.** This is the seam's one caller
    // and the only place the choice is made; everything after the fetch is
    // unchanged. **A release build compiles none of it**, so there the reader
    // is always the one over HTTPS.
    #[cfg(debug_assertions)]
    let mut the_desk_program = crate::the_desk_program::the_one_this_copy_was_given();
    #[cfg(debug_assertions)]
    let releases: &mut dyn ReadsTheCurrentRelease = match the_desk_program.as_mut() {
        Some(desk) => desk,
        None => &mut over_https,
    };
    #[cfg(not(debug_assertions))]
    let releases: &mut dyn ReadsTheCurrentRelease = &mut over_https;
    let runtime = match tokio::runtime::Builder::new_current_thread().enable_all().build() {
        Ok(runtime) => runtime,
        Err(_) => {
            return WhereTheSetUpGot::ThePlacementCouldNotBeCarriedOut {
                said: "Could not start the task.".to_string(),
            }
        }
    };
    let mut taking = TheOpeningReadingsOverThePoint { sessions: point };
    // **`Server Setup` G2: the material is placed by whichever entry point runs
    // while Telegram is on, and is not placed in advance.** Whether it is on is
    // `Telegram`'s own switch and the record's own proved bot — this act
    // decides nothing about it and only asks.
    let mut telegram = TheMaterialFromTheRecord {
        record: &record,
        server: server.to_string(),
        store: crate::surface::keys::the_store_for_this_system(),
    };
    let asked_for = WhatWasAskedFor {
        // **Where the caller asked for a Blender, R1 composes the fetch and the
        // publication into this one run** — `Server Setup` §11's *"the quick
        // path's two ticked options run as one run (R1, which composes the
        // Blender publication into the ready action's own sequence)"*. **The Set
        // up card's own action asks for none**: a Blender picked in the Blender
        // section is a run of its own kind (`Server Setup` Y6b), and it is
        // [`a_blender_install_on`]'s. *(Until 2026-09-21 this was `None` for
        // every caller, so the Add dialog's tick installed nothing.)*
        install_this_blender,
        // **`Server Setup` R1 item 8 is composed into the run only where the
        // user asked for Telegram**, and a run with it off never meets the step
        // at all.
        telegram_is_on: telegram.it_is_on(),
        // **`Server Setup` R1b: the ready action runs the search**, so item
        // 5's bar covers the Blenders already there, including where this run
        // installs none. It is taken before the run, over the same point, and
        // nothing is claimed where the machine could not be asked. Added
        // 2026-10-12.
        the_blenders_on_the_server: runtime.block_on(
            crate::servers::the_driver_swap::the_blenders_found_now(point, server, row.row.the_row().as_ref()),
        ),
        the_server: Some(server.to_string()),
    };
    // **What an earlier run told of item 5 is forgotten** before this one
    // compares (`Server Setup` R1c).
    crate::servers::the_driver_swap::a_run_whose_item_5_may_compare_begins(server);
    let mut placing = TheReleaseThisRunPlaces { reads: releases, placed: None };
    let came_to = runtime.block_on(a_machine_is_set_up(
        point,
        &mut taking,
        &mut row,
        OverTheLentConnection,
        &mut placing,
        &asked_for,
        // **`Server Setup` E2's flow begins at the moment a step needs a
        // password and never before**, so nothing is held here — and
        // `Server Setup` M3's probe, taken inside the order, is what turns this
        // into passwordless elevation on a login that needs no password.
        HowElevationIsObtained::APasswordIsOwedAndNoneIsHeld,
        &mut telegram,
        &mut TheSetUpTabAsks {
            app: app.clone(),
            server: server.to_string(),
        },
        &stop,
        the_period,
        the_live_connection_read,
    ));
    // **`Servers` H11b's second case, kept**: whether this run left the driver
    // below the OptiX bar, on the row, before anything is told. Added
    // 2026-10-11, the fourth piece of the OptiX and driver work.
    the_driver_miss_is_kept(&record, server, &came_to);
    // **`Telegram` V1c: a run that placed a release and reached *done* says
    // what the machine now runs**, and the queue is told again, since step 9
    // does not read a server a setup is driving (the twenty-fourth notes,
    // note 4).
    if the_release_a_finished_setup_placed_is_kept(server, &came_to, placing.placed) {
        crate::surface::the_queue::the_queue_is_told_again(app);
    }
    // **`Server Setup` R1b: what item 5 concluded is held for the connected
    // period** — a swap left out for live work is owed, and anything else this
    // run reached *done* with replaces it. Added 2026-10-12.
    crate::servers::the_driver_swap::the_ending_is_kept(server, &came_to, &asked_for);
    // **`Server Setup` R1c: the holds of the driver's reason are released at
    // the ending of the run whose item 5 compared and acted**, where that
    // ending says so. Added 2026-10-12, held-job piece A.
    crate::servers::the_driver_swap::the_holds_are_released_after_the_run(app, server, &came_to);
    // **`Server Setup` §12's *the ready action ran*, one per action asked
    // for.** Added 2026-09-22.
    the_act_is_in_the_diary(
        app,
        server,
        smoozy_server_setup::diary::ActLine::TheReadyActionRan {
            ending: the_endings_name(&came_to),
        },
    );
    // **Settings' Telegram section is posted again where the material was not
    // composed**, because *not there* forgot the stored fact and the section
    // now draws *not stored* (`Security and licence` §3.0 rules 4 and 6). The
    // Creator's eleventh notes, note 2.
    if matches!(
        came_to,
        WhereTheSetUpGot::TheRunEnded {
            ending: TheRunsEnding::TheTelegramMaterialWasNotComposed { .. },
            ..
        }
    ) {
        the_telegram_section_is_posted_again(app);
    }
    came_to
}

/// **`Server Setup` R1b's swap as a run of its own, on one connected server** —
/// started by the desktop, never by a press, once the server's work has ended
/// ([`crate::servers::the_driver_swap::an_owed_swap_is_tried`]). Added
/// 2026-10-12.
///
/// **It runs off the drawing thread** and blocks the thread it is given, as
/// the ready action does. **It takes `Server Setup` T4's mark on this server's
/// connected period** the way [`the_ready_action_on`] does, because item 6's
/// restart needs it; with no period standing no restart is issued at all.
/// **Its miss is kept on the row** as the ready action's is — the runner's
/// ruling of 2026-10-11: a missed swap is not retried on its own — and what
/// item 5 concluded is held for the period. **It writes no `Server Setup` §12
/// act line**: §12's lines are per act the user asked for, and nobody asked
/// for this one.
pub fn the_swap_run_on<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    point: &ThisServersSessions,
    server: &str,
    the_blenders_on_the_server: Vec<smoozy_server_setup::blender::Version>,
) -> WhereTheSetUpGot {
    use tauri::Manager;
    let the_period = app
        .try_state::<std::sync::Arc<crate::servers::a_period::ThePeriodsThisRun>>()
        .and_then(|periods| periods.the_restart_mark_of(server));
    let Some(started) = app.try_state::<std::sync::Mutex<crate::start::Started>>() else {
        return WhereTheSetUpGot::TheServerIsNotSetUpYet;
    };
    let record = crate::keys::TheRunsRecord {
        started: started.inner(),
    };
    let mut row = TheServersRowHere {
        record: &record,
        server: server.to_string(),
    };
    let Ok(runtime) = tokio::runtime::Builder::new_current_thread().enable_all().build() else {
        return WhereTheSetUpGot::ThePlacementCouldNotBeCarriedOut {
            said: "Could not start the task.".to_string(),
        };
    };
    let mut taking = TheOpeningReadingsOverThePoint { sessions: point };
    let asked_for = WhatWasAskedFor {
        the_blenders_on_the_server,
        the_server: Some(server.to_string()),
        ..WhatWasAskedFor::default()
    };
    // **What an earlier run told of item 5 is forgotten** before this one
    // compares (`Server Setup` R1c).
    crate::servers::the_driver_swap::a_run_whose_item_5_may_compare_begins(server);
    let came_to = runtime.block_on(the_swap_is_made(
        point,
        &mut taking,
        &mut row,
        &asked_for,
        HowElevationIsObtained::APasswordIsOwedAndNoneIsHeld,
        &mut TheSetUpTabAsks {
            app: app.clone(),
            server: server.to_string(),
        },
        &TheStopThisServerWasAsked {
            server: server.to_string(),
        },
        the_period,
    ));
    the_driver_miss_is_kept(&record, server, &came_to);
    crate::servers::the_driver_swap::the_ending_is_kept(server, &came_to, &asked_for);
    // **`Server Setup` R1c's release**, as at the ready action's ending.
    crate::servers::the_driver_swap::the_holds_are_released_after_the_run(app, server, &came_to);
    came_to
}

/// **`Server Setup` R1b, as written 2026-11-03: whether the swap's run may begin
/// on this machine at all**, read before the exclusion is taken. No setup may
/// stand, because the machine refuses a fresh run then and carrying that setup on
/// is the user's *Set up again*. And the driver must give its version, because
/// the comparison reads the connection's first answer, which the Sesterce pass's
/// swap had removed by the time the comparison read it.
///
/// **A health answer that could not be had says no setup stands**, as
/// [`the_run_the_state_stands_for`] reads it, and the machine's own refusal is
/// still there behind it.
///
/// **A stopped run this copy drove that left no state counts the same**
/// (`Server Setup` S4 item 5): `drove` is those runs, newest first, and a swap's
/// run opened over one would replace it (`Server Setup` §11's sweep).
pub async fn a_swap_may_begin<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    package_area: &str,
    drove: &[Identity],
) -> bool {
    if the_run_the_state_stands_for(sessions, package_area).await.is_some()
        || the_run_that_stopped_and_left_no_state(sessions, package_area, drove).await.is_some()
    {
        return false;
    }
    let mut channel = OnThisServersPoint { sessions };
    smoozy_servers::readings::ask_the_drivers_versions(&mut channel)
        .await
        .0
        .is_some()
}

/// **`Server Setup` R1b's swap run, driven: the readings, item 5's swap, item
/// 6's restart and *done*.** Added 2026-10-12.
///
/// **The order**, the Blender install's own: `Server Setup` M1's and `Server Setup` M1a's
/// readings, afresh, and nothing else if either refuses; the package area this
/// copy recorded, without which there is no program to open a run with;
/// `Server Setup` M3's probe, **so elevation is settled before the
/// machine-level state is set** — where a password is owed and none is held,
/// `Server Setup` E2's ask is made at the driver step, before the state, and a
/// declined ask makes no swap; `Server Setup` R1 item 1's readings, which read
/// the driver's version again, so **where it now meets the bar nothing is
/// done**; and the run, of the ready action's own kind, composed of items 5
/// and 6 alone ([`ready_action::compose_the_swap_run`]).
///
/// **It takes no stopped setup over.** A run nobody pressed for never carries
/// on a setup the user left standing; where one stands the machine refuses a
/// fresh run, and this ends there with nothing done.
///
/// **The live set is read at the driver step, and again once the state
/// stands** (`Server Setup` R1b's last guard): where work has arrived since
/// the desktop's reading, the run reaches *done* with nothing done and the
/// swap waits again.
#[allow(clippy::too_many_arguments)]
pub async fn the_swap_is_made<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    taking: &mut dyn TakesTheOpeningReadings,
    row: &mut dyn KeepsTheServersRow,
    asked_for: &WhatWasAskedFor,
    elevation: HowElevationIsObtained<'_>,
    asker: &mut dyn AsksForThePassword,
    stop: &dyn AsksToStop,
    the_period: Option<TheRestartMark>,
) -> WhereTheSetUpGot {
    let said = match the_opening_readings(taking).await {
        Ok(said) => said,
        Err(stopped) => return stopped,
    };
    let row_now = row.the_row();
    let Some(package_area) = row_now
        .as_ref()
        .filter(|row| row.last_confirmed_installation_identity.is_some())
        .and_then(the_package_area_of)
    else {
        return WhereTheSetUpGot::TheServerIsNotSetUpYet;
    };
    // **Its own words, apart from the setup's** (`Server Setup` Y7a as
    // changed 2026-10-30, the twenty-third notes, note 3).
    let words = asker.where_the_swaps_words_go();
    let elevation = the_elevation_this_login_has_telling(sessions, elevation, words.as_ref()).await;
    let needs = what_this_machine_needs(sessions, &said, asked_for, false, words.as_ref()).await;
    let steps = ready_action::compose_the_swap_run(&needs);
    let mut run = match TheRunBeingDriven::opened_with(
        sessions,
        &package_area,
        SetupKind::TheReadyAction,
        steps,
        needs,
        the_period,
        Vec::new(),
        stop,
        false,
        asker,
    )
    .await
    {
        Ok(run) => run,
        Err(ending) => {
            return WhereTheSetUpGot::TheRunEnded {
                at: Boundary::Started,
                ending,
            }
        }
    };
    run.the_words_go = words;
    run.driven(elevation, asked_for, &mut TelegramIsNotOn, asker, stop)
        .await
}

/// **What a run's ending leaves of `Server Setup` R1 item 5's driver check**:
/// whether the driver missed the OptiX bar, where the run reached *done*, and
/// nothing where it ended any other way.
///
/// **Only *done* speaks.** A run that stopped part-way, failed, or never
/// reached its driver check says nothing about the driver, so a miss recorded
/// earlier stands — the runner's ruling of 2026-10-11 (`Audits/Architecture/
/// Server Setup/Decisions record.md`, its last entry): *a missed swap is not
/// retried on its own*, and the miss stands for that server until the user's
/// next setup run on it.
pub fn the_miss_this_run_leaves(came_to: &WhereTheSetUpGot) -> Option<bool> {
    match came_to {
        WhereTheSetUpGot::TheRunEnded {
            ending: TheRunsEnding::ItReachedDone { the_driver_missed_the_optix_bar, .. },
            ..
        } => Some(*the_driver_missed_the_optix_bar),
        _ => None,
    }
}

/// **`Servers` H11b's second case, kept on the server's row** (`Servers` D3's
/// row of 2026-10-11) — the statement's third input when the card is drawn,
/// read beside the reading on every connection after this one and while none
/// stands.
///
/// **Called by every run that reaches `Server Setup` R1 item 5's driver
/// check**: the ready action, by either of its two callers, and **`Server
/// Setup` R1b's swap run** ([`the_swap_run_on`], from 2026-10-12), which
/// needed nothing new here. A Blender install, a removal and Telegram's
/// placement take no driver check and do not call it, so they leave a miss
/// where it was.
///
/// **A write that does not land is not retried and is not said**: the run's
/// own ending is told as it stands, and the next run's ending writes again.
pub fn the_driver_miss_is_kept(
    record: &dyn crate::keys::LendsTheRecord,
    server: &str,
    came_to: &WhereTheSetUpGot,
) {
    let Some(missed) = the_miss_this_run_leaves(came_to) else {
        return;
    };
    record.lend(&mut |connection| {
        let _ = crate::record::server_rows::the_driver_miss_is_written(connection, server, missed);
    });
}

/// **The row this ready action keeps, and the one moment it hands on** —
/// `Servers` Q19a's commit, told to the period the connection belongs to.
/// Added 2026-09-27, step 10c's eighth-pass fix piece 3.
struct AndTheCommitIsTold<'r, R: tauri::Runtime> {
    row: TheServersRowHere<'r>,
    app: &'r tauri::AppHandle<R>,
}

impl<R: tauri::Runtime> KeepsTheServersRow for AndTheCommitIsTold<'_, R> {
    fn the_row(&mut self) -> Option<ServerRow> {
        self.row.the_row()
    }

    fn record_the_root(&mut self, recorded: RecordedMachineRoot) -> Result<(), String> {
        self.row.record_the_root(recorded)
    }

    fn record_the_installation_and_clear_the_cursor(
        &mut self,
        identity: &Identity,
    ) -> Result<(), String> {
        self.row.record_the_installation_and_clear_the_cursor(identity)
    }

    fn clear_the_remembered_setup(&mut self) -> Result<(), String> {
        self.row.clear_the_remembered_setup()
    }

    fn the_placement_committed(&mut self) {
        crate::servers::a_period::a_placement_committed(self.app, &self.row.server);
    }
}

/// **The name this module writes its diary lines under**, read from the
/// module's own declaration rather than written a second time here.
pub use smoozy_server_setup::diary::THE_MODULE;

/// **`Server Setup` §12's act lines, written where the act ends.** Added
/// 2026-09-22, step 10c's fourth pass on Vast, fault 7.
///
/// **The declaration was written and registered by nothing**, so
/// `Desktop app and delivery` A7's hosting rule refused every line this module
/// composed and a whole pass on a real rental left three lines in the diary.
/// **Two of that declaration's five acts are built and are the two written
/// here** — the ready action and a Blender install; an add-on install, a
/// Blender removal and Telegram's own entry point are acts nothing in this
/// application performs yet, and a line for an act that did not happen is the
/// one thing a declaration may never carry.
///
/// *Note added 2026-10-16, step 10c's owed piece 12; the words above kept as
/// written.* Four of the five are written from here: the ready action, a
/// Blender install (both routes), Telegram's own entry point and a Blender
/// removal (both routes). An add-on install is still an act nothing here
/// performs.
///
/// **`Server Setup` §12: the line is per act the user asked for and never per
/// setup run**, so it is written once where the act ends, whatever the run
/// inside it did — a recovery that carried an interrupted run on is that same
/// act and writes no second line.
///
/// **The identities are §12's own**: the server row, and the installation
/// identity the machine answered under, read off the row rather than held a
/// second time (`State, identity and recovery` §2.3).
///
/// **The outcome goes nowhere**, which is `Desktop app and delivery` L16's one
/// legitimate use of it: a refused write loses the entry and nothing else.
fn the_act_is_in_the_diary<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    server: &str,
    line: smoozy_server_setup::diary::ActLine,
) {
    use tauri::Manager;
    let Some(started) = app.try_state::<std::sync::Mutex<crate::start::Started>>() else {
        return;
    };
    let Ok(started) = started.lock() else {
        return;
    };
    let (Some(service), Some(run)) = (started.diary.as_ref(), started.run.as_ref()) else {
        return;
    };
    let Some(server_row) = Identity::read_back(IdentityKind::LocalServer, server) else {
        return;
    };
    // **The installation identity the machine answered under**, off the row.
    // **A row that carries none writes no line**: §12 names it among the
    // identities every line of this module carries, and a line without it could
    // read as a line about the machine that replaced this one.
    let installation = started
        .record
        .as_ref()
        .and_then(|record| {
            crate::record::server_rows::one(record.connection(), server)
                .ok()
                .flatten()
        })
        .and_then(|row| row.last_confirmed_installation_identity)
        .and_then(|text| Identity::read_back(IdentityKind::Installation, &text));
    let Some(installation) = installation else {
        return;
    };
    let carries = smoozy_server_setup::diary::WhatEveryLineCarries {
        run: run.clone(),
        server_row,
        installation,
        at: std::time::SystemTime::now(),
        clock: smoozy_state::observed::Clock::ThisComputer,
    };
    let diary = crate::diary_service::ModulesDiary {
        service,
        declarations: &started.declarations,
        run: run.clone(),
        module: THE_MODULE,
    };
    let _ = diary.write(&line.line(&carries));
}

/// **Which of `Server Setup` W1's endings the act came to**, from the closed set
/// of six the declaration fixes — W1's five statements plus *stopped part-way*
/// for a run that ended at a boundary without completing.
fn the_endings_name(came_to: &WhereTheSetUpGot) -> smoozy_server_setup::diary::TheEndingsName {
    use smoozy_server_setup::diary::TheEndingsName as Name;
    match came_to {
        WhereTheSetUpGot::TheKindOrTheFamilyWouldNotRead { .. }
        | WhereTheSetUpGot::TheRentalWouldNotNameItself => Name::NotAKindSmoozyPrepares,
        WhereTheSetUpGot::TheRunEnded { ending, .. } => match ending {
            TheRunsEnding::ItReachedDone { .. } => Name::Ready,
            TheRunsEnding::AStepNeedsAdminAccess { .. } => Name::Refused,
            TheRunsEnding::TheUserStoppedIt
            | TheRunsEnding::ItStoppedInsideAStep { .. }
            | TheRunsEnding::ASetupThatStoppedPartWayStands { .. }
            | TheRunsEnding::TheMachineIsRestarting
            | TheRunsEnding::TheServerEndedTheRun { .. }
            | TheRunsEnding::TheMachineDidNotComeBack { .. } => Name::StoppedPartWay,
            _ => Name::CouldNotBeMadeReady,
        },
        _ => Name::CouldNotBeMadeReady,
    }
}

/// **A setup run, as the machine's own record holds it, read fresh** —
/// `Server Setup` S1, `Server Setup` S2 and `Server Setup` Y6a. Added 2026-09-21.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheSetupAsRead {
    /// `Server Setup` Y1 and `Server Setup` S6: the fresh health answer names this run as the
    /// owner of the machine-level setup hold. A failed record alone does not
    /// establish that hold. False where the health reading is unavailable.
    pub the_run_holds_the_machine: bool,
    /// The record, in `Server Setup`'s own shape.
    pub record: smoozy_server_setup::run::TheSetupRecord,
    /// **Whether the run is still open** — no recorded ending yet.
    pub still_open: bool,
    /// `Server Setup` S1: the step that was in flight, where the reading names
    /// one.
    pub the_step_in_flight: Option<Boundary>,
    /// **An admin step the desktop recorded as started, still standing when the
    /// reading was taken** — `Server Setup` Y7's second spelling of the step
    /// under way, named by its first boundary, and nothing where the reading
    /// names a worker step or none. **A run that failed with this standing
    /// failed in its admin act**: the machine clears it when the next worker
    /// step is commanded, and a worker step's failure ends the run with it
    /// cleared. So a tooling step that failed with *tooling* standing failed
    /// at the unpack tool's own install, not at the unpack. Added 2026-10-25,
    /// the small owed fixes after the eighteenth notes.
    pub the_admin_step_recorded_as_started: Option<Boundary>,
    /// Which machine clock its moments are on.
    pub the_clock: String,
}

/// **`Server Setup` S1 and S2: the machine read for its setup, fresh, on the
/// connection that stands.** Added 2026-09-21, out of the first pass on a real
/// rental, where the Set up card said *not read* on a connected server with a
/// setup that had just failed on it.
///
/// **Which run is read, in order:**
///
/// 1. **The run `Rented-server runtime` V9's health answer names** — the one
///    the machine-level setup-in-progress state stands for. It is the
///    machine's own answer, so it is found whichever copy of Smoozy drove the
///    run and however long ago, and it is exactly the part-finished
///    machine-changing setup `Design/Server Setup § Cut off part-way` shows.
/// 2. **Otherwise the run of the asked kind this copy last drove on this
///    server** ([`the_last_run_this_copy_drove`]).
///
/// **What it cannot find by itself**: a finished setup driven before this copy
/// was opened. **Since 2026-10-03 (the 38's piece 6) the machine names the
/// ready action's retained run** — `smoozy_framing`'s
/// `THE_SET_UP_CARDS_READING_REQUEST`, read by [`the_card_as_read`], which
/// hands that run here where neither of the two above names one. **An
/// additive run that stopped before this copy was opened is still not found**:
/// the card reads the ready action's record and no other kind
/// (`Server Setup` Y6b).
///
/// **Nothing where no run is known or the machine does not answer**, and the
/// caller says so rather than drawing a reading that was not taken.
pub async fn the_setup_as_read<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    package_area: &str,
    remembered: Option<Identity>,
) -> Option<TheSetupAsRead> {
    let standing = the_run_the_state_stands_for(sessions, package_area).await;
    let the_run_holds_the_machine = standing.is_some();
    let run = standing.or(remembered)?;
    let Ok(identity) = Identity::issue(IdentityKind::Operation) else {
        return None;
    };
    let request = Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        identity,
        asked::THE_SETUP_RUNS_STATE_REQUEST,
    )
    .carrying(Fields::none().and(asked::THE_SETUP_RUN_FIELD, run.as_text()));
    let answered = ask(sessions, package_area, &request).await?;
    let mut read = the_setup_as_the_machine_answered(&answered)?;
    read.the_run_holds_the_machine = the_run_holds_the_machine;
    Some(read)
}

/// **The run this copy drove that stopped at a boundary and left no
/// machine-level state**, where there is one, read off the machine. Added
/// 2026-09-22, step 10c's fourth pass on Vast, fault 6.
///
/// **Nothing where a state stands**, because the state names the one run a
/// recovery may take over (`Server Setup` S4 item 2) and
/// [`TheRunBeingDriven::opened`]'s refusal route is what finds that one.
///
/// **A run is taken over only where the machine's own record says it stopped
/// without finishing** — still open, or ended at its boundary, or left stopped,
/// or a step failed. A run that reached *done*, was abandoned, or declined at
/// the version answer is finished business and a fresh run is what the press
/// means. **The reading is the machine's and nothing is remembered but the
/// identity** (`Server Setup` S1, S2).
async fn the_run_that_stopped_and_left_no_state<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    package_area: &str,
    drove: &[Identity],
) -> Option<Identity> {
    use smoozy_server_setup::recovery::HowTheRunEnded;
    if drove.is_empty() {
        return None;
    }
    if the_run_the_state_stands_for(sessions, package_area)
        .await
        .is_some()
    {
        return None;
    }
    for run in drove {
        let Some(read) = the_state_of_one_run(sessions, package_area, run).await else {
            continue;
        };
        if read.record.a_setup_finished() {
            continue;
        }
        let it_stopped = match &read.record.ending {
            None => read.still_open,
            Some(HowTheRunEnded::Completed)
            | Some(HowTheRunEnded::Abandoned { .. })
            | Some(HowTheRunEnded::DeclinedAtTheVersionAnswer) => false,
            Some(_) => true,
        };
        if it_stopped {
            return Some(run.clone());
        }
    }
    None
}

/// One run's state, read off the machine by its identity.
async fn the_state_of_one_run<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    package_area: &str,
    run: &Identity,
) -> Option<TheSetupAsRead> {
    let Ok(identity) = Identity::issue(IdentityKind::Operation) else {
        return None;
    };
    let request = Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        identity,
        asked::THE_SETUP_RUNS_STATE_REQUEST,
    )
    .carrying(Fields::none().and(asked::THE_SETUP_RUN_FIELD, run.as_text()));
    let answered = ask(sessions, package_area, &request).await?;
    the_setup_as_the_machine_answered(&answered)
}

/// **One run's record, read off the machine by its identity, in both of this
/// file's shapes**: the state the driving reads, and the record the screen
/// composes from — one request, two readings of the same answer. Added
/// 2026-09-27, because a recovery resuming between the driver step's two
/// boundaries needs the driver step's own `Server Setup` Y4a mark, which only
/// the second shape carries.
async fn a_runs_record<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    package_area: &str,
    run: &Identity,
) -> Option<(TheRunsState, Option<TheSetupAsRead>)> {
    let Ok(identity) = Identity::issue(IdentityKind::Operation) else {
        return None;
    };
    let request = Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        identity,
        asked::THE_SETUP_RUNS_STATE_REQUEST,
    )
    .carrying(Fields::none().and(asked::THE_SETUP_RUN_FIELD, run.as_text()));
    let answered = ask(sessions, package_area, &request).await?;
    Some((
        the_state_in(&answered),
        the_setup_as_the_machine_answered(&answered),
    ))
}

/// **`Rented-server runtime` V9's field, read off the health answer**: the run
/// the machine-level setup-in-progress state stands for, or nothing.
async fn the_run_the_state_stands_for<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    package_area: &str,
) -> Option<Identity> {
    let Ok(identity) = Identity::issue(IdentityKind::Operation) else {
        return None;
    };
    let request = Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        identity,
        smoozy_server_protocol::placement::THE_HEALTH_ANSWER_REQUEST,
    );
    let answered = ask(sessions, package_area, &request).await?;
    let text = answered.fields.text(asked::THE_SETUP_IN_PROGRESS_FIELD)?;
    Identity::read_back(IdentityKind::Operation, text)
}

/// **The state reading's answer, in `Server Setup`'s own record shape.**
///
/// **Everything is the machine's**: the kind, the composed list, each boundary
/// reached with its mark, its moment and its clock, the ending and its words.
/// **A field this side cannot read is dropped from the reading rather than
/// guessed**, and a reading with no kind it knows is no reading at all.
pub fn the_setup_as_the_machine_answered(answered: &Answered) -> Option<TheSetupAsRead> {
    use smoozy_server_setup::recovery::{HowTheRunEnded, TheFailedStepsWords};
    use smoozy_server_setup::run::{RecordedBoundary, TheSetupRecord};
    if answered.fields.text(asked::ON_THIS_MACHINE_FIELD) != Some(asked::IT_IS_PRESENT) {
        return None;
    }
    let kind = the_kind_named(answered.fields.text(asked::THE_RUNS_KIND_FIELD)?)?;
    let state = the_state_in(answered);
    let how_many = answered
        .fields
        .text(asked::HOW_MANY_BOUNDARIES_REACHED_FIELD)
        .and_then(|text| text.parse::<usize>().ok())
        .unwrap_or(0);
    let mut reached = Vec::new();
    let mut the_clock = String::new();
    for at in 0..how_many {
        let field = |name: &str| {
            answered
                .fields
                .text(&asked::the_field_of_item(asked::A_BOUNDARY_REACHED, at, name))
        };
        let (Some(boundary), Some(did), Some(moment), Some(clock)) = (
            field(asked::THE_BOUNDARY_FIELD).and_then(the_boundary_named),
            field(asked::WHAT_THE_STEP_DID_FIELD),
            field(asked::THE_BOUNDARY_AT_FIELD).and_then(|text| text.parse::<u64>().ok()),
            field(asked::THE_BOUNDARY_CLOCK_FIELD),
        ) else {
            continue;
        };
        if the_clock.is_empty() {
            the_clock = clock.to_string();
        }
        reached.push(RecordedBoundary {
            boundary,
            what_the_step_did: if did == asked::IT_HAD_NOTHING_TO_DO {
                WhatTheStepDid::NothingToDo
            } else {
                WhatTheStepDid::Performed
            },
            at: std::time::UNIX_EPOCH + Duration::from_secs(moment),
            clock: smoozy_state::observed::Clock::AMachine(smoozy_state::observed::MachineClock(
                clock.to_string(),
            )),
        });
    }
    // **`Server Setup` S2a: a failed run's ending names the step that
    // failed**, and that step is the one the run was in when it ended — which
    // is what `Server Setup` S1 reads to tell a step that stopped part-way from
    // one never started, and what the panel puts the failure on. **Where the
    // ending names none** — a record written before it could — the step under
    // way is read as before. Added 2026-09-21.
    let the_failed_step = state.the_failed_step;
    // **The runner's ruling of 2026-09-27**: a refused restart's mark is read
    // as taken back, so the panel draws the restart as the step that did not
    // finish rather than as done.
    if the_restart_mark_was_taken_back(state.ending.as_deref(), the_failed_step) {
        reached.retain(|recorded| recorded.boundary != Boundary::RestartIssued);
    }
    // **Two spellings of the step under way, one per side of `Server Setup`
    // Y7's split**: a worker step as it was commanded, and an admin step the
    // desktop recorded as started, named by its first boundary
    // (`smoozy_framing`'s `THE_STEP_FIELD`). **Reading only the first dropped
    // the second**, and a run cut during the driver install read *not started*
    // at that step (`Server Setup` S1). Added 2026-09-29, step 10c's transfer
    // wave piece 12.
    let the_step_in_flight = the_failed_step.or_else(|| {
        state.the_step_in_flight.as_deref().and_then(|under_way| {
            the_boundary_a_step_reaches(under_way).or_else(|| the_boundary_named(under_way))
        })
    });
    let last = reached.last().map(|it: &RecordedBoundary| it.boundary);
    let ending = state.ending.as_deref().map(|said| {
        if said == asked::IT_COMPLETED {
            HowTheRunEnded::Completed
        } else if said == asked::IT_WAS_STOPPED {
            HowTheRunEnded::LeftStopped { at: last }
        } else if said == asked::IT_WAS_ABANDONED {
            // **`Server Setup` S6's abandon, with what its final verification
            // found** — words where it did not pass. Added 2026-09-22.
            HowTheRunEnded::Abandoned {
                what_the_check_found: answered
                    .fields
                    .text(asked::THE_MACHINES_OWN_WORDS_FIELD)
                    .map(|words| TheFailedStepsWords::SmoozysOwnVerification {
                        what_was_read: words.to_string(),
                    })
                    .unwrap_or(TheFailedStepsWords::None),
            }
        } else if said == asked::A_STEP_FAILED {
            HowTheRunEnded::AStepFailed {
                step: the_step_in_flight
                    .map(|boundary| boundary.its_step())
                    .unwrap_or(Step::Done),
                // **One string on the wire, and which stream it came from is
                // not carried**, so it is shown whole as the step's words and
                // nothing is claimed about the stream.
                words: answered
                    .fields
                    .text(asked::THE_MACHINES_OWN_WORDS_FIELD)
                    .map(|words| TheFailedStepsWords::SmoozysOwnVerification {
                        what_was_read: words.to_string(),
                    })
                    .unwrap_or(TheFailedStepsWords::None),
            }
        } else {
            HowTheRunEnded::EndedAtBoundaryNotCompleted { at: last }
        }
    });
    Some(TheSetupAsRead {
        // The operation record does not establish a machine-level hold.
        // `the_setup_as_read` adds the fresh health answer's reading.
        the_run_holds_the_machine: false,
        still_open: ending.is_none(),
        record: TheSetupRecord {
            kind,
            the_composed_list: state.composed,
            the_boundaries_reached: reached,
            ending,
        },
        the_step_in_flight,
        the_admin_step_recorded_as_started: state.the_step_in_flight.as_deref().and_then(the_boundary_named),
        the_clock,
    })
}

/// `Server Setup` Y6b's kinds, read back from the wire's own words.
fn the_kind_named(word: &str) -> Option<SetupKind> {
    [
        SetupKind::TheReadyAction,
        SetupKind::ABlenderInstall,
        SetupKind::ABlenderRemoval,
        SetupKind::AnAddOnInstall,
        SetupKind::TheTelegramPlacement,
    ]
    .into_iter()
    .find(|kind| the_word_for(*kind) == word)
}

/// **The Set up card's reading of the machine**, as the machine answered it —
/// `Server Setup` Y6b's ready-action run, `Server Setup` W3a's three answers
/// with `Server Setup` W4's line
/// and ask, and the Telegram role's own state. The 38's piece 6.
///
/// **Every part is the machine's and none is remembered** (`Server Setup` S2;
/// `Server Setup` Y6a: *"the desktop stores none of it"*). **A part the machine could not
/// read is `None`**, never an answer of *none*.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheCardsReading {
    /// **`Server Setup` Y6b: the ready action's retained run**, by its
    /// identity; `None` where the machine holds no record of that kind.
    pub the_ready_actions_run: Option<Identity>,
    /// **`Server Setup` W3a**, or `None` where the jobs could not be read.
    pub the_jobs: Option<TheJobsThatEnded>,
    /// **`Server Setup` Y6b's Telegram line: the role's desired state is
    /// *enabled***, or `None` where it could not be read.
    pub the_role_is_enabled: Option<bool>,
}

/// **`Server Setup` W3a's three answers, as the machine gave them.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheJobsThatEnded {
    /// The most recent job to end, by the machine's own clock — `None` where
    /// no job has ended there.
    pub the_most_recent: Option<String>,
    /// **The most recent job whose outcome is *finished* and which carries a
    /// device line** — the job, the line verbatim, and what it asked to render
    /// on where that could be read.
    pub the_finished_job_with_a_line: Option<(String, String, Option<DeviceAsked>)>,
}

/// **`Server Setup` W4's second condition, as the machine read it.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceAsked {
    TheCards,
    TheProcessor,
}

/// **The card's reading, read off the machine's answer.** `None` where the
/// machine does not answer the request at all — a program from before it,
/// which is what the card then draws without it.
pub fn the_cards_reading_as_answered(answered: &Answered) -> Option<TheCardsReading> {
    if answered.the_request_name_it_says_it_does_not_answer().is_some() {
        return None;
    }
    let fields = &answered.fields;
    let the_jobs = match fields.text(asked::THE_JOBS_THAT_ENDED_HERE_FIELD) {
        Some(asked::NO_JOB_HAS_ENDED_HERE) => Some(TheJobsThatEnded {
            the_most_recent: None,
            the_finished_job_with_a_line: None,
        }),
        Some(asked::SOME_JOBS_HAVE_ENDED_HERE) => Some(TheJobsThatEnded {
            the_most_recent: fields
                .text(asked::THE_MOST_RECENT_JOB_TO_END_FIELD)
                .map(str::to_string),
            the_finished_job_with_a_line: match (
                fields.text(asked::THE_FINISHED_JOB_WITH_A_DEVICE_LINE_FIELD),
                fields.text(asked::THE_DEVICE_LINE_FIELD),
            ) {
                (Some(job), Some(line)) if !line.is_empty() => Some((
                    job.to_string(),
                    line.to_string(),
                    match fields.text(asked::THE_DEVICE_ASKED_FOR_FIELD) {
                        Some(asked::IT_ASKED_FOR_THE_CARDS) => Some(DeviceAsked::TheCards),
                        Some(asked::IT_ASKED_FOR_THE_PROCESSOR) => Some(DeviceAsked::TheProcessor),
                        _ => None,
                    },
                )),
                _ => None,
            },
        }),
        _ => None,
    };
    Some(TheCardsReading {
        the_ready_actions_run: fields
            .text(asked::THE_READY_ACTIONS_RUN_FIELD)
            .and_then(|text| Identity::read_back(IdentityKind::Operation, text)),
        the_jobs,
        the_role_is_enabled: match fields.text(asked::THE_TELEGRAM_ROLES_STATE_FIELD) {
            Some(asked::THE_ROLE_IS_ENABLED) => Some(true),
            Some(asked::THE_ROLE_IS_DISABLED) => Some(false),
            _ => None,
        },
    })
}

/// **`Server Setup` G2a's reading: `Server Setup` Y6b's own, the role's half of it.**
pub async fn the_telegram_roles_reading<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    package_area: &str,
) -> Option<bool> {
    the_cards_reading(sessions, package_area).await?.the_role_is_enabled
}

/// **`Server Setup` G2a: whether the dedicated entry point is running on that
/// server or waiting its turn** — read from `Server Setup` Y9a's line, not kept.
pub fn the_telegram_placement_is_asked_for_on(server: &str) -> bool {
    the_kind_being_driven_on(server) == Some(SetupKind::TheTelegramPlacement)
        || the_acts_still_waiting_on(server)
            .iter()
            .any(|act| matches!(act, ASetUpAct::TheTelegramPlacement))
}

/// **The card's reading, asked of the machine over the connection that
/// stands.** `None` where it could not be asked or was not answered.
pub async fn the_cards_reading<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    package_area: &str,
) -> Option<TheCardsReading> {
    let Ok(identity) = Identity::issue(IdentityKind::Operation) else {
        return None;
    };
    let request = Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        identity,
        asked::THE_SET_UP_CARDS_READING_REQUEST,
    );
    let answered = ask(sessions, package_area, &request).await?;
    the_cards_reading_as_answered(&answered)
}

/// **`Server Setup` B8a's install records, as the machine answered them** —
/// each installed executable with the route that placed it
/// ([`asked::YOUR_BUILD`] or [`asked::FROM_THE_PUBLISHED_LISTING`]). Added
/// 2026-10-09, the 38's piece 14b. **`None` where nothing was read** — no
/// count, or a record with a hole in it — which is never *no record*.
pub fn the_install_records_as_answered(answered: &Answered) -> Option<Vec<(String, String)>> {
    let how_many: usize = answered
        .fields
        .text(asked::HOW_MANY_INSTALL_RECORDS_FIELD)?
        .parse()
        .ok()?;
    (0..how_many)
        .map(|at| {
            let field = |name| {
                answered
                    .fields
                    .text(&asked::the_field_of_item(asked::AN_INSTALL_RECORD, at, name))
                    .map(str::to_string)
            };
            Some((
                field(asked::THE_INSTALLED_EXECUTABLE_FIELD)?,
                field(asked::THE_INSTALL_ROUTE_FIELD)?,
            ))
        })
        .collect()
}

/// **The install records, asked of the machine over the connection that
/// stands** — read at connection, when the Blender table's search is taken.
pub async fn the_install_records<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    package_area: &str,
) -> Option<Vec<(String, String)>> {
    let identity = Identity::issue(IdentityKind::Operation).ok()?;
    let request = Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        identity,
        asked::THE_INSTALL_RECORDS_REQUEST,
    );
    let answered = ask(sessions, package_area, &request).await?;
    the_install_records_as_answered(&answered)
}

/// **What the Set up card is drawn from, read fresh on the connection that
/// stands**: the run, and the rest of the card's reading beside it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheCardAsRead {
    /// **The run the card shows**, read by [`the_setup_as_read`].
    pub the_run: Option<TheSetupAsRead>,
    /// **The rest of the machine the card reads**, where the machine
    /// answered the request.
    pub the_rest: Option<TheCardsReading>,
}

/// **The card, read: the run the standing state names, else the ready run
/// this copy drove, else the ready action's retained run the machine names**
/// — the last being what a copy opened after the setup has
/// (`Server Setup` Y6a, `Server Setup` Y6b; the 38's piece 6). **The rest of the card's reading rides
/// beside it either way.**
pub async fn the_card_as_read<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    package_area: &str,
    remembered: Option<Identity>,
) -> TheCardAsRead {
    let the_rest = the_cards_reading(sessions, package_area).await;
    let named = the_rest
        .as_ref()
        .and_then(|rest| rest.the_ready_actions_run.clone());
    let mut the_run = the_setup_as_read(sessions, package_area, remembered).await;
    if the_run.is_none() {
        if let Some(named) = named {
            the_run = the_state_of_one_run(sessions, package_area, &named).await;
        }
    }
    TheCardAsRead { the_run, the_rest }
}

/// **The Set up tab's reading of one connected server** — the ready action's
/// run, over that server's own point, and the rest of the card's reading
/// beside it. Added 2026-09-21; the card's reading 2026-10-03.
pub fn the_setup_on<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    point: &ThisServersSessions,
    server: &str,
) -> Option<TheCardAsRead> {
    use tauri::Manager;
    let started = app.try_state::<std::sync::Mutex<crate::start::Started>>()?;
    let record = crate::keys::TheRunsRecord {
        started: started.inner(),
    };
    let mut row = TheServersRowHere {
        record: &record,
        server: server.to_string(),
    };
    let the_row = row.the_row();
    let package_area = the_row.as_ref().and_then(the_package_area_of)?;
    // **`State, identity and recovery` Y10: the remembered run is
    // disqualified where the installation has
    // moved under it.** Step 10c's fifth pass on Vast: a second placement gave
    // the machine a new installation identity and the card went on reading the
    // run of the old one. A run identity names a record in one installation's
    // own store, so an identity from another installation is a value read under
    // an installation that is no longer there.
    let the_installation_now = the_row
        .as_ref()
        .and_then(|row| row.last_confirmed_installation_identity.clone());
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .ok()?;
    Some(runtime.block_on(the_card_as_read(
        point,
        &package_area,
        the_last_run_this_copy_drove(
            server,
            SetupKind::TheReadyAction,
            the_installation_now.as_deref(),
        ),
    )))
}

/// **Whether the machine holds no installation of Smoozy's at all**, read fresh
/// over the connection that stands — `Server protocol and transfers` C15's own
/// reading of the state area. Added 2026-09-21, for the second pass's fault 4.
///
/// **Why this is *never set up* and nothing weaker.** `Server Setup` Y6a
/// (Row 356): *has this server ever been set up* is the setup record's presence
/// and nothing else. **A setup record is opened by Smoozy's own program**, which
/// a placement puts on the machine, and `Rented-server runtime` G10 writes the
/// installation identity before the pointer ever names that program — so a
/// machine with no identity has never had a program, and so has never held a
/// setup record. **That is the reading a placement that failed before any run
/// leaves**, and the Set up card then draws what `Design/Server Setup § This
/// server` draws for it: *Not set up*, with the loud action.
///
/// **`true` only on the machine's own *nothing there***. A read that could not
/// be taken, a record that does not read back, a server with no root recorded to
/// look under, and an identity all answer `false`, and the card keeps saying
/// only what it knows.
pub fn the_machine_holds_no_installation<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    point: &ThisServersSessions,
    server: &str,
) -> bool {
    use tauri::Manager;
    let Some(started) = app.try_state::<std::sync::Mutex<crate::start::Started>>() else {
        return false;
    };
    let record = crate::keys::TheRunsRecord {
        started: started.inner(),
    };
    let mut row = TheServersRowHere {
        record: &record,
        server: server.to_string(),
    };
    let Some(package_area) = row.the_row().as_ref().and_then(the_package_area_of) else {
        return false;
    };
    let Ok(runtime) = tokio::runtime::Builder::new_current_thread().enable_all().build() else {
        return false;
    };
    runtime.block_on(the_state_area_holds_nothing(point, &package_area))
}

/// [`the_machine_holds_no_installation`]'s own reading, over any point — **so it
/// is driven at the desk.**
pub async fn the_state_area_holds_nothing<C: AHeldConnection>(
    point: &ThisServersSessions<C>,
    package_area: &str,
) -> bool {
    let area = package_area.as_bytes().to_vec();
    let read = point
        .a_reading(true, move |subsystem| {
            Box::pin(async move {
                Ok(smoozy_server_protocol::reconnection::read_the_installation_identity(
                    subsystem, &area,
                )
                .await)
            })
        })
        .await;
    matches!(
        read,
        Ok(Ok(Ok(smoozy_server_protocol::reconnection::WhatTheStateAreaHeld::Nothing)))
    )
}

/// **`State, identity and recovery` J6's server-package trust record, reached
/// through the spine's own lending** (L4).
///
/// **`crate::transfers`' own store is the same row and takes an open
/// connection**; this one takes the lending, because
/// `Server protocol and transfers` B9's admission reads the record, fetches
/// over the internet and writes it back — and the lock must not be held across
/// the fetch.
pub struct TheTrustRecordThroughTheSpine<'a> {
    pub record: &'a dyn crate::keys::LendsTheRecord,
}

impl smoozy_server_protocol::release::trust::RemembersTheServerPackageTrustRecord
    for TheTrustRecordThroughTheSpine<'_>
{
    fn read_it(
        &self,
    ) -> Result<Option<smoozy_server_protocol::release::trust::TheTrustRecord>, String> {
        let mut read = Err("Could not read Smoozy's database.".to_string());
        self.record.lend(&mut |connection| {
            read = crate::record::server_package_trust::read(connection)
                .map_err(|failure| format!("{failure:?}"));
        });
        read
    }

    fn write_it(
        &mut self,
        record: &smoozy_server_protocol::release::trust::TheTrustRecord,
    ) -> Result<(), String> {
        let mut written = Err("Could not write to Smoozy's database.".to_string());
        self.record.lend(&mut |connection| {
            written = crate::record::server_package_trust::write(connection, record)
                .map_err(|failure| format!("{failure:?}"));
        });
        written
    }

    fn discard_it(&mut self) -> Result<(), String> {
        let mut discarded = Err("Could not write to Smoozy's database.".to_string());
        self.record.lend(&mut |connection| {
            discarded = crate::record::server_package_trust::discard(connection)
                .map_err(|failure| format!("{failure:?}"));
        });
        discarded
    }
}

/// **`Servers` Q1 step 8: a remembered setup, carried out.**
///
/// **It runs on a connect carrying one and on no other**, and it runs exactly
/// once: `Servers` B5 item 1 cleared the bit at `Servers` B2's instant, so
/// there is nothing left to run a second time.
///
/// **Which options were ticked is what the add wrote down** — `Servers` B1 —
/// and the pair is the Add dialog's own: *get the server ready* and *install
/// the latest stable Blender*. **A tick of either is the ready action**,
/// because `Server Setup` R1 composes one run and takes a Blender with it
/// where the user asked for one; two ticks are not two runs
/// (`Server Setup` Y9).
pub fn the_remembered_setup_is_carried_out<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    server: String,
    options: Vec<String>,
) {
    if options.is_empty() {
        return;
    }
    // **`Server Setup` Y9's exclusion, taken before anything is sent.** A tick
    // of either option is the ready action, so that is the kind. **And
    // `Server Setup` Y9a's line goes with it**, as at *Set up again*: the ready
    // action replaces the installs an earlier Smoozy closed before it began.
    // Changed 2026-10-16, step 10c's owed piece 5: until then this took the
    // run by `a_run_begins` alone and the line outlived the setup.
    let taken = the_remembered_setup_is_taken(&server, &mut |waiting| {
        crate::surface::set_up::the_installs_waiting_are_kept(&app, &server, waiting);
    });
    if !taken {
        return;
    }
    // **P5: it runs off the drawing thread**, on a thread of its own, and the
    // Set up tab is told what it came to.
    //
    // **The tab and the server row are told the run was taken, and told its
    // ending only once the run is let go** — the Creator's notes piece 3,
    // 2026-10-02. *Add and set up* lands on the Set up tab, which draws the
    // run on its card from the moment it is taken (on a fresh machine the
    // whole placement comes before the run's first step), and the row reads
    // *Setting up* while it runs. Until then the ending was told before
    // [`a_run_ends`], so anything asked at the ending still found the run.
    crate::surface::set_up::the_run_under_way_is_told(&app, &server);
    std::thread::spawn(move || {
        use tauri::Manager;
        let point = app
            .try_state::<std::sync::Arc<crate::servers::a_period::ThePeriodsThisRun>>()
            .and_then(|periods| periods.the_sessions_of(&server));
        let came_to = point.map(|point| match the_blender_the_add_asked_for(&options) {
            Ok(install_this_blender) => the_ready_action_on(&app, &point, &server, install_this_blender),
            Err(stopped) => stopped,
        });
        // **`Server Setup` Y9a item 6: what the card asked for meanwhile
        // waited its turn behind this run**, and begins where it reached
        // *done* without restarting the server (item 7). Changed 2026-10-27,
        // the nineteenth notes, note 2: until then the run was let go by
        // `a_run_ends` alone.
        let restarted = the_run_restarted_the_server(&server);
        let finished = matches!(
            came_to,
            Some(WhereTheSetUpGot::TheRunEnded { ending: TheRunsEnding::ItReachedDone { .. }, .. })
        );
        let follows = a_run_in_the_line_ends(&server, finished && !restarted);
        crate::surface::set_up::the_run_is_told_what_it_came_to(&app, &server, came_to.as_ref(), follows, restarted);
    });
}

/// **`Servers` Q1 step 8's run, taken: the ready action, by the one way a
/// press on the Set up card takes it** ([`a_run_is_taken_from_the_card`]), so
/// the installs an earlier Smoozy closed before it began are replaced with
/// nothing, as *Set up again* replaces them (`Server Setup` Y9a). `keep` is the
/// local record's write. False where `Server Setup` Y9 refused the run, and
/// then nothing was kept. Added 2026-10-16, step 10c's owed piece 5.
pub fn the_remembered_setup_is_taken(server: &str, keep: &mut dyn FnMut(&[String])) -> bool {
    a_run_is_taken_from_the_card(server, SetupKind::TheReadyAction, Vec::new(), keep)
}

/// **The Add dialog's *Install the latest stable Blender*, turned into the one
/// version `Server Setup` B3 names.** Added 2026-09-21: until then the tick was
/// carried out as the ready action with no Blender at all.
///
/// **`Server Setup` B3: *the latest finished release* is the highest stable
/// version across tracks**, read off the shop-front's own listing
/// (`Server Setup` B2) — `Server Setup`'s own `blender::the_latest_finished_release`,
/// never a version written into this build and never a long-term-support line
/// preferred for being one. **Only the version travels**; `Server Setup` B4
/// derives the archive's address and its checksum's from it, on the machine,
/// over the machine's own line.
///
/// **A listing that cannot be read starts nothing** — `Server Setup` B5: Smoozy
/// never quietly substitutes a different version — and says so, rather than
/// running the rest of the setup and leaving the tick silently undone.
pub fn the_blender_the_add_asked_for(options: &[String]) -> Result<Option<String>, WhereTheSetUpGot> {
    let asked = options
        .iter()
        .any(|one| one == crate::surface::servers::INSTALL_THE_LATEST_STABLE_BLENDER);
    if !asked {
        return Ok(None);
    }
    let read = match tokio::runtime::Builder::new_current_thread().enable_all().build() {
        Ok(runtime) => runtime.block_on(smoozy_server_setup::listing::the_list_smoozy_offers()),
        Err(_) => {
            return Err(WhereTheSetUpGot::TheBlenderListCouldNotBeRead {
                said: "Could not start reading Blender's download list.".to_string(),
            })
        }
    };
    match read {
        Ok(listing) => the_latest_finished_release_in(&listing.channels)
            .map(Some)
            .ok_or(WhereTheSetUpGot::TheBlenderListCouldNotBeRead {
                said: "Blender's download list has no stable release.".to_string(),
            }),
        Err(why) => Err(WhereTheSetUpGot::TheBlenderListCouldNotBeRead {
            said: crate::surface::set_up::why_the_list_could_not_be_read(&why),
        }),
    }
}

/// **`Server Setup` B3 over a listing already read**, as the version's own text.
pub fn the_latest_finished_release_in(
    channels: &[smoozy_server_setup::blender::PublishedChannel],
) -> Option<String> {
    smoozy_server_setup::blender::the_latest_finished_release(channels)
        .map(|channel| {
            let version = channel.version;
            format!("{}.{}.{}", version.major, version.minor, version.patch)
        })
}

/// **`Server Setup` E2's offer, made on the Set up tab.** Added 2026-09-21.
///
/// **The box is `Design/Server Setup § Admin access`'s**, and it names the step
/// that was refused; the answer comes back through
/// [`the_offer_is_answered`] when the user presses *Try again* or *Cancel*.
/// **Nothing is held here but the way back**: the password travels once, from
/// the box to the run that asked, and is never written anywhere on the way.
pub struct TheSetUpTabAsks<R: tauri::Runtime> {
    pub app: tauri::AppHandle<R>,
    pub server: String,
}

impl<R: tauri::Runtime> AsksForThePassword for TheSetUpTabAsks<R> {
    fn the_cut_run_is_being_waited_for(&mut self, waiting: bool) {
        crate::surface::set_up::the_wait_for_the_cut_run_is_kept(&self.server, waiting);
        crate::surface::set_up::the_run_under_way_is_told(&self.app, &self.server);
    }

    fn the_password_for(&mut self, step: Step) -> ItsOwnFuture<'_, Option<Origin>> {
        let (answer, answered) = tokio::sync::oneshot::channel();
        if let Ok(mut standing) = the_offers_standing().lock() {
            // **One offer per server.** An earlier one still standing has
            // nobody waiting on it, and dropping its way back answers it
            // *declined*.
            standing.retain(|(server, _)| server != &self.server);
            standing.push((self.server.clone(), answer));
        }
        crate::surface::set_up::the_offer_is_made(&self.app, &self.server, step);
        Box::pin(async move { answered.await.ok().flatten() })
    }

    fn a_run_stands(&mut self, kind: SetupKind, run: &Identity) {
        // **The installation the row confirms now is kept with the run** — see
        // [`ARunThisCopyDrove`]. Nothing where the record will not read, which
        // is nothing to disqualify against rather than a mismatch.
        a_run_this_copy_drove(
            &self.server,
            kind,
            run,
            the_installation_confirmed_for(&self.app, &self.server),
        );
    }

    fn the_runs_this_copy_drove_here(&self) -> Vec<Identity> {
        the_runs_this_copy_drove_on(
            &self.server,
            the_installation_confirmed_for(&self.app, &self.server).as_deref(),
        )
    }

    /// **Kept for the panel and told to the tab** — the tab asks for the panel
    /// when it is told, and draws it. Added 2026-09-27, fix piece 9.
    fn the_run_is_as_read(&mut self, read: &TheSetupAsRead) {
        if the_run_is_read_under_way(&self.server, read) {
            crate::surface::set_up::the_run_under_way_is_told(&self.app, &self.server);
        }
    }

    /// **`Server Setup` B7a's question, drawn on the card** (`su-low`): held
    /// here and the run panel told, which the page asks for and draws.
    fn the_answer_below_the_floor(&mut self, version: &str) -> ItsOwnFuture<'_, bool> {
        let answered = the_question_is_asked(&self.server, version);
        crate::surface::set_up::the_run_under_way_is_told(&self.app, &self.server);
        Box::pin(async move { answered.await.unwrap_or(false) })
    }

    /// **Kept for the card and told to the tab, piece by piece** —
    /// `Server Setup` Y7a; the tab asks for the run's panel when it is told,
    /// and the window's lines ride it.
    fn where_the_words_go(&mut self) -> Option<TheWordsGoHere> {
        let app = self.app.clone();
        let server = self.server.clone();
        Some(the_words_go_to_the_card(&self.server, move || {
            crate::surface::set_up::the_servers_words_are_told(&app, &server);
        }))
    }

    fn where_the_swaps_words_go(&mut self) -> Option<TheWordsGoHere> {
        Some(the_swaps_words_go_apart(&self.server))
    }

    /// **Kept for the card and told to the tab as it moves** —
    /// `Server Setup` Y7b and `Server Setup` Y7c; the tab asks for the run's panel when it is
    /// told, and the step's figure rides it.
    fn where_the_figure_goes(&mut self, the_build: Option<&str>) -> Option<TheFigureGoesHere> {
        let app = self.app.clone();
        let server = self.server.clone();
        Some(the_figure_goes_to_the_card(&self.server, the_build, move || {
            crate::surface::set_up::the_run_under_way_is_told(&app, &server);
        }))
    }
}

/// **`Server Setup` Y7a: where a run's words go for the Set up card** — this
/// run's words begun on `server`, the last run's let go, and each piece kept
/// there with `told` called once it is, which is the tab being told. **The
/// Set up tab's own route** ([`TheSetUpTabAsks`]), written apart so the desk
/// drives the same keeping without a window. Added 2026-10-18, the Creator's
/// fourteenth notes, piece S.
///
/// **It empties nothing** (changed 2026-10-30, the twenty-third notes, note
/// 3): the act the user asked for already let the last act's words go
/// ([`a_runs_words_begin`]), so a run taken up again, or the next act in a
/// line, adds to the words kept; a record is begun only where none is.
pub fn the_words_go_to_the_card(server: &str, told: impl Fn() + Send + Sync + 'static) -> TheWordsGoHere {
    the_words_are_kept_on(the_words_kept(), server);
    let server = server.to_string();
    std::sync::Arc::new(move |step: Step, printed: &WhatTheServerPrinted| {
        let kept = match printed {
            WhatTheServerPrinted::APiece(piece) => a_piece_is_kept(&server, step, piece),
            WhatTheServerPrinted::SomeOfItCouldNotBeBroughtBack => a_mark_is_kept(&server, step),
        };
        if kept {
            told();
        }
    })
}

/// **`Server Setup` R1b's swap's words, kept apart** — each swap's begun from
/// nothing, and the setup's words never touched. Added 2026-10-30, the
/// twenty-third notes, note 3.
pub fn the_swaps_words_go_apart(server: &str) -> TheWordsGoHere {
    if let Ok(mut kept) = the_swaps_words_kept().lock() {
        kept.retain(|(one, _)| one != server);
        kept.push((server.to_string(), TheRunsWords::default()));
    }
    let server = server.to_string();
    std::sync::Arc::new(move |step: Step, printed: &WhatTheServerPrinted| {
        kept_in(the_swaps_words_kept(), &server, |words| match printed {
            WhatTheServerPrinted::APiece(piece) => words.a_piece_is_printed(step, piece),
            WhatTheServerPrinted::SomeOfItCouldNotBeBroughtBack => words.its_words_were_not_kept(step),
        });
    })
}

/// **A run this copy drove, and the installation it was driven under.**
///
/// **The installation identity is here because of `State, identity and
/// recovery` §2.3 and `State, identity and recovery` Y10**: a machine-derived
/// value read under an installation
/// identity that is now absent or different is disqualified. A run identity is
/// exactly such a value — it names a record in *that* installation's own store
/// — and step 10c's fifth pass on Vast watched the Set up card go on reading a
/// run of the installation a second placement had replaced. **`None` is *no
/// installation was confirmed when the run was told*,** which is nothing to
/// disqualify against rather than a mismatch.
/// **The installation identity the server's row confirms now**, or nothing
/// where the record will not read or none is confirmed.
fn the_installation_confirmed_for<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    server: &str,
) -> Option<String> {
    use tauri::Manager;
    let started = app.try_state::<std::sync::Mutex<crate::start::Started>>()?;
    let record = crate::keys::TheRunsRecord {
        started: started.inner(),
    };
    let mut row = TheServersRowHere {
        record: &record,
        server: server.to_string(),
    };
    row.the_row()?.last_confirmed_installation_identity.clone()
}

type ARunThisCopyDrove = (String, SetupKind, Identity, Option<String>);

/// **The last setup run of each kind this copy drove on each server** — the
/// identity and nothing else. Added 2026-09-21.
///
/// **Run-held and never durable**: `Server Setup` S2 gives *what it managed*
/// no second home on the desktop, and this is not one — it holds no boundary,
/// no ending and no words, only which run to ask the machine about. **What is
/// shown is always the machine's own record, read fresh** (`Server Setup` S1).
/// **What it cannot do, said plainly**: a copy opened after the run was driven
/// holds no identity for it, so a finished setup from an earlier sitting is not
/// found this way — see [`the_setup_as_read`].
fn the_runs_this_copy_drove() -> &'static std::sync::Mutex<Vec<ARunThisCopyDrove>> {
    static HELD: std::sync::OnceLock<std::sync::Mutex<Vec<ARunThisCopyDrove>>> =
        std::sync::OnceLock::new();
    HELD.get_or_init(|| std::sync::Mutex::new(Vec::new()))
}

/// A run this copy drove, kept as the latest of its kind on that server, with
/// the installation identity confirmed for that server when it was told.
pub fn a_run_this_copy_drove(
    server: &str,
    kind: SetupKind,
    run: &Identity,
    under: Option<String>,
) {
    if let Ok(mut held) = the_runs_this_copy_drove().lock() {
        held.retain(|(one, its_kind, _, _)| !(one == server && *its_kind == kind));
        held.push((server.to_string(), kind, run.clone(), under));
    }
}

/// **`State, identity and recovery` Y10's gate over one remembered run.** A run told under an installation the
/// machine no longer answers under is disqualified; a run told under none is
/// nothing to disqualify.
fn it_still_stands_under(under: Option<&str>, the_installation_now: Option<&str>) -> bool {
    match under {
        None => true,
        Some(under) => the_installation_now == Some(under),
    }
}

/// **Every setup run this copy drove on that server, most recent first** —
/// one per kind, which is all this list ever holds. Added 2026-09-22.
///
/// **The installation the machine answers under now is handed in**, and a run
/// from another installation is not returned
/// (`State, identity and recovery` Y10).
/// **[`the_runs_this_copy_drove_on`], under the installation the row confirms
/// now**, for a caller holding the application rather than a run's asker.
pub fn the_runs_this_copy_drove_on_the_confirmed_installation<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    server: &str,
) -> Vec<Identity> {
    the_runs_this_copy_drove_on(server, the_installation_confirmed_for(app, server).as_deref())
}

pub fn the_runs_this_copy_drove_on(
    server: &str,
    the_installation_now: Option<&str>,
) -> Vec<Identity> {
    the_runs_this_copy_drove()
        .lock()
        .ok()
        .map(|held| {
            held.iter()
                .rev()
                .filter(|(one, _, _, under)| {
                    one == server && it_still_stands_under(under.as_deref(), the_installation_now)
                })
                .map(|(_, _, run, _)| run.clone())
                .collect()
        })
        .unwrap_or_default()
}

/// The latest run of that kind this copy drove on that server, where there is
/// one **and the machine still answers under the installation it was told
/// under** (`State, identity and recovery` Y10).
pub fn the_last_run_this_copy_drove(
    server: &str,
    kind: SetupKind,
    the_installation_now: Option<&str>,
) -> Option<Identity> {
    the_runs_this_copy_drove().lock().ok().and_then(|held| {
        held.iter()
            .find(|(one, its_kind, _, under)| {
                one == server
                    && *its_kind == kind
                    && it_still_stands_under(under.as_deref(), the_installation_now)
            })
            .map(|(_, _, run, _)| run.clone())
    })
}

type TheWayBack = tokio::sync::oneshot::Sender<Option<Origin>>;

/// The offers standing now, one per server, each with its way back. **Run-held
/// and never durable**, and it holds no password: a password passes through
/// once, on its way to the run that asked.
fn the_offers_standing() -> &'static std::sync::Mutex<Vec<(String, TheWayBack)>> {
    static STANDING: std::sync::OnceLock<std::sync::Mutex<Vec<(String, TheWayBack)>>> =
        std::sync::OnceLock::new();
    STANDING.get_or_init(|| std::sync::Mutex::new(Vec::new()))
}

/// **The offer, answered** — a password, or nothing where the user declined.
/// **Nothing where no offer stands for that server**, and it says so: the box
/// was left open after the run it asked for ended.
pub fn the_offer_is_answered(server: &str, answer: Option<Origin>) -> bool {
    let way_back = the_offers_standing().lock().ok().and_then(|mut standing| {
        let at = standing.iter().position(|(one, _)| one == server)?;
        Some(standing.remove(at).1)
    });
    match way_back {
        Some(way_back) => way_back.send(answer).is_ok(),
        None => false,
    }
}

/// **`Server Setup` B7a's questions standing, one per server**: the version
/// asked about and the way back to the run that waits on the answer. Added
/// 2026-10-09, the 38's piece 14b. **Run-held and never durable**: a question
/// lives exactly as long as the run that asked it.
type TheQuestionsWayBack = tokio::sync::oneshot::Sender<bool>;
fn the_questions_standing() -> &'static std::sync::Mutex<Vec<(String, String, TheQuestionsWayBack)>> {
    static STANDING: std::sync::OnceLock<std::sync::Mutex<Vec<(String, String, TheQuestionsWayBack)>>> =
        std::sync::OnceLock::new();
    STANDING.get_or_init(|| std::sync::Mutex::new(Vec::new()))
}

/// **`Server Setup` B7a's question, put to the user for `server`** — held
/// until answered, and the run panel is told so the card draws it (`su-low`).
pub fn the_question_is_asked(server: &str, version: &str) -> tokio::sync::oneshot::Receiver<bool> {
    let (answer, answered) = tokio::sync::oneshot::channel();
    if let Ok(mut standing) = the_questions_standing().lock() {
        // **One per server**: an earlier one has nobody waiting on it.
        standing.retain(|(one, _, _)| one != server);
        standing.push((server.to_string(), version.to_string(), answer));
    }
    answered
}

/// **The version the question standing on `server` asks about**, if one does.
pub fn the_question_standing_on(server: &str) -> Option<String> {
    the_questions_standing()
        .lock()
        .ok()?
        .iter()
        .find(|(one, _, _)| one == server)
        .map(|(_, version, _)| version.clone())
}

/// **`Server Setup` B7a's question, answered: *Install anyway* (true) or the way out
/// (false).** False where no question stands for that server.
pub fn the_question_is_answered(server: &str, install_anyway: bool) -> bool {
    let way_back = the_questions_standing().lock().ok().and_then(|mut standing| {
        let at = standing.iter().position(|(one, _, _)| one == server)?;
        Some(standing.remove(at).2)
    });
    way_back.is_some_and(|way_back| way_back.send(install_anyway).is_ok())
}

/// **`Server Setup` B7's send, through the application's ordinary upload
/// route** (`src/uploading.rs`), so it shows as an ordinary row on Transfers
/// (the runner's ruling on piece 14). Added 2026-10-09, the 38's piece 14b.
///
/// **The archive must land as `<the run>-<its name>`**, which is where the
/// machine reads it, and the upload route lands a file under its own name. So
/// the route is handed a link of that name to the user's file, in a folder of
/// this copy's own under the system's temporary folder, taken away when the
/// send ends: a hard link, else a symbolic one, else a copy. **The user's own
/// folder is never written to.**
pub struct TheUploadRoute<R: tauri::Runtime> {
    pub app: tauri::AppHandle<R>,
    pub point: ThisServersSessions,
    pub server: String,
    pub chosen: std::path::PathBuf,
}

impl<R: tauri::Runtime> SendsTheUsersArchiveUp for TheUploadRoute<R> {
    fn the_archive_goes_up(
        &mut self,
        staging_area: String,
        landing: String,
    ) -> ItsOwnFuture<'_, Result<(), String>> {
        Box::pin(async move {
            let folder = std::env::temp_dir().join(format!("smoozy-build-{landing}"));
            std::fs::create_dir_all(&folder)
                .map_err(|why| format!("Could not prepare the upload on this computer: {why}."))?;
            let link = folder.join(&landing);
            let made = std::fs::hard_link(&self.chosen, &link)
                .or_else(|_| a_symbolic_link(&self.chosen, &link))
                .or_else(|_| std::fs::copy(&self.chosen, &link).map(|_| ()));
            let went = match made {
                Ok(()) => self.the_link_goes_up(staging_area, link.clone()).await,
                Err(why) => Err(format!("Could not prepare the upload on this computer: {why}.")),
            };
            let _ = std::fs::remove_file(&link);
            let _ = std::fs::remove_dir(&folder);
            went
        })
    }
}

#[cfg(unix)]
fn a_symbolic_link(to: &std::path::Path, at: &std::path::Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(to, at)
}

#[cfg(windows)]
fn a_symbolic_link(to: &std::path::Path, at: &std::path::Path) -> std::io::Result<()> {
    std::os::windows::fs::symlink_file(to, at)
}

impl<R: tauri::Runtime> TheUploadRoute<R> {
    async fn the_link_goes_up(&self, staging_area: String, link: std::path::PathBuf) -> Result<(), String> {
        use tauri::Manager;
        let not_started = || "The upload could not start.".to_string();
        let (row, run) = {
            let started = self
                .app
                .try_state::<std::sync::Mutex<crate::start::Started>>()
                .ok_or_else(not_started)?;
            let started = started.lock().map_err(|_| not_started())?;
            let rows = crate::record::server_rows::all(
                started.record.as_ref().ok_or_else(not_started)?.connection(),
            )
            .map_err(|_| not_started())?;
            let row = rows
                .into_iter()
                .find(|row| row.local_server_identifier == self.server)
                .ok_or_else(not_started)?;
            (row, started.run.clone().ok_or_else(not_started)?)
        };
        let server_row = Identity::read_back(IdentityKind::LocalServer, &row.local_server_identifier)
            .ok_or_else(not_started)?;
        let asked = crate::uploading::WhatIsBeingUploaded {
            server_row,
            installation: row
                .last_confirmed_installation_identity
                .as_deref()
                .and_then(|text| Identity::read_back(IdentityKind::Installation, text)),
            destination: staging_area.into_bytes(),
            chosen: vec![link],
            run,
        };
        let door = crate::surface::render_settings::the_applications_record(self.app.clone());
        let mut ready = crate::uploading::an_upload_is_prepared(&asked, &self.point, &door)
            .await
            .map_err(|_| not_started())?;
        // **The Transfers row names the user's own file**, not the landing
        // name the machine reads it under. Added 2026-10-16, step 10c's owed
        // piece 5.
        if let Some(name) = self.chosen.file_name() {
            a_build_goes_up(&ready.request.record.as_text(), &name.to_string_lossy());
        }
        if crate::uploading::the_answer_is_applied(&mut ready, None, &self.point, &door).await
            != crate::uploading::WhatTheAnswerLeft::ItGoes
        {
            return Err(not_started());
        }
        let the_run = self
            .app
            .try_state::<std::sync::Arc<crate::downloading::TheRunsTransfers>>()
            .ok_or_else(not_started)?;
        let request = ready.request.record.clone();
        let door = std::sync::Arc::new(door);
        let came_to = crate::uploading::the_upload_runs(
            ready,
            &self.point,
            the_run.inner(),
            door.clone(),
            &NothingNeighboursABuild,
        )
        .await;
        // The source link goes when this send ends. Nothing still waiting
        // under this request can succeed by being resumed afterwards.
        let mut saved = Err("The upload's result could not be saved.".to_string());
        crate::uploading::TheRecordAnUploadWrites::with_the_record(door.as_ref(), &mut |connection| {
            saved = crate::record::upload_requests::a_build_send_has_ended(connection, &request)
                .map_err(|_| "The upload's result could not be saved.".to_string());
        });
        the_run.every_leftover_is_read_again();
        crate::uploading::TheRecordAnUploadWrites::a_transfer_moved(door.as_ref());
        saved?;
        the_upload_landed(&came_to)
    }
}

/// **The uploads this run sent as a user's own Blender build, each with the
/// user's own file name** — so the Transfers row is named after the file the
/// user picked (`Design/Server Setup § Pointing at a build on your own
/// computer`), and never after the landing name, `<the run>-<its name>`, that
/// the machine reads it under, or the staging folder it went into. Added
/// 2026-10-16, step 10c's owed piece 5.
///
/// **Held for this run only**, as the dependency check's sends are
/// (`crate::the_found_files_sent`): the upload's own record carries no name of
/// the user's, so a later sitting's row reads as any upload's does.
fn the_builds_going_up() -> &'static std::sync::Mutex<std::collections::HashMap<String, String>> {
    static HELD: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<String, String>>> =
        std::sync::OnceLock::new();
    HELD.get_or_init(Default::default)
}

/// **One upload request, held as the user's own build going up**, by its
/// record's identity and the file name the user picked.
pub fn a_build_goes_up(request: &str, file: &str) {
    if let Ok(mut held) = the_builds_going_up().lock() {
        held.insert(request.to_string(), file.to_string());
    }
}

/// **The user's own file name, where this upload request sent a Blender
/// build** this run — nothing for every other upload.
pub fn the_build_this_upload_sends(request: &str) -> Option<String> {
    the_builds_going_up().lock().ok()?.get(request).cloned()
}

/// **Whether the one file landed**, and the upload's own words where it did
/// not. Added 2026-10-09, the 38's piece 14b.
pub fn the_upload_landed(came_to: &crate::uploading::WhatTheUploadCameTo) -> Result<(), String> {
    if came_to.uploaded == 1 && came_to.failed.is_empty() && came_to.refused.is_empty() {
        return Ok(());
    }
    Err(match came_to.refused.first() {
        Some((_, said)) => format!("The upload was refused: {}", a_sentence(said)),
        None if came_to.did_not_finish > 0 => "The upload did not finish.".to_string(),
        None => "The upload failed.".to_string(),
    })
}

/// **Words that close as a sentence**, so a reason after *The upload was
/// refused:* never runs into what follows it. Added 2026-10-28, step 13's
/// words sweep.
fn a_sentence(said: &str) -> String {
    let said = said.trim_end();
    if said.ends_with(|last: char| matches!(last, '.' | '!' | '?')) {
        said.to_string()
    } else {
        format!("{said}.")
    }
}

/// **A Blender archive in staging is nobody's neighbour**: `File Browser`
/// X7a's writes are about a project's files, and staging holds none.
struct NothingNeighboursABuild;

impl crate::uploading::WhenAFileIsPublished for NothingNeighboursABuild {
    fn a_file_was_published(&self, _landing_path: &[u8], _outcome: &smoozy_state::outcomes::UnitOutcome) {}
}

/// **The user's own build, installed on one connected server** — `Server
/// Setup` B7, `Server Setup` B7a and `Server Setup` B8 as a run of
/// `Server Setup` Y6b's *a Blender install*, driven by
/// [`the_users_build_is_installed`]. Added 2026-10-09, the 38's piece 14b.
/// **It runs off the drawing thread** and blocks the thread it is given, as
/// [`a_blender_install_on`] does.
pub fn the_users_build_install_on<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    point: &ThisServersSessions,
    server: &str,
    chosen: std::path::PathBuf,
) -> WhereTheSetUpGot {
    use tauri::Manager;
    let Some(archive) = chosen.file_name().and_then(|name| name.to_str()).map(str::to_string) else {
        return WhereTheSetUpGot::ThePlacementCouldNotBeCarriedOut {
            said: "The chosen file's name could not be read.".to_string(),
        };
    };
    let the_period = tauri::Manager::try_state::<
        std::sync::Arc<crate::servers::a_period::ThePeriodsThisRun>,
    >(app)
        .and_then(|periods| periods.the_restart_mark_of(server));
    let Some(started) = app.try_state::<std::sync::Mutex<crate::start::Started>>() else {
        return WhereTheSetUpGot::TheServerIsNotSetUpYet;
    };
    let record = crate::keys::TheRunsRecord {
        started: started.inner(),
    };
    let mut row = TheServersRowHere {
        record: &record,
        server: server.to_string(),
    };
    let runtime = match tokio::runtime::Builder::new_current_thread().enable_all().build() {
        Ok(runtime) => runtime,
        Err(_) => {
            return WhereTheSetUpGot::ThePlacementCouldNotBeCarriedOut {
                said: "Could not start the task.".to_string(),
            }
        }
    };
    let mut taking = TheOpeningReadingsOverThePoint { sessions: point };
    let mut sender = TheUploadRoute {
        app: app.clone(),
        point: point.clone(),
        server: server.to_string(),
        chosen,
    };
    runtime.block_on(the_users_build_is_installed(
        point,
        &mut taking,
        &mut row,
        archive,
        &mut sender,
        HowElevationIsObtained::APasswordIsOwedAndNoneIsHeld,
        &mut TheSetUpTabAsks {
            app: app.clone(),
            server: server.to_string(),
        },
        &TheStopThisServerWasAsked {
            server: server.to_string(),
        },
        the_period,
        // **`Server Setup` §12's act line, through the module's own diary.**
        // Added 2026-10-16, step 10c's owed piece 5.
        &mut |line| the_act_is_in_the_diary(app, server, line),
    ))
}

/// **A Blender install picked in the Blender section: `Server Setup` Y6b's own
/// kind of run, on one connected server.** Added 2026-09-21: until then the
/// control answered that the act was not built.
///
/// **It runs off the drawing thread** (P5) and blocks the thread it is given,
/// which is what the ready action's own act already does.
pub fn a_blender_install_on<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    point: &ThisServersSessions,
    server: &str,
    version: String,
) -> WhereTheSetUpGot {
    use tauri::Manager;
    let the_period = tauri::Manager::try_state::<
        std::sync::Arc<crate::servers::a_period::ThePeriodsThisRun>,
    >(app)
        .and_then(|periods| periods.the_restart_mark_of(server));
    let Some(started) = app.try_state::<std::sync::Mutex<crate::start::Started>>() else {
        return WhereTheSetUpGot::TheServerIsNotSetUpYet;
    };
    let record = crate::keys::TheRunsRecord {
        started: started.inner(),
    };
    let mut row = TheServersRowHere {
        record: &record,
        server: server.to_string(),
    };
    let runtime = match tokio::runtime::Builder::new_current_thread().enable_all().build() {
        Ok(runtime) => runtime,
        Err(_) => {
            return WhereTheSetUpGot::ThePlacementCouldNotBeCarriedOut {
                said: "Could not start the task.".to_string(),
            }
        }
    };
    let mut taking = TheOpeningReadingsOverThePoint { sessions: point };
    // **The version as asked**: the version, and for an unfinished build the
    // build service's bare file name beside it (`Server Setup` B4, 2026-10-02).
    let asked_for = smoozy_server_setup::blender::TheVersionAsked::read(&version).map(|it| it.version);
    let came_to = runtime.block_on(a_blender_is_installed(
        point,
        &mut taking,
        &mut row,
        version,
        HowElevationIsObtained::APasswordIsOwedAndNoneIsHeld,
        &mut TheSetUpTabAsks {
            app: app.clone(),
            server: server.to_string(),
        },
        &TheStopThisServerWasAsked {
            server: server.to_string(),
        },
        the_period,
    ));
    // **`Server Setup` §12's *a Blender was installed*, one per install asked
    // for.** Added 2026-09-22.
    //
    // **The version is the one the run was commanded with**, and the line is
    // written only where the run reached *done* having published a build —
    // which is the one reading this side has of `Server Setup` B1's answer in
    // staging. **An install that did not publish writes no act line**: §12's
    // form names the version the build answered, and an install that never got
    // that answer has nothing to name. What such a run left is what the failure
    // sentence already said.
    let published = matches!(
        &came_to,
        WhereTheSetUpGot::TheRunEnded {
            ending: TheRunsEnding::ItReachedDone { a_blender_was_published: true, .. },
            ..
        }
    );
    if let (true, Some(answered)) = (published, asked_for) {
        the_act_is_in_the_diary(
            app,
            server,
            smoozy_server_setup::diary::ActLine::ABlenderWasInstalled {
                answered,
                route: smoozy_server_setup::blender::HowTheBuildGotHere::ThePublishedListing,
                published: true,
            },
        );
    }
    came_to
}

/// **A Blender install, driven: `Server Setup` Y6b's kind, composed of R1's
/// Blender half alone.** Added 2026-09-21.
///
/// **The order**: `Server Setup` M1 and `Server Setup` M1a's readings, which every setup run
/// takes afresh (`Rented-server runtime` K2) and nothing else if either
/// refuses; `Server Setup` M3's probe; the package area this copy recorded at
/// the hand-over (`Job files and outputs` L1), without which there is no program
/// on the machine to open a run with; `Server Setup` R1 item 1's readings; and
/// the run, opened with `Server Setup` Y6b's *a Blender install* and driven
/// exactly as the ready action's is.
///
/// **No placement is performed and no driver, restart or Telegram step is
/// composed** ([`ready_action::compose_a_blender_install`]). **So a Blender
/// install never restarts the machine**, and on a machine that already holds
/// the unpack tool it is `Server Setup` Y1's additive run and sets no
/// machine-level state at all.
#[allow(clippy::too_many_arguments)]
pub async fn a_blender_is_installed<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    taking: &mut dyn TakesTheOpeningReadings,
    row: &mut dyn KeepsTheServersRow,
    version: String,
    elevation: HowElevationIsObtained<'_>,
    asker: &mut dyn AsksForThePassword,
    stop: &dyn AsksToStop,
    the_period: Option<TheRestartMark>,
) -> WhereTheSetUpGot {
    let said = match the_opening_readings(taking).await {
        Ok(said) => said,
        Err(stopped) => return stopped,
    };
    // **A program placed by a setup is what the row's confirmed installation
    // says, and a recorded root alone is not it.** `Server Setup` R2 records the
    // root at the hand-over, before the placement, so a setup whose placement
    // then failed leaves a root and no program — and the second pass on a real
    // rental, 2026-09-21, saw this guard pass on exactly that row and the run
    // then answer *"would not start the setup"* on a server with nothing to
    // start one. **The installation identity is written only once a placement
    // is verified** (`State, identity and recovery` §2.3), or read at a
    // connection from a machine that holds one, so it is the reading that says
    // a program is there.
    let row_now = row.the_row();
    let a_program_was_placed = row_now
        .as_ref()
        .is_some_and(|row| row.last_confirmed_installation_identity.is_some());
    let Some(package_area) = row_now
        .as_ref()
        .filter(|_| a_program_was_placed)
        .and_then(the_package_area_of)
    else {
        return WhereTheSetUpGot::TheServerIsNotSetUpYet;
    };
    let words = asker.where_the_words_go();
    let elevation = the_elevation_this_login_has_telling(sessions, elevation, words.as_ref()).await;
    let asked_for = WhatWasAskedFor {
        install_this_blender: Some(version),
        telegram_is_on: false,
        ..WhatWasAskedFor::default()
    };
    let needs = what_this_machine_needs(sessions, &said, &asked_for, false, words.as_ref()).await;
    let mut run = match TheRunBeingDriven::opened(
        sessions,
        &package_area,
        SetupKind::ABlenderInstall,
        ready_action::compose_a_blender_install(),
        needs,
        the_period,
        // **A Blender install never takes a stopped run over.** The press names
        // a version, and carrying on a run that stopped installing another one
        // would install something nobody asked for. *Set up again* is the
        // control `Design/Server Setup § Cut off part-way` gives that job.
        Vec::new(),
        stop,
        asker,
    )
    .await
    {
        Ok(run) => run,
        Err(ending) => {
            return WhereTheSetUpGot::TheRunEnded {
                at: Boundary::Started,
                ending,
            }
        }
    };
    run.the_words_go = words;
    run.driven(elevation, &asked_for, &mut TelegramIsNotOn, asker, stop)
        .await
}

/// **`Server Setup` B7's send: the user's archive goes up into the open run's
/// own staging area** — a seam, because the upload is the application's
/// ordinary upload route and shows as an ordinary row on Transfers (the
/// runner's ruling on piece 14). Added 2026-10-09, the 38's piece 14b.
///
/// **`landing` is the whole file name it must arrive under** —
/// `<the run>-<the archive's bare name>`, which is where the machine reads it
/// (`Server Setup — machine`'s `where_the_users_archive_is_staged`).
/// **What comes back is whether every byte landed**, or the words saying why
/// not.
pub trait SendsTheUsersArchiveUp {
    fn the_archive_goes_up(
        &mut self,
        staging_area: String,
        landing: String,
    ) -> ItsOwnFuture<'_, Result<(), String>>;
}

/// **`Server Setup` B7, `Server Setup` B7a and `Server Setup` B8: the user's
/// own build installed** — the
/// same kind of run as a Blender picked from the list (`Server Setup` Y6b's
/// *a Blender install*), with the fetch left out. Added 2026-10-09, the 38's
/// piece 14b.
///
/// **The order**: the run is opened first, its state is read for its staging
/// area, the archive goes up there (the runner's ruling), and the run is then
/// driven with the archive's bare name on the unpack and the publication and
/// no version anywhere. **Below 4.5 the publication is held for a person**,
/// and the screen asks (`AsksForThePassword::the_answer_below_the_floor`).
/// **An upload that does not land fails the run at *tooling***, in the
/// upload's own words, so the run is never left open with nothing in staging.
///
/// **`diary` is handed `Server Setup` §12's *a Blender was installed***, once,
/// where the run reached *done* having published the build or having
/// discarded it at the user's decline — naming the version the build
/// answered, *a build the user pointed at*, and which of the two. A run that
/// ended any other way never had that answer settled, and writes no act line;
/// its sentence on the card says what it left. Added 2026-10-16, step 10c's
/// owed piece 5.
#[allow(clippy::too_many_arguments)]
pub async fn the_users_build_is_installed<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    taking: &mut dyn TakesTheOpeningReadings,
    row: &mut dyn KeepsTheServersRow,
    archive: String,
    sender: &mut dyn SendsTheUsersArchiveUp,
    elevation: HowElevationIsObtained<'_>,
    asker: &mut dyn AsksForThePassword,
    stop: &dyn AsksToStop,
    the_period: Option<TheRestartMark>,
    diary: &mut dyn FnMut(smoozy_server_setup::diary::ActLine),
) -> WhereTheSetUpGot {
    let said = match the_opening_readings(taking).await {
        Ok(said) => said,
        Err(stopped) => return stopped,
    };
    let row_now = row.the_row();
    let a_program_was_placed = row_now
        .as_ref()
        .is_some_and(|row| row.last_confirmed_installation_identity.is_some());
    let Some(package_area) = row_now
        .as_ref()
        .filter(|_| a_program_was_placed)
        .and_then(the_package_area_of)
    else {
        return WhereTheSetUpGot::TheServerIsNotSetUpYet;
    };
    let words = asker.where_the_words_go();
    let elevation = the_elevation_this_login_has_telling(sessions, elevation, words.as_ref()).await;
    let asked_for = WhatWasAskedFor {
        install_this_blender: None,
        telegram_is_on: false,
        ..WhatWasAskedFor::default()
    };
    let mut needs = what_this_machine_needs(sessions, &said, &asked_for, false, words.as_ref()).await;
    // **A build arrives in this run**, so the library chain is read against it
    // exactly as against a fetched one.
    needs.this_run_fetches_a_blender = true;
    let steps: Vec<Step> = ready_action::compose_a_blender_install()
        .into_iter()
        .filter(|step| *step != Step::Fetch)
        .collect();
    let mut run = match TheRunBeingDriven::opened(
        sessions,
        &package_area,
        SetupKind::ABlenderInstall,
        steps,
        needs,
        the_period,
        Vec::new(),
        stop,
        asker,
    )
    .await
    {
        Ok(run) => run,
        Err(ending) => {
            return WhereTheSetUpGot::TheRunEnded {
                at: Boundary::Started,
                ending,
            }
        }
    };
    let staging = run
        .the_runs_state()
        .await
        .and_then(|state| state.the_staging_area);
    let went_up = match staging {
        Some(staging) => {
            sender
                .the_archive_goes_up(staging, format!("{}-{archive}", run.run.as_text()))
                .await
        }
        None => Err("The server did not say where to upload the build.".to_string()),
    };
    if let Err(said) = went_up {
        // Nothing has reached an install step: close at the opening boundary,
        // rather than recording an unpack-tool failure for work never tried.
        let recorded = crate::surface::set_up::what_a_failed_build_send_says(&said);
        run.the_failed_step_is_recorded(Step::Readings, &recorded).await;
        return WhereTheSetUpGot::TheRunEnded {
            at: run.where_it_stands().await,
            ending: TheRunsEnding::TheBuildCouldNotBeSent { said },
        };
    }
    run.the_users_own_archive = Some(archive);
    run.the_words_go = words;
    let came_to = run
        .driven(elevation, &asked_for, &mut TelegramIsNotOn, asker, stop)
        .await;
    if let Some(line) = the_users_build_line(
        &came_to,
        run.the_version_answered.as_deref(),
        run.the_users_answer == Some(asked::THE_USER_DECLINED),
    ) {
        diary(line);
    }
    came_to
}

/// **`Server Setup` §12's *a Blender was installed*, for a build the user
/// pointed at** — where the run reached *done* having published it, or having
/// discarded it at the user's decline, and the version it answered is known.
/// Nothing for any other ending. Added 2026-10-16, step 10c's owed piece 5.
pub fn the_users_build_line(
    came_to: &WhereTheSetUpGot,
    answered: Option<&str>,
    declined: bool,
) -> Option<smoozy_server_setup::diary::ActLine> {
    let WhereTheSetUpGot::TheRunEnded {
        ending: TheRunsEnding::ItReachedDone { a_blender_was_published, .. },
        ..
    } = came_to
    else {
        return None;
    };
    let published = *a_blender_was_published;
    if !published && !declined {
        return None;
    }
    let answered = smoozy_server_setup::blender::Version::parse(answered?)?;
    Some(smoozy_server_setup::diary::ActLine::ABlenderWasInstalled {
        answered,
        route: smoozy_server_setup::blender::HowTheBuildGotHere::ABuildTheUserPointedAt,
        published,
    })
}


// ════════════════════════════════════════════════════════════════════════════
// What this file is held to
// ════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// **A recovery that was not carried on is read in all three of its
    /// wordings**, because a server keeps the words it was written with, and
    /// a machine's words in any other shape are not it. Added 2026-11-06, the
    /// check after the last fix.
    #[test]
    fn the_recovery_not_carried_on_is_read_in_every_wording() {
        for words in [
            THE_RECOVERY_WAS_NOT_CARRIED_ON,
            THE_RECOVERY_WAS_NOT_CARRIED_ON_UNTIL_2026_11_06,
            THE_RECOVERY_WAS_NOT_CARRIED_ON_EARLIER,
        ] {
            assert!(the_recovery_was_not_carried_on(words), "{words}");
        }
        assert!(!the_recovery_was_not_carried_on("The GPU driver did not respond after the restart."));
        assert!(!THE_RECOVERY_WAS_NOT_CARRIED_ON.contains('\''), "shown verbatim, so no straight apostrophe");
    }

    /// **Step 10c's owed piece 12: `Server Setup` §12's *a Blender was taken
    /// off*, in the diary's fixed form, for a removal that reached *done*** —
    /// naming the version and the route: the machine's own software manager
    /// where the removal named a package, the folder deletion where it named
    /// none. **A removal that ended any other way writes nothing**, and nor
    /// does one whose build never answered a version.
    #[test]
    fn a_removal_that_reached_done_is_one_taken_off_line_naming_its_route() {
        let done = WhereTheSetUpGot::TheRunEnded {
            at: Boundary::Done,
            ending: TheRunsEnding::ItReachedDone {
                a_blender_was_published: false,
                the_driver_missed_the_optix_bar: false,
                the_swap_waits: false,
            },
        };
        let removal = |version: &str, package: Option<&'static str>| TheRemovalAsked {
            executable: "/snap/bin/blender".to_string(),
            the_answered_version: version.to_string(),
            covered: Vec::new(),
            the_managers_package: package,
        };
        assert_eq!(
            the_removal_line(&done, &removal("5.2.0", Some("blender"))).map(|line| line.text()),
            Some("a Blender was taken off\t5.2.0\tthe machine's own software manager".to_string())
        );
        assert_eq!(
            the_removal_line(&done, &removal("4.5.9", None)).map(|line| line.text()),
            Some("a Blender was taken off\t4.5.9\tthe folder deletion".to_string())
        );
        assert_eq!(the_removal_line(&done, &removal("", None)), None, "a line with no version");
        let failed = WhereTheSetUpGot::TheRunEnded {
            at: Boundary::Started,
            ending: TheRunsEnding::AStepFailed {
                step: Step::BlenderRemoval,
                said: "the server could not remove it".to_string(),
            },
        };
        assert_eq!(the_removal_line(&failed, &removal("5.2.0", None)), None);
        assert_eq!(the_removal_line(&WhereTheSetUpGot::TheServerIsNotSetUpYet, &removal("5.2.0", None)), None);
    }

    /// **`Servers` H11b's second case, kept: a run that reaches *done* leaves
    /// its own answer about the bar, and a run that ends any other way leaves
    /// none** — so a miss recorded earlier stands until the next run that
    /// reaches *done* (the runner's ruling of 2026-10-11). The fourth piece of
    /// the OptiX and driver work.
    #[test]
    fn only_a_run_that_reached_done_says_whether_the_driver_missed_the_bar() {
        let done = |missed: bool| WhereTheSetUpGot::TheRunEnded {
            at: Boundary::Done,
            ending: TheRunsEnding::ItReachedDone {
                a_blender_was_published: false,
                the_driver_missed_the_optix_bar: missed,
                the_swap_waits: false,
            },
        };
        assert_eq!(the_miss_this_run_leaves(&done(true)), Some(true));
        assert_eq!(the_miss_this_run_leaves(&done(false)), Some(false));
        let failed = WhereTheSetUpGot::TheRunEnded {
            at: Boundary::DriverPlaced,
            ending: TheRunsEnding::AStepFailed {
                step: Step::Driver,
                said: "the driver step failed".to_string(),
            },
        };
        assert_eq!(the_miss_this_run_leaves(&failed), None);
        assert_eq!(the_miss_this_run_leaves(&WhereTheSetUpGot::TheRentalWouldNotNameItself), None);

        // **And every ready action keeps it**, by either of its two callers,
        // because both go through the one function that drives it — before the
        // ending is told.
        let working = include_str!("setting_up.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("the working half");
        let the_ready_action = working
            .split("pub fn the_ready_action_on<R: tauri::Runtime>(")
            .nth(1)
            .and_then(|rest| rest.split("\n}\n").next())
            .expect("the ready action");
        assert!(the_ready_action.contains("the_driver_miss_is_kept(&record, server, &came_to);"));
    }

    /// **The 38's piece 14b: the install records, read as the machine answers
    /// them** — and a list with a hole in it, or no count, is nothing read.
    #[test]
    fn the_install_records_are_read_whole_or_not_at_all() {
        let answered = |fields: Fields| {
            let request = Request::speaking(
                THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
                Identity::issue(IdentityKind::Operation).expect("an identity"),
                asked::THE_INSTALL_RECORDS_REQUEST,
            );
            Answered::to(&request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "0.1.0").carrying(fields)
        };
        let item = |at, name| asked::the_field_of_item(asked::AN_INSTALL_RECORD, at, name);
        let whole = Fields::none()
            .and(asked::HOW_MANY_INSTALL_RECORDS_FIELD, "1")
            .and(&item(0, asked::THE_INSTALLED_EXECUTABLE_FIELD), "/s/blender/blender")
            .and(&item(0, asked::THE_INSTALL_ROUTE_FIELD), asked::YOUR_BUILD);
        assert_eq!(
            the_install_records_as_answered(&answered(whole)),
            Some(vec![("/s/blender/blender".to_string(), asked::YOUR_BUILD.to_string())])
        );
        let holed = Fields::none().and(asked::HOW_MANY_INSTALL_RECORDS_FIELD, "1");
        assert_eq!(the_install_records_as_answered(&answered(holed)), None);
        assert_eq!(the_install_records_as_answered(&answered(Fields::none())), None);
    }
    use smoozy_server_setup::driving::WhoPerformsTheStep;

    /// **`Server Setup` Y9a: *Add and set up*'s ready action clears the line
    /// naming installs an earlier Smoozy closed before it began, as *Set up
    /// again* does** — by the same one way a card press takes its run — and a
    /// refused one keeps it. Step 10c's owed piece 5, 2026-10-16: until then
    /// the add's own route took the run by `a_run_begins` alone and the line
    /// outlived the setup.
    #[test]
    fn add_and_set_up_clears_the_installs_smoozy_closed_before_as_set_up_again_does() {
        use crate::record::installs_not_begun::{the_ones_waiting_are_kept, the_ones_waiting_on};
        let folder = std::env::temp_dir().join(format!("smoozy-y9a-add-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a folder for the record");
        let record = crate::record::LocalRecord::open_at(&folder.join("local-record.sqlite3")).expect("the record opens");
        let c = record.connection();
        let server = "y9a-add-and-set-up";
        the_ones_waiting_are_kept(c, server, &["5.2.2".to_string(), "4.5.14".to_string()]).expect("kept");
        assert!(crate::surface::set_up::the_installs_smoozy_closed_on(c, server).is_some(), "no line to clear");

        // **A run already going refuses the add's**, and the line stands.
        assert!(a_run_begins(server, SetupKind::ABlenderInstall));
        assert!(!the_remembered_setup_is_taken(server, &mut |waiting| {
            the_ones_waiting_are_kept(c, server, waiting).expect("kept")
        }));
        a_run_ends(server);
        assert_eq!(the_ones_waiting_on(c, server).expect("read").len(), 2, "a refused add cleared the line");

        assert!(the_remembered_setup_is_taken(server, &mut |waiting| {
            the_ones_waiting_are_kept(c, server, waiting).expect("kept")
        }));
        assert_eq!(the_kind_being_driven_on(server), Some(SetupKind::TheReadyAction));
        a_run_ends(server);
        assert!(the_ones_waiting_on(c, server).expect("read").is_empty(), "the add left the list");
        assert_eq!(crate::surface::set_up::the_installs_smoozy_closed_on(c, server), None, "the add left the line");

        // **And the add's own route is the one that does it.**
        let working = include_str!("setting_up.rs").split("#[cfg(test)]").next().expect("the working half");
        let the_add = working
            .split("pub fn the_remembered_setup_is_carried_out<R: tauri::Runtime>(")
            .nth(1)
            .and_then(|rest| rest.split("\n}\n").next())
            .expect("the add's route");
        assert!(the_add.contains("let taken = the_remembered_setup_is_taken(&server,"), "the add takes its run another way");
        assert!(!the_add.contains("a_run_begins("), "the add still takes its run by a_run_begins alone");
        drop(record);
        let _ = std::fs::remove_dir_all(&folder);
    }

    /// **Several Blenders picked: the next begins only where the last one
    /// installed its Blender and nobody asked to stop, and the exclusion passes
    /// from one to the next without being let go** — `Server Setup` Y9 kept
    /// (the Creator's third notes, note 2, piece H; the line of
    /// [`ASetUpAct`]s from 2026-10-27).
    #[test]
    fn the_next_install_begins_only_after_one_that_installed_and_no_stop() {
        let server = "piece-h-the-handover";
        let install = |version: &str| ASetUpAct::AnInstallFromTheList(version.to_string());
        assert!(a_run_begins(server, SetupKind::ABlenderInstall));
        the_installs_wait_on(server, vec!["4.5.14".to_string(), "4.2.9".to_string()]);
        assert_eq!(a_run_in_the_line_ends(server, true), WhatFollowsARun::TheNextBegins(install("4.5.14")));
        assert!(!a_run_begins(server, SetupKind::TheReadyAction), "nothing slips between two installs");
        the_user_asked_to_stop(server);
        assert_eq!(a_run_in_the_line_ends(server, true), WhatFollowsARun::TheseWereNotBegun(vec![install("4.2.9")]));
        assert_eq!(the_kind_being_driven_on(server), None);

        assert!(a_run_begins(server, SetupKind::ABlenderInstall));
        the_installs_wait_on(server, vec!["4.5.14".to_string()]);
        assert_eq!(a_run_in_the_line_ends(server, false), WhatFollowsARun::TheseWereNotBegun(vec![install("4.5.14")]));
        assert!(a_run_begins(server, SetupKind::ABlenderInstall));
        assert_eq!(a_run_in_the_line_ends(server, true), WhatFollowsARun::NothingWaits);
        assert_eq!(the_kind_being_driven_on(server), None);
    }

    /// **`Server Setup` Y9a item 6: a removal pressed while an install runs
    /// joins the line instead of being refused, and a second install pressed
    /// after it joins behind it** — each the next act, of its own kind, when
    /// the one before it finishes. The local record is handed the whole line
    /// at each press. The nineteenth notes, note 2: a removal pressed during an
    /// install was refused and its line wiped.
    #[test]
    fn a_removal_and_a_second_install_pressed_during_an_install_wait_their_turn() {
        let server = "note-2-a-removal-waits";
        let removal = ASetUpAct::ARemoval(TheRemovalAsked {
            executable: "/root/smoozy/blender/5.2.2/blender".to_string(),
            the_answered_version: "5.2.2".to_string(),
            covered: Vec::new(),
            the_managers_package: None,
        });
        let install = |version: &str| ASetUpAct::AnInstallFromTheList(version.to_string());
        let mut kept = Vec::new();

        assert_eq!(
            the_acts_are_taken_from_the_card(server, vec![install("5.2.3")], &mut |line| kept.push(line.to_vec())),
            HowThePressWasTaken::ItBegan
        );
        assert_eq!(the_kind_being_driven_on(server), Some(SetupKind::ABlenderInstall));
        assert_eq!(
            the_acts_are_taken_from_the_card(server, vec![removal.clone()], &mut |line| kept.push(line.to_vec())),
            HowThePressWasTaken::ItWaitsItsTurn,
            "the removal was refused"
        );
        assert_eq!(
            the_acts_are_taken_from_the_card(server, vec![install("4.5.15")], &mut |line| kept.push(line.to_vec())),
            HowThePressWasTaken::ItWaitsItsTurn
        );
        assert_eq!(the_acts_still_waiting_on(server), vec![removal.clone(), install("4.5.15")]);
        assert_eq!(
            kept,
            vec![vec![], vec![removal.clone()], vec![removal.clone(), install("4.5.15")]],
            "the record was not handed the line as each press left it"
        );
        assert_eq!(the_kind_being_driven_on(server), Some(SetupKind::ABlenderInstall), "the press took the run");

        // **The install finishes: the removal is next, as a removal.**
        assert_eq!(a_run_in_the_line_ends(server, true), WhatFollowsARun::TheNextBegins(removal));
        assert_eq!(the_kind_being_driven_on(server), Some(SetupKind::ABlenderRemoval));
        assert_eq!(the_acts_still_waiting_on(server), vec![install("4.5.15")]);
        assert_eq!(a_run_in_the_line_ends(server, true), WhatFollowsARun::TheNextBegins(install("4.5.15")));
        assert_eq!(the_kind_being_driven_on(server), Some(SetupKind::ABlenderInstall));
        assert_eq!(a_run_in_the_line_ends(server, true), WhatFollowsARun::NothingWaits);
        assert_eq!(the_kind_being_driven_on(server), None);

        // **Several picked at once where nothing runs: the first begins, the
        // rest wait, and the record is handed the rest.**
        kept.clear();
        assert_eq!(
            the_acts_are_taken_from_the_card(server, vec![install("5.2.3"), install("4.5.15")], &mut |line| kept
                .push(line.to_vec())),
            HowThePressWasTaken::ItBegan
        );
        assert_eq!(kept, vec![vec![install("4.5.15")]]);
        a_run_ends(server);
        let _ = the_line_stopped_on(server);
    }

    /// **`Server Setup` Y9a item 7: a run that issued a restart is read as one,
    /// from the boundaries its driving last read**, so the line stops behind
    /// it; and item 8: a run that lets its exclusion go without moving the
    /// line leaves what waited for its ending to name, once.
    #[test]
    fn a_restart_is_read_off_the_run_and_a_plain_ending_keeps_the_stopped_line() {
        use smoozy_server_setup::run::{RecordedBoundary, TheSetupRecord};
        let server = "note-2-a-restart";
        let reached = |boundary| RecordedBoundary {
            boundary,
            what_the_step_did: WhatTheStepDid::Performed,
            at: std::time::SystemTime::UNIX_EPOCH,
            clock: smoozy_state::observed::Clock::AMachine(smoozy_state::observed::MachineClock("a clock".into())),
        };
        let read = |boundaries: Vec<Boundary>| TheSetupAsRead {
            the_run_holds_the_machine: true,
            record: TheSetupRecord {
                kind: SetupKind::TheReadyAction,
                the_composed_list: boundaries.clone(),
                the_boundaries_reached: boundaries.into_iter().map(reached).collect(),
                ending: None,
            },
            still_open: true,
            the_step_in_flight: None,
            the_admin_step_recorded_as_started: None,
            the_clock: "a clock".to_string(),
        };
        assert!(!the_run_restarted_the_server(server), "no run, no restart");
        assert!(a_run_begins(server, SetupKind::TheReadyAction));
        assert!(the_run_is_read_under_way(server, &read(vec![Boundary::Started, Boundary::DriverPlaced])));
        assert!(!the_run_restarted_the_server(server));
        assert!(the_run_is_read_under_way(
            server,
            &read(vec![Boundary::Started, Boundary::DriverPlaced, Boundary::RestartIssued, Boundary::CardAnswering])
        ));
        assert!(the_run_restarted_the_server(server));

        // **`Server Setup` R1b's swap run ends by the plain ending**, and what joined during
        // it is kept for its ending to name.
        let telegram = ASetUpAct::TheTelegramPlacement;
        assert_eq!(
            the_acts_are_taken_from_the_card(server, vec![telegram.clone()], &mut |_| {}),
            HowThePressWasTaken::ItWaitsItsTurn
        );
        a_run_ends(server);
        assert_eq!(the_kind_being_driven_on(server), None);
        assert_eq!(the_line_stopped_on(server), vec![telegram]);
        assert!(the_line_stopped_on(server).is_empty(), "named twice");
    }

    /// **`Telegram` L3's derivation, read for one server: what a connection
    /// owes it** — the 38's piece 11, hole 14 as the map leans.
    ///
    /// - **Told nothing, or told only under an earlier installation: nothing.**
    ///   It was never given Telegram there, and *Install* is its route.
    /// - **Off: the switch alone**, never the files.
    /// - **On again after off, everything else current: the switch alone.**
    /// - **A renamed server: the files**, because the label rides the role
    ///   configuration.
    #[test]
    fn what_a_connection_owes_a_server_is_l3_read_for_it_and_nothing_for_one_never_given_it() {
        use smoozy_servers::identity::{Host, KeyReference, Port, Username};
        use smoozy_telegram::proof::{KeepsTheProvedBot, TheBot, TheConversation};
        use smoozy_telegram::told::WhatSmoozyTells;
        let folder = std::env::temp_dir().join(format!("smoozy-owed-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a folder for the record");
        let record = crate::record::LocalRecord::open_at(&folder.join("local-record.sqlite3"))
            .expect("the record opens");
        let c = record.connection();
        let mut row = ServerRow::saved(
            Identity::issue(IdentityKind::LocalServer).expect("randomness").as_text(),
            Host::what_the_user_typed("5.180.183.77").expect("a host"),
            Port::what_the_user_typed(22).expect("a port"),
            Username::what_the_user_typed("root").expect("a username"),
            KeyReference { local_key_identifier: "key-1".into(), fingerprint: "SHA256:aaa".into() },
        );
        row.label = Some("render box".into());
        row.last_confirmed_installation_identity = Some("i2".into());
        crate::record::server_rows::add(c, &row).expect("the row is added");
        crate::record::server_rows::write(c, &row).expect("the row is written");
        let server = row.local_server_identifier.clone();
        crate::record::telegram_bot::TheProvedBotRecord::over(c)
            .the_proof_was_recorded(&TheBot { id: 7, username: "smoozy_a7k2_bot".into() }, TheConversation(9001))
            .expect("the proof is recorded");
        crate::record::settings::telegram_was_switched(c, true).expect("the switch is written");
        let owed = || the_owed_reading(c, &server).expect("the record answers");
        let tell = |value: WhatSmoozyTells, under: &str| {
            crate::record::told_record::the_telling_was_acknowledged(
                c,
                &server,
                &value,
                std::time::SystemTime::UNIX_EPOCH,
                under,
            )
            .expect("the entry is written")
        };

        assert!(owed().is_nothing(), "a server told nothing was never given Telegram");
        tell(WhatSmoozyTells::the_roles_desired_state(true), "i1");
        assert!(owed().is_nothing(), "told only under an earlier installation");

        // Everything Settings tells now, under the installation it holds now.
        let settings = crate::record::settings::the_telegram_settings(c).expect("the settings read");
        let now = crate::surface::telegram::what_smoozy_tells_now(c, &settings, Some(TheConversation(9001)))
            .expect("the current values");
        for value in now {
            tell(value, "i2");
        }
        tell(WhatSmoozyTells::the_label(Some("render box")), "i2");
        assert!(owed().is_nothing(), "told everything");

        crate::record::settings::telegram_was_switched(c, false).expect("the switch is written");
        assert_eq!(owed(), WhatThisServerIsOwed { the_files: false, the_desired_state: Some(false) });
        tell(WhatSmoozyTells::the_roles_desired_state(false), "i2");
        assert!(owed().is_nothing(), "told off");

        crate::record::settings::telegram_was_switched(c, true).expect("the switch is written");
        assert_eq!(owed(), WhatThisServerIsOwed { the_files: false, the_desired_state: Some(true) });
        tell(WhatSmoozyTells::the_roles_desired_state(true), "i2");

        row.label = Some("renamed".into());
        crate::record::server_rows::write(c, &row).expect("the row is written");
        assert_eq!(owed(), WhatThisServerIsOwed { the_files: true, the_desired_state: None });
        crate::record::settings::telegram_was_switched(c, false).expect("the switch is written");
        assert_eq!(
            owed(),
            WhatThisServerIsOwed { the_files: false, the_desired_state: Some(false) },
            "renamed while off: the switch alone"
        );
        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    /// **The card's reading, read off the machine's answer, part by part** —
    /// and a part the machine left off is `None`, never *none*; a program that
    /// does not answer the request gives no reading at all. The 38's piece 6.
    #[test]
    fn the_cards_reading_is_read_part_by_part_and_a_missing_part_is_not_none() {
        let request = Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            Identity::issue(IdentityKind::Operation).expect("randomness"),
            asked::THE_SET_UP_CARDS_READING_REQUEST,
        );
        let run = Identity::issue(IdentityKind::Operation).expect("randomness");
        let answered = |fields: Fields| Answered::to(&request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "0").carrying(fields);
        let whole = the_cards_reading_as_answered(&answered(
            Fields::none()
                .and(asked::THE_READY_ACTIONS_RUN_FIELD, run.as_text())
                .and(asked::THE_JOBS_THAT_ENDED_HERE_FIELD, asked::SOME_JOBS_HAVE_ENDED_HERE)
                .and(asked::THE_MOST_RECENT_JOB_TO_END_FIELD, "job-2")
                .and(asked::THE_FINISHED_JOB_WITH_A_DEVICE_LINE_FIELD, "job-1")
                .and(asked::THE_DEVICE_LINE_FIELD, "Path tracing on: a processor (CPU) (8 threads) [CPU]")
                .and(asked::THE_DEVICE_ASKED_FOR_FIELD, asked::IT_ASKED_FOR_THE_CARDS)
                .and(asked::THE_TELEGRAM_ROLES_STATE_FIELD, asked::THE_ROLE_IS_ENABLED),
        ))
        .expect("a reading");
        assert_eq!(whole.the_ready_actions_run, Some(run));
        assert_eq!(whole.the_role_is_enabled, Some(true));
        assert_eq!(
            whole.the_jobs,
            Some(TheJobsThatEnded {
                the_most_recent: Some("job-2".to_string()),
                the_finished_job_with_a_line: Some((
                    "job-1".to_string(),
                    "Path tracing on: a processor (CPU) (8 threads) [CPU]".to_string(),
                    Some(DeviceAsked::TheCards),
                )),
            })
        );

        let parts_left_off = the_cards_reading_as_answered(&answered(
            Fields::none().and(asked::THE_MACHINES_OWN_WORDS_FIELD, "a store would not read"),
        ))
        .expect("a reading");
        assert_eq!(parts_left_off.the_ready_actions_run, None);
        assert_eq!(parts_left_off.the_jobs, None, "jobs not read are not *none ended*");
        assert_eq!(parts_left_off.the_role_is_enabled, None);

        let none_ended = the_cards_reading_as_answered(&answered(
            Fields::none().and(asked::THE_JOBS_THAT_ENDED_HERE_FIELD, asked::NO_JOB_HAS_ENDED_HERE),
        ))
        .expect("a reading");
        assert_eq!(
            none_ended.the_jobs,
            Some(TheJobsThatEnded { the_most_recent: None, the_finished_job_with_a_line: None })
        );

        let an_older_program =
            Answered::to(&request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "0")
                .saying_it_does_not_answer_that_request(&request.name);
        assert_eq!(the_cards_reading_as_answered(&an_older_program), None);
    }

    /// **`Server Setup` Y7's own judgement decides which side drives a step,
    /// and this file never decides it a second time.**
    ///
    /// **Two of `Server Setup` R1's steps the judgement gives the setup worker
    /// cannot be performed there**, which is what step 10c piece 8 found and
    /// refused by name on the machine's side, so this file takes both. **They
    /// are named here rather than left as a difference nobody meets.**
    #[test]
    fn which_side_drives_a_step_is_server_setups_and_the_two_exceptions_are_named() {
        for step in Step::IN_ORDER {
            let the_judgement = driving::who_performs(step);
            let this_file_runs_an_elevated_command = the_elevated_step_for(
                step,
                DistributionFamily::Ubuntu,
            )
            .is_some();
            match step {
                // **The two the worker cannot perform.** The readings' two
                // command strings are declared in a crate the machine's
                // program may never reach, and the Telegram material is a
                // built desktop act.
                Step::Readings | Step::TelegramMaterial => {
                    assert_eq!(the_judgement, WhoPerformsTheStep::TheSetupWorker);
                    assert!(
                        !this_file_runs_an_elevated_command,
                        "{step:?} is not an elevated command"
                    );
                }
                // **`Server Setup` R1 item 4's install is elevated and is
                // owed** — no request carries the not-found list the
                // selection needs.
                Step::Libraries => {
                    assert_eq!(
                        the_judgement,
                        WhoPerformsTheStep::TheCommandChannelUnderElevation
                    );
                    assert!(!this_file_runs_an_elevated_command);
                }
                _ => assert_eq!(
                    this_file_runs_an_elevated_command,
                    the_judgement == WhoPerformsTheStep::TheCommandChannelUnderElevation,
                    "{step:?} is driven against `Server Setup`'s own judgement"
                ),
            }
        }
    }

    /// **`Server Setup` Y6b's five kinds and no sixth**, held to
    /// `smoozy_framing`'s own list in both directions.
    ///
    /// **A recovery run is not a sixth kind.** `Server Setup` S4 item 2's
    /// take-over gives the recovery the taken-over run's kind, and
    /// `Server Setup` Y6b's
    /// per-kind retention rests on that.
    #[test]
    fn the_setup_kinds_are_the_five_and_each_has_one_spelling() {
        let said: Vec<String> = SetupKind::ALL.into_iter().map(the_word_for).collect();
        assert_eq!(said.len(), asked::EVERY_KIND_OF_SETUP.len());
        for word in asked::EVERY_KIND_OF_SETUP {
            assert!(
                said.iter().any(|named| named == word),
                "{word} is a kind of setup this file cannot name"
            );
        }
    }

    /// **`Server Setup` Y4's ten boundaries, each with one spelling**, read
    /// back the way the run's own state reading reads them.
    #[test]
    fn every_boundary_has_one_spelling_and_reads_back() {
        for word in asked::EVERY_SETUP_BOUNDARY {
            let boundary = the_boundary_named(word)
                .unwrap_or_else(|| panic!("{word} is a boundary this file cannot name"));
            assert_eq!(the_word_for_boundary(boundary), word);
        }
    }

    /// **Every step this file commands is one `smoozy_framing` names**, so a
    /// desktop and a worker cannot spell one act two ways.
    #[test]
    fn every_step_this_file_commands_is_one_the_machine_knows() {
        for commanded in [
            asked::THE_FETCH_STEP,
            asked::THE_UNPACK_STEP,
            asked::THE_LIBRARY_CHAIN_IS_READ_STEP,
            asked::THE_LIBRARY_CHAIN_IS_READ_AGAIN_STEP,
            asked::THE_BUILD_IS_PUBLISHED_STEP,
            asked::THE_FINAL_VERIFICATION_STEP,
        ] {
            assert!(
                asked::EVERY_SETUP_STEP_THE_WORKER_IS_COMMANDED.contains(&commanded),
                "{commanded} is not a step the worker is commanded"
            );
        }
    }

    /// **`Server protocol and transfers` B8d: a desktop reads its own
    /// release's file, from that release's own address, and never the latest.**
    /// Step 10c's transfer wave piece 8; the gap it proves is that until then
    /// every desktop read `…/releases/latest/download/release`.
    #[test]
    fn the_release_address_is_this_builds_own_and_never_the_latest() {
        assert!(WHERE_THE_CURRENT_RELEASE_IS_READ_FROM.starts_with("https://"));
        assert_eq!(
            WHERE_THE_CURRENT_RELEASE_IS_READ_FROM,
            format!(
                "https://github.com/a-kislyakov/Smoozy-releases/releases/download/v{}/release",
                env!("CARGO_PKG_VERSION")
            )
        );
        assert!(
            !WHERE_THE_CURRENT_RELEASE_IS_READ_FROM.contains("latest"),
            "the address is the latest release's, not this build's own"
        );
        assert!(WHERE_THE_CURRENT_RELEASE_IS_READ_FROM
            .contains(&format!("/releases/download/{THE_TAG_OF_THIS_BUILDS_OWN_RELEASE}/")));
    }

    /// **W0-1: the tag's form is the release runner's, read off that brief's
    /// own sentence rather than copied here** — `Preparation/Groundwork/
    /// Templates/Release Runner.md` step 6: *"The tag is the letter `v` followed
    /// by the package version … — `v0.1.7` for 0.1.7 — and nothing else."* The
    /// example's tag is taken apart into what goes before the version, and this
    /// build's tag is held to the same form.
    #[test]
    fn the_tag_is_the_release_runners_own_form() {
        let brief = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../Preparation/Groundwork/Templates/Release Runner.md");
        let text = std::fs::read_to_string(&brief).expect("the release runner's brief is readable");
        let sentence_at = text
            .find("The tag is the letter")
            .expect("the release runner's brief fixes the tag's form");
        let sentence = &text[sentence_at..];
        // The example is the first backticked token after the dash, and the
        // version it stands for follows *for*.
        let example_at = sentence.find("— `").expect("the sentence carries its example") + "— `".len();
        let example_ends = example_at + sentence[example_at..].find('`').expect("the example closes");
        let the_example_tag = &sentence[example_at..example_ends];
        let after = &sentence[example_ends + 1..];
        let version = after
            .trim_start()
            .strip_prefix("for ")
            .expect("the example says which version it is for")
            .split(|c: char| c == ' ' || c == '*')
            .next()
            .expect("a version follows");
        let before_the_version = the_example_tag
            .strip_suffix(version)
            .expect("the example's tag ends in the version it is for");
        assert_eq!(
            THE_TAG_OF_THIS_BUILDS_OWN_RELEASE,
            format!("{before_the_version}{}", env!("CARGO_PKG_VERSION")),
            "this build's tag is not in the form the release runner publishes under"
        );
    }

    // ── A release, signed with keys written down here ────────────────────────

    const THE_ROOT_SEED: [u8; 32] = *b"piece eight's release root, test";
    const THE_ONLINE_SEED: [u8; 32] = *b"piece eight's online signing key";
    const THE_ARTIFACTS_SOURCE: &str = "https://example.invalid/smoozy-server/linux";

    /// **A release of `package_version` at `sequence`, signed the way a real
    /// one is**, with the root's public half beside it.
    fn a_release_of(package_version: &str, sequence: u64) -> (AnOfferedRelease, PublicKeyBytes, Vec<u8>) {
        use smoozy_server_protocol::release::composing::{self, test_only_keys};
        use smoozy_server_protocol::release::trust::{
            AnAllowedKey, KeySetStatement, THE_GREATEST_KEY_SET_GENERATION_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE,
            THE_KEY_SET_FORMAT,
        };
        use smoozy_server_protocol::release::{
            a_signature_over, THE_KEY_SET_SIGNING_CONTEXT, THE_MANIFEST_SIGNING_CONTEXT,
        };
        let artifact = b"the machine's own program, standing in".to_vec();
        let root = test_only_keys::a_key_pair_from_a_written_down_seed(THE_ROOT_SEED);
        let online = test_only_keys::a_key_pair_from_a_written_down_seed(THE_ONLINE_SEED);
        let statement = KeySetStatement {
            statement_format: THE_KEY_SET_FORMAT.into(),
            generation: THE_GREATEST_KEY_SET_GENERATION_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE,
            allowed: vec![AnAllowedKey {
                identity: "the online signing key".into(),
                public_key: online.public,
                valid_from_release_sequence: 1,
            }],
            revocations: Vec::new(),
        };
        let statement_bytes = statement.canonical_bytes();
        let entry = composing::compose_an_artifact_entry(
            "linux-x86_64-gnu",
            composing::the_tuple_this_build_is_made_for(),
            &artifact,
            composing::THE_FIXED_ENTRY_POINT_NAME,
            &[THE_ARTIFACTS_SOURCE.to_string()],
        );
        let manifest = composing::compose_a_manifest(sequence, package_version, 2, None, vec![entry]);
        let manifest_bytes = manifest.canonical_bytes();
        (
            AnOfferedRelease {
                its_root_signature: a_signature_over(
                    &root.secret,
                    THE_KEY_SET_SIGNING_CONTEXT,
                    &statement_bytes,
                ),
                the_key_set_statements_bytes: statement_bytes,
                its_online_signature: a_signature_over(
                    &online.secret,
                    THE_MANIFEST_SIGNING_CONTEXT,
                    &manifest_bytes,
                ),
                the_manifests_bytes: manifest_bytes,
            },
            root.public,
            artifact,
        )
    }

    /// Hosting, as a table of source to bytes.
    struct HostingHere(Vec<(String, Vec<u8>)>);

    impl FetchesOneSource for HostingHere {
        fn fetch(&mut self, source: &str, _ceiling: u64) -> Result<Vec<u8>, String> {
            self.0
                .iter()
                .find(|(named, _)| named == source)
                .map(|(_, bytes)| bytes.clone())
                .ok_or_else(|| format!("{source}: nothing answers there"))
        }
    }

    /// The trust record, in memory.
    #[derive(Default)]
    struct TrustHere(Option<smoozy_server_protocol::release::trust::TheTrustRecord>);

    impl smoozy_server_protocol::release::trust::RemembersTheServerPackageTrustRecord for TrustHere {
        fn read_it(&self) -> Result<Option<smoozy_server_protocol::release::trust::TheTrustRecord>, String> {
            Ok(self.0.clone())
        }
        fn write_it(&mut self, record: &smoozy_server_protocol::release::trust::TheTrustRecord) -> Result<(), String> {
            self.0 = Some(record.clone());
            Ok(())
        }
        fn discard_it(&mut self) -> Result<(), String> {
            self.0 = None;
            Ok(())
        }
    }

    fn hosting_for(offered: &AnOfferedRelease, artifact: &[u8]) -> HostingHere {
        HostingHere(vec![
            (WHERE_THE_CURRENT_RELEASE_IS_READ_FROM.to_string(), offered.canonical_bytes()),
            (THE_ARTIFACTS_SOURCE.to_string(), artifact.to_vec()),
        ])
    }

    fn the_next_sequence() -> u64 {
        smoozy_server_protocol::release::trust::THE_GREATEST_RELEASE_SEQUENCE_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE + 1
    }

    /// **`Server protocol and transfers` B8d: a manifest naming another package
    /// version is refused, and nothing durable moves.** A signed, genuine
    /// release of another version is exactly what the latest address served
    /// every desktop until 2026-09-28.
    #[test]
    fn a_manifest_naming_another_package_version_is_refused_and_moves_no_floor() {
        let (offered, root, artifact) = a_release_of("0.0.1-another", the_next_sequence());
        let mut hosting = hosting_for(&offered, &artifact);
        let mut store = TrustHere::default();
        let refused = this_builds_own_release(
            &root,
            &mut hosting,
            &mut store,
            &smoozy_server_protocol::release::composing::the_tuple_this_build_is_made_for(),
            WHERE_THE_CURRENT_RELEASE_IS_READ_FROM,
            "0.0.2-own",
        )
        .expect_err("a release of another version was taken as this build's own");
        assert!(refused.contains("0.0.1-another") && refused.contains("0.0.2-own"), "{refused}");
        assert!(store.0.is_none(), "a release of another version moved the trust record");

        // **And its own version is admitted**, with its sequence held for the
        // run as this copy's own.
        let (offered, root, artifact) = a_release_of("0.0.2-own", the_next_sequence());
        let mut hosting = hosting_for(&offered, &artifact);
        let (verified, bytes) = this_builds_own_release(
            &root,
            &mut hosting,
            &mut store,
            &smoozy_server_protocol::release::composing::the_tuple_this_build_is_made_for(),
            WHERE_THE_CURRENT_RELEASE_IS_READ_FROM,
            "0.0.2-own",
        )
        .expect("its own release is admitted");
        assert_eq!(verified.manifest.package_version, "0.0.2-own");
        assert_eq!(bytes, artifact);
        assert_eq!(
            this_copys_own_release_sequence_if_known("0.0.2-own"),
            Some(the_next_sequence())
        );
    }

    /// **`Server protocol and transfers` B8d's one edge case: a copy whose
    /// remembered floor is above its own release refuses its own manifest**,
    /// with the ordinary verification failure, and places nothing.
    #[test]
    fn a_remembered_floor_above_its_own_release_refuses_with_the_ordinary_failure() {
        // A newer release accepted earlier, by the latest address…
        let (newer, root, newer_artifact) = a_release_of("0.0.9-newer", the_next_sequence() + 5);
        let mut store = TrustHere::default();
        the_current_release(
            &root,
            &mut hosting_for(&newer, &newer_artifact),
            &mut store,
            &smoozy_server_protocol::release::composing::the_tuple_this_build_is_made_for(),
            WHERE_THE_CURRENT_RELEASE_IS_READ_FROM,
        )
        .expect("the newer release was admitted then");
        // …then its own, older one.
        let (own, root, artifact) = a_release_of("0.0.3-own", the_next_sequence());
        let refused = this_builds_own_release(
            &root,
            &mut hosting_for(&own, &artifact),
            &mut store,
            &smoozy_server_protocol::release::composing::the_tuple_this_build_is_made_for(),
            WHERE_THE_CURRENT_RELEASE_IS_READ_FROM,
            "0.0.3-own",
        )
        .expect_err("a copy placed its own release beneath a floor it remembers");
        assert_eq!(refused, "The release failed the signature check.");
        assert_eq!(this_copys_own_release_sequence_if_known("0.0.3-own"), None);
    }

    /// **W0-2: a copy given a desk program takes it as its own release, at
    /// nought, whatever this run has read of any published release** — and a
    /// copy given none answers only what its own manifest said.
    #[cfg(debug_assertions)]
    #[test]
    fn a_copy_given_a_desk_program_takes_nought_as_its_own_release() {
        this_copys_own_release_is_known("0.0.5-own", 12);
        assert_eq!(the_own_release_sequence_known_to(true, "0.0.5-own"), Some(0));
        assert_eq!(the_own_release_sequence_known_to(false, "0.0.5-own"), Some(12));
        assert_eq!(the_own_release_sequence_known_to(false, "0.0.6-another"), None);
    }

    /// **V14 at `Server Setup` R2's placement.** A machine running a newer
    /// release is refused in the too-new line's words; the same release, an
    /// older one and a machine with nothing active are not.
    #[test]
    fn a_newer_machine_is_refused_at_set_up() {
        let (offered, root, artifact) = a_release_of("0.0.4-own", the_next_sequence());
        let (verified, _) = the_current_release(
            &root,
            &mut hosting_for(&offered, &artifact),
            &mut TrustHere::default(),
            &smoozy_server_protocol::release::composing::the_tuple_this_build_is_made_for(),
            WHERE_THE_CURRENT_RELEASE_IS_READ_FROM,
        )
        .expect("admitted");
        let holding = |sequence: Option<u64>| crate::servers::installation::WhatThePackageAreaHolds {
            it_was_read: true,
            its_release_sequence: sequence,
            ..Default::default()
        };
        let own = the_next_sequence();
        assert_eq!(
            what_v14_refuses(&holding(Some(own + 1)), &verified),
            Some(WhereTheSetUpGot::ThePlacementCouldNotBeCarriedOut {
                said: THE_MACHINE_IS_NEWER.to_string()
            })
        );
        assert_eq!(what_v14_refuses(&holding(Some(own)), &verified), None);
        assert_eq!(what_v14_refuses(&holding(Some(own - 1)), &verified), None);
        assert_eq!(what_v14_refuses(&holding(None), &verified), None);
    }

    /// **W0-2 at `Server Setup` R2's placement: a shortcut copy neither refuses
    /// nor is refused by its own desk program**, and does not place its desk
    /// program over a published release. A debug build's alone, as the desk
    /// program is.
    #[cfg(debug_assertions)]
    #[test]
    fn a_shortcut_copy_sets_up_over_its_own_desk_program_and_not_over_a_release() {
        let holding = |sequence: Option<u64>| crate::servers::installation::WhatThePackageAreaHolds {
            it_was_read: true,
            its_release_sequence: sequence,
            ..Default::default()
        };
        let own = the_next_sequence();
        // **The shortcut.** The desk program's own release, as the ready action
        // reads it in a copy given one.
        let mut an_elf = vec![0x7f, b'E', b'L', b'F', 2, 1, 1, 0];
        an_elf.resize(16, 0);
        an_elf.extend_from_slice(&3u16.to_le_bytes());
        an_elf.extend_from_slice(&62u16.to_le_bytes());
        let (the_desk_programs, _) = crate::the_desk_program::the_desk_programs_release(
            &an_elf,
            &smoozy_server_protocol::release::composing::the_tuple_this_build_is_made_for(),
        )
        .expect("a desk program");
        let desk_placed = holding(Some(crate::the_desk_program::THE_SEQUENCE_A_DESK_PROGRAM_IS_PLACED_UNDER));
        assert_eq!(
            what_v14_refuses(&desk_placed, &the_desk_programs),
            None,
            "a shortcut copy refused its own desk program"
        );
        assert!(
            what_v14_refuses(&holding(Some(own)), &the_desk_programs).is_some(),
            "a shortcut copy would place its desk program over a published release"
        );
    }

    /// **The twenty-fourth notes, note 4: a server set up in this run, on the
    /// same connection, offers the queue's Telegram control.** Step 9, the only
    /// reading of the server's release until then, is held back while a setup
    /// drives the server and runs once per connection, so a server set up and
    /// kept connected was never read and the control stayed grey. The ready
    /// action's placement is handed the desk program here (sequence 0, as in
    /// the test copy); a run that does not reach *done*, or placed nothing,
    /// keeps nothing. A debug build's alone, as the desk program is.
    #[cfg(debug_assertions)]
    #[test]
    fn twenty_fourth_4_a_server_set_up_in_this_run_offers_the_telegram_control() {
        use crate::servers::published::{a_live_jobs_reporting_can_be_changed_on, ALiveJobsReportingCanBeChanged};
        struct TheDeskProgram;
        impl ReadsTheCurrentRelease for TheDeskProgram {
            fn the_current_release(
                &mut self,
                detected: &smoozy_server_protocol::release::manifest::TargetTuple,
            ) -> Result<(AVerifiedRelease, Vec<u8>), String> {
                let mut an_elf = vec![0x7f, b'E', b'L', b'F', 2, 1, 1, 0];
                an_elf.resize(16, 0);
                an_elf.extend_from_slice(&3u16.to_le_bytes());
                an_elf.extend_from_slice(&62u16.to_le_bytes());
                crate::the_desk_program::the_desk_programs_release(&an_elf, detected)
            }
        }
        let done = WhereTheSetUpGot::TheRunEnded {
            at: Boundary::Done,
            ending: TheRunsEnding::ItReachedDone {
                a_blender_was_published: false,
                the_driver_missed_the_optix_bar: false,
                the_swap_waits: false,
            },
        };
        let server = Identity::issue(IdentityKind::LocalServer)
            .expect("this desk has randomness")
            .as_text();
        assert!(a_run_begins(&server, SetupKind::TheReadyAction));
        assert!(
            the_kind_being_driven_on(&server).is_some(),
            "step 9 is held back while the run drives the server"
        );
        let mut desk = TheDeskProgram;
        let mut placing = TheReleaseThisRunPlaces { reads: &mut desk, placed: None };
        placing
            .the_current_release(&smoozy_server_protocol::release::composing::the_tuple_this_build_is_made_for())
            .expect("a desk program");
        assert_eq!(
            placing.placed,
            Some(crate::the_desk_program::THE_SEQUENCE_A_DESK_PROGRAM_IS_PLACED_UNDER)
        );
        assert!(the_release_a_finished_setup_placed_is_kept(&server, &done, placing.placed));
        let _ = a_run_in_the_line_ends(&server, true);
        assert_eq!(
            a_live_jobs_reporting_can_be_changed_on(&server),
            Some(ALiveJobsReportingCanBeChanged::Yes),
            "a server set up in this run left the Telegram control unread"
        );

        // **Not *done*, or nothing placed: nothing is kept**, and step 9's
        // reading on a later connection stays the only one.
        let failed = WhereTheSetUpGot::TheRunEnded {
            at: Boundary::Started,
            ending: TheRunsEnding::AStepFailed { step: Step::Tooling, said: "no".to_string() },
        };
        let another = Identity::issue(IdentityKind::LocalServer)
            .expect("this desk has randomness")
            .as_text();
        assert!(!the_release_a_finished_setup_placed_is_kept(&another, &failed, Some(0)));
        assert!(!the_release_a_finished_setup_placed_is_kept(&another, &done, None));
        assert_eq!(a_live_jobs_reporting_can_be_changed_on(&another), None);
    }

    /// **`Server Setup` Y9: one setup run per server at a time.**
    #[test]
    fn a_second_run_on_one_server_is_refused_while_the_first_stands() {
        let server = Identity::issue(IdentityKind::LocalServer)
            .expect("this desk has randomness")
            .as_text();
        assert!(a_run_begins(&server, SetupKind::TheReadyAction));
        assert!(
            !a_run_begins(&server, SetupKind::ABlenderInstall),
            "a second run was admitted"
        );
        assert_eq!(the_kind_being_driven_on(&server), Some(SetupKind::TheReadyAction));
        // **`Server Setup` Y8's stop is this server's and no other's.**
        the_user_asked_to_stop(&server);
        let asked = TheStopThisServerWasAsked {
            server: server.clone(),
        };
        assert!(asked.the_user_asked_to_stop());
        let another = TheStopThisServerWasAsked {
            server: "another".to_string(),
        };
        assert!(!another.the_user_asked_to_stop());
        a_run_ends(&server);
        assert!(
            a_run_begins(&server, SetupKind::ABlenderInstall),
            "the exclusion was not given back"
        );
        assert_eq!(the_kind_being_driven_on(&server), Some(SetupKind::ABlenderInstall));
        // **The stop goes with the run**, so a later run is not born stopped.
        assert!(!asked.the_user_asked_to_stop());
        a_run_ends(&server);
    }

    /// **`Server Setup` Y7: the five boundaries this side writes are exactly
    /// the ones no worker report can reach.**
    ///
    /// **The other side of the same split is the machine's**, which refuses a
    /// boundary its own worker feeds. Neither list is written twice: this one
    /// is derived from `Server Setup`'s own mapping of a commandable step to
    /// the boundary it feeds, and from `Server Setup` Y4's own set of ten.
    #[test]
    fn the_five_this_side_writes_are_the_ones_no_worker_step_feeds() {
        let the_workers: Vec<Boundary> = asked::EVERY_SETUP_STEP_THE_WORKER_IS_COMMANDED
            .into_iter()
            .filter_map(the_boundary_a_step_reaches)
            .collect();
        let ours: Vec<Boundary> = Boundary::IN_ORDER
            .into_iter()
            .filter(|boundary| {
                *boundary != Boundary::Started && !the_workers.contains(boundary)
            })
            .collect();
        assert_eq!(
            ours,
            vec![
                Boundary::DriverPlaced,
                Boundary::RestartIssued,
                Boundary::CardAnswering,
                Boundary::Telegram,
                Boundary::Done,
            ]
        );
    }

    /// **Row 358's *one under way marked*, read back** — and it is what a step
    /// reaching no boundary of its own is waited on by.
    #[test]
    fn the_runs_state_carries_the_step_under_way() {
        let request = Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            Identity::issue(IdentityKind::Operation).expect("randomness"),
            asked::THE_SETUP_RUNS_STATE_REQUEST,
        );
        let answered = Answered::to(&request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "0.1.0")
            .carrying(
                Fields::none()
                    .and(asked::ON_THIS_MACHINE_FIELD, asked::IT_IS_PRESENT)
                    .and(asked::THE_STEP_FIELD, asked::THE_FINAL_VERIFICATION_STEP),
            );
        assert_eq!(
            the_state_in(&answered).the_step_in_flight.as_deref(),
            Some(asked::THE_FINAL_VERIFICATION_STEP)
        );
        let nothing = Answered::to(&request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "0.1.0");
        assert_eq!(the_state_in(&nothing).the_step_in_flight, None);
    }

    /// **`Server Setup` Y7b and `Server Setup` Y7c, read back: the count and its whole off
    /// the run's state, the count alone where no whole came, and nothing
    /// where no count came or it does not read** — a machine program older
    /// than the count carries none. The Creator's fourteenth notes, piece U.
    #[test]
    fn piece_u_the_runs_state_carries_the_count_and_nothing_without_one() {
        let request = Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            Identity::issue(IdentityKind::Operation).expect("randomness"),
            asked::THE_SETUP_RUNS_STATE_REQUEST,
        );
        let with = |fields: Fields| {
            the_state_in(
                &Answered::to(&request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "0.1.0")
                    .carrying(fields.and(asked::THE_STEP_FIELD, asked::THE_FETCH_STEP)),
            )
            .the_count
        };
        assert_eq!(
            with(
                Fields::none()
                    .and(asked::THE_BYTES_SO_FAR_FIELD, "214000000")
                    .and(asked::THE_BYTES_IN_THE_WHOLE_FIELD, "352000000")
            ),
            Some(TheStepsCount { so_far: 214_000_000, whole: Some(352_000_000) })
        );
        assert_eq!(
            with(Fields::none().and(asked::THE_BYTES_SO_FAR_FIELD, "214000000")),
            Some(TheStepsCount { so_far: 214_000_000, whole: None })
        );
        assert_eq!(with(Fields::none()), None);
        assert_eq!(with(Fields::none().and(asked::THE_BYTES_IN_THE_WHOLE_FIELD, "352000000")), None);
        assert_eq!(with(Fields::none().and(asked::THE_BYTES_SO_FAR_FIELD, "a lot")), None);
        assert_eq!(TheCountedAct::of(asked::THE_FETCH_STEP), Some(TheCountedAct::TheDownload));
        assert_eq!(TheCountedAct::of(asked::THE_UNPACK_STEP), Some(TheCountedAct::TheUnpack));
        assert_eq!(TheCountedAct::of(asked::THE_LIBRARY_CHAIN_IS_READ_STEP), None);
    }

    /// **What the machine printed on the first pass on a real rental,
    /// 2026-09-21**, whole — its newline and its quotes as the machine wrote
    /// them. The screen showed it as `TheMachineWouldNot("…")` with the newline
    /// and the quotes escaped.
    pub(crate) const WHAT_THE_RENTAL_SAID: &str = "the machine would not start the supervisor: \
Job for smoozy-supervisor.service failed because the control process exited with error code.\n\
See \"systemctl --user status smoozy-supervisor.service\" and \"journalctl --user -xeu \
smoozy-supervisor.service\" for details.";

    /// **Fault 4 of the first pass: the placement's failure is the machine's
    /// own words, never a debugging rendering of them.**
    #[test]
    fn the_placements_failure_is_the_machines_own_words_and_never_a_debugging_rendering() {
        let said = the_words_of(&WhyThePlacementCouldNotBeCarriedOut::TheMachineWouldNot(
            WHAT_THE_RENTAL_SAID.to_string(),
        ));
        assert_eq!(said, WHAT_THE_RENTAL_SAID);
        assert!(!said.contains("TheMachineWouldNot"));
        assert!(!said.contains("\\n") && !said.contains("\\\""), "escapes were written out");
        let subsystem = the_words_of(&WhyThePlacementCouldNotBeCarriedOut::TheSubsystem(
            smoozy_server_protocol::subsystem::SubsystemFailure::TheChannelIsGone,
        ));
        assert!(!subsystem.contains("TheChannelIsGone"), "{subsystem}");
    }

    /// **`Server Setup` S1 and S2: the machine's own record of a run, read
    /// back into `Server Setup`'s own shape** — kind, composed list, each
    /// boundary with its mark, moment and clock, and a failed step's ending
    /// with its words.
    #[test]
    fn a_run_is_read_back_off_the_machines_own_answer() {
        let request = Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            Identity::issue(IdentityKind::Operation).expect("randomness"),
            asked::THE_SETUP_RUNS_STATE_REQUEST,
        );
        let item = |at, name| asked::the_field_of_item(asked::A_BOUNDARY_REACHED, at, name);
        let composed = |at| {
            asked::the_field_of_item(asked::A_COMPOSED_BOUNDARY, at, asked::THE_BOUNDARY_FIELD)
        };
        let answered = Answered::to(&request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "0.1.0")
            .carrying(
                Fields::none()
                    .and(asked::ON_THIS_MACHINE_FIELD, asked::IT_IS_PRESENT)
                    .and(asked::THE_RUNS_KIND_FIELD, asked::THE_READY_ACTION)
                    .and(asked::HOW_MANY_COMPOSED_BOUNDARIES_FIELD, "3")
                    .and(&composed(0), asked::THE_BOUNDARY_STARTED)
                    .and(&composed(1), asked::THE_BOUNDARY_FETCHED)
                    .and(&composed(2), asked::THE_BOUNDARY_TOOLING)
                    .and(asked::HOW_MANY_BOUNDARIES_REACHED_FIELD, "1")
                    .and(&item(0, asked::THE_BOUNDARY_FIELD), asked::THE_BOUNDARY_STARTED)
                    .and(&item(0, asked::WHAT_THE_STEP_DID_FIELD), asked::IT_PERFORMED)
                    .and(&item(0, asked::THE_BOUNDARY_AT_FIELD), "1790000000")
                    .and(&item(0, asked::THE_BOUNDARY_CLOCK_FIELD), "server-1")
                    .and(asked::THE_RUNS_ENDING_FIELD, asked::A_STEP_FAILED)
                    .and(asked::THE_STEP_FIELD, asked::THE_FETCH_STEP)
                    .and(asked::THE_MACHINES_OWN_WORDS_FIELD, "404 Not Found"),
            );
        let read = the_setup_as_the_machine_answered(&answered).expect("a reading");
        assert_eq!(read.record.kind, SetupKind::TheReadyAction);
        assert_eq!(
            read.record.the_composed_list,
            vec![Boundary::Started, Boundary::Fetched, Boundary::Tooling]
        );
        assert_eq!(read.record.the_boundaries_reached.len(), 1);
        assert_eq!(read.record.the_boundaries_reached[0].boundary, Boundary::Started);
        assert!(!read.still_open);
        assert_eq!(read.the_step_in_flight, Some(Boundary::Fetched));
        assert!(matches!(
            read.record.ending,
            Some(smoozy_server_setup::recovery::HowTheRunEnded::AStepFailed {
                step: Step::Fetch,
                ..
            })
        ));
        assert_eq!(read.the_clock, "server-1");
        // **A machine that holds no record under the run reads as no reading**,
        // never as an empty one.
        let absent = Answered::to(&request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "0.1.0")
            .carrying(Fields::none().and(asked::ON_THIS_MACHINE_FIELD, asked::IT_IS_ABSENT));
        assert_eq!(the_setup_as_the_machine_answered(&absent), None);
    }

    /// **`Server Setup` S2a, read back: a run whose elevated step failed at the
    /// desk reads as *failed*, on the step it names, with the machine's own
    /// words — and the panel puts the failure shape there.** Added 2026-09-21.
    ///
    /// **The other reading is the one this replaced**: the run left open until
    /// `Server Setup` Y7's window ended it, with no step and no words. **The
    /// panel then said the connection dropped**, which is Smoozy telling the
    /// user less than it knew.
    #[test]
    fn an_elevated_failure_written_by_the_desk_reads_as_failed_with_its_words() {
        let request = Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            Identity::issue(IdentityKind::Operation).expect("randomness"),
            asked::THE_SETUP_RUNS_STATE_REQUEST,
        );
        let item = |at, name| asked::the_field_of_item(asked::A_BOUNDARY_REACHED, at, name);
        let composed = |at| {
            asked::the_field_of_item(asked::A_COMPOSED_BOUNDARY, at, asked::THE_BOUNDARY_FIELD)
        };
        let the_words = "E: Unable to locate package \"nvidia-driver-999\"\n";
        let a_run_that_ended = |ending: Fields| {
            Answered::to(&request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "0.1.0").carrying(
                ending
                    .and(asked::ON_THIS_MACHINE_FIELD, asked::IT_IS_PRESENT)
                    .and(asked::THE_RUNS_KIND_FIELD, asked::THE_READY_ACTION)
                    .and(asked::HOW_MANY_COMPOSED_BOUNDARIES_FIELD, "3")
                    .and(&composed(0), asked::THE_BOUNDARY_STARTED)
                    .and(&composed(1), asked::THE_BOUNDARY_DRIVER_PLACED)
                    .and(&composed(2), asked::THE_BOUNDARY_DONE)
                    .and(asked::HOW_MANY_BOUNDARIES_REACHED_FIELD, "1")
                    .and(&item(0, asked::THE_BOUNDARY_FIELD), asked::THE_BOUNDARY_STARTED)
                    .and(&item(0, asked::WHAT_THE_STEP_DID_FIELD), asked::IT_HAD_NOTHING_TO_DO)
                    .and(&item(0, asked::THE_BOUNDARY_AT_FIELD), "1790000000")
                    .and(&item(0, asked::THE_BOUNDARY_CLOCK_FIELD), "server-1"),
            )
        };
        let failed = the_setup_as_the_machine_answered(&a_run_that_ended(
            Fields::none()
                .and(asked::THE_RUNS_ENDING_FIELD, asked::A_STEP_FAILED)
                .and(asked::THE_FAILED_STEP_FIELD, asked::THE_BOUNDARY_DRIVER_PLACED)
                .and(asked::THE_MACHINES_OWN_WORDS_FIELD, the_words),
        ))
        .expect("a reading");
        assert_eq!(failed.the_step_in_flight, Some(Boundary::DriverPlaced));
        let Some(smoozy_server_setup::recovery::HowTheRunEnded::AStepFailed { step, words }) =
            &failed.record.ending
        else {
            panic!("the run did not read as failed: {:?}", failed.record.ending);
        };
        assert_eq!(*step, Step::Driver);
        assert_eq!(
            *words,
            smoozy_server_setup::recovery::TheFailedStepsWords::SmoozysOwnVerification {
                what_was_read: the_words.to_string()
            }
        );

        let cut = the_setup_as_the_machine_answered(&a_run_that_ended(Fields::none().and(
            asked::THE_RUNS_ENDING_FIELD,
            asked::IT_ENDED_AT_ITS_BOUNDARY_NOT_COMPLETED,
        )))
        .expect("a reading");

        let panel = |read: &TheSetupAsRead| {
            crate::surface::set_up::the_run_panel(
                &read.record,
                read.the_step_in_flight,
                &read.the_clock,
                read.still_open,
                false,
            )
        };
        let the_failed_panel = panel(&failed);
        let the_cut_panel = panel(&cut);
        let shapes: Vec<_> = the_failed_panel
            .steps
            .iter()
            .filter_map(|line| line.the_failure.as_ref())
            .collect();
        assert_eq!(shapes.len(), 1, "the failure shape sits on the one step that failed");
        assert_eq!(
            shapes[0].the_output,
            Some(crate::typography::Verbatim::new(the_words)),
            "the machine's own words are not behind the control, whole"
        );
        assert!(
            the_cut_panel.steps.iter().all(|line| line.the_failure.is_none()),
            "a cut run claims a failure"
        );
        assert_ne!(
            the_failed_panel.how_it_ended.map(|it| it.lead),
            the_cut_panel.how_it_ended.map(|it| it.lead),
            "the failed run reads as the cut one"
        );
    }

    /// **`Server Setup` S6's abandon reads back as its own ending, with what
    /// the final check found**, and never as a failed step. Added 2026-09-22.
    #[test]
    fn an_abandoned_run_reads_back_as_abandoned_with_its_finding() {
        let request = Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            Identity::issue(IdentityKind::Operation).expect("randomness"),
            asked::THE_SETUP_RUNS_STATE_REQUEST,
        );
        let answered = Answered::to(&request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "0.1.2")
            .carrying(
                Fields::none()
                    .and(asked::ON_THIS_MACHINE_FIELD, asked::IT_IS_PRESENT)
                    .and(asked::THE_RUNS_KIND_FIELD, asked::THE_READY_ACTION)
                    .and(asked::THE_RUNS_ENDING_FIELD, asked::IT_WAS_ABANDONED)
                    .and(asked::THE_MACHINES_OWN_WORDS_FIELD, "the driver did not answer"),
            );
        let read = the_setup_as_the_machine_answered(&answered).expect("a reading");
        assert!(!read.still_open);
        assert_eq!(
            read.record.ending,
            Some(smoozy_server_setup::recovery::HowTheRunEnded::Abandoned {
                what_the_check_found:
                    smoozy_server_setup::recovery::TheFailedStepsWords::SmoozysOwnVerification {
                        what_was_read: "the driver did not answer".to_string(),
                    },
            })
        );
    }

    /// **`Server Setup` S1 over a run cut during an admin step: the step the
    /// machine recorded as started reads *stopped part-way*, never *not
    /// started*.** Added 2026-09-29, step 10c's transfer wave piece 12 (the
    /// eighth-pass map's piece 7).
    ///
    /// **The answer is the one the machine's own program gives** once
    /// `Server Setup` Y7's window has ended a run whose driver step the desktop
    /// had recorded as started: the ending *ended at its boundary, not
    /// completed*, and the step under way named by the step's first boundary —
    /// the same naming `Server Setup` S2a's failed step uses. **Undone, the
    /// reading dropped that name**, because it read the step under way only
    /// as a step the worker is commanded, and the panel drew *Install the
    /// graphics driver — not started* over a driver install that had been cut
    /// part-way.
    #[test]
    fn a_run_cut_during_an_admin_step_reads_that_step_as_stopped_part_way() {
        let request = Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            Identity::issue(IdentityKind::Operation).expect("randomness"),
            asked::THE_SETUP_RUNS_STATE_REQUEST,
        );
        let item = |at, name| asked::the_field_of_item(asked::A_BOUNDARY_REACHED, at, name);
        let composed = |at| {
            asked::the_field_of_item(asked::A_COMPOSED_BOUNDARY, at, asked::THE_BOUNDARY_FIELD)
        };
        let a_cut_run = |under_way: Option<&str>| {
            let mut fields = Fields::none()
                .and(asked::ON_THIS_MACHINE_FIELD, asked::IT_IS_PRESENT)
                .and(asked::THE_RUNS_KIND_FIELD, asked::THE_READY_ACTION)
                .and(asked::HOW_MANY_COMPOSED_BOUNDARIES_FIELD, "5")
                .and(&composed(0), asked::THE_BOUNDARY_STARTED)
                .and(&composed(1), asked::THE_BOUNDARY_DRIVER_PLACED)
                .and(&composed(2), asked::THE_BOUNDARY_RESTART_ISSUED)
                .and(&composed(3), asked::THE_BOUNDARY_CARD_ANSWERING)
                .and(&composed(4), asked::THE_BOUNDARY_DONE)
                .and(asked::HOW_MANY_BOUNDARIES_REACHED_FIELD, "1")
                .and(&item(0, asked::THE_BOUNDARY_FIELD), asked::THE_BOUNDARY_STARTED)
                .and(&item(0, asked::WHAT_THE_STEP_DID_FIELD), asked::IT_PERFORMED)
                .and(&item(0, asked::THE_BOUNDARY_AT_FIELD), "1790000000")
                .and(&item(0, asked::THE_BOUNDARY_CLOCK_FIELD), "server-1")
                .and(
                    asked::THE_RUNS_ENDING_FIELD,
                    asked::IT_ENDED_AT_ITS_BOUNDARY_NOT_COMPLETED,
                );
            if let Some(step) = under_way {
                fields = fields.and(asked::THE_STEP_FIELD, step);
            }
            Answered::to(&request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "0.1.7")
                .carrying(fields)
        };
        let the_drivers_mark = |read: &TheSetupAsRead| {
            let panel = crate::surface::set_up::the_run_panel(
                &read.record,
                read.the_step_in_flight,
                &read.the_clock,
                read.still_open,
                false,
            );
            let driver = crate::surface::set_up::the_boundarys_name(Boundary::DriverPlaced)
                .expect("the driver step has a name");
            panel
                .steps
                .iter()
                .find(|line| line.name == crate::typography::Prose::new(driver))
                .map(|line| line.mark.as_str().to_string())
                .expect("the driver step is a line of the panel")
        };

        let cut = the_setup_as_the_machine_answered(&a_cut_run(Some(
            asked::THE_BOUNDARY_DRIVER_PLACED,
        )))
        .expect("a reading");
        assert!(!cut.still_open);
        assert_eq!(cut.the_step_in_flight, Some(Boundary::DriverPlaced));
        assert_eq!(
            the_drivers_mark(&cut),
            "bad",
            "the driver step the machine recorded as started reads as stopped part-way"
        );

        // **The other direction, and it is `Server Setup` S1's own
        // distinction**: a run cut before the driver step began names no step
        // under way, and the driver step reads *not started*.
        let before = the_setup_as_the_machine_answered(&a_cut_run(None)).expect("a reading");
        assert_eq!(before.the_step_in_flight, None);
        assert_eq!(the_drivers_mark(&before), "skip");
    }

    /// **`Server Setup` B3 over the shop-front's listing: the highest stable
    /// version, and never a long-term-support line for being one.**
    #[test]
    fn the_add_dialogs_tick_asks_for_the_latest_finished_release() {
        use smoozy_server_setup::blender::{PublishedChannel, Risk, Version};
        let a = |track: &str, risk, major, minor, patch| PublishedChannel {
            track: track.to_string(),
            risk,
            version: Version {
                major,
                minor,
                patch,
            },
            released_at: String::new(),
            size_bytes: 0,
            the_build_services_file: None,
        };
        let listing = vec![
            a("4.5lts", Risk::Stable, 4, 5, 13),
            a("latest", Risk::Edge, 5, 3, 0),
            a("latest", Risk::Stable, 5, 2, 1),
        ];
        assert_eq!(the_latest_finished_release_in(&listing), Some("5.2.1".to_string()));
        // **No tick, no Blender, and nothing is read for it.**
        assert_eq!(
            the_blender_the_add_asked_for(&[crate::surface::servers::GET_THE_SERVER_READY
                .to_string()]),
            Ok(None)
        );
    }

    /// **Step 10c's thirteenth-pass fix piece 2: a deferral over a held job is
    /// said as the held jobs of an earlier setup, counted**, and every other
    /// phase as it was — `Design/Server Setup § When work on the server holds
    /// the setup back`. Waiting and rendering work is not counted: it ends on
    /// its own and the next press goes ahead.
    #[test]
    fn a_deferral_over_held_jobs_counts_them_and_nothing_else_does() {
        use smoozy_runtime::package::retry::AcceptedWorkStanding as Standing;
        let deferred = || WhatThePlacementReached::Deferred {
            because: "a, b, c".to_string(),
        };
        let read = vec![
            ("a".to_string(), Standing::Live),
            ("b".to_string(), Standing::Held),
            ("c".to_string(), Standing::Held),
            ("d".to_string(), Standing::Nothing),
        ];
        assert_eq!(
            what_a_placement_not_verified_came_to(deferred(), &read),
            WhereTheSetUpGot::JobsOfAnEarlierSetupAreHeld {
                how_many: 2,
                because: "a, b, c".to_string()
            }
        );
        let only_live = vec![("a".to_string(), Standing::Live)];
        assert_eq!(
            what_a_placement_not_verified_came_to(deferred(), &only_live),
            WhereTheSetUpGot::ThePlacement { reached: deferred() }
        );
        assert_eq!(
            what_a_placement_not_verified_came_to(WhatThePlacementReached::RolledBack, &read),
            WhereTheSetUpGot::ThePlacement {
                reached: WhatThePlacementReached::RolledBack
            }
        );
    }

    /// **What a press of *Set up this server* in the gap does** — the
    /// Creator's fourth note 1, piece M. Before the connection is lent the
    /// press finds none and says *Connect to this server first.* After it is
    /// lent, the user's own run can take `Server Setup` Y9's exclusion first,
    /// and step 8's hand-over is then refused with nothing said, so a ticked
    /// *Install the latest stable Blender* is never carried out: the card's
    /// own action asks for no Blender.
    #[test]
    fn piece_m_proof_a_press_in_the_gap_takes_the_run_and_the_remembered_setup_is_refused_in_silence() {
        let server = "server-m-press-in-the-gap";
        assert!(a_run_begins(server, SetupKind::TheReadyAction), "the user's press takes the run");
        assert!(
            !a_run_begins(server, SetupKind::TheReadyAction),
            "step 8's hand-over is refused"
        );
        a_run_ends(server);
        let working = include_str!("setting_up.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("the working half");
        let carried = working
            .split("pub fn the_remembered_setup_is_carried_out<R: tauri::Runtime>(")
            .nth(1)
            .and_then(|rest| rest.split("\n}\n").next())
            .expect("step 8");
        // **Taken by the card's one way since 2026-10-16** (step 10c's owed
        // piece 5, `Server Setup` Y9a), and the refusal is as silent as it was.
        assert!(
            carried.contains("let taken = the_remembered_setup_is_taken(&server,")
                && carried.contains("if !taken {\n        return;\n    }"),
            "the refusal says nothing and carries nothing"
        );
        let press = include_str!("surface/set_up.rs")
            .split("fn the_ready_action_is_driven<R: tauri::Runtime>(")
            .nth(1)
            .and_then(|rest| rest.split("\n}\n").next())
            .expect("the press");
        assert!(press.contains("words::NO_SERVER_IS_CONNECTED"), "before the lending");
        // **The run is the line's act from 2026-10-27** (`Server Setup` Y9a),
        // performed where every act is.
        let performed = include_str!("surface/set_up.rs")
            .split("fn the_act_is_performed<R: tauri::Runtime>(")
            .nth(1)
            .and_then(|rest| rest.split("\n}\n").next())
            .expect("the act performed");
        assert!(
            performed.contains("crate::setting_up::the_ready_action_on(app, &point, server, None)"),
            "the press asks for no Blender"
        );
    }

    // ── `Server Setup` Y7a — the thirteenth notes, piece H, 2026-10-18 ─────

    /// **Each piece a command prints is told while the command is still
    /// running**, and every piece still waiting is told before the answer is
    /// handed back. Before piece H a command's words reached this side only in
    /// its answer.
    #[tokio::test]
    async fn y7a_each_piece_is_told_while_the_command_runs_and_none_after_its_answer() {
        let (up, mut pieces) = tokio::sync::mpsc::unbounded_channel();
        let (let_it_end, it_may_end) = tokio::sync::oneshot::channel::<()>();
        let told = std::sync::Arc::new(std::sync::Mutex::new(Vec::<APiecePrinted>::new()));
        let seen_before_the_end = std::sync::Arc::new(std::sync::Mutex::new(0usize));
        let command = {
            let up = up.clone();
            let told = std::sync::Arc::clone(&told);
            let seen_before_the_end = std::sync::Arc::clone(&seen_before_the_end);
            async move {
                up.send(APiecePrinted::Output(b"Reading package lists...\n".to_vec())).unwrap();
                // **The command waits here until the first piece has been
                // told**, which is only possible if pieces are told while it
                // runs.
                while told.lock().unwrap().is_empty() {
                    tokio::task::yield_now().await;
                }
                *seen_before_the_end.lock().unwrap() = told.lock().unwrap().len();
                up.send(APiecePrinted::Complaint(b"debconf: delaying\n".to_vec())).unwrap();
                up.send(APiecePrinted::Output(b"Setting up zstd ...\n".to_vec())).unwrap();
                let _ = it_may_end.await;
                "the answer"
            }
        };
        drop(up);
        let _ = let_it_end.send(());
        let answer = tokio::time::timeout(
            std::time::Duration::from_secs(5),
            the_pieces_are_told_while(command, &mut pieces, |piece| told.lock().unwrap().push(piece)),
        )
        .await
        .expect("a piece told only at the end would leave the command waiting for ever");
        assert_eq!(answer, "the answer");
        assert_eq!(*seen_before_the_end.lock().unwrap(), 1, "the first piece was told while it ran");
        assert_eq!(
            *told.lock().unwrap(),
            vec![
                APiecePrinted::Output(b"Reading package lists...\n".to_vec()),
                APiecePrinted::Complaint(b"debconf: delaying\n".to_vec()),
                APiecePrinted::Output(b"Setting up zstd ...\n".to_vec()),
            ],
            "every piece, in order, before the answer"
        );
    }

    /// **The window's lines: the newest three of either stream, in the order
    /// they arrived, each with its stream** — a line ended at its line feed, a
    /// carriage return before it changing nothing; **a line not yet ended
    /// shown as it stands**; **a line rewritten in place shown in its newest
    /// form**; another step's piece on a stream letting that stream's unended
    /// line go, so two steps' words are never joined; and the kept words hold
    /// both streams apart by step, every byte as printed (`Server Setup` Y7a,
    /// M13b's second narrowing). **Until 2026-10-25 the complaints stream
    /// never reached the window, and neither an unended line nor its newest
    /// form was shown** (the eighteenth notes, piece 2c).
    #[test]
    fn eighteenth_the_windows_lines_are_the_newest_three_of_either_stream_as_a_terminal_shows_them() {
        use elevated::WhichStream::{TheComplaints as Errors, TheOrdinaryOutput as Ordinary};
        let line = |stream, words: &str| (stream, words.to_string());
        let mut words = TheRunsWords::default();
        assert!(words.the_window().is_empty(), "nothing before the first line");
        words.a_piece_is_printed(Step::Driver, &APiecePrinted::Output(b"Reading package li".to_vec()));
        assert_eq!(words.the_window(), vec![line(Ordinary, "Reading package li")], "an unended line is not shown");
        words.a_piece_is_printed(Step::Driver, &APiecePrinted::Output(b"sts...\r\nBuilding dependency tree...\n".to_vec()));
        words.a_piece_is_printed(Step::Driver, &APiecePrinted::Complaint(b"E: a complaint\n".to_vec()));
        assert_eq!(
            words.the_window(),
            vec![
                line(Ordinary, "Reading package lists..."),
                line(Ordinary, "Building dependency tree..."),
                line(Errors, "E: a complaint"),
            ],
            "the complaints stream did not reach the window, in its place"
        );
        words.a_piece_is_printed(
            Step::Driver,
            &APiecePrinted::Output(b"Unpacking nvidia-driver-595 ...\nSetting up nvidia-dkms-595 ...\nBuilding initial module".to_vec()),
        );
        assert_eq!(
            words.the_window(),
            vec![
                line(Ordinary, "Unpacking nvidia-driver-595 ..."),
                line(Ordinary, "Setting up nvidia-dkms-595 ..."),
                line(Ordinary, "Building initial module"),
            ]
        );
        // **A line rewritten in place shows its newest form.**
        words.a_piece_is_printed(Step::Driver, &APiecePrinted::Output(b" 10%\rBuilding initial module 57%".to_vec()));
        assert_eq!(words.the_window().last(), Some(&line(Ordinary, "Building initial module 57%")));
        // **Another step's complaint leaves the ordinary stream's unended line
        // as it stands**, and takes its place among the lines.
        words.a_piece_is_printed(Step::Restart, &APiecePrinted::Complaint(b"going down\n".to_vec()));
        assert_eq!(
            words.the_window(),
            vec![
                line(Ordinary, "Setting up nvidia-dkms-595 ..."),
                line(Errors, "going down"),
                line(Ordinary, "Building initial module 57%"),
            ]
        );
        // **Another step's ordinary line lets the driver step's unended line
        // go**, never joined to it.
        words.a_piece_is_printed(Step::Restart, &APiecePrinted::Output(b"Restarting\n".to_vec()));
        assert_eq!(
            words.the_window(),
            vec![
                line(Ordinary, "Setting up nvidia-dkms-595 ..."),
                line(Errors, "going down"),
                line(Ordinary, "Restarting"),
            ]
        );
        assert_eq!(
            words.the_newest_lines(),
            vec!["Setting up nvidia-dkms-595 ...".to_string(), "going down".to_string(), "Restarting".to_string()]
        );

        assert_eq!(
            words.the_whole_of(elevated::WhichStream::TheOrdinaryOutput),
            "Reading package lists...\r\nBuilding dependency tree...\nUnpacking nvidia-driver-595 ...\nSetting up nvidia-dkms-595 ...\nBuilding initial module 10%\rBuilding initial module 57%Restarting\n",
            "the record is whole and unedited, the unfinished and rewritten line included"
        );
        assert_eq!(words.the_whole_of(elevated::WhichStream::TheComplaints), "E: a complaint\ngoing down\n");
        let steps: Vec<Step> = words.printed.the_steps().iter().map(|one| one.step).collect();
        assert_eq!(steps, vec![Step::Driver, Step::Restart], "kept apart by step");

        // **A character whose bytes have not all arrived waits for the rest.**
        let mut split = TheRunsWords::default();
        let euro = "€".as_bytes();
        split.a_piece_is_printed(Step::Fetch, &APiecePrinted::Output([b"1 ".as_slice(), &euro[..2]].concat()));
        assert_eq!(split.the_newest_lines(), vec!["1 ".to_string()]);
        split.a_piece_is_printed(Step::Fetch, &APiecePrinted::Output(euro[2..].to_vec()));
        assert_eq!(split.the_newest_lines(), vec!["1 €".to_string()]);
    }

    /// **`Server Setup` Y7a item 2 on the desk, from 2026-10-25: each piece
    /// kept once as its reading lands, the reading naming what is held, under
    /// the turn the machine gave** — a new turn being another commanding of
    /// the step, whose lengths start again; the mark kept once; and *more
    /// follows* read only for this turn. Piece 2c.
    #[test]
    fn eighteenth_a_steps_pieces_are_kept_once_under_their_turn_and_the_reading_names_what_is_held() {
        let server = "eighteenth-pieces-kept-under-their-turn";
        assert!(a_run_begins(server, SetupKind::ABlenderInstall));
        let go = the_words_go_to_the_card(server, || {});
        let fetch = asked::THE_FETCH_STEP;
        let reading = |turn: &str, ordinary: &str, complaints: &str, more: bool, marked: bool| TheRunsState {
            the_step_in_flight: Some(fetch.to_string()),
            the_steps_words: Some(TheStepsWords {
                step: fetch.to_string(),
                printed: Some((ordinary.to_string(), complaints.to_string())),
                turn: Some(turn.to_string()),
                more_follow: more,
                the_rest_was_not_kept: marked,
            }),
            ..TheRunsState::default()
        };
        let mut held = TheStepsWordsHeldHere::default();
        let named = |held: &TheStepsWordsHeldHere| {
            let fields = held.the_reading_names(Fields::none());
            (
                fields.text(asked::THE_STEPS_WORDS_TURN_FIELD).map(str::to_string),
                fields.text(asked::THE_ORDINARY_WORDS_HELD_FIELD).map(str::to_string),
                fields.text(asked::THE_COMPLAINT_WORDS_HELD_FIELD).map(str::to_string),
            )
        };
        assert_eq!(named(&held), (None, None, None), "the first reading named lengths it had no turn for");
        assert!(held.a_reading_is_kept(Some(&go), Step::Fetch, fetch, &reading("t1", "a: 18 of 352 MB\nb", "", true, false)));
        assert_eq!(named(&held), (Some("t1".to_string()), Some("17".to_string()), Some("0".to_string())));
        assert!(held.more_follow(fetch, &reading("t1", "", "", true, false)));
        assert!(!held.more_follow(fetch, &reading("t2", "", "", true, false)), "another turn's more was read as this one's");
        assert!(!held.a_reading_is_kept(Some(&go), Step::Fetch, fetch, &reading("t1", "", "", false, false)), "nothing new was new");
        assert!(held.a_reading_is_kept(Some(&go), Step::Fetch, fetch, &reading("t1", "c\n", "w\n", false, true)));
        assert!(!held.a_reading_is_kept(Some(&go), Step::Fetch, fetch, &reading("t1", "", "", false, true)), "the mark kept twice");
        assert_eq!(
            the_whole_of_what_was_printed_on(server),
            Some(("a: 18 of 352 MB\nbc\n".to_string(), "w\n".to_string()))
        );
        assert!(some_of_what_was_printed_was_not_kept_on(server));
        // **Another turn starts its lengths again.**
        assert!(held.a_reading_is_kept(Some(&go), Step::Fetch, fetch, &reading("t2", "again\n", "", false, false)));
        assert_eq!(named(&held), (Some("t2".to_string()), Some("6".to_string()), Some("0".to_string())));
        // **Words of another step are not this wait's.**
        assert!(!held.a_reading_is_kept(Some(&go), Step::Fetch, asked::THE_UNPACK_STEP, &reading("t2", "x\n", "", false, false)));
        a_run_ends(server);
    }

    /// **The Creator's fourteenth notes, note 1, piece S: the window's lines
    /// stay while the steps after them print nothing, and go at the next
    /// run.** `Design/Server Setup § A long step`: *"kept until another step
    /// prints or the run ends"*. Before piece S they went the moment the next
    /// step began, so on a container — whose long steps are the machine's own
    /// and print nothing to the desktop — a few seconds of an install were all
    /// the window ever held.
    #[test]
    fn piece_s_the_lines_stay_across_a_step_that_prints_nothing_and_go_at_the_next_run() {
        let server = "piece-s-the-lines-kept";
        assert!(a_run_begins(server, SetupKind::TheReadyAction));
        assert!(a_piece_is_kept(server, Step::Libraries, &APiecePrinted::Output(b"Setting up libgl1 ...\n".to_vec())));
        // The steps after it print nothing to the desktop, or only
        // complaints — which, from 2026-10-25, join the lines in their place
        // (M13b; the eighteenth notes, piece 2c).
        assert!(a_piece_is_kept(server, Step::BlenderPublication, &APiecePrinted::Complaint(b"a warning\n".to_vec())));
        assert_eq!(
            the_newest_lines_on(server),
            vec!["Setting up libgl1 ...".to_string(), "a warning".to_string()],
            "the lines went when a step that printed nothing began"
        );
        a_run_ends(server);
        assert!(a_run_begins(server, SetupKind::ABlenderInstall));
        assert!(the_newest_lines_on(server).is_empty(), "the last run's lines stood in the next run");
        a_run_ends(server);
    }

    /// **The words are kept from the run being taken until the next run**:
    /// taking a run lets the last one's go, and a server with no run taken
    /// keeps nothing.
    #[test]
    fn y7a_a_runs_words_are_kept_until_the_next_run() {
        let server = "piece-h-the-words-kept";
        assert!(!the_words_are_kept_for(server));
        assert!(!a_piece_is_kept(server, Step::Driver, &APiecePrinted::Output(b"x\n".to_vec())));
        assert!(a_run_begins(server, SetupKind::TheReadyAction));
        assert!(the_words_are_kept_for(server), "kept from the run being taken");
        assert!(the_newest_lines_on(server).is_empty(), "nothing before the first line");
        assert!(a_piece_is_kept(server, Step::Driver, &APiecePrinted::Output(b"one\n".to_vec())));
        assert_eq!(the_newest_lines_on(server), vec!["one".to_string()]);
        a_run_ends(server);
        assert!(the_words_are_kept_for(server), "the run's ending keeps its words");
        assert_eq!(
            the_whole_of_what_was_printed_on(server),
            Some(("one\n".to_string(), String::new()))
        );
        assert!(a_run_begins(server, SetupKind::ABlenderInstall));
        assert_eq!(
            the_whole_of_what_was_printed_on(server),
            Some((String::new(), String::new())),
            "the next run lets the last run's words go"
        );
        a_run_ends(server);
    }

    // ── `Server Setup` Y7a as changed — the sixteenth notes, note 1, 2026-10-24

    /// **A worker step's words are kept once, at the reading that shows the
    /// step ended, after an earlier admin step's words and before the next
    /// command's** — so *Server output* holds the run's words in the order the
    /// run printed them, however many readings carried the same words. A
    /// reading with the step still in flight, and words naming another step,
    /// keep nothing; a mark keeps the mark and no words. Until 2026-10-24 a
    /// worker step's words never reached the keeping at all.
    #[test]
    fn y7a_a_worker_steps_words_are_kept_once_between_the_admin_steps_around_it() {
        let server = "y7a-a-worker-steps-words-kept-once";
        assert!(a_run_begins(server, SetupKind::TheReadyAction));
        let go = the_words_go_to_the_card(server, || {});
        // **An earlier admin step prints, as it prints.**
        go(Step::Tooling, &WhatTheServerPrinted::APiece(APiecePrinted::Output(b"Setting up xz-utils ...\n".to_vec())));
        let reading = |in_flight: Option<&str>, step: &str, printed: Option<(&str, &str)>| TheRunsState {
            the_step_in_flight: in_flight.map(str::to_string),
            the_steps_words: Some(TheStepsWords {
                step: step.to_string(),
                the_rest_was_not_kept: printed.is_none(),
                printed: printed.map(|(ordinary, complaints)| (ordinary.to_string(), complaints.to_string())),
                ..TheStepsWords::default()
            }),
            ..TheRunsState::default()
        };
        let mut kept = false;
        let unpack = asked::THE_UNPACK_STEP;
        // **Still in flight**: nothing, even were words there.
        a_worker_steps_words_are_kept_once(
            Some(&go),
            Step::Tooling,
            unpack,
            &reading(Some(unpack), unpack, Some(("early\n", ""))),
            &mut kept,
        );
        // **Another step's words** are not this wait's.
        a_worker_steps_words_are_kept_once(
            Some(&go),
            Step::Tooling,
            unpack,
            &reading(None, asked::THE_FETCH_STEP, Some(("not mine\n", ""))),
            &mut kept,
        );
        assert!(!kept);
        // **Ended: kept, once, however many readings carry them.**
        for _ in 0..3 {
            a_worker_steps_words_are_kept_once(
                Some(&go),
                Step::Tooling,
                unpack,
                &reading(None, unpack, Some(("x blender/blender\n", "tar: a warning\n"))),
                &mut kept,
            );
        }
        assert!(kept);
        // **The next command's words come after.**
        go(Step::Libraries, &WhatTheServerPrinted::APiece(APiecePrinted::Output(b"Setting up libegl1 ...\n".to_vec())));
        assert_eq!(
            the_whole_of_what_was_printed_on(server),
            Some((
                "Setting up xz-utils ...\nx blender/blender\nSetting up libegl1 ...\n".to_string(),
                "tar: a warning\n".to_string(),
            )),
            "the worker step's words were not kept once, in the run's order"
        );
        // **Both streams, in the order they were kept** (from 2026-10-25).
        assert_eq!(
            the_newest_lines_on(server),
            vec![
                "x blender/blender".to_string(),
                "tar: a warning".to_string(),
                "Setting up libegl1 ...".to_string(),
            ],
            "the window took the worker step's lines"
        );
        assert!(!some_of_what_was_printed_was_not_kept_on(server));

        // **A step whose words could not be brought back keeps the mark, once,
        // and no words.**
        let mut kept = false;
        for _ in 0..2 {
            a_worker_steps_words_are_kept_once(
                Some(&go),
                Step::Libraries,
                asked::THE_LIBRARY_CHAIN_IS_READ_AGAIN_STEP,
                &reading(None, asked::THE_LIBRARY_CHAIN_IS_READ_AGAIN_STEP, None),
                &mut kept,
            );
        }
        assert!(some_of_what_was_printed_was_not_kept_on(server));
        assert_eq!(
            the_whole_of_what_was_printed_on(server).map(|(ordinary, _)| ordinary),
            Some("Setting up xz-utils ...\nx blender/blender\nSetting up libegl1 ...\n".to_string())
        );
        a_run_ends(server);
    }

    /// **The run's state reading, read for `Server Setup` Y7a item 2's words**:
    /// the words with their step, the mark in their place, and nothing where
    /// the reading names no step for them — an older machine program.
    #[test]
    fn y7a_the_runs_state_carries_a_worker_steps_words_or_their_mark() {
        let answered = |fields: Fields| {
            Answered::to(
                &Request::speaking(
                    THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
                    Identity::issue(IdentityKind::Operation).expect("randomness"),
                    asked::THE_SETUP_RUNS_STATE_REQUEST,
                ),
                THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
                "0.1.8",
            )
            .carrying(fields)
        };
        let with_words = the_state_in(&answered(
            Fields::none()
                .and(asked::THE_STEP_THE_WORDS_BELONG_TO_FIELD, asked::THE_BUILD_IS_PUBLISHED_STEP)
                .and(asked::THE_STEPS_ORDINARY_WORDS_FIELD, "Blender 5.2.2\n")
                .and(asked::THE_STEPS_COMPLAINT_WORDS_FIELD, ""),
        ));
        assert_eq!(
            with_words.the_steps_words,
            Some(TheStepsWords {
                step: asked::THE_BUILD_IS_PUBLISHED_STEP.to_string(),
                printed: Some(("Blender 5.2.2\n".to_string(), String::new())),
                ..TheStepsWords::default()
            })
        );
        let marked = the_state_in(&answered(
            Fields::none()
                .and(asked::THE_STEP_THE_WORDS_BELONG_TO_FIELD, asked::THE_UNPACK_STEP)
                .and(asked::THE_STEPS_WORDS_WERE_NOT_KEPT_FIELD, asked::THE_WORDS_WERE_NOT_KEPT),
        ));
        assert_eq!(
            marked.the_steps_words,
            Some(TheStepsWords {
                step: asked::THE_UNPACK_STEP.to_string(),
                printed: None,
                the_rest_was_not_kept: true,
                ..TheStepsWords::default()
            })
        );
        assert_eq!(the_state_in(&answered(Fields::none())).the_steps_words, None);
    }

    /// **`Server Setup` Y7a item 4, the desk's half — the Creator's ruling on
    /// the sixteenth notes, note 1: no command string a setup run can send
    /// reads or prints a secret.** Every one is fixed at build time
    /// (`Server Setup` E7), so this holds the closed set: every elevated step
    /// in all three of its forms, the elevated search, the validate, the
    /// probes, every reading `Server Setup` and `Servers` send, and the
    /// restart. **None names the environment, a process's
    /// environment file, the login's keys or the bot token's file.** The
    /// password rides standard input and the token rides the file-transfer
    /// subsystem, so neither is ever in a string. The worker's half is held in
    /// the machine's program (`the_setup_worker`).
    #[test]
    fn y7a_no_command_a_setup_run_sends_reads_or_prints_a_secret() {
        use smoozy_server_setup::elevated;
        let mut every: Vec<String> = Vec::new();
        for step in elevated::THE_ELEVATED_STEPS.iter().chain([&elevated::THE_ELEVATED_SEARCH]) {
            every.push(step.plain.as_str().to_string());
            every.push(step.elevated.as_str().to_string());
            every.push(step.elevated_reading_the_password_from_standard_input.as_str().to_string());
        }
        every.push(smoozy_server_setup::elevation::THE_VALIDATE_FORM.as_str().to_string());
        every.push(smoozy_server_setup::kind::THE_NON_INTERACTIVE_PROBE.as_str().to_string());
        every.push(smoozy_server_setup::restart::THE_RESTART.as_str().to_string());
        every.extend(
            smoozy_server_setup::readings::EVERY_COMMAND_THIS_MODULE_SENDS
                .iter()
                .map(|command| command.as_str().to_string()),
        );
        every.extend(
            smoozy_server_setup::readings::WHAT_SERVERS_SENDS_FOR_THE_CONTAINER_ROW
                .iter()
                .map(|command| command.to_string()),
        );
        every.extend(
            smoozy_servers::readings::EVERY_COMMAND_THIS_MODULE_SENDS
                .iter()
                .map(|command| command.as_str().to_string()),
        );
        assert!(every.len() > 70, "the set was not read: {}", every.len());
        for one in &every {
            for word in one.split(|c: char| c.is_whitespace() || ";|&()$\"'`".contains(c)) {
                assert!(word != "env" && word != "printenv", "{one:?} prints the environment");
            }
            for named in ["/environ", "~/.ssh", ".ssh/", smoozy_telegram::material::THE_TOKEN_FILES_NAME] {
                assert!(!one.contains(named), "{one:?} names {named}");
            }
        }
    }

    // ------------------------------------------- the live Telegram pass, fault 11

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

    fn a_server_told_telegram_is_on(c: &rusqlite::Connection, label: &str) -> String {
        use smoozy_servers::identity::{Host, KeyReference, Port, Username};
        let mut row = ServerRow::saved(
            smoozy_state::identities::Identity::issue(smoozy_state::identities::IdentityKind::LocalServer)
                .unwrap()
                .as_text(),
            Host::what_the_user_typed("192.0.2.50").unwrap(),
            Port::what_the_user_typed(22).unwrap(),
            Username::what_the_user_typed("root").unwrap(),
            KeyReference { local_key_identifier: "key-1".into(), fingerprint: "SHA256:aaa".into() },
        );
        row.label = Some(label.into());
        row.last_confirmed_installation_identity = Some("i1".into());
        crate::record::server_rows::add(c, &row).unwrap();
        crate::record::server_rows::write(c, &row).unwrap();
        crate::record::told_record::the_telling_was_acknowledged(
            c,
            &row.local_server_identifier,
            &smoozy_telegram::told::WhatSmoozyTells::the_roles_desired_state(true),
            std::time::SystemTime::UNIX_EPOCH,
            "i1",
        )
        .unwrap();
        row.local_server_identifier
    }

    /// **The live Telegram pass, fault 11: turning Telegram off is told at once
    /// to every server connected now** (Specs `Telegram` :37, as changed
    /// 2026-10-31). Three servers were connected when the switch went off, and
    /// each was listed *not reached* until a reconnect that never came. **The
    /// servers told are the ones whose live connection settled on the confirmed
    /// branch** — the rule a connection's own telling keeps — and **no other**:
    /// a server whose connection is being made again, and a server with no
    /// connection at all, wait for the next one and stay listed.
    #[test]
    fn fault_11_switching_off_is_told_at_once_to_every_server_connected_now() {
        use crate::servers::a_period::{ThePeriodsThisRun, TheRunThroughTheApp, WhatTheLastConnectionRead};
        use smoozy_state::identities::InstallationCheck;
        let mut folder = std::env::temp_dir();
        folder.push(format!("smoozy-setting-up-fault-11-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).unwrap();
        let record = crate::record::LocalRecord::open_at(&folder.join("local-record.sqlite3")).unwrap();
        let c = record.connection();
        let a = a_server_told_telegram_is_on(c, "Live A");
        let b = a_server_told_telegram_is_on(c, "Live B");
        let reconnecting = a_server_told_telegram_is_on(c, "Live C");
        let not_connected = a_server_told_telegram_is_on(c, "Live D");
        crate::record::settings::telegram_was_switched(c, true).unwrap();

        let periods = ThePeriodsThisRun::over(
            TheRunThroughTheApp::lending_nothing(),
            std::sync::Arc::new(crate::events::TheOneQueue::new()),
            std::sync::Arc::new(crate::notifications::TheNotificationService::on(Box::new(NothingIsPosted))),
        );
        for server in [&a, &b, &reconnecting] {
            let period = smoozy_servers::driving::APeriod::begun(
                server,
                smoozy_servers::period::OpeningAct::TheUsersConnectAction,
                smoozy_servers::period::WhoTookIt::TheUser,
                std::time::SystemTime::UNIX_EPOCH,
            );
            assert!(periods.a_period_has_begun(server, &period));
            periods.a_connection_read(
                server,
                WhatTheLastConnectionRead {
                    check: InstallationCheck::PresentAndEqualConfirmed,
                    a_fresh_reading_was_kept: true,
                    on_the_connection_that_is_live: true,
                },
            );
        }
        periods.another_attempt_begins(&reconnecting);

        // The switch, the way Settings turns it off.
        assert_eq!(crate::surface::telegram::telegram_is_switched_off_in(c), None);
        let every = crate::record::server_rows::all(c)
            .unwrap()
            .into_iter()
            .map(|row| row.local_server_identifier);
        let told_now: Vec<String> = the_servers_connected_now(&periods, every)
            .into_iter()
            .map(|(server, _)| server)
            .collect();
        let mut expected = vec![a.clone(), b.clone()];
        expected.sort();
        let mut told_sorted = told_now.clone();
        told_sorted.sort();
        assert_eq!(told_sorted, expected, "**fault 11**: only the two connected servers are told now");
        for server in [&a, &b] {
            assert_eq!(
                the_owed_reading(c, server),
                Some(WhatThisServerIsOwed { the_files: false, the_desired_state: Some(false) }),
                "what a connected server is told at once is the switch, and nothing else"
            );
        }
        for server in [&reconnecting, &not_connected] {
            assert!(!told_now.contains(server), "a server with no live connection is told on its next one");
        }
        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    /// **Fault 11's second half: one telling per server at a time.** A telling
    /// asked for while one runs on that server — a connection becoming live
    /// while the switch is turned, or the switch turned twice — is run once
    /// that one ends, over the newest connection it was asked on, because the
    /// running one read what was owed before the change. Two at once could
    /// leave the role on after *disabled* was acknowledged.
    #[test]
    fn fault_11_a_telling_asked_while_one_runs_follows_it_once_with_the_newest_connection() {
        let under_way: TheTellingsUnderWay<&'static str> = TheTellingsUnderWay::default();
        assert_eq!(under_way.asked("one", "first"), Some("first"), "nothing runs, so it starts");
        assert_eq!(under_way.asked("one", "second"), None, "one runs, so this waits for it");
        assert_eq!(under_way.asked("one", "third"), None);
        assert_eq!(under_way.asked("two", "other"), Some("other"), "another server is not held up");
        assert_eq!(under_way.ended("one"), Some("third"), "the newest connection runs next, once");
        assert_eq!(under_way.ended("one"), None, "nothing more was asked, so it stops");
        assert_eq!(under_way.asked("one", "fourth"), Some("fourth"), "and the next starts at once");
        under_way.not_started("one");
        assert_eq!(under_way.asked("one", "fifth"), Some("fifth"), "a telling that never started holds nothing");
    }
}
